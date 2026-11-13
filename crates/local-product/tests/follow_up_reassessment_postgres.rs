//! PostgreSQL and Responses-transport integration tests for the patient
//! follow-up reassessment recipe, clinical library discovery, longitudinal
//! follow-up coherence, and due / physician-triggered dispatch.
//!
//! Runs PostgreSQL tests automatically whenever `PHYSICIAN_RUNTIME_TEST_DATABASE_URL`
//! or `PHYSICIAN_DATABASE_URL` is set; panics if neither is set when
//! `CODA_REQUIRE_POSTGRES_TESTS=1`.

use std::collections::{BTreeMap, VecDeque};
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
    FollowUpReassessmentError, FollowUpReassessmentInitialLoad, FollowUpReassessmentRequest,
    FollowUpReassessmentTrigger, SavedFollowUpReassessmentOutcome,
    build_follow_up_reassessment_starting_context, dispatch_due_follow_up_reassessments,
    follow_up_reassessment_instructions_sha256, recipe as follow_up_reassessment_recipe,
    run_follow_up_reassessment,
};
use local_product::initial_understanding::SAVE_UNDERSTANDING_TOOL;
use local_product::message_review_recipe::{
    MessageReviewRunStatus, RecordedToolDiscovery, ToolDiscoveryDisposition,
};
use local_product::patient_reviews::ReviewOutcome;
use local_product::physician_chat::{PhysicianSkills, SKILL_VIEW};
use local_product::physician_sources::{
    MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SourcePatient, SourceRevision,
    workspace_id,
};
use local_product::runtime::chat_operations::{
    MemoryProvenance, PatientMemoryId, PatientMemoryOperation, PostgresPatientMemoryOperation,
    RecordMemoryCorrection,
};
use local_product::runtime::concerns::{
    CONCERN_TOOL, CommandKey, ExactRecordReference, WorkspaceAccess,
};
use local_product::runtime::follow_ups::{
    FOLLOW_UP_TOOL, FollowUpChangeNote, FollowUpCommandKey, FollowUpId, FollowUpReviewSchedule,
    FollowUpStanding, FollowUpSummary, FollowUpUpdateKind, PhysicianChangeFollowUp,
    PhysicianCreateFollowUp, WithdrawWrongPatientRequest,
};
use local_product::runtime::jobs::is_physician_only_tool;
use local_product::runtime::reviews::{RecordedReviewAction, ReviewLifecycleState};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::today_updates::OpenItemRequest;
use local_product::runtime::{
    Authority, PublicationTransaction, Scope, Store, async_trait, engine,
};
use local_product::{ModelBinding, ModelRoles, open_ai_api_key, sha256_hex};
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
    assert_eq!(wire[0]["reasoning"], json!({ "effort": "high" }));
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

#[tokio::test]
async fn ac2_four_clinical_dispositions_continue_change_complete_and_end_without_completion_with_guards()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let now_ts = current_unix_ts();
    let practice = format!("prac-fu-ac2-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, _token, dr_hash) =
        invite_and_sign_in(&auth, &practice, "dr-fu-ac2", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-fu-ac2-{}", Uuid::new_v4().simple()),
        display_name: "Arthur Pendelton".into(),
        birth_date: Some("1959-07-22".into()),
        sex_or_gender: Some("male".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-9202".into(),
        }],
    };
    let workspace = workspace_id("clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let baseline_note = chart_record(
        &workspace,
        "clinic-ehr",
        "note-baseline-ac2",
        "rev-1",
        "DocumentReference",
        "2026-11-01",
        "Outpatient Care Plan",
        "Ordered sleep study referral, renal/uACR check, repeat potassium, and spironolactone discontinuance note.",
    );
    let repeat_k_lab = chart_record(
        &workspace,
        "clinic-ehr",
        "lab-repeat-k-ac2",
        "rev-1",
        "Observation",
        "2026-11-11",
        "Repeat Serum Potassium",
        "Potassium 4.2 mmol/L (normal, non-hemolyzed).",
    );
    let dc_med_note = chart_record(
        &workspace,
        "clinic-ehr",
        "med-spiro-stopped",
        "rev-1",
        "MedicationRequest",
        "2026-11-04",
        "Spironolactone Stopped",
        "Spironolactone discontinued prior to starting therapy due to hypotension.",
    );
    store
        .record_chart(
            &workspace,
            &[baseline_note, repeat_k_lab, dc_med_note],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient,
                pulled_at: Some(now_ts),
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
    for (id, summary) in [
        ("fu-continue", "Check outside sleep study report"),
        (
            "fu-change",
            "Check renal panel and urine albumin-creatinine ratio",
        ),
        ("fu-complete", "Verify repeat serum potassium normalization"),
        ("fu-end", "Monitor potassium after starting spironolactone"),
    ] {
        store
            .create_follow_up_for_physician(
                &access,
                &FollowUpCommandKey::new(format!("seed-{id}")).unwrap(),
                &PhysicianCreateFollowUp {
                    id: FollowUpId::new(id).unwrap(),
                    summary: FollowUpSummary::new(summary).unwrap(),
                    due_at: now_ts - 60,
                    records: vec![ExactRecordReference::new("note-baseline-ac2", "rev-1").unwrap()],
                },
                &mut PostgresWorkspaceAuthority,
            )
            .await
            .unwrap();
    }

    let skills = load_skills();
    let roles = distinct_roles();

    // --- 1. `ContinueUnchanged` ---
    let (mut p1, _) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "read-1",
            READ_TOOL,
            &json!({
                "references": [{ "candidate_id": "note-baseline-ac2", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384,
            }),
        )],
        vec![sse_function_call(
            "done-1",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "continue_unchanged",
                    "rationale": "Outside sleep lab turnaround is 2-3 weeks; still within expected window with no new symptoms.",
                    "quiet": true,
                },
                "cited_records": [{ "candidate_id": "note-baseline-ac2", "revision": "rev-1" }],
                "basis_and_limitations": "Checked Clinic EHR chart; sleep study report still pending within normal turnaround.",
                "explicit_uncertainty": ["Outside sleep lab report has not arrived yet."],
                "unfinished_work": [],
            }),
        )],
        vec![sse_assistant_message("Continuing unchanged quietly.")],
    ])
    .await;
    let out_1 = run_follow_up_reassessment(
        &store,
        auth.as_ref(),
        FollowUpReassessmentRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            follow_up_id: FollowUpId::new("fu-continue").unwrap(),
            trigger: FollowUpReassessmentTrigger::DueWakeup {
                clinical_now: now_ts,
            },
            review_id: None,
        },
        &roles,
        &skills,
        &mut p1,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert!(matches!(
        out_1.outcome,
        SavedFollowUpReassessmentOutcome::ContinuedUnchanged { quiet: true, .. }
    ));

    // --- 2. `ChangeNextReassessment` ---
    let revised_due = now_ts + 86_400 * 5;
    let (mut p2, _) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "read-2",
            READ_TOOL,
            &json!({
                "references": [{ "candidate_id": "note-baseline-ac2", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384,
            }),
        )],
        vec![sse_function_call(
            "done-2",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "change_next_reassessment",
                    "summary": "Check pending urine albumin-creatinine ratio result",
                    "next_due_at": revised_due,
                    "change_note": "Serum potassium resulted normal; uACR send-out lab remains pending for 5 days.",
                    "rationale": "Partial panel completed; narrowed responsibility to remaining send-out uACR.",
                },
                "cited_records": [{ "candidate_id": "note-baseline-ac2", "revision": "rev-1" }],
                "basis_and_limitations": "Reviewed 2026-11-01 care plan.",
                "explicit_uncertainty": ["Send-out urine albumin-creatinine ratio still pending."],
                "unfinished_work": [],
            }),
        )],
        vec![sse_assistant_message("Changed next reassessment date.")],
    ])
    .await;
    let out_2 = run_follow_up_reassessment(
        &store,
        auth.as_ref(),
        FollowUpReassessmentRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            follow_up_id: FollowUpId::new("fu-change").unwrap(),
            trigger: FollowUpReassessmentTrigger::DueWakeup {
                clinical_now: now_ts,
            },
            review_id: None,
        },
        &roles,
        &skills,
        &mut p2,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert!(matches!(
        out_2.outcome,
        SavedFollowUpReassessmentOutcome::ChangedNextReassessment { ref follow_up, .. }
            if follow_up.due_at == revised_due && follow_up.version == 2
    ));

    // --- 3. `Complete` (proving `complete` WITHOUT verified read evidence is refused first!) ---
    let (mut p3, complete_reqs) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "premature-complete",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "complete",
                    "resolution_note": "Potassium looks normal.",
                    "rationale": "Guessed without reading lab.",
                },
                "cited_records": [],
                "basis_and_limitations": "No records read yet.",
                "explicit_uncertainty": [],
                "unfinished_work": [],
            }),
        )],
        vec![sse_function_call(
            "read-k",
            READ_TOOL,
            &json!({
                "references": [{ "candidate_id": "lab-repeat-k-ac2", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384,
            }),
        )],
        vec![sse_function_call(
            "verified-complete",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "complete",
                    "resolution_note": "Repeat non-hemolyzed serum potassium on 2026-11-11 normalized to 4.2 mmol/L.",
                    "rationale": "Confirmed normal repeat potassium 4.2 mmol/L in Clinic EHR.",
                },
                "cited_records": [{ "candidate_id": "lab-repeat-k-ac2", "revision": "rev-1" }],
                "basis_and_limitations": "Verified 2026-11-11 Repeat Serum Potassium (`lab-repeat-k-ac2` @ `rev-1`).",
                "explicit_uncertainty": [],
                "unfinished_work": [],
            }),
        )],
        vec![sse_assistant_message(
            "Completed follow-up after reading repeat potassium.",
        )],
    ])
    .await;
    let out_3 = run_follow_up_reassessment(
        &store,
        auth.as_ref(),
        FollowUpReassessmentRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            follow_up_id: FollowUpId::new("fu-complete").unwrap(),
            trigger: FollowUpReassessmentTrigger::DueWakeup {
                clinical_now: now_ts,
            },
            review_id: None,
        },
        &roles,
        &skills,
        &mut p3,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    let complete_wire = serde_json::to_string(&complete_reqs.lock().unwrap()[1]["input"]).unwrap();
    assert!(
        complete_wire.contains("verified") || complete_wire.contains("complete"),
        "completing without verified read evidence must be refused: {complete_wire}"
    );
    assert!(matches!(
        out_3.outcome,
        SavedFollowUpReassessmentOutcome::Completed { ref follow_up, .. }
            if matches!(follow_up.standing, FollowUpStanding::Completed { .. })
    ));

    // --- 4. `EndWithoutCompletion` (proving false clinical-completion phrasing is refused first!) ---
    let (mut p4, end_reqs) = spawn_responses_sse_server(vec![
        vec![
            sse_function_call(
                "read-dc",
                READ_TOOL,
                &json!({
                    "references": [{ "candidate_id": "med-spiro-stopped", "revision": "rev-1", "cursor": null }],
                    "max_result_bytes": 16_384,
                }),
            ),
            sse_function_call(
                "bad-end-phrasing",
                COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
                &json!({
                    "decision": {
                        "kind": "end_without_completion",
                        "ending_reason": "Completed successfully with normal results.",
                        "rationale": "Medication stopped.",
                    },
                    "cited_records": [{ "candidate_id": "med-spiro-stopped", "revision": "rev-1" }],
                    "basis_and_limitations": "Read medication stop record.",
                    "explicit_uncertainty": [],
                    "unfinished_work": [],
                }),
            ),
        ],
        vec![sse_function_call(
            "honest-end",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "end_without_completion",
                    "ending_reason": "Spironolactone was discontinued on 2026-11-04 prior to initiation; potassium monitoring loop is no longer applicable.",
                    "rationale": "Medication being monitored was stopped before therapy began; responsibility cancelled rather than completed.",
                },
                "cited_records": [{ "candidate_id": "med-spiro-stopped", "revision": "rev-1" }],
                "basis_and_limitations": "Verified 2026-11-04 MedicationRequest stop (`med-spiro-stopped` @ `rev-1`).",
                "explicit_uncertainty": [],
                "unfinished_work": [],
            }),
        )],
        vec![sse_assistant_message(
            "Ended follow-up without claiming clinical completion.",
        )],
    ])
    .await;
    let out_4 = run_follow_up_reassessment(
        &store,
        auth.as_ref(),
        FollowUpReassessmentRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            follow_up_id: FollowUpId::new("fu-end").unwrap(),
            trigger: FollowUpReassessmentTrigger::DueWakeup {
                clinical_now: now_ts,
            },
            review_id: None,
        },
        &roles,
        &skills,
        &mut p4,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    let end_wire = serde_json::to_string(&end_reqs.lock().unwrap()[1]["input"]).unwrap();
    assert!(
        end_wire.contains("end_without_completion") || end_wire.contains("completed"),
        "false completion wording in EndWithoutCompletion must be refused: {end_wire}"
    );
    assert!(matches!(
        out_4.outcome,
        SavedFollowUpReassessmentOutcome::EndedWithoutCompletion {
            ref follow_up,
            claims_clinical_completion: false,
            ..
        } if matches!(follow_up.standing, FollowUpStanding::Cancelled { .. })
    ));
}

