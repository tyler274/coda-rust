//! Postgres-backed qualification for Coda staff practice creation, hosted physician
//! invitation sign-in, physician account lifecycle (create, deactivate, reactivate),
//! and cross-practice isolation (`/auth/staff/*`).

use std::env;
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex as StdMutex, Weak};
use std::time::Duration;

use base64::Engine as _;
use front_door::PostgresFrontDoorStore;
use local_product::physician_sources::{
    MedicalRecordNumber, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
use local_product::runtime::Store;
use local_product::runtime::sources::SourcePull;
use physician_auth::{AuthStore, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use sea_orm::ConnectionTrait;
use serde_json::{Value, json};
use uuid::Uuid;

const STAFF_SECRET: &str = "coda-staff-setup-secret-2026-test-token";
const WRONG_SECRET: &str = "wrong-staff-setup-secret-2026-test-token";
const FIXED_CLOCK_UNIX: u64 = 1_787_846_400;

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
        let port = 24_000 + (Uuid::new_v4().as_u128() % 7_000) as u16;
        if TcpListener::bind(("127.0.0.1", port)).is_ok() {
            return port;
        }
    }
    panic!("no free loopback port for test postgres cluster");
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
        "coda-staff-accounts-pg-{}-{}",
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
        "-F -N 300 -p {port} -h 127.0.0.1 -k {}",
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
        let db_name = format!("staff_acct_{label}_{}", Uuid::new_v4().simple());
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
            .args(["--maintenance-db", &self.admin_url, &self.db_name])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}

async fn spawn_router(router: axum::Router) -> (SocketAddr, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind loopback");
    let addr = listener.local_addr().expect("local addr");
    let handle = tokio::spawn(async move {
        axum::serve(listener, router).await.expect("serve router");
    });
    (addr, handle)
}

fn decode_mail_part(raw: &str, mime: &str) -> String {
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(raw)
        .expect("url-safe base64 MIME payload");
    let message = mailparse::parse_mail(&bytes).expect("parse MIME email");
    if message.subparts.is_empty() {
        return message.get_body().expect("single-part body");
    }
    message
        .subparts
        .iter()
        .find(|part| part.ctype.mimetype == mime)
        .expect("matching MIME part")
        .get_body()
        .expect("decoded MIME body")
}

fn extract_code_and_email_from_mail(text_body: &str, origin: &str) -> (String, String) {
    let link_line = text_body
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with(origin))
        .unwrap_or_else(|| panic!("expected link starting with {origin} in:\n{text_body}"));
    let (_, query) = link_line
        .split_once('?')
        .unwrap_or_else(|| panic!("expected query parameters in {link_line}"));
    let mut code = None;
    let mut email = None;
    for pair in query.split('&') {
        if let Some(val) = pair.strip_prefix("code=") {
            code = Some(percent_decode(val));
        } else if let Some(val) = pair.strip_prefix("email=") {
            email = Some(percent_decode(val));
        }
    }
    (
        code.expect("code param in mailed link"),
        email.expect("email param in mailed link"),
    )
}

fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut idx = 0;
    while idx < bytes.len() {
        if bytes[idx] == b'%'
            && idx + 2 < bytes.len()
            && let Ok(hex) = std::str::from_utf8(&bytes[idx + 1..idx + 3])
            && let Ok(val) = u8::from_str_radix(hex, 16)
        {
            out.push(val);
            idx += 3;
            continue;
        }
        out.push(bytes[idx]);
        idx += 1;
    }
    String::from_utf8(out).expect("utf-8 query param")
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn staff_routes_fail_closed_when_unset_short_or_unauthorized() {
    let short_hosted = |key: &str| match key {
        "K_SERVICE" => Some("physician".to_owned()),
        "CODA_ENVIRONMENT" => Some("stg".to_owned()),
        "CODA_HOSTED_PROFILE" => Some("staging-preview".to_owned()),
        "PHYSICIAN_AUTH" => Some("required".to_owned()),
        "PHYSICIAN_DATABASE_URL" => Some("postgresql://localhost/coda".to_owned()),
        "CODA_STAFF_SETUP_TOKEN" => Some("too-short".to_owned()),
        _ => None,
    };
    let err = physician_sample::hosting::validate_startup(short_hosted)
        .expect_err("short staff token must fail closed");
    assert!(err.contains("CODA_STAFF_SETUP_TOKEN"), "{err}");

    let valid_hosted = |key: &str| match key {
        "K_SERVICE" => Some("physician".to_owned()),
        "CODA_ENVIRONMENT" => Some("stg".to_owned()),
        "CODA_HOSTED_PROFILE" => Some("staging-preview".to_owned()),
        "PHYSICIAN_AUTH" => Some("required".to_owned()),
        "PHYSICIAN_DATABASE_URL" => Some("postgresql://localhost/coda".to_owned()),
        "CODA_STAFF_SETUP_TOKEN" => Some(STAFF_SECRET.to_owned()),
        _ => None,
    };
    physician_sample::hosting::validate_startup(valid_hosted)
        .expect("hosted startup succeeds with valid staff token and zero rehearsal config");

    let without_auth = |key: &str| match key {
        "CODA_STAFF_SETUP_TOKEN" => Some(STAFF_SECRET.to_owned()),
        _ => None,
    };
    let err = match physician_sample::site_router_from_env(without_auth).await {
        Ok(_) => panic!("staff setup without PHYSICIAN_AUTH=required must fail"),
        Err(err) => err,
    };
    assert!(err.contains("PHYSICIAN_AUTH=required"), "{err}");

    let db = DisposableDb::create("fail_closed").await;
    let db_url = db.url.clone();
    let (unset_router, _) = physician_sample::site_router_from_env(move |key| match key {
        "PHYSICIAN_AUTH" => Some("required".to_owned()),
        "PHYSICIAN_DATABASE_URL" => Some(db_url.clone()),
        _ => None,
    })
    .await
    .expect("router without staff token");
    let (unset_addr, unset_srv) = spawn_router(unset_router).await;
    let client = reqwest::Client::new();

    let resp = client
        .put(format!(
            "http://{unset_addr}/auth/staff/practices/practice-hidden"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({ "time_zone": "America/New_York" }))
        .send()
        .await
        .expect("send to unmounted staff route");
    assert_eq!(resp.status(), reqwest::StatusCode::NOT_FOUND);
    unset_srv.abort();

    let db_url = db.url.clone();
    let (mounted_router, _) = physician_sample::site_router_from_env(move |key| match key {
        "PHYSICIAN_AUTH" => Some("required".to_owned()),
        "PHYSICIAN_DATABASE_URL" => Some(db_url.clone()),
        "CODA_STAFF_SETUP_TOKEN" => Some(STAFF_SECRET.to_owned()),
        _ => None,
    })
    .await
    .expect("router with staff token");
    let (addr, srv) = spawn_router(mounted_router).await;

    let missing_auth = client
        .get(format!("http://{addr}/auth/staff/practices/practice-a"))
        .send()
        .await
        .expect("missing bearer");
    assert_eq!(missing_auth.status(), reqwest::StatusCode::UNAUTHORIZED);
    let body: Value = missing_auth.json().await.expect("401 json");
    assert_eq!(body["code"], "staff_unauthorized");

    let wrong_auth = client
        .put(format!("http://{addr}/auth/staff/practices/practice-a"))
        .bearer_auth(WRONG_SECRET)
        .json(&json!({ "time_zone": "America/New_York" }))
        .send()
        .await
        .expect("wrong bearer");
    assert_eq!(wrong_auth.status(), reqwest::StatusCode::UNAUTHORIZED);

    // Rehearsal endpoint remains completely absent when only CODA_STAFF_SETUP_TOKEN is configured.
    let rehearsal = client
        .post(format!("http://{addr}/auth/rehearsal"))
        .send()
        .await
        .expect("rehearsal check");
    assert_eq!(rehearsal.status(), reqwest::StatusCode::NOT_FOUND);

    srv.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn staff_creates_synthetic_practice_and_invited_physician_completes_hosted_sign_in() {
    let db = DisposableDb::create("hosted_invite").await;
    let fake_gmail = gmail::testing::spawn(false).await;
    let endpoints = fake_gmail.endpoints();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind listener");
    let addr = listener.local_addr().expect("local addr");
    let public_origin = format!("http://{addr}");
    let db_url = db.url.clone();
    let origin_for_env = public_origin.clone();
    let send_url = endpoints.send.clone();
    let token_url = endpoints.token.clone();

    let (router, _) = physician_sample::site_router_from_env(move |key| match key {
        "PHYSICIAN_AUTH" => Some("required".to_owned()),
        "PHYSICIAN_DATABASE_URL" => Some(db_url.clone()),
        "CODA_STAFF_SETUP_TOKEN" => Some(STAFF_SECRET.to_owned()),
        "PHYSICIAN_GMAIL_FROM" => Some("signin@coda.test".to_owned()),
        "PHYSICIAN_PUBLIC_ORIGIN" => Some(origin_for_env.clone()),
        "PHYSICIAN_GMAIL_CLIENT_ID" => Some("client-id".to_owned()),
        "PHYSICIAN_GMAIL_CLIENT_SECRET" => Some("client-secret".to_owned()),
        "PHYSICIAN_GMAIL_REFRESH_TOKEN" => Some("refresh-token".to_owned()),
        "PHYSICIAN_GMAIL_SEND_URL" => Some(send_url.clone()),
        "PHYSICIAN_GMAIL_TOKEN_URL" => Some(token_url.clone()),
        _ => None,
    })
    .await
    .expect("build hosted-mail site router");
    let srv = tokio::spawn(async move {
        axum::serve(listener, router).await.expect("serve");
    });

    let client = reqwest::Client::new();

    // 1. Invalid IANA timezone is rejected with 422.
    let bad_tz = client
        .put(format!(
            "{public_origin}/auth/staff/practices/practice-synth-west"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "clock": { "mode": "fixed", "unix_seconds": FIXED_CLOCK_UNIX },
            "time_zone": "Mars/Phobos",
        }))
        .send()
        .await
        .expect("bad tz request");
    assert_eq!(bad_tz.status(), reqwest::StatusCode::UNPROCESSABLE_ENTITY);
    let bad_tz_body: Value = bad_tz.json().await.expect("bad tz json");
    assert_eq!(bad_tz_body["code"], "invalid_time_zone");

    // 2. Create synthetic practice with validated IANA time zone.
    let created = client
        .put(format!(
            "{public_origin}/auth/staff/practices/practice-synth-west"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "clock": { "mode": "fixed", "unix_seconds": FIXED_CLOCK_UNIX },
            "time_zone": "America/Los_Angeles",
        }))
        .send()
        .await
        .expect("create practice");
    assert_eq!(created.status(), reqwest::StatusCode::CREATED);
    let created_body: Value = created.json().await.expect("created json");
    assert_eq!(created_body["outcome"], "created");
    assert_eq!(created_body["practice_id"], "practice-synth-west");
    assert_eq!(created_body["time_zone"], "America/Los_Angeles");
    assert_eq!(
        created_body["clock"],
        json!({ "mode": "fixed", "unix_seconds": FIXED_CLOCK_UNIX })
    );

    // Idempotent repeat returns 200 unchanged; conflicting zone returns 409.
    let unchanged = client
        .put(format!(
            "{public_origin}/auth/staff/practices/practice-synth-west"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "clock": { "mode": "fixed", "unix_seconds": FIXED_CLOCK_UNIX },
            "time_zone": "America/Los_Angeles",
        }))
        .send()
        .await
        .expect("idempotent practice");
    assert_eq!(unchanged.status(), reqwest::StatusCode::OK);
    let unchanged_body: Value = unchanged.json().await.expect("unchanged json");
    assert_eq!(unchanged_body["outcome"], "unchanged");

    let conflict = client
        .put(format!(
            "{public_origin}/auth/staff/practices/practice-synth-west"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "clock": { "mode": "fixed", "unix_seconds": FIXED_CLOCK_UNIX },
            "time_zone": "America/Chicago",
        }))
        .send()
        .await
        .expect("conflicting practice");
    assert_eq!(conflict.status(), reqwest::StatusCode::CONFLICT);

    // Verify saved Postgres practice row directly.
    let pg_auth = PostgresAuthStore::connect_without_migrations(&db.url)
        .await
        .expect("connect pg auth");
    let saved_practice = pg_auth
        .practice("practice-synth-west")
        .await
        .expect("read practice")
        .expect("practice exists in Postgres");
    assert_eq!(saved_practice.clock, PracticeClock::Fixed(FIXED_CLOCK_UNIX));
    assert_eq!(saved_practice.time_zone.as_str(), "America/Los_Angeles");

    // 3. Invite a physician through the staff route; hosted mail sends the single-use link and omits the token from HTTP JSON.
    let invite_resp = client
        .post(format!(
            "{public_origin}/auth/staff/practices/practice-synth-west/invitations"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "email": "dr.west@synth-west.example",
            "display_name": "Dr. Maya West",
            "role": "admin",
        }))
        .send()
        .await
        .expect("invite physician");
    assert_eq!(invite_resp.status(), reqwest::StatusCode::OK);
    let invite_body: Value = invite_resp.json().await.expect("invite json");
    assert_eq!(invite_body["status"], "invited");
    assert_eq!(invite_body["delivery"], "mailed");
    assert!(
        invite_body.get("token").is_none(),
        "hosted invitation response must never leak raw token: {invite_body}"
    );

    let (invite_code, mailed_email) = {
        let sent = fake_gmail.sent.lock().expect("gmail sent lock");
        assert_eq!(sent.len(), 1, "expected 1 invitation email");
        let text = decode_mail_part(sent[0]["raw"].as_str().expect("raw mime"), "text/plain");
        extract_code_and_email_from_mail(&text, &public_origin)
    };
    assert_eq!(mailed_email, "dr.west@synth-west.example");

    // 4. Physician opens the invitation entry and completes sign-in via POST /auth/verify.
    let verify_resp = client
        .post(format!("{public_origin}/auth/verify"))
        .json(&json!({
            "email": mailed_email,
            "token": invite_code,
        }))
        .send()
        .await
        .expect("redeem physician invitation");
    assert_eq!(verify_resp.status(), reqwest::StatusCode::OK);
    let verify_body: Value = verify_resp.json().await.expect("verify json");
    let session_token = verify_body["session"]
        .as_str()
        .expect("session token")
        .to_owned();
    assert_eq!(verify_body["practice_id"], "practice-synth-west");
    let redeemed_physician_id = verify_body["physician_id"]
        .as_str()
        .expect("physician_id")
        .to_owned();
    assert!(
        redeemed_physician_id.len() == 32
            && redeemed_physician_id
                .chars()
                .all(|ch| ch.is_ascii_hexdigit()),
        "expected redeemed physician_id, got {redeemed_physician_id}"
    );

    let whoami: Value = client
        .get(format!("{public_origin}/auth/whoami"))
        .bearer_auth(&session_token)
        .send()
        .await
        .expect("whoami")
        .json()
        .await
        .expect("whoami json");
    assert_eq!(whoami["practice_id"], "practice-synth-west");
    assert_eq!(whoami["physician_id"], redeemed_physician_id);
    assert_eq!(whoami["display_name"], "Dr. Maya West");
    assert_eq!(whoami["admin"], true);

    let today_resp = client
        .get(format!("{public_origin}/today"))
        .bearer_auth(&session_token)
        .send()
        .await
        .expect("today");
    assert_eq!(today_resp.status(), reqwest::StatusCode::OK);

    // Reusing the single-use invitation code fails closed.
    let replay = client
        .post(format!("{public_origin}/auth/verify"))
        .json(&json!({
            "email": "dr.west@synth-west.example",
            "token": invite_code,
        }))
        .send()
        .await
        .expect("replay invite");
    assert_eq!(replay.status(), reqwest::StatusCode::UNAUTHORIZED);

    // Subsequent standard hosted sign-in works for the redeemed physician account.
    let sign_in_resp = client
        .post(format!("{public_origin}/auth/sign-in"))
        .json(&json!({ "email": "dr.west@synth-west.example" }))
        .send()
        .await
        .expect("subsequent sign-in");
    assert_eq!(sign_in_resp.status(), reqwest::StatusCode::OK);
    let (next_code, _) = {
        let sent = fake_gmail.sent.lock().expect("gmail sent lock");
        assert_eq!(sent.len(), 2, "expected second email for standard sign-in");
        let text = decode_mail_part(sent[1]["raw"].as_str().expect("raw mime"), "text/plain");
        extract_code_and_email_from_mail(&text, &public_origin)
    };
    let next_verify = client
        .post(format!("{public_origin}/auth/verify"))
        .json(&json!({
            "email": "dr.west@synth-west.example",
            "token": next_code,
        }))
        .send()
        .await
        .expect("verify subsequent sign-in");
    assert_eq!(next_verify.status(), reqwest::StatusCode::OK);

    srv.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn staff_creates_deactivates_and_reactivates_physician_with_immediate_session_revocation() {
    let db = DisposableDb::create("lifecycle").await;
    let db_url = db.url.clone();
    let (router, _) = physician_sample::site_router_from_env(move |key| match key {
        "PHYSICIAN_AUTH" => Some("required".to_owned()),
        "PHYSICIAN_DATABASE_URL" => Some(db_url.clone()),
        "CODA_STAFF_SETUP_TOKEN" => Some(STAFF_SECRET.to_owned()),
        _ => None,
    })
    .await
    .expect("build lifecycle router");
    let (addr, srv) = spawn_router(router).await;
    let base = format!("http://{addr}");
    let client = reqwest::Client::new();
    let pg_auth = PostgresAuthStore::connect_without_migrations(&db.url)
        .await
        .expect("connect pg auth");

    // 1. Create practice and active physician account.
    let practice_resp = client
        .put(format!("{base}/auth/staff/practices/practice-lifecycle"))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "clock": { "mode": "fixed", "unix_seconds": FIXED_CLOCK_UNIX },
            "time_zone": "America/New_York",
        }))
        .send()
        .await
        .expect("create practice");
    assert_eq!(practice_resp.status(), reqwest::StatusCode::CREATED);

    let create_acct = client
        .post(format!(
            "{base}/auth/staff/practices/practice-lifecycle/accounts"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "physician_id": "dr-chen",
            "email": "dr.chen@lifecycle.example",
            "display_name": "Dr. Marcus Chen",
            "role": "physician",
        }))
        .send()
        .await
        .expect("create physician account");
    assert_eq!(create_acct.status(), reqwest::StatusCode::CREATED);
    let acct_json: Value = create_acct.json().await.expect("account json");
    assert_eq!(acct_json["outcome"], "created");
    assert_eq!(acct_json["account"]["active"], true);
    assert_eq!(acct_json["account"]["sessions_ended_before"], 0);

    let identity = PhysicianIdentity {
        practice_id: "practice-lifecycle".to_owned(),
        physician_id: "dr-chen".to_owned(),
    };
    let saved_created = pg_auth
        .account_by_email("dr.chen@lifecycle.example")
        .await
        .expect("pg account after create");
    assert_eq!(saved_created.identity, identity);
    assert!(saved_created.active);
    assert_eq!(saved_created.sessions_ended_before, 0);

    // 2. Physician signs in and opens both an HTTP read and a live SSE event stream.
    let sign_in_1: Value = client
        .post(format!("{base}/auth/sign-in"))
        .json(&json!({ "email": "dr.chen@lifecycle.example" }))
        .send()
        .await
        .expect("sign-in 1")
        .json()
        .await
        .expect("sign-in 1 json");
    let code_1 = sign_in_1["token"].as_str().expect("proof token 1");
    let verify_1: Value = client
        .post(format!("{base}/auth/verify"))
        .json(&json!({
            "email": "dr.chen@lifecycle.example",
            "token": code_1,
        }))
        .send()
        .await
        .expect("verify 1")
        .json()
        .await
        .expect("verify 1 json");
    let session_1 = verify_1["session"].as_str().expect("session 1").to_owned();

    let today_ok = client
        .get(format!("{base}/today"))
        .bearer_auth(&session_1)
        .send()
        .await
        .expect("today while active");
    assert_eq!(today_ok.status(), reqwest::StatusCode::OK);

    let mut sse_resp = client
        .get(format!("{base}/patient-workspaces/events?after=0"))
        .bearer_auth(&session_1)
        .send()
        .await
        .expect("open SSE stream while active");
    assert_eq!(sse_resp.status(), reqwest::StatusCode::OK);

    // 3. Deactivate physician via staff route; verify Postgres state, immediate HTTP 401, SSE termination, and sign-in block.
    let deactivate_resp = client
        .post(format!(
            "{base}/auth/staff/practices/practice-lifecycle/accounts/deactivate"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({ "email": "dr.chen@lifecycle.example" }))
        .send()
        .await
        .expect("deactivate physician");
    assert_eq!(deactivate_resp.status(), reqwest::StatusCode::OK);
    let deact_json: Value = deactivate_resp.json().await.expect("deactivate json");
    assert_eq!(deact_json["account"]["active"], false);
    let cutoff = deact_json["account"]["sessions_ended_before"]
        .as_u64()
        .expect("cutoff timestamp");
    assert!(cutoff > 0);

    let saved_deactivated = pg_auth
        .account_by_email("dr.chen@lifecycle.example")
        .await
        .expect("pg account after deactivate");
    assert_eq!(saved_deactivated.identity, identity);
    assert!(!saved_deactivated.active);
    assert_eq!(saved_deactivated.sessions_ended_before, cutoff);

    // Allow NOTIFY physician_authority invalidation to reach CachedAuthStore (< 2s).
    let mut saw_revoked_401 = false;
    for _ in 0..40 {
        let check = client
            .get(format!("{base}/today"))
            .bearer_auth(&session_1)
            .send()
            .await
            .expect("today after deactivate");
        if check.status() == reqwest::StatusCode::UNAUTHORIZED {
            let err_body: Value = check.json().await.expect("401 body");
            assert_eq!(err_body["reason"], "access_removed");
            saw_revoked_401 = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert!(
        saw_revoked_401,
        "active session must be immediately invalidated upon deactivation"
    );

    // Open SSE stream terminates cleanly once authority changes.
    let sse_closed = tokio::time::timeout(Duration::from_secs(5), async {
        while let Some(_chunk) = sse_resp.chunk().await.expect("read sse chunk") {}
    })
    .await;
    assert!(
        sse_closed.is_ok(),
        "open SSE stream must close promptly when physician account is deactivated"
    );

    // Deactivated physician cannot request a new sign-in token.
    let blocked_sign_in: Value = client
        .post(format!("{base}/auth/sign-in"))
        .json(&json!({ "email": "dr.chen@lifecycle.example" }))
        .send()
        .await
        .expect("blocked sign-in")
        .json()
        .await
        .expect("blocked sign-in json");
    assert_eq!(blocked_sign_in["status"], "unavailable");
    assert!(blocked_sign_in.get("token").is_none());

    // 4. Reactivate physician via staff route; verify Postgres state, old session stays dead, new sign-in succeeds.
    let reactivate_resp = client
        .post(format!(
            "{base}/auth/staff/practices/practice-lifecycle/accounts/reactivate"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({ "email": "dr.chen@lifecycle.example" }))
        .send()
        .await
        .expect("reactivate physician");
    assert_eq!(reactivate_resp.status(), reqwest::StatusCode::OK);
    let react_json: Value = reactivate_resp.json().await.expect("reactivate json");
    assert_eq!(react_json["account"]["active"], true);
    assert_eq!(react_json["account"]["sessions_ended_before"], cutoff);

    let saved_reactivated = pg_auth
        .account_by_email("dr.chen@lifecycle.example")
        .await
        .expect("pg account after reactivate");
    assert_eq!(saved_reactivated.identity, identity);
    assert!(saved_reactivated.active);
    assert_eq!(saved_reactivated.sessions_ended_before, cutoff);

    // Old session from before deactivation MUST remain invalid.
    let old_session_check = client
        .get(format!("{base}/today"))
        .bearer_auth(&session_1)
        .send()
        .await
        .expect("old session after reactivate");
    assert_eq!(
        old_session_check.status(),
        reqwest::StatusCode::UNAUTHORIZED
    );

    // Wait briefly for CachedAuthStore invalidation if needed, then sign in afresh.
    let mut new_code = None;
    for _ in 0..40 {
        let attempt: Value = client
            .post(format!("{base}/auth/sign-in"))
            .json(&json!({ "email": "dr.chen@lifecycle.example" }))
            .send()
            .await
            .expect("sign-in after reactivate")
            .json()
            .await
            .expect("sign-in json");
        if let Some(tok) = attempt.get("token").and_then(Value::as_str) {
            new_code = Some(tok.to_owned());
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let code_2 = new_code.expect("reactivated physician receives new sign-in token");
    let verify_2: Value = client
        .post(format!("{base}/auth/verify"))
        .json(&json!({
            "email": "dr.chen@lifecycle.example",
            "token": code_2,
        }))
        .send()
        .await
        .expect("verify 2")
        .json()
        .await
        .expect("verify 2 json");
    let session_2 = verify_2["session"].as_str().expect("session 2");

    let today_restored = client
        .get(format!("{base}/today"))
        .bearer_auth(session_2)
        .send()
        .await
        .expect("today with new session");
    assert_eq!(today_restored.status(), reqwest::StatusCode::OK);

    srv.abort();
}

async fn sign_in_proof(client: &reqwest::Client, base: &str, email: &str) -> String {
    let issued: Value = client
        .post(format!("{base}/auth/sign-in"))
        .json(&json!({ "email": email }))
        .send()
        .await
        .expect("sign-in")
        .json()
        .await
        .expect("sign-in json");
    let code = issued["token"].as_str().expect("proof token");
    let verified: Value = client
        .post(format!("{base}/auth/verify"))
        .json(&json!({ "email": email, "token": code }))
        .send()
        .await
        .expect("verify")
        .json()
        .await
        .expect("verify json");
    verified["session"]
        .as_str()
        .expect("bearer session")
        .to_owned()
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
        candidate_id: format!("{workspace_id}-seed-rev-1"),
        revision: format!("rev-{}", &sha256[..12]),
        record_type: "source_record".to_owned(),
        dates: vec![RecordDate {
            name: "recorded_at".to_owned(),
            value: "2026-08-27T16:00:00Z".to_owned(),
        }],
        content,
        sha256,
        readable_parts: vec![ReadablePart {
            label: "Problem list".to_owned(),
            text: format!("{full_name} routine primary care chart."),
        }],
        provenance: json!({
            "source_id": "elation_fhir",
            "native_id": format!("{workspace_id}-seed-rev-1"),
        }),
        envelope: None,
    };
    let pull = SourcePull {
        source_id: "elation_fhir".to_owned(),
        source_label: "Elation / FHIR".to_owned(),
        patient: SourcePatient {
            source_patient_id: workspace_id.to_owned(),
            display_name: full_name.to_owned(),
            birth_date: Some("1980-05-14".to_owned()),
            sex_or_gender: Some("female".to_owned()),
            medical_record_numbers: vec![MedicalRecordNumber {
                system: "Clinic".to_owned(),
                value: mrn.to_owned(),
            }],
        },
        pulled_at: Some(FIXED_CLOCK_UNIX as i64),
    };
    runtime
        .record_chart(workspace_id, &[rev], &pull)
        .await
        .expect("seed chart");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn new_practice_starts_with_zero_patients_and_enforces_strict_cross_practice_isolation() {
    let db = DisposableDb::create("isolation").await;
    let db_url = db.url.clone();
    let (router, _) = physician_sample::site_router_from_env(move |key| match key {
        "PHYSICIAN_AUTH" => Some("required".to_owned()),
        "PHYSICIAN_DATABASE_URL" => Some(db_url.clone()),
        "CODA_STAFF_SETUP_TOKEN" => Some(STAFF_SECRET.to_owned()),
        _ => None,
    })
    .await
    .expect("build isolation router");
    let (addr, srv) = spawn_router(router).await;
    let base = format!("http://{addr}");
    let client = reqwest::Client::new();
    let pg_auth = PostgresAuthStore::connect_without_migrations(&db.url)
        .await
        .expect("connect pg auth");
    let runtime = Store::connect(&db.url).await.expect("connect runtime");

    // Create practice-alpha and practice-beta and one admin in each.
    for (practice_id, tz, phys_id, email, name) in [
        (
            "practice-alpha",
            "America/New_York",
            "admin-alpha",
            "admin@alpha.example",
            "Dr. Alpha Admin",
        ),
        (
            "practice-beta",
            "America/Chicago",
            "admin-beta",
            "admin@beta.example",
            "Dr. Beta Admin",
        ),
    ] {
        let p = client
            .put(format!("{base}/auth/staff/practices/{practice_id}"))
            .bearer_auth(STAFF_SECRET)
            .json(&json!({
                "clock": { "mode": "fixed", "unix_seconds": FIXED_CLOCK_UNIX },
                "time_zone": tz,
            }))
            .send()
            .await
            .expect("create practice");
        assert_eq!(p.status(), reqwest::StatusCode::CREATED);

        let a = client
            .post(format!(
                "{base}/auth/staff/practices/{practice_id}/accounts"
            ))
            .bearer_auth(STAFF_SECRET)
            .json(&json!({
                "physician_id": phys_id,
                "email": email,
                "display_name": name,
                "role": "admin",
            }))
            .send()
            .await
            .expect("create admin");
        assert_eq!(a.status(), reqwest::StatusCode::CREATED);
    }

    // Seed Elena Vance in practice-alpha and assign her to admin-alpha.
    seed_practice_patient(
        &pg_auth,
        &runtime,
        "practice-alpha",
        "pw-alpha-001",
        "Elena Vance",
        "MRN-ALPHA-1",
    )
    .await;

    let token_alpha = sign_in_proof(&client, &base, "admin@alpha.example").await;
    let token_beta = sign_in_proof(&client, &base, "admin@beta.example").await;

    let mut assigned_alpha = false;
    for _ in 0..40 {
        let assign_alpha = client
            .put(format!("{base}/auth/panels/admin-alpha/pw-alpha-001"))
            .bearer_auth(&token_alpha)
            .send()
            .await
            .expect("assign alpha");
        if assign_alpha.status() == reqwest::StatusCode::OK {
            assigned_alpha = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert!(assigned_alpha, "admin-alpha assigns own practice patient");

    // Newly created practice-beta admin sees 0 patients on panel and 0 patients on roster.
    let beta_workspaces: Value = client
        .get(format!("{base}/patient-workspaces"))
        .bearer_auth(&token_beta)
        .send()
        .await
        .expect("beta workspaces")
        .json()
        .await
        .expect("beta workspaces json");
    assert_eq!(beta_workspaces["patients"], json!([]));

    let beta_roster: Value = client
        .get(format!("{base}/patient-roster?query="))
        .bearer_auth(&token_beta)
        .send()
        .await
        .expect("beta roster")
        .json()
        .await
        .expect("beta roster json");
    assert_eq!(beta_roster["patients"], json!([]));

    let beta_search_alpha: Value = client
        .get(format!("{base}/patient-roster?query=Elena"))
        .bearer_auth(&token_beta)
        .send()
        .await
        .expect("beta search alpha")
        .json()
        .await
        .expect("beta search alpha json");
    assert_eq!(beta_search_alpha["patients"], json!([]));

    // Cross-practice reads and mutations from practice-beta against practice-alpha return 404.
    for resp in [
        client
            .get(format!("{base}/patient-workspaces/pw-alpha-001"))
            .bearer_auth(&token_beta)
            .send()
            .await
            .expect("read alpha workspace"),
        client
            .put(format!("{base}/auth/panels/admin-beta/pw-alpha-001"))
            .bearer_auth(&token_beta)
            .send()
            .await
            .expect("assign alpha patient to beta panel"),
        client
            .put(format!("{base}/auth/panels/admin-alpha/pw-alpha-001"))
            .bearer_auth(&token_beta)
            .send()
            .await
            .expect("mutate alpha physician panel"),
        client
            .get(format!("{base}/auth/panels/admin-alpha"))
            .bearer_auth(&token_beta)
            .send()
            .await
            .expect("read alpha physician panel"),
        client
            .post(format!("{base}/auth/invitations"))
            .bearer_auth(&token_beta)
            .json(&json!({
                "patient_workspace_id": "pw-alpha-001",
                "email": "elena@alpha.example",
                "display_name": "Elena Vance",
            }))
            .send()
            .await
            .expect("invite alpha patient from beta"),
    ] {
        assert_eq!(
            resp.status(),
            reqwest::StatusCode::NOT_FOUND,
            "cross-practice operation must fail closed with 404"
        );
    }

    // Once practice-beta registers and assigns its own patient, admin-beta sees only that patient.
    seed_practice_patient(
        &pg_auth,
        &runtime,
        "practice-beta",
        "pw-beta-001",
        "Marcus Sterling",
        "MRN-BETA-1",
    )
    .await;
    // Wait briefly if CachedAuthStore needs to receive register_patient notification.
    let mut assigned_beta = false;
    for _ in 0..40 {
        let resp = client
            .put(format!("{base}/auth/panels/admin-beta/pw-beta-001"))
            .bearer_auth(&token_beta)
            .send()
            .await
            .expect("assign beta patient");
        if resp.status() == reqwest::StatusCode::OK {
            assigned_beta = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert!(assigned_beta, "admin-beta assigns own practice patient");

    let beta_after: Value = client
        .get(format!("{base}/patient-workspaces"))
        .bearer_auth(&token_beta)
        .send()
        .await
        .expect("beta workspaces after assign")
        .json()
        .await
        .expect("beta workspaces after assign json");
    let beta_list = beta_after["patients"].as_array().expect("patients array");
    assert_eq!(beta_list.len(), 1);
    assert_eq!(beta_list[0]["display_name"], "Marcus Sterling");

    let alpha_cross_check = client
        .get(format!("{base}/patient-workspaces/pw-beta-001"))
        .bearer_auth(&token_alpha)
        .send()
        .await
        .expect("alpha cross check beta");
    assert_eq!(alpha_cross_check.status(), reqwest::StatusCode::NOT_FOUND);

    srv.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn practice_time_zone_and_account_conflict_guards_hold_across_routes() {
    // 2026-08-28T05:30:00Z == 2026-08-27T22:30:00-07:00 in America/Los_Angeles.
    const PACIFIC_LATE_EVENING_UNIX: u64 = 1_787_895_000;

    let db = DisposableDb::create("tz_and_conflicts").await;
    let db_url = db.url.clone();
    let (router, _) = physician_sample::site_router_from_env(move |key| match key {
        "PHYSICIAN_AUTH" => Some("required".to_owned()),
        "PHYSICIAN_DATABASE_URL" => Some(db_url.clone()),
        "CODA_STAFF_SETUP_TOKEN" => Some(STAFF_SECRET.to_owned()),
        _ => None,
    })
    .await
    .expect("build tz router");
    let (addr, srv) = spawn_router(router).await;
    let base = format!("http://{addr}");
    let client = reqwest::Client::new();
    let pg_auth = PostgresAuthStore::connect_without_migrations(&db.url)
        .await
        .expect("connect pg auth");
    let runtime = Store::connect(&db.url).await.expect("connect runtime");

    let created = client
        .put(format!(
            "{base}/auth/staff/practices/practice-pacific-night"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "clock": { "mode": "fixed", "unix_seconds": PACIFIC_LATE_EVENING_UNIX },
            "time_zone": "America/Los_Angeles",
        }))
        .send()
        .await
        .expect("create pacific practice");
    assert_eq!(created.status(), reqwest::StatusCode::CREATED);

    let create_acct = client
        .post(format!(
            "{base}/auth/staff/practices/practice-pacific-night/accounts"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "physician_id": "dr-pacific",
            "email": "pacific@practice.example",
            "display_name": "Dr. Pacific",
            "role": "admin",
        }))
        .send()
        .await
        .expect("create pacific account");
    assert_eq!(create_acct.status(), reqwest::StatusCode::CREATED);

    // Exact attribute match returns 200 Existing.
    let exact_repeat = client
        .post(format!(
            "{base}/auth/staff/practices/practice-pacific-night/accounts"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "email": "pacific@practice.example",
            "display_name": "Dr. Pacific",
            "role": "admin",
        }))
        .send()
        .await
        .expect("exact repeat");
    assert_eq!(exact_repeat.status(), reqwest::StatusCode::OK);
    let exact_json: Value = exact_repeat.json().await.expect("exact json");
    assert_eq!(exact_json["outcome"], "existing");

    // Role mismatch returns 409 account_conflict.
    let role_mismatch = client
        .post(format!(
            "{base}/auth/staff/practices/practice-pacific-night/accounts"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "email": "pacific@practice.example",
            "display_name": "Dr. Pacific",
            "role": "physician",
        }))
        .send()
        .await
        .expect("role mismatch");
    assert_eq!(role_mismatch.status(), reqwest::StatusCode::CONFLICT);
    let role_json: Value = role_mismatch.json().await.expect("role mismatch json");
    assert_eq!(role_json["code"], "account_conflict");

    // Display-name mismatch returns 409 account_conflict.
    let name_mismatch = client
        .post(format!(
            "{base}/auth/staff/practices/practice-pacific-night/accounts"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "email": "pacific@practice.example",
            "display_name": "Dr. Renamed",
            "role": "admin",
        }))
        .send()
        .await
        .expect("name mismatch");
    assert_eq!(name_mismatch.status(), reqwest::StatusCode::CONFLICT);

    // Deactivated account cannot be silently treated as active by create_account (409).
    let deact = client
        .post(format!(
            "{base}/auth/staff/practices/practice-pacific-night/accounts/deactivate"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({ "email": "pacific@practice.example" }))
        .send()
        .await
        .expect("deactivate");
    assert_eq!(deact.status(), reqwest::StatusCode::OK);

    let create_while_deactivated = client
        .post(format!(
            "{base}/auth/staff/practices/practice-pacific-night/accounts"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "email": "pacific@practice.example",
            "display_name": "Dr. Pacific",
            "role": "admin",
        }))
        .send()
        .await
        .expect("create while deactivated");
    assert_eq!(
        create_while_deactivated.status(),
        reqwest::StatusCode::CONFLICT
    );

    let react = client
        .post(format!(
            "{base}/auth/staff/practices/practice-pacific-night/accounts/reactivate"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({ "email": "pacific@practice.example" }))
        .send()
        .await
        .expect("reactivate");
    assert_eq!(react.status(), reqwest::StatusCode::OK);

    // Seed patient born 1986-08-28 so UTC date (2026-08-28 -> age 40) vs Pacific date (2026-08-27 -> age 39) is observable.
    pg_auth
        .register_patient("practice-pacific-night", "pw-pacific-001")
        .await
        .expect("register pacific patient");
    let content = json!({
        "workspace": "pw-pacific-001",
        "patient": "Clara Reyes",
        "mrn": "MRN-PAC-1",
    })
    .to_string();
    let sha256 = local_product::sha256_hex(content.as_bytes());
    runtime
        .record_chart(
            "pw-pacific-001",
            &[SourceRevision {
                workspace: "pw-pacific-001".to_owned(),
                source_id: "elation_fhir".to_owned(),
                candidate_id: "pw-pacific-001-seed".to_owned(),
                revision: format!("rev-{}", &sha256[..12]),
                record_type: "source_record".to_owned(),
                dates: vec![RecordDate {
                    name: "recorded_at".to_owned(),
                    value: "2026-08-27T20:00:00Z".to_owned(),
                }],
                content,
                sha256,
                readable_parts: vec![ReadablePart {
                    label: "Chart".to_owned(),
                    text: "Clara Reyes primary care note.".to_owned(),
                }],
                provenance: json!({
                    "source_id": "elation_fhir",
                    "native_id": "pw-pacific-001-seed",
                }),
                envelope: None,
            }],
            &SourcePull {
                source_id: "elation_fhir".to_owned(),
                source_label: "Elation / FHIR".to_owned(),
                patient: SourcePatient {
                    source_patient_id: "pw-pacific-001".to_owned(),
                    display_name: "Clara Reyes".to_owned(),
                    birth_date: Some("1986-08-28".to_owned()),
                    sex_or_gender: Some("female".to_owned()),
                    medical_record_numbers: vec![MedicalRecordNumber {
                        system: "Clinic".to_owned(),
                        value: "MRN-PAC-1".to_owned(),
                    }],
                },
                pulled_at: Some(PACIFIC_LATE_EVENING_UNIX as i64),
            },
        )
        .await
        .expect("seed Clara Reyes");

    let token = sign_in_proof(&client, &base, "pacific@practice.example").await;
    let mut assigned = false;
    for _ in 0..40 {
        let resp = client
            .put(format!("{base}/auth/panels/dr-pacific/pw-pacific-001"))
            .bearer_auth(&token)
            .send()
            .await
            .expect("assign pacific patient");
        if resp.status() == reqwest::StatusCode::OK {
            assigned = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert!(assigned, "assigned pw-pacific-001 to dr-pacific");

    let day_json: Value = client
        .get(format!("{base}/practice-day"))
        .bearer_auth(&token)
        .send()
        .await
        .expect("practice-day")
        .json()
        .await
        .expect("practice-day json");
    assert_eq!(day_json["date"], "2026-08-27");
    assert_eq!(day_json["time_zone"], "America/Los_Angeles");

    let today_json: Value = client
        .get(format!("{base}/today"))
        .bearer_auth(&token)
        .send()
        .await
        .expect("today")
        .json()
        .await
        .expect("today json");
    assert_eq!(today_json["date"], "2026-08-27");
    assert_eq!(today_json["time_zone"], "America/Los_Angeles");

    let updates_json: Value = client
        .get(format!("{base}/updates"))
        .bearer_auth(&token)
        .send()
        .await
        .expect("updates")
        .json()
        .await
        .expect("updates json");
    assert_eq!(updates_json["date"], "2026-08-27");
    assert_eq!(updates_json["time_zone"], "America/Los_Angeles");

    let ws_json: Value = client
        .get(format!("{base}/patient-workspaces/pw-pacific-001"))
        .bearer_auth(&token)
        .send()
        .await
        .expect("workspace")
        .json()
        .await
        .expect("workspace json");
    assert_eq!(ws_json["time_zone"], "America/Los_Angeles");
    assert_eq!(ws_json["identity"]["age_as_of"], "2026-08-27");
    assert_eq!(ws_json["identity"]["age_years"], 39);

    srv.abort();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn source_setup_persists_across_postgres_restarts_and_gates_simulation_to_synthetic_practices()
 {
    let db = DisposableDb::create("source_restart_gate").await;
    let pg_auth = PostgresAuthStore::connect_without_migrations(&db.url)
        .await
        .expect("connect pg auth");
    let runtime = Store::connect(&db.url).await.expect("connect runtime");
    let client = reqwest::Client::new();

    // Instance 1: mount site over PostgreSQL.
    let db_url_1 = db.url.clone();
    let (router_1, _) = physician_sample::site_router_from_env(move |key| match key {
        "PHYSICIAN_AUTH" => Some("required".to_owned()),
        "PHYSICIAN_DATABASE_URL" => Some(db_url_1.clone()),
        "CODA_STAFF_SETUP_TOKEN" => Some(STAFF_SECRET.to_owned()),
        _ => None,
    })
    .await
    .expect("build router_1");
    let (addr_1, srv_1) = spawn_router(router_1).await;
    let base_1 = format!("http://{addr_1}");

    // Create one synthetic fixed-clock practice and one real-time practice.
    let synth_practice = client
        .put(format!(
            "{base_1}/auth/staff/practices/practice-synth-durable"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "clock": { "mode": "fixed", "unix_seconds": FIXED_CLOCK_UNIX },
            "time_zone": "America/New_York",
        }))
        .send()
        .await
        .expect("create synth practice");
    assert_eq!(synth_practice.status(), reqwest::StatusCode::CREATED);

    let live_practice = client
        .put(format!(
            "{base_1}/auth/staff/practices/practice-live-assisted"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "clock": { "mode": "real_time" },
            "time_zone": "America/Chicago",
        }))
        .send()
        .await
        .expect("create live practice");
    assert_eq!(live_practice.status(), reqwest::StatusCode::CREATED);

    for (practice_id, phys_id, email, name) in [
        (
            "practice-synth-durable",
            "admin-synth",
            "synth-admin@durable.example",
            "Dr. Synth Admin",
        ),
        (
            "practice-live-assisted",
            "admin-live",
            "live-admin@assisted.example",
            "Dr. Live Admin",
        ),
    ] {
        let acct = client
            .post(format!(
                "{base_1}/auth/staff/practices/{practice_id}/accounts"
            ))
            .bearer_auth(STAFF_SECRET)
            .json(&json!({
                "physician_id": phys_id,
                "email": email,
                "display_name": name,
                "role": "admin",
            }))
            .send()
            .await
            .expect("create admin");
        assert_eq!(acct.status(), reqwest::StatusCode::CREATED);
    }

    seed_practice_patient(
        &pg_auth,
        &runtime,
        "practice-synth-durable",
        "pw-durable-001",
        "Mary Louis",
        "MRN-DUR-1",
    )
    .await;
    seed_practice_patient(
        &pg_auth,
        &runtime,
        "practice-live-assisted",
        "pw-live-001",
        "Hannah Cole",
        "MRN-LIVE-1",
    )
    .await;

    let synth_token = sign_in_proof(&client, &base_1, "synth-admin@durable.example").await;
    let live_token = sign_in_proof(&client, &base_1, "live-admin@assisted.example").await;

    for (token, phys_id, ws_id) in [
        (&synth_token, "admin-synth", "pw-durable-001"),
        (&live_token, "admin-live", "pw-live-001"),
    ] {
        let mut ok = false;
        for _ in 0..40 {
            let resp = client
                .put(format!("{base_1}/auth/panels/{phys_id}/{ws_id}"))
                .bearer_auth(token)
                .send()
                .await
                .expect("assign patient");
            if resp.status() == reqwest::StatusCode::OK {
                ok = true;
                break;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        assert!(ok, "assigned {ws_id}");
    }

    // Real-time practice gates out SimulatedPractice and fails closed on unverified live connectors.
    let live_conns: Value = client
        .get(format!("{base_1}/connections"))
        .bearer_auth(&live_token)
        .send()
        .await
        .expect("live connections")
        .json()
        .await
        .expect("live connections json");
    for src in live_conns["sources"].as_array().expect("sources array") {
        assert_eq!(
            src["supported_methods"],
            json!(["coda_assisted"]),
            "real-time practice must offer only Coda-assisted onboarding until live verifiers land"
        );
    }

    let live_sim_endpoint = client
        .post(format!("{base_1}/connections/spruce/simulate"))
        .bearer_auth(&live_token)
        .json(&json!({ "event": "quiet_catch_up" }))
        .send()
        .await
        .expect("live simulate");
    assert_eq!(live_sim_endpoint.status(), reqwest::StatusCode::NOT_FOUND);

    let live_sim_method = client
        .post(format!("{base_1}/connections/spruce/connect"))
        .bearer_auth(&live_token)
        .json(&json!({
            "method": "simulated_practice",
            "account_label": "Fake Sim",
        }))
        .send()
        .await
        .expect("live simulated_practice connect");
    assert_eq!(
        live_sim_method.status(),
        reqwest::StatusCode::UNPROCESSABLE_ENTITY
    );

    let live_cred_method = client
        .post(format!("{base_1}/connections/spruce/connect"))
        .bearer_auth(&live_token)
        .json(&json!({
            "method": "credential_sign_in",
            "account_label": "Real Clinic Spruce",
            "secret_or_code": "unverified-token-value-1234",
        }))
        .send()
        .await
        .expect("live credential connect");
    assert_eq!(
        live_cred_method.status(),
        reqwest::StatusCode::SERVICE_UNAVAILABLE
    );
    let live_cred_body: Value = live_cred_method.json().await.expect("live cred body");
    assert_eq!(live_cred_body["code"], "live_connector_unavailable");

    // Requesting Coda-assisted setup succeeds on the real-time practice and persists.
    let live_assist = client
        .post(format!("{base_1}/connections/spruce/assist"))
        .bearer_auth(&live_token)
        .json(&json!({
            "contact_email": "ops@live.example",
            "notes": "Connect clinic Spruce org",
        }))
        .send()
        .await
        .expect("live assist");
    assert_eq!(live_assist.status(), reqwest::StatusCode::OK);

    // Synthetic practice exercises Connect -> Import -> Accept backlog -> AccessLost on Instance 1.
    let synth_connect = client
        .post(format!("{base_1}/connections/spruce/connect"))
        .bearer_auth(&synth_token)
        .json(&json!({
            "method": "credential_sign_in",
            "account_label": "Spruce Synthetic Org",
            "secret_or_code": "spruce-synth-secret-9999",
        }))
        .send()
        .await
        .expect("synth connect");
    assert_eq!(synth_connect.status(), reqwest::StatusCode::OK);

    let synth_import = client
        .post(format!("{base_1}/connections/spruce/import"))
        .bearer_auth(&synth_token)
        .send()
        .await
        .expect("synth import");
    assert_eq!(synth_import.status(), reqwest::StatusCode::OK);

    let synth_backlog = client
        .post(format!("{base_1}/connections/spruce/accept-backlog"))
        .bearer_auth(&synth_token)
        .send()
        .await
        .expect("synth backlog");
    assert_eq!(synth_backlog.status(), reqwest::StatusCode::OK);

    let synth_revoke = client
        .post(format!("{base_1}/connections/spruce/simulate"))
        .bearer_auth(&synth_token)
        .json(&json!({
            "event": "revoked_grant",
            "reason": "Spruce token revoked before server restart.",
        }))
        .send()
        .await
        .expect("synth revoke");
    assert_eq!(synth_revoke.status(), reqwest::StatusCode::OK);

    // Stop Instance 1 completely, then boot fresh Instance 2 against the same PostgreSQL database.
    srv_1.abort();

    let db_url_2 = db.url.clone();
    let (router_2, _) = physician_sample::site_router_from_env(move |key| match key {
        "PHYSICIAN_AUTH" => Some("required".to_owned()),
        "PHYSICIAN_DATABASE_URL" => Some(db_url_2.clone()),
        "CODA_STAFF_SETUP_TOKEN" => Some(STAFF_SECRET.to_owned()),
        _ => None,
    })
    .await
    .expect("build router_2 after restart");
    let (addr_2, srv_2) = spawn_router(router_2).await;
    let base_2 = format!("http://{addr_2}");

    // Verify synthetic practice source standing, backlog, /today recovery notice, and retained chart/drafts survived restart!
    let after_restart_conns: Value = client
        .get(format!("{base_2}/connections"))
        .bearer_auth(&synth_token)
        .send()
        .await
        .expect("connections after restart")
        .json()
        .await
        .expect("connections after restart json");
    let spruce_after = after_restart_conns["sources"]
        .as_array()
        .expect("sources")
        .iter()
        .find(|s| s["source_id"] == "spruce")
        .expect("spruce entry");
    assert_eq!(spruce_after["standing"]["state"], "access_lost");
    assert_eq!(spruce_after["standing"]["cause"], "revoked_grant");
    assert_eq!(spruce_after["backlog"]["state"], "accepted");
    assert_eq!(spruce_after["backlog"]["imported_message_count"], 3);
    assert_eq!(spruce_after["retained_linked_records"], 5);
    assert_eq!(spruce_after["retained_reply_drafts"], 1);

    let today_after: Value = client
        .get(format!("{base_2}/today"))
        .bearer_auth(&synth_token)
        .send()
        .await
        .expect("today after restart")
        .json()
        .await
        .expect("today after restart json");
    let today_conns = today_after["connections"].as_array().expect("connections");
    assert!(
        today_conns
            .iter()
            .any(|c| { c["source_id"] == "spruce" && c["recovery"] == "reconnect_required" }),
        "expected durable reconnect_required Today notice after server restart: {today_conns:?}"
    );

    let ws_after: Value = client
        .get(format!("{base_2}/patient-workspaces/pw-durable-001"))
        .bearer_auth(&synth_token)
        .send()
        .await
        .expect("workspace after restart")
        .json()
        .await
        .expect("workspace after restart json");
    let spruce_ws_src = ws_after["connected_sources"]
        .as_array()
        .expect("connected_sources")
        .iter()
        .find(|s| s["source_id"] == "spruce")
        .expect("spruce connected_source");
    assert_eq!(spruce_ws_src["state"]["state"], "failed");
    assert!(spruce_ws_src["state"]["usable_snapshot"]["synchronized_at"].is_string());
    let replies_after = ws_after["replies"].as_array().expect("replies array");
    assert!(
        replies_after
            .iter()
            .any(|r| r["reply_work_id"] == "spruce-retained-reply-draft"),
        "expected retained Spruce reply draft after restart: {replies_after:?}"
    );

    // Reconnect on Instance 2 transitions standing back to UpToDate and clears Today notice.
    let reconnect_2 = client
        .post(format!("{base_2}/connections/spruce/reconnect"))
        .bearer_auth(&synth_token)
        .json(&json!({
            "method": "credential_sign_in",
            "account_label": "Spruce Synthetic Org",
            "secret_or_code": "spruce-synth-rotated-9999",
        }))
        .send()
        .await
        .expect("reconnect on instance 2");
    assert_eq!(reconnect_2.status(), reqwest::StatusCode::OK);
    let reconnect_2_json: Value = reconnect_2.json().await.expect("reconnect json");
    assert_eq!(
        reconnect_2_json["source"]["standing"]["state"],
        "up_to_date"
    );

    // Verify real-time practice's Coda-assisted onboarding request also survived restart.
    let live_after: Value = client
        .get(format!("{base_2}/connections"))
        .bearer_auth(&live_token)
        .send()
        .await
        .expect("live connections after restart")
        .json()
        .await
        .expect("live connections after restart json");
    let live_spruce = live_after["sources"]
        .as_array()
        .expect("sources")
        .iter()
        .find(|s| s["source_id"] == "spruce")
        .expect("live spruce");
    assert_eq!(live_spruce["standing"]["state"], "coda_assisted_requested");
    assert_eq!(live_spruce["standing"]["contact_email"], "ops@live.example");

    srv_2.abort();
}
