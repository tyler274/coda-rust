//! Synthetic bank calendar adapter and practice schedule sync/readback routes.
//!
//! [`SyntheticBankCalendar`] reads `calendar/Appointment.ndjson` from a built
//! synthetic patient bank through [`ScheduleSource`]. [`sync_synthetic_schedule`]
//! seeds schedule patient links strictly from patients already registered on
//! the practice's roster and persists the schedule through
//! [`Store::sync_practice_schedule`].

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use axum::Json;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use local_product::physician_sources::RecordDate;
use local_product::runtime::Store;
use local_product::runtime::briefings::{
    VisitPreparationRecord, VisitPreparationState, visit_can_start_preparation,
};
use local_product::runtime::schedule::{
    AppointmentId, AppointmentStatus, AppointmentWindow, PatientMatch, SavedAppointment,
    ScheduleDelivery, ScheduleDomainError, ScheduleError, ScheduleFreshness, ScheduleReadIssue,
    ScheduleSource, ScheduleSourceError, ScheduleStanding, ScheduleSyncOutcome, SourceAppointment,
    SourceAppointmentParts, SourceId, SourcePatientId, format_unix_rfc3339,
};
use physician_auth::AuthStore;
use serde_json::{Value, json};

use crate::auth::now;
use crate::care::ConnectedCare;
use crate::patients::{self, PRACTICE_TIME_ZONE};

/// Stable source identifier for the synthetic bank practice calendar.
pub const SOURCE_ID: &str = "synthetic-practice-calendar";

/// Physician-facing source label for the synthetic bank practice calendar.
pub const SOURCE_LABEL: &str = "Synthetic practice calendar";

/// Environment variable naming a synthetic calendar NDJSON file or bank root.
pub const CALENDAR_ENV: &str = "PHYSICIAN_SYNTHETIC_CALENDAR";

/// Synthetic practice calendar backed by a built patient bank's
/// `calendar/Appointment.ndjson` file.
#[derive(Clone, Debug)]
pub struct SyntheticBankCalendar {
    source_id: SourceId,
    appointments_file: PathBuf,
}

impl SyntheticBankCalendar {
    /// Opens `bank/calendar/Appointment.ndjson`.
    pub fn open(bank: &Path) -> Self {
        Self::from_file(bank.join("calendar").join("Appointment.ndjson"))
    }

    /// Opens an explicit `Appointment.ndjson` file path.
    pub fn from_file(appointments_file: impl Into<PathBuf>) -> Self {
        Self {
            source_id: SourceId::new(SOURCE_ID).expect("static synthetic calendar source_id"),
            appointments_file: appointments_file.into(),
        }
    }

    /// Resolves a synthetic bank calendar from environment configuration:
    /// - `PHYSICIAN_SYNTHETIC_CALENDAR` if set (accepting either a bank directory
    ///   containing `calendar/Appointment.ndjson` or a direct `.ndjson` path,
    ///   even if temporarily missing so failed reads are reported honestly);
    /// - otherwise `PHYSICIAN_SYNTHETIC_CHARTS` when its `calendar/Appointment.ndjson`
    ///   file is present on disk.
    pub fn from_env(get: impl Fn(&str) -> Option<String>) -> Option<Self> {
        if let Some(configured) = get(CALENDAR_ENV)
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
        {
            let path = PathBuf::from(configured);
            if path.is_dir() {
                return Some(Self::open(&path));
            }
            return Some(Self::from_file(path));
        }
        let charts = get("PHYSICIAN_SYNTHETIC_CHARTS")
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())?;
        let calendar_file = Path::new(&charts)
            .join("calendar")
            .join("Appointment.ndjson");
        calendar_file
            .is_file()
            .then(|| Self::from_file(calendar_file))
    }

    /// Underlying `Appointment.ndjson` path read by this adapter.
    pub fn path(&self) -> &Path {
        &self.appointments_file
    }
}

