//! Real PostgreSQL integration proof for linked-message and due-follow-up
//! dispatch triggers, coalescing with chart record changes, background review
//! execution via `PracticeJobHost` + `ResponsesProvider` SSE, `Stop` / `Resume`
//! follow-up review controls across restart and worker replay, and saving reply
//! proposals strictly as physician drafts.

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
        pulled_at: seed.reg_time,
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

#[tokio::test]
async fn due_follow_up_starts_one_review_of_current_version_while_held_completed_cancelled_start_none()
 {
    let Some((store, auth, _url, _cluster)) = connect_stores().await else {
        return;
    };
    let skills = load_skills();
    let roles = distinct_roles();
    let reg_time = current_unix_i64();
    let practice = format!("prac-fu-disp-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    let (dr, _token, dr_hash) =
        invite_and_sign_in(&auth, &practice, "DrFollowUp", reg_time as u64).await;

    let ws_due = format!("pw-fu-due-{}", Uuid::new_v4().simple());
    let ws_held = format!("pw-fu-held-{}", Uuid::new_v4().simple());
    let ws_completed = format!("pw-fu-comp-{}", Uuid::new_v4().simple());
    let ws_cancelled = format!("pw-fu-canc-{}", Uuid::new_v4().simple());
    let ws_future = format!("pw-fu-future-{}", Uuid::new_v4().simple());

    for ws in [&ws_due, &ws_held, &ws_completed, &ws_cancelled, &ws_future] {
        seed_patient_with_baseline_chart(
            &store,
            &auth,
            BaselineChartSeed {
                practice: &practice,
                workspace: ws,
                patient_name: "FollowUp Test Patient",
                candidate_id: "Observation/bmp-fu",
                revision: "rev-1",
                text: "Serum creatinine 1.1 mg/dL, potassium 4.4 mmol/L.",
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

    // Settle initial registration wake at `reg_time`.
    let _ = store
        .wake_practice_dispatch(auth.as_ref(), &practice, reg_time, &checker, &receiver)
        .await
        .unwrap();
    let _ = receiver.take_deliveries().await;

    let mut authority = PostgresWorkspaceAuthority;

    // 1. Active due follow-up on `ws_due` (due at `reg_time + 60`).
    let due_id = FollowUpId::new("fu-active-due").unwrap();
    store
        .create_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws_due,
                session_hash: &dr_hash,
            },
            &FollowUpCommandKey::new("cmd-fu-create-due").unwrap(),
            &PhysicianCreateFollowUp {
                id: due_id.clone(),
                summary: FollowUpSummary::new("Recheck potassium 1 week after ACEi start").unwrap(),
                due_at: reg_time + 60,
                records: vec![ExactRecordReference::new("Observation/bmp-fu", "rev-1").unwrap()],
            },
            &mut authority,
        )
        .await
        .unwrap();

    // 2. Held follow-up on `ws_held` (due_at is in the past relative to wake, but standing is `Held`).
    let held_id = FollowUpId::new("fu-held").unwrap();
    store
        .create_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws_held,
                session_hash: &dr_hash,
            },
            &FollowUpCommandKey::new("cmd-fu-create-held").unwrap(),
            &PhysicianCreateFollowUp {
                id: held_id.clone(),
                summary: FollowUpSummary::new("Paused lab check while patient travels").unwrap(),
                due_at: reg_time + 60,
                records: vec![],
            },
            &mut authority,
        )
        .await
        .unwrap();
    store
        .stop_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws_held,
                session_hash: &dr_hash,
            },
            &FollowUpCommandKey::new("cmd-fu-stop-held").unwrap(),
            &held_id,
            None,
            &mut authority,
        )
        .await
        .unwrap();

    // 3. Completed follow-up on `ws_completed`.
    let comp_id = FollowUpId::new("fu-completed").unwrap();
    store
        .create_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws_completed,
                session_hash: &dr_hash,
            },
            &FollowUpCommandKey::new("cmd-fu-create-comp").unwrap(),
            &PhysicianCreateFollowUp {
                id: comp_id.clone(),
                summary: FollowUpSummary::new("Completed repeat metabolic panel").unwrap(),
                due_at: reg_time + 60,
                records: vec![],
            },
            &mut authority,
        )
        .await
        .unwrap();
    store
        .complete_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws_completed,
                session_hash: &dr_hash,
            },
            &FollowUpCommandKey::new("cmd-fu-complete").unwrap(),
            &comp_id,
            local_product::runtime::follow_ups::FollowUpChangeNote::new("Repeat labs normal")
                .unwrap(),
            &mut authority,
        )
        .await
        .unwrap();

    // 4. Cancelled follow-up on `ws_cancelled`.
    let canc_id = FollowUpId::new("fu-cancelled").unwrap();
    store
        .create_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws_cancelled,
                session_hash: &dr_hash,
            },
            &FollowUpCommandKey::new("cmd-fu-create-canc").unwrap(),
            &PhysicianCreateFollowUp {
                id: canc_id.clone(),
                summary: FollowUpSummary::new("Cancelled imaging follow-up").unwrap(),
                due_at: reg_time + 60,
                records: vec![],
            },
            &mut authority,
        )
        .await
        .unwrap();
    store
        .cancel_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws_cancelled,
                session_hash: &dr_hash,
            },
            &FollowUpCommandKey::new("cmd-fu-cancel").unwrap(),
            &canc_id,
            local_product::runtime::follow_ups::FollowUpChangeNote::new(
                "Medication discontinued instead",
            )
            .unwrap(),
            &mut authority,
        )
        .await
        .unwrap();

    // 5. Future-scheduled active follow-up on `ws_future` (`due_at = reg_time + 7200`).
    store
        .create_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws_future,
                session_hash: &dr_hash,
            },
            &FollowUpCommandKey::new("cmd-fu-create-future").unwrap(),
            &PhysicianCreateFollowUp {
                id: FollowUpId::new("fu-future").unwrap(),
                summary: FollowUpSummary::new("Check lipid panel in 3 months").unwrap(),
                due_at: reg_time + 7_200,
                records: vec![],
            },
            &mut authority,
        )
        .await
        .unwrap();

    // Wake at `reg_time + 120`: ONLY `ws_due` starts a review, and its reason is `"follow-up"` at version 1!
    let wake1 = store
        .wake_practice_dispatch(
            auth.as_ref(),
            &practice,
            reg_time + 120,
            &checker,
            &receiver,
        )
        .await
        .expect("wake dispatches due follow-up only");
    assert_eq!(
        wake1.started_reviews.len(),
        1,
        "held, completed, cancelled, and future follow-ups must start zero reviews: {:?}",
        wake1.started_reviews
    );
    assert_eq!(wake1.started_reviews[0].patient_workspace_id, ws_due);
    assert_eq!(wake1.started_reviews[0].reason, ReviewReason::FollowUp);
    assert_eq!(wake1.started_reviews[0].reason.as_str(), "follow-up");

    let due_reviews_v1 = store
        .patient_reviews(&practice, &ws_due, auth.as_ref())
        .await
        .unwrap();
    let rev_v1 = due_reviews_v1
        .iter()
        .find(|r| r.state == ReviewLifecycleState::Pending)
        .expect("pending v1 follow-up review");
    assert!(matches!(
        rev_v1.triggers.as_slice(),
        [ReviewTrigger::FollowUp {
            follow_up_id,
            version: 1,
            ..
        }] if follow_up_id == "fu-active-due"
    ));

    // Re-waking at `reg_time + 180` without a version bump starts ZERO reviews for version 1.
    let wake_dup = store
        .wake_practice_dispatch(
            auth.as_ref(),
            &practice,
            reg_time + 180,
            &checker,
            &receiver,
        )
        .await
        .unwrap();
    assert!(wake_dup.started_reviews.is_empty());

    // Revising `fu-active-due` to version 2 (due at `reg_time + 200`) starts ONE review of version 2 at `reg_time + 240`!
    store
        .change_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws_due,
                session_hash: &dr_hash,
            },
            &FollowUpCommandKey::new("cmd-fu-change-v2").unwrap(),
            &local_product::runtime::follow_ups::PhysicianChangeFollowUp {
                id: due_id,
                summary: FollowUpSummary::new(
                    "Recheck potassium and creatinine after dose increase",
                )
                .unwrap(),
                due_at: reg_time + 200,
                records: vec![ExactRecordReference::new("Observation/bmp-fu", "rev-1").unwrap()],
                note: local_product::runtime::follow_ups::FollowUpChangeNote::new(
                    "Added creatinine check",
                )
                .unwrap(),
            },
            &mut authority,
        )
        .await
        .unwrap();

    let wake_v2 = store
        .wake_practice_dispatch(
            auth.as_ref(),
            &practice,
            reg_time + 240,
            &checker,
            &receiver,
        )
        .await
        .unwrap();
    assert_eq!(wake_v2.started_reviews.len(), 1);
    assert_eq!(wake_v2.started_reviews[0].patient_workspace_id, ws_due);
    assert_eq!(wake_v2.started_reviews[0].reason, ReviewReason::FollowUp);

    let due_reviews_v2 = store
        .patient_reviews(&practice, &ws_due, auth.as_ref())
        .await
        .unwrap();
    let rev_v2 = due_reviews_v2
        .iter()
        .find(|r| r.state == ReviewLifecycleState::Pending)
        .expect("pending v2 follow-up review");
    assert!(matches!(
        rev_v2.triggers.as_slice(),
        [ReviewTrigger::FollowUp {
            follow_up_id,
            version: 2,
            ..
        }] if follow_up_id == "fu-active-due"
    ));
}

