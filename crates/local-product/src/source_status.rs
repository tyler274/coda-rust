//! Practice-scoped source patient links, pull status, standing derivation,
//! outage aggregation, and connector check/reconnect operations.
//!
//! Each domain rule is stated once here:
//! - [`derive_standing`] combines practice-level [`SourceAccessState`] and
//!   per-patient [`PatientSourceRecordStatus`] into [`SourceStanding`].
//! - [`summarize_practice_source`] collapses a source that is down or needs
//!   reconnect for most of a practice's patients into a single
//!   [`PracticeSourceSummary::PracticeWideOutage`] with zero per-patient rows.
//! - [`SourceCoordinator`] executes patient checks, practice checks, and
//!   reconnect operations through [`PatientSource`] over [`Store`].

use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::time::Instant;

use crate::physician_sources::{
    ChartPullReceipt, PatientSource, SourcePatient, SourceRecordKind, SourceRevision,
};
pub use crate::physician_sources::{
    FailedPart, PullOutcome, ReconnectReceipt, SourceAdapterError, SourceCommandKey,
    SourcePullOutput,
};
use crate::runtime::Store;
use crate::runtime::source_status::release_reconnect_command_lease_db;

/// Errors returned by source patient linking, status persistence, and connector operations.
#[derive(Debug, thiserror::Error)]
pub enum SourceStatusError {
    /// A required practice, source, source-patient, or workspace identifier was blank.
    #[error("empty source status identity field `{field}`")]
    EmptyIdentity {
        /// Name of the blank field.
        field: &'static str,
    },
    /// The target workspace belongs to a different practice than the caller's practice.
    #[error(
        "workspace `{workspace_id}` belongs to practice `{owner_practice_id}`, not `{requested_practice_id}`"
    )]
    PracticeMismatch {
        /// Target workspace identifier.
        workspace_id: String,
        /// Practice that owns the workspace.
        owner_practice_id: String,
        /// Practice that attempted to read or write the workspace.
        requested_practice_id: String,
    },
    /// Linking attempted to bind an already-linked source patient or workspace to a conflicting identity.
    #[error(
        "source patient `{source_patient_id}` on `{source_id}` in practice `{practice_id}` conflicts with existing workspace `{existing_workspace_id}` (requested `{requested_workspace_id}`)"
    )]
    ConflictingWorkspaceLink {
        /// Owning practice identifier.
        practice_id: String,
        /// Source adapter identifier.
        source_id: String,
        /// Source-native patient identifier.
        source_patient_id: String,
        /// Already-linked workspace identifier.
        existing_workspace_id: String,
        /// Conflicting workspace identifier requested by caller.
        requested_workspace_id: String,
    },
    /// The source adapter refused a connector operation such as reconnect or patient listing.
    #[error("source `{source_id}` failed: {source}")]
    SourceAdapter {
        /// Source adapter identifier.
        source_id: String,
        /// Underlying typed adapter error cause.
        #[source]
        source: SourceAdapterError,
    },
    /// Active reconnect command execution lost lease ownership or was already completed.
    #[error(
        "reconnect command `{idempotency_key}` for physician `{physician_id}` in practice `{practice_id}` lost lease ownership"
    )]
    CommandLeaseLost {
        /// Owning practice identifier.
        practice_id: String,
        /// Physician who issued the command.
        physician_id: String,
        /// Client-supplied command idempotency key.
        idempotency_key: String,
    },
    /// Underlying database read, write, or decoding failure.
    #[error("{0}")]
    Database(#[from] sea_orm::DbErr),
}

/// External patient identity within one practice's connected source.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourcePatientKey {
    /// Practice identifier.
    pub practice_id: String,
    /// Source adapter identifier.
    pub source_id: String,
    /// Source's native patient identifier.
    pub source_patient_id: String,
}

impl SourcePatientKey {
    /// Validates that every component of the key is non-blank.
    pub fn validate(&self) -> Result<(), SourceStatusError> {
        validate_non_empty("practice_id", &self.practice_id)?;
        validate_non_empty("source_id", &self.source_id)?;
        validate_non_empty("source_patient_id", &self.source_patient_id)?;
        Ok(())
    }
}

/// Durable binding from a source patient in a practice to one Chords workspace.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourcePatientLink {
    /// External `(practice_id, source_id, source_patient_id)` key.
    pub key: SourcePatientKey,
    /// Bound Chords patient workspace identifier.
    pub workspace_id: String,
    /// Practice-clock Unix timestamp in seconds when first linked.
    pub linked_at: i64,
}

/// How a caller supplies a workspace identifier when resolving or creating a patient link.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkTarget<'a> {
    /// Reuse the already-linked workspace if one exists; otherwise create the link to this default workspace.
    ExistingOrDefault(&'a str),
    /// Require the link to resolve to this exact workspace, refusing if already linked elsewhere.
    Exact(&'a str),
}

impl<'a> LinkTarget<'a> {
    /// Candidate workspace identifier carried by the target.
    pub const fn workspace(self) -> &'a str {
        match self {
            Self::ExistingOrDefault(workspace) | Self::Exact(workspace) => workspace,
        }
    }
}

/// Recorded metadata for the latest pull attempt on `(practice_id, workspace_id, source_id)`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PullAttempt {
    /// Practice-clock Unix timestamp in seconds when the pull was attempted.
    pub attempted_at: i64,
    /// Outcome of the attempt.
    pub outcome: PullOutcome,
}

/// Practice-level connection access state for `(practice_id, source_id)`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum SourceAccessState {
    /// Credentials are active as of `connected_at`.
    Connected {
        /// Practice-clock Unix timestamp in seconds when connected or reconnected.
        connected_at: i64,
    },
    /// Credentials or authorization lapsed and require reconnecting.
    ReconnectNeeded {
        /// Practice-clock Unix timestamp in seconds when the lapse was detected.
        since: i64,
        /// Diagnostic reason reported by the source adapter.
        reason: String,
    },
}

