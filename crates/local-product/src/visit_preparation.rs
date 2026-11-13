//! Recipe, starting context, preloaded skill binding, and tool descriptors for
//! visit preparation selection and pre-visit briefing authoring.
//!
//! Visit preparation runs on [`crate::ModelRoles::background`] as
//! [`PracticeJobKind::VisitPreparation`] in two typed phases:
//! 1. **Visit selection** ([`SELECT_VISITS_TOOL`]): evaluates upcoming or
//!    in-progress matched practice visits from appointment context and patient
//!    evidence, recording whether each visit prepares immediately, prepares at a
//!    scheduled pre-visit lead time, defers until expected records arrive, or is
//!    skipped with its clinical reason.
//! 2. **Visit briefing authoring** ([`SAVE_BRIEFING_TOOL`]): opens exact chart
//!    revisions, refreshes stale sources when warranted, consults clinical
//!    literature when helpful, saves a cited briefing keyed to the visit's start
//!    time (`trajectory`, `meaningful_changes`, `unresolved_issues`, open
//!    `visit_decisions`, `visit_card_orientation`, and `basis_and_limitations`),
//!    and optionally raises a pre-visit concern (`today` or `now`) when news
//!    warrants physician attention before the encounter.

use harness::engine::Tool;
use harness::engine::recipe::RequestRecipe;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::ModelBinding;
use crate::physician_chat::PhysicianSkills;
use crate::physician_sources::{ChartScope, RecordDate, SourcePatient};
use crate::runtime::chat_operations::{
    DispatchCheckStanding, OpenConcernSummary, PatientStartingContext, RecordLimit,
    RecordLimitKind, SourceFreshnessSummary,
};
use crate::runtime::concerns::{
    ConcernId, ConcernInputError, ConcernUrgency, ExactRecordReference,
};
use crate::runtime::jobs::{Assignment, AssignmentError, PracticeJobKind, PreloadedSkill};
use crate::runtime::schedule::ScheduleStanding;

/// Tool offered to selection runs for recording which visits to prepare and when.
pub const SELECT_VISITS_TOOL: &str = "select_visit_preparations";

/// Write-barrier tool offered to briefing runs for saving a cited visit briefing
/// and optional pre-visit concern.
pub const SAVE_BRIEFING_TOOL: &str = "save_visit_briefing";

/// Preloaded skill name carrying clinical visit-preparation guidance.
pub const VISIT_PREPARATION_SKILL: &str = "visit-preparation";

/// Preloaded skill name carrying patient chart retrieval guidance.
pub const RECORD_RETRIEVAL_SKILL: &str = "patient-record-retrieval";

/// System instructions for selecting upcoming visits and preparation times.
pub const SELECTION_INSTRUCTIONS: &str = "\
You are Coda, selecting which upcoming visits in this practice warrant pre-visit preparation and choosing when each preparation should run.
No physician is signed in to this background run.

Evaluate every candidate visit in the starting context on its clinical purpose, time remaining before the encounter, chart evidence, active patient memory, open concerns, and source freshness. Judge timing per visit rather than applying one fixed rule to every appointment:
- Choose `prepare_now` when a visit is imminent or already in progress, or when hospital discharge summaries, critical interval results, unstable symptoms, or complex medication changes are already present and warrant immediate synthesis.
- Choose `schedule` with `prepare_at_unix` before `starts_at_unix` when an appointment is further ahead and pending results or specialist notes may still arrive before the visit, leaving practical pre-visit review time.
- Choose `defer` with `check_again_at_unix` before `starts_at_unix` when key expected records are not yet available and the visit remains well ahead.
- Choose `skip` only when an encounter genuinely needs no pre-visit chart synthesis, stating the concrete clinical reason.
- Never schedule or start preparation at or after `ends_at_unix`.

Record a decision and a patient-specific clinical `reason` for every candidate visit through `select_visit_preparations`. Preloaded assignment skills are already open below; do not reload them via `skill_view`. Use clinical language, never runtime terminology, model names, or internal identifiers.";

/// System instructions for preparing and saving one visit's clinical briefing.
pub const BRIEFING_INSTRUCTIONS: &str = "\
You are Coda, preparing a pre-visit clinical briefing for one upcoming or in-progress patient encounter in this practice.
No physician is signed in to this background run.

Orient the physician so they enter the visit knowing the patient's trajectory, meaningful interval changes, unresolved clinical work, and the concrete decisions the visit can settle:
- Start from the appointment context and patient starting context, inspect saved understanding (`read_patient_understanding`), active physician corrections (`patient_memory` with `operation: \"read\"`), and prior background work (`retrieve_patient_work`) when helpful.
- When source freshness is stale or new external records could change visit preparation, call `refresh_patient_sources`. If a source refresh fails, continue from the available retained records and state the exact freshness and unavailable source limits truthfully in `basis_and_limitations` and `limits`.
- Discover candidate records with `search_patient_information` and open exact revisions with `read_patient_sources` before relying on or citing patient-specific facts. Every `ExactRecordReference` (`candidate_id` and `revision`) attached to claims, decisions, basis citations, or a pre-visit concern must match a current workspace record read via `read_patient_sources` in this run (or carried forward at a still-current revision from `read_patient_understanding`).
- When published clinical guidelines or drug labeling materially sharpen a visit decision, search and open passages with `search_clinical_library` and `read_clinical_library` using clinical concepts only — never include patient names, birth dates, medical record numbers, or identifiers.
- Keep consequential uncertainty (`uncertainty`) directly beside the claim or visit decision it qualifies whenever records are missing, conflicting, unreadable, or stale.
- Frame every item in `visit_decisions` strictly as an open clinical question or management option for the physician to consider at the visit — never present a prepared recommendation as accepted care, a placed order, or a completed treatment change.
- Write `visit_card_orientation` as a concise one-to-two sentence orientation for the schedule card that agrees with the briefing's core trajectory and visit focus.
- Most visits need only their saved briefing. Include `visit_concern` only when preparation uncovers news the physician should see before that visit (`today` for important pre-visit news ahead of the encounter, `now` for urgent findings demanding immediate physician attention).

Call `save_visit_briefing` once you have verified the supporting chart revisions, then finish with a brief clinical summary. Preloaded assignment skills are already open below; do not reload them via `skill_view`. Communicate completed clinical work in physician language, never runtime terminology, model names, schemas, or internal identifiers.";

/// Concise preview of one retained chart record supplied in selection context.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateRecordPreview {
    /// Opaque candidate identifier in the patient's workspace.
    pub candidate_id: String,
    /// Current immutable revision identifier.
    pub revision: String,
    /// Source system holding the record.
    pub source_id: String,
    /// Source's record type label.
    pub record_type: String,
    /// Recorded dates on this revision.
    #[serde(default)]
    pub dates: Vec<RecordDate>,
    /// Bounded readable excerpt from the record.
    pub excerpt: String,
}

