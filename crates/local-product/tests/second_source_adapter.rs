//! Proof that a second, non-FHIR source loads into the shared record format
//! and serves chart discovery and read tools with only its own declared
//! [`AdapterTable`] and [`PatientSource`] implementation.

use async_trait::async_trait;
use base64::Engine as _;
use local_product::physician_sources::adapter::{
    AdapterRow, AdapterTable, AttachmentRule, CategoryMatch, UnmappedRecordRule, ValueExtractor,
    file_json_record,
};
use local_product::physician_sources::{
    Bucket, ChartRequest, ChartScope, DateRole, FiledDate, MedicalRecordNumber, PatientSource,
    READ_TOOL, RecordVersion, SEARCH_TOOL, SourcePatient, SourcePullOutput, SourceRecordKind,
    SourceRevision, workspace_id,
};
use local_product::runtime::{Store, sources::SourcePull};
use local_product::sha256_hex;
use serde_json::{Value, json};

const MEMO_ATTACHMENTS: &[AttachmentRule] = &[AttachmentRule::Base64Text {
    items_path: "blobs",
    label: "memo_body",
    content_type_path: "mime",
    data_path: "b64",
}];

const WORKFLOW_STATE: &[ValueExtractor] = &[ValueExtractor::Scalar("workflow_state")];

const NORTHWIND_PORTAL_TABLE: AdapterTable = AdapterTable {
    source_id: "northwind-portal",
    kind_field: "entry_kind",
    id_field: "record_id",
    version_field: None,
    category_extractors: &[ValueExtractor::Scalar("section")],
    rows: &[
        AdapterRow {
            kind: "member_profile",
            category: CategoryMatch::Any,
            bucket: Bucket::Patient,
            dates: &[(DateRole::BirthDate, "dob")],
            status: &[],
            codes: &[],
            title: &[ValueExtractor::Scalar("full_name")],
            attachments: &[],
        },
        AdapterRow {
            kind: "assay_panel",
            category: CategoryMatch::OneOf(&["chemistry", "hematology"]),
            bucket: Bucket::Results,
            dates: &[
                (DateRole::Effective, "collected_at"),
                (DateRole::Issued, "released_at"),
            ],
            status: WORKFLOW_STATE,
            codes: &["assay_code"],
            title: &[ValueExtractor::Scalar("panel_title")],
            attachments: &[],
        },
        AdapterRow {
            kind: "clinician_memo",
            category: CategoryMatch::OneOf(&["visit_summary"]),
            bucket: Bucket::Notes,
            dates: &[(DateRole::Authored, "signed_on")],
            status: WORKFLOW_STATE,
            codes: &[],
            title: &[ValueExtractor::Scalar("subject_line")],
            attachments: MEMO_ATTACHMENTS,
        },
        AdapterRow {
            kind: "clinician_memo",
            category: CategoryMatch::OtherOrNone,
            bucket: Bucket::Documents,
            dates: &[(DateRole::Authored, "signed_on")],
            status: WORKFLOW_STATE,
            codes: &[],
            title: &[ValueExtractor::Scalar("subject_line")],
            attachments: MEMO_ATTACHMENTS,
        },
        AdapterRow {
            kind: "intake_sheet",
            category: CategoryMatch::Any,
            bucket: Bucket::Forms,
            dates: &[(DateRole::Recorded, "submitted_at")],
            status: WORKFLOW_STATE,
            codes: &[],
            title: &[ValueExtractor::Scalar("sheet_name")],
            attachments: &[],
        },
    ],
    unmapped: UnmappedRecordRule {
        bucket: Bucket::Other,
        dates: &[],
        status: WORKFLOW_STATE,
        codes: &[],
        title: &[ValueExtractor::Scalar("title")],
        attachments: &[],
    },
};

struct NorthwindPortalSource {
    raw_lines: Vec<String>,
}