#[tokio::test]
async fn linked_message_and_record_change_coalesce_into_single_review() {
    let Some((store, auth, _url, _cluster)) = connect_stores().await else {
        return;
    };
    let skills = load_skills();
    let roles = distinct_roles();
    let reg_time = current_unix_i64();
    let practice = format!("prac-coal-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();

    let ws = format!("pw-coalesce-{}", Uuid::new_v4().simple());
    seed_patient_with_baseline_chart(
        &store,
        &auth,
        BaselineChartSeed {
            practice: &practice,
            workspace: &ws,
            patient_name: "Sofia Alvarez",
            candidate_id: "Observation/a1c-1",
            revision: "rev-1",
            text: "Hemoglobin A1c 7.1%.",
            reg_time,
        },
    )
    .await;

    let checker = StoredObservationChecker::new(store.clone());
    let receiver = PatientReviewReceiver::new(&store, &roles, &skills, auth.as_ref());

    // Registration wake at `reg_time` starts zero reviews on unchanged baseline chart.
    let initial = store
        .wake_practice_dispatch(auth.as_ref(), &practice, reg_time, &checker, &receiver)
        .await
        .unwrap();
    assert!(initial.started_reviews.is_empty());

    // Seed BOTH a new chart record AND a linked patient message on `ws` before the next wake!
    let new_pull = SourcePull {
        source_id: "clinic-ehr".to_owned(),
        source_label: "Clinic EHR".to_owned(),
        patient: source_patient(&ws, "Sofia Alvarez"),
        pulled_at: reg_time + 86_405,
    };
    let new_lab = chart_record(
        &ws,
        "Observation/potassium-new",
        "rev-1",
        "Repeat Potassium",
        "2026-11-11",
        "Serum potassium 5.4 mmol/L (high; reference 3.5-5.1 mmol/L).",
    );
    store
        .record_chart(&ws, &[new_lab], &new_pull)
        .await
        .unwrap();

    store
        .link_reply_conversation(
            &practice,
            "conv-sofia-1",
            ConversationLink::LinkedPatient {
                workspace: ws.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: "conv-sofia-1".into(),
            message_id: "pm-sofia-1".into(),
            sender: MessageSender::Patient,
            text: "My lab results just posted in the portal — should I adjust my diet?".into(),
            sent_at: reg_time + 86_410,
        })
        .await
        .unwrap();

    // Wake at `reg_time + 86_420` (daily check is due AND linked message is waiting):
    // MUST emit ONE review request carrying BOTH inputs!
    let coalesced_wake = store
        .wake_practice_dispatch(
            auth.as_ref(),
            &practice,
            reg_time + 86_420,
            &checker,
            &receiver,
        )
        .await
        .expect("coalesced wake");
    assert_eq!(
        coalesced_wake.started_reviews.len(),
        1,
        "linked message + record change for same patient must start ONE review: {:?}",
        coalesced_wake.started_reviews
    );
    let req = &coalesced_wake.started_reviews[0];
    let inputs = coalesced_wake
        .coalesced_inputs
        .get(&req.command_key)
        .expect("coalesced_inputs recorded for combined review");
    assert!(
        inputs
            .iter()
            .any(|i| matches!(i, ReviewDispatchInput::ChangedRecords { .. }))
    );
    assert!(inputs.iter().any(|i| matches!(
        i,
        ReviewDispatchInput::LinkedMessage(m) if m.conversation_id == "conv-sofia-1"
    )));

    let deliveries = receiver.take_deliveries().await;
    assert_eq!(deliveries.len(), 1);

    let saved = store
        .patient_reviews(&practice, &ws, auth.as_ref())
        .await
        .unwrap();
    let pending: Vec<_> = saved
        .iter()
        .filter(|r| r.state == ReviewLifecycleState::Pending)
        .collect();
    assert_eq!(pending.len(), 1, "exactly one active pending review");
    let combined = pending[0];
    assert_eq!(combined.triggers.len(), 2);
    assert!(combined.triggers.iter().any(|t| matches!(
        t,
        ReviewTrigger::ChangedRecords {
            changed_candidates,
            ..
        } if changed_candidates.contains(&"Observation/potassium-new".to_owned())
    )));
    assert!(combined.triggers.iter().any(|t| matches!(
        t,
        ReviewTrigger::Message {
            conversation_id,
            message_ids,
            ..
        } if conversation_id == "conv-sofia-1" && message_ids.contains(&"pm-sofia-1".to_owned())
    )));
}

#[tokio::test]
async fn end_to_end_sse_dispatch_executes_message_followup_and_combined_reviews_once_and_saves_reply_drafts()
 {
    let Some((store, auth, _url, _cluster)) = connect_stores().await else {
        return;
    };
    let skills = load_skills();
    let roles = distinct_roles();
    let reg_time = current_unix_i64();
    let practice = format!("prac-sse-e2e-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    let (dr, _token, dr_hash) =
        invite_and_sign_in(&auth, &practice, "DrSse", reg_time as u64).await;

    let ws_msg = format!("pw-sse-msg-{}", Uuid::new_v4().simple());
    let ws_fu = format!("pw-sse-fu-{}", Uuid::new_v4().simple());
    let ws_combo = format!("pw-sse-combo-{}", Uuid::new_v4().simple());

    for (ws, name) in [
        (&ws_msg, "Hannah Park"),
        (&ws_fu, "David Miller"),
        (&ws_combo, "Maya Lin"),
    ] {
        seed_patient_with_baseline_chart(
            &store,
            &auth,
            BaselineChartSeed {
                practice: &practice,
                workspace: ws,
                patient_name: name,
                candidate_id: "Observation/bmp-1",
                revision: "rev-1",
                text: "Serum potassium 4.1 mmol/L, serum creatinine 0.9 mg/dL.",
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

    // Settle initial baseline dispatch at `reg_time` and clear those initial pending jobs via `stop`?
    // Wait: let's complete or let the new review supersede them — new review admission supersedes prior incomplete reviews cleanly!
    let _ = store
        .wake_practice_dispatch(auth.as_ref(), &practice, reg_time, &checker, &receiver)
        .await
        .unwrap();
    let _ = receiver.take_deliveries().await;

    // --- Seed Case 1: Linked message review on `ws_msg` ---
    store
        .link_reply_conversation(
            &practice,
            "conv-hannah",
            ConversationLink::LinkedPatient {
                workspace: ws_msg.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: "conv-hannah".into(),
            message_id: "pm-hannah-1".into(),
            sender: MessageSender::Patient,
            text: "Hi Dr. Sse, were my kidney and potassium numbers okay?".into(),
            sent_at: reg_time + 20,
        })
        .await
        .unwrap();

    // --- Seed Case 2: Due follow-up review on `ws_fu` ---
    let mut authority = PostgresWorkspaceAuthority;
    let fu_id = FollowUpId::new("fu-david-bmp").unwrap();
    store
        .create_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws_fu,
                session_hash: &dr_hash,
            },
            &FollowUpCommandKey::new("cmd-david-fu").unwrap(),
            &PhysicianCreateFollowUp {
                id: fu_id.clone(),
                summary: FollowUpSummary::new("Verify metabolic panel stability").unwrap(),
                due_at: reg_time + 30,
                records: vec![ExactRecordReference::new("Observation/bmp-1", "rev-1").unwrap()],
            },
            &mut authority,
        )
        .await
        .unwrap();

    // --- Seed Case 3: Combined linked message + changed record on `ws_combo` ---
    let combo_pull = SourcePull {
        source_id: "clinic-ehr".to_owned(),
        source_label: "Clinic EHR".to_owned(),
        patient: source_patient(&ws_combo, "Maya Lin"),
        pulled_at: reg_time + 86_410,
    };
    let combo_lab = chart_record(
        &ws_combo,
        "Observation/ldl-new",
        "rev-2",
        "Lipid Panel",
        "2026-11-12",
        "LDL cholesterol 92 mg/dL (improved from 142 mg/dL on atorvastatin 20 mg).",
    );
    store
        .record_chart(&ws_combo, &[combo_lab], &combo_pull)
        .await
        .unwrap();
    store
        .link_reply_conversation(
            &practice,
            "conv-maya",
            ConversationLink::LinkedPatient {
                workspace: ws_combo.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: "conv-maya".into(),
            message_id: "pm-maya-1".into(),
            sender: MessageSender::Patient,
            text: "Did my cholesterol come down on the atorvastatin?".into(),
            sent_at: reg_time + 86_415,
        })
        .await
        .unwrap();

    // Dispatch all three in one wake pass at `reg_time + 86_420` (where `ws_msg` and `ws_fu` have
    // unchanged charts + event triggers, and `ws_combo` has both a changed chart + linked message):
    let wake = store
        .wake_practice_dispatch(
            auth.as_ref(),
            &practice,
            reg_time + 86_420,
            &checker,
            &receiver,
        )
        .await
        .expect("wake dispatches message, follow-up, and combined reviews");
    assert_eq!(wake.started_reviews.len(), 3);
    let deliveries = receiver.take_deliveries().await;
    assert_eq!(deliveries.len(), 3);

    let (sse_state, mut sse_provider, sse_task) = spawn_local_sse_provider(&store).await;

    // --- Execute Case 1 (`ws_msg` message review) over real HTTP SSE `ResponsesProvider` ---
    let msg_reviews = store
        .patient_reviews(&practice, &ws_msg, auth.as_ref())
        .await
        .unwrap();
    let msg_rev = msg_reviews
        .iter()
        .find(|r| r.state == ReviewLifecycleState::Pending)
        .expect("pending message review on ws_msg");
    assert_eq!(msg_rev.triggers.len(), 1);
    assert!(matches!(msg_rev.triggers[0], ReviewTrigger::Message { .. }));

    let msg_claim = store
        .claim_practice_job(&msg_rev.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .expect("claim message review job");
    assert_eq!(msg_claim.kind, PracticeJobKind::PatientReview);

    sse_state.push_call(
        "call-read-chart-msg",
        READ_TOOL,
        json!({
            "references": [{ "candidate_id": "Observation/bmp-1", "revision": "rev-1", "cursor": null }],
            "max_result_bytes": 8192
        }),
    );
    sse_state.push_call(
        "call-read-conv-msg",
        READ_PATIENT_CONVERSATION_TOOL,
        json!({ "conversation_id": "conv-hannah" }),
    );
    sse_state.push_call(
        "call-propose-reply-msg",
        PATIENT_MESSAGE_REPLY_TOOL,
        json!({
            "decision": "propose_reply",
            "conversation_id": "conv-hannah",
            "draft_text": "Hi Hannah — your kidney function (creatinine 0.9 mg/dL) and potassium (4.1 mmol/L) are both normal.",
            "basis_summary": "Reviewed metabolic panel Observation/bmp-1 rev-1.",
            "limits": ["No external labs connected."],
            "citations": [{ "candidate_id": "Observation/bmp-1", "revision": "rev-1" }],
            "answered_closures": [],
            "raised_concern_ids": []
        }),
    );
    sse_state.push_call(
        "call-complete-msg",
        COMPLETE_PATIENT_REVIEW_TOOL,
        json!({
            "disposition": "assessed",
            "summary": "Drafted reassuring reply to Hannah Park confirming normal potassium and creatinine.",
            "citations": [{ "candidate_id": "Observation/bmp-1", "revision": "rev-1" }],
            "basis_and_limitations": "Clinic EHR metabolic panel Observation/bmp-1 rev-1.",
            "unfinished_work": []
        }),
    );
    sse_state.push_assistant("Completed message review and saved reply draft.");

    let mut msg_host = PracticeJobHost {
        store: store.clone(),
        claim: msg_claim,
        authority: auth.as_ref(),
    };
    msg_host
        .run(
            &mut sse_provider,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .expect("execute message review over SSE");
    let saved_msg_job = store
        .practice_job(&msg_rev.practice_scope())
        .await
        .unwrap()
        .expect("saved message review job");
    assert!(
        saved_msg_job
            .result
            .as_ref()
            .expect("saved message review job result")
            .output_text
            .contains("saved reply draft")
    );

    // Verify reply proposal saved through the reply store strictly as a physician draft!
    let saved_proposals = store
        .reply_proposals(&practice, "conv-hannah")
        .await
        .unwrap();
    assert_eq!(saved_proposals.len(), 1);
    assert_eq!(saved_proposals[0].version, 1);
    assert!(saved_proposals[0].text.contains("creatinine 0.9 mg/dL"));

    let conv_after = store
        .reply_conversation(&practice, "conv-hannah")
        .await
        .unwrap()
        .expect("conv-hannah exists");
    assert!(matches!(
        &conv_after.reply_standing,
        ReplyStanding::Ready { proposal } if proposal.version == 1
    ));
    assert!(
        conv_after.practice_messages.is_empty(),
        "background review must NEVER send an external channel message"
    );

    let surface = store
        .physician_reply_surface(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws_msg,
                session_hash: &dr_hash,
            },
            "conv-hannah",
            &mut authority,
        )
        .await
        .expect("physician_reply_surface reads saved draft");
    let PhysicianReplySurface::OpenCard(card) = surface else {
        panic!("expected OpenCard on physician reply surface, got {surface:?}");
    };
    assert_eq!(card.draft_standing, PhysicianDraftStanding::NoDraft);
    assert!(matches!(
        &card.displayed_text,
        DisplayedReplyText::CodaProposal {
            version: 1,
            text,
            ..
        } if text.contains("potassium (4.1 mmol/L)")
    ));

    let completed_msg_rev = store
        .patient_review(&practice, &ws_msg, &msg_rev.review_id, auth.as_ref())
        .await
        .unwrap()
        .expect("completed message review");
    assert!(matches!(
        completed_msg_rev.state,
        ReviewLifecycleState::Completed {
            outcome: ReviewOutcome::Assessed { .. },
            ..
        }
    ));
    assert!(completed_msg_rev.actions.iter().any(|a| matches!(
        a,
        RecordedReviewAction::ReplyProposed {
            conversation_id,
            version: 1,
            ..
        } if conversation_id == "conv-hannah"
    )));

    // --- Execute Case 2 (`ws_fu` due follow-up review) over real HTTP SSE `ResponsesProvider` ---
    let fu_reviews = store
        .patient_reviews(&practice, &ws_fu, auth.as_ref())
        .await
        .unwrap();
    let fu_rev = fu_reviews
        .iter()
        .find(|r| r.state == ReviewLifecycleState::Pending)
        .expect("pending follow-up review on ws_fu");
    let fu_claim = store
        .claim_practice_job(&fu_rev.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .expect("claim follow-up review job");

    sse_state.push_call(
        "call-read-chart-fu",
        READ_TOOL,
        json!({
            "references": [{ "candidate_id": "Observation/bmp-1", "revision": "rev-1", "cursor": null }],
            "max_result_bytes": 8192
        }),
    );
    sse_state.push_call(
        "call-complete-fu-item",
        FOLLOW_UP_TOOL,
        json!({
            "operation": "complete",
            "follow_up_id": "fu-david-bmp",
            "resolution": "Metabolic panel potassium 4.1 mmol/L and creatinine 0.9 mg/dL confirmed stable."
        }),
    );
    sse_state.push_call(
        "call-complete-fu-rev",
        COMPLETE_PATIENT_REVIEW_TOOL,
        json!({
            "disposition": "quiet",
            "explanation": "Completed due metabolic panel follow-up; values remain normal.",
            "citations": [{ "candidate_id": "Observation/bmp-1", "revision": "rev-1" }],
            "basis_and_limitations": "Clinic EHR Observation/bmp-1 rev-1.",
            "unfinished_work": []
        }),
    );
    sse_state.push_assistant("Completed due follow-up review.");

    let mut fu_host = PracticeJobHost {
        store: store.clone(),
        claim: fu_claim,
        authority: auth.as_ref(),
    };
    fu_host
        .run(
            &mut sse_provider,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .expect("execute follow-up review over SSE");

    let fu_after = store
        .workspace_follow_ups(&practice, &ws_fu)
        .await
        .unwrap()
        .into_iter()
        .find(|f| f.id == fu_id)
        .expect("follow-up exists");
    assert!(matches!(
        fu_after.standing,
        FollowUpStanding::Completed { .. }
    ));
    let completed_fu_rev = store
        .patient_review(&practice, &ws_fu, &fu_rev.review_id, auth.as_ref())
        .await
        .unwrap()
        .expect("completed follow-up review");
    assert!(completed_fu_rev.actions.iter().any(|a| matches!(
        a,
        RecordedReviewAction::FollowUpChanged {
            follow_up_id,
            change_kind,
            ..
        } if follow_up_id == "fu-david-bmp" && change_kind == "completed"
    )));

    // --- Execute Case 3 (`ws_combo` combined message + new result review) ONCE with BOTH inputs ---
    let combo_reviews = store
        .patient_reviews(&practice, &ws_combo, auth.as_ref())
        .await
        .unwrap();
    let combo_pending: Vec<_> = combo_reviews
        .iter()
        .filter(|r| r.state == ReviewLifecycleState::Pending)
        .collect();
    assert_eq!(
        combo_pending.len(),
        1,
        "combined message + result request must produce exactly one pending review"
    );
    let combo_rev = combo_pending[0];
    assert_eq!(combo_rev.triggers.len(), 2);

    let req_count_before_combo = sse_state.requests().len();
    let combo_claim = store
        .claim_practice_job(&combo_rev.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .expect("claim combined review job");

    sse_state.push_call(
        "call-read-combo-chart",
        READ_TOOL,
        json!({
            "references": [
                { "candidate_id": "Observation/bmp-1", "revision": "rev-1", "cursor": null },
                { "candidate_id": "Observation/ldl-new", "revision": "rev-2", "cursor": null }
            ],
            "max_result_bytes": 16384
        }),
    );
    sse_state.push_call(
        "call-read-combo-conv",
        READ_PATIENT_CONVERSATION_TOOL,
        json!({ "conversation_id": "conv-maya" }),
    );
    sse_state.push_call(
        "call-reply-combo",
        PATIENT_MESSAGE_REPLY_TOOL,
        json!({
            "decision": "propose_reply",
            "conversation_id": "conv-maya",
            "draft_text": "Good news Maya — your LDL cholesterol improved from 142 to 92 mg/dL on atorvastatin 20 mg.",
            "basis_summary": "Reviewed new Lipid Panel Observation/ldl-new rev-2.",
            "limits": [],
            "citations": [{ "candidate_id": "Observation/ldl-new", "revision": "rev-2" }],
            "answered_closures": [],
            "raised_concern_ids": []
        }),
    );
    sse_state.push_call(
        "call-complete-combo",
        COMPLETE_PATIENT_REVIEW_TOOL,
        json!({
            "disposition": "assessed",
            "summary": "Reviewed improved LDL 92 mg/dL and drafted reply answering Maya Lin's cholesterol question.",
            "citations": [{ "candidate_id": "Observation/ldl-new", "revision": "rev-2" }],
            "basis_and_limitations": "Clinic EHR Lipid Panel Observation/ldl-new rev-2.",
            "unfinished_work": []
        }),
    );
    sse_state.push_assistant("Completed combined result and message review in one run.");

    let mut combo_host = PracticeJobHost {
        store: store.clone(),
        claim: combo_claim,
        authority: auth.as_ref(),
    };
    combo_host
        .run(
            &mut sse_provider,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .expect("execute combined review once");

    // Inspect the actual wire JSON sent to `/v1/responses` on the first turn of the combined run:
    // it MUST contain BOTH the changed candidate (`Observation/ldl-new`) AND the linked message (`conv-maya` / `pm-maya-1`)!
    let combo_first_wire_req = &sse_state.requests()[req_count_before_combo];
    let combo_wire_str = serde_json::to_string(combo_first_wire_req).unwrap();
    assert!(
        combo_wire_str.contains("Observation/ldl-new") && combo_wire_str.contains("conv-maya"),
        "combined review wire request must include both changed result and linked message inputs: {combo_wire_str}"
    );

    // No further unclaimed jobs remain for `ws_combo`, and a subsequent wake starts 0 reviews.
    assert!(
        store
            .claim_practice_job(&combo_rev.practice_scope(), 60, auth.as_ref())
            .await
            .unwrap()
            .is_none()
    );
    let post_wake = store
        .wake_practice_dispatch(
            auth.as_ref(),
            &practice,
            reg_time + 86_430,
            &checker,
            &receiver,
        )
        .await
        .unwrap();
    assert!(post_wake.started_reviews.is_empty());

    sse_task.abort();
}

#[tokio::test]
async fn stop_holds_follow_up_review_across_restart_and_worker_replay() {
    let Some((store, auth, url, _cluster)) = connect_stores().await else {
        return;
    };
    let skills = load_skills();
    let roles = distinct_roles();
    let reg_time = current_unix_i64();
    let practice = format!("prac-stop-fu-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    let (dr, _token, dr_hash) =
        invite_and_sign_in(&auth, &practice, "DrStopFu", reg_time as u64).await;

    let ws = format!("pw-stop-fu-{}", Uuid::new_v4().simple());
    seed_patient_with_baseline_chart(
        &store,
        &auth,
        BaselineChartSeed {
            practice: &practice,
            workspace: &ws,
            patient_name: "Arthur Pendelton",
            candidate_id: "Observation/inr-1",
            revision: "rev-1",
            text: "INR 2.4 (therapeutic target 2.0-3.0).",
            reg_time,
        },
    )
    .await;
    auth.set_membership(&dr, &ws, Membership::Allowed)
        .await
        .unwrap();

    let checker = StoredObservationChecker::new(store.clone());
    let receiver = PatientReviewReceiver::new(&store, &roles, &skills, auth.as_ref());

    // Baseline registration wake starts zero reviews on unchanged baseline chart.
    let init_wake = store
        .wake_practice_dispatch(auth.as_ref(), &practice, reg_time, &checker, &receiver)
        .await
        .unwrap();
    assert!(init_wake.started_reviews.is_empty());

    // Create due follow-up `fu-inr-check` and dispatch its review.
    let mut authority = PostgresWorkspaceAuthority;
    let fu_id = FollowUpId::new("fu-inr-check").unwrap();
    store
        .create_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws,
                session_hash: &dr_hash,
            },
            &FollowUpCommandKey::new("cmd-create-inr-fu").unwrap(),
            &PhysicianCreateFollowUp {
                id: fu_id.clone(),
                summary: FollowUpSummary::new("Verify repeat INR in 7 days").unwrap(),
                due_at: reg_time + 30,
                records: vec![ExactRecordReference::new("Observation/inr-1", "rev-1").unwrap()],
            },
            &mut authority,
        )
        .await
        .unwrap();
    let created_fu = store
        .workspace_follow_ups(&practice, &ws)
        .await
        .unwrap()
        .into_iter()
        .find(|f| f.id == fu_id)
        .unwrap();
    assert!(matches!(
        created_fu.standing,
        FollowUpStanding::Active {
            schedule: FollowUpReviewSchedule::Scheduled { .. }
        }
    ));

    let fu_wake = store
        .wake_practice_dispatch(auth.as_ref(), &practice, reg_time + 60, &checker, &receiver)
        .await
        .unwrap();
    assert_eq!(fu_wake.started_reviews.len(), 1);
    assert_eq!(fu_wake.started_reviews[0].reason, ReviewReason::FollowUp);
    let _ = receiver.take_deliveries().await;

    let fu_rev = store
        .patient_reviews(&practice, &ws, auth.as_ref())
        .await
        .unwrap()
        .into_iter()
        .find(|r| r.state == ReviewLifecycleState::Pending)
        .expect("pending follow-up review");
    let fu_scope = fu_rev.practice_scope();

    // Claim the follow-up review with a short 1-second lease so an unheld job WOULD become
    // reclaimable after lease expiry, proving `Stop` (not mere lease timing) holds it!
    let stale_worker_claim = store
        .claim_practice_job(&fu_scope, 1, auth.as_ref())
        .await
        .unwrap()
        .expect("worker 1 claims follow-up review");

    let running_rev = store
        .patient_review(&practice, &ws, &fu_rev.review_id, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        running_rev.state,
        ReviewLifecycleState::Running { .. }
    ));
    let running_job = store.practice_job(&fu_scope).await.unwrap().unwrap();
    let expected_ver = patient_review_version(&running_rev, Some(&running_job));

    // Issue version-fenced `Stop` on the running follow-up review!
    let stop_ctl = store
        .control_patient_review_versioned(
            PatientReviewTarget {
                practice: &practice,
                workspace: &ws,
                review_id: &fu_rev.review_id,
            },
            &expected_ver,
            "cmd-stop-fu-review-1",
            VersionedReviewAction::Stop,
            auth.as_ref(),
        )
        .await
        .expect("stop running follow-up review");
    assert_eq!(
        stop_ctl,
        VersionedReviewControl::Accepted { delivery: None }
    );

    // Replaying the exact same `Stop` command key after version advanced is idempotent (`Accepted`).
    let stop_replay = store
        .control_patient_review_versioned(
            PatientReviewTarget {
                practice: &practice,
                workspace: &ws,
                review_id: &fu_rev.review_id,
            },
            &expected_ver,
            "cmd-stop-fu-review-1",
            VersionedReviewAction::Stop,
            auth.as_ref(),
        )
        .await
        .unwrap();
    assert_eq!(
        stop_replay,
        VersionedReviewControl::Accepted { delivery: None }
    );

    // Wait past the 1-second lease window, drop `store`, and reconnect fresh (`restarted_store`)!
    tokio::time::sleep(std::time::Duration::from_millis(1_200)).await;
    drop(store);
    let restarted_store = Store::connect(&url).await.expect("reconnect after restart");

    // 1. Stale in-flight worker 1 from before `Stop` is fenced out and cannot execute/commit!
    struct NeverCalledOrFencedProvider;
    impl engine::recipe::Provider for NeverCalledOrFencedProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            Ok(engine::Turn {
                items: vec![engine::Item::assistant("Should be fenced before commit.")],
                receipt: json!({ "stale": true }),
                usage: Default::default(),
            })
        }
    }
    let mut stale_host = PracticeJobHost {
        store: restarted_store.clone(),
        claim: stale_worker_claim,
        authority: auth.as_ref(),
    };
    assert!(
        stale_host
            .run(
                &mut NeverCalledOrFencedProvider,
                engine::RunLimits::default(),
                &engine::Cancellation::default(),
            )
            .await
            .is_err(),
        "stale worker attempt from before Stop must be fenced out"
    );

    // 2. Across restart + expired lease, worker replay (`claim_practice_job`) returns `None`!
    assert!(
        restarted_store
            .claim_practice_job(&fu_scope, 60, auth.as_ref())
            .await
            .unwrap()
            .is_none(),
        "Stopped follow-up review job must remain Held and unclaimable across restart"
    );

    // 3. Dispatcher replay (`wake_practice_dispatch`) starts ZERO duplicate follow-up reviews!
    let restarted_checker = StoredObservationChecker::new(restarted_store.clone());
    let restarted_receiver =
        PatientReviewReceiver::new(&restarted_store, &roles, &skills, auth.as_ref());
    let wake_while_held = restarted_store
        .wake_practice_dispatch(
            auth.as_ref(),
            &practice,
            reg_time + 300,
            &restarted_checker,
            &restarted_receiver,
        )
        .await
        .unwrap();
    assert!(
        wake_while_held.started_reviews.is_empty(),
        "Stopped follow-up review must remain held across dispatcher wake: {:?}",
        wake_while_held.started_reviews
    );

    let held_rev = restarted_store
        .patient_review(&practice, &ws, &fu_rev.review_id, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        held_rev.state,
        ReviewLifecycleState::Held {
            reason: ReviewHoldReason::Stopped,
            ..
        }
    ));
    let held_job = restarted_store
        .practice_job(&fu_scope)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(held_job.status, WorkStatus::Held);

    // 4. Explicit `Resume` rearms the held follow-up review so it can be claimed and completed cleanly!
    let held_ver = patient_review_version(&held_rev, Some(&held_job));
    let resume_ctl = restarted_store
        .control_patient_review_versioned(
            PatientReviewTarget {
                practice: &practice,
                workspace: &ws,
                review_id: &fu_rev.review_id,
            },
            &held_ver,
            "cmd-resume-fu-review-1",
            VersionedReviewAction::Resume {
                roles: &roles,
                skills: &skills,
            },
            auth.as_ref(),
        )
        .await
        .expect("resume held follow-up review");
    assert!(matches!(
        resume_ctl,
        VersionedReviewControl::Accepted { delivery: Some(_) }
    ));

    let (sse_state, mut sse_provider, sse_task) = spawn_local_sse_provider(&restarted_store).await;
    sse_state.push_call(
        "call-read-inr",
        READ_TOOL,
        json!({
            "references": [{ "candidate_id": "Observation/inr-1", "revision": "rev-1", "cursor": null }],
            "max_result_bytes": 8192
        }),
    );
    sse_state.push_call(
        "call-complete-inr-fu",
        FOLLOW_UP_TOOL,
        json!({
            "operation": "complete",
            "follow_up_id": "fu-inr-check",
            "resolution": "INR 2.4 verified therapeutic."
        }),
    );
    sse_state.push_call(
        "call-complete-resumed-rev",
        COMPLETE_PATIENT_REVIEW_TOOL,
        json!({
            "disposition": "quiet",
            "explanation": "Resumed follow-up review completed INR verification.",
            "citations": [{ "candidate_id": "Observation/inr-1", "revision": "rev-1" }],
            "basis_and_limitations": "Clinic EHR Observation/inr-1 rev-1.",
            "unfinished_work": []
        }),
    );
    sse_state.push_assistant("Resumed follow-up review finished.");

    let resumed_claim = restarted_store
        .claim_practice_job(&fu_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .expect("resumed follow-up review is claimable");
    let mut resumed_host = PracticeJobHost {
        store: restarted_store.clone(),
        claim: resumed_claim,
        authority: auth.as_ref(),
    };
    resumed_host
        .run(
            &mut sse_provider,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .expect("run resumed follow-up review");

    let final_rev = restarted_store
        .patient_review(&practice, &ws, &fu_rev.review_id, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        final_rev.state,
        ReviewLifecycleState::Completed { .. }
    ));
    sse_task.abort();
}

#[tokio::test]
async fn direct_spruce_receiver_handoff_admits_message_review_without_duplicate_wake_and_classifies_senders()
 {
    let Some((store, auth, _url, _cluster)) = connect_stores().await else {
        return;
    };
    let skills = load_skills();
    let roles = distinct_roles();
    let reg_time = current_unix_i64();
    let practice = format!("prac-direct-spruce-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    let (dr, _token, dr_hash) =
        invite_and_sign_in(&auth, &practice, "DrDirectSpruce", reg_time as u64).await;

    let ws_patient_recv = format!("pw-spruce-ptrecv-{}", Uuid::new_v4().simple());
    let ws_job_recv = format!("pw-spruce-jobrecv-{}", Uuid::new_v4().simple());
    for (ws, name) in [
        (&ws_patient_recv, "Liam Brooks"),
        (&ws_job_recv, "Maya Lin"),
    ] {
        seed_patient_with_baseline_chart(
            &store,
            &auth,
            BaselineChartSeed {
                practice: &practice,
                workspace: ws,
                patient_name: name,
                candidate_id: "Observation/bmp-direct",
                revision: "rev-1",
                text: "Serum potassium 4.1 mmol/L.",
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
    let init_wake = store
        .wake_practice_dispatch(auth.as_ref(), &practice, reg_time, &checker, &receiver)
        .await
        .unwrap();
    assert!(init_wake.started_reviews.is_empty());

    store
        .connect_spruce_channel(auth.as_ref(), &dr, 0, None, reg_time)
        .await
        .unwrap();
    let lease = store
        .acquire_spruce_reader_lease(&practice, "worker-direct-spruce", 300, reg_time)
        .await
        .unwrap();

    let liam_phone = SenderAddress::parse("+14155550791").unwrap();
    let maya_phone = SenderAddress::parse("+14155550792").unwrap();
    for (phone, ws) in [(&liam_phone, &ws_patient_recv), (&maya_phone, &ws_job_recv)] {
        store
            .record_spruce_confirmed_sender(&practice, phone, ws, &dr.physician_id, reg_time)
            .await
            .unwrap();
    }

    // 1. Direct `PatientReviewReceiver` handoff with patient-authored `author_name: Some("Liam Brooks")`.
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &IncomingSpruceItem::Readable(ReadableSpruceItem {
                message_id: "spruce-direct-liam-1".into(),
                conversation_id: "conv-direct-liam".into(),
                channel_position: 1,
                content_version: "v1".into(),
                sender_address: Some(liam_phone.clone()),
                author_name: Some("Liam Brooks".into()),
                body_text: "Should I take my morning dose before lab work?".into(),
                attachments: Vec::new(),
                app_url: Some("https://app.sprucehealth.com/conversations/conv-direct-liam".into()),
                created_at_spruce: Some("2026-11-12T16:00:00Z".into()),
                outside_participants: vec![OutsideParticipant {
                    display_name: Some("Liam Brooks".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(liam_phone.clone()),
                    saved_contact_id: None,
                }],
            }),
            reg_time + 10,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "spruce-direct-liam-1",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::SuggestedPatient {
                    workspace: ws_patient_recv.clone(),
                    evidence_summary: "Confirmed phone for Liam Brooks.".into(),
                },
                rationale: "Patient medication timing question.".into(),
            },
            Some(&receiver),
            reg_time + 11,
        )
        .await
        .expect("direct spruce handoff to PatientReviewReceiver");

    // Verify admitted review is `ReviewTrigger::Message` (not `"new records"`) and preloads `patient-communication`.
    let liam_reviews = store
        .patient_reviews(&practice, &ws_patient_recv, auth.as_ref())
        .await
        .unwrap();
    assert_eq!(liam_reviews.len(), 1);
    assert!(matches!(
        liam_reviews[0].triggers.as_slice(),
        [ReviewTrigger::Message {
            conversation_id,
            message_ids,
            ..
        }] if conversation_id == "conv-direct-liam"
            && message_ids == &vec!["spruce-direct-liam-1".to_owned()]
    ));
    let skill_names: BTreeSet<&str> = liam_reviews[0]
        .recipe_version
        .preloaded_skills
        .iter()
        .map(|s| s.name.as_str())
        .collect();
    assert!(skill_names.contains("patient-communication"));

    // Verify patient-authored message mapped to `MessageSender::Patient` (not third-party `NamedSender`).
    let mut authority = PostgresWorkspaceAuthority;
    let surface_patient = store
        .physician_reply_surface(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws_patient_recv,
                session_hash: &dr_hash,
            },
            "conv-direct-liam",
            &mut authority,
        )
        .await
        .unwrap();
    assert!(matches!(
        &surface_patient,
        PhysicianReplySurface::NoOpenCard {
            open_messages,
            non_patient_sender_names,
            ..
        } if open_messages.len() == 1
            && open_messages[0].sender == MessageSender::Patient
            && non_patient_sender_names.is_empty()
    ));

    // Ingest a caregiver message (`"Dana Brooks"`) on the same conversation and verify `NamedSender`.
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &IncomingSpruceItem::Readable(ReadableSpruceItem {
                message_id: "spruce-direct-dana-2".into(),
                conversation_id: "conv-direct-liam".into(),
                channel_position: 2,
                content_version: "v1".into(),
                sender_address: Some(liam_phone.clone()),
                author_name: Some("Dana Brooks".into()),
                body_text: "This is Liam's wife Dana — he also had mild dizziness.".into(),
                attachments: Vec::new(),
                app_url: None,
                created_at_spruce: None,
                outside_participants: vec![OutsideParticipant {
                    display_name: Some("Dana Brooks".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(liam_phone),
                    saved_contact_id: None,
                }],
            }),
            reg_time + 15,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "spruce-direct-dana-2",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::SuggestedPatient {
                    workspace: ws_patient_recv.clone(),
                    evidence_summary: "Confirmed family thread for Liam Brooks.".into(),
                },
                rationale: "Spouse symptom report.".into(),
            },
            Some(&receiver),
            reg_time + 16,
        )
        .await
        .unwrap();
    let surface_caregiver = store
        .physician_reply_surface(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws_patient_recv,
                session_hash: &dr_hash,
            },
            "conv-direct-liam",
            &mut authority,
        )
        .await
        .unwrap();
    assert!(matches!(
        &surface_caregiver,
        PhysicianReplySurface::NoOpenCard {
            non_patient_sender_names,
            ..
        } if non_patient_sender_names == &vec!["Dana Brooks".to_owned()]
    ));

    // 2. Direct `PracticeJobReviewReceiver` handoff on `ws_job_recv`.
    let job_receiver =
        PracticeJobReviewReceiver::new(store.clone(), &roles, &skills, auth.as_ref());
    store
        .ingest_spruce_item(
            &practice,
            &lease.lease_token,
            &IncomingSpruceItem::Readable(ReadableSpruceItem {
                message_id: "spruce-direct-maya-1".into(),
                conversation_id: "conv-direct-maya".into(),
                channel_position: 3,
                content_version: "v1".into(),
                sender_address: Some(maya_phone.clone()),
                author_name: Some("Maya Lin".into()),
                body_text: "Checking in on my blood pressure readings.".into(),
                attachments: Vec::new(),
                app_url: None,
                created_at_spruce: None,
                outside_participants: vec![OutsideParticipant {
                    display_name: Some("Maya Lin".into()),
                    endpoint_channel: Some("sms".into()),
                    endpoint_address: Some(maya_phone),
                    saved_contact_id: None,
                }],
            }),
            reg_time + 20,
        )
        .await
        .unwrap();
    store
        .record_spruce_message_classification(
            auth.as_ref(),
            &practice,
            "spruce-direct-maya-1",
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::SuggestedPatient {
                    workspace: ws_job_recv.clone(),
                    evidence_summary: "Confirmed phone for Maya Lin.".into(),
                },
                rationale: "Patient BP check-in.".into(),
            },
            Some(&job_receiver),
            reg_time + 21,
        )
        .await
        .unwrap();

    let maya_triggers = store.spruce_review_triggers(&practice).await.unwrap();
    let maya_trig = maya_triggers
        .iter()
        .find(|t| t.workspace == ws_job_recv)
        .expect("spruce trigger for maya");
    assert!(maya_trig.dispatched);
    let maya_job = store
        .practice_job(&local_product::runtime::jobs::PracticeScope {
            practice: practice.clone(),
            workspace: ws_job_recv.clone(),
            job_id: maya_trig.command_key.clone(),
        })
        .await
        .unwrap()
        .expect("practice job admitted by PracticeJobReviewReceiver");
    assert!(
        maya_job
            .assignment
            .prompt
            .contains("Conduct a message chart review"),
        "expected message reason prompt, got: {}",
        maya_job.assignment.prompt
    );
    assert!(
        maya_job
            .assignment
            .preloaded_skills
            .iter()
            .any(|s| s.name == "patient-communication"),
        "expected patient-communication skill on PracticeJobReviewReceiver job"
    );

    // 3. Subsequent `wake_practice_dispatch` starts ZERO duplicate reviews for either patient!
    let follow_wake = store
        .wake_practice_dispatch(auth.as_ref(), &practice, reg_time + 30, &checker, &receiver)
        .await
        .unwrap();
    assert!(
        follow_wake.started_reviews.is_empty(),
        "direct Spruce receiver handoff must not start duplicate reviews on wake: {:?}",
        follow_wake.started_reviews
    );
}

#[tokio::test]
async fn reply_review_fence_releases_on_quiet_completion_and_preserves_non_patient_wakes_on_supersession()
 {
    let Some((store, auth, _url, _cluster)) = connect_stores().await else {
        return;
    };
    let skills = load_skills();
    let roles = distinct_roles();
    let reg_time = current_unix_i64();
    let practice = format!("prac-fence-rel-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    let (dr, _token, dr_hash) =
        invite_and_sign_in(&auth, &practice, "DrFenceRel", reg_time as u64).await;

    let ws = format!("pw-fence-rel-{}", Uuid::new_v4().simple());
    seed_patient_with_baseline_chart(
        &store,
        &auth,
        BaselineChartSeed {
            practice: &practice,
            workspace: &ws,
            patient_name: "Hannah Abbott",
            candidate_id: "Observation/bmp-fence",
            revision: "rev-1",
            text: "Serum sodium 139 mmol/L, potassium 4.0 mmol/L.",
            reg_time,
        },
    )
    .await;
    auth.set_membership(&dr, &ws, Membership::Allowed)
        .await
        .unwrap();

    let checker = StoredObservationChecker::new(store.clone());
    let receiver = PatientReviewReceiver::new(&store, &roles, &skills, auth.as_ref());
    let _ = store
        .wake_practice_dispatch(auth.as_ref(), &practice, reg_time, &checker, &receiver)
        .await
        .unwrap();

    let conv_id = "conv-fence-quiet";
    store
        .link_reply_conversation(
            &practice,
            conv_id,
            ConversationLink::LinkedPatient {
                workspace: ws.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            message_id: "msg-fence-1".into(),
            sender: MessageSender::Patient,
            text: "Just confirming I picked up my prescription, thank you!".into(),
            sent_at: reg_time + 10,
        })
        .await
        .unwrap();

    // 1. Dispatch & run `rev1` where the model calls `read_patient_conversation` and finishes
    //    `complete_patient_review` WITHOUT calling `patient_message_reply`.
    let wake1 = store
        .wake_practice_dispatch(auth.as_ref(), &practice, reg_time + 15, &checker, &receiver)
        .await
        .unwrap();
    assert_eq!(wake1.started_reviews.len(), 1);
    let _ = receiver.take_deliveries().await;

    let rev1 = store
        .patient_reviews(&practice, &ws, auth.as_ref())
        .await
        .unwrap()
        .into_iter()
        .find(|r| r.state == ReviewLifecycleState::Pending)
        .expect("rev1 pending");

    let (sse_state, mut sse_provider, sse_task) = spawn_local_sse_provider(&store).await;
    sse_state.push_call(
        "call-read-conv-1",
        READ_PATIENT_CONVERSATION_TOOL,
        json!({ "conversation_id": conv_id }),
    );
    sse_state.push_call(
        "call-read-bmp-fence-1",
        READ_TOOL,
        json!({
            "references": [{ "candidate_id": "Observation/bmp-fence", "revision": "rev-1", "cursor": null }],
            "max_result_bytes": 8192
        }),
    );
    sse_state.push_call(
        "call-complete-quiet-1",
        COMPLETE_PATIENT_REVIEW_TOOL,
        json!({
            "disposition": "quiet",
            "explanation": "Reviewed conversation context without drafting a reply.",
            "citations": [{ "candidate_id": "Observation/bmp-fence", "revision": "rev-1" }],
            "basis_and_limitations": "Clinic EHR Observation/bmp-fence rev-1.",
            "unfinished_work": []
        }),
    );
    sse_state.push_assistant("Finished quiet review.");

    let claim1 = store
        .claim_practice_job(&rev1.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    PracticeJobHost {
        store: store.clone(),
        claim: claim1,
        authority: auth.as_ref(),
    }
    .run(
        &mut sse_provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("run rev1 without patient_message_reply");

    // Verify `active_review` was cleanly released on `Completed` (`review_state == Idle`, NOT stuck `Reviewing`!).
    let conv_after_rev1 = store
        .reply_conversation(&practice, conv_id)
        .await
        .unwrap()
        .unwrap();
    assert!(
        conv_after_rev1.active_review.is_none(),
        "completed review without patient_message_reply must release active_review: {:?}",
        conv_after_rev1.active_review
    );
    let mut authority = PostgresWorkspaceAuthority;
    let surface_after_rev1 = store
        .physician_reply_surface(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws,
                session_hash: &dr_hash,
            },
            conv_id,
            &mut authority,
        )
        .await
        .unwrap();
    assert!(matches!(
        surface_after_rev1,
        PhysicianReplySurface::NoOpenCard {
            review_state: CardReviewState::Idle,
            ..
        }
    ));

    // Verify a subsequent message arrival + review (`rev2`) can acquire the conversation fence
    // without `StaleReview` (previously blocked forever by leaked `active_review`) and mark `NoReplyWouldHelp`.
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            message_id: "msg-fence-2".into(),
            sender: MessageSender::Patient,
            text: "No questions on my end — have a good weekend!".into(),
            sent_at: reg_time + 20,
        })
        .await
        .unwrap();
    let snap2 = store
        .start_or_read_reply_review(&practice, conv_id, "rev-manual-2", "run-manual-2")
        .await
        .expect("subsequent review acquires released conversation fence cleanly");
    assert_eq!(snap2.review.review_id, "rev-manual-2");
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            review_id: "rev-manual-2".into(),
            run_id: "run-manual-2".into(),
            completion_key: "comp-manual-2".into(),
            answered_closures: Vec::new(),
            decision: ReplyReviewDecision::NoReplyWouldHelp {
                reason: "Patient courtesy confirmation requires no reply.".into(),
            },
            raised_concern_ids: Vec::new(),
        })
        .await
        .unwrap();

    // 2. Now all patient messages on `conv-fence-quiet` are closed (`NeedsNoReply`).
    //    Observe a practice message (`PracticeMessage` wake ONLY — zero open patient messages!).
    store
        .observe_practice_message(&ObservePracticeMessage {
            practice: practice.clone(),
            conversation_id: conv_id.into(),
            practice_message_id: "pm-staff-ack-1".into(),
            kind: PracticeMessageKind::PracticeSent,
            text: "Glad you got it, Hannah!".into(),
            sent_at: reg_time + 25,
        })
        .await
        .unwrap();

    // Dispatch `rev3` for that `PracticeMessage` wake and have `rev3` call `start_or_read_reply_review`
    // (draining `woke_events` into `active_review.consumed_wakes`).
    let wake3 = store
        .wake_practice_dispatch(auth.as_ref(), &practice, reg_time + 30, &checker, &receiver)
        .await
        .unwrap();
    assert_eq!(wake3.started_reviews.len(), 1);
    let _ = receiver.take_deliveries().await;
    let rev3 = store
        .patient_reviews(&practice, &ws, auth.as_ref())
        .await
        .unwrap()
        .into_iter()
        .find(|r| r.state == ReviewLifecycleState::Pending)
        .expect("rev3 pending");
    let claim3 = store
        .claim_practice_job(&rev3.practice_scope(), 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    let _ = store
        .start_or_read_reply_review(&practice, conv_id, &rev3.review_id, &claim3.attempt)
        .await
        .expect("rev3 drains PracticeMessage wake");

    // Supersede `rev3` via a physician-requested review (`rev4`) before `rev3` finishes!
    let _ = store
        .submit_physician_patient_review(
            auth.as_ref(),
            &dr,
            &ws,
            "cmd-phys-supersede-rev3",
            "Re-evaluate conversation and latest metabolic panel.",
            &roles,
            &skills,
        )
        .await
        .expect("admit rev4 superseding rev3");

    let conv_after_supersede = store
        .reply_conversation(&practice, conv_id)
        .await
        .unwrap()
        .unwrap();
    assert!(
        conv_after_supersede.active_review.is_none(),
        "superseded review must release active_review"
    );
    assert!(
        conv_after_supersede
            .woke_events
            .contains(&ReviewWakeEvent::PracticeMessage {
                practice_message_id: "pm-staff-ack-1".into(),
            }),
        "aborted/superseded review must restore consumed PracticeMessage wake: {:?}",
        conv_after_supersede.woke_events
    );

    // Replacement review (`rev4`) can immediately read `conv-fence-quiet` without `NoPendingReviewWake`!
    let snap4 = store
        .start_or_read_reply_review(&practice, conv_id, "rev-replacement-4", "run-replacement-4")
        .await
        .expect(
            "replacement review reads restored PracticeMessage wake without NoPendingReviewWake",
        );
    assert_eq!(snap4.review.review_id, "rev-replacement-4");

    sse_task.abort();
}

#[tokio::test]
async fn stopped_follow_up_review_remains_held_when_later_patient_inputs_admit_new_review() {
    let Some((store, auth, _url, _cluster)) = connect_stores().await else {
        return;
    };
    let skills = load_skills();
    let roles = distinct_roles();
    let reg_time = current_unix_i64();
    let practice = format!("prac-held-keep-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::RealTime)
        .await
        .unwrap();
    let (dr, _token, dr_hash) =
        invite_and_sign_in(&auth, &practice, "DrHeldKeep", reg_time as u64).await;

    let ws = format!("pw-held-keep-{}", Uuid::new_v4().simple());
    seed_patient_with_baseline_chart(
        &store,
        &auth,
        BaselineChartSeed {
            practice: &practice,
            workspace: &ws,
            patient_name: "Neville Longbottom",
            candidate_id: "Observation/inr-base",
            revision: "rev-1",
            text: "INR 2.3 (therapeutic).",
            reg_time,
        },
    )
    .await;
    auth.set_membership(&dr, &ws, Membership::Allowed)
        .await
        .unwrap();

    let checker = StoredObservationChecker::new(store.clone());
    let receiver = PatientReviewReceiver::new(&store, &roles, &skills, auth.as_ref());
    let _ = store
        .wake_practice_dispatch(auth.as_ref(), &practice, reg_time, &checker, &receiver)
        .await
        .unwrap();

    // 1. Create due follow-up, dispatch its review (`fu_rev`), and `Stop` it (`Held { reason: Stopped }`).
    let mut authority = PostgresWorkspaceAuthority;
    let fu_id = FollowUpId::new("fu-neville-inr").unwrap();
    store
        .create_follow_up_for_physician(
            &WorkspaceAccess {
                practice: &dr.practice_id,
                physician: &dr.physician_id,
                workspace: &ws,
                session_hash: &dr_hash,
            },
            &FollowUpCommandKey::new("cmd-create-neville-fu").unwrap(),
            &PhysicianCreateFollowUp {
                id: fu_id.clone(),
                summary: FollowUpSummary::new("Recheck INR after dietary change").unwrap(),
                due_at: reg_time + 20,
                records: vec![ExactRecordReference::new("Observation/inr-base", "rev-1").unwrap()],
            },
            &mut authority,
        )
        .await
        .unwrap();

    let wake_fu = store
        .wake_practice_dispatch(auth.as_ref(), &practice, reg_time + 30, &checker, &receiver)
        .await
        .unwrap();
    assert_eq!(wake_fu.started_reviews.len(), 1);
    let _ = receiver.take_deliveries().await;

    let fu_rev = store
        .patient_reviews(&practice, &ws, auth.as_ref())
        .await
        .unwrap()
        .into_iter()
        .find(|r| r.state == ReviewLifecycleState::Pending)
        .expect("pending follow-up review");
    let fu_scope = fu_rev.practice_scope();
    let fu_job = store.practice_job(&fu_scope).await.unwrap().unwrap();
    let fu_ver = patient_review_version(&fu_rev, Some(&fu_job));

    let stop_res = store
        .control_patient_review_versioned(
            PatientReviewTarget {
                practice: &practice,
                workspace: &ws,
                review_id: &fu_rev.review_id,
            },
            &fu_ver,
            "cmd-stop-neville-fu",
            VersionedReviewAction::Stop,
            auth.as_ref(),
        )
        .await
        .expect("stop follow-up review");
    assert_eq!(
        stop_res,
        VersionedReviewControl::Accepted { delivery: None }
    );

    // 2. While `fu_rev` is Stopped (`Held`), a new linked message AND a new chart record arrive!
    let new_pull = SourcePull {
        source_id: "clinic-ehr".to_owned(),
        source_label: "Clinic EHR".to_owned(),
        patient: source_patient(&ws, "Neville Longbottom"),
        pulled_at: reg_time + 86_450,
    };
    let new_rec = chart_record(
        &ws,
        "Observation/bmp-neville-2",
        "rev-1",
        "Basic Metabolic Panel",
        "2026-11-12",
        "Serum potassium 4.3 mmol/L, creatinine 0.9 mg/dL.",
    );
    store
        .record_chart(&ws, &[new_rec], &new_pull)
        .await
        .unwrap();
    store
        .link_reply_conversation(
            &practice,
            "conv-neville-1",
            ConversationLink::LinkedPatient {
                workspace: ws.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: "conv-neville-1".into(),
            message_id: "msg-neville-1".into(),
            sender: MessageSender::Patient,
            text: "Did my basic metabolic panel look okay?".into(),
            sent_at: reg_time + 86_460,
        })
        .await
        .unwrap();

    // Wake dispatcher at `reg_time + 86_470`: admits a new review for the new message + record change.
    let wake_new = store
        .wake_practice_dispatch(
            auth.as_ref(),
            &practice,
            reg_time + 86_470,
            &checker,
            &receiver,
        )
        .await
        .unwrap();
    assert_eq!(wake_new.started_reviews.len(), 1);
    let _ = receiver.take_deliveries().await;

    // 3. Verify `fu_rev` was NOT overwritten to `Superseded` — it remains `Held { reason: Stopped }`!
    let fu_rev_after_new_input = store
        .patient_review(&practice, &ws, &fu_rev.review_id, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    assert!(
        matches!(
            fu_rev_after_new_input.state,
            ReviewLifecycleState::Held {
                reason: ReviewHoldReason::Stopped,
                ..
            }
        ),
        "Stopped follow-up review must remain Held across later patient inputs, got: {:?}",
        fu_rev_after_new_input.state
    );
    let fu_job_after = store.practice_job(&fu_scope).await.unwrap().unwrap();
    assert_eq!(fu_job_after.status, WorkStatus::Held);

    // 4. Verify `Resume` on `fu_rev` still works and completes the follow-up!
    let held_ver = patient_review_version(&fu_rev_after_new_input, Some(&fu_job_after));
    let resume_res = store
        .control_patient_review_versioned(
            PatientReviewTarget {
                practice: &practice,
                workspace: &ws,
                review_id: &fu_rev.review_id,
            },
            &held_ver,
            "cmd-resume-neville-fu",
            VersionedReviewAction::Resume {
                roles: &roles,
                skills: &skills,
            },
            auth.as_ref(),
        )
        .await
        .expect("resume stopped follow-up review after later patient inputs");
    assert!(matches!(
        resume_res,
        VersionedReviewControl::Accepted { delivery: Some(_) }
    ));

    let resumed_rev = store
        .patient_review(&practice, &ws, &fu_rev.review_id, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(resumed_rev.state, ReviewLifecycleState::Pending);
    assert!(matches!(
        resumed_rev.triggers.as_slice(),
        [ReviewTrigger::FollowUp { follow_up_id, .. }] if follow_up_id == "fu-neville-inr"
    ));
}
