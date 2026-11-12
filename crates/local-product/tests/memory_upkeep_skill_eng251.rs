//! Verification for the `patient-memory-maintenance` skill (`ENG-251`):
//!
//! - AC1: A physician's chat correction makes the warranted memory change through real tools;
//!   the instruction remains distinct from reported chart fact, and unrelated understanding
//!   remains intact.
//! - AC2: A current-medication question reads the live chart (`search_patient_information` +
//!   `read_patient_sources`) instead of treating memory as medication truth.
//! - AC3: Run records capture exact skill versions across initial preloading (`PatientReview`)
//!   and on-demand discovery (`Conversation`, `VisitBriefing`), record reads and writes, verify
//!   guidance names real tools without initialization scope creep, and distinguish a quiet
//!   no-change outcome from a failed/refused memory update.

#[path = "common/disposable_postgres.rs"]
mod disposable_postgres;
#[path = "../../../verification/model_text.rs"]
mod model_text;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use disposable_postgres::DisposablePostgres;
use harness::engine::results::ToolResult;
use harness::engine::{EventKind, Invocation, Item, Tool};
use harness::skills::{
    Package, SKILL_VIEW, SkillLoadOrigin, SkillLoadRecord, SkillTarget, SkillView, version_id,
};
use local_product::ModelBinding;
use local_product::physician_chat::{self, PhysicianJob, PhysicianSkills, job_recipe};
use local_product::physician_sources::{READ_TOOL, RecordDate, SEARCH_TOOL, SourceRevision};
use local_product::runtime::engine::recipe::{Observer, Provider, Request};
use local_product::runtime::memory::{
    CorrectionStanding, MemoryRead, MemoryVersionNumber, READ_MEMORY_TOOL, RECORD_CORRECTION_TOOL,
    SAVE_MEMORY_TOOL, SAVE_SUMMARY_TOOL, SourceMessageRef, SummaryReference, TopicKey,
    VersionSelection,
};
use local_product::runtime::{
    Accepted, Authority, Input, MessageRole, PhysicianHost, PublicationTransaction, Scope,
    WorkStatus, async_trait, engine,
};
use model_text::{assert_clean_model_text, assert_skill_tools};
use serde_json::{Value, json};
use uuid::Uuid;

fn skills_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../skills")
}

fn skills() -> &'static PhysicianSkills {
    static SKILLS: OnceLock<PhysicianSkills> = OnceLock::new();
    SKILLS.get_or_init(|| PhysicianSkills::load(&skills_root()).expect("load workspace skills"))
}

fn file_digest(relative: &str) -> String {
    let bytes = std::fs::read(skills_root().join(relative)).expect("skill file exists");
    version_id(&bytes)
}

fn fixture_binding(label: &str) -> ModelBinding {
    ModelBinding {
        model: format!("fixture-{label}"),
        effort: "high".into(),
        context_window_tokens: 272_000,
        max_input_tokens: 250_000,
        adapter: "fixture".into(),
    }
}

fn make_source(
    workspace: &str,
    source_id: &str,
    candidate_id: &str,
    revision: &str,
    record_type: &str,
    recorded_date: &str,
    content: &str,
) -> SourceRevision {
    SourceRevision {
        workspace: workspace.into(),
        source_id: source_id.into(),
        candidate_id: candidate_id.into(),
        revision: revision.into(),
        record_type: record_type.into(),
        dates: vec![RecordDate {
            name: "recorded".into(),
            value: recorded_date.into(),
        }],
        content: content.into(),
        sha256: local_product::sha256_hex(content.as_bytes()),
        readable_parts: Vec::new(),
        provenance: json!({
            "source_id": source_id,
            "native_id": candidate_id,
            "revision": revision,
        }),
    }
}

fn topic(name: &str) -> TopicKey {
    TopicKey::parse(name).expect("valid topic key")
}

struct AllowSession {
    expected_practice: String,
    expected_physician: String,
    allowed_workspaces: BTreeSet<String>,
}

impl AllowSession {
    fn single(scope: &Scope) -> Self {
        let mut allowed_workspaces = BTreeSet::new();
        allowed_workspaces.insert(scope.workspace.clone());
        Self {
            expected_practice: scope.practice.clone(),
            expected_physician: scope.physician.clone(),
            allowed_workspaces,
        }
    }

    fn verify(&self, scope: &Scope, session_hash: Option<&[u8]>) -> Result<(), String> {
        let Some(hash) = session_hash else {
            return Err("missing session reference".into());
        };
        if hash != [7_u8; 32] {
            return Err("revoked or unrecognized session".into());
        }
        if scope.practice != self.expected_practice {
            return Err("practice mismatch".into());
        }
        if scope.physician != self.expected_physician {
            return Err("physician mismatch".into());
        }
        if !self.allowed_workspaces.contains(&scope.workspace) {
            return Err("patient workspace not on physician panel".into());
        }
        Ok(())
    }
}

#[async_trait]
impl Authority for AllowSession {
    async fn check(
        &mut self,
        _tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        self.verify(scope, session_hash)
    }

    async fn lock_publication(
        &mut self,
        _tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        self.verify(scope, session_hash)
    }
}

struct ScriptedProvider {
    turns: Vec<engine::Turn>,
    seen_requests: Vec<Request>,
}

impl ScriptedProvider {
    fn new(turns: Vec<engine::Turn>) -> Self {
        Self {
            turns,
            seen_requests: Vec::new(),
        }
    }
}

impl Provider for ScriptedProvider {
    async fn exchange(
        &mut self,
        request: &Request,
        _cancel: &engine::Cancellation,
        _limits: &engine::RunLimits,
        _observations: &mut dyn Observer,
    ) -> Result<engine::Turn, engine::Cause> {
        self.seen_requests.push(request.clone());
        if self.turns.is_empty() {
            return Err("scripted provider ran out of turns".into());
        }
        Ok(self.turns.remove(0))
    }
}

fn call_turn(calls: Vec<Invocation>) -> engine::Turn {
    engine::Turn {
        items: calls.into_iter().map(Item::Call).collect(),
        receipt: json!({ "turn": "tool_calls" }),
        usage: Default::default(),
    }
}