/// Summary of an earlier preparation decision recorded for a candidate visit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriorVisitDecisionSummary {
    /// Decision kind previously saved (`prepare_now`, `schedule`, `defer`, or `skip`).
    pub action: String,
    /// Scheduled preparation or re-evaluation Unix timestamp, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_at_unix: Option<i64>,
    /// Clinical reason retained with the earlier decision.
    pub reason: String,
    /// Workspace chart version observed when the earlier decision was saved.
    pub observed_source_version: u64,
}

/// Appointment and patient evidence brief supplied for one candidate visit during
/// a scheduled selection run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateVisitBrief {
    /// Schedule source identifier.
    pub source_id: String,
    /// Stable source appointment identifier.
    pub appointment_id: String,
    /// Saved schedule appointment version (`>= 1`).
    pub appointment_saved_version: u64,
    /// Visit start timestamp in RFC 3339 format.
    pub starts_at: String,
    /// Visit start timestamp in Unix epoch seconds.
    pub starts_at_unix: i64,
    /// Visit end timestamp in RFC 3339 format.
    pub ends_at: String,
    /// Visit end timestamp in Unix epoch seconds.
    pub ends_at_unix: i64,
    /// Appointment reason/type label from the schedule source.
    pub label: String,
    /// Raw status reported by the schedule source.
    pub source_status: String,
    /// Matched Coda patient workspace identifier.
    pub workspace: String,
    /// Patient demographics from the latest recorded source pull, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patient: Option<SourcePatient>,
    /// Current retained chart version for `workspace`.
    pub chart_version: u64,
    /// Total current retained chart records in `workspace`.
    pub chart_record_count: usize,
    /// Per-source pull timestamps recorded for `workspace`.
    #[serde(default)]
    pub source_pulls: Vec<SourceFreshnessSummary>,
    /// Dispatcher source-check standing for `workspace`.
    pub check_standing: DispatchCheckStanding,
    /// Saved patient summary claims when an understanding exists.
    #[serde(default)]
    pub summary_claims: Vec<String>,
    /// Saved material gaps from patient understanding.
    #[serde(default)]
    pub material_gaps: Vec<String>,
    /// Active durable patient memory statements (such as physician corrections).
    #[serde(default)]
    pub active_memories: Vec<String>,
    /// Currently open concerns for `workspace`.
    #[serde(default)]
    pub open_concerns: Vec<OpenConcernSummary>,
    /// Bounded previews of recent retained chart records in `workspace`.
    #[serde(default)]
    pub recent_records: Vec<CandidateRecordPreview>,
    /// Prior decision recorded for an earlier chart/appointment version or deferred check.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_decision: Option<PriorVisitDecisionSummary>,
}

/// Typed starting context embedded in [`Assignment`] for a scheduled visit-selection job.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VisitSelectionContext {
    /// Practice identifier owning the schedule.
    pub practice: String,
    /// Practice-clock Unix second when the scheduled wake-up evaluated the schedule.
    pub evaluated_at_unix: i64,
    /// Practice-clock RFC 3339 timestamp when the scheduled wake-up ran.
    pub evaluated_at: String,
    /// Standing and freshness of the practice schedule source.
    pub schedule_standing: ScheduleStanding,
    /// Candidate upcoming or in-progress matched visits requiring a selection decision.
    pub candidates: Vec<CandidateVisitBrief>,
}

/// Per-source refresh standing supplied to a visit briefing run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRefreshStatusSummary {
    /// Connected source identifier.
    pub source_id: String,
    /// Human-readable source label.
    pub source_label: String,
    /// Last successful pull timestamp in Unix seconds, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_good_pulled_at: Option<i64>,
    /// Latest failed refresh timestamp in Unix seconds, when the latest refresh failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_failed_at: Option<i64>,
    /// Failure message when the latest refresh attempt could not update this source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}

/// Summary of a dated historical briefing retained from an earlier scheduled slot.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoricalBriefingSummary {
    /// Stable briefing identifier.
    pub briefing_id: String,
    /// Original visit start timestamp (RFC 3339) that the historical briefing belonged to.
    pub starts_at: String,
    /// Unix second when the historical briefing was prepared.
    pub prepared_at: i64,
    /// Visit-card orientation from that earlier briefing.
    pub visit_card_orientation: String,
    /// Why that earlier slot became historical (`moved` or `cancelled`).
    pub standing: String,
}

/// Typed starting context embedded in [`Assignment`] for a single-visit briefing job.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VisitBriefingContext {
    /// Practice identifier owning the schedule and patient link.
    pub practice: String,
    /// Matched patient workspace identifier.
    pub workspace: String,
    /// Schedule source identifier.
    pub source_id: String,
    /// Stable source appointment identifier.
    pub appointment_id: String,
    /// Saved version of the appointment targeted by this preparation run.
    pub appointment_saved_version: u64,
    /// Target visit start timestamp in RFC 3339 format.
    pub starts_at: String,
    /// Target visit start timestamp in Unix seconds.
    pub starts_at_unix: i64,
    /// Target visit end timestamp in RFC 3339 format.
    pub ends_at: String,
    /// Target visit end timestamp in Unix seconds.
    pub ends_at_unix: i64,
    /// Appointment type/description label.
    pub label: String,
    /// Raw schedule status string.
    pub source_status: String,
    /// Clinical reason retained when Coda selected this visit for preparation.
    pub selection_reason: String,
    /// Workspace chart observation version when this briefing run was queued.
    pub target_source_version: u64,
    /// Practice-clock Unix second when this briefing run was queued, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queued_at_unix: Option<i64>,
    /// Bounded patient starting context (demographics, understanding, memories, open concerns, recent work).
    pub starting_context: PatientStartingContext,
    /// Per-source freshness and failed-refresh status records.
    #[serde(default)]
    pub source_statuses: Vec<SourceRefreshStatusSummary>,
    /// Earlier dated briefings retained if this appointment was rescheduled.
    #[serde(default)]
    pub prior_visit_briefings: Vec<HistoricalBriefingSummary>,
}

/// Validated model decision for when and whether to prepare one scheduled visit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum VisitPreparationDecision {
    /// Start preparing the visit briefing immediately on the current wake-up.
    PrepareNow {
        /// Grounded clinical reason for preparing now.
        reason: String,
    },
    /// Prepare the visit briefing at `prepare_at_unix` ahead of the encounter.
    Schedule {
        /// Target Unix epoch second to start preparation (`< ends_at_unix`).
        prepare_at_unix: i64,
        /// Grounded clinical reason for choosing this preparation time.
        reason: String,
    },
    /// Wait for pending evidence and re-evaluate the visit at `check_again_at_unix`.
    Defer {
        /// Target Unix epoch second to re-evaluate (`< ends_at_unix`).
        check_again_at_unix: i64,
        /// Grounded clinical reason for deferring preparation.
        reason: String,
    },
    /// Do not prepare a briefing for this visit unless the chart or appointment changes.
    Skip {
        /// Grounded clinical reason why no pre-visit briefing is warranted.
        reason: String,
    },
}

