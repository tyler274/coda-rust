//! Chromium browser qualification over PostgreSQL for wrong-patient link correction
//! withholding in history, open conversation detail, open citation inspection, and
//! open reply-draft detail across `Undo`, unfinished removal, and settled states.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex as StdMutex, OnceLock, Weak};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use base64::Engine as _;
use front_door::PostgresFrontDoorStore;
use local_product::ModelRoles;
use local_product::physician_chat::{ChatAssignment, PhysicianSkills};
use local_product::physician_sources::{
    MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
use local_product::runtime::concerns::{ExactRecordReference, WorkspaceAccess};
use local_product::runtime::delivery::Delivery;
use local_product::runtime::dispatch::ReviewReceiver;
use local_product::runtime::jobs::PracticeJobError;
use local_product::runtime::link_correction::{
    CorrectPatientLinkCommand, CorrectionDestination, LinkConversationCommand,
    LinkCorrectionOutcome, LinkCorrectionRemovalProbe, LinkCorrectionStanding, LinkCorrectionStep,
};
use local_product::runtime::reply::proposals::{
    CompleteReplyReview, ConversationLink, ExpectedDraftBase, MessageSender, RecordPatientMessage,
    ReplyProposalBasisInput, ReplyProposalInput, ReplyReviewDecision,
};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::spruce_matching::{
    IncomingSpruceItem, MatchAssessment, MatchingDecision, MessageUrgencyChoice,
    OutsideParticipant, ReadableSpruceItem, SenderAddress, SenderConfirmationPolicy,
};
use local_product::runtime::{
    Accepted, Authority, Input, PhysicianHost, PublicationTransaction, Scope, Store, async_trait,
    engine::{
        self, Invocation, Item, Turn,
        recipe::{Observer, Provider, Request},
    },
};
use local_product::sha256_hex;
use local_product::spruce_matching_recipe::{GUIDANCE_VERSION, RECIPE_VERSION};
use patient_work::ReviewRequest;
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use physician_sample::STAGING_PRACTICE_ID;
use physician_sample::durable::DurableChat;
use physician_sample::queue::ChatQueue;
use sea_orm::ConnectionTrait;
use serde_json::{Value, json};
use tokio::sync::Mutex;
use uuid::Uuid;

#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;

use devtools::Browser;

const CLINICAL_NOW_UNIX: i64 = 1_787_846_400;
const WRONG_PATIENT_MESSAGE_SECRET: &str = "WRONG_PATIENT_PENICILLIN_RASH_SECRET";
const WRONG_PATIENT_DRAFT_SECRET: &str = "WRONG_PATIENT_DRAFT_BODY_SECRET";
const CLEAN_PATIENT_BASELINE_NOTE: &str = "CLEAN_CLARA_LISINOPRIL_BASELINE_10MG";

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
static BUILT_SITE: OnceLock<PathBuf> = OnceLock::new();

fn test_lock() -> &'static Mutex<()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(()))
}

fn real_now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or(1_787_846_400)
}

#[derive(Default)]
struct HoldingChatQueue {
    deliveries: StdMutex<Vec<Delivery>>,
}

#[async_trait]
impl ChatQueue for HoldingChatQueue {
    async fn publish(&self, delivery: &Delivery) -> Result<(), String> {
        self.deliveries
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push(delivery.clone());
        Ok(())
    }
}

#[derive(Default)]
struct RecordingReviewReceiver {
    accepted: StdMutex<BTreeMap<(String, String), ReviewRequest>>,
}

#[async_trait]
impl ReviewReceiver for RecordingReviewReceiver {
    async fn submit_review(&self, request: &ReviewRequest) -> Result<(), PracticeJobError> {
        self.accepted
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .entry((request.practice_id.clone(), request.command_key.clone()))
            .or_insert_with(|| request.clone());
        Ok(())
    }
}

struct StepFailureProbe {
    step: LinkCorrectionStep,
    reason: String,
}

