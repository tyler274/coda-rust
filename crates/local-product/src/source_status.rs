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

use serde::{Deserialize, Serialize};

use crate::physician_sources::{
    ChartPullReceipt, PatientSource, SourcePatient, SourceRecordKind, SourceRevision,
};
pub use crate::physician_sources::{
    FailedPart, PullOutcome, ReconnectReceipt, SourceAdapterError, SourceCommandKey,
    SourcePullOutput,
};
use crate::runtime::Store;

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

/// Monotonic ordering token issued before a patient pull begins so an older in-flight pull
/// cannot overwrite chart records, timestamps, or recovery standing committed by a newer pull
/// or practice reconnect.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PullTicket {
    /// Monotonic per-`(practice_id, source_id, source_patient_id)` pull sequence (`>= 1`).
    pub seq: i64,
    /// Practice-level `access_seq` observed when this pull began (`0` when unset).
    pub access_seq: i64,
}

/// Whether a patient pull committed as the newest observation or yielded to a newer pull
/// or practice reconnect.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullDisposition {
    /// The pull was the newest for this patient and source and updated stored state.
    #[default]
    Applied,
    /// A newer pull or practice reconnect superseded this in-flight pull; stored records,
    /// directory timestamps, and standing were left untouched.
    Superseded,
}

/// Decision from [`decide_pull_order`] for an arriving pull result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PullOrderOutcome {
    /// Persist this pull's records, directory state, and status at `applied_seq`.
    Apply {
        /// Monotonic `applied_pull_seq` to persist on `patient_source_status`.
        applied_seq: i64,
    },
    /// Leave persisted records, directory state, and status untouched because a newer pull
    /// or practice reconnect already superseded this pull.
    Superseded,
}

