//! Patient Review ("rounds") recipe, starting context, preloaded skill binding,
//! and tool schemas for changed-record, periodic, and physician-requested
//! patient reviews.
//!
//! Coda reviews a patient's changed records and periodically reassesses their
//! longitudinal clinical picture on [`crate::ModelRoles::background`]. It
//! recognizes clinically meaningful changes, revisits unresolved work and prior
//! review decisions without repeating stale alerts, keeps missing or stale
//! sources explicit as limits, leaves proposed medication or care changes as
//! unapproved considerations for the physician, and can finish quietly with
//! `"nothing to do"` when nothing needs physician attention.

use std::collections::BTreeMap;
use std::fmt;

use harness::engine::Tool;
use harness::engine::recipe::RequestRecipe;
use patient_work::{ReviewReason, ReviewRequest};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::ModelBinding;
use crate::initial_understanding::{EvidenceScope, understanding_tools};
use crate::physician_chat::{PhysicianSkills, work_tools};
use crate::physician_sources::{ChartScope, SourcePatient, read_tool, search_tool};
use crate::runtime::chat_operations::{
    DispatchCheckStanding, OpenConcernSummary, PatientMemoryStanding, RecordLimit,
    SourceFreshnessSummary, patient_memory_tool,
};
use crate::runtime::concerns::{ConcernInputError, ExactRecordReference, concern_tool};
use crate::runtime::jobs::{Assignment, AssignmentError, PracticeJobKind};
use crate::runtime::refresh::{SavedSourceStatus, SourceRefreshOutcome, refresh_tool};
use crate::runtime::understanding::PatientUnderstanding;

/// Read-only tool returning saved patient reviews, decisions, and unfinished work
/// for the current patient workspace.
pub const READ_PATIENT_REVIEWS_TOOL: &str = "read_patient_reviews";

/// Structured decision barrier recording a patient review's outcome, verified
/// citations, basis and limitations, and unfinished work before the run finishes.
pub const COMPLETE_PATIENT_REVIEW_TOOL: &str = "complete_patient_review";

/// Maximum number of prior reviews returned in one `read_patient_reviews` list call.
pub const MAX_LISTED_REVIEWS: usize = 25;

/// Maximum length of a review clinical summary, quiet explanation, or basis string.
pub const MAX_REVIEW_TEXT_CHARS: usize = 6000;

/// Maximum length of an unfinished-work topic label.
pub const MAX_UNFINISHED_TOPIC_CHARS: usize = 200;

/// Maximum length of an unfinished-work detail description.
pub const MAX_UNFINISHED_DETAIL_CHARS: usize = 2000;

/// Resident clinical instructions for changed-record, periodic, and
/// physician-requested patient reviews on the background lane.
pub const INSTRUCTIONS: &str = "\
You are Coda performing a longitudinal patient review (rounds) for one patient in this practice. \
No physician is signed in to this background run.

Whether this review was started because patient records changed, a periodic reassessment window \
elapsed, or a physician requested a focused reassessment, follow one disciplined clinical process:

1. Orient from starting context and inspect exact source records:
   - Review the starting context below: patient identity, review trigger, candidate record IDs \
     that changed, source pull freshness, open concerns on Today, durable patient memory, saved \
     patient understanding, and earlier saved reviews (including their decisions and unfinished work).
   - Search with `search_patient_information` and read full source revisions with \
     `read_patient_sources` before stating patient-specific facts or citing records. Never rely on \
     search snippets alone. When recency matters or a connected source appears stale, call \
     `refresh_patient_sources`.
   - Use `read_patient_reviews` when you need full details of earlier review decisions, actions, \
     or unfinished work, and `search_clinical_library` / `read_clinical_library` with clinical \
     concepts only (never patient identifiers) when literature context helps.

2. Exercise calibrated clinical judgment across urgent, nonurgent, conflicting, and routine findings:
   - Urgent or time-critical abnormalities needing prompt physician attention: raise or revise a \
     `patient_concern` with `urgency: \"now\"` citing the exact `candidate_id` and `revision`.
   - Meaningful nonurgent changes (worsening chronic disease control, new clinically significant \
     results, overdue follow-up, or discordant findings needing review today): raise or revise a \
     `patient_concern` with `urgency: \"today\"` citing the exact supporting records.
   - Conflicting evidence across sources or dates: read both records in full, state the conflict \
     plainly with citations to each record, and surface the discrepancy rather than silently \
     choosing one source over the other.
   - Normal or clinically expected routine results with nothing open requiring physician attention: \
     do not raise unnecessary concerns; finish quietly via `complete_patient_review` with \
     `disposition: \"quiet\"` (which records the reason `\"nothing to do\"`) and a concise \
     clinical explanation of what you verified.

3. Reassess longitudinally without repeating stale alerts:
   - On periodic reviews and follow-up record arrivals, check open concerns, prior review \
     decisions, durable memory, and `unfinished_work` before raising anything.
   - Never raise a duplicate concern for an issue already represented by an open concern or \
     already settled by an earlier review unless new, corrected, or worsening evidence changes \
     the assessment.
   - When current chart evidence resolves or supersedes an open concern, retire or revise that \
     concern via `patient_concern`.
   - Update durable `patient_memory` or `save_patient_understanding` only when clinically \
     warranted by a genuine change in longitudinal trajectory, new durable facts, or resolved \
     gaps. Keep physician-supplied corrections distinct from chart-derived clinical summaries, \
     and never overwrite physician corrections without dated chart evidence explained in context.

