//! Chromium browser qualification over PostgreSQL for:
//! - Find patient empty-search roster pagination, query filtering/reset, failure recovery,
//!   and practice isolation (`GET /patient-roster`).
//! - Immediate listing of newly accepted conversations while Coda is still writing,
//!   including switching conversations and live fanout to a second browser tab
//!   (`POST /commands` + `GET /patient-workspaces/events`).

use std::env;
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex as StdMutex, OnceLock, Weak};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use base64::Engine as _;
use front_door::PostgresFrontDoorStore;
use local_product::ModelRoles;
use local_product::physician_chat::PhysicianSkills;
use local_product::physician_sources::{
    MedicalRecordNumber, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
use local_product::runtime::delivery::{DeliveryKind, Wakeups};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::{Input, Scope, Store};
use local_product::sha256_hex;
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use physician_sample::STAGING_PRACTICE_ID;
use physician_sample::durable::DurableChat;
use sea_orm::ConnectionTrait;
use serde_json::json;
use tokio::sync::Mutex;
use uuid::Uuid;

#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;

use devtools::Browser;

const CLINICAL_NOW_UNIX: i64 = 1_787_846_400;
const OTHER_PRACTICE_ID: &str = "other-practice-isolation";

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

/// Fault injection mode for `GET /patient-roster`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RosterFaultMode {
    Healthy,
    FailInitialPage,
    FailNextPage,
}

struct RosterFaultState {
    mode: StdMutex<RosterFaultMode>,
}

impl RosterFaultState {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            mode: StdMutex::new(RosterFaultMode::Healthy),
        })
    }

    fn set(&self, mode: RosterFaultMode) {
        *self.mode.lock().unwrap_or_else(|p| p.into_inner()) = mode;
    }

    fn get(&self) -> RosterFaultMode {
        *self.mode.lock().unwrap_or_else(|p| p.into_inner())
    }
}

async fn roster_fault_middleware(
    axum::extract::State(faults): axum::extract::State<Arc<RosterFaultState>>,
    request: Request<Body>,
    next: Next,
) -> Response {
    if request.uri().path() == "/patient-roster" {
        let query = request.uri().query().unwrap_or_default();
        let has_after = query
            .split('&')
            .any(|pair| pair.starts_with("after=") && pair.len() > "after=".len());
        match (faults.get(), has_after) {
            (RosterFaultMode::FailInitialPage, false) | (RosterFaultMode::FailNextPage, true) => {
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    axum::Json(json!({ "detail": "Patient roster is temporarily unavailable." })),
                )
                    .into_response();
            }
            _ => {}
        }
    }
    next.run(request).await
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
        "coda-rqa-browser-pg-{}-{}",
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

