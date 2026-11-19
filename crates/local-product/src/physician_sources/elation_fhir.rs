//! Elation / FHIR R4 read-only EHR adapter for patient rosters, appointment
//! schedules (`ScheduleSource`), and patient chart refreshes (`PatientSourceRefresh`).
//!
//! Single-owner FHIR R4 `Appointment` parsing ([`parse_fhir_appointment`]) is
//! shared across live Elation FHIR endpoints and synthetic FHIR banks so status,
//! participant, window, and label rules stay stated once.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use serde_json::{Value, json};
use tokio::sync::RwLock;

use super::adapter::{RecordFilingError, file_fhir_record, validate_fhir_patient_binding};
use super::{MedicalRecordNumber, RecordDate, SourcePatient, SourceRevision};
use crate::runtime::Store;
use crate::runtime::refresh::{
    PatientSourceRefresh, SourceCheckOutcome, SourceId as RefreshSourceId,
    SourceRefreshAdapterError,
};
use crate::runtime::schedule::{
    AppointmentId, AppointmentStatus, AppointmentWindow, ScheduleDelivery, ScheduleDomainError,
    ScheduleReadIssue, ScheduleSource, ScheduleSourceError, SourceAppointment,
    SourceAppointmentParts, SourceId as ScheduleSourceId, SourcePatientId,
};

/// Canonical source identifier for Elation FHIR R4 connections.
pub const SOURCE_ID: &str = "elation-fhir";

/// Default physician-facing source label for Elation FHIR R4 connections.
pub const SOURCE_LABEL: &str = "Elation / FHIR";

/// Environment variable naming the Elation FHIR R4 base URL.
pub const BASE_URL_ENV: &str = "PHYSICIAN_ELATION_FHIR_BASE_URL";

/// Environment variable carrying a static bearer token for Elation FHIR R4.
pub const BEARER_TOKEN_ENV: &str = "PHYSICIAN_ELATION_FHIR_BEARER_TOKEN";

/// Environment variable naming the OAuth2 token endpoint URL for Elation FHIR R4.
pub const TOKEN_URL_ENV: &str = "PHYSICIAN_ELATION_FHIR_TOKEN_URL";
/// Environment variable naming the OAuth2 client identifier for Elation FHIR R4.
pub const CLIENT_ID_ENV: &str = "PHYSICIAN_ELATION_FHIR_CLIENT_ID";
/// Environment variable naming the OAuth2 client secret for Elation FHIR R4.
pub const CLIENT_SECRET_ENV: &str = "PHYSICIAN_ELATION_FHIR_CLIENT_SECRET";
/// Environment variable naming the optional OAuth2 scope for Elation FHIR R4.
pub const SCOPE_ENV: &str = "PHYSICIAN_ELATION_FHIR_SCOPE";

/// Maximum FHIR pagination pages followed in one read, preventing unbounded loops.
const MAX_FHIR_PAGES: usize = 25;

/// HTTP timeout per FHIR request.
const FHIR_HTTP_TIMEOUT: Duration = Duration::from_secs(20);