4. Handle corrected records, explicit evidence limits, and physician authority boundaries:
   - When a laboratory result, report, or note has been corrected or amended, base your \
     assessment on the current revision, cite that current revision, and revise or retire any \
     open concern or summary that relied on the superseded version. If patient sources change \
     while you are working, read the updated revisions with `read_patient_sources` before \
     completing the review.
   - Keep missing, unreadable, or unavailable sources explicit in `basis_and_limitations` and \
     `unfinished_work`. Never treat a missing external record, unreadable attachment, or failed \
     source pull as normal or negative clinical evidence.
   - You act only as a clinical decision-support analyst, never as the prescribing or treating \
     clinician. Never enter orders, send messages, or treat a proposed medication change or \
     diagnostic step as an accepted or active plan. Frame any potential medication or care \
     adjustment strictly as a consideration for the physician to evaluate and decide.

5. Record your structured review outcome before finishing:
   - Preloaded assignment skills are already open below; do not reload them with `skill_view`. \
     Other skills in the catalog (such as `patient-communication`) remain available via \
     `skill_view` when needed, and working notes help organize multi-step chart review.
   - Always call `complete_patient_review` once your assessment and any warranted tool actions \
     are complete, supplying your `disposition` (`\"assessed\"` or `\"quiet\"`), clinical \
     summary or quiet explanation, exact read `citations`, `basis_and_limitations`, and any \
     remaining `unfinished_work` for future reviews.";

/// Why a patient review was started. Changed-record dispatches, periodic
/// reassessments, and physician requests share one execution pipeline.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ReviewTrigger {
    /// One or more patient source records were added, corrected, or withdrawn.
    ChangedRecords {
        /// Workspace source version observed when the trigger fired.
        workspace_version: u64,
        /// Active `(candidate_id -> revision)` map observed when triggered.
        record_versions: BTreeMap<String, String>,
        /// Candidate IDs added, updated, or withdrawn relative to the prior check.
        #[serde(default)]
        changed_candidates: Vec<String>,
    },
    /// Scheduled periodic reassessment elapsed without new record arrivals.
    Periodic {
        /// Workspace source version observed when the periodic window elapsed.
        workspace_version: u64,
        /// Active `(candidate_id -> revision)` map observed when triggered.
        record_versions: BTreeMap<String, String>,
        /// Unix timestamp of the prior review submission, when one existed.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        last_reviewed_at: Option<i64>,
    },
    /// Explicit reassessment requested by an authorized physician.
    PhysicianRequest {
        /// Requesting physician identifier.
        physician_id: String,
        /// Clinical focus or question supplied by the physician.
        request_text: String,
        /// Workspace source version observed when the physician requested the review.
        workspace_version: u64,
        /// Active `(candidate_id -> revision)` map observed when requested.
        record_versions: BTreeMap<String, String>,
    },
}

impl ReviewTrigger {
    /// Canonical trigger reason label (`"new records"`, `"periodic"`, or `"physician request"`).
    #[must_use]
    pub const fn reason_label(&self) -> &'static str {
        match self {
            Self::ChangedRecords { .. } => ReviewReason::NewRecords.as_str(),
            Self::Periodic { .. } => ReviewReason::Periodic.as_str(),
            Self::PhysicianRequest { .. } => "physician request",
        }
    }

    /// Workspace source version observed when this trigger was recorded.
    #[must_use]
    pub const fn workspace_version(&self) -> u64 {
        match self {
            Self::ChangedRecords {
                workspace_version, ..
            }
            | Self::Periodic {
                workspace_version, ..
            }
            | Self::PhysicianRequest {
                workspace_version, ..
            } => *workspace_version,
        }
    }

    /// Per-record revision map observed when this trigger was recorded.
    #[must_use]
    pub const fn record_versions(&self) -> &BTreeMap<String, String> {
        match self {
            Self::ChangedRecords {
                record_versions, ..
            }
            | Self::Periodic {
                record_versions, ..
            }
            | Self::PhysicianRequest {
                record_versions, ..
            } => record_versions,
        }
    }

    /// Constructs a [`ReviewTrigger`] from an admitted background [`ReviewRequest`].
    #[must_use]
    pub fn from_dispatch_request(
        request: &ReviewRequest,
        changed_candidates: Vec<String>,
        last_reviewed_at: Option<i64>,
    ) -> Self {
        match request.reason {
            ReviewReason::NewRecords => Self::ChangedRecords {
                workspace_version: request.workspace_version,
                record_versions: request.record_versions.clone(),
                changed_candidates,
            },
            ReviewReason::Periodic => Self::Periodic {
                workspace_version: request.workspace_version,
                record_versions: request.record_versions.clone(),
                last_reviewed_at,
            },
        }
    }
}

/// Controlled reason recorded when a review completes quietly without needing
/// physician attention, kept distinct from failure, hold, or skipped dispatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuietReviewReason {
    /// Reviewed evidence and open threads need no concern or clinical update.
    #[serde(rename = "nothing to do")]
    NothingToDo,
}

impl QuietReviewReason {
    /// Exact wire and persisted reason string (`"nothing to do"`).
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NothingToDo => "nothing to do",
        }
    }
}

impl fmt::Display for QuietReviewReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Verified chart record citation stored on a completed patient review.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewCitation {
    /// Exact `(candidate_id, revision)` reference verified against chart reads.
    pub reference: ExactRecordReference,
    /// Originating connected source identifier.
    pub source_id: String,
    /// Human-readable source label.
    pub source_label: String,
    /// SHA-256 content digest of the cited source revision.
    pub sha256: String,
    /// Clinical record title or display label.
    pub display_label: String,
    /// Primary ISO calendar date (`YYYY-MM-DD`) on the record, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub as_of: Option<String>,
    /// Bounded clinical excerpt from the verified record.
    pub excerpt: String,
}

/// Deferred follow-up item or unresolved evidence gap recorded on a saved review
/// so subsequent periodic or changed-record reviews can revisit it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnfinishedReviewWork {
    /// Short clinical topic naming the unresolved question or pending record.
    pub topic: String,
    /// Specific follow-up check or missing evidence needed on future reassessment.
    pub detail: String,
    /// Optional structured evidence limit (`missing`, `unreadable`, or `unavailable`)
    /// blocking resolution during this review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<RecordLimit>,
}

