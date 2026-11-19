//! Every screen of the physician shell that holds a shared control or card, walked in a real browser
//! the way a person reaches it, at desktop and phone sizes in light and dark: the sidebar in each
//! form, Find patient through loading, empty, error and recovery, the dialogs, a patient's cards, and
//! the clinic record's panel picker.
//!
//! `js/sign_in_states.js` scripts the answers the sample server never gives. With `SHELL_SHOTS` set to
//! a folder, each screen is also saved there as a screenshot, so two revisions can be compared.

use std::sync::Arc;
use std::time::{Duration, Instant};

use base64::Engine;
use physician_auth::{AuthStore, Membership, MemoryAuthStore, PhysicianIdentity};
use serde_json::{Value, json};

#[allow(dead_code)]
mod common;
#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;
#[allow(dead_code)]
mod required_auth;

use devtools::{Browser, DESKTOP, PHONE};
use required_auth::shared_site;

const MARY: &str = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
const SEARCH_FIELD: &str = ".search-field input";
const FIRST_RESULT: &str = ".dialog-results button";
const FOLD_SCHEDULE: &str = r#".schedule-rail button[aria-expanded="true"]"#;
const UNFOLD_SCHEDULE: &str = r#".schedule-rail button[aria-expanded="false"]"#;

struct Walk {
    browser: Browser,
    size: &'static str,
    theme: &'static str,
    shots: Option<std::path::PathBuf>,
}

