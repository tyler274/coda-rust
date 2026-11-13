//! Linked patient message review and physician-requested reply revision recipes.
//!
//! Owns the background assignment, model settings (`ModelRoles::background`),
//! starting context, resident system prompts, eagerly preloaded skills
//! (`patient-communication`, `patient-record-retrieval`), discoverable workspace
//! capabilities, completion safety and currency barriers, and durable run
//! provenance for linked patient messaging conversations, alongside the
//! physician-requested reply draft revision recipe within the revision
//! operation's authority.

use std::collections::{BTreeMap, BTreeSet};

use async_trait::async_trait;
use harness::engine::recipe::{Finish, Provider, RequestBinding, RequestRecipe};
use harness::engine::{
    self, AttemptFailure, Event, EventKind, Invocation, Item, Product, Tool, ToolOutcome, Turn,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::ModelRoles;
use crate::attachments::{READ_DOCUMENT_PAGE_TOOL, document_page_tool, is_document_page_tool};
use crate::clinical_library::is_library_tool;
use crate::initial_understanding::{
    SAVE_UNDERSTANDING_TOOL, is_understanding_tool, understanding_tools,
};
use crate::patient_reviews::{
    READ_PATIENT_REVIEWS_TOOL, ReadReviewsCall, read_only_review_tool, read_patient_reviews_tool,
};
use crate::physician_chat::{PhysicianSkills, SKILL_VIEW, read_only as chat_read_only, work_tools};
use crate::physician_sources::{
    ChartScope, READ_TOOL, SEARCH_TOOL, SourcePatient, SourceRevision, read_sources, read_tool,
    search_tool, unread_result,
};
use crate::runtime::chat_operations::{
    DispatchCheckStanding, MemoryMutation, MemoryProvenance, MemorySelection, OpenConcernSummary,
    PATIENT_MEMORY_TOOL, PatientMemoryError, PatientMemoryId, PatientMemoryStanding,
    PatientStartingContext, PriorWorkKind, PriorWorkPointer, RETRIEVE_PATIENT_WORK_TOOL,
    RecordMemoryCorrection, RetirePatientMemory, SourceFreshnessSummary, patient_memory_tool,
    retrieve_patient_work_tool,
};
use crate::runtime::concerns::{
    ConcernError, ConcernId, ConcernUrgency, ExactRecordReference, SourceWork, concern_tool,
    is_concern_tool,
};
use crate::runtime::delivery::AttemptSettlement;
use crate::runtime::follow_ups::{
    FollowUp, FollowUpError, FollowUpStanding, WorkspaceWithdrawalBoundary, follow_up_tool,
    is_follow_up_tool,
};
use crate::runtime::jobs::{
    AssignmentError, PracticeAccessError, PracticeAuthority, PracticeScope, is_physician_only_tool,
};
use crate::runtime::refresh::{REFRESH_TOOL, SavedSourceStatus, is_refresh_tool, refresh_tool};
use crate::runtime::reply::proposals::{
    AnsweredMessageClosure, CompleteReplyReview, ConversationLink, LinkedPatientMessage,
    MessageSender, ObservedPracticeMessage, PROPOSAL_DRAFT_CHAT_PREFIX, PracticeMessageKind,
    ReplyProposalBasisInput, ReplyProposalError, ReplyProposalInput, ReplyReviewDecision,
    ReplyReviewOutcome, ReplyReviewSnapshot, ReplyStanding, ReviewReadState, SavedReviewCompletion,
};
use crate::runtime::reply::{
    RevisionClaim, RevisionCommit, RevisionContext, requests_finish, revised_text,
    revision_feedback, tools as reply_revision_runtime_tools,
};
use crate::runtime::spruce_matching::{
    SpruceAttachmentSummary, execute_spruce_working_note, is_spruce_working_note_tool,
    spruce_conversion_error as conversion_error, spruce_db_error as error,
    spruce_working_note_references,
};
use crate::runtime::understanding::PatientUnderstanding;
use crate::runtime::{
    Authority, BindingRef, Scope, SkillPackageId, Store, WorkStatus, persistence_failure,
};

/// Immutable recipe identifier recorded with every linked message review run.
pub const MESSAGE_REVIEW_RECIPE_VERSION: &str = "message-review/v1";

/// Immutable resident guidance version recorded with every linked message review run.
pub const MESSAGE_REVIEW_GUIDANCE_VERSION: &str = "message-review-guidance/v1";

/// Structured completion barrier tool submitting Coda's message review outcome.
pub const COMPLETE_MESSAGE_REVIEW_TOOL: &str = "complete_message_review";

/// Skills eagerly preloaded at the start of every linked patient message review.
pub const PRELOADED_MESSAGE_REVIEW_SKILLS: [&str; 2] =
    ["patient-communication", "patient-record-retrieval"];

/// Immutable recipe identifier recorded with every physician-requested reply revision run.
pub const REPLY_REVISION_RECIPE_VERSION: &str = "physician-reply-revision/v1";

/// Immutable resident guidance version recorded with every physician-requested reply revision run.
pub const REPLY_REVISION_GUIDANCE_VERSION: &str = "physician-reply-revision-guidance/v1";

/// Skills eagerly preloaded at the start of every physician-requested reply revision run.
pub const PRELOADED_REPLY_REVISION_SKILLS: [&str; 1] = ["patient-communication"];

const MESSAGE_REVIEW_RUN_ACTOR: &str = "__message_review_run__";
const REPLY_REVISION_RUN_ACTOR: &str = "__reply_revision_run__";
const MAX_SUBMISSION_TEXT_CHARS: usize = 8_000;

/// Resident system instructions for linked patient message review.
///
/// States the clinical job, possible dispositions, safety boundaries, and
/// grounding discipline without prescribing a rigid investigation sequence.
pub const MESSAGE_REVIEW_INSTRUCTIONS: &str = "\
You are Coda reviewing a linked patient messaging conversation for a practice in Chords. \
No physician is signed in to this background run, and the practice sends all patient replies \
directly in Spruce; you propose replies, surface clinical concerns, and keep durable patient \
state current, and you never send messages, place orders, or write back to external systems.

Own the clinical review of the full conversation and any newly arrived chart records coalesced \
with it:

1. Understand the full conversation and its participants:
   - Read every open patient-side message (whether written by the patient or by a named caregiver \
     or third party on the thread) alongside every observed practice message in chronological order.
   - Distinguish human practice messages (`practice_sent`) from automatic responder notices \
     (`automatic`). When a human practice message has already answered one or more open patient \
     messages, record those exact closures in `answered_closures` citing that `practice_message_id`. \
     An automatic practice reply never answers a patient message. When a practice message answers \
     only part of the thread, close only the answered messages and address the remaining open \
     messages.
   - Copying a reply never sends it; rely only on observed `practice_sent` messages as sent by \
     the practice. Closing a reply never closes an open clinical concern or active follow-up.

2. Investigate what the clinical question requires:
   - Use the starting context to orient to patient identity, chart freshness, saved understanding, \
     active `patient_memory`, active follow-ups, and open concerns, then open exact chart records \
     with `search_patient_information`, `read_patient_sources` (including attached photos or \
     images), and `read_document_page` (for multi-page faxes, PDFs, or scanned reports) whenever \
     patient-specific facts matter.
   - Whenever an open message asks about a current medication, dose, refill, interaction, or \
     whether to start, hold, or stop a medication, call `refresh_patient_sources` first and read \
     the current medication records before proposing a reply; if a source refresh fails or is \
     unavailable, state that freshness limit explicitly in `limitations`.
   - Keep unreadable photos, corrupted attachments, or unextracted scan pages explicitly \
     `unreadable` in `limitations`; never guess their contents or treat unreadable pages as normal.
   - Additional skills in the catalog (such as `patient-follow-up` or `visit-preparation`) and \
     clinical reference passages (`search_clinical_library`, `read_clinical_library`, queried \
     with clinical concepts only and zero patient identifiers) remain available on demand when \
     helpful.

3. Keep durable patient context accurate without disturbing unrelated state:
   - When the patient's message or accompanying records establish a durable clinical update worth \
     carrying forward, record or update `patient_memory` or an existing `patient_follow_up` \
     through its tool while leaving unrelated memory entries and unrelated follow-ups untouched. \
     Leave both unchanged when nothing durable changed.
   - Judge clinical urgency across the conversation and chart:
     * Routine questions that need only a proposed reply require no `patient_concern`.
     * Meaningful nonurgent clinical issues warranting physician awareness today belong in \
       `patient_concern` with `today` urgency alongside an appropriate proposed reply or \
       no-reply disposition.
     * Potentially acute, life-threatening, or time-critical symptoms or findings warrant \
       `patient_concern` with `now` urgency AND a `propose_reply` draft that clearly directs \
       the patient or sender to seek immediate emergency evaluation (calling 911 or going to \
       the nearest emergency department right away) rather than waiting on clinic messaging.

4. Conclude through `complete_message_review`:
   - Choose `propose_reply` when a reply helps answer remaining open messages, or whenever a \
     physician explicitly asked Coda to draft a reply (`saw_physician_reply_request: true`). \
     Write warm, plain, accurate patient-facing prose ready for the physician to review and send \
     in Spruce, accompanied by a concise `basis_summary`, any explicit `limitations`, and exact \
     read `cited_records`.
   - Choose `no_reply_would_help` with a clear `reason` when remaining open messages need no \
     further reply (for example a brief acknowledgment or thank-you with no clinical question).
   - Choose `all_handled` when `answered_closures` (or prior handling) leaves zero open patient \
     messages and no physician reply request is pending.";

/// Resident system instructions for physician-requested reply draft revision.
///
/// Governs revisions within the draft-revision operation's authority: honoring
/// the physician's current draft and instruction, verifying chart claims, and
/// reconsidering affected content when chart or conversation sources move.
pub const REPLY_REVISION_INSTRUCTIONS: &str = "\
You are Coda revising a physician's patient-facing reply draft at the physician's direction. \
Work strictly within the draft-revision operation's authority: revise the reply text for the \
physician to review and copy into Spruce, and never imply that a message has been sent, an order \
placed, or durable patient memory, concerns, or follow-ups altered.

Follow these principles:
1. Start from the physician's `saved_draft` and honor their `instruction` directly while \
   preserving their clinical judgment, tone, and unedited commitments unless chart evidence or \
   the instruction calls for changing them.
2. Verify patient-specific clinical facts, dates, laboratory values, medications, or paged \
   documents with `read_patient_sources` or `read_document_page` before stating them in the \
   revised draft. Keep unreadable or missing records explicit rather than guessing.
3. If completion returns `sources_changed` or `sources_unread` because new messages or corrected \
   chart records arrived while you were revising, read the changed or unread records, reconsider \
   every affected statement in the draft, and submit the updated revision through \
   `finish_reply_revision`.
4. Submit the complete revised patient-facing reply text exclusively by calling \
   `finish_reply_revision` as your final action.";

mod tables {
    pub(super) mod runtime_command_record {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_commands")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub physician: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub key: String,
            #[sea_orm(column_type = "JsonBinary")]
            pub command: Json,
            pub accepted_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    pub(super) mod workspace_concern_record {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_concerns")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub workspace: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub id: String,
            #[sea_orm(column_type = "Text")]
            pub urgency: String,
            #[sea_orm(column_type = "Text")]
            pub status: String,
            #[sea_orm(column_type = "Text")]
            pub reason: String,
            #[sea_orm(column_type = "JsonBinary")]
            pub records: Json,
            #[sea_orm(column_type = "JsonBinary")]
            pub source_work: Json,
            #[sea_orm(column_type = "Text")]
            pub cleared_by_physician: Option<String>,
            pub cleared_at: Option<i64>,
            pub feedback_revision: Option<i64>,
            #[sea_orm(column_type = "Text")]
            pub retirement_reason: Option<String>,
            #[sea_orm(column_type = "JsonBinary")]
            pub retired_by_work: Option<Json>,
            pub retired_at: Option<i64>,
            pub created_at: i64,
            pub updated_at: i64,
            pub revision: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    pub(super) mod source_withdrawal_record {
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

    pub(super) mod understanding_withdrawal_record {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_understanding_withdrawals")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub workspace: String,
            #[sea_orm(primary_key, auto_increment = false)]
            pub revision: i64,
            #[sea_orm(column_type = "Text")]
            pub withdrawal_id: String,
            pub withdrawal_epoch: i64,
            #[sea_orm(column_type = "Text")]
            pub reason: String,
            pub withdrawn_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    pub(super) mod practice_job_record {
        use crate::runtime::WorkStatus;
        use crate::runtime::jobs::PracticeJobKind;
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "physician_runtime_practice_jobs")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub practice: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub workspace: String,
            #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
            pub job_id: String,
            pub kind: PracticeJobKind,
            pub status: WorkStatus,
            #[sea_orm(column_type = "Text")]
            pub work: String,
            pub revision: i64,
            #[sea_orm(column_type = "JsonBinary")]
            pub assignment: Json,
            #[sea_orm(column_type = "JsonBinary")]
            pub preloaded_skills: Json,
            #[sea_orm(column_type = "Text")]
            pub skills: String,
            #[sea_orm(column_type = "JsonBinary")]
            pub binding: Json,
            #[sea_orm(column_type = "JsonBinary")]
            pub runs: Json,
            pub failures: i64,
            pub allowance_after: i64,
            pub sources_version: i64,
            #[sea_orm(column_type = "JsonBinary")]
            pub sources: Json,
            #[sea_orm(column_type = "JsonBinary")]
            pub result: Option<Json>,
            pub dispatch_after: i64,
            pub created_at: i64,
            pub updated_at: i64,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }
}

use tables::{
    practice_job_record, runtime_command_record, source_withdrawal_record,
    understanding_withdrawal_record, workspace_concern_record,
};

/// Digest and name of one skill preloaded into a recipe's resident instructions.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreloadedSkillDigest {
    /// Approved catalog skill name.
    pub name: String,
    /// Lowercase SHA-256 hex digest of the preloaded `SKILL.md` content.
    pub sha256: String,
}

/// Exact recipe, guidance, model-lane settings, and preloaded skill digests recorded on a message review run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageReviewProvenance {
    /// Immutable recipe identifier (`MESSAGE_REVIEW_RECIPE_VERSION`).
    pub recipe_version: String,
    /// Immutable resident guidance version (`MESSAGE_REVIEW_GUIDANCE_VERSION`).
    pub guidance_version: String,
    /// Lowercase SHA-256 hex digest of [`MESSAGE_REVIEW_INSTRUCTIONS`].
    pub guidance_sha256: String,
    /// Model identifier from `ModelRoles::background`.
    pub model: String,
    /// Reasoning effort from `ModelRoles::background`.
    pub effort: String,
    /// Context window token cap from `ModelRoles::background`.
    pub context_window_tokens: u64,
    /// Per-call input token cap from `ModelRoles::background`.
    pub max_input_tokens: u64,
    /// Provider adapter label from `ModelRoles::background`.
    pub adapter: String,
    /// Frozen provider request binding stored in `physician_runtime_recipes`.
    pub binding: BindingRef,
    /// Frozen skill catalog package stored in `physician_runtime_skill_packages`.
    pub skills: SkillPackageId,
    /// Eagerly preloaded skills embedded at run start.
    pub preloaded_skills: Vec<PreloadedSkillDigest>,
}

/// Context item eagerly loaded at the start of a message review run, distinct from later tool discovery.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MessageReviewInitialLoad {
    /// Resident message-review system guidance.
    ResidentGuidance {
        /// Guidance version identifier.
        version: String,
        /// Lowercase SHA-256 hex digest of the resident guidance text.
        sha256: String,
    },
    /// Approved skill preloaded in full before the first model call.
    PreloadedSkill {
        /// Skill catalog name.
        name: String,
        /// Lowercase SHA-256 hex digest of the skill body.
        sha256: String,
    },
    /// Metadata-only catalog of remaining discoverable skills (`skill_view`).
    SkillCatalogMetadata {
        /// Frozen skill package identifier.
        skills_package_id: SkillPackageId,
        /// Skill names left discoverable via `skill_view` rather than preloaded.
        discoverable_skill_names: Vec<String>,
    },
    /// Linked messaging conversation snapshot read via `Store::start_or_read_reply_review`.
    LinkedConversationSnapshot {
        /// Conversation identifier.
        conversation_id: String,
        /// Pinned conversation version.
        conversation_version: u64,
        /// Count of open patient-side messages on the thread.
        open_patient_message_count: usize,
        /// Count of observed practice messages on the thread.
        practice_message_count: usize,
        /// Whether a physician `Ask Coda to reply` request was pending.
        saw_physician_reply_request: bool,
    },
    /// Patient workspace orientation snapshot loaded at review start.
    PatientChartSnapshot {
        /// Patient workspace identifier.
        workspace: String,
        /// Pinned workspace chart source version.
        source_version: u64,
        /// Active retained chart record count.
        chart_record_count: usize,
        /// Active durable patient memory entry count.
        active_memory_count: usize,
        /// Active patient follow-up count.
        active_follow_up_count: usize,
        /// Open patient concern count.
        open_concern_count: usize,
    },
    /// Newly arrived or corrected chart candidate IDs sharing one coalesced review with the message.
    CoalescedRecordChanges {
        /// Candidate record IDs coalesced into this review.
        changed_candidates: Vec<String>,
    },
}

/// Settled result classification for an on-demand tool discovery recorded during a run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum ToolDiscoveryDisposition {
    /// Tool returned structured content, history, or an image/document view.
    Returned,
    /// Tool call was refused by validation or authority boundaries.
    Refused {
        /// Refusal explanation returned to the model.
        reason: String,
    },
}

/// One on-demand tool call or `skill_view` load executed after the initial prompt,
/// kept distinct from [`MessageReviewInitialLoad`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedToolDiscovery {
    /// Harness event ordinal where `ToolReturned` settled.
    pub ordinal: u64,
    /// Provider tool call identifier.
    pub call_id: String,
    /// Invoked tool name (and loaded skill name when `tool_name == "skill_view"`).
    pub tool_name: String,
    /// Skill name requested when `tool_name == "skill_view"`, if present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovered_skill_name: Option<String>,
    /// Whether the call returned data or was refused.
    pub disposition: ToolDiscoveryDisposition,
    /// Database-clock Unix second when recorded.
    pub recorded_at: i64,
}

