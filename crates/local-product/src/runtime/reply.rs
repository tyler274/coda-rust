//! Hosted physician reply drafts and their revision work.
//!
//! One row per physician reply draft owns the saved text, the accepted
//! revision request, its source observation and its attempts. Every mutation
//! locks that row. Accepting a new revision moves the previous one, with its
//! evidence, to an append-only history row. A revision is accepted against an exact draft revision; a
//! physician save in between makes it stale, and it then never publishes.
//! Publication replaces the draft text and keeps the replaced text for undo.
//! Model output never creates reply work or sends a patient message.
//!
//! Draft scopes reuse [`Scope`]: its `chat` field names the reply work.
use harness::execution::{Item, Record, Request, Response};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set, TransactionTrait,
    sea_query::{Alias, Expr, ExprTrait, OnConflict, extension::postgres::PgExpr},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::delivery::{Delivery, DeliveryKind, DeliveryState, DispatchPage};
use super::entities::{command, reply_draft, reply_revision};
use super::{Authority, MAX_FAILED_ATTEMPTS, Scope, Store, database_clock, error, keys, now};
use crate::physician_sources::SourceRevision;

/// The terminal tool that ends a revision with the complete revised text.
pub const FINISH_TOOL: &str = "finish_reply_revision";

/// A physician's revision command, compared whole for idempotency.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RevisionInput {
    /// Client command identity, scoped to tenant and physician.
    pub key: String,
    /// The draft revision the physician was looking at.
    pub expected_revision: u64,
    /// The physician's revision instruction.
    pub instruction: String,
    /// Physician's timezone.
    pub timezone: String,
}

/// One attempt at the accepted revision, with its retained execution records.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RevisionAttempt {
    /// Attempt identity.
    pub id: String,
    /// Accepted command owning this attempt; retained across later revisions.
    pub input_id: String,
    /// Active, interrupted, failed, or completed.
    pub status: String,
    /// Database-clock lease expiry in seconds.
    pub expires: i64,
    /// Ordered dispatch, observation, exchange and failure records.
    pub records: Vec<Record>,
}

/// An earlier accepted revision, kept with its execution evidence in
/// `physician_runtime_reply_revisions` when the physician accepts another
/// revision on the same draft.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EarlierRevision {
    /// The accepted revision command.
    pub input: RevisionInput,
    /// Draft text that revision started from.
    pub base_text: String,
    /// Source observation pinned for that revision.
    pub sources: Vec<SourceRevision>,
    /// Its final status: completed, failed, or superseded.
    pub status: String,
    /// Its unsuccessful attempts.
    pub failures: u32,
    /// Its attempts and their retained records.
    pub attempts: Vec<RevisionAttempt>,
}

/// Durable physician reply draft and its latest revision work.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Draft {
    /// Saved revision; every save and every published revision increments it.
    pub revision: u64,
    /// Current draft text.
    pub text: String,
    /// The text the latest save or revision replaced.
    pub restore_text: Option<String>,
    /// Revision work: idle, pending, active, completed, failed, or superseded.
    pub status: String,
    /// Latest accepted revision command.
    pub input: Option<RevisionInput>,
    /// Draft text the accepted revision starts from.
    pub base_text: String,
    /// Exact source observation pinned when the revision was accepted.
    pub sources: Vec<SourceRevision>,
    /// Attempts on the latest accepted revision only.
    pub attempts: Vec<RevisionAttempt>,
    /// Accepted session reference, never the bearer secret.
    pub session_hash: Option<Vec<u8>>,
    /// Database-clock time before which recovery need not republish a wakeup.
    pub dispatch_after: i64,
    /// Unsuccessful attempts on the accepted revision, including expired leases.
    pub failures: u32,
}

impl Draft {
    fn opened(text: &str) -> Self {
        Self {
            revision: 1,
            text: text.into(),
            restore_text: None,
            status: "idle".into(),
            input: None,
            base_text: String::new(),
            sources: vec![],
            attempts: vec![],
            session_hash: None,
            dispatch_after: 0,
            failures: 0,
        }
    }

    fn running(&self) -> bool {
        matches!(self.status.as_str(), "pending" | "active")
    }

