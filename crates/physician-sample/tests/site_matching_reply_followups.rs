//! Chromium browser qualification over PostgreSQL for Spruce `Messages to match`,
//! `Filtered`, patient-scoped Spruce reply cards, private drafts, Coda revisions,
//! copy attributions, wrong-patient moves, channel disconnects, and patient-page
//! `Follow-ups` (`crates/physician-sample/src/{matching_reply.rs, follow_ups.rs}`).

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex as StdMutex, OnceLock, Weak};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use base64::Engine as _;
use front_door::PostgresFrontDoorStore;
use harness::provider::ResponsesProvider;
use local_product::ModelRoles;
use local_product::physician_chat::PhysicianSkills;
use local_product::physician_sources::{
    MedicalRecordNumber, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
use local_product::runtime::concerns::{CommandKey, ExactRecordReference, SourceWork};
use local_product::runtime::delivery::Delivery;
use local_product::runtime::dispatch::{PracticeJobReviewReceiver, ReviewReceiver};
use local_product::runtime::engine::{
    self, Event as HarnessEvent, EventKind, Invocation, Item, Turn, recipe::Usage,
};
use local_product::runtime::follow_ups::{
    FollowUpCommandKey, FollowUpId, FollowUpReviewOutcome, FollowUpStanding,
};
use local_product::runtime::jobs::{PracticeAccessError, PracticeAuthority, PracticeScope};
use local_product::runtime::reply::proposals::{
    ChannelConnection, CompleteReplyReview, ConversationLink, MessageSender,
    ObservePracticeMessage, PracticeMessageKind, RecordPatientMessage, ReplyProposalBasisInput,
    ReplyProposalInput, ReplyReviewDecision, ReplyStanding,
};
use local_product::runtime::reply::{FINISH_TOOL, RevisionCommit};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::spruce_matching::{
    ConversationLinkState, IncomingSpruceItem, MessageStanding, MoveDestination,
    MoveSpruceConversationCommand, OutsideParticipant, ReadableSpruceItem, SenderAddress,
    SenderConfirmationPolicy, SpruceAttachmentSummary, UnreadableSpruceItem,
};
use local_product::runtime::{Authority, PublicationTransaction, Scope, Store, async_trait};
use local_product::sha256_hex;
use local_product::spruce_matching_recipe::{
    PATIENT_LOOKUP_TOOL, SUBMIT_CLASSIFICATION_TOOL, run_spruce_message_matching,
};
use physician_auth::{
    AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock, PracticeRecord,
    PracticeTimeZone,
};
use physician_bus::LocalBus;
use physician_sample::durable::DurableChat;
use physician_sample::matching_reply::{
    MatchingReplyFollowUpsFault, MatchingReplyFollowUpsFaultInjection, fault_layer,
};
use physician_sample::queue::ChatQueue;
use physician_sample::{STAGING_PRACTICE_ID, WORKSPACE_IDS};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection};
use serde_json::{Value, json};
use tokio::sync::Mutex;
use uuid::Uuid;

#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;

use devtools::Browser;

/// Synthetic practice clock unix timestamp (`2026-08-27T16:00:00Z`).
const CLINICAL_NOW_UNIX: u64 = 1_787_846_400;

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
static BUILT_SITE: OnceLock<PathBuf> = OnceLock::new();

fn test_lock() -> &'static Mutex<()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(()))
}

fn real_now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or(CLINICAL_NOW_UNIX)
}

fn real_now_i64() -> i64 {
    i64::try_from(real_now_unix()).unwrap_or(1_787_846_400)
}

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

static LOCAL_CLUSTER: StdMutex<Weak<LocalCluster>> = StdMutex::new(Weak::new());

fn ensure_admin_postgres_url() -> (String, Option<Arc<LocalCluster>>) {
    for key in [
        "PHYSICIAN_DATABASE_URL",
        "PHYSICIAN_RUNTIME_TEST_DATABASE_URL",
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
        "coda-mrf-browser-pg-{}-{}",
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
            "-w",
            "-o",
            &opts,
            "start",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .expect("run pg_ctl start");
    assert!(
        start.status.success(),
        "pg_ctl start failed: {}\nlog: {}",
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

fn free_loopback_port() -> u16 {
    use std::net::TcpListener;
    for _ in 0..500 {
        let port = 22_000 + (Uuid::new_v4().as_u128() % 9_000) as u16;
        if TcpListener::bind(("127.0.0.1", port)).is_ok() {
            return port;
        }
    }
    panic!("no free loopback port for test cluster");
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
        let db_name = format!("mrf_site_{label}_{}", Uuid::new_v4().simple());
        let admin = Database::connect(&admin_url)
            .await
            .expect("connect admin db");
        admin
            .execute_unprepared(&format!("CREATE DATABASE {db_name}"))
            .await
            .expect("create disposable db");
        admin.close().await.expect("close admin db");

        let url = format!("{base}/{db_name}");
        PostgresAuthStore::connect_with_migrations(&url)
            .await
            .expect("migrate physician auth");
        PostgresFrontDoorStore::connect_with_migrations(&url)
            .await
            .expect("migrate front door");
        Store::connect(&url)
            .await
            .expect("connect runtime store")
            .migrate()
            .await
            .expect("migrate runtime store");

        Self {
            admin_url,
            db_name,
            url,
            _cluster: cluster,
        }
    }
}

impl Drop for DisposableDb {
    fn drop(&mut self) {
        let sql = format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}' AND pid <> pg_backend_pid()",
            self.db_name
        );
        let _ = Command::new("psql")
            .args([&self.admin_url, "-c", &sql])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        let _ = Command::new("dropdb")
            .args([
                "--maintenance-db",
                &self.admin_url,
                "--if-exists",
                &self.db_name,
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}

#[derive(Default)]
struct RecordingQueue {
    deliveries: StdMutex<Vec<Delivery>>,
}

#[async_trait]
impl ChatQueue for RecordingQueue {
    async fn publish(&self, delivery: &Delivery) -> Result<(), String> {
        self.deliveries
            .lock()
            .expect("queue lock")
            .push(delivery.clone());
        Ok(())
    }
}

struct TestPracticeAuthority<'a>(&'a dyn AuthStore);

#[async_trait]
impl PracticeAuthority for TestPracticeAuthority<'_> {
    async fn check_practice_patient(
        &self,
        scope: &PracticeScope,
    ) -> Result<(), PracticeAccessError> {
        <dyn AuthStore as PracticeAuthority>::check_practice_patient(self.0, scope).await
    }
}

struct OpenWorkspaceAuthority;

#[async_trait]
impl Authority for OpenWorkspaceAuthority {
    async fn check(
        &mut self,
        _tx: &PublicationTransaction,
        _scope: &Scope,
        _session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }

    async fn lock_publication(
        &mut self,
        _tx: &PublicationTransaction,
        _scope: &Scope,
        _session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }
}

struct RunningHarness {
    db_holder: DisposableDb,
    address: SocketAddr,
    store: Store,
    _db: DatabaseConnection,
    auth: Arc<PostgresAuthStore>,
    queue: Arc<RecordingQueue>,
    faults: Arc<MatchingReplyFollowUpsFaultInjection>,
    server_task: tokio::task::JoinHandle<()>,
}

impl Drop for RunningHarness {
    fn drop(&mut self) {
        self.server_task.abort();
    }
}

impl RunningHarness {
    async fn start(label: &str) -> Self {
        let site = build_site();
        // SAFETY: serialized by `test_lock()` held across each test.
        unsafe {
            env::set_var("PHYSICIAN_SITE_DIR", &site);
        }
        let disposable = DisposableDb::create(label).await;
        let store = Store::connect(&disposable.url)
            .await
            .expect("connect store");
        let db = Database::connect(&disposable.url)
            .await
            .expect("connect sea_orm db");
        let auth = Arc::new(
            PostgresAuthStore::connect_without_migrations(&disposable.url)
                .await
                .expect("connect auth store"),
        );

        auth.upsert_practice(&PracticeRecord {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            clock: PracticeClock::Fixed(CLINICAL_NOW_UNIX),
            time_zone: PracticeTimeZone::parse("America/New_York").expect("valid tz"),
        })
        .await
        .expect("upsert practice");

        let queue = Arc::new(RecordingQueue::default());
        let faults = MatchingReplyFollowUpsFaultInjection::new();

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind listener");
        let address = listener.local_addr().expect("local addr");
        let server_task = spawn_durable_server(
            listener,
            &disposable.url,
            store.clone(),
            Arc::clone(&queue),
            Arc::clone(&faults),
        )
        .await;
        wait_until_serving(address).await;

        Self {
            db_holder: disposable,
            address,
            store,
            _db: db,
            auth,
            queue,
            faults,
            server_task,
        }
    }

    async fn restart_server(&mut self) {
        self.server_task.abort();
        let _ = (&mut self.server_task).await;

        let fresh_store = Store::connect(&self.db_holder.url)
            .await
            .expect("reconnect store after server restart");
        let mut bound = None;
        for _ in 0..40 {
            let socket = tokio::net::TcpSocket::new_v4().expect("tcp v4 socket");
            socket.set_reuseaddr(true).expect("set_reuseaddr");
            if socket.bind(self.address).is_ok()
                && let Ok(listener) = socket.listen(128)
            {
                bound = Some(listener);
                break;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        let listener = bound.expect("rebind server listener on same address after restart");
        self.server_task = spawn_durable_server(
            listener,
            &self.db_holder.url,
            fresh_store.clone(),
            Arc::clone(&self.queue),
            Arc::clone(&self.faults),
        )
        .await;
        self.store = fresh_store;
        wait_until_serving(self.address).await;
    }

    async fn enroll_physician(
        &self,
        email: &str,
        display_name: &str,
        workspaces: &[&str],
    ) -> PhysicianIdentity {
        let now = real_now_unix();
        let invite = self
            .auth
            .invite(STAGING_PRACTICE_ID, email, display_name, true, now)
            .await
            .expect("invite physician");
        let (identity, _initial_session) = self
            .auth
            .redeem_invitation(email, &invite, now)
            .await
            .expect("redeem physician invitation");
        for ws in workspaces {
            self.auth
                .set_membership(&identity, ws, Membership::Allowed)
                .await
                .expect("grant panel membership");
        }
        identity
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "seeds one patient's identity and one chart record together"
    )]
    async fn seed_patient_with_chart(
        &self,
        workspace: &str,
        display_name: &str,
        birth_date: &str,
        mrn: &str,
        candidate_id: &str,
        revision: &str,
        content: &str,
    ) -> ExactRecordReference {
        self.auth
            .register_patient(STAGING_PRACTICE_ID, workspace)
            .await
            .expect("register patient workspace");
        let source_rev = SourceRevision {
            workspace: workspace.to_owned(),
            source_id: "clinic-ehr".to_owned(),
            candidate_id: candidate_id.to_owned(),
            revision: revision.to_owned(),
            record_type: "Observation".to_owned(),
            dates: vec![RecordDate {
                name: "effectiveDateTime".to_owned(),
                value: "2026-08-26".to_owned(),
            }],
            content: content.to_owned(),
            sha256: sha256_hex(content.as_bytes()),
            readable_parts: vec![ReadablePart {
                label: "Lab result".to_owned(),
                text: content.to_owned(),
            }],
            provenance: json!({
                "source": "Clinic EHR",
                "patient_name": display_name,
            }),
        };
        let pull = SourcePull {
            source_id: "clinic-ehr".to_owned(),
            source_label: "Clinic EHR".to_owned(),
            patient: SourcePatient {
                source_patient_id: format!("ehr-{workspace}"),
                display_name: display_name.to_owned(),
                birth_date: Some(birth_date.to_owned()),
                sex_or_gender: Some("female".to_owned()),
                medical_record_numbers: vec![MedicalRecordNumber {
                    system: "MRN".to_owned(),
                    value: mrn.to_owned(),
                }],
            },
            pulled_at: real_now_i64() - 120,
        };
        self.store
            .record_chart(workspace, &[source_rev], &pull)
            .await
            .expect("record patient chart");
        ExactRecordReference::new(candidate_id, revision).expect("valid record ref")
    }
}

async fn spawn_durable_server(
    listener: tokio::net::TcpListener,
    db_url: &str,
    store: Store,
    queue: Arc<RecordingQueue>,
    faults: Arc<MatchingReplyFollowUpsFaultInjection>,
) -> tokio::task::JoinHandle<()> {
    let skills = PhysicianSkills::configured(None, false).expect("load physician skills");
    let roles = test_model_roles();
    let chat = Arc::new(DurableChat::new(store.clone(), queue, skills, &roles));
    let bus: Arc<dyn physician_bus::Bus> = Arc::new(LocalBus::new(128));
    let base = physician_sample::durable_site(chat, Arc::clone(&bus)).layer(fault_layer(faults));
    let app = physician_sample::authed_postgres_router(base, db_url, store, bus).await;
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    })
}

