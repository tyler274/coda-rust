//! Typed status and attempt storage. Public readback combines these rows with
//! authored state and, when requested, the appended execution record.
use super::{
    Attempt, AttemptStatus, BindingRef, Conversation, Discusses, Message, QueuedMessage, Scope,
    SkillPackageId, conversion_error,
    entities::{chat_attempt, conversation, reply_draft, reply_revision, revision_attempt},
    error,
    reply::{AcceptedRevision, Draft, EarlierRevision, RevisionAttempt},
};
use crate::physician_sources::SourceRevision;
use harness::engine::runs::RunHistory;
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, Set,
    sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

// Bound ownership by the 600-second push acknowledgement deadline in
// infra/gcp/rust-environments/queue.tf; zero would expire immediately.
const LEASE_SECONDS: std::ops::RangeInclusive<i64> = 1..=600;
// Renew five times per lease, leaving four intervals to survive a stalled renewal.
const LEASE_RENEWAL_DIVISOR: u32 = 5;

/// Lease duration and renewal cadence for a claimed attempt.
/// Renew after one fifth of the duration, leaving four intervals to survive a stall.
#[derive(Clone, Copy, Debug)]
pub struct Lease(i64);

impl Lease {
    /// Validate the bounded ownership duration, independently of run time.
    pub fn new(seconds: i64) -> Result<Self, DbErr> {
        if !LEASE_SECONDS.contains(&seconds) {
            return Err(error("invalid lease duration"));
        }
        Ok(Self(seconds))
    }

    pub(super) fn expires(self, now: i64) -> i64 {
        now + self.0
    }

    /// Hosting renews every fifth of the duration, leaving four intervals for stalls.
    pub fn renewal_interval(self) -> std::time::Duration {
        std::time::Duration::from_secs(self.0 as u64) / LEASE_RENEWAL_DIVISOR
    }
}

/// What the saved state says to one attempt. An ended attempt and new input
/// are separate facts: an attempt that was ended is withdrawn, even when
/// newer input waits, and a running attempt whose input changed is
/// superseded. An attempt that no longer holds the work, or whose lease ran
/// out at a write fence, is unowned, which is an operational failure.
#[derive(Clone, Copy)]
pub(super) enum Currentness {
    Current,
    Superseded,
    Withdrawn,
    Unowned,
}
impl Currentness {
    pub(super) fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
    pub(super) fn require(self) -> Result<(), super::engine::Cause> {
        match self {
            Self::Current => Ok(()),
            Self::Superseded => Err(super::engine::Superseded.into()),
            Self::Withdrawn => Err(super::engine::Withdrawn.into()),
            Self::Unowned => Err(error("attempt ownership lost").into()),
        }
    }
}

/// What the saved state says to the claimed attempt. `last` is the last
/// saved attempt the caller read: for chat, the conversation's last attempt,
/// whatever work it ran; for a reply revision, the last attempt for one
/// instruction. When that attempt is another one, or there is none, the
/// claimed attempt has lost the work. An attempt that was ended, by Stop, a
/// saved draft or settlement, is withdrawn, even when newer input waits: the
/// end wins over new input. A running attempt is current while its input is
/// the same, and superseded once newer input arrived. The caller decides
/// whether the input is the same. The work's status is not read.
pub(super) fn attempt_currency(
    last: Option<(&str, AttemptStatus)>,
    claimed: &str,
    same_input: bool,
) -> Currentness {
    let Some((_, status)) = last.filter(|(id, _)| *id == claimed) else {
        return Currentness::Unowned;
    };
    match (status, same_input) {
        (AttemptStatus::Active, true) => Currentness::Current,
        (AttemptStatus::Active, false) => Currentness::Superseded,
        (
            AttemptStatus::Interrupted
            | AttemptStatus::Cancelled
            | AttemptStatus::Failed
            | AttemptStatus::Held
            | AttemptStatus::Completed,
            _,
        ) => Currentness::Withdrawn,
    }
}

/// The write fence: [`attempt_currency`], and a current attempt must still
/// hold its lease at `now`. Source and session authority is still checked by
/// its product owner.
pub(super) fn current(
    last: Option<(&str, AttemptStatus, i64)>,
    claimed: &str,
    same_input: bool,
    now: i64,
) -> Currentness {
    let currentness = attempt_currency(
        last.map(|(id, status, _)| (id, status)),
        claimed,
        same_input,
    );
    if currentness.is_current() && last.is_some_and(|(_, _, expires)| expires <= now) {
        return Currentness::Unowned;
    }
    currentness
}

fn decode<T: serde::de::DeserializeOwned>(state: Value, from: &'static str) -> Result<T, DbErr> {
    serde_json::from_value(state).map_err(conversion_error(from, std::any::type_name::<T>()))
}