impl VisitPreparationDecision {
    /// Canonical snake_case action label.
    pub const fn action_str(&self) -> &'static str {
        match self {
            Self::PrepareNow { .. } => "prepare_now",
            Self::Schedule { .. } => "schedule",
            Self::Defer { .. } => "defer",
            Self::Skip { .. } => "skip",
        }
    }

    /// Clinical reason retained with this decision.
    pub fn reason(&self) -> &str {
        match self {
            Self::PrepareNow { reason }
            | Self::Schedule { reason, .. }
            | Self::Defer { reason, .. }
            | Self::Skip { reason } => reason,
        }
    }
}

/// One validated visit selection entry submitted to [`SELECT_VISITS_TOOL`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectedVisitItem {
    /// Schedule source identifier.
    pub source_id: String,
    /// Stable appointment identifier.
    pub appointment_id: String,
    /// Exact visit start time (`starts_at_unix`) this decision applies to.
    pub starts_at_unix: i64,
    /// Typed preparation timing or skip decision with its clinical reason.
    pub decision: VisitPreparationDecision,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSelectedVisitItem {
    source_id: String,
    appointment_id: String,
    starts_at_unix: i64,
    action: String,
    #[serde(default)]
    prepare_at_unix: Option<i64>,
    #[serde(default)]
    check_again_at_unix: Option<i64>,
    reason: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSelectVisitsArgs {
    decisions: Vec<RawSelectedVisitItem>,
}

/// Validated arguments to [`SELECT_VISITS_TOOL`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectVisitsArgs {
    /// Non-empty list of per-visit preparation decisions without duplicate visit slots.
    pub decisions: Vec<SelectedVisitItem>,
}

/// One cited clinical statement in a visit briefing (`trajectory`,
/// `meaningful_changes`, or `unresolved_issues`), keeping any consequential
/// uncertainty directly beside the conclusion it qualifies.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BriefingClaimInput {
    /// Physician-facing clinical statement.
    pub text: String,
    /// Consequential uncertainty qualifying `text` (kept beside the conclusion).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncertainty: Option<String>,
    /// Exact chart record revisions supporting `text`.
    #[serde(default)]
    pub citations: Vec<ExactRecordReference>,
}

/// One open decision or clinical question prepared for the physician to settle
/// during the visit — never presented as accepted care or an executed order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BriefingDecisionInput {
    /// Open clinical decision or question the upcoming visit can address.
    pub decision: String,
    /// Why this decision matters now given the patient's course and interval changes.
    pub clinical_rationale: String,
    /// Concrete options, trade-offs, or questions for the physician to consider.
    pub options_or_questions: Vec<String>,
    /// Consequential uncertainty qualifying this decision or its options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncertainty: Option<String>,
    /// Exact chart record revisions grounding this decision prompt.
    #[serde(default)]
    pub citations: Vec<ExactRecordReference>,
}

/// Optional pre-visit concern requested alongside [`SaveBriefingArgs`] when
/// preparation finds news the physician should see before the encounter.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VisitConcernInput {
    /// Stable concern identifier within `(practice, workspace)`.
    pub concern_id: ConcernId,
    /// Urgency tier (`Today` for pre-visit awareness, `Now` for immediate attention).
    pub urgency: ConcernUrgency,
    /// Clinical explanation of why the physician should see this before the visit.
    pub reason: String,
    /// Verified exact chart record revisions supporting the concern (`1..=16`).
    pub records: Vec<ExactRecordReference>,
}

/// Validated arguments to [`SAVE_BRIEFING_TOOL`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveBriefingArgs {
    /// Concise 1–2 sentence visit-card orientation consistent with the briefing.
    pub visit_card_orientation: String,
    /// Longitudinal clinical trajectory statements and their citations.
    pub trajectory: Vec<BriefingClaimInput>,
    /// Meaningful interval changes since prior care and their citations.
    pub meaningful_changes: Vec<BriefingClaimInput>,
    /// Unresolved clinical issues, pending tests, or open follow-up threads.
    pub unresolved_issues: Vec<BriefingClaimInput>,
    /// Useful decisions and options for the physician to consider at the visit.
    pub visit_decisions: Vec<BriefingDecisionInput>,
    /// Physician-readable summary of dated evidence, source freshness, and material limits.
    pub basis_and_limitations: String,
    /// Optional exact record citations referenced in `basis_and_limitations`.
    #[serde(default)]
    pub basis_citations: Vec<ExactRecordReference>,
    /// Explicit missing, unreadable, or unavailable evidence limits noted by the run.
    #[serde(default)]
    pub limits: Vec<RecordLimit>,
    /// Optional pre-visit concern raised only when news warrants attention before the visit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visit_concern: Option<VisitConcernInput>,
}

/// Boundary validation failure on [`SelectVisitsArgs`] or [`SaveBriefingArgs`].
#[derive(Debug, thiserror::Error)]
pub enum VisitPreparationInputError {
    /// Arguments JSON failed schema decoding.
    #[error("invalid {tool} arguments: {source}")]
    MalformedJson {
        /// Tool name whose JSON payload failed deserialization.
        tool: &'static str,
        /// Underlying JSON decoding error.
        #[source]
        source: serde_json::Error,
    },
    /// Required text field was empty or whitespace.
    #[error("{field} must not be empty")]
    EmptyField {
        /// Name of the blank field.
        field: &'static str,
    },
    /// Selection call contained zero decisions.
    #[error("decisions must contain at least one candidate visit selection")]
    EmptyDecisions,
    /// Selection call listed the same `(source_id, appointment_id, starts_at_unix)` twice.
    #[error("duplicate selection decision for appointment `{appointment_id}` at {starts_at_unix}")]
    DuplicateVisitSlot {
        /// Duplicated appointment identifier.
        appointment_id: String,
        /// Duplicated visit start Unix timestamp.
        starts_at_unix: i64,
    },
    /// Unknown selection `action` string or mismatched timestamp fields.
    #[error("invalid selection action `{action}`: {detail}")]
    InvalidSelectionAction {
        /// Supplied action string.
        action: String,
        /// Corrective explanation.
        detail: &'static str,
    },
    /// Required briefing section had zero items.
    #[error("{section} must contain at least one item")]
    EmptySection {
        /// Name of the empty briefing section.
        section: &'static str,
    },
    /// Exact record reference failed validation.
    #[error("invalid {field}: {cause}")]
    InvalidReference {
        /// Field containing the invalid reference.
        field: &'static str,
        /// Underlying reference validation cause.
        #[source]
        cause: ConcernInputError,
    },
    /// Pre-visit concern input failed concern domain validation.
    #[error("invalid visit_concern: {0}")]
    InvalidConcern(#[source] ConcernInputError),
    /// A prepared visit decision presented a recommendation as already accepted or ordered care.
    #[error(
        "visit decision `{phrase}` presents a recommendation as accepted or executed care; frame recommendations as options or questions for the physician"
    )]
    RecommendationPresentedAsAcceptedCare {
        /// Phrase that violated the physician-authority boundary.
        phrase: &'static str,
    },
}

fn require_trimmed(field: &'static str, raw: &str) -> Result<String, VisitPreparationInputError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(VisitPreparationInputError::EmptyField { field });
    }
    Ok(trimmed.to_owned())
}