/// Error parsing a single FHIR R4 `Appointment` JSON resource.
#[derive(Debug, thiserror::Error)]
pub enum FhirAppointmentParseError {
    /// Raw JSON could not be parsed.
    #[error("invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    /// Resource was not an `Appointment`.
    #[error("expected resourceType \"Appointment\", got {0:?}")]
    WrongResourceType(Option<String>),
    /// Required string field was absent or blank.
    #[error("Appointment is missing required string field \"{0}\"")]
    MissingField(&'static str),
    /// Appointment status token was not a recognized FHIR R4 status.
    #[error("unsupported Appointment status {0:?}")]
    UnsupportedStatus(String),
    /// Schedule domain invariant failed (window, identifier, etc.).
    #[error("{0}")]
    Domain(#[from] ScheduleDomainError),
}

/// Parses one FHIR R4 `Appointment` JSON string into a validated [`SourceAppointment`]
/// and an optional non-fatal [`ScheduleReadIssue`] (e.g. multiple distinct Patient participants).
///
/// Single owner shared by [`ElationFhirClient`] and synthetic FHIR calendar readers.
pub fn parse_fhir_appointment(
    line: &str,
) -> Result<(SourceAppointment, Option<ScheduleReadIssue>), FhirAppointmentParseError> {
    let resource: Value = serde_json::from_str(line)?;
    parse_fhir_appointment_value(&resource, line)
}

/// Parses a pre-parsed FHIR R4 `Appointment` [`Value`] while retaining `raw_content` untouched.
pub fn parse_fhir_appointment_value(
    resource: &Value,
    raw_content: &str,
) -> Result<(SourceAppointment, Option<ScheduleReadIssue>), FhirAppointmentParseError> {
    let resource_type = resource.get("resourceType").and_then(Value::as_str);
    if resource_type != Some("Appointment") {
        return Err(FhirAppointmentParseError::WrongResourceType(
            resource_type.map(str::to_owned),
        ));
    }
    let raw_id = resource
        .get("id")
        .and_then(Value::as_str)
        .ok_or(FhirAppointmentParseError::MissingField("id"))?;
    let appointment_id = AppointmentId::new(raw_id)?;

    let source_status = resource
        .get("status")
        .and_then(Value::as_str)
        .ok_or(FhirAppointmentParseError::MissingField("status"))?
        .trim()
        .to_owned();
    let status = parse_fhir_status(&source_status)?;

    let start = resource
        .get("start")
        .and_then(Value::as_str)
        .ok_or(FhirAppointmentParseError::MissingField("start"))?;
    let end = resource
        .get("end")
        .and_then(Value::as_str)
        .ok_or(FhirAppointmentParseError::MissingField("end"))?;
    let window = AppointmentWindow::parse(start, end)?;

    let label = extract_appointment_label(resource)
        .ok_or(FhirAppointmentParseError::MissingField("description"))?;
    let candidate_id = format!("Appointment/{appointment_id}");
    let (source_patient_id, participant_issue) =
        extract_patient_participant(resource, &candidate_id)?;
    let dates = extract_appointment_dates(resource);
    let source_revision = resource
        .get("meta")
        .and_then(|meta| meta.get("versionId"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_owned);

    let appointment = SourceAppointment::new(SourceAppointmentParts {
        appointment_id,
        source_revision,
        window,
        status,
        source_status,
        label,
        source_patient_id,
        dates,
        content: raw_content.to_owned(),
        provenance: json!({ "native_record": candidate_id }),
    })?;
    Ok((appointment, participant_issue))
}

fn parse_fhir_status(status: &str) -> Result<AppointmentStatus, FhirAppointmentParseError> {
    match status {
        "booked" | "pending" | "proposed" | "arrived" | "checked-in" | "waitlist" | "scheduled" => {
            Ok(AppointmentStatus::Scheduled)
        }
        "cancelled" | "entered-in-error" => Ok(AppointmentStatus::Cancelled),
        "fulfilled" => Ok(AppointmentStatus::Fulfilled),
        "noshow" => Ok(AppointmentStatus::NoShow),
        other => Err(FhirAppointmentParseError::UnsupportedStatus(
            other.to_owned(),
        )),
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
    target_label: &str,
) -> Result<(Option<SourcePatientId>, Option<ScheduleReadIssue>), FhirAppointmentParseError> {
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
            let issue = ScheduleReadIssue::new(
                target_label,
                "Appointment lists multiple distinct Patient participants",
            )
            .expect("non-empty issue target and reason");
            return Ok((None, Some(issue)));
        }
        matched = Some(patient_id);
    }
    Ok((matched, None))
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

/// Parses one FHIR R4 `Patient` resource into a [`SourcePatient`] for roster
/// reconciliation without rewriting clinical or demographic values.
pub fn parse_fhir_patient(resource: &Value) -> Result<SourcePatient, RecordFilingError> {
    let resource_type = resource.get("resourceType").and_then(Value::as_str);
    if resource_type != Some("Patient") {
        return Err(RecordFilingError::MissingField("resourceType=Patient"));
    }
    let id = resource
        .get("id")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .ok_or(RecordFilingError::MissingField("id"))?;

    let display_name =
        extract_fhir_human_name(resource).ok_or(RecordFilingError::MissingField("name"))?;
    let birth_date = resource
        .get("birthDate")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_owned);
    let sex_or_gender = resource
        .get("gender")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_owned);
    let medical_record_numbers = extract_fhir_mrns(resource);

    Ok(SourcePatient {
        source_patient_id: id.to_owned(),
        display_name,
        birth_date,
        sex_or_gender,
        medical_record_numbers,
    })
}

fn extract_fhir_human_name(resource: &Value) -> Option<String> {
    let names = resource.get("name").and_then(Value::as_array)?;
    for entry in names {
        if let Some(text) = entry
            .get("text")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            return Some(text.to_owned());
        }
        let mut parts = Vec::new();
        if let Some(given) = entry.get("given").and_then(Value::as_array) {
            for token in given {
                if let Some(s) = token.as_str().map(str::trim).filter(|v| !v.is_empty()) {
                    parts.push(s);
                }
            }
        }
        if let Some(family) = entry
            .get("family")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            parts.push(family);
        }
        if !parts.is_empty() {
            return Some(parts.join(" "));
        }
    }
    None
}

