//! Durable physician and Coda job follow-ups, one-entry-per-change follow-up
//! Updates records, and transactional wrong-patient source withdrawal fencing.
//!
//! One store owns every follow-up item for `(practice, workspace)` across both
//! signed-in panel physicians and Coda background/chat jobs (`FOLLOW_UP_TOOL`).
//! Every state transition commits atomically with a once-only command receipt
//! and appends a single change record to `physician_runtime_follow_up_updates`.

use super::concerns::{ExactRecordReference, SourceWork, WorkspaceAccess};
use super::{Authority, Scope, Store, conversion_error, error, now, receipt};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, DbErr, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait, Unchanged,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;
use std::fmt;

pub(super) mod follow_up_entity {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_follow_ups")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub id: String,
        #[sea_orm(column_type = "Text")]
        pub summary: String,
        #[sea_orm(column_type = "Text")]
        pub status: String,
        pub due_at: i64,
        #[sea_orm(column_type = "JsonBinary")]
        pub standing_detail: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub records: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub origin: Json,
        pub version: i64,
        pub created_at: i64,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub(super) mod follow_up_update_entity {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_follow_up_updates")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub follow_up_id: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub version: i64,
        #[sea_orm(column_type = "Text")]
        pub kind: String,
        #[sea_orm(column_type = "Text")]
        pub summary: String,
        #[sea_orm(column_type = "Text")]
        pub detail: String,
        pub due_at: i64,
        #[sea_orm(column_type = "JsonBinary")]
        pub actor: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub records: Json,
        pub occurred_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub(super) mod source_withdrawal_entity {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_source_withdrawals")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub withdrawal_id: String,
        pub withdrawal_epoch: i64,
        #[sea_orm(column_type = "JsonBinary")]
        pub candidate_ids: Json,
        #[sea_orm(column_type = "Text", nullable)]
        pub message_id: Option<String>,
        #[sea_orm(column_type = "JsonBinary")]
        pub fenced_work_ids: Json,
        #[sea_orm(column_type = "Text")]
        pub reason: String,
        #[sea_orm(column_type = "Text")]
        pub actor: String,
        #[sea_orm(column_type = "JsonBinary")]
        pub receipt: Json,
        pub withdrawn_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Maximum length of a follow-up summary statement, in characters.
pub const MAX_FOLLOW_UP_SUMMARY_CHARS: usize = 2000;

/// Maximum length of a single-line follow-up change or resolution note, in characters.
pub const MAX_FOLLOW_UP_NOTE_CHARS: usize = 500;

/// Maximum exact chart record references cited on one follow-up.
pub const MAX_FOLLOW_UP_RECORDS: usize = 16;

/// Tool name offered to Coda jobs and chat runs to create, revise, complete, or
/// cancel patient follow-ups through the shared follow-up store.
pub const FOLLOW_UP_TOOL: &str = "patient_follow_up";

const FOLLOW_UP_RECEIPT_ACTOR: &str = "coda:follow-up";
const AUTHORITY_CHAT_SLOT: &str = "follow-ups";

/// Whether `name` is [`FOLLOW_UP_TOOL`].
pub fn is_follow_up_tool(name: &str) -> bool {
    name == FOLLOW_UP_TOOL
}

/// Validation failure on a follow-up identifier, command key, or tool payload.
#[derive(Debug, thiserror::Error)]
pub enum FollowUpInputError {
    /// Required scope or actor field was blank or contained control characters.
    #[error("{0} must be non-empty without control characters")]
    InvalidField(&'static str),
    /// Identifier or command key failed boundary validation.
    #[error("invalid identifier `{0}`: supply 1 to 256 trimmed visible characters")]
    InvalidIdentifier(String),
    /// Summary statement was empty or exceeded [`MAX_FOLLOW_UP_SUMMARY_CHARS`].
    #[error("follow-up summary must be 1 to {MAX_FOLLOW_UP_SUMMARY_CHARS} characters")]
    InvalidSummary,
    /// Change or resolution note was empty, multiline, or exceeded [`MAX_FOLLOW_UP_NOTE_CHARS`].
    #[error(
        "follow-up note must be a single non-empty line of at most {MAX_FOLLOW_UP_NOTE_CHARS} characters"
    )]
    InvalidNote,
    /// Follow-up due timestamp was not positive.
    #[error("follow-up due_at must be a positive Unix timestamp")]
    InvalidDueAt,
    /// Cited record count exceeded [`MAX_FOLLOW_UP_RECORDS`].
    #[error("follow-up may cite at most {MAX_FOLLOW_UP_RECORDS} exact chart record references")]
    TooManyRecords,
    /// Two cited references named the same candidate with different revisions.
    #[error("cited record `{candidate_id}` appears with conflicting revisions")]
    ConflictingRecordRevision {
        /// Candidate ID cited more than once with different revisions.
        candidate_id: String,
    },
    /// Tool JSON arguments did not match the offered schema.
    #[error(
        "supply a valid operation (`create`, `change`, `complete`, or `cancel`) and its required fields ({0})"
    )]
    MalformedArguments(#[source] serde_json::Error),
    /// Tool invocation named an unknown tool.
    #[error("unknown follow-up tool `{0}`")]
    UnknownTool(String),
}

/// Named violation when an operation attempts to write or revive state derived
/// from a withdrawn wrong-patient source or a fenced pre-withdrawal run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[serde(tag = "violation", rename_all = "snake_case")]
pub enum WithdrawalFenceViolation {
    /// Operation cited a chart/message candidate withdrawn as wrong-patient.
    #[error(
        "record `{candidate_id}` was withdrawn as wrong-patient in `{withdrawal_id}` (epoch {withdrawal_epoch})"
    )]
    WithdrawnCandidate {
        /// Withdrawn candidate identifier.
        candidate_id: String,
        /// Withdrawal receipt identifier.
        withdrawal_id: String,
        /// Monotonic workspace withdrawal epoch.
        withdrawal_epoch: u64,
    },
    /// Operation attributed provenance to a message withdrawn as wrong-patient.
    #[error(
        "message `{message_id}` was withdrawn as wrong-patient in `{withdrawal_id}` (epoch {withdrawal_epoch})"
    )]
    WithdrawnMessage {
        /// Withdrawn message identifier.
        message_id: String,
        /// Withdrawal receipt identifier.
        withdrawal_id: String,
        /// Monotonic workspace withdrawal epoch.
        withdrawal_epoch: u64,
    },
    /// Operation originated from an old in-flight run fenced by a wrong-patient withdrawal.
    #[error(
        "work `{work_id}` predates wrong-patient withdrawal `{withdrawal_id}` (epoch {withdrawal_epoch}) and is fenced"
    )]
    FencedWork {
        /// Stale/in-flight work identifier.
        work_id: String,
        /// Withdrawal receipt identifier.
        withdrawal_id: String,
        /// Monotonic workspace withdrawal epoch.
        withdrawal_epoch: u64,
    },
    /// Operation targeted an item already marked `EnteredInError` / `Withdrawn`.
    #[error(
        "item `{item_id}` was withdrawn as wrong-patient in `{withdrawal_id}` (epoch {withdrawal_epoch}) and cannot be restored"
    )]
    WithdrawnItem {
        /// Withdrawn item identifier.
        item_id: String,
        /// Withdrawal receipt identifier.
        withdrawal_id: String,
        /// Monotonic workspace withdrawal epoch.
        withdrawal_epoch: u64,
    },
}

