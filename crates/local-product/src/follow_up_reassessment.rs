//! Patient follow-up reassessment recipe, starting context composition, and
//! shared-harness execution on `ModelRoles::background`.
//!
//! Composes each open follow-up responsibility — what it watches for, why it
//! exists, its operational constraints, and its full change history — with the
//! shared longitudinal patient review context, preloading `patient-follow-up`
//! and `patient-record-retrieval` while keeping workspace capabilities
//! discoverable on demand. Evaluates four structured dispositions (`continue
//! unchanged`, `change next reassessment`, `complete`, and `end without
//! completion`), enforces chart-evidence currency and sibling-responsibility
//! isolation, and records durable outcomes across follow-ups, follow-up
//! updates, saved patient reviews, and Today / Updates.

#![expect(
    dead_code,
    reason = "FollowUpReassessmentHost Product execution lands in companion slice"
)]
#![expect(
    unused_imports,
    reason = "harness Product execution imports are exercised in companion slice"
)]

use std::collections::{BTreeMap, BTreeSet};

use async_trait::async_trait;
use harness::engine::recipe::{Finish, Provider, RequestBinding, RequestRecipe};
use harness::engine::{self, Event, EventKind, Invocation, Item, Product, Tool, ToolOutcome, Turn};
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
    ConnectedSourceBrief, EvidenceScope, SAVE_UNDERSTANDING_TOOL, is_understanding_tool,
    understanding_tools,
};
use crate::message_review_recipe::{
    AdmittedPracticePatient, MessageReviewRunStatus, PreloadedSkillDigest, RecordedToolDiscovery,
    ToolDiscoveryDisposition, deduplicate_and_verify_unique_tools, discoverable_skill_names,
    execute_background_memory_tool, load_clean_sources, preload_skills_from_catalog,
    retain_non_withdrawn_sources,
};
use crate::patient_reviews::{
    PriorPatientReviewSummary, QuietReviewReason, READ_PATIENT_REVIEWS_TOOL, ReadReviewsCall,
    ReviewCitation, ReviewOutcome, ReviewStartingContext, ReviewTrigger, UnfinishedReviewWork,
    read_only_review_tool, read_patient_reviews_tool,
};
use crate::physician_chat::{PhysicianSkills, SKILL_VIEW, read_only as chat_read_only, work_tools};
use crate::physician_sources::{
    ChartScope, READ_TOOL, SEARCH_TOOL, SourceRevision, read_tool, search_tool,
};
use crate::runtime::chat_operations::{
    MemoryProvenance, MemorySelection, PATIENT_MEMORY_TOOL, PatientMemoryError,
    PatientMemoryStanding, RETRIEVE_PATIENT_WORK_TOOL, SourceFreshnessSummary, patient_memory_tool,
    retrieve_patient_work_tool,
};
use crate::runtime::concerns::{
    ConcernUrgency, ExactRecordReference, SourceWork, concern_tool, is_concern_tool,
};
use crate::runtime::follow_ups::{
    FOLLOW_UP_TOOL, FollowUp, FollowUpActor, FollowUpChangeNote, FollowUpCommandKey, FollowUpError,
    FollowUpId, FollowUpReviewOutcome, FollowUpReviewSchedule, FollowUpStanding, FollowUpSummary,
    FollowUpToolCall, FollowUpUpdateEntry, WorkspaceWithdrawalBoundary, follow_up_tool,
    is_follow_up_tool,
};
use crate::runtime::jobs::{
    AssignmentError, PracticeAccessError, PracticeAuthority, PracticeScope, is_physician_only_tool,
};
use crate::runtime::refresh::{is_refresh_tool, refresh_tool};
use crate::runtime::reviews::{
    PatientReviewError, RecordedReviewAction, ReviewLifecycleState, ReviewRecipeVersion,
    ReviewSkillDigest, ReviewUsage, SavedPatientReview,
};
use crate::runtime::spruce_matching::{
    execute_spruce_working_note, is_spruce_working_note_tool,
    spruce_conversion_error as conversion_error, spruce_db_error as error,
    spruce_working_note_references,
};
use crate::runtime::{BindingRef, Scope, SkillPackageId, Store, persistence_failure};

/// Immutable recipe identifier recorded with every follow-up reassessment run.
pub const FOLLOW_UP_REASSESSMENT_RECIPE_VERSION: &str = "follow-up-reassessment/v1";

/// Immutable resident guidance version recorded with every follow-up reassessment run.
pub const FOLLOW_UP_REASSESSMENT_GUIDANCE_VERSION: &str = "follow-up-reassessment-guidance/v1";

/// Structured completion barrier tool submitting a follow-up reassessment disposition.
pub const COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL: &str = "complete_follow_up_reassessment";

/// Skills eagerly preloaded at the start of every follow-up reassessment run.
pub const PRELOADED_FOLLOW_UP_REASSESSMENT_SKILLS: [&str; 2] =
    ["patient-follow-up", "patient-record-retrieval"];

const FOLLOW_UP_REASSESSMENT_RUN_ACTOR: &str = "__follow_up_reassessment_run__";
const MAX_REASSESSMENT_TEXT_CHARS: usize = 8_000;
const EXCERPT_MAX_CHARS: usize = 280;

/// Resident clinical instructions for reassessing one open patient follow-up on
/// the background lane.
pub const FOLLOW_UP_REASSESSMENT_INSTRUCTIONS: &str = "\
You are Coda reassessing one open patient follow-up responsibility for a practice in Chords. \
No physician is signed in to this background run; you act strictly as a clinical decision-support \
analyst, never entering orders, sending patient messages, or treating unapproved care changes as \
an active plan.

Own the clinical reassessment of the target follow-up responsibility:

1. Orient to the responsibility and longitudinal patient picture:
   - Read the target follow-up responsibility in the starting context: what it watches for, when \
     it became due, why it was created, its operational constraints, and its chronological update \
     history.
   - Review the shared patient context alongside it: patient identity, chart freshness, saved \
     patient understanding, active durable patient memory (distinguishing physician corrections \
     from clinical summaries), open concerns on Today, prior completed reviews, and any sibling \
     follow-ups on the same patient. Keep sibling follow-ups intact and focused on their own \
     responsibilities.

2. Gather dated chart evidence before deciding:
   - Search with `search_patient_information`, read full source revisions with `read_patient_sources`, \
     and inspect paged clinical documents with `read_document_page` before citing records or \
     concluding that an expected result arrived or changed.
   - When recency matters or a connected source could hold newly delivered external labs, imaging, \
     consult notes, or medication updates, call `refresh_patient_sources` and read any newly \
     delivered revisions.
   - Use `search_clinical_library` and `read_clinical_library` with clinical concepts only (never \
     patient identifiers) when literature or guideline intervals inform the reassessment, and open \
     other catalog skills via `skill_view` when relevant.

3. Choose the clinically warranted disposition through `complete_follow_up_reassessment`:
   - `continue_unchanged`: keep the responsibility active on its current schedule when expected \
     evidence is still pending within a reasonable window or current evidence leaves the watch \
     unchanged. Set `quiet: true` when nothing requires physician attention today, or `quiet: false` \
     when surfacing an assessed update or open concern.
   - `change_next_reassessment`: revise the summary, next `next_due_at` Unix timestamp, cited \
     chart records, and single-line `change_note` when new clinical evidence, overdue timing, or a \
     physician correction shifts what to watch for or when to check next.
   - `complete`: mark the responsibility completed with a single-line `resolution_note` ONLY when \
     verified chart evidence read in this run or an active physician memory correction resolves \
     what the follow-up was watching for.
   - `end_without_completion`: end and cancel the responsibility with an explicit single-line \
     `ending_reason` when the clinical watch is obsolete, superseded, or transferred elsewhere \
     without clinical completion. Never claim clinical completion when ending without completion.

4. Handle missing evidence, patient silence, overdue work, and conflicting records honestly:
   - Patient silence, an unanswered message, a missing external report, an unreadable document, or \
     a failed source check never implies clinical resolution or normal results. Record explicit \
     uncertainty in `explicit_uncertainty` and `basis_and_limitations`.
   - When expected high-stakes work is overdue or new abnormal results warrant physician review, \
     raise or revise a calibrated `patient_concern` (`today` or `now`) citing the exact supporting \
     records.
   - When dated records conflict across sources or revisions, read both records in full, cite both \
     revisions, and state the discrepancy plainly rather than silently choosing one source.
   - On longitudinal reassessments where nothing material has changed and an open concern or \
     active follow-up already represents the issue, continue quietly without raising duplicate \
     concerns.";