impl LinkCorrectionRemovalProbe for StepFailureProbe {
    fn before_step(
        &self,
        _correction_id: &str,
        _conversation_id: &str,
        step: LinkCorrectionStep,
    ) -> Result<(), String> {
        if step == self.step {
            return Err(self.reason.clone());
        }
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

fn free_loopback_port() -> u16 {
    use std::net::TcpListener;
    for _ in 0..500 {
        let port = 25_000 + (Uuid::new_v4().as_u128() % 6_000) as u16;
        if TcpListener::bind(("127.0.0.1", port)).is_ok() {
            return port;
        }
    }
    panic!("no free loopback port for test cluster");
}

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
        "coda-site-link-corr-pg-{}-{}",
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
        let db_name = format!("site_link_corr_{label}_{}", Uuid::new_v4().simple());
        let admin = sea_orm::Database::connect(&admin_url)
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

struct RunningHarness {
    _db: DisposableDb,
    address: SocketAddr,
    store: Store,
    auth: Arc<PostgresAuthStore>,
    bus: Arc<dyn physician_bus::Bus>,
    server_task: tokio::task::JoinHandle<()>,
}

impl Drop for RunningHarness {
    fn drop(&mut self) {
        self.server_task.abort();
    }
}

impl RunningHarness {
    async fn start_after<T, F, Fut>(label: &str, seed: F) -> (Self, T)
    where
        F: FnOnce(Store, Arc<PostgresAuthStore>) -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let site = build_site();
        // SAFETY: serialized by `test_lock()` held across each test.
        unsafe {
            env::set_var("PHYSICIAN_SITE_DIR", &site);
        }
        let disposable = DisposableDb::create(label).await;
        let store = Store::connect(&disposable.url)
            .await
            .expect("connect store");
        let auth = Arc::new(
            PostgresAuthStore::connect_without_migrations(&disposable.url)
                .await
                .expect("connect auth store"),
        );
        auth.ensure_practice(
            STAGING_PRACTICE_ID,
            PracticeClock::Fixed(CLINICAL_NOW_UNIX as u64),
        )
        .await
        .expect("ensure staging practice");

        let seeded = seed(store.clone(), Arc::clone(&auth)).await;

        let queue: Arc<dyn ChatQueue> = Arc::new(HoldingChatQueue::default());
        let skills = PhysicianSkills::configured(None, false).expect("load workspace skills");
        let roles = test_model_roles();
        let chat = Arc::new(DurableChat::new(store.clone(), queue, skills, &roles));
        let bus: Arc<dyn physician_bus::Bus> = Arc::new(physician_bus::LocalBus::new(256));

        let base_router = physician_sample::durable_site(chat, Arc::clone(&bus));
        let app = physician_sample::authed_postgres_router(
            base_router,
            &disposable.url,
            store.clone(),
            Arc::clone(&bus),
        )
        .await;

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind listener");
        let address = listener.local_addr().expect("local addr");
        let server_task = tokio::spawn(async move {
            let _ = axum::serve(listener, app).await;
        });
        wait_until_serving(address).await;

        (
            Self {
                _db: disposable,
                address,
                store,
                auth,
                bus,
                server_task,
            },
            seeded,
        )
    }
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
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/verification/shots")
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
        tab.set_viewport(1440, 900, true);
        tab.browser.page().navigate(url);
        tab
    }

    fn navigate(&mut self, url: &str) {
        self.browser.page().navigate(url);
    }

    fn call(&mut self, method: &str, params: serde_json::Value) -> serde_json::Value {
        self.browser.page().call_session(method, params)
    }

    fn step(&mut self, name: &str, args: serde_json::Value) -> serde_json::Value {
        let mut arguments = vec![serde_json::json!(name)];
        arguments.extend(args.as_array().expect("step args").iter().cloned());
        self.browser
            .page()
            .call_function(include_str!("js/roger_qa_tail.js"), &arguments)
    }

    fn click(&mut self, selector: &str) {
        self.browser.click(selector);
    }

    fn click_containing(&mut self, selector: &str, words: &str) {
        self.step("clickContaining", serde_json::json!([selector, words]));
    }

    fn sign_in_as(&mut self, email: &str) {
        self.wait_for_text("Work email", Duration::from_secs(20));
        self.browser.type_into("input[type=\"email\"]", email);
        std::thread::sleep(Duration::from_millis(60));
        self.click("button.control-primary");
    }

    fn visible_snapshot(&mut self) -> (String, String) {
        let snapshot = self.step("visibleSnapshot", serde_json::json!([]));
        let text = snapshot["text"].as_str().unwrap_or_default().to_owned();
        let html = snapshot["html"].as_str().unwrap_or_default().to_owned();
        (text, html)
    }