/// Failure on a follow-up read or mutation.
#[derive(Debug, thiserror::Error)]
pub enum FollowUpError {
    /// Input failed boundary validation.
    #[error("{0}")]
    InvalidInput(#[from] FollowUpInputError),
    /// Physician session or patient panel access was refused.
    #[error("patient workspace access refused: {reason}")]
    AccessDenied {
        /// Refusal reason reported by the authority owner.
        reason: String,
    },
    /// Command key was already used for a different mutation payload.
    #[error("command key `{key}` already recorded a different follow-up command")]
    IdempotencyCollision {
        /// Colliding command key.
        key: String,
    },
    /// Creating a follow-up reused an existing identifier in the workspace.
    #[error("follow-up `{id}` already exists in this patient workspace")]
    AlreadyExists {
        /// Existing follow-up identifier.
        id: FollowUpId,
    },
    /// Target follow-up does not exist in the workspace.
    #[error("follow-up `{id}` does not exist in this patient workspace")]
    NotFound {
        /// Missing follow-up identifier.
        id: FollowUpId,
    },
    /// Target follow-up is in a terminal standing (`Completed`, `Cancelled`, or `WithdrawnWrongPatient`) and never reopens.
    #[error("follow-up `{id}` is already `{terminal}` and cannot reopen")]
    Terminal {
        /// Terminal follow-up identifier.
        id: FollowUpId,
        /// Terminal status label.
        terminal: &'static str,
    },
    /// Operation required a specific non-terminal standing (such as `CannotRestart`).
    #[error("follow-up `{id}` is in `{actual}` standing; operation requires `{expected}`")]
    InvalidTransition {
        /// Follow-up identifier.
        id: FollowUpId,
        /// Expected standing label.
        expected: &'static str,
        /// Actual standing label.
        actual: &'static str,
    },
    /// Cited chart record revision is absent from the workspace's retained sources.
    #[error(
        "cited chart record `{candidate_id}` at revision `{revision}` is not in the workspace's current retained sources"
    )]
    UnverifiedRecord {
        /// Cited candidate ID.
        candidate_id: String,
        /// Cited revision identity.
        revision: String,
    },
    /// Mutation was blocked by a wrong-patient source withdrawal fence.
    #[error("{0}")]
    WithdrawnFence(#[from] WithdrawalFenceViolation),
    /// Database error from SeaORM.
    #[error("{0}")]
    Database(#[from] DbErr),
}

/// Internal error returned by [`check_withdrawal_fence_in_tx`], separating a
/// substantive wrong-patient fence violation from a database or JSON decode failure
/// so callers fail closed without hiding persistence errors.
#[derive(Debug, thiserror::Error)]
pub(super) enum WithdrawalFenceCheckError {
    #[error("{0}")]
    Violation(#[from] WithdrawalFenceViolation),
    #[error("{0}")]
    Database(#[from] DbErr),
}

impl From<WithdrawalFenceCheckError> for FollowUpError {
    fn from(err: WithdrawalFenceCheckError) -> Self {
        match err {
            WithdrawalFenceCheckError::Violation(v) => Self::WithdrawnFence(v),
            WithdrawalFenceCheckError::Database(db) => Self::Database(db),
        }
    }
}

fn valid_token(value: &str) -> bool {
    let count = value.chars().count();
    (1..=256).contains(&count) && value.trim() == value && !value.chars().any(char::is_control)
}

fn validate_non_empty_field(
    field: &'static str,
    value: &str,
) -> Result<String, FollowUpInputError> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.chars().any(char::is_control) {
        return Err(FollowUpInputError::InvalidField(field));
    }
    Ok(trimmed.to_owned())
}

/// Validated follow-up identifier within `(practice, workspace)`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct FollowUpId(String);

impl FollowUpId {
    /// Parses a trimmed, non-empty identifier of 1 to 256 visible characters.
    pub fn new(value: impl Into<String>) -> Result<Self, FollowUpInputError> {
        let value = value.into();
        if !valid_token(&value) {
            return Err(FollowUpInputError::InvalidIdentifier(value));
        }
        Ok(Self(value))
    }

    /// Identifier text as stored.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for FollowUpId {
    type Error = FollowUpInputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<FollowUpId> for String {
    fn from(id: FollowUpId) -> Self {
        id.0
    }
}

impl fmt::Display for FollowUpId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Once-only idempotency key for a follow-up mutation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FollowUpCommandKey(String);

impl FollowUpCommandKey {
    /// Parses a trimmed, non-empty command key of 1 to 256 visible characters.
    pub fn new(value: impl Into<String>) -> Result<Self, FollowUpInputError> {
        let value = value.into();
        if !valid_token(&value) {
            return Err(FollowUpInputError::InvalidIdentifier(value));
        }
        Ok(Self(value))
    }

    /// Key text as stored in `physician_runtime_commands`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FollowUpCommandKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Validated clinical description of what a follow-up watches for.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct FollowUpSummary(String);

impl FollowUpSummary {
    /// Validates a 1 to [`MAX_FOLLOW_UP_SUMMARY_CHARS`] statement.
    pub fn new(value: impl Into<String>) -> Result<Self, FollowUpInputError> {
        let raw = value.into();
        let trimmed = raw.trim();
        let count = trimmed.chars().count();
        if !(1..=MAX_FOLLOW_UP_SUMMARY_CHARS).contains(&count)
            || trimmed
                .chars()
                .any(|ch| ch.is_control() && ch != '\n' && ch != '\r' && ch != '\t')
        {
            return Err(FollowUpInputError::InvalidSummary);
        }
        Ok(Self(trimmed.to_owned()))
    }

    /// Summary text as stored.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for FollowUpSummary {
    type Error = FollowUpInputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<FollowUpSummary> for String {
    fn from(summary: FollowUpSummary) -> Self {
        summary.0
    }
}

/// Single-line explanation recorded for Updates when changing, completing, or
/// cancelling a follow-up.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct FollowUpChangeNote(String);

impl FollowUpChangeNote {
    /// Validates a single non-empty line of at most [`MAX_FOLLOW_UP_NOTE_CHARS`] characters.
    pub fn new(value: impl Into<String>) -> Result<Self, FollowUpInputError> {
        let raw = value.into();
        if raw.contains('\n') || raw.contains('\r') {
            return Err(FollowUpInputError::InvalidNote);
        }
        let trimmed = raw.trim();
        let count = trimmed.chars().count();
        if !(1..=MAX_FOLLOW_UP_NOTE_CHARS).contains(&count) || trimmed.chars().any(char::is_control)
        {
            return Err(FollowUpInputError::InvalidNote);
        }
        Ok(Self(trimmed.to_owned()))
    }

    /// Note text as stored.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for FollowUpChangeNote {
    type Error = FollowUpInputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<FollowUpChangeNote> for String {
    fn from(note: FollowUpChangeNote) -> Self {
        note.0
    }
}

/// Authenticated physician or Coda work execution authoring a follow-up change.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "actor_kind", rename_all = "snake_case")]
pub enum FollowUpActor {
    /// Signed-in panel physician acting on the patient workspace.
    Physician {
        /// Authenticated physician identifier.
        physician_id: String,
    },
    /// Coda background review or conversation run acting through [`FOLLOW_UP_TOOL`].
    CodaWork {
        /// Originating work and attempt provenance.
        source_work: SourceWork,
    },
}

impl FollowUpActor {
    fn validate(&self) -> Result<Self, FollowUpInputError> {
        match self {
            Self::Physician { physician_id } => Ok(Self::Physician {
                physician_id: validate_non_empty_field("physician_id", physician_id)?,
            }),
            Self::CodaWork { source_work } => {
                let checked = source_work
                    .clone()
                    .validate()
                    .map_err(|_| FollowUpInputError::InvalidField("source_work"))?;
                Ok(Self::CodaWork {
                    source_work: checked,
                })
            }
        }
    }

    fn work_id(&self) -> Option<&str> {
        match self {
            Self::Physician { .. } => None,
            Self::CodaWork { source_work } => Some(source_work.work_id.as_str()),
        }
    }
}

/// Active review scheduling mode for an open follow-up.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "schedule_kind", rename_all = "snake_case")]
pub enum FollowUpReviewSchedule {
    /// Eligible for review once `clinical_now >= due_at`.
    Scheduled {
        /// Scheduled review Unix timestamp.
        due_at: i64,
    },
    /// Explicitly marked due for immediate review (`Resume` or `Review now`) on the current version.
    DueNow {
        /// Target due timestamp carried on the follow-up.
        due_at: i64,
        /// Database-clock Unix timestamp when immediate review was requested.
        requested_at: i64,
        /// Physician or operator who requested immediate review.
        requested_by: FollowUpActor,
    },
}

/// Action choices offered to the physician when a follow-up's review exhausted
/// retries and cannot restart automatically.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CannotRestartAction {
    /// Mark the follow-up completed with a resolution note.
    Complete,
    /// Schedule a new follow-up review date.
    NewDate,
    /// Cancel the follow-up with a single-line reason.
    Cancel,
}

/// Closed lifecycle standing of a [`FollowUp`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum FollowUpStanding {
    /// Open and eligible for scheduled or immediate (`DueNow`) review.
    Active {
        /// Scheduled vs immediate review readiness.
        schedule: FollowUpReviewSchedule,
    },
    /// Paused via `Stop`; never dispatches review until explicitly resumed.
    Held {
        /// Due timestamp retained while held.
        paused_due_at: i64,
        /// Database-clock Unix timestamp when stopped.
        held_at: i64,
        /// Physician or actor who stopped the follow-up.
        held_by: FollowUpActor,
        /// Optional one-line pause note.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reason: Option<FollowUpChangeNote>,
    },
    /// Review failed and cannot restart automatically; awaits physician choice of
    /// `Complete`, `NewDate`, or `Cancel`.
    CannotRestart {
        /// Retained due timestamp at the time of failure.
        due_at: i64,
        /// Work identifier of the failed review.
        failed_work_id: String,
        /// Database-clock Unix timestamp when marked non-restartable.
        failed_at: i64,
        /// Physician-legible failure summary.
        reason: String,
    },
    /// Terminal completed standing; never reopens on replay or late attempt completion.
    Completed {
        /// Single-line completion note.
        resolution: FollowUpChangeNote,
        /// Database-clock Unix timestamp when completed.
        completed_at: i64,
        /// Actor that completed the follow-up.
        completed_by: FollowUpActor,
    },
    /// Terminal cancelled standing; never reopens on replay or late attempt completion.
    Cancelled {
        /// Single-line cancellation reason.
        reason: FollowUpChangeNote,
        /// Database-clock Unix timestamp when cancelled.
        cancelled_at: i64,
        /// Actor that cancelled the follow-up.
        cancelled_by: FollowUpActor,
    },
    /// Terminal wrong-patient holdback recorded when a source message was removed.
    WithdrawnWrongPatient {
        /// Withdrawal receipt identifier.
        withdrawal_id: String,
        /// Single-line withdrawal reason.
        reason: FollowUpChangeNote,
        /// Database-clock Unix timestamp when held back.
        withdrawn_at: i64,
    },
}

impl FollowUpStanding {
    /// Stored column discriminator matching the database `CHECK` constraint.
    pub const fn status_str(&self) -> &'static str {
        match self {
            Self::Active { .. } => "active",
            Self::Held { .. } => "held",
            Self::CannotRestart { .. } => "cannot_restart",
            Self::Completed { .. } => "completed",
            Self::Cancelled { .. } => "cancelled",
            Self::WithdrawnWrongPatient { .. } => "withdrawn_wrong_patient",
        }
    }

    /// Whether this follow-up is terminal (`Completed`, `Cancelled`, or `WithdrawnWrongPatient`).
    pub const fn is_terminal(&self) -> Option<&'static str> {
        match self {
            Self::Completed { .. } => Some("completed"),
            Self::Cancelled { .. } => Some("cancelled"),
            Self::WithdrawnWrongPatient { .. } => Some("withdrawn_wrong_patient"),
            Self::Active { .. } | Self::Held { .. } | Self::CannotRestart { .. } => None,
        }
    }

    /// Whether this follow-up's current version is due for review at `clinical_now`.
    pub const fn is_due_at(&self, clinical_now: i64) -> bool {
        match self {
            Self::Active {
                schedule: FollowUpReviewSchedule::DueNow { .. },
            } => true,
            Self::Active {
                schedule: FollowUpReviewSchedule::Scheduled { due_at },
            } => *due_at <= clinical_now,
            Self::Held { .. }
            | Self::CannotRestart { .. }
            | Self::Completed { .. }
            | Self::Cancelled { .. }
            | Self::WithdrawnWrongPatient { .. } => false,
        }
    }

    /// Physician resolution choices when in [`Self::CannotRestart`].
    pub const fn cannot_restart_choices(&self) -> &'static [CannotRestartAction] {
        match self {
            Self::CannotRestart { .. } => &[
                CannotRestartAction::Complete,
                CannotRestartAction::NewDate,
                CannotRestartAction::Cancel,
            ],
            Self::Active { .. }
            | Self::Held { .. }
            | Self::Completed { .. }
            | Self::Cancelled { .. }
            | Self::WithdrawnWrongPatient { .. } => &[],
        }
    }
}

/// Durable follow-up record for `(practice, workspace, id)`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowUp {
    /// Practice tenant owning the patient link.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Stable follow-up identifier.
    pub id: FollowUpId,
    /// Clinical description of what Coda or the physician is watching for.
    pub summary: FollowUpSummary,
    /// Target review due timestamp (Unix seconds).
    pub due_at: i64,
    /// Exact chart record revisions cited by the follow-up, when grounded in chart sources.
    pub records: Vec<ExactRecordReference>,
    /// Actor that originally created the follow-up.
    pub origin: FollowUpActor,
    /// Monotonic version (`1` on creation; incremented on every change).
    pub version: u64,
    /// Database-clock Unix seconds when created.
    pub created_at: i64,
    /// Database-clock Unix seconds when last updated.
    pub updated_at: i64,
    /// Current lifecycle standing.
    pub standing: FollowUpStanding,
}