fn extract_fhir_mrns(resource: &Value) -> Vec<MedicalRecordNumber> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for identifier in resource
        .get("identifier")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let Some(value) = identifier
            .get("value")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|v| !v.is_empty())
        else {
            continue;
        };
        let is_mrn = identifier
            .get("type")
            .and_then(|t| t.get("coding"))
            .and_then(Value::as_array)
            .is_some_and(|codings| {
                codings
                    .iter()
                    .any(|c| c.get("code").and_then(Value::as_str) == Some("MR"))
            })
            || identifier
                .get("system")
                .and_then(Value::as_str)
                .is_some_and(|s| s.to_ascii_lowercase().contains("mrn"));
        if !is_mrn {
            continue;
        }
        let system = identifier
            .get("system")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .unwrap_or("urn:elation:mrn")
            .to_owned();
        if seen.insert((system.clone(), value.to_owned())) {
            out.push(MedicalRecordNumber {
                value: value.to_owned(),
                system,
            });
        }
    }
    out
}

/// Authentication method for an Elation FHIR R4 endpoint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ElationFhirAuth {
    /// Static bearer access token supplied via environment/Doppler.
    Bearer(String),
    /// OAuth2 client-credentials token exchange.
    ClientCredentials {
        /// Token endpoint URL (`http` or `https`).
        token_url: reqwest::Url,
        /// OAuth2 client identifier.
        client_id: String,
        /// OAuth2 client secret.
        client_secret: String,
        /// Optional OAuth2 scope list.
        scope: Option<String>,
    },
}

/// Validated configuration for an Elation FHIR R4 practice connection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElationFhirConfig {
    /// Practice identifier this connection belongs to (optional when bound globally).
    pub practice_id: Option<String>,
    /// Validated FHIR R4 base URL without trailing slash.
    pub base_url: reqwest::Url,
    /// Authentication strategy.
    pub auth: ElationFhirAuth,
    /// Stable source identifier (defaults to `elation-fhir`).
    pub source_id: ScheduleSourceId,
    /// Physician-facing source label (defaults to `Elation / FHIR`).
    pub source_label: String,
}

/// Configuration validation errors for [`ElationFhirConfig`].
#[derive(Debug, thiserror::Error)]
pub enum ElationFhirConfigError {
    /// Base or token URL could not be parsed.
    #[error("invalid Elation FHIR URL `{field}`: {detail}")]
    InvalidUrl {
        /// Environment or config field name.
        field: &'static str,
        /// Underlying URL parse detail.
        detail: String,
    },
    /// URL scheme was not `http` or `https`.
    #[error("Elation FHIR URL `{field}` must use http or https")]
    UnsupportedScheme {
        /// Environment or config field name.
        field: &'static str,
    },
    /// Required credential setting was missing when base URL was configured.
    #[error(
        "Elation FHIR authentication missing: set {BEARER_TOKEN_ENV} or ({TOKEN_URL_ENV}, {CLIENT_ID_ENV}, {CLIENT_SECRET_ENV})"
    )]
    MissingAuth,
}

impl ElationFhirConfig {
    /// Builds a validated configuration with a bearer token.
    pub fn bearer(
        base_url: &str,
        token: impl Into<String>,
    ) -> Result<Self, ElationFhirConfigError> {
        let parsed = parse_http_url("base_url", base_url)?;
        let token = token.into();
        if token.trim().is_empty() {
            return Err(ElationFhirConfigError::MissingAuth);
        }
        Ok(Self {
            practice_id: None,
            base_url: parsed,
            auth: ElationFhirAuth::Bearer(token.trim().to_owned()),
            source_id: ScheduleSourceId::new(SOURCE_ID).expect("static elation-fhir source_id"),
            source_label: SOURCE_LABEL.to_owned(),
        })
    }