/// Stored source pull status row for one patient workspace in one practice.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatientSourceRecordStatus {
    /// Owning practice identifier.
    pub practice_id: String,
    /// Patient workspace identifier.
    pub workspace_id: String,
    /// Source adapter identifier.
    pub source_id: String,
    /// Last complete (`PullOutcome::Succeeded`) pull timestamp; preserved across failed, partial, and lapsed attempts.
    pub last_good_read_at: Option<i64>,
    /// Most recent pull attempt, whether complete, partial, failed, or lapsed.
    pub latest_attempt: Option<PullAttempt>,
}

/// Effective read-side standing of one source for one patient, derived from practice access and patient pull status.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum SourceStanding {
    /// Linked or registered, with no pull attempt recorded yet.
    NotYetRead,
    /// Latest pull succeeded completely and practice access is active.
    UpToDate {
        /// Timestamp of the complete pull.
        last_good_read_at: i64,
        /// Candidate IDs from prior pulls absent on the latest pull.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        missing_candidates: Vec<String>,
    },
    /// Latest pull saved some parts while other parts failed.
    Partial {
        /// Timestamp of the last complete good pull, if any.
        last_good_read_at: Option<i64>,
        /// Timestamp of the partial attempt.
        attempted_at: i64,
        /// Resource families or parts that failed.
        failed_parts: Vec<FailedPart>,
        /// Candidate IDs absent among the parts that succeeded.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        missing_candidates: Vec<String>,
    },
    /// Latest pull failed completely; prior records remain as of `last_good_read_at`.
    Failed {
        /// Timestamp of the last complete good pull, if any.
        last_good_read_at: Option<i64>,
        /// Timestamp of the failed attempt.
        attempted_at: i64,
        /// Diagnostic failure reason.
        reason: String,
    },
    /// Practice or patient source access lapsed and needs reconnect.
    ReconnectNeeded {
        /// Timestamp of the last complete good pull, if any.
        last_good_read_at: Option<i64>,
        /// Timestamp of the attempt that detected lapsed access, if recorded on this patient.
        attempted_at: Option<i64>,
        /// Diagnostic access refusal reason.
        reason: String,
    },
}

/// Derives the authoritative [`SourceStanding`] for one patient's source from
/// practice-level access state and the patient's recorded status row.
pub fn derive_standing(
    access: Option<&SourceAccessState>,
    status: Option<&PatientSourceRecordStatus>,
) -> SourceStanding {
    if let Some(SourceAccessState::ReconnectNeeded { since, reason }) = access {
        let fresh_success_after_lapse = status.is_some_and(|row| {
            row.latest_attempt.as_ref().is_some_and(|attempt| {
                attempt.attempted_at > *since
                    && matches!(
                        attempt.outcome,
                        PullOutcome::Succeeded { .. } | PullOutcome::Partial { .. }
                    )
            })
        });
        if !fresh_success_after_lapse {
            let last_good_read_at = status.and_then(|row| row.last_good_read_at);
            let attempted_at = status
                .and_then(|row| row.latest_attempt.as_ref().map(|a| a.attempted_at))
                .or(Some(*since));
            let effective_reason = status
                .and_then(|row| match &row.latest_attempt {
                    Some(PullAttempt {
                        outcome: PullOutcome::LapsedAccess { reason: patient },
                        ..
                    }) => Some(patient.clone()),
                    Some(PullAttempt {
                        outcome:
                            PullOutcome::Succeeded { .. }
                            | PullOutcome::Partial { .. }
                            | PullOutcome::Failed { .. },
                        ..
                    })
                    | None => None,
                })
                .unwrap_or_else(|| reason.clone());
            return SourceStanding::ReconnectNeeded {
                last_good_read_at,
                attempted_at,
                reason: effective_reason,
            };
        }
    }

    let Some(status) = status else {
        return SourceStanding::NotYetRead;
    };

    let Some(attempt) = &status.latest_attempt else {
        return match status.last_good_read_at {
            Some(last_good_read_at) => SourceStanding::UpToDate {
                last_good_read_at,
                missing_candidates: Vec::new(),
            },
            None => SourceStanding::NotYetRead,
        };
    };

    match &attempt.outcome {
        PullOutcome::Succeeded { missing_candidates } => SourceStanding::UpToDate {
            last_good_read_at: status.last_good_read_at.unwrap_or(attempt.attempted_at),
            missing_candidates: missing_candidates.clone(),
        },
        PullOutcome::Partial {
            failed_parts,
            missing_candidates,
        } => SourceStanding::Partial {
            last_good_read_at: status.last_good_read_at,
            attempted_at: attempt.attempted_at,
            failed_parts: failed_parts.clone(),
            missing_candidates: missing_candidates.clone(),
        },
        PullOutcome::Failed { reason } => SourceStanding::Failed {
            last_good_read_at: status.last_good_read_at,
            attempted_at: attempt.attempted_at,
            reason: reason.clone(),
        },
        PullOutcome::LapsedAccess { reason } => SourceStanding::ReconnectNeeded {
            last_good_read_at: status.last_good_read_at,
            attempted_at: Some(attempt.attempted_at),
            reason: reason.clone(),
        },
    }
}

/// Nature of a practice-wide source outage.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PracticeOutageKind {
    /// Practice credentials or authorization lapsed and need reconnect.
    ReconnectNeeded {
        /// Diagnostic refusal reason.
        reason: String,
    },
    /// Upstream source is failing or partially degraded across most of the practice's patients.
    SourceDown {
        /// Diagnostic failure summary.
        reason: String,
    },
}

/// Practice-wide outage item emitted instead of per-patient failure rows when a source is down across the practice.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PracticeOutage {
    /// Owning practice identifier.
    pub practice_id: String,
    /// Source adapter identifier.
    pub source_id: String,
    /// Number of linked patients affected by the outage.
    pub affected_patients: usize,
    /// Total number of patients linked to this source in the practice.
    pub total_patients: usize,
    /// Outage classification and diagnostic reason.
    pub kind: PracticeOutageKind,
}

