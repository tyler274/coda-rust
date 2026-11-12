//! Physician operational truth. Every mutation locks its conversation in Postgres.
//! Construction never migrates or starts work. Expired attempts terminate as
//! interrupted; a claim creates a successor rather than replaying an attempt.
//! Hosted chat opens packaged skills and reads the authorized workspace chart
//! on demand ([`crate::physician_chat`]). A commit publishes only the attempt's
//! final model response, never narration from earlier tool rounds. Reply
//! drafts and their revisions live in [`reply`]. No process-memory
//! persistence fallback.

/// Async callback implementation helper for product adapters.
pub use async_trait::async_trait;
use engine::recipe::{Finish, Provider, RequestBinding, RequestRecipe};
use engine::{Event, Invocation, Item, Product, ToolOutcome, Turn};

use crate::physician_chat::PhysicianSkills;
/// Neutral execution types, reexported for the physician composition.
pub use harness::engine;
/// Shared API adapter; the job supplies its frozen model binding.
pub use harness::provider::ResponsesProvider;
use sea_orm::sea_query::StringLen;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, Database, DatabaseConnection, DbErr,
    EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait,
    sea_query::{Alias, Expr, ExprTrait, OnConflict, Query, extension::postgres::PgExpr},
};
use sea_orm_migration::MigratorTrait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Hosted chat drafts: the physician's unsent composer text per chat.
pub mod chat_draft;
mod control;
pub use control::Lease;
/// Queue delivery recovery uses the conversation owner, not a parallel ledger.
pub mod delivery;
mod entities;
mod frozen;
pub use frozen::{BindingRef, SkillPackageId};
mod migration;
mod quota;
/// Command receipts and their 90-day reduction to a salted digest.
pub mod receipt;
mod records;
/// Hosted physician reply drafts and their revision work.
pub mod reply;
/// Versioned patient source observations: a stand-in source owner.
pub mod sources;
#[cfg(test)]
mod tests;
/// Working notes derived from the authorized product record.
pub mod working_notes;
use crate::physician_sources::{ChartRequest, ChartScope, READ_TOOL, SEARCH_TOOL};
use entities::conversation;

/// Authenticated product scope, supplied by the server, never the command body.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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

impl Scope {
    fn validate(&self) -> Result<(), DbErr> {
        if [&self.practice, &self.physician, &self.workspace, &self.chat]
            .iter()
            .any(|s| s.trim().is_empty())
        {
            return Err(error("empty physician scope"));
        }
        Ok(())
    }

    /// Stable cache partition for this scoped conversation or reply job.
    pub fn cache_key(&self, namespace: &str) -> harness::PromptCacheKey {
        harness::PromptCacheKey::scoped(
            namespace,
            &[&self.practice, &self.physician, &self.workspace, &self.chat],
        )
    }
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

/// Origin of a conversation entry. Only physician input carries a command receipt.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum MessageRole {
    /// Physician command accepted on this work, in this fenced revision.
    User {
        /// Original physician command key.
        id: String,
        /// Accepted work that carries the command.
        work: String,
        /// Input generation of this command.
        revision: u64,
    },
    /// Physician-visible model output.
    Assistant {
        /// Stable identity of the published assistant message.
        id: String,
        /// A retained terminal answer that did not complete the accepted work.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        partial: bool,
    },
}

impl MessageRole {
    /// Role in product readback.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User { .. } => "user",
            Self::Assistant { .. } => "assistant",
        }
    }
}

/// A saved authored message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    /// Authorship and, for physician input, its durable identity.
    #[serde(flatten)]
    pub role: MessageRole,
    /// Exact authored text.
    pub text: String,
}

/// The durable name of one of Coda's published replies: the id its
/// [`MessageRole::Assistant`] message was saved with. It stays the same across
/// restarts, so feedback and evaluation can refer to the reply by it.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct ResponseId(String);

impl ResponseId {
    /// The reply id, as stored and shown on the wire.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ResponseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// One of Coda's published replies, named by its [`ResponseId`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response<'a> {
    /// Its place among the conversation's messages.
    pub place: usize,
    /// The reply's durable name.
    pub id: ResponseId,
    /// The reply as saved.
    pub text: &'a str,
}

/// Where a conversation's current work stands. Each value is saved as its
/// text in the conversation row's status column.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    sea_orm::EnumIter,
    sea_orm::DeriveActiveEnum,
)]
#[serde(rename_all = "lowercase")]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum WorkStatus {
    /// Accepted and waiting for an attempt.
    #[sea_orm(string_value = "pending")]
    Pending,
    /// An attempt is running the work.
    #[sea_orm(string_value = "active")]
    Active,
    /// The run published its answer. A conversation with no work yet also
    /// starts here.
    #[sea_orm(string_value = "completed")]
    Completed,
    /// Held until the physician continues it.
    #[sea_orm(string_value = "held")]
    Held,
    /// Recovery gave up; Try again resumes it.
    #[sea_orm(string_value = "failed")]
    Failed,
    /// The run was cancelled.
    #[sea_orm(string_value = "cancelled")]
    Cancelled,
}

impl WorkStatus {
    /// Pending or active: the work has not ended.
    pub fn running(self) -> bool {
        match self {
            Self::Pending | Self::Active => true,
            Self::Completed | Self::Held | Self::Failed | Self::Cancelled => false,
        }
    }

    /// Failed or held: the physician's Try again can continue it.
    fn resumable(self) -> bool {
        match self {
            Self::Failed | Self::Held => true,
            Self::Pending | Self::Active | Self::Completed | Self::Cancelled => false,
        }
    }
}

impl From<RunOutcome> for WorkStatus {
    fn from(outcome: RunOutcome) -> Self {
        match outcome {
            RunOutcome::Completed => Self::Completed,
            RunOutcome::Held { .. } => Self::Held,
            RunOutcome::Failed => Self::Failed,
            RunOutcome::Cancelled => Self::Cancelled,
        }
    }
}

/// Where one attempt stands, for chat and revision attempts alike. Each value
/// is saved as its text in the attempt row's status column.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    sea_orm::EnumIter,
    sea_orm::DeriveActiveEnum,
)]
#[serde(rename_all = "lowercase")]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum AttemptStatus {
    /// Running under its lease.
    #[sea_orm(string_value = "active")]
    Active,
    /// Ended before a result: lost lease, steer or cancellation of a revision.
    #[sea_orm(string_value = "interrupted")]
    Interrupted,
    /// Ended by the physician's Stop.
    #[sea_orm(string_value = "cancelled")]
    Cancelled,
    /// Ended by a failure.
    #[sea_orm(string_value = "failed")]
    Failed,
    /// Ended at a limit or a provider block; the work is held.
    #[sea_orm(string_value = "held")]
    Held,
    /// Published its result.
    #[sea_orm(string_value = "completed")]
    Completed,
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
    /// Where this attempt stands.
    pub status: AttemptStatus,
    /// Database-clock lease expiry in seconds.
    pub expires: i64,
    /// Ordered acknowledged engine evidence, internal only.
    pub events: Vec<Event>,
}