/// Completed clinical outcome of a patient review, distinguishing quiet
/// `"nothing to do"` completions from reviews that raised findings or updated state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "disposition", rename_all = "snake_case")]
pub enum ReviewOutcome {
    /// Nothing requires physician attention or clinical state changes.
    Quiet {
        /// Controlled quiet completion reason (`"nothing to do"`).
        reason: QuietReviewReason,
        /// Concise clinical explanation of what was reviewed and why no action is needed.
        explanation: String,
        /// Verified exact chart citations read during the quiet review.
        citations: Vec<ReviewCitation>,
        /// Dated sources, guidance used, and any explicit evidence limitations.
        basis_and_limitations: String,
    },
    /// Clinical findings, concern updates, or warranted memory/summary revisions were recorded.
    Assessed {
        /// Clinical synthesis of what changed, what was decided, and what awaits physician review.
        summary: String,
        /// Verified exact chart citations supporting the assessment.
        citations: Vec<ReviewCitation>,
        /// Dated sources, guidance used, unapproved physician considerations, and explicit limits.
        basis_and_limitations: String,
    },
}

impl ReviewOutcome {
    /// Whether this review completed quietly with `"nothing to do"`.
    #[must_use]
    pub const fn is_quiet(&self) -> bool {
        matches!(self, Self::Quiet { .. })
    }

    /// Controlled reason string (`"nothing to do"` for quiet reviews; clinical
    /// summary for assessed reviews).
    #[must_use]
    pub fn reason_text(&self) -> &str {
        match self {
            Self::Quiet { reason, .. } => reason.as_str(),
            Self::Assessed { summary, .. } => summary.as_str(),
        }
    }

    /// Clinical prose body (`explanation` for quiet reviews; `summary` for assessed reviews).
    #[must_use]
    pub fn clinical_text(&self) -> &str {
        match self {
            Self::Quiet { explanation, .. } => explanation.as_str(),
            Self::Assessed { summary, .. } => summary.as_str(),
        }
    }

    /// Verified source citations attached to this outcome.
    #[must_use]
    pub fn citations(&self) -> &[ReviewCitation] {
        match self {
            Self::Quiet { citations, .. } | Self::Assessed { citations, .. } => citations,
        }
    }

    /// Clinical basis and explicit limitations recorded on this outcome.
    #[must_use]
    pub fn basis_and_limitations(&self) -> &str {
        match self {
            Self::Quiet {
                basis_and_limitations,
                ..
            }
            | Self::Assessed {
                basis_and_limitations,
                ..
            } => basis_and_limitations.as_str(),
        }
    }
}

/// Concise projection of an earlier saved review included in [`ReviewStartingContext`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PriorPatientReviewSummary {
    /// Saved review identifier.
    pub review_id: String,
    /// Unix second when the earlier review completed.
    pub completed_at: i64,
    /// Workspace source version covered when the earlier review completed.
    pub completed_source_version: u64,
    /// Trigger reason labels (`"new records"`, `"periodic"`, `"physician request"`).
    pub triggers: Vec<String>,
    /// `"quiet"` or `"assessed"`.
    pub disposition: String,
    /// `"nothing to do"` for quiet reviews, or the clinical summary for assessed reviews.
    pub reason: String,
    /// Concise descriptions of authoritative actions taken in that review.
    pub actions: Vec<String>,
    /// Unfinished work carried forward from that review.
    pub unfinished_work: Vec<UnfinishedReviewWork>,
}

/// Starting clinical context assembled for a patient review run and refreshed
/// on `Resume` and `Try again`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewStartingContext {
    /// Patient workspace identifier.
    pub workspace: String,
    /// Saved review identifier.
    pub review_id: String,
    /// Patient demographic summary from the latest source pull, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patient: Option<SourcePatient>,
    /// Primary trigger that started this review.
    pub trigger: ReviewTrigger,
    /// Additional coalesced trigger notes or physician follow-up requests.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_inputs: Vec<String>,
    /// Current workspace chart version.
    pub chart_version: u64,
    /// Count of active retained chart records.
    pub chart_record_count: usize,
    /// Connected source coverage (`clinical_chart` vs `duet_patient_shared_only`).
    pub evidence_scope: EvidenceScope,
    /// Per-source successful pull timestamps.
    #[serde(default)]
    pub source_pulls: Vec<SourceFreshnessSummary>,
    /// Saved per-source refresh status (including any recorded source pull failures).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub refresh_statuses: Vec<SavedSourceStatus>,
    /// Dispatcher source-check standing for this workspace.
    pub check_standing: DispatchCheckStanding,
    /// Saved patient understanding (including private operating index and summary), when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub understanding: Option<PatientUnderstanding>,
    /// Durable patient memory standing (physician corrections and clinical summaries).
    pub memory_standing: PatientMemoryStanding,
    /// Open patient concerns currently active on Today.
    #[serde(default)]
    pub open_concerns: Vec<OpenConcernSummary>,
    /// Recent completed reviews for this patient, newest first, with their decisions
    /// and unfinished work.
    #[serde(default)]
    pub prior_reviews: Vec<PriorPatientReviewSummary>,
}