/// One patient-specific source issue reported only when the source is healthy for at least half the practice.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatientSourceIssue {
    /// Affected patient workspace identifier.
    pub workspace_id: String,
    /// Source adapter identifier.
    pub source_id: String,
    /// Patient's unhealthy source standing.
    pub standing: SourceStanding,
}

/// Aggregated practice-level summary for one source.
///
/// Structurally prevents returning both a practice-wide outage and per-patient
/// failure rows for the same source.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "scope", rename_all = "snake_case")]
pub enum PracticeSourceSummary {
    /// Every linked patient is healthy (`NotYetRead` or `UpToDate`) and practice access is active.
    Healthy {
        /// Source adapter identifier.
        source_id: String,
        /// Total linked patients for this source in the practice.
        total_patients: usize,
    },
    /// Source needs reconnect or is unhealthy for a strict majority of linked patients: one outage, zero per-patient rows.
    PracticeWideOutage(PracticeOutage),
    /// Unhealthy for half or fewer of linked patients: reported as individual patient issues.
    PatientIssues {
        /// Source adapter identifier.
        source_id: String,
        /// Total linked patients for this source in the practice.
        total_patients: usize,
        /// Per-patient issues (at most half of `total_patients`).
        issues: Vec<PatientSourceIssue>,
    },
}

/// Authoritative practice-level source aggregation rule.
///
/// Emits [`PracticeSourceSummary::PracticeWideOutage`] (and zero per-patient rows)
/// whenever practice-level access is [`SourceAccessState::ReconnectNeeded`] or
/// unhealthy patients (`Partial`, `Failed`, or `ReconnectNeeded`) form a strict
/// majority (`unhealthy * 2 > total_patients`) of the practice's linked patients.
pub fn summarize_practice_source(
    practice_id: &str,
    source_id: &str,
    access: Option<&SourceAccessState>,
    patients: &[(String, SourceStanding)],
) -> PracticeSourceSummary {
    let total_patients = patients.len();
    let mut issues = Vec::new();
    let mut reconnect_count = 0usize;

    for (workspace_id, standing) in patients {
        match standing {
            SourceStanding::NotYetRead | SourceStanding::UpToDate { .. } => {}
            SourceStanding::Partial { .. } | SourceStanding::Failed { .. } => {
                issues.push(PatientSourceIssue {
                    workspace_id: workspace_id.clone(),
                    source_id: source_id.to_owned(),
                    standing: standing.clone(),
                });
            }
            SourceStanding::ReconnectNeeded { .. } => {
                reconnect_count += 1;
                issues.push(PatientSourceIssue {
                    workspace_id: workspace_id.clone(),
                    source_id: source_id.to_owned(),
                    standing: standing.clone(),
                });
            }
        }
    }

    if let Some(SourceAccessState::ReconnectNeeded { reason, .. }) = access {
        return PracticeSourceSummary::PracticeWideOutage(PracticeOutage {
            practice_id: practice_id.to_owned(),
            source_id: source_id.to_owned(),
            affected_patients: issues.len().max(total_patients),
            total_patients,
            kind: PracticeOutageKind::ReconnectNeeded {
                reason: reason.clone(),
            },
        });
    }

    if issues.is_empty() {
        return PracticeSourceSummary::Healthy {
            source_id: source_id.to_owned(),
            total_patients,
        };
    }

    if total_patients > 0 && issues.len() * 2 > total_patients {
        // Practice-wide outage copy stays strictly practice-level so patient-specific
        // failure diagnostics never leak across panel membership boundaries.
        let kind = if reconnect_count * 2 > issues.len() {
            PracticeOutageKind::ReconnectNeeded {
                reason: "source access requires reconnect across most linked patients".to_owned(),
            }
        } else {
            PracticeOutageKind::SourceDown {
                reason: "source reads are failing across most linked patients".to_owned(),
            }
        };
        return PracticeSourceSummary::PracticeWideOutage(PracticeOutage {
            practice_id: practice_id.to_owned(),
            source_id: source_id.to_owned(),
            affected_patients: issues.len(),
            total_patients,
            kind,
        });
    }

    PracticeSourceSummary::PatientIssues {
        source_id: source_id.to_owned(),
        total_patients,
        issues,
    }
}

/// Complete stored and derived status snapshot for one `(practice_id, source_id)`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticeSourceReport {
    /// Practice identifier.
    pub practice_id: String,
    /// Source adapter identifier.
    pub source_id: String,
    /// Practice-level access state, if recorded.
    pub access: Option<SourceAccessState>,
    /// Authoritative practice-level summary.
    pub summary: PracticeSourceSummary,
    /// Linked `(workspace_id, SourceStanding)` entries ordered by workspace.
    pub patients: Vec<(String, SourceStanding)>,
}

/// Result of checking one patient's records through [`SourceCoordinator::check_patient`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatientCheckReport {
    /// Bound Chords patient workspace identifier.
    pub workspace_id: String,
    /// Source-native patient identifier.
    pub source_patient_id: String,
    /// Resulting patient source standing after the check.
    pub standing: SourceStanding,
    /// Number of candidate observations added or updated in the store.
    pub changed: usize,
    /// Number of source revisions delivered by the adapter on this check.
    pub records_delivered: usize,
    /// Previously active candidate IDs absent on this pull.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub missing_candidates: Vec<String>,
}

/// Result of checking all linked patients for one practice's source.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticeCheckReport {
    /// Practice identifier.
    pub practice_id: String,
    /// Source adapter identifier.
    pub source_id: String,
    /// Practice-level access state after the check.
    pub access: Option<SourceAccessState>,
    /// Authoritative practice summary (`PracticeWideOutage` collapses per-patient issues).
    pub summary: PracticeSourceSummary,
    /// Per-patient check reports collected during the run.
    pub patients: Vec<PatientCheckReport>,
}