/// Single authoritative ordering rule for competing patient pulls on the same
/// `(practice_id, workspace_id, source_id)`.
pub fn decide_pull_order(
    ticket: Option<PullTicket>,
    existing_applied_seq: i64,
    existing_latest_attempted_at: Option<i64>,
    current_access: Option<&SourceAccessState>,
    current_access_seq: i64,
    attempt: &PullAttempt,
) -> PullOrderOutcome {
    if let Some(ticket) = ticket {
        if ticket.seq <= existing_applied_seq {
            return PullOrderOutcome::Superseded;
        }
        if matches!(attempt.outcome, PullOutcome::LapsedAccess { .. })
            && matches!(current_access, Some(SourceAccessState::Connected { .. }))
            && current_access_seq > ticket.access_seq
        {
            return PullOrderOutcome::Superseded;
        }
        PullOrderOutcome::Apply {
            applied_seq: ticket.seq,
        }
    } else {
        if existing_latest_attempted_at.is_some_and(|prev| prev > attempt.attempted_at) {
            return PullOrderOutcome::Superseded;
        }
        PullOrderOutcome::Apply {
            applied_seq: existing_applied_seq + 1,
        }
    }
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
    /// Whether this check committed or yielded to a newer pull or practice reconnect.
    #[serde(default)]
    pub applied: PullDisposition,
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
    /// Monotonic pull ticket issued before the adapter call, if coordinated.
    pub ticket: Option<PullTicket>,
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
    /// Monotonic pull ticket issued before the adapter call, if coordinated.
    pub ticket: Option<PullTicket>,
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
        let key = SourcePatientKey {
            practice_id: practice_id.to_owned(),
            source_id: source.source_id().to_owned(),
            source_patient_id: source_patient_id.to_owned(),
        };
        let (link, ticket) = store.begin_patient_pull(&key, target, now).await?;
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
            Some(ticket),
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
        validate_non_empty("practice_id", practice_id)?;
        let links = store
            .practice_source_links(practice_id, source.source_id())
            .await?;
        let mut patients = Vec::with_capacity(links.len());
        for link in links {
            let report = Self::check_patient(
                store,
                source,
                practice_id,
                &link.key.source_patient_id,
                LinkTarget::Exact(&link.workspace_id),
                now,
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
    reason = "pull application requires store, source, patient keys, output, timestamp, and pull ticket"
)]
async fn apply_pull_output(
    store: &Store,
    source: &dyn PatientSource,
    practice_id: &str,
    source_patient_id: &str,
    workspace_id: String,
    output: SourcePullOutput,
    now: i64,
    ticket: Option<PullTicket>,
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
                applied,
            ) = store
                .record_complete_pull_ordered(CompleteDelivery {
                    practice_id,
                    workspace_id: &workspace_id,
                    source_id: source.source_id(),
                    source_label: source.source_label(),
                    patient: &patient,
                    records: &records,
                    pulled_at: now,
                    ticket,
                })
                .await?;
            let access = store
                .practice_source_access(practice_id, source.source_id())
                .await?;
            let standing = derive_standing(access.as_ref(), Some(&status));
            Ok(PatientCheckReport {
                workspace_id,
                source_patient_id: source_patient_id.to_owned(),
                standing,
                applied,
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
                applied,
            ) = store
                .record_partial_pull_ordered(PartialDelivery {
                    practice_id,
                    workspace_id: &workspace_id,
                    source_id: source.source_id(),
                    source_label: source.source_label(),
                    patient: patient.as_ref(),
                    succeeded_kinds: &succeeded_kinds,
                    records: &records,
                    failed_parts: &failed_parts,
                    attempted_at: now,
                    ticket,
                })
                .await?;
            let access = store
                .practice_source_access(practice_id, source.source_id())
                .await?;
            let standing = derive_standing(access.as_ref(), Some(&status));
            Ok(PatientCheckReport {
                workspace_id,
                source_patient_id: source_patient_id.to_owned(),
                standing,
                applied,
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
            let (status, applied) = store
                .record_pull_status_ordered(
                    practice_id,
                    &workspace_id,
                    source.source_id(),
                    &attempt,
                    ticket,
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
                applied,
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
            let (status, applied) = store
                .record_pull_status_ordered(
                    practice_id,
                    &workspace_id,
                    source.source_id(),
                    &attempt,
                    ticket,
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
                applied,
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

    #[test]
    fn decide_pull_order_rejects_superseded_tickets_and_pre_reconnect_lapses() {
        let connected = SourceAccessState::Connected { connected_at: 200 };
        let lapsed_attempt = PullAttempt {
            attempted_at: 150,
            outcome: PullOutcome::LapsedAccess {
                reason: "token expired".into(),
            },
        };
        let ok_attempt = PullAttempt {
            attempted_at: 220,
            outcome: PullOutcome::Succeeded {
                missing_candidates: Vec::new(),
            },
        };

        assert_eq!(
            decide_pull_order(
                Some(PullTicket {
                    seq: 2,
                    access_seq: 1,
                }),
                3,
                Some(200),
                Some(&connected),
                1,
                &ok_attempt,
            ),
            PullOrderOutcome::Superseded,
        );
        assert_eq!(
            decide_pull_order(
                Some(PullTicket {
                    seq: 2,
                    access_seq: 1,
                }),
                1,
                Some(100),
                Some(&connected),
                2,
                &lapsed_attempt,
            ),
            PullOrderOutcome::Superseded,
        );
        assert_eq!(
            decide_pull_order(
                Some(PullTicket {
                    seq: 3,
                    access_seq: 2,
                }),
                1,
                Some(100),
                Some(&connected),
                2,
                &lapsed_attempt,
            ),
            PullOrderOutcome::Apply { applied_seq: 3 },
        );
        assert_eq!(
            decide_pull_order(None, 4, Some(300), Some(&connected), 2, &lapsed_attempt),
            PullOrderOutcome::Superseded,
        );
        assert_eq!(
            decide_pull_order(None, 4, Some(150), Some(&connected), 2, &ok_attempt),
            PullOrderOutcome::Apply { applied_seq: 5 },
        );
    }
}
