//! PostgreSQL and Responses-transport integration tests for the linked patient
//! message review recipe and physician reply draft revision recipe.
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
use local_product::attachments::{
    AttachmentMediaKind, PagedDocumentInput, READ_DOCUMENT_PAGE_TOOL, SavedAttachmentInput,
    build_test_pdf, ingest_attachment_batch, ingest_paged_document, render_synthetic_clinical_png,
};
use local_product::message_review_recipe::{
    COMPLETE_MESSAGE_REVIEW_TOOL, MESSAGE_REVIEW_GUIDANCE_VERSION, MESSAGE_REVIEW_INSTRUCTIONS,
    MESSAGE_REVIEW_RECIPE_VERSION, MessageReviewInitialLoad, MessageReviewOptions,
    MessageReviewRecipeError, MessageReviewRunStatus, REPLY_REVISION_GUIDANCE_VERSION,
    REPLY_REVISION_RECIPE_VERSION, RecordedToolDiscovery, ToolDiscoveryDisposition,
    message_review_instructions_sha256, recipe as message_review_recipe, reply_revision_recipe,
    run_message_review, run_physician_reply_revision,
};
use local_product::physician_chat::{PhysicianSkills, SKILL_VIEW};
use local_product::physician_sources::{
    MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SourcePatient, SourceRevision,
    workspace_id,
};
use local_product::runtime::chat_operations::{
    MemoryProvenance, MemorySelection, PATIENT_MEMORY_TOOL, PatientMemoryId,
    PatientMemoryOperation, PatientMemoryStanding, PostgresPatientMemoryOperation,
    RecordMemoryCorrection,
};
use local_product::runtime::concerns::{
    CONCERN_TOOL, CommandKey, ConcernStanding, ConcernUrgency, ExactRecordReference,
    WorkspaceAccess,
};
use local_product::runtime::delivery::Delivery;
use local_product::runtime::dispatch::ReviewReceiver;
use local_product::runtime::follow_ups::{
    FOLLOW_UP_TOOL, FollowUpCommandKey, FollowUpId, FollowUpMutation, FollowUpStanding,
    FollowUpSummary, PhysicianCreateFollowUp,
};
use local_product::runtime::jobs::{PracticeJobError, is_physician_only_tool};
use local_product::runtime::refresh::{
    PatientSourceRefresh, REFRESH_TOOL, SourceCheckOutcome, SourceId, SourceRefreshAdapterError,
};
use local_product::runtime::reply::proposals::{
    AskCodaToReplyOutcome, ConversationLink, CopiedReplySource, CopyReplyOutcome,
    DisplayedReplyText, ExpectedDraftBase, MessageSender, NoReplyReason, ObservePracticeMessage,
    PatientMessageStanding, PhysicianDraftKind, PhysicianReplySurface, PracticeMessageKind,
    ProposalRevisionAccepted, RecordPatientMessage, ReplyReviewOutcome,
};
use local_product::runtime::reply::{FINISH_TOOL, RevisionInput};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::spruce_matching::{
    IncomingSpruceItem, OutsideParticipant, ReadableSpruceItem, SenderAddress,
};
use local_product::runtime::today_updates::OpenItemRequest;
use local_product::runtime::{
    Authority, PublicationTransaction, Scope, Store, async_trait, engine,
};
use local_product::spruce_matching_recipe::{
    SUBMIT_CLASSIFICATION_TOOL, run_spruce_message_matching,
};
use local_product::{ModelBinding, ModelRoles, open_ai_api_key, sha256_hex};
use patient_work::ReviewRequest;
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use serde_json::{Value, json};
use uuid::Uuid;

#[allow(dead_code)]
#[path = "../../../verification/model_text.rs"]
mod model_text;

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
            model: "gpt-5.4-pro-background-review".into(),
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

#[derive(Default)]
struct RecordingReviewReceiver {
    requests: Mutex<Vec<ReviewRequest>>,
}

impl RecordingReviewReceiver {
    fn snapshot(&self) -> Vec<ReviewRequest> {
        self.requests.lock().unwrap().clone()
    }
}

#[async_trait]
impl ReviewReceiver for RecordingReviewReceiver {
    async fn submit_review(&self, request: &ReviewRequest) -> Result<(), PracticeJobError> {
        self.requests.lock().unwrap().push(request.clone());
        Ok(())
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
                    "input_tokens": 240,
                    "output_tokens": 80,
                    "total_tokens": 320,
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
    }
}

struct ControllableRefreshAdapter {
    source_id: SourceId,
    source_label: String,
    patient: SourcePatient,
    pulled_at: i64,
    records: Mutex<Vec<SourceRevision>>,
}

#[async_trait]
impl PatientSourceRefresh for ControllableRefreshAdapter {
    async fn configured_sources(
        &self,
        _practice: &str,
        _workspace: &str,
    ) -> Result<Vec<SourceId>, SourceRefreshAdapterError> {
        Ok(vec![self.source_id.clone()])
    }

    async fn check_source(
        &self,
        _practice: &str,
        _workspace: &str,
        source_id: &SourceId,
    ) -> Result<SourceCheckOutcome, SourceRefreshAdapterError> {
        assert_eq!(source_id, &self.source_id);
        let records = self.records.lock().unwrap().clone();
        Ok(SourceCheckOutcome::Updated {
            source_label: self.source_label.clone(),
            patient: self.patient.clone(),
            pulled_at: self.pulled_at,
            records,
        })
    }
}

#[test]
fn recipe_and_reply_revision_guidance_are_clean_and_enforce_tool_boundaries() {
    let skills = load_skills();
    let roles = distinct_roles();
    let (rev_recipe, preloaded) = message_review_recipe(&roles, &skills).unwrap();
    assert_eq!(rev_recipe.model, roles.background.model);
    assert_eq!(rev_recipe.options, json!({ "effort": "high" }));
    assert_eq!(preloaded.len(), 2);
    assert_eq!(preloaded[0].name, "patient-communication");
    assert_eq!(preloaded[1].name, "patient-record-retrieval");

    model_text::assert_clean_model_text(&rev_recipe.instructions);
    model_text::assert_clean_model_text(&serde_json::to_string(&rev_recipe.tools).unwrap());

    for tool in &rev_recipe.tools {
        let name = tool["name"].as_str().unwrap();
        assert!(
            !is_physician_only_tool(name),
            "message review must never offer physician-only tool `{name}`"
        );
    }

    let (draft_rev_recipe, draft_rev_preloaded) = reply_revision_recipe(&roles, &skills).unwrap();
    assert_eq!(draft_rev_recipe.model, roles.background.model);
    assert_eq!(draft_rev_preloaded.len(), 1);
    assert_eq!(draft_rev_preloaded[0].name, "patient-communication");
    assert_eq!(REPLY_REVISION_RECIPE_VERSION, "physician-reply-revision/v1");
    assert_eq!(
        REPLY_REVISION_GUIDANCE_VERSION,
        "physician-reply-revision-guidance/v1"
    );
    assert!(
        !draft_rev_recipe
            .tools
            .iter()
            .any(|t| t["name"] == PATIENT_MEMORY_TOOL),
        "reply draft revision must not offer patient_memory"
    );
    model_text::assert_clean_model_text(&draft_rev_recipe.instructions);
    model_text::assert_clean_model_text(&serde_json::to_string(&draft_rev_recipe.tools).unwrap());
}