    /// A save or another revision moved the draft past the accepted request.
    fn stale(&self) -> bool {
        self.input
            .as_ref()
            .is_some_and(|input| input.expected_revision != self.revision)
    }

    fn supersede(&mut self) {
        if let Some(attempt) = self.attempts.last_mut().filter(|a| a.status == "active") {
            attempt.status = "interrupted".into();
        }
        self.status = "superseded".into();
    }
}

/// Outcome of accepting a revision command.
#[derive(Debug, PartialEq, Eq)]
pub enum RevisionAccepted {
    /// Saved; publish a wakeup for it.
    New,
    /// The exact same command was already accepted; do not publish again.
    Duplicate,
    /// No saved draft, a stale revision, or a revision already running.
    Refused,
}

/// A fenced revision attempt and everything the worker needs to run it.
#[derive(Clone, Debug)]
pub struct RevisionClaim {
    /// Draft scope; `chat` names the reply work.
    pub scope: Scope,
    /// Saved attempt identity.
    pub attempt: String,
    /// The accepted revision command.
    pub input: RevisionInput,
    /// Draft text the revision starts from.
    pub base_text: String,
    /// Pinned source observation.
    pub sources: Vec<SourceRevision>,
    /// Attempts saved on this revision before this one.
    pub earlier_attempts: u32,
}

/// Instructions for a reply revision. Revisions never write memory, delegate,
/// send a message or change patient state.
pub const INSTRUCTIONS: &str = "Revise only this physician's saved patient-facing reply according to their instruction. Read exact sources when needed. Never write memory, delegate, send a message, or modify patient state. Finish with finish_reply_revision containing the complete text and review_basis. Preserve wording the instruction does not change.";

/// The provider request for the site's inline revision path. The hosted
/// worker assembles its own request with additional instructions, an output
/// token cap and the expected draft revision. Both expose only exact source
/// reads and terminal revision output. Only pinned source references enter
/// this context; their bytes are read through `read_patient_sources`.
pub fn request(
    model: &str,
    effort: &str,
    saved_draft: &str,
    instruction: &str,
    timezone: &str,
    sources: &[SourceRevision],
) -> Request {
    Request {
        model: model.into(),
        options: serde_json::json!({ "effort": effort }),
        instructions: INSTRUCTIONS.into(),
        tools: vec![
            crate::physician_sources::read_definition(),
            serde_json::json!({
                "type": "function",
                "name": FINISH_TOOL,
                "description": "Return the complete revised reply and source-based review basis; never sends a message.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "text": { "type": "string" },
                        "review_basis": { "type": "string" },
                    },
                    "required": ["text", "review_basis"],
                    "additionalProperties": false,
                },
            }),
        ],
        items: vec![Item::Message {
            role: "user".into(),
            text: serde_json::json!({
                "saved_draft": saved_draft,
                "instruction": instruction,
                "source_references": sources.iter().map(|s| &s.candidate_id).collect::<Vec<_>>(),
                "physician_time_zone": timezone,
            })
            .to_string(),
        }],
    }
}

/// The complete revised text of a valid terminal response.
pub fn revised_text(response: &Response) -> Result<&str, String> {
    let [call] = response.calls.as_slice() else {
        return Err("reply revision requires one terminal call".into());
    };
    if call.name != FINISH_TOOL
        || !call
            .arguments
            .get("review_basis")
            .is_some_and(serde_json::Value::is_string)
    {
        return Err("invalid reply revision terminal".into());
    }
    call.arguments
        .get("text")
        .and_then(serde_json::Value::as_str)
        .filter(|text| !text.trim().is_empty())
        .ok_or_else(|| "missing revised text".into())
}

fn draft_model(scope: &Scope, state: &Draft) -> Result<reply_draft::ActiveModel, DbErr> {
    Ok(reply_draft::ActiveModel {
        practice: Set(scope.practice.clone()),
        physician: Set(scope.physician.clone()),
        workspace: Set(scope.workspace.clone()),
        reply: Set(scope.chat.clone()),
        state: Set(serde_json::to_value(state).map_err(|_| error("invalid reply draft"))?),
    })
}

fn decode(state: serde_json::Value) -> Result<Draft, DbErr> {
    serde_json::from_value(state).map_err(|_| error("corrupt reply draft"))
}

fn nonempty(scope: &Scope) -> Result<(), DbErr> {
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
    Ok(())
}