mod runtime_command_record {
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

mod saved_review_record {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_patient_reviews")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub practice: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub workspace: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub review_id: String,
        #[sea_orm(column_type = "Text")]
        pub command_key: String,
        #[sea_orm(column_type = "Text")]
        pub job_id: String,
        #[sea_orm(column_type = "Text")]
        pub work_id: String,
        #[sea_orm(column_type = "Text")]
        pub status: String,
        #[sea_orm(column_type = "JsonBinary")]
        pub patient_snapshot: Option<Json>,
        #[sea_orm(column_type = "JsonBinary")]
        pub triggers: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub accepted_inputs: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub recipe_version: Json,
        pub initial_source_version: i64,
        #[sea_orm(column_type = "JsonBinary")]
        pub initial_sources: Json,
        pub completed_source_version: Option<i64>,
        #[sea_orm(column_type = "JsonBinary")]
        pub completed_sources: Option<Json>,
        #[sea_orm(column_type = "JsonBinary")]
        pub evidence_read: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub actions: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub unfinished_work: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub state: Json,
        #[sea_orm(column_type = "JsonBinary")]
        pub usage: Json,
        pub duration_ms: Option<i64>,
        #[sea_orm(column_type = "JsonBinary")]
        pub staged_completion: Option<Json>,
        pub queued_at: i64,
        pub started_at: Option<i64>,
        pub completed_at: Option<i64>,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

mod workspace_concern_record {
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

mod follow_up_record {
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

#[derive(Debug, thiserror::Error)]
#[error("follow-up `{follow_up_id}` transitioned to `{standing}` during reassessment")]
struct MidRunFollowUpIneligible {
    follow_up_id: FollowUpId,
    standing: &'static str,
}

fn extract_mid_run_ineligible(failure: &engine::Failure) -> Option<&MidRunFollowUpIneligible> {
    match failure {
        engine::Failure::Host(cause) => cause.downcast_ref::<MidRunFollowUpIneligible>(),
        engine::Failure::Final(inner) => extract_mid_run_ineligible(inner),
        engine::Failure::Cancelled
        | engine::Failure::NotCurrent(_)
        | engine::Failure::Limit(_)
        | engine::Failure::ContextRefused
        | engine::Failure::Protocol(_)
        | engine::Failure::Provider(_)
        | engine::Failure::AfterOutput(_)
        | engine::Failure::Encoding(_) => None,
    }
}

/// Closed event that started a follow-up reassessment run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FollowUpReassessmentTrigger {
    /// Scheduled due wake-up selected an active due follow-up at `clinical_now`.
    DueWakeup {
        /// Practice clinical Unix timestamp when the due sweep ran.
        clinical_now: i64,
    },
    /// Signed-in physician requested an immediate reassessment (`Review now`).
    PhysicianReviewNow {
        /// Requesting physician identifier.
        physician_id: String,
    },
}

impl FollowUpReassessmentTrigger {
    /// Short trigger kind label (`"due_wakeup"` or `"physician_review_now"`).
    #[must_use]
    pub const fn kind_label(&self) -> &'static str {
        match self {
            Self::DueWakeup { .. } => "due_wakeup",
            Self::PhysicianReviewNow { .. } => "physician_review_now",
        }
    }
}

/// Invariant clinical resolution policy for follow-up reassessments:
/// completion requires verified chart evidence or an active physician memory
/// correction, patient silence or missing external records never imply
/// resolution, and ending without completion cancels tracking without claiming
/// clinical resolution.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FollowUpResolutionPolicy {
    /// Completion requires verified chart evidence; silence never implies resolution;
    /// ending without completion cancels tracking rather than claiming resolution.
    VerifiedEvidenceOnly,
}

/// Operational and clinical safety constraints carried on a follow-up reassessment.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowUpResponsibilityConstraints {
    /// Clinical resolution policy governing completion and cancellation.
    pub resolution_policy: FollowUpResolutionPolicy,
    /// Sibling follow-up identifiers on the same patient that must remain intact.
    pub sibling_follow_up_ids: Vec<FollowUpId>,
    /// Active wrong-patient source withdrawal boundary on the workspace, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withdrawal_boundary: Option<WorkspaceWithdrawalBoundary>,
}

/// Target follow-up responsibility, origin, constraints, history, and sibling context.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetFollowUpResponsibility {
    /// Current durable follow-up snapshot at admission.
    pub follow_up: FollowUp,
    /// Physician-legible explanation of why this responsibility exists and who created it.
    pub why_it_exists: String,
    /// Authoring actor recorded at creation.
    pub origin: FollowUpActor,
    /// Operational constraints governing this reassessment.
    pub constraints: FollowUpResponsibilityConstraints,
    /// Chronological change history for this follow-up (`Created`, `Changed`, `Resumed`, `ReviewRequestedNow`, etc.).
    pub update_history: Vec<FollowUpUpdateEntry>,
    /// Other non-terminal follow-ups on this patient workspace, shown read-only for longitudinal awareness.
    pub sibling_follow_ups: Vec<FollowUp>,
}

/// Composed starting context for a follow-up reassessment run, pairing the
/// target responsibility with the shared [`ReviewStartingContext`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowUpReassessmentStartingContext {
    /// Practice tenant identifier.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Saved patient review identifier linked to this reassessment run.
    pub review_id: String,
    /// Trigger that initiated this reassessment.
    pub trigger: FollowUpReassessmentTrigger,
    /// Target follow-up responsibility, why it exists, constraints, and update history.
    pub responsibility: TargetFollowUpResponsibility,
    /// Shared patient review starting context (patient identity, chart freshness, understanding, memory, concerns, prior reviews).
    pub review_context: ReviewStartingContext,
}

impl FollowUpReassessmentStartingContext {
    /// Renders the model-facing user message combining the target follow-up
    /// responsibility with the shared patient review context.
    #[must_use]
    pub fn render_prompt(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "Reassess open patient follow-up [{}] in workspace {}.",
            self.responsibility.follow_up.id, self.workspace
        ));
        let trigger_line = match &self.trigger {
            FollowUpReassessmentTrigger::DueWakeup { clinical_now } => format!(
                "Reassessment trigger: scheduled due wake-up at unix {clinical_now} (target due_at unix {}).",
                self.responsibility.follow_up.due_at
            ),
            FollowUpReassessmentTrigger::PhysicianReviewNow { physician_id } => format!(
                "Reassessment trigger: physician {physician_id} requested immediate reassessment (Review now; target due_at unix {}).",
                self.responsibility.follow_up.due_at
            ),
        };
        lines.push(trigger_line);
        lines.push(format!(
            "Current responsibility (version {}): {}",
            self.responsibility.follow_up.version,
            self.responsibility.follow_up.summary.as_str()
        ));
        lines.push(format!(
            "Why this responsibility exists: {}",
            self.responsibility.why_it_exists
        ));
        if self.responsibility.follow_up.records.is_empty() {
            lines.push("Baseline cited chart records on responsibility: none.".to_owned());
        } else {
            let refs = self
                .responsibility
                .follow_up
                .records
                .iter()
                .map(|r| format!("{}@{}", r.candidate_id, r.revision))
                .collect::<Vec<_>>()
                .join(", ");
            lines.push(format!(
                "Baseline cited chart records on responsibility: {refs}"
            ));
        }

        lines.push(
            "Operational constraints: completion requires verified chart evidence or an active physician memory correction; patient silence or missing external records never imply resolution; ending without completion cancels tracking without claiming clinical completion; unrelated sibling follow-ups on this patient must remain intact."
                .to_owned(),
        );

        if self.responsibility.update_history.is_empty() {
            lines.push("Follow-up update history: none recorded.".to_owned());
        } else {
            lines.push(format!(
                "Follow-up update history ({} entry/entries, oldest first):",
                self.responsibility.update_history.len()
            ));
            for entry in &self.responsibility.update_history {
                lines.push(format!(
                    "- v{} at unix {} ({:?}, due unix {}): {}",
                    entry.version, entry.occurred_at, entry.kind, entry.due_at, entry.detail
                ));
            }
        }

        if self.responsibility.sibling_follow_ups.is_empty() {
            lines.push("Other open follow-ups on this patient: none.".to_owned());
        } else {
            lines.push(format!(
                "Other open follow-ups on this patient ({} item(s), keep intact):",
                self.responsibility.sibling_follow_ups.len()
            ));
            for sibling in &self.responsibility.sibling_follow_ups {
                lines.push(format!(
                    "- [{}] (v{}, due unix {}, status {}): {}",
                    sibling.id,
                    sibling.version,
                    sibling.due_at,
                    sibling.standing.status_str(),
                    sibling.summary.as_str()
                ));
            }
        }

        lines.push(String::new());
        lines.push("Shared patient review context:".to_owned());
        lines.push(self.review_context.developer_text());
        lines.join("\n")
    }
}

/// Exact recipe, guidance, model-binding, and skill-package provenance recorded
/// with every follow-up reassessment run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowUpReassessmentProvenance {
    /// Recipe version string (`FOLLOW_UP_REASSESSMENT_RECIPE_VERSION`).
    pub recipe_version: String,
    /// Resident guidance version string (`FOLLOW_UP_REASSESSMENT_GUIDANCE_VERSION`).
    pub guidance_version: String,
    /// SHA-256 hex digest of [`FOLLOW_UP_REASSESSMENT_INSTRUCTIONS`].
    pub instructions_sha256: String,
    /// Background model name from `ModelRoles::background`.
    pub model: String,
    /// Reasoning effort from `ModelRoles::background`.
    pub effort: String,
    /// Working context window ceiling in tokens.
    pub context_window_tokens: u64,
    /// Provider input ceiling in tokens.
    pub max_input_tokens: u64,
    /// Immutable frozen binding reference in Postgres.
    pub binding: BindingRef,
    /// Immutable frozen skill catalog package digest in Postgres.
    pub skills_package: SkillPackageId,
    /// Skills preloaded eagerly into the run's instructions.
    pub preloaded_skills: Vec<PreloadedSkillDigest>,
    /// Remaining catalog skills left discoverable via `skill_view`.
    pub discoverable_skills: Vec<String>,
    /// Tool names offered in the bound recipe.
    pub offered_tools: Vec<String>,
}

