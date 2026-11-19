//! Durable reply proposals, conversation review fencing, channel moves and
//! disconnects, copy attribution, and private per-physician draft coordination.
//!
//! Each linked conversation (`(practice, conversation_id)`) tracks its patient
//! link, channel connection, open and handled patient messages, observed
//! practice messages, deduplicated review wake events, active review read
//! currency, and current reply standing (`None`, `Ready`, `Withheld`, or
//! `ClosedWithUndo`). Proposed replies are saved append-only by version with
//! verified chart citations and basis metadata. Private physician drafts keep
//! their own rows in [`super::Draft`] and bind to a specific `reply_id` so two
//! physicians never overwrite each other's drafts and a patient move never
//! carries an earlier patient's draft onto the new patient's reply.

use super::super::concerns::{ConcernError, ConcernId, ConcernInputError, ExactRecordReference};
use super::super::understanding::SummaryCitation;
use super::super::{RuntimeError, Scope};
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;

/// Prefix identifying a [`Scope::chat`] owned by a reply proposal draft.
pub const PROPOSAL_DRAFT_CHAT_PREFIX: &str = "reply-proposal:";
const MAX_PROPOSAL_CITATIONS: usize = 64;

/// SeaORM table definitions for reply proposals and physician draft bindings.
pub(in crate::runtime) mod tables {
    /// Conversation-level reply state and review wake queue.
    pub(in crate::runtime) mod reply_conversation {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_reply_conversations")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub conversation_id: String,
            #[sea_orm(column_type = "Text")]
            pub link_kind: String,
            #[sea_orm(column_type = "Text", nullable)]
            pub workspace: Option<String>,
            #[sea_orm(column_type = "Text")]
            pub channel_state: String,
            #[sea_orm(column_type = "Text")]
            pub reply_state: String,
            pub conversation_version: i64,
            pub move_epoch: i64,
            pub last_reply_id: i64,
            pub last_proposal_version: i64,
            pub last_practice_observation_seq: i64,
            #[sea_orm(column_type = "JsonBinary")]
            pub state: Json,
            pub created_at: i64,
            pub updated_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    /// Append-only proposed reply revisions with verified basis and citations.
    pub(in crate::runtime) mod reply_proposal {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_reply_proposals")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub conversation_id: String,
            #[sea_orm(primary_key, auto_increment = false)]
            pub version: i64,
            pub reply_id: i64,
            #[sea_orm(column_type = "Text")]
            pub workspace: String,
            #[sea_orm(column_type = "Text")]
            pub review_id: String,
            #[sea_orm(column_type = "Text")]
            pub run_id: String,
            #[sea_orm(column_type = "Text")]
            pub completion_key: String,
            pub conversation_version: i64,
            pub source_version: i64,
            pub move_epoch: i64,
            pub replaced_proposal_version: Option<i64>,
            #[sea_orm(column_type = "Text")]
            pub text: String,
            #[sea_orm(column_type = "JsonBinary")]
            pub covered_message_ids: Json,
            #[sea_orm(column_type = "JsonBinary")]
            pub basis: Json,
            #[sea_orm(column_type = "JsonBinary")]
            pub citations: Json,
            pub proposed_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    /// Once-only completion outcomes for reply review runs.
    pub(in crate::runtime) mod reply_review_completion {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_reply_review_completions")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub conversation_id: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub completion_key: String,
            #[sea_orm(column_type = "Text")]
            pub request_digest: String,
            #[sea_orm(column_type = "Text")]
            pub review_id: String,
            #[sea_orm(column_type = "Text")]
            pub run_id: String,
            #[sea_orm(column_type = "Text")]
            pub workspace: String,
            #[sea_orm(column_type = "JsonBinary")]
            pub outcome: Json,
            pub completed_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    /// Per-physician private draft standing bound to one `reply_id`.
    pub(in crate::runtime) mod reply_draft_binding {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_reply_draft_bindings")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub conversation_id: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub physician: String,
            #[sea_orm(primary_key, auto_increment = false)]
            pub reply_id: i64,
            #[sea_orm(column_type = "Text")]
            pub workspace: String,
            #[sea_orm(column_type = "Text")]
            pub draft_scope_chat: String,
            #[sea_orm(column_type = "Text")]
            pub standing: String,
            pub draft_revision: i64,
            pub based_on_proposal_version: i64,
            #[sea_orm(column_type = "Text", nullable)]
            pub pre_revision_your_text: Option<String>,
            pub update_offered_proposal_version: Option<i64>,
            pub updated_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }

    /// Append-only copy attribution records per reply copy action.
    pub(in crate::runtime) mod reply_copy {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_reply_copies")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub conversation_id: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub copy_id: String,
            #[sea_orm(column_type = "Text")]
            pub command_key: String,
            #[sea_orm(column_type = "Text")]
            pub physician: String,
            #[sea_orm(column_type = "Text")]
            pub workspace: String,
            pub reply_id: i64,
            pub practice_observation_seq: i64,
            #[sea_orm(column_type = "JsonBinary")]
            pub copied_source: Json,
            #[sea_orm(column_type = "Text")]
            pub copied_text: String,
            pub copied_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        impl ActiveModelBehavior for ActiveModel {}
    }
}

/// Boundary validation failure on reply proposal or draft coordination inputs.
#[derive(Debug, thiserror::Error)]
pub enum ReplyProposalInputError {
    /// Identifier was empty, untrimmed, over 256 characters, or contained control characters.
    #[error("invalid identifier for {field}: `{value}`")]
    InvalidIdentifier {
        /// Name of the invalid identifier field.
        field: &'static str,
        /// Rejected raw value.
        value: String,
    },
    /// Required text field was empty, exceeded its character limit, or held disallowed control bytes.
    #[error("invalid {field} text")]
    InvalidText {
        /// Name of the invalid text field.
        field: &'static str,
    },
    /// Too many chart record citations were attached to one proposal.
    #[error("reply proposal cites {count} records (maximum {MAX_PROPOSAL_CITATIONS})")]
    TooManyCitations {
        /// Number of citations supplied.
        count: usize,
    },
    /// Exact chart record reference failed validation.
    #[error("invalid cited chart record in {field}: {cause}")]
    InvalidRecordReference {
        /// Field containing the invalid reference.
        field: &'static str,
        /// Underlying validation error.
        #[source]
        cause: ConcernInputError,
    },
    /// Two citations or basis references named the same candidate with different revisions.
    #[error("conflicting cited revisions for chart candidate `{candidate_id}`")]
    ConflictingRecordRevision {
        /// Candidate identifier cited at conflicting revisions.
        candidate_id: String,
    },
    /// Two answered-closure entries named the same patient message with different practice messages.
    #[error("conflicting answered closures for patient message `{message_id}`")]
    ConflictingAnsweredClosure {
        /// Patient message identifier with conflicting closures.
        message_id: String,
    },
}

/// Failure on a reply conversation, proposal, review, or physician draft operation.
#[derive(Debug, thiserror::Error)]
pub enum ReplyProposalError {
    /// Input failed boundary validation.
    #[error("{0}")]
    InvalidInput(#[from] ReplyProposalInputError),
    /// Physician session or patient panel access was refused by [`Authority`].
    #[error("patient workspace access refused: {reason}")]
    AccessDenied {
        /// Refusal explanation from the authority owner.
        reason: String,
    },
    /// Command or completion key was reused with a different payload.
    #[error("idempotency key `{key}` already recorded a different command")]
    IdempotencyCollision {
        /// Colliding key.
        key: String,
    },
    /// Target reply conversation has not been linked.
    #[error("reply conversation `{conversation_id}` does not exist")]
    ConversationNotFound {
        /// Missing conversation identifier.
        conversation_id: String,
    },
    /// Operation requires a conversation linked to a patient workspace.
    #[error("conversation `{conversation_id}` is linked to not-a-patient")]
    ConversationNotLinkedToPatient {
        /// Conversation identifier.
        conversation_id: String,
    },
    /// Move targeted the patient workspace the conversation is already linked to.
    #[error("conversation `{conversation_id}` is already linked to workspace `{workspace}`")]
    MoveTargetUnchanged {
        /// Conversation identifier.
        conversation_id: String,
        /// Already-linked workspace.
        workspace: String,
    },
    /// Messaging channel is currently disconnected and cannot observe live channel arrivals.
    #[error("messaging channel for `{conversation_id}` is disconnected")]
    ChannelDisconnected {
        /// Conversation identifier.
        conversation_id: String,
    },
    /// Marking an open reply `Done` is permitted only while the messaging channel is disconnected.
    #[error("marking an open reply `Done` requires the messaging channel to be disconnected")]
    ChannelStillConnected {
        /// Conversation identifier.
        conversation_id: String,
    },
    /// No review wakeup is waiting when trying to start a new review.
    #[error("conversation `{conversation_id}` has no pending review wake events")]
    NoPendingReviewWake {
        /// Conversation identifier.
        conversation_id: String,
    },
    /// Review completion did not match the conversation's active review, workspace, or move epoch.
    #[error("stale or inactive reply review `{review_id}`: {reason}")]
    StaleReview {
        /// Rejected review identifier.
        review_id: String,
        /// Why the review is no longer current.
        reason: String,
    },
    /// Conversation gained unread events or restarted since the review last read it.
    #[error(
        "reply review `{review_id}` must read the current conversation (read {read_version:?}, current {current_version})"
    )]
    StaleConversationRead {
        /// Review identifier.
        review_id: String,
        /// Conversation version the review last read, if any.
        read_version: Option<u64>,
        /// Current conversation version.
        current_version: u64,
    },
    /// Patient chart sources changed since the review pinned them.
    #[error(
        "reply review `{review_id}` pinned source version {read_source_version}, but workspace is at {current_source_version}"
    )]
    StaleSourceObservation {
        /// Review identifier.
        review_id: String,
        /// Source version read by the review.
        read_source_version: u64,
        /// Current workspace source version.
        current_source_version: u64,
    },
    /// Proposal cited a chart record revision that is absent or superseded in workspace sources.
    #[error(
        "cited chart record `{candidate_id}` at revision `{revision}` is not in the workspace's current sources"
    )]
    UnverifiedCitation {
        /// Candidate identifier.
        candidate_id: String,
        /// Cited revision.
        revision: String,
    },
    /// Closure named an automatic practice reply, which never counts as answering patient messages.
    #[error(
        "automatic practice message `{practice_message_id}` cannot close patient messages as answered"
    )]
    AutomaticPracticeMessageCannotAnswer {
        /// Automatic practice message identifier.
        practice_message_id: String,
    },
    /// Closure named a practice message that the review did not read or that does not exist.
    #[error("practice message `{practice_message_id}` was not read by review `{review_id}`")]
    UnreadPracticeMessage {
        /// Review identifier.
        review_id: String,
        /// Unread or unknown practice message identifier.
        practice_message_id: String,
    },
    /// Closure named an unknown patient message identifier.
    #[error("patient message `{message_id}` does not exist on conversation `{conversation_id}`")]
    UnknownPatientMessage {
        /// Conversation identifier.
        conversation_id: String,
        /// Unknown patient message identifier.
        message_id: String,
    },
    /// Proposing a reply requires at least one open unanswered patient message.
    #[error("cannot propose a reply when no open patient messages remain")]
    NoOpenMessagesForProposal,
    /// Concluding without a reply while open messages remain requires `NoReplyWouldHelp`.
    #[error("open patient messages remain; supply a proposal or a `NoReplyWouldHelp` reason")]
    OpenMessagesRemainUnhandled,
    /// A physician explicitly asked Coda to draft a reply, so `NoReplyWouldHelp` is not permitted.
    #[error("physician requested a reply draft; review must propose a reply")]
    PhysicianRequestedReplyRequiresProposal,
    /// Action requires a ready reply on the conversation (or an open reply for `Done`).
    #[error("reply action refused because reply standing is `{standing}`")]
    ReplyNotInRequiredState {
        /// Current reply standing label.
        standing: &'static str,
    },
    /// `Undo` after `No reply needed` is no longer available.
    #[error("`No reply needed` undo is no longer available on `{conversation_id}`")]
    UndoNoLongerAvailable {
        /// Conversation identifier.
        conversation_id: String,
    },
    /// Physician draft edit or revision matched a stale proposal or draft version.
    #[error("stale physician draft base: {detail}")]
    StaleDraftVersion {
        /// Version mismatch explanation.
        detail: String,
    },
    /// Restoring a physician version requires an active Coda revision on the current reply.
    #[error("`Restore my version` requires an active Coda revision on the current reply")]
    NoRevisionToRestore,
    /// Choosing Coda's update requires an active update offer on the physician's draft.
    #[error("no newer Coda reply update is currently offered on this physician's draft")]
    NoUpdateOffered,
    /// Concern retirement failed during a `NotAPatient` move.
    #[error("concern retirement failed during move: {0}")]
    Concern(#[from] ConcernError),
    /// Runtime draft lifecycle error.
    #[error("{0}")]
    Runtime(#[from] RuntimeError),
    /// Database error from SeaORM.
    #[error("{0}")]
    Database(#[from] DbErr),
}

/// Patient link state of a messaging conversation (`link` in `MessageReply.tla`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ConversationLink {
    /// Linked to a patient workspace in the practice.
    LinkedPatient {
        /// Authorized Coda patient workspace identifier.
        workspace: String,
    },
    /// Determined not to belong to a patient.
    NotAPatient,
}

/// Messaging channel connection state (`channel` in `MessageReply.tla`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ChannelConnection {
    /// Channel is connected and delivering patient and practice messages.
    Connected,
    /// Channel is disconnected; open and withheld replies show the disconnect note and offer `Done`.
    Disconnected {
        /// Database-clock Unix second when the disconnect was recorded.
        disconnected_at: i64,
        /// Physician-readable disconnect note shown on the reply card.
        note: String,
    },
}

