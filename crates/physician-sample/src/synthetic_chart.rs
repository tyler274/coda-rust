//! The synthetic source: charts from a built patient bank, read the way an
//! adapter reads a connected EHR. This is the one test-only piece of the chart
//! path; everything past [`PatientSource`] is the real product.
//!
//! The adapter reads the bank's `patients/` folder and nothing else. The
//! ledgers beside it hold what the charts were written from, so no part of the
//! product ever sees them.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use local_product::physician_sources::{
    FailedPart, MedicalRecordNumber, PatientSource, ReconnectReceipt, SourceAdapterError,
    SourceCommandKey, SourcePatient, SourcePullOutput, SourceRecordKind, SourceRevision,
};
use local_product::runtime::Store;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Identity of the synthetic chart adapter, including developer-plan admission.
pub const SOURCE_ID: &str = "synthetic-charts";
const SOURCE_LABEL: &str = "Synthetic chart";

/// The fields that date a FHIR chart record, in the source's names.
pub(crate) const DATE_FIELDS: [&str; 9] = [
    "abatementDateTime",
    "authoredOn",
    "date",
    "effectiveDateTime",
    "issued",
    "occurrenceDateTime",
    "onsetDateTime",
    "performedDateTime",
    "recordedDate",
];

/// Why a synthetic FHIR chart record line failed adapter validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ChartRecordError {
    /// Structural JSON, missing FHIR fields, or `SourceRevision` validation error.
    Malformed(String),
    /// Non-text `contentType`, external URL, invalid base64, or non-UTF-8 attachment payload.
    UnsupportedAttachment(String),
}

/// Parses one NDJSON FHIR chart line into a validated [`SourceRevision`] using
/// the synthetic chart adapter's single-owner [`crate::synthetic_chart_fhir`] rules.
pub(crate) fn parse_source_revision(
    workspace: &str,
    line: &str,
) -> Result<SourceRevision, ChartRecordError> {
    use local_product::physician_sources::adapter::RecordFilingError;

    crate::synthetic_chart_fhir::source_revision(workspace, SOURCE_ID, line).map_err(
        |err| match err {
            RecordFilingError::UnsupportedAttachment { .. }
            | RecordFilingError::AttachmentDecode { .. }
            | RecordFilingError::UnavailableAttachment { .. }
            | RecordFilingError::ExternalAttachment { .. }
            | RecordFilingError::InvalidBase64 { .. }
            | RecordFilingError::InvalidUtf8 { .. } => {
                ChartRecordError::UnsupportedAttachment(err.to_string())
            }
            RecordFilingError::InvalidJson(_)
            | RecordFilingError::MissingField(_)
            | RecordFilingError::InvalidRevision(_)
            | RecordFilingError::PatientMismatch { .. } => {
                ChartRecordError::Malformed(err.to_string())
            }
        },
    )
}

