//! Staging-only synthetic event intake, source failure injection, rounds/visit
//! execution triggers, and practice reset for [`crate::STAGING_PRACTICE_ID`].
//!
//! Synthetic events never write to PostgreSQL. Accepted chart events write
//! NDJSON records exclusively into the mutable staging working bank's
//! `patients/` directory, and accepted schedule events write NDJSON records
//! exclusively into `calendar/Appointment.ndjson`. Records enter PostgreSQL
//! solely through [`PatientSourceRefresh`] / [`PatientSource`] (`rounds` or
//! source refresh) and [`ScheduleSource`] (`prepare-visits` or schedule sync),
//! executing [`PracticeJobHost`] on the shared harness with zero synthetic
//! branches in product code.

use std::collections::BTreeSet;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use axum::Json;
use axum::Router;
use axum::body::Body;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use base64::Engine as _;
use harness::engine::{self, Cancellation, RunLimits};
use harness::provider::ResponsesProvider;
use local_product::physician_sources::{
    PatientSource, ReadablePart, RecordDate, SourcePatient, SourceRevision, workspace_id,
};
use local_product::runtime::briefings::{
    BriefingError, SavedVisitBriefing, ScheduledWakeSummary, VisitPreparationRecord,
};
use local_product::runtime::care_plan::{
    CarePlanError, NewMedicationRecommendation, RecommendationStanding,
};
use local_product::runtime::concerns::{Concern, WorkspaceAccess};
use local_product::runtime::dispatch::{
    DispatchError, PatientSourceChecker, SourceCheckError, StoredObservationChecker, WakeSummary,
};
use local_product::runtime::jobs::{PracticeJobHost, PracticeScope};
use local_product::runtime::memory::PatientKey;
use local_product::runtime::refresh::{
    PatientSourceRefresh, RefreshCommandKey, RefreshError, RefreshReport, RefreshRequest,
    RefreshTarget, SavedSourceStatus, SourceCheckOutcome, SourceId as RefreshSourceId,
    SourceRefreshAdapterError, SourceRefreshOutcome,
};
use local_product::runtime::reviews::{
    PatientReviewError, PatientReviewReceiver, SavedPatientReview,
};
use local_product::runtime::schedule::{
    AppointmentWindow, ScheduleDelivery, ScheduleError, ScheduleSource, ScheduleSourceError,
    ScheduleSyncOutcome, SourceAppointment, SourceId as ScheduleSourceId,
};
use local_product::runtime::spruce_matching::{
    ChannelStanding, IncomingSpruceItem, OutsideParticipant, ReadableSpruceItem, SenderAddress,
    SpruceContactPatient, SpruceContactRecord, SpruceEhrLink, SpruceMatchingError,
};
use local_product::runtime::{Authority, PublicationTransaction, Scope, Store};
use patient_work::CheckedSources;
use physician_auth::{
    AuthStore, ClockUpdateError, PostgresAuthStore, PracticeClock, SessionError,
    SyntheticClockUpdate,
};
use sea_orm::sea_query::{Expr, Iden, Query};
use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, TransactionTrait};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::auth::{STAGING_PRACTICE_ID, bearer, now};
use crate::hosting::StagingSyntheticControlConfig;
use crate::schedule::{self, SyntheticBankCalendar};
use crate::synthetic_chart::{self, ChartRecordError, SyntheticCharts};

/// HTTP method and path of one published route on the physician server.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct PublishedRouteEntry {
    /// HTTP verb (`GET`, `POST`, `PUT`, `DELETE`).
    pub method: &'static str,
    /// Route path pattern as mounted on Axum.
    pub path: &'static str,
}

/// Staging-only synthetic control routes mounted exclusively when
/// [`StagingSyntheticControlConfig`] is enabled.
pub const STAGING_SYNTHETIC_ROUTE_MANIFEST: &[PublishedRouteEntry] = &[
    PublishedRouteEntry {
        method: "GET",
        path: "/staging-synthetic/patients",
    },
    PublishedRouteEntry {
        method: "GET",
        path: "/staging-synthetic/status",
    },
    PublishedRouteEntry {
        method: "GET",
        path: "/staging-synthetic/clock",
    },
    PublishedRouteEntry {
        method: "POST",
        path: "/staging-synthetic/clock",
    },
    PublishedRouteEntry {
        method: "GET",
        path: "/staging-synthetic/measure",
    },
    PublishedRouteEntry {
        method: "POST",
        path: "/staging-synthetic/events",
    },
    PublishedRouteEntry {
        method: "POST",
        path: "/staging-synthetic/source-failure",
    },
    PublishedRouteEntry {
        method: "POST",
        path: "/staging-synthetic/rounds",
    },
    PublishedRouteEntry {
        method: "POST",
        path: "/staging-synthetic/visits/prepare",
    },
    PublishedRouteEntry {
        method: "POST",
        path: "/staging-synthetic/reset",
    },
];

/// Stable source identifier for synthetic Duet connected-care updates (distinct
/// from the EHR `synthetic-charts` source).
pub const DUET_CONNECTED_CARE_SOURCE_ID: &str = "duet-connected-care";

/// Which synthetic source adapter an operation targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyntheticSourceKind {
    /// Synthetic EHR chart source (`synthetic-charts`).
    Chart,
    /// Synthetic practice schedule source (`synthetic-practice-calendar`).
    Calendar,
    /// Synthetic Spruce messaging channel (`spruce`).
    Spruce,
    /// Synthetic Duet connected-care patient feed (`duet-connected-care`).
    Duet,
}

impl SyntheticSourceKind {
    /// Human-readable source adapter name.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Chart => "Synthetic chart",
            Self::Calendar => schedule::SOURCE_LABEL,
            Self::Spruce => "Synthetic Spruce",
            Self::Duet => "Duet connected care",
        }
    }
}

/// Simulated health state of one synthetic source adapter.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum SourceHealthDirective {
    /// Adapter reads normally from `working_bank_dir`.
    #[default]
    Healthy,
    /// Adapter fails every read and rejects new event intake with `reason`.
    Failing {
        /// Diagnostic failure message reported by the adapter.
        reason: String,
    },
}

/// Current health directives across all synthetic source adapters.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceHealthSnapshot {
    /// Health directive of the synthetic chart source.
    pub chart: SourceHealthDirective,
    /// Health directive of the synthetic calendar source.
    pub calendar: SourceHealthDirective,
    /// Health directive of the synthetic Spruce messaging source.
    #[serde(default)]
    pub spruce: SourceHealthDirective,
    /// Health directive of the synthetic Duet connected-care source.
    #[serde(default)]
    pub duet: SourceHealthDirective,
}

/// How a synthetic event names its target patient in the synthetic bank.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "by", content = "value", rename_all = "snake_case")]
pub enum PatientSelector {
    /// Patient workspace ID (`pw-...`).
    WorkspaceId(String),
    /// Native synthetic bank patient folder ID (`MRN...`).
    SourcePatientId(String),
    /// Medical record number (`00000001`, `1`, etc.).
    MedicalRecordNumber(String),
}

impl PatientSelector {
    fn summary(&self) -> String {
        match self {
            Self::WorkspaceId(v) => format!("workspace_id={v}"),
            Self::SourcePatientId(v) => format!("source_patient_id={v}"),
            Self::MedicalRecordNumber(v) => format!("mrn={v}"),
        }
    }
}

/// Kind of free-form clinical chart event authored by an operator or test.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChartEventRecordKind {
    /// Laboratory or vital observation (`Observation`).
    LabResult,
    /// Free-form clinical note (`DocumentReference` with `text/plain` attachment).
    ClinicalNote,
    /// Diagnostic or imaging study report (`DiagnosticReport` with `text/plain` form).
    DiagnosticReport,
}

/// Appointment status for a structured synthetic schedule change.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisitChangeStatus {
    /// Scheduled / booked visit (`booked`).
    Booked,
    /// Patient arrived (`arrived`).
    Arrived,
    /// Visit completed (`fulfilled`).
    Fulfilled,
    /// Visit cancelled (`cancelled`).
    Cancelled,
    /// Patient missed the visit (`noshow`).
    NoShow,
}

impl VisitChangeStatus {
    const fn fhir_status(self) -> &'static str {
        match self {
            Self::Booked => "booked",
            Self::Arrived => "arrived",
            Self::Fulfilled => "fulfilled",
            Self::Cancelled => "cancelled",
            Self::NoShow => "noshow",
        }
    }
}

/// Canonical, replayable specification of a synthetic source event.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum SyntheticEventSpec {
    /// Free-form clinical result, note, or diagnostic report for a synthetic patient.
    FreeFormChartRecord {
        /// Target patient selector.
        patient: PatientSelector,
        /// Optional deterministic native FHIR `id`; generated when omitted.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        record_id: Option<String>,
        /// Clinical record kind.
        kind: ChartEventRecordKind,
        /// Short title / test name (for example `"Basic Metabolic Panel - Potassium"`).
        title: String,
        /// Free-form clinical prose authored by the sender.
        text: String,
        /// ISO date (`YYYY-MM-DD`) or RFC3339 timestamp (`YYYY-MM-DDTHH:MM:SSZ`).
        effective_at: String,
    },
    /// Raw FHIR chart resource validated by the synthetic chart adapter's rules.
    RawFhirChartResource {
        /// Target patient selector.
        patient: PatientSelector,
        /// Raw FHIR JSON object.
        resource: Value,
    },
    /// Structured schedule visit addition, reschedule, label change, or cancellation.
    VisitChange {
        /// Target patient selector.
        patient: PatientSelector,
        /// Stable appointment identifier (`Appointment/{appointment_id}`).
        appointment_id: String,
        /// Appointment status.
        status: VisitChangeStatus,
        /// RFC3339 visit start timestamp.
        starts_at: String,
        /// RFC3339 visit end timestamp.
        ends_at: String,
        /// Visit label / reason description.
        label: String,
    },
    /// Raw FHIR `Appointment` resource validated by the synthetic calendar adapter's rules.
    RawFhirAppointment {
        /// Raw FHIR `Appointment` JSON object.
        resource: Value,
    },
    /// Synthetic inbound Spruce patient message delivered through the real
    /// `spruce_matching` ingestion path.
    SprucePatientMessage {
        /// Target patient selector.
        patient: PatientSelector,
        /// Spruce conversation identifier.
        conversation_id: String,
        /// Spruce message identifier.
        message_id: String,
        /// Outside sender display name.
        sender_name: String,
        /// Outside sender endpoint address (for example `"+15551234567"` or `"alice@example.test"`).
        sender_contact: String,
        /// Plain-text patient message body.
        text: String,
        /// ISO date or RFC3339 timestamp when the message was sent.
        sent_at: String,
    },
    /// Synthetic Duet connected-care patient update recorded under
    /// [`DUET_CONNECTED_CARE_SOURCE_ID`] (distinct from EHR `synthetic-charts`).
    DuetPatientUpdate {
        /// Target patient selector.
        patient: PatientSelector,
        /// Optional deterministic update identifier; generated when omitted.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        record_id: Option<String>,
        /// Duet update category (for example `"home_bp"`, `"symptom_log"`, `"medication_check_in"`).
        category: String,
        /// Short title for the Duet update.
        title: String,
        /// Patient-authored or Duet-shared clinical update text.
        text: String,
        /// ISO date or RFC3339 timestamp when the update was recorded in Duet.
        observed_at: String,
    },
    /// Synthetic undecided medication recommendation held outside `active_plan_items`
    /// until an authorized panel physician explicitly decides it.
    MedicationRecommendation {
        /// Target patient selector.
        patient: PatientSelector,
        /// Optional deterministic recommendation identifier; generated when omitted.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        recommendation_id: Option<String>,
        /// Medication name and dose.
        medication: String,
        /// Clinical rationale or context.
        rationale: String,
        /// Active-plan detail shown once approved.
        detail: String,
        /// Originating note or review label.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source_label: Option<String>,
    },
}

/// Envelope returned when the synthetic source accepts an event.
///
/// Carrying the canonical [`SyntheticEventSpec`] in `event` lets callers save an
/// accepted event and replay the exact specification after a practice reset.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedSyntheticEvent {
    /// Monotonic event receipt identifier within the staging session.
    pub event_id: String,
    /// Unix second when the synthetic source accepted the event.
    pub accepted_at_unix: u64,
    /// Which synthetic source adapter accepted the record.
    pub target_source: SyntheticSourceKind,
    /// Resolved Coda patient workspace ID when matched to a synthetic chart patient.
    pub resolved_workspace_id: Option<String>,
    /// Native patient identifier in the synthetic source.
    pub resolved_source_patient_id: String,
    /// Native FHIR locator (`<resourceType>/<id>`).
    pub native_locator: String,
    /// Canonical replayable event payload.
    pub event: SyntheticEventSpec,
}

/// Why a synthetic source adapter refused an incoming event without saving anything.
#[derive(Debug)]
pub enum SyntheticEventRefusal {
    /// Target synthetic source is currently marked failing/offline.
    SourceUnavailable {
        /// Offline source adapter.
        source: SyntheticSourceKind,
        /// Failure reason configured on the adapter.
        reason: String,
    },
    /// Target patient selector did not match any patient in the synthetic chart bank.
    UnknownPatient {
        /// Summary of the selector that failed to resolve.
        selector: String,
    },
    /// Free-form clinical title was empty or whitespace.
    EmptyRecordTitle,
    /// Free-form clinical body text was empty or whitespace.
    EmptyRecordText,
    /// Record date/timestamp was empty or failed ISO-8601 / RFC3339 validation.
    InvalidTimestamp {
        /// Field name carrying the invalid date.
        field: &'static str,
        /// Rejected value.
        value: String,
    },
    /// Attachment or `presentedForm` used an unreadable media type or invalid UTF-8 base64.
    UnsupportedAttachmentMedia {
        /// Detailed adapter rejection reason.
        detail: String,
    },
    /// Raw FHIR chart resource violated adapter structural or patient-reference rules.
    MalformedFhirResource {
        /// Detailed adapter rejection reason.
        reason: String,
    },
    /// Schedule appointment violated `Appointment` schema or window rules.
    InvalidAppointment {
        /// Detailed schedule parser / domain rejection reason.
        reason: String,
    },
    /// Spruce, Duet, or care-plan synthetic event failed payload validation.
    InvalidEventPayload {
        /// Detailed payload validation error.
        reason: String,
    },
    /// Working bank I/O failed before or during file write.
    StorageIo {
        /// Underlying filesystem error.
        source: std::io::Error,
    },
}

impl SyntheticEventRefusal {
    /// Stable machine-readable refusal code returned in HTTP `422` / `503` bodies.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::SourceUnavailable { .. } => "source_unavailable",
            Self::UnknownPatient { .. } => "unknown_patient",
            Self::EmptyRecordTitle => "empty_record_title",
            Self::EmptyRecordText => "empty_record_text",
            Self::InvalidTimestamp { .. } => "invalid_timestamp",
            Self::UnsupportedAttachmentMedia { .. } => "unsupported_attachment_media",
            Self::MalformedFhirResource { .. } => "malformed_fhir_resource",
            Self::InvalidAppointment { .. } => "invalid_appointment",
            Self::InvalidEventPayload { .. } => "invalid_event_payload",
            Self::StorageIo { .. } => "source_storage_io",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::StorageIo { .. } => StatusCode::SERVICE_UNAVAILABLE,
            Self::SourceUnavailable { .. }
            | Self::UnknownPatient { .. }
            | Self::EmptyRecordTitle
            | Self::EmptyRecordText
            | Self::InvalidTimestamp { .. }
            | Self::UnsupportedAttachmentMedia { .. }
            | Self::MalformedFhirResource { .. }
            | Self::InvalidAppointment { .. }
            | Self::InvalidEventPayload { .. } => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
}

impl std::fmt::Display for SyntheticEventRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceUnavailable { source, reason } => {
                write!(f, "{} cannot accept events: {reason}", source.label())
            }
            Self::UnknownPatient { selector } => {
                write!(f, "synthetic source holds no patient matching {selector}")
            }
            Self::EmptyRecordTitle => f.write_str("chart event title must not be empty"),
            Self::EmptyRecordText => f.write_str("chart event clinical text must not be empty"),
            Self::InvalidTimestamp { field, value } => write!(
                f,
                "{field} `{value}` must be a valid YYYY-MM-DD date or RFC3339 timestamp"
            ),
            Self::UnsupportedAttachmentMedia { detail } => {
                write!(f, "synthetic chart source refused attachment: {detail}")
            }
            Self::MalformedFhirResource { reason } => {
                write!(f, "synthetic chart source refused FHIR resource: {reason}")
            }
            Self::InvalidAppointment { reason } => {
                write!(f, "synthetic calendar source refused appointment: {reason}")
            }
            Self::InvalidEventPayload { reason } => {
                write!(f, "synthetic event payload refused: {reason}")
            }
            Self::StorageIo { source } => write!(f, "synthetic working bank I/O failed: {source}"),
        }
    }
}

impl std::error::Error for SyntheticEventRefusal {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::StorageIo { source } => Some(source),
            Self::SourceUnavailable { .. }
            | Self::UnknownPatient { .. }
            | Self::EmptyRecordTitle
            | Self::EmptyRecordText
            | Self::InvalidTimestamp { .. }
            | Self::UnsupportedAttachmentMedia { .. }
            | Self::MalformedFhirResource { .. }
            | Self::InvalidAppointment { .. }
            | Self::InvalidEventPayload { .. } => None,
        }
    }
}

const HEALTH_STATE_FILE: &str = ".staging-synthetic-health.json";
const ACCEPTED_EVENTS_FILE: &str = ".staging-synthetic-events.ndjson";

