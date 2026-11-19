//! Real PostgreSQL integration suite for wrong-patient conversation link correction,
//! patient access enforcement, immediate cross-surface withholding, `Undo` window
//! semantics, unfinished-removal fencing and recovery, and post-`Undo` settlement
//! across memory, patient understanding, concerns, follow-ups, reply proposals,
//! private reply drafts, and once-per-affected-patient review requests.

#![expect(
    dead_code,
    reason = "ac3..ac5, round1, and Today/Updates withholding tests land in companion slices"
)]
#![expect(
    unused_imports,
    reason = "settlement and Today/Updates fence imports are exercised in companion slices"
)]

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::error::Error as _;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex, Weak};

use local_product::attachments::READ_DOCUMENT_PAGE_TOOL;
use local_product::initial_understanding::{
    HealthSystemStanding, IndexedAnchor, KnownHealthSystem, OperatingIndex,
    SAVE_UNDERSTANDING_TOOL, SaveUnderstandingArgs, SummaryClaimInput,
};
use local_product::patient_reviews::{
    COMPLETE_PATIENT_REVIEW_TOOL, CompleteReviewArgs, CompleteReviewDisposition, ReviewTrigger,
};
use local_product::physician_chat::{ChatAssignment, PhysicianSkills};
use local_product::physician_sources::{
    MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SEARCH_TOOL, SourcePatient,
    SourceRevision, read_tool,
};
use local_product::runtime::concerns::{
    CommandKey, ConcernId, ConcernStanding, ConcernUrgency, ExactRecordReference, RaiseConcern,
    SourceWork, WorkspaceAccess,
};
use local_product::runtime::dispatch::ReviewReceiver;
use local_product::runtime::follow_ups::{
    FOLLOW_UP_TOOL, FollowUpCommandKey, FollowUpError, FollowUpId, FollowUpReviewOutcome,
    FollowUpStanding, FollowUpSummary, PhysicianCreateFollowUp, WithdrawalFenceViolation,
    follow_up_tool, is_follow_up_tool,
};
use local_product::runtime::history::HistoryWorkTarget;
use local_product::runtime::jobs::{PracticeAccessError, PracticeJobError, PracticeJobHost};
use local_product::runtime::link_correction::{
    CorrectPatientLinkCommand, CorrectionDestination, LinkConversationCommand, LinkCorrectionError,
    LinkCorrectionOutcome, LinkCorrectionRemovalProbe, LinkCorrectionStanding, LinkCorrectionStep,
    UndoLinkCorrectionCommand, WithheldStanding,
};
use local_product::runtime::memory::{NoteDraft, NoteOrigin, PatientKey};
use local_product::runtime::reply::proposals::{
    AnsweredMessageClosure, CompleteReplyReview, ConversationLink, DisplayedReplyText,
    ExpectedDraftBase, MessageIntakeOutcome, MessageSender, ObservePracticeMessage,
    PatientMessageStanding, PhysicianDraftStanding, PracticeMessageKind, RecordPatientMessage,
    ReplyProposalBasisInput, ReplyProposalError, ReplyProposalInput, ReplyReviewDecision,
    ReplyReviewOutcome, ReplyStanding, proposal_draft_scope,
};
use local_product::runtime::reviews::AdmittedPatientReview;
use local_product::runtime::sources::SourcePull;
use local_product::runtime::spruce_matching::{
    ConversationLinkState, IncomingSpruceItem, MatchAssessment, MatchingDecision,
    MessageRoutingOutcome, MessageStanding, MessageUrgencyChoice, OutsideParticipant,
    ReadableSpruceItem, SenderAddress, SenderConfirmationPolicy,
};
use local_product::runtime::today_updates::{OpenItemRequest, SeenItemKey, TodayUpdatesError};
use local_product::runtime::understanding::{UnderstandingStanding, UnderstandingTriggerOutcome};
use local_product::runtime::{
    Accepted, Authority, Input, MessageRole, PhysicianHost, PublicationTransaction, Scope, Store,
    async_trait,
    engine::{
        self, AcceptedInput, Event, Invocation, Item, Product, ProductMessage, ToolOutcome, Turn,
        WindowSeed,
        recipe::{Finish, Observer, Provider, Request, RequestBinding, RequestRecipe},
    },
};
use local_product::spruce_matching_recipe::{GUIDANCE_VERSION, RECIPE_VERSION};
use local_product::{ModelBinding, ModelRoles, sha256_hex};
use patient_work::{ReviewReason, ReviewRequest};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use sea_orm::ConnectionTrait;
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
        let _ = fs::remove_dir_all(&self.cluster_dir);
    }
}

static LOCAL_CLUSTER: Mutex<Weak<LocalCluster>> = Mutex::new(Weak::new());

fn free_loopback_port() -> u16 {
    use std::net::TcpListener;
    for _ in 0..500 {
        let port = 24_000 + (Uuid::new_v4().as_u128() % 7_000) as u16;
        if TcpListener::bind(("127.0.0.1", port)).is_ok() {
            return port;
        }
    }
    panic!("no free loopback port for test postgres cluster");
}