/// Inputs for atomically persisting a complete source pull and its [`PullOutcome::Succeeded`] status.
#[derive(Clone, Copy, Debug)]
pub struct CompleteDelivery<'a> {
    /// Practice owning the workspace.
    pub practice_id: &'a str,
    /// Bound patient workspace identifier.
    pub workspace_id: &'a str,
    /// Source adapter identifier.
    pub source_id: &'a str,
    /// Physician-facing source label.
    pub source_label: &'a str,
    /// Patient identity reported by the source.
    pub patient: &'a SourcePatient,
    /// Admitted source revisions from the complete pull.
    pub records: &'a [SourceRevision],
    /// Practice-clock Unix second when the complete pull finished.
    pub pulled_at: i64,
}

/// Inputs for atomically persisting the succeeded portion of a partial source pull
/// and its [`PullOutcome::Partial`] status.
#[derive(Clone, Copy, Debug)]
pub struct PartialDelivery<'a> {
    /// Practice owning the workspace.
    pub practice_id: &'a str,
    /// Bound patient workspace identifier.
    pub workspace_id: &'a str,
    /// Source adapter identifier.
    pub source_id: &'a str,
    /// Physician-facing source label, used when `patient` identity is recorded.
    pub source_label: &'a str,
    /// Patient identity if the identity part of the pull succeeded.
    pub patient: Option<&'a SourcePatient>,
    /// Resource kinds or families that completed during this pull, including empty ones.
    pub succeeded_kinds: &'a [SourceRecordKind],
    /// Admitted source revisions from the parts that succeeded.
    pub records: &'a [SourceRevision],
    /// Resource families or parts that failed.
    pub failed_parts: &'a [FailedPart],
    /// Practice-clock Unix second when the partial pull ran.
    pub attempted_at: i64,
}

/// Result of reconnecting a practice's source and catching up linked patients.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconnectReport {
    /// Adapter confirmation receipt.
    pub receipt: ReconnectReceipt,
    /// Restored practice access state.
    pub access: SourceAccessState,
    /// Catch-up check across linked patients in the practice.
    pub check: PracticeCheckReport,
}

/// Inputs to [`SourceCoordinator::reconnect_source_command`].
#[derive(Clone, Copy)]
pub struct ReconnectCommand<'a> {
    /// Practice owning the command.
    pub practice_id: &'a str,
    /// Physician submitting the command.
    pub physician_id: &'a str,
    /// Client-supplied command idempotency key.
    pub idempotency_key: &'a str,
    /// Requested source adapter identifier.
    pub source_id: &'a str,
    /// Registered adapter for `source_id`, if present.
    pub source: Option<&'a dyn PatientSource>,
    /// Practice-clock Unix second when the command is evaluated.
    pub now: i64,
}

/// Outcome of an idempotent `reconnect_source` command backed by `physician_runtime_commands`
/// and `physician_runtime_reconnect_commands`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReconnectCommandOutcome {
    /// Newly executed reconnect and catch-up check, or a duplicate replay of the same command.
    Accepted {
        /// Original adapter confirmation receipt persisted for this command key.
        receipt: ReconnectReceipt,
        /// Practice-level source access state recorded by this command.
        access: SourceAccessState,
        /// Practice-level source summary recorded when this command completed.
        summary: PracticeSourceSummary,
    },
    /// The caller's `idempotency_key` already belongs to a different command or source.
    ConflictingKey,
    /// The requested `source_id` is not registered for execution.
    UnregisteredSource,
}

/// Lease duration and renewal cadence protecting an in-flight `reconnect_source` command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReconnectLeasePolicy {
    /// Lease duration in database-clock seconds (clamped to `>= 2`).
    pub lease_seconds: i64,
    /// Heartbeat interval between background lease renewals while execution remains active.
    pub renew_interval: Duration,
}

impl ReconnectLeasePolicy {
    /// Effective lease TTL in database-clock seconds (`>= 2` so integer-second clock ticks cannot
    /// expire an freshly acquired lease immediately).
    pub fn effective_lease_seconds(self) -> i64 {
        self.lease_seconds.max(2)
    }
}

impl Default for ReconnectLeasePolicy {
    fn default() -> Self {
        Self {
            lease_seconds: 30,
            renew_interval: Duration::from_secs(5),
        }
    }
}

/// Ownership token fencing every store write performed on behalf of an active
/// `reconnect_source` command so a superseded execution cannot write stale patient data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReconnectCommandFence<'a> {
    /// Practice owning the command.
    pub practice_id: &'a str,
    /// Physician who issued the command.
    pub physician_id: &'a str,
    /// Client-supplied idempotency key.
    pub idempotency_key: &'a str,
    /// Unique lease token assigned to the active runner.
    pub lease_owner: &'a str,
    /// Lease duration in database-clock seconds used when refreshing ownership on write.
    pub lease_seconds: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdapterPresence {
    Registered,
    Unregistered,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "phase", rename_all = "snake_case")]
pub(crate) enum ReconnectCommandState {
    Claimed {
        source_id: String,
    },
    Reconnected {
        receipt: ReconnectReceipt,
        access: SourceAccessState,
    },
    Completed {
        receipt: ReconnectReceipt,
        access: SourceAccessState,
        summary: PracticeSourceSummary,
    },
}

pub(crate) enum ReconnectCommandClaim {
    Execute {
        lease_owner: String,
        state: ReconnectCommandState,
    },
    AlreadyCompleted {
        receipt: ReconnectReceipt,
        access: SourceAccessState,
        summary: PracticeSourceSummary,
    },
    WaitForActiveLease,
    ConflictingKey,
    UnregisteredSource,
}

struct ReconnectLeaseGuard {
    armed: bool,
    db: sea_orm::DatabaseConnection,
    practice_id: String,
    physician_id: String,
    key: String,
    lease_owner: String,
}

