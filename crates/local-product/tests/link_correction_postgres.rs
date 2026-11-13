//! Real PostgreSQL integration suite for wrong-patient conversation link correction,
//! patient access enforcement, immediate cross-surface withholding, `Undo` window
//! semantics, unfinished-removal fencing and recovery, and post-`Undo` settlement
//! across memory, patient understanding, concerns, follow-ups, reply proposals,
//! private reply drafts, and once-per-affected-patient review requests.

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
use local_product::physician_chat::{ChatAssignment, PhysicianSkills};
use local_product::physician_sources::{
    MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SEARCH_TOOL, SourcePatient,
    SourceRevision, read_tool,
};
use local_product::runtime::chat_operations::{
    MemoryProvenance, MemorySelection, MemoryStanding, PatientMemoryId, RecordMemoryCorrection,
};
use local_product::runtime::concerns::{
    CommandKey, ConcernId, ConcernStanding, ConcernUrgency, ExactRecordReference, RaiseConcern,
    SourceWork, WorkspaceAccess,
};
use local_product::runtime::dispatch::ReviewReceiver;
use local_product::runtime::follow_ups::{
    FOLLOW_UP_TOOL, FollowUpCommandKey, FollowUpError, FollowUpId, FollowUpStanding,
    FollowUpSummary, PhysicianCreateFollowUp, WithdrawalFenceViolation, follow_up_tool,
    is_follow_up_tool,
};
use local_product::runtime::history::HistoryWorkTarget;
use local_product::runtime::jobs::{PracticeAccessError, PracticeJobError, PracticeJobHost};
use local_product::runtime::link_correction::{
    CorrectPatientLinkCommand, CorrectionDestination, LinkConversationCommand, LinkCorrectionError,
    LinkCorrectionOutcome, LinkCorrectionRemovalProbe, LinkCorrectionStanding, LinkCorrectionStep,
    UndoLinkCorrectionCommand, WithheldStanding,
};
use local_product::runtime::reply::proposals::{
    AnsweredMessageClosure, CompleteReplyReview, ConversationLink, DisplayedReplyText,
    ExpectedDraftBase, MessageIntakeOutcome, MessageSender, ObservePracticeMessage,
    PatientMessageStanding, PhysicianDraftStanding, PracticeMessageKind, RecordPatientMessage,
    ReplyProposalBasisInput, ReplyProposalError, ReplyProposalInput, ReplyReviewDecision,
    ReplyReviewOutcome, ReplyStanding, proposal_draft_scope,
};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::spruce_matching::{
    ConversationLinkState, IncomingSpruceItem, MatchAssessment, MatchingDecision,
    MessageRoutingOutcome, MessageStanding, MessageUrgencyChoice, OutsideParticipant,
    ReadableSpruceItem, SenderAddress, SenderConfirmationPolicy,
};
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
        pulled_at,
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
                2 => Ok(Turn {
                    items: vec![Item::assistant(self.answer_text.clone())],
                    receipt: json!({"step": 2}),
                    usage: Default::default(),
                }),
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
    async fn message_sources(&mut self) -> Result<Option<Vec<ProductMessage>>, engine::Cause> {
        Ok(Some(vec![]))
    }

    async fn admit_binding(&mut self, _: &RequestBinding) -> Result<(), engine::Cause> {
        Ok(())
    }

    fn read_only(&self, name: &str) -> bool {
        name == READ_TOOL
    }

    fn read_result(&mut self, _: &str) -> engine::recipe::ToolFuture<Option<String>> {
        Box::pin(async { Ok(None) })
    }

    async fn assert_current(&mut self) -> Result<(), engine::Cause> {
        Ok(())
    }

    async fn persist(&mut self, event: Event) -> Result<(), engine::Cause> {
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
                .map_err(|r| engine::Cause::from(r.to_string()))?;
                return request
                    .run(&workspace, 1, &sources)
                    .map_err(engine::Cause::from);
            }
            Ok(ToolOutcome::refused(format!(
                "unknown tool {}",
                typed_call.name
            )))
        })
    }

    async fn finish(&mut self, _: &Turn) -> Result<Finish, engine::Cause> {
        Ok(Finish::Committed)
    }

    async fn finish_partial(
        &mut self,
        _: &Turn,
        _: &[engine::LimitBackstop],
    ) -> Result<(), engine::Cause> {
        Ok(())
    }

    async fn window_seed(&mut self) -> Result<WindowSeed, engine::Cause> {
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

#[tokio::test]
async fn ac3_undo_window_returns_to_first_patient_includes_mid_window_arrivals_chains_moves_and_cancels_on_move_back()
 {
    let db = DisposableDb::create("ac3").await;
    let (store, auth) = connect_stores(&db.url).await;
    let practice = Uuid::new_v4().to_string();
    let ws_a = format!("pw-a-{}", Uuid::new_v4().simple());
    let ws_b = format!("pw-b-{}", Uuid::new_v4().simple());
    let ws_c = format!("pw-c-{}", Uuid::new_v4().simple());
    let t0 = 1_790_200_000;

    let (dr, _, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-ac3", true).await;
    for ws in [&ws_a, &ws_b, &ws_c] {
        auth.register_patient(&practice, ws).await.unwrap();
        auth.set_membership(&dr, ws, Membership::Allowed)
            .await
            .unwrap();
    }

    store
        .connect_spruce_channel(auth.as_ref(), &dr, 0, None, t0)
        .await
        .unwrap();
    let lease = store
        .acquire_spruce_reader_lease(&practice, "worker-ac3", 3_600, t0)
        .await
        .unwrap();
    let receiver = RecordingReviewReceiver::default();

    // --- Sequence A: Move `ws_a -> ws_b`, receive new message during `Undo`, verify cleanup/reviews wait, then `Undo` restores BOTH messages! ---
    ingest_waiting_patient_message(
        &store,
        auth.as_ref(),
        &practice,
        &lease.lease_token,
        1,
        "msg-s1-initial",
        "conv-s1",
        "+14155550301",
        "Patient S1",
        "Initial S1 message.",
        &ws_a,
        &receiver,
        t0,
    )
    .await;
    store
        .link_conversation_to_patient(
            auth.as_ref(),
            &dr,
            LinkConversationCommand {
                command_key: "link-s1".into(),
                practice: practice.clone(),
                conversation_id: "conv-s1".into(),
                workspace: ws_a.clone(),
                sender_policy: SenderConfirmationPolicy::ConversationOnly,
                now_ts: t0 + 1,
            },
            Some(&receiver),
        )
        .await
        .unwrap();

    store
        .record_chart(
            &ws_a,
            &[chart_record_with_provenance(
                &ws_a,
                "spruce",
                "cand-s1-initial",
                "rev-1",
                "2026-11-12",
                "Spruce S1",
                "S1_CHART_BODY_SECRET",
                json!({"conversation_id": "conv-s1", "message_id": "msg-s1-initial"}),
            )],
            &source_pull("spruce", "Spruce", "pt-a", "Patient A", t0 + 2),
        )
        .await
        .unwrap();
    let s1_chat = Scope {
        practice: practice.clone(),
        physician: dr.physician_id.clone(),
        workspace: ws_a.clone(),
        chat: "chat-s1".into(),
    };
    run_chat_citing_source(
        &store,
        &s1_chat,
        &dr_hash,
        "Read S1.",
        "cand-s1-initial",
        "rev-1",
        "S1 said S1_CHART_BODY_SECRET [1].",
    )
    .await;

    let reviews_before_move = receiver.accepted_requests().len();

    // Move `ws_a -> ws_b` at `t0 + 10` (`Undo` expires at `t0 + 20`):
    store
        .correct_patient_link(
            auth.as_ref(),
            &dr,
            CorrectPatientLinkCommand {
                command_key: "move-s1-a-to-b".into(),
                practice: practice.clone(),
                conversation_id: "conv-s1".into(),
                destination: CorrectionDestination::Patient {
                    workspace: ws_b.clone(),
                },
                undo_window_seconds: 10,
                now_ts: t0 + 10,
            },
            Some(&receiver),
        )
        .await
        .unwrap();

    // Cleanup and review requests MUST wait until `Undo` expires:
    let early_batch = store
        .settle_expired_link_corrections(&practice, Some(&receiver), t0 + 14)
        .await
        .unwrap();
    assert!(early_batch.settled.is_empty());
    assert_eq!(
        receiver.accepted_requests().len(),
        reviews_before_move,
        "no move settlement review requests may fire before Undo expires"
    );
    assert!(
        store
            .entered_in_error_spruce_messages(&practice, &ws_a)
            .await
            .unwrap()
            .is_empty()
    );
    // Content is withheld on `ws_a` during `Undo`:
    assert!(
        !store
            .reopen_conversation(&s1_chat)
            .await
            .unwrap()
            .unwrap()
            .turns[1]
            .raw_text
            .contains("S1_CHART_BODY_SECRET")
    );

    // Mid-window message `msg-s1-during-undo` arrives at `t0 + 15` (joins `conv-s1` currently on `ws_b`):
    let mid_routing = ingest_waiting_patient_message(
        &store,
        auth.as_ref(),
        &practice,
        &lease.lease_token,
        2,
        "msg-s1-during-undo",
        "conv-s1",
        "+14155550301",
        "Patient S1",
        "Follow-up sent during Undo window.",
        &ws_b,
        &receiver,
        t0 + 15,
    )
    .await;
    assert!(matches!(
        mid_routing,
        MessageRoutingOutcome::Linked { ref workspace, .. } if workspace == &ws_b
    ));

    // Press `Undo` at `t0 + 18` (`< t0 + 20`):
    let undone = store
        .undo_link_correction(
            auth.as_ref(),
            &dr,
            UndoLinkCorrectionCommand {
                command_key: "undo-s1".into(),
                practice: practice.clone(),
                conversation_id: "conv-s1".into(),
                now_ts: t0 + 18,
            },
            Some(&receiver),
        )
        .await
        .unwrap();
    let LinkCorrectionOutcome::Undone {
        record: undone_rec,
        conversation: undone_conv,
        restored_message_ids,
    } = undone
    else {
        panic!("expected Undone outcome");
    };
    assert!(matches!(
        undone_rec.standing,
        LinkCorrectionStanding::Undone { .. }
    ));
    assert_eq!(
        undone_conv.link_state,
        ConversationLinkState::Linked {
            workspace: ws_a.clone(),
            linked_at: t0 + 18,
        }
    );
    assert_eq!(
        restored_message_ids,
        vec!["msg-s1-initial".to_owned(), "msg-s1-during-undo".to_owned()]
    );
    let msgs_on_a = store
        .authorized_patient_spruce_messages(auth.as_ref(), &dr, &ws_a)
        .await
        .unwrap();
    assert_eq!(msgs_on_a.len(), 2);
    assert!(
        store
            .authorized_patient_spruce_messages(auth.as_ref(), &dr, &ws_b)
            .await
            .unwrap()
            .is_empty()
    );
    // Withholding on `ws_a` is lifted!
    assert!(
        store
            .reopen_conversation(&s1_chat)
            .await
            .unwrap()
            .unwrap()
            .turns[1]
            .raw_text
            .contains("S1_CHART_BODY_SECRET")
    );

    // --- Sequence B: Chained moves (`ws_a -> ws_b -> ws_c -> NotAPatient`) keep original `Undo` target `ws_a`! ---
    ingest_waiting_patient_message(
        &store,
        auth.as_ref(),
        &practice,
        &lease.lease_token,
        3,
        "msg-s2-1",
        "conv-s2",
        "+14155550302",
        "Patient S2",
        "Chained move test.",
        &ws_a,
        &receiver,
        t0 + 100,
    )
    .await;
    store
        .link_conversation_to_patient(
            auth.as_ref(),
            &dr,
            LinkConversationCommand {
                command_key: "link-s2".into(),
                practice: practice.clone(),
                conversation_id: "conv-s2".into(),
                workspace: ws_a.clone(),
                sender_policy: SenderConfirmationPolicy::ConversationOnly,
                now_ts: t0 + 101,
            },
            Some(&receiver),
        )
        .await
        .unwrap();

    // Move 1: `ws_a -> ws_b`
    store
        .correct_patient_link(
            auth.as_ref(),
            &dr,
            CorrectPatientLinkCommand {
                command_key: "move-s2-1".into(),
                practice: practice.clone(),
                conversation_id: "conv-s2".into(),
                destination: CorrectionDestination::Patient {
                    workspace: ws_b.clone(),
                },
                undo_window_seconds: 10,
                now_ts: t0 + 110,
            },
            Some(&receiver),
        )
        .await
        .unwrap();
    // Move 2: `ws_b -> ws_c`
    store
        .correct_patient_link(
            auth.as_ref(),
            &dr,
            CorrectPatientLinkCommand {
                command_key: "move-s2-2".into(),
                practice: practice.clone(),
                conversation_id: "conv-s2".into(),
                destination: CorrectionDestination::Patient {
                    workspace: ws_c.clone(),
                },
                undo_window_seconds: 10,
                now_ts: t0 + 114,
            },
            Some(&receiver),
        )
        .await
        .unwrap();
    // Move 3: `ws_c -> NotAPatient`
    let move3 = store
        .correct_patient_link(
            auth.as_ref(),
            &dr,
            CorrectPatientLinkCommand {
                command_key: "move-s2-3".into(),
                practice: practice.clone(),
                conversation_id: "conv-s2".into(),
                destination: CorrectionDestination::NotAPatient,
                undo_window_seconds: 10,
                now_ts: t0 + 117,
            },
            Some(&receiver),
        )
        .await
        .unwrap();
    let LinkCorrectionOutcome::Moved { record: rec3, .. } = move3 else {
        panic!("expected Moved");
    };
    assert_eq!(rec3.correction_epoch, 1, "chained moves stay in epoch 1");
    assert_eq!(
        rec3.original_workspace, ws_a,
        "original Undo target must stay ws_a across chained moves"
    );
    assert_eq!(
        rec3.intermediate_workspaces,
        vec![ws_b.clone(), ws_c.clone()]
    );
    assert_eq!(rec3.destination, CorrectionDestination::NotAPatient);

    // Also verify moving FROM `NotAPatient -> ws_b` while `Undo` is open works and keeps `ws_a` as `Undo` target!
    let move4 = store
        .correct_patient_link(
            auth.as_ref(),
            &dr,
            CorrectPatientLinkCommand {
                command_key: "move-s2-4".into(),
                practice: practice.clone(),
                conversation_id: "conv-s2".into(),
                destination: CorrectionDestination::Patient {
                    workspace: ws_b.clone(),
                },
                undo_window_seconds: 10,
                now_ts: t0 + 119,
            },
            Some(&receiver),
        )
        .await
        .unwrap();
    let LinkCorrectionOutcome::Moved { record: rec4, .. } = move4 else {
        panic!("expected Moved from NotAPatient");
    };
    assert_eq!(rec4.original_workspace, ws_a);

    // Pressing `Undo` returns directly to `ws_a` (NOT `ws_c` or `NotAPatient`) and clears withholding on `ws_a`, `ws_b`, and `ws_c`!
    let undone_chain = store
        .undo_link_correction(
            auth.as_ref(),
            &dr,
            UndoLinkCorrectionCommand {
                command_key: "undo-s2-chain".into(),
                practice: practice.clone(),
                conversation_id: "conv-s2".into(),
                now_ts: t0 + 121,
            },
            Some(&receiver),
        )
        .await
        .unwrap();
    let LinkCorrectionOutcome::Undone {
        conversation: conv_s2_after,
        ..
    } = undone_chain
    else {
        panic!("expected Undone");
    };
    assert!(matches!(
        conv_s2_after.link_state,
        ConversationLinkState::Linked { ref workspace, .. } if workspace == &ws_a
    ));
    for ws in [&ws_a, &ws_b, &ws_c] {
        assert!(
            store.workspace_withheld_state(ws).await.unwrap().is_empty(),
            "Undo must clear withholding on {ws}"
        );
    }

    // --- Sequence C: Moving back to `ws_a` (`correct_patient_link` to `ws_a`) during `Undo` cancels the move! ---
    let move_s2_again = store
        .correct_patient_link(
            auth.as_ref(),
            &dr,
            CorrectPatientLinkCommand {
                command_key: "move-s2-epoch2".into(),
                practice: practice.clone(),
                conversation_id: "conv-s2".into(),
                destination: CorrectionDestination::Patient {
                    workspace: ws_b.clone(),
                },
                undo_window_seconds: 10,
                now_ts: t0 + 200,
            },
            Some(&receiver),
        )
        .await
        .unwrap();
    let LinkCorrectionOutcome::Moved {
        record: rec_epoch2, ..
    } = move_s2_again
    else {
        panic!("expected Moved");
    };
    assert_eq!(rec_epoch2.correction_epoch, 2);

    let cancelled = store
        .correct_patient_link(
            auth.as_ref(),
            &dr,
            CorrectPatientLinkCommand {
                command_key: "move-s2-back-to-a".into(),
                practice: practice.clone(),
                conversation_id: "conv-s2".into(),
                destination: CorrectionDestination::Patient {
                    workspace: ws_a.clone(),
                },
                undo_window_seconds: 10,
                now_ts: t0 + 204,
            },
            Some(&receiver),
        )
        .await
        .unwrap();
    let LinkCorrectionOutcome::CancelledReturnedToOrigin {
        record: cancelled_rec,
        conversation: cancelled_conv,
    } = cancelled
    else {
        panic!("expected CancelledReturnedToOrigin");
    };
    assert!(matches!(
        cancelled_rec.standing,
        LinkCorrectionStanding::CancelledReturnedToOrigin { .. }
    ));
    assert!(cancelled_conv.move_undo.is_none());
    assert!(
        store
            .workspace_withheld_state(&ws_a)
            .await
            .unwrap()
            .is_empty()
    );

    db.cleanup().await;
}

#[tokio::test]
async fn ac4_failed_removal_leaves_correction_unfinished_unavailable_with_reason_blocks_restoration_and_recovers_without_duplicate_reviews()
 {
    let db = DisposableDb::create("ac4").await;
    let (store, auth) = connect_stores(&db.url).await;
    let practice = Uuid::new_v4().to_string();
    let ws_a = format!("pw-a-{}", Uuid::new_v4().simple());
    let ws_b = format!("pw-b-{}", Uuid::new_v4().simple());
    let t0 = 1_790_300_000;

    let (dr, _, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-ac4", true).await;
    for ws in [&ws_a, &ws_b] {
        auth.register_patient(&practice, ws).await.unwrap();
        auth.set_membership(&dr, ws, Membership::Allowed)
            .await
            .unwrap();
    }

    store
        .connect_spruce_channel(auth.as_ref(), &dr, 0, None, t0)
        .await
        .unwrap();
    let lease = store
        .acquire_spruce_reader_lease(&practice, "worker-ac4", 3_600, t0)
        .await
        .unwrap();
    let receiver = RecordingReviewReceiver::default();

    let secret = "WRONG_PATIENT_AC4_WARFARIN_INR_SECRET_5541";
    ingest_waiting_patient_message(
        &store,
        auth.as_ref(),
        &practice,
        &lease.lease_token,
        1,
        "msg-ac4-1",
        "conv-ac4",
        "+14155550404",
        "Patient AC4",
        secret,
        &ws_a,
        &receiver,
        t0,
    )
    .await;
    store
        .link_conversation_to_patient(
            auth.as_ref(),
            &dr,
            LinkConversationCommand {
                command_key: "link-ac4".into(),
                practice: practice.clone(),
                conversation_id: "conv-ac4".into(),
                workspace: ws_a.clone(),
                sender_policy: SenderConfirmationPolicy::ConversationOnly,
                now_ts: t0 + 1,
            },
            Some(&receiver),
        )
        .await
        .unwrap();

    store
        .record_chart(
            &ws_a,
            &[chart_record_with_provenance(
                &ws_a,
                "spruce",
                "cand-ac4-wrong",
                "rev-1",
                "2026-11-12",
                "Inbound Warfarin Message",
                secret,
                json!({"conversation_id": "conv-ac4", "message_id": "msg-ac4-1"}),
            )],
            &source_pull("spruce", "Spruce", "pt-a", "Patient A", t0 + 2),
        )
        .await
        .unwrap();

    let chat_scope = Scope {
        practice: practice.clone(),
        physician: dr.physician_id.clone(),
        workspace: ws_a.clone(),
        chat: "chat-ac4".into(),
    };
    run_chat_citing_source(
        &store,
        &chat_scope,
        &dr_hash,
        "Check warfarin message.",
        "cand-ac4-wrong",
        "rev-1",
        &format!("Message states {secret} [1]."),
    )
    .await;

    // Clear initial link review from `receiver` so we measure exact move settlement reviews.
    let baseline_a_reviews = receiver.accepted_for_workspace(&ws_a).len();
    let baseline_b_reviews = receiver.accepted_for_workspace(&ws_b).len();

    let moved = store
        .correct_patient_link(
            auth.as_ref(),
            &dr,
            CorrectPatientLinkCommand {
                command_key: "move-ac4-a-to-b".into(),
                practice: practice.clone(),
                conversation_id: "conv-ac4".into(),
                destination: CorrectionDestination::Patient {
                    workspace: ws_b.clone(),
                },
                undo_window_seconds: 10,
                now_ts: t0 + 10,
            },
            Some(&receiver),
        )
        .await
        .unwrap();
    let LinkCorrectionOutcome::Moved { record, .. } = moved else {
        panic!("expected Moved");
    };

    // 1. Inject failure at `LinkCorrectionStep::WithdrawOriginEffects` after `Undo` expires (`t0 + 25`):
    let probe = StepFailureProbe::new(
        LinkCorrectionStep::WithdrawOriginEffects,
        "transient storage timeout during concern withdrawal",
    );
    let batch_fail1 = store
        .settle_expired_link_corrections_with_probe(&practice, Some(&receiver), &probe, t0 + 25)
        .await
        .unwrap();
    assert!(batch_fail1.settled.is_empty());
    assert_eq!(batch_fail1.failed.len(), 1);
    assert_eq!(
        batch_fail1.failed[0].failed_step,
        LinkCorrectionStep::WithdrawOriginEffects
    );

    let after_fail1 = store
        .link_correction(&practice, &record.correction_id)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        &after_fail1.standing,
        LinkCorrectionStanding::SettlingFailed {
            failed_step: LinkCorrectionStep::WithdrawOriginEffects,
            reason,
            progress,
            ..
        } if reason.contains("transient storage timeout")
            && progress.entered_in_error_message_ids.contains("msg-ac4-1")
    ));

    // Content on `ws_a` remains unavailable WITH the unfinished-removal reason (never showing `secret`!):
    let withheld_a = store.workspace_withheld_state(&ws_a).await.unwrap();
    assert!(matches!(
        &withheld_a.entries[0].standing,
        WithheldStanding::CleanupFailed {
            failed_step: LinkCorrectionStep::WithdrawOriginEffects,
            ..
        }
    ));
    let reopened_fail = store
        .authorized_reopen_conversation(auth.as_ref(), &dr, &ws_a, "chat-ac4")
        .await
        .unwrap()
        .unwrap();
    let reopened_fail_json = serde_json::to_string(&reopened_fail).unwrap();
    assert!(!reopened_fail_json.contains(secret));
    assert!(
        reopened_fail.turns[1]
            .raw_text
            .contains("wrong-patient removal for conversation conv-ac4 is unfinished")
    );
    assert!(
        reopened_fail.turns[1]
            .raw_text
            .contains("transient storage timeout")
    );

    // 2. Repeated commands CANNOT restore wrong-patient content or duplicate review requests:
    // - Repeating `correct_patient_link` to `ws_b` returns `Duplicate` and leaves `SettlingFailed` untouched.
    let dup_move = store
        .correct_patient_link(
            auth.as_ref(),
            &dr,
            CorrectPatientLinkCommand {
                command_key: "move-ac4-repeat-while-failed".into(),
                practice: practice.clone(),
                conversation_id: "conv-ac4".into(),
                destination: CorrectionDestination::Patient {
                    workspace: ws_b.clone(),
                },
                undo_window_seconds: 10,
                now_ts: t0 + 26,
            },
            Some(&receiver),
        )
        .await
        .unwrap();
    assert!(matches!(dup_move, LinkCorrectionOutcome::Duplicate { .. }));

    // - Attempting `Undo` while failed is refused (`UnfinishedCorrectionBlocksReturn`).
    assert!(matches!(
        store
            .undo_link_correction(
                auth.as_ref(),
                &dr,
                UndoLinkCorrectionCommand {
                    command_key: "undo-ac4-while-failed".into(),
                    practice: practice.clone(),
                    conversation_id: "conv-ac4".into(),
                    now_ts: t0 + 27,
                },
                Some(&receiver),
            )
            .await,
        Err(LinkCorrectionError::UnfinishedCorrectionBlocksReturn { ref workspace, .. })
            if workspace == &ws_a
    ));

    // - Attempting to move `conv-ac4` BACK to `ws_a` while `ws_a` has unfinished cleanup is refused (`UnfinishedCorrectionBlocksReturn`)!
    assert!(matches!(
        store
            .correct_patient_link(
                auth.as_ref(),
                &dr,
                CorrectPatientLinkCommand {
                    command_key: "move-ac4-back-to-dirty-a".into(),
                    practice: practice.clone(),
                    conversation_id: "conv-ac4".into(),
                    destination: CorrectionDestination::Patient {
                        workspace: ws_a.clone(),
                    },
                    undo_window_seconds: 10,
                    now_ts: t0 + 28,
                },
                Some(&receiver),
            )
            .await,
        Err(LinkCorrectionError::UnfinishedCorrectionBlocksReturn { ref workspace, .. }) if workspace == &ws_a
    ));

    // - Late/stale write attempting to create a follow-up citing withheld `cand-ac4-wrong` on `ws_a` is fenced (`WithdrawnFence`)!
    let access_a = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &ws_a,
        session_hash: &dr_hash,
    };
    let fenced_fu = store
        .create_follow_up_for_physician(
            &access_a,
            &FollowUpCommandKey::new("cmd-stale-fu-ac4").unwrap(),
            &PhysicianCreateFollowUp {
                id: FollowUpId::new("stale-warfarin-fu").unwrap(),
                summary: FollowUpSummary::new("Recheck INR from wrong-patient message.").unwrap(),
                due_at: t0 + 86_400,
                records: vec![ExactRecordReference::new("cand-ac4-wrong", "rev-1").unwrap()],
            },
            &mut PostgresWorkspaceAuthority,
        )
        .await;
    assert!(matches!(
        fenced_fu,
        Err(FollowUpError::WithdrawnFence(
            WithdrawalFenceViolation::WithdrawnCandidate { ref candidate_id, .. }
        )) if candidate_id == "cand-ac4-wrong"
    ));

    // 3. Now clear `WithdrawOriginEffects` fault, but fail `ReviewReceiver` on `ws_b` (so `ws_a`'s review dispatches once and `ws_b`'s fails!):
    probe.clear();
    receiver.fail_workspace(&ws_b);
    let batch_fail2 = store
        .settle_expired_link_corrections(&practice, Some(&receiver), t0 + 30)
        .await
        .unwrap();
    assert_eq!(batch_fail2.failed.len(), 1);
    assert_eq!(
        batch_fail2.failed[0].failed_step,
        LinkCorrectionStep::RequestAffectedReviews
    );
    // `ws_a` got its 1 review request; `ws_b` failed before acceptance:
    assert_eq!(
        receiver.accepted_for_workspace(&ws_a).len() - baseline_a_reviews,
        1
    );
    assert_eq!(
        receiver.accepted_for_workspace(&ws_b).len() - baseline_b_reviews,
        0
    );

    // 4. Clear `ReviewReceiver` failure and recover via `settle_expired_link_corrections` -> finishes `Settled` with ZERO duplicate review requests!
    receiver.clear_failures();
    let batch_recovered = store
        .settle_expired_link_corrections(&practice, Some(&receiver), t0 + 35)
        .await
        .unwrap();
    assert_eq!(batch_recovered.settled.len(), 1);
    assert!(batch_recovered.failed.is_empty());
    assert!(matches!(
        batch_recovered.settled[0].standing,
        LinkCorrectionStanding::Settled { .. }
    ));
    assert_eq!(
        receiver.accepted_for_workspace(&ws_a).len() - baseline_a_reviews,
        1,
        "origin workspace must receive exactly 1 settlement review request across retries"
    );
    assert_eq!(
        receiver.accepted_for_workspace(&ws_b).len() - baseline_b_reviews,
        1,
        "target workspace must receive exactly 1 settlement review request"
    );

    // Re-running batch settlement again is a clean no-op:
    let batch_noop = store
        .settle_expired_link_corrections(&practice, Some(&receiver), t0 + 40)
        .await
        .unwrap();
    assert!(batch_noop.settled.is_empty());
    assert!(batch_noop.failed.is_empty());
    assert_eq!(
        receiver.accepted_for_workspace(&ws_a).len() - baseline_a_reviews,
        1
    );
    assert_eq!(
        receiver.accepted_for_workspace(&ws_b).len() - baseline_b_reviews,
        1
    );

    db.cleanup().await;
}