#[tokio::test]
async fn ac3_silence_missing_results_overdue_high_stakes_concern_and_conflicting_dated_records() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let now_ts = current_unix_ts();
    let practice = format!("prac-fu-ac3-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, dr_token, dr_hash) =
        invite_and_sign_in(&auth, &practice, "dr-fu-ac3", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-fu-ac3-{}", Uuid::new_v4().simple()),
        display_name: "Miriam Solis".into(),
        birth_date: Some("1952-10-08".into()),
        sex_or_gender: Some("female".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-9303".into(),
        }],
    };
    let workspace = workspace_id("clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let hosp_discharge = chart_record(
        &workspace,
        "regional-hospital",
        "hosp-discharge-1",
        "rev-1",
        "DocumentReference",
        "2026-11-08",
        "Hospital Discharge Summary",
        "Discharged after AKI (creatinine 2.4 mg/dL, potassium 5.6 mmol/L); HOLD lisinopril and recheck BMP in 72 hours.",
    );
    let clinic_med_list = chart_record(
        &workspace,
        "clinic-ehr",
        "clinic-med-lisinopril",
        "rev-1",
        "MedicationRequest",
        "2026-11-09",
        "Outpatient Medication List",
        "Lisinopril 20 mg orally once daily listed as ACTIVE in outpatient medication reconciliation.",
    );
    store
        .record_chart(
            &workspace,
            &[hosp_discharge],
            &SourcePull {
                source_id: "regional-hospital".into(),
                source_label: "Regional Hospital".into(),
                patient: patient.clone(),
                pulled_at: Some(now_ts),
            },
        )
        .await
        .unwrap();
    store
        .record_chart(
            &workspace,
            &[clinic_med_list],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient: patient.clone(),
                pulled_at: Some(now_ts),
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
    for (id, summary) in [
        (
            "fu-silent-echo",
            "Check outside outpatient echocardiogram report",
        ),
        (
            "fu-overdue-aki-bmp",
            "Verify 72-hour post-discharge creatinine and potassium recheck",
        ),
        (
            "fu-conflict-ace",
            "Reconcile post-discharge lisinopril hold vs outpatient active list",
        ),
    ] {
        store
            .create_follow_up_for_physician(
                &access,
                &FollowUpCommandKey::new(format!("seed-{id}")).unwrap(),
                &PhysicianCreateFollowUp {
                    id: FollowUpId::new(id).unwrap(),
                    summary: FollowUpSummary::new(summary).unwrap(),
                    due_at: now_ts - 3_600,
                    records: vec![ExactRecordReference::new("hosp-discharge-1", "rev-1").unwrap()],
                },
                &mut PostgresWorkspaceAuthority,
            )
            .await
            .unwrap();
    }

    let skills = load_skills();
    let roles = distinct_roles();

    // --- A. Missing expected outside echo + patient silence -> explicit uncertainty, quiet continuation ---
    let (mut p_silent, _) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "read-silent",
            READ_TOOL,
            &json!({
                "references": [{ "candidate_id": "hosp-discharge-1", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384,
            }),
        )],
        vec![sse_function_call(
            "done-silent",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "continue_unchanged",
                    "rationale": "Outside echocardiogram report has not yet arrived and patient has sent no messages; silence does not resolve the follow-up.",
                    "quiet": true,
                },
                "cited_records": [{ "candidate_id": "hosp-discharge-1", "revision": "rev-1" }],
                "basis_and_limitations": "Checked Regional Hospital and Clinic EHR records; no echocardiogram report on file.",
                "explicit_uncertainty": [
                    "Outside outpatient echocardiogram report is missing; patient silence does not confirm whether the study occurred."
                ],
                "unfinished_work": [],
            }),
        )],
        vec![sse_assistant_message(
            "Kept missing-echo follow-up active with explicit uncertainty.",
        )],
    ])
    .await;
    let silent_done = run_follow_up_reassessment(
        &store,
        auth.as_ref(),
        FollowUpReassessmentRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            follow_up_id: FollowUpId::new("fu-silent-echo").unwrap(),
            trigger: FollowUpReassessmentTrigger::DueWakeup {
                clinical_now: now_ts,
            },
            review_id: None,
        },
        &roles,
        &skills,
        &mut p_silent,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert!(matches!(
        &silent_done.outcome,
        SavedFollowUpReassessmentOutcome::ContinuedUnchanged {
            explicit_uncertainty,
            quiet: true,
            ..
        } if explicit_uncertainty.iter().any(|u| u.contains("patient silence"))
    ));

    // --- B. Overdue high-stakes post-discharge AKI BMP -> raises `Today` concern + updates next reassessment ---
    let (mut p_overdue, _) = spawn_responses_sse_server(vec![
        vec![
            sse_function_call(
                "read-aki",
                READ_TOOL,
                &json!({
                    "references": [{ "candidate_id": "hosp-discharge-1", "revision": "rev-1", "cursor": null }],
                    "max_result_bytes": 16_384,
                }),
            ),
            sse_function_call(
                "raise-aki-concern",
                CONCERN_TOOL,
                &json!({
                    "operation": "raise",
                    "concern_id": "concern-overdue-aki-bmp",
                    "urgency": "today",
                    "reason": "Overdue 72-hour post-discharge BMP after AKI (creatinine 2.4 mg/dL, potassium 5.6 mmol/L on 2026-11-08) with no repeat lab on file.",
                    "records": [{ "candidate_id": "hosp-discharge-1", "revision": "rev-1" }],
                }),
            ),
        ],
        vec![sse_function_call(
            "done-overdue",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "change_next_reassessment",
                    "summary": "Verify urgent outreach / repeat post-AKI BMP completion",
                    "next_due_at": now_ts + 86_400,
                    "change_note": "72-hour post-AKI BMP overdue; raised Today concern and set 24-hour recheck.",
                    "rationale": "High-stakes post-AKI renal and potassium check is overdue with no lab result received.",
                },
                "cited_records": [{ "candidate_id": "hosp-discharge-1", "revision": "rev-1" }],
                "basis_and_limitations": "2026-11-08 Regional Hospital discharge summary directed 72-hour BMP after Cr 2.4 / K 5.6.",
                "explicit_uncertainty": ["No post-discharge metabolic panel found in Regional Hospital or Clinic EHR."],
                "unfinished_work": [],
            }),
        )],
        vec![sse_assistant_message(
            "Raised overdue AKI BMP concern and rescheduled reassessment.",
        )],
    ])
    .await;
    run_follow_up_reassessment(
        &store,
        auth.as_ref(),
        FollowUpReassessmentRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            follow_up_id: FollowUpId::new("fu-overdue-aki-bmp").unwrap(),
            trigger: FollowUpReassessmentTrigger::DueWakeup {
                clinical_now: now_ts,
            },
            review_id: None,
        },
        &roles,
        &skills,
        &mut p_overdue,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // --- C. Conflicting dated records across two sources -> cites BOTH revisions and states discrepancy ---
    let (mut p_conflict, _) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "read-both-conflict",
            READ_TOOL,
            &json!({
                "references": [
                    { "candidate_id": "hosp-discharge-1", "revision": "rev-1", "cursor": null },
                    { "candidate_id": "clinic-med-lisinopril", "revision": "rev-1", "cursor": null },
                ],
                "max_result_bytes": 32_768,
            }),
        )],
        vec![sse_function_call(
            "done-conflict",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "continue_unchanged",
                    "rationale": "Conflicting dated medication status across hospital discharge vs clinic list prevents closing follow-up as complete.",
                    "quiet": false,
                },
                "cited_records": [
                    { "candidate_id": "hosp-discharge-1", "revision": "rev-1" },
                    { "candidate_id": "clinic-med-lisinopril", "revision": "rev-1" },
                ],
                "basis_and_limitations": "2026-11-08 Regional Hospital discharge summary (`hosp-discharge-1` @ `rev-1`) holds lisinopril for AKI/hyperkalemia, whereas 2026-11-09 Clinic EHR medication list (`clinic-med-lisinopril` @ `rev-1`) still lists lisinopril 20 mg daily as active.",
                "explicit_uncertainty": [
                    "Conflicting dated records: 2026-11-08 hospital discharge holds lisinopril, while 2026-11-09 outpatient list shows lisinopril 20 mg active."
                ],
                "unfinished_work": [],
            }),
        )],
        vec![sse_assistant_message(
            "Cited both conflicting revisions and kept follow-up open.",
        )],
    ])
    .await;
    let conflict_done = run_follow_up_reassessment(
        &store,
        auth.as_ref(),
        FollowUpReassessmentRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            follow_up_id: FollowUpId::new("fu-conflict-ace").unwrap(),
            trigger: FollowUpReassessmentTrigger::DueWakeup {
                clinical_now: now_ts,
            },
            review_id: None,
        },
        &roles,
        &skills,
        &mut p_conflict,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    let review = store
        .patient_review(
            &practice,
            &workspace,
            &conflict_done.patient_review.review_id,
            auth.as_ref(),
        )
        .await
        .unwrap()
        .unwrap();
    let ReviewLifecycleState::Completed { outcome, .. } = review.state else {
        panic!("expected completed patient review");
    };
    let (citations, basis_text) = match outcome {
        ReviewOutcome::Quiet {
            citations,
            basis_and_limitations,
            ..
        }
        | ReviewOutcome::Assessed {
            citations,
            basis_and_limitations,
            ..
        } => (citations, basis_and_limitations),
    };
    assert_eq!(citations.len(), 2);
    assert!(basis_text.contains("Conflicting dated records"));

    let today = store
        .read_today(auth.as_ref(), &dr_token, now_ts as u64, &BTreeMap::new())
        .await
        .unwrap();
    assert!(
        today
            .rows
            .iter()
            .any(|r| r.item_key.as_str().contains("concern-overdue-aki-bmp"))
    );
}