impl ReconnectLeaseGuard {
    fn disarm(&mut self) {
        self.armed = false;
    }

    async fn release_now(&mut self) {
        if !self.armed {
            return;
        }
        self.armed = false;
        let _ = release_reconnect_command_lease_db(
            &self.db,
            &self.practice_id,
            &self.physician_id,
            &self.key,
            &self.lease_owner,
        )
        .await;
    }
}

impl Drop for ReconnectLeaseGuard {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        self.armed = false;
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            let db = self.db.clone();
            let practice_id = self.practice_id.clone();
            let physician_id = self.physician_id.clone();
            let key = self.key.clone();
            let lease_owner = self.lease_owner.clone();
            handle.spawn(async move {
                let _ = release_reconnect_command_lease_db(
                    &db,
                    &practice_id,
                    &physician_id,
                    &key,
                    &lease_owner,
                )
                .await;
            });
        }
    }
}

/// Coordinates patient-link resolution, chart pulling, status recording, and reconnects
/// across [`Store`] and any [`PatientSource`] implementation.
pub struct SourceCoordinator;

impl SourceCoordinator {
    /// Resolves (or creates) the practice-scoped patient link, pulls the patient's chart
    /// through `source`, persists any fetched records, and records the attempt outcome.
    ///
    /// Cross-practice or conflicting workspace attempts fail before calling `source` or
    /// writing any chart records.
    pub async fn check_patient(
        store: &Store,
        source: &dyn PatientSource,
        practice_id: &str,
        source_patient_id: &str,
        target: LinkTarget<'_>,
        now: i64,
    ) -> Result<PatientCheckReport, SourceStatusError> {
        Self::check_patient_fenced(
            store,
            source,
            practice_id,
            source_patient_id,
            target,
            now,
            None,
        )
        .await
    }

    /// Executes [`Self::check_patient`] fenced by `fence` when running inside an active
    /// `reconnect_source` command so every link resolution and delivery write atomically
    /// verifies lease ownership before committing.
    pub async fn check_patient_fenced(
        store: &Store,
        source: &dyn PatientSource,
        practice_id: &str,
        source_patient_id: &str,
        target: LinkTarget<'_>,
        now: i64,
        fence: Option<ReconnectCommandFence<'_>>,
    ) -> Result<PatientCheckReport, SourceStatusError> {
        let key = SourcePatientKey {
            practice_id: practice_id.to_owned(),
            source_id: source.source_id().to_owned(),
            source_patient_id: source_patient_id.to_owned(),
        };
        let link = store
            .resolve_or_link_patient_fenced(&key, target, now, fence)
            .await?;
        let output = source
            .pull_patient(practice_id, source_patient_id, &link.workspace_id)
            .await;
        apply_pull_output(
            store,
            source,
            practice_id,
            source_patient_id,
            link.workspace_id,
            output,
            now,
            fence,
        )
        .await
    }

    /// Loads or refreshes one patient from `source` during bulk onboarding:
    /// resolves or creates the practice-scoped source-patient link so failed, lapsed, or
    /// identity-less initial pulls remain addressable by [`Self::check_practice`] and
    /// [`Self::reconnect_source`], while keeping directory identity (`source_pulls`) withheld
    /// until `source` delivers a [`SourcePatient`].
    pub async fn onboard_or_check_patient(
        store: &Store,
        source: &dyn PatientSource,
        practice_id: &str,
        source_patient_id: &str,
        target: LinkTarget<'_>,
        now: i64,
    ) -> Result<PatientCheckReport, SourceStatusError> {
        Self::check_patient(store, source, practice_id, source_patient_id, target, now).await
    }

    /// Checks every patient linked to `(practice_id, source.source_id())` and computes
    /// the authoritative [`PracticeSourceSummary`].
    pub async fn check_practice(
        store: &Store,
        source: &dyn PatientSource,
        practice_id: &str,
        now: i64,
    ) -> Result<PracticeCheckReport, SourceStatusError> {
        Self::check_practice_fenced(store, source, practice_id, now, None).await
    }

    /// Executes [`Self::check_practice`] fenced by `fence` when running inside an active
    /// `reconnect_source` command so a superseded execution aborts without writing patient data.
    pub async fn check_practice_fenced(
        store: &Store,
        source: &dyn PatientSource,
        practice_id: &str,
        now: i64,
        fence: Option<ReconnectCommandFence<'_>>,
    ) -> Result<PracticeCheckReport, SourceStatusError> {
        validate_non_empty("practice_id", practice_id)?;
        if let Some(active_fence) = fence {
            store.renew_reconnect_command_lease(active_fence).await?;
        }
        let links = store
            .practice_source_links(practice_id, source.source_id())
            .await?;
        let mut patients = Vec::with_capacity(links.len());
        for link in links {
            let report = Self::check_patient_fenced(
                store,
                source,
                practice_id,
                &link.key.source_patient_id,
                LinkTarget::Exact(&link.workspace_id),
                now,
                fence,
            )
            .await?;
            patients.push(report);
        }
        let snapshot = store
            .practice_source_report(practice_id, source.source_id())
            .await?;
        Ok(PracticeCheckReport {
            practice_id: practice_id.to_owned(),
            source_id: source.source_id().to_owned(),
            access: snapshot.access,
            summary: snapshot.summary,
            patients,
        })
    }

    /// Reconnects `source` for `practice_id`, restores [`SourceAccessState::Connected`],
    /// and checks all linked patients in the practice.
    pub async fn reconnect_source(
        store: &Store,
        source: &dyn PatientSource,
        practice_id: &str,
        now: i64,
    ) -> Result<ReconnectReport, SourceStatusError> {
        validate_non_empty("practice_id", practice_id)?;
        let receipt = source
            .reconnect(practice_id, now, None)
            .await
            .map_err(|error| SourceStatusError::SourceAdapter {
                source_id: source.source_id().to_owned(),
                source: error,
            })?;
        let access = SourceAccessState::Connected {
            connected_at: receipt.reconnected_at,
        };
        store
            .set_practice_source_access(practice_id, source.source_id(), &access, now)
            .await?;
        let check = Self::check_practice(store, source, practice_id, now).await?;
        Ok(ReconnectReport {
            receipt,
            access,
            check,
        })
    }

