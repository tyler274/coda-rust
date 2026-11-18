//! Shared-harness execution, tool integration, lifecycle controls (`Stop`,
//! `Resume`, `Try again`), mid-run record-correction fencing, and durable
//! storage for patient reviews.
//!
//! Changed-record dispatches, periodic reassessments, and physician-requested
//! reviews all enter through one admission path, run as practice-owned
//! background jobs on [`crate::ModelRoles::background`] via
//! [`super::jobs::PracticeJobHost`], route tool calls to their authoritative
//! runtime owners, reconcile interrupted tool actions once-only on replay, and
//! commit the saved review atomically with the run record.

use std::collections::{BTreeMap, BTreeSet};

use async_trait::async_trait;
use harness::engine;
use patient_work::ReviewRequest;
use physician_auth::{AuthStore, Membership, PhysicianIdentity};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set, TransactionTrait, Unchanged,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

use super::chat_operations::{OpenConcernSummary, SourceFreshnessSummary};
use super::concerns::{ConcernUrgency, ExactRecordReference, SourceWork};
use super::delivery::Delivery;
use super::dispatch::ReviewReceiver;
use super::entities::{
    concern, memory_note, patient_dispatch, patient_review, patient_understanding_revision,
    practice_job,
};
use super::jobs::{
    AssignmentError, PRACTICE_OWNER, PracticeAccessError, PracticeAuthority, PracticeJobError,
    PracticeScope,
};
use super::receipt::{self, Receipt};
use super::{
    AttemptStatus, BindingRef, RuntimeError, SkillPackageId, Stopped, Store, WorkStatus,
    conversion_error, error, frozen, now,
};
use crate::ModelRoles;
use crate::initial_understanding::{ConnectedSourceBrief, EvidenceScope};
use crate::patient_reviews::{
    PriorPatientReviewSummary, ReviewOutcome, ReviewStartingContext, ReviewTrigger,
    UnfinishedReviewWork, assignment as build_review_assignment, review_job_id,
};
use crate::physician_chat::PhysicianSkills;
use crate::physician_sources::{SourcePatient, SourceRefresh, SourceRevision};

const REVIEW_MEMORY_ACTOR: &str = "coda:patient-review-control";

/// Preloaded skill digest recorded with a saved patient review's recipe version.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewSkillDigest {
    /// Approved catalog skill name.
    pub name: String,
    /// Lowercase SHA-256 hex digest of the preloaded `SKILL.md` content.
    pub sha256: String,
}

/// Exact recipe and model binding provenance recorded on a saved patient review.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewRecipeVersion {
    /// Background-lane model identifier.
    pub model: String,
    /// Configured reasoning effort.
    pub effort: String,
    /// Working context window ceiling in tokens.
    pub context_window_tokens: u64,
    /// Provider input ceiling in tokens.
    pub max_input_tokens: u64,
    /// Immutable binding digest in the run store.
    pub binding_sha256: String,
    /// Immutable skill catalog package digest in the run store.
    pub skills_package_id: String,
    /// Skills preloaded into the assignment at start.
    pub preloaded_skills: Vec<ReviewSkillDigest>,
}

/// Measured provider token usage for a patient review. Unknown usage stays
/// [`ReviewUsage::Unknown`] and is never coerced to zero.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ReviewUsage {
    /// Provider omitted token counts on at least one exchange (or zero exchanges completed).
    Unknown,
    /// Complete measured token counts reported across every completed exchange.
    Known {
        /// Sum of measured input tokens across completed exchanges.
        input_tokens: u64,
        /// Sum of measured cached input tokens when reported on every exchange (`None` if omitted).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cached_input_tokens: Option<u64>,
        /// Sum of measured output tokens across completed exchanges.
        output_tokens: u64,
        /// Count of completed provider exchanges.
        exchanges: u32,
    },
}

/// Authoritative durable effect recorded under a patient review's `work_id`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RecordedReviewAction {
    /// Coda raised or revised an open concern on Today during this review.
    ConcernOpen {
        /// Concern identifier.
        concern_id: String,
        /// `now` or `today`.
        urgency: ConcernUrgency,
        /// Physician-facing concern statement.
        reason: String,
        /// Saved concern revision.
        revision: u64,
        /// Exact cited chart records.
        records: Vec<ExactRecordReference>,
    },
    /// Coda retired a resolved or superseded concern during this review.
    ConcernRetired {
        /// Concern identifier.
        concern_id: String,
        /// Single-line retirement reason.
        retirement_reason: String,
        /// Saved concern revision after retirement.
        revision: u64,
    },
    /// Coda recorded or updated a durable clinical summary memory entry.
    MemoryRecorded {
        /// Memory entry identifier.
        memory_id: String,
        /// Clinical topic label.
        topic: String,
        /// Saved memory revision.
        revision: u64,
    },
    /// Coda retired a superseded durable patient memory entry.
    MemoryRetired {
        /// Memory entry identifier.
        memory_id: String,
        /// Single-line retirement reason.
        reason: String,
        /// Saved memory revision.
        revision: u64,
    },
    /// Coda saved a new `PatientUnderstanding` revision during this review.
    UnderstandingSaved {
        /// Saved understanding revision number.
        revision: u64,
        /// Workspace source version covered by the saved understanding.
        source_version: u64,
    },
    /// Coda refreshed a connected patient source during this review.
    SourceRefreshed {
        /// Refreshed source identifier.
        source_id: String,
        /// Workspace source version after the source refresh settled.
        workspace_version_after: u64,
    },
}

impl RecordedReviewAction {
    fn summary_label(&self) -> String {
        match self {
            Self::ConcernOpen {
                concern_id,
                urgency,
                reason,
                ..
            } => format!("concern {concern_id} ({}) open: {reason}", urgency.as_str()),
            Self::ConcernRetired {
                concern_id,
                retirement_reason,
                ..
            } => format!("concern {concern_id} retired: {retirement_reason}"),
            Self::MemoryRecorded {
                memory_id, topic, ..
            } => format!("memory {memory_id} recorded ({topic})"),
            Self::MemoryRetired {
                memory_id, reason, ..
            } => format!("memory {memory_id} retired: {reason}"),
            Self::UnderstandingSaved {
                revision,
                source_version,
            } => format!("understanding saved (rev {revision} at chart v{source_version})"),
            Self::SourceRefreshed {
                source_id,
                workspace_version_after,
            } => format!("source {source_id} refreshed (chart v{workspace_version_after})"),
        }
    }
}

/// Why a patient review was placed on hold.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewHoldReason {
    /// Explicitly stopped; holds across restart until `Resume` or new input.
    Stopped,
    /// Reached a turn or budget backstop; `Try again` rearms with current evidence.
    Limit,
}

/// Why an incomplete review result was superseded without completing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ReviewSupersededReason {
    /// New review request superseded this review.
    NewInput {
        /// Replacement review identifier, when replaced by a newer review row.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        replacement_review_id: Option<String>,
    },
    /// Practice or physician access to the patient workspace was lost.
    AccessLost {
        /// Access refusal detail.
        detail: String,
    },
}

/// Lifecycle state of a saved patient review.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ReviewLifecycleState {
    /// Queued and awaiting worker claim.
    Pending,
    /// Claimed and actively executing on the shared harness.
    Running {
        /// Active practice-job attempt identifier.
        attempt_id: String,
        /// Unix second when execution began.
        started_at: i64,
    },
    /// Held by `Stop` or a run limit; automatic recovery will not restart it.
    Held {
        /// Whether held by `Stop` or by a turn/budget limit.
        reason: ReviewHoldReason,
        /// Unix second when held.
        held_at: i64,
        /// Retained partial output text when held at a limit.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        partial_text: Option<String>,
    },
    /// Completed and durably saved.
    Completed {
        /// Quiet (`"nothing to do"`) or assessed outcome with verified citations.
        outcome: ReviewOutcome,
        /// Unix second when committed.
        completed_at: i64,
        /// Workspace source version verified at commit.
        completed_source_version: u64,
    },
    /// Exhausted retry allowance or blocked by provider error; recoverable via `Try again`.
    Failed {
        /// Failure summary.
        reason: String,
        /// Unix second when settled as failed.
        failed_at: i64,
        /// Failed attempt count in the exhausted run.
        attempts: u32,
    },
    /// Superseded without completing (for example after access loss or replacement).
    Superseded {
        /// Why the review was superseded.
        reason: ReviewSupersededReason,
        /// Unix second when superseded.
        superseded_at: i64,
    },
}

