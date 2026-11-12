//! The synthetic source: charts from a built patient bank, read the way an
//! adapter reads a connected EHR. This is the one test-only piece of the chart
//! path; everything past [`PatientSource`] is the real product.
//!
//! The adapter reads the bank's `patients/` folder and nothing else. The
//! ledgers beside it hold what the charts were written from, so no part of the
//! product ever sees them.

use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use local_product::physician_sources::{
    FailedPart, MedicalRecordNumber, PatientSource, ReconnectReceipt, SourceAdapterError,
    SourcePatient, SourcePullOutput, SourceRecordKind, SourceRevision,
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

#[derive(Default)]
struct ModeTable {
    practice_modes: HashMap<String, SyntheticSourceMode>,
    patient_modes: HashMap<(String, String), SyntheticSourceMode>,
}

/// A built patient bank on disk.
#[derive(Clone)]
pub struct SyntheticCharts {
    patients: PathBuf,
    modes: Arc<RwLock<ModeTable>>,
}

impl SyntheticCharts {
    /// Opens the bank built at `bank`.
    pub fn open(bank: &Path) -> Self {
        Self {
            patients: bank.join("patients"),
            modes: Arc::new(RwLock::new(ModeTable::default())),
        }
    }

    /// Sets the practice-wide connector mode for `practice_id`, clearing any per-patient overrides
    /// in that practice so the practice mode applies uniformly.
    pub fn set_practice_mode(&self, practice_id: &str, mode: SyntheticSourceMode) {
        let mut table = self.modes.write().expect("synthetic source mode lock");
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
    }

    /// Sets a patient-specific connector mode override for `(practice_id, source_patient_id)`.
    pub fn set_patient_mode(
        &self,
        practice_id: &str,
        source_patient_id: &str,
        mode: SyntheticSourceMode,
    ) {
        let mut table = self.modes.write().expect("synthetic source mode lock");
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
    }

    /// Reads the practice-wide connector mode configured for `practice_id`.
    pub fn practice_mode(&self, practice_id: &str) -> SyntheticSourceMode {
        let table = self.modes.read().expect("synthetic source mode lock");
        table
            .practice_modes
            .get(practice_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Resolves the effective connector mode for `(practice_id, source_patient_id)`.
    pub fn effective_mode(
        &self,
        practice_id: &str,
        source_patient_id: &str,
    ) -> SyntheticSourceMode {
        let table = self.modes.read().expect("synthetic source mode lock");
        let key = (practice_id.to_owned(), source_patient_id.to_owned());
        table
            .patient_modes
            .get(&key)
            .or_else(|| table.practice_modes.get(practice_id))
            .cloned()
            .unwrap_or_default()
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
        match self.effective_mode(practice_id, source_patient_id) {
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
    ) -> Result<ReconnectReceipt, SourceAdapterError> {
        let mut table = self.modes.write().expect("synthetic source mode lock");
        if let Some(SyntheticSourceMode::Outage { reason }) = table.practice_modes.get(practice_id)
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
            !(practice == practice_id && matches!(mode, SyntheticSourceMode::LapsedAccess { .. }))
        });
        Ok(ReconnectReceipt {
            source_id: SOURCE_ID.to_owned(),
            practice_id: practice_id.to_owned(),
            reconnected_at: now,
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
        let bank = std::env::temp_dir().join(format!(
            "coda-synthetic-chart-{name}-{}",
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

        let receipt = source.reconnect("practice-a", 1_788_000_000).await.unwrap();
        assert_eq!(receipt.source_id, SOURCE_ID);
        assert_eq!(receipt.practice_id, "practice-a");
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
}
