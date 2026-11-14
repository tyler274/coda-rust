//! Practice schedule reads, patient matching, and versioned saved appointments.
//!
//! Appointments enter Coda through [`ScheduleSource`]. Each sync resolves every
//! appointment against seeded schedule patient links and the practice's current
//! roster (`AuthStore::practice_patients`), never registering new patients on
//! the roster. Unmatched appointments stay visible on the saved schedule as
//! [`PatientMatch::Unmatched`].
//!
//! All appointment writes and source standing updates commit in one Postgres
//! transaction under the `(practice, source_id)` row lock on
//! `physician_runtime_schedule_sources`:
//! - Reschedules and cancellations update the saved appointment and bump its
//!   `saved_version`; replaying unchanged bytes updates `as_of` without
//!   duplicating rows or incrementing `saved_version`.
//! - A complete read marks previously [`AppointmentStatus::Scheduled`]
//!   appointments omitted from the delivery as [`AppointmentStatus::Cancelled`]
//!   (`source_status = "withdrawn"`).
//! - A partial read saves the appointments it received, preserves any prior
//!   saved appointments omitted from the partial subset, and records every
//!   named [`ScheduleReadIssue`].
//! - A failed read leaves previously saved appointments and their `"as of"`
//!   timestamp untouched while recording [`ScheduleFreshness::RefreshFailed`],
//!   or records [`ScheduleStanding::Unavailable`] when no good read has ever
//!   completed.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use async_trait::async_trait;
use physician_auth::{AuthStore, StoreUnavailable};
use sea_orm::sea_query::StringLen;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set, TransactionTrait, sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Store;
use super::entities::{schedule_appointment, schedule_patient_link, schedule_source};
use crate::physician_sources::RecordDate;

/// Domain validation errors for schedule identifiers, windows, and deliveries.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ScheduleDomainError {
    /// Practice identifier was blank.
    #[error("practice identifier must not be empty")]
    EmptyPractice,
    /// Schedule source identifier was blank.
    #[error("schedule source identifier must not be empty")]
    EmptySourceId,
    /// Schedule source display label was blank.
    #[error("schedule source label must not be empty")]
    EmptySourceLabel,
    /// Native appointment identifier was blank.
    #[error("appointment identifier must not be empty")]
    EmptyAppointmentId,
    /// Source-native patient identifier was blank.
    #[error("source patient identifier must not be empty")]
    EmptySourcePatientId,
    /// Coda workspace identifier was blank.
    #[error("patient workspace identifier must not be empty")]
    EmptyWorkspaceId,
    /// Appointment label was blank.
    #[error("appointment label must not be empty")]
    EmptyLabel,
    /// Retained source status text was blank.
    #[error("appointment source status must not be empty")]
    EmptySourceStatus,
    /// Explicit source revision string was blank.
    #[error("appointment source revision must not be empty")]
    EmptySourceRevision,
    /// Retained source provenance was null.
    #[error("appointment provenance must not be null")]
    NullProvenance,
    /// Retained content bytes did not match the recorded SHA-256 digest.
    #[error("appointment {appointment_id} content digest mismatch")]
    DigestMismatch {
        /// Appointment whose digest failed verification.
        appointment_id: String,
    },
    /// RFC 3339 timestamp failed parsing or calendar bounds checks.
    #[error("invalid appointment timestamp {field} ({value:?}): {reason}")]
    InvalidTimestamp {
        /// Field name (`starts_at` or `ends_at`).
        field: &'static str,
        /// Raw timestamp string.
        value: String,
        /// Specific validation failure.
        reason: &'static str,
    },
    /// Numeric component of an RFC 3339 timestamp could not be parsed.
    #[error("invalid appointment timestamp {field} ({value:?}): {component}: {source}")]
    InvalidTimestampNumber {
        /// Field name (`starts_at` or `ends_at`).
        field: &'static str,
        /// Raw timestamp string.
        value: String,
        /// Component name (`year`, `month`, `day`, `hour`, `minute`, `second`).
        component: &'static str,
        /// Underlying integer parse error.
        #[source]
        source: std::num::ParseIntError,
    },
    /// Appointment end preceded its start.
    #[error("appointment end ({ends_at}) precedes start ({starts_at})")]
    InvertedWindow {
        /// Canonical start timestamp.
        starts_at: String,
        /// Canonical end timestamp.
        ends_at: String,
    },
    /// Partial schedule read carried zero failure entries.
    #[error("partial schedule delivery must name at least one failed item")]
    EmptyPartialFailures,
    /// Named schedule read failure had a blank target or reason.
    #[error("schedule read failure target and reason must not be empty")]
    BlankFailureDetail,
    /// Two appointments in one delivery reused an appointment ID with different content.
    #[error("duplicate appointment identifier {appointment_id} with conflicting content")]
    DuplicateAppointmentId {
        /// Conflicting appointment identifier.
        appointment_id: String,
    },
}

/// Errors returned when reading a [`ScheduleSource`].
#[derive(Debug, thiserror::Error)]
pub enum ScheduleSourceError {
    /// Transport or backing file I/O failed before a schedule could be read.
    #[error("schedule source {source_id} could not be read: {source}")]
    Io {
        /// Source identifier.
        source_id: SourceId,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },
    /// Upstream calendar service refused or failed the read.
    #[error("schedule source {source_id} unavailable: {reason}")]
    Unavailable {
        /// Source identifier.
        source_id: SourceId,
        /// Failure detail reported by the adapter.
        reason: String,
    },
}