async fn wait_until_serving(address: SocketAddr) {
    let client = reqwest::Client::new();
    let started = Instant::now();
    loop {
        assert!(
            started.elapsed() < Duration::from_secs(20),
            "server did not answer /health on {address}"
        );
        if let Ok(resp) = client
            .get(format!("http://{address}/health"))
            .timeout(Duration::from_secs(2))
            .send()
            .await
            && resp.status().is_success()
        {
            return;
        }
        tokio::time::sleep(Duration::from_millis(40)).await;
    }
}

fn test_model_roles() -> ModelRoles {
    local_product::model_roles(
        "test",
        &[
            ("CODA_CHAT_MODEL", "gpt-5.4"),
            ("CODA_CHAT_EFFORT", "medium"),
            ("CODA_CHAT_CONTEXT_WINDOW_TOKENS", "128000"),
            ("CODA_CHAT_INPUT_LIMIT_TOKENS", "64000"),
            ("CODA_BACKGROUND_MODEL", "gpt-5.4"),
            ("CODA_BACKGROUND_EFFORT", "medium"),
            ("CODA_BACKGROUND_CONTEXT_WINDOW_TOKENS", "128000"),
            ("CODA_BACKGROUND_INPUT_LIMIT_TOKENS", "64000"),
        ],
    )
    .expect("valid test model roles")
}

fn build_site() -> PathBuf {
    BUILT_SITE
        .get_or_init(|| {
            let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
            let target = workspace.join("target/coda-physician-wasm");
            let wasm = target.join("wasm32-unknown-unknown/debug/physician-app.wasm");
            let mut cargo = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
            cargo_without_parent_package(&mut cargo);
            let status = cargo
                .current_dir(&workspace)
                .env("CARGO_TARGET_DIR", &target)
                .env_remove("PHYSICIAN_API_URL")
                .args([
                    "build",
                    "-p",
                    "physician-app",
                    "--target",
                    "wasm32-unknown-unknown",
                    "--features",
                    "web",
                ])
                .status()
                .expect("cargo build wasm");
            assert!(status.success(), "physician-app wasm build failed");
            let site = workspace.join("target/physician-web");
            fs::create_dir_all(&site).unwrap();
            let status = Command::new(wasm_bindgen())
                .args([
                    "--target",
                    "web",
                    "--out-dir",
                    site.to_str().unwrap(),
                    wasm.to_str().unwrap(),
                ])
                .status()
                .expect("wasm-bindgen");
            assert!(status.success(), "wasm-bindgen failed");
            site
        })
        .clone()
}

fn cargo_without_parent_package(command: &mut Command) {
    command.env_clear();
    for key in [
        "HOME",
        "USER",
        "RUSTUP_HOME",
        "CARGO_HOME",
        "RUSTUP_TOOLCHAIN",
        "TMPDIR",
    ] {
        if let Some(value) = env::var_os(key) {
            command.env(key, value);
        }
    }
    if let Some(paths) = env::var_os("PATH") {
        let filtered = env::join_paths(
            env::split_paths(&paths)
                .filter(|dir| !dir.to_string_lossy().contains(".local/share/swiftly")),
        )
        .expect("join PATH");
        command.env("PATH", filtered);
    }
}

fn wasm_bindgen() -> PathBuf {
    if let Some(path) = env::var_os("WASM_BINDGEN") {
        return PathBuf::from(path);
    }
    if let Some(paths) = env::var_os("PATH")
        && let Some(found) = env::split_paths(&paths)
            .map(|dir| dir.join("wasm-bindgen"))
            .find(|path| path.is_file())
    {
        return found;
    }
    let local = env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/bin/wasm-bindgen"));
    if let Some(path) = local.filter(|path| path.is_file()) {
        return path;
    }
    panic!("wasm-bindgen is required to build the physician page");
}

