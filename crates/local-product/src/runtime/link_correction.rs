//! Wrong-patient conversation link correction, `Undo` lifecycle, durable
//! settlement progress, and cross-surface withholding enforcement.
//!
//! Moving a linked patient conversation to another patient or to `"not a patient"`
//! immediately withholds the conversation's messages, chart source candidates,
//! saved outputs, exact citations, and reply drafts on every patient workspace it
//! left while keeping an `Undo` window pointing back to the patient before the
//! first move. Permanent removal (`entered_in_error` audit rows, wrong-patient
//! withdrawal of memory/understanding/concerns/follow-ups/sources, reply move
//! fencing, and once-per-affected-patient review requests) waits until `Undo`
//! expires and records per-step progress so a partial failure leaves the
//! correction unfinished and content unavailable with a clear reason until
//! recovery finishes it without restoring wrong-patient content or duplicating
//! review requests.

use std::collections::{BTreeMap, BTreeSet};

use harness::engine::EventKind;
use physician_auth::{AuthStore, PhysicianIdentity};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set, TransactionTrait, sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use thiserror::Error;

use super::concerns::{CommandKey, Concern};
use super::dispatch::ReviewReceiver;
use super::entities::{
    chat_attempt, concern, execution_event, memory_note, patient_understanding_revision,
    practice_job_attempt, reply_draft, revision_attempt, source,
};
use super::follow_ups::{
    FollowUpActor, FollowUpError, WithdrawWrongPatientRequest, WithdrawalFenceViolation,
    follow_up_entity,
};
use super::history::{HistoryError, SavedConversationReadback, WorkspacePastWorkItem};
use super::jobs::PracticeJobError;
use super::receipt::{self, Receipt};
use super::records::Owner;
use super::reply::proposals::{
    ConversationLink, ReplyProposalError,
    tables::{reply_conversation, reply_draft_binding, reply_proposal},
};
use super::spruce_matching::{
    ConfirmSpruceLinkOutcome, ConversationLinkState, LinkOrigin, MessageStanding, MoveDestination,
    PendingMoveUndo, PendingSpruceReviewTrigger, SavedSpruceConversation, SenderConfirmationPolicy,
    SpruceMatchingError, SpruceReviewTriggerOrigin, decode_message_row, dispatch_trigger_list,
    entities as spruce_entities, linked_messages_for_conversation_in_tx, lock_conversation_in_tx,
    queue_review_trigger_in_tx, require_active_physician, require_patient_access,
    save_conversation_in_tx, save_message_in_tx, still_open_viewers,
    undispatched_triggers_for_conversation_in_tx,
};
use super::{Authority, Conversation, MessageRole, Scope, Store, conversion_error, error};
use crate::attachments::READ_DOCUMENT_PAGE_TOOL;
use crate::physician_sources::{READ_TOOL, SourceRevision};

/// Default `Undo` window duration in seconds when correcting a wrong patient link.
pub const DEFAULT_LINK_CORRECTION_UNDO_SECONDS: i64 = 10;

pub(crate) mod link_correction_entity {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_link_corrections")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub correction_id: String,
        pub conversation_id: String,
        pub correction_epoch: i64,
        pub move_ordinal: i64,
        pub original_workspace: String,
        pub intermediate_workspaces: Json,
        pub destination: Json,
        pub standing_kind: String,
        pub standing_detail: Json,
        pub source_candidate_ids: Json,
        pub withheld_work_ids: Json,
        pub withheld_reply_ids: Json,
        pub withheld_draft_keys: Json,
        pub created_at: i64,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub(crate) mod link_correction_workspace_entity {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_link_correction_workspaces")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub correction_id: String,
        pub conversation_id: String,
        pub active_withholding: bool,
        pub withheld_standing: Json,
        pub message_ids: Json,
        pub candidate_ids: Json,
        pub work_ids: Json,
        pub reply_ids: Json,
        pub draft_keys: Json,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Target of a wrong-patient conversation link correction.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CorrectionDestination {
    /// Move the conversation to another authorized patient workspace.
    Patient {
        /// Target patient workspace identifier.
        workspace: String,
    },
    /// Move the conversation out of patient records (`"not a patient"`).
    NotAPatient,
}

impl CorrectionDestination {
    /// Converts to the Spruce matching destination representation.
    #[must_use]
    pub fn to_move_destination(&self) -> MoveDestination {
        match self {
            Self::Patient { workspace } => MoveDestination::AnotherPatient {
                workspace: workspace.clone(),
            },
            Self::NotAPatient => MoveDestination::NotAPatient,
        }
    }

    /// Converts from the Spruce matching destination representation.
    #[must_use]
    pub fn from_move_destination(dest: &MoveDestination) -> Self {
        match dest {
            MoveDestination::AnotherPatient { workspace } => Self::Patient {
                workspace: workspace.clone(),
            },
            MoveDestination::NotAPatient => Self::NotAPatient,
        }
    }
}

/// Physician command linking an unlinked waiting conversation to an authorized patient.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkConversationCommand {
    /// Idempotency key scoped to `(practice, physician)`.
    pub command_key: String,
    /// Practice identifier.
    pub practice: String,
    /// Conversation identifier.
    pub conversation_id: String,
    /// Target patient workspace identifier.
    pub workspace: String,
    /// Sender confirmation policy (`ConversationOnly` or `ConfirmSenderForPatient`).
    pub sender_policy: SenderConfirmationPolicy,
    /// Monotonic command timestamp (Unix seconds).
    pub now_ts: i64,
}

/// Physician command moving a linked conversation to the right patient or `"not a patient"`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorrectPatientLinkCommand {
    /// Idempotency key scoped to `(practice, physician)`.
    pub command_key: String,
    /// Practice identifier.
    pub practice: String,
    /// Conversation identifier.
    pub conversation_id: String,
    /// Destination (`Patient { workspace }` or `NotAPatient`).
    pub destination: CorrectionDestination,
    /// `Undo` window duration in seconds (at least 1 second).
    pub undo_window_seconds: i64,
    /// Monotonic command timestamp (Unix seconds).
    pub now_ts: i64,
}

/// Physician command undoing an active wrong-patient link correction.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoLinkCorrectionCommand {
    /// Idempotency key scoped to `(practice, physician)`.
    pub command_key: String,
    /// Practice identifier.
    pub practice: String,
    /// Conversation identifier.
    pub conversation_id: String,
    /// Monotonic command timestamp (Unix seconds).
    pub now_ts: i64,
}

/// Ordered settlement phase executed after a link correction's `Undo` window expires.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkCorrectionStep {
    /// Record `entered_in_error` audit rows for moved messages without deleting them.
    MarkMessagesEnteredInError,
    /// Withdraw affected durable memory, understanding, concerns, follow-ups, and chart sources.
    WithdrawOriginEffects,
    /// Apply reply withholding or closure on the conversation's reply state.
    ApplyReplyMove,
    /// Queue and dispatch one patient review request per affected patient.
    RequestAffectedReviews,
}

impl LinkCorrectionStep {
    /// Human-readable step name for unavailable notices and diagnostics.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::MarkMessagesEnteredInError => "marking moved messages entered in error",
            Self::WithdrawOriginEffects => "withdrawing wrong-patient chart effects",
            Self::ApplyReplyMove => "updating patient reply standing",
            Self::RequestAffectedReviews => "requesting affected patient reviews",
        }
    }
}

/// Persisted per-step receipts for a link correction's post-`Undo` settlement.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementProgress {
    /// Message IDs marked `entered_in_error` on left workspaces.
    pub entered_in_error_message_ids: BTreeSet<String>,
    /// Completed wrong-patient withdrawal IDs keyed by left workspace.
    pub withdrawal_ids_by_workspace: BTreeMap<String, String>,
    /// Whether the reply proposal/draft move transition has committed.
    pub reply_move_applied: bool,
    /// Queued review command keys keyed by affected workspace.
    pub review_command_keys_by_workspace: BTreeMap<String, String>,
}

/// Lifecycle standing of a saved wrong-patient conversation link correction.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LinkCorrectionStanding {
    /// Waiting inside the `Undo` window before permanent cleanup and review requests run.
    UndoWindow {
        /// Physician who started the correction and holds `Undo` authority.
        by_physician: String,
        /// Timestamp of the latest move inside this correction epoch.
        moved_at: i64,
        /// Exclusive timestamp (`now_ts < expires_at`) until which `Undo` remains open.
        expires_at: i64,
        /// Messages originally linked to `original_workspace` when the first move occurred.
        moved_message_ids: BTreeSet<String>,
    },
    /// Returned to `original_workspace` via `Undo` before expiry.
    Undone {
        /// Physician who pressed `Undo`.
        undone_by: String,
        /// Timestamp when `Undo` committed.
        undone_at: i64,
        /// Message IDs restored to `original_workspace` (including mid-window arrivals).
        restored_message_ids: Vec<String>,
    },
    /// Cancelled because the physician moved the conversation back to `original_workspace` during `Undo`.
    CancelledReturnedToOrigin {
        /// Physician who moved the conversation back.
        cancelled_by: String,
        /// Timestamp when the return move committed.
        cancelled_at: i64,
    },
    /// `Undo` expired, and a removal or review-request step failed; content stays withheld with a reason.
    SettlingFailed {
        /// Timestamp of the failed settlement attempt.
        failed_at: i64,
        /// Step where settlement stopped.
        failed_step: LinkCorrectionStep,
        /// Cause reported for operators and unavailable read notices.
        reason: String,
        /// Completed step receipts preserved for idempotent recovery.
        progress: SettlementProgress,
    },
    /// `Undo` expired and all removal effects and affected-patient review requests finished.
    Settled {
        /// Timestamp when settlement completed.
        settled_at: i64,
        /// Completed step receipts.
        progress: SettlementProgress,
    },
}

impl LinkCorrectionStanding {
    pub(crate) const fn kind_str(&self) -> &'static str {
        match self {
            Self::UndoWindow { .. } => "undo_window",
            Self::Undone { .. } => "undone",
            Self::CancelledReturnedToOrigin { .. } => "cancelled_returned_to_origin",
            Self::SettlingFailed { .. } => "settling_failed",
            Self::Settled { .. } => "settled",
        }
    }
}

/// Durable record of one wrong-patient conversation link correction epoch.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkCorrectionRecord {
    /// Practice identifier.
    pub practice: String,
    /// Stable correction identifier (`corr:{conversation_id}:{correction_epoch}`).
    pub correction_id: String,
    /// Conversation identifier.
    pub conversation_id: String,
    /// Monotonic 1-based correction epoch on this conversation.
    pub correction_epoch: u64,
    /// Latest conversation `move_count` ordinal carried by this correction.
    pub move_ordinal: u64,
    /// Patient workspace linked before the first move in this correction (`Undo` target).
    pub original_workspace: String,
    /// Any intermediate patient workspaces visited by further moves inside the `Undo` window.
    pub intermediate_workspaces: Vec<String>,
    /// Current target destination of the correction.
    pub destination: CorrectionDestination,
    /// Current lifecycle standing and any settlement progress.
    pub standing: LinkCorrectionStanding,
    /// Union of withheld chart source candidate IDs across left workspaces.
    pub source_candidate_ids: BTreeSet<String>,
    /// Union of withheld work/chat/job IDs across left workspaces.
    pub withheld_work_ids: BTreeSet<String>,
    /// Union of withheld reply IDs across left workspaces.
    pub withheld_reply_ids: BTreeSet<u64>,
    /// Union of withheld reply draft scope keys across left workspaces.
    pub withheld_draft_keys: BTreeSet<String>,
    /// Creation timestamp (Unix seconds).
    pub created_at: i64,
    /// Last update timestamp (Unix seconds).
    pub updated_at: i64,
}

/// Outcome of a physician wrong-patient link correction or `Undo` command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkCorrectionOutcome {
    /// Conversation moved to a patient or `"not a patient"`, saving or updating the correction.
    Moved {
        /// Saved correction state.
        record: Box<LinkCorrectionRecord>,
        /// Saved conversation state.
        conversation: SavedSpruceConversation,
    },
    /// Conversation moved back to `original_workspace` during `Undo`, cancelling the correction.
    CancelledReturnedToOrigin {
        /// Cancelled correction state.
        record: Box<LinkCorrectionRecord>,
        /// Saved conversation state.
        conversation: SavedSpruceConversation,
    },
    /// Active correction undone back to `original_workspace`, including mid-window arrivals.
    Undone {
        /// Undone correction state.
        record: Box<LinkCorrectionRecord>,
        /// Saved conversation state.
        conversation: SavedSpruceConversation,
        /// All message IDs now linked on `original_workspace`.
        restored_message_ids: Vec<String>,
    },
    /// Repeated command returned the already-saved correction without creating another.
    Duplicate {
        /// Previously saved correction state.
        record: Box<LinkCorrectionRecord>,
        /// Saved conversation state.
        conversation: SavedSpruceConversation,
    },
}

/// Why a moved conversation's content is currently withheld on a patient workspace.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum WithheldStanding {
    /// Conversation was moved away and its `Undo` window has not yet settled.
    UndoPending {
        /// Exclusive Unix timestamp when `Undo` expires.
        expires_at: i64,
    },
    /// `Undo` expired and a removal step failed; content stays unavailable until recovery finishes.
    CleanupFailed {
        /// Step that failed.
        failed_step: LinkCorrectionStep,
        /// Diagnostic reason recorded at failure.
        failure_reason: String,
    },
    /// `Undo` expired and wrong-patient content was permanently marked entered in error.
    SettledEnteredInError,
}

impl WithheldStanding {
    /// Physician-readable reason explaining why withheld content is unavailable on this patient.
    #[must_use]
    pub fn unavailable_reason(&self, conversation_id: &str) -> String {
        match self {
            Self::UndoPending { .. } => format!(
                "Unavailable: patient conversation {conversation_id} was moved away from this patient while Undo is open."
            ),
            Self::CleanupFailed {
                failed_step,
                failure_reason,
            } => format!(
                "Unavailable: wrong-patient removal for conversation {conversation_id} is unfinished while {} ({failure_reason}).",
                failed_step.label()
            ),
            Self::SettledEnteredInError => format!(
                "Unavailable: patient conversation {conversation_id} was moved away from this patient and marked entered in error."
            ),
        }
    }
}

/// Active withholding entry for one conversation on one left patient workspace.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithheldConversationEntry {
    /// Owning correction identifier.
    pub correction_id: String,
    /// Conversation identifier.
    pub conversation_id: String,
    /// Why this conversation's artifacts are withheld on `workspace`.
    pub standing: WithheldStanding,
    /// Withheld Spruce message IDs on this workspace.
    pub message_ids: BTreeSet<String>,
    /// Withheld chart source candidate IDs on this workspace.
    pub candidate_ids: BTreeSet<String>,
    /// Withheld work, chat, and job IDs on this workspace.
    pub work_ids: BTreeSet<String>,
    /// Withheld reply IDs on this workspace.
    pub reply_ids: BTreeSet<u64>,
    /// Withheld reply draft scope keys on this workspace.
    pub draft_keys: BTreeSet<String>,
}

/// Aggregated active withholding state for one patient workspace across all corrections.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithheldWorkspaceState {
    /// Patient workspace identifier.
    pub workspace: String,
    /// Individual withheld conversation entries (most recent last).
    pub entries: Vec<WithheldConversationEntry>,
    /// Union of withheld message IDs.
    pub message_ids: BTreeSet<String>,
    /// Union of withheld chart source candidate IDs.
    pub candidate_ids: BTreeSet<String>,
    /// Union of withheld work/chat/job IDs.
    pub work_ids: BTreeSet<String>,
    /// Union of withheld reply IDs.
    pub reply_ids: BTreeSet<u64>,
    /// Union of withheld reply draft scope keys.
    pub draft_keys: BTreeSet<String>,
}

impl WithheldWorkspaceState {
    /// Whether nothing is currently withheld on this workspace.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Unavailable reason if `conversation_id` is withheld on this workspace.
    #[must_use]
    pub fn reason_for_conversation(&self, conversation_id: &str) -> Option<String> {
        self.entries
            .iter()
            .rev()
            .find(|e| e.conversation_id == conversation_id)
            .map(|e| e.standing.unavailable_reason(&e.conversation_id))
    }

    /// Unavailable reason if `candidate_id` is withheld on this workspace.
    #[must_use]
    pub fn reason_for_candidate(&self, candidate_id: &str) -> Option<String> {
        self.entries
            .iter()
            .rev()
            .find(|e| e.candidate_ids.contains(candidate_id))
            .map(|e| e.standing.unavailable_reason(&e.conversation_id))
    }

    /// Unavailable reason if `work_id` (or `chat_id` / `job_id`) is withheld on this workspace.
    #[must_use]
    pub fn reason_for_work(&self, work_id: &str) -> Option<String> {
        self.entries
            .iter()
            .rev()
            .find(|e| e.work_ids.contains(work_id))
            .map(|e| e.standing.unavailable_reason(&e.conversation_id))
    }