/// Errors returned by practice schedule link seeding, sync, and readback.
#[derive(Debug, thiserror::Error)]
pub enum ScheduleError {
    /// Schedule domain invariant failed validation.
    #[error("{0}")]
    Domain(#[from] ScheduleDomainError),
    /// Practice roster lookup failed in the auth store.
    #[error("practice roster unavailable: {0}")]
    AuthStore(#[source] StoreUnavailable),
    /// Patient link seeding named a workspace that is not on the practice's roster.
    #[error("workspace {workspace} is not on practice {practice} roster")]
    PatientNotOnRoster {
        /// Practice identifier.
        practice: String,
        /// Refused workspace identifier.
        workspace: String,
    },
    /// Runtime Postgres query or transaction failed.
    #[error("schedule database operation failed: {0}")]
    Database(#[from] DbErr),
    /// Stored schedule JSON column could not be serialized or decoded.
    #[error("schedule {context} serialization failed: {source}")]
    Serialization {
        /// Column or payload context.
        context: &'static str,
        /// Underlying `serde_json` error.
        #[source]
        source: serde_json::Error,
    },
    /// Stored schedule row violated a domain invariant on readback.
    #[error("corrupt stored schedule state ({0})")]
    CorruptStoredState(&'static str),
    /// Stored schedule identifier or collection failed domain reconstruction.
    #[error("corrupt stored schedule {context}: {source}")]
    CorruptStoredDomain {
        /// Stored column or field context.
        context: &'static str,
        /// Underlying domain validation error.
        #[source]
        source: ScheduleDomainError,
    },
    /// Stored schedule version integer was negative.
    #[error("corrupt stored schedule {context}: {source}")]
    CorruptStoredVersion {
        /// Version column context.
        context: &'static str,
        /// Underlying integer conversion error.
        #[source]
        source: std::num::TryFromIntError,
    },
}

fn trim_non_empty(
    value: impl Into<String>,
    err: ScheduleDomainError,
) -> Result<String, ScheduleDomainError> {
    let trimmed = value.into().trim().to_owned();
    if trimmed.is_empty() {
        return Err(err);
    }
    Ok(trimmed)
}

/// Validated non-empty schedule source identifier.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct SourceId(String);

impl SourceId {
    /// Validates that `value` is non-empty after trimming whitespace.
    pub fn new(value: impl Into<String>) -> Result<Self, ScheduleDomainError> {
        trim_non_empty(value, ScheduleDomainError::EmptySourceId).map(Self)
    }

    /// Borrowed identifier text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for SourceId {
    type Error = ScheduleDomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<SourceId> for String {
    fn from(value: SourceId) -> Self {
        value.0
    }
}

impl fmt::Display for SourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Validated non-empty native source appointment identifier.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct AppointmentId(String);

impl AppointmentId {
    /// Validates that `value` is non-empty after trimming whitespace.
    pub fn new(value: impl Into<String>) -> Result<Self, ScheduleDomainError> {
        trim_non_empty(value, ScheduleDomainError::EmptyAppointmentId).map(Self)
    }

    /// Borrowed identifier text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for AppointmentId {
    type Error = ScheduleDomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<AppointmentId> for String {
    fn from(value: AppointmentId) -> Self {
        value.0
    }
}

impl fmt::Display for AppointmentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Validated non-empty source-native patient identifier (for example an EHR MRN).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct SourcePatientId(String);

impl SourcePatientId {
    /// Validates that `value` is non-empty after trimming whitespace.
    pub fn new(value: impl Into<String>) -> Result<Self, ScheduleDomainError> {
        trim_non_empty(value, ScheduleDomainError::EmptySourcePatientId).map(Self)
    }

    /// Borrowed identifier text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for SourcePatientId {
    type Error = ScheduleDomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<SourcePatientId> for String {
    fn from(value: SourcePatientId) -> Self {
        value.0
    }
}

impl fmt::Display for SourcePatientId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Canonical appointment lifecycle status across schedule sources.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    sea_orm::EnumIter,
    sea_orm::DeriveActiveEnum,
)]
#[serde(rename_all = "lowercase")]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum AppointmentStatus {
    /// Booked or active upcoming/in-progress visit (`"scheduled"`).
    #[sea_orm(string_value = "scheduled")]
    Scheduled,
    /// Cancelled or withdrawn visit (`"cancelled"`).
    #[sea_orm(string_value = "cancelled")]
    Cancelled,
    /// Completed visit (`"fulfilled"`).
    #[sea_orm(string_value = "fulfilled")]
    Fulfilled,
    /// Missed visit (`"noshow"`).
    #[sea_orm(string_value = "noshow")]
    NoShow,
}

impl AppointmentStatus {
    /// Wire and storage representation of this status.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scheduled => "scheduled",
            Self::Cancelled => "cancelled",
            Self::Fulfilled => "fulfilled",
            Self::NoShow => "noshow",
        }
    }
}

/// Validated `[starts_at, ends_at]` window for an appointment.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppointmentWindow {
    starts_at: String,
    starts_at_unix: i64,
    ends_at: String,
    ends_at_unix: i64,
}

impl AppointmentWindow {
    /// Parses RFC 3339 `starts_at` and `ends_at` strings, canonicalizes them to
    /// UTC (`YYYY-MM-DDTHH:MM:SS.mmmZ`), and verifies `starts_at <= ends_at`.
    pub fn parse(starts_at: &str, ends_at: &str) -> Result<Self, ScheduleDomainError> {
        let (starts_at_unix, start_ms) = parse_rfc3339("starts_at", starts_at)?;
        let (ends_at_unix, end_ms) = parse_rfc3339("ends_at", ends_at)?;
        let canonical_start = format_unix_rfc3339_millis(starts_at_unix, start_ms);
        let canonical_end = format_unix_rfc3339_millis(ends_at_unix, end_ms);
        if (ends_at_unix, end_ms) < (starts_at_unix, start_ms) {
            return Err(ScheduleDomainError::InvertedWindow {
                starts_at: canonical_start,
                ends_at: canonical_end,
            });
        }
        Ok(Self {
            starts_at: canonical_start,
            starts_at_unix,
            ends_at: canonical_end,
            ends_at_unix,
        })
    }

    /// Reconstructs a validated window from stored columns.
    fn from_stored(
        starts_at: String,
        starts_at_unix: i64,
        ends_at: String,
        ends_at_unix: i64,
    ) -> Result<Self, ScheduleError> {
        if starts_at.trim().is_empty() || ends_at.trim().is_empty() || ends_at_unix < starts_at_unix
        {
            return Err(ScheduleError::CorruptStoredState("appointment window"));
        }
        Ok(Self {
            starts_at,
            starts_at_unix,
            ends_at,
            ends_at_unix,
        })
    }

    /// Canonical UTC RFC 3339 start timestamp.
    pub fn starts_at(&self) -> &str {
        &self.starts_at
    }

    /// Start timestamp in Unix seconds.
    pub const fn starts_at_unix(&self) -> i64 {
        self.starts_at_unix
    }

    /// Canonical UTC RFC 3339 end timestamp.
    pub fn ends_at(&self) -> &str {
        &self.ends_at
    }

    /// End timestamp in Unix seconds.
    pub const fn ends_at_unix(&self) -> i64 {
        self.ends_at_unix
    }
}

/// Inputs used to construct a validated [`SourceAppointment`].
#[derive(Clone, Debug)]
pub struct SourceAppointmentParts {
    /// Native appointment identifier within the schedule source.
    pub appointment_id: AppointmentId,
    /// Explicit per-edit revision supplied by the source; `None` versions the
    /// appointment by the SHA-256 digest of `content`.
    pub source_revision: Option<String>,
    /// Validated appointment start/end window.
    pub window: AppointmentWindow,
    /// Canonical appointment status.
    pub status: AppointmentStatus,
    /// Exact status text reported by the source.
    pub source_status: String,
    /// Visit type or reason label shown on the schedule.
    pub label: String,
    /// Source-native patient identifier, when the appointment names a patient.
    pub source_patient_id: Option<SourcePatientId>,
    /// Source dates in the source's own field names and precision.
    pub dates: Vec<RecordDate>,
    /// Exact UTF-8 source representation of the appointment.
    pub content: String,
    /// Source-owned provenance metadata (must not be JSON `null`).
    pub provenance: Value,
}

/// One validated appointment delivered by a [`ScheduleSource`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceAppointment {
    appointment_id: AppointmentId,
    source_revision: String,
    window: AppointmentWindow,
    status: AppointmentStatus,
    source_status: String,
    label: String,
    source_patient_id: Option<SourcePatientId>,
    dates: Vec<RecordDate>,
    content: String,
    sha256: String,
    provenance: Value,
}

impl SourceAppointment {
    /// Validates `parts`, computes the SHA-256 content digest, and assigns
    /// content-hash versioning when the source supplies no per-edit revision.
    pub fn new(parts: SourceAppointmentParts) -> Result<Self, ScheduleDomainError> {
        let source_status =
            trim_non_empty(parts.source_status, ScheduleDomainError::EmptySourceStatus)?;
        let label = trim_non_empty(parts.label, ScheduleDomainError::EmptyLabel)?;
        if parts.provenance.is_null() {
            return Err(ScheduleDomainError::NullProvenance);
        }
        let sha256 = crate::sha256_hex(parts.content.as_bytes());
        let source_revision = match parts.source_revision {
            Some(revision) => trim_non_empty(revision, ScheduleDomainError::EmptySourceRevision)?,
            None => sha256.clone(),
        };
        Ok(Self {
            appointment_id: parts.appointment_id,
            source_revision,
            window: parts.window,
            status: parts.status,
            source_status,
            label,
            source_patient_id: parts.source_patient_id,
            dates: parts.dates,
            content: parts.content,
            sha256,
            provenance: parts.provenance,
        })
    }

    /// Native appointment identifier.
    pub fn appointment_id(&self) -> &AppointmentId {
        &self.appointment_id
    }

    /// Source revision or content-digest version when the source carries none.
    pub fn source_revision(&self) -> &str {
        &self.source_revision
    }

    /// Validated appointment start/end window.
    pub fn window(&self) -> &AppointmentWindow {
        &self.window
    }

    /// Canonical appointment status.
    pub const fn status(&self) -> AppointmentStatus {
        self.status
    }

    /// Exact status text reported by the source.
    pub fn source_status(&self) -> &str {
        &self.source_status
    }

    /// Visit label shown on the schedule.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Source-native patient identifier, if the appointment names a patient.
    pub fn source_patient_id(&self) -> Option<&SourcePatientId> {
        self.source_patient_id.as_ref()
    }

    /// Dates in the source's own words.
    pub fn dates(&self) -> &[RecordDate] {
        &self.dates
    }

    /// Retained UTF-8 source representation.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// SHA-256 hex digest of [`Self::content`].
    pub fn sha256(&self) -> &str {
        &self.sha256
    }

    /// Source-owned provenance metadata.
    pub fn provenance(&self) -> &Value {
        &self.provenance
    }
}

/// One named item or slice that failed during a partial schedule read.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduleReadIssue {
    target: String,
    reason: String,
}