/// Kind of change entry recorded in `physician_runtime_follow_up_updates`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FollowUpUpdateKind {
    /// Follow-up created.
    Created,
    /// Summary, due date, or cited records revised.
    Changed,
    /// Paused (`Stop`).
    Stopped,
    /// Resumed and marked due for review.
    Resumed,
    /// Marked immediately due for review (`Review now`).
    ReviewRequestedNow,
    /// Marked non-restartable after review failure.
    MarkedCannotRestart,
    /// Completed.
    Completed,
    /// Cancelled.
    Cancelled,
    /// Held back due to wrong-patient source withdrawal.
    WithdrawnWrongPatient,
}

impl FollowUpUpdateKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Changed => "changed",
            Self::Stopped => "stopped",
            Self::Resumed => "resumed",
            Self::ReviewRequestedNow => "review_requested_now",
            Self::MarkedCannotRestart => "marked_cannot_restart",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
            Self::WithdrawnWrongPatient => "withdrawn_wrong_patient",
        }
    }
}

/// One append-only `Updates` change record emitted per follow-up mutation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowUpUpdateEntry {
    /// Practice tenant.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Follow-up identifier.
    pub follow_up_id: FollowUpId,
    /// Follow-up version produced by this change (`1..=N`).
    pub version: u64,
    /// Change category.
    pub kind: FollowUpUpdateKind,
    /// Follow-up summary snapshot after the change.
    pub summary: FollowUpSummary,
    /// Single-line physician-readable change detail for Updates.
    pub detail: String,
    /// Due timestamp snapshot after the change.
    pub due_at: i64,
    /// Actor that performed the change.
    pub actor: FollowUpActor,
    /// Cited exact chart record references after the change.
    pub records: Vec<ExactRecordReference>,
    /// Database-clock Unix seconds when recorded.
    pub occurred_at: i64,
}

/// Outcome of a once-only follow-up command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FollowUpMutation {
    /// New change committed to Postgres.
    Applied(FollowUp),
    /// Exact command key already committed earlier; no second change or Updates entry was written.
    Duplicate(FollowUp),
}

impl FollowUpMutation {
    /// Durable follow-up state after the command.
    pub const fn follow_up(&self) -> &FollowUp {
        match self {
            Self::Applied(item) | Self::Duplicate(item) => item,
        }
    }
}

/// Typed input to create a follow-up.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateFollowUp {
    /// Practice tenant.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// New follow-up identifier.
    pub id: FollowUpId,
    /// Clinical description of what to watch for.
    pub summary: FollowUpSummary,
    /// Target review due Unix timestamp (`> 0`).
    pub due_at: i64,
    /// Optional exact chart record references supporting this follow-up.
    pub records: Vec<ExactRecordReference>,
    /// Authoring physician or Coda job.
    pub actor: FollowUpActor,
}

/// Typed input to revise an open or held follow-up's summary, due date, or records.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChangeFollowUp {
    /// Practice tenant.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Existing follow-up identifier.
    pub id: FollowUpId,
    /// Updated clinical description.
    pub summary: FollowUpSummary,
    /// Updated target review due Unix timestamp (`> 0`).
    pub due_at: i64,
    /// Updated exact chart record references.
    pub records: Vec<ExactRecordReference>,
    /// Single-line explanation of the change for Updates.
    pub note: FollowUpChangeNote,
    /// Authoring physician or Coda job.
    pub actor: FollowUpActor,
}