fn screenshot_dir() -> PathBuf {
    if let Some(folder) = env::var_os("PHYSICIAN_PROOF_SHOTS") {
        return PathBuf::from(folder);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/verification/captures/matching-reply-followups")
}

struct Tab {
    browser: Browser,
}

impl Tab {
    fn open(url: &str) -> Self {
        let mut browser = Browser::launch();
        browser
            .page()
            .call_session("Page.enable", serde_json::json!({}));
        browser
            .page()
            .call_session("Runtime.enable", serde_json::json!({}));
        let mut tab = Self { browser };
        tab.set_viewport(1440, 960, true);
        tab.browser.page().navigate(url);
        tab
    }

    fn navigate(&mut self, url: &str) {
        self.browser.page().navigate(url);
    }

    fn eval(&mut self, expression: &str) -> serde_json::Value {
        self.browser.page().evaluate(expression)
    }

    fn call(&mut self, method: &str, params: serde_json::Value) -> serde_json::Value {
        self.browser.page().call_session(method, params)
    }

    fn step(&mut self, name: &str, args: serde_json::Value) -> serde_json::Value {
        let mut arguments = vec![serde_json::json!(name)];
        arguments.extend(args.as_array().expect("step args").iter().cloned());
        self.browser
            .page()
            .call_function(include_str!("js/site.js"), &arguments)
    }

    fn click(&mut self, selector: &str) {
        self.browser.click(selector);
    }

    fn type_into(&mut self, selector: &str, text: &str) {
        self.browser.type_into(selector, text);
    }

    fn sign_in_as(&mut self, email: &str) {
        self.wait_for_text("Work email", Duration::from_secs(20));
        self.type_into("input[type=\"email\"]", email);
        std::thread::sleep(Duration::from_millis(60));
        self.click("button.control-primary");
    }

    fn wait_for_selector(&mut self, selector: &str, timeout: Duration) {
        let started = Instant::now();
        let expr = format!(
            "Boolean(document.querySelector({}))",
            serde_json::to_string(selector).unwrap()
        );
        loop {
            if self.eval(&expr).as_bool() == Some(true) {
                return;
            }
            let html = self
                .eval("document.documentElement?.outerHTML ?? ''")
                .as_str()
                .unwrap_or_default()
                .to_owned();
            assert!(
                started.elapsed() < timeout,
                "timed out waiting for selector {selector:?}; DOM:\n{html}"
            );
            std::thread::sleep(Duration::from_millis(60));
        }
    }

    fn wait_for_text(&mut self, expected: &str, timeout: Duration) -> String {
        let started = Instant::now();
        loop {
            let snapshot = self.eval(
                "({text: document.body?.innerText ?? '', html: document.documentElement?.outerHTML ?? ''})",
            );
            let html = snapshot["html"].as_str().unwrap_or_default();
            if snapshot["text"]
                .as_str()
                .is_some_and(|text| text.contains(expected))
            {
                return html.to_owned();
            }
            assert!(
                started.elapsed() < timeout,
                "timed out waiting for {expected:?} in visible text; DOM:\n{html}"
            );
            std::thread::sleep(Duration::from_millis(60));
        }
    }

    fn wait_until_absent(&mut self, forbidden: &str, timeout: Duration) {
        let started = Instant::now();
        loop {
            let text = self
                .eval("document.body?.innerText ?? ''")
                .as_str()
                .unwrap_or_default()
                .to_owned();
            if !text.contains(forbidden) {
                return;
            }
            assert!(
                started.elapsed() < timeout,
                "timed out waiting for {forbidden:?} to leave visible text; still showing:\n{text}"
            );
            std::thread::sleep(Duration::from_millis(60));
        }
    }

    fn reload(&mut self) {
        let previous = self.document_loader();
        self.call("Page.reload", serde_json::json!({}));
        let started = Instant::now();
        while self.document_loader() == previous {
            assert!(
                started.elapsed() < Duration::from_secs(10),
                "reload did not replace document"
            );
            std::thread::sleep(Duration::from_millis(25));
        }
    }

    fn document_loader(&mut self) -> String {
        self.call("Page.getFrameTree", serde_json::json!({}))
            .pointer("/result/frameTree/frame/loaderId")
            .and_then(|id| id.as_str())
            .expect("main frame loader id")
            .to_owned()
    }

    fn set_viewport(&mut self, width: u32, height: u32, reduced_motion: bool) {
        self.call(
            "Emulation.setDeviceMetricsOverride",
            serde_json::json!({
                "width": width,
                "height": height,
                "deviceScaleFactor": 1,
                "mobile": width < 600,
            }),
        );
        self.call(
            "Emulation.setEmulatedMedia",
            serde_json::json!({
                "features": [{
                    "name": "prefers-reduced-motion",
                    "value": if reduced_motion { "reduce" } else { "no-preference" },
                }]
            }),
        );
    }

    fn save_screenshot(&mut self, name: &str) -> PathBuf {
        let shot = self.call(
            "Page.captureScreenshot",
            serde_json::json!({ "format": "png" }),
        );
        let encoded = shot["result"]["data"].as_str().expect("screenshot data");
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .expect("decode screenshot png");
        let dir = screenshot_dir();
        fs::create_dir_all(&dir).expect("create screenshot dir");
        let path = dir.join(format!("{name}.png"));
        fs::write(&path, bytes).expect("write screenshot png");
        path
    }

    fn assert_no_horizontal_overflow(&mut self, context: &str) {
        let metrics = self.eval(
            "({scrollWidth: document.documentElement.scrollWidth, clientWidth: document.documentElement.clientWidth})",
        );
        let scroll_w = metrics["scrollWidth"].as_f64().unwrap_or(9999.0);
        let client_w = metrics["clientWidth"].as_f64().unwrap_or(0.0);
        assert!(
            scroll_w <= client_w + 1.0,
            "horizontal overflow on {context}: scrollWidth={scroll_w}, clientWidth={client_w}"
        );
    }
}

async fn connect_spruce_and_lease(
    store: &Store,
    auth: &dyn AuthStore,
    actor: &PhysicianIdentity,
) -> String {
    let now_ts = real_now_i64();
    store
        .connect_spruce_channel(auth, actor, 10, Some("cursor-0".to_owned()), now_ts - 60)
        .await
        .expect("connect spruce channel");
    store
        .accept_spruce_backlog(auth, actor, now_ts - 50)
        .await
        .expect("accept spruce backlog");
    let lease = store
        .acquire_spruce_reader_lease(STAGING_PRACTICE_ID, "test-reader-1", 300, now_ts - 40)
        .await
        .expect("acquire spruce lease");
    lease.lease_token
}

#[allow(clippy::too_many_arguments)]
fn make_readable_spruce_item(
    pos: u64,
    message_id: &str,
    conversation_id: &str,
    sender: Option<&str>,
    author: Option<&str>,
    body: &str,
    attachments: Vec<SpruceAttachmentSummary>,
    participants: Vec<OutsideParticipant>,
) -> IncomingSpruceItem {
    IncomingSpruceItem::Readable(ReadableSpruceItem {
        message_id: message_id.to_owned(),
        conversation_id: conversation_id.to_owned(),
        channel_position: pos,
        content_version: format!("ver-{message_id}-{pos}"),
        sender_address: sender.map(|s| SenderAddress::parse(s).expect("valid sender address")),
        author_name: author.map(str::to_owned),
        body_text: body.to_owned(),
        attachments,
        app_url: Some(format!(
            "https://app.sprucehealth.com/conversations/{conversation_id}"
        )),
        created_at_spruce: Some("2026-08-27T15:30:00Z".to_owned()),
        outside_participants: participants,
    })
}

#[derive(Clone, Default)]
struct ScriptedSseMatchingState {
    decisions_by_message: Arc<BTreeMap<String, Value>>,
    calls_by_message: Arc<StdMutex<BTreeMap<String, usize>>>,
}

async fn start_matching_sse_server(
    decisions: BTreeMap<String, Value>,
) -> (String, tokio::task::JoinHandle<()>) {
    let state = ScriptedSseMatchingState {
        decisions_by_message: Arc::new(decisions),
        calls_by_message: Arc::new(StdMutex::new(BTreeMap::new())),
    };
    let app = Router::new()
        .route(
            "/v1/responses",
            post(
                |State(state): State<ScriptedSseMatchingState>, Json(body): Json<Value>| async move {
                    let body_str = serde_json::to_string(&body).unwrap_or_default();
                    let matched_id = state
                        .decisions_by_message
                        .keys()
                        .find(|mid| body_str.contains(mid.as_str()))
                        .cloned()
                        .expect("SSE request must reference one scripted message_id");
                    let step = {
                        let mut guard = state.calls_by_message.lock().expect("sse lock");
                        let count = guard.entry(matched_id.clone()).or_insert(0);
                        *count += 1;
                        *count
                    };
                    let model_name = body["model"]
                        .as_str()
                        .unwrap_or("gpt-5.4")
                        .to_owned();
                    let (done_item, response_obj) = match step {
                        1 => {
                            let item = json!({
                                "type": "function_call",
                                "status": "completed",
                                "call_id": format!("lookup-{matched_id}"),
                                "name": PATIENT_LOOKUP_TOOL,
                                "arguments": json!({}).to_string(),
                            });
                            let resp = json!({
                                "id": format!("resp-{matched_id}-1"),
                                "model": model_name,
                                "status": "completed",
                                "output": [item],
                                "usage": { "input_tokens": 240, "output_tokens": 18 }
                            });
                            (item, resp)
                        }
                        2 => {
                            let decision_args = state
                                .decisions_by_message
                                .get(&matched_id)
                                .expect("decision args")
                                .clone();
                            let item = json!({
                                "type": "function_call",
                                "status": "completed",
                                "call_id": format!("submit-{matched_id}"),
                                "name": SUBMIT_CLASSIFICATION_TOOL,
                                "arguments": decision_args.to_string(),
                            });
                            let resp = json!({
                                "id": format!("resp-{matched_id}-2"),
                                "model": model_name,
                                "status": "completed",
                                "output": [item],
                                "usage": { "input_tokens": 360, "output_tokens": 42 }
                            });
                            (item, resp)
                        }
                        _ => {
                            let item = json!({
                                "type": "message",
                                "role": "assistant",
                                "status": "completed",
                                "content": [{
                                    "type": "output_text",
                                    "text": "Classification and match assessment recorded."
                                }]
                            });
                            let resp = json!({
                                "id": format!("resp-{matched_id}-3"),
                                "model": model_name,
                                "status": "completed",
                                "output": [item],
                                "usage": { "input_tokens": 410, "output_tokens": 12 }
                            });
                            (item, resp)
                        }
                    };
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
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind sse mock");
    let addr = listener.local_addr().expect("sse addr");
    let handle = tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    (format!("http://{addr}/v1/responses"), handle)
}

fn revision_turn(text: &str, basis: &str) -> Turn {
    Turn {
        items: vec![Item::Call(Invocation {
            id: "finish-rev".to_owned(),
            name: FINISH_TOOL.to_owned(),
            arguments: json!({
                "text": text,
                "review_basis": basis,
            })
            .to_string(),
        })],
        usage: Usage::default(),
        receipt: json!({ "synthetic": true }),
    }
}

async fn record_revision_candidate(
    store: &Store,
    claim: &local_product::runtime::reply::RevisionClaim,
    turn: &Turn,
) {
    for (sequence, kind) in [
        (1, EventKind::ProviderReturned(turn.clone())),
        (2, EventKind::Candidate { text: turn.text() }),
    ] {
        store
            .record_revision(
                claim,
                HarnessEvent {
                    attempt_id: claim.attempt.clone(),
                    sequence,
                    ordinal: 1,
                    kind,
                },
            )
            .await
            .expect("record revision candidate event");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "launches headless Chromium"]
async fn matching_screen_filtered_override_and_sse_recipe_loop() {
    let _guard = test_lock().lock().await;
    let harness = RunningHarness::start("matching_sse").await;

    let ws_elena = WORKSPACE_IDS[0];
    let ws_jake = WORKSPACE_IDS[1];
    let ws_maria = WORKSPACE_IDS[2];
    let ws_robert = WORKSPACE_IDS[3];

    harness
        .seed_patient_with_chart(
            ws_elena,
            "Elena Morales",
            "1968-05-14",
            "MRN-1001",
            "obs-elena-1",
            "rev-1",
            "Basic metabolic panel potassium 4.4 mmol/L.",
        )
        .await;
    harness
        .seed_patient_with_chart(
            ws_jake,
            "Jake Patel",
            "1975-11-02",
            "MRN-1002",
            "obs-jake-1",
            "rev-1",
            "Home blood pressure log average 128/78 mmHg.",
        )
        .await;
    harness
        .seed_patient_with_chart(
            ws_maria,
            "Maria Gomez",
            "1982-03-19",
            "MRN-1003",
            "obs-maria-1",
            "rev-1",
            "Annual physical normal.",
        )
        .await;
    harness
        .seed_patient_with_chart(
            ws_robert,
            "Robert Lin",
            "1959-09-08",
            "MRN-1004",
            "obs-robert-1",
            "rev-1",
            "Physical therapy referral evaluation.",
        )
        .await;

    // Dr. Rivera has panel access to Elena, Jake, and Maria, NOT Robert Lin.
    let dr_rivera = harness
        .enroll_physician(
            "dr.rivera@codahealth.test",
            "Dr. Rivera",
            &[ws_elena, ws_jake, ws_maria],
        )
        .await;
    // Dr. Chen has panel access to Robert Lin so Robert is an active panel patient in the practice.
    let _dr_chen = harness
        .enroll_physician("dr.chen@codahealth.test", "Dr. Chen", &[ws_robert])
        .await;

    let lease_token =
        connect_spruce_and_lease(&harness.store, harness.auth.as_ref(), &dr_rivera).await;
    let now_ts = real_now_i64();

    // 1. Ordinary single-patient suggestion (ingested first so urgent ordering must promote item 2 ahead of it!)
    harness
        .store
        .ingest_spruce_item(
            STAGING_PRACTICE_ID,
            &lease_token,
            &make_readable_spruce_item(
                1,
                "msg-suggest-elena",
                "conv-suggest-elena",
                Some("+14155550102"),
                Some("Elena Morales"),
                "Hi clinic, requesting a refill on my lisinopril 10 mg tablets. Attached bottle photo.",
                vec![SpruceAttachmentSummary {
                    kind: "image".to_owned(),
                    title: Some("Rx Bottle".to_owned()),
                    name: Some("lisinopril-bottle.jpg".to_owned()),
                    mimetype: Some("image/jpeg".to_owned()),
                }],
                vec![OutsideParticipant {
                    display_name: Some("Elena Morales".to_owned()),
                    endpoint_channel: Some("sms".to_owned()),
                    endpoint_address: Some(SenderAddress::parse("+14155550102").unwrap()),
                    saved_contact_id: None,
                }],
            ),
            now_ts - 30,
        )
        .await
        .expect("ingest msg-suggest-elena");

    // 2. Urgent unmatched message (`Now`) needing physician choice
    harness
        .store
        .ingest_spruce_item(
            STAGING_PRACTICE_ID,
            &lease_token,
            &make_readable_spruce_item(
                2,
                "msg-urgent-bp",
                "conv-urgent-bp",
                Some("+14155550101"),
                Some("Carlos Morales"),
                "URGENT: Home blood pressure is 196/114 with severe headache and blurry vision right now.",
                Vec::new(),
                vec![OutsideParticipant {
                    display_name: Some("Carlos Morales".to_owned()),
                    endpoint_channel: Some("sms".to_owned()),
                    endpoint_address: Some(SenderAddress::parse("+14155550101").unwrap()),
                    saved_contact_id: None,
                }],
            ),
            now_ts - 25,
        )
        .await
        .expect("ingest msg-urgent-bp");

    // 3. Off-panel suggestion (`ws_robert`) — must be filtered out of Dr. Rivera's suggestions!
    harness
        .store
        .ingest_spruce_item(
            STAGING_PRACTICE_ID,
            &lease_token,
            &make_readable_spruce_item(
                3,
                "msg-suggest-offpanel",
                "conv-suggest-offpanel",
                Some("+14155550103"),
                Some("Care Coordinator"),
                "Following up on physical therapy authorization request.",
                Vec::new(),
                vec![OutsideParticipant {
                    display_name: Some("Care Coordinator".to_owned()),
                    endpoint_channel: Some("sms".to_owned()),
                    endpoint_address: Some(SenderAddress::parse("+14155550103").unwrap()),
                    saved_contact_id: None,
                }],
            ),
            now_ts - 20,
        )
        .await
        .expect("ingest msg-suggest-offpanel");

    // 4. Non-patient vendor spam -> `Filtered`
    harness
        .store
        .ingest_spruce_item(
            STAGING_PRACTICE_ID,
            &lease_token,
            &make_readable_spruce_item(
                4,
                "msg-vendor-fax",
                "conv-vendor-fax",
                Some("+18005559999"),
                Some("MedSupply Deals"),
                "QUARTERLY FAX PROMO: Save 40 percent on exam table paper rolls!",
                Vec::new(),
                vec![OutsideParticipant {
                    display_name: Some("MedSupply Deals".to_owned()),
                    endpoint_channel: Some("fax".to_owned()),
                    endpoint_address: Some(SenderAddress::parse("+18005559999").unwrap()),
                    saved_contact_id: None,
                }],
            ),
            now_ts - 15,
        )
        .await
        .expect("ingest msg-vendor-fax");

    // 5. Unreadable Spruce item -> surfaces `Open in Spruce` + `Done`
    harness
        .store
        .ingest_spruce_item(
            STAGING_PRACTICE_ID,
            &lease_token,
            &IncomingSpruceItem::Unreadable(UnreadableSpruceItem {
                message_id: "msg-unreadable-1".to_owned(),
                conversation_id: "conv-unreadable-1".to_owned(),
                channel_position: 5,
                content_version: "ver-unreadable-1".to_owned(),
                failure_reason: "Corrupted audio attachment payload could not be decoded."
                    .to_owned(),
                app_url: Some(
                    "https://app.sprucehealth.com/conversations/conv-unreadable-1".to_owned(),
                ),
                outside_participants: Vec::new(),
            }),
            now_ts - 10,
        )
        .await
        .expect("ingest unreadable spruce item");

    // Run `run_spruce_message_matching` over a real local HTTP SSE `ResponsesProvider` server!
    let mut decisions = BTreeMap::new();
    decisions.insert(
        "msg-suggest-elena".to_owned(),
        json!({
            "classification": "patient_mail",
            "urgency": "ordinary",
            "match_outcome": "suggested_patient",
            "suggested_workspace": ws_elena,
            "evidence_summary": "Sender name Elena Morales matches registered patient Elena Morales.",
            "rationale": "Patient refill request naming lisinopril from Elena Morales."
        }),
    );
    decisions.insert(
        "msg-urgent-bp".to_owned(),
        json!({
            "classification": "patient_mail",
            "urgency": "urgent",
            "urgent_reason": "Symptomatic severe hypertension 196/114 with headache and blurry vision",
            "match_outcome": "ambiguous_candidates",
            "candidate_workspaces": [ws_elena, ws_maria],
            "evidence_summary": "Household surname matches Elena Morales while sender Carlos Morales is unconfirmed.",
            "rationale": "Acute hypertensive symptoms requiring immediate clinical routing."
        }),
    );
    decisions.insert(
        "msg-suggest-offpanel".to_owned(),
        json!({
            "classification": "patient_mail",
            "urgency": "ordinary",
            "match_outcome": "suggested_patient",
            "suggested_workspace": ws_robert,
            "evidence_summary": "PT referral inquiry matches Robert Lin.",
            "rationale": "Patient care coordination message."
        }),
    );
    decisions.insert(
        "msg-vendor-fax".to_owned(),
        json!({
            "classification": "not_patient_mail",
            "non_patient_category": "vendor_or_spam",
            "rationale": "Commercial medical supply marketing fax unrelated to any patient."
        }),
    );

    let (sse_endpoint, sse_task) = start_matching_sse_server(decisions).await;
    let mut provider = ResponsesProvider::new(
        sse_endpoint,
        "local-sse-key".to_owned(),
        Arc::new(harness.store.clone()),
    )
    .expect("build ResponsesProvider");
    let roles = test_model_roles();
    let skills = PhysicianSkills::configured(None, false).expect("load skills");
    let practice_auth = TestPracticeAuthority(harness.auth.as_ref());
    let receiver =
        PracticeJobReviewReceiver::new(harness.store.clone(), &roles, &skills, &practice_auth);

    for mid in [
        "msg-suggest-elena",
        "msg-urgent-bp",
        "msg-suggest-offpanel",
        "msg-vendor-fax",
    ] {
        run_spruce_message_matching(
            &harness.store,
            harness.auth.as_ref(),
            STAGING_PRACTICE_ID,
            mid,
            &roles,
            &skills,
            &mut provider,
            Some(&receiver as &dyn ReviewReceiver),
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap_or_else(|err| panic!("run_spruce_message_matching failed for {mid}: {err}"));
    }
    sse_task.abort();

    // Open Chromium and sign in as Dr. Rivera
    let mut tab = Tab::open(&format!("http://{}/", harness.address));
    tab.sign_in_as("dr.rivera@codahealth.test");
    tab.wait_for_selector(
        "[data-nav-messages-to-match=\"true\"]",
        Duration::from_secs(20),
    );
    tab.click("[data-nav-messages-to-match=\"true\"]");

    tab.wait_for_selector(
        "[data-surface=\"messages-to-match\"]",
        Duration::from_secs(15),
    );
    tab.wait_for_text(
        "URGENT: Home blood pressure is 196/114",
        Duration::from_secs(15),
    );
    tab.wait_for_text("lisinopril-bottle.jpg", Duration::from_secs(10));
    tab.wait_for_text("+14155550101", Duration::from_secs(10));
    tab.wait_for_text("Carlos Morales", Duration::from_secs(10));

    // Verify urgent `Now` row leads `Messages to match` ahead of ordinary items!
    let row_ids: Vec<String> = serde_json::from_value(tab.eval(
        "[...document.querySelectorAll('[data-unmatched-row]')].map(el => el.getAttribute('data-unmatched-row'))",
    ))
    .expect("row ids");
    assert_eq!(
        row_ids.first().map(String::as_str),
        Some("msg-urgent-bp"),
        "urgent `Now` message must lead `Messages to match`, got {row_ids:?}"
    );
    assert!(
        tab.eval("Boolean(document.querySelector('[data-urgent-now-pill=\"msg-urgent-bp\"]'))")
            .as_bool()
            == Some(true),
        "urgent `Now` pill must render on msg-urgent-bp"
    );

    // Verify access-filtered suggestions AND zero numeric confidence scores!
    let visible_matching_text = tab
        .eval("document.querySelector('[data-surface=\"messages-to-match\"]').innerText")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        visible_matching_text.contains("Elena Morales"),
        "panel patient Elena Morales must appear as suggested match"
    );
    assert!(
        !visible_matching_text.contains("Robert Lin"),
        "off-panel patient Robert Lin must never appear in Dr. Rivera's suggested matches:\n{visible_matching_text}"
    );
    let lower_matching = visible_matching_text.to_ascii_lowercase();
    assert!(
        !visible_matching_text.contains("pw-"),
        "suggested and candidate patients must show names, never raw workspace ids:\n{visible_matching_text}"
    );
    assert!(
        visible_matching_text.contains("Maria Gomez"),
        "ambiguous candidates must name each authorized patient:\n{visible_matching_text}"
    );
    assert!(
        !lower_matching.contains("confidence") && !visible_matching_text.contains('%'),
        "zero numeric confidence scores or percentages may appear in Messages to match:\n{visible_matching_text}"
    );

    tab.assert_no_horizontal_overflow("Messages to match desktop 1440x960");
    tab.save_screenshot("01-messages-to-match-desktop");

    // Check phone viewport (390x844) has zero sideways scroll
    tab.set_viewport(390, 844, true);
    std::thread::sleep(Duration::from_millis(120));
    tab.assert_no_horizontal_overflow("Messages to match phone 390x844");
    tab.save_screenshot("02-messages-to-match-phone");
    tab.set_viewport(1440, 960, true);
    std::thread::sleep(Duration::from_millis(80));

    // Exercise `Confirm` on `msg-suggest-elena` -> verifies saved link in UI and `Store`!
    tab.click("[data-confirm-match=\"msg-suggest-elena\"]");
    tab.wait_for_selector(
        "[data-matching-outcome-banner=\"true\"]",
        Duration::from_secs(15),
    );
    let conv_elena = harness
        .store
        .spruce_conversation(STAGING_PRACTICE_ID, "conv-suggest-elena")
        .await
        .expect("read spruce conv")
        .expect("conv-suggest-elena exists");
    assert!(
        matches!(
            conv_elena.link_state,
            ConversationLinkState::Linked { ref workspace, .. } if workspace == ws_elena
        ),
        "expected conv-suggest-elena linked to Elena ({ws_elena}), got {:?}",
        conv_elena.link_state
    );

    // Exercise `Choose another patient` on `msg-urgent-bp` -> search `"Jake"` -> link to `ws_jake`!
    tab.click("[data-choose-another-patient=\"msg-urgent-bp\"]");
    tab.wait_for_selector(
        "[data-patient-picker-dialog=\"true\"]",
        Duration::from_secs(10),
    );
    tab.type_into("[data-patient-picker-dialog=\"true\"] input", "Jake");
    tab.wait_for_selector(
        &format!("[data-picker-select=\"{ws_jake}\"]"),
        Duration::from_secs(10),
    );
    tab.click(&format!("[data-picker-select=\"{ws_jake}\"]"));
    tab.wait_until_absent(
        "URGENT: Home blood pressure is 196/114",
        Duration::from_secs(15),
    );
    let conv_urgent = harness
        .store
        .spruce_conversation(STAGING_PRACTICE_ID, "conv-urgent-bp")
        .await
        .expect("read conv-urgent-bp")
        .expect("conv-urgent-bp exists");
    assert!(
        matches!(
            conv_urgent.link_state,
            ConversationLinkState::Linked { ref workspace, .. } if workspace == ws_jake
        ),
        "expected conv-urgent-bp linked to Jake ({ws_jake}), got {:?}",
        conv_urgent.link_state
    );

    // Exercise `Leave unlinked` on `msg-suggest-offpanel`
    tab.click("[data-leave-unlinked=\"msg-suggest-offpanel\"]");
    tab.wait_until_absent(
        "Following up on physical therapy authorization request.",
        Duration::from_secs(15),
    );
    let msg_offpanel = harness
        .store
        .spruce_message(STAGING_PRACTICE_ID, "msg-suggest-offpanel")
        .await
        .expect("read msg-suggest-offpanel")
        .expect("msg-suggest-offpanel exists");
    assert!(
        matches!(msg_offpanel.standing, MessageStanding::LeftUnlinked { .. }),
        "expected LeftUnlinked standing, got {:?}",
        msg_offpanel.standing
    );

    // Exercise `Done` on unreadable item
    tab.click("[data-unreadable-done=\"msg-unreadable-1\"]");
    tab.wait_until_absent(
        "Corrupted audio attachment payload could not be decoded.",
        Duration::from_secs(15),
    );

    // Open `Filtered` -> inspect category + rationale -> click `"This is a patient"`!
    tab.click("[data-open-filtered=\"true\"]");
    tab.wait_for_selector(
        "[data-surface=\"filtered-messages\"]",
        Duration::from_secs(15),
    );
    tab.wait_for_text("MedSupply Deals", Duration::from_secs(10));
    tab.wait_for_text(
        "Commercial medical supply marketing fax unrelated to any patient.",
        Duration::from_secs(10),
    );
    tab.save_screenshot("03-filtered-messages-desktop");

    tab.click("[data-this-is-a-patient=\"msg-vendor-fax\"]");
    tab.wait_for_selector(
        "[data-this-is-patient-outcome=\"true\"]",
        Duration::from_secs(15),
    );
    let overridden = harness
        .store
        .spruce_message(STAGING_PRACTICE_ID, "msg-vendor-fax")
        .await
        .expect("read msg-vendor-fax")
        .expect("msg-vendor-fax exists");
    assert!(
        overridden
            .saved_classification
            .as_ref()
            .and_then(|c| c.physician_override.as_ref())
            .is_some(),
        "expected physician_override recorded on msg-vendor-fax"
    );
    assert!(
        matches!(overridden.standing, MessageStanding::Waiting { .. }),
        "expected msg-vendor-fax returned to Waiting routing, got {:?}",
        overridden.standing
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "launches headless Chromium"]
async fn reply_editor_basis_private_drafts_moves_withheld_and_disconnected() {
    let _guard = test_lock().lock().await;
    let harness = RunningHarness::start("reply_lifecycle").await;

    let ws_elena = WORKSPACE_IDS[0];
    let ws_jake = WORKSPACE_IDS[1];

    let ref_bmp = harness
        .seed_patient_with_chart(
            ws_elena,
            "Elena Morales",
            "1968-05-14",
            "MRN-2001",
            "obs-bmp-elena",
            "rev-1",
            "Morning basic metabolic panel: serum potassium 4.4 mmol/L (normal 3.5-5.1), serum creatinine 1.0 mg/dL.",
        )
        .await;
    harness
        .seed_patient_with_chart(
            ws_jake,
            "Jake Patel",
            "1975-11-02",
            "MRN-2002",
            "obs-bp-jake",
            "rev-1",
            "Ambulatory blood pressure check: 124/76 mmHg.",
        )
        .await;

    let dr_rivera = harness
        .enroll_physician(
            "dr.rivera@codahealth.test",
            "Dr. Rivera",
            &[ws_elena, ws_jake],
        )
        .await;
    let _dr_chen = harness
        .enroll_physician("dr.chen@codahealth.test", "Dr. Chen", &[ws_elena, ws_jake])
        .await;

    let lease_token =
        connect_spruce_and_lease(&harness.store, harness.auth.as_ref(), &dr_rivera).await;
    let now_ts = real_now_i64();
    let conv_id = "conv-caregiver-elena";

    // Ingest Spruce thread from caregiver "Marco Morales (Spouse)" (NOT Elena herself)
    harness
        .store
        .ingest_spruce_item(
            STAGING_PRACTICE_ID,
            &lease_token,
            &make_readable_spruce_item(
                1,
                "msg-marco-1",
                conv_id,
                Some("+14155550199"),
                Some("Marco Morales (Spouse)"),
                "Hi Dr. Rivera, this is Marco writing about Elena's morning bloodwork — is her potassium safe to stay on lisinopril?",
                Vec::new(),
                vec![OutsideParticipant {
                    display_name: Some("Marco Morales (Spouse)".to_owned()),
                    endpoint_channel: Some("sms".to_owned()),
                    endpoint_address: Some(SenderAddress::parse("+14155550199").unwrap()),
                    saved_contact_id: None,
                }],
            ),
            now_ts - 60,
        )
        .await
        .expect("ingest caregiver message");

    harness
        .store
        .record_spruce_message_classification(
            harness.auth.as_ref(),
            STAGING_PRACTICE_ID,
            "msg-marco-1",
            None,
            "spruce-message-matching/v1",
            "spruce-matching-guidance/v1",
            local_product::runtime::spruce_matching::MatchingDecision::PatientMail {
                urgency: local_product::runtime::spruce_matching::MessageUrgencyChoice::Ordinary,
                assessment:
                    local_product::runtime::spruce_matching::MatchAssessment::SuggestedPatient {
                        workspace: ws_elena.to_owned(),
                        evidence_summary: "Spouse Marco Morales writing about Elena Morales."
                            .to_owned(),
                    },
                rationale: "Patient medication and lab safety inquiry.".to_owned(),
            },
            None,
            now_ts - 58,
        )
        .await
        .expect("classify caregiver message into Waiting");
    harness
        .store
        .confirm_spruce_message_link(
            harness.auth.as_ref(),
            &dr_rivera,
            "msg-marco-1",
            ws_elena,
            SenderConfirmationPolicy::ConversationOnly,
            "cmd-confirm-marco-1",
            None,
            now_ts - 55,
        )
        .await
        .expect("confirm caregiver conversation link to Elena");

    // Link reply conversation + record caregiver sender explicitly (`MessageSender::NamedSender`)
    harness
        .store
        .link_reply_conversation(
            STAGING_PRACTICE_ID,
            conv_id,
            ConversationLink::LinkedPatient {
                workspace: ws_elena.to_owned(),
            },
        )
        .await
        .expect("link reply conversation");
    harness
        .store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: STAGING_PRACTICE_ID.to_owned(),
            conversation_id: conv_id.to_owned(),
            message_id: "msg-marco-1".to_owned(),
            sender: MessageSender::NamedSender {
                name: "Marco Morales (Spouse)".to_owned(),
            },
            text: "Hi Dr. Rivera, this is Marco writing about Elena's morning bloodwork — is her potassium safe to stay on lisinopril?".to_owned(),
            sent_at: now_ts - 60,
        })
        .await
        .expect("record caregiver message on reply conversation");

    // Complete Proposal v1 citing `obs-bmp-elena@rev-1`
    harness
        .store
        .start_or_read_reply_review(STAGING_PRACTICE_ID, conv_id, "rev-reply-1", "run-reply-1")
        .await
        .expect("start reply review 1");
    let proposal_v1_text = "Hi Marco, Elena's potassium on yesterday's metabolic panel is normal at 4.4 mmol/L, so she can continue her current lisinopril dose.";
    harness
        .store
        .complete_reply_review(&CompleteReplyReview {
            practice: STAGING_PRACTICE_ID.to_owned(),
            conversation_id: conv_id.to_owned(),
            review_id: "rev-reply-1".to_owned(),
            run_id: "run-reply-1".to_owned(),
            completion_key: "comp-reply-1".to_owned(),
            answered_closures: Vec::new(),
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: proposal_v1_text.to_owned(),
                basis: ReplyProposalBasisInput {
                    summary: "Grounded in Clinic EHR basic metabolic panel (2026-08-26) showing potassium 4.4 mmol/L and creatinine 1.0 mg/dL.".to_owned(),
                    limits: vec!["No home blood pressure reading attached in thread.".to_owned()],
                    cited_records: vec![ref_bmp.clone()],
                },
                cited_records: vec![ref_bmp.clone()],
            }),
            raised_concern_ids: Vec::new(),
        })
        .await
        .expect("complete proposal v1");

    // Open Tab A as Dr. Rivera directly on Elena Morales's patient page
    let mut tab_a = Tab::open(&format!("http://{}/?patient={ws_elena}", harness.address));
    tab_a.sign_in_as("dr.rivera@codahealth.test");
    tab_a.wait_for_selector(
        &format!("[data-spruce-reply-card=\"{conv_id}\"]"),
        Duration::from_secs(20),
    );
    tab_a.wait_for_text("Marco Morales (Spouse)", Duration::from_secs(10));

    // AC5: Card names the channel sender (`Marco Morales (Spouse)`), never naming linked patient Elena as sender!
    let card_text_a = tab_a
        .eval(&format!(
            "document.querySelector('[data-spruce-reply-card=\"{conv_id}\"]').innerText"
        ))
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        card_text_a.contains("Marco Morales (Spouse)"),
        "reply card must name channel sender Marco Morales (Spouse):\n{card_text_a}"
    );
    assert!(
        tab_a
            .eval(&format!(
                "Boolean(document.querySelector('[data-non-patient-sender=\"{conv_id}\"]'))"
            ))
            .as_bool()
            == Some(true),
        "non-patient caregiver sender indicator must be shown alongside patient"
    );

    // AC2: Open `Basis` and `Sources` (`CitationChip` -> `ReaderDialog`)
    tab_a.click(&format!("[data-open-reply-basis=\"{conv_id}\"]"));
    tab_a.wait_for_text(
        "Grounded in Clinic EHR basic metabolic panel (2026-08-26)",
        Duration::from_secs(10),
    );
    tab_a.click(&format!(
        "[data-reply-sources=\"{conv_id}\"] button.citation-chip"
    ));
    tab_a.wait_for_text(
        "serum potassium 4.4 mmol/L (normal 3.5-5.1)",
        Duration::from_secs(10),
    );
    tab_a.save_screenshot("04-reply-card-evidence-reader");
    tab_a.step("keyOn", json!(["dialog.dialog", "Escape"]));
    std::thread::sleep(Duration::from_millis(120));

    // AC2: Edit physician private draft + `Save draft`
    let rivera_private_text = "Hi Marco, Elena's potassium is normal at 4.4 mmol/L; please keep her lisinopril 10 mg daily.";
    tab_a.type_into(
        &format!("[data-reply-editor=\"{conv_id}\"] textarea"),
        rivera_private_text,
    );
    tab_a.click(&format!("[data-save-reply-draft=\"{conv_id}\"]"));
    tab_a.wait_for_text("Your draft", Duration::from_secs(10));

    // AC2: `Ask Coda to revise` -> commit worker revision -> `Restore my version`
    let queue_len_before = harness.queue.deliveries.lock().expect("queue lock").len();
    tab_a.type_into(
        &format!("[data-spruce-reply-card=\"{conv_id}\"] input[type=\"text\"], [data-spruce-reply-card=\"{conv_id}\"] input:not([type])"),
        "Add reminder to recheck home BP on Friday",
    );
    tab_a.click(&format!("[data-ask-coda-revise=\"{conv_id}\"]"));
    tab_a.wait_for_selector("[data-revision-status]", Duration::from_secs(15));

    // Claim the queued revision via `Store::claim_revision` and commit Coda's revised wording!
    let draft_scope = Scope {
        practice: STAGING_PRACTICE_ID.to_owned(),
        physician: dr_rivera.physician_id.clone(),
        workspace: ws_elena.to_owned(),
        chat: format!("reply-proposal:{conv_id}:1"),
    };
    let saved_draft = harness
        .store
        .draft(&draft_scope)
        .await
        .expect("read draft")
        .expect("draft exists");
    let rev_key = saved_draft
        .accepted
        .as_ref()
        .expect("accepted revision")
        .input
        .key
        .clone();
    let _ = queue_len_before;
    let delivery = Delivery::reply_revision(draft_scope.clone(), rev_key);
    let claim = harness
        .store
        .claim_revision(&delivery, 60)
        .await
        .expect("claim revision")
        .expect("revision claim granted");
    let coda_revised_text = "Hi Marco, Elena's potassium is normal at 4.4 mmol/L. Please keep her lisinopril 10 mg daily and recheck her home BP on Friday.";
    let turn = revision_turn(
        coda_revised_text,
        "Added Friday home BP recheck instruction over verified BMP.",
    );
    record_revision_candidate(&harness.store, &claim, &turn).await;
    assert_eq!(
        harness
            .store
            .commit_revision(&claim, &turn, &mut OpenWorkspaceAuthority)
            .await
            .expect("commit revision"),
        RevisionCommit::Committed
    );

    tab_a.reload();
    tab_a.wait_for_selector(
        &format!("[data-restore-my-version=\"{conv_id}\"]"),
        Duration::from_secs(15),
    );
    let revised_textarea_val = tab_a
        .eval(&format!(
            "document.querySelector('[data-reply-editor=\"{conv_id}\"] textarea').value"
        ))
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert_eq!(revised_textarea_val, coda_revised_text);
    tab_a.save_screenshot("05-reply-coda-revision-and-restore");

    // Click `Restore my version` -> restores Dr. Rivera's exact pre-revision text!
    tab_a.click(&format!("[data-restore-my-version=\"{conv_id}\"]"));
    let started = Instant::now();
    loop {
        let cur = tab_a
            .eval(&format!(
                "document.querySelector('[data-reply-editor=\"{conv_id}\"] textarea')?.value ?? ''"
            ))
            .as_str()
            .unwrap_or_default()
            .to_owned();
        if cur == rivera_private_text {
            break;
        }
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "timed out waiting for Restore my version to restore {rivera_private_text:?}, got {cur:?}"
        );
        std::thread::sleep(Duration::from_millis(60));
    }

    // AC5: Open second browser Tab B as Dr. Chen -> Dr. Chen sees Coda's proposal v1, NOT Dr. Rivera's private draft!
    let mut tab_b = Tab::open(&format!("http://{}/?patient={ws_elena}", harness.address));
    tab_b.sign_in_as("dr.chen@codahealth.test");
    tab_b.wait_for_selector(
        &format!("[data-spruce-reply-card=\"{conv_id}\"]"),
        Duration::from_secs(20),
    );
    let chen_textarea_val = tab_b
        .eval(&format!(
            "document.querySelector('[data-reply-editor=\"{conv_id}\"] textarea').value"
        ))
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert_eq!(
        chen_textarea_val, proposal_v1_text,
        "Dr. Chen must see Coda's proposal v1, never Dr. Rivera's private draft"
    );

    // AC2 + AC5: Dr. Rivera clicks `Copy reply` -> copies displayed text to clipboard, records copy in `Store`,
    // keeps reply OPEN, and exposes `Open in Spruce` link!
    let copied_clipboard = tab_a
        .step(
            "copied",
            json!([format!("[data-copy-reply=\"{conv_id}\"]")]),
        )
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert_eq!(
        copied_clipboard, rivera_private_text,
        "Copy reply must copy exact displayed draft text to clipboard"
    );
    tab_a.wait_for_selector(
        &format!("[data-reply-copied-notice=\"{conv_id}\"]"),
        Duration::from_secs(10),
    );
    let spruce_href = tab_a
        .eval(&format!(
            "document.querySelector('[data-open-in-spruce=\"{conv_id}\"]')?.getAttribute('href') ?? ''"
        ))
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert_eq!(
        spruce_href,
        format!("https://app.sprucehealth.com/conversations/{conv_id}")
    );
    let conv_after_copy = harness
        .store
        .reply_conversation(STAGING_PRACTICE_ID, conv_id)
        .await
        .expect("read reply_conversation")
        .expect("reply_conversation exists");
    assert!(
        matches!(conv_after_copy.reply_standing, ReplyStanding::Ready { .. }),
        "Copy reply must NOT close the reply card!"
    );
    let copy_records = harness
        .store
        .reply_copy_records(STAGING_PRACTICE_ID, conv_id)
        .await
        .expect("read copy records");
    assert_eq!(copy_records.len(), 1);
    assert_eq!(copy_records[0].copied_text, rivera_private_text);

    // Reload Tab B (Dr. Chen) -> Dr. Chen sees Dr. Rivera's copy attribution until a practice message is observed!
    tab_b.reload();
    tab_b.wait_for_selector(
        &format!("[data-other-physician-copy=\"{conv_id}\"]"),
        Duration::from_secs(15),
    );
    let attribution_text = tab_b
        .eval(&format!(
            "document.querySelector('[data-other-physician-copy=\"{conv_id}\"]').innerText"
        ))
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        attribution_text.contains("Copied by Dr. Rivera")
            && !attribution_text.contains(&dr_rivera.physician_id),
        "copy attribution must name the copying physician, never their id:\n{attribution_text}"
    );
    tab_b.save_screenshot("06-two-physician-copy-attribution");

    // Observe a practice message on the channel -> clears prior copy attribution!
    harness
        .store
        .observe_practice_message(&ObservePracticeMessage {
            practice: STAGING_PRACTICE_ID.to_owned(),
            conversation_id: conv_id.to_owned(),
            practice_message_id: "prac-msg-1".to_owned(),
            kind: PracticeMessageKind::PracticeSent,
            text: rivera_private_text.to_owned(),
            sent_at: now_ts - 10,
        })
        .await
        .expect("observe practice message");
    tab_b.reload();
    tab_b.wait_for_selector(
        &format!("[data-spruce-reply-card=\"{conv_id}\"]"),
        Duration::from_secs(15),
    );
    assert_eq!(
        tab_b
            .eval(&format!(
                "Boolean(document.querySelector('[data-other-physician-copy=\"{conv_id}\"]'))"
            ))
            .as_bool(),
        Some(false),
        "copy attribution must clear once a practice message is observed on the channel"
    );

    // AC5: Publish a newer Coda proposal (v2) while Dr. Rivera holds an active private draft ->
    // offers `"Choose update"` (`[data-coda-update-offer]`) WITHOUT replacing Dr. Rivera's edits!
    harness
        .store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: STAGING_PRACTICE_ID.to_owned(),
            conversation_id: conv_id.to_owned(),
            message_id: "msg-marco-2".to_owned(),
            sender: MessageSender::NamedSender {
                name: "Marco Morales (Spouse)".to_owned(),
            },
            text: "Thank you! Should she take lisinopril in the morning or evening?".to_owned(),
            sent_at: now_ts - 5,
        })
        .await
        .expect("record msg-marco-2");
    harness
        .store
        .start_or_read_reply_review(STAGING_PRACTICE_ID, conv_id, "rev-reply-2", "run-reply-2")
        .await
        .expect("start reply review 2");
    let proposal_v2_text =
        "Hi Marco, Elena can take lisinopril 10 mg once daily in the morning at her usual time.";
    harness
        .store
        .complete_reply_review(&CompleteReplyReview {
            practice: STAGING_PRACTICE_ID.to_owned(),
            conversation_id: conv_id.to_owned(),
            review_id: "rev-reply-2".to_owned(),
            run_id: "run-reply-2".to_owned(),
            completion_key: "comp-reply-2".to_owned(),
            answered_closures: Vec::new(),
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: proposal_v2_text.to_owned(),
                basis: ReplyProposalBasisInput {
                    summary: "Follow-up timing question over verified normal BMP.".to_owned(),
                    limits: Vec::new(),
                    cited_records: vec![ref_bmp.clone()],
                },
                cited_records: vec![ref_bmp.clone()],
            }),
            raised_concern_ids: Vec::new(),
        })
        .await
        .expect("complete proposal v2");

    tab_a.reload();
    tab_a.wait_for_selector(
        &format!("[data-coda-update-offer=\"{conv_id}\"]"),
        Duration::from_secs(15),
    );
    let preserved_rivera_val = tab_a
        .eval(&format!(
            "document.querySelector('[data-reply-editor=\"{conv_id}\"] textarea').value"
        ))
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert_eq!(
        preserved_rivera_val, rivera_private_text,
        "newer Coda proposal must NOT overwrite Dr. Rivera's private draft"
    );
    tab_a.save_screenshot("07-reply-coda-update-offer");

    // Click `Choose update` -> adopts Coda's proposal v2!
    tab_a.click(&format!("[data-choose-coda-update=\"{conv_id}\"]"));
    tab_a.wait_until_absent("Choose update", Duration::from_secs(10));
    let chosen_val = tab_a
        .eval(&format!(
            "document.querySelector('[data-reply-editor=\"{conv_id}\"] textarea').value"
        ))
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert_eq!(chosen_val, proposal_v2_text);

    // AC5: New message arrives while Coda reviews -> new message remains visible alongside `"Coda is reviewing"`!
    harness
        .store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: STAGING_PRACTICE_ID.to_owned(),
            conversation_id: conv_id.to_owned(),
            message_id: "msg-marco-3".to_owned(),
            sender: MessageSender::NamedSender {
                name: "Marco Morales (Spouse)".to_owned(),
            },
            text: "Also, is light walking okay today?".to_owned(),
            sent_at: now_ts,
        })
        .await
        .expect("record msg-marco-3");
    harness
        .store
        .start_or_read_reply_review(STAGING_PRACTICE_ID, conv_id, "rev-reply-3", "run-reply-3")
        .await
        .expect("start reply review 3");
    tab_a.reload();
    tab_a.wait_for_selector(
        &format!("[data-coda-reviewing=\"{conv_id}\"]"),
        Duration::from_secs(15),
    );
    tab_a.wait_for_text(
        "Also, is light walking okay today?",
        Duration::from_secs(10),
    );
    tab_a.wait_for_text("Coda is reviewing", Duration::from_secs(10));

    // Finish review 3 so we can exercise `No reply needed` + `Undo` and `Wrong patient?`
    harness
        .store
        .complete_reply_review(&CompleteReplyReview {
            practice: STAGING_PRACTICE_ID.to_owned(),
            conversation_id: conv_id.to_owned(),
            review_id: "rev-reply-3".to_owned(),
            run_id: "run-reply-3".to_owned(),
            completion_key: "comp-reply-3".to_owned(),
            answered_closures: Vec::new(),
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "Hi Marco, Elena can take lisinopril in the morning and light walking today is fine.".to_owned(),
                basis: ReplyProposalBasisInput {
                    summary: "Normal BMP and stable routine activity question.".to_owned(),
                    limits: Vec::new(),
                    cited_records: vec![ref_bmp.clone()],
                },
                cited_records: vec![ref_bmp],
            }),
            raised_concern_ids: Vec::new(),
        })
        .await
        .expect("complete proposal v3");
    tab_a.reload();
    tab_a.wait_for_selector(
        &format!("[data-no-reply-needed=\"{conv_id}\"]"),
        Duration::from_secs(15),
    );

    // AC2: `No reply needed` + `Undo`
    tab_a.click(&format!("[data-no-reply-needed=\"{conv_id}\"]"));
    tab_a.wait_for_selector(
        &format!("[data-closed-undo-banner=\"{conv_id}\"]"),
        Duration::from_secs(15),
    );
    tab_a.click(&format!("[data-undo-no-reply-needed=\"{conv_id}\"]"));
    tab_a.wait_for_selector(
        &format!("[data-reply-editor=\"{conv_id}\"]"),
        Duration::from_secs(15),
    );

    // AC6: `Wrong patient?` + `Undo`, then settled move -> `Withheld`, then disconnected channel -> `Done`!
    tab_a.click(&format!("[data-wrong-patient=\"{conv_id}\"]"));
    tab_a.wait_for_selector(
        "[data-patient-picker-dialog=\"true\"]",
        Duration::from_secs(10),
    );
    tab_a.type_into("[data-patient-picker-dialog=\"true\"] input", "Jake");
    tab_a.wait_for_selector(
        &format!("[data-picker-select=\"{ws_jake}\"]"),
        Duration::from_secs(10),
    );
    tab_a.click(&format!("[data-picker-select=\"{ws_jake}\"]"));
    tab_a.wait_for_selector(
        &format!("[data-pending-move-banner=\"{conv_id}\"]"),
        Duration::from_secs(15),
    );
    // Undo the move!
    tab_a.click(&format!("[data-undo-move=\"{conv_id}\"]"));
    tab_a.wait_for_selector(
        &format!("[data-reply-editor=\"{conv_id}\"]"),
        Duration::from_secs(15),
    );

    // Now move to Jake Patel with a 1-second undo window and settle past expiry so `Withheld` takes effect!
    let move_now = real_now_i64();
    harness
        .store
        .move_spruce_conversation(
            harness.auth.as_ref(),
            &dr_rivera,
            MoveSpruceConversationCommand {
                conversation_id: conv_id.to_owned(),
                destination: MoveDestination::AnotherPatient {
                    workspace: ws_jake.to_owned(),
                },
                undo_window_seconds: 1,
                command_key: "cmd-move-settle-jake".to_owned(),
            },
            None,
            move_now - 5,
        )
        .await
        .expect("move conversation to Jake");
    harness
        .store
        .settle_expired_spruce_moves(STAGING_PRACTICE_ID, None, move_now)
        .await
        .expect("settle expired spruce move");
    harness
        .store
        .act_on_conversation_move(
            &CommandKey::new("move-key-jake-1").expect("valid cmd key"),
            STAGING_PRACTICE_ID,
            conv_id,
            ConversationLink::LinkedPatient {
                workspace: ws_jake.to_owned(),
            },
        )
        .await
        .expect("act on conversation move into Withheld");

    // Navigate Tab A to Jake Patel -> reply card shows `Withheld` (zero leaked Elena reply words)!
    tab_a.navigate(&format!("http://{}/?patient={ws_jake}", harness.address));
    tab_a.wait_for_selector(
        &format!("[data-reply-withheld=\"{conv_id}\"]"),
        Duration::from_secs(20),
    );
    let jake_page_text = tab_a
        .eval("document.body?.innerText ?? ''")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    // The moved conversation's own history (incoming messages and the sent practice reply)
    // travels with it; only Coda's proposals written against Elena's chart are withheld.
    for withheld_proposal in [
        proposal_v1_text,
        proposal_v2_text,
        coda_revised_text,
        "Hi Marco, Elena can take lisinopril in the morning and light walking today is fine.",
    ] {
        assert!(
            !jake_page_text.contains(withheld_proposal),
            "withheld reply card on Jake Patel must never show a proposal written for Elena ({withheld_proposal:?}):\n{jake_page_text}"
        );
    }
    tab_a.save_screenshot("08-reply-withheld-after-move");

    // AC6: Disconnect Spruce channel -> card stays readable under current access, links to Spruce, and offers `Done`!
    harness
        .store
        .disconnect_spruce_channel(harness.auth.as_ref(), &dr_rivera, real_now_i64())
        .await
        .expect("disconnect spruce channel");
    harness
        .store
        .set_reply_channel_connection(
            STAGING_PRACTICE_ID,
            conv_id,
            ChannelConnection::Disconnected {
                disconnected_at: real_now_i64(),
                note: "Spruce channel is disconnected. Open in Spruce to reply manually, or mark Done.".to_owned(),
            },
        )
        .await
        .expect("set reply channel disconnected");

    tab_a.reload();
    tab_a.wait_for_selector(
        &format!("[data-disconnected-reply-banner=\"{conv_id}\"]"),
        Duration::from_secs(15),
    );
    tab_a.wait_for_selector(
        &format!("[data-disconnected-reply-done=\"{conv_id}\"]"),
        Duration::from_secs(10),
    );
    tab_a.save_screenshot("09-reply-disconnected-done");

    tab_a.click(&format!("[data-disconnected-reply-done=\"{conv_id}\"]"));
    tab_a.wait_until_absent("Spruce channel is disconnected.", Duration::from_secs(15));
    let closed_conv = harness
        .store
        .reply_conversation(STAGING_PRACTICE_ID, conv_id)
        .await
        .expect("read reply_conversation")
        .expect("reply_conversation exists");
    assert_eq!(closed_conv.reply_standing, ReplyStanding::None);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "launches headless Chromium"]
