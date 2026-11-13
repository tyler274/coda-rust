//! Focused verification of the declared FHIR R4 adapter table, byte-for-byte
//! source record retention, absent-field fidelity, and bucket-aware chart tools.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use base64::Engine as _;
use local_product::physician_sources::{
    Bucket, ChartPullReceipt, ChartRequest, ChartScope, DateRole, FiledDate, PatientSource,
    READ_TOOL, RecordVersion, SEARCH_TOOL, SourceRecordKind, SourceRevision, exact_read,
};
use local_product::runtime::{Store, sources::SourcePull};
use local_product::sha256_hex;
use physician_sample::synthetic_chart::{SOURCE_ID, SyntheticCharts};
use serde_json::{Value, json};

fn temp_bank_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "coda-fhir-table-{tag}-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create temp synthetic bank");
    dir
}

#[tokio::test]
async fn synthetic_chart_files_every_fhir_row_and_routes_unmapped_kinds_to_other() {
    let bank = temp_bank_dir("all-rows");
    let chart = bank.join("patients/MRN-FHIR-1");
    fs::create_dir_all(&chart).unwrap();

    let note_b64 = base64::engine::general_purpose::STANDARD.encode("Assessment: stable HTN.");
    let doc_b64 = base64::engine::general_purpose::STANDARD.encode("Outside cardiology fax.");

    let files_and_lines: &[(&str, &[&str])] = &[
        (
            "Patient.ndjson",
            &[
                r#"{"resourceType":"Patient","id":"pat-1","meta":{"versionId":"1","lastUpdated":"2026-08-27T00:00:00+00:00"},"identifier":[{"system":"urn:mrn","value":"MRN-FHIR-1"}],"name":[{"use":"official","family":"Rivera","given":["Elena"]}],"gender":"female","birthDate":"1980-04-12"}"#,
            ],
        ),
        (
            "Condition.ndjson",
            &[
                r#"{"resourceType":"Condition","id":"cond-1","meta":{"versionId":"2"},"clinicalStatus":{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/condition-clinical","code":"active"}]},"code":{"coding":[{"system":"http://hl7.org/fhir/sid/icd-10-cm","code":"I10","display":"Essential hypertension"}],"text":"Hypertension"},"onsetDateTime":"2021-05-01","recordedDate":"2021-05-02"}"#,
            ],
        ),
        (
            "Medication.ndjson",
            &[
                r#"{"resourceType":"MedicationRequest","id":"medreq-1","status":"active","medicationCodeableConcept":{"coding":[{"system":"http://www.nlm.nih.gov/research/umls/rxnorm","code":"314076","display":"Lisinopril 10 MG Oral Tablet"}],"text":"Lisinopril 10 mg"},"authoredOn":"2026-01-15"}"#,
                r#"{"resourceType":"MedicationStatement","id":"medstmt-1","status":"completed","medicationCodeableConcept":{"text":"OTC Ibuprofen"},"effectiveDateTime":"2026-02-01","dateAsserted":"2026-02-03"}"#,
                r#"{"resourceType":"MedicationDispense","id":"meddisp-1","status":"completed","medicationCodeableConcept":{"text":"Lisinopril 10 mg"},"whenPrepared":"2026-01-15T09:00:00Z","whenHandedOver":"2026-01-15T14:30:00Z"}"#,
                r#"{"resourceType":"MedicationAdministration","id":"medadm-1","status":"completed","medicationCodeableConcept":{"text":"Ketorolac 15 mg IV"},"effectivePeriod":{"start":"2026-03-01T10:00:00Z","end":"2026-03-01T10:05:00Z"}}"#,
            ],
        ),
        (
            "AllergyIntolerance.ndjson",
            &[
                r#"{"resourceType":"AllergyIntolerance","id":"alg-1","clinicalStatus":{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/allergyintolerance-clinical","code":"active"}]},"code":{"text":"Penicillin"},"recordedDate":"2019-11-20"}"#,
            ],
        ),
        (
            "DiagnosticReport.ndjson",
            &[
                r#"{"resourceType":"DiagnosticReport","id":"dr-1","status":"final","category":[{"coding":[{"code":"LAB"}]}],"code":{"text":"Comprehensive metabolic panel"},"effectiveDateTime":"2026-08-01T08:00:00Z","issued":"2026-08-01T12:00:00Z"}"#,
            ],
        ),
        (
            "Observation.ndjson",
            &[
                r#"{"resourceType":"Observation","id":"obs-lab","status":"final","category":[{"coding":[{"code":"laboratory"}]}],"code":{"coding":[{"system":"http://loinc.org","code":"2823-3","display":"Potassium"}],"text":"Potassium"},"effectiveDateTime":"2026-08-01T08:00:00Z"}"#,
                r#"{"resourceType":"Observation","id":"obs-img","status":"final","category":[{"coding":[{"code":"imaging"}]}],"code":{"text":"Carotid IMT"},"effectiveDateTime":"2026-07-10"}"#,
                r#"{"resourceType":"Observation","id":"obs-vital","status":"final","category":[{"coding":[{"code":"vital-signs"}]}],"code":{"text":"Systolic blood pressure"},"effectiveDateTime":"2026-08-12T11:55:00Z"}"#,
                r#"{"resourceType":"Observation","id":"obs-soc","status":"final","category":[{"coding":[{"code":"social-history"}]}],"code":{"text":"Tobacco smoking status"},"effectiveDateTime":"2026-08-12"}"#,
                r#"{"resourceType":"Observation","id":"obs-survey","status":"final","category":[{"coding":[{"code":"survey"}]}],"code":{"text":"PHQ-9 total score"},"effectiveDateTime":"2026-08-12"}"#,
                r#"{"resourceType":"Observation","id":"obs-exam","status":"final","category":[{"coding":[{"code":"exam"}]}],"code":{"text":"Foot monofilament exam"},"effectiveDateTime":"2026-08-12"}"#,
                r#"{"resourceType":"Observation","id":"obs-nocat","status":"final","code":{"text":"Uncategorized observation"},"effectiveDateTime":"2026-08-12"}"#,
            ],
        ),
        (
            "DocumentReference.ndjson",
            &[
                &format!(
                    r#"{{"resourceType":"DocumentReference","id":"doc-note","status":"current","category":[{{"coding":[{{"code":"clinical-note"}}]}}],"type":{{"text":"Progress note"}},"date":"2026-08-12T12:05:00+00:00","description":"Office visit note","content":[{{"attachment":{{"contentType":"text/plain; charset=utf-8","data":"{note_b64}"}}}}]}}"#
                ),
                &format!(
                    r#"{{"resourceType":"DocumentReference","id":"doc-fax","status":"current","category":[{{"coding":[{{"code":"consult-scan"}}]}}],"type":{{"text":"External consult"}},"date":"2026-08-10T09:00:00+00:00","description":"Outside cardiology fax","content":[{{"attachment":{{"contentType":"text/plain; charset=utf-8","data":"{doc_b64}"}}}}]}}"#
                ),
            ],
        ),
        (
            "Visits.ndjson",
            &[
                r#"{"resourceType":"Encounter","id":"enc-1","status":"finished","type":[{"text":"Established patient visit"}],"period":{"start":"2026-08-12T11:45:00+00:00","end":"2026-08-12T12:15:00+00:00"}}"#,
                r#"{"resourceType":"Appointment","id":"appt-1","status":"booked","appointmentType":{"text":"Follow-up"},"description":"Blood pressure recheck","start":"2026-09-12T14:00:00+00:00","end":"2026-09-12T14:30:00+00:00"}"#,
            ],
        ),
        (
            "Procedure.ndjson",
            &[
                r#"{"resourceType":"Procedure","id":"proc-1","status":"completed","code":{"text":"12-lead ECG"},"performedDateTime":"2026-08-12T12:00:00+00:00"}"#,
            ],
        ),
        (
            "Immunization.ndjson",
            &[
                r#"{"resourceType":"Immunization","id":"imm-1","status":"completed","vaccineCode":{"text":"Influenza, seasonal"},"occurrenceDateTime":"2025-10-18"}"#,
            ],
        ),
        (
            "ServiceRequest.ndjson",
            &[
                r#"{"resourceType":"ServiceRequest","id":"sr-1","status":"active","code":{"text":"Lipid panel order"},"authoredOn":"2026-08-12"}"#,
            ],
        ),
        (
            "HistoryAndForms.ndjson",
            &[
                r#"{"resourceType":"FamilyMemberHistory","id":"fmh-1","status":"completed","relationship":{"text":"Father"},"date":"2024-06-01"}"#,
                r#"{"resourceType":"QuestionnaireResponse","id":"qr-1","status":"completed","questionnaire":"Questionnaire/phq-9","authored":"2026-08-12T11:40:00Z"}"#,
                r#"{"resourceType":"Communication","id":"comm-1","status":"completed","category":[{"coding":[{"code":"notification"}],"text":"Portal message"}],"sent":"2026-08-13T08:15:00Z","received":"2026-08-13T08:15:02Z"}"#,
            ],
        ),
        (
            "Unmapped.ndjson",
            &[
                r#"{"resourceType":"Media","id":"media-1","status":"completed","issued":"2026-08-14T10:00:00Z"}"#,
                r#"{"resourceType":"RiskAssessment","id":"risk-1","status":"final","occurrenceDateTime":"2026-08-14"}"#,
            ],
        ),
    ];

    for (filename, lines) in files_and_lines {
        let mut body = lines.join("\n");
        body.push('\n');
        fs::write(chart.join(filename), body).unwrap();
    }

    let source = SyntheticCharts::open(&bank);
    let patient = source.patient("MRN-FHIR-1").await.unwrap();
    let records = source.records(&patient, "pw-elena").await.unwrap();
    assert_eq!(
        records.len(),
        27,
        "all 27 records load without halting on unmapped kinds"
    );

    let by_id: BTreeMap<&str, _> = records
        .iter()
        .map(|record| {
            let env = record
                .envelope
                .as_ref()
                .unwrap_or_else(|| panic!("missing envelope on {}", record.candidate_id));
            (record.candidate_id.as_str(), env)
        })
        .collect();

    let expected_buckets = [
        ("synthetic-charts/Patient/pat-1", Bucket::Patient),
        ("synthetic-charts/Condition/cond-1", Bucket::Problems),
        (
            "synthetic-charts/MedicationRequest/medreq-1",
            Bucket::Medications,
        ),
        (
            "synthetic-charts/MedicationStatement/medstmt-1",
            Bucket::Medications,
        ),
        (
            "synthetic-charts/MedicationDispense/meddisp-1",
            Bucket::Medications,
        ),
        (
            "synthetic-charts/MedicationAdministration/medadm-1",
            Bucket::Medications,
        ),
        (
            "synthetic-charts/AllergyIntolerance/alg-1",
            Bucket::Allergies,
        ),
        ("synthetic-charts/DiagnosticReport/dr-1", Bucket::Results),
        ("synthetic-charts/Observation/obs-lab", Bucket::Results),
        ("synthetic-charts/Observation/obs-img", Bucket::Results),
        ("synthetic-charts/Observation/obs-vital", Bucket::Vitals),
        ("synthetic-charts/Observation/obs-soc", Bucket::History),
        ("synthetic-charts/Observation/obs-survey", Bucket::Forms),
        ("synthetic-charts/Observation/obs-exam", Bucket::Other),
        ("synthetic-charts/Observation/obs-nocat", Bucket::Other),
        ("synthetic-charts/DocumentReference/doc-note", Bucket::Notes),
        (
            "synthetic-charts/DocumentReference/doc-fax",
            Bucket::Documents,
        ),
        ("synthetic-charts/Encounter/enc-1", Bucket::Visits),
        ("synthetic-charts/Appointment/appt-1", Bucket::Visits),
        ("synthetic-charts/Procedure/proc-1", Bucket::Procedures),
        ("synthetic-charts/Immunization/imm-1", Bucket::Immunizations),
        ("synthetic-charts/ServiceRequest/sr-1", Bucket::Orders),
        (
            "synthetic-charts/FamilyMemberHistory/fmh-1",
            Bucket::History,
        ),
        ("synthetic-charts/QuestionnaireResponse/qr-1", Bucket::Forms),
        ("synthetic-charts/Communication/comm-1", Bucket::Messages),
        ("synthetic-charts/Media/media-1", Bucket::Other),
        ("synthetic-charts/RiskAssessment/risk-1", Bucket::Other),
    ];

    for (candidate_id, expected_bucket) in expected_buckets {
        assert_eq!(
            by_id[candidate_id].bucket, expected_bucket,
            "wrong bucket for {candidate_id}"
        );
    }

    let covered_buckets: BTreeSet<Bucket> = by_id.values().map(|env| env.bucket).collect();
    assert_eq!(
        covered_buckets,
        Bucket::ALL.into_iter().collect(),
        "test exercises every one of the 16 chart buckets"
    );

    assert_eq!(
        by_id["synthetic-charts/Observation/obs-lab"].source_kind,
        SourceRecordKind {
            kind: "Observation".into(),
            category: Some("laboratory".into()),
        }
    );
    assert_eq!(
        by_id["synthetic-charts/DocumentReference/doc-note"].source_kind,
        SourceRecordKind {
            kind: "DocumentReference".into(),
            category: Some("clinical-note".into()),
        }
    );
    assert_eq!(
        by_id["synthetic-charts/Observation/obs-nocat"].source_kind,
        SourceRecordKind {
            kind: "Observation".into(),
            category: None,
        }
    );

    let _ = fs::remove_dir_all(&bank);
}

