//! Verification for the `patient-record-retrieval` chart-reading skill (`ENG-250`):
//!
//! - AC1: Guidance names real tools and explains their results and limits. Initial loading
//!   (`PatientReview`, `VisitBriefing`) and later discovery (`Conversation`) work through the
//!   shared loader, with exact versions recorded in durable Postgres run records.
//! - AC2: A run with the skill reads a chart that has a corrected record and one unreadable
//!   record. The answer uses the readable records (current corrected version, not the superseded
//!   error) and names the record it could not read.

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
use local_product::physician_sources::read::{
    PageNumber, PartState, ScanDocument, ScanDocumentState, ScanPageEntry,
};
use local_product::physician_sources::{READ_TOOL, RecordDate, SourceRevision};
use local_product::runtime::engine::recipe::{Observer, Provider, Request};
use local_product::runtime::memory::{READ_MEMORY_TOOL, SAVE_MEMORY_TOOL};
use local_product::runtime::patient_search::SEARCH_TOOL;
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

struct SourceFixture<'a> {
    workspace: &'a str,
    source_id: &'a str,
    candidate_id: &'a str,
    revision: &'a str,
    record_type: &'a str,
    recorded_date: &'a str,
    content: &'a str,
    scans: Vec<ScanDocument>,
}

fn make_source(spec: SourceFixture<'_>) -> SourceRevision {
    let mut provenance = json!({
        "source_id": spec.source_id,
        "native_id": spec.candidate_id,
        "revision": spec.revision,
    });
    if !spec.scans.is_empty() {
        provenance["scans"] = serde_json::to_value(spec.scans).expect("serialize scans");
    }
    SourceRevision {
        workspace: spec.workspace.into(),
        source_id: spec.source_id.into(),
        candidate_id: spec.candidate_id.into(),
        revision: spec.revision.into(),
        record_type: spec.record_type.into(),
        dates: vec![RecordDate {
            name: "recorded".into(),
            value: spec.recorded_date.into(),
        }],
        content: spec.content.into(),
        sha256: local_product::sha256_hex(spec.content.as_bytes()),
        readable_parts: Vec::new(),
        provenance,
    }
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
async fn ac1_guidance_names_real_tools_explains_results_and_limits_and_records_preload_and_discovery_versions()
 {
    // 1. Skill / tool comparison and product visibility check.
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
        .execute_skill_view(&json!({ "name": "patient-record-retrieval" }))
        .expect("open patient-record-retrieval in chords");
    let SkillView::Available { content, .. } = view else {
        panic!("expected Available skill view, got {view:?}");
    };
    assert_clean_model_text(&content);

    // Every real retrieval and read tool across Chords jobs is documented accurately.
    for required_tool in [
        SEARCH_TOOL,
        READ_TOOL,
        READ_MEMORY_TOOL,
        "list_conversation_history",
        "read_conversation_message",
        "search_conversation_history",
        "read_tool_result",
    ] {
        assert!(
            content.contains(required_tool),
            "patient-record-retrieval must name real offered tool {required_tool}"
        );
    }
    // Unimplemented or historical tools must not appear.
    for absent_tool in [
        "acquire_patient_sources",
        "read_patient_history",
        "read_patient_daily_logs",
    ] {
        assert!(
            !content.contains(absent_tool),
            "patient-record-retrieval must not name unavailable tool {absent_tool}"
        );
    }
    // Key result shapes, location selectors, continuations, standing, and limit disclosures are explained.
    for expected_concept in [
        "Metadata only, never content",
        "snapshot_token",
        "sort_key",
        "part_ordinal",
        "byte_offset",
        "start_byte",
        "end_byte",
        "scan_id",
        "Corrected and superseded revisions",
        "Non-readable parts do not block readable evidence",
        "Plain statement of reading limits",
    ] {
        assert!(
            content.contains(expected_concept),
            "patient-record-retrieval must explain {expected_concept}"
        );
    }

    // Duet never lists or opens Chords chart-reading guidance.
    assert!(!duet.skill_names().contains(&"patient-record-retrieval"));
    assert_eq!(
        duet.execute_skill_view(&json!({ "name": "patient-record-retrieval" }))
            .unwrap(),
        SkillView::UnknownSkill {
            name: "patient-record-retrieval".into(),
            file_path: None,
            available_skills: vec![
                "clinical-communication".into(),
                "duet-health-communication".into(),
            ],
        }
    );

    let expected_retrieval_version = file_digest("patient-record-retrieval/SKILL.md");

    // 2. Durable Postgres run records proving initial preloading (`VisitBriefing`, `PatientReview`)
    //    AND on-demand discovery (`Conversation`), plus multi-domain search and continuation/location reads.
    let Some(pg) = DisposablePostgres::acquire("eng250-ac1").await else {
        return;
    };
    let store = pg.store().await;
    let practice = format!("prac-{}", Uuid::new_v4().simple());
    let workspace = "pw-eng250-ac1".to_owned();
    let session = [7_u8; 32];
    let binding = fixture_binding("eng250-ac1");

    // Seed a multi-part hospital discharge packet (>4,096 bytes native text + 1-page Holter scan)
    // so the preloaded briefing run exercises `next` continuations and unrecoverable location fallback.
    let long_discharge = "Hospital Discharge Summary (2026-10-25): Admitted for transient palpitations; telemetry remained in normal sinus rhythm.\n".repeat(40)
        + "Discharge medication: metoprolol succinate 25 mg oral tablet once daily.";
    store
        .record_source(&make_source(SourceFixture {
            workspace: &workspace,
            source_id: "ehr-hospital",
            candidate_id: "cand-discharge-packet",
            revision: "rev-ds-2026-10-25",
            record_type: "DocumentReference",
            recorded_date: "2026-10-25",
            content: &long_discharge,
            scans: vec![ScanDocument {
                scan_id: "scan-holter-1".into(),
                label: "24-hour Holter summary sheet".into(),
                content: ScanDocumentState::Pages {
                    pages: vec![ScanPageEntry {
                        page: PageNumber::FIRST,
                        state: PartState::Readable {
                            text: "Holter Page 1 (2026-10-25): 0 pauses, sinus rhythm 58-92 bpm, <1% isolated PACs.".into(),
                        },
                    }],
                },
            }],
        }))
        .await
        .unwrap();

    // Seed an earlier conversation and durable patient memory so `search_patient_information`
    // discovers leads across all three domains (`chart_records`, `earlier_work`, `patient_memory`).
    let prior_chat_scope = Scope {
        practice: practice.clone(),
        physician: "dr-lin".into(),
        workspace: workspace.clone(),
        chat: "chat-prior-holter".into(),
    };
    let review_recipe = job_recipe(&binding, PhysicianJob::PatientReview, chords);
    assert!(
        review_recipe
            .preloaded_skills
            .contains(&SkillTarget::body("patient-record-retrieval")),
        "PatientReview must preload patient-record-retrieval"
    );
    assert_eq!(
        store
            .accept_authorized(
                &prior_chat_scope,
                &Input {
                    key: "prior-msg-1".into(),
                    text: "Remember to verify the Holter scan in her 2026-10-25 discharge packet before the follow-up visit.".into(),
                    timezone: "America/Los_Angeles".into(),
                    scenario: None,
                },
                session.to_vec(),
                chords,
                &review_recipe,
            )
            .await
            .unwrap(),
        Accepted::New
    );
    let prior_claim = store.claim(&prior_chat_scope, 60).await.unwrap().unwrap();
    let mut prior_provider = ScriptedProvider::new(vec![
        call_turn(vec![Invocation {
            id: "seed-holter-mem".into(),
            name: SAVE_MEMORY_TOOL.into(),
            arguments: json!({
                "upsert": [{
                    "topic": "arrhythmia_workup",
                    "note": "Hospitalized 2026-10-25 for palpitations; verify Holter scan in discharge packet (`cand-discharge-packet`)."
                }],
                "remove": []
            })
            .to_string(),
        }]),
        assistant_turn("Noted Holter follow-up for upcoming visit."),
    ]);
    PhysicianHost {
        store: store.clone(),
        claim: prior_claim,
        authority: AllowSession::single(&prior_chat_scope),
    }
    .run(
        &mut prior_provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // (a) Preloaded `VisitBriefing` run: starts with `patient-record-retrieval` preloaded,
    //     searches across all domains, reads a multi-part record via `next` continuation, and
    //     observes an explicit `location_limit` when requesting out-of-range page 5.
    let briefing_scope = Scope {
        practice: practice.clone(),
        physician: "dr-lin".into(),
        workspace: workspace.clone(),
        chat: "job-visit-briefing-ac1".into(),
    };
    let briefing_recipe = job_recipe(&binding, PhysicianJob::VisitBriefing, chords);
    assert!(
        briefing_recipe
            .instructions
            .contains("# Patient Record Retrieval"),
        "VisitBriefing must include preloaded patient-record-retrieval in initial context"
    );
    store
        .accept_authorized(
            &briefing_scope,
            &Input {
                key: "briefing-ac1-1".into(),
                text: "Prepare pre-visit briefing for post-hospitalization palpitations follow-up."
                    .into(),
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
                id: "brief-search-all".into(),
                name: SEARCH_TOOL.into(),
                arguments: json!({
                    "query": "Holter",
                    "record_type": null,
                    "bucket": null,
                    "domain": "all",
                    "standing": "current_only",
                    "after": null
                })
                .to_string(),
            },
            // Open whole multi-part discharge packet under a 4,096-byte budget (triggers `next` continuation),
            // and test unrecoverable scan page 5 (triggers `location_limit` fallback to verified source start).
            Invocation {
                id: "brief-read-first".into(),
                name: READ_TOOL.into(),
                arguments: json!({
                    "references": [
                        {
                            "candidate_id": "cand-discharge-packet",
                            "revision": "rev-ds-2026-10-25",
                            "cursor": null,
                            "source_id": "ehr-hospital",
                            "location": { "kind": "whole" }
                        },
                        {
                            "candidate_id": "cand-discharge-packet",
                            "revision": "rev-ds-2026-10-25",
                            "cursor": null,
                            "source_id": "ehr-hospital",
                            "location": {
                                "kind": "scan_page",
                                "scan": { "scope": "named", "scan_id": "scan-holter-1" },
                                "page": 5,
                                "slice": { "slice": "full" }
                            }
                        }
                    ],
                    "max_result_bytes": 16384
                })
                .to_string(),
            },
        ]),
        // Read exact Holter scan page 1 and finish the briefing.
        call_turn(vec![Invocation {
            id: "brief-read-holter-p1".into(),
            name: READ_TOOL.into(),
            arguments: json!({
                "references": [{
                    "candidate_id": "cand-discharge-packet",
                    "revision": "rev-ds-2026-10-25",
                    "cursor": null,
                    "source_id": "ehr-hospital",
                    "location": {
                        "kind": "scan_page",
                        "scan": { "scope": "named", "scan_id": "scan-holter-1" },
                        "page": 1,
                        "slice": { "slice": "full" }
                    }
                }],
                "max_result_bytes": 16384
            })
            .to_string(),
        }]),
        assistant_turn(
            "Post-discharge follow-up after 2026-10-25 hospitalization for palpitations: telemetry and 24-hour Holter page 1 showed sinus rhythm (58-92 bpm) with zero pauses and <1% PACs; discharged on metoprolol succinate 25 mg daily.\n\nBasis: 2026-10-25 Hospital Discharge Summary and attached Holter scan page 1 (`cand-discharge-packet`, `rev-ds-2026-10-25`).",
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

    let saved_briefing = store.read(&briefing_scope).await.unwrap().unwrap();
    let frozen_briefing_skills = store
        .frozen_skills(&briefing_scope, &saved_briefing.skills)
        .await
        .unwrap();
    let frozen_briefing_binding = store
        .frozen_binding(
            &briefing_scope,
            saved_briefing.bindings.get(&saved_briefing.work).unwrap(),
        )
        .await
        .unwrap();
    let briefing_trace = frozen_briefing_skills
        .trace_run(
            &frozen_briefing_binding.recipe,
            &saved_briefing.attempts[0].events,
        )
        .unwrap();
    assert!(
        briefing_trace.loads.contains(&SkillLoadRecord {
            target: SkillTarget::body("patient-record-retrieval"),
            origin: SkillLoadOrigin::Preloaded,
            version_id: expected_retrieval_version.clone(),
        }),
        "VisitBriefing run trace must record preloaded patient-record-retrieval SHA-256 version"
    );

    let briefing_calls = extract_tool_calls(&saved_briefing.attempts[0].events);
    // Unified search returned hits across chart_records, earlier_work, and patient_memory without body leaks.
    let search_json: Value = serde_json::from_str(
        briefing_calls
            .iter()
            .find(|c| c.call_id == "brief-search-all")
            .unwrap()
            .result
            .original()
            .unwrap(),
    )
    .unwrap();
    let domains: BTreeSet<&str> = search_json["results"]
        .as_array()
        .unwrap()
        .iter()
        .map(|hit| hit["domain"].as_str().unwrap())
        .collect();
    assert_eq!(
        domains,
        BTreeSet::from(["chart_record", "earlier_work", "patient_memory"])
    );

    // Out-of-range page 5 produced an explicit `location_limit` while returning the verified source start,
    // and valid page 1 delivered exact Holter scan text.
    let first_read_json: Value = serde_json::from_str(
        briefing_calls
            .iter()
            .find(|c| c.call_id == "brief-read-first")
            .unwrap()
            .result
            .original()
            .unwrap(),
    )
    .unwrap();
    assert!(first_read_json["results"][0]["next"].is_object());
    assert!(
        first_read_json["results"][1]["location_limit"]
            .as_str()
            .unwrap()
            .contains("page 5"),
        "expected explicit location_limit on unrecoverable scan page 5"
    );
    let holter_p1_json: Value = serde_json::from_str(
        briefing_calls
            .iter()
            .find(|c| c.call_id == "brief-read-holter-p1")
            .unwrap()
            .result
            .original()
            .unwrap(),
    )
    .unwrap();
    assert!(
        holter_p1_json["results"][0]["content"]
            .as_str()
            .unwrap()
            .contains("0 pauses, sinus rhythm 58-92 bpm")
    );

    // (b) Discovered `Conversation` run: starts small without `patient-record-retrieval` body,
    //     discovers it via `skill_view`, and records exact `version_id` in the durable run trace.
    let chat_scope = Scope {
        practice: practice.clone(),
        physician: "dr-lin".into(),
        workspace: workspace.clone(),
        chat: "chat-discover-retrieval".into(),
    };
    let chat_recipe = job_recipe(&binding, PhysicianJob::Conversation, chords);
    assert!(chat_recipe.preloaded_skills.is_empty());
    assert!(
        !chat_recipe
            .instructions
            .contains("# Patient Record Retrieval"),
        "Conversation must keep patient-record-retrieval out of initial window until opened"
    );
    store
        .accept_authorized(
            &chat_scope,
            &Input {
                key: "chat-discover-1".into(),
                text: "What did the Holter monitor in her October discharge packet show?".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            session.to_vec(),
            chords,
            &chat_recipe,
        )
        .await
        .unwrap();
    let chat_claim = store.claim(&chat_scope, 60).await.unwrap().unwrap();
    let mut chat_provider = ScriptedProvider::new(vec![
        call_turn(vec![
            Invocation {
                id: "chat-open-retrieval".into(),
                name: SKILL_VIEW.into(),
                arguments: json!({ "name": "patient-record-retrieval" }).to_string(),
            },
            Invocation {
                id: "chat-read-holter".into(),
                name: READ_TOOL.into(),
                arguments: json!({
                    "references": [{
                        "candidate_id": "cand-discharge-packet",
                        "revision": "rev-ds-2026-10-25",
                        "cursor": null,
                        "source_id": "ehr-hospital",
                        "location": {
                            "kind": "scan_page",
                            "scan": { "scope": "primary" },
                            "page": 1,
                            "slice": { "slice": "full" }
                        }
                    }],
                    "max_result_bytes": 16384
                })
                .to_string(),
            },
        ]),
        assistant_turn(
            "Her 2026-10-25 24-hour Holter summary (page 1) showed sinus rhythm at 58-92 bpm with zero pauses and <1% isolated PACs.\n\nBasis: Attached Holter scan page 1 on `cand-discharge-packet` (`rev-ds-2026-10-25`).",
        ),
    ]);
    PhysicianHost {
        store: store.clone(),
        claim: chat_claim,
        authority: AllowSession::single(&chat_scope),
    }
    .run(
        &mut chat_provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let saved_chat = store.read(&chat_scope).await.unwrap().unwrap();
    let frozen_chat_skills = store
        .frozen_skills(&chat_scope, &saved_chat.skills)
        .await
        .unwrap();
    let frozen_chat_binding = store
        .frozen_binding(
            &chat_scope,
            saved_chat.bindings.get(&saved_chat.work).unwrap(),
        )
        .await
        .unwrap();
    let chat_trace = frozen_chat_skills
        .trace_run(&frozen_chat_binding.recipe, &saved_chat.attempts[0].events)
        .unwrap();
    assert_eq!(
        chat_trace.loads,
        vec![SkillLoadRecord {
            target: SkillTarget::body("patient-record-retrieval"),
            origin: SkillLoadOrigin::Discovered {
                call_id: "chat-open-retrieval".into(),
            },
            version_id: expected_retrieval_version,
        }]
    );
}

#[tokio::test]
async fn ac2_chart_with_correction_and_unreadable_record_uses_corrected_readable_evidence_and_names_unreadable_record()
 {
    let Some(pg) = DisposablePostgres::acquire("eng250-ac2").await else {
        return;
    };
    let store = pg.store().await;

    let practice = format!("prac-{}", Uuid::new_v4().simple());
    let workspace = "pw-eng250-ac2".to_owned();
    let session = [7_u8; 32];
    let binding = fixture_binding("eng250-ac2");

    // 1. Ingest initial erroneous hemolyzed potassium result (`6.7 mmol/L`), then its corrected
    //    revision (`4.3 mmol/L`) under the same `cand-lab-potassium` candidate.
    store
        .record_source(&make_source(SourceFixture {
            workspace: &workspace,
            source_id: "ehr-quest",
            candidate_id: "cand-lab-potassium",
            revision: "rev-k-hemolyzed-1",
            record_type: "Observation",
            recorded_date: "2026-11-09",
            content: "Preliminary Metabolic Panel (2026-11-09 08:15): Serum Potassium 6.7 mmol/L — hemolyzed specimen; repeat redraw ordered.",
            scans: Vec::new(),
        }))
        .await
        .unwrap();
    store
        .record_source(&make_source(SourceFixture {
            workspace: &workspace,
            source_id: "ehr-quest",
            candidate_id: "cand-lab-potassium",
            revision: "rev-k-corrected-2",
            record_type: "Observation",
            recorded_date: "2026-11-09",
            content: "Corrected Metabolic Panel (2026-11-09 11:40): Serum Potassium 4.3 mmol/L, Creatinine 0.96 mg/dL — non-hemolyzed redraw replacing preliminary hemolyzed result.",
            scans: Vec::new(),
        }))
        .await
        .unwrap();

    // 2. Ingest a readable nephrology consult note referencing both the repeat potassium and an outside urgent-care ECG fax.
    store
        .record_source(&make_source(SourceFixture {
            workspace: &workspace,
            source_id: "ehr-athena",
            candidate_id: "cand-note-nephrology",
            revision: "rev-neph-2026-11-10",
            record_type: "DocumentReference",
            recorded_date: "2026-11-10",
            content: "Signed Nephrology Consult (2026-11-10): Repeat non-hemolyzed potassium normal at 4.3 mmol/L with creatinine 0.96 mg/dL; continue losartan 50 mg once daily. Outside urgent-care 12-lead ECG fax filed under `cand-fax-urgent-ecg`.",
            scans: Vec::new(),
        }))
        .await
        .unwrap();

    // 3. Ingest an outside urgent-care 12-lead ECG fax whose scanned page is UNREADABLE due to failed OCR / blacked-out thermal fax.
    store
        .record_source(&make_source(SourceFixture {
            workspace: &workspace,
            source_id: "fax-spruce",
            candidate_id: "cand-fax-urgent-ecg",
            revision: "rev-fax-2026-11-09",
            record_type: "DocumentReference",
            recorded_date: "2026-11-09",
            content: "Outside Urgent Care Fax Envelope (2026-11-09): 1-page scanned 12-lead ECG transmission attached.",
            scans: vec![ScanDocument {
                scan_id: "scan-urgent-ecg-1".into(),
                label: "Outside urgent-care 12-lead ECG fax".into(),
                content: ScanDocumentState::Pages {
                    pages: vec![ScanPageEntry {
                        page: PageNumber::FIRST,
                        state: PartState::Unreadable {
                            reason: "OCR failed: thermal fax page completely blacked out and illegible".into(),
                        },
                    }],
                },
            }],
        }))
        .await
        .unwrap();

    // Run physician conversation loading `patient-record-retrieval`, discovering current and historical
    // standing via `search_patient_information`, and reading both the corrected/readable records and the
    // unreadable ECG scan via `read_patient_sources`.
    let scope = Scope {
        practice: practice.clone(),
        physician: "dr-patel".into(),
        workspace: workspace.clone(),
        chat: "chat-potassium-and-ecg".into(),
    };
    let chat_recipe = job_recipe(&binding, PhysicianJob::Conversation, skills());
    assert_eq!(
        store
            .accept_authorized(
                &scope,
                &Input {
                    key: "ask-k-ecg-1".into(),
                    text: "Did his repeat potassium clear continuing losartan, and what did nephrology and the outside urgent-care ECG fax show?".into(),
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
    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let mut provider = ScriptedProvider::new(vec![
        // Turn 1: Open `patient-record-retrieval` + communication guidance and search chart records
        // including historical standing so any corrected/superseded lab is visible with its replacement.
        call_turn(vec![
            Invocation {
                id: "ac2-open-retrieval".into(),
                name: SKILL_VIEW.into(),
                arguments: json!({ "name": "patient-record-retrieval" }).to_string(),
            },
            Invocation {
                id: "ac2-open-shared".into(),
                name: SKILL_VIEW.into(),
                arguments: json!({ "name": "clinical-communication" }).to_string(),
            },
            Invocation {
                id: "ac2-open-phys".into(),
                name: SKILL_VIEW.into(),
                arguments: json!({ "name": "physician-clinical-communication" }).to_string(),
            },
            Invocation {
                id: "ac2-search-chart".into(),
                name: SEARCH_TOOL.into(),
                arguments: json!({
                    "query": "",
                    "record_type": null,
                    "bucket": null,
                    "domain": "chart_records",
                    "standing": "current_and_historical",
                    "after": null
                })
                .to_string(),
            },
        ]),
        // Turn 2: Open current corrected potassium (`rev-k-corrected-2`), verify superseded status on
        // `rev-k-hemolyzed-1`, read the signed nephrology consult (`rev-neph-2026-11-10`), and open the
        // multi-part outside urgent-care ECG fax (`cand-fax-urgent-ecg`) with `location: { "kind": "whole" }`.
        call_turn(vec![Invocation {
            id: "ac2-read-evidence".into(),
            name: READ_TOOL.into(),
            arguments: json!({
                "references": [
                    {
                        "candidate_id": "cand-lab-potassium",
                        "revision": "rev-k-corrected-2",
                        "cursor": null
                    },
                    {
                        "candidate_id": "cand-lab-potassium",
                        "revision": "rev-k-hemolyzed-1",
                        "cursor": null
                    },
                    {
                        "candidate_id": "cand-note-nephrology",
                        "revision": "rev-neph-2026-11-10",
                        "cursor": null
                    },
                    {
                        "candidate_id": "cand-fax-urgent-ecg",
                        "revision": "rev-fax-2026-11-09",
                        "cursor": null,
                        "source_id": "fax-spruce",
                        "location": {
                            "kind": "whole"
                        }
                    }
                ],
                "max_result_bytes": 32768
            })
            .to_string(),
        }]),
        // Turn 3: Follow the `next` continuation reference on `cand-fax-urgent-ecg` to inspect the attached
        // scanned 12-lead ECG page (`part_ordinal: 1`), which delivers `outcome: "unreadable_part"`.
        call_turn(vec![Invocation {
            id: "ac2-read-fax-scan".into(),
            name: READ_TOOL.into(),
            arguments: json!({
                "references": [
                    {
                        "candidate_id": "cand-fax-urgent-ecg",
                        "revision": "rev-fax-2026-11-09",
                        "cursor": null,
                        "source_id": "fax-spruce",
                        "location": {
                            "kind": "continuation",
                            "source_id": "fax-spruce",
                            "candidate_id": "cand-fax-urgent-ecg",
                            "revision": "rev-fax-2026-11-09",
                            "part_ordinal": 1,
                            "byte_offset": 0,
                            "scope": { "scope": "document" }
                        }
                    }
                ],
                "max_result_bytes": 16384
            })
            .to_string(),
        }]),
        // Turn 4: Grounded clinical answer using readable current records (4.3 mmol/L + nephrology note),
        // rejecting the superseded 6.7 mmol/L hemolyzed value as current truth, and naming the unreadable ECG fax.
        assistant_turn(
            "Yes — his corrected non-hemolyzed redraw on 2026-11-09 showed a normal potassium of 4.3 mmol/L and creatinine of 0.96 mg/dL (superseding the earlier hemolyzed preliminary 6.7 mmol/L), and nephrology's 2026-11-10 consult recommends continuing losartan 50 mg once daily. However, the outside urgent-care 12-lead ECG fax from 2026-11-09 (`cand-fax-urgent-ecg`, page 1) could not be read because the thermal fax page is blacked out and OCR failed, so ECG findings remain unverified.\n\nBasis: Corrected 2026-11-09 metabolic panel (`cand-lab-potassium`, `rev-k-corrected-2`) and 2026-11-10 nephrology consult (`cand-note-nephrology`, `rev-neph-2026-11-10`); outside 2026-11-09 urgent-care ECG fax (`cand-fax-urgent-ecg`, `scan-urgent-ecg-1` page 1) was unreadable.",
        ),
    ]);
    PhysicianHost {
        store: store.clone(),
        claim,
        authority: AllowSession::single(&scope),
    }
    .run(
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Verify Turn 2 received the exact `patient-record-retrieval` body in context.
    assert!(provider.seen_requests[1].items.iter().any(|item| {
        matches!(
            item,
            Item::MaterializedResult { call_id, output }
                if call_id == "ac2-open-retrieval" && output.contains("# Patient Record Retrieval")
        )
    }));

    let saved = store.read(&scope).await.unwrap().unwrap();
    assert_eq!(saved.status, WorkStatus::Completed);
    let calls = extract_tool_calls(&saved.attempts[0].events);

    // 1. Verify search inputs and delivered standing metadata: current corrected revision vs superseded error,
    //    with zero record body leakage in search output.
    let search_call = calls
        .iter()
        .find(|c| c.call_id == "ac2-search-chart")
        .expect("search call recorded");
    assert_eq!(search_call.tool_name, SEARCH_TOOL);
    assert_eq!(search_call.arguments["standing"], "current_and_historical");
    let search_raw = search_call.result.original().expect("search executed");
    assert!(!search_raw.contains("4.3 mmol/L"));
    assert!(!search_raw.contains("6.7 mmol/L"));
    let search_page: Value = serde_json::from_str(search_raw).unwrap();
    let hits = search_page["results"].as_array().unwrap();

    let current_k_hit = hits
        .iter()
        .find(|h| h["reference"]["revision"] == "rev-k-corrected-2")
        .expect("current corrected potassium hit present");
    assert_eq!(current_k_hit["standing"]["status"], "current");
    assert_eq!(current_k_hit["bucket"], "results");

    let superseded_k_hit = hits
        .iter()
        .find(|h| h["reference"]["revision"] == "rev-k-hemolyzed-1")
        .expect("superseded hemolyzed potassium hit present");
    assert_eq!(superseded_k_hit["standing"]["status"], "superseded");
    assert_eq!(
        superseded_k_hit["standing"]["replaced_by"],
        "rev-k-corrected-2"
    );

    // 2. Verify delivered `read_patient_sources` evidence across initial batch and `next` continuation:
    //    - Corrected potassium (`rev-k-corrected-2`) is `available` without `superseded_by`
    //    - Erroneous potassium (`rev-k-hemolyzed-1`) carries `superseded_by: "rev-k-corrected-2"`
    //    - Nephrology note (`rev-neph-2026-11-10`) is `available`
    //    - Outside ECG fax (`rev-fax-2026-11-09`) delivers envelope `passage` + `next` cursor in Turn 2,
    //      and `unreadable_part` with scan identity and reason in Turn 3!
    let read_call = calls
        .iter()
        .find(|c| c.call_id == "ac2-read-evidence")
        .expect("read_patient_sources call recorded");
    assert_eq!(read_call.tool_name, READ_TOOL);
    let read_json: Value =
        serde_json::from_str(read_call.result.original().expect("read executed")).unwrap();
    let results = read_json["results"].as_array().unwrap();
    assert_eq!(results.len(), 4);

    assert_eq!(results[0]["outcome"], "available");
    assert_eq!(results[0]["revision"], "rev-k-corrected-2");
    assert!(results[0].get("superseded_by").is_none());
    assert!(
        results[0]["content"]
            .as_str()
            .unwrap()
            .contains("4.3 mmol/L")
    );

    assert_eq!(results[1]["outcome"], "available");
    assert_eq!(results[1]["revision"], "rev-k-hemolyzed-1");
    assert_eq!(results[1]["superseded_by"], "rev-k-corrected-2");

    assert_eq!(results[2]["outcome"], "available");
    assert_eq!(results[2]["revision"], "rev-neph-2026-11-10");
    assert!(
        results[2]["content"]
            .as_str()
            .unwrap()
            .contains("continue losartan 50 mg once daily")
    );

    assert_eq!(results[3]["outcome"], "passage");
    assert_eq!(results[3]["reference"], "cand-fax-urgent-ecg");
    assert_eq!(results[3]["revision"], "rev-fax-2026-11-09");
    let fax_next = &results[3]["next"];
    assert!(
        fax_next.is_object(),
        "whole-record read emits next continuation to scan page"
    );

    let fax_scan_call = calls
        .iter()
        .find(|c| c.call_id == "ac2-read-fax-scan")
        .expect("continuation read_patient_sources call recorded");
    assert_eq!(fax_scan_call.tool_name, READ_TOOL);
    assert_eq!(&fax_scan_call.arguments["references"][0], fax_next);
    let fax_scan_json: Value = serde_json::from_str(
        fax_scan_call
            .result
            .original()
            .expect("fax scan read executed"),
    )
    .unwrap();
    let fax_scan_results = fax_scan_json["results"].as_array().unwrap();
    assert_eq!(fax_scan_results.len(), 1);
    assert_eq!(fax_scan_results[0]["outcome"], "unreadable_part");
    assert_eq!(fax_scan_results[0]["reference"], "cand-fax-urgent-ecg");
    assert_eq!(fax_scan_results[0]["revision"], "rev-fax-2026-11-09");
    assert_eq!(fax_scan_results[0]["part_identity"]["kind"], "scan_page");
    assert_eq!(
        fax_scan_results[0]["part_identity"]["scan_id"],
        "scan-urgent-ecg-1"
    );
    assert_eq!(fax_scan_results[0]["part_identity"]["page"], 1);
    assert!(
        fax_scan_results[0]["reason"]
            .as_str()
            .unwrap()
            .contains("blacked out and illegible")
    );

    // 3. Verify grounded output: uses current corrected 4.3 mmol/L + nephrology note,
    //    explains 6.7 mmol/L was superseded, and names the unreadable outside ECG fax.
    let published: Vec<&str> = saved
        .messages
        .iter()
        .filter(|m| matches!(m.role, MessageRole::Assistant { .. }))
        .map(|m| m.text.as_str())
        .collect();
    assert_eq!(published.len(), 1);
    let answer = published[0];
    assert!(answer.contains("4.3 mmol/L"));
    assert!(answer.contains("continue losartan") || answer.contains("continuing losartan"));
    assert!(answer.contains("superseding the earlier hemolyzed preliminary 6.7 mmol/L"));
    assert!(answer.contains("cand-fax-urgent-ecg"));
    assert!(answer.contains("could not be read"));
    assert!(answer.contains("unreadable"));
    assert!(!answer.contains("normal sinus") && !answer.contains("normal ECG"));
}