#[tokio::test]
async fn ac5_standing_move_withdraws_memory_understanding_concerns_follow_ups_withholds_reply_preserves_draft_identity_and_handled_messages_and_not_a_patient_closes_reply_and_concern()
 {
    let db = DisposableDb::create("ac5").await;
    let (store, auth) = connect_stores(&db.url).await;
    let practice = Uuid::new_v4().to_string();
    let ws_a = format!("pw-a-{}", Uuid::new_v4().simple());
    let ws_b = format!("pw-b-{}", Uuid::new_v4().simple());
    let ws_c = format!("pw-c-{}", Uuid::new_v4().simple());
    let t0 = 1_790_400_000;

    let (dr, _, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-ac5", true).await;
    for ws in [&ws_a, &ws_b, &ws_c] {
        auth.register_patient(&practice, ws).await.unwrap();
        auth.set_membership(&dr, ws, Membership::Allowed)
            .await
            .unwrap();
    }

    let skills = load_skills();
    let model_roles = roles();
    store
        .connect_spruce_channel(auth.as_ref(), &dr, 0, None, t0)
        .await
        .unwrap();
    let lease = store
        .acquire_spruce_reader_lease(&practice, "worker-ac5", 3_600, t0)
        .await
        .unwrap();
    let receiver = RecordingReviewReceiver::default();

    // =========================================================================
    // PART 1: Standing move `ws_a -> ws_b`
    // =========================================================================
    ingest_waiting_patient_message(
        &store,
        auth.as_ref(),
        &practice,
        &lease.lease_token,
        1,
        "msg-ac5-answered",
        "conv-ac5-ab",
        "+14155550501",
        "Elena Vance",
        "Can I get a copy of last week's lab slip?",
        &ws_a,
        &receiver,
        t0,
    )
    .await;
    store
        .link_conversation_to_patient(
            auth.as_ref(),
            &dr,
            LinkConversationCommand {
                command_key: "link-ac5-ab".into(),
                practice: practice.clone(),
                conversation_id: "conv-ac5-ab".into(),
                workspace: ws_a.clone(),
                sender_policy: SenderConfirmationPolicy::ConversationOnly,
                now_ts: t0 + 1,
            },
            Some(&receiver),
        )
        .await
        .unwrap();
    ingest_waiting_patient_message(
        &store,
        auth.as_ref(),
        &practice,
        &lease.lease_token,
        2,
        "msg-ac5-open",
        "conv-ac5-ab",
        "+14155550501",
        "Elena Vance",
        "My lithium tremor is worse this morning.",
        &ws_a,
        &receiver,
        t0 + 2,
    )
    .await;

    // Seed clean record `cand-clean-htn` + tainted record `cand-wrong-lithium` on `ws_a`, plus `cand-b-lab` on `ws_b`.
    let clean_a = chart_record_with_provenance(
        &ws_a,
        "clinic-ehr",
        "cand-clean-htn",
        "rev-1",
        "2026-11-01",
        "Hypertension Visit",
        "Hypertension controlled on amlodipine 5 mg daily.",
        json!({}),
    );
    store
        .record_chart(
            &ws_a,
            std::slice::from_ref(&clean_a),
            &source_pull("clinic-ehr", "Clinic EHR", "pt-a", "Clara Morales", t0 + 3),
        )
        .await
        .unwrap();

    // Save clean understanding `v1` on `ws_a`:
    let UnderstandingTriggerOutcome::Started { scope: u_v1, .. } = store
        .trigger_patient_understanding(&practice, &ws_a, &model_roles, &skills, auth.as_ref())
        .await
        .unwrap()
    else {
        panic!("expected understanding v1 Started");
    };
    run_understanding_save_job(
        &store,
        auth.as_ref(),
        &u_v1,
        vec![("cand-clean-htn", "rev-1")],
        make_save_args(
            0,
            "Hypertension controlled on amlodipine 5 mg daily.",
            "clinic-ehr",
            "cand-clean-htn",
            "rev-1",
            "Hypertension controlled on amlodipine 5 mg daily",
        ),
    )
    .await;

    // Record tainted `cand-wrong-lithium` and save contaminated understanding `v2`:
    let wrong_a = chart_record_with_provenance(
        &ws_a,
        "spruce",
        "cand-wrong-lithium",
        "rev-w1",
        "2026-11-12",
        "Inbound Lithium Message",
        "Patient reports worsening lithium tremor.",
        json!({"conversation_id": "conv-ac5-ab", "message_id": "msg-ac5-open"}),
    );
    store
        .record_chart(
            &ws_a,
            std::slice::from_ref(&wrong_a),
            &source_pull("spruce", "Spruce", "pt-a", "Clara Morales", t0 + 4),
        )
        .await
        .unwrap();

    let UnderstandingTriggerOutcome::Started { scope: u_v2, .. } = store
        .trigger_patient_understanding(&practice, &ws_a, &model_roles, &skills, auth.as_ref())
        .await
        .unwrap()
    else {
        panic!("expected understanding v2 Started");
    };
    run_understanding_save_job(
        &store,
        auth.as_ref(),
        &u_v2,
        vec![
            ("cand-clean-htn", "rev-1"),
            ("cand-wrong-lithium", "rev-w1"),
        ],
        make_save_args(
            1,
            "Contaminated overview mentioning lithium tremor.",
            "spruce",
            "cand-wrong-lithium",
            "rev-w1",
            "Worsening lithium tremor reported today",
        ),
    )
    .await;

    // Seed clean + tainted `patient_memory`, clean + tainted `Concern`, and clean + tainted `FollowUp` on `ws_a`:
    store
        .record_patient_memory(
            "mem-clean-ac5",
            &RecordMemoryCorrection {
                practice: practice.clone(),
                workspace: ws_a.clone(),
                id: PatientMemoryId::new("mem-clean-amlodipine").unwrap(),
                topic: "BP regimen".into(),
                statement: "Takes amlodipine 5 mg every morning.".into(),
                provenance: MemoryProvenance::PhysicianCorrection {
                    physician: dr.physician_id.clone(),
                    chat_id: "chat-clean".into(),
                    message_id: "msg-clean".into(),
                    work_id: "work-clean".into(),
                    attempt_id: "att-1".into(),
                },
            },
        )
        .await
        .unwrap();
    store
        .record_patient_memory(
            "mem-tainted-ac5",
            &RecordMemoryCorrection {
                practice: practice.clone(),
                workspace: ws_a.clone(),
                id: PatientMemoryId::new("mem-tainted-lithium").unwrap(),
                topic: "Lithium therapy".into(),
                statement: "Takes lithium carbonate with coarse hand tremor.".into(),
                provenance: MemoryProvenance::ClinicalSummary {
                    job_id: "job-tainted".into(),
                    work_id: "work-tainted-lithium".into(),
                    attempt_id: "att-1".into(),
                    records: vec![
                        ExactRecordReference::new("cand-wrong-lithium", "rev-w1").unwrap(),
                    ],
                },
            },
        )
        .await
        .unwrap();

    store
        .raise_concern(
            &CommandKey::new("cmd-conc-clean-ac5").unwrap(),
            &RaiseConcern {
                practice: practice.clone(),
                workspace: ws_a.clone(),
                id: ConcernId::new("concern-clean-htn").unwrap(),
                urgency: ConcernUrgency::Today,
                reason: "Annual BMP check on amlodipine.".into(),
                records: vec![ExactRecordReference::new("cand-clean-htn", "rev-1").unwrap()],
                source_work: SourceWork::new("patient_review", "work-clean-c", "att-1").unwrap(),
            },
        )
        .await
        .unwrap();
    store
        .raise_concern(
            &CommandKey::new("cmd-conc-tainted-ac5").unwrap(),
            &RaiseConcern {
                practice: practice.clone(),
                workspace: ws_a.clone(),
                id: ConcernId::new("concern-tainted-lithium").unwrap(),
                urgency: ConcernUrgency::Now,
                reason: "Possible acute lithium toxicity tremor.".into(),
                records: vec![ExactRecordReference::new("cand-wrong-lithium", "rev-w1").unwrap()],
                source_work: SourceWork::new("patient_review", "work-tainted-c", "att-1").unwrap(),
            },
        )
        .await
        .unwrap();

    create_job_follow_up(
        &store,
        &practice,
        &ws_a,
        SourceWork::new("patient_review", "work-clean-fu", "att-1").unwrap(),
        clean_a.clone(),
        "fu-clean-bp",
        "Recheck home blood pressure log in 30 days.",
        t0 + 30 * 86_400,
    )
    .await;
    create_job_follow_up(
        &store,
        &practice,
        &ws_a,
        SourceWork::new("patient_review", "work-tainted-fu", "att-1").unwrap(),
        wrong_a.clone(),
        "fu-tainted-lithium",
        "Verify stat serum lithium trough level tomorrow.",
        t0 + 86_400,
    )
    .await;

    // Seed `ReplyConversation` on `ws_a` with:
    // - `msg-ac5-answered` handled (`Answered` by `prac-lab-slip`)
    // - `msg-ac5-open` still `Open` with `Ready` proposal (`reply_id == 1`, `version == 1`)
    // - Physician private draft edited on `reply_id == 1`!
    store
        .link_reply_conversation(
            &practice,
            "conv-ac5-ab",
            ConversationLink::LinkedPatient {
                workspace: ws_a.clone(),
            },
        )
        .await
        .unwrap();
    for (mid, txt, ts) in [
        (
            "msg-ac5-answered",
            "Can I get a copy of last week's lab slip?",
            t0,
        ),
        (
            "msg-ac5-open",
            "My lithium tremor is worse this morning.",
            t0 + 2,
        ),
    ] {
        let out = store
            .record_linked_patient_message(&RecordPatientMessage {
                practice: practice.clone(),
                conversation_id: "conv-ac5-ab".into(),
                message_id: mid.into(),
                sender: MessageSender::Patient,
                text: txt.into(),
                sent_at: ts,
            })
            .await
            .unwrap();
        assert!(matches!(out, MessageIntakeOutcome::Recorded { .. }));
    }
    store
        .observe_practice_message(&ObservePracticeMessage {
            practice: practice.clone(),
            conversation_id: "conv-ac5-ab".into(),
            practice_message_id: "prac-lab-slip".into(),
            kind: PracticeMessageKind::PracticeSent,
            text: "Sent your lab slip to the portal.".into(),
            sent_at: t0 + 3,
        })
        .await
        .unwrap();
    store
        .start_or_read_reply_review(&practice, "conv-ac5-ab", "rev-ab-1", "run-ab-1")
        .await
        .unwrap();
    let ReplyReviewOutcome::Applied(applied_ab) = store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: "conv-ac5-ab".into(),
            review_id: "rev-ab-1".into(),
            run_id: "run-ab-1".into(),
            completion_key: "comp-ab-1".into(),
            answered_closures: vec![AnsweredMessageClosure {
                practice_message_id: "prac-lab-slip".into(),
                answered_message_ids: BTreeSet::from(["msg-ac5-answered".to_owned()]),
            }],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "Wrong-patient proposal on Clara's chart.".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Reviewed inbound message.".into(),
                    limits: vec![],
                    cited_records: vec![
                        ExactRecordReference::new("cand-wrong-lithium", "rev-w1").unwrap(),
                    ],
                },
                cited_records: vec![
                    ExactRecordReference::new("cand-wrong-lithium", "rev-w1").unwrap(),
                ],
            }),
            raised_concern_ids: vec![ConcernId::new("concern-tainted-lithium").unwrap()],
        })
        .await
        .unwrap()
    else {
        panic!("expected Applied reply review");
    };
    assert_eq!(applied_ab.proposal.as_ref().unwrap().reply_id, 1);

    let access_a = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &ws_a,
        session_hash: &dr_hash,
    };
    let saved_draft = store
        .edit_reply_proposal_draft(
            &access_a,
            "conv-ac5-ab",
            ExpectedDraftBase::ProposalVersion(1),
            "Dr. AC5 private draft on reply_id 1.",
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert_eq!(saved_draft.reply_id, 1);

    // Seed Patient B (`ws_b`) chart so the post-move review on `ws_b` can cite `ws_b`'s real lithium record!
    store
        .record_chart(
            &ws_b,
            &[chart_record_with_provenance(
                &ws_b,
                "clinic-ehr",
                "cand-b-lithium",
                "rev-b1",
                "2026-11-11",
                "Lithium Level",
                "Elena Vance serum lithium 1.4 mmol/L.",
                json!({}),
            )],
            &source_pull("clinic-ehr", "Clinic EHR", "pt-b", "Elena Vance", t0 + 5),
        )
        .await
        .unwrap();

    let baseline_a = receiver.accepted_for_workspace(&ws_a).len();
    let baseline_b = receiver.accepted_for_workspace(&ws_b).len();

    // Move `conv-ac5-ab` from `ws_a -> ws_b` and settle after `Undo` expires:
    store
        .correct_patient_link(
            auth.as_ref(),
            &dr,
            CorrectPatientLinkCommand {
                command_key: "move-ac5-ab".into(),
                practice: practice.clone(),
                conversation_id: "conv-ac5-ab".into(),
                destination: CorrectionDestination::Patient {
                    workspace: ws_b.clone(),
                },
                undo_window_seconds: 10,
                now_ts: t0 + 10,
            },
            Some(&receiver),
        )
        .await
        .unwrap();

    let settled_ab = store
        .settle_expired_link_corrections(&practice, Some(&receiver), t0 + 25)
        .await
        .unwrap();
    assert_eq!(settled_ab.settled.len(), 1);
    assert!(settled_ab.failed.is_empty());

    // 1. Verify ONE review request recorded for `ws_a` and ONE for `ws_b`:
    let new_a_reviews = receiver.accepted_for_workspace(&ws_a);
    let new_b_reviews = receiver.accepted_for_workspace(&ws_b);
    assert_eq!(new_a_reviews.len() - baseline_a, 1);
    assert_eq!(new_b_reviews.len() - baseline_b, 1);
    assert_eq!(
        new_a_reviews.last().unwrap().reason,
        ReviewReason::NewRecords
    );
    assert_eq!(
        new_b_reviews.last().unwrap().reason,
        ReviewReason::NewRecords
    );

    // 2. Verify memory on `ws_a`: `mem-tainted-lithium` is `EnteredInError` (hidden even from `ActiveAndRetired`), `mem-clean-amlodipine` stays `Active`!
    let active_mem_a = store
        .patient_memories(&practice, &ws_a, MemorySelection::ActiveAndRetired)
        .await
        .unwrap();
    assert_eq!(active_mem_a.len(), 1);
    assert_eq!(active_mem_a[0].id.as_str(), "mem-clean-amlodipine");
    assert_eq!(active_mem_a[0].standing, MemoryStanding::Active);
    let audit_mem_a = store
        .patient_memories(&practice, &ws_a, MemorySelection::AuditAll)
        .await
        .unwrap();
    let tainted_mem = audit_mem_a
        .iter()
        .find(|m| m.id.as_str() == "mem-tainted-lithium")
        .expect("entered-in-error audit row retained");
    assert!(matches!(
        tainted_mem.standing,
        MemoryStanding::EnteredInError { .. }
    ));

    // 3. Verify patient understanding on `ws_a`: contaminated `v2` withdrawn (`EnteredInError`), active understanding rolled back to clean `v1`!
    let u_after = store
        .patient_understanding_for_practice(&practice, &ws_a, auth.as_ref())
        .await
        .unwrap()
        .expect("rolled back to clean understanding v1");
    assert_eq!(u_after.revision, 1);
    assert!(
        !serde_json::to_string(&u_after.summary)
            .unwrap()
            .to_ascii_lowercase()
            .contains("lithium")
    );
    assert!(
        !u_after
            .operating_index
            .working_overview
            .to_ascii_lowercase()
            .contains("lithium")
    );
    assert!(
        !serde_json::to_string(&u_after)
            .unwrap()
            .to_ascii_lowercase()
            .contains("tremor")
    );
    let u_audits = store
        .patient_understanding_revision_audits(&practice, &ws_a, auth.as_ref())
        .await
        .unwrap();
    assert_eq!(u_audits.len(), 2);
    assert_eq!(u_audits[0].standing, UnderstandingStanding::Active);
    assert!(matches!(
        u_audits[1].standing,
        UnderstandingStanding::EnteredInError { .. }
    ));

    // 4. Verify concerns on `ws_a`: `concern-tainted-lithium` withdrawn, `concern-clean-htn` stays `Open`!
    let open_concerns_a = store
        .authorized_open_concerns(
            auth.as_ref(),
            &dr,
            &ws_a,
            &dr_hash,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert_eq!(open_concerns_a.len(), 1);
    assert_eq!(open_concerns_a[0].id.as_str(), "concern-clean-htn");
    let all_concerns_a = store
        .workspace_concerns(&access_a, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    let tainted_c = all_concerns_a
        .iter()
        .find(|c| c.id.as_str() == "concern-tainted-lithium")
        .unwrap();
    assert!(matches!(
        tainted_c.standing,
        ConcernStanding::Withdrawn { .. }
    ));

    // 5. Verify follow-ups on `ws_a`: `fu-tainted-lithium` is `WithdrawnWrongPatient` and omitted from `physician_follow_ups`, while `fu-clean-bp` stays `Active`!
    let visible_fus_a = store
        .physician_follow_ups(&access_a, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert_eq!(visible_fus_a.len(), 1);
    assert_eq!(visible_fus_a[0].id.as_str(), "fu-clean-bp");
    assert!(matches!(
        visible_fus_a[0].standing,
        FollowUpStanding::Active { .. }
    ));

    // 6. Verify reply standing on `ws_b`, preserved `Answered` message, and preserved `reply_id == 1` draft identity:
    let conv_ab_after = store
        .reply_conversation(&practice, "conv-ac5-ab")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        conv_ab_after.link,
        ConversationLink::LinkedPatient {
            workspace: ws_b.clone(),
        }
    );
    // Handled message stays `Answered`!
    let answered_msg = conv_ab_after
        .patient_messages
        .iter()
        .find(|m| m.message_id == "msg-ac5-answered")
        .unwrap();
    assert!(matches!(
        answered_msg.standing,
        PatientMessageStanding::Answered { ref practice_message_id, .. } if practice_message_id == "prac-lab-slip"
    ));
    // Open message stays `Open`, and reply is `Withheld` under new `reply_id == 2`!
    assert!(matches!(
        &conv_ab_after.reply_standing,
        ReplyStanding::Withheld {
            reply_id: 2,
            workspace,
            covered_message_ids,
            ..
        } if workspace == &ws_b && covered_message_ids == &BTreeSet::from(["msg-ac5-open".to_owned()])
    ));

    let access_b = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &ws_b,
        session_hash: &dr_hash,
    };
    let card_b_withheld = store
        .reply_card(&access_b, "conv-ac5-ab", &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .expect("withheld card on ws_b");
    assert_eq!(card_b_withheld.reply_id, 2);
    assert_eq!(card_b_withheld.displayed_text, DisplayedReplyText::Withheld);
    assert_eq!(
        card_b_withheld.draft_standing,
        PhysicianDraftStanding::NoDraft,
        "pre-move draft bound to reply_id 1 must keep its reply_id 1 identity and never attach to reply_id 2"
    );
    let draft_scope_1 = proposal_draft_scope(&practice, &dr.physician_id, &ws_a, "conv-ac5-ab", 1);
    assert_eq!(draft_scope_1.chat, "reply-proposal:conv-ac5-ab:1");
    let saved_draft_1 = store
        .draft(&draft_scope_1)
        .await
        .unwrap()
        .expect("draft kept its reply_id == 1 identity");
    assert_eq!(saved_draft_1.text, "Dr. AC5 private draft on reply_id 1.");

    // Verify the reply remains withheld until the new review on `ws_b` writes it again (or closes it):
    store
        .start_or_read_reply_review(&practice, "conv-ac5-ab", "rev-b-fresh", "run-b-fresh")
        .await
        .unwrap();
    let still_withheld = store
        .reply_card(&access_b, "conv-ac5-ab", &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(still_withheld.displayed_text, DisplayedReplyText::Withheld);

    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: "conv-ac5-ab".into(),
            review_id: "rev-b-fresh".into(),
            run_id: "run-b-fresh".into(),
            completion_key: "comp-b-fresh".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "Elena, please hold this morning's lithium dose and come in for a repeat level today.".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Elena Vance serum lithium 1.4 mmol/L with worsening tremor.".into(),
                    limits: vec![],
                    cited_records: vec![
                        ExactRecordReference::new("cand-b-lithium", "rev-b1").unwrap(),
                    ],
                },
                cited_records: vec![
                    ExactRecordReference::new("cand-b-lithium", "rev-b1").unwrap(),
                ],
            }),
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();

    let card_b_ready = store
        .reply_card(&access_b, "conv-ac5-ab", &mut PostgresWorkspaceAuthority)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        card_b_ready.displayed_text,
        DisplayedReplyText::CodaProposal { reply_id: 2, version: 2, ref text }
            if text.contains("Elena, please hold this morning's lithium dose")
    ));

    // =========================================================================
    // PART 2: Standing move `ws_c -> NotAPatient` closes the reply and concern!
    // =========================================================================
    store
        .record_chart(
            &ws_c,
            &[chart_record_with_provenance(
                &ws_c,
                "clinic-ehr",
                "cand-c-base",
                "rev-c1",
                "2026-11-10",
                "Baseline Lab",
                "Routine metabolic panel normal.",
                json!({}),
            )],
            &source_pull("clinic-ehr", "Clinic EHR", "pt-c", "danny Pink", t0 + 50),
        )
        .await
        .unwrap();

    ingest_waiting_patient_message(
        &store,
        auth.as_ref(),
        &practice,
        &lease.lease_token,
        3,
        "msg-ac5-vendor",
        "conv-ac5-notpt",
        "+14155550599",
        "Office Copier Vendor",
        "Urgent toner invoice reminder.",
        &ws_c,
        &receiver,
        t0 + 51,
    )
    .await;
    store
        .link_conversation_to_patient(
            auth.as_ref(),
            &dr,
            LinkConversationCommand {
                command_key: "link-ac5-c".into(),
                practice: practice.clone(),
                conversation_id: "conv-ac5-notpt".into(),
                workspace: ws_c.clone(),
                sender_policy: SenderConfirmationPolicy::ConversationOnly,
                now_ts: t0 + 52,
            },
            Some(&receiver),
        )
        .await
        .unwrap();

    // Raise a concern associated with `conv-ac5-notpt`'s reply review (citing `cand-c-base` so we also test `act_on_conversation_move` concern closure!):
    store
        .raise_concern(
            &CommandKey::new("cmd-conc-c-notpt").unwrap(),
            &RaiseConcern {
                practice: practice.clone(),
                workspace: ws_c.clone(),
                id: ConcernId::new("concern-from-notpt-conv").unwrap(),
                urgency: ConcernUrgency::Today,
                reason: "Review inbound message on Danny's chart.".into(),
                records: vec![ExactRecordReference::new("cand-c-base", "rev-c1").unwrap()],
                source_work: SourceWork::new("reply_review", "work-c-reply", "att-1").unwrap(),
            },
        )
        .await
        .unwrap();

    store
        .link_reply_conversation(
            &practice,
            "conv-ac5-notpt",
            ConversationLink::LinkedPatient {
                workspace: ws_c.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: "conv-ac5-notpt".into(),
            message_id: "msg-ac5-vendor".into(),
            sender: MessageSender::Patient,
            text: "Urgent toner invoice reminder.".into(),
            sent_at: t0 + 52,
        })
        .await
        .unwrap();
    store
        .start_or_read_reply_review(&practice, "conv-ac5-notpt", "rev-c-1", "run-c-1")
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: "conv-ac5-notpt".into(),
            review_id: "rev-c-1".into(),
            run_id: "run-c-1".into(),
            completion_key: "comp-c-1".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "Could you clarify your question?".into(),
                basis: ReplyProposalBasisInput {
                    summary: "Inbound message.".into(),
                    limits: vec![],
                    cited_records: vec![
                        ExactRecordReference::new("cand-c-base", "rev-c1").unwrap(),
                    ],
                },
                cited_records: vec![ExactRecordReference::new("cand-c-base", "rev-c1").unwrap()],
            }),
            raised_concern_ids: vec![ConcernId::new("concern-from-notpt-conv").unwrap()],
        })
        .await
        .unwrap();

    let baseline_c = receiver.accepted_for_workspace(&ws_c).len();
    let baseline_total = receiver.accepted_requests().len();

    store
        .correct_patient_link(
            auth.as_ref(),
            &dr,
            CorrectPatientLinkCommand {
                command_key: "move-ac5-c-to-notpt".into(),
                practice: practice.clone(),
                conversation_id: "conv-ac5-notpt".into(),
                destination: CorrectionDestination::NotAPatient,
                undo_window_seconds: 10,
                now_ts: t0 + 60,
            },
            Some(&receiver),
        )
        .await
        .unwrap();

    let settled_notpt = store
        .settle_expired_link_corrections(&practice, Some(&receiver), t0 + 75)
        .await
        .unwrap();
    assert_eq!(settled_notpt.settled.len(), 1);
    assert!(settled_notpt.failed.is_empty());

    // Verify 1 review request for `ws_c` and 0 for `NotAPatient` (`total` increased by 1):
    assert_eq!(receiver.accepted_for_workspace(&ws_c).len() - baseline_c, 1);
    assert_eq!(receiver.accepted_requests().len() - baseline_total, 1);

    // Verify reply closed (`ReplyStanding::None`, message `MovedToNotPatient`) and concern closed on `ws_c`:
    let conv_notpt_after = store
        .reply_conversation(&practice, "conv-ac5-notpt")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(conv_notpt_after.link, ConversationLink::NotAPatient);
    assert_eq!(conv_notpt_after.reply_standing, ReplyStanding::None);
    assert!(matches!(
        conv_notpt_after.patient_messages[0].standing,
        PatientMessageStanding::MovedToNotPatient { .. }
    ));

    let open_concerns_c = store
        .authorized_open_concerns(
            auth.as_ref(),
            &dr,
            &ws_c,
            &dr_hash,
            &mut PostgresWorkspaceAuthority,
        )
        .await
        .unwrap();
    assert!(
        open_concerns_c.is_empty(),
        "moving to not a patient must close associated concerns: {open_concerns_c:?}"
    );
    let spruce_msg_notpt = store
        .spruce_message(&practice, "msg-ac5-vendor")
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        spruce_msg_notpt.standing,
        MessageStanding::NotAPatient { .. }
    ));

    db.cleanup().await;
}

