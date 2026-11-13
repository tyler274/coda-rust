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

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use harness::engine;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, DbErr, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait, Unchanged,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use super::super::concerns::{
    CommandKey, ConcernError, ConcernId, ConcernInputError, ConcernMutation, ExactRecordReference,
    RetireConcern, RetirementReason, SourceWork, WorkspaceAccess, load_concern,
    retire_concern_in_tx,
};
use super::super::entities::{reply_draft, source_pull};
use super::super::receipt::{self, Receipt};
use super::super::sources::SourcePull;
use super::super::understanding::{SummaryCitation, build_summary_citation};
use super::super::{
    Authority, RuntimeError, Scope, Store, control, conversion_error, error, keys, now,
};
use super::{Draft, RevisionInput, draft_model};
use crate::physician_sources::SourcePatient;

/// Prefix identifying a [`Scope::chat`] owned by a reply proposal draft.
pub const PROPOSAL_DRAFT_CHAT_PREFIX: &str = "reply-proposal:";

const MAX_IDENTIFIER_CHARS: usize = 256;
const MAX_MESSAGE_TEXT_CHARS: usize = 32_768;
const MAX_REASON_CHARS: usize = 4_000;
const MAX_PROPOSAL_CITATIONS: usize = 64;
const JOB_RECEIPT_ACTOR: &str = "__practice_job__";
const DEFAULT_DISCONNECT_NOTE: &str =
    "Messaging channel is disconnected; replies cannot be observed until reconnected.";

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

use tables::{
    reply_conversation, reply_copy, reply_draft_binding, reply_proposal, reply_review_completion,
};

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
    /// Conversation content is withheld on `workspace` due to a wrong-patient link correction.
    #[error("{reason}")]
    ConversationWithheld {
        /// Conversation identifier.
        conversation_id: String,
        /// Patient workspace where content is withheld.
        workspace: String,
        /// Unavailable reason explaining the link correction status.
        reason: String,
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
    const fn db_label(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Ready { .. } => "ready",
            Self::Withheld { .. } => "withheld",
            Self::ClosedWithUndo { .. } => "closed_with_undo",
        }
    }

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

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StoredConversationState {
    channel: ChannelConnection,
    patient_messages: Vec<LinkedPatientMessage>,
    practice_messages: Vec<ObservedPracticeMessage>,
    recorded_practice_history: Vec<RecordedPracticeHistoryEntry>,
    woke_events: BTreeSet<ReviewWakeEvent>,
    active_review: Option<ActiveReplyReview>,
    pending_reply_request: Option<PendingPhysicianReplyRequest>,
    reply_standing: StoredReplyStanding,
    associated_concern_ids: BTreeSet<ConcernId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
enum StoredReplyStanding {
    None,
    Ready {
        proposal_version: u64,
    },
    Withheld {
        reply_id: u64,
        workspace: String,
        covered_message_ids: BTreeSet<String>,
        withheld_from_proposal_version: u64,
        move_epoch: u64,
        withheld_at: i64,
    },
    ClosedWithUndo {
        proposal_version: u64,
        closed_message_ids: BTreeSet<String>,
        closed_by: String,
        closed_at: i64,
    },
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

pub(super) enum ProposalDraftScope<'a> {
    NotProposal,
    Valid {
        conversation_id: &'a str,
        reply_id: u64,
    },
    Malformed,
}

pub(super) fn classify_proposal_draft_chat(chat: &str) -> ProposalDraftScope<'_> {
    if !chat.starts_with(PROPOSAL_DRAFT_CHAT_PREFIX) {
        return ProposalDraftScope::NotProposal;
    }
    match parse_proposal_draft_chat(chat) {
        Some((conversation_id, reply_id)) => ProposalDraftScope::Valid {
            conversation_id,
            reply_id,
        },
        None => ProposalDraftScope::Malformed,
    }
}

pub(super) fn parse_proposal_draft_chat(chat: &str) -> Option<(&str, u64)> {
    let rest = chat.strip_prefix(PROPOSAL_DRAFT_CHAT_PREFIX)?;
    let (conversation_id, reply_id_raw) = rest.rsplit_once(':')?;
    if conversation_id.is_empty()
        || conversation_id.chars().count() > MAX_IDENTIFIER_CHARS
        || conversation_id.trim() != conversation_id
        || conversation_id.chars().any(char::is_control)
    {
        return None;
    }
    let reply_id = reply_id_raw.parse::<u64>().ok().filter(|id| *id >= 1)?;
    Some((conversation_id, reply_id))
}

pub(super) fn validate_proposal_draft_text(raw: &str) -> Result<String, RuntimeError> {
    validate_text("draft.text", raw, MAX_MESSAGE_TEXT_CHARS)
        .map_err(|_| error("invalid proposal draft text").into())
}

fn validate_identifier(field: &'static str, raw: &str) -> Result<String, ReplyProposalInputError> {
    let count = raw.chars().count();
    if !(1..=MAX_IDENTIFIER_CHARS).contains(&count)
        || raw.trim() != raw
        || raw.chars().any(char::is_control)
    {
        return Err(ReplyProposalInputError::InvalidIdentifier {
            field,
            value: raw.to_owned(),
        });
    }
    Ok(raw.to_owned())
}

fn validate_text(
    field: &'static str,
    raw: &str,
    max_chars: usize,
) -> Result<String, ReplyProposalInputError> {
    let trimmed = raw.trim();
    let count = trimmed.chars().count();
    if !(1..=max_chars).contains(&count)
        || trimmed
            .chars()
            .any(|ch| ch.is_control() && !matches!(ch, '\n' | '\r' | '\t'))
    {
        return Err(ReplyProposalInputError::InvalidText { field });
    }
    Ok(trimmed.to_owned())
}

/// Validates channel-delivered words without trimming leading/trailing whitespace
/// (`Rule 8: HistoryTrue`, `README.md` line 72).
fn validate_channel_message_text(
    field: &'static str,
    raw: &str,
    max_chars: usize,
) -> Result<String, ReplyProposalInputError> {
    let count = raw.chars().count();
    if raw.trim().is_empty()
        || count > max_chars
        || raw
            .chars()
            .any(|ch| ch.is_control() && !matches!(ch, '\n' | '\r' | '\t'))
    {
        return Err(ReplyProposalInputError::InvalidText { field });
    }
    Ok(raw.to_owned())
}

fn validate_single_line(field: &'static str, raw: &str) -> Result<String, ReplyProposalInputError> {
    if raw.contains('\n') || raw.contains('\r') {
        return Err(ReplyProposalInputError::InvalidText { field });
    }
    let trimmed = raw.trim();
    let count = trimmed.chars().count();
    if !(1..=MAX_REASON_CHARS).contains(&count) || trimmed.chars().any(char::is_control) {
        return Err(ReplyProposalInputError::InvalidText { field });
    }
    Ok(trimmed.to_owned())
}

fn validate_link(link: &ConversationLink) -> Result<ConversationLink, ReplyProposalInputError> {
    match link {
        ConversationLink::LinkedPatient { workspace } => Ok(ConversationLink::LinkedPatient {
            workspace: validate_identifier("workspace", workspace)?,
        }),
        ConversationLink::NotAPatient => Ok(ConversationLink::NotAPatient),
    }
}

fn validate_sender(sender: &MessageSender) -> Result<MessageSender, ReplyProposalInputError> {
    match sender {
        MessageSender::Patient => Ok(MessageSender::Patient),
        MessageSender::NamedSender { name } => Ok(MessageSender::NamedSender {
            name: validate_single_line("sender.name", name)?,
        }),
    }
}

fn validate_channel(
    channel: &ChannelConnection,
) -> Result<ChannelConnection, ReplyProposalInputError> {
    match channel {
        ChannelConnection::Connected => Ok(ChannelConnection::Connected),
        ChannelConnection::Disconnected {
            disconnected_at,
            note,
        } => {
            let clean_note = if note.trim().is_empty() {
                DEFAULT_DISCONNECT_NOTE.to_owned()
            } else {
                validate_single_line("channel.note", note)?
            };
            Ok(ChannelConnection::Disconnected {
                disconnected_at: *disconnected_at,
                note: clean_note,
            })
        }
    }
}

fn normalize_record_references(
    field: &'static str,
    references: &[ExactRecordReference],
) -> Result<Vec<ExactRecordReference>, ReplyProposalInputError> {
    if references.len() > MAX_PROPOSAL_CITATIONS {
        return Err(ReplyProposalInputError::TooManyCitations {
            count: references.len(),
        });
    }
    let mut seen: BTreeMap<String, String> = BTreeMap::new();
    let mut normalized = Vec::with_capacity(references.len());
    for reference in references {
        let checked = ExactRecordReference::new(&reference.candidate_id, &reference.revision)
            .map_err(|cause| ReplyProposalInputError::InvalidRecordReference { field, cause })?;
        if let Some(existing) = seen.get(&checked.candidate_id) {
            if existing != &checked.revision {
                return Err(ReplyProposalInputError::ConflictingRecordRevision {
                    candidate_id: checked.candidate_id,
                });
            }
            continue;
        }
        seen.insert(checked.candidate_id.clone(), checked.revision.clone());
        normalized.push(checked);
    }
    Ok(normalized)
}

fn non_negative_u64(value: i64, what: &'static str) -> Result<u64, DbErr> {
    u64::try_from(value).map_err(conversion_error(what, "u64"))
}

fn positive_u64(value: i64, what: &'static str) -> Result<u64, DbErr> {
    let parsed = non_negative_u64(value, what)?;
    if parsed == 0 {
        return Err(error(what));
    }
    Ok(parsed)
}

fn to_i64(value: u64, what: &'static str) -> Result<i64, DbErr> {
    i64::try_from(value).map_err(conversion_error(what, "i64"))
}

/// Single rule owner settling `ClosedWithUndo -> None` (`EndUndo` in `MessageReply.tla`).
fn end_closed_undo(standing: &mut ReplyStanding) {
    if matches!(standing, ReplyStanding::ClosedWithUndo { .. }) {
        *standing = ReplyStanding::None;
    }
}

/// Single rule owner reopening the latest `NeedsNoReply` patient message when a physician
/// explicitly asks Coda to draft a reply and no patient message is currently `Open`,
/// while keeping `Answered`, `MarkedDoneWhileDisconnected`, and `MovedToNotPatient` intact.
fn reopen_latest_needs_no_reply_for_request(conv: &mut ReplyConversation) -> Option<String> {
    if !conv.patient_messages.iter().any(|m| m.standing.is_open())
        && let Some(last_msg) = conv
            .patient_messages
            .iter_mut()
            .rev()
            .find(|m| matches!(m.standing, PatientMessageStanding::NeedsNoReply { .. }))
    {
        last_msg.standing = PatientMessageStanding::Open;
        return Some(last_msg.message_id.clone());
    }
    None
}

/// Single rule owner deciding whether Coda is reviewing (`Reviewing` in `MessageReply.tla`).
fn conversation_review_state(conv: &ReplyConversation) -> CardReviewState {
    if !conv.woke_events.is_empty() || conv.active_review.is_some() {
        CardReviewState::Reviewing
    } else {
        CardReviewState::Idle
    }
}

fn open_patient_messages(conv: &ReplyConversation) -> Vec<LinkedPatientMessage> {
    conv.patient_messages
        .iter()
        .filter(|m| m.standing.is_open())
        .cloned()
        .collect()
}

fn non_patient_senders(messages: &[LinkedPatientMessage]) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut ordered = Vec::new();
    for message in messages {
        if let MessageSender::NamedSender { name } = &message.sender
            && seen.insert(name.clone())
        {
            ordered.push(name.clone());
        }
    }
    ordered
}

fn decode_proposal(row: reply_proposal::Model) -> Result<SavedReplyProposal, DbErr> {
    let version = positive_u64(row.version, "corrupt proposal version")?;
    let reply_id = positive_u64(row.reply_id, "corrupt proposal reply_id")?;
    let conversation_version = positive_u64(
        row.conversation_version,
        "corrupt proposal conversation_version",
    )?;
    let source_version = non_negative_u64(row.source_version, "corrupt proposal source_version")?;
    let move_epoch = non_negative_u64(row.move_epoch, "corrupt proposal move_epoch")?;
    let replaced_proposal_version = row
        .replaced_proposal_version
        .map(|v| positive_u64(v, "corrupt replaced_proposal_version"))
        .transpose()?;
    let covered_message_ids: BTreeSet<String> = serde_json::from_value(row.covered_message_ids)
        .map_err(conversion_error(
            "proposal covered_message_ids",
            "BTreeSet<String>",
        ))?;
    let basis: ReplyProposalBasis = serde_json::from_value(row.basis)
        .map_err(conversion_error("proposal basis", "ReplyProposalBasis"))?;
    let citations: Vec<SummaryCitation> = serde_json::from_value(row.citations).map_err(
        conversion_error("proposal citations", "Vec<SummaryCitation>"),
    )?;
    Ok(SavedReplyProposal {
        practice: row.practice,
        conversation_id: row.conversation_id,
        reply_id,
        version,
        workspace: row.workspace,
        covered_message_ids,
        text: row.text,
        basis,
        citations,
        review_id: row.review_id,
        run_id: row.run_id,
        completion_key: row.completion_key,
        conversation_version,
        source_version,
        move_epoch,
        replaced_proposal_version,
        proposed_at: row.proposed_at,
    })
}

async fn load_proposal_by_version(
    tx: &impl ConnectionTrait,
    practice: &str,
    conversation_id: &str,
    version: u64,
) -> Result<SavedReplyProposal, DbErr> {
    let version_i64 = to_i64(version, "proposal version")?;
    let row = reply_proposal::Entity::find_by_id((
        practice.to_owned(),
        conversation_id.to_owned(),
        version_i64,
    ))
    .one(tx)
    .await?
    .ok_or_else(|| error("missing saved reply proposal referenced by conversation"))?;
    decode_proposal(row)
}