/// Items loaded eagerly into the reassessment run before the first model turn,
/// recorded separately from later `tool_discoveries`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FollowUpReassessmentInitialLoad {
    /// Skill preloaded into the resident instructions with its exact SHA-256 digest.
    PreloadedSkill {
        /// Skill catalog name.
        name: String,
        /// Lowercase SHA-256 hex digest of the `SKILL.md` content.
        sha256: String,
    },
    /// Target follow-up responsibility and its change history loaded at admission.
    TargetResponsibility {
        /// Reassessed follow-up identifier.
        follow_up_id: FollowUpId,
        /// Follow-up version at admission.
        version: u64,
        /// Target due timestamp at admission.
        due_at: i64,
        /// Count of historical `FollowUpUpdateEntry` records loaded.
        update_history_count: usize,
    },
    /// Sibling follow-ups loaded read-only so reassessment preserves unrelated items.
    SiblingResponsibilities {
        /// Number of sibling non-terminal follow-ups loaded.
        count: usize,
        /// Identifiers of those sibling follow-ups.
        follow_up_ids: Vec<FollowUpId>,
    },
    /// Shared longitudinal patient review context composed into the run start.
    SharedReviewContext {
        /// Workspace chart version observed at run start.
        chart_version: u64,
        /// Count of retained chart records observed at run start.
        chart_record_count: usize,
        /// Active `PatientUnderstanding` revision when present.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        understanding_revision: Option<u64>,
        /// Count of active durable patient memory entries loaded.
        active_memory_count: usize,
        /// Count of open concerns on Today loaded.
        open_concern_count: usize,
        /// Count of prior completed patient reviews summarized in starting context.
        prior_review_count: usize,
    },
}

/// Closed 4-way reassessment decision submitted to [`COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SubmittedFollowUpDecision {
    /// Keep the responsibility active on its current schedule without changing `due_at` or `summary`.
    ContinueUnchanged {
        /// Clinical explanation of why the responsibility continues on its current schedule.
        rationale: String,
        /// True when nothing requires physician attention today (`ReviewOutcome::Quiet`).
        quiet: bool,
    },
    /// Revise what the responsibility watches for or when its next reassessment is due.
    ChangeNextReassessment {
        /// Updated clinical summary statement.
        summary: String,
        /// Next scheduled reassessment Unix timestamp (`> 0`).
        next_due_at: i64,
        /// Single-line explanation recorded in follow-up `Updates`.
        change_note: String,
        /// Clinical synthesis supporting the revised watch or schedule.
        rationale: String,
    },
    /// Complete the responsibility based on verified resolving evidence.
    Complete {
        /// Single-line clinical resolution note recorded in follow-up `Updates`.
        resolution_note: String,
        /// Clinical synthesis explaining how the verified evidence resolves the follow-up.
        rationale: String,
    },
    /// End and cancel the responsibility without claiming clinical completion.
    EndWithoutCompletion {
        /// Single-line non-completion ending reason recorded as cancellation in follow-up `Updates`.
        ending_reason: String,
        /// Clinical and operational explanation of why tracking ended without completion.
        rationale: String,
    },
}

/// Staged reassessment completion validated at tool time and committed at `finish`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StagedFollowUpReassessment {
    /// Tool invocation identifier that staged this decision.
    pub call_id: String,
    /// Validated 4-way decision.
    pub decision: SubmittedFollowUpDecision,
    /// Read-verified chart record references supporting this reassessment.
    pub cited_records: Vec<ExactRecordReference>,
    /// Dated sources, guidance used, and explicit limitations.
    pub basis_and_limitations: String,
    /// Explicit clinical or operational uncertainties (missing results, silence, conflicts).
    pub explicit_uncertainty: Vec<String>,
    /// Unfinished follow-up items carried forward onto the saved patient review.
    pub unfinished_work: Vec<UnfinishedReviewWork>,
    /// Workspace chart version observed when staged.
    pub staged_chart_version: u64,
}

/// Durable committed outcome of a follow-up reassessment run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome_kind", rename_all = "snake_case")]
pub enum SavedFollowUpReassessmentOutcome {
    /// Responsibility remained active on its existing schedule and version.
    ContinuedUnchanged {
        /// Current unchanged follow-up snapshot.
        follow_up: FollowUp,
        /// Clinical rationale recorded for continuing unchanged.
        rationale: String,
        /// Whether this continuation finished quietly (`"nothing to do"`).
        quiet: bool,
        /// Explicit uncertainties recorded during reassessment.
        explicit_uncertainty: Vec<String>,
        /// Saved patient review identifier recording this reassessment.
        review_id: String,
    },
    /// Responsibility's next reassessment schedule or summary was revised (`FollowUpUpdateKind::Changed`).
    ChangedNextReassessment {
        /// Updated follow-up snapshot in `Active` standing.
        follow_up: FollowUp,
        /// Single-line change note recorded in follow-up `Updates`.
        change_note: String,
        /// Clinical rationale recorded for the schedule/summary change.
        rationale: String,
        /// Explicit uncertainties recorded during reassessment.
        explicit_uncertainty: Vec<String>,
        /// Saved patient review identifier recording this reassessment.
        review_id: String,
    },
    /// Responsibility was clinically completed (`FollowUpStanding::Completed`) with verified evidence.
    Completed {
        /// Completed follow-up snapshot.
        follow_up: FollowUp,
        /// Single-line clinical resolution note.
        resolution_note: String,
        /// Clinical rationale recorded for completion.
        rationale: String,
        /// Saved patient review identifier recording this reassessment.
        review_id: String,
    },
    /// Responsibility was ended without clinical completion (`FollowUpStanding::Cancelled`).
    EndedWithoutCompletion {
        /// Cancelled follow-up snapshot.
        follow_up: FollowUp,
        /// Single-line cancellation reason explaining why tracking ended without completion.
        ending_reason: String,
        /// Clinical and operational rationale.
        rationale: String,
        /// Structural invariant: ending without completion never claims clinical completion (`false`).
        claims_clinical_completion: bool,
        /// Explicit uncertainties recorded during reassessment.
        explicit_uncertainty: Vec<String>,
        /// Saved patient review identifier recording this reassessment.
        review_id: String,
    },
}

impl SavedFollowUpReassessmentOutcome {
    /// Resulting durable [`FollowUp`] snapshot after reassessment committed.
    #[must_use]
    pub const fn follow_up(&self) -> &FollowUp {
        match self {
            Self::ContinuedUnchanged { follow_up, .. }
            | Self::ChangedNextReassessment { follow_up, .. }
            | Self::Completed { follow_up, .. }
            | Self::EndedWithoutCompletion { follow_up, .. } => follow_up,
        }
    }

    /// Whether this outcome claims clinical completion (`true` ONLY for [`Self::Completed`]).
    #[must_use]
    pub const fn claims_clinical_completion(&self) -> bool {
        match self {
            Self::Completed { .. } => true,
            Self::ContinuedUnchanged { .. }
            | Self::ChangedNextReassessment { .. }
            | Self::EndedWithoutCompletion { .. } => false,
        }
    }
}

/// Durable audit and execution record for one follow-up reassessment run.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FollowUpReassessmentRunRecord {
    /// Unique reassessment run identifier (`work_id`).
    pub run_id: String,
    /// Harness attempt identifier.
    pub attempt_id: String,
    /// Linked saved patient review identifier in `physician_runtime_patient_reviews`.
    pub review_id: String,
    /// Practice tenant identifier.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Target follow-up identifier.
    pub follow_up_id: FollowUpId,
    /// Follow-up version observed when the run started.
    pub initial_follow_up_version: u64,
    /// Latest active/retired patient memory revision sum observed when the run started.
    pub initial_memory_fingerprint: String,
    /// Run status (`Running`, `Completed`, `Held`, or `Failed`).
    pub status: MessageReviewRunStatus,
    /// Recipe, guidance, model, and skill provenance.
    pub provenance: FollowUpReassessmentProvenance,
    /// Eagerly loaded skills, target responsibility, siblings, and shared review context.
    pub initial_loads: Vec<FollowUpReassessmentInitialLoad>,
    /// Capabilities discovered and invoked during the run (`skill_view`, chart reads, library, concerns, etc.).
    pub tool_discoveries: Vec<RecordedToolDiscovery>,
    /// Assembled starting context delivered as the initial user prompt.
    pub starting_context: FollowUpReassessmentStartingContext,
    /// Initial workspace chart version observed at run start.
    pub initial_chart_version: u64,
    /// Initial chart sources snapshot observed at run start.
    pub initial_sources: Vec<SourceRevision>,
    /// Admitted harness execution events.
    pub events: Vec<Event>,
    /// Latest validated staged completion, when submitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staged: Option<StagedFollowUpReassessment>,
    /// Committed reassessment outcome, present iff `status == Completed`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saved_outcome: Option<SavedFollowUpReassessmentOutcome>,
    /// Final model turn summary text, when finished.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub final_summary: Option<String>,
    /// Database-clock Unix timestamp when started.
    pub started_at: i64,
    /// Database-clock Unix timestamp when settled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
}