/// Controllable connector condition for [`SyntheticCharts`] in tests and local proof runs.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum SyntheticSourceMode {
    /// Normal operation: every patient's NDJSON files read completely.
    #[default]
    Healthy,
    /// Source credentials or authorization have lapsed until [`PatientSource::reconnect`] runs.
    LapsedAccess {
        /// Access failure message reported by the adapter.
        reason: String,
    },
    /// Whole-source or whole-patient outage where reads fail before returning records.
    Outage {
        /// Outage failure message reported by the adapter.
        reason: String,
    },
    /// Partial pull where named FHIR resource families fail while all other families succeed.
    Partial {
        /// Resource family names (such as `"DocumentReference"` or `"Observation"`) that fail.
        failed_families: BTreeSet<String>,
        /// Failure reason reported for each failed family.
        reason: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct SavedCommandReceipt {
    physician_id: String,
    idempotency_key: String,
    receipt: ReconnectReceipt,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
struct PersistedPracticeState {
    #[serde(default)]
    practice_mode: SyntheticSourceMode,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    patient_modes: BTreeMap<String, SyntheticSourceMode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    command_receipts: Vec<SavedCommandReceipt>,
    #[serde(default)]
    reconnect_effects: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct PracticeModeState {
    practice_mode: SyntheticSourceMode,
    patient_modes: BTreeMap<String, SyntheticSourceMode>,
    command_receipts: BTreeMap<(String, String), ReconnectReceipt>,
    reconnect_effects: usize,
}

impl PracticeModeState {
    fn from_persisted(persisted: PersistedPracticeState) -> Self {
        Self {
            practice_mode: persisted.practice_mode,
            patient_modes: persisted.patient_modes,
            command_receipts: persisted
                .command_receipts
                .into_iter()
                .map(|entry| ((entry.physician_id, entry.idempotency_key), entry.receipt))
                .collect(),
            reconnect_effects: persisted.reconnect_effects,
        }
    }

    fn into_persisted(self) -> PersistedPracticeState {
        PersistedPracticeState {
            practice_mode: self.practice_mode,
            patient_modes: self.patient_modes,
            command_receipts: self
                .command_receipts
                .into_iter()
                .map(
                    |((physician_id, idempotency_key), receipt)| SavedCommandReceipt {
                        physician_id,
                        idempotency_key,
                        receipt,
                    },
                )
                .collect(),
            reconnect_effects: self.reconnect_effects,
        }
    }

    fn apply_practice_mode(&mut self, mode: SyntheticSourceMode) {
        self.patient_modes.clear();
        self.practice_mode = mode;
    }

    fn apply_patient_mode(&mut self, source_patient_id: &str, mode: SyntheticSourceMode) {
        match mode {
            SyntheticSourceMode::Healthy => {
                self.patient_modes.remove(source_patient_id);
            }
            other @ (SyntheticSourceMode::LapsedAccess { .. }
            | SyntheticSourceMode::Outage { .. }
            | SyntheticSourceMode::Partial { .. }) => {
                self.patient_modes
                    .insert(source_patient_id.to_owned(), other);
            }
        }
    }

    fn effective_mode(&self, source_patient_id: &str) -> SyntheticSourceMode {
        self.patient_modes
            .get(source_patient_id)
            .unwrap_or(&self.practice_mode)
            .clone()
    }

    fn apply_reconnect(
        &mut self,
        practice_id: &str,
        now: i64,
        command_key: Option<SourceCommandKey<'_>>,
    ) -> Result<ReconnectReceipt, SourceAdapterError> {
        if let Some(cmd_key) = command_key
            && let Some(existing) = self
                .command_receipts
                .get(&(
                    cmd_key.physician_id.to_owned(),
                    cmd_key.idempotency_key.to_owned(),
                ))
                .cloned()
        {
            return Ok(existing);
        }
        if let SyntheticSourceMode::Outage { reason } = &self.practice_mode {
            return Err(SourceAdapterError::Unavailable {
                reason: reason.clone(),
            });
        }
        if matches!(self.practice_mode, SyntheticSourceMode::LapsedAccess { .. }) {
            self.practice_mode = SyntheticSourceMode::Healthy;
        }
        self.patient_modes
            .retain(|_, mode| !matches!(mode, SyntheticSourceMode::LapsedAccess { .. }));
        self.reconnect_effects += 1;
        let receipt = ReconnectReceipt {
            source_id: SOURCE_ID.to_owned(),
            practice_id: practice_id.to_owned(),
            reconnected_at: now,
        };
        if let Some(cmd_key) = command_key {
            self.command_receipts.insert(
                (
                    cmd_key.physician_id.to_owned(),
                    cmd_key.idempotency_key.to_owned(),
                ),
                receipt.clone(),
            );
        }
        Ok(receipt)
    }
}

#[derive(Clone)]
enum StateBackend {
    Database(Store),
    Memory(Arc<Mutex<BTreeMap<String, PracticeModeState>>>),
}

/// A built patient bank on disk, with shared connector modes and command receipts persisted in
/// Postgres via [`Store`] (or held in memory for standalone unit tests without a database).
#[derive(Clone)]
pub struct SyntheticCharts {
    patients: PathBuf,
    backend: StateBackend,
}

impl SyntheticCharts {
    /// Opens the bank built at `bank` with an in-memory mode ledger for standalone unit tests.
    pub fn open(bank: &Path) -> Self {
        Self {
            patients: bank.join("patients"),
            backend: StateBackend::Memory(Arc::new(Mutex::new(BTreeMap::new()))),
        }
    }

    /// Opens the bank built at `bank`, persisting shared connector modes and reconnect receipts
    /// across instances in Postgres via `store`.
    pub fn open_with_store(bank: &Path, store: Store) -> Self {
        Self {
            patients: bank.join("patients"),
            backend: StateBackend::Database(store),
        }
    }

    /// Opens the bank built at `bank`, attaching `store` when configured.
    pub fn open_with_optional_store(bank: &Path, store: Option<Store>) -> Self {
        match store {
            Some(store) => Self::open_with_store(bank, store),
            None => Self::open(bank),
        }
    }

    /// Returns a copy backed by `store` so connector modes and command receipts persist in Postgres.
    pub fn with_store(mut self, store: Store) -> Self {
        self.backend = StateBackend::Database(store);
        self
    }

    /// Whether this adapter persists shared connector state in Postgres via [`Store`].
    pub fn uses_store(&self) -> bool {
        matches!(self.backend, StateBackend::Database(_))
    }

    async fn read_practice_state(
        &self,
        practice_id: &str,
    ) -> Result<PracticeModeState, SourceAdapterError> {
        match &self.backend {
            StateBackend::Database(store) => {
                let persisted: PersistedPracticeState = store
                    .read_source_adapter_state(practice_id, SOURCE_ID)
                    .await?;
                Ok(PracticeModeState::from_persisted(persisted))
            }
            StateBackend::Memory(memory) => {
                let guard = memory.lock().expect("synthetic source memory lock");
                Ok(guard.get(practice_id).cloned().unwrap_or_default())
            }
        }
    }

    async fn mutate_practice_state<T>(
        &self,
        practice_id: &str,
        mutate: impl FnOnce(&mut PracticeModeState) -> Result<T, SourceAdapterError>,
    ) -> Result<T, SourceAdapterError> {
        match &self.backend {
            StateBackend::Database(store) => {
                store
                    .mutate_source_adapter_state(
                        practice_id,
                        SOURCE_ID,
                        |persisted: &mut PersistedPracticeState| {
                            let mut state =
                                PracticeModeState::from_persisted(std::mem::take(persisted));
                            let result = mutate(&mut state);
                            *persisted = state.into_persisted();
                            result
                        },
                    )
                    .await
            }
            StateBackend::Memory(memory) => {
                let mut guard = memory.lock().expect("synthetic source memory lock");
                let mut candidate = guard.get(practice_id).cloned().unwrap_or_default();
                let output = mutate(&mut candidate)?;
                guard.insert(practice_id.to_owned(), candidate);
                Ok(output)
            }
        }
    }

    /// Sets the practice-wide connector mode for `practice_id`, clearing any per-patient overrides
    /// in that practice so the practice mode applies uniformly.
    pub async fn set_practice_mode(
        &self,
        practice_id: &str,
        mode: SyntheticSourceMode,
    ) -> Result<(), SourceAdapterError> {
        self.mutate_practice_state(practice_id, |state| {
            state.apply_practice_mode(mode);
            Ok(())
        })
        .await
    }

    /// Sets a patient-specific connector mode override for `(practice_id, source_patient_id)`.
    pub async fn set_patient_mode(
        &self,
        practice_id: &str,
        source_patient_id: &str,
        mode: SyntheticSourceMode,
    ) -> Result<(), SourceAdapterError> {
        self.mutate_practice_state(practice_id, |state| {
            state.apply_patient_mode(source_patient_id, mode);
            Ok(())
        })
        .await
    }

    /// Reads the practice-wide connector mode configured for `practice_id`.
    pub async fn practice_mode(
        &self,
        practice_id: &str,
    ) -> Result<SyntheticSourceMode, SourceAdapterError> {
        let state = self.read_practice_state(practice_id).await?;
        Ok(state.practice_mode)
    }

    /// Number of non-deduplicated reconnect effects performed for `practice_id`.
    pub async fn reconnect_effect_count(
        &self,
        practice_id: &str,
    ) -> Result<usize, SourceAdapterError> {
        let state = self.read_practice_state(practice_id).await?;
        Ok(state.reconnect_effects)
    }

    /// Resolves the effective connector mode for `(practice_id, source_patient_id)`.
    pub async fn effective_mode(
        &self,
        practice_id: &str,
        source_patient_id: &str,
    ) -> Result<SyntheticSourceMode, SourceAdapterError> {
        let state = self.read_practice_state(practice_id).await?;
        Ok(state.effective_mode(source_patient_id))
    }

    fn acquire_chart(
        &self,
        source_patient_id: &str,
        workspace: &str,
        failed_families: &BTreeSet<String>,
        reason: &str,
    ) -> SourcePullOutput {
        let patient = if failed_families.contains("Patient") {
            None
        } else {
            match self.read_patient_sync(source_patient_id) {
                Ok(patient) => Some(patient),
                Err(error) => return SourcePullOutput::Failed { error },
            }
        };

        let chart = self.patients.join(source_patient_id);
        let entries = match fs::read_dir(&chart) {
            Ok(entries) => entries,
            Err(source) => {
                return SourcePullOutput::Failed {
                    error: SourceAdapterError::Io {
                        path: chart,
                        source,
                    },
                };
            }
        };
        let mut files = Vec::new();
        for entry in entries {
            match entry {
                Ok(entry) => files.push(entry.path()),
                Err(source) => {
                    return SourcePullOutput::Failed {
                        error: SourceAdapterError::Io {
                            path: chart,
                            source,
                        },
                    };
                }
            }
        }
        files.sort();

        let mut failed_parts: Vec<FailedPart> = failed_families
            .iter()
            .map(|family| FailedPart {
                part: family.clone(),
                reason: reason.to_owned(),
            })
            .collect();
        let mut succeeded_family_names = BTreeSet::new();
        let mut records = Vec::new();
        for file in files {
            let family = file
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or_default();
            if failed_families.contains(family) {
                continue;
            }
            let text = match read(&file) {
                Ok(text) => text,
                Err(error) => {
                    failed_parts.push(FailedPart {
                        part: family.to_owned(),
                        reason: error.to_string(),
                    });
                    continue;
                }
            };
            let failures_before = failed_parts.len();
            for (index, line) in text.lines().enumerate() {
                match crate::synthetic_chart_fhir::patient_source_revision(
                    workspace,
                    SOURCE_ID,
                    source_patient_id,
                    line,
                ) {
                    Ok(revision) => {
                        records.push(revision);
                    }
                    Err(source) => failed_parts.push(FailedPart {
                        part: format!("{family}:{}", index + 1),
                        reason: source.to_string(),
                    }),
                }
            }
            if failed_parts.len() == failures_before {
                succeeded_family_names.insert(family.to_owned());
            }
        }

        let succeeded_kinds = succeeded_family_names
            .into_iter()
            .map(|kind| SourceRecordKind {
                kind,
                category: None,
            })
            .collect();

        if failed_parts.is_empty()
            && let Some(patient) = patient.clone()
        {
            return SourcePullOutput::Complete { patient, records };
        }

        SourcePullOutput::Partial {
            patient,
            succeeded_kinds,
            records,
            failed_parts,
        }
    }

    fn read_patient_sync(
        &self,
        source_patient_id: &str,
    ) -> Result<SourcePatient, SourceAdapterError> {
        let path = self.patients.join(source_patient_id).join("Patient.ndjson");
        let text = read(&path)?;
        let line =
            text.lines()
                .next()
                .ok_or_else(|| SourceAdapterError::MissingPatientIdentity {
                    context: path.display().to_string(),
                    detail: "no Patient record".to_owned(),
                })?;
        source_patient(source_patient_id.to_owned(), line, &path)
    }
}

#[async_trait]
impl PatientSource for SyntheticCharts {
    fn source_id(&self) -> &str {
        SOURCE_ID
    }

    fn source_label(&self) -> &str {
        SOURCE_LABEL
    }

    async fn patient_ids(&self) -> Result<Vec<String>, SourceAdapterError> {
        let entries = fs::read_dir(&self.patients).map_err(|source| SourceAdapterError::Io {
            path: self.patients.clone(),
            source,
        })?;
        let mut ids = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|source| SourceAdapterError::Io {
                path: self.patients.clone(),
                source,
            })?;
            ids.push(entry.file_name().to_string_lossy().into_owned());
        }
        ids.sort();
        Ok(ids)
    }

    async fn patient(&self, source_patient_id: &str) -> Result<SourcePatient, SourceAdapterError> {
        self.read_patient_sync(source_patient_id)
    }

    async fn records(
        &self,
        patient: &SourcePatient,
        workspace: &str,
    ) -> Result<Vec<SourceRevision>, SourceAdapterError> {
        match self.acquire_chart(&patient.source_patient_id, workspace, &BTreeSet::new(), "") {
            SourcePullOutput::Complete { records, .. } => Ok(records),
            SourcePullOutput::Partial { failed_parts, .. } => {
                Err(SourceAdapterError::Incomplete { failed_parts })
            }
            SourcePullOutput::Failed { error } | SourcePullOutput::LapsedAccess { error } => {
                Err(error)
            }
        }
    }

    async fn pull_patient(
        &self,
        practice_id: &str,
        source_patient_id: &str,
        workspace: &str,
    ) -> SourcePullOutput {
        let mode = match self.effective_mode(practice_id, source_patient_id).await {
            Ok(mode) => mode,
            Err(error) => return SourcePullOutput::Failed { error },
        };
        match mode {
            SyntheticSourceMode::Healthy => {
                self.acquire_chart(source_patient_id, workspace, &BTreeSet::new(), "")
            }
            SyntheticSourceMode::LapsedAccess { reason } => SourcePullOutput::LapsedAccess {
                error: SourceAdapterError::LapsedAccess { reason },
            },
            SyntheticSourceMode::Outage { reason } => SourcePullOutput::Failed {
                error: SourceAdapterError::Unavailable { reason },
            },
            SyntheticSourceMode::Partial {
                failed_families,
                reason,
            } => self.acquire_chart(source_patient_id, workspace, &failed_families, &reason),
        }
    }

    async fn reconnect(
        &self,
        practice_id: &str,
        now: i64,
        command_key: Option<SourceCommandKey<'_>>,
    ) -> Result<ReconnectReceipt, SourceAdapterError> {
        self.mutate_practice_state(practice_id, |state| {
            state.apply_reconnect(practice_id, now, command_key)
        })
        .await
    }
}

fn read(path: &Path) -> Result<String, SourceAdapterError> {
    fs::read_to_string(path).map_err(|source| SourceAdapterError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn source_patient(
    source_patient_id: String,
    line: &str,
    path: &Path,
) -> Result<SourcePatient, SourceAdapterError> {
    let resource: Value =
        serde_json::from_str(line).map_err(|source| SourceAdapterError::InvalidPatientJson {
            context: path.display().to_string(),
            source,
        })?;
    if resource["resourceType"] != "Patient" {
        return Err(SourceAdapterError::MissingPatientIdentity {
            context: path.display().to_string(),
            detail: "identity resource is not a Patient".into(),
        });
    }
    local_product::physician_sources::adapter::validate_fhir_patient_binding(
        &resource,
        &source_patient_id,
    )
    .map_err(|source| SourceAdapterError::RecordFiling {
        context: path.display().to_string(),
        source,
    })?;
    let name = &resource["name"][0];
    let given = name["given"].as_array().into_iter().flatten();
    let parts: Vec<&str> = given
        .chain(std::iter::once(&name["family"]))
        .filter_map(Value::as_str)
        .collect();
    if parts.is_empty() {
        return Err(SourceAdapterError::MissingPatientIdentity {
            context: path.display().to_string(),
            detail: "patient has no name".to_owned(),
        });
    }
    Ok(SourcePatient {
        source_patient_id,
        display_name: parts.join(" "),
        birth_date: resource["birthDate"].as_str().map(str::to_owned),
        sex_or_gender: resource["gender"].as_str().map(str::to_owned),
        medical_record_numbers: resource["identifier"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|identifier| {
                Some(MedicalRecordNumber {
                    system: identifier["system"].as_str()?.to_owned(),
                    value: identifier["value"].as_str()?.to_owned(),
                })
            })
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::atomic::{AtomicU64, Ordering};

    const PATIENT: &str = r#"{"resourceType":"Patient","id":"MRN1","meta":{"lastUpdated":"2026-08-27T00:00:00+00:00"},"identifier":[{"system":"urn:mrn","value":"1"}],"name":[{"family":"Louis","given":["Mary","Ann"]}],"gender":"female","birthDate":"1972-03-18"}"#;
    const NOTE: &str = r#"{"resourceType":"DocumentReference","id":"note-1","date":"2026-08-12T12:05:00+00:00","content":[{"attachment":{"contentType":"text/plain; charset=utf-8","data":"U2VlbiBpbiBjbGluaWMu"}}]}"#;
    const VISIT: &str = r#"{"resourceType":"Encounter","id":"visit-1","period":{"start":"2026-07-26T14:15:00+00:00","end":"2026-07-26T15:00:00+00:00"}}"#;

    fn bank(name: &str, note: &str) -> PathBuf {
        static NEXT_BANK: AtomicU64 = AtomicU64::new(1);
        let seq = NEXT_BANK.fetch_add(1, Ordering::Relaxed);
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let bank = std::env::temp_dir().join(format!(
            "coda-synthetic-chart-{name}-{}-{nanos:x}-{seq}",
            std::process::id()
        ));
        let chart = bank.join("patients/MRN1");
        fs::create_dir_all(&chart).unwrap();
        fs::write(chart.join("Patient.ndjson"), format!("{PATIENT}\n")).unwrap();
        fs::write(chart.join("DocumentReference.ndjson"), format!("{note}\n")).unwrap();
        fs::write(chart.join("Encounter.ndjson"), format!("{VISIT}\n")).unwrap();
        bank
    }

    #[tokio::test]
    async fn a_chart_reads_as_coda_records_with_its_exact_bytes() {
        let source = SyntheticCharts::open(&bank("reads", NOTE));
        let ids = source.patient_ids().await.unwrap();
        assert_eq!(ids.len(), 1);
        let patient = &source.patient(&ids[0]).await.unwrap();
        assert_eq!(patient.display_name, "Mary Ann Louis");
        assert_eq!(patient.birth_date.as_deref(), Some("1972-03-18"));
        assert_eq!(patient.sex_or_gender.as_deref(), Some("female"));
        assert_eq!(patient.medical_record_numbers[0].value, "1");

        let records = source.records(patient, "pw-1").await.unwrap();
        let types: Vec<&str> = records.iter().map(|r| r.record_type.as_str()).collect();
        assert_eq!(types, ["DocumentReference", "Encounter", "Patient"]);
        let note = &records[0];
        assert_eq!(note.workspace, "pw-1");
        assert_eq!(
            note.candidate_id,
            "synthetic-charts/DocumentReference/note-1"
        );
        assert_eq!(note.content, NOTE);
        assert_eq!(note.readable_parts[0].text, "Seen in clinic.");
        assert_eq!(note.dates[0].name, "date");
        assert_eq!(note.dates[0].value, "2026-08-12T12:05:00+00:00");
        let visit: Vec<&str> = records[1].dates.iter().map(|d| d.name.as_str()).collect();
        assert_eq!(visit, ["period.start", "period.end"]);
    }

    #[tokio::test]
    #[ignore = "requires `cargo run -p patient-bank -- build`"]
    async fn every_built_chart_reads_in_full() {
        let bank = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/patient-bank/bank");
        let manifest: Value =
            serde_json::from_str(&read(&bank.join("manifest.json")).unwrap()).unwrap();
        let source = SyntheticCharts::open(&bank);
        let ids = source.patient_ids().await.unwrap();
        assert_eq!(
            json!(ids.len()),
            json!(manifest["patients"].as_array().unwrap().len())
        );
        let mut totals = std::collections::BTreeMap::<String, u64>::new();
        for id in &ids {
            let patient = source.patient(id).await.unwrap();
            for record in source.records(&patient, "pw-1").await.unwrap() {
                assert!(!record.dates.is_empty(), "{}", record.candidate_id);
                assert_eq!(
                    record.record_type == "DocumentReference",
                    record.record_type == "DocumentReference" && !record.readable_parts.is_empty(),
                    "{}",
                    record.candidate_id
                );
                *totals.entry(record.record_type).or_default() += 1;
            }
        }
        assert_eq!(json!(totals), manifest["totals"]);
    }

    #[tokio::test]
    async fn an_unread_attachment_keeps_its_parent_and_chart_siblings() {
        let scan = NOTE.replace("text/plain; charset=utf-8", "image/tiff");
        let source = SyntheticCharts::open(&bank("scan", &scan));
        let SourcePullOutput::Complete { records, .. } =
            source.pull_patient("practice", "MRN1", "pw-1").await
        else {
            panic!("completed acquisition must refresh despite unreadable attachment content");
        };
        assert_eq!(records.len(), 3);
        let note = records
            .iter()
            .find(|record| record.record_type == "DocumentReference")
            .unwrap();
        assert_eq!(note.content, scan);
        assert!(note.readable_parts.is_empty());
        assert_eq!(note.envelope.as_ref().unwrap().unavailable_parts.len(), 1);
    }

    #[tokio::test]
    async fn controllable_modes_simulate_lapsed_access_partial_outage_and_reconnect() {
        let source = SyntheticCharts::open(&bank("modes", NOTE));

        source
            .set_practice_mode(
                "practice-a",
                SyntheticSourceMode::LapsedAccess {
                    reason: "oauth refresh token expired".into(),
                },
            )
            .await
            .unwrap();
        match source.pull_patient("practice-a", "MRN1", "pw-1").await {
            SourcePullOutput::LapsedAccess { error } => {
                assert_eq!(error.to_string(), "oauth refresh token expired");
            }
            other => panic!("expected LapsedAccess, got {other:?}"),
        }

        let cmd_key = SourceCommandKey {
            physician_id: "dr-1",
            idempotency_key: "reconn-1",
        };
        let receipt = source
            .reconnect("practice-a", 1_788_000_000, Some(cmd_key))
            .await
            .unwrap();
        assert_eq!(receipt.source_id, SOURCE_ID);
        assert_eq!(receipt.practice_id, "practice-a");
        assert_eq!(
            source.reconnect_effect_count("practice-a").await.unwrap(),
            1
        );

        // Retrying on a cloned handle with the same command key recovers the prior receipt without a second effect.
        let cloned = source.clone();
        let replayed = cloned
            .reconnect("practice-a", 1_788_000_999, Some(cmd_key))
            .await
            .unwrap();
        assert_eq!(replayed, receipt);
        assert_eq!(
            cloned.reconnect_effect_count("practice-a").await.unwrap(),
            1
        );
        assert!(matches!(
            source.pull_patient("practice-a", "MRN1", "pw-1").await,
            SourcePullOutput::Complete { records, .. } if records.len() == 3
        ));

        source
            .set_patient_mode(
                "practice-a",
                "MRN1",
                SyntheticSourceMode::Partial {
                    failed_families: BTreeSet::from(["DocumentReference".to_owned()]),
                    reason: "DocumentReference endpoint timed out".into(),
                },
            )
            .await
            .unwrap();
        match source.pull_patient("practice-a", "MRN1", "pw-1").await {
            SourcePullOutput::Partial {
                patient,
                succeeded_kinds,
                records,
                failed_parts,
            } => {
                assert!(patient.is_some());
                let kinds: Vec<&str> = succeeded_kinds.iter().map(|k| k.kind.as_str()).collect();
                assert_eq!(kinds, ["Encounter", "Patient"]);
                let types: Vec<&str> = records.iter().map(|r| r.record_type.as_str()).collect();
                assert_eq!(types, ["Encounter", "Patient"]);
                assert_eq!(
                    failed_parts,
                    vec![FailedPart {
                        part: "DocumentReference".into(),
                        reason: "DocumentReference endpoint timed out".into(),
                    }]
                );
            }
            other => panic!("expected Partial, got {other:?}"),
        }
    }

    #[tokio::test]
    #[ignore = "requires explicit local Postgres"]
    async fn reconnect_receipts_and_modes_persist_in_postgres_across_separate_bank_directories() {
        let db_url = std::env::var("PHYSICIAN_TEST_DATABASE_URL")
            .or_else(|_| std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL"))
            .expect("set PHYSICIAN_TEST_DATABASE_URL or PHYSICIAN_RUNTIME_TEST_DATABASE_URL");
        let store = Store::connect(&db_url).await.unwrap();
        store.migrate().await.unwrap();

        let practice = format!(
            "practice-shared-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let bank_a = bank("reopened-site-a", NOTE);
        let bank_b = bank("reopened-site-b", NOTE);
        assert_ne!(bank_a, bank_b);

        let cmd_key_a = SourceCommandKey {
            physician_id: "dr-a",
            idempotency_key: "idem-cross-instance",
        };

        let first_receipt = {
            let instance_a = SyntheticCharts::open_with_store(&bank_a, store.clone());
            instance_a
                .set_practice_mode(
                    &practice,
                    SyntheticSourceMode::LapsedAccess {
                        reason: "token expired on site A".into(),
                    },
                )
                .await
                .unwrap();
            let receipt = instance_a
                .reconnect(&practice, 1_788_010_000, Some(cmd_key_a))
                .await
                .unwrap();
            assert_eq!(
                instance_a.reconnect_effect_count(&practice).await.unwrap(),
                1
            );
            receipt
        };

        // Remove Site A's bank directory completely so Site B shares ONLY Postgres.
        fs::remove_dir_all(&bank_a).unwrap();
        assert!(!bank_a.exists());

        let instance_b = SyntheticCharts::open_with_store(&bank_b, store);
        assert_eq!(
            instance_b.reconnect_effect_count(&practice).await.unwrap(),
            1
        );
        assert_eq!(
            instance_b.practice_mode(&practice).await.unwrap(),
            SyntheticSourceMode::Healthy
        );

        // Even if the practice later enters Outage before retry, the cached receipt is recovered
        // from Postgres without repeating the underlying reconnect effect.
        instance_b
            .set_practice_mode(
                &practice,
                SyntheticSourceMode::Outage {
                    reason: "upstream maintenance".into(),
                },
            )
            .await
            .unwrap();
        let recovered = instance_b
            .reconnect(&practice, 1_788_019_999, Some(cmd_key_a))
            .await
            .unwrap();
        assert_eq!(recovered, first_receipt);
        assert_eq!(recovered.reconnected_at, 1_788_010_000);
        assert_eq!(
            instance_b.reconnect_effect_count(&practice).await.unwrap(),
            1
        );

        // Restore healthy mode and verify a second physician reusing the same key string executes
        // an independent reconnect effect scoped to `(practice_id, physician_id, idempotency_key)`.
        instance_b
            .set_practice_mode(&practice, SyntheticSourceMode::Healthy)
            .await
            .unwrap();
        let cmd_key_b = SourceCommandKey {
            physician_id: "dr-b",
            idempotency_key: "idem-cross-instance",
        };
        let second_receipt = instance_b
            .reconnect(&practice, 1_788_025_000, Some(cmd_key_b))
            .await
            .unwrap();
        assert_eq!(second_receipt.reconnected_at, 1_788_025_000);
        assert_eq!(
            instance_b.reconnect_effect_count(&practice).await.unwrap(),
            2
        );

        let _ = fs::remove_dir_all(&bank_b);
    }
}