fn initial_monotonic_seed() -> u64 {
    u64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros(),
    )
    .unwrap_or(1)
}

/// Mutable working-bank wrapper implementing [`PatientSource`],
/// [`PatientSourceRefresh`], and [`ScheduleSource`] with failure injection.
pub struct StagingSyntheticSources {
    base_bank_dir: PathBuf,
    working_bank_dir: PathBuf,
    chart_refresh_id: RefreshSourceId,
    calendar_source_id: ScheduleSourceId,
    health: RwLock<SourceHealthSnapshot>,
    clock_unix: AtomicU64,
    event_seq: AtomicU64,
}

impl StagingSyntheticSources {
    /// Creates a synthetic source set backed by `base_bank_dir` and `working_bank_dir`.
    pub fn new(config: &StagingSyntheticControlConfig, initial_clock_unix: u64) -> Self {
        let initial_health = fs::read_to_string(config.working_bank_dir.join(HEALTH_STATE_FILE))
            .ok()
            .and_then(|raw| serde_json::from_str::<SourceHealthSnapshot>(&raw).ok())
            .unwrap_or_default();
        Self {
            base_bank_dir: config.base_bank_dir.clone(),
            working_bank_dir: config.working_bank_dir.clone(),
            chart_refresh_id: RefreshSourceId::new(synthetic_chart::SOURCE_ID)
                .expect("static synthetic chart source_id"),
            calendar_source_id: ScheduleSourceId::new(schedule::SOURCE_ID)
                .expect("static synthetic calendar source_id"),
            health: RwLock::new(initial_health),
            clock_unix: AtomicU64::new(initial_clock_unix),
            event_seq: AtomicU64::new(initial_monotonic_seed()),
        }
    }

    /// Path of the mutable working bank directory.
    pub fn working_bank_dir(&self) -> &Path {
        &self.working_bank_dir
    }

    /// Path of the canonical read-only base bank directory.
    pub fn base_bank_dir(&self) -> &Path {
        &self.base_bank_dir
    }

    /// Updates the timestamp reported on adapter refresh checks.
    pub fn set_clock_unix(&self, unix_secs: u64) {
        self.clock_unix.store(unix_secs, Ordering::SeqCst);
    }

    /// Current simulated health directives of both synthetic adapters.
    pub fn health_snapshot(&self) -> SourceHealthSnapshot {
        if let Ok(raw) = fs::read_to_string(self.working_bank_dir.join(HEALTH_STATE_FILE))
            && let Ok(disk_snapshot) = serde_json::from_str::<SourceHealthSnapshot>(&raw)
        {
            return disk_snapshot;
        }
        self.health
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
    }

    /// Sets the health directive for `source` and persists it inside `working_bank_dir`
    /// when the working bank exists on disk.
    pub fn set_source_health(&self, source: SyntheticSourceKind, directive: SourceHealthDirective) {
        let mut guard = self
            .health
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut next = fs::read_to_string(self.working_bank_dir.join(HEALTH_STATE_FILE))
            .ok()
            .and_then(|raw| serde_json::from_str::<SourceHealthSnapshot>(&raw).ok())
            .unwrap_or_else(|| guard.clone());
        match source {
            SyntheticSourceKind::Chart => next.chart = directive,
            SyntheticSourceKind::Calendar => next.calendar = directive,
            SyntheticSourceKind::Spruce => next.spruce = directive,
            SyntheticSourceKind::Duet => next.duet = directive,
        }
        if self.working_bank_dir.is_dir()
            && let Ok(serialized) = serde_json::to_vec(&next)
        {
            let _ = fs::write(self.working_bank_dir.join(HEALTH_STATE_FILE), serialized);
        }
        *guard = next;
    }

    /// Reads accepted synthetic events persisted in `working_bank_dir`.
    pub fn read_accepted_events(&self) -> Vec<SavedSyntheticEvent> {
        let Ok(raw) = fs::read_to_string(self.working_bank_dir.join(ACCEPTED_EVENTS_FILE)) else {
            return Vec::new();
        };
        raw.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .filter_map(|line| serde_json::from_str::<SavedSyntheticEvent>(line).ok())
            .collect()
    }

    fn append_accepted_event(&self, event: &SavedSyntheticEvent) -> Result<(), std::io::Error> {
        if !self.working_bank_dir.is_dir() {
            return Ok(());
        }
        let mut line = serde_json::to_string(event)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
        line.push('\n');
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.working_bank_dir.join(ACCEPTED_EVENTS_FILE))?;
        file.write_all(line.as_bytes())
    }

    /// Restores `working_bank_dir` cleanly from `base_bank_dir` and clears any
    /// simulated source failures and persisted event ledger.
    pub fn restore_working_bank_from_base(&self) -> Result<(), std::io::Error> {
        if self.working_bank_dir.exists() {
            fs::remove_dir_all(&self.working_bank_dir)?;
        }
        copy_dir_recursive(&self.base_bank_dir, &self.working_bank_dir)?;
        let mut guard = self
            .health
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        *guard = SourceHealthSnapshot::default();
        Ok(())
    }

    /// Ensures `working_bank_dir` is populated from `base_bank_dir` when absent.
    pub fn ensure_working_bank_initialized(&self) -> Result<(), std::io::Error> {
        if !self.working_bank_dir.join("patients").is_dir() {
            self.restore_working_bank_from_base()?;
        }
        Ok(())
    }

    /// Validates `spec` strictly against the synthetic source adapter rules and,
    /// if valid, writes only to `working_bank_dir` (never PostgreSQL).
    pub async fn accept_event(
        &self,
        mut spec: SyntheticEventSpec,
        accepted_at_unix: u64,
    ) -> Result<SavedSyntheticEvent, SyntheticEventRefusal> {
        let target_source = match &spec {
            SyntheticEventSpec::FreeFormChartRecord { .. }
            | SyntheticEventSpec::RawFhirChartResource { .. }
            | SyntheticEventSpec::MedicationRecommendation { .. } => SyntheticSourceKind::Chart,
            SyntheticEventSpec::VisitChange { .. }
            | SyntheticEventSpec::RawFhirAppointment { .. } => SyntheticSourceKind::Calendar,
            SyntheticEventSpec::SprucePatientMessage { .. } => SyntheticSourceKind::Spruce,
            SyntheticEventSpec::DuetPatientUpdate { .. } => SyntheticSourceKind::Duet,
        };

        let directive = {
            let health = self.health_snapshot();
            match target_source {
                SyntheticSourceKind::Chart => health.chart,
                SyntheticSourceKind::Calendar => health.calendar,
                SyntheticSourceKind::Spruce => health.spruce,
                SyntheticSourceKind::Duet => health.duet,
            }
        };
        if let SourceHealthDirective::Failing { reason } = directive {
            return Err(SyntheticEventRefusal::SourceUnavailable {
                source: target_source,
                reason,
            });
        }

        let seq = self.event_seq.fetch_add(1, Ordering::SeqCst) + 1;
        let event_id = format!("syn-evt-{seq:04}");

        match &mut spec {
            SyntheticEventSpec::FreeFormChartRecord {
                patient,
                record_id,
                kind,
                title,
                text,
                effective_at,
            } => {
                let resolved = self.resolve_chart_patient(patient).await?;
                let trimmed_title = title.trim().to_owned();
                if trimmed_title.is_empty() {
                    return Err(SyntheticEventRefusal::EmptyRecordTitle);
                }
                let trimmed_text = text.trim().to_owned();
                if trimmed_text.is_empty() {
                    return Err(SyntheticEventRefusal::EmptyRecordText);
                }
                let trimmed_date = effective_at.trim().to_owned();
                validate_iso_date_or_rfc3339("effective_at", &trimmed_date)?;

                let assigned_id = match record_id {
                    Some(explicit) => {
                        let cleaned = explicit.trim().to_owned();
                        validate_fhir_id(&cleaned)?;
                        cleaned
                    }
                    None => {
                        let generated = format!("syn-{seq:04}");
                        *record_id = Some(generated.clone());
                        generated
                    }
                };
                *title = trimmed_title.clone();
                *text = trimmed_text.clone();
                *effective_at = trimmed_date.clone();

                let resource = build_free_form_fhir_resource(
                    *kind,
                    &assigned_id,
                    &resolved.source_patient_id,
                    &trimmed_title,
                    &trimmed_text,
                    &trimmed_date,
                );
                let (record_type, line, native_locator) = validate_chart_resource_for_patient(
                    &resolved.workspace_id,
                    &resolved.patient,
                    &resource,
                )?;
                append_ndjson_line(
                    &self
                        .working_bank_dir
                        .join("patients")
                        .join(&resolved.source_patient_id)
                        .join(format!("{record_type}.ndjson")),
                    &line,
                )?;

                Ok(SavedSyntheticEvent {
                    event_id,
                    accepted_at_unix,
                    target_source,
                    resolved_workspace_id: Some(resolved.workspace_id),
                    resolved_source_patient_id: resolved.source_patient_id,
                    native_locator,
                    event: spec,
                })
            }
            SyntheticEventSpec::RawFhirChartResource { patient, resource } => {
                let resolved = self.resolve_chart_patient(patient).await?;
                let (record_type, line, native_locator) = validate_chart_resource_for_patient(
                    &resolved.workspace_id,
                    &resolved.patient,
                    resource,
                )?;
                append_ndjson_line(
                    &self
                        .working_bank_dir
                        .join("patients")
                        .join(&resolved.source_patient_id)
                        .join(format!("{record_type}.ndjson")),
                    &line,
                )?;

                Ok(SavedSyntheticEvent {
                    event_id,
                    accepted_at_unix,
                    target_source,
                    resolved_workspace_id: Some(resolved.workspace_id),
                    resolved_source_patient_id: resolved.source_patient_id,
                    native_locator,
                    event: spec,
                })
            }
            SyntheticEventSpec::VisitChange {
                patient,
                appointment_id,
                status,
                starts_at,
                ends_at,
                label,
            } => {
                let resolved = self.resolve_chart_patient(patient).await?;
                let trimmed_id = appointment_id.trim().to_owned();
                let trimmed_label = label.trim().to_owned();
                let trimmed_start = starts_at.trim().to_owned();
                let trimmed_end = ends_at.trim().to_owned();
                let resource = json!({
                    "resourceType": "Appointment",
                    "id": trimmed_id,
                    "status": status.fhir_status(),
                    "description": trimmed_label,
                    "start": trimmed_start,
                    "end": trimmed_end,
                    "participant": [
                        {
                            "actor": {
                                "reference": format!("Patient/{}", resolved.source_patient_id)
                            },
                            "status": "accepted"
                        }
                    ]
                });
                let parsed = validate_fhir_appointment_resource(&resource)?;
                *appointment_id = parsed.appointment_id().as_str().to_owned();
                *label = trimmed_label;
                *starts_at = trimmed_start;
                *ends_at = trimmed_end;
                let line = serde_json::to_string(&resource)
                    .expect("validated Appointment Value serializes");
                upsert_appointment_ndjson_line(
                    &self
                        .working_bank_dir
                        .join("calendar")
                        .join("Appointment.ndjson"),
                    parsed.appointment_id().as_str(),
                    &line,
                )?;

                Ok(SavedSyntheticEvent {
                    event_id,
                    accepted_at_unix,
                    target_source,
                    resolved_workspace_id: Some(resolved.workspace_id),
                    resolved_source_patient_id: resolved.source_patient_id,
                    native_locator: format!("Appointment/{}", parsed.appointment_id()),
                    event: spec,
                })
            }
            SyntheticEventSpec::RawFhirAppointment { resource } => {
                let parsed = validate_fhir_appointment_resource(resource)?;
                let Some(source_patient_id) = parsed.source_patient_id().cloned() else {
                    return Err(SyntheticEventRefusal::InvalidAppointment {
                        reason: "Appointment must include a Patient/<id> participant".to_owned(),
                    });
                };
                let resolved = self
                    .resolve_chart_patient(&PatientSelector::SourcePatientId(
                        source_patient_id.as_str().to_owned(),
                    ))
                    .await?;
                let line = serde_json::to_string(resource)
                    .expect("validated Appointment Value serializes");
                upsert_appointment_ndjson_line(
                    &self
                        .working_bank_dir
                        .join("calendar")
                        .join("Appointment.ndjson"),
                    parsed.appointment_id().as_str(),
                    &line,
                )?;

                Ok(SavedSyntheticEvent {
                    event_id,
                    accepted_at_unix,
                    target_source,
                    resolved_workspace_id: Some(resolved.workspace_id),
                    resolved_source_patient_id: resolved.source_patient_id,
                    native_locator: format!("Appointment/{}", parsed.appointment_id()),
                    event: spec,
                })
            }
            SyntheticEventSpec::SprucePatientMessage {
                patient,
                conversation_id,
                message_id,
                sender_name,
                sender_contact,
                text,
                sent_at,
            } => {
                let resolved = self.resolve_chart_patient(patient).await?;
                let trimmed_conv = conversation_id.trim().to_owned();
                let trimmed_msg = message_id.trim().to_owned();
                let trimmed_name = sender_name.trim().to_owned();
                let trimmed_contact = sender_contact.trim().to_owned();
                let trimmed_text = text.trim().to_owned();
                let trimmed_sent = sent_at.trim().to_owned();
                if trimmed_conv.is_empty() || trimmed_msg.is_empty() {
                    return Err(SyntheticEventRefusal::InvalidEventPayload {
                        reason: "Spruce conversation_id and message_id must not be empty"
                            .to_owned(),
                    });
                }
                if trimmed_name.is_empty() || trimmed_contact.is_empty() {
                    return Err(SyntheticEventRefusal::InvalidEventPayload {
                        reason: "Spruce sender_name and sender_contact must not be empty"
                            .to_owned(),
                    });
                }
                if trimmed_text.is_empty() {
                    return Err(SyntheticEventRefusal::EmptyRecordText);
                }
                validate_iso_date_or_rfc3339("sent_at", &trimmed_sent)?;
                SenderAddress::parse(&trimmed_contact).map_err(|err| {
                    SyntheticEventRefusal::InvalidEventPayload {
                        reason: format!("invalid Spruce sender_contact: {err}"),
                    }
                })?;

                *conversation_id = trimmed_conv.clone();
                *message_id = trimmed_msg.clone();
                *sender_name = trimmed_name;
                *sender_contact = trimmed_contact;
                *text = trimmed_text;
                *sent_at = trimmed_sent;

                Ok(SavedSyntheticEvent {
                    event_id,
                    accepted_at_unix,
                    target_source,
                    resolved_workspace_id: Some(resolved.workspace_id),
                    resolved_source_patient_id: resolved.source_patient_id,
                    native_locator: format!("SpruceMessage/{trimmed_conv}/{trimmed_msg}"),
                    event: spec,
                })
            }
            SyntheticEventSpec::DuetPatientUpdate {
                patient,
                record_id,
                category,
                title,
                text,
                observed_at,
            } => {
                let resolved = self.resolve_chart_patient(patient).await?;
                let trimmed_category = category.trim().to_owned();
                let trimmed_title = title.trim().to_owned();
                let trimmed_text = text.trim().to_owned();
                let trimmed_observed = observed_at.trim().to_owned();
                if trimmed_category.is_empty() {
                    return Err(SyntheticEventRefusal::InvalidEventPayload {
                        reason: "Duet update category must not be empty".to_owned(),
                    });
                }
                if trimmed_title.is_empty() {
                    return Err(SyntheticEventRefusal::EmptyRecordTitle);
                }
                if trimmed_text.is_empty() {
                    return Err(SyntheticEventRefusal::EmptyRecordText);
                }
                validate_iso_date_or_rfc3339("observed_at", &trimmed_observed)?;
                let assigned_id = match record_id {
                    Some(explicit) => {
                        let cleaned = explicit.trim().to_owned();
                        validate_fhir_id(&cleaned)?;
                        cleaned
                    }
                    None => {
                        let generated = format!("duet-{seq:04}");
                        *record_id = Some(generated.clone());
                        generated
                    }
                };
                *category = trimmed_category;
                *title = trimmed_title;
                *text = trimmed_text;
                *observed_at = trimmed_observed;

                Ok(SavedSyntheticEvent {
                    event_id,
                    accepted_at_unix,
                    target_source,
                    resolved_workspace_id: Some(resolved.workspace_id),
                    resolved_source_patient_id: resolved.source_patient_id,
                    native_locator: format!("DuetUpdate/{assigned_id}"),
                    event: spec,
                })
            }
            SyntheticEventSpec::MedicationRecommendation {
                patient,
                recommendation_id,
                medication,
                rationale,
                detail,
                source_label,
            } => {
                let resolved = self.resolve_chart_patient(patient).await?;
                let trimmed_med = medication.trim().to_owned();
                let trimmed_rationale = rationale.trim().to_owned();
                let trimmed_detail = detail.trim().to_owned();
                if trimmed_med.is_empty() {
                    return Err(SyntheticEventRefusal::InvalidEventPayload {
                        reason: "medication must not be empty".to_owned(),
                    });
                }
                if trimmed_rationale.is_empty() {
                    return Err(SyntheticEventRefusal::InvalidEventPayload {
                        reason: "rationale must not be empty".to_owned(),
                    });
                }
                let assigned_id = match recommendation_id {
                    Some(explicit) => {
                        let cleaned = explicit.trim().to_owned();
                        validate_fhir_id(&cleaned)?;
                        cleaned
                    }
                    None => {
                        let generated = format!("rec-{seq:04}");
                        *recommendation_id = Some(generated.clone());
                        generated
                    }
                };
                *medication = trimmed_med;
                *rationale = trimmed_rationale;
                *detail = if trimmed_detail.is_empty() {
                    rationale.clone()
                } else {
                    trimmed_detail
                };
                *source_label = source_label
                    .as_deref()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_owned);

                Ok(SavedSyntheticEvent {
                    event_id,
                    accepted_at_unix,
                    target_source,
                    resolved_workspace_id: Some(resolved.workspace_id),
                    resolved_source_patient_id: resolved.source_patient_id,
                    native_locator: format!("MedicationRecommendation/{assigned_id}"),
                    event: spec,
                })
            }
        }
    }

    async fn resolve_chart_patient(
        &self,
        selector: &PatientSelector,
    ) -> Result<ResolvedChartPatient, SyntheticEventRefusal> {
        let charts = SyntheticCharts::open(&self.working_bank_dir);
        let ids = charts.patient_ids().await.map_err(|err| {
            SyntheticEventRefusal::MalformedFhirResource {
                reason: err.to_string(),
            }
        })?;
        for id in ids {
            let ws = workspace_id(charts.source_id(), &id);
            let patient = charts.patient(&id).await.map_err(|err| {
                SyntheticEventRefusal::MalformedFhirResource {
                    reason: err.to_string(),
                }
            })?;
            let matched = match selector {
                PatientSelector::WorkspaceId(target) => ws == target.trim(),
                PatientSelector::SourcePatientId(target) => id == target.trim(),
                PatientSelector::MedicalRecordNumber(target) => {
                    let needle = target.trim();
                    !needle.is_empty()
                        && patient
                            .medical_record_numbers
                            .iter()
                            .any(|mrn| mrn.value.trim().eq_ignore_ascii_case(needle))
                }
            };
            if matched {
                return Ok(ResolvedChartPatient {
                    workspace_id: ws,
                    source_patient_id: id,
                    patient,
                });
            }
        }
        Err(SyntheticEventRefusal::UnknownPatient {
            selector: selector.summary(),
        })
    }
}