    /// Executes an idempotent `reconnect_source` command scoped to `(practice_id, physician_id)`
    /// and coordinated durably in `physician_runtime_commands` + `physician_runtime_reconnect_commands`.
    ///
    /// - Conflicting key reuse is detected before adapter lookup or side effects.
    /// - Concurrent duplicate calls serialize on a durable lease renewed periodically throughout
    ///   adapter reconnect and practice catch-up so slow work never loses ownership while alive.
    /// - Adapter reconnect receives `Some(SourceCommandKey)` so pre-checkpoint interruptions
    ///   recover the original receipt on retry without repeating the external side effect.
    /// - Every patient link resolution and delivery write is fenced by `(idempotency_key, lease_owner)`.
    /// - Replaying an accepted key returns its original persisted [`ReconnectReceipt`].
    pub async fn reconnect_source_command(
        store: &Store,
        cmd: ReconnectCommand<'_>,
    ) -> Result<ReconnectCommandOutcome, SourceStatusError> {
        Self::reconnect_source_command_with_lease(store, cmd, ReconnectLeasePolicy::default()).await
    }

    /// Executes [`Self::reconnect_source_command`] with an explicit [`ReconnectLeasePolicy`].
    pub async fn reconnect_source_command_with_lease(
        store: &Store,
        cmd: ReconnectCommand<'_>,
        policy: ReconnectLeasePolicy,
    ) -> Result<ReconnectCommandOutcome, SourceStatusError> {
        validate_non_empty("practice_id", cmd.practice_id)?;
        validate_non_empty("physician_id", cmd.physician_id)?;
        validate_non_empty("idempotency_key", cmd.idempotency_key)?;
        validate_non_empty("source_id", cmd.source_id)?;

        let lease_seconds = policy.effective_lease_seconds();
        let renew_every = policy.renew_interval.max(Duration::from_millis(10));
        let command_payload = json!({
            "scope": {
                "practice": cmd.practice_id,
                "physician": cmd.physician_id,
            },
            "reconnect_source": {
                "source_id": cmd.source_id,
            },
        });
        let presence = if cmd.source.is_some() {
            AdapterPresence::Registered
        } else {
            AdapterPresence::Unregistered
        };

        let (lease_owner, state) = loop {
            match store
                .claim_reconnect_command(
                    cmd.practice_id,
                    cmd.physician_id,
                    cmd.idempotency_key,
                    cmd.source_id,
                    presence,
                    &command_payload,
                    lease_seconds,
                )
                .await?
            {
                ReconnectCommandClaim::ConflictingKey => {
                    return Ok(ReconnectCommandOutcome::ConflictingKey);
                }
                ReconnectCommandClaim::UnregisteredSource => {
                    return Ok(ReconnectCommandOutcome::UnregisteredSource);
                }
                ReconnectCommandClaim::AlreadyCompleted {
                    receipt,
                    access,
                    summary,
                } => {
                    return Ok(ReconnectCommandOutcome::Accepted {
                        receipt,
                        access,
                        summary,
                    });
                }
                ReconnectCommandClaim::WaitForActiveLease => {
                    tokio::time::sleep(Duration::from_millis(20)).await;
                }
                ReconnectCommandClaim::Execute { lease_owner, state } => {
                    break (lease_owner, state);
                }
            }
        };

        let Some(source) = cmd.source else {
            return Ok(ReconnectCommandOutcome::UnregisteredSource);
        };

        let mut guard = ReconnectLeaseGuard {
            armed: true,
            db: store.connection().clone(),
            practice_id: cmd.practice_id.to_owned(),
            physician_id: cmd.physician_id.to_owned(),
            key: cmd.idempotency_key.to_owned(),
            lease_owner: lease_owner.clone(),
        };

        let fence = ReconnectCommandFence {
            practice_id: cmd.practice_id,
            physician_id: cmd.physician_id,
            idempotency_key: cmd.idempotency_key,
            lease_owner: &lease_owner,
            lease_seconds,
        };
        let command_key = SourceCommandKey {
            physician_id: cmd.physician_id,
            idempotency_key: cmd.idempotency_key,
        };

        let work_fut = async {
            let (receipt, access) = match state {
                ReconnectCommandState::Claimed { .. } => {
                    let receipt = source
                        .reconnect(cmd.practice_id, cmd.now, Some(command_key))
                        .await
                        .map_err(|error| SourceStatusError::SourceAdapter {
                            source_id: source.source_id().to_owned(),
                            source: error,
                        })?;
                    let access = SourceAccessState::Connected {
                        connected_at: receipt.reconnected_at,
                    };
                    store
                        .record_reconnect_command_reconnected(fence, &receipt, &access)
                        .await?;
                    (receipt, access)
                }
                ReconnectCommandState::Reconnected { receipt, access } => (receipt, access),
                ReconnectCommandState::Completed {
                    receipt,
                    access,
                    summary,
                } => {
                    return Ok(ReconnectCommandOutcome::Accepted {
                        receipt,
                        access,
                        summary,
                    });
                }
            };

            let check =
                Self::check_practice_fenced(store, source, cmd.practice_id, cmd.now, Some(fence))
                    .await?;
            let summary = check.summary;
            store
                .complete_reconnect_command(
                    fence,
                    ReconnectCommandState::Completed {
                        receipt: receipt.clone(),
                        access: access.clone(),
                        summary: summary.clone(),
                    },
                )
                .await?;
            Ok(ReconnectCommandOutcome::Accepted {
                receipt,
                access,
                summary,
            })
        };
        tokio::pin!(work_fut);

        let mut ticker = tokio::time::interval_at(Instant::now() + renew_every, renew_every);
        let run: Result<ReconnectCommandOutcome, SourceStatusError> = loop {
            tokio::select! {
                biased;
                outcome = &mut work_fut => break outcome,
                _ = ticker.tick() => {
                    if let Err(error) = store.renew_reconnect_command_lease(fence).await {
                        break Err(error);
                    }
                }
            }
        };

        match run {
            Ok(outcome) => {
                guard.disarm();
                Ok(outcome)
            }
            Err(error) => {
                guard.release_now().await;
                Err(error)
            }
        }
    }
}