/// Attachment or inbound fax descriptor associated with a linked conversation message.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageAttachmentDescriptor {
    /// Linked channel message identifier carrying this attachment, when tied to a specific message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Retained chart `candidate_id` or `document_id` when ingested into workspace sources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub candidate_id: Option<String>,
    /// Attachment metadata from Spruce channel intake, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_attachment: Option<SpruceAttachmentSummary>,
    /// Media MIME type or record type label.
    pub content_type: String,
    /// Total page count when ingested as a paged PDF/fax document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_count: Option<u32>,
}

/// Starting context assembled for one linked patient message review run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageReviewStartingContext {
    /// Practice tenant identifier.
    pub practice: String,
    /// Linked patient workspace identifier.
    pub workspace: String,
    /// Messaging conversation identifier.
    pub conversation_id: String,
    /// Active conversation review identifier.
    pub review_id: String,
    /// Background run identifier.
    pub run_id: String,
    /// Conversation version pinned at review start.
    pub conversation_version: u64,
    /// Workspace chart source version pinned at review start.
    pub source_version: u64,
    /// Conversation move epoch pinned at review start.
    pub move_epoch: u64,
    /// Ordered linked patient-side messages on the conversation.
    pub patient_messages: Vec<LinkedPatientMessage>,
    /// Ordered observed practice messages on the conversation (`practice_sent` and `automatic`).
    pub practice_messages: Vec<ObservedPracticeMessage>,
    /// Attachment and fax descriptors associated with this conversation or workspace.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<MessageAttachmentDescriptor>,
    /// Current conversation reply standing prior to this review's decision.
    pub current_reply_standing: ReplyStanding,
    /// Whether an explicit physician `Ask Coda to reply` request is pending.
    pub saw_physician_reply_request: bool,
    /// Chart candidate IDs coalesced into this same review when a message and lab/result arrived together.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coalesced_changed_candidates: Vec<String>,
    /// Patient demographic snapshot from connected sources, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patient: Option<SourcePatient>,
    /// Count of active retained chart records in the workspace.
    pub chart_record_count: usize,
    /// Per-source successful pull timestamps.
    #[serde(default)]
    pub source_pulls: Vec<SourceFreshnessSummary>,
    /// Saved per-source refresh statuses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub refresh_statuses: Vec<SavedSourceStatus>,
    /// Dispatcher source-check standing for this workspace.
    pub check_standing: DispatchCheckStanding,
    /// Saved patient understanding, when initialized.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub understanding: Option<PatientUnderstanding>,
    /// Active workspace wrong-patient withdrawal boundary, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withdrawal_boundary: Option<WorkspaceWithdrawalBoundary>,
    /// Standing of durable patient memory in this workspace.
    pub memory_standing: PatientMemoryStanding,
    /// Active follow-up responsibilities in this workspace.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub active_follow_ups: Vec<FollowUp>,
    /// Open clinical concerns currently active for this patient.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub open_concerns: Vec<OpenConcernSummary>,
}

impl MessageReviewStartingContext {
    /// Renders model-facing starting context with zero internal implementation labels or ticket IDs.
    #[must_use]
    pub fn prompt_text(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "Linked patient message review context (conversation {}, workspace {}, conversation version {}, chart version {})",
            self.conversation_id, self.workspace, self.conversation_version, self.source_version
        ));

        match &self.patient {
            Some(patient) => {
                let dob = patient.birth_date.as_deref().unwrap_or("not recorded");
                let mrns = if patient.medical_record_numbers.is_empty() {
                    "not recorded".to_owned()
                } else {
                    patient
                        .medical_record_numbers
                        .iter()
                        .map(|mrn| format!("{}: {}", mrn.system, mrn.value))
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                lines.push(format!(
                    "Patient identity: {} | birth date: {} | MRN: {}",
                    patient.display_name, dob, mrns
                ));
            }
            None => lines.push(
                "Patient identity: no source demographic pull recorded yet for this workspace."
                    .to_owned(),
            ),
        }

        if self.saw_physician_reply_request {
            lines.push(
                "Physician request: an authorized physician explicitly clicked Ask Coda to reply on this conversation; you must propose a helpful patient-facing reply (`propose_reply`)."
                    .to_owned(),
            );
        }

        if !self.coalesced_changed_candidates.is_empty() {
            lines.push(format!(
                "Coalesced chart updates arriving with this message review (shared review): {}",
                self.coalesced_changed_candidates.join(", ")
            ));
        }

        lines.push("Conversation patient-side messages (chronological):".to_owned());
        for msg in &self.patient_messages {
            let sender_label = match &msg.sender {
                MessageSender::Patient => "patient".to_owned(),
                MessageSender::NamedSender { name } => format!("sender `{name}`"),
            };
            let standing_label = match &msg.standing {
                crate::runtime::reply::proposals::PatientMessageStanding::Open => {
                    "open".to_owned()
                }
                crate::runtime::reply::proposals::PatientMessageStanding::Answered {
                    practice_message_id,
                    ..
                } => format!("already answered by practice message `{practice_message_id}`"),
                crate::runtime::reply::proposals::PatientMessageStanding::NeedsNoReply {
                    ..
                } => "previously marked needs no reply".to_owned(),
                crate::runtime::reply::proposals::PatientMessageStanding::MarkedDoneWhileDisconnected {
                    ..
                } => "marked done while channel disconnected".to_owned(),
                crate::runtime::reply::proposals::PatientMessageStanding::MovedToNotPatient {
                    ..
                } => "retired on move to not-a-patient".to_owned(),
            };
            lines.push(format!(
                "- [{}] ({sender_label}, unix {}, {standing_label}): {}",
                msg.message_id, msg.sent_at, msg.text
            ));
        }

        if self.practice_messages.is_empty() {
            lines.push("Observed practice messages on channel: none.".to_owned());
        } else {
            lines.push("Observed practice messages on channel (chronological):".to_owned());
            for pm in &self.practice_messages {
                let kind_label = match pm.kind {
                    PracticeMessageKind::PracticeSent => {
                        "practice_sent (eligible to close answered patient messages)"
                    }
                    PracticeMessageKind::Automatic => {
                        "automatic (auto-responder; never closes patient messages as answered)"
                    }
                };
                lines.push(format!(
                    "- [{}] ({kind_label}, unix {}): {}",
                    pm.practice_message_id, pm.sent_at, pm.text
                ));
            }
        }

        if !self.attachments.is_empty() {
            lines.push("Conversation & chart attachments / inbound documents:".to_owned());
            for att in &self.attachments {
                let msg_part = att
                    .message_id
                    .as_deref()
                    .map(|id| format!("message `{id}`"))
                    .unwrap_or_else(|| "workspace document".to_owned());
                let cand_part = att
                    .candidate_id
                    .as_deref()
                    .map(|id| format!("candidate_id/document_id `{id}`"))
                    .unwrap_or_else(|| "not ingested as chart candidate".to_owned());
                let pages_part = att
                    .page_count
                    .map(|p| format!(", {p} page(s)"))
                    .unwrap_or_default();
                lines.push(format!(
                    "- {msg_part}: {cand_part}, type `{}`{pages_part}",
                    att.content_type
                ));
            }
        }

        let pulls_summary = if self.source_pulls.is_empty() {
            "no completed source pulls recorded".to_owned()
        } else {
            self.source_pulls
                .iter()
                .map(|pull| {
                    format!(
                        "{} ({}) as of unix {}",
                        pull.source_label, pull.source_id, pull.pulled_at
                    )
                })
                .collect::<Vec<_>>()
                .join("; ")
        };
        lines.push(format!(
            "Retained chart: version {}, {} active record(s); source pulls: {}",
            self.source_version, self.chart_record_count, pulls_summary
        ));

        if let Some(saved) = &self.understanding {
            lines.push(format!(
                "Saved patient summary (revision {}, source version {}): {}",
                saved.revision, saved.source_version, saved.summary.narrative
            ));
        }

        match &self.memory_standing {
            PatientMemoryStanding::Available { entries } if entries.is_empty() => {
                lines.push("Active patient memory: none.".to_owned());
            }
            PatientMemoryStanding::Available { entries } => {
                lines.push("Active patient memory:".to_owned());
                for entry in entries {
                    lines.push(format!(
                        "- [{}] {}: {}",
                        entry.id, entry.topic, entry.statement
                    ));
                }
            }
            PatientMemoryStanding::Unavailable { reason } => {
                lines.push(format!(
                    "Patient memory limit (unavailable): durable memory could not be read ({reason})."
                ));
            }
        }

        if self.active_follow_ups.is_empty() {
            lines.push("Active patient follow-ups: none.".to_owned());
        } else {
            lines.push("Active patient follow-ups:".to_owned());
            for item in &self.active_follow_ups {
                lines.push(format!(
                    "- [{}] due {}: {}",
                    item.id,
                    item.due_at,
                    item.summary.as_str()
                ));
            }
        }

        if self.open_concerns.is_empty() {
            lines.push("Open patient concerns: none.".to_owned());
        } else {
            lines.push("Open patient concerns:".to_owned());
            for concern in &self.open_concerns {
                let urgency = match concern.urgency {
                    ConcernUrgency::Now => "now",
                    ConcernUrgency::Today => "today",
                };
                lines.push(format!(
                    "- [{}] ({urgency}): {}",
                    concern.concern_id, concern.reason
                ));
            }
        }

        lines.join("\n")
    }
}

/// Optional additional inputs supplied when starting a linked message review run.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MessageReviewOptions {
    /// Optional explicit review identifier (generated deterministically or via UUID when omitted).
    pub review_id: Option<String>,
    /// Newly arrived or corrected chart candidate IDs coalesced into this message review.
    pub coalesced_changed_candidates: Vec<String>,
    /// Additional attachment descriptors to surface in starting context.
    pub extra_attachments: Vec<MessageAttachmentDescriptor>,
}

/// Closed decision submitted to [`COMPLETE_MESSAGE_REVIEW_TOOL`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SubmittedMessageReviewDecision {
    /// Proposes a patient-facing reply draft for the physician to review and send in Spruce.
    ProposeReply {
        /// Proposed patient-facing reply prose.
        reply_text: String,
        /// Concise clinical and operational basis summary.
        basis_summary: String,
        /// Explicit missing, unreadable, or unavailable evidence limits.
        limitations: Vec<String>,
        /// Exact chart record revisions relied on by the reply and basis.
        cited_records: Vec<ExactRecordReference>,
        /// Whether the reply answers a medication question requiring a fresh chart check first.
        addresses_medication_question: bool,
    },
    /// Concludes that remaining open patient messages need no reply.
    NoReplyWouldHelp {
        /// Physician-readable clinical/operational explanation why no reply is needed.
        reason: String,
        /// Concise basis summary.
        basis_summary: String,
        /// Explicit evidence limits, if any.
        limitations: Vec<String>,
    },
    /// Concludes with zero open patient messages remaining after `answered_closures` or prior handling.
    AllHandled {
        /// Concise summary of how all open messages were handled.
        basis_summary: String,
    },
}

/// Validated submission staged by [`COMPLETE_MESSAGE_REVIEW_TOOL`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StagedMessageReviewSubmission {
    /// Tool call identifier that staged this submission.
    pub call_id: String,
    /// Practice-sent message closures for answered patient messages.
    pub answered_closures: Vec<AnsweredMessageClosure>,
    /// Structured disposition for remaining open messages.
    pub decision: SubmittedMessageReviewDecision,
    /// Concern IDs raised or attached during this review.
    pub raised_concern_ids: Vec<ConcernId>,
}

/// Lifecycle status of a persisted message review or reply revision recipe run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageReviewRunStatus {
    /// Attempt is actively executing.
    Running,
    /// Run committed its review or revision outcome.
    Completed,
    /// Run stopped cleanly at a run-limit backstop without committing.
    Held,
    /// Run failed before committing.
    Failed,
}

/// Durable PostgreSQL record of one linked patient message review execution.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageReviewRunRecord {
    /// Practice tenant identifier.
    pub practice: String,
    /// Linked patient workspace identifier.
    pub workspace: String,
    /// Messaging conversation identifier.
    pub conversation_id: String,
    /// Conversation review identifier.
    pub review_id: String,
    /// Unique run identifier.
    pub run_id: String,
    /// Attempt identifier.
    pub attempt_id: String,
    /// Current lifecycle status.
    pub status: MessageReviewRunStatus,
    /// Exact recipe, guidance, model-lane settings, and preloaded skill digests.
    pub provenance: MessageReviewProvenance,
    /// Assembled starting context passed to the run.
    pub starting_context: MessageReviewStartingContext,
    /// Context and skills eagerly loaded at run start.
    pub initial_loads: Vec<MessageReviewInitialLoad>,
    /// Later on-demand tool and skill discoveries recorded during execution.
    pub tool_discoveries: Vec<RecordedToolDiscovery>,
    /// Ordered harness execution events.
    pub events: Vec<Event>,
    /// Latest conversation version read by this run (updated on mid-run re-read).
    pub last_read_conversation_version: u64,
    /// Latest workspace source version pinned by this run (updated after `refresh_patient_sources`).
    pub last_pinned_source_version: u64,
    /// Validated submission staged by `complete_message_review`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staged_submission: Option<StagedMessageReviewSubmission>,
    /// Durable `SavedReviewCompletion` once committed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saved_completion: Option<SavedReviewCompletion>,
    /// Final model summary text, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub final_summary: Option<String>,
    /// Database-clock Unix second when started.
    pub started_at: i64,
    /// Database-clock Unix second when settled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
}

/// Result returned when a linked patient message review run completes and commits.
#[derive(Clone, Debug)]
pub struct CompletedMessageReviewRun {
    /// Persisted run record including provenance, initial loads, tool discoveries, and events.
    pub run: MessageReviewRunRecord,
    /// Committed reply review outcome from `Store::complete_reply_review`.
    pub outcome: ReplyReviewOutcome,
}

/// Provenance recorded on a physician-requested reply draft revision run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyRevisionProvenance {
    /// Immutable revision recipe version (`REPLY_REVISION_RECIPE_VERSION`).
    pub recipe_version: String,
    /// Immutable revision guidance version (`REPLY_REVISION_GUIDANCE_VERSION`).
    pub guidance_version: String,
    /// Lowercase SHA-256 hex digest of [`REPLY_REVISION_INSTRUCTIONS`].
    pub guidance_sha256: String,
    /// Model identifier from `ModelRoles::background`.
    pub model: String,
    /// Reasoning effort from `ModelRoles::background`.
    pub effort: String,
    /// Context window token cap.
    pub context_window_tokens: u64,
    /// Per-call input token cap.
    pub max_input_tokens: u64,
    /// Provider adapter label.
    pub adapter: String,
    /// Preloaded skill digests (`patient-communication`).
    pub preloaded_skills: Vec<PreloadedSkillDigest>,
}

/// Durable record of a physician-requested reply draft revision run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplyRevisionRunRecord {
    /// Practice tenant identifier.
    pub practice: String,
    /// Authoring physician identifier.
    pub physician: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Draft scope chat key (`reply-proposal:{conversation_id}:{reply_id}` or chat key).
    pub draft_scope_chat: String,
    /// Claimed revision attempt identifier (`run_id`).
    pub attempt_id: String,
    /// Revision command key.
    pub input_key: String,
    /// Physician's base draft text entering the revision.
    pub base_draft_text: String,
    /// Physician's revision instruction.
    pub instruction: String,
    /// Initial workspace source version pinned at claim time.
    pub initial_source_version: u64,
    /// Final workspace source version verified at commit time.
    pub committed_source_version: Option<u64>,
    /// Number of `SourcesMoved` / `SourcesUnread` reconsiderations handled before commit.
    pub source_reconsiderations: u32,
    /// Recipe and skill provenance.
    pub provenance: ReplyRevisionProvenance,
    /// Later tool discoveries (`read_patient_sources`, `read_document_page`, `skill_view`, `finish_reply_revision`).
    pub tool_discoveries: Vec<RecordedToolDiscovery>,
    /// Revised reply text committed to the physician's draft, once completed.
    pub revised_text: Option<String>,
    /// Run status (`Running`, `Completed`, `Held`, `Failed`).
    pub status: MessageReviewRunStatus,
    /// Database-clock Unix second when started.
    pub started_at: i64,
    /// Database-clock Unix second when settled.
    pub completed_at: Option<i64>,
}