/// Completed follow-up reassessment bundle returned by [`run_follow_up_reassessment`].
#[derive(Clone, Debug)]
pub struct CompletedFollowUpReassessment {
    /// Durable reassessment run record including `provenance`, `initial_loads`, and `tool_discoveries`.
    pub run: FollowUpReassessmentRunRecord,
    /// Committed follow-up reassessment outcome.
    pub outcome: SavedFollowUpReassessmentOutcome,
    /// Saved patient review record persisted in `physician_runtime_patient_reviews`.
    pub patient_review: SavedPatientReview,
}

/// Request parameters for [`run_follow_up_reassessment`].
#[derive(Clone, Debug)]
pub struct FollowUpReassessmentRequest {
    /// Practice tenant identifier.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Target follow-up identifier to reassess.
    pub follow_up_id: FollowUpId,
    /// Closed trigger (`DueWakeup` or `PhysicianReviewNow`).
    pub trigger: FollowUpReassessmentTrigger,
    /// Optional deterministic review identifier (generated when `None`).
    pub review_id: Option<String>,
}

/// Errors produced when admitting or executing a follow-up reassessment run.
#[derive(Debug, thiserror::Error)]
pub enum FollowUpReassessmentError {
    /// Practice patient authorization failed.
    #[error("practice patient access refused: {0}")]
    Access(#[from] PracticeAccessError),
    /// Preloaded skill or assignment validation failed.
    #[error("follow-up reassessment assignment error: {0}")]
    Assignment(#[from] AssignmentError),
    /// Target follow-up does not exist in `(practice, workspace)`.
    #[error("follow-up `{follow_up_id}` does not exist in workspace `{workspace}`")]
    NotFound {
        /// Patient workspace identifier.
        workspace: String,
        /// Missing follow-up identifier.
        follow_up_id: FollowUpId,
    },
    /// Follow-up is in a non-active standing (`Held`, `CannotRestart`, `Completed`, `Cancelled`, or `WithdrawnWrongPatient`) and cannot run reassessment.
    #[error(
        "follow-up `{follow_up_id}` is in `{standing}` standing and is not eligible for `{trigger}` reassessment"
    )]
    NotEligible {
        /// Follow-up identifier.
        follow_up_id: FollowUpId,
        /// Current standing discriminator (`held`, `completed`, `cancelled`, `cannot_restart`, `withdrawn_wrong_patient`).
        standing: &'static str,
        /// Trigger kind label (`due_wakeup` or `physician_review_now`).
        trigger: &'static str,
    },
    /// Automatic `DueWakeup` was requested before the follow-up's scheduled `due_at` timestamp.
    #[error(
        "follow-up `{follow_up_id}` is scheduled for unix {due_at} and is not yet due at unix {clinical_now}"
    )]
    NotYetDue {
        /// Follow-up identifier.
        follow_up_id: FollowUpId,
        /// Scheduled due timestamp.
        due_at: i64,
        /// Current clinical timestamp supplied on the wake-up.
        clinical_now: i64,
    },
    /// Underlying follow-up store operation failed.
    #[error("follow-up operation failed: {0}")]
    FollowUp(#[from] FollowUpError),
    /// Underlying saved patient review read failed.
    #[error("patient review read failed: {0}")]
    PatientReview(#[from] PatientReviewError),
    /// Source refresh status read failed.
    #[error("source refresh status read failed: {0}")]
    Refresh(#[from] crate::runtime::refresh::RefreshError),
    /// Database error from SeaORM.
    #[error("database error: {0}")]
    Database(#[from] DbErr),
    /// Shared harness execution failed before committing a reassessment disposition.
    #[error("follow-up reassessment run `{run_id}` failed: {source}")]
    Harness {
        /// Run identifier.
        run_id: String,
        /// Underlying harness failure.
        #[source]
        source: engine::Failure,
    },
    /// Run held on turn/context limits before committing a reassessment disposition.
    #[error(
        "follow-up reassessment run `{run_id}` stopped before calling `{COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL}`"
    )]
    Incomplete {
        /// Held run record.
        run_id: String,
        /// Saved held run snapshot.
        run: Box<FollowUpReassessmentRunRecord>,
    },
}

/// SHA-256 hex digest of [`FOLLOW_UP_REASSESSMENT_INSTRUCTIONS`].
#[must_use]
pub fn follow_up_reassessment_instructions_sha256() -> String {
    crate::sha256_hex(FOLLOW_UP_REASSESSMENT_INSTRUCTIONS.as_bytes())
}

/// Structured completion tool schema offered to follow-up reassessment runs.
#[must_use]
pub fn complete_follow_up_reassessment_tool() -> Tool {
    let citation_schema = json!({
        "type": "object",
        "properties": {
            "candidate_id": { "type": "string", "minLength": 1, "maxLength": 256 },
            "revision": { "type": "string", "minLength": 1, "maxLength": 256 }
        },
        "required": ["candidate_id", "revision"],
        "additionalProperties": false
    });
    let unfinished_schema = json!({
        "type": "object",
        "properties": {
            "topic": { "type": "string", "minLength": 1, "maxLength": 200 },
            "detail": { "type": "string", "minLength": 1, "maxLength": 2000 }
        },
        "required": ["topic", "detail"],
        "additionalProperties": false
    });
    let decision_schema = json!({
        "anyOf": [
            {
                "type": "object",
                "properties": {
                    "kind": { "type": "string", "enum": ["continue_unchanged"] },
                    "rationale": { "type": "string", "minLength": 1, "maxLength": MAX_REASSESSMENT_TEXT_CHARS },
                    "quiet": { "type": "boolean" }
                },
                "required": ["kind", "rationale", "quiet"],
                "additionalProperties": false
            },
            {
                "type": "object",
                "properties": {
                    "kind": { "type": "string", "enum": ["change_next_reassessment"] },
                    "summary": { "type": "string", "minLength": 1, "maxLength": 2000 },
                    "next_due_at": { "type": "integer", "minimum": 1 },
                    "change_note": { "type": "string", "minLength": 1, "maxLength": 500 },
                    "rationale": { "type": "string", "minLength": 1, "maxLength": MAX_REASSESSMENT_TEXT_CHARS }
                },
                "required": ["kind", "summary", "next_due_at", "change_note", "rationale"],
                "additionalProperties": false
            },
            {
                "type": "object",
                "properties": {
                    "kind": { "type": "string", "enum": ["complete"] },
                    "resolution_note": { "type": "string", "minLength": 1, "maxLength": 500 },
                    "rationale": { "type": "string", "minLength": 1, "maxLength": MAX_REASSESSMENT_TEXT_CHARS }
                },
                "required": ["kind", "resolution_note", "rationale"],
                "additionalProperties": false
            },
            {
                "type": "object",
                "properties": {
                    "kind": { "type": "string", "enum": ["end_without_completion"] },
                    "ending_reason": { "type": "string", "minLength": 1, "maxLength": 500 },
                    "rationale": { "type": "string", "minLength": 1, "maxLength": MAX_REASSESSMENT_TEXT_CHARS }
                },
                "required": ["kind", "ending_reason", "rationale"],
                "additionalProperties": false
            }
        ]
    });

    Tool {
        name: COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL.into(),
        description: "Submit the structured reassessment outcome for the target patient follow-up responsibility (`continue_unchanged`, `change_next_reassessment`, `complete`, or `end_without_completion`), citing read chart record revisions (`cited_records`), clinical `basis_and_limitations`, any `explicit_uncertainty` (missing evidence, silence, overdue work, or conflicting records), and any `unfinished_work`.".into(),
        parameters: json!({
            "type": "object",
            "properties": {
                "decision": decision_schema,
                "cited_records": {
                    "type": "array",
                    "items": citation_schema
                },
                "basis_and_limitations": {
                    "type": "string",
                    "minLength": 1,
                    "maxLength": MAX_REASSESSMENT_TEXT_CHARS
                },
                "explicit_uncertainty": {
                    "type": "array",
                    "items": { "type": "string", "minLength": 1, "maxLength": 1000 }
                },
                "unfinished_work": {
                    "type": "array",
                    "items": unfinished_schema
                }
            },
            "required": [
                "decision",
                "cited_records",
                "basis_and_limitations",
                "explicit_uncertainty",
                "unfinished_work"
            ],
            "additionalProperties": false
        }),
    }
}

/// Capabilities offered to a follow-up reassessment run on `ModelRoles::background`.
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
    offered.push(complete_follow_up_reassessment_tool());
    offered
}