#[tokio::test]
async fn saved_record_equals_source_bytes_and_absent_parts_stay_absent() {
    let bank = temp_bank_dir("fidelity");
    let chart = bank.join("patients/MRN-FID-1");
    fs::create_dir_all(&chart).unwrap();

    // Deliberate irregular spacing, field ordering, and exact precision to prove
    // zero re-serialization or normalization of source bytes, dates, or status.
    let raw_patient = r#"{"resourceType":"Patient",  "id":"MRN-FID-1","name":[{"family":"Chen","given":["David"]}],"birthDate":"1965-09"}"#;
    let raw_versioned_lab = r#"{"resourceType":"Observation" , "id":"k-1", "meta":{"versionId":"v7"}, "status":"preliminary", "category":[{"coding":[{"code":"laboratory"}]}], "code":{"coding":[{"system":"http://loinc.org","code":"2823-3","display":"Potassium [Moles/volume] in Serum or Plasma"}]}, "effectiveDateTime":"2026-08-19T07:42:11.120-04:00", "valueQuantity":{"value":4.10,"unit":"mmol/L"}}"#;
    let raw_sparse_unmapped = r#"{"resourceType":"Basic","id":"sparse-1"}"#;

    fs::write(chart.join("Patient.ndjson"), format!("{raw_patient}\n")).unwrap();
    fs::write(
        chart.join("Observation.ndjson"),
        format!("{raw_versioned_lab}\n"),
    )
    .unwrap();
    fs::write(
        chart.join("Basic.ndjson"),
        format!("{raw_sparse_unmapped}\n"),
    )
    .unwrap();

    let source = SyntheticCharts::open(&bank);
    let patient = source.patient("MRN-FID-1").await.unwrap();
    let records = source.records(&patient, "pw-david").await.unwrap();
    let by_id: BTreeMap<&str, _> = records
        .iter()
        .map(|r| (r.candidate_id.as_str(), r))
        .collect();

    // 1. Byte-for-byte content + SHA-256 + explicit version + status + date precision
    let lab = by_id["synthetic-charts/Observation/k-1"];
    assert_eq!(lab.content.as_bytes(), raw_versioned_lab.as_bytes());
    assert_eq!(lab.sha256, sha256_hex(raw_versioned_lab.as_bytes()));
    let lab_env = lab.envelope.as_ref().expect("lab envelope");
    assert_eq!(lab_env.native_id, "k-1");
    assert_eq!(lab_env.bucket, Bucket::Results);
    assert_eq!(lab_env.version, RecordVersion::Source("v7".into()));
    assert_eq!(lab_env.status.as_deref(), Some("preliminary"));
    assert_eq!(
        lab_env.dates,
        vec![FiledDate {
            role: DateRole::Effective,
            field: "effectiveDateTime".into(),
            value: "2026-08-19T07:42:11.120-04:00".into(),
        }],
        "only effectiveDateTime was sent; issued and effectivePeriod stay absent"
    );
    assert!(lab.readable_parts.is_empty());
    assert_eq!(lab_env.native_link, None);

    // 2. Partial date kept verbatim; no versionId -> versioned by content hash; absent status stays None
    let pat = by_id["synthetic-charts/Patient/MRN-FID-1"];
    assert_eq!(pat.content.as_bytes(), raw_patient.as_bytes());
    let pat_env = pat.envelope.as_ref().expect("patient envelope");
    assert_eq!(pat_env.native_id, "MRN-FID-1");
    assert_eq!(
        pat_env.version,
        RecordVersion::ContentHash(pat.sha256.clone())
    );
    assert_eq!(pat_env.status, None);
    assert_eq!(
        pat_env.dates,
        vec![FiledDate {
            role: DateRole::BirthDate,
            field: "birthDate".into(),
            value: "1965-09".into(),
        }]
    );

    // 3. Sparse unmapped record: every absent optional field stays None / empty
    let sparse = by_id["synthetic-charts/Basic/sparse-1"];
    assert_eq!(sparse.content.as_bytes(), raw_sparse_unmapped.as_bytes());
    let sparse_env = sparse.envelope.as_ref().expect("sparse envelope");
    assert_eq!(sparse_env.native_id, "sparse-1");
    assert_eq!(sparse_env.bucket, Bucket::Other);
    assert_eq!(sparse_env.status, None);
    assert!(sparse_env.dates.is_empty());
    assert!(sparse_env.codes.is_empty());
    assert_eq!(sparse_env.title, None);
    assert_eq!(sparse_env.native_link, None);
    assert!(sparse.readable_parts.is_empty());

    let _ = fs::remove_dir_all(&bank);
}

