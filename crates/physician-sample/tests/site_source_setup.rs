//! Headless Chromium qualification across desktop (`1440x960`) and phone (`390x844`)
//! viewports for practice-facing clinical source setup, backlog acceptance, quiet
//! catch-up/retry, transient read failure vs lost-access recovery, and disconnect
//! data retention (`/connections/*` + `/today` + patient workspace).

use std::env;
use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex as StdMutex, Weak};
use std::time::{Duration, Instant};

use base64::Engine as _;
use front_door::PostgresFrontDoorStore;
use local_product::physician_sources::{
    MedicalRecordNumber, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
use local_product::runtime::Store;
use local_product::runtime::sources::SourcePull;
use physician_auth::{AuthStore, PostgresAuthStore};
use sea_orm::ConnectionTrait;
use serde_json::{Value, json};
use uuid::Uuid;

#[allow(dead_code)]
mod common;
#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;
#[allow(dead_code)]
mod required_auth;

use devtools::{Browser, DESKTOP, PHONE};
use required_auth::shared_site;

const STAFF_SECRET: &str = "coda-staff-setup-secret-2026-browser-token";
const FIXED_CLOCK_UNIX: u64 = 1_787_846_400; // 2026-08-27T16:00:00Z
const SEARCH_FIELD: &str = ".search-field input";
const FIRST_RESULT: &str = ".dialog-results button";

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
        let port = 31_000 + (Uuid::new_v4().as_u128() % 7_000) as u16;
        if TcpListener::bind(("127.0.0.1", port)).is_ok() {
            return port;
        }
    }
    panic!("no free loopback port for test postgres cluster");
}