#[derive(Debug)]
enum AppointmentParseError {
    Json(serde_json::Error),
    WrongResourceType(Option<String>),
    MissingField(&'static str),
    UnsupportedStatus(String),
    AmbiguousPatientParticipants,
    Domain(ScheduleDomainError),
}

impl std::fmt::Display for AppointmentParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(f, "invalid JSON: {error}"),
            Self::WrongResourceType(got) => {
                write!(f, "expected resourceType \"Appointment\", got {got:?}")
            }
            Self::MissingField(field) => {
                write!(
                    f,
                    "Appointment is missing required string field \"{field}\""
                )
            }
            Self::UnsupportedStatus(status) => {
                write!(f, "unsupported Appointment status {status:?}")
            }
            Self::AmbiguousPatientParticipants => {
                f.write_str("Appointment lists multiple distinct Patient participants")
            }
            Self::Domain(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for AppointmentParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::Domain(error) => Some(error),
            Self::WrongResourceType(_)
            | Self::MissingField(_)
            | Self::UnsupportedStatus(_)
            | Self::AmbiguousPatientParticipants => None,
        }
    }
}

impl From<serde_json::Error> for AppointmentParseError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<ScheduleDomainError> for AppointmentParseError {
    fn from(value: ScheduleDomainError) -> Self {
        Self::Domain(value)
    }
}

#[async_trait]
impl ScheduleSource for SyntheticBankCalendar {
    fn source_id(&self) -> &SourceId {
        &self.source_id
    }

    fn source_label(&self) -> &str {
        SOURCE_LABEL
    }

    async fn read_schedule(&self) -> Result<ScheduleDelivery, ScheduleSourceError> {
        let text = fs::read_to_string(&self.appointments_file).map_err(|source| {
            ScheduleSourceError::Io {
                source_id: self.source_id.clone(),
                source,
            }
        })?;
        let file_label = self
            .appointments_file
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Appointment.ndjson");

        let mut appointments: Vec<SourceAppointment> = Vec::new();
        let mut seen_digests: BTreeMap<AppointmentId, String> = BTreeMap::new();
        let mut issues: Vec<ScheduleReadIssue> = Vec::new();

        for (idx, raw_line) in text.lines().enumerate() {
            let line_no = idx + 1;
            if raw_line.trim().is_empty() {
                continue;
            }
            match parse_fhir_appointment(raw_line) {
                Ok(appointment) => {
                    let id = appointment.appointment_id().clone();
                    if let Some(prior_sha) = seen_digests.get(&id) {
                        if prior_sha != appointment.sha256() {
                            let issue = ScheduleReadIssue::new(
                                format!("Appointment/{id} ({file_label}:{line_no})"),
                                "duplicate appointment id with conflicting content",
                            )
                            .expect("non-empty issue target and reason");
                            issues.push(issue);
                        }
                        continue;
                    }
                    seen_digests.insert(id, appointment.sha256().to_owned());
                    appointments.push(appointment);
                }
                Err(error) => {
                    let target = extract_target_label(raw_line, file_label, line_no);
                    let issue = ScheduleReadIssue::new(target, error.to_string())
                        .expect("non-empty issue target and reason");
                    issues.push(issue);
                }
            }
        }

        ScheduleDelivery::from_outcomes(appointments, issues).map_err(|error| {
            ScheduleSourceError::Unavailable {
                source_id: self.source_id.clone(),
                reason: error.to_string(),
            }
        })
    }
}

fn extract_target_label(raw_line: &str, file_label: &str, line_no: usize) -> String {
    if let Ok(value) = serde_json::from_str::<Value>(raw_line)
        && let Some(id) = value
            .get("id")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|id| !id.is_empty())
    {
        return format!("Appointment/{id}");
    }
    format!("{file_label}:{line_no}")
}

fn parse_fhir_appointment(line: &str) -> Result<SourceAppointment, AppointmentParseError> {
    let resource: Value = serde_json::from_str(line)?;
    let resource_type = resource.get("resourceType").and_then(Value::as_str);
    if resource_type != Some("Appointment") {
        return Err(AppointmentParseError::WrongResourceType(
            resource_type.map(str::to_owned),
        ));
    }
    let raw_id = resource
        .get("id")
        .and_then(Value::as_str)
        .ok_or(AppointmentParseError::MissingField("id"))?;
    let appointment_id = AppointmentId::new(raw_id)?;

    let source_status = resource
        .get("status")
        .and_then(Value::as_str)
        .ok_or(AppointmentParseError::MissingField("status"))?
        .trim()
        .to_owned();
    let status = parse_fhir_status(&source_status)?;

    let start = resource
        .get("start")
        .and_then(Value::as_str)
        .ok_or(AppointmentParseError::MissingField("start"))?;
    let end = resource
        .get("end")
        .and_then(Value::as_str)
        .ok_or(AppointmentParseError::MissingField("end"))?;
    let window = AppointmentWindow::parse(start, end)?;

    let label = extract_appointment_label(&resource)
        .ok_or(AppointmentParseError::MissingField("description"))?;
    let source_patient_id = extract_patient_participant(&resource)?;
    let dates = extract_appointment_dates(&resource);
    let candidate_id = format!("Appointment/{appointment_id}");

    Ok(SourceAppointment::new(SourceAppointmentParts {
        appointment_id,
        source_revision: None,
        window,
        status,
        source_status,
        label,
        source_patient_id,
        dates,
        content: line.to_owned(),
        provenance: json!({ "native_record": candidate_id }),
    })?)
}

