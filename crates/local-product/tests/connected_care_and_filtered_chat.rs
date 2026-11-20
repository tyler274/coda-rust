//! End-to-end PostgreSQL proof for:
//! 1. Plain-words filtered-mail physician chat tools (`list_filtered_messages`,
//!    `restore_filtered_message`) executing through `PhysicianHost` with
//!    practice isolation, session context checks, and optional review start on
//!    already-linked conversations.
//! 2. Connected-care `"For my doctor"` intake and consent join/rejoin wakes:
//!    explicit consent gate, immediate join/rejoin chart sync + priority wake,
//!    withdrawal preserving already-shared dated records, single-ingestion
//!    `"For my doctor"` provenance + `"received"` writeback, priority dispatch
//!    ordering ahead of normal periodic work, and escalation to `Today`.

use std::collections::{BTreeMap, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::{
    Json, Router,
    extract::State,
    http::{StatusCode, header},
    response::IntoResponse,
    routing::post,
};
use front_door::{
    ConnectionEndReason, ForMyDoctorError, ForMyDoctorSendRequest, ForMyDoctorShareStatus,
    FrontDoorStore, PostgresFrontDoorStore, RosterPatient, SharedPatientRecordInput,
};
use harness::provider::ResponsesProvider;
use local_product::patient_reviews::COMPLETE_PATIENT_REVIEW_TOOL;
use local_product::physician_chat::{
    ChatAssignment, LIST_FILTERED_MESSAGES_TOOL, PhysicianSkills, RESTORE_FILTERED_MESSAGE_TOOL,
};
use local_product::physician_sources::{READ_TOOL, SourcePatient, workspace_id};
use local_product::runtime::concerns::CONCERN_TOOL;
use local_product::runtime::connected_care::{
    ConnectedCareError, DUET_FOR_MY_DOCTOR_SOURCE_ID, DUET_PATIENT_SHARE_SOURCE_ID,
};
use local_product::runtime::dispatch::ReviewReceiver;
use local_product::runtime::engine::{
    self, Invocation, Item, Turn,
    recipe::{Observer, Provider, Request},
};
use local_product::runtime::jobs::{JobPriority, PracticeJobHost, PracticeScope};
use local_product::runtime::reviews::{OwnedPatientReviewReceiver, PatientReviewReceiver};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::spruce_matching::{
    IncomingSpruceItem, MatchAssessment, MatchingDecision, MessageStanding, NonPatientCategory,
    OutsideParticipant, ReadableSpruceItem, SenderAddress, SenderConfirmationPolicy,
};
use local_product::runtime::{
    Accepted, Authority, Input, PhysicianHost, PublicationTransaction, Scope, ShownSteps,
    SpruceChatContext, Store, async_trait,
};
use local_product::{ModelBinding, ModelRoles};
use patient_work::{ReviewDispatchInput, ReviewReason, ReviewRequest};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use serde_json::{Value, json};
use uuid::Uuid;

fn test_database_url() -> Option<String> {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
        .or_else(|_| std::env::var("PHYSICIAN_DATABASE_URL"))
        .unwrap_or_default();
    let trimmed = url.trim().to_owned();
    if trimmed.is_empty() {
        assert!(
            std::env::var("CODA_REQUIRE_POSTGRES_TESTS").as_deref() != Ok("1"),
            "CODA_REQUIRE_POSTGRES_TESTS=1 requires PHYSICIAN_RUNTIME_TEST_DATABASE_URL"
        );
        None
    } else {
        Some(trimmed)
    }
}

fn load_skills() -> PhysicianSkills {
    PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
        .expect("load workspace skills")
}

fn model_roles() -> ModelRoles {
    ModelRoles {
        chat: ModelBinding {
            model: "interactive-chat-lane".into(),
            effort: "medium".into(),
            context_window_tokens: 192_000,
            max_input_tokens: 160_000,
            adapter: "responses".into(),
        },
        background: ModelBinding {
            model: "background-rounds-lane".into(),
            effort: "high".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 250_000,
            adapter: "responses".into(),
        },
    }
}

async fn setup_stores(url: &str) -> (Store, Arc<PostgresAuthStore>, PostgresFrontDoorStore) {
    let store = Store::connect(url).await.expect("connect runtime store");
    store.migrate().await.expect("migrate runtime store");
    let auth = Arc::new(
        PostgresAuthStore::connect_with_migrations(url)
            .await
            .expect("connect auth store"),
    );
    let front_door = PostgresFrontDoorStore::connect_with_migrations(url)
        .await
        .expect("connect front_door store");
    (store, auth, front_door)
}

async fn invite_and_sign_in(
    auth: &PostgresAuthStore,
    practice_id: &str,
    label: &str,
    now: u64,
) -> (PhysicianIdentity, String, Vec<u8>) {
    let email = format!("{label}-{}@codahealth.test", Uuid::new_v4().simple());
    let token_invite = auth
        .invite(practice_id, &email, label, true, now)
        .await
        .expect("invite physician");
    let (identity, session) = auth
        .redeem_invitation(&email, &token_invite, now)
        .await
        .expect("redeem physician");
    let session_hash = PostgresAuthStore::session_reference(&session);
    (identity, session, session_hash)
}

struct ChatTestAuthority {
    spruce: Option<SpruceChatContext>,
}

#[async_trait]
impl Authority for ChatTestAuthority {
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

    fn spruce_chat_context(&self) -> Option<SpruceChatContext> {
        self.spruce.clone()
    }
}

fn readable_spruce(
    pos: u64,
    message_id: &str,
    conversation_id: &str,
    sender: &str,
    author: &str,
    body: &str,
) -> IncomingSpruceItem {
    IncomingSpruceItem::Readable(ReadableSpruceItem {
        message_id: message_id.to_owned(),
        conversation_id: conversation_id.to_owned(),
        channel_position: pos,
        content_version: format!("ver-{message_id}-{pos}"),
        sender_address: Some(SenderAddress::parse(sender).expect("valid sender")),
        author_name: Some(author.to_owned()),
        body_text: body.to_owned(),
        attachments: Vec::new(),
        app_url: Some(format!(
            "https://app.sprucehealth.com/conversations/{conversation_id}"
        )),
        created_at_spruce: Some("2026-11-19T12:00:00Z".to_owned()),
        outside_participants: vec![OutsideParticipant {
            display_name: Some(author.to_owned()),
            endpoint_channel: Some("sms".into()),
            endpoint_address: Some(SenderAddress::parse(sender).expect("valid participant")),
            saved_contact_id: None,
        }],
    })
}

fn materialized_outputs(request: &Request) -> BTreeMap<String, String> {
    request
        .items
        .iter()
        .filter_map(|item| match item {
            Item::MaterializedResult { call_id, output } => Some((call_id.clone(), output.clone())),
            _ => None,
        })
        .collect()
}

#[derive(Clone)]
enum ScriptedSseStep {
    FunctionCall {
        call_id: String,
        name: String,
        arguments: Value,
    },
    AssistantMessage {
        text: String,
    },
}

#[derive(Clone, Default)]
struct LocalSseState {
    script: Arc<Mutex<VecDeque<ScriptedSseStep>>>,
    captured_requests: Arc<Mutex<Vec<Value>>>,
}

impl LocalSseState {
    fn push_call(&self, call_id: &str, name: &str, arguments: Value) {
        self.script
            .lock()
            .unwrap()
            .push_back(ScriptedSseStep::FunctionCall {
                call_id: call_id.to_owned(),
                name: name.to_owned(),
                arguments,
            });
    }

    fn push_assistant(&self, text: &str) {
        self.script
            .lock()
            .unwrap()
            .push_back(ScriptedSseStep::AssistantMessage {
                text: text.to_owned(),
            });
    }

    fn requests(&self) -> Vec<Value> {
        self.captured_requests.lock().unwrap().clone()
    }
}

async fn spawn_local_sse_provider(
    store: &Store,
) -> (
    LocalSseState,
    ResponsesProvider,
    tokio::task::JoinHandle<()>,
) {
    let sse_state = LocalSseState::default();
    let app = Router::new()
        .route(
            "/v1/responses",
            post(
                |State(state): State<LocalSseState>, Json(body): Json<Value>| async move {
                    let req_num = {
                        let mut guard = state.captured_requests.lock().unwrap();
                        guard.push(body);
                        guard.len()
                    };
                    let step = state
                        .script
                        .lock()
                        .unwrap()
                        .pop_front()
                        .unwrap_or_else(|| panic!("SSE script exhausted on request #{req_num}"));

                    let done_item = match step {
                        ScriptedSseStep::FunctionCall {
                            call_id,
                            name,
                            arguments,
                        } => json!({
                            "type": "function_call",
                            "status": "completed",
                            "call_id": call_id,
                            "name": name,
                            "arguments": arguments.to_string(),
                        }),
                        ScriptedSseStep::AssistantMessage { text } => json!({
                            "type": "message",
                            "role": "assistant",
                            "status": "completed",
                            "content": [{
                                "type": "output_text",
                                "text": text,
                            }],
                        }),
                    };
                    let response_obj = json!({
                        "id": format!("resp-sse-{req_num}"),
                        "model": "background-rounds-lane",
                        "status": "completed",
                        "output": [done_item],
                        "usage": {
                            "input_tokens": 500,
                            "input_tokens_details": { "cached_tokens": 80 },
                            "output_tokens": 60
                        }
                    });
                    let sse_payload = format!(
                        "event: response.output_item.done\ndata: {}\n\nevent: response.completed\ndata: {}\n\n",
                        json!({
                            "type": "response.output_item.done",
                            "output_index": 0,
                            "item": done_item,
                        }),
                        json!({
                            "type": "response.completed",
                            "response": response_obj,
                        })
                    );
                    (
                        StatusCode::OK,
                        [(header::CONTENT_TYPE, "text/event-stream")],
                        sse_payload,
                    )
                        .into_response()
                },
            ),
        )
        .with_state(sse_state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind SSE listener");
    let addr = listener.local_addr().expect("SSE local addr");
    let handle = tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    let provider = ResponsesProvider::new(
        format!("http://{addr}/v1/responses"),
        "local-sse-key".into(),
        Arc::new(store.clone()),
    )
    .expect("ResponsesProvider");
    (sse_state, provider, handle)
}

#[tokio::test]
async fn filtered_mail_chat_tools_list_isolate_and_restore_patient_messages() {
    let Some(url) = test_database_url() else {
        return;
    };
    let (store, auth, _front_door) = setup_stores(&url).await;
    let now_u64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let now_ts = now_u64 as i64;
    let practice_a = format!("practice-fmail-a-{}", Uuid::new_v4().simple());
    let practice_b = format!("practice-fmail-b-{}", Uuid::new_v4().simple());

    let (dr_a, _tok_a, hash_a) = invite_and_sign_in(&auth, &practice_a, "dr-alpha", now_u64).await;
    let (dr_b, _tok_b, _hash_b) = invite_and_sign_in(&auth, &practice_b, "dr-beta", now_u64).await;

    let maria_source_id = format!("pat-maria-filter-{}", Uuid::new_v4().simple());
    let workspace_maria = workspace_id("synthetic-clinic-ehr", &maria_source_id);
    auth.register_patient(&practice_a, &workspace_maria)
        .await
        .unwrap();
    auth.set_membership(&dr_a, &workspace_maria, Membership::Allowed)
        .await
        .unwrap();
    store
        .record_chart(
            &workspace_maria,
            &[],
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient: SourcePatient {
                    source_patient_id: maria_source_id,
                    display_name: "Maria Alvarez".into(),
                    birth_date: Some("1975-06-12".into()),
                    sex_or_gender: Some("female".into()),
                    medical_record_numbers: Vec::new(),
                },
                pulled_at: Some(now_ts),
            },
        )
        .await
        .unwrap();

    // Connect Spruce for both practices and seed filtered rows + one linked conversation.
    store
        .connect_spruce_channel(auth.as_ref(), &dr_a, 0, None, now_ts)
        .await
        .unwrap();
    let lease_a = store
        .acquire_spruce_reader_lease(&practice_a, "worker-a", 120, now_ts)
        .await
        .unwrap();

    store
        .connect_spruce_channel(auth.as_ref(), &dr_b, 0, None, now_ts)
        .await
        .unwrap();
    let lease_b = store
        .acquire_spruce_reader_lease(&practice_b, "worker-b", 120, now_ts)
        .await
        .unwrap();

    let conv_linked = format!("conv-linked-{}", Uuid::new_v4().simple());
    let msg_prior = format!("msg-prior-{}", Uuid::new_v4().simple());
    let msg_linked_filtered = format!("msg-maria-linked-{}", Uuid::new_v4().simple());
    let msg_unlinked_filtered = format!("msg-maria-unlinked-{}", Uuid::new_v4().simple());
    let msg_noise = format!("msg-vendor-noise-{}", Uuid::new_v4().simple());
    let msg_foreign = format!("msg-foreign-maria-{}", Uuid::new_v4().simple());

    // Confirm an initial message in `conv_linked` -> `workspace_maria` so restoring another
    // filtered message in that same conversation links directly and starts a PatientReview.
    store
        .ingest_spruce_item(
            &practice_a,
            &lease_a.lease_token,
            &readable_spruce(
                1,
                &msg_prior,
                &conv_linked,
                "+14155550111",
                "Maria Alvarez",
                "Earlier check-in message",
            ),
            now_ts + 1,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice_a,
            &msg_prior,
            None,
            "v1",
            "g1",
            MatchingDecision::PatientMail {
                urgency: local_product::runtime::spruce_matching::MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::SuggestedPatient {
                    workspace: workspace_maria.clone(),
                    evidence_summary: "Single matching chart.".into(),
                },
                rationale: "Patient mail.".into(),
            },
            None,
            now_ts + 2,
        )
        .await
        .unwrap();
    store
        .confirm_spruce_message_link(
            auth.as_ref(),
            &dr_a,
            &msg_prior,
            &workspace_maria,
            SenderConfirmationPolicy::ConversationOnly,
            "confirm-prior-link",
            None,
            now_ts + 3,
        )
        .await
        .unwrap();

    // Seed filtered messages in Practice A + one unrelated Practice B filtered message.
    for (pos, mid, conv, phone, author, body, cat, reason) in [
        (
            2_u64,
            msg_linked_filtered.as_str(),
            conv_linked.as_str(),
            "+14155550111",
            "Maria Alvarez",
            "Checking in on my blood pressure medication refill",
            NonPatientCategory::AdministrativeNotice,
            "Automated clinic follow-up template false positive.",
        ),
        (
            3_u64,
            msg_unlinked_filtered.as_str(),
            "conv-maria-unlinked",
            "+14155550199",
            "Maria Alvarez",
            "Hi team, Maria calling back from my new mobile number",
            NonPatientCategory::VendorOrSpam,
            "Unknown caller number misclassified.",
        ),
        (
            4_u64,
            msg_noise.as_str(),
            "conv-vendor-noise",
            "+18005559999",
            "LabSupply Promo",
            "Discount autoclave wipes this week",
            NonPatientCategory::NonPatientOutreach,
            "Commercial supply marketing.",
        ),
    ] {
        store
            .ingest_spruce_item(
                &practice_a,
                &lease_a.lease_token,
                &readable_spruce(pos, mid, conv, phone, author, body),
                now_ts + 10 + pos as i64,
            )
            .await
            .unwrap();
        store
            .record_spruce_message_classification(
                auth.as_ref(),
                &practice_a,
                mid,
                None,
                "v1",
                "g1",
                MatchingDecision::NotPatientMail {
                    category: cat,
                    rationale: reason.to_owned(),
                },
                None,
                now_ts + 20 + pos as i64,
            )
            .await
            .unwrap();
    }

    store
        .ingest_spruce_item(
            &practice_b,
            &lease_b.lease_token,
            &readable_spruce(
                1,
                &msg_foreign,
                "conv-foreign-b",
                "+14155550888",
                "Maria OtherClinic",
                "Other practice filtered message",
            ),
            now_ts + 30,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice_b,
            &msg_foreign,
            None,
            "v1",
            "g1",
            MatchingDecision::NotPatientMail {
                category: NonPatientCategory::VendorOrSpam,
                rationale: "Other practice noise.".into(),
            },
            None,
            now_ts + 31,
        )
        .await
        .unwrap();

    // Chat turn without Spruce context refuses filtered-mail tools cleanly.
    let roles = model_roles();
    let skills = load_skills();
    let scope_no_spruce = Scope {
        practice: practice_a.clone(),
        physician: dr_a.physician_id.clone(),
        workspace: workspace_maria.clone(),
        chat: format!("chat-no-spruce-{}", Uuid::new_v4().simple()),
    };
    let start_ctx = store
        .patient_starting_context(&scope_no_spruce)
        .await
        .unwrap();
    let recipe = ChatAssignment::for_patient(scope_no_spruce.clone(), start_ctx.clone())
        .recipe(&roles, &skills);
    assert_eq!(
        store
            .start_chat_authorized(
                &scope_no_spruce,
                &Input {
                    key: "cmd-no-spruce".into(),
                    text: "Check filtered messages".into(),
                    timezone: "UTC".into(),
                    scenario: None,
                },
                hash_a.clone(),
                &skills,
                &recipe,
                &mut ChatTestAuthority { spruce: None },
            )
            .await
            .unwrap(),
        Accepted::New
    );

    struct NoSpruceCheckProvider {
        step: usize,
        saw_refusal: bool,
    }
    impl Provider for NoSpruceCheckProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.step += 1;
            match self.step {
                1 => Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "call-list-no-ctx".into(),
                        name: LIST_FILTERED_MESSAGES_TOOL.into(),
                        arguments: json!({ "query": null }).to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let out = materialized_outputs(request);
                    let list_out = out.get("call-list-no-ctx").expect("list output");
                    assert!(
                        list_out.contains("active physician session context"),
                        "expected refusal without spruce context, got {list_out}"
                    );
                    self.saw_refusal = true;
                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "finish-no-ctx".into(),
                            name: local_product::physician_sources::citations::FINISH_TOOL.into(),
                            arguments: json!({
                                "answer": "Filtered message context was unavailable.",
                                "quotes": [],
                                "descriptions": [],
                                "basis": "Tool refusal.",
                                "cites_none": true
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                _ => panic!("unexpected turn"),
            }
        }
    }

    let claim_no_ctx = store.claim(&scope_no_spruce, 60).await.unwrap().unwrap();
    let mut host_no_ctx = PhysicianHost {
        store: store.clone(),
        claim: claim_no_ctx,
        authority: ChatTestAuthority { spruce: None },
        shown_steps: ShownSteps::new(()),
    };
    let mut no_ctx_provider = NoSpruceCheckProvider {
        step: 0,
        saw_refusal: false,
    };
    host_no_ctx
        .run(
            &mut no_ctx_provider,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();
    assert!(no_ctx_provider.saw_refusal);

    // Chat turn WITH SpruceChatContext + OwnedPatientReviewReceiver executes plain-words list + restore.
    let receiver = Arc::new(OwnedPatientReviewReceiver::new(
        store.clone(),
        roles.clone(),
        skills.clone(),
        auth.clone(),
    ));
    let spruce_ctx = SpruceChatContext {
        auth: auth.clone(),
        review_receiver: Some(receiver),
    };
    let scope = Scope {
        practice: practice_a.clone(),
        physician: dr_a.physician_id.clone(),
        workspace: workspace_maria.clone(),
        chat: format!("chat-spruce-restore-{}", Uuid::new_v4().simple()),
    };
    let recipe_main = ChatAssignment::for_patient(scope.clone(), start_ctx).recipe(&roles, &skills);
    assert_eq!(
        store
            .start_chat_authorized(
                &scope,
                &Input {
                    key: "cmd-filter-restore".into(),
                    text: "Find filtered messages for Maria and restore them as patient mail."
                        .into(),
                    timezone: "UTC".into(),
                    scenario: None,
                },
                hash_a,
                &skills,
                &recipe_main,
                &mut ChatTestAuthority {
                    spruce: Some(spruce_ctx.clone()),
                },
            )
            .await
            .unwrap(),
        Accepted::New
    );

    struct RestoreFlowProvider {
        step: usize,
        msg_unlinked: String,
        msg_linked: String,
        msg_noise: String,
        msg_foreign: String,
    }
    impl Provider for RestoreFlowProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.step += 1;
            match self.step {
                1 => Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "list-maria".into(),
                        name: LIST_FILTERED_MESSAGES_TOOL.into(),
                        arguments: json!({ "query": "maria" }).to_string(),
                    })],
                    receipt: json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let outputs = materialized_outputs(request);
                    let listed: Value =
                        serde_json::from_str(outputs.get("list-maria").unwrap()).unwrap();
                    assert_eq!(
                        listed["count"], 2,
                        "only Maria messages in practice_a: {listed}"
                    );
                    let ids: Vec<&str> = listed["filtered_messages"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .filter_map(|row| row["message_id"].as_str())
                        .collect();
                    assert!(ids.contains(&self.msg_unlinked.as_str()), "{ids:?}");
                    assert!(ids.contains(&self.msg_linked.as_str()), "{ids:?}");
                    assert!(!ids.contains(&self.msg_noise.as_str()), "{ids:?}");
                    assert!(!ids.contains(&self.msg_foreign.as_str()), "{ids:?}");

                    Ok(Turn {
                        items: vec![
                            Item::Call(Invocation {
                                id: "restore-unlinked".into(),
                                name: RESTORE_FILTERED_MESSAGE_TOOL.into(),
                                arguments: json!({ "message_id": self.msg_unlinked }).to_string(),
                            }),
                            Item::Call(Invocation {
                                id: "restore-linked".into(),
                                name: RESTORE_FILTERED_MESSAGE_TOOL.into(),
                                arguments: json!({ "message_id": self.msg_linked }).to_string(),
                            }),
                        ],
                        receipt: json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let outputs = materialized_outputs(request);
                    let unlinked: Value =
                        serde_json::from_str(outputs.get("restore-unlinked").unwrap()).unwrap();
                    assert_eq!(unlinked["restored_standing"], "waiting_for_match");
                    assert_eq!(unlinked["review_started"], false);

                    let linked: Value =
                        serde_json::from_str(outputs.get("restore-linked").unwrap()).unwrap();
                    assert_eq!(linked["restored_standing"], "linked_patient");
                    assert_eq!(linked["review_started"], true);

                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "finish-restore".into(),
                            name: local_product::physician_sources::citations::FINISH_TOOL.into(),
                            arguments: json!({
                                "answer": "Restored both filtered messages for Maria Alvarez: the message in her existing thread rejoined her conversation and started a review, and the message from her new number is waiting to match.",
                                "quotes": [],
                                "descriptions": [],
                                "basis": "Practice filtered message queue lookup and restore.",
                                "cites_none": true
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store: store.clone(),
        claim,
        authority: ChatTestAuthority {
            spruce: Some(spruce_ctx),
        },
        shown_steps: ShownSteps::new(()),
    };
    let mut provider = RestoreFlowProvider {
        step: 0,
        msg_unlinked: msg_unlinked_filtered.clone(),
        msg_linked: msg_linked_filtered.clone(),
        msg_noise: msg_noise.clone(),
        msg_foreign: msg_foreign.clone(),
    };
    host.run(
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Verify durable DB state after chat restore tools:
    let remaining = store
        .filtered_spruce_messages_for_physician(auth.as_ref(), &dr_a)
        .await
        .unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].message_id, msg_noise);

    let unlinked_row = store
        .spruce_message(&practice_a, &msg_unlinked_filtered)
        .await
        .unwrap()
        .expect("unlinked row");
    assert!(matches!(
        unlinked_row.standing,
        MessageStanding::Waiting { .. }
    ));

    let linked_row = store
        .spruce_message(&practice_a, &msg_linked_filtered)
        .await
        .unwrap()
        .expect("linked row");
    assert!(matches!(
        linked_row.standing,
        MessageStanding::Linked { .. }
    ));

    let reviews = store
        .patient_reviews(&practice_a, &workspace_maria, auth.as_ref())
        .await
        .unwrap();
    assert_eq!(
        reviews.len(),
        1,
        "restoring onto linked conversation queues one review"
    );

    // Practice B's filtered message stays untouched and strictly isolated.
    let foreign_remaining = store
        .filtered_spruce_messages_for_physician(auth.as_ref(), &dr_b)
        .await
        .unwrap();
    assert_eq!(foreign_remaining.len(), 1);
    assert_eq!(foreign_remaining[0].message_id, msg_foreign);
}

#[tokio::test]
async fn connected_care_consent_join_rejoin_and_for_my_doctor_flow() {
    let Some(url) = test_database_url() else {
        return;
    };
    let (store, auth, front_door) = setup_stores(&url).await;
    let now_u64 = 1_763_200_000;
    let now_ts = now_u64 as i64;
    let practice = format!("practice-ccare-{}", Uuid::new_v4().simple());
    let (_dr, dr_token, _) = invite_and_sign_in(&auth, &practice, "dr-care", now_u64).await;
    let dr_id = &_dr;

    let workspace = format!("ws-ccare-{}", Uuid::new_v4().simple());
    let workspace_other = format!("ws-ccare-periodic-{}", Uuid::new_v4().simple());
    auth.register_patient(&practice, &workspace).await.unwrap();
    auth.set_membership(dr_id, &workspace, Membership::Allowed)
        .await
        .unwrap();
    auth.register_patient(&practice, &workspace_other)
        .await
        .unwrap();
    auth.set_membership(dr_id, &workspace_other, Membership::Allowed)
        .await
        .unwrap();

    // Seed baseline EHR info for both workspaces so `Today` renders clean names.
    for (ws, name) in [(&workspace, "Sam Rivera"), (&workspace_other, "Liam Cho")] {
        store
            .record_chart(
                ws,
                &[],
                &SourcePull {
                    source_id: "synthetic-clinic-ehr".into(),
                    source_label: "Synthetic Clinic EHR".into(),
                    patient: SourcePatient {
                        source_patient_id: ws.clone(),
                        display_name: name.to_owned(),
                        birth_date: Some("1982-04-19".into()),
                        sex_or_gender: None,
                        medical_record_numbers: Vec::new(),
                    },
                    pulled_at: Some(now_ts),
                },
            )
            .await
            .unwrap();
    }

    // 1. Invite patient in `front-door`. Before joining (`Connected`), "For my doctor" fails closed.
    let roster = RosterPatient {
        practice_id: practice.clone(),
        patient_id: workspace.clone(),
    };
    let patient_email = format!("sam-{}@patient.test", Uuid::new_v4().simple());
    let pt_invite = front_door
        .invite(&roster, &patient_email, "Sam Rivera", now_u64)
        .await
        .unwrap();

    let unlinked = front_door
        .send_for_my_doctor(
            Uuid::new_v4(),
            ForMyDoctorSendRequest {
                share_key: "share-before-connect".into(),
                title: "Pre-connect note".into(),
                summary_text: "Should fail without active connection".into(),
                origin_label: None,
                target: Some(roster.clone()),
            },
            now_u64 + 2,
        )
        .await
        .unwrap_err();
    assert!(
        matches!(unlinked, ForMyDoctorError::Unlinked),
        "expected Unlinked before connection, got {unlinked:?}"
    );

    // 2. Redeem invitation (`Connected` = sharing is live) + stage Duet record -> immediate sync & priority wake.
    let roles = model_roles();
    let skills = load_skills();
    let (account_id, _pt_session) = front_door
        .redeem_invitation(&patient_email, &pt_invite, now_u64 + 10)
        .await
        .unwrap();
    front_door
        .upsert_shared_records(
            account_id,
            &[SharedPatientRecordInput {
                record_id: "rec-bp-1".into(),
                revision: "rev-1".into(),
                record_type: "Observation".into(),
                title: "Home BP Log".into(),
                content_text: "Morning home blood pressure 154/96 mmHg.".into(),
                recorded_at: now_u64 + 9,
            }],
            now_u64 + 10,
        )
        .await
        .unwrap();

    let join_report = store
        .sync_connected_care_from_front_door(
            auth.as_ref(),
            &front_door,
            &practice,
            &roles,
            &skills,
            auth.as_ref(),
            now_ts + 11,
        )
        .await
        .unwrap();
    assert_eq!(join_report.consented_workspaces_checked, 1);
    assert_eq!(join_report.consent_wakes_started, 1);
    assert_eq!(join_report.shares_ingested, 0);

    let obs_after_join = store.sources(&workspace).await.unwrap();
    let bp1 = obs_after_join
        .sources
        .iter()
        .find(|src| src.candidate_id == "duet-shared:rec-bp-1")
        .expect("rec-bp-1 filed into chart");
    assert_eq!(bp1.source_id, DUET_PATIENT_SHARE_SOURCE_ID);
    assert!(bp1.content.contains("154/96 mmHg"));

    let reviews_after_join = store
        .patient_reviews(&practice, &workspace, auth.as_ref())
        .await
        .unwrap();
    assert_eq!(reviews_after_join.len(), 1);
    let join_job = store
        .practice_job(&reviews_after_join[0].practice_scope())
        .await
        .unwrap()
        .expect("join wake job exists");
    assert_eq!(join_job.priority, JobPriority::Priority);

    // Repeating sync at the same `connected_at` does not duplicate the join wake.
    let replay_join = store
        .sync_connected_care_from_front_door(
            auth.as_ref(),
            &front_door,
            &practice,
            &roles,
            &skills,
            auth.as_ref(),
            now_ts + 12,
        )
        .await
        .unwrap();
    assert_eq!(replay_join.consent_wakes_started, 0);

    // 3. End connection: `send_for_my_doctor` fails, no new records sync, existing dated record stays.
    front_door
        .end_connection(
            account_id,
            &roster,
            ConnectionEndReason::PatientLeft,
            now_u64 + 20,
        )
        .await
        .unwrap();
    let ended_err = front_door
        .send_for_my_doctor(
            account_id,
            ForMyDoctorSendRequest {
                share_key: "share-while-ended".into(),
                title: "Ended note".into(),
                summary_text: "Must be rejected".into(),
                origin_label: None,
                target: Some(roster.clone()),
            },
            now_u64 + 21,
        )
        .await
        .unwrap_err();
    assert!(
        matches!(ended_err, ForMyDoctorError::Unlinked),
        "expected Unlinked after connection ended, got {ended_err:?}"
    );
    let ended_sync = store
        .sync_connected_care_from_front_door(
            auth.as_ref(),
            &front_door,
            &practice,
            &roles,
            &skills,
            auth.as_ref(),
            now_ts + 22,
        )
        .await
        .unwrap();
    assert_eq!(ended_sync.consented_workspaces_checked, 0);
    assert!(
        store
            .sources(&workspace)
            .await
            .unwrap()
            .sources
            .iter()
            .any(|src| src.candidate_id == "duet-shared:rec-bp-1"),
        "already-shared dated record must remain after connection ends"
    );

    // 4. Re-invite and rejoin + new Duet record + `"For my doctor"` share.
    let rejoin_ts = now_u64 + 50;
    let rejoin_invite = front_door
        .invite(&roster, &patient_email, "Sam Rivera", rejoin_ts - 1)
        .await
        .unwrap();
    let (rejoined_account_id, _) = front_door
        .redeem_invitation(&patient_email, &rejoin_invite, rejoin_ts)
        .await
        .unwrap();
    assert_eq!(rejoined_account_id, account_id);
    front_door
        .upsert_shared_records(
            account_id,
            &[SharedPatientRecordInput {
                record_id: "rec-bp-2".into(),
                revision: "rev-1".into(),
                record_type: "Observation".into(),
                title: "Evening BP Log".into(),
                content_text: "Evening home blood pressure 162/100 mmHg.".into(),
                recorded_at: rejoin_ts,
            }],
            rejoin_ts,
        )
        .await
        .unwrap();
    let share_key = format!("share-{}", Uuid::new_v4().simple());
    let sent_at = rejoin_ts + 5;
    let sent = front_door
        .send_for_my_doctor(
            account_id,
            ForMyDoctorSendRequest {
                share_key,
                title: "Chest tightness climbing stairs".into(),
                summary_text:
                    "Patient reports chest tightness since last night alongside BP 162/100.".into(),
                origin_label: None,
                target: None,
            },
            sent_at,
        )
        .await
        .unwrap();
    let share_id = sent.share_id.clone();
    assert_eq!(sent.origin_label, "Duet — For my doctor");
    assert_eq!(sent.standing, ForMyDoctorShareStatus::Sent);

    // Enqueue a normal-priority Periodic review on `workspace_other` BEFORE syncing connected care
    // so we prove Priority connected-care shares jump ahead of Normal background jobs.
    let obs_other = store.sources(&workspace_other).await.unwrap();
    let receiver_other = PatientReviewReceiver::new(&store, &roles, &skills, auth.as_ref());
    receiver_other
        .submit_coalesced_review(
            &ReviewRequest {
                practice_id: practice.clone(),
                patient_workspace_id: workspace_other.clone(),
                command_key: format!("cmd-normal-periodic-{}", Uuid::new_v4().simple()),
                reason: ReviewReason::Periodic,
                workspace_version: obs_other.version,
                record_versions: BTreeMap::new(),
            },
            &[ReviewDispatchInput::Periodic {
                workspace_version: obs_other.version,
                last_reviewed_at: None,
            }],
        )
        .await
        .unwrap();

    // Execute and settle the earlier join review cleanly via PracticeJobHost over SSE so the
    // subsequent rejoin + "For my doctor" share creates a fresh Priority review ahead of Normal.
    let (join_sse_state, mut join_sse_provider, join_sse_handle) =
        spawn_local_sse_provider(&store).await;
    join_sse_state.push_call(
        "call-read-join-bp",
        READ_TOOL,
        json!({
            "references": [{
                "candidate_id": "duet-shared:rec-bp-1",
                "revision": "rev-1",
                "cursor": null
            }],
            "max_result_bytes": 16384
        }),
    );
    join_sse_state.push_call(
        "call-complete-join",
        COMPLETE_PATIENT_REVIEW_TOOL,
        json!({
            "disposition": "quiet",
            "explanation": "Initial home blood pressure check filed; no acute concern yet.",
            "citations": [{
                "candidate_id": "duet-shared:rec-bp-1",
                "revision": "rev-1"
            }],
            "basis_and_limitations": "Initial consented Duet shared record.",
            "unfinished_work": []
        }),
    );
    join_sse_state.push_assistant("Settled initial join wake quietly.");
    let claim_old = store
        .claim_practice_job(&join_job.scope, 60, auth.as_ref())
        .await
        .unwrap()
        .expect("claim join job");
    let mut join_host = PracticeJobHost {
        store: store.clone(),
        claim: claim_old,
        authority: auth.as_ref(),
    };
    join_host
        .run(
            &mut join_sse_provider,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .expect("settle join wake cleanly");
    join_sse_handle.abort();

    let second_sync = store
        .sync_connected_care_from_front_door(
            auth.as_ref(),
            &front_door,
            &practice,
            &roles,
            &skills,
            auth.as_ref(),
            now_ts + 60,
        )
        .await
        .unwrap();
    assert_eq!(second_sync.consented_workspaces_checked, 1);
    assert_eq!(second_sync.consent_wakes_started, 1);
    assert_eq!(second_sync.shares_ingested, 1);
    assert_eq!(second_sync.shares_marked_received, 1);

    // Re-sync is a pure no-op (no duplicate share ingestion or status overwrite).
    let idempotent_sync = store
        .sync_connected_care_from_front_door(
            auth.as_ref(),
            &front_door,
            &practice,
            &roles,
            &skills,
            auth.as_ref(),
            now_ts + 61,
        )
        .await
        .unwrap();
    assert_eq!(idempotent_sync.shares_ingested, 0);
    assert_eq!(idempotent_sync.shares_marked_received, 0);
    assert_eq!(idempotent_sync.consent_wakes_started, 0);

    // Front-door sees `"received"` status ONLY (never `"physician looked"`).
    let account_shares = front_door
        .for_my_doctor_shares_for_account(account_id)
        .await
        .unwrap();
    assert_eq!(account_shares.len(), 1);
    assert_eq!(
        account_shares[0].standing,
        ForMyDoctorShareStatus::Received {
            received_at: (now_ts + 60) as u64
        }
    );

    // Chart now holds `rec-bp-2` and `for-my-doctor:<share_id>` with explicit origin & sent date.
    let expected_candidate = format!("for-my-doctor:{share_id}");
    let obs_after_share = store.sources(&workspace).await.unwrap();
    let fmd_rev = obs_after_share
        .sources
        .iter()
        .find(|src| src.candidate_id == expected_candidate)
        .expect("For my doctor share filed in chart");
    assert_eq!(fmd_rev.source_id, DUET_FOR_MY_DOCTOR_SOURCE_ID);
    assert!(fmd_rev.content.contains("Duet — For my doctor"));
    assert!(fmd_rev.content.contains("Chest tightness climbing stairs"));

    let synced_rows = store
        .synced_for_my_doctor_shares(&practice, &workspace)
        .await
        .unwrap();
    assert_eq!(synced_rows.len(), 1);
    assert_eq!(synced_rows[0].share_id, share_id);
    assert_eq!(synced_rows[0].received_written_at, Some(now_ts + 60));

    // Coalescing upgraded Sam Rivera's priority review with `ForMyDoctor` trigger & priority queue order.
    let due_scopes: Vec<PracticeScope> = store.practice_jobs_due(&practice, 10).await.unwrap();
    assert!(
        due_scopes.len() >= 2,
        "expected priority + normal due jobs: {due_scopes:?}"
    );
    let mut due_jobs = Vec::with_capacity(due_scopes.len());
    for scope in &due_scopes {
        due_jobs.push(
            store
                .practice_job(scope)
                .await
                .unwrap()
                .expect("due job row"),
        );
    }
    assert_eq!(due_jobs[0].scope.workspace, workspace);
    assert_eq!(due_jobs[0].priority, JobPriority::Priority);
    assert_eq!(
        due_jobs.last().unwrap().priority,
        JobPriority::Normal,
        "normal periodic job trails priority connected-care work"
    );

    // Execute the priority `"For my doctor"` review through `PracticeJobHost` over real SSE!
    let claim_priority = store
        .claim_practice_job(&due_scopes[0], 120, auth.as_ref())
        .await
        .unwrap()
        .expect("claim priority For my doctor job");
    let (sse_state, mut sse_provider, sse_handle) = spawn_local_sse_provider(&store).await;
    sse_state.push_call(
        "call-read-fmd",
        READ_TOOL,
        json!({
            "references": [{
                "candidate_id": expected_candidate,
                "revision": format!("sent-{sent_at}"),
                "cursor": null
            }],
            "max_result_bytes": 16384
        }),
    );
    sse_state.push_call(
        "call-raise-fmd-concern",
        CONCERN_TOOL,
        json!({
            "operation": "raise",
            "concern_id": format!("concern-fmd-{share_id}"),
            "urgency": "now",
            "reason": "Patient reports chest tightness climbing stairs with BP 162/100 in a Duet — For my doctor share.",
            "records": [{
                "candidate_id": expected_candidate,
                "revision": format!("sent-{sent_at}")
            }]
        }),
    );
    sse_state.push_call(
        "call-complete-fmd",
        COMPLETE_PATIENT_REVIEW_TOOL,
        json!({
            "disposition": "assessed",
            "summary": "Escalated Duet — For my doctor report of chest tightness with BP 162/100 to Today (Now).",
            "citations": [{
                "candidate_id": expected_candidate,
                "revision": format!("sent-{sent_at}")
            }],
            "basis_and_limitations": "Patient-prepared Duet — For my doctor share; awaiting direct clinical follow-up.",
            "unfinished_work": []
        }),
    );
    sse_state
        .push_assistant("Completed For my doctor review and escalated chest tightness concern.");

    let mut host = PracticeJobHost {
        store: store.clone(),
        claim: claim_priority,
        authority: auth.as_ref(),
    };
    host.run(
        &mut sse_provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("For my doctor review execution succeeds");
    sse_handle.abort();

    let requests = sse_state.requests();
    assert_eq!(requests[0]["model"], roles.background.model);

    // Read Today projection -> urgent concern surfaced immediately with patient name.
    let patient_names = BTreeMap::from([(workspace.clone(), "Sam Rivera".to_owned())]);
    let today = store
        .read_today(auth.as_ref(), &dr_token, now_u64 + 65, &patient_names)
        .await
        .unwrap();
    assert!(
        today.rows.iter().any(|row| {
            row.workspace == workspace
                && row.is_now()
                && (row.title.contains("Duet — For my doctor")
                    || row.detail.contains("Duet — For my doctor"))
        }),
        "expected escalated Today Now row for For my doctor share, got {:?}",
        today.rows
    );
}

#[tokio::test]
async fn cross_practice_connected_care_collision_refuses_foreign_chart_writes() {
    let Some(url) = test_database_url() else {
        return;
    };
    let (store, auth, front_door) = setup_stores(&url).await;
    let now_u64 = 1_763_300_000;
    let now_ts = now_u64 as i64;
    let practice_a = format!("practice-owner-a-{}", Uuid::new_v4().simple());
    let practice_b = format!("practice-foreign-b-{}", Uuid::new_v4().simple());
    let shared_workspace = format!("ws-collision-{}", Uuid::new_v4().simple());

    auth.register_patient(&practice_a, &shared_workspace)
        .await
        .unwrap();
    store
        .record_chart(
            &shared_workspace,
            &[],
            &SourcePull {
                source_id: "synthetic-clinic-ehr".into(),
                source_label: "Synthetic Clinic EHR".into(),
                patient: SourcePatient {
                    source_patient_id: shared_workspace.clone(),
                    display_name: "Practice A Patient".into(),
                    birth_date: Some("1970-01-15".into()),
                    sex_or_gender: None,
                    medical_record_numbers: Vec::new(),
                },
                pulled_at: Some(now_ts),
            },
        )
        .await
        .unwrap();

    // Practice B links an account against `shared_workspace`, grants consent, stages shared
    // records, and sends a "For my doctor" share.
    let foreign_email = format!("collision-{}@patient.test", Uuid::new_v4().simple());
    let invite = front_door
        .invite(
            &RosterPatient {
                practice_id: practice_b.clone(),
                patient_id: shared_workspace.clone(),
            },
            &foreign_email,
            "Foreign Collision",
            now_u64,
        )
        .await
        .unwrap();
    let (foreign_account, _) = front_door
        .redeem_invitation(&foreign_email, &invite, now_u64 + 1)
        .await
        .unwrap();
    front_door
        .upsert_shared_records(
            foreign_account,
            &[SharedPatientRecordInput {
                record_id: "rec-foreign-1".into(),
                revision: "rev-1".into(),
                record_type: "Observation".into(),
                title: "Foreign BP".into(),
                content_text: "Should never enter Practice A chart.".into(),
                recorded_at: now_u64 + 2,
            }],
            now_u64 + 2,
        )
        .await
        .unwrap();
    front_door
        .send_for_my_doctor(
            foreign_account,
            ForMyDoctorSendRequest {
                share_key: "share-foreign-collision".into(),
                title: "Foreign Note".into(),
                summary_text: "Should never enter Practice A chart.".into(),
                origin_label: None,
                target: None,
            },
            now_u64 + 3,
        )
        .await
        .unwrap();

    let roles = model_roles();
    let skills = load_skills();
    let err = store
        .sync_connected_care_from_front_door(
            auth.as_ref(),
            &front_door,
            &practice_b,
            &roles,
            &skills,
            auth.as_ref(),
            now_ts + 5,
        )
        .await
        .unwrap_err();
    match err {
        ConnectedCareError::ForeignWorkspace {
            requested_practice,
            owner_practice,
            workspace,
        } => {
            assert_eq!(requested_practice, practice_b);
            assert_eq!(owner_practice, practice_a);
            assert_eq!(workspace, shared_workspace);
        }
        other => panic!("expected ForeignWorkspace error, got {other:?}"),
    }

    // Practice A's chart must hold zero foreign Duet records or For my doctor shares.
    let obs_a = store.sources(&shared_workspace).await.unwrap();
    assert!(
        obs_a.sources.is_empty(),
        "foreign practice sync must not write into Practice A's sources: {:?}",
        obs_a.sources
    );
    let shares = front_door
        .for_my_doctor_shares_for_account(foreign_account)
        .await
        .unwrap();
    assert_eq!(shares[0].standing, ForMyDoctorShareStatus::Sent);
}