impl ScheduleReadIssue {
    /// Creates a named schedule read issue, rejecting blank `target` or `reason`.
    pub fn new(
        target: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<Self, ScheduleDomainError> {
        let target = trim_non_empty(target, ScheduleDomainError::BlankFailureDetail)?;
        let reason = trim_non_empty(reason, ScheduleDomainError::BlankFailureDetail)?;
        Ok(Self { target, reason })
    }

    /// Record, line, or source slice that failed.
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Human-readable reason the target could not be read.
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

/// Non-empty collection of [`ScheduleReadIssue`] entries from a partial read.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Vec<ScheduleReadIssue>", into = "Vec<ScheduleReadIssue>")]
pub struct ScheduleReadFailures(Vec<ScheduleReadIssue>);

impl ScheduleReadFailures {
    /// Wraps `issues`, refusing an empty list or any blank target/reason.
    pub fn new(issues: Vec<ScheduleReadIssue>) -> Result<Self, ScheduleDomainError> {
        if issues.is_empty() {
            return Err(ScheduleDomainError::EmptyPartialFailures);
        }
        let mut validated = Vec::with_capacity(issues.len());
        for issue in issues {
            validated.push(ScheduleReadIssue::new(issue.target, issue.reason)?);
        }
        Ok(Self(validated))
    }

    /// Borrowed slice of named failures (always non-empty).
    pub fn as_slice(&self) -> &[ScheduleReadIssue] {
        &self.0
    }

    /// Number of named failures (always `>= 1`).
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Always `false`: [`ScheduleReadFailures`] cannot be empty.
    pub fn is_empty(&self) -> bool {
        false
    }
}

impl TryFrom<Vec<ScheduleReadIssue>> for ScheduleReadFailures {
    type Error = ScheduleDomainError;

    fn try_from(value: Vec<ScheduleReadIssue>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ScheduleReadFailures> for Vec<ScheduleReadIssue> {
    fn from(value: ScheduleReadFailures) -> Self {
        value.0
    }
}

/// Result of reading a [`ScheduleSource`] when at least part of the source could be read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScheduleDelivery {
    /// Every item in the source read cleanly.
    Complete {
        /// Validated appointments currently held by the source.
        appointments: Vec<SourceAppointment>,
    },
    /// Valid appointments were read while one or more items failed.
    Partial {
        /// Validated appointments recovered from the partial read.
        appointments: Vec<SourceAppointment>,
        /// Non-empty list naming what failed and why.
        failures: ScheduleReadFailures,
    },
}

impl ScheduleDelivery {
    /// Builds [`Self::Complete`] when `issues` is empty, or [`Self::Partial`]
    /// when one or more items failed. Deduplicates identical replays within the
    /// same delivery and rejects conflicting duplicate appointment IDs.
    pub fn from_outcomes(
        appointments: Vec<SourceAppointment>,
        issues: Vec<ScheduleReadIssue>,
    ) -> Result<Self, ScheduleDomainError> {
        let appointments = deduplicate_appointments(appointments)?;
        if issues.is_empty() {
            Ok(Self::Complete { appointments })
        } else {
            Ok(Self::Partial {
                appointments,
                failures: ScheduleReadFailures::new(issues)?,
            })
        }
    }
}

fn deduplicate_appointments(
    appointments: Vec<SourceAppointment>,
) -> Result<Vec<SourceAppointment>, ScheduleDomainError> {
    let mut by_id: BTreeMap<AppointmentId, SourceAppointment> = BTreeMap::new();
    let mut ordered_ids: Vec<AppointmentId> = Vec::with_capacity(appointments.len());
    for appointment in appointments {
        let id = appointment.appointment_id().clone();
        if let Some(existing) = by_id.get(&id) {
            if existing.sha256() != appointment.sha256() {
                return Err(ScheduleDomainError::DuplicateAppointmentId {
                    appointment_id: id.as_str().to_owned(),
                });
            }
            continue;
        }
        ordered_ids.push(id.clone());
        by_id.insert(id, appointment);
    }
    Ok(ordered_ids
        .into_iter()
        .filter_map(|id| by_id.remove(&id))
        .collect())
}

/// Source adapter interface for reading a practice's appointment calendar.
#[async_trait]
pub trait ScheduleSource: Send + Sync {
    /// Stable identifier of this schedule source.
    fn source_id(&self) -> &SourceId;
    /// Physician-facing name of this schedule source.
    fn source_label(&self) -> &str;
    /// Reads the practice's schedule from this source.
    async fn read_schedule(&self) -> Result<ScheduleDelivery, ScheduleSourceError>;
}

/// Resolution of one appointment against seeded schedule patient links and the
/// practice's current roster.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "match_state", rename_all = "snake_case")]
pub enum PatientMatch {
    /// Matched through a seeded link to a workspace currently on the practice roster.
    Matched {
        /// Coda patient workspace identifier.
        workspace_id: String,
        /// Source-native patient identifier that linked to `workspace_id`.
        source_patient_id: SourcePatientId,
    },
    /// Visible on the schedule without a matched Coda patient workspace.
    Unmatched {
        /// Unlinked or unregistered source patient identifier, when the source carried one.
        source_patient_id: Option<SourcePatientId>,
    },
}

/// Resolves an appointment's optional [`SourcePatientId`] against `links` and
/// `roster_workspaces`.
///
/// This is the single owner of the appointment-to-patient matching rule: an
/// appointment matches if and only if its `source_patient_id` has a seeded link
/// for the source **and** the linked workspace is currently registered on the
/// practice's roster.
pub fn resolve_patient_match(
    source_patient_id: Option<&SourcePatientId>,
    links: &BTreeMap<SourcePatientId, String>,
    roster_workspaces: &BTreeSet<String>,
) -> PatientMatch {
    let Some(patient_id) = source_patient_id else {
        return PatientMatch::Unmatched {
            source_patient_id: None,
        };
    };
    if let Some(workspace) = links.get(patient_id)
        && roster_workspaces.contains(workspace.as_str())
    {
        return PatientMatch::Matched {
            workspace_id: workspace.clone(),
            source_patient_id: patient_id.clone(),
        };
    }
    PatientMatch::Unmatched {
        source_patient_id: Some(patient_id.clone()),
    }
}

/// One seeded link mapping a schedule source's patient identifier to an existing
/// Coda patient workspace on the practice's roster.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeededSchedulePatientLink {
    /// Practice identifier.
    pub practice: String,
    /// Schedule source identifier.
    pub source_id: SourceId,
    /// Source-native patient identifier.
    pub source_patient_id: SourcePatientId,
    /// Linked Coda patient workspace identifier.
    pub workspace_id: String,
    /// Practice-clock Unix second when the link was seeded.
    pub seeded_at: i64,
}

/// One saved appointment on a practice's schedule, with its saved version and
/// `"as of"` timestamp.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedAppointment {
    /// Practice identifier.
    pub practice: String,
    /// Schedule source identifier.
    pub source_id: SourceId,
    /// Native source appointment identifier.
    pub appointment_id: AppointmentId,
    /// Monotonic saved version, starting at `1` and incrementing only on material change.
    pub saved_version: u64,
    /// Source revision or content-digest version.
    pub source_revision: String,
    /// Validated appointment start/end window.
    pub window: AppointmentWindow,
    /// Canonical appointment status.
    pub status: AppointmentStatus,
    /// Exact status text from the source (`"withdrawn"` when omitted on a complete read).
    pub source_status: String,
    /// Visit label shown on the schedule.
    pub label: String,
    /// Matched Coda patient workspace or unmatched standing.
    pub patient_match: PatientMatch,
    /// Dates in the source's own words.
    pub dates: Vec<RecordDate>,
    /// Retained UTF-8 source representation.
    pub content: String,
    /// SHA-256 hex digest of `content`.
    pub sha256: String,
    /// Source-owned provenance metadata.
    pub provenance: Value,
    /// Practice-clock Unix second when this appointment was last confirmed by a read.
    pub as_of: i64,
    /// Practice-clock Unix second when `saved_version` was written.
    pub updated_at: i64,
}

/// Recorded failure of a whole schedule read attempt.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduleFailure {
    /// Practice-clock Unix second when the failed read ran.
    pub failed_at: i64,
    /// Failure detail reported by the source adapter.
    pub reason: String,
}

/// Freshness of a usable saved schedule relative to its latest read attempt.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ScheduleFreshness {
    /// Latest read at `as_of` succeeded completely.
    Current,
    /// Latest read at `as_of` was partial; keeps delivered and previously saved
    /// visits while naming every failed item.
    Partial {
        /// Non-empty list naming what failed during the partial read.
        failures: ScheduleReadFailures,
        /// Timestamp of the last complete read, when one occurred earlier.
        last_complete_as_of: Option<i64>,
    },
    /// Latest refresh failed; `as_of` on [`ScheduleStanding::Usable`] preserves
    /// the last good schedule's timestamp and saved visits stay intact.
    RefreshFailed {
        /// Failure detail from the unsuccessful refresh attempt.
        failure: ScheduleFailure,
    },
}