struct ResolvedChartPatient {
    workspace_id: String,
    source_patient_id: String,
    patient: SourcePatient,
}

#[async_trait]
impl PatientSource for StagingSyntheticSources {
    fn source_id(&self) -> &str {
        synthetic_chart::SOURCE_ID
    }

    fn source_label(&self) -> &str {
        SyntheticSourceKind::Chart.label()
    }

    async fn patient_ids(
        &self,
    ) -> Result<Vec<String>, local_product::physician_sources::SourceAdapterError> {
        if let SourceHealthDirective::Failing { reason } = self.health_snapshot().chart {
            return Err(
                local_product::physician_sources::SourceAdapterError::Unavailable { reason },
            );
        }
        SyntheticCharts::open(&self.working_bank_dir)
            .patient_ids()
            .await
    }

    async fn patient(
        &self,
        source_patient_id: &str,
    ) -> Result<SourcePatient, local_product::physician_sources::SourceAdapterError> {
        if let SourceHealthDirective::Failing { reason } = self.health_snapshot().chart {
            return Err(
                local_product::physician_sources::SourceAdapterError::Unavailable { reason },
            );
        }
        SyntheticCharts::open(&self.working_bank_dir)
            .patient(source_patient_id)
            .await
    }

    async fn records(
        &self,
        patient: &SourcePatient,
        workspace: &str,
    ) -> Result<Vec<SourceRevision>, local_product::physician_sources::SourceAdapterError> {
        if let SourceHealthDirective::Failing { reason } = self.health_snapshot().chart {
            return Err(
                local_product::physician_sources::SourceAdapterError::Unavailable { reason },
            );
        }
        SyntheticCharts::open(&self.working_bank_dir)
            .records(patient, workspace)
            .await
    }
}

#[async_trait]
impl PatientSourceRefresh for StagingSyntheticSources {
    async fn configured_sources(
        &self,
        practice: &str,
        _workspace: &str,
    ) -> Result<Vec<RefreshSourceId>, SourceRefreshAdapterError> {
        if practice != STAGING_PRACTICE_ID {
            return Ok(Vec::new());
        }
        Ok(vec![self.chart_refresh_id.clone()])
    }

    async fn check_source(
        &self,
        _practice: &str,
        workspace: &str,
        source_id: &RefreshSourceId,
    ) -> Result<SourceCheckOutcome, SourceRefreshAdapterError> {
        let checked_at = i64::try_from(self.clock_unix.load(Ordering::SeqCst)).unwrap_or(i64::MAX);
        if source_id != &self.chart_refresh_id {
            return Err(SourceRefreshAdapterError::Interrupted {
                source_id: source_id.clone(),
                reason: format!("unrecognized source_id `{source_id}`"),
            });
        }
        if let SourceHealthDirective::Failing { reason } = self.health_snapshot().chart {
            return Ok(SourceCheckOutcome::Failed {
                source_label: Some(SyntheticSourceKind::Chart.label().to_owned()),
                checked_at,
                reason,
            });
        }

        let charts = SyntheticCharts::open(&self.working_bank_dir);
        let ids = match charts.patient_ids().await {
            Ok(ids) => ids,
            Err(reason) => {
                return Ok(SourceCheckOutcome::Failed {
                    source_label: Some(charts.source_label().to_owned()),
                    checked_at,
                    reason: reason.to_string(),
                });
            }
        };
        let Some(source_patient_id) = ids
            .into_iter()
            .find(|id| workspace_id(charts.source_id(), id) == workspace)
        else {
            return Ok(SourceCheckOutcome::Failed {
                source_label: Some(charts.source_label().to_owned()),
                checked_at,
                reason: format!(
                    "patient workspace `{workspace}` is absent from synthetic chart source"
                ),
            });
        };

        let patient = match charts.patient(&source_patient_id).await {
            Ok(patient) => patient,
            Err(reason) => {
                return Ok(SourceCheckOutcome::Failed {
                    source_label: Some(charts.source_label().to_owned()),
                    checked_at,
                    reason: reason.to_string(),
                });
            }
        };
        let records = match charts.records(&patient, workspace).await {
            Ok(records) => records,
            Err(reason) => {
                return Ok(SourceCheckOutcome::Failed {
                    source_label: Some(charts.source_label().to_owned()),
                    checked_at,
                    reason: reason.to_string(),
                });
            }
        };
        Ok(SourceCheckOutcome::Updated {
            source_label: charts.source_label().to_owned(),
            patient,
            pulled_at: checked_at,
            records,
        })
    }
}

#[async_trait]
impl ScheduleSource for StagingSyntheticSources {
    fn source_id(&self) -> &ScheduleSourceId {
        &self.calendar_source_id
    }

    fn source_label(&self) -> &str {
        schedule::SOURCE_LABEL
    }

    async fn read_schedule(&self) -> Result<ScheduleDelivery, ScheduleSourceError> {
        if let SourceHealthDirective::Failing { reason } = self.health_snapshot().calendar {
            return Err(ScheduleSourceError::Unavailable {
                source_id: self.calendar_source_id.clone(),
                reason,
            });
        }
        SyntheticBankCalendar::open(&self.working_bank_dir)
            .read_schedule()
            .await
    }
}

/// [`PatientSourceChecker`] bridging `wake_practice_dispatch` through
/// [`Store::refresh_patient_sources_for_practice`] and [`StoredObservationChecker`].
struct RefreshingSourceChecker<'a> {
    store: &'a Store,
    auth: &'a PostgresAuthStore,
    sources: &'a StagingSyntheticSources,
    command_seq: &'a AtomicU64,
}

#[async_trait]
impl PatientSourceChecker for RefreshingSourceChecker<'_> {
    async fn check_patient_sources(
        &self,
        practice_id: &str,
        workspace_id: &str,
        now_sec: i64,
    ) -> Result<CheckedSources, SourceCheckError> {
        // Record previous observation BEFORE refreshing so `CheckedSources` compares
        // the newly refreshed source store against what dispatch last checked!
        let observed = StoredObservationChecker::new(self.store.clone());
        self.sources
            .set_clock_unix(u64::try_from(now_sec.max(0)).unwrap_or(0));
        let seq = self.command_seq.fetch_add(1, Ordering::SeqCst) + 1;
        let command_key = RefreshCommandKey::new(format!("rounds-refresh-{now_sec}-{seq}"))
            .map_err(|err| SourceCheckError::Upstream(std::io::Error::other(err.to_string())))?;
        let request = RefreshRequest::new(command_key, RefreshTarget::AllSources);
        let scope = PracticeScope {
            practice: practice_id.to_owned(),
            workspace: workspace_id.to_owned(),
            job_id: format!("rounds-check-{seq}"),
        };
        let report = self
            .store
            .refresh_patient_sources_for_practice(&scope, self.auth, &request, self.sources)
            .await
            .map_err(|err| SourceCheckError::Upstream(std::io::Error::other(err.to_string())))?;
        for status in &report.sources {
            if let SourceRefreshOutcome::Failed { reason, .. } = &status.outcome {
                return Err(SourceCheckError::Upstream(std::io::Error::other(
                    reason.clone(),
                )));
            }
        }
        observed
            .check_patient_sources(practice_id, workspace_id, now_sec)
            .await
    }
}

/// Model endpoint configuration for executing background review and visit-preparation
/// jobs triggered from the staging synthetic control surface.
#[derive(Clone, Debug)]
pub enum SyntheticModelEndpoint {
    /// OpenAI Responses HTTP endpoint (production API or local `ResponsesStub`).
    Responses {
        /// Full Responses URL (`https://api.openai.com/v1/responses` or `http://127.0.0.1:PORT/responses`).
        endpoint: String,
        /// Bearer API key.
        key: String,
    },
    /// Local developer ChatGPT plan subscription token (unhosted local runs only).
    DeveloperPlan,
    /// No model execution configured on this process.
    Unconfigured,
}

impl SyntheticModelEndpoint {
    /// Resolves model execution settings from environment variables.
    pub fn from_env(get: impl Fn(&str) -> Option<String>, hosted: bool) -> Result<Self, String> {
        let configured = |key: &str| {
            get(key)
                .map(|value| value.trim().to_owned())
                .filter(|value| !value.is_empty())
        };
        if let Some(port) = configured("PHYSICIAN_RESPONSES_STUB_PORT") {
            if hosted {
                return Err(
                    "PHYSICIAN_RESPONSES_STUB_PORT is forbidden in hosted execution".into(),
                );
            }
            let port: u16 = port
                .parse()
                .map_err(|err| format!("invalid PHYSICIAN_RESPONSES_STUB_PORT: {err}"))?;
            return Ok(Self::Responses {
                endpoint: format!("http://127.0.0.1:{port}/responses"),
                key: "local-proof".to_owned(),
            });
        }
        if hosted {
            if let Ok(key) = local_product::open_ai_api_key(
                configured("CODA_ALLOW_MODEL_CALLS").as_deref(),
                configured("OPENAI_API_KEY").as_deref(),
            ) {
                return Ok(Self::Responses {
                    endpoint: harness::responses::ENDPOINT.to_owned(),
                    key,
                });
            }
            return Ok(Self::Unconfigured);
        }
        match local_product::model_execution(configured("CODA_LOCAL_MODEL_EXECUTION").as_deref())? {
            local_product::ModelExecution::Subscription => Ok(Self::DeveloperPlan),
            local_product::ModelExecution::Api => {
                let key = local_product::open_ai_api_key(
                    configured("CODA_ALLOW_MODEL_CALLS").as_deref(),
                    configured("OPENAI_API_KEY").as_deref(),
                )?;
                Ok(Self::Responses {
                    endpoint: harness::responses::ENDPOINT.to_owned(),
                    key,
                })
            }
            local_product::ModelExecution::Disabled => Ok(Self::Unconfigured),
        }
    }

    /// Adapter identifier passed to [`local_product::model_roles`].
    pub const fn adapter(&self) -> &'static str {
        match self {
            Self::DeveloperPlan => "chatgpt-plan",
            Self::Responses { .. } | Self::Unconfigured => "responses",
        }
    }

    async fn provider(&self, store: &Store) -> Result<ResponsesProvider, SyntheticControlError> {
        let quota: Arc<dyn harness::provider::QuotaGate> = Arc::new(store.clone());
        match self {
            Self::Responses { endpoint, key } => {
                ResponsesProvider::new(endpoint.clone(), key.clone(), quota)
                    .map_err(SyntheticControlError::ModelProviderInit)
            }
            Self::DeveloperPlan => {
                let token = local_product::chatgpt::access_token()
                    .await
                    .map_err(|err| SyntheticControlError::ModelProviderInit(err.to_string()))?;
                ResponsesProvider::chatgpt_plan(token, quota)
                    .map_err(SyntheticControlError::ModelProviderInit)
            }
            Self::Unconfigured => Err(SyntheticControlError::ModelUnconfigured),
        }
    }
}

/// Errors produced by staging synthetic control operations.
#[derive(Debug)]
pub enum SyntheticControlError {
    /// Synthetic event failed source-level validation.
    EventRefused(SyntheticEventRefusal),
    /// Working bank restoration failed on disk.
    WorkingBankIo(std::io::Error),
    /// Loading base synthetic charts failed.
    ChartLoad(String),
    /// Auth store query failed.
    AuthUnavailable(physician_auth::StoreUnavailable),
    /// Persisted simulation clock update failed.
    ClockUpdate(ClockUpdateError),
    /// Runtime database query failed.
    Database(DbErr),
    /// Source refresh failed.
    Refresh(RefreshError),
    /// Practice dispatch wake failed.
    Dispatch(DispatchError),
    /// Patient review query failed.
    Review(PatientReviewError),
    /// Schedule sync failed.
    Schedule(ScheduleError),
    /// Visit preparation wake or briefing query failed.
    Briefing(BriefingError),
    /// Synthetic Spruce channel ingestion failed.
    Spruce(SpruceMatchingError),
    /// Synthetic care-plan recommendation operation failed.
    CarePlan(CarePlanError),
    /// Model provider is not configured on this process.
    ModelUnconfigured,
    /// Model provider initialization failed.
    ModelProviderInit(String),
    /// Background practice job execution failed.
    JobExecution(String),
}

impl std::fmt::Display for SyntheticControlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EventRefused(err) => write!(f, "{err}"),
            Self::WorkingBankIo(err) => write!(f, "working bank I/O failed: {err}"),
            Self::ChartLoad(reason) => write!(f, "synthetic chart load failed: {reason}"),
            Self::AuthUnavailable(err) => write!(f, "auth store unavailable: {err}"),
            Self::ClockUpdate(err) => write!(f, "simulation clock update failed: {err}"),
            Self::Database(err) => write!(f, "runtime database error: {err}"),
            Self::Refresh(err) => write!(f, "source refresh error: {err}"),
            Self::Dispatch(err) => write!(f, "practice dispatch error: {err}"),
            Self::Review(err) => write!(f, "patient review error: {err}"),
            Self::Schedule(err) => write!(f, "schedule sync error: {err}"),
            Self::Briefing(err) => write!(f, "visit preparation error: {err}"),
            Self::Spruce(err) => write!(f, "spruce matching error: {err}"),
            Self::CarePlan(err) => write!(f, "care plan error: {err}"),
            Self::ModelUnconfigured => {
                f.write_str("model execution is not configured for staging synthetic control")
            }
            Self::ModelProviderInit(reason) => {
                write!(f, "model provider initialization failed: {reason}")
            }
            Self::JobExecution(reason) => write!(f, "practice job execution failed: {reason}"),
        }
    }
}

impl std::error::Error for SyntheticControlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::EventRefused(err) => Some(err),
            Self::WorkingBankIo(err) => Some(err),
            Self::AuthUnavailable(err) => Some(err),
            Self::ClockUpdate(err) => Some(err),
            Self::Database(err) => Some(err),
            Self::Refresh(err) => Some(err),
            Self::Dispatch(err) => Some(err),
            Self::Review(err) => Some(err),
            Self::Schedule(err) => Some(err),
            Self::Briefing(err) => Some(err),
            Self::Spruce(err) => Some(err),
            Self::CarePlan(err) => Some(err),
            Self::ChartLoad(_)
            | Self::ModelUnconfigured
            | Self::ModelProviderInit(_)
            | Self::JobExecution(_) => None,
        }
    }
}

impl IntoResponse for SyntheticControlError {
    fn into_response(self) -> Response {
        match self {
            Self::EventRefused(refusal) => (
                refusal.status_code(),
                Json(json!({
                    "status": "refused",
                    "reason_code": refusal.code(),
                    "detail": refusal.to_string(),
                })),
            )
                .into_response(),
            Self::ClockUpdate(err) => {
                let status = match &err {
                    ClockUpdateError::InvalidAdvance { .. }
                    | ClockUpdateError::RealTimePractice { .. }
                    | ClockUpdateError::TimestampOutOfRange { .. } => {
                        StatusCode::UNPROCESSABLE_ENTITY
                    }
                    ClockUpdateError::PracticeNotFound { .. } => StatusCode::NOT_FOUND,
                    ClockUpdateError::Unavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
                };
                (
                    status,
                    Json(json!({
                        "status": "refused",
                        "detail": err.to_string(),
                    })),
                )
                    .into_response()
            }
            other => {
                let status = match &other {
                    Self::ModelUnconfigured
                    | Self::ModelProviderInit(_)
                    | Self::AuthUnavailable(_)
                    | Self::Database(_)
                    | Self::WorkingBankIo(_) => StatusCode::SERVICE_UNAVAILABLE,
                    Self::ChartLoad(_)
                    | Self::Refresh(_)
                    | Self::Dispatch(_)
                    | Self::Review(_)
                    | Self::Schedule(_)
                    | Self::Briefing(_)
                    | Self::Spruce(_)
                    | Self::CarePlan(_)
                    | Self::JobExecution(_) => StatusCode::BAD_GATEWAY,
                    Self::EventRefused(_) | Self::ClockUpdate(_) => unreachable!(),
                };
                (
                    status,
                    Json(json!({
                        "status": "failed",
                        "detail": other.to_string(),
                    })),
                )
                    .into_response()
            }
        }
    }
}