fn validate_references(
    field: &'static str,
    refs: Vec<ExactRecordReference>,
) -> Result<Vec<ExactRecordReference>, VisitPreparationInputError> {
    let mut validated: Vec<ExactRecordReference> = Vec::with_capacity(refs.len());
    for item in refs {
        let exact = ExactRecordReference::new(&item.candidate_id, &item.revision)
            .map_err(|cause| VisitPreparationInputError::InvalidReference { field, cause })?;
        if let Some(existing) = validated
            .iter()
            .find(|prior| prior.candidate_id == exact.candidate_id)
        {
            if existing.revision != exact.revision {
                return Err(VisitPreparationInputError::InvalidReference {
                    field,
                    cause: ConcernInputError::ConflictingRecordRevision {
                        candidate_id: exact.candidate_id,
                    },
                });
            }
            continue;
        }
        validated.push(exact);
    }
    Ok(validated)
}

fn validate_claims(
    section: &'static str,
    raw_claims: Vec<BriefingClaimInput>,
) -> Result<Vec<BriefingClaimInput>, VisitPreparationInputError> {
    if raw_claims.is_empty() {
        return Err(VisitPreparationInputError::EmptySection { section });
    }
    let mut out = Vec::with_capacity(raw_claims.len());
    for claim in raw_claims {
        let text = require_trimmed(section, &claim.text)?;
        let uncertainty = claim
            .uncertainty
            .map(|u| require_trimmed("uncertainty", &u))
            .transpose()?;
        let citations = validate_references(section, claim.citations)?;
        out.push(BriefingClaimInput {
            text,
            uncertainty,
            citations,
        });
    }
    Ok(out)
}

/// Checks that prepared recommendations in `visit_decisions` or `visit_card_orientation`
/// do not claim Coda has already placed an order, sent a prescription, or finalized care.
fn detect_accepted_care_phrasing(text: &str) -> Option<&'static str> {
    let lower = text.to_ascii_lowercase();
    [
        "order has been placed",
        "orders have been placed",
        "order was placed",
        "orders were placed",
        "has been ordered",
        "have been ordered",
        "has been prescribed",
        "have been prescribed",
        "prescription has been sent",
        "prescription sent to pharmacy",
        "coda ordered",
        "coda prescribed",
        "coda initiated",
        "plan approved and active",
        "accepted care plan:",
        "i have ordered",
        "i have prescribed",
    ]
    .into_iter()
    .find(|&banned| lower.contains(banned))
}

impl SelectVisitsArgs {
    /// Parses and validates raw tool JSON into [`SelectVisitsArgs`].
    pub fn parse(arguments: &Value) -> Result<Self, VisitPreparationInputError> {
        let raw: RawSelectVisitsArgs =
            serde_json::from_value(arguments.clone()).map_err(|source| {
                VisitPreparationInputError::MalformedJson {
                    tool: SELECT_VISITS_TOOL,
                    source,
                }
            })?;
        if raw.decisions.is_empty() {
            return Err(VisitPreparationInputError::EmptyDecisions);
        }
        let mut seen = std::collections::BTreeSet::new();
        let mut decisions = Vec::with_capacity(raw.decisions.len());
        for item in raw.decisions {
            let source_id = require_trimmed("decisions.source_id", &item.source_id)?;
            let appointment_id = require_trimmed("decisions.appointment_id", &item.appointment_id)?;
            let reason = require_trimmed("decisions.reason", &item.reason)?;
            if !seen.insert((
                source_id.clone(),
                appointment_id.clone(),
                item.starts_at_unix,
            )) {
                return Err(VisitPreparationInputError::DuplicateVisitSlot {
                    appointment_id,
                    starts_at_unix: item.starts_at_unix,
                });
            }
            let decision = match item.action.trim() {
                "prepare_now" => {
                    if item.prepare_at_unix.is_some() || item.check_again_at_unix.is_some() {
                        return Err(VisitPreparationInputError::InvalidSelectionAction {
                            action: item.action,
                            detail: "prepare_now must not set prepare_at_unix or check_again_at_unix",
                        });
                    }
                    VisitPreparationDecision::PrepareNow { reason }
                }
                "schedule" => {
                    if item.check_again_at_unix.is_some() {
                        return Err(VisitPreparationInputError::InvalidSelectionAction {
                            action: item.action,
                            detail: "schedule uses prepare_at_unix, not check_again_at_unix",
                        });
                    }
                    let Some(prepare_at_unix) = item.prepare_at_unix else {
                        return Err(VisitPreparationInputError::InvalidSelectionAction {
                            action: item.action,
                            detail: "schedule requires prepare_at_unix",
                        });
                    };
                    VisitPreparationDecision::Schedule {
                        prepare_at_unix,
                        reason,
                    }
                }
                "defer" => {
                    if item.prepare_at_unix.is_some() {
                        return Err(VisitPreparationInputError::InvalidSelectionAction {
                            action: item.action,
                            detail: "defer uses check_again_at_unix, not prepare_at_unix",
                        });
                    }
                    let Some(check_again_at_unix) = item.check_again_at_unix else {
                        return Err(VisitPreparationInputError::InvalidSelectionAction {
                            action: item.action,
                            detail: "defer requires check_again_at_unix",
                        });
                    };
                    VisitPreparationDecision::Defer {
                        check_again_at_unix,
                        reason,
                    }
                }
                "skip" => {
                    if item.prepare_at_unix.is_some() || item.check_again_at_unix.is_some() {
                        return Err(VisitPreparationInputError::InvalidSelectionAction {
                            action: item.action,
                            detail: "skip must not set prepare_at_unix or check_again_at_unix",
                        });
                    }
                    VisitPreparationDecision::Skip { reason }
                }
                other => {
                    return Err(VisitPreparationInputError::InvalidSelectionAction {
                        action: other.to_owned(),
                        detail: "expected one of `prepare_now`, `schedule`, `defer`, or `skip`",
                    });
                }
            };
            decisions.push(SelectedVisitItem {
                source_id,
                appointment_id,
                starts_at_unix: item.starts_at_unix,
                decision,
            });
        }
        Ok(Self { decisions })
    }
}

impl SaveBriefingArgs {
    /// Parses and validates raw tool JSON into [`SaveBriefingArgs`].
    pub fn parse(arguments: &Value) -> Result<Self, VisitPreparationInputError> {
        let raw: Self = serde_json::from_value(arguments.clone()).map_err(|source| {
            VisitPreparationInputError::MalformedJson {
                tool: SAVE_BRIEFING_TOOL,
                source,
            }
        })?;
        raw.validate()
    }