/// Typed input for a signed-in panel physician creating a follow-up.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicianCreateFollowUp {
    /// New follow-up identifier.
    pub id: FollowUpId,
    /// Clinical description of what to watch for.
    pub summary: FollowUpSummary,
    /// Target review due Unix timestamp (`> 0`).
    pub due_at: i64,
    /// Optional exact chart record references supporting this follow-up.
    pub records: Vec<ExactRecordReference>,
}

/// Typed input for a signed-in panel physician revising an open or held follow-up.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicianChangeFollowUp {
    /// Existing follow-up identifier.
    pub id: FollowUpId,
    /// Updated clinical description.
    pub summary: FollowUpSummary,
    /// Updated target review due Unix timestamp (`> 0`).
    pub due_at: i64,
    /// Updated exact chart record references.
    pub records: Vec<ExactRecordReference>,
    /// Single-line explanation of the change for Updates.
    pub note: FollowUpChangeNote,
}

/// Physician choice resolving a [`FollowUpStanding::CannotRestart`] follow-up.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CannotRestartResolution {
    /// Mark the follow-up completed.
    Complete {
        /// Single-line completion note.
        resolution: FollowUpChangeNote,
    },
    /// Choose a new due date and return the follow-up to scheduled active standing.
    NewDate {
        /// New due Unix timestamp (`> 0`).
        due_at: i64,
        /// Single-line reschedule note.
        note: FollowUpChangeNote,
    },
    /// Cancel the follow-up.
    Cancel {
        /// Single-line cancellation reason.
        reason: FollowUpChangeNote,
    },
}

/// Returned outcome from a background review evaluating a due follow-up.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FollowUpReviewOutcome {
    /// Clinical condition watched by the follow-up is resolved; complete the follow-up.
    Resolved {
        /// Single-line clinical resolution note.
        resolution: FollowUpChangeNote,
    },
    /// Follow-up remains open with a revised summary, next due date, or updated chart citations.
    Revise {
        /// Updated clinical summary.
        summary: FollowUpSummary,
        /// Next scheduled review Unix timestamp (`> 0`).
        due_at: i64,
        /// Updated exact chart record citations.
        records: Vec<ExactRecordReference>,
        /// Single-line explanation of the revision.
        note: FollowUpChangeNote,
    },
    /// Review exhausted retries and cannot restart automatically; surface physician choices.
    CannotRestart {
        /// Failed review work identifier.
        failed_work_id: String,
        /// Physician-legible failure summary.
        reason: String,
    },
}

fn normalize_records(
    records: &[ExactRecordReference],
) -> Result<Vec<ExactRecordReference>, FollowUpInputError> {
    if records.len() > MAX_FOLLOW_UP_RECORDS {
        return Err(FollowUpInputError::TooManyRecords);
    }
    let mut seen: BTreeMap<String, String> = BTreeMap::new();
    let mut out = Vec::with_capacity(records.len());
    for record in records {
        let checked = ExactRecordReference::new(&record.candidate_id, &record.revision)
            .map_err(|_| FollowUpInputError::InvalidField("records"))?;
        if let Some(existing) = seen.get(&checked.candidate_id) {
            if existing != &checked.revision {
                return Err(FollowUpInputError::ConflictingRecordRevision {
                    candidate_id: checked.candidate_id,
                });
            }
            continue;
        }
        seen.insert(checked.candidate_id.clone(), checked.revision.clone());
        out.push(checked);
    }
    Ok(out)
}

async fn verify_cited_records(
    tx: &impl ConnectionTrait,
    workspace: &str,
    records: &[ExactRecordReference],
) -> Result<(), FollowUpError> {
    if records.is_empty() {
        return Ok(());
    }
    let observation = super::sources::observed(tx, workspace).await?;
    for cited in records {
        let present = observation.sources.iter().any(|source| {
            source.workspace == workspace
                && source.candidate_id == cited.candidate_id
                && source.revision == cited.revision
        });
        if !present {
            return Err(FollowUpError::UnverifiedRecord {
                candidate_id: cited.candidate_id.clone(),
                revision: cited.revision.clone(),
            });
        }
    }
    Ok(())
}

fn decode_follow_up(row: follow_up_entity::Model) -> Result<FollowUp, DbErr> {
    let id =
        FollowUpId::new(row.id).map_err(conversion_error("stored follow-up id", "FollowUpId"))?;
    let summary = FollowUpSummary::new(row.summary).map_err(conversion_error(
        "stored follow-up summary",
        "FollowUpSummary",
    ))?;
    let standing: FollowUpStanding = serde_json::from_value(row.standing_detail).map_err(
        conversion_error("stored follow-up standing_detail", "FollowUpStanding"),
    )?;
    if standing.status_str() != row.status {
        return Err(error("corrupt follow-up standing status mismatch"));
    }
    let records: Vec<ExactRecordReference> = serde_json::from_value(row.records).map_err(
        conversion_error("stored follow-up records", "ExactRecordReference"),
    )?;
    let origin: FollowUpActor = serde_json::from_value(row.origin)
        .map_err(conversion_error("stored follow-up origin", "FollowUpActor"))?;
    let version =
        u64::try_from(row.version).map_err(conversion_error("stored follow-up version", "u64"))?;
    if version == 0 {
        return Err(error("corrupt follow-up version 0"));
    }
    Ok(FollowUp {
        practice: row.practice,
        workspace: row.workspace,
        id,
        summary,
        due_at: row.due_at,
        records,
        origin,
        version,
        created_at: row.created_at,
        updated_at: row.updated_at,
        standing,
    })
}

async fn load_follow_up(
    tx: &impl ConnectionTrait,
    practice: &str,
    workspace: &str,
    id: &FollowUpId,
) -> Result<Option<FollowUp>, DbErr> {
    follow_up_entity::Entity::find_by_id((
        practice.to_owned(),
        workspace.to_owned(),
        id.as_str().to_owned(),
    ))
    .one(tx)
    .await?
    .map(decode_follow_up)
    .transpose()
}

async fn append_follow_up_update(
    tx: &DatabaseTransaction,
    follow_up: &FollowUp,
    kind: FollowUpUpdateKind,
    detail: &str,
    actor: &FollowUpActor,
) -> Result<(), DbErr> {
    let version_i64 =
        i64::try_from(follow_up.version).map_err(conversion_error("follow_up.version", "i64"))?;
    let actor_json =
        serde_json::to_value(actor).map_err(conversion_error("FollowUpActor", "json"))?;
    let records_json = serde_json::to_value(&follow_up.records)
        .map_err(conversion_error("ExactRecordReference", "json"))?;
    follow_up_update_entity::Entity::insert(follow_up_update_entity::ActiveModel {
        practice: Set(follow_up.practice.clone()),
        workspace: Set(follow_up.workspace.clone()),
        follow_up_id: Set(follow_up.id.as_str().to_owned()),
        version: Set(version_i64),
        kind: Set(kind.as_str().to_owned()),
        summary: Set(follow_up.summary.as_str().to_owned()),
        detail: Set(detail.to_owned()),
        due_at: Set(follow_up.due_at),
        actor: Set(actor_json),
        records: Set(records_json),
        occurred_at: Set(follow_up.updated_at),
    })
    .exec_without_returning(tx)
    .await?;
    Ok(())
}

fn authority_scope(access: &WorkspaceAccess<'_>) -> Result<Scope, FollowUpError> {
    let practice = validate_non_empty_field("practice", access.practice)?;
    let physician = validate_non_empty_field("physician", access.physician)?;
    let workspace = validate_non_empty_field("workspace", access.workspace)?;
    if access.session_hash.len() != 32 {
        return Err(FollowUpError::AccessDenied {
            reason: "invalid session reference".into(),
        });
    }
    Ok(Scope {
        practice,
        physician,
        workspace,
        chat: AUTHORITY_CHAT_SLOT.into(),
    })
}

