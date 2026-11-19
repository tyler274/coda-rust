//! PostgreSQL and Responses-transport integration tests for the patient
//! follow-up reassessment recipe, clinical library discovery, longitudinal
//! follow-up coherence, and due / physician-triggered dispatch.
//!
//! Runs PostgreSQL tests automatically whenever `PHYSICIAN_RUNTIME_TEST_DATABASE_URL`
//! or `PHYSICIAN_DATABASE_URL` is set; panics if neither is set when
//! `CODA_REQUIRE_POSTGRES_TESTS=1`.

use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::{
    Json, Router,
    extract::State,
    http::{StatusCode, header},
    response::IntoResponse,
    routing::post,
};
use harness::provider::{QuotaGate, ResponsesProvider};
use local_product::clinical_library::{
    ClinicalLibraryClient, READ_LIBRARY_TOOL, SEARCH_LIBRARY_TOOL, SearchWireResponse,
    WireCandidate, WireEvidenceUnit, WireIdentifier, WireLifecycleState, WireReadUnavailableCause,
    WireSearchPage,
};
use local_product::follow_up_reassessment::{
    COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL, FOLLOW_UP_REASSESSMENT_GUIDANCE_VERSION,
    FOLLOW_UP_REASSESSMENT_INSTRUCTIONS, FOLLOW_UP_REASSESSMENT_RECIPE_VERSION,
    FollowUpReassessmentInitialLoad, FollowUpReassessmentRequest, FollowUpReassessmentTrigger,
    follow_up_reassessment_instructions_sha256, recipe as follow_up_reassessment_recipe,
    run_follow_up_reassessment,
};
use local_product::message_review_recipe::{
    MessageReviewRunStatus, RecordedToolDiscovery, ToolDiscoveryDisposition,
};
use local_product::physician_chat::{PhysicianSkills, SKILL_VIEW};
use local_product::physician_sources::{
    MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SourcePatient, SourceRevision,
    workspace_id,
};
use local_product::runtime::concerns::{ExactRecordReference, WorkspaceAccess};
use local_product::runtime::follow_ups::{
    FollowUpChangeNote, FollowUpCommandKey, FollowUpId, FollowUpSummary, FollowUpUpdateKind,
    PhysicianChangeFollowUp, PhysicianCreateFollowUp,
};
use local_product::runtime::jobs::is_physician_only_tool;
use local_product::runtime::sources::SourcePull;
use local_product::runtime::{
    Authority, PublicationTransaction, Scope, Store, async_trait, engine,
};
use local_product::{ModelBinding, ModelRoles, sha256_hex};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use serde_json::{Value, json};
use uuid::Uuid;

#[allow(dead_code)]
#[path = "../../../verification/model_text.rs"]
mod model_text;

const LIBRARY_UNIT_ID: &str = "22222222-2222-4222-8222-222222222399";

fn database_url() -> Option<String> {
    if let Ok(url) = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
        .or_else(|_| std::env::var("PHYSICIAN_DATABASE_URL"))
    {
        let trimmed = url.trim().to_owned();
        if !trimmed.is_empty() {
            return Some(trimmed);
        }
    }
    assert!(
        std::env::var("CODA_REQUIRE_POSTGRES_TESTS").as_deref() != Ok("1"),
        "CODA_REQUIRE_POSTGRES_TESTS=1 requires PHYSICIAN_RUNTIME_TEST_DATABASE_URL or PHYSICIAN_DATABASE_URL"
    );
    None
}

fn load_skills() -> PhysicianSkills {
    PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
        .expect("load workspace skill catalog")
}

fn distinct_roles() -> ModelRoles {
    ModelRoles {
        chat: ModelBinding {
            model: "gpt-5.4-chat-interactive".into(),
            effort: "low".into(),
            context_window_tokens: 160_000,
            max_input_tokens: 128_000,
            adapter: "responses".into(),
        },
        background: ModelBinding {
            model: "gpt-5.4-pro-background-followup".into(),
            effort: "high".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 240_000,
            adapter: "responses".into(),
        },
    }
}