fn free_loopback_port() -> u16 {
    use std::net::TcpListener;
    for _ in 0..500 {
        let port = 23_000 + (Uuid::new_v4().as_u128() % 8_000) as u16;
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
        let db_name = format!("rqa_site_{label}_{}", Uuid::new_v4().simple());
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
    roster_faults: Arc<RosterFaultState>,
    wakeups: Wakeups,
    server_task: tokio::task::JoinHandle<()>,
}

impl Drop for RunningHarness {
    fn drop(&mut self) {
        self.server_task.abort();
    }
}

impl RunningHarness {
    async fn start_after<F, Fut>(label: &str, seed: F) -> Self
    where
        F: FnOnce(Store, Arc<PostgresAuthStore>) -> Fut,
        Fut: std::future::Future<Output = ()>,
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
        auth.ensure_practice(
            OTHER_PRACTICE_ID,
            PracticeClock::Fixed(CLINICAL_NOW_UNIX as u64),
        )
        .await
        .expect("ensure other practice");

        seed(store.clone(), Arc::clone(&auth)).await;

        let roster_faults = RosterFaultState::new();
        let wakeups = store.wakeups().await.expect("subscribe runtime wakeups");
        let skills = PhysicianSkills::configured(None, false).expect("load workspace skills");
        let roles = test_model_roles();
        let chat = Arc::new(DurableChat::new(store.clone(), skills, &roles));
        let bus: Arc<dyn physician_bus::Bus> = Arc::new(physician_bus::LocalBus::new(256));

        let base_router = physician_sample::durable_site(chat, Arc::clone(&bus)).layer(
            middleware::from_fn_with_state(Arc::clone(&roster_faults), roster_fault_middleware),
        );
        let app = physician_sample::authed_postgres_router(
            base_router,
            &disposable.url,
            store.clone(),
            bus,
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

        Self {
            _db: disposable,
            address,
            roster_faults,
            wakeups,
            server_task,
        }
    }
}

async fn enroll_physician(
    auth: &PostgresAuthStore,
    practice_id: &str,
    email: &str,
    display_name: &str,
    workspaces: &[String],
) -> PhysicianIdentity {
    let now = real_now_unix();
    let invite = auth
        .invite(practice_id, email, display_name, false, now)
        .await
        .expect("invite physician");
    let (identity, _session) = auth
        .redeem_invitation(email, &invite, now)
        .await
        .expect("redeem invitation");
    for ws in workspaces {
        auth.set_membership(&identity, ws, Membership::Allowed)
            .await
            .expect("grant panel membership");
    }
    identity
}

async fn register_and_seed_patient(
    auth: &PostgresAuthStore,
    store: &Store,
    practice_id: &str,
    workspace: &str,
    display_name: &str,
    birth_date: &str,
    mrn_value: &str,
) {
    auth.register_patient(practice_id, workspace)
        .await
        .expect("register patient");
    let content = json!({
        "resourceType": "Observation",
        "id": format!("obs-{workspace}"),
        "note": format!("Baseline chart observation for {display_name}."),
    })
    .to_string();
    let pull = SourcePull {
        source_id: "regional-ehr".to_owned(),
        source_label: "Regional EHR".to_owned(),
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
        pulled_at: Some(CLINICAL_NOW_UNIX - 600),
    };
    let rev = SourceRevision {
        workspace: workspace.to_owned(),
        source_id: "regional-ehr".to_owned(),
        candidate_id: format!("obs-{workspace}"),
        revision: "rev-1".to_owned(),
        record_type: "Clinical observation".to_owned(),
        dates: vec![RecordDate {
            name: "effectiveDate".to_owned(),
            value: "2026-08-27".to_owned(),
        }],
        sha256: sha256_hex(content.as_bytes()),
        content,
        readable_parts: vec![ReadablePart {
            label: "Baseline".to_owned(),
            text: format!("Baseline chart observation for {display_name}."),
        }],
        provenance: json!({"system": "regional-ehr", "id": format!("obs-{workspace}")}),
        envelope: None,
    };
    store
        .record_chart(workspace, &[rev], &pull)
        .await
        .expect("record patient chart");
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

    fn click_named(&mut self, selector: &str, words: &str) {
        self.step("clickNamed", serde_json::json!([selector, words]));
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

    fn wait_for_text(&mut self, expected: &str, timeout: Duration) -> String {
        let started = Instant::now();
        loop {
            let snapshot = self.step("visibleSnapshot", serde_json::json!([]));
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

    fn dialog_row_titles(&mut self) -> Vec<String> {
        self.step("dialogRowTitles", serde_json::json!([]))
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn dialog_row_details(&mut self) -> Vec<String> {
        self.step("dialogRowDetails", serde_json::json!([]))
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn wait_for_dialog_row_count(&mut self, expected: usize, timeout: Duration) -> Vec<String> {
        let started = Instant::now();
        loop {
            let rows = self.dialog_row_titles();
            if rows.len() == expected {
                return rows;
            }
            if started.elapsed() >= timeout {
                let snap = self.step("visibleSnapshot", serde_json::json!([]));
                panic!(
                    "timed out waiting for {expected} dialog rows; got {} rows: {rows:?}\nvisible text:\n{}\nDOM:\n{}",
                    rows.len(),
                    snap["text"].as_str().unwrap_or_default(),
                    snap["html"].as_str().unwrap_or_default(),
                );
            }
            std::thread::sleep(Duration::from_millis(60));
        }
    }

    fn wait_for_no_dialog(&mut self, timeout: Duration) {
        let started = Instant::now();
        loop {
            let open = self
                .browser
                .page()
                .evaluate("Boolean(document.querySelector('dialog'))")
                .as_bool()
                .unwrap_or(false);
            if !open {
                return;
            }
            assert!(
                started.elapsed() < timeout,
                "timed out waiting for open dialog to unmount"
            );
            std::thread::sleep(Duration::from_millis(30));
        }
    }

    fn wait_for_search_input_value(&mut self, timeout: Duration) -> serde_json::Value {
        let started = Instant::now();
        loop {
            let val = self.step("searchInputValue", serde_json::json!([]));
            if !val.is_null() || started.elapsed() >= timeout {
                return val;
            }
            std::thread::sleep(Duration::from_millis(30));
        }
    }

    fn conversation_titles(&mut self) -> Vec<String> {
        self.step("conversationTitles", serde_json::json!([]))
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn conversation_row_texts(&mut self) -> Vec<String> {
        self.step("conversationRowTexts", serde_json::json!([]))
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn wait_for_conversation_titled(&mut self, title: &str, timeout: Duration) -> Vec<String> {
        let started = Instant::now();
        loop {
            let titles = self.conversation_titles();
            if titles.iter().any(|item| item == title) {
                return titles;
            }
            assert!(
                started.elapsed() < timeout,
                "timed out waiting for conversation {title:?} in sidebar; currently showing: {titles:?}"
            );
            std::thread::sleep(Duration::from_millis(60));
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

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn find_patient_lists_paged_roster_recovers_failures_and_isolates_practices() {
    let _guard = test_lock().lock().await;
    let harness = RunningHarness::start_after("roster_paging", |store, auth| async move {
        // Seed 32 patients in reverse order so name ordering is genuinely exercised.
        let mut panel = Vec::new();
        for index in (1..=32).rev() {
            let workspace = format!("pw-roster-{index:02}");
            let name = if index == 19 {
                "Patient 19 Nora Sterling".to_owned()
            } else {
                format!("Patient {index:02}")
            };
            let birth_date = format!("1975-03-{:02}", (index % 28) + 1);
            let mrn = format!("MRN-{index:04}");
            register_and_seed_patient(
                &auth,
                &store,
                STAGING_PRACTICE_ID,
                &workspace,
                &name,
                &birth_date,
                &mrn,
            )
            .await;
            panel.push(workspace);
        }
        // Seed a cross-practice patient whose name ("Aaron CrossPractice") would sort at row 0
        // if practice isolation ever leaked.
        register_and_seed_patient(
            &auth,
            &store,
            OTHER_PRACTICE_ID,
            "pw-other-practice-01",
            "Aaron CrossPractice",
            "1969-11-04",
            "MRN-OTHER-01",
        )
        .await;

        enroll_physician(
            &auth,
            STAGING_PRACTICE_ID,
            "roster-physician@codahealth.test",
            "Dr. Roster",
            &panel,
        )
        .await;
    })
    .await;

    let mut tab = Tab::open(&format!("http://{}/", harness.address));
    tab.sign_in_as("roster-physician@codahealth.test");
    tab.wait_for_text("Nothing needs you right now.", Duration::from_secs(20));

    // Open Find patient: empty query lists first 25 patients ordered by name with DOB/MRN,
    // excluding other-practice patients.
    tab.click("button[aria-label=\"Find patient\"]");
    assert_eq!(
        tab.wait_for_search_input_value(Duration::from_secs(5)),
        serde_json::json!("")
    );
    let roster_name = |i: usize| {
        if i == 19 {
            "Patient 19 Nora Sterling".to_owned()
        } else {
            format!("Patient {i:02}")
        }
    };
    let first_page = tab.wait_for_dialog_row_count(25, Duration::from_secs(15));
    let expected_first_25: Vec<String> = (1..=25).map(roster_name).collect();
    assert_eq!(first_page, expected_first_25);
    assert!(
        !first_page.iter().any(|name| name.contains("Aaron")),
        "other-practice patient must never appear: {first_page:?}"
    );
    let details = tab.dialog_row_details();
    assert_eq!(details.len(), 25);
    assert!(
        details[0].contains("DOB Mar 2, 1975") && details[0].contains("MRN MRN-0001"),
        "row detail must show DOB and MRN: {:?}",
        details[0]
    );
    tab.assert_no_horizontal_overflow("desktop Find patient top");
    tab.save_screenshot("eng-450-roster-top");

    // Inject a next-page failure and scroll to bottom: must keep existing 25 rows AND surface
    // failure with Retry ("Try again"), never masquerading as the end of the list.
    harness.roster_faults.set(RosterFaultMode::FailNextPage);
    tab.step(
        "scrollToBottom",
        serde_json::json!([".patient-search-scroll"]),
    );
    tab.wait_for_text("Coda couldn’t load more patients.", Duration::from_secs(10));
    tab.wait_for_text("Try again", Duration::from_secs(5));
    assert_eq!(tab.dialog_row_titles().len(), 25);
    tab.save_screenshot("eng-450-roster-page-failure");

    // Recover from next-page failure via "Try again": appends remaining 7 patients (total 32).
    harness.roster_faults.set(RosterFaultMode::Healthy);
    tab.click_named("dialog button", "Try again");
    let all_rows = tab.wait_for_dialog_row_count(32, Duration::from_secs(15));
    let expected_all_32: Vec<String> = (1..=32).map(roster_name).collect();
    assert_eq!(all_rows, expected_all_32);
    assert!(
        !all_rows.iter().any(|name| name.contains("Aaron")),
        "other-practice patient must never appear after paging: {all_rows:?}"
    );
    tab.step(
        "scrollToBottom",
        serde_json::json!([".patient-search-scroll"]),
    );
    std::thread::sleep(Duration::from_millis(150));
    tab.save_screenshot("eng-450-roster-scrolled");

    // Typing a query replaces the list with matches; clearing returns to the paged full list.
    tab.browser.type_into(".search-field input", "Sterling");
    let narrowed = tab.wait_for_dialog_row_count(1, Duration::from_secs(10));
    assert_eq!(narrowed, vec!["Patient 19 Nora Sterling".to_owned()]);

    // Searching for the other-practice patient returns "No patients match.".
    tab.click("button[aria-label=\"Clear search\"]");
    let restored = tab.wait_for_dialog_row_count(25, Duration::from_secs(10));
    assert_eq!(restored, expected_first_25);

    tab.browser
        .type_into(".search-field input", "Aaron CrossPractice");
    tab.wait_for_text("No patients match.", Duration::from_secs(10));
    assert!(tab.dialog_row_titles().is_empty());

    // Close with non-empty search text, then reopen: dialog opens with an empty search each time
    // and smoothly loads page 2 on scroll without fault.
    tab.click("dialog button[aria-label=\"Close\"]");
    tab.wait_for_no_dialog(Duration::from_secs(5));
    tab.click("button[aria-label=\"Find patient\"]");
    assert_eq!(
        tab.wait_for_search_input_value(Duration::from_secs(5)),
        serde_json::json!("")
    );
    assert_eq!(
        tab.wait_for_dialog_row_count(25, Duration::from_secs(10)),
        expected_first_25
    );
    tab.step(
        "scrollToBottom",
        serde_json::json!([".patient-search-scroll"]),
    );
    assert_eq!(
        tab.wait_for_dialog_row_count(32, Duration::from_secs(10)),
        expected_all_32
    );

    // Initial page load failure also surfaces failure + Try again, never empty roster.
    tab.click("dialog button[aria-label=\"Close\"]");
    tab.wait_for_no_dialog(Duration::from_secs(5));
    harness.roster_faults.set(RosterFaultMode::FailInitialPage);
    tab.click("button[aria-label=\"Find patient\"]");
    tab.wait_for_text("Coda couldn’t load patients.", Duration::from_secs(10));
    tab.wait_for_text("Try again", Duration::from_secs(5));
    harness.roster_faults.set(RosterFaultMode::Healthy);
    tab.click_named("dialog button", "Try again");
    assert_eq!(
        tab.wait_for_dialog_row_count(25, Duration::from_secs(10)),
        expected_first_25
    );

    // Verify phone viewport (<900px) has zero horizontal overflow.
    tab.set_viewport(390, 844, true);
    std::thread::sleep(Duration::from_millis(150));
    tab.assert_no_horizontal_overflow("phone Find patient roster");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn new_conversation_appears_immediately_survives_switch_and_notifies_second_tab() {
    let _guard = test_lock().lock().await;
    let workspace = "pw-chat-immediate-01";
    let mut harness = RunningHarness::start_after("chat_immediate", |store, auth| async move {
        register_and_seed_patient(
            &auth,
            &store,
            STAGING_PRACTICE_ID,
            workspace,
            "Mary Louis",
            "1972-03-18",
            "00000005",
        )
        .await;
        let identity = enroll_physician(
            &auth,
            STAGING_PRACTICE_ID,
            "chat-physician@codahealth.test",
            "Dr. Chat",
            &[workspace.to_owned()],
        )
        .await;

        // Seed an earlier conversation on Mary Louis so switching chats before reply completion
        // can be exercised directly.
        let skills = PhysicianSkills::configured(None, false).expect("load workspace skills");
        let roles = test_model_roles();
        let recipe = store
            .chat_recipe_with_understanding(
                &roles.chat,
                local_product::physician_chat::INSTRUCTIONS,
                &skills,
                &identity.practice_id,
                workspace,
            )
            .await
            .expect("chat recipe");
        store
            .accept_authorized(
                &Scope {
                    practice: identity.practice_id.clone(),
                    physician: identity.physician_id.clone(),
                    workspace: workspace.to_owned(),
                    chat: "chat-earlier-bp".to_owned(),
                },
                &Input {
                    key: "seed-earlier-bp".to_owned(),
                    text: "Prior blood pressure review".to_owned(),
                    timezone: "America/New_York".to_owned(),
                    scenario: None,
                },
                vec![3_u8; 32],
                &skills,
                &recipe,
                None,
            )
            .await
            .expect("seed earlier conversation");
    })
    .await;

    let patient_url = format!("http://{}/?patient={workspace}", harness.address);
    let mut tab_one = Tab::open(&patient_url);
    let mut tab_two = Tab::open(&patient_url);

    for tab in [&mut tab_one, &mut tab_two] {
        tab.sign_in_as("chat-physician@codahealth.test");
        tab.wait_for_conversation_titled("Prior blood pressure review", Duration::from_secs(20));
    }
    // Give Tab 2's EventSource / streaming connection a beat to settle before Tab 1 submits.
    std::thread::sleep(Duration::from_millis(350));

    let prompt = "Check potassium trend before tomorrow's visit";
    tab_one.click("button.sidebar-new-conversation");
    tab_one.wait_for_text(
        "What would you like Coda to review?",
        Duration::from_secs(10),
    );
    tab_one.browser.type_into("textarea.composer-input", prompt);
    std::thread::sleep(Duration::from_millis(80));
    tab_one.click_named(".composer-toolbar button", "Send");

    // AC1: New conversation appears immediately in Tab 1's Conversations list, titled from
    // its first message, while Coda is still writing (no worker has run; queue holds wakeup).
    let tab_one_titles = tab_one.wait_for_conversation_titled(prompt, Duration::from_secs(10));
    assert_eq!(
        tab_one_titles.first().map(String::as_str),
        Some(prompt),
        "newest accepted conversation must appear at the top of Conversations: {tab_one_titles:?}"
    );
    assert!(
        !tab_one_titles.iter().any(|t| t == "Untitled"),
        "conversation must be titled from its first message, never Untitled: {tab_one_titles:?}"
    );
    let tab_one_rows = tab_one.conversation_row_texts();
    assert!(
        tab_one_rows[0].contains("In progress"),
        "pending conversation row must show In progress while Coda is writing: {:?}",
        tab_one_rows[0]
    );
    let notified = tokio::time::timeout(Duration::from_secs(5), harness.wakeups.next())
        .await
        .expect("turn wakeup notified within 5s")
        .expect("wakeup channel open")
        .expect("delivery notification");
    assert_eq!(
        notified.kind,
        DeliveryKind::ChatTurn,
        "turn was queued once and no model worker completed it"
    );
    tab_one.save_screenshot("eng-451-pending-conversation-listed");

    // AC3: Without any user action in Tab 2, the server's WorkspaceChanged event on message
    // accept causes Tab 2 to refetch and list the new conversation before the reply lands.
    let tab_two_titles = tab_two.wait_for_conversation_titled(prompt, Duration::from_secs(15));
    assert_eq!(
        tab_two_titles.first().map(String::as_str),
        Some(prompt),
        "second tab must list the newly accepted conversation at the top: {tab_two_titles:?}"
    );
    tab_two.save_screenshot("eng-451-second-tab-listed");

    // AC2: Physician switches to another conversation ("Prior blood pressure review") in Tab 1
    // before Coda's reply lands; the new conversation remains in Conversations.
    tab_one.click_containing(
        ".glass-sidebar button.sidebar-conversation-link",
        "Prior blood pressure review",
    );
    tab_one.wait_for_text("Prior blood pressure review", Duration::from_secs(10));
    std::thread::sleep(Duration::from_millis(200));
    let after_switch_titles = tab_one.conversation_titles();
    assert_eq!(
        after_switch_titles,
        vec![prompt.to_owned(), "Prior blood pressure review".to_owned(),],
        "new conversation must stay listed after switching to another chat: {after_switch_titles:?}"
    );
    let page_titles: Vec<String> = tab_one
        .step("pageConversationTitles", serde_json::json!([]))
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();
    assert_eq!(
        page_titles,
        vec![prompt.to_owned(), "Prior blood pressure review".to_owned(),],
        "patient page Conversation list must also keep the pending chat listed: {page_titles:?}"
    );
    tab_one.save_screenshot("eng-451-switched-chat-still-listed");
}