async fn verify_physician_access(
    tx: &DatabaseTransaction,
    access: &WorkspaceAccess<'_>,
    authority: &mut dyn Authority,
    lock_publication: bool,
) -> Result<Scope, FollowUpError> {
    let scope = authority_scope(access)?;
    authority
        .check(tx, &scope, Some(access.session_hash))
        .await
        .map_err(|reason| FollowUpError::AccessDenied { reason })?;
    if lock_publication {
        authority
            .lock_publication(tx, &scope, Some(access.session_hash))
            .await
            .map_err(|reason| FollowUpError::AccessDenied { reason })?;
    }
    Ok(scope)
}

/// Single rule owner checking whether a candidate write to durable memory,
/// patient understanding, concerns, or follow-ups violates a wrong-patient
/// withdrawal fence in `(practice, workspace)`. Fails closed on any database
/// or stored JSON decoding error.
pub(super) async fn check_withdrawal_fence_in_tx(
    tx: &impl ConnectionTrait,
    practice: &str,
    workspace: &str,
    cited_candidates: &[String],
    message_id: Option<&str>,
    work_id: Option<&str>,
) -> Result<(), WithdrawalFenceCheckError> {
    let rows = source_withdrawal_entity::Entity::find()
        .filter(source_withdrawal_entity::Column::Practice.eq(practice))
        .filter(source_withdrawal_entity::Column::Workspace.eq(workspace))
        .order_by_asc(source_withdrawal_entity::Column::WithdrawalEpoch)
        .all(tx)
        .await?;

    for row in rows {
        let epoch = u64::try_from(row.withdrawal_epoch).map_err(conversion_error(
            "source_withdrawal.withdrawal_epoch",
            "u64",
        ))?;
        let candidates: Vec<String> = serde_json::from_value(row.candidate_ids).map_err(
            conversion_error("source_withdrawal.candidate_ids", "Vec<String>"),
        )?;
        let fenced_works: Vec<String> = serde_json::from_value(row.fenced_work_ids).map_err(
            conversion_error("source_withdrawal.fenced_work_ids", "Vec<String>"),
        )?;

        if let Some(work) = work_id
            && fenced_works.iter().any(|w| w == work)
        {
            return Err(WithdrawalFenceViolation::FencedWork {
                work_id: work.to_owned(),
                withdrawal_id: row.withdrawal_id,
                withdrawal_epoch: epoch,
            }
            .into());
        }

        if let Some(msg) = message_id
            && row.message_id.as_deref() == Some(msg)
        {
            return Err(WithdrawalFenceViolation::WithdrawnMessage {
                message_id: msg.to_owned(),
                withdrawal_id: row.withdrawal_id,
                withdrawal_epoch: epoch,
            }
            .into());
        }

        for cited in cited_candidates {
            if candidates.iter().any(|c| c == cited) {
                return Err(WithdrawalFenceViolation::WithdrawnCandidate {
                    candidate_id: cited.clone(),
                    withdrawal_id: row.withdrawal_id,
                    withdrawal_epoch: epoch,
                }
                .into());
            }
        }
    }

    Ok(())
}

async fn ensure_not_withdrawn_follow_up(
    tx: &impl ConnectionTrait,
    practice: &str,
    workspace: &str,
    existing: &FollowUp,
) -> Result<(), FollowUpError> {
    if let FollowUpStanding::WithdrawnWrongPatient { withdrawal_id, .. } = &existing.standing {
        let epoch = source_withdrawal_entity::Entity::find_by_id((
            practice.to_owned(),
            workspace.to_owned(),
            withdrawal_id.clone(),
        ))
        .one(tx)
        .await?
        .and_then(|row| u64::try_from(row.withdrawal_epoch).ok())
        .unwrap_or(1);
        return Err(FollowUpError::WithdrawnFence(
            WithdrawalFenceViolation::WithdrawnItem {
                item_id: existing.id.to_string(),
                withdrawal_id: withdrawal_id.clone(),
                withdrawal_epoch: epoch,
            },
        ));
    }
    Ok(())
}

async fn create_follow_up_in_tx(
    tx: &DatabaseTransaction,
    command_key: &FollowUpCommandKey,
    input: &CreateFollowUp,
) -> Result<FollowUpMutation, FollowUpError> {
    let practice = validate_non_empty_field("practice", &input.practice)?;
    let workspace = validate_non_empty_field("workspace", &input.workspace)?;
    if input.due_at <= 0 {
        return Err(FollowUpInputError::InvalidDueAt.into());
    }
    let records = normalize_records(&input.records)?;
    let actor = input.actor.validate()?;
    let cited_candidates: Vec<String> = records.iter().map(|r| r.candidate_id.clone()).collect();

    check_withdrawal_fence_in_tx(
        tx,
        &practice,
        &workspace,
        &cited_candidates,
        None,
        actor.work_id(),
    )
    .await?;

    let command_json = json!({
        "kind": "follow_up_create",
        "practice": &practice,
        "workspace": &workspace,
        "follow_up_id": input.id.as_str(),
        "summary": input.summary.as_str(),
        "due_at": input.due_at,
        "records": &records,
        "actor": &actor,
    });

    match receipt::record(
        tx,
        &practice,
        FOLLOW_UP_RECEIPT_ACTOR,
        command_key.as_str(),
        &command_json,
    )
    .await?
    {
        receipt::Receipt::New => {}
        receipt::Receipt::Same => {
            let existing = load_follow_up(tx, &practice, &workspace, &input.id)
                .await?
                .ok_or_else(|| error("missing follow-up for accepted create receipt"))?;
            ensure_not_withdrawn_follow_up(tx, &practice, &workspace, &existing).await?;
            return Ok(FollowUpMutation::Duplicate(existing));
        }
        receipt::Receipt::Different => {
            return Err(FollowUpError::IdempotencyCollision {
                key: command_key.to_string(),
            });
        }
    }

    if let Some(existing_row) = follow_up_entity::Entity::find_by_id((
        practice.clone(),
        workspace.clone(),
        input.id.as_str().to_owned(),
    ))
    .lock_exclusive()
    .one(tx)
    .await?
    {
        let existing = decode_follow_up(existing_row)?;
        if let Some(terminal) = existing.standing.is_terminal() {
            return Err(FollowUpError::Terminal {
                id: input.id.clone(),
                terminal,
            });
        }
        return Err(FollowUpError::AlreadyExists {
            id: input.id.clone(),
        });
    }

    verify_cited_records(tx, &workspace, &records).await?;
    let timestamp = now(tx).await?;
    let standing = FollowUpStanding::Active {
        schedule: FollowUpReviewSchedule::Scheduled {
            due_at: input.due_at,
        },
    };
    let standing_json =
        serde_json::to_value(&standing).map_err(conversion_error("FollowUpStanding", "json"))?;
    let records_json =
        serde_json::to_value(&records).map_err(conversion_error("ExactRecordReference", "json"))?;
    let origin_json =
        serde_json::to_value(&actor).map_err(conversion_error("FollowUpActor", "json"))?;

    follow_up_entity::Entity::insert(follow_up_entity::ActiveModel {
        practice: Set(practice.clone()),
        workspace: Set(workspace.clone()),
        id: Set(input.id.as_str().to_owned()),
        summary: Set(input.summary.as_str().to_owned()),
        status: Set(standing.status_str().to_owned()),
        due_at: Set(input.due_at),
        standing_detail: Set(standing_json),
        records: Set(records_json),
        origin: Set(origin_json),
        version: Set(1),
        created_at: Set(timestamp),
        updated_at: Set(timestamp),
    })
    .exec_without_returning(tx)
    .await?;

    let created = FollowUp {
        practice,
        workspace,
        id: input.id.clone(),
        summary: input.summary.clone(),
        due_at: input.due_at,
        records,
        origin: actor.clone(),
        version: 1,
        created_at: timestamp,
        updated_at: timestamp,
        standing,
    };
    append_follow_up_update(
        tx,
        &created,
        FollowUpUpdateKind::Created,
        created.summary.as_str(),
        &actor,
    )
    .await?;

    Ok(FollowUpMutation::Applied(created))
}