    fn wait_for_text(&mut self, expected: &str, timeout: Duration) -> (String, String) {
        let started = Instant::now();
        loop {
            let (text, html) = self.visible_snapshot();
            if text.contains(expected) {
                return (text, html);
            }
            assert!(
                started.elapsed() < timeout,
                "timed out waiting for {expected:?} in visible text; visible text:\n{text}\nDOM:\n{html}"
            );
            std::thread::sleep(Duration::from_millis(60));
        }
    }

    fn assert_no_stale_secrets(&mut self, context: &str) {
        let (text, html) = self.visible_snapshot();
        for forbidden in [WRONG_PATIENT_MESSAGE_SECRET, WRONG_PATIENT_DRAFT_SECRET] {
            assert!(
                !text.contains(forbidden) && !html.contains(forbidden),
                "stale wrong-patient content {forbidden:?} leaked in {context}!\nvisible text:\n{text}\nDOM:\n{html}"
            );
        }
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
        let metrics = self.step("overflowMetrics", serde_json::json!([]));
        let scroll_w = metrics["scrollWidth"].as_f64().unwrap_or(9999.0);
        let client_w = metrics["clientWidth"].as_f64().unwrap_or(0.0);
        assert!(
            scroll_w <= client_w + 1.0,
            "horizontal overflow on {context}: scrollWidth={scroll_w}, clientWidth={client_w}"
        );
    }
}

async fn enroll_physician(
    auth: &PostgresAuthStore,
    practice_id: &str,
    email: &str,
    display_name: &str,
    workspaces: &[String],
) -> (PhysicianIdentity, Vec<u8>) {
    let now = real_now_unix();
    let invite = auth
        .invite(practice_id, email, display_name, false, now)
        .await
        .expect("invite physician");
    let (identity, token) = auth
        .redeem_invitation(email, &invite, now)
        .await
        .expect("redeem invitation");
    for ws in workspaces {
        auth.set_membership(&identity, ws, Membership::Allowed)
            .await
            .expect("grant panel membership");
    }
    let session_hash = PostgresAuthStore::session_reference(&token);
    (identity, session_hash)
}

#[allow(clippy::too_many_arguments)]
fn chart_revision(
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
        record_type: "Communication".to_owned(),
        dates: vec![RecordDate {
            name: "effectiveDateTime".to_owned(),
            value: date.to_owned(),
        }],
        sha256: sha256_hex(text.as_bytes()),
        content: text.to_owned(),
        readable_parts: vec![ReadablePart {
            label: title.to_owned(),
            text: text.to_owned(),
        }],
        provenance: prov,
    }
}

