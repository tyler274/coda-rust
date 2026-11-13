//! The synthetic source: charts from a built patient bank, read the way an
//! adapter reads a connected EHR. This is the one test-only piece of the chart
//! path; everything past [`PatientSource`] is the real product.
//!
//! The adapter reads the bank's `patients/` folder and nothing else. The
//! ledgers beside it hold what the charts were written from, so no part of the
//! product ever sees them.

use std::fs;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use local_product::physician_sources::{
    MedicalRecordNumber, PatientSource, SourceAdapterError, SourcePatient, SourceRevision,
};
use serde_json::Value;

/// Identity of the synthetic chart adapter, including developer-plan admission.
pub const SOURCE_ID: &str = "synthetic-charts";
const SOURCE_LABEL: &str = "Synthetic chart";

/// A built patient bank on disk.
pub struct SyntheticCharts {
    patients: PathBuf,
}

impl SyntheticCharts {
    /// Opens the bank built at `bank`.
    pub fn open(bank: &Path) -> Self {
        Self {
            patients: bank.join("patients"),
        }
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
}
