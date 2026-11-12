//! Focused verification for reading a patient record by source, version, and location.

use std::collections::BTreeMap;
use std::error::Error as _;
use std::io;

#[path = "common/disposable_postgres.rs"]
mod disposable_postgres;
use disposable_postgres::DisposablePostgres;

use local_product::physician_sources::read::{
    AttachmentPart, AttachmentReader, ByteSpan, CompositePartReader, ContentSlice,
    ContinuationCursor, ContinuationScope, DeliveredPart, HistoricalStanding, InlinePartReader,
    InvalidSourceRevisionError, LocationLimit, LocationResolution, NonReadableOutcome,
    ObservationEntry, PageBudget, PageNumber, PanelStanding, PartIdentity, PartSelector, PartState,
    PatientAccess, PatientRecordRead, ProvenancePartReader, ReadCompletion, RecordLocation,
    RecordMissingReason, RecordReadError, RecordReadRequest, RecordReadResult,
    RecordUnavailableReason, ScanDocument, ScanDocumentState, ScanPageEntry, ScanReader,
    ScanSelector, SourcePartError, SourceSelector, VersionResolution, VersionSelector,
    VersionedRecordCatalog, read_patient_record,
};
use local_product::physician_sources::{
    ChartRequest, ChartScope, READ_TOOL, ReadablePart, RecordDate, SourcePatient, SourceRevision,
    read_tool,
};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::{Authority, PublicationTransaction, Scope, async_trait};
use physician_auth::{
    AuthStore, Membership, MemoryAuthStore, PhysicianIdentity, PostgresAuthStore, PracticeClock,
};
use serde_json::json;

fn page(n: u32) -> PageNumber {
    PageNumber::new(n).expect("non-zero 1-indexed page")
}

struct RevisionFixture<'a> {
    workspace: &'a str,
    source_id: &'a str,
    candidate_id: &'a str,
    revision: &'a str,
    record_type: &'a str,
    date: &'a str,
    content: &'a str,
    readable_parts: Vec<ReadablePart>,
}

fn make_revision(spec: RevisionFixture<'_>) -> SourceRevision {
    SourceRevision {
        workspace: spec.workspace.into(),
        source_id: spec.source_id.into(),
        candidate_id: spec.candidate_id.into(),
        revision: spec.revision.into(),
        record_type: spec.record_type.into(),
        dates: vec![RecordDate {
            name: "recorded".into(),
            value: spec.date.into(),
        }],
        content: spec.content.into(),
        sha256: local_product::sha256_hex(spec.content.as_bytes()),
        readable_parts: spec.readable_parts,
        provenance: json!({
            "source_id": spec.source_id,
            "native_id": spec.candidate_id,
            "revision": spec.revision,
        }),
    }
}

#[derive(Default)]
struct MapAttachments {
    by_revision: BTreeMap<(String, String), Vec<AttachmentPart>>,
}

impl AttachmentReader for MapAttachments {
    fn read_attachments(
        &self,
        revision: &SourceRevision,
    ) -> Result<Vec<AttachmentPart>, SourcePartError> {
        Ok(self
            .by_revision
            .get(&(revision.candidate_id.clone(), revision.revision.clone()))
            .cloned()
            .unwrap_or_default())
    }
}

#[derive(Default)]
struct MapScans {
    by_revision: BTreeMap<(String, String), Vec<ScanDocument>>,
}

impl ScanReader for MapScans {
    fn read_scans(&self, revision: &SourceRevision) -> Result<Vec<ScanDocument>, SourcePartError> {
        Ok(self
            .by_revision
            .get(&(revision.candidate_id.clone(), revision.revision.clone()))
            .cloned()
            .unwrap_or_default())
    }
}

struct FailingScanReader;

impl ScanReader for FailingScanReader {
    fn read_scans(&self, _revision: &SourceRevision) -> Result<Vec<ScanDocument>, SourcePartError> {
        Err(SourcePartError::new(io::Error::new(
            io::ErrorKind::InvalidData,
            "scan storage stream truncated mid-frame",
        )))
    }
}

fn map_standing(membership: Membership) -> PanelStanding {
    match membership {
        Membership::Allowed => PanelStanding::Allowed,
        Membership::ClinicDemographic => PanelStanding::ClinicDemographic,
        Membership::Denied => PanelStanding::Denied,
    }
}

#[test]
fn ac1_large_multibyte_document_reads_in_parts_matching_saved_source_without_silent_cuts() {
    let native = "Discharge summary — patient Mary Louis (α/β blocker review) 🔥\n".repeat(18)
        + "Final line: follow potassium in 7 days.";
    let retained_text =
        "Decoded HL7 addendum: K+ 4.3 mmol/L; eGFR > 60 mL/min/1.73m² — verified.\n".repeat(12);
    let attachment_text =
        " Cardiology attachment: Holter monitor 24h — 0 pauses, sinus rhythm throughout.\n"
            .repeat(10);
    let scan_page_1 =
        "Fax Page 1/2 — Outside Clinic Lab Panel: Na 139, K 4.2, Cr 1.05 mg/dL.\n".repeat(9);
    let scan_page_2 = "Fax Page 2/2 — Signed by Dr. Rivera — continue current regimen.\n".repeat(8);

    let source = make_revision(RevisionFixture {
        workspace: "pw-ac1",
        source_id: "ehr-main",
        candidate_id: "doc-discharge-1",
        revision: "rev-10",
        record_type: "DocumentReference",
        date: "2026-09-14",
        content: &native,
        readable_parts: vec![ReadablePart {
            label: "hl7_addendum".into(),
            text: retained_text.clone(),
        }],
    });

    let mut attachments = MapAttachments::default();
    attachments.by_revision.insert(
        ("doc-discharge-1".into(), "rev-10".into()),
        vec![AttachmentPart {
            attachment_id: "att-holter".into(),
            label: "Holter Report".into(),
            media_type: Some("application/pdf".into()),
            state: PartState::Readable {
                text: attachment_text.clone(),
            },
        }],
    );
    let mut scans = MapScans::default();
    scans.by_revision.insert(
        ("doc-discharge-1".into(), "rev-10".into()),
        vec![ScanDocument {
            scan_id: "fax-outside-labs".into(),
            label: "Outside Lab Fax".into(),
            content: ScanDocumentState::Pages {
                pages: vec![
                    ScanPageEntry {
                        page: page(1),
                        state: PartState::Readable {
                            text: scan_page_1.clone(),
                        },
                    },
                    ScanPageEntry {
                        page: page(2),
                        state: PartState::Readable {
                            text: scan_page_2.clone(),
                        },
                    },
                ],
            },
        }],
    );
    let reader = CompositePartReader::new(attachments, scans);
    let catalog = VersionedRecordCatalog::from_current(1, std::slice::from_ref(&source));
    let access = PatientAccess::Allowed {
        workspace: "pw-ac1".into(),
    };

    // Prime 53-byte budget forces repeated splits near multi-byte UTF-8 characters and part boundaries.
    let budget = PageBudget::new(53).unwrap();
    let mut location = RecordLocation::Whole;
    let mut reassembled: BTreeMap<PartIdentity, (usize, String)> = BTreeMap::new();
    let mut call_count = 0usize;

    loop {
        call_count += 1;
        assert!(call_count <= 500, "continuation must make strict progress");
        let request = RecordReadRequest::new("doc-discharge-1", budget)
            .with_source("ehr-main")
            .with_location(location.clone());
        let outcome = read_patient_record(&access, &request, &catalog, &reader).unwrap();
        let PatientRecordRead::Authorized(RecordReadResult::Opened(opened)) = outcome else {
            panic!("expected opened record, got {outcome:?}");
        };
        assert_eq!(opened.envelope.candidate_id, "doc-discharge-1");
        assert_eq!(opened.envelope.revision, "rev-10");
        assert!(matches!(
            opened.location,
            LocationResolution::Recovered { .. }
        ));

        let mut delivered_bytes = 0usize;
        for item in opened.parts {
            let DeliveredPart::Readable(slice) = item else {
                panic!("all parts in this document are readable, got {item:?}");
            };
            delivered_bytes += slice.content.len();
            let entry = reassembled
                .entry(slice.part.clone())
                .or_insert_with(|| (slice.total_bytes, String::new()));
            assert_eq!(entry.0, slice.total_bytes);
            assert_eq!(
                slice.start_byte,
                entry.1.len(),
                "slices must be contiguous with nothing cut or skipped"
            );
            entry.1.push_str(&slice.content);
            assert_eq!(slice.end_byte, entry.1.len());
        }
        assert!(
            delivered_bytes <= budget.get(),
            "delivered content {delivered_bytes} exceeded budget {}",
            budget.get()
        );

        match opened.completion {
            ReadCompletion::Partial(cursor) => {
                location = RecordLocation::Continuation(cursor);
            }
            ReadCompletion::Complete => break,
        }
    }

    assert!(
        call_count > 20,
        "large document should span many continuation calls"
    );
    assert_eq!(reassembled.len(), 5);
    for (total_bytes, text) in reassembled.values() {
        assert_eq!(
            text.len(),
            *total_bytes,
            "every part must finish at exact total_bytes"
        );
    }
    assert_eq!(reassembled[&PartIdentity::NativeRecord].1, native);
    assert_eq!(
        reassembled[&PartIdentity::Retained {
            index: 0,
            label: "hl7_addendum".into(),
        }]
            .1,
        retained_text
    );
    assert_eq!(
        reassembled[&PartIdentity::Attachment {
            attachment_id: "att-holter".into(),
            label: "Holter Report".into(),
            media_type: Some("application/pdf".into()),
        }]
            .1,
        attachment_text
    );
    assert_eq!(
        reassembled[&PartIdentity::ScanPage {
            scan_id: "fax-outside-labs".into(),
            label: "Outside Lab Fax".into(),
            page: page(1),
            total_pages: Some(page(2)),
        }]
            .1,
        scan_page_1
    );
    assert_eq!(
        reassembled[&PartIdentity::ScanPage {
            scan_id: "fax-outside-labs".into(),
            label: "Outside Lab Fax".into(),
            page: page(2),
            total_pages: Some(page(2)),
        }]
            .1,
        scan_page_2
    );
}