fn parse_fhir_status(status: &str) -> Result<AppointmentStatus, AppointmentParseError> {
    match status {
        "booked" | "pending" | "proposed" | "arrived" | "checked-in" | "waitlist" | "scheduled" => {
            Ok(AppointmentStatus::Scheduled)
        }
        "cancelled" | "entered-in-error" => Ok(AppointmentStatus::Cancelled),
        "fulfilled" => Ok(AppointmentStatus::Fulfilled),
        "noshow" => Ok(AppointmentStatus::NoShow),
        other => Err(AppointmentParseError::UnsupportedStatus(other.to_owned())),
    }
}

fn extract_appointment_label(resource: &Value) -> Option<String> {
    let candidates = [
        resource.get("description").and_then(Value::as_str),
        resource
            .get("appointmentType")
            .and_then(|v| v.get("text"))
            .and_then(Value::as_str),
        resource
            .get("serviceType")
            .and_then(Value::as_array)
            .and_then(|arr| arr.first())
            .and_then(|v| v.get("text"))
            .and_then(Value::as_str),
    ];
    candidates
        .into_iter()
        .flatten()
        .map(str::trim)
        .find(|text| !text.is_empty())
        .map(str::to_owned)
}

fn extract_patient_participant(
    resource: &Value,
) -> Result<Option<SourcePatientId>, AppointmentParseError> {
    let mut matched: Option<SourcePatientId> = None;
    for participant in resource
        .get("participant")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let Some(reference) = participant
            .get("actor")
            .and_then(|actor| actor.get("reference"))
            .and_then(Value::as_str)
            .map(str::trim)
        else {
            continue;
        };
        let Some(raw_patient_id) = reference.strip_prefix("Patient/") else {
            continue;
        };
        let patient_id = SourcePatientId::new(raw_patient_id)?;
        if let Some(existing) = &matched
            && existing != &patient_id
        {
            return Err(AppointmentParseError::AmbiguousPatientParticipants);
        }
        matched = Some(patient_id);
    }
    Ok(matched)
}

fn extract_appointment_dates(resource: &Value) -> Vec<RecordDate> {
    let mut dates = Vec::new();
    for (name, value) in [
        ("start", resource.get("start")),
        ("end", resource.get("end")),
        (
            "meta.lastUpdated",
            resource
                .get("meta")
                .and_then(|meta| meta.get("lastUpdated")),
        ),
    ] {
        if let Some(text) = value.and_then(Value::as_str) {
            dates.push(RecordDate {
                name: name.to_owned(),
                value: text.to_owned(),
            });
        }
    }
    dates
}

/// Seeds schedule patient links for `practice_id` from already-loaded roster
/// chart pulls and syncs `calendar` into `runtime` at `practice_id`'s clock time.
pub async fn sync_synthetic_schedule(
    calendar: &dyn ScheduleSource,
    auth: &dyn AuthStore,
    runtime: &Store,
    practice_id: &str,
) -> Result<ScheduleSyncOutcome, ScheduleError> {
    let clock = auth
        .practice_clock(practice_id)
        .await
        .map_err(ScheduleError::AuthStore)?;
    let read_at = clock.now(now()) as i64;
    runtime
        .seed_schedule_links_from_pulls(auth, practice_id, calendar.source_id(), read_at)
        .await?;
    runtime
        .sync_practice_schedule(auth, practice_id, calendar, read_at)
        .await
}