pub(crate) fn validate_non_empty(
    field: &'static str,
    value: &str,
) -> Result<(), SourceStatusError> {
    if value.trim().is_empty() {
        return Err(SourceStatusError::EmptyIdentity { field });
    }
    Ok(())
}

#[expect(
    clippy::too_many_arguments,
    reason = "pull application requires store, source, patient keys, output, timestamp, and optional command fence"
)]
async fn apply_pull_output(
    store: &Store,
    source: &dyn PatientSource,
    practice_id: &str,
    source_patient_id: &str,
    workspace_id: String,
    output: SourcePullOutput,
    now: i64,
    fence: Option<ReconnectCommandFence<'_>>,
) -> Result<PatientCheckReport, SourceStatusError> {
    match output {
        SourcePullOutput::Complete { patient, records } => {
            let records_delivered = records.len();
            let (
                ChartPullReceipt {
                    changed,
                    missing_candidates,
                },
                status,
            ) = store
                .record_complete_pull_fenced(
                    CompleteDelivery {
                        practice_id,
                        workspace_id: &workspace_id,
                        source_id: source.source_id(),
                        source_label: source.source_label(),
                        patient: &patient,
                        records: &records,
                        pulled_at: now,
                    },
                    fence,
                )
                .await?;
            let access = store
                .practice_source_access(practice_id, source.source_id())
                .await?;
            let standing = derive_standing(access.as_ref(), Some(&status));
            Ok(PatientCheckReport {
                workspace_id,
                source_patient_id: source_patient_id.to_owned(),
                standing,
                changed,
                records_delivered,
                missing_candidates,
            })
        }
        SourcePullOutput::Partial {
            patient,
            succeeded_kinds,
            records,
            failed_parts,
        } => {
            let records_delivered = records.len();
            let (
                ChartPullReceipt {
                    changed,
                    missing_candidates,
                },
                status,
            ) = store
                .record_partial_pull_fenced(
                    PartialDelivery {
                        practice_id,
                        workspace_id: &workspace_id,
                        source_id: source.source_id(),
                        source_label: source.source_label(),
                        patient: patient.as_ref(),
                        succeeded_kinds: &succeeded_kinds,
                        records: &records,
                        failed_parts: &failed_parts,
                        attempted_at: now,
                    },
                    fence,
                )
                .await?;
            let access = store
                .practice_source_access(practice_id, source.source_id())
                .await?;
            let standing = derive_standing(access.as_ref(), Some(&status));
            Ok(PatientCheckReport {
                workspace_id,
                source_patient_id: source_patient_id.to_owned(),
                standing,
                changed,
                records_delivered,
                missing_candidates,
            })
        }
        SourcePullOutput::Failed { error } => {
            let attempt = PullAttempt {
                attempted_at: now,
                outcome: PullOutcome::Failed {
                    reason: error.to_string(),
                },
            };
            let status = store
                .record_pull_status_fenced(
                    practice_id,
                    &workspace_id,
                    source.source_id(),
                    &attempt,
                    fence,
                )
                .await?;
            let access = store
                .practice_source_access(practice_id, source.source_id())
                .await?;
            let standing = derive_standing(access.as_ref(), Some(&status));
            Ok(PatientCheckReport {
                workspace_id,
                source_patient_id: source_patient_id.to_owned(),
                standing,
                changed: 0,
                records_delivered: 0,
                missing_candidates: Vec::new(),
            })
        }
        SourcePullOutput::LapsedAccess { error } => {
            let attempt = PullAttempt {
                attempted_at: now,
                outcome: PullOutcome::LapsedAccess {
                    reason: error.to_string(),
                },
            };
            let status = store
                .record_pull_status_fenced(
                    practice_id,
                    &workspace_id,
                    source.source_id(),
                    &attempt,
                    fence,
                )
                .await?;
            let access = store
                .practice_source_access(practice_id, source.source_id())
                .await?;
            let standing = derive_standing(access.as_ref(), Some(&status));
            Ok(PatientCheckReport {
                workspace_id,
                source_patient_id: source_patient_id.to_owned(),
                standing,
                changed: 0,
                records_delivered: 0,
                missing_candidates: Vec::new(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn status_with(
        last_good_read_at: Option<i64>,
        attempt: Option<PullAttempt>,
    ) -> PatientSourceRecordStatus {
        PatientSourceRecordStatus {
            practice_id: "practice-1".into(),
            workspace_id: "pw-1".into(),
            source_id: "ehr-1".into(),
            last_good_read_at,
            latest_attempt: attempt,
        }
    }

    #[test]
    fn derive_standing_preserves_last_good_read_across_failed_partial_and_lapsed_attempts() {
        let connected = SourceAccessState::Connected { connected_at: 100 };

        let failed = status_with(
            Some(120),
            Some(PullAttempt {
                attempted_at: 180,
                outcome: PullOutcome::Failed {
                    reason: "upstream timeout".into(),
                },
            }),
        );
        assert_eq!(
            derive_standing(Some(&connected), Some(&failed)),
            SourceStanding::Failed {
                last_good_read_at: Some(120),
                attempted_at: 180,
                reason: "upstream timeout".into(),
            }
        );

        let partial = status_with(
            Some(120),
            Some(PullAttempt {
                attempted_at: 200,
                outcome: PullOutcome::Partial {
                    failed_parts: vec![FailedPart {
                        part: "DocumentReference".into(),
                        reason: "502 bad gateway".into(),
                    }],
                    missing_candidates: Vec::new(),
                },
            }),
        );
        assert_eq!(
            derive_standing(Some(&connected), Some(&partial)),
            SourceStanding::Partial {
                last_good_read_at: Some(120),
                attempted_at: 200,
                failed_parts: vec![FailedPart {
                    part: "DocumentReference".into(),
                    reason: "502 bad gateway".into(),
                }],
                missing_candidates: Vec::new(),
            }
        );

        let lapsed_access = SourceAccessState::ReconnectNeeded {
            since: 220,
            reason: "refresh token expired".into(),
        };
        assert_eq!(
            derive_standing(Some(&lapsed_access), Some(&partial)),
            SourceStanding::ReconnectNeeded {
                last_good_read_at: Some(120),
                attempted_at: Some(200),
                reason: "refresh token expired".into(),
            }
        );

        let patient_lapsed_at_reconnect = status_with(
            Some(120),
            Some(PullAttempt {
                attempted_at: 300,
                outcome: PullOutcome::LapsedAccess {
                    reason: "patient consent scope revoked".into(),
                },
            }),
        );
        let reconnected = SourceAccessState::Connected { connected_at: 300 };
        assert_eq!(
            derive_standing(Some(&reconnected), Some(&patient_lapsed_at_reconnect)),
            SourceStanding::ReconnectNeeded {
                last_good_read_at: Some(120),
                attempted_at: Some(300),
                reason: "patient consent scope revoked".into(),
            }
        );
    }

    #[test]
    fn summarize_practice_source_collapses_majority_failures_into_one_outage_without_patient_rows()
    {
        let connected = SourceAccessState::Connected { connected_at: 100 };
        let up_to_date = SourceStanding::UpToDate {
            last_good_read_at: 100,
            missing_candidates: Vec::new(),
        };
        let failed = SourceStanding::Failed {
            last_good_read_at: Some(100),
            attempted_at: 200,
            reason: "MRN-9921 chart directory missing".into(),
        };

        // 2 of 4 (<= half) -> per-patient issues, not a practice outage.
        let minority = vec![
            ("pw-1".into(), failed.clone()),
            ("pw-2".into(), failed.clone()),
            ("pw-3".into(), up_to_date.clone()),
            ("pw-4".into(), up_to_date.clone()),
        ];
        match summarize_practice_source("practice-1", "ehr-1", Some(&connected), &minority) {
            PracticeSourceSummary::PatientIssues {
                total_patients,
                issues,
                ..
            } => {
                assert_eq!(total_patients, 4);
                assert_eq!(issues.len(), 2);
            }
            other => panic!("expected PatientIssues, got {other:?}"),
        }

        // 3 of 4 (> half) -> single PracticeWideOutage, zero per-patient rows,
        // and neutral practice-level copy with zero patient-specific diagnostic strings.
        let majority = vec![
            ("pw-1".into(), failed.clone()),
            ("pw-2".into(), failed.clone()),
            ("pw-3".into(), failed),
            ("pw-4".into(), up_to_date.clone()),
        ];
        let outage_summary =
            summarize_practice_source("practice-1", "ehr-1", Some(&connected), &majority);
        assert_eq!(
            outage_summary,
            PracticeSourceSummary::PracticeWideOutage(PracticeOutage {
                practice_id: "practice-1".into(),
                source_id: "ehr-1".into(),
                affected_patients: 3,
                total_patients: 4,
                kind: PracticeOutageKind::SourceDown {
                    reason: "source reads are failing across most linked patients".into(),
                },
            })
        );
        assert!(
            !serde_json::to_string(&outage_summary)
                .unwrap()
                .contains("MRN-9921")
        );

        // 2 of 3 LapsedAccess + 1 UpToDate at fixed timestamp 200:
        // practice summary is PracticeWideOutage(ReconnectNeeded), while patient 3 remains UpToDate.
        let lapsed_p1 = derive_standing(
            Some(&connected),
            Some(&status_with(
                Some(100),
                Some(PullAttempt {
                    attempted_at: 200,
                    outcome: PullOutcome::LapsedAccess {
                        reason: "MRN-1001 token revoked".into(),
                    },
                }),
            )),
        );
        let lapsed_p2 = derive_standing(
            Some(&connected),
            Some(&status_with(
                Some(100),
                Some(PullAttempt {
                    attempted_at: 200,
                    outcome: PullOutcome::LapsedAccess {
                        reason: "MRN-1002 token revoked".into(),
                    },
                }),
            )),
        );
        let ok_p3 = derive_standing(
            Some(&connected),
            Some(&status_with(
                Some(200),
                Some(PullAttempt {
                    attempted_at: 200,
                    outcome: PullOutcome::Succeeded {
                        missing_candidates: Vec::new(),
                    },
                }),
            )),
        );
        assert_eq!(
            ok_p3,
            SourceStanding::UpToDate {
                last_good_read_at: 200,
                missing_candidates: Vec::new(),
            }
        );
        let mixed_same_ts = vec![
            ("pw-1".into(), lapsed_p1),
            ("pw-2".into(), lapsed_p2),
            ("pw-3".into(), ok_p3),
        ];
        assert_eq!(
            summarize_practice_source("practice-1", "ehr-1", Some(&connected), &mixed_same_ts),
            PracticeSourceSummary::PracticeWideOutage(PracticeOutage {
                practice_id: "practice-1".into(),
                source_id: "ehr-1".into(),
                affected_patients: 2,
                total_patients: 3,
                kind: PracticeOutageKind::ReconnectNeeded {
                    reason: "source access requires reconnect across most linked patients".into(),
                },
            })
        );
    }
}