    /// Trims and validates every nested section, citation reference, decision
    /// boundary, limit entry, and optional pre-visit concern.
    pub fn validate(self) -> Result<Self, VisitPreparationInputError> {
        let visit_card_orientation =
            require_trimmed("visit_card_orientation", &self.visit_card_orientation)?;
        if let Some(phrase) = detect_accepted_care_phrasing(&visit_card_orientation) {
            return Err(
                VisitPreparationInputError::RecommendationPresentedAsAcceptedCare { phrase },
            );
        }

        let trajectory = validate_claims("trajectory", self.trajectory)?;
        let meaningful_changes = validate_claims("meaningful_changes", self.meaningful_changes)?;
        let unresolved_issues = validate_claims("unresolved_issues", self.unresolved_issues)?;

        if self.visit_decisions.is_empty() {
            return Err(VisitPreparationInputError::EmptySection {
                section: "visit_decisions",
            });
        }
        let mut visit_decisions = Vec::with_capacity(self.visit_decisions.len());
        for item in self.visit_decisions {
            let decision = require_trimmed("visit_decisions.decision", &item.decision)?;
            let clinical_rationale = require_trimmed(
                "visit_decisions.clinical_rationale",
                &item.clinical_rationale,
            )?;
            if item.options_or_questions.is_empty() {
                return Err(VisitPreparationInputError::EmptySection {
                    section: "visit_decisions.options_or_questions",
                });
            }
            let mut options_or_questions = Vec::with_capacity(item.options_or_questions.len());
            for opt in item.options_or_questions {
                let trimmed = require_trimmed("visit_decisions.options_or_questions", &opt)?;
                if let Some(phrase) = detect_accepted_care_phrasing(&trimmed) {
                    return Err(
                        VisitPreparationInputError::RecommendationPresentedAsAcceptedCare {
                            phrase,
                        },
                    );
                }
                options_or_questions.push(trimmed);
            }
            if let Some(phrase) = detect_accepted_care_phrasing(&decision)
                .or_else(|| detect_accepted_care_phrasing(&clinical_rationale))
            {
                return Err(
                    VisitPreparationInputError::RecommendationPresentedAsAcceptedCare { phrase },
                );
            }
            let uncertainty = item
                .uncertainty
                .map(|u| require_trimmed("visit_decisions.uncertainty", &u))
                .transpose()?;
            let citations = validate_references("visit_decisions.citations", item.citations)?;
            visit_decisions.push(BriefingDecisionInput {
                decision,
                clinical_rationale,
                options_or_questions,
                uncertainty,
                citations,
            });
        }

        let basis_and_limitations =
            require_trimmed("basis_and_limitations", &self.basis_and_limitations)?;
        let basis_citations = validate_references("basis_citations", self.basis_citations)?;

        let mut limits = Vec::with_capacity(self.limits.len());
        for limit in self.limits {
            limits.push(RecordLimit {
                kind: limit.kind,
                subject: require_trimmed("limits.subject", &limit.subject)?,
                detail: require_trimmed("limits.detail", &limit.detail)?,
            });
        }

        let visit_concern = match self.visit_concern {
            None => None,
            Some(concern) => {
                let reason = require_trimmed("visit_concern.reason", &concern.reason)?;
                if concern.records.is_empty()
                    || concern.records.len() > crate::runtime::concerns::MAX_CONCERN_RECORDS
                {
                    return Err(VisitPreparationInputError::InvalidConcern(
                        ConcernInputError::InvalidRecordCount,
                    ));
                }
                let records = validate_references("visit_concern.records", concern.records)?;
                Some(VisitConcernInput {
                    concern_id: concern.concern_id,
                    urgency: concern.urgency,
                    reason,
                    records,
                })
            }
        };

        Ok(Self {
            visit_card_orientation,
            trajectory,
            meaningful_changes,
            unresolved_issues,
            visit_decisions,
            basis_and_limitations,
            basis_citations,
            limits,
            visit_concern,
        })
    }
}

/// Whether `name` is one of the visit preparation tools (`select_visit_preparations`
/// or `save_visit_briefing`).
pub fn is_visit_preparation_tool(name: &str) -> bool {
    matches!(name, SELECT_VISITS_TOOL | SAVE_BRIEFING_TOOL)
}

fn exact_ref_schema() -> Value {
    json!({
        "type": "object",
        "additionalProperties": false,
        "required": ["candidate_id", "revision"],
        "properties": {
            "candidate_id": {
                "type": "string",
                "description": "Opaque record identifier returned by search_patient_information or read_patient_sources."
            },
            "revision": {
                "type": "string",
                "description": "Exact immutable record revision verified via read_patient_sources."
            }
        }
    })
}

fn briefing_claim_schema(description: &str) -> Value {
    json!({
        "type": "object",
        "additionalProperties": false,
        "required": ["text", "citations"],
        "properties": {
            "text": {
                "type": "string",
                "description": description
            },
            "uncertainty": {
                "type": "string",
                "description": "Optional consequential uncertainty kept directly beside this statement when missing, conflicting, unreadable, or stale evidence qualifies it."
            },
            "citations": {
                "type": "array",
                "items": exact_ref_schema(),
                "description": "Exact chart record revisions supporting this statement."
            }
        }
    })
}

/// Tool descriptor for [`SELECT_VISITS_TOOL`].
pub fn select_visits_tool() -> Tool {
    Tool {
        name: SELECT_VISITS_TOOL.to_owned(),
        description: "Record which upcoming or in-progress candidate visits to prepare, when to prepare or re-check each one, or why preparation is skipped, retaining the clinical reason for each visit.".to_owned(),
        parameters: json!({
            "type": "object",
            "additionalProperties": false,
            "required": ["decisions"],
            "properties": {
                "decisions": {
                    "type": "array",
                    "minItems": 1,
                    "description": "Selection decisions for candidate visits in the practice schedule.",
                    "items": {
                        "type": "object",
                        "additionalProperties": false,
                        "required": ["source_id", "appointment_id", "starts_at_unix", "action", "reason"],
                        "properties": {
                            "source_id": {
                                "type": "string",
                                "description": "Schedule source identifier from the candidate visit."
                            },
                            "appointment_id": {
                                "type": "string",
                                "description": "Appointment identifier from the candidate visit."
                            },
                            "starts_at_unix": {
                                "type": "integer",
                                "description": "Scheduled start time in Unix seconds for the visit slot."
                            },
                            "action": {
                                "type": "string",
                                "enum": ["prepare_now", "schedule", "defer", "skip"],
                                "description": "Whether to prepare immediately, schedule preparation at prepare_at_unix, defer until check_again_at_unix, or skip preparation."
                            },
                            "prepare_at_unix": {
                                "type": "integer",
                                "description": "Required when action is `schedule`: Unix timestamp before ends_at_unix when briefing preparation should start."
                            },
                            "check_again_at_unix": {
                                "type": "integer",
                                "description": "Required when action is `defer`: Unix timestamp before ends_at_unix when Coda should check again for expected records."
                            },
                            "reason": {
                                "type": "string",
                                "description": "Clinical reason grounded in appointment context and patient evidence."
                            }
                        }
                    }
                }
            }
        }),
    }
}