/// The conversation row's `state`: each [`Conversation`] field that has no
/// column or rows of its own.
#[derive(Serialize, Deserialize)]
struct ConversationState {
    messages: Vec<Message>,
    work: String,
    runs: BTreeMap<String, RunHistory>,
    skills: SkillPackageId,
    dispatch_after: i64,
    failures: u32,
    budget_resume_after: u64,
    bindings: BTreeMap<String, BindingRef>,
    opened_at: i64,
    last_activity_at: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    scenario: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    queued: Vec<QueuedMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    archived_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    discusses: Option<Discusses>,
}

/// The reply draft row's `state`: each [`Draft`] field that has no column or
/// rows of its own.
#[derive(Serialize, Deserialize)]
struct DraftState {
    text: String,
    restore_text: Option<String>,
    accepted: Option<AcceptedRevision>,
    runs: RunHistory,
    base_text: String,
    sources: Vec<SourceRevision>,
    sources_version: u64,
    allowance_after: u64,
    dispatch_after: i64,
    failures: u32,
}

/// The earlier revision row's `state`: each [`EarlierRevision`] field except
/// its attempts, which keep their own rows.
#[derive(Serialize, Deserialize)]
struct EarlierState {
    accepted: AcceptedRevision,
    base_text: String,
    sources: Vec<SourceRevision>,
    sources_version: u64,
    runs: RunHistory,
    failures: u32,
}

/// The conversation row's `state`. Status, revision, session and attempts
/// are saved in their own columns and rows.
pub(super) fn conversation_state(conversation: &Conversation) -> Result<Value, DbErr> {
    let Conversation {
        messages,
        revision: _,
        status: _,
        work,
        runs,
        skills,
        attempts: _,
        session_hash: _,
        dispatch_after,
        failures,
        budget_resume_after,
        bindings,
        opened_at,
        last_activity_at,
        scenario,
        queued,
        archived_at,
        discusses,
    } = conversation;
    serde_json::to_value(ConversationState {
        messages: messages.clone(),
        work: work.clone(),
        runs: runs.clone(),
        skills: skills.clone(),
        dispatch_after: *dispatch_after,
        failures: *failures,
        budget_resume_after: *budget_resume_after,
        bindings: bindings.clone(),
        opened_at: *opened_at,
        last_activity_at: *last_activity_at,
        scenario: scenario.clone(),
        queued: queued.clone(),
        archived_at: *archived_at,
        discusses: discusses.clone(),
    })
    .map_err(conversion_error("Conversation", "conversation state"))
}

/// The reply draft row's `state`. Status, revision, session and attempts are
/// saved in their own columns and rows.
pub(super) fn draft_state(draft: &Draft) -> Result<Value, DbErr> {
    let Draft {
        revision: _,
        text,
        restore_text,
        status: _,
        accepted,
        runs,
        base_text,
        sources,
        sources_version,
        allowance_after,
        attempts: _,
        session_hash: _,
        dispatch_after,
        failures,
    } = draft;
    serde_json::to_value(DraftState {
        text: text.clone(),
        restore_text: restore_text.clone(),
        accepted: accepted.clone(),
        runs: runs.clone(),
        base_text: base_text.clone(),
        sources: sources.clone(),
        sources_version: *sources_version,
        allowance_after: *allowance_after,
        dispatch_after: *dispatch_after,
        failures: *failures,
    })
    .map_err(conversion_error("Draft", "reply draft state"))
}

/// The earlier revision row's `state`. Its attempts keep their own rows.
pub(super) fn earlier_state(earlier: &EarlierRevision) -> Result<Value, DbErr> {
    let EarlierRevision {
        accepted,
        base_text,
        sources,
        sources_version,
        runs,
        failures,
        attempts: _,
    } = earlier;
    serde_json::to_value(EarlierState {
        accepted: accepted.clone(),
        base_text: base_text.clone(),
        sources: sources.clone(),
        sources_version: *sources_version,
        runs: runs.clone(),
        failures: *failures,
    })
    .map_err(conversion_error(
        "EarlierRevision",
        "earlier revision state",
    ))
}

pub(super) fn conversation(
    row: conversation::Model,
    attempts: Vec<Attempt>,
) -> Result<Conversation, DbErr> {
    let ConversationState {
        messages,
        work,
        runs,
        skills,
        dispatch_after,
        failures,
        budget_resume_after,
        bindings,
        opened_at,
        last_activity_at,
        scenario,
        queued,
        archived_at,
        discusses,
    } = serde_json::from_value(row.state)
        .map_err(conversion_error("conversation state", "ConversationState"))?;
    let mut conversation = Conversation {
        messages,
        revision: u64::try_from(row.revision)
            .map_err(conversion_error("saved conversation revision", "u64"))?,
        status: row.status,
        work,
        runs,
        skills,
        attempts,
        session_hash: row.session_hash,
        dispatch_after,
        failures,
        budget_resume_after,
        bindings,
        opened_at,
        last_activity_at,
        scenario,
        queued,
        archived_at,
        discusses,
    };
    if let Some(outcome) = conversation.run().and_then(|run| run.outcome) {
        conversation.status = outcome.into();
    }
    Ok(conversation)
}