struct NoopQuotaGate;

fn current_unix_ts() -> i64 {
    i64::try_from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock after unix epoch")
            .as_secs(),
    )
    .expect("unix timestamp fits in i64")
}

#[async_trait]
impl QuotaGate for NoopQuotaGate {
    async fn remaining(&self) -> Result<Duration, engine::Cause> {
        Ok(Duration::ZERO)
    }

    async fn defer(&self, _delay: Duration) -> Result<(), engine::Cause> {
        Ok(())
    }
}

struct PostgresWorkspaceAuthority;

#[async_trait]
impl Authority for PostgresWorkspaceAuthority {
    async fn check(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        PostgresAuthStore::check_runtime_authority(
            tx,
            session_hash.ok_or("missing session reference")?,
            &PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|cause| cause.to_string())
    }

    async fn lock_publication(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        PostgresAuthStore::lock_runtime_authority(
            tx,
            session_hash.ok_or("missing session reference")?,
            &PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|cause| cause.to_string())
    }
}

#[derive(Clone)]
struct SseServerState {
    turns: Arc<Mutex<VecDeque<Vec<Value>>>>,
    captured_requests: Arc<Mutex<Vec<Value>>>,
}

fn sse_function_call(call_id: &str, name: &str, arguments: &Value) -> Value {
    json!({
        "type": "function_call",
        "call_id": call_id,
        "name": name,
        "arguments": arguments.to_string(),
        "status": "completed",
    })
}

fn sse_assistant_message(text: &str) -> Value {
    json!({
        "type": "message",
        "role": "assistant",
        "status": "completed",
        "content": [{
            "type": "output_text",
            "text": text,
        }],
    })
}

async fn spawn_responses_sse_server(
    turns: Vec<Vec<Value>>,
) -> (ResponsesProvider, Arc<Mutex<Vec<Value>>>) {
    let captured = Arc::new(Mutex::new(Vec::new()));
    let state = SseServerState {
        turns: Arc::new(Mutex::new(VecDeque::from(turns))),
        captured_requests: captured.clone(),
    };

    async fn handle_responses(
        State(state): State<SseServerState>,
        Json(body): Json<Value>,
    ) -> impl IntoResponse {
        state.captured_requests.lock().unwrap().push(body);
        let items = state
            .turns
            .lock()
            .unwrap()
            .pop_front()
            .expect("SSE server received more turns than scripted");
        let resp_id = format!("resp_{}", Uuid::new_v4().simple());
        let mut sse = String::new();
        let created = json!({
            "type": "response.created",
            "response": {
                "id": resp_id,
                "status": "in_progress",
            }
        });
        sse.push_str(&format!("data: {created}\n\n"));
        for (idx, item) in items.into_iter().enumerate() {
            let done = json!({
                "type": "response.output_item.done",
                "output_index": idx,
                "item": item,
            });
            sse.push_str(&format!("data: {done}\n\n"));
        }
        let completed = json!({
            "type": "response.completed",
            "response": {
                "id": resp_id,
                "status": "completed",
                "usage": {
                    "input_tokens": 220,
                    "output_tokens": 90,
                    "total_tokens": 310,
                }
            }
        });
        sse.push_str(&format!("data: {completed}\n\n"));
        sse.push_str("data: [DONE]\n\n");

        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/event-stream")],
            sse,
        )
    }

    let app = Router::new()
        .route("/v1/responses", post(handle_responses))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    let provider = ResponsesProvider::new(
        format!("http://{addr}/v1/responses"),
        "synthetic-test-key".into(),
        Arc::new(NoopQuotaGate),
    )
    .expect("construct local ResponsesProvider");

    (provider, captured)
}

