//! Durable visit preparation selection, pre-visit clinical briefings keyed to
//! visit time, schedule/evidence change reconciliation, and visit-concern lifecycle.
//!
//! Ownership rules stated once here:
//! - [`visit_can_start_preparation`] decides whether a visit may begin preparation:
//!   only [`AppointmentStatus::Scheduled`] visits with `now_unix < ends_at_unix`
//!   (upcoming or currently in progress) may start; once `now_unix >= ends_at_unix`,
//!   preparation never starts.
//! - [`should_retire_visit_concern`] decides when a pre-visit concern retires:
//!   ordinary [`ConcernUrgency::Today`] visit concerns retire when the visit ends
//!   or is cancelled/rescheduled, whereas [`ConcernUrgency::Now`] visit concerns
//!   remain [`ConcernStanding::Open`] past the visit end until opened by an
//!   authorized physician via [`Store::open_concern`].
//! - Each saved briefing belongs to `(practice, source_id, appointment_id, starts_at_unix)`.
//!   Cancelling or rescheduling an appointment cancels unfinished preparation work
//!   for the old time while preserving any completed briefing at the old time as
//!   dated history ([`VisitStanding::Moved`] or [`VisitStanding::Cancelled`]).

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use harness::engine::recipe::Provider;
use harness::engine::{self, Cancellation, Invocation, RunLimits, ToolOutcome};
use physician_auth::AuthStore;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
    TransactionTrait, Unchanged, sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::chat_operations::{
    MAX_RETRIEVED_WORK_ITEMS, PatientMemoryStanding, PatientStartingContext, PriorWorkKind,
    PriorWorkPointer, RecordLimit, RecordLimitKind, SourceFreshnessSummary,
};
use super::concerns::{
    CommandKey, Concern, ConcernError, ConcernId, ConcernMutation, ConcernUrgency,
    ExactRecordReference, RaiseConcern, RetireConcern, RetirementReason, SourceWork,
    WorkspaceAccess, load_concern, raise_concern_in_tx, retire_concern_in_tx,
};
use super::entities::{
    concern, practice_job, schedule_appointment, skill_package, visit_briefing, visit_concern,
    visit_preparation,
};
use super::jobs::{
    AssignmentError, PRACTICE_OWNER, PracticeAccessError, PracticeAuthority, PracticeJob,
    PracticeJobClaim, PracticeJobError, PracticeJobHost, PracticeScope,
};
use super::receipt::{self, Receipt};
use super::refresh::SourceRefreshOutcome;
use super::schedule::{
    AppointmentStatus, PatientMatch, SavedAppointment, ScheduleError, format_unix_rfc3339,
};
use super::sources::{self, SourcePull};
use super::understanding::{
    SummaryCitation, build_summary_citation, known_understanding_pairs, record_excerpt,
};
use super::{
    Accepted, AttemptFailure, Authority, Store, WorkStatus, conversion_error, error, now,
    persistence_failure,
};
use crate::ModelRoles;
use crate::physician_chat::PhysicianSkills;
use crate::physician_sources::SourceRevision;
use crate::visit_preparation::{
    BriefingClaimInput, BriefingDecisionInput, CandidateRecordPreview, CandidateVisitBrief,
    HistoricalBriefingSummary, PriorVisitDecisionSummary, SAVE_BRIEFING_TOOL, SELECT_VISITS_TOOL,
    SaveBriefingArgs, SelectVisitsArgs, SourceRefreshStatusSummary, VisitBriefingContext,
    VisitPreparationDecision, VisitPreparationInputError, VisitSelectionContext, briefing_job_id,
    selection_job_key,
};

/// Maximum recent chart record previews embedded per candidate visit in selection context.
const MAX_SELECTION_RECORD_PREVIEWS: usize = 6;

/// Lease duration in seconds for scheduled wake-up selection and briefing jobs.
const WAKE_JOB_LEASE_SECONDS: i64 = 540;

/// Reserved receipt actor for physician concern-open commands.
const CONCERN_OPEN_RECEIPT_ACTOR: &str = "coda:visit-concern-open";

/// Single rule deciding whether preparation may start for an appointment at `now_unix`.
///
/// Upcoming visits (`now_unix < starts_at_unix`) and in-progress visits
/// (`starts_at_unix <= now_unix < ends_at_unix`) may prepare while scheduled;
/// preparation never starts once `now_unix >= ends_at_unix` or once cancelled/closed.
pub const fn visit_can_start_preparation(
    status: AppointmentStatus,
    ends_at_unix: i64,
    now_unix: i64,
) -> bool {
    matches!(status, AppointmentStatus::Scheduled) && now_unix < ends_at_unix
}

/// Single rule deciding whether a visit briefing slot accepts a write from
/// `current_work_id`.
///
/// Once a briefing has been saved for `(practice, source_id, appointment_id,
/// starts_at_unix)`, only the originating briefing job (`existing_work_id ==
/// Some(current_work_id)`) may revise it during a mid-run evidence-change retry
/// before completion. Later chart input after completion must never rewrite the
/// briefing; a subsequent patient review raises its own concern if needed.
pub fn slot_allows_briefing_write(existing_work_id: Option<&str>, current_work_id: &str) -> bool {
    match existing_work_id {
        None => true,
        Some(owner) => owner == current_work_id,
    }
}

/// Single rule deciding whether an open visit concern retires automatically.
///
/// - Ordinary (`Today`) visit concerns retire as soon as the visit ends or is
///   cancelled/moved.
/// - Urgent (`Now`) visit concerns stay open past the visit end until an
///   authorized physician opens the concern (`was_opened == true`), after which
///   an ended/changed visit concern closes cleanly.
pub const fn should_retire_visit_concern(
    urgency: ConcernUrgency,
    was_opened: bool,
    visit_ended_or_changed: bool,
) -> bool {
    match urgency {
        ConcernUrgency::Today => visit_ended_or_changed,
        ConcernUrgency::Now => visit_ended_or_changed && was_opened,
    }
}

/// Composite key binding a visit preparation decision or saved briefing to one
/// scheduled visit time.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct VisitSlotKey {
    /// Practice identifier owning the schedule.
    pub practice: String,
    /// Schedule source identifier.
    pub source_id: String,
    /// Stable source appointment identifier.
    pub appointment_id: String,
    /// Exact scheduled start time in Unix seconds.
    pub starts_at_unix: i64,
}

impl VisitSlotKey {
    /// Deterministic briefing identifier unique within `practice`.
    pub fn briefing_id(&self) -> String {
        let raw = format!(
            "{}:{}:{}:{}",
            self.practice, self.source_id, self.appointment_id, self.starts_at_unix
        );
        let digest = &crate::sha256_hex(raw.as_bytes())[..16];
        format!(
            "briefing-{}-{}-{digest}",
            self.appointment_id, self.starts_at_unix
        )
    }

    fn pk_tuple(&self) -> (String, String, String, i64) {
        (
            self.practice.clone(),
            self.source_id.clone(),
            self.appointment_id.clone(),
            self.starts_at_unix,
        )
    }
}

/// Whether a saved briefing still belongs to the appointment's active slot or is
/// retained as dated history after the appointment moved or was cancelled.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum VisitStanding {
    /// The appointment remains scheduled at `slot.starts_at_unix`.
    Current,
    /// The appointment moved to a different start time; this briefing stays kept
    /// as dated history for the original `slot.starts_at_unix`.
    Moved {
        /// Updated RFC 3339 start timestamp of the rescheduled appointment.
        new_starts_at: String,
        /// Updated Unix start timestamp of the rescheduled appointment.
        new_starts_at_unix: i64,
        /// Practice-clock Unix second when the reschedule was recorded.
        noted_at: i64,
    },
    /// The appointment was cancelled or withdrawn; this briefing stays kept as
    /// dated history for the original `slot.starts_at_unix`.
    Cancelled {
        /// Practice-clock Unix second when the cancellation was recorded.
        noted_at: i64,
    },
}

#[derive(Serialize, Deserialize)]
struct StoredMovedDetail {
    new_starts_at: String,
    new_starts_at_unix: i64,
    noted_at: i64,
}

#[derive(Serialize, Deserialize)]
struct StoredCancelledDetail {
    noted_at: i64,
}

/// Background job provenance recorded on a saved visit briefing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BriefingOrigin {
    /// Practice background job identifier (`PracticeScope.job_id`).
    pub job_id: String,
    /// Work item identifier within the run store.
    pub work_id: String,
    /// Claimed attempt identifier that saved the briefing.
    pub attempt_id: String,
}

/// Durable pre-visit clinical briefing saved for one `(practice, source_id, appointment_id, starts_at_unix)` slot.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedVisitBriefing {
    /// Stable practice-unique briefing identifier.
    pub briefing_id: String,
    /// Scheduled visit slot this briefing belongs to.
    pub slot: VisitSlotKey,
    /// Matched Coda patient workspace identifier.
    pub workspace: String,
    /// Scheduled start timestamp in RFC 3339 format.
    pub starts_at: String,
    /// Scheduled end timestamp in RFC 3339 format.
    pub ends_at: String,
    /// Scheduled end timestamp in Unix seconds.
    pub ends_at_unix: i64,
    /// Saved version of the schedule appointment when prepared.
    pub appointment_saved_version: u64,
    /// Practice-clock Unix second when this briefing was saved.
    pub prepared_at: i64,
    /// Workspace chart observation version covered by this briefing.
    pub source_version: u64,
    /// Exact `(candidate_id -> revision)` map of the chart observation at `source_version`.
    pub source_versions: BTreeMap<String, String>,
    /// Per-source freshness and failed-refresh standing recorded with the briefing.
    pub source_freshness: Vec<SourceRefreshStatusSummary>,
    /// Explicit missing, unreadable, or unavailable evidence limits.
    pub limits: Vec<RecordLimit>,
    /// Concise 1–2 sentence visit-card orientation matching the briefing.
    pub visit_card_orientation: String,
    /// Longitudinal clinical trajectory claims with exact citations and inline uncertainty.
    pub trajectory: Vec<BriefingClaimInput>,
    /// Meaningful interval changes since prior care with exact citations and inline uncertainty.
    pub meaningful_changes: Vec<BriefingClaimInput>,
    /// Unresolved clinical issues and pending threads with exact citations and inline uncertainty.
    pub unresolved_issues: Vec<BriefingClaimInput>,
    /// Open clinical decisions and options framed strictly for physician judgment at the visit.
    pub visit_decisions: Vec<BriefingDecisionInput>,
    /// Physician-readable summary of dated evidence, freshness, and limits.
    pub basis_and_limitations: String,
    /// Optional exact record citations attached to `basis_and_limitations`.
    pub basis_citations: Vec<ExactRecordReference>,
    /// Verified record citations with excerpts, digests, dates, and native URIs for inspection.
    pub citations: Vec<SummaryCitation>,
    /// Rendered physician-facing markdown narrative of the full briefing.
    pub narrative: String,
    /// Optional pre-visit concern raised alongside this briefing.
    pub raised_concern_id: Option<ConcernId>,
    /// Whether this slot is still current or retained as dated history after a move/cancellation.
    pub visit_standing: VisitStanding,
    /// Originating background job, work, and attempt provenance.
    pub origin: BriefingOrigin,
    /// Practice-clock Unix second of the latest metadata update.
    pub updated_at: i64,
}

/// Execution standing of one visit's preparation lifecycle.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum VisitPreparationState {
    /// Selected to start preparation when `now_unix >= prepare_at_unix`.
    Scheduled {
        /// Target Unix second to begin briefing preparation (`< ends_at_unix`).
        prepare_at_unix: i64,
        /// Grounded clinical reason for the chosen preparation time.
        reason: String,
    },
    /// Deferred until `check_again_at_unix` while awaiting expected records.
    Deferred {
        /// Target Unix second to re-evaluate the visit (`< ends_at_unix`).
        check_again_at_unix: i64,
        /// Grounded clinical reason for deferring preparation.
        reason: String,
    },
    /// Skipped with an explicit clinical reason unless evidence or appointment changes.
    Skipped {
        /// Grounded clinical reason why no briefing is warranted.
        reason: String,
    },
    /// Briefing job has been accepted and is queued or running.
    Preparing {
        /// Practice background job identifier preparing this slot.
        job_id: String,
        /// Unix second when the briefing job started.
        started_at: i64,
        /// Grounded clinical reason retained from selection.
        reason: String,
    },
    /// Briefing is saved and ready to open.
    Ready {
        /// Saved briefing identifier.
        briefing_id: String,
        /// Unix second when the briefing was saved.
        prepared_at: i64,
        /// Concise orientation text for the schedule visit card.
        visit_card_orientation: String,
        /// Grounded clinical reason retained from selection.
        reason: String,
    },
    /// Briefing job exhausted automatic retries or held without finishing.
    CouldNotFinish {
        /// Practice background job identifier that failed or held.
        job_id: String,
        /// Unix second when the failed state was recorded.
        failed_at: i64,
        /// Failure explanation.
        reason: String,
    },
    /// Unfinished preparation ended because the appointment was cancelled or moved.
    EndedVisitChanged {
        /// Unix second when the schedule change ended preparation for this slot.
        ended_at: i64,
        /// Explanation of the cancellation or reschedule.
        reason: String,
    },
}

/// Durable preparation selection and lifecycle record for one visit slot.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisitPreparationRecord {
    /// Scheduled visit slot key.
    pub slot: VisitSlotKey,
    /// Matched Coda patient workspace identifier.
    pub workspace: String,
    /// Scheduled visit end time in Unix seconds.
    pub ends_at_unix: i64,
    /// Saved schedule appointment version observed when selected.
    pub appointment_saved_version: u64,
    /// Workspace chart observation version observed when selected.
    pub observed_source_version: u64,
    /// Model selection decision (`PrepareNow`, `Schedule`, `Defer`, or `Skip`).
    pub decision: VisitPreparationDecision,
    /// Unix second when the selection decision was recorded.
    pub decided_at: i64,
    /// Selection practice job identifier.
    pub selection_job_id: String,
    /// Selection work identifier.
    pub selection_work_id: String,
    /// Current preparation execution state.
    pub state: VisitPreparationState,
    /// Latest update Unix second.
    pub updated_at: i64,
}

/// Whether an authorized physician has opened a visit concern.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum VisitConcernOpenState {
    /// Not yet opened by a physician.
    Unopened,
    /// Opened by `physician` at `opened_at`.
    Opened {
        /// Unix second when first opened.
        opened_at: i64,
        /// Physician identifier who opened the concern.
        physician: String,
    },
}

/// Visit-bound lifecycle metadata linking a raised concern to its encounter window.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisitConcernLink {
    /// Practice identifier.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Linked concern identifier in `physician_runtime_concerns`.
    pub concern_id: ConcernId,
    /// Visit slot that raised this pre-visit concern.
    pub slot: VisitSlotKey,
    /// Scheduled visit end Unix timestamp.
    pub ends_at_unix: i64,
    /// Concern urgency (`Today` or `Now`).
    pub urgency: ConcernUrgency,
    /// Physician open state (governing `Now` concern post-visit persistence).
    pub open_state: VisitConcernOpenState,
    /// Unix second when linked.
    pub created_at: i64,
    /// Unix second when last updated.
    pub updated_at: i64,
}

/// Outcome of an authorized physician opening a concern via [`Store::open_concern`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConcernOpenOutcome {
    /// Recorded the physician opening (and retired the concern if it was an
    /// ended-visit `Now` concern waiting to be seen).
    Opened {
        /// Resulting durable concern state.
        concern: Box<Concern>,
        /// `true` when opening an ended/changed visit's `Now` concern completed
        /// its post-visit lifecycle and retired it.
        retired_after_visit_end: bool,
    },
    /// Exact `command_key` was already processed earlier; no duplicate mutation ran.
    Duplicate {
        /// Current durable concern state.
        concern: Box<Concern>,
    },
}

/// Summary of work executed during one scheduled visit-preparation wake-up for a practice.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduledWakeSummary {
    /// Practice identifier evaluated by the wake-up.
    pub practice: String,
    /// Practice-clock Unix second at which the wake-up evaluated the schedule.
    pub evaluated_at_unix: i64,
    /// Selection job executed during this wake-up, when undecided/changed visits existed.
    pub selection_job: Option<PracticeScope>,
    /// Briefing jobs newly queued or resumed during this wake-up.
    pub queued_briefing_jobs: Vec<PracticeScope>,
    /// Briefing IDs saved and committed during this wake-up.
    pub completed_briefings: Vec<String>,
    /// Practice jobs that exhausted retries or held without finishing (`CouldNotFinish`).
    pub failed_jobs: Vec<PracticeScope>,
    /// Ended/changed visit concerns retired during this wake-up.
    pub retired_concerns: Vec<ConcernId>,
}