impl NorthwindPortalSource {
    fn sample() -> Self {
        let memo_text = base64::engine::general_purpose::STANDARD
            .encode("Blood pressure improved on home cuff log; continue amlodipine.");
        let referral_text = base64::engine::general_purpose::STANDARD
            .encode("External ophthalmology retinal screening summary.");
        Self {
            raw_lines: vec![
                r#"{"entry_kind":"member_profile","record_id":"NW-900","full_name":"Nadia Okafor","dob":"1984-02-19"}"#.to_owned(),
                r#"{"entry_kind":"assay_panel" , "record_id":"LAB-44", "section":"chemistry", "workflow_state":"verified", "assay_code":"CHEM-8", "panel_title":"Basic chemistry panel", "collected_at":"2026-08-18T07:15:00-05:00", "released_at":"2026-08-18T10:02:00-05:00"}"#.to_owned(),
                format!(
                    r#"{{"entry_kind":"clinician_memo","record_id":"MEMO-7","section":"visit_summary","workflow_state":"signed","subject_line":"Hypertension follow-up","signed_on":"2026-08-19","blobs":[{{"mime":"text/plain","b64":"{memo_text}"}}]}}"#
                ),
                format!(
                    r#"{{"entry_kind":"clinician_memo","record_id":"MEMO-8","section":"scanned_fax","workflow_state":"archived","subject_line":"Retinal screening","signed_on":"2026-08-10","blobs":[{{"mime":"text/plain","b64":"{referral_text}"}}]}}"#
                ),
                r#"{"entry_kind":"intake_sheet","record_id":"FORM-2","workflow_state":"submitted","sheet_name":"Pre-visit symptom check","submitted_at":"2026-08-19T08:45:00-05:00"}"#.to_owned(),
                r#"{"entry_kind":"parking_voucher","record_id":"VOUCH-1"}"#.to_owned(),
            ],
        }
    }
}

#[async_trait]
impl PatientSource for NorthwindPortalSource {
    fn source_id(&self) -> &str {
        NORTHWIND_PORTAL_TABLE.source_id
    }

    fn source_label(&self) -> &str {
        "Northwind Portal"
    }

    async fn patient_ids(&self) -> Result<Vec<String>, String> {
        Ok(vec!["NW-900".into()])
    }

    async fn patient(&self, source_patient_id: &str) -> Result<SourcePatient, String> {
        if source_patient_id != "NW-900" {
            return Err(format!("unknown patient {source_patient_id}"));
        }
        Ok(SourcePatient {
            source_patient_id: "NW-900".into(),
            display_name: "Nadia Okafor".into(),
            birth_date: Some("1984-02-19".into()),
            sex_or_gender: None,
            medical_record_numbers: vec![MedicalRecordNumber {
                system: "urn:northwind:member".into(),
                value: "NW-900".into(),
            }],
        })
    }

    async fn records(
        &self,
        _patient: &SourcePatient,
        workspace: &str,
    ) -> Result<Vec<SourceRevision>, String> {
        self.raw_lines
            .iter()
            .map(|line| {
                file_json_record(
                    &NORTHWIND_PORTAL_TABLE,
                    workspace,
                    self.source_id(),
                    line,
                    None,
                )
                .map_err(|err| err.to_string())
            })
            .collect()
    }
}