impl ReviewLifecycleState {
    /// Canonical lowercase status label (`"pending"`, `"running"`, `"held"`,
    /// `"completed"`, `"failed"`, or `"superseded"`).
    #[must_use]
    pub const fn status_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Running { .. } => "running",
            Self::Held { .. } => "held",
            Self::Completed { .. } => "completed",
            Self::Failed { .. } => "failed",
            Self::Superseded { .. } => "superseded",
        }
    }
}

/// Follow-up direction or updated trigger accepted onto an existing review before completion.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewAcceptedInput {
    /// Stable command/input identifier within the review's work.
    pub id: String,
    /// Model-facing text describing the updated request or clinical note.
    pub text: String,
    /// Unix second when accepted.
    pub recorded_at: i64,
}

/// Authoritative saved patient review record in `physician_runtime_patient_reviews`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedPatientReview {
    /// Practice tenant owning the patient link.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Unique review identifier within `(practice, workspace)`.
    pub review_id: String,
    /// Once-only admission command key.
    pub command_key: String,
    /// Underlying practice background job identifier (`physician_runtime_practice_jobs`).
    pub job_id: String,
    /// Run-store work item identifier acted on by `Stop`, `Resume`, and `Try again`.
    pub work_id: String,
    /// Patient demographic snapshot from connected sources, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patient: Option<SourcePatient>,
    /// Chronological triggers recorded for this review (`new records`, `periodic`, `physician request`).
    pub triggers: Vec<ReviewTrigger>,
    /// Follow-up inputs accepted into this review's work before completion.
    #[serde(default)]
    pub accepted_inputs: Vec<ReviewAcceptedInput>,
    /// Exact recipe, model settings, and preloaded skill digests.
    pub recipe_version: ReviewRecipeVersion,
    /// Workspace chart version when the review was admitted (or rearmed).
    pub initial_source_version: u64,
    /// Workspace chart version when the review completed (`None` until completed).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_source_version: Option<u64>,
    /// Exact `(candidate_id, revision)` source records read during the review.
    pub evidence_read: Vec<ExactRecordReference>,
    /// Authoritative durable actions committed under `work_id`.
    pub actions: Vec<RecordedReviewAction>,
    /// Unfinished work recorded for subsequent reviews to revisit.
    pub unfinished_work: Vec<UnfinishedReviewWork>,
    /// Current lifecycle state (including `ReviewOutcome` once `Completed`).
    pub state: ReviewLifecycleState,
    /// Measured provider token usage (`Unknown` when unreported, never coerced to zero).
    pub usage: ReviewUsage,
    /// Elapsed active review duration in milliseconds once completed (`None` while incomplete).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
    /// Unix second when admitted.
    pub queued_at: i64,
    /// Unix second when first claimed by a worker.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    /// Unix second when completed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    /// Unix second of the latest state transition.
    pub updated_at: i64,
}

/// Borrowed composite key targeting one saved patient review (`practice`, `workspace`, `review_id`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PatientReviewTarget<'a> {
    /// Practice tenant owning the patient link.
    pub practice: &'a str,
    /// Patient workspace identifier.
    pub workspace: &'a str,
    /// Saved review identifier.
    pub review_id: &'a str,
}

/// Updated direction or additional trigger submitted onto an incomplete patient review.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReviewInputSubmission<'a> {
    /// Stable command/input identifier within the review's work.
    pub input_key: &'a str,
    /// Clinical note or updated direction for the review.
    pub input_text: &'a str,
    /// Optional coalesced trigger metadata to append to `triggers`.
    pub additional_trigger: Option<ReviewTrigger>,
}

impl SavedPatientReview {
    /// Composite key referencing this saved review.
    #[must_use]
    pub fn target(&self) -> PatientReviewTarget<'_> {
        PatientReviewTarget {
            practice: &self.practice,
            workspace: &self.workspace,
            review_id: &self.review_id,
        }
    }

    /// Practice scope of the underlying background run.
    #[must_use]
    pub fn practice_scope(&self) -> PracticeScope {
        PracticeScope {
            practice: self.practice.clone(),
            workspace: self.workspace.clone(),
            job_id: self.job_id.clone(),
        }
    }

    /// Completed outcome when `state` is [`ReviewLifecycleState::Completed`].
    #[must_use]
    pub const fn completed_outcome(&self) -> Option<&ReviewOutcome> {
        match &self.state {
            ReviewLifecycleState::Completed { outcome, .. } => Some(outcome),
            ReviewLifecycleState::Pending
            | ReviewLifecycleState::Running { .. }
            | ReviewLifecycleState::Held { .. }
            | ReviewLifecycleState::Failed { .. }
            | ReviewLifecycleState::Superseded { .. } => None,
        }
    }
}

/// Result of admitting a patient review request.
#[derive(Clone, Debug)]
pub enum AdmittedPatientReview {
    /// Newly queued review and its worker wakeup [`Delivery`].
    New {
        /// Saved review in `Pending` state.
        review: Box<SavedPatientReview>,
        /// Practice-job wakeup delivery.
        delivery: Delivery,
    },
    /// Command key was already admitted; returns the existing review and delivery.
    Duplicate {
        /// Existing saved review.
        review: Box<SavedPatientReview>,
        /// Practice-job wakeup delivery.
        delivery: Delivery,
    },
}

impl AdmittedPatientReview {
    /// Saved review record.
    #[must_use]
    pub fn review(&self) -> &SavedPatientReview {
        match self {
            Self::New { review, .. } | Self::Duplicate { review, .. } => review,
        }
    }

    /// Practice-job wakeup delivery for the worker bus.
    #[must_use]
    pub const fn delivery(&self) -> &Delivery {
        match self {
            Self::New { delivery, .. } | Self::Duplicate { delivery, .. } => delivery,
        }
    }
}

/// Errors produced when admitting, steering, controlling, or reading patient reviews.
#[derive(Debug, thiserror::Error)]
pub enum PatientReviewError {
    /// Practice, workspace, review, or command identifier was blank or invalid.
    #[error("patient review identifiers and command keys must be non-empty")]
    InvalidScope,
    /// Physician request text or steering note was blank or invalid.
    #[error("invalid patient review input: {reason}")]
    InvalidInput {
        /// Validation failure reason.
        reason: String,
    },
    /// Practice patient membership check refused access.
    #[error("{0}")]
    Access(#[from] PracticeAccessError),
    /// Requesting physician lacked panel access (`Membership::Allowed`) to `workspace`.
    #[error(
        "physician {physician_id} in practice {practice_id} lacks panel access to {workspace} ({membership:?})"
    )]
    PhysicianUnauthorized {
        /// Practice identifier.
        practice_id: String,
        /// Physician identifier.
        physician_id: String,
        /// Patient workspace identifier.
        workspace: String,
        /// Refused membership standing.
        membership: Membership,
    },
    /// Assignment or skill preloading failed.
    #[error("{0}")]
    Assignment(#[from] AssignmentError),
    /// Named review does not exist in `(practice, workspace)`.
    #[error("patient review {review_id} not found in workspace {workspace} of practice {practice}")]
    NotFound {
        /// Practice identifier.
        practice: String,
        /// Patient workspace identifier.
        workspace: String,
        /// Missing review identifier.
        review_id: String,
    },
    /// Lifecycle control (`Stop`, `Resume`, `Try again`, or new input) was refused for the review's current state.
    #[error("cannot {action} patient review {review_id} while in state `{state}`")]
    ControlRefused {
        /// Review identifier.
        review_id: String,
        /// Current review status label.
        state: String,
        /// Requested lifecycle action.
        action: &'static str,
    },
    /// Command key collided with a different control payload.
    #[error("idempotency collision on patient review command key `{key}`")]
    IdempotencyCollision {
        /// Colliding command key.
        key: String,
    },
    /// Underlying runtime lifecycle error.
    #[error("{0}")]
    Runtime(#[from] RuntimeError),
    /// Database error from SeaORM.
    #[error("{0}")]
    Database(#[from] DbErr),
}

/// [`ReviewReceiver`] implementation bridging practice dispatch wakeups into
/// shared-harness patient review runs (`physician_runtime_patient_reviews` +
/// `physician_runtime_practice_jobs`).
pub struct PatientReviewReceiver<'a> {
    store: &'a Store,
    roles: &'a ModelRoles,
    skills: &'a PhysicianSkills,
    authority: &'a dyn PracticeAuthority,
    deliveries: tokio::sync::Mutex<Vec<Delivery>>,
}

impl<'a> PatientReviewReceiver<'a> {
    /// Creates a [`PatientReviewReceiver`] bound to `store`, `roles`, `skills`, and `authority`.
    pub fn new(
        store: &'a Store,
        roles: &'a ModelRoles,
        skills: &'a PhysicianSkills,
        authority: &'a dyn PracticeAuthority,
    ) -> Self {
        Self {
            store,
            roles,
            skills,
            authority,
            deliveries: tokio::sync::Mutex::new(Vec::new()),
        }
    }

    /// Drains all [`Delivery`] wakeups admitted during `wake_practice_dispatch`.
    pub async fn take_deliveries(&self) -> Vec<Delivery> {
        let mut guard = self.deliveries.lock().await;
        std::mem::take(&mut *guard)
    }
}

#[async_trait]
impl ReviewReceiver for PatientReviewReceiver<'_> {
    async fn submit_review(&self, request: &ReviewRequest) -> Result<(), PracticeJobError> {
        let admitted = self
            .store
            .submit_dispatched_patient_review(request, self.roles, self.skills, self.authority)
            .await
            .map_err(|err| match err {
                PatientReviewError::Access(access) => PracticeJobError::Access(access),
                PatientReviewError::Assignment(assign) => PracticeJobError::Assignment(assign),
                PatientReviewError::Runtime(runtime) => PracticeJobError::Runtime(runtime),
                PatientReviewError::Database(db) => {
                    PracticeJobError::Runtime(RuntimeError::Database(db))
                }
                other => PracticeJobError::from(error(&other.to_string())),
            })?;
        if let AdmittedPatientReview::New { delivery, .. } = admitted {
            self.deliveries.lock().await.push(delivery);
        }
        Ok(())
    }
}