async fn failed_commands_lookup_unavailable_and_lost_access_closure() {
    let _guard = test_lock().lock().await;
    let harness = RunningHarness::start("faults_and_access").await;

    let ws_elena = WORKSPACE_IDS[0];
    let ws_jake = WORKSPACE_IDS[1];

    let ref_elena = harness
        .seed_patient_with_chart(
            ws_elena,
            "Elena Morales",
            "1968-05-14",
            "MRN-3001",
            "obs-fault-elena",
            "rev-1",
            "Potassium 4.3 mmol/L.",
        )
        .await;
    harness
        .seed_patient_with_chart(
            ws_jake,
            "Jake Patel",
            "1975-11-02",
            "MRN-3002",
            "obs-fault-jake",
            "rev-1",
            "BP 122/76 mmHg.",
        )
        .await;

    let dr_rivera = harness
        .enroll_physician(
            "dr.rivera@codahealth.test",
            "Dr. Rivera",
            &[ws_elena, ws_jake],
        )
        .await;

    let lease_token =
        connect_spruce_and_lease(&harness.store, harness.auth.as_ref(), &dr_rivera).await;
    let now_ts = real_now_i64();

    // Seed one unmatched message in `Waiting` (`msg-fault-match`)
    harness
        .store
        .ingest_spruce_item(
            STAGING_PRACTICE_ID,
            &lease_token,
            &make_readable_spruce_item(
                1,
                "msg-fault-match",
                "conv-fault-match",
                Some("+14155550201"),
                Some("Elena Morales"),
                "Checking whether my lab order was sent.",
                Vec::new(),
                Vec::new(),
            ),
            now_ts - 40,
        )
        .await
        .expect("ingest msg-fault-match");
    harness
        .store
        .record_spruce_message_classification(
            harness.auth.as_ref(),
            STAGING_PRACTICE_ID,
            "msg-fault-match",
            None,
            "spruce-message-matching/v1",
            "spruce-matching-guidance/v1",
            local_product::runtime::spruce_matching::MatchingDecision::PatientMail {
                urgency: local_product::runtime::spruce_matching::MessageUrgencyChoice::Ordinary,
                assessment:
                    local_product::runtime::spruce_matching::MatchAssessment::SuggestedPatient {
                        workspace: ws_elena.to_owned(),
                        evidence_summary: "Matches Elena Morales.".to_owned(),
                    },
                rationale: "Lab order question.".to_owned(),
            },
            None,
            now_ts - 35,
        )
        .await
        .expect("classify msg-fault-match");

    // Seed one open reply card on `ws_elena` (`conv-fault-reply`)
    let conv_id = "conv-fault-reply";
    harness
        .store
        .ingest_spruce_item(
            STAGING_PRACTICE_ID,
            &lease_token,
            &make_readable_spruce_item(
                2,
                "msg-fault-reply",
                conv_id,
                Some("+14155550202"),
                Some("Elena Morales"),
                "Can I take ibuprofen for shoulder soreness?",
                Vec::new(),
                Vec::new(),
            ),
            now_ts - 30,
        )
        .await
        .expect("ingest msg-fault-reply");
    harness
        .store
        .record_spruce_message_classification(
            harness.auth.as_ref(),
            STAGING_PRACTICE_ID,
            "msg-fault-reply",
            None,
            "spruce-message-matching/v1",
            "spruce-matching-guidance/v1",
            local_product::runtime::spruce_matching::MatchingDecision::PatientMail {
                urgency: local_product::runtime::spruce_matching::MessageUrgencyChoice::Ordinary,
                assessment:
                    local_product::runtime::spruce_matching::MatchAssessment::SuggestedPatient {
                        workspace: ws_elena.to_owned(),
                        evidence_summary: "Matches Elena Morales.".to_owned(),
                    },
                rationale: "Patient NSAID question.".to_owned(),
            },
            None,
            now_ts - 29,
        )
        .await
        .expect("classify msg-fault-reply into Waiting");
    harness
        .store
        .confirm_spruce_message_link(
            harness.auth.as_ref(),
            &dr_rivera,
            "msg-fault-reply",
            ws_elena,
            SenderConfirmationPolicy::ConversationOnly,
            "cmd-confirm-fault-reply",
            None,
            now_ts - 28,
        )
        .await
        .expect("confirm conv-fault-reply");
    harness
        .store
        .link_reply_conversation(
            STAGING_PRACTICE_ID,
            conv_id,
            ConversationLink::LinkedPatient {
                workspace: ws_elena.to_owned(),
            },
        )
        .await
        .expect("link reply conversation");
    harness
        .store
        .record_linked_patient_message(&RecordPatientMessage {
            practice: STAGING_PRACTICE_ID.to_owned(),
            conversation_id: conv_id.to_owned(),
            message_id: "msg-fault-reply".to_owned(),
            sender: MessageSender::Patient,
            text: "Can I take ibuprofen for shoulder soreness?".to_owned(),
            sent_at: now_ts - 30,
        })
        .await
        .expect("record linked message");
    harness
        .store
        .start_or_read_reply_review(STAGING_PRACTICE_ID, conv_id, "rev-fault-1", "run-fault-1")
        .await
        .expect("start review");
    harness
        .store
        .complete_reply_review(&CompleteReplyReview {
            practice: STAGING_PRACTICE_ID.to_owned(),
            conversation_id: conv_id.to_owned(),
            review_id: "rev-fault-1".to_owned(),
            run_id: "run-fault-1".to_owned(),
            completion_key: "comp-fault-1".to_owned(),
            answered_closures: Vec::new(),
            decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                text: "SECRET_CLINICAL_REPLY_ELENA: Avoid NSAIDs like ibuprofen while taking lisinopril; acetaminophen is preferred.".to_owned(),
                basis: ReplyProposalBasisInput {
                    summary: "Avoid NSAID + ACEi interaction.".to_owned(),
                    limits: Vec::new(),
                    cited_records: vec![ref_elena.clone()],
                },
                cited_records: vec![ref_elena],
            }),
            raised_concern_ids: Vec::new(),
        })
        .await
        .expect("complete proposal");

    let mut tab = Tab::open(&format!("http://{}/", harness.address));
    tab.sign_in_as("dr.rivera@codahealth.test");
    tab.wait_for_selector(
        "[data-nav-messages-to-match=\"true\"]",
        Duration::from_secs(20),
    );
    tab.click("[data-nav-messages-to-match=\"true\"]");
    tab.wait_for_selector(
        "[data-confirm-match=\"msg-fault-match\"]",
        Duration::from_secs(15),
    );

    // Force `MatchingCommand` failure -> `Confirm` shows `"Not saved"`!
    harness
        .faults
        .fail(MatchingReplyFollowUpsFault::MatchingCommand);
    tab.click("[data-confirm-match=\"msg-fault-match\"]");
    tab.wait_for_selector(
        "[data-matching-command-error=\"true\"]",
        Duration::from_secs(10),
    );
    tab.wait_for_text("Not saved", Duration::from_secs(10));
    harness
        .faults
        .clear_fault(MatchingReplyFollowUpsFault::MatchingCommand);

    // Force `PatientLookup` failure -> `Choose another patient` shows `"Coda couldn’t search patients."`, NEVER `"No patients match."`!
    harness
        .faults
        .fail(MatchingReplyFollowUpsFault::PatientLookup);
    tab.click("[data-choose-another-patient=\"msg-fault-match\"]");
    tab.wait_for_selector(
        "[data-picker-lookup-error=\"true\"]",
        Duration::from_secs(10),
    );
    let dialog_text = tab
        .eval("document.querySelector('[data-patient-picker-dialog=\"true\"]').innerText")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        dialog_text.contains("couldn’t search patients")
            || dialog_text.contains("couldn't search patients"),
        "expected explicit lookup failure notice, got:\n{dialog_text}"
    );
    assert!(
        !dialog_text.contains("No patients match"),
        "failed lookup must NEVER claim No patients match:\n{dialog_text}"
    );
    tab.save_screenshot("10-patient-picker-lookup-failure");

    // Clear `PatientLookup` fault and retry -> patients load cleanly!
    harness
        .faults
        .clear_fault(MatchingReplyFollowUpsFault::PatientLookup);
    tab.click("[data-picker-lookup-retry=\"true\"]");
    tab.wait_for_selector(
        &format!("[data-picker-select=\"{ws_jake}\"]"),
        Duration::from_secs(10),
    );

    // Navigate to Elena Morales -> edit draft -> force `ReplyCommand` fault -> `"Not saved"` + edits preserved!
    tab.navigate(&format!("http://{}/?patient={ws_elena}", harness.address));
    tab.wait_for_selector(
        &format!("[data-reply-editor=\"{conv_id}\"] textarea"),
        Duration::from_secs(20),
    );
    let unsaved_draft = "PHYSICIAN_UNSAVED_EDIT_PRESERVED_ON_FAILURE: Use acetaminophen 500 mg instead of ibuprofen.";
    tab.type_into(
        &format!("[data-reply-editor=\"{conv_id}\"] textarea"),
        unsaved_draft,
    );
    harness
        .faults
        .fail(MatchingReplyFollowUpsFault::ReplyCommand);
    tab.click(&format!("[data-save-reply-draft=\"{conv_id}\"]"));
    tab.wait_for_selector(
        &format!("[data-reply-command-error=\"{conv_id}\"]"),
        Duration::from_secs(10),
    );
    tab.wait_for_text("Not saved", Duration::from_secs(10));
    let preserved = tab
        .eval(&format!(
            "document.querySelector('[data-reply-editor=\"{conv_id}\"] textarea').value"
        ))
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert_eq!(
        preserved, unsaved_draft,
        "failed reply command must preserve physician draft edits"
    );
    tab.save_screenshot("11-reply-command-failure-preserves-draft");
    harness
        .faults
        .clear_fault(MatchingReplyFollowUpsFault::ReplyCommand);

    // Remove Dr. Rivera's panel access to `ws_elena` -> readable clinical content closes immediately and refuses further actions!
    harness
        .auth
        .set_membership(&dr_rivera, ws_elena, Membership::ClinicDemographic)
        .await
        .expect("revoke panel membership");
    // Attempt an action on the open reply card -> 404 triggers immediate `on_patient_gone` closure!
    tab.click(&format!("[data-save-reply-draft=\"{conv_id}\"]"));
    tab.wait_until_absent("SECRET_CLINICAL_REPLY_ELENA", Duration::from_secs(15));
    tab.wait_until_absent(unsaved_draft, Duration::from_secs(15));
    let after_revoke_text = tab
        .eval("document.body?.innerText ?? ''")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        !after_revoke_text.contains("SECRET_CLINICAL_REPLY_ELENA"),
        "revoking panel access must immediately close readable reply content:\n{after_revoke_text}"
    );
    tab.save_screenshot("12-lost-patient-access-closed");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "launches headless Chromium"]