impl ReviewStartingContext {
    /// Renders model-clean clinical starting context with zero ticket IDs or
    /// internal implementation labels.
    #[must_use]
    pub fn developer_text(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "Patient review starting context (workspace {}, review {})",
            self.workspace, self.review_id
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

        match &self.trigger {
            ReviewTrigger::ChangedRecords {
                workspace_version,
                changed_candidates,
                ..
            } => {
                let changed_list = if changed_candidates.is_empty() {
                    "inspect current chart observation via search_patient_information".to_owned()
                } else {
                    changed_candidates.join(", ")
                };
                lines.push(format!(
                    "Review trigger: new records at chart version {workspace_version} (changed candidate IDs: {changed_list})."
                ));
            }
            ReviewTrigger::Periodic {
                workspace_version,
                last_reviewed_at,
                ..
            } => {
                let prior = match last_reviewed_at {
                    Some(ts) => format!("last reviewed at unix {ts}"),
                    None => "no prior periodic review recorded".to_owned(),
                };
                lines.push(format!(
                    "Review trigger: periodic whole-patient reassessment at chart version {workspace_version} ({prior})."
                ));
            }
            ReviewTrigger::PhysicianRequest {
                physician_id,
                request_text,
                workspace_version,
                ..
            } => {
                lines.push(format!(
                    "Review trigger: physician request from {physician_id} at chart version {workspace_version}: {request_text}"
                ));
            }
        }

        for extra in &self.additional_inputs {
            lines.push(format!("Updated review direction: {extra}"));
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
            self.chart_version, self.chart_record_count, pulls_summary
        ));

        if matches!(self.evidence_scope, EvidenceScope::DuetPatientSharedOnly) {
            lines.push(
                "Evidence scope: only patient-shared Duet records are connected; keep conclusions bounded to patient-shared information and state unshared clinical chart gaps explicitly."
                    .to_owned(),
            );
        }

        if let DispatchCheckStanding::FailedSinceLastSuccess {
            last_attempt_at,
            last_success_at,
        } = self.check_standing
        {
            match last_success_at {
                Some(ok_at) => lines.push(format!(
                    "Source limit (unavailable): latest source check at unix {last_attempt_at} failed; retained chart reflects earlier records as of unix {ok_at}."
                )),
                None => lines.push(format!(
                    "Source limit (unavailable): latest source check at unix {last_attempt_at} failed before any source pull succeeded."
                )),
            }
        }

        for status in &self.refresh_statuses {
            match &status.outcome {
                SourceRefreshOutcome::Failed {
                    checked_at, reason, ..
                } => lines.push(format!(
                    "Source refresh limit (unavailable, {} / {}): {} at unix {checked_at}.",
                    status.source_label, status.source_id, reason
                )),
                SourceRefreshOutcome::PartlyUpdated {
                    pulled_at, reason, ..
                } => lines.push(format!(
                    "Source refresh partial limit ({} / {}): {} at unix {pulled_at}.",
                    status.source_label, status.source_id, reason
                )),
                SourceRefreshOutcome::Updated { .. } => {}
            }
        }

        if let Some(saved) = &self.understanding {
            lines.push(format!(
                "Saved patient understanding (revision {}, source version {}; internal working overview is never medication truth):",
                saved.revision, saved.source_version
            ));
            lines.push(format!("- Patient summary: {}", saved.summary.narrative));
            if !saved.summary.material_gaps.is_empty() {
                lines.push(format!(
                    "- Material gaps: {}",
                    saved.summary.material_gaps.join("; ")
                ));
            }
            lines.push(format!(
                "- Internal working overview: {}",
                saved.operating_index.working_overview
            ));
            if !saved.operating_index.open_threads.is_empty() {
                lines.push(format!(
                    "- Internal open threads: {}",
                    saved.operating_index.open_threads.join("; ")
                ));
            }
        }

        match &self.memory_standing {
            PatientMemoryStanding::Available { entries } => {
                if !entries.is_empty() {
                    lines.push("Durable patient memory:".to_owned());
                    for entry in entries {
                        lines.push(format!(
                            "- [{}] {}: {} (revision {}, updated unix {})",
                            entry.id,
                            entry.topic,
                            entry.statement,
                            entry.revision,
                            entry.updated_at
                        ));
                    }
                }
            }
            PatientMemoryStanding::Unavailable { reason } => {
                lines.push(format!(
                    "Memory limit (unavailable): durable patient memory could not be read ({reason})."
                ));
            }
        }

        if !self.open_concerns.is_empty() {
            lines.push("Open patient concerns on Today:".to_owned());
            for concern in &self.open_concerns {
                let refs = concern
                    .records
                    .iter()
                    .map(|r| format!("{}@{}", r.candidate_id, r.revision))
                    .collect::<Vec<_>>()
                    .join(", ");
                lines.push(format!(
                    "- [{}] ({}): {} [cited: {}]",
                    concern.concern_id,
                    concern.urgency.as_str(),
                    concern.reason,
                    refs
                ));
            }
        }

        if !self.prior_reviews.is_empty() {
            lines.push(
                "Prior completed patient reviews (use read_patient_reviews to inspect full details):"
                    .to_owned(),
            );
            for review in &self.prior_reviews {
                let actions = if review.actions.is_empty() {
                    "no durable mutations".to_owned()
                } else {
                    review.actions.join("; ")
                };
                lines.push(format!(
                    "- [{}] completed unix {} at chart v{} (triggers: {}): {} — {} (actions: {})",
                    review.review_id,
                    review.completed_at,
                    review.completed_source_version,
                    review.triggers.join(", "),
                    review.disposition,
                    review.reason,
                    actions
                ));
                for item in &review.unfinished_work {
                    lines.push(format!(
                        "  * Unfinished work [{}]: {}",
                        item.topic, item.detail
                    ));
                }
            }
        }

        lines.join("\n")
    }
}

/// Structured review context stored on [`Assignment::review`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewAssignmentContext {
    /// Saved review identifier in `physician_runtime_patient_reviews`.
    pub review_id: String,
    /// Assembled clinical starting context for this review.
    pub starting_context: ReviewStartingContext,
}