/// Who sent a message on the linked patient conversation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MessageSender {
    /// Sent by the linked patient themselves.
    Patient,
    /// Sent by another participant on the thread (such as a caregiver); shown alongside the patient.
    NamedSender {
        /// Display name of the non-patient sender.
        name: String,
    },
}

/// Why an open patient message settled as `NeedsNoReply`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "origin", rename_all = "snake_case")]
pub enum NoReplyReason {
    /// Review judged that no reply would help and recorded its clinical/operational reason.
    ReviewJudgedNoReplyWouldHelp {
        /// Review identifier that recorded the judgment.
        review_id: String,
        /// Physician-readable explanation saved with the outcome.
        reason: String,
    },
    /// An authorized physician closed the ready reply with `No reply needed`.
    PhysicianMarkedNoReplyNeeded {
        /// Physician who closed the reply.
        physician: String,
        /// Reply identity that was closed.
        reply_id: u64,
        /// Proposal version that was closed.
        reply_version: u64,
    },
}

/// Lifecycle standing of one linked patient message (`msg[i]` in `MessageReply.tla`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum PatientMessageStanding {
    /// Open and awaiting handling (`Coda is reviewing` or covered by an open reply).
    Open,
    /// Closed as answered by a non-automatic practice message read by the review.
    Answered {
        /// Cited practice message identifier that answered this message.
        practice_message_id: String,
        /// Review identifier that verified the answer.
        review_id: String,
        /// Database-clock Unix second when closed.
        answered_at: i64,
    },
    /// Closed as needing no reply (by Coda's review or physician `No reply needed`).
    NeedsNoReply {
        /// Saved reason and origin for needing no reply.
        reason: NoReplyReason,
        /// Database-clock Unix second when marked.
        decided_at: i64,
    },
    /// Marked `Done` by an authorized physician while the messaging channel was disconnected.
    MarkedDoneWhileDisconnected {
        /// Physician who marked the reply `Done`.
        physician: String,
        /// Database-clock Unix second when marked `Done`.
        marked_at: i64,
    },
    /// Closed when Coda acted on a move of the conversation to not-a-patient.
    MovedToNotPatient {
        /// Move epoch that retired the message from patient work.
        move_epoch: u64,
        /// Database-clock Unix second when moved.
        moved_at: i64,
    },
}

impl PatientMessageStanding {
    /// Whether this patient message is still open for reply work.
    pub const fn is_open(&self) -> bool {
        matches!(self, Self::Open)
    }
}

/// One patient-side message linked to a reply conversation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkedPatientMessage {
    /// Channel message identifier, unique within the conversation.
    pub message_id: String,
    /// Monotonic 1-based arrival ordinal on this conversation.
    pub sequence: u64,
    /// Whether the patient or a named third-party sender wrote the message.
    pub sender: MessageSender,
    /// Exact message body text.
    pub text: String,
    /// Channel timestamp (Unix seconds) when the sender sent the message.
    pub sent_at: i64,
    /// Conversation version at which Coda linked this message.
    pub linked_at_version: u64,
    /// Current message handling standing.
    pub standing: PatientMessageStanding,
}