#[tokio::test]
async fn ac1_linked_message_and_coalesced_result_runs_on_background_lane_with_preloads_and_discoveries()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let now_ts = current_unix_ts();
    let practice = format!("prac-ac1-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, _token, _hash) = invite_and_sign_in(&auth, &practice, "dr-ac1", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-ac1-{}", Uuid::new_v4().simple()),
        display_name: "Elena Morales".into(),
        birth_date: Some("1964-08-19".into()),
        sex_or_gender: Some("female".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-8841".into(),
        }],
    };
    let workspace = workspace_id("clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let bmp = chart_record(
        &workspace,
        "clinic-ehr",
        "obs-bmp-1",
        "rev-1",
        "Observation",
        "2026-11-10",
        "Basic Metabolic Panel",
        "Potassium 4.4 mmol/L, Creatinine 0.92 mg/dL, eGFR 74 mL/min/1.73m2.",
    );
    store
        .record_chart(
            &workspace,
            &[bmp],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient: patient.clone(),
                pulled_at: now_ts,
            },
        )
        .await
        .unwrap();

    let conv_id = "conv-ac1-coalesced";
    store
        .link_reply_conversation(
            &practice,
            conv_id,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            message_id: "pm-ac1-1".into(),
            sender: MessageSender::Patient,
            text: "My lab notification just popped up—did my potassium and kidney panel look okay?"
                .into(),
            sent_at: now_ts + 10,
        })
        .await
        .unwrap();

    let skills = load_skills();
    let roles = distinct_roles();
    let expected_comm_sha =
        sha256_hex(include_str!("../../../skills/patient-communication/SKILL.md").as_bytes());
    let expected_retr_sha =
        sha256_hex(include_str!("../../../skills/patient-record-retrieval/SKILL.md").as_bytes());

    let (mut provider, captured_requests) = spawn_responses_sse_server(vec![
        vec![
            sse_function_call(
                "call-skill-fu",
                SKILL_VIEW,
                &json!({ "name": "patient-follow-up" }),
            ),
            sse_function_call(
                "call-read-bmp",
                READ_TOOL,
                &json!({
                    "references": [{
                        "candidate_id": "obs-bmp-1",
                        "revision": "rev-1",
                        "cursor": null,
                    }],
                    "max_result_bytes": 32_768,
                }),
            ),
        ],
        vec![sse_function_call(
            "call-complete-ac1",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [],
                "decision": {
                    "kind": "propose_reply",
                    "reply_text": "Your November 10 metabolic panel looks good: your potassium is 4.4 mmol/L and your kidney function is stable. No changes are needed right now.",
                    "basis_summary": "Reviewed 2026-11-10 Clinic EHR Basic Metabolic Panel alongside patient's message.",
                    "limitations": [],
                    "cited_records": [{ "candidate_id": "obs-bmp-1", "revision": "rev-1" }],
                    "addresses_medication_question": false,
                },
                "raised_concern_ids": [],
            }),
        )],
        vec![sse_assistant_message(
            "Completed coalesced message and metabolic panel review.",
        )],
    ])
    .await;

    let completed = run_message_review(
        &store,
        auth.as_ref(),
        &practice,
        conv_id,
        &MessageReviewOptions {
            review_id: Some("rev-ac1-coalesced".into()),
            coalesced_changed_candidates: vec!["obs-bmp-1".into()],
            extra_attachments: Vec::new(),
        },
        &roles,
        &skills,
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("run coalesced message review over Responses SSE");

    // Inspect wire HTTP request sent to `/v1/responses`:
    let wire_reqs = captured_requests.lock().unwrap().clone();
    assert_eq!(wire_reqs.len(), 3);
    assert_eq!(wire_reqs[0]["model"], roles.background.model);
    assert_eq!(wire_reqs[0]["store"], false);
    assert_eq!(wire_reqs[0]["reasoning"], json!({ "effort": "high" }));
    let wire_instructions = wire_reqs[0]["instructions"].as_str().unwrap();
    assert!(wire_instructions.starts_with(MESSAGE_REVIEW_INSTRUCTIONS));
    model_text::assert_clean_model_text(wire_instructions);
    model_text::assert_clean_model_text(&completed.run.starting_context.prompt_text());

    // Inspect persisted provenance, initial loads, and later tool discoveries:
    assert_eq!(completed.run.status, MessageReviewRunStatus::Completed);
    assert_eq!(
        completed.run.provenance.recipe_version,
        MESSAGE_REVIEW_RECIPE_VERSION
    );
    assert_eq!(
        completed.run.provenance.guidance_version,
        MESSAGE_REVIEW_GUIDANCE_VERSION
    );
    assert_eq!(
        completed.run.provenance.guidance_sha256,
        message_review_instructions_sha256()
    );
    assert_eq!(completed.run.provenance.model, roles.background.model);

    assert!(
        completed
            .run
            .initial_loads
            .contains(&MessageReviewInitialLoad::PreloadedSkill {
                name: "patient-communication".into(),
                sha256: expected_comm_sha,
            })
    );
    assert!(
        completed
            .run
            .initial_loads
            .contains(&MessageReviewInitialLoad::PreloadedSkill {
                name: "patient-record-retrieval".into(),
                sha256: expected_retr_sha,
            })
    );
    assert!(completed.run.initial_loads.contains(
        &MessageReviewInitialLoad::CoalescedRecordChanges {
            changed_candidates: vec!["obs-bmp-1".into()],
        }
    ));
    assert!(completed.run.initial_loads.iter().any(|load| matches!(
        load,
        MessageReviewInitialLoad::SkillCatalogMetadata { discoverable_skill_names, .. }
            if discoverable_skill_names.contains(&"patient-follow-up".to_owned())
                && discoverable_skill_names.contains(&"visit-preparation".to_owned())
    )));

    // Later on-demand `skill_view` and `read_patient_sources` live in `tool_discoveries`, separate from `initial_loads`:
    assert!(
        completed
            .run
            .tool_discoveries
            .iter()
            .any(|d: &RecordedToolDiscovery| {
                d.tool_name == SKILL_VIEW
                    && d.discovered_skill_name.as_deref() == Some("patient-follow-up")
                    && d.disposition == ToolDiscoveryDisposition::Returned
            })
    );
    assert!(
        completed
            .run
            .tool_discoveries
            .iter()
            .any(|d: &RecordedToolDiscovery| {
                d.tool_name == READ_TOOL && d.disposition == ToolDiscoveryDisposition::Returned
            })
    );

    let reloaded = store
        .message_review_runs_for_conversation(&practice, conv_id)
        .await
        .unwrap();
    assert_eq!(reloaded.len(), 1);
    assert_eq!(reloaded[0].run_id, completed.run.run_id);
}

#[tokio::test]
async fn ac2_multimodal_photo_paged_pdf_fax_unreadable_limit_and_medication_refresh_discipline() {
    let Some(url) = database_url() else {
        return;
    };
    let (base_store, auth) = connect_stores(&url).await;
    let now_ts = current_unix_ts();
    let practice = format!("prac-ac2-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, _token, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-ac2", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-ac2-{}", Uuid::new_v4().simple()),
        display_name: "Marcus Vance".into(),
        birth_date: Some("1958-02-11".into()),
        sex_or_gender: Some("male".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-2209".into(),
        }],
    };
    let workspace = workspace_id("clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    // 1. Synthetic bottle label PNG + corrupted unreadable sibling PNG:
    let bottle_png = render_synthetic_clinical_png(&["RX LISINOPRIL 10MG", "ONCE DAILY"], false);
    let attachment_batch = ingest_attachment_batch(&[
        SavedAttachmentInput {
            attachment_id: "att-bottle-photo".into(),
            workspace: workspace.clone(),
            source_id: "spruce-attachments".into(),
            revision: "rev-1".into(),
            authored_date: "2026-11-12".into(),
            title: "Pill bottle photo".into(),
            media_kind: AttachmentMediaKind::PngPhoto,
            bytes: bottle_png,
            expected_sha256: None,
        },
        SavedAttachmentInput {
            attachment_id: "att-corrupt-rash".into(),
            workspace: workspace.clone(),
            source_id: "spruce-attachments".into(),
            revision: "rev-1".into(),
            authored_date: "2026-11-12".into(),
            title: "Corrupted second image".into(),
            media_kind: AttachmentMediaKind::PngPhoto,
            bytes: b"not-a-valid-png-payload".to_vec(),
            expected_sha256: None,
        },
    ]);
    assert!(attachment_batch.records[0].outcome.is_ok());
    assert!(attachment_batch.records[1].outcome.is_err());

    // 2. Multi-page PDF cardiology fax:
    let fax_pdf_bytes = build_test_pdf(&[
        (vec!["CARDIOLOGY CONSULT FAX PAGE 1".into()], false, false),
        (
            vec!["CONTINUE LISINOPRIL 20MG DAILY EF 58%".into()],
            false,
            false,
        ),
    ]);
    let ingested_fax = ingest_paged_document(&PagedDocumentInput {
        document_id: "doc-cardiology-fax".into(),
        version: 1,
        workspace: workspace.clone(),
        source_id: "clinic-ehr".into(),
        authored_date: "2026-11-11".into(),
        title: "Cardiology Consultation Fax".into(),
        media_kind: AttachmentMediaKind::SpruceFaxPdf,
        pdf_bytes: fax_pdf_bytes,
        expected_sha256: None,
    })
    .expect("ingest 2-page PDF fax");
    assert_eq!(ingested_fax.page_count, 2);

    let stale_med = chart_record(
        &workspace,
        "clinic-ehr",
        "med-lisinopril",
        "rev-1",
        "MedicationRequest",
        "2026-09-01",
        "Lisinopril 10 mg oral tablet",
        "Lisinopril 10 mg orally once daily (prior September order).",
    );
    base_store
        .record_chart(
            &workspace,
            &[stale_med, ingested_fax.source_revision.clone()],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient: patient.clone(),
                pulled_at: now_ts,
            },
        )
        .await
        .unwrap();
    base_store
        .record_chart(
            &workspace,
            &[
                attachment_batch.records[0].source_revision.clone(),
                attachment_batch.records[1].source_revision.clone(),
            ],
            &SourcePull {
                source_id: "spruce-attachments".into(),
                source_label: "Spruce Attachments".into(),
                patient: patient.clone(),
                pulled_at: now_ts,
            },
        )
        .await
        .unwrap();

    // Configure refresh adapter delivering updated MedicationRequest `rev-2` (20 mg daily):
    let refreshed_med = chart_record(
        &workspace,
        "clinic-ehr",
        "med-lisinopril",
        "rev-2",
        "MedicationRequest",
        "2026-11-11",
        "Lisinopril 20 mg oral tablet",
        "Lisinopril 20 mg orally once daily per Nov 11 cardiology update.",
    );
    let refresh_adapter = Arc::new(ControllableRefreshAdapter {
        source_id: SourceId::new("clinic-ehr").unwrap(),
        source_label: "Clinic EHR".into(),
        patient: patient.clone(),
        pulled_at: now_ts + 30,
        records: Mutex::new(vec![refreshed_med, ingested_fax.source_revision.clone()]),
    });
    let store = base_store.with_source_refresh(refresh_adapter);

    let conv_id = "conv-ac2-multimodal-med";
    store
        .link_reply_conversation(
            &practice,
            conv_id,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            message_id: "pm-ac2-1".into(),
            sender: MessageSender::Patient,
            text: "I attached a photo of my old 10 mg lisinopril bottle and another image—what dose should I take after yesterday's cardiology visit?".into(),
            sent_at: now_ts + 20,
        })
        .await
        .unwrap();

    // Turn 1 attempts medication-question completion BEFORE refreshing medication sources -> must be REFUSED!
    // Turn 2 refreshes sources and reads PDF fax page 2.
    // Turn 3 reads bottle photo + refreshed med + corrupted image (surfacing unreadable status).
    // Turn 4 completes with verified citations and explicit unreadable-image limitation!
    let (mut provider, captured_reqs) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "call-premature-complete",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [],
                "decision": {
                    "kind": "propose_reply",
                    "reply_text": "Keep taking 10 mg daily.",
                    "basis_summary": "Premature attempt before refreshing medications.",
                    "limitations": [],
                    "cited_records": [],
                    "addresses_medication_question": true,
                },
                "raised_concern_ids": [],
            }),
        )],
        vec![
            sse_function_call("call-refresh-meds", REFRESH_TOOL, &json!({})),
            sse_function_call(
                "call-read-fax-p2",
                READ_DOCUMENT_PAGE_TOOL,
                &json!({
                    "document_id": "doc-cardiology-fax",
                    "version": 1,
                    "page_number": 2,
                }),
            ),
        ],
        vec![sse_function_call(
            "call-read-after-refresh",
            READ_TOOL,
            &json!({
                "references": [
                    { "candidate_id": "med-lisinopril", "revision": "rev-2", "cursor": null },
                    { "candidate_id": "att-bottle-photo", "revision": "rev-1", "cursor": null },
                    { "candidate_id": "att-corrupt-rash", "revision": "rev-1", "cursor": null },
                ],
                "max_result_bytes": 32_768,
            }),
        )],
        vec![sse_function_call(
            "call-complete-ac2",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [],
                "decision": {
                    "kind": "propose_reply",
                    "reply_text": "Your November 11 cardiology note updated your lisinopril prescription to 20 mg once daily rather than the 10 mg bottle in your photo. We could not open the second attached image—please resend it if there is anything else you want us to look at.",
                    "basis_summary": "Refreshed Clinic EHR medication list (2026-11-11 Lisinopril 20 mg), read page 2 of the cardiology fax, and inspected the readable bottle label photo.",
                    "limitations": ["Second attached image (`att-corrupt-rash`) was corrupted and unreadable."],
                    "cited_records": [
                        { "candidate_id": "med-lisinopril", "revision": "rev-2" },
                        { "candidate_id": "att-bottle-photo", "revision": "rev-1" },
                        { "candidate_id": "doc-cardiology-fax", "revision": "v1" },
                    ],
                    "addresses_medication_question": true,
                },
                "raised_concern_ids": [],
            }),
        )],
        vec![sse_assistant_message(
            "Completed medication, photo, and cardiology fax review.",
        )],
    ])
    .await;

    let skills = load_skills();
    let roles = distinct_roles();
    let completed = run_message_review(
        &store,
        auth.as_ref(),
        &practice,
        conv_id,
        &MessageReviewOptions::default(),
        &roles,
        &skills,
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("complete multimodal + medication refresh message review");

    // Verify Turn 2 request saw the refusal of `call-premature-complete`:
    let wire = captured_reqs.lock().unwrap().clone();
    let turn2_body = serde_json::to_string(&wire[1]["input"]).unwrap();
    assert!(
        turn2_body.contains("refresh_patient_sources"),
        "medication question completion before refresh must be refused: {turn2_body}"
    );
    // Verify `starting_context.attachments` surfaced the paged fax and image attachments:
    assert!(
        completed
            .run
            .starting_context
            .attachments
            .iter()
            .any(|a| a.candidate_id.as_deref() == Some("doc-cardiology-fax")
                && a.page_count == Some(2)),
        "paged PDF fax must appear in starting_context.attachments: {:?}",
        completed.run.starting_context.attachments
    );

    let access = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &workspace,
        session_hash: &dr_hash,
    };
    let card = store
        .reply_card(&access, conv_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .expect("saved reply card");
    assert_eq!(card.citations.len(), 3);
    let basis = card.basis.as_ref().expect("saved reply card basis");
    assert!(
        basis
            .limits
            .iter()
            .any(|lim| lim.contains("att-corrupt-rash")),
        "saved reply basis must preserve unreadable attachment limit: {:?}",
        basis.limits
    );
}

