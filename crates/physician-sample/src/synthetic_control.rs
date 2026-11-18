//! Staging-only synthetic event intake and mutable bank source adapter for
//! [`crate::STAGING_PRACTICE_ID`].
//!
//! Synthetic events never write to PostgreSQL. Accepted chart events write
//! NDJSON records exclusively into the mutable staging working bank's
//! `patients/` directory, and accepted schedule events write NDJSON records
//! exclusively into `calendar/Appointment.ndjson`. Records enter PostgreSQL
//! solely through [`PatientSourceRefresh`] / [`PatientSource`] and
//! [`ScheduleSource`].

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use base64::Engine as _;
use local_product::physician_sources::{
    PatientSource, SourcePatient, SourceRevision, workspace_id,
};
use local_product::runtime::refresh::{
    PatientSourceRefresh, SourceCheckOutcome, SourceId as RefreshSourceId,
    SourceRefreshAdapterError,
};
use local_product::runtime::schedule::{
    AppointmentWindow, ScheduleDelivery, ScheduleSource, ScheduleSourceError, SourceAppointment,
    SourceId as ScheduleSourceId,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::auth::STAGING_PRACTICE_ID;
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

/// Which synthetic source adapter an operation targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyntheticSourceKind {
    /// Synthetic EHR chart source (`synthetic-charts`).
    Chart,
    /// Synthetic practice schedule source (`synthetic-practice-calendar`).
    Calendar,
}

impl SyntheticSourceKind {
    /// Human-readable source adapter name.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Chart => "Synthetic chart",
            Self::Calendar => schedule::SOURCE_LABEL,
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

/// Current health directives across both synthetic source adapters.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceHealthSnapshot {
    /// Health directive of the synthetic chart source.
    pub chart: SourceHealthDirective,
    /// Health directive of the synthetic calendar source.
    pub calendar: SourceHealthDirective,
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
            Self::StorageIo { .. } => "source_storage_io",
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
            | Self::InvalidAppointment { .. } => None,
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
            | SyntheticEventSpec::RawFhirChartResource { .. } => SyntheticSourceKind::Chart,
            SyntheticEventSpec::VisitChange { .. }
            | SyntheticEventSpec::RawFhirAppointment { .. } => SyntheticSourceKind::Calendar,
        };

        let directive = {
            let health = self.health_snapshot();
            match target_source {
                SyntheticSourceKind::Chart => health.chart,
                SyntheticSourceKind::Calendar => health.calendar,
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