/// Standing of a practice's schedule source.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ScheduleStanding {
    /// No schedule source has been read for the practice yet.
    Absent,
    /// Schedule reads have been attempted and none has ever succeeded.
    Unavailable {
        /// Source identifier.
        source_id: SourceId,
        /// Physician-facing source label.
        source_label: String,
        /// Failure from the latest unsuccessful read attempt.
        failure: ScheduleFailure,
    },
    /// At least one complete or partial schedule read has been saved.
    Usable {
        /// Source identifier.
        source_id: SourceId,
        /// Physician-facing source label.
        source_label: String,
        /// Monotonic schedule version (`>= 1`).
        schedule_version: u64,
        /// Practice-clock Unix second of the last good (complete or partial) read.
        as_of: i64,
        /// Whether `as_of` reflects a complete read, partial read, or preserved
        /// snapshot after a failed refresh.
        freshness: ScheduleFreshness,
    },
}

/// Consistent readback of a practice's saved schedule and source standing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticeSchedule {
    /// Practice identifier.
    pub practice: String,
    /// Standing of the schedule source (`Absent`, `Unavailable`, or `Usable`).
    pub standing: ScheduleStanding,
    /// Saved appointments ordered by `(starts_at_unix, appointment_id)`.
    pub appointments: Vec<SavedAppointment>,
}

/// Summary of an applied complete or partial schedule sync.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduleSyncSummary {
    /// Resulting `Usable` schedule standing.
    pub standing: ScheduleStanding,
    /// Total saved appointments held for this source after the sync.
    pub saved_appointments: usize,
    /// Appointments newly inserted or materially updated (`saved_version` bumped).
    pub changed_appointments: usize,
    /// Saved appointments currently matched to a roster patient workspace.
    pub matched_appointments: usize,
    /// Saved appointments currently visible without a matched workspace.
    pub unmatched_appointments: usize,
}

/// Outcome of calling [`Store::sync_practice_schedule`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum ScheduleSyncOutcome {
    /// Complete or partial schedule read committed atomically.
    Applied(ScheduleSyncSummary),
    /// Whole-source read failed; prior saved schedule and its `"as of"` time
    /// (when present) were kept intact and the failure was recorded.
    FailedRead {
        /// Resulting standing (`Unavailable` before any good read, `Usable` with
        /// `RefreshFailed` afterward).
        standing: ScheduleStanding,
        /// Count of previously saved appointments preserved untouched.
        retained_appointments: usize,
    },
}

impl Store {
    /// Seeds one schedule patient link mapping `(practice, source_id, source_patient_id)`
    /// to `workspace`.
    ///
    /// Verifies that `workspace` is already registered to `practice` on `auth`
    /// and never registers a new patient on the roster.
    pub async fn seed_schedule_patient_link(
        &self,
        auth: &dyn AuthStore,
        practice: &str,
        source_id: &SourceId,
        source_patient_id: &SourcePatientId,
        workspace: &str,
        seeded_at: i64,
    ) -> Result<SeededSchedulePatientLink, ScheduleError> {
        let practice = trim_non_empty(practice, ScheduleDomainError::EmptyPractice)?;
        let workspace = trim_non_empty(workspace, ScheduleDomainError::EmptyWorkspaceId)?;
        let owner = auth
            .patient_practice(&workspace)
            .await
            .map_err(ScheduleError::AuthStore)?;
        if owner.as_deref() != Some(practice.as_str()) {
            return Err(ScheduleError::PatientNotOnRoster {
                practice,
                workspace,
            });
        }

        schedule_patient_link::Entity::insert(schedule_patient_link::ActiveModel {
            practice: Set(practice.clone()),
            source_id: Set(source_id.as_str().to_owned()),
            source_patient_id: Set(source_patient_id.as_str().to_owned()),
            workspace: Set(workspace.clone()),
            seeded_at: Set(seeded_at),
        })
        .on_conflict(
            OnConflict::columns([
                schedule_patient_link::Column::Practice,
                schedule_patient_link::Column::SourceId,
                schedule_patient_link::Column::SourcePatientId,
            ])
            .update_columns([
                schedule_patient_link::Column::Workspace,
                schedule_patient_link::Column::SeededAt,
            ])
            .to_owned(),
        )
        .exec_without_returning(&self.db)
        .await?;

        Ok(SeededSchedulePatientLink {
            practice,
            source_id: source_id.clone(),
            source_patient_id: source_patient_id.clone(),
            workspace_id: workspace,
            seeded_at,
        })
    }

    /// Seeds schedule patient links for `schedule_source_id` from the recorded
    /// chart pulls (`physician_runtime_source_pulls`) of patients already on
    /// `practice`'s roster. Never registers any patient on the roster.
    pub async fn seed_schedule_links_from_pulls(
        &self,
        auth: &dyn AuthStore,
        practice: &str,
        schedule_source_id: &SourceId,
        seeded_at: i64,
    ) -> Result<usize, ScheduleError> {
        let practice = trim_non_empty(practice, ScheduleDomainError::EmptyPractice)?;
        let roster = auth
            .practice_patients(&practice)
            .await
            .map_err(ScheduleError::AuthStore)?;
        if roster.is_empty() {
            return Ok(0);
        }
        let pulls_by_workspace = self.source_pulls(&roster).await?;
        let tx = self.db.begin().await?;
        let existing = schedule_patient_link::Entity::find()
            .filter(schedule_patient_link::Column::Practice.eq(practice.as_str()))
            .filter(schedule_patient_link::Column::SourceId.eq(schedule_source_id.as_str()))
            .all(&tx)
            .await?;
        let existing_map: BTreeMap<String, String> = existing
            .into_iter()
            .map(|row| (row.source_patient_id, row.workspace))
            .collect();

        let mut changed = 0usize;
        for (workspace, pulls) in pulls_by_workspace {
            for pull in pulls {
                let Ok(patient_id) = SourcePatientId::new(pull.patient.source_patient_id) else {
                    continue;
                };
                if existing_map.get(patient_id.as_str()) == Some(&workspace) {
                    continue;
                }
                schedule_patient_link::Entity::insert(schedule_patient_link::ActiveModel {
                    practice: Set(practice.clone()),
                    source_id: Set(schedule_source_id.as_str().to_owned()),
                    source_patient_id: Set(patient_id.as_str().to_owned()),
                    workspace: Set(workspace.clone()),
                    seeded_at: Set(seeded_at),
                })
                .on_conflict(
                    OnConflict::columns([
                        schedule_patient_link::Column::Practice,
                        schedule_patient_link::Column::SourceId,
                        schedule_patient_link::Column::SourcePatientId,
                    ])
                    .update_columns([
                        schedule_patient_link::Column::Workspace,
                        schedule_patient_link::Column::SeededAt,
                    ])
                    .to_owned(),
                )
                .exec_without_returning(&tx)
                .await?;
                changed += 1;
            }
        }
        tx.commit().await?;
        Ok(changed)
    }

    /// Lists seeded schedule patient links for `(practice, source_id)`, ordered
    /// by `source_patient_id`.
    pub async fn schedule_patient_links(
        &self,
        practice: &str,
        source_id: &SourceId,
    ) -> Result<Vec<SeededSchedulePatientLink>, ScheduleError> {
        let practice = trim_non_empty(practice, ScheduleDomainError::EmptyPractice)?;
        let rows = schedule_patient_link::Entity::find()
            .filter(schedule_patient_link::Column::Practice.eq(practice.as_str()))
            .filter(schedule_patient_link::Column::SourceId.eq(source_id.as_str()))
            .order_by_asc(schedule_patient_link::Column::SourcePatientId)
            .all(&self.db)
            .await?;
        rows.into_iter()
            .map(|row| {
                Ok(SeededSchedulePatientLink {
                    practice: row.practice,
                    source_id: SourceId::new(row.source_id).map_err(|source| {
                        ScheduleError::CorruptStoredDomain {
                            context: "link source_id",
                            source,
                        }
                    })?,
                    source_patient_id: SourcePatientId::new(row.source_patient_id).map_err(
                        |source| ScheduleError::CorruptStoredDomain {
                            context: "link source_patient_id",
                            source,
                        },
                    )?,
                    workspace_id: row.workspace,
                    seeded_at: row.seeded_at,
                })
            })
            .collect()
    }