/// Summary of one synthetic patient on `GET /staging-synthetic/patients`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyntheticPatientSummary {
    /// Coda patient workspace ID (`pw-...`).
    pub patient_workspace_id: String,
    /// Native synthetic chart folder ID (`MRN...`).
    pub source_patient_id: String,
    /// Patient display name.
    pub display_name: String,
    /// ISO birth date, if present.
    pub birth_date: Option<String>,
    /// Primary MRN value, if present.
    pub mrn: Option<String>,
    /// Current saved chart observation version in PostgreSQL.
    pub chart_version: u64,
    /// Number of active chart records stored in PostgreSQL for this workspace.
    pub chart_record_count: usize,
}

/// Response body of `GET /staging-synthetic/patients`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyntheticPatientListResponse {
    /// Shared staging practice ID (`"roger-synthetic-practice"`).
    pub practice_id: String,
    /// Registered synthetic patients ordered by display name.
    pub patients: Vec<SyntheticPatientSummary>,
}

/// Aggregate counts of runtime state held for [`STAGING_PRACTICE_ID`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticeStateCounts {
    /// Registered synthetic patients.
    pub patients: usize,
    /// Saved physician conversations across the practice.
    pub conversations: usize,
    /// Saved patient reviews across all workspaces.
    pub reviews: usize,
    /// Open Today concerns across all workspaces.
    pub open_concerns: usize,
    /// Active or retired patient memory entries across all workspaces.
    pub memories: usize,
    /// Saved patient understandings across all workspaces.
    pub understandings: usize,
    /// Saved schedule appointments across the practice.
    pub appointments: usize,
    /// Saved visit preparation decision rows across the practice.
    pub visit_preparations: usize,
    /// Saved pre-visit briefings across all workspaces.
    pub briefings: usize,
}

/// Persisted simulation clock snapshot returned by `GET` and `POST /staging-synthetic/clock`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyntheticClockSnapshot {
    /// Shared staging practice ID (`"roger-synthetic-practice"`).
    pub practice_id: String,
    /// Current persisted clinical Unix second.
    pub unix_seconds: u64,
    /// Current persisted clinical instant in RFC3339 UTC.
    pub rfc3339: String,
    /// Practice-local ISO calendar date (`YYYY-MM-DD`).
    pub date_iso: String,
    /// Validated IANA time zone of the practice.
    pub time_zone: String,
    /// Baseline Unix second used on initialization and reset.
    pub baseline_unix_seconds: u64,
    /// Elapsed simulation seconds since `baseline_unix_seconds`.
    pub elapsed_seconds: u64,
}

/// Breakdown of items requiring physician attention across the simulation window.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhysicianTouchedBreakdown {
    /// Currently open patient concerns waiting for physician clearance.
    pub open_concerns: usize,
    /// Ready patient reply proposals awaiting physician review/copy.
    pub proposed_replies_waiting_for_physician: usize,
    /// Undecided medication recommendations waiting for explicit physician approval/decline.
    pub undecided_recommendations: usize,
    /// Follow-ups in `cannot_restart` requiring physician intervention.
    pub unhandled_follow_ups: usize,
    /// Spruce conversations held unmatched or needing physician linking attention.
    pub unmatched_spruce_conversations: usize,
    /// Patient source status rows currently reporting a failed check.
    pub failed_source_statuses: usize,
}

impl PhysicianTouchedBreakdown {
    /// Total count of physician-touch items across all surfaced categories.
    pub const fn total(&self) -> usize {
        self.open_concerns
            + self.proposed_replies_waiting_for_physician
            + self.undecided_recommendations
            + self.unhandled_follow_ups
            + self.unmatched_spruce_conversations
            + self.failed_source_statuses
    }
}

/// Necessary-care accounting captured alongside physician-touch rate (`L1.Measure`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NecessaryCareAccounting {
    /// Total completed patient reviews across the panel.
    pub total_reviews_completed: usize,
    /// Completed patient reviews that settled quietly with zero surfaced actions.
    pub quiet_reviews_completed: usize,
    /// Completed patient reviews that recorded clinical assessment actions.
    pub assessed_reviews_completed: usize,
    /// Total concerns raised (open, cleared, or retired).
    pub concerns_raised: usize,
    /// Concerns explicitly cleared or retired.
    pub concerns_addressed: usize,
    /// Total follow-ups created across the panel.
    pub follow_ups_created: usize,
    /// Follow-ups explicitly completed.
    pub follow_ups_completed: usize,
    /// Prepared pre-visit briefings saved across the panel.
    pub visit_briefings_ready: usize,
    /// Undecided medication recommendations held out of the active plan.
    pub undecided_recommendations_held_out_of_plan: usize,
    /// Medication recommendations explicitly approved into the active plan.
    pub approved_plan_items: usize,
    /// Medication recommendations explicitly declined by a physician.
    pub declined_recommendations: usize,
    /// Synthetic or live Spruce patient messages persisted.
    pub spruce_messages_ingested: usize,
    /// Stale or superseded reply proposals retired instead of kept ready.
    pub stale_replies_retired: usize,
    /// Duet connected-care source records persisted separately from EHR charts.
    pub duet_updates_recorded: usize,
}

/// L1 simulation measure computed directly from PostgreSQL runtime truth (`L1.Measure`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct L1SimulationMeasure {
    /// Shared staging practice ID (`"roger-synthetic-practice"`).
    pub practice_id: String,
    /// Number of registered panel patients.
    pub panel_size: usize,
    /// Baseline simulation clock Unix second.
    pub clock_baseline_unix: u64,
    /// Current persisted simulation clock Unix second.
    pub clock_current_unix: u64,
    /// Elapsed simulation seconds since baseline.
    pub elapsed_seconds: u64,
    /// Elapsed simulation weeks (`elapsed_seconds / 604800.0`).
    pub elapsed_weeks: f64,
    /// Total physician-touched items across the panel.
    pub physician_touched_total: usize,
    /// Physician-touched items per 100 patients per week (`None` before simulation time elapses or with an empty panel).
    pub physician_touched_per_100_patients_per_week: Option<f64>,
    /// Per-category breakdown of physician-touched items.
    pub physician_touched_breakdown: PhysicianTouchedBreakdown,
    /// Necessary-care accounting ensuring low touch does not hide missed care.
    pub necessary_care: NecessaryCareAccounting,
}

/// Full readback of the staging synthetic practice (`GET /staging-synthetic/status`
/// and `POST /staging-synthetic/reset`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PracticeSyntheticStatus {
    /// Shared staging practice ID (`"roger-synthetic-practice"`).
    pub practice_id: String,
    /// Current simulated health state of the synthetic chart, calendar, Spruce, and Duet adapters.
    pub source_health: SourceHealthSnapshot,
    /// Aggregate runtime counts in PostgreSQL.
    pub counts: PracticeStateCounts,
    /// Per-patient chart versions and metadata.
    pub patients: Vec<SyntheticPatientSummary>,
    /// Events accepted by the synthetic sources since startup or the last reset.
    pub accepted_events: Vec<SavedSyntheticEvent>,
    /// L1 simulation measure computed from PostgreSQL runtime tables.
    #[serde(default)]
    pub l1_measure: L1SimulationMeasure,
}

/// Request payload for `POST /staging-synthetic/source-failure`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceFailureRequest {
    /// Target synthetic source adapter (`chart` or `calendar`).
    pub source: SyntheticSourceKind,
    /// Desired health directive (`healthy` or `failing`).
    pub directive: SourceHealthDirective,
    /// Optional single patient workspace to refresh immediately when `source == chart`;
    /// when `None`, refreshes every registered patient in the staging practice.
    #[serde(default)]
    pub patient_workspace_id: Option<String>,
}

/// Response payload for `POST /staging-synthetic/source-failure`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceFailureResponse {
    /// Updated source health snapshot.
    pub source_health: SourceHealthSnapshot,
    /// Per-workspace saved chart refresh statuses when `source == chart`.
    pub chart_statuses: Vec<SavedSourceStatus>,
    /// Schedule sync outcome message when `source == calendar`.
    pub calendar_sync_error: Option<String>,
}

/// Optional parameters for `POST /staging-synthetic/rounds`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoundsRequest {
    /// Optional explicit Unix timestamp for dispatch wake evaluation.
    /// Defaults to the next due daily-rounds check second so newly arrived source
    /// records are checked immediately without crossing the weekly periodic window.
    #[serde(default)]
    pub now_unix: Option<i64>,
}

/// Summary of one executed patient review returned by `POST /staging-synthetic/rounds`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutedReviewSummary {
    /// Patient workspace reviewed.
    pub patient_workspace_id: String,
    /// Durable patient review ID.
    pub review_id: String,
    /// Trigger reason (`"new records"` or `"periodic"`).
    pub trigger_reason: String,
    /// Final review status (`"completed"`, `"failed"`, `"held"`, etc.).
    pub status: String,
    /// Observed workspace chart version when completed.
    pub completed_source_version: Option<u64>,
    /// Number of authoritative actions (`patient_concern`, `save_patient_understanding`, etc.) recorded.
    pub action_count: usize,
}

/// Response payload for `POST /staging-synthetic/rounds`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoundsExecutionReport {
    /// Unix timestamp at which rounds ran.
    pub evaluated_at_unix: i64,
    /// Raw dispatch wake summary (`checked_patients`, `failed_checks`, `started_reviews`).
    pub dispatch: WakeSummary,
    /// Patient reviews executed through the shared harness during this rounds pass.
    pub executed_reviews: Vec<ExecutedReviewSummary>,
    /// Open concerns currently visible across the staging practice after rounds settled.
    pub open_concerns: Vec<Concern>,
}

/// Optional parameters for `POST /staging-synthetic/visits/prepare`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrepareVisitsRequest {
    /// Optional explicit Unix timestamp for visit-preparation selection and briefing execution.
    /// Defaults to the staging practice clock's current Unix second.
    #[serde(default)]
    pub now_unix: Option<i64>,
}

/// Response payload for `POST /staging-synthetic/visits/prepare`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VisitPreparationExecutionReport {
    /// Unix timestamp at which visit preparation ran.
    pub evaluated_at_unix: i64,
    /// Outcome of syncing `working_bank/calendar/Appointment.ndjson` into PostgreSQL.
    pub schedule_sync: ScheduleSyncOutcome,
    /// Summary of visit selection and briefing jobs run on the shared harness.
    pub wake_summary: ScheduledWakeSummary,
    /// Saved visit preparation records across the staging practice.
    pub preparations: Vec<VisitPreparationRecord>,
    /// Saved pre-visit briefings across the staging practice.
    pub briefings: Vec<SavedVisitBriefing>,
}

/// Staging synthetic control coordinator shared by the `/staging-synthetic/*` routes.
pub struct StagingSyntheticControl {
    sources: Arc<StagingSyntheticSources>,
    db: DatabaseConnection,
    store: Store,
    auth: PostgresAuthStore,
    bus: Arc<dyn physician_bus::Bus>,
    roles: local_product::ModelRoles,
    skills_dir: Option<String>,
    model: SyntheticModelEndpoint,
    accepted_events: tokio::sync::Mutex<Vec<SavedSyntheticEvent>>,
    refresh_seq: AtomicU64,
}

/// Dependencies used to construct a [`StagingSyntheticControl`] instance.
pub struct StagingSyntheticControlDeps {
    /// Validated base and working bank directories.
    pub config: StagingSyntheticControlConfig,
    /// Shared Postgres connection used for atomic practice resets.
    pub db: DatabaseConnection,
    /// Runtime store bound to the same Postgres database.
    pub store: Store,
    /// Auth store bound to the same Postgres database.
    pub auth: PostgresAuthStore,
    /// Workspace invalidation bus notifying open browser tabs.
    pub bus: Arc<dyn physician_bus::Bus>,
    /// Model bindings for chat and background jobs.
    pub roles: local_product::ModelRoles,
    /// Optional explicit skills directory override.
    pub skills_dir: Option<String>,
    /// Model execution endpoint for background review and visit-preparation runs.
    pub model: SyntheticModelEndpoint,
}

impl StagingSyntheticControl {
    /// Initializes the working bank from the base bank, wires [`StagingSyntheticSources`]
    /// into `Store::with_source_refresh`, and loads the base synthetic charts and
    /// practice schedule if not already loaded.
    pub async fn initialize(deps: StagingSyntheticControlDeps) -> Result<Arc<Self>, String> {
        let initial_clock = crate::auth::SYNTHETIC_PRACTICE_CLOCK.now(now());
        let sources = Arc::new(StagingSyntheticSources::new(&deps.config, initial_clock));
        sources
            .ensure_working_bank_initialized()
            .map_err(|err| format!("initialize staging synthetic working bank: {err}"))?;

        let refresh_trait: Arc<dyn PatientSourceRefresh> =
            Arc::clone(&sources) as Arc<dyn PatientSourceRefresh>;
        let store = deps.store.with_source_refresh(refresh_trait);

        let existing_patients = deps
            .auth
            .practice_patients(STAGING_PRACTICE_ID)
            .await
            .map_err(|err| format!("query staging practice patients: {err}"))?;
        if existing_patients.is_empty() {
            let load_summary = crate::chart_load::load_synthetic_practice(
                sources.working_bank_dir(),
                &deps.auth,
                &store,
            )
            .await?;
            if !load_summary.failed.is_empty() {
                return Err(format!(
                    "synthetic practice chart load reported failures: {}",
                    load_summary.failed.join("; ")
                ));
            }
            if sources
                .working_bank_dir()
                .join("calendar")
                .join("Appointment.ndjson")
                .is_file()
            {
                schedule::sync_synthetic_schedule(
                    sources.as_ref(),
                    &deps.auth,
                    &store,
                    STAGING_PRACTICE_ID,
                )
                .await
                .map_err(|err| format!("initial synthetic schedule sync failed: {err}"))?;
            }
        }

        // Sync adapter clock to the persisted practice clock so service restarts
        // resume at the exact same simulated "today".
        let persisted_clock = deps
            .auth
            .practice_clock(STAGING_PRACTICE_ID)
            .await
            .map_err(|err| format!("read persisted practice clock: {err}"))?;
        sources.set_clock_unix(persisted_clock.now(now()));

        let initial_events = sources.read_accepted_events();
        Ok(Arc::new(Self {
            sources,
            db: deps.db,
            store,
            auth: deps.auth,
            bus: deps.bus,
            roles: deps.roles,
            skills_dir: deps.skills_dir,
            model: deps.model,
            accepted_events: tokio::sync::Mutex::new(initial_events),
            refresh_seq: AtomicU64::new(initial_monotonic_seed()),
        }))
    }

    /// Underlying synthetic source adapters (`PatientSource`, `PatientSourceRefresh`,
    /// and `ScheduleSource`).
    pub fn sources(&self) -> Arc<StagingSyntheticSources> {
        Arc::clone(&self.sources)
    }

    /// Runtime [`Store`] wired to [`Self::sources`].
    pub fn store(&self) -> &Store {
        &self.store
    }

    /// Lists every synthetic patient registered in [`STAGING_PRACTICE_ID`].
    pub async fn list_patients(
        &self,
    ) -> Result<SyntheticPatientListResponse, SyntheticControlError> {
        let patients = self.collect_patient_summaries().await?;
        Ok(SyntheticPatientListResponse {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            patients,
        })
    }