fn ensure_admin_postgres_url() -> (String, Option<Arc<LocalCluster>>) {
    if let Ok(explicit) = env::var("PHYSICIAN_AUTH_TEST_DATABASE_URL") {
        let trimmed = explicit.trim();
        if !trimmed.is_empty() {
            return (trimmed.to_owned(), None);
        }
    }
    let mut guard = LOCAL_CLUSTER.lock().expect("local cluster lock");
    if let Some(existing) = guard.upgrade() {
        return (existing.admin_url.clone(), Some(existing));
    }

    let cluster_dir =
        env::temp_dir().join(format!("coda-source-setup-pg-{}", Uuid::new_v4().simple()));
    let data_dir = cluster_dir.join("data");
    let sock_dir = cluster_dir.join("sock");
    let log_file = cluster_dir.join("postgres.log");
    fs::create_dir_all(&sock_dir).expect("create sock dir");

    let init = Command::new("initdb")
        .args([
            "-D",
            data_dir.to_str().expect("utf8 data dir"),
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

    let port = free_loopback_port();
    let opts = format!(
        "-F -N 300 -h 127.0.0.1 -p {port} -k {}",
        sock_dir.to_str().expect("utf8 sock dir")
    );
    let start = Command::new("pg_ctl")
        .args([
            "-D",
            data_dir.to_str().expect("utf8 data dir"),
            "-l",
            log_file.to_str().expect("utf8 log"),
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
        "pg_ctl start failed: {}",
        String::from_utf8_lossy(&start.stderr)
    );

    let admin_url = format!("postgres://postgres@127.0.0.1:{port}/postgres");
    let cluster = Arc::new(LocalCluster {
        cluster_dir,
        data_dir,
        admin_url: admin_url.clone(),
    });
    *guard = Arc::downgrade(&cluster);
    (admin_url, Some(cluster))
}

struct DisposableDb {
    admin_url: String,
    db_name: String,
    url: String,
    _cluster: Option<Arc<LocalCluster>>,
}

impl DisposableDb {
    async fn create(prefix: &str) -> Self {
        let (admin_url, cluster) = ensure_admin_postgres_url();
        let db_name = format!("coda_{prefix}_{}", Uuid::new_v4().simple());
        let admin = sea_orm::Database::connect(&admin_url)
            .await
            .expect("connect admin db");
        admin
            .execute_unprepared(&format!(r#"CREATE DATABASE "{db_name}""#))
            .await
            .expect("create disposable db");

        let base = admin_url
            .rsplit_once('/')
            .map(|(prefix, _)| prefix)
            .expect("postgres url slash");
        let url = format!("{base}/{db_name}");

        let auth = PostgresAuthStore::connect_with_migrations(&url)
            .await
            .expect("migrate physician-auth");
        drop(auth);
        let fd = PostgresFrontDoorStore::connect_with_migrations(&url)
            .await
            .expect("migrate front-door");
        drop(fd);
        let store = Store::connect(&url).await.expect("connect runtime");
        store.migrate().await.expect("migrate runtime");
        drop(store);

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
        let admin_url = self.admin_url.clone();
        let db_name = self.db_name.clone();
        let _ = std::thread::spawn(move || {
            if let Ok(rt) = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                rt.block_on(async move {
                    if let Ok(admin) = sea_orm::Database::connect(&admin_url).await {
                        let _ = admin
                            .execute_unprepared(&format!(
                                r#"DROP DATABASE IF EXISTS "{db_name}" WITH (FORCE)"#
                            ))
                            .await;
                    }
                });
            }
        })
        .join();
    }
}

async fn spawn_router(router: axum::Router) -> (SocketAddr, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind listener");
    let addr = listener.local_addr().expect("local addr");
    let handle = tokio::spawn(async move {
        axum::serve(listener, router).await.expect("serve router");
    });
    (addr, handle)
}

async fn seed_practice_patient(
    pg_auth: &PostgresAuthStore,
    runtime: &Store,
    practice_id: &str,
    workspace_id: &str,
    full_name: &str,
    mrn: &str,
) {
    pg_auth
        .register_patient(practice_id, workspace_id)
        .await
        .expect("register patient");
    let content = json!({
        "workspace": workspace_id,
        "patient": full_name,
        "mrn": mrn,
    })
    .to_string();
    let sha256 = local_product::sha256_hex(content.as_bytes());
    let rev = SourceRevision {
        workspace: workspace_id.to_owned(),
        source_id: "elation_fhir".to_owned(),
        candidate_id: format!("{workspace_id}-seed-demographics"),
        revision: format!("rev-{}", &sha256[..12]),
        record_type: "source_record".to_owned(),
        dates: vec![RecordDate {
            name: "recorded_at".to_owned(),
            value: "2026-08-27T16:00:00Z".to_owned(),
        }],
        content,
        sha256,
        readable_parts: vec![ReadablePart {
            label: "Demographics".to_owned(),
            text: format!("{full_name} primary care chart registered."),
        }],
        provenance: json!({
            "source_id": "elation_fhir",
            "native_id": format!("{workspace_id}-seed-demographics"),
        }),
    };
    let pull = SourcePull {
        source_id: "elation_fhir".to_owned(),
        source_label: "Elation / FHIR".to_owned(),
        patient: SourcePatient {
            source_patient_id: workspace_id.to_owned(),
            display_name: full_name.to_owned(),
            birth_date: Some("1982-04-14".to_owned()),
            sex_or_gender: Some("female".to_owned()),
            medical_record_numbers: vec![MedicalRecordNumber {
                system: "Clinic".to_owned(),
                value: mrn.to_owned(),
            }],
        },
        pulled_at: FIXED_CLOCK_UNIX as i64,
    };
    runtime
        .record_chart(workspace_id, &[rev], &pull)
        .await
        .expect("seed chart");
}

struct SourceWalk {
    browser: Browser,
    size: &'static str,
    proof_dir: PathBuf,
}

impl SourceWalk {
    fn launch(size: (&'static str, (u32, u32)), proof_dir: PathBuf) -> Self {
        let mut browser = Browser::launch();
        browser.resize(size.1);
        browser.page().call_session("Page.enable", json!({}));
        browser.page().call_session(
            "Emulation.setTimezoneOverride",
            json!({ "timezoneId": "America/New_York" }),
        );
        Self {
            browser,
            size: size.0,
            proof_dir,
        }
    }

    fn run(&mut self, action: &str, args: &[Value]) -> Value {
        let mut arguments = vec![json!(action)];
        arguments.extend_from_slice(args);
        self.browser
            .page()
            .call_function(include_str!("js/source_setup.js"), &arguments)
    }

    fn open(&mut self, url: &str, needle: &str) {
        self.browser.text_until(url, needle);
    }

    fn wait_dialog_contains(&mut self, needle: &str) -> String {
        let deadline = Instant::now() + Duration::from_secs(12);
        loop {
            let text = self
                .run("dialogText", &[])
                .as_str()
                .unwrap_or_default()
                .to_owned();
            if text.contains(needle) {
                return text;
            }
            assert!(
                Instant::now() < deadline,
                "dialog never showed `{needle}` at {}; last dialogText:\n{text}",
                self.size
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    fn wait_page_contains(&mut self, needle: &str) -> String {
        let deadline = Instant::now() + Duration::from_secs(12);
        loop {
            let text = self
                .run("pageText", &[])
                .as_str()
                .unwrap_or_default()
                .to_owned();
            if text.contains(needle) {
                return text;
            }
            assert!(
                Instant::now() < deadline,
                "page never showed `{needle}` at {}; last pageText:\n{text}",
                self.size
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    fn until(&mut self, what: &str, helper: &str, args: &[Value]) {
        let deadline = Instant::now() + Duration::from_secs(12);
        while self.run(helper, args) != json!(true) {
            assert!(
                Instant::now() < deadline,
                "{what} never succeeded at {}",
                self.size
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    fn press(&mut self, words: &str) {
        self.until(words, "press", &[json!(words)]);
    }

    fn press_in_sidebar(&mut self, words: &str) {
        if self.size == "phone" {
            self.press("Open navigation");
        }
        self.press(words);
    }

    fn press_source_tab(&mut self, label: &str) {
        self.until(label, "pressSourceTab", &[json!(label)]);
    }

    fn choose_radio(&mut self, label_words: &str) {
        self.until(label_words, "chooseRadio", &[json!(label_words)]);
    }

    fn set_checkbox(&mut self, prefix: &str, checked: bool) {
        self.until(prefix, "setCheckbox", &[json!(prefix), json!(checked)]);
    }

    fn fill_field(&mut self, field_label: &str, value: &str) {
        self.until(
            field_label,
            "fillField",
            &[json!(field_label), json!(value)],
        );
    }

    fn appears(&mut self, selector: &str) {
        self.until(selector, "exists", &[json!(selector)]);
    }

    fn press_first(&mut self, selector: &str) {
        self.until(selector, "pressFirst", &[json!(selector)]);
    }

    fn open_patient(&mut self, query: &str, needle: &str) {
        self.press_in_sidebar("Find patient");
        self.appears(SEARCH_FIELD);
        self.browser.type_into(SEARCH_FIELD, query);
        self.appears(FIRST_RESULT);
        self.press_first(FIRST_RESULT);
        self.wait_page_contains(needle);
    }

    fn assert_no_sideways_scroll(&mut self, stage: &str) {
        let overflow = self.run("sidewaysScroll", &[]);
        assert!(
            overflow.as_f64().is_some_and(|by| by <= 0.0),
            "{stage} scrolls sideways by {overflow} at {}",
            self.size
        );
    }

    fn capture(&mut self, stage_name: &str) -> PathBuf {
        self.assert_no_sideways_scroll(stage_name);
        std::thread::sleep(Duration::from_millis(150));
        let shot = self
            .browser
            .page()
            .call_session("Page.captureScreenshot", json!({ "format": "png" }));
        let data = shot
            .pointer("/result/data")
            .and_then(Value::as_str)
            .expect("screenshot base64 data");
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(data)
            .expect("decode screenshot png");
        fs::create_dir_all(&self.proof_dir).expect("create proof dir");
        let path = self
            .proof_dir
            .join(format!("{}-{stage_name}.png", self.size));
        fs::write(&path, bytes).expect("write screenshot png");
        path
    }
}

async fn get_connections(client: &reqwest::Client, base: &str, token: &str) -> Value {
    client
        .get(format!("{base}/connections"))
        .bearer_auth(token)
        .send()
        .await
        .expect("GET /connections")
        .json()
        .await
        .expect("connections json")
}

fn find_source<'a>(connections: &'a Value, slug: &str) -> &'a Value {
    connections["sources"]
        .as_array()
        .expect("sources array")
        .iter()
        .find(|src| src["source_id"].as_str() == Some(slug))
        .unwrap_or_else(|| panic!("missing source `{slug}` in {connections}"))
}

async fn simulate_spruce(client: &reqwest::Client, base: &str, token: &str, body: Value) -> Value {
    let resp = client
        .post(format!("{base}/connections/spruce/simulate"))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await
        .expect("POST /connections/spruce/simulate");
    assert_eq!(resp.status(), reqwest::StatusCode::OK);
    resp.json().await.expect("simulate response json")
}

#[allow(clippy::too_many_lines)]
async fn walk_viewport(
    size: (&'static str, (u32, u32)),
    base: &str,
    pg_auth: &PostgresAuthStore,
    runtime: &Store,
    client: &reqwest::Client,
    proof_dir: &Path,
) {
    let practice_id = format!("practice-sources-{}", size.0);
    let email = format!("dr.{}@codahealth.test", size.0);
    let workspace_id = format!("pw-mary-{}", size.0);

    // 1. Staff creates the synthetic practice + invites the practice physician.
    let create_practice = client
        .put(format!("{base}/auth/staff/practices/{practice_id}"))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "clock": { "mode": "fixed", "unix_seconds": FIXED_CLOCK_UNIX },
            "time_zone": "America/New_York",
        }))
        .send()
        .await
        .expect("create practice");
    assert_eq!(create_practice.status(), reqwest::StatusCode::CREATED);

    let invite: Value = client
        .post(format!(
            "{base}/auth/staff/practices/{practice_id}/invitations"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "email": email,
            "display_name": "Dr. Maya West",
            "role": "admin",
        }))
        .send()
        .await
        .expect("invite physician")
        .json()
        .await
        .expect("invite json");
    let sign_in_path = invite["sign_in_path"]
        .as_str()
        .expect("proof invitation sign_in_path");

    // 2. Physician opens the invitation entry in Chromium and signs into Today.
    let mut walk = SourceWalk::launch(size, proof_dir.to_path_buf());
    walk.open(
        &format!("{base}{sign_in_path}"),
        "Nothing needs you right now.",
    );
    walk.assert_no_sideways_scroll("00-signed-in-today");

    let session_token = walk
        .run("sessionToken", &[])
        .as_str()
        .expect("browser session token")
        .to_owned();
    assert!(
        !session_token.is_empty(),
        "expected stored browser session token after redeeming invitation"
    );

    let whoami: Value = client
        .get(format!("{base}/auth/whoami"))
        .bearer_auth(&session_token)
        .send()
        .await
        .expect("whoami")
        .json()
        .await
        .expect("whoami json");
    let physician_id = whoami["physician_id"]
        .as_str()
        .expect("physician_id")
        .to_owned();

    // Seed Mary Louis in this practice and assign her to the signed-in physician's panel.
    seed_practice_patient(
        pg_auth,
        runtime,
        &practice_id,
        &workspace_id,
        "Mary Louis",
        "MRN-4410",
    )
    .await;
    let mut assigned = false;
    for _ in 0..40 {
        let resp = client
            .put(format!("{base}/auth/panels/{physician_id}/{workspace_id}"))
            .bearer_auth(&session_token)
            .send()
            .await
            .expect("assign Mary Louis");
        if resp.status() == reqwest::StatusCode::OK {
            assigned = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert!(assigned, "Mary Louis assigned to physician panel");

    // 3. Open Connections and verify Hint Health Chrome connector prerequisites + connection.
    walk.press_in_sidebar("Connections");
    let hint_initial = walk.wait_dialog_contains("Hint Health");
    assert!(
        hint_initial.contains("Chrome browser with the Coda Hint connector extension enabled."),
        "expected Hint connector prerequisite in dialog:\n{hint_initial}"
    );
    assert!(
        hint_initial.contains(
            "Active Hint Health practice session with whole-chart and full-note read access."
        ),
        "expected Hint whole-chart access prerequisite in dialog:\n{hint_initial}"
    );

    // Attempting Hint BrowserSession without confirming prerequisites surfaces clear error.
    walk.fill_field(
        "Chrome connector verification code",
        "hint-chrome-session-8821",
    );
    walk.press("Verify Chrome connector");
    walk.wait_dialog_contains("Confirm the Hint Chrome connector prerequisites before connecting.");

    // Confirm Hint prerequisite checkbox and complete verification -> Connected -> Records imported -> Up to date.
    walk.set_checkbox("I am signed in to Hint Health in Chrome", true);
    walk.press("Verify Chrome connector");
    walk.wait_dialog_contains("Connection verified — no records imported yet");
    let after_hint_connect = get_connections(client, base, &session_token).await;
    assert_eq!(
        find_source(&after_hint_connect, "hint")["standing"]["state"],
        "connected"
    );
    walk.capture("01-hint-connected");

    walk.press("Import initial records");
    walk.wait_dialog_contains("Initial records imported");
    walk.press("Sync latest records");
    walk.wait_dialog_contains("Hint Health · Up to date");

    // 4. Elation / FHIR: Coda-assisted setup request + simulated practice walkthrough.
    walk.press_source_tab("Elation / FHIR");
    walk.wait_dialog_contains("Elation / FHIR R4 base URL");
    walk.choose_radio("Coda-assisted setup");
    walk.fill_field("Practice contact email", "ops@codahealth.test");
    walk.fill_field(
        "Setup notes (optional)",
        "Prefer Tuesday morning onboarding window",
    );
    walk.press("Request Coda-assisted setup");
    let assisted_dom = walk.wait_dialog_contains("Coda-assisted setup requested");
    assert!(assisted_dom.contains("ops@codahealth.test"));
    let after_assist = get_connections(client, base, &session_token).await;
    assert_eq!(
        find_source(&after_assist, "elation_fhir")["standing"]["state"],
        "coda_assisted_requested"
    );
    walk.capture("02-elation-coda-assisted");

    walk.choose_radio("Simulated practice walkthrough");
    walk.press("Connect simulated source");
    walk.wait_dialog_contains("Elation / FHIR · Connected");
    walk.press("Import initial records");
    walk.wait_dialog_contains("Elation / FHIR · Records imported");
    walk.press("Sync latest records");
    walk.wait_dialog_contains("Elation / FHIR · Up to date");

    // 5. Spruce Health: prove Connected != RecordsImported != UpToDate + Rule 14 backlog gate.
    walk.press_source_tab("Spruce Health");
    walk.wait_dialog_contains("Spruce Health organization administrator API token");
    walk.fill_field(
        "API credential or bearer secret",
        "spruce-org-token-2026-live-check",
    );
    walk.press("Verify credentials");

    // Stage A: Connected (zero records imported yet)
    let spruce_connected_dom =
        walk.wait_dialog_contains("Connection verified — no records imported yet");
    let spruce_connected_json = find_source(
        &get_connections(client, base, &session_token).await,
        "spruce",
    )
    .clone();
    assert_eq!(spruce_connected_json["standing"]["state"], "connected");
    assert_eq!(spruce_connected_json["retained_linked_records"], 0);
    assert!(spruce_connected_dom.contains("Spruce Health · Connected"));
    assert!(!spruce_connected_dom.contains("Initial records imported"));
    assert!(!spruce_connected_dom.contains("Spruce Health · Up to date"));
    walk.capture("03-spruce-connected");

    // Stage B: Records imported + pre-connection backlog held behind explicit acceptance (Rule 14)
    walk.press("Import initial records");
    let spruce_imported_dom = walk.wait_dialog_contains("Pre-connection Spruce backlog held");
    let spruce_imported_json = find_source(
        &get_connections(client, base, &session_token).await,
        "spruce",
    )
    .clone();
    assert_eq!(
        spruce_imported_json["standing"]["state"],
        "records_imported"
    );
    assert_eq!(spruce_imported_json["standing"]["imported_records"], 2);
    assert_eq!(spruce_imported_json["backlog"]["state"], "offered");
    assert_eq!(spruce_imported_json["backlog"]["message_count"], 3);
    assert_eq!(spruce_imported_json["unlinked_spruce_messages"], 2);
    assert_eq!(spruce_imported_json["retained_reply_drafts"], 1);
    assert!(spruce_imported_dom.contains("Spruce Health · Records imported"));
    assert!(spruce_imported_dom.contains("Initial records imported"));
    assert!(
        spruce_imported_dom.contains("Aug 22, 2026, 12:00 PM"),
        "expected multi-day Spruce backlog timestamp with calendar date:\n{spruce_imported_dom}"
    );
    assert!(spruce_imported_dom.contains("Review backlog"));
    assert!(!spruce_imported_dom.contains("Spruce Health · Up to date"));
    walk.capture("04-spruce-imported-backlog-held");

    // Accept Spruce backlog review -> imports 3 held historical messages (total 5 linked records)
    walk.press("Review backlog");
    let after_backlog_dom =
        walk.wait_dialog_contains("Historical Spruce backlog reviewed (3 messages imported");
    let after_backlog_json = find_source(
        &get_connections(client, base, &session_token).await,
        "spruce",
    )
    .clone();
    assert_eq!(after_backlog_json["backlog"]["state"], "accepted");
    assert_eq!(after_backlog_json["backlog"]["imported_message_count"], 3);
    assert_eq!(after_backlog_json["standing"]["state"], "records_imported");
    assert_eq!(after_backlog_json["standing"]["imported_records"], 5);
    assert!(after_backlog_dom.contains("5 linked chart records"));

    // Stage C: Up to date
    walk.press("Sync latest records");
    let spruce_up_to_date_dom = walk.wait_dialog_contains("Spruce Health · Up to date");
    let spruce_up_to_date_json = find_source(
        &get_connections(client, base, &session_token).await,
        "spruce",
    )
    .clone();
    assert_eq!(spruce_up_to_date_json["standing"]["state"], "up_to_date");
    assert_eq!(spruce_up_to_date_json["standing"]["imported_records"], 5);
    assert!(!spruce_up_to_date_dom.contains("Connection verified — no records imported yet"));
    walk.capture("05-spruce-up-to-date");

    // Persist JSON+DOM state comparison artifact across all 3 Spruce states for AC3 proof.
    fs::create_dir_all(proof_dir).expect("create proof dir");
    fs::write(
        proof_dir.join(format!("ac3-spruce-three-states-{}.json", size.0)),
        serde_json::to_string_pretty(&json!({
            "viewport": size.0,
            "connected": {
                "api_source": spruce_connected_json,
                "dom_text": spruce_connected_dom,
            },
            "records_imported": {
                "api_source": spruce_imported_json,
                "dom_text": spruce_imported_dom,
            },
            "up_to_date": {
                "api_source": spruce_up_to_date_json,
                "dom_text": spruce_up_to_date_dom,
            },
        }))
        .expect("serialize ac3 comparison"),
    )
    .expect("write ac3 comparison");

    walk.press("Close");

    // 6. Quiet catch-up & transient retry stay completely silent on /today and browser DOM.
    simulate_spruce(
        client,
        base,
        &session_token,
        json!({ "event": "quiet_catch_up" }),
    )
    .await;
    let today_catch_up: Value = client
        .get(format!("{base}/today"))
        .bearer_auth(&session_token)
        .send()
        .await
        .expect("today during catch-up")
        .json()
        .await
        .expect("today catch-up json");
    assert_eq!(today_catch_up["connections"], json!([]));

    simulate_spruce(
        client,
        base,
        &session_token,
        json!({ "event": "quiet_transient_retry", "attempt": 2 }),
    )
    .await;
    let today_retry: Value = client
        .get(format!("{base}/today"))
        .bearer_auth(&session_token)
        .send()
        .await
        .expect("today during transient retry")
        .json()
        .await
        .expect("today retry json");
    assert_eq!(today_retry["connections"], json!([]));

    walk.open(&format!("{base}/"), "Nothing needs you right now.");
    let quiet_page = walk
        .run("pageText", &[])
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(!quiet_page.contains("Spruce Health:"));
    assert!(!quiet_page.contains("Reconnect"));
    assert!(!quiet_page.to_lowercase().contains("retrying"));

    // 7. Transient ReadFailed: shows 'Records couldn’t be updated' + reason + last read time,
    // keeps saved records readable, and NEVER asks for Reconnect (Rule 10).
    simulate_spruce(
        client,
        base,
        &session_token,
        json!({
            "event": "transient_read_failure",
            "reason": "Spruce API gateway 503 timeout",
        }),
    )
    .await;
    walk.open(
        &format!("{base}/"),
        "Spruce Health: Records couldn’t be updated",
    );
    let failed_today_text = walk.wait_page_contains("Spruce API gateway 503 timeout");
    assert!(failed_today_text.contains("Last successful read"));
    assert!(failed_today_text.contains("Previously saved records remain readable."));
    assert!(failed_today_text.contains("Inspect source"));
    assert!(
        !failed_today_text.contains("Reconnect"),
        "Rule 10 violation: transient ReadFailed on Today must NEVER ask to Reconnect:\n{failed_today_text}"
    );
    walk.capture("06-today-read-failed");

    walk.press("Inspect source");
    let failed_dialog_text = walk.wait_dialog_contains("Retry read");
    assert!(failed_dialog_text.contains("Records couldn’t be updated"));
    assert!(failed_dialog_text.contains("Spruce API gateway 503 timeout"));
    assert!(failed_dialog_text.contains("Last successful read"));
    let failed_buttons = walk.run("activeButtons", &[]);
    assert!(
        !failed_buttons
            .as_array()
            .expect("buttons array")
            .iter()
            .any(|b| b.as_str() == Some("Reconnect")),
        "Rule 10 violation: transient ReadFailed dialog must NEVER offer Reconnect: {failed_buttons}"
    );
    walk.capture("07-connections-read-failed");
    walk.press("Close");

    // Open patient Mary Louis -> previously saved clinical records & reply draft remain readable,
    // and source freshness reports "Records as of …, couldn't update".
    walk.open_patient(
        "Mary",
        "Spruce post-connection patient blood pressure log message linked to chart.",
    );
    let patient_during_failure =
        walk.wait_page_contains("Spruce message: blood pressure log follow-up");
    assert!(patient_during_failure.contains("Accepted historical Spruce message #1"));
    assert!(patient_during_failure.contains("Records as of"));
    assert!(
        patient_during_failure.contains("couldn't update")
            || patient_during_failure.contains("couldn’t update"),
        "expected stale refresh indicator on patient page:\n{patient_during_failure}"
    );
    walk.capture("08-patient-readable-during-read-failed");

    // 8. Lost access (AdministratorBlock + RevokedGrant): shows reason + last read time + Reconnect,
    // and reconnecting restores UpToDate.
    let admin_blocked = simulate_spruce(
        client,
        base,
        &session_token,
        json!({
            "event": "administrator_block",
            "reason": "Practice Spruce administrator disabled third-party API tokens.",
        }),
    )
    .await;
    assert_eq!(admin_blocked["source"]["standing"]["state"], "access_lost");
    assert_eq!(
        admin_blocked["source"]["standing"]["cause"],
        "administrator_block"
    );

    simulate_spruce(
        client,
        base,
        &session_token,
        json!({
            "event": "revoked_grant",
            "reason": "Spruce organization OAuth grant was revoked.",
        }),
    )
    .await;
    walk.open(&format!("{base}/"), "Spruce Health: Access lost");
    let lost_today_text = walk.wait_page_contains("Spruce organization OAuth grant was revoked.");
    assert!(lost_today_text.contains("Last successful read"));
    assert!(lost_today_text.contains("Reconnect"));
    walk.capture("09-today-access-lost");

    walk.press("Reconnect");
    let lost_dialog_text = walk.wait_dialog_contains("Access lost: Access revoked");
    assert!(lost_dialog_text.contains("Spruce organization OAuth grant was revoked."));
    assert!(lost_dialog_text.contains("Last successful read"));
    assert!(lost_dialog_text.contains("Previously saved records remain readable."));
    walk.capture("10-connections-access-lost");

    walk.fill_field(
        "API credential or bearer secret",
        "spruce-rotated-grant-token-2026",
    );
    walk.press("Reconnect");
    walk.wait_dialog_contains("Spruce Health · Up to date");
    let reconnected_json = find_source(
        &get_connections(client, base, &session_token).await,
        "spruce",
    )
    .clone();
    assert_eq!(reconnected_json["standing"]["state"], "up_to_date");

    // 9. Disconnect retention (Rule 11): purges never-linked Spruce messages (2 -> 0) while
    // retaining linked clinical records (5) and physician reply drafts (1).
    assert_eq!(reconnected_json["unlinked_spruce_messages"], 2);
    assert_eq!(reconnected_json["retained_linked_records"], 5);
    assert_eq!(reconnected_json["retained_reply_drafts"], 1);

    walk.press("Disconnect");
    let disconnected_dom = walk.wait_dialog_contains("Spruce disconnected");
    let disconnected_json = find_source(
        &get_connections(client, base, &session_token).await,
        "spruce",
    )
    .clone();
    assert_eq!(disconnected_json["standing"]["state"], "disconnected");
    assert_eq!(disconnected_json["unlinked_spruce_messages"], 0);
    assert_eq!(disconnected_json["retained_linked_records"], 5);
    assert_eq!(disconnected_json["retained_reply_drafts"], 1);
    assert!(disconnected_dom.contains("0 unlinked Spruce messages retained"));
    assert!(
        disconnected_dom
            .contains("5 linked chart records and 1 physician reply draft remain readable")
    );
    walk.capture("11-spruce-disconnected-retention");

    walk.press("Close");

    // Confirm Mary Louis's patient page still renders retained linked records and reply draft!
    walk.open_patient(
        "Mary",
        "Spruce post-connection patient blood pressure log message linked to chart.",
    );
    let patient_after_disconnect =
        walk.wait_page_contains("Spruce message: blood pressure log follow-up");
    assert!(patient_after_disconnect.contains("Accepted historical Spruce message #1"));
    walk.capture("12-patient-retained-after-disconnect");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn desktop_and_phone_complete_source_connection_backlog_recovery_and_disconnect() {
    let site_dir = shared_site();
    // `site_file` serves `/physician-app.js` and `/physician-app_bg.wasm` from `PHYSICIAN_SITE_DIR`.
    unsafe {
        std::env::set_var("PHYSICIAN_SITE_DIR", &site_dir);
    }
    let db = DisposableDb::create("source_setup").await;
    let db_url = db.url.clone();
    let site_dir_str = site_dir.to_str().expect("utf8 site dir").to_owned();

    let (router, _) = physician_sample::site_router_from_env(move |key| match key {
        "PHYSICIAN_AUTH" => Some("required".to_owned()),
        "PHYSICIAN_DATABASE_URL" => Some(db_url.clone()),
        "PHYSICIAN_SITE_DIR" => Some(site_dir_str.clone()),
        "CODA_STAFF_SETUP_TOKEN" => Some(STAFF_SECRET.to_owned()),
        _ => None,
    })
    .await
    .expect("build source setup router");

    let (addr, srv) = spawn_router(router).await;
    let base = format!("http://{addr}");
    let client = reqwest::Client::new();
    let pg_auth = PostgresAuthStore::connect_without_migrations(&db.url)
        .await
        .expect("connect pg auth");
    let runtime = Store::connect(&db.url).await.expect("connect runtime");

    let proof_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/verification/captures/eng-317-319");

    for size in [("desktop", DESKTOP), ("phone", PHONE)] {
        walk_viewport(size, &base, &pg_auth, &runtime, &client, &proof_dir).await;
    }

    srv.abort();
}
