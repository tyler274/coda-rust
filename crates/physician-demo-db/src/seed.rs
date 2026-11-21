//! Build-time seeder converting `patients/units/*/ledger.json` fixtures into `demo.sqlite`.

use crate::driver::DbError;
use crate::schema::{
    Briefings, Chats, FollowUps, PatientMemory, PatientReviews, Patients, PracticeVisits,
    SourceRecords, SpruceMatchQueue, TodayItems, UpdatesFeed,
};
use crate::spruce::{StoredMatchItem, StoredSpruceReply};
use crate::store::{
    DEMO_DATE, DEMO_NOW, DEMO_ZONE, DemoSourceRecord, DemoStore, build_cited_source,
    build_exchange_with_segments, build_item_citation, synthesize_chart_answer_with_citations,
};
use physician_wire::briefings::{
    BriefingClaimWire, BriefingDecisionWire, BriefingSourceStatusWire, BriefingVisitStandingWire,
    VisitBriefingWire,
};
use physician_wire::follow_ups::{FollowUpOriginWire, FollowUpStandingWire, FollowUpWire};
use physician_wire::matching_reply::{
    CardChannelBannerWire, CardReviewStateWire, ChannelSenderRoleWire, DisplayedReplyTextWire,
    FilteredMessageWire, MatchSuggestionWire, NonPatientCategoryWire, PatientReplyConversationWire,
    ReplyCardWire, ReplyPatientMessageWire, UnmatchedMessageWire, UnmatchedUrgencyWire,
    UnreadableSpruceMessageWire,
};
use physician_wire::reviews::{
    PatientReviewOutcomeWire, PatientReviewStateWire, PatientReviewWire, ReviewActionWire,
    ReviewEvidenceRefWire, ReviewHoldReasonWire, ReviewStepStateWire, ReviewStepWire,
    ReviewTriggerWire, ReviewVersion,
};
use physician_wire::source_setup::{
    PracticeSourceWire, SourceConnectionMethodWire, SourceStandingWire, SpruceBacklogWire,
    SupportedSourceWire,
};
use physician_wire::today_updates::{SeenStandingWire, UpdateItemKindWire, UpdateItemWire};
use physician_wire::{
    AutomationRun, ChatSnapshot, CitationDestination, CitationTargetKind, ClinicalSegment,
    ConnectedSource, ConnectionStatusWire, FollowUp, MedicalRecordNumber, MemoryDocumentWire,
    MessageSegment, PatientIdentity, PatientLinkWire, PatientMemoryResponse, PatientSynopsis,
    PatientWorkspaceResponse, RecordGroup, SexOrGender, SnapshotFeedback, SourceConnectionState,
    SourcePresentation, SourcePreviewContent, SourceSnapshot, WorkPurpose,
    WorkspaceMedicationRecommendation, WorkspaceMembership, WorkspacePlanItem, WorkspaceReply,
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const PRIORITY_UNITS: [&str; 3] = ["unit-2013", "unit-2188", "unit-2797"];

/// Errors produced while discovering or reading unit fixtures.
#[derive(Debug, thiserror::Error)]
pub enum SeedError {
    /// Underlying SQLite error.
    #[error("demo SQLite error")]
    Db(#[from] DbError),
    /// Filesystem read failure.
    #[error("failed reading unit fixture")]
    Io(#[from] std::io::Error),
    /// JSON parse failure for a unit ledger.
    #[error("failed parsing unit ledger JSON")]
    Json(#[from] serde_json::Error),
}

type ExtractedRecord = (String, &'static str, String, String, String);

/// Seeds `store` from `units_dir`, prioritizing pilot units (`unit-2013`, `unit-2188`, `unit-2797`).
pub fn seed_from_units_dir(
    store: &DemoStore,
    units_dir: &Path,
    limit: usize,
) -> Result<usize, SeedError> {
    store.init_schema()?;
    seed_practice_connections_and_matching(store)?;
    let mut imported = 0usize;
    for (idx, unit_dir) in collect_unit_dirs(units_dir, limit)?.iter().enumerate() {
        let unit_id = unit_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unit-unknown");
        let ledger_path = unit_dir.join("ledger.json");
        if !ledger_path.is_file() {
            continue;
        }
        let ledger: Value = serde_json::from_str(&fs::read_to_string(&ledger_path)?)?;
        if let Some(patient) = jarr(&ledger, "patients").first() {
            insert_patient_unit(store, idx, unit_id, patient)?;
            imported += 1;
        }
    }
    Ok(imported)
}

fn collect_unit_dirs(units_dir: &Path, limit: usize) -> Result<Vec<PathBuf>, SeedError> {
    let mut all: Vec<PathBuf> = fs::read_dir(units_dir)?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    all.sort();
    let mut ordered = Vec::new();
    let mut seen = BTreeSet::new();
    for c in PRIORITY_UNITS.iter().map(|p| units_dir.join(p)).chain(all) {
        if limit > 0 && ordered.len() >= limit {
            break;
        }
        if c.is_dir() && seen.insert(c.clone()) {
            ordered.push(c);
        }
    }
    Ok(ordered)
}

fn insert_patient_unit(
    store: &DemoStore,
    idx: usize,
    ws_id: &str,
    patient: &Value,
) -> Result<(), SeedError> {
    let (given, family, mrn, birth_date, summary) = (
        jstr(patient, "given"),
        jstr(patient, "family"),
        jstr(patient, "mrn"),
        jstr(patient, "birth_date"),
        jstr(patient, "summary"),
    );
    let display_name = format!("{given} {family}");
    let records = extract_source_records(ws_id, patient);
    for (id, group, title, dt, body) in &records {
        store.driver().insert_row(
            SourceRecords::Table,
            SourceRecords::COLS,
            [
                id.as_str().into(),
                ws_id.into(),
                (*group).into(),
                title.as_str().into(),
                dt.as_str().into(),
                body.as_str().into(),
                "Practice EHR".into(),
            ],
        )?;
    }
    let (headline, replies) = if idx < 6 {
        let (h, r) = seed_featured(store, idx, ws_id, &display_name, given, summary, &records)?;
        (Some(h), r)
    } else {
        if idx < 8 {
            seed_quiet_review_update(store, ws_id, &display_name, summary)?;
        }
        (None, Vec::new())
    };
    let active_follow_ups = if idx < 4 {
        let fu = seed_follow_up_row(store, idx, ws_id)?;
        vec![fu]
    } else {
        Vec::new()
    };
    let sex = infer_sex_label(summary);
    let year = birth_date
        .split('-')
        .next()
        .and_then(|y| y.parse::<i64>().ok())
        .unwrap_or(1970);
    let workspace = PatientWorkspaceResponse {
        patient_workspace_id: ws_id.into(),
        display_name: display_name.clone(),
        membership: WorkspaceMembership::Allowed,
        time_zone: Some(DEMO_ZONE.into()),
        chats: Vec::new(),
        identity: Some(PatientIdentity {
            display_name: Some(display_name.clone()),
            birth_date: Some(birth_date.into()),
            age_years: Some((2026 - year).max(1)),
            sex_or_gender: Some(SexOrGender {
                value: Some(sex.to_lowercase()),
                label: Some(sex.into()),
            }),
            medical_record_numbers: vec![MedicalRecordNumber {
                value: Some(mrn.into()),
            }],
            email: None,
            state: Some("available".into()),
        }),
        patient_synopsis: Some(PatientSynopsis {
            version_id: format!("syn-{ws_id}-v1"),
            authored_text: None,
            segments: build_synopsis(ws_id, summary, &records),
            authored_at: Some(DEMO_NOW.into()),
        }),
        automation_run: headline.map(|h| AutomationRun {
            kind: "visit_preparation".into(),
            state: "finished".into(),
            ended_cause: None,
            purpose: Some(WorkPurpose {
                headline: Some(h.into()),
            }),
        }),
        active_follow_ups,
        has_past_work: idx < 6,
        last_reviewed_at: Some(DEMO_NOW.into()),
        connected_sources: vec![ConnectedSource {
            source_id: Some("synthetic-ehr".into()),
            source_label: Some("Primary EHR".into()),
            state: SourceConnectionState {
                state: "usable".into(),
                synchronized_at: Some(DEMO_NOW.into()),
                usable_snapshot: Some(SourceSnapshot {
                    synchronized_at: Some(DEMO_NOW.into()),
                }),
            },
        }],
        replies,
        superseded_replies: Vec::new(),
        active_plan_items: build_initial_plan(ws_id, idx),
        undecided_recommendations: build_recommendations(ws_id, idx),
        connection_status: if idx < 12 {
            ConnectionStatusWire::Connected
        } else {
            ConnectionStatusWire::NotInvited
        },
        patient_link: Some(PatientLinkWire {
            practice_id: "prac-coda-demo".into(),
            patient_workspace_id: ws_id.into(),
            duet_account_id: Some(format!("duet-{ws_id}")),
            linked: idx < 12,
            sharing_consented: idx < 10,
            connection_status: if idx < 12 {
                ConnectionStatusWire::Connected
            } else {
                ConnectionStatusWire::NotInvited
            },
            consent_version: Some("v1".into()),
            consented_at: Some(1_785_000_000),
            revoked_at: None,
            invited_email: None,
            wrong_dob_attempts: 0,
            locked_at: None,
            delivery_failed_at: None,
            delivery_error: None,
        }),
    };
    let json = serde_json::to_string(&workspace)?;
    store.driver().insert_row(
        Patients::Table,
        Patients::COLS,
        [
            ws_id.into(),
            display_name.into(),
            mrn.into(),
            birth_date.into(),
            json.into(),
        ],
    )?;
    Ok(())
}

fn jarr<'a>(val: &'a Value, key: &str) -> &'a [Value] {
    val.get(key)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[])
}
fn jstr<'a>(val: &'a Value, key: &str) -> &'a str {
    val.get(key).and_then(Value::as_str).unwrap_or("")
}
fn jnonempty<'a>(val: &'a Value, key: &str, fb: &'a str) -> &'a str {
    match jstr(val, key).trim() {
        "" => fb,
        s => s,
    }
}
fn date_prefix(raw: Option<&str>, fb: &str) -> String {
    raw.and_then(|s| s.get(..10))
        .filter(|s| !s.is_empty())
        .unwrap_or(fb)
        .into()
}