struct VerifiedPracticePatient;

#[async_trait]
impl PracticeAuthority for VerifiedPracticePatient {
    async fn check_practice_patient(
        &self,
        scope: &PracticeScope,
    ) -> Result<(), PracticeAccessError> {
        if scope.validate().is_err() {
            return Err(PracticeAccessError::InvalidScope);
        }
        Ok(())
    }
}

fn validate_id(value: &str) -> Result<String, PatientReviewError> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.chars().any(char::is_control) {
        return Err(PatientReviewError::InvalidScope);
    }
    Ok(trimmed.to_owned())
}

fn review_membership_scope(practice: &str, workspace: &str) -> PracticeScope {
    PracticeScope {
        practice: practice.to_owned(),
        workspace: workspace.to_owned(),
        job_id: "patient_review_membership".to_owned(),
    }
}

fn deterministic_review_id(workspace: &str, command_key: &str) -> String {
    let digest = &crate::sha256_hex(format!("{workspace}|{command_key}").as_bytes())[..20];
    format!("rev-{digest}")
}

fn record_versions_map(sources: &[SourceRevision]) -> BTreeMap<String, String> {
    sources
        .iter()
        .map(|s| (s.candidate_id.clone(), s.revision.clone()))
        .collect()
}

fn diff_changed_candidates(
    baseline: &BTreeMap<String, String>,
    current: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut changed = BTreeSet::new();
    for (cid, rev) in current {
        if baseline.get(cid) != Some(rev) {
            changed.insert(cid.clone());
        }
    }
    for cid in baseline.keys() {
        if !current.contains_key(cid) {
            changed.insert(cid.clone());
        }
    }
    changed.into_iter().collect()
}

fn decode_saved_review(row: patient_review::Model) -> Result<SavedPatientReview, DbErr> {
    let patient: Option<SourcePatient> = row
        .patient_snapshot
        .map(serde_json::from_value)
        .transpose()
        .map_err(conversion_error(
            "patient_review.patient_snapshot",
            "SourcePatient",
        ))?;
    let triggers: Vec<ReviewTrigger> = serde_json::from_value(row.triggers).map_err(
        conversion_error("patient_review.triggers", "Vec<ReviewTrigger>"),
    )?;
    let accepted_inputs: Vec<ReviewAcceptedInput> = serde_json::from_value(row.accepted_inputs)
        .map_err(conversion_error(
            "patient_review.accepted_inputs",
            "Vec<ReviewAcceptedInput>",
        ))?;
    let recipe_version: ReviewRecipeVersion = serde_json::from_value(row.recipe_version).map_err(
        conversion_error("patient_review.recipe_version", "ReviewRecipeVersion"),
    )?;
    let initial_source_version = u64::try_from(row.initial_source_version).map_err(
        conversion_error("patient_review.initial_source_version", "u64"),
    )?;
    let completed_source_version = row
        .completed_source_version
        .map(u64::try_from)
        .transpose()
        .map_err(conversion_error(
            "patient_review.completed_source_version",
            "u64",
        ))?;
    let evidence_read: Vec<ExactRecordReference> = serde_json::from_value(row.evidence_read)
        .map_err(conversion_error(
            "patient_review.evidence_read",
            "Vec<ExactRecordReference>",
        ))?;
    let actions: Vec<RecordedReviewAction> = serde_json::from_value(row.actions).map_err(
        conversion_error("patient_review.actions", "Vec<RecordedReviewAction>"),
    )?;
    let unfinished_work: Vec<UnfinishedReviewWork> = serde_json::from_value(row.unfinished_work)
        .map_err(conversion_error(
            "patient_review.unfinished_work",
            "Vec<UnfinishedReviewWork>",
        ))?;
    let state: ReviewLifecycleState = serde_json::from_value(row.state).map_err(
        conversion_error("patient_review.state", "ReviewLifecycleState"),
    )?;
    let usage: ReviewUsage = serde_json::from_value(row.usage)
        .map_err(conversion_error("patient_review.usage", "ReviewUsage"))?;
    let duration_ms = row
        .duration_ms
        .map(u64::try_from)
        .transpose()
        .map_err(conversion_error("patient_review.duration_ms", "u64"))?;

    Ok(SavedPatientReview {
        practice: row.practice,
        workspace: row.workspace,
        review_id: row.review_id,
        command_key: row.command_key,
        job_id: row.job_id,
        work_id: row.work_id,
        patient,
        triggers,
        accepted_inputs,
        recipe_version,
        initial_source_version,
        completed_source_version,
        evidence_read,
        actions,
        unfinished_work,
        state,
        usage,
        duration_ms,
        queued_at: row.queued_at,
        started_at: row.started_at,
        completed_at: row.completed_at,
        updated_at: row.updated_at,
    })
}

fn decode_sources_json(raw: Value, field: &'static str) -> Result<Vec<SourceRevision>, DbErr> {
    serde_json::from_value(raw).map_err(conversion_error(field, "Vec<SourceRevision>"))
}

fn decode_initial_sources(raw: Value) -> Result<Vec<SourceRevision>, DbErr> {
    decode_sources_json(raw, "patient_review.initial_sources")
}

async fn collect_review_actions_in_tx(
    tx: &impl ConnectionTrait,
    practice: &str,
    workspace: &str,
    work_id: &str,
) -> Result<Vec<RecordedReviewAction>, DbErr> {
    let mut actions = Vec::new();

    let concerns = concern::Entity::find()
        .filter(concern::Column::Practice.eq(practice))
        .filter(concern::Column::Workspace.eq(workspace))
        .order_by_asc(concern::Column::CreatedAt)
        .order_by_asc(concern::Column::Id)
        .all(tx)
        .await?;
    for row in concerns {
        let rev =
            u64::try_from(row.revision).map_err(conversion_error("concern.revision", "u64"))?;
        if row.status == "retired"
            && let Some(retired_by) = row.retired_by_work.clone()
        {
            let sw: SourceWork = serde_json::from_value(retired_by)
                .map_err(conversion_error("concern.retired_by_work", "SourceWork"))?;
            if sw.work_id == work_id {
                let retirement_reason = row
                    .retirement_reason
                    .ok_or_else(|| error("retired concern missing retirement_reason"))?;
                actions.push(RecordedReviewAction::ConcernRetired {
                    concern_id: row.id,
                    retirement_reason,
                    revision: rev,
                });
                continue;
            }
        }
        let sw: SourceWork = serde_json::from_value(row.source_work)
            .map_err(conversion_error("concern.source_work", "SourceWork"))?;
        if sw.work_id == work_id && row.status == "open" {
            let urgency: ConcernUrgency = serde_json::from_value(Value::String(row.urgency))
                .map_err(conversion_error("concern.urgency", "ConcernUrgency"))?;
            let records: Vec<ExactRecordReference> = serde_json::from_value(row.records).map_err(
                conversion_error("concern.records", "Vec<ExactRecordReference>"),
            )?;
            actions.push(RecordedReviewAction::ConcernOpen {
                concern_id: row.id,
                urgency,
                reason: row.reason,
                revision: rev,
                records,
            });
        }
    }

    let notes = memory_note::Entity::find()
        .filter(memory_note::Column::Practice.eq(practice))
        .filter(memory_note::Column::Workspace.eq(workspace))
        .filter(memory_note::Column::Message.eq(work_id))
        .order_by_asc(memory_note::Column::Position)
        .all(tx)
        .await?;
    for row in notes {
        let pos =
            u64::try_from(row.position).map_err(conversion_error("memory_note.position", "u64"))?;
        actions.push(RecordedReviewAction::MemoryRecorded {
            memory_id: row.note,
            topic: "patient_memory_note".to_owned(),
            revision: pos,
        });
    }

    let understanding_revs = patient_understanding_revision::Entity::find()
        .filter(patient_understanding_revision::Column::Practice.eq(practice))
        .filter(patient_understanding_revision::Column::Workspace.eq(workspace))
        .order_by_asc(patient_understanding_revision::Column::Revision)
        .all(tx)
        .await?;
    for row in understanding_revs {
        let origin: super::understanding::RevisionOrigin = serde_json::from_value(row.origin)
            .map_err(conversion_error(
                "patient_understanding_revision.origin",
                "RevisionOrigin",
            ))?;
        let origin_work_id = match &origin {
            super::understanding::RevisionOrigin::PracticeRun { work_id, .. }
            | super::understanding::RevisionOrigin::PhysicianConversation { work_id, .. } => {
                work_id.as_str()
            }
        };
        if origin_work_id == work_id {
            actions.push(RecordedReviewAction::UnderstandingSaved {
                revision: u64::try_from(row.revision.max(0)).unwrap_or(0),
                source_version: u64::try_from(row.source_version.max(0)).unwrap_or(0),
            });
        }
    }

    let refresh_rows =
        super::refresh::load_practice_work_refreshes(tx, practice, workspace, work_id).await?;
    for row in refresh_rows {
        actions.push(RecordedReviewAction::SourceRefreshed {
            source_id: row.source_id,
            workspace_version_after: u64::try_from(row.workspace_version_after.max(0)).unwrap_or(0),
        });
    }

    Ok(actions)
}