/// Tool descriptor for [`SAVE_BRIEFING_TOOL`].
pub fn save_briefing_tool() -> Tool {
    Tool {
        name: SAVE_BRIEFING_TOOL.to_owned(),
        description: "Save the cited pre-visit clinical briefing for this appointment's scheduled visit time, including the concise visit-card orientation, trajectory, meaningful changes, unresolved issues, open physician visit decisions, basis and limitations, and an optional pre-visit concern when news warrants physician attention before the encounter.".to_owned(),
        parameters: json!({
            "type": "object",
            "additionalProperties": false,
            "required": [
                "visit_card_orientation",
                "trajectory",
                "meaningful_changes",
                "unresolved_issues",
                "visit_decisions",
                "basis_and_limitations"
            ],
            "properties": {
                "visit_card_orientation": {
                    "type": "string",
                    "description": "Concise one-to-two sentence orientation shown on the visit card, consistent with the full briefing."
                },
                "trajectory": {
                    "type": "array",
                    "minItems": 1,
                    "items": briefing_claim_schema("Longitudinal clinical trajectory statement with dates and direction."),
                    "description": "Patient's longitudinal clinical course across principal conditions and markers."
                },
                "meaningful_changes": {
                    "type": "array",
                    "minItems": 1,
                    "items": briefing_claim_schema("Meaningful interval clinical update or explicit statement of interval stability."),
                    "description": "New results, medication changes, outside encounters, or interval updates since prior care."
                },
                "unresolved_issues": {
                    "type": "array",
                    "minItems": 1,
                    "items": briefing_claim_schema("Unresolved follow-up item, pending test, open thread, or discrepancy."),
                    "description": "Open clinical work, pending studies, overdue monitoring, or unanswered questions."
                },
                "visit_decisions": {
                    "type": "array",
                    "minItems": 1,
                    "description": "Open clinical questions and management options the visit can settle, framed strictly for physician decision rather than accepted care.",
                    "items": {
                        "type": "object",
                        "additionalProperties": false,
                        "required": ["decision", "clinical_rationale", "options_or_questions", "citations"],
                        "properties": {
                            "decision": {
                                "type": "string",
                                "description": "Open clinical question or decision for the physician during the visit."
                            },
                            "clinical_rationale": {
                                "type": "string",
                                "description": "Clinical reasoning connecting patient evidence to this visit decision."
                            },
                            "options_or_questions": {
                                "type": "array",
                                "minItems": 1,
                                "items": { "type": "string" },
                                "description": "Concrete options, trade-offs, or questions for the physician to consider."
                            },
                            "uncertainty": {
                                "type": "string",
                                "description": "Optional consequential uncertainty qualifying this decision or its options."
                            },
                            "citations": {
                                "type": "array",
                                "items": exact_ref_schema(),
                                "description": "Exact chart record revisions grounding this decision."
                            }
                        }
                    }
                },
                "basis_and_limitations": {
                    "type": "string",
                    "description": "Concise physician-readable statement naming dated sources, source freshness, and any missing, conflicting, unreadable, or unavailable evidence."
                },
                "basis_citations": {
                    "type": "array",
                    "items": exact_ref_schema(),
                    "description": "Optional exact chart record revisions referenced in basis_and_limitations."
                },
                "limits": {
                    "type": "array",
                    "description": "Optional structured missing, unreadable, or unavailable evidence limits affecting this briefing.",
                    "items": {
                        "type": "object",
                        "additionalProperties": false,
                        "required": ["kind", "subject", "detail"],
                        "properties": {
                            "kind": {
                                "type": "string",
                                "enum": [
                                    RecordLimitKind::Missing.as_str(),
                                    RecordLimitKind::Unreadable.as_str(),
                                    RecordLimitKind::Unavailable.as_str()
                                ]
                            },
                            "subject": { "type": "string" },
                            "detail": { "type": "string" }
                        }
                    }
                },
                "visit_concern": {
                    "type": "object",
                    "additionalProperties": false,
                    "required": ["concern_id", "urgency", "reason", "records"],
                    "description": "Optional pre-visit concern raised only when preparation uncovers news the physician should see before walking into the visit.",
                    "properties": {
                        "concern_id": {
                            "type": "string",
                            "description": "Stable identifier (1-256 characters) for this pre-visit concern."
                        },
                        "urgency": {
                            "type": "string",
                            "enum": ["today", "now"],
                            "description": "`today` for pre-visit awareness that ends with the visit; `now` for urgent findings that remain open until opened by a physician."
                        },
                        "reason": {
                            "type": "string",
                            "description": "Clinical statement explaining why the physician should see this before the visit."
                        },
                        "records": {
                            "type": "array",
                            "minItems": 1,
                            "maxItems": 16,
                            "items": exact_ref_schema(),
                            "description": "Verified exact chart record revisions supporting the pre-visit concern."
                        }
                    }
                }
            }
        }),
    }
}

///Read-only patient memory descriptor offered to background visit-briefing runs
/// so Coda can inspect durable physician corrections without mutating memory.
pub fn read_only_memory_tool() -> Tool {
    Tool {
        name: crate::runtime::chat_operations::PATIENT_MEMORY_TOOL.to_owned(),
        description: "Read durable patient memory entries (including physician-stated corrections) for this patient workspace.".to_owned(),
        parameters: json!({
            "type": "object",
            "additionalProperties": false,
            "required": ["operation"],
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["read"],
                    "description": "Background visit preparation may only read durable patient memory."
                },
                "include_retired": {
                    "type": "boolean",
                    "description": "When true, include retired memory entries alongside active ones."
                }
            }
        }),
    }
}

/// Read-only prior background work descriptor offered to visit-briefing runs.
pub fn background_retrieve_work_tool() -> Tool {
    Tool {
        name: crate::runtime::chat_operations::RETRIEVE_PATIENT_WORK_TOOL.to_owned(),
        description: "List or read completed practice background jobs and earlier saved briefings for this patient workspace.".to_owned(),
        parameters: json!({
            "type": "object",
            "additionalProperties": false,
            "required": ["operation"],
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["list", "read_background_job"],
                    "description": "List recent practice background work or read one completed background job."
                },
                "limit": {
                    "type": "integer",
                    "minimum": 1,
                    "maximum": 25,
                    "description": "Maximum items to return when listing."
                },
                "job_id": {
                    "type": "string",
                    "description": "Background job identifier when operation is `read_background_job`."
                }
            }
        }),
    }
}

/// Tools offered to a scheduled visit-selection job.
pub fn selection_tools() -> Vec<Tool> {
    let mut offered = crate::physician_chat::work_tools();
    offered.push(select_visits_tool());
    offered
}

/// Tools offered to a per-visit briefing preparation job.
pub fn briefing_tools() -> Vec<Tool> {
    let mut offered = crate::physician_chat::work_tools();
    offered.push(crate::physician_sources::search_tool());
    offered.push(crate::physician_sources::read_tool(ChartScope::Chat));
    offered.push(crate::attachments::document_page_tool());
    offered.push(crate::runtime::refresh::refresh_tool());
    offered.push(
        crate::initial_understanding::understanding_tools()
            .into_iter()
            .find(|tool| tool.name == crate::initial_understanding::READ_UNDERSTANDING_TOOL)
            .expect("read_patient_understanding tool exists"),
    );
    offered.push(read_only_memory_tool());
    offered.push(background_retrieve_work_tool());
    offered.extend(crate::clinical_library::tools());
    offered.push(save_briefing_tool());
    offered
}