/// Builds the immutable [`RequestRecipe`] for patient follow-up reassessment on
/// `roles.background`, eagerly preloading `patient-follow-up` and
/// `patient-record-retrieval` while leaving remaining skills and tools discoverable.
pub fn recipe(
    roles: &ModelRoles,
    skills: &PhysicianSkills,
) -> Result<(RequestRecipe, Vec<PreloadedSkillDigest>), AssignmentError> {
    let background = &roles.background;
    let (digests, payloads) =
        preload_skills_from_catalog(skills, &PRELOADED_FOLLOW_UP_REASSESSMENT_SKILLS)?;
    let mut instructions = skills.instructions(
        FOLLOW_UP_REASSESSMENT_INSTRUCTIONS,
        crate::physician_chat::Job::FollowUpReassessment,
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

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCompleteFollowUpReassessmentArgs {
    decision: SubmittedFollowUpDecision,
    cited_records: Vec<ExactRecordReference>,
    basis_and_limitations: String,
    explicit_uncertainty: Vec<String>,
    unfinished_work: Vec<UnfinishedReviewWork>,
}

fn validate_prose_field(field: &'static str, raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    let count = trimmed.chars().count();
    if !(1..=MAX_REASSESSMENT_TEXT_CHARS).contains(&count)
        || trimmed
            .chars()
            .any(|ch| ch.is_control() && ch != '\n' && ch != '\t')
    {
        return Err(format!(
            "`{field}` must be 1 to {MAX_REASSESSMENT_TEXT_CHARS} printable characters"
        ));
    }
    Ok(trimmed.to_owned())
}

fn ending_reason_falsely_claims_completion(ending_reason: &str) -> bool {
    let lower = ending_reason.to_ascii_lowercase();
    lower.contains("clinically completed")
        || lower.contains("condition resolved")
        || lower.contains("fully resolved")
        || lower.contains("goal met and completed")
}

fn describe_origin(origin: &FollowUpActor, history: &[FollowUpUpdateEntry]) -> String {
    let actor_desc = match origin {
        FollowUpActor::Physician { physician_id } => {
            format!("Created by physician {physician_id}")
        }
        FollowUpActor::CodaWork { source_work } => {
            format!("Created by Coda background work {}", source_work.work_id)
        }
    };
    if let Some(first) = history.first() {
        format!(
            "{actor_desc} at unix {} (initial due unix {}): {}",
            first.occurred_at, first.due_at, first.detail
        )
    } else {
        actor_desc
    }
}

fn memory_fingerprint(standing: &PatientMemoryStanding) -> String {
    match standing {
        PatientMemoryStanding::Available { entries } => {
            let mut parts = Vec::with_capacity(entries.len());
            for entry in entries {
                parts.push(format!("{}:{}", entry.id, entry.revision));
            }
            crate::sha256_hex(parts.join("|").as_bytes())
        }
        PatientMemoryStanding::Unavailable { reason } => {
            format!("unavailable:{}", crate::sha256_hex(reason.as_bytes()))
        }
    }
}

fn has_active_physician_memory_correction(standing: &PatientMemoryStanding) -> bool {
    match standing {
        PatientMemoryStanding::Available { entries } => entries.iter().any(|entry| {
            matches!(
                entry.provenance,
                MemoryProvenance::PhysicianCorrection { .. }
            )
        }),
        PatientMemoryStanding::Unavailable { .. } => false,
    }
}

fn verify_eligibility(
    follow_up: &FollowUp,
    trigger: &FollowUpReassessmentTrigger,
) -> Result<(), FollowUpReassessmentError> {
    if !matches!(follow_up.standing, FollowUpStanding::Active { .. }) {
        return Err(FollowUpReassessmentError::NotEligible {
            follow_up_id: follow_up.id.clone(),
            standing: follow_up.standing.status_str(),
            trigger: trigger.kind_label(),
        });
    }
    if let FollowUpReassessmentTrigger::DueWakeup { clinical_now } = trigger
        && !follow_up.standing.is_due_at(*clinical_now)
    {
        return Err(FollowUpReassessmentError::NotYetDue {
            follow_up_id: follow_up.id.clone(),
            due_at: follow_up.due_at,
            clinical_now: *clinical_now,
        });
    }
    Ok(())
}

/// Assembles [`FollowUpReassessmentStartingContext`] for `follow_up_id` in
/// `(practice, workspace)`, validating eligibility (`Active` and due when
/// `DueWakeup`) and composing the target responsibility with [`ReviewStartingContext`].
pub async fn build_follow_up_reassessment_starting_context(
    store: &Store,
    authority: &dyn PracticeAuthority,
    request: &FollowUpReassessmentRequest,
) -> Result<(FollowUpReassessmentStartingContext, Vec<SourceRevision>), FollowUpReassessmentError> {
    let practice = request.practice.trim().to_owned();
    let workspace = request.workspace.trim().to_owned();
    let review_id = request
        .review_id
        .clone()
        .unwrap_or_else(|| format!("fu-rev-{}", Uuid::new_v4()));

    authority
        .check_practice_patient(&PracticeScope {
            practice: practice.clone(),
            workspace: workspace.clone(),
            job_id: format!("follow-up-reassessment:{review_id}"),
        })
        .await?;

    let all_follow_ups = store.workspace_follow_ups(&practice, &workspace).await?;
    let target = all_follow_ups
        .iter()
        .find(|item| item.id == request.follow_up_id)
        .cloned()
        .ok_or_else(|| FollowUpReassessmentError::NotFound {
            workspace: workspace.clone(),
            follow_up_id: request.follow_up_id.clone(),
        })?;

    verify_eligibility(&target, &request.trigger)?;

    let sibling_follow_ups: Vec<FollowUp> = all_follow_ups
        .into_iter()
        .filter(|item| item.id != target.id && item.standing.is_terminal().is_none())
        .collect();
    let sibling_ids: Vec<FollowUpId> = sibling_follow_ups
        .iter()
        .map(|item| item.id.clone())
        .collect();

    let all_updates = store
        .workspace_follow_up_updates_audit(&practice, &workspace)
        .await?;
    let update_history: Vec<FollowUpUpdateEntry> = all_updates
        .into_iter()
        .filter(|entry| entry.follow_up_id == target.id)
        .collect();

    let patient_ctx = crate::message_review_recipe::load_practice_patient_starting_context(
        store, &practice, &workspace,
    )
    .await?;
    let mut observation = store.sources(&workspace).await?;
    retain_non_withdrawn_sources(
        &mut observation.sources,
        patient_ctx.withdrawal_boundary.as_ref(),
    );
    let record_versions: BTreeMap<String, String> = observation
        .sources
        .iter()
        .map(|s| (s.candidate_id.clone(), s.revision.clone()))
        .collect();

    let refresh_statuses = store.saved_source_statuses(&practice, &workspace).await?;

    let saved_reviews = store
        .patient_reviews(&practice, &workspace, authority)
        .await?;
    let mut prior_reviews = Vec::new();
    for saved in saved_reviews.into_iter().take(8) {
        if saved.review_id == review_id {
            continue;
        }
        if let ReviewLifecycleState::Completed {
            outcome,
            completed_at,
            completed_source_version,
        } = saved.state
        {
            let (disposition, reason) = match &outcome {
                ReviewOutcome::Quiet { reason, .. } => {
                    ("quiet".to_owned(), reason.as_str().to_owned())
                }
                ReviewOutcome::Assessed { summary, .. } => ("assessed".to_owned(), summary.clone()),
            };
            prior_reviews.push(PriorPatientReviewSummary {
                review_id: saved.review_id,
                completed_at,
                completed_source_version,
                triggers: saved
                    .triggers
                    .iter()
                    .map(|t| t.reason_label().to_owned())
                    .collect(),
                disposition,
                reason,
                actions: saved
                    .actions
                    .iter()
                    .map(RecordedReviewAction::summary_label)
                    .collect(),
                unfinished_work: saved.unfinished_work,
            });
        }
    }

    let last_reviewed_at = prior_reviews.first().map(|r| r.completed_at);
    let review_trigger = match &request.trigger {
        FollowUpReassessmentTrigger::DueWakeup { .. } => ReviewTrigger::Periodic {
            workspace_version: observation.version,
            record_versions,
            last_reviewed_at,
        },
        FollowUpReassessmentTrigger::PhysicianReviewNow { physician_id } => {
            ReviewTrigger::PhysicianRequest {
                physician_id: physician_id.clone(),
                request_text: format!(
                    "Reassess follow-up `{}` now: {}",
                    target.id,
                    target.summary.as_str()
                ),
                workspace_version: observation.version,
                record_versions,
            }
        }
    };

    let briefs: Vec<ConnectedSourceBrief> = patient_ctx
        .source_pulls
        .iter()
        .map(|s| ConnectedSourceBrief {
            source_id: s.source_id.clone(),
            source_label: s.source_label.clone(),
            pulled_at: s.pulled_at,
        })
        .collect();
    let evidence_scope = EvidenceScope::classify(
        &briefs,
        observation.sources.iter().map(|s| s.source_id.as_str()),
    );

    let review_context = ReviewStartingContext {
        workspace: workspace.clone(),
        review_id: review_id.clone(),
        patient: patient_ctx.patient,
        trigger: review_trigger,
        additional_inputs: Vec::new(),
        chart_version: observation.version,
        chart_record_count: observation.sources.len(),
        evidence_scope,
        source_pulls: patient_ctx.source_pulls,
        refresh_statuses,
        check_standing: patient_ctx.check_standing,
        understanding: patient_ctx.understanding,
        withdrawal_boundary: patient_ctx.withdrawal_boundary.clone(),
        memory_standing: patient_ctx.memory_standing,
        open_concerns: patient_ctx.open_concerns,
        prior_reviews,
    };

    let why_it_exists = describe_origin(&target.origin, &update_history);
    let origin = target.origin.clone();
    let responsibility = TargetFollowUpResponsibility {
        follow_up: target,
        why_it_exists,
        origin,
        constraints: FollowUpResponsibilityConstraints {
            resolution_policy: FollowUpResolutionPolicy::VerifiedEvidenceOnly,
            sibling_follow_up_ids: sibling_ids,
            withdrawal_boundary: patient_ctx.withdrawal_boundary,
        },
        update_history,
        sibling_follow_ups,
    };

    Ok((
        FollowUpReassessmentStartingContext {
            practice,
            workspace,
            review_id,
            trigger: request.trigger.clone(),
            responsibility,
            review_context,
        },
        observation.sources,
    ))
}

#[derive(Default)]
struct RunReadEvidence {
    read_revisions: BTreeSet<(String, String)>,
}

fn collect_read_revisions(events: &[Event], current_sources: &[SourceRevision]) -> RunReadEvidence {
    let mut state = RunReadEvidence::default();
    let mut call_meta: BTreeMap<String, (String, Value)> = BTreeMap::new();
    for event in events {
        match &event.kind {
            EventKind::ToolStarted(call) => {
                let parsed = serde_json::from_str::<Value>(&call.arguments).unwrap_or(Value::Null);
                call_meta.insert(call.id.clone(), (call.name.clone(), parsed));
            }
            EventKind::ProviderReturned(turn) => {
                for call in turn.calls() {
                    let parsed =
                        serde_json::from_str::<Value>(&call.arguments).unwrap_or(Value::Null);
                    call_meta
                        .entry(call.id.clone())
                        .or_insert_with(|| (call.name.clone(), parsed));
                }
            }
            _ => {}
        }
    }

    for event in events {
        let EventKind::ToolReturned { call_id, result } = &event.kind else {
            continue;
        };
        let Some((tool_name, args)) = call_meta.get(call_id) else {
            continue;
        };

        if tool_name == READ_TOOL {
            match result {
                engine::results::ToolResult::Original(raw) => {
                    if let Ok(parsed) = serde_json::from_str::<Value>(raw)
                        && let Some(results) = parsed.get("results").and_then(Value::as_array)
                    {
                        for item in results {
                            let item_outcome = item.get("outcome").and_then(Value::as_str);
                            if matches!(item_outcome, Some("available" | "passage")) {
                                let cand = item
                                    .get("candidate_id")
                                    .or_else(|| item.get("reference"))
                                    .and_then(Value::as_str);
                                let rev = item.get("revision").and_then(Value::as_str);
                                if let (Some(c), Some(r)) = (cand, rev) {
                                    state.read_revisions.insert((c.to_owned(), r.to_owned()));
                                } else if let Some(c) = cand
                                    && let Some(src) =
                                        current_sources.iter().find(|s| s.candidate_id == c)
                                {
                                    state
                                        .read_revisions
                                        .insert((src.candidate_id.clone(), src.revision.clone()));
                                }
                            }
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
                                state
                                    .read_revisions
                                    .insert((src.candidate_id.clone(), src.revision.clone()));
                            }
                        }
                    }
                }
                engine::results::ToolResult::Refused(_)
                | engine::results::ToolResult::History(_) => {}
            }
        } else if tool_name == READ_DOCUMENT_PAGE_TOOL
            && let engine::results::ToolResult::Original(raw) = result
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
                    || src.provenance.get("document_id").and_then(Value::as_str) == Some(doc_id);
                if matches_doc && rev_opt.is_none_or(|r| r == src.revision) {
                    state
                        .read_revisions
                        .insert((src.candidate_id.clone(), src.revision.clone()));
                }
            }
        }
    }
    state
}