    /// Reads the persisted simulation clock for [`STAGING_PRACTICE_ID`].
    pub async fn clock(&self) -> Result<SyntheticClockSnapshot, SyntheticControlError> {
        let practice = self
            .auth
            .practice(STAGING_PRACTICE_ID)
            .await
            .map_err(SyntheticControlError::AuthUnavailable)?;
        let clock = practice
            .as_ref()
            .map(|row| row.clock)
            .unwrap_or(crate::auth::SYNTHETIC_PRACTICE_CLOCK);
        let time_zone = practice.map(|row| row.time_zone).unwrap_or_default();
        let baseline_unix_seconds = match crate::auth::SYNTHETIC_PRACTICE_CLOCK {
            PracticeClock::Fixed(ts) => ts,
            PracticeClock::RealTime => now(),
        };
        let unix_seconds = clock.now(now());
        let rfc3339 = chrono::DateTime::from_timestamp(unix_seconds as i64, 0)
            .map(|ts| ts.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
            .unwrap_or_else(|| unix_seconds.to_string());
        let date_iso = time_zone.calendar_date_iso(unix_seconds);
        Ok(SyntheticClockSnapshot {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            unix_seconds,
            rfc3339,
            date_iso,
            time_zone: time_zone.as_str().to_owned(),
            baseline_unix_seconds,
            elapsed_seconds: unix_seconds.saturating_sub(baseline_unix_seconds),
        })
    }

    /// Advances or resets the persisted simulation clock for [`STAGING_PRACTICE_ID`].
    pub async fn update_clock(
        &self,
        update: SyntheticClockUpdate,
    ) -> Result<SyntheticClockSnapshot, SyntheticControlError> {
        let updated = self
            .auth
            .set_synthetic_practice_clock(STAGING_PRACTICE_ID, update)
            .await
            .map_err(SyntheticControlError::ClockUpdate)?;
        let unix_seconds = updated.now(now());
        self.sources.set_clock_unix(unix_seconds);

        let workspaces = self
            .auth
            .practice_patients(STAGING_PRACTICE_ID)
            .await
            .map_err(SyntheticControlError::AuthUnavailable)?;
        for ws in workspaces {
            let _ = self
                .bus
                .publish(physician_bus::Event::WorkspaceChanged { workspace_id: ws })
                .await;
        }
        self.clock().await
    }

    /// Computes the L1 simulation measure directly from PostgreSQL runtime tables.
    pub async fn measure(&self) -> Result<L1SimulationMeasure, SyntheticControlError> {
        let clock = self.clock().await?;
        let workspaces = self
            .auth
            .practice_patients(STAGING_PRACTICE_ID)
            .await
            .map_err(SyntheticControlError::AuthUnavailable)?;
        let panel_size = workspaces.len();

        let mut total_reviews_completed = 0_usize;
        let mut quiet_reviews_completed = 0_usize;
        let mut assessed_reviews_completed = 0_usize;
        let mut visit_briefings_ready = 0_usize;
        let mut duet_updates_recorded = 0_usize;

        for ws in &workspaces {
            let reviews = self
                .store
                .patient_reviews(STAGING_PRACTICE_ID, ws, &self.auth)
                .await
                .map_err(SyntheticControlError::Review)?;
            for rev in reviews {
                if rev.state.status_str() == "completed" {
                    total_reviews_completed += 1;
                    if rev.actions.is_empty() {
                        quiet_reviews_completed += 1;
                    } else {
                        assessed_reviews_completed += 1;
                    }
                }
            }
            visit_briefings_ready += self
                .store
                .visit_briefings_for_workspace(STAGING_PRACTICE_ID, ws, &self.auth)
                .await
                .map_err(SyntheticControlError::Briefing)?
                .len();
            let observation = self
                .store
                .sources(ws)
                .await
                .map_err(SyntheticControlError::Database)?;
            duet_updates_recorded += observation
                .sources
                .iter()
                .filter(|rev| rev.source_id == DUET_CONNECTED_CARE_SOURCE_ID)
                .count();
        }

        let recs = self
            .store
            .practice_care_plan_recommendations(STAGING_PRACTICE_ID)
            .await
            .map_err(SyntheticControlError::CarePlan)?;
        let mut undecided_recommendations = 0_usize;
        let mut approved_plan_items = 0_usize;
        let mut declined_recommendations = 0_usize;
        for rec in recs {
            match rec.standing {
                RecommendationStanding::Undecided => undecided_recommendations += 1,
                RecommendationStanding::ApprovedIntoPlan { .. } => approved_plan_items += 1,
                RecommendationStanding::Declined { .. } => declined_recommendations += 1,
            }
        }

        let db_counts = query_l1_measure_counts(&self.db, STAGING_PRACTICE_ID)
            .await
            .map_err(SyntheticControlError::Database)?;

        let breakdown = PhysicianTouchedBreakdown {
            open_concerns: db_counts.open_concerns,
            proposed_replies_waiting_for_physician: db_counts.ready_replies,
            undecided_recommendations,
            unhandled_follow_ups: db_counts.unhandled_follow_ups,
            unmatched_spruce_conversations: db_counts.unmatched_spruce_conversations,
            failed_source_statuses: db_counts.failed_source_statuses,
        };
        let physician_touched_total = breakdown.total();
        let elapsed_weeks = (clock.elapsed_seconds as f64) / (7.0 * 86_400.0);
        let physician_touched_per_100_patients_per_week = if panel_size > 0
            && clock.elapsed_seconds > 0
        {
            Some((physician_touched_total as f64) * 100.0 / ((panel_size as f64) * elapsed_weeks))
        } else {
            None
        };

        Ok(L1SimulationMeasure {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            panel_size,
            clock_baseline_unix: clock.baseline_unix_seconds,
            clock_current_unix: clock.unix_seconds,
            elapsed_seconds: clock.elapsed_seconds,
            elapsed_weeks,
            physician_touched_total,
            physician_touched_per_100_patients_per_week,
            physician_touched_breakdown: breakdown,
            necessary_care: NecessaryCareAccounting {
                total_reviews_completed,
                quiet_reviews_completed,
                assessed_reviews_completed,
                concerns_raised: db_counts.concerns_raised,
                concerns_addressed: db_counts.concerns_addressed,
                follow_ups_created: db_counts.follow_ups_created,
                follow_ups_completed: db_counts.follow_ups_completed,
                visit_briefings_ready,
                undecided_recommendations_held_out_of_plan: undecided_recommendations,
                approved_plan_items,
                declined_recommendations,
                spruce_messages_ingested: db_counts.spruce_messages_ingested,
                stale_replies_retired: db_counts.stale_replies_retired,
                duet_updates_recorded,
            },
        })
    }

    /// Reads full synthetic practice status and counts from PostgreSQL.
    pub async fn status(&self) -> Result<PracticeSyntheticStatus, SyntheticControlError> {
        let patients = self.collect_patient_summaries().await?;
        let workspaces: Vec<String> = patients
            .iter()
            .map(|p| p.patient_workspace_id.clone())
            .collect();

        let conversations = count_practice_conversations(&self.db, STAGING_PRACTICE_ID)
            .await
            .map_err(SyntheticControlError::Database)?;

        let mut reviews = 0_usize;
        let mut memories = 0_usize;
        let mut understandings = 0_usize;
        let mut briefings = 0_usize;
        for ws in &workspaces {
            reviews += self
                .store
                .patient_reviews(STAGING_PRACTICE_ID, ws, &self.auth)
                .await
                .map_err(SyntheticControlError::Review)?
                .len();
            let mem_key = PatientKey::for_rounds(STAGING_PRACTICE_ID, ws).map_err(|err| {
                SyntheticControlError::JobExecution(format!("patient memory key: {err}"))
            })?;
            let mem_listing = self.store.list_memory(&mem_key).await.map_err(|err| {
                SyntheticControlError::JobExecution(format!("read patient memories: {err}"))
            })?;
            memories += usize::from(mem_listing.summary.is_some())
                + usize::from(mem_listing.direction.is_some())
                + usize::from(mem_listing.person.is_some())
                + mem_listing.pages.len()
                + mem_listing.log_days.len();
            if self
                .store
                .patient_understanding_for_practice(STAGING_PRACTICE_ID, ws, &self.auth)
                .await
                .map_err(|err| {
                    SyntheticControlError::JobExecution(format!("read understanding: {err}"))
                })?
                .is_some()
            {
                understandings += 1;
            }
            briefings += self
                .store
                .visit_briefings_for_workspace(STAGING_PRACTICE_ID, ws, &self.auth)
                .await
                .map_err(SyntheticControlError::Briefing)?
                .len();
        }

        let open_concerns = self.collect_all_open_concerns(&workspaces).await?.len();
        let schedule_view = self
            .store
            .practice_schedule(STAGING_PRACTICE_ID)
            .await
            .map_err(SyntheticControlError::Schedule)?;
        let preparations = self
            .store
            .visit_preparations_for_practice(STAGING_PRACTICE_ID, &self.auth)
            .await
            .map_err(SyntheticControlError::Briefing)?;
        let accepted_events = {
            let guard = self.accepted_events.lock().await;
            let persisted = self.sources.read_accepted_events();
            if persisted.is_empty() {
                guard.clone()
            } else {
                persisted
            }
        };

        let l1_measure = self.measure().await?;
        Ok(PracticeSyntheticStatus {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            source_health: self.sources.health_snapshot(),
            counts: PracticeStateCounts {
                patients: patients.len(),
                conversations,
                reviews,
                open_concerns,
                memories,
                understandings,
                appointments: schedule_view.appointments.len(),
                visit_preparations: preparations.len(),
                briefings,
            },
            patients,
            accepted_events,
            l1_measure,
        })
    }

    /// Validates and records a synthetic event: chart and calendar events write
    /// to `working_bank_dir` without touching PostgreSQL; Spruce messages, Duet
    /// connected-care updates, and undecided medication recommendations enter
    /// through their typed PostgreSQL runtime owners.
    pub async fn send_event(
        &self,
        spec: SyntheticEventSpec,
    ) -> Result<SavedSyntheticEvent, SyntheticControlError> {
        let clock = self
            .auth
            .practice_clock(STAGING_PRACTICE_ID)
            .await
            .map_err(SyntheticControlError::AuthUnavailable)?;
        let accepted_at = clock.now(now());
        let saved = self
            .sources
            .accept_event(spec, accepted_at)
            .await
            .map_err(SyntheticControlError::EventRefused)?;

        match &saved.event {
            SyntheticEventSpec::SprucePatientMessage {
                conversation_id,
                message_id,
                sender_name,
                sender_contact,
                text,
                sent_at,
                ..
            } => {
                let workspace_id = saved
                    .resolved_workspace_id
                    .as_deref()
                    .unwrap_or_default()
                    .to_owned();
                self.deliver_spruce_message(
                    &workspace_id,
                    &saved.resolved_source_patient_id,
                    conversation_id,
                    message_id,
                    sender_name,
                    sender_contact,
                    text,
                    sent_at,
                    accepted_at as i64,
                )
                .await?;
            }
            SyntheticEventSpec::DuetPatientUpdate {
                record_id,
                category,
                title,
                text,
                observed_at,
                ..
            } => {
                let workspace_id = saved
                    .resolved_workspace_id
                    .as_deref()
                    .unwrap_or_default()
                    .to_owned();
                let candidate_id = record_id.clone().unwrap_or_else(|| saved.event_id.clone());
                self.deliver_duet_update(
                    &workspace_id,
                    &candidate_id,
                    category,
                    title,
                    text,
                    observed_at,
                )
                .await?;
            }
            SyntheticEventSpec::MedicationRecommendation {
                recommendation_id,
                medication,
                rationale,
                detail,
                source_label,
                ..
            } => {
                let workspace_id = saved
                    .resolved_workspace_id
                    .as_deref()
                    .unwrap_or_default()
                    .to_owned();
                let rec_id = recommendation_id
                    .clone()
                    .unwrap_or_else(|| saved.event_id.clone());
                self.store
                    .propose_medication_recommendation(
                        STAGING_PRACTICE_ID,
                        &workspace_id,
                        NewMedicationRecommendation {
                            recommendation_id: rec_id,
                            medication: medication.clone(),
                            rationale: rationale.clone(),
                            detail: detail.clone(),
                            source_label: source_label
                                .clone()
                                .unwrap_or_else(|| "Patient review".to_owned()),
                            proposed_at: accepted_at as i64,
                        },
                    )
                    .await
                    .map_err(SyntheticControlError::CarePlan)?;
                let _ = self
                    .bus
                    .publish(physician_bus::Event::WorkspaceChanged { workspace_id })
                    .await;
            }
            SyntheticEventSpec::FreeFormChartRecord { .. }
            | SyntheticEventSpec::RawFhirChartResource { .. }
            | SyntheticEventSpec::VisitChange { .. }
            | SyntheticEventSpec::RawFhirAppointment { .. } => {}
        }

        let mut guard = self.accepted_events.lock().await;
        self.sources
            .append_accepted_event(&saved)
            .map_err(SyntheticControlError::WorkingBankIo)?;
        guard.push(saved.clone());
        Ok(saved)
    }

    #[allow(clippy::too_many_arguments)]
    async fn deliver_spruce_message(
        &self,
        workspace_id: &str,
        source_patient_id: &str,
        conversation_id: &str,
        message_id: &str,
        sender_name: &str,
        sender_contact: &str,
        text: &str,
        sent_at: &str,
        now_ts: i64,
    ) -> Result<(), SyntheticControlError> {
        let (standing, _existing_lease) = self
            .store
            .spruce_channel_standing(STAGING_PRACTICE_ID)
            .await
            .map_err(SyntheticControlError::Spruce)?;
        let current_place = match standing {
            ChannelStanding::Disconnected { .. } => {
                let panel = crate::auth::staging_panel(&self.auth)
                    .await
                    .map_err(SyntheticControlError::AuthUnavailable)?;
                let actor = panel.into_iter().next().ok_or_else(|| {
                    SyntheticControlError::JobExecution(
                        "no active staging physician available to connect synthetic Spruce channel"
                            .to_owned(),
                    )
                })?;
                self.store
                    .connect_spruce_channel(&self.auth, &actor, 0, None, now_ts)
                    .await
                    .map_err(SyntheticControlError::Spruce)?;
                0_u64
            }
            ChannelStanding::Connected { place, .. } => place,
        };

        let sender_address =
            SenderAddress::parse(sender_contact).map_err(SyntheticControlError::Spruce)?;
        let contact_id = format!("spruce-contact-{source_patient_id}");
        self.store
            .register_spruce_ehr_patient_link(
                STAGING_PRACTICE_ID,
                synthetic_chart::SOURCE_ID,
                source_patient_id,
                workspace_id,
                now_ts,
            )
            .await
            .map_err(SyntheticControlError::Spruce)?;
        self.store
            .save_spruce_contact_record(
                &SpruceContactRecord {
                    practice: STAGING_PRACTICE_ID.to_owned(),
                    contact_id: contact_id.clone(),
                    spruce_patients: vec![SpruceContactPatient {
                        spruce_patient_id: format!("spruce-patient-{source_patient_id}"),
                        ehr_links: vec![SpruceEhrLink {
                            source_id: synthetic_chart::SOURCE_ID.to_owned(),
                            source_patient_id: source_patient_id.to_owned(),
                        }],
                    }],
                },
                now_ts,
            )
            .await
            .map_err(SyntheticControlError::Spruce)?;

        let holder_id = format!(
            "staging-synthetic-spruce-{}",
            self.refresh_seq.fetch_add(1, Ordering::SeqCst) + 1
        );
        let lease = self
            .store
            .acquire_spruce_reader_lease(STAGING_PRACTICE_ID, &holder_id, 60, now_ts)
            .await
            .map_err(SyntheticControlError::Spruce)?;
        let next_position = current_place.saturating_add(1);
        let content_version = local_product::sha256_hex(
            format!("{conversation_id}:{message_id}:{text}:{sent_at}").as_bytes(),
        );
        let item = IncomingSpruceItem::Readable(ReadableSpruceItem {
            message_id: message_id.to_owned(),
            conversation_id: conversation_id.to_owned(),
            channel_position: next_position,
            content_version,
            sender_address: Some(sender_address.clone()),
            author_name: Some(sender_name.to_owned()),
            body_text: text.to_owned(),
            attachments: Vec::new(),
            app_url: None,
            created_at_spruce: Some(sent_at.to_owned()),
            outside_participants: vec![OutsideParticipant {
                display_name: Some(sender_name.to_owned()),
                endpoint_channel: Some("sms".to_owned()),
                endpoint_address: Some(sender_address),
                saved_contact_id: Some(contact_id),
            }],
        });
        let ingest_res = self
            .store
            .ingest_spruce_item(STAGING_PRACTICE_ID, &lease.lease_token, &item, now_ts)
            .await;
        if ingest_res.is_ok() {
            let _ = self
                .store
                .advance_spruce_channel_place(STAGING_PRACTICE_ID, &lease.lease_token, None, now_ts)
                .await;
        }
        let _ = self
            .store
            .release_spruce_reader_lease(STAGING_PRACTICE_ID, &lease.lease_token, now_ts)
            .await;
        ingest_res.map_err(SyntheticControlError::Spruce)?;

        let _ = self
            .bus
            .publish(physician_bus::Event::WorkspaceChanged {
                workspace_id: workspace_id.to_owned(),
            })
            .await;
        Ok(())
    }

    async fn deliver_duet_update(
        &self,
        workspace_id: &str,
        candidate_id: &str,
        category: &str,
        title: &str,
        text: &str,
        observed_at: &str,
    ) -> Result<(), SyntheticControlError> {
        let content = json!({
            "source": DUET_CONNECTED_CARE_SOURCE_ID,
            "category": category,
            "title": title,
            "text": text,
            "observed_at": observed_at,
        })
        .to_string();
        let sha256 = local_product::sha256_hex(content.as_bytes());
        let revision = SourceRevision {
            workspace: workspace_id.to_owned(),
            source_id: DUET_CONNECTED_CARE_SOURCE_ID.to_owned(),
            candidate_id: format!("duet:{candidate_id}"),
            revision: format!("duet-rev-{}", &sha256[..12]),
            record_type: format!("Duet/{category}"),
            dates: vec![RecordDate {
                name: "observed".to_owned(),
                value: observed_at.to_owned(),
            }],
            content,
            sha256,
            readable_parts: vec![ReadablePart {
                label: title.to_owned(),
                text: text.to_owned(),
            }],
            provenance: json!({
                "source_id": DUET_CONNECTED_CARE_SOURCE_ID,
                "source_label": "Duet connected care",
                "record_id": candidate_id,
                "category": category,
                "observed_at": observed_at,
            }),
            envelope: None,
        };
        self.store
            .record_source(&revision)
            .await
            .map_err(SyntheticControlError::Database)?;
        let _ = self
            .bus
            .publish(physician_bus::Event::WorkspaceChanged {
                workspace_id: workspace_id.to_owned(),
            })
            .await;
        Ok(())
    }

    /// Injects or clears a source failure on the synthetic chart or calendar adapter
    /// and runs a source refresh / schedule sync so the product observes the outcome
    /// through the real [`PatientSourceRefresh`] or [`ScheduleSource`] boundary.
    pub async fn set_source_failure(
        &self,
        request: SourceFailureRequest,
    ) -> Result<SourceFailureResponse, SyntheticControlError> {
        self.sources
            .set_source_health(request.source, request.directive);
        let clock = self
            .auth
            .practice_clock(STAGING_PRACTICE_ID)
            .await
            .map_err(SyntheticControlError::AuthUnavailable)?;
        let now_unix = clock.now(now());
        self.sources.set_clock_unix(now_unix);

        let mut chart_statuses = Vec::new();
        let mut calendar_sync_error = None;

        match request.source {
            SyntheticSourceKind::Chart => {
                let targets = match request.patient_workspace_id {
                    Some(ws) => vec![ws],
                    None => self
                        .auth
                        .practice_patients(STAGING_PRACTICE_ID)
                        .await
                        .map_err(SyntheticControlError::AuthUnavailable)?,
                };
                for ws in targets {
                    let seq = self.refresh_seq.fetch_add(1, Ordering::SeqCst) + 1;
                    let key = RefreshCommandKey::new(format!("source-health-{now_unix}-{seq}"))
                        .map_err(|err| {
                            SyntheticControlError::JobExecution(format!(
                                "invalid refresh key: {err}"
                            ))
                        })?;
                    let refresh_req = RefreshRequest::new(key, RefreshTarget::AllSources);
                    let scope = PracticeScope {
                        practice: STAGING_PRACTICE_ID.to_owned(),
                        workspace: ws.clone(),
                        job_id: format!("source-health-{seq}"),
                    };
                    let report: RefreshReport = self
                        .store
                        .refresh_patient_sources_for_practice(
                            &scope,
                            &self.auth,
                            &refresh_req,
                            self.sources.as_ref(),
                        )
                        .await
                        .map_err(SyntheticControlError::Refresh)?;
                    chart_statuses.extend(report.sources);
                    let _ = self
                        .bus
                        .publish(physician_bus::Event::WorkspaceChanged { workspace_id: ws })
                        .await;
                }
            }
            SyntheticSourceKind::Calendar => {
                if let Err(err) = schedule::sync_synthetic_schedule(
                    self.sources.as_ref(),
                    &self.auth,
                    &self.store,
                    STAGING_PRACTICE_ID,
                )
                .await
                {
                    calendar_sync_error = Some(err.to_string());
                }
            }
            SyntheticSourceKind::Spruce | SyntheticSourceKind::Duet => {}
        }

        Ok(SourceFailureResponse {
            source_health: self.sources.health_snapshot(),
            chart_statuses,
            calendar_sync_error,
        })
    }

    /// Runs daily rounds for [`STAGING_PRACTICE_ID`]: refreshes due patient sources
    /// via [`PatientSourceRefresh`], dispatches changed/periodic reviews via
    /// [`Store::wake_practice_dispatch`], and runs admitted review jobs to settlement
    /// on the shared harness (`PracticeJobHost` + `ResponsesProvider`).
    pub async fn run_rounds(
        &self,
        request: RoundsRequest,
    ) -> Result<RoundsExecutionReport, SyntheticControlError> {
        let evaluated_at_unix = match request.now_unix {
            Some(explicit) => explicit,
            None => {
                let clock = self
                    .auth
                    .practice_clock(STAGING_PRACTICE_ID)
                    .await
                    .map_err(SyntheticControlError::AuthUnavailable)?;
                clock.now(now()) as i64
            }
        };
        if let Ok(unix_u64) = u64::try_from(evaluated_at_unix) {
            self.sources.set_clock_unix(unix_u64);
        }
        let skills = self
            .store
            .resolve_worker_skills(self.skills_dir.as_deref())
            .await
            .map_err(SyntheticControlError::Briefing)?;

        let checker = RefreshingSourceChecker {
            store: &self.store,
            auth: &self.auth,
            sources: self.sources.as_ref(),
            command_seq: &self.refresh_seq,
        };
        let receiver = PatientReviewReceiver::new(&self.store, &self.roles, &skills, &self.auth);

        let dispatch = self
            .store
            .wake_practice_dispatch(
                &self.auth,
                STAGING_PRACTICE_ID,
                evaluated_at_unix,
                &checker,
                &receiver,
            )
            .await
            .map_err(SyntheticControlError::Dispatch)?;

        let mut executed_reviews = Vec::new();
        if !dispatch.started_reviews.is_empty() {
            let mut provider = self.model.provider(&self.store).await?;
            for review_req in &dispatch.started_reviews {
                let queued_reviews = self
                    .store
                    .patient_reviews(
                        STAGING_PRACTICE_ID,
                        &review_req.patient_workspace_id,
                        &self.auth,
                    )
                    .await
                    .map_err(SyntheticControlError::Review)?;
                let Some(queued) = queued_reviews
                    .iter()
                    .find(|rev| rev.command_key == review_req.command_key)
                else {
                    continue;
                };
                let scope = queued.practice_scope();
                if let Some(claim) = self
                    .store
                    .claim_practice_job(&scope, 60, &self.auth)
                    .await
                    .map_err(|err| SyntheticControlError::JobExecution(err.to_string()))?
                {
                    let cancel = Cancellation::default();
                    let mut host = PracticeJobHost {
                        store: self.store.clone(),
                        claim: claim.clone(),
                        authority: &self.auth,
                    };
                    if let Err(err) = host.run(&mut provider, RunLimits::default(), &cancel).await {
                        let reason = engine::AttemptFailure::of(&err, &cancel);
                        let _ = self
                            .store
                            .settle_practice_job_attempt(
                                &claim,
                                reason,
                                local_product::runtime::delivery::Backoff::IMMEDIATE,
                            )
                            .await;
                        return Err(SyntheticControlError::JobExecution(err.to_string()));
                    }
                }
                let reviews = self
                    .store
                    .patient_reviews(
                        STAGING_PRACTICE_ID,
                        &review_req.patient_workspace_id,
                        &self.auth,
                    )
                    .await
                    .map_err(SyntheticControlError::Review)?;
                if let Some(saved) = reviews
                    .iter()
                    .find(|rev| rev.command_key == review_req.command_key)
                {
                    executed_reviews
                        .push(summarize_saved_review(saved, review_req.reason.as_str()));
                }
                let _ = self
                    .bus
                    .publish(physician_bus::Event::WorkspaceChanged {
                        workspace_id: review_req.patient_workspace_id.clone(),
                    })
                    .await;
            }
        }
        for failed in &dispatch.failed_checks {
            let _ = self
                .bus
                .publish(physician_bus::Event::WorkspaceChanged {
                    workspace_id: failed.patient_workspace_id.clone(),
                })
                .await;
        }

        let workspaces = self
            .auth
            .practice_patients(STAGING_PRACTICE_ID)
            .await
            .map_err(SyntheticControlError::AuthUnavailable)?;
        let open_concerns = self.collect_all_open_concerns(&workspaces).await?;

        Ok(RoundsExecutionReport {
            evaluated_at_unix,
            dispatch,
            executed_reviews,
            open_concerns,
        })
    }

    /// Syncs the synthetic calendar via [`ScheduleSource`] and runs pre-visit
    /// selection + briefing preparation via [`Store::wake_visit_preparation`] on the
    /// shared harness.
    pub async fn prepare_visits(
        &self,
        request: PrepareVisitsRequest,
    ) -> Result<VisitPreparationExecutionReport, SyntheticControlError> {
        let clock = self
            .auth
            .practice_clock(STAGING_PRACTICE_ID)
            .await
            .map_err(SyntheticControlError::AuthUnavailable)?;
        let evaluated_at_unix = request.now_unix.unwrap_or_else(|| clock.now(now()) as i64);
        if let Ok(unix_u64) = u64::try_from(evaluated_at_unix) {
            self.sources.set_clock_unix(unix_u64);
        }

        let schedule_sync = schedule::sync_synthetic_schedule(
            self.sources.as_ref(),
            &self.auth,
            &self.store,
            STAGING_PRACTICE_ID,
        )
        .await
        .map_err(SyntheticControlError::Schedule)?;

        let skills = self
            .store
            .resolve_worker_skills(self.skills_dir.as_deref())
            .await
            .map_err(SyntheticControlError::Briefing)?;
        let mut provider = self.model.provider(&self.store).await?;

        let wake_summary = self
            .store
            .wake_visit_preparation(
                &self.auth,
                STAGING_PRACTICE_ID,
                &self.roles,
                &skills,
                &mut provider,
                evaluated_at_unix,
            )
            .await
            .map_err(SyntheticControlError::Briefing)?;

        let preparations = self
            .store
            .visit_preparations_for_practice(STAGING_PRACTICE_ID, &self.auth)
            .await
            .map_err(SyntheticControlError::Briefing)?;

        let workspaces = self
            .auth
            .practice_patients(STAGING_PRACTICE_ID)
            .await
            .map_err(SyntheticControlError::AuthUnavailable)?;
        let mut briefings = Vec::new();
        for ws in &workspaces {
            let mut ws_briefings = self
                .store
                .visit_briefings_for_workspace(STAGING_PRACTICE_ID, ws, &self.auth)
                .await
                .map_err(SyntheticControlError::Briefing)?;
            briefings.append(&mut ws_briefings);
            let _ = self
                .bus
                .publish(physician_bus::Event::WorkspaceChanged {
                    workspace_id: ws.clone(),
                })
                .await;
        }

        Ok(VisitPreparationExecutionReport {
            evaluated_at_unix,
            schedule_sync,
            wake_summary,
            preparations,
            briefings,
        })
    }

    /// Restores `working_bank_dir` from `base_bank_dir`, purges all runtime state for
    /// [`STAGING_PRACTICE_ID`] (conversations, reviews, concerns, memories,
    /// understandings, briefings, dispatches, source versions, schedule rows, and
    /// panel overrides), reloads the base charts and base calendar, restores default
    /// staging panel memberships, and returns the verified readback.
    pub async fn reset(&self) -> Result<PracticeSyntheticStatus, SyntheticControlError> {
        self.reset_with_auth(None).await
    }

    async fn reset_with_auth(
        &self,
        auth_override: Option<&dyn AuthStore>,
    ) -> Result<PracticeSyntheticStatus, SyntheticControlError> {
        let auth_target: &dyn AuthStore = auth_override.unwrap_or(&self.auth);
        self.sources
            .restore_working_bank_from_base()
            .map_err(SyntheticControlError::WorkingBankIo)?;
        self.accepted_events.lock().await.clear();

        let mut workspaces = auth_target
            .practice_patients(STAGING_PRACTICE_ID)
            .await
            .map_err(SyntheticControlError::AuthUnavailable)?;
        let base_charts = SyntheticCharts::open(self.sources.working_bank_dir());
        let seeded_ids = base_charts
            .patient_ids()
            .await
            .map_err(|err| SyntheticControlError::ChartLoad(err.to_string()))?;
        let seeded_workspaces: Vec<String> = seeded_ids
            .iter()
            .map(|id| workspace_id(base_charts.source_id(), id))
            .collect();
        for ws in &seeded_workspaces {
            if !workspaces.contains(ws) {
                workspaces.push(ws.clone());
            }
        }

        purge_practice_runtime_state(&self.db, STAGING_PRACTICE_ID, &workspaces)
            .await
            .map_err(SyntheticControlError::Database)?;

        let load_summary = crate::chart_load::load_synthetic_practice(
            self.sources.working_bank_dir(),
            auth_target,
            &self.store,
        )
        .await
        .map_err(SyntheticControlError::ChartLoad)?;
        if !load_summary.failed.is_empty() {
            return Err(SyntheticControlError::ChartLoad(
                load_summary.failed.join("; "),
            ));
        }

        // Restore the persisted practice simulation clock back to the synthetic baseline.
        let baseline_unix = crate::auth::SYNTHETIC_PRACTICE_CLOCK.now(now());
        auth_target
            .set_synthetic_practice_clock(
                STAGING_PRACTICE_ID,
                SyntheticClockUpdate::ResetToUnix {
                    unix_seconds: baseline_unix,
                },
            )
            .await
            .map_err(SyntheticControlError::ClockUpdate)?;
        self.sources.set_clock_unix(baseline_unix);

        let panel_identities = crate::auth::staging_panel(auth_target)
            .await
            .map_err(SyntheticControlError::AuthUnavailable)?;
        for identity in &panel_identities {
            for seeded_ws in &seeded_workspaces {
                auth_target
                    .set_membership(identity, seeded_ws, physician_auth::Membership::Allowed)
                    .await
                    .map_err(SyntheticControlError::AuthUnavailable)?;
            }
        }

        if self
            .sources
            .working_bank_dir()
            .join("calendar")
            .join("Appointment.ndjson")
            .is_file()
        {
            schedule::sync_synthetic_schedule(
                self.sources.as_ref(),
                auth_target,
                &self.store,
                STAGING_PRACTICE_ID,
            )
            .await
            .map_err(SyntheticControlError::Schedule)?;
        }

        for ws in &workspaces {
            let _ = self
                .bus
                .publish(physician_bus::Event::WorkspaceChanged {
                    workspace_id: ws.clone(),
                })
                .await;
        }

        self.status().await
    }

    async fn collect_patient_summaries(
        &self,
    ) -> Result<Vec<SyntheticPatientSummary>, SyntheticControlError> {
        let workspaces = self
            .auth
            .practice_patients(STAGING_PRACTICE_ID)
            .await
            .map_err(SyntheticControlError::AuthUnavailable)?;
        let pulls = self
            .store
            .source_pulls(&workspaces)
            .await
            .map_err(SyntheticControlError::Database)?;
        let mut summaries = Vec::new();
        for ws in workspaces {
            let Some(first_pull) = pulls.get(&ws).and_then(|list| list.first()) else {
                continue;
            };
            let observation = self
                .store
                .sources(&ws)
                .await
                .map_err(SyntheticControlError::Database)?;
            let mrn = first_pull
                .patient
                .medical_record_numbers
                .first()
                .map(|m| m.value.clone());
            summaries.push(SyntheticPatientSummary {
                patient_workspace_id: ws,
                source_patient_id: first_pull.patient.source_patient_id.clone(),
                display_name: first_pull.patient.display_name.clone(),
                birth_date: first_pull.patient.birth_date.clone(),
                mrn,
                chart_version: observation.version,
                chart_record_count: observation.sources.len(),
            });
        }
        summaries.sort_by(|left, right| {
            left.display_name
                .to_lowercase()
                .cmp(&right.display_name.to_lowercase())
                .then_with(|| left.patient_workspace_id.cmp(&right.patient_workspace_id))
        });
        Ok(summaries)
    }

    async fn collect_all_open_concerns(
        &self,
        workspaces: &[String],
    ) -> Result<Vec<Concern>, SyntheticControlError> {
        let mut authority = StagingPracticeReadAuthority { auth: &self.auth };
        let dummy_session = [0_u8; 32];
        let mut concerns = Vec::new();
        for ws in workspaces {
            let access = WorkspaceAccess {
                practice: STAGING_PRACTICE_ID,
                physician: "staging-physician",
                workspace: ws,
                session_hash: &dummy_session,
            };
            let mut open = self
                .store
                .open_concerns(&access, &mut authority)
                .await
                .map_err(|err| {
                    SyntheticControlError::JobExecution(format!("read open concerns: {err}"))
                })?;
            concerns.append(&mut open);
        }
        Ok(concerns)
    }
}

struct StagingPracticeReadAuthority<'a> {
    auth: &'a PostgresAuthStore,
}