async fn change_follow_up_in_tx(
    tx: &DatabaseTransaction,
    command_key: &FollowUpCommandKey,
    input: &ChangeFollowUp,
) -> Result<FollowUpMutation, FollowUpError> {
    let practice = validate_non_empty_field("practice", &input.practice)?;
    let workspace = validate_non_empty_field("workspace", &input.workspace)?;
    if input.due_at <= 0 {
        return Err(FollowUpInputError::InvalidDueAt.into());
    }
    let records = normalize_records(&input.records)?;
    let actor = input.actor.validate()?;
    let cited_candidates: Vec<String> = records.iter().map(|r| r.candidate_id.clone()).collect();

    check_withdrawal_fence_in_tx(
        tx,
        &practice,
        &workspace,
        &cited_candidates,
        None,
        actor.work_id(),
    )
    .await?;

    let command_json = json!({
        "kind": "follow_up_change",
        "practice": &practice,
        "workspace": &workspace,
        "follow_up_id": input.id.as_str(),
        "summary": input.summary.as_str(),
        "due_at": input.due_at,
        "records": &records,
        "note": input.note.as_str(),
        "actor": &actor,
    });

    match receipt::record(
        tx,
        &practice,
        FOLLOW_UP_RECEIPT_ACTOR,
        command_key.as_str(),
        &command_json,
    )
    .await?
    {
        receipt::Receipt::New => {}
        receipt::Receipt::Same => {
            let existing = load_follow_up(tx, &practice, &workspace, &input.id)
                .await?
                .ok_or_else(|| error("missing follow-up for accepted change receipt"))?;
            ensure_not_withdrawn_follow_up(tx, &practice, &workspace, &existing).await?;
            return Ok(FollowUpMutation::Duplicate(existing));
        }
        receipt::Receipt::Different => {
            return Err(FollowUpError::IdempotencyCollision {
                key: command_key.to_string(),
            });
        }
    }

    let Some(row) = follow_up_entity::Entity::find_by_id((
        practice.clone(),
        workspace.clone(),
        input.id.as_str().to_owned(),
    ))
    .lock_exclusive()
    .one(tx)
    .await?
    else {
        return Err(FollowUpError::NotFound {
            id: input.id.clone(),
        });
    };
    let current = decode_follow_up(row.clone())?;
    if let Some(terminal) = current.standing.is_terminal() {
        return Err(FollowUpError::Terminal {
            id: input.id.clone(),
            terminal,
        });
    }

    verify_cited_records(tx, &workspace, &records).await?;
    let timestamp = now(tx).await?;
    let next_version = current.version + 1;
    let next_version_i64 =
        i64::try_from(next_version).map_err(conversion_error("follow_up.version", "i64"))?;

    let next_standing = match current.standing {
        FollowUpStanding::Held {
            held_at,
            held_by,
            reason,
            ..
        } => FollowUpStanding::Held {
            paused_due_at: input.due_at,
            held_at,
            held_by,
            reason,
        },
        FollowUpStanding::Active { .. } => FollowUpStanding::Active {
            schedule: FollowUpReviewSchedule::Scheduled {
                due_at: input.due_at,
            },
        },
        FollowUpStanding::CannotRestart { .. } => {
            return Err(FollowUpError::InvalidTransition {
                id: input.id.clone(),
                expected: "active_or_held",
                actual: "cannot_restart",
            });
        }
        FollowUpStanding::Completed { .. } => {
            return Err(FollowUpError::Terminal {
                id: input.id.clone(),
                terminal: "completed",
            });
        }
        FollowUpStanding::Cancelled { .. } => {
            return Err(FollowUpError::Terminal {
                id: input.id.clone(),
                terminal: "cancelled",
            });
        }
        FollowUpStanding::WithdrawnWrongPatient { .. } => {
            return Err(FollowUpError::Terminal {
                id: input.id.clone(),
                terminal: "withdrawn_wrong_patient",
            });
        }
    };

    let standing_json = serde_json::to_value(&next_standing)
        .map_err(conversion_error("FollowUpStanding", "json"))?;
    let records_json =
        serde_json::to_value(&records).map_err(conversion_error("ExactRecordReference", "json"))?;

    follow_up_entity::ActiveModel {
        practice: Unchanged(row.practice),
        workspace: Unchanged(row.workspace),
        id: Unchanged(row.id),
        summary: Set(input.summary.as_str().to_owned()),
        status: Set(next_standing.status_str().to_owned()),
        due_at: Set(input.due_at),
        standing_detail: Set(standing_json),
        records: Set(records_json),
        origin: Unchanged(row.origin),
        version: Set(next_version_i64),
        created_at: Unchanged(row.created_at),
        updated_at: Set(timestamp),
    }
    .update(tx)
    .await?;

    let updated = FollowUp {
        practice,
        workspace,
        id: input.id.clone(),
        summary: input.summary.clone(),
        due_at: input.due_at,
        records,
        origin: current.origin,
        version: next_version,
        created_at: current.created_at,
        updated_at: timestamp,
        standing: next_standing,
    };
    append_follow_up_update(
        tx,
        &updated,
        FollowUpUpdateKind::Changed,
        input.note.as_str(),
        &actor,
    )
    .await?;

    Ok(FollowUpMutation::Applied(updated))
}

#[derive(Clone, Debug)]
enum TransitionSpec {
    Stop {
        reason: Option<FollowUpChangeNote>,
    },
    Resume,
    ReviewNow,
    Complete {
        resolution: FollowUpChangeNote,
    },
    Cancel {
        reason: FollowUpChangeNote,
    },
    MarkCannotRestart {
        failed_work_id: String,
        reason: String,
    },
    ResolveCannotRestart(CannotRestartResolution),
}