fn assistant_turn(text: &str) -> engine::Turn {
    engine::Turn {
        items: vec![Item::assistant(text)],
        receipt: json!({ "turn": "assistant" }),
        usage: Default::default(),
    }
}

#[derive(Debug)]
struct RecordedToolCall {
    call_id: String,
    tool_name: String,
    arguments: Value,
    result: ToolResult,
}

fn extract_tool_calls(events: &[engine::Event]) -> Vec<RecordedToolCall> {
    let mut started: Vec<(String, String, Value)> = Vec::new();
    let mut completed = Vec::new();
    for event in events {
        match &event.kind {
            EventKind::ToolStarted(call) => {
                let args = serde_json::from_str(&call.arguments).unwrap_or(Value::Null);
                started.push((call.id.clone(), call.name.clone(), args));
            }
            EventKind::ToolReturned { call_id, result } => {
                if let Some((_, tool_name, arguments)) =
                    started.iter().find(|(id, _, _)| id == call_id).cloned()
                {
                    completed.push(RecordedToolCall {
                        call_id: call_id.clone(),
                        tool_name,
                        arguments,
                        result: result.clone(),
                    });
                }
            }
            _ => {}
        }
    }
    completed
}

#[tokio::test]
async fn ac1_physician_chat_correction_updates_memory_distinct_from_reported_fact_keeping_unrelated_understanding()
 {
    let Some(pg) = DisposablePostgres::acquire("eng251-ac1").await else {
        return;
    };
    let store = pg.store().await;

    let practice = format!("prac-{}", Uuid::new_v4().simple());
    let workspace = "pw-ac1-claire".to_owned();
    let session = [7_u8; 32];
    let binding = fixture_binding("ac1");

    // Ingest signed cardiology office note into the live chart.
    let chart_note = make_source(
        &workspace,
        "ehr-athena",
        "cand-cardio-note-1001",
        "rev-cardio-2026-10-01",
        "DocumentReference",
        "2026-10-01",
        "Signed office note (2026-10-01): Stage 1 hypertension; started amlodipine 5 mg oral tablet once daily.",
    );
    store.record_source(&chart_note).await.unwrap();

    // Seed initial patient memory (v1 operating index across 4 topics, v2 summary citing the signed note).
    let seed_scope = Scope {
        practice: practice.clone(),
        physician: "dr-nguyen".into(),
        workspace: workspace.clone(),
        chat: "chat-seed".into(),
    };
    let seed_recipe = job_recipe(&binding, PhysicianJob::PatientReview, skills());
    assert_eq!(
        store
            .accept_authorized(
                &seed_scope,
                &Input {
                    key: "seed-review-1".into(),
                    text: "Review signed 2026-10-01 office note.".into(),
                    timezone: "America/Los_Angeles".into(),
                    scenario: None,
                },
                session.to_vec(),
                skills(),
                &seed_recipe,
            )
            .await
            .unwrap(),
        Accepted::New
    );
    let seed_claim = store.claim(&seed_scope, 60).await.unwrap().unwrap();
    let mut seed_provider = ScriptedProvider::new(vec![
        call_turn(vec![
            Invocation {
                id: "seed-save-mem".into(),
                name: SAVE_MEMORY_TOOL.into(),
                arguments: json!({
                    "upsert": [
                        {
                            "topic": "hypertension",
                            "note": "Started amlodipine 5 mg daily per 2026-10-01 signed office note."
                        },
                        {
                            "topic": "preventive_care",
                            "note": "Screening colonoscopy normal on 2024-05-18; next due 2034."
                        },
                        {
                            "topic": "incidental_findings",
                            "note": "4 mm right lower lobe pulmonary nodule on 2026-06-12 CT; no routine follow-up required in low-risk never-smoker."
                        },
                        {
                            "topic": "social_context",
                            "note": "High-school chemistry teacher; prefers early-morning appointments."
                        }
                    ],
                    "remove": []
                })
                .to_string(),
            },
            Invocation {
                id: "seed-save-summary".into(),
                name: SAVE_SUMMARY_TOOL.into(),
                arguments: json!({
                    "content": "Stage 1 hypertension started on amlodipine 5 mg daily at the 2026-10-01 office visit.",
                    "references": [
                        {
                            "kind": "chart_record",
                            "source_id": "ehr-athena",
                            "candidate_id": "cand-cardio-note-1001",
                            "revision": "rev-cardio-2026-10-01",
                            "label": "2026-10-01 office note",
                            "record_date": "2026-10-01"
                        }
                    ],
                    "dates": {
                        "written_at": "2026-10-01",
                        "reviewed_at": "2026-10-01",
                        "pulled_at": "2026-10-01"
                    }
                })
                .to_string(),
            },
        ]),
        assistant_turn(
            "Updated memory from the 2026-10-01 signed office note.\n\nBasis: 2026-10-01 office note.",
        ),
    ]);
    PhysicianHost {
        store: store.clone(),
        claim: seed_claim,
        authority: AllowSession::single(&seed_scope),
    }
    .run(
        &mut seed_provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Physician sends a correction in interactive chat clarifying why amlodipine was stopped
    // and directing Coda to hold replacement medication pending home BP logs.
    let corr_scope = Scope {
        practice: practice.clone(),
        physician: "dr-nguyen".into(),
        workspace: workspace.clone(),
        chat: "chat-physician-correction".into(),
    };
    let chat_recipe = job_recipe(&binding, PhysicianJob::Conversation, skills());
    assert_eq!(
        store
            .accept_authorized(
                &corr_scope,
                &Input {
                    key: "msg-corr-amlodipine-77".into(),
                    text: "She stopped amlodipine last week due to bilateral ankle edema; hold off starting another antihypertensive until she brings two weeks of home BP readings next month.".into(),
                    timezone: "America/Los_Angeles".into(),
                    scenario: None,
                },
                session.to_vec(),
                skills(),
                &chat_recipe,
            )
            .await
            .unwrap(),
        Accepted::New
    );
    let corr_claim = store.claim(&corr_scope, 60).await.unwrap().unwrap();
    let mut corr_provider = ScriptedProvider::new(vec![
        // Turn 1: Open patient-memory-maintenance + clinical + physician communication skills,
        // read current memory, and check the signed 2026-10-01 chart note.
        call_turn(vec![
            Invocation {
                id: "open-mem-skill".into(),
                name: SKILL_VIEW.into(),
                arguments: json!({ "name": "patient-memory-maintenance" }).to_string(),
            },
            Invocation {
                id: "open-shared-comm".into(),
                name: SKILL_VIEW.into(),
                arguments: json!({ "name": "clinical-communication" }).to_string(),
            },
            Invocation {
                id: "open-phys-comm".into(),
                name: SKILL_VIEW.into(),
                arguments: json!({ "name": "physician-clinical-communication" }).to_string(),
            },
            Invocation {
                id: "read-mem-current".into(),
                name: READ_MEMORY_TOOL.into(),
                arguments: json!({ "version": null }).to_string(),
            },
            Invocation {
                id: "read-chart-note".into(),
                name: READ_TOOL.into(),
                arguments: json!({
                    "references": [{
                        "candidate_id": "cand-cardio-note-1001",
                        "revision": "rev-cardio-2026-10-01",
                        "cursor": null
                    }],
                    "max_result_bytes": 16384
                })
                .to_string(),
            },
        ]),
        // Turn 2: Record dated physician correction (keeping physician statement distinct from the
        // 2026-10-01 EHR note and patching only `hypertension`) and update patient-page summary.
        call_turn(vec![
            Invocation {
                id: "record-corr-1".into(),
                name: RECORD_CORRECTION_TOOL.into(),
                arguments: json!({
                    "correction_id": "corr-amlodipine-edema-hold",
                    "recorded_on": "2026-11-11",
                    "statement": "Patient stopped amlodipine due to bilateral ankle edema; hold replacement antihypertensive until 2-week home BP readings are reviewed next month.",
                    "source_message_id": null,
                    "source_chat_id": null,
                    "source_physician_id": null,
                    "supersedes_correction_id": null,
                    "upsert_topics": [
                        {
                            "topic": "hypertension",
                            "note": "Per physician direction on 2026-11-11, patient stopped amlodipine due to bilateral ankle edema and is holding replacement therapy pending 2-week home BP readings; the last signed chart note (2026-10-01) still reflects initial amlodipine 5 mg initiation."
                        }
                    ],
                    "remove_topics": []
                })
                .to_string(),
            },
            Invocation {
                id: "update-summary-1".into(),
                name: SAVE_SUMMARY_TOOL.into(),
                arguments: json!({
                    "content": "Stage 1 hypertension: amlodipine 5 mg started on 2026-10-01 was discontinued due to bilateral ankle edema per Dr. Nguyen on 2026-11-11; replacement therapy is on hold pending a 2-week home blood pressure log.",
                    "references": [
                        {
                            "kind": "chart_record",
                            "source_id": "ehr-athena",
                            "candidate_id": "cand-cardio-note-1001",
                            "revision": "rev-cardio-2026-10-01",
                            "label": "2026-10-01 office note",
                            "record_date": "2026-10-01"
                        },
                        {
                            "kind": "conversation_message",
                            "chat_id": "chat-physician-correction",
                            "message_id": "msg-corr-amlodipine-77",
                            "label": "Dr. Nguyen chat direction",
                            "stated_on": "2026-11-11"
                        }
                    ],
                    "dates": {
                        "written_at": "2026-11-11",
                        "reviewed_at": "2026-11-11",
                        "pulled_at": "2026-10-01"
                    }
                })
                .to_string(),
            },
        ]),
        assistant_turn(
            "Recorded your 2026-11-11 direction: she stopped amlodipine for bilateral ankle edema and we will hold replacement therapy until reviewing her two-week home blood pressure log next month. Note that the latest signed chart record (2026-10-01) still lists the initial amlodipine 5 mg start.\n\nBasis: Physician direction in this conversation (2026-11-11) and 2026-10-01 signed office note.",
        ),
    ]);
    PhysicianHost {
        store: store.clone(),
        claim: corr_claim,
        authority: AllowSession::single(&corr_scope),
    }
    .run(
        &mut corr_provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Verify Turn 2 received the full patient-memory-maintenance body via skill_view.
    assert!(corr_provider.seen_requests[1].items.iter().any(|item| {
        matches!(
            item,
            Item::MaterializedResult { call_id, output }
                if call_id == "open-mem-skill" && output.contains("# Patient Memory Maintenance")
        )
    }));

    // Inspect saved versions in Postgres: v1 & v2 (initial understanding + summary),
    // v3 (dated correction + hypertension patch), v4 (updated patient-page summary).
    let MemoryRead::Available(current) = store
        .patient_memory_authorized(
            &corr_scope,
            &session,
            VersionSelection::Current,
            &mut AllowSession::single(&corr_scope),
        )
        .await
        .unwrap()
    else {
        panic!("expected current memory snapshot");
    };
    assert_eq!(current.version.get(), 4);

    // 1. Physician correction is recorded with exact conversation provenance, distinct from EHR facts.
    assert_eq!(current.corrections.len(), 1);
    let corr = &current.corrections[0];
    assert_eq!(corr.correction_id.as_str(), "corr-amlodipine-edema-hold");
    assert_eq!(corr.recorded_on.as_str(), "2026-11-11");
    assert_eq!(corr.standing, CorrectionStanding::Active);
    assert_eq!(
        corr.source_message,
        SourceMessageRef::new(
            "chat-physician-correction",
            "msg-corr-amlodipine-77",
            "dr-nguyen"
        )
        .unwrap()
    );
    assert!(corr.statement.contains("hold replacement antihypertensive"));

    // 2. Unrelated operating-index understanding remained byte-for-byte intact across v1 -> v4.
    assert_eq!(
        current.operating_index.get(&topic("preventive_care")),
        Some("Screening colonoscopy normal on 2024-05-18; next due 2034.")
    );
    assert_eq!(
        current.operating_index.get(&topic("incidental_findings")),
        Some(
            "4 mm right lower lobe pulmonary nodule on 2026-06-12 CT; no routine follow-up required in low-risk never-smoker."
        )
    );
    assert_eq!(
        current.operating_index.get(&topic("social_context")),
        Some("High-school chemistry teacher; prefers early-morning appointments.")
    );
    // And the patched `hypertension` topic explicitly distinguishes physician direction from the 2026-10-01 chart note.
    let htn_note = current
        .operating_index
        .get(&topic("hypertension"))
        .expect("hypertension topic present");
    assert!(htn_note.contains("Per physician direction on 2026-11-11"));
    assert!(htn_note.contains("2026-10-01"));

    // 3. Summary separates chart_record provenance from conversation_message provenance and leaks no private topics.
    let summary = current.summary.as_ref().expect("saved summary present");
    assert_eq!(summary.summary.references.len(), 2);
    assert!(matches!(
        &summary.summary.references[0],
        SummaryReference::ChartRecord { candidate_id, revision, .. }
            if candidate_id == "cand-cardio-note-1001" && revision == "rev-cardio-2026-10-01"
    ));
    assert!(matches!(
        &summary.summary.references[1],
        SummaryReference::ConversationMessage { chat_id, message_id, .. }
            if chat_id == "chat-physician-correction" && message_id == "msg-corr-amlodipine-77"
    ));
    assert!(!summary.summary.content.contains("chemistry teacher"));
    assert!(!summary.summary.content.contains("pulmonary nodule"));

    // 4. Historical v2 snapshot remains preserved and inspectable in version history.
    let MemoryRead::Available(v2) = store
        .patient_memory_authorized(
            &corr_scope,
            &session,
            VersionSelection::Exact(MemoryVersionNumber::new(2).unwrap()),
            &mut AllowSession::single(&corr_scope),
        )
        .await
        .unwrap()
    else {
        panic!("expected exact version 2");
    };
    assert!(v2.corrections.is_empty());
    assert_eq!(
        v2.operating_index.get(&topic("hypertension")),
        Some("Started amlodipine 5 mg daily per 2026-10-01 signed office note.")
    );
}

#[tokio::test]
async fn ac2_current_medication_question_reads_live_chart_instead_of_treating_memory_as_truth() {
    let Some(pg) = DisposablePostgres::acquire("eng251-ac2").await else {
        return;
    };
    let store = pg.store().await;

    let practice = format!("prac-{}", Uuid::new_v4().simple());
    let workspace = "pw-ac2-marcus".to_owned();
    let session = [7_u8; 32];
    let binding = fixture_binding("ac2");

    // Seed patient memory with an OLDER dose orientation note from 2026-09-01 ("lisinopril 10 mg daily").
    let seed_scope = Scope {
        practice: practice.clone(),
        physician: "dr-patel".into(),
        workspace: workspace.clone(),
        chat: "chat-seed-meds".into(),
    };
    let review_recipe = job_recipe(&binding, PhysicianJob::PatientReview, skills());
    store
        .accept_authorized(
            &seed_scope,
            &Input {
                key: "seed-meds-1".into(),
                text: "Record September visit orientation.".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            session.to_vec(),
            skills(),
            &review_recipe,
        )
        .await
        .unwrap();
    let seed_claim = store.claim(&seed_scope, 60).await.unwrap().unwrap();
    let mut seed_provider = ScriptedProvider::new(vec![
        call_turn(vec![Invocation {
            id: "seed-stale-med-note".into(),
            name: SAVE_MEMORY_TOOL.into(),
            arguments: json!({
                "upsert": [
                    {
                        "topic": "antihypertensive_context",
                        "note": "Initiated lisinopril 10 mg once daily on 2026-09-01; tolerated well initially."
                    }
                ],
                "remove": []
            })
            .to_string(),
        }]),
        assistant_turn("Saved initial hypertension note."),
    ]);
    PhysicianHost {
        store: store.clone(),
        claim: seed_claim,
        authority: AllowSession::single(&seed_scope),
    }
    .run(
        &mut seed_provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Subsequent live chart feed delivers updated MedicationRequest on 2026-11-10:
    // lisinopril uptitrated to 20 mg daily + chlorthalidone 12.5 mg daily added.
    store
        .record_source(&make_source(
            &workspace,
            "ehr-epic",
            "cand-active-meds-2026-11",
            "rev-meds-2026-11-10",
            "MedicationRequest",
            "2026-11-10",
            "Active Medication List (2026-11-10): 1) Lisinopril 20 mg oral tablet once daily (uptitrated 2026-10-28). 2) Chlorthalidone 12.5 mg oral tablet once daily (started 2026-11-10).",
        ))
        .await
        .unwrap();
    store
        .record_source(&make_source(
            &workspace,
            "ehr-epic",
            "cand-bmp-2026-11",
            "rev-bmp-2026-11-10",
            "Observation",
            "2026-11-10",
            "Basic Metabolic Panel (2026-11-10): Potassium 4.2 mmol/L, Creatinine 0.94 mg/dL.",
        ))
        .await
        .unwrap();

    // Physician asks a current-medication question in chat.
    let ask_scope = Scope {
        practice: practice.clone(),
        physician: "dr-patel".into(),
        workspace: workspace.clone(),
        chat: "chat-current-meds".into(),
    };
    let chat_recipe = job_recipe(&binding, PhysicianJob::Conversation, skills());
    store
        .accept_authorized(
            &ask_scope,
            &Input {
                key: "ask-meds-now-1".into(),
                text: "What blood pressure medications and doses is he currently taking?".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            session.to_vec(),
            skills(),
            &chat_recipe,
        )
        .await
        .unwrap();
    let ask_claim = store.claim(&ask_scope, 60).await.unwrap().unwrap();
    let mut ask_provider = ScriptedProvider::new(vec![
        // Turn 1: Load patient-memory-maintenance + communication skills, read memory (seeing stale 10 mg note),
        // and search the live chart for active medication records rather than answering from memory.
        call_turn(vec![
            Invocation {
                id: "med-skill".into(),
                name: SKILL_VIEW.into(),
                arguments: json!({ "name": "patient-memory-maintenance" }).to_string(),
            },
            Invocation {
                id: "med-shared-comm".into(),
                name: SKILL_VIEW.into(),
                arguments: json!({ "name": "clinical-communication" }).to_string(),
            },
            Invocation {
                id: "med-phys-comm".into(),
                name: SKILL_VIEW.into(),
                arguments: json!({ "name": "physician-clinical-communication" }).to_string(),
            },
            Invocation {
                id: "med-read-memory".into(),
                name: READ_MEMORY_TOOL.into(),
                arguments: json!({ "version": null }).to_string(),
            },
            Invocation {
                id: "med-search-chart".into(),
                name: SEARCH_TOOL.into(),
                arguments: json!({
                    "query": "lisinopril medication",
                    "record_type": "MedicationRequest",
                    "after": null
                })
                .to_string(),
            },
        ]),
        // Turn 2: Open exact live chart MedicationRequest revision with read_patient_sources,
        // then reconcile the stale operating-index note with the 2026-11-10 chart record.
        call_turn(vec![
            Invocation {
                id: "med-read-live-chart".into(),
                name: READ_TOOL.into(),
                arguments: json!({
                    "references": [{
                        "candidate_id": "cand-active-meds-2026-11",
                        "revision": "rev-meds-2026-11-10",
                        "cursor": null
                    }],
                    "max_result_bytes": 16384
                })
                .to_string(),
            },
            Invocation {
                id: "med-reconcile-memory".into(),
                name: SAVE_MEMORY_TOOL.into(),
                arguments: json!({
                    "upsert": [
                        {
                            "topic": "antihypertensive_context",
                            "note": "Active chart list on 2026-11-10 shows lisinopril 20 mg once daily (uptitrated 2026-10-28 from initial 10 mg started 2026-09-01) plus chlorthalidone 12.5 mg once daily."
                        }
                    ],
                    "remove": []
                })
                .to_string(),
            },
        ]),
        assistant_turn(
            "He is currently prescribed lisinopril 20 mg once daily (uptitrated on 2026-10-28) and chlorthalidone 12.5 mg once daily (started on 2026-11-10).\n\nBasis: Active Medication List from Epic dated 2026-11-10 (`cand-active-meds-2026-11`, revision `rev-meds-2026-11-10`).",
        ),
    ]);
    PhysicianHost {
        store: store.clone(),
        claim: ask_claim,
        authority: AllowSession::single(&ask_scope),
    }
    .run(
        &mut ask_provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Inspect retained run record and published conversation in Postgres.
    let saved_conv = store.read(&ask_scope).await.unwrap().unwrap();
    assert_eq!(saved_conv.status, WorkStatus::Completed);
    let calls = extract_tool_calls(&saved_conv.attempts[0].events);

    // 1. Memory was read and actually contained the stale "10 mg once daily" note.
    let mem_read = calls
        .iter()
        .find(|c| c.call_id == "med-read-memory")
        .expect("read_patient_memory recorded");
    assert_eq!(mem_read.tool_name, READ_MEMORY_TOOL);
    let mem_output = mem_read
        .result
        .original()
        .expect("executed memory read bytes");
    assert!(mem_output.contains("Initiated lisinopril 10 mg once daily on 2026-09-01"));

    // 2. Search and exact live chart read (`read_patient_sources`) executed before the final answer.
    let chart_search = calls
        .iter()
        .find(|c| c.call_id == "med-search-chart")
        .expect("search_patient_information recorded");
    assert_eq!(chart_search.tool_name, SEARCH_TOOL);
    assert!(
        chart_search
            .result
            .original()
            .unwrap()
            .contains("cand-active-meds-2026-11")
    );

    let chart_read = calls
        .iter()
        .find(|c| c.call_id == "med-read-live-chart")
        .expect("read_patient_sources recorded");
    assert_eq!(chart_read.tool_name, READ_TOOL);
    assert_eq!(
        chart_read.arguments["references"][0]["candidate_id"],
        "cand-active-meds-2026-11"
    );
    assert_eq!(
        chart_read.arguments["references"][0]["revision"],
        "rev-meds-2026-11-10"
    );
    let chart_output = chart_read
        .result
        .original()
        .expect("executed live chart read");
    assert!(chart_output.contains("Lisinopril 20 mg oral tablet once daily"));
    assert!(chart_output.contains("Chlorthalidone 12.5 mg oral tablet once daily"));

    // 3. Published physician answer reflects live chart truth (20 mg + chlorthalidone 12.5 mg),
    //    never reporting stale 10 mg memory as the current regimen.
    let assistant_messages: Vec<&str> = saved_conv
        .messages
        .iter()
        .filter(|m| matches!(m.role, MessageRole::Assistant { .. }))
        .map(|m| m.text.as_str())
        .collect();
    assert_eq!(assistant_messages.len(), 1);
    assert!(assistant_messages[0].contains("lisinopril 20 mg once daily"));
    assert!(assistant_messages[0].contains("chlorthalidone 12.5 mg once daily"));
    assert!(!assistant_messages[0].contains("10 mg"));

    // 4. Reconciled operating index in Postgres now reflects the 2026-11-10 chart regimen at v2.
    let MemoryRead::Available(reconciled) = store
        .patient_memory_authorized(
            &ask_scope,
            &session,
            VersionSelection::Current,
            &mut AllowSession::single(&ask_scope),
        )
        .await
        .unwrap()
    else {
        panic!("expected v2 memory");
    };
    assert_eq!(reconciled.version.get(), 2);
    assert!(
        reconciled
            .operating_index
            .get(&topic("antihypertensive_context"))
            .unwrap()
            .contains("lisinopril 20 mg once daily")
    );
}

#[tokio::test]
async fn ac3_run_records_capture_skill_versions_preload_and_discovery_and_distinguish_no_change_from_failed_update()
 {
    // Part A: Guidance contract inspection — real tool names, Chords-only visibility,
    // initialization excluded, and explicit no-change vs failed-update rules.
    let root = skills_root();
    let chords = skills();
    let duet = Package::load_visible("duet", &root, |skill| {
        skill.visibility.admits_explicit("duet")
    })
    .expect("load duet skills");

    let mut offered = physician_chat::tools()
        .iter()
        .map(Tool::function)
        .collect::<Vec<_>>();
    offered.extend(local_product::runtime::reply::tools());
    assert_skill_tools(&serde_json::to_value(chords).unwrap(), &offered, &[]);

    let view = chords
        .package()
        .execute_skill_view(&json!({ "name": "patient-memory-maintenance" }))
        .expect("open patient-memory-maintenance in chords");
    let SkillView::Available { content, .. } = view else {
        panic!("expected Available skill view, got {view:?}");
    };
    assert_clean_model_text(&content);
    for required_tool in [
        READ_MEMORY_TOOL,
        SAVE_MEMORY_TOOL,
        RECORD_CORRECTION_TOOL,
        SAVE_SUMMARY_TOOL,
        SEARCH_TOOL,
        READ_TOOL,
    ] {
        assert!(
            content.contains(required_tool),
            "patient-memory-maintenance must name real offered tool {required_tool}"
        );
    }
    assert!(
        content.contains(
            "Initial patient understanding when a patient's chart is first connected is a separate workflow"
        ),
        "skill must explicitly keep initial chart-connection initialization out of scope"
    );
    assert!(content.contains("**No change warranted**"));
    assert!(content.contains("**Failed or refused update**"));

    // Duet cannot list or open Chords-only memory upkeep guidance.
    assert!(!duet.skill_names().contains(&"patient-memory-maintenance"));
    assert_eq!(
        duet.execute_skill_view(&json!({ "name": "patient-memory-maintenance" }))
            .unwrap(),
        SkillView::UnknownSkill {
            name: "patient-memory-maintenance".into(),
            file_path: None,
            available_skills: vec![
                "clinical-communication".into(),
                "duet-health-communication".into(),
            ],
        }
    );

    let expected_skill_version = file_digest("patient-memory-maintenance/SKILL.md");

    // Part B & Part C: Durable Postgres run traces across preloaded PatientReview (no change warranted)
    // and discovered VisitBriefing + Conversation (refused/failed writes vs recovered write).
    let Some(pg) = DisposablePostgres::acquire("eng251-ac3").await else {
        return;
    };
    let store = pg.store().await;
    let practice = format!("prac-{}", Uuid::new_v4().simple());
    let workspace = "pw-ac3-elena".to_owned();
    let session = [7_u8; 32];
    let binding = fixture_binding("ac3");

    store
        .record_source(&make_source(
            &workspace,
            "ehr-main",
            "cand-a1c-2026-11",
            "rev-a1c-1",
            "Observation",
            "2026-11-05",
            "Hemoglobin A1c (2026-11-05): 6.4% (goal <7.0% met on stable metformin 1000 mg twice daily).",
        ))
        .await
        .unwrap();

    // Seed v1 memory so baseline understanding already matches the 2026-11-05 A1c result.
    let seed_scope = Scope {
        practice: practice.clone(),
        physician: "dr-chen".into(),
        workspace: workspace.clone(),
        chat: "chat-ac3-seed".into(),
    };
    let review_recipe = job_recipe(&binding, PhysicianJob::PatientReview, chords);
    store
        .accept_authorized(
            &seed_scope,
            &Input {
                key: "seed-ac3-1".into(),
                text: "Seed baseline diabetes memory.".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            session.to_vec(),
            chords,
            &review_recipe,
        )
        .await
        .unwrap();
    let seed_claim = store.claim(&seed_scope, 60).await.unwrap().unwrap();
    let mut seed_provider = ScriptedProvider::new(vec![
        call_turn(vec![Invocation {
            id: "seed-v1".into(),
            name: SAVE_MEMORY_TOOL.into(),
            arguments: json!({
                "upsert": [
                    {
                        "topic": "diabetes",
                        "note": "Type 2 diabetes at goal (A1c 6.4% on 2026-11-05) on metformin 1000 mg twice daily."
                    }
                ],
                "remove": []
            })
            .to_string(),
        }]),
        assistant_turn("Baseline diabetes memory saved."),
    ]);
    PhysicianHost {
        store: store.clone(),
        claim: seed_claim,
        authority: AllowSession::single(&seed_scope),
    }
    .run(
        &mut seed_provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Part B: Preloaded `PatientReview` run where NO memory change is warranted.
    let quiet_review_scope = Scope {
        practice: practice.clone(),
        physician: "dr-chen".into(),
        workspace: workspace.clone(),
        chat: "job-review-no-change".into(),
    };
    store
        .accept_authorized(
            &quiet_review_scope,
            &Input {
                key: "review-no-change-1".into(),
                text: "Duplicate copy of 2026-11-05 A1c 6.4% arrived from quest feed.".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            session.to_vec(),
            chords,
            &review_recipe,
        )
        .await
        .unwrap();
    let quiet_claim = store.claim(&quiet_review_scope, 60).await.unwrap().unwrap();
    let mut quiet_provider = ScriptedProvider::new(vec![
        call_turn(vec![
            Invocation {
                id: "quiet-read-mem".into(),
                name: READ_MEMORY_TOOL.into(),
                arguments: json!({ "version": null }).to_string(),
            },
            Invocation {
                id: "quiet-read-chart".into(),
                name: READ_TOOL.into(),
                arguments: json!({
                    "references": [{
                        "candidate_id": "cand-a1c-2026-11",
                        "revision": "rev-a1c-1",
                        "cursor": null
                    }],
                    "max_result_bytes": 16384
                })
                .to_string(),
            },
        ]),
        assistant_turn(
            "Updates: Duplicate 2026-11-05 A1c 6.4% matches current diabetes control at goal; no action or memory change needed.\n\nBasis: 2026-11-05 Hemoglobin A1c (`cand-a1c-2026-11`, `rev-a1c-1`).",
        ),
    ]);
    PhysicianHost {
        store: store.clone(),
        claim: quiet_claim,
        authority: AllowSession::single(&quiet_review_scope),
    }
    .run(
        &mut quiet_provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let quiet_saved = store.read(&quiet_review_scope).await.unwrap().unwrap();
    let quiet_frozen_skills = store
        .frozen_skills(&quiet_review_scope, &quiet_saved.skills)
        .await
        .unwrap();
    let quiet_frozen_binding = store
        .frozen_binding(
            &quiet_review_scope,
            quiet_saved.bindings.get(&quiet_saved.work).unwrap(),
        )
        .await
        .unwrap();
    let quiet_trace = quiet_frozen_skills
        .trace_run(
            &quiet_frozen_binding.recipe,
            &quiet_saved.attempts[0].events,
        )
        .unwrap();
    assert!(
        quiet_trace.loads.contains(&SkillLoadRecord {
            target: SkillTarget::body("patient-memory-maintenance"),
            origin: SkillLoadOrigin::Preloaded,
            version_id: expected_skill_version.clone(),
        }),
        "PatientReview must record preloaded patient-memory-maintenance version in run trace"
    );
    let quiet_calls = extract_tool_calls(&quiet_saved.attempts[0].events);
    assert_eq!(quiet_calls.len(), 2);
    assert!(quiet_calls.iter().all(|c| c.result.original().is_some()));
    assert!(
        !quiet_calls.iter().any(|c| matches!(
            c.tool_name.as_str(),
            SAVE_MEMORY_TOOL | RECORD_CORRECTION_TOOL | SAVE_SUMMARY_TOOL
        )),
        "no-change review must emit zero memory mutation calls"
    );

    // Memory remains cleanly at v1 after quiet no-change review.
    let MemoryRead::Available(after_quiet) = store
        .patient_memory_authorized(
            &quiet_review_scope,
            &session,
            VersionSelection::Current,
            &mut AllowSession::single(&quiet_review_scope),
        )
        .await
        .unwrap()
    else {
        panic!("expected v1 memory");
    };
    assert_eq!(after_quiet.version.get(), 1);

    // Part C1: Discovered `VisitBriefing` run where a memory write FAILS (is refused) and remains
    // uncommitted — proving retained run records distinguish a failed update from Part B's quiet no-change.
    let briefing_scope = Scope {
        practice: practice.clone(),
        physician: "dr-chen".into(),
        workspace: workspace.clone(),
        chat: "job-briefing-failed-write".into(),
    };
    let briefing_recipe = job_recipe(&binding, PhysicianJob::VisitBriefing, chords);
    assert!(
        !briefing_recipe
            .instructions
            .contains("# Patient Memory Maintenance"),
        "VisitBriefing keeps patient-memory-maintenance out of initial context until discovered"
    );
    store
        .accept_authorized(
            &briefing_scope,
            &Input {
                key: "briefing-fail-1".into(),
                text: "Prepare visit briefing for tomorrow's diabetes follow-up.".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            session.to_vec(),
            chords,
            &briefing_recipe,
        )
        .await
        .unwrap();
    let briefing_claim = store.claim(&briefing_scope, 60).await.unwrap().unwrap();
    let mut briefing_provider = ScriptedProvider::new(vec![
        call_turn(vec![
            Invocation {
                id: "brief-discover-mem-skill".into(),
                name: SKILL_VIEW.into(),
                arguments: json!({ "name": "patient-memory-maintenance" }).to_string(),
            },
            Invocation {
                id: "brief-read-mem".into(),
                name: READ_MEMORY_TOOL.into(),
                arguments: json!({ "version": null }).to_string(),
            },
            Invocation {
                id: "brief-read-chart".into(),
                name: READ_TOOL.into(),
                arguments: json!({
                    "references": [{
                        "candidate_id": "cand-a1c-2026-11",
                        "revision": "rev-a1c-1",
                        "cursor": null
                    }],
                    "max_result_bytes": 16384
                })
                .to_string(),
            },
        ]),
        // Attempt an invalid patch where `diabetes` appears in both `upsert` and `remove`.
        call_turn(vec![Invocation {
            id: "brief-refused-save".into(),
            name: SAVE_MEMORY_TOOL.into(),
            arguments: json!({
                "upsert": [{ "topic": "diabetes", "note": "Updated pre-visit note." }],
                "remove": ["diabetes"]
            })
            .to_string(),
        }]),
        assistant_turn(
            "Type 2 diabetes remains at goal (A1c 6.4% on 2026-11-05) on metformin 1000 mg twice daily; durable patient memory could not be updated in this run.\n\nBasis: 2026-11-05 Hemoglobin A1c (`cand-a1c-2026-11`, `rev-a1c-1`); durable memory write was refused and did not save.",
        ),
    ]);
    PhysicianHost {
        store: store.clone(),
        claim: briefing_claim,
        authority: AllowSession::single(&briefing_scope),
    }
    .run(
        &mut briefing_provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let briefing_saved = store.read(&briefing_scope).await.unwrap().unwrap();
    let briefing_frozen_skills = store
        .frozen_skills(&briefing_scope, &briefing_saved.skills)
        .await
        .unwrap();
    let briefing_frozen_binding = store
        .frozen_binding(
            &briefing_scope,
            briefing_saved.bindings.get(&briefing_saved.work).unwrap(),
        )
        .await
        .unwrap();
    let briefing_trace = briefing_frozen_skills
        .trace_run(
            &briefing_frozen_binding.recipe,
            &briefing_saved.attempts[0].events,
        )
        .unwrap();
    assert!(
        briefing_trace.loads.contains(&SkillLoadRecord {
            target: SkillTarget::body("patient-memory-maintenance"),
            origin: SkillLoadOrigin::Discovered {
                call_id: "brief-discover-mem-skill".into(),
            },
            version_id: expected_skill_version.clone(),
        }),
        "VisitBriefing must record discovered patient-memory-maintenance version in run trace"
    );

    let briefing_calls = extract_tool_calls(&briefing_saved.attempts[0].events);
    let refused_write = briefing_calls
        .iter()
        .find(|c| c.call_id == "brief-refused-save")
        .expect("refused save_patient_memory retained in run record");
    assert_eq!(refused_write.tool_name, SAVE_MEMORY_TOOL);
    assert!(refused_write.result.original().is_none());
    let ToolResult::Refused(refusal) = &refused_write.result else {
        panic!(
            "expected ToolResult::Refused, got {:?}",
            refused_write.result
        );
    };
    let refusal_wire = serde_json::to_value(refusal).unwrap();
    assert_eq!(refusal_wire["executed"], false);
    assert!(
        refusal.reason().contains("upsert") && refusal.reason().contains("remove"),
        "unexpected refusal reason: {}",
        refusal.reason()
    );

    // Postgres memory is STILL at v1 — the failed write neither corrupted v1 nor advanced version!
    let MemoryRead::Available(after_failed_briefing) = store
        .patient_memory_authorized(
            &briefing_scope,
            &session,
            VersionSelection::Current,
            &mut AllowSession::single(&briefing_scope),
        )
        .await
        .unwrap()
    else {
        panic!("expected v1 memory after refused write");
    };
    assert_eq!(after_failed_briefing.version.get(), 1);

    // Part C2: Discovered `Conversation` run where a first correction call is refused for an
    // ambiguous date ("11/11/26") and then corrected ("2026-11-11") and saved at v2 — proving the
    // retained run trace records both the failed attempt (`Refused`) and the committed write (`Original`).
    let recovery_scope = Scope {
        practice: practice.clone(),
        physician: "dr-chen".into(),
        workspace: workspace.clone(),
        chat: "chat-refusal-then-save".into(),
    };
    let chat_recipe = job_recipe(&binding, PhysicianJob::Conversation, chords);
    store
        .accept_authorized(
            &recovery_scope,
            &Input {
                key: "msg-corr-recovery-1".into(),
                text: "She takes extended-release metformin with dinner to avoid GI upset.".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            session.to_vec(),
            chords,
            &chat_recipe,
        )
        .await
        .unwrap();
    let recovery_claim = store.claim(&recovery_scope, 60).await.unwrap().unwrap();
    let mut recovery_provider = ScriptedProvider::new(vec![
        call_turn(vec![
            Invocation {
                id: "rec-open-skill".into(),
                name: SKILL_VIEW.into(),
                arguments: json!({ "name": "patient-memory-maintenance" }).to_string(),
            },
            Invocation {
                id: "rec-read-mem".into(),
                name: READ_MEMORY_TOOL.into(),
                arguments: json!({ "version": null }).to_string(),
            },
        ]),
        // First write attempt uses ambiguous non-ISO date -> refused!
        call_turn(vec![Invocation {
            id: "rec-bad-date".into(),
            name: RECORD_CORRECTION_TOOL.into(),
            arguments: json!({
                "correction_id": "corr-metformin-er",
                "recorded_on": "11/11/26",
                "statement": "Takes extended-release metformin with dinner for GI tolerance.",
                "source_message_id": null,
                "source_chat_id": null,
                "source_physician_id": null,
                "supersedes_correction_id": null,
                "upsert_topics": [],
                "remove_topics": []
            })
            .to_string(),
        }]),
        // Second write attempt fixes `recorded_on` to YYYY-MM-DD -> executes and commits v2!
        call_turn(vec![Invocation {
            id: "rec-good-date".into(),
            name: RECORD_CORRECTION_TOOL.into(),
            arguments: json!({
                "correction_id": "corr-metformin-er",
                "recorded_on": "2026-11-11",
                "statement": "Takes extended-release metformin with dinner for GI tolerance.",
                "source_message_id": null,
                "source_chat_id": null,
                "source_physician_id": null,
                "supersedes_correction_id": null,
                "upsert_topics": [
                    {
                        "topic": "diabetes",
                        "note": "Type 2 diabetes at goal (A1c 6.4% on 2026-11-05); takes extended-release metformin 1000 mg twice daily with dinner/meals for GI tolerance per Dr. Chen (2026-11-11)."
                    }
                ],
                "remove_topics": []
            })
            .to_string(),
        }]),
        assistant_turn(
            "Recorded that she takes extended-release metformin with dinner for GI tolerance.\n\nBasis: Physician statement in this conversation (2026-11-11).",
        ),
    ]);
    PhysicianHost {
        store: store.clone(),
        claim: recovery_claim,
        authority: AllowSession::single(&recovery_scope),
    }
    .run(
        &mut recovery_provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let recovery_saved = store.read(&recovery_scope).await.unwrap().unwrap();
    let recovery_trace = chords
        .trace_run(&chat_recipe, &recovery_saved.attempts[0].events)
        .unwrap();
    assert_eq!(
        recovery_trace.loads,
        vec![SkillLoadRecord {
            target: SkillTarget::body("patient-memory-maintenance"),
            origin: SkillLoadOrigin::Discovered {
                call_id: "rec-open-skill".into(),
            },
            version_id: expected_skill_version,
        }]
    );

    let recovery_calls = extract_tool_calls(&recovery_saved.attempts[0].events);
    let bad_call = recovery_calls
        .iter()
        .find(|c| c.call_id == "rec-bad-date")
        .expect("bad date call retained");
    assert_eq!(bad_call.arguments["recorded_on"], "11/11/26");
    assert!(matches!(bad_call.result, ToolResult::Refused(_)));

    let good_call = recovery_calls
        .iter()
        .find(|c| c.call_id == "rec-good-date")
        .expect("corrected call retained");
    assert_eq!(good_call.arguments["recorded_on"], "2026-11-11");
    let saved_payload: Value =
        serde_json::from_str(good_call.result.original().expect("executed write")).unwrap();
    assert_eq!(saved_payload["outcome"], "saved");
    assert_eq!(saved_payload["receipt"]["version"], 2);

    let MemoryRead::Available(final_mem) = store
        .patient_memory_authorized(
            &recovery_scope,
            &session,
            VersionSelection::Current,
            &mut AllowSession::single(&recovery_scope),
        )
        .await
        .unwrap()
    else {
        panic!("expected v2 memory");
    };
    assert_eq!(final_mem.version.get(), 2);
    assert_eq!(final_mem.corrections.len(), 1);
    assert_eq!(final_mem.corrections[0].recorded_on.as_str(), "2026-11-11");
}