#[test]
fn multi_category_fhir_records_and_attachment_edge_cases_file_faithfully() {
    use local_product::physician_sources::adapter::{
        FHIR_R4_TABLE, RecordFilingError, file_json_record,
    };

    // Multi-category DocumentReference with local LOINC code first and `clinical-note` second
    // files into Bucket::Notes with uppercase MIME type `TEXT/PLAIN; charset=UTF-8`.
    let note_b64 = base64::engine::general_purpose::STANDARD.encode("Follow-up note body.");
    let multi_cat_note = format!(
        r#"{{"resourceType":"DocumentReference","id":"doc-multi","status":"current","category":[{{"coding":[{{"system":"http://loinc.org","code":"11506-3"}}]}},{{"coding":[{{"code":"clinical-note"}}]}}],"content":[{{"attachment":{{"contentType":"TEXT/PLAIN; charset=UTF-8","data":"{note_b64}"}}}}]}}"#
    );
    let filed_note =
        file_json_record(&FHIR_R4_TABLE, "pw-multi", "ehr", &multi_cat_note, None).unwrap();
    let note_env = filed_note.envelope.as_ref().unwrap();
    assert_eq!(note_env.bucket, Bucket::Notes);
    assert_eq!(
        note_env.source_kind,
        SourceRecordKind {
            kind: "DocumentReference".into(),
            category: Some("clinical-note".into()),
        }
    );
    assert_eq!(filed_note.readable_parts.len(), 1);
    assert_eq!(filed_note.readable_parts[0].text, "Follow-up note body.");

    // Multi-coded Observation category with vendor code first and `laboratory` second files into Bucket::Results.
    let multi_cat_obs = r#"{"resourceType":"Observation","id":"obs-multi","status":"final","category":[{"coding":[{"code":"local-chem"},{"code":"laboratory"}]}],"code":{"text":"Sodium"}}"#;
    let filed_obs =
        file_json_record(&FHIR_R4_TABLE, "pw-multi", "ehr", multi_cat_obs, None).unwrap();
    let obs_env = filed_obs.envelope.as_ref().unwrap();
    assert_eq!(obs_env.bucket, Bucket::Results);
    assert_eq!(
        obs_env.source_kind,
        SourceRecordKind {
            kind: "Observation".into(),
            category: Some("laboratory".into()),
        }
    );

    // Attachment with contentType `text/plain` and absent `data` (and no external URL) does NOT synthesize an empty ReadablePart.
    let metadata_only_doc = r#"{"resourceType":"DocumentReference","id":"doc-nodata","status":"current","category":[{"coding":[{"code":"clinical-note"}]}],"content":[{"attachment":{"contentType":"text/plain"}}]}"#;
    let filed_nodata =
        file_json_record(&FHIR_R4_TABLE, "pw-multi", "ehr", metadata_only_doc, None).unwrap();
    assert!(
        filed_nodata.readable_parts.is_empty(),
        "absent attachment.data must not synthesize an empty ReadablePart"
    );

    // Attachment pointing only to an external `url` without inline `data` fails explicitly.
    let external_url_doc = r#"{"resourceType":"DocumentReference","id":"doc-url","status":"current","content":[{"attachment":{"contentType":"text/plain","url":"https://ehr.example/Binary/99"}}]}"#;
    let err =
        file_json_record(&FHIR_R4_TABLE, "pw-multi", "ehr", external_url_doc, None).unwrap_err();
    assert!(
        matches!(err, RecordFilingError::ExternalAttachment { .. }),
        "expected ExternalAttachment error, got {err:?}"
    );
}

