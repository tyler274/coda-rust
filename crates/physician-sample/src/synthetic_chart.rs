//! The synthetic source: charts from a built patient bank, read the way an
//! adapter reads a connected EHR. This is the one test-only piece of the chart
//! path; everything past [`PatientSource`] is the real product.
//!
//! The adapter reads the bank's `patients/` folder and nothing else. The
//! ledgers beside it hold what the charts were written from, so no part of the
//! product ever sees them.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use async_trait::async_trait;
use local_product::physician_sources::{
    FailedPart, MedicalRecordNumber, PatientSource, ReconnectReceipt, SourceAdapterError,
    SourceCommandKey, SourcePatient, SourcePullOutput, SourceRecordKind, SourceRevision,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Identity of the synthetic chart adapter, including developer-plan admission.
pub const SOURCE_ID: &str = "synthetic-charts";
const SOURCE_LABEL: &str = "Synthetic chart";

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
struct SavedPatientMode {
    practice_id: String,
    source_patient_id: String,
    mode: SyntheticSourceMode,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct SavedCommandReceipt {
    practice_id: String,
    physician_id: String,
    idempotency_key: String,
    receipt: ReconnectReceipt,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
struct PersistedSourceState {
    #[serde(default)]
    practice_modes: BTreeMap<String, SyntheticSourceMode>,
    #[serde(default)]
    patient_modes: Vec<SavedPatientMode>,
    #[serde(default)]
    command_receipts: Vec<SavedCommandReceipt>,
    #[serde(default)]
    reconnect_effects: BTreeMap<String, usize>,
}

#[derive(Default)]
struct ModeTable {
    practice_modes: BTreeMap<String, SyntheticSourceMode>,
    patient_modes: BTreeMap<(String, String), SyntheticSourceMode>,
    command_receipts: BTreeMap<(String, String, String), ReconnectReceipt>,
    reconnect_effects: BTreeMap<String, usize>,
}

impl ModeTable {
    fn from_persisted(persisted: PersistedSourceState) -> Self {
        Self {
            practice_modes: persisted.practice_modes,
            patient_modes: persisted
                .patient_modes
                .into_iter()
                .map(|entry| ((entry.practice_id, entry.source_patient_id), entry.mode))
                .collect(),
            command_receipts: persisted
                .command_receipts
                .into_iter()
                .map(|entry| {
                    (
                        (entry.practice_id, entry.physician_id, entry.idempotency_key),
                        entry.receipt,
                    )
                })
                .collect(),
            reconnect_effects: persisted.reconnect_effects,
        }
    }

    fn into_persisted(self) -> PersistedSourceState {
        PersistedSourceState {
            practice_modes: self.practice_modes,
            patient_modes: self
                .patient_modes
                .into_iter()
                .map(
                    |((practice_id, source_patient_id), mode)| SavedPatientMode {
                        practice_id,
                        source_patient_id,
                        mode,
                    },
                )
                .collect(),
            command_receipts: self
                .command_receipts
                .into_iter()
                .map(|((practice_id, physician_id, idempotency_key), receipt)| {
                    SavedCommandReceipt {
                        practice_id,
                        physician_id,
                        idempotency_key,
                        receipt,
                    }
                })
                .collect(),
            reconnect_effects: self.reconnect_effects,
        }
    }
}

fn ledger_paths(bank: &Path) -> (PathBuf, PathBuf) {
    let canonical = fs::canonicalize(bank).unwrap_or_else(|_| bank.to_path_buf());
    let digest = local_product::sha256_hex(canonical.as_os_str().as_encoded_bytes());
    let temp = std::env::temp_dir();
    (
        temp.join(format!("coda-synthetic-source-{digest}.json")),
        temp.join(format!("coda-synthetic-source-{digest}.lock")),
    )
}

fn process_lock_for(lock_path: &Path) -> Arc<Mutex<()>> {
    static LOCKS: OnceLock<Mutex<HashMap<PathBuf, Arc<Mutex<()>>>>> = OnceLock::new();
    let mut map = LOCKS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .expect("synthetic source process-lock registry");
    Arc::clone(
        map.entry(lock_path.to_path_buf())
            .or_insert_with(|| Arc::new(Mutex::new(()))),
    )
}

/// A built patient bank on disk, with durable connector modes and command receipts shared across
/// adapter instances opened against the same bank path.
#[derive(Clone)]
pub struct SyntheticCharts {
    patients: PathBuf,
    state_path: PathBuf,
    lock_path: PathBuf,
    process_lock: Arc<Mutex<()>>,
}

impl SyntheticCharts {
    /// Opens the bank built at `bank`.
    pub fn open(bank: &Path) -> Self {
        let (state_path, lock_path) = ledger_paths(bank);
        let process_lock = process_lock_for(&lock_path);
        Self {
            patients: bank.join("patients"),
            state_path,
            lock_path,
            process_lock,
        }
    }

    fn load_table_unlocked(&self) -> Result<ModeTable, SourceAdapterError> {
        let bytes = match fs::read(&self.state_path) {
            Ok(bytes) => bytes,
            Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(ModeTable::default()),
            Err(source) => {
                return Err(SourceAdapterError::Io {
                    path: self.state_path.clone(),
                    source,
                });
            }
        };
        let persisted: PersistedSourceState =
            serde_json::from_slice(&bytes).map_err(|source| SourceAdapterError::Io {
                path: self.state_path.clone(),
                source: io::Error::new(io::ErrorKind::InvalidData, source),
            })?;
        Ok(ModeTable::from_persisted(persisted))
    }

    fn read_table(&self) -> Result<ModeTable, SourceAdapterError> {
        let _guard = self
            .process_lock
            .lock()
            .expect("synthetic source process lock");
        if !self.lock_path.exists() && !self.state_path.exists() {
            return Ok(ModeTable::default());
        }
        let lock_file = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(&self.lock_path)
            .map_err(|source| SourceAdapterError::Io {
                path: self.lock_path.clone(),
                source,
            })?;
        lock_file
            .lock_shared()
            .map_err(|source| SourceAdapterError::Io {
                path: self.lock_path.clone(),
                source,
            })?;
        self.load_table_unlocked()
    }

    fn mutate_table<T>(
        &self,
        f: impl FnOnce(&mut ModeTable) -> Result<T, SourceAdapterError>,
    ) -> Result<T, SourceAdapterError> {
        static TMP_SEQ: AtomicU64 = AtomicU64::new(1);

        let _guard = self
            .process_lock
            .lock()
            .expect("synthetic source process lock");
        let lock_file = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(&self.lock_path)
            .map_err(|source| SourceAdapterError::Io {
                path: self.lock_path.clone(),
                source,
            })?;
        lock_file.lock().map_err(|source| SourceAdapterError::Io {
            path: self.lock_path.clone(),
            source,
        })?;

        let mut table = self.load_table_unlocked()?;
        let out = f(&mut table)?;
        let persisted = table.into_persisted();
        let encoded = serde_json::to_vec(&persisted).map_err(|source| SourceAdapterError::Io {
            path: self.state_path.clone(),
            source: io::Error::new(io::ErrorKind::InvalidData, source),
        })?;
        let seq = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
        let tmp_path = self
            .state_path
            .with_extension(format!("tmp.{}.{seq}", std::process::id()));
        fs::write(&tmp_path, encoded).map_err(|source| SourceAdapterError::Io {
            path: tmp_path.clone(),
            source,
        })?;
        fs::rename(&tmp_path, &self.state_path).map_err(|source| SourceAdapterError::Io {
            path: self.state_path.clone(),
            source,
        })?;
        Ok(out)
    }

    /// Sets the practice-wide connector mode for `practice_id`, clearing any per-patient overrides
    /// in that practice so the practice mode applies uniformly.
    pub fn set_practice_mode(&self, practice_id: &str, mode: SyntheticSourceMode) {
        self.mutate_table(|table| {
            table
                .patient_modes
                .retain(|(practice, _), _| practice != practice_id);
            match mode {
                SyntheticSourceMode::Healthy => {
                    table.practice_modes.remove(practice_id);
                }
                other @ (SyntheticSourceMode::LapsedAccess { .. }
                | SyntheticSourceMode::Outage { .. }
                | SyntheticSourceMode::Partial { .. }) => {
                    table.practice_modes.insert(practice_id.to_owned(), other);
                }
            }
            Ok(())
        })
        .expect("persist synthetic source practice mode");
    }

    /// Sets a patient-specific connector mode override for `(practice_id, source_patient_id)`.
    pub fn set_patient_mode(
        &self,
        practice_id: &str,
        source_patient_id: &str,
        mode: SyntheticSourceMode,
    ) {
        self.mutate_table(|table| {
            let key = (practice_id.to_owned(), source_patient_id.to_owned());
            match mode {
                SyntheticSourceMode::Healthy => {
                    table.patient_modes.remove(&key);
                }
                other @ (SyntheticSourceMode::LapsedAccess { .. }
                | SyntheticSourceMode::Outage { .. }
                | SyntheticSourceMode::Partial { .. }) => {
                    table.patient_modes.insert(key, other);
                }
            }
            Ok(())
        })
        .expect("persist synthetic source patient mode");
    }

    /// Reads the practice-wide connector mode configured for `practice_id`.
    pub fn practice_mode(&self, practice_id: &str) -> SyntheticSourceMode {
        let table = self.read_table().expect("read synthetic source state");
        table
            .practice_modes
            .get(practice_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Number of non-deduplicated reconnect effects performed for `practice_id`.
    pub fn reconnect_effect_count(&self, practice_id: &str) -> usize {
        let table = self.read_table().expect("read synthetic source state");
        table
            .reconnect_effects
            .get(practice_id)
            .copied()
            .unwrap_or(0)
    }

    /// Resolves the effective connector mode for `(practice_id, source_patient_id)`.
    pub fn effective_mode(
        &self,
        practice_id: &str,
        source_patient_id: &str,
    ) -> SyntheticSourceMode {
        self.try_effective_mode(practice_id, source_patient_id)
            .expect("read synthetic source state")
    }

    fn try_effective_mode(
        &self,
        practice_id: &str,
        source_patient_id: &str,
    ) -> Result<SyntheticSourceMode, SourceAdapterError> {
        let table = self.read_table()?;
        let key = (practice_id.to_owned(), source_patient_id.to_owned());
        Ok(table
            .patient_modes
            .get(&key)
            .or_else(|| table.practice_modes.get(practice_id))
            .cloned()
            .unwrap_or_default())
    }

    fn partial_pull(
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
            if !family.is_empty() {
                succeeded_family_names.insert(family.to_owned());
            }
            let text = match read(&file) {
                Ok(text) => text,
                Err(error) => return SourcePullOutput::Failed { error },
            };
            for line in text.lines() {
                match crate::synthetic_chart_fhir::source_revision(workspace, SOURCE_ID, line) {
                    Ok(revision) => {
                        succeeded_family_names.insert(revision.record_type.clone());
                        records.push(revision);
                    }
                    Err(source) => {
                        return SourcePullOutput::Failed {
                            error: SourceAdapterError::RecordFiling {
                                context: file.display().to_string(),
                                source,
                            },
                        };
                    }
                }
            }
        }

        let succeeded_kinds = succeeded_family_names
            .into_iter()
            .map(|kind| SourceRecordKind {
                kind,
                category: None,
            })
            .collect();

        let failed_parts = failed_families
            .iter()
            .map(|family| FailedPart {
                part: family.clone(),
                reason: reason.to_owned(),
            })
            .collect();

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
        let chart = self.patients.join(&patient.source_patient_id);
        let entries = fs::read_dir(&chart).map_err(|source| SourceAdapterError::Io {
            path: chart.clone(),
            source,
        })?;
        let mut files = Vec::new();
        for entry in entries {
            files.push(
                entry
                    .map_err(|source| SourceAdapterError::Io {
                        path: chart.clone(),
                        source,
                    })?
                    .path(),
            );
        }
        files.sort();
        let mut records = Vec::new();
        for file in files {
            for line in read(&file)?.lines() {
                records.push(
                    crate::synthetic_chart_fhir::source_revision(workspace, SOURCE_ID, line)
                        .map_err(|source| SourceAdapterError::RecordFiling {
                            context: file.display().to_string(),
                            source,
                        })?,
                );
            }
        }
        Ok(records)
    }

    async fn pull_patient(
        &self,
        practice_id: &str,
        source_patient_id: &str,
        workspace: &str,
    ) -> SourcePullOutput {
        let mode = match self.try_effective_mode(practice_id, source_patient_id) {
            Ok(mode) => mode,
            Err(error) => return SourcePullOutput::Failed { error },
        };
        match mode {
            SyntheticSourceMode::Healthy => match self.patient(source_patient_id).await {
                Ok(patient) => match self.records(&patient, workspace).await {
                    Ok(records) => SourcePullOutput::Complete { patient, records },
                    Err(error) => SourcePullOutput::Failed { error },
                },
                Err(error) => SourcePullOutput::Failed { error },
            },
            SyntheticSourceMode::LapsedAccess { reason } => SourcePullOutput::LapsedAccess {
                error: SourceAdapterError::LapsedAccess { reason },
            },
            SyntheticSourceMode::Outage { reason } => SourcePullOutput::Failed {
                error: SourceAdapterError::Unavailable { reason },
            },
            SyntheticSourceMode::Partial {
                failed_families,
                reason,
            } => {
                if failed_families.is_empty() {
                    match self.patient(source_patient_id).await {
                        Ok(patient) => match self.records(&patient, workspace).await {
                            Ok(records) => SourcePullOutput::Complete { patient, records },
                            Err(error) => SourcePullOutput::Failed { error },
                        },
                        Err(error) => SourcePullOutput::Failed { error },
                    }
                } else {
                    self.partial_pull(source_patient_id, workspace, &failed_families, &reason)
                }
            }
        }
    }

    async fn reconnect(
        &self,
        practice_id: &str,
        now: i64,
        command_key: Option<SourceCommandKey<'_>>,
    ) -> Result<ReconnectReceipt, SourceAdapterError> {
        self.mutate_table(|table| {
            if let Some(cmd_key) = command_key
                && let Some(existing) = table
                    .command_receipts
                    .get(&cmd_key.scoped_key(practice_id))
                    .cloned()
            {
                return Ok(existing);
            }
            if let Some(SyntheticSourceMode::Outage { reason }) =
                table.practice_modes.get(practice_id)
            {
                return Err(SourceAdapterError::Unavailable {
                    reason: reason.clone(),
                });
            }
            if matches!(
                table.practice_modes.get(practice_id),
                Some(SyntheticSourceMode::LapsedAccess { .. })
            ) {
                table.practice_modes.remove(practice_id);
            }
            table.patient_modes.retain(|(practice, _), mode| {
                !(practice == practice_id
                    && matches!(mode, SyntheticSourceMode::LapsedAccess { .. }))
            });
            *table
                .reconnect_effects
                .entry(practice_id.to_owned())
                .or_insert(0) += 1;
            let receipt = ReconnectReceipt {
                source_id: SOURCE_ID.to_owned(),
                practice_id: practice_id.to_owned(),
                reconnected_at: now,
            };
            if let Some(cmd_key) = command_key {
                table
                    .command_receipts
                    .insert(cmd_key.scoped_key(practice_id), receipt.clone());
            }
            Ok(receipt)
        })
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
    async fn an_attachment_coda_cannot_read_stops_the_chart() {
        let scan = NOTE.replace("text/plain; charset=utf-8", "image/tiff");
        let source = SyntheticCharts::open(&bank("scan", &scan));
        let ids = source.patient_ids().await.unwrap();
        let patient = source.patient(&ids[0]).await.unwrap();
        let error = source.records(&patient, "pw-1").await.unwrap_err();
        assert!(error.to_string().contains("image/tiff"), "{error}");
        assert!(
            std::error::Error::source(&error).is_some(),
            "RecordFilingError cause must be preserved on SourceAdapterError"
        );
    }

    #[tokio::test]
    async fn controllable_modes_simulate_lapsed_access_partial_outage_and_reconnect() {
        let source = SyntheticCharts::open(&bank("modes", NOTE));

        source.set_practice_mode(
            "practice-a",
            SyntheticSourceMode::LapsedAccess {
                reason: "oauth refresh token expired".into(),
            },
        );
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
        assert_eq!(source.reconnect_effect_count("practice-a"), 1);

        // Retrying with the same command key recovers the prior receipt without a second effect.
        let replayed = source
            .reconnect("practice-a", 1_788_000_999, Some(cmd_key))
            .await
            .unwrap();
        assert_eq!(replayed, receipt);
        assert_eq!(source.reconnect_effect_count("practice-a"), 1);
        assert!(matches!(
            source.pull_patient("practice-a", "MRN1", "pw-1").await,
            SourcePullOutput::Complete { records, .. } if records.len() == 3
        ));

        source.set_patient_mode(
            "practice-a",
            "MRN1",
            SyntheticSourceMode::Partial {
                failed_families: BTreeSet::from(["DocumentReference".to_owned()]),
                reason: "DocumentReference endpoint timed out".into(),
            },
        );
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
    async fn reconnect_receipts_persist_across_reopened_synthetic_chart_instances() {
        let dir = bank("reopened-receipts", NOTE);
        let cmd_key_a = SourceCommandKey {
            physician_id: "dr-a",
            idempotency_key: "idem-cross-instance",
        };

        let first_receipt = {
            let instance_a = SyntheticCharts::open(&dir);
            instance_a.set_practice_mode(
                "practice-shared",
                SyntheticSourceMode::LapsedAccess {
                    reason: "token expired on site A".into(),
                },
            );
            let receipt = instance_a
                .reconnect("practice-shared", 1_788_010_000, Some(cmd_key_a))
                .await
                .unwrap();
            assert_eq!(instance_a.reconnect_effect_count("practice-shared"), 1);
            receipt
        };

        // Construct a brand-new `SyntheticCharts` instance on the same bank (simulating Site B
        // or a restarted Site A after a pre-checkpoint crash) and retry with the same key.
        let instance_b = SyntheticCharts::open(&dir);
        assert_eq!(instance_b.reconnect_effect_count("practice-shared"), 1);
        assert_eq!(
            instance_b.practice_mode("practice-shared"),
            SyntheticSourceMode::Healthy
        );

        // Even if the practice later enters Outage before retry, the cached receipt is recovered
        // without repeating the underlying reconnect effect.
        instance_b.set_practice_mode(
            "practice-shared",
            SyntheticSourceMode::Outage {
                reason: "upstream maintenance".into(),
            },
        );
        let recovered = instance_b
            .reconnect("practice-shared", 1_788_019_999, Some(cmd_key_a))
            .await
            .unwrap();
        assert_eq!(recovered, first_receipt);
        assert_eq!(recovered.reconnected_at, 1_788_010_000);
        assert_eq!(instance_b.reconnect_effect_count("practice-shared"), 1);

        // Restore healthy mode and verify a second physician reusing the same key string executes
        // an independent reconnect effect scoped to `(practice_id, physician_id, idempotency_key)`.
        instance_b.set_practice_mode("practice-shared", SyntheticSourceMode::Healthy);
        let cmd_key_b = SourceCommandKey {
            physician_id: "dr-b",
            idempotency_key: "idem-cross-instance",
        };
        let second_receipt = instance_b
            .reconnect("practice-shared", 1_788_025_000, Some(cmd_key_b))
            .await
            .unwrap();
        assert_eq!(second_receipt.reconnected_at, 1_788_025_000);
        assert_eq!(instance_b.reconnect_effect_count("practice-shared"), 2);
    }
}