    /// Unavailable reason if `draft_key` (or its embedded conversation) is withheld on this workspace.
    #[must_use]
    pub fn reason_for_draft_key(&self, draft_key: &str) -> Option<String> {
        if let Some(entry) = self
            .entries
            .iter()
            .rev()
            .find(|e| e.draft_keys.contains(draft_key))
        {
            return Some(entry.standing.unavailable_reason(&entry.conversation_id));
        }
        for entry in self.entries.iter().rev() {
            let prefix = format!("reply-proposal:{}:", entry.conversation_id);
            let direct = format!("reply:{}", entry.conversation_id);
            if draft_key.starts_with(&prefix) || draft_key == direct {
                return Some(entry.standing.unavailable_reason(&entry.conversation_id));
            }
        }
        None
    }

    /// Unavailable reason if either `candidate_id` or `work_id` is withheld on this workspace.
    #[must_use]
    pub fn reason_for_candidate_or_work(
        &self,
        candidate_id: Option<&str>,
        work_id: Option<&str>,
    ) -> Option<String> {
        if let Some(cid) = candidate_id
            && let Some(reason) = self.reason_for_candidate(cid)
        {
            return Some(reason);
        }
        if let Some(wid) = work_id
            && let Some(reason) = self.reason_for_work(wid)
        {
            return Some(reason);
        }
        None
    }
}

/// Fault-injection seam for deterministic testing of unfinished wrong-patient removal.
pub trait LinkCorrectionRemovalProbe: Send + Sync {
    /// Called immediately before `step` executes during settlement.
    fn before_step(
        &self,
        correction_id: &str,
        conversation_id: &str,
        step: LinkCorrectionStep,
    ) -> Result<(), String>;
}

/// Default production probe that never injects failures.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoopLinkCorrectionRemovalProbe;

impl LinkCorrectionRemovalProbe for NoopLinkCorrectionRemovalProbe {
    fn before_step(
        &self,
        _correction_id: &str,
        _conversation_id: &str,
        _step: LinkCorrectionStep,
    ) -> Result<(), String> {
        Ok(())
    }
}

/// Per-correction failure entry reported when settling expired corrections in batch.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkCorrectionBatchFailure {
    /// Failed correction identifier.
    pub correction_id: String,
    /// Conversation identifier.
    pub conversation_id: String,
    /// Step where settlement failed.
    pub failed_step: LinkCorrectionStep,
    /// Failure description.
    pub reason: String,
}

/// Batch settlement report isolating per-conversation failures.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkCorrectionBatchReport {
    /// Corrections that finished all removal steps and review requests.
    pub settled: Vec<LinkCorrectionRecord>,
    /// Corrections that failed a step and remain unfinished with content withheld.
    pub failed: Vec<LinkCorrectionBatchFailure>,
}

