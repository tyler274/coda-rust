//! Records in the shape the Hint Chrome connector acquires them, filed by the Hint filer: one
//! patient's captured sections, as `intent/record-mapping.md` ("Hint Chrome responses") and the
//! capture fixture `crates/physician-sample/tests/support/hint-capture.json` give them. A capture
//! is bounded, so its filing is a partial delivery, stored as one.

use local_product::physician_sources::hint::ChartCapture;
use local_product::physician_sources::{
    FailedPart, SourcePatient, SourcePullOutput, SourceRecordKind, SourceRevision,
};
use local_product::runtime::Store;
use local_product::source_status::PartialDelivery;

/// The source the Hint records come from.
pub const SOURCE: &str = "hint";

/// The Hint patient the capture is of.
pub const PATIENT: &str = "hint-patient";

/// The patient as Hint's patient section sends them.
const PATIENT_BODY: &str =
    r#"{"dob":"1984-02-19","first_name":"Nadia","id":"hint-patient","last_name":"Okafor"}"#;

/// A social history entry as Hint's social-history section sends it: its question, all its
/// answers as a list, and when it started. It has no text of its own.
pub const SOCIAL_HISTORY: &str = r#"{"answers":[{"answer":"Former smoker","ident":"answer-1","patientId":"hint-patient"},{"answer":"Quit in 2015","ident":"answer-2","patientId":"hint-patient"}],"ident":"social-1","patientId":"hint-patient","question":"Tobacco use","startDate":"2020"}"#;

/// A visit note as Hint's notes section sends it: `noteType` NOTE, its title and text under
/// `data`, its two versions, and when it happened and was recorded.
pub const NOTE: &str = r#"{"data":{"formattedText":"<p>Seen for her annual wellness visit. Blood pressure 122/78. Continue lisinopril.</p>","text":"Seen for her annual wellness visit. Blood pressure 122/78. Continue lisinopril.","title":"Annual wellness visit"},"eventTimestamp":"2026-10-12","noteId":"note-1","noteType":"NOTE","noteVersion":2,"patientId":"hint-patient","recordCreationTimestamp":"2026-10-12T17:00:00Z","recordVersion":9,"status":"SIGNED"}"#;

/// What the Hint filer files of the capture.
pub struct Filed {
    pub patient: Option<SourcePatient>,
    pub records: Vec<SourceRevision>,
    pub succeeded_kinds: Vec<SourceRecordKind>,
    pub failed_parts: Vec<FailedPart>,
}

impl Filed {
    /// The filed record whose content is `body`.
    pub fn record(&self, body: &str) -> &SourceRevision {
        self.records
            .iter()
            .find(|record| record.content == body)
            .expect("the capture holds the record")
    }
}

/// One section of a capture, done, holding the item `bodies`.
fn section(name: &str, bodies: &[&str]) -> serde_json::Value {
    let items: Vec<serde_json::Value> = bodies
        .iter()
        .map(|body| {
            serde_json::json!({
                "body": serde_json::from_str::<serde_json::Value>(body).unwrap(),
                "unavailable_parts": []
            })
        })
        .collect();
    serde_json::json!({"section": name, "state": "done",
        "result": {"body": {"items": items, "failed_parts": []}}})
}

/// The capture of [`PATIENT`]'s patient, social-history and notes sections, filed by the Hint
/// filer into `workspace`.
pub fn filed(workspace: &str) -> Filed {
    let capture = serde_json::json!({"request": {"patient_id": PATIENT}, "sections": [
        section("patient", &[PATIENT_BODY]),
        section("social-history", &[SOCIAL_HISTORY]),
        section("notes", &[NOTE]),
    ]});
    let SourcePullOutput::Partial {
        patient,
        records,
        succeeded_kinds,
        failed_parts,
    } = serde_json::from_value::<ChartCapture>(capture)
        .unwrap()
        .file(workspace, SOURCE, PATIENT)
    else {
        panic!("a bounded capture is a partial delivery");
    };
    Filed {
        patient,
        records,
        succeeded_kinds,
        failed_parts,
    }
}

/// Stores `filed` as the Hint source's partial delivery to `workspace` of `practice`.
pub async fn deliver(store: &Store, practice: &str, workspace: &str, filed: &Filed) {
    store
        .record_partial_pull(PartialDelivery {
            practice_id: practice,
            workspace_id: workspace,
            source_id: SOURCE,
            source_label: "Hint",
            patient: filed.patient.as_ref(),
            records: &filed.records,
            failed_parts: &filed.failed_parts,
            succeeded_kinds: &filed.succeeded_kinds,
            attempted_at: 1_760_000_000,
            ticket: None,
        })
        .await
        .expect("the Hint delivery");
}