#[tokio::test]
async fn ac3_four_clinical_cases_routine_nonurgent_concern_urgent_emergency_and_no_reply() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let now_ts = current_unix_ts();
    let practice = format!("prac-ac3-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, _token, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-ac3", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-ac3-{}", Uuid::new_v4().simple()),
        display_name: "Nadia Khouri".into(),
        birth_date: Some("1972-06-03".into()),
        sex_or_gender: Some("female".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-3301".into(),
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
        "note-htn-ac3",
        "rev-1",
        "DocumentReference",
        "2026-11-05",
        "Hypertension Follow-Up Note",
        "Started amlodipine 5 mg daily; lipid panel fasting check planned next month.",
    );
    store
        .record_chart(
            &workspace,
            &[note],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient: patient.clone(),
                pulled_at: now_ts,
            },
        )
        .await
        .unwrap();

    let skills = load_skills();
    let roles = distinct_roles();
    let access = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &workspace,
        session_hash: &dr_hash,
    };

    // --- Case 1: Routine reply (no concern warranted) ---
    let conv_routine = "conv-ac3-routine";
    store
        .link_reply_conversation(
            &practice,
            conv_routine,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_routine.into(),
            message_id: "pm-routine-1".into(),
            sender: MessageSender::Patient,
            text: "Should I fast before my lipid lab check next month?".into(),
            sent_at: now_ts + 10,
        })
        .await
        .unwrap();

    let (mut p1, _) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "read-1",
            READ_TOOL,
            &json!({
                "references": [{ "candidate_id": "note-htn-ac3", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384,
            }),
        )],
        vec![sse_function_call(
            "done-1",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [],
                "decision": {
                    "kind": "propose_reply",
                    "reply_text": "Yes—please fast for 8 to 12 hours (water is fine) before your lipid panel next month.",
                    "basis_summary": "2026-11-05 visit note documents fasting lipid panel planned next month.",
                    "limitations": [],
                    "cited_records": [{ "candidate_id": "note-htn-ac3", "revision": "rev-1" }],
                    "addresses_medication_question": false,
                },
                "raised_concern_ids": [],
            }),
        )],
        vec![sse_assistant_message("Routine fasting question answered.")],
    ])
    .await;
    run_message_review(
        &store,
        auth.as_ref(),
        &practice,
        conv_routine,
        &MessageReviewOptions::default(),
        &roles,
        &skills,
        &mut p1,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert!(
        store
            .workspace_concerns(&access, &mut PostgresWorkspaceAuthority)
            .await
            .unwrap()
            .is_empty(),
        "routine lab question must not raise any concern"
    );

    // --- Case 2: Meaningful nonurgent `Today` concern + reply ---
    let conv_today = "conv-ac3-today";
    store
        .link_reply_conversation(
            &practice,
            conv_today,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_today.into(),
            message_id: "pm-today-1".into(),
            sender: MessageSender::Patient,
            text: "Both of my ankles have been noticeably swollen in the evenings since starting the new blood pressure pill, though I have no shortness of breath or chest discomfort.".into(),
            sent_at: now_ts + 20,
        })
        .await
        .unwrap();

    let (mut p2, _) = spawn_responses_sse_server(vec![
        vec![
            sse_function_call(
                "read-2",
                READ_TOOL,
                &json!({
                    "references": [{ "candidate_id": "note-htn-ac3", "revision": "rev-1", "cursor": null }],
                    "max_result_bytes": 16_384,
                }),
            ),
            sse_function_call(
                "concern-today",
                CONCERN_TOOL,
                &json!({
                    "operation": "raise",
                    "concern_id": "concern-ankle-edema",
                    "urgency": "today",
                    "reason": "Bilateral evening ankle edema reported after starting amlodipine 5 mg daily on 2026-11-05; needs physician medication review.",
                    "records": [{ "candidate_id": "note-htn-ac3", "revision": "rev-1" }],
                }),
            ),
        ],
        vec![sse_function_call(
            "done-2",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [],
                "decision": {
                    "kind": "propose_reply",
                    "reply_text": "Thank you for letting us know about the ankle swelling. Your doctor is reviewing whether to adjust your blood pressure medication and we will follow up with you. If you develop shortness of breath, chest pain, or rapid swelling, seek prompt medical care.",
                    "basis_summary": "Amlodipine 5 mg started 2026-11-05; nonurgent peripheral edema requires physician medication decision.",
                    "limitations": [],
                    "cited_records": [{ "candidate_id": "note-htn-ac3", "revision": "rev-1" }],
                    "addresses_medication_question": false,
                },
                "raised_concern_ids": ["concern-ankle-edema"],
            }),
        )],
        vec![sse_assistant_message("Raised Today concern and proposed reply.")],
    ])
    .await;
    run_message_review(
        &store,
        auth.as_ref(),
        &practice,
        conv_today,
        &MessageReviewOptions::default(),
        &roles,
        &skills,
        &mut p2,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // --- Case 3: Urgent `Now` concern + emergency-care reply (proving non-emergency draft is refused!) ---
    let conv_urgent = "conv-ac3-urgent";
    store
        .link_reply_conversation(
            &practice,
            conv_urgent,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_urgent.into(),
            message_id: "pm-urgent-1".into(),
            sender: MessageSender::Patient,
            text: "I started having crushing chest pressure spreading into my left jaw and sweating 15 minutes ago.".into(),
            sent_at: now_ts + 30,
        })
        .await
        .unwrap();

    let (mut p3, urgent_reqs) = spawn_responses_sse_server(vec![
        vec![
            sse_function_call(
                "read-3",
                READ_TOOL,
                &json!({
                    "references": [{ "candidate_id": "note-htn-ac3", "revision": "rev-1", "cursor": null }],
                    "max_result_bytes": 16_384,
                }),
            ),
            sse_function_call(
                "concern-now",
                CONCERN_TOOL,
                &json!({
                    "operation": "raise",
                    "concern_id": "concern-acute-chest-pain",
                    "urgency": "now",
                    "reason": "Patient messaged reporting acute crushing chest pressure radiating to left jaw with diaphoresis starting 15 minutes ago.",
                    "records": [{ "candidate_id": "note-htn-ac3", "revision": "rev-1" }],
                }),
            ),
        ],
        // First attempt omits emergency care guidance -> MUST be refused by host!
        vec![sse_function_call(
            "done-3-non-emergency",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [],
                "decision": {
                    "kind": "propose_reply",
                    "reply_text": "We have shared your message with the clinic team and will get back to you later today.",
                    "basis_summary": "Acute chest pain message.",
                    "limitations": [],
                    "cited_records": [{ "candidate_id": "note-htn-ac3", "revision": "rev-1" }],
                    "addresses_medication_question": false,
                },
                "raised_concern_ids": ["concern-acute-chest-pain"],
            }),
        )],
        // Corrected draft leads with immediate 911 / emergency department instructions:
        vec![sse_function_call(
            "done-3-emergency",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [],
                "decision": {
                    "kind": "propose_reply",
                    "reply_text": "Please call 911 or have someone take you to the nearest emergency department immediately—do not drive yourself. Chest pressure spreading to your jaw with sweating needs emergency evaluation right now.",
                    "basis_summary": "Acute chest pressure with jaw radiation and diaphoresis; raised urgent `now` concern for immediate physician awareness alongside emergency-care draft.",
                    "limitations": [],
                    "cited_records": [{ "candidate_id": "note-htn-ac3", "revision": "rev-1" }],
                    "addresses_medication_question": false,
                },
                "raised_concern_ids": ["concern-acute-chest-pain"],
            }),
        )],
        vec![sse_assistant_message(
            "Raised urgent Now concern and proposed emergency-care reply.",
        )],
    ])
    .await;
    run_message_review(
        &store,
        auth.as_ref(),
        &practice,
        conv_urgent,
        &MessageReviewOptions::default(),
        &roles,
        &skills,
        &mut p3,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let urgent_wire = urgent_reqs.lock().unwrap().clone();
    let refusal_wire = serde_json::to_string(&urgent_wire[2]["input"]).unwrap();
    assert!(
        refusal_wire.contains("911") || refusal_wire.contains("emergency"),
        "non-emergency reply must be refused when raising a `now` concern: {refusal_wire}"
    );

    let concerns = store
        .workspace_concerns(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    let by_id: BTreeMap<_, _> = concerns
        .iter()
        .map(|c| (c.id.as_str(), (c.urgency, &c.standing)))
        .collect();
    assert_eq!(
        by_id.get("concern-ankle-edema"),
        Some(&(ConcernUrgency::Today, &ConcernStanding::Open))
    );
    assert_eq!(
        by_id.get("concern-acute-chest-pain"),
        Some(&(ConcernUrgency::Now, &ConcernStanding::Open))
    );
    let urgent_card = store
        .reply_card(&access, conv_urgent, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        &urgent_card.displayed_text,
        DisplayedReplyText::CodaProposal { text, .. } if text.contains("call 911")
    ));

    // --- Case 4: `NoReplyWouldHelp` ---
    let conv_closing = "conv-ac3-no-reply";
    store
        .link_reply_conversation(
            &practice,
            conv_closing,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_closing.into(),
            message_id: "pm-closing-1".into(),
            sender: MessageSender::Patient,
            text: "Sounds good, thanks!".into(),
            sent_at: now_ts + 40,
        })
        .await
        .unwrap();

    let (mut p4, _) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "done-4",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [],
                "decision": {
                    "kind": "no_reply_would_help",
                    "reason": "Brief closing acknowledgement with no open question or clinical symptom.",
                    "basis_summary": "Reviewed closing thank-you message in context.",
                    "limitations": [],
                },
                "raised_concern_ids": [],
            }),
        )],
        vec![sse_assistant_message("Recorded no reply needed.")],
    ])
    .await;
    run_message_review(
        &store,
        auth.as_ref(),
        &practice,
        conv_closing,
        &MessageReviewOptions::default(),
        &roles,
        &skills,
        &mut p4,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert!(
        store
            .reply_card(&access, conv_closing, &mut PostgresWorkspaceAuthority)
            .await
            .unwrap()
            .is_none()
    );
    let conv_state = store
        .reply_conversation(&practice, conv_closing)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        &conv_state.patient_messages[0].standing,
        PatientMessageStanding::NeedsNoReply {
            reason: NoReplyReason::ReviewJudgedNoReplyWouldHelp { reason, .. },
            ..
        } if reason.contains("Brief closing acknowledgement")
    ));
}