#[test]
fn ac2_unreadable_part_does_not_stop_others_names_failed_part_and_distinguishes_outcomes() {
    let source = make_revision(RevisionFixture {
        workspace: "pw-ac2",
        source_id: "ehr",
        candidate_id: "consult-cardio",
        revision: "rev-1",
        record_type: "DiagnosticReport",
        date: "2026-10-02",
        content: "Cardiology packet cover note: see attached ECG, echo, prior labs, and 3-page fax.",
        readable_parts: vec![ReadablePart {
            label: "cover_ addendum".into(),
            text: "Inline decoded cover note text.".into(),
        }],
    });

    let mut attachments = MapAttachments::default();
    attachments.by_revision.insert(
        ("consult-cardio".into(), "rev-1".into()),
        vec![
            AttachmentPart {
                attachment_id: "att-echo".into(),
                label: "Transthoracic Echo".into(),
                media_type: Some("text/plain".into()),
                state: PartState::Readable {
                    text: "LVEF 60%, normal valvular function.".into(),
                },
            },
            AttachmentPart {
                attachment_id: "att-ecg-corrupt".into(),
                label: "12-Lead ECG Strip".into(),
                media_type: Some("application/pdf".into()),
                state: PartState::Unreadable {
                    reason: "corrupt PDF cross-reference table".into(),
                },
            },
            AttachmentPart {
                attachment_id: "att-prior-labs".into(),
                label: "Prior Outside Chemistry".into(),
                media_type: None,
                state: PartState::Missing {
                    detail: "source manifest marks attachment not supplied by sender".into(),
                },
            },
            AttachmentPart {
                attachment_id: "att-holter-remote".into(),
                label: "Archived Holter Strip".into(),
                media_type: Some("application/dicom".into()),
                state: PartState::Unavailable {
                    reason: "external archive offline during pull".into(),
                },
            },
            AttachmentPart {
                attachment_id: "att-plan".into(),
                label: "Medication Plan".into(),
                media_type: Some("text/plain".into()),
                state: PartState::Readable {
                    text: "Continue lisinopril 10 mg daily.".into(),
                },
            },
        ],
    );

    let mut scans = MapScans::default();
    scans.by_revision.insert(
        ("consult-cardio".into(), "rev-1".into()),
        vec![ScanDocument {
            scan_id: "scan-fax-7".into(),
            label: "Referral Fax".into(),
            content: ScanDocumentState::Pages {
                pages: vec![
                    ScanPageEntry {
                        page: page(1),
                        state: PartState::Readable {
                            text: "Referral Fax p1: resting BP 124/76.".into(),
                        },
                    },
                    ScanPageEntry {
                        page: page(2),
                        state: PartState::Unreadable {
                            reason: "smudged thermal scan; OCR confidence below threshold".into(),
                        },
                    },
                    ScanPageEntry {
                        page: page(3),
                        state: PartState::Readable {
                            text: "Referral Fax p3: return to clinic in 3 months.".into(),
                        },
                    },
                ],
            },
        }],
    );

    let reader = CompositePartReader::new(attachments, scans);
    let catalog = VersionedRecordCatalog::from_current(4, std::slice::from_ref(&source));
    let access = PatientAccess::Allowed {
        workspace: "pw-ac2".into(),
    };

    // Exercise continuation across mixed readable / unreadable / missing / unavailable parts.
    let budget = PageBudget::new(40).unwrap();
    let mut location = RecordLocation::Whole;
    let mut readable_by_part: BTreeMap<PartIdentity, String> = BTreeMap::new();
    let mut non_readable: Vec<DeliveredPart> = Vec::new();

    loop {
        let request = RecordReadRequest::new("consult-cardio", budget).with_location(location);
        let outcome = read_patient_record(&access, &request, &catalog, &reader).unwrap();
        let PatientRecordRead::Authorized(RecordReadResult::Opened(opened)) = outcome else {
            panic!("expected opened record, got {outcome:?}");
        };
        for item in opened.parts {
            match item {
                DeliveredPart::Readable(slice) => {
                    readable_by_part
                        .entry(slice.part)
                        .or_default()
                        .push_str(&slice.content);
                }
                other @ (DeliveredPart::Missing { .. }
                | DeliveredPart::Unreadable { .. }
                | DeliveredPart::Unavailable { .. }) => non_readable.push(other),
            }
        }
        match opened.completion {
            ReadCompletion::Partial(cursor) => location = RecordLocation::Continuation(cursor),
            ReadCompletion::Complete => break,
        }
    }

    // Readable parts both before AND after unreadable/missing/unavailable parts were delivered completely.
    assert_eq!(readable_by_part.len(), 6);
    assert_eq!(
        readable_by_part[&PartIdentity::Attachment {
            attachment_id: "att-echo".into(),
            label: "Transthoracic Echo".into(),
            media_type: Some("text/plain".into()),
        }],
        "LVEF 60%, normal valvular function."
    );
    assert_eq!(
        readable_by_part[&PartIdentity::Attachment {
            attachment_id: "att-plan".into(),
            label: "Medication Plan".into(),
            media_type: Some("text/plain".into()),
        }],
        "Continue lisinopril 10 mg daily."
    );
    assert_eq!(
        readable_by_part[&PartIdentity::ScanPage {
            scan_id: "scan-fax-7".into(),
            label: "Referral Fax".into(),
            page: page(1),
            total_pages: Some(page(3)),
        }],
        "Referral Fax p1: resting BP 124/76."
    );
    assert_eq!(
        readable_by_part[&PartIdentity::ScanPage {
            scan_id: "scan-fax-7".into(),
            label: "Referral Fax".into(),
            page: page(3),
            total_pages: Some(page(3)),
        }],
        "Referral Fax p3: return to clinic in 3 months."
    );

    // Missing, Unreadable, and Unavailable each produced their own distinct typed variant naming the exact part.
    assert_eq!(
        non_readable,
        vec![
            DeliveredPart::Unreadable {
                part: PartIdentity::Attachment {
                    attachment_id: "att-ecg-corrupt".into(),
                    label: "12-Lead ECG Strip".into(),
                    media_type: Some("application/pdf".into()),
                },
                reason: "corrupt PDF cross-reference table".into(),
            },
            DeliveredPart::Missing {
                part: PartIdentity::Attachment {
                    attachment_id: "att-prior-labs".into(),
                    label: "Prior Outside Chemistry".into(),
                    media_type: None,
                },
                detail: "source manifest marks attachment not supplied by sender".into(),
            },
            DeliveredPart::Unavailable {
                part: PartIdentity::Attachment {
                    attachment_id: "att-holter-remote".into(),
                    label: "Archived Holter Strip".into(),
                    media_type: Some("application/dicom".into()),
                },
                reason: "external archive offline during pull".into(),
            },
            DeliveredPart::Unreadable {
                part: PartIdentity::ScanPage {
                    scan_id: "scan-fax-7".into(),
                    label: "Referral Fax".into(),
                    page: page(2),
                    total_pages: Some(page(3)),
                },
                reason: "smudged thermal scan; OCR confidence below threshold".into(),
            },
        ]
    );

    // Record-level Missing vs Unavailable are also distinct typed outcomes.
    let full_catalog = VersionedRecordCatalog::from_observations(
        5,
        vec![
            ObservationEntry::Present {
                observed_at_version: 1,
                revision: source,
            },
            ObservationEntry::Present {
                observed_at_version: 2,
                revision: make_revision(RevisionFixture {
                    workspace: "pw-ac2",
                    source_id: "ehr",
                    candidate_id: "withdrawn-lab",
                    revision: "rev-1",
                    record_type: "Observation",
                    date: "2026-09-01",
                    content: "Temp lab",
                    readable_parts: vec![],
                }),
            },
            ObservationEntry::Withdrawn {
                observed_at_version: 5,
                candidate_id: "withdrawn-lab".into(),
                workspace: "pw-ac2".into(),
                source_id: Some("ehr".into()),
            },
        ],
    );
    let missing_record = read_patient_record(
        &access,
        &RecordReadRequest::new("never-recorded", PageBudget::default()),
        &full_catalog,
        &InlinePartReader,
    )
    .unwrap();
    let withdrawn_record = read_patient_record(
        &access,
        &RecordReadRequest::new("withdrawn-lab", PageBudget::default()),
        &full_catalog,
        &InlinePartReader,
    )
    .unwrap();
    assert_eq!(
        missing_record,
        PatientRecordRead::Authorized(RecordReadResult::Missing {
            candidate_id: "never-recorded".into(),
            source: SourceSelector::AnySource,
            reason: RecordMissingReason::UnknownRecord,
        })
    );
    assert_eq!(
        withdrawn_record,
        PatientRecordRead::Authorized(RecordReadResult::Unavailable {
            candidate_id: "withdrawn-lab".into(),
            source: SourceSelector::AnySource,
            reason: RecordUnavailableReason::Withdrawn,
        })
    );
    assert_ne!(missing_record, withdrawn_record);
}

