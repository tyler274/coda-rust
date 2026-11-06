//! After the session ends, the page holds no patient content and the browser
//! stores nothing that names the physician.

use std::time::{Duration, Instant};

mod common;
#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;
mod required_auth;

use devtools::{Browser, DESKTOP};
use required_auth::{END_THE_SESSION, SIGNED_IN, STAGING_EMAIL, spawn_required_server};

fn wait_until(browser: &mut Browser, condition: &str, what: &str) {
    let deadline = Instant::now() + Duration::from_secs(30);
    while Instant::now() < deadline {
        if browser.page().evaluate(condition).as_bool() == Some(true) {
            return;
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    panic!("{what} never happened");
}

fn page_text(browser: &mut Browser) -> String {
    browser
        .page()
        .evaluate("document.body.innerText")
        .as_str()
        .unwrap_or_default()
        .to_owned()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "runs the physician page in Chromium"]
async fn session_end_clears_patient_content() {
    let server = spawn_required_server().await;
    let base = format!("http://{}", server.address);
    let mut browser = Browser::launch();
    browser.resize(DESKTOP);
    browser.text_until(&base, "Sign in to Coda");
    browser.type_into(r#"input[type="email"]"#, STAGING_EMAIL);
    browser.click(".control-primary");
    browser.wait_for(SIGNED_IN);
    browser.click(r#"button[aria-label="Find patient"]"#);
    browser.type_into(".search-field input", "Mary");
    browser.wait_for("DOB");
    browser.click(".dialog-results button");
    browser.wait_for("Records as of");
    browser.click(r#"button[aria-label="Collapse navigation"]"#);

    browser.page().evaluate(END_THE_SESSION);
    wait_until(
        &mut browser,
        "!!document.querySelector('.session-prompt[open]')",
        "the sign-in prompt",
    );
    let text = page_text(&mut browser);
    assert!(
        !text.contains("Mary Louis") && !text.contains("Age 54"),
        "patient content stayed in the page after the session ended:\n{text}"
    );
    assert!(
        text.contains("Sign in to continue"),
        "the prompt did not replace the workspace:\n{text}"
    );

    browser.page().evaluate(
        "[...document.querySelectorAll('.session-prompt button')].find((b) => b.innerText.trim() === 'Sign out')?.click(); 'x'",
    );
    browser.wait_for("Sign in to Coda");
    let storage = browser
        .page()
        .evaluate("JSON.stringify([...Object.keys(localStorage), ...Object.keys(sessionStorage)])");
    let keys = storage.as_str().unwrap_or_default();
    assert!(
        !keys.contains("staging-physician") && !keys.contains("sidebar-collapsed"),
        "a physician-identifying key survived sign-out: {keys}"
    );
}
