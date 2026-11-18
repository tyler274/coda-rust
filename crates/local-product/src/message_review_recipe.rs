//! Linked patient message review and physician-requested reply revision recipes.
//!
//! Owns the background assignment, model settings (`ModelRoles::background`),
//! starting context, resident system prompts, eagerly preloaded skills
//! (`patient-communication`, `patient-record-retrieval`), discoverable workspace
//! capabilities, completion safety and currency barriers, and durable run
//! provenance for linked patient messaging conversations, alongside the
//! physician-requested reply draft revision recipe within the revision
//! operation's authority.

use std::collections::BTreeSet;

use harness::engine::recipe::RequestRecipe;
use harness::engine::{self, Event, Tool, ToolOutcome};
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::ModelRoles;
use crate::attachments::document_page_tool;
use crate::initial_understanding::understanding_tools;
use crate::patient_reviews::read_patient_reviews_tool;
use crate::physician_chat::{PhysicianSkills, SKILL_VIEW, work_tools};
use crate::physician_sources::{ChartScope, SourcePatient, SourceRevision, read_tool, search_tool};
use crate::runtime::chat_operations::{
    DispatchCheckStanding, OpenConcernSummary, PatientMemoryStanding, PatientStartingContext,
    PriorWorkKind, PriorWorkPointer, SourceFreshnessSummary, retrieve_patient_work_tool,
};
use crate::runtime::concerns::{
    ConcernError, ConcernId, ConcernUrgency, ExactRecordReference, concern_tool,
};
use crate::runtime::follow_ups::{
    FollowUp, FollowUpError, FollowUpStanding, WorkspaceWithdrawalBoundary, follow_up_tool,
};
use crate::runtime::jobs::{AssignmentError, PracticeAuthority, PracticeScope};
use crate::runtime::refresh::{SavedSourceStatus, refresh_tool};
use crate::runtime::reply::proposals::{
    AnsweredMessageClosure, ConversationLink, LinkedPatientMessage, MessageSender,
    ObservedPracticeMessage, PracticeMessageKind, ReplyProposalError, ReplyReviewOutcome,
    ReplyReviewSnapshot, ReplyStanding, ReviewReadState, SavedReviewCompletion,
};
use crate::runtime::reply::tools as reply_revision_runtime_tools;
use crate::runtime::spruce_matching::{
    SpruceAttachmentSummary, spruce_conversion_error as conversion_error, spruce_db_error as error,
};
use crate::runtime::understanding::PatientUnderstanding;
use crate::runtime::{BindingRef, SkillPackageId, Store, WorkStatus};

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
    understanding_withdrawal_record,
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
            PatientMemoryStanding::Available {
                summary_markdown,
                direction_markdown,
                has_person_page,
                page_paths,
                log_days,
            } => {
                if summary_markdown.is_none()
                    && direction_markdown.is_none()
                    && !*has_person_page
                    && page_paths.is_empty()
                    && log_days.is_empty()
                {
                    lines.push("Active patient memory: none.".to_owned());
                } else {
                    lines.push("Active patient memory (runtime::memory):".to_owned());
                    if let Some(summary) = summary_markdown {
                        lines.push(format!("- summary.md:\n{summary}"));
                    }
                    if let Some(direction) = direction_markdown {
                        lines.push(format!("- direction.md:\n{direction}"));
                    }
                    if *has_person_page {
                        lines.push("- person.md: present".to_owned());
                    }
                    if !page_paths.is_empty() {
                        lines.push(format!("- pages: {}", page_paths.join(", ")));
                    }
                    if !log_days.is_empty() {
                        lines.push(format!("- log days: {}", log_days.join(", ")));
                    }
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

/// Lists discoverable workspace skills available on demand beyond `preloaded`.
pub fn discoverable_skill_names(skills: &PhysicianSkills, preloaded: &[&str]) -> Vec<String> {
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
    let mut instructions = skills.instructions(
        MESSAGE_REVIEW_INSTRUCTIONS,
        crate::physician_chat::Job::MessageReview,
    );
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
    let mut instructions = skills.instructions(
        REPLY_REVISION_INSTRUCTIONS,
        crate::physician_chat::Job::ReplyRevision,
    );
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::ModelBinding;
    use crate::attachments::READ_DOCUMENT_PAGE_TOOL;
    use crate::physician_sources::READ_TOOL;
    use crate::runtime::jobs::is_physician_only_tool;
    use crate::runtime::refresh::REFRESH_TOOL;
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
        assert!(!seen.contains("patient_memory"));
        assert!(!seen.contains("patient_concern"));
        assert!(!seen.contains("patient_follow_up"));
        assert!(!seen.contains(COMPLETE_MESSAGE_REVIEW_TOOL));
    }
}
