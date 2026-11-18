//! A practice's patients as a physician reads them.
//!
//! The auth store says who belongs to a practice and whose panel they are on.
//! Each source's last pull says who the patient is and how current their
//! records are, while [`SourceStanding`] reflects whether a later pull failed,
//! partially degraded, or lost source access. Search, the patient list and the
//! patient page all read from here, so a patient exists on every surface or on
//! none: a registered patient no source holds reads as not found.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};
use std::time::{Duration, UNIX_EPOCH};

use async_trait::async_trait;
use local_product::physician_sources::{MedicalRecordNumber, SourcePatient};
use local_product::runtime::Store;
use local_product::runtime::sources::SourcePull;
use local_product::source_status::{SourceStanding, SourceStatusError};
use physician_auth::{AuthStore, Membership, MemoryAuthStore, StoreUnavailable};
use serde_json::{Value, json};

use crate::auth::{STAGING_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK, staging_panel};
use crate::sources::SourceRegistry;
use crate::timestamp::format_rfc3339;

/// The zone the practice's times are worded in.
pub(crate) const PRACTICE_TIME_ZONE: &str = "America/New_York";

/// The most patients one search answers with.
const SEARCH_LIMIT: usize = 50;

/// Failure reading patient pulls, source standings, or synopses from [`PatientDirectory`].
#[derive(Debug, thiserror::Error)]
pub enum DirectoryError {
    /// Reading patient source pulls failed.
    #[error("failed to read patient source pulls")]
    Pulls(#[source] sea_orm::DbErr),
    /// Reading workspace source standings failed.
    #[error("failed to read workspace source standings")]
    Standings(#[from] SourceStatusError),
    /// Reading saved workspace source refresh statuses failed.
    #[error("failed to read saved workspace source refresh statuses")]
    RefreshStatuses(#[from] local_product::runtime::refresh::RefreshError),
    /// Directory store is unavailable.
    #[error(transparent)]
    Unavailable(#[from] StoreUnavailable),
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
        let delivery = match pull.pulled_at {
            Some(pulled_at) => SourceDeliveryState::Usable { pulled_at },
            None => SourceDeliveryState::RefreshFailed {
                last_good_pulled_at: None,
            },
        };
        Self {
            source_id: pull.source_id,
            source_label: pull.source_label,
            patient: pull.patient,
            delivery,
        }
    }
}

/// Reads each source's last pull, effective standing, and saved summary for a set of patients.
#[async_trait]
pub trait PatientDirectory: Send + Sync {
    /// The pulls of `workspaces`, keyed by workspace, each ordered by source.
    async fn pulls(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Vec<SourcePull>>, DirectoryError>;

    /// Effective source standings for `workspaces`, keyed by workspace then `source_id`.
    async fn standings(
        &self,
        _workspaces: &[String],
    ) -> Result<BTreeMap<String, BTreeMap<String, SourceStanding>>, DirectoryError> {
        Ok(BTreeMap::new())
    }

    /// Per-source delivery state for `workspaces`, defaulting to [`Self::pulls`] as source records.
    async fn source_records(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Vec<PatientSourceRecord>>, DirectoryError> {
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
    ) -> Result<BTreeMap<String, Value>, DirectoryError> {
        Ok(BTreeMap::new())
    }

    /// Running or failed background understanding runs (`automation_run` JSON) keyed by workspace.
    async fn automation_runs(
        &self,
        _workspaces: &[String],
    ) -> Result<BTreeMap<String, Value>, DirectoryError> {
        Ok(BTreeMap::new())
    }
}

#[async_trait]
impl PatientDirectory for Store {
    async fn pulls(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Vec<SourcePull>>, DirectoryError> {
        self.source_pulls(workspaces)
            .await
            .map_err(DirectoryError::Pulls)
    }

    async fn standings(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, BTreeMap<String, SourceStanding>>, DirectoryError> {
        Ok(self.workspace_source_standings(workspaces).await?)
    }

    async fn source_records(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Vec<PatientSourceRecord>>, DirectoryError> {
        use local_product::runtime::refresh::SourceRefreshOutcome;

        let pulls = self.pulls(workspaces).await?;
        if pulls.is_empty() {
            return Ok(BTreeMap::new());
        }
        let practices = self
            .registered_dispatch_practices()
            .await
            .map_err(DirectoryError::Pulls)?;
        let mut out = BTreeMap::new();
        for (workspace, list) in pulls {
            let mut statuses = BTreeMap::new();
            for practice in &practices {
                let saved = self.saved_source_statuses(practice, &workspace).await?;
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
                            last_good_pulled_at: (*last_good_pulled_at).or(pull.pulled_at),
                        },
                        Some(
                            SourceRefreshOutcome::Updated { pulled_at, .. }
                            | SourceRefreshOutcome::PartlyUpdated { pulled_at, .. },
                        ) => SourceDeliveryState::Usable {
                            pulled_at: *pulled_at,
                        },
                        None => match pull.pulled_at {
                            Some(pulled_at) => SourceDeliveryState::Usable { pulled_at },
                            None => SourceDeliveryState::RefreshFailed {
                                last_good_pulled_at: None,
                            },
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
    ) -> Result<BTreeMap<String, Value>, DirectoryError> {
        let views = self
            .patient_summaries_for_workspaces(workspaces)
            .await
            .map_err(|_| DirectoryError::Unavailable(StoreUnavailable))?;
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
    ) -> Result<BTreeMap<String, Value>, DirectoryError> {
        let runs = self
            .active_or_failed_understanding_runs(workspaces)
            .await
            .map_err(|_| DirectoryError::Unavailable(StoreUnavailable))?;
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
        pulled_at: Some(1_787_846_400),
    }
}

#[async_trait]
impl PatientDirectory for MemoryPatients {
    async fn pulls(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, Vec<SourcePull>>, DirectoryError> {
        if self.unavailable.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(DirectoryError::Unavailable(StoreUnavailable));
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
                            pulled_at: Some(pulled_at),
                        }),
                        SourceDeliveryState::RefreshFailed {
                            last_good_pulled_at: Some(pulled_at),
                        } => Some(SourcePull {
                            source_id: record.source_id.clone(),
                            source_label: record.source_label.clone(),
                            patient: record.patient.clone(),
                            pulled_at: Some(pulled_at),
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
    ) -> Result<BTreeMap<String, Vec<PatientSourceRecord>>, DirectoryError> {
        if self.unavailable.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(DirectoryError::Unavailable(StoreUnavailable));
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
    ) -> Result<BTreeMap<String, Value>, DirectoryError> {
        if self.unavailable.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(DirectoryError::Unavailable(StoreUnavailable));
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
    ) -> Result<BTreeMap<String, Value>, DirectoryError> {
        if self.unavailable.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(DirectoryError::Unavailable(StoreUnavailable));
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
        let pulled_at = pull.pulled_at.unwrap_or(1_787_846_400);
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
    standings: BTreeMap<String, SourceStanding>,
    synopsis: Option<Value>,
    automation_run: Option<Value>,
}

fn unix_to_rfc3339(seconds: i64) -> String {
    format_rfc3339(UNIX_EPOCH + Duration::from_secs(seconds.unsigned_abs()))
}

impl Patient {
    fn from_entry(
        (workspace, sources): (String, Vec<PatientSourceRecord>),
        standings: BTreeMap<String, SourceStanding>,
        synopsis: Option<Value>,
        automation_run: Option<Value>,
    ) -> Option<Self> {
        (!sources.is_empty()).then_some(Self {
            workspace,
            sources,
            standings,
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

    fn identity(&self, now: &str) -> Value {
        let patient = self.holder();
        let today = now.get(..10).unwrap_or_default();
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

    fn connected_sources(&self, registry: Option<&SourceRegistry>) -> Vec<Value> {
        let records_by_source: BTreeMap<&str, &PatientSourceRecord> = self
            .sources
            .iter()
            .map(|record| (record.source_id.as_str(), record))
            .collect();
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        let source_ids: Vec<&str> = self
            .sources
            .iter()
            .map(|record| record.source_id.as_str())
            .chain(self.standings.keys().map(String::as_str))
            .filter(|source_id| seen.insert(source_id))
            .collect();

        let mut sources = Vec::with_capacity(source_ids.len());
        for source_id in source_ids {
            let record = records_by_source.get(source_id).copied();
            let standing = self.standings.get(source_id);
            let source_label = match record {
                Some(record) => record.source_label.clone(),
                None => registry
                    .and_then(|reg| reg.adapter(source_id))
                    .map(|adapter| adapter.source_label().to_owned())
                    .unwrap_or_else(|| source_id.to_owned()),
            };

            let entry = match standing {
                None | Some(SourceStanding::NotYetRead) => {
                    let Some(record) = record else {
                        continue;
                    };
                    match record.delivery {
                        SourceDeliveryState::Usable { pulled_at } => {
                            let synchronized_at = unix_to_rfc3339(pulled_at);
                            json!({
                                "source_id": source_id,
                                "source_label": source_label,
                                "standing": "up_to_date",
                                "reconnect_needed": false,
                                "last_good_read_at": synchronized_at,
                                "state": {
                                    "state": "usable",
                                    "synchronized_at": synchronized_at,
                                },
                            })
                        }
                        SourceDeliveryState::RefreshFailed {
                            last_good_pulled_at,
                        } => failed_source_json(source_id, &source_label, last_good_pulled_at, &[]),
                        SourceDeliveryState::Hydrating => json!({
                            "source_id": source_id,
                            "source_label": source_label,
                            "standing": "not_yet_read",
                            "reconnect_needed": false,
                            "last_good_read_at": null,
                            "state": { "state": "hydrating" },
                        }),
                        SourceDeliveryState::NeverPulled => json!({
                            "source_id": source_id,
                            "source_label": source_label,
                            "standing": "not_yet_read",
                            "reconnect_needed": false,
                            "last_good_read_at": null,
                            "state": { "state": "not_yet_pulled" },
                        }),
                    }
                }
                Some(SourceStanding::UpToDate {
                    last_good_read_at,
                    missing_candidates,
                }) => match record.map(|rec| &rec.delivery) {
                    Some(SourceDeliveryState::RefreshFailed {
                        last_good_pulled_at,
                    }) => failed_source_json(
                        source_id,
                        &source_label,
                        (*last_good_pulled_at).or(Some(*last_good_read_at)),
                        missing_candidates,
                    ),
                    _ => {
                        let synchronized_at = unix_to_rfc3339(*last_good_read_at);
                        json!({
                            "source_id": source_id,
                            "source_label": source_label,
                            "standing": "up_to_date",
                            "reconnect_needed": false,
                            "last_good_read_at": synchronized_at,
                            "missing_candidates": missing_candidates,
                            "state": {
                                "state": "usable",
                                "synchronized_at": synchronized_at,
                            },
                        })
                    }
                },
                Some(SourceStanding::Partial {
                    last_good_read_at,
                    attempted_at,
                    failed_parts,
                    missing_candidates,
                }) => {
                    let good = last_good_read_at.map(unix_to_rfc3339);
                    let attempted = unix_to_rfc3339(*attempted_at);
                    let usable_snapshot = good.as_ref().map(|ts| json!({ "synchronized_at": ts }));
                    json!({
                        "source_id": source_id,
                        "source_label": source_label,
                        "standing": "partial",
                        "reconnect_needed": false,
                        "last_good_read_at": good,
                        "attempted_at": attempted,
                        "failed_parts": failed_parts,
                        "missing_candidates": missing_candidates,
                        "state": {
                            "state": "failed",
                            "synchronized_at": attempted,
                            "usable_snapshot": usable_snapshot,
                        },
                    })
                }
                Some(SourceStanding::Failed {
                    last_good_read_at,
                    attempted_at,
                    reason,
                }) => {
                    let good = last_good_read_at.map(unix_to_rfc3339);
                    let attempted = unix_to_rfc3339(*attempted_at);
                    let usable_snapshot = good.as_ref().map(|ts| json!({ "synchronized_at": ts }));
                    json!({
                        "source_id": source_id,
                        "source_label": source_label,
                        "standing": "failed",
                        "reconnect_needed": false,
                        "reason": reason,
                        "last_good_read_at": good,
                        "attempted_at": attempted,
                        "state": {
                            "state": "failed",
                            "synchronized_at": attempted,
                            "usable_snapshot": usable_snapshot,
                        },
                    })
                }
                Some(SourceStanding::ReconnectNeeded {
                    last_good_read_at,
                    attempted_at,
                    reason,
                }) => {
                    let good = last_good_read_at.map(unix_to_rfc3339);
                    let attempted = attempted_at.map(unix_to_rfc3339);
                    let usable_snapshot = good.as_ref().map(|ts| json!({ "synchronized_at": ts }));
                    json!({
                        "source_id": source_id,
                        "source_label": source_label,
                        "standing": "reconnect_needed",
                        "reconnect_needed": true,
                        "reason": reason,
                        "last_good_read_at": good,
                        "attempted_at": attempted,
                        "state": {
                            "state": "failed",
                            "synchronized_at": attempted,
                            "usable_snapshot": usable_snapshot,
                        },
                    })
                }
            };
            sources.push(entry);
        }
        sources
    }

    /// The patient page: identity, saved summary when present, and how current the records are.
    pub(crate) fn workspace_body(&self, now: &str, registry: Option<&SourceRegistry>) -> Value {
        json!({
            "patient_workspace_id": self.workspace,
            "display_name": self.holder().display_name,
            "time_zone": PRACTICE_TIME_ZONE,
            "current_time": now,
            "identity": self.identity(now),
            "patient_synopsis": self.synopsis,
            "automation_run": self.automation_run,
            "connected_sources": self.connected_sources(registry),
            "chats": [],
            "active_follow_ups": [],
            "has_past_work": false,
        })
    }

    /// What a physician in the clinic reads of a patient not on their panel:
    /// strictly display name, birth date, and medical record numbers.
    pub(crate) fn clinic_demographics(&self, now: &str) -> Value {
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
            "time_zone": PRACTICE_TIME_ZONE,
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

fn failed_source_json(
    source_id: &str,
    source_label: &str,
    last_good_pulled_at: Option<i64>,
    missing_candidates: &[String],
) -> Value {
    match last_good_pulled_at {
        Some(pulled_at) => {
            let good = unix_to_rfc3339(pulled_at);
            json!({
                "source_id": source_id,
                "source_label": source_label,
                "standing": "failed",
                "reconnect_needed": false,
                "last_good_read_at": good,
                "missing_candidates": missing_candidates,
                "state": {
                    "state": "failed",
                    "usable_snapshot": {
                        "synchronized_at": good,
                    },
                },
            })
        }
        None => json!({
            "source_id": source_id,
            "source_label": source_label,
            "standing": "partial",
            "reconnect_needed": false,
            "last_good_read_at": null,
            "missing_candidates": missing_candidates,
            "state": {
                "state": "failed",
                "synchronized_at": null,
                "usable_snapshot": null,
            },
        }),
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
) -> Result<Vec<Patient>, DirectoryError> {
    let (records, mut standings, mut synopses, mut automation_runs) = tokio::try_join!(
        directory.source_records(workspaces),
        directory.standings(workspaces),
        directory.synopses(workspaces),
        directory.automation_runs(workspaces),
    )?;
    let mut patients: Vec<Patient> = records
        .into_iter()
        .filter_map(|(workspace, ws_records)| {
            let ws_standings = standings.remove(&workspace).unwrap_or_default();
            let synopsis = synopses.remove(&workspace);
            let automation_run = automation_runs.remove(&workspace);
            Patient::from_entry(
                (workspace, ws_records),
                ws_standings,
                synopsis,
                automation_run,
            )
        })
        .collect();
    patients.sort_by(|left, right| left.sort_key().cmp(&right.sort_key()));
    Ok(patients)
}

/// One patient any source holds.
pub(crate) async fn read_patient(
    directory: &dyn PatientDirectory,
    workspace: &str,
) -> Result<Option<Patient>, DirectoryError> {
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
) -> Result<Vec<Patient>, DirectoryError> {
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
    use std::error::Error as _;

    use super::*;

    struct FailingStandingsDirectory;

    #[async_trait]
    impl PatientDirectory for FailingStandingsDirectory {
        async fn pulls(
            &self,
            _workspaces: &[String],
        ) -> Result<BTreeMap<String, Vec<SourcePull>>, DirectoryError> {
            Ok(BTreeMap::new())
        }

        async fn standings(
            &self,
            _workspaces: &[String],
        ) -> Result<BTreeMap<String, BTreeMap<String, SourceStanding>>, DirectoryError> {
            Err(DirectoryError::from(SourceStatusError::EmptyIdentity {
                field: "workspace_id",
            }))
        }
    }

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
    async fn directory_error_preserves_source_status_cause() {
        let err = read_patient(&FailingStandingsDirectory, "pw-test")
            .await
            .expect_err("standings failure must surface");
        let source = err
            .source()
            .expect("DirectoryError::Standings preserves SourceStatusError cause");
        let typed = source
            .downcast_ref::<SourceStatusError>()
            .expect("cause downcasts to SourceStatusError");
        assert!(matches!(
            typed,
            SourceStatusError::EmptyIdentity {
                field: "workspace_id"
            }
        ));
    }

    #[test]
    fn connected_sources_projects_pull_less_linked_standings_and_skips_not_yet_read() {
        let mut standings = BTreeMap::new();
        standings.insert(
            "fixture-chart".to_owned(),
            SourceStanding::UpToDate {
                last_good_read_at: 1_787_846_400,
                missing_candidates: Vec::new(),
            },
        );
        standings.insert(
            "regional-labs".to_owned(),
            SourceStanding::Failed {
                last_good_read_at: None,
                attempted_at: 1_787_846_700,
                reason: "upstream timeout".to_owned(),
            },
        );
        standings.insert("never-attempted".to_owned(), SourceStanding::NotYetRead);

        let ws = crate::WORKSPACE_IDS[0];
        let patient = Patient::from_entry(
            (
                ws.to_owned(),
                vec![PatientSourceRecord::from(fixture_pull(ws))],
            ),
            standings,
            None,
            None,
        )
        .unwrap();
        let connected = patient.connected_sources(None);
        assert_eq!(connected.len(), 2);
        assert_eq!(connected[0]["source_id"], "fixture-chart");
        assert_eq!(connected[0]["standing"], "up_to_date");
        assert_eq!(connected[1]["source_id"], "regional-labs");
        assert_eq!(connected[1]["source_label"], "regional-labs");
        assert_eq!(connected[1]["standing"], "failed");
        assert_eq!(connected[1]["reason"], "upstream timeout");
        assert_eq!(connected[1]["last_good_read_at"], Value::Null);
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
        let body = patient.workspace_body("2026-08-28T16:00:00Z", None);
        let sources = body["connected_sources"].as_array().expect("sources array");
        assert_eq!(sources[0]["state"]["state"], "failed");
        assert_eq!(
            sources[0]["state"]["usable_snapshot"]["synchronized_at"],
            "2026-08-27T16:00:00.000Z"
        );
        assert_eq!(sources[1]["state"]["state"], "not_yet_pulled");
        assert!(sources[1]["state"].get("synchronized_at").is_none());
    }
}