async fn transition_follow_up_in_tx(
    tx: &DatabaseTransaction,
    practice: &str,
    workspace: &str,
    command_key: &FollowUpCommandKey,
    id: &FollowUpId,
    actor: &FollowUpActor,
    spec: TransitionSpec,
) -> Result<FollowUpMutation, FollowUpError> {
    let practice = validate_non_empty_field("practice", practice)?;
    let workspace = validate_non_empty_field("workspace", workspace)?;
    let actor = actor.validate()?;

    check_withdrawal_fence_in_tx(tx, &practice, &workspace, &[], None, actor.work_id()).await?;

    let command_tag = match &spec {
        TransitionSpec::Stop { reason } => json!({ "op": "stop", "reason": reason }),
        TransitionSpec::Resume => json!({ "op": "resume" }),
        TransitionSpec::ReviewNow => json!({ "op": "review_now" }),
        TransitionSpec::Complete { resolution } => {
            json!({ "op": "complete", "resolution": resolution })
        }
        TransitionSpec::Cancel { reason } => json!({ "op": "cancel", "reason": reason }),
        TransitionSpec::MarkCannotRestart {
            failed_work_id,
            reason,
        } => json!({
            "op": "cannot_restart",
            "failed_work_id": failed_work_id,
            "reason": reason,
        }),
        TransitionSpec::ResolveCannotRestart(res) => match res {
            CannotRestartResolution::Complete { resolution } => {
                json!({ "op": "resolve_cannot_restart_complete", "resolution": resolution })
            }
            CannotRestartResolution::NewDate { due_at, note } => {
                json!({ "op": "resolve_cannot_restart_new_date", "due_at": due_at, "note": note })
            }
            CannotRestartResolution::Cancel { reason } => {
                json!({ "op": "resolve_cannot_restart_cancel", "reason": reason })
            }
        },
    };
    let command_json = json!({
        "kind": "follow_up_transition",
        "practice": &practice,
        "workspace": &workspace,
        "follow_up_id": id.as_str(),
        "actor": &actor,
        "transition": command_tag,
    });

    match receipt::record(
        tx,
        &practice,
        FOLLOW_UP_RECEIPT_ACTOR,
        command_key.as_str(),
        &command_json,
    )
    .await?
    {
        receipt::Receipt::New => {}
        receipt::Receipt::Same => {
            let existing = load_follow_up(tx, &practice, &workspace, id)
                .await?
                .ok_or_else(|| error("missing follow-up for accepted transition receipt"))?;
            ensure_not_withdrawn_follow_up(tx, &practice, &workspace, &existing).await?;
            return Ok(FollowUpMutation::Duplicate(existing));
        }
        receipt::Receipt::Different => {
            return Err(FollowUpError::IdempotencyCollision {
                key: command_key.to_string(),
            });
        }
    }

    let Some(row) = follow_up_entity::Entity::find_by_id((
        practice.clone(),
        workspace.clone(),
        id.as_str().to_owned(),
    ))
    .lock_exclusive()
    .one(tx)
    .await?
    else {
        return Err(FollowUpError::NotFound { id: id.clone() });
    };
    let current = decode_follow_up(row.clone())?;

    if let Some(terminal) = current.standing.is_terminal() {
        return Err(FollowUpError::Terminal {
            id: id.clone(),
            terminal,
        });
    }

    let timestamp = now(tx).await?;
    let mut next_due_at = current.due_at;

    let (next_standing, update_kind, detail) = match spec {
        TransitionSpec::Stop { reason } => {
            let detail_text = reason
                .as_ref()
                .map_or_else(|| "Follow-up paused".to_owned(), |r| r.as_str().to_owned());
            (
                FollowUpStanding::Held {
                    paused_due_at: current.due_at,
                    held_at: timestamp,
                    held_by: actor.clone(),
                    reason,
                },
                FollowUpUpdateKind::Stopped,
                detail_text,
            )
        }
        TransitionSpec::Resume => {
            if !matches!(current.standing, FollowUpStanding::Held { .. }) {
                return Err(FollowUpError::InvalidTransition {
                    id: id.clone(),
                    expected: "held",
                    actual: current.standing.status_str(),
                });
            }
            (
                FollowUpStanding::Active {
                    schedule: FollowUpReviewSchedule::DueNow {
                        due_at: current.due_at,
                        requested_at: timestamp,
                        requested_by: actor.clone(),
                    },
                },
                FollowUpUpdateKind::Resumed,
                "Follow-up resumed and marked due for review".to_owned(),
            )
        }
        TransitionSpec::ReviewNow => {
            if matches!(current.standing, FollowUpStanding::CannotRestart { .. }) {
                return Err(FollowUpError::InvalidTransition {
                    id: id.clone(),
                    expected: "active_or_held",
                    actual: current.standing.status_str(),
                });
            }
            (
                FollowUpStanding::Active {
                    schedule: FollowUpReviewSchedule::DueNow {
                        due_at: current.due_at,
                        requested_at: timestamp,
                        requested_by: actor.clone(),
                    },
                },
                FollowUpUpdateKind::ReviewRequestedNow,
                "Follow-up marked due for immediate review".to_owned(),
            )
        }
        TransitionSpec::Complete { resolution } => {
            let detail_text = resolution.as_str().to_owned();
            (
                FollowUpStanding::Completed {
                    resolution,
                    completed_at: timestamp,
                    completed_by: actor.clone(),
                },
                FollowUpUpdateKind::Completed,
                detail_text,
            )
        }
        TransitionSpec::Cancel { reason } => {
            let detail_text = reason.as_str().to_owned();
            (
                FollowUpStanding::Cancelled {
                    reason,
                    cancelled_at: timestamp,
                    cancelled_by: actor.clone(),
                },
                FollowUpUpdateKind::Cancelled,
                detail_text,
            )
        }
        TransitionSpec::MarkCannotRestart {
            failed_work_id,
            reason,
        } => {
            let work_trimmed = validate_non_empty_field("failed_work_id", &failed_work_id)?;
            let reason_trimmed = validate_non_empty_field("reason", &reason)?;
            (
                FollowUpStanding::CannotRestart {
                    due_at: current.due_at,
                    failed_work_id: work_trimmed,
                    failed_at: timestamp,
                    reason: reason_trimmed.clone(),
                },
                FollowUpUpdateKind::MarkedCannotRestart,
                reason_trimmed,
            )
        }
        TransitionSpec::ResolveCannotRestart(resolution) => {
            if !matches!(current.standing, FollowUpStanding::CannotRestart { .. }) {
                return Err(FollowUpError::InvalidTransition {
                    id: id.clone(),
                    expected: "cannot_restart",
                    actual: current.standing.status_str(),
                });
            }
            match resolution {
                CannotRestartResolution::Complete { resolution } => {
                    let detail_text = resolution.as_str().to_owned();
                    (
                        FollowUpStanding::Completed {
                            resolution,
                            completed_at: timestamp,
                            completed_by: actor.clone(),
                        },
                        FollowUpUpdateKind::Completed,
                        detail_text,
                    )
                }
                CannotRestartResolution::NewDate { due_at, note } => {
                    if due_at <= 0 {
                        return Err(FollowUpInputError::InvalidDueAt.into());
                    }
                    next_due_at = due_at;
                    (
                        FollowUpStanding::Active {
                            schedule: FollowUpReviewSchedule::Scheduled { due_at },
                        },
                        FollowUpUpdateKind::Changed,
                        note.as_str().to_owned(),
                    )
                }
                CannotRestartResolution::Cancel { reason } => {
                    let detail_text = reason.as_str().to_owned();
                    (
                        FollowUpStanding::Cancelled {
                            reason,
                            cancelled_at: timestamp,
                            cancelled_by: actor.clone(),
                        },
                        FollowUpUpdateKind::Cancelled,
                        detail_text,
                    )
                }
            }
        }
    };

    let next_version = current.version + 1;
    let next_version_i64 =
        i64::try_from(next_version).map_err(conversion_error("follow_up.version", "i64"))?;
    let standing_json = serde_json::to_value(&next_standing)
        .map_err(conversion_error("FollowUpStanding", "json"))?;

    follow_up_entity::ActiveModel {
        practice: Unchanged(row.practice),
        workspace: Unchanged(row.workspace),
        id: Unchanged(row.id),
        summary: Unchanged(row.summary),
        status: Set(next_standing.status_str().to_owned()),
        due_at: Set(next_due_at),
        standing_detail: Set(standing_json),
        records: Unchanged(row.records),
        origin: Unchanged(row.origin),
        version: Set(next_version_i64),
        created_at: Unchanged(row.created_at),
        updated_at: Set(timestamp),
    }
    .update(tx)
    .await?;

    let updated = FollowUp {
        due_at: next_due_at,
        version: next_version,
        updated_at: timestamp,
        standing: next_standing,
        ..current
    };
    append_follow_up_update(tx, &updated, update_kind, &detail, &actor).await?;

    Ok(FollowUpMutation::Applied(updated))
}

impl Store {
    /// Creates a follow-up once under `command_key`.
    pub async fn create_follow_up(
        &self,
        command_key: &FollowUpCommandKey,
        input: &CreateFollowUp,
    ) -> Result<FollowUpMutation, FollowUpError> {
        let tx = self.db.begin().await?;
        let outcome = create_follow_up_in_tx(&tx, command_key, input).await?;
        tx.commit().await?;
        Ok(outcome)
    }