    /// Reads `source` and atomically persists the resulting schedule and source
    /// standing for `practice` at `read_at` (Unix seconds).
    ///
    /// `source.read_schedule()` runs before opening the write transaction, so a
    /// process crash or abort mid-read leaves Postgres untouched.
    pub async fn sync_practice_schedule(
        &self,
        auth: &dyn AuthStore,
        practice: &str,
        source: &dyn ScheduleSource,
        read_at: i64,
    ) -> Result<ScheduleSyncOutcome, ScheduleError> {
        let practice = trim_non_empty(practice, ScheduleDomainError::EmptyPractice)?;
        let source_id = source.source_id().clone();
        let source_label =
            trim_non_empty(source.source_label(), ScheduleDomainError::EmptySourceLabel)?;
        let roster: BTreeSet<String> = auth
            .practice_patients(&practice)
            .await
            .map_err(ScheduleError::AuthStore)?
            .into_iter()
            .collect();

        match source.read_schedule().await {
            Ok(delivery) => {
                let tx = self.db.begin().await?;
                let summary = apply_delivery_in_tx(
                    &tx,
                    &practice,
                    &source_id,
                    &source_label,
                    &roster,
                    delivery,
                    read_at,
                    None,
                )
                .await?;
                super::briefings::reconcile_schedule_changes_in_tx(&tx, &practice, read_at)
                    .await
                    .map_err(ScheduleError::Database)?;
                tx.commit().await?;
                Ok(ScheduleSyncOutcome::Applied(summary))
            }
            Err(read_error) => {
                let failure = ScheduleFailure {
                    failed_at: read_at,
                    reason: read_error.to_string(),
                };
                let tx = self.db.begin().await?;
                let (standing, retained_appointments) =
                    record_failed_read_in_tx(&tx, &practice, &source_id, &source_label, failure)
                        .await?;
                tx.commit().await?;
                Ok(ScheduleSyncOutcome::FailedRead {
                    standing,
                    retained_appointments,
                })
            }
        }
    }

    /// Runs the live [`Store::sync_practice_schedule`] transaction write path
    /// up to `abort_after_appointment_writes` appointment rows, confirms those
    /// rows were written inside the uncommitted transaction, and aborts before
    /// `commit` so Postgres rolls the partial write back.
    #[doc(hidden)]
    pub async fn abort_schedule_sync_after_writes(
        &self,
        auth: &dyn AuthStore,
        practice: &str,
        source: &dyn ScheduleSource,
        read_at: i64,
        abort_after_appointment_writes: usize,
    ) -> Result<(), ScheduleError> {
        let practice = trim_non_empty(practice, ScheduleDomainError::EmptyPractice)?;
        let source_id = source.source_id().clone();
        let source_label =
            trim_non_empty(source.source_label(), ScheduleDomainError::EmptySourceLabel)?;
        let roster: BTreeSet<String> = auth
            .practice_patients(&practice)
            .await
            .map_err(ScheduleError::AuthStore)?
            .into_iter()
            .collect();
        let delivery = source
            .read_schedule()
            .await
            .map_err(|cause| ScheduleError::Database(DbErr::Custom(cause.to_string())))?;
        let tx = self.db.begin().await?;
        let outcome = apply_delivery_in_tx(
            &tx,
            &practice,
            &source_id,
            &source_label,
            &roster,
            delivery,
            read_at,
            Some(abort_after_appointment_writes),
        )
        .await;
        match outcome {
            Err(err) => Err(err),
            Ok(_) => Err(ScheduleError::Database(DbErr::Custom(
                "expected mid-transaction schedule write interruption".to_owned(),
            ))),
        }
    }

    /// Reads the consistent saved schedule and source standing for `practice`.
    /// When `practice` has not read any schedule source yet, returns
    /// [`ScheduleStanding::Absent`] with zero appointments.
    pub async fn practice_schedule(
        &self,
        practice: &str,
    ) -> Result<PracticeSchedule, ScheduleError> {
        let practice = trim_non_empty(practice, ScheduleDomainError::EmptyPractice)?;
        let tx = self.db.begin().await?;
        let source_row = schedule_source::Entity::find()
            .filter(schedule_source::Column::Practice.eq(practice.as_str()))
            .order_by_asc(schedule_source::Column::SourceId)
            .one(&tx)
            .await?;
        let Some(source_row) = source_row else {
            tx.commit().await?;
            return Ok(PracticeSchedule {
                practice,
                standing: ScheduleStanding::Absent,
                appointments: Vec::new(),
            });
        };
        let standing = decode_source_standing(&source_row)?;
        let appointment_rows = schedule_appointment::Entity::find()
            .filter(schedule_appointment::Column::Practice.eq(practice.as_str()))
            .filter(schedule_appointment::Column::SourceId.eq(source_row.source_id.as_str()))
            .order_by_asc(schedule_appointment::Column::StartsAtUnix)
            .order_by_asc(schedule_appointment::Column::AppointmentId)
            .all(&tx)
            .await?;
        tx.commit().await?;
        let mut appointments = Vec::with_capacity(appointment_rows.len());
        for row in appointment_rows {
            appointments.push(decode_saved_appointment(row)?);
        }
        Ok(PracticeSchedule {
            practice,
            standing,
            appointments,
        })
    }
}