#[tokio::test]
async fn ac4_longitudinal_three_run_sequence_no_duplicate_concerns_memory_correction_and_sibling_isolation()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let now_ts = current_unix_ts();
    let practice = format!("prac-fu-ac4-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, _token, dr_hash) =
        invite_and_sign_in(&auth, &practice, "dr-fu-ac4", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-fu-ac4-{}", Uuid::new_v4().simple()),
        display_name: "Walter Bishop".into(),
        birth_date: Some("1948-09-20".into()),
        sex_or_gender: Some("male".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-9404".into(),
        }],
    };
    let workspace = workspace_id("clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let inr_note = chart_record(
        &workspace,
        "clinic-ehr",
        "note-warfarin-1",
        "rev-1",
        "DocumentReference",
        "2026-11-01",
        "Anticoagulation Clinic Note",
        "On warfarin for atrial fibrillation; monthly INR due 2026-11-10.",
    );
    store
        .record_chart(
            &workspace,
            &[inr_note],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient,
                pulled_at: Some(now_ts),
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
    let target_id = FollowUpId::new("fu-target-inr").unwrap();
    let sibling_id = FollowUpId::new("fu-sibling-colonoscopy").unwrap();
    store
        .create_follow_up_for_physician(
            &access,
            &FollowUpCommandKey::new("seed-target-inr").unwrap(),
            &PhysicianCreateFollowUp {
                id: target_id.clone(),
                summary: FollowUpSummary::new("Check monthly warfarin INR").unwrap(),
                due_at: now_ts - 3_600,
                records: vec![ExactRecordReference::new("note-warfarin-1", "rev-1").unwrap()],
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    store
        .create_follow_up_for_physician(
            &access,
            &FollowUpCommandKey::new("seed-sibling-colon").unwrap(),
            &PhysicianCreateFollowUp {
                id: sibling_id.clone(),
                summary: FollowUpSummary::new("Confirm 5-year surveillance colonoscopy report")
                    .unwrap(),
                due_at: now_ts + 86_400 * 30,
                records: vec![ExactRecordReference::new("note-warfarin-1", "rev-1").unwrap()],
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    let skills = load_skills();
    let roles = distinct_roles();

    // --- Run 1: Raises `concern-overdue-inr`, attempts to cancel sibling (REFUSED!), and continues on schedule ---
    let (mut p_r1, r1_reqs) = spawn_responses_sse_server(vec![
        vec![
            sse_function_call(
                "read-w1",
                READ_TOOL,
                &json!({
                    "references": [{ "candidate_id": "note-warfarin-1", "revision": "rev-1", "cursor": null }],
                    "max_result_bytes": 16_384,
                }),
            ),
            sse_function_call(
                "illegal-sibling-cancel",
                FOLLOW_UP_TOOL,
                &json!({
                    "operation": "cancel",
                    "follow_up_id": "fu-sibling-colonoscopy",
                    "reason": "Attempted accidental sibling cancellation.",
                }),
            ),
            sse_function_call(
                "raise-inr-c",
                CONCERN_TOOL,
                &json!({
                    "operation": "raise",
                    "concern_id": "concern-overdue-inr",
                    "urgency": "today",
                    "reason": "Monthly warfarin INR due 2026-11-10 has not resulted.",
                    "records": [{ "candidate_id": "note-warfarin-1", "revision": "rev-1" }],
                }),
            ),
        ],
        vec![sse_function_call(
            "done-r1",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "continue_unchanged",
                    "rationale": "Overdue warfarin INR surfaced via Today concern while keeping follow-up active.",
                    "quiet": false,
                },
                "cited_records": [{ "candidate_id": "note-warfarin-1", "revision": "rev-1" }],
                "basis_and_limitations": "2026-11-01 Anticoagulation note shows INR due 2026-11-10.",
                "explicit_uncertainty": ["No November INR result on file."],
                "unfinished_work": [],
            }),
        )],
        vec![sse_assistant_message("Run 1 raised overdue INR concern.")],
    ])
    .await;

    let mut dispatched_r1 = dispatch_due_follow_up_reassessments(
        &store,
        auth.as_ref(),
        &practice,
        &workspace,
        now_ts,
        &roles,
        &skills,
        &mut p_r1,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(dispatched_r1.len(), 1);
    let r1_done = dispatched_r1.pop().unwrap();
    assert_eq!(r1_done.run.follow_up_id, target_id);
    let r1_wire = serde_json::to_string(&r1_reqs.lock().unwrap()[1]["input"]).unwrap();
    assert!(
        r1_wire.contains("sibling") || r1_wire.contains("fu-sibling-colonoscopy"),
        "cancelling unrelated sibling follow-up must be refused: {r1_wire}"
    );

    // Verify `dispatch_due_follow_up_reassessments` skips re-running unchanged chart + follow-up + memory!
    let (mut p_noop, _) = spawn_responses_sse_server(vec![]).await;
    let skipped = dispatch_due_follow_up_reassessments(
        &store,
        auth.as_ref(),
        &practice,
        &workspace,
        now_ts + 60,
        &roles,
        &skills,
        &mut p_noop,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert!(
        skipped.is_empty(),
        "due dispatcher must not re-run unchanged follow-up without new chart or memory changes"
    );

    // --- Run 2: Explicit `PhysicianReviewNow` on unchanged chart -> sees Run 1 + open concern, does NOT raise duplicate concern! ---
    let (mut p_r2, r2_reqs) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "read-w2",
            READ_TOOL,
            &json!({
                "references": [{ "candidate_id": "note-warfarin-1", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384,
            }),
        )],
        vec![sse_function_call(
            "done-r2",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "continue_unchanged",
                    "rationale": "Chart unchanged since prior reassessment and `concern-overdue-inr` is already open; continuing quietly without duplicate concern.",
                    "quiet": true,
                },
                "cited_records": [{ "candidate_id": "note-warfarin-1", "revision": "rev-1" }],
                "basis_and_limitations": "No new records since Run 1; existing Today concern remains open.",
                "explicit_uncertainty": ["INR lab still pending."],
                "unfinished_work": [],
            }),
        )],
        vec![sse_assistant_message(
            "Run 2 continued quietly without duplicate concern.",
        )],
    ])
    .await;
    let r2_done = run_follow_up_reassessment(
        &store,
        auth.as_ref(),
        FollowUpReassessmentRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            follow_up_id: target_id.clone(),
            trigger: FollowUpReassessmentTrigger::PhysicianReviewNow {
                physician_id: dr.physician_id.clone(),
            },
            review_id: None,
        },
        &roles,
        &skills,
        &mut p_r2,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(
        r2_done
            .run
            .starting_context
            .review_context
            .prior_reviews
            .len(),
        1
    );
    let r2_wire = serde_json::to_string(&r2_reqs.lock().unwrap()[0]["input"]).unwrap();
    assert!(r2_wire.contains("concern-overdue-inr"));
    let concerns_after_r2 = store
        .workspace_concerns(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(
        concerns_after_r2.len(),
        1,
        "Run 2 on unchanged chart must not create duplicate concerns"
    );

    // --- Run 3: Physician records `patient_memory` correction -> `dispatch_due_follow_up_reassessments` wakes and retires/ends target while sibling stays intact! ---
    PostgresPatientMemoryOperation
        .record_correction(
            &store,
            "mem-warfarin-switched",
            &RecordMemoryCorrection {
                practice: practice.clone(),
                workspace: workspace.clone(),
                id: PatientMemoryId::new("mem-apixaban-switch").unwrap(),
                topic: "Anticoagulation transition".into(),
                statement: "Physician confirmed outside cardiology transitioned patient from warfarin to apixaban 5 mg BID on 2026-11-05; INR monitoring discontinued.".into(),
                provenance: MemoryProvenance::PhysicianCorrection {
                    physician: dr.physician_id.clone(),
                    chat_id: "chat-anticoag-update".into(),
                    message_id: "msg-anticoag-1".into(),
                    work_id: "work-anticoag-1".into(),
                    attempt_id: "attempt-anticoag-1".into(),
                },
            },
        )
        .await
        .unwrap();

    let (mut p_r3, r3_reqs) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "retire-inr-c",
            CONCERN_TOOL,
            &json!({
                "operation": "retire",
                "concern_id": "concern-overdue-inr",
                "reason": "Physician memory correction confirmed warfarin was transitioned to apixaban 5 mg BID; INR monitoring no longer indicated.",
            }),
        )],
        vec![sse_function_call(
            "done-r3",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "end_without_completion",
                    "ending_reason": "Warfarin discontinued after transition to apixaban 5 mg BID per physician memory correction; INR surveillance no longer applicable.",
                    "rationale": "Physician memory correction superseded warfarin INR premise; retired INR concern and cancelled INR follow-up.",
                },
                "cited_records": [],
                "basis_and_limitations": "Grounded in active physician memory correction (`mem-apixaban-switch`).",
                "explicit_uncertainty": [],
                "unfinished_work": [],
            }),
        )],
        vec![sse_assistant_message(
            "Ended warfarin INR follow-up per physician memory correction.",
        )],
    ])
    .await;

    let dispatched_r3 = dispatch_due_follow_up_reassessments(
        &store,
        auth.as_ref(),
        &practice,
        &workspace,
        now_ts + 200,
        &roles,
        &skills,
        &mut p_r3,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(dispatched_r3.len(), 1);
    let r3_wire = serde_json::to_string(&r3_reqs.lock().unwrap()[0]["input"]).unwrap();
    assert!(r3_wire.contains("mem-apixaban-switch"));
    assert!(
        dispatched_r3[0]
            .patient_review
            .actions
            .iter()
            .any(|action| {
                matches!(
                    action,
                    RecordedReviewAction::ConcernRetired {
                        concern_id,
                        retirement_reason,
                        ..
                    } if concern_id == "concern-overdue-inr"
                        && retirement_reason.contains("transitioned to apixaban 5 mg BID")
                )
            }),
        "Run 3 patient_review.actions must record ConcernRetired with retirement_reason for pre-existing concern: {:?}",
        dispatched_r3[0].patient_review.actions
    );

    let final_follow_ups = store
        .physician_follow_ups(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    let target_final = final_follow_ups.iter().find(|f| f.id == target_id).unwrap();
    let sibling_final = final_follow_ups
        .iter()
        .find(|f| f.id == sibling_id)
        .unwrap();
    assert!(matches!(
        target_final.standing,
        FollowUpStanding::Cancelled { .. }
    ));
    assert!(
        matches!(sibling_final.standing, FollowUpStanding::Active { .. })
            && sibling_final.version == 1,
        "unrelated sibling follow-up must remain completely intact across all 3 runs"
    );

    let runs_history = store
        .follow_up_reassessment_runs_for_item(&practice, &workspace, &target_id)
        .await
        .unwrap();
    assert_eq!(runs_history.len(), 3);
}

#[tokio::test]
async fn ac5_due_wakeup_and_review_now_dispatch_surfaces_and_stopped_completed_cancelled_fences() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let now_ts = current_unix_ts();
    let practice = format!("prac-fu-ac5-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, dr_token, dr_hash) =
        invite_and_sign_in(&auth, &practice, "dr-fu-ac5", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-fu-ac5-{}", Uuid::new_v4().simple()),
        display_name: "Grace Holloway".into(),
        birth_date: Some("1963-05-17".into()),
        sex_or_gender: Some("female".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-9505".into(),
        }],
    };
    let workspace = workspace_id("clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let chest_xray = chart_record(
        &workspace,
        "clinic-ehr",
        "rad-cxr-1",
        "rev-1",
        "DiagnosticReport",
        "2026-11-11",
        "Follow-Up Chest Radiograph",
        "Complete resolution of right lower lobe infiltrate; lungs clear.",
    );
    store
        .record_chart(
            &workspace,
            &[chest_xray],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient,
                pulled_at: Some(now_ts),
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
    let fu_due = FollowUpId::new("fu-ac5-due").unwrap();
    let fu_review_now = FollowUpId::new("fu-ac5-review-now").unwrap();
    let fu_stopped = FollowUpId::new("fu-ac5-stopped").unwrap();
    let fu_cancelled = FollowUpId::new("fu-ac5-cancelled").unwrap();

    for (id, summary, due_at) in [
        (
            &fu_due,
            "Verify 6-week post-pneumonia chest radiograph clearance",
            now_ts - 300,
        ),
        (
            &fu_review_now,
            "Check pulmonary function test scheduling",
            now_ts + 86_400 * 21,
        ),
        (
            &fu_stopped,
            "Paused home peak-flow diary check",
            now_ts - 300,
        ),
        (
            &fu_cancelled,
            "Cancelled duplicate spirometry check",
            now_ts - 300,
        ),
    ] {
        store
            .create_follow_up_for_physician(
                &access,
                &FollowUpCommandKey::new(format!("create-{}", id.as_str())).unwrap(),
                &PhysicianCreateFollowUp {
                    id: id.clone(),
                    summary: FollowUpSummary::new(summary).unwrap(),
                    due_at,
                    records: vec![ExactRecordReference::new("rad-cxr-1", "rev-1").unwrap()],
                },
                &mut PostgresWorkspaceAuthority,
            )
            .await
            .unwrap();
    }

    // Physician triggers `Review now` on `fu_review_now`, `Stop` (`Held`) on `fu_stopped`, and `Cancel` on `fu_cancelled`:
    store
        .review_follow_up_now_for_physician(
            &access,
            &FollowUpCommandKey::new("cmd-rev-now").unwrap(),
            &fu_review_now,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    store
        .stop_follow_up_for_physician(
            &access,
            &FollowUpCommandKey::new("cmd-stop").unwrap(),
            &fu_stopped,
            Some(FollowUpChangeNote::new("Paused by physician until spring").unwrap()),
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    store
        .cancel_follow_up_for_physician(
            &access,
            &FollowUpCommandKey::new("cmd-cancel").unwrap(),
            &fu_cancelled,
            FollowUpChangeNote::new("Duplicate item").unwrap(),
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    // Verify `due_follow_ups` includes ONLY `fu_due` and `fu_review_now`, excluding `Held` and `Cancelled`!
    let due_list = store
        .due_follow_ups(&practice, &workspace, now_ts)
        .await
        .unwrap();
    let due_ids: Vec<&str> = due_list.iter().map(|f| f.id.as_str()).collect();
    assert_eq!(due_ids, vec!["fu-ac5-due", "fu-ac5-review-now"]);

    let skills = load_skills();
    let roles = distinct_roles();

    // Dispatch due reassessments (`fu-ac5-due` -> Complete; `fu-ac5-review-now` -> ChangeNextReassessment):
    let (mut p_dispatch, _) = spawn_responses_sse_server(vec![
        // Item 1 (`fu-ac5-due`):
        vec![sse_function_call(
            "read-cxr-1",
            READ_TOOL,
            &json!({
                "references": [{ "candidate_id": "rad-cxr-1", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384,
            }),
        )],
        vec![sse_function_call(
            "complete-cxr",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "complete",
                    "resolution_note": "2026-11-11 follow-up chest radiograph confirmed complete resolution of right lower lobe infiltrate.",
                    "rationale": "Radiology report confirms complete clearance.",
                },
                "cited_records": [{ "candidate_id": "rad-cxr-1", "revision": "rev-1" }],
                "basis_and_limitations": "Verified 2026-11-11 DiagnosticReport `rad-cxr-1` @ `rev-1`.",
                "explicit_uncertainty": [],
                "unfinished_work": [],
            }),
        )],
        vec![sse_assistant_message(
            "Completed post-pneumonia CXR follow-up.",
        )],
        // Item 2 (`fu-ac5-review-now`):
        vec![sse_function_call(
            "read-cxr-2",
            READ_TOOL,
            &json!({
                "references": [{ "candidate_id": "rad-cxr-1", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384,
            }),
        )],
        vec![sse_function_call(
            "change-pft",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "change_next_reassessment",
                    "summary": "Check outpatient spirometry / PFT scheduling after clear CXR",
                    "next_due_at": now_ts + 86_400 * 10,
                    "change_note": "Reviewed immediately per physician request; CXR clear, PFT still pending scheduling.",
                    "rationale": "CXR resolved; moved PFT check to 10 days.",
                },
                "cited_records": [{ "candidate_id": "rad-cxr-1", "revision": "rev-1" }],
                "basis_and_limitations": "Checked Clinic EHR chart on physician Review now request.",
                "explicit_uncertainty": ["PFT appointment not yet booked."],
                "unfinished_work": [],
            }),
        )],
        vec![sse_assistant_message(
            "Updated physician-requested follow-up.",
        )],
    ])
    .await;

    let outcomes = dispatch_due_follow_up_reassessments(
        &store,
        auth.as_ref(),
        &practice,
        &workspace,
        now_ts,
        &roles,
        &skills,
        &mut p_dispatch,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(outcomes.len(), 2);

    // Direct `run_follow_up_reassessment` on `Held`, `Completed`, or `Cancelled` MUST fail with `NotEligible`!
    for ineligible_id in [&fu_stopped, &fu_due, &fu_cancelled] {
        let (mut p_unused, _) = spawn_responses_sse_server(vec![]).await;
        let err = run_follow_up_reassessment(
            &store,
            auth.as_ref(),
            FollowUpReassessmentRequest {
                practice: practice.clone(),
                workspace: workspace.clone(),
                follow_up_id: ineligible_id.clone(),
                trigger: FollowUpReassessmentTrigger::DueWakeup {
                    clinical_now: now_ts,
                },
                review_id: None,
            },
            &roles,
            &skills,
            &mut p_unused,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap_err();
        assert!(
            matches!(err, FollowUpReassessmentError::NotEligible { .. }),
            "expected NotEligible for `{ineligible_id}`, got {err:?}"
        );
    }

    // Verify patient Follow-Ups list, Follow-Up Updates log, patient_reviews, and Updates projection + evidence reopen!
    let follow_ups = store
        .physician_follow_ups(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(follow_ups.len(), 4);

    let fu_updates = store
        .physician_follow_up_updates(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert!(
        fu_updates
            .iter()
            .any(|u| u.follow_up_id == fu_due && u.kind == FollowUpUpdateKind::Completed)
    );
    assert!(
        fu_updates
            .iter()
            .any(|u| u.follow_up_id == fu_review_now && u.kind == FollowUpUpdateKind::Changed)
    );

    let saved_reviews = store
        .patient_reviews(&practice, &workspace, auth.as_ref())
        .await
        .unwrap();
    assert_eq!(saved_reviews.len(), 2);

    let updates = store
        .read_updates(auth.as_ref(), &dr_token, now_ts as u64, &BTreeMap::new())
        .await
        .unwrap();
    let cxr_update = updates
        .today_items
        .iter()
        .find(|u| u.title.contains("fu-ac5-due") || u.detail.contains("fu-ac5-due"))
        .expect("completed follow-up reassessment visible in Updates");
    let reopened = store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut PostgresWorkspaceAuthority,
            &OpenItemRequest {
                token: &dr_token,
                real_now_unix: now_ts as u64,
                workspace: &workspace,
                item_key: &cxr_update.item_key,
                command_key: &CommandKey::new("open-ac5-reassessment").unwrap(),
                patient_names: &BTreeMap::new(),
            },
        )
        .await
        .unwrap();
    assert_eq!(reopened.citations.len(), 1);
    assert!(
        reopened.citations[0]
            .body
            .contains("Complete resolution of right lower lobe infiltrate")
    );
}

#[tokio::test]
async fn round1_complete_citing_unread_understanding_anchor_is_refused_until_read_in_run() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let now_ts = current_unix_ts();
    let practice = format!("prac-fu-r1-unread-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, _tok, dr_hash) =
        invite_and_sign_in(&auth, &practice, "dr-fu-r1-unread", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-fu-r1-unread-{}", Uuid::new_v4().simple()),
        display_name: "Lydia Okafor".into(),
        birth_date: Some("1961-08-29".into()),
        sex_or_gender: Some("female".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-FU-UNREAD".into(),
        }],
    };
    let workspace = workspace_id("clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let anchor_lab = chart_record(
        &workspace,
        "clinic-ehr",
        "anchor-lab-1",
        "rev-1",
        "Observation",
        "2026-11-10",
        "Serum TSH",
        "TSH 2.1 mIU/L (euthyroid on levothyroxine 75 mcg daily).",
    );
    store
        .record_chart(
            &workspace,
            &[anchor_lab],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient,
                pulled_at: Some(now_ts),
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
    let fu_id = FollowUpId::new("fu-r1-unread-anchor").unwrap();
    store
        .create_follow_up_for_physician(
            &access,
            &FollowUpCommandKey::new("seed-fu-unread-anchor").unwrap(),
            &PhysicianCreateFollowUp {
                id: fu_id.clone(),
                summary: FollowUpSummary::new("Verify 6-week TSH after levothyroxine adjustment")
                    .unwrap(),
                due_at: now_ts - 60,
                records: vec![ExactRecordReference::new("anchor-lab-1", "rev-1").unwrap()],
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    let skills = load_skills();
    let roles = distinct_roles();

    // Run 1: reads `anchor-lab-1` @ `rev-1` and saves `PatientUnderstanding` indexing `anchor-lab-1`
    // in BOTH `claims[0].citations` AND `operating_index.anchors`, while keeping the follow-up active.
    let (mut p_seed_u, _) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "read-anchor-r1",
            READ_TOOL,
            &json!({
                "references": [{ "candidate_id": "anchor-lab-1", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384,
            }),
        )],
        vec![
            sse_function_call(
                "save-u-r1",
                SAVE_UNDERSTANDING_TOOL,
                &json!({
                    "expected_revision": 0,
                    "operating_index": {
                        "working_overview": "Hypothyroidism on levothyroxine 75 mcg; 2026-11-10 TSH 2.1 mIU/L.",
                        "known_health_systems": [{
                            "name": "Clinic EHR",
                            "standing": { "kind": "connected", "source_id": "clinic-ehr" },
                            "note": "Primary outpatient chart",
                        }],
                        "open_threads": ["Confirm symptom stability alongside euthyroid TSH"],
                        "anchors": [{
                            "reference": { "candidate_id": "anchor-lab-1", "revision": "rev-1" },
                            "source_id": "clinic-ehr",
                            "topic": "Thyroid monitoring",
                        }],
                    },
                    "claims": [{
                        "text": "2026-11-10 TSH is 2.1 mIU/L on levothyroxine 75 mcg daily.",
                        "citations": [{ "candidate_id": "anchor-lab-1", "revision": "rev-1" }],
                    }],
                    "material_gaps": ["Symptom check still open before closing follow-up."],
                }),
            ),
            sse_function_call(
                "done-u-r1",
                COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
                &json!({
                    "decision": {
                        "kind": "continue_unchanged",
                        "rationale": "Saved longitudinal understanding; awaiting next check.",
                        "quiet": true,
                    },
                    "cited_records": [{ "candidate_id": "anchor-lab-1", "revision": "rev-1" }],
                    "basis_and_limitations": "Read TSH 2.1 mIU/L in Run 1.",
                    "explicit_uncertainty": [],
                    "unfinished_work": [],
                }),
            ),
        ],
        vec![sse_assistant_message("Seeded understanding in Run 1.")],
    ])
    .await;
    run_follow_up_reassessment(
        &store,
        auth.as_ref(),
        FollowUpReassessmentRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            follow_up_id: fu_id.clone(),
            trigger: FollowUpReassessmentTrigger::PhysicianReviewNow {
                physician_id: dr.physician_id.clone(),
            },
            review_id: None,
        },
        &roles,
        &skills,
        &mut p_seed_u,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Run 2: attempts `Complete` citing `anchor-lab-1` @ `rev-1` WITHOUT calling `read_patient_sources` in this run!
    // Must be REFUSED until `read_patient_sources` is called in Run 2.
    let (mut p_r2, r2_reqs) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "complete-without-read",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "complete",
                    "resolution_note": "TSH 2.1 mIU/L confirmed euthyroid.",
                    "rationale": "Attempted completion from prior understanding citation without reading in this run.",
                },
                "cited_records": [{ "candidate_id": "anchor-lab-1", "revision": "rev-1" }],
                "basis_and_limitations": "Relied on prior understanding without reading in Run 2.",
                "explicit_uncertainty": [],
                "unfinished_work": [],
            }),
        )],
        vec![sse_function_call(
            "read-in-r2",
            READ_TOOL,
            &json!({
                "references": [{ "candidate_id": "anchor-lab-1", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384,
            }),
        )],
        vec![sse_function_call(
            "complete-after-read",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "complete",
                    "resolution_note": "Verified 2026-11-10 TSH 2.1 mIU/L directly in Run 2; thyroid replacement stable.",
                    "rationale": "Read `anchor-lab-1` @ `rev-1` in this run before completing.",
                },
                "cited_records": [{ "candidate_id": "anchor-lab-1", "revision": "rev-1" }],
                "basis_and_limitations": "Directly read `anchor-lab-1` @ `rev-1` in Run 2.",
                "explicit_uncertainty": [],
                "unfinished_work": [],
            }),
        )],
        vec![sse_assistant_message("Completed after reading in Run 2.")],
    ])
    .await;

    let r2_done = run_follow_up_reassessment(
        &store,
        auth.as_ref(),
        FollowUpReassessmentRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            follow_up_id: fu_id,
            trigger: FollowUpReassessmentTrigger::DueWakeup {
                clinical_now: now_ts + 30,
            },
            review_id: None,
        },
        &roles,
        &skills,
        &mut p_r2,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert!(matches!(
        r2_done.run.saved_outcome,
        Some(SavedFollowUpReassessmentOutcome::Completed { .. })
    ));
    let turn2_wire = serde_json::to_string(&r2_reqs.lock().unwrap()[1]["input"]).unwrap();
    assert!(
        turn2_wire.contains("via `read_patient_sources` before citing it"),
        "Run 2 must refuse Complete citing unread PatientUnderstanding anchor: {turn2_wire}"
    );
}