/// Classification of a message sent by the practice on the channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PracticeMessageKind {
    /// Authored or sent by a person at the practice; eligible to answer open patient messages.
    PracticeSent,
    /// Automatic responder message; never answers open patient messages.
    Automatic,
}

/// One practice-sent channel message observed on the conversation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservedPracticeMessage {
    /// Channel practice message identifier, unique within the conversation.
    pub practice_message_id: String,
    /// Monotonic 1-based observation ordinal among practice messages.
    pub sequence: u64,
    /// Whether this was a human practice reply or an automatic responder.
    pub kind: PracticeMessageKind,
    /// Exact words observed on the channel.
    pub text: String,
    /// Channel send timestamp (Unix seconds).
    pub sent_at: i64,
    /// Database-clock Unix second when Coda observed the message.
    pub observed_at: i64,
    /// Conversation version at which Coda observed the message.
    pub observed_at_version: u64,
}

/// Practice message entry recorded in the patient's conversation history when read by a review.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedPracticeHistoryEntry {
    /// Channel practice message identifier.
    pub practice_message_id: String,
    /// Human practice reply vs automatic message.
    pub kind: PracticeMessageKind,
    /// Exact channel text sent by the practice (`Rule 8: HistoryTrue`).
    pub text: String,
    /// Channel send timestamp (Unix seconds).
    pub sent_at: i64,
    /// Database-clock Unix second when recorded into patient history.
    pub recorded_at: i64,
}

/// Dated evidence summary and explicit limitations saved alongside a proposed reply.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyProposalBasis {
    /// Concise summary of the records, freshness, and conversation context relied on.
    pub summary: String,
    /// Explicit missing, stale, or unavailable evidence limits.
    pub limits: Vec<String>,
    /// Exact chart record revisions referenced in the basis statement.
    pub cited_records: Vec<ExactRecordReference>,
}

/// One immutable reply proposal revision saved for a conversation (`physician_runtime_reply_proposals`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedReplyProposal {
    /// Practice tenant identifier.
    pub practice: String,
    /// Channel conversation identifier.
    pub conversation_id: String,
    /// Reply card identity (`reply.id` in `MessageReply.tla`).
    pub reply_id: u64,
    /// Monotonic proposal version on this conversation (`reply.v` in `MessageReply.tla`).
    pub version: u64,
    /// Patient workspace this proposal was authored for.
    pub workspace: String,
    /// Exact subset of open patient message IDs this proposal answers (`reply.msgs`).
    pub covered_message_ids: BTreeSet<String>,
    /// Proposed patient-facing reply text.
    pub text: String,
    /// Basis and limitations metadata saved with the proposal.
    pub basis: ReplyProposalBasis,
    /// Deduplicated, source-verified chart citations supporting the proposal and basis.
    pub citations: Vec<SummaryCitation>,
    /// Review identifier that published this proposal.
    pub review_id: String,
    /// Background run/attempt identifier that published this proposal.
    pub run_id: String,
    /// Idempotency completion key that published this proposal.
    pub completion_key: String,
    /// Conversation version read by the review.
    pub conversation_version: u64,
    /// Patient workspace source version pinned by the review.
    pub source_version: u64,
    /// Conversation move epoch when published.
    pub move_epoch: u64,
    /// Previous open proposal version on the same `reply_id` replaced by this proposal, when any.
    pub replaced_proposal_version: Option<u64>,
    /// Database-clock Unix second when saved.
    pub proposed_at: i64,
}

/// Conversation-wide reply lifecycle standing (`reply.state` in `MessageReply.tla`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ReplyStanding {
    /// No active or undoable reply exists.
    None,
    /// A proposed reply is ready for authorized physicians on `proposal.workspace`.
    Ready {
        /// Current ready reply proposal.
        proposal: Box<SavedReplyProposal>,
    },
    /// An earlier open reply is withheld after a move to `workspace` until `workspace`'s review finishes.
    Withheld {
        /// Incremented reply identity assigned on the move (`reply.id + 1`).
        reply_id: u64,
        /// New patient workspace the conversation moved to.
        workspace: String,
        /// Open message IDs carried from the withheld reply (`reply.msgs`).
        covered_message_ids: BTreeSet<String>,
        /// Earlier proposal version from before the move (never shown as text).
        withheld_from_proposal_version: u64,
        /// Move epoch that placed the reply into `Withheld`.
        move_epoch: u64,
        /// Database-clock Unix second when withheld.
        withheld_at: i64,
    },
    /// Closed by a physician with `No reply needed`; `Undo` remains available until `EndUndo`.
    ClosedWithUndo {
        /// Closed proposal restored if a physician presses `Undo` in time.
        proposal: Box<SavedReplyProposal>,
        /// Exact message IDs transitioned `Open -> NeedsNoReply` by that close.
        closed_message_ids: BTreeSet<String>,
        /// Physician who closed the reply.
        closed_by: String,
        /// Database-clock Unix second when closed.
        closed_at: i64,
    },
}

impl ReplyStanding {
    /// Whether the reply is currently open (`Ready` or `Withheld`, matching `ReplyOpen` in `MessageReply.tla`).
    pub const fn is_open(&self) -> bool {
        matches!(self, Self::Ready { .. } | Self::Withheld { .. })
    }
}