/// Applies a [`ScheduleDelivery`] inside caller-supplied transaction `tx`.
/// Exposed to crate tests via [`Store`] and transaction rollback tests.
#[expect(
    clippy::too_many_arguments,
    reason = "single-owner transaction write path shared by live sync and rollback proof"
)]
pub(crate) async fn apply_delivery_in_tx(
    tx: &impl ConnectionTrait,
    practice: &str,
    source_id: &SourceId,
    source_label: &str,
    roster: &BTreeSet<String>,
    delivery: ScheduleDelivery,
    read_at: i64,
    abort_after_appointment_writes: Option<usize>,
) -> Result<ScheduleSyncSummary, ScheduleError> {
    let existing_source = lock_schedule_source(tx, practice, source_id).await?;
    let prior_version = match &existing_source {
        Some(row) => u64::try_from(row.schedule_version).map_err(|source| {
            ScheduleError::CorruptStoredVersion {
                context: "schedule_version",
                source,
            }
        })?,
        None => 0,
    };
    let prior_last_complete = existing_source
        .as_ref()
        .and_then(|row| row.last_complete_as_of);

    let link_rows = schedule_patient_link::Entity::find()
        .filter(schedule_patient_link::Column::Practice.eq(practice))
        .filter(schedule_patient_link::Column::SourceId.eq(source_id.as_str()))
        .all(tx)
        .await?;
    let mut links = BTreeMap::new();
    for row in link_rows {
        let patient_id = SourcePatientId::new(row.source_patient_id).map_err(|source| {
            ScheduleError::CorruptStoredDomain {
                context: "link source_patient_id",
                source,
            }
        })?;
        links.insert(patient_id, row.workspace);
    }

    let existing_rows = schedule_appointment::Entity::find()
        .filter(schedule_appointment::Column::Practice.eq(practice))
        .filter(schedule_appointment::Column::SourceId.eq(source_id.as_str()))
        .all(tx)
        .await?;
    let mut existing_by_id = BTreeMap::new();
    for row in existing_rows {
        let decoded = decode_saved_appointment(row)?;
        existing_by_id.insert(decoded.appointment_id.clone(), decoded);
    }

    let (is_complete, incoming, failures) = match delivery {
        ScheduleDelivery::Complete { appointments } => (true, appointments, None),
        ScheduleDelivery::Partial {
            appointments,
            failures,
        } => (false, appointments, Some(failures)),
    };

    let mut delivered_ids = BTreeSet::new();
    let mut changed_appointments = 0usize;
    let mut written_appointments = 0usize;

    for item in incoming {
        delivered_ids.insert(item.appointment_id().clone());
        let resolved_match = resolve_patient_match(item.source_patient_id(), &links, roster);
        let (match_state, workspace_col, source_patient_col) =
            encode_patient_match(&resolved_match);
        let dates_json =
            serde_json::to_value(item.dates()).map_err(|source| ScheduleError::Serialization {
                context: "appointment dates",
                source,
            })?;

        match existing_by_id.get(item.appointment_id()) {
            Some(existing) => {
                let material_change = existing.sha256 != item.sha256()
                    || existing.source_revision != item.source_revision()
                    || existing.window != *item.window()
                    || existing.status != item.status()
                    || existing.source_status != item.source_status()
                    || existing.label != item.label()
                    || existing.patient_match != resolved_match;

                let next_saved_version = if material_change {
                    changed_appointments += 1;
                    existing.saved_version + 1
                } else {
                    existing.saved_version
                };
                let next_updated_at = if material_change {
                    read_at
                } else {
                    existing.updated_at
                };

                schedule_appointment::ActiveModel {
                    practice: Set(practice.to_owned()),
                    source_id: Set(source_id.as_str().to_owned()),
                    appointment_id: Set(item.appointment_id().as_str().to_owned()),
                    saved_version: Set(next_saved_version as i64),
                    source_revision: Set(item.source_revision().to_owned()),
                    starts_at: Set(item.window().starts_at().to_owned()),
                    starts_at_unix: Set(item.window().starts_at_unix()),
                    ends_at: Set(item.window().ends_at().to_owned()),
                    ends_at_unix: Set(item.window().ends_at_unix()),
                    status: Set(item.status()),
                    source_status: Set(item.source_status().to_owned()),
                    label: Set(item.label().to_owned()),
                    match_state: Set(match_state.to_owned()),
                    workspace: Set(workspace_col),
                    source_patient_id: Set(source_patient_col),
                    dates: Set(dates_json),
                    content: Set(item.content().to_owned()),
                    sha256: Set(item.sha256().to_owned()),
                    provenance: Set(item.provenance().clone()),
                    as_of: Set(read_at),
                    updated_at: Set(next_updated_at),
                }
                .update(tx)
                .await?;
            }
            None => {
                changed_appointments += 1;
                schedule_appointment::Entity::insert(schedule_appointment::ActiveModel {
                    practice: Set(practice.to_owned()),
                    source_id: Set(source_id.as_str().to_owned()),
                    appointment_id: Set(item.appointment_id().as_str().to_owned()),
                    saved_version: Set(1),
                    source_revision: Set(item.source_revision().to_owned()),
                    starts_at: Set(item.window().starts_at().to_owned()),
                    starts_at_unix: Set(item.window().starts_at_unix()),
                    ends_at: Set(item.window().ends_at().to_owned()),
                    ends_at_unix: Set(item.window().ends_at_unix()),
                    status: Set(item.status()),
                    source_status: Set(item.source_status().to_owned()),
                    label: Set(item.label().to_owned()),
                    match_state: Set(match_state.to_owned()),
                    workspace: Set(workspace_col),
                    source_patient_id: Set(source_patient_col),
                    dates: Set(dates_json),
                    content: Set(item.content().to_owned()),
                    sha256: Set(item.sha256().to_owned()),
                    provenance: Set(item.provenance().clone()),
                    as_of: Set(read_at),
                    updated_at: Set(read_at),
                })
                .exec_without_returning(tx)
                .await?;
            }
        }
        written_appointments += 1;
        if abort_after_appointment_writes == Some(written_appointments) {
            return Err(ScheduleError::Database(DbErr::Custom(format!(
                "aborted schedule write transaction after {written_appointments} appointment writes"
            ))));
        }
    }

    if is_complete {
        for (id, existing) in &existing_by_id {
            if !delivered_ids.contains(id) && existing.status == AppointmentStatus::Scheduled {
                changed_appointments += 1;
                let resolved_match = resolve_patient_match(
                    match &existing.patient_match {
                        PatientMatch::Matched {
                            source_patient_id, ..
                        } => Some(source_patient_id),
                        PatientMatch::Unmatched { source_patient_id } => source_patient_id.as_ref(),
                    },
                    &links,
                    roster,
                );
                let (match_state, workspace_col, source_patient_col) =
                    encode_patient_match(&resolved_match);
                let dates_json = serde_json::to_value(&existing.dates).map_err(|source| {
                    ScheduleError::Serialization {
                        context: "appointment dates",
                        source,
                    }
                })?;
                schedule_appointment::ActiveModel {
                    practice: Set(practice.to_owned()),
                    source_id: Set(source_id.as_str().to_owned()),
                    appointment_id: Set(id.as_str().to_owned()),
                    saved_version: Set((existing.saved_version + 1) as i64),
                    source_revision: Set(existing.source_revision.clone()),
                    starts_at: Set(existing.window.starts_at().to_owned()),
                    starts_at_unix: Set(existing.window.starts_at_unix()),
                    ends_at: Set(existing.window.ends_at().to_owned()),
                    ends_at_unix: Set(existing.window.ends_at_unix()),
                    status: Set(AppointmentStatus::Cancelled),
                    source_status: Set("withdrawn".to_owned()),
                    label: Set(existing.label.clone()),
                    match_state: Set(match_state.to_owned()),
                    workspace: Set(workspace_col),
                    source_patient_id: Set(source_patient_col),
                    dates: Set(dates_json),
                    content: Set(existing.content.clone()),
                    sha256: Set(existing.sha256.clone()),
                    provenance: Set(existing.provenance.clone()),
                    as_of: Set(read_at),
                    updated_at: Set(read_at),
                }
                .update(tx)
                .await?;
            }
        }
    }

    let next_schedule_version = if prior_version == 0 || changed_appointments > 0 {
        prior_version + 1
    } else {
        prior_version
    };

    let (standing_col, last_complete_col, issues_json, freshness) = match failures {
        None => (
            "current",
            Some(read_at),
            serde_json::json!([]),
            ScheduleFreshness::Current,
        ),
        Some(failures) => {
            let json = serde_json::to_value(failures.as_slice()).map_err(|source| {
                ScheduleError::Serialization {
                    context: "schedule read issues",
                    source,
                }
            })?;
            (
                "partial",
                prior_last_complete,
                json,
                ScheduleFreshness::Partial {
                    failures,
                    last_complete_as_of: prior_last_complete,
                },
            )
        }
    };

    upsert_schedule_source(
        tx,
        existing_source.is_some(),
        schedule_source::ActiveModel {
            practice: Set(practice.to_owned()),
            source_id: Set(source_id.as_str().to_owned()),
            source_label: Set(source_label.to_owned()),
            schedule_version: Set(next_schedule_version as i64),
            standing: Set(standing_col.to_owned()),
            as_of: Set(Some(read_at)),
            last_complete_as_of: Set(last_complete_col),
            issues: Set(issues_json),
            last_failure: Set(None),
        },
    )
    .await?;

    let final_rows = schedule_appointment::Entity::find()
        .filter(schedule_appointment::Column::Practice.eq(practice))
        .filter(schedule_appointment::Column::SourceId.eq(source_id.as_str()))
        .all(tx)
        .await?;
    let saved_appointments = final_rows.len();
    let matched_appointments = final_rows
        .iter()
        .filter(|row| row.match_state == "matched")
        .count();
    let unmatched_appointments = saved_appointments - matched_appointments;

    let standing = ScheduleStanding::Usable {
        source_id: source_id.clone(),
        source_label: source_label.to_owned(),
        schedule_version: next_schedule_version,
        as_of: read_at,
        freshness,
    };

    Ok(ScheduleSyncSummary {
        standing,
        saved_appointments,
        changed_appointments,
        matched_appointments,
        unmatched_appointments,
    })
}

async fn record_failed_read_in_tx(
    tx: &impl ConnectionTrait,
    practice: &str,
    source_id: &SourceId,
    source_label: &str,
    failure: ScheduleFailure,
) -> Result<(ScheduleStanding, usize), ScheduleError> {
    let existing_source = lock_schedule_source(tx, practice, source_id).await?;
    let failure_json =
        serde_json::to_value(&failure).map_err(|source| ScheduleError::Serialization {
            context: "schedule failure",
            source,
        })?;

    let retained_rows = schedule_appointment::Entity::find()
        .filter(schedule_appointment::Column::Practice.eq(practice))
        .filter(schedule_appointment::Column::SourceId.eq(source_id.as_str()))
        .all(tx)
        .await?;
    let retained_appointments = retained_rows.len();

    match existing_source {
        Some(row) if row.schedule_version > 0 => {
            let preserved_as_of = row.as_of.ok_or(ScheduleError::CorruptStoredState(
                "usable source missing as_of",
            ))?;
            let schedule_version = u64::try_from(row.schedule_version).map_err(|source| {
                ScheduleError::CorruptStoredVersion {
                    context: "schedule_version",
                    source,
                }
            })?;
            upsert_schedule_source(
                tx,
                true,
                schedule_source::ActiveModel {
                    practice: Set(practice.to_owned()),
                    source_id: Set(source_id.as_str().to_owned()),
                    source_label: Set(source_label.to_owned()),
                    schedule_version: Set(row.schedule_version),
                    standing: Set("refresh_failed".to_owned()),
                    as_of: Set(Some(preserved_as_of)),
                    last_complete_as_of: Set(row.last_complete_as_of),
                    issues: Set(serde_json::json!([])),
                    last_failure: Set(Some(failure_json)),
                },
            )
            .await?;
            Ok((
                ScheduleStanding::Usable {
                    source_id: source_id.clone(),
                    source_label: source_label.to_owned(),
                    schedule_version,
                    as_of: preserved_as_of,
                    freshness: ScheduleFreshness::RefreshFailed { failure },
                },
                retained_appointments,
            ))
        }
        existing => {
            upsert_schedule_source(
                tx,
                existing.is_some(),
                schedule_source::ActiveModel {
                    practice: Set(practice.to_owned()),
                    source_id: Set(source_id.as_str().to_owned()),
                    source_label: Set(source_label.to_owned()),
                    schedule_version: Set(0),
                    standing: Set("unavailable".to_owned()),
                    as_of: Set(None),
                    last_complete_as_of: Set(None),
                    issues: Set(serde_json::json!([])),
                    last_failure: Set(Some(failure_json)),
                },
            )
            .await?;
            Ok((
                ScheduleStanding::Unavailable {
                    source_id: source_id.clone(),
                    source_label: source_label.to_owned(),
                    failure,
                },
                0,
            ))
        }
    }
}