#[test]
fn ac3_older_version_and_distinct_locations_resolve_and_unrecoverable_location_returns_same_source_while_failure_stays_error()
 {
    let v1 = make_revision(RevisionFixture {
        workspace: "pw-ac3",
        source_id: "ehr",
        candidate_id: "note-htn",
        revision: "rev-v1",
        record_type: "DocumentReference",
        date: "2025-05-10",
        content: "Assessment v1: BP 152/94 mmHg; start amlodipine 5 mg.",
        readable_parts: vec![
            ReadablePart {
                label: "medication_table".into(),
                text: "v1 meds: amlodipine 5 mg daily".into(),
            },
            ReadablePart {
                label: "vitals_flowsheet".into(),
                text: "v1 vitals: seated BP 152/94, HR 74".into(),
            },
        ],
    });
    let mut v2 = make_revision(RevisionFixture {
        workspace: "pw-ac3",
        source_id: "ehr",
        candidate_id: "note-htn",
        revision: "rev-v2",
        record_type: "DocumentReference",
        date: "2026-02-18",
        content: "Assessment v2: BP 126/78 mmHg; controlled on amlodipine 10 mg.",
        readable_parts: vec![
            ReadablePart {
                label: "medication_table".into(),
                text: "v2 meds: amlodipine 10 mg daily".into(),
            },
            ReadablePart {
                label: "vitals_flowsheet".into(),
                text: "v2 vitals: seated BP 126/78, HR 68".into(),
            },
        ],
    });
    v2.provenance = json!({
        "source_id": "ehr",
        "native_id": "note-htn",
        "scans": [{
            "scan_id": "fax-home-bp",
            "label": "Home BP Log Scan",
            "content": {
                "status": "pages",
                "pages": [
                    {"page": 1, "state": {"state": "readable", "text": "Week 1 log: avg 128/80"}},
                    {"page": 2, "state": {"state": "readable", "text": "Week 2 log: avg 124/76"}},
                    {"page": 3, "state": {"state": "unreadable", "reason": "page 3 torn during scan"}}
                ]
            }
        }]
    });

    let mut scans = MapScans::default();
    scans.by_revision.insert(
        ("note-htn".into(), "rev-v1".into()),
        vec![ScanDocument {
            scan_id: "fax-home-bp".into(),
            label: "Home BP Log Scan".into(),
            content: ScanDocumentState::Pages {
                pages: vec![
                    ScanPageEntry {
                        page: page(1),
                        state: PartState::Readable {
                            text: "2025 Week 1 log: avg 150/92".into(),
                        },
                    },
                    ScanPageEntry {
                        page: page(2),
                        state: PartState::Readable {
                            text: "2025 Week 2 log: avg 148/90".into(),
                        },
                    },
                ],
            },
        }],
    );

    let catalog = VersionedRecordCatalog::from_observations(
        2,
        vec![
            ObservationEntry::Present {
                observed_at_version: 1,
                revision: v1.clone(),
            },
            ObservationEntry::Present {
                observed_at_version: 2,
                revision: v2.clone(),
            },
        ],
    );
    let access = PatientAccess::Allowed {
        workspace: "pw-ac3".into(),
    };
    let v1_reader = CompositePartReader::new(
        local_product::physician_sources::read::NoExternalReader,
        scans,
    );

    // 1. Requesting older version rev-v1 at ScanPage 2 returns rev-v1 and Page 2's content (not v2, not Page 1).
    let older_page_2 = read_patient_record(
        &access,
        &RecordReadRequest::new("note-htn", PageBudget::default())
            .with_version(VersionSelector::Revision("rev-v1".into()))
            .with_location(RecordLocation::ScanPage {
                scan: ScanSelector::Named("fax-home-bp".into()),
                page: page(2),
                slice: ContentSlice::Full,
            }),
        &catalog,
        &v1_reader,
    )
    .unwrap();
    let PatientRecordRead::Authorized(RecordReadResult::Opened(opened_v1_p2)) = older_page_2 else {
        panic!("expected opened older version, got {older_page_2:?}");
    };
    assert_eq!(opened_v1_p2.envelope.revision, "rev-v1");
    assert_eq!(
        opened_v1_p2.version_resolution,
        VersionResolution::Historical {
            standing: HistoricalStanding::SupersededBy {
                current_revision: "rev-v2".into(),
            }
        }
    );
    assert!(matches!(
        opened_v1_p2.location,
        LocationResolution::Recovered { .. }
    ));
    assert_eq!(
        opened_v1_p2.parts,
        vec![DeliveredPart::Readable(
            local_product::physician_sources::read::PartSlice {
                part: PartIdentity::ScanPage {
                    scan_id: "fax-home-bp".into(),
                    label: "Home BP Log Scan".into(),
                    page: page(2),
                    total_pages: Some(page(2)),
                },
                start_byte: 0,
                end_byte: "2025 Week 2 log: avg 148/90".len(),
                total_bytes: "2025 Week 2 log: avg 148/90".len(),
                content: "2025 Week 2 log: avg 148/90".into(),
            }
        )]
    );

    // 2. ObservedAt(1) also selects v1, and different locations return their own content.
    let v1_meds = read_patient_record(
        &access,
        &RecordReadRequest::new("note-htn", PageBudget::default())
            .with_version(VersionSelector::ObservedAt(1))
            .with_location(RecordLocation::Part {
                part: PartSelector::Label("medication_table".into()),
                slice: ContentSlice::Full,
            }),
        &catalog,
        &v1_reader,
    )
    .unwrap();
    let v1_vitals = read_patient_record(
        &access,
        &RecordReadRequest::new("note-htn", PageBudget::default())
            .with_version(VersionSelector::Revision("rev-v1".into()))
            .with_location(RecordLocation::Part {
                part: PartSelector::Label("vitals_flowsheet".into()),
                slice: ContentSlice::ByteSpan(ByteSpan {
                    start_byte: 11,
                    end_byte: 27,
                }),
            }),
        &catalog,
        &v1_reader,
    )
    .unwrap();
    let PatientRecordRead::Authorized(RecordReadResult::Opened(opened_meds)) = v1_meds else {
        panic!("expected meds part");
    };
    let PatientRecordRead::Authorized(RecordReadResult::Opened(opened_vitals)) = v1_vitals else {
        panic!("expected vitals byte span");
    };
    let DeliveredPart::Readable(meds_slice) = &opened_meds.parts[0] else {
        panic!("expected readable meds");
    };
    let DeliveredPart::Readable(vitals_slice) = &opened_vitals.parts[0] else {
        panic!("expected readable vitals");
    };
    assert_eq!(meds_slice.content, "v1 meds: amlodipine 5 mg daily");
    assert_eq!(vitals_slice.content, "seated BP 152/94");
    assert_ne!(meds_slice.content, vitals_slice.content);

    // Latest version on ScanPage 1 vs ScanPage 2 via ProvenancePartReader returns v2's distinct pages.
    let v2_p1 = read_patient_record(
        &access,
        &RecordReadRequest::new("note-htn", PageBudget::default()).with_location(
            RecordLocation::ScanPage {
                scan: ScanSelector::Primary,
                page: page(1),
                slice: ContentSlice::Full,
            },
        ),
        &catalog,
        &ProvenancePartReader,
    )
    .unwrap();
    let v2_p2 = read_patient_record(
        &access,
        &RecordReadRequest::new("note-htn", PageBudget::default()).with_location(
            RecordLocation::ScanPage {
                scan: ScanSelector::Primary,
                page: page(2),
                slice: ContentSlice::Full,
            },
        ),
        &catalog,
        &ProvenancePartReader,
    )
    .unwrap();
    let PatientRecordRead::Authorized(RecordReadResult::Opened(v2_p1)) = v2_p1 else {
        panic!("expected v2 page 1");
    };
    let PatientRecordRead::Authorized(RecordReadResult::Opened(v2_p2)) = v2_p2 else {
        panic!("expected v2 page 2");
    };
    assert_eq!(v2_p1.envelope.revision, "rev-v2");
    let DeliveredPart::Readable(s1) = &v2_p1.parts[0] else {
        panic!("expected readable page 1");
    };
    let DeliveredPart::Readable(s2) = &v2_p2.parts[0] else {
        panic!("expected readable page 2");
    };
    assert_eq!(s1.content, "Week 1 log: avg 128/80");
    assert_eq!(s2.content, "Week 2 log: avg 124/76");

    // 3. Unrecoverable locations on a valid source return THAT SAME SOURCE with a stated limit.
    let unrecoverable_cases = vec![
        RecordLocation::ScanPage {
            scan: ScanSelector::Primary,
            page: page(9),
            slice: ContentSlice::Full,
        },
        RecordLocation::ScanPage {
            scan: ScanSelector::Primary,
            page: page(3), // page 3 is unreadable on v2
            slice: ContentSlice::Full,
        },
        RecordLocation::Part {
            part: PartSelector::Label("nonexistent_section".into()),
            slice: ContentSlice::Full,
        },
        RecordLocation::Part {
            part: PartSelector::NativeRecord,
            slice: ContentSlice::ByteSpan(ByteSpan {
                start_byte: 0,
                end_byte: 9999,
            }),
        },
        RecordLocation::Continuation(ContinuationCursor {
            source_id: "ehr".into(),
            candidate_id: "note-htn".into(),
            revision: "wrong-revision".into(),
            part_ordinal: 0,
            byte_offset: 0,
            scope: ContinuationScope::Document,
        }),
    ];
    for requested_location in unrecoverable_cases {
        let fallback = read_patient_record(
            &access,
            &RecordReadRequest::new("note-htn", PageBudget::default())
                .with_location(requested_location.clone()),
            &catalog,
            &ProvenancePartReader,
        )
        .unwrap();
        let PatientRecordRead::Authorized(RecordReadResult::Opened(opened)) = fallback else {
            panic!("unrecoverable location must still open same source, got {fallback:?}");
        };
        assert_eq!(opened.envelope.candidate_id, "note-htn");
        assert_eq!(opened.envelope.revision, "rev-v2");
        let LocationResolution::Unrecoverable {
            requested,
            limit,
            stated_limit,
        } = &opened.location
        else {
            panic!(
                "expected Unrecoverable location resolution, got {:?}",
                opened.location
            );
        };
        assert_eq!(requested, &requested_location);
        assert!(!stated_limit.trim().is_empty());
        assert!(matches!(
            limit,
            LocationLimit::PageOutOfRange { .. }
                | LocationLimit::TargetPartNotReadable {
                    state: NonReadableOutcome::Unreadable { .. },
                    ..
                }
                | LocationLimit::PartNotFound { .. }
                | LocationLimit::SpanOutOfBounds { .. }
                | LocationLimit::InvalidContinuation { .. }
        ));
        // Returns the same source starting from its native content.
        assert!(opened.parts.iter().any(|part| matches!(
            part,
            DeliveredPart::Readable(slice)
                if slice.part == PartIdentity::NativeRecord && slice.content == v2.content
        )));
    }

    // Also check ChartRequest (READ_TOOL) with older version, explicit location, and unrecoverable cursor/location.
    let chart_older = ChartRequest::parse(
        READ_TOOL,
        &json!({
            "references": [{
                "candidate_id": "note-htn",
                "revision": "rev-v1",
                "cursor": null
            }],
            "max_result_bytes": 4096
        }),
        ChartScope::Chat,
    )
    .unwrap();
    let chart_out = chart_older
        .run("pw-ac3", 2, &[v2.clone(), v1.clone()])
        .unwrap();
    let chart_json: serde_json::Value = serde_json::from_str(chart_out.text().unwrap()).unwrap();
    assert_eq!(chart_json["results"][0]["revision"], "rev-v1");
    assert_eq!(chart_json["results"][0]["content"], v1.content);
    assert_eq!(chart_json["results"][0]["superseded_by"], "rev-v2");

    let chart_scan_p2 = ChartRequest::parse(
        READ_TOOL,
        &json!({
            "references": [{
                "candidate_id": "note-htn",
                "revision": "rev-v2",
                "cursor": null,
                "location": {
                    "kind": "scan_page",
                    "scan": {"scope": "primary"},
                    "page": 2
                }
            }],
            "max_result_bytes": 4096
        }),
        ChartScope::Chat,
    )
    .unwrap();
    let scan_p2_out = chart_scan_p2
        .run("pw-ac3", 2, std::slice::from_ref(&v2))
        .unwrap();
    let scan_p2_json: serde_json::Value =
        serde_json::from_str(scan_p2_out.text().unwrap()).unwrap();
    assert_eq!(scan_p2_json["results"][0]["outcome"], "passage");
    assert_eq!(
        scan_p2_json["results"][0]["content"],
        "Week 2 log: avg 124/76"
    );

    let chart_bad_cursor = ChartRequest::parse(
        READ_TOOL,
        &json!({
            "references": [{
                "candidate_id": "note-htn",
                "revision": "rev-v2",
                "cursor": {"part": 99, "offset": 0}
            }],
            "max_result_bytes": 4096
        }),
        ChartScope::Chat,
    )
    .unwrap();
    let bad_cursor_out = chart_bad_cursor
        .run("pw-ac3", 2, std::slice::from_ref(&v2))
        .unwrap();
    let bad_cursor_json: serde_json::Value =
        serde_json::from_str(bad_cursor_out.text().unwrap()).unwrap();
    assert_eq!(bad_cursor_json["results"][0]["reference"], "note-htn");
    assert_eq!(bad_cursor_json["results"][0]["revision"], "rev-v2");
    assert_eq!(bad_cursor_json["results"][0]["content"], v2.content);
    assert!(
        bad_cursor_json["results"][0]["location_limit"]
            .as_str()
            .is_some_and(|s| !s.is_empty()),
        "unrecoverable cursor on valid source must return same source with stated limit: {bad_cursor_json}"
    );

    // 4. Source read failure STAYS a failure (Err), never empty, Missing, or Unavailable.
    let mut corrupt = v2.clone();
    corrupt.content.push_str(" tampered");
    let corrupt_catalog = VersionedRecordCatalog::from_current(3, &[corrupt]);
    let digest_err = read_patient_record(
        &access,
        &RecordReadRequest::new("note-htn", PageBudget::default()),
        &corrupt_catalog,
        &InlinePartReader,
    )
    .expect_err("corrupt source digest must fail the read");
    assert!(matches!(
        digest_err,
        RecordReadError::InvalidSourceRevision {
            ref candidate_id,
            source: InvalidSourceRevisionError::DigestMismatch { .. },
            ..
        } if candidate_id == "note-htn"
    ));
    assert!(digest_err.source().is_some());

    let reader_err = read_patient_record(
        &access,
        &RecordReadRequest::new("note-htn", PageBudget::default()),
        &catalog,
        &CompositePartReader::new(
            local_product::physician_sources::read::NoExternalReader,
            FailingScanReader,
        ),
    )
    .expect_err("broken scan reader must fail the read");
    assert!(matches!(reader_err, RecordReadError::PartReader { .. }));
    assert!(
        reader_err
            .source()
            .and_then(|inner| inner.source())
            .is_some_and(|cause| cause.to_string().contains("truncated mid-frame"))
    );
}

