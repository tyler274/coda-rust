//! Browser qualification for the Today schedule pane and pre-visit briefing view.
//!
//! Exercises the compiled `physician-app` WebAssembly bundle in headless Chromium
//! against a live `physician-sample` server backed by PostgreSQL:
//! - Absent calendar hides the schedule pane completely.
//! - Preparing, ready, unavailable, failed, and cancelled visit states render on
//!   Today while ended visits drop off; times follow the browser's time zone.
//! - Ready visits open their briefing, inline citation chips and source rows open
//!   the citation reader without leaving the briefing, late or missing sources
//!   show prominently, and Basis and limitations plus Sources open in place.
//! - Narrow phone screens (`390x844`) show only the next visit with zero
//!   horizontal overflow; desktop (`1280x840` / `1280x800`) shows the day's list.
//! - Failed schedule refreshes retain the last good visits alongside `"as of"`
//!   and `Try again`; cancelled and moved visits keep their briefings in history
//!   with explicit notices rather than looking prepared for obsolete slots.

#[allow(dead_code)]
mod common;
#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;
#[path = "common/disposable.rs"]
mod disposable;
#[allow(dead_code)]
mod required_auth;

use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use base64::Engine as _;
use devtools::{Browser, DESKTOP, PHONE};
use local_product::physician_chat::PhysicianSkills;
use local_product::physician_sources::{READ_TOOL, workspace_id};
use local_product::runtime::briefings::VisitPreparationState;
use local_product::runtime::chat_operations::{RecordLimit, RecordLimitKind};
use local_product::runtime::concerns::ExactRecordReference;
use local_product::runtime::jobs::PracticeScope;
use local_product::runtime::refresh::{
    FixedSourceCheck, RefreshCommandKey, RefreshRequest, RefreshTarget, SourceCheckOutcome,
    SourceId as RefreshSourceId,
};
use local_product::runtime::{Store, engine};
use local_product::visit_preparation::{
    BriefingClaimInput, BriefingDecisionInput, SAVE_BRIEFING_TOOL, SELECT_VISITS_TOOL,
    SaveBriefingArgs,
};
use local_product::{ModelBinding, ModelRoles};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use physician_sample::STAGING_PRACTICE_ID;
use physician_sample::hosting::staging_physician_id;
use physician_sample::schedule::SOURCE_ID as CALENDAR_SOURCE_ID;
use required_auth::shared_site;
use sea_orm::Database;
use serde_json::{Value, json};
use tokio::sync::oneshot;

const PHYSICIAN_EMAIL: &str = "physician@codahealth.test";
/// Synthetic staging practice fixed clock (`2026-08-27T16:00:00Z`).
const PRACTICE_NOW_UNIX: i64 = 1_787_846_400;
/// Pre-clinic preparation wake before `14:00Z` (`2026-08-27T13:30:00Z`).
const EARLY_WAKE_UNIX: i64 = 1_787_837_400;

struct TempPgServer {
    admin_url: String,
    data_dir: Option<PathBuf>,
}

impl TempPgServer {
    async fn start() -> Self {
        for key in [
            "PHYSICIAN_RUNTIME_TEST_DATABASE_URL",
            "PHYSICIAN_TEST_DATABASE_URL",
            "PHYSICIAN_DATABASE_URL",
        ] {
            if let Ok(url) = std::env::var(key) {
                let trimmed = url.trim();
                if !trimmed.is_empty() {
                    let (server, _) = trimmed
                        .rsplit_once('/')
                        .expect("database URL must include database name");
                    return Self {
                        admin_url: format!("{server}/postgres"),
                        data_dir: None,
                    };
                }
            }
        }

        let port = free_local_port();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let data_dir = std::env::temp_dir().join(format!(
            "coda-pg-sched-briefing-{}-{nanos:x}",
            std::process::id()
        ));
        fs::create_dir_all(&data_dir).expect("create temp pg dir");

        let init = Command::new("initdb")
            .args([
                "-U",
                "postgres",
                "-A",
                "trust",
                "--no-locale",
                "-E",
                "UTF8",
                "-D",
            ])
            .arg(&data_dir)
            .output()
            .expect("run initdb");
        assert!(
            init.status.success(),
            "initdb failed: {}",
            String::from_utf8_lossy(&init.stderr)
        );

        let log_file = data_dir.join("postgres.log");
        let pg_opts = format!("-p {port} -h 127.0.0.1 -k {}", data_dir.display());
        let start = Command::new("pg_ctl")
            .arg("-D")
            .arg(&data_dir)
            .arg("-l")
            .arg(&log_file)
            .arg("-o")
            .arg(&pg_opts)
            .arg("-w")
            .arg("start")
            .output()
            .expect("run pg_ctl start");
        assert!(
            start.status.success(),
            "pg_ctl start failed: {}",
            String::from_utf8_lossy(&start.stderr)
        );

        Self {
            admin_url: format!("postgresql://postgres@127.0.0.1:{port}/postgres"),
            data_dir: Some(data_dir),
        }
    }

    async fn create_database(&self, tag: &str) -> String {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let db_name = format!("sched_briefing_{tag}_{}_{nanos:x}", std::process::id());
        let admin = Database::connect(&self.admin_url)
            .await
            .expect("connect admin postgres");
        disposable::replace_database(&admin, &db_name).await;
        admin.close().await.expect("close admin postgres");
        let (server, _) = self
            .admin_url
            .rsplit_once('/')
            .expect("admin URL includes database");
        format!("{server}/{db_name}")
    }
}