#[async_trait]
impl Authority for StagingPracticeReadAuthority<'_> {
    async fn check(
        &mut self,
        _tx: &PublicationTransaction,
        scope: &Scope,
        _session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        let owner = self
            .auth
            .patient_practice(&scope.workspace)
            .await
            .map_err(|err| err.to_string())?;
        if owner.as_deref() == Some(scope.practice.as_str()) {
            Ok(())
        } else {
            Err("workspace does not belong to practice".to_owned())
        }
    }

    async fn lock_publication(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        self.check(tx, scope, session_hash).await
    }
}

/// Builds the Axum router mounting `/staging-synthetic/*` routes.
pub fn router(control: Arc<StagingSyntheticControl>) -> Router {
    Router::new()
        .route("/staging-synthetic/patients", get(list_patients_route))
        .route("/staging-synthetic/status", get(status_route))
        .route(
            "/staging-synthetic/clock",
            get(get_clock_route).post(update_clock_route),
        )
        .route("/staging-synthetic/measure", get(measure_route))
        .route("/staging-synthetic/events", post(send_event_route))
        .route(
            "/staging-synthetic/source-failure",
            post(source_failure_route),
        )
        .route("/staging-synthetic/rounds", post(rounds_route))
        .route(
            "/staging-synthetic/visits/prepare",
            post(prepare_visits_route),
        )
        .route("/staging-synthetic/reset", post(reset_route))
        .with_state(control)
}

fn extract_auth_inputs(request: &Request<Body>) -> (Option<String>, Option<Arc<dyn AuthStore>>) {
    let token = bearer(request);
    let store = request.extensions().get::<Arc<dyn AuthStore>>().cloned();
    (token, store)
}

async fn authorize_staging_request(
    control: &StagingSyntheticControl,
    token: Option<String>,
    auth_ext: Option<Arc<dyn AuthStore>>,
) -> Option<Response> {
    let Some(token) = token else {
        return Some(
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "detail": "Sign in to operate staging synthetic control.",
                    "reason": "no_session",
                })),
            )
                .into_response(),
        );
    };
    let resolved = match auth_ext {
        Some(store) => store.identity(&token, now()).await,
        None => AuthStore::identity(&control.auth, &token, now()).await,
    };
    let identity = match resolved {
        Ok(identity) => identity,
        Err(SessionError::Invalid) => {
            return Some(
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "detail": "Sign in to operate staging synthetic control.",
                        "reason": "no_session",
                    })),
                )
                    .into_response(),
            );
        }
        Err(SessionError::Inactive) => {
            return Some(
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "detail": "Sign in to operate staging synthetic control.",
                        "reason": "access_removed",
                    })),
                )
                    .into_response(),
            );
        }
    };
    if identity.practice_id != STAGING_PRACTICE_ID {
        return Some(
            (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "status": "forbidden",
                    "detail": format!(
                        "staging synthetic control operates exclusively in `{STAGING_PRACTICE_ID}`"
                    ),
                })),
            )
                .into_response(),
        );
    }
    None
}

