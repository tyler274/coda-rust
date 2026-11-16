//! The made-up portal the tests file records from: a source that is not FHIR, with its own names
//! for kinds of records, their fields and their dates, declared as its adapter table. The
//! local-product tests load it as a source; the physician site's tests file its records through
//! the same table.

use local_product::physician_sources::Bucket;
use local_product::physician_sources::DateRole;
use local_product::physician_sources::adapter::{
    AdapterRow, AdapterTable, AttachmentRule, CategoryMatch, UnmappedRecordRule, ValueExtractor,
};

/// An intake sheet whose title is long and holds commas, as real titles do, with the portal's
/// own timestamp.
pub const INTAKE_SHEET: &str = r#"{"entry_kind":"intake_sheet","record_id":"FORM-9","workflow_state":"submitted","sheet_name":"Pre-visit symptom check, cardiology clinic, after emergency department visit, with home medication list","submitted_at":"2026-03-22T08:45:00-05:00"}"#;

const MEMO_ATTACHMENTS: &[AttachmentRule] = &[AttachmentRule::Base64Text {
    items_path: "blobs",
    label: "memo_body",
    content_type_path: "mime",
    data_path: "b64",
}];

const WORKFLOW_STATE: &[ValueExtractor] = &[ValueExtractor::Scalar("workflow_state")];

pub const NORTHWIND_PORTAL_TABLE: AdapterTable = AdapterTable {
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