impl Walk {
    fn launch(size: (&'static str, (u32, u32)), theme: &'static str) -> Self {
        let mut browser = Browser::launch();
        browser.resize(size.1);
        browser.page().call_session("Page.enable", json!({}));
        browser.page().call_session(
            "Emulation.setTimezoneOverride",
            json!({ "timezoneId": "America/New_York" }),
        );
        browser.page().call_session(
            "Page.addScriptToEvaluateOnNewDocument",
            json!({ "source": include_str!("js/sign_in_states.js") }),
        );
        Self {
            browser,
            size: size.0,
            theme,
            shots: std::env::var_os("SHELL_SHOTS").map(Into::into),
        }
    }

    /// Runs `name` from `js/shell_walk.js` with `args`.
    fn run(&mut self, name: &str, args: &[Value]) -> Value {
        let mut arguments = vec![json!(name)];
        arguments.extend_from_slice(args);
        self.browser
            .page()
            .call_function(include_str!("js/shell_walk.js"), &arguments)
    }

    /// Loads `url` until it says `needle`, in this walk's theme. The app has no theme setting of its
    /// own: dark is the library's `data-theme`.
    fn open(&mut self, url: &str, needle: &str) {
        self.browser.text_until(url, needle);
        let theme = self.theme;
        self.run("theme", &[json!(theme)]);
    }

    fn answer(&mut self, step: Value) {
        self.run("answer", &[step]);
    }

    fn until(&mut self, what: &str, helper: &str, args: &[Value]) {
        let deadline = Instant::now() + Duration::from_secs(10);
        while self.run(helper, args) != json!(true) {
            assert!(
                Instant::now() < deadline,
                "{what} never came at {} {}",
                self.size,
                self.theme
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    /// Presses the button with these words or this name once it can be pressed.
    fn press(&mut self, words: &str) {
        self.until(words, "press", &[json!(words)]);
    }

    /// Presses a button in the sidebar, which a phone opens as a drawer first.
    fn press_in_sidebar(&mut self, words: &str) {
        if self.size == "phone" {
            self.press("Open navigation");
        }
        self.press(words);
    }

    /// Presses the first button this selector finds once it can be pressed.
    fn press_first(&mut self, selector: &str) {
        self.until(selector, "pressFirst", &[json!(selector)]);
    }

    fn appears(&mut self, selector: &str) {
        self.until(selector, "exists", &[json!(selector)]);
    }

    fn exists(&mut self, selector: &str) -> bool {
        self.run("exists", &[json!(selector)]) == json!(true)
    }

    /// Holds the screen: no sideways scroll, and a screenshot once it has arrived.
    fn stage(&mut self, name: &str) {
        let overflow = self.run("sidewaysScroll", &[]);
        assert!(
            overflow.as_f64().is_some_and(|by| by <= 0.0),
            "{name} scrolls sideways by {overflow} at {} {}",
            self.size,
            self.theme
        );
        let Some(folder) = self.shots.clone() else {
            return;
        };
        std::thread::sleep(Duration::from_millis(500));
        let shot = self
            .browser
            .page()
            .call_session("Page.captureScreenshot", json!({ "format": "png" }));
        let data = shot
            .pointer("/result/data")
            .and_then(Value::as_str)
            .expect("a screenshot");
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(data)
            .expect("a screenshot's bytes");
        std::fs::create_dir_all(&folder).expect("the screenshot folder");
        let file = folder.join(format!("{}-{}-{name}.png", self.size, self.theme));
        std::fs::write(file, bytes).expect("the screenshot");
    }

    /// Scrolls `selector` to the top and holds the screen.
    fn stage_at(&mut self, selector: &str, name: &str) {
        self.run("reveal", &[json!(selector)]);
        self.stage(name);
    }

    /// The sidebar in each form this size has, the schedule folded and open, and the app put away.
    fn walk_the_frame(&mut self) {
        if self.size == "phone" {
            self.press("Open navigation");
            self.browser.wait_for("Find patient");
            self.stage("02-sidebar-drawer");
            self.press("Close navigation");
        } else {
            self.press("Collapse navigation");
            self.appears(".glass-app.has-collapsed-sidebar");
            self.stage("02-sidebar-rail");
            self.press("Expand navigation");
        }
        if self.exists(FOLD_SCHEDULE) {
            self.stage_at(".schedule-rail", "03-schedule-open");
            self.press_first(FOLD_SCHEDULE);
            self.appears(UNFOLD_SCHEDULE);
            self.stage_at(".schedule-rail", "03-schedule-folded");
            self.press_first(UNFOLD_SCHEDULE);
            self.appears(FOLD_SCHEDULE);
        }
        self.press("Collapse Coda");
        self.appears(r#"button[aria-label="Open Coda"]"#);
        self.stage("04-app-put-away");
        self.press("Open Coda");
        self.browser.wait_for("Collapse Coda");
    }

    /// Loads `base` with these answers scripted, until it says `needle`.
    fn reload_with(&mut self, base: &str, steps: Value, needle: &str) {
        self.run("answerNextLoad", &[steps]);
        self.open(&format!("{base}/"), needle);
    }

    /// The schedule rail through loading, nothing scheduled, the failure and its way back.
    fn walk_the_schedule(&mut self, base: &str) {
        self.reload_with(
            base,
            json!([{ "path": "/practice-day", "hold": true, "keep": true,
                "status": 200, "body": calendar() }]),
            "Loading the schedule…",
        );
        self.stage_at(".schedule-rail", "03a-schedule-loading");
        self.run("forgetAnswers", &[]);
        self.run("release", &[]);
        self.browser.wait_for("3:30 PM");

        let mut empty_day = calendar();
        empty_day["visits"] = json!([]);
        self.reload_with(
            base,
            json!([{ "path": "/practice-day", "status": 200, "keep": true, "body": empty_day }]),
            "No visits scheduled today.",
        );
        self.stage_at(".schedule-rail", "03b-schedule-empty");

        self.reload_with(
            base,
            json!([{ "path": "/practice-day", "status": 500, "keep": true, "body": {} }]),
            "Coda couldn’t load the schedule.",
        );
        self.stage_at(".schedule-rail", "03c-schedule-failed");
        self.run("forgetAnswers", &[]);
        self.answer(json!({ "path": "/practice-day", "status": 200, "body": calendar() }));
        self.press("Try again");
        let recovered = self.browser.wait_for("3:30 PM");
        assert!(
            !recovered.contains("Coda couldn’t load the schedule."),
            "the failure outlived its answer:\n{recovered}"
        );
    }

    /// The Connections dialog, which only an account with connections offers: quiet, then with
    /// Today out of reach.
    fn walk_the_connections(&mut self, base: &str) {
        let health = json!({ "path": "/health", "status": 200, "keep": true,
            "body": { "status": "ok", "capabilities": { "connection_accounts": true } } });
        self.reload_with(base, json!([health.clone()]), "Collapse Coda");
        self.press_in_sidebar("Connections");
        self.browser.wait_for("Nothing needs you right now.");
        self.stage("10a-connections-quiet");
        self.press("Close");

        self.reload_with(
            base,
            json!([health, { "path": "/today", "status": 500, "body": {}, "keep": true }]),
            "Collapse Coda",
        );
        self.press_in_sidebar("Connections");
        self.appears("dialog[open]");
        self.stage("10b-connections-failed");
        self.press("Close");
        self.run("forgetAnswers", &[]);
        self.open(&format!("{base}/"), "Collapse Coda");
    }

    /// Find patient: nothing typed, loading, results, no match, the failure and its way back.
    fn walk_the_search(&mut self) {
        self.press_in_sidebar("Find patient");
        self.appears(SEARCH_FIELD);
        self.stage("05-search-open");

        self.answer(json!({ "path": "/patient-roster", "hold": true }));
        self.browser.type_into(SEARCH_FIELD, "Jake");
        self.browser.wait_for("Searching…");
        self.stage("06-search-loading");
        self.run("release", &[]);
        self.appears(FIRST_RESULT);
        self.browser.wait_for("DOB");
        self.stage("07-search-results");

        self.browser.type_into(SEARCH_FIELD, "zzz");
        self.browser.wait_for("No patients match.");
        self.stage("08-search-none");

        self.answer(json!({ "path": "/patient-roster", "status": 500, "body": {} }));
        self.browser.type_into(SEARCH_FIELD, "q");
        self.browser.wait_for("Try again");
        self.stage("09-search-failed");
        self.press("Try again");
        let recovered = self.browser.wait_for("No patients match.");
        assert!(
            !recovered.contains("Try again"),
            "the failure outlived its answer:\n{recovered}"
        );
        self.press("Close");
    }

    fn walk_the_dialogs(&mut self) {
        self.press_in_sidebar("Keyboard shortcuts");
        self.browser.wait_for("Navigation shortcuts pause");
        self.stage("10-shortcuts");
        self.press("Close");
        self.browser.wait_for("Collapse Coda");
    }

    /// Opens a patient from Find patient, as a person does.
    fn open_patient(&mut self, query: &str, needle: &str) {
        self.press_in_sidebar("Find patient");
        self.appears(SEARCH_FIELD);
        self.browser.type_into(SEARCH_FIELD, query);
        self.appears(FIRST_RESULT);
        self.press_first(FIRST_RESULT);
        self.browser.wait_for(needle);
    }

    /// Opens the retained patient records, past work and a new conversation.
    fn walk_the_patient(&mut self, base: &str) {
        // The fixture directory has no past work. Supply that display state explicitly.
        let handle = tokio::runtime::Handle::current();
        let (path, body) = tokio::task::block_in_place(|| {
            handle.block_on(async {
                let rows: Value = reqwest::get(format!("{base}/patient-workspaces"))
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                let patient = rows["patients"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|row| row["display_name"].as_str().unwrap().contains("Jake"))
                    .unwrap();
                let path = format!(
                    "/patient-workspaces/{}",
                    patient["patient_workspace_id"].as_str().unwrap()
                );
                let mut body: Value = reqwest::get(format!("{base}{path}"))
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                body["has_past_work"] = json!(true);
                (path, body)
            })
        });
        self.answer(json!({"path": path, "status": 200, "body": body}));
        self.open_patient("Jake", "Records as of");
        self.stage("11-patient-top");

        self.press("Past work");
        self.browser.wait_for("Past work");
        self.stage("16-past-work");
        self.press("Close");

        if self.exists("button.citation-chip") {
            self.press_first("button.citation-chip");
            self.appears(".source-reading-pane");
            self.stage("17-citation");
            self.press("Close");
        }

        self.press_in_sidebar("New conversation");
        self.appears(".composer");
        self.stage_at(".composer", "18-composer");
    }

    /// The clinic record: its card, the picker, a failed assignment and the way back. A failed answer
    /// names the physician in its address, so it also proves which one the picker chose.
    fn walk_the_clinic_record(&mut self, base: &str) {
        self.open(base, "Sign in to Coda");
        self.browser
            .type_into(r#"input[type="email"]"#, "admin@clinic-a.test");
        self.press("Send sign-in link");
        self.browser.wait_for("Collapse Coda");
        self.open_patient("Mary", "Patient demographics");
        self.stage("20-clinic-record");

        assert_eq!(
            self.run("chosen", &[])
                .as_str()
                .map(|words| words.contains("Dr. Clinic Admin")),
            Some(true),
            "the picker didn't start on the signed-in physician"
        );
        self.run("choose", &[json!("Dr. Colleague")]);
        std::thread::sleep(Duration::from_millis(200));
        assert_eq!(
            self.run("chosen", &[])
                .as_str()
                .map(|words| words.contains("Dr. Colleague")),
            Some(true),
            "the picker didn't keep the choice"
        );
        self.answer(json!({ "path": "/auth/panels/phys-colleague", "status": 500, "body": {} }));
        self.press("Assign to panel");
        self.browser
            .wait_for("Coda couldn’t update panel assignment.");
        self.stage_at(".assign-panel-button", "21-clinic-assign-failed");

        self.run("choose", &[json!("Dr. Clinic Admin")]);
        self.press("Assign to panel");
        self.browser.wait_for("Records as of");
        self.press("Remove from panel");
        self.browser.wait_for("Patient demographics");
    }
}

fn calendar() -> Value {
    serde_json::from_str(include_str!("../fixtures/practice-day.json")).unwrap()
}

fn each_way(mut walk: impl FnMut((&'static str, (u32, u32)), &'static str)) {
    for size in [("desktop", DESKTOP), ("phone", PHONE)] {
        for theme in ["light", "dark"] {
            walk(size, theme);
        }
    }
}

async fn spawn_fixture() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, physician_sample::fixture_site())
            .await
            .unwrap();
    });
    format!("http://{address}")
}

async fn spawn_two_physician_clinic() -> String {
    let store = Arc::new(MemoryAuthStore::default());
    store.seed_account(
        "practice-clinic-a",
        "phys-admin",
        "admin@clinic-a.test",
        "Dr. Clinic Admin",
        true,
    );
    store.seed_account(
        "practice-clinic-a",
        "phys-colleague",
        "colleague@clinic-a.test",
        "Dr. Colleague",
        false,
    );
    store
        .register_patient("practice-clinic-a", MARY)
        .await
        .unwrap();
    let admin = PhysicianIdentity {
        practice_id: "practice-clinic-a".to_owned(),
        physician_id: "phys-admin".to_owned(),
    };
    store
        .assign_panel(&admin, "phys-colleague", MARY, Membership::Allowed)
        .await
        .unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let store: Arc<dyn AuthStore> = store;
    tokio::spawn(async move { physician_sample::serve_with_store(listener, store).await });
    format!("http://{address}")
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn every_shell_screen_keeps_its_controls_and_states() {
    let site = shared_site();
    // SAFETY: this binary's one test sets these before it starts either server.
    unsafe {
        std::env::set_var("PHYSICIAN_SITE_DIR", &site);
        std::env::remove_var("PHYSICIAN_AUTH");
        std::env::remove_var("PHYSICIAN_UPSTREAM");
        std::env::remove_var("PHYSICIAN_DATABASE_URL");
        std::env::remove_var("CODA_LOCAL_MODEL_EXECUTION");
        std::env::remove_var("CODA_ALLOW_MODEL_CALLS");
    }

    each_way(|size, theme| {
        let handle = tokio::runtime::Handle::current();
        // Each layout starts with its own fixture server.
        let base = tokio::task::block_in_place(|| handle.block_on(spawn_fixture()));
        std::thread::sleep(Duration::from_millis(200));
        let mut walk = Walk::launch(size, theme);
        walk.open(&format!("{base}/"), "Collapse Coda");
        walk.reload_with(
            &base,
            json!([{ "path": "/practice-day", "status": 200,
            "keep": true, "body": calendar() }]),
            "3:30 PM",
        );
        walk.stage("01-today");
        walk.walk_the_frame();
        walk.walk_the_schedule(&base);
        walk.walk_the_connections(&base);
        walk.walk_the_search();
        walk.walk_the_dialogs();
        walk.walk_the_patient(&base);
    });

    // SAFETY: as above; the fixture servers read theirs when they started.
    unsafe {
        std::env::set_var("PHYSICIAN_AUTH", "required");
        std::env::set_var("PHYSICIAN_AUTH_PROOF_TOKENS", "1");
    }
    let clinic = spawn_two_physician_clinic().await;
    each_way(|size, theme| {
        let mut walk = Walk::launch(size, theme);
        walk.walk_the_clinic_record(&clinic);
    });
}