async fn follow_ups_restart_controls_faults_cannot_restart_and_responsive_layout() {
    let _guard = test_lock().lock().await;
    let mut harness = RunningHarness::start("follow_ups").await;

    let ws_elena = WORKSPACE_IDS[0];
    harness
        .seed_patient_with_chart(
            ws_elena,
            "Elena Morales",
            "1968-05-14",
            "MRN-4001",
            "obs-fu-elena",
            "rev-1",
            "Baseline BMP potassium 4.5 mmol/L, creatinine 1.0 mg/dL prior to lisinopril uptitration.",
        )
        .await;
    let _dr_rivera = harness
        .enroll_physician("dr.rivera@codahealth.test", "Dr. Rivera", &[ws_elena])
        .await;

    let mut tab = Tab::open(&format!("http://{}/?patient={ws_elena}", harness.address));
    tab.sign_in_as("dr.rivera@codahealth.test");
    tab.wait_for_selector(
        "[data-patient-follow-ups-section=\"true\"]",
        Duration::from_secs(20),
    );

    // AC1: Create a follow-up, edit it, restart the server, and reopen -> saved result shows!
    tab.click("[data-open-create-follow-up=\"true\"]");
    tab.wait_for_selector(
        "[data-create-follow-up-card=\"true\"]",
        Duration::from_secs(10),
    );
    tab.type_into(
        "[data-create-follow-up-summary=\"true\"] input",
        "Recheck metabolic panel after lisinopril titration",
    );
    tab.type_into(
        "[data-create-follow-up-due-date=\"true\"] input",
        "2026-09-10",
    );
    tab.click("[data-submit-create-follow-up=\"true\"]");
    tab.wait_for_selector(
        "[data-follow-up-standing=\"scheduled\"]",
        Duration::from_secs(15),
    );

    let fid1 = tab
        .eval(
            "document.querySelector('[data-follow-up-card]')?.getAttribute('data-follow-up-card') ?? ''",
        )
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(!fid1.is_empty(), "created follow-up id must be present");

    // Edit summary and due date
    tab.click(&format!("[data-follow-up-edit=\"{fid1}\"]"));
    tab.wait_for_selector(
        &format!("[data-follow-up-edit-form=\"{fid1}\"]"),
        Duration::from_secs(10),
    );
    let edited_summary = "Recheck BMP potassium and creatinine after lisinopril 20 mg";
    tab.type_into(
        &format!("[data-edit-follow-up-summary=\"{fid1}\"] input"),
        edited_summary,
    );
    tab.type_into(
        &format!("[data-edit-follow-up-due-date=\"{fid1}\"] input"),
        "2026-09-14",
    );
    tab.click(&format!("[data-save-follow-up-edit=\"{fid1}\"]"));
    tab.wait_for_text(edited_summary, Duration::from_secs(15));

    // Restart the Axum server over the same PostgreSQL database and reload the page!
    harness.restart_server().await;
    tab.reload();
    tab.wait_for_selector(
        &format!("[data-follow-up-card=\"{fid1}\"]"),
        Duration::from_secs(20),
    );
    tab.wait_for_text(edited_summary, Duration::from_secs(10));
    tab.eval(&format!(
        "document.querySelector('[data-follow-up-card=\"{fid1}\"]').scrollIntoView({{block: 'center'}})"
    ));
    std::thread::sleep(Duration::from_millis(80));
    tab.save_screenshot("13-follow-up-persisted-after-restart");

    // AC1: `Review now` marks the follow-up `DueNow` AND starts/queues a patient chart review!
    tab.click(&format!("[data-follow-up-review-now=\"{fid1}\"]"));
    tab.wait_for_selector(
        &format!("[data-follow-up-card=\"{fid1}\"][data-follow-up-standing=\"due_now\"]"),
        Duration::from_secs(15),
    );
    tab.wait_for_selector(
        &format!("[data-follow-up-review-started=\"{fid1}\"]"),
        Duration::from_secs(10),
    );
    let practice_auth = TestPracticeAuthority(harness.auth.as_ref());
    let reviews = harness
        .store
        .patient_reviews(STAGING_PRACTICE_ID, ws_elena, &practice_auth)
        .await
        .expect("read workspace reviews");
    assert!(
        !reviews.is_empty(),
        "Review now must submit a durable patient chart review"
    );
    tab.eval(&format!(
        "document.querySelector('[data-follow-up-card=\"{fid1}\"]').scrollIntoView({{block: 'center'}})"
    ));
    std::thread::sleep(Duration::from_millis(80));
    tab.save_screenshot("14-follow-up-review-now");

    // AC2: Forced failure shows `"Not saved"` and preserves edits!
    tab.click(&format!("[data-follow-up-edit=\"{fid1}\"]"));
    tab.wait_for_selector(
        &format!("[data-edit-follow-up-summary=\"{fid1}\"] input"),
        Duration::from_secs(10),
    );
    let unsaved_fu_edit = "UNSAVED_FOLLOW_UP_EDIT_PRESERVED_ON_ERROR";
    tab.type_into(
        &format!("[data-edit-follow-up-summary=\"{fid1}\"] input"),
        unsaved_fu_edit,
    );
    harness
        .faults
        .fail(MatchingReplyFollowUpsFault::FollowUpCommand);
    tab.click(&format!("[data-save-follow-up-edit=\"{fid1}\"]"));
    tab.wait_for_selector(
        &format!("[data-follow-up-command-error=\"{fid1}\"]"),
        Duration::from_secs(10),
    );
    tab.wait_for_text("Not saved", Duration::from_secs(10));
    let preserved_fu_val = tab
        .eval(&format!(
            "document.querySelector('[data-edit-follow-up-summary=\"{fid1}\"] input').value"
        ))
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert_eq!(
        preserved_fu_val, unsaved_fu_edit,
        "failed follow-up edit must preserve unsaved text"
    );
    tab.eval(&format!(
        "document.querySelector('[data-follow-up-card=\"{fid1}\"]').scrollIntoView({{block: 'center'}})"
    ));
    std::thread::sleep(Duration::from_millis(80));
    tab.save_screenshot("15-follow-up-failure-not-saved");
    harness
        .faults
        .clear_fault(MatchingReplyFollowUpsFault::FollowUpCommand);
    tab.click(&format!("[data-cancel-follow-up-edit=\"{fid1}\"]"));

    // AC2: `Stop` -> `Held`, then `Resume` -> `DueNow`
    tab.click(&format!("[data-follow-up-stop=\"{fid1}\"]"));
    tab.wait_for_selector(
        &format!("[data-follow-up-card=\"{fid1}\"][data-follow-up-standing=\"held\"]"),
        Duration::from_secs(15),
    );
    tab.click(&format!("[data-follow-up-resume=\"{fid1}\"]"));
    tab.wait_for_selector(
        &format!("[data-follow-up-card=\"{fid1}\"][data-follow-up-standing=\"due_now\"]"),
        Duration::from_secs(15),
    );

    // AC2: `Stop and discuss` holds the follow-up AND opens a durable patient conversation seeded with its context!
    tab.click(&format!("[data-follow-up-stop-discuss=\"{fid1}\"]"));
    tab.wait_for_selector(
        &format!("[data-follow-up-card=\"{fid1}\"][data-follow-up-standing=\"held\"]"),
        Duration::from_secs(15),
    );
    tab.wait_for_text(
        "Let's discuss held follow-up \"Recheck BMP potassium and creatinine after lisinopril 20 mg\"",
        Duration::from_secs(20),
    );
    let stored_fu_after_discuss = harness
        .store
        .workspace_follow_ups(STAGING_PRACTICE_ID, ws_elena)
        .await
        .expect("read workspace follow-ups")
        .into_iter()
        .find(|f| f.id.as_str() == fid1)
        .expect("fid1 exists");
    assert!(
        matches!(
            stored_fu_after_discuss.standing,
            FollowUpStanding::Held { .. }
        ),
        "follow-up must remain Held after Stop and discuss until Resume"
    );
    tab.eval(&format!(
        "document.querySelector('[data-follow-up-card=\"{fid1}\"]').scrollIntoView({{block: 'center'}})"
    ));
    std::thread::sleep(Duration::from_millis(80));
    tab.save_screenshot("16-follow-up-stop-and-discuss");

    // `Resume` then `Complete` on `fid1`
    tab.click(&format!("[data-follow-up-resume=\"{fid1}\"]"));
    tab.wait_for_selector(
        &format!("[data-follow-up-card=\"{fid1}\"][data-follow-up-standing=\"due_now\"]"),
        Duration::from_secs(15),
    );
    tab.click(&format!("[data-follow-up-complete=\"{fid1}\"]"));
    tab.wait_for_selector(
        &format!("[data-follow-up-card=\"{fid1}\"][data-follow-up-standing=\"completed\"]"),
        Duration::from_secs(15),
    );

    // Create a second follow-up and exercise `Cancel`
    tab.click("[data-open-create-follow-up=\"true\"]");
    tab.wait_for_selector(
        "[data-create-follow-up-card=\"true\"]",
        Duration::from_secs(10),
    );
    tab.type_into(
        "[data-create-follow-up-summary=\"true\"] input",
        "Confirm home blood pressure cuff delivery",
    );
    tab.type_into(
        "[data-create-follow-up-due-date=\"true\"] input",
        "2026-09-20",
    );
    tab.click("[data-submit-create-follow-up=\"true\"]");
    tab.wait_for_text(
        "Confirm home blood pressure cuff delivery",
        Duration::from_secs(15),
    );
    let fid2 = tab
        .eval(
            "[...document.querySelectorAll('[data-follow-up-card]')].find(el => el.getAttribute('data-follow-up-standing') === 'scheduled')?.getAttribute('data-follow-up-card') ?? ''",
        )
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(!fid2.is_empty());
    tab.click(&format!("[data-follow-up-cancel=\"{fid2}\"]"));
    tab.wait_for_selector(
        &format!("[data-follow-up-card=\"{fid2}\"][data-follow-up-standing=\"cancelled\"]"),
        Duration::from_secs(15),
    );

    // AC3: Create a third follow-up, transition into `CannotRestart` ("Can't restart"), verify `Complete`, `New date`, and `Cancel`,
    // and verify desktop (1440x960) + phone (390x844, zero sideways scroll)!
    tab.click("[data-open-create-follow-up=\"true\"]");
    tab.wait_for_selector(
        "[data-create-follow-up-card=\"true\"]",
        Duration::from_secs(10),
    );
    tab.type_into(
        "[data-create-follow-up-summary=\"true\"] input",
        "Quarterly A1c surveillance after discharge",
    );
    tab.type_into(
        "[data-create-follow-up-due-date=\"true\"] input",
        "2026-09-25",
    );
    tab.click("[data-submit-create-follow-up=\"true\"]");
    tab.wait_for_text(
        "Quarterly A1c surveillance after discharge",
        Duration::from_secs(15),
    );
    let fid3 = tab
        .eval(
            "[...document.querySelectorAll('[data-follow-up-card]')].find(el => el.getAttribute('data-follow-up-standing') === 'scheduled')?.getAttribute('data-follow-up-card') ?? ''",
        )
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(!fid3.is_empty());

    harness
        .store
        .apply_follow_up_review_outcome(
            STAGING_PRACTICE_ID,
            ws_elena,
            &FollowUpCommandKey::new("cmd-mark-cannot-restart").expect("valid cmd key"),
            &FollowUpId::new(&fid3).expect("valid fid3"),
            SourceWork::new("patient_review", "work-fu-cr", "att-fu-cr").expect("source work"),
            FollowUpReviewOutcome::CannotRestart {
                failed_work_id: "work-fu-cr".to_owned(),
                reason: "Recurring A1c schedule needs a new due date after inpatient stay."
                    .to_owned(),
            },
        )
        .await
        .expect("mark follow-up CannotRestart");

    tab.reload();
    tab.wait_for_selector(
        &format!("[data-follow-up-card=\"{fid3}\"][data-follow-up-standing=\"cannot_restart\"]"),
        Duration::from_secs(15),
    );
    tab.wait_for_selector(
        &format!("[data-cannot-restart-new-date=\"{fid3}\"]"),
        Duration::from_secs(10),
    );
    tab.wait_for_selector(
        &format!("[data-cannot-restart-complete=\"{fid3}\"]"),
        Duration::from_secs(10),
    );
    tab.wait_for_selector(
        &format!("[data-cannot-restart-cancel=\"{fid3}\"]"),
        Duration::from_secs(10),
    );
    tab.wait_for_text("Can't restart", Duration::from_secs(10));

    // Desktop (1440x960) layout check + screenshot
    let reveal_cannot_restart = format!(
        "document.querySelector('[data-follow-up-card=\"{fid3}\"]').scrollIntoView({{block: 'center'}})"
    );
    tab.set_viewport(1440, 960, true);
    std::thread::sleep(Duration::from_millis(100));
    tab.assert_no_horizontal_overflow("Patient follow-ups desktop 1440x960");
    tab.eval(&reveal_cannot_restart);
    std::thread::sleep(Duration::from_millis(80));
    tab.save_screenshot("17-follow-up-cannot-restart-desktop");

    // Phone (390x844) layout check + screenshot
    tab.set_viewport(390, 844, true);
    std::thread::sleep(Duration::from_millis(120));
    tab.assert_no_horizontal_overflow("Patient follow-ups phone 390x844");
    tab.eval(&reveal_cannot_restart);
    std::thread::sleep(Duration::from_millis(80));
    tab.save_screenshot("18-follow-up-cannot-restart-phone");

    // Restore desktop and resolve `CannotRestart` via `New date`!
    tab.set_viewport(1440, 960, true);
    std::thread::sleep(Duration::from_millis(80));
    tab.type_into(
        &format!("[data-cannot-restart-date=\"{fid3}\"] input"),
        "2026-10-15",
    );
    tab.click(&format!("[data-cannot-restart-new-date=\"{fid3}\"]"));
    tab.wait_for_selector(
        &format!("[data-follow-up-card=\"{fid3}\"][data-follow-up-standing=\"scheduled\"]"),
        Duration::from_secs(15),
    );
}