/// Union of visit-preparation-specific tools for catalog tool-surface verification.
pub fn all_tools() -> Vec<Tool> {
    vec![select_visits_tool(), save_briefing_tool()]
}

fn render_instructions(
    base: &str,
    preloaded_skills: &[PreloadedSkill],
    skills: &PhysicianSkills,
) -> String {
    let mut rendered = skills.instructions(base);
    if !preloaded_skills.is_empty() {
        let preloaded: Vec<Value> = preloaded_skills
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
        rendered.push_str(
            "\n\nPreloaded assignment skills (already open in this run; do not reload via skill_view):\n",
        );
        rendered.push_str(&encoded);
    }
    rendered
}

/// Builds the [`RequestRecipe`] for a `PracticeJobKind::VisitPreparation` assignment
/// on `background` (`ModelRoles.background`).
pub fn recipe(
    background: &ModelBinding,
    assignment: &Assignment,
    skills: &PhysicianSkills,
) -> RequestRecipe {
    let (base_instructions, tool_list) = if assignment.visit_selection.is_some() {
        (SELECTION_INSTRUCTIONS, selection_tools())
    } else {
        (BRIEFING_INSTRUCTIONS, briefing_tools())
    };
    let mut built = RequestRecipe {
        model: background.model.clone(),
        context_window_tokens: background.context_window_tokens,
        max_input_tokens: background.max_input_tokens,
        options: json!({ "effort": background.effort }),
        instructions: render_instructions(base_instructions, &assignment.preloaded_skills, skills),
        tools: tool_list.into_iter().map(|tool| tool.function()).collect(),
    };
    harness::engine::offer_working_notes(&mut built.instructions, &mut built.tools);
    built
}

/// Deterministic command/job key for a visit-selection evaluation batch.
pub fn selection_job_key(practice: &str, fingerprint: &str) -> String {
    format!("visit-selection:{practice}:{fingerprint}")
}

/// Deterministic practice-job identifier for preparing one visit slot.
pub fn briefing_job_id(source_id: &str, appointment_id: &str, starts_at_unix: i64) -> String {
    let raw = format!("{source_id}:{appointment_id}:{starts_at_unix}");
    let digest = &crate::sha256_hex(raw.as_bytes())[..16];
    format!("visit-briefing-{appointment_id}-{starts_at_unix}-{digest}")
}

/// Deterministic command key for preparing one visit slot at a specific
/// appointment and chart version.
pub fn briefing_command_key(
    source_id: &str,
    appointment_id: &str,
    starts_at_unix: i64,
    appointment_saved_version: u64,
    target_source_version: u64,
) -> String {
    format!(
        "visit-briefing:{source_id}:{appointment_id}:{starts_at_unix}:av{appointment_saved_version}:sv{target_source_version}"
    )
}

/// Constructs a typed [`Assignment`] for a scheduled visit-selection run,
/// preloading `visit-preparation` guidance.
pub fn selection_assignment(
    key: impl Into<String>,
    context: VisitSelectionContext,
    skills: &PhysicianSkills,
) -> Result<Assignment, AssignmentError> {
    if context.practice.trim().is_empty() || context.candidates.is_empty() {
        return Err(AssignmentError::EmptyAssignment);
    }
    let context_json =
        serde_json::to_string(&context).expect("VisitSelectionContext serializes cleanly as JSON");
    let prompt = format!(
        "Evaluate the {} upcoming or in-progress matched visit(s) for practice {} at {} (unix {}) \
         and record a preparation selection decision and clinical reason for every candidate visit \
         using `select_visit_preparations`.\n\
         Scheduled visit selection context:\n{context_json}",
        context.candidates.len(),
        context.practice,
        context.evaluated_at,
        context.evaluated_at_unix,
    );
    let mut created = Assignment::new(
        PracticeJobKind::VisitPreparation,
        key,
        prompt,
        &[VISIT_PREPARATION_SKILL],
        skills,
    )?;
    created.visit_selection = Some(context);
    Ok(created)
}

