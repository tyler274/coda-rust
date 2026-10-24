//! Suite 1: the authenticated physician journeys in a real browser.
//!
//! `PHYSICIAN_AUTH=required` on the recorded practice: an unauthenticated page
//! shows the email form, local proof delivery automatically redeems its token,
//! Today and a panel workspace render, sign-out returns to the sign-in screen,
//! and the steps hold at desktop, phone, and reduced-motion viewports.
//! Book: `book/src/physician-site.md`.

use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

mod common;
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;

use devtools::{Browser, DESKTOP, PHONE};

const STAGING_EMAIL: &str = "physician@codahealth.test";

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
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
    browser.type_into(".sign-in-input", STAGING_EMAIL);
    browser.click(".sign-in-primary");
    let text = browser.wait_for("Mary Louis");
    assert!(
        text.contains("Today"),
        "a signed-in Today did not show the practice:\n{text}"
    );

    // The panel workspace opens inside the running app: clicking the Today
    // schedule visit keeps the in-memory bearer session, so the workspace
    // reads clinically. A full navigation would reload the app and drop it.
    browser.click(r#"button[aria-label="Open Mary Louis"]"#);
    let text = browser.wait_for("Patient summary");
    assert!(
        text.contains("Age 54 years"),
        "the panel workspace did not read clinically:\n{text}"
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

/// Ends the page's session on the server, as expiry does; the page is not told.
const END_THE_SESSION: &str = "fetch('/auth/sign-out', { method: 'POST', headers: { \
     Authorization: 'Bearer ' + sessionStorage.getItem('coda.physician.session') } }); 'sent'";

/// The prompt covers the page, hides it from assistive technology, and takes
/// it out of the tab order.
const COVERED: &str = "(() => { const shell = document.querySelector('.glass-app'); \
     const prompt = document.querySelector('.session-prompt'); \
     const top = document.elementFromPoint(innerWidth / 2, innerHeight / 2); \
     return !!prompt && !!shell && prompt.contains(top) \
         && shell.getAttribute('aria-hidden') === 'true' && shell.hasAttribute('inert'); })()";

const RESUMED_ON_THE_PATIENT: &str = "!document.querySelector('.session-prompt') \
     && !document.querySelector('.glass-app').hasAttribute('inert') \
     && document.body.innerText.includes('Patient summary')";

/// What another tab sends after redeeming its link: a fresh session for
/// `physician`, minted here through the same proof sign-in.
fn another_tab_signs_in(physician: &str) -> String {
    format!(
        "(async () => {{ \
           const post = (path, body) => fetch(path, {{ method: 'POST', \
             headers: {{ 'content-type': 'application/json' }}, body: JSON.stringify(body) }}) \
             .then(r => r.json()); \
           const issued = await post('/auth/sign-in', {{ email: '{STAGING_EMAIL}' }}); \
           const verified = await post('/auth/verify', \
             {{ email: '{STAGING_EMAIL}', token: issued.token }}); \
           new BroadcastChannel('coda.physician.session').postMessage(JSON.stringify({{ \
             from: 'another-tab', kind: 'signed_in', \
             physician_id: {physician}, session: verified.session }})); }})(); 'sent'"
    )
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn an_ended_session_covers_the_page_and_signing_in_again_resumes_it() {
    let server = spawn_required_server().await;
    let base = format!("http://{}", server.address);
    let mut browser = Browser::launch();
    browser.resize(DESKTOP);
    browser.text_until(&base, "Sign in to Coda");
    // Other tabs hear this tab's sign-in.
    browser.page().evaluate(
        "window.__news = []; new BroadcastChannel('coda.physician.session').onmessage = \
         event => window.__news.push(JSON.parse(event.data).kind); 'listening'",
    );
    browser.type_into(".sign-in-input", STAGING_EMAIL);
    browser.click(".sign-in-primary");
    browser.wait_for("Mary Louis");
    wait_until(
        &mut browser,
        "window.__news.includes('signed_in')",
        "the sign-in reaching other tabs",
    );
    browser.click(r#"button[aria-label="Open Mary Louis"]"#);
    browser.wait_for("Patient summary");

    // The session ends: the prompt covers the patient page, which stays put.
    browser.page().evaluate(END_THE_SESSION);
    wait_until(
        &mut browser,
        COVERED,
        "the sign-in prompt covering the page",
    );
    let text = browser.wait_for("Sign in to continue");
    assert!(
        text.contains(STAGING_EMAIL) && text.contains("Email me a sign-in link"),
        "the prompt did not offer a link to the same address:\n{text}"
    );

    // Signing in under the prompt picks up on the same patient.
    browser.click(".session-prompt .sign-in-primary");
    wait_until(
        &mut browser,
        RESUMED_ON_THE_PATIENT,
        "resuming on the patient",
    );

    // Another tab's link resumes a covered page the same way.
    browser.page().evaluate(END_THE_SESSION);
    wait_until(&mut browser, COVERED, "the prompt covering the page again");
    browser
        .page()
        .evaluate(&another_tab_signs_in("'staging-physician'"));
    wait_until(
        &mut browser,
        RESUMED_ON_THE_PATIENT,
        "resuming on another tab's link",
    );

    // Another tab's sign-out reaches this one.
    browser.page().evaluate(
        "new BroadcastChannel('coda.physician.session').postMessage(JSON.stringify(\
         { from: 'another-tab', kind: 'signed_out' })); 'sent'",
    );
    let text = browser.wait_for("Sign in to Coda");
    assert!(
        !text.contains("Patient summary"),
        "signing out elsewhere left the patient showing:\n{text}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn another_physicians_sign_in_starts_this_tab_over_at_today() {
    let server = spawn_required_server().await;
    let base = format!("http://{}", server.address);
    let mut browser = Browser::launch();
    browser.resize(DESKTOP);
    browser.text_until(&base, "Sign in to Coda");
    browser.type_into(".sign-in-input", STAGING_EMAIL);
    browser.click(".sign-in-primary");
    browser.wait_for("Mary Louis");
    browser.click(r#"button[aria-label="Open Mary Louis"]"#);
    browser.wait_for("Patient summary");
    // The news names someone else; the tab ends its own session and starts
    // over at Today on the session it was handed.
    browser
        .page()
        .evaluate(&another_tab_signs_in("'another-physician'"));
    wait_until(
        &mut browser,
        "!document.body.innerText.includes('Patient summary') \
         && document.body.innerText.includes('Mary Louis')",
        "starting over at Today",
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_link_that_no_longer_grants_access_opens_no_access() {
    let server = spawn_required_server().await;
    // The server's refusal is proved in its own tests; this stands in for it so
    // the page's answer is exercised in the browser.
    let refusing = refusing_verifier().await;
    let mut browser = Browser::launch();
    browser.resize(DESKTOP);
    let url = format!(
        "http://{}/?api=http://{refusing}&email={STAGING_EMAIL}&code=codas-held",
        server.address
    );
    let text = browser.text_until(&url, "No access to this practice");
    assert!(
        text.contains("no longer gives you access to this practice")
            && !text.contains("Mary Louis"),
        "the no-access page was wrong:\n{text}"
    );
    browser.click(".sign-in-text-action");
    browser.wait_for("Work email");
}

/// Answers `/auth/verify` the way the server answers a held link whose account
/// lost access, with the CORS a cross-port `?api=` page needs.
async fn refusing_verifier() -> std::net::SocketAddr {
    use axum::http::{StatusCode, header};
    use axum::response::IntoResponse;
    let cors = || {
        [
            (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
            (header::ACCESS_CONTROL_ALLOW_HEADERS, "*"),
            (header::ACCESS_CONTROL_ALLOW_METHODS, "*"),
        ]
    };
    let app = axum::Router::new().route(
        "/auth/verify",
        axum::routing::post(move || async move {
            (
                StatusCode::FORBIDDEN,
                cors(),
                axum::Json(serde_json::json!({
                    "detail": "This sign-in link no longer gives access to this practice.",
                    "reason": "no_access",
                })),
            )
                .into_response()
        })
        .options(move || async move { (StatusCode::NO_CONTENT, cors()).into_response() }),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    address
}

/// Boots the recorded practice with `PHYSICIAN_AUTH=required`. The server
/// seeds its staging account and panel itself on the memory store. Its
/// physician session is a temporary file and its execution is offline replay.
async fn spawn_required_server() -> common::IsolatedServer {
    let site = shared_site();
    common::serve_isolated(|| {
        // SAFETY: `serve_isolated` holds its start lock while this runs.
        unsafe {
            std::env::set_var("PHYSICIAN_SITE_DIR", &site);
            std::env::set_var("PHYSICIAN_AUTH", "required");
            std::env::remove_var("PHYSICIAN_DATABASE_URL");
            std::env::remove_var("PHYSICIAN_UPSTREAM");
        }
    })
    .await
}

/// Builds the WASM site once for this test binary; the two browser tests
/// would otherwise race two cargo builds over one output directory.
fn shared_site() -> PathBuf {
    static SITE: OnceLock<PathBuf> = OnceLock::new();
    SITE.get_or_init(build_site).clone()
}

fn build_site() -> PathBuf {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let target = std::env::temp_dir().join("coda-physician-wasm");
    let wasm = target.join("wasm32-unknown-unknown/debug/physician-app.wasm");
    let mut cargo =
        std::process::Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
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
        .expect("cargo build");
    assert!(status.success(), "physician-app wasm build failed");
    std::fs::create_dir_all(workspace.join("target/physician-web")).unwrap();
    let status = std::process::Command::new(wasm_bindgen())
        .args([
            "--target",
            "web",
            "--out-dir",
            workspace.join("target/physician-web").to_str().unwrap(),
            wasm.to_str().unwrap(),
        ])
        .status()
        .expect("wasm-bindgen");
    assert!(status.success(), "wasm-bindgen failed");
    workspace.join("target/physician-web")
}

fn cargo_without_parent_package(command: &mut std::process::Command) {
    command.env_clear();
    // LD_LIBRARY_PATH rides along: the tools on PATH (for example the lld
    // that links wasm) resolve their own shared libraries through it.
    for key in [
        "PATH",
        "HOME",
        "USER",
        "RUSTUP_HOME",
        "CARGO_HOME",
        "RUSTUP_TOOLCHAIN",
        "TMPDIR",
        "LD_LIBRARY_PATH",
    ] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
}

fn wasm_bindgen() -> PathBuf {
    if let Some(path) = std::env::var_os("WASM_BINDGEN") {
        return PathBuf::from(path);
    }
    if let Some(path) = find_on_path("wasm-bindgen") {
        return path;
    }
    let local =
        std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/bin/wasm-bindgen"));
    if let Some(path) = local.filter(|path| path.is_file()) {
        return path;
    }
    panic!("wasm-bindgen 0.2.129 is required to build the physician page");
}

fn find_on_path(name: &str) -> Option<PathBuf> {
    let paths = std::env::var_os("PATH")?;
    std::env::split_paths(&paths)
        .map(|dir| dir.join(name))
        .find(|path| path.is_file())
}