/// Deduplicated event waiting to start or re-read a conversation review (`woke` in `MessageReply.tla`).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ReviewWakeEvent {
    /// Linked patient message arrival.
    PatientMessage {
        /// Linked patient message identifier.
        message_id: String,
    },
    /// Observed practice channel message.
    PracticeMessage {
        /// Observed practice message identifier.
        practice_message_id: String,
    },
    /// Move to another patient workspace.
    MovedToPatient {
        /// Monotonic move epoch of the move.
        move_epoch: u64,
    },
    /// Explicit physician `Ask Coda to reply` request when no review was already active.
    PhysicianAskedForReply {
        /// Request identifier.
        request_id: String,
    },
}

/// Whether an active conversation review has read the latest conversation and chart state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ReviewReadState {
    /// Review restarted or has not yet read the current conversation (`review.current = FALSE`).
    NeedsRead,
    /// Review read the conversation and workspace sources at these versions (`review.current = TRUE`).
    Current {
        /// Conversation version read.
        conversation_version: u64,
        /// Workspace source observation version pinned during the read.
        source_version: u64,
        /// Practice message IDs present when the review read the conversation.
        read_practice_message_ids: BTreeSet<String>,
        /// Whether an open physician `Ask Coda to reply` request was present at read time.
        saw_physician_reply_request: bool,
    },
}

/// Active conversation review (`review` in `MessageReply.tla`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveReplyReview {
    /// Stable review identifier.
    pub review_id: String,
    /// Current worker run or attempt identifier.
    pub run_id: String,
    /// Patient workspace this review belongs to.
    pub workspace: String,
    /// Conversation move epoch when the review started.
    pub move_epoch: u64,
    /// Database-clock Unix second when started.
    pub started_at: i64,
    /// Read currency of this review.
    pub read_state: ReviewReadState,
}

/// Pending physician request from `Ask Coda to reply`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingPhysicianReplyRequest {
    /// Stable request identifier.
    pub request_id: String,
    /// Command key of the request.
    pub command_key: String,
    /// Requesting physician identifier.
    pub requested_by: String,
    /// Database-clock Unix second when requested.
    pub requested_at: i64,
}

/// Full durable state of one reply conversation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyConversation {
    /// Practice tenant identifier.
    pub practice: String,
    /// Channel conversation identifier.
    pub conversation_id: String,
    /// Current patient link (`LinkedPatient` or `NotAPatient`).
    pub link: ConversationLink,
    /// Current channel connection standing.
    pub channel: ChannelConnection,
    /// Monotonic conversation input version.
    pub conversation_version: u64,
    /// Number of moves Coda has acted on (`moves` in `MessageReply.tla`).
    pub move_epoch: u64,
    /// Highest allocated `reply_id` on this conversation.
    pub last_reply_id: u64,
    /// Highest allocated proposal `version` on this conversation.
    pub last_proposal_version: u64,
    /// Monotonic count of observed practice messages (used to scope copy attributions).
    pub last_practice_observation_seq: u64,
    /// Ordered linked patient messages.
    pub patient_messages: Vec<LinkedPatientMessage>,
    /// Ordered observed practice messages.
    pub practice_messages: Vec<ObservedPracticeMessage>,
    /// Exact practice message history recorded by reviews (`Rule 8: HistoryTrue`).
    pub recorded_practice_history: Vec<RecordedPracticeHistoryEntry>,
    /// Deduplicated events waiting for a review (`woke`).
    pub woke_events: BTreeSet<ReviewWakeEvent>,
    /// Currently running review, when one is active.
    pub active_review: Option<ActiveReplyReview>,
    /// Open physician `Ask Coda to reply` request awaiting satisfaction by a `Ready` proposal.
    pub pending_reply_request: Option<PendingPhysicianReplyRequest>,
    /// Current reply standing.
    pub reply_standing: ReplyStanding,
    /// Concerns raised in connection with this conversation (retired only on `NotAPatient` move).
    pub associated_concern_ids: BTreeSet<ConcernId>,
    /// Database-clock Unix second when created.
    pub created_at: i64,
    /// Database-clock Unix second when last updated.
    pub updated_at: i64,
}

/// Authorship kind of a physician's private draft on a reply (`Mine` vs `Revised`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PhysicianDraftKind {
    /// Physician's own edited text (`Your text`).
    YourText,
    /// Coda's published revision of the physician's draft (`Coda's revision`), holding `YourText` for restore.
    CodaRevision {
        /// Exact physician text prior to Coda's revision, restored by `Restore my version`.
        pre_revision_your_text: String,
    },
}

/// Standing of one physician's private draft on a specific `reply_id`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum PhysicianDraftStanding {
    /// Physician has no active draft on this reply and sees Coda's newest proposal.
    NoDraft,
    /// Physician has an active private draft on `reply_id`.
    Active {
        /// Reply identity this draft belongs to.
        reply_id: u64,
        /// Patient workspace this draft was written in.
        workspace: String,
        /// Underlying [`Scope::chat`] key in `physician_runtime_reply_drafts`.
        draft_scope_chat: String,
        /// Whether this draft currently holds physician edits or Coda's revision.
        kind: PhysicianDraftKind,
        /// Current revision in `physician_runtime_reply_drafts`.
        draft_revision: u64,
        /// Proposal version the draft was originally started from.
        based_on_proposal_version: u64,
        /// Newer proposal version offered by Coda while preserving this draft (`offers`), when any.
        update_offered_proposal_version: Option<u64>,
    },
}

/// Exact text shown on a physician's reply card (`Shown(d)` in `MessageReply.tla`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum DisplayedReplyText {
    /// Reply is withheld after a patient move; no text is shown until the new patient's review writes one.
    Withheld,
    /// Physician has no draft on `reply_id` and sees Coda's newest proposal version.
    CodaProposal {
        /// Reply identity.
        reply_id: u64,
        /// Proposal version shown.
        version: u64,
        /// Proposed reply text.
        text: String,
    },
    /// Physician sees their own private draft (`YourText` or `CodaRevision`) on `reply_id`.
    PhysicianDraft {
        /// Reply identity.
        reply_id: u64,
        /// Draft revision in `physician_runtime_reply_drafts`.
        draft_revision: u64,
        /// Physician text vs Coda revision kind.
        kind: PhysicianDraftKind,
        /// Private draft text shown to this physician alone.
        text: String,
    },
}

