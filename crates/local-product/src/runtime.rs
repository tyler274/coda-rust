//! Physician operational truth. Every mutation locks its conversation in Postgres.
//! Construction never migrates or starts work. Expired attempts terminate as
//! interrupted; a claim creates a successor rather than replaying an attempt.
//! Hosted chat's only tool is the read-only `skill_view`
//! ([`crate::physician_chat`]); mutation tools remain unavailable until their
//! product effect owners are ported. A commit publishes only the attempt's
//! final model response, never narration from earlier tool rounds. Reply
//! drafts and their revisions live in [`reply`]. No process-memory
//! persistence fallback.

/// Async callback implementation helper for product adapters.
pub use async_trait::async_trait;
use engine::{Admission, Candidate, Context, Event, Host, Invocation, Item, Usage};

use crate::physician_chat::PhysicianSkills;
/// Neutral execution types, reexported for the physician composition.
pub use harness::engine;
/// Existing API adapter; no second physician provider transport.
pub use harness::responses::Responses;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, Database, DatabaseConnection, DbErr,
    EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait,
    sea_query::{Alias, Expr, ExprTrait, OnConflict, Query, extension::postgres::PgExpr},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Hosted chat drafts: the physician's unsent composer text per chat.
pub mod chat_draft;
/// Hosted work is found, leased and retried on the rows that own it.
pub mod delivery;
mod entities;
/// Command receipts and their 90-day reduction to a salted digest.
pub mod receipt;
/// Explicit bounded restart activation, separate from construction.
pub mod recovery;
/// Hosted physician reply drafts and their revision work.
pub mod reply;
/// Versioned patient source observations: a stand-in source owner.
pub mod sources;
#[cfg(test)]
mod tests;
use entities::{command, conversation};

/// Authenticated product scope, supplied by the server, never the command body.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Scope {
    /// Practice tenant.
    pub practice: String,
    /// Signed-in physician.
    pub physician: String,
    /// Authorized patient workspace.
    pub workspace: String,
    /// Conversation identity. For a reply draft and its revisions, the reply work.
    pub chat: String,
}

/// Accepted physician input, including timezone in the idempotency comparison.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Input {
    /// Client command identity, scoped to tenant and physician.
    pub key: String,
    /// Exact authored text.
    pub text: String,
    /// Physician's timezone.
    pub timezone: String,
    /// The rehearsal scenario this input belongs to. Only a simulation
    /// practice's session may set it; the site refuses it from any other.
    /// Absent inputs serialize exactly as before, so receipts saved earlier
    /// still compare equal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}

/// A saved authored message, not execution telemetry.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    /// User or assistant.
    pub role: String,
    /// Exact authored text.
    pub text: String,
}

/// Product attempt with its immutable predecessor and durable evidence.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attempt {
    /// Attempt identity.
    pub id: String,
    /// Exact previous attempt, if interrupted or steered.
    pub predecessor: Option<String>,
    /// Work identity shared by successor attempts.
    pub work: String,
    /// Input generation fenced at publication.
    pub revision: u64,
    /// Active, interrupted, cancelled, failed, or completed.
    pub status: String,
    /// Database-clock lease expiry in seconds.
    pub expires: i64,
    /// Ordered acknowledged engine evidence, internal only.
    pub events: Vec<Event>,
}

/// Durable conversation and work projection.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Conversation {
    /// Authored conversation only.
    pub messages: Vec<Message>,
    /// Current accepted input generation.
    pub revision: u64,
    /// Pending, active, held, failed, or completed.
    pub status: String,
    /// Current work identity; successors preserve it.
    pub work: String,
    /// All retained attempts, including interrupted attempts.
    pub attempts: Vec<Attempt>,
    /// Accepted session reference, never the bearer secret; absent legacy work is blocked.
    #[serde(default)]
    pub session_hash: Option<Vec<u8>>,
    /// Latest accepted command identity; saved atomically with authored input.
    #[serde(default)]
    pub input_id: Option<String>,
    /// Database-clock time before which pending work is not due: the retry
    /// delay after a failed attempt. Never proof of execution or completion.
    #[serde(default)]
    pub dispatch_after: i64,
    /// Trace context of the request that last made this input runnable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<delivery::TraceContext>,
    /// Unsuccessful attempts on the current input, including expired leases.
    /// New input and Resume reset it; at [`MAX_FAILED_ATTEMPTS`] the work fails.
    #[serde(default)]
    pub failures: u32,
    /// Database-clock seconds when the first physician message was accepted.
    /// Zero on rows saved before the chat list existed.
    #[serde(default)]
    pub opened_at: i64,
    /// Database-clock seconds of the latest accepted message or published reply.
    #[serde(default)]
    pub last_activity_at: i64,
    /// The latest accepted input's rehearsal scenario; see [`Input::scenario`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}

/// One chat in a physician's workspace, for the workspace's chat list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChatSummary {
    /// Conversation identity.
    pub chat: String,
    /// The first physician message, which names the chat until titles exist.
    pub first_message: String,
    /// Database-clock seconds the chat was opened (0 when unknown).
    pub opened_at: i64,
    /// Database-clock seconds of the latest visible activity (0 when unknown).
    pub last_activity_at: i64,
}