fn extract_source_excerpt(source: &SourceRevision) -> String {
    for part in &source.readable_parts {
        let collapsed = part.text.split_whitespace().collect::<Vec<_>>().join(" ");
        if !collapsed.is_empty() {
            let mut out = String::new();
            for (idx, ch) in collapsed.chars().enumerate() {
                if idx >= EXCERPT_MAX_CHARS {
                    out.push('…');
                    break;
                }
                out.push(ch);
            }
            return out;
        }
    }
    let collapsed = source
        .content
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if !collapsed.is_empty() {
        return collapsed.chars().take(EXCERPT_MAX_CHARS).collect();
    }
    source.record_type.clone()
}

fn build_review_citation(
    source: &SourceRevision,
    pulls: &[SourceFreshnessSummary],
) -> ReviewCitation {
    let source_label = pulls
        .iter()
        .find(|p| p.source_id == source.source_id)
        .map_or_else(|| source.source_id.clone(), |p| p.source_label.clone());
    let display_label = source
        .provenance
        .get("title")
        .or_else(|| source.provenance.get("display_label"))
        .and_then(Value::as_str)
        .unwrap_or(&source.record_type)
        .to_owned();
    let as_of = source.dates.first().map(|d| d.value.clone());
    ReviewCitation {
        reference: ExactRecordReference {
            candidate_id: source.candidate_id.clone(),
            revision: source.revision.clone(),
        },
        source_id: source.source_id.clone(),
        source_label,
        sha256: source.sha256.clone(),
        display_label,
        as_of,
        excerpt: extract_source_excerpt(source),
    }
}

fn compute_usage_and_duration(
    events: &[Event],
    started_at: i64,
    completed_at: i64,
) -> (ReviewUsage, u64) {
    #[derive(Default)]
    struct Tally {
        input: Option<u64>,
        cached: Option<u64>,
        output: Option<u64>,
        done: bool,
    }

    let mut tallies: BTreeMap<(String, u64), Tally> = BTreeMap::new();
    for event in events {
        let key = (event.attempt_id.clone(), event.ordinal);
        match &event.kind {
            EventKind::ProviderStarted { .. } => {
                tallies.entry(key).or_default();
            }
            EventKind::ProviderObservation { usage, .. } => {
                let entry = tallies.entry(key).or_default();
                if usage.input_tokens.is_some() {
                    entry.input = usage.input_tokens;
                }
                if usage.cached_input_tokens.is_some() {
                    entry.cached = usage.cached_input_tokens;
                }
                if usage.output_tokens.is_some() {
                    entry.output = usage.output_tokens;
                }
            }
            EventKind::ProviderReturned(turn) => {
                let entry = tallies.entry(key).or_default();
                entry.done = true;
                if turn.usage.input_tokens.is_some() {
                    entry.input = turn.usage.input_tokens;
                }
                if turn.usage.cached_input_tokens.is_some() {
                    entry.cached = turn.usage.cached_input_tokens;
                }
                if turn.usage.output_tokens.is_some() {
                    entry.output = turn.usage.output_tokens;
                }
            }
            EventKind::ProviderFailed { .. }
            | EventKind::ProviderBlocked { .. }
            | EventKind::TransportRetry { .. }
            | EventKind::ToolStarted(_)
            | EventKind::ToolReturned { .. }
            | EventKind::ContextRejected
            | EventKind::Candidate { .. }
            | EventKind::WindowOpened { .. } => {}
        }
    }

    let completed: Vec<&Tally> = tallies.values().filter(|t| t.done).collect();
    let usage = if completed.is_empty() {
        ReviewUsage::Unknown
    } else {
        let mut sum_in = 0_u64;
        let mut sum_out = 0_u64;
        let mut sum_cached: Option<u64> = Some(0);
        let mut all_known = true;
        for ex in &completed {
            let (Some(i), Some(o)) = (ex.input, ex.output) else {
                all_known = false;
                break;
            };
            sum_in = sum_in.saturating_add(i);
            sum_out = sum_out.saturating_add(o);
            sum_cached = match (sum_cached, ex.cached) {
                (Some(acc), Some(c)) => Some(acc.saturating_add(c)),
                _ => None,
            };
        }
        if all_known {
            ReviewUsage::Known {
                input_tokens: sum_in,
                cached_input_tokens: sum_cached,
                output_tokens: sum_out,
                exchanges: u32::try_from(completed.len()).unwrap_or(u32::MAX),
            }
        } else {
            ReviewUsage::Unknown
        }
    };

    let elapsed_secs = completed_at.saturating_sub(started_at).max(0);
    let duration_ms = u64::try_from(elapsed_secs)
        .unwrap_or(0)
        .saturating_mul(1_000);
    (usage, duration_ms)
}

fn decode_reassessment_run(
    row: runtime_command_record::Model,
) -> Result<FollowUpReassessmentRunRecord, DbErr> {
    serde_json::from_value(row.command).map_err(conversion_error(
        "follow_up_reassessment_run",
        "FollowUpReassessmentRunRecord",
    ))
}

struct FollowUpReassessmentHost<'a> {
    store: Store,
    authority: &'a dyn PracticeAuthority,
    practice: String,
    workspace: String,
    follow_up_id: FollowUpId,
    run_id: String,
    attempt_id: String,
    review_id: String,
    committed: Option<(SavedFollowUpReassessmentOutcome, SavedPatientReview)>,
}