/// Validation failure when parsing `complete_patient_review` or `read_patient_reviews` arguments.
#[derive(Debug, thiserror::Error)]
pub enum ReviewInputError {
    /// Tool JSON did not match the strict schema.
    #[error("invalid patient review tool arguments: {0}")]
    MalformedJson(#[source] serde_json::Error),
    /// Required clinical summary, explanation, or basis text was empty, contained
    /// control characters, or exceeded [`MAX_REVIEW_TEXT_CHARS`].
    #[error("{field} must be 1 to {MAX_REVIEW_TEXT_CHARS} printable characters")]
    InvalidText {
        /// Argument field name.
        field: &'static str,
    },
    /// Unfinished work topic or detail failed length or single-line validation.
    #[error("unfinished_work `{field}` must be non-empty without control characters")]
    InvalidUnfinishedWork {
        /// Subfield name (`topic` or `detail`).
        field: &'static str,
    },
    /// Citation record reference failed boundary validation.
    #[error("invalid citation in `{field}`: {source}")]
    InvalidReference {
        /// Argument field name.
        field: &'static str,
        /// Underlying reference validation cause.
        #[source]
        source: ConcernInputError,
    },
    /// Target `review_id` in `read_patient_reviews` was blank or contained control characters.
    #[error("review_id must be a non-empty identifier without control characters")]
    InvalidReviewId,
}

/// Validated disposition submitted to `complete_patient_review`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "disposition", rename_all = "snake_case")]
pub enum CompleteReviewDisposition {
    /// Quiet completion saving controlled reason `"nothing to do"`.
    Quiet {
        /// Concise explanation of what was checked and why no physician action is needed.
        explanation: String,
    },
    /// Assessed completion recording clinical findings, concern updates, or memory updates.
    Assessed {
        /// Clinical synthesis of findings, decisions, and considerations for the physician.
        summary: String,
    },
}

/// Validated arguments to `complete_patient_review`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompleteReviewArgs {
    /// Whether the review finished quietly (`"nothing to do"`) or with clinical findings.
    #[serde(flatten)]
    pub disposition: CompleteReviewDisposition,
    /// Exact `(candidate_id, revision)` pairs supporting this review decision.
    pub citations: Vec<ExactRecordReference>,
    /// Clinical basis naming dated sources, skill guidance, any unapproved physician
    /// considerations, and explicit missing/unreadable/unavailable limitations.
    pub basis_and_limitations: String,
    /// Deferred follow-ups or unresolved evidence gaps for later reviews to revisit.
    pub unfinished_work: Vec<UnfinishedReviewWork>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "disposition", rename_all = "snake_case", deny_unknown_fields)]