/// The chat list reads at most this many conversations of one workspace.
pub const MAX_LISTED_CHATS: u64 = 200;

/// Recovery gives up after this many unsuccessful attempts on one input, so a
/// turn that always fails ends as failed instead of calling the model forever.
pub const MAX_FAILED_ATTEMPTS: u32 = 5;

impl Default for Conversation {
    fn default() -> Self {
        Self {
            messages: vec![],
            revision: 0,
            status: "completed".into(),
            work: String::new(),
            attempts: vec![],
            session_hash: None,
            input_id: None,
            dispatch_after: 0,
            origin: None,
            failures: 0,
            opened_at: 0,
            last_activity_at: 0,
            scenario: None,
        }
    }
}

/// A fenced claim; safe to pass to the shared engine, never authority by itself.
#[derive(Clone, Debug)]
pub struct Claim {
    /// Product scope.
    pub scope: Scope,
    /// Saved attempt identity.
    pub attempt: String,
    /// Input generation.
    pub revision: u64,
    /// Trace context of the request that made this input runnable.
    pub origin: Option<delivery::TraceContext>,
}

/// Durable accepted command outcome.
#[derive(Debug, PartialEq, Eq)]
pub enum Accepted {
    /// Input saved for execution.
    New,
    /// Exact original command already saved; do not dispatch it again.
    Duplicate,
}

/// Outcome of a physician's Try again on failed or held work.
#[derive(Debug, PartialEq, Eq)]
pub enum Resumed {
    /// The work is pending again and workers were told.
    Pending {
        /// Latest accepted input, the delivery's fence.
        input_id: String,
    },
    /// The exact same Try again was already taken; nothing changed.
    Duplicate,
    /// The named work or run is not the current failed or held work, or a
    /// command field is blank.
    Refused,
}

/// Outcome of a physician's Stop on the current chat work.
#[derive(Debug, PartialEq, Eq)]
pub enum Stopped {
    /// The conversation is held until Resume or newer input.
    Held,
    /// The exact same Stop was already taken, or the named work is already
    /// held.
    Duplicate,
    /// Nothing open to hold: missing, completed, or a different work identity,
    /// or a command field is blank.
    Refused,
}

/// Postgres physician runtime owner. Cloning shares connections, not state.
#[derive(Clone)]
pub struct Store {
    db: DatabaseConnection,
}

fn error(message: &str) -> DbErr {
    DbErr::Custom(message.into())
}
fn keys(scope: &Scope) -> (String, String, String, String) {
    (
        scope.practice.clone(),
        scope.physician.clone(),
        scope.workspace.clone(),
        scope.chat.clone(),
    )
}

fn conversation_model(scope: &Scope, state: Value) -> conversation::ActiveModel {
    conversation::ActiveModel {
        practice: Set(scope.practice.clone()),
        physician: Set(scope.physician.clone()),
        workspace: Set(scope.workspace.clone()),
        chat: Set(scope.chat.clone()),
        state: Set(state),
    }
}

fn database_clock() -> Expr {
    // PostgreSQL wall clock, deliberately not transaction-start CURRENT_TIMESTAMP:
    // leases must be checked after waiting for conversation and authority locks.
    // SeaQuery has no dedicated clock_timestamp/epoch extraction expression.
    Expr::cust("floor(extract(epoch FROM clock_timestamp()))::bigint")
}

async fn now(tx: &sea_orm::DatabaseTransaction) -> Result<i64, DbErr> {
    let query = Query::select()
        .expr_as(database_clock(), Alias::new("now"))
        .to_owned();
    tx.query_one(&query)
        .await?
        .ok_or_else(|| error("missing database clock"))?
        .try_get("", "now")
}

impl Store {
    /// Connects only. Schema-owner credentials belong to the migration job.
    pub async fn connect(url: &str) -> Result<Self, DbErr> {
        let mut options = sea_orm::ConnectOptions::new(url);
        options.max_connections(8).sqlx_logging(false);
        Ok(Self::from_connection(Database::connect(options).await?))
    }