async fn list_patients_route(
    State(control): State<Arc<StagingSyntheticControl>>,
    request: Request<Body>,
) -> Response {
    let (token, auth_ext) = extract_auth_inputs(&request);
    if let Some(denied) = authorize_staging_request(&control, token, auth_ext).await {
        return denied;
    }
    match control.list_patients().await {
        Ok(list) => Json(list).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn status_route(
    State(control): State<Arc<StagingSyntheticControl>>,
    request: Request<Body>,
) -> Response {
    let (token, auth_ext) = extract_auth_inputs(&request);
    if let Some(denied) = authorize_staging_request(&control, token, auth_ext).await {
        return denied;
    }
    match control.status().await {
        Ok(status) => Json(status).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn get_clock_route(
    State(control): State<Arc<StagingSyntheticControl>>,
    request: Request<Body>,
) -> Response {
    let (token, auth_ext) = extract_auth_inputs(&request);
    if let Some(denied) = authorize_staging_request(&control, token, auth_ext).await {
        return denied;
    }
    match control.clock().await {
        Ok(clock) => Json(clock).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn update_clock_route(
    State(control): State<Arc<StagingSyntheticControl>>,
    request: Request<Body>,
) -> Response {
    let (token, auth_ext) = extract_auth_inputs(&request);
    if let Some(denied) = authorize_staging_request(&control, token, auth_ext).await {
        return denied;
    }
    let update: SyntheticClockUpdate = match read_json_body(request).await {
        Ok(parsed) => parsed,
        Err(detail) => return bad_request_response(detail),
    };
    match control.update_clock(update).await {
        Ok(clock) => Json(clock).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn measure_route(
    State(control): State<Arc<StagingSyntheticControl>>,
    request: Request<Body>,
) -> Response {
    let (token, auth_ext) = extract_auth_inputs(&request);
    if let Some(denied) = authorize_staging_request(&control, token, auth_ext).await {
        return denied;
    }
    match control.measure().await {
        Ok(measure) => Json(measure).into_response(),
        Err(err) => err.into_response(),
    }
}

const MAX_CONTROL_BODY_BYTES: usize = 1024 * 1024;

fn bad_request_response(detail: String) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({ "status": "bad_request", "detail": detail })),
    )
        .into_response()
}

async fn read_json_body<T: serde::de::DeserializeOwned>(
    request: Request<Body>,
) -> Result<T, String> {
    let bytes = axum::body::to_bytes(request.into_body(), MAX_CONTROL_BODY_BYTES)
        .await
        .map_err(|err| err.to_string())?;
    serde_json::from_slice(&bytes).map_err(|err| err.to_string())
}

async fn read_optional_json_body<T: serde::de::DeserializeOwned + Default>(
    request: Request<Body>,
) -> Result<T, String> {
    let bytes = axum::body::to_bytes(request.into_body(), MAX_CONTROL_BODY_BYTES)
        .await
        .map_err(|err| err.to_string())?;
    if bytes.is_empty() {
        return Ok(T::default());
    }
    serde_json::from_slice(&bytes).map_err(|err| err.to_string())
}

async fn send_event_route(
    State(control): State<Arc<StagingSyntheticControl>>,
    request: Request<Body>,
) -> Response {
    let (token, auth_ext) = extract_auth_inputs(&request);
    if let Some(denied) = authorize_staging_request(&control, token, auth_ext).await {
        return denied;
    }
    let spec: SyntheticEventSpec = match read_json_body(request).await {
        Ok(parsed) => parsed,
        Err(detail) => return bad_request_response(detail),
    };
    match control.send_event(spec).await {
        Ok(saved) => (StatusCode::CREATED, Json(saved)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn source_failure_route(
    State(control): State<Arc<StagingSyntheticControl>>,
    request: Request<Body>,
) -> Response {
    let (token, auth_ext) = extract_auth_inputs(&request);
    if let Some(denied) = authorize_staging_request(&control, token, auth_ext).await {
        return denied;
    }
    let payload: SourceFailureRequest = match read_json_body(request).await {
        Ok(parsed) => parsed,
        Err(detail) => return bad_request_response(detail),
    };
    match control.set_source_failure(payload).await {
        Ok(resp) => Json(resp).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn rounds_route(
    State(control): State<Arc<StagingSyntheticControl>>,
    request: Request<Body>,
) -> Response {
    let (token, auth_ext) = extract_auth_inputs(&request);
    if let Some(denied) = authorize_staging_request(&control, token, auth_ext).await {
        return denied;
    }
    let payload: RoundsRequest = match read_optional_json_body(request).await {
        Ok(parsed) => parsed,
        Err(detail) => return bad_request_response(detail),
    };
    match control.run_rounds(payload).await {
        Ok(report) => Json(report).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn prepare_visits_route(
    State(control): State<Arc<StagingSyntheticControl>>,
    request: Request<Body>,
) -> Response {
    let (token, auth_ext) = extract_auth_inputs(&request);
    if let Some(denied) = authorize_staging_request(&control, token, auth_ext).await {
        return denied;
    }
    let payload: PrepareVisitsRequest = match read_optional_json_body(request).await {
        Ok(parsed) => parsed,
        Err(detail) => return bad_request_response(detail),
    };
    match control.prepare_visits(payload).await {
        Ok(report) => Json(report).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn reset_route(
    State(control): State<Arc<StagingSyntheticControl>>,
    request: Request<Body>,
) -> Response {
    let (token, auth_ext) = extract_auth_inputs(&request);
    if let Some(denied) = authorize_staging_request(&control, token, auth_ext.clone()).await {
        return denied;
    }
    match control.reset_with_auth(auth_ext.as_deref()).await {
        Ok(status) => Json(status).into_response(),
        Err(err) => err.into_response(),
    }
}

fn summarize_saved_review(
    saved: &SavedPatientReview,
    trigger_reason: &str,
) -> ExecutedReviewSummary {
    ExecutedReviewSummary {
        patient_workspace_id: saved.workspace.clone(),
        review_id: saved.review_id.clone(),
        trigger_reason: trigger_reason.to_owned(),
        status: saved.state.status_str().to_owned(),
        completed_source_version: saved.completed_source_version,
        action_count: saved.actions.len(),
    }
}

fn validate_fhir_id(id: &str) -> Result<(), SyntheticEventRefusal> {
    if id.is_empty() || id.len() > 128 || id.contains('/') || id.chars().any(char::is_whitespace) {
        return Err(SyntheticEventRefusal::MalformedFhirResource {
            reason: format!("invalid FHIR resource id `{id}`"),
        });
    }
    Ok(())
}

fn validate_iso_date_or_rfc3339(
    field: &'static str,
    value: &str,
) -> Result<(), SyntheticEventRefusal> {
    if is_valid_iso_date(value) || is_valid_rfc3339(value) {
        Ok(())
    } else {
        Err(SyntheticEventRefusal::InvalidTimestamp {
            field,
            value: value.to_owned(),
        })
    }
}

fn is_valid_iso_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    let Ok(year) = value[0..4].parse::<u32>() else {
        return false;
    };
    let Ok(month) = value[5..7].parse::<u32>() else {
        return false;
    };
    let Ok(day) = value[8..10].parse::<u32>() else {
        return false;
    };
    if !(1900..=2200).contains(&year) || day == 0 {
        return false;
    }
    let is_leap = (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400);
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap {
                29
            } else {
                28
            }
        }
        _ => return false,
    };
    day <= max_day
}

fn is_valid_rfc3339(value: &str) -> bool {
    // Reuse AppointmentWindow::parse by constructing a +1 second window or checking
    // date prefix + time separator + offset.
    let Some((date_part, rest)) = value.split_once('T') else {
        return false;
    };
    if !is_valid_iso_date(date_part) || rest.len() < 6 {
        return false;
    }
    let end_probe = "2299-12-31T23:59:59Z";
    AppointmentWindow::parse(value, end_probe).is_ok()
}

fn build_free_form_fhir_resource(
    kind: ChartEventRecordKind,
    id: &str,
    source_patient_id: &str,
    title: &str,
    text: &str,
    effective_at: &str,
) -> Value {
    let rendered = format!("{title}\nEffective: {effective_at}\n{text}");
    let encoded = base64::engine::general_purpose::STANDARD.encode(rendered.as_bytes());
    let subject = json!({ "reference": format!("Patient/{source_patient_id}") });
    match kind {
        ChartEventRecordKind::LabResult => json!({
            "resourceType": "Observation",
            "id": id,
            "status": "final",
            "subject": subject,
            "code": { "text": title },
            "effectiveDateTime": effective_at,
            "valueString": text,
        }),
        ChartEventRecordKind::ClinicalNote => json!({
            "resourceType": "DocumentReference",
            "id": id,
            "status": "current",
            "subject": subject,
            "description": title,
            "date": effective_at,
            "content": [{
                "attachment": {
                    "contentType": "text/plain; charset=utf-8",
                    "data": encoded,
                }
            }],
        }),
        ChartEventRecordKind::DiagnosticReport => json!({
            "resourceType": "DiagnosticReport",
            "id": id,
            "status": "final",
            "subject": subject,
            "code": { "text": title },
            "effectiveDateTime": effective_at,
            "conclusion": text,
            "presentedForm": [{
                "contentType": "text/plain; charset=utf-8",
                "data": encoded,
            }],
        }),
    }
}

fn validate_chart_resource_for_patient(
    workspace: &str,
    patient: &SourcePatient,
    resource: &Value,
) -> Result<(String, String, String), SyntheticEventRefusal> {
    if !resource.is_object() {
        return Err(SyntheticEventRefusal::MalformedFhirResource {
            reason: "FHIR chart resource must be a JSON object".to_owned(),
        });
    }
    let record_type = resource
        .get("resourceType")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .ok_or_else(|| SyntheticEventRefusal::MalformedFhirResource {
            reason: "record has no resourceType".to_owned(),
        })?;
    if record_type == "Patient" || !record_type.chars().all(|ch| ch.is_ascii_alphanumeric()) {
        return Err(SyntheticEventRefusal::MalformedFhirResource {
            reason: format!("unsupported chart event resourceType `{record_type}`"),
        });
    }
    let id = resource
        .get("id")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .ok_or_else(|| SyntheticEventRefusal::MalformedFhirResource {
            reason: "record has no id".to_owned(),
        })?;
    validate_fhir_id(id)?;

    if let Some(reference) = resource
        .get("subject")
        .and_then(|s| s.get("reference"))
        .and_then(Value::as_str)
        .map(str::trim)
    {
        let expected_primary = format!("Patient/{}", patient.source_patient_id);
        let matches_mrn = patient
            .medical_record_numbers
            .iter()
            .any(|mrn| reference == format!("Patient/{}", mrn.value));
        if reference != expected_primary && !matches_mrn {
            return Err(SyntheticEventRefusal::MalformedFhirResource {
                reason: format!(
                    "subject.reference `{reference}` does not match target patient `{expected_primary}`"
                ),
            });
        }
    }

    for field in synthetic_chart::DATE_FIELDS {
        if let Some(val) = resource.get(field) {
            let text = val
                .as_str()
                .ok_or_else(|| SyntheticEventRefusal::InvalidTimestamp {
                    field,
                    value: val.to_string(),
                })?;
            validate_iso_date_or_rfc3339(field, text)?;
        }
    }
    for (field, nested) in [
        (
            "period.start",
            resource.get("period").and_then(|p| p.get("start")),
        ),
        (
            "period.end",
            resource.get("period").and_then(|p| p.get("end")),
        ),
        (
            "meta.lastUpdated",
            resource.get("meta").and_then(|m| m.get("lastUpdated")),
        ),
    ] {
        if let Some(val) = nested {
            let text = val
                .as_str()
                .ok_or_else(|| SyntheticEventRefusal::InvalidTimestamp {
                    field,
                    value: val.to_string(),
                })?;
            validate_iso_date_or_rfc3339(field, text)?;
        }
    }

    let line = serde_json::to_string(resource).map_err(|err| {
        SyntheticEventRefusal::MalformedFhirResource {
            reason: err.to_string(),
        }
    })?;
    let revision: SourceRevision = synthetic_chart::parse_source_revision(workspace, &line)
        .map_err(|err| match err {
            ChartRecordError::UnsupportedAttachment(detail) => {
                SyntheticEventRefusal::UnsupportedAttachmentMedia { detail }
            }
            ChartRecordError::Malformed(reason) => {
                SyntheticEventRefusal::MalformedFhirResource { reason }
            }
        })?;
    if revision.dates.is_empty() {
        return Err(SyntheticEventRefusal::MalformedFhirResource {
            reason: "chart record must carry at least one valid clinical date field".to_owned(),
        });
    }
    if revision.readable_parts.is_empty()
        && let Some(env) = &revision.envelope
        && !env.unavailable_parts.is_empty()
    {
        return Err(SyntheticEventRefusal::UnsupportedAttachmentMedia {
            detail: env
                .unavailable_parts
                .iter()
                .map(|part| part.reason.as_str())
                .collect::<Vec<_>>()
                .join("; "),
        });
    }
    if revision
        .readable_parts
        .iter()
        .any(|part| part.text.trim().is_empty())
    {
        return Err(SyntheticEventRefusal::EmptyRecordText);
    }

    Ok((
        revision.record_type,
        revision.content,
        revision.candidate_id,
    ))
}

fn validate_fhir_appointment_resource(
    resource: &Value,
) -> Result<SourceAppointment, SyntheticEventRefusal> {
    if !resource.is_object() {
        return Err(SyntheticEventRefusal::InvalidAppointment {
            reason: "FHIR Appointment must be a JSON object".to_owned(),
        });
    }
    for (field, val) in [
        ("start", resource.get("start")),
        ("end", resource.get("end")),
    ] {
        if let Some(text) = val.and_then(Value::as_str)
            && !is_valid_rfc3339(text)
        {
            return Err(SyntheticEventRefusal::InvalidAppointment {
                reason: format!(
                    "Appointment `{field}` (`{text}`) is not a valid RFC3339 civil timestamp"
                ),
            });
        }
    }
    let line = serde_json::to_string(resource).map_err(|err| {
        SyntheticEventRefusal::InvalidAppointment {
            reason: err.to_string(),
        }
    })?;
    let parsed = schedule::parse_fhir_appointment_line(&line)
        .map_err(|reason| SyntheticEventRefusal::InvalidAppointment { reason })?;
    if parsed.source_patient_id().is_none() {
        return Err(SyntheticEventRefusal::InvalidAppointment {
            reason: "Appointment must list one Patient/<id> participant".to_owned(),
        });
    }
    Ok(parsed)
}

fn append_ndjson_line(path: &Path, line: &str) -> Result<(), SyntheticEventRefusal> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| SyntheticEventRefusal::StorageIo { source })?;
    }
    let mut existing = if path.exists() {
        fs::read_to_string(path).map_err(|source| SyntheticEventRefusal::StorageIo { source })?
    } else {
        String::new()
    };
    if !existing.is_empty() && !existing.ends_with('\n') {
        existing.push('\n');
    }
    existing.push_str(line);
    existing.push('\n');
    fs::write(path, existing).map_err(|source| SyntheticEventRefusal::StorageIo { source })
}

fn upsert_appointment_ndjson_line(
    path: &Path,
    appointment_id: &str,
    new_line: &str,
) -> Result<(), SyntheticEventRefusal> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| SyntheticEventRefusal::StorageIo { source })?;
    }
    let existing = if path.exists() {
        fs::read_to_string(path).map_err(|source| SyntheticEventRefusal::StorageIo { source })?
    } else {
        String::new()
    };
    let mut replaced = false;
    let mut lines: Vec<String> = Vec::new();
    for raw in existing.lines() {
        if raw.trim().is_empty() {
            continue;
        }
        let is_target = serde_json::from_str::<Value>(raw)
            .ok()
            .and_then(|val| val.get("id").and_then(Value::as_str).map(str::to_owned))
            .is_some_and(|id| id == appointment_id);
        if is_target {
            if !replaced {
                lines.push(new_line.to_owned());
                replaced = true;
            }
        } else {
            lines.push(raw.to_owned());
        }
    }
    if !replaced {
        lines.push(new_line.to_owned());
    }
    let mut output = lines.join("\n");
    output.push('\n');
    fs::write(path, output).map_err(|source| SyntheticEventRefusal::StorageIo { source })
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(dst)?;
    let mut entries: Vec<_> = fs::read_dir(src)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let file_type = entry.file_type()?;
        let target = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}

/// Typed `sea_query` identifiers for atomic staging practice runtime reset and
/// conversation count queries (zero raw SQL strings).
mod runtime_tables {
    use super::Iden;

    #[derive(Clone, Copy)]
    pub(super) enum PracticeTable {
        Events,
        ChatAttempts,
        ChatDrafts,
        ResponseFeedback,
        Conversations,
        RevisionAttempts,
        ReplyRevisions,
        ReplyDrafts,
        ReplyCopies,
        ReplyDraftBindings,
        ReplyReviewCompletions,
        ReplyProposals,
        ReplyConversations,
        SpruceMatchingRuns,
        SpruceReviewTriggers,
        SpruceEnteredInError,
        SpruceMessages,
        SpruceConversations,
        SpruceConfirmedSenders,
        SpruceEhrLinks,
        SpruceContacts,
        SpruceChannels,
        LinkCorrectionWorkspaces,
        LinkCorrections,
        FollowUpUpdates,
        FollowUps,
        ConcernWithdrawals,
        UnderstandingWithdrawals,
        MemoryWithdrawals,
        SourceWithdrawals,
        CarePlanRecommendations,
        PracticeJobAttempts,
        PracticeJobs,
        PatientReviews,
        VisitConcerns,
        VisitBriefings,
        VisitPreparations,
        ConcernFeedback,
        Concerns,
        MemoryVersions,
        MemoryNotes,
        MemoryRounds,
        PatientUnderstandingRevisions,
        PatientUnderstanding,
        SourceRefreshReceipts,
        SourceStatuses,
        PatientSourceStatus,
        PracticeSourceAccess,
        ReconnectCommands,
        SourceAdapterState,
        SourcePatientLinks,
        WorkspacePractices,
        PatientDispatchTriggers,
        PracticeCadence,
        PatientDispatch,
        Appointments,
        SchedulePatientLinks,
        ScheduleSources,
        PanelPreferences,
        HintCaptures,
        HintJobs,
        HintSchedules,
        HintPairings,
        Commands,
        Recipes,
        SkillPackages,
        TodayUpdatesSeen,
        TodayWorkDismissals,
    }

    impl Iden for PracticeTable {
        fn unquoted(&self) -> &'static str {
            match self {
                Self::Events => "physician_runtime_events",
                Self::ChatAttempts => "physician_runtime_chat_attempts",
                Self::ChatDrafts => "physician_runtime_chat_drafts",
                Self::ResponseFeedback => "physician_runtime_response_feedback",
                Self::Conversations => "physician_runtime_conversations",
                Self::RevisionAttempts => "physician_runtime_revision_attempts",
                Self::ReplyRevisions => "physician_runtime_reply_revisions",
                Self::ReplyDrafts => "physician_runtime_reply_drafts",
                Self::ReplyCopies => "physician_runtime_reply_copies",
                Self::ReplyDraftBindings => "physician_runtime_reply_draft_bindings",
                Self::ReplyReviewCompletions => "physician_runtime_reply_review_completions",
                Self::ReplyProposals => "physician_runtime_reply_proposals",
                Self::ReplyConversations => "physician_runtime_reply_conversations",
                Self::SpruceMatchingRuns => "physician_runtime_spruce_matching_runs",
                Self::SpruceReviewTriggers => "physician_runtime_spruce_review_triggers",
                Self::SpruceEnteredInError => "physician_runtime_spruce_entered_in_error",
                Self::SpruceMessages => "physician_runtime_spruce_messages",
                Self::SpruceConversations => "physician_runtime_spruce_conversations",
                Self::SpruceConfirmedSenders => "physician_runtime_spruce_confirmed_senders",
                Self::SpruceEhrLinks => "physician_runtime_spruce_ehr_links",
                Self::SpruceContacts => "physician_runtime_spruce_contacts",
                Self::SpruceChannels => "physician_runtime_spruce_channels",
                Self::LinkCorrectionWorkspaces => "physician_runtime_link_correction_workspaces",
                Self::LinkCorrections => "physician_runtime_link_corrections",
                Self::FollowUpUpdates => "physician_runtime_follow_up_updates",
                Self::FollowUps => "physician_runtime_follow_ups",
                Self::ConcernWithdrawals => "physician_runtime_concern_withdrawals",
                Self::UnderstandingWithdrawals => "physician_runtime_understanding_withdrawals",
                Self::MemoryWithdrawals => "physician_runtime_memory_withdrawals",
                Self::SourceWithdrawals => "physician_runtime_source_withdrawals",
                Self::CarePlanRecommendations => "physician_runtime_care_plan_recommendations",
                Self::PracticeJobAttempts => "physician_runtime_practice_job_attempts",
                Self::PracticeJobs => "physician_runtime_practice_jobs",
                Self::PatientReviews => "physician_runtime_patient_reviews",
                Self::VisitConcerns => "physician_runtime_visit_concerns",
                Self::VisitBriefings => "physician_runtime_visit_briefings",
                Self::VisitPreparations => "physician_runtime_visit_preparations",
                Self::ConcernFeedback => "physician_runtime_concern_feedback",
                Self::Concerns => "physician_runtime_concerns",
                Self::MemoryVersions => "physician_runtime_memory_versions",
                Self::MemoryNotes => "physician_runtime_memory_notes",
                Self::MemoryRounds => "physician_runtime_memory_rounds",
                Self::PatientUnderstandingRevisions => {
                    "physician_runtime_patient_understanding_revisions"
                }
                Self::PatientUnderstanding => "physician_runtime_patient_understanding",
                Self::SourceRefreshReceipts => "physician_runtime_source_refresh_receipts",
                Self::SourceStatuses => "physician_runtime_source_statuses",
                Self::PatientSourceStatus => "physician_runtime_patient_source_status",
                Self::PracticeSourceAccess => "physician_runtime_practice_source_access",
                Self::ReconnectCommands => "physician_runtime_reconnect_commands",
                Self::SourceAdapterState => "physician_runtime_source_adapter_state",
                Self::SourcePatientLinks => "physician_runtime_source_patient_links",
                Self::WorkspacePractices => "physician_runtime_workspace_practices",
                Self::PatientDispatchTriggers => "physician_runtime_patient_dispatch_triggers",
                Self::PracticeCadence => "physician_runtime_practice_cadence",
                Self::PatientDispatch => "physician_runtime_patient_dispatch",
                Self::Appointments => "physician_runtime_appointments",
                Self::SchedulePatientLinks => "physician_runtime_schedule_patient_links",
                Self::ScheduleSources => "physician_runtime_schedule_sources",
                Self::PanelPreferences => "physician_runtime_panel_preferences",
                Self::HintCaptures => "physician_runtime_hint_captures",
                Self::HintJobs => "physician_runtime_hint_jobs",
                Self::HintSchedules => "physician_runtime_hint_schedules",
                Self::HintPairings => "physician_runtime_hint_pairings",
                Self::Commands => "physician_runtime_commands",
                Self::Recipes => "physician_runtime_recipes",
                Self::SkillPackages => "physician_runtime_skill_packages",
                Self::TodayUpdatesSeen => "physician_runtime_today_updates_seen",
                Self::TodayWorkDismissals => "physician_runtime_today_work_dismissals",
            }
        }
    }

    #[derive(Clone, Copy)]
    pub(super) enum WorkspaceTable {
        Sources,
        SourcePulls,
        SourceVersions,
    }

    impl Iden for WorkspaceTable {
        fn unquoted(&self) -> &'static str {
            match self {
                Self::Sources => "physician_runtime_sources",
                Self::SourcePulls => "physician_runtime_source_pulls",
                Self::SourceVersions => "physician_runtime_source_versions",
            }
        }
    }

    #[derive(Clone, Copy)]
    pub(super) enum AuthPracticeTable {
        PatientLinks,
        Memberships,
        Patients,
    }

    impl Iden for AuthPracticeTable {
        fn unquoted(&self) -> &'static str {
            match self {
                Self::PatientLinks => "physician_patient_links",
                Self::Memberships => "physician_memberships",
                Self::Patients => "physician_patients",
            }
        }
    }

    #[derive(Clone, Copy)]
    pub(super) enum RuntimeCol {
        Practice,
        PracticeId,
        Workspace,
        Chat,
        Status,
        ReplyState,
        LinkKind,
        Outcome,
    }

    impl Iden for RuntimeCol {
        fn unquoted(&self) -> &'static str {
            match self {
                Self::Practice => "practice",
                Self::PracticeId => "practice_id",
                Self::Workspace => "workspace",
                Self::Chat => "chat",
                Self::Status => "status",
                Self::ReplyState => "reply_state",
                Self::LinkKind => "link_kind",
                Self::Outcome => "outcome",
            }
        }
    }

    pub(super) const ALL_PRACTICE_TABLES: &[PracticeTable] = &[
        PracticeTable::Events,
        PracticeTable::ChatAttempts,
        PracticeTable::ChatDrafts,
        PracticeTable::ResponseFeedback,
        PracticeTable::Conversations,
        PracticeTable::RevisionAttempts,
        PracticeTable::ReplyRevisions,
        PracticeTable::ReplyDrafts,
        PracticeTable::ReplyCopies,
        PracticeTable::ReplyDraftBindings,
        PracticeTable::ReplyReviewCompletions,
        PracticeTable::ReplyProposals,
        PracticeTable::ReplyConversations,
        PracticeTable::SpruceMatchingRuns,
        PracticeTable::SpruceReviewTriggers,
        PracticeTable::SpruceEnteredInError,
        PracticeTable::SpruceMessages,
        PracticeTable::SpruceConversations,
        PracticeTable::SpruceConfirmedSenders,
        PracticeTable::SpruceEhrLinks,
        PracticeTable::SpruceContacts,
        PracticeTable::SpruceChannels,
        PracticeTable::LinkCorrectionWorkspaces,
        PracticeTable::LinkCorrections,
        PracticeTable::FollowUpUpdates,
        PracticeTable::FollowUps,
        PracticeTable::ConcernWithdrawals,
        PracticeTable::UnderstandingWithdrawals,
        PracticeTable::MemoryWithdrawals,
        PracticeTable::SourceWithdrawals,
        PracticeTable::CarePlanRecommendations,
        PracticeTable::PracticeJobAttempts,
        PracticeTable::PracticeJobs,
        PracticeTable::PatientReviews,
        PracticeTable::VisitConcerns,
        PracticeTable::VisitBriefings,
        PracticeTable::VisitPreparations,
        PracticeTable::ConcernFeedback,
        PracticeTable::Concerns,
        PracticeTable::MemoryVersions,
        PracticeTable::MemoryNotes,
        PracticeTable::MemoryRounds,
        PracticeTable::PatientUnderstandingRevisions,
        PracticeTable::PatientUnderstanding,
        PracticeTable::SourceRefreshReceipts,
        PracticeTable::SourceStatuses,
        PracticeTable::PatientSourceStatus,
        PracticeTable::PracticeSourceAccess,
        PracticeTable::ReconnectCommands,
        PracticeTable::SourceAdapterState,
        PracticeTable::SourcePatientLinks,
        PracticeTable::WorkspacePractices,
        PracticeTable::PatientDispatchTriggers,
        PracticeTable::PracticeCadence,
        PracticeTable::PatientDispatch,
        PracticeTable::Appointments,
        PracticeTable::SchedulePatientLinks,
        PracticeTable::ScheduleSources,
        PracticeTable::PanelPreferences,
        PracticeTable::HintCaptures,
        PracticeTable::HintJobs,
        PracticeTable::HintSchedules,
        PracticeTable::HintPairings,
        PracticeTable::Commands,
        PracticeTable::Recipes,
        PracticeTable::SkillPackages,
        PracticeTable::TodayUpdatesSeen,
        PracticeTable::TodayWorkDismissals,
    ];

    pub(super) const ALL_WORKSPACE_TABLES: &[WorkspaceTable] = &[
        WorkspaceTable::Sources,
        WorkspaceTable::SourcePulls,
        WorkspaceTable::SourceVersions,
    ];

    pub(super) const ALL_AUTH_PRACTICE_TABLES: &[AuthPracticeTable] = &[
        AuthPracticeTable::PatientLinks,
        AuthPracticeTable::Memberships,
        AuthPracticeTable::Patients,
    ];
}

/// Returns every `physician_runtime_*` table name cleared by
/// `POST /staging-synthetic/reset` (both practice-scoped and workspace-scoped).
pub fn reset_covered_runtime_table_names() -> Vec<&'static str> {
    use runtime_tables::{ALL_PRACTICE_TABLES, ALL_WORKSPACE_TABLES};
    let mut names: Vec<&'static str> = ALL_PRACTICE_TABLES
        .iter()
        .map(Iden::unquoted)
        .chain(ALL_WORKSPACE_TABLES.iter().map(Iden::unquoted))
        .collect();
    names.sort_unstable();
    names
}