fn current(state: &Draft, claim: &RevisionClaim, now: i64) -> bool {
    state.status == "active"
        && state.input.as_ref() == Some(&claim.input)
        && !state.stale()
        && state
            .attempts
            .last()
            .is_some_and(|a| a.id == claim.attempt && a.status == "active" && a.expires > now)
}

impl Store {
    async fn locked_draft(
        &self,
        scope: &Scope,
    ) -> Result<(sea_orm::DatabaseTransaction, Draft, i64), DbErr> {
        nonempty(scope)?;
        let tx = self.db.begin().await?;
        let row = reply_draft::Entity::find_by_id(keys(scope))
            .lock_exclusive()
            .one(&tx)
            .await?
            .ok_or_else(|| error("reply draft not found"))?;
        let state = decode(row.state)?;
        let now = now(&tx).await?;
        Ok((tx, state, now))
    }

    async fn save_draft_row(
        tx: sea_orm::DatabaseTransaction,
        scope: &Scope,
        state: &Draft,
    ) -> Result<(), DbErr> {
        draft_model(scope, state)?.update(&tx).await?;
        tx.commit().await
    }

    /// Opens reply work's first saved draft at revision 1. Only the physician
    /// reply owner calls this, when it admits reply work; HTTP saves never
    /// create a draft. An existing draft is left as it is.
    pub async fn open_draft(&self, scope: &Scope, text: &str) -> Result<u64, DbErr> {
        nonempty(scope)?;
        if text.trim().is_empty() {
            return Err(error("empty reply draft"));
        }
        let tx = self.db.begin().await?;
        reply_draft::Entity::insert(draft_model(scope, &Draft::opened(text))?)
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .exec_without_returning(&tx)
            .await?;
        let row = reply_draft::Entity::find_by_id(keys(scope))
            .one(&tx)
            .await?
            .ok_or_else(|| error("missing reply draft"))?;
        tx.commit().await?;
        Ok(decode(row.state)?.revision)
    }

    /// Unlocked display read. `None` when this physician has no such draft.
    pub async fn draft(&self, scope: &Scope) -> Result<Option<Draft>, DbErr> {
        nonempty(scope)?;
        reply_draft::Entity::find_by_id(keys(scope))
            .one(&self.db)
            .await?
            .map(|row| decode(row.state))
            .transpose()
    }