#[tokio::test]
async fn ac4_longitudinal_memory_and_follow_up_updates_carry_into_later_run_preserving_siblings_and_no_change()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let now_ts = current_unix_ts();
    let practice = format!("prac-ac4-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, _token, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-ac4", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-ac4-{}", Uuid::new_v4().simple()),
        display_name: "Hannah Sterling".into(),
        birth_date: Some("1969-11-02".into()),
        sex_or_gender: Some("female".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-4410".into(),
        }],
    };
    let workspace = workspace_id("clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let bp_note = chart_record(
        &workspace,
        "clinic-ehr",
        "note-bp-ac4",
        "rev-1",
        "DocumentReference",
        "2026-11-08",
        "Hypertension Home Monitoring Plan",
        "Check home blood pressure twice weekly and recheck metabolic panel in 2 weeks.",
    );
    store
        .record_chart(
            &workspace,
            &[bp_note],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient: patient.clone(),
                pulled_at: now_ts,
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

    // Seed unrelated memory (`mem-penicillin-rash`) + two active follow-ups (`fu-bp-log` and `fu-lipid-annual`):
    PostgresPatientMemoryOperation
        .record_correction(
            &store,
            "seed-mem-allergy",
            &RecordMemoryCorrection {
                practice: practice.clone(),
                workspace: workspace.clone(),
                id: PatientMemoryId::new("mem-penicillin-rash").unwrap(),
                topic: "Allergy clarification".into(),
                statement: "Childhood amoxicillin rash was non-urticarial.".into(),
                provenance: MemoryProvenance::PhysicianCorrection {
                    physician: dr.physician_id.clone(),
                    chat_id: "seed-chat".into(),
                    message_id: "seed-msg-1".into(),
                    work_id: "seed-work-1".into(),
                    attempt_id: "seed-attempt-1".into(),
                },
            },
        )
        .await
        .unwrap();

    for (id, summary, due_at) in [
        (
            "fu-bp-log",
            "Review two-week home blood pressure log",
            now_ts + 86_400 * 7,
        ),
        (
            "fu-lipid-annual",
            "Check annual fasting lipid panel",
            now_ts + 86_400 * 30,
        ),
    ] {
        let m = store
            .create_follow_up_for_physician(
                &access,
                &FollowUpCommandKey::new(format!("seed-{id}")).unwrap(),
                &PhysicianCreateFollowUp {
                    id: FollowUpId::new(id).unwrap(),
                    summary: FollowUpSummary::new(summary).unwrap(),
                    due_at,
                    records: vec![ExactRecordReference::new("note-bp-ac4", "rev-1").unwrap()],
                },
                &mut PostgresWorkspaceAuthority,
            )
            .await
            .unwrap();
        assert!(matches!(m, FollowUpMutation::Applied(_)));
    }

    let conv_id = "conv-ac4-longitudinal";
    store
        .link_reply_conversation(
            &practice,
            conv_id,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();

    let skills = load_skills();
    let roles = distinct_roles();

    // --- Run 1: Patient reports Spanish written preference and travels until Nov 24 -> updates memory + `fu-bp-log` ---
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            message_id: "pm-ac4-1".into(),
            sender: MessageSender::Patient,
            text: "I am traveling without my home cuff until November 24 and prefer Spanish written instructions when possible—can I send my BP log right when I get back?".into(),
            sent_at: now_ts + 100,
        })
        .await
        .unwrap();

    let new_bp_due_at = now_ts + 86_400 * 14;
    let (mut p_run1, _) = spawn_responses_sse_server(vec![
        vec![
            sse_function_call(
                "read-note-1",
                READ_TOOL,
                &json!({
                    "references": [{ "candidate_id": "note-bp-ac4", "revision": "rev-1", "cursor": null }],
                    "max_result_bytes": 16_384,
                }),
            ),
            sse_function_call(
                "mem-lang",
                PATIENT_MEMORY_TOOL,
                &json!({
                    "operation": "record_correction",
                    "memory_id": "mem-spanish-written-pref",
                    "topic": "Communication preference",
                    "statement": "Patient requested Spanish written instructions when possible.",
                }),
            ),
            sse_function_call(
                "fu-update-bp",
                FOLLOW_UP_TOOL,
                &json!({
                    "operation": "change",
                    "follow_up_id": "fu-bp-log",
                    "summary": "Review home blood pressure log after patient returns from travel Nov 24",
                    "due_at": new_bp_due_at,
                    "records": [{ "candidate_id": "note-bp-ac4", "revision": "rev-1" }],
                    "note": "Patient traveling without cuff until Nov 24; moved reassessment to 14 days.",
                }),
            ),
        ],
        vec![sse_function_call(
            "complete-run1",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [],
                "decision": {
                    "kind": "propose_reply",
                    "reply_text": "Sí, claro—puede enviarnos su registro de presión arterial cuando regrese el 24 de noviembre. Si presenta dolor de cabeza intenso, dolor en el pecho o falta de aire durante el viaje, busque atención médica de inmediato.",
                    "basis_summary": "Updated BP follow-up window to post-travel date and saved Spanish written preference.",
                    "limitations": [],
                    "cited_records": [{ "candidate_id": "note-bp-ac4", "revision": "rev-1" }],
                    "addresses_medication_question": false,
                },
                "raised_concern_ids": [],
            }),
        )],
        vec![sse_assistant_message("Updated memory and BP follow-up.")],
    ])
    .await;

    run_message_review(
        &store,
        auth.as_ref(),
        &practice,
        conv_id,
        &MessageReviewOptions::default(),
        &roles,
        &skills,
        &mut p_run1,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // --- Run 2: Later message arrives -> verify starting_context carries BOTH updated memory & updated follow-up, and siblings are intact! ---
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            message_id: "pm-ac4-2".into(),
            sender: MessageSender::Patient,
            text: "¿Debo tomar lecturas por la mañana y por la noche cuando vuelva?".into(),
            sent_at: now_ts + 200,
        })
        .await
        .unwrap();

    let (mut p_run2, run2_reqs) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "read-note-2",
            READ_TOOL,
            &json!({
                "references": [{ "candidate_id": "note-bp-ac4", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384,
            }),
        )],
        vec![sse_function_call(
            "complete-run2",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [],
                "decision": {
                    "kind": "propose_reply",
                    "reply_text": "Sí—cuando regrese el 24 de noviembre, tome su presión arterial dos veces por semana (por la mañana antes de sus medicamentos y por la noche) y envíenos los valores.",
                    "basis_summary": "Used saved Spanish preference and post-travel Nov 24 BP follow-up context without modifying durable memory or follow-ups.",
                    "limitations": [],
                    "cited_records": [{ "candidate_id": "note-bp-ac4", "revision": "rev-1" }],
                    "addresses_medication_question": false,
                },
                "raised_concern_ids": [],
            }),
        )],
        vec![sse_assistant_message(
            "Completed Run 2 with no-change decision on memory/follow-ups.",
        )],
    ])
    .await;

    let completed_2 = run_message_review(
        &store,
        auth.as_ref(),
        &practice,
        conv_id,
        &MessageReviewOptions::default(),
        &roles,
        &skills,
        &mut p_run2,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Verify Run 2's starting context received both Run 1's memory update and follow-up update, plus unrelated siblings intact:
    let PatientMemoryStanding::Available { ref entries } =
        completed_2.run.starting_context.memory_standing
    else {
        panic!("expected available memory standing");
    };
    let mem_ids: Vec<&str> = entries.iter().map(|e| e.id.as_str()).collect();
    assert!(mem_ids.contains(&"mem-penicillin-rash"));
    assert!(mem_ids.contains(&"mem-spanish-written-pref"));

    let fu_by_id: BTreeMap<_, _> = completed_2
        .run
        .starting_context
        .active_follow_ups
        .iter()
        .map(|f| (f.id.as_str(), (f.version, f.due_at, f.summary.as_str())))
        .collect();
    assert_eq!(
        fu_by_id.get("fu-bp-log"),
        Some(&(
            2,
            new_bp_due_at,
            "Review home blood pressure log after patient returns from travel Nov 24"
        ))
    );
    assert_eq!(
        fu_by_id.get("fu-lipid-annual"),
        Some(&(1, now_ts + 86_400 * 30, "Check annual fasting lipid panel"))
    );

    let wire2 = serde_json::to_string(&run2_reqs.lock().unwrap()[0]["input"]).unwrap();
    assert!(wire2.contains("mem-spanish-written-pref"));
    assert!(wire2.contains("returns from travel Nov 24"));

    // Verify Run 2's no-change decision left memory and follow-ups unchanged afterwards:
    let post_memories = PostgresPatientMemoryOperation
        .list_memories(&store, &practice, &workspace, MemorySelection::ActiveOnly)
        .await
        .unwrap();
    assert_eq!(post_memories.len(), 2);
    let post_follow_ups = store
        .physician_follow_ups(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(post_follow_ups.len(), 2);
    assert_eq!(
        post_follow_ups
            .iter()
            .find(|f| f.id.as_str() == "fu-bp-log")
            .unwrap()
            .version,
        2
    );
    assert_eq!(
        post_follow_ups
            .iter()
            .find(|f| f.id.as_str() == "fu-lipid-annual")
            .unwrap()
            .version,
        1
    );
}

#[tokio::test]
async fn ac5_physician_reply_revision_reconsiders_moved_sources_preserves_concurrent_physician_edit_and_fences_tools()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let now_ts = current_unix_ts();
    let practice = format!("prac-ac5-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, _token, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-ac5", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-ac5-{}", Uuid::new_v4().simple()),
        display_name: "David Cho".into(),
        birth_date: Some("1975-04-12".into()),
        sex_or_gender: Some("male".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-5512".into(),
        }],
    };
    let workspace = workspace_id("clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let lab_v1 = chart_record(
        &workspace,
        "clinic-ehr",
        "lab-k-ac5",
        "rev-1",
        "Observation",
        "2026-11-10",
        "Serum Potassium",
        "Potassium 5.2 mmol/L (mildly hemolyzed specimen).",
    );
    store
        .record_chart(
            &workspace,
            &[lab_v1],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient: patient.clone(),
                pulled_at: now_ts,
            },
        )
        .await
        .unwrap();

    let conv_id = "conv-ac5-revision";
    store
        .link_reply_conversation(
            &practice,
            conv_id,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            message_id: "pm-ac5-1".into(),
            sender: MessageSender::Patient,
            text: "How did my potassium lab look?".into(),
            sent_at: now_ts + 10,
        })
        .await
        .unwrap();

    let skills = load_skills();
    let roles = distinct_roles();
    let (mut p_init, _) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "read-k",
            READ_TOOL,
            &json!({
                "references": [{ "candidate_id": "lab-k-ac5", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384,
            }),
        )],
        vec![sse_function_call(
            "done-init",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [],
                "decision": {
                    "kind": "propose_reply",
                    "reply_text": "Your potassium came back slightly borderline at 5.2 mmol/L on a mildly hemolyzed sample.",
                    "basis_summary": "2026-11-10 potassium 5.2 mmol/L.",
                    "limitations": [],
                    "cited_records": [{ "candidate_id": "lab-k-ac5", "revision": "rev-1" }],
                    "addresses_medication_question": false,
                },
                "raised_concern_ids": [],
            }),
        )],
        vec![sse_assistant_message("Initial proposal saved.")],
    ])
    .await;
    run_message_review(
        &store,
        auth.as_ref(),
        &practice,
        conv_id,
        &MessageReviewOptions::default(),
        &roles,
        &skills,
        &mut p_init,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let access = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &workspace,
        session_hash: &dr_hash,
    };
    store
        .edit_reply_proposal_draft(
            &access,
            conv_id,
            ExpectedDraftBase::ProposalVersion(1),
            "Your potassium was 5.2 on a hemolyzed draw; let's recheck next week.",
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    let (rev_recipe, _) = reply_revision_recipe(&roles, &skills).unwrap();
    let ProposalRevisionAccepted::New { scope: draft_scope } = store
        .request_reply_draft_revision(
            &access,
            conv_id,
            &RevisionInput {
                key: "rev-req-1".into(),
                expected_revision: 1,
                instruction: "Check if the repeat potassium landed and make the tone reassuring."
                    .into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            &skills,
            &rev_recipe,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap()
    else {
        panic!("expected New revision acceptance");
    };

    let delivery_1 = Delivery::reply_revision(draft_scope.clone(), "rev-req-1".into());

    // Custom provider that tests:
    // 1) out-of-authority `patient_memory` is refused on the wire, and
    // 2) a mid-revision chart update (`rev-2`, potassium 4.3 mmol/L) triggers `SourcesMoved` reconsideration!
    struct MidRevisionSourceMoveProvider {
        store: Store,
        workspace: String,
        turn: usize,
    }

    impl engine::recipe::Provider for MidRevisionSourceMoveProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "illegal-mem-in-rev".into(),
                        name: PATIENT_MEMORY_TOOL.into(),
                        arguments: json!({
                            "operation": "record_correction",
                            "memory_id": "illegal",
                            "topic": "Should fail",
                            "statement": "Revision must not write memory",
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let wire = serde_json::to_string(&request.items).unwrap();
                    assert!(
                        wire.contains("Refused")
                            || wire.contains("refused")
                            || wire.contains("unknown")
                            || wire.contains("does not exist")
                            || wire.contains("\"executed\":false")
                            || wire.contains("outside physician reply revision authority"),
                        "reply revision must refuse patient_memory: {wire}"
                    );
                    // Inject repeat potassium `rev-2` mid-revision BEFORE `commit_revision`!
                    let lab_v2 = chart_record(
                        &self.workspace,
                        "clinic-ehr",
                        "lab-k-ac5",
                        "rev-2",
                        "Observation",
                        "2026-11-12",
                        "Repeat Serum Potassium",
                        "Repeat non-hemolyzed Potassium 4.3 mmol/L (normal).",
                    );
                    self.store.record_source(&lab_v2).await.unwrap();

                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "finish-before-reading-v2".into(),
                            name: FINISH_TOOL.into(),
                            arguments: json!({
                                "text": "Your potassium was 5.2 on a hemolyzed sample; please recheck next week.",
                                "review_basis": "Drafted before reading repeat potassium rev-2.",
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    // `commit_revision` caught `SourcesMoved`! Now read `lab-k-ac5` @ `rev-2`:
                    let wire = serde_json::to_string(&request.items).unwrap();
                    assert!(
                        wire.contains("lab-k-ac5") && wire.contains("rev-2"),
                        "reconsideration feedback must identify moved record rev-2: {wire}"
                    );
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "read-lab-v2".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{ "candidate_id": "lab-k-ac5", "revision": "rev-2", "cursor": null }],
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
                        id: "finish-after-v2".into(),
                        name: FINISH_TOOL.into(),
                        arguments: json!({
                            "text": "Good news—your repeat non-hemolyzed potassium on November 12 is completely normal at 4.3 mmol/L, so no repeat draw is needed.",
                            "review_basis": "Incorporated updated 2026-11-12 repeat potassium (4.3 mmol/L, rev-2).",
                        })
                        .to_string(),
                    })],
                    receipt: json!({"turn": 4}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected revision turn {other}"),
            }
        }
    }

    let rev_done = run_physician_reply_revision(
        &store,
        &mut PostgresWorkspaceAuthority,
        &delivery_1,
        60,
        &roles,
        &skills,
        &mut MidRevisionSourceMoveProvider {
            store: store.clone(),
            workspace: workspace.clone(),
            turn: 0,
        },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(rev_done.source_reconsiderations, 1);
    assert_eq!(rev_done.status, MessageReviewRunStatus::Completed);
    assert!(
        rev_done
            .revised_text
            .as_deref()
            .unwrap_or_default()
            .contains("4.3 mmol/L")
    );

    let card_after_rev = store
        .reply_card(&access, conv_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        &card_after_rev.displayed_text,
        DisplayedReplyText::PhysicianDraft {
            draft_revision: 2,
            kind: PhysicianDraftKind::CodaRevision { pre_revision_your_text },
            text,
            ..
        } if pre_revision_your_text.contains("5.2 on a hemolyzed draw")
            && text.contains("normal at 4.3 mmol/L")
    ));

    // --- Sub-case B: Concurrent mid-flight physician edit while revision is leased -> preserves `YourText` (`DraftStaysYours`) ---
    let ProposalRevisionAccepted::New { scope: scope_2 } = store
        .request_reply_draft_revision(
            &access,
            conv_id,
            &RevisionInput {
                key: "rev-req-concurrent".into(),
                expected_revision: 2,
                instruction: "Shorten to one sentence.".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            &skills,
            &rev_recipe,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap()
    else {
        panic!("expected second revision acceptance");
    };
    let delivery_2 = Delivery::reply_revision(scope_2, "rev-req-concurrent".into());

    struct ConcurrentPhysicianEditProvider {
        store: Store,
        practice: String,
        physician_id: String,
        workspace: String,
        session_hash: Vec<u8>,
        conv_id: String,
    }

    impl engine::recipe::Provider for ConcurrentPhysicianEditProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            let access = WorkspaceAccess {
                practice: &self.practice,
                physician: &self.physician_id,
                workspace: &self.workspace,
                session_hash: &self.session_hash,
            };
            self.store
                .edit_reply_proposal_draft(
                    &access,
                    &self.conv_id,
                    ExpectedDraftBase::DraftRevision(2),
                    "Physician's own concurrent wording: repeat potassium is normal (4.3), see you in clinic.",
                    &mut PostgresWorkspaceAuthority,
                )
                .await
                .unwrap();

            Ok(engine::Turn {
                items: vec![engine::Item::Call(engine::Invocation {
                    id: "superseded-finish".into(),
                    name: FINISH_TOOL.into(),
                    arguments: json!({
                        "text": "Shortened stale revision text that must never overwrite physician's edit.",
                        "review_basis": "Stale revision attempt.",
                    })
                    .to_string(),
                })],
                receipt: json!({"turn": 1}),
                usage: Default::default(),
            })
        }
    }

    let superseded_err = run_physician_reply_revision(
        &store,
        &mut PostgresWorkspaceAuthority,
        &delivery_2,
        60,
        &roles,
        &skills,
        &mut ConcurrentPhysicianEditProvider {
            store: store.clone(),
            practice: practice.clone(),
            physician_id: dr.physician_id.clone(),
            workspace: workspace.clone(),
            session_hash: dr_hash.clone(),
            conv_id: conv_id.into(),
        },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect_err("concurrent physician edit must supersede in-flight revision");
    assert!(matches!(
        superseded_err,
        MessageReviewRecipeError::Harness { .. }
    ));

    let card_preserved = store
        .reply_card(&access, conv_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        &card_preserved.displayed_text,
        DisplayedReplyText::PhysicianDraft {
            draft_revision: 3,
            kind: PhysicianDraftKind::YourText,
            text,
            ..
        } if text.contains("Physician's own concurrent wording")
    ));
}