/// Errors produced by wrong-patient conversation link correction and authorized reads.
#[derive(Debug, Error)]
pub enum LinkCorrectionError {
    /// A required identifier or parameter failed validation.
    #[error("invalid `{field}`: {reason}")]
    InvalidField {
        /// Field name.
        field: &'static str,
        /// Validation reason.
        reason: String,
    },
    /// Conversation was not found in the practice.
    #[error("conversation `{conversation_id}` not found in practice `{practice}`")]
    ConversationNotFound {
        /// Practice identifier.
        practice: String,
        /// Conversation identifier.
        conversation_id: String,
    },
    /// Correction record was not found in the practice.
    #[error("link correction `{correction_id}` not found in practice `{practice}`")]
    CorrectionNotFound {
        /// Practice identifier.
        practice: String,
        /// Correction identifier.
        correction_id: String,
    },
    /// Physician lacks active practice standing or patient access (`Q2: practice roles`).
    #[error("physician `{physician_id}` does not have access to `{subject}`: {reason}")]
    AccessDenied {
        /// Physician identifier.
        physician_id: String,
        /// Denied practice or workspace.
        subject: String,
        /// Denial explanation.
        reason: &'static str,
    },
    /// Command key was reused with different command arguments.
    #[error("idempotency collision on command key `{command_key}`")]
    IdempotencyCollision {
        /// Colliding command key.
        command_key: String,
    },
    /// An unfinished failed removal on `workspace` blocks restoring or moving back to it.
    #[error(
        "unfinished wrong-patient removal `{correction_id}` on workspace `{workspace}` for conversation `{conversation_id}` blocks restoring content ({reason})"
    )]
    UnfinishedCorrectionBlocksReturn {
        /// Conversation identifier.
        conversation_id: String,
        /// Workspace with unfinished cleanup.
        workspace: String,
        /// Unfinished correction identifier.
        correction_id: String,
        /// Recorded failure reason.
        reason: String,
    },
    /// Requested command transition is invalid for the conversation's current link state.
    #[error("invalid link correction transition on conversation `{conversation_id}`: {reason}")]
    InvalidTransition {
        /// Conversation identifier.
        conversation_id: String,
        /// Explanation.
        reason: String,
    },
    /// A post-`Undo` removal or review-request step failed, leaving the correction unfinished.
    #[error(
        "link correction `{correction_id}` for conversation `{conversation_id}` failed while {}: {reason}",
        step.label()
    )]
    RemovalStepFailed {
        /// Correction identifier.
        correction_id: String,
        /// Conversation identifier.
        conversation_id: String,
        /// Failed step.
        step: LinkCorrectionStep,
        /// Cause message.
        reason: String,
        /// Underlying store error when caused by a runtime subsystem failure rather than a probe fault.
        #[source]
        source: Option<Box<LinkCorrectionError>>,
    },
    /// Underlying Spruce matching failure.
    #[error("spruce matching failure: {0}")]
    SpruceMatching(#[from] SpruceMatchingError),
    /// Underlying wrong-patient withdrawal / follow-up store failure.
    #[error("follow-up withdrawal failure: {0}")]
    FollowUp(#[from] FollowUpError),
    /// Underlying concern store failure.
    #[error("concern failure: {0}")]
    Concern(#[from] super::concerns::ConcernError),
    /// Underlying reply proposal store failure.
    #[error("reply proposal failure: {0}")]
    ReplyProposal(#[from] ReplyProposalError),
    /// Underlying practice job store failure.
    #[error("practice job failure: {0}")]
    PracticeJob(#[from] PracticeJobError),
    /// Underlying history store failure.
    #[error("history read failure: {0}")]
    History(#[from] HistoryError),
    /// Database error.
    #[error("database failure: {0}")]
    Database(#[from] DbErr),
}

#[derive(Clone, Debug, Default)]
pub(crate) struct DiscoveredWorkspaceArtifacts {
    pub message_ids: BTreeSet<String>,
    pub candidate_ids: BTreeSet<String>,
    pub work_ids: BTreeSet<String>,
    pub reply_ids: BTreeSet<u64>,
    pub draft_keys: BTreeSet<String>,
}

fn require_non_empty<'a>(
    value: &'a str,
    field: &'static str,
) -> Result<&'a str, LinkCorrectionError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(LinkCorrectionError::InvalidField {
            field,
            reason: "must not be empty".to_owned(),
        });
    }
    Ok(trimmed)
}

pub(crate) async fn ensure_physician_workspace_access(
    auth: &dyn AuthStore,
    actor: &PhysicianIdentity,
    workspace: &str,
) -> Result<(), LinkCorrectionError> {
    require_patient_access(auth, actor, workspace)
        .await
        .map_err(|err| match err {
            SpruceMatchingError::AccessDenied {
                physician, reason, ..
            } => LinkCorrectionError::AccessDenied {
                physician_id: physician,
                subject: workspace.to_owned(),
                reason,
            },
            other => LinkCorrectionError::SpruceMatching(other),
        })
}

pub(crate) fn decode_link_correction_row(
    row: link_correction_entity::Model,
) -> Result<LinkCorrectionRecord, DbErr> {
    let correction_epoch = u64::try_from(row.correction_epoch)
        .map_err(conversion_error("link_correction.correction_epoch", "u64"))?;
    let move_ordinal = u64::try_from(row.move_ordinal)
        .map_err(conversion_error("link_correction.move_ordinal", "u64"))?;
    let intermediate_workspaces: Vec<String> = serde_json::from_value(row.intermediate_workspaces)
        .map_err(conversion_error(
            "link_correction.intermediate_workspaces",
            "Vec<String>",
        ))?;
    let destination: CorrectionDestination = serde_json::from_value(row.destination).map_err(
        conversion_error("link_correction.destination", "CorrectionDestination"),
    )?;
    let standing: LinkCorrectionStanding = serde_json::from_value(row.standing_detail).map_err(
        conversion_error("link_correction.standing_detail", "LinkCorrectionStanding"),
    )?;
    let source_candidate_ids: BTreeSet<String> = serde_json::from_value(row.source_candidate_ids)
        .map_err(conversion_error(
        "link_correction.source_candidate_ids",
        "BTreeSet<String>",
    ))?;
    let withheld_work_ids: BTreeSet<String> = serde_json::from_value(row.withheld_work_ids)
        .map_err(conversion_error(
            "link_correction.withheld_work_ids",
            "BTreeSet<String>",
        ))?;
    let withheld_reply_ids: BTreeSet<u64> = serde_json::from_value(row.withheld_reply_ids)
        .map_err(conversion_error(
            "link_correction.withheld_reply_ids",
            "BTreeSet<u64>",
        ))?;
    let withheld_draft_keys: BTreeSet<String> = serde_json::from_value(row.withheld_draft_keys)
        .map_err(conversion_error(
            "link_correction.withheld_draft_keys",
            "BTreeSet<String>",
        ))?;

    Ok(LinkCorrectionRecord {
        practice: row.practice,
        correction_id: row.correction_id,
        conversation_id: row.conversation_id,
        correction_epoch,
        move_ordinal,
        original_workspace: row.original_workspace,
        intermediate_workspaces,
        destination,
        standing,
        source_candidate_ids,
        withheld_work_ids,
        withheld_reply_ids,
        withheld_draft_keys,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

pub(crate) async fn save_link_correction_in_tx(
    tx: &impl ConnectionTrait,
    record: &LinkCorrectionRecord,
) -> Result<(), DbErr> {
    let epoch_i64 = i64::try_from(record.correction_epoch)
        .map_err(conversion_error("correction_epoch", "i64"))?;
    let move_i64 =
        i64::try_from(record.move_ordinal).map_err(conversion_error("move_ordinal", "i64"))?;
    let intermediate_json = serde_json::to_value(&record.intermediate_workspaces)
        .map_err(conversion_error("intermediate_workspaces", "json"))?;
    let destination_json = serde_json::to_value(&record.destination)
        .map_err(conversion_error("destination", "json"))?;
    let standing_json =
        serde_json::to_value(&record.standing).map_err(conversion_error("standing", "json"))?;
    let candidates_json = serde_json::to_value(&record.source_candidate_ids)
        .map_err(conversion_error("source_candidate_ids", "json"))?;
    let works_json = serde_json::to_value(&record.withheld_work_ids)
        .map_err(conversion_error("withheld_work_ids", "json"))?;
    let replies_json = serde_json::to_value(&record.withheld_reply_ids)
        .map_err(conversion_error("withheld_reply_ids", "json"))?;
    let drafts_json = serde_json::to_value(&record.withheld_draft_keys)
        .map_err(conversion_error("withheld_draft_keys", "json"))?;

    link_correction_entity::Entity::insert(link_correction_entity::ActiveModel {
        practice: Set(record.practice.clone()),
        correction_id: Set(record.correction_id.clone()),
        conversation_id: Set(record.conversation_id.clone()),
        correction_epoch: Set(epoch_i64),
        move_ordinal: Set(move_i64),
        original_workspace: Set(record.original_workspace.clone()),
        intermediate_workspaces: Set(intermediate_json),
        destination: Set(destination_json),
        standing_kind: Set(record.standing.kind_str().to_owned()),
        standing_detail: Set(standing_json),
        source_candidate_ids: Set(candidates_json),
        withheld_work_ids: Set(works_json),
        withheld_reply_ids: Set(replies_json),
        withheld_draft_keys: Set(drafts_json),
        created_at: Set(record.created_at),
        updated_at: Set(record.updated_at),
    })
    .on_conflict(
        OnConflict::columns([
            link_correction_entity::Column::Practice,
            link_correction_entity::Column::CorrectionId,
        ])
        .update_columns([
            link_correction_entity::Column::MoveOrdinal,
            link_correction_entity::Column::IntermediateWorkspaces,
            link_correction_entity::Column::Destination,
            link_correction_entity::Column::StandingKind,
            link_correction_entity::Column::StandingDetail,
            link_correction_entity::Column::SourceCandidateIds,
            link_correction_entity::Column::WithheldWorkIds,
            link_correction_entity::Column::WithheldReplyIds,
            link_correction_entity::Column::WithheldDraftKeys,
            link_correction_entity::Column::UpdatedAt,
        ])
        .to_owned(),
    )
    .exec_without_returning(tx)
    .await
    .map(|_| ())
}

pub(crate) struct WorkspaceWithholdingUpsert<'a> {
    pub workspace: &'a str,
    pub practice: &'a str,
    pub correction_id: &'a str,
    pub conversation_id: &'a str,
    pub active_withholding: bool,
    pub withheld_standing: &'a WithheldStanding,
    pub artifacts: &'a DiscoveredWorkspaceArtifacts,
    pub updated_at: i64,
}

pub(crate) async fn upsert_workspace_withholding_in_tx(
    tx: &impl ConnectionTrait,
    upsert: WorkspaceWithholdingUpsert<'_>,
) -> Result<(), DbErr> {
    let standing_json = serde_json::to_value(upsert.withheld_standing)
        .map_err(conversion_error("WithheldStanding", "json"))?;
    let msgs_json = serde_json::to_value(&upsert.artifacts.message_ids)
        .map_err(conversion_error("message_ids", "json"))?;
    let cands_json = serde_json::to_value(&upsert.artifacts.candidate_ids)
        .map_err(conversion_error("candidate_ids", "json"))?;
    let works_json = serde_json::to_value(&upsert.artifacts.work_ids)
        .map_err(conversion_error("work_ids", "json"))?;
    let replies_json = serde_json::to_value(&upsert.artifacts.reply_ids)
        .map_err(conversion_error("reply_ids", "json"))?;
    let drafts_json = serde_json::to_value(&upsert.artifacts.draft_keys)
        .map_err(conversion_error("draft_keys", "json"))?;

    link_correction_workspace_entity::Entity::insert(
        link_correction_workspace_entity::ActiveModel {
            workspace: Set(upsert.workspace.to_owned()),
            practice: Set(upsert.practice.to_owned()),
            correction_id: Set(upsert.correction_id.to_owned()),
            conversation_id: Set(upsert.conversation_id.to_owned()),
            active_withholding: Set(upsert.active_withholding),
            withheld_standing: Set(standing_json),
            message_ids: Set(msgs_json),
            candidate_ids: Set(cands_json),
            work_ids: Set(works_json),
            reply_ids: Set(replies_json),
            draft_keys: Set(drafts_json),
            updated_at: Set(upsert.updated_at),
        },
    )
    .on_conflict(
        OnConflict::columns([
            link_correction_workspace_entity::Column::Workspace,
            link_correction_workspace_entity::Column::Practice,
            link_correction_workspace_entity::Column::CorrectionId,
        ])
        .update_columns([
            link_correction_workspace_entity::Column::ActiveWithholding,
            link_correction_workspace_entity::Column::WithheldStanding,
            link_correction_workspace_entity::Column::MessageIds,
            link_correction_workspace_entity::Column::CandidateIds,
            link_correction_workspace_entity::Column::WorkIds,
            link_correction_workspace_entity::Column::ReplyIds,
            link_correction_workspace_entity::Column::DraftKeys,
            link_correction_workspace_entity::Column::UpdatedAt,
        ])
        .to_owned(),
    )
    .exec_without_returning(tx)
    .await
    .map(|_| ())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ToolReadReference {
    pub candidate_id: String,
    pub revision: Option<String>,
}

/// Single-owner extractor returning chart candidate and attachment document references
/// whose clinical text was returned to a model turn via `read_patient_sources` or `read_document_page`.
pub(crate) fn tool_output_read_references(
    tool_name: &str,
    raw_output: &str,
) -> Vec<ToolReadReference> {
    let mut out = Vec::new();
    if tool_name == READ_TOOL {
        let Ok(val) = serde_json::from_str::<Value>(raw_output) else {
            return out;
        };
        let Some(results) = val.get("results").and_then(Value::as_array) else {
            return out;
        };
        for item in results {
            let status = item
                .get("outcome")
                .or_else(|| item.get("status"))
                .and_then(Value::as_str);
            if matches!(status, Some("unavailable" | "over_budget")) {
                continue;
            }
            let Some(reference) = item.get("reference").and_then(Value::as_str) else {
                continue;
            };
            let revision = item
                .get("revision")
                .and_then(Value::as_str)
                .map(str::to_owned);
            out.push(ToolReadReference {
                candidate_id: reference.to_owned(),
                revision,
            });
        }
    } else if tool_name == READ_DOCUMENT_PAGE_TOOL {
        let Ok(val) = serde_json::from_str::<Value>(raw_output) else {
            return out;
        };
        if val.get("outcome").and_then(Value::as_str) != Some("available") {
            return out;
        }
        let revision = val
            .get("revision")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .or_else(|| {
                val.get("version")
                    .and_then(Value::as_u64)
                    .map(|v| format!("v{v}"))
            });
        if let Some(doc_id) = val.get("document_id").and_then(Value::as_str) {
            out.push(ToolReadReference {
                candidate_id: doc_id.to_owned(),
                revision: revision.clone(),
            });
        }
        if let Some(cand_id) = val.get("candidate_id").and_then(Value::as_str)
            && !out.iter().any(|r| r.candidate_id == cand_id)
        {
            out.push(ToolReadReference {
                candidate_id: cand_id.to_owned(),
                revision,
            });
        }
    }
    out
}

/// Single-owner extractor returning candidate/document IDs confirmed `unavailable` in a tool return.
pub(crate) fn tool_output_confirmed_withdrawn_candidates(
    tool_name: &str,
    raw_output: &str,
) -> Vec<String> {
    let mut out = Vec::new();
    if tool_name == READ_TOOL {
        let Ok(val) = serde_json::from_str::<Value>(raw_output) else {
            return out;
        };
        if let Some(results) = val.get("results").and_then(Value::as_array) {
            for item in results {
                let status = item
                    .get("outcome")
                    .or_else(|| item.get("status"))
                    .and_then(Value::as_str);
                if status == Some("unavailable")
                    && let Some(reference) = item.get("reference").and_then(Value::as_str)
                {
                    out.push(reference.to_owned());
                }
            }
        }
    } else if tool_name == READ_DOCUMENT_PAGE_TOOL {
        let Ok(val) = serde_json::from_str::<Value>(raw_output) else {
            return out;
        };
        if val.get("outcome").and_then(Value::as_str) == Some("unavailable")
            && let Some(doc_id) = val
                .get("document_id")
                .or_else(|| val.get("candidate_id"))
                .and_then(Value::as_str)
        {
            out.push(doc_id.to_owned());
        }
    }
    out
}

/// Single-owner rule deciding whether a patient concern (or starting-context concern summary)
/// is withheld on `workspace` because its producing `source_work_id` or cited `records`
/// belong to a withheld wrong-patient conversation.
#[must_use]
pub(crate) fn concern_is_withheld(
    withheld: &WithheldWorkspaceState,
    source_work_id: &str,
    records: &[super::concerns::ExactRecordReference],
) -> bool {
    !withheld.is_empty()
        && (withheld.work_ids.contains(source_work_id)
            || records
                .iter()
                .any(|r| withheld.candidate_ids.contains(&r.candidate_id)))
}

pub(crate) async fn set_correction_workspaces_standing_in_tx(
    tx: &impl ConnectionTrait,
    practice: &str,
    correction_id: &str,
    active_withholding: bool,
    standing: Option<&WithheldStanding>,
    updated_at: i64,
) -> Result<(), DbErr> {
    let rows = link_correction_workspace_entity::Entity::find()
        .filter(link_correction_workspace_entity::Column::Practice.eq(practice))
        .filter(link_correction_workspace_entity::Column::CorrectionId.eq(correction_id))
        .all(tx)
        .await?;
    for row in rows {
        let next_standing = match standing {
            Some(s) => {
                serde_json::to_value(s).map_err(conversion_error("WithheldStanding", "json"))?
            }
            None => row.withheld_standing.clone(),
        };
        link_correction_workspace_entity::ActiveModel {
            workspace: sea_orm::Unchanged(row.workspace),
            practice: sea_orm::Unchanged(row.practice),
            correction_id: sea_orm::Unchanged(row.correction_id),
            conversation_id: sea_orm::Unchanged(row.conversation_id),
            active_withholding: Set(active_withholding),
            withheld_standing: Set(next_standing),
            message_ids: sea_orm::Unchanged(row.message_ids),
            candidate_ids: sea_orm::Unchanged(row.candidate_ids),
            work_ids: sea_orm::Unchanged(row.work_ids),
            reply_ids: sea_orm::Unchanged(row.reply_ids),
            draft_keys: sea_orm::Unchanged(row.draft_keys),
            updated_at: Set(updated_at),
        }
        .update(tx)
        .await?;
    }
    Ok(())
}

/// Discovers every message ID, chart source candidate ID, work/chat/job ID,
/// reply ID, and draft key associated with `conversation_id` on `workspace`.
pub(crate) async fn discover_conversation_artifacts_in_tx(
    tx: &impl ConnectionTrait,
    practice: &str,
    workspace: &str,
    conversation_id: &str,
    seed_message_ids: &BTreeSet<String>,
) -> Result<DiscoveredWorkspaceArtifacts, DbErr> {
    let mut out = DiscoveredWorkspaceArtifacts {
        message_ids: seed_message_ids.clone(),
        ..Default::default()
    };

    if out.message_ids.is_empty() {
        let msg_rows = spruce_entities::spruce_message::Entity::find()
            .filter(spruce_entities::spruce_message::Column::Practice.eq(practice))
            .filter(spruce_entities::spruce_message::Column::ConversationId.eq(conversation_id))
            .all(tx)
            .await?;
        for row in msg_rows {
            let msg = decode_message_row(row)?;
            if matches!(
                msg.standing,
                MessageStanding::Linked { .. } | MessageStanding::NotAPatient { .. }
            ) || msg.ever_linked
            {
                out.message_ids.insert(msg.message_id);
            }
        }
    }

    if let Some(reply_conv_row) =
        reply_conversation::Entity::find_by_id((practice.to_owned(), conversation_id.to_owned()))
            .one(tx)
            .await?
    {
        if let Ok(last_reply_id) = u64::try_from(reply_conv_row.last_reply_id) {
            for id in 1..=last_reply_id {
                out.reply_ids.insert(id);
            }
        }
        if seed_message_ids.is_empty()
            && let Some(patient_msgs) = reply_conv_row
                .state
                .get("patient_messages")
                .and_then(Value::as_array)
        {
            for pm in patient_msgs {
                if let Some(mid) = pm.get("message_id").and_then(Value::as_str) {
                    out.message_ids.insert(mid.to_owned());
                }
            }
        }
        if let Some(concern_ids) = reply_conv_row
            .state
            .get("associated_concern_ids")
            .and_then(Value::as_array)
        {
            for cid_val in concern_ids {
                let Some(cid) = cid_val.as_str() else {
                    continue;
                };
                if let Some(crow) = concern::Entity::find_by_id((
                    practice.to_owned(),
                    workspace.to_owned(),
                    cid.to_owned(),
                ))
                .one(tx)
                .await?
                {
                    let sw: super::concerns::SourceWork = serde_json::from_value(crow.source_work)
                        .map_err(conversion_error("concern.source_work", "SourceWork"))?;
                    out.work_ids.insert(sw.work_id);
                }
            }
        }
    }

    let proposal_rows = reply_proposal::Entity::find()
        .filter(reply_proposal::Column::Practice.eq(practice))
        .filter(reply_proposal::Column::ConversationId.eq(conversation_id))
        .filter(reply_proposal::Column::Workspace.eq(workspace))
        .all(tx)
        .await?;
    for prow in proposal_rows {
        if let Ok(rid) = u64::try_from(prow.reply_id) {
            out.reply_ids.insert(rid);
        }
    }

    let binding_rows = reply_draft_binding::Entity::find()
        .filter(reply_draft_binding::Column::Practice.eq(practice))
        .filter(reply_draft_binding::Column::ConversationId.eq(conversation_id))
        .all(tx)
        .await?;
    for brow in binding_rows {
        if let Ok(rid) = u64::try_from(brow.reply_id) {
            out.reply_ids.insert(rid);
        }
        out.draft_keys.insert(brow.draft_scope_chat);
    }

    let draft_rows = reply_draft::Entity::find()
        .filter(reply_draft::Column::Practice.eq(practice))
        .filter(reply_draft::Column::Workspace.eq(workspace))
        .all(tx)
        .await?;
    let proposal_prefix = format!("reply-proposal:{conversation_id}:");
    let direct_draft = format!("reply:{conversation_id}");
    for drow in draft_rows {
        if drow.reply.starts_with(&proposal_prefix)
            || drow.reply == direct_draft
            || drow.reply == conversation_id
        {
            out.draft_keys.insert(drow.reply);
        }
    }

    out.candidate_ids
        .insert(format!("conversation:{conversation_id}"));
    out.candidate_ids
        .insert(format!("spruce:{conversation_id}"));
    for mid in &out.message_ids {
        out.candidate_ids.insert(mid.clone());
        out.candidate_ids.insert(format!("spruce:{mid}"));
        out.candidate_ids
            .insert(format!("spruce:{conversation_id}:{mid}"));
        out.candidate_ids.insert(format!("spruce-message:{mid}"));
    }

    let source_rows = source::Entity::find()
        .filter(source::Column::Workspace.eq(workspace))
        .all(tx)
        .await?;
    for srow in source_rows {
        let Some(raw_rev) = srow.revision else {
            continue;
        };
        let rev: SourceRevision = serde_json::from_value(raw_rev)
            .map_err(conversion_error("source.revision", "SourceRevision"))?;
        let prov = &rev.provenance;
        let matches_conv = prov
            .get("conversation_id")
            .or_else(|| prov.get("spruce_conversation_id"))
            .and_then(Value::as_str)
            .is_some_and(|cid| cid == conversation_id);
        let matches_msg = prov
            .get("message_id")
            .or_else(|| prov.get("spruce_message_id"))
            .and_then(Value::as_str)
            .is_some_and(|mid| out.message_ids.contains(mid));
        if matches_conv || matches_msg || out.candidate_ids.contains(&srow.candidate) {
            out.candidate_ids.insert(srow.candidate);
            if let Some(doc_id) = prov.get("document_id").and_then(Value::as_str) {
                out.candidate_ids.insert(doc_id.to_owned());
            }
        }
    }

    // Discover chat/job/reply attempts on `workspace` that read any withheld candidate.
    let event_rows = execution_event::Entity::find()
        .filter(execution_event::Column::Practice.eq(practice))
        .filter(execution_event::Column::Workspace.eq(workspace))
        .order_by_asc(execution_event::Column::Sequence)
        .all(tx)
        .await?;
    let mut tool_names_by_attempt_call: BTreeMap<(String, String), String> = BTreeMap::new();
    let mut tainted_chat_attempts = BTreeSet::new();
    let mut tainted_job_attempts = BTreeSet::new();
    let mut tainted_revision_attempts = BTreeSet::new();
    for erow in event_rows {
        let kind: EventKind = serde_json::from_value(erow.record)
            .map_err(conversion_error("execution_event.record", "EventKind"))?;
        match kind {
            EventKind::ToolStarted(call) => {
                tool_names_by_attempt_call.insert((erow.attempt, call.id), call.name);
            }
            EventKind::ToolReturned {
                call_id,
                result: harness::engine::results::ToolResult::Original(raw),
            } => {
                let Some(name) = tool_names_by_attempt_call
                    .get(&(erow.attempt.clone(), call_id))
                    .map(String::as_str)
                else {
                    continue;
                };
                let read_withheld = tool_output_read_references(name, &raw)
                    .iter()
                    .any(|r| out.candidate_ids.contains(&r.candidate_id));
                if read_withheld {
                    match erow.owner {
                        Owner::Chat => {
                            out.work_ids.insert(erow.target);
                            tainted_chat_attempts.insert(erow.attempt);
                        }
                        Owner::PracticeJob => {
                            out.work_ids.insert(erow.target);
                            tainted_job_attempts.insert(erow.attempt);
                        }
                        Owner::Revision => {
                            out.draft_keys.insert(erow.target);
                            tainted_revision_attempts.insert(erow.attempt);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    if !tainted_chat_attempts.is_empty() {
        let attempt_list: Vec<String> = tainted_chat_attempts.into_iter().collect();
        let chat_attempts = chat_attempt::Entity::find()
            .filter(chat_attempt::Column::Practice.eq(practice))
            .filter(chat_attempt::Column::Workspace.eq(workspace))
            .filter(chat_attempt::Column::Id.is_in(attempt_list))
            .all(tx)
            .await?;
        for ca in chat_attempts {
            out.work_ids.insert(ca.work);
            out.work_ids.insert(ca.chat);
        }
    }

    if !tainted_job_attempts.is_empty() {
        let attempt_list: Vec<String> = tainted_job_attempts.into_iter().collect();
        let job_attempts = practice_job_attempt::Entity::find()
            .filter(practice_job_attempt::Column::Practice.eq(practice))
            .filter(practice_job_attempt::Column::Workspace.eq(workspace))
            .filter(practice_job_attempt::Column::Id.is_in(attempt_list))
            .all(tx)
            .await?;
        for ja in job_attempts {
            out.work_ids.insert(ja.work);
            out.work_ids.insert(ja.job_id);
        }
    }

    if !tainted_revision_attempts.is_empty() {
        let attempt_list: Vec<String> = tainted_revision_attempts.into_iter().collect();
        let r_attempts = revision_attempt::Entity::find()
            .filter(revision_attempt::Column::Practice.eq(practice))
            .filter(revision_attempt::Column::Workspace.eq(workspace))
            .filter(revision_attempt::Column::Id.is_in(attempt_list))
            .all(tx)
            .await?;
        for ra in r_attempts {
            out.draft_keys.insert(ra.reply);
        }
    }

    // Also include work IDs recorded directly on concerns, follow-ups, memories, and understanding revisions.
    let concerns_on_ws = concern::Entity::find()
        .filter(concern::Column::Practice.eq(practice))
        .filter(concern::Column::Workspace.eq(workspace))
        .all(tx)
        .await?;
    for crow in concerns_on_ws {
        let records: Vec<super::concerns::ExactRecordReference> =
            serde_json::from_value(crow.records)
                .map_err(conversion_error("concern.records", "ExactRecordReference"))?;
        if records
            .iter()
            .any(|r| out.candidate_ids.contains(&r.candidate_id))
        {
            let sw: super::concerns::SourceWork = serde_json::from_value(crow.source_work)
                .map_err(conversion_error("concern.source_work", "SourceWork"))?;
            out.work_ids.insert(sw.work_id);
        }
    }

    let follow_ups_on_ws = follow_up_entity::Entity::find()
        .filter(follow_up_entity::Column::Practice.eq(practice))
        .filter(follow_up_entity::Column::Workspace.eq(workspace))
        .all(tx)
        .await?;
    for frow in follow_ups_on_ws {
        let recs: Vec<super::concerns::ExactRecordReference> = serde_json::from_value(frow.records)
            .map_err(conversion_error(
                "follow_up.records",
                "ExactRecordReference",
            ))?;
        let cites_records = recs
            .iter()
            .any(|r| out.candidate_ids.contains(&r.candidate_id));
        if cites_records {
            let actor: FollowUpActor = serde_json::from_value(frow.origin)
                .map_err(conversion_error("follow_up.origin", "FollowUpActor"))?;
            if let FollowUpActor::CodaWork { source_work } = actor {
                out.work_ids.insert(source_work.work_id);
            }
        }
    }

    let notes_on_ws = memory_note::Entity::find()
        .filter(memory_note::Column::Practice.eq(practice))
        .filter(memory_note::Column::Workspace.eq(workspace))
        .all(tx)
        .await?;
    for mrow in notes_on_ws {
        let note_work = mrow.note.split_once(':').map(|(work, _)| work);
        if out.message_ids.contains(&mrow.message)
            || out.candidate_ids.contains(&mrow.message)
            || out.work_ids.contains(&mrow.chat)
            || out.work_ids.contains(&mrow.note)
            || note_work.is_some_and(|w| out.work_ids.contains(w))
        {
            out.work_ids.insert(mrow.chat);
            if let Some(work) = note_work {
                out.work_ids.insert(work.to_owned());
            }
        }
    }

    let u_revs = patient_understanding_revision::Entity::find()
        .filter(patient_understanding_revision::Column::Practice.eq(practice))
        .filter(patient_understanding_revision::Column::Workspace.eq(workspace))
        .all(tx)
        .await?;
    for urow in u_revs {
        let summary: super::understanding::PatientSummary = serde_json::from_value(urow.summary)
            .map_err(conversion_error(
                "patient_understanding_revision.summary",
                "PatientSummary",
            ))?;
        let index: crate::initial_understanding::OperatingIndex =
            serde_json::from_value(urow.operating_index).map_err(conversion_error(
                "patient_understanding_revision.operating_index",
                "OperatingIndex",
            ))?;
        let cites = summary
            .citations
            .iter()
            .any(|c| out.candidate_ids.contains(&c.reference.candidate_id))
            || summary.claims.iter().any(|cl| {
                cl.citations
                    .iter()
                    .any(|r| out.candidate_ids.contains(&r.candidate_id))
            })
            || index
                .anchors
                .iter()
                .any(|a| out.candidate_ids.contains(&a.reference.candidate_id));
        if cites {
            let origin: super::understanding::RevisionOrigin = serde_json::from_value(urow.origin)
                .map_err(conversion_error(
                    "patient_understanding_revision.origin",
                    "RevisionOrigin",
                ))?;
            let wid = match origin {
                super::understanding::RevisionOrigin::PracticeRun { work_id, .. }
                | super::understanding::RevisionOrigin::PhysicianConversation { work_id, .. } => {
                    work_id
                }
            };
            out.work_ids.insert(wid);
        }
    }

    Ok(out)
}

/// Single-owner query returning every active withheld conversation and artifact on `workspace`,
/// enriched live with any in-flight `execution_event` tool returns that read withheld candidates.
pub(crate) async fn load_workspace_withheld_state_in_tx(
    db: &impl ConnectionTrait,
    workspace: &str,
) -> Result<WithheldWorkspaceState, DbErr> {
    let rows = link_correction_workspace_entity::Entity::find()
        .filter(link_correction_workspace_entity::Column::Workspace.eq(workspace))
        .filter(link_correction_workspace_entity::Column::ActiveWithholding.eq(true))
        .order_by_asc(link_correction_workspace_entity::Column::UpdatedAt)
        .order_by_asc(link_correction_workspace_entity::Column::CorrectionId)
        .all(db)
        .await?;

    let mut state = WithheldWorkspaceState {
        workspace: workspace.to_owned(),
        ..Default::default()
    };

    for row in rows {
        let standing: WithheldStanding = serde_json::from_value(row.withheld_standing)
            .map_err(conversion_error("withheld_standing", "WithheldStanding"))?;
        let message_ids: BTreeSet<String> = serde_json::from_value(row.message_ids)
            .map_err(conversion_error("message_ids", "BTreeSet<String>"))?;
        let candidate_ids: BTreeSet<String> = serde_json::from_value(row.candidate_ids)
            .map_err(conversion_error("candidate_ids", "BTreeSet<String>"))?;
        let work_ids: BTreeSet<String> = serde_json::from_value(row.work_ids)
            .map_err(conversion_error("work_ids", "BTreeSet<String>"))?;
        let reply_ids: BTreeSet<u64> = serde_json::from_value(row.reply_ids)
            .map_err(conversion_error("reply_ids", "BTreeSet<u64>"))?;
        let draft_keys: BTreeSet<String> = serde_json::from_value(row.draft_keys)
            .map_err(conversion_error("draft_keys", "BTreeSet<String>"))?;

        state.message_ids.extend(message_ids.iter().cloned());
        state.candidate_ids.extend(candidate_ids.iter().cloned());
        state.work_ids.extend(work_ids.iter().cloned());
        state.reply_ids.extend(reply_ids.iter().copied());
        state.draft_keys.extend(draft_keys.iter().cloned());

        state.entries.push(WithheldConversationEntry {
            correction_id: row.correction_id,
            conversation_id: row.conversation_id,
            standing,
            message_ids,
            candidate_ids,
            work_ids,
            reply_ids,
            draft_keys,
        });
    }

    if !state.entries.is_empty() && !state.candidate_ids.is_empty() {
        let event_rows = execution_event::Entity::find()
            .filter(execution_event::Column::Workspace.eq(workspace))
            .order_by_asc(execution_event::Column::Sequence)
            .all(db)
            .await?;
        let mut tool_names: BTreeMap<(String, String), String> = BTreeMap::new();
        let mut chat_attempts_by_entry: BTreeMap<usize, BTreeSet<String>> = BTreeMap::new();
        let mut job_attempts_by_entry: BTreeMap<usize, BTreeSet<String>> = BTreeMap::new();
        let mut rev_attempts_by_entry: BTreeMap<usize, BTreeSet<String>> = BTreeMap::new();
        let mut all_chat_attempts = BTreeSet::new();
        let mut all_job_attempts = BTreeSet::new();
        let mut all_rev_attempts = BTreeSet::new();

        for erow in event_rows {
            let kind: EventKind = serde_json::from_value(erow.record)
                .map_err(conversion_error("execution_event.record", "EventKind"))?;
            match kind {
                EventKind::ToolStarted(call) => {
                    tool_names.insert((erow.attempt, call.id), call.name);
                }
                EventKind::ToolReturned {
                    call_id,
                    result: harness::engine::results::ToolResult::Original(raw),
                } => {
                    let Some(name) = tool_names
                        .get(&(erow.attempt.clone(), call_id))
                        .map(String::as_str)
                    else {
                        continue;
                    };
                    let refs = tool_output_read_references(name, &raw);
                    if refs.is_empty() {
                        continue;
                    }
                    for (idx, entry) in state.entries.iter_mut().enumerate() {
                        if refs
                            .iter()
                            .any(|r| entry.candidate_ids.contains(&r.candidate_id))
                        {
                            match erow.owner {
                                Owner::Chat => {
                                    entry.work_ids.insert(erow.target.clone());
                                    state.work_ids.insert(erow.target.clone());
                                    chat_attempts_by_entry
                                        .entry(idx)
                                        .or_default()
                                        .insert(erow.attempt.clone());
                                    all_chat_attempts.insert(erow.attempt.clone());
                                }
                                Owner::PracticeJob => {
                                    entry.work_ids.insert(erow.target.clone());
                                    state.work_ids.insert(erow.target.clone());
                                    job_attempts_by_entry
                                        .entry(idx)
                                        .or_default()
                                        .insert(erow.attempt.clone());
                                    all_job_attempts.insert(erow.attempt.clone());
                                }
                                Owner::Revision => {
                                    entry.draft_keys.insert(erow.target.clone());
                                    state.draft_keys.insert(erow.target.clone());
                                    rev_attempts_by_entry
                                        .entry(idx)
                                        .or_default()
                                        .insert(erow.attempt.clone());
                                    all_rev_attempts.insert(erow.attempt.clone());
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if !all_chat_attempts.is_empty() {
            let list: Vec<String> = all_chat_attempts.into_iter().collect();
            let rows = chat_attempt::Entity::find()
                .filter(chat_attempt::Column::Workspace.eq(workspace))
                .filter(chat_attempt::Column::Id.is_in(list))
                .all(db)
                .await?;
            for ca in rows {
                for (idx, set) in &chat_attempts_by_entry {
                    if set.contains(&ca.id) {
                        state.entries[*idx].work_ids.insert(ca.work.clone());
                        state.entries[*idx].work_ids.insert(ca.chat.clone());
                        state.work_ids.insert(ca.work.clone());
                        state.work_ids.insert(ca.chat.clone());
                    }
                }
            }
        }

        if !all_job_attempts.is_empty() {
            let list: Vec<String> = all_job_attempts.into_iter().collect();
            let rows = practice_job_attempt::Entity::find()
                .filter(practice_job_attempt::Column::Workspace.eq(workspace))
                .filter(practice_job_attempt::Column::Id.is_in(list))
                .all(db)
                .await?;
            for ja in rows {
                for (idx, set) in &job_attempts_by_entry {
                    if set.contains(&ja.id) {
                        state.entries[*idx].work_ids.insert(ja.work.clone());
                        state.entries[*idx].work_ids.insert(ja.job_id.clone());
                        state.work_ids.insert(ja.work.clone());
                        state.work_ids.insert(ja.job_id.clone());
                    }
                }
            }
        }

        if !all_rev_attempts.is_empty() {
            let list: Vec<String> = all_rev_attempts.into_iter().collect();
            let rows = revision_attempt::Entity::find()
                .filter(revision_attempt::Column::Workspace.eq(workspace))
                .filter(revision_attempt::Column::Id.is_in(list))
                .all(db)
                .await?;
            for ra in rows {
                for (idx, set) in &rev_attempts_by_entry {
                    if set.contains(&ra.id) {
                        state.entries[*idx].draft_keys.insert(ra.reply.clone());
                        state.draft_keys.insert(ra.reply.clone());
                    }
                }
            }
        }
    }

    Ok(state)
}

/// Checks at chat-finish publication (`finish_chat_with_currency_check`) whether the current
/// work read any wrong-patient candidate or attachment document that is now withheld or withdrawn
/// without subsequently confirming its withdrawal via `read_patient_sources` / `read_document_page`.
pub(crate) async fn evaluate_withheld_chat_finish_in_tx(
    db: &impl ConnectionTrait,
    scope: &Scope,
    state: &Conversation,
    workspace_version: u64,
) -> Result<Option<crate::physician_sources::ChatChartArrivalNote>, DbErr> {
    let withheld = load_workspace_withheld_state_in_tx(db, &scope.workspace).await?;
    let boundary = super::follow_ups::load_workspace_withdrawal_boundary(
        db,
        &scope.practice,
        &scope.workspace,
    )
    .await?;
    if withheld.is_empty() && boundary.is_none() {
        return Ok(None);
    }

    let boundary_candidates: BTreeSet<String> = boundary
        .as_ref()
        .map(|b| b.candidate_ids.iter().cloned().collect())
        .unwrap_or_default();

    let mut pending_rereads: BTreeMap<String, String> = BTreeMap::new();
    let mut withdrawn_revs: BTreeMap<String, String> = BTreeMap::new();
    let mut tool_names: BTreeMap<String, String> = BTreeMap::new();

    let ordered = super::records::ordered(
        db,
        scope,
        super::records::Owner::Chat,
        &state.work_attempt_ids(),
    )
    .await?;
    for (event, admitted) in &ordered {
        if !admitted {
            continue;
        }
        match &event.kind {
            EventKind::ToolStarted(call) => {
                tool_names.insert(call.id.clone(), call.name.clone());
            }
            EventKind::ToolReturned {
                call_id,
                result: harness::engine::results::ToolResult::Original(raw),
            } => {
                let Some(name) = tool_names.get(call_id).map(String::as_str) else {
                    continue;
                };
                for r in tool_output_read_references(name, raw) {
                    if withheld.candidate_ids.contains(&r.candidate_id)
                        || boundary_candidates.contains(&r.candidate_id)
                    {
                        let rev = r.revision.unwrap_or_else(|| "v1".to_owned());
                        withdrawn_revs.insert(r.candidate_id.clone(), rev.clone());
                        pending_rereads.insert(r.candidate_id, rev);
                    }
                }
                for confirmed in tool_output_confirmed_withdrawn_candidates(name, raw) {
                    pending_rereads.remove(&confirmed);
                }
            }
            _ => {}
        }
    }

    if pending_rereads.is_empty() {
        return Ok(None);
    }

    let withdrawn = withdrawn_revs
        .into_iter()
        .map(
            |(cid, withdrawn_revision)| crate::physician_sources::WithdrawnChartRecord {
                candidate_id: cid,
                source_id: "spruce".to_owned(),
                record_type: "patient_message".to_owned(),
                dates: Vec::new(),
                shape: crate::physician_sources::WithdrawnShape::RemovedFromObservation {
                    withdrawn_revision,
                },
            },
        )
        .collect();
    let must_read_again = pending_rereads
        .into_iter()
        .map(|(candidate_id, withdrawn_revision)| {
            crate::physician_sources::ReliedRecordRequirement {
                candidate_id,
                action: crate::physician_sources::ReliedRereadAction::ConfirmWithdrawnFromChart {
                    withdrawn_revision,
                },
            }
        })
        .collect();

    Ok(Some(crate::physician_sources::ChatChartArrivalNote {
        outcome: crate::physician_sources::ChatArrivalOutcome::SourcesChanged,
        workspace_version,
        added: Vec::new(),
        corrected: Vec::new(),
        withdrawn,
        must_read_again,
        message: "Patient chart records were added, corrected, or withdrawn while you were answering, so your reply was not published yet. Call read_patient_sources on every record in must_read_again, judge whether any added records affect the question, update or retire any affected patient memory or open concern, and then provide your complete updated answer.".to_owned(),
    }))
}

/// Returns withheld chart source candidate IDs on `workspace`.
pub(crate) async fn withheld_candidate_ids_in_tx(
    db: &impl ConnectionTrait,
    workspace: &str,
) -> Result<BTreeSet<String>, DbErr> {
    Ok(load_workspace_withheld_state_in_tx(db, workspace)
        .await?
        .candidate_ids)
}

/// Returns the physician-facing unavailable reason if `conversation_id` is withheld on `workspace`.
pub(crate) async fn conversation_withheld_reason_in_tx(
    db: &impl ConnectionTrait,
    practice: &str,
    workspace: &str,
    conversation_id: &str,
) -> Result<Option<String>, DbErr> {
    let rows = link_correction_workspace_entity::Entity::find()
        .filter(link_correction_workspace_entity::Column::Practice.eq(practice))
        .filter(link_correction_workspace_entity::Column::Workspace.eq(workspace))
        .filter(link_correction_workspace_entity::Column::ConversationId.eq(conversation_id))
        .filter(link_correction_workspace_entity::Column::ActiveWithholding.eq(true))
        .order_by_desc(link_correction_workspace_entity::Column::UpdatedAt)
        .all(db)
        .await?;
    let Some(row) = rows.into_iter().next() else {
        return Ok(None);
    };
    let standing: WithheldStanding = serde_json::from_value(row.withheld_standing)
        .map_err(conversion_error("withheld_standing", "WithheldStanding"))?;
    Ok(Some(standing.unavailable_reason(&row.conversation_id)))
}

/// Enforces that writes on `(practice, workspace)` cannot cite withheld candidates, messages, or works.
pub(super) async fn check_withheld_fence_in_tx(
    db: &impl ConnectionTrait,
    _practice: &str,
    workspace: &str,
    cited_candidates: &[String],
    message_id: Option<&str>,
    work_id: Option<&str>,
) -> Result<(), super::follow_ups::WithdrawalFenceCheckError> {
    let state = load_workspace_withheld_state_in_tx(db, workspace)
        .await
        .map_err(super::follow_ups::WithdrawalFenceCheckError::Database)?;
    if state.is_empty() {
        return Ok(());
    }
    for cand in cited_candidates {
        if let Some(entry) = state
            .entries
            .iter()
            .rev()
            .find(|e| e.candidate_ids.contains(cand))
        {
            return Err(super::follow_ups::WithdrawalFenceCheckError::Violation(
                WithdrawalFenceViolation::WithdrawnCandidate {
                    candidate_id: cand.clone(),
                    withdrawal_id: entry.correction_id.clone(),
                    withdrawal_epoch: 1,
                },
            ));
        }
    }
    if let Some(mid) = message_id
        && let Some(entry) = state
            .entries
            .iter()
            .rev()
            .find(|e| e.message_ids.contains(mid) || e.candidate_ids.contains(mid))
    {
        return Err(super::follow_ups::WithdrawalFenceCheckError::Violation(
            WithdrawalFenceViolation::WithdrawnMessage {
                message_id: mid.to_owned(),
                withdrawal_id: entry.correction_id.clone(),
                withdrawal_epoch: 1,
            },
        ));
    }
    if let Some(wid) = work_id
        && let Some(entry) = state
            .entries
            .iter()
            .rev()
            .find(|e| e.work_ids.contains(wid))
    {
        return Err(super::follow_ups::WithdrawalFenceCheckError::Violation(
            WithdrawalFenceViolation::FencedWork {
                work_id: wid.to_owned(),
                withdrawal_id: entry.correction_id.clone(),
                withdrawal_epoch: 1,
            },
        ));
    }
    Ok(())
}

/// Redacts assistant turns on display reads (`Store::read` / `Store::conversation`) when a
/// turn's producing work or chat read a withheld wrong-patient candidate or document page.
pub(crate) async fn redact_conversation_if_withheld_in_tx(
    db: &impl ConnectionTrait,
    scope: &Scope,
    state: &mut Conversation,
) -> Result<(), DbErr> {
    let withheld = load_workspace_withheld_state_in_tx(db, &scope.workspace).await?;
    let boundary = super::follow_ups::load_workspace_withdrawal_boundary(
        db,
        &scope.practice,
        &scope.workspace,
    )
    .await?;
    let boundary_works =
        super::follow_ups::load_workspace_fenced_work_ids(db, &scope.practice, &scope.workspace)
            .await?;
    if withheld.is_empty() && boundary.is_none() && boundary_works.is_empty() {
        return Ok(());
    }

    let mut tainted_work_reason: BTreeMap<String, String> = BTreeMap::new();
    for attempt in &state.attempts {
        if let Some(reason) = withheld.reason_for_work(&attempt.work) {
            tainted_work_reason.insert(attempt.work.clone(), reason);
            continue;
        }
        if boundary_works.contains(&attempt.work) {
            tainted_work_reason.insert(
                attempt.work.clone(),
                "Unavailable: patient message content was withdrawn after a wrong-patient link correction."
                    .to_owned(),
            );
            continue;
        }
        let mut tool_names: BTreeMap<String, String> = BTreeMap::new();
        for event in &attempt.events {
            match &event.kind {
                EventKind::ToolStarted(call) => {
                    tool_names.insert(call.id.clone(), call.name.clone());
                }
                EventKind::ToolReturned {
                    call_id,
                    result: harness::engine::results::ToolResult::Original(raw),
                } => {
                    let Some(name) = tool_names.get(call_id).map(String::as_str) else {
                        continue;
                    };
                    for item in tool_output_read_references(name, raw) {
                        let cand = item.candidate_id.as_str();
                        if let Some(reason) = withheld.reason_for_candidate(cand) {
                            tainted_work_reason.insert(attempt.work.clone(), reason);
                            break;
                        }
                        if let Some(b) = &boundary
                            && b.candidate_ids.iter().any(|c| c == cand)
                        {
                            tainted_work_reason.insert(
                                attempt.work.clone(),
                                format!(
                                    "Unavailable: cited patient message {cand} was withdrawn after a wrong-patient link correction."
                                ),
                            );
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    for msg in &state.messages {
        if let MessageRole::User { work, .. } = &msg.role
            && !tainted_work_reason.contains_key(work)
        {
            if let Some(reason) = withheld.reason_for_work(work) {
                tainted_work_reason.insert(work.clone(), reason);
            } else if boundary_works.contains(work) {
                tainted_work_reason.insert(
                    work.clone(),
                    "Unavailable: patient message content was withdrawn after a wrong-patient link correction."
                        .to_owned(),
                );
            }
        }
    }

    let chat_level_reason = withheld.reason_for_work(&scope.chat).or_else(|| {
        boundary_works.contains(&scope.chat).then(|| {
            "Unavailable: patient message content was withdrawn after a wrong-patient link correction."
                .to_owned()
        })
    });
    if tainted_work_reason.is_empty() && chat_level_reason.is_none() {
        return Ok(());
    }

    let mut active_work: Option<String> = None;
    for msg in &mut state.messages {
        match &mut msg.role {
            MessageRole::User { work, .. } => {
                active_work = Some(work.clone());
            }
            MessageRole::Assistant { kind, .. } => {
                let reason = active_work
                    .as_deref()
                    .and_then(|w| tainted_work_reason.get(w))
                    .or(chat_level_reason.as_ref());
                if let Some(reason) = reason {
                    msg.text = reason.clone();
                    if let super::AssistantKind::Finished(finished) = kind {
                        *finished = crate::physician_sources::citations::Finished::redacted();
                    }
                }
            }
        }
    }
    Ok(())
}

pub(crate) async fn latest_correction_for_conversation_in_tx(
    db: &impl ConnectionTrait,
    practice: &str,
    conversation_id: &str,
) -> Result<Option<LinkCorrectionRecord>, DbErr> {
    let row = link_correction_entity::Entity::find()
        .filter(link_correction_entity::Column::Practice.eq(practice))
        .filter(link_correction_entity::Column::ConversationId.eq(conversation_id))
        .order_by_desc(link_correction_entity::Column::CorrectionEpoch)
        .one(db)
        .await?;
    row.map(decode_link_correction_row).transpose()
}

pub(crate) async fn check_unfinished_correction_blocking_workspace_in_tx(
    db: &impl ConnectionTrait,
    practice: &str,
    conversation_id: &str,
    target_workspace: &str,
) -> Result<(), LinkCorrectionError> {
    let rows = link_correction_entity::Entity::find()
        .filter(link_correction_entity::Column::Practice.eq(practice))
        .filter(link_correction_entity::Column::ConversationId.eq(conversation_id))
        .filter(link_correction_entity::Column::StandingKind.eq("settling_failed"))
        .order_by_desc(link_correction_entity::Column::CorrectionEpoch)
        .all(db)
        .await?;
    for row in rows {
        let rec = decode_link_correction_row(row)?;
        let affects_target = rec.original_workspace == target_workspace
            || rec
                .intermediate_workspaces
                .iter()
                .any(|w| w == target_workspace)
            || matches!(&rec.destination, CorrectionDestination::Patient { workspace } if workspace != target_workspace);
        if affects_target
            && let LinkCorrectionStanding::SettlingFailed { reason, .. } = &rec.standing
        {
            return Err(LinkCorrectionError::UnfinishedCorrectionBlocksReturn {
                conversation_id: conversation_id.to_owned(),
                workspace: target_workspace.to_owned(),
                correction_id: rec.correction_id,
                reason: reason.clone(),
            });
        }
    }
    Ok(())
}

pub(crate) struct MoveCorrectionWrite<'a> {
    pub practice: &'a str,
    pub conversation_id: &'a str,
    pub actor_physician_id: &'a str,
    pub departed_workspace: Option<&'a str>,
    pub departed_linked_ids: &'a BTreeSet<String>,
    pub active_prior_undo: Option<&'a PendingMoveUndo>,
    pub new_undo: Option<&'a PendingMoveUndo>,
    pub returned_to_origin: bool,
    pub destination: &'a CorrectionDestination,
    pub move_ordinal: u64,
    pub now_ts: i64,
}

/// Synchronizes `physician_runtime_link_corrections` and `physician_runtime_link_correction_workspaces`
/// when a move occurs inside `tx` (`move_spruce_conversation` or `correct_patient_link`).
pub(crate) async fn record_move_correction_in_tx(
    tx: &impl ConnectionTrait,
    write: MoveCorrectionWrite<'_>,
) -> Result<LinkCorrectionRecord, DbErr> {
    let practice = write.practice;
    let conversation_id = write.conversation_id;
    let actor_physician_id = write.actor_physician_id;
    let departed_workspace = write.departed_workspace;
    let departed_linked_ids = write.departed_linked_ids;
    let active_prior_undo = write.active_prior_undo;
    let new_undo = write.new_undo;
    let returned_to_origin = write.returned_to_origin;
    let destination = write.destination;
    let move_ordinal = write.move_ordinal;
    let now_ts = write.now_ts;

    let latest = latest_correction_for_conversation_in_tx(tx, practice, conversation_id).await?;

    if returned_to_origin
        && let Some(mut active_rec) = latest
            .clone()
            .filter(|r| matches!(r.standing, LinkCorrectionStanding::UndoWindow { .. }))
    {
        set_correction_workspaces_standing_in_tx(
            tx,
            practice,
            &active_rec.correction_id,
            false,
            None,
            now_ts,
        )
        .await?;
        active_rec.move_ordinal = move_ordinal;
        active_rec.destination = destination.clone();
        active_rec.standing = LinkCorrectionStanding::CancelledReturnedToOrigin {
            cancelled_by: actor_physician_id.to_owned(),
            cancelled_at: now_ts,
        };
        active_rec.updated_at = now_ts;
        save_link_correction_in_tx(tx, &active_rec).await?;
        return Ok(active_rec);
    }

    if let (Some(_prior), Some(undo_now), Some(mut active_rec)) = (
        active_prior_undo,
        new_undo,
        latest
            .clone()
            .filter(|r| matches!(r.standing, LinkCorrectionStanding::UndoWindow { .. })),
    ) {
        let next_withheld = WithheldStanding::UndoPending {
            expires_at: undo_now.expires_at,
        };
        set_correction_workspaces_standing_in_tx(
            tx,
            practice,
            &active_rec.correction_id,
            true,
            Some(&next_withheld),
            now_ts,
        )
        .await?;

        if let Some(left_ws) = departed_workspace
            && left_ws != active_rec.original_workspace
        {
            if !active_rec
                .intermediate_workspaces
                .iter()
                .any(|w| w == left_ws)
            {
                active_rec.intermediate_workspaces.push(left_ws.to_owned());
            }
            let hop_artifacts = discover_conversation_artifacts_in_tx(
                tx,
                practice,
                left_ws,
                conversation_id,
                departed_linked_ids,
            )
            .await?;
            active_rec
                .source_candidate_ids
                .extend(hop_artifacts.candidate_ids.iter().cloned());
            active_rec
                .withheld_work_ids
                .extend(hop_artifacts.work_ids.iter().cloned());
            active_rec
                .withheld_reply_ids
                .extend(hop_artifacts.reply_ids.iter().copied());
            active_rec
                .withheld_draft_keys
                .extend(hop_artifacts.draft_keys.iter().cloned());
            upsert_workspace_withholding_in_tx(
                tx,
                WorkspaceWithholdingUpsert {
                    workspace: left_ws,
                    practice,
                    correction_id: &active_rec.correction_id,
                    conversation_id,
                    active_withholding: true,
                    withheld_standing: &next_withheld,
                    artifacts: &hop_artifacts,
                    updated_at: now_ts,
                },
            )
            .await?;
        }

        // If moving onto an intermediate workspace that had previously been withheld in this same
        // Undo window, lift active withholding on that destination workspace while keeping origin withheld.
        if let CorrectionDestination::Patient { workspace: dest_ws } = destination
            && dest_ws != &active_rec.original_workspace
            && let Some(existing_dest_row) = link_correction_workspace_entity::Entity::find_by_id((
                dest_ws.clone(),
                practice.to_owned(),
                active_rec.correction_id.clone(),
            ))
            .one(tx)
            .await?
        {
            link_correction_workspace_entity::ActiveModel {
                workspace: sea_orm::Unchanged(existing_dest_row.workspace),
                practice: sea_orm::Unchanged(existing_dest_row.practice),
                correction_id: sea_orm::Unchanged(existing_dest_row.correction_id),
                conversation_id: sea_orm::Unchanged(existing_dest_row.conversation_id),
                active_withholding: Set(false),
                withheld_standing: sea_orm::Unchanged(existing_dest_row.withheld_standing),
                message_ids: sea_orm::Unchanged(existing_dest_row.message_ids),
                candidate_ids: sea_orm::Unchanged(existing_dest_row.candidate_ids),
                work_ids: sea_orm::Unchanged(existing_dest_row.work_ids),
                reply_ids: sea_orm::Unchanged(existing_dest_row.reply_ids),
                draft_keys: sea_orm::Unchanged(existing_dest_row.draft_keys),
                updated_at: Set(now_ts),
            }
            .update(tx)
            .await?;
        }

        active_rec.move_ordinal = move_ordinal;
        active_rec.destination = destination.clone();
        active_rec.standing = LinkCorrectionStanding::UndoWindow {
            by_physician: undo_now.by_physician.clone(),
            moved_at: undo_now.moved_at,
            expires_at: undo_now.expires_at,
            moved_message_ids: undo_now.moved_message_ids.clone(),
        };
        active_rec.updated_at = now_ts;
        save_link_correction_in_tx(tx, &active_rec).await?;
        return Ok(active_rec);
    }

    let next_epoch = latest.map_or(1, |r| r.correction_epoch + 1);
    let correction_id = format!("corr:{conversation_id}:{next_epoch}");
    let undo_state = new_undo.ok_or_else(|| error("missing undo state for new link correction"))?;
    let origin_ws = undo_state.from_workspace.clone();
    let artifacts = discover_conversation_artifacts_in_tx(
        tx,
        practice,
        &origin_ws,
        conversation_id,
        &undo_state.moved_message_ids,
    )
    .await?;
    let withheld_standing = WithheldStanding::UndoPending {
        expires_at: undo_state.expires_at,
    };
    upsert_workspace_withholding_in_tx(
        tx,
        WorkspaceWithholdingUpsert {
            workspace: &origin_ws,
            practice,
            correction_id: &correction_id,
            conversation_id,
            active_withholding: true,
            withheld_standing: &withheld_standing,
            artifacts: &artifacts,
            updated_at: now_ts,
        },
    )
    .await?;

    let record = LinkCorrectionRecord {
        practice: practice.to_owned(),
        correction_id,
        conversation_id: conversation_id.to_owned(),
        correction_epoch: next_epoch,
        move_ordinal,
        original_workspace: origin_ws,
        intermediate_workspaces: Vec::new(),
        destination: destination.clone(),
        standing: LinkCorrectionStanding::UndoWindow {
            by_physician: undo_state.by_physician.clone(),
            moved_at: undo_state.moved_at,
            expires_at: undo_state.expires_at,
            moved_message_ids: undo_state.moved_message_ids.clone(),
        },
        source_candidate_ids: artifacts.candidate_ids,
        withheld_work_ids: artifacts.work_ids,
        withheld_reply_ids: artifacts.reply_ids,
        withheld_draft_keys: artifacts.draft_keys,
        created_at: now_ts,
        updated_at: now_ts,
    };
    save_link_correction_in_tx(tx, &record).await?;
    Ok(record)
}

/// Synchronizes `physician_runtime_link_corrections` and `physician_runtime_link_correction_workspaces`
/// when `Undo` commits inside `tx`.
pub(crate) async fn record_undo_correction_in_tx(
    tx: &impl ConnectionTrait,
    practice: &str,
    conversation_id: &str,
    actor_physician_id: &str,
    restored_message_ids: Vec<String>,
    now_ts: i64,
) -> Result<Option<LinkCorrectionRecord>, DbErr> {
    let Some(mut active_rec) =
        latest_correction_for_conversation_in_tx(tx, practice, conversation_id).await?
    else {
        return Ok(None);
    };
    if !matches!(
        active_rec.standing,
        LinkCorrectionStanding::UndoWindow { .. }
    ) {
        return Ok(Some(active_rec));
    }
    set_correction_workspaces_standing_in_tx(
        tx,
        practice,
        &active_rec.correction_id,
        false,
        None,
        now_ts,
    )
    .await?;
    active_rec.standing = LinkCorrectionStanding::Undone {
        undone_by: actor_physician_id.to_owned(),
        undone_at: now_ts,
        restored_message_ids,
    };
    active_rec.updated_at = now_ts;
    save_link_correction_in_tx(tx, &active_rec).await?;
    Ok(Some(active_rec))
}

fn left_workspaces_for_record(record: &LinkCorrectionRecord) -> Vec<String> {
    let dest_ws = match &record.destination {
        CorrectionDestination::Patient { workspace } => Some(workspace.as_str()),
        CorrectionDestination::NotAPatient => None,
    };
    let mut list = Vec::new();
    if Some(record.original_workspace.as_str()) != dest_ws {
        list.push(record.original_workspace.clone());
    }
    for ws in &record.intermediate_workspaces {
        if Some(ws.as_str()) != dest_ws && !list.iter().any(|existing| existing == ws) {
            list.push(ws.clone());
        }
    }
    list
}

impl Store {
    /// Links an unlinked waiting Spruce conversation to an authorized patient workspace (`AC1`).
    pub async fn link_conversation_to_patient(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        cmd: LinkConversationCommand,
        review_receiver: Option<&dyn ReviewReceiver>,
    ) -> Result<ConfirmSpruceLinkOutcome, LinkCorrectionError> {
        let practice = require_non_empty(&cmd.practice, "practice")?;
        let conversation_id = require_non_empty(&cmd.conversation_id, "conversation_id")?;
        let workspace = require_non_empty(&cmd.workspace, "workspace")?;
        let command_key = require_non_empty(&cmd.command_key, "command_key")?;
        if actor.practice_id != practice {
            return Err(LinkCorrectionError::AccessDenied {
                physician_id: actor.physician_id.clone(),
                subject: practice.to_owned(),
                reason: "actor practice does not match target practice",
            });
        }
        ensure_physician_workspace_access(auth, actor, workspace).await?;
        let representative_msg_id = {
            let tx = self.db.begin().await?;
            check_unfinished_correction_blocking_workspace_in_tx(
                &tx,
                practice,
                conversation_id,
                workspace,
            )
            .await?;
            let msgs = spruce_entities::spruce_message::Entity::find()
                .filter(spruce_entities::spruce_message::Column::Practice.eq(practice))
                .filter(spruce_entities::spruce_message::Column::ConversationId.eq(conversation_id))
                .order_by_asc(spruce_entities::spruce_message::Column::ChannelPosition)
                .all(&tx)
                .await?;
            tx.commit().await?;
            let mut waiting_id = None;
            let mut any_id = None;
            for row in msgs {
                if any_id.is_none() {
                    any_id = Some(row.message_id.clone());
                }
                if row.standing_kind == "waiting" {
                    waiting_id = Some(row.message_id);
                    break;
                }
            }
            waiting_id
                .or(any_id)
                .ok_or_else(|| LinkCorrectionError::ConversationNotFound {
                    practice: practice.to_owned(),
                    conversation_id: conversation_id.to_owned(),
                })?
        };
        let outcome = self
            .confirm_spruce_message_link(
                auth,
                actor,
                &representative_msg_id,
                workspace,
                cmd.sender_policy,
                command_key,
                review_receiver,
                cmd.now_ts,
            )
            .await
            .map_err(|err| match err {
                SpruceMatchingError::AccessDenied {
                    physician, reason, ..
                } => LinkCorrectionError::AccessDenied {
                    physician_id: physician,
                    subject: workspace.to_owned(),
                    reason,
                },
                SpruceMatchingError::IdempotencyCollision { key } => {
                    LinkCorrectionError::IdempotencyCollision { command_key: key }
                }
                SpruceMatchingError::NotFound {
                    practice,
                    kind: "conversation" | "message",
                    ..
                } => LinkCorrectionError::ConversationNotFound {
                    practice,
                    conversation_id: conversation_id.to_owned(),
                },
                other => LinkCorrectionError::SpruceMatching(other),
            })?;
        Ok(outcome)
    }

    /// Moves a linked conversation to the right patient or `"not a patient"`,
    /// enforcing patient access on both origin and destination, withholding content
    /// immediately on the patient it left, preserving the original `Undo` target
    /// across multi-hop moves, cancelling when moved back to origin, and saving
    /// only one correction across repeated commands (`AC1`, `AC2`, `AC3`, `AC4`).
    pub async fn correct_patient_link(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        cmd: CorrectPatientLinkCommand,
        review_receiver: Option<&dyn ReviewReceiver>,
    ) -> Result<LinkCorrectionOutcome, LinkCorrectionError> {
        self.correct_patient_link_with_probe(
            auth,
            actor,
            cmd,
            review_receiver,
            &NoopLinkCorrectionRemovalProbe,
        )
        .await
    }

    /// [`Self::correct_patient_link`] with an explicit [`LinkCorrectionRemovalProbe`] for
    /// settling any previously expired correction on the same conversation first.
    pub async fn correct_patient_link_with_probe(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        cmd: CorrectPatientLinkCommand,
        review_receiver: Option<&dyn ReviewReceiver>,
        probe: &dyn LinkCorrectionRemovalProbe,
    ) -> Result<LinkCorrectionOutcome, LinkCorrectionError> {
        let practice = require_non_empty(&cmd.practice, "practice")?;
        let conversation_id = require_non_empty(&cmd.conversation_id, "conversation_id")?;
        let command_key = require_non_empty(&cmd.command_key, "command_key")?;
        if actor.practice_id != practice {
            return Err(LinkCorrectionError::AccessDenied {
                physician_id: actor.physician_id.clone(),
                subject: practice.to_owned(),
                reason: "actor practice does not match target practice",
            });
        }
        require_active_physician(auth, actor)
            .await
            .map_err(|err| match err {
                SpruceMatchingError::AccessDenied {
                    physician, reason, ..
                } => LinkCorrectionError::AccessDenied {
                    physician_id: physician,
                    subject: practice.to_owned(),
                    reason,
                },
                other => LinkCorrectionError::SpruceMatching(other),
            })?;
        if let CorrectionDestination::Patient { workspace } = &cmd.destination {
            let ws = require_non_empty(workspace, "destination.workspace")?;
            ensure_physician_workspace_access(auth, actor, ws).await?;
        }

        let command_body = json!({
            "op": "correct_patient_link",
            "practice": practice,
            "conversation_id": conversation_id,
            "destination": &cmd.destination,
            "undo_window_seconds": cmd.undo_window_seconds,
        });

        // Check command idempotency and inspect current conversation / correction state.
        let expired_correction_to_settle = {
            let tx = self.db.begin().await?;
            match receipt::check(
                &tx,
                practice,
                &actor.physician_id,
                command_key,
                &command_body,
            )
            .await?
            {
                Some(Receipt::Same) => {
                    let conv = lock_conversation_in_tx(&tx, practice, conversation_id)
                        .await?
                        .ok_or_else(|| LinkCorrectionError::ConversationNotFound {
                            practice: practice.to_owned(),
                            conversation_id: conversation_id.to_owned(),
                        })?;
                    let rec =
                        latest_correction_for_conversation_in_tx(&tx, practice, conversation_id)
                            .await?
                            .ok_or_else(|| LinkCorrectionError::ConversationNotFound {
                                practice: practice.to_owned(),
                                conversation_id: conversation_id.to_owned(),
                            })?;
                    let triggers = undispatched_triggers_for_conversation_in_tx(
                        &tx,
                        practice,
                        conversation_id,
                    )
                    .await?;
                    tx.commit().await?;
                    dispatch_trigger_list(self, &triggers, review_receiver, cmd.now_ts).await?;
                    return Ok(LinkCorrectionOutcome::Duplicate {
                        record: Box::new(rec),
                        conversation: conv,
                    });
                }
                Some(Receipt::Different) => {
                    return Err(LinkCorrectionError::IdempotencyCollision {
                        command_key: command_key.to_owned(),
                    });
                }
                None | Some(Receipt::New) => {}
            }

            let conv = lock_conversation_in_tx(&tx, practice, conversation_id)
                .await?
                .ok_or_else(|| LinkCorrectionError::ConversationNotFound {
                    practice: practice.to_owned(),
                    conversation_id: conversation_id.to_owned(),
                })?;
            let latest =
                latest_correction_for_conversation_in_tx(&tx, practice, conversation_id).await?;

            // Enforce access on current workspace (or Undo origin if currently MovedNotAPatient during Undo).
            match &conv.link_state {
                ConversationLinkState::Linked { workspace, .. } => {
                    ensure_physician_workspace_access(auth, actor, workspace).await?;
                }
                ConversationLinkState::MovedNotAPatient { .. } => {
                    if let Some(undo) = conv
                        .move_undo
                        .as_ref()
                        .filter(|u| u.is_active_at(cmd.now_ts))
                    {
                        ensure_physician_workspace_access(auth, actor, &undo.from_workspace)
                            .await?;
                    } else if let Some(rec) = &latest
                        && rec.destination == CorrectionDestination::NotAPatient
                    {
                        ensure_physician_workspace_access(auth, actor, &rec.original_workspace)
                            .await?;
                    } else {
                        return Err(LinkCorrectionError::InvalidTransition {
                            conversation_id: conversation_id.to_owned(),
                            reason: "conversation is already moved to not-a-patient and Undo has expired".to_owned(),
                        });
                    }
                }
                ConversationLinkState::Unlinked => {
                    return Err(LinkCorrectionError::InvalidTransition {
                        conversation_id: conversation_id.to_owned(),
                        reason: "conversation is unlinked; link it to a patient first".to_owned(),
                    });
                }
            }

            // If an earlier correction is in `SettlingFailed`:
            // - Repeating the same target destination saves NO new correction (`Duplicate`), keeping
            //   the failed standing unfinished and content unavailable without duplicating reviews.
            // - Moving back to origin or to any other target while unfinished is refused so
            //   repeated/follow-up commands cannot restore wrong-patient content (`AC4`).
            if let Some(rec) = &latest
                && let LinkCorrectionStanding::SettlingFailed { reason, .. } = &rec.standing
            {
                if rec.destination == cmd.destination {
                    let _ = receipt::record(
                        &tx,
                        practice,
                        &actor.physician_id,
                        command_key,
                        &command_body,
                    )
                    .await?;
                    tx.commit().await?;
                    return Ok(LinkCorrectionOutcome::Duplicate {
                        record: Box::new(rec.clone()),
                        conversation: conv,
                    });
                }
                let blocked_ws = match &cmd.destination {
                    CorrectionDestination::Patient { workspace } => workspace.clone(),
                    CorrectionDestination::NotAPatient => rec.original_workspace.clone(),
                };
                return Err(LinkCorrectionError::UnfinishedCorrectionBlocksReturn {
                    conversation_id: conversation_id.to_owned(),
                    workspace: blocked_ws,
                    correction_id: rec.correction_id.clone(),
                    reason: reason.clone(),
                });
            }

            // Repeated command targeting the already-applied destination of the latest correction
            // saves one correction (`Duplicate`) instead of creating a second correction.
            let already_at_destination = match (&conv.link_state, &cmd.destination) {
                (
                    ConversationLinkState::Linked { workspace: cur, .. },
                    CorrectionDestination::Patient { workspace: dest },
                ) => cur == dest,
                (
                    ConversationLinkState::MovedNotAPatient { .. },
                    CorrectionDestination::NotAPatient,
                ) => true,
                _ => false,
            };
            if already_at_destination {
                if let Some(rec) = latest.filter(|r| {
                    r.destination == cmd.destination
                        && matches!(
                            r.standing,
                            LinkCorrectionStanding::UndoWindow { .. }
                                | LinkCorrectionStanding::Settled { .. }
                        )
                }) {
                    let _ = receipt::record(
                        &tx,
                        practice,
                        &actor.physician_id,
                        command_key,
                        &command_body,
                    )
                    .await?;
                    tx.commit().await?;
                    return Ok(LinkCorrectionOutcome::Duplicate {
                        record: Box::new(rec),
                        conversation: conv,
                    });
                }
                return Err(LinkCorrectionError::InvalidTransition {
                    conversation_id: conversation_id.to_owned(),
                    reason: "conversation is already at the requested destination".to_owned(),
                });
            }

            if let CorrectionDestination::Patient { workspace } = &cmd.destination {
                check_unfinished_correction_blocking_workspace_in_tx(
                    &tx,
                    practice,
                    conversation_id,
                    workspace,
                )
                .await?;
            }

            let expired_id = latest.and_then(|rec| match rec.standing {
                LinkCorrectionStanding::UndoWindow { expires_at, .. }
                    if cmd.now_ts >= expires_at =>
                {
                    Some(rec.correction_id)
                }
                _ => None,
            });
            tx.commit().await?;
            expired_id
        };

        // If a prior Undo window on this conversation already expired, settle it first before starting
        // a brand-new correction epoch.
        if let Some(expired_correction_id) = expired_correction_to_settle {
            self.settle_link_correction_with_probe(
                practice,
                &expired_correction_id,
                review_receiver,
                probe,
                cmd.now_ts,
            )
            .await?;
        }

        let tx = self.db.begin().await?;
        let mut conv = lock_conversation_in_tx(&tx, practice, conversation_id)
            .await?
            .ok_or_else(|| LinkCorrectionError::ConversationNotFound {
                practice: practice.to_owned(),
                conversation_id: conversation_id.to_owned(),
            })?;

        let departed_workspace = match &conv.link_state {
            ConversationLinkState::Linked { workspace, .. } => {
                ensure_physician_workspace_access(auth, actor, workspace).await?;
                Some(workspace.clone())
            }
            ConversationLinkState::MovedNotAPatient { .. } => {
                let Some(undo) = conv
                    .move_undo
                    .as_ref()
                    .filter(|u| u.is_active_at(cmd.now_ts))
                else {
                    return Err(LinkCorrectionError::InvalidTransition {
                        conversation_id: conversation_id.to_owned(),
                        reason: "conversation is moved to not-a-patient and Undo has expired"
                            .to_owned(),
                    });
                };
                ensure_physician_workspace_access(auth, actor, &undo.from_workspace).await?;
                None
            }
            ConversationLinkState::Unlinked => {
                return Err(LinkCorrectionError::InvalidTransition {
                    conversation_id: conversation_id.to_owned(),
                    reason: "conversation is unlinked".to_owned(),
                });
            }
        };

        let current_linked_msgs =
            linked_messages_for_conversation_in_tx(&tx, practice, conversation_id).await?;
        let mut current_linked_ids: BTreeSet<String> = current_linked_msgs
            .iter()
            .map(|m| m.message_id.clone())
            .collect();

        // When moving from MovedNotAPatient -> Patient during active Undo, restore NotAPatient messages
        // back to Linked so all conversation messages travel together.
        if departed_workspace.is_none()
            && matches!(cmd.destination, CorrectionDestination::Patient { .. })
        {
            let msg_rows = spruce_entities::spruce_message::Entity::find()
                .filter(spruce_entities::spruce_message::Column::Practice.eq(practice))
                .filter(spruce_entities::spruce_message::Column::ConversationId.eq(conversation_id))
                .lock_exclusive()
                .all(&tx)
                .await?;
            for row in msg_rows {
                let mut msg = decode_message_row(row)?;
                if matches!(msg.standing, MessageStanding::NotAPatient { .. }) {
                    msg.standing = MessageStanding::Linked {
                        linked_at: cmd.now_ts,
                        origin: LinkOrigin::RestoredByMoveUndo {
                            physician_id: actor.physician_id.clone(),
                        },
                    };
                    msg.ever_linked = true;
                    msg.updated_at = cmd.now_ts;
                    save_message_in_tx(&tx, &msg).await?;
                    current_linked_ids.insert(msg.message_id);
                }
            }
        }

        let target_ws_opt = match &cmd.destination {
            CorrectionDestination::Patient { workspace } => Some(workspace.as_str()),
            CorrectionDestination::NotAPatient => None,
        };
        conv.open_viewers =
            still_open_viewers(auth, practice, &conv.open_viewers, target_ws_opt).await;

        let undo_window = cmd.undo_window_seconds.max(1);
        let active_prior_undo = conv
            .move_undo
            .clone()
            .filter(|u| u.is_active_at(cmd.now_ts));
        let returned_to_origin = active_prior_undo.as_ref().is_some_and(|u| {
            matches!(
                &cmd.destination,
                CorrectionDestination::Patient { workspace } if workspace == &u.from_workspace
            )
        });

        conv.move_undo = if returned_to_origin {
            None
        } else if let Some(prior) = &active_prior_undo {
            Some(PendingMoveUndo {
                by_physician: actor.physician_id.clone(),
                from_workspace: prior.from_workspace.clone(),
                moved_at: cmd.now_ts,
                expires_at: cmd.now_ts.saturating_add(undo_window),
                moved_message_ids: prior.moved_message_ids.clone(),
            })
        } else {
            let origin_ws = departed_workspace
                .clone()
                .ok_or_else(|| error("missing departed workspace for initial move"))?;
            Some(PendingMoveUndo {
                by_physician: actor.physician_id.clone(),
                from_workspace: origin_ws,
                moved_at: cmd.now_ts,
                expires_at: cmd.now_ts.saturating_add(undo_window),
                moved_message_ids: current_linked_ids.clone(),
            })
        };

        conv.link_state = match &cmd.destination {
            CorrectionDestination::Patient { workspace } => ConversationLinkState::Linked {
                workspace: workspace.clone(),
                linked_at: cmd.now_ts,
            },
            CorrectionDestination::NotAPatient => ConversationLinkState::MovedNotAPatient {
                moved_at: cmd.now_ts,
            },
        };
        conv.move_count += 1;
        conv.last_move_acted = false;
        conv.updated_at = cmd.now_ts;
        save_conversation_in_tx(&tx, &conv).await?;

        let mut pending_triggers: Vec<PendingSpruceReviewTrigger> = Vec::new();
        if matches!(cmd.destination, CorrectionDestination::NotAPatient) {
            let msg_rows = spruce_entities::spruce_message::Entity::find()
                .filter(spruce_entities::spruce_message::Column::Practice.eq(practice))
                .filter(spruce_entities::spruce_message::Column::ConversationId.eq(conversation_id))
                .lock_exclusive()
                .all(&tx)
                .await?;
            for row in msg_rows {
                let mut msg = decode_message_row(row)?;
                if matches!(msg.standing, MessageStanding::Linked { .. }) {
                    msg.standing = MessageStanding::NotAPatient {
                        moved_at: cmd.now_ts,
                    };
                    msg.updated_at = cmd.now_ts;
                    save_message_in_tx(&tx, &msg).await?;
                }
            }
        } else if returned_to_origin
            && let Some(prior_undo) = &active_prior_undo
            && current_linked_ids
                .iter()
                .any(|id| !prior_undo.moved_message_ids.contains(id))
        {
            let arrival_key = format!(
                "spruce-move-return-arrivals:{practice}:{conversation_id}:move-{}:{}",
                conv.move_count, prior_undo.from_workspace
            );
            pending_triggers.push(
                queue_review_trigger_in_tx(
                    &tx,
                    practice,
                    conversation_id,
                    &prior_undo.from_workspace,
                    &arrival_key,
                    SpruceReviewTriggerOrigin::LinkedConversationArrival,
                    cmd.now_ts,
                )
                .await?,
            );
        }

        let record = record_move_correction_in_tx(
            &tx,
            MoveCorrectionWrite {
                practice,
                conversation_id,
                actor_physician_id: &actor.physician_id,
                departed_workspace: departed_workspace.as_deref(),
                departed_linked_ids: &current_linked_ids,
                active_prior_undo: active_prior_undo.as_ref(),
                new_undo: conv.move_undo.as_ref(),
                returned_to_origin,
                destination: &cmd.destination,
                move_ordinal: conv.move_count,
                now_ts: cmd.now_ts,
            },
        )
        .await?;

        let _ = receipt::record(
            &tx,
            practice,
            &actor.physician_id,
            command_key,
            &command_body,
        )
        .await?;
        tx.commit().await?;

        dispatch_trigger_list(self, &pending_triggers, review_receiver, cmd.now_ts).await?;
        if returned_to_origin {
            Ok(LinkCorrectionOutcome::CancelledReturnedToOrigin {
                record: Box::new(record),
                conversation: conv,
            })
        } else {
            Ok(LinkCorrectionOutcome::Moved {
                record: Box::new(record),
                conversation: conv,
            })
        }
    }

    /// Undoes an active wrong-patient link correction before `Undo` expires,
    /// returning the conversation (including any messages received during the `Undo`
    /// window) to the patient linked before the first move (`AC1`, `AC3`, `AC4`).
    pub async fn undo_link_correction(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        cmd: UndoLinkCorrectionCommand,
        review_receiver: Option<&dyn ReviewReceiver>,
    ) -> Result<LinkCorrectionOutcome, LinkCorrectionError> {
        let practice = require_non_empty(&cmd.practice, "practice")?;
        let conversation_id = require_non_empty(&cmd.conversation_id, "conversation_id")?;
        let command_key = require_non_empty(&cmd.command_key, "command_key")?;
        if actor.practice_id != practice {
            return Err(LinkCorrectionError::AccessDenied {
                physician_id: actor.physician_id.clone(),
                subject: practice.to_owned(),
                reason: "actor practice does not match target practice",
            });
        }
        require_active_physician(auth, actor)
            .await
            .map_err(|err| match err {
                SpruceMatchingError::AccessDenied {
                    physician, reason, ..
                } => LinkCorrectionError::AccessDenied {
                    physician_id: physician,
                    subject: practice.to_owned(),
                    reason,
                },
                other => LinkCorrectionError::SpruceMatching(other),
            })?;

        let command_body = json!({
            "op": "undo_link_correction",
            "practice": practice,
            "conversation_id": conversation_id,
        });

        let tx = self.db.begin().await?;
        match receipt::check(
            &tx,
            practice,
            &actor.physician_id,
            command_key,
            &command_body,
        )
        .await?
        {
            Some(Receipt::Same) => {
                let conv = lock_conversation_in_tx(&tx, practice, conversation_id)
                    .await?
                    .ok_or_else(|| LinkCorrectionError::ConversationNotFound {
                        practice: practice.to_owned(),
                        conversation_id: conversation_id.to_owned(),
                    })?;
                let rec = latest_correction_for_conversation_in_tx(&tx, practice, conversation_id)
                    .await?
                    .ok_or_else(|| LinkCorrectionError::ConversationNotFound {
                        practice: practice.to_owned(),
                        conversation_id: conversation_id.to_owned(),
                    })?;
                let triggers =
                    undispatched_triggers_for_conversation_in_tx(&tx, practice, conversation_id)
                        .await?;
                tx.commit().await?;
                dispatch_trigger_list(self, &triggers, review_receiver, cmd.now_ts).await?;
                return Ok(LinkCorrectionOutcome::Duplicate {
                    record: Box::new(rec),
                    conversation: conv,
                });
            }
            Some(Receipt::Different) => {
                return Err(LinkCorrectionError::IdempotencyCollision {
                    command_key: command_key.to_owned(),
                });
            }
            None | Some(Receipt::New) => {}
        }

        let mut conv = lock_conversation_in_tx(&tx, practice, conversation_id)
            .await?
            .ok_or_else(|| LinkCorrectionError::ConversationNotFound {
                practice: practice.to_owned(),
                conversation_id: conversation_id.to_owned(),
            })?;

        if let Some(latest) =
            latest_correction_for_conversation_in_tx(&tx, practice, conversation_id).await?
            && let LinkCorrectionStanding::SettlingFailed { reason, .. } = &latest.standing
        {
            return Err(LinkCorrectionError::UnfinishedCorrectionBlocksReturn {
                conversation_id: conversation_id.to_owned(),
                workspace: latest.original_workspace,
                correction_id: latest.correction_id,
                reason: reason.clone(),
            });
        }

        let Some(undo) = conv.move_undo.clone() else {
            return Err(LinkCorrectionError::InvalidTransition {
                conversation_id: conversation_id.to_owned(),
                reason: "no active Undo window on conversation".to_owned(),
            });
        };
        if !undo.is_active_at(cmd.now_ts) {
            return Err(LinkCorrectionError::InvalidTransition {
                conversation_id: conversation_id.to_owned(),
                reason: "Undo window has expired".to_owned(),
            });
        }

        ensure_physician_workspace_access(auth, actor, &undo.from_workspace).await?;
        if let ConversationLinkState::Linked {
            workspace: current_ws,
            ..
        } = &conv.link_state
        {
            ensure_physician_workspace_access(auth, actor, current_ws).await?;
        }
        check_unfinished_correction_blocking_workspace_in_tx(
            &tx,
            practice,
            conversation_id,
            &undo.from_workspace,
        )
        .await?;

        conv.link_state = ConversationLinkState::Linked {
            workspace: undo.from_workspace.clone(),
            linked_at: cmd.now_ts,
        };
        conv.open_viewers = still_open_viewers(
            auth,
            practice,
            &conv.open_viewers,
            Some(&undo.from_workspace),
        )
        .await;
        conv.move_undo = None;
        conv.updated_at = cmd.now_ts;
        save_conversation_in_tx(&tx, &conv).await?;

        let msg_rows = spruce_entities::spruce_message::Entity::find()
            .filter(spruce_entities::spruce_message::Column::Practice.eq(practice))
            .filter(spruce_entities::spruce_message::Column::ConversationId.eq(conversation_id))
            .order_by_asc(spruce_entities::spruce_message::Column::ChannelPosition)
            .lock_exclusive()
            .all(&tx)
            .await?;
        let mut saw_mid_undo_arrival = false;
        let mut restored_ids = Vec::new();
        for row in msg_rows {
            let mut msg = decode_message_row(row)?;
            if matches!(msg.standing, MessageStanding::NotAPatient { .. }) {
                msg.standing = MessageStanding::Linked {
                    linked_at: cmd.now_ts,
                    origin: LinkOrigin::RestoredByMoveUndo {
                        physician_id: actor.physician_id.clone(),
                    },
                };
                msg.ever_linked = true;
                msg.updated_at = cmd.now_ts;
                save_message_in_tx(&tx, &msg).await?;
            } else if matches!(msg.standing, MessageStanding::Linked { .. }) && !msg.ever_linked {
                msg.ever_linked = true;
                msg.updated_at = cmd.now_ts;
                save_message_in_tx(&tx, &msg).await?;
            }
            if matches!(msg.standing, MessageStanding::Linked { .. }) {
                if !undo.moved_message_ids.contains(&msg.message_id) {
                    saw_mid_undo_arrival = true;
                }
                restored_ids.push(msg.message_id);
            }
        }

        let mut triggers = Vec::new();
        if saw_mid_undo_arrival {
            let undo_arrival_key = format!(
                "spruce-undo-arrivals:{practice}:{conversation_id}:move-{}:{}",
                conv.move_count, undo.from_workspace
            );
            triggers.push(
                queue_review_trigger_in_tx(
                    &tx,
                    practice,
                    conversation_id,
                    &undo.from_workspace,
                    &undo_arrival_key,
                    SpruceReviewTriggerOrigin::LinkedConversationArrival,
                    cmd.now_ts,
                )
                .await?,
            );
        }

        let record = record_undo_correction_in_tx(
            &tx,
            practice,
            conversation_id,
            &actor.physician_id,
            restored_ids.clone(),
            cmd.now_ts,
        )
        .await?
        .ok_or_else(|| LinkCorrectionError::ConversationNotFound {
            practice: practice.to_owned(),
            conversation_id: conversation_id.to_owned(),
        })?;

        let _ = receipt::record(
            &tx,
            practice,
            &actor.physician_id,
            command_key,
            &command_body,
        )
        .await?;
        tx.commit().await?;

        dispatch_trigger_list(self, &triggers, review_receiver, cmd.now_ts).await?;
        Ok(LinkCorrectionOutcome::Undone {
            record: Box::new(record),
            conversation: conv,
            restored_message_ids: restored_ids,
        })
    }

    /// Settles one expired or previously failed wrong-patient link correction.
    pub async fn settle_link_correction(
        &self,
        practice: &str,
        correction_id: &str,
        review_receiver: Option<&dyn ReviewReceiver>,
        now_ts: i64,
    ) -> Result<LinkCorrectionRecord, LinkCorrectionError> {
        self.settle_link_correction_with_probe(
            practice,
            correction_id,
            review_receiver,
            &NoopLinkCorrectionRemovalProbe,
            now_ts,
        )
        .await
    }

    /// Settles one expired or previously failed wrong-patient link correction using `probe`,
    /// recording per-step progress and persisting `SettlingFailed` + `CleanupFailed` if any
    /// step fails (`AC2`, `AC4`, `AC5`).
    pub async fn settle_link_correction_with_probe(
        &self,
        practice: &str,
        correction_id: &str,
        review_receiver: Option<&dyn ReviewReceiver>,
        probe: &dyn LinkCorrectionRemovalProbe,
        now_ts: i64,
    ) -> Result<LinkCorrectionRecord, LinkCorrectionError> {
        let practice = require_non_empty(practice, "practice")?;
        let correction_id = require_non_empty(correction_id, "correction_id")?;

        let (mut record, actor_physician, origin_moved_msgs) = {
            let tx = self.db.begin().await?;
            let row = link_correction_entity::Entity::find_by_id((
                practice.to_owned(),
                correction_id.to_owned(),
            ))
            .lock_exclusive()
            .one(&tx)
            .await?
            .ok_or_else(|| LinkCorrectionError::CorrectionNotFound {
                practice: practice.to_owned(),
                correction_id: correction_id.to_owned(),
            })?;
            let rec = decode_link_correction_row(row)?;
            match &rec.standing {
                LinkCorrectionStanding::Settled { .. } => {
                    tx.commit().await?;
                    return Ok(rec);
                }
                LinkCorrectionStanding::Undone { .. }
                | LinkCorrectionStanding::CancelledReturnedToOrigin { .. } => {
                    return Err(LinkCorrectionError::InvalidTransition {
                        conversation_id: rec.conversation_id,
                        reason: "correction was undone or cancelled".to_owned(),
                    });
                }
                LinkCorrectionStanding::UndoWindow {
                    by_physician,
                    expires_at,
                    moved_message_ids,
                    ..
                } => {
                    if now_ts < *expires_at {
                        return Err(LinkCorrectionError::InvalidTransition {
                            conversation_id: rec.conversation_id,
                            reason: "Undo window has not expired yet".to_owned(),
                        });
                    }
                    let physician = by_physician.clone();
                    let msgs = moved_message_ids.clone();
                    tx.commit().await?;
                    (rec, physician, msgs)
                }
                LinkCorrectionStanding::SettlingFailed { .. } => {
                    let ws_row = link_correction_workspace_entity::Entity::find_by_id((
                        rec.original_workspace.clone(),
                        practice.to_owned(),
                        correction_id.to_owned(),
                    ))
                    .one(&tx)
                    .await?;
                    let msgs: BTreeSet<String> = match ws_row {
                        Some(w) => serde_json::from_value(w.message_ids)
                            .map_err(conversion_error("message_ids", "BTreeSet<String>"))?,
                        None => BTreeSet::new(),
                    };
                    tx.commit().await?;
                    (rec, "link-correction-recovery".to_owned(), msgs)
                }
            }
        };

        let mut progress = match &record.standing {
            LinkCorrectionStanding::SettlingFailed { progress, .. } => progress.clone(),
            _ => SettlementProgress::default(),
        };

        let left_workspaces = left_workspaces_for_record(&record);

        // Step 1: Mark moved messages `entered_in_error` without deleting them.
        if let Err(reason) = probe.before_step(
            &record.correction_id,
            &record.conversation_id,
            LinkCorrectionStep::MarkMessagesEnteredInError,
        ) {
            return self
                .persist_settlement_failure(
                    &mut record,
                    LinkCorrectionStep::MarkMessagesEnteredInError,
                    reason,
                    None,
                    progress,
                    now_ts,
                )
                .await;
        }
        if let Err(err) = self
            .execute_mark_entered_in_error_step(
                &record,
                &left_workspaces,
                &origin_moved_msgs,
                &mut progress,
                now_ts,
            )
            .await
        {
            let reason = err.to_string();
            return self
                .persist_settlement_failure(
                    &mut record,
                    LinkCorrectionStep::MarkMessagesEnteredInError,
                    reason,
                    Some(err),
                    progress,
                    now_ts,
                )
                .await;
        }

        // Step 2: Withdraw affected memory, understanding, concerns, follow-ups, and chart sources.
        if let Err(reason) = probe.before_step(
            &record.correction_id,
            &record.conversation_id,
            LinkCorrectionStep::WithdrawOriginEffects,
        ) {
            return self
                .persist_settlement_failure(
                    &mut record,
                    LinkCorrectionStep::WithdrawOriginEffects,
                    reason,
                    None,
                    progress,
                    now_ts,
                )
                .await;
        }
        if let Err(err) = self
            .execute_withdraw_origin_effects_step(
                &record,
                &left_workspaces,
                &actor_physician,
                &mut progress,
            )
            .await
        {
            let reason = err.to_string();
            return self
                .persist_settlement_failure(
                    &mut record,
                    LinkCorrectionStep::WithdrawOriginEffects,
                    reason,
                    Some(err),
                    progress,
                    now_ts,
                )
                .await;
        }

        // Step 3: Apply reply withholding (on patient destination) or closure + concern retirement (on NotAPatient).
        if let Err(reason) = probe.before_step(
            &record.correction_id,
            &record.conversation_id,
            LinkCorrectionStep::ApplyReplyMove,
        ) {
            return self
                .persist_settlement_failure(
                    &mut record,
                    LinkCorrectionStep::ApplyReplyMove,
                    reason,
                    None,
                    progress,
                    now_ts,
                )
                .await;
        }
        if let Err(err) = self
            .execute_apply_reply_move_step(&record, &mut progress)
            .await
        {
            let reason = err.to_string();
            return self
                .persist_settlement_failure(
                    &mut record,
                    LinkCorrectionStep::ApplyReplyMove,
                    reason,
                    Some(err),
                    progress,
                    now_ts,
                )
                .await;
        }

        // Step 4: Queue and dispatch exactly one patient review request per affected patient.
        if let Err(reason) = probe.before_step(
            &record.correction_id,
            &record.conversation_id,
            LinkCorrectionStep::RequestAffectedReviews,
        ) {
            return self
                .persist_settlement_failure(
                    &mut record,
                    LinkCorrectionStep::RequestAffectedReviews,
                    reason,
                    None,
                    progress,
                    now_ts,
                )
                .await;
        }
        if let Err(err) = self
            .execute_request_affected_reviews_step(
                &record,
                &left_workspaces,
                &mut progress,
                review_receiver,
                now_ts,
            )
            .await
        {
            let reason = err.to_string();
            return self
                .persist_settlement_failure(
                    &mut record,
                    LinkCorrectionStep::RequestAffectedReviews,
                    reason,
                    Some(err),
                    progress,
                    now_ts,
                )
                .await;
        }

        let tx = self.db.begin().await?;
        set_correction_workspaces_standing_in_tx(
            &tx,
            practice,
            &record.correction_id,
            true,
            Some(&WithheldStanding::SettledEnteredInError),
            now_ts,
        )
        .await?;
        record.standing = LinkCorrectionStanding::Settled {
            settled_at: now_ts,
            progress,
        };
        record.updated_at = now_ts;
        save_link_correction_in_tx(&tx, &record).await?;
        tx.commit().await?;
        Ok(record)
    }

    /// Settles every expired `UndoWindow` or previously failed `SettlingFailed` correction in `practice`.
    pub async fn settle_expired_link_corrections(
        &self,
        practice: &str,
        review_receiver: Option<&dyn ReviewReceiver>,
        now_ts: i64,
    ) -> Result<LinkCorrectionBatchReport, LinkCorrectionError> {
        self.settle_expired_link_corrections_with_probe(
            practice,
            review_receiver,
            &NoopLinkCorrectionRemovalProbe,
            now_ts,
        )
        .await
    }

    /// Batch settlement with an explicit [`LinkCorrectionRemovalProbe`].
    pub async fn settle_expired_link_corrections_with_probe(
        &self,
        practice: &str,
        review_receiver: Option<&dyn ReviewReceiver>,
        probe: &dyn LinkCorrectionRemovalProbe,
        now_ts: i64,
    ) -> Result<LinkCorrectionBatchReport, LinkCorrectionError> {
        let practice = require_non_empty(practice, "practice")?;
        let rows = link_correction_entity::Entity::find()
            .filter(link_correction_entity::Column::Practice.eq(practice))
            .filter(
                link_correction_entity::Column::StandingKind
                    .is_in(["undo_window", "settling_failed"]),
            )
            .order_by_asc(link_correction_entity::Column::CreatedAt)
            .order_by_asc(link_correction_entity::Column::CorrectionId)
            .all(&self.db)
            .await?;

        let mut report = LinkCorrectionBatchReport::default();
        for row in rows {
            let rec = decode_link_correction_row(row)?;
            if let LinkCorrectionStanding::UndoWindow { expires_at, .. } = &rec.standing
                && now_ts < *expires_at
            {
                continue;
            }
            match self
                .settle_link_correction_with_probe(
                    practice,
                    &rec.correction_id,
                    review_receiver,
                    probe,
                    now_ts,
                )
                .await
            {
                Ok(settled) => report.settled.push(settled),
                Err(LinkCorrectionError::RemovalStepFailed {
                    correction_id,
                    conversation_id,
                    step,
                    reason,
                    ..
                }) => report.failed.push(LinkCorrectionBatchFailure {
                    correction_id,
                    conversation_id,
                    failed_step: step,
                    reason,
                }),
                Err(other) => return Err(other),
            }
        }
        Ok(report)
    }

    async fn persist_settlement_failure<T>(
        &self,
        record: &mut LinkCorrectionRecord,
        step: LinkCorrectionStep,
        reason: String,
        source: Option<LinkCorrectionError>,
        progress: SettlementProgress,
        now_ts: i64,
    ) -> Result<T, LinkCorrectionError> {
        let tx = self.db.begin().await?;
        let withheld = WithheldStanding::CleanupFailed {
            failed_step: step,
            failure_reason: reason.clone(),
        };
        set_correction_workspaces_standing_in_tx(
            &tx,
            &record.practice,
            &record.correction_id,
            true,
            Some(&withheld),
            now_ts,
        )
        .await?;
        record.standing = LinkCorrectionStanding::SettlingFailed {
            failed_at: now_ts,
            failed_step: step,
            reason: reason.clone(),
            progress,
        };
        record.updated_at = now_ts;
        save_link_correction_in_tx(&tx, record).await?;
        tx.commit().await?;
        Err(LinkCorrectionError::RemovalStepFailed {
            correction_id: record.correction_id.clone(),
            conversation_id: record.conversation_id.clone(),
            step,
            reason,
            source: source.map(Box::new),
        })
    }

    async fn execute_mark_entered_in_error_step(
        &self,
        record: &LinkCorrectionRecord,
        left_workspaces: &[String],
        origin_moved_msgs: &BTreeSet<String>,
        progress: &mut SettlementProgress,
        now_ts: i64,
    ) -> Result<(), LinkCorrectionError> {
        let tx = self.db.begin().await?;
        let move_ordinal_i64 =
            i64::try_from(record.move_ordinal).map_err(conversion_error("move_ordinal", "i64"))?;
        let move_dest = record.destination.to_move_destination();
        let dest_json = serde_json::to_value(&move_dest)
            .map_err(conversion_error("MoveDestination", "json"))?;

        for ws in left_workspaces {
            let ws_row = link_correction_workspace_entity::Entity::find_by_id((
                ws.clone(),
                record.practice.clone(),
                record.correction_id.clone(),
            ))
            .one(&tx)
            .await?;
            let msg_ids: BTreeSet<String> = match ws_row {
                Some(w) => serde_json::from_value(w.message_ids)
                    .map_err(conversion_error("message_ids", "BTreeSet<String>"))?,
                None if ws == &record.original_workspace => origin_moved_msgs.clone(),
                None => BTreeSet::new(),
            };
            let ids_to_mark = if ws == &record.original_workspace && !origin_moved_msgs.is_empty() {
                origin_moved_msgs
            } else {
                &msg_ids
            };
            for message_id in ids_to_mark {
                spruce_entities::spruce_entered_in_error::Entity::insert(
                    spruce_entities::spruce_entered_in_error::ActiveModel {
                        practice: Set(record.practice.clone()),
                        from_workspace: Set(ws.clone()),
                        message_id: Set(message_id.clone()),
                        move_ordinal: Set(move_ordinal_i64),
                        conversation_id: Set(record.conversation_id.clone()),
                        destination: Set(dest_json.clone()),
                        marked_at: Set(now_ts),
                    },
                )
                .on_conflict(
                    OnConflict::columns([
                        spruce_entities::spruce_entered_in_error::Column::Practice,
                        spruce_entities::spruce_entered_in_error::Column::FromWorkspace,
                        spruce_entities::spruce_entered_in_error::Column::MessageId,
                        spruce_entities::spruce_entered_in_error::Column::MoveOrdinal,
                    ])
                    .do_nothing()
                    .to_owned(),
                )
                .exec_without_returning(&tx)
                .await?;
                progress
                    .entered_in_error_message_ids
                    .insert(message_id.clone());
            }
        }
        tx.commit().await?;
        Ok(())
    }

    async fn execute_withdraw_origin_effects_step(
        &self,
        record: &LinkCorrectionRecord,
        left_workspaces: &[String],
        actor_physician: &str,
        progress: &mut SettlementProgress,
    ) -> Result<(), LinkCorrectionError> {
        for ws in left_workspaces {
            if progress.withdrawal_ids_by_workspace.contains_key(ws) {
                continue;
            }
            let artifacts = {
                let tx = self.db.begin().await?;
                let existing_ws = link_correction_workspace_entity::Entity::find_by_id((
                    ws.clone(),
                    record.practice.clone(),
                    record.correction_id.clone(),
                ))
                .one(&tx)
                .await?;
                let seed_msgs: BTreeSet<String> = match &existing_ws {
                    Some(w) => serde_json::from_value(w.message_ids.clone())
                        .map_err(conversion_error("message_ids", "BTreeSet<String>"))?,
                    None => BTreeSet::new(),
                };
                let mut fresh = discover_conversation_artifacts_in_tx(
                    &tx,
                    &record.practice,
                    ws,
                    &record.conversation_id,
                    &seed_msgs,
                )
                .await?;
                if let Some(w) = existing_ws {
                    let prev_cands: BTreeSet<String> = serde_json::from_value(w.candidate_ids)
                        .map_err(conversion_error("candidate_ids", "BTreeSet<String>"))?;
                    let prev_works: BTreeSet<String> = serde_json::from_value(w.work_ids)
                        .map_err(conversion_error("work_ids", "BTreeSet<String>"))?;
                    let prev_replies: BTreeSet<u64> = serde_json::from_value(w.reply_ids)
                        .map_err(conversion_error("reply_ids", "BTreeSet<u64>"))?;
                    let prev_drafts: BTreeSet<String> = serde_json::from_value(w.draft_keys)
                        .map_err(conversion_error("draft_keys", "BTreeSet<String>"))?;
                    fresh.candidate_ids.extend(prev_cands);
                    fresh.work_ids.extend(prev_works);
                    fresh.reply_ids.extend(prev_replies);
                    fresh.draft_keys.extend(prev_drafts);
                    let current_standing: WithheldStanding =
                        serde_json::from_value(w.withheld_standing)
                            .map_err(conversion_error("withheld_standing", "WithheldStanding"))?;
                    upsert_workspace_withholding_in_tx(
                        &tx,
                        WorkspaceWithholdingUpsert {
                            workspace: ws,
                            practice: &record.practice,
                            correction_id: &record.correction_id,
                            conversation_id: &record.conversation_id,
                            active_withholding: true,
                            withheld_standing: &current_standing,
                            artifacts: &fresh,
                            updated_at: record.updated_at,
                        },
                    )
                    .await?;
                }
                tx.commit().await?;
                fresh
            };

            let digest = &crate::sha256_hex(
                format!(
                    "link-corr-withdraw:{}:{}:{}:{}",
                    record.practice, record.correction_id, record.conversation_id, ws
                )
                .as_bytes(),
            )[..24];
            let withdrawal_id = format!("lcw-{digest}");
            let reason = format!(
                "Conversation {} moved away from patient {ws}.",
                record.conversation_id
            );
            let mut candidate_ids: Vec<String> = artifacts.candidate_ids.iter().cloned().collect();
            if candidate_ids.is_empty() {
                candidate_ids.push(format!("conversation:{}", record.conversation_id));
            }
            let primary_message_id = artifacts.message_ids.iter().next().cloned();
            let fenced_work_ids: Vec<String> = artifacts.work_ids.iter().cloned().collect();

            let report = self
                .withdraw_wrong_patient_source(&WithdrawWrongPatientRequest {
                    practice: record.practice.clone(),
                    workspace: ws.clone(),
                    withdrawal_id: withdrawal_id.clone(),
                    candidate_ids,
                    message_id: primary_message_id,
                    fenced_work_ids,
                    reason,
                    actor: actor_physician.to_owned(),
                })
                .await?;
            progress
                .withdrawal_ids_by_workspace
                .insert(ws.clone(), report.receipt().withdrawal_id.clone());
        }
        Ok(())
    }

    async fn execute_apply_reply_move_step(
        &self,
        record: &LinkCorrectionRecord,
        progress: &mut SettlementProgress,
    ) -> Result<(), LinkCorrectionError> {
        if progress.reply_move_applied {
            return Ok(());
        }
        let target = match &record.destination {
            CorrectionDestination::Patient { workspace } => ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
            CorrectionDestination::NotAPatient => ConversationLink::NotAPatient,
        };
        let should_apply = {
            let tx = self.db.begin().await?;
            let row = reply_conversation::Entity::find_by_id((
                record.practice.clone(),
                record.conversation_id.clone(),
            ))
            .one(&tx)
            .await?;
            tx.commit().await?;
            match row {
                None => false,
                Some(r) => {
                    let current_link = match (r.link_kind.as_str(), r.workspace) {
                        ("linked_patient", Some(ws)) => {
                            Some(ConversationLink::LinkedPatient { workspace: ws })
                        }
                        ("not_a_patient", _) => Some(ConversationLink::NotAPatient),
                        _ => None,
                    };
                    current_link.as_ref() != Some(&target)
                }
            }
        };
        if should_apply {
            let digest = &crate::sha256_hex(
                format!(
                    "link-corr-reply:{}:{}:{}",
                    record.practice, record.correction_id, record.conversation_id
                )
                .as_bytes(),
            )[..24];
            let cmd_key = CommandKey::new(format!("lc-reply:{digest}")).map_err(|cause| {
                LinkCorrectionError::InvalidField {
                    field: "reply_move.command_key",
                    reason: cause.to_string(),
                }
            })?;
            let _ = self
                .act_on_conversation_move(
                    &cmd_key,
                    &record.practice,
                    &record.conversation_id,
                    target,
                )
                .await?;
        }
        progress.reply_move_applied = true;
        Ok(())
    }

    async fn execute_request_affected_reviews_step(
        &self,
        record: &LinkCorrectionRecord,
        left_workspaces: &[String],
        progress: &mut SettlementProgress,
        review_receiver: Option<&dyn ReviewReceiver>,
        now_ts: i64,
    ) -> Result<(), LinkCorrectionError> {
        let triggers = {
            let tx = self.db.begin().await?;
            if let Some(mut conv) =
                lock_conversation_in_tx(&tx, &record.practice, &record.conversation_id).await?
            {
                conv.move_undo = None;
                conv.last_move_acted = true;
                conv.updated_at = now_ts;
                save_conversation_in_tx(&tx, &conv).await?;
            }

            for ws in left_workspaces {
                let from_key = format!(
                    "spruce-move-settled-from:{}:{}:move-{}:{}",
                    record.practice, record.conversation_id, record.move_ordinal, ws
                );
                let _ = queue_review_trigger_in_tx(
                    &tx,
                    &record.practice,
                    &record.conversation_id,
                    ws,
                    &from_key,
                    SpruceReviewTriggerOrigin::MoveSettledFrom,
                    now_ts,
                )
                .await?;
                progress
                    .review_command_keys_by_workspace
                    .insert(ws.clone(), from_key);
            }

            if let CorrectionDestination::Patient { workspace } = &record.destination {
                let to_key = format!(
                    "spruce-move-settled-to:{}:{}:move-{}:{}",
                    record.practice, record.conversation_id, record.move_ordinal, workspace
                );
                let _ = queue_review_trigger_in_tx(
                    &tx,
                    &record.practice,
                    &record.conversation_id,
                    workspace,
                    &to_key,
                    SpruceReviewTriggerOrigin::MoveSettledTo,
                    now_ts,
                )
                .await?;
                progress
                    .review_command_keys_by_workspace
                    .insert(workspace.clone(), to_key);
            }

            let pending = undispatched_triggers_for_conversation_in_tx(
                &tx,
                &record.practice,
                &record.conversation_id,
            )
            .await?;
            tx.commit().await?;
            pending
        };

        dispatch_trigger_list(self, &triggers, review_receiver, now_ts).await?;
        Ok(())
    }

    /// Reads one saved [`LinkCorrectionRecord`] by ID.
    pub async fn link_correction(
        &self,
        practice: &str,
        correction_id: &str,
    ) -> Result<Option<LinkCorrectionRecord>, LinkCorrectionError> {
        let practice = require_non_empty(practice, "practice")?;
        let correction_id = require_non_empty(correction_id, "correction_id")?;
        let row = link_correction_entity::Entity::find_by_id((
            practice.to_owned(),
            correction_id.to_owned(),
        ))
        .one(&self.db)
        .await?;
        Ok(row.map(decode_link_correction_row).transpose()?)
    }

    /// Reads the latest saved [`LinkCorrectionRecord`] for `conversation_id`.
    pub async fn latest_conversation_link_correction(
        &self,
        practice: &str,
        conversation_id: &str,
    ) -> Result<Option<LinkCorrectionRecord>, LinkCorrectionError> {
        let practice = require_non_empty(practice, "practice")?;
        let conversation_id = require_non_empty(conversation_id, "conversation_id")?;
        Ok(latest_correction_for_conversation_in_tx(&self.db, practice, conversation_id).await?)
    }

    /// Lists all saved [`LinkCorrectionRecord`] rows for `conversation_id` in epoch order.
    pub async fn conversation_link_corrections(
        &self,
        practice: &str,
        conversation_id: &str,
    ) -> Result<Vec<LinkCorrectionRecord>, LinkCorrectionError> {
        let practice = require_non_empty(practice, "practice")?;
        let conversation_id = require_non_empty(conversation_id, "conversation_id")?;
        let rows = link_correction_entity::Entity::find()
            .filter(link_correction_entity::Column::Practice.eq(practice))
            .filter(link_correction_entity::Column::ConversationId.eq(conversation_id))
            .order_by_asc(link_correction_entity::Column::CorrectionEpoch)
            .all(&self.db)
            .await?;
        Ok(rows
            .into_iter()
            .map(decode_link_correction_row)
            .collect::<Result<Vec<_>, _>>()?)
    }

    /// Reads active withholding state for `workspace`.
    pub async fn workspace_withheld_state(
        &self,
        workspace: &str,
    ) -> Result<WithheldWorkspaceState, LinkCorrectionError> {
        let workspace = require_non_empty(workspace, "workspace")?;
        Ok(load_workspace_withheld_state_in_tx(&self.db, workspace).await?)
    }

    /// Authorized read of active withholding state for `workspace` (`AC1`).
    pub async fn authorized_workspace_withheld_state(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        workspace: &str,
    ) -> Result<WithheldWorkspaceState, LinkCorrectionError> {
        let workspace = require_non_empty(workspace, "workspace")?;
        ensure_physician_workspace_access(auth, actor, workspace).await?;
        Ok(load_workspace_withheld_state_in_tx(&self.db, workspace).await?)
    }

    /// Authorized direct read of current patient chart sources (`AC1`, `AC2`).
    pub async fn authorized_workspace_sources(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        workspace: &str,
    ) -> Result<Vec<SourceRevision>, LinkCorrectionError> {
        let workspace = require_non_empty(workspace, "workspace")?;
        ensure_physician_workspace_access(auth, actor, workspace).await?;
        Ok(self.sources(workspace).await?.sources)
    }

    /// Authorized direct read of linked Spruce messages on `workspace` (`AC1`, `AC2`).
    pub async fn authorized_patient_spruce_messages(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        workspace: &str,
    ) -> Result<Vec<super::spruce_matching::SavedSpruceMessage>, LinkCorrectionError> {
        let workspace = require_non_empty(workspace, "workspace")?;
        ensure_physician_workspace_access(auth, actor, workspace).await?;
        Ok(self
            .patient_linked_spruce_messages(&actor.practice_id, workspace)
            .await?)
    }

    /// Authorized direct read of `Past work` history on `workspace` (`AC1`, `AC2`, `AC6`).
    pub async fn authorized_workspace_past_work(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        workspace: &str,
    ) -> Result<Vec<WorkspacePastWorkItem>, LinkCorrectionError> {
        let workspace = require_non_empty(workspace, "workspace")?;
        ensure_physician_workspace_access(auth, actor, workspace).await?;
        Ok(self
            .workspace_past_work(&actor.practice_id, &actor.physician_id, workspace)
            .await?)
    }

    /// Authorized direct read of a reopened conversation with citations (`AC1`, `AC2`, `AC6`).
    pub async fn authorized_reopen_conversation(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        workspace: &str,
        chat_id: &str,
    ) -> Result<Option<SavedConversationReadback>, LinkCorrectionError> {
        let workspace = require_non_empty(workspace, "workspace")?;
        let chat_id = require_non_empty(chat_id, "chat_id")?;
        ensure_physician_workspace_access(auth, actor, workspace).await?;
        let scope = Scope {
            practice: actor.practice_id.clone(),
            physician: actor.physician_id.clone(),
            workspace: workspace.to_owned(),
            chat: chat_id.to_owned(),
        };
        Ok(self.reopen_conversation(&scope).await?)
    }

    /// Authorized direct read of open patient concerns on `workspace` (`AC1`, `AC5`).
    pub async fn authorized_open_concerns(
        &self,
        auth: &dyn AuthStore,
        actor: &PhysicianIdentity,
        workspace: &str,
        session_hash: &[u8],
        authority: &mut dyn Authority,
    ) -> Result<Vec<Concern>, LinkCorrectionError> {
        let workspace = require_non_empty(workspace, "workspace")?;
        ensure_physician_workspace_access(auth, actor, workspace).await?;
        let access = super::concerns::WorkspaceAccess {
            practice: &actor.practice_id,
            physician: &actor.physician_id,
            workspace,
            session_hash,
        };
        Ok(self.open_concerns(&access, authority).await?)
    }
}