enum RawCompleteReviewArgs {
    Quiet {
        explanation: String,
        #[serde(default)]
        citations: Vec<RawExactRef>,
        basis_and_limitations: String,
        #[serde(default)]
        unfinished_work: Vec<RawUnfinishedItem>,
    },
    Assessed {
        summary: String,
        #[serde(default)]
        citations: Vec<RawExactRef>,
        basis_and_limitations: String,
        #[serde(default)]
        unfinished_work: Vec<RawUnfinishedItem>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawExactRef {
    candidate_id: String,
    revision: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawUnfinishedItem {
    topic: String,
    detail: String,
    #[serde(default)]
    limit: Option<RecordLimit>,
}

fn validate_prose(field: &'static str, value: &str) -> Result<String, ReviewInputError> {
    let trimmed = value.trim();
    let count = trimmed.chars().count();
    if !(1..=MAX_REVIEW_TEXT_CHARS).contains(&count)
        || trimmed
            .chars()
            .any(|ch| ch.is_control() && ch != '\n' && ch != '\t')
    {
        return Err(ReviewInputError::InvalidText { field });
    }
    Ok(trimmed.to_owned())
}

fn validate_unfinished(
    items: Vec<RawUnfinishedItem>,
) -> Result<Vec<UnfinishedReviewWork>, ReviewInputError> {
    let mut validated = Vec::with_capacity(items.len());
    for raw in items {
        if raw.topic.contains('\n') || raw.topic.contains('\r') {
            return Err(ReviewInputError::InvalidUnfinishedWork { field: "topic" });
        }
        let topic = raw.topic.trim();
        let topic_len = topic.chars().count();
        if !(1..=MAX_UNFINISHED_TOPIC_CHARS).contains(&topic_len)
            || topic.chars().any(char::is_control)
        {
            return Err(ReviewInputError::InvalidUnfinishedWork { field: "topic" });
        }
        let detail = raw.detail.trim();
        let detail_len = detail.chars().count();
        if !(1..=MAX_UNFINISHED_DETAIL_CHARS).contains(&detail_len)
            || detail
                .chars()
                .any(|ch| ch.is_control() && ch != '\n' && ch != '\t')
        {
            return Err(ReviewInputError::InvalidUnfinishedWork { field: "detail" });
        }
        validated.push(UnfinishedReviewWork {
            topic: topic.to_owned(),
            detail: detail.to_owned(),
            limit: raw.limit,
        });
    }
    Ok(validated)
}

fn validate_citations(
    raw: Vec<RawExactRef>,
) -> Result<Vec<ExactRecordReference>, ReviewInputError> {
    let mut out = Vec::with_capacity(raw.len());
    let mut seen = std::collections::BTreeSet::new();
    for item in raw {
        let reference =
            ExactRecordReference::new(item.candidate_id, item.revision).map_err(|source| {
                ReviewInputError::InvalidReference {
                    field: "citations",
                    source,
                }
            })?;
        let key = (reference.candidate_id.clone(), reference.revision.clone());
        if seen.insert(key) {
            out.push(reference);
        }
    }
    Ok(out)
}

impl CompleteReviewArgs {
    /// Parses and validates `complete_patient_review` JSON arguments.
    pub fn parse(arguments: &Value) -> Result<Self, ReviewInputError> {
        let raw: RawCompleteReviewArgs =
            serde_json::from_value(arguments.clone()).map_err(ReviewInputError::MalformedJson)?;
        match raw {
            RawCompleteReviewArgs::Quiet {
                explanation,
                citations,
                basis_and_limitations,
                unfinished_work,
            } => Ok(Self {
                disposition: CompleteReviewDisposition::Quiet {
                    explanation: validate_prose("explanation", &explanation)?,
                },
                citations: validate_citations(citations)?,
                basis_and_limitations: validate_prose(
                    "basis_and_limitations",
                    &basis_and_limitations,
                )?,
                unfinished_work: validate_unfinished(unfinished_work)?,
            }),
            RawCompleteReviewArgs::Assessed {
                summary,
                citations,
                basis_and_limitations,
                unfinished_work,
            } => Ok(Self {
                disposition: CompleteReviewDisposition::Assessed {
                    summary: validate_prose("summary", &summary)?,
                },
                citations: validate_citations(citations)?,
                basis_and_limitations: validate_prose(
                    "basis_and_limitations",
                    &basis_and_limitations,
                )?,
                unfinished_work: validate_unfinished(unfinished_work)?,
            }),
        }
    }
}

/// Validated arguments to `read_patient_reviews`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReadReviewsCall {
    /// Lists recent saved reviews for this patient workspace, newest first.
    List {
        /// Maximum number of reviews to return (`1..=MAX_LISTED_REVIEWS`).
        limit: usize,
        /// Whether to include held, pending, or failed reviews alongside completed ones.
        include_incomplete: bool,
    },
    /// Reads one saved review record in full by `review_id`.
    Read {
        /// Saved review identifier.
        review_id: String,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
enum RawReadReviewsCall {
    List {
        #[serde(default)]
        limit: Option<usize>,
        #[serde(default)]
        include_incomplete: bool,
    },
    Read {
        review_id: String,
    },
}

impl ReadReviewsCall {
    /// Parses and validates `read_patient_reviews` JSON arguments.
    pub fn parse(arguments: &Value) -> Result<Self, ReviewInputError> {
        let raw: RawReadReviewsCall =
            serde_json::from_value(arguments.clone()).map_err(ReviewInputError::MalformedJson)?;
        match raw {
            RawReadReviewsCall::List {
                limit,
                include_incomplete,
            } => Ok(Self::List {
                limit: limit.unwrap_or(10).clamp(1, MAX_LISTED_REVIEWS),
                include_incomplete,
            }),
            RawReadReviewsCall::Read { review_id } => {
                let trimmed = review_id.trim();
                if trimmed.is_empty() || trimmed.chars().any(char::is_control) {
                    return Err(ReviewInputError::InvalidReviewId);
                }
                Ok(Self::Read {
                    review_id: trimmed.to_owned(),
                })
            }
        }
    }
}

/// Whether `name` is one of the patient review tools (`read_patient_reviews` or
/// `complete_patient_review`).
#[must_use]
pub fn is_patient_review_tool(name: &str) -> bool {
    matches!(
        name,
        READ_PATIENT_REVIEWS_TOOL | COMPLETE_PATIENT_REVIEW_TOOL
    )
}

/// Whether `name` is a read-only patient review tool.
#[must_use]
pub fn read_only_review_tool(name: &str) -> bool {
    name == READ_PATIENT_REVIEWS_TOOL
}

/// Read-only tool schema for inspecting earlier saved patient reviews, decisions,
/// actions, and unfinished work in the current patient workspace.
#[must_use]
pub fn read_patient_reviews_tool() -> Tool {
    Tool {
        name: READ_PATIENT_REVIEWS_TOOL.into(),
        description: "List or read earlier saved patient reviews for this patient workspace, including triggers, clinical decisions, quiet reasons, verified actions, citations, and unfinished work.".into(),
        parameters: json!({
            "anyOf": [
                {
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string", "enum": ["list"] },
                        "limit": { "type": ["integer", "null"], "minimum": 1, "maximum": MAX_LISTED_REVIEWS },
                        "include_incomplete": { "type": "boolean" }
                    },
                    "required": ["operation"],
                    "additionalProperties": false
                },
                {
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string", "enum": ["read"] },
                        "review_id": { "type": "string", "minLength": 1, "maxLength": 256 }
                    },
                    "required": ["operation", "review_id"],
                    "additionalProperties": false
                }
            ]
        }),
    }
}

/// Write-barrier tool schema recording a patient review's structured disposition,
/// verified citations, basis and limitations, and unfinished work.
#[must_use]
pub fn complete_patient_review_tool() -> Tool {
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
            "topic": { "type": "string", "minLength": 1, "maxLength": MAX_UNFINISHED_TOPIC_CHARS },
            "detail": { "type": "string", "minLength": 1, "maxLength": MAX_UNFINISHED_DETAIL_CHARS },
            "limit": {
                "anyOf": [
                    { "type": "null" },
                    {
                        "type": "object",
                        "properties": {
                            "kind": { "type": "string", "enum": ["missing", "unreadable", "unavailable"] },
                            "subject": { "type": "string", "minLength": 1 },
                            "detail": { "type": "string", "minLength": 1 }
                        },
                        "required": ["kind", "subject", "detail"],
                        "additionalProperties": false
                    }
                ]
            }
        },
        "required": ["topic", "detail"],
        "additionalProperties": false
    });

    Tool {
        name: COMPLETE_PATIENT_REVIEW_TOOL.into(),
        description: "Record the structured clinical outcome of this patient review before finishing. Use `disposition: \"quiet\"` (which records the controlled reason \"nothing to do\") with `explanation` when nothing requires physician attention, or `disposition: \"assessed\"` with `summary` when raising/revising concerns or updating clinical state. Supply exact read `citations`, `basis_and_limitations` (including any explicit missing, unreadable, or unavailable evidence limits and any unapproved considerations left for physician decision), and `unfinished_work` for future reviews.".into(),
        parameters: json!({
            "anyOf": [
                {
                    "type": "object",
                    "properties": {
                        "disposition": { "type": "string", "enum": ["quiet"] },
                        "explanation": { "type": "string", "minLength": 1, "maxLength": MAX_REVIEW_TEXT_CHARS },
                        "citations": { "type": "array", "items": citation_schema },
                        "basis_and_limitations": { "type": "string", "minLength": 1, "maxLength": MAX_REVIEW_TEXT_CHARS },
                        "unfinished_work": { "type": "array", "items": unfinished_schema }
                    },
                    "required": ["disposition", "explanation", "citations", "basis_and_limitations", "unfinished_work"],
                    "additionalProperties": false
                },
                {
                    "type": "object",
                    "properties": {
                        "disposition": { "type": "string", "enum": ["assessed"] },
                        "summary": { "type": "string", "minLength": 1, "maxLength": MAX_REVIEW_TEXT_CHARS },
                        "citations": { "type": "array", "items": citation_schema },
                        "basis_and_limitations": { "type": "string", "minLength": 1, "maxLength": MAX_REVIEW_TEXT_CHARS },
                        "unfinished_work": { "type": "array", "items": unfinished_schema }
                    },
                    "required": ["disposition", "summary", "citations", "basis_and_limitations", "unfinished_work"],
                    "additionalProperties": false
                }
            ]
        }),
    }
}