/// Regression suite covering unified `spruce_matching` move/Undo/settlement enforcement,
/// in-flight chat `finish_chat_authorized` refusal on withheld candidates,
/// `READ_DOCUMENT_PAGE_TOOL` turn redaction + reopen citation withholding,
/// `PatientSummaryView` `origin`/`operating_index` withholding, `PatientStartingContext`
/// `source_work` concern withholding, and `LinkCorrectionError::Concern` `#[source]` preservation.
#[tokio::test]
async fn round1_unified_spruce_settlement_inflight_chat_doc_page_and_summary_concern_withholding() {
    use local_product::runtime::concerns::{ConcernError, ConcernInputError};
    use local_product::runtime::spruce_matching::{
        MoveDestination, MoveSpruceConversationCommand, SpruceMatchingError,
    };

    // 1. Verify `LinkCorrectionError::Concern` and `RemovalStepFailed` preserve `#[source]`.
    let concern_err = LinkCorrectionError::from(ConcernError::InvalidInput(
        ConcernInputError::InvalidField("concern_id"),
    ));
    assert!(
        concern_err.source().is_some(),
        "LinkCorrectionError::Concern must preserve #[source]"
    );
    let removal_err = LinkCorrectionError::RemovalStepFailed {
        correction_id: "corr:1".into(),
        conversation_id: "conv-1".into(),
        step: LinkCorrectionStep::WithdrawOriginEffects,
        reason: "step failed".into(),
        source: Some(Box::new(LinkCorrectionError::from(
            ConcernError::InvalidInput(ConcernInputError::InvalidField("concern_id")),
        ))),
    };
    assert!(
        removal_err.source().is_some(),
        "LinkCorrectionError::RemovalStepFailed must preserve #[source] when caused by a store error"
    );

    let db = DisposableDb::create("r1_unified").await;
    let (store, auth) = connect_stores(&db.url).await;
    let practice = format!("prac-r1-{}", Uuid::new_v4().simple());
    let ws_a = format!("pw-r1-a-{}", Uuid::new_v4().simple());
    let ws_b = format!("pw-r1-b-{}", Uuid::new_v4().simple());
    let ws_c = format!("pw-r1-c-{}", Uuid::new_v4().simple());
    let t0 = 1_790_500_000_i64;

    let (dr, _, dr_hash) = invite_and_sign_in(&auth, &practice, "dr-r1", true).await;
    for ws in [&ws_a, &ws_b, &ws_c] {
        auth.register_patient(&practice, ws).await.unwrap();
        auth.set_membership(&dr, ws, Membership::Allowed)
            .await
            .unwrap();
    }

    let skills = load_skills();
    let model_roles = roles();
    store
        .connect_spruce_channel(auth.as_ref(), &dr, 0, None, t0)
        .await
        .unwrap();
    let lease = store
        .acquire_spruce_reader_lease(&practice, "worker-r1", 3_600, t0)
        .await
        .unwrap();
    let receiver = RecordingReviewReceiver::default();

    let wrong_secret = "WRONG-R1-SECRET: Elena troponin 0.42 ng/mL at St. Jude.";
    let wrong_doc_secret = "WRONG-R1-PDF-PAGE: Elena outside cardiology discharge fax page 1.";
    let clean_text = "Clara Morales baseline A1c 5.6% and lisinopril 10 mg daily.";

    ingest_waiting_patient_message(
        &store,
        auth.as_ref(),
        &practice,
        &lease.lease_token,
        1,
        "msg-r1-1",
        "conv-r1",
        "+14155550901",
        "Elena Vance",
        wrong_secret,
        &ws_a,
        &receiver,
        t0,
    )
    .await;

    store
        .link_conversation_to_patient(
            auth.as_ref(),
            &dr,
            LinkConversationCommand {
                command_key: "link-r1-a".into(),
                practice: practice.clone(),
                conversation_id: "conv-r1".into(),
                workspace: ws_a.clone(),
                sender_policy: SenderConfirmationPolicy::ConversationOnly,
                now_ts: t0 + 1,
            },
            Some(&receiver),
        )
        .await
        .unwrap();

    let clean_rec = chart_record_with_provenance(
        &ws_a,
        "clinic-ehr",
        "cand-r1-clean",
        "rev-1",
        "2026-11-10",
        "Primary Care Note",
        clean_text,
        json!({}),
    );
    store
        .record_chart(
            &ws_a,
            std::slice::from_ref(&clean_rec),
            &source_pull("clinic-ehr", "Clinic EHR", "pt-a", "Clara Morales", t0 + 2),
        )
        .await
        .unwrap();

    // Seed clean Revision 1 understanding on `ws_a` first:
    let UnderstandingTriggerOutcome::Started { scope: u_v1, .. } = store
        .trigger_patient_understanding(&practice, &ws_a, &model_roles, &skills, auth.as_ref())
        .await
        .unwrap()
    else {
        panic!("expected understanding v1 Started");
    };
    run_understanding_save_job(
        &store,
        auth.as_ref(),
        &u_v1,
        vec![("cand-r1-clean", "rev-1")],
        make_save_args(
            0,
            clean_text,
            "clinic-ehr",
            "cand-r1-clean",
            "rev-1",
            clean_text,
        ),
    )
    .await;

    // Now record wrong-patient Spruce message + PDF attachment on `ws_a`:
    let wrong_rec = chart_record_with_provenance(
        &ws_a,
        "spruce",
        "cand-r1-wrong",
        "rev-w1",
        "2026-11-12",
        "Inbound Spruce Message",
        wrong_secret,
        json!({
            "conversation_id": "conv-r1",
            "message_id": "msg-r1-1",
        }),
    );
    let wrong_pdf_rec = SourceRevision {
        workspace: ws_a.clone(),
        candidate_id: "cand-r1-pdf-source".into(),
        source_id: "spruce".into(),
        record_type: "DiagnosticReport".into(),
        revision: "v1".into(),
        dates: vec![RecordDate {
            name: "recorded".into(),
            value: "2026-11-12".into(),
        }],
        content: "Cardiology fax attachment".into(),
        sha256: sha256_hex(b"Cardiology fax attachment"),
        readable_parts: vec![ReadablePart {
            label: "page:doc-r1-fax:v1:p1".into(),
            text: wrong_doc_secret.into(),
        }],
        provenance: json!({
            "title": "Cardiology Fax",
            "conversation_id": "conv-r1",
            "message_id": "msg-r1-1",
            "document_id": "doc-r1-fax",
            "document_version": 1,
            "page_count": 1,
            "pages": [{
                "page_number": 1,
                "status": "ok",
                "text": wrong_doc_secret,
                "extraction_method": "pdf_text"
            }]
        }),
    };
    store
        .record_chart(
            &ws_a,
            &[wrong_rec.clone(), wrong_pdf_rec.clone()],
            &source_pull("spruce", "Spruce", "pt-a", "Clara Morales", t0 + 3),
        )
        .await
        .unwrap();

    // Seed tainted Revision 2 understanding whose `summary.claims` ONLY cites `cand-r1-clean`
    // (omitting `cand-r1-wrong` from `summary.citations`!) while its job read `cand-r1-wrong`
    // (`origin` tainted) and placed `cand-r1-wrong` in `operating_index.anchors`!
    let UnderstandingTriggerOutcome::Started { scope: u_v2, .. } = store
        .trigger_patient_understanding(&practice, &ws_a, &model_roles, &skills, auth.as_ref())
        .await
        .unwrap()
    else {
        panic!("expected understanding v2 Started");
    };
    let mut origin_only_tainted_save = make_save_args(
        1,
        wrong_secret,
        "spruce",
        "cand-r1-wrong",
        "rev-w1",
        &format!("Narrative tainted by {wrong_secret}"),
    );
    origin_only_tainted_save.claims = vec![SummaryClaimInput {
        text: format!("Narrative tainted by {wrong_secret}"),
        citations: vec![ExactRecordReference::new("cand-r1-clean", "rev-1").unwrap()],
    }];
    run_understanding_save_job(
        &store,
        auth.as_ref(),
        &u_v2,
        vec![("cand-r1-clean", "rev-1"), ("cand-r1-wrong", "rev-w1")],
        origin_only_tainted_save,
    )
    .await;

    // Seed a chat turn that read `doc-r1-fax` via `READ_DOCUMENT_PAGE_TOOL` and quoted `wrong_doc_secret` with `[1]`.
    let doc_chat_scope = Scope {
        practice: practice.clone(),
        physician: dr.physician_id.clone(),
        workspace: ws_a.clone(),
        chat: "chat-r1-doc-page".into(),
    };
    let doc_work_id: String;
    {
        struct DocPageChatProvider {
            step: usize,
            answer: String,
        }
        impl Provider for DocPageChatProvider {
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
                            id: "call-doc-page".into(),
                            name: READ_DOCUMENT_PAGE_TOOL.into(),
                            arguments: json!({
                                "document_id": "doc-r1-fax",
                                "version": 1,
                                "page_number": 1
                            })
                            .to_string(),
                        })],
                        receipt: json!({"step": 1}),
                        usage: Default::default(),
                    }),
                    2 => Ok(Turn {
                        items: vec![Item::assistant(self.answer.clone())],
                        receipt: json!({"step": 2}),
                        usage: Default::default(),
                    }),
                    other => panic!("unexpected doc page step {other}"),
                }
            }
        }
        let starting = store
            .patient_starting_context(&doc_chat_scope)
            .await
            .unwrap();
        let recipe = ChatAssignment::for_patient(doc_chat_scope.clone(), starting)
            .recipe(&model_roles, &skills);
        store
            .accept_authorized(
                &doc_chat_scope,
                &Input {
                    key: "input-doc-page-1".into(),
                    text: "What was on page 1 of the fax?".into(),
                    timezone: "America/Los_Angeles".into(),
                    scenario: None,
                },
                dr_hash.clone(),
                &skills,
                &recipe,
            )
            .await
            .unwrap();
        let claim = store.claim(&doc_chat_scope, 60).await.unwrap().unwrap();
        let claim_attempt = claim.attempt.clone();
        let mut host = PhysicianHost {
            store: store.clone(),
            claim,
            authority: PostgresWorkspaceAuthority,
        };
        let mut provider = DocPageChatProvider {
            step: 0,
            answer: format!("Fax page 1 reported {wrong_doc_secret} [1]."),
        };
        host.run(
            &mut provider,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();
        let conv_state = store.read(&doc_chat_scope).await.unwrap().unwrap();
        doc_work_id = conv_state.work;
        // Raise an open concern whose `records` cites ONLY `cand-r1-clean` while its `source_work` is
        // `doc_work_id` (tainted by reading the wrong-patient fax!) — testing `concern_is_withheld`
        // in `patient_starting_context` and `open_concerns`!
        store
            .raise_concern(
                &CommandKey::new("cmd-r1-concern-work-tainted").unwrap(),
                &RaiseConcern {
                    practice: practice.clone(),
                    workspace: ws_a.clone(),
                    id: ConcernId::new("concern-r1-work-tainted").unwrap(),
                    urgency: ConcernUrgency::Now,
                    reason: format!("Work-tainted concern mentioning {wrong_doc_secret}"),
                    records: vec![ExactRecordReference::new("cand-r1-clean", "rev-1").unwrap()],
                    source_work: SourceWork::new("physician_chat", &doc_work_id, &claim_attempt)
                        .unwrap(),
                },
            )
            .await
            .unwrap();
    }

    // Seed wrong-patient memory, follow-up, and reply proposal on `ws_a` for `conv-r1`.
    let access_a = WorkspaceAccess {
        practice: &practice,
        physician: &dr.physician_id,
        workspace: &ws_a,
        session_hash: &dr_hash,
    };
    store
        .record_patient_memory(
            "cmd-r1-mem",
            &RecordMemoryCorrection {
                practice: practice.clone(),
                workspace: ws_a.clone(),
                id: PatientMemoryId::new("mem-r1-wrong").unwrap(),
                topic: "Outside cardiology".into(),
                statement: wrong_secret.into(),
                provenance: MemoryProvenance::ClinicalSummary {
                    job_id: "job-r1-mem".into(),
                    work_id: "work-r1-mem".into(),
                    attempt_id: "att-1".into(),
                    records: vec![ExactRecordReference::new("cand-r1-wrong", "rev-w1").unwrap()],
                },
            },
        )
        .await
        .unwrap();
    create_job_follow_up(
        &store,
        &practice,
        &ws_a,
        SourceWork::new("patient_review", "work-r1-fu", "att-1").unwrap(),
        wrong_rec.clone(),
        "fu-r1-wrong",
        wrong_secret,
        t0 + 3_600,
    )
    .await;
    store
        .link_reply_conversation(
            &practice,
            "conv-r1",
            ConversationLink::LinkedPatient {
                workspace: ws_a.clone(),
            },
        )
        .await
        .unwrap();
    store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: practice.clone(),
            conversation_id: "conv-r1".into(),
            message_id: "msg-r1-1".into(),
            sender: MessageSender::Patient,
            text: wrong_secret.into(),
            sent_at: t0 + 4,
        })
        .await
        .unwrap();
    store
        .start_or_read_reply_review(&practice, "conv-r1", "rev-r1-1", "run-r1-1")
        .await
        .unwrap();
    store
        .complete_reply_review(&CompleteReplyReview {
            practice: practice.clone(),
            conversation_id: "conv-r1".into(),
            review_id: "rev-r1-1".into(),
            run_id: "run-r1-1".into(),
            completion_key: "comp-r1-1".into(),
            answered_closures: vec![],
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: format!("Reply citing {wrong_secret}"),
                basis: ReplyProposalBasisInput {
                    summary: "Inbound review".into(),
                    limits: vec![],
                    cited_records: vec![
                        ExactRecordReference::new("cand-r1-wrong", "rev-w1").unwrap(),
                    ],
                },
                cited_records: vec![ExactRecordReference::new("cand-r1-wrong", "rev-w1").unwrap()],
            }),
            raised_concern_ids: vec![],
        })
        .await
        .unwrap();

    // 2. Run a real `PhysicianHost` chat turn on `ws_a` where:
    //    - Step 1 reads `cand-r1-wrong` BEFORE `move_spruce_conversation`.
    //    - Step 2 moves `conv-r1` (`ws_a -> ws_b`) via `Store::move_spruce_conversation` mid-run,
    //      then tries to publish an assistant answer quoting `wrong_secret`.
    //    - `PhysicianHost` / `finish_chat_authorized` REFUSES publication (`ChatFinish::Refused`),
    //      injecting `ChatChartArrivalNote` into Step 3!
    //    - Step 3 verifies the refusal note (`model_text::assert_clean_model_text`), confirms nothing
    //      tainted was published to `Store::read`, re-reads `cand-r1-wrong` (`RecordLimitKind::Unavailable`),
    //      and Step 4 publishes a clean answer!
    let inflight_scope = Scope {
        practice: practice.clone(),
        physician: dr.physician_id.clone(),
        workspace: ws_a.clone(),
        chat: "chat-r1-inflight".into(),
    };
    let starting_pre = store
        .patient_starting_context(&inflight_scope)
        .await
        .unwrap();
    let inflight_recipe = ChatAssignment::for_patient(inflight_scope.clone(), starting_pre)
        .recipe(&model_roles, &skills);
    store
        .accept_authorized(
            &inflight_scope,
            &Input {
                key: "input-inflight-1".into(),
                text: "Summarize the inbound message.".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            dr_hash.clone(),
            &skills,
            &inflight_recipe,
        )
        .await
        .unwrap();
    let inflight_claim = store.claim(&inflight_scope, 60).await.unwrap().unwrap();

    struct MidRunWithholdProvider<'a> {
        step: usize,
        store: &'a Store,
        auth: &'a PostgresAuthStore,
        dr: &'a PhysicianIdentity,
        receiver: &'a RecordingReviewReceiver,
        inflight_scope: &'a Scope,
        ws_b: String,
        wrong_secret: String,
        t0: i64,
    }

    impl Provider for MidRunWithholdProvider<'_> {
        async fn exchange(
            &mut self,
            request: &Request,
            _cancel: &engine::Cancellation,
            _limits: &engine::RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.step += 1;
            match self.step {
                1 => Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "call-inflight-read-1".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{
                                "candidate_id": "cand-r1-wrong",
                                "revision": "rev-w1",
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
                    // Confirm Step 1 read `wrong_secret` while still linked to `ws_a`:
                    let read_out = request
                        .items
                        .iter()
                        .find_map(|i| match i {
                            Item::MaterializedResult { call_id, output }
                                if call_id == "call-inflight-read-1" =>
                            {
                                Some(output.clone())
                            }
                            _ => None,
                        })
                        .expect("step 1 tool output");
                    assert!(read_out.contains(&self.wrong_secret));

                    // Physician moves `conv-r1` (`ws_a -> ws_b`) via `move_spruce_conversation` MID-RUN!
                    self.store
                        .move_spruce_conversation(
                            self.auth,
                            self.dr,
                            MoveSpruceConversationCommand {
                                conversation_id: "conv-r1".into(),
                                destination: MoveDestination::AnotherPatient {
                                    workspace: self.ws_b.clone(),
                                },
                                undo_window_seconds: 10,
                                command_key: "spruce-move-r1-a-to-b".into(),
                            },
                            Some(self.receiver),
                            self.t0 + 10,
                        )
                        .await
                        .expect("mid-run move_spruce_conversation");

                    // Attempt to publish wrong-patient text — `finish_chat_authorized` MUST refuse!
                    Ok(Turn {
                        items: vec![Item::assistant(format!(
                            "Inbound message says {} [1].",
                            self.wrong_secret
                        ))],
                        receipt: json!({"step": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    // Verify `finish_chat_authorized` refused publication and injected `ChatChartArrivalNote`:
                    let mid_conv = self.store.read(self.inflight_scope).await.unwrap().unwrap();
                    assert!(
                        mid_conv
                            .messages
                            .iter()
                            .all(|m| !m.text.contains("WRONG-R1-SECRET")),
                        "refused in-flight turn must NOT publish wrong-patient text: {:?}",
                        mid_conv.messages
                    );
                    let arrival_note_text = request
                        .items
                        .iter()
                        .rev()
                        .find_map(|i| match i {
                            Item::FinishRefused(refusal) => Some(refusal.reason().to_owned()),
                            Item::Message { role, text } if role == "developer" => {
                                let env: Value = serde_json::from_str(text).ok()?;
                                let err_str = env.get("error")?.as_str()?;
                                err_str
                                    .contains("must_read_again")
                                    .then(|| err_str.to_owned())
                            }
                            _ => None,
                        })
                        .expect("expected ChatChartArrivalNote refusal message in step 3 request");
                    assert!(arrival_note_text.contains("cand-r1-wrong"));
                    model_text::assert_clean_model_text(&arrival_note_text);

                    // Confirm withdrawal via `READ_TOOL` on `cand-r1-wrong`:
                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "call-inflight-confirm-withdrawn".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{
                                    "candidate_id": "cand-r1-wrong",
                                    "revision": "rev-w1",
                                    "cursor": null
                                }],
                                "max_result_bytes": 16_384
                            })
                            .to_string(),
                        })],
                        receipt: json!({"step": 3}),
                        usage: Default::default(),
                    })
                }
                4 => {
                    let confirm_out = request
                        .items
                        .iter()
                        .find_map(|i| match i {
                            Item::MaterializedResult { call_id, output }
                                if call_id == "call-inflight-confirm-withdrawn" =>
                            {
                                Some(output.clone())
                            }
                            _ => None,
                        })
                        .expect("step 3 confirm output");
                    assert!(confirm_out.contains("\"kind\":\"unavailable\""));
                    assert!(!confirm_out.contains("WRONG-R1-SECRET"));

                    Ok(Turn {
                        items: vec![Item::assistant(
                            "No inbound messages belong to Clara Morales.",
                        )],
                        receipt: json!({"step": 4}),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected mid-run withhold step {other}"),
            }
        }
    }

    let mut inflight_host = PhysicianHost {
        store: store.clone(),
        claim: inflight_claim,
        authority: PostgresWorkspaceAuthority,
    };
    let mut inflight_provider = MidRunWithholdProvider {
        step: 0,
        store: &store,
        auth: auth.as_ref(),
        dr: &dr,
        receiver: &receiver,
        inflight_scope: &inflight_scope,
        ws_b: ws_b.clone(),
        wrong_secret: wrong_secret.to_owned(),
        t0,
    };
    inflight_host
        .run(
            &mut inflight_provider,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(
        inflight_provider.step, 4,
        "PhysicianHost must refuse step 2 publication and complete after step 3 withdrawal confirmation"
    );

    let after_inflight = store.read(&inflight_scope).await.unwrap().unwrap();
    assert!(
        after_inflight
            .messages
            .iter()
            .all(|m| !m.text.contains("WRONG-R1-SECRET")),
        "completed turn after withdrawal confirmation must contain zero wrong-patient text"
    );

    // 3. Verify immediate withholding across `patient_summary_for_physician`, `patient_summaries_for_workspaces`,
    // `patient_starting_context`, and `READ_DOCUMENT_PAGE_TOOL` conversation display/reopen!
    let summary_view_a = store
        .patient_summary_for_physician(auth.as_ref(), &dr, &ws_a)
        .await
        .unwrap()
        .expect("clean fallback summary on ws_a");
    assert_eq!(
        summary_view_a.revision, 1,
        "patient_summary_for_physician must withhold origin/index-tainted rev 2 and serve clean rev 1"
    );
    assert!(
        !serde_json::to_string(&summary_view_a.summary)
            .unwrap()
            .contains("WRONG-R1-SECRET")
    );

    let directory_summaries = store
        .patient_summaries_for_workspaces(std::slice::from_ref(&ws_a))
        .await
        .unwrap();
    assert_eq!(directory_summaries[&ws_a].revision, 1);
    assert!(
        !serde_json::to_string(&directory_summaries[&ws_a].summary)
            .unwrap()
            .contains("WRONG-R1-SECRET")
    );

    let starting_post = store
        .patient_starting_context(&inflight_scope)
        .await
        .unwrap();
    assert!(
        starting_post.open_concerns.is_empty(),
        "patient_starting_context must withhold open concerns whose source_work is tainted: {:?}",
        starting_post.open_concerns
    );

    let read_doc_conv = store.read(&doc_chat_scope).await.unwrap().unwrap();
    for msg in &read_doc_conv.messages {
        assert!(
            !msg.text.contains("WRONG-R1-PDF-PAGE"),
            "Store::read must redact assistant turn that read wrong-patient PDF via READ_DOCUMENT_PAGE_TOOL: {}",
            msg.text
        );
        if matches!(msg.role, MessageRole::Assistant { .. }) {
            assert!(msg.text.starts_with("Unavailable:"));
            assert!(!msg.text.contains('`'));
        }
    }

    let reopened_doc = store
        .reopen_conversation(&doc_chat_scope)
        .await
        .unwrap()
        .unwrap();
    assert!(!reopened_doc.turns[1].raw_text.contains("WRONG-R1-PDF-PAGE"));
    assert_eq!(reopened_doc.sources.len(), 1);
    let (_, doc_src) = reopened_doc.sources.iter().next().unwrap();
    assert_eq!(doc_src.title, "Unavailable · Withheld patient message");
    assert!(!doc_src.body.contains("WRONG-R1-PDF-PAGE"));
    assert!(doc_src.raw_content.is_empty());

    // 4. Inject a Step 2 failure (`SettlingFailed`), then verify `move_spruce_conversation`,
    // `undo_spruce_conversation_move`, and `settle_expired_spruce_moves` NEVER overwrite `SettlingFailed`
    // or allow moving/undoing while unfinished — and once cleared, `settle_expired_spruce_moves`
    // executes ALL 4 steps (withdrawing memory, understanding, concern, follow-up, and withholding reply)!
    struct FailWithdrawOnce {
        armed: Mutex<bool>,
    }
    impl LinkCorrectionRemovalProbe for FailWithdrawOnce {
        fn before_step(
            &self,
            _correction_id: &str,
            _conversation_id: &str,
            step: LinkCorrectionStep,
        ) -> Result<(), String> {
            let mut flag = self.armed.lock().unwrap();
            if *flag && step == LinkCorrectionStep::WithdrawOriginEffects {
                *flag = false;
                return Err("injected step-2 failure".into());
            }
            Ok(())
        }
    }

    let probe = FailWithdrawOnce {
        armed: Mutex::new(true),
    };
    let batch_fail = store
        .settle_expired_link_corrections_with_probe(&practice, Some(&receiver), &probe, t0 + 25)
        .await
        .unwrap();
    assert_eq!(batch_fail.failed.len(), 1);
    assert_eq!(
        batch_fail.failed[0].failed_step,
        LinkCorrectionStep::WithdrawOriginEffects
    );

    // Both `move_spruce_conversation` (back to `ws_a` or onward to `ws_c`) and `undo_spruce_conversation_move`
    // MUST refuse while `SettlingFailed`!
    assert!(matches!(
        store
            .move_spruce_conversation(
                auth.as_ref(),
                &dr,
                MoveSpruceConversationCommand {
                    conversation_id: "conv-r1".into(),
                    destination: MoveDestination::AnotherPatient {
                        workspace: ws_a.clone(),
                    },
                    undo_window_seconds: 10,
                    command_key: "spruce-move-blocked-during-failed".into(),
                },
                Some(&receiver),
                t0 + 26,
            )
            .await,
        Err(SpruceMatchingError::InvalidTransition {
            action: "move_conversation",
            ..
        })
    ));
    assert!(matches!(
        store
            .undo_spruce_conversation_move(
                auth.as_ref(),
                &dr,
                "conv-r1",
                "spruce-undo-blocked-during-failed",
                Some(&receiver),
                t0 + 26,
            )
            .await,
        Err(SpruceMatchingError::InvalidTransition {
            action: "undo_move",
            ..
        })
    ));

    // Recover via `Store::settle_expired_spruce_moves` (NOT `settle_expired_link_corrections`) to prove
    // `spruce_matching` delegates to the unified 4-step settlement!
    let spruce_settle = store
        .settle_expired_spruce_moves(&practice, Some(&receiver), t0 + 27)
        .await
        .unwrap();
    assert_eq!(spruce_settle.succeeded.len(), 1);
    assert!(spruce_settle.failures.is_empty());

    let latest_corr = store
        .latest_conversation_link_correction(&practice, "conv-r1")
        .await
        .unwrap()
        .unwrap();
    let LinkCorrectionStanding::Settled { progress, .. } = &latest_corr.standing else {
        panic!(
            "expected Settled standing after settle_expired_spruce_moves, got {:?}",
            latest_corr.standing
        );
    };
    assert!(progress.withdrawal_ids_by_workspace.contains_key(&ws_a));
    assert!(progress.reply_move_applied);
    assert_eq!(progress.review_command_keys_by_workspace.len(), 2);

    // Verify Step 2 withdrew memory, understanding rev 2, concern, and follow-up on `ws_a`,
    // and Step 3 withheld the reply proposal on `conv-r1`!
    let audit_memories_a = store
        .patient_memories(&practice, &ws_a, MemorySelection::AuditAll)
        .await
        .unwrap();
    assert!(
        audit_memories_a
            .iter()
            .all(|m| matches!(m.standing, MemoryStanding::EnteredInError { .. }))
    );
    let u_audits_a = store
        .patient_understanding_revision_audits(&practice, &ws_a, auth.as_ref())
        .await
        .unwrap();
    assert!(matches!(
        u_audits_a
            .iter()
            .find(|r| r.record.revision == 2)
            .unwrap()
            .standing,
        UnderstandingStanding::EnteredInError { .. }
    ));
    let all_concerns_a = store
        .workspace_concerns(&access_a, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert!(matches!(
        all_concerns_a[0].standing,
        ConcernStanding::Withdrawn { .. }
    ));
    let fu_a = store
        .physician_follow_ups(&access_a, &mut PostgresWorkspaceAuthority)
        .await
        .unwrap();
    assert!(fu_a.is_empty(), "wrong-patient follow-up must be withdrawn");
    let reply_conv = store
        .reply_conversation(&practice, "conv-r1")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        reply_conv.link,
        ConversationLink::LinkedPatient {
            workspace: ws_b.clone()
        }
    );
    assert!(
        matches!(reply_conv.reply_standing, ReplyStanding::Withheld { .. }),
        "Step 3 via settle_expired_spruce_moves must withhold origin patient's reply proposal"
    );

    db.cleanup().await;
}