async fn spawn_clinical_library_server() -> ClinicalLibraryClient {
    let content = "Recheck serum potassium and renal function within 7 to 14 days after initiating or titrating mineralocorticoid receptor antagonists or ACE inhibitors.";
    let unit = WireEvidenceUnit {
        bibliographic_observations: None,
        unit_id: LIBRARY_UNIT_ID.into(),
        version_id: "22222222-2222-4222-8222-000000000399".into(),
        source_code: "dailymed".into(),
        source_display_name: "DailyMed".into(),
        native_document_id: "raas-potassium-surveillance-2026".into(),
        native_version_id: Some("v1".into()),
        source_version_label: None,
        title: "Potassium and Renal Surveillance on RAAS Blockade".into(),
        document_type: "clinical_guidance".into(),
        publication_date: Some("2026-02-10".into()),
        version_date: Some("2026-02-10".into()),
        acquired_at: "2026-11-10T00:00:00Z".into(),
        lifecycle_state: WireLifecycleState::Current,
        source_status_code: None,
        native_unit_id: "sec-2".into(),
        native_locator: "Section 2".into(),
        unit_kind: "section".into(),
        ordinal: 0,
        label: Some("Laboratory Monitoring Interval".into()),
        content: content.into(),
        content_sha256: sha256_hex(content.as_bytes()),
        native_destination_uri: "https://clinical.example.org/raas-potassium-surveillance-2026"
            .into(),
        version_destination_uri: "https://clinical.example.org/raas-potassium-surveillance-2026/v1"
            .into(),
        rights_decision_id: "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb".into(),
        license_identifier: Some("public-domain".into()),
        attribution_text: Some("DailyMed Clinical Monograph 2026; public domain".into()),
        may_display_excerpt: true,
        identifiers: vec![WireIdentifier {
            scheme: "native".into(),
            raw_value: "raas-potassium-surveillance-2026".into(),
            normalized_value: Some("raas-potassium-surveillance-2026".into()),
        }],
        relationships: vec![],
    };

    #[derive(Clone)]
    struct LibState {
        unit: WireEvidenceUnit,
    }

    async fn search(State(state): State<LibState>) -> impl IntoResponse {
        (
            StatusCode::OK,
            Json(SearchWireResponse {
                candidates: vec![WireCandidate {
                    candidate_id: state.unit.unit_id.clone(),
                    source_code: state.unit.source_code.clone(),
                    source_display_name: state.unit.source_display_name.clone(),
                    native_document_id: state.unit.native_document_id.clone(),
                    title: state.unit.title.clone(),
                }],
                page: WireSearchPage {
                    examined_candidate_count: 1,
                    returned_candidate_count: 1,
                    unavailable_candidate_count: 0,
                    truncated: false,
                },
                next_cursor: None,
            }),
        )
    }

    async fn read(State(state): State<LibState>, Json(body): Json<Value>) -> impl IntoResponse {
        let ids = body
            .get("candidate_ids")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let mut results = Vec::new();
        for raw_id in ids {
            let id = raw_id.as_str().unwrap_or_default();
            if id == state.unit.unit_id {
                results.push(json!({
                    "candidate_id": id,
                    "outcome": "available",
                    "unit": state.unit,
                }));
            } else {
                results.push(json!({
                    "candidate_id": id,
                    "outcome": "unavailable",
                    "cause": WireReadUnavailableCause::NotFoundOrNotAuthorized,
                }));
            }
        }
        (StatusCode::OK, Json(json!({ "results": results })))
    }

    let app = Router::new()
        .route("/v1/search", post(search))
        .route("/v1/read", post(read))
        .with_state(LibState { unit });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    ClinicalLibraryClient::http(format!("http://{addr}"))
        .unwrap()
        .with_bearer_token("test-clinical-library-token")
}

async fn connect_stores(url: &str) -> (Store, Arc<PostgresAuthStore>) {
    let store = Store::connect(url).await.expect("connect runtime store");
    store.migrate().await.expect("migrate runtime store");
    let auth = Arc::new(
        PostgresAuthStore::connect_with_migrations(url)
            .await
            .expect("connect and migrate auth store"),
    );
    (store, auth)
}