impl Store {
    /// Returns the database clock's current Unix timestamp in seconds.
    pub async fn current_unix_seconds(&self) -> Result<i64, DbErr> {
        now(&self.db).await
    }

    /// Lists distinct practice identifiers that have registered patients in the
    /// durable patient dispatch store (`physician_runtime_patient_dispatch`).
    pub async fn registered_dispatch_practices(&self) -> Result<Vec<String>, DbErr> {
        patient_dispatch::Entity::find()
            .select_only()
            .column(patient_dispatch::Column::Practice)
            .distinct()
            .order_by_asc(patient_dispatch::Column::Practice)
            .into_tuple::<String>()
            .all(&self.db)
            .await
    }

    /// Assembles [`ReviewStartingContext`] for `(practice, workspace)` from
    /// authoritative patient pulls, chart observations, source refresh statuses,
    /// dispatch standing, saved understanding, durable memory, open concerns,
    /// and prior completed patient reviews.
    async fn review_starting_context(
        &self,
        practice: &str,
        workspace: &str,
        review_id: &str,
        trigger: ReviewTrigger,
        additional_inputs: Vec<String>,
    ) -> Result<(ReviewStartingContext, super::sources::Observation), DbErr> {
        let observation = self.sources(workspace).await?;
        let pulls = self
            .source_pulls(&[workspace.to_owned()])
            .await?
            .remove(workspace)
            .unwrap_or_default();
        let patient = pulls
            .iter()
            .max_by_key(|pull| pull.pulled_at)
            .map(|pull| pull.patient.clone());
        let briefs: Vec<ConnectedSourceBrief> = pulls
            .iter()
            .map(|pull| ConnectedSourceBrief {
                source_id: pull.source_id.clone(),
                source_label: pull.source_label.clone(),
                pulled_at: pull.pulled_at.unwrap_or(0),
            })
            .collect();
        let evidence_scope = EvidenceScope::classify(
            &briefs,
            observation.sources.iter().map(|s| s.source_id.as_str()),
        );
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
        let refresh_statuses = self
            .saved_source_statuses(practice, workspace)
            .await
            .map_err(|err| match err {
                super::refresh::RefreshError::Database(db_err) => db_err,
                other => error(&other.to_string()),
            })?;
        let check_standing = self.dispatch_check_standing(practice, workspace).await?;
        let understanding = self
            .patient_understanding_internal(practice, workspace)
            .await?;

        let memory_standing = self.active_memory_standing(practice, workspace).await?;

        let mut raw_open_concerns = concern::Entity::find()
            .filter(concern::Column::Practice.eq(practice))
            .filter(concern::Column::Workspace.eq(workspace))
            .filter(concern::Column::Status.eq("open"))
            .all(&self.db)
            .await?;
        raw_open_concerns.sort_by(|left, right| {
            let rank = |urgency: &str| if urgency == "now" { 0_u8 } else { 1_u8 };
            rank(&left.urgency)
                .cmp(&rank(&right.urgency))
                .then_with(|| left.created_at.cmp(&right.created_at))
                .then_with(|| left.id.cmp(&right.id))
        });
        let mut open_concerns = Vec::with_capacity(raw_open_concerns.len());
        for row in raw_open_concerns {
            let urgency: ConcernUrgency = serde_json::from_value(Value::String(row.urgency))
                .map_err(conversion_error("concern.urgency", "ConcernUrgency"))?;
            let records: Vec<ExactRecordReference> = serde_json::from_value(row.records)
                .map_err(conversion_error("concern.records", "ExactRecordReference"))?;
            open_concerns.push(OpenConcernSummary {
                concern_id: row.id,
                urgency,
                reason: row.reason,
                records,
            });
        }

        let completed_rows = patient_review::Entity::find()
            .filter(patient_review::Column::Practice.eq(practice))
            .filter(patient_review::Column::Workspace.eq(workspace))
            .filter(patient_review::Column::Status.eq("completed"))
            .filter(patient_review::Column::ReviewId.ne(review_id))
            .order_by_desc(patient_review::Column::CompletedAt)
            .order_by_desc(patient_review::Column::UpdatedAt)
            .all(&self.db)
            .await?;
        let mut prior_reviews = Vec::new();
        for row in completed_rows.into_iter().take(8) {
            let saved = decode_saved_review(row)?;
            if let ReviewLifecycleState::Completed {
                outcome,
                completed_at,
                completed_source_version,
            } = &saved.state
            {
                let (disposition, reason) = match outcome {
                    ReviewOutcome::Quiet { reason, .. } => {
                        ("quiet".to_owned(), reason.as_str().to_owned())
                    }
                    ReviewOutcome::Assessed { summary, .. } => {
                        ("assessed".to_owned(), summary.clone())
                    }
                };
                prior_reviews.push(PriorPatientReviewSummary {
                    review_id: saved.review_id,
                    completed_at: *completed_at,
                    completed_source_version: *completed_source_version,
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

        Ok((
            ReviewStartingContext {
                workspace: workspace.to_owned(),
                review_id: review_id.to_owned(),
                patient,
                trigger,
                additional_inputs,
                chart_version: observation.version,
                chart_record_count: observation.sources.len(),
                evidence_scope,
                source_pulls,
                refresh_statuses,
                check_standing,
                understanding,
                memory_standing,
                open_concerns,
                prior_reviews,
            },
            observation,
        ))
    }

    /// Admits a background dispatch [`ReviewRequest`] (`"new records"` or
    /// `"periodic"`) into the shared patient review and practice-job pipeline.
    pub async fn submit_dispatched_patient_review(
        &self,
        request: &ReviewRequest,
        roles: &ModelRoles,
        skills: &PhysicianSkills,
        authority: &dyn PracticeAuthority,
    ) -> Result<AdmittedPatientReview, PatientReviewError> {
        let practice = validate_id(&request.practice_id)?;
        let workspace = validate_id(&request.patient_workspace_id)?;
        let command_key = validate_id(&request.command_key)?;

        authority
            .check_practice_patient(&PracticeScope {
                practice: practice.clone(),
                workspace: workspace.clone(),
                job_id: "review-dispatch-admission".into(),
            })
            .await?;

        let (baseline_records, last_reviewed_at) = {
            let latest_completed = patient_review::Entity::find()
                .filter(patient_review::Column::Practice.eq(&practice))
                .filter(patient_review::Column::Workspace.eq(&workspace))
                .filter(patient_review::Column::Status.eq("completed"))
                .order_by_desc(patient_review::Column::CompletedAt)
                .order_by_desc(patient_review::Column::UpdatedAt)
                .one(&self.db)
                .await?;
            let dispatch_row =
                patient_dispatch::Entity::find_by_id((practice.clone(), workspace.clone()))
                    .one(&self.db)
                    .await?;
            let last_rev_ts = latest_completed
                .as_ref()
                .and_then(|r| r.completed_at)
                .or_else(|| dispatch_row.as_ref().and_then(|d| d.last_reviewed_at));
            let baseline = if let Some(prev) = latest_completed {
                let completed_raw = prev
                    .completed_sources
                    .ok_or_else(|| error("completed patient review missing completed_sources"))?;
                let sources =
                    decode_sources_json(completed_raw, "patient_review.completed_sources")?;
                record_versions_map(&sources)
            } else {
                BTreeMap::new()
            };
            (baseline, last_rev_ts)
        };

        let changed_candidates = match request.reason {
            patient_work::ReviewReason::NewRecords => {
                diff_changed_candidates(&baseline_records, &request.record_versions)
            }
            patient_work::ReviewReason::Periodic => Vec::new(),
        };
        let trigger =
            ReviewTrigger::from_dispatch_request(request, changed_candidates, last_reviewed_at);
        self.admit_patient_review_with_trigger(
            &practice,
            &workspace,
            &command_key,
            trigger,
            roles,
            skills,
            &VerifiedPracticePatient,
        )
        .await
    }

    /// Admits a physician-requested patient review onto the exact same execution
    /// path as changed-record and periodic reviews after enforcing physician
    /// panel authorization (`Membership::Allowed`).
    #[allow(clippy::too_many_arguments)]
    pub async fn submit_physician_patient_review(
        &self,
        auth: &dyn AuthStore,
        identity: &PhysicianIdentity,
        workspace: &str,
        command_key: &str,
        request_text: &str,
        roles: &ModelRoles,
        skills: &PhysicianSkills,
    ) -> Result<AdmittedPatientReview, PatientReviewError> {
        let practice = validate_id(&identity.practice_id)?;
        let physician = validate_id(&identity.physician_id)?;
        let workspace = validate_id(workspace)?;
        let command_key = validate_id(command_key)?;
        let trimmed_request = request_text.trim();
        if trimmed_request.is_empty()
            || trimmed_request
                .chars()
                .any(|ch| ch.is_control() && ch != '\n' && ch != '\t')
        {
            return Err(PatientReviewError::InvalidInput {
                reason: "physician review request text must be non-empty".into(),
            });
        }

        let membership = auth.membership(identity, &workspace).await;
        if membership != Membership::Allowed {
            return Err(PatientReviewError::PhysicianUnauthorized {
                practice_id: practice,
                physician_id: physician,
                workspace,
                membership,
            });
        }

        let observation = self.sources(&workspace).await?;
        let trigger = ReviewTrigger::PhysicianRequest {
            physician_id: physician,
            request_text: trimmed_request.to_owned(),
            workspace_version: observation.version,
            record_versions: record_versions_map(&observation.sources),
        };
        self.admit_patient_review_with_trigger(
            &practice,
            &workspace,
            &command_key,
            trigger,
            roles,
            skills,
            &VerifiedPracticePatient,
        )
        .await
    }

    /// Shared once-only review admission owner for changed-record, periodic, and
    /// physician-requested patient reviews.
    #[allow(clippy::too_many_arguments)]
    pub async fn admit_patient_review_with_trigger(
        &self,
        practice: &str,
        workspace: &str,
        command_key: &str,
        trigger: ReviewTrigger,
        roles: &ModelRoles,
        skills: &PhysicianSkills,
        authority: &dyn PracticeAuthority,
    ) -> Result<AdmittedPatientReview, PatientReviewError> {
        let practice = validate_id(practice)?;
        let workspace = validate_id(workspace)?;
        let command_key = validate_id(command_key)?;
        let review_id = deterministic_review_id(&workspace, &command_key);
        let job_id = review_job_id(&review_id);
        let scope = PracticeScope {
            practice: practice.clone(),
            workspace: workspace.clone(),
            job_id: job_id.clone(),
        };
        authority.check_practice_patient(&scope).await?;

        if let Some(existing_row) = patient_review::Entity::find()
            .filter(patient_review::Column::Practice.eq(&practice))
            .filter(patient_review::Column::Workspace.eq(&workspace))
            .filter(patient_review::Column::CommandKey.eq(&command_key))
            .one(&self.db)
            .await?
        {
            let existing = decode_saved_review(existing_row)?;
            let delivery = Delivery::practice_job(&existing.practice_scope(), command_key);
            return Ok(AdmittedPatientReview::Duplicate {
                review: Box::new(existing),
                delivery,
            });
        }

        let (starting_context, observation) = self
            .review_starting_context(
                &practice,
                &workspace,
                &review_id,
                trigger.clone(),
                Vec::new(),
            )
            .await?;
        let patient_snapshot = starting_context.patient.clone();
        let assignment = build_review_assignment(&command_key, starting_context, skills)?;
        let job_accept = self
            .accept_practice_job(&scope, &assignment, roles, skills, &VerifiedPracticePatient)
            .await
            .map_err(|err| match err {
                super::jobs::PracticeJobError::Access(a) => PatientReviewError::Access(a),
                super::jobs::PracticeJobError::Assignment(a) => PatientReviewError::Assignment(a),
                super::jobs::PracticeJobError::Runtime(r) => PatientReviewError::Runtime(r),
            })?;

        let job = self
            .practice_job(&scope)
            .await?
            .ok_or_else(|| error("accepted practice job missing from store"))?;
        let recipe_version = build_recipe_version(roles, &job.binding, &job.skills, &assignment)?;

        let tx = self.db.begin().await?;
        if let Some(existing_row) = patient_review::Entity::find_by_id((
            practice.clone(),
            workspace.clone(),
            review_id.clone(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        {
            let existing = decode_saved_review(existing_row)?;
            tx.commit().await?;
            let delivery = Delivery::practice_job(&scope, command_key);
            return Ok(AdmittedPatientReview::Duplicate {
                review: Box::new(existing),
                delivery,
            });
        }

        let clock = now(&tx).await?;
        let prior_incomplete = patient_review::Entity::find()
            .filter(patient_review::Column::Practice.eq(&practice))
            .filter(patient_review::Column::Workspace.eq(&workspace))
            .filter(patient_review::Column::ReviewId.ne(&review_id))
            .filter(patient_review::Column::Status.is_in(["pending", "running", "held", "failed"]))
            .lock_exclusive()
            .all(&tx)
            .await?;
        for prior_row in prior_incomplete {
            Self::supersede_review_in_tx(
                &tx,
                prior_row,
                ReviewSupersededReason::NewInput {
                    replacement_review_id: Some(review_id.clone()),
                },
                clock,
            )
            .await?;
        }

        let initial_version_i64 = i64::try_from(observation.version)
            .map_err(conversion_error("initial_source_version", "i64"))?;
        let state = ReviewLifecycleState::Pending;
        let usage = ReviewUsage::Unknown;

        patient_review::ActiveModel {
            practice: Set(practice.clone()),
            workspace: Set(workspace.clone()),
            review_id: Set(review_id.clone()),
            command_key: Set(command_key.clone()),
            job_id: Set(job_id.clone()),
            work_id: Set(job.work.clone()),
            status: Set(state.status_str().to_owned()),
            patient_snapshot: Set(patient_snapshot
                .as_ref()
                .map(serde_json::to_value)
                .transpose()
                .map_err(conversion_error("SourcePatient", "json"))?),
            triggers: Set(serde_json::to_value(vec![trigger.clone()])
                .map_err(conversion_error("Vec<ReviewTrigger>", "json"))?),
            accepted_inputs: Set(json!([])),
            recipe_version: Set(serde_json::to_value(&recipe_version)
                .map_err(conversion_error("ReviewRecipeVersion", "json"))?),
            initial_source_version: Set(initial_version_i64),
            initial_sources: Set(serde_json::to_value(&observation.sources)
                .map_err(conversion_error("Vec<SourceRevision>", "json"))?),
            completed_source_version: Set(None),
            completed_sources: Set(None),
            evidence_read: Set(json!([])),
            actions: Set(json!([])),
            unfinished_work: Set(json!([])),
            state: Set(serde_json::to_value(&state)
                .map_err(conversion_error("ReviewLifecycleState", "json"))?),
            usage: Set(
                serde_json::to_value(&usage).map_err(conversion_error("ReviewUsage", "json"))?
            ),
            duration_ms: Set(None),
            staged_completion: Set(None),
            queued_at: Set(clock),
            started_at: Set(None),
            completed_at: Set(None),
            updated_at: Set(clock),
        }
        .insert(&tx)
        .await?;
        tx.commit().await?;

        let saved = SavedPatientReview {
            practice,
            workspace,
            review_id,
            command_key: command_key.clone(),
            job_id,
            work_id: job.work,
            patient: patient_snapshot,
            triggers: vec![trigger],
            accepted_inputs: Vec::new(),
            recipe_version,
            initial_source_version: observation.version,
            completed_source_version: None,
            evidence_read: Vec::new(),
            actions: Vec::new(),
            unfinished_work: Vec::new(),
            state,
            usage,
            duration_ms: None,
            queued_at: clock,
            started_at: None,
            completed_at: None,
            updated_at: clock,
        };
        let _ = job_accept;
        let delivery = Delivery::practice_job(&scope, command_key);
        Ok(AdmittedPatientReview::New {
            review: Box::new(saved),
            delivery,
        })
    }

    async fn patient_review_internal(
        &self,
        practice: &str,
        workspace: &str,
        review_id: &str,
    ) -> Result<Option<SavedPatientReview>, PatientReviewError> {
        patient_review::Entity::find_by_id((
            practice.to_owned(),
            workspace.to_owned(),
            review_id.to_owned(),
        ))
        .one(&self.db)
        .await?
        .map(decode_saved_review)
        .transpose()
        .map_err(Into::into)
    }

    /// Reads one saved patient review by `(practice, workspace, review_id)` after
    /// verifying practice patient membership.
    pub async fn patient_review(
        &self,
        practice: &str,
        workspace: &str,
        review_id: &str,
        authority: &dyn PracticeAuthority,
    ) -> Result<Option<SavedPatientReview>, PatientReviewError> {
        let practice = validate_id(practice)?;
        let workspace = validate_id(workspace)?;
        let review_id = validate_id(review_id)?;
        authority
            .check_practice_patient(&review_membership_scope(&practice, &workspace))
            .await?;
        self.patient_review_internal(&practice, &workspace, &review_id)
            .await
    }

    /// Lists saved patient reviews for `(practice, workspace)`, newest first, after
    /// verifying practice patient membership.
    pub async fn patient_reviews(
        &self,
        practice: &str,
        workspace: &str,
        authority: &dyn PracticeAuthority,
    ) -> Result<Vec<SavedPatientReview>, PatientReviewError> {
        let practice = validate_id(practice)?;
        let workspace = validate_id(workspace)?;
        authority
            .check_practice_patient(&review_membership_scope(&practice, &workspace))
            .await?;
        let rows = patient_review::Entity::find()
            .filter(patient_review::Column::Practice.eq(practice))
            .filter(patient_review::Column::Workspace.eq(workspace))
            .order_by_desc(patient_review::Column::UpdatedAt)
            .order_by_desc(patient_review::Column::ReviewId)
            .all(&self.db)
            .await?;
        rows.into_iter()
            .map(decode_saved_review)
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    /// Accepts updated input or trigger direction onto an incomplete patient
    /// review (`Pending`, `Running`, or `Held`), advancing the job revision so
    /// any in-flight attempt's stale candidate cannot complete (`Superseded`) and
    /// rearming execution with the updated request and current chart evidence.
    pub async fn submit_patient_review_input(
        &self,
        target: PatientReviewTarget<'_>,
        submission: ReviewInputSubmission<'_>,
        roles: &ModelRoles,
        skills: &PhysicianSkills,
        authority: &dyn PracticeAuthority,
    ) -> Result<Delivery, PatientReviewError> {
        let practice = validate_id(target.practice)?;
        let workspace = validate_id(target.workspace)?;
        let review_id = validate_id(target.review_id)?;
        let input_key = validate_id(submission.input_key)?;
        let additional_trigger = submission.additional_trigger;
        let trimmed = submission.input_text.trim();
        if trimmed.is_empty()
            || trimmed
                .chars()
                .any(|ch| ch.is_control() && ch != '\n' && ch != '\t')
        {
            return Err(PatientReviewError::InvalidInput {
                reason: "review input text must be non-empty".into(),
            });
        }

        authority
            .check_practice_patient(&review_membership_scope(&practice, &workspace))
            .await?;
        let existing = self
            .patient_review_internal(&practice, &workspace, &review_id)
            .await?
            .ok_or_else(|| PatientReviewError::NotFound {
                practice: practice.clone(),
                workspace: workspace.clone(),
                review_id: review_id.clone(),
            })?;
        let scope = existing.practice_scope();
        authority.check_practice_patient(&scope).await?;

        let preview_trigger = existing
            .triggers
            .first()
            .cloned()
            .ok_or_else(|| error("corrupt patient review missing primary trigger"))?;
        let (mut starting_context, observation) = self
            .review_starting_context(
                &practice,
                &workspace,
                &review_id,
                preview_trigger,
                Vec::new(),
            )
            .await?;

        let (tx, mut job, clock) = self.locked_practice_job(&scope).await?;
        let Some(rev_row) = patient_review::Entity::find_by_id((
            practice.clone(),
            workspace.clone(),
            review_id.clone(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        else {
            return Err(PatientReviewError::NotFound {
                practice,
                workspace,
                review_id,
            });
        };
        let locked_review = decode_saved_review(rev_row.clone())?;

        if let Some(prev) = locked_review
            .accepted_inputs
            .iter()
            .find(|i| i.id == input_key)
        {
            if prev.text == trimmed {
                tx.commit().await?;
                return Ok(Delivery::practice_job(&scope, locked_review.command_key));
            }
            tx.rollback().await?;
            return Err(PatientReviewError::IdempotencyCollision { key: input_key });
        }
        if matches!(
            locked_review.state,
            ReviewLifecycleState::Completed { .. } | ReviewLifecycleState::Superseded { .. }
        ) {
            tx.rollback().await?;
            return Err(PatientReviewError::ControlRefused {
                review_id,
                state: locked_review.state.status_str().to_owned(),
                action: "steer",
            });
        }

        let mut next_inputs = locked_review.accepted_inputs.clone();
        next_inputs.push(ReviewAcceptedInput {
            id: input_key,
            text: trimmed.to_owned(),
            recorded_at: clock,
        });
        let next_input_texts: Vec<String> = next_inputs.iter().map(|i| i.text.clone()).collect();
        let primary_trigger = locked_review
            .triggers
            .first()
            .cloned()
            .ok_or_else(|| error("corrupt patient review missing primary trigger"))?;
        starting_context.trigger = primary_trigger;
        starting_context.additional_inputs = next_input_texts;
        let updated_assignment =
            build_review_assignment(&locked_review.command_key, starting_context, skills)?;

        let mut triggers = locked_review.triggers;
        if let Some(trig) = additional_trigger {
            triggers.push(trig);
        }

        let (binding, package) =
            Self::refresh_job_binding_in_tx(&tx, &scope, &updated_assignment, roles, skills)
                .await?;
        job.assignment = updated_assignment;
        job.binding = binding;
        job.skills = package;
        job.sources_version = observation.version;
        job.sources = observation.sources.clone();
        job.result = None;
        job.revision += 1;
        if let Some(attempt) = job
            .attempts
            .last_mut()
            .filter(|a| a.status == AttemptStatus::Active)
        {
            attempt.status = AttemptStatus::Cancelled;
        }
        if !job.status.running() {
            super::jobs::load_attempt_events_for_reviews(&tx, &scope, &mut job.attempts).await?;
            job.allowance_after =
                engine::recovery::resume_boundary(job.attempts.iter().flat_map(|a| &a.events));
            job.failures = job
                .runs
                .start(Uuid::new_v4().to_string())
                .map_err(RuntimeError::from)?;
        }
        job.status = WorkStatus::Pending;
        job.dispatch_after = 0;
        job.updated_at = clock;

        let refreshed_recipe =
            build_recipe_version(roles, &job.binding, &job.skills, &job.assignment)?;
        let next_state = ReviewLifecycleState::Pending;
        patient_review::ActiveModel {
            practice: Unchanged(rev_row.practice),
            workspace: Unchanged(rev_row.workspace),
            review_id: Unchanged(rev_row.review_id),
            command_key: Unchanged(rev_row.command_key.clone()),
            job_id: Unchanged(rev_row.job_id),
            work_id: Set(job.work.clone()),
            status: Set(next_state.status_str().to_owned()),
            patient_snapshot: Unchanged(rev_row.patient_snapshot),
            triggers: Set(serde_json::to_value(&triggers)
                .map_err(conversion_error("Vec<ReviewTrigger>", "json"))?),
            accepted_inputs: Set(serde_json::to_value(&next_inputs)
                .map_err(conversion_error("Vec<ReviewAcceptedInput>", "json"))?),
            recipe_version: Set(serde_json::to_value(&refreshed_recipe)
                .map_err(conversion_error("ReviewRecipeVersion", "json"))?),
            // Preserve `initial_source_version` / `initial_sources` so records added,
            // corrected, or withdrawn between admission and steering remain fenced!
            initial_source_version: Unchanged(rev_row.initial_source_version),
            initial_sources: Unchanged(rev_row.initial_sources),
            completed_source_version: Set(None),
            completed_sources: Set(None),
            evidence_read: Unchanged(rev_row.evidence_read),
            actions: Unchanged(rev_row.actions),
            unfinished_work: Unchanged(rev_row.unfinished_work),
            state: Set(serde_json::to_value(&next_state)
                .map_err(conversion_error("ReviewLifecycleState", "json"))?),
            usage: Unchanged(rev_row.usage),
            duration_ms: Set(None),
            staged_completion: Set(None),
            queued_at: Unchanged(rev_row.queued_at),
            started_at: Unchanged(rev_row.started_at),
            completed_at: Set(None),
            updated_at: Set(clock),
        }
        .update(&tx)
        .await?;

        let delivery = Delivery::practice_job(&scope, job.assignment.key.clone());
        if job.status == WorkStatus::Pending {
            super::delivery::notify(&tx, &delivery).await?;
        }
        Self::save_practice_job_in_tx(tx, &job).await?;
        Ok(delivery)
    }

    /// Stops a pending or active patient review after verifying practice patient
    /// membership, holding both the practice job and the saved review across
    /// process restart.
    pub async fn stop_patient_review(
        &self,
        practice: &str,
        workspace: &str,
        review_id: &str,
        command_key: &str,
        authority: &dyn PracticeAuthority,
    ) -> Result<Stopped, PatientReviewError> {
        let practice = validate_id(practice)?;
        let workspace = validate_id(workspace)?;
        let review_id = validate_id(review_id)?;
        let command_key = validate_id(command_key)?;
        authority
            .check_practice_patient(&review_membership_scope(&practice, &workspace))
            .await?;
        let review = self
            .patient_review_internal(&practice, &workspace, &review_id)
            .await?
            .ok_or_else(|| PatientReviewError::NotFound {
                practice: practice.clone(),
                workspace: workspace.clone(),
                review_id: review_id.clone(),
            })?;
        let scope = review.practice_scope();
        self.stop_practice_job(&scope, &command_key, &review.work_id)
            .await
            .map_err(Into::into)
    }

    /// Resumes a `Stopped` patient review after re-verifying practice patient
    /// access and refreshing starting context with the current request and
    /// current chart evidence.
    pub async fn resume_patient_review(
        &self,
        target: PatientReviewTarget<'_>,
        command_key: &str,
        roles: &ModelRoles,
        skills: &PhysicianSkills,
        authority: &dyn PracticeAuthority,
    ) -> Result<Delivery, PatientReviewError> {
        self.rearm_patient_review_internal(
            target,
            command_key,
            RearmKind::Resume,
            roles,
            skills,
            authority,
        )
        .await
    }

    /// Rearms a `Failed` or limit-`Held` patient review (`Try again`) after
    /// re-verifying practice patient access and refreshing starting context with
    /// the current request and current chart evidence.
    pub async fn try_again_patient_review(
        &self,
        target: PatientReviewTarget<'_>,
        command_key: &str,
        roles: &ModelRoles,
        skills: &PhysicianSkills,
        authority: &dyn PracticeAuthority,
    ) -> Result<Delivery, PatientReviewError> {
        self.rearm_patient_review_internal(
            target,
            command_key,
            RearmKind::TryAgain,
            roles,
            skills,
            authority,
        )
        .await
    }

    async fn rearm_patient_review_internal(
        &self,
        target: PatientReviewTarget<'_>,
        command_key: &str,
        rearm_kind: RearmKind,
        roles: &ModelRoles,
        skills: &PhysicianSkills,
        authority: &dyn PracticeAuthority,
    ) -> Result<Delivery, PatientReviewError> {
        let practice = validate_id(target.practice)?;
        let workspace = validate_id(target.workspace)?;
        let review_id = validate_id(target.review_id)?;
        let command_key = validate_id(command_key)?;

        // Re-verify practice patient access first. If access was lost, record any
        // existing review as superseded by access loss so the old result can never complete.
        if let Err(access_err) = authority
            .check_practice_patient(&review_membership_scope(&practice, &workspace))
            .await
        {
            self.mark_review_access_lost(
                &practice,
                &workspace,
                &review_id,
                &access_err.to_string(),
            )
            .await?;
            return Err(PatientReviewError::Access(access_err));
        }

        let existing = self
            .patient_review_internal(&practice, &workspace, &review_id)
            .await?
            .ok_or_else(|| PatientReviewError::NotFound {
                practice: practice.clone(),
                workspace: workspace.clone(),
                review_id: review_id.clone(),
            })?;
        let scope = existing.practice_scope();
        if let Err(access_err) = authority.check_practice_patient(&scope).await {
            self.mark_review_access_lost(
                &practice,
                &workspace,
                &review_id,
                &access_err.to_string(),
            )
            .await?;
            return Err(PatientReviewError::Access(access_err));
        }

        let action_label = match rearm_kind {
            RearmKind::Resume => "resume",
            RearmKind::TryAgain => "try_again",
        };
        let preview_trigger = existing
            .triggers
            .first()
            .cloned()
            .ok_or_else(|| error("corrupt patient review missing primary trigger"))?;
        let (mut starting_context, observation) = self
            .review_starting_context(
                &practice,
                &workspace,
                &review_id,
                preview_trigger,
                Vec::new(),
            )
            .await?;

        let (tx, mut job, clock) = self.locked_practice_job(&scope).await?;
        super::jobs::load_attempt_events_for_reviews(&tx, &scope, &mut job.attempts).await?;
        let Some(rev_row) = patient_review::Entity::find_by_id((
            practice.clone(),
            workspace.clone(),
            review_id.clone(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        else {
            return Err(PatientReviewError::NotFound {
                practice,
                workspace,
                review_id,
            });
        };
        let locked_review = decode_saved_review(rev_row.clone())?;

        let command_envelope = json!({
            "kind": action_label,
            "practice": &practice,
            "workspace": &workspace,
            "review_id": &review_id,
            "work_id": &locked_review.work_id,
        });
        match receipt::record(
            &tx,
            &practice,
            REVIEW_MEMORY_ACTOR,
            &command_key,
            &command_envelope,
        )
        .await?
        {
            Receipt::New => {}
            Receipt::Same => {
                tx.commit().await?;
                return Ok(Delivery::practice_job(&scope, locked_review.command_key));
            }
            Receipt::Different => {
                tx.rollback().await?;
                return Err(PatientReviewError::IdempotencyCollision { key: command_key });
            }
        }

        let eligible = matches!(
            (&locked_review.state, rearm_kind),
            (
                ReviewLifecycleState::Held {
                    reason: ReviewHoldReason::Stopped,
                    ..
                },
                RearmKind::Resume,
            ) | (
                ReviewLifecycleState::Held {
                    reason: ReviewHoldReason::Limit,
                    ..
                } | ReviewLifecycleState::Failed { .. },
                RearmKind::TryAgain,
            )
        );
        if !eligible {
            tx.rollback().await?;
            return Err(PatientReviewError::ControlRefused {
                review_id,
                state: locked_review.state.status_str().to_owned(),
                action: action_label,
            });
        }

        let primary_trigger = locked_review
            .triggers
            .first()
            .cloned()
            .ok_or_else(|| error("corrupt patient review missing primary trigger"))?;
        let extra_texts: Vec<String> = locked_review
            .accepted_inputs
            .iter()
            .map(|i| i.text.clone())
            .collect();
        starting_context.trigger = primary_trigger;
        starting_context.additional_inputs = extra_texts;
        let updated_assignment =
            build_review_assignment(&locked_review.command_key, starting_context.clone(), skills)?;

        let prior_initial_sources = decode_initial_sources(rev_row.initial_sources.clone())?;
        let sources_moved = !SourceRefresh::between(
            locked_review.initial_source_version,
            &prior_initial_sources,
            observation.version,
            &observation.sources,
        )
        .is_empty();

        let (binding, package) =
            Self::refresh_job_binding_in_tx(&tx, &scope, &updated_assignment, roles, skills)
                .await?;
        job.assignment = updated_assignment;
        job.binding = binding;
        job.skills = package;
        job.sources_version = observation.version;
        job.sources = observation.sources.clone();
        if sources_moved {
            // Invalidate any partial or terminal turn produced against older chart evidence.
            job.result = None;
            job.revision += 1;
        }
        job.allowance_after =
            engine::recovery::resume_boundary(job.attempts.iter().flat_map(|a| &a.events));
        job.failures = job
            .runs
            .start(Uuid::new_v4().to_string())
            .map_err(RuntimeError::from)?;
        job.status = WorkStatus::Pending;
        job.dispatch_after = 0;
        job.updated_at = clock;

        let refreshed_recipe =
            build_recipe_version(roles, &job.binding, &job.skills, &job.assignment)?;
        let next_state = ReviewLifecycleState::Pending;
        patient_review::ActiveModel {
            practice: Unchanged(rev_row.practice),
            workspace: Unchanged(rev_row.workspace),
            review_id: Unchanged(rev_row.review_id),
            command_key: Unchanged(rev_row.command_key.clone()),
            job_id: Unchanged(rev_row.job_id),
            work_id: Set(job.work.clone()),
            status: Set(next_state.status_str().to_owned()),
            patient_snapshot: Set(starting_context
                .patient
                .as_ref()
                .map(serde_json::to_value)
                .transpose()
                .map_err(conversion_error("SourcePatient", "json"))?),
            triggers: Unchanged(rev_row.triggers),
            accepted_inputs: Unchanged(rev_row.accepted_inputs),
            recipe_version: Set(serde_json::to_value(&refreshed_recipe)
                .map_err(conversion_error("ReviewRecipeVersion", "json"))?),
            // Preserve `initial_source_version` / `initial_sources` when sources moved so
            // recovery's `source_feedback` and `verify_review_evidence_currency` require
            // reading the changed records before completing!
            initial_source_version: Unchanged(rev_row.initial_source_version),
            initial_sources: Unchanged(rev_row.initial_sources),
            completed_source_version: Set(None),
            completed_sources: Set(None),
            evidence_read: Unchanged(rev_row.evidence_read),
            actions: Unchanged(rev_row.actions),
            unfinished_work: Unchanged(rev_row.unfinished_work),
            state: Set(serde_json::to_value(&next_state)
                .map_err(conversion_error("ReviewLifecycleState", "json"))?),
            usage: Unchanged(rev_row.usage),
            duration_ms: Set(None),
            staged_completion: if sources_moved {
                Set(None)
            } else {
                Unchanged(rev_row.staged_completion)
            },
            queued_at: Unchanged(rev_row.queued_at),
            started_at: Unchanged(rev_row.started_at),
            completed_at: Set(None),
            updated_at: Set(clock),
        }
        .update(&tx)
        .await?;

        let delivery = Delivery::practice_job(&scope, job.assignment.key.clone());
        super::delivery::notify(&tx, &delivery).await?;
        Self::save_practice_job_in_tx(tx, &job).await?;
        Ok(delivery)
    }

    async fn mark_review_access_lost(
        &self,
        practice: &str,
        workspace: &str,
        review_id: &str,
        detail: &str,
    ) -> Result<(), DbErr> {
        let tx = self.db.begin().await?;
        let clock = now(&tx).await?;
        let Some(rev_row) = patient_review::Entity::find_by_id((
            practice.to_owned(),
            workspace.to_owned(),
            review_id.to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        else {
            tx.commit().await?;
            return Ok(());
        };
        Self::supersede_review_in_tx(
            &tx,
            rev_row,
            ReviewSupersededReason::AccessLost {
                detail: detail.to_owned(),
            },
            clock,
        )
        .await?;
        tx.commit().await
    }

    async fn supersede_review_in_tx(
        tx: &sea_orm::DatabaseTransaction,
        rev_row: patient_review::Model,
        reason: ReviewSupersededReason,
        clock: i64,
    ) -> Result<(), DbErr> {
        if matches!(rev_row.status.as_str(), "completed" | "superseded") {
            return Ok(());
        }
        let scope = PracticeScope {
            practice: rev_row.practice.clone(),
            workspace: rev_row.workspace.clone(),
            job_id: rev_row.job_id.clone(),
        };
        if let Some(job_row) = practice_job::Entity::find_by_id((
            rev_row.practice.clone(),
            rev_row.workspace.clone(),
            rev_row.job_id.clone(),
        ))
        .lock_exclusive()
        .one(tx)
        .await?
        {
            let mut job = super::jobs::decode_job_for_reviews(tx, &scope, job_row).await?;
            if let Some(attempt) = job
                .attempts
                .last_mut()
                .filter(|a| a.status == AttemptStatus::Active)
            {
                attempt.status = AttemptStatus::Cancelled;
            }
            if job.status.running() {
                job.runs
                    .end(harness::engine::runs::RunOutcome::Held {
                        reason: harness::engine::runs::HoldReason::Stopped,
                    })
                    .map_err(|err| error(&err.to_string()))?;
            }
            job.result = None;
            job.revision += 1;
            job.status = WorkStatus::Held;
            job.updated_at = clock;
            super::jobs::write_practice_job_in_tx(tx, &job).await?;
        }
        let actions = collect_review_actions_in_tx(
            tx,
            &rev_row.practice,
            &rev_row.workspace,
            &rev_row.work_id,
        )
        .await?;
        let superseded = ReviewLifecycleState::Superseded {
            reason,
            superseded_at: clock,
        };
        patient_review::ActiveModel {
            practice: Unchanged(rev_row.practice),
            workspace: Unchanged(rev_row.workspace),
            review_id: Unchanged(rev_row.review_id),
            command_key: Unchanged(rev_row.command_key),
            job_id: Unchanged(rev_row.job_id),
            work_id: Unchanged(rev_row.work_id),
            status: Set(superseded.status_str().to_owned()),
            patient_snapshot: Unchanged(rev_row.patient_snapshot),
            triggers: Unchanged(rev_row.triggers),
            accepted_inputs: Unchanged(rev_row.accepted_inputs),
            recipe_version: Unchanged(rev_row.recipe_version),
            initial_source_version: Unchanged(rev_row.initial_source_version),
            initial_sources: Unchanged(rev_row.initial_sources),
            completed_source_version: Set(None),
            completed_sources: Set(None),
            evidence_read: Unchanged(rev_row.evidence_read),
            actions: Set(serde_json::to_value(&actions)
                .map_err(conversion_error("Vec<RecordedReviewAction>", "json"))?),
            unfinished_work: Unchanged(rev_row.unfinished_work),
            state: Set(serde_json::to_value(&superseded)
                .map_err(conversion_error("ReviewLifecycleState", "json"))?),
            usage: Unchanged(rev_row.usage),
            duration_ms: Set(None),
            staged_completion: Set(None),
            queued_at: Unchanged(rev_row.queued_at),
            started_at: Unchanged(rev_row.started_at),
            completed_at: Set(None),
            updated_at: Set(clock),
        }
        .update(tx)
        .await?;
        Ok(())
    }

    async fn refresh_job_binding_in_tx(
        tx: &sea_orm::DatabaseTransaction,
        scope: &PracticeScope,
        assignment: &super::jobs::Assignment,
        roles: &ModelRoles,
        skills: &PhysicianSkills,
    ) -> Result<(BindingRef, SkillPackageId), DbErr> {
        let package =
            frozen::keep_owner_skills(tx, &scope.practice, PRACTICE_OWNER, skills).await?;
        let built = super::jobs::recipe(roles, assignment, skills)
            .bind(Some(scope.cache_key(assignment.kind)));
        let binding =
            frozen::keep_owner_binding(tx, &scope.practice, PRACTICE_OWNER, &built).await?;
        Ok((binding, package))
    }
}

enum RearmKind {
    Resume,
    TryAgain,
}

fn build_recipe_version(
    roles: &ModelRoles,
    binding: &BindingRef,
    skills: &SkillPackageId,
    assignment: &super::jobs::Assignment,
) -> Result<ReviewRecipeVersion, DbErr> {
    let binding_json =
        serde_json::to_value(binding).map_err(conversion_error("BindingRef", "json"))?;
    let binding_sha256 = binding_json
        .get("id")
        .and_then(Value::as_str)
        .ok_or_else(|| error("BindingRef serialized without id"))?
        .to_owned();
    let skills_json =
        serde_json::to_value(skills).map_err(conversion_error("SkillPackageId", "json"))?;
    let skills_package_id = skills_json
        .as_str()
        .ok_or_else(|| error("SkillPackageId did not serialize as string"))?
        .to_owned();
    Ok(ReviewRecipeVersion {
        model: roles.background.model.clone(),
        effort: roles.background.effort.clone(),
        context_window_tokens: roles.background.context_window_tokens,
        max_input_tokens: roles.background.max_input_tokens,
        binding_sha256,
        skills_package_id,
        preloaded_skills: assignment
            .preloaded_skills
            .iter()
            .map(|s| ReviewSkillDigest {
                name: s.name.clone(),
                sha256: s.sha256.clone(),
            })
            .collect(),
    })
}