async fn lock_schedule_source(
    tx: &impl ConnectionTrait,
    practice: &str,
    source_id: &SourceId,
) -> Result<Option<schedule_source::Model>, DbErr> {
    schedule_source::Entity::find_by_id((practice.to_owned(), source_id.as_str().to_owned()))
        .lock_exclusive()
        .one(tx)
        .await
}

async fn upsert_schedule_source(
    tx: &impl ConnectionTrait,
    exists: bool,
    model: schedule_source::ActiveModel,
) -> Result<(), DbErr> {
    if exists {
        model.update(tx).await?;
    } else {
        schedule_source::Entity::insert(model)
            .exec_without_returning(tx)
            .await?;
    }
    Ok(())
}

fn encode_patient_match(
    patient_match: &PatientMatch,
) -> (&'static str, Option<String>, Option<String>) {
    match patient_match {
        PatientMatch::Matched {
            workspace_id,
            source_patient_id,
        } => (
            "matched",
            Some(workspace_id.clone()),
            Some(source_patient_id.as_str().to_owned()),
        ),
        PatientMatch::Unmatched { source_patient_id } => (
            "unmatched",
            None,
            source_patient_id.as_ref().map(|id| id.as_str().to_owned()),
        ),
    }
}

fn decode_source_standing(row: &schedule_source::Model) -> Result<ScheduleStanding, ScheduleError> {
    let source_id = SourceId::new(row.source_id.clone()).map_err(|source| {
        ScheduleError::CorruptStoredDomain {
            context: "source_id",
            source,
        }
    })?;
    let source_label = trim_non_empty(
        row.source_label.clone(),
        ScheduleDomainError::EmptySourceLabel,
    )
    .map_err(|source| ScheduleError::CorruptStoredDomain {
        context: "source_label",
        source,
    })?;

    if row.standing == "unavailable" {
        let failure_json = row
            .last_failure
            .clone()
            .ok_or(ScheduleError::CorruptStoredState(
                "unavailable missing last_failure",
            ))?;
        let failure: ScheduleFailure = serde_json::from_value(failure_json).map_err(|source| {
            ScheduleError::Serialization {
                context: "schedule failure",
                source,
            }
        })?;
        return Ok(ScheduleStanding::Unavailable {
            source_id,
            source_label,
            failure,
        });
    }

    let schedule_version = u64::try_from(row.schedule_version).map_err(|source| {
        ScheduleError::CorruptStoredVersion {
            context: "schedule_version",
            source,
        }
    })?;
    if schedule_version == 0 {
        return Err(ScheduleError::CorruptStoredState(
            "usable schedule_version is zero",
        ));
    }
    let as_of = row.as_of.ok_or(ScheduleError::CorruptStoredState(
        "usable source missing as_of",
    ))?;

    let freshness = match row.standing.as_str() {
        "current" => ScheduleFreshness::Current,
        "partial" => {
            let issues: Vec<ScheduleReadIssue> = serde_json::from_value(row.issues.clone())
                .map_err(|source| ScheduleError::Serialization {
                    context: "schedule read issues",
                    source,
                })?;
            let failures = ScheduleReadFailures::new(issues).map_err(|source| {
                ScheduleError::CorruptStoredDomain {
                    context: "partial issues",
                    source,
                }
            })?;
            ScheduleFreshness::Partial {
                failures,
                last_complete_as_of: row.last_complete_as_of,
            }
        }
        "refresh_failed" => {
            let failure_json =
                row.last_failure
                    .clone()
                    .ok_or(ScheduleError::CorruptStoredState(
                        "refresh_failed missing last_failure",
                    ))?;
            let failure: ScheduleFailure =
                serde_json::from_value(failure_json).map_err(|source| {
                    ScheduleError::Serialization {
                        context: "schedule failure",
                        source,
                    }
                })?;
            ScheduleFreshness::RefreshFailed { failure }
        }
        _ => {
            return Err(ScheduleError::CorruptStoredState(
                "unknown schedule standing",
            ));
        }
    };

    Ok(ScheduleStanding::Usable {
        source_id,
        source_label,
        schedule_version,
        as_of,
        freshness,
    })
}

fn decode_saved_appointment(
    row: schedule_appointment::Model,
) -> Result<SavedAppointment, ScheduleError> {
    let source_id =
        SourceId::new(row.source_id).map_err(|source| ScheduleError::CorruptStoredDomain {
            context: "source_id",
            source,
        })?;
    let appointment_id = AppointmentId::new(row.appointment_id).map_err(|source| {
        ScheduleError::CorruptStoredDomain {
            context: "appointment_id",
            source,
        }
    })?;
    let saved_version =
        u64::try_from(row.saved_version).map_err(|source| ScheduleError::CorruptStoredVersion {
            context: "saved_version",
            source,
        })?;
    if saved_version == 0 {
        return Err(ScheduleError::CorruptStoredState("saved_version is zero"));
    }
    if crate::sha256_hex(row.content.as_bytes()) != row.sha256 {
        return Err(ScheduleError::CorruptStoredDomain {
            context: "appointment sha256",
            source: ScheduleDomainError::DigestMismatch {
                appointment_id: appointment_id.as_str().to_owned(),
            },
        });
    }
    let window = AppointmentWindow::from_stored(
        row.starts_at,
        row.starts_at_unix,
        row.ends_at,
        row.ends_at_unix,
    )?;
    let patient_match = match row.match_state.as_str() {
        "matched" => {
            let workspace_id = row.workspace.filter(|ws| !ws.trim().is_empty()).ok_or(
                ScheduleError::CorruptStoredState("matched missing workspace"),
            )?;
            let raw_patient_id = row
                .source_patient_id
                .ok_or(ScheduleError::CorruptStoredState(
                    "matched missing source_patient_id",
                ))?;
            let source_patient_id = SourcePatientId::new(raw_patient_id).map_err(|source| {
                ScheduleError::CorruptStoredDomain {
                    context: "matched source_patient_id",
                    source,
                }
            })?;
            PatientMatch::Matched {
                workspace_id,
                source_patient_id,
            }
        }
        "unmatched" => {
            if row.workspace.is_some() {
                return Err(ScheduleError::CorruptStoredState(
                    "unmatched carrying workspace",
                ));
            }
            let source_patient_id = row
                .source_patient_id
                .map(SourcePatientId::new)
                .transpose()
                .map_err(|source| ScheduleError::CorruptStoredDomain {
                    context: "unmatched source_patient_id",
                    source,
                })?;
            PatientMatch::Unmatched { source_patient_id }
        }
        _ => return Err(ScheduleError::CorruptStoredState("unknown match_state")),
    };
    let dates: Vec<RecordDate> =
        serde_json::from_value(row.dates).map_err(|source| ScheduleError::Serialization {
            context: "appointment dates",
            source,
        })?;

    Ok(SavedAppointment {
        practice: row.practice,
        source_id,
        appointment_id,
        saved_version,
        source_revision: row.source_revision,
        window,
        status: row.status,
        source_status: row.source_status,
        label: row.label,
        patient_match,
        dates,
        content: row.content,
        sha256: row.sha256,
        provenance: row.provenance,
        as_of: row.as_of,
        updated_at: row.updated_at,
    })
}