/// Authorized tools offered to a Patient Review background run. Physician-only
/// actions (message sending, reply drafts, plan approval, feedback) are never offered.
#[must_use]
pub fn tools() -> Vec<Tool> {
    let mut offered = work_tools();
    offered.push(search_tool());
    offered.push(read_tool(ChartScope::Chat));
    offered.push(refresh_tool());
    offered.push(concern_tool());
    offered.push(patient_memory_tool());
    offered.extend(understanding_tools());
    offered.push(read_patient_reviews_tool());
    offered.push(complete_patient_review_tool());
    offered.extend(crate::clinical_library::tools());
    offered
}

/// Builds the Patient Review [`Assignment`] for `command_key` and `starting_context`,
/// preloading `patient-record-retrieval` at assignment start while keeping the
/// rest of the approved skill catalog discoverable via `skill_view`.
pub fn assignment(
    command_key: impl Into<String>,
    starting_context: ReviewStartingContext,
    skills: &PhysicianSkills,
) -> Result<Assignment, AssignmentError> {
    let key = command_key.into();
    let workspace = starting_context.workspace.trim();
    if workspace.is_empty() || starting_context.review_id.trim().is_empty() {
        return Err(AssignmentError::EmptyAssignment);
    }
    let trigger_headline = match &starting_context.trigger {
        ReviewTrigger::ChangedRecords {
            workspace_version,
            changed_candidates,
            ..
        } => {
            if changed_candidates.is_empty() {
                format!(
                    "Review changed patient records for workspace {workspace} at chart version {workspace_version}."
                )
            } else {
                format!(
                    "Review changed patient records ({}) for workspace {workspace} at chart version {workspace_version}.",
                    changed_candidates.join(", ")
                )
            }
        }
        ReviewTrigger::Periodic {
            workspace_version, ..
        } => format!(
            "Perform a periodic clinical reassessment for patient workspace {workspace} at chart version {workspace_version}."
        ),
        ReviewTrigger::PhysicianRequest {
            physician_id,
            request_text,
            workspace_version,
            ..
        } => format!(
            "Perform the physician-requested patient review ({physician_id}) for workspace {workspace} at chart version {workspace_version}: {request_text}"
        ),
    };
    let developer_context = starting_context.developer_text();
    let prompt = format!(
        "{trigger_headline}\n\
         Inspect the relevant dated source revisions with `read_patient_sources`, check prior \
         decisions and open concerns so nothing stale is repeated, record any warranted concern or \
         memory updates, and call `complete_patient_review` before finishing.\n\n\
         {developer_context}"
    );
    let review_id = starting_context.review_id.clone();
    let mut created = Assignment::new(
        PracticeJobKind::PatientReview,
        key,
        prompt,
        &["patient-record-retrieval"],
        skills,
    )?;
    created.review = Some(ReviewAssignmentContext {
        review_id,
        starting_context,
    });
    Ok(created)
}

/// Builds the immutable [`RequestRecipe`] for a Patient Review background run on
/// `background` (`ModelRoles.background`), embedding preloaded skill content while
/// preserving full skill and tool discoverability.
#[must_use]
pub fn recipe(
    background: &ModelBinding,
    assignment: &Assignment,
    skills: &PhysicianSkills,
) -> RequestRecipe {
    let mut instructions =
        skills.instructions(INSTRUCTIONS, crate::physician_chat::Job::PatientReview);
    if !assignment.preloaded_skills.is_empty() {
        let preloaded: Vec<Value> = assignment
            .preloaded_skills
            .iter()
            .map(|skill| {
                json!({
                    "name": skill.name,
                    "sha256": skill.sha256,
                    "content": skill.content,
                })
            })
            .collect();
        let encoded = serde_json::to_string(&json!({
            "kind": "preloaded_assignment_skills",
            "skills": preloaded,
        }))
        .expect("preloaded skills serialize as JSON");
        instructions.push_str(
            "\n\nPreloaded assignment skills (already open in this run; do not reload via skill_view):\n",
        );
        instructions.push_str(&encoded);
    }
    let mut built = RequestRecipe {
        model: background.model.clone(),
        context_window_tokens: background.context_window_tokens,
        max_input_tokens: background.max_input_tokens,
        options: json!({ "effort": background.effort }),
        instructions,
        tools: tools().into_iter().map(|tool| tool.function()).collect(),
    };
    harness::engine::offer_working_notes(&mut built.instructions, &mut built.tools);
    built
}

