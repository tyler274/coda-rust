//! A practice's patients as a physician reads them.
//!
//! The auth store says who belongs to a practice and whose panel they are on.
//! Each source's last pull says who the patient is and how current their
//! records are. Search, the patient list and the patient page all read from
//! here, so a patient exists on every surface or on none: a registered patient
//! no source holds reads as not found.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use crate::timestamp::format_rfc3339;
use async_trait::async_trait;
use local_product::physician_sources::{MedicalRecordNumber, SourcePatient};
use local_product::runtime::Store;
use local_product::runtime::sources::SourcePull;
use physician_auth::{AuthStore, Membership, MemoryAuthStore, StoreUnavailable};
use serde_json::{Value, json};
use std::time::{Duration, UNIX_EPOCH};

use crate::auth::{STAGING_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK, staging_panel};

/// Default fixture zone used only in unit tests that bypass practice setup.
#[cfg(test)]
pub(crate) const PRACTICE_TIME_ZONE: &str = "America/New_York";

/// The most patients one search answers with.
const SEARCH_LIMIT: usize = 50;

/// Number of patients returned per page when Find patient opens with no search text.
pub const ROSTER_PAGE_SIZE: usize = 25;

/// A malformed pagination cursor supplied to the patient roster endpoint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidRosterCursor;

impl std::fmt::Display for InvalidRosterCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid patient roster cursor")
    }
}

impl std::error::Error for InvalidRosterCursor {}

/// Keyset position in the name-ordered patient roster.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct RosterCursor {
    name_key: String,
    workspace: String,
}

impl RosterCursor {
    fn from_patient(patient: &Patient) -> Self {
        Self {
            name_key: patient.holder().display_name.to_lowercase(),
            workspace: patient.workspace.clone(),
        }
    }

    pub(crate) fn encode(&self) -> String {
        use base64::Engine as _;
        let json = serde_json::to_vec(self).expect("serialize roster cursor");
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(json)
    }

    pub(crate) fn decode(raw: &str) -> Result<Self, InvalidRosterCursor> {
        use base64::Engine as _;
        let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(raw.trim())
            .map_err(|_| InvalidRosterCursor)?;
        let cursor: Self = serde_json::from_slice(&bytes).map_err(|_| InvalidRosterCursor)?;
        if cursor.workspace.trim().is_empty() {
            return Err(InvalidRosterCursor);
        }
        Ok(cursor)
    }
}

/// One page of roster patients and any continuation cursor.
#[derive(Clone, Debug)]
pub(crate) struct RosterPage {
    pub(crate) patients: Vec<Patient>,
    pub(crate) next_after: Option<String>,
}

/// Failure reading or paging the practice's patient roster.
#[derive(Debug)]
pub(crate) enum RosterSearchError {
    Unavailable(StoreUnavailable),
    InvalidCursor(InvalidRosterCursor),
}

impl std::fmt::Display for RosterSearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable(err) => std::fmt::Display::fmt(err, f),
            Self::InvalidCursor(err) => std::fmt::Display::fmt(err, f),
        }
    }
}

impl std::error::Error for RosterSearchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Unavailable(err) => Some(err),
            Self::InvalidCursor(err) => Some(err),
        }
    }
}

impl From<StoreUnavailable> for RosterSearchError {
    fn from(err: StoreUnavailable) -> Self {
        Self::Unavailable(err)
    }
}

impl From<InvalidRosterCursor> for RosterSearchError {
    fn from(err: InvalidRosterCursor) -> Self {
        Self::InvalidCursor(err)
    }
}

/// How a connected source's records currently stand for a patient.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SourceDeliveryState {
    /// A pull succeeded at `pulled_at` (Unix seconds) and is current.
    Usable {
        /// Unix timestamp (seconds) of the successful pull.
        pulled_at: i64,
    },
    /// The latest refresh failed; `last_good_pulled_at` preserves any prior good pull.
    RefreshFailed {
        /// Unix timestamp (seconds) of the last successful pull, if one occurred.
        last_good_pulled_at: Option<i64>,
    },
    /// Records are still connecting for the first time.
    Hydrating,
    /// Source is registered for the patient with no completed pull yet.
    NeverPulled,
}

/// One source's patient identity and record delivery state.
#[derive(Clone, Debug)]
pub struct PatientSourceRecord {
    /// Identifier of the source system.
    pub source_id: String,
    /// Human-readable label of the source system.
    pub source_label: String,
    /// Demographics known by this source for the patient.
    pub patient: SourcePatient,
    /// Delivery and freshness status of this source's records.
    pub delivery: SourceDeliveryState,
}

impl From<SourcePull> for PatientSourceRecord {
    fn from(pull: SourcePull) -> Self {
        Self {
            source_id: pull.source_id,
            source_label: pull.source_label,
            patient: pull.patient,
            delivery: SourceDeliveryState::Usable {
                pulled_at: pull.pulled_at,
            },
        }
    }
}