/// Provenance recorded when a physician copies the displayed reply text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum CopiedReplySource {
    /// Copied Coda's proposal directly.
    CodaProposal {
        /// Reply identity.
        reply_id: u64,
        /// Proposal version copied.
        version: u64,
    },
    /// Copied the physician's private draft.
    PhysicianDraft {
        /// Reply identity.
        reply_id: u64,
        /// Draft revision copied.
        draft_revision: u64,
        /// Whether the draft was `YourText` or `CodaRevision`.
        kind: PhysicianDraftKind,
    },
}

/// Durable record of a physician `Copy reply` action (`physician_runtime_reply_copies`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyCopyRecord {
    /// Unique copy record identifier.
    pub copy_id: String,
    /// Idempotency command key.
    pub command_key: String,
    /// Practice tenant identifier.
    pub practice: String,
    /// Conversation identifier.
    pub conversation_id: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Physician who copied the reply.
    pub physician: String,
    /// Reply identity copied.
    pub reply_id: u64,
    /// Practice message observation sequence at copy time.
    pub practice_observation_seq: u64,
    /// Exact version/source metadata of the copied text.
    pub copied_source: CopiedReplySource,
    /// Exact text placed on the clipboard (`Rule 5: CopiesWhatYouSee`).
    pub copied_text: String,
    /// Database-clock Unix second when copied.
    pub copied_at: i64,
}

/// Copy attribution shown to other authorized physicians until the practice sends a channel message.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyAttributionView {
    /// Physician who copied the reply.
    pub physician: String,
    /// Reply identity copied.
    pub reply_id: u64,
    /// Source version metadata copied.
    pub copied_source: CopiedReplySource,
    /// Database-clock Unix second when copied.
    pub copied_at: i64,
}

/// Whether Coda is currently reviewing messages for this conversation (`Reviewing` in `MessageReply.tla`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardReviewState {
    /// No review is waiting or running.
    Idle,
    /// A review is queued or running (`Coda is reviewing`).
    Reviewing,
}

/// Channel banner projected on an open reply card (`Card(d).note` in `MessageReply.tla`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum CardChannelBanner {
    /// Channel is connected; no disconnect banner is shown and `Done` is unavailable.
    Connected,
    /// Channel is disconnected; banner note is shown and `Done` is permitted.
    Disconnected {
        /// Database-clock Unix second when disconnected.
        disconnected_at: i64,
        /// Disconnect note shown on the card.
        note: String,
    },
}

/// One-line arrival/replacement notice shown on the reply card (`README.md` line 73).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CardArrivalNotice {
    /// Coda's newer reply replaced an earlier proposal version on the card.
    NewerReplyReplacedPrevious {
        /// Replaced proposal version.
        previous_version: u64,
        /// Current proposal version.
        current_version: u64,
    },
    /// A new message arrived with `sent_at > proposal.proposed_at` while the reply was open.
    PatientWroteAgain,
    /// An older message (`sent_at <= proposal.proposed_at`) was read late; never says the patient wrote again.
    OlderMessageReadLate,
}

/// Newer Coda proposal offered to a physician who holds an active private draft (`Card(d).offer`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodaUpdateOfferView {
    /// Offered proposal version (`> based_on_proposal_version`).
    pub offered_version: u64,
    /// Proposed text if the physician chooses Coda's update.
    pub offered_text: String,
    /// Basis of the offered proposal.
    pub offered_basis: ReplyProposalBasis,
    /// Verified citations of the offered proposal.
    pub offered_citations: Vec<SummaryCitation>,
}

/// Projected reply card for one authorized physician (`Card(d)` in `MessageReply.tla`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyCardView {
    /// Conversation identifier.
    pub conversation_id: String,
    /// Linked patient workspace identifier.
    pub workspace: String,
    /// Current reply identity.
    pub reply_id: u64,
    /// Non-patient sender names among open messages (shown alongside the patient per `README.md` line 72).
    pub non_patient_sender_names: Vec<String>,
    /// Every currently open patient message on the conversation (`OpenMsgs`, `Rule 6`).
    pub open_messages: Vec<LinkedPatientMessage>,
    /// Observed practice messages on the conversation, shown on the card immediately (`README.md` line 74).
    pub practice_messages: Vec<ObservedPracticeMessage>,
    /// Exact reply text shown to this physician (`Withheld`, `CodaProposal`, or `PhysicianDraft`).
    pub displayed_text: DisplayedReplyText,
    /// Basis of the current `Ready` proposal, when not withheld.
    pub basis: Option<ReplyProposalBasis>,
    /// Verified citations of the current `Ready` proposal, when not withheld.
    pub citations: Vec<SummaryCitation>,
    /// Whether Coda is reviewing (`Coda is reviewing`).
    pub review_state: CardReviewState,
    /// Channel connection banner (`NoteWhileDisconnected`, `Rule 11`).
    pub channel_banner: CardChannelBanner,
    /// Optional Coda update offer when Coda published a newer reply while this physician had a draft.
    pub coda_update_offer: Option<CodaUpdateOfferView>,
    /// Physician's current draft standing on `reply_id`.
    pub draft_standing: PhysicianDraftStanding,
    /// Ordered one-line arrival/replacement notices for the card.
    pub notices: Vec<CardArrivalNotice>,
    /// Other physicians who copied this reply before a practice message was observed.
    pub other_physician_copies: Vec<CopyAttributionView>,
}

