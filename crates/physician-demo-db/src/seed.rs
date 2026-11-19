//! Build-time seeder converting `patients/units/*/ledger.json` fixtures into `demo.sqlite`.

use crate::driver::DbError;
use crate::schema::{Chats, Patients, PracticeVisits, SourceRecords, TodayItems, UpdatesFeed};
use crate::store::{DEMO_NOW, DEMO_ZONE, DemoStore, build_exchange};
use physician_wire::{
    AutomationRun, ChatSnapshot, CitationDestination, ClinicalSegment, ConnectedSource, FollowUp,
    MedicalRecordNumber, PatientIdentity, PatientLinkWire, PatientSynopsis,
    PatientWorkspaceResponse, RecordGroup, SexOrGender, SnapshotFeedback, SourceConnectionState,
    SourcePresentation, SourcePreviewContent, SourceSnapshot, WorkPurpose,
    WorkspaceMedicationRecommendation, WorkspaceMembership, WorkspacePlanItem, WorkspaceReply,
};
use serde_json::{Value, json};
use std::collections::BTreeSet;
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
        (None, Vec::new())
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
        active_follow_ups: if idx < 4 {
            vec![FollowUp {
                follow_up_id: format!("fu-{ws_id}"),
                description: Some(
                    "Review interval labs and medication adherence before next visit.".into(),
                ),
            }]
        } else {
            Vec::new()
        },
        has_past_work: false,
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
        patient_link: Some(PatientLinkWire {
            practice_id: "prac-coda-demo".into(),
            patient_workspace_id: ws_id.into(),
            duet_account_id: Some(format!("duet-{ws_id}")),
            linked: idx < 12,
            sharing_consented: idx < 10,
            consent_version: Some("v1".into()),
            consented_at: Some(1_785_000_000),
            revoked_at: None,
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
            "reply",
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
    let item_id = format!("today-{ws_id}");
    let row_json = json!({ "patient_workspace_id": ws_id, "patient_display_name": display_name, "item_id": item_id, "version_id": format!("ver-{ws_id}-1"), "waiting_since": waited, "urgency": urgency, "kind": kind, "reason": reason });
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
        ],
    )?;

    let (hr, min) = (19 + (idx % 4), if idx >= 4 { 45 } else { 30 });
    let starts = format!("2026-08-27T{hr:02}:{min:02}:00.000Z");
    let visit_json = json!({
        "patient_display_name": display_name, "patient_workspace_id": ws_id,
        "label": if idx == 0 { "Cardiometabolic follow-up" } else { "Primary care follow-up" },
        "starts_at": starts, "ends_at": format!("2026-08-27T{hr:02}:55:00.000Z"), "status": "scheduled",
        "patient_time_zone": DEMO_ZONE, "preparation": { "state": "prepared", "last_attempt": { "outcome": "completed" } }
    });
    store.driver().insert_row(
        PracticeVisits::Table,
        PracticeVisits::COLS,
        [
            format!("visit-{ws_id}").into(),
            starts.into(),
            visit_json.to_string().into(),
        ],
    )?;
    let upd = json!({ "patient_workspace_id": ws_id, "patient_display_name": display_name, "summary": reason });
    store.driver().insert_row(
        UpdatesFeed::Table,
        UpdatesFeed::COLS,
        [
            format!("upd-{ws_id}").into(),
            ws_id.into(),
            upd.to_string().into(),
            waited.into(),
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

    let (reads, query, answer) = match records.last() {
        Some((id, _, title, dt, _)) => (
            vec![(id.clone(), RecordGroup::Notes, title.clone(), dt.clone())],
            "recent summary labs medications",
            format!(
                "{summary}\n\nLatest documented record: {title} ({dt}). Ask any question below to run live SQLite chart search across {} records.",
                records.len()
            ),
        ),
        None => (
            Vec::new(),
            "",
            format!("{summary} Ask any question below to query this patient's SQLite chart."),
        ),
    };
    let snap = ChatSnapshot {
        sources: Default::default(),
        queued: Vec::new(),
        transcript: vec![build_exchange(
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
            answer,
        )],
        feedback: SnapshotFeedback::Loaded,
    };
    store.driver().insert_row(
        Chats::Table,
        Chats::COLS,
        [
            format!("chat-{ws_id}-1").into(),
            ws_id.into(),
            "Pre-visit chart brief".into(),
            waited.into(),
            serde_json::to_string(&snap)?.into(),
        ],
    )?;
    Ok((reason, replies))
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