impl Attempt {
    fn owns_lease(&self, now: i64) -> bool {
        let leased = match self.status {
            AttemptStatus::Active | AttemptStatus::Interrupted | AttemptStatus::Cancelled => true,
            AttemptStatus::Failed | AttemptStatus::Held | AttemptStatus::Completed => false,
        };
        self.expires > now && leased
    }
}

/// Durable conversation and work projection.
#[derive(Clone, Debug)]
pub struct Conversation {
    /// Authored conversation only.
    pub messages: Vec<Message>,
    /// Current accepted input generation.
    pub revision: u64,
    /// Pending or active, otherwise the shared run outcome.
    pub status: WorkStatus,
    /// Current work identity; successors preserve it.
    pub work: String,
    /// Lifecycle facts retained once for each accepted work.
    pub runs: std::collections::BTreeMap<String, RunHistory>,
    /// Identity of the immutable skill package supplied at acceptance.
    pub skills: SkillPackageId,
    /// All retained attempts, including interrupted attempts.
    pub attempts: Vec<Attempt>,
    /// Accepted session reference, absent before acceptance; never the bearer secret.
    pub session_hash: Option<Vec<u8>>,
    /// Database-clock time before which recovery need not republish a wakeup.
    /// This is a delivery hint, never proof of execution or completion.
    pub dispatch_after: i64,
    /// Unsuccessful attempts in the current run, including expired leases.
    /// Only a new run resets it; at [`harness::engine::MAX_FAILED_ATTEMPTS`] the work fails.
    pub failures: u32,
    /// Last provider ordinal excluded by the physician’s explicit fresh allowance.
    pub budget_resume_after: u64,
    /// Each work's immutable recipe, retained once even after a later message.
    pub bindings: std::collections::BTreeMap<String, BindingRef>,
    /// Database-clock seconds when the first physician message was accepted.
    pub opened_at: i64,
    /// Database-clock seconds of the latest accepted message or published reply.
    pub last_activity_at: i64,
    /// The latest accepted input's rehearsal scenario; see [`Input::scenario`].
    pub scenario: Option<String>,
}

use harness::engine::{
    AttemptFailure,
    runs::{HoldReason, RunHistory, RunOutcome},
};

/// Ends one attempt under the shared failure rule. Returns the outcome when
/// the attempt also ended the run; `None` leaves the work pending.
fn settle_attempt(
    reason: AttemptFailure,
    failures: &mut u32,
    attempt_status: &mut AttemptStatus,
    runs: &mut RunHistory,
) -> Result<Option<RunOutcome>, RuntimeError> {
    *attempt_status = match reason {
        AttemptFailure::Interrupted | AttemptFailure::Abandoned => AttemptStatus::Interrupted,
        AttemptFailure::Limit | AttemptFailure::ProviderBlocked => AttemptStatus::Held,
        AttemptFailure::Failed => AttemptStatus::Failed,
    };
    *failures += u32::from(reason.counts_as_failure());
    let outcome = RunOutcome::after_attempt(reason, *failures);
    if let Some(outcome) = outcome {
        runs.end(outcome)?;
    }
    Ok(outcome)
}
impl Conversation {
    /// Current authorized messages used to resolve dispatch references.
    pub fn message_sources(&self) -> Vec<engine::ProductMessage> {
        self.messages.iter().map(Message::source).collect()
    }
    /// Ends the last attempt. An attempt that ran an older input than the
    /// conversation's current revision ends as superseded, whatever reason was
    /// reported: that reason belongs to the input it ran, not the newer one.
    fn end_active_attempt(&mut self, reason: AttemptFailure) -> Result<(), RuntimeError> {
        let attempt = self.attempts.last_mut().expect("active attempt");
        let reason = if attempt.revision == self.revision {
            reason
        } else {
            AttemptFailure::from_cause(&engine::Superseded)
        };
        let outcome = settle_attempt(
            reason,
            &mut self.failures,
            &mut attempt.status,
            self.runs
                .get_mut(&self.work)
                .ok_or_else(|| error("missing work lifecycle"))?,
        )?;
        self.status = outcome.map_or(WorkStatus::Pending, WorkStatus::from);
        Ok(())
    }

    /// The current run is independent of worker attempts and survives recovery.
    pub fn run(&self) -> Option<engine::runs::RunState<'_>> {
        self.runs.get(&self.work).and_then(RunHistory::current)
    }

    /// The attempts whose records are the current work's records: every
    /// attempt of this work, across steers. A run and its partial answer read
    /// the same set.
    fn work_attempt_ids(&self) -> Vec<String> {
        self.work_attempts()
            .map(|attempt| attempt.id.clone())
            .collect()
    }

    /// The current work's attempts, oldest first.
    fn work_attempts(&self) -> impl Iterator<Item = &Attempt> {
        self.attempts
            .iter()
            .filter(|attempt| attempt.work == self.work)
    }

    /// The latest attempt, when it belongs to the current work.
    fn latest_work_attempt(&self) -> Option<&Attempt> {
        self.attempts
            .last()
            .filter(|attempt| attempt.work == self.work)
    }

    fn end_run(&mut self, outcome: RunOutcome) -> Result<(), RuntimeError> {
        self.runs
            .get_mut(&self.work)
            .ok_or_else(|| error("missing work lifecycle"))?
            .end(outcome)?;
        self.status = outcome.into();
        Ok(())
    }

    /// Latest accepted physician command, independent of assistant messages.
    pub fn input_id(&self) -> Option<&str> {
        self.messages.iter().rev().find_map(|m| match &m.role {
            MessageRole::User { id, .. } => Some(id.as_str()),
            MessageRole::Assistant { .. } => None,
        })
    }

    /// Coda's published replies in order, each named by its saved id. A
    /// partial answer kept from a run that did not complete is not a
    /// response, and physician messages are not either.
    pub fn responses(&self) -> impl Iterator<Item = Response<'_>> {
        self.messages
            .iter()
            .enumerate()
            .filter_map(|(place, message)| match &message.role {
                MessageRole::Assistant { id, partial: false } => Some(Response {
                    place,
                    id: ResponseId(id.clone()),
                    text: &message.text,
                }),
                MessageRole::Assistant { partial: true, .. } | MessageRole::User { .. } => None,
            })
    }

    /// The published reply named `id`, if this conversation holds one.
    pub fn response(&self, id: &ResponseId) -> Option<Response<'_>> {
        self.responses().find(|response| &response.id == id)
    }
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