fn source_pull(
    source_id: &str,
    source_label: &str,
    workspace: &str,
    display_name: &str,
    birth_date: &str,
    mrn_value: &str,
    pulled_at: i64,
) -> SourcePull {
    SourcePull {
        source_id: source_id.to_owned(),
        source_label: source_label.to_owned(),
        patient: SourcePatient {
            source_patient_id: workspace.to_owned(),
            display_name: display_name.to_owned(),
            birth_date: Some(birth_date.to_owned()),
            sex_or_gender: Some("female".to_owned()),
            medical_record_numbers: vec![MedicalRecordNumber {
                system: "MRN".to_owned(),
                value: mrn_value.to_owned(),
            }],
        },
        pulled_at,
    }
}

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
                        id: "call-read-chart".to_owned(),
                        name: READ_TOOL.to_owned(),
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

    let skills = PhysicianSkills::configured(None, false).expect("load workspace skills");
    let model_roles = test_model_roles();
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
                timezone: "America/Los_Angeles".to_owned(),
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

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn withheld_wrong_patient_content_shows_unavailable_with_reason_in_history_and_open_detail() {
    let _guard = test_lock().lock().await;
    let ws_origin = "ws-clara-origin".to_owned();
    let ws_target = "ws-elena-target".to_owned();
    let email = "avery.lin@codahealth.test";
    let tainted_chat_id = "chat-citing-wrong-msg";
    let clean_chat_id = "chat-clean-baseline";
    let conv_id = "conv-wrong-link";

    let (harness, identity) = RunningHarness::start_after("withheld_ui", {
        let ws_origin = ws_origin.clone();
        let ws_target = ws_target.clone();
        move |store, auth| async move {
            auth.register_patient(STAGING_PRACTICE_ID, &ws_origin)
                .await
                .expect("register origin patient");
            auth.register_patient(STAGING_PRACTICE_ID, &ws_target)
                .await
                .expect("register target patient");

            let panel = vec![ws_origin.clone(), ws_target.clone()];
            let (dr, dr_hash) = enroll_physician(
                auth.as_ref(),
                STAGING_PRACTICE_ID,
                email,
                "Dr. Avery Lin",
                &panel,
            )
            .await;

            // Seed baseline EHR chart observations on both workspaces.
            let clean_rev = chart_revision(
                &ws_origin,
                "regional-ehr",
                "obs-clara-baseline",
                "rev-1",
                "2026-10-14",
                "Hypertension Follow-up",
                CLEAN_PATIENT_BASELINE_NOTE,
                json!({}),
            );
            store
                .record_chart(
                    &ws_origin,
                    std::slice::from_ref(&clean_rev),
                    &source_pull(
                        "regional-ehr",
                        "Regional EHR",
                        &ws_origin,
                        "Clara Morales",
                        "1972-06-19",
                        "MRN-4410",
                        CLINICAL_NOW_UNIX - 600,
                    ),
                )
                .await
                .expect("record origin baseline chart");

            let target_rev = chart_revision(
                &ws_target,
                "regional-ehr",
                "obs-elena-baseline",
                "rev-1",
                "2026-10-14",
                "Allergy History",
                "Baseline chart observation for Elena Vance.",
                json!({}),
            );
            store
                .record_chart(
                    &ws_target,
                    std::slice::from_ref(&target_rev),
                    &source_pull(
                        "regional-ehr",
                        "Regional EHR",
                        &ws_target,
                        "Elena Vance",
                        "1980-02-11",
                        "MRN-8821",
                        CLINICAL_NOW_UNIX - 600,
                    ),
                )
                .await
                .expect("record target baseline chart");

            // Connect Spruce channel, ingest inbound patient message, and link to `ws_origin`.
            let receiver = RecordingReviewReceiver::default();
            store
                .connect_spruce_channel(auth.as_ref(), &dr, 0, None, CLINICAL_NOW_UNIX - 300)
                .await
                .expect("connect spruce channel");
            let lease = store
                .acquire_spruce_reader_lease(
                    STAGING_PRACTICE_ID,
                    "worker-site-link",
                    600,
                    CLINICAL_NOW_UNIX - 300,
                )
                .await
                .expect("acquire spruce reader lease");
            let addr = SenderAddress::parse("+14155550199").expect("valid sender phone");
            store
                .ingest_spruce_item(
                    STAGING_PRACTICE_ID,
                    &lease.lease_token,
                    &IncomingSpruceItem::Readable(ReadableSpruceItem {
                        message_id: "msg-wrong-1".to_owned(),
                        conversation_id: conv_id.to_owned(),
                        channel_position: 1,
                        content_version: "ver-msg-wrong-1".to_owned(),
                        sender_address: Some(addr.clone()),
                        author_name: Some("Elena Vance".to_owned()),
                        body_text: WRONG_PATIENT_MESSAGE_SECRET.to_owned(),
                        attachments: Vec::new(),
                        app_url: Some(format!(
                            "https://app.sprucehealth.com/conversations/{conv_id}"
                        )),
                        created_at_spruce: Some("2026-11-12T15:00:00Z".to_owned()),
                        outside_participants: vec![OutsideParticipant {
                            display_name: Some("Elena Vance".to_owned()),
                            endpoint_channel: Some("sms".to_owned()),
                            endpoint_address: Some(addr),
                            saved_contact_id: None,
                        }],
                    }),
                    CLINICAL_NOW_UNIX - 290,
                )
                .await
                .expect("ingest spruce message");
            store
                .record_spruce_message_classification(
                    auth.as_ref(),
                    STAGING_PRACTICE_ID,
                    "msg-wrong-1",
                    None,
                    RECIPE_VERSION,
                    GUIDANCE_VERSION,
                    MatchingDecision::PatientMail {
                        urgency: MessageUrgencyChoice::Ordinary,
                        assessment: MatchAssessment::SuggestedPatient {
                            workspace: ws_origin.clone(),
                            evidence_summary: "Suggested match before correction.".to_owned(),
                        },
                        rationale: "Inbound patient message.".to_owned(),
                    },
                    Some(&receiver),
                    CLINICAL_NOW_UNIX - 285,
                )
                .await
                .expect("classify spruce message");
            store
                .link_conversation_to_patient(
                    auth.as_ref(),
                    &dr,
                    LinkConversationCommand {
                        command_key: "link-wrong-initial".to_owned(),
                        practice: STAGING_PRACTICE_ID.to_owned(),
                        conversation_id: conv_id.to_owned(),
                        workspace: ws_origin.clone(),
                        sender_policy: SenderConfirmationPolicy::ConversationOnly,
                        now_ts: CLINICAL_NOW_UNIX - 280,
                    },
                    Some(&receiver),
                )
                .await
                .expect("link conversation to origin patient");

            // Record wrong-patient Spruce message in `ws_origin`'s chart sources.
            let wrong_rev = chart_revision(
                &ws_origin,
                "spruce",
                "cand-wrong-msg",
                "rev-w1",
                "2026-11-12",
                "Inbound Spruce Message",
                WRONG_PATIENT_MESSAGE_SECRET,
                json!({
                    "conversation_id": conv_id,
                    "message_id": "msg-wrong-1",
                }),
            );
            store
                .record_chart(
                    &ws_origin,
                    std::slice::from_ref(&wrong_rev),
                    &source_pull(
                        "spruce",
                        "Spruce",
                        &ws_origin,
                        "Clara Morales",
                        "1972-06-19",
                        "MRN-4410",
                        CLINICAL_NOW_UNIX - 270,
                    ),
                )
                .await
                .expect("record wrong spruce chart observation");

            // 1) Run clean baseline chat first so it stays readable in history throughout.
            let clean_scope = Scope {
                practice: STAGING_PRACTICE_ID.to_owned(),
                physician: dr.physician_id.clone(),
                workspace: ws_origin.clone(),
                chat: clean_chat_id.to_owned(),
            };
            run_chat_citing_source(
                &store,
                &clean_scope,
                &dr_hash,
                "What is Clara's blood pressure baseline?",
                "obs-clara-baseline",
                "rev-1",
                &format!("Clara's baseline chart notes {CLEAN_PATIENT_BASELINE_NOTE} [1]."),
            )
            .await;

            tokio::time::sleep(Duration::from_millis(1100)).await;

            // 2) Run tainted chat citing `cand-wrong-msg` so it is the active open conversation.
            let tainted_scope = Scope {
                practice: STAGING_PRACTICE_ID.to_owned(),
                physician: dr.physician_id.clone(),
                workspace: ws_origin.clone(),
                chat: tainted_chat_id.to_owned(),
            };
            run_chat_citing_source(
                &store,
                &tainted_scope,
                &dr_hash,
                "What did the inbound Spruce message report?",
                "cand-wrong-msg",
                "rev-w1",
                &format!("The inbound message reported {WRONG_PATIENT_MESSAGE_SECRET} [1]."),
            )
            .await;

            // 3) Seed a reply proposal and private physician draft on `conv-wrong-link` so `Past work` also holds a reply draft.
            store
                .link_reply_conversation(
                    STAGING_PRACTICE_ID,
                    conv_id,
                    ConversationLink::LinkedPatient {
                        workspace: ws_origin.clone(),
                    },
                )
                .await
                .expect("link reply conversation");
            store
                .record_linked_patient_message(&RecordPatientMessage {
                    practice: STAGING_PRACTICE_ID.to_owned(),
                    conversation_id: conv_id.to_owned(),
                    message_id: "msg-wrong-1".to_owned(),
                    sender: MessageSender::Patient,
                    text: WRONG_PATIENT_MESSAGE_SECRET.to_owned(),
                    sent_at: CLINICAL_NOW_UNIX - 260,
                })
                .await
                .expect("record linked patient message");
            store
                .start_or_read_reply_review(
                    STAGING_PRACTICE_ID,
                    conv_id,
                    "rev-site-1",
                    "run-site-1",
                )
                .await
                .expect("start reply review");
            store
                .complete_reply_review(&CompleteReplyReview {
                    practice: STAGING_PRACTICE_ID.to_owned(),
                    conversation_id: conv_id.to_owned(),
                    review_id: "rev-site-1".to_owned(),
                    run_id: "run-site-1".to_owned(),
                    completion_key: "comp-site-1".to_owned(),
                    answered_closures: Vec::new(),
                    decision: ReplyReviewDecision::ProposeReply(ReplyProposalInput {
                        text: format!("Proposed reply referencing {WRONG_PATIENT_MESSAGE_SECRET}."),
                        basis: ReplyProposalBasisInput {
                            summary: "Inbound message review.".to_owned(),
                            limits: Vec::new(),
                            cited_records: vec![
                                ExactRecordReference::new("cand-wrong-msg", "rev-w1").unwrap(),
                            ],
                        },
                        cited_records: vec![
                            ExactRecordReference::new("cand-wrong-msg", "rev-w1").unwrap(),
                        ],
                    }),
                    raised_concern_ids: Vec::new(),
                })
                .await
                .expect("complete reply review");

            let access = WorkspaceAccess {
                practice: STAGING_PRACTICE_ID,
                physician: &dr.physician_id,
                workspace: &ws_origin,
                session_hash: &dr_hash,
            };
            store
                .edit_reply_proposal_draft(
                    &access,
                    conv_id,
                    ExpectedDraftBase::ProposalVersion(1),
                    WRONG_PATIENT_DRAFT_SECRET,
                    &mut PostgresWorkspaceAuthority,
                )
                .await
                .expect("save private reply draft");

            dr
        }
    })
    .await;

    let origin_url = format!(
        "http://{}/?patient={ws_origin}&chat={tainted_chat_id}",
        harness.address
    );
    let mut tab = Tab::open(&origin_url);
    tab.sign_in_as(email);

    // Pre-correction check: open conversation detail genuinely renders the pre-move message.
    tab.wait_for_text(WRONG_PATIENT_MESSAGE_SECRET, Duration::from_secs(20));

    // Step 1: Move `conv-wrong-link` from `ws_origin -> ws_target` while the conversation is open in the browser!
    let receiver = RecordingReviewReceiver::default();
    let move_now = real_now_unix() as i64;
    let moved = harness
        .store
        .correct_patient_link(
            harness.auth.as_ref(),
            &identity,
            CorrectPatientLinkCommand {
                command_key: "site-move-wrong-link-1".to_owned(),
                practice: STAGING_PRACTICE_ID.to_owned(),
                conversation_id: conv_id.to_owned(),
                destination: CorrectionDestination::Patient {
                    workspace: ws_target.clone(),
                },
                undo_window_seconds: 30,
                now_ts: move_now,
            },
            Some(&receiver),
        )
        .await
        .expect("correct wrong patient link");
    assert!(matches!(
        moved,
        LinkCorrectionOutcome::Moved {
            ref record,
            ..
        } if matches!(record.standing, LinkCorrectionStanding::UndoWindow { .. })
    ));

    harness
        .bus
        .publish(physician_bus::Event::WorkspaceChanged {
            workspace_id: ws_origin.clone(),
        })
        .await
        .expect("publish workspace invalidation");

    // Open conversation detail updates live in place to unavailable with reason — zero stale text!
    let undo_reason = "Unavailable: patient conversation `conv-wrong-link` was moved away from this patient while Undo is open.";
    tab.wait_for_text(undo_reason, Duration::from_secs(15));
    tab.assert_no_stale_secrets("live open conversation after move during Undo");

    // Open citation detail (`[1]`) from the withheld turn and verify unavailable inspection with reason.
    tab.click("button.citation-chip");
    tab.wait_for_text(
        "Unavailable · Withheld patient message",
        Duration::from_secs(10),
    );
    tab.wait_for_text(undo_reason, Duration::from_secs(10));
    tab.assert_no_stale_secrets("open citation inspection during Undo");
    tab.assert_no_horizontal_overflow("desktop citation inspection during Undo");
    tab.save_screenshot("01-desktop-withheld-open-detail-and-citation");

    // Step 2: Close citation inspection and open `Past work` history list.
    tab.click("dialog.reader button[aria-label=\"Close\"]");
    std::thread::sleep(Duration::from_millis(200));
    tab.click("button.past-work-button");
    let (past_work_text, _) =
        tab.wait_for_text("Unavailable patient reply draft", Duration::from_secs(10));
    assert!(
        past_work_text.contains(undo_reason),
        "Past work list must show unavailable reason; got:\n{past_work_text}"
    );
    assert!(
        past_work_text.contains("What is Clara's blood pressure baseline?"),
        "Unrelated conversation must remain readable in Past work; got:\n{past_work_text}"
    );
    tab.assert_no_stale_secrets("Past work history dialog during Undo");
    tab.assert_no_horizontal_overflow("desktop Past work history dialog");
    tab.save_screenshot("02-desktop-past-work-unavailable");

    // Step 3: Open the withheld reply draft detail from `Past work` and verify `ReaderDialog` never shows stale draft prose.
    tab.click_containing(
        ".past-work-items button.list-row",
        "Unavailable patient reply draft",
    );
    tab.wait_for_text("Unavailable patient reply draft", Duration::from_secs(10));
    tab.wait_for_text(undo_reason, Duration::from_secs(10));
    tab.assert_no_stale_secrets("open reply draft detail during Undo");
    tab.assert_no_horizontal_overflow("desktop reply draft detail");
    tab.save_screenshot("03-desktop-draft-detail-unavailable");

    // Close reply draft detail.
    tab.click("dialog.reader button[aria-label=\"Close\"]");
    std::thread::sleep(Duration::from_millis(200));

    // Step 4: Expire `Undo` with an injected removal failure (`AC4` + `AC6`) and verify unfinished-removal reason in open detail.
    let probe = StepFailureProbe {
        step: LinkCorrectionStep::WithdrawOriginEffects,
        reason: "injected withdrawal failure".to_owned(),
    };
    let failed_settlement = harness
        .store
        .settle_expired_link_corrections_with_probe(
            STAGING_PRACTICE_ID,
            Some(&receiver as &dyn ReviewReceiver),
            &probe,
            move_now + 31,
        )
        .await
        .expect("run settlement with injected failure");
    assert_eq!(failed_settlement.failed.len(), 1);
    assert_eq!(
        failed_settlement.failed[0].failed_step,
        LinkCorrectionStep::WithdrawOriginEffects
    );

    harness
        .bus
        .publish(physician_bus::Event::WorkspaceChanged {
            workspace_id: ws_origin.clone(),
        })
        .await
        .expect("publish workspace invalidation after failed removal");

    let unfinished_reason = "Unavailable: wrong-patient removal for conversation `conv-wrong-link` is unfinished while withdrawing wrong-patient chart effects (injected withdrawal failure).";
    tab.wait_for_text(unfinished_reason, Duration::from_secs(15));
    tab.assert_no_stale_secrets("live open conversation during unfinished removal");
    tab.click("button.citation-chip");
    tab.wait_for_text(
        "Unavailable · Withheld patient message",
        Duration::from_secs(10),
    );
    tab.wait_for_text(unfinished_reason, Duration::from_secs(10));
    tab.assert_no_stale_secrets("open citation inspection during unfinished removal");
    tab.save_screenshot("04-desktop-unfinished-removal-unavailable");
    tab.click("dialog.reader button[aria-label=\"Close\"]");
    std::thread::sleep(Duration::from_millis(200));

    // Step 5: Recover settlement cleanly, switch to 390x844 mobile viewport, and verify settled entered-in-error withholding in open detail & Past work.
    let settled = harness
        .store
        .settle_expired_link_corrections(
            STAGING_PRACTICE_ID,
            Some(&receiver as &dyn ReviewReceiver),
            move_now + 35,
        )
        .await
        .expect("recover link correction settlement");
    assert_eq!(settled.settled.len(), 1);
    assert!(matches!(
        settled.settled[0].standing,
        LinkCorrectionStanding::Settled { .. }
    ));

    tab.set_viewport(390, 844, true);
    tab.navigate(&origin_url);
    let settled_reason = "Unavailable: patient conversation `conv-wrong-link` was moved away from this patient and marked entered in error.";
    tab.wait_for_text(settled_reason, Duration::from_secs(15));
    tab.assert_no_stale_secrets("mobile open conversation after settled correction");
    tab.assert_no_horizontal_overflow("mobile open conversation after settled correction");

    tab.click("button.past-work-button");
    let (mobile_past_work, _) =
        tab.wait_for_text("Unavailable patient reply draft", Duration::from_secs(10));
    assert!(
        mobile_past_work.contains(settled_reason),
        "Mobile Past work must show settled unavailable reason; got:\n{mobile_past_work}"
    );
    tab.assert_no_stale_secrets("mobile Past work after settled correction");
    tab.assert_no_horizontal_overflow("mobile Past work after settled correction");
    tab.save_screenshot("05-mobile-settled-unavailable");
}