async fn invite_and_sign_in(
    auth: &PostgresAuthStore,
    practice: &str,
    label: &str,
    now_secs: u64,
) -> (PhysicianIdentity, String, Vec<u8>) {
    let email = format!("{label}-{}@codahealth.test", Uuid::new_v4().simple());
    let invite = auth
        .invite(practice, &email, label, false, now_secs)
        .await
        .unwrap();
    let (identity, token) = auth
        .redeem_invitation(&email, &invite, now_secs)
        .await
        .unwrap();
    let hash = PostgresAuthStore::session_reference(&token).to_vec();
    (identity, token, hash)
}

#[expect(clippy::too_many_arguments, reason = "test chart fixture builder")]
fn chart_record(
    workspace: &str,
    source_id: &str,
    candidate_id: &str,
    revision: &str,
    record_type: &str,
    date: &str,
    title: &str,
    text: &str,
) -> SourceRevision {
    let content = json!({
        "resourceType": record_type,
        "id": candidate_id,
        "date": date,
        "title": title,
        "summary": text,
    })
    .to_string();
    SourceRevision {
        workspace: workspace.to_owned(),
        source_id: source_id.to_owned(),
        candidate_id: candidate_id.to_owned(),
        revision: revision.to_owned(),
        record_type: record_type.to_owned(),
        dates: vec![RecordDate {
            name: "effectiveDate".into(),
            value: date.to_owned(),
        }],
        sha256: sha256_hex(content.as_bytes()),
        content,
        readable_parts: vec![ReadablePart {
            label: title.to_owned(),
            text: text.to_owned(),
        }],
        provenance: json!({
            "system": source_id,
            "id": candidate_id,
            "title": title,
        }),
        envelope: None,
    }
}

#[test]
fn recipe_guidance_and_tool_surface_are_clean_and_background_bound() {
    let skills = load_skills();
    let roles = distinct_roles();
    let (built, preloaded) = follow_up_reassessment_recipe(&roles, &skills).unwrap();
    assert_eq!(built.model, roles.background.model);
    assert_eq!(built.options, json!({ "effort": "high" }));
    assert_eq!(preloaded.len(), 2);
    assert_eq!(preloaded[0].name, "patient-follow-up");
    assert_eq!(preloaded[1].name, "patient-record-retrieval");

    model_text::assert_clean_model_text(&built.instructions);
    model_text::assert_clean_model_text(&serde_json::to_string(&built.tools).unwrap());

    for tool in &built.tools {
        let name = tool["name"].as_str().unwrap();
        assert!(
            !is_physician_only_tool(name),
            "follow-up reassessment must never offer physician-only tool `{name}`"
        );
    }
}