    /// This physician's earlier revisions of one reply draft, oldest first,
    /// each with its pinned sources and every attempt's retained records.
    pub async fn earlier_revisions(&self, scope: &Scope) -> Result<Vec<EarlierRevision>, DbErr> {
        use reply_revision::Column as C;
        nonempty(scope)?;
        reply_revision::Entity::find()
            .filter(C::Practice.eq(scope.practice.as_str()))
            .filter(C::Physician.eq(scope.physician.as_str()))
            .filter(C::Workspace.eq(scope.workspace.as_str()))
            .filter(C::Reply.eq(scope.chat.as_str()))
            .order_by_asc(C::ArchivedAt)
            .order_by_asc(C::Key)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row.state).map_err(|_| error("corrupt earlier revision"))
            })
            .collect()
    }

    /// Saves physician text under an exact revision fence, holding the auth
    /// owner's locks for `session_hash` in the same transaction. A revision
    /// accepted against the previous text is superseded and never publishes.
    pub async fn save_draft(
        &self,
        scope: &Scope,
        expected: u64,
        text: &str,
        session_hash: &[u8],
        authority: &mut dyn Authority,
    ) -> Result<u64, DbErr> {
        let (tx, mut state, _) = self.locked_draft(scope).await?;
        authority
            .lock_publication(&tx, scope, Some(session_hash))
            .await
            .map_err(|e| error(&e))?;
        if state.revision != expected {
            return Err(error("stale draft"));
        }
        state.restore_text = Some(std::mem::replace(&mut state.text, text.into()));
        state.revision += 1;
        if state.running() {
            state.supersede();
        }
        let revision = state.revision;
        Self::save_draft_row(tx, scope, &state).await?;
        Ok(revision)
    }

    /// Saves the receipt and the revision request with its pinned sources.
    /// The same key and command answers `Duplicate`; the same key with a
    /// different command is an idempotency collision. A refused command
    /// leaves no receipt, so the physician may retry it once the draft allows.
    pub async fn accept_revision(
        &self,
        scope: &Scope,
        input: &RevisionInput,
        session_hash: Vec<u8>,
        sources: Vec<SourceRevision>,
    ) -> Result<RevisionAccepted, DbErr> {
        nonempty(scope)?;
        if session_hash.len() != 32 {
            return Err(error("invalid session reference"));
        }
        if [&input.key, &input.instruction, &input.timezone]
            .iter()
            .any(|s| s.trim().is_empty())
        {
            return Err(error("empty command field"));
        }
        let mut ids = std::collections::HashSet::new();
        for source in &sources {
            source.validate().map_err(|e| error(&e))?;
            if source.workspace != scope.workspace || !ids.insert(source.candidate_id.clone()) {
                return Err(error("invalid source observation scope"));
            }
        }
        let tx = self.db.begin().await?;
        let receipt = serde_json::json!({"scope": scope, "revise": input});
        let inserted = command::Entity::insert(command::ActiveModel {
            practice: Set(scope.practice.clone()),
            physician: Set(scope.physician.clone()),
            key: Set(input.key.clone()),
            command: Set(receipt.clone()),
        })
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .exec_without_returning(&tx)
        .await?;
        if inserted == 0 {
            let previous = command::Entity::find_by_id((
                scope.practice.clone(),
                scope.physician.clone(),
                input.key.clone(),
            ))
            .one(&tx)
            .await?
            .ok_or_else(|| error("missing command receipt"))?
            .command;
            if previous != receipt {
                return Err(error("idempotency collision"));
            }
            tx.commit().await?;
            return Ok(RevisionAccepted::Duplicate);
        }
        let Some(row) = reply_draft::Entity::find_by_id(keys(scope))
            .lock_exclusive()
            .one(&tx)
            .await?
        else {
            // Dropping the transaction rolls the receipt back.
            return Ok(RevisionAccepted::Refused);
        };
        let mut state = decode(row.state)?;
        if state.revision != input.expected_revision || state.running() {
            return Ok(RevisionAccepted::Refused);
        }
        // Move the previous revision's request, pinned sources and execution
        // records into the append-only history in this same transaction, so
        // the draft row only ever holds current work.
        if let Some(previous) = state.input.take() {
            let key = previous.key.clone();
            let earlier = EarlierRevision {
                input: previous,
                base_text: std::mem::take(&mut state.base_text),
                sources: std::mem::take(&mut state.sources),
                status: state.status.clone(),
                failures: state.failures,
                attempts: std::mem::take(&mut state.attempts),
            };
            let archived_at = now(&tx).await?;
            // A plain insert: the command receipt makes the key unique per
            // physician, so a conflict means that invariant broke.
            reply_revision::Entity::insert(reply_revision::ActiveModel {
                practice: Set(scope.practice.clone()),
                physician: Set(scope.physician.clone()),
                workspace: Set(scope.workspace.clone()),
                reply: Set(scope.chat.clone()),
                key: Set(key),
                archived_at: Set(archived_at),
                state: Set(serde_json::to_value(&earlier)
                    .map_err(|_| error("invalid earlier revision"))?),
            })
            .exec_without_returning(&tx)
            .await?;
        }
        state.status = "pending".into();
        state.base_text = state.text.clone();
        state.input = Some(input.clone());
        state.sources = sources;
        state.session_hash = Some(session_hash);
        state.dispatch_after = 0;
        state.failures = 0;
        Self::save_draft_row(tx, scope, &state).await?;
        Ok(RevisionAccepted::New)
    }

    /// Claims only the revision named by the wakeup. A stale revision is
    /// superseded; an expired lease counts as a failed attempt.
    pub async fn claim_revision(
        &self,
        delivery: &Delivery,
        lease_seconds: i64,
    ) -> Result<Option<RevisionClaim>, DbErr> {
        if delivery.kind != DeliveryKind::ReplyRevision {
            return Err(error("not a reply revision delivery"));
        }
        if !(1..=600).contains(&lease_seconds) {
            return Err(error("invalid lease duration"));
        }
        let (tx, mut state, now) = self.locked_draft(&delivery.scope).await?;
        let Some(input) = state
            .input
            .clone()
            .filter(|input| input.key == delivery.input_id)
        else {
            tx.commit().await?;
            return Ok(None);
        };
        if state.stale() {
            if state.running() {
                state.supersede();
                Self::save_draft_row(tx, &delivery.scope, &state).await?;
            } else {
                tx.commit().await?;
            }
            return Ok(None);
        }
        if state.status == "active" && state.attempts.last().is_some_and(|a| a.expires <= now) {
            state.attempts.last_mut().unwrap().status = "interrupted".into();
            state.status = "pending".into();
            state.failures += 1;
            if state.failures >= MAX_FAILED_ATTEMPTS {
                state.status = "failed".into();
                Self::save_draft_row(tx, &delivery.scope, &state).await?;
                return Ok(None);
            }
        }
        if state.status != "pending" {
            tx.commit().await?;
            return Ok(None);
        }
        let earlier_attempts = u32::try_from(state.attempts.len()).unwrap_or(u32::MAX);
        let id = Uuid::new_v4().to_string();
        state.attempts.push(RevisionAttempt {
            id: id.clone(),
            input_id: input.key.clone(),
            status: "active".into(),
            expires: now + lease_seconds,
            records: vec![],
        });
        state.status = "active".into();
        let claim = RevisionClaim {
            scope: delivery.scope.clone(),
            attempt: id,
            input,
            base_text: state.base_text.clone(),
            sources: state.sources.clone(),
            earlier_attempts,
        };
        Self::save_draft_row(tx, &delivery.scope, &state).await?;
        Ok(Some(claim))
    }

    /// Revalidates the accepted session using the real auth owner's locks.
    pub async fn authorize_revision(
        &self,
        scope: &Scope,
        authority: &mut dyn Authority,
    ) -> Result<(), DbErr> {
        let (tx, state, _) = self.locked_draft(scope).await?;
        authority
            .lock_publication(&tx, scope, state.session_hash.as_deref())
            .await
            .map_err(|e| error(&e))?;
        tx.commit().await
    }

    /// Rechecks the attempt against a newer save, a newer revision and expiry.
    pub async fn assert_revision_current(&self, claim: &RevisionClaim) -> Result<(), DbErr> {
        let (tx, state, now) = self.locked_draft(&claim.scope).await?;
        if !current(&state, claim, now) {
            return Err(error("revision no longer current"));
        }
        tx.commit().await
    }

    /// Retains one execution record on this attempt. A dispatch needs a
    /// current attempt; late provider evidence is kept even once stale.
    pub async fn record_revision(
        &self,
        claim: &RevisionClaim,
        record: Record,
    ) -> Result<(), DbErr> {
        let (tx, mut state, now) = self.locked_draft(&claim.scope).await?;
        if matches!(record, Record::Dispatch { .. }) && !current(&state, claim, now) {
            return Err(error("stale revision dispatch"));
        }
        let attempt = state
            .attempts
            .iter_mut()
            .find(|a| a.id == claim.attempt)
            .ok_or_else(|| error("unknown revision attempt"))?;
        attempt.records.push(record);
        Self::save_draft_row(tx, &claim.scope, &state).await
    }

    /// Publishes the revised text only if this attempt is still current, its
    /// last retained record is exactly `response`, and the auth owner's locks
    /// for the accepted session hold in the same transaction.
    pub async fn commit_revision(
        &self,
        claim: &RevisionClaim,
        response: &Response,
        authority: &mut dyn Authority,
    ) -> Result<(), DbErr> {
        let text = revised_text(response).map_err(|e| error(&e))?;
        let (tx, mut state, _) = self.locked_draft(&claim.scope).await?;
        authority
            .lock_publication(&tx, &claim.scope, state.session_hash.as_deref())
            .await
            .map_err(|e| error(&e))?;
        let now = now(&tx).await?;
        if !current(&state, claim, now) {
            return Err(error("stale revision completion"));
        }
        let attempt = state.attempts.last_mut().unwrap();
        if !matches!(attempt.records.last(), Some(Record::Exchange { response: retained, .. }) if retained == response)
        {
            return Err(error("revision lacks durable evidence"));
        }
        attempt.status = "completed".into();
        state.status = "completed".into();
        state.restore_text = Some(std::mem::replace(&mut state.text, text.into()));
        state.revision += 1;
        Self::save_draft_row(tx, &claim.scope, &state).await
    }

    /// Releases this attempt for broker retry. The [`MAX_FAILED_ATTEMPTS`]th
    /// failure ends the revision as failed. Never reopens settled work.
    pub async fn retry_revision(&self, claim: &RevisionClaim) -> Result<(), DbErr> {
        let (tx, mut state, _) = self.locked_draft(&claim.scope).await?;
        if state.input.as_ref() == Some(&claim.input)
            && state.status == "active"
            && let Some(attempt) = state.attempts.last_mut().filter(|a| a.id == claim.attempt)
        {
            if attempt.status == "active" {
                attempt.status = "interrupted".into();
            }
            state.failures += 1;
            state.status = if state.failures >= MAX_FAILED_ATTEMPTS {
                "failed"
            } else {
                "pending"
            }
            .into();
        }
        Self::save_draft_row(tx, &claim.scope, &state).await
    }

    pub(super) async fn revision_state(&self, delivery: &Delivery) -> Result<DeliveryState, DbErr> {
        let (tx, state, now) = self.locked_draft(&delivery.scope).await?;
        tx.commit().await?;
        if state.input.as_ref().map(|i| i.key.as_str()) != Some(delivery.input_id.as_str())
            || state.stale()
        {
            return Ok(DeliveryState::Settled);
        }
        Ok(match state.status.as_str() {
            "failed" => DeliveryState::Failed,
            "pending" => DeliveryState::Due,
            "active" if state.attempts.last().is_some_and(|a| a.expires > now) => {
                DeliveryState::Claimed
            }
            "active" => DeliveryState::Due,
            _ => DeliveryState::Settled,
        })
    }

    pub(super) async fn revision_dispatched(
        &self,
        delivery: &Delivery,
        retry_after: i64,
    ) -> Result<(), DbErr> {
        let (tx, mut state, now) = self.locked_draft(&delivery.scope).await?;
        if state.input.as_ref().map(|i| i.key.as_str()) == Some(delivery.input_id.as_str()) {
            state.dispatch_after = now + retry_after;
        }
        Self::save_draft_row(tx, &delivery.scope, &state).await
    }

    /// One keyset page of recoverable revision wakeups: accepted revisions
    /// whose publish never landed and attempts whose lease ran out.
    pub async fn revisions_due(
        &self,
        maximum: usize,
        after: Option<&Scope>,
    ) -> Result<DispatchPage, DbErr> {
        use reply_draft::Column as C;
        if !(1..=100).contains(&maximum) {
            return Err(error("invalid dispatch bounds"));
        }
        let after = after.cloned().unwrap_or(Scope {
            practice: String::new(),
            physician: String::new(),
            workspace: String::new(),
            chat: String::new(),
        });
        let columns = [C::Practice, C::Physician, C::Workspace, C::Reply];
        let status = Expr::col(C::State).cast_json_field("status");
        let expires = Expr::col(C::State)
            .get_json_field("attempts")
            .get_json_field(-1_i32)
            .cast_json_field("expires")
            .cast_as(Alias::new("bigint"));
        let mut query = reply_draft::Entity::find()
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
        let scopes: Vec<Scope> = query
            .into_tuple::<(String, String, String, String)>()
            .all(&self.db)
            .await?
            .into_iter()
            .map(|(practice, physician, workspace, chat)| Scope {
                practice,
                physician,
                workspace,
                chat,
            })
            .collect();
        let next = (scopes.len() == maximum).then(|| scopes.last().unwrap().clone());
        let mut deliveries = Vec::new();
        for scope in scopes {
            let (tx, state, now) = self.locked_draft(&scope).await?;
            if state.dispatch_after <= now
                && state.session_hash.is_some()
                && !state.stale()
                && (state.status == "pending"
                    || (state.status == "active"
                        && state.attempts.last().is_some_and(|a| a.expires <= now)))
                && let Some(input) = state.input
            {
                deliveries.push(Delivery {
                    kind: DeliveryKind::ReplyRevision,
                    scope,
                    input_id: input.key,
                });
            }
            tx.commit().await?;
        }
        Ok(DispatchPage { deliveries, next })
    }
}