#[tokio::test]
async fn ac6_already_handled_automatic_vs_real_partial_closures_ask_coda_copy_and_close_orthogonality()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let now_ts = current_unix_ts();
    let practice = format!("prac-ac6-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, _token, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-ac6", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-ac6-{}", Uuid::new_v4().simple()),
        display_name: "Sonia Alvarez".into(),
        birth_date: Some("1981-09-25".into()),
        sex_or_gender: Some("female".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-6602".into(),
        }],
    };
    let workspace = workspace_id("clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let chart_note = chart_record(
        &workspace,
        "clinic-ehr",
        "note-ac6",
        "rev-1",
        "DocumentReference",
        "2026-11-09",
        "Clinic Visit Summary",
        "Vitamin D3 2000 IU daily; repeat 25-OH Vitamin D in 3 months.",
    );
    store
        .record_chart(
            &workspace,
            &[chart_note],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient: patient.clone(),
                pulled_at: now_ts,
            },
        )
        .await
        .unwrap();

    let skills = load_skills();
    let roles = distinct_roles();
    let access = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &workspace,
        session_hash: &dr_hash,
    };

    // 1. Automatic reply vs real practice reply + partial closure:
    let conv_partial = "conv-ac6-partial";
    store
        .link_reply_conversation(
            &practice,
            conv_partial,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_partial.into(),
            message_id: "m1-parking".into(),
            sender: MessageSender::Patient,
            text: "Where do I park for the lab draw?".into(),
            sent_at: now_ts + 10,
        })
        .await
        .unwrap();
    store
        .observe_practice_message(&ObservePracticeMessage {
            practice: practice.clone(),
            conversation_id: conv_partial.into(),
            practice_message_id: "auto-ack-1".into(),
            kind: PracticeMessageKind::Automatic,
            text: "Thanks for messaging our clinic; we respond within 1 business day.".into(),
            sent_at: now_ts + 12,
        })
        .await
        .unwrap();
    store
        .observe_practice_message(&ObservePracticeMessage {
            practice: practice.clone(),
            conversation_id: conv_partial.into(),
            practice_message_id: "staff-parking-1".into(),
            kind: PracticeMessageKind::PracticeSent,
            text: "Parking is validated in Garage B right behind the lab.".into(),
            sent_at: now_ts + 20,
        })
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_partial.into(),
            message_id: "m2-vitd".into(),
            sender: MessageSender::Patient,
            text: "Thanks! Also, how much Vitamin D3 should I take each day?".into(),
            sent_at: now_ts + 30,
        })
        .await
        .unwrap();

    // Turn 1 tries closing `m1-parking` and `m2-vitd` with `auto-ack-1` (`Automatic`) -> REFUSED!
    // Turn 2 reads chart + raises a concern + creates a follow-up.
    // Turn 3 closes `m1-parking` with `staff-parking-1` (`PracticeSent`) and proposes reply for `m2-vitd`!
    let (mut p_partial, partial_reqs) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "bad-auto-close",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [
                    {
                        "practice_message_id": "auto-ack-1",
                        "answered_message_ids": ["m1-parking", "m2-vitd"],
                    }
                ],
                "decision": {
                    "kind": "all_handled",
                    "basis_summary": "Tried closing with automatic acknowledgement.",
                },
                "raised_concern_ids": [],
            }),
        )],
        vec![
            sse_function_call(
                "read-vitd",
                READ_TOOL,
                &json!({
                    "references": [{ "candidate_id": "note-ac6", "revision": "rev-1", "cursor": null }],
                    "max_result_bytes": 16_384,
                }),
            ),
            sse_function_call(
                "raise-c-ac6",
                CONCERN_TOOL,
                &json!({
                    "operation": "raise",
                    "concern_id": "concern-ac6-orthog",
                    "urgency": "today",
                    "reason": "Confirm Vitamin D recheck lab order is entered for 3 months.",
                    "records": [{ "candidate_id": "note-ac6", "revision": "rev-1" }],
                }),
            ),
            sse_function_call(
                "create-fu-ac6",
                FOLLOW_UP_TOOL,
                &json!({
                    "operation": "create",
                    "follow_up_id": "fu-ac6-vitd",
                    "summary": "Check 3-month 25-OH Vitamin D lab result",
                    "due_at": now_ts + 86_400 * 90,
                    "records": [{ "candidate_id": "note-ac6", "revision": "rev-1" }],
                }),
            ),
        ],
        vec![sse_function_call(
            "good-partial-close",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [
                    {
                        "practice_message_id": "staff-parking-1",
                        "answered_message_ids": ["m1-parking"],
                    }
                ],
                "decision": {
                    "kind": "propose_reply",
                    "reply_text": "Your November 9 visit plan is Vitamin D3 2,000 IU once daily, with a repeat Vitamin D lab check in 3 months.",
                    "basis_summary": "Front desk answered parking (`m1-parking`); proposed reply for open Vitamin D dosing question (`m2-vitd`).",
                    "limitations": [],
                    "cited_records": [{ "candidate_id": "note-ac6", "revision": "rev-1" }],
                    "addresses_medication_question": false,
                },
                "raised_concern_ids": ["concern-ac6-orthog"],
            }),
        )],
        vec![sse_assistant_message(
            "Closed m1-parking and proposed reply for m2-vitd.",
        )],
    ])
    .await;

    run_message_review(
        &store,
        auth.as_ref(),
        &practice,
        conv_partial,
        &MessageReviewOptions::default(),
        &roles,
        &skills,
        &mut p_partial,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let partial_wire = serde_json::to_string(&partial_reqs.lock().unwrap()[1]["input"]).unwrap();
    assert!(
        partial_wire.contains("automatic") || partial_wire.contains("auto-ack-1"),
        "Automatic practice message must be refused as closure: {partial_wire}"
    );

    // Prove `copy_reply` records attribution WITHOUT marking `m2-vitd` `Answered`:
    let CopyReplyOutcome::Recorded(copy_rec) = store
        .copy_reply(
            &access,
            &CommandKey::new("copy-ac6").unwrap(),
            conv_partial,
            ExpectedDraftBase::ProposalVersion(1),
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap()
    else {
        panic!("expected Recorded copy");
    };
    assert_eq!(
        copy_rec.copied_source,
        CopiedReplySource::CodaProposal {
            reply_id: 1,
            version: 1,
        }
    );
    let after_copy = store
        .reply_conversation(&practice, conv_partial)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        after_copy.patient_messages[0].standing,
        PatientMessageStanding::Answered { ref practice_message_id, .. }
            if practice_message_id == "staff-parking-1"
    ));
    assert_eq!(
        after_copy.patient_messages[1].standing,
        PatientMessageStanding::Open,
        "copy_reply must never mark open patient messages Answered before channel delivery"
    );

    // Prove `close_reply_no_reply_needed` closes ONLY the reply card while leaving open concerns and active follow-ups untouched!
    store
        .close_reply_no_reply_needed(
            &access,
            &CommandKey::new("close-nrn-ac6").unwrap(),
            conv_partial,
            1,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    let concerns_after_close = store
        .workspace_concerns(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(concerns_after_close.len(), 1);
    assert_eq!(concerns_after_close[0].standing, ConcernStanding::Open);
    let follow_ups_after_close = store
        .physician_follow_ups(&access, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(follow_ups_after_close.len(), 1);
    assert!(matches!(
        follow_ups_after_close[0].standing,
        FollowUpStanding::Active { .. }
    ));

    // 2. `Ask Coda to reply` reopens settled `NoReplyWouldHelp` and refuses `NoReplyWouldHelp` on the requested run:
    let conv_ask = "conv-ac6-ask-coda";
    store
        .link_reply_conversation(
            &practice,
            conv_ask,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_ask.into(),
            message_id: "m-thanks-ask".into(),
            sender: MessageSender::Patient,
            text: "Got it, appreciate the help!".into(),
            sent_at: now_ts + 60,
        })
        .await
        .unwrap();

    let (mut p_nr, _) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "nr-1",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [],
                "decision": {
                    "kind": "no_reply_would_help",
                    "reason": "Courtesy thank-you message.",
                    "basis_summary": "No open clinical question.",
                    "limitations": [],
                },
                "raised_concern_ids": [],
            }),
        )],
        vec![sse_assistant_message("Settled as no reply needed.")],
    ])
    .await;
    run_message_review(
        &store,
        auth.as_ref(),
        &practice,
        conv_ask,
        &MessageReviewOptions::default(),
        &roles,
        &skills,
        &mut p_nr,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let ask_out = store
        .ask_coda_to_reply(
            &access,
            &CommandKey::new("ask-coda-ac6").unwrap(),
            conv_ask,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert!(matches!(
        ask_out,
        AskCodaToReplyOutcome::QueuedReview { .. }
    ));

    let (mut p_asked, asked_reqs) = spawn_responses_sse_server(vec![
        // Turn 1 tries `no_reply_would_help` -> MUST be refused because physician clicked `Ask Coda to reply`!
        vec![sse_function_call(
            "refused-nr",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [],
                "decision": {
                    "kind": "no_reply_would_help",
                    "reason": "Still just thanks.",
                    "basis_summary": "Attempted no-reply after physician request.",
                    "limitations": [],
                },
                "raised_concern_ids": [],
            }),
        )],
        vec![sse_function_call(
            "propose-after-ask",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [],
                "decision": {
                    "kind": "propose_reply",
                    "reply_text": "You're very welcome—please message us anytime if questions come up.",
                    "basis_summary": "Drafted closing acknowledgement requested by physician.",
                    "limitations": [],
                    "cited_records": [],
                    "addresses_medication_question": false,
                },
                "raised_concern_ids": [],
            }),
        )],
        vec![sse_assistant_message("Proposed physician-requested reply.")],
    ])
    .await;
    run_message_review(
        &store,
        auth.as_ref(),
        &practice,
        conv_ask,
        &MessageReviewOptions::default(),
        &roles,
        &skills,
        &mut p_asked,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let asked_wire = serde_json::to_string(&asked_reqs.lock().unwrap()[1]["input"]).unwrap();
    assert!(
        asked_wire.contains("Ask Coda to reply") || asked_wire.contains("propose_reply"),
        "Ask Coda to reply must refuse no_reply_would_help: {asked_wire}"
    );
}