/// Deterministic background job identifier for a saved patient review.
#[must_use]
pub fn review_job_id(review_id: &str) -> String {
    format!("patient-review:{review_id}")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::initial_understanding::ConnectedSourceBrief;
    use crate::runtime::jobs::is_physician_only_tool;

    fn load_skills() -> PhysicianSkills {
        PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
            .expect("load physician skills catalog")
    }

    fn background_binding() -> ModelBinding {
        ModelBinding {
            model: "background-rounds-model".into(),
            effort: "high".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 250_000,
            adapter: "fixture".into(),
        }
    }

    fn sample_starting_context(trigger: ReviewTrigger) -> ReviewStartingContext {
        ReviewStartingContext {
            workspace: "pw-rounds-1".into(),
            review_id: "rev-1".into(),
            patient: None,
            trigger,
            additional_inputs: Vec::new(),
            chart_version: 2,
            chart_record_count: 3,
            evidence_scope: EvidenceScope::ClinicalChart {
                connected_sources: vec![ConnectedSourceBrief {
                    source_id: "clinic-ehr".into(),
                    source_label: "Clinic EHR".into(),
                    pulled_at: 1_787_850_000,
                }],
            },
            source_pulls: vec![SourceFreshnessSummary {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                pulled_at: 1_787_850_000,
            }],
            refresh_statuses: Vec::new(),
            check_standing: DispatchCheckStanding::Current {
                checked_at: 1_787_850_000,
            },
            understanding: None,
            memory_standing: PatientMemoryStanding::Available {
                entries: Vec::new(),
            },
            open_concerns: Vec::new(),
            prior_reviews: Vec::new(),
        }
    }

    #[test]
    fn recipe_binds_background_lane_preloads_retrieval_keeps_skills_and_tools_discoverable() {
        let skills = load_skills();
        let binding = background_binding();
        let context = sample_starting_context(ReviewTrigger::ChangedRecords {
            workspace_version: 2,
            record_versions: BTreeMap::from([("obs-k-1".into(), "r2".into())]),
            changed_candidates: vec!["obs-k-1".into()],
        });
        let assigned = assignment("cmd-rev-1", context, &skills).expect("build review assignment");
        assert_eq!(assigned.kind, PracticeJobKind::PatientReview);
        assert_eq!(assigned.preloaded_skills.len(), 1);
        assert_eq!(
            assigned.preloaded_skills[0].name,
            "patient-record-retrieval"
        );

        let built = recipe(&binding, &assigned, &skills);
        assert_eq!(built.model, "background-rounds-model");
        assert_eq!(built.context_window_tokens, 272_000);
        assert_eq!(built.max_input_tokens, 250_000);
        assert_eq!(built.options, json!({ "effort": "high" }));
        assert!(built.instructions.contains("preloaded_assignment_skills"));
        assert!(built.instructions.contains("patient-record-retrieval"));
        // Catalog index still advertises patient-communication for on-demand discovery via skill_view.
        assert!(built.instructions.contains("patient-communication"));

        let mut names = Vec::new();
        for tool in &built.tools {
            let name = tool["name"].as_str().expect("tool name");
            assert!(
                !is_physician_only_tool(name),
                "patient review recipe must never offer physician-only tool {name}"
            );
            assert!(
                !tool["description"]
                    .as_str()
                    .expect("tool description")
                    .is_empty()
            );
            names.push(name);
        }
        for expected in [
            crate::physician_sources::SEARCH_TOOL,
            crate::physician_sources::READ_TOOL,
            crate::runtime::refresh::REFRESH_TOOL,
            crate::runtime::concerns::CONCERN_TOOL,
            crate::runtime::chat_operations::PATIENT_MEMORY_TOOL,
            crate::initial_understanding::READ_UNDERSTANDING_TOOL,
            crate::initial_understanding::SAVE_UNDERSTANDING_TOOL,
            READ_PATIENT_REVIEWS_TOOL,
            COMPLETE_PATIENT_REVIEW_TOOL,
            crate::clinical_library::SEARCH_LIBRARY_TOOL,
            crate::clinical_library::READ_LIBRARY_TOOL,
            crate::physician_chat::SKILL_VIEW,
        ] {
            assert!(
                names.contains(&expected),
                "missing expected tool {expected}"
            );
        }
        let unique: std::collections::BTreeSet<&str> = names.iter().copied().collect();
        assert_eq!(
            names.len(),
            unique.len(),
            "duplicate tool names in {names:?}"
        );
    }

    #[test]
    fn quiet_and_assessed_completion_args_enforce_strict_schema_and_preserve_error_sources() {
        let quiet = CompleteReviewArgs::parse(&json!({
            "disposition": "quiet",
            "explanation": "Routine basic metabolic panel within reference range; no open clinical issues.",
            "citations": [{ "candidate_id": "obs-bmp-1", "revision": "r1" }],
            "basis_and_limitations": "Reviewed Clinic EHR BMP dated 2026-11-10 (obs-bmp-1@r1).",
            "unfinished_work": []
        }))
        .expect("valid quiet completion");
        assert!(matches!(
            quiet.disposition,
            CompleteReviewDisposition::Quiet { .. }
        ));
        assert_eq!(QuietReviewReason::NothingToDo.as_str(), "nothing to do");

        let unknown_field = CompleteReviewArgs::parse(&json!({
            "disposition": "quiet",
            "explanation": "Normal.",
            "citations": [],
            "basis_and_limitations": "Basis.",
            "unfinished_work": [],
            "accepted_medication_plan": "lisinopril 20 mg"
        }))
        .expect_err("unknown property must fail strict schema");
        assert!(std::error::Error::source(&unknown_field).is_some());

        let bad_ref = CompleteReviewArgs::parse(&json!({
            "disposition": "assessed",
            "summary": "Potassium elevated.",
            "citations": [{ "candidate_id": "obs\u{0007}bad", "revision": "r1" }],
            "basis_and_limitations": "Basis.",
            "unfinished_work": []
        }))
        .expect_err("control char in citation must fail");
        assert!(std::error::Error::source(&bad_ref).is_some());
    }
}