/// Errors produced by visit preparation orchestration and briefing operations.
#[derive(Debug, thiserror::Error)]
pub enum BriefingError {
    /// Practice identifier or slot field was blank.
    #[error("visit preparation scope identifier must not be empty")]
    InvalidScope,
    /// Practice patient membership check failed.
    #[error("{0}")]
    Access(#[from] PracticeAccessError),
    /// Schedule readback failed.
    #[error("{0}")]
    Schedule(#[from] ScheduleError),
    /// Assignment construction failed.
    #[error("{0}")]
    Assignment(#[from] AssignmentError),
    /// Practice job acceptance or claim failed.
    #[error("{0}")]
    PracticeJob(#[from] PracticeJobError),
    /// Run-store lifecycle operation failed.
    #[error("{0}")]
    Runtime(#[from] super::RuntimeError),
    /// Skill catalog loading failed.
    #[error("{0}")]
    Skills(#[from] crate::physician_chat::SkillsError),
    /// Database query or transaction failed.
    #[error("{0}")]
    Database(#[from] DbErr),
}

fn membership_scope(practice: &str, workspace: &str) -> PracticeScope {
    PracticeScope {
        practice: practice.to_owned(),
        workspace: workspace.to_owned(),
        job_id: "visit-briefing-read".to_owned(),
    }
}

fn decode_visit_standing(standing: &str, detail: Option<Value>) -> Result<VisitStanding, DbErr> {
    match standing {
        "current" => Ok(VisitStanding::Current),
        "moved" => {
            let raw = detail.ok_or_else(|| error("corrupt moved visit_briefing missing detail"))?;
            let parsed: StoredMovedDetail = serde_json::from_value(raw).map_err(
                conversion_error("visit_briefing.visit_standing_detail", "StoredMovedDetail"),
            )?;
            Ok(VisitStanding::Moved {
                new_starts_at: parsed.new_starts_at,
                new_starts_at_unix: parsed.new_starts_at_unix,
                noted_at: parsed.noted_at,
            })
        }
        "cancelled" => {
            let raw =
                detail.ok_or_else(|| error("corrupt cancelled visit_briefing missing detail"))?;
            let parsed: StoredCancelledDetail =
                serde_json::from_value(raw).map_err(conversion_error(
                    "visit_briefing.visit_standing_detail",
                    "StoredCancelledDetail",
                ))?;
            Ok(VisitStanding::Cancelled {
                noted_at: parsed.noted_at,
            })
        }
        _ => Err(error("corrupt visit_briefing.visit_standing")),
    }
}

fn encode_visit_standing(standing: &VisitStanding) -> Result<(&'static str, Option<Value>), DbErr> {
    match standing {
        VisitStanding::Current => Ok(("current", None)),
        VisitStanding::Moved {
            new_starts_at,
            new_starts_at_unix,
            noted_at,
        } => Ok((
            "moved",
            Some(
                serde_json::to_value(StoredMovedDetail {
                    new_starts_at: new_starts_at.clone(),
                    new_starts_at_unix: *new_starts_at_unix,
                    noted_at: *noted_at,
                })
                .map_err(conversion_error("StoredMovedDetail", "json"))?,
            ),
        )),
        VisitStanding::Cancelled { noted_at } => Ok((
            "cancelled",
            Some(
                serde_json::to_value(StoredCancelledDetail {
                    noted_at: *noted_at,
                })
                .map_err(conversion_error("StoredCancelledDetail", "json"))?,
            ),
        )),
    }
}

fn decode_briefing(row: visit_briefing::Model) -> Result<SavedVisitBriefing, DbErr> {
    let appointment_saved_version = u64::try_from(row.appointment_saved_version).map_err(
        conversion_error("visit_briefing.appointment_saved_version", "u64"),
    )?;
    let source_version = u64::try_from(row.source_version)
        .map_err(conversion_error("visit_briefing.source_version", "u64"))?;
    let source_versions: BTreeMap<String, String> = serde_json::from_value(row.source_versions)
        .map_err(conversion_error(
            "visit_briefing.source_versions",
            "BTreeMap<String, String>",
        ))?;
    let source_freshness: Vec<SourceRefreshStatusSummary> =
        serde_json::from_value(row.source_freshness).map_err(conversion_error(
            "visit_briefing.source_freshness",
            "Vec<SourceRefreshStatusSummary>",
        ))?;
    let limits: Vec<RecordLimit> = serde_json::from_value(row.limits).map_err(conversion_error(
        "visit_briefing.limits",
        "Vec<RecordLimit>",
    ))?;
    let trajectory: Vec<BriefingClaimInput> = serde_json::from_value(row.trajectory).map_err(
        conversion_error("visit_briefing.trajectory", "Vec<BriefingClaimInput>"),
    )?;
    let meaningful_changes: Vec<BriefingClaimInput> =
        serde_json::from_value(row.meaningful_changes).map_err(conversion_error(
            "visit_briefing.meaningful_changes",
            "Vec<BriefingClaimInput>",
        ))?;
    let unresolved_issues: Vec<BriefingClaimInput> = serde_json::from_value(row.unresolved_issues)
        .map_err(conversion_error(
            "visit_briefing.unresolved_issues",
            "Vec<BriefingClaimInput>",
        ))?;
    let visit_decisions: Vec<BriefingDecisionInput> = serde_json::from_value(row.visit_decisions)
        .map_err(conversion_error(
        "visit_briefing.visit_decisions",
        "Vec<BriefingDecisionInput>",
    ))?;
    let basis_citations: Vec<ExactRecordReference> = serde_json::from_value(row.basis_citations)
        .map_err(conversion_error(
            "visit_briefing.basis_citations",
            "Vec<ExactRecordReference>",
        ))?;
    let citations: Vec<SummaryCitation> = serde_json::from_value(row.citations).map_err(
        conversion_error("visit_briefing.citations", "Vec<SummaryCitation>"),
    )?;
    let raised_concern_id = row
        .raised_concern_id
        .map(ConcernId::new)
        .transpose()
        .map_err(conversion_error(
            "visit_briefing.raised_concern_id",
            "ConcernId",
        ))?;
    let visit_standing = decode_visit_standing(&row.visit_standing, row.visit_standing_detail)?;
    let origin: BriefingOrigin = serde_json::from_value(row.origin)
        .map_err(conversion_error("visit_briefing.origin", "BriefingOrigin"))?;

    Ok(SavedVisitBriefing {
        briefing_id: row.briefing_id,
        slot: VisitSlotKey {
            practice: row.practice,
            source_id: row.source_id,
            appointment_id: row.appointment_id,
            starts_at_unix: row.starts_at_unix,
        },
        workspace: row.workspace,
        starts_at: row.starts_at,
        ends_at: row.ends_at,
        ends_at_unix: row.ends_at_unix,
        appointment_saved_version,
        prepared_at: row.prepared_at,
        source_version,
        source_versions,
        source_freshness,
        limits,
        visit_card_orientation: row.visit_card_orientation,
        trajectory,
        meaningful_changes,
        unresolved_issues,
        visit_decisions,
        basis_and_limitations: row.basis_and_limitations,
        basis_citations,
        citations,
        narrative: row.narrative,
        raised_concern_id,
        visit_standing,
        origin,
        updated_at: row.updated_at,
    })
}

fn decode_preparation_record(
    row: visit_preparation::Model,
    briefing_orientation: Option<&str>,
) -> Result<VisitPreparationRecord, DbErr> {
    let appointment_saved_version = u64::try_from(row.appointment_saved_version).map_err(
        conversion_error("visit_preparation.appointment_saved_version", "u64"),
    )?;
    let observed_source_version = u64::try_from(row.observed_source_version).map_err(
        conversion_error("visit_preparation.observed_source_version", "u64"),
    )?;
    let decision: VisitPreparationDecision = serde_json::from_value(row.decision).map_err(
        conversion_error("visit_preparation.decision", "VisitPreparationDecision"),
    )?;
    let reason = decision.reason().to_owned();
    let state = match row.state.as_str() {
        "scheduled" => {
            let prepare_at_unix = match &decision {
                VisitPreparationDecision::Schedule {
                    prepare_at_unix, ..
                } => *prepare_at_unix,
                VisitPreparationDecision::PrepareNow { .. } => row.decided_at,
                VisitPreparationDecision::Defer {
                    check_again_at_unix,
                    ..
                } => *check_again_at_unix,
                VisitPreparationDecision::Skip { .. } => row.decided_at,
            };
            VisitPreparationState::Scheduled {
                prepare_at_unix,
                reason,
            }
        }
        "deferred" => {
            let check_again_at_unix = match &decision {
                VisitPreparationDecision::Defer {
                    check_again_at_unix,
                    ..
                } => *check_again_at_unix,
                _ => row.decided_at,
            };
            VisitPreparationState::Deferred {
                check_again_at_unix,
                reason,
            }
        }
        "skipped" => VisitPreparationState::Skipped { reason },
        "preparing" => VisitPreparationState::Preparing {
            job_id: row
                .briefing_job_id
                .ok_or_else(|| error("corrupt preparing visit_preparation missing job_id"))?,
            started_at: row.updated_at,
            reason,
        },
        "ready" => VisitPreparationState::Ready {
            briefing_id: row
                .briefing_id
                .ok_or_else(|| error("corrupt ready visit_preparation missing briefing_id"))?,
            prepared_at: row.updated_at,
            visit_card_orientation: briefing_orientation.unwrap_or_default().to_owned(),
            reason,
        },
        "could_not_finish" => VisitPreparationState::CouldNotFinish {
            job_id: row.briefing_job_id.unwrap_or_default(),
            failed_at: row.updated_at,
            reason: row
                .failed_reason
                .unwrap_or_else(|| "Visit preparation could not finish.".to_owned()),
        },
        "ended_visit_changed" => VisitPreparationState::EndedVisitChanged {
            ended_at: row.updated_at,
            reason: row
                .failed_reason
                .unwrap_or_else(|| "Appointment was cancelled or rescheduled.".to_owned()),
        },
        _ => return Err(error("corrupt visit_preparation.state")),
    };

    Ok(VisitPreparationRecord {
        slot: VisitSlotKey {
            practice: row.practice,
            source_id: row.source_id,
            appointment_id: row.appointment_id,
            starts_at_unix: row.starts_at_unix,
        },
        workspace: row.workspace,
        ends_at_unix: row.ends_at_unix,
        appointment_saved_version,
        observed_source_version,
        decision,
        decided_at: row.decided_at,
        selection_job_id: row.selection_job_id,
        selection_work_id: row.selection_work_id,
        state,
        updated_at: row.updated_at,
    })
}

fn decode_visit_concern_link(row: visit_concern::Model) -> Result<VisitConcernLink, DbErr> {
    let concern_id = ConcernId::new(row.concern_id)
        .map_err(conversion_error("visit_concern.concern_id", "ConcernId"))?;
    let urgency = match row.urgency.as_str() {
        "now" => ConcernUrgency::Now,
        "today" => ConcernUrgency::Today,
        _ => return Err(error("corrupt visit_concern.urgency")),
    };
    let open_state = match (row.opened_at, row.opened_by_physician) {
        (None, None) => VisitConcernOpenState::Unopened,
        (Some(opened_at), Some(physician)) => VisitConcernOpenState::Opened {
            opened_at,
            physician,
        },
        _ => return Err(error("corrupt visit_concern open state")),
    };
    Ok(VisitConcernLink {
        practice: row.practice.clone(),
        workspace: row.workspace,
        concern_id,
        slot: VisitSlotKey {
            practice: row.practice,
            source_id: row.source_id,
            appointment_id: row.appointment_id,
            starts_at_unix: row.starts_at_unix,
        },
        ends_at_unix: row.ends_at_unix,
        urgency,
        open_state,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

fn render_briefing_narrative(starts_at: &str, label: &str, args: &SaveBriefingArgs) -> String {
    let mut out = String::new();
    out.push_str(&format!("## Visit Orientation ({label} — {starts_at})\n"));
    out.push_str(&args.visit_card_orientation);
    out.push_str("\n\n## Trajectory\n");
    for item in &args.trajectory {
        out.push_str("- ");
        out.push_str(&item.text);
        if let Some(uncertainty) = &item.uncertainty {
            out.push_str(" *(Uncertainty: ");
            out.push_str(uncertainty);
            out.push_str(")*");
        }
        out.push('\n');
    }
    out.push_str("\n## Meaningful Changes\n");
    for item in &args.meaningful_changes {
        out.push_str("- ");
        out.push_str(&item.text);
        if let Some(uncertainty) = &item.uncertainty {
            out.push_str(" *(Uncertainty: ");
            out.push_str(uncertainty);
            out.push_str(")*");
        }
        out.push('\n');
    }
    out.push_str("\n## Unresolved Issues\n");
    for item in &args.unresolved_issues {
        out.push_str("- ");
        out.push_str(&item.text);
        if let Some(uncertainty) = &item.uncertainty {
            out.push_str(" *(Uncertainty: ");
            out.push_str(uncertainty);
            out.push_str(")*");
        }
        out.push('\n');
    }
    out.push_str("\n## Decisions & Questions for Physician Consideration\n");
    for item in &args.visit_decisions {
        out.push_str("- **");
        out.push_str(&item.decision);
        out.push_str("**: ");
        out.push_str(&item.clinical_rationale);
        if let Some(uncertainty) = &item.uncertainty {
            out.push_str(" *(Uncertainty: ");
            out.push_str(uncertainty);
            out.push_str(")*");
        }
        out.push('\n');
        for opt in &item.options_or_questions {
            out.push_str("  - ");
            out.push_str(opt);
            out.push('\n');
        }
    }
    out.push_str("\n## Basis and Limitations\n");
    out.push_str(&args.basis_and_limitations);
    out
}

/// Reconciles unfinished visit preparations, completed briefings, and visit-bound
/// concerns inside caller-supplied transaction `tx` after a schedule sync or wake-up.
pub(super) async fn reconcile_schedule_changes_in_tx(
    tx: &sea_orm::DatabaseTransaction,
    practice: &str,
    now_unix: i64,
) -> Result<Vec<ConcernId>, DbErr> {
    let appt_rows = schedule_appointment::Entity::find()
        .filter(schedule_appointment::Column::Practice.eq(practice))
        .all(tx)
        .await?;
    let appt_map: BTreeMap<(String, String), schedule_appointment::Model> = appt_rows
        .into_iter()
        .map(|row| ((row.source_id.clone(), row.appointment_id.clone()), row))
        .collect();

    // 1. End unfinished preparation work whose appointment slot was cancelled or moved.
    let prep_rows = visit_preparation::Entity::find()
        .filter(visit_preparation::Column::Practice.eq(practice))
        .filter(visit_preparation::Column::State.is_in(["scheduled", "deferred", "preparing"]))
        .all(tx)
        .await?;

    for prep in prep_rows {
        let change_reason =
            match appt_map.get(&(prep.source_id.clone(), prep.appointment_id.clone())) {
                None => Some("Appointment was withdrawn from the practice schedule.".to_owned()),
                Some(appt) if appt.status != AppointmentStatus::Scheduled => Some(format!(
                    "Appointment is no longer scheduled (status `{}`).",
                    appt.source_status
                )),
                Some(appt) if appt.starts_at_unix != prep.starts_at_unix => Some(format!(
                    "Appointment moved from unix {} to {} ({}).",
                    prep.starts_at_unix, appt.starts_at_unix, appt.starts_at
                )),
                Some(appt) if appt.workspace.as_deref() != Some(prep.workspace.as_str()) => {
                    Some("Appointment patient match changed on the practice schedule.".to_owned())
                }
                Some(_) => None,
            };

        if let Some(reason) = change_reason {
            if let Some(job_id) = &prep.briefing_job_id {
                let job_scope = PracticeScope {
                    practice: prep.practice.clone(),
                    workspace: prep.workspace.clone(),
                    job_id: job_id.clone(),
                };
                Store::cancel_practice_job_in_tx(tx, &job_scope, now_unix)
                    .await
                    .map_err(|err| match err {
                        super::RuntimeError::Database(db) => db,
                        other => error(&other.to_string()),
                    })?;
            }
            visit_preparation::ActiveModel {
                practice: Unchanged(prep.practice),
                source_id: Unchanged(prep.source_id),
                appointment_id: Unchanged(prep.appointment_id),
                starts_at_unix: Unchanged(prep.starts_at_unix),
                workspace: Unchanged(prep.workspace),
                ends_at_unix: Unchanged(prep.ends_at_unix),
                appointment_saved_version: Unchanged(prep.appointment_saved_version),
                observed_source_version: Unchanged(prep.observed_source_version),
                decision: Unchanged(prep.decision),
                decided_at: Unchanged(prep.decided_at),
                selection_job_id: Unchanged(prep.selection_job_id),
                selection_work_id: Unchanged(prep.selection_work_id),
                selection_idempotency_key: Unchanged(prep.selection_idempotency_key),
                state: Set("ended_visit_changed".to_owned()),
                briefing_job_id: Unchanged(prep.briefing_job_id),
                briefing_id: Unchanged(prep.briefing_id),
                failed_reason: Set(Some(reason)),
                updated_at: Set(now_unix),
            }
            .update(tx)
            .await?;
        }
    }

    // 2. Preserve completed briefings at moved or cancelled visit times as dated history.
    let current_briefings = visit_briefing::Entity::find()
        .filter(visit_briefing::Column::Practice.eq(practice))
        .filter(visit_briefing::Column::VisitStanding.eq("current"))
        .all(tx)
        .await?;

    for row in current_briefings {
        let next_standing = match appt_map.get(&(row.source_id.clone(), row.appointment_id.clone()))
        {
            None => Some(VisitStanding::Cancelled { noted_at: now_unix }),
            Some(appt) if appt.starts_at_unix != row.starts_at_unix => Some(VisitStanding::Moved {
                new_starts_at: appt.starts_at.clone(),
                new_starts_at_unix: appt.starts_at_unix,
                noted_at: now_unix,
            }),
            Some(appt) if appt.status == AppointmentStatus::Cancelled => {
                Some(VisitStanding::Cancelled { noted_at: now_unix })
            }
            Some(_) => None,
        };
        if let Some(standing) = next_standing {
            let (standing_col, detail_col) = encode_visit_standing(&standing)?;
            visit_briefing::ActiveModel {
                practice: Unchanged(row.practice),
                source_id: Unchanged(row.source_id),
                appointment_id: Unchanged(row.appointment_id),
                starts_at_unix: Unchanged(row.starts_at_unix),
                briefing_id: Unchanged(row.briefing_id),
                workspace: Unchanged(row.workspace),
                idempotency_key: Unchanged(row.idempotency_key),
                starts_at: Unchanged(row.starts_at),
                ends_at: Unchanged(row.ends_at),
                ends_at_unix: Unchanged(row.ends_at_unix),
                appointment_saved_version: Unchanged(row.appointment_saved_version),
                prepared_at: Unchanged(row.prepared_at),
                source_version: Unchanged(row.source_version),
                source_versions: Unchanged(row.source_versions),
                source_freshness: Unchanged(row.source_freshness),
                limits: Unchanged(row.limits),
                visit_card_orientation: Unchanged(row.visit_card_orientation),
                trajectory: Unchanged(row.trajectory),
                meaningful_changes: Unchanged(row.meaningful_changes),
                unresolved_issues: Unchanged(row.unresolved_issues),
                visit_decisions: Unchanged(row.visit_decisions),
                basis_and_limitations: Unchanged(row.basis_and_limitations),
                basis_citations: Unchanged(row.basis_citations),
                citations: Unchanged(row.citations),
                narrative: Unchanged(row.narrative),
                raised_concern_id: Unchanged(row.raised_concern_id),
                visit_standing: Set(standing_col.to_owned()),
                visit_standing_detail: Set(detail_col),
                origin: Unchanged(row.origin),
                updated_at: Set(now_unix),
            }
            .update(tx)
            .await?;
        }
    }

    // 3. Reconcile visit-scoped concerns against `now_unix` and appointment changes.
    reconcile_visit_concerns_in_tx(tx, practice, &appt_map, now_unix).await
}

async fn reconcile_visit_concerns_in_tx(
    tx: &sea_orm::DatabaseTransaction,
    practice: &str,
    appt_map: &BTreeMap<(String, String), schedule_appointment::Model>,
    now_unix: i64,
) -> Result<Vec<ConcernId>, DbErr> {
    let links = visit_concern::Entity::find()
        .filter(visit_concern::Column::Practice.eq(practice))
        .order_by_asc(visit_concern::Column::CreatedAt)
        .order_by_asc(visit_concern::Column::ConcernId)
        .all(tx)
        .await?;

    let mut retired_ids = Vec::new();
    for raw_link in links {
        let link = decode_visit_concern_link(raw_link)?;
        let appt = appt_map.get(&(
            link.slot.source_id.clone(),
            link.slot.appointment_id.clone(),
        ));
        if reconcile_one_visit_concern_in_tx(tx, &link, appt, now_unix).await? {
            retired_ids.push(link.concern_id);
        }
    }

    Ok(retired_ids)
}

async fn reconcile_one_visit_concern_in_tx(
    tx: &sea_orm::DatabaseTransaction,
    link: &VisitConcernLink,
    appt: Option<&schedule_appointment::Model>,
    now_unix: i64,
) -> Result<bool, DbErr> {
    let Some(current_concern) =
        load_concern(tx, &link.practice, &link.workspace, &link.concern_id).await?
    else {
        return Ok(false);
    };
    if !current_concern.shows_on_today() {
        return Ok(false);
    }

    let (visit_ended_or_changed, reason_text) = match appt {
        None => (
            true,
            format!(
                "Scheduled visit {} was withdrawn from the practice calendar.",
                link.slot.appointment_id
            ),
        ),
        Some(appt) if appt.status == AppointmentStatus::Cancelled => (
            true,
            format!(
                "Scheduled visit {} was cancelled on the practice calendar.",
                link.slot.appointment_id
            ),
        ),
        Some(appt) if appt.starts_at_unix != link.slot.starts_at_unix => (
            true,
            format!(
                "Scheduled visit {} moved to {}; earlier slot pre-visit concern retired.",
                link.slot.appointment_id, appt.starts_at
            ),
        ),
        Some(appt)
            if matches!(
                appt.status,
                AppointmentStatus::Fulfilled | AppointmentStatus::NoShow
            ) || now_unix >= link.ends_at_unix =>
        {
            (
                true,
                format!(
                    "Scheduled visit {} ended at {}.",
                    link.slot.appointment_id,
                    format_unix_rfc3339(link.ends_at_unix)
                ),
            )
        }
        Some(_) if now_unix >= link.ends_at_unix => (
            true,
            format!(
                "Scheduled visit {} ended at {}.",
                link.slot.appointment_id,
                format_unix_rfc3339(link.ends_at_unix)
            ),
        ),
        Some(_) => (false, String::new()),
    };

    let was_opened = matches!(link.open_state, VisitConcernOpenState::Opened { .. });
    if !should_retire_visit_concern(link.urgency, was_opened, visit_ended_or_changed) {
        return Ok(false);
    }

    let retirement_reason = RetirementReason::new(reason_text).map_err(conversion_error(
        "visit concern retirement reason",
        "RetirementReason",
    ))?;
    let source_work = SourceWork::new(
        "visit_preparation_lifecycle",
        format!(
            "visit-concern-close:{}:{}:{}",
            link.slot.source_id, link.slot.appointment_id, link.slot.starts_at_unix
        ),
        "slot-lifecycle",
    )
    .map_err(conversion_error("visit concern source_work", "SourceWork"))?;
    let key_digest = &crate::sha256_hex(
        format!(
            "retire-visit-concern:{}:{}:{}:{}:{}",
            link.practice,
            link.workspace,
            link.concern_id,
            link.slot.starts_at_unix,
            retirement_reason.as_str(),
        )
        .as_bytes(),
    )[..24];
    let command_key = CommandKey::new(format!("vc-retire:{key_digest}"))
        .map_err(conversion_error("visit concern retire key", "CommandKey"))?;

    match retire_concern_in_tx(
        tx,
        &command_key,
        &RetireConcern {
            practice: link.practice.clone(),
            workspace: link.workspace.clone(),
            id: link.concern_id.clone(),
            reason: retirement_reason,
            source_work,
        },
    )
    .await
    {
        Ok(ConcernMutation::Applied(_) | ConcernMutation::Duplicate(_)) => Ok(true),
        Err(ConcernError::NotOpen { .. }) => Ok(false),
        Err(ConcernError::Database(db)) => Err(db),
        Err(other) => Err(error(&other.to_string())),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BriefingFenceStage {
    SaveTool,
    Recovery,
    Finish,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChartChangedFeedback {
    outcome: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pinned_source_version: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saved_source_version: Option<u64>,
    current_source_version: u64,
    changed_records: Vec<CandidateRecordPreview>,
    withdrawn_candidates: Vec<String>,
    message: String,
}

#[derive(Deserialize)]
struct DeliveredReadReceipt {
    results: Vec<DeliveredRecordReceipt>,
}

#[derive(Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
enum DeliveredRecordReceipt {
    Available { reference: String, revision: String },
    Passage { reference: String, revision: String },
    Unavailable {},
    OverBudget {},
}

fn apply_acknowledged_withdrawal_feedback(
    text: &str,
    known_revisions: &mut BTreeMap<String, String>,
) {
    if let Ok(feedback) = serde_json::from_str::<ChartChangedFeedback>(text)
        && feedback.outcome == "chart_changed_mid_run"
    {
        for cid in feedback.withdrawn_candidates {
            known_revisions.remove(&cid);
        }
    }
}

/// Reconstructs the exact `(candidate_id -> revision)` map known to the model
/// at this point in a visit-briefing run, starting from `baseline_sources`
/// (`job.sources`), advancing in chronological order through delivered
/// `read_patient_sources` receipts, and clearing candidates whose withdrawal was
/// already surfaced to the model via `chart_changed_mid_run` feedback.
fn model_chart_knowledge(
    baseline_sources: &[SourceRevision],
    ordered: &[(engine::Event, bool)],
    message_sources: &[engine::ProductMessage],
    flush_trailing_turn: bool,
) -> Result<BTreeMap<String, String>, String> {
    let delivered_revs =
        crate::physician_sources::delivered_exact_revisions(ordered.iter(), message_sources)?;
    let mut known_revisions: BTreeMap<String, String> = baseline_sources
        .iter()
        .map(|src| (src.candidate_id.clone(), src.revision.clone()))
        .collect();
    let mut pending_reads: Vec<(String, String)> = Vec::new();
    let mut pending_withdrawn: Vec<String> = Vec::new();

    for (event, _) in ordered {
        match &event.kind {
            engine::EventKind::ToolReturned {
                result: engine::results::ToolResult::Original(output),
                ..
            } => {
                if let Ok(receipt) = serde_json::from_str::<DeliveredReadReceipt>(output) {
                    for item in receipt.results {
                        match item {
                            DeliveredRecordReceipt::Available {
                                reference,
                                revision,
                            }
                            | DeliveredRecordReceipt::Passage {
                                reference,
                                revision,
                            } => {
                                if flush_trailing_turn
                                    || delivered_revs
                                        .contains(&(reference.clone(), revision.clone()))
                                {
                                    pending_reads.push((reference, revision));
                                }
                            }
                            DeliveredRecordReceipt::Unavailable {}
                            | DeliveredRecordReceipt::OverBudget {} => {}
                        }
                    }
                }
            }
            engine::EventKind::ToolReturned {
                result: engine::results::ToolResult::Refused(refusal),
                ..
            } => {
                if let Ok(feedback) = serde_json::from_str::<ChartChangedFeedback>(refusal.reason())
                    && feedback.outcome == "chart_changed_mid_run"
                {
                    pending_withdrawn.extend(feedback.withdrawn_candidates);
                }
            }
            engine::EventKind::ProviderStarted { request, .. } => {
                for (cid, rev) in pending_reads.drain(..) {
                    known_revisions.insert(cid, rev);
                }
                for cid in pending_withdrawn.drain(..) {
                    known_revisions.remove(&cid);
                }
                for item in &request.added {
                    match item {
                        engine::Item::Feedback { text } => {
                            apply_acknowledged_withdrawal_feedback(text, &mut known_revisions);
                        }
                        engine::Item::FinishRefused(refusal) => {
                            apply_acknowledged_withdrawal_feedback(
                                refusal.reason(),
                                &mut known_revisions,
                            );
                        }
                        engine::Item::ProductMessage(_)
                        | engine::Item::OutputCutoff
                        | engine::Item::Message { .. }
                        | engine::Item::Call(_)
                        | engine::Item::Result { .. }
                        | engine::Item::MaterializedResult { .. }
                        | engine::Item::Opaque(_) => {}
                    }
                }
            }
            engine::EventKind::ToolReturned { .. }
            | engine::EventKind::ProviderObservation { .. }
            | engine::EventKind::ProviderFailed { .. }
            | engine::EventKind::TransportRetry { .. }
            | engine::EventKind::ProviderReturned(_)
            | engine::EventKind::ToolStarted(_)
            | engine::EventKind::ProviderBlocked { .. }
            | engine::EventKind::ContextRejected
            | engine::EventKind::Candidate { .. }
            | engine::EventKind::CallBegan { .. }
            | engine::EventKind::WindowOpened { .. } => {}
        }
    }

    if flush_trailing_turn {
        for (cid, rev) in pending_reads.drain(..) {
            known_revisions.insert(cid, rev);
        }
        for cid in pending_withdrawn.drain(..) {
            known_revisions.remove(&cid);
        }
    }

    Ok(known_revisions)
}

/// Single-owner chart-currency check shared across `save_visit_briefing`,
/// interrupted-attempt recovery (`PracticeJobHost::run`), and run completion
/// (`PracticeJobHost::finish`).
fn briefing_source_change_feedback(
    stage: BriefingFenceStage,
    baseline_version: u64,
    known_revisions: &BTreeMap<String, String>,
    current_version: u64,
    current_sources: &[SourceRevision],
) -> Option<Value> {
    let mut unread_changes = Vec::new();
    let mut current_candidates = BTreeSet::new();
    for source in current_sources {
        current_candidates.insert(source.candidate_id.as_str());
        if known_revisions
            .get(&source.candidate_id)
            .map(String::as_str)
            == Some(source.revision.as_str())
        {
            continue;
        }
        unread_changes.push(CandidateRecordPreview {
            candidate_id: source.candidate_id.clone(),
            revision: source.revision.clone(),
            source_id: source.source_id.clone(),
            record_type: source.record_type.clone(),
            dates: source.dates.clone(),
            excerpt: record_excerpt(source),
        });
    }

    let withdrawn_candidates: Vec<String> = known_revisions
        .keys()
        .filter(|cid| !current_candidates.contains(cid.as_str()))
        .cloned()
        .collect();

    let needs_feedback = match stage {
        BriefingFenceStage::SaveTool | BriefingFenceStage::Recovery => {
            !unread_changes.is_empty() || !withdrawn_candidates.is_empty()
        }
        BriefingFenceStage::Finish => {
            current_version > baseline_version
                || !unread_changes.is_empty()
                || !withdrawn_candidates.is_empty()
        }
    };
    if !needs_feedback {
        return None;
    }

    let (pinned_source_version, saved_source_version, message) = match stage {
        BriefingFenceStage::SaveTool => (
            Some(baseline_version),
            None,
            "Patient chart records changed during preparation. Open any changed record revisions via read_patient_sources, remove reliance on any withdrawn records, and call save_visit_briefing with the updated synthesis.",
        ),
        BriefingFenceStage::Recovery => (
            Some(baseline_version),
            None,
            "Patient chart records changed before recovery completed. Inspect any changed record revisions via read_patient_sources, remove reliance on any withdrawn records, and call save_visit_briefing again before finishing.",
        ),
        BriefingFenceStage::Finish => (
            None,
            Some(baseline_version),
            "New, updated, or withdrawn patient chart records arrived before run completion. Read any changed revisions via read_patient_sources, remove reliance on any withdrawn records, and call save_visit_briefing with the updated synthesis before finishing.",
        ),
    };

    Some(
        serde_json::to_value(ChartChangedFeedback {
            outcome: "chart_changed_mid_run".to_owned(),
            pinned_source_version,
            saved_source_version,
            current_source_version: current_version,
            changed_records: unread_changes,
            withdrawn_candidates,
            message: message.to_owned(),
        })
        .expect("ChartChangedFeedback serializes cleanly as JSON"),
    )
}

pub(super) async fn sync_visit_preparation_job_in_tx(
    tx: &sea_orm::DatabaseTransaction,
    job: &PracticeJob,
) -> Result<(), DbErr> {
    if job.kind != super::jobs::PracticeJobKind::VisitPreparation {
        return Ok(());
    }
    let Some(briefing_ctx) = job.assignment.visit_briefing.as_ref() else {
        return Ok(());
    };
    let slot_pk = (
        job.scope.practice.clone(),
        briefing_ctx.source_id.clone(),
        briefing_ctx.appointment_id.clone(),
        briefing_ctx.starts_at_unix,
    );
    let Some(row) = visit_preparation::Entity::find_by_id(slot_pk)
        .lock_exclusive()
        .one(tx)
        .await?
    else {
        return Ok(());
    };
    if row.state != "preparing" || row.briefing_job_id.as_deref() != Some(job.scope.job_id.as_str())
    {
        return Ok(());
    }
    let (next_state, next_reason) = match job.status {
        WorkStatus::Failed => (
            "could_not_finish",
            Some(format!(
                "Visit preparation job `{}` ended in `failed` state without saving a briefing.",
                job.scope.job_id
            )),
        ),
        WorkStatus::Held => (
            "could_not_finish",
            Some(format!(
                "Visit preparation job `{}` ended in `held` state without saving a briefing.",
                job.scope.job_id
            )),
        ),
        WorkStatus::Cancelled => (
            "ended_visit_changed",
            Some("Appointment changed or preparation cancelled before completion.".to_owned()),
        ),
        WorkStatus::Pending | WorkStatus::Active | WorkStatus::Completed => return Ok(()),
    };
    visit_preparation::ActiveModel {
        practice: Unchanged(row.practice),
        source_id: Unchanged(row.source_id),
        appointment_id: Unchanged(row.appointment_id),
        starts_at_unix: Unchanged(row.starts_at_unix),
        workspace: Unchanged(row.workspace),
        ends_at_unix: Unchanged(row.ends_at_unix),
        appointment_saved_version: Unchanged(row.appointment_saved_version),
        observed_source_version: Unchanged(row.observed_source_version),
        decision: Unchanged(row.decision),
        decided_at: Unchanged(row.decided_at),
        selection_job_id: Unchanged(row.selection_job_id),
        selection_work_id: Unchanged(row.selection_work_id),
        selection_idempotency_key: Unchanged(row.selection_idempotency_key),
        state: Set(next_state.to_owned()),
        briefing_job_id: Unchanged(row.briefing_job_id),
        briefing_id: Unchanged(row.briefing_id),
        failed_reason: Set(next_reason),
        updated_at: Set(job.updated_at),
    }
    .update(tx)
    .await?;
    Ok(())
}

fn verify_briefing_reference<'a>(
    reference: &ExactRecordReference,
    by_candidate: &BTreeMap<&str, &'a SourceRevision>,
    read_in_work: &BTreeSet<(String, String)>,
    carried_forward: &BTreeSet<(String, String)>,
) -> Result<&'a SourceRevision, Value> {
    let Some(source) = by_candidate.get(reference.candidate_id.as_str()).copied() else {
        return Err(json!({
            "outcome": "unknown_source_reference",
            "candidate_id": reference.candidate_id,
            "message": format!(
                "Cited record `{}` is not present in the workspace's current chart observation. Nothing was saved.",
                reference.candidate_id
            ),
        }));
    };
    if source.revision != reference.revision {
        return Err(json!({
            "outcome": "stale_source_revision",
            "candidate_id": reference.candidate_id,
            "cited_revision": reference.revision,
            "current_revision": source.revision,
            "current_excerpt": record_excerpt(source),
            "message": format!(
                "Cited record `{}` moved from revision `{}` to `{}`. Read `{}` via read_patient_sources before saving. Nothing was saved.",
                reference.candidate_id, reference.revision, source.revision, source.revision
            ),
        }));
    }
    let key = (reference.candidate_id.clone(), reference.revision.clone());
    if !read_in_work.contains(&key) && !carried_forward.contains(&key) {
        return Err(json!({
            "outcome": "unread_source_reference",
            "candidate_id": reference.candidate_id,
            "revision": reference.revision,
            "message": format!(
                "Cited record `{}` at revision `{}` has not been opened via read_patient_sources in this run. Open the exact revision before citing it. Nothing was saved.",
                reference.candidate_id, reference.revision
            ),
        }));
    }
    Ok(source)
}

impl Store {
    /// Resolves the [`PhysicianSkills`] catalog for background worker wake-ups:
    /// 1. Explicit directory path when configured;
    /// 2. Workspace `skills/` directory when present on disk;
    /// 3. Latest skill package already stored in `physician_runtime_skills`;
    /// 4. Minimal built-in fallback catalog when running in an isolated container
    ///    before any skill package has been persisted.
    pub async fn resolve_worker_skills(
        &self,
        explicit_dir: Option<&str>,
    ) -> Result<PhysicianSkills, BriefingError> {
        if let Some(dir) = explicit_dir.map(str::trim).filter(|s| !s.is_empty()) {
            return Ok(PhysicianSkills::load(std::path::Path::new(dir))?);
        }
        let repo_skills = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills");
        if repo_skills.is_dir() {
            return Ok(PhysicianSkills::load(&repo_skills)?);
        }
        if let Some(saved) = skill_package::Entity::find()
            .order_by_desc(skill_package::Column::Practice)
            .one(&self.db)
            .await?
        {
            let decoded: PhysicianSkills = serde_json::from_value(saved.body)
                .map_err(conversion_error("skill_package.body", "PhysicianSkills"))?;
            if matches!(
                decoded.execute(
                    harness::skills::SKILL_VIEW,
                    &json!({ "name": crate::visit_preparation::VISIT_PREPARATION_SKILL }),
                ),
                ToolOutcome::Returned(_)
            ) {
                return Ok(decoded);
            }
        }
        Ok(PhysicianSkills::bundled_fallback()?)
    }

    /// Assembles [`SourceRefreshStatusSummary`] entries combining last-good pulls
    /// (`physician_runtime_source_pulls`) and per-source refresh status rows
    /// (`physician_runtime_source_statuses`).
    pub(super) async fn workspace_source_refresh_summaries(
        &self,
        practice: &str,
        workspace: &str,
    ) -> Result<Vec<SourceRefreshStatusSummary>, DbErr> {
        let pulls = self
            .source_pulls(&[workspace.to_owned()])
            .await?
            .remove(workspace)
            .unwrap_or_default();
        let statuses = self
            .saved_source_statuses(practice, workspace)
            .await
            .map_err(|err| match err {
                super::refresh::RefreshError::Database(db) => db,
                other => error(&other.to_string()),
            })?;

        let mut map: BTreeMap<String, SourceRefreshStatusSummary> = BTreeMap::new();
        for pull in pulls {
            map.insert(
                pull.source_id.clone(),
                SourceRefreshStatusSummary {
                    source_id: pull.source_id,
                    source_label: pull.source_label,
                    last_good_pulled_at: pull.pulled_at,
                    last_failed_at: None,
                    failure_reason: None,
                },
            );
        }
        for saved in statuses {
            let sid = saved.source_id.as_str().to_owned();
            match saved.outcome {
                SourceRefreshOutcome::Updated { pulled_at, .. } => {
                    map.insert(
                        sid.clone(),
                        SourceRefreshStatusSummary {
                            source_id: sid,
                            source_label: saved.source_label,
                            last_good_pulled_at: Some(pulled_at),
                            last_failed_at: None,
                            failure_reason: None,
                        },
                    );
                }
                SourceRefreshOutcome::PartlyUpdated {
                    pulled_at, reason, ..
                } => {
                    map.insert(
                        sid.clone(),
                        SourceRefreshStatusSummary {
                            source_id: sid,
                            source_label: saved.source_label,
                            last_good_pulled_at: Some(pulled_at),
                            last_failed_at: Some(pulled_at),
                            failure_reason: Some(reason),
                        },
                    );
                }
                SourceRefreshOutcome::Failed {
                    last_good_pulled_at,
                    checked_at,
                    reason,
                } => {
                    let prior_good = map
                        .get(&sid)
                        .and_then(|existing| existing.last_good_pulled_at)
                        .or(last_good_pulled_at);
                    map.insert(
                        sid.clone(),
                        SourceRefreshStatusSummary {
                            source_id: sid,
                            source_label: saved.source_label,
                            last_good_pulled_at: prior_good,
                            last_failed_at: Some(checked_at),
                            failure_reason: Some(reason),
                        },
                    );
                }
            }
        }
        Ok(map.into_values().collect())
    }

    /// Assembles [`PatientStartingContext`] for a practice-owned background job
    /// with no physician session.
    pub(super) async fn practice_starting_context(
        &self,
        practice: &str,
        workspace: &str,
    ) -> Result<PatientStartingContext, DbErr> {
        let mut observation = self.sources(workspace).await?;
        let withdrawal_boundary =
            super::follow_ups::load_workspace_withdrawal_boundary(&self.db, practice, workspace)
                .await?;
        if let Some(boundary) = &withdrawal_boundary {
            let withdrawn_set: std::collections::BTreeSet<&str> =
                boundary.candidate_ids.iter().map(String::as_str).collect();
            observation
                .sources
                .retain(|source| !withdrawn_set.contains(source.candidate_id.as_str()));
        }
        let pulls = self
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
        let check_standing = self.dispatch_check_standing(practice, workspace).await?;
        let memory_standing = self.active_memory_standing(practice, workspace).await?;

        let open_concerns = self
            .load_open_concern_summaries(practice, workspace)
            .await?;

        let completed_jobs = practice_job::Entity::find()
            .filter(practice_job::Column::Practice.eq(practice))
            .filter(practice_job::Column::Workspace.eq(workspace))
            .filter(practice_job::Column::Status.eq(WorkStatus::Completed))
            .order_by_desc(practice_job::Column::UpdatedAt)
            .limit(5)
            .all(&self.db)
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

        let understanding = self
            .patient_understanding_internal(practice, workspace)
            .await?;

        Ok(PatientStartingContext {
            workspace: workspace.to_owned(),
            patient,
            chart_version: observation.version,
            chart_record_count: observation.sources.len(),
            source_pulls,
            check_standing,
            understanding,
            withdrawal_boundary,
            memory_standing,
            open_concerns,
            recent_work,
        })
    }

    /// Executes `retrieve_patient_work` (`list` or `read_background_job`) for a
    /// practice background job, including saved briefings for `(practice, workspace)`.
    pub(super) async fn execute_practice_retrieve_work_tool(
        &self,
        scope: &PracticeScope,
        arguments: &Value,
    ) -> Result<ToolOutcome, engine::ProductError> {
        let operation = arguments
            .get("operation")
            .and_then(Value::as_str)
            .unwrap_or("");
        match operation {
            "list" => {
                let cap = arguments
                    .get("limit")
                    .and_then(Value::as_u64)
                    .and_then(|n| usize::try_from(n).ok())
                    .unwrap_or(10)
                    .clamp(1, MAX_RETRIEVED_WORK_ITEMS);
                let jobs = practice_job::Entity::find()
                    .filter(practice_job::Column::Practice.eq(&scope.practice))
                    .filter(practice_job::Column::Workspace.eq(&scope.workspace))
                    .filter(practice_job::Column::Status.eq(WorkStatus::Completed))
                    .order_by_desc(practice_job::Column::UpdatedAt)
                    .limit(cap as u64)
                    .all(&self.db)
                    .await
                    .map_err(persistence_failure)?;
                let briefings = visit_briefing::Entity::find()
                    .filter(visit_briefing::Column::Practice.eq(&scope.practice))
                    .filter(visit_briefing::Column::Workspace.eq(&scope.workspace))
                    .order_by_desc(visit_briefing::Column::StartsAtUnix)
                    .limit(cap as u64)
                    .all(&self.db)
                    .await
                    .map_err(persistence_failure)?;
                let background_jobs: Vec<_> = jobs
                    .into_iter()
                    .map(|job| {
                        json!({
                            "job_id": job.job_id,
                            "kind": job.kind.as_str(),
                            "sources_version": job.sources_version,
                            "updated_at": job.updated_at,
                        })
                    })
                    .collect();
                let saved_briefings: Vec<_> = briefings
                    .into_iter()
                    .map(|row| {
                        json!({
                            "briefing_id": row.briefing_id,
                            "appointment_id": row.appointment_id,
                            "starts_at": row.starts_at,
                            "prepared_at": row.prepared_at,
                            "source_version": row.source_version,
                            "visit_standing": row.visit_standing,
                            "visit_card_orientation": row.visit_card_orientation,
                        })
                    })
                    .collect();
                Ok(ToolOutcome::Returned(
                    json!({
                        "outcome": "available",
                        "background_jobs": background_jobs,
                        "saved_briefings": saved_briefings,
                    })
                    .to_string(),
                ))
            }
            "read_background_job" => {
                let Some(job_id) = arguments
                    .get("job_id")
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                else {
                    return Ok(ToolOutcome::refused("Supply a non-empty `job_id`."));
                };
                let target_scope = PracticeScope {
                    practice: scope.practice.clone(),
                    workspace: scope.workspace.clone(),
                    job_id: job_id.to_owned(),
                };
                let Some(job) = self
                    .practice_job(&target_scope)
                    .await
                    .map_err(persistence_failure)?
                else {
                    return Ok(ToolOutcome::Returned(
                        json!({
                            "outcome": "unavailable",
                            "limits": [{
                                "kind": "missing",
                                "subject": job_id,
                                "detail": format!("Background job `{job_id}` was not found in this patient workspace."),
                            }],
                        })
                        .to_string(),
                    ));
                };
                Ok(ToolOutcome::Returned(
                    json!({
                        "outcome": "available",
                        "job_id": job.scope.job_id,
                        "kind": job.kind.as_str(),
                        "status": job.status,
                        "sources_version": job.sources_version,
                        "result": job.result,
                    })
                    .to_string(),
                ))
            }
            "read_conversation" => Ok(ToolOutcome::refused(
                "practice background job has no physician session and cannot read private physician conversations",
            )),
            _ => Ok(ToolOutcome::refused(
                "Supply `operation` as `list` or `read_background_job`.",
            )),
        }
    }

    /// Executes `search_clinical_library` or `read_clinical_library` for a
    /// practice background job, attaching structured `unavailable` limits when
    /// the clinical library service cannot be reached.
    pub(super) async fn execute_practice_clinical_library_tool(
        &self,
        name: &str,
        arguments: &Value,
    ) -> Result<ToolOutcome, engine::ProductError> {
        let outcome = self.clinical_library().execute(name, arguments).await;
        if let ToolOutcome::Returned(raw) = &outcome
            && let Ok(mut parsed) = serde_json::from_str::<Value>(raw)
            && parsed.get("outcome").and_then(Value::as_str) == Some("unavailable")
        {
            let detail = parsed
                .get("reason")
                .or_else(|| parsed.get("message"))
                .and_then(Value::as_str)
                .unwrap_or("Clinical library service is unavailable.")
                .to_owned();
            let limit = RecordLimit {
                kind: RecordLimitKind::Unavailable,
                subject: "clinical_library".into(),
                detail,
            };
            if let Some(obj) = parsed.as_object_mut() {
                obj.insert("limits".into(), serde_json::to_value([limit])?);
            }
            return Ok(ToolOutcome::Returned(parsed.to_string()));
        }
        Ok(outcome)
    }

    /// Reconciles a previously saved `select_visit_preparations` or
    /// `save_visit_briefing` receipt when recovering an interrupted attempt.
    pub(super) async fn saved_visit_preparation_outcome(
        &self,
        claim: &PracticeJobClaim,
        call: &Invocation,
    ) -> Result<Option<ToolOutcome>, engine::ProductError> {
        self.assert_practice_job_current(claim).await?;
        let (job, _, _) = self.practice_job_snapshot(&claim.scope).await?;
        let idempotency_key = format!("{}:{}", job.work, call.id.trim());

        if call.name == SELECT_VISITS_TOOL {
            let rows = visit_preparation::Entity::find()
                .filter(visit_preparation::Column::Practice.eq(&claim.scope.practice))
                .filter(visit_preparation::Column::SelectionIdempotencyKey.eq(&idempotency_key))
                .order_by_asc(visit_preparation::Column::StartsAtUnix)
                .order_by_asc(visit_preparation::Column::AppointmentId)
                .all(&self.db)
                .await
                .map_err(persistence_failure)?;
            if rows.is_empty() {
                return Ok(None);
            }
            let saved: Vec<_> = rows
                .into_iter()
                .map(|row| {
                    json!({
                        "source_id": row.source_id,
                        "appointment_id": row.appointment_id,
                        "starts_at_unix": row.starts_at_unix,
                        "workspace": row.workspace,
                        "state": row.state,
                    })
                })
                .collect();
            return Ok(Some(ToolOutcome::Returned(
                json!({
                    "outcome": "saved",
                    "saved_decisions": saved.len(),
                    "decisions": saved,
                })
                .to_string(),
            )));
        }

        if call.name == SAVE_BRIEFING_TOOL {
            let existing = visit_briefing::Entity::find()
                .filter(visit_briefing::Column::Practice.eq(&claim.scope.practice))
                .filter(visit_briefing::Column::Workspace.eq(&claim.scope.workspace))
                .filter(visit_briefing::Column::IdempotencyKey.eq(&idempotency_key))
                .one(&self.db)
                .await
                .map_err(persistence_failure)?;
            let Some(row) = existing else {
                return Ok(None);
            };
            let decoded = decode_briefing(row).map_err(persistence_failure)?;
            return Ok(Some(ToolOutcome::Returned(
                json!({
                    "outcome": "saved",
                    "briefing_id": decoded.briefing_id,
                    "appointment_id": decoded.slot.appointment_id,
                    "starts_at": decoded.starts_at,
                    "prepared_at": decoded.prepared_at,
                    "source_version": decoded.source_version,
                    "citation_count": decoded.citations.len(),
                    "raised_concern_id": decoded.raised_concern_id,
                })
                .to_string(),
            )));
        }

        Ok(None)
    }

    /// Executes `select_visit_preparations` or `save_visit_briefing` inside a
    /// fenced `PracticeJobHost` attempt.
    pub(super) async fn execute_visit_preparation_tool_fenced(
        &self,
        claim: &PracticeJobClaim,
        call: &Invocation,
        arguments: &Value,
    ) -> Result<ToolOutcome, engine::ProductError> {
        self.assert_practice_job_current(claim).await?;
        match call.name.as_str() {
            SELECT_VISITS_TOOL => {
                self.execute_select_visits_fenced(claim, call, arguments)
                    .await
            }
            SAVE_BRIEFING_TOOL => {
                self.execute_save_briefing_fenced(claim, call, arguments)
                    .await
            }
            other => Ok(ToolOutcome::refused(format!(
                "unknown visit preparation tool `{other}`"
            ))),
        }
    }

    async fn execute_select_visits_fenced(
        &self,
        claim: &PracticeJobClaim,
        call: &Invocation,
        arguments: &Value,
    ) -> Result<ToolOutcome, engine::ProductError> {
        let parsed = match SelectVisitsArgs::parse(arguments) {
            Ok(parsed) => parsed,
            Err(err) => {
                return Ok(ToolOutcome::refused(
                    json!({
                        "outcome": "invalid_arguments",
                        "message": err.to_string(),
                    })
                    .to_string(),
                ));
            }
        };

        let (job, _, _) = self
            .practice_job_snapshot(&claim.scope)
            .await
            .map_err(persistence_failure)?;
        let Some(selection_ctx) = job.assignment.visit_selection.as_ref() else {
            return Ok(ToolOutcome::refused(
                "select_visit_preparations is only available to visit selection jobs",
            ));
        };

        let candidate_by_slot: BTreeMap<(&str, &str, i64), &CandidateVisitBrief> = selection_ctx
            .candidates
            .iter()
            .map(|cand| {
                (
                    (
                        cand.source_id.as_str(),
                        cand.appointment_id.as_str(),
                        cand.starts_at_unix,
                    ),
                    cand,
                )
            })
            .collect();

        let tx = self.db.begin().await.map_err(persistence_failure)?;
        let db_clock = now(&tx).await.map_err(persistence_failure)?;
        let effective_now = selection_ctx.evaluated_at_unix + (db_clock - job.created_at).max(0);
        let idempotency_key = format!("{}:{}", job.work, call.id.trim());

        let command_body = json!({
            "kind": "select_visit_preparations",
            "practice": &claim.scope.practice,
            "work_id": &job.work,
            "decisions": &parsed.decisions,
        });
        match receipt::record(
            &tx,
            &claim.scope.practice,
            PRACTICE_OWNER,
            &idempotency_key,
            &command_body,
        )
        .await
        .map_err(persistence_failure)?
        {
            Receipt::New => {}
            Receipt::Same => {
                tx.commit().await.map_err(persistence_failure)?;
                if let Some(saved) = self.saved_visit_preparation_outcome(claim, call).await? {
                    return Ok(saved);
                }
                return Ok(ToolOutcome::Returned(
                    json!({ "outcome": "saved", "duplicate": true }).to_string(),
                ));
            }
            Receipt::Different => {
                return Ok(ToolOutcome::refused(
                    "conflicting select_visit_preparations payload for same call idempotency key",
                ));
            }
        }

        let mut saved_summaries = Vec::with_capacity(parsed.decisions.len());
        for item in &parsed.decisions {
            let Some(cand) = candidate_by_slot
                .get(&(
                    item.source_id.as_str(),
                    item.appointment_id.as_str(),
                    item.starts_at_unix,
                ))
                .copied()
            else {
                return Ok(ToolOutcome::refused(
                    json!({
                        "outcome": "unknown_candidate_visit",
                        "source_id": item.source_id,
                        "appointment_id": item.appointment_id,
                        "starts_at_unix": item.starts_at_unix,
                        "message": "Every selected visit must match a candidate visit in the starting context.",
                    })
                    .to_string(),
                ));
            };

            let appt_row = schedule_appointment::Entity::find_by_id((
                claim.scope.practice.clone(),
                item.source_id.clone(),
                item.appointment_id.clone(),
            ))
            .lock_exclusive()
            .one(&tx)
            .await
            .map_err(persistence_failure)?;

            let Some(appt) = appt_row else {
                return Ok(ToolOutcome::refused(
                    json!({
                        "outcome": "appointment_missing",
                        "appointment_id": item.appointment_id,
                        "message": "Appointment is no longer present on the practice schedule.",
                    })
                    .to_string(),
                ));
            };

            if appt.status != AppointmentStatus::Scheduled
                || appt.starts_at_unix != item.starts_at_unix
            {
                return Ok(ToolOutcome::refused(
                    json!({
                        "outcome": "appointment_changed",
                        "appointment_id": item.appointment_id,
                        "current_status": appt.status.as_str(),
                        "current_starts_at_unix": appt.starts_at_unix,
                        "message": "Appointment was cancelled or moved; do not schedule preparation for the superseded time.",
                    })
                    .to_string(),
                ));
            }

            if !visit_can_start_preparation(appt.status, appt.ends_at_unix, effective_now)
                && !matches!(item.decision, VisitPreparationDecision::Skip { .. })
            {
                return Ok(ToolOutcome::refused(
                    json!({
                        "outcome": "visit_already_ended",
                        "appointment_id": item.appointment_id,
                        "ends_at_unix": appt.ends_at_unix,
                        "now_unix": effective_now,
                        "message": "No preparation may start or be scheduled at or after ends_at_unix.",
                    })
                    .to_string(),
                ));
            }

            let state_str = match &item.decision {
                VisitPreparationDecision::PrepareNow { .. } => "scheduled",
                VisitPreparationDecision::Schedule {
                    prepare_at_unix, ..
                } => {
                    if *prepare_at_unix >= appt.ends_at_unix {
                        return Ok(ToolOutcome::refused(
                            json!({
                                "outcome": "prepare_time_after_visit_end",
                                "appointment_id": item.appointment_id,
                                "prepare_at_unix": prepare_at_unix,
                                "ends_at_unix": appt.ends_at_unix,
                                "message": "prepare_at_unix must be strictly earlier than ends_at_unix.",
                            })
                            .to_string(),
                        ));
                    }
                    "scheduled"
                }
                VisitPreparationDecision::Defer {
                    check_again_at_unix,
                    ..
                } => {
                    if *check_again_at_unix >= appt.ends_at_unix {
                        return Ok(ToolOutcome::refused(
                            json!({
                                "outcome": "defer_time_after_visit_end",
                                "appointment_id": item.appointment_id,
                                "check_again_at_unix": check_again_at_unix,
                                "ends_at_unix": appt.ends_at_unix,
                                "message": "check_again_at_unix must be strictly earlier than ends_at_unix.",
                            })
                            .to_string(),
                        ));
                    }
                    "deferred"
                }
                VisitPreparationDecision::Skip { .. } => "skipped",
            };

            let slot_pk = (
                claim.scope.practice.clone(),
                item.source_id.clone(),
                item.appointment_id.clone(),
                item.starts_at_unix,
            );
            let existing_prep = visit_preparation::Entity::find_by_id(slot_pk.clone())
                .lock_exclusive()
                .one(&tx)
                .await
                .map_err(persistence_failure)?;
            let existing_current_briefing = visit_briefing::Entity::find_by_id(slot_pk)
                .one(&tx)
                .await
                .map_err(persistence_failure)?
                .filter(|b| b.visit_standing == "current");

            if let Some(saved_briefing) = existing_current_briefing.as_ref() {
                return Ok(ToolOutcome::refused(
                    json!({
                        "outcome": "briefing_already_prepared",
                        "appointment_id": item.appointment_id,
                        "briefing_id": saved_briefing.briefing_id,
                        "message": "A completed briefing already exists for this visit time. Later chart updates must not rewrite a finished briefing; handle them through a regular patient review.",
                    })
                    .to_string(),
                ));
            }

            let (final_state_str, preserved_briefing_job_id, preserved_briefing_id) = (
                state_str,
                None,
                existing_prep
                    .as_ref()
                    .and_then(|prep| prep.briefing_id.clone()),
            );

            let obs = sources::observed(&tx, &cand.workspace)
                .await
                .map_err(persistence_failure)?;
            let decision_json = serde_json::to_value(&item.decision)
                .map_err(conversion_error("VisitPreparationDecision", "json"))
                .map_err(persistence_failure)?;
            let obs_version_i64 = i64::try_from(obs.version)
                .map_err(conversion_error("observed_source_version", "i64"))
                .map_err(persistence_failure)?;

            visit_preparation::Entity::insert(visit_preparation::ActiveModel {
                practice: Set(claim.scope.practice.clone()),
                source_id: Set(item.source_id.clone()),
                appointment_id: Set(item.appointment_id.clone()),
                starts_at_unix: Set(item.starts_at_unix),
                workspace: Set(cand.workspace.clone()),
                ends_at_unix: Set(appt.ends_at_unix),
                appointment_saved_version: Set(appt.saved_version),
                observed_source_version: Set(obs_version_i64),
                decision: Set(decision_json),
                decided_at: Set(effective_now),
                selection_job_id: Set(claim.scope.job_id.clone()),
                selection_work_id: Set(job.work.clone()),
                selection_idempotency_key: Set(idempotency_key.clone()),
                state: Set(final_state_str.to_owned()),
                briefing_job_id: Set(preserved_briefing_job_id),
                briefing_id: Set(preserved_briefing_id),
                failed_reason: Set(None),
                updated_at: Set(effective_now),
            })
            .on_conflict(
                OnConflict::columns([
                    visit_preparation::Column::Practice,
                    visit_preparation::Column::SourceId,
                    visit_preparation::Column::AppointmentId,
                    visit_preparation::Column::StartsAtUnix,
                ])
                .update_columns([
                    visit_preparation::Column::Workspace,
                    visit_preparation::Column::EndsAtUnix,
                    visit_preparation::Column::AppointmentSavedVersion,
                    visit_preparation::Column::ObservedSourceVersion,
                    visit_preparation::Column::Decision,
                    visit_preparation::Column::DecidedAt,
                    visit_preparation::Column::SelectionJobId,
                    visit_preparation::Column::SelectionWorkId,
                    visit_preparation::Column::SelectionIdempotencyKey,
                    visit_preparation::Column::State,
                    visit_preparation::Column::BriefingJobId,
                    visit_preparation::Column::BriefingId,
                    visit_preparation::Column::FailedReason,
                    visit_preparation::Column::UpdatedAt,
                ])
                .to_owned(),
            )
            .exec_without_returning(&tx)
            .await
            .map_err(persistence_failure)?;

            saved_summaries.push(json!({
                "source_id": item.source_id,
                "appointment_id": item.appointment_id,
                "starts_at_unix": item.starts_at_unix,
                "workspace": cand.workspace,
                "action": item.decision.action_str(),
                "state": final_state_str,
            }));
        }

        tx.commit().await.map_err(persistence_failure)?;
        Ok(ToolOutcome::Returned(
            json!({
                "outcome": "saved",
                "saved_decisions": saved_summaries.len(),
                "decisions": saved_summaries,
            })
            .to_string(),
        ))
    }

    async fn execute_save_briefing_fenced(
        &self,
        claim: &PracticeJobClaim,
        call: &Invocation,
        arguments: &Value,
    ) -> Result<ToolOutcome, engine::ProductError> {
        let parsed = match SaveBriefingArgs::parse(arguments) {
            Ok(parsed) => parsed,
            Err(VisitPreparationInputError::RecommendationPresentedAsAcceptedCare { phrase }) => {
                return Ok(ToolOutcome::refused(
                    json!({
                        "outcome": "accepted_care_phrasing_refused",
                        "phrase": phrase,
                        "message": format!(
                            "Prepared visit decisions must never present recommendations as accepted or executed care (`{phrase}`). Frame recommendations strictly as options or open questions for the physician."
                        ),
                    })
                    .to_string(),
                ));
            }
            Err(err) => {
                return Ok(ToolOutcome::refused(
                    json!({
                        "outcome": "invalid_arguments",
                        "message": err.to_string(),
                    })
                    .to_string(),
                ));
            }
        };

        let (job, ordered, _) = self
            .practice_job_snapshot(&claim.scope)
            .await
            .map_err(persistence_failure)?;
        let Some(briefing_ctx) = job.assignment.visit_briefing.as_ref() else {
            return Ok(ToolOutcome::refused(
                "save_visit_briefing is only available to single-visit briefing preparation jobs",
            ));
        };

        let idempotency_key = format!("{}:{}", job.work, call.id.trim());
        if let Some(already_saved) = self.saved_visit_preparation_outcome(claim, call).await? {
            return Ok(already_saved);
        }

        let read_in_work = crate::physician_sources::delivered_exact_revisions(
            ordered.iter(),
            &job.message_sources(),
        )
        .map_err(|err| engine::Cause::from(error(&err)))?;

        let source_freshness = self
            .workspace_source_refresh_summaries(&claim.scope.practice, &claim.scope.workspace)
            .await
            .map_err(persistence_failure)?;
        let pulls: Vec<SourcePull> = self
            .source_pulls(std::slice::from_ref(&claim.scope.workspace))
            .await
            .map_err(persistence_failure)?
            .remove(&claim.scope.workspace)
            .unwrap_or_default();
        let existing_understanding = self
            .patient_understanding_internal(&claim.scope.practice, &claim.scope.workspace)
            .await
            .map_err(persistence_failure)?;
        let carried_forward = existing_understanding
            .as_ref()
            .map(known_understanding_pairs)
            .unwrap_or_default();

        let tx = self.db.begin().await.map_err(persistence_failure)?;
        let db_clock = now(&tx).await.map_err(persistence_failure)?;
        let prepared_at = match briefing_ctx.queued_at_unix {
            Some(queued) => queued + (db_clock - job.created_at).max(0),
            None => db_clock,
        };

        // Re-check idempotency inside the write transaction.
        if let Some(existing_by_key) = visit_briefing::Entity::find()
            .filter(visit_briefing::Column::Practice.eq(&claim.scope.practice))
            .filter(visit_briefing::Column::Workspace.eq(&claim.scope.workspace))
            .filter(visit_briefing::Column::IdempotencyKey.eq(&idempotency_key))
            .one(&tx)
            .await
            .map_err(persistence_failure)?
        {
            let decoded = decode_briefing(existing_by_key).map_err(persistence_failure)?;
            tx.commit().await.map_err(persistence_failure)?;
            return Ok(ToolOutcome::Returned(
                json!({
                    "outcome": "saved",
                    "briefing_id": decoded.briefing_id,
                    "appointment_id": decoded.slot.appointment_id,
                    "starts_at": decoded.starts_at,
                    "prepared_at": decoded.prepared_at,
                    "source_version": decoded.source_version,
                    "citation_count": decoded.citations.len(),
                    "raised_concern_id": decoded.raised_concern_id,
                })
                .to_string(),
            ));
        }

        // Verify the appointment is still scheduled at its target visit time.
        let appt_row = schedule_appointment::Entity::find_by_id((
            claim.scope.practice.clone(),
            briefing_ctx.source_id.clone(),
            briefing_ctx.appointment_id.clone(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await
        .map_err(persistence_failure)?;

        let Some(appt) = appt_row else {
            return Ok(ToolOutcome::refused(
                json!({
                    "outcome": "appointment_cancelled",
                    "appointment_id": briefing_ctx.appointment_id,
                    "message": "Appointment was withdrawn from the practice schedule before preparation completed. Nothing was saved.",
                })
                .to_string(),
            ));
        };
        if appt.status != AppointmentStatus::Scheduled
            || appt.starts_at_unix != briefing_ctx.starts_at_unix
            || appt.workspace.as_deref() != Some(claim.scope.workspace.as_str())
        {
            return Ok(ToolOutcome::refused(
                json!({
                    "outcome": "appointment_changed",
                    "appointment_id": briefing_ctx.appointment_id,
                    "target_starts_at_unix": briefing_ctx.starts_at_unix,
                    "current_status": appt.status.as_str(),
                    "current_starts_at": appt.starts_at,
                    "current_starts_at_unix": appt.starts_at_unix,
                    "message": "Appointment was cancelled or rescheduled while preparation was running; unfinished preparation for the superseded visit time ends without saving.",
                })
                .to_string(),
            ));
        }

        // Mid-run record change detection: refuse stale completion and return
        // any newly arrived, modified/re-corrected, or withdrawn chart records!
        let observation = sources::observed(&tx, &claim.scope.workspace)
            .await
            .map_err(persistence_failure)?;
        let known_revisions =
            model_chart_knowledge(&job.sources, &ordered, &job.message_sources(), false)
                .map_err(|err| engine::Cause::from(error(&err)))?;
        if let Some(feedback) = briefing_source_change_feedback(
            BriefingFenceStage::SaveTool,
            job.sources_version,
            &known_revisions,
            observation.version,
            &observation.sources,
        ) {
            return Ok(ToolOutcome::refused(feedback.to_string()));
        }

        let by_candidate: BTreeMap<&str, &SourceRevision> = observation
            .sources
            .iter()
            .map(|src| (src.candidate_id.as_str(), src))
            .collect();

        let mut ordered_refs: Vec<ExactRecordReference> = Vec::new();
        let mut seen_refs: BTreeSet<(String, String)> = BTreeSet::new();
        let mut collect_refs = |refs: &[ExactRecordReference]| -> Result<(), Value> {
            for reference in refs {
                verify_briefing_reference(
                    reference,
                    &by_candidate,
                    &read_in_work,
                    &carried_forward,
                )?;
                let key = (reference.candidate_id.clone(), reference.revision.clone());
                if seen_refs.insert(key) {
                    ordered_refs.push(reference.clone());
                }
            }
            Ok(())
        };

        for claim_item in parsed
            .trajectory
            .iter()
            .chain(parsed.meaningful_changes.iter())
            .chain(parsed.unresolved_issues.iter())
        {
            if let Err(refusal) = collect_refs(&claim_item.citations) {
                return Ok(ToolOutcome::refused(refusal.to_string()));
            }
        }
        for decision_item in &parsed.visit_decisions {
            if let Err(refusal) = collect_refs(&decision_item.citations) {
                return Ok(ToolOutcome::refused(refusal.to_string()));
            }
        }
        if let Err(refusal) = collect_refs(&parsed.basis_citations) {
            return Ok(ToolOutcome::refused(refusal.to_string()));
        }
        if let Some(concern_input) = &parsed.visit_concern
            && let Err(refusal) = collect_refs(&concern_input.records)
        {
            return Ok(ToolOutcome::refused(refusal.to_string()));
        }

        if !observation.sources.is_empty() && ordered_refs.is_empty() {
            return Ok(ToolOutcome::refused(
                json!({
                    "outcome": "missing_citations",
                    "message": "Workspace chart records are available; cite exact record revisions verified via read_patient_sources on your briefing claims and visit decisions. Nothing was saved.",
                })
                .to_string(),
            ));
        }

        let citations: Vec<SummaryCitation> = ordered_refs
            .iter()
            .map(|reference| {
                let source = by_candidate[reference.candidate_id.as_str()];
                build_summary_citation(&claim.scope.workspace, reference, source, &pulls)
            })
            .collect();

        let source_versions: BTreeMap<String, String> = observation
            .sources
            .iter()
            .map(|src| (src.candidate_id.clone(), src.revision.clone()))
            .collect();

        // Ensure failed source refreshes are always represented truthfully in `limits`.
        let mut limits = parsed.limits.clone();
        for status in &source_freshness {
            if let Some(reason) = &status.failure_reason {
                let already_noted = limits.iter().any(|lim| {
                    lim.kind == RecordLimitKind::Unavailable && lim.subject == status.source_id
                });
                if !already_noted {
                    let detail = match status.last_good_pulled_at {
                        Some(last_ok) => format!(
                            "Refresh of `{}` ({}) failed ({reason}); prepared from records retained as of {}.",
                            status.source_label,
                            status.source_id,
                            format_unix_rfc3339(last_ok)
                        ),
                        None => format!(
                            "Refresh of `{}` ({}) failed ({reason}) with no prior pull timestamp.",
                            status.source_label, status.source_id
                        ),
                    };
                    limits.push(RecordLimit {
                        kind: RecordLimitKind::Unavailable,
                        subject: status.source_id.clone(),
                        detail,
                    });
                }
            }
        }

        let slot = VisitSlotKey {
            practice: claim.scope.practice.clone(),
            source_id: briefing_ctx.source_id.clone(),
            appointment_id: briefing_ctx.appointment_id.clone(),
            starts_at_unix: briefing_ctx.starts_at_unix,
        };
        let existing_slot_briefing = visit_briefing::Entity::find_by_id(slot.pk_tuple())
            .lock_exclusive()
            .one(&tx)
            .await
            .map_err(persistence_failure)?;
        if let Some(existing_row) = existing_slot_briefing.as_ref() {
            let existing_origin: BriefingOrigin =
                serde_json::from_value(existing_row.origin.clone())
                    .map_err(conversion_error("visit_briefing.origin", "BriefingOrigin"))
                    .map_err(persistence_failure)?;
            if !slot_allows_briefing_write(Some(&existing_origin.work_id), &job.work) {
                return Ok(ToolOutcome::refused(
                    json!({
                        "outcome": "briefing_immutable_after_completion",
                        "briefing_id": existing_row.briefing_id,
                        "appointment_id": briefing_ctx.appointment_id,
                        "message": "A completed briefing already exists for this visit time and cannot be rewritten by a later job. Later chart updates are handled by regular patient reviews.",
                    })
                    .to_string(),
                ));
            }
        }
        let briefing_id = existing_slot_briefing
            .as_ref()
            .map(|row| row.briefing_id.clone())
            .unwrap_or_else(|| slot.briefing_id());
        let prior_concern_id = existing_slot_briefing
            .as_ref()
            .and_then(|row| row.raised_concern_id.as_deref())
            .map(ConcernId::new)
            .transpose()
            .map_err(|err| {
                persistence_failure(conversion_error(
                    "visit_briefing.raised_concern_id",
                    "ConcernId",
                )(err))
            })?;

        // Settle optional pre-visit concern atomically in the same transaction.
        let source_work = SourceWork::new("visit_preparation", &job.work, &claim.attempt)
            .map_err(|err| engine::ProductError::from(Box::new(err) as engine::Cause))?;
        let raised_concern_id = match &parsed.visit_concern {
            Some(concern_input) => {
                if let Some(prior_id) = &prior_concern_id
                    && prior_id != &concern_input.concern_id
                {
                    let retire_key =
                        CommandKey::new(format!("vc-supersede:{}:{}", job.work, call.id.trim()))
                            .map_err(|err| {
                                engine::ProductError::from(Box::new(err) as engine::Cause)
                            })?;
                    let reason = RetirementReason::new(
                        "Superseded by updated pre-visit concern during briefing revision.",
                    )
                    .map_err(|err| engine::ProductError::from(Box::new(err) as engine::Cause))?;
                    match retire_concern_in_tx(
                        &tx,
                        &retire_key,
                        &RetireConcern {
                            practice: claim.scope.practice.clone(),
                            workspace: claim.scope.workspace.clone(),
                            id: prior_id.clone(),
                            reason,
                            source_work: source_work.clone(),
                        },
                    )
                    .await
                    {
                        Ok(_)
                        | Err(ConcernError::NotOpen { .. } | ConcernError::NotFound { .. }) => {}
                        Err(ConcernError::Database(db)) => return Err(persistence_failure(db)),
                        Err(other) => {
                            return Err(engine::ProductError::from(
                                Box::new(other) as engine::Cause
                            ));
                        }
                    }
                }

                let existing_concern = load_concern(
                    &tx,
                    &claim.scope.practice,
                    &claim.scope.workspace,
                    &concern_input.concern_id,
                )
                .await
                .map_err(persistence_failure)?;

                if existing_concern.is_none() {
                    let raise_key =
                        CommandKey::new(format!("vc-raise:{}:{}", job.work, call.id.trim()))
                            .map_err(|err| {
                                engine::ProductError::from(Box::new(err) as engine::Cause)
                            })?;
                    match raise_concern_in_tx(
                        &tx,
                        &raise_key,
                        &RaiseConcern {
                            practice: claim.scope.practice.clone(),
                            workspace: claim.scope.workspace.clone(),
                            id: concern_input.concern_id.clone(),
                            urgency: concern_input.urgency,
                            reason: concern_input.reason.clone(),
                            records: concern_input.records.clone(),
                            source_work: source_work.clone(),
                        },
                    )
                    .await
                    {
                        Ok(_) => {}
                        Err(ConcernError::Database(db)) => return Err(persistence_failure(db)),
                        Err(other) => {
                            return Ok(ToolOutcome::refused(
                                json!({
                                    "outcome": "invalid_visit_concern",
                                    "message": other.to_string(),
                                })
                                .to_string(),
                            ));
                        }
                    }
                } else if prior_concern_id.as_ref() == Some(&concern_input.concern_id) {
                    // Same slot updated its own raised concern: only valid if still Open!
                    let locked_row = concern::Entity::find_by_id((
                        claim.scope.practice.clone(),
                        claim.scope.workspace.clone(),
                        concern_input.concern_id.as_str().to_owned(),
                    ))
                    .lock_exclusive()
                    .one(&tx)
                    .await
                    .map_err(persistence_failure)?;
                    match locked_row {
                        Some(row) if row.status == "open" => {
                            let records_json = serde_json::to_value(&concern_input.records)
                                .map_err(conversion_error("ExactRecordReference", "json"))
                                .map_err(persistence_failure)?;
                            let work_json = serde_json::to_value(&source_work)
                                .map_err(conversion_error("SourceWork", "json"))
                                .map_err(persistence_failure)?;
                            let next_rev = row.revision + 1;
                            concern::ActiveModel {
                                practice: Unchanged(row.practice),
                                workspace: Unchanged(row.workspace),
                                id: Unchanged(row.id),
                                urgency: Set(concern_input.urgency.as_str().to_owned()),
                                status: Unchanged(row.status),
                                reason: Set(concern_input.reason.clone()),
                                records: Set(records_json),
                                source_work: Set(work_json),
                                cleared_by_physician: Unchanged(row.cleared_by_physician),
                                cleared_at: Unchanged(row.cleared_at),
                                feedback_revision: Unchanged(row.feedback_revision),
                                retirement_reason: Unchanged(row.retirement_reason),
                                retired_by_work: Unchanged(row.retired_by_work),
                                retired_at: Unchanged(row.retired_at),
                                created_at: Unchanged(row.created_at),
                                updated_at: Set(prepared_at),
                                revision: Set(next_rev),
                            }
                            .update(&tx)
                            .await
                            .map_err(persistence_failure)?;
                        }
                        Some(row) => {
                            return Ok(ToolOutcome::refused(
                                json!({
                                    "outcome": "concern_already_closed",
                                    "concern_id": concern_input.concern_id,
                                    "current_status": row.status,
                                    "message": format!(
                                        "Earlier pre-visit concern `{}` is already `{}` and cannot be reopened under the same id; choose a fresh concern_id for this new pre-visit finding.",
                                        concern_input.concern_id, row.status
                                    ),
                                })
                                .to_string(),
                            ));
                        }
                        None => {
                            return Ok(ToolOutcome::refused(
                                json!({
                                    "outcome": "concern_already_closed",
                                    "concern_id": concern_input.concern_id,
                                    "message": format!(
                                        "Earlier pre-visit concern `{}` is no longer open; choose a fresh concern_id for this new pre-visit finding.",
                                        concern_input.concern_id
                                    ),
                                })
                                .to_string(),
                            ));
                        }
                    }
                } else {
                    return Ok(ToolOutcome::refused(
                        json!({
                            "outcome": "concern_already_exists",
                            "concern_id": concern_input.concern_id,
                            "message": format!(
                                "Concern `{}` already exists in this patient workspace; choose a distinct concern_id for this visit concern.",
                                concern_input.concern_id
                            ),
                        })
                        .to_string(),
                    ));
                }

                visit_concern::Entity::insert(visit_concern::ActiveModel {
                    practice: Set(claim.scope.practice.clone()),
                    workspace: Set(claim.scope.workspace.clone()),
                    concern_id: Set(concern_input.concern_id.as_str().to_owned()),
                    source_id: Set(briefing_ctx.source_id.clone()),
                    appointment_id: Set(briefing_ctx.appointment_id.clone()),
                    starts_at_unix: Set(briefing_ctx.starts_at_unix),
                    ends_at_unix: Set(appt.ends_at_unix),
                    urgency: Set(concern_input.urgency.as_str().to_owned()),
                    opened_at: Set(None),
                    opened_by_physician: Set(None),
                    created_at: Set(prepared_at),
                    updated_at: Set(prepared_at),
                })
                .on_conflict(
                    OnConflict::columns([
                        visit_concern::Column::Practice,
                        visit_concern::Column::Workspace,
                        visit_concern::Column::ConcernId,
                    ])
                    .update_columns([
                        visit_concern::Column::SourceId,
                        visit_concern::Column::AppointmentId,
                        visit_concern::Column::StartsAtUnix,
                        visit_concern::Column::EndsAtUnix,
                        visit_concern::Column::Urgency,
                        visit_concern::Column::UpdatedAt,
                    ])
                    .to_owned(),
                )
                .exec_without_returning(&tx)
                .await
                .map_err(persistence_failure)?;

                // Enforce lifecycle immediately if the visit already reached ends_at_unix while running.
                if should_retire_visit_concern(
                    concern_input.urgency,
                    false,
                    prepared_at >= appt.ends_at_unix,
                ) {
                    let retire_key = CommandKey::new(format!(
                        "vc-ended-at-save:{}:{}",
                        job.work,
                        call.id.trim()
                    ))
                    .map_err(|err| engine::ProductError::from(Box::new(err) as engine::Cause))?;
                    let reason = RetirementReason::new(format!(
                        "Scheduled visit {} ended at {}.",
                        briefing_ctx.appointment_id,
                        format_unix_rfc3339(appt.ends_at_unix)
                    ))
                    .map_err(|err| engine::ProductError::from(Box::new(err) as engine::Cause))?;
                    match retire_concern_in_tx(
                        &tx,
                        &retire_key,
                        &RetireConcern {
                            practice: claim.scope.practice.clone(),
                            workspace: claim.scope.workspace.clone(),
                            id: concern_input.concern_id.clone(),
                            reason,
                            source_work: source_work.clone(),
                        },
                    )
                    .await
                    {
                        Ok(_)
                        | Err(ConcernError::NotOpen { .. } | ConcernError::NotFound { .. }) => {}
                        Err(ConcernError::Database(db)) => return Err(persistence_failure(db)),
                        Err(other) => {
                            return Err(engine::ProductError::from(
                                Box::new(other) as engine::Cause
                            ));
                        }
                    }
                }

                Some(concern_input.concern_id.clone())
            }
            None => {
                if let Some(prior_id) = &prior_concern_id {
                    let retire_key =
                        CommandKey::new(format!("vc-resolved:{}:{}", job.work, call.id.trim()))
                            .map_err(|err| {
                                engine::ProductError::from(Box::new(err) as engine::Cause)
                            })?;
                    let reason = RetirementReason::new(
                        "Updated chart evidence before the visit resolved the earlier pre-visit concern.",
                    )
                    .map_err(|err| engine::ProductError::from(Box::new(err) as engine::Cause))?;
                    match retire_concern_in_tx(
                        &tx,
                        &retire_key,
                        &RetireConcern {
                            practice: claim.scope.practice.clone(),
                            workspace: claim.scope.workspace.clone(),
                            id: prior_id.clone(),
                            reason,
                            source_work: source_work.clone(),
                        },
                    )
                    .await
                    {
                        Ok(_)
                        | Err(ConcernError::NotOpen { .. } | ConcernError::NotFound { .. }) => {}
                        Err(ConcernError::Database(db)) => return Err(persistence_failure(db)),
                        Err(other) => {
                            return Err(engine::ProductError::from(
                                Box::new(other) as engine::Cause
                            ));
                        }
                    }
                }
                None
            }
        };

        let narrative = render_briefing_narrative(&appt.starts_at, &appt.label, &parsed);
        let origin = BriefingOrigin {
            job_id: claim.scope.job_id.clone(),
            work_id: job.work.clone(),
            attempt_id: claim.attempt.clone(),
        };
        let source_version_i64 = i64::try_from(observation.version)
            .map_err(conversion_error("source_version", "i64"))
            .map_err(persistence_failure)?;

        visit_briefing::Entity::insert(visit_briefing::ActiveModel {
            practice: Set(claim.scope.practice.clone()),
            source_id: Set(briefing_ctx.source_id.clone()),
            appointment_id: Set(briefing_ctx.appointment_id.clone()),
            starts_at_unix: Set(briefing_ctx.starts_at_unix),
            briefing_id: Set(briefing_id.clone()),
            workspace: Set(claim.scope.workspace.clone()),
            idempotency_key: Set(idempotency_key),
            starts_at: Set(appt.starts_at.clone()),
            ends_at: Set(appt.ends_at.clone()),
            ends_at_unix: Set(appt.ends_at_unix),
            appointment_saved_version: Set(appt.saved_version),
            prepared_at: Set(prepared_at),
            source_version: Set(source_version_i64),
            source_versions: Set(serde_json::to_value(&source_versions)
                .map_err(conversion_error("source_versions", "json"))
                .map_err(persistence_failure)?),
            source_freshness: Set(serde_json::to_value(&source_freshness)
                .map_err(conversion_error("source_freshness", "json"))
                .map_err(persistence_failure)?),
            limits: Set(serde_json::to_value(&limits)
                .map_err(conversion_error("limits", "json"))
                .map_err(persistence_failure)?),
            visit_card_orientation: Set(parsed.visit_card_orientation.clone()),
            trajectory: Set(serde_json::to_value(&parsed.trajectory)
                .map_err(conversion_error("trajectory", "json"))
                .map_err(persistence_failure)?),
            meaningful_changes: Set(serde_json::to_value(&parsed.meaningful_changes)
                .map_err(conversion_error("meaningful_changes", "json"))
                .map_err(persistence_failure)?),
            unresolved_issues: Set(serde_json::to_value(&parsed.unresolved_issues)
                .map_err(conversion_error("unresolved_issues", "json"))
                .map_err(persistence_failure)?),
            visit_decisions: Set(serde_json::to_value(&parsed.visit_decisions)
                .map_err(conversion_error("visit_decisions", "json"))
                .map_err(persistence_failure)?),
            basis_and_limitations: Set(parsed.basis_and_limitations.clone()),
            basis_citations: Set(serde_json::to_value(&parsed.basis_citations)
                .map_err(conversion_error("basis_citations", "json"))
                .map_err(persistence_failure)?),
            citations: Set(serde_json::to_value(&citations)
                .map_err(conversion_error("citations", "json"))
                .map_err(persistence_failure)?),
            narrative: Set(narrative),
            raised_concern_id: Set(raised_concern_id.as_ref().map(|id| id.as_str().to_owned())),
            visit_standing: Set("current".to_owned()),
            visit_standing_detail: Set(None),
            origin: Set(serde_json::to_value(&origin)
                .map_err(conversion_error("BriefingOrigin", "json"))
                .map_err(persistence_failure)?),
            updated_at: Set(prepared_at),
        })
        .on_conflict(
            OnConflict::columns([
                visit_briefing::Column::Practice,
                visit_briefing::Column::SourceId,
                visit_briefing::Column::AppointmentId,
                visit_briefing::Column::StartsAtUnix,
            ])
            .update_columns([
                visit_briefing::Column::Workspace,
                visit_briefing::Column::IdempotencyKey,
                visit_briefing::Column::StartsAt,
                visit_briefing::Column::EndsAt,
                visit_briefing::Column::EndsAtUnix,
                visit_briefing::Column::AppointmentSavedVersion,
                visit_briefing::Column::PreparedAt,
                visit_briefing::Column::SourceVersion,
                visit_briefing::Column::SourceVersions,
                visit_briefing::Column::SourceFreshness,
                visit_briefing::Column::Limits,
                visit_briefing::Column::VisitCardOrientation,
                visit_briefing::Column::Trajectory,
                visit_briefing::Column::MeaningfulChanges,
                visit_briefing::Column::UnresolvedIssues,
                visit_briefing::Column::VisitDecisions,
                visit_briefing::Column::BasisAndLimitations,
                visit_briefing::Column::BasisCitations,
                visit_briefing::Column::Citations,
                visit_briefing::Column::Narrative,
                visit_briefing::Column::RaisedConcernId,
                visit_briefing::Column::VisitStanding,
                visit_briefing::Column::VisitStandingDetail,
                visit_briefing::Column::Origin,
                visit_briefing::Column::UpdatedAt,
            ])
            .to_owned(),
        )
        .exec_without_returning(&tx)
        .await
        .map_err(persistence_failure)?;

        // Advance `visit_preparation` row to `ready` (creating one if briefing job was started directly).
        if let Some(prep_row) = visit_preparation::Entity::find_by_id(slot.pk_tuple())
            .lock_exclusive()
            .one(&tx)
            .await
            .map_err(persistence_failure)?
        {
            visit_preparation::ActiveModel {
                practice: Unchanged(prep_row.practice),
                source_id: Unchanged(prep_row.source_id),
                appointment_id: Unchanged(prep_row.appointment_id),
                starts_at_unix: Unchanged(prep_row.starts_at_unix),
                workspace: Unchanged(prep_row.workspace),
                ends_at_unix: Set(appt.ends_at_unix),
                appointment_saved_version: Set(appt.saved_version),
                observed_source_version: Set(source_version_i64),
                decision: Unchanged(prep_row.decision),
                decided_at: Unchanged(prep_row.decided_at),
                selection_job_id: Unchanged(prep_row.selection_job_id),
                selection_work_id: Unchanged(prep_row.selection_work_id),
                selection_idempotency_key: Unchanged(prep_row.selection_idempotency_key),
                state: Set("ready".to_owned()),
                briefing_job_id: Set(Some(claim.scope.job_id.clone())),
                briefing_id: Set(Some(briefing_id.clone())),
                failed_reason: Set(None),
                updated_at: Set(prepared_at),
            }
            .update(&tx)
            .await
            .map_err(persistence_failure)?;
        } else {
            let direct_decision = VisitPreparationDecision::PrepareNow {
                reason: briefing_ctx.selection_reason.clone(),
            };
            visit_preparation::Entity::insert(visit_preparation::ActiveModel {
                practice: Set(claim.scope.practice.clone()),
                source_id: Set(briefing_ctx.source_id.clone()),
                appointment_id: Set(briefing_ctx.appointment_id.clone()),
                starts_at_unix: Set(briefing_ctx.starts_at_unix),
                workspace: Set(claim.scope.workspace.clone()),
                ends_at_unix: Set(appt.ends_at_unix),
                appointment_saved_version: Set(appt.saved_version),
                observed_source_version: Set(source_version_i64),
                decision: Set(serde_json::to_value(&direct_decision)
                    .map_err(conversion_error("VisitPreparationDecision", "json"))
                    .map_err(persistence_failure)?),
                decided_at: Set(prepared_at),
                selection_job_id: Set(claim.scope.job_id.clone()),
                selection_work_id: Set(job.work.clone()),
                selection_idempotency_key: Set(format!("direct:{}", job.work)),
                state: Set("ready".to_owned()),
                briefing_job_id: Set(Some(claim.scope.job_id.clone())),
                briefing_id: Set(Some(briefing_id.clone())),
                failed_reason: Set(None),
                updated_at: Set(prepared_at),
            })
            .exec_without_returning(&tx)
            .await
            .map_err(persistence_failure)?;
        }

        tx.commit().await.map_err(persistence_failure)?;

        Ok(ToolOutcome::Returned(
            json!({
                "outcome": "saved",
                "briefing_id": briefing_id,
                "appointment_id": briefing_ctx.appointment_id,
                "starts_at": appt.starts_at,
                "prepared_at": prepared_at,
                "source_version": observation.version,
                "citation_count": citations.len(),
                "raised_concern_id": raised_concern_id,
            })
            .to_string(),
        ))
    }

    /// Completion barrier invoked by [`PracticeJobHost::finish`] on
    /// [`PracticeJobKind::VisitPreparation`] runs before committing the job.
    ///
    /// Returns `Some(feedback)` (`Finish::Continue`) when:
    /// - A selection run has not recorded a decision for every still-eligible candidate visit;
    /// - A briefing run has not called `save_visit_briefing`; or
    /// - Patient chart evidence changed after `save_visit_briefing` and before `finish`,
    ///   requiring the run to inspect the updated records and refresh the saved briefing.
    pub(super) async fn finish_visit_preparation_gate(
        &self,
        claim: &PracticeJobClaim,
        job: &PracticeJob,
    ) -> Result<Option<Value>, engine::ProductError> {
        if let Some(selection_ctx) = job.assignment.visit_selection.as_ref() {
            let saved_rows = visit_preparation::Entity::find()
                .filter(visit_preparation::Column::Practice.eq(&claim.scope.practice))
                .filter(visit_preparation::Column::SelectionWorkId.eq(&job.work))
                .all(&self.db)
                .await
                .map_err(persistence_failure)?;
            let decided_slots: BTreeSet<(String, String, i64)> = saved_rows
                .into_iter()
                .map(|row| (row.source_id, row.appointment_id, row.starts_at_unix))
                .collect();

            let mut undecided = Vec::new();
            for cand in &selection_ctx.candidates {
                if decided_slots.contains(&(
                    cand.source_id.clone(),
                    cand.appointment_id.clone(),
                    cand.starts_at_unix,
                )) {
                    continue;
                }
                let appt = schedule_appointment::Entity::find_by_id((
                    claim.scope.practice.clone(),
                    cand.source_id.clone(),
                    cand.appointment_id.clone(),
                ))
                .one(&self.db)
                .await
                .map_err(persistence_failure)?;
                if let Some(appt) = appt
                    && appt.status == AppointmentStatus::Scheduled
                    && appt.starts_at_unix == cand.starts_at_unix
                {
                    undecided.push(json!({
                        "source_id": cand.source_id,
                        "appointment_id": cand.appointment_id,
                        "starts_at_unix": cand.starts_at_unix,
                        "label": cand.label,
                    }));
                }
            }

            if !undecided.is_empty() {
                return Ok(Some(json!({
                    "outcome": "undecided_candidate_visits",
                    "undecided": undecided,
                    "message": "Call select_visit_preparations to record a preparation decision and reason for every remaining candidate visit before finishing.",
                })));
            }
            return Ok(None);
        }

        if let Some(briefing_ctx) = job.assignment.visit_briefing.as_ref() {
            let slot = VisitSlotKey {
                practice: claim.scope.practice.clone(),
                source_id: briefing_ctx.source_id.clone(),
                appointment_id: briefing_ctx.appointment_id.clone(),
                starts_at_unix: briefing_ctx.starts_at_unix,
            };
            let existing = visit_briefing::Entity::find_by_id(slot.pk_tuple())
                .one(&self.db)
                .await
                .map_err(persistence_failure)?;
            let Some(saved_row) = existing else {
                return Ok(Some(json!({
                    "outcome": "briefing_not_saved",
                    "appointment_id": briefing_ctx.appointment_id,
                    "message": "Call save_visit_briefing to record the cited pre-visit briefing before finishing.",
                })));
            };
            let saved = decode_briefing(saved_row).map_err(persistence_failure)?;
            if saved.origin.work_id != job.work {
                return Ok(Some(json!({
                    "outcome": "briefing_not_saved_for_current_run",
                    "appointment_id": briefing_ctx.appointment_id,
                    "message": "Call save_visit_briefing in this run before finishing.",
                })));
            }

            let observation = self
                .sources(&claim.scope.workspace)
                .await
                .map_err(persistence_failure)?;
            if let Some(feedback) = briefing_source_change_feedback(
                BriefingFenceStage::Finish,
                saved.source_version,
                &saved.source_versions,
                observation.version,
                &observation.sources,
            ) {
                return Ok(Some(feedback));
            }
        }

        Ok(None)
    }

    /// Computes optional `source_feedback` for [`PracticeJobHost::run`] when an
    /// interrupted visit-briefing attempt recovers after chart records were
    /// added, corrected, or withdrawn.
    pub(super) async fn visit_preparation_recovery_feedback(
        &self,
        claim: &PracticeJobClaim,
        job: &PracticeJob,
        ordered: &[(engine::Event, bool)],
    ) -> Result<Option<Value>, DbErr> {
        let Some(briefing_ctx) = job.assignment.visit_briefing.as_ref() else {
            return Ok(None);
        };
        if ordered.is_empty() {
            return Ok(None);
        }
        let observation = self.sources(&claim.scope.workspace).await?;
        let slot = VisitSlotKey {
            practice: claim.scope.practice.clone(),
            source_id: briefing_ctx.source_id.clone(),
            appointment_id: briefing_ctx.appointment_id.clone(),
            starts_at_unix: briefing_ctx.starts_at_unix,
        };
        if let Some(saved) = self.visit_briefing_internal(&slot).await?
            && saved.origin.work_id == job.work
            && saved.source_version == observation.version
        {
            return Ok(None);
        }
        let known_revisions =
            model_chart_knowledge(&job.sources, ordered, &job.message_sources(), true)
                .map_err(|err| error(&err))?;
        Ok(briefing_source_change_feedback(
            BriefingFenceStage::Recovery,
            job.sources_version,
            &known_revisions,
            observation.version,
            &observation.sources,
        ))
    }

    async fn visit_briefing_internal(
        &self,
        slot: &VisitSlotKey,
    ) -> Result<Option<SavedVisitBriefing>, DbErr> {
        visit_briefing::Entity::find_by_id(slot.pk_tuple())
            .one(&self.db)
            .await?
            .map(decode_briefing)
            .transpose()
    }

    async fn visit_briefing_by_id_internal(
        &self,
        practice: &str,
        briefing_id: &str,
    ) -> Result<Option<SavedVisitBriefing>, DbErr> {
        visit_briefing::Entity::find()
            .filter(visit_briefing::Column::Practice.eq(practice))
            .filter(visit_briefing::Column::BriefingId.eq(briefing_id))
            .one(&self.db)
            .await?
            .map(decode_briefing)
            .transpose()
    }

    async fn visit_briefings_for_appointment_internal(
        &self,
        practice: &str,
        source_id: &str,
        appointment_id: &str,
    ) -> Result<Vec<SavedVisitBriefing>, DbErr> {
        let rows = visit_briefing::Entity::find()
            .filter(visit_briefing::Column::Practice.eq(practice))
            .filter(visit_briefing::Column::SourceId.eq(source_id))
            .filter(visit_briefing::Column::AppointmentId.eq(appointment_id))
            .order_by_asc(visit_briefing::Column::StartsAtUnix)
            .all(&self.db)
            .await?;
        rows.into_iter().map(decode_briefing).collect()
    }

    /// Reads the saved visit briefing for an exact `(practice, source_id, appointment_id, starts_at_unix)` slot
    /// after verifying practice patient membership on `authority`.
    pub async fn visit_briefing<A: PracticeAuthority + ?Sized>(
        &self,
        slot: &VisitSlotKey,
        authority: &A,
    ) -> Result<Option<SavedVisitBriefing>, BriefingError> {
        if [
            slot.practice.as_str(),
            slot.source_id.as_str(),
            slot.appointment_id.as_str(),
        ]
        .iter()
        .any(|part| part.trim().is_empty())
        {
            return Err(BriefingError::Access(PracticeAccessError::InvalidScope));
        }
        let Some(briefing) = self.visit_briefing_internal(slot).await? else {
            return Ok(None);
        };
        authority
            .check_practice_patient(&membership_scope(&slot.practice, &briefing.workspace))
            .await?;
        Ok(Some(briefing))
    }

    /// Reads one saved visit briefing by `(practice, briefing_id)` after verifying
    /// practice patient membership on `authority`.
    pub async fn visit_briefing_by_id<A: PracticeAuthority + ?Sized>(
        &self,
        practice: &str,
        briefing_id: &str,
        authority: &A,
    ) -> Result<Option<SavedVisitBriefing>, BriefingError> {
        if practice.trim().is_empty() || briefing_id.trim().is_empty() {
            return Err(BriefingError::Access(PracticeAccessError::InvalidScope));
        }
        let Some(briefing) = self
            .visit_briefing_by_id_internal(practice, briefing_id)
            .await?
        else {
            return Ok(None);
        };
        authority
            .check_practice_patient(&membership_scope(practice, &briefing.workspace))
            .await?;
        Ok(Some(briefing))
    }

    /// Lists all saved briefings for `(practice, source_id, appointment_id)`
    /// ordered by `starts_at_unix`, including historical briefings retained when
    /// an appointment moved or was cancelled, after verifying practice patient membership.
    pub async fn visit_briefings_for_appointment<A: PracticeAuthority + ?Sized>(
        &self,
        practice: &str,
        source_id: &str,
        appointment_id: &str,
        authority: &A,
    ) -> Result<Vec<SavedVisitBriefing>, BriefingError> {
        if [practice, source_id, appointment_id]
            .iter()
            .any(|part| part.trim().is_empty())
        {
            return Err(BriefingError::Access(PracticeAccessError::InvalidScope));
        }
        let briefings = self
            .visit_briefings_for_appointment_internal(practice, source_id, appointment_id)
            .await?;
        for briefing in &briefings {
            authority
                .check_practice_patient(&membership_scope(practice, &briefing.workspace))
                .await?;
        }
        Ok(briefings)
    }

    /// Lists all saved visit briefings for `(practice, workspace)` ordered by
    /// `starts_at_unix` after verifying practice patient membership on `authority`.
    pub async fn visit_briefings_for_workspace<A: PracticeAuthority + ?Sized>(
        &self,
        practice: &str,
        workspace: &str,
        authority: &A,
    ) -> Result<Vec<SavedVisitBriefing>, BriefingError> {
        authority
            .check_practice_patient(&membership_scope(practice, workspace))
            .await?;
        let rows = visit_briefing::Entity::find()
            .filter(visit_briefing::Column::Practice.eq(practice))
            .filter(visit_briefing::Column::Workspace.eq(workspace))
            .order_by_asc(visit_briefing::Column::StartsAtUnix)
            .all(&self.db)
            .await?;
        Ok(rows
            .into_iter()
            .map(decode_briefing)
            .collect::<Result<Vec<_>, _>>()?)
    }

    /// Reconciles any `Preparing` row in `practice` whose underlying `practice_job`
    /// has already settled in `Failed`, `Held`, or `Cancelled`.
    pub(super) async fn reconcile_unfinished_visit_preparations(
        &self,
        practice: &str,
    ) -> Result<(), DbErr> {
        let preparing_rows = visit_preparation::Entity::find()
            .filter(visit_preparation::Column::Practice.eq(practice))
            .filter(visit_preparation::Column::State.eq("preparing"))
            .all(&self.db)
            .await?;
        for row in preparing_rows {
            let Some(job_id) = row.briefing_job_id.clone() else {
                continue;
            };
            let Some(job) = practice_job::Entity::find_by_id((
                row.practice.clone(),
                row.workspace.clone(),
                job_id.clone(),
            ))
            .one(&self.db)
            .await?
            else {
                continue;
            };
            let (next_state, next_reason) = match job.status {
                WorkStatus::Failed => (
                    "could_not_finish",
                    Some(format!(
                        "Visit preparation job `{job_id}` ended in `failed` state without saving a briefing."
                    )),
                ),
                WorkStatus::Held => (
                    "could_not_finish",
                    Some(format!(
                        "Visit preparation job `{job_id}` ended in `held` state without saving a briefing."
                    )),
                ),
                WorkStatus::Cancelled => (
                    "ended_visit_changed",
                    Some(
                        "Appointment changed or preparation cancelled before completion."
                            .to_owned(),
                    ),
                ),
                WorkStatus::Pending | WorkStatus::Active | WorkStatus::Completed => continue,
            };
            visit_preparation::ActiveModel {
                practice: Unchanged(row.practice),
                source_id: Unchanged(row.source_id),
                appointment_id: Unchanged(row.appointment_id),
                starts_at_unix: Unchanged(row.starts_at_unix),
                workspace: Unchanged(row.workspace),
                ends_at_unix: Unchanged(row.ends_at_unix),
                appointment_saved_version: Unchanged(row.appointment_saved_version),
                observed_source_version: Unchanged(row.observed_source_version),
                decision: Unchanged(row.decision),
                decided_at: Unchanged(row.decided_at),
                selection_job_id: Unchanged(row.selection_job_id),
                selection_work_id: Unchanged(row.selection_work_id),
                selection_idempotency_key: Unchanged(row.selection_idempotency_key),
                state: Set(next_state.to_owned()),
                briefing_job_id: Unchanged(row.briefing_job_id),
                briefing_id: Unchanged(row.briefing_id),
                failed_reason: Set(next_reason),
                updated_at: Set(job.updated_at),
            }
            .update(&self.db)
            .await?;
        }
        Ok(())
    }

    pub(super) async fn visit_preparations_for_practice_internal(
        &self,
        practice: &str,
    ) -> Result<Vec<VisitPreparationRecord>, DbErr> {
        let rows = visit_preparation::Entity::find()
            .filter(visit_preparation::Column::Practice.eq(practice))
            .order_by_asc(visit_preparation::Column::StartsAtUnix)
            .order_by_asc(visit_preparation::Column::AppointmentId)
            .all(&self.db)
            .await?;

        let briefing_rows = visit_briefing::Entity::find()
            .filter(visit_briefing::Column::Practice.eq(practice))
            .all(&self.db)
            .await?;
        let orientations: BTreeMap<String, String> = briefing_rows
            .into_iter()
            .map(|row| (row.briefing_id, row.visit_card_orientation))
            .collect();

        let mut records = Vec::with_capacity(rows.len());
        for row in rows {
            let orientation = row
                .briefing_id
                .as_deref()
                .and_then(|id| orientations.get(id).map(String::as_str));
            records.push(decode_preparation_record(row, orientation)?);
        }
        Ok(records)
    }

    /// Reads all visit preparation records for `practice` without mutating state,
    /// verifying practice patient membership on `authority` for every workspace
    /// in the returned records.
    pub async fn visit_preparations_for_practice<A: PracticeAuthority + ?Sized>(
        &self,
        practice: &str,
        authority: &A,
    ) -> Result<Vec<VisitPreparationRecord>, BriefingError> {
        if practice.trim().is_empty() {
            return Err(BriefingError::Access(PracticeAccessError::InvalidScope));
        }
        let records = self
            .visit_preparations_for_practice_internal(practice)
            .await?;
        let mut checked_workspaces = BTreeSet::new();
        for record in &records {
            if checked_workspaces.insert(record.workspace.as_str()) {
                authority
                    .check_practice_patient(&membership_scope(practice, &record.workspace))
                    .await?;
            }
        }
        Ok(records)
    }

    /// Reads the visit-concern lifecycle link for `(practice, workspace, concern_id)`
    /// after verifying practice patient membership on `authority`.
    pub async fn visit_concern_link<A: PracticeAuthority + ?Sized>(
        &self,
        practice: &str,
        workspace: &str,
        concern_id: &ConcernId,
        authority: &A,
    ) -> Result<Option<VisitConcernLink>, BriefingError> {
        authority
            .check_practice_patient(&membership_scope(practice, workspace))
            .await?;
        Ok(visit_concern::Entity::find_by_id((
            practice.to_owned(),
            workspace.to_owned(),
            concern_id.as_str().to_owned(),
        ))
        .one(&self.db)
        .await?
        .map(decode_visit_concern_link)
        .transpose()?)
    }

    /// Reconciles schedule changes and retires ended `Today` visit concerns (and
    /// already-opened `Now` visit concerns) for `practice` at `now_unix`.
    pub async fn reconcile_visit_concerns(
        &self,
        practice: &str,
        now_unix: i64,
    ) -> Result<Vec<ConcernId>, DbErr> {
        let tx = self.db.begin().await?;
        let retired = reconcile_schedule_changes_in_tx(&tx, practice, now_unix).await?;
        tx.commit().await?;
        Ok(retired)
    }

    /// Records that an authorized physician sharing `access.workspace` opened
    /// `concern_id` at `opened_at_unix`.
    ///
    /// When `concern_id` is a `Now` visit concern whose visit has already ended
    /// (`opened_at_unix >= ends_at_unix`) or been cancelled/moved, opening it
    /// satisfies its post-visit visibility requirement and retires the concern
    /// in the same transaction.
    pub async fn open_concern(
        &self,
        access: &WorkspaceAccess<'_>,
        command_key: &CommandKey,
        concern_id: &ConcernId,
        authority: &mut dyn Authority,
        opened_at_unix: i64,
    ) -> Result<ConcernOpenOutcome, ConcernError> {
        let scope = access.authority_scope()?;
        let practice = scope.practice.as_str();
        let physician = scope.physician.as_str();
        let workspace = scope.workspace.as_str();

        let tx = self.db.begin().await?;
        if let Err(reason) = authority
            .lock_publication(&tx, &scope, Some(access.session_hash))
            .await
        {
            tx.rollback().await?;
            return Err(ConcernError::AccessDenied { reason });
        }

        let command_body = json!({
            "kind": "open_visit_concern",
            "practice": practice,
            "physician": physician,
            "workspace": workspace,
            "concern_id": concern_id.as_str(),
        });

        match receipt::record(
            &tx,
            practice,
            CONCERN_OPEN_RECEIPT_ACTOR,
            command_key.as_str(),
            &command_body,
        )
        .await?
        {
            Receipt::New => {}
            Receipt::Same => {
                let existing = load_concern(&tx, practice, workspace, concern_id)
                    .await?
                    .ok_or_else(|| ConcernError::NotFound {
                        id: concern_id.clone(),
                    })?;
                tx.commit().await?;
                return Ok(ConcernOpenOutcome::Duplicate {
                    concern: Box::new(existing),
                });
            }
            Receipt::Different => {
                return Err(ConcernError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
        }

        let current_concern = load_concern(&tx, practice, workspace, concern_id)
            .await?
            .ok_or_else(|| ConcernError::NotFound {
                id: concern_id.clone(),
            })?;

        let mut retired_after_visit_end = false;
        if let Some(vc_row) = visit_concern::Entity::find_by_id((
            practice.to_owned(),
            workspace.to_owned(),
            concern_id.as_str().to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        {
            let opened_at = vc_row.opened_at.unwrap_or(opened_at_unix);
            let opened_by = vc_row
                .opened_by_physician
                .clone()
                .unwrap_or_else(|| physician.to_owned());

            let updated_vc_row = visit_concern::ActiveModel {
                practice: Unchanged(vc_row.practice.clone()),
                workspace: Unchanged(vc_row.workspace.clone()),
                concern_id: Unchanged(vc_row.concern_id.clone()),
                source_id: Unchanged(vc_row.source_id.clone()),
                appointment_id: Unchanged(vc_row.appointment_id.clone()),
                starts_at_unix: Unchanged(vc_row.starts_at_unix),
                ends_at_unix: Unchanged(vc_row.ends_at_unix),
                urgency: Unchanged(vc_row.urgency.clone()),
                opened_at: Set(Some(opened_at)),
                opened_by_physician: Set(Some(opened_by)),
                created_at: Unchanged(vc_row.created_at),
                updated_at: Set(opened_at_unix),
            }
            .update(&tx)
            .await?;

            let updated_link = decode_visit_concern_link(updated_vc_row)?;
            let appt_row = schedule_appointment::Entity::find_by_id((
                practice.to_owned(),
                vc_row.source_id.clone(),
                vc_row.appointment_id.clone(),
            ))
            .one(&tx)
            .await?;
            retired_after_visit_end = reconcile_one_visit_concern_in_tx(
                &tx,
                &updated_link,
                appt_row.as_ref(),
                opened_at_unix,
            )
            .await?;
        }

        let updated = load_concern(&tx, practice, workspace, concern_id)
            .await?
            .unwrap_or(current_concern);
        tx.commit().await?;

        Ok(ConcernOpenOutcome::Opened {
            concern: Box::new(updated),
            retired_after_visit_end,
        })
    }

    /// Lists distinct practices that currently hold a saved schedule source in
    /// `physician_runtime_schedule_sources`.
    pub async fn scheduled_practices(&self) -> Result<Vec<String>, DbErr> {
        let rows = super::entities::schedule_source::Entity::find()
            .select_only()
            .column(super::entities::schedule_source::Column::Practice)
            .order_by_asc(super::entities::schedule_source::Column::Practice)
            .into_tuple::<String>()
            .all(&self.db)
            .await?;
        let unique: BTreeSet<String> = rows.into_iter().collect();
        Ok(unique.into_iter().collect())
    }

    async fn build_candidate_brief(
        &self,
        practice: &str,
        appt: &SavedAppointment,
        workspace: &str,
        prior: Option<&VisitPreparationRecord>,
    ) -> Result<CandidateVisitBrief, DbErr> {
        let starting = self.practice_starting_context(practice, workspace).await?;
        let observation = self.sources(workspace).await?;
        let mut recent_records: Vec<CandidateRecordPreview> = observation
            .sources
            .iter()
            .rev()
            .take(MAX_SELECTION_RECORD_PREVIEWS)
            .map(|src| CandidateRecordPreview {
                candidate_id: src.candidate_id.clone(),
                revision: src.revision.clone(),
                source_id: src.source_id.clone(),
                record_type: src.record_type.clone(),
                dates: src.dates.clone(),
                excerpt: record_excerpt(src),
            })
            .collect();
        recent_records.reverse();

        let (summary_claims, material_gaps) = match &starting.understanding {
            Some(saved) => (
                saved
                    .summary
                    .claims
                    .iter()
                    .map(|c| c.text.clone())
                    .collect(),
                saved.summary.material_gaps.clone(),
            ),
            None => (Vec::new(), Vec::new()),
        };
        let active_memories = match &starting.memory_standing {
            PatientMemoryStanding::Available {
                summary_markdown,
                direction_markdown,
                ..
            } => summary_markdown
                .iter()
                .chain(direction_markdown.iter())
                .cloned()
                .collect(),
            PatientMemoryStanding::Unavailable { .. } => Vec::new(),
        };
        let prior_decision = prior.map(|rec| {
            let target_at_unix = match &rec.decision {
                VisitPreparationDecision::Schedule {
                    prepare_at_unix, ..
                } => Some(*prepare_at_unix),
                VisitPreparationDecision::Defer {
                    check_again_at_unix,
                    ..
                } => Some(*check_again_at_unix),
                VisitPreparationDecision::PrepareNow { .. }
                | VisitPreparationDecision::Skip { .. } => None,
            };
            PriorVisitDecisionSummary {
                action: rec.decision.action_str().to_owned(),
                target_at_unix,
                reason: rec.decision.reason().to_owned(),
                observed_source_version: rec.observed_source_version,
            }
        });

        Ok(CandidateVisitBrief {
            source_id: appt.source_id.as_str().to_owned(),
            appointment_id: appt.appointment_id.as_str().to_owned(),
            appointment_saved_version: appt.saved_version,
            starts_at: appt.window.starts_at().to_owned(),
            starts_at_unix: appt.window.starts_at_unix(),
            ends_at: appt.window.ends_at().to_owned(),
            ends_at_unix: appt.window.ends_at_unix(),
            label: appt.label.clone(),
            source_status: appt.source_status.clone(),
            workspace: workspace.to_owned(),
            patient: starting.patient,
            chart_version: starting.chart_version,
            chart_record_count: starting.chart_record_count,
            source_pulls: starting.source_pulls,
            check_standing: starting.check_standing,
            summary_claims,
            material_gaps,
            active_memories,
            open_concerns: starting.open_concerns,
            recent_records,
            prior_decision,
        })
    }

    async fn collect_selection_candidates(
        &self,
        auth: &dyn AuthStore,
        practice: &str,
        appointments: &[SavedAppointment],
        now_unix: i64,
    ) -> Result<Vec<CandidateVisitBrief>, BriefingError> {
        let roster: BTreeSet<String> = auth
            .practice_patients(practice)
            .await
            .map_err(PracticeAccessError::StoreUnavailable)?
            .into_iter()
            .collect();
        let existing_preps = self
            .visit_preparations_for_practice_internal(practice)
            .await?;
        let prep_map: BTreeMap<(String, String, i64), VisitPreparationRecord> = existing_preps
            .into_iter()
            .map(|rec| {
                (
                    (
                        rec.slot.source_id.clone(),
                        rec.slot.appointment_id.clone(),
                        rec.slot.starts_at_unix,
                    ),
                    rec,
                )
            })
            .collect();

        let mut candidates = Vec::new();
        for appt in appointments {
            if !visit_can_start_preparation(appt.status, appt.window.ends_at_unix(), now_unix) {
                continue;
            }
            let PatientMatch::Matched { workspace_id, .. } = &appt.patient_match else {
                continue;
            };
            if !roster.contains(workspace_id) {
                continue;
            }
            let obs = self.sources(workspace_id).await?;
            let slot_key = (
                appt.source_id.as_str().to_owned(),
                appt.appointment_id.as_str().to_owned(),
                appt.window.starts_at_unix(),
            );
            let prior = prep_map.get(&slot_key);
            let needs_selection = match prior {
                None => true,
                Some(rec) => match &rec.state {
                    VisitPreparationState::Deferred {
                        check_again_at_unix,
                        ..
                    } => {
                        now_unix >= *check_again_at_unix
                            || obs.version > rec.observed_source_version
                            || appt.saved_version > rec.appointment_saved_version
                    }
                    VisitPreparationState::Scheduled { .. }
                    | VisitPreparationState::Skipped { .. } => {
                        obs.version > rec.observed_source_version
                            || appt.saved_version > rec.appointment_saved_version
                    }
                    VisitPreparationState::Ready { .. }
                    | VisitPreparationState::Preparing { .. }
                    | VisitPreparationState::CouldNotFinish { .. }
                    | VisitPreparationState::EndedVisitChanged { .. } => false,
                },
            };

            if needs_selection {
                candidates.push(
                    self.build_candidate_brief(practice, appt, workspace_id, prior)
                        .await?,
                );
            }
        }
        Ok(candidates)
    }

    /// Single-owner pre-preparation source readiness check (`P4.4`).
    ///
    /// Checks whether the patient's configured sources have updates before
    /// starting a visit briefing job. A source that fails to update never blocks
    /// preparation: its failed status and `last_good_pulled_at` are recorded in
    /// Postgres so the briefing works from the last good records and states so.
    async fn ensure_previsit_sources_checked<A: AuthStore>(
        &self,
        auth: &A,
        practice: &str,
        workspace: &str,
        slot: &VisitSlotKey,
        now_unix: i64,
    ) {
        let configured = match self
            .source_refresh()
            .configured_sources(practice, workspace)
            .await
        {
            Ok(sources) if !sources.is_empty() => sources,
            Ok(_) => return,
            Err(error) => {
                tracing::warn!(
                    %error,
                    practice,
                    workspace,
                    "pre-preparation source discovery failed; continuing from retained records"
                );
                return;
            }
        };
        let _ = configured;
        let key_digest = &crate::sha256_hex(
            format!(
                "previsit-refresh:{practice}:{workspace}:{}:{}:{}:{now_unix}",
                slot.source_id, slot.appointment_id, slot.starts_at_unix
            )
            .as_bytes(),
        )[..24];
        let Ok(command_key) =
            super::refresh::RefreshCommandKey::new(format!("previsit-refresh:{key_digest}"))
        else {
            return;
        };
        let refresh_scope = PracticeScope {
            practice: practice.to_owned(),
            workspace: workspace.to_owned(),
            job_id: format!("previsit-refresh:{key_digest}"),
        };
        let request = super::refresh::RefreshRequest {
            command_key,
            target: super::refresh::RefreshTarget::AllSources,
        };
        if let Err(error) = self
            .refresh_patient_sources_for_practice(
                &refresh_scope,
                auth,
                &request,
                self.source_refresh(),
            )
            .await
        {
            tracing::warn!(
                %error,
                practice,
                workspace,
                "pre-preparation source refresh failed; continuing from last good records"
            );
        }
    }

    async fn accept_due_briefing_job<A: AuthStore>(
        &self,
        auth: &A,
        appt: &SavedAppointment,
        prep: &VisitPreparationRecord,
        roles: &ModelRoles,
        skills: &PhysicianSkills,
        now_unix: i64,
    ) -> Result<Option<PracticeScope>, BriefingError> {
        let practice = prep.slot.practice.as_str();
        if !visit_can_start_preparation(appt.status, appt.window.ends_at_unix(), now_unix) {
            return Ok(None);
        }
        self.ensure_previsit_sources_checked(auth, practice, &prep.workspace, &prep.slot, now_unix)
            .await;
        let starting_context = self
            .practice_starting_context(practice, &prep.workspace)
            .await?;
        let source_statuses = self
            .workspace_source_refresh_summaries(practice, &prep.workspace)
            .await?;
        let historical = self
            .visit_briefings_for_appointment_internal(
                practice,
                appt.source_id.as_str(),
                appt.appointment_id.as_str(),
            )
            .await?;
        let prior_visit_briefings: Vec<HistoricalBriefingSummary> = historical
            .into_iter()
            .filter(|b| b.slot.starts_at_unix != appt.window.starts_at_unix())
            .map(|b| HistoricalBriefingSummary {
                briefing_id: b.briefing_id,
                starts_at: b.starts_at,
                prepared_at: b.prepared_at,
                visit_card_orientation: b.visit_card_orientation,
                standing: match b.visit_standing {
                    VisitStanding::Current => "current".to_owned(),
                    VisitStanding::Moved { .. } => "moved".to_owned(),
                    VisitStanding::Cancelled { .. } => "cancelled".to_owned(),
                },
            })
            .collect();

        let base_job_id = format!(
            "{}-av{}-sv{}",
            briefing_job_id(
                appt.source_id.as_str(),
                appt.appointment_id.as_str(),
                appt.window.starts_at_unix(),
            ),
            appt.saved_version,
            starting_context.chart_version,
        );
        let context = VisitBriefingContext {
            practice: practice.to_owned(),
            workspace: prep.workspace.clone(),
            source_id: appt.source_id.as_str().to_owned(),
            appointment_id: appt.appointment_id.as_str().to_owned(),
            appointment_saved_version: appt.saved_version,
            starts_at: appt.window.starts_at().to_owned(),
            starts_at_unix: appt.window.starts_at_unix(),
            ends_at: appt.window.ends_at().to_owned(),
            ends_at_unix: appt.window.ends_at_unix(),
            label: appt.label.clone(),
            source_status: appt.source_status.clone(),
            selection_reason: prep.decision.reason().to_owned(),
            target_source_version: starting_context.chart_version,
            queued_at_unix: Some(now_unix),
            starting_context,
            source_statuses,
            prior_visit_briefings,
        };
        let base_assignment = crate::visit_preparation::briefing_assignment(context, skills)?;
        let mut epoch = 0usize;
        let (job_id, scope) = loop {
            let candidate_job_id = if epoch == 0 {
                base_job_id.clone()
            } else {
                format!("{base_job_id}-r{epoch}")
            };
            let candidate_scope = PracticeScope {
                practice: practice.to_owned(),
                workspace: prep.workspace.clone(),
                job_id: candidate_job_id.clone(),
            };
            match self.practice_job(&candidate_scope).await? {
                Some(existing) => match existing.status {
                    WorkStatus::Pending | WorkStatus::Active => {
                        break (candidate_job_id, candidate_scope);
                    }
                    WorkStatus::Completed
                    | WorkStatus::Failed
                    | WorkStatus::Held
                    | WorkStatus::Cancelled => {
                        epoch += 1;
                    }
                },
                None => {
                    let mut assignment = base_assignment.clone();
                    assignment.key = if epoch == 0 {
                        format!("{}:{}", prep.workspace, base_assignment.key)
                    } else {
                        format!("{}:{}:r{epoch}", prep.workspace, base_assignment.key)
                    };
                    match self
                        .accept_practice_job(&candidate_scope, &assignment, roles, skills, auth)
                        .await?
                    {
                        Accepted::New | Accepted::Duplicate => {}
                    }
                    break (candidate_job_id, candidate_scope);
                }
            }
        };

        if let Some(row) = visit_preparation::Entity::find_by_id(prep.slot.pk_tuple())
            .one(&self.db)
            .await?
        {
            visit_preparation::ActiveModel {
                practice: Unchanged(row.practice),
                source_id: Unchanged(row.source_id),
                appointment_id: Unchanged(row.appointment_id),
                starts_at_unix: Unchanged(row.starts_at_unix),
                workspace: Unchanged(row.workspace),
                ends_at_unix: Unchanged(row.ends_at_unix),
                appointment_saved_version: Unchanged(row.appointment_saved_version),
                observed_source_version: Unchanged(row.observed_source_version),
                decision: Unchanged(row.decision),
                decided_at: Unchanged(row.decided_at),
                selection_job_id: Unchanged(row.selection_job_id),
                selection_work_id: Unchanged(row.selection_work_id),
                selection_idempotency_key: Unchanged(row.selection_idempotency_key),
                state: Set("preparing".to_owned()),
                briefing_job_id: Set(Some(job_id)),
                briefing_id: Unchanged(row.briefing_id),
                failed_reason: Set(None),
                updated_at: Set(now_unix),
            }
            .update(&self.db)
            .await?;
        }

        Ok(Some(scope))
    }

    async fn drive_practice_job_to_settlement<A: AuthStore, P: Provider>(
        &self,
        auth: &A,
        scope: &PracticeScope,
        provider: &mut P,
    ) -> Result<WorkStatus, BriefingError> {
        loop {
            let Some(job) = self.practice_job(scope).await? else {
                return Ok(WorkStatus::Cancelled);
            };
            match job.status {
                WorkStatus::Completed
                | WorkStatus::Failed
                | WorkStatus::Held
                | WorkStatus::Cancelled => return Ok(job.status),
                WorkStatus::Pending | WorkStatus::Active => {}
            }

            let Some(claim) = self
                .claim_practice_job(scope, WAKE_JOB_LEASE_SECONDS, auth)
                .await?
            else {
                let current = self
                    .practice_job(scope)
                    .await?
                    .map_or(WorkStatus::Cancelled, |j| j.status);
                return Ok(current);
            };

            let cancel = Cancellation::default();
            let mut host = PracticeJobHost {
                store: self.clone(),
                claim: claim.clone(),
                authority: auth,
            };
            match host.run(provider, RunLimits::default(), &cancel).await {
                Ok(()) => return Ok(WorkStatus::Completed),
                Err(failure) => {
                    let reason = AttemptFailure::of(&failure, &cancel);
                    self.settle_practice_job_attempt(
                        &claim,
                        reason,
                        super::delivery::Backoff::IMMEDIATE,
                    )
                    .await?;
                }
            }
        }
    }

    /// Runs one complete scheduled visit-preparation wake-up cycle for `practice`
    /// at practice-clock `now_unix`:
    /// 1. Reconciles schedule changes and retires ended `Today` visit concerns;
    /// 2. Evaluates undecided, deferred-due, or evidence-changed upcoming/in-progress
    ///    visits via `select_visit_preparations` on `roles.background`;
    /// 3. Recovers any interrupted `Preparing` briefing jobs and queues + executes
    ///    due briefing jobs (`prepare_at_unix <= now_unix < ends_at_unix`) via
    ///    `save_visit_briefing` on `roles.background`, marking exhausted jobs
    ///    `CouldNotFinish`.
    pub async fn wake_visit_preparation<A: AuthStore, P: Provider>(
        &self,
        auth: &A,
        practice: &str,
        roles: &ModelRoles,
        skills: &PhysicianSkills,
        provider: &mut P,
        now_unix: i64,
    ) -> Result<ScheduledWakeSummary, BriefingError> {
        let practice = practice.trim();
        if practice.is_empty() {
            return Err(BriefingError::InvalidScope);
        }

        self.reconcile_unfinished_visit_preparations(practice)
            .await?;
        let mut retired_concerns = self.reconcile_visit_concerns(practice, now_unix).await?;
        let schedule = self.practice_schedule(practice).await?;
        let mut selection_job = None;
        let mut queued_briefing_jobs = Vec::new();
        let mut completed_briefings = Vec::new();
        let mut failed_jobs = Vec::new();

        // 1. Run visit selection if any eligible upcoming/in-progress visits need a decision.
        let candidates = self
            .collect_selection_candidates(auth, practice, &schedule.appointments, now_unix)
            .await?;
        if !candidates.is_empty() {
            let host_workspace = candidates[0].workspace.clone();
            let fingerprint_input = serde_json::to_vec(
                &candidates
                    .iter()
                    .map(|c| {
                        (
                            &c.source_id,
                            &c.appointment_id,
                            c.starts_at_unix,
                            c.appointment_saved_version,
                            c.chart_version,
                            &c.prior_decision,
                        )
                    })
                    .collect::<Vec<_>>(),
            )
            .expect("selection fingerprint tuple serializes");
            let fingerprint = &crate::sha256_hex(&fingerprint_input)[..16];
            let base_job_id = selection_job_key(practice, fingerprint);
            let sel_context = VisitSelectionContext {
                practice: practice.to_owned(),
                evaluated_at_unix: now_unix,
                evaluated_at: format_unix_rfc3339(now_unix),
                schedule_standing: schedule.standing.clone(),
                candidates,
            };
            let mut epoch = 0usize;
            let sel_scope = loop {
                let candidate_job_id = if epoch == 0 {
                    base_job_id.clone()
                } else {
                    format!("{base_job_id}-r{epoch}")
                };
                let candidate_scope = PracticeScope {
                    practice: practice.to_owned(),
                    workspace: host_workspace.clone(),
                    job_id: candidate_job_id.clone(),
                };
                match self.practice_job(&candidate_scope).await? {
                    Some(existing) => match existing.status {
                        WorkStatus::Pending | WorkStatus::Active => {
                            break candidate_scope;
                        }
                        WorkStatus::Completed
                        | WorkStatus::Failed
                        | WorkStatus::Held
                        | WorkStatus::Cancelled => {
                            epoch += 1;
                        }
                    },
                    None => {
                        let assignment = crate::visit_preparation::selection_assignment(
                            &candidate_job_id,
                            sel_context.clone(),
                            skills,
                        )?;
                        match self
                            .accept_practice_job(&candidate_scope, &assignment, roles, skills, auth)
                            .await?
                        {
                            Accepted::New | Accepted::Duplicate => {}
                        }
                        break candidate_scope;
                    }
                }
            };
            let status = self
                .drive_practice_job_to_settlement(auth, &sel_scope, provider)
                .await?;
            selection_job = Some(sel_scope.clone());
            if matches!(status, WorkStatus::Failed | WorkStatus::Held) {
                failed_jobs.push(sel_scope);
            }
        }

        // 2. Queue due `Scheduled` visits (`prepare_at_unix <= now_unix < ends_at_unix`)
        //    and recover any interrupted `Preparing` briefing jobs.
        let appt_by_slot: BTreeMap<(String, String, i64), &SavedAppointment> = schedule
            .appointments
            .iter()
            .map(|a| {
                (
                    (
                        a.source_id.as_str().to_owned(),
                        a.appointment_id.as_str().to_owned(),
                        a.window.starts_at_unix(),
                    ),
                    a,
                )
            })
            .collect();

        let preparations = self
            .visit_preparations_for_practice_internal(practice)
            .await?;
        let mut jobs_to_drive: Vec<(VisitSlotKey, PracticeScope)> = Vec::new();

        for prep in preparations {
            let slot_tuple = (
                prep.slot.source_id.clone(),
                prep.slot.appointment_id.clone(),
                prep.slot.starts_at_unix,
            );
            let Some(appt) = appt_by_slot.get(&slot_tuple).copied() else {
                continue;
            };

            match &prep.state {
                VisitPreparationState::Scheduled {
                    prepare_at_unix, ..
                } => {
                    let is_due =
                        matches!(prep.decision, VisitPreparationDecision::PrepareNow { .. })
                            || *prepare_at_unix <= now_unix;
                    if is_due
                        && visit_can_start_preparation(
                            appt.status,
                            appt.window.ends_at_unix(),
                            now_unix,
                        )
                        && let Some(scope) = self
                            .accept_due_briefing_job(auth, appt, &prep, roles, skills, now_unix)
                            .await?
                    {
                        queued_briefing_jobs.push(scope.clone());
                        jobs_to_drive.push((prep.slot, scope));
                    }
                }
                VisitPreparationState::Preparing { job_id, .. } => {
                    let scope = PracticeScope {
                        practice: practice.to_owned(),
                        workspace: prep.workspace.clone(),
                        job_id: job_id.clone(),
                    };
                    jobs_to_drive.push((prep.slot, scope));
                }
                VisitPreparationState::Deferred { .. }
                | VisitPreparationState::Skipped { .. }
                | VisitPreparationState::Ready { .. }
                | VisitPreparationState::CouldNotFinish { .. }
                | VisitPreparationState::EndedVisitChanged { .. } => {}
            }
        }

        for (slot, scope) in jobs_to_drive {
            let status = self
                .drive_practice_job_to_settlement(auth, &scope, provider)
                .await?;
            match status {
                WorkStatus::Completed => {
                    if let Some(saved) = self.visit_briefing_internal(&slot).await? {
                        completed_briefings.push(saved.briefing_id);
                    }
                }
                WorkStatus::Failed | WorkStatus::Held => {
                    failed_jobs.push(scope);
                }
                WorkStatus::Pending | WorkStatus::Active | WorkStatus::Cancelled => {}
            }
        }

        // Reconcile any newly Failed/Held preparation rows and any concerns
        // whose visit ended during execution.
        self.reconcile_unfinished_visit_preparations(practice)
            .await?;
        retired_concerns.extend(self.reconcile_visit_concerns(practice, now_unix).await?);

        Ok(ScheduledWakeSummary {
            practice: practice.to_owned(),
            evaluated_at_unix: now_unix,
            selection_job,
            queued_briefing_jobs,
            completed_briefings,
            failed_jobs,
            retired_concerns,
        })
    }
}