#[tokio::test]
async fn ac7_end_to_end_synthetic_spruce_webhook_match_link_review_surfaces_evidence_reopen_and_later_update()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let now_ts = current_unix_ts();
    let practice = format!("prac-ac7-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, dr_token, dr_hash) =
        invite_and_sign_in(&auth, &practice, "dr-ac7", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-ac7-{}", Uuid::new_v4().simple()),
        display_name: "Clara Oswald".into(),
        birth_date: Some("1986-11-23".into()),
        sex_or_gender: Some("female".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-7701".into(),
        }],
    };
    let workspace = workspace_id("clinic-ehr", &patient.source_patient_id);
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(&dr, &workspace, Membership::Allowed)
        .await
        .unwrap();

    let bmp_v1 = chart_record(
        &workspace,
        "clinic-ehr",
        "lab-bmp-ac7",
        "rev-1",
        "Observation",
        "2026-11-11",
        "Basic Metabolic Panel",
        "Potassium 5.5 mmol/L, Creatinine 1.04 mg/dL on lisinopril 20 mg daily.",
    );
    store
        .record_chart(
            &workspace,
            &[bmp_v1],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient: patient.clone(),
                pulled_at: now_ts,
            },
        )
        .await
        .unwrap();

    // 1. Connect Spruce channel, confirm sender address, ingest webhook item, and run matching -> auto-link!
    store
        .connect_spruce_channel(auth.as_ref(), &dr, 0, None, now_ts)
        .await
        .unwrap();
    let lease = store
        .acquire_spruce_reader_lease(&practice, "worker-ac7", 120, now_ts)
        .await
        .unwrap();
    let clara_phone = SenderAddress::parse("(415) 555-0199").unwrap();
    store
        .record_spruce_confirmed_sender(
            &practice,
            &clara_phone,
            &workspace,
            &dr.physician_id,
            now_ts,
        )
        .await
        .unwrap();

    let conv_id = "spruce-conv-ac7";
    let msg_1_text =
        "I saw my potassium was 5.5 on yesterday's portal result—do I need a repeat lab?";
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &IncomingSpruceItem::Readable(ReadableSpruceItem {
                message_id: "spruce-msg-1".into(),
                conversation_id: conv_id.into(),
                channel_position: 1,
                content_version: "v1".into(),
                sender_address: Some(clara_phone.clone()),
                author_name: Some("Clara Oswald".into()),
                body_text: msg_1_text.into(),
                attachments: Vec::new(),
                app_url: None,
                created_at_spruce: None,
                outside_participants: vec![OutsideParticipant {
                    display_name: Some("Clara Oswald".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(clara_phone),
                    saved_contact_id: None,
                }],
            }),
            now_ts + 5,
        )
        .await
        .unwrap();

    let skills = load_skills();
    let roles = distinct_roles();
    let receiver = RecordingReviewReceiver::default();

    let (mut p_match, _) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "submit-match",
            SUBMIT_CLASSIFICATION_TOOL,
            &json!({
                "classification": "patient_mail",
                "urgency": "ordinary",
                "match_outcome": "suggested_patient",
                "suggested_workspace": workspace,
                "evidence_summary": "Confirmed SMS sender +14155550199 uniquely matches Clara Oswald.",
                "rationale": "Patient portal lab question from confirmed sender.",
            }),
        )],
        vec![sse_assistant_message("Matched and auto-linked message.")],
    ])
    .await;

    run_spruce_message_matching(
        &store,
        auth.as_ref(),
        &practice,
        "spruce-msg-1",
        &roles,
        &skills,
        &mut p_match,
        Some(&receiver),
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(receiver.snapshot().len(), 1);
    assert_eq!(receiver.snapshot()[0].patient_workspace_id, workspace);

    // 2. Mirror the auto-linked conversation into the reply owner and run `run_message_review`:
    store
        .link_reply_conversation(
            &practice,
            conv_id,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            message_id: "spruce-msg-1".into(),
            sender: MessageSender::Patient,
            text: msg_1_text.into(),
            sent_at: now_ts + 5,
        })
        .await
        .unwrap();

    let (mut p_rev1, _) = spawn_responses_sse_server(vec![
        vec![
            sse_function_call(
                "read-bmp-ac7",
                READ_TOOL,
                &json!({
                    "references": [{ "candidate_id": "lab-bmp-ac7", "revision": "rev-1", "cursor": null }],
                    "max_result_bytes": 16_384,
                }),
            ),
            sse_function_call(
                "concern-k-ac7",
                CONCERN_TOOL,
                &json!({
                    "operation": "raise",
                    "concern_id": "concern-mild-hyperkalemia",
                    "urgency": "today",
                    "reason": "Potassium 5.5 mmol/L on 2026-11-11 while taking lisinopril 20 mg daily; patient asking whether to repeat BMP.",
                    "records": [{ "candidate_id": "lab-bmp-ac7", "revision": "rev-1" }],
                }),
            ),
        ],
        vec![sse_function_call(
            "done-rev1",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [],
                "decision": {
                    "kind": "propose_reply",
                    "reply_text": "Thank you for checking in. Your potassium was mildly elevated at 5.5 mmol/L on November 11; your doctor is reviewing whether to order a repeat lab draw and we will update you shortly.",
                    "basis_summary": "2026-11-11 BMP shows potassium 5.5 mmol/L on lisinopril 20 mg; physician decision needed on repeat BMP.",
                    "limitations": [],
                    "cited_records": [{ "candidate_id": "lab-bmp-ac7", "revision": "rev-1" }],
                    "addresses_medication_question": false,
                },
                "raised_concern_ids": ["concern-mild-hyperkalemia"],
            }),
        )],
        vec![sse_assistant_message("Saved v1 proposal and Today concern.")],
    ])
    .await;

    let rev1 = run_message_review(
        &store,
        auth.as_ref(),
        &practice,
        conv_id,
        &MessageReviewOptions::default(),
        &roles,
        &skills,
        &mut p_rev1,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    assert!(matches!(rev1.outcome, ReplyReviewOutcome::Applied(_)));

    // 3. Inspect physician reply surface, Today projection, Updates projection, and reopen cited evidence:
    let access = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &workspace,
        session_hash: &dr_hash,
    };
    let surface_1 = store
        .physician_reply_surface(&access, conv_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert!(matches!(surface_1, PhysicianReplySurface::OpenCard(_)));

    let today = store
        .read_today(auth.as_ref(), &dr_token, now_ts as u64, &BTreeMap::new())
        .await
        .unwrap();
    let concern_row = today
        .rows
        .iter()
        .find(|r| r.item_key.as_str().contains("concern-mild-hyperkalemia"))
        .expect("Today concern row visible to panel physician");
    let opened_evidence = store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut PostgresWorkspaceAuthority,
            &OpenItemRequest {
                token: &dr_token,
                real_now_unix: now_ts as u64,
                workspace: &workspace,
                item_key: &concern_row.item_key,
                command_key: &CommandKey::new("open-ac7-concern").unwrap(),
                patient_names: &BTreeMap::new(),
            },
        )
        .await
        .unwrap();
    assert_eq!(opened_evidence.citations.len(), 1);
    assert_eq!(
        opened_evidence.citations[0].reference,
        ExactRecordReference::new("lab-bmp-ac7", "rev-1").unwrap()
    );
    assert!(opened_evidence.citations[0].body.contains("5.5 mmol/L"));

    let updates = store
        .read_updates(auth.as_ref(), &dr_token, now_ts as u64, &BTreeMap::new())
        .await
        .unwrap();
    assert!(
        updates
            .today_items
            .iter()
            .any(|u| u.item_key.as_str().contains("concern-mild-hyperkalemia"))
    );

    // 4. Later message arrives BEFORE physician edits draft -> replaces v1 (`NewerReplyReplacedPrevious`):
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            message_id: "spruce-msg-2".into(),
            sender: MessageSender::Patient,
            text: "Also I have no muscle weakness or palpitations.".into(),
            sent_at: now_ts + 30,
        })
        .await
        .unwrap();

    let (mut p_rev2, _) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "read-bmp-2",
            READ_TOOL,
            &json!({
                "references": [{ "candidate_id": "lab-bmp-ac7", "revision": "rev-1", "cursor": null }],
                "max_result_bytes": 16_384,
            }),
        )],
        vec![sse_function_call(
            "done-rev2",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [],
                "decision": {
                    "kind": "propose_reply",
                    "reply_text": "Glad to hear you feel well without weakness or palpitations. Your doctor is reviewing your 5.5 mmol/L potassium result and will let you know next steps.",
                    "basis_summary": "Incorporated reassuring symptom update alongside 2026-11-11 BMP.",
                    "limitations": [],
                    "cited_records": [{ "candidate_id": "lab-bmp-ac7", "revision": "rev-1" }],
                    "addresses_medication_question": false,
                },
                "raised_concern_ids": [],
            }),
        )],
        vec![sse_assistant_message("Published v2 proposal.")],
    ])
    .await;
    run_message_review(
        &store,
        auth.as_ref(),
        &practice,
        conv_id,
        &MessageReviewOptions::default(),
        &roles,
        &skills,
        &mut p_rev2,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let card_v2 = store
        .reply_card(&access, conv_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        &card_v2.displayed_text,
        DisplayedReplyText::CodaProposal { version: 2, text, .. }
            if text.contains("without weakness or palpitations")
    ));

    // 5. Physician edits v2 (`YourText`), then corrected lab (`rev-2`, 4.4 mmol/L) arrives ->
    // `run_message_review` publishes v3 while keeping physician's `YourText` and offering `coda_update_offer`!
    store
        .edit_reply_proposal_draft(
            &access,
            conv_id,
            ExpectedDraftBase::ProposalVersion(2),
            "Physician draft: please come to the lab Friday morning for a non-hemolyzed repeat potassium.",
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    let bmp_v2 = chart_record(
        &workspace,
        "clinic-ehr",
        "lab-bmp-ac7",
        "rev-2",
        "Observation",
        "2026-11-12",
        "Corrected Basic Metabolic Panel",
        "Addendum: repeat spun potassium 4.4 mmol/L (initial 5.5 was pseudohyperkalemia).",
    );
    store.record_source(&bmp_v2).await.unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            message_id: "spruce-msg-3".into(),
            sender: MessageSender::Patient,
            text: "Did the lab post any update on whether that first sample was hemolyzed?".into(),
            sent_at: now_ts + 45,
        })
        .await
        .unwrap();

    let (mut p_rev3, _) = spawn_responses_sse_server(vec![
        vec![sse_function_call(
            "read-bmp-3",
            READ_TOOL,
            &json!({
                "references": [{ "candidate_id": "lab-bmp-ac7", "revision": "rev-2", "cursor": null }],
                "max_result_bytes": 16_384,
            }),
        )],
        vec![sse_function_call(
            "done-rev3",
            COMPLETE_MESSAGE_REVIEW_TOOL,
            &json!({
                "answered_closures": [],
                "decision": {
                    "kind": "propose_reply",
                    "reply_text": "Good news—the lab posted a corrected potassium result of 4.4 mmol/L (normal), so no repeat blood draw is needed.",
                    "basis_summary": "Lab addendum rev-2 corrected potassium to 4.4 mmol/L.",
                    "limitations": [],
                    "cited_records": [{ "candidate_id": "lab-bmp-ac7", "revision": "rev-2" }],
                    "addresses_medication_question": false,
                },
                "raised_concern_ids": [],
            }),
        )],
        vec![sse_assistant_message(
            "Published v3 proposal after lab correction.",
        )],
    ])
    .await;
    run_message_review(
        &store,
        auth.as_ref(),
        &practice,
        conv_id,
        &MessageReviewOptions {
            review_id: None,
            coalesced_changed_candidates: vec!["lab-bmp-ac7".into()],
            extra_attachments: Vec::new(),
        },
        &roles,
        &skills,
        &mut p_rev3,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let card_v3 = store
        .reply_card(&access, conv_id, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        &card_v3.displayed_text,
        DisplayedReplyText::PhysicianDraft {
            kind: PhysicianDraftKind::YourText,
            text,
            ..
        } if text.contains("Physician draft: please come to the lab Friday morning")
    ));
    let offer = card_v3
        .coda_update_offer
        .expect("physician holding a draft must receive coda_update_offer when v3 arrives");
    assert_eq!(offer.offered_version, 3);
    assert!(
        offer
            .offered_text
            .contains("corrected potassium result of 4.4 mmol/L")
    );
}

