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

use harness::engine::{self, ToolOutcome};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
    TransactionTrait, Unchanged,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::chat_operations::{MAX_RETRIEVED_WORK_ITEMS, RecordLimit, RecordLimitKind};
use super::concerns::{
    CommandKey, Concern, ConcernError, ConcernId, ConcernMutation, ConcernUrgency,
    ExactRecordReference, RetireConcern, RetirementReason, SourceWork, WorkspaceAccess,
    load_concern, retire_concern_in_tx,
};
use super::entities::{
    practice_job, schedule_appointment, skill_package, visit_briefing, visit_concern,
    visit_preparation,
};
use super::jobs::{
    AssignmentError, PracticeAccessError, PracticeAuthority, PracticeJob, PracticeJobError,
    PracticeScope,
};
use super::receipt::{self, Receipt};
use super::schedule::{AppointmentStatus, ScheduleError, format_unix_rfc3339};
use super::understanding::SummaryCitation;
use super::{Authority, Store, WorkStatus, conversion_error, error, persistence_failure};
use crate::physician_chat::PhysicianSkills;
use crate::visit_preparation::{
    BriefingClaimInput, BriefingDecisionInput, SourceRefreshStatusSummary, VisitPreparationDecision,
};

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
    pub async fn reconcile_unfinished_visit_preparations(
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
}