#[tokio::test]
async fn ac4_physician_without_patient_access_gets_nothing() {
    let auth = MemoryAuthStore::new();
    auth.seed_practice("practice-alpha", PracticeClock::RealTime);
    auth.seed_practice("practice-beta", PracticeClock::RealTime);
    auth.seed_patient("practice-alpha", "pw-restricted");

    let dr_panel = PhysicianIdentity {
        practice_id: "practice-alpha".into(),
        physician_id: "dr-panel".into(),
    };
    let dr_clinic_only = PhysicianIdentity {
        practice_id: "practice-alpha".into(),
        physician_id: "dr-clinic-only".into(),
    };
    let dr_denied = PhysicianIdentity {
        practice_id: "practice-alpha".into(),
        physician_id: "dr-denied".into(),
    };
    let dr_other_practice = PhysicianIdentity {
        practice_id: "practice-beta".into(),
        physician_id: "dr-other".into(),
    };

    auth.seed_membership(&dr_panel, "pw-restricted", Membership::Allowed);
    auth.seed_membership(
        &dr_clinic_only,
        "pw-restricted",
        Membership::ClinicDemographic,
    );
    auth.seed_membership(&dr_denied, "pw-restricted", Membership::Denied);
    auth.seed_membership(&dr_other_practice, "pw-restricted", Membership::Allowed);

    let secret_record = make_revision(RevisionFixture {
        workspace: "pw-restricted",
        source_id: "ehr",
        candidate_id: "hiv-panel-1",
        revision: "rev-secret",
        record_type: "Observation",
        date: "2026-10-01",
        content: "Confidential clinical payload",
        readable_parts: vec![],
    });
    let catalog = VersionedRecordCatalog::from_current(1, std::slice::from_ref(&secret_record));
    let patient_practice = auth.patient_practice("pw-restricted").await.unwrap();

    // Panel physician gets the record.
    let panel_access = PatientAccess::for_physician(
        &dr_panel.practice_id,
        patient_practice.as_deref(),
        "pw-restricted",
        map_standing(auth.membership(&dr_panel, "pw-restricted").await),
    );
    let allowed = read_patient_record(
        &panel_access,
        &RecordReadRequest::new("hiv-panel-1", PageBudget::default()),
        &catalog,
        &InlinePartReader,
    )
    .unwrap();
    assert!(matches!(
        allowed,
        PatientRecordRead::Authorized(RecordReadResult::Opened(_))
    ));

    // ClinicDemographic physician, Denied physician, and cross-practice physician all get NOTHING,
    // and cannot even distinguish existing vs nonexistent records or trigger part readers.
    for unauthorized in [&dr_clinic_only, &dr_denied, &dr_other_practice] {
        let access = PatientAccess::for_physician(
            &unauthorized.practice_id,
            patient_practice.as_deref(),
            "pw-restricted",
            map_standing(auth.membership(unauthorized, "pw-restricted").await),
        );
        for candidate in ["hiv-panel-1", "does-not-exist"] {
            let result = read_patient_record(
                &access,
                &RecordReadRequest::new(candidate, PageBudget::default()),
                &catalog,
                &CompositePartReader::new(
                    local_product::physician_sources::read::NoExternalReader,
                    FailingScanReader,
                ),
            )
            .unwrap();
            assert_eq!(result, PatientRecordRead::Denied);
            let serialized = serde_json::to_string(&result).unwrap();
            assert_eq!(serialized, r#"{"access":"denied"}"#);
            assert!(!serialized.contains("Confidential"));
            assert!(!serialized.contains("rev-secret"));
            assert!(!serialized.contains("hiv-panel-1"));
        }
    }
}

struct PostgresAuthority;

#[async_trait]
impl Authority for PostgresAuthority {
    async fn check(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        PostgresAuthStore::check_runtime_authority(
            tx,
            session_hash.ok_or("missing session hash")?,
            &PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|e| e.to_string())
    }
}

#[tokio::test]
async fn postgres_store_reads_older_versions_locations_and_denies_unauthorized_physicians() {
    let Some(pg) = DisposablePostgres::acquire("record-read-pg").await else {
        return;
    };
    let store = pg.store().await;
    let auth = pg.auth_store().await;

    let practice = format!("practice-{}", uuid::Uuid::new_v4().simple());
    let workspace = format!("pw-{}", uuid::Uuid::new_v4().simple());
    let untouched_workspace = format!("pw-untouched-{}", uuid::Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.register_patient(&practice, &untouched_workspace)
        .await
        .unwrap();

    let allowed_email = format!("allowed-{practice}@example.test");
    let denied_email = format!("denied-{practice}@example.test");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let inv_allowed = auth
        .invite(&practice, &allowed_email, "Dr. Allowed", false, now)
        .await
        .unwrap();
    let (allowed_id, allowed_token) = auth
        .redeem_invitation(&allowed_email, &inv_allowed, now + 1)
        .await
        .unwrap();
    auth.set_membership(&allowed_id, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let inv_denied = auth
        .invite(&practice, &denied_email, "Dr. Denied", false, now)
        .await
        .unwrap();
    let (denied_id, denied_token) = auth
        .redeem_invitation(&denied_email, &inv_denied, now + 1)
        .await
        .unwrap();
    auth.set_membership(&denied_id, &workspace, Membership::ClinicDemographic)
        .await
        .unwrap();

    let allowed_session = auth
        .verified_session(&allowed_token, now + 2)
        .await
        .unwrap();
    let denied_session = auth.verified_session(&denied_token, now + 2).await.unwrap();

    let v1 = make_revision(RevisionFixture {
        workspace: &workspace,
        source_id: "ehr",
        candidate_id: "renal-panel",
        revision: "rev-1",
        record_type: "Observation",
        date: "2025-11-03",
        content: "Creatinine 1.8 mg/dL (elevated)",
        readable_parts: vec![ReadablePart {
            label: "comment".into(),
            text: "v1 lab comment: repeat in 1 week".into(),
        }],
    });
    let v2 = make_revision(RevisionFixture {
        workspace: &workspace,
        source_id: "ehr",
        candidate_id: "renal-panel",
        revision: "rev-2",
        record_type: "Observation",
        date: "2025-11-10",
        content: "Creatinine 1.1 mg/dL (baseline)",
        readable_parts: vec![ReadablePart {
            label: "comment".into(),
            text: "v2 lab comment: resolved".into(),
        }],
    });
    let pull = SourcePull {
        source_id: "ehr".into(),
        source_label: "EHR".into(),
        patient: SourcePatient {
            source_patient_id: "pat-1".into(),
            display_name: "Mary Louis".into(),
            birth_date: None,
            sex_or_gender: None,
            medical_record_numbers: vec![],
        },
        pulled_at: 100,
    };
    store
        .record_chart(&workspace, std::slice::from_ref(&v1), &pull)
        .await
        .unwrap();
    store
        .record_chart(&workspace, std::slice::from_ref(&v2), &pull)
        .await
        .unwrap();

    let allowed_scope = Scope {
        practice: practice.clone(),
        physician: allowed_id.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "record-read".into(),
    };
    let denied_scope = Scope {
        practice: practice.clone(),
        physician: denied_id.physician_id.clone(),
        workspace: workspace.clone(),
        chat: "record-read".into(),
    };

    // Authorized physician reads older rev-1 and its retained comment part from Postgres!
    let older = store
        .read_patient_record(
            &allowed_scope,
            Some(&allowed_session.token_hash),
            &mut PostgresAuthority,
            &RecordReadRequest::new("renal-panel", PageBudget::default())
                .with_version(VersionSelector::Revision("rev-1".into()))
                .with_location(RecordLocation::Part {
                    part: PartSelector::Label("comment".into()),
                    slice: ContentSlice::Full,
                }),
            &InlinePartReader,
        )
        .await
        .unwrap();
    let PatientRecordRead::Authorized(RecordReadResult::Opened(opened_older)) = older else {
        panic!("expected authorized older version from Postgres, got {older:?}");
    };
    assert_eq!(opened_older.envelope.revision, "rev-1");
    assert_eq!(
        opened_older.version_resolution,
        VersionResolution::Historical {
            standing: HistoricalStanding::SupersededBy {
                current_revision: "rev-2".into(),
            }
        }
    );
    let DeliveredPart::Readable(comment_slice) = &opened_older.parts[0] else {
        panic!("expected readable comment part");
    };
    assert_eq!(comment_slice.content, "v1 lab comment: repeat in 1 week");

    // ChartRequest + Store::chart_observation also resolves retained historical revision rev-1!
    let chart_req = ChartRequest::parse(
        READ_TOOL,
        &json!({
            "references": [{"candidate_id": "renal-panel", "revision": "rev-1", "cursor": null}],
            "max_result_bytes": 4096
        }),
        ChartScope::Chat,
    )
    .unwrap();
    let obs = store
        .chart_observation(&workspace, &chart_req)
        .await
        .unwrap();
    let tool_out = chart_req
        .run(&workspace, obs.version, &obs.sources)
        .unwrap();
    let tool_json: serde_json::Value = serde_json::from_str(tool_out.text().unwrap()).unwrap();
    assert_eq!(tool_json["results"][0]["revision"], "rev-1");
    assert_eq!(tool_json["results"][0]["content"], v1.content);
    assert_eq!(tool_json["results"][0]["superseded_by"], "rev-2");

    // Unauthorized physician (ClinicDemographic, not on panel) gets Denied (nothing!),
    // including on an untouched workspace without creating source rows.
    let denied_out = store
        .read_patient_record(
            &denied_scope,
            Some(&denied_session.token_hash),
            &mut PostgresAuthority,
            &RecordReadRequest::new("renal-panel", PageBudget::default()),
            &InlinePartReader,
        )
        .await
        .unwrap();
    assert_eq!(denied_out, PatientRecordRead::Denied);

    let untouched_denied_scope = Scope {
        practice: practice.clone(),
        physician: denied_id.physician_id.clone(),
        workspace: untouched_workspace.clone(),
        chat: "record-read".into(),
    };
    let untouched_out = store
        .read_patient_record(
            &untouched_denied_scope,
            Some(&denied_session.token_hash),
            &mut PostgresAuthority,
            &RecordReadRequest::new("any-record", PageBudget::default()),
            &InlinePartReader,
        )
        .await
        .unwrap();
    assert_eq!(untouched_out, PatientRecordRead::Denied);

    // Revoking the allowed physician's panel access immediately denies subsequent reads too.
    auth.set_membership(&allowed_id, &workspace, Membership::Denied)
        .await
        .unwrap();
    let revoked_out = store
        .read_patient_record(
            &allowed_scope,
            Some(&allowed_session.token_hash),
            &mut PostgresAuthority,
            &RecordReadRequest::new("renal-panel", PageBudget::default()),
            &InlinePartReader,
        )
        .await
        .unwrap();
    assert_eq!(revoked_out, PatientRecordRead::Denied);
}

fn json_value_type(value: &serde_json::Value) -> &'static str {
    match value {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "boolean",
        serde_json::Value::Number(num) => {
            if num.is_i64() || num.is_u64() {
                "integer"
            } else {
                "number"
            }
        }
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}

fn matches_json_schema(schema: &serde_json::Value, value: &serde_json::Value) -> bool {
    if let Some(any_of) = schema.get("anyOf").and_then(serde_json::Value::as_array) {
        return any_of
            .iter()
            .any(|branch| matches_json_schema(branch, value));
    }
    if let Some(expected_type) = schema.get("type") {
        let actual = json_value_type(value);
        let type_ok = match expected_type {
            serde_json::Value::String(kind) => {
                kind == actual || (kind == "number" && actual == "integer")
            }
            serde_json::Value::Array(kinds) => kinds.iter().any(|k| {
                k.as_str()
                    .is_some_and(|kind| kind == actual || (kind == "number" && actual == "integer"))
            }),
            _ => false,
        };
        if !type_ok {
            return false;
        }
    }
    if let Some(allowed) = schema.get("enum").and_then(serde_json::Value::as_array)
        && !allowed.contains(value)
    {
        return false;
    }
    match value {
        serde_json::Value::String(text) => {
            if let Some(min_len) = schema.get("minLength").and_then(serde_json::Value::as_u64)
                && (text.chars().count() as u64) < min_len
            {
                return false;
            }
        }
        serde_json::Value::Number(num) => {
            if let Some(min) = schema.get("minimum").and_then(serde_json::Value::as_f64)
                && num.as_f64().is_none_or(|n| n < min)
            {
                return false;
            }
            if let Some(max) = schema.get("maximum").and_then(serde_json::Value::as_f64)
                && num.as_f64().is_none_or(|n| n > max)
            {
                return false;
            }
        }
        serde_json::Value::Array(items) => {
            if let Some(min_items) = schema.get("minItems").and_then(serde_json::Value::as_u64)
                && (items.len() as u64) < min_items
            {
                return false;
            }
            if let Some(max_items) = schema.get("maxItems").and_then(serde_json::Value::as_u64)
                && (items.len() as u64) > max_items
            {
                return false;
            }
            if let Some(item_schema) = schema.get("items")
                && !items
                    .iter()
                    .all(|item| matches_json_schema(item_schema, item))
            {
                return false;
            }
        }
        serde_json::Value::Object(map) => {
            if let Some(required) = schema.get("required").and_then(serde_json::Value::as_array) {
                for req in required {
                    let Some(key) = req.as_str() else {
                        return false;
                    };
                    if !map.contains_key(key) {
                        return false;
                    }
                }
            }
            let props = schema
                .get("properties")
                .and_then(serde_json::Value::as_object);
            if schema.get("additionalProperties") == Some(&serde_json::Value::Bool(false))
                && map
                    .keys()
                    .any(|key| props.is_none_or(|defined| !defined.contains_key(key)))
            {
                return false;
            }
            if let Some(defined) = props {
                for (key, field_val) in map {
                    if let Some(field_schema) = defined.get(key)
                        && !matches_json_schema(field_schema, field_val)
                    {
                        return false;
                    }
                }
            }
        }
        serde_json::Value::Null | serde_json::Value::Bool(_) => {}
    }
    true
}

#[test]
fn chart_read_tool_schema_accepts_source_id_locations_and_next_continuations_and_rejects_invalid_shapes()
 {
    let chat_tool = read_tool(ChartScope::Chat);
    let reply_tool = read_tool(ChartScope::PinnedReply);
    assert!(chat_tool.description.contains("source_id"));
    assert!(chat_tool.description.contains("location"));
    assert!(chat_tool.description.contains("unreadable_part"));

    let valid_chat_calls = [
        json!({
            "references": [{"candidate_id": "doc-1", "revision": "rev-1", "cursor": null}],
            "max_result_bytes": 4096
        }),
        json!({
            "references": [{
                "candidate_id": "doc-1",
                "revision": "rev-1",
                "cursor": {"part": 0, "offset": 32},
                "source_id": "ehr"
            }],
            "max_result_bytes": 8192
        }),
        json!({
            "references": [{
                "candidate_id": "doc-1",
                "revision": "rev-1",
                "cursor": null,
                "source_id": "ehr",
                "location": {"kind": "whole"}
            }],
            "max_result_bytes": 4096
        }),
        json!({
            "references": [{
                "candidate_id": "doc-1",
                "revision": "rev-1",
                "cursor": null,
                "location": {
                    "kind": "part",
                    "part": {"kind": "attachment_id", "value": "att-1"},
                    "slice": {"slice": "byte_span", "span": {"start_byte": 0, "end_byte": 64}}
                }
            }],
            "max_result_bytes": 4096
        }),
        json!({
            "references": [{
                "candidate_id": "doc-1",
                "revision": "rev-1",
                "cursor": null,
                "location": {
                    "kind": "scan_page",
                    "scan": {"scope": "named", "scan_id": "fax-1"},
                    "page": 2,
                    "slice": {"slice": "full"}
                }
            }],
            "max_result_bytes": 4096
        }),
        json!({
            "references": [{
                "candidate_id": "doc-1",
                "revision": "rev-1",
                "cursor": null,
                "location": {
                    "kind": "continuation",
                    "source_id": "ehr",
                    "candidate_id": "doc-1",
                    "revision": "rev-1",
                    "part_ordinal": 3,
                    "byte_offset": 128,
                    "scope": {"scope": "within_part", "end_byte": 512}
                }
            }],
            "max_result_bytes": 4096
        }),
    ];

    for payload in &valid_chat_calls {
        assert!(
            matches_json_schema(&chat_tool.parameters, payload),
            "chat read_tool schema must accept valid payload: {payload}"
        );
        assert!(
            matches_json_schema(&reply_tool.parameters, payload),
            "pinned-reply read_tool schema must accept valid payload: {payload}"
        );
        ChartRequest::parse(READ_TOOL, payload, ChartScope::Chat)
            .expect("ChartRequest::parse must accept schema-valid chat payload");
    }

    let pinned_only = json!({
        "references": ["doc-1"],
        "max_result_bytes": 4096
    });
    assert!(!matches_json_schema(&chat_tool.parameters, &pinned_only));
    assert!(matches_json_schema(&reply_tool.parameters, &pinned_only));
    assert!(ChartRequest::parse(READ_TOOL, &pinned_only, ChartScope::Chat).is_err());
    assert!(ChartRequest::parse(READ_TOOL, &pinned_only, ChartScope::PinnedReply).is_ok());

    let invalid_calls = [
        json!({
            "references": [{
                "candidate_id": "doc-1",
                "revision": "rev-1",
                "cursor": null,
                "unexpected": true
            }],
            "max_result_bytes": 4096
        }),
        json!({
            "references": [{
                "candidate_id": "doc-1",
                "revision": "rev-1",
                "cursor": null,
                "source_id": ""
            }],
            "max_result_bytes": 4096
        }),
        json!({
            "references": [{
                "candidate_id": "doc-1",
                "revision": "rev-1",
                "cursor": null,
                "location": {"kind": "scan_page", "page": 0}
            }],
            "max_result_bytes": 4096
        }),
        json!({
            "references": [{
                "candidate_id": "doc-1",
                "revision": "rev-1",
                "cursor": null,
                "location": {"kind": "whole", "extra": 1}
            }],
            "max_result_bytes": 4096
        }),
    ];
    for bad in &invalid_calls {
        assert!(
            !matches_json_schema(&chat_tool.parameters, bad),
            "chat read_tool schema must reject invalid payload: {bad}"
        );
        assert!(
            ChartRequest::parse(READ_TOOL, bad, ChartScope::Chat).is_err(),
            "ChartRequest::parse must reject invalid payload: {bad}"
        );
    }
}

#[test]
fn chart_request_continuations_preserve_all_parts_trim_scan_pages_and_name_failed_parts_distinctly()
{
    let chat_schema = read_tool(ChartScope::Chat).parameters;
    let native = "Hospital transfer summary — patient Mary Louis (α/β review) 🔥\n".repeat(82);
    let retained_text =
        "Decoded HL7 chemistry: Na 139, K 4.3, Cr 1.04 mg/dL — verified.\n".repeat(38);
    let att_echo =
        "Echo report: LVEF 60%, normal RV systolic function, no pericardial effusion.\n".repeat(24);
    let att_plan =
        "Discharge plan: continue amlodipine 10 mg daily and recheck BMP in 14 days.\n".repeat(20);
    // ~3,995 bytes (< 4,081-byte budget before JSON envelope overhead, forcing envelope trimming!).
    let scan_page_1 =
        "Outside Fax Page 1/3 — Renal ultrasound: kidneys symmetric, no hydronephrosis. —\n"
            .repeat(47);
    assert!(
        (3900..4050).contains(&scan_page_1.len()),
        "scan_page_1 length {} must sit just below 4081 so JSON envelope trimming runs",
        scan_page_1.len()
    );
    // ~9,200 bytes (spans 3 bounded 4096-byte READ_TOOL envelopes on a single ScanPage!).
    let scan_page_3 =
        "Outside Fax Page 3/3 — Signed nephrology consult addendum (α/β-blocker note) 🔥\n"
            .repeat(115);
    assert!(scan_page_3.len() > 8500);

    let mut packet = make_revision(RevisionFixture {
        workspace: "pw-chart-cont",
        source_id: "ehr-main",
        candidate_id: "pkt-cardiorenal",
        revision: "rev-4",
        record_type: "DocumentReference",
        date: "2026-10-18",
        content: &native,
        readable_parts: vec![ReadablePart {
            label: "hl7_chemistry".into(),
            text: retained_text.clone(),
        }],
    });
    packet.provenance = json!({
        "source_id": "ehr-main",
        "native_id": "pkt-cardiorenal",
        "attachments": [
            {
                "attachment_id": "att-echo",
                "label": "Transthoracic Echo",
                "media_type": "text/plain",
                "state": {"state": "readable", "text": att_echo}
            },
            {
                "attachment_id": "att-ecg-corrupt",
                "label": "12-Lead ECG Strip",
                "media_type": "application/pdf",
                "state": {"state": "unreadable", "reason": "corrupt PDF cross-reference table"}
            },
            {
                "attachment_id": "att-prior-labs",
                "label": "Prior Outside Chemistry",
                "state": {"state": "missing", "detail": "attachment omitted by outside sender"}
            },
            {
                "attachment_id": "att-holter-remote",
                "label": "Archived Holter Strip",
                "media_type": "application/dicom",
                "state": {"state": "unavailable", "reason": "external archive offline"}
            },
            {
                "attachment_id": "att-plan",
                "label": "Discharge Plan",
                "media_type": "text/plain",
                "state": {"state": "readable", "text": att_plan}
            }
        ],
        "scans": [{
            "scan_id": "fax-outside-labs",
            "label": "Outside Lab Fax",
            "content": {
                "status": "pages",
                "pages": [
                    {"page": 1, "state": {"state": "readable", "text": scan_page_1}},
                    {"page": 2, "state": {"state": "unreadable", "reason": "smudged thermal scan page"}},
                    {"page": 3, "state": {"state": "readable", "text": scan_page_3}}
                ]
            }
        }]
    });

    // 1. Whole-document continuation through ChartRequest (READ_TOOL) at tight max_result_bytes = 4096.
    let mut reference = json!({
        "candidate_id": "pkt-cardiorenal",
        "revision": "rev-4",
        "cursor": null,
        "source_id": "ehr-main",
        "location": {"kind": "whole"}
    });
    let mut reassembled_by_ordinal: BTreeMap<usize, (usize, String)> = BTreeMap::new();
    let mut non_readable_receipts: Vec<(usize, String, serde_json::Value, String)> = Vec::new();
    let mut steps = 0usize;

    loop {
        steps += 1;
        assert!(
            steps <= 60,
            "ChartRequest continuation must make strict progress"
        );
        let args = json!({
            "references": [reference],
            "max_result_bytes": 4096
        });
        assert!(
            matches_json_schema(&chat_schema, &args),
            "every continuation request (including returned next) must match read_tool schema: {args}"
        );
        let req = ChartRequest::parse(READ_TOOL, &args, ChartScope::Chat).unwrap();
        let out = req
            .run("pw-chart-cont", 7, std::slice::from_ref(&packet))
            .unwrap();
        let raw = out.text().unwrap();
        assert!(
            raw.len() <= 4096,
            "encoded result {} exceeded max_result_bytes 4096",
            raw.len()
        );
        let parsed: serde_json::Value = serde_json::from_str(raw).unwrap();
        let item = &parsed["results"][0];
        let outcome = item["outcome"].as_str().unwrap();
        let ordinal = item["part"].as_u64().unwrap() as usize;
        match outcome {
            "passage" => {
                let total_bytes = item["total_bytes"].as_u64().unwrap() as usize;
                let start_byte = item["start_byte"].as_u64().unwrap() as usize;
                let end_byte = item["end_byte"].as_u64().unwrap() as usize;
                let text = item["content"].as_str().unwrap();
                let entry = reassembled_by_ordinal
                    .entry(ordinal)
                    .or_insert_with(|| (total_bytes, String::new()));
                assert_eq!(entry.0, total_bytes);
                assert_eq!(
                    start_byte,
                    entry.1.len(),
                    "ChartRequest passages for part {ordinal} must be contiguous with nothing skipped"
                );
                entry.1.push_str(text);
                assert_eq!(end_byte, entry.1.len());
            }
            "missing_part" => {
                non_readable_receipts.push((
                    ordinal,
                    outcome.to_owned(),
                    item["part_identity"].clone(),
                    item["detail"].as_str().unwrap().to_owned(),
                ));
            }
            "unreadable_part" | "unavailable_part" => {
                non_readable_receipts.push((
                    ordinal,
                    outcome.to_owned(),
                    item["part_identity"].clone(),
                    item["reason"].as_str().unwrap().to_owned(),
                ));
            }
            other => panic!("unexpected outcome in multi-part continuation: {other} ({item})"),
        }
        reference = item["next"].clone();
        if reference.is_null() {
            break;
        }
        assert_eq!(
            reference["source_id"], "ehr-main",
            "next reference must preserve caller source_id"
        );
    }

    // Verify ALL 6 readable parts (ordinals 0, 1, 2, 6, 7, 9) reassembled byte-for-byte with nothing dropped!
    assert_eq!(reassembled_by_ordinal.len(), 6);
    for (ordinal, (total_bytes, text)) in &reassembled_by_ordinal {
        assert_eq!(
            text.len(),
            *total_bytes,
            "part {ordinal} must finish at exact total_bytes"
        );
    }
    assert_eq!(reassembled_by_ordinal[&0].1, native);
    assert_eq!(reassembled_by_ordinal[&1].1, retained_text);
    assert_eq!(reassembled_by_ordinal[&2].1, att_echo);
    assert_eq!(reassembled_by_ordinal[&6].1, att_plan);
    assert_eq!(reassembled_by_ordinal[&7].1, scan_page_1);
    assert_eq!(reassembled_by_ordinal[&9].1, scan_page_3);

    // Verify ALL 4 non-readable parts (ordinals 3, 4, 5, 8) surfaced distinctly and named each failed part!
    assert_eq!(
        non_readable_receipts,
        vec![
            (
                3,
                "unreadable_part".into(),
                json!({
                    "kind": "attachment",
                    "attachment_id": "att-ecg-corrupt",
                    "label": "12-Lead ECG Strip",
                    "media_type": "application/pdf"
                }),
                "corrupt PDF cross-reference table".into(),
            ),
            (
                4,
                "missing_part".into(),
                json!({
                    "kind": "attachment",
                    "attachment_id": "att-prior-labs",
                    "label": "Prior Outside Chemistry"
                }),
                "attachment omitted by outside sender".into(),
            ),
            (
                5,
                "unavailable_part".into(),
                json!({
                    "kind": "attachment",
                    "attachment_id": "att-holter-remote",
                    "label": "Archived Holter Strip",
                    "media_type": "application/dicom"
                }),
                "external archive offline".into(),
            ),
            (
                8,
                "unreadable_part".into(),
                json!({
                    "kind": "scan_page",
                    "scan_id": "fax-outside-labs",
                    "label": "Outside Lab Fax",
                    "page": 2,
                    "total_pages": 3
                }),
                "smudged thermal scan page".into(),
            ),
        ]
    );

    // 2. Direct ScanPage reads on envelope-trimmed Page 1 (~3,960B) and multi-envelope Page 3 (~9,200B)
    // must resume cleanly inside that page without PartNotFound fallback or truncation.
    for (target_page, expected_ordinal, expected_text, min_steps) in [
        (1u32, 7usize, &scan_page_1, 2usize),
        (3u32, 9usize, &scan_page_3, 3usize),
    ] {
        let mut page_ref = json!({
            "candidate_id": "pkt-cardiorenal",
            "revision": "rev-4",
            "cursor": null,
            "location": {
                "kind": "scan_page",
                "scan": {"scope": "named", "scan_id": "fax-outside-labs"},
                "page": target_page
            }
        });
        let mut page_text = String::new();
        let mut page_steps = 0usize;
        loop {
            page_steps += 1;
            assert!(page_steps <= 20);
            let args = json!({
                "references": [page_ref],
                "max_result_bytes": 4096
            });
            assert!(matches_json_schema(&chat_schema, &args));
            let req = ChartRequest::parse(READ_TOOL, &args, ChartScope::Chat).unwrap();
            let out = req
                .run("pw-chart-cont", 7, std::slice::from_ref(&packet))
                .unwrap();
            let raw = out.text().unwrap();
            assert!(raw.len() <= 4096);
            let parsed: serde_json::Value = serde_json::from_str(raw).unwrap();
            let item = &parsed["results"][0];
            assert_eq!(item["outcome"], "passage");
            assert_eq!(item["part"], expected_ordinal);
            assert!(
                item.get("location_limit").is_none(),
                "trimmed ScanPage continuation must recover directly without fallback: {item}"
            );
            assert_eq!(
                item["start_byte"].as_u64().unwrap() as usize,
                page_text.len()
            );
            page_text.push_str(item["content"].as_str().unwrap());
            assert_eq!(item["end_byte"].as_u64().unwrap() as usize, page_text.len());
            page_ref = item["next"].clone();
            if page_ref.is_null() {
                break;
            }
        }
        assert!(
            page_steps >= min_steps,
            "ScanPage {target_page} expected >= {min_steps} envelope steps, took {page_steps}"
        );
        assert_eq!(&page_text, expected_text);
    }
}