/// Conversation state visible to an authorized physician when no open reply card is shown,
/// including `Coda is reviewing` for open messages or `Undo` right after `No reply needed`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "surface", rename_all = "snake_case")]
pub enum PhysicianReplySurface {
    /// Open or withheld reply card (`Sees(d)` is true).
    OpenCard(Box<ReplyCardView>),
    /// Reply was just closed with `No reply needed`; `Undo` is available until `EndUndo`.
    ClosedUndoBanner {
        /// Conversation identifier.
        conversation_id: String,
        /// Linked patient workspace.
        workspace: String,
        /// Closed reply identity.
        reply_id: u64,
        /// Closed proposal version.
        proposal_version: u64,
        /// Message IDs that `Undo` will reopen.
        closed_message_ids: BTreeSet<String>,
        /// Physician who closed the reply.
        closed_by: String,
        /// Whether Coda is currently reviewing.
        review_state: CardReviewState,
    },
    /// No open reply card; surfaces any open messages while Coda is reviewing (`Rule 9: ShowsUntilHandled`).
    NoOpenCard {
        /// Conversation identifier.
        conversation_id: String,
        /// Linked patient workspace.
        workspace: String,
        /// Open patient messages currently awaiting review.
        open_messages: Vec<LinkedPatientMessage>,
        /// Observed practice messages on the conversation (`README.md` line 74).
        practice_messages: Vec<ObservedPracticeMessage>,
        /// Non-patient sender names on open messages.
        non_patient_sender_names: Vec<String>,
        /// Whether Coda is currently reviewing.
        review_state: CardReviewState,
    },
}

/// Input to record a newly linked patient message on a conversation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordPatientMessage {
    /// Practice tenant identifier.
    pub practice: String,
    /// Conversation identifier.
    pub conversation_id: String,
    /// Channel message identifier.
    pub message_id: String,
    /// Patient vs named non-patient sender.
    pub sender: MessageSender,
    /// Exact message body text.
    pub text: String,
    /// Channel send timestamp (Unix seconds).
    pub sent_at: i64,
}

/// Outcome of recording a linked patient message or observing a practice message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessageIntakeOutcome {
    /// Event was newly recorded and queued at most one review wake.
    Recorded {
        /// Updated conversation snapshot.
        conversation: Box<ReplyConversation>,
    },
    /// Same message identifier was already recorded; idempotent no-op (`Rule 7: OneReviewEach`).
    Duplicate {
        /// Unchanged conversation snapshot.
        conversation: Box<ReplyConversation>,
    },
}

/// Input to observe a practice message sent in the channel.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservePracticeMessage {
    /// Practice tenant identifier.
    pub practice: String,
    /// Conversation identifier.
    pub conversation_id: String,
    /// Channel practice message identifier.
    pub practice_message_id: String,
    /// Human practice reply vs automatic message.
    pub kind: PracticeMessageKind,
    /// Exact channel words sent by the practice.
    pub text: String,
    /// Channel send timestamp (Unix seconds).
    pub sent_at: i64,
}

/// Snapshot returned to a background review when starting or re-reading a conversation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplyReviewSnapshot {
    /// Active review metadata with `ReviewReadState::Current`.
    pub review: ActiveReplyReview,
    /// Full conversation snapshot (all linked patient messages and observed practice messages).
    pub conversation: ReplyConversation,
    /// Workspace source observation version pinned during this read.
    pub source_version: u64,
}

/// Subset of open patient messages closed as answered by one cited practice message (`AC3`, `Rule 2`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnsweredMessageClosure {
    /// Non-automatic practice message identifier read by the review.
    pub practice_message_id: String,
    /// Exact subset of patient message IDs that `practice_message_id` answered.
    pub answered_message_ids: BTreeSet<String>,
}

/// Basis input supplied when publishing a proposed reply.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyProposalBasisInput {
    /// Summary of dated records, freshness, and thread context relied on.
    pub summary: String,
    /// Explicit missing, stale, or unavailable evidence limits.
    pub limits: Vec<String>,
    /// Exact chart record revisions referenced by the basis summary.
    pub cited_records: Vec<ExactRecordReference>,
}

/// Proposed reply payload supplied when a review concludes that a reply helps.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyProposalInput {
    /// Proposed patient-facing reply text.
    pub text: String,
    /// Basis and limitations metadata.
    pub basis: ReplyProposalBasisInput,
    /// Exact chart record revisions cited by the reply claims.
    pub cited_records: Vec<ExactRecordReference>,
}

/// Review decision for the remaining open messages `R = OpenMsgs \ A`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "decision", rename_all = "snake_case")]
pub enum ReplyReviewDecision {
    /// Proposes a reply covering all remaining open messages `R`.
    ProposeReply(ReplyProposalInput),
    /// Concludes no reply would help and marks remaining open messages `R` as `NeedsNoReply`.
    NoReplyWouldHelp {
        /// Saved clinical/operational reason why no reply is needed.
        reason: String,
    },
    /// Concludes with no open messages remaining (`R` is empty after `answered_closures` or prior handling).
    AllHandled,
}

/// Fenced completion input for an active conversation reply review.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompleteReplyReview {
    /// Practice tenant identifier.
    pub practice: String,
    /// Conversation identifier.
    pub conversation_id: String,
    /// Active review identifier being completed.
    pub review_id: String,
    /// Run or attempt identifier executing the completion.
    pub run_id: String,
    /// Once-only completion key (prevents duplicate proposals on retry/replay).
    pub completion_key: String,
    /// Practice-message closures (`A` in `MessageReply.tla`).
    pub answered_closures: Vec<AnsweredMessageClosure>,
    /// Decision for remaining open messages (`R = OpenMsgs \ A`).
    pub decision: ReplyReviewDecision,
    /// Optional concern IDs raised on the patient workspace during this review (`flag` in `MessageReply.tla`).
    pub raised_concern_ids: Vec<ConcernId>,
}