impl Conversation {
    fn new(skills: SkillPackageId) -> Self {
        Self {
            messages: vec![],
            revision: 0,
            status: WorkStatus::Completed,
            work: String::new(),
            runs: Default::default(),
            skills,
            attempts: vec![],
            session_hash: None,
            dispatch_after: 0,
            failures: 0,
            budget_resume_after: 0,
            bindings: Default::default(),
            opened_at: 0,
            last_activity_at: 0,
            scenario: None,
        }
    }
}

/// A fenced claim; safe to pass to the shared engine, never authority by itself.
#[derive(Clone, Debug)]
pub struct Claim {
    /// Ownership duration and its renewal cadence.
    pub lease: Lease,
    /// Product scope.
    pub scope: Scope,
    /// Saved attempt identity.
    pub attempt: String,
    /// Input generation.
    pub revision: u64,
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
    /// The work is pending again; publish its owner-specific wakeup.
    Pending {
        /// The accepted input and durable owner to wake.
        delivery: delivery::Delivery,
    },
    /// The exact same Try again was already taken; do not publish again.
    Duplicate,
    /// The named work or run is not the current failed or held work, or a
    /// command field is blank.
    Refused,
}

/// Outcome of a physician's Stop on the current chat work.
#[derive(Debug, PartialEq, Eq)]
pub enum Stopped {
    /// The conversation is held until Resume or newer input. Do not publish a
    /// wakeup.
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

/// A value that did not convert between `from` and `into`, with the
/// conversion's own error (serde's or the integer range check's) kept as the
/// source.
fn conversion_error<E>(from: &'static str, into: &'static str) -> impl FnOnce(E) -> DbErr
where
    E: std::error::Error + Send + Sync + 'static,
{
    move |source| DbErr::TryIntoErr {
        from,
        into,
        source: std::sync::Arc::new(source),
    }
}

/// A failure on the run-lifecycle path of the Chords runtime. Each variant
/// keeps the original error as its source.
#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    /// The database refused or failed the operation.
    #[error("{0}")]
    Database(#[source] DbErr),
    /// The saved run lifecycle refused the transition.
    #[error("{0}")]
    RunLifecycle(#[source] engine::runs::RunError),
    /// The saved records could not be read as a reserved final exchange.
    #[error("{0}")]
    PendingFinal(#[source] engine::Failure),
}

impl From<DbErr> for RuntimeError {
    fn from(error: DbErr) -> Self {
        Self::Database(error)
    }
}

impl From<engine::runs::RunError> for RuntimeError {
    fn from(error: engine::runs::RunError) -> Self {
        Self::RunLifecycle(error)
    }
}

/// Classify only retryable database writes, preserving the original storage cause.
/// Authorization, stale claims, invalid events and decoding errors never retry.
/// A database error can arrive bare or inside a [`RuntimeError`].
pub fn persistence_failure(error: impl Into<engine::Cause>) -> engine::Cause {
    let error = error.into();
    use sea_orm::{ConnAcquireErr, RuntimeErr, SqlxError};
    let database = match error.downcast_ref::<RuntimeError>() {
        Some(RuntimeError::Database(database)) => Some(database),
        Some(RuntimeError::RunLifecycle(_) | RuntimeError::PendingFinal(_)) => None,
        None => error.downcast_ref::<DbErr>(),
    };
    let retry = match database {
        Some(DbErr::ConnectionAcquire(ConnAcquireErr::Timeout)) => true,
        Some(
            DbErr::Conn(RuntimeErr::SqlxError(source))
            | DbErr::Exec(RuntimeErr::SqlxError(source))
            | DbErr::Query(RuntimeErr::SqlxError(source)),
        ) => match source.as_ref() {
            SqlxError::Io(_) | SqlxError::PoolTimedOut => true,
            SqlxError::Database(database) => engine::retryable_sqlstate(database.code().as_deref()),

            _ => false,
        },
        _ => false,
    };
    if retry {
        engine::RetryablePersistence::new(error).into()
    } else {
        error
    }
}

fn keys(scope: &Scope) -> (String, String, String, String) {
    (
        scope.practice.clone(),
        scope.physician.clone(),
        scope.workspace.clone(),
        scope.chat.clone(),
    )
}

/// The conversation's input revision, which a chat claim is checked against.
/// The write fence adds its lock.
fn chat_revision_query(scope: &Scope) -> sea_orm::Select<conversation::Entity> {
    conversation::Entity::find_by_id(keys(scope))
        .select_only()
        .column(conversation::Column::Revision)
}

/// Runs [`chat_revision_query`]. A missing conversation or an unreadable
/// revision is an error.
async fn chat_revision(
    db: &impl ConnectionTrait,
    query: sea_orm::Select<conversation::Entity>,
) -> Result<u64, DbErr> {
    let revision = query
        .into_tuple::<i64>()
        .one(db)
        .await?
        .ok_or_else(|| error("conversation not found"))?;
    u64::try_from(revision).map_err(conversion_error("saved conversation revision", "u64"))
}

/// The chat's last attempt: its id and status. The write fence adds its expiry.
fn last_chat_attempt_query(scope: &Scope) -> sea_orm::Select<entities::chat_attempt::Entity> {
    use entities::chat_attempt::Column as A;
    entities::chat_attempt::Entity::find()
        .select_only()
        .columns([A::Id, A::Status])
        .filter(control::chat_scope(scope))
        .order_by_desc(A::Position)
}

fn conversation_model(
    scope: &Scope,
    state: &Conversation,
) -> Result<conversation::ActiveModel, DbErr> {
    Ok(conversation::ActiveModel {
        practice: Set(scope.practice.clone()),
        physician: Set(scope.physician.clone()),
        workspace: Set(scope.workspace.clone()),
        chat: Set(scope.chat.clone()),
        status: Set(state.status),
        revision: Set(i64::try_from(state.revision)
            .map_err(conversion_error("conversation revision", "i64"))?),
        session_hash: Set(state.session_hash.clone()),
        state: Set(control::conversation_state(state)?),
    })
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

    // Display readback now joins several authoritative rows. One read-only
    // snapshot keeps a concurrent finish from mixing old state with new events,
    // without taking the publication lock.
    async fn snapshot(&self) -> Result<sea_orm::DatabaseTransaction, DbErr> {
        self.db
            .begin_with_config(
                Some(sea_orm::IsolationLevel::RepeatableRead),
                Some(sea_orm::AccessMode::ReadOnly),
            )
            .await
    }

    /// Explicit migration, never invoked by connect. Applies the runtime
    /// chain's pending migrations and records each in
    /// `physician_runtime_migrations`, so a rerun changes nothing. The first
    /// migration creates every runtime table and fails if one already exists.
    /// Once a practice holds real data, storage changes never refuse to start
    /// or wipe it; permanent records are not rewritten to add fields.
    pub async fn migrate(&self) -> Result<(), DbErr> {
        let tx = self.db.begin().await?;
        // PostgreSQL-only migration serialization. The fixed DDL lives in its
        // own SQL file; runtime queries use the owning typed entities.
        tx.execute_unprepared("SELECT pg_advisory_xact_lock(804081)")
            .await?;
        migration::Migrator::up(&tx, None).await?;
        entities::provider_quota::Entity::insert(entities::provider_quota::ActiveModel {
            singleton: Set(true),
            not_before_ms: Set(0),
        })
        .on_conflict(
            OnConflict::column(entities::provider_quota::Column::Singleton)
                .do_nothing()
                .to_owned(),
        )
        .exec_without_returning(&tx)
        .await?;
        tx.commit().await
    }

    async fn locked(
        &self,
        scope: &Scope,
    ) -> Result<(sea_orm::DatabaseTransaction, Conversation, i64), DbErr> {
        scope.validate()?;
        let tx = self.db.begin().await?;
        let row = conversation::Entity::find_by_id(keys(scope))
            .lock_exclusive()
            .one(&tx)
            .await?
            .ok_or_else(|| error("conversation not found"))?;
        let state = Self::conversation_control(&tx, scope, row).await?;
        let now = now(&tx).await?;
        Ok((tx, state, now))
    }

    async fn save(
        tx: sea_orm::DatabaseTransaction,
        scope: &Scope,
        state: &Conversation,
    ) -> Result<(), DbErr> {
        control::save_chat_attempts(&tx, scope, &state.attempts).await?;
        conversation_model(scope, state)?.update(&tx).await?;
        tx.commit().await
    }

    /// Accepts hosted input with its original session reference for restart authorization.
    pub async fn accept_authorized(
        &self,
        scope: &Scope,
        input: &Input,
        session_hash: Vec<u8>,
        skills: &PhysicianSkills,
        recipe: &RequestRecipe,
    ) -> Result<Accepted, RuntimeError> {
        if session_hash.len() != 32 {
            return Err(error("invalid session reference").into());
        }
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
            return Err(error("empty command field").into());
        }
        let tx = self.db.begin().await?;
        let command = serde_json::json!({"scope": scope, "input": input});
        match receipt::record(&tx, &scope.practice, &scope.physician, &input.key, &command).await? {
            receipt::Receipt::New => {}
            receipt::Receipt::Same => {
                tx.commit().await?;
                return Ok(Accepted::Duplicate);
            }
            receipt::Receipt::Different => return Err(error("idempotency collision").into()),
        }
        let package = frozen::keep_skills(&tx, scope, skills).await?;
        conversation::Entity::insert(conversation_model(
            scope,
            &Conversation::new(package.clone()),
        )?)
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .exec_without_returning(&tx)
        .await?;
        let row = conversation::Entity::find_by_id(keys(scope))
            .lock_exclusive()
            .one(&tx)
            .await?
            .ok_or_else(|| error("missing conversation"))?;
        let mut state = Self::conversation_control(&tx, scope, row).await?;
        if !state.status.running() {
            state.work = Uuid::new_v4().to_string();
            let mut runs = RunHistory::default();
            state.failures = runs.start(Uuid::new_v4().to_string())?;
            state.runs.insert(state.work.clone(), runs);
            state.skills = package;
            let recipe = recipe.bind(Some(scope.cache_key("chords-chat")));
            let recipe = frozen::keep_binding(&tx, scope, &recipe).await?;
            state.bindings.insert(state.work.clone(), recipe);
            state.budget_resume_after = 0;
        }
        state.revision += 1;
        state.session_hash = Some(session_hash);
        state.scenario = input.scenario.clone();
        state.dispatch_after = 0;
        let now = now(&tx).await?;
        if state.messages.is_empty() {
            state.opened_at = now;
        }
        state.last_activity_at = now;
        state.messages.push(Message {
            role: MessageRole::User {
                id: input.key.clone(),
                work: state.work.clone(),
                revision: state.revision,
            },
            text: input.text.clone(),
        });
        // New input steers ordinary work. After Stop it is the physician
        // sending something else, which releases the hold and drops Resume.
        state.status = WorkStatus::Pending;
        Self::save(tx, scope, &state).await?;
        Ok(Accepted::New)
    }

    /// Reads authoritative state using the complete tenant/physician binding.
    pub async fn conversation(&self, scope: &Scope) -> Result<Conversation, DbErr> {
        let tx = self.snapshot().await?;
        let mut state = Self::read_conversation(&tx, scope).await?;
        Self::conversation_events(&tx, scope, &mut state).await?;
        tx.commit().await?;
        Ok(state)
    }

    async fn read_conversation(
        db: &impl ConnectionTrait,
        scope: &Scope,
    ) -> Result<Conversation, DbErr> {
        scope.validate()?;
        let row = conversation::Entity::find_by_id(keys(scope))
            .one(db)
            .await?
            .ok_or_else(|| error("conversation not found"))?;
        Self::conversation_control(db, scope, row).await
    }

    /// One unlocked view of authored state, controls and the current work's records.
    /// Earlier work remains available to note discovery through conversation().
    async fn work_snapshot(
        &self,
        scope: &Scope,
    ) -> Result<(Conversation, Vec<(Event, bool)>), DbErr> {
        let tx = self.snapshot().await?;
        let state = Self::read_conversation(&tx, scope).await?;
        let records =
            records::ordered(&tx, scope, records::Owner::Chat, &state.work_attempt_ids()).await?;
        tx.commit().await?;
        Ok((state, records))
    }

    /// Resolve an accepted recipe in this physician's immutable input store.
    pub async fn frozen_binding(
        &self,
        scope: &Scope,
        id: &BindingRef,
    ) -> Result<RequestBinding, DbErr> {
        frozen::binding(&self.db, scope, id).await
    }

    /// Resolve an accepted package, independent of the currently deployed library.
    pub async fn frozen_skills(
        &self,
        scope: &Scope,
        id: &SkillPackageId,
    ) -> Result<PhysicianSkills, DbErr> {
        frozen::skills(&self.db, scope, id).await
    }

    /// Resolve authored messages without publication locks or execution records.
    async fn message_sources(&self, scope: &Scope) -> Result<Vec<engine::ProductMessage>, DbErr> {
        scope.validate()?;
        let row = conversation::Entity::find_by_id(keys(scope))
            .one(&self.db)
            .await?
            .ok_or_else(|| error("conversation not found"))?;
        Ok(control::conversation(row, Vec::new())?.message_sources())
    }

    /// Claims pending work or recovers an expired active attempt as its successor.
    /// Renewable ownership is independent of run time; no two live claims coexist.
    pub async fn claim(
        &self,
        scope: &Scope,
        lease_seconds: i64,
    ) -> Result<Option<Claim>, RuntimeError> {
        self.claim_inner(scope, lease_seconds, None).await
    }

    async fn claim_inner(
        &self,
        scope: &Scope,
        lease_seconds: i64,
        input_id: Option<&str>,
    ) -> Result<Option<Claim>, RuntimeError> {
        let lease = Lease::new(lease_seconds)?;
        let (tx, mut state, now) = self.locked(scope).await?;
        if input_id.is_some_and(|id| state.input_id() != Some(id)) {
            tx.commit().await?;
            return Ok(None);
        }
        // A steer invalidates publication immediately, but the previous worker
        // still owns its running calls until settlement or lease expiry.
        if state
            .attempts
            .last()
            .is_some_and(|attempt| attempt.owns_lease(now))
        {
            tx.commit().await?;
            return Ok(None);
        }
        if state.latest_work_attempt().is_some_and(|attempt| {
            attempt.status == AttemptStatus::Active && attempt.expires <= now
        }) {
            state.end_active_attempt(AttemptFailure::Abandoned)?;
            if state.status == WorkStatus::Failed {
                Self::save(tx, scope, &state).await?;
                return Ok(None);
            }
        }
        if state.status != WorkStatus::Pending {
            tx.commit().await?;
            return Ok(None);
        }
        let id = Uuid::new_v4().to_string();
        state.attempts.push(Attempt {
            id: id.clone(),
            predecessor: state.latest_work_attempt().map(|a| a.id.clone()),
            work: state.work.clone(),
            revision: state.revision,
            status: AttemptStatus::Active,
            expires: lease.expires(now),
            events: vec![],
        });
        state.status = WorkStatus::Active;
        let claim = Claim {
            lease,
            scope: scope.clone(),
            attempt: id,
            revision: state.revision,
        };
        Self::save(tx, scope, &state).await?;
        Ok(Some(claim))
    }

    fn current(state: &Conversation, claim: &Claim, now: i64) -> control::Currentness {
        control::current(
            state
                .attempts
                .last()
                .map(|a| (a.id.as_str(), a.status, a.expires)),
            &claim.attempt,
            state.revision == claim.revision,
            now,
        )
    }

    async fn conversation_control(
        db: &impl ConnectionTrait,
        scope: &Scope,
        row: conversation::Model,
    ) -> Result<Conversation, DbErr> {
        control::conversation(row, control::chat_attempts(db, scope).await?)
    }

    async fn conversation_events(
        db: &impl ConnectionTrait,
        scope: &Scope,
        state: &mut Conversation,
    ) -> Result<(), DbErr> {
        let ids: Vec<_> = state
            .attempts
            .iter()
            .map(|attempt| attempt.id.clone())
            .collect();
        let mut events =
            records::load(db, scope, records::Owner::Chat, &ids, |event, _| event).await?;
        for attempt in &mut state.attempts {
            attempt.events = events.remove(&attempt.id).unwrap_or_default();
        }
        Ok(())
    }

    /// Revalidates the accepted session through the auth owner without taking publication locks.
    pub async fn authorize(
        &self,
        scope: &Scope,
        authority: &mut dyn Authority,
    ) -> Result<(), DbErr> {
        scope.validate()?;
        let tx = self.db.begin().await?;
        let hash = conversation::Entity::find_by_id(keys(scope))
            .select_only()
            .column(conversation::Column::SessionHash)
            .into_tuple::<Option<Vec<u8>>>()
            .one(&tx)
            .await?
            .ok_or_else(|| error("conversation not found"))?;
        authority
            .check(&tx, scope, hash.as_deref())
            .await
            .map_err(|e| error(&e))?;
        tx.commit().await
    }

    /// Whether the claim is still the current attempt: not stopped, not
    /// replaced by a newer message, and not expired. `Err` means currentness
    /// could not be read: the database, or a missing or unreadable
    /// conversation.
    pub async fn is_current(&self, claim: &Claim) -> Result<bool, DbErr> {
        claim.scope.validate()?;
        let tx = self.db.begin().await?;
        let current = Self::lock_claim(&tx, claim).await?;
        tx.commit().await?;
        Ok(current.is_current())
    }

    /// The write fence's read: the rows [`Store::assert_current`] reads, under
    /// the conversation lock, plus the last attempt's expiry and the clock.
    async fn lock_claim(
        tx: &sea_orm::DatabaseTransaction,
        claim: &Claim,
    ) -> Result<control::Currentness, DbErr> {
        let revision =
            chat_revision(tx, chat_revision_query(&claim.scope).lock_exclusive()).await?;
        let attempt = last_chat_attempt_query(&claim.scope)
            .column(entities::chat_attempt::Column::Expires)
            .into_tuple::<(String, AttemptStatus, i64)>()
            .one(tx)
            .await?;
        Ok(control::current(
            attempt
                .as_ref()
                .map(|(id, status, expires)| (id.as_str(), *status, *expires)),
            &claim.attempt,
            revision == claim.revision,
            now(tx).await?,
        ))
    }

    /// Reads, without a lock or the clock, what the saved state says to this
    /// attempt. An attempt that was ended, by Stop or by settlement, is
    /// [`engine::Withdrawn`], even when a newer message waits. A running
    /// attempt with a newer message is [`engine::Superseded`]. An attempt that
    /// another attempt replaced has lost ownership. Lease expiry is checked by
    /// hosting renewal and the write fences. A missing conversation or an
    /// unreadable revision is an error.
    pub async fn assert_current(&self, claim: &Claim) -> Result<(), engine::Cause> {
        claim.scope.validate()?;
        let revision = chat_revision(&self.db, chat_revision_query(&claim.scope)).await?;
        let attempt = last_chat_attempt_query(&claim.scope)
            .into_tuple::<(String, AttemptStatus)>()
            .one(&self.db)
            .await?;
        control::attempt_currency(
            attempt.as_ref().map(|(id, status)| (id.as_str(), *status)),
            &claim.attempt,
            revision == claim.revision,
        )
        .require()
    }

    /// Hosting retains ownership while the attempt executes or drains its calls.
    /// A steer removes publication authority without releasing that ownership.
    pub async fn renew_claim(&self, claim: &Claim) -> Result<(), DbErr> {
        claim.scope.validate()?;
        let (tx, state, now) = self.locked(&claim.scope).await?;
        if !state
            .attempts
            .last()
            .is_some_and(|attempt| attempt.id == claim.attempt && attempt.owns_lease(now))
        {
            return Err(error("attempt ownership lost"));
        }
        use entities::chat_attempt::{Column as C, Entity};
        Entity::update_many()
            .col_expr(C::Expires, Expr::value(claim.lease.expires(now)))
            .filter(control::chat_scope(&claim.scope))
            .filter(C::Id.eq(&claim.attempt))
            .exec(&tx)
            .await?;
        tx.commit().await
    }

    /// Validate dispatch against the recipe saved at acceptance.
    pub async fn admit_binding(
        &self,
        claim: &Claim,
        recipe: &RequestBinding,
    ) -> Result<(), engine::Cause> {
        let (tx, state, now) = self.locked(&claim.scope).await?;
        Self::current(&state, claim, now).require()?;
        if let Some(id) = state.bindings.get(&state.work) {
            let saved = frozen::binding(&tx, &claim.scope, id).await?;
            if &saved != recipe {
                return Err(error("dispatch changed the accepted work recipe").into());
            }
            return Ok(tx.commit().await?);
        }
        Err(error("accepted work recipe is missing").into())
    }

    /// Acknowledges an ordered event only after commit. Exact replay is idempotent.
    pub async fn persist(&self, claim: &Claim, event: Event) -> Result<(), engine::Cause> {
        claim.scope.validate()?;
        if event.attempt_id != claim.attempt {
            return Err(error("event attempt mismatch").into());
        }
        let tx = self.db.begin().await?;
        let current = Self::lock_claim(&tx, claim).await?;
        // Late provider/tool evidence is inspectable truth, not permission to
        // execute or publish. Never acknowledge a stale dispatch or candidate.
        if !event.kind.survives_loss_of_currency() {
            current.require()?;
        }
        let current = current.is_current();
        let blocked = current && matches!(event.kind, engine::EventKind::ProviderBlocked { .. });
        use entities::chat_attempt::Column as A;
        let known = entities::chat_attempt::Entity::find()
            .select_only()
            .column(A::Id)
            .filter(control::chat_scope(&claim.scope))
            .filter(A::Id.eq(claim.attempt.as_str()))
            .filter(A::Revision.eq(
                i64::try_from(claim.revision).map_err(conversion_error("claim revision", "i64"))?,
            ))
            .into_tuple::<String>()
            .one(&tx)
            .await?;
        if known.is_none() {
            return Err(error("unknown attempt").into());
        }
        records::append(&tx, &claim.scope, records::Owner::Chat, &event, current).await?;
        if blocked {
            let row = conversation::Entity::find_by_id(keys(&claim.scope))
                .one(&tx)
                .await?
                .ok_or_else(|| error("conversation not found"))?;
            let mut state = Self::conversation_control(&tx, &claim.scope, row).await?;
            state.end_active_attempt(AttemptFailure::ProviderBlocked)?;
            Ok(Self::save(tx, &claim.scope, &state).await?)
        } else {
            Ok(tx.commit().await?)
        }
    }

    /// Retains the reserved terminal answer as visibly incomplete work. It is
    /// not a published completion and automatic delivery recovery cannot
    /// reopen it; the physician's explicit Try again rearms the same work.
    pub async fn retain_partial_authorized(
        &self,
        claim: &Claim,
        turn: &Turn,
        authority: &mut dyn Authority,
    ) -> Result<(), engine::Cause> {
        let (tx, mut state, _) = self.locked(&claim.scope).await?;
        Self::conversation_events(&tx, &claim.scope, &mut state).await?;
        authority
            .lock_publication(&tx, &claim.scope, state.session_hash.as_deref())
            .await
            .map_err(|e| error(&e))?;
        let now = now(&tx).await?;
        Self::current(&state, claim, now).require()?;
        let attempt = state.attempts.last().unwrap();
        if !matches!(attempt.events.last().map(|e| &e.kind), Some(engine::EventKind::Candidate { text }) if text == &turn.text())
        {
            return Err(error("partial result lacks durable evidence").into());
        }
        let records = records::ordered(
            &tx,
            &claim.scope,
            records::Owner::Chat,
            &state.work_attempt_ids(),
        )
        .await?;
        let recipe = state
            .bindings
            .get(&state.work)
            .ok_or_else(|| error("accepted recipe missing"))?;
        let recipe = frozen::binding(&tx, &claim.scope, recipe).await?;
        if !engine::pending_final(
            &recipe,
            records.iter(),
            state.budget_resume_after,
            &state.message_sources(),
        )
        .map_err(RuntimeError::PendingFinal)?
        .is_some_and(|saved| saved.turn == *turn)
        {
            return Err(error("partial result differs from recorded terminal turn").into());
        }
        let answer = turn.text();
        let attempt = state.attempts.last_mut().unwrap();
        attempt.status = AttemptStatus::Held;
        state.end_run(RunOutcome::Held {
            reason: HoldReason::Limit,
        })?;
        state.last_activity_at = now;
        if !answer.trim().is_empty() {
            state.messages.push(Message {
                role: MessageRole::Assistant {
                    id: format!("{}:reply", claim.attempt),
                    partial: true,
                },
                text: answer,
            });
        }
        Ok(Self::save(tx, &claim.scope, &state).await?)
    }

    /// Locks the real auth owner on this same transaction through reply publication.
    pub async fn commit_authorized(
        &self,
        claim: &Claim,
        turn: &Turn,
        authority: &mut dyn Authority,
    ) -> Result<(), engine::Cause> {
        let (tx, mut state, _) = self.locked(&claim.scope).await?;
        Self::conversation_events(&tx, &claim.scope, &mut state).await?;
        authority
            .lock_publication(&tx, &claim.scope, state.session_hash.as_deref())
            .await
            .map_err(|e| error(&e))?;
        let now = now(&tx).await?;
        Self::current(&state, claim, now).require()?;
        let attempt = state.attempts.last_mut().unwrap();
        if !matches!(attempt.events.last().map(|e| &e.kind), Some(engine::EventKind::Candidate { text }) if text == &turn.text())
        {
            return Err(error("candidate lacks durable evidence").into());
        }
        let terminal = state
            .attempts
            .iter()
            .filter(|saved| saved.revision == claim.revision)
            .flat_map(|saved| &saved.events)
            .rev()
            .find_map(|event| match &event.kind {
                engine::EventKind::ProviderStarted { .. } => Some(None),
                engine::EventKind::ProviderReturned(retained) => Some(Some(retained)),
                engine::EventKind::ProviderObservation { .. }
                | engine::EventKind::ProviderFailed { .. }
                | engine::EventKind::TransportRetry { .. }
                | engine::EventKind::ToolStarted(_)
                | engine::EventKind::ToolReturned { .. }
                | engine::EventKind::ProviderBlocked { .. }
                | engine::EventKind::ContextRejected
                | engine::EventKind::Candidate { .. }
                | engine::EventKind::WindowOpened { .. } => None,
            })
            .flatten()
            .ok_or_else(|| error("candidate lacks a retained terminal turn"))?;
        if terminal != turn {
            return Err(error("completion differs from recorded terminal turn").into());
        }
        let answer = terminal_answer(terminal)?;
        let attempt = state.attempts.last_mut().unwrap();
        attempt.status = AttemptStatus::Completed;
        state.end_run(RunOutcome::Completed)?;
        state.last_activity_at = now;
        state.messages.push(Message {
            role: MessageRole::Assistant {
                id: format!("{}:reply", claim.attempt),
                partial: false,
            },
            text: answer,
        });
        Ok(Self::save(tx, &claim.scope, &state).await?)
    }

    /// Hosted Try again. Resumes only the named work while it is failed or held
    /// and its current run is `expected_run_id`, so a stale page never
    /// restarts newer work and completed work is never reopened. A blank field
    /// is refused. The receipt is saved with the change: the same `key` and
    /// target answers `Duplicate`.
    /// The accepted session moves to the physician's current session, because
    /// the original one may have ended while the work sat failed.
    pub async fn resume_authorized(
        &self,
        scope: &Scope,
        key: &str,
        session_hash: Vec<u8>,
        work_item_id: &str,
        expected_run_id: &str,
    ) -> Result<Resumed, RuntimeError> {
        if session_hash.len() != 32 {
            return Err(error("invalid session reference").into());
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
            return Self::resume_revision(tx, scope, session_hash, work_item_id, expected_run_id)
                .await;
        };
        let mut state = Self::conversation_control(&tx, scope, row).await?;
        let resumable = state.status.resumable()
            && state.work == work_item_id
            && state.run().is_some_and(|run| run.id == expected_run_id);
        let Some(input_id) = state.input_id().filter(|_| resumable).map(str::to_owned) else {
            return Ok(Resumed::Refused);
        };
        Self::conversation_events(&tx, scope, &mut state).await?;
        state.budget_resume_after = engine::recovery::resume_boundary(
            state.work_attempts().flat_map(|attempt| &attempt.events),
        );
        state.failures = state
            .runs
            .get_mut(&state.work)
            .ok_or_else(|| error("missing work lifecycle"))?
            .start(Uuid::new_v4().to_string())?;
        state.status = WorkStatus::Pending;
        state.dispatch_after = 0;
        state.session_hash = Some(session_hash);
        Self::save(tx, scope, &state).await?;
        Ok(Resumed::Pending {
            delivery: delivery::Delivery::chat_turn(scope.clone(), input_id),
        })
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
    /// keeps its receipt. No wakeup is published.
    pub async fn stop_authorized(
        &self,
        scope: &Scope,
        key: &str,
        session_hash: Vec<u8>,
        work_item_id: &str,
    ) -> Result<Stopped, RuntimeError> {
        if session_hash.len() != 32 {
            return Err(error("invalid session reference").into());
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
        let mut state = Self::conversation_control(&tx, scope, row).await?;
        // Another press on work that is already held, from a page that has
        // not caught up or a second tab, finds what it asked for. Nothing
        // changes, so its receipt is not kept.
        if state.status == WorkStatus::Held && state.work == work_item_id {
            return Ok(Stopped::Duplicate);
        }
        // A failed turn stays Try again. Stop does not turn it into a hold.
        if !state.status.running() || state.work != work_item_id {
            return Ok(Stopped::Refused);
        }
        if let Some(attempt) = state
            .attempts
            .last_mut()
            .filter(|attempt| attempt.status == AttemptStatus::Active)
        {
            attempt.status = AttemptStatus::Cancelled;
        }
        state.end_run(RunOutcome::Held {
            reason: HoldReason::Stopped,
        })?;
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
            let chat = row.chat.clone();
            let state = control::conversation(row, vec![])?;
            let Some(first) = state
                .messages
                .iter()
                .find(|m| matches!(m.role, MessageRole::User { .. }))
            else {
                continue;
            };
            chats.push(ChatSummary {
                chat,
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
        let expired = control::expired_chat();
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
                Condition::any().add(C::Status.eq(WorkStatus::Pending)).add(
                    Condition::all()
                        .add(C::Status.eq(WorkStatus::Active))
                        .add(expired),
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
/// the exact retained terminal provider turn, in order. The publication owner
/// validates its provenance and current candidate, including a successor that
/// recovers the same paid result. A terminal answer has no calls or blank text.
fn terminal_answer(turn: &Turn) -> Result<String, DbErr> {
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
    async fn check(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String>;
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

/// Ordinary physician chat on the shared execution loop. The product owns
/// skill admission, workspace access and the retained chart.
pub struct PhysicianHost<A> {
    /// Durable owner.
    pub store: Store,
    /// Fenced attempt.
    pub claim: Claim,
    /// Current authorization owner.
    pub authority: A,
}

impl Message {
    /// Identity and exact authored content from the canonical transcript.
    pub fn source(&self) -> engine::ProductMessage {
        let (id, role) = match &self.role {
            MessageRole::User { id, .. } => (id, engine::history::Role::User),
            MessageRole::Assistant { id, .. } => (id, engine::history::Role::Assistant),
        };
        engine::ProductMessage {
            id: id.clone(),
            role,
            text: self.text.clone(),
        }
    }
    /// A retained terminal answer that did not complete the accepted work.
    /// Physician input is never partial.
    pub fn partial(&self) -> bool {
        matches!(self.role, MessageRole::Assistant { partial: true, .. })
    }
    /// Model-visible history metadata: the partial flag, and authorship as the
    /// role with its identity. Authorship never repeats the partial flag.
    fn history_metadata(&self) -> Value {
        let authorship = match &self.role {
            MessageRole::User { id, work, revision } => {
                serde_json::json!({"role": "user", "id": id, "work": work, "revision": revision})
            }
            MessageRole::Assistant { id, .. } => serde_json::json!({"role": "assistant", "id": id}),
        };
        serde_json::json!({"partial": self.partial(), "authorship": authorship})
    }
    fn items(&self) -> impl Iterator<Item = Item> {
        let authored = Item::ProductMessage(self.source());
        let partial = self.partial().then(engine::history::partial_answer);
        std::iter::once(authored).chain(partial)
    }
}

/// Exact position in the authored transcript, also used by read_conversation.
fn current_work_start(state: &Conversation) -> Result<usize, DbErr> {
    state.messages.iter().position(|message| matches!(&message.role, MessageRole::User { work, .. } if work == &state.work))
        .ok_or_else(|| error("accepted work input is missing"))
}

fn earlier_messages(start: usize) -> Option<Item> {
    (start > 0).then(engine::history::earlier_messages)
}

#[async_trait]
impl<A: Authority> Product for PhysicianHost<A> {
    async fn message_sources(
        &mut self,
    ) -> Result<Option<Vec<engine::ProductMessage>>, engine::Cause> {
        Ok(Some(self.store.message_sources(&self.claim.scope).await?))
    }

    async fn admit_binding(&mut self, recipe: &RequestBinding) -> Result<(), engine::Cause> {
        self.store
            .authorize(&self.claim.scope, &mut self.authority)
            .await?;
        self.store.admit_binding(&self.claim, recipe).await?;
        Ok(())
    }

    fn read_only(&self, name: &str) -> bool {
        crate::physician_chat::read_only(name)
    }

    fn read_result(&mut self, id: &str) -> harness::engine::recipe::ToolFuture<Option<String>> {
        let store = self.store.clone();
        let claim = self.claim.clone();
        let id = id.to_owned();
        Box::pin(async move {
            store.assert_current(&claim).await?;
            let (state, records) = store.work_snapshot(&claim.scope).await?;
            engine::results::resolve(records.iter(), &id, &state.message_sources())
                .map_err(Into::into)
        })
    }
    async fn assert_current(&mut self) -> Result<(), engine::Cause> {
        self.store
            .authorize(&self.claim.scope, &mut self.authority)
            .await?;
        self.store.assert_current(&self.claim).await?;
        Ok(())
    }
    async fn persist(&mut self, event: Event) -> Result<(), engine::Cause> {
        self.store
            .persist(&self.claim, event)
            .await
            .map_err(persistence_failure)?;
        Ok(())
    }
    fn execute(
        &mut self,
        call: &Invocation,
        arguments: Value,
    ) -> harness::engine::recipe::ToolFuture {
        let store = self.store.clone();
        let claim = self.claim.clone();
        let call = call.clone();
        Box::pin(async move {
            store.assert_current(&claim).await?;
            if working_notes::is_tool(&call.name) {
                let state = store.conversation(&claim.scope).await?;
                let note_call = Invocation {
                    id: call.id.clone(),
                    name: call.name.clone(),
                    arguments,
                };
                let attempt = state
                    .attempts
                    .iter()
                    .find(|attempt| attempt.id == claim.attempt)
                    .ok_or("missing hosted attempt")?;
                return working_notes::execute(
                    &attempt.work,
                    &note_call,
                    state.attempts.iter().flat_map(|attempt| {
                        attempt
                            .events
                            .iter()
                            .map(|event| (attempt.work.as_str(), event))
                    }),
                )?
                .ok_or_else(|| "working note tool was not handled".into());
            }
            let (state, records) = store.work_snapshot(&claim.scope).await?;
            if engine::history::is_tool(&call.name) {
                let messages: Vec<_> = state
                    .messages
                    .iter()
                    .map(|message| engine::history::Message {
                        id: message.source().id,
                        role: match message.role {
                            MessageRole::User { .. } => engine::history::Role::User,
                            MessageRole::Assistant { .. } => engine::history::Role::Assistant,
                        },
                        text: message.text.clone(),
                        metadata: message.history_metadata(),
                    })
                    .collect();
                let page = match engine::history::execute(
                    &call.name,
                    &arguments,
                    &messages,
                    records.iter(),
                ) {
                    Ok(page) => page,
                    Err(reason) => return Ok(ToolOutcome::refused(reason)),
                };
                return Ok(ToolOutcome::History(page));
            }
            if matches!(call.name.as_str(), SEARCH_TOOL | READ_TOOL) {
                let chart = match ChartRequest::parse(&call.name, &arguments, ChartScope::Chat) {
                    Ok(chart) => chart,
                    Err(reason) => {
                        return Ok(ToolOutcome::refused(reason.to_string()));
                    }
                };
                let workspace = &claim.scope.workspace;
                let observation = store.chart_observation(workspace, &chart).await?;
                return chart
                    .run(workspace, observation.version, &observation.sources)
                    .map_err(Into::into);
            }
            let skills = store.frozen_skills(&claim.scope, &state.skills).await?;
            Ok(skills.execute(&call.name, &arguments))
        })
    }
    async fn finish(&mut self, turn: &Turn) -> Result<Finish, engine::Cause> {
        self.assert_current().await?;
        self.store
            .commit_authorized(&self.claim, turn, &mut self.authority)
            .await?;
        Ok(Finish::Committed)
    }

    async fn finish_partial(
        &mut self,
        turn: &Turn,
        _backstops: &[engine::LimitBackstop],
    ) -> Result<(), engine::Cause> {
        self.assert_current().await?;
        self.store
            .retain_partial_authorized(&self.claim, turn, &mut self.authority)
            .await?;
        Ok(())
    }

    async fn window_seed(&mut self) -> Result<harness::engine::WindowSeed, engine::Cause> {
        self.assert_current().await?;
        let state = self.store.conversation(&self.claim.scope).await?;
        let start = current_work_start(&state)?;
        let mut seed: Vec<_> = earlier_messages(start).into_iter().collect();
        seed.extend(state.messages[start..].iter().flat_map(Message::items));
        let note_references =
            working_notes::references(state.attempts.iter().flat_map(|attempt| {
                attempt
                    .events
                    .iter()
                    .map(|event| (attempt.work.as_str(), event))
            }))?;
        Ok(engine::WindowSeed {
            items: seed,
            note_references,
        })
    }
}

impl<A: Authority> PhysicianHost<A> {
    /// Hydrates one admitted recipe, then publishes through the product fence.
    pub async fn run<P: Provider>(
        &mut self,
        provider: &mut P,
        limits: engine::RunLimits,
        cancel: &engine::Cancellation,
    ) -> Result<(), engine::Failure> {
        self.assert_current().await.map_err(engine::Failure::Host)?;
        let (state, records) = self
            .store
            .work_snapshot(&self.claim.scope)
            .await
            .map_err(|error| engine::Failure::Host(error.into()))?;
        let recipe = state
            .bindings
            .get(&state.work)
            .ok_or_else(|| engine::Failure::Host("accepted work recipe is missing".into()))?;
        let mut request = self
            .store
            .frozen_binding(&self.claim.scope, recipe)
            .await
            .map_err(|cause| engine::Failure::Host(cause.into()))?
            .request(Vec::new());
        let messages = state.messages.as_slice();
        request.items.clear();
        let mut pending: Vec<engine::AcceptedInput> = Vec::new();
        for message in messages {
            if let MessageRole::User { id, work, .. } = &message.role
                && work == &state.work
            {
                pending.push(engine::AcceptedInput {
                    id: id.clone(),
                    items: Vec::new(),
                });
            }
            let target = match pending.last_mut() {
                Some(input) => &mut input.items,
                None => &mut request.items,
            };
            target.extend(message.items());
        }
        let work_start =
            current_work_start(&state).map_err(|cause| engine::Failure::Host(cause.into()))?;
        let messages = state.message_sources();
        let attempt = self.claim.attempt.clone();
        let work = request.freeze()?.recover(
            &attempt,
            engine::recovery::Records {
                events: records.iter().map(Into::into).collect(),
                allowance_after: state.budget_resume_after,
            },
            engine::recovery::Messages {
                sources: &messages,
                partial: state.messages[work_start..]
                    .iter()
                    .rev()
                    .find(|message| message.partial())
                    .map(|message| message.source().id),
            },
            engine::recovery::Inputs {
                accepted: pending,
                source_feedback: None,
            },
        )?;
        let result = work.run(provider, self, limits, cancel).await;
        result.map(|_| ())
    }
}