/// Constructs a typed [`Assignment`] for preparing one visit's clinical briefing,
/// preloading `visit-preparation` and `patient-record-retrieval` guidance.
pub fn briefing_assignment(
    context: VisitBriefingContext,
    skills: &PhysicianSkills,
) -> Result<Assignment, AssignmentError> {
    if context.practice.trim().is_empty()
        || context.workspace.trim().is_empty()
        || context.source_id.trim().is_empty()
        || context.appointment_id.trim().is_empty()
    {
        return Err(AssignmentError::EmptyAssignment);
    }
    let key = briefing_command_key(
        &context.source_id,
        &context.appointment_id,
        context.starts_at_unix,
        context.appointment_saved_version,
        context.target_source_version,
    );
    let developer_context = context.starting_context.developer_text();
    let context_json = serde_json::to_string(&json!({
        "practice": &context.practice,
        "workspace": &context.workspace,
        "source_id": &context.source_id,
        "appointment_id": &context.appointment_id,
        "appointment_saved_version": context.appointment_saved_version,
        "starts_at": &context.starts_at,
        "starts_at_unix": context.starts_at_unix,
        "ends_at": &context.ends_at,
        "ends_at_unix": context.ends_at_unix,
        "label": &context.label,
        "source_status": &context.source_status,
        "selection_reason": &context.selection_reason,
        "target_source_version": context.target_source_version,
        "source_statuses": &context.source_statuses,
        "prior_visit_briefings": &context.prior_visit_briefings,
    }))
    .expect("VisitBriefingContext metadata serializes cleanly as JSON");
    let prompt = format!(
        "Prepare the pre-visit clinical briefing for appointment `{}` ({}) starting at {} (ends {}) \
         for patient workspace `{}`.\n\
         Selection reason: {}\n\
         Read the relevant chart revisions with `search_patient_information` and `read_patient_sources`, \
         refresh stale sources if warranted, and save the briefing through `save_visit_briefing`.\n\n\
         Appointment and source freshness context:\n{context_json}\n\n\
         {developer_context}",
        context.appointment_id,
        context.label,
        context.starts_at,
        context.ends_at,
        context.workspace,
        context.selection_reason,
    );
    let mut created = Assignment::new(
        PracticeJobKind::VisitPreparation,
        key,
        prompt,
        &[VISIT_PREPARATION_SKILL, RECORD_RETRIEVAL_SKILL],
        skills,
    )?;
    created.visit_briefing = Some(context);
    Ok(created)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::physician_chat::model_text::{assert_clean_model_text, assert_skill_tools};
    use crate::runtime::chat_operations::PatientMemoryStanding;
    use crate::runtime::jobs::is_physician_only_tool;

    fn load_skills() -> PhysicianSkills {
        PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
            .expect("load physician skills catalog")
    }

    fn background_binding() -> ModelBinding {
        ModelBinding {
            model: "background-visit-prep-model".into(),
            effort: "high".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 250_000,
            adapter: "fixture".into(),
        }
    }

    #[test]
    fn selection_and_briefing_recipes_use_background_role_and_clean_model_text() {
        let skills = load_skills();
        let binding = background_binding();

        let selection = selection_assignment(
            "visit-selection:practice-a:batch-1",
            VisitSelectionContext {
                practice: "practice-a".into(),
                evaluated_at_unix: 1_787_800_000,
                evaluated_at: "2026-08-27T04:26:40Z".into(),
                schedule_standing: ScheduleStanding::Absent,
                candidates: vec![CandidateVisitBrief {
                    source_id: "clinic-cal".into(),
                    appointment_id: "appt-1".into(),
                    appointment_saved_version: 1,
                    starts_at: "2026-08-27T17:00:00Z".into(),
                    starts_at_unix: 1_787_845_200,
                    ends_at: "2026-08-27T17:30:00Z".into(),
                    ends_at_unix: 1_787_847_000,
                    label: "Post-discharge heart failure follow-up".into(),
                    source_status: "booked".into(),
                    workspace: "pw-1".into(),
                    patient: None,
                    chart_version: 2,
                    chart_record_count: 4,
                    source_pulls: Vec::new(),
                    check_standing: DispatchCheckStanding::NotRecorded,
                    summary_claims: vec![
                        "Recent admission for acute decompensated heart failure.".into(),
                    ],
                    material_gaps: Vec::new(),
                    active_memories: Vec::new(),
                    open_concerns: Vec::new(),
                    recent_records: Vec::new(),
                    prior_decision: None,
                }],
            },
            &skills,
        )
        .expect("selection assignment");
        assert_eq!(selection.preloaded_skills.len(), 1);
        assert_eq!(selection.preloaded_skills[0].name, VISIT_PREPARATION_SKILL);

        let selection_recipe = recipe(&binding, &selection, &skills);
        assert_eq!(selection_recipe.model, "background-visit-prep-model");
        assert_clean_model_text(&selection_recipe.instructions);
        assert_clean_model_text(&serde_json::to_string(&selection_recipe.tools).unwrap());
        assert_clean_model_text(&selection.prompt);

        let briefing = briefing_assignment(
            VisitBriefingContext {
                practice: "practice-a".into(),
                workspace: "pw-1".into(),
                source_id: "clinic-cal".into(),
                appointment_id: "appt-1".into(),
                appointment_saved_version: 1,
                starts_at: "2026-08-27T17:00:00Z".into(),
                starts_at_unix: 1_787_845_200,
                ends_at: "2026-08-27T17:30:00Z".into(),
                ends_at_unix: 1_787_847_000,
                label: "Post-discharge heart failure follow-up".into(),
                source_status: "booked".into(),
                selection_reason:
                    "Recent heart failure discharge requires pre-visit medication and renal review."
                        .into(),
                target_source_version: 2,
                queued_at_unix: Some(1_787_800_000),
                starting_context: PatientStartingContext {
                    workspace: "pw-1".into(),
                    patient: None,
                    chart_version: 2,
                    chart_record_count: 4,
                    source_pulls: Vec::new(),
                    check_standing: DispatchCheckStanding::NotRecorded,
                    understanding: None,
                    withdrawal_boundary: None,
                    memory_standing: PatientMemoryStanding::Available {
                        entries: Vec::new(),
                    },
                    open_concerns: Vec::new(),
                    recent_work: Vec::new(),
                },
                source_statuses: Vec::new(),
                prior_visit_briefings: Vec::new(),
            },
            &skills,
        )
        .expect("briefing assignment");
        assert_eq!(briefing.preloaded_skills.len(), 2);

        let briefing_recipe = recipe(&binding, &briefing, &skills);
        assert_eq!(briefing_recipe.model, "background-visit-prep-model");
        assert_clean_model_text(&briefing_recipe.instructions);
        assert_clean_model_text(&serde_json::to_string(&briefing_recipe.tools).unwrap());
        assert_clean_model_text(&briefing.prompt);
        assert_skill_tools(
            &serde_json::to_value(&skills).unwrap(),
            &briefing_recipe.tools,
            &[],
        );

        let mut names = Vec::new();
        for tool in &briefing_recipe.tools {
            let name = tool["name"].as_str().expect("tool name");
            assert!(
                !is_physician_only_tool(name),
                "briefing recipe must not offer physician-only tool {name}"
            );
            names.push(name);
        }
        for required in [
            SAVE_BRIEFING_TOOL,
            crate::physician_sources::SEARCH_TOOL,
            crate::physician_sources::READ_TOOL,
            crate::runtime::refresh::REFRESH_TOOL,
            crate::initial_understanding::READ_UNDERSTANDING_TOOL,
            crate::runtime::chat_operations::PATIENT_MEMORY_TOOL,
            crate::runtime::chat_operations::RETRIEVE_PATIENT_WORK_TOOL,
            crate::clinical_library::SEARCH_LIBRARY_TOOL,
            crate::clinical_library::READ_LIBRARY_TOOL,
        ] {
            assert!(names.contains(&required), "missing {required} in {names:?}");
        }
        assert!(
            !names.contains(&crate::initial_understanding::SAVE_UNDERSTANDING_TOOL),
            "briefing recipe must not mutate patient understanding directly"
        );
    }

    #[test]
    fn selection_and_briefing_validation_reject_invalid_actions_and_accepted_care_phrasing() {
        let missing_schedule_time = json!({
            "decisions": [{
                "source_id": "cal",
                "appointment_id": "appt-1",
                "starts_at_unix": 1_800_000_000,
                "action": "schedule",
                "reason": "Prepare evening before visit."
            }]
        });
        assert!(matches!(
            SelectVisitsArgs::parse(&missing_schedule_time),
            Err(VisitPreparationInputError::InvalidSelectionAction { .. })
        ));

        let accepted_care_briefing = json!({
            "visit_card_orientation": "Heart failure follow-up after discharge.",
            "trajectory": [{
                "text": "NT-proBNP fell from 4,200 to 1,650 pg/mL prior to discharge.",
                "citations": [{ "candidate_id": "lab-1", "revision": "r1" }]
            }],
            "meaningful_changes": [{
                "text": "Discharged on bumetanide 1 mg daily.",
                "citations": [{ "candidate_id": "note-1", "revision": "r1" }]
            }],
            "unresolved_issues": [{
                "text": "Post-discharge basic metabolic panel pending.",
                "citations": [{ "candidate_id": "note-1", "revision": "r1" }]
            }],
            "visit_decisions": [{
                "decision": "Potassium and renal function check",
                "clinical_rationale": "Loop diuretic dose increased at discharge.",
                "options_or_questions": ["Order has been placed for stat BMP today."],
                "citations": [{ "candidate_id": "note-1", "revision": "r1" }]
            }],
            "basis_and_limitations": "Based on hospital discharge summary from 2026-08-24."
        });
        assert!(matches!(
            SaveBriefingArgs::parse(&accepted_care_briefing),
            Err(VisitPreparationInputError::RecommendationPresentedAsAcceptedCare { .. })
        ));
    }
}