/// Reads each source's last pull and saved summary for a set of patients.
#[async_trait]
pub trait PatientDirectory: Send + Sync {
    /// The pulls of `workspaces`, keyed by workspace, each ordered by source.
    async fn pulls(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Vec<SourcePull>>, StoreUnavailable>;

    /// Per-source delivery state for `workspaces`, defaulting to [`Self::pulls`] as usable pulls.
    async fn source_records(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Vec<PatientSourceRecord>>, StoreUnavailable> {
        let pulls = self.pulls(workspaces).await?;
        Ok(pulls
            .into_iter()
            .map(|(workspace, list)| {
                (
                    workspace,
                    list.into_iter().map(PatientSourceRecord::from).collect(),
                )
            })
            .collect())
    }

    /// Saved patient summaries (`patient_synopsis` JSON) keyed by workspace.
    async fn synopses(
        &self,
        _workspaces: &[String],
    ) -> Result<BTreeMap<String, Value>, StoreUnavailable> {
        Ok(BTreeMap::new())
    }

    /// Running or failed background understanding runs (`automation_run` JSON) keyed by workspace.
    async fn automation_runs(
        &self,
        _workspaces: &[String],
    ) -> Result<BTreeMap<String, Value>, StoreUnavailable> {
        Ok(BTreeMap::new())
    }
}

#[async_trait]
impl PatientDirectory for Store {
    async fn pulls(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Vec<SourcePull>>, StoreUnavailable> {
        self.source_pulls(workspaces)
            .await
            .map_err(|_| StoreUnavailable)
    }

    async fn source_records(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Vec<PatientSourceRecord>>, StoreUnavailable> {
        use local_product::runtime::refresh::SourceRefreshOutcome;

        let pulls = self.pulls(workspaces).await?;
        if pulls.is_empty() {
            return Ok(BTreeMap::new());
        }
        let practices = self
            .registered_dispatch_practices()
            .await
            .map_err(|_| StoreUnavailable)?;
        let mut out = BTreeMap::new();
        for (workspace, list) in pulls {
            let mut statuses = BTreeMap::new();
            for practice in &practices {
                let saved = self
                    .saved_source_statuses(practice, &workspace)
                    .await
                    .map_err(|_| StoreUnavailable)?;
                if !saved.is_empty() {
                    for status in saved {
                        statuses.insert(status.source_id.as_str().to_owned(), status.outcome);
                    }
                    break;
                }
            }
            let workspace_records = list
                .into_iter()
                .map(|pull| {
                    let delivery = match statuses.get(&pull.source_id) {
                        Some(SourceRefreshOutcome::Failed {
                            last_good_pulled_at,
                            ..
                        }) => SourceDeliveryState::RefreshFailed {
                            last_good_pulled_at: (*last_good_pulled_at).or(Some(pull.pulled_at)),
                        },
                        Some(
                            SourceRefreshOutcome::Updated { pulled_at, .. }
                            | SourceRefreshOutcome::PartlyUpdated { pulled_at, .. },
                        ) => SourceDeliveryState::Usable {
                            pulled_at: *pulled_at,
                        },
                        None => SourceDeliveryState::Usable {
                            pulled_at: pull.pulled_at,
                        },
                    };
                    PatientSourceRecord {
                        source_id: pull.source_id,
                        source_label: pull.source_label,
                        patient: pull.patient,
                        delivery,
                    }
                })
                .collect();
            out.insert(workspace, workspace_records);
        }
        Ok(out)
    }

    async fn synopses(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Value>, StoreUnavailable> {
        let views = self
            .patient_summaries_for_workspaces(workspaces)
            .await
            .map_err(|_| StoreUnavailable)?;
        Ok(views
            .into_iter()
            .map(|(workspace, view)| {
                let authored = UNIX_EPOCH + Duration::from_secs(view.updated_at.unsigned_abs());
                (workspace, view.to_wire_synopsis(format_rfc3339(authored)))
            })
            .collect())
    }

    async fn automation_runs(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Value>, StoreUnavailable> {
        let runs = self
            .active_or_failed_understanding_runs(workspaces)
            .await
            .map_err(|_| StoreUnavailable)?;
        Ok(runs
            .into_iter()
            .map(|(workspace, status)| {
                let started = UNIX_EPOCH + Duration::from_secs(status.started_at.unsigned_abs());
                (
                    workspace,
                    status.to_wire_automation_run(format_rfc3339(started)),
                )
            })
            .collect())
    }
}

/// The directory of a site with no database, and of tests.
#[derive(Default)]
pub struct MemoryPatients {
    records: Mutex<BTreeMap<String, Vec<PatientSourceRecord>>>,
    synopses: Mutex<BTreeMap<String, Value>>,
    automation_runs: Mutex<BTreeMap<String, Value>>,
    unavailable: std::sync::atomic::AtomicBool,
}

impl MemoryPatients {
    /// Records one source's successful pull for `workspace`.
    pub fn add(&self, workspace: &str, pull: SourcePull) {
        self.add_source_record(workspace, PatientSourceRecord::from(pull));
    }

    /// Records one source's explicit delivery state for `workspace`.
    pub fn add_source_record(&self, workspace: &str, record: PatientSourceRecord) {
        self.records
            .lock()
            .expect("patient source records")
            .entry(workspace.to_owned())
            .or_default()
            .push(record);
    }

    /// Replaces the source records held for `workspace`.
    pub fn set_source_records(&self, workspace: &str, records: Vec<PatientSourceRecord>) {
        self.records
            .lock()
            .expect("patient source records")
            .insert(workspace.to_owned(), records);
    }

    /// Saves a `patient_synopsis` projection for `workspace`.
    pub fn set_synopsis(&self, workspace: &str, synopsis: Value) {
        self.synopses
            .lock()
            .expect("patient synopses")
            .insert(workspace.to_owned(), synopsis);
    }

    /// Saves an `automation_run` projection for `workspace`.
    pub fn set_automation_run(&self, workspace: &str, run: Value) {
        self.automation_runs
            .lock()
            .expect("patient automation runs")
            .insert(workspace.to_owned(), run);
    }

    /// Simulates directory unavailability so callers verify failure handling.
    pub fn set_unavailable(&self, unavailable: bool) {
        self.unavailable
            .store(unavailable, std::sync::atomic::Ordering::SeqCst);
    }

    /// The five patients the tests read, held in memory.
    #[doc(hidden)]
    pub fn fixture() -> Arc<Self> {
        let patients = Self::default();
        for workspace in crate::WORKSPACE_IDS {
            patients.add(workspace, fixture_pull(workspace));
        }
        Arc::new(patients)
    }

    fn workspaces(&self) -> Vec<String> {
        self.records
            .lock()
            .expect("patient source records")
            .keys()
            .cloned()
            .collect()
    }
}

/// A fixture patient as one source's pull.
pub(crate) fn fixture_pull(workspace: &str) -> SourcePull {
    let body = workspace_body(workspace).expect("fixture patient");
    let body: Value = serde_json::from_str(body).expect("fixture patient");
    let identity = &body["identity"];
    let text = |value: &Value| value.as_str().map(str::to_owned);
    SourcePull {
        source_id: "fixture-chart".to_owned(),
        source_label: "Fixture chart".to_owned(),
        patient: SourcePatient {
            source_patient_id: workspace.to_owned(),
            display_name: text(&body["display_name"]).expect("fixture name"),
            birth_date: text(&identity["birth_date"]),
            sex_or_gender: text(&identity["sex_or_gender"]["value"]),
            medical_record_numbers: identity["medical_record_numbers"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|number| {
                    Some(MedicalRecordNumber {
                        system: text(&number["system"])?,
                        value: text(&number["value"])?,
                    })
                })
                .collect(),
        },
        pulled_at: 1_787_846_400,
    }
}

#[async_trait]
impl PatientDirectory for MemoryPatients {
    async fn pulls(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Vec<SourcePull>>, StoreUnavailable> {
        if self.unavailable.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(StoreUnavailable);
        }
        let records = self.records.lock().expect("patient source records");
        Ok(workspaces
            .iter()
            .filter_map(|workspace| {
                let usable: Vec<SourcePull> = records
                    .get(workspace)?
                    .iter()
                    .filter_map(|record| match record.delivery {
                        SourceDeliveryState::Usable { pulled_at } => Some(SourcePull {
                            source_id: record.source_id.clone(),
                            source_label: record.source_label.clone(),
                            patient: record.patient.clone(),
                            pulled_at,
                        }),
                        SourceDeliveryState::RefreshFailed {
                            last_good_pulled_at: Some(pulled_at),
                        } => Some(SourcePull {
                            source_id: record.source_id.clone(),
                            source_label: record.source_label.clone(),
                            patient: record.patient.clone(),
                            pulled_at,
                        }),
                        SourceDeliveryState::RefreshFailed {
                            last_good_pulled_at: None,
                        }
                        | SourceDeliveryState::Hydrating
                        | SourceDeliveryState::NeverPulled => None,
                    })
                    .collect();
                (!usable.is_empty()).then_some((workspace.clone(), usable))
            })
            .collect())
    }

    async fn source_records(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Vec<PatientSourceRecord>>, StoreUnavailable> {
        if self.unavailable.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(StoreUnavailable);
        }
        let records = self.records.lock().expect("patient source records");
        Ok(workspaces
            .iter()
            .filter_map(|workspace| {
                records
                    .get(workspace)
                    .filter(|found| !found.is_empty())
                    .map(|found| (workspace.clone(), found.clone()))
            })
            .collect())
    }

    async fn synopses(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Value>, StoreUnavailable> {
        if self.unavailable.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(StoreUnavailable);
        }
        let synopses = self.synopses.lock().expect("patient synopses");
        Ok(workspaces
            .iter()
            .filter_map(|workspace| {
                synopses
                    .get(workspace)
                    .map(|synopsis| (workspace.clone(), synopsis.clone()))
            })
            .collect())
    }

    async fn automation_runs(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Value>, StoreUnavailable> {
        if self.unavailable.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(StoreUnavailable);
        }
        let runs = self
            .automation_runs
            .lock()
            .expect("patient automation runs");
        Ok(workspaces
            .iter()
            .filter_map(|workspace| {
                runs.get(workspace)
                    .map(|run| (workspace.clone(), run.clone()))
            })
            .collect())
    }
}

/// Creates the synthetic practice in memory, registers `patients` to it and
/// places them on its staging panel, as a load does with a database.
pub(crate) async fn enroll_in_memory(store: &MemoryAuthStore, patients: &MemoryPatients) {
    store.seed_practice(STAGING_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK);
    let panel = staging_panel(store)
        .await
        .expect("the memory store always reads");
    for workspace in patients.workspaces() {
        store.seed_patient(STAGING_PRACTICE_ID, &workspace);
        for identity in &panel {
            store
                .place_on_panel(identity, &workspace)
                .await
                .expect("the memory store always writes");
        }
    }
}

/// Registers the fixture patients to the synthetic practice in Postgres, with
/// a pull for each and the staging panel on each, for tests that start a real
/// server.
#[doc(hidden)]
pub async fn enroll_fixture(auth: &dyn AuthStore, runtime: &Store) -> Result<(), String> {
    auth.ensure_practice(STAGING_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK)
        .await
        .map_err(|_| "create the synthetic practice".to_owned())?;
    let panel = staging_panel(auth)
        .await
        .map_err(|_| "read the staging panel".to_owned())?;
    for workspace in crate::WORKSPACE_IDS {
        let pull = fixture_pull(workspace);
        let pulled_at = pull.pulled_at;
        runtime
            .record_chart(workspace, &[], &pull)
            .await
            .map_err(|error| format!("{workspace}: {error}"))?;
        auth.register_patient(STAGING_PRACTICE_ID, workspace)
            .await
            .map_err(|_| format!("{workspace}: register the patient"))?;
        runtime
            .register_patient_dispatch(auth, STAGING_PRACTICE_ID, workspace, pulled_at)
            .await
            .map_err(|error| format!("{workspace}: {error}"))?;
        for identity in &panel {
            auth.set_membership(identity, workspace, Membership::Allowed)
                .await
                .map_err(|_| format!("{workspace}: place the patient on the panel"))?;
        }
    }
    Ok(())
}

/// Deterministic match priority for Find patient queries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SearchRank {
    /// Exact match on a supplied medical record number or birth date.
    ExactIdentifier,
    /// Prefix match from the start of the patient's display name.
    FullNamePrefix,
    /// Prefix match on a later word of the patient's display name.
    TokenPrefix,
    /// Substring match across display name, birth date, and medical record numbers.
    Substring,
}

/// One patient: their workspace, what their sources delivered, and any saved summary.
#[derive(Clone, Debug)]
pub(crate) struct Patient {
    workspace: String,
    sources: Vec<PatientSourceRecord>,
    synopsis: Option<Value>,
    automation_run: Option<Value>,
}

impl Patient {
    fn from_entry(
        (workspace, sources): (String, Vec<PatientSourceRecord>),
        synopsis: Option<Value>,
        automation_run: Option<Value>,
    ) -> Option<Self> {
        (!sources.is_empty()).then_some(Self {
            workspace,
            sources,
            synopsis,
            automation_run,
        })
    }

    fn holder(&self) -> &SourcePatient {
        &self.sources[0].patient
    }

    fn primary_mrn(&self) -> Option<&str> {
        self.holder()
            .medical_record_numbers
            .first()
            .map(|mrn| mrn.value.as_str())
            .filter(|value| !value.trim().is_empty())
    }

    fn sort_key(&self) -> (String, &str) {
        (
            self.holder().display_name.to_lowercase(),
            self.workspace.as_str(),
        )
    }

    /// A row of the physician's patient list.
    pub(crate) fn list_row(&self) -> Value {
        json!({
            "patient_workspace_id": self.workspace,
            "display_name": self.holder().display_name,
        })
    }

    /// A row of roster search.
    pub(crate) fn roster_row(&self) -> Value {
        json!({
            "patient_workspace_id": self.workspace,
            "patient_display_name": self.holder().display_name,
            "birth_date": self.holder().birth_date,
            "mrn": self.primary_mrn(),
        })
    }

    fn search_rank(&self, normalized_query: &str, words: &[String]) -> Option<SearchRank> {
        let patient = self.holder();
        let birth_iso = patient
            .birth_date
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty());
        let birth_slash = birth_iso.and_then(slash_birth_date);
        let birth_words = birth_iso.and_then(spoken_birth_date);
        let exact_mrn = patient
            .medical_record_numbers
            .iter()
            .any(|mrn| mrn.value.trim().eq_ignore_ascii_case(normalized_query));
        let exact_dob = birth_iso.is_some_and(|dob| dob.eq_ignore_ascii_case(normalized_query))
            || birth_slash
                .as_deref()
                .is_some_and(|dob| dob.eq_ignore_ascii_case(normalized_query));
        if exact_mrn || exact_dob {
            return Some(SearchRank::ExactIdentifier);
        }

        let mrn_joined: String = patient
            .medical_record_numbers
            .iter()
            .map(|mrn| mrn.value.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        let haystack = format!(
            "{} {} {} {} {}",
            patient.display_name,
            birth_iso.unwrap_or_default(),
            birth_slash.as_deref().unwrap_or_default(),
            birth_words.as_deref().unwrap_or_default(),
            mrn_joined,
        )
        .to_lowercase();
        if !words.iter().all(|word| haystack.contains(word)) {
            return None;
        }

        let name_lower = patient.display_name.to_lowercase();
        if name_lower.starts_with(normalized_query) {
            Some(SearchRank::FullNamePrefix)
        } else if words.len() == 1
            && patient
                .display_name
                .split_whitespace()
                .any(|token| token.to_lowercase().starts_with(&words[0]))
        {
            Some(SearchRank::TokenPrefix)
        } else {
            Some(SearchRank::Substring)
        }
    }

    fn identity(&self, today: &str) -> Value {
        let patient = self.holder();
        let sex_or_gender = patient
            .sex_or_gender
            .as_ref()
            .map(|value| json!({ "label": "Administrative gender", "value": value }));
        let numbers: Vec<Value> = patient
            .medical_record_numbers
            .iter()
            .map(|number| json!({ "system": number.system, "value": number.value }))
            .collect();
        json!({
            "state": "available",
            "display_name": patient.display_name,
            "birth_date": patient.birth_date,
            "age_years": patient.birth_date.as_deref().and_then(|birth| age_on(birth, today)),
            "age_as_of": today,
            "sex_or_gender": sex_or_gender,
            "medical_record_numbers": numbers,
        })
    }

    fn connected_sources(&self) -> Vec<Value> {
        self.sources
            .iter()
            .map(|source| {
                let state = match source.delivery {
                    SourceDeliveryState::Usable { pulled_at } => {
                        let pulled = UNIX_EPOCH + Duration::from_secs(pulled_at.unsigned_abs());
                        json!({
                            "state": "usable",
                            "synchronized_at": format_rfc3339(pulled),
                        })
                    }
                    SourceDeliveryState::RefreshFailed {
                        last_good_pulled_at: Some(pulled_at),
                    } => {
                        let pulled = UNIX_EPOCH + Duration::from_secs(pulled_at.unsigned_abs());
                        json!({
                            "state": "failed",
                            "usable_snapshot": {
                                "synchronized_at": format_rfc3339(pulled),
                            },
                        })
                    }
                    SourceDeliveryState::RefreshFailed {
                        last_good_pulled_at: None,
                    } => json!({ "state": "failed" }),
                    SourceDeliveryState::Hydrating => json!({ "state": "hydrating" }),
                    SourceDeliveryState::NeverPulled => json!({ "state": "not_yet_pulled" }),
                };
                json!({
                    "source_id": source.source_id,
                    "source_label": source.source_label,
                    "state": state,
                })
            })
            .collect()
    }

    /// The patient page: identity, saved summary when present, and how current the records are.
    #[cfg(test)]
    pub(crate) fn workspace_body(&self, now: &str) -> Value {
        self.workspace_body_in_zone(now, now.get(..10).unwrap_or_default(), PRACTICE_TIME_ZONE)
    }

    /// The patient page in the practice's saved local date and time zone.
    pub(crate) fn workspace_body_in_zone(&self, now: &str, today: &str, time_zone: &str) -> Value {
        json!({
            "patient_workspace_id": self.workspace,
            "display_name": self.holder().display_name,
            "time_zone": time_zone,
            "current_time": now,
            "identity": self.identity(today),
            "patient_synopsis": self.synopsis,
            "automation_run": self.automation_run,
            "connected_sources": self.connected_sources(),
            "chats": [],
            "active_follow_ups": [],
            "has_past_work": false,
        })
    }

    /// What a physician in the clinic reads of a patient not on their panel:
    /// strictly display name, birth date, and medical record numbers.
    #[cfg(test)]
    pub(crate) fn clinic_demographics(&self, now: &str) -> Value {
        self.clinic_demographics_in_zone(now, PRACTICE_TIME_ZONE)
    }

    /// What a physician in the clinic reads of a patient not on their panel,
    /// carrying the practice's saved IANA time zone.
    pub(crate) fn clinic_demographics_in_zone(&self, now: &str, time_zone: &str) -> Value {
        let patient = self.holder();
        let numbers: Vec<Value> = patient
            .medical_record_numbers
            .iter()
            .map(|number| json!({ "system": number.system, "value": number.value }))
            .collect();
        json!({
            "patient_workspace_id": self.workspace,
            "display_name": patient.display_name,
            "membership": "clinic_demographic",
            "time_zone": time_zone,
            "current_time": now,
            "identity": {
                "display_name": patient.display_name,
                "birth_date": patient.birth_date,
                "medical_record_numbers": numbers,
            },
            "chats": [],
            "patient_synopsis": null,
            "automation_run": null,
            "active_follow_ups": [],
            "has_past_work": false,
            "connected_sources": [],
        })
    }
}

fn slash_birth_date(birth: &str) -> Option<String> {
    let mut parts = birth.split('-');
    let year = parts.next()?;
    let month: u32 = parts.next()?.parse().ok()?;
    let day: u32 = parts.next()?.parse().ok()?;
    if parts.next().is_some() || year.len() != 4 {
        return None;
    }
    Some(format!("{month:02}/{day:02}/{year}"))
}

fn spoken_birth_date(birth: &str) -> Option<String> {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let mut parts = birth.split('-');
    let year = parts.next()?;
    let month: usize = parts.next()?.parse().ok()?;
    let day: u32 = parts.next()?.parse().ok()?;
    let name = *MONTHS.get(month.checked_sub(1)?)?;
    Some(format!("{name} {day}, {year}"))
}

/// Whole years between two `YYYY-MM-DD` dates.
fn age_on(birth: &str, today: &str) -> Option<i64> {
    let parts = |date: &str| -> Option<(i64, i64, i64)> {
        let mut pieces = date.split('-');
        Some((
            pieces.next()?.parse().ok()?,
            pieces.next()?.parse().ok()?,
            pieces.next()?.parse().ok()?,
        ))
    };
    let (born_year, born_month, born_day) = parts(birth)?;
    let (year, month, day) = parts(today)?;
    let before_birthday = (month, day) < (born_month, born_day);
    Some(year - born_year - i64::from(before_birthday))
}

/// The patients of `workspaces` any source holds, by name.
pub(crate) async fn read_patients(
    directory: &dyn PatientDirectory,
    workspaces: &[String],
) -> Result<Vec<Patient>, StoreUnavailable> {
    let mut synopses = directory.synopses(workspaces).await?;
    let mut automation_runs = directory.automation_runs(workspaces).await?;
    let mut patients: Vec<Patient> = directory
        .source_records(workspaces)
        .await?
        .into_iter()
        .filter_map(|(workspace, records)| {
            let synopsis = synopses.remove(&workspace);
            let automation_run = automation_runs.remove(&workspace);
            Patient::from_entry((workspace, records), synopsis, automation_run)
        })
        .collect();
    patients.sort_by(|left, right| left.sort_key().cmp(&right.sort_key()));
    Ok(patients)
}

/// One patient any source holds.
pub(crate) async fn read_patient(
    directory: &dyn PatientDirectory,
    workspace: &str,
) -> Result<Option<Patient>, StoreUnavailable> {
    Ok(read_patients(directory, &[workspace.to_owned()])
        .await?
        .pop())
}

/// Searches `workspaces` deterministically across display name, DOB, and MRN:
/// exact MRN or DOB matches first, then name prefixes, then substrings.
/// A blank query finds no one.
pub(crate) async fn search(
    directory: &dyn PatientDirectory,
    workspaces: &[String],
    query: &str,
) -> Result<Vec<Patient>, StoreUnavailable> {
    let words: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    if words.is_empty() {
        return Ok(Vec::new());
    }
    let normalized = words.join(" ");
    let patients = read_patients(directory, workspaces).await?;
    let mut ranked: Vec<(SearchRank, Patient)> = patients
        .into_iter()
        .filter_map(|patient| {
            let rank = patient.search_rank(&normalized, &words)?;
            Some((rank, patient))
        })
        .collect();
    ranked.sort_by(|(left_rank, left), (right_rank, right)| {
        left_rank
            .cmp(right_rank)
            .then_with(|| left.sort_key().cmp(&right.sort_key()))
    });
    Ok(ranked
        .into_iter()
        .take(SEARCH_LIMIT)
        .map(|(_, patient)| patient)
        .collect())
}

/// Lists or searches `workspaces`: an empty query returns patients ordered by
/// name in pages of [`ROSTER_PAGE_SIZE`] with `next_after` when more remain; a
/// non-empty query returns ranked matches.
pub(crate) async fn roster_page(
    directory: &dyn PatientDirectory,
    workspaces: &[String],
    query: &str,
    after: Option<&str>,
) -> Result<RosterPage, RosterSearchError> {
    let words: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    if !words.is_empty() {
        let patients = search(directory, workspaces, query).await?;
        return Ok(RosterPage {
            patients,
            next_after: None,
        });
    }
    let cursor = match after.map(str::trim).filter(|raw| !raw.is_empty()) {
        Some(raw) => Some(RosterCursor::decode(raw)?),
        None => None,
    };
    let all = read_patients(directory, workspaces).await?;
    let mut page: Vec<Patient> = all
        .into_iter()
        .filter(|patient| {
            cursor.as_ref().is_none_or(|after_key| {
                patient.sort_key() > (after_key.name_key.clone(), after_key.workspace.as_str())
            })
        })
        .take(ROSTER_PAGE_SIZE + 1)
        .collect();
    let next_after = if page.len() > ROSTER_PAGE_SIZE {
        page.truncate(ROSTER_PAGE_SIZE);
        page.last()
            .map(|last| RosterCursor::from_patient(last).encode())
    } else {
        None
    };
    Ok(RosterPage {
        patients: page,
        next_after,
    })
}

pub(crate) fn workspace_body(id: &str) -> Option<&'static str> {
    Some(match id {
        "pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee" => {
            include_str!("../fixtures/workspaces/pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee.json")
        }
        "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d" => {
            include_str!("../fixtures/workspaces/pw-82c44e9b-58c0-4af1-bd21-771bf77e101d.json")
        }
        "pw-f5404d89-acde-4bb5-8c42-2fa191766811" => {
            include_str!("../fixtures/workspaces/pw-f5404d89-acde-4bb5-8c42-2fa191766811.json")
        }
        "pw-d11a35f7-0d8d-466f-a48e-5812df0a4971" => {
            include_str!("../fixtures/workspaces/pw-d11a35f7-0d8d-466f-a48e-5812df0a4971.json")
        }
        "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e" => {
            include_str!("../fixtures/workspaces/pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e.json")
        }
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_record(
        workspace: &str,
        name: &str,
        birth_date: Option<&str>,
        mrn: Option<&str>,
        delivery: SourceDeliveryState,
    ) -> PatientSourceRecord {
        PatientSourceRecord {
            source_id: "practice-ehr".to_owned(),
            source_label: "Practice EHR".to_owned(),
            patient: SourcePatient {
                source_patient_id: workspace.to_owned(),
                display_name: name.to_owned(),
                birth_date: birth_date.map(str::to_owned),
                sex_or_gender: Some("Female".to_owned()),
                medical_record_numbers: mrn
                    .into_iter()
                    .map(|value| MedicalRecordNumber {
                        system: "MRN".to_owned(),
                        value: value.to_owned(),
                    })
                    .collect(),
            },
            delivery,
        }
    }

    #[test]
    fn age_turns_on_the_birthday() {
        assert_eq!(age_on("1972-08-27", "2026-08-26"), Some(53));
        assert_eq!(age_on("1972-08-27", "2026-08-27"), Some(54));
        assert_eq!(age_on("unknown", "2026-08-27"), None);
    }

    #[tokio::test]
    async fn search_orders_exact_identifier_before_prefix_before_substring() {
        let directory = MemoryPatients::default();
        directory.add_source_record(
            "pw-substring",
            sample_record(
                "pw-substring",
                "Clara Marlow",
                Some("1981-04-10"),
                Some("99104200"),
                SourceDeliveryState::Usable {
                    pulled_at: 1_787_846_400,
                },
            ),
        );
        directory.add_source_record(
            "pw-prefix",
            sample_record(
                "pw-prefix",
                "Marlo Vance",
                Some("1979-11-02"),
                Some("88001122"),
                SourceDeliveryState::Usable {
                    pulled_at: 1_787_846_400,
                },
            ),
        );
        directory.add_source_record(
            "pw-exact",
            sample_record(
                "pw-exact",
                "Zoe Chen",
                Some("1990-01-15"),
                Some("MARLO"),
                SourceDeliveryState::Usable {
                    pulled_at: 1_787_846_400,
                },
            ),
        );
        directory.add_source_record(
            "pw-other-practice",
            sample_record(
                "pw-other-practice",
                "Marlo External",
                Some("1990-01-15"),
                Some("MARLO"),
                SourceDeliveryState::Usable {
                    pulled_at: 1_787_846_400,
                },
            ),
        );

        let practice_roster = vec![
            "pw-substring".to_owned(),
            "pw-prefix".to_owned(),
            "pw-exact".to_owned(),
        ];
        let found = search(&directory, &practice_roster, "marlo")
            .await
            .expect("search succeeds");
        let ids: Vec<&str> = found.iter().map(|p| p.workspace.as_str()).collect();
        assert_eq!(ids, vec!["pw-exact", "pw-prefix", "pw-substring"]);

        let by_dob = search(&directory, &practice_roster, "04/10/1981")
            .await
            .expect("search by formatted DOB succeeds");
        assert_eq!(by_dob.len(), 1);
        assert_eq!(by_dob[0].workspace, "pw-substring");
    }

    #[tokio::test]
    async fn clinic_demographics_exposes_only_name_dob_and_mrn() {
        let directory = MemoryPatients::default();
        directory.add_source_record(
            "pw-off-panel",
            sample_record(
                "pw-off-panel",
                "Mary Louis",
                Some("1972-08-27"),
                Some("00000005"),
                SourceDeliveryState::Usable {
                    pulled_at: 1_787_846_400,
                },
            ),
        );
        directory.set_synopsis(
            "pw-off-panel",
            json!({
                "version_id": "v1",
                "authored_text": "Persistent atrial fibrillation on apixaban."
            }),
        );
        let patient = read_patient(&directory, "pw-off-panel")
            .await
            .expect("read succeeds")
            .expect("patient present");
        let off_panel = patient.clinic_demographics("2026-08-28T16:00:00Z");
        assert_eq!(off_panel["membership"], "clinic_demographic");
        assert_eq!(off_panel["display_name"], "Mary Louis");
        assert_eq!(off_panel["identity"]["birth_date"], "1972-08-27");
        assert_eq!(
            off_panel["identity"]["medical_record_numbers"][0]["value"],
            "00000005"
        );
        assert!(off_panel["identity"].get("age_years").is_none());
        assert!(off_panel["identity"].get("sex_or_gender").is_none());
        assert!(off_panel["identity"].get("state").is_none());
        assert!(off_panel["patient_synopsis"].is_null());
        assert_eq!(off_panel["connected_sources"], json!([]));
    }

    #[tokio::test]
    async fn source_delivery_never_reports_usable_without_saved_pull_and_keeps_last_good_time() {
        let directory = MemoryPatients::default();
        directory.set_source_records(
            "pw-stale",
            vec![
                sample_record(
                    "pw-stale",
                    "Jake Smith",
                    Some("1950-03-12"),
                    Some("00000001"),
                    SourceDeliveryState::RefreshFailed {
                        last_good_pulled_at: Some(1_787_846_400),
                    },
                ),
                PatientSourceRecord {
                    source_id: "Quest".to_owned(),
                    source_label: "Quest Diagnostics".to_owned(),
                    patient: SourcePatient {
                        source_patient_id: "pw-stale".to_owned(),
                        display_name: "Jake Smith".to_owned(),
                        birth_date: Some("1950-03-12".to_owned()),
                        sex_or_gender: None,
                        medical_record_numbers: Vec::new(),
                    },
                    delivery: SourceDeliveryState::NeverPulled,
                },
            ],
        );
        let patient = read_patient(&directory, "pw-stale")
            .await
            .expect("read succeeds")
            .expect("patient present");
        let body = patient.workspace_body("2026-08-28T16:00:00Z");
        let sources = body["connected_sources"].as_array().expect("sources array");
        assert_eq!(sources[0]["state"]["state"], "failed");
        assert_eq!(
            sources[0]["state"]["usable_snapshot"]["synchronized_at"],
            "2026-08-27T16:00:00.000Z"
        );
        assert_eq!(sources[1]["state"]["state"], "not_yet_pulled");
        assert!(sources[1]["state"].get("synchronized_at").is_none());
    }

    #[tokio::test]
    async fn empty_roster_query_pages_twenty_five_by_name_and_excludes_other_practices() {
        let directory = MemoryPatients::default();
        let mut practice_a_workspaces = Vec::new();
        for idx in (1..=32).rev() {
            let workspace = format!("pw-a-{idx:02}");
            let name = format!("Patient {idx:02}");
            directory.add_source_record(
                &workspace,
                sample_record(
                    &workspace,
                    &name,
                    Some("1980-05-14"),
                    Some(&format!("1000{idx:02}")),
                    SourceDeliveryState::Usable {
                        pulled_at: 1_787_846_400,
                    },
                ),
            );
            practice_a_workspaces.push(workspace);
        }
        // Seed a patient belonging to another practice in the shared source store;
        // practice-a's roster slice must never expose it.
        directory.add_source_record(
            "pw-other-practice",
            sample_record(
                "pw-other-practice",
                "Aaron OtherPractice",
                Some("1975-01-01"),
                Some("999999"),
                SourceDeliveryState::Usable {
                    pulled_at: 1_787_846_400,
                },
            ),
        );

        let first = roster_page(&directory, &practice_a_workspaces, "", None)
            .await
            .expect("first page");
        assert_eq!(first.patients.len(), ROSTER_PAGE_SIZE);
        let first_names: Vec<_> = first
            .patients
            .iter()
            .map(|p| p.holder().display_name.clone())
            .collect();
        let expected_first: Vec<_> = (1..=25).map(|n| format!("Patient {n:02}")).collect();
        assert_eq!(first_names, expected_first);
        assert!(
            !first_names
                .iter()
                .any(|name| name.contains("OtherPractice")),
            "cross-practice patient leaked into page 1"
        );
        let cursor = first.next_after.expect("more than 25 patients sets cursor");

        let second = roster_page(&directory, &practice_a_workspaces, "   ", Some(&cursor))
            .await
            .expect("second page");
        let second_names: Vec<_> = second
            .patients
            .iter()
            .map(|p| p.holder().display_name.clone())
            .collect();
        let expected_second: Vec<_> = (26..=32).map(|n| format!("Patient {n:02}")).collect();
        assert_eq!(second_names, expected_second);
        assert!(second.next_after.is_none());
        assert!(
            !second_names
                .iter()
                .any(|name| name.contains("OtherPractice")),
            "cross-practice patient leaked into page 2"
        );

        let bad = roster_page(&directory, &practice_a_workspaces, "", Some("not-a-cursor")).await;
        assert!(matches!(bad, Err(RosterSearchError::InvalidCursor(_))));
    }
}