#[tokio::test]
async fn second_made_up_source_loads_and_serves_tools_with_only_its_table_and_adapter() {
    let source = NorthwindPortalSource::sample();
    let ws = workspace_id(source.source_id(), "NW-900");
    let SourcePullOutput::Complete { patient, records } =
        source.pull_patient("practice-nw", "NW-900", &ws).await
    else {
        panic!("expected complete pull");
    };
    assert_eq!(patient.display_name, "Nadia Okafor");
    assert_eq!(records.len(), 6);

    // Byte-for-byte preservation + content-hash versioning (no version_field) + absent fields None.
    for (raw, filed) in source.raw_lines.iter().zip(&records) {
        assert_eq!(filed.content.as_bytes(), raw.as_bytes());
        assert_eq!(filed.sha256, sha256_hex(raw.as_bytes()));
        let env = filed.envelope.as_ref().expect("filed envelope");
        assert_eq!(
            env.version,
            RecordVersion::ContentHash(filed.sha256.clone())
        );
    }

    // Unmapped `parking_voucher` lands in `Bucket::Other` with absent optional fields left None.
    let voucher = records
        .iter()
        .find(|r| r.candidate_id == "parking_voucher/VOUCH-1")
        .unwrap();
    let voucher_env = voucher.envelope.as_ref().unwrap();
    assert_eq!(voucher_env.bucket, Bucket::Other);
    assert_eq!(voucher_env.status, None);
    assert!(voucher_env.dates.is_empty());
    assert!(voucher_env.codes.is_empty());
    assert_eq!(voucher_env.title, None);

    // Search by `results` bucket returns ONLY `assay_panel/LAB-44` with custom source kind and dates.
    let results_search = ChartRequest::parse(
        SEARCH_TOOL,
        &json!({"query": "", "bucket": "results", "record_type": null, "after": null}),
        ChartScope::Chat,
    )
    .unwrap();
    let results_page: Value = serde_json::from_str(
        results_search
            .run(&ws, 1, &records)
            .unwrap()
            .text()
            .unwrap(),
    )
    .unwrap();
    let hits = results_page["results"].as_array().unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0]["bucket"], "results");
    assert_eq!(
        hits[0]["source_kind"],
        json!({"kind": "assay_panel", "category": "chemistry"})
    );
    assert_eq!(hits[0]["status"], "verified");
    assert_eq!(
        hits[0]["dates"],
        json!([
            {"role": "effective", "field": "collected_at", "value": "2026-08-18T07:15:00-05:00"},
            {"role": "issued", "field": "released_at", "value": "2026-08-18T10:02:00-05:00"}
        ])
    );

    // Search within `notes` bucket for text decoded from base64 attachment (`amlodipine`).
    let notes_search = ChartRequest::parse(
        SEARCH_TOOL,
        &json!({"query": "amlodipine", "bucket": "notes", "record_type": null, "after": null}),
        ChartScope::Chat,
    )
    .unwrap();
    let notes_page: Value =
        serde_json::from_str(notes_search.run(&ws, 1, &records).unwrap().text().unwrap()).unwrap();
    let note_hits = notes_page["results"].as_array().unwrap();
    assert_eq!(note_hits.len(), 1);
    assert_eq!(
        note_hits[0]["reference"]["candidate_id"],
        "clinician_memo/MEMO-7"
    );

    // Read the discovered memo and confirm envelope + decoded attachment + untouched native JSON.
    let read_req = ChartRequest::parse(
        READ_TOOL,
        &json!({"references": [note_hits[0]["reference"].clone()], "max_result_bytes": 8192}),
        ChartScope::Chat,
    )
    .unwrap();
    let read_page: Value =
        serde_json::from_str(read_req.run(&ws, 1, &records).unwrap().text().unwrap()).unwrap();
    let memo_item = &read_page["results"][0];
    assert_eq!(memo_item["bucket"], "notes");
    assert_eq!(
        memo_item["source_kind"],
        serde_json::to_value(SourceRecordKind {
            kind: "clinician_memo".into(),
            category: Some("visit_summary".into()),
        })
        .unwrap()
    );
    assert_eq!(
        memo_item["dates"],
        serde_json::to_value(vec![FiledDate {
            role: DateRole::Authored,
            field: "signed_on".into(),
            value: "2026-08-19".into(),
        }])
        .unwrap()
    );
    assert_eq!(
        memo_item["readable_parts"][0]["text"],
        "Blood pressure improved on home cuff log; continue amlodipine."
    );
    assert_eq!(memo_item["content"], source.raw_lines[2]);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn second_made_up_source_persists_in_store_and_serves_tools() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();

    let source = NorthwindPortalSource::sample();
    let ws = format!("pw-nw-{}", uuid::Uuid::new_v4());
    let patient = source.patient("NW-900").await.unwrap();
    let records = source.records(&patient, &ws).await.unwrap();
    let receipt = store
        .record_chart_with_receipt(
            &ws,
            &records,
            &SourcePull {
                source_id: source.source_id().into(),
                source_label: source.source_label().into(),
                patient,
                pulled_at: 1_760_000_000,
            },
        )
        .await
        .unwrap();
    assert_eq!(receipt.changed, 6);
    assert!(receipt.missing_candidates.is_empty());

    let search = ChartRequest::parse(
        SEARCH_TOOL,
        &json!({"query": "", "bucket": "documents", "record_type": null, "after": null}),
        ChartScope::Chat,
    )
    .unwrap();
    let observation = store.chart_observation(&ws, &search).await.unwrap();
    let page: Value = serde_json::from_str(
        search
            .run(&ws, observation.version, &observation.sources)
            .unwrap()
            .text()
            .unwrap(),
    )
    .unwrap();
    let hits = page["results"].as_array().unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(
        hits[0]["reference"]["candidate_id"],
        "clinician_memo/MEMO-8"
    );
    assert_eq!(hits[0]["bucket"], "documents");
}