fn extract_source_records(ws_id: &str, patient: &Value) -> Vec<ExtractedRecord> {
    let mut out = Vec::new();
    for (ei, ev) in jarr(patient, "events").iter().enumerate() {
        let dt = date_prefix(ev.get("start").and_then(Value::as_str), "2024-01-01");
        let reason = jnonempty(ev, "reason", "Clinical contact");
        for (di, d) in jarr(ev, "documents").iter().enumerate() {
            let body = jstr(d, "body").trim();
            if body.is_empty() {
                continue;
            }
            let kind = jnonempty(d, "type", "Clinical Note");
            let grp = if kind.to_lowercase().contains("outside") {
                "documents"
            } else {
                "notes"
            };
            out.push((
                format!("{ws_id}-doc-{ei}-{di}"),
                grp,
                jnonempty(d, "title", kind).into(),
                date_prefix(d.get("date").and_then(Value::as_str), &dt),
                body.into(),
            ));
        }
        let conds = jarr(ev, "conditions");
        if !conds.is_empty() {
            let body = conds
                .iter()
                .map(|c| {
                    format!(
                        "{} ({})",
                        jstr(c, "label"),
                        jnonempty(c, "action", "recorded")
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            let onset = conds
                .first()
                .and_then(|c| c.get("onset").and_then(Value::as_str))
                .unwrap_or(&dt);
            out.push((
                format!("{ws_id}-prob-{ei}"),
                "problems",
                format!("Diagnoses — {reason}"),
                onset.into(),
                body,
            ));
        }
        let meds = jarr(ev, "medications");
        if !meds.is_empty() {
            let body = meds
                .iter()
                .map(|m| {
                    format!(
                        "{} [{}]: {} ({})",
                        jnonempty(m, "drug", jstr(m, "drug_text")),
                        jstr(m, "action"),
                        jstr(m, "sig"),
                        jstr(m, "reason")
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            out.push((
                format!("{ws_id}-med-{ei}"),
                "medications",
                format!("Medications ({})", jstr(ev, "id")),
                dt.clone(),
                body,
            ));
        }
        for (li, lab) in jarr(ev, "labs").iter().enumerate() {
            let body = jarr(lab, "results")
                .iter()
                .map(|r| {
                    format!(
                        "{}: {}",
                        jstr(r, "test"),
                        r.get("value").unwrap_or(&Value::Null)
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            out.push((
                format!("{ws_id}-lab-{ei}-{li}"),
                "results",
                jnonempty(lab, "panel", "Laboratory results").into(),
                date_prefix(lab.get("collected").and_then(Value::as_str), &dt),
                body,
            ));
        }
        for (ri, r) in jarr(ev, "reports").iter().enumerate() {
            let (text, conc) = (jstr(r, "text").trim(), jstr(r, "conclusion").trim());
            if text.is_empty() && conc.is_empty() {
                continue;
            }
            let body = if conc.is_empty() {
                text.into()
            } else {
                format!("{text}\n\nConclusion: {conc}")
            };
            out.push((
                format!("{ws_id}-rep-{ei}-{ri}"),
                "results",
                jnonempty(r, "study", "Diagnostic report").into(),
                date_prefix(r.get("performed").and_then(Value::as_str), &dt),
                body,
            ));
        }
        let vit = ev.get("vitals").unwrap_or(&Value::Null);
        let v_obj = match vit {
            Value::Array(a) => a.first(),
            Value::Object(_) => Some(vit),
            _ => None,
        };
        if let Some(v) = v_obj
            && let Some(bp) = v.get("bp").and_then(Value::as_array)
            && bp.len() >= 2
        {
            let hr = v
                .get("hr")
                .and_then(Value::as_i64)
                .map(|n| format!(", HR {n}"))
                .unwrap_or_default();
            out.push((
                format!("{ws_id}-vit-{ei}"),
                "vitals",
                format!("Vitals ({reason})"),
                dt,
                format!("BP {}/{}{}", bp[0], bp[1], hr),
            ));
        }
    }
    out
}

fn build_synopsis(ws_id: &str, summary: &str, records: &[ExtractedRecord]) -> Vec<ClinicalSegment> {
    let mut segs = vec![ClinicalSegment::Text {
        text: format!("{} ", summary.trim()),
    }];
    for (id, _, title, dt, body) in records.iter().rev().take(2) {
        segs.push(ClinicalSegment::Citation {
            state: "resolved".into(),
            display_label: Some(title.clone()),
            source: Some("ehr".into()),
            source_id: Some(ws_id.into()),
            handle: Some(id.clone()),
            source_presentation: Some(SourcePresentation {
                content: Some(SourcePreviewContent {
                    text: body.clone(),
                    as_of: dt.clone(),
                }),
            }),
            destination: Some(CitationDestination {
                kind: "source_native".into(),
                source_label: Some("Practice EHR".into()),
                uri: None,
            }),
        });
    }
    segs
}

fn build_initial_plan(ws_id: &str, idx: usize) -> Vec<WorkspacePlanItem> {
    const PLANS: [(&str, &str); 3] = [
        (
            "Metformin 1000 mg twice daily",
            "Continue current glycemic therapy; review A1c and renal function at upcoming visit.",
        ),
        (
            "Abdominal aortic aneurysm surveillance",
            "Confirm outside vascular ultrasound timing and smoking cessation counseling.",
        ),
        (
            "Home blood pressure log review",
            "Track twice-daily cuff readings on current losartan regimen.",
        ),
    ];
    PLANS
        .get(idx)
        .map(|(t, d)| {
            vec![WorkspacePlanItem {
                item_id: format!("plan-seed-{idx}"),
                patient_workspace_id: ws_id.into(),
                title: (*t).into(),
                detail: (*d).into(),
                decided_at: DEMO_NOW.into(),
                source_recommendation_id: None,
            }]
        })
        .unwrap_or_default()
}

fn build_recommendations(ws_id: &str, idx: usize) -> Vec<WorkspaceMedicationRecommendation> {
    const RECS: [(&str, &str, &str, &str); 2] = [
        (
            "statin",
            "High-intensity statin review (atorvastatin 40 mg)",
            "LDL remains 130 mg/dL on atorvastatin 20 mg in the setting of type 2 diabetes and PAD.",
            "Lipid panel & chart review",
        ),
        (
            "copd",
            "Step-up COPD inhaler therapy (LAMA/LABA)",
            "Worsening exertional dyspnea on olodaterol monotherapy across recent outside notes.",
            "Pulmonary clinic fax",
        ),
    ];
    RECS.get(idx)
        .map(|(slug, m, r, s)| {
            vec![WorkspaceMedicationRecommendation {
                recommendation_id: format!("rec-{ws_id}-{slug}"),
                patient_workspace_id: ws_id.into(),
                medication: (*m).into(),
                rationale: (*r).into(),
                source_label: (*s).into(),
                decision_state: "undecided".into(),
            }]
        })
        .unwrap_or_default()
}

fn to_demo_record(ws_id: &str, rec: &ExtractedRecord) -> DemoSourceRecord {
    let (id, group, title, dt, body) = rec;
    DemoSourceRecord {
        id: id.clone(),
        workspace_id: ws_id.to_owned(),
        group: (*group).to_owned(),
        title: title.clone(),
        recorded_at: dt.clone(),
        body: body.clone(),
        source_label: "Practice EHR".to_owned(),
    }
}

fn seed_practice_connections_and_matching(store: &DemoStore) -> Result<(), SeedError> {
    let sources = [
        PracticeSourceWire {
            source_id: SupportedSourceWire::Hint,
            label: SupportedSourceWire::Hint.label().into(),
            summary: "Direct Primary Care membership roster, notes, and demographics.".into(),
            guidance: "Connected via demo practice walkthrough.".into(),
            prerequisites: vec!["Active Hint practice administrator session".into()],
            supported_methods: vec![
                SourceConnectionMethodWire::BrowserSession,
                SourceConnectionMethodWire::SimulatedPractice,
            ],
            standing: SourceStandingWire::UpToDate {
                connected_at: "2026-08-20T14:00:00Z".into(),
                method: SourceConnectionMethodWire::SimulatedPractice,
                account_label: "Coda Primary Care Demo".into(),
                imported_records: 48,
                last_read_at: DEMO_NOW.into(),
            },
            backlog: SpruceBacklogWire::NotApplicable,
            unlinked_spruce_messages: 0,
            retained_linked_records: 48,
            retained_reply_drafts: 0,
        },
        PracticeSourceWire {
            source_id: SupportedSourceWire::ElationFhir,
            label: SupportedSourceWire::ElationFhir.label().into(),
            summary: "Longitudinal problems, medications, labs, vitals, and outside PDFs.".into(),
            guidance: "FHIR R4 clinical pull active in demo SQLite.".into(),
            prerequisites: vec!["Practice FHIR client credentials".into()],
            supported_methods: vec![
                SourceConnectionMethodWire::CredentialSignIn,
                SourceConnectionMethodWire::SimulatedPractice,
            ],
            standing: SourceStandingWire::UpToDate {
                connected_at: "2026-08-20T14:05:00Z".into(),
                method: SourceConnectionMethodWire::SimulatedPractice,
                account_label: "Elation FHIR Demo".into(),
                imported_records: 64,
                last_read_at: DEMO_NOW.into(),
            },
            backlog: SpruceBacklogWire::NotApplicable,
            unlinked_spruce_messages: 0,
            retained_linked_records: 64,
            retained_reply_drafts: 0,
        },
        PracticeSourceWire {
            source_id: SupportedSourceWire::Spruce,
            label: SupportedSourceWire::Spruce.label().into(),
            summary: "Inbound patient SMS, portal threads, attachments, and clinic faxes.".into(),
            guidance: "Spruce channel active for matched replies and triage.".into(),
            prerequisites: vec!["Practice Spruce organization webhook & API key".into()],
            supported_methods: vec![
                SourceConnectionMethodWire::CredentialSignIn,
                SourceConnectionMethodWire::SimulatedPractice,
            ],
            standing: SourceStandingWire::UpToDate {
                connected_at: "2026-08-20T14:10:00Z".into(),
                method: SourceConnectionMethodWire::SimulatedPractice,
                account_label: "Spruce Clinic Inbox".into(),
                imported_records: 14,
                last_read_at: DEMO_NOW.into(),
            },
            backlog: SpruceBacklogWire::NonePending,
            unlinked_spruce_messages: 1,
            retained_linked_records: 14,
            retained_reply_drafts: 1,
        },
    ];
    for source in &sources {
        store.save_source(source)?;
    }

    let unmatched = UnmatchedMessageWire {
        message_id: "msg-match-1".into(),
        conversation_id: "conv-unmatched-1".into(),
        sender_display_name: "Jordan Kowalczyk (Caregiver)".into(),
        sender_address: Some("+14155550192".into()),
        body_text:
            "Checking if Gary needs to hold evening metformin before tomorrow's fasting lipid lab?"
                .into(),
        attachments: Vec::new(),
        spruce_app_url: Some("https://app.sprucehealth.com/conversations/conv-unmatched-1".into()),
        waiting_since: "2026-08-27T18:10:00.000Z".into(),
        urgency: UnmatchedUrgencyWire::Ordinary,
        suggestion: MatchSuggestionWire::SinglePatient {
            patient_workspace_id: "unit-2013".into(),
            patient_display_name: "Gary Kowalczyk".into(),
            birth_date: None,
            mrn: None,
            evidence_summary:
                "Caregiver shared last name Kowalczyk and referenced tomorrow's lipid lab.".into(),
        },
    };
    let filtered = FilteredMessageWire {
        message_id: "msg-filt-1".into(),
        conversation_id: "conv-filt-1".into(),
        sender_display_name: "Summit Medical Supply".into(),
        sender_address: Some("billing@summitmedsupply.example".into()),
        body_text: "Quarterly DME catalog pricing update for clinic administrators.".into(),
        spruce_app_url: None,
        category: NonPatientCategoryWire::VendorOrSpam,
        rationale: "Commercial vendor outreach with no patient clinical inquiry.".into(),
        filtered_at: "2026-08-27T16:40:00.000Z".into(),
    };
    let unreadable = UnreadableSpruceMessageWire {
        message_id: "msg-unrd-1".into(),
        conversation_id: "conv-unrd-1".into(),
        failure_reason: "Encrypted multi-page fax PDF could not be decoded locally.".into(),
        spruce_app_url: Some("https://app.sprucehealth.com/conversations/conv-unrd-1".into()),
        kept_at: "2026-08-27T15:20:00.000Z".into(),
    };
    for (id, conv, bucket, ts, item) in [
        (
            "msg-match-1",
            "conv-unmatched-1",
            "unmatched",
            "2026-08-27T18:10:00.000Z",
            StoredMatchItem::Unmatched(unmatched),
        ),
        (
            "msg-filt-1",
            "conv-filt-1",
            "filtered",
            "2026-08-27T16:40:00.000Z",
            StoredMatchItem::Filtered(filtered),
        ),
        (
            "msg-unrd-1",
            "conv-unrd-1",
            "unreadable",
            "2026-08-27T15:20:00.000Z",
            StoredMatchItem::Unreadable(unreadable),
        ),
    ] {
        store.driver().insert_row(
            SpruceMatchQueue::Table,
            SpruceMatchQueue::COLS,
            [
                id.into(),
                conv.into(),
                bucket.into(),
                ts.into(),
                serde_json::to_string(&item)?.into(),
            ],
        )?;
    }
    Ok(())
}

fn seed_quiet_review_update(
    store: &DemoStore,
    ws_id: &str,
    display_name: &str,
    summary: &str,
) -> Result<(), SeedError> {
    let item = UpdateItemWire {
        item_id: format!("review:quiet-{ws_id}"),
        version_id: "1".into(),
        patient_workspace_id: ws_id.into(),
        patient_display_name: display_name.into(),
        kind: UpdateItemKindWire::QuietReview,
        headline: "Periodic chart review — no new physician action needed".into(),
        detail: truncate_detail(summary),
        occurred_at: "2026-08-27T13:30:00.000Z".into(),
        standing: SeenStandingWire::Unseen,
    };
    store.driver().insert_row(
        UpdatesFeed::Table,
        UpdatesFeed::COLS,
        [
            item.item_id.as_str().into(),
            ws_id.into(),
            "quiet".into(),
            item.occurred_at.as_str().into(),
            serde_json::to_string(&item)?.into(),
        ],
    )?;
    Ok(())
}

fn seed_follow_up_row(store: &DemoStore, idx: usize, ws_id: &str) -> Result<FollowUp, SeedError> {
    let summary = match idx {
        0 => "Recheck fasting lipid panel and BMP 4 weeks after statin review.",
        1 => "Confirm inhaler spacer technique and pulse oximetry at follow-up.",
        _ => "Review interval labs and home blood pressure log before next visit.",
    };
    let fu = FollowUpWire {
        follow_up_id: format!("fu-{ws_id}"),
        patient_workspace_id: ws_id.into(),
        version: 1,
        summary: summary.into(),
        standing: FollowUpStandingWire::Scheduled {
            due_at: "2026-09-03T13:00:00Z".into(),
            due_date_input: "2026-09-03".into(),
        },
        origin: FollowUpOriginWire::CodaCreated {
            run_id: format!("run-{ws_id}-1"),
        },
        latest_note: Some("Scheduled during pre-visit preparation.".into()),
        updated_at: DEMO_NOW.into(),
    };
    store.driver().insert_row(
        FollowUps::Table,
        FollowUps::COLS,
        [
            fu.follow_up_id.as_str().into(),
            ws_id.into(),
            fu.standing.badge_label().into(),
            fu.updated_at.as_str().into(),
            serde_json::to_string(&fu)?.into(),
        ],
    )?;
    Ok(FollowUp {
        follow_up_id: fu.follow_up_id,
        description: Some(fu.summary),
    })
}

fn seed_featured(
    store: &DemoStore,
    idx: usize,
    ws_id: &str,
    display_name: &str,
    given: &str,
    summary: &str,
    records: &[ExtractedRecord],
) -> Result<(&'static str, Vec<WorkspaceReply>), SeedError> {
    const ITEMS: [(&str, &str, &str, &str); 4] = [
        (
            "now",
            "concern",
            "LDL 130 on atorvastatin 20 mg with PAD & diabetes",
            "2026-08-27T17:15:00.000Z",
        ),
        (
            "now",
            "drafted_reply",
            "Patient messaged about exertional shortness of breath",
            "2026-08-27T17:45:00.000Z",
        ),
        (
            "today",
            "concern",
            "Home BP readings trending 142/88 ahead of follow-up",
            "2026-08-27T15:30:00.000Z",
        ),
        (
            "today",
            "preparation",
            "Visit preparation summary ready",
            "2026-08-27T14:00:00.000Z",
        ),
    ];
    let (urgency, kind, reason, waited) = ITEMS[idx.min(3)];
    let is_spruce_row = idx == 1;
    let conv_id = format!("conv-{ws_id}-1");
    let item_id = if is_spruce_row {
        format!("spruce-reply:{conv_id}")
    } else {
        format!("today-{ws_id}")
    };
    let chat_id = format!("chat-{ws_id}-1");
    let row_json = json!({
        "patient_workspace_id": ws_id,
        "patient_display_name": display_name,
        "item_id": item_id,
        "version_id": format!("ver-{ws_id}-1"),
        "waiting_since": waited,
        "urgency": urgency,
        "kind": kind,
        "title": reason,
        "reason": reason,
        "can_mark_done": !is_spruce_row,
        "linked_chat_id": chat_id,
        "spruce_conversation_id": if is_spruce_row { json!(conv_id) } else { Value::Null },
    });
    store.driver().insert_row(
        TodayItems::Table,
        TodayItems::COLS,
        [
            item_id.into(),
            ws_id.into(),
            urgency.into(),
            waited.into(),
            kind.into(),
            row_json.to_string().into(),
            0.into(),
            0.into(),
        ],
    )?;

    let (hr, min) = (19 + (idx % 4), if idx >= 4 { 45 } else { 30 });
    let starts = format!("2026-08-27T{hr:02}:{min:02}:00.000Z");
    let ends = format!("2026-08-27T{hr:02}:55:00.000Z");
    let briefing_id = format!("briefing-{ws_id}");
    let visit_label = if idx == 0 {
        "Cardiometabolic follow-up"
    } else {
        "Primary care follow-up"
    };
    let visit_json = json!({
        "patient_display_name": display_name,
        "patient_workspace_id": ws_id,
        "label": visit_label,
        "starts_at": starts,
        "ends_at": ends,
        "status": "scheduled",
        "patient_time_zone": DEMO_ZONE,
        "preparation": {
            "state": "prepared",
            "briefing_id": briefing_id,
            "last_attempt": { "outcome": "completed" }
        }
    });
    store.driver().insert_row(
        PracticeVisits::Table,
        PracticeVisits::COLS,
        [
            format!("visit-{ws_id}").into(),
            starts.as_str().into(),
            visit_json.to_string().into(),
        ],
    )?;

    let demo_records: Vec<DemoSourceRecord> =
        records.iter().map(|r| to_demo_record(ws_id, r)).collect();
    seed_visit_briefing(
        store,
        ws_id,
        display_name,
        &briefing_id,
        visit_label,
        &starts,
        &ends,
        summary,
        &demo_records,
    )?;

    let (bucket, update_kind, update_time) = match idx {
        0 => ("today", UpdateItemKindWire::ConcernRaised, waited),
        1 => ("today", UpdateItemKindWire::AssessedReview, waited),
        2 => ("today", UpdateItemKindWire::VisitBriefing, waited),
        3 => (
            "yesterday",
            UpdateItemKindWire::AssessedReview,
            "2026-08-26T18:10:00.000Z",
        ),
        _ => ("quiet", UpdateItemKindWire::QuietReview, waited),
    };
    let update_item = UpdateItemWire {
        item_id: format!("upd-{ws_id}"),
        version_id: "1".into(),
        patient_workspace_id: ws_id.into(),
        patient_display_name: display_name.into(),
        kind: update_kind,
        headline: reason.into(),
        detail: truncate_detail(summary),
        occurred_at: update_time.into(),
        standing: SeenStandingWire::Unseen,
    };
    store.driver().insert_row(
        UpdatesFeed::Table,
        UpdatesFeed::COLS,
        [
            update_item.item_id.as_str().into(),
            ws_id.into(),
            bucket.into(),
            update_time.into(),
            serde_json::to_string(&update_item)?.into(),
        ],
    )?;

    let replies = if idx < 2 {
        let draft = if idx == 0 {
            format!(
                "Hi {given}, thank you for checking in. Before your upcoming visit, please bring your current pill bottles so we can review your cholesterol and blood pressure regimen together."
            )
        } else {
            format!(
                "Hi {given}, I reviewed your note about shortness of breath on exertion. Please let our clinic know if symptoms occur at rest, and we will review stepping up your daily inhaler."
            )
        };
        if idx == 1 {
            seed_spruce_reply_card(
                store,
                ws_id,
                display_name,
                &conv_id,
                waited,
                &draft,
                &demo_records,
            )?;
        }
        vec![WorkspaceReply {
            reply_work_id: format!("reply-{ws_id}"),
            patient_workspace_id: ws_id.into(),
            patient_display_name: display_name.into(),
            channel: if idx == 0 { "email" } else { "spruce" }.into(),
            headline: reason.into(),
            draft_text: draft,
            urgency: "routine".into(),
            created_at: waited.into(),
            state: "ready".into(),
            superseded_by_input: None,
            superseded_reason: None,
            draft_revision: Some(1),
            revision_run: None,
        }]
    } else {
        Vec::new()
    };

    if idx < 4 {
        seed_patient_reviews(store, idx, ws_id, reason, summary, &demo_records)?;
    }

    let mem = PatientMemoryResponse {
        patient_workspace_id: ws_id.into(),
        durable: true,
        summary: Some(MemoryDocumentWire {
            version: 1,
            saved_at: DEMO_DATE.into(),
            markdown: format!("# {display_name}\n\n{}", summary.trim()),
        }),
        direction: Some(MemoryDocumentWire {
            version: 1,
            saved_at: DEMO_DATE.into(),
            markdown: format!("Priority focus: {reason}"),
        }),
    };
    store.driver().insert_row(
        PatientMemory::Table,
        PatientMemory::COLS,
        [ws_id.into(), 0.into(), serde_json::to_string(&mem)?.into()],
    )?;

    let cited_recs: Vec<_> = demo_records.iter().rev().take(2).cloned().collect();
    let (reads, query, answer_text, segments, basis, sources) = if cited_recs.is_empty() {
        (
            Vec::new(),
            "",
            format!("{summary} Ask any question below to query this patient's SQLite chart."),
            vec![MessageSegment::Text {
                text: format!(
                    "{summary} Ask any question below to query this patient's SQLite chart."
                ),
            }],
            Vec::new(),
            BTreeMap::new(),
        )
    } else {
        let reads = cited_recs
            .iter()
            .map(|r| {
                (
                    r.id.clone(),
                    RecordGroup::Notes,
                    r.title.clone(),
                    r.recorded_at.clone(),
                )
            })
            .collect();
        let (_, segments, basis, sources) = synthesize_chart_answer_with_citations(
            "recent summary labs medications",
            &cited_recs,
            &format!("seed-{ws_id}"),
        );
        let latest = &cited_recs[0];
        let text = format!(
            "{summary}\n\nLatest documented record: {} ({}). Ask any question below to run live SQLite chart search across {} records.",
            latest.title,
            latest.recorded_at,
            demo_records.len()
        );
        (
            reads,
            "recent summary labs medications",
            text,
            segments,
            basis,
            sources,
        )
    };
    let snap = ChatSnapshot {
        sources,
        queued: Vec::new(),
        archived: false,
        transcript: vec![build_exchange_with_segments(
            "seed",
            ws_id,
            (
                format!("msg-seed-{ws_id}"),
                "Summarize key active risks and latest chart updates.".into(),
                1_787_850_000,
            ),
            query,
            &reads,
            records.len() > 1,
            (answer_text, segments, basis),
        )],
        feedback: SnapshotFeedback::Loaded,
    };
    store.driver().insert_row(
        Chats::Table,
        Chats::COLS,
        [
            chat_id.into(),
            ws_id.into(),
            "Pre-visit chart brief".into(),
            waited.into(),
            serde_json::to_string(&snap)?.into(),
        ],
    )?;
    Ok((reason, replies))
}

#[allow(clippy::too_many_arguments)]
fn seed_visit_briefing(
    store: &DemoStore,
    ws_id: &str,
    display_name: &str,
    briefing_id: &str,
    visit_label: &str,
    starts_at: &str,
    ends_at: &str,
    summary: &str,
    records: &[DemoSourceRecord],
) -> Result<(), SeedError> {
    let mut sources = BTreeMap::new();
    let mut cite_seg = Vec::new();
    if let Some(rec) = records.last() {
        let key = format!("b1-{ws_id}");
        let cited = build_cited_source(&key, 1, rec);
        cite_seg.push(MessageSegment::Text {
            text: format!("{} ", summary.trim()),
        });
        cite_seg.push(MessageSegment::Citation {
            source_key: key.clone(),
            number: 1,
            label: format!("{} ({})", rec.title, rec.recorded_at),
            quote: cited.quote.clone(),
            target: CitationTargetKind::WholeRecord,
        });
        sources.insert(key, cited);
    } else {
        cite_seg.push(MessageSegment::Text {
            text: summary.trim().to_owned(),
        });
    }
    let briefing = VisitBriefingWire {
        briefing_id: briefing_id.to_owned(),
        patient_workspace_id: ws_id.to_owned(),
        patient_display_name: display_name.to_owned(),
        source_id: "demo-calendar".into(),
        appointment_id: format!("visit-{ws_id}"),
        visit_label: visit_label.to_owned(),
        starts_at: starts_at.to_owned(),
        ends_at: ends_at.to_owned(),
        prepared_at: DEMO_NOW.into(),
        visit_standing: BriefingVisitStandingWire::Current,
        visit_card_orientation: truncate_detail(summary),
        source_freshness: vec![BriefingSourceStatusWire::Updated {
            source_id: "synthetic-ehr".into(),
            source_label: "Practice EHR".into(),
            last_good_at: DEMO_NOW.into(),
        }],
        limits: Vec::new(),
        trajectory: vec![BriefingClaimWire {
            segments: cite_seg.clone(),
            uncertainty: None,
        }],
        meaningful_changes: vec![BriefingClaimWire {
            segments: vec![MessageSegment::Text {
                text: "Prepared directly from the seeded chart records in browser SQLite.".into(),
            }],
            uncertainty: None,
        }],
        unresolved_issues: Vec::new(),
        visit_decisions: vec![BriefingDecisionWire {
            decision: "Confirm medication adherence and next surveillance interval".into(),
            rationale_segments: cite_seg,
            options_or_questions: vec![
                "Verify home monitoring readings and tolerance of current regimen.".into(),
            ],
            uncertainty: None,
        }],
        basis_segments: vec![MessageSegment::Text {
            text: "Prepared from saved browser SQLite records.".into(),
        }],
        sources,
    };
    store.driver().insert_row(
        Briefings::Table,
        Briefings::COLS,
        [
            briefing_id.into(),
            ws_id.into(),
            serde_json::to_string(&briefing)?.into(),
        ],
    )?;
    Ok(())
}

fn seed_spruce_reply_card(
    store: &DemoStore,
    ws_id: &str,
    display_name: &str,
    conv_id: &str,
    waited: &str,
    draft: &str,
    records: &[DemoSourceRecord],
) -> Result<(), SeedError> {
    let citations = records.iter().take(2).map(build_item_citation).collect();
    let stored = StoredSpruceReply {
        surface: PatientReplyConversationWire::OpenCard(Box::new(ReplyCardWire {
            conversation_id: conv_id.to_owned(),
            patient_workspace_id: ws_id.to_owned(),
            patient_display_name: display_name.to_owned(),
            primary_sender_name: display_name.to_owned(),
            primary_sender_role: ChannelSenderRoleWire::Patient,
            non_patient_sender_names: Vec::new(),
            spruce_app_url: Some(format!("https://app.sprucehealth.com/conversations/{conv_id}")),
            reply_id: 201,
            conversation_version: 1,
            proposal_version: 1,
            has_open_concern: true,
            patient_messages: vec![ReplyPatientMessageWire {
                message_id: format!("msg-spruce-{ws_id}-1"),
                sequence: 1,
                sender_display_name: display_name.to_owned(),
                sender_role: ChannelSenderRoleWire::Patient,
                text: "I have been more short of breath walking up the stairs this week and my rescue inhaler hasn't helped much.".into(),
                sent_at: waited.to_owned(),
                arrived_after_proposal: false,
            }],
            practice_messages: Vec::new(),
            displayed_reply: DisplayedReplyTextWire::CodaProposal {
                proposal_version: 1,
                text: draft.to_owned(),
            },
            revision_run: None,
            coda_update_offer: None,
            notices: Vec::new(),
            basis_and_limitations:
                "Grounded in outpatient pulmonary note and active inhaler list in browser SQLite."
                    .into(),
            citations,
            other_physician_copies: Vec::new(),
            copied_by_me: false,
            review_state: CardReviewStateWire::Idle,
            channel_banner: CardChannelBannerWire::Connected,
            pending_move: None,
        })),
        prior_open_card: None,
        prior_move_source_ws: None,
    };
    store.save_stored_reply(conv_id, ws_id, waited, &stored)?;
    Ok(())
}

fn seed_patient_reviews(
    store: &DemoStore,
    idx: usize,
    ws_id: &str,
    reason: &str,
    summary: &str,
    records: &[DemoSourceRecord],
) -> Result<(), SeedError> {
    let mut sources = BTreeMap::new();
    let mut evidence = Vec::new();
    if let Some(rec) = records.last() {
        let key = format!("rev-{ws_id}-s1");
        let cited = build_cited_source(&key, 1, rec);
        evidence.push(ReviewEvidenceRefWire {
            source_key: key.clone(),
            number: 1,
            candidate_id: rec.id.clone(),
            revision: "1".into(),
            source_label: rec.source_label.clone(),
            display_label: rec.title.clone(),
            as_of: Some(rec.recorded_at.clone()),
            excerpt: cited.quote.clone(),
        });
        sources.insert(key, cited);
    }
    let completed = PatientReviewWire {
        review_id: format!("rev-{ws_id}-1"),
        version: ReviewVersion("rv-1".into()),
        work_id: format!("wi-rev-{ws_id}-1"),
        triggers: vec![ReviewTriggerWire::ChangedRecords {
            workspace_version: 1,
            changed_records: vec![reason.into()],
        }],
        accepted_inputs: Vec::new(),
        state: PatientReviewStateWire::Completed {
            outcome: PatientReviewOutcomeWire::Assessed {
                summary: format!("{reason}. {}", truncate_detail(summary)),
                segments: vec![MessageSegment::Text {
                    text: format!("{reason}. {}", truncate_detail(summary)),
                }],
                basis: vec![MessageSegment::Text {
                    text: "Synthesized from chart records stored in browser SQLite.".into(),
                }],
            },
            completed_at: DEMO_NOW.into(),
        },
        steps: vec![ReviewStepWire {
            label: "Read latest clinical chart records".into(),
            detail: Some(format!("{} record(s)", records.len())),
            state: ReviewStepStateWire::Done,
        }],
        evidence,
        sources,
        actions: vec![ReviewActionWire {
            summary: format!("Prepared physician review item: {reason}"),
        }],
        unfinished_work: Vec::new(),
        queued_at: "2026-08-27T16:50:00Z".into(),
        updated_at: DEMO_NOW.into(),
    };
    store.driver().insert_row(
        PatientReviews::Table,
        PatientReviews::COLS,
        [
            completed.review_id.as_str().into(),
            ws_id.into(),
            completed.updated_at.as_str().into(),
            serde_json::to_string(&completed)?.into(),
        ],
    )?;
    if idx == 0 {
        let held = PatientReviewWire {
            review_id: format!("rev-{ws_id}-held"),
            version: ReviewVersion("rv-1".into()),
            work_id: format!("wi-rev-{ws_id}-held"),
            triggers: vec![ReviewTriggerWire::Periodic {
                workspace_version: 1,
                last_reviewed_at: Some("2026-08-20T12:00:00Z".into()),
            }],
            accepted_inputs: Vec::new(),
            state: PatientReviewStateWire::Held {
                reason: ReviewHoldReasonWire::Limit,
                held_at: "2026-08-27T18:45:00Z".into(),
                partial_text: Some(
                    "Reviewed interval metabolic panel; awaiting outside ultrasound check.".into(),
                ),
            },
            steps: vec![ReviewStepWire {
                label: "Check interval labs and vascular imaging".into(),
                detail: Some("Turn limit reached".into()),
                state: ReviewStepStateWire::Stopped,
            }],
            evidence: Vec::new(),
            sources: BTreeMap::new(),
            actions: Vec::new(),
            unfinished_work: Vec::new(),
            queued_at: "2026-08-27T18:40:00Z".into(),
            updated_at: "2026-08-27T18:45:00Z".into(),
        };
        store.driver().insert_row(
            PatientReviews::Table,
            PatientReviews::COLS,
            [
                held.review_id.as_str().into(),
                ws_id.into(),
                held.updated_at.as_str().into(),
                serde_json::to_string(&held)?.into(),
            ],
        )?;
    }
    Ok(())
}

fn truncate_detail(text: &str) -> String {
    let t = text.trim();
    if t.chars().count() <= 140 {
        t.to_owned()
    } else {
        format!("{}…", t.chars().take(139).collect::<String>())
    }
}

fn infer_sex_label(summary: &str) -> &'static str {
    let l = summary.to_ascii_lowercase();
    if l.contains(" she ") || l.contains(" her ") {
        "Female"
    } else if l.contains(" he ") || l.contains(" his ") || l.contains(" him ") {
        "Male"
    } else {
        "Unspecified"
    }
}