#[tokio::test]
#[ignore = "requires CODA_ALLOW_MODEL_CALLS=1, live OPENAI_API_KEY, and PHYSICIAN_RUNTIME_TEST_DATABASE_URL"]
async fn live_openai_message_review_smoke() {
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
    let practice = format!("prac-live-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_ts as u64))
        .await
        .unwrap();

    let (dr, _tok, _hash) = invite_and_sign_in(&auth, &practice, "dr-live", now_ts as u64).await;
    let patient = SourcePatient {
        source_patient_id: format!("pat-live-{}", Uuid::new_v4().simple()),
        display_name: "Synthetic Live Patient".into(),
        birth_date: Some("1970-01-01".into()),
        sex_or_gender: Some("female".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-ehr".into(),
            value: "MRN-LIVE-1".into(),
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
        "obs-live-1",
        "rev-1",
        "Observation",
        "2026-11-11",
        "Hemoglobin A1c",
        "Hemoglobin A1c 6.4% (improved from 7.1% six months ago).",
    );
    store
        .record_chart(
            &workspace,
            &[rec],
            &SourcePull {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                patient,
                pulled_at: now_ts,
            },
        )
        .await
        .unwrap();

    let conv_id = "conv-live-smoke";
    store
        .link_reply_conversation(
            &practice,
            conv_id,
            ConversationLink::LinkedPatient {
                workspace: workspace.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            message_id: "pm-live-1".into(),
            sender: MessageSender::Patient,
            text: "Did my A1c lab result come back yet?".into(),
            sent_at: now_ts + 5,
        })
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

    let done = run_message_review(
        &store,
        auth.as_ref(),
        &practice,
        conv_id,
        &MessageReviewOptions::default(),
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