    /// Reads optional Elation FHIR configuration from environment variables.
    /// Returns `Ok(None)` when `PHYSICIAN_ELATION_FHIR_BASE_URL` is unset or empty.
    pub fn from_env(
        get: impl Fn(&str) -> Option<String>,
    ) -> Result<Option<Self>, ElationFhirConfigError> {
        let Some(raw_base) = get(BASE_URL_ENV)
            .map(|v| v.trim().to_owned())
            .filter(|v| !v.is_empty())
        else {
            return Ok(None);
        };
        let base_url = parse_http_url(BASE_URL_ENV, &raw_base)?;
        let auth = if let Some(token) = get(BEARER_TOKEN_ENV)
            .map(|v| v.trim().to_owned())
            .filter(|v| !v.is_empty())
        {
            ElationFhirAuth::Bearer(token)
        } else if let (Some(raw_token_url), Some(client_id), Some(client_secret)) = (
            get(TOKEN_URL_ENV)
                .map(|v| v.trim().to_owned())
                .filter(|v| !v.is_empty()),
            get(CLIENT_ID_ENV)
                .map(|v| v.trim().to_owned())
                .filter(|v| !v.is_empty()),
            get(CLIENT_SECRET_ENV)
                .map(|v| v.trim().to_owned())
                .filter(|v| !v.is_empty()),
        ) {
            ElationFhirAuth::ClientCredentials {
                token_url: parse_http_url(TOKEN_URL_ENV, &raw_token_url)?,
                client_id,
                client_secret,
                scope: get(SCOPE_ENV)
                    .map(|v| v.trim().to_owned())
                    .filter(|v| !v.is_empty()),
            }
        } else {
            return Err(ElationFhirConfigError::MissingAuth);
        };
        let practice_id = get("PHYSICIAN_ELATION_FHIR_PRACTICE_ID")
            .map(|v| v.trim().to_owned())
            .filter(|v| !v.is_empty());
        Ok(Some(Self {
            practice_id,
            base_url,
            auth,
            source_id: ScheduleSourceId::new(SOURCE_ID).expect("static elation-fhir source_id"),
            source_label: SOURCE_LABEL.to_owned(),
        }))
    }
}

fn parse_http_url(field: &'static str, raw: &str) -> Result<reqwest::Url, ElationFhirConfigError> {
    let trimmed = raw.trim().trim_end_matches('/');
    let url = reqwest::Url::parse(trimmed).map_err(|err| ElationFhirConfigError::InvalidUrl {
        field,
        detail: err.to_string(),
    })?;
    if !matches!(url.scheme(), "http" | "https") {
        return Err(ElationFhirConfigError::UnsupportedScheme { field });
    }
    Ok(url)
}

/// Result of reading a practice roster from Elation FHIR R4 (`GET /Patient`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElationRosterDelivery {
    /// Successfully parsed roster patients.
    pub patients: Vec<SourcePatient>,
    /// Per-item parse failures isolated so one bad patient never blocks the rest.
    pub failures: Vec<String>,
}

/// Errors produced when communicating with an Elation FHIR R4 server.
#[derive(Debug, thiserror::Error)]
pub enum ElationFhirError {
    /// HTTP transport error.
    #[error("Elation FHIR HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    /// Non-success HTTP response status.
    #[error("Elation FHIR returned HTTP {status}: {detail}")]
    Status {
        /// HTTP status code.
        status: u16,
        /// Bounded response detail.
        detail: String,
    },
    /// Invalid JSON or non-Bundle FHIR payload.
    #[error("Elation FHIR payload invalid: {0}")]
    InvalidPayload(String),
    /// OAuth2 client-credentials token exchange failed.
    #[error("Elation FHIR token exchange failed: {0}")]
    TokenExchange(String),
}

#[derive(Clone, Debug)]
struct CachedToken {
    access_token: String,
    expires_at_unix: i64,
}