#[tokio::test]
async fn ac1_reassessment_run_carries_responsibility_origin_constraints_history_preloads_and_discovers_library()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (base_store, auth) = connect_stores(&url).await;
    let library_client = spawn_clinical_library_server().await;
    let store = base_store.with_clinical_library(library_client);

    let now_ts = current_unix_ts();
    let practice = format!("prac-fu-ac1-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, _token, dr_hash) =
        invite_and_sign_in(&auth, &practice, "dr-fu-ac1", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-fu-ac1-{}", Uuid::new_v4().simple()),
        display_name: "Evelyn Harper".into(),
        birth_date: Some("1956-03-14".into()),
        sex_or_gender: Some("female".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-9101".into(),
        }],
    };
    let workspace = workspace_id("clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let note = chart_record(
        &workspace,
        "clinic-ehr",
        "note-raas-1",
        "rev-1",
        "DocumentReference",
        "2026-11-01",
        "Heart Failure Titration Note",
        "Initiated spironolactone 25 mg daily; check basic metabolic panel in 10 to 14 days.",
    );
    store
        .record_chart(
            &workspace,
            &[note],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient: patient.clone(),
                pulled_at: Some(now_ts - 86_400),
            },
        )
        .await
        .unwrap();

    let access = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &workspace,
        session_hash: &dr_hash,
    };
    let fu_id = FollowUpId::new("fu-spironolactone-bmp").unwrap();
    store
        .create_follow_up_for_physician(
            &access,
            &FollowUpCommandKey::new("create-fu-ac1").unwrap(),
            &PhysicianCreateFollowUp {
                id: fu_id.clone(),
                summary: FollowUpSummary::new("Check BMP after starting spironolactone 25 mg")
                    .unwrap(),
                due_at: now_ts - 7_200,
                records: vec![ExactRecordReference::new("note-raas-1", "rev-1").unwrap()],
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    // Add one revision to `update_history` so we prove both `Created` and `Changed` history entries appear:
    store
        .change_follow_up_for_physician(
            &access,
            &FollowUpCommandKey::new("change-fu-ac1").unwrap(),
            &PhysicianChangeFollowUp {
                id: fu_id.clone(),
                summary: FollowUpSummary::new(
                    "Check serum potassium and creatinine 10-14 days after spironolactone 25 mg start",
                )
                .unwrap(),
                due_at: now_ts - 600,
                records: vec![ExactRecordReference::new("note-raas-1", "rev-1").unwrap()],
                note: FollowUpChangeNote::new(
                    "Clarified potassium + creatinine surveillance scope",
                )
                .unwrap(),
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    let expected_fu_sha =
        sha256_hex(include_str!("../../../skills/patient-follow-up/SKILL.md").as_bytes());
    let expected_retr_sha =
        sha256_hex(include_str!("../../../skills/patient-record-retrieval/SKILL.md").as_bytes());

    let (mut provider, captured_reqs) = spawn_responses_sse_server(vec![
        vec![
            sse_function_call(
                "lib-search-1",
                SEARCH_LIBRARY_TOOL,
                &json!({
                    "query": "spironolactone potassium renal monitoring interval",
                }),
            ),
            sse_function_call(
                "skill-comm-1",
                SKILL_VIEW,
                &json!({ "name": "patient-communication" }),
            ),
            sse_function_call(
                "read-note-1",
                READ_TOOL,
                &json!({
                    "references": [{ "candidate_id": "note-raas-1", "revision": "rev-1", "cursor": null }],
                    "max_result_bytes": 16_384,
                }),
            ),
        ],
        vec![sse_function_call(
            "lib-read-1",
            READ_LIBRARY_TOOL,
            &json!({
                "passage_id": LIBRARY_UNIT_ID,
            }),
        )],
        vec![sse_function_call(
            "complete-reassess-ac1",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "change_next_reassessment",
                    "summary": "Recheck overdue spironolactone potassium and creatinine panel",
                    "next_due_at": now_ts + 86_400 * 2,
                    "change_note": "Day-12 BMP not yet resulted; scheduled 48-hour follow-up check per RAAS surveillance guidance.",
                    "rationale": "Spironolactone 25 mg started 2026-11-01; DailyMed guidance recommends 7-14 day potassium/renal check and window remains open for 48 hours.",
                },
                "cited_records": [{ "candidate_id": "note-raas-1", "revision": "rev-1" }],
                "basis_and_limitations": "Reviewed 2026-11-01 titration note and DailyMed RAAS potassium surveillance guidance.",
                "explicit_uncertainty": ["Outpatient laboratory draw has not yet posted to Clinic EHR."],
                "unfinished_work": [],
            }),
        )],
        vec![sse_assistant_message(
            "Updated reassessment window with clinical library grounding.",
        )],
    ])
    .await;

    let skills = load_skills();
    let roles = distinct_roles();
    let completed = run_follow_up_reassessment(
        &store,
        auth.as_ref(),
        FollowUpReassessmentRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            follow_up_id: fu_id.clone(),
            trigger: FollowUpReassessmentTrigger::DueWakeup {
                clinical_now: now_ts,
            },
            review_id: Some("rev-fu-ac1".into()),
        },
        &roles,
        &skills,
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("complete AC1 follow-up reassessment run");

    let wire = captured_reqs.lock().unwrap().clone();
    assert_eq!(wire.len(), 4);
    assert_eq!(wire[0]["model"], roles.background.model);
    assert_eq!(wire[0]["store"], false);
    assert_eq!(
        wire[0]["reasoning"],
        json!({ "effort": "high", "summary": "auto" })
    );
    let wire_inst = wire[0]["instructions"].as_str().unwrap();
    assert!(wire_inst.starts_with(FOLLOW_UP_REASSESSMENT_INSTRUCTIONS));
    model_text::assert_clean_model_text(wire_inst);
    model_text::assert_clean_model_text(&completed.run.starting_context.render_prompt());

    // Verify starting_context carried responsibility, origin (`why_it_exists`), constraints, and update_history:
    assert_eq!(
        completed.run.starting_context.responsibility.follow_up.id,
        fu_id
    );
    assert_eq!(
        completed
            .run
            .starting_context
            .responsibility
            .follow_up
            .version,
        2
    );
    assert!(
        completed
            .run
            .starting_context
            .responsibility
            .why_it_exists
            .contains(&dr.physician_id)
    );
    assert_eq!(
        completed
            .run
            .starting_context
            .responsibility
            .constraints
            .resolution_policy,
        local_product::follow_up_reassessment::FollowUpResolutionPolicy::VerifiedEvidenceOnly
    );
    assert_eq!(
        completed
            .run
            .starting_context
            .responsibility
            .update_history
            .len(),
        2
    );
    assert_eq!(
        completed.run.starting_context.responsibility.update_history[0].kind,
        FollowUpUpdateKind::Created
    );
    assert_eq!(
        completed.run.starting_context.responsibility.update_history[1].kind,
        FollowUpUpdateKind::Changed
    );

    // Verify provenance, preloaded skills, and separate tool discoveries:
    assert_eq!(completed.run.status, MessageReviewRunStatus::Completed);
    assert_eq!(
        completed.run.provenance.recipe_version,
        FOLLOW_UP_REASSESSMENT_RECIPE_VERSION
    );
    assert_eq!(
        completed.run.provenance.guidance_version,
        FOLLOW_UP_REASSESSMENT_GUIDANCE_VERSION
    );
    assert_eq!(
        completed.run.provenance.instructions_sha256,
        follow_up_reassessment_instructions_sha256()
    );
    assert!(completed.run.initial_loads.contains(
        &FollowUpReassessmentInitialLoad::PreloadedSkill {
            name: "patient-follow-up".into(),
            sha256: expected_fu_sha,
        }
    ));
    assert!(completed.run.initial_loads.contains(
        &FollowUpReassessmentInitialLoad::PreloadedSkill {
            name: "patient-record-retrieval".into(),
            sha256: expected_retr_sha,
        }
    ));

    assert!(
        completed
            .run
            .tool_discoveries
            .iter()
            .any(|d: &RecordedToolDiscovery| {
                d.tool_name == SKILL_VIEW
                    && d.discovered_skill_name.as_deref() == Some("patient-communication")
                    && d.disposition == ToolDiscoveryDisposition::Returned
            })
    );
    for expected_tool in [SEARCH_LIBRARY_TOOL, READ_LIBRARY_TOOL, READ_TOOL] {
        assert!(
            completed
                .run
                .tool_discoveries
                .iter()
                .any(|d: &RecordedToolDiscovery| {
                    d.tool_name == expected_tool
                        && d.disposition == ToolDiscoveryDisposition::Returned
                }),
            "expected `{expected_tool}` in tool_discoveries: {:?}",
            completed.run.tool_discoveries
        );
    }
}