/// Formats `unix_secs` as canonical UTC RFC 3339 (`YYYY-MM-DDTHH:MM:SS.000Z`).
pub fn format_unix_rfc3339(unix_secs: i64) -> String {
    format_unix_rfc3339_millis(unix_secs, 0)
}

fn format_unix_rfc3339_millis(unix_secs: i64, millis: u32) -> String {
    let days = unix_secs.div_euclid(86_400);
    let tod = unix_secs.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = tod / 3_600;
    let minute = (tod % 3_600) / 60;
    let second = tod % 60;
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{millis:03}Z")
}

fn parse_rfc3339(field: &'static str, raw: &str) -> Result<(i64, u32), ScheduleDomainError> {
    let invalid = |reason: &'static str| ScheduleDomainError::InvalidTimestamp {
        field,
        value: raw.to_owned(),
        reason,
    };
    let trimmed = raw.trim();
    let (date_part, rest) = trimmed
        .split_once('T')
        .or_else(|| trimmed.split_once('t'))
        .ok_or_else(|| invalid("expected 'T' separator"))?;

    let mut date_iter = date_part.split('-');
    let year_str = date_iter.next().ok_or_else(|| invalid("missing year"))?;
    let month_str = date_iter.next().ok_or_else(|| invalid("missing month"))?;
    let day_str = date_iter.next().ok_or_else(|| invalid("missing day"))?;
    if date_iter.next().is_some()
        || year_str.len() != 4
        || month_str.len() != 2
        || day_str.len() != 2
    {
        return Err(invalid("expected YYYY-MM-DD date"));
    }
    let invalid_num = |component: &'static str, source: std::num::ParseIntError| {
        ScheduleDomainError::InvalidTimestampNumber {
            field,
            value: raw.to_owned(),
            component,
            source,
        }
    };
    let year: i32 = year_str
        .parse()
        .map_err(|source| invalid_num("year", source))?;
    let month: u32 = month_str
        .parse()
        .map_err(|source| invalid_num("month", source))?;
    let day: u32 = day_str
        .parse()
        .map_err(|source| invalid_num("day", source))?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return Err(invalid("calendar month or day out of range"));
    }
    let days = days_from_civil(year, month, day);
    if civil_from_days(days) != (year, month, day) {
        return Err(invalid("non-existent calendar date"));
    }

    let (time_part, offset_secs) = split_rfc3339_offset(field, raw, rest)?;
    let (hms_part, millis) = match time_part.split_once('.') {
        Some((hms, frac)) => {
            if frac.is_empty() || !frac.bytes().all(|b| b.is_ascii_digit()) {
                return Err(invalid("invalid fractional seconds"));
            }
            let millis = parse_millis(frac);
            (hms, millis)
        }
        None => (time_part, 0),
    };

    let mut time_iter = hms_part.split(':');
    let hour_str = time_iter.next().ok_or_else(|| invalid("missing hour"))?;
    let min_str = time_iter.next().ok_or_else(|| invalid("missing minute"))?;
    let sec_str = time_iter.next().ok_or_else(|| invalid("missing second"))?;
    if time_iter.next().is_some() || hour_str.len() != 2 || min_str.len() != 2 || sec_str.len() != 2
    {
        return Err(invalid("expected HH:MM:SS time"));
    }
    let hour: i64 = hour_str
        .parse()
        .map_err(|source| invalid_num("hour", source))?;
    let minute: i64 = min_str
        .parse()
        .map_err(|source| invalid_num("minute", source))?;
    let second: i64 = sec_str
        .parse()
        .map_err(|source| invalid_num("second", source))?;
    if !(0..=23).contains(&hour) || !(0..=59).contains(&minute) || !(0..=59).contains(&second) {
        return Err(invalid("clock hour, minute, or second out of range"));
    }

    let unix_secs = days * 86_400 + hour * 3_600 + minute * 60 + second - offset_secs;
    Ok((unix_secs, millis))
}

fn parse_millis(frac: &str) -> u32 {
    let mut digits = [b'0'; 3];
    for (idx, byte) in frac.bytes().take(3).enumerate() {
        digits[idx] = byte;
    }
    u32::from(digits[0] - b'0') * 100
        + u32::from(digits[1] - b'0') * 10
        + u32::from(digits[2] - b'0')
}

fn split_rfc3339_offset<'a>(
    field: &'static str,
    raw: &str,
    rest: &'a str,
) -> Result<(&'a str, i64), ScheduleDomainError> {
    let invalid = |reason: &'static str| ScheduleDomainError::InvalidTimestamp {
        field,
        value: raw.to_owned(),
        reason,
    };
    if let Some(stripped) = rest.strip_suffix('Z').or_else(|| rest.strip_suffix('z')) {
        return Ok((stripped, 0));
    }
    if let Some((clock, offset)) = rest.rsplit_once('+') {
        let secs = parse_offset_hm(offset).ok_or_else(|| invalid("invalid +HH:MM offset"))?;
        return Ok((clock, secs));
    }
    if let Some((clock, offset)) = rest.rsplit_once('-') {
        let secs = parse_offset_hm(offset).ok_or_else(|| invalid("invalid -HH:MM offset"))?;
        return Ok((clock, -secs));
    }
    Err(invalid("missing UTC 'Z' or +/-HH:MM offset"))
}

fn parse_offset_hm(offset: &str) -> Option<i64> {
    let (h_str, m_str) = offset.split_once(':')?;
    if h_str.len() != 2 || m_str.len() != 2 {
        return None;
    }
    let hours: i64 = h_str.parse().ok()?;
    let minutes: i64 = m_str.parse().ok()?;
    if !(0..=23).contains(&hours) || !(0..=59).contains(&minutes) {
        return None;
    }
    Some(hours * 3_600 + minutes * 60)
}

fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let y = i64::from(year) - if month <= 2 { 1 } else { 0 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u64;
    let m = i64::from(month);
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + i64::from(day) - 1;
    let doe = yoe as i64 * 365 + yoe as i64 / 4 - yoe as i64 / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(days: i64) -> (i32, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { y + 1 } else { y };
    (year as i32, month as u32, day as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_rfc3339_offsets_and_rejects_invalid_dates_or_windows() {
        let win =
            AppointmentWindow::parse("2026-08-27T10:00:00-04:00", "2026-08-27T14:30:00+00:00")
                .expect("valid window");
        assert_eq!(win.starts_at(), "2026-08-27T14:00:00.000Z");
        assert_eq!(win.ends_at(), "2026-08-27T14:30:00.000Z");
        assert_eq!(win.ends_at_unix() - win.starts_at_unix(), 1_800);

        assert!(matches!(
            AppointmentWindow::parse("2026-02-29T10:00:00Z", "2026-02-29T10:30:00Z"),
            Err(ScheduleDomainError::InvalidTimestamp { .. })
        ));
        assert!(matches!(
            AppointmentWindow::parse("2026-08-27T15:00:00Z", "2026-08-27T14:30:00Z"),
            Err(ScheduleDomainError::InvertedWindow { .. })
        ));
    }

    #[test]
    fn partial_failures_require_at_least_one_named_issue() {
        assert_eq!(
            ScheduleReadFailures::new(vec![]),
            Err(ScheduleDomainError::EmptyPartialFailures)
        );
        assert!(ScheduleReadIssue::new("", "bad json").is_err());
        assert!(ScheduleReadIssue::new("Appointment.ndjson:2", " ").is_err());
    }

    #[test]
    fn patient_match_requires_both_seeded_link_and_roster_membership() {
        let mrn = SourcePatientId::new("MRN-1").unwrap();
        let mut links = BTreeMap::new();
        links.insert(mrn.clone(), "pw-1".to_owned());

        let empty_roster = BTreeSet::new();
        assert_eq!(
            resolve_patient_match(Some(&mrn), &links, &empty_roster),
            PatientMatch::Unmatched {
                source_patient_id: Some(mrn.clone()),
            }
        );

        let mut roster = BTreeSet::new();
        roster.insert("pw-1".to_owned());
        assert_eq!(
            resolve_patient_match(Some(&mrn), &links, &roster),
            PatientMatch::Matched {
                workspace_id: "pw-1".to_owned(),
                source_patient_id: mrn,
            }
        );
        assert_eq!(
            resolve_patient_match(None, &links, &roster),
            PatientMatch::Unmatched {
                source_patient_id: None,
            }
        );
    }
}
