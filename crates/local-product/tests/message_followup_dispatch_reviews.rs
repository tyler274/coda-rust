//! Real PostgreSQL integration proof for linked-message and due-follow-up
//! dispatch triggers, coalescing with chart record changes, background review
//! execution via `PracticeJobHost` + `ResponsesProvider` SSE, `Stop` / `Resume`
//! follow-up review controls across restart and worker replay, and saving reply
//! proposals strictly as physician drafts.

#![expect(
    dead_code,
    reason = "SSE provider helpers are exercised in companion suite slice"
)]
#![expect(
    unused_imports,
    reason = "follow-up and Stop/Resume imports are exercised in companion suite slice"
)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex as StdMutex, Weak};

use axum::{
    Json, Router,
    extract::State,
    http::{StatusCode, header},
    response::IntoResponse,
    routing::post,
};
use harness::provider::ResponsesProvider;
use local_product::initial_understanding::{ConnectedSourceBrief, EvidenceScope};
use local_product::patient_reviews::{
    COMPLETE_PATIENT_REVIEW_TOOL, INSTRUCTIONS, PATIENT_MESSAGE_REPLY_TOOL,
    READ_PATIENT_CONVERSATION_TOOL, ReviewOutcome, ReviewStartingContext, ReviewTrigger,
    assignment as review_assignment, complete_patient_review_tool, patient_message_reply_tool,
    read_patient_conversation_tool, read_patient_reviews_tool, recipe as review_recipe,
};
use local_product::physician_chat::PhysicianSkills;
use local_product::physician_sources::{
    MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
use local_product::runtime::chat_operations::{
    DispatchCheckStanding, PatientMemoryStanding, SourceFreshnessSummary,
};
use local_product::runtime::concerns::{ExactRecordReference, WorkspaceAccess};
use local_product::runtime::dispatch::{PracticeJobReviewReceiver, StoredObservationChecker};
use local_product::runtime::follow_ups::{
    FOLLOW_UP_TOOL, FollowUpCommandKey, FollowUpId, FollowUpReviewSchedule, FollowUpStanding,
    FollowUpSummary, PhysicianCreateFollowUp, follow_up_tool,
};
use local_product::runtime::jobs::{PracticeJobHost, PracticeJobKind, is_physician_only_tool};
use local_product::runtime::reply::proposals::{
    CardReviewState, CompleteReplyReview, ConversationLink, DisplayedReplyText,
    MessageIntakeOutcome, MessageSender, ObservePracticeMessage, PhysicianDraftStanding,
    PhysicianReplySurface, PracticeMessageKind, RecordPatientMessage, ReplyReviewDecision,
    ReplyStanding, ReviewWakeEvent,
};
use local_product::runtime::reviews::{
    PatientReviewReceiver, PatientReviewTarget, RecordedReviewAction, ReviewHoldReason,
    ReviewLifecycleState, VersionedReviewAction, VersionedReviewControl, patient_review_version,
};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::spruce_matching::{
    IncomingSpruceItem, MatchAssessment, MatchingDecision, MessageUrgencyChoice,
    OutsideParticipant, ReadableSpruceItem, SenderAddress,
};
use local_product::runtime::{
    Authority, PublicationTransaction, Scope, Store, WorkStatus, async_trait, engine,
};
use local_product::spruce_matching_recipe::{GUIDANCE_VERSION, RECIPE_VERSION};
use local_product::{ModelBinding, ModelRoles, sha256_hex};
use patient_work::{ReviewDispatchInput, ReviewReason};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use serde_json::{Value, json};
use uuid::Uuid;

#[allow(dead_code)]
#[path = "../../../verification/model_text.rs"]
mod model_text;

struct LocalCluster {
    cluster_dir: PathBuf,
    data_dir: PathBuf,
    admin_url: String,
}

impl Drop for LocalCluster {
    fn drop(&mut self) {
        let _ = Command::new("pg_ctl")
            .args([
                "-D",
                self.data_dir.to_str().unwrap_or_default(),
                "-m",
                "immediate",
                "stop",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        let _ = std::fs::remove_dir_all(&self.cluster_dir);
    }
}

static LOCAL_CLUSTER: StdMutex<Weak<LocalCluster>> = StdMutex::new(Weak::new());
static MIGRATE_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

fn acquire_database() -> Option<(String, Option<Arc<LocalCluster>>)> {
    for key in [
        "PHYSICIAN_RUNTIME_TEST_DATABASE_URL",
        "PHYSICIAN_DATABASE_URL",
        "PHYSICIAN_TEST_DATABASE_URL",
    ] {
        if let Ok(url) = std::env::var(key) {
            let trimmed = url.trim().to_owned();
            if !trimmed.is_empty() {
                return Some((trimmed, None));
            }
        }
    }

    let mut slot = LOCAL_CLUSTER
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(existing) = slot.upgrade() {
        return Some((existing.admin_url.clone(), Some(existing)));
    }

    let port = {
        use std::net::TcpListener;
        (0..500)
            .find_map(|_| {
                let candidate = 23_000 + (Uuid::new_v4().as_u128() % 9_000) as u16;
                TcpListener::bind(("127.0.0.1", candidate))
                    .ok()
                    .map(|_| candidate)
            })
            .expect("free loopback port for local postgres")
    };
    let cluster_dir =
        std::env::temp_dir().join(format!("coda-mf-pg-{}-{}", std::process::id(), port));
    let _ = std::fs::remove_dir_all(&cluster_dir);
    let data_dir = cluster_dir.join("data");
    let sock_dir = cluster_dir.join("sock");
    std::fs::create_dir_all(&data_dir).ok()?;
    std::fs::create_dir_all(&sock_dir).ok()?;

    let init = Command::new("initdb")
        .args([
            "-D",
            data_dir.to_str()?,
            "-U",
            "postgres",
            "-A",
            "trust",
            "--no-locale",
            "-E",
            "UTF8",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok()?;
    if !init.success() {
        let _ = std::fs::remove_dir_all(&cluster_dir);
        return None;
    }

    let log_file = cluster_dir.join("postgres.log");
    let opts = format!("-F -p {port} -h 127.0.0.1 -k {}", sock_dir.to_str()?);
    let start = Command::new("pg_ctl")
        .args([
            "-D",
            data_dir.to_str()?,
            "-l",
            log_file.to_str()?,
            "-o",
            &opts,
            "-w",
            "start",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok()?;
    if !start.success() {
        let _ = std::fs::remove_dir_all(&cluster_dir);
        return None;
    }

    let cluster = Arc::new(LocalCluster {
        cluster_dir,
        data_dir,
        admin_url: format!("postgresql://postgres@127.0.0.1:{port}/postgres"),
    });
    *slot = Arc::downgrade(&cluster);
    Some((cluster.admin_url.clone(), Some(cluster)))
}

async fn connect_stores() -> Option<(
    Store,
    Arc<PostgresAuthStore>,
    String,
    Option<Arc<LocalCluster>>,
)> {
    let Some((url, cluster)) = acquire_database() else {
        assert!(
            std::env::var("CODA_REQUIRE_POSTGRES_TESTS").as_deref() != Ok("1"),
            "CODA_REQUIRE_POSTGRES_TESTS=1 requires PHYSICIAN_RUNTIME_TEST_DATABASE_URL"
        );
        return None;
    };
    let _guard = MIGRATE_LOCK.lock().await;
    let store = Store::connect(&url).await.expect("connect runtime store");
    store.migrate().await.expect("migrate runtime store");
    let auth = Arc::new(
        PostgresAuthStore::connect_with_migrations(&url)
            .await
            .expect("connect and migrate physician auth store"),
    );
    drop(_guard);
    Some((store, auth, url, cluster))
}

fn current_unix_i64() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock after epoch")
        .as_secs() as i64
}

fn load_skills() -> PhysicianSkills {
    PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
        .expect("load physician skills catalog")
}

fn distinct_roles() -> ModelRoles {
    ModelRoles {
        chat: ModelBinding {
            model: "interactive-chat-lane".into(),
            effort: "low".into(),
            context_window_tokens: 128_000,
            max_input_tokens: 100_000,
            adapter: "fixture".into(),
        },
        background: ModelBinding {
            model: "background-rounds-lane".into(),
            effort: "high".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 250_000,
            adapter: "fixture".into(),
        },
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

async fn invite_and_sign_in(
    auth: &PostgresAuthStore,
    practice_id: &str,
    label: &str,
    now_unix: u64,
) -> (PhysicianIdentity, String, Vec<u8>) {
    let email = format!("{label}-{}@codahealth.test", Uuid::new_v4().simple());
    let invite = auth
        .invite(practice_id, &email, label, true, now_unix)
        .await
        .expect("invite physician");
    let (identity, token) = auth
        .redeem_invitation(&email, &invite, now_unix)
        .await
        .expect("redeem invitation");
    let hash = PostgresAuthStore::session_reference(&token).to_vec();
    (identity, token, hash)
}

fn source_patient(id: &str, name: &str) -> SourcePatient {
    SourcePatient {
        source_patient_id: id.to_owned(),
        display_name: name.to_owned(),
        birth_date: Some("1974-09-11".to_owned()),
        sex_or_gender: Some("female".to_owned()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "clinic-mrn".to_owned(),
            value: format!("MRN-{id}"),
        }],
    }
}

fn chart_record(
    workspace: &str,
    candidate_id: &str,
    revision: &str,
    title: &str,
    date: &str,
    text: &str,
) -> SourceRevision {
    SourceRevision {
        workspace: workspace.to_owned(),
        source_id: "clinic-ehr".to_owned(),
        candidate_id: candidate_id.to_owned(),
        revision: revision.to_owned(),
        record_type: "Observation".to_owned(),
        dates: vec![RecordDate {
            name: "effectiveDateTime".to_owned(),
            value: date.to_owned(),
        }],
        content: text.to_owned(),
        sha256: sha256_hex(text.as_bytes()),
        readable_parts: vec![ReadablePart {
            label: title.to_owned(),
            text: text.to_owned(),
        }],
        provenance: json!({
            "title": title,
            "record_locator": candidate_id,
        }),
        envelope: None,
    }
}

struct BaselineChartSeed<'a> {
    practice: &'a str,
    workspace: &'a str,
    patient_name: &'a str,
    candidate_id: &'a str,
    revision: &'a str,
    text: &'a str,
    reg_time: i64,
}

async fn seed_patient_with_baseline_chart(
    store: &Store,
    auth: &PostgresAuthStore,
    seed: BaselineChartSeed<'_>,
) {
    auth.register_patient(seed.practice, seed.workspace)
        .await
        .expect("register patient");
    let pull = SourcePull {
        source_id: "clinic-ehr".to_owned(),
        source_label: "Clinic EHR".to_owned(),
        patient: source_patient(seed.workspace, seed.patient_name),
        pulled_at: Some(seed.reg_time),
    };
    let rec = chart_record(
        seed.workspace,
        seed.candidate_id,
        seed.revision,
        "Baseline Metabolic Panel",
        "2026-11-10",
        seed.text,
    );
    store
        .record_chart(seed.workspace, &[rec], &pull)
        .await
        .expect("record baseline chart");
    store
        .register_patient_dispatch(auth, seed.practice, seed.workspace, seed.reg_time)
        .await
        .expect("register patient dispatch");
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
    script: Arc<StdMutex<VecDeque<ScriptedSseStep>>>,
    captured_requests: Arc<StdMutex<Vec<Value>>>,
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
                            "input_tokens": 640,
                            "input_tokens_details": { "cached_tokens": 120 },
                            "output_tokens": 95
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
        .expect("bind local SSE listener");
    let addr = listener.local_addr().expect("SSE listener addr");
    let handle = tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    let provider = ResponsesProvider::new(
        format!("http://{addr}/v1/responses"),
        "local-sse-key".into(),
        Arc::new(store.clone()),
    )
    .expect("build ResponsesProvider");
    (sse_state, provider, handle)
}

#[test]
fn message_and_follow_up_review_recipe_and_tools_have_clean_model_text() {
    let skills = load_skills();
    let roles = distinct_roles();

    for tool in [
        read_patient_reviews_tool(),
        read_patient_conversation_tool(),
        patient_message_reply_tool(),
        follow_up_tool(),
        complete_patient_review_tool(),
    ] {
        model_text::assert_clean_model_text(&tool.description);
        model_text::assert_clean_model_text(&serde_json::to_string(&tool.function()).unwrap());
        assert!(
            !is_physician_only_tool(&tool.name),
            "background review tool {} must not be classified physician-only",
            tool.name
        );
    }

    for trigger in [
        ReviewTrigger::Message {
            conversation_id: "conv-clean-1".into(),
            message_ids: vec!["msg-clean-1".into()],
            workspace_version: 2,
            record_versions: BTreeMap::from([("Observation/bmp-1".to_owned(), "rev-1".to_owned())]),
        },
        ReviewTrigger::FollowUp {
            follow_up_id: "fu-clean-1".into(),
            version: 1,
            summary: "Recheck serum potassium within 7 days.".into(),
            due_at: 1_794_500_000,
            workspace_version: 2,
            record_versions: BTreeMap::from([("Observation/bmp-1".to_owned(), "rev-1".to_owned())]),
        },
    ] {
        let context = ReviewStartingContext {
            workspace: "pw-clean-msg-fu".into(),
            review_id: "review-clean-msg-fu".into(),
            patient: Some(source_patient("pt-clean", "Clara Chen")),
            trigger,
            additional_inputs: vec![
                "Coalesced due follow-up `fu-clean-2` (version 1): Check home blood pressure log."
                    .into(),
            ],
            chart_version: 2,
            chart_record_count: 1,
            evidence_scope: EvidenceScope::ClinicalChart {
                connected_sources: vec![ConnectedSourceBrief {
                    source_id: "clinic-ehr".into(),
                    source_label: "Clinic EHR".into(),
                    pulled_at: 1_794_400_000,
                }],
            },
            source_pulls: vec![SourceFreshnessSummary {
                source_id: "clinic-ehr".into(),
                source_label: "Clinic EHR".into(),
                pulled_at: 1_794_400_000,
            }],
            refresh_statuses: Vec::new(),
            check_standing: DispatchCheckStanding::Current {
                checked_at: 1_794_400_000,
            },
            understanding: None,
            withdrawal_boundary: None,
            memory_standing: PatientMemoryStanding::Available {
                entries: Vec::new(),
            },
            open_concerns: Vec::new(),
            prior_reviews: Vec::new(),
        };
        let assignment = review_assignment("cmd-clean-msg-fu", context, &skills).unwrap();
        let built = review_recipe(&roles.background, &assignment, &skills);
        model_text::assert_clean_model_text(INSTRUCTIONS);
        model_text::assert_clean_model_text(&built.instructions);
        model_text::assert_clean_model_text(&assignment.prompt);
        model_text::assert_clean_model_text(&serde_json::to_string(&built.tools).unwrap());
    }
}

#[tokio::test]
async fn linked_message_arrival_starts_one_review_with_reason_message() {
    let Some((store, auth, url, _cluster)) = connect_stores().await else {
        return;
    };
    let skills = load_skills();
    let roles = distinct_roles();
    let reg_time = current_unix_i64();
    let practice = format!("prac-msg-disp-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    let (dr, _token, _hash) =
        invite_and_sign_in(&auth, &practice, "DrMessage", reg_time as u64).await;

    let ws_direct = format!("pw-msg-direct-{}", Uuid::new_v4().simple());
    let ws_spruce = format!("pw-msg-spruce-{}", Uuid::new_v4().simple());
    for (ws, name) in [(&ws_direct, "Nora Patel"), (&ws_spruce, "Liam Brooks")] {
        seed_patient_with_baseline_chart(
            &store,
            &auth,
            BaselineChartSeed {
                practice: &practice,
                workspace: ws,
                patient_name: name,
                candidate_id: "Observation/bmp-base",
                revision: "rev-1",
                text: "Serum potassium 4.2 mmol/L (reference 3.5-5.1 mmol/L).",
                reg_time,
            },
        )
        .await;
        auth.set_membership(&dr, ws, Membership::Allowed)
            .await
            .unwrap();
    }

    let checker = StoredObservationChecker::new(store.clone());
    let receiver = PatientReviewReceiver::new(&store, &roles, &skills, auth.as_ref());

    // Baseline wake at `reg_time` starts zero reviews because registration seeded the chart baseline.
    let initial_wake = store
        .wake_practice_dispatch(auth.as_ref(), &practice, reg_time, &checker, &receiver)
        .await
        .unwrap();
    assert!(initial_wake.started_reviews.is_empty());
    // Complete or supersede baseline reviews? Actually let's check: with no new events at `reg_time + 10`, 0 reviews start!
    let quiet_wake = store
        .wake_practice_dispatch(auth.as_ref(), &practice, reg_time + 10, &checker, &receiver)
        .await
        .unwrap();
    assert!(quiet_wake.started_reviews.is_empty());

    // 1. Direct linked reply conversation arrival on `ws_direct` (scheduled check is NOT due yet).
    store
        .link_reply_conversation(
            &practice,
            "conv-direct-1",
            ConversationLink::LinkedPatient {
                workspace: ws_direct.clone(),
            },
        )
        .await
        .unwrap();
    assert!(matches!(
        store
            .record_linked_patient_message(&RecordPatientMessage {
                practice: practice.clone(),
                conversation_id: "conv-direct-1".into(),
                message_id: "pm-direct-1".into(),
                sender: MessageSender::Patient,
                text: "Was my potassium normal on Monday's lab draw?".into(),
                sent_at: reg_time + 20,
            })
            .await
            .unwrap(),
        MessageIntakeOutcome::Recorded { .. }
    ));

    // 2. Linked Spruce channel arrival on `ws_spruce` (ingested without immediate receiver so dispatcher handles it).
    let liam_phone = SenderAddress::parse("+14155550711").unwrap();
    store
        .connect_spruce_channel(auth.as_ref(), &dr, 0, None, reg_time)
        .await
        .unwrap();
    let lease = store
        .acquire_spruce_reader_lease(&practice, "worker-msg-disp", 300, reg_time)
        .await
        .unwrap();
    store
        .record_spruce_confirmed_sender(
            &practice,
            &liam_phone,
            &ws_spruce,
            &dr.physician_id,
            reg_time,
        )
        .await
        .unwrap();
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &IncomingSpruceItem::Readable(ReadableSpruceItem {
                message_id: "spruce-msg-1".into(),
                conversation_id: "conv-spruce-1".into(),
                channel_position: 1,
                content_version: "v1".into(),
                sender_address: Some(liam_phone.clone()),
                author_name: Some("Liam Brooks".into()),
                body_text: "Could you let me know if I should stay on my current dose?".into(),
                attachments: Vec::new(),
                app_url: Some("https://app.sprucehealth.com/conversations/conv-spruce-1".into()),
                created_at_spruce: Some("2026-11-12T15:00:25Z".into()),
                outside_participants: vec![OutsideParticipant {
                    display_name: Some("Liam Brooks".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(liam_phone),
                    saved_contact_id: None,
                }],
            }),
            reg_time + 25,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "spruce-msg-1",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::SuggestedPatient {
                    workspace: ws_spruce.clone(),
                    evidence_summary: "Confirmed phone for Liam Brooks.".into(),
                },
                rationale: "Patient question about medication dose.".into(),
            },
            None,
            reg_time + 26,
        )
        .await
        .unwrap();

    // Wake dispatcher at `reg_time + 30`: starts ONE review per patient, both with `reason == "message"`.
    let msg_wake = store
        .wake_practice_dispatch(auth.as_ref(), &practice, reg_time + 30, &checker, &receiver)
        .await
        .expect("wake dispatches linked message reviews");
    assert_eq!(msg_wake.started_reviews.len(), 2);
    for req in &msg_wake.started_reviews {
        assert_eq!(req.reason, ReviewReason::Message);
        assert_eq!(req.reason.as_str(), "message");
    }

    // Reconnect (simulating restart) and wake again: starts ZERO duplicate reviews for the same message arrivals!
    let reopened = Store::connect(&url).await.unwrap();
    let reopened_checker = StoredObservationChecker::new(reopened.clone());
    let reopened_receiver = PatientReviewReceiver::new(&reopened, &roles, &skills, auth.as_ref());
    let replay_wake = reopened
        .wake_practice_dispatch(
            auth.as_ref(),
            &practice,
            reg_time + 35,
            &reopened_checker,
            &reopened_receiver,
        )
        .await
        .unwrap();
    assert!(
        replay_wake.started_reviews.is_empty(),
        "already-dispatched linked messages must not start duplicate reviews across restart: {:?}",
        replay_wake.started_reviews
    );

    // Verify both saved reviews carry `ReviewTrigger::Message` and preloaded `patient-communication` skill.
    for (ws, expected_conv, expected_msg) in [
        (&ws_direct, "conv-direct-1", "pm-direct-1"),
        (&ws_spruce, "conv-spruce-1", "spruce-msg-1"),
    ] {
        let saved = reopened
            .patient_reviews(&practice, ws, auth.as_ref())
            .await
            .unwrap();
        let latest = saved
            .iter()
            .find(|r| r.state == ReviewLifecycleState::Pending)
            .expect("pending message review");
        assert!(
            matches!(
                latest.triggers.as_slice(),
                [ReviewTrigger::Message {
                    conversation_id,
                    message_ids,
                    ..
                }] if conversation_id == expected_conv && message_ids.contains(&expected_msg.to_owned())
            ),
            "unexpected triggers on {ws}: {:?}",
            latest.triggers
        );
        let skill_names: BTreeSet<&str> = latest
            .recipe_version
            .preloaded_skills
            .iter()
            .map(|s| s.name.as_str())
            .collect();
        assert!(skill_names.contains("patient-record-retrieval"));
        assert!(skill_names.contains("patient-communication"));
    }
}