impl FollowUpReassessmentHost<'_> {
    fn source_message(ctx: &FollowUpReassessmentStartingContext) -> engine::ProductMessage {
        engine::ProductMessage {
            id: format!(
                "follow-up-reassessment:{}:{}",
                ctx.follow_up_id(),
                ctx.review_id
            ),
            role: engine::history::Role::User,
            text: ctx.render_prompt(),
        }
    }

    fn practice_scope(&self) -> PracticeScope {
        PracticeScope {
            practice: self.practice.clone(),
            workspace: self.workspace.clone(),
            job_id: format!("follow-up-reassessment:{}", self.run_id),
        }
    }

    fn control_scope(&self) -> Scope {
        self.practice_scope().as_control_scope()
    }

    async fn load_run(&self) -> Result<FollowUpReassessmentRunRecord, DbErr> {
        self.store
            .follow_up_reassessment_run(&self.practice, &self.run_id)
            .await?
            .ok_or_else(|| error("follow-up reassessment run missing"))
    }

    async fn ensure_target_follow_up_active(&self) -> Result<FollowUp, engine::ProductError> {
        let current_items = self
            .store
            .workspace_follow_ups(&self.practice, &self.workspace)
            .await
            .map_err(|err| -> engine::Cause { Box::new(err) })?;
        let Some(current_item) = current_items
            .into_iter()
            .find(|item| item.id == self.follow_up_id)
        else {
            return Err(persistence_failure(error(
                "target follow-up missing during reassessment",
            )));
        };
        if !matches!(current_item.standing, FollowUpStanding::Active { .. }) {
            let cause: engine::Cause = Box::new(MidRunFollowUpIneligible {
                follow_up_id: current_item.id,
                standing: current_item.standing.status_str(),
            });
            return Err(cause.into());
        }
        Ok(current_item)
    }

    async fn clear_staged_reassessment(
        &self,
        updated_chart_version: Option<u64>,
    ) -> Result<(), engine::ProductError> {
        let tx = self
            .store
            .spruce_db()
            .begin()
            .await
            .map_err(persistence_failure)?;
        let locked = runtime_command_record::Entity::find_by_id((
            self.practice.clone(),
            FOLLOW_UP_REASSESSMENT_RUN_ACTOR.to_owned(),
            self.run_id.clone(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await
        .map_err(persistence_failure)?
        .ok_or_else(|| persistence_failure(error("follow-up reassessment run missing")))?;
        let mut latest = decode_reassessment_run(locked.clone()).map_err(persistence_failure)?;
        latest.staged = None;
        if let Some(ver) = updated_chart_version {
            latest.starting_context.review_context.chart_version = ver;
        }
        let mut active: runtime_command_record::ActiveModel = locked.into();
        active.command = Set(serde_json::to_value(&latest)
            .map_err(conversion_error("FollowUpReassessmentRunRecord", "json"))
            .map_err(persistence_failure)?);
        active.update(&tx).await.map_err(persistence_failure)?;
        tx.commit().await.map_err(persistence_failure)?;
        Ok(())
    }

    async fn open_concerns_for_work(&self) -> Result<Vec<workspace_concern_record::Model>, DbErr> {
        let rows = workspace_concern_record::Entity::find()
            .filter(workspace_concern_record::Column::Practice.eq(&self.practice))
            .filter(workspace_concern_record::Column::Workspace.eq(&self.workspace))
            .filter(workspace_concern_record::Column::Status.eq("open"))
            .all(self.store.spruce_db())
            .await?;
        let mut out = Vec::new();
        for row in rows {
            let sw: SourceWork = serde_json::from_value(row.source_work.clone())
                .map_err(conversion_error("concern.source_work", "SourceWork"))?;
            if sw.work_id == self.run_id {
                out.push(row);
            }
        }
        Ok(out)
    }

    async fn recorded_actions_for_work(&self) -> Result<Vec<RecordedReviewAction>, DbErr> {
        let rows = workspace_concern_record::Entity::find()
            .filter(workspace_concern_record::Column::Practice.eq(&self.practice))
            .filter(workspace_concern_record::Column::Workspace.eq(&self.workspace))
            .order_by_asc(workspace_concern_record::Column::UpdatedAt)
            .order_by_asc(workspace_concern_record::Column::Id)
            .all(self.store.spruce_db())
            .await?;
        let mut actions = Vec::new();
        for row in rows {
            let sw: SourceWork = serde_json::from_value(row.source_work)
                .map_err(conversion_error("concern.source_work", "SourceWork"))?;
            let rev = u64::try_from(row.revision.max(0)).unwrap_or(0);
            if row.status == "open" {
                if sw.work_id != self.run_id {
                    continue;
                }
                let urgency: ConcernUrgency = serde_json::from_value(Value::String(row.urgency))
                    .map_err(conversion_error("concern.urgency", "ConcernUrgency"))?;
                let records: Vec<ExactRecordReference> = serde_json::from_value(row.records)
                    .map_err(conversion_error("concern.records", "ExactRecordReference"))?;
                actions.push(RecordedReviewAction::ConcernOpen {
                    concern_id: row.id,
                    urgency,
                    reason: row.reason,
                    revision: rev,
                    records,
                });
            } else if row.status == "retired" {
                let retired_in_this_run = match row.retired_by_work {
                    Some(retired_json) => {
                        let retired_sw: SourceWork = serde_json::from_value(retired_json)
                            .map_err(conversion_error("concern.retired_by_work", "SourceWork"))?;
                        retired_sw.work_id == self.run_id
                    }
                    None => sw.work_id == self.run_id,
                };
                if retired_in_this_run {
                    actions.push(RecordedReviewAction::ConcernRetired {
                        concern_id: row.id,
                        retirement_reason: row.retirement_reason.unwrap_or(row.reason),
                        revision: rev,
                    });
                }
            }
        }
        Ok(actions)
    }

    async fn stage_reassessment(
        &self,
        run: &FollowUpReassessmentRunRecord,
        call: &Invocation<Value>,
    ) -> Result<Result<StagedFollowUpReassessment, String>, engine::ProductError> {
        let parsed: RawCompleteFollowUpReassessmentArgs =
            match serde_json::from_value(call.arguments.clone()) {
                Ok(args) => args,
                Err(err) => {
                    return Ok(Err(format!(
                        "invalid `{COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL}` arguments: {err}"
                    )));
                }
            };

        let basis_and_limitations =
            match validate_prose_field("basis_and_limitations", &parsed.basis_and_limitations) {
                Ok(v) => v,
                Err(msg) => return Ok(Err(msg)),
            };

        let mut explicit_uncertainty = Vec::with_capacity(parsed.explicit_uncertainty.len());
        for item in &parsed.explicit_uncertainty {
            let trimmed = item.trim();
            if trimmed.is_empty() || trimmed.chars().any(char::is_control) {
                return Ok(Err(
                    "`explicit_uncertainty` entries must be non-empty single-line strings".into(),
                ));
            }
            explicit_uncertainty.push(trimmed.to_owned());
        }

        let (current_chart_version, clean_sources, withdrawal_boundary) =
            load_clean_sources(&self.store, &self.practice, &self.workspace)
                .await
                .map_err(persistence_failure)?;
        let withdrawn_candidates: BTreeSet<&str> = withdrawal_boundary
            .as_ref()
            .map(|b| b.candidate_ids.iter().map(String::as_str).collect())
            .unwrap_or_default();
        let by_candidate: BTreeMap<&str, &SourceRevision> = clean_sources
            .iter()
            .map(|s| (s.candidate_id.as_str(), s))
            .collect();

        let read_state = collect_read_revisions(&run.events, &clean_sources);

        // Verify every cited record exists in current non-withdrawn sources at the exact cited
        // revision and was read in this run.
        let mut normalized_citations = Vec::with_capacity(parsed.cited_records.len());
        let mut seen_refs = BTreeSet::new();
        for reference in &parsed.cited_records {
            let checked =
                match ExactRecordReference::new(&reference.candidate_id, &reference.revision) {
                    Ok(c) => c,
                    Err(err) => return Ok(Err(format!("invalid cited record reference: {err}"))),
                };
            if withdrawn_candidates.contains(checked.candidate_id.as_str()) {
                return Ok(Err(format!(
                    "cited chart record `{}` was withdrawn from this patient workspace and cannot be cited",
                    checked.candidate_id
                )));
            }
            let Some(current_src) = by_candidate.get(checked.candidate_id.as_str()).copied() else {
                return Ok(Err(format!(
                    "cited chart record `{}` is not present in current retained patient sources",
                    checked.candidate_id
                )));
            };
            if current_src.revision != checked.revision {
                return Ok(Err(format!(
                    "cited revision `{}` for `{}` is superseded; current revision is `{}`. Read `{}` via `read_patient_sources` before completing",
                    checked.revision,
                    checked.candidate_id,
                    current_src.revision,
                    checked.candidate_id
                )));
            }
            let pair = (checked.candidate_id.clone(), checked.revision.clone());
            if !read_state.read_revisions.contains(&pair) {
                return Ok(Err(format!(
                    "read `{}` at revision `{}` via `read_patient_sources` before citing it in `{COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL}`",
                    checked.candidate_id, checked.revision
                )));
            }
            if seen_refs.insert(pair) {
                normalized_citations.push(checked);
            }
        }

        // Verify open concerns raised in this run do not cite stale/withdrawn revisions
        // and do not conflict with a `quiet: true` continuation.
        let open_work_concerns = self
            .open_concerns_for_work()
            .await
            .map_err(persistence_failure)?;
        for concern_row in &open_work_concerns {
            let crefs: Vec<ExactRecordReference> =
                serde_json::from_value(concern_row.records.clone())
                    .map_err(conversion_error("concern.records", "ExactRecordReference"))
                    .map_err(persistence_failure)?;
            for cref in crefs {
                match by_candidate.get(cref.candidate_id.as_str()) {
                    None => {
                        return Ok(Err(format!(
                            "open concern `{}` cites withdrawn record `{}`; revise or retire that concern first",
                            concern_row.id, cref.candidate_id
                        )));
                    }
                    Some(cur) if cur.revision != cref.revision => {
                        return Ok(Err(format!(
                            "open concern `{}` cites superseded revision `{}` of `{}` (current `{}`); read and update first",
                            concern_row.id, cref.revision, cref.candidate_id, cur.revision
                        )));
                    }
                    Some(_) => {}
                }
            }
        }

        let validated_decision = match parsed.decision {
            SubmittedFollowUpDecision::ContinueUnchanged { rationale, quiet } => {
                let rationale = match validate_prose_field("rationale", &rationale) {
                    Ok(v) => v,
                    Err(msg) => return Ok(Err(msg)),
                };
                if quiet && !open_work_concerns.is_empty() {
                    return Ok(Err(
                        "this reassessment raised or revised an open patient concern on Today, so `continue_unchanged` cannot set `quiet: true`".into(),
                    ));
                }
                SubmittedFollowUpDecision::ContinueUnchanged { rationale, quiet }
            }
            SubmittedFollowUpDecision::ChangeNextReassessment {
                summary,
                next_due_at,
                change_note,
                rationale,
            } => {
                let validated_summary = match FollowUpSummary::new(summary) {
                    Ok(s) => s,
                    Err(err) => return Ok(Err(err.to_string())),
                };
                if next_due_at <= 0 {
                    return Ok(Err("`next_due_at` must be a positive Unix timestamp".into()));
                }
                let validated_note = match FollowUpChangeNote::new(change_note) {
                    Ok(n) => n,
                    Err(err) => return Ok(Err(err.to_string())),
                };
                let rationale = match validate_prose_field("rationale", &rationale) {
                    Ok(v) => v,
                    Err(msg) => return Ok(Err(msg)),
                };
                SubmittedFollowUpDecision::ChangeNextReassessment {
                    summary: validated_summary.into(),
                    next_due_at,
                    change_note: validated_note.into(),
                    rationale,
                }
            }
            SubmittedFollowUpDecision::Complete {
                resolution_note,
                rationale,
            } => {
                let validated_note = match FollowUpChangeNote::new(resolution_note) {
                    Ok(n) => n,
                    Err(err) => return Ok(Err(err.to_string())),
                };
                let rationale = match validate_prose_field("rationale", &rationale) {
                    Ok(v) => v,
                    Err(msg) => return Ok(Err(msg)),
                };
                // Safety & grounding barrier: patient silence or missing evidence never resolves a follow-up.
                let live_memory = match self
                    .store
                    .patient_memories(&self.practice, &self.workspace, MemorySelection::ActiveOnly)
                    .await
                {
                    Ok(entries) => PatientMemoryStanding::Available { entries },
                    Err(PatientMemoryError::Unavailable { reason }) => {
                        PatientMemoryStanding::Unavailable { reason }
                    }
                    Err(PatientMemoryError::Database(db_err)) => {
                        return Err(persistence_failure(db_err));
                    }
                    Err(other) => return Ok(Err(other.to_string())),
                };
                if normalized_citations.is_empty()
                    && !has_active_physician_memory_correction(&live_memory)
                {
                    return Ok(Err(
                        "`complete` requires read-verified chart evidence (`cited_records` read via `read_patient_sources`) or an active physician memory correction; patient silence or missing results never resolve a follow-up".into(),
                    ));
                }
                SubmittedFollowUpDecision::Complete {
                    resolution_note: validated_note.into(),
                    rationale,
                }
            }
            SubmittedFollowUpDecision::EndWithoutCompletion {
                ending_reason,
                rationale,
            } => {
                let validated_reason = match FollowUpChangeNote::new(ending_reason) {
                    Ok(n) => n,
                    Err(err) => return Ok(Err(err.to_string())),
                };
                if ending_reason_falsely_claims_completion(validated_reason.as_str()) {
                    return Ok(Err(
                        "`end_without_completion` cancels tracking without clinical completion; state why tracking ended without claiming clinical completion, or use `complete` with verified chart evidence".into(),
                    ));
                }
                let rationale = match validate_prose_field("rationale", &rationale) {
                    Ok(v) => v,
                    Err(msg) => return Ok(Err(msg)),
                };
                SubmittedFollowUpDecision::EndWithoutCompletion {
                    ending_reason: validated_reason.into(),
                    rationale,
                }
            }
        };

        Ok(Ok(StagedFollowUpReassessment {
            call_id: call.id.clone(),
            decision: validated_decision,
            cited_records: normalized_citations,
            basis_and_limitations,
            explicit_uncertainty,
            unfinished_work: parsed.unfinished_work,
            staged_chart_version: current_chart_version,
        }))
    }
}

impl FollowUpReassessmentStartingContext {
    fn follow_up_id(&self) -> &FollowUpId {
        &self.responsibility.follow_up.id
    }
}

impl Store {
    /// Reads one durable [`FollowUpReassessmentRunRecord`] by `(practice, run_id)`.
    pub async fn follow_up_reassessment_run(
        &self,
        practice: &str,
        run_id: &str,
    ) -> Result<Option<FollowUpReassessmentRunRecord>, DbErr> {
        runtime_command_record::Entity::find_by_id((
            practice.trim().to_owned(),
            FOLLOW_UP_REASSESSMENT_RUN_ACTOR.to_owned(),
            run_id.trim().to_owned(),
        ))
        .one(self.spruce_db())
        .await?
        .map(decode_reassessment_run)
        .transpose()
    }

    /// Lists all [`FollowUpReassessmentRunRecord`] entries for `(practice, workspace, follow_up_id)`
    /// in chronological order (`started_at`, `run_id`).
    pub async fn follow_up_reassessment_runs_for_item(
        &self,
        practice: &str,
        workspace: &str,
        follow_up_id: &FollowUpId,
    ) -> Result<Vec<FollowUpReassessmentRunRecord>, DbErr> {
        let rows = runtime_command_record::Entity::find()
            .filter(runtime_command_record::Column::Practice.eq(practice.trim()))
            .filter(runtime_command_record::Column::Physician.eq(FOLLOW_UP_REASSESSMENT_RUN_ACTOR))
            .order_by_asc(runtime_command_record::Column::AcceptedAt)
            .order_by_asc(runtime_command_record::Column::Key)
            .all(self.spruce_db())
            .await?;
        let mut out = Vec::new();
        for row in rows {
            let decoded = decode_reassessment_run(row)?;
            if decoded.workspace == workspace.trim() && &decoded.follow_up_id == follow_up_id {
                out.push(decoded);
            }
        }
        out.sort_by(|a, b| {
            a.started_at
                .cmp(&b.started_at)
                .then_with(|| a.run_id.cmp(&b.run_id))
        });
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::ModelBinding;
    use crate::runtime::refresh::REFRESH_TOOL;

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
    fn follow_up_reassessment_recipe_uses_background_lane_preloads_skills_and_has_unique_tools() {
        let skills = load_skills();
        let roles = distinct_roles();
        let (built, digests) =
            recipe(&roles, &skills).expect("follow-up reassessment recipe builds");

        assert_eq!(built.model, "background-lane-model");
        assert_eq!(built.context_window_tokens, 272_000);
        assert_eq!(built.max_input_tokens, 250_000);
        assert_eq!(built.options, json!({ "effort": "high" }));
        assert_eq!(digests.len(), 2);
        assert_eq!(digests[0].name, "patient-follow-up");
        assert_eq!(digests[1].name, "patient-record-retrieval");
        assert!(built.instructions.contains("patient-follow-up"));
        assert!(built.instructions.contains("patient-record-retrieval"));

        let mut seen = BTreeSet::new();
        for tool in &built.tools {
            let name = tool["name"].as_str().expect("tool name");
            assert!(seen.insert(name.to_owned()), "duplicate tool `{name}`");
            assert!(
                !is_physician_only_tool(name),
                "follow-up reassessment must never offer physician-only tool `{name}`"
            );
        }
        assert!(seen.contains(COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL));
        assert!(seen.contains(SKILL_VIEW));
        assert!(seen.contains(REFRESH_TOOL));
        assert!(seen.contains("search_clinical_library"));
        assert!(seen.contains("save_working_note"));

        let discoverable =
            discoverable_skill_names(&skills, &PRELOADED_FOLLOW_UP_REASSESSMENT_SKILLS);
        assert!(discoverable.contains(&"patient-communication".to_owned()));
        assert!(discoverable.contains(&"visit-preparation".to_owned()));
        assert!(!discoverable.contains(&"patient-follow-up".to_owned()));
        assert!(!discoverable.contains(&"patient-record-retrieval".to_owned()));
    }
}
