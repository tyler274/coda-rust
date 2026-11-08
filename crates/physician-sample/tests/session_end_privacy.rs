//! After the session ends, the page holds no patient content and the browser
//! stores nothing that names the physician or holds their unsent text.

use std::time::{Duration, Instant};

use serde_json::{Value, json};

mod common;
#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;
#[allow(dead_code)]
mod required_auth;

use devtools::{Browser, DESKTOP};
use required_auth::{SIGNED_IN, STAGING_EMAIL, spawn_required_server};

const PHYSICIAN_ID: &str = "staging-physician";
const PATIENT: &str = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";

fn step(browser: &mut Browser, name: &str, args: Value) -> Value {
    let mut arguments = vec![json!(name)];
    arguments.extend(args.as_array().expect("a step's arguments").iter().cloned());
    browser
        .page()
        .call_function(include_str!("js/session_prompt.js"), &arguments)
}

fn wait_until(browser: &mut Browser, what: &str, mut done: impl FnMut(&mut Browser) -> bool) {
    let deadline = Instant::now() + Duration::from_secs(30);
    while Instant::now() < deadline {
        if done(browser) {
            return;
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    panic!("{what} never happened");
}

fn identifying_keys(browser: &mut Browser) -> Vec<String> {
    serde_json::from_value(step(browser, "identifyingKeys", json!([PHYSICIAN_ID])))
        .expect("a list of keys")
}

/// Signs in, opens a patient's conversation, types a draft and collapses the
/// navigation, so the browser holds a draft and a panel choice.
fn leave_work_in_the_browser(browser: &mut Browser, base: &str) {
    browser.text_until(base, "Sign in to Coda");
    browser.type_into(r#"input[type="email"]"#, STAGING_EMAIL);
    browser.click(".control-primary");
    browser.wait_for(SIGNED_IN);
    browser
        .page()
        .navigate(&format!("{base}/?patient={PATIENT}&chat=chat-privacy"));
    browser.wait_for("What would you like Coda to review?");
    browser.type_into("textarea.composer-input", "Unsent note about Mary");
    browser.click(r#"button[aria-label="Collapse navigation"]"#);
    wait_until(browser, "the draft and panel choice being stored", |b| {
        let keys = identifying_keys(b);
        keys.iter().any(|k| k.starts_with("coda.physician.draft."))
            && keys.iter().any(|k| k.ends_with("-collapsed"))
    });
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "runs the physician page in Chromium"]
async fn session_end_clears_patient_content() {
    let server = spawn_required_server().await;
    let base = format!("http://{}", server.address);
    let mut browser = Browser::launch();
    browser.resize(DESKTOP);
    leave_work_in_the_browser(&mut browser, &base);

    step(&mut browser, "endSession", json!([]));
    wait_until(&mut browser, "the prompt replacing the workspace", |b| {
        step(b, "promptReplacesWorkspace", json!([])) == json!(true)
    });
    let text = step(&mut browser, "text", json!([]));
    let text = text.as_str().unwrap_or_default();
    assert!(
        !text.contains("Mary Louis") && !text.contains("Unsent note about Mary"),
        "patient content stayed in the page after the session ended:\n{text}"
    );
    let keys = identifying_keys(&mut browser);
    assert!(
        keys.is_empty(),
        "the prompt left identifying keys stored: {keys:?}"
    );

    browser.click(".session-prompt .control-primary");
    let text = browser.wait_for(SIGNED_IN);
    assert!(
        !text.contains("Unsent note about Mary"),
        "signing in again brought back the draft:\n{text}"
    );
}

/// A tab opened after the session ended never learns who signed in, and
/// still clears what the last physician left stored.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "runs the physician page in Chromium"]
async fn reopening_after_the_session_ended_clears_stored_choices() {
    let server = spawn_required_server().await;
    let base = format!("http://{}", server.address);
    let mut browser = Browser::launch();
    browser.resize(DESKTOP);
    leave_work_in_the_browser(&mut browser, &base);
    let session = step(&mut browser, "storedSession", json!([]))
        .as_str()
        .expect("a stored session")
        .to_owned();

    browser.page().navigate("about:blank");
    let ended = reqwest::Client::new()
        .post(format!("{base}/auth/sign-out"))
        .bearer_auth(&session)
        .send()
        .await
        .expect("sign-out request");
    assert!(
        ended.status().is_success(),
        "sign-out answered {}",
        ended.status()
    );
    browser.text_until(&base, "Sign in");
    let keys = identifying_keys(&mut browser);
    assert!(
        keys.iter().all(|k| !k.ends_with("-collapsed")),
        "reopening after the session ended kept panel choices: {keys:?}"
    );
}