pub(super) fn draft(
    row: reply_draft::Model,
    attempts: Vec<RevisionAttempt>,
) -> Result<Draft, DbErr> {
    let DraftState {
        text,
        restore_text,
        accepted,
        runs,
        base_text,
        sources,
        sources_version,
        allowance_after,
        dispatch_after,
        failures,
    } = serde_json::from_value(row.state)
        .map_err(conversion_error("reply draft state", "DraftState"))?;
    Ok(Draft {
        revision: u64::try_from(row.revision)
            .map_err(conversion_error("saved draft revision", "u64"))?,
        text,
        restore_text,
        status: row.status,
        accepted,
        runs,
        base_text,
        sources,
        sources_version,
        allowance_after,
        attempts,
        session_hash: row.session_hash,
        dispatch_after,
        failures,
    })
}

pub(super) fn earlier(
    row: reply_revision::Model,
    attempts: Vec<RevisionAttempt>,
) -> Result<EarlierRevision, DbErr> {
    let EarlierState {
        accepted,
        base_text,
        sources,
        sources_version,
        runs,
        failures,
    } = serde_json::from_value(row.state)
        .map_err(conversion_error("earlier revision state", "EarlierState"))?;
    Ok(EarlierRevision {
        accepted,
        base_text,
        sources,
        sources_version,
        runs,
        failures,
        attempts,
    })
}

pub(super) fn chat_scope(scope: &Scope) -> Condition {
    use chat_attempt::Column as C;
    Condition::all()
        .add(C::Practice.eq(scope.practice.as_str()))
        .add(C::Physician.eq(scope.physician.as_str()))
        .add(C::Workspace.eq(scope.workspace.as_str()))
        .add(C::Chat.eq(scope.chat.as_str()))
}

pub(super) fn revision_scope(scope: &Scope) -> Condition {
    use revision_attempt::Column as C;
    Condition::all()
        .add(C::Practice.eq(scope.practice.as_str()))
        .add(C::Physician.eq(scope.physician.as_str()))
        .add(C::Workspace.eq(scope.workspace.as_str()))
        .add(C::Reply.eq(scope.chat.as_str()))
}

pub(super) async fn chat_attempts(
    db: &impl ConnectionTrait,
    scope: &Scope,
) -> Result<Vec<Attempt>, DbErr> {
    chat_attempt::Entity::find()
        .filter(chat_scope(scope))
        .order_by_asc(chat_attempt::Column::Position)
        .all(db)
        .await?
        .into_iter()
        .map(|row| {
            Ok(Attempt {
                id: row.id,
                predecessor: row.predecessor,
                work: row.work,
                revision: u64::try_from(row.revision)
                    .map_err(conversion_error("saved attempt revision", "u64"))?,
                status: row.status,
                expires: row.expires,
                events: vec![],
            })
        })
        .collect()
}

pub(super) async fn revision_attempts(
    db: &impl ConnectionTrait,
    scope: &Scope,
    input: Option<&str>,
) -> Result<Vec<RevisionAttempt>, DbErr> {
    let Some(input) = input else {
        return Ok(vec![]);
    };
    revision_attempt::Entity::find()
        .filter(revision_scope(scope))
        .filter(revision_attempt::Column::InputId.eq(input))
        .order_by_asc(revision_attempt::Column::Position)
        .all(db)
        .await?
        .into_iter()
        .map(|row| {
            Ok(RevisionAttempt {
                id: row.id,
                input_id: row.input_id,
                status: row.status,
                expires: row.expires,
                events: vec![],
                sources_version: u64::try_from(row.sources_version)
                    .map_err(conversion_error("saved source version", "u64"))?,
                sources: decode(row.sources, "revision attempt sources")?,
                refreshes: decode(row.refreshes, "revision attempt refreshes")?,
            })
        })
        .collect()
}