#[derive(Default)]
struct L1MeasureDbCounts {
    open_concerns: usize,
    concerns_raised: usize,
    concerns_addressed: usize,
    ready_replies: usize,
    stale_replies_retired: usize,
    follow_ups_created: usize,
    follow_ups_completed: usize,
    unhandled_follow_ups: usize,
    unmatched_spruce_conversations: usize,
    spruce_messages_ingested: usize,
    failed_source_statuses: usize,
}

async fn count_where(
    db: &DatabaseConnection,
    select: sea_orm::sea_query::SelectStatement,
) -> Result<usize, DbErr> {
    let Some(row) = db.query_one(&select).await? else {
        return Ok(0);
    };
    let count: i64 = row.try_get_by_index(0)?;
    Ok(usize::try_from(count).unwrap_or(0))
}

async fn query_l1_measure_counts(
    db: &DatabaseConnection,
    practice_id: &str,
) -> Result<L1MeasureDbCounts, DbErr> {
    use runtime_tables::{PracticeTable, RuntimeCol};
    use sea_orm::sea_query::ExprTrait as _;

    let count_all = |table: PracticeTable| {
        Query::select()
            .expr(Expr::col(RuntimeCol::Practice).count())
            .from(table)
            .and_where(Expr::col(RuntimeCol::Practice).eq(practice_id))
            .to_owned()
    };

    let open_concerns = count_where(
        db,
        count_all(PracticeTable::Concerns)
            .and_where(Expr::col(RuntimeCol::Status).eq("open"))
            .to_owned(),
    )
    .await?;
    let concerns_raised = count_where(db, count_all(PracticeTable::Concerns)).await?;
    let concerns_addressed = concerns_raised.saturating_sub(open_concerns);

    let ready_replies = count_where(
        db,
        count_all(PracticeTable::ReplyConversations)
            .and_where(Expr::col(RuntimeCol::ReplyState).eq("ready"))
            .to_owned(),
    )
    .await?;
    let stale_replies_retired = count_where(
        db,
        count_all(PracticeTable::ReplyConversations)
            .and_where(Expr::col(RuntimeCol::ReplyState).eq("withheld"))
            .to_owned(),
    )
    .await?;

    let follow_ups_created = count_where(db, count_all(PracticeTable::FollowUps)).await?;
    let follow_ups_completed = count_where(
        db,
        count_all(PracticeTable::FollowUps)
            .and_where(Expr::col(RuntimeCol::Status).eq("completed"))
            .to_owned(),
    )
    .await?;
    let unhandled_follow_ups = count_where(
        db,
        count_all(PracticeTable::FollowUps)
            .and_where(Expr::col(RuntimeCol::Status).eq("cannot_restart"))
            .to_owned(),
    )
    .await?;

    let unmatched_spruce_conversations = count_where(
        db,
        count_all(PracticeTable::SpruceConversations)
            .and_where(Expr::col(RuntimeCol::LinkKind).ne("linked"))
            .to_owned(),
    )
    .await?;
    let spruce_messages_ingested =
        count_where(db, count_all(PracticeTable::SpruceMessages)).await?;

    let failed_source_statuses = count_where(
        db,
        count_all(PracticeTable::SourceStatuses)
            .and_where(Expr::col(RuntimeCol::Outcome).eq("failed"))
            .to_owned(),
    )
    .await?;

    Ok(L1MeasureDbCounts {
        open_concerns,
        concerns_raised,
        concerns_addressed,
        ready_replies,
        stale_replies_retired,
        follow_ups_created,
        follow_ups_completed,
        unhandled_follow_ups,
        unmatched_spruce_conversations,
        spruce_messages_ingested,
        failed_source_statuses,
    })
}

async fn count_practice_conversations(
    db: &DatabaseConnection,
    practice_id: &str,
) -> Result<usize, DbErr> {
    use runtime_tables::{PracticeTable, RuntimeCol};
    use sea_orm::sea_query::ExprTrait as _;

    let select = Query::select()
        .expr(Expr::col(RuntimeCol::Chat).count())
        .from(PracticeTable::Conversations)
        .and_where(Expr::col(RuntimeCol::Practice).eq(practice_id))
        .to_owned();
    let Some(row) = db.query_one(&select).await? else {
        return Ok(0);
    };
    let count: i64 = row.try_get_by_index(0)?;
    Ok(usize::try_from(count).unwrap_or(0))
}

async fn purge_practice_runtime_state(
    db: &DatabaseConnection,
    practice_id: &str,
    workspaces: &[String],
) -> Result<(), DbErr> {
    use runtime_tables::{
        ALL_AUTH_PRACTICE_TABLES, ALL_PRACTICE_TABLES, ALL_WORKSPACE_TABLES, PracticeTable,
        RuntimeCol,
    };
    use sea_orm::sea_query::ExprTrait as _;

    let _ = PracticeTable::TodayUpdatesSeen;
    let tx = db.begin().await?;

    for table in ALL_PRACTICE_TABLES {
        let delete = Query::delete()
            .from_table(*table)
            .and_where(Expr::col(RuntimeCol::Practice).eq(practice_id))
            .to_owned();
        tx.execute(&delete).await?;
    }

    let workspace_set: BTreeSet<String> = workspaces
        .iter()
        .map(|w| w.trim().to_owned())
        .filter(|w| !w.is_empty())
        .collect();
    if !workspace_set.is_empty() {
        let ws_list: Vec<String> = workspace_set.into_iter().collect();
        for table in ALL_WORKSPACE_TABLES {
            let delete = Query::delete()
                .from_table(*table)
                .and_where(Expr::col(RuntimeCol::Workspace).is_in(ws_list.clone()))
                .to_owned();
            tx.execute(&delete).await?;
        }
    }

    for table in ALL_AUTH_PRACTICE_TABLES {
        let delete = Query::delete()
            .from_table(*table)
            .and_where(Expr::col(RuntimeCol::PracticeId).eq(practice_id))
            .to_owned();
        tx.execute(&delete).await?;
    }

    tx.commit().await?;
    Ok(())
}