/// Live HTTP client for Elation / FHIR R4 endpoints implementing roster reads,
/// [`ScheduleSource`], and [`PatientSourceRefresh`].
#[derive(Clone)]
pub struct ElationFhirClient {
    config: ElationFhirConfig,
    http: reqwest::Client,
    store: Option<Store>,
    cached_token: Arc<RwLock<Option<CachedToken>>>,
}

impl ElationFhirClient {
    /// Constructs an [`ElationFhirClient`] for `config`.
    pub fn new(config: ElationFhirConfig) -> Result<Self, ElationFhirError> {
        let http = reqwest::Client::builder()
            .timeout(FHIR_HTTP_TIMEOUT)
            .build()?;
        Ok(Self {
            config,
            http,
            store: None,
            cached_token: Arc::new(RwLock::new(None)),
        })
    }

    /// Attaches the runtime [`Store`] so [`PatientSourceRefresh`] can resolve
    /// each workspace's Elation `source_patient_id` from saved pulls/links.
    pub fn with_store(mut self, store: Store) -> Self {
        self.store = Some(store);
        self
    }

    /// Returns this client's validated configuration.
    pub fn config(&self) -> &ElationFhirConfig {
        &self.config
    }

    async fn bearer_token(&self) -> Result<String, ElationFhirError> {
        match &self.config.auth {
            ElationFhirAuth::Bearer(token) => Ok(token.clone()),
            ElationFhirAuth::ClientCredentials {
                token_url,
                client_id,
                client_secret,
                scope,
            } => {
                let now_unix = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_or(0, |d| d.as_secs() as i64);
                if let Some(cached) = self.cached_token.read().await.as_ref()
                    && cached.expires_at_unix > now_unix + 30
                {
                    return Ok(cached.access_token.clone());
                }
                let mut form = vec![
                    ("grant_type", "client_credentials"),
                    ("client_id", client_id.as_str()),
                    ("client_secret", client_secret.as_str()),
                ];
                if let Some(scope_val) = scope.as_deref() {
                    form.push(("scope", scope_val));
                }
                let resp = self.http.post(token_url.clone()).form(&form).send().await?;
                if !resp.status().is_success() {
                    let status = resp.status().as_u16();
                    let detail = resp.text().await.unwrap_or_default();
                    return Err(ElationFhirError::TokenExchange(format!(
                        "HTTP {status}: {}",
                        detail.chars().take(200).collect::<String>()
                    )));
                }
                let body: Value = resp.json().await?;
                let access_token = body
                    .get("access_token")
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .ok_or_else(|| {
                        ElationFhirError::TokenExchange("response missing access_token".into())
                    })?
                    .to_owned();
                let expires_in = body
                    .get("expires_in")
                    .and_then(Value::as_i64)
                    .unwrap_or(300)
                    .max(60);
                *self.cached_token.write().await = Some(CachedToken {
                    access_token: access_token.clone(),
                    expires_at_unix: now_unix + expires_in,
                });
                Ok(access_token)
            }
        }
    }