/// Request extension holding the practice's configured [`ScheduleSource`] for
/// `POST /director/schedule/sync`.
#[derive(Clone)]
pub struct ConfiguredScheduleSource(pub Arc<dyn ScheduleSource>);

/// Builds an Axum [`axum::Extension`] layer attaching `source` for schedule sync routes.
pub fn schedule_source_layer(
    source: Arc<dyn ScheduleSource>,
) -> axum::Extension<ConfiguredScheduleSource> {
    axum::Extension(ConfiguredScheduleSource(source))
}

/// Operator route `POST /director/schedule/sync`: requires an active admin
/// account in the signed-in practice, seeds schedule patient links from existing
/// roster pulls, and syncs the configured schedule source.
pub(crate) async fn sync_route(
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let extensions = request.extensions();
    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let is_admin = auth_store
        .account_by_identity(&identity)
        .await
        .is_some_and(|account| account.active && account.admin);
    if !is_admin {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "detail": "Practice admin access is required to sync the schedule." })),
        )
            .into_response();
    }
    let Some(runtime_store) = extensions.get::<Store>().cloned() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Runtime schedule store is not configured." })),
        )
            .into_response();
    };
    let Some(ConfiguredScheduleSource(source)) =
        extensions.get::<ConfiguredScheduleSource>().cloned()
    else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "No schedule source is configured for this practice." })),
        )
            .into_response();
    };

    match sync_synthetic_schedule(
        &*source,
        &*auth_store,
        &runtime_store,
        &identity.practice_id,
    )
    .await
    {
        Ok(ScheduleSyncOutcome::Applied(summary)) => {
            (StatusCode::OK, Json(sync_applied_json(&summary))).into_response()
        }
        Ok(ScheduleSyncOutcome::FailedRead {
            standing,
            retained_appointments,
        }) => (
            StatusCode::BAD_GATEWAY,
            Json(json!({
                "outcome": "failed_read",
                "retained_appointments": retained_appointments,
                "source_state": source_state_json(&standing),
            })),
        )
            .into_response(),
        Err(error) => {
            tracing::error!(%error, "practice schedule sync failed");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({ "detail": "Practice schedule sync could not complete." })),
            )
                .into_response()
        }
    }
}