#[tokio::test]
async fn round1_review_now_continue_unchanged_clears_due_now_back_to_scheduled_until_due_at() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let now_ts = current_unix_ts();
    let practice = format!("prac-fu-r1-duenow-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, _tok, dr_hash) =
        invite_and_sign_in(&auth, &practice, "dr-fu-r1-duenow", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-fu-r1-duenow-{}", Uuid::new_v4().simple()),
        display_name: "Trevor Alcott".into(),
        birth_date: Some("1973-02-11".into()),
        sex_or_gender: Some("male".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-FU-DUENOW".into(),
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
        "note-holter-1",
        "rev-1",
        "DocumentReference",
        "2026-11-10",
        "Cardiology Referral Note",
        "14-day ambulatory patch monitor ordered; results expected in two weeks.",
    );
    store
        .record_chart(
            &workspace,
            &[note],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient: patient.clone(),
                pulled_at: Some(now_ts),
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
    let future_due_at = now_ts + 86_400 * 14;
    let fu_id = FollowUpId::new("fu-r1-duenow").unwrap();
    store
        .create_follow_up_for_physician(
            &access,
            &FollowUpCommandKey::new("create-fu-duenow").unwrap(),
            &PhysicianCreateFollowUp {
                id: fu_id.clone(),
                summary: FollowUpSummary::new("Check 14-day ambulatory patch monitor report")
                    .unwrap(),
                due_at: future_due_at,
                records: vec![ExactRecordReference::new("note-holter-1", "rev-1").unwrap()],
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    // Physician triggers `Review now` well before `future_due_at`:
    store
        .review_follow_up_now_for_physician(
            &access,
            &FollowUpCommandKey::new("cmd-duenow-1").unwrap(),
            &fu_id,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    let skills = load_skills();
    let roles = distinct_roles();
    let (mut p_r1, _) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "read-holter-1",
            READ_TOOL,
            &json!({
                "references": [{ "candidate_id": "note-holter-1", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384,
            }),
        )],
        vec![sse_function_call(
            "continue-holter-1",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "continue_unchanged",
                    "rationale": "14-day monitor has not yet resulted; keep scheduled due date in 2 weeks.",
                    "quiet": true,
                },
                "cited_records": [{ "candidate_id": "note-holter-1", "revision": "rev-1" }],
                "basis_and_limitations": "Checked Clinic EHR; patch monitor report still pending.",
                "explicit_uncertainty": ["Monitor wear period still in progress."],
                "unfinished_work": [],
            }),
        )],
        vec![sse_assistant_message("Kept 2-week scheduled due date.")],
    ])
    .await;

    let d1 = dispatch_due_follow_up_reassessments(
        &store,
        auth.as_ref(),
        &practice,
        &workspace,
        now_ts + 10,
        &roles,
        &skills,
        &mut p_r1,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(d1.len(), 1);

    // Standing MUST be `Scheduled { due_at: future_due_at }`, NOT stuck on `DueNow`!
    let after_r1 = store
        .physician_follow_ups(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(
        after_r1[0].standing,
        FollowUpStanding::Active {
            schedule: FollowUpReviewSchedule::Scheduled {
                due_at: future_due_at
            },
        },
        "ContinueUnchanged must clear DueNow back to Scheduled"
    );

    // Unrelated chart pull BEFORE `future_due_at` MUST NOT re-trigger this item!
    let unrelated = chart_record(
        &workspace,
        "clinic-ehr",
        "obs-flu-shot",
        "rev-1",
        "Immunization",
        "2026-11-12",
        "Influenza Vaccine",
        "Routine seasonal influenza immunization administered.",
    );
    store.record_source(&unrelated).await.unwrap();

    let due_before_target = store
        .due_follow_ups(&practice, &workspace, now_ts + 300)
        .await
        .unwrap();
    assert!(
        due_before_target.is_empty(),
        "item must not remain due before scheduled due_at: {due_before_target:?}"
    );
    let (mut p_empty, _) = spawn_responses_sse_server(vec![]).await;
    let d_before = dispatch_due_follow_up_reassessments(
        &store,
        auth.as_ref(),
        &practice,
        &workspace,
        now_ts + 300,
        &roles,
        &skills,
        &mut p_empty,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert!(d_before.is_empty());

    // Once `clinical_now >= future_due_at`, scheduled due wakeup MUST dispatch cleanly!
    let (mut p_due, _) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "read-holter-due",
            READ_TOOL,
            &json!({
                "references": [{ "candidate_id": "note-holter-1", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384,
            }),
        )],
        vec![sse_function_call(
            "continue-holter-due",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "continue_unchanged",
                    "rationale": "Reached scheduled 14-day due date; report still pending from external lab.",
                    "quiet": false,
                },
                "cited_records": [{ "candidate_id": "note-holter-1", "revision": "rev-1" }],
                "basis_and_limitations": "Checked at scheduled 14-day due date.",
                "explicit_uncertainty": ["External Holter report still pending."],
                "unfinished_work": [],
            }),
        )],
        vec![sse_assistant_message("Ran scheduled due wakeup.")],
    ])
    .await;
    let d_at_due = dispatch_due_follow_up_reassessments(
        &store,
        auth.as_ref(),
        &practice,
        &workspace,
        future_due_at + 5,
        &roles,
        &skills,
        &mut p_due,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(d_at_due.len(), 1);
}