    async fn fetch_bundle_resources(
        &self,
        initial_path: &str,
    ) -> Result<Vec<Value>, ElationFhirError> {
        let token = self.bearer_token().await?;
        let mut next_url = format!(
            "{}/{initial_path}",
            self.config.base_url.as_str().trim_end_matches('/')
        );
        let mut resources = Vec::new();
        for _ in 0..MAX_FHIR_PAGES {
            let resp = self
                .http
                .get(&next_url)
                .bearer_auth(&token)
                .header(
                    reqwest::header::ACCEPT,
                    "application/fhir+json, application/json",
                )
                .send()
                .await?;
            if !resp.status().is_success() {
                let status = resp.status().as_u16();
                let detail = resp.text().await.unwrap_or_default();
                return Err(ElationFhirError::Status {
                    status,
                    detail: detail.chars().take(240).collect(),
                });
            }
            let bundle: Value = resp.json().await?;
            if bundle.get("resourceType").and_then(Value::as_str) != Some("Bundle") {
                return Err(ElationFhirError::InvalidPayload(
                    "expected FHIR Bundle response".into(),
                ));
            }
            for entry in bundle
                .get("entry")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                if let Some(res) = entry.get("resource") {
                    resources.push(res.clone());
                }
            }
            let next_link = bundle
                .get("link")
                .and_then(Value::as_array)
                .and_then(|links| {
                    links.iter().find_map(|link| {
                        (link.get("relation").and_then(Value::as_str) == Some("next"))
                            .then(|| link.get("url").and_then(Value::as_str))
                            .flatten()
                    })
                });
            match next_link {
                Some(url) if !url.trim().is_empty() => next_url = url.trim().to_owned(),
                _ => break,
            }
        }
        Ok(resources)
    }

    /// Reads the practice's patient roster from `GET {base_url}/Patient`.
    /// Malformed individual Patient entries are isolated in `failures`.
    pub async fn read_roster(&self) -> Result<ElationRosterDelivery, ElationFhirError> {
        let resources = self.fetch_bundle_resources("Patient").await?;
        let mut patients = Vec::new();
        let mut seen_ids = BTreeSet::new();
        let mut failures = Vec::new();
        for (idx, resource) in resources.iter().enumerate() {
            match parse_fhir_patient(resource) {
                Ok(patient) => {
                    if seen_ids.insert(patient.source_patient_id.clone()) {
                        patients.push(patient);
                    }
                }
                Err(err) => {
                    let label = resource
                        .get("id")
                        .and_then(Value::as_str)
                        .map(|id| format!("Patient/{id}"))
                        .unwrap_or_else(|| format!("Patient[{idx}]"));
                    failures.push(format!("{label}: {err}"));
                }
            }
        }
        Ok(ElationRosterDelivery { patients, failures })
    }

    async fn resolve_source_patient_id(
        &self,
        practice: &str,
        workspace: &str,
    ) -> Result<Option<String>, SourceRefreshAdapterError> {
        let Some(store) = &self.store else {
            return Ok(None);
        };
        let pulls = store
            .source_pulls(std::slice::from_ref(&workspace.to_owned()))
            .await
            .map_err(|err| SourceRefreshAdapterError::Interrupted {
                source_id: RefreshSourceId::new(self.config.source_id.as_str())
                    .expect("valid elation source id"),
                reason: err.to_string(),
            })?;
        if let Some(found) = pulls
            .get(workspace)
            .into_iter()
            .flatten()
            .find(|pull| pull.source_id == self.config.source_id.as_str())
        {
            return Ok(Some(found.patient.source_patient_id.clone()));
        }
        let links = store
            .schedule_patient_links(practice, &self.config.source_id)
            .await
            .map_err(|err| SourceRefreshAdapterError::Interrupted {
                source_id: RefreshSourceId::new(self.config.source_id.as_str())
                    .expect("valid elation source id"),
                reason: err.to_string(),
            })?;
        Ok(links
            .into_iter()
            .find(|link| link.workspace_id == workspace)
            .map(|link| link.source_patient_id.as_str().to_owned()))
    }
}

#[async_trait]
impl ScheduleSource for ElationFhirClient {
    fn source_id(&self) -> &ScheduleSourceId {
        &self.config.source_id
    }

    fn source_label(&self) -> &str {
        &self.config.source_label
    }

    async fn read_schedule(&self) -> Result<ScheduleDelivery, ScheduleSourceError> {
        let resources = self
            .fetch_bundle_resources("Appointment")
            .await
            .map_err(|err| ScheduleSourceError::Unavailable {
                source_id: self.config.source_id.clone(),
                reason: err.to_string(),
            })?;

        let mut appointments = Vec::new();
        let mut seen_digests: BTreeMap<AppointmentId, String> = BTreeMap::new();
        let mut issues = Vec::new();

        for (idx, resource) in resources.iter().enumerate() {
            let raw = serde_json::to_string(resource).unwrap_or_default();
            match parse_fhir_appointment_value(resource, &raw) {
                Ok((appointment, participant_issue)) => {
                    if let Some(issue) = participant_issue {
                        issues.push(issue);
                    }
                    let id = appointment.appointment_id().clone();
                    if let Some(prior_sha) = seen_digests.get(&id) {
                        if prior_sha != appointment.sha256() {
                            issues.push(
                                ScheduleReadIssue::new(
                                    format!("Appointment/{id}"),
                                    "duplicate appointment id with conflicting content",
                                )
                                .expect("non-empty issue"),
                            );
                        }
                        continue;
                    }
                    seen_digests.insert(id, appointment.sha256().to_owned());
                    appointments.push(appointment);
                }
                Err(err) => {
                    let target = resource
                        .get("id")
                        .and_then(Value::as_str)
                        .map(|id| format!("Appointment/{id}"))
                        .unwrap_or_else(|| format!("Appointment[{idx}]"));
                    issues.push(
                        ScheduleReadIssue::new(target, err.to_string()).expect("non-empty issue"),
                    );
                }
            }
        }

        ScheduleDelivery::from_outcomes(appointments, issues).map_err(|err| {
            ScheduleSourceError::Unavailable {
                source_id: self.config.source_id.clone(),
                reason: err.to_string(),
            }
        })
    }
}