#[tokio::test]
async fn source_tools_return_bucket_kind_and_dated_roles_and_filter_by_bucket() {
    let bank = temp_bank_dir("tools");
    let chart = bank.join("patients/MRN-TOOLS-1");
    fs::create_dir_all(&chart).unwrap();

    let note_b64 =
        base64::engine::general_purpose::STANDARD.encode("Follow-up on potassium and lisinopril.");
    fs::write(
        chart.join("Patient.ndjson"),
        r#"{"resourceType":"Patient","id":"MRN-TOOLS-1","meta":{"versionId":"1"},"name":[{"family":"Kim","given":["Hannah"]}],"birthDate":"1978-11-03"}"#.to_owned() + "\n",
    )
    .unwrap();
    fs::write(
        chart.join("MedicationRequest.ndjson"),
        r#"{"resourceType":"MedicationRequest","id":"med-10","meta":{"versionId":"1"},"status":"active","medicationCodeableConcept":{"coding":[{"system":"http://www.nlm.nih.gov/research/umls/rxnorm","code":"314076","display":"Lisinopril 10 MG Oral Tablet"}],"text":"Lisinopril 10 mg daily"},"authoredOn":"2026-07-01"}"#.to_owned() + "\n",
    )
    .unwrap();
    fs::write(
        chart.join("Observation.ndjson"),
        r#"{"resourceType":"Observation","id":"obs-20","meta":{"versionId":"1"},"status":"final","category":[{"coding":[{"code":"laboratory"}]}],"code":{"coding":[{"system":"http://loinc.org","code":"2823-3","display":"Potassium"}],"text":"Potassium"},"effectiveDateTime":"2026-08-10T08:30:00Z","issued":"2026-08-10T11:00:00Z"}"#.to_owned() + "\n",
    )
    .unwrap();
    fs::write(
        chart.join("DocumentReference.ndjson"),
        format!(
            r#"{{"resourceType":"DocumentReference","id":"note-30","meta":{{"versionId":"1"}},"status":"current","category":[{{"coding":[{{"code":"clinical-note"}}]}}],"type":{{"text":"Progress note"}},"date":"2026-08-11T15:00:00Z","description":"Primary care follow-up","content":[{{"attachment":{{"contentType":"text/plain; charset=utf-8","data":"{note_b64}"}}}}]}}"#
        ) + "\n",
    )
    .unwrap();

    let source = SyntheticCharts::open(&bank);
    let patient = source.patient("MRN-TOOLS-1").await.unwrap();
    let catalog = source.records(&patient, "pw-hannah").await.unwrap();

    // Invalid bucket name is rejected at parse time without silent fallback.
    assert!(
        ChartRequest::parse(
            SEARCH_TOOL,
            &json!({"query": "", "bucket": "not_a_bucket", "record_type": null, "after": null}),
            ChartScope::Chat,
        )
        .is_err()
    );

    // Bucket filter returns ONLY records in `results`.
    let results_search = ChartRequest::parse(
        SEARCH_TOOL,
        &json!({"query": "", "bucket": "results", "record_type": null, "after": null}),
        ChartScope::Chat,
    )
    .expect("valid bucket filter");
    let search_outcome = results_search.run("pw-hannah", 4, &catalog).unwrap();
    let search_json: Value = serde_json::from_str(search_outcome.text().unwrap()).unwrap();
    let hits = search_json["results"].as_array().unwrap();
    assert_eq!(
        hits.len(),
        1,
        "bucket filter must return only Results records"
    );
    assert_eq!(hits[0]["bucket"], "results");
    assert_eq!(
        hits[0]["source_kind"],
        json!({"kind": "Observation", "category": "laboratory"})
    );
    assert_eq!(
        hits[0]["dates"],
        json!([
            {"role": "effective", "field": "effectiveDateTime", "value": "2026-08-10T08:30:00Z"},
            {"role": "issued", "field": "issued", "value": "2026-08-10T11:00:00Z"}
        ])
    );

    // Read tool returns envelope fields (`bucket`, `source_kind`, `dates` with roles) + exact content.
    let reference = hits[0]["reference"].clone();
    let read_req = ChartRequest::parse(
        READ_TOOL,
        &json!({"references": [reference], "max_result_bytes": 16384}),
        ChartScope::Chat,
    )
    .expect("valid read request");
    let read_outcome = read_req.run("pw-hannah", 4, &catalog).unwrap();
    let read_json: Value = serde_json::from_str(read_outcome.text().unwrap()).unwrap();
    let item = &read_json["results"][0];
    assert_eq!(item["outcome"], "available");
    assert_eq!(item["bucket"], "results");
    assert_eq!(
        item["source_kind"],
        json!({"kind": "Observation", "category": "laboratory"})
    );
    assert_eq!(item["status"], "final");
    assert_eq!(
        item["dates"],
        json!([
            {"role": "effective", "field": "effectiveDateTime", "value": "2026-08-10T08:30:00Z"},
            {"role": "issued", "field": "issued", "value": "2026-08-10T11:00:00Z"}
        ])
    );

    let _ = fs::remove_dir_all(&bank);
}