/// Errors produced when building or executing a message review or physician reply revision run.
#[derive(Debug, thiserror::Error)]
pub enum MessageReviewRecipeError {
    /// Practice patient access check failed.
    #[error("{0}")]
    PracticeAccess(#[from] crate::runtime::jobs::PracticeAccessError),
    /// Skill preloading failed.
    #[error("{0}")]
    Assignment(#[from] AssignmentError),
    /// Conversation or reply proposal store operation failed.
    #[error("{0}")]
    ReplyProposal(#[from] ReplyProposalError),
    /// Runtime draft or persistence error.
    #[error("{0}")]
    Runtime(#[from] crate::runtime::RuntimeError),
    /// Underlying concern lookup failed.
    #[error("{0}")]
    Concern(#[from] ConcernError),
    /// Underlying follow-up lookup failed.
    #[error("{0}")]
    FollowUp(#[from] FollowUpError),
    /// Source refresh status read failed.
    #[error("{0}")]
    Refresh(#[from] crate::runtime::refresh::RefreshError),
    /// Database operation failed.
    #[error("{0}")]
    Database(#[from] DbErr),
    /// Revision delivery had no claimable active revision (already superseded or cancelled).
    #[error("reply revision delivery `{input_id}` on `{chat}` is no longer claimable")]
    RevisionNotClaimable {
        /// Draft scope chat identifier.
        chat: String,
        /// Revision input key.
        input_id: String,
    },
    /// Claimed revision binding did not match the built `reply_revision_recipe`.
    #[error(
        "reply revision delivery `{input_id}` on `{chat}` carries a frozen recipe that does not match `reply_revision_recipe`"
    )]
    RevisionRecipeMismatch {
        /// Draft scope chat identifier.
        chat: String,
        /// Revision input key.
        input_id: String,
    },
    /// Harness execution failed.
    #[error("message review harness run `{run_id}` failed: {source}")]
    Harness {
        /// Conversation review identifier, when executing a linked message review.
        review_id: Option<String>,
        /// Run identifier.
        run_id: String,
        /// Underlying harness failure.
        #[source]
        source: engine::Failure,
    },
    /// Run stopped at a limit backstop without committing a review decision.
    #[error(
        "message review run `{run_id}` (review `{review_id}`) held without committing `complete_message_review`"
    )]
    HeldWithoutDecision {
        /// Conversation review identifier.
        review_id: String,
        /// Run identifier.
        run_id: String,
    },
}

/// Returns the lowercase SHA-256 hex digest of [`MESSAGE_REVIEW_INSTRUCTIONS`].
#[must_use]
pub fn message_review_instructions_sha256() -> String {
    crate::sha256_hex(MESSAGE_REVIEW_INSTRUCTIONS.as_bytes())
}

/// Returns the lowercase SHA-256 hex digest of [`REPLY_REVISION_INSTRUCTIONS`].
#[must_use]
pub fn reply_revision_instructions_sha256() -> String {
    crate::sha256_hex(REPLY_REVISION_INSTRUCTIONS.as_bytes())
}

fn load_preloaded_skills(
    skills: &PhysicianSkills,
    names: &[&str],
) -> Result<(Vec<PreloadedSkillDigest>, Vec<Value>), AssignmentError> {
    let serialized =
        serde_json::to_value(skills).map_err(|err| AssignmentError::UnavailableSkill {
            name: "catalog".to_owned(),
            reason: err.to_string(),
        })?;
    let mut digests = Vec::with_capacity(names.len());
    let mut payloads = Vec::with_capacity(names.len());
    for &name in names {
        let content = extract_skill_body(&serialized, name).ok_or_else(|| {
            AssignmentError::UnavailableSkill {
                name: name.to_owned(),
                reason: "skill not found in PhysicianSkills catalog".to_owned(),
            }
        })?;
        let sha256 = crate::sha256_hex(content.as_bytes());
        digests.push(PreloadedSkillDigest {
            name: name.to_owned(),
            sha256: sha256.clone(),
        });
        payloads.push(json!({
            "name": name,
            "sha256": sha256,
            "content": content,
        }));
    }
    Ok((digests, payloads))
}

fn extract_skill_body(package_json: &Value, target_name: &str) -> Option<String> {
    if let Some(entries) = package_json.get("skills").and_then(Value::as_array) {
        for entry in entries {
            if entry.get("name").and_then(Value::as_str) == Some(target_name)
                && let Some(body) = entry
                    .get("body")
                    .or_else(|| entry.get("content"))
                    .and_then(Value::as_str)
            {
                return Some(body.to_owned());
            }
        }
    }
    if let Some(map) = package_json.get("skills").and_then(Value::as_object)
        && let Some(entry) = map.get(target_name)
        && let Some(body) = entry
            .get("body")
            .or_else(|| entry.get("content"))
            .and_then(Value::as_str)
    {
        return Some(body.to_owned());
    }
    // Fall back to executing `skill_view` against the live package so the exact
    // validated `SKILL.md` body is always retrieved through its owner.
    None
}

pub(crate) fn preload_skills_from_catalog(
    skills: &PhysicianSkills,
    names: &[&str],
) -> Result<(Vec<PreloadedSkillDigest>, Vec<Value>), AssignmentError> {
    if let Ok(found) = load_preloaded_skills(skills, names) {
        return Ok(found);
    }
    let mut digests = Vec::with_capacity(names.len());
    let mut payloads = Vec::with_capacity(names.len());
    for &name in names {
        let outcome = skills.execute(SKILL_VIEW, &json!({ "name": name }));
        let ToolOutcome::Returned(raw) = outcome else {
            return Err(AssignmentError::UnavailableSkill {
                name: name.to_owned(),
                reason: "skill_view refused skill load".to_owned(),
            });
        };
        let parsed: Value =
            serde_json::from_str(&raw).map_err(|err| AssignmentError::UnavailableSkill {
                name: name.to_owned(),
                reason: err.to_string(),
            })?;
        let content = parsed
            .get("body")
            .or_else(|| parsed.get("content"))
            .or_else(|| parsed.get("markdown"))
            .and_then(Value::as_str)
            .unwrap_or(&raw)
            .to_owned();
        let sha256 = crate::sha256_hex(content.as_bytes());
        digests.push(PreloadedSkillDigest {
            name: name.to_owned(),
            sha256: sha256.clone(),
        });
        payloads.push(json!({
            "name": name,
            "sha256": sha256,
            "content": content,
        }));
    }
    Ok((digests, payloads))
}

pub(crate) fn discoverable_skill_names(
    skills: &PhysicianSkills,
    preloaded: &[&str],
) -> Vec<String> {
    let preloaded_set: BTreeSet<&str> = preloaded.iter().copied().collect();
    let known = [
        "patient-communication",
        "patient-follow-up",
        "patient-record-retrieval",
        "visit-preparation",
    ];
    let mut out = Vec::new();
    for candidate in known {
        if preloaded_set.contains(candidate) {
            continue;
        }
        if matches!(
            skills.execute(SKILL_VIEW, &json!({ "name": candidate })),
            ToolOutcome::Returned(_)
        ) {
            out.push(candidate.to_owned());
        }
    }
    out
}

pub(crate) fn deduplicate_and_verify_unique_tools(tools: Vec<Value>) -> Vec<Value> {
    let mut seen = BTreeSet::new();
    let mut unique = Vec::with_capacity(tools.len());
    for tool in tools {
        let name = tool
            .get("name")
            .and_then(Value::as_str)
            .expect("every recipe tool definition must carry a string `name`")
            .to_owned();
        assert!(
            seen.insert(name.clone()),
            "duplicate tool `{name}` in background recipe"
        );
        unique.push(tool);
    }
    unique
}

/// Structured completion tool schema offered to linked patient message review runs.
#[must_use]
pub fn complete_message_review_tool() -> Tool {
    let citation_schema = json!({
        "type": "object",
        "properties": {
            "candidate_id": { "type": "string", "minLength": 1, "maxLength": 256 },
            "revision": { "type": "string", "minLength": 1, "maxLength": 256 }
        },
        "required": ["candidate_id", "revision"],
        "additionalProperties": false
    });
    let closure_schema = json!({
        "type": "object",
        "properties": {
            "practice_message_id": { "type": "string", "minLength": 1, "maxLength": 256 },
            "answered_message_ids": {
                "type": "array",
                "items": { "type": "string", "minLength": 1, "maxLength": 256 },
                "minItems": 1
            }
        },
        "required": ["practice_message_id", "answered_message_ids"],
        "additionalProperties": false
    });
    let decision_schema = json!({
        "anyOf": [
            {
                "type": "object",
                "properties": {
                    "kind": { "type": "string", "enum": ["propose_reply"] },
                    "reply_text": { "type": "string", "minLength": 1, "maxLength": MAX_SUBMISSION_TEXT_CHARS },
                    "basis_summary": { "type": "string", "minLength": 1, "maxLength": MAX_SUBMISSION_TEXT_CHARS },
                    "limitations": { "type": "array", "items": { "type": "string", "minLength": 1 } },
                    "cited_records": { "type": "array", "items": citation_schema },
                    "addresses_medication_question": { "type": "boolean" }
                },
                "required": [
                    "kind",
                    "reply_text",
                    "basis_summary",
                    "limitations",
                    "cited_records",
                    "addresses_medication_question"
                ],
                "additionalProperties": false
            },
            {
                "type": "object",
                "properties": {
                    "kind": { "type": "string", "enum": ["no_reply_would_help"] },
                    "reason": { "type": "string", "minLength": 1, "maxLength": MAX_SUBMISSION_TEXT_CHARS },
                    "basis_summary": { "type": "string", "minLength": 1, "maxLength": MAX_SUBMISSION_TEXT_CHARS },
                    "limitations": { "type": "array", "items": { "type": "string", "minLength": 1 } }
                },
                "required": ["kind", "reason", "basis_summary", "limitations"],
                "additionalProperties": false
            },
            {
                "type": "object",
                "properties": {
                    "kind": { "type": "string", "enum": ["all_handled"] },
                    "basis_summary": { "type": "string", "minLength": 1, "maxLength": MAX_SUBMISSION_TEXT_CHARS }
                },
                "required": ["kind", "basis_summary"],
                "additionalProperties": false
            }
        ]
    });

    Tool {
        name: COMPLETE_MESSAGE_REVIEW_TOOL.into(),
        description: "Submit the structured disposition for this linked patient message review. Record any `answered_closures` citing non-automatic `practice_sent` messages that answered open patient messages, supply `decision` (`propose_reply`, `no_reply_would_help`, or `all_handled`), and list any `raised_concern_ids` created or revised during this review.".into(),
        parameters: json!({
            "type": "object",
            "properties": {
                "answered_closures": {
                    "type": "array",
                    "items": closure_schema
                },
                "decision": decision_schema,
                "raised_concern_ids": {
                    "type": "array",
                    "items": { "type": "string", "minLength": 1, "maxLength": 256 }
                }
            },
            "required": ["answered_closures", "decision", "raised_concern_ids"],
            "additionalProperties": false
        }),
    }
}

/// Capabilities offered to a linked patient message review run.
#[must_use]
pub fn tools() -> Vec<Tool> {
    let mut offered = work_tools();
    offered.push(search_tool());
    offered.push(read_tool(ChartScope::Chat));
    offered.push(document_page_tool());
    offered.push(refresh_tool());
    offered.push(patient_memory_tool());
    offered.push(retrieve_patient_work_tool());
    offered.push(read_patient_reviews_tool());
    offered.push(concern_tool());
    offered.push(follow_up_tool());
    offered.extend(understanding_tools());
    offered.extend(crate::clinical_library::tools());
    offered.push(complete_message_review_tool());
    offered
}

/// Builds the immutable [`RequestRecipe`] for linked patient message review on
/// `roles.background`, eagerly preloading `patient-communication` and
/// `patient-record-retrieval` while keeping remaining skills and capabilities discoverable.
pub fn recipe(
    roles: &ModelRoles,
    skills: &PhysicianSkills,
) -> Result<(RequestRecipe, Vec<PreloadedSkillDigest>), AssignmentError> {
    let background = &roles.background;
    let (digests, payloads) =
        preload_skills_from_catalog(skills, &PRELOADED_MESSAGE_REVIEW_SKILLS)?;
    let mut instructions = skills.instructions(MESSAGE_REVIEW_INSTRUCTIONS);
    let encoded = serde_json::to_string(&json!({
        "kind": "preloaded_assignment_skills",
        "skills": payloads,
    }))
    .expect("preloaded skills serialize as JSON");
    instructions.push_str(
        "\n\nPreloaded assignment skills (already open in this run; do not reload via skill_view):\n",
    );
    instructions.push_str(&encoded);

    let mut built = RequestRecipe {
        model: background.model.clone(),
        context_window_tokens: background.context_window_tokens,
        max_input_tokens: background.max_input_tokens,
        options: json!({ "effort": background.effort }),
        instructions,
        tools: tools().into_iter().map(|tool| tool.function()).collect(),
    };
    engine::offer_working_notes(&mut built.instructions, &mut built.tools);
    built.tools = deduplicate_and_verify_unique_tools(built.tools);
    Ok((built, digests))
}

/// Builds the immutable [`RequestRecipe`] for physician-requested reply draft
/// revisions on `roles.background` within the revision operation's authority.
pub fn reply_revision_recipe(
    roles: &ModelRoles,
    skills: &PhysicianSkills,
) -> Result<(RequestRecipe, Vec<PreloadedSkillDigest>), AssignmentError> {
    let background = &roles.background;
    let (digests, payloads) =
        preload_skills_from_catalog(skills, &PRELOADED_REPLY_REVISION_SKILLS)?;
    let mut instructions = skills.instructions(REPLY_REVISION_INSTRUCTIONS);
    let encoded = serde_json::to_string(&json!({
        "kind": "preloaded_assignment_skills",
        "skills": payloads,
    }))
    .expect("preloaded skills serialize as JSON");
    instructions.push_str(
        "\n\nPreloaded assignment skills (already open in this run; do not reload via skill_view):\n",
    );
    instructions.push_str(&encoded);

    let mut tool_defs = reply_revision_runtime_tools();
    tool_defs.push(document_page_tool().function());
    let mut built = RequestRecipe {
        model: background.model.clone(),
        context_window_tokens: background.context_window_tokens,
        max_input_tokens: background.max_input_tokens,
        options: json!({ "effort": background.effort }),
        instructions,
        tools: tool_defs,
    };
    engine::offer_working_notes(&mut built.instructions, &mut built.tools);
    built.tools = deduplicate_and_verify_unique_tools(built.tools);
    Ok((built, digests))
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCompleteMessageReviewArgs {
    answered_closures: Vec<AnsweredMessageClosure>,
    decision: SubmittedMessageReviewDecision,
    raised_concern_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
enum RawMemoryToolCall {
    Read {
        #[serde(default)]
        include_retired: bool,
    },
    RecordCorrection {
        memory_id: PatientMemoryId,
        topic: String,
        statement: String,
    },
    Retire {
        memory_id: PatientMemoryId,
        reason: String,
    },
}

fn validate_non_empty_prose(field: &'static str, text: &str) -> Result<String, String> {
    let trimmed = text.trim();
    let count = trimmed.chars().count();
    if !(1..=MAX_SUBMISSION_TEXT_CHARS).contains(&count)
        || trimmed
            .chars()
            .any(|ch| ch.is_control() && ch != '\n' && ch != '\t')
    {
        return Err(format!(
            "`{field}` must be non-empty prose (up to {MAX_SUBMISSION_TEXT_CHARS} characters) with no control characters"
        ));
    }
    Ok(trimmed.to_owned())
}

fn parse_submission(
    call_id: &str,
    arguments: &Value,
) -> Result<StagedMessageReviewSubmission, String> {
    let raw: RawCompleteMessageReviewArgs = serde_json::from_value(arguments.clone())
        .map_err(|err| format!("invalid `{COMPLETE_MESSAGE_REVIEW_TOOL}` arguments: {err}"))?;

    let mut raised_concern_ids = Vec::with_capacity(raw.raised_concern_ids.len());
    let mut seen_concerns = BTreeSet::new();
    for raw_id in raw.raised_concern_ids {
        let id = ConcernId::new(raw_id)
            .map_err(|err| format!("invalid `raised_concern_ids` entry: {err}"))?;
        if seen_concerns.insert(id.clone()) {
            raised_concern_ids.push(id);
        }
    }

    let decision = match raw.decision {
        SubmittedMessageReviewDecision::ProposeReply {
            reply_text,
            basis_summary,
            limitations,
            cited_records,
            addresses_medication_question,
        } => {
            let reply_text = validate_non_empty_prose("reply_text", &reply_text)?;
            let basis_summary = validate_non_empty_prose("basis_summary", &basis_summary)?;
            let mut clean_limits = Vec::with_capacity(limitations.len());
            for lim in limitations {
                clean_limits.push(validate_non_empty_prose("limitations", &lim)?);
            }
            let mut clean_citations = Vec::with_capacity(cited_records.len());
            let mut seen_refs = BTreeSet::new();
            for item in cited_records {
                let validated = ExactRecordReference::new(item.candidate_id, item.revision)
                    .map_err(|err| format!("invalid `cited_records` reference: {err}"))?;
                if seen_refs.insert((validated.candidate_id.clone(), validated.revision.clone())) {
                    clean_citations.push(validated);
                }
            }
            SubmittedMessageReviewDecision::ProposeReply {
                reply_text,
                basis_summary,
                limitations: clean_limits,
                cited_records: clean_citations,
                addresses_medication_question,
            }
        }
        SubmittedMessageReviewDecision::NoReplyWouldHelp {
            reason,
            basis_summary,
            limitations,
        } => {
            let reason = validate_non_empty_prose("reason", &reason)?;
            let basis_summary = validate_non_empty_prose("basis_summary", &basis_summary)?;
            let mut clean_limits = Vec::with_capacity(limitations.len());
            for lim in limitations {
                clean_limits.push(validate_non_empty_prose("limitations", &lim)?);
            }
            SubmittedMessageReviewDecision::NoReplyWouldHelp {
                reason,
                basis_summary,
                limitations: clean_limits,
            }
        }
        SubmittedMessageReviewDecision::AllHandled { basis_summary } => {
            SubmittedMessageReviewDecision::AllHandled {
                basis_summary: validate_non_empty_prose("basis_summary", &basis_summary)?,
            }
        }
    };

    Ok(StagedMessageReviewSubmission {
        call_id: call_id.to_owned(),
        answered_closures: raw.answered_closures,
        decision,
        raised_concern_ids,
    })
}

#[derive(Default)]
struct RunReadAndRefreshEvidence {
    read_revisions: BTreeSet<(String, String)>,
    refresh_completed_ordinal: Option<u64>,
    refresh_had_failure: bool,
    chart_read_after_refresh: bool,
    raised_concern_ids_in_events: BTreeSet<ConcernId>,
}

fn collect_run_evidence(
    events: &[Event],
    current_sources: &[SourceRevision],
) -> RunReadAndRefreshEvidence {
    let mut call_meta: BTreeMap<String, (String, Value)> = BTreeMap::new();
    for event in events {
        match &event.kind {
            EventKind::ToolStarted(call) => {
                let args = serde_json::from_str::<Value>(&call.arguments).unwrap_or(Value::Null);
                call_meta.insert(call.id.clone(), (call.name.clone(), args));
            }
            EventKind::ProviderReturned(turn) => {
                for call in turn.calls() {
                    let args =
                        serde_json::from_str::<Value>(&call.arguments).unwrap_or(Value::Null);
                    call_meta
                        .entry(call.id.clone())
                        .or_insert((call.name.clone(), args));
                }
            }
            _ => {}
        }
    }

    let mut summary = RunReadAndRefreshEvidence::default();
    for event in events {
        let EventKind::ToolReturned { call_id, result } = &event.kind else {
            continue;
        };
        let Some((tool_name, args)) = call_meta.get(call_id) else {
            continue;
        };

        if tool_name == REFRESH_TOOL {
            if let engine::results::ToolResult::Original(raw) = result
                && let Ok(val) = serde_json::from_str::<Value>(raw)
            {
                summary.refresh_completed_ordinal = Some(event.ordinal);
                if val
                    .get("sources")
                    .and_then(Value::as_array)
                    .is_some_and(|arr| {
                        arr.iter().any(|s| {
                            matches!(
                                s.get("status").and_then(Value::as_str),
                                Some("failed" | "partly_updated")
                            )
                        })
                    })
                    || val.get("outcome").and_then(Value::as_str) == Some("failed")
                {
                    summary.refresh_had_failure = true;
                }
            }
            continue;
        }

        if tool_name == READ_TOOL {
            match result {
                engine::results::ToolResult::Original(raw) => {
                    if let Ok(val) = serde_json::from_str::<Value>(raw)
                        && let Some(results) = val.get("results").and_then(Value::as_array)
                    {
                        let mut any_read = false;
                        for item in results {
                            let outcome = item.get("outcome").and_then(Value::as_str);
                            if matches!(outcome, Some("available" | "passage"))
                                && let (Some(cid), Some(rev)) = (
                                    item.get("reference").and_then(Value::as_str),
                                    item.get("revision").and_then(Value::as_str),
                                )
                            {
                                summary
                                    .read_revisions
                                    .insert((cid.to_owned(), rev.to_owned()));
                                any_read = true;
                            }
                        }
                        if any_read
                            && summary
                                .refresh_completed_ordinal
                                .is_some_and(|ord| event.ordinal > ord)
                        {
                            summary.chart_read_after_refresh = true;
                        }
                    }
                }
                engine::results::ToolResult::View(_) => {
                    if let Some(refs) = args
                        .get("references")
                        .or_else(|| args.get("candidates"))
                        .and_then(Value::as_array)
                    {
                        for entry in refs {
                            let cand_opt = entry
                                .as_str()
                                .or_else(|| entry.get("candidate_id").and_then(Value::as_str));
                            let rev_opt = entry.get("revision").and_then(Value::as_str);
                            if let Some(cand) = cand_opt
                                && let Some(src) = current_sources.iter().find(|s| {
                                    s.candidate_id == cand
                                        && rev_opt.is_none_or(|r| r == s.revision)
                                })
                            {
                                summary
                                    .read_revisions
                                    .insert((src.candidate_id.clone(), src.revision.clone()));
                            }
                        }
                    }
                    if summary
                        .refresh_completed_ordinal
                        .is_some_and(|ord| event.ordinal > ord)
                    {
                        summary.chart_read_after_refresh = true;
                    }
                }
                engine::results::ToolResult::Refused(_)
                | engine::results::ToolResult::History(_) => {}
            }
            continue;
        }

        if tool_name == READ_DOCUMENT_PAGE_TOOL {
            if let engine::results::ToolResult::Original(raw) = result
                && let Ok(val) = serde_json::from_str::<Value>(raw)
                && matches!(
                    val.get("outcome").and_then(Value::as_str),
                    Some("available" | "failed_page")
                )
                && let Some(doc_id) = val.get("document_id").and_then(Value::as_str)
            {
                let rev_opt = val.get("revision").and_then(Value::as_str);
                for src in current_sources {
                    let matches_doc = src.candidate_id == doc_id
                        || src.provenance.get("document_id").and_then(Value::as_str)
                            == Some(doc_id);
                    if matches_doc && rev_opt.is_none_or(|r| r == src.revision) {
                        summary
                            .read_revisions
                            .insert((src.candidate_id.clone(), src.revision.clone()));
                    }
                }
                if summary
                    .refresh_completed_ordinal
                    .is_some_and(|ord| event.ordinal > ord)
                {
                    summary.chart_read_after_refresh = true;
                }
            }
            continue;
        }

        if is_concern_tool(tool_name)
            && let engine::results::ToolResult::Original(raw) = result
            && let Ok(val) = serde_json::from_str::<Value>(raw)
            && val.get("saved").and_then(Value::as_bool) == Some(true)
            && let Some(id_str) = val
                .get("concern")
                .and_then(|c| c.get("id"))
                .and_then(Value::as_str)
            && let Ok(cid) = ConcernId::new(id_str)
        {
            summary.raised_concern_ids_in_events.insert(cid);
        }
    }

    summary
}

fn directs_immediate_emergency_care(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("911")
        || lower.contains("emergency department")
        || lower.contains("emergency room")
        || lower.contains("nearest emergency")
        || lower.contains("immediate emergency")
        || lower.contains("seek emergency")
}

fn decode_message_review_run(
    row: runtime_command_record::Model,
) -> Result<MessageReviewRunRecord, DbErr> {
    serde_json::from_value(row.command).map_err(conversion_error(
        "runtime_command_record.command",
        "MessageReviewRunRecord",
    ))
}

fn decode_reply_revision_run(
    row: runtime_command_record::Model,
) -> Result<ReplyRevisionRunRecord, DbErr> {
    serde_json::from_value(row.command).map_err(conversion_error(
        "runtime_command_record.command",
        "ReplyRevisionRunRecord",
    ))
}

impl Store {
    /// Reads one persisted linked patient message review run record by `(practice, run_id)`.
    pub async fn message_review_run(
        &self,
        practice: &str,
        run_id: &str,
    ) -> Result<Option<MessageReviewRunRecord>, DbErr> {
        let practice = practice.trim();
        let run_id = run_id.trim();
        if practice.is_empty() || run_id.is_empty() {
            return Err(error(
                "message review run lookup requires non-empty practice and run_id",
            ));
        }
        runtime_command_record::Entity::find_by_id((
            practice.to_owned(),
            MESSAGE_REVIEW_RUN_ACTOR.to_owned(),
            run_id.to_owned(),
        ))
        .one(self.spruce_db())
        .await?
        .map(decode_message_review_run)
        .transpose()
    }

    /// Lists all persisted linked patient message review runs for `(practice, conversation_id)`
    /// in chronological order.
    pub async fn message_review_runs_for_conversation(
        &self,
        practice: &str,
        conversation_id: &str,
    ) -> Result<Vec<MessageReviewRunRecord>, DbErr> {
        let practice = practice.trim();
        let conversation_id = conversation_id.trim();
        if practice.is_empty() || conversation_id.is_empty() {
            return Err(error(
                "message review run listing requires non-empty practice and conversation_id",
            ));
        }
        let rows = runtime_command_record::Entity::find()
            .filter(runtime_command_record::Column::Practice.eq(practice))
            .filter(runtime_command_record::Column::Physician.eq(MESSAGE_REVIEW_RUN_ACTOR))
            .order_by_asc(runtime_command_record::Column::AcceptedAt)
            .order_by_asc(runtime_command_record::Column::Key)
            .all(self.spruce_db())
            .await?;
        let mut out = Vec::new();
        for row in rows {
            let decoded = decode_message_review_run(row)?;
            if decoded.conversation_id == conversation_id {
                out.push(decoded);
            }
        }
        Ok(out)
    }

    /// Reads one persisted physician-requested reply revision run record by `(practice, attempt_id)`.
    pub async fn reply_revision_run(
        &self,
        practice: &str,
        attempt_id: &str,
    ) -> Result<Option<ReplyRevisionRunRecord>, DbErr> {
        let practice = practice.trim();
        let attempt_id = attempt_id.trim();
        if practice.is_empty() || attempt_id.is_empty() {
            return Err(error(
                "reply revision run lookup requires non-empty practice and attempt_id",
            ));
        }
        runtime_command_record::Entity::find_by_id((
            practice.to_owned(),
            REPLY_REVISION_RUN_ACTOR.to_owned(),
            attempt_id.to_owned(),
        ))
        .one(self.spruce_db())
        .await?
        .map(decode_reply_revision_run)
        .transpose()
    }

    async fn persist_message_review_event(
        &self,
        practice: &str,
        run_id: &str,
        attempt_id: &str,
        event: Event,
    ) -> Result<(), engine::ProductError> {
        if event.attempt_id != attempt_id {
            return Err(error("message review event belongs to another attempt").into());
        }
        if event.sequence == 0 {
            return Err(error("invalid message review event sequence").into());
        }
        let clock = self.spruce_now().await.map_err(persistence_failure)?;
        let tx = self
            .spruce_db()
            .begin()
            .await
            .map_err(persistence_failure)?;
        let row = runtime_command_record::Entity::find_by_id((
            practice.to_owned(),
            MESSAGE_REVIEW_RUN_ACTOR.to_owned(),
            run_id.to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await
        .map_err(persistence_failure)?
        .ok_or_else(|| error("message review run missing"))?;
        let mut run = decode_message_review_run(row.clone())?;
        let is_current =
            run.status == MessageReviewRunStatus::Running && run.attempt_id == attempt_id;
        if !is_current && !event.kind.survives_loss_of_currency() {
            let reason = if run.attempt_id != attempt_id {
                engine::NotCurrent::Lost
            } else {
                engine::NotCurrent::Withdrawn
            };
            return Err(engine::ProductError::NotCurrent(reason));
        }

        if let Some(saved) = run
            .events
            .iter()
            .find(|prior| prior.attempt_id == event.attempt_id && prior.sequence == event.sequence)
        {
            let saved_json =
                serde_json::to_value(&saved.kind).map_err(conversion_error("EventKind", "json"))?;
            let incoming_json =
                serde_json::to_value(&event.kind).map_err(conversion_error("EventKind", "json"))?;
            if saved.ordinal != event.ordinal || saved_json != incoming_json {
                return Err(error("message review event collision").into());
            }
            tx.commit().await?;
            return Ok(());
        }

        let last_seq = run
            .events
            .iter()
            .filter(|prior| prior.attempt_id == event.attempt_id)
            .map(|prior| prior.sequence)
            .max();
        if event.sequence != last_seq.map_or(1, |seq| seq + 1) {
            return Err(error("message review event sequence gap").into());
        }

        if let EventKind::ToolReturned { call_id, result } = &event.kind {
            let (tool_name, discovered_skill_name) = run
                .events
                .iter()
                .rev()
                .find_map(|prior| match &prior.kind {
                    EventKind::ToolStarted(call) if call.id == *call_id => {
                        let skill = (call.name == SKILL_VIEW)
                            .then(|| {
                                serde_json::from_str::<Value>(&call.arguments)
                                    .ok()
                                    .and_then(|v| {
                                        v.get("name").and_then(Value::as_str).map(str::to_owned)
                                    })
                            })
                            .flatten();
                        Some((call.name.clone(), skill))
                    }
                    EventKind::ProviderReturned(turn) => {
                        turn.calls().find(|c| c.id == *call_id).map(|c| {
                            let skill = (c.name == SKILL_VIEW)
                                .then(|| {
                                    serde_json::from_str::<Value>(&c.arguments)
                                        .ok()
                                        .and_then(|v| {
                                            v.get("name").and_then(Value::as_str).map(str::to_owned)
                                        })
                                })
                                .flatten();
                            (c.name.clone(), skill)
                        })
                    }
                    _ => None,
                })
                .unwrap_or_else(|| ("unknown".to_owned(), None));
            let disposition = match result {
                engine::results::ToolResult::Refused(refusal) => {
                    ToolDiscoveryDisposition::Refused {
                        reason: refusal.reason().to_owned(),
                    }
                }
                engine::results::ToolResult::Original(_)
                | engine::results::ToolResult::History(_)
                | engine::results::ToolResult::View(_) => ToolDiscoveryDisposition::Returned,
            };
            run.tool_discoveries.push(RecordedToolDiscovery {
                ordinal: event.ordinal,
                call_id: call_id.clone(),
                tool_name,
                discovered_skill_name,
                disposition,
                recorded_at: clock,
            });
        }

        run.events.push(event);
        let encoded = serde_json::to_value(&run)
            .map_err(conversion_error("MessageReviewRunRecord", "json"))?;
        let mut active: runtime_command_record::ActiveModel = row.into();
        active.command = Set(encoded);
        active.update(&tx).await?;
        tx.commit().await?;
        Ok(())
    }
}

/// Loads the active wrong-patient source withdrawal boundary for `(practice, workspace)`, if any.
pub(crate) async fn load_workspace_withdrawal_boundary(
    store: &Store,
    practice: &str,
    workspace: &str,
) -> Result<Option<WorkspaceWithdrawalBoundary>, DbErr> {
    let conn = store.spruce_db();
    let withdrawal_rows = source_withdrawal_record::Entity::find()
        .filter(source_withdrawal_record::Column::Practice.eq(practice))
        .filter(source_withdrawal_record::Column::Workspace.eq(workspace))
        .order_by_asc(source_withdrawal_record::Column::WithdrawalEpoch)
        .all(conn)
        .await?;
    if withdrawal_rows.is_empty() {
        return Ok(None);
    }
    let mut latest_epoch = 0_u64;
    let mut candidate_ids = BTreeSet::new();
    let mut message_ids = BTreeSet::new();
    for row in withdrawal_rows {
        let ep = u64::try_from(row.withdrawal_epoch).unwrap_or(1);
        latest_epoch = latest_epoch.max(ep);
        let cands: Vec<String> = serde_json::from_value(row.candidate_ids).map_err(
            conversion_error("source_withdrawal.candidate_ids", "Vec<String>"),
        )?;
        candidate_ids.extend(cands);
        if let Some(msg) = row.message_id {
            message_ids.insert(msg);
        }
    }
    let rev_rows = understanding_withdrawal_record::Entity::find()
        .filter(understanding_withdrawal_record::Column::Practice.eq(practice))
        .filter(understanding_withdrawal_record::Column::Workspace.eq(workspace))
        .order_by_asc(understanding_withdrawal_record::Column::Revision)
        .all(conn)
        .await?;
    let withdrawn_understanding_revisions = rev_rows
        .into_iter()
        .filter_map(|rev| u64::try_from(rev.revision).ok())
        .collect();
    Ok(Some(WorkspaceWithdrawalBoundary {
        latest_epoch,
        candidate_ids: candidate_ids.into_iter().collect(),
        message_ids: message_ids.into_iter().collect(),
        withdrawn_understanding_revisions,
    }))
}

/// Excludes any retained chart source whose `candidate_id` (or `provenance.document_id`)
/// has been withdrawn under the workspace's wrong-patient withdrawal boundary.
pub(crate) fn retain_non_withdrawn_sources(
    sources: &mut Vec<SourceRevision>,
    boundary: Option<&WorkspaceWithdrawalBoundary>,
) {
    let Some(boundary) = boundary else {
        return;
    };
    let withdrawn_set: BTreeSet<&str> = boundary.candidate_ids.iter().map(String::as_str).collect();
    sources.retain(|source| {
        if withdrawn_set.contains(source.candidate_id.as_str()) {
            return false;
        }
        if let Some(doc_id) = source.provenance.get("document_id").and_then(Value::as_str)
            && withdrawn_set.contains(doc_id)
        {
            return false;
        }
        true
    });
}

/// Loads `(source_version, non_withdrawn_sources, withdrawal_boundary)` for `(practice, workspace)`.
pub(crate) async fn load_clean_sources(
    store: &Store,
    practice: &str,
    workspace: &str,
) -> Result<
    (
        u64,
        Vec<SourceRevision>,
        Option<WorkspaceWithdrawalBoundary>,
    ),
    DbErr,
> {
    let mut observation = store.sources(workspace).await?;
    let boundary = load_workspace_withdrawal_boundary(store, practice, workspace).await?;
    retain_non_withdrawn_sources(&mut observation.sources, boundary.as_ref());
    Ok((observation.version, observation.sources, boundary))
}

/// Assembles [`PatientStartingContext`] for a practice-owned background run
/// (`message-review` or `follow-up-reassessment`) with no physician session.
pub(crate) async fn load_practice_patient_starting_context(
    store: &Store,
    practice: &str,
    workspace: &str,
) -> Result<PatientStartingContext, DbErr> {
    let conn = store.spruce_db();
    let (chart_version, clean_sources, withdrawal_boundary) =
        load_clean_sources(store, practice, workspace).await?;

    let pulls = store
        .source_pulls(&[workspace.to_owned()])
        .await?
        .remove(workspace)
        .unwrap_or_default();
    let patient = pulls
        .iter()
        .max_by_key(|pull| pull.pulled_at)
        .map(|pull| pull.patient.clone());
    let source_pulls = pulls
        .into_iter()
        .filter_map(|pull| {
            Some(SourceFreshnessSummary {
                source_id: pull.source_id,
                source_label: pull.source_label,
                pulled_at: pull.pulled_at?,
            })
        })
        .collect();
    let check_standing = store.dispatch_check_standing(practice, workspace).await?;
    let memory_standing = store.active_memory_standing(practice, workspace).await?;

    let open_concerns = store
        .load_open_concern_summaries(practice, workspace)
        .await?;

    let completed_jobs = practice_job_record::Entity::find()
        .filter(practice_job_record::Column::Practice.eq(practice))
        .filter(practice_job_record::Column::Workspace.eq(workspace))
        .filter(practice_job_record::Column::Status.eq(WorkStatus::Completed))
        .order_by_desc(practice_job_record::Column::UpdatedAt)
        .limit(5)
        .all(conn)
        .await?;
    let recent_work = completed_jobs
        .into_iter()
        .map(|job| PriorWorkPointer {
            target: PriorWorkKind::BackgroundJob {
                job_id: job.job_id.clone(),
                job_kind: job.kind,
            },
            title: format!("{} ({})", job.kind.as_str(), job.job_id),
            outcome: "Completed".to_owned(),
            updated_at: job.updated_at,
        })
        .collect();

    let understanding = store
        .patient_understanding_internal(practice, workspace)
        .await?;

    Ok(PatientStartingContext {
        workspace: workspace.to_owned(),
        patient,
        chart_version,
        chart_record_count: clean_sources.len(),
        source_pulls,
        check_standing,
        understanding,
        withdrawal_boundary,
        memory_standing,
        open_concerns,
        recent_work,
    })
}

/// Assembles [`MessageReviewStartingContext`] by reading or starting the active
/// reply review on `(practice, conversation_id)` and joining patient chart,
/// memory, follow-up, concern, and attachment context.
pub async fn build_message_review_starting_context(
    store: &Store,
    authority: &dyn PracticeAuthority,
    practice: &str,
    conversation_id: &str,
    review_id: &str,
    run_id: &str,
    options: &MessageReviewOptions,
) -> Result<(ReplyReviewSnapshot, MessageReviewStartingContext), MessageReviewRecipeError> {
    if let Some(existing) = store.reply_conversation(practice, conversation_id).await?
        && let ConversationLink::LinkedPatient { workspace } = &existing.link
    {
        authority
            .check_practice_patient(&PracticeScope {
                practice: practice.to_owned(),
                workspace: workspace.clone(),
                job_id: format!("message-review:{review_id}"),
            })
            .await?;
    }

    let snapshot = store
        .start_or_read_reply_review(practice, conversation_id, review_id, run_id)
        .await?;
    let workspace = snapshot.review.workspace.clone();
    authority
        .check_practice_patient(&PracticeScope {
            practice: practice.to_owned(),
            workspace: workspace.clone(),
            job_id: format!("message-review:{review_id}"),
        })
        .await?;

    let patient_ctx = load_practice_patient_starting_context(store, practice, &workspace).await?;
    let refresh_statuses = store.saved_source_statuses(practice, &workspace).await?;
    let all_follow_ups = store.workspace_follow_ups(practice, &workspace).await?;
    let active_follow_ups: Vec<FollowUp> = all_follow_ups
        .into_iter()
        .filter(|item| matches!(item.standing, FollowUpStanding::Active { .. }))
        .collect();

    let saw_physician_reply_request = match &snapshot.review.read_state {
        ReviewReadState::Current {
            saw_physician_reply_request,
            ..
        } => *saw_physician_reply_request,
        ReviewReadState::NeedsRead => snapshot.conversation.pending_reply_request.is_some(),
    };

    let withdrawn_candidates: BTreeSet<&str> = patient_ctx
        .withdrawal_boundary
        .as_ref()
        .map(|b| b.candidate_ids.iter().map(String::as_str).collect())
        .unwrap_or_default();
    let withdrawn_messages: BTreeSet<&str> = patient_ctx
        .withdrawal_boundary
        .as_ref()
        .map(|b| b.message_ids.iter().map(String::as_str).collect())
        .unwrap_or_default();

    let mut attachments: Vec<MessageAttachmentDescriptor> = options
        .extra_attachments
        .iter()
        .filter(|att| {
            att.candidate_id
                .as_deref()
                .is_none_or(|cid| !withdrawn_candidates.contains(cid))
                && att
                    .message_id
                    .as_deref()
                    .is_none_or(|mid| !withdrawn_messages.contains(mid))
        })
        .cloned()
        .collect();
    let mut observation = store.sources(&workspace).await?;
    retain_non_withdrawn_sources(
        &mut observation.sources,
        patient_ctx.withdrawal_boundary.as_ref(),
    );
    for src in &observation.sources {
        let is_attachment_or_doc = src.provenance.get("attachment_id").is_some()
            || src.provenance.get("document_id").is_some()
            || src.record_type == "DocumentReference";
        if is_attachment_or_doc {
            let doc_id = src
                .provenance
                .get("document_id")
                .and_then(Value::as_str)
                .unwrap_or(&src.candidate_id)
                .to_owned();
            let message_id = src
                .provenance
                .get("message_id")
                .and_then(Value::as_str)
                .map(str::to_owned);
            if message_id
                .as_deref()
                .is_some_and(|mid| withdrawn_messages.contains(mid))
            {
                continue;
            }
            if !attachments
                .iter()
                .any(|existing| existing.candidate_id.as_deref() == Some(doc_id.as_str()))
            {
                let content_type = src
                    .provenance
                    .get("content_type")
                    .or_else(|| src.provenance.get("mime_type"))
                    .and_then(Value::as_str)
                    .unwrap_or(&src.record_type)
                    .to_owned();
                let page_count = src
                    .provenance
                    .get("page_count")
                    .and_then(Value::as_u64)
                    .and_then(|n| u32::try_from(n).ok());
                attachments.push(MessageAttachmentDescriptor {
                    message_id,
                    candidate_id: Some(doc_id),
                    channel_attachment: None,
                    content_type,
                    page_count,
                });
            }
        }
    }

    let coalesced_changed_candidates = options
        .coalesced_changed_candidates
        .iter()
        .filter(|cid| !withdrawn_candidates.contains(cid.as_str()))
        .cloned()
        .collect();

    let starting = MessageReviewStartingContext {
        practice: practice.to_owned(),
        workspace,
        conversation_id: conversation_id.to_owned(),
        review_id: review_id.to_owned(),
        run_id: run_id.to_owned(),
        conversation_version: snapshot.conversation.conversation_version,
        source_version: snapshot.source_version,
        move_epoch: snapshot.review.move_epoch,
        patient_messages: snapshot.conversation.patient_messages.clone(),
        practice_messages: snapshot.conversation.practice_messages.clone(),
        attachments,
        current_reply_standing: snapshot.conversation.reply_standing.clone(),
        saw_physician_reply_request,
        coalesced_changed_candidates,
        patient: patient_ctx.patient,
        chart_record_count: patient_ctx.chart_record_count,
        source_pulls: patient_ctx.source_pulls,
        refresh_statuses,
        check_standing: patient_ctx.check_standing,
        understanding: patient_ctx.understanding,
        withdrawal_boundary: patient_ctx.withdrawal_boundary,
        memory_standing: patient_ctx.memory_standing,
        active_follow_ups,
        open_concerns: patient_ctx.open_concerns,
    };

    Ok((snapshot, starting))
}

struct MessageReviewHost<'a> {
    store: Store,
    authority: &'a dyn PracticeAuthority,
    practice: String,
    workspace: String,
    conversation_id: String,
    review_id: String,
    run_id: String,
    attempt_id: String,
    committed_outcome: Option<ReplyReviewOutcome>,
}

impl MessageReviewHost<'_> {
    fn source_message(ctx: &MessageReviewStartingContext) -> engine::ProductMessage {
        engine::ProductMessage {
            id: format!("message-review:{}", ctx.review_id),
            role: engine::history::Role::User,
            text: ctx.prompt_text(),
        }
    }

    fn practice_scope(&self) -> PracticeScope {
        PracticeScope {
            practice: self.practice.clone(),
            workspace: self.workspace.clone(),
            job_id: format!("message-review:{}", self.run_id),
        }
    }

    fn control_scope(&self) -> Scope {
        self.practice_scope().as_control_scope()
    }

    async fn load_run(&self) -> Result<MessageReviewRunRecord, DbErr> {
        self.store
            .message_review_run(&self.practice, &self.run_id)
            .await?
            .ok_or_else(|| error("message review run missing"))
    }
}

#[async_trait]
impl Product for MessageReviewHost<'_> {
    async fn message_sources(
        &mut self,
    ) -> Result<Option<Vec<engine::ProductMessage>>, engine::ProductError> {
        let run = self.load_run().await.map_err(persistence_failure)?;
        Ok(Some(vec![Self::source_message(&run.starting_context)]))
    }

    async fn admit_binding(&mut self, recipe: &RequestBinding) -> Result<(), engine::ProductError> {
        self.authority
            .check_practice_patient(&self.practice_scope())
            .await?;
        let run = self.load_run().await.map_err(persistence_failure)?;
        let (saved_binding, _) = self
            .store
            .load_spruce_matching_frozen(
                &self.practice,
                &run.provenance.binding,
                &run.provenance.skills,
            )
            .await
            .map_err(persistence_failure)?;
        if saved_binding != *recipe {
            return Err(error("message review dispatch changed frozen recipe").into());
        }
        Ok(())
    }

    fn read_only(&self, name: &str) -> bool {
        chat_read_only(name)
            || read_only_review_tool(name)
            || crate::initial_understanding::read_only_understanding_tool(name)
            || is_library_tool(name)
            || is_document_page_tool(name)
            || is_spruce_working_note_tool(name)
    }

    fn read_result(&mut self, id: &str) -> engine::recipe::ToolFuture<Option<String>> {
        let store = self.store.clone();
        let practice = self.practice.clone();
        let run_id = self.run_id.clone();
        let id = id.to_owned();
        Box::pin(async move {
            let run = store
                .message_review_run(&practice, &run_id)
                .await?
                .ok_or_else(|| error("message review run missing"))?;
            let messages = vec![Self::source_message(&run.starting_context)];
            let admitted: Vec<(Event, bool)> = run.events.into_iter().map(|e| (e, true)).collect();
            engine::results::resolve(admitted.iter(), &id, &messages).map_err(Into::into)
        })
    }

    async fn assert_current(&mut self) -> Result<(), engine::ProductError> {
        self.authority
            .check_practice_patient(&self.practice_scope())
            .await?;
        let run = self.load_run().await.map_err(persistence_failure)?;
        if run.attempt_id != self.attempt_id {
            return Err(engine::ProductError::NotCurrent(engine::NotCurrent::Lost));
        }
        if run.status != MessageReviewRunStatus::Running {
            return Err(engine::ProductError::NotCurrent(
                engine::NotCurrent::Withdrawn,
            ));
        }
        Ok(())
    }

    async fn persist(&mut self, event: Event) -> Result<(), engine::ProductError> {
        self.store
            .persist_message_review_event(&self.practice, &self.run_id, &self.attempt_id, event)
            .await
            .map_err(persistence_failure)
    }

    fn saved_outcome(
        &mut self,
        call: &Invocation,
    ) -> engine::recipe::ToolFuture<Option<ToolOutcome>> {
        let store = self.store.clone();
        let practice = self.practice.clone();
        let workspace = self.workspace.clone();
        let run_id = self.run_id.clone();
        let practice_scope = self.practice_scope();
        let call = call.clone();
        Box::pin(async move {
            if call.name == COMPLETE_MESSAGE_REVIEW_TOOL {
                let Some(run) = store.message_review_run(&practice, &run_id).await? else {
                    return Ok(None);
                };
                if let Some(staged) = run.staged_submission
                    && staged.call_id == call.id
                {
                    return Ok(Some(ToolOutcome::Returned(
                        json!({
                            "outcome": "staged",
                            "decision": staged.decision,
                        })
                        .to_string(),
                    )));
                }
                return Ok(None);
            }
            if is_refresh_tool(&call.name) {
                return store
                    .saved_practice_refresh_outcome(&practice_scope, &run_id, &call)
                    .await;
            }
            if is_concern_tool(&call.name) {
                return store
                    .saved_concern_tool_outcome(&practice, &workspace, &run_id, &call)
                    .await;
            }
            if is_follow_up_tool(&call.name) {
                return store
                    .saved_follow_up_tool_outcome(&practice, &workspace, &run_id, &call)
                    .await;
            }
            Ok(None)
        })
    }

    fn execute(&mut self, call: &Invocation, arguments: Value) -> engine::recipe::ToolFuture {
        let store = self.store.clone();
        let practice = self.practice.clone();
        let workspace = self.workspace.clone();
        let conversation_id = self.conversation_id.clone();
        let review_id = self.review_id.clone();
        let run_id = self.run_id.clone();
        let attempt_id = self.attempt_id.clone();
        let practice_scope = self.practice_scope();
        let control_scope = self.control_scope();
        let call = call.clone();
        let admitted_authority = AdmittedPracticePatient {
            practice: practice.clone(),
            workspace: workspace.clone(),
        };

        Box::pin(async move {
            if is_physician_only_tool(&call.name) {
                return Ok(ToolOutcome::refused(format!(
                    "message review background run cannot perform physician-only action `{}`",
                    call.name
                )));
            }

            let run = store
                .message_review_run(&practice, &run_id)
                .await?
                .ok_or_else(|| error("message review run missing"))?;
            let (_, skills) = store
                .load_spruce_matching_frozen(
                    &practice,
                    &run.provenance.binding,
                    &run.provenance.skills,
                )
                .await?;

            if is_spruce_working_note_tool(&call.name) {
                let note_call = Invocation {
                    id: call.id.clone(),
                    name: call.name.clone(),
                    arguments,
                };
                return execute_spruce_working_note(
                    &run.run_id,
                    &note_call,
                    run.events.iter().map(|e| (run.run_id.as_str(), e)),
                )?
                .ok_or_else(|| "working note tool was not handled".into());
            }

            if engine::history::is_tool(&call.name) {
                let src = Self::source_message(&run.starting_context);
                let messages = vec![engine::history::Message {
                    id: src.id,
                    role: src.role,
                    text: src.text,
                    metadata: json!({
                        "practice": practice,
                        "workspace": workspace,
                        "conversation_id": conversation_id,
                        "run_id": run_id,
                    }),
                }];
                let admitted: Vec<(Event, bool)> =
                    run.events.iter().cloned().map(|e| (e, true)).collect();
                let page = match engine::history::execute(
                    &call.name,
                    &arguments,
                    &messages,
                    admitted.iter(),
                ) {
                    Ok(page) => page,
                    Err(reason) => return Ok(ToolOutcome::refused(reason)),
                };
                return Ok(ToolOutcome::History(page));
            }

            if call.name == SKILL_VIEW {
                return Ok(skills.execute(&call.name, &arguments));
            }

            if matches!(call.name.as_str(), SEARCH_TOOL | READ_TOOL) {
                if let Some(req_ws) = arguments.get("workspace").and_then(Value::as_str)
                    && req_ws != workspace
                {
                    return Ok(ToolOutcome::refused(
                        "message review may read only its linked patient workspace",
                    ));
                }
                return store
                    .execute_chat_chart_tool(&control_scope, &call.name, &arguments)
                    .await;
            }

            if is_document_page_tool(&call.name) {
                if let Some(req_ws) = arguments.get("workspace").and_then(Value::as_str)
                    && req_ws != workspace
                {
                    return Ok(ToolOutcome::refused(
                        "message review may read only its linked patient workspace",
                    ));
                }
                return store
                    .execute_document_page_tool(&practice, &workspace, &arguments)
                    .await;
            }

            if is_refresh_tool(&call.name) {
                let refresh_inv = Invocation {
                    id: call.id.clone(),
                    name: call.name.clone(),
                    arguments,
                };
                let outcome = store
                    .execute_practice_refresh_tool(
                        &practice_scope,
                        &admitted_authority,
                        &run_id,
                        &refresh_inv,
                        store.source_refresh(),
                    )
                    .await?;
                let current_obs = store.sources(&workspace).await?;
                let tx = store.spruce_db().begin().await?;
                if let Some(row) = runtime_command_record::Entity::find_by_id((
                    practice.clone(),
                    MESSAGE_REVIEW_RUN_ACTOR.to_owned(),
                    run_id.clone(),
                ))
                .lock_exclusive()
                .one(&tx)
                .await?
                {
                    let mut updated = decode_message_review_run(row.clone())?;
                    updated.last_pinned_source_version = current_obs.version;
                    let mut active: runtime_command_record::ActiveModel = row.into();
                    active.command = Set(serde_json::to_value(&updated)
                        .map_err(conversion_error("MessageReviewRunRecord", "json"))?);
                    active.update(&tx).await?;
                }
                tx.commit().await?;
                return Ok(outcome);
            }

            if call.name == PATIENT_MEMORY_TOOL {
                return execute_background_memory_tool(
                    &store,
                    &practice,
                    &workspace,
                    "message_review",
                    &run_id,
                    &attempt_id,
                    &call.id,
                    &run.events,
                    arguments,
                )
                .await;
            }

            if call.name == RETRIEVE_PATIENT_WORK_TOOL {
                let jobs = store
                    .patient_reviews(&practice, &workspace, &admitted_authority)
                    .await
                    .map_err(|err| -> engine::Cause { Box::new(err) })?;
                return Ok(ToolOutcome::Returned(
                    json!({
                        "outcome": "available",
                        "patient_reviews": jobs,
                    })
                    .to_string(),
                ));
            }

            if call.name == READ_PATIENT_REVIEWS_TOOL {
                let parsed = match ReadReviewsCall::parse(&arguments) {
                    Ok(parsed) => parsed,
                    Err(err) => return Ok(ToolOutcome::refused(err.to_string())),
                };
                return match parsed {
                    ReadReviewsCall::List {
                        limit,
                        include_incomplete,
                    } => {
                        let reviews = store
                            .patient_reviews(&practice, &workspace, &admitted_authority)
                            .await
                            .map_err(|err| -> engine::Cause { Box::new(err) })?;
                        let filtered: Vec<_> = reviews
                            .into_iter()
                            .filter(|r| include_incomplete || r.state.status_str() == "completed")
                            .take(limit)
                            .collect();
                        Ok(ToolOutcome::Returned(
                            json!({
                                "outcome": "available",
                                "reviews": filtered,
                            })
                            .to_string(),
                        ))
                    }
                    ReadReviewsCall::Read { review_id: target } => {
                        let review = store
                            .patient_review(&practice, &workspace, &target, &admitted_authority)
                            .await
                            .map_err(|err| -> engine::Cause { Box::new(err) })?;
                        Ok(ToolOutcome::Returned(
                            json!({
                                "outcome": "available",
                                "review": review,
                            })
                            .to_string(),
                        ))
                    }
                };
            }

            if is_concern_tool(&call.name) {
                let source_work = SourceWork::new("message_review", &run_id, &attempt_id)
                    .map_err(|err| -> engine::Cause { Box::new(err) })?;
                let inv = Invocation {
                    id: call.id.clone(),
                    name: call.name.clone(),
                    arguments,
                };
                return store
                    .execute_concern_tool(&practice, &workspace, &source_work, &inv)
                    .await;
            }

            if is_follow_up_tool(&call.name) {
                let source_work = SourceWork::new("message_review", &run_id, &attempt_id)
                    .map_err(|err| -> engine::Cause { Box::new(err) })?;
                let inv = Invocation {
                    id: call.id.clone(),
                    name: call.name.clone(),
                    arguments,
                };
                return store
                    .execute_follow_up_tool(&practice, &workspace, &source_work, &inv)
                    .await;
            }

            if is_understanding_tool(&call.name) {
                let current = store
                    .patient_understanding_internal(&practice, &workspace)
                    .await?;
                if call.name == SAVE_UNDERSTANDING_TOOL {
                    return Ok(ToolOutcome::refused(
                        "save_patient_understanding belongs to patient understanding runs; record durable message updates in patient_memory or patient_follow_up",
                    ));
                }
                return Ok(ToolOutcome::Returned(
                    json!({
                        "outcome": "available",
                        "understanding": current,
                    })
                    .to_string(),
                ));
            }

            if is_library_tool(&call.name) {
                return Ok(store
                    .clinical_library()
                    .execute(&call.name, &arguments)
                    .await);
            }

            if call.name == COMPLETE_MESSAGE_REVIEW_TOOL {
                let mut submission = match parse_submission(&call.id, &arguments) {
                    Ok(sub) => sub,
                    Err(reason) => return Ok(ToolOutcome::refused(reason)),
                };
                let (_, clean_sources, withdrawal_boundary) =
                    load_clean_sources(&store, &practice, &workspace).await?;
                let withdrawn_candidates: BTreeSet<&str> = withdrawal_boundary
                    .as_ref()
                    .map(|b| b.candidate_ids.iter().map(String::as_str).collect())
                    .unwrap_or_default();
                let evidence = collect_run_evidence(&run.events, &clean_sources);

                for auto_id in &evidence.raised_concern_ids_in_events {
                    if !submission.raised_concern_ids.contains(auto_id) {
                        submission.raised_concern_ids.push(auto_id.clone());
                    }
                }

                // Rule: medication question requires refreshing chart sources before proposing a reply.
                if let SubmittedMessageReviewDecision::ProposeReply {
                    addresses_medication_question: true,
                    limitations,
                    ..
                } = &submission.decision
                {
                    if evidence.refresh_completed_ordinal.is_none() {
                        return Ok(ToolOutcome::refused(
                            "Medication questions must call `refresh_patient_sources` and read the current medication chart before proposing a reply.",
                        ));
                    }
                    if !evidence.chart_read_after_refresh
                        && !(evidence.refresh_had_failure && !limitations.is_empty())
                    {
                        return Ok(ToolOutcome::refused(
                            "After calling `refresh_patient_sources` for a medication question, read the current medication records with `read_patient_sources` (or state the refresh failure in `limitations`) before proposing a reply.",
                        ));
                    }
                }

                // Rule: every cited chart record must not be withdrawn and must have been read in this run.
                if let SubmittedMessageReviewDecision::ProposeReply { cited_records, .. } =
                    &submission.decision
                {
                    for cited in cited_records {
                        if withdrawn_candidates.contains(cited.candidate_id.as_str()) {
                            return Ok(ToolOutcome::refused(format!(
                                "Cited chart record `{}` was withdrawn from this patient workspace and cannot be cited.",
                                cited.candidate_id
                            )));
                        }
                        if !evidence
                            .read_revisions
                            .contains(&(cited.candidate_id.clone(), cited.revision.clone()))
                        {
                            return Ok(ToolOutcome::refused(format!(
                                "Cited chart record `{}` at revision `{}` was not read in this run; call `read_patient_sources` or `read_document_page` first.",
                                cited.candidate_id, cited.revision
                            )));
                        }
                    }
                }

                // Rule: an urgent `Now` concern requires an emergency-care reply draft (`911` / ED).
                let open_now_concerns = workspace_concern_record::Entity::find()
                    .filter(workspace_concern_record::Column::Practice.eq(&practice))
                    .filter(workspace_concern_record::Column::Workspace.eq(&workspace))
                    .filter(workspace_concern_record::Column::Status.eq("open"))
                    .filter(workspace_concern_record::Column::Urgency.eq("now"))
                    .all(store.spruce_db())
                    .await?;
                let has_urgent_now_concern = submission
                    .raised_concern_ids
                    .iter()
                    .any(|cid| open_now_concerns.iter().any(|c| c.id == cid.as_str()));
                if has_urgent_now_concern {
                    match &submission.decision {
                        SubmittedMessageReviewDecision::ProposeReply { reply_text, .. }
                            if directs_immediate_emergency_care(reply_text) => {}
                        _ => {
                            return Ok(ToolOutcome::refused(
                                "When raising an urgent (`now`) clinical concern on a patient message, you must propose a reply (`propose_reply`) that directs immediate emergency care (calling 911 or going to the nearest emergency department).",
                            ));
                        }
                    }
                }

                // Rule: `Ask Coda to reply` requires `propose_reply`.
                if run.starting_context.saw_physician_reply_request
                    && !matches!(
                        submission.decision,
                        SubmittedMessageReviewDecision::ProposeReply { .. }
                    )
                {
                    return Ok(ToolOutcome::refused(
                        "An authorized physician explicitly requested a reply (`Ask Coda to reply`); `decision` must be `propose_reply`.",
                    ));
                }

                // Validate practice message closures against non-automatic read practice messages.
                for closure in &submission.answered_closures {
                    if let Some(pm) = run
                        .starting_context
                        .practice_messages
                        .iter()
                        .find(|m| m.practice_message_id == closure.practice_message_id)
                        && pm.kind == PracticeMessageKind::Automatic
                    {
                        return Ok(ToolOutcome::refused(format!(
                            "Automatic practice message `{}` is an auto-responder and cannot close patient messages as answered.",
                            closure.practice_message_id
                        )));
                    }
                }

                let tx = store.spruce_db().begin().await?;
                let locked = runtime_command_record::Entity::find_by_id((
                    practice,
                    MESSAGE_REVIEW_RUN_ACTOR.to_owned(),
                    run_id,
                ))
                .lock_exclusive()
                .one(&tx)
                .await?
                .ok_or_else(|| error("message review run missing"))?;
                let mut latest = decode_message_review_run(locked.clone())?;
                let preview = json!({
                    "outcome": "staged",
                    "review_id": review_id,
                    "decision": &submission.decision,
                });
                latest.staged_submission = Some(submission);
                let mut active: runtime_command_record::ActiveModel = locked.into();
                active.command = Set(serde_json::to_value(&latest)
                    .map_err(conversion_error("MessageReviewRunRecord", "json"))?);
                active.update(&tx).await?;
                tx.commit().await?;
                return Ok(ToolOutcome::Returned(preview.to_string()));
            }

            Ok(ToolOutcome::refused(format!(
                "tool `{}` is not available in linked message review",
                call.name
            )))
        })
    }

    async fn finish(&mut self, turn: &Turn) -> Result<Finish, engine::ProductError> {
        self.assert_current().await?;
        let run = self.load_run().await?;
        let Some(staged) = run.staged_submission.clone() else {
            return Ok(Finish::Continue(json!({
                "outcome": "decision_not_submitted",
                "message": format!(
                    "Call `{COMPLETE_MESSAGE_REVIEW_TOOL}` with your message review decision before finishing."
                ),
            })));
        };

        // Check whether conversation messages or workspace sources moved since our last pin.
        let latest_snapshot = self
            .store
            .start_or_read_reply_review(
                &self.practice,
                &self.conversation_id,
                &self.review_id,
                &self.run_id,
            )
            .await
            .map_err(|err| -> engine::Cause { Box::new(err) })?;

        let saw_request_now = match &latest_snapshot.review.read_state {
            ReviewReadState::Current {
                saw_physician_reply_request,
                ..
            } => *saw_physician_reply_request,
            ReviewReadState::NeedsRead => {
                latest_snapshot.conversation.pending_reply_request.is_some()
            }
        };

        if latest_snapshot.conversation.conversation_version > run.last_read_conversation_version {
            // New channel activity arrived while the model was working: update the run's
            // conversation snapshot and ask the model to reconsider before completing!
            let tx = self.store.spruce_db().begin().await?;
            let locked = runtime_command_record::Entity::find_by_id((
                self.practice.clone(),
                MESSAGE_REVIEW_RUN_ACTOR.to_owned(),
                self.run_id.clone(),
            ))
            .lock_exclusive()
            .one(&tx)
            .await?
            .ok_or_else(|| error("message review run missing"))?;
            let mut updated = decode_message_review_run(locked.clone())?;
            updated.last_read_conversation_version =
                latest_snapshot.conversation.conversation_version;
            updated.last_pinned_source_version = latest_snapshot.source_version;
            updated.starting_context.conversation_version =
                latest_snapshot.conversation.conversation_version;
            updated.starting_context.patient_messages =
                latest_snapshot.conversation.patient_messages.clone();
            updated.starting_context.practice_messages =
                latest_snapshot.conversation.practice_messages.clone();
            updated.starting_context.saw_physician_reply_request = saw_request_now;
            updated.staged_submission = None;
            let mut active: runtime_command_record::ActiveModel = locked.into();
            active.command = Set(serde_json::to_value(&updated)
                .map_err(conversion_error("MessageReviewRunRecord", "json"))?);
            active.update(&tx).await?;
            tx.commit().await?;

            return Ok(Finish::Continue(json!({
                "outcome": "conversation_updated",
                "conversation_version": latest_snapshot.conversation.conversation_version,
                "patient_messages": latest_snapshot.conversation.patient_messages,
                "practice_messages": latest_snapshot.conversation.practice_messages,
                "saw_physician_reply_request": saw_request_now,
                "message": "New conversation activity arrived during this review. Reconsider affected content and call `complete_message_review` again.",
            })));
        }

        if latest_snapshot.source_version > run.last_pinned_source_version {
            let tx = self.store.spruce_db().begin().await?;
            let locked = runtime_command_record::Entity::find_by_id((
                self.practice.clone(),
                MESSAGE_REVIEW_RUN_ACTOR.to_owned(),
                self.run_id.clone(),
            ))
            .lock_exclusive()
            .one(&tx)
            .await?
            .ok_or_else(|| error("message review run missing"))?;
            let mut updated = decode_message_review_run(locked.clone())?;
            updated.last_pinned_source_version = latest_snapshot.source_version;
            updated.starting_context.source_version = latest_snapshot.source_version;
            updated.staged_submission = None;
            let mut active: runtime_command_record::ActiveModel = locked.into();
            active.command = Set(serde_json::to_value(&updated)
                .map_err(conversion_error("MessageReviewRunRecord", "json"))?);
            active.update(&tx).await?;
            tx.commit().await?;

            return Ok(Finish::Continue(json!({
                "outcome": "sources_changed",
                "source_version": latest_snapshot.source_version,
                "message": "Patient chart records updated during this review. Verify any affected records via `search_patient_information` / `read_patient_sources` and call `complete_message_review` again.",
            })));
        }

        let runtime_decision = match staged.decision {
            SubmittedMessageReviewDecision::ProposeReply {
                reply_text,
                basis_summary,
                limitations,
                cited_records,
                addresses_medication_question: _,
            } => ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: reply_text,
                basis: ReplyProposalBasisInput {
                    summary: basis_summary,
                    limits: limitations,
                    cited_records: cited_records.clone(),
                },
                cited_records,
            }),
            SubmittedMessageReviewDecision::NoReplyWouldHelp {
                reason,
                basis_summary: _,
                limitations: _,
            } => ReplyReviewDecision::NoReplyWouldHelp { reason },
            SubmittedMessageReviewDecision::AllHandled { basis_summary: _ } => {
                ReplyReviewDecision::AllHandled
            }
        };

        let completion_key = format!("msg-review-complete:{}:{}", self.run_id, staged.call_id);
        let outcome = match self
            .store
            .complete_reply_review(&CompleteReplyReview {
                practice: self.practice.clone(),
                conversation_id: self.conversation_id.clone(),
                review_id: self.review_id.clone(),
                run_id: self.run_id.clone(),
                completion_key,
                answered_closures: staged.answered_closures,
                decision: runtime_decision,
                raised_concern_ids: staged.raised_concern_ids,
            })
            .await
        {
            Ok(outcome) => outcome,
            Err(
                err @ (ReplyProposalError::AutomaticPracticeMessageCannotAnswer { .. }
                | ReplyProposalError::UnverifiedCitation { .. }
                | ReplyProposalError::OpenMessagesRemainUnhandled
                | ReplyProposalError::NoOpenMessagesForProposal
                | ReplyProposalError::PhysicianRequestedReplyRequiresProposal
                | ReplyProposalError::InvalidInput(_)),
            ) => {
                let tx = self.store.spruce_db().begin().await?;
                let locked = runtime_command_record::Entity::find_by_id((
                    self.practice.clone(),
                    MESSAGE_REVIEW_RUN_ACTOR.to_owned(),
                    self.run_id.clone(),
                ))
                .lock_exclusive()
                .one(&tx)
                .await?
                .ok_or_else(|| error("message review run missing"))?;
                let mut updated = decode_message_review_run(locked.clone())?;
                updated.staged_submission = None;
                let mut active: runtime_command_record::ActiveModel = locked.into();
                active.command = Set(serde_json::to_value(&updated)
                    .map_err(conversion_error("MessageReviewRunRecord", "json"))?);
                active.update(&tx).await?;
                tx.commit().await?;

                return Ok(Finish::Continue(json!({
                    "outcome": "completion_refused",
                    "reason": err.to_string(),
                })));
            }
            Err(other) => return Err(engine::ProductError::Failed(Box::new(other))),
        };

        let saved_completion = match &outcome {
            ReplyReviewOutcome::Applied(saved) | ReplyReviewOutcome::Duplicate(saved) => {
                (**saved).clone()
            }
        };

        let clock = self.store.spruce_now().await?;
        let tx = self.store.spruce_db().begin().await?;
        let locked = runtime_command_record::Entity::find_by_id((
            self.practice.clone(),
            MESSAGE_REVIEW_RUN_ACTOR.to_owned(),
            self.run_id.clone(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        .ok_or_else(|| error("message review run missing"))?;
        let mut latest = decode_message_review_run(locked.clone())?;
        latest.status = MessageReviewRunStatus::Completed;
        latest.saved_completion = Some(saved_completion);
        latest.final_summary = Some(turn.text());
        latest.completed_at = Some(clock);
        let mut active: runtime_command_record::ActiveModel = locked.into();
        active.command = Set(serde_json::to_value(&latest)
            .map_err(conversion_error("MessageReviewRunRecord", "json"))?);
        active.update(&tx).await?;
        tx.commit().await?;

        self.committed_outcome = Some(outcome);
        Ok(Finish::Committed)
    }

    async fn finish_partial(
        &mut self,
        turn: &Turn,
        _backstops: &[engine::LimitBackstop],
    ) -> Result<(), engine::ProductError> {
        self.assert_current().await?;
        let clock = self.store.spruce_now().await.map_err(persistence_failure)?;
        let tx = self
            .store
            .spruce_db()
            .begin()
            .await
            .map_err(persistence_failure)?;
        let locked = runtime_command_record::Entity::find_by_id((
            self.practice.clone(),
            MESSAGE_REVIEW_RUN_ACTOR.to_owned(),
            self.run_id.clone(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await
        .map_err(persistence_failure)?
        .ok_or_else(|| error("message review run missing"))?;
        let mut latest = decode_message_review_run(locked.clone())?;
        latest.status = MessageReviewRunStatus::Held;
        latest.final_summary = Some(turn.text());
        latest.completed_at = Some(clock);
        let mut active: runtime_command_record::ActiveModel = locked.into();
        active.command = Set(serde_json::to_value(&latest)
            .map_err(conversion_error("MessageReviewRunRecord", "json"))?);
        active.update(&tx).await.map_err(persistence_failure)?;
        tx.commit().await.map_err(persistence_failure)?;
        Ok(())
    }

    async fn window_seed(&mut self) -> Result<engine::WindowSeed, engine::ProductError> {
        self.assert_current().await?;
        let run = self.load_run().await.map_err(persistence_failure)?;
        let src = Self::source_message(&run.starting_context);
        let note_references =
            spruce_working_note_references(run.events.iter().map(|e| (run.run_id.as_str(), e)))?;
        Ok(engine::WindowSeed {
            items: vec![Item::ProductMessage(src)],
            note_references,
        })
    }
}

/// Owned `'static` authority token scoped to the single `(practice, workspace)`
/// already verified by [`Product::admit_binding`] and [`Product::assert_current`].
pub(crate) struct AdmittedPracticePatient {
    pub(crate) practice: String,
    pub(crate) workspace: String,
}

#[async_trait]
impl PracticeAuthority for AdmittedPracticePatient {
    async fn check_practice_patient(
        &self,
        scope: &PracticeScope,
    ) -> Result<(), PracticeAccessError> {
        if scope.practice.trim().is_empty()
            || scope.workspace.trim().is_empty()
            || scope.job_id.trim().is_empty()
        {
            return Err(PracticeAccessError::InvalidScope);
        }
        if scope.practice != self.practice {
            return Err(PracticeAccessError::ForeignPractice {
                requested_practice: scope.practice.clone(),
                owner_practice: self.practice.clone(),
                workspace: scope.workspace.clone(),
            });
        }
        if scope.workspace != self.workspace {
            return Err(PracticeAccessError::UnregisteredPatient {
                practice: scope.practice.clone(),
                workspace: scope.workspace.clone(),
            });
        }
        Ok(())
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "internal helper carries practice, workspace, work provenance, attempt events, and parsed arguments"
)]
pub(crate) async fn execute_background_memory_tool(
    store: &Store,
    practice: &str,
    workspace: &str,
    job_label: &str,
    work_id: &str,
    attempt_id: &str,
    call_id: &str,
    events: &[Event],
    arguments: Value,
) -> Result<ToolOutcome, engine::ProductError> {
    let parsed: RawMemoryToolCall = match serde_json::from_value(arguments) {
        Ok(p) => p,
        Err(err) => {
            return Ok(ToolOutcome::refused(format!(
                "invalid `{PATIENT_MEMORY_TOOL}` arguments: {err}"
            )));
        }
    };
    let (_, clean_sources, _) = load_clean_sources(store, practice, workspace).await?;
    let evidence = collect_run_evidence(events, &clean_sources);
    let cited_records: Vec<ExactRecordReference> = evidence
        .read_revisions
        .into_iter()
        .filter_map(|(cid, rev)| ExactRecordReference::new(cid, rev).ok())
        .collect();
    let provenance = MemoryProvenance::ClinicalSummary {
        job_id: format!("{job_label}:{work_id}"),
        work_id: work_id.to_owned(),
        attempt_id: attempt_id.to_owned(),
        records: cited_records,
    };
    let command_key = format!("{work_id}:{}", call_id.trim());

    match parsed {
        RawMemoryToolCall::Read { include_retired } => {
            let selection = if include_retired {
                MemorySelection::ActiveAndRetired
            } else {
                MemorySelection::ActiveOnly
            };
            match store.patient_memories(practice, workspace, selection).await {
                Ok(entries) => Ok(ToolOutcome::Returned(
                    json!({
                        "outcome": "available",
                        "entries": entries,
                    })
                    .to_string(),
                )),
                Err(PatientMemoryError::Unavailable { reason }) => Ok(ToolOutcome::Returned(
                    json!({
                        "outcome": "failed",
                        "saved": false,
                        "limits": [{
                            "kind": "unavailable",
                            "subject": "patient_memory",
                            "detail": reason,
                        }],
                    })
                    .to_string(),
                )),
                Err(PatientMemoryError::Database(db_err)) => Err(persistence_failure(db_err)),
                Err(other) => Ok(ToolOutcome::refused(other.to_string())),
            }
        }
        RawMemoryToolCall::RecordCorrection {
            memory_id,
            topic,
            statement,
        } => {
            match store
                .record_patient_memory(
                    &command_key,
                    &RecordMemoryCorrection {
                        practice: practice.to_owned(),
                        workspace: workspace.to_owned(),
                        id: memory_id,
                        topic,
                        statement,
                        provenance,
                    },
                )
                .await
            {
                Ok(MemoryMutation::Applied(entry)) => Ok(ToolOutcome::Returned(
                    json!({
                        "outcome": "applied",
                        "saved": true,
                        "entry": entry,
                    })
                    .to_string(),
                )),
                Ok(MemoryMutation::Duplicate(entry)) => Ok(ToolOutcome::Returned(
                    json!({
                        "outcome": "duplicate",
                        "saved": true,
                        "entry": entry,
                    })
                    .to_string(),
                )),
                Err(PatientMemoryError::Unavailable { reason }) => Ok(ToolOutcome::Returned(
                    json!({
                        "outcome": "failed",
                        "saved": false,
                        "error": reason,
                    })
                    .to_string(),
                )),
                Err(PatientMemoryError::Database(db_err)) => Err(persistence_failure(db_err)),
                Err(other) => Ok(ToolOutcome::refused(other.to_string())),
            }
        }
        RawMemoryToolCall::Retire { memory_id, reason } => {
            match store
                .retire_patient_memory(
                    &command_key,
                    &RetirePatientMemory {
                        practice: practice.to_owned(),
                        workspace: workspace.to_owned(),
                        id: memory_id,
                        reason,
                        retired_by: provenance,
                    },
                )
                .await
            {
                Ok(MemoryMutation::Applied(entry)) => Ok(ToolOutcome::Returned(
                    json!({
                        "outcome": "applied",
                        "saved": true,
                        "entry": entry,
                    })
                    .to_string(),
                )),
                Ok(MemoryMutation::Duplicate(entry)) => Ok(ToolOutcome::Returned(
                    json!({
                        "outcome": "duplicate",
                        "saved": true,
                        "entry": entry,
                    })
                    .to_string(),
                )),
                Err(PatientMemoryError::Unavailable { reason }) => Ok(ToolOutcome::Returned(
                    json!({
                        "outcome": "failed",
                        "saved": false,
                        "error": reason,
                    })
                    .to_string(),
                )),
                Err(PatientMemoryError::Database(db_err)) => Err(persistence_failure(db_err)),
                Err(other) => Ok(ToolOutcome::refused(other.to_string())),
            }
        }
    }
}

/// Executes the linked patient message review recipe on `roles.background` over
/// `provider`, persisting versioned provenance, initial loads, later tool
/// discoveries, and the committed [`ReplyReviewOutcome`].
#[expect(
    clippy::too_many_arguments,
    reason = "entry point binds store, practice authority, conversation target, options, model roles, skills, provider, run limits, and cancellation"
)]
pub async fn run_message_review<P: Provider>(
    store: &Store,
    authority: &dyn PracticeAuthority,
    practice: &str,
    conversation_id: &str,
    options: &MessageReviewOptions,
    roles: &ModelRoles,
    skills: &PhysicianSkills,
    provider: &mut P,
    limits: engine::RunLimits,
    cancel: &engine::Cancellation,
) -> Result<CompletedMessageReviewRun, MessageReviewRecipeError> {
    let run_id = format!("msg-review-run-{}", Uuid::new_v4());
    let attempt_id = format!("attempt-{}", Uuid::new_v4());
    let existing_conv = store.reply_conversation(practice, conversation_id).await?;
    let (review_id, restart_existing_active) = match &options.review_id {
        Some(explicit) => {
            let matches_active = existing_conv.as_ref().is_some_and(|conv| {
                conv.active_review
                    .as_ref()
                    .is_some_and(|active| active.review_id == *explicit && active.run_id != run_id)
            });
            (explicit.clone(), matches_active)
        }
        None => match existing_conv
            .as_ref()
            .and_then(|conv| conv.active_review.as_ref())
        {
            Some(active) => (active.review_id.clone(), active.run_id != run_id),
            None => (format!("msg-review-{}", Uuid::new_v4()), false),
        },
    };
    if restart_existing_active {
        if let Some(conv) = &existing_conv
            && let ConversationLink::LinkedPatient { workspace } = &conv.link
        {
            authority
                .check_practice_patient(&PracticeScope {
                    practice: practice.to_owned(),
                    workspace: workspace.clone(),
                    job_id: format!("message-review:{review_id}"),
                })
                .await?;
        }
        store
            .restart_reply_review(practice, conversation_id, &run_id)
            .await?;
    }

    let (_snapshot, starting_context) = build_message_review_starting_context(
        store,
        authority,
        practice,
        conversation_id,
        &review_id,
        &run_id,
        options,
    )
    .await?;

    let (built_recipe, preloaded_digests) = recipe(roles, skills)?;
    let cache_key = harness::PromptCacheKey::scoped(
        "chords-message-review",
        &[
            &starting_context.practice,
            &starting_context.workspace,
            &starting_context.conversation_id,
            &review_id,
        ],
    );
    let built_binding = built_recipe.bind(Some(cache_key));
    let guidance_sha = message_review_instructions_sha256();
    let discoverable = discoverable_skill_names(skills, &PRELOADED_MESSAGE_REVIEW_SKILLS);

    let clock = store.spruce_now().await?;
    let tx = store.spruce_db().begin().await?;
    let (binding_ref, skills_id) = store
        .keep_spruce_matching_frozen(&tx, &starting_context.practice, &built_binding, skills)
        .await?;

    let provenance = MessageReviewProvenance {
        recipe_version: MESSAGE_REVIEW_RECIPE_VERSION.to_owned(),
        guidance_version: MESSAGE_REVIEW_GUIDANCE_VERSION.to_owned(),
        guidance_sha256: guidance_sha.clone(),
        model: roles.background.model.clone(),
        effort: roles.background.effort.clone(),
        context_window_tokens: roles.background.context_window_tokens,
        max_input_tokens: roles.background.max_input_tokens,
        adapter: roles.background.adapter.clone(),
        binding: binding_ref,
        skills: skills_id.clone(),
        preloaded_skills: preloaded_digests.clone(),
    };

    let open_patient_count = starting_context
        .patient_messages
        .iter()
        .filter(|m| m.standing.is_open())
        .count();
    let active_memory_count = match &starting_context.memory_standing {
        PatientMemoryStanding::Available { entries } => entries.len(),
        PatientMemoryStanding::Unavailable { .. } => 0,
    };

    let mut initial_loads = vec![MessageReviewInitialLoad::ResidentGuidance {
        version: MESSAGE_REVIEW_GUIDANCE_VERSION.to_owned(),
        sha256: guidance_sha,
    }];
    for digest in &preloaded_digests {
        initial_loads.push(MessageReviewInitialLoad::PreloadedSkill {
            name: digest.name.clone(),
            sha256: digest.sha256.clone(),
        });
    }
    initial_loads.push(MessageReviewInitialLoad::SkillCatalogMetadata {
        skills_package_id: skills_id,
        discoverable_skill_names: discoverable,
    });
    initial_loads.push(MessageReviewInitialLoad::LinkedConversationSnapshot {
        conversation_id: starting_context.conversation_id.clone(),
        conversation_version: starting_context.conversation_version,
        open_patient_message_count: open_patient_count,
        practice_message_count: starting_context.practice_messages.len(),
        saw_physician_reply_request: starting_context.saw_physician_reply_request,
    });
    initial_loads.push(MessageReviewInitialLoad::PatientChartSnapshot {
        workspace: starting_context.workspace.clone(),
        source_version: starting_context.source_version,
        chart_record_count: starting_context.chart_record_count,
        active_memory_count,
        active_follow_up_count: starting_context.active_follow_ups.len(),
        open_concern_count: starting_context.open_concerns.len(),
    });
    if !starting_context.coalesced_changed_candidates.is_empty() {
        initial_loads.push(MessageReviewInitialLoad::CoalescedRecordChanges {
            changed_candidates: starting_context.coalesced_changed_candidates.clone(),
        });
    }

    let initial_record = MessageReviewRunRecord {
        practice: starting_context.practice.clone(),
        workspace: starting_context.workspace.clone(),
        conversation_id: starting_context.conversation_id.clone(),
        review_id: review_id.clone(),
        run_id: run_id.clone(),
        attempt_id: attempt_id.clone(),
        status: MessageReviewRunStatus::Running,
        provenance,
        starting_context: starting_context.clone(),
        initial_loads,
        tool_discoveries: Vec::new(),
        events: Vec::new(),
        last_read_conversation_version: starting_context.conversation_version,
        last_pinned_source_version: starting_context.source_version,
        staged_submission: None,
        saved_completion: None,
        final_summary: None,
        started_at: clock,
        completed_at: None,
    };

    runtime_command_record::Entity::insert(runtime_command_record::ActiveModel {
        practice: Set(starting_context.practice.clone()),
        physician: Set(MESSAGE_REVIEW_RUN_ACTOR.to_owned()),
        key: Set(run_id.clone()),
        command: Set(serde_json::to_value(&initial_record)
            .map_err(conversion_error("MessageReviewRunRecord", "json"))?),
        accepted_at: Set(clock),
    })
    .exec_without_returning(&tx)
    .await?;
    tx.commit().await?;

    let mut host = MessageReviewHost {
        store: store.clone(),
        authority,
        practice: starting_context.practice.clone(),
        workspace: starting_context.workspace.clone(),
        conversation_id: starting_context.conversation_id.clone(),
        review_id: review_id.clone(),
        run_id: run_id.clone(),
        attempt_id: attempt_id.clone(),
        committed_outcome: None,
    };

    let source_msg = MessageReviewHost::source_message(&starting_context);
    let accepted_input = engine::AcceptedInput {
        id: format!("accept:{run_id}"),
        items: vec![Item::ProductMessage(source_msg.clone())],
    };
    let mut request = built_binding.request(Vec::new());
    request.items.clear();

    let run_result = async {
        let work = request.freeze()?.recover(
            &attempt_id,
            engine::recovery::Records {
                events: Vec::new(),
                allowance_after: 0,
            },
            engine::recovery::Messages {
                sources: &[source_msg],
                partial: None,
            },
            engine::recovery::Inputs {
                accepted: vec![accepted_input],
                source_feedback: None,
            },
        )?;
        work.run(provider, &mut host, limits, cancel).await
    }
    .await;

    if let Err(err) = run_result {
        let _ = store
            .restart_reply_review(&starting_context.practice, conversation_id, &run_id)
            .await;
        let fail_clock = store.spruce_now().await?;
        let tx = store.spruce_db().begin().await?;
        if let Some(locked) = runtime_command_record::Entity::find_by_id((
            starting_context.practice.clone(),
            MESSAGE_REVIEW_RUN_ACTOR.to_owned(),
            run_id.clone(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        {
            let mut rec = decode_message_review_run(locked.clone())?;
            if rec.status == MessageReviewRunStatus::Running {
                rec.status = MessageReviewRunStatus::Failed;
                rec.completed_at = Some(fail_clock);
                let mut active: runtime_command_record::ActiveModel = locked.into();
                active.command = Set(serde_json::to_value(&rec)
                    .map_err(conversion_error("MessageReviewRunRecord", "json"))?);
                active.update(&tx).await?;
            }
        }
        tx.commit().await?;
        return Err(MessageReviewRecipeError::Harness {
            review_id: Some(review_id),
            run_id,
            source: err,
        });
    }

    let persisted = store
        .message_review_run(&starting_context.practice, &run_id)
        .await?
        .ok_or_else(|| error("completed message review run missing"))?;
    let Some(outcome) = host.committed_outcome else {
        let _ = store
            .restart_reply_review(&starting_context.practice, conversation_id, &run_id)
            .await;
        return Err(MessageReviewRecipeError::HeldWithoutDecision { review_id, run_id });
    };

    Ok(CompletedMessageReviewRun {
        run: persisted,
        outcome,
    })
}

fn revision_context_from_claim(claim: &RevisionClaim) -> RevisionContext {
    RevisionContext {
        saved_draft: claim.base_text.clone(),
        instruction: claim.accepted.input.message(),
        expected_draft_revision: claim.accepted.input.expected_revision,
        physician_time_zone: claim.accepted.input.timezone.clone(),
    }
}

fn parse_proposal_draft_chat_scope(chat: &str) -> Option<(String, u64)> {
    let rest = chat.strip_prefix(PROPOSAL_DRAFT_CHAT_PREFIX)?;
    let (conversation_id, reply_id_raw) = rest.rsplit_once(':')?;
    if conversation_id.is_empty()
        || conversation_id.trim() != conversation_id
        || conversation_id.chars().any(char::is_control)
    {
        return None;
    }
    let reply_id = reply_id_raw.parse::<u64>().ok()?;
    (reply_id > 0).then(|| (conversation_id.to_owned(), reply_id))
}

fn extract_skill_view_name(tool_name: &str, raw_arguments: &str) -> Option<String> {
    if tool_name != SKILL_VIEW {
        return None;
    }
    serde_json::from_str::<Value>(raw_arguments)
        .ok()
        .and_then(|v| v.get("name").and_then(Value::as_str).map(str::to_owned))
}

fn record_event_calls(seen_calls: &mut BTreeMap<String, (String, Option<String>)>, event: &Event) {
    match &event.kind {
        EventKind::ToolStarted(call) => {
            let skill = extract_skill_view_name(&call.name, &call.arguments);
            seen_calls.insert(call.id.clone(), (call.name.clone(), skill));
        }
        EventKind::ProviderReturned(turn) => {
            for call in turn.calls() {
                let skill = extract_skill_view_name(&call.name, &call.arguments);
                seen_calls
                    .entry(call.id.clone())
                    .or_insert_with(|| (call.name.clone(), skill));
            }
        }
        EventKind::ProviderStarted { .. }
        | EventKind::ProviderObservation { .. }
        | EventKind::ProviderFailed { .. }
        | EventKind::TransportRetry { .. }
        | EventKind::ToolReturned { .. }
        | EventKind::ProviderBlocked { .. }
        | EventKind::ContextRejected
        | EventKind::Candidate { .. }
        | EventKind::WindowOpened { .. } => {}
    }
}

#[derive(Clone, Debug)]
struct PinnedProposalConversation {
    conversation_id: String,
    reply_id: u64,
    conversation_version: u64,
    patient_messages: Vec<LinkedPatientMessage>,
    practice_messages: Vec<ObservedPracticeMessage>,
}

impl PinnedProposalConversation {
    fn product_message(&self) -> engine::ProductMessage {
        engine::ProductMessage {
            id: format!("reply-proposal-conversation:{}", self.conversation_id),
            role: engine::history::Role::User,
            text: json!({
                "kind": "linked_reply_conversation",
                "conversation_id": self.conversation_id,
                "reply_id": self.reply_id,
                "conversation_version": self.conversation_version,
                "patient_messages": self.patient_messages,
                "practice_messages": self.practice_messages,
            })
            .to_string(),
        }
    }

    fn history_message(&self) -> engine::history::Message<Value> {
        let msg = self.product_message();
        engine::history::Message {
            id: msg.id,
            role: msg.role,
            text: msg.text,
            metadata: json!({
                "conversation_id": self.conversation_id,
                "reply_id": self.reply_id,
                "conversation_version": self.conversation_version,
            }),
        }
    }
}

struct ReplyRevisionHost<'a> {
    store: Store,
    claim: RevisionClaim,
    authority: &'a mut dyn Authority,
    pinned_conversation: Option<PinnedProposalConversation>,
    seen_calls: BTreeMap<String, (String, Option<String>)>,
    reconsiderations: u32,
}

#[async_trait]
impl Product for ReplyRevisionHost<'_> {
    async fn message_sources(
        &mut self,
    ) -> Result<Option<Vec<engine::ProductMessage>>, engine::ProductError> {
        let draft = self
            .store
            .draft(&self.claim.scope)
            .await
            .map_err(persistence_failure)?
            .ok_or("revision draft unavailable")?;
        let mut messages = self
            .store
            .revision_message_sources(&self.claim.scope, &draft)
            .await
            .map_err(persistence_failure)?;
        if let Some(pinned) = &self.pinned_conversation {
            messages.push(pinned.product_message());
        }
        Ok(Some(messages))
    }

    async fn admit_binding(&mut self, recipe: &RequestBinding) -> Result<(), engine::ProductError> {
        self.assert_current().await?;
        self.store
            .admit_revision_binding(&self.claim, recipe)
            .await?;
        Ok(())
    }

    async fn window_seed(&mut self) -> Result<engine::WindowSeed, engine::ProductError> {
        self.assert_current().await?;
        let draft = self
            .store
            .draft(&self.claim.scope)
            .await
            .map_err(persistence_failure)?
            .ok_or("revision draft unavailable")?;
        let note_references =
            spruce_working_note_references(draft.attempts.iter().flat_map(|attempt| {
                attempt
                    .events
                    .iter()
                    .map(|event| (attempt.input_id.as_str(), event))
            }))?;
        let boundary = load_workspace_withdrawal_boundary(
            &self.store,
            &self.claim.scope.practice,
            &self.claim.scope.workspace,
        )
        .await?;
        let mut clean_sources = self.claim.sources.clone();
        retain_non_withdrawn_sources(&mut clean_sources, boundary.as_ref());
        let mut items = Vec::new();
        if let Some(pinned) = &self.pinned_conversation {
            items.push(Item::ProductMessage(pinned.product_message()));
        }
        items.extend(revision_context_from_claim(&self.claim).items(&clean_sources));
        Ok(engine::WindowSeed {
            items,
            note_references,
        })
    }

    fn read_only(&self, name: &str) -> bool {
        chat_read_only(name)
            || is_library_tool(name)
            || is_document_page_tool(name)
            || is_spruce_working_note_tool(name)
    }

    fn read_result(&mut self, id: &str) -> engine::recipe::ToolFuture<Option<String>> {
        let store = self.store.clone();
        let claim = self.claim.clone();
        let pinned_conversation = self.pinned_conversation.clone();
        let id = id.to_owned();
        Box::pin(async move {
            store.assert_revision_current(&claim).await?;
            let draft = store
                .draft(&claim.scope)
                .await?
                .ok_or("revision draft unavailable")?;
            let records = store.revision_records(&claim.scope, &draft).await?;
            let mut messages = store.revision_message_sources(&claim.scope, &draft).await?;
            if let Some(pinned) = &pinned_conversation {
                messages.push(pinned.product_message());
            }
            engine::results::resolve(records.iter(), &id, &messages).map_err(Into::into)
        })
    }

    fn terminal(&self, response: &Turn) -> bool {
        requests_finish(response)
    }

    async fn assert_current(&mut self) -> Result<(), engine::ProductError> {
        self.store
            .authorize_revision(&self.claim.scope, self.authority)
            .await?;
        self.store.assert_revision_current(&self.claim).await
    }

    async fn persist(&mut self, event: Event) -> Result<(), engine::ProductError> {
        record_event_calls(&mut self.seen_calls, &event);
        let clock = self.store.spruce_now().await?;
        if let EventKind::ToolReturned { call_id, result } = &event.kind {
            let (tool_name, discovered_skill_name) =
                if let Some(found) = self.seen_calls.get(call_id).cloned() {
                    found
                } else if let Some(draft) = self.store.draft(&self.claim.scope).await? {
                    let records = self
                        .store
                        .revision_records(&self.claim.scope, &draft)
                        .await?;
                    for (prior_event, _) in &records {
                        record_event_calls(&mut self.seen_calls, prior_event);
                    }
                    self.seen_calls
                        .get(call_id)
                        .cloned()
                        .unwrap_or_else(|| ("unknown".to_owned(), None))
                } else {
                    ("unknown".to_owned(), None)
                };

            let tx = self.store.spruce_db().begin().await?;
            if let Some(row) = runtime_command_record::Entity::find_by_id((
                self.claim.scope.practice.clone(),
                REPLY_REVISION_RUN_ACTOR.to_owned(),
                self.claim.attempt.clone(),
            ))
            .lock_exclusive()
            .one(&tx)
            .await?
            {
                let mut rec = decode_reply_revision_run(row.clone())?;
                let disposition = match result {
                    engine::results::ToolResult::Refused(refusal) => {
                        ToolDiscoveryDisposition::Refused {
                            reason: refusal.reason().to_owned(),
                        }
                    }
                    engine::results::ToolResult::Original(_)
                    | engine::results::ToolResult::History(_)
                    | engine::results::ToolResult::View(_) => ToolDiscoveryDisposition::Returned,
                };
                rec.tool_discoveries.push(RecordedToolDiscovery {
                    ordinal: event.ordinal,
                    call_id: call_id.clone(),
                    tool_name,
                    discovered_skill_name,
                    disposition,
                    recorded_at: clock,
                });
                let mut active: runtime_command_record::ActiveModel = row.into();
                active.command = Set(serde_json::to_value(&rec)
                    .map_err(conversion_error("ReplyRevisionRunRecord", "json"))?);
                active.update(&tx).await?;
            }
            tx.commit().await?;
        }

        self.store
            .record_revision(&self.claim, event)
            .await
            .map_err(persistence_failure)
    }

    fn execute(&mut self, call: &Invocation, arguments: Value) -> engine::recipe::ToolFuture {
        let store = self.store.clone();
        let claim = self.claim.clone();
        let pinned_conversation = self.pinned_conversation.clone();
        let call = call.clone();
        Box::pin(async move {
            store.assert_revision_current(&claim).await?;
            let draft = store
                .draft(&claim.scope)
                .await?
                .ok_or("revision draft unavailable")?;
            let work_records = store.revision_records(&claim.scope, &draft).await?;
            let inv = Invocation {
                id: call.id.clone(),
                name: call.name.clone(),
                arguments,
            };
            let records = draft.attempts.iter().flat_map(|attempt| {
                attempt
                    .events
                    .iter()
                    .map(|event| (attempt.input_id.as_str(), event))
            });
            if let Some(outcome) =
                execute_spruce_working_note(&claim.accepted.input.key, &inv, records)?
            {
                return Ok(outcome);
            }
            if engine::history::is_tool(&inv.name) {
                let mut messages = store.revision_messages(&claim.scope, &draft).await?;
                if let Some(pinned) = &pinned_conversation {
                    messages.push(pinned.history_message());
                }
                return Ok(
                    match engine::history::execute(
                        &inv.name,
                        &inv.arguments,
                        &messages,
                        work_records.iter(),
                    ) {
                        Ok(page) => {
                            ToolOutcome::History(page.map_metadata(|metadata| json!(metadata)))
                        }
                        Err(reason) => ToolOutcome::refused(reason),
                    },
                );
            }
            if inv.name == SKILL_VIEW {
                return Ok(claim.skills.execute(&inv.name, &inv.arguments));
            }
            let boundary = load_workspace_withdrawal_boundary(
                &store,
                &claim.scope.practice,
                &claim.scope.workspace,
            )
            .await?;
            let mut clean_sources = claim.sources.clone();
            retain_non_withdrawn_sources(&mut clean_sources, boundary.as_ref());
            if is_document_page_tool(&inv.name) {
                return Ok(
                    crate::attachments::execute_document_page_read_from_revisions(
                        &inv.arguments,
                        &clean_sources,
                    ),
                );
            }
            if matches!(
                inv.name.as_str(),
                PATIENT_MEMORY_TOOL
                    | COMPLETE_MESSAGE_REVIEW_TOOL
                    | "patient_concern"
                    | "patient_follow_up"
                    | "send_patient_message"
            ) {
                return Ok(ToolOutcome::refused(format!(
                    "`{}` is outside physician reply revision authority",
                    inv.name
                )));
            }
            read_sources(&clean_sources, &claim.scope.workspace, &inv).map_err(Into::into)
        })
    }

    async fn finish(&mut self, response: &Turn) -> Result<Finish, engine::ProductError> {
        if let Err(reason) = revised_text(response) {
            return Ok(revision_feedback(reason));
        }

        if let Some(pinned) = &mut self.pinned_conversation
            && let Some(conv) = self
                .store
                .reply_conversation(&self.claim.scope.practice, &pinned.conversation_id)
                .await
                .map_err(|err| -> engine::Cause { Box::new(err) })?
            && matches!(
                &conv.link,
                ConversationLink::LinkedPatient { workspace } if workspace == &self.claim.scope.workspace
            )
            && matches!(
                &conv.reply_standing,
                ReplyStanding::Ready { proposal } if proposal.reply_id == pinned.reply_id
            )
            && conv.conversation_version != pinned.conversation_version
        {
            let previous_version = pinned.conversation_version;
            pinned.conversation_version = conv.conversation_version;
            pinned.patient_messages.clone_from(&conv.patient_messages);
            pinned.practice_messages.clone_from(&conv.practice_messages);
            self.reconsiderations = self.reconsiderations.saturating_add(1);
            return Ok(Finish::Continue(json!({
                "outcome": "conversation_updated",
                "conversation_id": conv.conversation_id,
                "previous_conversation_version": previous_version,
                "conversation_version": conv.conversation_version,
                "patient_messages": conv.patient_messages,
                "practice_messages": conv.practice_messages,
                "message": "New linked conversation messages arrived while revising this reply. Reconsider affected content and call `finish_reply_revision` again.",
            })));
        }

        match self
            .store
            .commit_revision(&self.claim, response, self.authority)
            .await?
        {
            RevisionCommit::Committed => Ok(Finish::Committed),
            RevisionCommit::SourcesMoved {
                refresh,
                mut sources,
            } => {
                let boundary = load_workspace_withdrawal_boundary(
                    &self.store,
                    &self.claim.scope.practice,
                    &self.claim.scope.workspace,
                )
                .await?;
                retain_non_withdrawn_sources(&mut sources, boundary.as_ref());
                self.reconsiderations = self.reconsiderations.saturating_add(1);
                self.claim.sources = sources;
                self.claim.sources_version = refresh.to_version;
                Ok(Finish::Continue(refresh.result(&self.claim.sources)))
            }
            RevisionCommit::SourcesUnread(references) => {
                self.reconsiderations = self.reconsiderations.saturating_add(1);
                Ok(Finish::Continue(unread_result(&references)))
            }
        }
    }

    async fn finish_partial(
        &mut self,
        response: &Turn,
        _backstops: &[engine::LimitBackstop],
    ) -> Result<(), engine::ProductError> {
        self.store.hold_revision(&self.claim, response).await?;
        Ok(())
    }
}

/// Claims and executes a due physician-requested reply draft revision using the
/// frozen [`reply_revision_recipe`], handling `SourcesMoved` / `SourcesUnread`
/// reconsiderations and preserving physician draft authority on concurrent edits.
#[expect(
    clippy::too_many_arguments,
    reason = "entry point binds store, authority, delivery, lease duration, model roles, skills, provider, limits, and cancellation"
)]
pub async fn run_physician_reply_revision<P: Provider>(
    store: &Store,
    authority: &mut dyn Authority,
    delivery: &crate::runtime::delivery::Delivery,
    lease_seconds: i64,
    roles: &ModelRoles,
    skills: &PhysicianSkills,
    provider: &mut P,
    limits: engine::RunLimits,
    cancel: &engine::Cancellation,
) -> Result<ReplyRevisionRunRecord, MessageReviewRecipeError> {
    let Some(mut claim) = store.claim_revision(delivery, lease_seconds).await? else {
        return Err(MessageReviewRecipeError::RevisionNotClaimable {
            chat: delivery.scope.chat.clone(),
            input_id: delivery.input_id.clone(),
        });
    };

    let (built_recipe, preloaded_digests) = reply_revision_recipe(roles, skills)?;
    if claim.binding.recipe != built_recipe {
        let _ = store
            .settle_revision_attempt(
                &claim,
                AttemptFailure::Failed,
                crate::runtime::delivery::Backoff::HOSTED,
            )
            .await;
        return Err(MessageReviewRecipeError::RevisionRecipeMismatch {
            chat: claim.scope.chat.clone(),
            input_id: claim.accepted.input.key.clone(),
        });
    }

    let boundary =
        load_workspace_withdrawal_boundary(store, &claim.scope.practice, &claim.scope.workspace)
            .await?;
    retain_non_withdrawn_sources(&mut claim.sources, boundary.as_ref());

    let pinned_conversation = if let Some((conversation_id, reply_id)) =
        parse_proposal_draft_chat_scope(&claim.scope.chat)
        && let Some(conv) = store
            .reply_conversation(&claim.scope.practice, &conversation_id)
            .await?
        && matches!(
            &conv.link,
            ConversationLink::LinkedPatient { workspace } if workspace == &claim.scope.workspace
        ) {
        Some(PinnedProposalConversation {
            conversation_id,
            reply_id,
            conversation_version: conv.conversation_version,
            patient_messages: conv.patient_messages,
            practice_messages: conv.practice_messages,
        })
    } else {
        None
    };

    let provenance = ReplyRevisionProvenance {
        recipe_version: REPLY_REVISION_RECIPE_VERSION.to_owned(),
        guidance_version: REPLY_REVISION_GUIDANCE_VERSION.to_owned(),
        guidance_sha256: reply_revision_instructions_sha256(),
        model: roles.background.model.clone(),
        effort: roles.background.effort.clone(),
        context_window_tokens: roles.background.context_window_tokens,
        max_input_tokens: roles.background.max_input_tokens,
        adapter: roles.background.adapter.clone(),
        preloaded_skills: preloaded_digests,
    };

    let clock = store.spruce_now().await?;
    let initial_rec = ReplyRevisionRunRecord {
        practice: claim.scope.practice.clone(),
        physician: claim.scope.physician.clone(),
        workspace: claim.scope.workspace.clone(),
        draft_scope_chat: claim.scope.chat.clone(),
        attempt_id: claim.attempt.clone(),
        input_key: claim.accepted.input.key.clone(),
        base_draft_text: claim.base_text.clone(),
        instruction: claim.accepted.input.instruction.clone(),
        initial_source_version: claim.sources_version,
        committed_source_version: None,
        source_reconsiderations: 0,
        provenance,
        tool_discoveries: Vec::new(),
        revised_text: None,
        status: MessageReviewRunStatus::Running,
        started_at: clock,
        completed_at: None,
    };
    let tx = store.spruce_db().begin().await?;
    runtime_command_record::Entity::insert(runtime_command_record::ActiveModel {
        practice: Set(claim.scope.practice.clone()),
        physician: Set(REPLY_REVISION_RUN_ACTOR.to_owned()),
        key: Set(claim.attempt.clone()),
        command: Set(serde_json::to_value(&initial_rec)
            .map_err(conversion_error("ReplyRevisionRunRecord", "json"))?),
        accepted_at: Set(clock),
    })
    .exec_without_returning(&tx)
    .await?;
    tx.commit().await?;

    let draft = store
        .draft(&claim.scope)
        .await?
        .ok_or_else(|| error("revision draft unavailable"))?;
    let mut messages = store.revision_message_sources(&claim.scope, &draft).await?;
    if let Some(pinned) = &pinned_conversation {
        messages.push(pinned.product_message());
    }
    let records = store.revision_records(&claim.scope, &draft).await?;
    let mut seen_calls = BTreeMap::new();
    for (prior_event, _) in &records {
        record_event_calls(&mut seen_calls, prior_event);
    }
    let source_feedback = draft
        .attempts
        .last()
        .and_then(|attempt| attempt.refreshes.last())
        .map(|refresh| refresh.result(&claim.sources));
    let allowance_after = draft.allowance_after;
    drop(draft);

    let mut initial_items = Vec::new();
    if let Some(pinned) = &pinned_conversation {
        initial_items.push(Item::ProductMessage(pinned.product_message()));
    }
    initial_items.extend(revision_context_from_claim(&claim).items(&claim.sources));

    let work_result = claim
        .binding
        .request(initial_items)
        .freeze()
        .and_then(|frozen| {
            frozen.recover(
                &claim.attempt,
                engine::recovery::Records {
                    events: records.iter().map(Into::into).collect(),
                    allowance_after,
                },
                engine::recovery::Messages {
                    sources: &messages,
                    partial: None,
                },
                engine::recovery::Inputs {
                    accepted: Vec::new(),
                    source_feedback,
                },
            )
        });

    let attempt_id = claim.attempt.clone();
    let practice = claim.scope.practice.clone();
    let mut host = ReplyRevisionHost {
        store: store.clone(),
        claim: claim.clone(),
        authority,
        pinned_conversation,
        seen_calls,
        reconsiderations: 0,
    };

    let run_res = match work_result {
        Ok(work) => work.run(provider, &mut host, limits, cancel).await,
        Err(err) => Err(err),
    };

    let finish_clock = store.spruce_now().await?;
    match run_res {
        Ok(turn) => {
            let committed_text = revised_text(&turn).ok();
            let committed_source_version = host.claim.sources_version;
            let reconsiderations = host.reconsiderations;
            let tx = store.spruce_db().begin().await?;
            let row = runtime_command_record::Entity::find_by_id((
                practice.clone(),
                REPLY_REVISION_RUN_ACTOR.to_owned(),
                attempt_id.clone(),
            ))
            .lock_exclusive()
            .one(&tx)
            .await?
            .ok_or_else(|| error("reply revision run missing"))?;
            let mut rec = decode_reply_revision_run(row.clone())?;
            rec.status = if committed_text.is_some() {
                MessageReviewRunStatus::Completed
            } else {
                MessageReviewRunStatus::Held
            };
            rec.committed_source_version = Some(committed_source_version);
            rec.source_reconsiderations = reconsiderations;
            rec.revised_text = committed_text;
            rec.completed_at = Some(finish_clock);
            let mut active: runtime_command_record::ActiveModel = row.into();
            active.command = Set(serde_json::to_value(&rec)
                .map_err(conversion_error("ReplyRevisionRunRecord", "json"))?);
            active.update(&tx).await?;
            tx.commit().await?;
            Ok(rec)
        }
        Err(err) => {
            let reason = AttemptFailure::of(&err, cancel);
            let settlement = store
                .settle_revision_attempt(&claim, reason, crate::runtime::delivery::Backoff::HOSTED)
                .await?;
            let tx = store.spruce_db().begin().await?;
            if let Some(row) = runtime_command_record::Entity::find_by_id((
                practice.clone(),
                REPLY_REVISION_RUN_ACTOR.to_owned(),
                attempt_id.clone(),
            ))
            .lock_exclusive()
            .one(&tx)
            .await?
            {
                let mut rec = decode_reply_revision_run(row.clone())?;
                rec.status = match settlement {
                    AttemptSettlement::Applied
                    | AttemptSettlement::Failed(_)
                    | AttemptSettlement::Superseded => MessageReviewRunStatus::Failed,
                };
                rec.source_reconsiderations = host.reconsiderations;
                rec.completed_at = Some(finish_clock);
                let mut active: runtime_command_record::ActiveModel = row.into();
                active.command = Set(serde_json::to_value(&rec)
                    .map_err(conversion_error("ReplyRevisionRunRecord", "json"))?);
                active.update(&tx).await?;
            }
            tx.commit().await?;
            Err(MessageReviewRecipeError::Harness {
                review_id: None,
                run_id: attempt_id,
                source: err,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::ModelBinding;
    use crate::runtime::reply::FINISH_TOOL;

    fn load_skills() -> PhysicianSkills {
        PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
            .expect("workspace skills load")
    }

    fn distinct_roles() -> ModelRoles {
        ModelRoles {
            chat: ModelBinding {
                model: "chat-lane-model".into(),
                effort: "low".into(),
                context_window_tokens: 128_000,
                max_input_tokens: 100_000,
                adapter: "openai".into(),
            },
            background: ModelBinding {
                model: "background-lane-model".into(),
                effort: "high".into(),
                context_window_tokens: 272_000,
                max_input_tokens: 250_000,
                adapter: "openai".into(),
            },
        }
    }

    #[test]
    fn message_review_recipe_uses_background_lane_preloads_skills_and_has_unique_tools() {
        let skills = load_skills();
        let roles = distinct_roles();
        let (built, digests) = recipe(&roles, &skills).expect("message review recipe builds");

        assert_eq!(built.model, "background-lane-model");
        assert_eq!(built.context_window_tokens, 272_000);
        assert_eq!(built.max_input_tokens, 250_000);
        assert_eq!(built.options, json!({ "effort": "high" }));
        assert_eq!(digests.len(), 2);
        assert_eq!(digests[0].name, "patient-communication");
        assert_eq!(digests[1].name, "patient-record-retrieval");
        assert!(built.instructions.contains("patient-communication"));
        assert!(built.instructions.contains("patient-record-retrieval"));

        let mut seen = BTreeSet::new();
        for tool in &built.tools {
            let name = tool["name"].as_str().expect("tool name");
            assert!(seen.insert(name.to_owned()), "duplicate tool `{name}`");
            assert!(
                !is_physician_only_tool(name),
                "message review must never offer physician-only tool `{name}`"
            );
        }
        assert!(seen.contains(COMPLETE_MESSAGE_REVIEW_TOOL));
        assert!(seen.contains(REFRESH_TOOL));
        assert!(seen.contains(READ_DOCUMENT_PAGE_TOOL));
        assert!(seen.contains("save_working_note"));
    }

    #[test]
    fn reply_revision_recipe_uses_background_lane_and_stays_within_revision_authority() {
        let skills = load_skills();
        let roles = distinct_roles();
        let (built, digests) =
            reply_revision_recipe(&roles, &skills).expect("reply revision recipe builds");

        assert_eq!(built.model, "background-lane-model");
        assert_eq!(digests.len(), 1);
        assert_eq!(digests[0].name, "patient-communication");

        let mut seen = BTreeSet::new();
        for tool in &built.tools {
            let name = tool["name"].as_str().expect("tool name");
            assert!(seen.insert(name.to_owned()), "duplicate tool `{name}`");
        }
        assert!(seen.contains(FINISH_TOOL));
        assert!(seen.contains(READ_TOOL));
        assert!(seen.contains(READ_DOCUMENT_PAGE_TOOL));
        assert!(!seen.contains(PATIENT_MEMORY_TOOL));
        assert!(!seen.contains("patient_concern"));
        assert!(!seen.contains("patient_follow_up"));
        assert!(!seen.contains(COMPLETE_MESSAGE_REVIEW_TOOL));
    }
}