#[async_trait]
impl PatientSourceRefresh for ElationFhirClient {
    async fn configured_sources(
        &self,
        practice: &str,
        workspace: &str,
    ) -> Result<Vec<RefreshSourceId>, SourceRefreshAdapterError> {
        if let Some(bound) = self.config.practice_id.as_deref()
            && bound != practice
        {
            return Ok(Vec::new());
        }
        if self
            .resolve_source_patient_id(practice, workspace)
            .await?
            .is_some()
        {
            Ok(vec![
                RefreshSourceId::new(self.config.source_id.as_str())
                    .expect("valid elation source id"),
            ])
        } else {
            Ok(Vec::new())
        }
    }

    async fn check_source(
        &self,
        practice: &str,
        workspace: &str,
        source_id: &RefreshSourceId,
    ) -> Result<SourceCheckOutcome, SourceRefreshAdapterError> {
        let now_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs() as i64);
        let Some(patient_id) = self.resolve_source_patient_id(practice, workspace).await? else {
            return Ok(SourceCheckOutcome::Failed {
                source_label: Some(self.config.source_label.clone()),
                checked_at: now_unix,
                reason: format!(
                    "No Elation FHIR patient mapping exists for workspace `{workspace}`."
                ),
            });
        };

        let resources = match self
            .fetch_bundle_resources(&format!("Patient/{patient_id}/$everything"))
            .await
        {
            Ok(items) => items,
            Err(err) => {
                return Ok(SourceCheckOutcome::Failed {
                    source_label: Some(self.config.source_label.clone()),
                    checked_at: now_unix,
                    reason: err.to_string(),
                });
            }
        };

        let mut patient: Option<SourcePatient> = None;
        let mut records: Vec<SourceRevision> = Vec::new();
        let mut failures: Vec<String> = Vec::new();

        for (idx, resource) in resources.iter().enumerate() {
            let rtype = resource
                .get("resourceType")
                .and_then(Value::as_str)
                .unwrap_or("Unknown");
            if rtype == "Patient" {
                match parse_fhir_patient(resource) {
                    Ok(p) if p.source_patient_id == patient_id => patient = Some(p),
                    Ok(p) => failures.push(format!(
                        "Patient/{}: patient binding mismatch (expected {patient_id})",
                        p.source_patient_id
                    )),
                    Err(err) => failures.push(format!("Patient[{idx}]: {err}")),
                }
            }
            if let Err(err) = validate_fhir_patient_binding(resource, &patient_id) {
                failures.push(format!("{rtype}[{idx}]: {err}"));
                continue;
            }
            let raw = serde_json::to_string(resource).unwrap_or_default();
            match file_fhir_record(workspace, source_id.as_str(), &raw, None) {
                Ok(revision) => records.push(revision),
                Err(err) => failures.push(format!("{rtype}[{idx}]: {err}")),
            }
        }

        let Some(patient) = patient else {
            return Ok(SourceCheckOutcome::Failed {
                source_label: Some(self.config.source_label.clone()),
                checked_at: now_unix,
                reason: format!(
                    "Elation FHIR $everything bundle for Patient/{patient_id} did not include a matching Patient resource."
                ),
            });
        };

        if failures.is_empty() {
            Ok(SourceCheckOutcome::Updated {
                source_label: self.config.source_label.clone(),
                patient,
                records,
                pulled_at: now_unix,
            })
        } else {
            Ok(SourceCheckOutcome::PartlyUpdated {
                source_label: self.config.source_label.clone(),
                patient,
                records,
                pulled_at: now_unix,
                reason: failures.join("; "),
            })
        }
    }
}