impl Drop for TempPgServer {
    fn drop(&mut self) {
        if let Some(dir) = &self.data_dir {
            let _ = Command::new("pg_ctl")
                .arg("-D")
                .arg(dir)
                .args(["-m", "fast", "-w", "stop"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
            let _ = fs::remove_dir_all(dir);
        }
    }
}

struct RunningSite(Child);

impl Drop for RunningSite {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn free_local_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .expect("bind ephemeral port")
        .local_addr()
        .expect("read ephemeral port")
        .port()
}

fn run_migrate_schema(db_url: &str, staging_accounts: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_physician-migrate"))
        .env("PHYSICIAN_DATABASE_URL", db_url)
        .env("CODA_HOSTED_PROFILE", "staging-preview")
        .env("PHYSICIAN_STAGING_ACCOUNTS", staging_accounts)
        .env_remove("PHYSICIAN_SIMULATION_PRACTICES")
        .output()
        .expect("run physician-migrate")
}

fn run_load_charts(db_url: &str, bank: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_physician-migrate"))
        .arg("load-charts")
        .env("PHYSICIAN_DATABASE_URL", db_url)
        .env("CODA_HOSTED_PROFILE", "staging-preview")
        .env("PHYSICIAN_SYNTHETIC_CHARTS", bank)
        .output()
        .expect("run physician-migrate load-charts")
}

async fn spawn_site_with_web(
    db_url: &str,
    site_dir: &Path,
    synthetic_calendar: Option<&Path>,
) -> (RunningSite, String) {
    let address = TcpListener::bind("127.0.0.1:0")
        .expect("bind site port")
        .local_addr()
        .expect("site local addr");
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_physician-sample"));
    cmd.env("PHYSICIAN_DATABASE_URL", db_url)
        .env("PHYSICIAN_SITE_DIR", site_dir)
        .env("PHYSICIAN_AUTH", "required")
        .env("PHYSICIAN_AUTH_PROOF_TOKENS", "1")
        .env("PHYSICIAN_LISTEN_ADDR", address.to_string())
        .env_remove("PHYSICIAN_SYNTHETIC_CHARTS")
        .env_remove("PHYSICIAN_SYNTHETIC_CALENDAR")
        .env_remove("CODA_LOCAL_MODEL_EXECUTION")
        .env_remove("PHYSICIAN_CHAT_TOPIC")
        .env_remove("PHYSICIAN_UPSTREAM")
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if let Some(calendar) = synthetic_calendar {
        cmd.env("PHYSICIAN_SYNTHETIC_CALENDAR", calendar);
    }
    let mut site = RunningSite(cmd.spawn().expect("spawn physician-sample"));
    let base = format!("http://{address}");
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        assert!(
            site.0.try_wait().expect("poll site").is_none(),
            "physician-sample exited unexpectedly"
        );
        let health = reqwest::get(format!("{base}/health")).await;
        if health.is_ok_and(|resp| resp.status().is_success()) {
            break;
        }
        assert!(Instant::now() < deadline, "physician-sample never answered");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    (site, base)
}

async fn sign_in_http(client: &reqwest::Client, base: &str, email: &str) -> String {
    let begun: Value = client
        .post(format!("{base}/auth/sign-in"))
        .json(&json!({ "email": email }))
        .send()
        .await
        .expect("POST /auth/sign-in")
        .json()
        .await
        .expect("decode /auth/sign-in");
    let verified: Value = client
        .post(format!("{base}/auth/verify"))
        .json(&json!({ "email": email, "token": begun["token"] }))
        .send()
        .await
        .expect("POST /auth/verify")
        .json()
        .await
        .expect("decode /auth/verify");
    verified["session"]
        .as_str()
        .expect("session token")
        .to_owned()
}

fn fhir_doc_ref(id: &str, description: &str, date: &str, body_text: &str) -> String {
    let encoded = base64::engine::general_purpose::STANDARD.encode(body_text.as_bytes());
    json!({
        "resourceType": "DocumentReference",
        "id": id,
        "description": description,
        "date": date,
        "content": [{
            "attachment": {
                "contentType": "text/plain; charset=utf-8",
                "title": description,
                "data": encoded,
            }
        }]
    })
    .to_string()
}

fn fhir_appointment(
    id: &str,
    status: &str,
    description: &str,
    start: &str,
    end: &str,
    patient_mrn: Option<&str>,
) -> String {
    let participant = match patient_mrn {
        Some(mrn) => json!([{
            "actor": { "reference": format!("Patient/{mrn}") },
            "status": "accepted",
        }]),
        None => json!([]),
    };
    json!({
        "resourceType": "Appointment",
        "id": id,
        "meta": {
            "versionId": "1",
            "lastUpdated": "2026-08-27T00:00:00+00:00"
        },
        "status": status,
        "serviceType": [{ "text": "Follow-up" }],
        "appointmentType": { "text": "Follow-up visit" },
        "description": description,
        "start": start,
        "end": end,
        "minutesDuration": 30,
        "participant": participant,
    })
    .to_string()
}

fn load_skills() -> PhysicianSkills {
    PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
        .expect("load physician skills")
}

fn model_roles() -> ModelRoles {
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

struct Wake1Provider {
    selection_call: Value,
    ended_read: Value,
    ended_save: Value,
    alice_read: Value,
    alice_save: Value,
    bob_read: Value,
    bob_save: Value,
}

impl engine::recipe::Provider for Wake1Provider {
    async fn exchange(
        &mut self,
        request: &engine::recipe::Request,
        _: &engine::Cancellation,
        _: &engine::RunLimits,
        _: &mut dyn engine::recipe::Observer,
    ) -> Result<engine::Turn, engine::Cause> {
        let prompt = match request.items.first() {
            Some(engine::Item::Message { text, .. }) => text.as_str(),
            other => panic!("expected starting user message, got {other:?}"),
        };
        let serialized = serde_json::to_string(&request.items).expect("serialize request items");
        assert!(
            !serialized.contains("\"refused\":true") && !serialized.contains("Refused("),
            "unexpected tool refusal in wake1 run:\n{serialized}"
        );

        let call_count = request
            .items
            .iter()
            .filter(|item| matches!(item, engine::Item::Call(_)))
            .count();

        if prompt.contains(SELECT_VISITS_TOOL) {
            return match call_count {
                0 => Ok(
                    call_turn("sel-wake1", SELECT_VISITS_TOOL, self.selection_call.clone())
                        .expect("selection call"),
                ),
                1 => {
                    Ok(done_turn("Recorded visit preparation decisions.").expect("selection done"))
                }
                n => panic!("unexpected selection call_count {n}:\n{serialized}"),
            };
        }
        if prompt.contains("`appt-ended`") {
            return match call_count {
                0 => Ok(call_turn("read-ended", READ_TOOL, self.ended_read.clone()).unwrap()),
                1 => Ok(
                    call_turn("save-ended", SAVE_BRIEFING_TOOL, self.ended_save.clone()).unwrap(),
                ),
                2 => Ok(done_turn("Saved morning lab briefing.").unwrap()),
                n => panic!("unexpected appt-ended call_count {n}:\n{serialized}"),
            };
        }
        if prompt.contains("`appt-alice-ready`") {
            return match call_count {
                0 => Ok(call_turn("read-alice", READ_TOOL, self.alice_read.clone()).unwrap()),
                1 => Ok(
                    call_turn("save-alice", SAVE_BRIEFING_TOOL, self.alice_save.clone()).unwrap(),
                ),
                2 => Ok(done_turn("Saved Alice HFrEF briefing.").unwrap()),
                n => panic!("unexpected appt-alice-ready call_count {n}:\n{serialized}"),
            };
        }
        if prompt.contains("`appt-bob-move`") {
            return match call_count {
                0 => Ok(call_turn("read-bob", READ_TOOL, self.bob_read.clone()).unwrap()),
                1 => Ok(call_turn("save-bob", SAVE_BRIEFING_TOOL, self.bob_save.clone()).unwrap()),
                2 => Ok(done_turn("Saved Bob anticoagulation briefing.").unwrap()),
                n => panic!("unexpected appt-bob-move call_count {n}:\n{serialized}"),
            };
        }
        if prompt.contains("`appt-clara-fail`") {
            return Err("simulated model stream interruption on Clara preparation".into());
        }
        panic!("unrecognized wake1 job prompt:\n{prompt}");
    }
}

struct PendingBriefingProvider {
    started: Option<oneshot::Sender<()>>,
}

impl engine::recipe::Provider for PendingBriefingProvider {
    async fn exchange(
        &mut self,
        _: &engine::recipe::Request,
        _: &engine::Cancellation,
        _: &engine::RunLimits,
        _: &mut dyn engine::recipe::Observer,
    ) -> Result<engine::Turn, engine::Cause> {
        if let Some(tx) = self.started.take() {
            let _ = tx.send(());
        }
        std::future::pending().await
    }
}

fn call_turn(id: &str, name: &str, args: Value) -> Result<engine::Turn, String> {
    Ok(engine::Turn {
        items: vec![engine::Item::Call(engine::Invocation {
            id: id.to_owned(),
            name: name.to_owned(),
            arguments: args.to_string(),
        })],
        receipt: json!({ "call_id": id }),
        usage: Default::default(),
    })
}

fn done_turn(message: &str) -> Result<engine::Turn, String> {
    Ok(engine::Turn {
        items: vec![engine::Item::assistant(message)],
        receipt: json!({ "done": true }),
        usage: Default::default(),
    })
}

struct BrowserHarness {
    browser: Browser,
    shots_dir: PathBuf,
}

impl BrowserHarness {
    fn launch(size: (u32, u32), timezone_id: &str) -> Self {
        let mut browser = Browser::launch();
        browser.watch_page_errors();
        browser.resize(size);
        browser.page().call_session("Page.enable", json!({}));
        browser.page().call_session(
            "Emulation.setTimezoneOverride",
            json!({ "timezoneId": timezone_id }),
        );
        let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let shots_dir = workspace_root.join("target/verification/screenshots");
        fs::create_dir_all(&shots_dir).expect("create target/verification/screenshots");
        Self { browser, shots_dir }
    }

    fn set_timezone(&mut self, timezone_id: &str) {
        self.browser.page().call_session(
            "Emulation.setTimezoneOverride",
            json!({ "timezoneId": timezone_id }),
        );
    }

    fn resize(&mut self, size: (u32, u32)) {
        self.browser.resize(size);
    }

    fn run_walk(&mut self, helper: &str, args: &[Value]) -> Value {
        let mut arguments = vec![json!(helper)];
        arguments.extend_from_slice(args);
        self.browser
            .page()
            .call_function(include_str!("js/shell_walk.js"), &arguments)
    }

    fn exists(&mut self, selector: &str) -> bool {
        self.run_walk("exists", &[json!(selector)]) == json!(true)
    }

    fn text(&mut self, selector: &str) -> String {
        self.run_walk("text", &[json!(selector)])
            .as_str()
            .unwrap_or_default()
            .to_owned()
    }

    fn reveal(&mut self, selector: &str) {
        self.run_walk("reveal", &[json!(selector)]);
    }

    fn wait_exists(&mut self, selector: &str) {
        let deadline = Instant::now() + Duration::from_secs(15);
        while !self.exists(selector) {
            assert!(
                Instant::now() < deadline,
                "selector `{selector}` never appeared:\n{}",
                self.browser.body_text().unwrap_or_default()
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    fn wait_absent(&mut self, selector: &str) {
        let deadline = Instant::now() + Duration::from_secs(15);
        while self.exists(selector) {
            assert!(
                Instant::now() < deadline,
                "selector `{selector}` remained in DOM:\n{}",
                self.browser.body_text().unwrap_or_default()
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    fn press(&mut self, words: &str) {
        let deadline = Instant::now() + Duration::from_secs(15);
        while self.run_walk("press", &[json!(words)]) != json!(true) {
            assert!(
                Instant::now() < deadline,
                "button `{words}` never became pressable:\n{}",
                self.browser.body_text().unwrap_or_default()
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    fn press_first(&mut self, selector: &str) {
        let deadline = Instant::now() + Duration::from_secs(15);
        while self.run_walk("pressFirst", &[json!(selector)]) != json!(true) {
            assert!(
                Instant::now() < deadline,
                "selector `{selector}` never became pressable:\n{}",
                self.browser.body_text().unwrap_or_default()
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    fn sign_in(&mut self, base: &str, email: &str) {
        self.browser.text_until(base, "Sign in to Coda");
        self.browser.type_into(r#"input[type="email"]"#, email);
        self.press("Send sign-in link");
        self.browser.wait_for("Collapse Coda");
        for pre_auth in self.browser.take_page_errors() {
            assert!(
                pre_auth.contains("401 (Unauthorized)"),
                "unexpected pre-sign-in error: {pre_auth}"
            );
        }
    }

    fn assert_no_sideways_scroll(&mut self, context: &str) {
        let overflow = self.run_walk("sidewaysScroll", &[]);
        assert!(
            overflow.as_f64().is_some_and(|by| by <= 0.0),
            "{context} scrolled sideways by {overflow}"
        );
    }

    fn capture_screenshot(&mut self, filename: &str) -> PathBuf {
        self.assert_no_sideways_scroll(filename);
        std::thread::sleep(Duration::from_millis(350));
        let shot = self
            .browser
            .page()
            .call_session("Page.captureScreenshot", json!({ "format": "png" }));
        let encoded = shot
            .pointer("/result/data")
            .and_then(Value::as_str)
            .expect("Page.captureScreenshot base64 data");
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .expect("decode PNG screenshot");
        assert!(
            bytes.starts_with(b"\x89PNG\r\n\x1a\n"),
            "captured screenshot `{filename}` is not a valid PNG"
        );
        let path = self.shots_dir.join(filename);
        fs::write(&path, bytes).expect("write PNG screenshot");
        path
    }

    fn assert_no_page_errors(&mut self) {
        let errors = self.browser.take_page_errors();
        assert!(
            errors.is_empty(),
            "unexpected browser console/runtime errors: {errors:#?}"
        );
    }
}

fn build_synthetic_bank() -> (PathBuf, PathBuf) {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let bank = std::env::temp_dir().join(format!(
        "coda-site-sched-briefing-{}-{nanos:x}",
        std::process::id()
    ));
    let calendar_dir = bank.join("calendar");
    fs::create_dir_all(&calendar_dir).expect("create calendar dir");

    let patients: [(&str, &str, &str, Vec<String>); 4] = [
        (
            "MRN-ALICE",
            "Alice",
            "Carter",
            vec![
                fhir_doc_ref(
                    "note-hf-discharge",
                    "Cardiology Hospital Discharge Summary",
                    "2026-08-24T15:00:00+00:00",
                    "Acute decompensated HFrEF (LVEF 30%) discharged 2026-08-24 after IV diuresis; loop diuretic switched from bumetanide to oral torsemide 20 mg daily. Check standing weight, orthostatic vitals, and basic metabolic panel within 72 hours.",
                ),
                fhir_doc_ref(
                    "note-hf-labs",
                    "Discharge Chemistry & Natriuretic Peptide",
                    "2026-08-24T11:30:00+00:00",
                    "Discharge serum creatinine 1.42 mg/dL (baseline 1.18 mg/dL), potassium 3.9 mEq/L, NT-proBNP 2,840 pg/mL.",
                ),
            ],
        ),
        (
            "MRN-BOB",
            "Bob",
            "Diaz",
            vec![fhir_doc_ref(
                "note-anticoag-1",
                "Anticoagulation Clinic Progress Note",
                "2026-08-26T14:20:00+00:00",
                "Atrial fibrillation on warfarin 5 mg daily; outpatient INR rose to 4.3 without overt bleeding after recent dietary change.",
            )],
        ),
        (
            "MRN-CLARA",
            "Clara",
            "Nguyen",
            vec![fhir_doc_ref(
                "note-copd-1",
                "Urgent Care Pulmonary Note",
                "2026-08-25T18:00:00+00:00",
                "Moderate COPD exacerbation treated with albuterol-ipratropium nebulizers; outpatient follow-up requested.",
            )],
        ),
        (
            "MRN-DAVID",
            "David",
            "Patel",
            vec![fhir_doc_ref(
                "note-ckd-1",
                "Nephrology Co-Management Note",
                "2026-08-20T10:00:00+00:00",
                "Stage 3b chronic kidney disease on dapagliflozin 10 mg daily; stable bicarbonate and blood pressure.",
            )],
        ),
    ];

    for (mrn, given, family, docs) in patients {
        let chart_dir = bank.join("patients").join(mrn);
        fs::create_dir_all(&chart_dir).expect("create patient chart dir");
        let patient_line = json!({
            "resourceType": "Patient",
            "id": mrn,
            "name": [{ "family": family, "given": [given] }],
            "gender": "female",
            "birthDate": "1968-04-12",
        })
        .to_string();
        fs::write(
            chart_dir.join("Patient.ndjson"),
            format!("{patient_line}\n"),
        )
        .expect("write Patient.ndjson");
        let joined_docs = format!("{}\n", docs.join("\n"));
        fs::write(chart_dir.join("DocumentReference.ndjson"), joined_docs)
            .expect("write DocumentReference.ndjson");
    }

    let calendar_file = calendar_dir.join("Appointment.ndjson");
    let initial_appointments = [
        // Ended before practice clock `16:00Z` (`10:00 AM–10:30 AM EDT`) -> must drop off Today rail!
        fhir_appointment(
            "appt-ended",
            "booked",
            "Morning lab review",
            "2026-08-27T14:00:00+00:00",
            "2026-08-27T14:30:00+00:00",
            Some("MRN-ALICE"),
        ),
        // Next upcoming visit (`17:00Z` = `1:00 PM EDT` / `10:00 AM PDT`) -> Ready!
        fhir_appointment(
            "appt-alice-ready",
            "booked",
            "Post-discharge heart failure follow-up",
            "2026-08-27T17:00:00+00:00",
            "2026-08-27T17:30:00+00:00",
            Some("MRN-ALICE"),
        ),
        // Second ready visit (`18:00Z` = `2:00 PM EDT` / `11:00 AM PDT`) -> Ready, later moved!
        fhir_appointment(
            "appt-bob-move",
            "booked",
            "Anticoagulation dose review",
            "2026-08-27T18:00:00+00:00",
            "2026-08-27T18:30:00+00:00",
            Some("MRN-BOB"),
        ),
        // Unmatched patient (`18:30Z` = `2:30 PM EDT`) -> Preparation unavailable!
        fhir_appointment(
            "appt-unmatched",
            "booked",
            "Walk-in transfer consult",
            "2026-08-27T18:30:00+00:00",
            "2026-08-27T19:00:00+00:00",
            Some("MRN-UNREGISTERED"),
        ),
        // Exhausted job (`19:00Z` = `3:00 PM EDT`) -> Preparation failed!
        fhir_appointment(
            "appt-clara-fail",
            "booked",
            "Pulmonary exacerbation check",
            "2026-08-27T19:00:00+00:00",
            "2026-08-27T19:30:00+00:00",
            Some("MRN-CLARA"),
        ),
        // Active in-flight job (`20:00Z` = `4:00 PM EDT`) -> Preparing!
        fhir_appointment(
            "appt-david-prep",
            "booked",
            "Renal function co-management",
            "2026-08-27T20:00:00+00:00",
            "2026-08-27T20:30:00+00:00",
            Some("MRN-DAVID"),
        ),
        // Cancelled slot (`21:00Z` = `5:00 PM EDT`) -> Cancelled!
        fhir_appointment(
            "appt-cancelled-initial",
            "cancelled",
            "Cancelled blood pressure check",
            "2026-08-27T21:00:00+00:00",
            "2026-08-27T21:30:00+00:00",
            Some("MRN-CLARA"),
        ),
    ];
    fs::write(
        &calendar_file,
        format!("{}\n", initial_appointments.join("\n")),
    )
    .expect("write initial Appointment.ndjson");

    (bank, calendar_file)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn schedule_pane_and_briefing_view_pass_desktop_and_phone_browser_checks() {
    let site_dir = shared_site();
    let pg = TempPgServer::start().await;

    // -------------------------------------------------------------------------
    // Part 1: With no calendar source configured/synced, the schedule pane is absent.
    // -------------------------------------------------------------------------
    {
        let absent_db = pg.create_database("absent").await;
        let migrate = run_migrate_schema(
            &absent_db,
            &format!("{PHYSICIAN_EMAIL}|Dr. Test Physician|admin"),
        );
        assert!(
            migrate.status.success(),
            "absent schema migrate failed: {}",
            String::from_utf8_lossy(&migrate.stderr)
        );
        let (_site, base) = spawn_site_with_web(&absent_db, &site_dir, None).await;
        let http = reqwest::Client::new();
        let session = sign_in_http(&http, &base, PHYSICIAN_EMAIL).await;
        let day_json: Value = http
            .get(format!("{base}/practice-day"))
            .bearer_auth(&session)
            .send()
            .await
            .expect("GET /practice-day")
            .json()
            .await
            .expect("decode /practice-day");
        assert_eq!(day_json["source_state"]["state"], "absent");

        let mut browser = BrowserHarness::launch(DESKTOP, "America/New_York");
        browser.sign_in(&base, PHYSICIAN_EMAIL);
        let body = browser.browser.wait_for("Nothing needs you right now.");
        assert!(
            !browser.exists(".schedule-rail"),
            "schedule pane must not mount when calendar source is absent:\n{body}"
        );
        assert!(
            !body.contains("Schedule") && !body.contains("No visits scheduled today."),
            "absent calendar must not show an empty schedule card:\n{body}"
        );
        browser.assert_no_page_errors();
    }

    // -------------------------------------------------------------------------
    // Part 2: Seed real charts, schedule, source refresh failure, and visit briefings.
    // -------------------------------------------------------------------------
    let main_db = pg.create_database("full").await;
    let migrate = run_migrate_schema(
        &main_db,
        &format!("{PHYSICIAN_EMAIL}|Dr. Test Physician|admin"),
    );
    assert!(
        migrate.status.success(),
        "schema migrate failed: {}",
        String::from_utf8_lossy(&migrate.stderr)
    );

    let (bank, calendar_file) = build_synthetic_bank();
    let load = run_load_charts(&main_db, &bank);
    assert!(
        load.status.success(),
        "load-charts failed: {}",
        String::from_utf8_lossy(&load.stderr)
    );

    let chart_source = physician_sample::synthetic_chart::SOURCE_ID;
    let ws_alice = workspace_id(chart_source, "MRN-ALICE");
    let ws_bob = workspace_id(chart_source, "MRN-BOB");
    let ws_clara = workspace_id(chart_source, "MRN-CLARA");
    let ws_david = workspace_id(chart_source, "MRN-DAVID");

    let store = Store::connect(&main_db).await.expect("connect Store");
    let auth = Arc::new(
        PostgresAuthStore::connect_with_migrations(&main_db)
            .await
            .expect("connect PostgresAuthStore"),
    );
    let physician_id = staging_physician_id(PHYSICIAN_EMAIL);
    let admin_identity = PhysicianIdentity {
        practice_id: STAGING_PRACTICE_ID.to_owned(),
        physician_id: physician_id.clone(),
    };
    for ws in [&ws_alice, &ws_bob, &ws_clara, &ws_david] {
        auth.assign_panel(&admin_identity, &physician_id, ws, Membership::Allowed)
            .await
            .expect("assign patient workspace to physician panel");
    }

    // Record a failed external source refresh on Alice's workspace so the briefing's
    // `source_freshness` and `limits` carry real late/failed refresh state.
    let regional_source = RefreshSourceId::new("regional-hie").expect("source id");
    let failed_checker = FixedSourceCheck::new();
    failed_checker.set_source_outcome(
        STAGING_PRACTICE_ID,
        &ws_alice,
        regional_source.clone(),
        SourceCheckOutcome::Failed {
            source_label: Some("Regional HIE Lab Feed".to_owned()),
            checked_at: EARLY_WAKE_UNIX - 120,
            reason: "Regional HIE gateway timed out".to_owned(),
        },
    );
    let refresh_report = store
        .refresh_patient_sources_for_practice(
            &PracticeScope {
                practice: STAGING_PRACTICE_ID.to_owned(),
                workspace: ws_alice.clone(),
                job_id: "previsit-refresh-alice".to_owned(),
            },
            auth.as_ref(),
            &RefreshRequest {
                command_key: RefreshCommandKey::new("refresh-alice-regional-hie")
                    .expect("refresh command key"),
                target: RefreshTarget::selected([regional_source]).expect("selected source"),
            },
            &failed_checker,
        )
        .await
        .expect("record failed regional HIE refresh");
    assert_eq!(refresh_report.sources.len(), 1);

    // Inspect exact loaded chart revisions for Alice and Bob so citations verify cleanly.
    let alice_obs = store.sources(&ws_alice).await.expect("alice chart sources");
    let alice_discharge = alice_obs
        .sources
        .iter()
        .find(|src| src.candidate_id.contains("note-hf-discharge"))
        .expect("alice discharge DocumentReference");
    let alice_labs = alice_obs
        .sources
        .iter()
        .find(|src| src.candidate_id.contains("note-hf-labs"))
        .expect("alice labs DocumentReference");
    let alice_ref_1 = ExactRecordReference {
        candidate_id: alice_discharge.candidate_id.clone(),
        revision: alice_discharge.revision.clone(),
    };
    let alice_ref_2 = ExactRecordReference {
        candidate_id: alice_labs.candidate_id.clone(),
        revision: alice_labs.revision.clone(),
    };

    let bob_obs = store.sources(&ws_bob).await.expect("bob chart sources");
    let bob_note = bob_obs
        .sources
        .first()
        .expect("bob anticoagulation DocumentReference");
    let bob_ref = ExactRecordReference {
        candidate_id: bob_note.candidate_id.clone(),
        revision: bob_note.revision.clone(),
    };

    // Wake 1 at `13:30Z` (`EARLY_WAKE_UNIX`):
    // - `appt-ended` (`14:00Z`) -> `prepare_now` -> `Ready` (will be ended by `16:00Z`!)
    // - `appt-alice-ready` (`17:00Z`) -> `prepare_now` -> `Ready`
    // - `appt-bob-move` (`18:00Z`) -> `prepare_now` -> `Ready`
    // - `appt-clara-fail` (`19:00Z`) -> `prepare_now` -> exhausts attempts -> `CouldNotFinish`
    // - `appt-david-prep` (`20:00Z`) -> `schedule` at `16:00Z` (`PRACTICE_NOW_UNIX`)
    let saved_schedule = store
        .practice_schedule(STAGING_PRACTICE_ID)
        .await
        .expect("read saved schedule");
    let slot_start = |appt_id: &str| -> i64 {
        saved_schedule
            .appointments
            .iter()
            .find(|a| a.appointment_id.as_str() == appt_id)
            .unwrap_or_else(|| panic!("missing appointment {appt_id}"))
            .window
            .starts_at_unix()
    };
    let ended_starts = slot_start("appt-ended");
    let alice_starts = slot_start("appt-alice-ready");
    let bob_starts = slot_start("appt-bob-move");
    let clara_starts = slot_start("appt-clara-fail");
    let david_starts = slot_start("appt-david-prep");

    let ended_briefing = SaveBriefingArgs {
        visit_card_orientation: "Completed morning lab check.".to_owned(),
        trajectory: vec![BriefingClaimInput {
            text: "Morning lab check prior to afternoon heart failure follow-up.".to_owned(),
            uncertainty: None,
            citations: vec![alice_ref_2.clone()],
        }],
        meaningful_changes: vec![BriefingClaimInput {
            text: "Discharge serum creatinine was 1.42 mg/dL at hospital discharge.".to_owned(),
            uncertainty: None,
            citations: vec![alice_ref_2.clone()],
        }],
        unresolved_issues: vec![BriefingClaimInput {
            text: "Repeat outpatient basic metabolic panel pending.".to_owned(),
            uncertainty: None,
            citations: vec![alice_ref_2.clone()],
        }],
        visit_decisions: vec![BriefingDecisionInput {
            decision: "Confirm morning chemistry receipt".to_owned(),
            clinical_rationale: "Discharge creatinine was 1.42 mg/dL.".to_owned(),
            options_or_questions: vec!["Review morning chemistry if drawn.".to_owned()],
            uncertainty: None,
            citations: vec![alice_ref_2.clone()],
        }],
        basis_and_limitations: "Prepared from discharge chemistry note.".to_owned(),
        basis_citations: vec![alice_ref_2.clone()],
        limits: Vec::new(),
        visit_concern: None,
    };

    let alice_briefing = SaveBriefingArgs {
        visit_card_orientation:
            "3-day post-HFrEF discharge follow-up on torsemide 20 mg; outpatient BMP and standing weight need confirmation."
                .to_owned(),
        trajectory: vec![BriefingClaimInput {
            text:
                "Hospitalized for acute decompensated HFrEF (LVEF 30%) and discharged 2026-08-24 after IV diuresis."
                    .to_owned(),
            uncertainty: None,
            citations: vec![alice_ref_1.clone()],
        }],
        meaningful_changes: vec![BriefingClaimInput {
            text:
                "Loop diuretic switched at hospital discharge from bumetanide to oral torsemide 20 mg daily, with discharge creatinine 1.42 mg/dL (baseline 1.18 mg/dL) and NT-proBNP 2,840 pg/mL."
                    .to_owned(),
            uncertainty: None,
            citations: vec![alice_ref_1.clone(), alice_ref_2.clone()],
        }],
        unresolved_issues: vec![BriefingClaimInput {
            text:
                "72-hour post-discharge basic metabolic panel and daily home weight trend have not arrived from the regional lab feed."
                    .to_owned(),
            uncertainty: Some(
                "Current potassium and renal trajectory on torsemide 20 mg remain unverified until today's chemistry is reviewed."
                    .to_owned(),
            ),
            citations: vec![alice_ref_1.clone(), alice_ref_2.clone()],
        }],
        visit_decisions: vec![BriefingDecisionInput {
            decision:
                "Loop diuretic dosing and same-day electrolyte/renal assessment"
                    .to_owned(),
            clinical_rationale:
                "Discharge creatinine rose to 1.42 mg/dL with potassium 3.9 mEq/L at transition to torsemide 20 mg daily."
                    .to_owned(),
            options_or_questions: vec![
                "Assess standing weight, orthostatic blood pressure, and jugular venous pressure before adjusting torsemide 20 mg daily."
                    .to_owned(),
                "Obtain same-day basic metabolic panel prior to escalating renin-angiotensin or mineralocorticoid therapy."
                    .to_owned(),
            ],
            uncertainty: Some(
                "Regional HIE lab feed timed out during preparation; outside BMP drawn yesterday may already exist."
                    .to_owned(),
            ),
            citations: vec![alice_ref_1.clone(), alice_ref_2.clone()],
        }],
        basis_and_limitations:
            "Prepared from Synthetic chart discharge summary and discharge chemistry (2026-08-24). Regional HIE Lab Feed failed to refresh and post-discharge telemetry strip is missing."
                .to_owned(),
        basis_citations: vec![alice_ref_1.clone()],
        limits: vec![RecordLimit {
            kind: RecordLimitKind::Missing,
            subject: "cardiology-telemetry".to_owned(),
            detail: "Post-discharge ambulatory telemetry strip has not been delivered.".to_owned(),
        }],
        visit_concern: None,
    };

    let bob_briefing = SaveBriefingArgs {
        visit_card_orientation:
            "Warfarin follow-up after INR rose to 4.3 without bleeding; weekly dose reduction options open."
                .to_owned(),
        trajectory: vec![BriefingClaimInput {
            text: "Atrial fibrillation managed on warfarin 5 mg daily with supratherapeutic outpatient INR 4.3 on 2026-08-26.".to_owned(),
            uncertainty: None,
            citations: vec![bob_ref.clone()],
        }],
        meaningful_changes: vec![BriefingClaimInput {
            text: "Outpatient INR increased to 4.3 following reported dietary vitamin K change.".to_owned(),
            uncertainty: None,
            citations: vec![bob_ref.clone()],
        }],
        unresolved_issues: vec![BriefingClaimInput {
            text: "Confirm whether any over-the-counter NSAID or antibiotic interacted with warfarin this week.".to_owned(),
            uncertainty: Some("Recent non-prescription medication exposures have not yet been reviewed.".to_owned()),
            citations: vec![bob_ref.clone()],
        }],
        visit_decisions: vec![BriefingDecisionInput {
            decision: "Warfarin hold and maintenance dose adjustment".to_owned(),
            clinical_rationale: "INR 4.3 without bleeding warrants temporary hold confirmation and maintenance dose review.".to_owned(),
            options_or_questions: vec![
                "Confirm absence of mucosal or bruising symptoms and review 1-dose hold vs 10-15% weekly warfarin reduction.".to_owned(),
            ],
            uncertainty: None,
            citations: vec![bob_ref.clone()],
        }],
        basis_and_limitations: "Prepared from anticoagulation clinic note dated 2026-08-26.".to_owned(),
        basis_citations: vec![bob_ref.clone()],
        limits: Vec::new(),
        visit_concern: None,
    };

    let skills = load_skills();
    let roles = model_roles();
    let mut wake1_provider = Wake1Provider {
        selection_call: json!({
            "decisions": [
                {
                    "source_id": CALENDAR_SOURCE_ID,
                    "appointment_id": "appt-ended",
                    "starts_at_unix": ended_starts,
                    "action": "prepare_now",
                    "reason": "Morning lab review before afternoon visit."
                },
                {
                    "source_id": CALENDAR_SOURCE_ID,
                    "appointment_id": "appt-alice-ready",
                    "starts_at_unix": alice_starts,
                    "action": "prepare_now",
                    "reason": "3-day post-HFrEF discharge visit on torsemide 20 mg."
                },
                {
                    "source_id": CALENDAR_SOURCE_ID,
                    "appointment_id": "appt-bob-move",
                    "starts_at_unix": bob_starts,
                    "action": "prepare_now",
                    "reason": "Supratherapeutic INR 4.3 ahead of anticoagulation visit."
                },
                {
                    "source_id": CALENDAR_SOURCE_ID,
                    "appointment_id": "appt-clara-fail",
                    "starts_at_unix": clara_starts,
                    "action": "prepare_now",
                    "reason": "Post-urgent-care COPD exacerbation follow-up."
                },
                {
                    "source_id": CALENDAR_SOURCE_ID,
                    "appointment_id": "appt-david-prep",
                    "starts_at_unix": david_starts,
                    "action": "schedule",
                    "prepare_at_unix": PRACTICE_NOW_UNIX,
                    "reason": "Prepare renal co-management briefing at 16:00Z."
                }
            ]
        }),
        ended_read: json!({
            "references": [{
                "candidate_id": alice_ref_2.candidate_id,
                "revision": alice_ref_2.revision,
                "cursor": null
            }],
            "max_result_bytes": 16_384
        }),
        ended_save: serde_json::to_value(&ended_briefing).expect("serialize ended briefing"),
        alice_read: json!({
            "references": [
                {
                    "candidate_id": alice_ref_1.candidate_id,
                    "revision": alice_ref_1.revision,
                    "cursor": null
                },
                {
                    "candidate_id": alice_ref_2.candidate_id,
                    "revision": alice_ref_2.revision,
                    "cursor": null
                }
            ],
            "max_result_bytes": 32_768
        }),
        alice_save: serde_json::to_value(&alice_briefing).expect("serialize alice briefing"),
        bob_read: json!({
            "references": [{
                "candidate_id": bob_ref.candidate_id,
                "revision": bob_ref.revision,
                "cursor": null
            }],
            "max_result_bytes": 16_384
        }),
        bob_save: serde_json::to_value(&bob_briefing).expect("serialize bob briefing"),
    };
    let wake1_summary = store
        .wake_visit_preparation(
            auth.as_ref(),
            STAGING_PRACTICE_ID,
            &roles,
            &skills,
            &mut wake1_provider,
            EARLY_WAKE_UNIX,
        )
        .await
        .expect("wake1 visit preparation");
    assert_eq!(wake1_summary.completed_briefings.len(), 3);
    assert_eq!(wake1_summary.failed_jobs.len(), 1);

    // Wake 2 at `16:00Z` (`PRACTICE_NOW_UNIX`): queues and claims `appt-david-prep`
    // in `Preparing` state while the run is actively held in-flight.
    let (started_tx, started_rx) = oneshot::channel();
    let store_bg = store.clone();
    let auth_bg = Arc::clone(&auth);
    let roles_bg = roles.clone();
    let skills_bg = skills.clone();
    let in_flight_task = tokio::spawn(async move {
        let mut pending_provider = PendingBriefingProvider {
            started: Some(started_tx),
        };
        let _ = store_bg
            .wake_visit_preparation(
                auth_bg.as_ref(),
                STAGING_PRACTICE_ID,
                &roles_bg,
                &skills_bg,
                &mut pending_provider,
                PRACTICE_NOW_UNIX,
            )
            .await;
    });
    tokio::time::timeout(Duration::from_secs(10), started_rx)
        .await
        .expect("David briefing job entered active Preparing state")
        .expect("started signal");
    in_flight_task.abort();
    let _ = in_flight_task.await;

    // Confirm all expected backend preparation states are persisted in Postgres.
    let preps = store
        .visit_preparations_for_practice(STAGING_PRACTICE_ID, auth.as_ref())
        .await
        .expect("read visit preparations");
    assert!(
        preps.iter().any(|p| {
            p.slot.appointment_id == "appt-david-prep"
                && matches!(p.state, VisitPreparationState::Preparing { .. })
        }),
        "expected appt-david-prep in Preparing state, got {preps:#?}"
    );
    assert!(
        preps.iter().any(|p| {
            p.slot.appointment_id == "appt-clara-fail"
                && matches!(p.state, VisitPreparationState::CouldNotFinish { .. })
        }),
        "expected appt-clara-fail in CouldNotFinish state, got {preps:#?}"
    );

    // -------------------------------------------------------------------------
    // Part 3: Start live `physician-sample` with `PHYSICIAN_SYNTHETIC_CALENDAR`
    // and run full Chromium verification across desktop (`1280x800`) and phone (`390x844`).
    // -------------------------------------------------------------------------
    let (_site, base) = spawn_site_with_web(&main_db, &site_dir, Some(&calendar_file)).await;
    let http = reqwest::Client::new();
    let api_session = sign_in_http(&http, &base, PHYSICIAN_EMAIL).await;

    let mut browser = BrowserHarness::launch(DESKTOP, "America/New_York");
    browser.sign_in(&base, PHYSICIAN_EMAIL);
    let today_ny = browser.browser.wait_for("Alice Carter");

    // AC1: Ended 10:00 AM appointment (`appt-ended`) dropped off and does not show as prepared;
    // all 5 active states (`Briefing ready`, `Preparation unavailable`, `Preparation failed`,
    // `Preparing`, `Cancelled`) render with times in `America/New_York`.
    assert!(
        !today_ny.contains("Morning lab review") && !today_ny.contains("10:00 AM"),
        "ended appointment must drop off Today schedule rail:\n{today_ny}"
    );
    for expected in [
        "Schedule",
        "1:00 PM",
        "Briefing ready",
        "Alice Carter",
        "3-day post-HFrEF discharge follow-up on torsemide 20 mg",
        "2:00 PM",
        "Bob Diaz",
        "2:30 PM",
        "Preparation unavailable",
        "Walk-in transfer consult",
        "3:00 PM",
        "Preparation failed",
        "Clara Nguyen",
        "4:00 PM",
        "Preparing",
        "David Patel",
        "5:00 PM",
        "Cancelled",
        "Cancelled blood pressure check",
    ] {
        assert!(
            today_ny.contains(expected),
            "desktop Today schedule missing `{expected}`:\n{today_ny}"
        );
    }
    browser.capture_screenshot("briefing-schedule-01-desktop-today.png");

    // AC1: Browser time zone shift (`America/Los_Angeles` shifts `17:00Z` from `1:00 PM` to `10:00 AM`).
    browser.set_timezone("America/Los_Angeles");
    let today_la = browser
        .browser
        .text_until(&format!("{base}/"), "Alice Carter");
    for expected_la in [
        "10:00 AM", "11:00 AM", "11:30 AM", "12:00 PM", "1:00 PM", "2:00 PM",
    ] {
        assert!(
            today_la.contains(expected_la),
            "Pacific browser timezone missing `{expected_la}`:\n{today_la}"
        );
    }
    assert!(
        !today_la.contains("5:00 PM"),
        "Eastern 5:00 PM slot should shift to 2:00 PM in America/Los_Angeles:\n{today_la}"
    );

    // Restore `America/New_York` for the remaining desktop and phone checks.
    browser.set_timezone("America/New_York");
    browser.browser.text_until(&format!("{base}/"), "1:00 PM");

    // AC3: Narrow phone screen (`390x844`) shows ONLY the next visit (`Alice Carter` at `1:00 PM`),
    // while keeping the full list in DOM without horizontal overflow.
    browser.resize(PHONE);
    std::thread::sleep(Duration::from_millis(250));
    let phone_html = browser.browser.html();
    let phone_schedule_text = browser.text(".schedule-rail");
    assert!(
        phone_html.contains("Bob Diaz") && phone_html.contains("David Patel"),
        "full schedule remains mounted in DOM for responsive transitions"
    );
    assert!(
        phone_schedule_text.contains("Alice Carter")
            && phone_schedule_text.contains("1:00 PM")
            && phone_schedule_text.contains("Briefing ready"),
        "phone schedule must show next visit (`Alice Carter` at `1:00 PM`):\n{phone_schedule_text}"
    );
    for hidden_later_visit in [
        "Bob Diaz",
        "Walk-in transfer consult",
        "Clara Nguyen",
        "David Patel",
        "Cancelled blood pressure check",
    ] {
        assert!(
            !phone_schedule_text.contains(hidden_later_visit),
            "narrow phone schedule must hide non-next visit `{hidden_later_visit}`:\n{phone_schedule_text}"
        );
    }
    browser.capture_screenshot("briefing-schedule-02-phone-today-next-visit.png");

    // AC1 + AC3 + AC5: Open ready visit briefing on phone (`390x844`), verify zero sideways scroll,
    // then inspect citations, late/missing sources, Basis and limitations, and Sources on desktop.
    browser.press_first(".schedule-visit.is-next");
    let phone_briefing_text = browser
        .browser
        .wait_for("Loop diuretic dosing and same-day electrolyte/renal assessment");
    let phone_briefing_upper = phone_briefing_text.to_uppercase();
    for expected_heading in [
        "TRAJECTORY",
        "MEANINGFUL CHANGES",
        "UNRESOLVED ISSUES",
        "DECISIONS FOR THIS VISIT",
    ] {
        assert!(
            phone_briefing_upper.contains(expected_heading),
            "briefing view missing heading `{expected_heading}`:\n{phone_briefing_text}"
        );
    }
    for expected_section in [
        "Late or missing sources",
        "Regional HIE Lab Feed: couldn’t update records",
        "Post-discharge ambulatory telemetry strip has not been delivered.",
        "Uncertainty",
    ] {
        assert!(
            phone_briefing_text.contains(expected_section),
            "briefing view missing `{expected_section}`:\n{phone_briefing_text}"
        );
    }
    assert!(
        !phone_briefing_text.contains("(regional-hie)"),
        "briefing notice must not duplicate or leak raw source_id `(regional-hie)`:\n{phone_briefing_text}"
    );
    browser.capture_screenshot("briefing-schedule-03-phone-briefing.png");

    browser.resize(DESKTOP);
    std::thread::sleep(Duration::from_millis(250));
    browser.capture_screenshot("briefing-schedule-04-desktop-briefing.png");

    // AC1: Inline citation chip opens the exact cited passage in the side panel without losing the briefing underneath.
    browser.press_first("button.citation-chip");
    browser.wait_exists(".source-reading-pane");
    let with_citation = browser
        .browser
        .wait_for("loop diuretic switched from bumetanide to oral torsemide 20 mg daily");
    assert!(
        browser.exists(".briefing-document"),
        "briefing document must stay mounted beside citation side panel:\n{with_citation}"
    );
    assert!(
        !with_citation
            .lines()
            .any(|line| line.trim().eq_ignore_ascii_case("attachment")),
        "citation side panel must omit generic `attachment` part labels:\n{with_citation}"
    );
    browser.capture_screenshot("briefing-schedule-05-desktop-citation.png");
    browser.press("Close");
    browser.wait_absent(".source-reading-pane");

    // AC5: Basis and limitations and Sources open directly from the briefing footer.
    browser.press_first("button.briefing-basis-toggle");
    browser.wait_exists(".briefing-basis-panel");
    browser.press_first("button.briefing-sources-toggle");
    browser.wait_exists(".briefing-sources-panel");
    let expanded_footer_text = browser
        .browser
        .wait_for("Prepared from Synthetic chart discharge summary");
    assert!(
        expanded_footer_text.contains("Synthetic chart: records as of")
            && expanded_footer_text.contains("Cardiology Hospital Discharge Summary"),
        "expanded Basis and Sources panels missing expected basis/provenance:\n{expanded_footer_text}"
    );
    browser.reveal(".briefing-footer");
    browser.capture_screenshot("briefing-schedule-06-desktop-basis-sources.png");

    // Clicking a source row inside `Sources` also opens the citation inspection side panel.
    browser.press_first(".briefing-sources-list button");
    browser.wait_exists(".source-reading-pane");
    browser.press("Close");
    browser.wait_absent(".source-reading-pane");

    browser.reveal(".briefing-nav");
    browser.press("Back to Today");
    browser.browser.wait_for("Schedule");

    // -------------------------------------------------------------------------
    // Part 4 (AC2): Failed schedule refresh retains last visits with `"as of"` + `Try again`;
    // cancelled and moved visits keep their briefings in history with explicit notices.
    // -------------------------------------------------------------------------
    fs::remove_file(&calendar_file).expect("remove calendar file to simulate source outage");
    let failed_refresh_resp = http
        .post(format!("{base}/practice-day/refresh"))
        .bearer_auth(&api_session)
        .send()
        .await
        .expect("POST /practice-day/refresh during outage");
    assert!(failed_refresh_resp.status().is_success());
    let stale_payload: Value = failed_refresh_resp
        .json()
        .await
        .expect("decode stale /practice-day/refresh");
    assert_eq!(stale_payload["source_state"]["state"], "usable");
    assert_eq!(stale_payload["source_state"]["freshness"], "refresh_failed");
    assert_eq!(stale_payload["source_state"]["refresh_status"], "failed");

    let stale_today_text = browser.browser.text_until(
        &format!("{base}/"),
        "Coda couldn’t refresh the schedule. Showing visits as of ",
    );
    assert!(
        stale_today_text.contains("Try again")
            && stale_today_text.contains("Alice Carter")
            && stale_today_text.contains("Bob Diaz"),
        "failed refresh must retain prior visits alongside stale `as of` notice and Try again:\n{stale_today_text}"
    );
    browser.capture_screenshot("briefing-schedule-07-desktop-stale-refresh.png");

    // Restore calendar with:
    // - `appt-ended` (`10:00 AM`) still completed (`fulfilled`)
    // - `appt-alice-ready` (`1:00 PM`) CANCELLED
    // - `appt-bob-move` MOVED from `18:00Z` (`2:00 PM`) to `20:30Z` (`4:30 PM`)
    let updated_appointments = [
        fhir_appointment(
            "appt-ended",
            "fulfilled",
            "Morning lab review",
            "2026-08-27T14:00:00+00:00",
            "2026-08-27T14:30:00+00:00",
            Some("MRN-ALICE"),
        ),
        fhir_appointment(
            "appt-alice-ready",
            "cancelled",
            "Post-discharge heart failure follow-up",
            "2026-08-27T17:00:00+00:00",
            "2026-08-27T17:30:00+00:00",
            Some("MRN-ALICE"),
        ),
        fhir_appointment(
            "appt-bob-move",
            "booked",
            "Anticoagulation dose review",
            "2026-08-27T20:30:00+00:00",
            "2026-08-27T21:00:00+00:00",
            Some("MRN-BOB"),
        ),
        fhir_appointment(
            "appt-unmatched",
            "booked",
            "Walk-in transfer consult",
            "2026-08-27T18:30:00+00:00",
            "2026-08-27T19:00:00+00:00",
            Some("MRN-UNREGISTERED"),
        ),
        fhir_appointment(
            "appt-clara-fail",
            "booked",
            "Pulmonary exacerbation check",
            "2026-08-27T19:00:00+00:00",
            "2026-08-27T19:30:00+00:00",
            Some("MRN-CLARA"),
        ),
        fhir_appointment(
            "appt-david-prep",
            "booked",
            "Renal function co-management",
            "2026-08-27T20:00:00+00:00",
            "2026-08-27T20:30:00+00:00",
            Some("MRN-DAVID"),
        ),
    ];
    fs::write(
        &calendar_file,
        format!("{}\n", updated_appointments.join("\n")),
    )
    .expect("write updated Appointment.ndjson");

    // Click `Try again` in `.schedule-stale-notice` -> calls `POST /practice-day/refresh` and updates live!
    browser.press_first(".schedule-stale-notice button");
    browser.wait_absent(".schedule-stale-notice");
    let refreshed_today = browser.browser.wait_for("Briefing kept in history");
    assert!(
        !refreshed_today.contains("Coda couldn’t refresh the schedule"),
        "stale notice must clear once Try again succeeds:\n{refreshed_today}"
    );
    assert!(
        !refreshed_today.contains("Briefing ready"),
        "neither Alice's cancelled slot nor Bob's moved 4:30 PM slot may look currently prepared:\n{refreshed_today}"
    );
    assert!(
        refreshed_today.contains("4:30 PM") && refreshed_today.contains("Bob Diaz"),
        "Bob's rescheduled 4:30 PM slot must appear on the refreshed schedule:\n{refreshed_today}"
    );

    // Clicking Alice's cancelled slot opens her preserved historical briefing with explicit cancellation notice.
    browser.press("Open cancelled visit briefing for Alice Carter");
    let cancelled_briefing_text = browser.browser.wait_for("This visit was cancelled.");
    assert!(
        cancelled_briefing_text.contains("Kept in history for the originally scheduled slot")
            && cancelled_briefing_text
                .to_uppercase()
                .contains("DECISIONS FOR THIS VISIT"),
        "cancelled briefing must retain its full content and cancellation notice:\n{cancelled_briefing_text}"
    );

    // Navigate to Alice's patient page -> verify `Briefing history` and `Past work` show both
    // `Visit cancelled` (1:00 PM) and `Past visit` (10:00 AM ended slot), and never `Briefing ready`.
    browser.press("Alice Carter · Patient page");
    let alice_page_text = browser.browser.wait_for("Visit cancelled");
    assert!(
        alice_page_text.to_uppercase().contains("BRIEFING HISTORY")
            && alice_page_text.contains("Kept in history")
            && alice_page_text.contains("Past visit")
            && !alice_page_text.contains("Briefing ready"),
        "Alice's patient page must show cancelled and ended briefings in Briefing history without looking currently prepared:\n{alice_page_text}"
    );
    browser.capture_screenshot("briefing-schedule-09-desktop-patient-history.png");
    browser.press("Past work");
    browser.wait_exists("dialog[open]");
    let past_work_text = browser
        .browser
        .wait_for("Cancelled visit (Today, 1:00 PM) · Kept in history");
    assert!(
        past_work_text.contains("Post-discharge heart failure follow-up · Visit briefing")
            && past_work_text.contains("Visit cancelled")
            && past_work_text.contains("Past visit"),
        "Past work dialog must list historical cancelled and ended briefings:\n{past_work_text}"
    );
    browser.press("Close");
    browser.wait_absent("dialog[open]");

    // Open Bob Diaz's patient page -> verify `Upcoming visits` shows `4:30 PM` (unprepared) and
    // `Briefing history` keeps the `2:00 PM` briefing with `Visit moved` notice; open it!
    browser.press("Find patient");
    browser.wait_exists(".search-field input");
    browser.browser.type_into(".search-field input", "Bob Diaz");
    browser.browser.wait_for("Bob Diaz");
    browser.wait_exists(".dialog-results button");
    browser.press_first(".dialog-results button");
    let bob_page_text = browser.browser.wait_for("Visit moved");
    assert!(
        bob_page_text.to_uppercase().contains("BRIEFING HISTORY")
            && bob_page_text.contains("Moved to Today, 4:30 PM")
            && bob_page_text.contains("Kept in history")
            && !bob_page_text.contains("Briefing ready"),
        "Bob's patient page must show Visit moved in Briefing history without marking 4:30 PM prepared:\n{bob_page_text}"
    );
    browser.press_first(".patient-briefings-history .patient-briefing-row");
    let moved_briefing_text = browser
        .browser
        .wait_for("This visit moved to Today, 4:30 PM.");
    assert!(
        moved_briefing_text.contains("Kept in history for the earlier scheduled slot")
            && moved_briefing_text.contains("Warfarin hold and maintenance dose adjustment"),
        "moved briefing must show moved notice and preserved clinical content:\n{moved_briefing_text}"
    );
    browser.capture_screenshot("briefing-schedule-08-desktop-moved-history.png");

    browser.assert_no_page_errors();
    let _ = fs::remove_dir_all(&bank);
}

async fn spawn_durable_site_with_web(
    db_url: &str,
    site_dir: &Path,
    synthetic_calendar: &Path,
) -> (RunningSite, String) {
    let address = TcpListener::bind("127.0.0.1:0")
        .expect("bind durable site port")
        .local_addr()
        .expect("durable site local addr");
    let skills_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills");
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_physician-sample"));
    cmd.env("PHYSICIAN_DATABASE_URL", db_url)
        .env("PHYSICIAN_SITE_DIR", site_dir)
        .env("PHYSICIAN_AUTH", "required")
        .env("PHYSICIAN_AUTH_PROOF_TOKENS", "1")
        .env("PHYSICIAN_DURABLE_CHAT", "1")
        .env("PHYSICIAN_SKILLS_DIR", &skills_dir)
        .env("CODA_CHAT_MODEL", "browser-test-chat-model")
        .env("CODA_CHAT_EFFORT", "low")
        .env("CODA_CHAT_CONTEXT_WINDOW_TOKENS", "128000")
        .env("CODA_CHAT_INPUT_LIMIT_TOKENS", "100000")
        .env("CODA_BACKGROUND_MODEL", "browser-test-bg-model")
        .env("CODA_BACKGROUND_EFFORT", "low")
        .env("CODA_BACKGROUND_CONTEXT_WINDOW_TOKENS", "128000")
        .env("CODA_BACKGROUND_INPUT_LIMIT_TOKENS", "100000")
        .env("PHYSICIAN_SYNTHETIC_CALENDAR", synthetic_calendar)
        .env("PHYSICIAN_LISTEN_ADDR", address.to_string())
        .env_remove("PHYSICIAN_SYNTHETIC_CHARTS")
        .env_remove("CODA_LOCAL_MODEL_EXECUTION")
        .env_remove("PHYSICIAN_CHAT_TOPIC")
        .env_remove("PHYSICIAN_UPSTREAM")
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    let mut site = RunningSite(cmd.spawn().expect("spawn durable physician-sample"));
    let base = format!("http://{address}");
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        if let Some(status) = site.0.try_wait().expect("poll durable site") {
            let mut err = String::new();
            if let Some(mut stderr) = site.0.stderr.take() {
                let _ = std::io::Read::read_to_string(&mut stderr, &mut err);
            }
            panic!("durable physician-sample exited unexpectedly ({status}):\n{err}");
        }
        let ready = reqwest::get(format!("{base}/readyz")).await;
        if ready.is_ok_and(|resp| resp.status().is_success()) {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "durable physician-sample never became ready"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    (site, base)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn schedule_panel_choice_persists_across_sign_out_and_background_sync_does_not_steal_focus() {
    let site_dir = shared_site();
    let pg = TempPgServer::start().await;
    let db = pg.create_database("panel_focus").await;
    let migrate = run_migrate_schema(&db, &format!("{PHYSICIAN_EMAIL}|Dr. Test Physician|admin"));
    assert!(
        migrate.status.success(),
        "schema migrate failed: {}",
        String::from_utf8_lossy(&migrate.stderr)
    );

    let (bank, calendar_file) = build_synthetic_bank();
    let load = run_load_charts(&db, &bank);
    assert!(
        load.status.success(),
        "load-charts failed: {}",
        String::from_utf8_lossy(&load.stderr)
    );

    let chart_source = physician_sample::synthetic_chart::SOURCE_ID;
    let ws_alice = workspace_id(chart_source, "MRN-ALICE");
    let ws_bob = workspace_id(chart_source, "MRN-BOB");
    let store = Store::connect(&db).await.expect("connect Store");
    let auth = Arc::new(
        PostgresAuthStore::connect_with_migrations(&db)
            .await
            .expect("connect PostgresAuthStore"),
    );
    let physician_id = staging_physician_id(PHYSICIAN_EMAIL);
    let admin_identity = PhysicianIdentity {
        practice_id: STAGING_PRACTICE_ID.to_owned(),
        physician_id,
    };
    for ws in [&ws_alice, &ws_bob] {
        auth.set_membership(&admin_identity, ws, Membership::Allowed)
            .await
            .expect("place patient on admin panel");
    }
    let calendar = physician_sample::schedule::SyntheticBankCalendar::from_file(&calendar_file);
    physician_sample::schedule::sync_synthetic_schedule(
        &calendar,
        auth.as_ref(),
        &store,
        STAGING_PRACTICE_ID,
    )
    .await
    .expect("initial schedule sync");

    let (_site, base) = spawn_durable_site_with_web(&db, &site_dir, &calendar_file).await;
    let http = reqwest::Client::new();
    let api_session = sign_in_http(&http, &base, PHYSICIAN_EMAIL).await;

    let mut browser = BrowserHarness::launch(DESKTOP, "America/New_York");
    browser.sign_in(&base, PHYSICIAN_EMAIL);
    browser.browser.wait_for("Alice Carter");
    browser.wait_exists(".schedule-rail");
    assert!(
        !browser.exists(".schedule-rail.is-collapsed"),
        "schedule rail must start expanded by default"
    );

    // -------------------------------------------------------------------------
    // AC4 (`P4.Rule.PanelChoiceRemembered`): Collapse schedule rail -> survives
    // reload AND survives sign-out -> sign-in (even after localStorage is cleared).
    // -------------------------------------------------------------------------
    browser.press_first(r#".schedule-rail button[aria-label^="Collapse "]"#);
    browser.wait_exists(".schedule-rail.is-collapsed");

    // Wait for PUT /physician-state/panels to land durably in Postgres.
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        let prefs: physician_wire::PanelPreferencesWire = http
            .get(format!("{base}/physician-state/panels"))
            .bearer_auth(&api_session)
            .send()
            .await
            .expect("GET /physician-state/panels")
            .json()
            .await
            .expect("decode panels");
        if prefs.schedule_collapsed {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "PUT /physician-state/panels never persisted schedule_collapsed=true"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    // Reload preserves collapsed schedule rail.
    browser.browser.text_until(&base, "Today");
    browser.wait_exists(".schedule-rail.is-collapsed");

    // Sign out clears browser localStorage, then sign back in -> server restores collapsed state!
    browser.press("Sign out of Coda");
    browser.browser.wait_for("Sign in to Coda");
    browser.sign_in(&base, PHYSICIAN_EMAIL);
    browser.wait_exists(".schedule-rail.is-collapsed");
    browser.capture_screenshot("briefing-schedule-10-panel-persisted-after-signout.png");

    // Expand schedule rail again for the background focus-retention check.
    browser.press_first(r#".schedule-rail button[aria-label^="Show "]"#);
    browser.wait_absent(".schedule-rail.is-collapsed");
    browser.browser.wait_for("Alice Carter");

    // -------------------------------------------------------------------------
    // AC3 (`P4.Rule.BackgroundNoStealFocus`): Keep focus on an interactive control
    // on Today, then trigger a background schedule sync (`POST /director/schedule/sync`)
    // adding a new appointment. The schedule updates live via SSE while focus stays
    // untouched and no briefing or patient page is auto-opened.
    // -------------------------------------------------------------------------
    let focused = browser.run_walk("focus", &[json!(r#"button[aria-label="Find patient"]"#)]);
    assert_eq!(
        focused,
        json!(true),
        "expected Find patient button to take focus before background sync"
    );
    assert_eq!(
        browser.run_walk(
            "activeMatches",
            &[json!(r#"button[aria-label="Find patient"]"#)]
        ),
        json!(true)
    );

    let mut lines = fs::read_to_string(&calendar_file).expect("read calendar");
    lines.push_str(&fhir_appointment(
        "appt-bg-live-focus",
        "booked",
        "Live background cardiology consult",
        "2026-08-27T17:30:00+00:00",
        "2026-08-27T18:00:00+00:00",
        Some("MRN-ALICE"),
    ));
    lines.push('\n');
    fs::write(&calendar_file, lines).expect("append live background appointment");

    let sync_status = http
        .post(format!("{base}/director/schedule/sync"))
        .bearer_auth(&api_session)
        .send()
        .await
        .expect("POST /director/schedule/sync")
        .status();
    assert_eq!(sync_status, reqwest::StatusCode::OK);

    // The newly synced appointment appears live in Today's schedule pane via SSE.
    browser
        .browser
        .wait_for("Live background cardiology consult");

    // Focus must still rest on `Find patient`, and no briefing/patient view was opened.
    assert_eq!(
        browser.run_walk(
            "activeMatches",
            &[json!(r#"button[aria-label="Find patient"]"#)]
        ),
        json!(true),
        "background schedule/briefing SSE update must never steal keyboard focus"
    );
    assert!(
        !browser.exists(".briefing-view"),
        "background update must never auto-open a visit briefing"
    );
    browser.capture_screenshot("briefing-schedule-11-background-live-no-focus-steal.png");

    browser.assert_no_page_errors();
    let _ = fs::remove_dir_all(&bank);
}