pub(super) fn chat_models(
    scope: &Scope,
    attempts: &[Attempt],
) -> Result<Vec<chat_attempt::ActiveModel>, DbErr> {
    attempts
        .iter()
        .enumerate()
        .map(|(position, attempt)| {
            Ok(chat_attempt::ActiveModel {
                practice: Set(scope.practice.clone()),
                physician: Set(scope.physician.clone()),
                workspace: Set(scope.workspace.clone()),
                chat: Set(scope.chat.clone()),
                id: Set(attempt.id.clone()),
                predecessor: Set(attempt.predecessor.clone()),
                work: Set(attempt.work.clone()),
                revision: Set(i64::try_from(attempt.revision)
                    .map_err(conversion_error("attempt revision", "i64"))?),
                position: Set(
                    i64::try_from(position).map_err(conversion_error("attempt position", "i64"))?
                ),
                status: Set(attempt.status),
                expires: Set(attempt.expires),
            })
        })
        .collect()
}

pub(super) fn revision_models(
    scope: &Scope,
    attempts: &[RevisionAttempt],
) -> Result<Vec<revision_attempt::ActiveModel>, DbErr> {
    attempts
        .iter()
        .enumerate()
        .map(|(position, attempt)| {
            Ok(revision_attempt::ActiveModel {
                practice: Set(scope.practice.clone()),
                physician: Set(scope.physician.clone()),
                workspace: Set(scope.workspace.clone()),
                reply: Set(scope.chat.clone()),
                id: Set(attempt.id.clone()),
                input_id: Set(attempt.input_id.clone()),
                position: Set(
                    i64::try_from(position).map_err(conversion_error("attempt position", "i64"))?
                ),
                status: Set(attempt.status),
                expires: Set(attempt.expires),
                sources_version: Set(i64::try_from(attempt.sources_version)
                    .map_err(conversion_error("source version", "i64"))?),
                sources: Set(
                    serde_json::to_value(&attempt.sources).map_err(conversion_error(
                        "Vec<SourceRevision>",
                        "revision attempt sources",
                    ))?,
                ),
                refreshes: Set(serde_json::to_value(&attempt.refreshes).map_err(
                    conversion_error("Vec<SourceRefresh>", "revision attempt refreshes"),
                )?),
            })
        })
        .collect()
}

/// Historical identity and source pins are immutable. State transitions update
/// only status, expiry and recorded source refreshes, never execution evidence.
pub(super) async fn save_chat_attempts(
    tx: &impl ConnectionTrait,
    scope: &Scope,
    attempts: &[Attempt],
) -> Result<(), DbErr> {
    use chat_attempt::Column as C;
    for row in chat_models(scope, attempts)? {
        chat_attempt::Entity::insert(row)
            .on_conflict(
                OnConflict::columns([C::Practice, C::Physician, C::Workspace, C::Chat, C::Id])
                    .update_columns([C::Status, C::Expires])
                    .to_owned(),
            )
            .exec_without_returning(tx)
            .await?;
    }
    Ok(())
}

pub(super) async fn save_revision_attempts(
    tx: &impl ConnectionTrait,
    scope: &Scope,
    attempts: &[RevisionAttempt],
) -> Result<(), DbErr> {
    use revision_attempt::Column as C;
    for row in revision_models(scope, attempts)? {
        revision_attempt::Entity::insert(row)
            .on_conflict(
                OnConflict::columns([C::Practice, C::Physician, C::Workspace, C::Reply, C::Id])
                    .update_columns([C::Status, C::Expires, C::Refreshes])
                    .to_owned(),
            )
            .exec_without_returning(tx)
            .await?;
    }
    Ok(())
}

/// The parent lock gives a chat at most one active attempt; only that attempt
/// can make active work due for recovery. Old interrupted attempts are excluded.
pub(super) fn expired_chat() -> sea_orm::sea_query::SimpleExpr {
    use chat_attempt::Column as A;
    use conversation::Column as C;
    use sea_orm::sea_query::{Expr, ExprTrait, Query};
    Expr::tuple([C::Practice, C::Physician, C::Workspace, C::Chat].map(Expr::col)).in_subquery(
        Query::select()
            .columns([A::Practice, A::Physician, A::Workspace, A::Chat])
            .from(chat_attempt::Entity)
            .and_where(A::Status.eq(AttemptStatus::Active))
            .and_where(Expr::col(A::Expires).lte(super::database_clock()))
            .to_owned(),
    )
}

pub(super) fn expired_revision() -> sea_orm::sea_query::SimpleExpr {
    use reply_draft::Column as C;
    use revision_attempt::Column as A;
    use sea_orm::sea_query::{Expr, ExprTrait, Query};
    Expr::tuple([C::Practice, C::Physician, C::Workspace, C::Reply].map(Expr::col)).in_subquery(
        Query::select()
            .columns([A::Practice, A::Physician, A::Workspace, A::Reply])
            .from(revision_attempt::Entity)
            .and_where(A::Status.eq(AttemptStatus::Active))
            .and_where(Expr::col(A::Expires).lte(super::database_clock()))
            .to_owned(),
    )
}
