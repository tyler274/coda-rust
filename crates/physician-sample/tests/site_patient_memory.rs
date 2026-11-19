//! Headless Chromium + Postgres qualification for `Memory.Code.20` and
//! `Memory.Code.22`:
//! - The physician sees only `summary.md` and `direction.md` on the patient page.
//! - Subject pages (`pages/*.md`), `person.md`, and the dated log (`log/*.md`)
//!   are Coda's working files and never appear in the HTTP API or browser DOM.
//! - A physician's edit of `Direction` is saved as a log note in their exact
//!   words (leaving `direction.md` untouched until patient rounds fold it in).
//! - Running patient rounds folds the note into `direction.md`, and reloading
//!   the browser shows the folded version.
//! - Auth boundaries fail closed (`401` unauthenticated, `404` non-panel).

use std::env;
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use base64::Engine as _;
use chrono::Utc;
use front_door::PostgresFrontDoorStore;
use harness::engine::recipe::Provider;
use harness::engine::{Invocation, Item, Turn};
use local_product::physician_sources::{
    MedicalRecordNumber, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
use local_product::runtime::memory::{
    Day, Edit, MemoryPath, NoteDraft, NoteOrigin, PageId, PatientKey, VersionedFile, Words,
};
use local_product::runtime::rounds;
use local_product::runtime::sources::SourcePull;
use local_product::runtime::{Scope, Store};
use physician_auth::{AuthStore, Membership, PostgresAuthStore};
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

use devtools::{Browser, DESKTOP};
use required_auth::shared_site;

const STAFF_SECRET: &str = "coda-staff-memory-secret-2026";
const FIXED_CLOCK_UNIX: u64 = 1_795_000_000;
const SEARCH_FIELD: &str = ".search-field input";
const FIRST_RESULT: &str = ".dialog-results button";

struct DisposableDb {
    admin_url: String,
    db_name: String,
    url: String,
}

impl DisposableDb {
    async fn create(prefix: &str) -> Self {
        let admin_url = env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
            .or_else(|_| env::var("PHYSICIAN_AUTH_TEST_DATABASE_URL"))
            .expect("set PHYSICIAN_RUNTIME_TEST_DATABASE_URL to run site_patient_memory");
        let db_name = format!(
            "coda_{prefix}_{}",
            &Uuid::new_v4().simple().to_string()[..12]
        );
        let admin = sea_orm::Database::connect(&admin_url)
            .await
            .expect("connect admin postgres");
        admin
            .execute_unprepared(&format!(r#"CREATE DATABASE "{db_name}""#))
            .await
            .expect("create disposable db");
        drop(admin);

        let url = replace_database_name(&admin_url, &db_name);
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

fn replace_database_name(url: &str, db_name: &str) -> String {
    let (base, query) = match url.split_once('?') {
        Some((b, q)) => (b, Some(q)),
        None => (url, None),
    };
    let prefix = base
        .rsplit_once('/')
        .map(|(p, _)| p)
        .expect("postgres url has path");
    match query {
        Some(q) => format!("{prefix}/{db_name}?{q}"),
        None => format!("{prefix}/{db_name}"),
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

async fn seed_patient(
    pg_auth: &PostgresAuthStore,
    runtime: &Store,
    practice_id: &str,
    workspace_id: &str,
    full_name: &str,
) {
    pg_auth
        .register_patient(practice_id, workspace_id)
        .await
        .expect("register patient");
    let content = json!({
        "workspace": workspace_id,
        "patient": full_name,
        "mrn": "MRN-MEMORY-1",
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
            value: "2026-11-19T12:00:00Z".to_owned(),
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
        envelope: None,
    };
    let pull = SourcePull {
        source_id: "elation_fhir".to_owned(),
        source_label: "Elation / FHIR".to_owned(),
        patient: SourcePatient {
            source_patient_id: workspace_id.to_owned(),
            display_name: full_name.to_owned(),
            birth_date: Some("1978-05-12".to_owned()),
            sex_or_gender: Some("female".to_owned()),
            medical_record_numbers: vec![MedicalRecordNumber {
                system: "Clinic".to_owned(),
                value: "MRN-MEMORY-1".to_owned(),
            }],
        },
        pulled_at: Some(FIXED_CLOCK_UNIX as i64),
    };
    runtime
        .record_chart_with_receipt(workspace_id, &[rev], &pull)
        .await
        .expect("seed chart");
}

struct ScriptedRoundsProvider {
    turns: std::collections::VecDeque<Turn>,
}

impl Provider for ScriptedRoundsProvider {
    async fn exchange(
        &mut self,
        _request: &harness::engine::recipe::Request,
        _cancellation: &harness::engine::Cancellation,
        _limits: &harness::engine::RunLimits,
        _observer: &mut dyn harness::engine::recipe::Observer,
    ) -> Result<Turn, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(self.turns.pop_front().expect("scripted rounds turn"))
    }
}

struct MemoryWalk {
    browser: Browser,
    proof_dir: PathBuf,
}

impl MemoryWalk {
    fn launch(proof_dir: PathBuf) -> Self {
        let mut browser = Browser::launch();
        browser.resize(DESKTOP);
        browser.page().call_session("Page.enable", json!({}));
        browser.page().call_session(
            "Emulation.setTimezoneOverride",
            json!({ "timezoneId": "America/New_York" }),
        );
        Self { browser, proof_dir }
    }

    fn run(&mut self, action: &str, args: &[Value]) -> Value {
        let mut arguments = vec![json!(action)];
        arguments.extend_from_slice(args);
        self.browser
            .page()
            .call_function(include_str!("js/patient_memory.js"), &arguments)
    }

    fn wait_until(&mut self, what: &str, action: &str, args: &[Value]) {
        let deadline = Instant::now() + Duration::from_secs(12);
        while self.run(action, args) != json!(true) {
            assert!(Instant::now() < deadline, "timed out waiting for {what}");
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
                "page never showed `{needle}`; last text:\n{text}"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    fn capture(&mut self, name: &str) -> PathBuf {
        let overflow = self.run("sidewaysScroll", &[]);
        assert!(
            overflow.as_f64().is_some_and(|by| by <= 0.0),
            "{name} scrolls sideways by {overflow}"
        );
        std::thread::sleep(Duration::from_millis(150));
        let shot = self
            .browser
            .page()
            .call_session("Page.captureScreenshot", json!({ "format": "png" }));
        let data = shot
            .pointer("/result/data")
            .and_then(Value::as_str)
            .expect("screenshot base64");
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(data)
            .expect("decode png");
        fs::create_dir_all(&self.proof_dir).expect("create proof dir");
        let path = self.proof_dir.join(format!("{name}.png"));
        fs::write(&path, bytes).expect("write png");
        path
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn physician_sees_only_summary_and_direction_and_edit_saves_note_folded_by_rounds() {
    let site_dir = shared_site();
    unsafe {
        env::set_var("PHYSICIAN_SITE_DIR", &site_dir);
    }
    let db = DisposableDb::create("patient_memory").await;
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
    .expect("build patient memory router");

    let (addr, srv) = spawn_router(router).await;
    let base = format!("http://{addr}");
    let client = reqwest::Client::new();
    let pg_auth = PostgresAuthStore::connect_without_migrations(&db.url)
        .await
        .expect("connect pg auth");
    let runtime = Store::connect(&db.url).await.expect("connect runtime");

    let practice_id = "practice-memory-e2";
    let workspace_id = "pw-elena-memory";
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
            "email": "dr.memory@codahealth.test",
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

    // Also invite a second physician NOT assigned to Elena's panel to prove 404.
    let other_invite: Value = client
        .post(format!(
            "{base}/auth/staff/practices/{practice_id}/invitations"
        ))
        .bearer_auth(STAFF_SECRET)
        .json(&json!({
            "email": "dr.other@codahealth.test",
            "display_name": "Dr. Other",
            "role": "physician",
        }))
        .send()
        .await
        .expect("invite other")
        .json()
        .await
        .expect("other json");
    let other_token = other_invite["token"].as_str().expect("other token");
    let other_verify: Value = client
        .post(format!("{base}/auth/verify"))
        .json(&json!({
            "email": "dr.other@codahealth.test",
            "token": other_token,
        }))
        .send()
        .await
        .expect("verify other")
        .json()
        .await
        .expect("other session");
    let other_bearer = other_verify["session"].as_str().expect("other bearer");

    seed_patient(
        &pg_auth,
        &runtime,
        practice_id,
        workspace_id,
        "Elena Rostova",
    )
    .await;

    let proof_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/verification/captures/pr-e2-patient-memory");
    let mut walk = MemoryWalk::launch(proof_dir);
    walk.browser.text_until(
        &format!("{base}{sign_in_path}"),
        "Nothing needs you right now.",
    );
    let session_token = walk
        .run("sessionToken", &[])
        .as_str()
        .expect("browser session token")
        .to_owned();
    let whoami: Value = client
        .get(format!("{base}/auth/whoami"))
        .bearer_auth(&session_token)
        .send()
        .await
        .expect("whoami")
        .json()
        .await
        .expect("whoami json");
    let physician_id = whoami["physician_id"].as_str().expect("physician id");

    // Put Elena on Dr. Maya West's panel.
    let identity = physician_auth::PhysicianIdentity {
        practice_id: practice_id.to_owned(),
        physician_id: physician_id.to_owned(),
    };
    pg_auth
        .set_membership(&identity, workspace_id, Membership::Allowed)
        .await
        .expect("set panel membership");

    // Seed summary, direction, person, subject page, and log note.
    let scope = Scope {
        practice: practice_id.to_owned(),
        physician: physician_id.to_owned(),
        workspace: workspace_id.to_owned(),
        chat: "seed-chat".to_owned(),
    };
    let pkey = PatientKey::try_from(&scope).unwrap();
    runtime
        .edit_memory(
            &pkey,
            &VersionedFile::Summary,
            &[Ok(Edit::Create {
                text: Words::parse("CKD stage 3b with stable potassium 4.2.").unwrap(),
            })],
        )
        .await
        .unwrap();
    runtime
        .edit_memory(
            &pkey,
            &VersionedFile::Direction,
            &[Ok(Edit::Create {
                text: Words::parse("Check BMP before every visit.").unwrap(),
            })],
        )
        .await
        .unwrap();
    runtime
        .edit_memory(
            &pkey,
            &VersionedFile::Person,
            &[Ok(Edit::Create {
                text: Words::parse("SECRET-PERSON-WORKING-FILE").unwrap(),
            })],
        )
        .await
        .unwrap();
    runtime
        .edit_memory(
            &pkey,
            &VersionedFile::Page(PageId::parse("renal").unwrap()),
            &[Ok(Edit::Create {
                text: Words::parse("SECRET-PAGE-WORKING-FILE").unwrap(),
            })],
        )
        .await
        .unwrap();
    runtime
        .append_note(
            &scope,
            &NoteOrigin {
                work: "seed-w1".into(),
                call: "seed-c1".into(),
                message: "seed-m1".into(),
            },
            &NoteDraft {
                text: "SECRET-LOG-WORKING-FILE".into(),
                quote: None,
            },
        )
        .await
        .unwrap();

    // Auth boundaries on GET /patient-workspaces/{id}/memory.
    let unauth = client
        .get(format!("{base}/patient-workspaces/{workspace_id}/memory"))
        .send()
        .await
        .expect("unauth GET");
    assert_eq!(unauth.status(), reqwest::StatusCode::UNAUTHORIZED);

    let non_panel = client
        .get(format!("{base}/patient-workspaces/{workspace_id}/memory"))
        .bearer_auth(other_bearer)
        .send()
        .await
        .expect("non-panel GET");
    assert_eq!(non_panel.status(), reqwest::StatusCode::NOT_FOUND);

    let authed_mem: Value = client
        .get(format!("{base}/patient-workspaces/{workspace_id}/memory"))
        .bearer_auth(&session_token)
        .send()
        .await
        .expect("authed GET")
        .json()
        .await
        .expect("memory json");
    let raw_api = authed_mem.to_string();
    assert!(raw_api.contains("CKD stage 3b with stable potassium 4.2."));
    assert!(raw_api.contains("Check BMP before every visit."));
    for secret in [
        "SECRET-PERSON-WORKING-FILE",
        "SECRET-PAGE-WORKING-FILE",
        "SECRET-LOG-WORKING-FILE",
    ] {
        assert!(
            !raw_api.contains(secret),
            "API leaked working file: {raw_api}"
        );
    }

    // Open Elena Rostova in headless Chromium.
    walk.wait_until("Find patient", "press", &[json!("Find patient")]);
    walk.wait_until("search input", "exists", &[json!(SEARCH_FIELD)]);
    walk.browser.type_into(SEARCH_FIELD, "Elena");
    walk.wait_until("first result", "exists", &[json!(FIRST_RESULT)]);
    walk.wait_until("click first result", "click", &[json!(FIRST_RESULT)]);
    let text_before = walk.wait_page_contains("CKD stage 3b with stable potassium 4.2.");
    assert!(text_before.contains("Check BMP before every visit."));
    for secret in [
        "SECRET-PERSON-WORKING-FILE",
        "SECRET-PAGE-WORKING-FILE",
        "SECRET-LOG-WORKING-FILE",
    ] {
        assert!(
            !text_before.contains(secret),
            "DOM leaked Coda working file `{secret}`:\n{text_before}"
        );
    }
    walk.run("scrollTo", &[json!("[data-patient-memory='true']")]);
    walk.capture("01-patient-memory-summary-and-direction");

    // Physician edits Direction inline -> saved as note N2, direction.md stays v1.
    walk.wait_until(
        "Edit direction button",
        "click",
        &[json!("[data-memory-edit='direction']")],
    );
    walk.wait_until(
        "direction textarea",
        "fillTextarea",
        &[
            json!("[data-memory-input='direction']"),
            json!("Hold lisinopril until potassium is rechecked next week."),
        ],
    );
    walk.wait_until(
        "Save note button",
        "click",
        &[json!("[data-memory-save='direction']")],
    );
    let text_after_save = walk.wait_page_contains("Saved as note N2 in your words.");
    assert!(text_after_save.contains("Patient rounds will fold it into Direction."));
    walk.run("scrollTo", &[json!("[data-patient-memory='true']")]);
    walk.capture("02-patient-memory-edit-saved-as-note");

    // Confirm direction.md is STILL version 1 in Postgres while N2 holds physician's words.
    let mem_before_rounds = runtime.read_physician_memory(&pkey).await.unwrap();
    assert_eq!(mem_before_rounds.direction.as_ref().unwrap().version, 1);
    let log_path = MemoryPath::Log(Day::of(Utc::now())).to_string();
    let log_read = runtime
        .read_memory(&pkey, &MemoryPath::parse(&log_path).unwrap(), None)
        .await
        .unwrap();
    let log_md = match log_read {
        local_product::runtime::memory::FileRead::Log { markdown } => markdown,
        _ => panic!("expected log file"),
    };
    assert!(
        log_md.contains("Hold lisinopril until potassium is rechecked next week."),
        "log missing physician words:\n{log_md}"
    );

    // Patient rounds folds note N2 into direction.md.
    let mut provider = ScriptedRoundsProvider {
        turns: std::collections::VecDeque::from(vec![
            Turn {
                items: vec![Item::Call(Invocation {
                    id: "r1".into(),
                    name: "read_patient_memory".into(),
                    arguments: json!({"path": log_path, "version": null}).to_string(),
                })],
                receipt: json!({}),
                usage: Default::default(),
            },
            Turn {
                items: vec![Item::Call(Invocation {
                    id: "e1".into(),
                    name: "edit_patient_memory".into(),
                    arguments: json!({
                        "path": "direction.md",
                        "edits": [{"append": {"text": "Hold lisinopril until potassium is rechecked next week [N2]."}}]
                    })
                    .to_string(),
                })],
                receipt: json!({}),
                usage: Default::default(),
            },
            Turn {
                items: vec![Item::Message {
                    role: "assistant".into(),
                    text: "Folded physician direction note N2 into direction.md.".into(),
                }],
                receipt: json!({}),
                usage: Default::default(),
            },
        ]),
    };
    let binding = local_product::ModelBinding {
        model: "scripted".into(),
        effort: "medium".into(),
        context_window_tokens: 128_000,
        max_input_tokens: 100_000,
        adapter: "scripted".into(),
    };
    let skills = local_product::physician_chat::PhysicianSkills::bundled_fallback().unwrap();
    let run = rounds::run(
        &runtime,
        &mut provider,
        &pkey,
        Utc::now() + chrono::Duration::seconds(2),
        &skills,
        &binding,
    )
    .await;
    assert!(run.finished(), "rounds failed: {:?}", run.ending);

    // Reload patient workspace in Chromium and confirm folded Direction v2 renders!
    walk.browser.text_until(
        &format!("{base}/?patient={workspace_id}"),
        "Hold lisinopril until potassium is rechecked next week [N2].",
    );
    walk.run("scrollTo", &[json!("[data-patient-memory='true']")]);
    walk.capture("03-patient-memory-after-rounds-folded");

    // Seed a second panel patient (Marcus Vance) to verify SPA Patient A -> Patient B
    // switching loads Patient B's memory and never carries Patient A's open draft PHI.
    let marcus_workspace = "pw-marcus-memory";
    seed_patient(
        &pg_auth,
        &runtime,
        practice_id,
        marcus_workspace,
        "Marcus Vance",
    )
    .await;
    pg_auth
        .set_membership(&identity, marcus_workspace, Membership::Allowed)
        .await
        .expect("set Marcus panel membership");
    let marcus_scope = Scope {
        practice: practice_id.to_owned(),
        physician: physician_id.to_owned(),
        workspace: marcus_workspace.to_owned(),
        chat: "seed-marcus".to_owned(),
    };
    let marcus_key = PatientKey::try_from(&marcus_scope).unwrap();
    runtime
        .edit_memory(
            &marcus_key,
            &VersionedFile::Summary,
            &[Ok(Edit::Create {
                text: Words::parse("Hypertension controlled on amlodipine 5 mg.").unwrap(),
            })],
        )
        .await
        .unwrap();
    runtime
        .edit_memory(
            &marcus_key,
            &VersionedFile::Direction,
            &[Ok(Edit::Create {
                text: Words::parse("Review home BP log every 3 months.").unwrap(),
            })],
        )
        .await
        .unwrap();

    // Open Elena's summary editor and enter unsaved PHI without saving, then
    // navigate in-SPA to Marcus Vance via Find patient.
    walk.wait_until(
        "Edit Elena summary button",
        "click",
        &[json!("[data-memory-edit='summary']")],
    );
    walk.wait_until(
        "Elena unsaved summary draft",
        "fillTextarea",
        &[
            json!("[data-memory-input='summary']"),
            json!("ELENA-UNSAVED-PHI-DRAFT"),
        ],
    );
    walk.wait_until(
        "Find patient for SPA switch",
        "press",
        &[json!("Find patient")],
    );
    walk.wait_until("search input for Marcus", "exists", &[json!(SEARCH_FIELD)]);
    walk.browser.type_into(SEARCH_FIELD, "Marcus");
    walk.wait_until("first Marcus result", "exists", &[json!(FIRST_RESULT)]);
    walk.wait_until("click Marcus result", "click", &[json!(FIRST_RESULT)]);

    let marcus_text = walk.wait_page_contains("Hypertension controlled on amlodipine 5 mg.");
    assert!(marcus_text.contains("Review home BP log every 3 months."));
    assert!(
        !marcus_text.contains("ELENA-UNSAVED-PHI-DRAFT"),
        "SPA switch leaked Elena's unsaved draft onto Marcus's page:\n{marcus_text}"
    );
    assert!(
        !marcus_text.contains("CKD stage 3b with stable potassium 4.2."),
        "SPA switch leaked Elena's summary onto Marcus's page:\n{marcus_text}"
    );
    assert_eq!(
        walk.run("exists", &[json!("[data-memory-editor]")]),
        json!(false),
        "editor from Elena must not stay open on Marcus's page"
    );
    walk.run("scrollTo", &[json!("[data-patient-memory='true']")]);
    walk.capture("04-patient-memory-spa-switch-marcus");

    srv.abort();
}