/// Durable completion summary saved in `physician_runtime_reply_review_completions`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedReviewCompletion {
    /// Completed review identifier.
    pub review_id: String,
    /// Run identifier.
    pub run_id: String,
    /// Completion key.
    pub completion_key: String,
    /// Workspace reviewed.
    pub workspace: String,
    /// Patient message IDs transitioned `Open -> Answered` by this review.
    pub newly_answered_message_ids: BTreeSet<String>,
    /// Patient message IDs transitioned `Open -> NeedsNoReply` by this review.
    pub newly_needs_no_reply_message_ids: BTreeSet<String>,
    /// Saved proposal when `decision` was `ProposeReply`.
    pub proposal: Option<SavedReplyProposal>,
    /// Physician IDs whose active drafts received a Coda update offer (`offers`).
    pub offered_update_physicians: BTreeSet<String>,
    /// Database-clock Unix second when completed.
    pub completed_at: i64,
}

/// Result of [`Store::complete_reply_review`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReplyReviewOutcome {
    /// Review completed and updated the conversation.
    Applied(Box<SavedReviewCompletion>),
    /// Exact `completion_key` was already recorded; returns the original completion without duplicating proposals.
    Duplicate(Box<SavedReviewCompletion>),
}

/// Outcome of [`Store::act_on_conversation_move`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConversationMoveOutcome {
    /// Conversation moved to another patient workspace (`Withheld` if open, review woken).
    MovedToPatient {
        /// Updated conversation snapshot.
        conversation: Box<ReplyConversation>,
    },
    /// Conversation moved to not-a-patient (open messages closed, reply closed, associated concerns retired).
    MovedToNotAPatient {
        /// Updated conversation snapshot.
        conversation: Box<ReplyConversation>,
        /// Open concerns associated with this conversation that were retired on the move.
        retired_concern_ids: Vec<ConcernId>,
    },
    /// Exact move command key was already applied.
    Duplicate {
        /// Current conversation snapshot.
        conversation: Box<ReplyConversation>,
    },
}

/// Outcome of [`Store::ask_coda_to_reply`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AskCodaToReplyOutcome {
    /// Queued a new review wake to draft a reply.
    QueuedReview {
        /// Request identifier.
        request_id: String,
        /// Updated conversation snapshot.
        conversation: Box<ReplyConversation>,
    },
    /// Recorded the request behind an already waiting or running review without spawning a second review.
    WaitingBehindActiveReview {
        /// Request identifier.
        request_id: String,
        /// Updated conversation snapshot.
        conversation: Box<ReplyConversation>,
    },
    /// A `Ready` reply already exists on the conversation.
    AlreadyReady {
        /// Existing ready proposal.
        proposal: Box<SavedReplyProposal>,
    },
    /// Exact command key was already recorded.
    Duplicate {
        /// Current conversation snapshot.
        conversation: Box<ReplyConversation>,
    },
}

/// Expected version fence when a physician saves edits or copies text on a reply card.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "base", content = "version", rename_all = "snake_case")]
pub enum ExpectedDraftBase {
    /// First edit from Coda's proposal (or after `ChooseUpdate`), fenced on `proposal.version`.
    ProposalVersion(u64),
    /// Subsequent edit on an existing physician draft, fenced on `draft.revision`.
    DraftRevision(u64),
}

/// Expected card state and version fence when marking a disconnected reply `Done` (`Rule 11`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum ExpectedDisconnectedCard {
    /// Card displayed a `Ready` reply at `(reply_id, proposal_version)`.
    Ready {
        /// Expected reply identity.
        reply_id: u64,
        /// Expected proposal version.
        proposal_version: u64,
    },
    /// Card displayed a `Withheld` reply at `reply_id`.
    Withheld {
        /// Expected withheld reply identity.
        reply_id: u64,
    },
}

/// Outcome of [`Store::request_reply_draft_revision`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProposalRevisionAccepted {
    /// Revision accepted on the physician's draft at `scope`.
    New {
        /// Draft scope (`Scope.chat` = `"reply-proposal:{conversation_id}:{reply_id}"`).
        scope: Scope,
    },
    /// Exact revision key and instruction already accepted; preserves physician edits untouched.
    Duplicate {
        /// Draft scope.
        scope: Scope,
    },
}

/// Outcome of [`Store::copy_reply`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CopyReplyOutcome {
    /// Recorded a new copy event for the exact text displayed on the physician's card.
    Recorded(ReplyCopyRecord),
    /// Exact copy command key was already recorded.
    Duplicate(ReplyCopyRecord),
}

impl fmt::Display for DisplayedReplyText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Withheld => f.write_str(""),
            Self::CodaProposal { text, .. } | Self::PhysicianDraft { text, .. } => {
                f.write_str(text)
            }
        }
    }
}

/// Builds the canonical [`Scope`] owning a physician's private draft for `(conversation_id, reply_id)`.
pub fn proposal_draft_scope(
    practice: &str,
    physician: &str,
    workspace: &str,
    conversation_id: &str,
    reply_id: u64,
) -> Scope {
    Scope {
        practice: practice.to_owned(),
        physician: physician.to_owned(),
        workspace: workspace.to_owned(),
        chat: format!("{PROPOSAL_DRAFT_CHAT_PREFIX}{conversation_id}:{reply_id}"),
    }
}