/// Serves `GET /practice-day` from [`Store::practice_schedule`] when Postgres
/// runtime state is attached, or reports `"absent"` when no schedule has been read.
pub(crate) async fn practice_day(
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let extensions = request.extensions();
    let Ok(current_time) = connected.practice_now(extensions).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let date = crate::practice_date(&current_time).to_owned();
    let Some(runtime_store) = extensions.get::<Store>().cloned() else {
        return Json(absent_day_json(&date, &current_time)).into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    let Ok(schedule) = runtime_store.practice_schedule(&identity.practice_id).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    if matches!(schedule.standing, ScheduleStanding::Absent) {
        return Json(absent_day_json(&date, &current_time)).into_response();
    }

    let auth_store = connected.auth_store(extensions);
    let Ok(clock) = auth_store.practice_clock(&identity.practice_id).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let current_time_unix = clock.now(now()) as i64;
    let panel = auth_store.panel(&identity).await;
    let panel_set: BTreeSet<&str> = panel.iter().map(String::as_str).collect();

    let Ok(preparations) = runtime_store
        .visit_preparations_for_practice(&identity.practice_id, auth_store.as_ref())
        .await
    else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let prep_by_slot: BTreeMap<(String, String, i64), VisitPreparationRecord> = preparations
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

    let matched_workspaces: Vec<String> = schedule
        .appointments
        .iter()
        .filter_map(|appointment| match &appointment.patient_match {
            PatientMatch::Matched { workspace_id, .. }
                if panel_set.contains(workspace_id.as_str()) =>
            {
                Some(workspace_id.clone())
            }
            _ => None,
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    let directory = connected.patients(extensions);
    let Ok(patients) = patients::read_patients(&*directory, &matched_workspaces).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let display_names: BTreeMap<String, String> = patients
        .into_iter()
        .filter_map(|patient| {
            let row = patient.list_row();
            let ws = row.get("patient_workspace_id")?.as_str()?.to_owned();
            let name = row.get("display_name")?.as_str()?.to_owned();
            Some((ws, name))
        })
        .collect();

    let version = match &schedule.standing {
        ScheduleStanding::Usable {
            schedule_version, ..
        } => *schedule_version,
        ScheduleStanding::Unavailable { .. } | ScheduleStanding::Absent => 0,
    };

    let visits: Vec<Value> = if matches!(schedule.standing, ScheduleStanding::Usable { .. }) {
        schedule
            .appointments
            .iter()
            .filter_map(|appointment| {
                visit_json(
                    appointment,
                    &panel_set,
                    &display_names,
                    &prep_by_slot,
                    current_time_unix,
                )
            })
            .collect()
    } else {
        Vec::new()
    };

    Json(json!({
        "date": date,
        "time_zone": PRACTICE_TIME_ZONE,
        "current_time": current_time,
        "version": version,
        "source_state": source_state_json(&schedule.standing),
        "featured_encounter": Value::Null,
        "visits": visits,
    }))
    .into_response()
}

fn absent_day_json(date: &str, current_time: &str) -> Value {
    json!({
        "date": date,
        "time_zone": PRACTICE_TIME_ZONE,
        "current_time": current_time,
        "source_state": { "state": "absent" },
        "visits": [],
    })
}

fn sync_applied_json(summary: &local_product::runtime::schedule::ScheduleSyncSummary) -> Value {
    json!({
        "outcome": "applied",
        "saved_appointments": summary.saved_appointments,
        "changed_appointments": summary.changed_appointments,
        "matched_appointments": summary.matched_appointments,
        "unmatched_appointments": summary.unmatched_appointments,
        "source_state": source_state_json(&summary.standing),
    })
}

fn source_state_json(standing: &ScheduleStanding) -> Value {
    match standing {
        ScheduleStanding::Absent => json!({ "state": "absent" }),
        ScheduleStanding::Unavailable {
            source_id,
            source_label,
            failure,
        } => json!({
            "state": "unavailable",
            "source_code": source_id.as_str(),
            "source_label": source_label,
            "last_error": {
                "failed_at": format_unix_rfc3339(failure.failed_at),
                "reason": failure.reason,
            },
        }),
        ScheduleStanding::Usable {
            source_id,
            source_label,
            as_of,
            freshness,
            ..
        } => {
            let as_of_iso = format_unix_rfc3339(*as_of);
            match freshness {
                ScheduleFreshness::Current => json!({
                    "state": "usable",
                    "source_code": source_id.as_str(),
                    "source_label": source_label,
                    "snapshot_as_of": as_of_iso,
                    "synchronized_at": as_of_iso,
                    "freshness": "current",
                    "partial": false,
                }),
                ScheduleFreshness::Partial {
                    failures,
                    last_complete_as_of,
                } => json!({
                    "state": "usable",
                    "source_code": source_id.as_str(),
                    "source_label": source_label,
                    "snapshot_as_of": as_of_iso,
                    "synchronized_at": as_of_iso,
                    "freshness": "partial",
                    "partial": true,
                    "last_complete_as_of": last_complete_as_of.map(format_unix_rfc3339),
                    "failures": failures.as_slice(),
                }),
                ScheduleFreshness::RefreshFailed { failure } => json!({
                    "state": "usable",
                    "source_code": source_id.as_str(),
                    "source_label": source_label,
                    "snapshot_as_of": as_of_iso,
                    "synchronized_at": as_of_iso,
                    "freshness": "refresh_failed",
                    "partial": false,
                    "refresh_status": "failed",
                    "last_error": {
                        "failed_at": format_unix_rfc3339(failure.failed_at),
                        "reason": failure.reason,
                    },
                }),
            }
        }
    }
}

fn visit_json(
    appointment: &SavedAppointment,
    panel_set: &BTreeSet<&str>,
    display_names: &BTreeMap<String, String>,
    prep_by_slot: &BTreeMap<(String, String, i64), VisitPreparationRecord>,
    current_time_unix: i64,
) -> Option<Value> {
    let encounter = json!({
        "source_code": appointment.source_id.as_str(),
        "native_event_id": appointment.appointment_id.as_str(),
        "source_event_revision": appointment.source_revision,
        "saved_version": appointment.saved_version,
    });
    let as_of_iso = format_unix_rfc3339(appointment.as_of);

    match &appointment.patient_match {
        PatientMatch::Matched {
            workspace_id,
            source_patient_id,
        } => {
            if !panel_set.contains(workspace_id.as_str()) {
                return None;
            }
            let slot_key = (
                appointment.source_id.as_str().to_owned(),
                appointment.appointment_id.as_str().to_owned(),
                appointment.window.starts_at_unix(),
            );
            let preparation = matched_visit_preparation_json(
                appointment,
                prep_by_slot.get(&slot_key),
                current_time_unix,
            );
            Some(json!({
                "encounter": encounter,
                "starts_at": appointment.window.starts_at(),
                "ends_at": appointment.window.ends_at(),
                "status": appointment.status.as_str(),
                "source_status": appointment.source_status,
                "label": appointment.label,
                "as_of": as_of_iso,
                "source_patient_id": source_patient_id.as_str(),
                "patient_workspace_id": workspace_id,
                "patient_display_name": display_names.get(workspace_id),
                "patient_time_zone": PRACTICE_TIME_ZONE,
                "preparation": preparation,
            }))
        }
        PatientMatch::Unmatched { source_patient_id } => Some(json!({
            "encounter": encounter,
            "starts_at": appointment.window.starts_at(),
            "ends_at": appointment.window.ends_at(),
            "status": appointment.status.as_str(),
            "source_status": appointment.source_status,
            "label": appointment.label,
            "as_of": as_of_iso,
            "unmatched_source_patient_id": source_patient_id.as_ref().map(SourcePatientId::as_str),
            "patient_workspace_id": Value::Null,
            "patient_display_name": Value::Null,
            "patient_time_zone": Value::Null,
            "preparation": { "state": "unavailable" },
        })),
    }
}

fn matched_visit_preparation_json(
    appointment: &SavedAppointment,
    prep: Option<&VisitPreparationRecord>,
    current_time_unix: i64,
) -> Value {
    let default_state = || {
        if visit_can_start_preparation(
            appointment.status,
            appointment.window.ends_at_unix(),
            current_time_unix,
        ) {
            json!({ "state": "scheduled" })
        } else {
            json!({ "state": "pending" })
        }
    };
    let Some(prep) = prep else {
        return default_state();
    };
    match &prep.state {
        VisitPreparationState::Ready {
            briefing_id,
            prepared_at,
            visit_card_orientation,
            reason: _,
        } => json!({
            "state": "prepared",
            "briefing_id": briefing_id,
            "prepared_at": format_unix_rfc3339(*prepared_at),
            "visit_card_orientation": visit_card_orientation,
        }),
        VisitPreparationState::Preparing {
            job_id,
            started_at: _,
            reason: _,
        } => json!({
            "state": "preparing",
            "run_id": job_id,
        }),
        VisitPreparationState::CouldNotFinish {
            job_id,
            failed_at,
            reason,
        } => {
            let occurred_at = format_unix_rfc3339(*failed_at);
            json!({
                "state": "could_not_prepare",
                "run_id": job_id,
                "occurred_at": occurred_at,
                "reason": reason,
                "last_attempt": {
                    "run_id": job_id,
                    "outcome": "failed",
                    "occurred_at": occurred_at,
                },
            })
        }
        VisitPreparationState::Skipped { reason } => json!({
            "state": "pending",
            "reason": reason,
        }),
        VisitPreparationState::EndedVisitChanged { ended_at, reason } => {
            let occurred_at = format_unix_rfc3339(*ended_at);
            json!({
                "state": "pending",
                "occurred_at": occurred_at,
                "reason": reason,
                "last_attempt": {
                    "outcome": "cancelled",
                    "occurred_at": occurred_at,
                },
            })
        }
        VisitPreparationState::Scheduled { .. } | VisitPreparationState::Deferred { .. } => {
            default_state()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn fhir_appointment(
        id: &str,
        status: &str,
        description: &str,
        start: &str,
        end: &str,
        patient_mrn: Option<&str>,
    ) -> String {
        let participant = match patient_mrn {
            Some(mrn) => {
                format!(r#"[{{"actor":{{"reference":"Patient/{mrn}"}},"status":"accepted"}}]"#)
            }
            None => "[]".to_owned(),
        };
        format!(
            r#"{{"resourceType":"Appointment","id":"{id}","meta":{{"versionId":"1","lastUpdated":"2026-08-27T00:00:00+00:00"}},"status":"{status}","serviceType":[{{"text":"Follow-up"}}],"appointmentType":{{"text":"Follow-up visit"}},"description":"{description}","start":"{start}","end":"{end}","minutesDuration":30,"participant":{participant}}}"#
        )
    }

    fn temp_bank(tag: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "coda-synthetic-calendar-{tag}-{}-{nanos:x}",
            std::process::id()
        ));
        fs::create_dir_all(dir.join("calendar")).unwrap();
        dir
    }

    #[tokio::test]
    async fn synthetic_bank_calendar_parses_appointments_and_names_corrupt_lines() {
        let bank = temp_bank("unit");
        let calendar_path = bank.join("calendar/Appointment.ndjson");
        let valid_matched = fhir_appointment(
            "appt-1",
            "booked",
            "Medication follow-up",
            "2026-08-27T17:00:00+00:00",
            "2026-08-27T17:30:00+00:00",
            Some("MRN-100"),
        );
        let valid_unmatched = fhir_appointment(
            "appt-2",
            "booked",
            "New patient visit",
            "2026-08-27T18:00:00+00:00",
            "2026-08-27T18:30:00+00:00",
            None,
        );
        let corrupt_with_id = r#"{"resourceType":"Appointment","id":"appt-bad-status","status":"mystery","start":"2026-08-27T19:00:00+00:00","end":"2026-08-27T19:30:00+00:00","description":"Check"}"#;

        fs::write(
            &calendar_path,
            format!("{valid_matched}\n{corrupt_with_id}\nnot-json-at-all\n{valid_unmatched}\n"),
        )
        .unwrap();

        let adapter = SyntheticBankCalendar::open(&bank);
        let delivery = adapter.read_schedule().await.expect("partial delivery");
        match delivery {
            ScheduleDelivery::Partial {
                appointments,
                failures,
            } => {
                assert_eq!(appointments.len(), 2);
                assert_eq!(appointments[0].appointment_id().as_str(), "appt-1");
                assert_eq!(appointments[0].status(), AppointmentStatus::Scheduled);
                assert_eq!(appointments[0].source_status(), "booked");
                assert_eq!(
                    appointments[0]
                        .source_patient_id()
                        .map(SourcePatientId::as_str),
                    Some("MRN-100")
                );
                assert_eq!(appointments[1].appointment_id().as_str(), "appt-2");
                assert!(appointments[1].source_patient_id().is_none());

                assert_eq!(failures.len(), 2);
                assert_eq!(
                    failures.as_slice()[0].target(),
                    "Appointment/appt-bad-status"
                );
                assert!(failures.as_slice()[0].reason().contains("mystery"));
                assert_eq!(failures.as_slice()[1].target(), "Appointment.ndjson:3");
            }
            ScheduleDelivery::Complete { .. } => panic!("expected partial delivery"),
        }

        let _ = fs::remove_dir_all(&bank);
    }

    #[test]
    fn from_env_prefers_explicit_calendar_and_ignores_chart_only_banks() {
        let chart_only = temp_bank("chart-only");
        fs::remove_dir_all(chart_only.join("calendar")).unwrap();
        fs::create_dir_all(chart_only.join("patients")).unwrap();
        assert!(
            SyntheticBankCalendar::from_env(|key| {
                (key == "PHYSICIAN_SYNTHETIC_CHARTS")
                    .then(|| chart_only.to_string_lossy().into_owned())
            })
            .is_none()
        );

        let full_bank = temp_bank("full-bank");
        let calendar_file = full_bank.join("calendar/Appointment.ndjson");
        fs::write(&calendar_file, "").unwrap();
        let resolved = SyntheticBankCalendar::from_env(|key| {
            (key == "PHYSICIAN_SYNTHETIC_CHARTS").then(|| full_bank.to_string_lossy().into_owned())
        })
        .expect("resolved from built bank");
        assert_eq!(resolved.path(), calendar_file.as_path());

        let missing_explicit = full_bank.join("missing-calendar.ndjson");
        let explicit = SyntheticBankCalendar::from_env(|key| {
            (key == CALENDAR_ENV).then(|| missing_explicit.to_string_lossy().into_owned())
        })
        .expect("explicit calendar path preserved even when file is missing");
        assert_eq!(explicit.path(), missing_explicit.as_path());

        let _ = fs::remove_dir_all(&chart_only);
        let _ = fs::remove_dir_all(&full_bank);
    }
}