async fn decode_conversation(
    tx: &impl ConnectionTrait,
    row: reply_conversation::Model,
) -> Result<ReplyConversation, DbErr> {
    let link = match (row.link_kind.as_str(), row.workspace) {
        ("linked_patient", Some(workspace)) if !workspace.is_empty() => {
            ConversationLink::LinkedPatient { workspace }
        }
        ("not_a_patient", None) => ConversationLink::NotAPatient,
        _ => return Err(error("corrupt reply conversation link")),
    };
    let conversation_version =
        positive_u64(row.conversation_version, "corrupt conversation_version")?;
    let move_epoch = non_negative_u64(row.move_epoch, "corrupt move_epoch")?;
    let last_reply_id = non_negative_u64(row.last_reply_id, "corrupt last_reply_id")?;
    let last_proposal_version =
        non_negative_u64(row.last_proposal_version, "corrupt last_proposal_version")?;
    let last_practice_observation_seq = non_negative_u64(
        row.last_practice_observation_seq,
        "corrupt last_practice_observation_seq",
    )?;
    let stored: StoredConversationState = serde_json::from_value(row.state).map_err(
        conversion_error("reply conversation state", "StoredConversationState"),
    )?;
    match (row.channel_state.as_str(), &stored.channel) {
        ("connected", ChannelConnection::Connected)
        | ("disconnected", ChannelConnection::Disconnected { .. }) => {}
        _ => return Err(error("corrupt reply conversation channel_state")),
    }
    let reply_standing = match (row.reply_state.as_str(), stored.reply_standing) {
        ("none", StoredReplyStanding::None) => ReplyStanding::None,
        ("ready", StoredReplyStanding::Ready { proposal_version }) => {
            let proposal =
                load_proposal_by_version(tx, &row.practice, &row.conversation_id, proposal_version)
                    .await?;
            ReplyStanding::Ready {
                proposal: Box::new(proposal),
            }
        }
        (
            "withheld",
            StoredReplyStanding::Withheld {
                reply_id,
                workspace,
                covered_message_ids,
                withheld_from_proposal_version,
                move_epoch,
                withheld_at,
            },
        ) => ReplyStanding::Withheld {
            reply_id,
            workspace,
            covered_message_ids,
            withheld_from_proposal_version,
            move_epoch,
            withheld_at,
        },
        (
            "closed_with_undo",
            StoredReplyStanding::ClosedWithUndo {
                proposal_version,
                closed_message_ids,
                closed_by,
                closed_at,
            },
        ) => {
            let proposal =
                load_proposal_by_version(tx, &row.practice, &row.conversation_id, proposal_version)
                    .await?;
            ReplyStanding::ClosedWithUndo {
                proposal: Box::new(proposal),
                closed_message_ids,
                closed_by,
                closed_at,
            }
        }
        _ => return Err(error("corrupt reply conversation reply_state")),
    };

    Ok(ReplyConversation {
        practice: row.practice,
        conversation_id: row.conversation_id,
        link,
        channel: stored.channel,
        conversation_version,
        move_epoch,
        last_reply_id,
        last_proposal_version,
        last_practice_observation_seq,
        patient_messages: stored.patient_messages,
        practice_messages: stored.practice_messages,
        recorded_practice_history: stored.recorded_practice_history,
        woke_events: stored.woke_events,
        active_review: stored.active_review,
        pending_reply_request: stored.pending_reply_request,
        reply_standing,
        associated_concern_ids: stored.associated_concern_ids,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

fn encode_conversation_model(
    conv: &ReplyConversation,
) -> Result<reply_conversation::ActiveModel, DbErr> {
    let (link_kind, workspace) = match &conv.link {
        ConversationLink::LinkedPatient { workspace } => {
            ("linked_patient".to_owned(), Some(workspace.clone()))
        }
        ConversationLink::NotAPatient => ("not_a_patient".to_owned(), None),
    };
    let channel_state = match &conv.channel {
        ChannelConnection::Connected => "connected",
        ChannelConnection::Disconnected { .. } => "disconnected",
    };
    let stored_standing = match &conv.reply_standing {
        ReplyStanding::None => StoredReplyStanding::None,
        ReplyStanding::Ready { proposal } => StoredReplyStanding::Ready {
            proposal_version: proposal.version,
        },
        ReplyStanding::Withheld {
            reply_id,
            workspace,
            covered_message_ids,
            withheld_from_proposal_version,
            move_epoch,
            withheld_at,
        } => StoredReplyStanding::Withheld {
            reply_id: *reply_id,
            workspace: workspace.clone(),
            covered_message_ids: covered_message_ids.clone(),
            withheld_from_proposal_version: *withheld_from_proposal_version,
            move_epoch: *move_epoch,
            withheld_at: *withheld_at,
        },
        ReplyStanding::ClosedWithUndo {
            proposal,
            closed_message_ids,
            closed_by,
            closed_at,
        } => StoredReplyStanding::ClosedWithUndo {
            proposal_version: proposal.version,
            closed_message_ids: closed_message_ids.clone(),
            closed_by: closed_by.clone(),
            closed_at: *closed_at,
        },
    };
    let state_json = serde_json::to_value(StoredConversationState {
        channel: conv.channel.clone(),
        patient_messages: conv.patient_messages.clone(),
        practice_messages: conv.practice_messages.clone(),
        recorded_practice_history: conv.recorded_practice_history.clone(),
        woke_events: conv.woke_events.clone(),
        active_review: conv.active_review.clone(),
        pending_reply_request: conv.pending_reply_request.clone(),
        reply_standing: stored_standing,
        associated_concern_ids: conv.associated_concern_ids.clone(),
    })
    .map_err(conversion_error("StoredConversationState", "json"))?;

    Ok(reply_conversation::ActiveModel {
        practice: Set(conv.practice.clone()),
        conversation_id: Set(conv.conversation_id.clone()),
        link_kind: Set(link_kind),
        workspace: Set(workspace),
        channel_state: Set(channel_state.to_owned()),
        reply_state: Set(conv.reply_standing.db_label().to_owned()),
        conversation_version: Set(to_i64(conv.conversation_version, "conversation_version")?),
        move_epoch: Set(to_i64(conv.move_epoch, "move_epoch")?),
        last_reply_id: Set(to_i64(conv.last_reply_id, "last_reply_id")?),
        last_proposal_version: Set(to_i64(conv.last_proposal_version, "last_proposal_version")?),
        last_practice_observation_seq: Set(to_i64(
            conv.last_practice_observation_seq,
            "last_practice_observation_seq",
        )?),
        state: Set(state_json),
        created_at: Set(conv.created_at),
        updated_at: Set(conv.updated_at),
    })
}

async fn load_locked_conversation(
    tx: &DatabaseTransaction,
    practice: &str,
    conversation_id: &str,
) -> Result<ReplyConversation, ReplyProposalError> {
    let Some(row) =
        reply_conversation::Entity::find_by_id((practice.to_owned(), conversation_id.to_owned()))
            .lock_exclusive()
            .one(tx)
            .await?
    else {
        return Err(ReplyProposalError::ConversationNotFound {
            conversation_id: conversation_id.to_owned(),
        });
    };
    Ok(decode_conversation(tx, row).await?)
}

fn decode_copy_record(row: reply_copy::Model) -> Result<ReplyCopyRecord, DbErr> {
    let reply_id = positive_u64(row.reply_id, "corrupt reply_copy.reply_id")?;
    let practice_observation_seq = non_negative_u64(
        row.practice_observation_seq,
        "corrupt reply_copy.practice_observation_seq",
    )?;
    let copied_source: CopiedReplySource = serde_json::from_value(row.copied_source).map_err(
        conversion_error("reply_copy.copied_source", "CopiedReplySource"),
    )?;
    Ok(ReplyCopyRecord {
        copy_id: row.copy_id,
        command_key: row.command_key,
        practice: row.practice,
        conversation_id: row.conversation_id,
        workspace: row.workspace,
        physician: row.physician,
        reply_id,
        practice_observation_seq,
        copied_source,
        copied_text: row.copied_text,
        copied_at: row.copied_at,
    })
}

fn decode_draft_binding_standing(
    row: &reply_draft_binding::Model,
) -> Result<PhysicianDraftStanding, DbErr> {
    let reply_id = positive_u64(row.reply_id, "corrupt binding reply_id")?;
    let draft_revision = positive_u64(row.draft_revision, "corrupt binding draft_revision")?;
    let based_on_proposal_version = positive_u64(
        row.based_on_proposal_version,
        "corrupt binding based_on_proposal_version",
    )?;
    let update_offered_proposal_version = row
        .update_offered_proposal_version
        .map(|v| positive_u64(v, "corrupt update_offered_proposal_version"))
        .transpose()?;
    let kind = match row.standing.as_str() {
        "your_text" => PhysicianDraftKind::YourText,
        "coda_revision" => {
            let pre = row
                .pre_revision_your_text
                .clone()
                .filter(|s| !s.is_empty())
                .ok_or_else(|| error("corrupt coda_revision binding missing restore text"))?;
            PhysicianDraftKind::CodaRevision {
                pre_revision_your_text: pre,
            }
        }
        "detached_chose_update" => return Ok(PhysicianDraftStanding::NoDraft),
        _ => return Err(error("corrupt reply_draft_binding.standing")),
    };
    Ok(PhysicianDraftStanding::Active {
        reply_id,
        workspace: row.workspace.clone(),
        draft_scope_chat: row.draft_scope_chat.clone(),
        kind,
        draft_revision,
        based_on_proposal_version,
        update_offered_proposal_version,
    })
}

async fn load_workspace_pulls_in_tx(
    tx: &impl ConnectionTrait,
    workspace: &str,
) -> Result<Vec<SourcePull>, DbErr> {
    let rows = source_pull::Entity::find()
        .filter(source_pull::Column::Workspace.eq(workspace))
        .order_by_asc(source_pull::Column::SourceId)
        .all(tx)
        .await?;
    let mut pulls = Vec::with_capacity(rows.len());
    for row in rows {
        let patient: SourcePatient = serde_json::from_value(row.patient)
            .map_err(conversion_error("source_pull.patient", "SourcePatient"))?;
        pulls.push(SourcePull {
            source_id: row.source_id,
            source_label: row.source_label,
            patient,
            pulled_at: row.pulled_at,
        });
    }
    Ok(pulls)
}

/// Single rule owner projecting `ReplyCardView` (`Card(d)` in `MessageReply.tla`).
async fn project_reply_card_in_tx(
    tx: &impl ConnectionTrait,
    conv: &ReplyConversation,
    viewer: &str,
) -> Result<Option<ReplyCardView>, DbErr> {
    let ConversationLink::LinkedPatient { workspace } = &conv.link else {
        return Ok(None);
    };
    let open_messages = open_patient_messages(conv);
    let non_patient_sender_names = non_patient_senders(&open_messages);
    let review_state = conversation_review_state(conv);
    let channel_banner = match &conv.channel {
        ChannelConnection::Connected => CardChannelBanner::Connected,
        ChannelConnection::Disconnected {
            disconnected_at,
            note,
        } => CardChannelBanner::Disconnected {
            disconnected_at: *disconnected_at,
            note: note.clone(),
        },
    };

    match &conv.reply_standing {
        ReplyStanding::None | ReplyStanding::ClosedWithUndo { .. } => Ok(None),
        ReplyStanding::Withheld {
            reply_id,
            workspace: withheld_workspace,
            ..
        } => {
            if withheld_workspace != workspace {
                return Err(error("corrupt withheld reply workspace mismatch"));
            }
            Ok(Some(ReplyCardView {
                conversation_id: conv.conversation_id.clone(),
                workspace: workspace.clone(),
                reply_id: *reply_id,
                non_patient_sender_names,
                open_messages,
                practice_messages: conv.practice_messages.clone(),
                displayed_text: DisplayedReplyText::Withheld,
                basis: None,
                citations: Vec::new(),
                review_state,
                channel_banner,
                coda_update_offer: None,
                draft_standing: PhysicianDraftStanding::NoDraft,
                notices: Vec::new(),
                other_physician_copies: Vec::new(),
            }))
        }
        ReplyStanding::Ready { proposal } => {
            let reply_id_i64 = to_i64(proposal.reply_id, "proposal.reply_id")?;
            let copy_rows = reply_copy::Entity::find()
                .filter(reply_copy::Column::Practice.eq(conv.practice.as_str()))
                .filter(reply_copy::Column::ConversationId.eq(conv.conversation_id.as_str()))
                .filter(reply_copy::Column::Workspace.eq(workspace.as_str()))
                .filter(reply_copy::Column::ReplyId.eq(reply_id_i64))
                .filter(reply_copy::Column::PracticeObservationSeq.eq(to_i64(
                    conv.last_practice_observation_seq,
                    "last_practice_observation_seq",
                )?))
                .order_by_asc(reply_copy::Column::CopiedAt)
                .order_by_asc(reply_copy::Column::CopyId)
                .all(tx)
                .await?;
            let mut other_physician_copies = Vec::new();
            for row in copy_rows {
                let record = decode_copy_record(row)?;
                if record.physician != viewer {
                    other_physician_copies.push(CopyAttributionView {
                        physician: record.physician,
                        reply_id: record.reply_id,
                        copied_source: record.copied_source,
                        copied_at: record.copied_at,
                    });
                }
            }
            let binding_row = reply_draft_binding::Entity::find_by_id((
                conv.practice.clone(),
                conv.conversation_id.clone(),
                viewer.to_owned(),
                reply_id_i64,
            ))
            .one(tx)
            .await?;

            let draft_standing = match binding_row {
                Some(row) => decode_draft_binding_standing(&row)?,
                None => PhysicianDraftStanding::NoDraft,
            };

            let (displayed_text, coda_update_offer) = match &draft_standing {
                PhysicianDraftStanding::NoDraft => (
                    DisplayedReplyText::CodaProposal {
                        reply_id: proposal.reply_id,
                        version: proposal.version,
                        text: proposal.text.clone(),
                    },
                    None,
                ),
                PhysicianDraftStanding::Active {
                    reply_id,
                    workspace: draft_workspace,
                    draft_scope_chat,
                    kind,
                    draft_revision,
                    update_offered_proposal_version,
                    ..
                } => {
                    let scope = Scope {
                        practice: conv.practice.clone(),
                        physician: viewer.to_owned(),
                        workspace: draft_workspace.clone(),
                        chat: draft_scope_chat.clone(),
                    };
                    let draft_row = reply_draft::Entity::find_by_id(keys(&scope))
                        .one(tx)
                        .await?
                        .ok_or_else(|| error("missing reply_draft row for active binding"))?;
                    let draft = control::draft(draft_row, vec![])?;
                    let offer = match update_offered_proposal_version {
                        Some(offered_v) => {
                            let offered = load_proposal_by_version(
                                tx,
                                &conv.practice,
                                &conv.conversation_id,
                                *offered_v,
                            )
                            .await?;
                            Some(CodaUpdateOfferView {
                                offered_version: offered.version,
                                offered_text: offered.text,
                                offered_basis: offered.basis,
                                offered_citations: offered.citations,
                            })
                        }
                        None => None,
                    };
                    (
                        DisplayedReplyText::PhysicianDraft {
                            reply_id: *reply_id,
                            draft_revision: *draft_revision,
                            kind: kind.clone(),
                            text: draft.text,
                        },
                        offer,
                    )
                }
            };

            let mut notices = Vec::new();
            if matches!(displayed_text, DisplayedReplyText::CodaProposal { .. })
                && let Some(previous_version) = proposal.replaced_proposal_version
            {
                notices.push(CardArrivalNotice::NewerReplyReplacedPrevious {
                    previous_version,
                    current_version: proposal.version,
                });
            }
            let mut saw_newer_message = false;
            let mut saw_late_older_message = false;
            for message in &open_messages {
                if !proposal.covered_message_ids.contains(&message.message_id) {
                    if message.sent_at > proposal.proposed_at {
                        saw_newer_message = true;
                    } else {
                        saw_late_older_message = true;
                    }
                }
            }
            if saw_newer_message {
                notices.push(CardArrivalNotice::PatientWroteAgain);
            }
            if saw_late_older_message {
                notices.push(CardArrivalNotice::OlderMessageReadLate);
            }

            Ok(Some(ReplyCardView {
                conversation_id: conv.conversation_id.clone(),
                workspace: workspace.clone(),
                reply_id: proposal.reply_id,
                non_patient_sender_names,
                open_messages,
                practice_messages: conv.practice_messages.clone(),
                displayed_text,
                basis: Some(proposal.basis.clone()),
                citations: proposal.citations.clone(),
                review_state,
                channel_banner,
                coda_update_offer,
                draft_standing,
                notices,
                other_physician_copies,
            }))
        }
    }
}

fn verify_physician_access_scope(
    access: &WorkspaceAccess<'_>,
) -> Result<Scope, ReplyProposalError> {
    access.authority_scope().map_err(|err| match err {
        ConcernError::AccessDenied { reason } => ReplyProposalError::AccessDenied { reason },
        ConcernError::InvalidInput(cause) => {
            ReplyProposalError::InvalidInput(ReplyProposalInputError::InvalidIdentifier {
                field: "access",
                value: cause.to_string(),
            })
        }
        other => ReplyProposalError::Concern(other),
    })
}

fn verify_conversation_workspace(
    access: &WorkspaceAccess<'_>,
    conv: &ReplyConversation,
) -> Result<(Scope, String), ReplyProposalError> {
    let scope = verify_physician_access_scope(access)?;
    let ConversationLink::LinkedPatient { workspace } = &conv.link else {
        return Err(ReplyProposalError::ConversationNotLinkedToPatient {
            conversation_id: conv.conversation_id.clone(),
        });
    };
    if workspace != &scope.workspace {
        return Err(ReplyProposalError::AccessDenied {
            reason: format!(
                "conversation `{}` is linked to workspace `{workspace}`, not `{}`",
                conv.conversation_id, scope.workspace
            ),
        });
    }
    Ok((scope, workspace.clone()))
}

async fn verify_conversation_workspace_in_tx(
    tx: &DatabaseTransaction,
    access: &WorkspaceAccess<'_>,
    conv: &ReplyConversation,
) -> Result<(Scope, String), ReplyProposalError> {
    let scope = verify_physician_access_scope(access)?;
    if let Some(reason) = super::super::link_correction::conversation_withheld_reason_in_tx(
        tx,
        &scope.practice,
        &scope.workspace,
        &conv.conversation_id,
    )
    .await?
    {
        return Err(ReplyProposalError::ConversationWithheld {
            conversation_id: conv.conversation_id.clone(),
            workspace: scope.workspace,
            reason,
        });
    }
    verify_conversation_workspace(access, conv)
}

async fn check_physician_read_gate_in_tx(
    tx: &DatabaseTransaction,
    scope: &Scope,
    session_hash: &[u8],
    authority: &mut dyn Authority,
) -> Result<(), ReplyProposalError> {
    authority
        .check(tx, scope, Some(session_hash))
        .await
        .map_err(|reason| ReplyProposalError::AccessDenied { reason })
}

async fn lock_proposal_access_in_tx(
    tx: &DatabaseTransaction,
    scope: &Scope,
    session_hash: &[u8],
    authority: &mut dyn Authority,
) -> Result<(), ReplyProposalError> {
    authority
        .lock_publication(tx, scope, Some(session_hash))
        .await
        .map_err(|reason| ReplyProposalError::AccessDenied { reason })
}

async fn load_locked_conversation_for_physician(
    tx: &DatabaseTransaction,
    scope: &Scope,
    session_hash: &[u8],
    conversation_id: &str,
    authority: &mut dyn Authority,
) -> Result<ReplyConversation, ReplyProposalError> {
    match load_locked_conversation(tx, &scope.practice, conversation_id).await {
        Ok(conv) => Ok(conv),
        Err(err @ ReplyProposalError::ConversationNotFound { .. }) => {
            lock_proposal_access_in_tx(tx, scope, session_hash, authority).await?;
            Err(err)
        }
        Err(other) => Err(other),
    }
}

async fn verify_conversation_workspace_for_write(
    tx: &DatabaseTransaction,
    access: &WorkspaceAccess<'_>,
    scope: &Scope,
    conv: &ReplyConversation,
    authority: &mut dyn Authority,
) -> Result<String, ReplyProposalError> {
    if let Some(reason) = super::super::link_correction::conversation_withheld_reason_in_tx(
        tx,
        &scope.practice,
        &scope.workspace,
        &conv.conversation_id,
    )
    .await?
    {
        lock_proposal_access_in_tx(tx, scope, access.session_hash, authority).await?;
        return Err(ReplyProposalError::ConversationWithheld {
            conversation_id: conv.conversation_id.clone(),
            workspace: scope.workspace.clone(),
            reason,
        });
    }
    let ConversationLink::LinkedPatient { workspace } = &conv.link else {
        lock_proposal_access_in_tx(tx, scope, access.session_hash, authority).await?;
        return Err(ReplyProposalError::ConversationNotLinkedToPatient {
            conversation_id: conv.conversation_id.clone(),
        });
    };
    if workspace != &scope.workspace {
        return Err(ReplyProposalError::AccessDenied {
            reason: format!(
                "conversation `{}` is linked to workspace `{workspace}`, not `{}`",
                conv.conversation_id, scope.workspace
            ),
        });
    }
    Ok(workspace.clone())
}

async fn check_conversation_access_in_tx(
    tx: &DatabaseTransaction,
    access: &WorkspaceAccess<'_>,
    scope: &Scope,
    conv: &ReplyConversation,
    authority: &mut dyn Authority,
) -> Result<String, ReplyProposalError> {
    let workspace =
        verify_conversation_workspace_for_write(tx, access, scope, conv, authority).await?;
    lock_proposal_access_in_tx(tx, scope, access.session_hash, authority).await?;
    Ok(workspace)
}

pub(super) async fn lock_proposal_draft_binding_in_tx(
    tx: &DatabaseTransaction,
    scope: &Scope,
    conversation_id: &str,
    reply_id: u64,
) -> Result<(), RuntimeError> {
    let reply_id_i64 = to_i64(reply_id, "reply_id")?;
    let _ = reply_draft_binding::Entity::find_by_id((
        scope.practice.clone(),
        conversation_id.to_owned(),
        scope.physician.clone(),
        reply_id_i64,
    ))
    .lock_exclusive()
    .one(tx)
    .await?
    .ok_or_else(|| error("missing reply_draft_binding for committed revision"))?;
    Ok(())
}

pub(super) async fn lock_active_proposal_draft_binding_in_tx(
    tx: &DatabaseTransaction,
    scope: &Scope,
    conversation_id: &str,
    reply_id: u64,
) -> Result<bool, RuntimeError> {
    let reply_id_i64 = to_i64(reply_id, "reply_id")?;
    let Some(binding) = reply_draft_binding::Entity::find_by_id((
        scope.practice.clone(),
        conversation_id.to_owned(),
        scope.physician.clone(),
        reply_id_i64,
    ))
    .lock_exclusive()
    .one(tx)
    .await?
    else {
        return Ok(false);
    };
    Ok(matches!(
        decode_draft_binding_standing(&binding)?,
        PhysicianDraftStanding::Active { .. }
    ))
}

pub(super) async fn lock_active_proposal_draft_binding_model_in_tx(
    tx: &DatabaseTransaction,
    scope: &Scope,
    conversation_id: &str,
    reply_id: u64,
) -> Result<reply_draft_binding::Model, RuntimeError> {
    let reply_id_i64 = to_i64(reply_id, "reply_id")?;
    let binding = reply_draft_binding::Entity::find_by_id((
        scope.practice.clone(),
        conversation_id.to_owned(),
        scope.physician.clone(),
        reply_id_i64,
    ))
    .lock_exclusive()
    .one(tx)
    .await?
    .ok_or_else(|| error("missing active reply_draft_binding"))?;
    if !matches!(
        decode_draft_binding_standing(&binding)?,
        PhysicianDraftStanding::Active { .. }
    ) {
        return Err(error("proposal draft was detached; edit through reply proposal").into());
    }
    Ok(binding)
}

pub(super) async fn sync_saved_proposal_draft_binding_in_tx(
    tx: &DatabaseTransaction,
    row: reply_draft_binding::Model,
    new_draft_revision: u64,
    timestamp: i64,
) -> Result<(), RuntimeError> {
    reply_draft_binding::ActiveModel {
        practice: Unchanged(row.practice),
        conversation_id: Unchanged(row.conversation_id),
        physician: Unchanged(row.physician),
        reply_id: Unchanged(row.reply_id),
        workspace: Unchanged(row.workspace),
        draft_scope_chat: Unchanged(row.draft_scope_chat),
        standing: Set("your_text".to_owned()),
        draft_revision: Set(to_i64(new_draft_revision, "draft_revision")?),
        based_on_proposal_version: Unchanged(row.based_on_proposal_version),
        pre_revision_your_text: Set(None),
        update_offered_proposal_version: Unchanged(row.update_offered_proposal_version),
        updated_at: Set(timestamp),
    }
    .update(tx)
    .await?;
    Ok(())
}

pub(super) async fn check_proposal_draft_commit_fence_in_tx(
    tx: &DatabaseTransaction,
    scope: &Scope,
    conversation_id: &str,
    reply_id: u64,
) -> Result<(), RuntimeError> {
    let Some(row) = reply_conversation::Entity::find_by_id((
        scope.practice.clone(),
        conversation_id.to_owned(),
    ))
    .lock_exclusive()
    .one(tx)
    .await?
    else {
        return Err(error("reply conversation not found for draft revision").into());
    };
    let conv = decode_conversation(tx, row).await?;
    let ConversationLink::LinkedPatient { workspace } = &conv.link else {
        return Err(error("stale revision completion after conversation move").into());
    };
    if workspace != &scope.workspace {
        return Err(error("stale revision completion after conversation move").into());
    }
    match &conv.reply_standing {
        ReplyStanding::Ready { proposal } if proposal.reply_id == reply_id => Ok(()),
        ReplyStanding::Ready { .. }
        | ReplyStanding::Withheld { .. }
        | ReplyStanding::ClosedWithUndo { .. }
        | ReplyStanding::None => {
            Err(error("stale revision completion: reply is no longer ready").into())
        }
    }
}

pub(super) async fn record_committed_proposal_draft_revision_in_tx(
    tx: &DatabaseTransaction,
    scope: &Scope,
    conversation_id: &str,
    reply_id: u64,
    replaced_text: &str,
    new_draft_revision: u64,
    timestamp: i64,
) -> Result<(), RuntimeError> {
    let reply_id_i64 = to_i64(reply_id, "reply_id")?;
    let Some(binding) = reply_draft_binding::Entity::find_by_id((
        scope.practice.clone(),
        conversation_id.to_owned(),
        scope.physician.clone(),
        reply_id_i64,
    ))
    .lock_exclusive()
    .one(tx)
    .await?
    else {
        return Err(error("missing reply_draft_binding for committed revision").into());
    };
    let pre_revision_your_text = match binding.standing.as_str() {
        "your_text" => replaced_text.to_owned(),
        "coda_revision" => binding
            .pre_revision_your_text
            .clone()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| replaced_text.to_owned()),
        "detached_chose_update" => {
            return Err(error("stale revision completion after choosing Coda update").into());
        }
        _ => return Err(error("corrupt reply_draft_binding.standing").into()),
    };

    reply_draft_binding::ActiveModel {
        practice: Unchanged(binding.practice),
        conversation_id: Unchanged(binding.conversation_id),
        physician: Unchanged(binding.physician),
        reply_id: Unchanged(binding.reply_id),
        workspace: Unchanged(binding.workspace),
        draft_scope_chat: Unchanged(binding.draft_scope_chat),
        standing: Set("coda_revision".to_owned()),
        draft_revision: Set(to_i64(new_draft_revision, "draft_revision")?),
        based_on_proposal_version: Unchanged(binding.based_on_proposal_version),
        pre_revision_your_text: Set(Some(pre_revision_your_text)),
        update_offered_proposal_version: Unchanged(binding.update_offered_proposal_version),
        updated_at: Set(timestamp),
    }
    .update(tx)
    .await?;
    Ok(())
}

async fn cancel_running_proposal_draft_revisions_in_tx(
    tx: &DatabaseTransaction,
    practice: &str,
    workspace: &str,
    conversation_id: &str,
    reply_id: u64,
) -> Result<(), RuntimeError> {
    let chat_key = format!("{PROPOSAL_DRAFT_CHAT_PREFIX}{conversation_id}:{reply_id}");
    let draft_rows = reply_draft::Entity::find()
        .filter(reply_draft::Column::Practice.eq(practice))
        .filter(reply_draft::Column::Workspace.eq(workspace))
        .filter(reply_draft::Column::Reply.eq(chat_key.as_str()))
        .lock_exclusive()
        .all(tx)
        .await?;
    for row in draft_rows {
        let scope = Scope {
            practice: row.practice.clone(),
            physician: row.physician.clone(),
            workspace: row.workspace.clone(),
            chat: row.reply.clone(),
        };
        let mut state = Store::draft_control(tx, &scope, row).await?;
        if state.running() {
            state.cancel()?;
            control::save_revision_attempts(tx, &scope, &state.attempts).await?;
            draft_model(&scope, &state)?.update(tx).await?;
        }
    }
    Ok(())
}

impl Store {
    /// Registers or re-reads a reply conversation and its initial [`ConversationLink`].
    pub async fn link_reply_conversation(
        &self,
        practice: &str,
        conversation_id: &str,
        link: ConversationLink,
    ) -> Result<ReplyConversation, ReplyProposalError> {
        let practice = validate_identifier("practice", practice)?;
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let link = validate_link(&link)?;

        let tx = self.db.begin().await?;
        if let Some(existing_row) =
            reply_conversation::Entity::find_by_id((practice.clone(), conversation_id.clone()))
                .lock_exclusive()
                .one(&tx)
                .await?
        {
            let existing = decode_conversation(&tx, existing_row).await?;
            if existing.link != link {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: format!("link:{practice}:{conversation_id}"),
                });
            }
            tx.commit().await?;
            return Ok(existing);
        }

        let timestamp = now(&tx).await?;
        let conv = ReplyConversation {
            practice,
            conversation_id,
            link,
            channel: ChannelConnection::Connected,
            conversation_version: 1,
            move_epoch: 0,
            last_reply_id: 0,
            last_proposal_version: 0,
            last_practice_observation_seq: 0,
            patient_messages: Vec::new(),
            practice_messages: Vec::new(),
            recorded_practice_history: Vec::new(),
            woke_events: BTreeSet::new(),
            active_review: None,
            pending_reply_request: None,
            reply_standing: ReplyStanding::None,
            associated_concern_ids: BTreeSet::new(),
            created_at: timestamp,
            updated_at: timestamp,
        };
        reply_conversation::Entity::insert(encode_conversation_model(&conv)?)
            .exec_without_returning(&tx)
            .await?;
        tx.commit().await?;
        Ok(conv)
    }

    /// Reads the durable state of a reply conversation, if linked.
    pub async fn reply_conversation(
        &self,
        practice: &str,
        conversation_id: &str,
    ) -> Result<Option<ReplyConversation>, ReplyProposalError> {
        let practice = validate_identifier("practice", practice)?;
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let tx = self.db.begin().await?;
        let maybe_row = reply_conversation::Entity::find_by_id((practice, conversation_id))
            .one(&tx)
            .await?;
        let decoded = match maybe_row {
            Some(row) => Some(decode_conversation(&tx, row).await?),
            None => None,
        };
        tx.commit().await?;
        Ok(decoded)
    }

    /// Lists every saved proposal version on a conversation, oldest first.
    pub async fn reply_proposals(
        &self,
        practice: &str,
        conversation_id: &str,
    ) -> Result<Vec<SavedReplyProposal>, ReplyProposalError> {
        let practice = validate_identifier("practice", practice)?;
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let rows = reply_proposal::Entity::find()
            .filter(reply_proposal::Column::Practice.eq(practice))
            .filter(reply_proposal::Column::ConversationId.eq(conversation_id))
            .order_by_asc(reply_proposal::Column::Version)
            .all(&self.db)
            .await?;
        rows.into_iter()
            .map(decode_proposal)
            .collect::<Result<Vec<_>, _>>()
            .map_err(ReplyProposalError::from)
    }

    /// Lists every saved `Copy reply` record on a conversation, oldest first.
    pub async fn reply_copy_records(
        &self,
        practice: &str,
        conversation_id: &str,
    ) -> Result<Vec<ReplyCopyRecord>, ReplyProposalError> {
        let practice = validate_identifier("practice", practice)?;
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let rows = reply_copy::Entity::find()
            .filter(reply_copy::Column::Practice.eq(practice))
            .filter(reply_copy::Column::ConversationId.eq(conversation_id))
            .order_by_asc(reply_copy::Column::CopiedAt)
            .order_by_asc(reply_copy::Column::CopyId)
            .all(&self.db)
            .await?;
        rows.into_iter()
            .map(decode_copy_record)
            .collect::<Result<Vec<_>, _>>()
            .map_err(ReplyProposalError::from)
    }

    /// Associates a patient concern raised during message review with this conversation,
    /// so only a move to `NotAPatient` retires it alongside the conversation (`Rule 3`).
    pub async fn attach_reply_conversation_concern(
        &self,
        practice: &str,
        conversation_id: &str,
        concern_id: &ConcernId,
    ) -> Result<ReplyConversation, ReplyProposalError> {
        let practice = validate_identifier("practice", practice)?;
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let tx = self.db.begin().await?;
        let mut conv = load_locked_conversation(&tx, &practice, &conversation_id).await?;
        if matches!(conv.link, ConversationLink::NotAPatient) {
            tx.rollback().await?;
            return Err(ReplyProposalError::ConversationNotLinkedToPatient { conversation_id });
        }
        conv.associated_concern_ids.insert(concern_id.clone());
        conv.updated_at = now(&tx).await?;
        encode_conversation_model(&conv)?.update(&tx).await?;
        tx.commit().await?;
        Ok(conv)
    }

    /// Records a linked patient message (`PatientWrites`), ending any closed-reply `Undo`
    /// and queuing at most one deduplicated review wake event (`Rule 7`, `Rule 12`).
    pub async fn record_linked_patient_message(
        &self,
        input: &RecordPatientMessage,
    ) -> Result<MessageIntakeOutcome, ReplyProposalError> {
        let practice = validate_identifier("practice", &input.practice)?;
        let conversation_id = validate_identifier("conversation_id", &input.conversation_id)?;
        let message_id = validate_identifier("message_id", &input.message_id)?;
        let sender = validate_sender(&input.sender)?;
        let text = validate_channel_message_text("text", &input.text, MAX_MESSAGE_TEXT_CHARS)?;

        let tx = self.db.begin().await?;
        let mut conv = load_locked_conversation(&tx, &practice, &conversation_id).await?;
        if let Some(existing) = conv
            .patient_messages
            .iter()
            .find(|m| m.message_id == message_id)
        {
            if existing.text != text
                || existing.sender != sender
                || existing.sent_at != input.sent_at
            {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision { key: message_id });
            }
            tx.commit().await?;
            return Ok(MessageIntakeOutcome::Duplicate {
                conversation: Box::new(conv),
            });
        }
        if matches!(conv.link, ConversationLink::NotAPatient) {
            tx.rollback().await?;
            return Err(ReplyProposalError::ConversationNotLinkedToPatient { conversation_id });
        }
        if matches!(conv.channel, ChannelConnection::Disconnected { .. }) {
            tx.rollback().await?;
            return Err(ReplyProposalError::ChannelDisconnected { conversation_id });
        }

        let timestamp = now(&tx).await?;
        conv.conversation_version += 1;
        let sequence = conv.patient_messages.len() as u64 + 1;
        conv.patient_messages.push(LinkedPatientMessage {
            message_id: message_id.clone(),
            sequence,
            sender,
            text,
            sent_at: input.sent_at,
            linked_at_version: conv.conversation_version,
            standing: PatientMessageStanding::Open,
        });
        conv.woke_events
            .insert(ReviewWakeEvent::PatientMessage { message_id });
        end_closed_undo(&mut conv.reply_standing);
        conv.updated_at = timestamp;
        encode_conversation_model(&conv)?.update(&tx).await?;
        tx.commit().await?;
        Ok(MessageIntakeOutcome::Recorded {
            conversation: Box::new(conv),
        })
    }

    /// Observes a channel message sent by the practice (`PracticeSends`), clearing
    /// prior copy attributions on the card, ending `Undo`, and queuing one review wake.
    pub async fn observe_practice_message(
        &self,
        input: &ObservePracticeMessage,
    ) -> Result<MessageIntakeOutcome, ReplyProposalError> {
        let practice = validate_identifier("practice", &input.practice)?;
        let conversation_id = validate_identifier("conversation_id", &input.conversation_id)?;
        let practice_message_id =
            validate_identifier("practice_message_id", &input.practice_message_id)?;
        let text = validate_channel_message_text("text", &input.text, MAX_MESSAGE_TEXT_CHARS)?;

        let tx = self.db.begin().await?;
        let mut conv = load_locked_conversation(&tx, &practice, &conversation_id).await?;
        if let Some(existing) = conv
            .practice_messages
            .iter()
            .find(|m| m.practice_message_id == practice_message_id)
        {
            if existing.text != text
                || existing.kind != input.kind
                || existing.sent_at != input.sent_at
            {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: practice_message_id,
                });
            }
            tx.commit().await?;
            return Ok(MessageIntakeOutcome::Duplicate {
                conversation: Box::new(conv),
            });
        }
        if matches!(conv.link, ConversationLink::NotAPatient) {
            tx.rollback().await?;
            return Err(ReplyProposalError::ConversationNotLinkedToPatient { conversation_id });
        }
        if matches!(conv.channel, ChannelConnection::Disconnected { .. }) {
            tx.rollback().await?;
            return Err(ReplyProposalError::ChannelDisconnected { conversation_id });
        }

        let timestamp = now(&tx).await?;
        conv.conversation_version += 1;
        conv.last_practice_observation_seq += 1;
        let sequence = conv.practice_messages.len() as u64 + 1;
        conv.practice_messages.push(ObservedPracticeMessage {
            practice_message_id: practice_message_id.clone(),
            sequence,
            kind: input.kind,
            text,
            sent_at: input.sent_at,
            observed_at: timestamp,
            observed_at_version: conv.conversation_version,
        });
        conv.woke_events.insert(ReviewWakeEvent::PracticeMessage {
            practice_message_id,
        });
        end_closed_undo(&mut conv.reply_standing);
        conv.updated_at = timestamp;
        encode_conversation_model(&conv)?.update(&tx).await?;
        tx.commit().await?;
        Ok(MessageIntakeOutcome::Recorded {
            conversation: Box::new(conv),
        })
    }

    /// Starts a waiting review (`ReviewStarts`) or reads the full current conversation
    /// for an already-active review (`ReviewReadsAgain`), pinning workspace sources
    /// and recording observed practice words into patient history (`Rule 7`, `Rule 8`).
    pub async fn start_or_read_reply_review(
        &self,
        practice: &str,
        conversation_id: &str,
        review_id: &str,
        run_id: &str,
    ) -> Result<ReplyReviewSnapshot, ReplyProposalError> {
        let practice = validate_identifier("practice", practice)?;
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let review_id = validate_identifier("review_id", review_id)?;
        let run_id = validate_identifier("run_id", run_id)?;

        let tx = self.db.begin().await?;
        let mut conv = load_locked_conversation(&tx, &practice, &conversation_id).await?;
        let ConversationLink::LinkedPatient { workspace } = conv.link.clone() else {
            tx.rollback().await?;
            return Err(ReplyProposalError::ConversationNotLinkedToPatient { conversation_id });
        };

        let timestamp = now(&tx).await?;
        let started_at = match &conv.active_review {
            None => {
                if conv.woke_events.is_empty() {
                    tx.rollback().await?;
                    return Err(ReplyProposalError::NoPendingReviewWake { conversation_id });
                }
                timestamp
            }
            Some(active) => {
                if active.review_id != review_id {
                    tx.rollback().await?;
                    return Err(ReplyProposalError::StaleReview {
                        review_id,
                        reason: format!(
                            "another review `{}` is already active on `{conversation_id}`",
                            active.review_id
                        ),
                    });
                }
                active.started_at
            }
        };

        let observation = super::super::sources::observed(&tx, &workspace).await?;
        conv.woke_events.clear();
        if conv.pending_reply_request.is_some() {
            if reopen_latest_needs_no_reply_for_request(&mut conv).is_some() {
                conv.conversation_version += 1;
            }
            end_closed_undo(&mut conv.reply_standing);
        }

        let existing_history_ids: BTreeSet<String> = conv
            .recorded_practice_history
            .iter()
            .map(|entry| entry.practice_message_id.clone())
            .collect();
        for observed in &conv.practice_messages {
            if !existing_history_ids.contains(&observed.practice_message_id) {
                conv.recorded_practice_history
                    .push(RecordedPracticeHistoryEntry {
                        practice_message_id: observed.practice_message_id.clone(),
                        kind: observed.kind,
                        text: observed.text.clone(),
                        sent_at: observed.sent_at,
                        recorded_at: timestamp,
                    });
            }
        }

        let read_practice_message_ids: BTreeSet<String> = conv
            .practice_messages
            .iter()
            .map(|m| m.practice_message_id.clone())
            .collect();
        let active = ActiveReplyReview {
            review_id,
            run_id,
            workspace,
            move_epoch: conv.move_epoch,
            started_at,
            read_state: ReviewReadState::Current {
                conversation_version: conv.conversation_version,
                source_version: observation.version,
                read_practice_message_ids,
                saw_physician_reply_request: conv.pending_reply_request.is_some(),
            },
        };
        conv.active_review = Some(active.clone());
        conv.updated_at = timestamp;
        encode_conversation_model(&conv)?.update(&tx).await?;
        tx.commit().await?;

        Ok(ReplyReviewSnapshot {
            review: active,
            conversation: conv,
            source_version: observation.version,
        })
    }

    /// Marks an active review as needing a fresh full-conversation read after worker
    /// interruption/restart (`CodaRestarts`), without requeuing already-consumed wakes (`Rule 7`).
    pub async fn restart_reply_review(
        &self,
        practice: &str,
        conversation_id: &str,
        new_run_id: &str,
    ) -> Result<ReplyConversation, ReplyProposalError> {
        let practice = validate_identifier("practice", practice)?;
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let new_run_id = validate_identifier("new_run_id", new_run_id)?;

        let tx = self.db.begin().await?;
        let mut conv = load_locked_conversation(&tx, &practice, &conversation_id).await?;
        let Some(active) = conv.active_review.as_mut() else {
            tx.rollback().await?;
            return Err(ReplyProposalError::StaleReview {
                review_id: String::new(),
                reason: "no active review to restart".into(),
            });
        };
        active.run_id = new_run_id;
        active.read_state = ReviewReadState::NeedsRead;
        conv.updated_at = now(&tx).await?;
        encode_conversation_model(&conv)?.update(&tx).await?;
        tx.commit().await?;
        Ok(conv)
    }

    /// Completes an active reply review (`ReviewFinishes`) with full idempotency,
    /// move/input/source currency checks, non-automatic read-receipt verification,
    /// partial `Answered` closures, and verified proposal basis & citations.
    pub async fn complete_reply_review(
        &self,
        input: &CompleteReplyReview,
    ) -> Result<ReplyReviewOutcome, ReplyProposalError> {
        let practice = validate_identifier("practice", &input.practice)?;
        let conversation_id = validate_identifier("conversation_id", &input.conversation_id)?;
        let review_id = validate_identifier("review_id", &input.review_id)?;
        let run_id = validate_identifier("run_id", &input.run_id)?;
        let completion_key = validate_identifier("completion_key", &input.completion_key)?;

        let request_digest = crate::sha256_hex(
            serde_json::to_vec(&json!({
                "review_id": &review_id,
                "run_id": &run_id,
                "answered_closures": &input.answered_closures,
                "decision": &input.decision,
                "raised_concern_ids": &input.raised_concern_ids,
            }))
            .map_err(conversion_error("CompleteReplyReview", "json"))?
            .as_slice(),
        );

        let tx = self.db.begin().await?;
        let mut conv = load_locked_conversation(&tx, &practice, &conversation_id).await?;
        if let Some(existing_completion) = reply_review_completion::Entity::find_by_id((
            practice.clone(),
            conversation_id.clone(),
            completion_key.clone(),
        ))
        .one(&tx)
        .await?
        {
            if existing_completion.request_digest != request_digest {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: completion_key,
                });
            }
            let saved: SavedReviewCompletion = serde_json::from_value(existing_completion.outcome)
                .map_err(conversion_error(
                    "reply_review_completion.outcome",
                    "SavedReviewCompletion",
                ))?;
            tx.commit().await?;
            return Ok(ReplyReviewOutcome::Duplicate(Box::new(saved)));
        }

        let ConversationLink::LinkedPatient { workspace } = conv.link.clone() else {
            tx.rollback().await?;
            return Err(ReplyProposalError::StaleReview {
                review_id,
                reason: "conversation moved to not-a-patient".into(),
            });
        };
        let Some(active) = conv.active_review.clone() else {
            tx.rollback().await?;
            return Err(ReplyProposalError::StaleReview {
                review_id,
                reason: "no active review on conversation".into(),
            });
        };
        if active.review_id != review_id {
            tx.rollback().await?;
            return Err(ReplyProposalError::StaleReview {
                review_id,
                reason: format!("active review is `{}`", active.review_id),
            });
        }
        if active.workspace != workspace || active.move_epoch != conv.move_epoch {
            tx.rollback().await?;
            return Err(ReplyProposalError::StaleReview {
                review_id,
                reason: format!(
                    "review pinned workspace `{}` at move_epoch {}, current is `{workspace}` at {}",
                    active.workspace, active.move_epoch, conv.move_epoch
                ),
            });
        }
        if active.run_id != run_id {
            tx.rollback().await?;
            return Err(ReplyProposalError::StaleReview {
                review_id,
                reason: format!("active run is `{}`, not `{run_id}`", active.run_id),
            });
        }
        if !conv.woke_events.is_empty() {
            let read_v = match active.read_state {
                ReviewReadState::NeedsRead => None,
                ReviewReadState::Current {
                    conversation_version,
                    ..
                } => Some(conversation_version),
            };
            tx.rollback().await?;
            return Err(ReplyProposalError::StaleConversationRead {
                review_id,
                read_version: read_v,
                current_version: conv.conversation_version,
            });
        }
        let ReviewReadState::Current {
            conversation_version: read_conv_version,
            source_version: read_source_version,
            read_practice_message_ids,
            saw_physician_reply_request,
        } = active.read_state
        else {
            tx.rollback().await?;
            return Err(ReplyProposalError::StaleConversationRead {
                review_id,
                read_version: None,
                current_version: conv.conversation_version,
            });
        };
        if read_conv_version != conv.conversation_version {
            tx.rollback().await?;
            return Err(ReplyProposalError::StaleConversationRead {
                review_id,
                read_version: Some(read_conv_version),
                current_version: conv.conversation_version,
            });
        }

        let observation = super::super::sources::observed(&tx, &workspace).await?;
        if observation.version != read_source_version {
            tx.rollback().await?;
            return Err(ReplyProposalError::StaleSourceObservation {
                review_id,
                read_source_version,
                current_source_version: observation.version,
            });
        }

        // Validate practice-message closures (`Rule 2: OnlyTheReviewAnswers`).
        let practice_by_id: BTreeMap<&str, &ObservedPracticeMessage> = conv
            .practice_messages
            .iter()
            .map(|m| (m.practice_message_id.as_str(), m))
            .collect();
        let known_patient_ids: BTreeSet<&str> = conv
            .patient_messages
            .iter()
            .map(|m| m.message_id.as_str())
            .collect();
        let mut closure_map: BTreeMap<String, String> = BTreeMap::new();
        for closure in &input.answered_closures {
            let pm_id = validate_identifier(
                "answered_closures.practice_message_id",
                &closure.practice_message_id,
            )?;
            if !read_practice_message_ids.contains(&pm_id) {
                tx.rollback().await?;
                return Err(ReplyProposalError::UnreadPracticeMessage {
                    review_id,
                    practice_message_id: pm_id,
                });
            }
            let Some(pm) = practice_by_id.get(pm_id.as_str()).copied() else {
                tx.rollback().await?;
                return Err(ReplyProposalError::UnreadPracticeMessage {
                    review_id,
                    practice_message_id: pm_id,
                });
            };
            match pm.kind {
                PracticeMessageKind::Automatic => {
                    tx.rollback().await?;
                    return Err(ReplyProposalError::AutomaticPracticeMessageCannotAnswer {
                        practice_message_id: pm_id,
                    });
                }
                PracticeMessageKind::PracticeSent => {}
            }
            for raw_msg_id in &closure.answered_message_ids {
                let msg_id =
                    validate_identifier("answered_closures.answered_message_ids", raw_msg_id)?;
                if !known_patient_ids.contains(msg_id.as_str()) {
                    tx.rollback().await?;
                    return Err(ReplyProposalError::UnknownPatientMessage {
                        conversation_id,
                        message_id: msg_id,
                    });
                }
                if let Some(existing_pm) = closure_map.get(&msg_id)
                    && existing_pm != &pm_id
                {
                    tx.rollback().await?;
                    return Err(ReplyProposalError::InvalidInput(
                        ReplyProposalInputError::ConflictingAnsweredClosure { message_id: msg_id },
                    ));
                }
                closure_map.insert(msg_id, pm_id.clone());
            }
        }

        let open_before: BTreeSet<String> = conv
            .patient_messages
            .iter()
            .filter(|m| m.standing.is_open())
            .map(|m| m.message_id.clone())
            .collect();
        let newly_answered: BTreeSet<String> = open_before
            .iter()
            .filter(|id| closure_map.contains_key(*id))
            .cloned()
            .collect();
        let remaining_open: BTreeSet<String> =
            open_before.difference(&newly_answered).cloned().collect();

        let timestamp = now(&tx).await?;
        let mut newly_needs_no_reply = BTreeSet::new();
        let mut offered_update_physicians = BTreeSet::new();
        let mut saved_proposal_out: Option<SavedReplyProposal> = None;

        match &input.decision {
            ReplyReviewDecision::ProposeReply(proposal_input) => {
                let covered_message_ids = if !remaining_open.is_empty() {
                    remaining_open
                } else if saw_physician_reply_request {
                    let Some(last_msg) = conv.patient_messages.last() else {
                        tx.rollback().await?;
                        return Err(ReplyProposalError::NoOpenMessagesForProposal);
                    };
                    BTreeSet::from([last_msg.message_id.clone()])
                } else {
                    tx.rollback().await?;
                    return Err(ReplyProposalError::NoOpenMessagesForProposal);
                };
                let text = validate_text(
                    "proposal.text",
                    &proposal_input.text,
                    MAX_MESSAGE_TEXT_CHARS,
                )?;
                let basis_summary = validate_text(
                    "proposal.basis.summary",
                    &proposal_input.basis.summary,
                    MAX_REASON_CHARS,
                )?;
                let mut limits = Vec::with_capacity(proposal_input.basis.limits.len());
                for limit in &proposal_input.basis.limits {
                    limits.push(validate_single_line("proposal.basis.limits", limit)?);
                }
                let basis_refs = normalize_record_references(
                    "proposal.basis.cited_records",
                    &proposal_input.basis.cited_records,
                )?;
                let claim_refs = normalize_record_references(
                    "proposal.cited_records",
                    &proposal_input.cited_records,
                )?;

                let by_candidate: BTreeMap<&str, &crate::physician_sources::SourceRevision> =
                    observation
                        .sources
                        .iter()
                        .map(|s| (s.candidate_id.as_str(), s))
                        .collect();
                let pulls = load_workspace_pulls_in_tx(&tx, &workspace).await?;
                let mut all_refs = Vec::new();
                for r in claim_refs.iter().chain(basis_refs.iter()) {
                    if let Some(existing) = all_refs
                        .iter()
                        .find(|x: &&ExactRecordReference| x.candidate_id == r.candidate_id)
                    {
                        if existing.revision != r.revision {
                            tx.rollback().await?;
                            return Err(ReplyProposalError::InvalidInput(
                                ReplyProposalInputError::ConflictingRecordRevision {
                                    candidate_id: r.candidate_id.clone(),
                                },
                            ));
                        }
                    } else {
                        all_refs.push(r.clone());
                    }
                }
                let mut verified_citations = Vec::with_capacity(all_refs.len());
                for reference in &all_refs {
                    let Some(source) = by_candidate.get(reference.candidate_id.as_str()).copied()
                    else {
                        tx.rollback().await?;
                        return Err(ReplyProposalError::UnverifiedCitation {
                            candidate_id: reference.candidate_id.clone(),
                            revision: reference.revision.clone(),
                        });
                    };
                    if source.revision != reference.revision {
                        tx.rollback().await?;
                        return Err(ReplyProposalError::UnverifiedCitation {
                            candidate_id: reference.candidate_id.clone(),
                            revision: reference.revision.clone(),
                        });
                    }
                    verified_citations.push(build_summary_citation(
                        &workspace, reference, source, &pulls,
                    ));
                }

                for msg in &mut conv.patient_messages {
                    if newly_answered.contains(&msg.message_id)
                        && let Some(pm_id) = closure_map.get(&msg.message_id)
                    {
                        msg.standing = PatientMessageStanding::Answered {
                            practice_message_id: pm_id.clone(),
                            review_id: review_id.clone(),
                            answered_at: timestamp,
                        };
                    }
                }

                let (reply_id, replaced_proposal_version) = match &conv.reply_standing {
                    ReplyStanding::Ready { proposal } => {
                        (proposal.reply_id, Some(proposal.version))
                    }
                    ReplyStanding::Withheld { reply_id, .. } => (*reply_id, None),
                    ReplyStanding::None | ReplyStanding::ClosedWithUndo { .. } => {
                        conv.last_reply_id += 1;
                        (conv.last_reply_id, None)
                    }
                };
                conv.last_proposal_version += 1;
                let next_version = conv.last_proposal_version;

                let basis = ReplyProposalBasis {
                    summary: basis_summary,
                    limits,
                    cited_records: basis_refs,
                };
                let saved = SavedReplyProposal {
                    practice: practice.clone(),
                    conversation_id: conversation_id.clone(),
                    reply_id,
                    version: next_version,
                    workspace: workspace.clone(),
                    covered_message_ids,
                    text: text.clone(),
                    basis: basis.clone(),
                    citations: verified_citations.clone(),
                    review_id: review_id.clone(),
                    run_id: run_id.clone(),
                    completion_key: completion_key.clone(),
                    conversation_version: read_conv_version,
                    source_version: observation.version,
                    move_epoch: conv.move_epoch,
                    replaced_proposal_version,
                    proposed_at: timestamp,
                };

                reply_proposal::Entity::insert(reply_proposal::ActiveModel {
                    practice: Set(practice.clone()),
                    conversation_id: Set(conversation_id.clone()),
                    version: Set(to_i64(next_version, "proposal.version")?),
                    reply_id: Set(to_i64(reply_id, "proposal.reply_id")?),
                    workspace: Set(workspace.clone()),
                    review_id: Set(review_id.clone()),
                    run_id: Set(run_id.clone()),
                    completion_key: Set(completion_key.clone()),
                    conversation_version: Set(to_i64(read_conv_version, "conversation_version")?),
                    source_version: Set(to_i64(observation.version, "source_version")?),
                    move_epoch: Set(to_i64(conv.move_epoch, "move_epoch")?),
                    replaced_proposal_version: Set(replaced_proposal_version
                        .map(|v| to_i64(v, "replaced_proposal_version"))
                        .transpose()?),
                    text: Set(text),
                    covered_message_ids: Set(serde_json::to_value(&saved.covered_message_ids)
                        .map_err(conversion_error("covered_message_ids", "json"))?),
                    basis: Set(
                        serde_json::to_value(&basis).map_err(conversion_error("basis", "json"))?
                    ),
                    citations: Set(serde_json::to_value(&verified_citations)
                        .map_err(conversion_error("citations", "json"))?),
                    proposed_at: Set(timestamp),
                })
                .exec_without_returning(&tx)
                .await?;

                // Offer Coda's update to any physician holding an active draft on `reply_id` (`Rule 4`, `AC6`).
                let binding_rows = reply_draft_binding::Entity::find()
                    .filter(reply_draft_binding::Column::Practice.eq(practice.as_str()))
                    .filter(
                        reply_draft_binding::Column::ConversationId.eq(conversation_id.as_str()),
                    )
                    .filter(reply_draft_binding::Column::ReplyId.eq(to_i64(reply_id, "reply_id")?))
                    .lock_exclusive()
                    .all(&tx)
                    .await?;
                for row in binding_rows {
                    if matches!(row.standing.as_str(), "your_text" | "coda_revision") {
                        offered_update_physicians.insert(row.physician.clone());
                        reply_draft_binding::ActiveModel {
                            practice: Unchanged(row.practice),
                            conversation_id: Unchanged(row.conversation_id),
                            physician: Unchanged(row.physician),
                            reply_id: Unchanged(row.reply_id),
                            workspace: Unchanged(row.workspace),
                            draft_scope_chat: Unchanged(row.draft_scope_chat),
                            standing: Unchanged(row.standing),
                            draft_revision: Unchanged(row.draft_revision),
                            based_on_proposal_version: Unchanged(row.based_on_proposal_version),
                            pre_revision_your_text: Unchanged(row.pre_revision_your_text),
                            update_offered_proposal_version: Set(Some(to_i64(
                                next_version,
                                "offered_version",
                            )?)),
                            updated_at: Set(timestamp),
                        }
                        .update(&tx)
                        .await?;
                    }
                }

                conv.reply_standing = ReplyStanding::Ready {
                    proposal: Box::new(saved.clone()),
                };
                conv.pending_reply_request = None;
                saved_proposal_out = Some(saved);
            }
            ReplyReviewDecision::NoReplyWouldHelp { reason } => {
                let clean_reason = validate_text("decision.reason", reason, MAX_REASON_CHARS)?;
                if saw_physician_reply_request {
                    tx.rollback().await?;
                    return Err(ReplyProposalError::PhysicianRequestedReplyRequiresProposal);
                }
                for msg in &mut conv.patient_messages {
                    if newly_answered.contains(&msg.message_id)
                        && let Some(pm_id) = closure_map.get(&msg.message_id)
                    {
                        msg.standing = PatientMessageStanding::Answered {
                            practice_message_id: pm_id.clone(),
                            review_id: review_id.clone(),
                            answered_at: timestamp,
                        };
                    } else if remaining_open.contains(&msg.message_id) {
                        newly_needs_no_reply.insert(msg.message_id.clone());
                        msg.standing = PatientMessageStanding::NeedsNoReply {
                            reason: NoReplyReason::ReviewJudgedNoReplyWouldHelp {
                                review_id: review_id.clone(),
                                reason: clean_reason.clone(),
                            },
                            decided_at: timestamp,
                        };
                    }
                }
                conv.reply_standing = ReplyStanding::None;
                if let Some(pending) = &conv.pending_reply_request {
                    let request_id = pending.request_id.clone();
                    if let Some(reopened_id) = reopen_latest_needs_no_reply_for_request(&mut conv) {
                        newly_needs_no_reply.remove(&reopened_id);
                    }
                    conv.conversation_version += 1;
                    conv.woke_events
                        .insert(ReviewWakeEvent::PhysicianAskedForReply { request_id });
                }
            }
            ReplyReviewDecision::AllHandled => {
                if !remaining_open.is_empty() {
                    tx.rollback().await?;
                    return Err(ReplyProposalError::OpenMessagesRemainUnhandled);
                }
                if saw_physician_reply_request {
                    tx.rollback().await?;
                    return Err(ReplyProposalError::PhysicianRequestedReplyRequiresProposal);
                }
                for msg in &mut conv.patient_messages {
                    if newly_answered.contains(&msg.message_id)
                        && let Some(pm_id) = closure_map.get(&msg.message_id)
                    {
                        msg.standing = PatientMessageStanding::Answered {
                            practice_message_id: pm_id.clone(),
                            review_id: review_id.clone(),
                            answered_at: timestamp,
                        };
                    }
                }
                conv.reply_standing = ReplyStanding::None;
                if let Some(pending) = &conv.pending_reply_request {
                    let request_id = pending.request_id.clone();
                    let _ = reopen_latest_needs_no_reply_for_request(&mut conv);
                    conv.conversation_version += 1;
                    conv.woke_events
                        .insert(ReviewWakeEvent::PhysicianAskedForReply { request_id });
                }
            }
        }

        for concern_id in &input.raised_concern_ids {
            conv.associated_concern_ids.insert(concern_id.clone());
        }
        conv.active_review = None;
        conv.updated_at = timestamp;
        encode_conversation_model(&conv)?.update(&tx).await?;

        let completion = SavedReviewCompletion {
            review_id: review_id.clone(),
            run_id: run_id.clone(),
            completion_key: completion_key.clone(),
            workspace,
            newly_answered_message_ids: newly_answered,
            newly_needs_no_reply_message_ids: newly_needs_no_reply,
            proposal: saved_proposal_out,
            offered_update_physicians,
            completed_at: timestamp,
        };
        reply_review_completion::Entity::insert(reply_review_completion::ActiveModel {
            practice: Set(practice),
            conversation_id: Set(conversation_id),
            completion_key: Set(completion_key),
            request_digest: Set(request_digest),
            review_id: Set(review_id),
            run_id: Set(run_id),
            workspace: Set(completion.workspace.clone()),
            outcome: Set(serde_json::to_value(&completion)
                .map_err(conversion_error("SavedReviewCompletion", "json"))?),
            completed_at: Set(timestamp),
        })
        .exec_without_returning(&tx)
        .await?;

        tx.commit().await?;
        Ok(ReplyReviewOutcome::Applied(Box::new(completion)))
    }

    /// Acts on a settled message-matching move (`CodaActsOnMove`):
    /// - To another patient: stops any active review, cancels pre-move draft revisions,
    ///   withholds any open reply under `reply_id + 1` for the new patient, leaves
    ///   handled messages handled, and wakes one review for the new patient (`Rule 10`).
    /// - To `NotAPatient`: marks open messages `MovedToNotPatient`, closes the reply,
    ///   clears pending wakes, and retires open concerns linked to this conversation (`Rule 3`).
    pub async fn act_on_conversation_move(
        &self,
        command_key: &CommandKey,
        practice: &str,
        conversation_id: &str,
        target: ConversationLink,
    ) -> Result<ConversationMoveOutcome, ReplyProposalError> {
        let practice = validate_identifier("practice", practice)?;
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let target = validate_link(&target)?;

        let tx = self.db.begin().await?;
        let command = json!({
            "kind": "reply_conversation_move",
            "practice": &practice,
            "conversation_id": &conversation_id,
            "target": &target,
        });
        match receipt::record(
            &tx,
            &practice,
            JOB_RECEIPT_ACTOR,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            Receipt::New => {}
            Receipt::Same => {
                let conv = load_locked_conversation(&tx, &practice, &conversation_id).await?;
                tx.commit().await?;
                return Ok(ConversationMoveOutcome::Duplicate {
                    conversation: Box::new(conv),
                });
            }
            Receipt::Different => {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
        }

        let mut conv = load_locked_conversation(&tx, &practice, &conversation_id).await?;
        let ConversationLink::LinkedPatient {
            workspace: old_workspace,
        } = conv.link.clone()
        else {
            tx.rollback().await?;
            return Err(ReplyProposalError::ConversationNotLinkedToPatient { conversation_id });
        };
        if let ConversationLink::LinkedPatient { workspace: new_ws } = &target
            && new_ws == &old_workspace
        {
            tx.rollback().await?;
            return Err(ReplyProposalError::MoveTargetUnchanged {
                conversation_id,
                workspace: old_workspace,
            });
        }

        // Cancel any running draft revision on the pre-move reply so it cannot publish stale edits.
        match &conv.reply_standing {
            ReplyStanding::Ready { proposal } => {
                cancel_running_proposal_draft_revisions_in_tx(
                    &tx,
                    &practice,
                    &old_workspace,
                    &conversation_id,
                    proposal.reply_id,
                )
                .await?;
            }
            ReplyStanding::Withheld { reply_id, .. } => {
                cancel_running_proposal_draft_revisions_in_tx(
                    &tx,
                    &practice,
                    &old_workspace,
                    &conversation_id,
                    *reply_id,
                )
                .await?;
            }
            ReplyStanding::ClosedWithUndo { proposal, .. } => {
                cancel_running_proposal_draft_revisions_in_tx(
                    &tx,
                    &practice,
                    &old_workspace,
                    &conversation_id,
                    proposal.reply_id,
                )
                .await?;
            }
            ReplyStanding::None => {}
        }

        let timestamp = now(&tx).await?;
        conv.move_epoch += 1;
        conv.conversation_version += 1;
        conv.active_review = None;
        conv.pending_reply_request = None;

        match target {
            ConversationLink::LinkedPatient {
                workspace: new_workspace,
            } => {
                conv.link = ConversationLink::LinkedPatient {
                    workspace: new_workspace.clone(),
                };
                let next_standing = match &conv.reply_standing {
                    ReplyStanding::Ready { proposal } => {
                        conv.last_reply_id += 1;
                        ReplyStanding::Withheld {
                            reply_id: conv.last_reply_id,
                            workspace: new_workspace,
                            covered_message_ids: proposal.covered_message_ids.clone(),
                            withheld_from_proposal_version: proposal.version,
                            move_epoch: conv.move_epoch,
                            withheld_at: timestamp,
                        }
                    }
                    ReplyStanding::Withheld {
                        covered_message_ids,
                        withheld_from_proposal_version,
                        ..
                    } => {
                        conv.last_reply_id += 1;
                        ReplyStanding::Withheld {
                            reply_id: conv.last_reply_id,
                            workspace: new_workspace,
                            covered_message_ids: covered_message_ids.clone(),
                            withheld_from_proposal_version: *withheld_from_proposal_version,
                            move_epoch: conv.move_epoch,
                            withheld_at: timestamp,
                        }
                    }
                    ReplyStanding::ClosedWithUndo { .. } | ReplyStanding::None => {
                        ReplyStanding::None
                    }
                };
                conv.reply_standing = next_standing;
                conv.woke_events.insert(ReviewWakeEvent::MovedToPatient {
                    move_epoch: conv.move_epoch,
                });
                conv.updated_at = timestamp;
                encode_conversation_model(&conv)?.update(&tx).await?;
                tx.commit().await?;
                Ok(ConversationMoveOutcome::MovedToPatient {
                    conversation: Box::new(conv),
                })
            }
            ConversationLink::NotAPatient => {
                conv.link = ConversationLink::NotAPatient;
                for msg in &mut conv.patient_messages {
                    if msg.standing.is_open() {
                        msg.standing = PatientMessageStanding::MovedToNotPatient {
                            move_epoch: conv.move_epoch,
                            moved_at: timestamp,
                        };
                    }
                }
                conv.reply_standing = ReplyStanding::None;
                conv.woke_events.clear();
                conv.updated_at = timestamp;

                let mut retired_concern_ids = Vec::new();
                let retirement_reason =
                    RetirementReason::new("Conversation moved to not-a-patient.").map_err(
                        conversion_error("move retirement reason", "RetirementReason"),
                    )?;
                for concern_id in conv.associated_concern_ids.clone() {
                    if let Some(current_concern) =
                        load_concern(&tx, &practice, &old_workspace, &concern_id).await?
                        && current_concern.shows_on_today()
                    {
                        let source_work = SourceWork::new(
                            "reply_conversation_move",
                            format!("move:{conversation_id}:{}", conv.move_epoch),
                            command_key.as_str(),
                        )
                        .map_err(conversion_error("move source_work", "SourceWork"))?;
                        let key_digest = &crate::sha256_hex(
                            format!(
                                "retire-on-move:{practice}:{old_workspace}:{conversation_id}:{}:{concern_id}",
                                conv.move_epoch
                            )
                            .as_bytes(),
                        )[..24];
                        let retire_key = CommandKey::new(format!("mv-retire:{key_digest}"))
                            .map_err(conversion_error("move retire key", "CommandKey"))?;
                        match retire_concern_in_tx(
                            &tx,
                            &retire_key,
                            &RetireConcern {
                                practice: practice.clone(),
                                workspace: old_workspace.clone(),
                                id: concern_id.clone(),
                                reason: retirement_reason.clone(),
                                source_work,
                            },
                        )
                        .await?
                        {
                            ConcernMutation::Applied(_) | ConcernMutation::Duplicate(_) => {
                                retired_concern_ids.push(concern_id);
                            }
                        }
                    }
                }

                encode_conversation_model(&conv)?.update(&tx).await?;
                tx.commit().await?;
                Ok(ConversationMoveOutcome::MovedToNotAPatient {
                    conversation: Box::new(conv),
                    retired_concern_ids,
                })
            }
        }
    }

    /// Updates the messaging channel's connection state (`ChannelDisconnects` / `CodaReconnects`).
    pub async fn set_reply_channel_connection(
        &self,
        practice: &str,
        conversation_id: &str,
        channel: ChannelConnection,
    ) -> Result<ReplyConversation, ReplyProposalError> {
        let practice = validate_identifier("practice", practice)?;
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let channel = validate_channel(&channel)?;

        let tx = self.db.begin().await?;
        let mut conv = load_locked_conversation(&tx, &practice, &conversation_id).await?;
        conv.channel = channel;
        conv.updated_at = now(&tx).await?;
        encode_conversation_model(&conv)?.update(&tx).await?;
        tx.commit().await?;
        Ok(conv)
    }

    /// Projects the reply card (`Card(d)`) for an authorized physician (`Rule 1: OnlyWithAccess`).
    pub async fn reply_card(
        &self,
        access: &WorkspaceAccess<'_>,
        conversation_id: &str,
        authority: &mut dyn Authority,
    ) -> Result<Option<ReplyCardView>, ReplyProposalError> {
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let scope = verify_physician_access_scope(access)?;
        let tx = self.db.begin().await?;
        if let Err(err) =
            check_physician_read_gate_in_tx(&tx, &scope, access.session_hash, authority).await
        {
            tx.rollback().await?;
            return Err(err);
        }
        let Some(row) = reply_conversation::Entity::find_by_id((
            scope.practice.clone(),
            conversation_id.clone(),
        ))
        .one(&tx)
        .await?
        else {
            tx.rollback().await?;
            return Err(ReplyProposalError::ConversationNotFound { conversation_id });
        };
        let conv = decode_conversation(&tx, row).await?;
        if let Err(err) = verify_conversation_workspace_in_tx(&tx, access, &conv).await {
            tx.rollback().await?;
            return Err(err);
        }
        let card = project_reply_card_in_tx(&tx, &conv, &scope.physician).await?;
        tx.commit().await?;
        Ok(card)
    }

    /// Projects the full reply surface visible to an authorized physician (`OpenCard`,
    /// `ClosedUndoBanner`, or `NoOpenCard` with `Coda is reviewing` for open messages).
    pub async fn physician_reply_surface(
        &self,
        access: &WorkspaceAccess<'_>,
        conversation_id: &str,
        authority: &mut dyn Authority,
    ) -> Result<PhysicianReplySurface, ReplyProposalError> {
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let scope = verify_physician_access_scope(access)?;
        let tx = self.db.begin().await?;
        if let Err(err) =
            check_physician_read_gate_in_tx(&tx, &scope, access.session_hash, authority).await
        {
            tx.rollback().await?;
            return Err(err);
        }
        let Some(row) = reply_conversation::Entity::find_by_id((
            scope.practice.clone(),
            conversation_id.clone(),
        ))
        .one(&tx)
        .await?
        else {
            tx.rollback().await?;
            return Err(ReplyProposalError::ConversationNotFound { conversation_id });
        };
        let conv = decode_conversation(&tx, row).await?;
        let (_, workspace) = match verify_conversation_workspace_in_tx(&tx, access, &conv).await {
            Ok(pair) => pair,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        if let Some(card) = project_reply_card_in_tx(&tx, &conv, &scope.physician).await? {
            tx.commit().await?;
            return Ok(PhysicianReplySurface::OpenCard(Box::new(card)));
        }
        let review_state = conversation_review_state(&conv);
        if let ReplyStanding::ClosedWithUndo {
            proposal,
            closed_message_ids,
            closed_by,
            ..
        } = &conv.reply_standing
        {
            let surface = PhysicianReplySurface::ClosedUndoBanner {
                conversation_id,
                workspace,
                reply_id: proposal.reply_id,
                proposal_version: proposal.version,
                closed_message_ids: closed_message_ids.clone(),
                closed_by: closed_by.clone(),
                review_state,
            };
            tx.commit().await?;
            return Ok(surface);
        }
        let open_messages = open_patient_messages(&conv);
        let non_patient_sender_names = non_patient_senders(&open_messages);
        let practice_messages = conv.practice_messages;
        tx.commit().await?;
        Ok(PhysicianReplySurface::NoOpenCard {
            conversation_id,
            workspace,
            open_messages,
            practice_messages,
            non_patient_sender_names,
            review_state,
        })
    }

    /// Requests Coda to draft a reply when none is ready (`Ask Coda to reply`),
    /// waiting behind any already queued or running review so a single review can satisfy it (`AC5`).
    pub async fn ask_coda_to_reply(
        &self,
        access: &WorkspaceAccess<'_>,
        command_key: &CommandKey,
        conversation_id: &str,
        authority: &mut dyn Authority,
    ) -> Result<AskCodaToReplyOutcome, ReplyProposalError> {
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let scope = verify_physician_access_scope(access)?;
        let tx = self.db.begin().await?;
        let mut conv = match load_locked_conversation_for_physician(
            &tx,
            &scope,
            access.session_hash,
            &conversation_id,
            authority,
        )
        .await
        {
            Ok(conv) => conv,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        if let Err(err) =
            check_conversation_access_in_tx(&tx, access, &scope, &conv, authority).await
        {
            tx.rollback().await?;
            return Err(err);
        }

        let command = json!({
            "kind": "ask_coda_to_reply",
            "practice": &scope.practice,
            "physician": &scope.physician,
            "workspace": &scope.workspace,
            "conversation_id": &conversation_id,
        });
        match receipt::check(
            &tx,
            &scope.practice,
            &scope.physician,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            None | Some(Receipt::New) => {}
            Some(Receipt::Same) => {
                tx.commit().await?;
                return Ok(AskCodaToReplyOutcome::Duplicate {
                    conversation: Box::new(conv),
                });
            }
            Some(Receipt::Different) => {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
        }

        if let ReplyStanding::Ready { proposal } = &conv.reply_standing {
            let ready = proposal.clone();
            let _ = receipt::record(
                &tx,
                &scope.practice,
                &scope.physician,
                command_key.as_str(),
                &command,
            )
            .await?;
            tx.commit().await?;
            return Ok(AskCodaToReplyOutcome::AlreadyReady { proposal: ready });
        }
        if conv.patient_messages.is_empty() {
            tx.rollback().await?;
            return Err(ReplyProposalError::NoOpenMessagesForProposal);
        }

        match receipt::record(
            &tx,
            &scope.practice,
            &scope.physician,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            Receipt::New => {}
            Receipt::Same => {
                tx.commit().await?;
                return Ok(AskCodaToReplyOutcome::Duplicate {
                    conversation: Box::new(conv),
                });
            }
            Receipt::Different => {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
        }

        let timestamp = now(&tx).await?;
        let request_id = format!("req-{}", Uuid::new_v4());
        let review_already_waiting_or_running =
            !conv.woke_events.is_empty() || conv.active_review.is_some();
        conv.pending_reply_request = Some(PendingPhysicianReplyRequest {
            request_id: request_id.clone(),
            command_key: command_key.to_string(),
            requested_by: scope.physician,
            requested_at: timestamp,
        });

        if review_already_waiting_or_running {
            conv.updated_at = timestamp;
            encode_conversation_model(&conv)?.update(&tx).await?;
            tx.commit().await?;
            return Ok(AskCodaToReplyOutcome::WaitingBehindActiveReview {
                request_id,
                conversation: Box::new(conv),
            });
        }

        // If no open messages remain (e.g. previous review concluded `NoReplyWouldHelp`),
        // reopen the latest `NeedsNoReply` patient message so the requested draft covers it.
        let _ = reopen_latest_needs_no_reply_for_request(&mut conv);
        end_closed_undo(&mut conv.reply_standing);
        conv.conversation_version += 1;
        conv.woke_events
            .insert(ReviewWakeEvent::PhysicianAskedForReply {
                request_id: request_id.clone(),
            });
        conv.updated_at = timestamp;
        encode_conversation_model(&conv)?.update(&tx).await?;
        tx.commit().await?;
        Ok(AskCodaToReplyOutcome::QueuedReview {
            request_id,
            conversation: Box::new(conv),
        })
    }

    /// Saves a physician's private draft edits on the current `Ready` reply (`YouEdit`),
    /// isolated from other physicians and bound to the current `reply_id` (`Rule 4`, `AC6`).
    pub async fn edit_reply_proposal_draft(
        &self,
        access: &WorkspaceAccess<'_>,
        conversation_id: &str,
        expected: ExpectedDraftBase,
        text: &str,
        authority: &mut dyn Authority,
    ) -> Result<ReplyCardView, ReplyProposalError> {
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let clean_text = validate_text("draft.text", text, MAX_MESSAGE_TEXT_CHARS)?;
        let scope = verify_physician_access_scope(access)?;

        let tx = self.db.begin().await?;
        let conv = match load_locked_conversation_for_physician(
            &tx,
            &scope,
            access.session_hash,
            &conversation_id,
            authority,
        )
        .await
        {
            Ok(conv) => conv,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        let workspace =
            match verify_conversation_workspace_for_write(&tx, access, &scope, &conv, authority)
                .await
            {
                Ok(workspace) => workspace,
                Err(err) => {
                    tx.rollback().await?;
                    return Err(err);
                }
            };
        let ReplyStanding::Ready { proposal } = &conv.reply_standing else {
            let label = conv.reply_standing.db_label();
            // Verify access first so unauthorized callers still receive `AccessDenied`.
            if let Err(err) =
                lock_proposal_access_in_tx(&tx, &scope, access.session_hash, authority).await
            {
                tx.rollback().await?;
                return Err(err);
            }
            tx.rollback().await?;
            return Err(ReplyProposalError::ReplyNotInRequiredState { standing: label });
        };

        let reply_id = proposal.reply_id;
        let draft_scope = proposal_draft_scope(
            &scope.practice,
            &scope.physician,
            &workspace,
            &conversation_id,
            reply_id,
        );
        let existing_draft_row = reply_draft::Entity::find_by_id(keys(&draft_scope))
            .lock_exclusive()
            .one(&tx)
            .await?;
        let existing_binding = reply_draft_binding::Entity::find_by_id((
            scope.practice.clone(),
            conversation_id.clone(),
            scope.physician.clone(),
            to_i64(reply_id, "reply_id")?,
        ))
        .lock_exclusive()
        .one(&tx)
        .await?;
        if let Err(err) =
            lock_proposal_access_in_tx(&tx, &scope, access.session_hash, authority).await
        {
            tx.rollback().await?;
            return Err(err);
        }

        let current_binding_standing = match &existing_binding {
            Some(row) => decode_draft_binding_standing(row)?,
            None => PhysicianDraftStanding::NoDraft,
        };
        match (&current_binding_standing, expected) {
            (PhysicianDraftStanding::NoDraft, ExpectedDraftBase::ProposalVersion(v)) => {
                if v != proposal.version {
                    tx.rollback().await?;
                    return Err(ReplyProposalError::StaleDraftVersion {
                        detail: format!(
                            "expected proposal version {v}, current is {}",
                            proposal.version
                        ),
                    });
                }
            }
            (
                PhysicianDraftStanding::Active { draft_revision, .. },
                ExpectedDraftBase::DraftRevision(r),
            ) => {
                if r != *draft_revision {
                    tx.rollback().await?;
                    return Err(ReplyProposalError::StaleDraftVersion {
                        detail: format!("expected draft revision {r}, current is {draft_revision}"),
                    });
                }
            }
            (PhysicianDraftStanding::NoDraft, ExpectedDraftBase::DraftRevision(r)) => {
                tx.rollback().await?;
                return Err(ReplyProposalError::StaleDraftVersion {
                    detail: format!("physician has no active draft (supplied draft revision {r})"),
                });
            }
            (
                PhysicianDraftStanding::Active { draft_revision, .. },
                ExpectedDraftBase::ProposalVersion(v),
            ) => {
                tx.rollback().await?;
                return Err(ReplyProposalError::StaleDraftVersion {
                    detail: format!(
                        "physician already has active draft at revision {draft_revision} (supplied proposal version {v})"
                    ),
                });
            }
        }

        let timestamp = now(&tx).await?;
        let new_draft_revision = match existing_draft_row {
            None => {
                let mut opened = Draft::opened(&clean_text);
                opened.session_hash = Some(access.session_hash.to_vec());
                reply_draft::Entity::insert(draft_model(&draft_scope, &opened)?)
                    .exec_without_returning(&tx)
                    .await?;
                opened.revision
            }
            Some(row) => {
                let mut state = Self::draft_control(&tx, &draft_scope, row).await?;
                state.restore_text = Some(std::mem::replace(&mut state.text, clean_text));
                state.revision += 1;
                state.session_hash = Some(access.session_hash.to_vec());
                if state.running() {
                    state.cancel()?;
                }
                control::save_revision_attempts(&tx, &draft_scope, &state.attempts).await?;
                draft_model(&draft_scope, &state)?.update(&tx).await?;
                state.revision
            }
        };

        match existing_binding {
            None => {
                reply_draft_binding::Entity::insert(reply_draft_binding::ActiveModel {
                    practice: Set(scope.practice.clone()),
                    conversation_id: Set(conversation_id),
                    physician: Set(scope.physician.clone()),
                    reply_id: Set(to_i64(reply_id, "reply_id")?),
                    workspace: Set(workspace),
                    draft_scope_chat: Set(draft_scope.chat),
                    standing: Set("your_text".to_owned()),
                    draft_revision: Set(to_i64(new_draft_revision, "draft_revision")?),
                    based_on_proposal_version: Set(to_i64(proposal.version, "proposal.version")?),
                    pre_revision_your_text: Set(None),
                    update_offered_proposal_version: Set(None),
                    updated_at: Set(timestamp),
                })
                .exec_without_returning(&tx)
                .await?;
            }
            Some(row) => {
                let based_on = match row.standing.as_str() {
                    "detached_chose_update" => to_i64(proposal.version, "proposal.version")?,
                    _ => row.based_on_proposal_version,
                };
                let preserved_offer = match row.standing.as_str() {
                    "detached_chose_update" => None,
                    _ => row.update_offered_proposal_version,
                };
                reply_draft_binding::ActiveModel {
                    practice: Unchanged(row.practice),
                    conversation_id: Unchanged(row.conversation_id),
                    physician: Unchanged(row.physician),
                    reply_id: Unchanged(row.reply_id),
                    workspace: Unchanged(row.workspace),
                    draft_scope_chat: Unchanged(row.draft_scope_chat),
                    standing: Set("your_text".to_owned()),
                    draft_revision: Set(to_i64(new_draft_revision, "draft_revision")?),
                    based_on_proposal_version: Set(based_on),
                    pre_revision_your_text: Set(None),
                    update_offered_proposal_version: Set(preserved_offer),
                    updated_at: Set(timestamp),
                }
                .update(&tx)
                .await?;
            }
        }

        let card = project_reply_card_in_tx(&tx, &conv, &scope.physician)
            .await?
            .ok_or_else(|| error("ready reply failed to project card after draft edit"))?;
        tx.commit().await?;
        Ok(card)
    }

    /// Accepts a physician-requested Coda revision on their private draft (`YouAskRevise`, `AC2`),
    /// pinned to the selected `expected_revision` and workspace sources; repeating the same
    /// command key returns `Duplicate` and preserves any subsequent physician edits untouched.
    pub async fn request_reply_draft_revision(
        &self,
        access: &WorkspaceAccess<'_>,
        conversation_id: &str,
        input: &RevisionInput,
        skills: &crate::physician_chat::PhysicianSkills,
        recipe: &engine::recipe::RequestRecipe,
        authority: &mut dyn Authority,
    ) -> Result<ProposalRevisionAccepted, ReplyProposalError> {
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let _ = validate_identifier("input.key", &input.key)?;
        let _ = validate_text(
            "input.instruction",
            &input.instruction,
            MAX_MESSAGE_TEXT_CHARS,
        )?;
        let _ = validate_single_line("input.timezone", &input.timezone)?;
        let access_scope = verify_physician_access_scope(access)?;

        let tx = self.db.begin().await?;
        let conv = match load_locked_conversation_for_physician(
            &tx,
            &access_scope,
            access.session_hash,
            &conversation_id,
            authority,
        )
        .await
        {
            Ok(conv) => conv,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        let workspace = match verify_conversation_workspace_for_write(
            &tx,
            access,
            &access_scope,
            &conv,
            authority,
        )
        .await
        {
            Ok(workspace) => workspace,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        let ReplyStanding::Ready { proposal } = &conv.reply_standing else {
            let label = conv.reply_standing.db_label();
            if let Err(err) =
                lock_proposal_access_in_tx(&tx, &access_scope, access.session_hash, authority).await
            {
                tx.rollback().await?;
                return Err(err);
            }
            tx.rollback().await?;
            return Err(ReplyProposalError::ReplyNotInRequiredState { standing: label });
        };
        let draft_scope = proposal_draft_scope(
            &access_scope.practice,
            &access_scope.physician,
            &workspace,
            &conversation_id,
            proposal.reply_id,
        );
        let maybe_draft_row = reply_draft::Entity::find_by_id(keys(&draft_scope))
            .lock_exclusive()
            .one(&tx)
            .await?;
        let maybe_binding_row = reply_draft_binding::Entity::find_by_id((
            access_scope.practice.clone(),
            conversation_id,
            access_scope.physician.clone(),
            to_i64(proposal.reply_id, "reply_id")?,
        ))
        .lock_exclusive()
        .one(&tx)
        .await?;
        if let Err(err) =
            lock_proposal_access_in_tx(&tx, &access_scope, access.session_hash, authority).await
        {
            tx.rollback().await?;
            return Err(err);
        }

        let command = json!({
            "scope": &draft_scope,
            "revise": input,
        });
        match receipt::check(
            &tx,
            &draft_scope.practice,
            &draft_scope.physician,
            &input.key,
            &command,
        )
        .await?
        {
            Some(Receipt::Same) => {
                tx.commit().await?;
                return Ok(ProposalRevisionAccepted::Duplicate { scope: draft_scope });
            }
            Some(Receipt::Different) => {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: input.key.clone(),
                });
            }
            None | Some(Receipt::New) => {}
        }

        let Some(draft_row) = maybe_draft_row else {
            tx.rollback().await?;
            return Err(ReplyProposalError::StaleDraftVersion {
                detail: "physician has no saved draft on this reply".into(),
            });
        };
        let Some(binding_row) = maybe_binding_row else {
            tx.rollback().await?;
            return Err(ReplyProposalError::StaleDraftVersion {
                detail: "physician has no active draft binding on this reply".into(),
            });
        };
        if matches!(
            decode_draft_binding_standing(&binding_row)?,
            PhysicianDraftStanding::NoDraft
        ) {
            tx.rollback().await?;
            return Err(ReplyProposalError::StaleDraftVersion {
                detail: "physician draft was detached; edit before requesting revision".into(),
            });
        }

        match receipt::record(
            &tx,
            &draft_scope.practice,
            &draft_scope.physician,
            &input.key,
            &command,
        )
        .await?
        {
            Receipt::New => {}
            Receipt::Same => {
                tx.commit().await?;
                return Ok(ProposalRevisionAccepted::Duplicate { scope: draft_scope });
            }
            Receipt::Different => {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: input.key.clone(),
                });
            }
        }

        if Self::accept_locked_draft_revision_in_tx(
            tx,
            &draft_scope,
            draft_row,
            input,
            access.session_hash.to_vec(),
            skills,
            recipe,
        )
        .await?
        {
            Ok(ProposalRevisionAccepted::New { scope: draft_scope })
        } else {
            Err(ReplyProposalError::StaleDraftVersion {
                detail: format!(
                    "expected idle draft at revision {}",
                    input.expected_revision
                ),
            })
        }
    }

    /// Restores the physician's pre-revision text after Coda revised their draft (`YouRestore`, `AC6`).
    pub async fn restore_my_reply_version(
        &self,
        access: &WorkspaceAccess<'_>,
        command_key: &CommandKey,
        conversation_id: &str,
        expected_draft_revision: u64,
        authority: &mut dyn Authority,
    ) -> Result<ReplyCardView, ReplyProposalError> {
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let scope = verify_physician_access_scope(access)?;
        let tx = self.db.begin().await?;
        let conv = match load_locked_conversation_for_physician(
            &tx,
            &scope,
            access.session_hash,
            &conversation_id,
            authority,
        )
        .await
        {
            Ok(conv) => conv,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        let workspace =
            match verify_conversation_workspace_for_write(&tx, access, &scope, &conv, authority)
                .await
            {
                Ok(workspace) => workspace,
                Err(err) => {
                    tx.rollback().await?;
                    return Err(err);
                }
            };
        let ReplyStanding::Ready { proposal } = &conv.reply_standing else {
            let label = conv.reply_standing.db_label();
            if let Err(err) =
                lock_proposal_access_in_tx(&tx, &scope, access.session_hash, authority).await
            {
                tx.rollback().await?;
                return Err(err);
            }
            tx.rollback().await?;
            return Err(ReplyProposalError::ReplyNotInRequiredState { standing: label });
        };

        let draft_scope = proposal_draft_scope(
            &scope.practice,
            &scope.physician,
            &workspace,
            &conversation_id,
            proposal.reply_id,
        );
        let maybe_draft_row = reply_draft::Entity::find_by_id(keys(&draft_scope))
            .lock_exclusive()
            .one(&tx)
            .await?;
        let maybe_binding_row = reply_draft_binding::Entity::find_by_id((
            scope.practice.clone(),
            conversation_id.clone(),
            scope.physician.clone(),
            to_i64(proposal.reply_id, "reply_id")?,
        ))
        .lock_exclusive()
        .one(&tx)
        .await?;
        if let Err(err) =
            lock_proposal_access_in_tx(&tx, &scope, access.session_hash, authority).await
        {
            tx.rollback().await?;
            return Err(err);
        }

        let command = json!({
            "kind": "restore_my_reply_version",
            "practice": &scope.practice,
            "physician": &scope.physician,
            "workspace": &workspace,
            "conversation_id": &conversation_id,
            "reply_id": proposal.reply_id,
            "expected_draft_revision": expected_draft_revision,
        });
        match receipt::check(
            &tx,
            &scope.practice,
            &scope.physician,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            Some(Receipt::Same) => {
                let card = project_reply_card_in_tx(&tx, &conv, &scope.physician)
                    .await?
                    .ok_or_else(|| error("ready reply failed to project card"))?;
                tx.commit().await?;
                return Ok(card);
            }
            Some(Receipt::Different) => {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
            None | Some(Receipt::New) => {}
        }

        let Some(draft_row) = maybe_draft_row else {
            tx.rollback().await?;
            return Err(ReplyProposalError::NoRevisionToRestore);
        };
        let Some(binding_row) = maybe_binding_row else {
            tx.rollback().await?;
            return Err(ReplyProposalError::NoRevisionToRestore);
        };

        let PhysicianDraftStanding::Active {
            draft_revision: current_draft_revision,
            kind:
                PhysicianDraftKind::CodaRevision {
                    pre_revision_your_text,
                },
            ..
        } = decode_draft_binding_standing(&binding_row)?
        else {
            tx.rollback().await?;
            return Err(ReplyProposalError::NoRevisionToRestore);
        };

        let mut draft = Self::draft_control(&tx, &draft_scope, draft_row).await?;
        if current_draft_revision != expected_draft_revision
            || draft.revision != expected_draft_revision
        {
            tx.rollback().await?;
            return Err(ReplyProposalError::StaleDraftVersion {
                detail: format!(
                    "expected Coda-revised draft at revision {expected_draft_revision}, current is {current_draft_revision}"
                ),
            });
        }

        match receipt::record(
            &tx,
            &scope.practice,
            &scope.physician,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            Receipt::New => {}
            Receipt::Same => {
                let card = project_reply_card_in_tx(&tx, &conv, &scope.physician)
                    .await?
                    .ok_or_else(|| error("ready reply failed to project card"))?;
                tx.commit().await?;
                return Ok(card);
            }
            Receipt::Different => {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
        }

        draft.restore_text = Some(std::mem::replace(&mut draft.text, pre_revision_your_text));
        draft.revision += 1;
        if draft.running() {
            draft.cancel()?;
        }
        control::save_revision_attempts(&tx, &draft_scope, &draft.attempts).await?;
        draft_model(&draft_scope, &draft)?.update(&tx).await?;

        let timestamp = now(&tx).await?;
        reply_draft_binding::ActiveModel {
            practice: Unchanged(binding_row.practice),
            conversation_id: Unchanged(binding_row.conversation_id),
            physician: Unchanged(binding_row.physician),
            reply_id: Unchanged(binding_row.reply_id),
            workspace: Unchanged(binding_row.workspace),
            draft_scope_chat: Unchanged(binding_row.draft_scope_chat),
            standing: Set("your_text".to_owned()),
            draft_revision: Set(to_i64(draft.revision, "draft.revision")?),
            based_on_proposal_version: Unchanged(binding_row.based_on_proposal_version),
            pre_revision_your_text: Set(None),
            update_offered_proposal_version: Unchanged(binding_row.update_offered_proposal_version),
            updated_at: Set(timestamp),
        }
        .update(&tx)
        .await?;

        let card = project_reply_card_in_tx(&tx, &conv, &scope.physician)
            .await?
            .ok_or_else(|| error("ready reply failed to project card after restore"))?;
        tx.commit().await?;
        Ok(card)
    }

    /// Adopts Coda's newer reply proposal over the physician's private draft (`YouChooseUpdate`, `AC6`).
    pub async fn choose_coda_reply_update(
        &self,
        access: &WorkspaceAccess<'_>,
        command_key: &CommandKey,
        conversation_id: &str,
        expected_draft_revision: u64,
        expected_proposal_version: u64,
        authority: &mut dyn Authority,
    ) -> Result<ReplyCardView, ReplyProposalError> {
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let scope = verify_physician_access_scope(access)?;
        let tx = self.db.begin().await?;
        let conv = match load_locked_conversation_for_physician(
            &tx,
            &scope,
            access.session_hash,
            &conversation_id,
            authority,
        )
        .await
        {
            Ok(conv) => conv,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        let workspace =
            match verify_conversation_workspace_for_write(&tx, access, &scope, &conv, authority)
                .await
            {
                Ok(workspace) => workspace,
                Err(err) => {
                    tx.rollback().await?;
                    return Err(err);
                }
            };
        let ReplyStanding::Ready { proposal } = &conv.reply_standing else {
            let label = conv.reply_standing.db_label();
            if let Err(err) =
                lock_proposal_access_in_tx(&tx, &scope, access.session_hash, authority).await
            {
                tx.rollback().await?;
                return Err(err);
            }
            tx.rollback().await?;
            return Err(ReplyProposalError::ReplyNotInRequiredState { standing: label });
        };

        let draft_scope = proposal_draft_scope(
            &scope.practice,
            &scope.physician,
            &workspace,
            &conversation_id,
            proposal.reply_id,
        );
        let maybe_draft_row = reply_draft::Entity::find_by_id(keys(&draft_scope))
            .lock_exclusive()
            .one(&tx)
            .await?;
        let maybe_binding_row = reply_draft_binding::Entity::find_by_id((
            scope.practice.clone(),
            conversation_id.clone(),
            scope.physician.clone(),
            to_i64(proposal.reply_id, "reply_id")?,
        ))
        .lock_exclusive()
        .one(&tx)
        .await?;
        if let Err(err) =
            lock_proposal_access_in_tx(&tx, &scope, access.session_hash, authority).await
        {
            tx.rollback().await?;
            return Err(err);
        }

        let command = json!({
            "kind": "choose_coda_reply_update",
            "practice": &scope.practice,
            "physician": &scope.physician,
            "workspace": &workspace,
            "conversation_id": &conversation_id,
            "reply_id": proposal.reply_id,
            "expected_draft_revision": expected_draft_revision,
            "expected_proposal_version": expected_proposal_version,
        });
        match receipt::check(
            &tx,
            &scope.practice,
            &scope.physician,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            Some(Receipt::Same) => {
                let card = project_reply_card_in_tx(&tx, &conv, &scope.physician)
                    .await?
                    .ok_or_else(|| error("ready reply failed to project card"))?;
                tx.commit().await?;
                return Ok(card);
            }
            Some(Receipt::Different) => {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
            None | Some(Receipt::New) => {}
        }

        let Some(binding_row) = maybe_binding_row else {
            tx.rollback().await?;
            return Err(ReplyProposalError::NoUpdateOffered);
        };
        let PhysicianDraftStanding::Active {
            draft_revision: current_draft_revision,
            update_offered_proposal_version: Some(offered_version),
            ..
        } = decode_draft_binding_standing(&binding_row)?
        else {
            tx.rollback().await?;
            return Err(ReplyProposalError::NoUpdateOffered);
        };
        if current_draft_revision != expected_draft_revision
            || proposal.version != expected_proposal_version
            || offered_version != expected_proposal_version
        {
            let current_proposal_version = proposal.version;
            tx.rollback().await?;
            return Err(ReplyProposalError::StaleDraftVersion {
                detail: format!(
                    "expected draft revision {expected_draft_revision} and offered proposal version {expected_proposal_version}, found draft revision {current_draft_revision} and proposal version {current_proposal_version} (offered {offered_version})"
                ),
            });
        }

        match receipt::record(
            &tx,
            &scope.practice,
            &scope.physician,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            Receipt::New => {}
            Receipt::Same => {
                let card = project_reply_card_in_tx(&tx, &conv, &scope.physician)
                    .await?
                    .ok_or_else(|| error("ready reply failed to project card"))?;
                tx.commit().await?;
                return Ok(card);
            }
            Receipt::Different => {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
        }

        if let Some(draft_row) = maybe_draft_row {
            let mut draft = Self::draft_control(&tx, &draft_scope, draft_row).await?;
            if draft.running() {
                draft.cancel()?;
                control::save_revision_attempts(&tx, &draft_scope, &draft.attempts).await?;
                draft_model(&draft_scope, &draft)?.update(&tx).await?;
            }
        }

        let timestamp = now(&tx).await?;
        reply_draft_binding::ActiveModel {
            practice: Unchanged(binding_row.practice),
            conversation_id: Unchanged(binding_row.conversation_id),
            physician: Unchanged(binding_row.physician),
            reply_id: Unchanged(binding_row.reply_id),
            workspace: Unchanged(binding_row.workspace),
            draft_scope_chat: Unchanged(binding_row.draft_scope_chat),
            standing: Set("detached_chose_update".to_owned()),
            draft_revision: Unchanged(binding_row.draft_revision),
            based_on_proposal_version: Set(to_i64(proposal.version, "proposal.version")?),
            pre_revision_your_text: Set(None),
            update_offered_proposal_version: Set(None),
            updated_at: Set(timestamp),
        }
        .update(&tx)
        .await?;

        let card = project_reply_card_in_tx(&tx, &conv, &scope.physician)
            .await?
            .ok_or_else(|| error("ready reply failed to project card after update"))?;
        tx.commit().await?;
        Ok(card)
    }

    /// Records a physician `Copy reply` action (`YouCopy`), capturing the exact text
    /// shown on `Card(d)` without changing any reply, draft, message, or concern state (`Rule 5`, `AC5`).
    pub async fn copy_reply(
        &self,
        access: &WorkspaceAccess<'_>,
        command_key: &CommandKey,
        conversation_id: &str,
        expected: ExpectedDraftBase,
        authority: &mut dyn Authority,
    ) -> Result<CopyReplyOutcome, ReplyProposalError> {
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let scope = verify_physician_access_scope(access)?;
        let tx = self.db.begin().await?;
        let conv = match load_locked_conversation_for_physician(
            &tx,
            &scope,
            access.session_hash,
            &conversation_id,
            authority,
        )
        .await
        {
            Ok(conv) => conv,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        let workspace =
            match check_conversation_access_in_tx(&tx, access, &scope, &conv, authority).await {
                Ok(workspace) => workspace,
                Err(err) => {
                    tx.rollback().await?;
                    return Err(err);
                }
            };

        let command = json!({
            "kind": "copy_reply",
            "practice": &scope.practice,
            "physician": &scope.physician,
            "workspace": &workspace,
            "conversation_id": &conversation_id,
            "expected": expected,
        });
        match receipt::check(
            &tx,
            &scope.practice,
            &scope.physician,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            Some(Receipt::Same) => {
                let existing = reply_copy::Entity::find()
                    .filter(reply_copy::Column::Practice.eq(scope.practice.as_str()))
                    .filter(reply_copy::Column::ConversationId.eq(conversation_id.as_str()))
                    .filter(reply_copy::Column::Physician.eq(scope.physician.as_str()))
                    .filter(reply_copy::Column::CommandKey.eq(command_key.as_str()))
                    .one(&tx)
                    .await?
                    .ok_or_else(|| error("missing reply_copy row for accepted receipt"))?;
                tx.commit().await?;
                return Ok(CopyReplyOutcome::Duplicate(decode_copy_record(existing)?));
            }
            Some(Receipt::Different) => {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
            None | Some(Receipt::New) => {}
        }

        if !matches!(conv.reply_standing, ReplyStanding::Ready { .. }) {
            let label = conv.reply_standing.db_label();
            tx.rollback().await?;
            return Err(ReplyProposalError::ReplyNotInRequiredState { standing: label });
        }
        let card = project_reply_card_in_tx(&tx, &conv, &scope.physician)
            .await?
            .ok_or_else(|| error("ready reply failed to project card for copy"))?;
        let (reply_id, copied_source, copied_text) = match (card.displayed_text, expected) {
            (
                DisplayedReplyText::CodaProposal {
                    reply_id,
                    version,
                    text,
                },
                ExpectedDraftBase::ProposalVersion(expected_v),
            ) => {
                if version != expected_v {
                    tx.rollback().await?;
                    return Err(ReplyProposalError::StaleDraftVersion {
                        detail: format!(
                            "expected proposal version {expected_v} for copy, current is {version}"
                        ),
                    });
                }
                (
                    reply_id,
                    CopiedReplySource::CodaProposal { reply_id, version },
                    text,
                )
            }
            (
                DisplayedReplyText::PhysicianDraft {
                    reply_id,
                    draft_revision,
                    kind,
                    text,
                },
                ExpectedDraftBase::DraftRevision(expected_r),
            ) => {
                if draft_revision != expected_r {
                    tx.rollback().await?;
                    return Err(ReplyProposalError::StaleDraftVersion {
                        detail: format!(
                            "expected draft revision {expected_r} for copy, current is {draft_revision}"
                        ),
                    });
                }
                (
                    reply_id,
                    CopiedReplySource::PhysicianDraft {
                        reply_id,
                        draft_revision,
                        kind,
                    },
                    text,
                )
            }
            (
                DisplayedReplyText::CodaProposal { version, .. },
                ExpectedDraftBase::DraftRevision(expected_r),
            ) => {
                tx.rollback().await?;
                return Err(ReplyProposalError::StaleDraftVersion {
                    detail: format!(
                        "expected draft revision {expected_r} for copy, card displays proposal version {version}"
                    ),
                });
            }
            (
                DisplayedReplyText::PhysicianDraft { draft_revision, .. },
                ExpectedDraftBase::ProposalVersion(expected_v),
            ) => {
                tx.rollback().await?;
                return Err(ReplyProposalError::StaleDraftVersion {
                    detail: format!(
                        "expected proposal version {expected_v} for copy, card displays draft revision {draft_revision}"
                    ),
                });
            }
            (DisplayedReplyText::Withheld, _) => {
                tx.rollback().await?;
                return Err(ReplyProposalError::ReplyNotInRequiredState {
                    standing: "withheld",
                });
            }
        };

        match receipt::record(
            &tx,
            &scope.practice,
            &scope.physician,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            Receipt::New => {}
            Receipt::Same => {
                let existing = reply_copy::Entity::find()
                    .filter(reply_copy::Column::Practice.eq(scope.practice.as_str()))
                    .filter(reply_copy::Column::ConversationId.eq(conversation_id.as_str()))
                    .filter(reply_copy::Column::Physician.eq(scope.physician.as_str()))
                    .filter(reply_copy::Column::CommandKey.eq(command_key.as_str()))
                    .one(&tx)
                    .await?
                    .ok_or_else(|| error("missing reply_copy row for accepted receipt"))?;
                tx.commit().await?;
                return Ok(CopyReplyOutcome::Duplicate(decode_copy_record(existing)?));
            }
            Receipt::Different => {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
        }

        let timestamp = now(&tx).await?;
        let copy_id = format!("copy-{}", Uuid::new_v4());
        let record = ReplyCopyRecord {
            copy_id: copy_id.clone(),
            command_key: command_key.to_string(),
            practice: scope.practice,
            conversation_id,
            workspace,
            physician: scope.physician,
            reply_id,
            practice_observation_seq: conv.last_practice_observation_seq,
            copied_source,
            copied_text,
            copied_at: timestamp,
        };
        reply_copy::Entity::insert(reply_copy::ActiveModel {
            practice: Set(record.practice.clone()),
            conversation_id: Set(record.conversation_id.clone()),
            copy_id: Set(copy_id),
            command_key: Set(record.command_key.clone()),
            physician: Set(record.physician.clone()),
            workspace: Set(record.workspace.clone()),
            reply_id: Set(to_i64(record.reply_id, "reply_id")?),
            practice_observation_seq: Set(to_i64(
                record.practice_observation_seq,
                "practice_observation_seq",
            )?),
            copied_source: Set(serde_json::to_value(&record.copied_source)
                .map_err(conversion_error("CopiedReplySource", "json"))?),
            copied_text: Set(record.copied_text.clone()),
            copied_at: Set(timestamp),
        })
        .exec_without_returning(&tx)
        .await?;
        tx.commit().await?;
        Ok(CopyReplyOutcome::Recorded(record))
    }

    /// Closes a `Ready` reply with `No reply needed` (`YouCloseNoReply`), marking its open
    /// covered messages `NeedsNoReply` and offering `Undo` until `EndUndo` (`Rule 3`, `Rule 12`, `AC5`).
    pub async fn close_reply_no_reply_needed(
        &self,
        access: &WorkspaceAccess<'_>,
        command_key: &CommandKey,
        conversation_id: &str,
        expected_proposal_version: u64,
        authority: &mut dyn Authority,
    ) -> Result<ReplyConversation, ReplyProposalError> {
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let scope = verify_physician_access_scope(access)?;
        let tx = self.db.begin().await?;
        let mut conv = match load_locked_conversation_for_physician(
            &tx,
            &scope,
            access.session_hash,
            &conversation_id,
            authority,
        )
        .await
        {
            Ok(conv) => conv,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        let workspace =
            match check_conversation_access_in_tx(&tx, access, &scope, &conv, authority).await {
                Ok(workspace) => workspace,
                Err(err) => {
                    tx.rollback().await?;
                    return Err(err);
                }
            };

        let command = json!({
            "kind": "close_reply_no_reply_needed",
            "practice": &scope.practice,
            "physician": &scope.physician,
            "workspace": &workspace,
            "conversation_id": &conversation_id,
            "expected_proposal_version": expected_proposal_version,
        });
        match receipt::check(
            &tx,
            &scope.practice,
            &scope.physician,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            Some(Receipt::Same) => {
                tx.commit().await?;
                return Ok(conv);
            }
            Some(Receipt::Different) => {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
            None | Some(Receipt::New) => {}
        }

        let ReplyStanding::Ready { proposal } = conv.reply_standing.clone() else {
            let label = conv.reply_standing.db_label();
            tx.rollback().await?;
            return Err(ReplyProposalError::ReplyNotInRequiredState { standing: label });
        };
        if proposal.version != expected_proposal_version {
            let current_version = proposal.version;
            tx.rollback().await?;
            return Err(ReplyProposalError::StaleDraftVersion {
                detail: format!(
                    "expected proposal version {expected_proposal_version} to close, current is {current_version}"
                ),
            });
        }

        match receipt::record(
            &tx,
            &scope.practice,
            &scope.physician,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            Receipt::New => {}
            Receipt::Same => {
                tx.commit().await?;
                return Ok(conv);
            }
            Receipt::Different => {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
        }

        let timestamp = now(&tx).await?;
        let mut closed_message_ids = BTreeSet::new();
        for msg in &mut conv.patient_messages {
            if proposal.covered_message_ids.contains(&msg.message_id) && msg.standing.is_open() {
                closed_message_ids.insert(msg.message_id.clone());
                msg.standing = PatientMessageStanding::NeedsNoReply {
                    reason: NoReplyReason::PhysicianMarkedNoReplyNeeded {
                        physician: scope.physician.clone(),
                        reply_id: proposal.reply_id,
                        reply_version: proposal.version,
                    },
                    decided_at: timestamp,
                };
            }
        }
        conv.reply_standing = ReplyStanding::ClosedWithUndo {
            proposal,
            closed_message_ids,
            closed_by: scope.physician,
            closed_at: timestamp,
        };
        conv.updated_at = timestamp;
        encode_conversation_model(&conv)?.update(&tx).await?;
        tx.commit().await?;
        Ok(conv)
    }

    /// Undoes `No reply needed` (`YouUndoClose`) while `ClosedWithUndo` is still active,
    /// bringing back the exact `Ready` reply and reopening its closed messages (`Rule 12`, `AC5`).
    pub async fn undo_close_no_reply_needed(
        &self,
        access: &WorkspaceAccess<'_>,
        command_key: &CommandKey,
        conversation_id: &str,
        authority: &mut dyn Authority,
    ) -> Result<ReplyCardView, ReplyProposalError> {
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let scope = verify_physician_access_scope(access)?;
        let tx = self.db.begin().await?;
        let mut conv = match load_locked_conversation_for_physician(
            &tx,
            &scope,
            access.session_hash,
            &conversation_id,
            authority,
        )
        .await
        {
            Ok(conv) => conv,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        let workspace =
            match check_conversation_access_in_tx(&tx, access, &scope, &conv, authority).await {
                Ok(workspace) => workspace,
                Err(err) => {
                    tx.rollback().await?;
                    return Err(err);
                }
            };

        let command = json!({
            "kind": "undo_close_no_reply_needed",
            "practice": &scope.practice,
            "physician": &scope.physician,
            "workspace": &workspace,
            "conversation_id": &conversation_id,
        });
        match receipt::check(
            &tx,
            &scope.practice,
            &scope.physician,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            Some(Receipt::Same) => {
                if let Some(card) = project_reply_card_in_tx(&tx, &conv, &scope.physician).await? {
                    tx.commit().await?;
                    return Ok(card);
                }
                tx.rollback().await?;
                return Err(ReplyProposalError::UndoNoLongerAvailable { conversation_id });
            }
            Some(Receipt::Different) => {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
            None | Some(Receipt::New) => {}
        }

        let ReplyStanding::ClosedWithUndo {
            proposal,
            closed_message_ids,
            ..
        } = conv.reply_standing.clone()
        else {
            tx.rollback().await?;
            return Err(ReplyProposalError::UndoNoLongerAvailable { conversation_id });
        };

        match receipt::record(
            &tx,
            &scope.practice,
            &scope.physician,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            Receipt::New => {}
            Receipt::Same => {
                if let Some(card) = project_reply_card_in_tx(&tx, &conv, &scope.physician).await? {
                    tx.commit().await?;
                    return Ok(card);
                }
                tx.rollback().await?;
                return Err(ReplyProposalError::UndoNoLongerAvailable { conversation_id });
            }
            Receipt::Different => {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
        }

        for msg in &mut conv.patient_messages {
            if closed_message_ids.contains(&msg.message_id)
                && matches!(msg.standing, PatientMessageStanding::NeedsNoReply { .. })
            {
                msg.standing = PatientMessageStanding::Open;
            }
        }
        conv.reply_standing = ReplyStanding::Ready { proposal };
        conv.updated_at = now(&tx).await?;
        encode_conversation_model(&conv)?.update(&tx).await?;
        let card = project_reply_card_in_tx(&tx, &conv, &scope.physician)
            .await?
            .ok_or_else(|| error("undone reply failed to project card"))?;
        tx.commit().await?;
        Ok(card)
    }

    /// Settles an open `No reply needed` undo window (`UndoGoesAway`).
    pub async fn expire_no_reply_undo(
        &self,
        practice: &str,
        conversation_id: &str,
    ) -> Result<ReplyConversation, ReplyProposalError> {
        let practice = validate_identifier("practice", practice)?;
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let tx = self.db.begin().await?;
        let mut conv = load_locked_conversation(&tx, &practice, &conversation_id).await?;
        if matches!(conv.reply_standing, ReplyStanding::ClosedWithUndo { .. }) {
            end_closed_undo(&mut conv.reply_standing);
            conv.updated_at = now(&tx).await?;
            encode_conversation_model(&conv)?.update(&tx).await?;
        }
        tx.commit().await?;
        Ok(conv)
    }

    /// Marks an open (`Ready` or `Withheld`) reply `Done` while the messaging channel
    /// is disconnected (`YouMarkDone`, `Rule 11`, `AC4`).
    pub async fn mark_disconnected_reply_done(
        &self,
        access: &WorkspaceAccess<'_>,
        command_key: &CommandKey,
        conversation_id: &str,
        expected: ExpectedDisconnectedCard,
        authority: &mut dyn Authority,
    ) -> Result<ReplyConversation, ReplyProposalError> {
        let conversation_id = validate_identifier("conversation_id", conversation_id)?;
        let scope = verify_physician_access_scope(access)?;
        let tx = self.db.begin().await?;
        let mut conv = match load_locked_conversation_for_physician(
            &tx,
            &scope,
            access.session_hash,
            &conversation_id,
            authority,
        )
        .await
        {
            Ok(conv) => conv,
            Err(err) => {
                tx.rollback().await?;
                return Err(err);
            }
        };
        let workspace =
            match check_conversation_access_in_tx(&tx, access, &scope, &conv, authority).await {
                Ok(workspace) => workspace,
                Err(err) => {
                    tx.rollback().await?;
                    return Err(err);
                }
            };

        let command = json!({
            "kind": "mark_disconnected_reply_done",
            "practice": &scope.practice,
            "physician": &scope.physician,
            "workspace": &workspace,
            "conversation_id": &conversation_id,
            "expected": expected,
        });
        match receipt::check(
            &tx,
            &scope.practice,
            &scope.physician,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            Some(Receipt::Same) => {
                tx.commit().await?;
                return Ok(conv);
            }
            Some(Receipt::Different) => {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
            None | Some(Receipt::New) => {}
        }

        if matches!(conv.channel, ChannelConnection::Connected) {
            tx.rollback().await?;
            return Err(ReplyProposalError::ChannelStillConnected { conversation_id });
        }
        let covered_ids = match (&conv.reply_standing, expected) {
            (
                ReplyStanding::Ready { proposal },
                ExpectedDisconnectedCard::Ready {
                    reply_id,
                    proposal_version,
                },
            ) => {
                if proposal.reply_id != reply_id || proposal.version != proposal_version {
                    let actual_id = proposal.reply_id;
                    let actual_version = proposal.version;
                    tx.rollback().await?;
                    return Err(ReplyProposalError::StaleDraftVersion {
                        detail: format!(
                            "expected ready reply {reply_id} v{proposal_version}, current is ready reply {actual_id} v{actual_version}"
                        ),
                    });
                }
                proposal.covered_message_ids.clone()
            }
            (
                ReplyStanding::Withheld {
                    reply_id: actual_id,
                    covered_message_ids,
                    ..
                },
                ExpectedDisconnectedCard::Withheld { reply_id },
            ) => {
                if *actual_id != reply_id {
                    tx.rollback().await?;
                    return Err(ReplyProposalError::StaleDraftVersion {
                        detail: format!(
                            "expected withheld reply {reply_id}, current is withheld reply {actual_id}"
                        ),
                    });
                }
                covered_message_ids.clone()
            }
            (
                ReplyStanding::Ready { proposal },
                ExpectedDisconnectedCard::Withheld { reply_id },
            ) => {
                let actual_id = proposal.reply_id;
                let actual_version = proposal.version;
                tx.rollback().await?;
                return Err(ReplyProposalError::StaleDraftVersion {
                    detail: format!(
                        "expected withheld reply {reply_id}, current is ready reply {actual_id} v{actual_version}"
                    ),
                });
            }
            (
                ReplyStanding::Withheld {
                    reply_id: actual_id,
                    ..
                },
                ExpectedDisconnectedCard::Ready {
                    reply_id,
                    proposal_version,
                },
            ) => {
                tx.rollback().await?;
                return Err(ReplyProposalError::StaleDraftVersion {
                    detail: format!(
                        "expected ready reply {reply_id} v{proposal_version}, current is withheld reply {actual_id}"
                    ),
                });
            }
            (ReplyStanding::None | ReplyStanding::ClosedWithUndo { .. }, _) => {
                let label = conv.reply_standing.db_label();
                tx.rollback().await?;
                return Err(ReplyProposalError::ReplyNotInRequiredState { standing: label });
            }
        };

        match receipt::record(
            &tx,
            &scope.practice,
            &scope.physician,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            Receipt::New => {}
            Receipt::Same => {
                tx.commit().await?;
                return Ok(conv);
            }
            Receipt::Different => {
                tx.rollback().await?;
                return Err(ReplyProposalError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
        }

        let timestamp = now(&tx).await?;
        for msg in &mut conv.patient_messages {
            if covered_ids.contains(&msg.message_id) && msg.standing.is_open() {
                msg.standing = PatientMessageStanding::MarkedDoneWhileDisconnected {
                    physician: scope.physician.clone(),
                    marked_at: timestamp,
                };
            }
        }
        conv.reply_standing = ReplyStanding::None;
        conv.updated_at = timestamp;
        encode_conversation_model(&conv)?.update(&tx).await?;
        tx.commit().await?;
        Ok(conv)
    }
}