#[tokio::test]
async fn real_patient_bank_build_files_through_fhir_table_and_serves_tools() {
    let out = temp_bank_dir("built-bank");
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let status = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "-p",
            "patient-bank",
            "--",
            "build",
            "--out",
        ])
        .arg(&out)
        .current_dir(&workspace_root)
        .status()
        .expect("invoke patient-bank build");
    assert!(status.success(), "patient-bank build must succeed");

    let source = SyntheticCharts::open(&out);
    let ids = source.patient_ids().await.unwrap();
    assert_eq!(ids.len(), 468, "built patient-bank holds all 468 patients");

    let mut total_records = 0usize;
    let mut first_patient_id = String::new();
    let mut first_records = Vec::new();

    for (idx, id) in ids.iter().enumerate() {
        let ws = format!("pw-synth-{idx}");
        let patient = source.patient(id).await.unwrap();
        let records = source.records(&patient, &ws).await.unwrap();
        assert!(!records.is_empty(), "chart {id} must hold records");

        // Read raw NDJSON lines in sorted filename order (matching `SyntheticCharts::records`)
        // to prove byte-for-byte preservation across every record of all 468 patients.
        let chart_dir = out.join("patients").join(id);
        let mut files: Vec<PathBuf> = fs::read_dir(&chart_dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect();
        files.sort();
        let mut raw_lines = Vec::new();
        for file in files {
            let text = fs::read_to_string(&file).unwrap();
            raw_lines.extend(text.lines().map(str::to_owned));
        }
        assert_eq!(
            records.len(),
            raw_lines.len(),
            "every raw NDJSON line in {id} files without error"
        );

        for (raw_line, record) in raw_lines.iter().zip(&records) {
            assert_eq!(record.content.as_bytes(), raw_line.as_bytes());
            assert_eq!(record.sha256, sha256_hex(raw_line.as_bytes()));
            let env = record
                .envelope
                .as_ref()
                .expect("every synthetic record carries an envelope");
            assert!(!env.source_kind.kind.is_empty());
            assert!(
                record.candidate_id.starts_with("synthetic-charts/"),
                "candidate_id must be source-scoped: {}",
                record.candidate_id
            );
        }

        total_records += records.len();
        if idx == 0 {
            first_patient_id = patient.source_patient_id;
            first_records = records;
        }
    }
    assert!(total_records > 0);

    // Exercise bucket-filtered discovery on the real synthetic patient chart.
    let meds_search = ChartRequest::parse(
        SEARCH_TOOL,
        &json!({"query": "", "bucket": "medications", "record_type": null, "after": null}),
        ChartScope::Chat,
    )
    .unwrap();
    let meds_outcome = meds_search.run("pw-synth-0", 1, &first_records).unwrap();
    let meds_json: Value = serde_json::from_str(meds_outcome.text().unwrap()).unwrap();
    let med_hits = meds_json["results"].as_array().unwrap();
    assert!(!med_hits.is_empty(), "synthetic patient has medications");
    for hit in med_hits {
        assert_eq!(hit["bucket"], "medications");
        assert!(hit["source_kind"].is_object());
        assert!(hit["dates"].is_array());
    }

    let read_req = ChartRequest::parse(
        READ_TOOL,
        &json!({"references": [med_hits[0]["reference"].clone()], "max_result_bytes": 16384}),
        ChartScope::Chat,
    )
    .unwrap();
    let read_outcome = read_req.run("pw-synth-0", 1, &first_records).unwrap();
    let read_json: Value = serde_json::from_str(read_outcome.text().unwrap()).unwrap();
    assert_eq!(read_json["results"][0]["bucket"], "medications");

    let verification_dir = workspace_root.join("target/verification");
    let _ = fs::create_dir_all(&verification_dir);
    let _ = fs::write(
        verification_dir.join("eng-282-ac3-tool-results.json"),
        serde_json::to_string_pretty(&json!({
            "patients_verified": ids.len(),
            "records_verified": total_records,
            "patient_id": first_patient_id,
            "search_medications": meds_json,
            "read_first_medication": read_json,
        }))
        .unwrap(),
    );

    let _ = fs::remove_dir_all(&out);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn refreshing_pre_table_synthetic_workspace_reconciles_unprefixed_ids_without_duplicates() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();

    let bank = temp_bank_dir("upgrade-reconcile");
    let chart = bank.join("patients/MRN-UPGRADE-1");
    fs::create_dir_all(&chart).unwrap();

    let patient_line = r#"{"resourceType":"Patient","id":"MRN-UPGRADE-1","meta":{"versionId":"1"},"name":[{"family":"Vance","given":["Clara"]}],"birthDate":"1975-06-14"}"#;
    let obs_v1_line = r#"{"resourceType":"Observation","id":"obs-k1","meta":{"versionId":"1"},"status":"preliminary","category":[{"coding":[{"code":"laboratory"}]}],"code":{"text":"Potassium 6.4 hemolyzed"},"effectiveDateTime":"2026-08-12T08:00:00Z"}"#;
    let note_b64 = base64::engine::general_purpose::STANDARD.encode("Initial visit note.");
    let note_line = format!(
        r#"{{"resourceType":"DocumentReference","id":"note-1","meta":{{"versionId":"1"}},"status":"current","category":[{{"coding":[{{"code":"clinical-note"}}]}}],"type":{{"text":"Progress note"}},"date":"2026-08-12T09:00:00Z","content":[{{"attachment":{{"contentType":"text/plain; charset=utf-8","data":"{note_b64}"}}}}]}}"#
    );

    fs::write(chart.join("Patient.ndjson"), format!("{patient_line}\n")).unwrap();
    fs::write(chart.join("Observation.ndjson"), format!("{obs_v1_line}\n")).unwrap();
    fs::write(
        chart.join("DocumentReference.ndjson"),
        format!("{note_line}\n"),
    )
    .unwrap();

    let ws = format!("pw-synth-upgrade-{}", uuid::Uuid::new_v4());
    let source = SyntheticCharts::open(&bank);
    let patient = source.patient("MRN-UPGRADE-1").await.unwrap();

    // Seed pre-table unprefixed `{resourceType}/{id}` revisions matching base `0ee84a1bbd`.
    let legacy_revisions: Vec<SourceRevision> = [
        ("DocumentReference", "note-1", note_line.as_str()),
        ("Observation", "obs-k1", obs_v1_line),
        ("Patient", "MRN-UPGRADE-1", patient_line),
    ]
    .into_iter()
    .map(|(record_type, id, line)| {
        let candidate_id = format!("{record_type}/{id}");
        SourceRevision {
            workspace: ws.clone(),
            source_id: SOURCE_ID.into(),
            candidate_id: candidate_id.clone(),
            revision: sha256_hex(line.as_bytes()),
            record_type: record_type.into(),
            dates: Vec::new(),
            content: line.into(),
            sha256: sha256_hex(line.as_bytes()),
            readable_parts: Vec::new(),
            provenance: json!({"native_record": candidate_id}),
            envelope: None,
        }
    })
    .collect();

    let seeded = store
        .record_chart_with_receipt(
            &ws,
            &legacy_revisions,
            &SourcePull {
                source_id: SOURCE_ID.into(),
                source_label: source.source_label().into(),
                patient: patient.clone(),
                pulled_at: Some(1_760_000_000),
            },
        )
        .await
        .unwrap();
    assert_eq!(seeded.changed, 3);
    assert!(seeded.missing_candidates.is_empty());

    // Refresh through upgraded `SyntheticCharts` (`synthetic_chart_fhir::source_revision`).
    let upgraded_records = source.records(&patient, &ws).await.unwrap();
    let upgraded = store
        .record_chart_with_receipt(
            &ws,
            &upgraded_records,
            &SourcePull {
                source_id: SOURCE_ID.into(),
                source_label: source.source_label().into(),
                patient: patient.clone(),
                pulled_at: Some(1_760_000_100),
            },
        )
        .await
        .unwrap();
    assert_eq!(
        upgraded,
        ChartPullReceipt {
            changed: 3,
            missing_candidates: Vec::new(),
        }
    );

    // Update Observation on disk with a corrected potassium value and refresh again.
    let obs_v2_line = r#"{"resourceType":"Observation","id":"obs-k1","meta":{"versionId":"2"},"status":"corrected","category":[{"coding":[{"code":"laboratory"}]}],"code":{"text":"Potassium 4.1 corrected"},"effectiveDateTime":"2026-08-12T08:00:00Z"}"#;
    fs::write(chart.join("Observation.ndjson"), format!("{obs_v2_line}\n")).unwrap();
    let corrected_records = source.records(&patient, &ws).await.unwrap();
    let corrected = store
        .record_chart_with_receipt(
            &ws,
            &corrected_records,
            &SourcePull {
                source_id: SOURCE_ID.into(),
                source_label: source.source_label().into(),
                patient,
                pulled_at: Some(1_760_000_200),
            },
        )
        .await
        .unwrap();
    assert_eq!(
        corrected,
        ChartPullReceipt {
            changed: 1,
            missing_candidates: Vec::new(),
        }
    );
    assert!(
        store
            .pull_missing_candidates(&ws, SOURCE_ID)
            .await
            .unwrap()
            .is_empty()
    );

    let current = store.sources(&ws).await.unwrap();
    assert_eq!(current.version, 3);
    assert_eq!(
        current.sources.len(),
        3,
        "reconciled unprefixed records must not remain active alongside prefixed records"
    );

    let all_search = ChartRequest::parse(
        SEARCH_TOOL,
        &json!({"query": "", "bucket": null, "record_type": null, "after": null}),
        ChartScope::Chat,
    )
    .unwrap();
    let all_hits: Value = serde_json::from_str(
        all_search
            .run(&ws, current.version, &current.sources)
            .unwrap()
            .text()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(all_hits["results"].as_array().unwrap().len(), 3);

    let stale_search = ChartRequest::parse(
        SEARCH_TOOL,
        &json!({"query": "hemolyzed", "bucket": null, "record_type": null, "after": null}),
        ChartScope::Chat,
    )
    .unwrap();
    let stale_hits: Value = serde_json::from_str(
        stale_search
            .run(&ws, current.version, &current.sources)
            .unwrap()
            .text()
            .unwrap(),
    )
    .unwrap();
    assert!(stale_hits["results"].as_array().unwrap().is_empty());

    let results_search = ChartRequest::parse(
        SEARCH_TOOL,
        &json!({"query": "potassium", "bucket": "results", "record_type": null, "after": null}),
        ChartScope::Chat,
    )
    .unwrap();
    let results_hits: Value = serde_json::from_str(
        results_search
            .run(&ws, current.version, &current.sources)
            .unwrap()
            .text()
            .unwrap(),
    )
    .unwrap();
    let hits = results_hits["results"].as_array().unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(
        hits[0]["reference"]["candidate_id"],
        "synthetic-charts/Observation/obs-k1"
    );

    let read_req = ChartRequest::parse(
        READ_TOOL,
        &json!({"references": [hits[0]["reference"].clone()], "max_result_bytes": 16384}),
        ChartScope::Chat,
    )
    .unwrap();
    let read_page: Value = serde_json::from_str(
        read_req
            .run(&ws, current.version, &current.sources)
            .unwrap()
            .text()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(read_page["results"][0]["content"], obs_v2_line);

    // Pinned v1 observation still serves exact reads of the historical unprefixed ID.
    let v1 = store.sources_at(&ws, 1).await.unwrap();
    let v1_read = exact_read(&v1.sources, &ws, &["Observation/obs-k1".into()], 16384).unwrap();
    let v1_json: Value = serde_json::from_str(v1_read.text().unwrap()).unwrap();
    assert_eq!(v1_json["results"][0]["outcome"], "available");
    assert_eq!(v1_json["results"][0]["content"], obs_v1_line);

    let _ = fs::remove_dir_all(&bank);
}