    /// Creates a follow-up as a signed-in panel physician after enforcing patient workspace access.
    pub async fn create_follow_up_for_physician(
        &self,
        access: &WorkspaceAccess<'_>,
        command_key: &FollowUpCommandKey,
        input: &PhysicianCreateFollowUp,
        authority: &mut dyn Authority,
    ) -> Result<FollowUpMutation, FollowUpError> {
        let tx = self.db.begin().await?;
        let scope = match verify_physician_access(&tx, access, authority, true).await {
            Ok(scope) => scope,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        let outcome = create_follow_up_in_tx(
            &tx,
            command_key,
            &CreateFollowUp {
                practice: scope.practice,
                workspace: scope.workspace,
                id: input.id.clone(),
                summary: input.summary.clone(),
                due_at: input.due_at,
                records: input.records.clone(),
                actor: FollowUpActor::Physician {
                    physician_id: scope.physician,
                },
            },
        )
        .await?;
        tx.commit().await?;
        Ok(outcome)
    }

    /// Revises an open or held follow-up once under `command_key`.
    pub async fn change_follow_up(
        &self,
        command_key: &FollowUpCommandKey,
        input: &ChangeFollowUp,
    ) -> Result<FollowUpMutation, FollowUpError> {
        let tx = self.db.begin().await?;
        let outcome = change_follow_up_in_tx(&tx, command_key, input).await?;
        tx.commit().await?;
        Ok(outcome)
    }

    /// Revises an open or held follow-up as a signed-in panel physician after enforcing patient access.
    pub async fn change_follow_up_for_physician(
        &self,
        access: &WorkspaceAccess<'_>,
        command_key: &FollowUpCommandKey,
        input: &PhysicianChangeFollowUp,
        authority: &mut dyn Authority,
    ) -> Result<FollowUpMutation, FollowUpError> {
        let tx = self.db.begin().await?;
        let scope = match verify_physician_access(&tx, access, authority, true).await {
            Ok(scope) => scope,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        let outcome = change_follow_up_in_tx(
            &tx,
            command_key,
            &ChangeFollowUp {
                practice: scope.practice,
                workspace: scope.workspace,
                id: input.id.clone(),
                summary: input.summary.clone(),
                due_at: input.due_at,
                records: input.records.clone(),
                note: input.note.clone(),
                actor: FollowUpActor::Physician {
                    physician_id: scope.physician,
                },
            },
        )
        .await?;
        tx.commit().await?;
        Ok(outcome)
    }

    /// Holds an open follow-up (`Stop`) as an authorized physician so it is not dispatched until resumed.
    pub async fn stop_follow_up_for_physician(
        &self,
        access: &WorkspaceAccess<'_>,
        command_key: &FollowUpCommandKey,
        id: &FollowUpId,
        reason: Option<FollowUpChangeNote>,
        authority: &mut dyn Authority,
    ) -> Result<FollowUpMutation, FollowUpError> {
        let tx = self.db.begin().await?;
        let scope = match verify_physician_access(&tx, access, authority, true).await {
            Ok(scope) => scope,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        let actor = FollowUpActor::Physician {
            physician_id: scope.physician,
        };
        let outcome = transition_follow_up_in_tx(
            &tx,
            &scope.practice,
            &scope.workspace,
            command_key,
            id,
            &actor,
            TransitionSpec::Stop { reason },
        )
        .await?;
        tx.commit().await?;
        Ok(outcome)
    }

    /// Resumes a held follow-up (`Resume`) as an authorized physician, marking its current version due for review.
    pub async fn resume_follow_up_for_physician(
        &self,
        access: &WorkspaceAccess<'_>,
        command_key: &FollowUpCommandKey,
        id: &FollowUpId,
        authority: &mut dyn Authority,
    ) -> Result<FollowUpMutation, FollowUpError> {
        let tx = self.db.begin().await?;
        let scope = match verify_physician_access(&tx, access, authority, true).await {
            Ok(scope) => scope,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        let actor = FollowUpActor::Physician {
            physician_id: scope.physician,
        };
        let outcome = transition_follow_up_in_tx(
            &tx,
            &scope.practice,
            &scope.workspace,
            command_key,
            id,
            &actor,
            TransitionSpec::Resume,
        )
        .await?;
        tx.commit().await?;
        Ok(outcome)
    }

    /// Marks an open or held follow-up's current version immediately due for review (`Review now`).
    pub async fn review_follow_up_now_for_physician(
        &self,
        access: &WorkspaceAccess<'_>,
        command_key: &FollowUpCommandKey,
        id: &FollowUpId,
        authority: &mut dyn Authority,
    ) -> Result<FollowUpMutation, FollowUpError> {
        let tx = self.db.begin().await?;
        let scope = match verify_physician_access(&tx, access, authority, true).await {
            Ok(scope) => scope,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        let actor = FollowUpActor::Physician {
            physician_id: scope.physician,
        };
        let outcome = transition_follow_up_in_tx(
            &tx,
            &scope.practice,
            &scope.workspace,
            command_key,
            id,
            &actor,
            TransitionSpec::ReviewNow,
        )
        .await?;
        tx.commit().await?;
        Ok(outcome)
    }

    /// Completes a follow-up as an authorized physician. Terminal: never reopens on replay or late completion.
    pub async fn complete_follow_up_for_physician(
        &self,
        access: &WorkspaceAccess<'_>,
        command_key: &FollowUpCommandKey,
        id: &FollowUpId,
        resolution: FollowUpChangeNote,
        authority: &mut dyn Authority,
    ) -> Result<FollowUpMutation, FollowUpError> {
        let tx = self.db.begin().await?;
        let scope = match verify_physician_access(&tx, access, authority, true).await {
            Ok(scope) => scope,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        let actor = FollowUpActor::Physician {
            physician_id: scope.physician,
        };
        let outcome = transition_follow_up_in_tx(
            &tx,
            &scope.practice,
            &scope.workspace,
            command_key,
            id,
            &actor,
            TransitionSpec::Complete { resolution },
        )
        .await?;
        tx.commit().await?;
        Ok(outcome)
    }

    /// Cancels a follow-up as an authorized physician. Terminal: never reopens on replay or late completion.
    pub async fn cancel_follow_up_for_physician(
        &self,
        access: &WorkspaceAccess<'_>,
        command_key: &FollowUpCommandKey,
        id: &FollowUpId,
        reason: FollowUpChangeNote,
        authority: &mut dyn Authority,
    ) -> Result<FollowUpMutation, FollowUpError> {
        let tx = self.db.begin().await?;
        let scope = match verify_physician_access(&tx, access, authority, true).await {
            Ok(scope) => scope,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        let actor = FollowUpActor::Physician {
            physician_id: scope.physician,
        };
        let outcome = transition_follow_up_in_tx(
            &tx,
            &scope.practice,
            &scope.workspace,
            command_key,
            id,
            &actor,
            TransitionSpec::Cancel { reason },
        )
        .await?;
        tx.commit().await?;
        Ok(outcome)
    }

    /// Resolves a non-restartable (`CannotRestart`) follow-up through one of the
    /// physician's three explicit choices (`Complete`, `NewDate`, or `Cancel`).
    pub async fn resolve_cannot_restart_for_physician(
        &self,
        access: &WorkspaceAccess<'_>,
        command_key: &FollowUpCommandKey,
        id: &FollowUpId,
        resolution: CannotRestartResolution,
        authority: &mut dyn Authority,
    ) -> Result<FollowUpMutation, FollowUpError> {
        let tx = self.db.begin().await?;
        let scope = match verify_physician_access(&tx, access, authority, true).await {
            Ok(scope) => scope,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        let actor = FollowUpActor::Physician {
            physician_id: scope.physician,
        };
        let outcome = transition_follow_up_in_tx(
            &tx,
            &scope.practice,
            &scope.workspace,
            command_key,
            id,
            &actor,
            TransitionSpec::ResolveCannotRestart(resolution),
        )
        .await?;
        tx.commit().await?;
        Ok(outcome)
    }

    /// Applies a returned background review outcome (`Resolved`, `Revise`, or `CannotRestart`)
    /// to the same follow-up through its shared operations. Late completions targeting a
    /// `Completed`, `Cancelled`, or `WithdrawnWrongPatient` follow-up fail with [`FollowUpError::Terminal`]
    /// and never reopen it.
    pub async fn apply_follow_up_review_outcome(
        &self,
        practice: &str,
        workspace: &str,
        command_key: &FollowUpCommandKey,
        id: &FollowUpId,
        source_work: SourceWork,
        outcome: FollowUpReviewOutcome,
    ) -> Result<FollowUpMutation, FollowUpError> {
        let actor = FollowUpActor::CodaWork { source_work };
        let tx = self.db.begin().await?;
        let result = match outcome {
            FollowUpReviewOutcome::Resolved { resolution } => {
                transition_follow_up_in_tx(
                    &tx,
                    practice,
                    workspace,
                    command_key,
                    id,
                    &actor,
                    TransitionSpec::Complete { resolution },
                )
                .await?
            }
            FollowUpReviewOutcome::Revise {
                summary,
                due_at,
                records,
                note,
            } => {
                change_follow_up_in_tx(
                    &tx,
                    command_key,
                    &ChangeFollowUp {
                        practice: practice.to_owned(),
                        workspace: workspace.to_owned(),
                        id: id.clone(),
                        summary,
                        due_at,
                        records,
                        note,
                        actor,
                    },
                )
                .await?
            }
            FollowUpReviewOutcome::CannotRestart {
                failed_work_id,
                reason,
            } => {
                transition_follow_up_in_tx(
                    &tx,
                    practice,
                    workspace,
                    command_key,
                    id,
                    &actor,
                    TransitionSpec::MarkCannotRestart {
                        failed_work_id,
                        reason,
                    },
                )
                .await?
            }
        };
        tx.commit().await?;
        Ok(result)
    }
}