#[tokio::test]
async fn round1_mid_run_physician_stop_terminal_fences_and_source_change_clears_staged() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let now_ts = current_unix_ts();
    let practice = format!("prac-fu-r1-fence-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, _tok, dr_hash) =
        invite_and_sign_in(&auth, &practice, "dr-fu-r1-fence", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-fu-r1-fence-{}", Uuid::new_v4().simple()),
        display_name: "Simone Pelletier".into(),
        birth_date: Some("1959-12-04".into()),
        sex_or_gender: Some("female".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-FU-FENCE".into(),
        }],
    };
    let workspace = workspace_id("clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let obs = chart_record(
        &workspace,
        "clinic-ehr",
        "obs-ferritin-1",
        "rev-1",
        "Observation",
        "2026-11-10",
        "Serum Ferritin",
        "Ferritin 22 ng/mL.",
    );
    store
        .record_chart(
            &workspace,
            &[obs],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient,
                pulled_at: Some(now_ts),
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
    let fu_stop = FollowUpId::new("fu-mid-stop").unwrap();
    let fu_cancel = FollowUpId::new("fu-mid-cancel").unwrap();
    let fu_src = FollowUpId::new("fu-mid-src").unwrap();
    for id in [&fu_stop, &fu_cancel, &fu_src] {
        store
            .create_follow_up_for_physician(
                &access,
                &FollowUpCommandKey::new(format!("seed-{}", id.as_str())).unwrap(),
                &PhysicianCreateFollowUp {
                    id: id.clone(),
                    summary: FollowUpSummary::new("Recheck ferritin after oral iron").unwrap(),
                    due_at: now_ts - 60,
                    records: vec![ExactRecordReference::new("obs-ferritin-1", "rev-1").unwrap()],
                },
                &mut PostgresWorkspaceAuthority,
            )
            .await
            .unwrap();
    }

    let skills = load_skills();
    let roles = distinct_roles();

    // --- Sub-case A: Physician pauses (`Stop` -> `Held`) mid-run BEFORE `finish` ---
    struct MidRunStopProvider {
        store: Store,
        practice: String,
        physician_id: String,
        workspace: String,
        session_hash: Vec<u8>,
        fu_id: FollowUpId,
        turn: usize,
    }
    impl engine::recipe::Provider for MidRunStopProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "stage-complete-before-stop".into(),
                        name: COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL.into(),
                        arguments: json!({
                            "decision": {
                                "kind": "end_without_completion",
                                "ending_reason": "Would have ended if not paused.",
                                "rationale": "Testing mid-run Held fence.",
                            },
                            "cited_records": [],
                            "basis_and_limitations": "Testing mid-run physician Stop.",
                            "explicit_uncertainty": [],
                            "unfinished_work": [],
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let access = WorkspaceAccess {
                        practice: &self.practice,
                        physician: &self.physician_id,
                        workspace: &self.workspace,
                        session_hash: &self.session_hash,
                    };
                    self.store
                        .stop_follow_up_for_physician(
                            &access,
                            &FollowUpCommandKey::new("midrun-stop-cmd").unwrap(),
                            &self.fu_id,
                            Some(FollowUpChangeNote::new("Paused mid-run by physician").unwrap()),
                            &mut PostgresWorkspaceAuthority,
                        )
                        .await
                        .unwrap();
                    Ok(engine::Turn {
                        items: vec![engine::Item::assistant("Finishing after mid-run Stop.")],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                other => panic!("must abort immediately on Held without looping to turn {other}"),
            }
        }
    }

    let err_held = run_follow_up_reassessment(
        &store,
        auth.as_ref(),
        FollowUpReassessmentRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            follow_up_id: fu_stop.clone(),
            trigger: FollowUpReassessmentTrigger::DueWakeup {
                clinical_now: now_ts,
            },
            review_id: None,
        },
        &roles,
        &skills,
        &mut MidRunStopProvider {
            store: store.clone(),
            practice: practice.clone(),
            physician_id: dr.physician_id.clone(),
            workspace: workspace.clone(),
            session_hash: dr_hash.clone(),
            fu_id: fu_stop.clone(),
            turn: 0,
        },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap_err();
    assert!(
        matches!(
            err_held,
            FollowUpReassessmentError::NotEligible {
                standing: "held",
                ..
            }
        ),
        "expected NotEligible(held), got {err_held:?}"
    );

    // --- Sub-case B: Physician cancels (`Cancelled`) mid-run -> aborts immediately without looping to `RunLimits` ---
    struct MidRunCancelProvider {
        store: Store,
        practice: String,
        physician_id: String,
        workspace: String,
        session_hash: Vec<u8>,
        fu_id: FollowUpId,
        turn: usize,
    }
    impl engine::recipe::Provider for MidRunCancelProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "stage-continue-before-cancel".into(),
                        name: COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL.into(),
                        arguments: json!({
                            "decision": {
                                "kind": "continue_unchanged",
                                "rationale": "Testing mid-run Cancelled fence.",
                                "quiet": true,
                            },
                            "cited_records": [],
                            "basis_and_limitations": "Testing mid-run physician Cancel.",
                            "explicit_uncertainty": [],
                            "unfinished_work": [],
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let access = WorkspaceAccess {
                        practice: &self.practice,
                        physician: &self.physician_id,
                        workspace: &self.workspace,
                        session_hash: &self.session_hash,
                    };
                    self.store
                        .cancel_follow_up_for_physician(
                            &access,
                            &FollowUpCommandKey::new("midrun-cancel-cmd").unwrap(),
                            &self.fu_id,
                            FollowUpChangeNote::new("Cancelled mid-run by physician").unwrap(),
                            &mut PostgresWorkspaceAuthority,
                        )
                        .await
                        .unwrap();
                    Ok(engine::Turn {
                        items: vec![engine::Item::assistant("Finishing after mid-run Cancel.")],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                other => {
                    panic!("must abort immediately on Cancelled without looping to turn {other}")
                }
            }
        }
    }

    let err_cancelled = run_follow_up_reassessment(
        &store,
        auth.as_ref(),
        FollowUpReassessmentRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            follow_up_id: fu_cancel.clone(),
            trigger: FollowUpReassessmentTrigger::DueWakeup {
                clinical_now: now_ts,
            },
            review_id: None,
        },
        &roles,
        &skills,
        &mut MidRunCancelProvider {
            store: store.clone(),
            practice: practice.clone(),
            physician_id: dr.physician_id.clone(),
            workspace: workspace.clone(),
            session_hash: dr_hash.clone(),
            fu_id: fu_cancel.clone(),
            turn: 0,
        },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap_err();
    assert!(
        matches!(
            err_cancelled,
            FollowUpReassessmentError::NotEligible {
                standing: "cancelled",
                ..
            }
        ),
        "expected NotEligible(cancelled), got {err_cancelled:?}"
    );

    // Zero `SavedPatientReview` rows should exist after the aborted Held / Cancelled runs, and standings stay intact!
    let reviews_before_src = store
        .patient_reviews(&practice, &workspace, auth.as_ref())
        .await
        .unwrap();
    assert!(reviews_before_src.is_empty());
    let f_list = store
        .physician_follow_ups(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert!(matches!(
        f_list.iter().find(|f| f.id == fu_stop).unwrap().standing,
        FollowUpStanding::Held { .. }
    ));
    assert!(matches!(
        f_list.iter().find(|f| f.id == fu_cancel).unwrap().standing,
        FollowUpStanding::Cancelled { .. }
    ));

    // --- Sub-case C: `patient_sources_changed` in `finish` clears `staged = None` ---
    struct MidRunSourceChangeProvider {
        store: Store,
        practice: String,
        workspace: String,
        fu_id: FollowUpId,
        turn: usize,
    }
    impl engine::recipe::Provider for MidRunSourceChangeProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            let wire = serde_json::to_string(&request.items).unwrap();
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "stage-before-ferritin-v2".into(),
                        name: COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL.into(),
                        arguments: json!({
                            "decision": {
                                "kind": "continue_unchanged",
                                "rationale": "Staged before repeat ferritin arrived.",
                                "quiet": true,
                            },
                            "cited_records": [],
                            "basis_and_limitations": "Pre-update check.",
                            "explicit_uncertainty": [],
                            "unfinished_work": [],
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let ferritin_v2 = chart_record(
                        &self.workspace,
                        "clinic-ehr",
                        "obs-ferritin-1",
                        "rev-2",
                        "Observation",
                        "2026-11-12",
                        "Repeat Serum Ferritin",
                        "Repeat Ferritin 58 ng/mL (repleted).",
                    );
                    self.store.record_source(&ferritin_v2).await.unwrap();
                    Ok(engine::Turn {
                        items: vec![engine::Item::assistant(
                            "Attempting finish before reading ferritin rev-2.",
                        )],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    assert!(
                        wire.contains("patient_sources_changed"),
                        "expected patient_sources_changed on Turn 3: {wire}"
                    );
                    let runs = self
                        .store
                        .follow_up_reassessment_runs_for_item(
                            &self.practice,
                            &self.workspace,
                            &self.fu_id,
                        )
                        .await
                        .unwrap();
                    assert!(
                        runs[0].staged.is_none(),
                        "staged submission must clear on patient_sources_changed"
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "read-ferritin-v2".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{ "candidate_id": "obs-ferritin-1", "revision": "rev-2", "cursor": null }],
                                "max_result_bytes": 16_384,
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                4 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "complete-ferritin-v2".into(),
                        name: COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL.into(),
                        arguments: json!({
                            "decision": {
                                "kind": "complete",
                                "resolution_note": "Repeat ferritin repleted to 58 ng/mL on 2026-11-12.",
                                "rationale": "Read updated ferritin rev-2 after patient_sources_changed.",
                            },
                            "cited_records": [{ "candidate_id": "obs-ferritin-1", "revision": "rev-2" }],
                            "basis_and_limitations": "Verified repeat ferritin 58 ng/mL (`obs-ferritin-1` @ `rev-2`).",
                            "explicit_uncertainty": [],
                            "unfinished_work": [],
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 4}),
                    usage: Default::default(),
                }),
                5 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant("Completed ferritin follow-up.")],
                    receipt: json!({"turn": 5}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let ok_src = run_follow_up_reassessment(
        &store,
        auth.as_ref(),
        FollowUpReassessmentRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            follow_up_id: fu_src,
            trigger: FollowUpReassessmentTrigger::DueWakeup {
                clinical_now: now_ts,
            },
            review_id: None,
        },
        &roles,
        &skills,
        &mut MidRunSourceChangeProvider {
            store: store.clone(),
            practice: practice.clone(),
            workspace: workspace.clone(),
            fu_id: FollowUpId::new("fu-mid-src").unwrap(),
            turn: 0,
        },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert!(matches!(
        ok_src.run.saved_outcome,
        Some(SavedFollowUpReassessmentOutcome::Completed { .. })
    ));
}

#[tokio::test]
async fn round1_withdrawn_wrong_patient_candidates_excluded_from_reassessment_context_and_staging()
{
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let now_ts = current_unix_ts();
    let practice = format!("prac-fu-r1-wd-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, _tok, dr_hash) =
        invite_and_sign_in(&auth, &practice, "dr-fu-r1-wd", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-fu-r1-wd-{}", Uuid::new_v4().simple()),
        display_name: "Arthur Callahan".into(),
        birth_date: Some("1965-06-15".into()),
        sex_or_gender: Some("male".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-FU-WD".into(),
        }],
    };
    let workspace = workspace_id("clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let clean_psa = chart_record(
        &workspace,
        "clinic-ehr",
        "lab-clean-psa",
        "rev-1",
        "Observation",
        "2026-11-10",
        "Prostate Specific Antigen",
        "PSA 1.2 ng/mL (stable).",
    );
    let wrong_psa = chart_record(
        &workspace,
        "clinic-ehr",
        "lab-wrong-psa",
        "rev-1",
        "Observation",
        "2026-11-10",
        "Misfiled PSA",
        "Wrong-patient PSA 9.8 ng/mL.",
    );
    store
        .record_chart(
            &workspace,
            &[clean_psa, wrong_psa.clone()],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient,
                pulled_at: Some(now_ts),
            },
        )
        .await
        .unwrap();

    store
        .withdraw_wrong_patient_source(&WithdrawWrongPatientRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            withdrawal_id: "wd-fu-r1".into(),
            candidate_ids: vec!["lab-wrong-psa".into()],
            message_id: None,
            fenced_work_ids: Vec::new(),
            reason: "Withdrawn wrong-patient lab".into(),
            actor: dr.physician_id.clone(),
        })
        .await
        .unwrap();
    // Simulate stale source row present alongside active withdrawal boundary:
    store.record_source(&wrong_psa).await.unwrap();

    let access = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &workspace,
        session_hash: &dr_hash,
    };
    let fu_id = FollowUpId::new("fu-r1-wd-psa").unwrap();
    store
        .create_follow_up_for_physician(
            &access,
            &FollowUpCommandKey::new("create-fu-wd-psa").unwrap(),
            &PhysicianCreateFollowUp {
                id: fu_id.clone(),
                summary: FollowUpSummary::new("Verify annual PSA screening").unwrap(),
                due_at: now_ts - 60,
                records: vec![ExactRecordReference::new("lab-clean-psa", "rev-1").unwrap()],
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    let req = FollowUpReassessmentRequest {
        practice: practice.clone(),
        workspace: workspace.clone(),
        follow_up_id: fu_id.clone(),
        trigger: FollowUpReassessmentTrigger::DueWakeup {
            clinical_now: now_ts,
        },
        review_id: Some("rev-fu-wd-preview".into()),
    };
    let (starting_ctx, clean_sources) =
        build_follow_up_reassessment_starting_context(&store, auth.as_ref(), &req)
            .await
            .unwrap();
    assert!(
        clean_sources
            .iter()
            .all(|s| s.candidate_id != "lab-wrong-psa"),
        "clean_sources must exclude withdrawn candidate"
    );
    assert_eq!(starting_ctx.review_context.chart_record_count, 1);

    let skills = load_skills();
    let roles = distinct_roles();
    let (mut p_wd, wd_reqs) = spawn_responses_sse_server(vec![
        vec![
            sse_function_call(
                "read-clean-psa",
                READ_TOOL,
                &json!({
                    "references": [{ "candidate_id": "lab-clean-psa", "revision": "rev-1", "cursor": null }],
                    "max_result_bytes": 16_384,
                }),
            ),
            sse_function_call(
                "cite-withdrawn-psa",
                COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
                &json!({
                    "decision": {
                        "kind": "complete",
                        "resolution_note": "Attempted citing withdrawn record.",
                        "rationale": "Should be refused.",
                    },
                    "cited_records": [{ "candidate_id": "lab-wrong-psa", "revision": "rev-1" }],
                    "basis_and_limitations": "Attempted withdrawn citation.",
                    "explicit_uncertainty": [],
                    "unfinished_work": [],
                }),
            ),
        ],
        vec![sse_function_call(
            "cite-clean-psa",
            COMPLETE_FOLLOW_UP_REASSESSMENT_TOOL,
            &json!({
                "decision": {
                    "kind": "complete",
                    "resolution_note": "2026-11-10 PSA 1.2 ng/mL confirmed stable.",
                    "rationale": "Clean PSA rev-1 verified.",
                },
                "cited_records": [{ "candidate_id": "lab-clean-psa", "revision": "rev-1" }],
                "basis_and_limitations": "Verified clean PSA 1.2 ng/mL (`lab-clean-psa` @ `rev-1`).",
                "explicit_uncertainty": [],
                "unfinished_work": [],
            }),
        )],
        vec![sse_assistant_message("Completed PSA follow-up cleanly.")],
    ])
    .await;

    run_follow_up_reassessment(
        &store,
        auth.as_ref(),
        FollowUpReassessmentRequest {
            practice,
            workspace,
            follow_up_id: fu_id,
            trigger: FollowUpReassessmentTrigger::DueWakeup {
                clinical_now: now_ts,
            },
            review_id: None,
        },
        &roles,
        &skills,
        &mut p_wd,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    let wd_turn2_wire = serde_json::to_string(&wd_reqs.lock().unwrap()[1]["input"]).unwrap();
    assert!(
        wd_turn2_wire.contains("was withdrawn from this patient workspace and cannot be cited"),
        "expected withdrawn citation refusal in stage_reassessment: {wd_turn2_wire}"
    );
}

#[tokio::test]
#[ignore = "requires CODA_ALLOW_MODEL_CALLS=1, live OPENAI_API_KEY, and PHYSICIAN_RUNTIME_TEST_DATABASE_URL"]
async fn live_openai_follow_up_reassessment_smoke() {
    let Some(url) = database_url() else {
        return;
    };
    let allow = std::env::var("CODA_ALLOW_MODEL_CALLS").ok();
    let key = std::env::var("OPENAI_API_KEY").ok();
    let Ok(api_key) = open_ai_api_key(allow.as_deref(), key.as_deref()) else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let now_ts = current_unix_ts();
    let practice = format!("prac-fu-live-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, _tok, dr_hash) =
        invite_and_sign_in(&auth, &practice, "dr-fu-live", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-fu-live-{}", Uuid::new_v4().simple()),
        display_name: "Synthetic Follow-Up Patient".into(),
        birth_date: Some("1968-01-15".into()),
        sex_or_gender: Some("male".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-FU-LIVE".into(),
        }],
    };
    let workspace = workspace_id("clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let rec = chart_record(
        &workspace,
        "clinic-ehr",
        "obs-k-live",
        "rev-1",
        "Observation",
        "2026-11-11",
        "Repeat Serum Potassium",
        "Serum potassium 4.3 mmol/L (normal).",
    );
    store
        .record_chart(
            &workspace,
            &[rec],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient,
                pulled_at: Some(now_ts),
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
    let fu_id = FollowUpId::new("fu-live-k").unwrap();
    store
        .create_follow_up_for_physician(
            &access,
            &FollowUpCommandKey::new("seed-fu-live").unwrap(),
            &PhysicianCreateFollowUp {
                id: fu_id.clone(),
                summary: FollowUpSummary::new("Verify repeat serum potassium normalized").unwrap(),
                due_at: now_ts - 60,
                records: vec![ExactRecordReference::new("obs-k-live", "rev-1").unwrap()],
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    let roles = ModelRoles {
        chat: ModelBinding {
            model: "gpt-5.4-mini".into(),
            effort: "low".into(),
            context_window_tokens: 128_000,
            max_input_tokens: 100_000,
            adapter: "responses".into(),
        },
        background: ModelBinding {
            model: "gpt-5.4".into(),
            effort: "medium".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 240_000,
            adapter: "responses".into(),
        },
    };
    let skills = load_skills();
    let mut provider = ResponsesProvider::new(
        harness::responses::ENDPOINT.to_owned(),
        api_key,
        Arc::new(NoopQuotaGate),
    )
    .unwrap();

    let done = run_follow_up_reassessment(
        &store,
        auth.as_ref(),
        FollowUpReassessmentRequest {
            practice: practice.clone(),
            workspace: workspace.clone(),
            follow_up_id: fu_id,
            trigger: FollowUpReassessmentTrigger::DueWakeup {
                clinical_now: now_ts,
            },
            review_id: None,
        },
        &roles,
        &skills,
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(done.run.status, MessageReviewRunStatus::Completed);
}