    /// Uses an existing pool without applying migrations or changing its limits.
    pub fn from_connection(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Explicit additive, rerunnable migration, never invoked by connect.
    pub async fn migrate(&self) -> Result<(), DbErr> {
        let tx = self.db.begin().await?;
        // PostgreSQL-only migration serialization; the recorded schema below stays
        // unchanged. Ordinary runtime access uses the owning typed entities.
        tx.execute_unprepared("SELECT pg_advisory_xact_lock(804081)")
            .await?;
        tx.execute_unprepared(include_str!("runtime.sql")).await?;
        // Later tables come from their entities through the SeaORM schema
        // builder, so the recorded statements above never change.
        let schema = sea_orm::Schema::new(sea_orm::DbBackend::Postgres);
        let mut chat_drafts = schema.create_table_from_entity(entities::chat_draft::Entity);
        chat_drafts.if_not_exists();
        tx.execute(&chat_drafts).await?;
        let mut drafts = schema.create_table_from_entity(entities::reply_draft::Entity);
        drafts.if_not_exists();
        tx.execute(&drafts).await?;
        let mut revisions = schema.create_table_from_entity(entities::reply_revision::Entity);
        revisions.if_not_exists();
        tx.execute(&revisions).await?;
        // The unique (draft, ordinal) index that orders a draft's history.
        for mut index in schema.create_index_from_entity(entities::reply_revision::Entity) {
            index.if_not_exists();
            tx.execute(&index).await?;
        }
        let mut source_versions = schema.create_table_from_entity(entities::source_version::Entity);
        source_versions.if_not_exists();
        tx.execute(&source_versions).await?;
        let mut sources = schema.create_table_from_entity(entities::source::Entity);
        sources.if_not_exists();
        tx.execute(&sources).await?;
        let mut source_pulls = schema.create_table_from_entity(entities::source_pull::Entity);
        source_pulls.if_not_exists();
        tx.execute(&source_pulls).await?;
        // Receipts older than this column get the migration's time, so the
        // 90-day reduction counts from when it could first see them. `now()`
        // is stable, so Postgres stores it once instead of rewriting rows.
        let accepted_at = sea_orm::sea_query::Table::alter()
            .table(command::Entity)
            .add_column_if_not_exists(
                sea_orm::sea_query::ColumnDef::new(command::Column::AcceptedAt)
                    .big_integer()
                    .not_null()
                    .default(Expr::cust("floor(extract(epoch FROM now()))::bigint")),
            )
            .to_owned();
        tx.execute(&accepted_at).await?;
        tx.commit().await
    }

    async fn locked(
        &self,
        scope: &Scope,
    ) -> Result<(sea_orm::DatabaseTransaction, Conversation, i64), DbErr> {
        if [
            &scope.practice,
            &scope.physician,
            &scope.workspace,
            &scope.chat,
        ]
        .iter()
        .any(|s| s.trim().is_empty())
        {
            return Err(error("empty physician scope"));
        }
        let tx = self.db.begin().await?;
        let row = conversation::Entity::find_by_id(keys(scope))
            .lock_exclusive()
            .one(&tx)
            .await?
            .ok_or_else(|| error("conversation not found"))?;
        let state = row.state;
        let now = now(&tx).await?;
        Ok((
            tx,
            serde_json::from_value(state).map_err(|_| error("corrupt conversation"))?,
            now,
        ))
    }

    async fn save(
        tx: sea_orm::DatabaseTransaction,
        scope: &Scope,
        state: &Conversation,
    ) -> Result<(), DbErr> {
        conversation_model(
            scope,
            serde_json::to_value(state).map_err(|_| error("invalid conversation"))?,
        )
        .update(&tx)
        .await?;
        tx.commit().await
    }

    /// Atomically saves the command receipt and input. Colliding payloads fail.
    pub async fn accept(&self, scope: &Scope, input: &Input) -> Result<Accepted, DbErr> {
        self.accept_inner(scope, input, None, None).await
    }

    /// Accepts hosted input with its original session reference for restart
    /// authorization, and tells listening workers in the same transaction.
    /// `origin` is the accepting request's trace context.
    pub async fn accept_authorized(
        &self,
        scope: &Scope,
        input: &Input,
        session_hash: Vec<u8>,
        origin: Option<delivery::TraceContext>,
    ) -> Result<Accepted, DbErr> {
        if session_hash.len() != 32 {
            return Err(error("invalid session reference"));
        }
        self.accept_inner(scope, input, Some(session_hash), origin)
            .await
    }

    async fn accept_inner(
        &self,
        scope: &Scope,
        input: &Input,
        session_hash: Option<Vec<u8>>,
        origin: Option<delivery::TraceContext>,
    ) -> Result<Accepted, DbErr> {
        if [
            &input.key,
            &input.text,
            &input.timezone,
            &scope.practice,
            &scope.physician,
            &scope.workspace,
            &scope.chat,
        ]
        .iter()
        .any(|s| s.trim().is_empty())
        {
            return Err(error("empty command field"));
        }
        let tx = self.db.begin().await?;
        let command = serde_json::json!({"scope": scope, "input": input});
        match receipt::record(&tx, &scope.practice, &scope.physician, &input.key, &command).await? {
            receipt::Receipt::New => {}
            receipt::Receipt::Same => {
                tx.commit().await?;
                return Ok(Accepted::Duplicate);
            }
            receipt::Receipt::Different => return Err(error("idempotency collision")),
        }
        conversation::Entity::insert(conversation_model(
            scope,
            serde_json::to_value(Conversation::default()).unwrap(),
        ))
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .exec_without_returning(&tx)
        .await?;
        let row = conversation::Entity::find_by_id(keys(scope))
            .lock_exclusive()
            .one(&tx)
            .await?
            .ok_or_else(|| error("missing conversation"))?;
        let mut state: Conversation =
            serde_json::from_value(row.state).map_err(|_| error("corrupt conversation"))?;
        if state.status == "completed" {
            state.work = Uuid::new_v4().to_string();
        }
        if let Some(attempt) = state.attempts.last_mut().filter(|a| a.status == "active") {
            attempt.status = "interrupted".into();
        }
        state.revision += 1;
        // Only hosted work is run by workers; local work is claimed in process.
        if session_hash.is_some() {
            let delivery = delivery::Delivery::chat_turn(scope.clone(), input.key.clone());
            delivery::notify(&tx, &delivery).await?;
        }
        state.session_hash = session_hash;
        state.input_id = Some(input.key.clone());
        state.scenario = input.scenario.clone();
        state.dispatch_after = 0;
        state.origin = origin;
        state.failures = 0;
        let now = now(&tx).await?;
        if state.messages.is_empty() {
            state.opened_at = now;
        }
        state.last_activity_at = now;
        state.messages.push(Message {
            role: "user".into(),
            text: input.text.clone(),
        });
        // New input steers ordinary work. After Stop it is the physician
        // sending something else, which releases the hold and drops Resume.
        state.status = "pending".into();
        Self::save(tx, scope, &state).await?;
        Ok(Accepted::New)
    }

    /// Reads authoritative state using the complete tenant/physician binding.
    pub async fn conversation(&self, scope: &Scope) -> Result<Conversation, DbErr> {
        let (tx, state, _) = self.locked(scope).await?;
        tx.commit().await?;
        Ok(state)
    }

    /// Claims pending work or recovers an expired active attempt as its successor.
    /// Lease must cover the bounded engine deadline; no two live claims coexist.
    pub async fn claim(&self, scope: &Scope, lease_seconds: i64) -> Result<Option<Claim>, DbErr> {
        self.claim_inner(scope, lease_seconds, None).await
    }

    async fn claim_inner(
        &self,
        scope: &Scope,
        lease_seconds: i64,
        input_id: Option<&str>,
    ) -> Result<Option<Claim>, DbErr> {
        if !(1..=600).contains(&lease_seconds) {
            return Err(error("invalid lease duration"));
        }
        let (tx, mut state, now) = self.locked(scope).await?;
        if input_id.is_some_and(|id| state.input_id.as_deref() != Some(id)) {
            tx.commit().await?;
            return Ok(None);
        }
        if state.status == "active" && state.attempts.last().is_some_and(|a| a.expires <= now) {
            state.attempts.last_mut().unwrap().status = "interrupted".into();
            state.status = "pending".into();
            // A lease that ran out is an unsuccessful attempt too: a turn that
            // keeps crashing its worker must reach the same end as one that fails.
            state.failures += 1;
            if state.failures >= MAX_FAILED_ATTEMPTS {
                state.status = "failed".into();
                Self::save(tx, scope, &state).await?;
                return Ok(None);
            }
        }
        // Failed work waits out its retry delay.
        if state.status != "pending" || state.dispatch_after > now {
            tx.commit().await?;
            return Ok(None);
        }
        let id = Uuid::new_v4().to_string();
        state.attempts.push(Attempt {
            id: id.clone(),
            predecessor: state
                .attempts
                .last()
                .filter(|a| a.work == state.work)
                .map(|a| a.id.clone()),
            work: state.work.clone(),
            revision: state.revision,
            status: "active".into(),
            expires: now + lease_seconds,
            events: vec![],
        });
        state.status = "active".into();
        let claim = Claim {
            scope: scope.clone(),
            attempt: id,
            revision: state.revision,
            origin: state.origin.clone(),
        };
        Self::save(tx, scope, &state).await?;
        Ok(Some(claim))
    }

    fn current(state: &Conversation, claim: &Claim, now: i64) -> bool {
        state.status == "active"
            && state.revision == claim.revision
            && state
                .attempts
                .last()
                .is_some_and(|a| a.id == claim.attempt && a.status == "active" && a.expires > now)
    }

    /// Revalidates the accepted session using the real auth owner's transaction locks.
    pub async fn authorize(
        &self,
        scope: &Scope,
        authority: &mut dyn Authority,
    ) -> Result<(), DbErr> {
        let (tx, state, _) = self.locked(scope).await?;
        authority
            .lock_publication(&tx, scope, state.session_hash.as_deref())
            .await
            .map_err(|e| error(&e))?;
        tx.commit().await
    }

    /// Whether the claim is still the current attempt: not stopped, not
    /// replaced by a newer message, and not expired. `Err` means currentness
    /// could not be read: the database, or a missing or unreadable
    /// conversation.
    pub async fn is_current(&self, claim: &Claim) -> Result<bool, DbErr> {
        let (tx, state, now) = self.locked(&claim.scope).await?;
        let current = Self::current(&state, claim, now);
        tx.commit().await?;
        Ok(current)
    }

    /// Rechecks the current attempt against durable Stop, steering and expiry.
    pub async fn assert_current(&self, claim: &Claim) -> Result<(), DbErr> {
        if self.is_current(claim).await? {
            Ok(())
        } else {
            Err(error("attempt no longer current"))
        }
    }

    /// Acknowledges an ordered event only after commit. Exact replay is idempotent.
    pub async fn persist(&self, claim: &Claim, event: Event) -> Result<(), DbErr> {
        let (tx, mut state, now) = self.locked(&claim.scope).await?;
        if event.attempt_id != claim.attempt {
            return Err(error("event attempt mismatch"));
        }
        let current = Self::current(&state, claim, now);
        // Late provider/tool evidence is inspectable truth, not permission to
        // execute or publish. Never acknowledge a stale dispatch or candidate.
        if !current
            && !matches!(
                event.kind,
                engine::EventKind::ProviderReturned(_) | engine::EventKind::ToolReturned { .. }
            )
        {
            return Err(error("stale event"));
        }
        let attempt = state
            .attempts
            .iter_mut()
            .find(|a| a.id == claim.attempt && a.revision == claim.revision)
            .ok_or_else(|| error("unknown attempt"))?;
        let events = &mut attempt.events;
        if event.sequence == 0 {
            return Err(error("invalid event sequence"));
        }
        if let Some(previous) = events.get((event.sequence - 1) as usize) {
            if serde_json::to_value(previous).unwrap() != serde_json::to_value(&event).unwrap() {
                return Err(error("event collision"));
            }
        } else if event.sequence == events.len() as u64 + 1 {
            events.push(event);
        } else {
            return Err(error("event sequence gap"));
        }
        Self::save(tx, &claim.scope, &state).await
    }

    /// Publishes a candidate only if its acknowledged evidence and claim still match.
    pub async fn commit(&self, claim: &Claim, candidate: &Candidate) -> Result<(), DbErr> {
        self.commit_inner(claim, candidate, None).await
    }

    /// Locks the real auth owner on this same transaction through reply publication.
    pub async fn commit_authorized(
        &self,
        claim: &Claim,
        candidate: &Candidate,
        authority: &mut dyn Authority,
    ) -> Result<(), DbErr> {
        self.commit_inner(claim, candidate, Some(authority)).await
    }

    async fn commit_inner(
        &self,
        claim: &Claim,
        candidate: &Candidate,
        authority: Option<&mut dyn Authority>,
    ) -> Result<(), DbErr> {
        let (tx, mut state, _) = self.locked(&claim.scope).await?;
        if let Some(authority) = authority {
            authority
                .lock_publication(&tx, &claim.scope, state.session_hash.as_deref())
                .await
                .map_err(|e| error(&e))?;
        } else if state.session_hash.is_some() {
            return Err(error("hosted publication requires transactional authority"));
        }
        let now = now(&tx).await?;
        if !Self::current(&state, claim, now) {
            return Err(error("stale completion"));
        }
        let attempt = state.attempts.last_mut().unwrap();
        if !matches!(attempt.events.last().map(|e| &e.kind), Some(engine::EventKind::Candidate { text }) if text == &candidate.text)
        {
            return Err(error("candidate lacks durable evidence"));
        }
        let answer = terminal_answer(&attempt.events)?;
        attempt.status = "completed".into();
        state.status = "completed".into();
        state.last_activity_at = now;
        state.messages.push(Message {
            role: "assistant".into(),
            text: answer,
        });
        Self::save(tx, &claim.scope, &state).await
    }

    /// Ends only the named still-open attempt. Failure does not restart itself.
    pub async fn interrupt(&self, claim: &Claim, failed: bool) -> Result<(), DbErr> {
        let (tx, mut state, _) = self.locked(&claim.scope).await?;
        if state
            .attempts
            .last()
            .is_some_and(|a| a.id == claim.attempt && a.status == "active")
        {
            state.attempts.last_mut().unwrap().status =
                if failed { "failed" } else { "interrupted" }.into();
            state.status = if failed { "failed" } else { "pending" }.into();
        }
        Self::save(tx, &claim.scope, &state).await
    }

    /// Intentional Stop is durable and cannot be released by restart or late output.
    pub async fn stop(&self, scope: &Scope) -> Result<(), DbErr> {
        let (tx, mut state, _) = self.locked(scope).await?;
        if state.status != "completed" {
            if let Some(a) = state.attempts.last_mut().filter(|a| a.status == "active") {
                a.status = "cancelled".into();
            }
            state.status = "held".into();
        }
        Self::save(tx, scope, &state).await
    }

    /// Explicit physician resume of held or failed work; never repeats completed work.
    pub async fn resume(&self, scope: &Scope) -> Result<(), DbErr> {
        let (tx, mut state, _) = self.locked(scope).await?;
        if state.status == "held" || state.status == "failed" {
            state.status = "pending".into();
            state.failures = 0;
            state.dispatch_after = 0;
        }
        Self::save(tx, scope, &state).await
    }

    /// Hosted Try again. Resumes only the named work while it is failed or held
    /// and its latest attempt is `expected_run_id`, so a stale page never
    /// restarts newer work and completed work is never reopened. A blank field
    /// is refused. The receipt is saved with the change: the same `key` and
    /// target answers `Duplicate`.
    /// The accepted session moves to the physician's current session, because
    /// the original one may have ended while the work sat failed. Listening
    /// workers are told in the same transaction, and `origin`, this request's
    /// trace context, replaces the accepting request's.
    pub async fn resume_authorized(
        &self,
        scope: &Scope,
        key: &str,
        session_hash: Vec<u8>,
        work_item_id: &str,
        expected_run_id: &str,
        origin: Option<delivery::TraceContext>,
    ) -> Result<Resumed, DbErr> {
        if session_hash.len() != 32 {
            return Err(error("invalid session reference"));
        }
        if [
            key,
            work_item_id,
            expected_run_id,
            &scope.practice,
            &scope.physician,
            &scope.workspace,
            &scope.chat,
        ]
        .iter()
        .any(|s| s.trim().is_empty())
        {
            return Ok(Resumed::Refused);
        }
        let tx = self.db.begin().await?;
        let command = serde_json::json!({
            "scope": scope,
            "resume": {"work_item_id": work_item_id, "expected_run_id": expected_run_id},
        });
        match receipt::record(&tx, &scope.practice, &scope.physician, key, &command).await? {
            receipt::Receipt::New => {}
            receipt::Receipt::Same => {
                tx.commit().await?;
                return Ok(Resumed::Duplicate);
            }
            receipt::Receipt::Different => {
                tx.commit().await?;
                return Ok(Resumed::Refused);
            }
        }
        let Some(row) = conversation::Entity::find_by_id(keys(scope))
            .lock_exclusive()
            .one(&tx)
            .await?
        else {
            // Dropping the transaction rolls the receipt back.
            return Ok(Resumed::Refused);
        };
        let mut state: Conversation =
            serde_json::from_value(row.state).map_err(|_| error("corrupt conversation"))?;
        let resumable = matches!(state.status.as_str(), "failed" | "held")
            && state.work == work_item_id
            && state
                .attempts
                .last()
                .is_some_and(|attempt| attempt.id == expected_run_id);
        let Some(input_id) = state.input_id.clone().filter(|_| resumable) else {
            return Ok(Resumed::Refused);
        };
        state.status = "pending".into();
        state.failures = 0;
        state.dispatch_after = 0;
        state.origin = origin;
        state.session_hash = Some(session_hash);
        delivery::notify(
            &tx,
            &delivery::Delivery::chat_turn(scope.clone(), input_id.clone()),
        )
        .await?;
        Self::save(tx, scope, &state).await?;
        Ok(Resumed::Pending { input_id })
    }

    /// Hosted Stop. Holds the named work until Resume or newer input, and
    /// only while that work is still pending or active. Each hold names its
    /// own run, so Resume keyed by that run is never a replay of an earlier
    /// Resume. A failed turn stays failed, so Try again can resume it. The
    /// receipt is saved with the change: the same `key` and work answers
    /// [`Stopped::Duplicate`], and so does a new key while that work is
    /// already held. A different command under that key is refused.
    /// Completed work, a failed turn, a different work identity, a blank
    /// field, and a scope with no conversation are refused. Only a new hold
    /// keeps its receipt. Workers are not told: a held turn has nothing to run.
    pub async fn stop_authorized(
        &self,
        scope: &Scope,
        key: &str,
        session_hash: Vec<u8>,
        work_item_id: &str,
    ) -> Result<Stopped, DbErr> {
        if session_hash.len() != 32 {
            return Err(error("invalid session reference"));
        }
        if [
            key,
            work_item_id,
            &scope.practice,
            &scope.physician,
            &scope.workspace,
            &scope.chat,
        ]
        .iter()
        .any(|part| part.trim().is_empty())
        {
            return Ok(Stopped::Refused);
        }
        let tx = self.db.begin().await?;
        let command = serde_json::json!({
            "scope": scope,
            "stop": {"work_item_id": work_item_id},
        });
        match receipt::record(&tx, &scope.practice, &scope.physician, key, &command).await? {
            receipt::Receipt::New => {}
            receipt::Receipt::Same => {
                tx.commit().await?;
                return Ok(Stopped::Duplicate);
            }
            receipt::Receipt::Different => {
                tx.commit().await?;
                return Ok(Stopped::Refused);
            }
        }
        let Some(row) = conversation::Entity::find_by_id(keys(scope))
            .lock_exclusive()
            .one(&tx)
            .await?
        else {
            return Ok(Stopped::Refused);
        };
        let mut state: Conversation =
            serde_json::from_value(row.state).map_err(|_| error("corrupt conversation"))?;
        // Another press on work that is already held, from a page that has
        // not caught up or a second tab, finds what it asked for. Nothing
        // changes, so its receipt is not kept.
        if state.status == "held" && state.work == work_item_id {
            return Ok(Stopped::Duplicate);
        }
        // A failed turn stays Try again. Stop does not turn it into a hold.
        if !matches!(state.status.as_str(), "pending" | "active") || state.work != work_item_id {
            return Ok(Stopped::Refused);
        }
        if let Some(attempt) = state
            .attempts
            .last_mut()
            .filter(|attempt| attempt.status == "active")
        {
            attempt.status = "cancelled".into();
        } else {
            // Stop before a claim still needs a run Resume can name. The last
            // run may be the one an earlier hold named and Resume already
            // took, so this hold gets its own run, or Resume would be a replay.
            state.attempts.push(Attempt {
                id: Uuid::new_v4().to_string(),
                predecessor: state
                    .attempts
                    .last()
                    .filter(|a| a.work == state.work)
                    .map(|a| a.id.clone()),
                work: state.work.clone(),
                revision: state.revision,
                status: "cancelled".into(),
                expires: 0,
                events: vec![],
            });
        }
        state.status = "held".into();
        state.session_hash = Some(session_hash);
        Self::save(tx, scope, &state).await?;
        Ok(Stopped::Held)
    }

    /// The physician's chats in one workspace, latest activity first, then
    /// chat id descending. Another physician's chats are never listed.
    pub async fn chats(
        &self,
        practice: &str,
        physician: &str,
        workspace: &str,
    ) -> Result<Vec<ChatSummary>, DbErr> {
        use conversation::Column as C;
        if [practice, physician, workspace]
            .iter()
            .any(|s| s.trim().is_empty())
        {
            return Err(error("empty physician scope"));
        }
        // Newest activity first, ordered in Postgres before the limit: a chat
        // that just got a message must make the list however many older chats
        // the workspace has. Rows saved before the activity fields existed
        // count as 0. Chat id breaks ties, as in `chats.sort_by` below.
        let seconds = |field: &str| {
            sea_orm::sea_query::Func::coalesce([
                Expr::col(C::State)
                    .cast_json_field(field)
                    .cast_as(Alias::new("bigint")),
                Expr::val(0_i64),
            ])
        };
        let activity = sea_orm::sea_query::Func::greatest([
            Expr::from(seconds("last_activity_at")),
            Expr::from(seconds("opened_at")),
        ]);
        let rows = conversation::Entity::find()
            .filter(C::Practice.eq(practice))
            .filter(C::Physician.eq(physician))
            .filter(C::Workspace.eq(workspace))
            .order_by_desc(Expr::from(activity))
            .order_by_desc(C::Chat)
            .limit(MAX_LISTED_CHATS)
            .all(&self.db)
            .await?;
        let mut chats = Vec::with_capacity(rows.len());
        for row in rows {
            let state: Conversation =
                serde_json::from_value(row.state).map_err(|_| error("corrupt conversation"))?;
            let Some(first) = state.messages.iter().find(|m| m.role == "user") else {
                continue;
            };
            chats.push(ChatSummary {
                chat: row.chat,
                first_message: first.text.clone(),
                opened_at: state.opened_at,
                last_activity_at: Ord::max(state.last_activity_at, state.opened_at),
            });
        }
        chats.sort_by(|a, b| {
            b.last_activity_at
                .cmp(&a.last_activity_at)
                .then_with(|| b.chat.cmp(&a.chat))
        });
        Ok(chats)
    }

    /// Enumerates durable pending/expired work for explicit server activation.
    /// Caller must reauthorize each scope; construction never starts execution.
    pub async fn pending(&self) -> Result<Vec<Scope>, DbErr> {
        self.pending_batch(100, None).await
    }

    async fn pending_batch(
        &self,
        maximum: usize,
        after: Option<&Scope>,
    ) -> Result<Vec<Scope>, DbErr> {
        use conversation::Column as C;
        let after = after.cloned().unwrap_or(Scope {
            practice: String::new(),
            physician: String::new(),
            workspace: String::new(),
            chat: String::new(),
        });
        let columns = [C::Practice, C::Physician, C::Workspace, C::Chat];
        let status = Expr::col(C::State).cast_json_field("status");
        let expires = Expr::col(C::State)
            .get_json_field("attempts")
            .get_json_field(-1_i32)
            .cast_json_field("expires")
            .cast_as(Alias::new("bigint"));
        let mut query = conversation::Entity::find()
            .select_only()
            .columns(columns)
            .filter(Expr::tuple(columns.map(Expr::col)).gt(Expr::tuple([
                Expr::val(after.practice),
                Expr::val(after.physician),
                Expr::val(after.workspace),
                Expr::val(after.chat),
            ])))
            .filter(
                Condition::any().add(status.clone().eq("pending")).add(
                    Condition::all()
                        .add(status.eq("active"))
                        .add(expires.lte(database_clock())),
                ),
            )
            .limit(maximum as u64);
        for column in columns {
            query = query.order_by_asc(column);
        }
        let rows = query
            .into_tuple::<(String, String, String, String)>()
            .all(&self.db)
            .await?;
        Ok(rows
            .into_iter()
            .map(|(practice, physician, workspace, chat)| Scope {
                practice,
                physician,
                workspace,
                chat,
            })
            .collect())
    }
}

/// The physician-visible answer of a completed attempt: the assistant text of
/// the recorded provider turn that ended it, in order. The candidate spans
/// every round, including narration written before a tool call; that stays in
/// the attempt's events and is never published. The ending turn must be the
/// call-free `ProviderReturned` recorded just before the candidate, and its
/// text must not be blank. Earlier prose is never substituted.
fn terminal_answer(events: &[Event]) -> Result<String, DbErr> {
    let terminal = events
        .len()
        .checked_sub(2)
        .and_then(|index| events.get(index))
        .map(|event| &event.kind);
    let Some(engine::EventKind::ProviderReturned(turn)) = terminal else {
        return Err(error("candidate lacks a recorded terminal turn"));
    };
    let mut answer = String::new();
    for item in &turn.items {
        match item {
            Item::Call(_) => return Err(error("terminal turn requested a tool")),
            Item::Message { role, text } if role == "assistant" => answer.push_str(text),
            _ => {}
        }
    }
    if answer.trim().is_empty() {
        return Err(error("terminal turn has no authored answer"));
    }
    Ok(answer)
}

/// Product access is checked on every dispatch and again before publication.
#[async_trait]
pub trait Authority: Send {
    /// Revalidates physician/session and patient workspace access.
    async fn check(&mut self, scope: &Scope) -> Result<(), String>;
    /// Hold the auth owner's revocation locks until the supplied transaction commits.
    /// Default is fail-closed, never a separate preflight substituted for a fence.
    async fn lock_publication(
        &mut self,
        _tx: &PublicationTransaction,
        _scope: &Scope,
        _session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        Err("transactional publication authority is unavailable".into())
    }
}

/// Transaction borrowed by the auth owner, never a separate connection.
pub type PublicationTransaction = sea_orm::DatabaseTransaction;

/// Physician Host for the one shared execution loop. The only tool is
/// `skill_view`, offered when `skills` is supplied; other tools stay
/// unavailable until their real product effect/read owners exist, never fake
/// success.
pub struct PhysicianHost<A> {
    /// Durable owner.
    pub store: Store,
    /// Fenced attempt.
    pub claim: Claim,
    /// Current authorization owner.
    pub authority: A,
    /// Product-selected versioned instructions, including the skill catalog
    /// when `skills` is set (see [`PhysicianSkills::instructions`]).
    pub instructions: String,
    /// The process's loaded skill catalog. `None` offers no tools.
    pub skills: Option<PhysicianSkills>,
}

#[async_trait]
impl<A: Authority> Host for PhysicianHost<A> {
    async fn prepare(&mut self, transcript: &[Item], _: &Usage) -> Result<Context, String> {
        self.assert_current().await?;
        let state = self
            .store
            .conversation(&self.claim.scope)
            .await
            .map_err(|e| e.to_string())?;
        let items = if transcript.is_empty() {
            state
                .messages
                .into_iter()
                .map(|m| Item::Message {
                    role: m.role,
                    text: m.text,
                })
                .collect()
        } else {
            // The engine returns the full prepared context, not only its delta.
            transcript.to_vec()
        };
        Ok(Context {
            instructions: self.instructions.clone(),
            items,
            tools: self
                .skills
                .as_ref()
                .map(|_| vec![PhysicianSkills::tool()])
                .unwrap_or_default(),
        })
    }
    async fn assert_current(&mut self) -> Result<(), String> {
        self.authority.check(&self.claim.scope).await?;
        self.store
            .assert_current(&self.claim)
            .await
            .map_err(|e| e.to_string())
    }
    async fn persist(&mut self, event: Event) -> Result<(), String> {
        self.store
            .persist(&self.claim, event)
            .await
            .map_err(|e| e.to_string())
    }
    async fn admit(&mut self, call: &Invocation, arguments: &Value) -> Result<Admission, String> {
        // The engine has already rechecked currentness; reads change nothing.
        Ok(match &self.skills {
            Some(skills) => skills.admit(call, arguments),
            None => Admission::Deny("This hosted tool is not yet available".into()),
        })
    }
    async fn execute(&mut self, call: &Invocation, arguments: Value) -> Result<String, String> {
        match &self.skills {
            Some(skills) => Ok(skills.execute(call, &arguments)),
            None => Err("No hosted tool effect owner is configured".into()),
        }
    }
}

impl<A: Authority> PhysicianHost<A> {
    /// Executes through the shared harness, then applies the product publication fence.
    pub async fn run(
        &mut self,
        provider: &mut dyn engine::Provider,
        limits: &engine::Limits,
        cancel: &engine::Cancellation,
    ) -> Result<(), String> {
        let attempt = self.claim.attempt.clone();
        let result = engine::run(provider, self, &attempt, vec![], limits, cancel).await;
        match result {
            Ok(candidate) => {
                self.assert_current().await?;
                self.store
                    .commit_authorized(&self.claim, &candidate, &mut self.authority)
                    .await
                    .map_err(|e| e.to_string())
            }
            Err(failure) => {
                // A reached bound would be reached again, so the turn ends
                // as couldn't finish. Any other failure leaves the attempt
                // leased: Stop has already cancelled it, and an
                // infrastructure failure stays active so `retry_delivery` or
                // lease expiry counts it even if this process dies first.
                if matches!(failure.failure, engine::Failure::Limit(_)) {
                    self.store
                        .interrupt(&self.claim, true)
                        .await
                        .map_err(|e| e.to_string())?;
                }
                Err(format!("execution ended: {:?}", failure.failure))
            }
        }
    }
}
