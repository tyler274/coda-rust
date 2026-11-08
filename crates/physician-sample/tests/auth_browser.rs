//! Suite 1: the authenticated physician journeys in a real browser.
//!
//! `PHYSICIAN_AUTH=required` on the recorded practice: an unauthenticated page
//! shows the email form, local proof delivery automatically redeems its token,
//! Today and a panel workspace render, sign-out returns to the sign-in screen,
//! and the steps hold at desktop, phone, and reduced-motion viewports.
//! Book: `book/src/physician-site.md`.

use std::time::{Duration, Instant};

use serde_json::json;

mod common;
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;
mod required_auth;

use devtools::{Browser, DESKTOP, PHONE};
use required_auth::{END_THE_SESSION, SIGNED_IN, STAGING_EMAIL, spawn_required_server};

/// What a tab tells the others when the browser's session changes. It names no
/// session: each tab reads what the browser stores.
const TELL_THE_OTHER_TABS: &str = "new BroadcastChannel('coda.physician.session')\
     .postMessage(JSON.stringify({ from: 'another-tab', kind: 'changed' }))";

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn sign_in_and_sign_out_walk_the_browser() {
    let server = spawn_required_server().await;
    let base = format!("http://{}", server.address);

    let mut browser = Browser::launch();

    // Unauthenticated: the sign-in screen, never physician data.
    let text = browser.text_until(&base, "Sign in to Coda");
    assert!(
        !text.contains("Mary Louis") && !text.contains("Find patient"),
        "the signed-out page leaked physician data:\n{text}"
    );

    // The email stage: type the address and send the link.
    browser.resize(DESKTOP);
    browser.type_into(r#"input[type="email"]"#, STAGING_EMAIL);
    browser.click(".control-primary");
    let text = browser.wait_for(SIGNED_IN);
    assert!(
        text.contains("Today"),
        "a signed-in Today did not show the practice:\n{text}"
    );

    // The panel workspace opens inside the running app: Find patient keeps
    // the bearer session in the page, so the workspace reads in full.
    let text = open_mary(&mut browser);
    assert!(
        text.contains("Age 54 years"),
        "the panel workspace did not read in full:\n{text}"
    );

    // Sign out: the app returns to the sign-in screen immediately.
    browser.click(r#"button[aria-label="Sign out of Coda"]"#);
    let text = browser.wait_for("Sign in to Coda");
    assert!(
        !text.contains("Mary Louis"),
        "the signed-out page still showed the practice:\n{text}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn the_sign_in_screen_holds_the_phone_and_reduced_motion_viewports() {
    let server = spawn_required_server().await;
    let base = format!("http://{}", server.address);

    let mut browser = Browser::launch();

    // Phone: the sign-in card stays visible and does not scroll sideways.
    browser.resize(PHONE);
    let text = browser.text_until(&base, "Sign in to Coda");
    assert!(
        text.contains("Work email"),
        "the phone viewport lost the sign-in card:\n{text}"
    );
    let overflow: f64 = serde_json::from_value(browser.page().evaluate(
        "Math.max(0, document.documentElement.scrollWidth \
             - document.documentElement.clientWidth)",
    ))
    .expect("overflow number");
    assert!(
        overflow <= 4.0,
        "the phone viewport scrolled horizontally by {overflow}px:\n{text}"
    );

    // Reduced motion: the document reports the emulated preference.
    browser.emulate_reduced_motion();
    assert!(
        browser.prefers_reduced_motion(),
        "the emulated reduced-motion preference did not reach the page"
    );
    browser.resize(DESKTOP);
    assert!(
        browser.prefers_reduced_motion(),
        "reduced motion did not survive a resize"
    );
}

/// Waits until `condition`, a JavaScript expression, is true on the page.
fn wait_until(browser: &mut Browser, condition: &str, what: &str) {
    let deadline = Instant::now() + Duration::from_secs(30);
    while Instant::now() < deadline {
        if browser.page().evaluate(condition).as_bool() == Some(true) {
            return;
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    panic!(
        "{what} never happened:\n{}",
        browser.body_text().unwrap_or_default()
    );
}

/// What every patient's page says.
const PATIENT_PAGE: &str = "Records as of";

/// A session-end check from `js/session_prompt.js`.
#[derive(Clone, Copy)]
enum PromptCheck {
    /// The prompt stands in place of the workspace and its patient.
    Replaced,
    /// The prompt is gone and the workspace shows Today.
    BackAtToday,
}

/// Waits until `check` holds on the page.
fn wait_for_prompt(browser: &mut Browser, check: PromptCheck, what: &str) {
    let arguments = match check {
        PromptCheck::Replaced => vec![json!("promptReplacesWorkspace")],
        PromptCheck::BackAtToday => vec![json!("backAtToday"), json!(SIGNED_IN)],
    };
    let deadline = Instant::now() + Duration::from_secs(30);
    while Instant::now() < deadline {
        let holds = browser
            .page()
            .call_function(include_str!("js/session_prompt.js"), &arguments);
        if holds == json!(true) {
            return;
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    panic!(
        "{what} never happened:\n{}",
        browser.body_text().unwrap_or_default()
    );
}

/// Another tab redeems a link: a fresh session for the staging account,
/// stored with `owner` as its owner. `then` runs once it is stored.
fn another_tab_stores_a_session(owner: &str, then: &str) -> String {
    format!(
        "(async () => {{ \
           const post = (path, body) => fetch(path, {{ method: 'POST', \
             headers: {{ 'content-type': 'application/json' }}, body: JSON.stringify(body) }}) \
             .then(r => r.json()); \
           const issued = await post('/auth/sign-in', {{ email: '{STAGING_EMAIL}' }}); \
           const verified = await post('/auth/verify', \
             {{ email: '{STAGING_EMAIL}', token: issued.token }}); \
           window.__minted = verified.session; \
           localStorage.setItem('coda.physician.session', verified.session); \
           localStorage.setItem('coda.physician.session.email', '{owner}'); \
           {then}; }})(); 'sent'"
    )
}

/// Another tab signs `owner` in and tells this one.
fn another_tab_signs_in(owner: &str) -> String {
    another_tab_stores_a_session(owner, TELL_THE_OTHER_TABS)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn an_ended_session_replaces_the_page_and_signing_in_again_opens_today() {
    let server = spawn_required_server().await;
    let base = format!("http://{}", server.address);
    let mut browser = Browser::launch();
    browser.resize(DESKTOP);
    browser.text_until(&base, "Sign in to Coda");
    // Other tabs hear this tab's sign-in.
    browser.page().evaluate(
        "window.__news = []; new BroadcastChannel('coda.physician.session').onmessage = \
         event => window.__news.push(JSON.parse(event.data)); 'listening'",
    );
    browser.type_into(r#"input[type="email"]"#, STAGING_EMAIL);
    browser.click(".control-primary");
    browser.wait_for(SIGNED_IN);
    wait_until(
        &mut browser,
        "window.__news.some((news) => news.kind === 'changed')",
        "the sign-in reaching other tabs",
    );
    open_mary(&mut browser);

    browser.page().evaluate(END_THE_SESSION);
    wait_for_prompt(
        &mut browser,
        PromptCheck::Replaced,
        "the sign-in prompt replacing the page",
    );
    let text = browser.wait_for("Sign in to continue");
    assert!(
        text.contains(STAGING_EMAIL) && text.contains("Email me a sign-in link"),
        "the prompt did not offer a link to the same address:\n{text}"
    );
    // Escape never dismisses the prompt, even pressed again without other input.
    for _ in 0..3 {
        for kind in ["keyDown", "keyUp"] {
            browser.page().call_session(
                "Input.dispatchKeyEvent",
                serde_json::json!({ "type": kind, "key": "Escape", "code": "Escape", "windowsVirtualKeyCode": 27 }),
            );
        }
    }
    wait_for_prompt(
        &mut browser,
        PromptCheck::Replaced,
        "the prompt still up after Escape",
    );

    browser.click(".session-prompt .control-primary");
    wait_for_prompt(
        &mut browser,
        PromptCheck::BackAtToday,
        "signing in again opening Today",
    );

    open_mary(&mut browser);
    browser.page().evaluate(END_THE_SESSION);
    wait_for_prompt(
        &mut browser,
        PromptCheck::Replaced,
        "the prompt replacing the page again",
    );
    browser
        .page()
        .evaluate(&another_tab_signs_in(STAGING_EMAIL));
    wait_for_prompt(
        &mut browser,
        PromptCheck::BackAtToday,
        "another tab's link opening Today",
    );

    open_mary(&mut browser);
    browser.page().evaluate(END_THE_SESSION);
    wait_for_prompt(
        &mut browser,
        PromptCheck::Replaced,
        "the prompt before another tab signs out",
    );

    // Another tab's sign-out reaches this one.
    browser.page().evaluate(&format!(
        "{END_THE_SESSION}; localStorage.removeItem('coda.physician.session'); \
         localStorage.removeItem('coda.physician.session.email'); {TELL_THE_OTHER_TABS}; 'sent'"
    ));
    let text = browser.wait_for("Sign in to Coda");
    assert!(
        !text.contains("Records as of"),
        "signing out elsewhere left the patient showing:\n{text}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn a_sign_out_answered_after_you_signed_in_again_elsewhere_keeps_the_new_session() {
    let server = spawn_required_server().await;
    let base = format!("http://{}", server.address);
    let mut browser = Browser::launch();
    browser.resize(DESKTOP);
    sign_in(&mut browser, &base);
    browser.wait_for("Staging Physician");
    let first = stored_session(&mut browser).expect("signed in");
    // Hold this tab's sign-out request.
    browser.page().evaluate(
        "const send = window.fetch.bind(window); \
         window.fetch = (input, init) => \
           String(input.url || input).includes('/auth/sign-out') \
             ? new Promise((resolve) => { window.__release = () => resolve(send(input, init)); }) \
             : send(input, init); 'held'",
    );
    browser.click(r#"button[aria-label="Sign out of Coda"]"#);
    wait_until(&mut browser, "!!window.__release", "the sign-out request");
    // Meanwhile you sign in again in another tab.
    browser
        .page()
        .evaluate(&another_tab_signs_in(STAGING_EMAIL));
    wait_until(
        &mut browser,
        "!!window.__minted && localStorage.getItem('coda.physician.session') === window.__minted",
        "the new sign-in",
    );
    let renewed = stored_session(&mut browser).expect("renewed session");
    assert_ne!(renewed, first);
    std::thread::sleep(Duration::from_millis(500));
    browser.page().evaluate("window.__release(); 'released'");
    std::thread::sleep(Duration::from_secs(1));
    assert_eq!(stored_session(&mut browser), Some(renewed.clone()));
    assert_eq!(whoami_status(&mut browser, &renewed), 200);
    assert!(
        browser
            .body_text()
            .is_some_and(|text| !text.contains("Sign in to Coda") && text.contains(SIGNED_IN)),
        "the late sign-out undid the new sign-in"
    );
}

fn sign_in(browser: &mut Browser, base: &str) {
    browser.text_until(base, "Sign in to Coda");
    browser.type_into(r#"input[type="email"]"#, STAGING_EMAIL);
    browser.click(".control-primary");
    browser.wait_for(SIGNED_IN);
}

/// Finds Mary Louis from the open app; returns her page's text.
fn open_mary(browser: &mut Browser) -> String {
    browser.click(r#"button[aria-label="Find patient"]"#);
    browser.type_into(".search-field input", "Mary");
    browser.wait_for("DOB");
    browser.click(".dialog-results button");
    browser.wait_for(PATIENT_PAGE)
}

/// The session this browser holds.
fn stored_session(browser: &mut Browser) -> Option<String> {
    browser
        .page()
        .evaluate("localStorage.getItem('coda.physician.session')")
        .as_str()
        .map(str::to_owned)
}

/// What the server answers `session` now.
fn whoami_status(browser: &mut Browser, session: &str) -> u64 {
    browser.page().evaluate(&format!(
        "window.__whoami = 0; fetch('/auth/whoami', {{ headers: {{ Authorization: 'Bearer {session}' }} }}) \
           .then(r => {{ window.__whoami = r.status; }}); 'sent'"
    ));
    wait_until(browser, "window.__whoami > 0", "the server's answer");
    browser
        .page()
        .evaluate("window.__whoami")
        .as_u64()
        .expect("status")
}