fn ensure_admin_postgres_url() -> (String, Option<Arc<LocalCluster>>) {
    for key in [
        "PHYSICIAN_RUNTIME_TEST_DATABASE_URL",
        "PHYSICIAN_DATABASE_URL",
        "PHYSICIAN_TEST_DATABASE_URL",
    ] {
        if let Ok(val) = env::var(key) {
            let trimmed = val.trim();
            if !trimmed.is_empty() {
                let (base, _) = trimmed
                    .rsplit_once('/')
                    .expect("database URL must name a database");
                return (format!("{base}/postgres"), None);
            }
        }
    }

    let mut slot = LOCAL_CLUSTER
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(existing) = slot.upgrade() {
        return (existing.admin_url.clone(), Some(existing));
    }

    let port = free_loopback_port();
    let cluster_dir = env::temp_dir().join(format!(
        "coda-link-correction-pg-{}-{}",
        std::process::id(),
        port
    ));
    let _ = fs::remove_dir_all(&cluster_dir);
    let data_dir = cluster_dir.join("data");
    let sock_dir = cluster_dir.join("sock");
    fs::create_dir_all(&data_dir).expect("create temp postgres data dir");
    fs::create_dir_all(&sock_dir).expect("create temp postgres sock dir");

    let init = Command::new("initdb")
        .args([
            "-D",
            data_dir.to_str().unwrap(),
            "-U",
            "postgres",
            "-A",
            "trust",
            "--no-locale",
            "-E",
            "UTF8",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .expect("run initdb");
    assert!(
        init.status.success(),
        "initdb failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );

    let log_file = cluster_dir.join("postgres.log");
    let opts = format!(
        "-F -p {port} -h 127.0.0.1 -k {}",
        sock_dir.to_str().unwrap()
    );
    let start = Command::new("pg_ctl")
        .args([
            "-D",
            data_dir.to_str().unwrap(),
            "-l",
            log_file.to_str().unwrap(),
            "-o",
            &opts,
            "-w",
            "start",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .expect("run pg_ctl start");
    assert!(
        start.status.success(),
        "pg_ctl start failed: {}\nlog:\n{}",
        String::from_utf8_lossy(&start.stderr),
        fs::read_to_string(&log_file).unwrap_or_default()
    );

    let cluster = Arc::new(LocalCluster {
        cluster_dir,
        data_dir,
        admin_url: format!("postgresql://postgres@127.0.0.1:{port}/postgres"),
    });
    *slot = Arc::downgrade(&cluster);
    (cluster.admin_url.clone(), Some(cluster))
}

struct DisposableDb {
    admin_url: String,
    db_name: String,
    url: String,
    _cluster: Option<Arc<LocalCluster>>,
}

impl DisposableDb {
    async fn create(label: &str) -> Self {
        let (admin_url, cluster) = ensure_admin_postgres_url();
        let (base, _) = admin_url.rsplit_once('/').expect("admin url base");
        let db_name = format!("link_corr_{label}_{}", Uuid::new_v4().simple());
        let url = format!("{base}/{db_name}");
        let admin = sea_orm::Database::connect(&admin_url)
            .await
            .expect("connect admin db");
        admin
            .execute_unprepared(&format!("CREATE DATABASE {db_name}"))
            .await
            .expect("create disposable db");
        let _ = admin.close().await;
        Self {
            admin_url,
            url,
            db_name,
            _cluster: cluster,
        }
    }

    async fn cleanup(self) {
        if let Ok(admin) = sea_orm::Database::connect(&self.admin_url).await {
            let _ = admin
                .execute_unprepared(&format!(
                    "DROP DATABASE IF EXISTS {} WITH (FORCE)",
                    self.db_name
                ))
                .await;
            let _ = admin.close().await;
        }
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
    accepted_by_key: Mutex<BTreeMap<(String, String), ReviewRequest>>,
    all_calls: Mutex<Vec<ReviewRequest>>,
    failing_workspaces: Mutex<BTreeSet<String>>,
}

impl RecordingReviewReceiver {
    fn fail_workspace(&self, workspace: &str) {
        self.failing_workspaces
            .lock()
            .unwrap()
            .insert(workspace.to_owned());
    }

    fn clear_failures(&self) {
        self.failing_workspaces.lock().unwrap().clear();
    }

    fn accepted_requests(&self) -> Vec<ReviewRequest> {
        self.accepted_by_key
            .lock()
            .unwrap()
            .values()
            .cloned()
            .collect()
    }

    fn accepted_for_workspace(&self, workspace: &str) -> Vec<ReviewRequest> {
        self.accepted_by_key
            .lock()
            .unwrap()
            .values()
            .filter(|r| r.patient_workspace_id == workspace)
            .cloned()
            .collect()
    }
}

#[async_trait]
impl ReviewReceiver for RecordingReviewReceiver {
    async fn submit_review(&self, request: &ReviewRequest) -> Result<(), PracticeJobError> {
        self.all_calls.lock().unwrap().push(request.clone());
        if self
            .failing_workspaces
            .lock()
            .unwrap()
            .contains(&request.patient_workspace_id)
        {
            return Err(PracticeJobError::Access(
                PracticeAccessError::UnregisteredPatient {
                    practice: request.practice_id.clone(),
                    workspace: request.patient_workspace_id.clone(),
                },
            ));
        }
        self.accepted_by_key
            .lock()
            .unwrap()
            .entry((request.practice_id.clone(), request.command_key.clone()))
            .or_insert_with(|| request.clone());
        Ok(())
    }
}

struct StepFailureProbe {
    fail_step: Mutex<Option<(LinkCorrectionStep, String)>>,
}

impl StepFailureProbe {
    fn new(step: LinkCorrectionStep, reason: impl Into<String>) -> Self {
        Self {
            fail_step: Mutex::new(Some((step, reason.into()))),
        }
    }

    fn clear(&self) {
        *self.fail_step.lock().unwrap() = None;
    }
}

impl LinkCorrectionRemovalProbe for StepFailureProbe {
    fn before_step(
        &self,
        _correction_id: &str,
        _conversation_id: &str,
        step: LinkCorrectionStep,
    ) -> Result<(), String> {
        if let Some((target, reason)) = &*self.fail_step.lock().unwrap()
            && *target == step
        {
            return Err(reason.clone());
        }
        Ok(())
    }
}

fn load_skills() -> PhysicianSkills {
    PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
        .expect("load physician skills catalog")
}

fn roles() -> ModelRoles {
    ModelRoles {
        chat: ModelBinding {
            model: "interactive-chat-lane".into(),
            effort: "low".into(),
            context_window_tokens: 128_000,
            max_input_tokens: 100_000,
            adapter: "fixture".into(),
        },
        background: ModelBinding {
            model: "background-worker-lane".into(),
            effort: "high".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 250_000,
            adapter: "fixture".into(),
        },
    }
}

fn unix_seconds() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

async fn connect_stores(url: &str) -> (Store, Arc<PostgresAuthStore>) {
    let store = Store::connect(url).await.expect("connect runtime store");
    store.migrate().await.expect("migrate runtime store");
    let auth = Arc::new(
        PostgresAuthStore::connect_with_migrations(url)
            .await
            .expect("connect and migrate physician auth store"),
    );
    (store, auth)
}

async fn invite_and_sign_in(
    auth: &PostgresAuthStore,
    practice_id: &str,
    label: &str,
    admin: bool,
) -> (PhysicianIdentity, String, Vec<u8>) {
    let email = format!("{label}-{}@codahealth.test", Uuid::new_v4().simple());
    let now = unix_seconds();
    let invite = auth
        .invite(practice_id, &email, label, admin, now)
        .await
        .expect("invite physician");
    let (identity, token) = auth
        .redeem_invitation(&email, &invite, now)
        .await
        .expect("redeem physician invitation");
    let hash = PostgresAuthStore::session_reference(&token);
    (identity, email, hash)
}

fn source_pull(
    source_id: &str,
    label: &str,
    pt_id: &str,
    name: &str,
    pulled_at: i64,
) -> SourcePull {
    SourcePull {
        source_id: source_id.to_owned(),
        source_label: label.to_owned(),
        patient: SourcePatient {
            source_patient_id: pt_id.to_owned(),
            display_name: name.to_owned(),
            birth_date: Some("1972-06-19".to_owned()),
            sex_or_gender: Some("female".to_owned()),
            medical_record_numbers: vec![MedicalRecordNumber {
                system: "clinic-mrn".to_owned(),
                value: format!("MRN-{pt_id}"),
            }],
        },
        pulled_at: Some(pulled_at),
    }
}

#[allow(clippy::too_many_arguments)]
fn chart_record_with_provenance(
    workspace: &str,
    source_id: &str,
    candidate_id: &str,
    revision: &str,
    date: &str,
    title: &str,
    text: &str,
    extra_provenance: Value,
) -> SourceRevision {
    let mut prov = json!({
        "title": title,
        "record_locator": candidate_id,
    });
    if let (Some(dst), Some(src)) = (prov.as_object_mut(), extra_provenance.as_object()) {
        for (k, v) in src {
            dst.insert(k.clone(), v.clone());
        }
    }
    SourceRevision {
        workspace: workspace.to_owned(),
        source_id: source_id.to_owned(),
        candidate_id: candidate_id.to_owned(),
        revision: revision.to_owned(),
        record_type: "Communication".into(),
        dates: vec![RecordDate {
            name: "effectiveDateTime".into(),
            value: date.to_owned(),
        }],
        content: text.to_owned(),
        sha256: sha256_hex(text.as_bytes()),
        readable_parts: vec![ReadablePart {
            label: title.to_owned(),
            text: text.to_owned(),
        }],
        provenance: prov,
        envelope: None,
    }
}

fn readable_spruce_item(
    pos: u64,
    message_id: &str,
    conversation_id: &str,
    phone: &str,
    author: &str,
    body: &str,
) -> IncomingSpruceItem {
    let addr = SenderAddress::parse(phone).expect("valid phone");
    IncomingSpruceItem::Readable(ReadableSpruceItem {
        message_id: message_id.to_owned(),
        conversation_id: conversation_id.to_owned(),
        channel_position: pos,
        content_version: format!("ver-{message_id}-{pos}"),
        sender_address: Some(addr.clone()),
        author_name: Some(author.to_owned()),
        body_text: body.to_owned(),
        attachments: Vec::new(),
        app_url: Some(format!(
            "https://app.sprucehealth.com/conversations/{conversation_id}"
        )),
        created_at_spruce: Some("2026-11-12T15:00:00Z".to_owned()),
        outside_participants: vec![OutsideParticipant {
            display_name: Some(author.to_owned()),
            endpoint_channel: Some("sms".into()),
            endpoint_address: Some(addr),
            saved_contact_id: None,
        }],
    })
}

#[allow(clippy::too_many_arguments)]
async fn ingest_waiting_patient_message(
    store: &Store,
    auth: &PostgresAuthStore,
    practice: &str,
    lease_token: &str,
    pos: u64,
    message_id: &str,
    conversation_id: &str,
    phone: &str,
    author: &str,
    body: &str,
    suggested_workspace: &str,
    receiver: &RecordingReviewReceiver,
    now_ts: i64,
) -> MessageRoutingOutcome {
    store
        .ingest_spruce_item(
            practice,
            lease_token,
            &readable_spruce_item(pos, message_id, conversation_id, phone, author, body),
            now_ts,
        )
        .await
        .expect("ingest spruce item");
    store
        .record_spruce_message_classification(
            auth,
            practice,
            message_id,
            None,
            RECIPE_VERSION,
            GUIDANCE_VERSION,
            MatchingDecision::PatientMail {
                urgency: MessageUrgencyChoice::Ordinary,
                assessment: MatchAssessment::SuggestedPatient {
                    workspace: suggested_workspace.to_owned(),
                    evidence_summary: format!("Suggested match for {author}."),
                },
                rationale: "Inbound clinical message from unconfirmed phone.".into(),
            },
            Some(receiver),
            now_ts,
        )
        .await
        .expect("classify spruce message")
}

/// Runs one real `PhysicianHost` chat turn that reads `(candidate_id, revision)` via `READ_TOOL`
/// and finishes with `answer_text` (e.g. containing `[1]`).
async fn run_chat_citing_source(
    store: &Store,
    scope: &Scope,
    session_hash: &[u8],
    question: &str,
    candidate_id: &str,
    revision: &str,
    answer_text: &str,
) {
    struct CitingChatProvider {
        step: usize,
        candidate_id: String,
        revision: String,
        answer_text: String,
    }

    impl Provider for CitingChatProvider {
        async fn exchange(
            &mut self,
            _request: &Request,
            _cancel: &engine::Cancellation,
            _limits: &engine::RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.step += 1;
            match self.step {
                1 => Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "call-read-chart".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": self.candidate_id,
                                "revision": self.revision,
                                "cursor": null
                            }],
                            "max_result_bytes": 16_384
                        })
                        .to_string(),
                    })],
                    receipt: json!({"step": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let marked = if self.answer_text.contains("[1]") {
                        self.answer_text.replace("[1]", "[S1]")
                    } else if self.answer_text.contains("[S1]") {
                        self.answer_text.clone()
                    } else {
                        format!("{} [S1]", self.answer_text)
                    };
                    Ok(Turn {
                        items: vec![Item::Call(local_product::runtime::engine::Invocation {
                            id: "finish-citing-chat".into(),
                            name: local_product::physician_sources::citations::FINISH_TOOL.into(),
                            arguments: json!({
                                "answer": marked,
                                "basis": "Cited workspace chart record.",
                                "descriptions": [], "quotes": [{ "label": "S1", "quote": "Cited chart record" }],
                                "cites_none": false,
                            })
                            .to_string(),
                        })],
                        receipt: json!({"step": 2}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected chat step {other}"),
            }
        }
    }

    let skills = load_skills();
    let model_roles = roles();
    let starting = store
        .patient_starting_context(scope)
        .await
        .expect("starting context");
    let recipe = ChatAssignment::for_patient(scope.clone(), starting).recipe(&model_roles, &skills);
    let accepted = store
        .accept_authorized(
            scope,
            &Input {
                key: format!("input-{}", Uuid::new_v4().simple()),
                text: question.to_owned(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            session_hash.to_vec(),
            &skills,
            &recipe,
            None,
        )
        .await
        .expect("accept chat turn");
    assert_eq!(accepted, Accepted::New);

    let claim = store
        .claim(scope, 60)
        .await
        .expect("claim chat")
        .expect("active chat claim");
    let mut host = PhysicianHost {
        store: store.clone(),
        claim,
        authority: PostgresWorkspaceAuthority,
        shown_steps: local_product::runtime::ShownSteps::new(()),
    };
    let mut provider = CitingChatProvider {
        step: 0,
        candidate_id: candidate_id.to_owned(),
        revision: revision.to_owned(),
        answer_text: answer_text.to_owned(),
    };
    host.run(
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("complete citing chat turn");
}

/// Runs one real `PhysicianHost` chat turn that answers directly without calling chart tools,
/// returning the accepted work ID.
async fn run_chat_direct_answer(
    store: &Store,
    scope: &Scope,
    session_hash: &[u8],
    question: &str,
    answer_text: &str,
) -> String {
    struct DirectChatProvider {
        step: usize,
        answer_text: String,
    }

    impl Provider for DirectChatProvider {
        async fn exchange(
            &mut self,
            _request: &Request,
            _cancel: &engine::Cancellation,
            _limits: &engine::RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.step += 1;
            match self.step {
                1 => Ok(Turn {
                    items: vec![Item::Call(local_product::runtime::engine::Invocation {
                        id: "finish-direct-chat".into(),
                        name: local_product::physician_sources::citations::FINISH_TOOL.into(),
                        arguments: json!({
                            "answer": self.answer_text,
                            "basis": "Direct workspace answer.",
                            "descriptions": [], "quotes": [],
                            "cites_none": true,
                        })
                        .to_string(),
                    })],
                    receipt: json!({"step": 1}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected direct chat step {other}"),
            }
        }
    }

    let skills = load_skills();
    let model_roles = roles();
    let starting = store
        .patient_starting_context(scope)
        .await
        .expect("starting context");
    let recipe = ChatAssignment::for_patient(scope.clone(), starting).recipe(&model_roles, &skills);
    let accepted = store
        .accept_authorized(
            scope,
            &Input {
                key: format!("input-{}", Uuid::new_v4().simple()),
                text: question.to_owned(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            session_hash.to_vec(),
            &skills,
            &recipe,
            None,
        )
        .await
        .expect("accept direct chat turn");
    assert_eq!(accepted, Accepted::New);

    let claim = store
        .claim(scope, 60)
        .await
        .expect("claim direct chat")
        .expect("active direct chat claim");
    let mut host = PhysicianHost {
        store: store.clone(),
        claim,
        authority: PostgresWorkspaceAuthority,
        shown_steps: local_product::runtime::ShownSteps::new(()),
    };
    let mut provider = DirectChatProvider {
        step: 0,
        answer_text: answer_text.to_owned(),
    };
    host.run(
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("complete direct chat turn");
    store
        .conversation(scope)
        .await
        .expect("read direct chat conversation")
        .work
}

async fn run_understanding_save_job(
    store: &Store,
    auth: &PostgresAuthStore,
    scope: &local_product::runtime::jobs::PracticeScope,
    refs_to_read: Vec<(&str, &str)>,
    save_args: SaveUnderstandingArgs,
) {
    struct SaveJobProvider {
        turn: usize,
        refs_to_read: Vec<(String, String)>,
        save_args: SaveUnderstandingArgs,
    }
    impl Provider for SaveJobProvider {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => {
                    let references: Vec<Value> = self
                        .refs_to_read
                        .iter()
                        .map(|(cid, rev)| {
                            json!({
                                "candidate_id": cid,
                                "revision": rev,
                                "cursor": null
                            })
                        })
                        .collect();
                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "read-sources".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": references,
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        })],
                        receipt: json!({"turn": 1}),
                        usage: Default::default(),
                    })
                }
                2 => Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "save-understanding".into(),
                        name: SAVE_UNDERSTANDING_TOOL.into(),
                        arguments: serde_json::to_string(&self.save_args).unwrap(),
                    })],
                    receipt: json!({"turn": 2}),
                    usage: Default::default(),
                }),
                3 => {
                    let out = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            Item::MaterializedResult { call_id, output }
                                if call_id == "save-understanding" =>
                            {
                                Some(output.clone())
                            }
                            _ => None,
                        })
                        .unwrap_or_default();
                    assert!(out.contains("\"outcome\":\"saved\""), "{out}");
                    Ok(Turn {
                        items: vec![Item::assistant("Completed understanding save.")],
                        receipt: json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let claim = store
        .claim_practice_job(scope, 60, auth)
        .await
        .unwrap()
        .expect("claim understanding job");
    let mut provider = SaveJobProvider {
        turn: 0,
        refs_to_read: refs_to_read
            .into_iter()
            .map(|(c, r)| (c.to_owned(), r.to_owned()))
            .collect(),
        save_args,
    };
    let mut host = PracticeJobHost {
        store: store.clone(),
        claim,
        authority: auth,
    };
    host.run(
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("understanding job finishes");
}

fn make_save_args(
    expected_revision: u64,
    overview: &str,
    source_id: &str,
    candidate_id: &str,
    revision: &str,
    claim_text: &str,
) -> SaveUnderstandingArgs {
    SaveUnderstandingArgs {
        expected_revision: Some(expected_revision),
        operating_index: OperatingIndex {
            working_overview: overview.to_owned(),
            known_health_systems: vec![KnownHealthSystem {
                name: "Clinic EHR".into(),
                standing: HealthSystemStanding::Connected {
                    source_id: source_id.to_owned(),
                },
                note: "Primary care chart".into(),
            }],
            open_threads: vec!["Longitudinal primary care".into()],
            anchors: vec![IndexedAnchor {
                reference: ExactRecordReference {
                    candidate_id: candidate_id.to_owned(),
                    revision: revision.to_owned(),
                },
                source_id: source_id.to_owned(),
                topic: "Primary anchor".into(),
            }],
        },
        claims: vec![SummaryClaimInput {
            text: claim_text.to_owned(),
            citations: vec![ExactRecordReference {
                candidate_id: candidate_id.to_owned(),
                revision: revision.to_owned(),
            }],
        }],
        material_gaps: vec!["No external gaps".into()],
        scope_note: None,
    }
}

struct FollowUpSeedHost {
    store: Store,
    practice: String,
    workspace: String,
    source_work: SourceWork,
    sources: Vec<SourceRevision>,
    events: Vec<Event>,
}

#[async_trait]
impl Product for FollowUpSeedHost {
    async fn message_sources(
        &mut self,
    ) -> Result<Option<Vec<ProductMessage>>, engine::ProductError> {
        Ok(Some(vec![]))
    }

    async fn admit_binding(&mut self, _: &RequestBinding) -> Result<(), engine::ProductError> {
        Ok(())
    }

    fn read_only(&self, name: &str) -> bool {
        name == READ_TOOL
    }

    fn read_result(&mut self, _: &str) -> engine::recipe::ToolFuture<Option<String>> {
        Box::pin(async { Ok(None) })
    }

    async fn assert_current(&mut self) -> Result<(), engine::ProductError> {
        Ok(())
    }

    async fn persist(&mut self, event: Event) -> Result<(), engine::ProductError> {
        self.events.push(event);
        Ok(())
    }

    fn execute(&mut self, call: &Invocation, arguments: Value) -> engine::recipe::ToolFuture {
        let store = self.store.clone();
        let practice = self.practice.clone();
        let workspace = self.workspace.clone();
        let source_work = self.source_work.clone();
        let sources = self.sources.clone();
        let typed_call = Invocation {
            id: call.id.clone(),
            name: call.name.clone(),
            arguments: arguments.clone(),
        };
        Box::pin(async move {
            if is_follow_up_tool(&typed_call.name) {
                return store
                    .execute_follow_up_tool(&practice, &workspace, &source_work, &typed_call)
                    .await;
            }
            if typed_call.name == READ_TOOL {
                let request = local_product::physician_sources::ChartRequest::parse(
                    &typed_call.name,
                    &arguments,
                    local_product::physician_sources::ChartScope::Chat,
                )
                .map_err(|r| engine::ProductError::Failed(engine::Cause::from(r.to_string())))?;
                return request
                    .run(&workspace, 1, &sources)
                    .map_err(|e| engine::ProductError::Failed(engine::Cause::from(e)));
            }
            Ok(ToolOutcome::refused(format!(
                "unknown tool {}",
                typed_call.name
            )))
        })
    }

    async fn finish(&mut self, _: &Turn) -> Result<Finish, engine::ProductError> {
        Ok(Finish::Committed)
    }

    async fn finish_partial(
        &mut self,
        _: &Turn,
        _: &[engine::LimitBackstop],
    ) -> Result<(), engine::ProductError> {
        Ok(())
    }

    async fn window_seed(&mut self) -> Result<WindowSeed, engine::ProductError> {
        Ok(WindowSeed {
            items: vec![],
            note_references: vec![],
        })
    }
}

#[allow(clippy::too_many_arguments)]
async fn create_job_follow_up(
    store: &Store,
    practice: &str,
    workspace: &str,
    source_work: SourceWork,
    source_rev: SourceRevision,
    follow_up_id: &str,
    summary: &str,
    due_at: i64,
) {
    struct OneFollowUpProvider {
        step: usize,
        follow_up_id: String,
        summary: String,
        due_at: i64,
        candidate_id: String,
        revision: String,
    }
    impl Provider for OneFollowUpProvider {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.step += 1;
            match self.step {
                1 => Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "fu-create".into(),
                        name: FOLLOW_UP_TOOL.into(),
                        arguments: json!({
                            "operation": "create",
                            "follow_up_id": self.follow_up_id,
                            "summary": self.summary,
                            "due_at": self.due_at,
                            "records": [{
                                "candidate_id": self.candidate_id,
                                "revision": self.revision
                            }]
                        })
                        .to_string(),
                    })],
                    receipt: json!({"step": 1}),
                    usage: Default::default(),
                }),
                2 => Ok(Turn {
                    items: vec![Item::assistant("Follow-up scheduled.")],
                    receipt: json!({"step": 2}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected follow-up step {other}"),
            }
        }
    }

    let recipe = RequestRecipe {
        model: "controlled-follow-up-job".into(),
        context_window_tokens: 200_000,
        max_input_tokens: 150_000,
        options: json!({"effort": "high"}),
        instructions: "Schedule clinical follow-ups.".into(),
        tools: vec![
            follow_up_tool().function(),
            read_tool(local_product::physician_sources::ChartScope::Chat).function(),
        ],
    };
    let mut request = recipe.bind(None).request(Vec::new());
    request.items.clear();
    let work = request
        .freeze()
        .unwrap()
        .recover(
            &source_work.attempt_id,
            engine::recovery::Records {
                events: vec![],
                allowance_after: 0,
            },
            engine::recovery::Messages {
                sources: &[],
                partial: None,
            },
            engine::recovery::Inputs {
                accepted: vec![AcceptedInput {
                    id: source_work.work_id.clone(),
                    items: vec![Item::ProductMessage(ProductMessage {
                        id: source_work.work_id.clone(),
                        role: engine::history::Role::User,
                        text: "Schedule follow-up.".into(),
                    })],
                }],
                source_feedback: None,
            },
        )
        .unwrap();
    let mut host = FollowUpSeedHost {
        store: store.clone(),
        practice: practice.to_owned(),
        workspace: workspace.to_owned(),
        source_work,
        sources: vec![source_rev.clone()],
        events: vec![],
    };
    let mut provider = OneFollowUpProvider {
        step: 0,
        follow_up_id: follow_up_id.to_owned(),
        summary: summary.to_owned(),
        due_at,
        candidate_id: source_rev.candidate_id,
        revision: source_rev.revision,
    };
    work.run(
        &mut provider,
        &mut host,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("create job follow-up");
}

#[test]
fn link_correction_types_and_withholding_notices_preserve_causes_and_clean_model_text() {
    let undo_standing = WithheldStanding::UndoPending {
        expires_at: 1_790_000_010,
    };
    let failed_standing = WithheldStanding::CleanupFailed {
        failed_step: LinkCorrectionStep::WithdrawOriginEffects,
        failure_reason: "simulated database interruption".into(),
    };
    let settled_standing = WithheldStanding::SettledEnteredInError;

    for standing in [&undo_standing, &failed_standing, &settled_standing] {
        let notice = standing.unavailable_reason("conv-sample-42");
        assert!(notice.starts_with("Unavailable:"));
        model_text::assert_clean_model_text(&notice);
    }

    let db_err = LinkCorrectionError::Database(sea_orm::DbErr::Custom("connection reset".into()));
    assert!(
        db_err
            .source()
            .expect("DbErr preserved as #[source]")
            .to_string()
            .contains("connection reset")
    );
}

#[tokio::test]
async fn ac1_patient_access_enforced_on_link_move_undo_and_direct_reads() {
    let db = DisposableDb::create("ac1").await;
    let (store, auth) = connect_stores(&db.url).await;
    let practice = Uuid::new_v4().to_string();
    let ws_a = format!("pw-a-{}", Uuid::new_v4().simple());
    let ws_b = format!("pw-b-{}", Uuid::new_v4().simple());
    let ws_secret = format!("pw-secret-{}", Uuid::new_v4().simple());
    let now_ts = 1_790_000_000;

    let (dr_admin, _, _) = invite_and_sign_in(&auth, &practice, "dr-admin", true).await;
    let (dr_panel, panel_email, panel_hash) =
        invite_and_sign_in(&auth, &practice, "dr-panel", false).await;
    let (dr_outsider, _, outsider_hash) =
        invite_and_sign_in(&auth, &practice, "dr-outsider", false).await;

    for ws in [&ws_a, &ws_b, &ws_secret] {
        auth.register_patient(&practice, ws).await.unwrap();
        auth.set_membership(&dr_admin, ws, Membership::Allowed)
            .await
            .unwrap();
    }
    // `dr_panel` has access to `ws_a` and `ws_b`, NOT `ws_secret`; `dr_outsider` has access to neither `ws_a` nor `ws_b`.
    auth.set_membership(&dr_panel, &ws_a, Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&dr_panel, &ws_b, Membership::Allowed)
        .await
        .unwrap();

    store
        .record_chart(
            &ws_a,
            &[chart_record_with_provenance(
                &ws_a,
                "clinic-ehr",
                "cand-ac1-a",
                "rev-1",
                "2026-11-10",
                "Office Visit",
                "PANEL_ONLY_CHART_NOTE_A_7711",
                json!({}),
            )],
            &source_pull("clinic-ehr", "Clinic EHR", "pt-a", "Alice Carter", now_ts),
        )
        .await
        .unwrap();

    store
        .connect_spruce_channel(auth.as_ref(), &dr_admin, 0, None, now_ts)
        .await
        .unwrap();
    let lease = store
        .acquire_spruce_reader_lease(&practice, "worker-ac1", 600, now_ts)
        .await
        .unwrap();
    let receiver = RecordingReviewReceiver::default();

    let routing = ingest_waiting_patient_message(
        &store,
        auth.as_ref(),
        &practice,
        &lease.lease_token,
        1,
        "msg-ac1-1",
        "conv-ac1",
        "+14155550101",
        "Alice Carter",
        "PANEL_ONLY_SPRUCE_BODY_7711",
        &ws_a,
        &receiver,
        now_ts,
    )
    .await;
    assert!(matches!(routing, MessageRoutingOutcome::Waiting { .. }));

    // 1. Outsider without access to `ws_a` cannot link `conv-ac1` to `ws_a`.
    let denied_link = store
        .link_conversation_to_patient(
            auth.as_ref(),
            &dr_outsider,
            LinkConversationCommand {
                command_key: "link-outsider-denied".into(),
                practice: practice.clone(),
                conversation_id: "conv-ac1".into(),
                workspace: ws_a.clone(),
                sender_policy: SenderConfirmationPolicy::ConversationOnly,
                now_ts: now_ts + 1,
            },
            Some(&receiver),
        )
        .await;
    assert!(matches!(
        denied_link,
        Err(LinkCorrectionError::AccessDenied { .. })
    ));

    // 2. Authorized `dr_panel` links `conv-ac1` to `ws_a` and opens the conversation + runs a chat turn + raises a concern.
    store
        .link_conversation_to_patient(
            auth.as_ref(),
            &dr_panel,
            LinkConversationCommand {
                command_key: "link-panel-ok".into(),
                practice: practice.clone(),
                conversation_id: "conv-ac1".into(),
                workspace: ws_a.clone(),
                sender_policy: SenderConfirmationPolicy::ConversationOnly,
                now_ts: now_ts + 2,
            },
            Some(&receiver),
        )
        .await
        .unwrap();

    let chat_scope = Scope {
        practice: practice.clone(),
        physician: dr_panel.physician_id.clone(),
        workspace: ws_a.clone(),
        chat: "chat-ac1".into(),
    };
    run_chat_citing_source(
        &store,
        &chat_scope,
        &panel_hash,
        "Summarize Alice's office visit.",
        "cand-ac1-a",
        "rev-1",
        "Alice's office visit recorded PANEL_ONLY_CHART_NOTE_A_7711 [1].",
    )
    .await;

    store
        .raise_concern(
            &CommandKey::new("cmd-ac1-concern").unwrap(),
            &RaiseConcern {
                practice: practice.clone(),
                workspace: ws_a.clone(),
                id: ConcernId::new("concern-ac1").unwrap(),
                urgency: ConcernUrgency::Today,
                reason: "Follow up PANEL_ONLY_CHART_NOTE_A_7711.".into(),
                records: vec![ExactRecordReference::new("cand-ac1-a", "rev-1").unwrap()],
                source_work: SourceWork::new("patient_review", "work-ac1", "attempt-1").unwrap(),
            },
        )
        .await
        .unwrap();

    store
        .link_reply_conversation(
            &practice,
            "conv-ac1",
            ConversationLink::LinkedPatient {
                workspace: ws_a.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: "conv-ac1".into(),
            message_id: "msg-ac1-1".into(),
            sender: MessageSender::Patient,
            text: "PANEL_ONLY_SPRUCE_BODY_7711".into(),
            sent_at: now_ts + 2,
        })
        .await
        .unwrap();

    // Direct reads BEFORE losing access succeed for `dr_panel` and are refused for `dr_outsider`.
    let opened_before = store
        .open_spruce_conversation(auth.as_ref(), &dr_panel, "conv-ac1", now_ts + 3)
        .await
        .unwrap();
    assert_eq!(opened_before.messages.len(), 1);
    assert!(opened_before.is_open_for_viewer);
    let saved_conv_before = store
        .spruce_conversation(&practice, "conv-ac1")
        .await
        .unwrap()
        .unwrap();
    assert!(
        saved_conv_before
            .open_viewers
            .contains(&dr_panel.physician_id)
    );
    assert_eq!(
        store
            .authorized_workspace_sources(auth.as_ref(), &dr_panel, &ws_a)
            .await
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        store
            .authorized_patient_spruce_messages(auth.as_ref(), &dr_panel, &ws_a)
            .await
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        store
            .authorized_workspace_past_work(auth.as_ref(), &dr_panel, &ws_a)
            .await
            .unwrap()
            .len(),
        1
    );
    assert!(
        store
            .authorized_reopen_conversation(auth.as_ref(), &dr_panel, &ws_a, "chat-ac1")
            .await
            .unwrap()
            .is_some()
    );
    assert_eq!(
        store
            .authorized_open_concerns(
                auth.as_ref(),
                &dr_panel,
                &ws_a,
                &panel_hash,
                &mut PostgresWorkspaceAuthority,
            )
            .await
            .unwrap()
            .len(),
        1
    );

    assert!(matches!(
        store
            .authorized_workspace_sources(auth.as_ref(), &dr_outsider, &ws_a)
            .await,
        Err(LinkCorrectionError::AccessDenied { .. })
    ));
    let outsider_access = WorkspaceAccess {
        practice: &practice,
        physician: &dr_outsider.physician_id,
        workspace: &ws_a,
        session_hash: &outsider_hash,
    };
    assert!(
        store
            .physician_reply_surface(
                &outsider_access,
                "conv-ac1",
                &mut PostgresWorkspaceAuthority
            )
            .await
            .is_err()
    );

    // 3. `dr_panel` cannot move `conv-ac1` to `ws_secret` (lacks target access), and `dr_outsider` cannot move from `ws_a` (lacks origin access).
    assert!(matches!(
        store
            .correct_patient_link(
                auth.as_ref(),
                &dr_panel,
                CorrectPatientLinkCommand {
                    command_key: "move-to-secret-denied".into(),
                    practice: practice.clone(),
                    conversation_id: "conv-ac1".into(),
                    destination: CorrectionDestination::Patient {
                        workspace: ws_secret.clone(),
                    },
                    undo_window_seconds: 10,
                    now_ts: now_ts + 4,
                },
                Some(&receiver),
            )
            .await,
        Err(LinkCorrectionError::AccessDenied { ref subject, .. }) if subject == &ws_secret
    ));
    assert!(matches!(
        store
            .correct_patient_link(
                auth.as_ref(),
                &dr_outsider,
                CorrectPatientLinkCommand {
                    command_key: "move-from-a-denied".into(),
                    practice: practice.clone(),
                    conversation_id: "conv-ac1".into(),
                    destination: CorrectionDestination::NotAPatient,
                    undo_window_seconds: 10,
                    now_ts: now_ts + 4,
                },
                Some(&receiver),
            )
            .await,
        Err(LinkCorrectionError::AccessDenied { ref subject, .. }) if subject == &ws_a
    ));

    // 4. Revoke `dr_panel`'s access to `ws_a` -> immediately refuses `correct_patient_link`, closes open Spruce conversation & visibility, and closes all direct reads!
    auth.set_membership(&dr_panel, &ws_a, Membership::Denied)
        .await
        .unwrap();

    assert!(matches!(
        store
            .correct_patient_link(
                auth.as_ref(),
                &dr_panel,
                CorrectPatientLinkCommand {
                    command_key: "move-after-loss-denied".into(),
                    practice: practice.clone(),
                    conversation_id: "conv-ac1".into(),
                    destination: CorrectionDestination::Patient {
                        workspace: ws_b.clone(),
                    },
                    undo_window_seconds: 10,
                    now_ts: now_ts + 5,
                },
                Some(&receiver),
            )
            .await,
        Err(LinkCorrectionError::AccessDenied { ref subject, .. }) if subject == &ws_a
    ));

    assert!(
        store
            .open_spruce_conversation(auth.as_ref(), &dr_panel, "conv-ac1", now_ts + 5)
            .await
            .is_err()
    );
    assert!(
        store
            .open_spruce_conversations_for_physician(auth.as_ref(), &dr_panel, now_ts + 5)
            .await
            .unwrap()
            .is_empty(),
        "losing patient access must immediately close open Spruce conversation visibility"
    );

    assert!(matches!(
        store
            .authorized_workspace_sources(auth.as_ref(), &dr_panel, &ws_a)
            .await,
        Err(LinkCorrectionError::AccessDenied { .. })
    ));
    assert!(matches!(
        store
            .authorized_patient_spruce_messages(auth.as_ref(), &dr_panel, &ws_a)
            .await,
        Err(LinkCorrectionError::AccessDenied { .. })
    ));
    assert!(matches!(
        store
            .authorized_workspace_past_work(auth.as_ref(), &dr_panel, &ws_a)
            .await,
        Err(LinkCorrectionError::AccessDenied { .. })
    ));
    assert!(matches!(
        store
            .authorized_reopen_conversation(auth.as_ref(), &dr_panel, &ws_a, "chat-ac1")
            .await,
        Err(LinkCorrectionError::AccessDenied { .. })
    ));
    assert!(matches!(
        store
            .authorized_open_concerns(
                auth.as_ref(),
                &dr_panel,
                &ws_a,
                &panel_hash,
                &mut PostgresWorkspaceAuthority,
            )
            .await,
        Err(LinkCorrectionError::AccessDenied { .. })
    ));
    let panel_access_a = WorkspaceAccess {
        practice: &practice,
        physician: &dr_panel.physician_id,
        workspace: &ws_a,
        session_hash: &panel_hash,
    };
    assert!(
        store
            .physician_reply_surface(&panel_access_a, "conv-ac1", &mut PostgresWorkspaceAuthority)
            .await
            .is_err()
    );

    // 5. Restore `ws_a` access, start move `ws_a -> ws_b`, then revoke `ws_b` access -> `Undo` is refused because physician lost access to `ws_b`!
    auth.set_membership(&dr_panel, &ws_a, Membership::Allowed)
        .await
        .unwrap();
    store
        .correct_patient_link(
            auth.as_ref(),
            &dr_panel,
            CorrectPatientLinkCommand {
                command_key: "move-a-to-b-ok".into(),
                practice: practice.clone(),
                conversation_id: "conv-ac1".into(),
                destination: CorrectionDestination::Patient {
                    workspace: ws_b.clone(),
                },
                undo_window_seconds: 10,
                now_ts: now_ts + 6,
            },
            Some(&receiver),
        )
        .await
        .unwrap();

    auth.set_membership(&dr_panel, &ws_b, Membership::Denied)
        .await
        .unwrap();
    assert!(matches!(
        store
            .undo_link_correction(
                auth.as_ref(),
                &dr_panel,
                UndoLinkCorrectionCommand {
                    command_key: "undo-without-b-access".into(),
                    practice: practice.clone(),
                    conversation_id: "conv-ac1".into(),
                    now_ts: now_ts + 8,
                },
                Some(&receiver),
            )
            .await,
        Err(LinkCorrectionError::AccessDenied { ref subject, .. }) if subject == &ws_b
    ));

    // Also verify deactivating `dr_panel` refuses all commands even when `ws_b` membership is restored!
    auth.set_membership(&dr_panel, &ws_b, Membership::Allowed)
        .await
        .unwrap();
    assert!(auth.deactivate(&panel_email).await);
    assert!(matches!(
        store
            .undo_link_correction(
                auth.as_ref(),
                &dr_panel,
                UndoLinkCorrectionCommand {
                    command_key: "undo-after-deactivate".into(),
                    practice: practice.clone(),
                    conversation_id: "conv-ac1".into(),
                    now_ts: now_ts + 9,
                },
                Some(&receiver),
            )
            .await,
        Err(LinkCorrectionError::AccessDenied { .. })
    ));

    db.cleanup().await;
}

#[tokio::test]
async fn ac2_move_immediately_withholds_sources_saved_outputs_and_exact_citations_dedupes_commands_and_marks_entered_in_error_without_deletion()
 {
    let db = DisposableDb::create("ac2").await;
    let (store, auth) = connect_stores(&db.url).await;
    let practice = Uuid::new_v4().to_string();
    let ws_a = format!("pw-origin-{}", Uuid::new_v4().simple());
    let ws_b = format!("pw-target-{}", Uuid::new_v4().simple());
    let now_ts = 1_790_100_000;

    let (dr, _, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-ac2", true).await;
    for ws in [&ws_a, &ws_b] {
        auth.register_patient(&practice, ws).await.unwrap();
        auth.set_membership(&dr, ws, Membership::Allowed)
            .await
            .unwrap();
    }

    let wrong_secret = "WRONG_PATIENT_AC2_PENICILLIN_SECRET_9182";
    let draft_secret = "WRONG_PATIENT_AC2_REPLY_DRAFT_SECRET_9182";
    let clean_note = "CLEAN_PATIENT_A_AMLODIPINE_NOTE_1104";

    store
        .connect_spruce_channel(auth.as_ref(), &dr, 0, None, now_ts)
        .await
        .unwrap();
    let lease = store
        .acquire_spruce_reader_lease(&practice, "worker-ac2", 600, now_ts)
        .await
        .unwrap();
    let receiver = RecordingReviewReceiver::default();

    ingest_waiting_patient_message(
        &store,
        auth.as_ref(),
        &practice,
        &lease.lease_token,
        1,
        "msg-ac2-1",
        "conv-ac2",
        "+14155550202",
        "Elena Vance",
        wrong_secret,
        &ws_a,
        &receiver,
        now_ts,
    )
    .await;

    store
        .link_conversation_to_patient(
            auth.as_ref(),
            &dr,
            LinkConversationCommand {
                command_key: "link-ac2-to-a".into(),
                practice: practice.clone(),
                conversation_id: "conv-ac2".into(),
                workspace: ws_a.clone(),
                sender_policy: SenderConfirmationPolicy::ConversationOnly,
                now_ts: now_ts + 1,
            },
            Some(&receiver),
        )
        .await
        .unwrap();

    // Seed clean chart record + wrong-patient Spruce message chart record (including a PDF page attachment!) on `ws_a`.
    let clean_rec = chart_record_with_provenance(
        &ws_a,
        "clinic-ehr",
        "cand-clean-a",
        "rev-1",
        "2026-11-10",
        "Hypertension Note",
        clean_note,
        json!({}),
    );
    let wrong_rec = chart_record_with_provenance(
        &ws_a,
        "spruce",
        "cand-wrong-ac2",
        "rev-w1",
        "2026-11-12",
        "Inbound Spruce Message",
        wrong_secret,
        json!({
            "conversation_id": "conv-ac2",
            "message_id": "msg-ac2-1",
        }),
    );

    store
        .record_chart(
            &ws_a,
            std::slice::from_ref(&clean_rec),
            &source_pull(
                "clinic-ehr",
                "Clinic EHR",
                "pt-a",
                "Clara Morales",
                now_ts + 2,
            ),
        )
        .await
        .unwrap();
    store
        .record_chart(
            &ws_a,
            std::slice::from_ref(&wrong_rec),
            &source_pull("spruce", "Spruce", "pt-a", "Clara Morales", now_ts + 2),
        )
        .await
        .unwrap();

    // Run a physician chat turn on `ws_a` that reads `cand-wrong-ac2` and quotes `wrong_secret` with citation `[1]`.
    let chat_scope = Scope {
        practice: practice.clone(),
        physician: dr.physician_id.clone(),
        workspace: ws_a.clone(),
        chat: "chat-ac2-tainted".into(),
    };
    run_chat_citing_source(
        &store,
        &chat_scope,
        &dr_hash,
        "What did the inbound Spruce message report?",
        "cand-wrong-ac2",
        "rev-w1",
        &format!("The inbound message reported {wrong_secret} [1]."),
    )
    .await;

    // Also run a clean chat turn on `ws_a` citing only `cand-clean-a` to prove unrelated history stays intact!
    let clean_chat_scope = Scope {
        practice: practice.clone(),
        physician: dr.physician_id.clone(),
        workspace: ws_a.clone(),
        chat: "chat-ac2-clean".into(),
    };
    run_chat_citing_source(
        &store,
        &clean_chat_scope,
        &dr_hash,
        "What is Clara's blood pressure medication?",
        "cand-clean-a",
        "rev-1",
        &format!("Clara takes {clean_note} [1]."),
    )
    .await;

    // Seed a reply proposal and physician reply draft on `ws_a` for `conv-ac2`.
    store
        .link_reply_conversation(
            &practice,
            "conv-ac2",
            ConversationLink::LinkedPatient {
                workspace: ws_a.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: "conv-ac2".into(),
            message_id: "msg-ac2-1".into(),
            sender: MessageSender::Patient,
            text: wrong_secret.into(),
            sent_at: now_ts + 2,
        })
        .await
        .unwrap();
    store
        .start_or_read_reply_review(&practice, "conv-ac2", "rev-ac2-1", "run-ac2-1")
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: "conv-ac2".into(),
            review_id: "rev-ac2-1".into(),
            run_id: "run-ac2-1".into(),
            completion_key: "comp-ac2-1".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: format!("Proposal referencing {wrong_secret}."),
                basis: ReplyProposalBasisInput {
                    summary: "Inbound message review.".into(),
                    limits: vec![],
                    cited_records: vec![
                        ExactRecordReference::new("cand-wrong-ac2", "rev-w1").unwrap(),
                    ],
                },
                cited_records: vec![ExactRecordReference::new("cand-wrong-ac2", "rev-w1").unwrap()],
            }),
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();

    let access_a = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &ws_a,
        session_hash: &dr_hash,
    };
    store
        .edit_reply_proposal_draft(
            &access_a,
            "conv-ac2",
            ExpectedDraftBase::ProposalVersion(1),
            draft_secret,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();

    // Sanity check pre-move: `reopen_conversation` and `sources` expose `wrong_secret`.
    let pre_reopen = store
        .reopen_conversation(&chat_scope)
        .await
        .unwrap()
        .unwrap();
    assert!(
        serde_json::to_string(&pre_reopen)
            .unwrap()
            .contains(wrong_secret)
    );

    // 1. Move `conv-ac2` from `ws_a -> ws_b` (`Undo` open for 10s).
    let move_cmd = CorrectPatientLinkCommand {
        command_key: "cmd-move-ac2-1".into(),
        practice: practice.clone(),
        conversation_id: "conv-ac2".into(),
        destination: CorrectionDestination::Patient {
            workspace: ws_b.clone(),
        },
        undo_window_seconds: 10,
        now_ts: now_ts + 10,
    };
    let moved = store
        .correct_patient_link(auth.as_ref(), &dr, move_cmd.clone(), Some(&receiver))
        .await
        .unwrap();
    let LinkCorrectionOutcome::Moved { record, .. } = moved else {
        panic!("expected Moved outcome");
    };
    assert_eq!(record.correction_epoch, 1);
    assert_eq!(record.original_workspace, ws_a);
    assert!(record.source_candidate_ids.contains("cand-wrong-ac2"));
    assert!(record.withheld_work_ids.contains("chat-ac2-tainted"));

    // 2. Repeated commands (same key AND fresh key to same target) save ONE correction (`Duplicate`).
    let replay_same_key = store
        .correct_patient_link(auth.as_ref(), &dr, move_cmd.clone(), Some(&receiver))
        .await
        .unwrap();
    assert!(matches!(
        replay_same_key,
        LinkCorrectionOutcome::Duplicate { .. }
    ));

    let replay_fresh_key = store
        .correct_patient_link(
            auth.as_ref(),
            &dr,
            CorrectPatientLinkCommand {
                command_key: "cmd-move-ac2-fresh-repeat".into(),
                now_ts: now_ts + 12,
                ..move_cmd
            },
            Some(&receiver),
        )
        .await
        .unwrap();
    assert!(matches!(
        replay_fresh_key,
        LinkCorrectionOutcome::Duplicate { .. }
    ));
    assert_eq!(
        store
            .conversation_link_corrections(&practice, "conv-ac2")
            .await
            .unwrap()
            .len(),
        1,
        "repeated move commands must save exactly one correction record"
    );

    // 3. Immediately on move (even while `Undo` is still open!), NO content from `conv-ac2` is readable on `ws_a` across EVERY read owner:
    let sources_a = store
        .authorized_workspace_sources(auth.as_ref(), &dr, &ws_a)
        .await
        .unwrap();
    assert_eq!(sources_a.len(), 1);
    assert_eq!(sources_a[0].candidate_id, "cand-clean-a");
    assert!(
        !serde_json::to_string(&sources_a)
            .unwrap()
            .contains(wrong_secret)
    );

    let linked_msgs_a = store
        .authorized_patient_spruce_messages(auth.as_ref(), &dr, &ws_a)
        .await
        .unwrap();
    assert!(linked_msgs_a.is_empty());

    // Direct chart search & exact read & document-page read tools on `ws_a`:
    let claim_probe = {
        let probe_scope = Scope {
            practice: practice.clone(),
            physician: dr.physician_id.clone(),
            workspace: ws_a.clone(),
            chat: "chat-ac2-probe".into(),
        };
        let skills = load_skills();
        let starting = store.patient_starting_context(&probe_scope).await.unwrap();
        assert!(
            !starting.developer_text().contains(wrong_secret),
            "starting context must never leak withheld content"
        );
        model_text::assert_clean_model_text(&starting.developer_text());
        let recipe =
            ChatAssignment::for_patient(probe_scope.clone(), starting).recipe(&roles(), &skills);
        store
            .accept_authorized(
                &probe_scope,
                &Input {
                    key: "probe-input-1".into(),
                    text: "Check sources.".into(),
                    timezone: "America/Los_Angeles".into(),
                    scenario: None,
                },
                dr_hash.clone(),
                &skills,
                &recipe,
                None,
            )
            .await
            .unwrap();
        store.claim(&probe_scope, 60).await.unwrap().unwrap()
    };
    let probe_conv = store.conversation(&claim_probe.scope).await.unwrap();

    let ToolOutcome::Returned(search_out) = store
        .execute_chat_tool(
            &claim_probe,
            &probe_conv,
            &[],
            &Invocation {
                id: "s1".into(),
                name: SEARCH_TOOL.into(),
                arguments: json!({"query": "PENICILLIN"}).to_string(),
            },
            json!({"query": "PENICILLIN"}),
        )
        .await
        .unwrap()
    else {
        panic!("expected Returned from search");
    };
    assert!(!search_out.contains(wrong_secret));
    assert!(!search_out.contains("cand-wrong-ac2"));

    let ToolOutcome::Returned(read_out) = store
        .execute_chat_tool(
            &claim_probe,
            &probe_conv,
            &[],
            &Invocation {
                id: "r1".into(),
                name: READ_TOOL.into(),
                arguments: json!({
                    "references": [{"candidate_id": "cand-wrong-ac2", "revision": "rev-w1", "cursor": null}],
                    "max_result_bytes": 16_384
                })
                .to_string(),
            },
            json!({
                "references": [{"candidate_id": "cand-wrong-ac2", "revision": "rev-w1", "cursor": null}],
                "max_result_bytes": 16_384
            }),
        )
        .await
        .unwrap()
    else {
        panic!("expected Returned from read_patient_sources");
    };
    assert!(!read_out.contains(wrong_secret));
    assert!(
        read_out.contains("withheld_wrong_patient_link") && read_out.contains("Unavailable:"),
        "withheld exact chart read must return explicit unavailable reason: {read_out}"
    );
    model_text::assert_clean_model_text(&read_out);

    let ToolOutcome::Returned(page_out) = store
        .execute_chat_tool(
            &claim_probe,
            &probe_conv,
            &[],
            &Invocation {
                id: "p1".into(),
                name: READ_DOCUMENT_PAGE_TOOL.into(),
                arguments: json!({
                    "document_id": "cand-wrong-ac2",
                    "version": 1,
                    "page_number": 1
                })
                .to_string(),
            },
            json!({
                "document_id": "cand-wrong-ac2",
                "version": 1,
                "page_number": 1
            }),
        )
        .await
        .unwrap()
    else {
        panic!("expected Returned from read_document_page");
    };
    assert!(!page_out.contains(wrong_secret));
    assert!(page_out.contains("withheld_wrong_patient_link"));
    assert!(page_out.contains("Unavailable:"));

    // Saved outputs, history lists, `Past work`, and exact citations on `ws_a`:
    let read_state = store
        .read(&chat_scope)
        .await
        .unwrap()
        .expect("saved conversation");
    for msg in &read_state.messages {
        if matches!(msg.role, MessageRole::Assistant { .. }) {
            assert!(!msg.text.contains(wrong_secret));
            assert!(msg.text.contains("Unavailable:"));
        }
    }

    let reopened = store
        .authorized_reopen_conversation(auth.as_ref(), &dr, &ws_a, "chat-ac2-tainted")
        .await
        .unwrap()
        .expect("reopened conversation");
    let reopened_json = serde_json::to_string(&reopened).unwrap();
    assert!(
        !reopened_json.contains(wrong_secret),
        "reopened conversation and exact citations must never expose wrong-patient text: {reopened_json}"
    );
    assert!(reopened.turns[1].raw_text.contains("Unavailable:"));
    assert_eq!(reopened.sources.len(), 1);
    let (_, withheld_src) = reopened.sources.iter().next().unwrap();
    assert_eq!(withheld_src.title, "Unavailable · Withheld patient message");
    assert!(withheld_src.quote.contains("Unavailable:"));
    assert!(withheld_src.body.contains("Unavailable:"));
    assert!(withheld_src.raw_content.is_empty());

    // Meanwhile clean conversation on `ws_a` remains completely readable!
    let clean_reopened = store
        .authorized_reopen_conversation(auth.as_ref(), &dr, &ws_a, "chat-ac2-clean")
        .await
        .unwrap()
        .unwrap();
    assert!(clean_reopened.turns[1].raw_text.contains(clean_note));

    let past_work = store
        .authorized_workspace_past_work(auth.as_ref(), &dr, &ws_a)
        .await
        .unwrap();
    let past_work_json = serde_json::to_string(&past_work).unwrap();
    assert!(
        !past_work_json.contains(wrong_secret) && !past_work_json.contains(draft_secret),
        "workspace_past_work must withhold both tainted conversation summary and reply draft text: {past_work_json}"
    );
    let draft_item = past_work
        .iter()
        .find(|item| matches!(item.target, HistoryWorkTarget::ReplyDraft { .. }))
        .expect("withheld reply draft row present");
    assert_eq!(draft_item.title, "Unavailable patient reply draft");
    assert!(draft_item.summary.contains("Unavailable:"));

    // Reply surface on `ws_a` refuses access to the moved conversation (`ConversationWithheld`):
    assert!(matches!(
        store
            .physician_reply_surface(&access_a, "conv-ac2", &mut PostgresWorkspaceAuthority)
            .await,
        Err(ReplyProposalError::ConversationWithheld { .. })
    ));

    // 4. Before `Undo` expires (`now_ts + 15 < now_ts + 20`), messages are NOT yet marked `entered_in_error`:
    assert!(
        store
            .entered_in_error_spruce_messages(&practice, &ws_a)
            .await
            .unwrap()
            .is_empty()
    );

    // 5. Advance past `Undo` (`now_ts + 25 >= now_ts + 20`) and settle:
    let batch = store
        .settle_expired_link_corrections(&practice, Some(&receiver), now_ts + 25)
        .await
        .unwrap();
    assert_eq!(batch.settled.len(), 1);
    assert!(batch.failed.is_empty());

    let eie = store
        .entered_in_error_spruce_messages(&practice, &ws_a)
        .await
        .unwrap();
    assert_eq!(eie.len(), 1);
    assert_eq!(eie[0].message_id, "msg-ac2-1");

    // Verify underlying audit records in Postgres were NOT deleted (`spruce_messages` + `sources` still hold audit rows):
    let raw_conn = sea_orm::Database::connect(&db.url).await.unwrap();
    let msg_row = raw_conn
        .query_one_raw(sea_orm::Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT COUNT(*)::bigint AS cnt FROM physician_runtime_spruce_messages WHERE practice = $1 AND message_id = $2",
            [practice.clone().into(), "msg-ac2-1".into()],
        ))
        .await
        .unwrap()
        .unwrap();
    let msg_cnt: i64 = msg_row.try_get("", "cnt").unwrap();
    assert_eq!(msg_cnt, 1, "spruce message row must never be deleted");

    let src_row = raw_conn
        .query_one_raw(sea_orm::Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT COUNT(*)::bigint AS cnt FROM physician_runtime_sources WHERE workspace = $1 AND candidate = $2",
            [ws_a.clone().into(), "cand-wrong-ac2".into()],
        ))
        .await
        .unwrap()
        .unwrap();
    let src_cnt: i64 = src_row.try_get("", "cnt").unwrap();
    assert!(src_cnt >= 1, "source audit rows must never be deleted");
    let _ = raw_conn.close().await;

    // And settled reads on `ws_a` now state `"marked entered in error"` while keeping zero stale content!
    let settled_reopen = store
        .authorized_reopen_conversation(auth.as_ref(), &dr, &ws_a, "chat-ac2-tainted")
        .await
        .unwrap()
        .unwrap();
    assert!(
        !serde_json::to_string(&settled_reopen)
            .unwrap()
            .contains(wrong_secret)
    );
    assert!(
        settled_reopen.turns[1]
            .raw_text
            .contains("entered in error")
    );

    db.cleanup().await;
}
