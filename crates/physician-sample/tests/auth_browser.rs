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
/// A stored owner other than the signed-in physician: the browser now holds
/// another physician's session.
const ANOTHER_EMAIL: &str = "another.physician@codahealth.test";

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

/// Ends the page's session on the server, as expiry does; the page is not told.
const END_THE_SESSION: &str = "fetch('/auth/sign-out', { method: 'POST', headers: { \
     Authorization: 'Bearer ' + localStorage.getItem('coda.physician.session') } }); 'sent'";

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
async fn an_ended_session_covers_the_page_and_signing_in_again_resumes_it() {
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
    browser.type_into(".sign-in-input", STAGING_EMAIL);
    browser.click(".sign-in-primary");
    browser.wait_for("Mary Louis");
    wait_until(
        &mut browser,
        "window.__news.some((news) => news.kind === 'changed')",
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
        .evaluate(&another_tab_signs_in(STAGING_EMAIL));
    wait_until(
        &mut browser,
        RESUMED_ON_THE_PATIENT,
        "resuming on another tab's link",
    );

    // Another tab's sign-out reaches this one.
    browser.page().evaluate(&format!(
        "{END_THE_SESSION}; localStorage.removeItem('coda.physician.session'); \
         localStorage.removeItem('coda.physician.session.email'); {TELL_THE_OTHER_TABS}; 'sent'"
    ));
    let text = browser.wait_for("Sign in to Coda");
    assert!(
        !text.contains("Patient summary"),
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
            .is_some_and(|text| !text.contains("Sign in to Coda") && text.contains("Mary Louis")),
        "the late sign-out undid the new sign-in"
    );
}

fn sign_in(browser: &mut Browser, base: &str) {
    browser.text_until(base, "Sign in to Coda");
    browser.type_into(".sign-in-input", STAGING_EMAIL);
    browser.click(".sign-in-primary");
    browser.wait_for("Mary Louis");
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

// The sign-in model's walks, played in two tabs of one browser.
//
// `verification/statechart.mjs walks` covers every case of every event in the
// model, from both tabs closed. Each step does what the event names: you open,
// click and sign out in the real page; Coda's answers wait in the page until
// the model says Coda answers, and another tab's news waits until the tab
// hears it. After every step both tabs must look as the model says: what each
// shows, what it still waits on from Coda, whether news is on its way, and the
// session the browser stores. The driver answers for what the sample server
// can't be: another physician (a real session whose whoami names someone
// else), a practice that removed your access, a used link, and Coda out of
// reach. Sessions expire by ending them on the server.

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn every_walk_of_the_sign_in_model_holds_in_two_tabs() {
    let server = spawn_required_server().await;
    // The page reads on its own, so a walk can't make it read.
    play(
        server.address,
        &model_walks(&["--only", "^(?!the workspace reads)"]),
    );
}

/// Paths on which the code once broke a rule the model holds: a tab opened at
/// a patient link while signed out, following another tab's sign-in, still
/// lands on the patient; a sign-in page that followed another tab's sign-in
/// and then signed out doesn't redeem its link again; and that opening tab,
/// when a sign-out came between, opens Today.
const BUG_PATHS: &str = r#"[
  ["open a patient link in tab A", "open your sign-in link in tab B", "Coda signs in the link in tab B",
   "this tab hears the news in tab A", "Coda answers the read in tab A"],
  ["open your sign-in link in tab A", "Coda can't reach sign-in in tab A",
   "open another physician's sign-in link in tab B", "Coda signs in the link in tab B",
   "this tab hears the news in tab A", "sign out in tab A"],
  ["open a patient link in tab A", "open your sign-in link in tab B", "Coda signs in the link in tab B",
   "Coda answers who signed in in tab B", "sign out in tab B", "open your sign-in link in tab B",
   "Coda signs in the link in tab B", "this tab hears the news in tab A", "Coda answers the read in tab A"]
]"#;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn the_sign_in_model_s_bug_paths_hold_in_two_tabs() {
    let server = spawn_required_server().await;
    play(server.address, &model_walks(&["--paths", BUG_PATHS]));
}

/// Plays the walks in browsers side by side and fails with every walk that
/// strays from the model.
fn play(address: std::net::SocketAddr, walks: &Walks) {
    // SIGN_IN_WALKS picks walks by index ("3,17"); SIGN_IN_WALKERS runs browsers in parallel.
    let picked: Vec<usize> = match std::env::var("SIGN_IN_WALKS") {
        Ok(list) => list
            .split(',')
            .map(str::trim)
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().expect("walk index"))
            .collect(),
        Err(_) => (0..walks.len()).collect(),
    };
    let walkers: usize = std::env::var("SIGN_IN_WALKERS")
        .ok()
        .and_then(|n| n.parse().ok())
        .unwrap_or(3);
    let next = std::sync::atomic::AtomicUsize::new(0);
    let failures = std::sync::Mutex::new(Vec::new());
    tokio::task::block_in_place(|| {
        std::thread::scope(|scope| {
            for _ in 0..walkers {
                scope.spawn(|| {
                    loop {
                        let n = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        let Some(&index) = picked.get(n) else { return };
                        let walk = &walks[index];
                        let outcome =
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                Walker::new(address).walk(&walks.start, walk)
                            }));
                        if let Err(panic) = outcome {
                            let message = panic
                                .downcast_ref::<String>()
                                .cloned()
                                .or_else(|| panic.downcast_ref::<&str>().map(|s| (*s).to_owned()))
                                .unwrap_or_default();
                            failures
                                .lock()
                                .unwrap()
                                .push(format!("walk {index}: {message}"));
                        }
                    }
                });
            }
        });
    });
    let failures = failures.into_inner().unwrap();
    assert!(
        failures.is_empty(),
        "{} of {} walks failed:\n\n{}",
        failures.len(),
        picked.len(),
        failures.join("\n\n")
    );
}

type ModelState = std::collections::BTreeMap<String, String>;

#[derive(serde::Deserialize)]
struct WalkStep {
    event: String,
    state: ModelState,
}

#[derive(serde::Deserialize)]
struct Walks {
    start: ModelState,
    walks: Vec<Vec<WalkStep>>,
}

impl std::ops::Index<usize> for Walks {
    type Output = Vec<WalkStep>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.walks[index]
    }
}

impl Walks {
    fn len(&self) -> usize {
        self.walks.len()
    }
}

fn model_walks(choice: &[&str]) -> Walks {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = std::process::Command::new("node")
        .current_dir(&workspace)
        .args([
            "verification/statechart.mjs",
            "walks",
            "verification/physician/sign-in/sign-in.mjs",
        ])
        .args(choice)
        .output()
        .expect("node runs the model");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("walks")
}

/// Holds Coda's answers and other tabs' news in the page until the driver
/// lets them through, and lets it end the page's event streams.
const HOLD_CODA: &str = r#"(() => {
  const send = window.fetch.bind(window);
  const coda = window.__coda = { held: [], news: [], sent: new Set(), signingOut: 0, streams: [], log: [] };
  addEventListener('error', (e) => coda.log.push('error ' + e.message));
  addEventListener('unhandledrejection', (e) => coda.log.push('rejection ' + e.reason));
  const bearerOf = (input, init) => {
    const headers = new Headers((init && init.headers) || (input instanceof Request ? input.headers : undefined));
    const value = headers.get('authorization');
    return value ? value.replace(/^Bearer\s+/i, '') : null;
  };
  const kindOf = (url) => {
    const path = new URL(url, location.href).pathname;
    if (path === '/auth/verify') return 'verify';
    if (path === '/auth/sign-in') return 'request';
    if (path === '/auth/sign-out') return 'sign-out';
    if (path === '/auth/whoami') return 'whoami';
    if (path === '/patient-workspaces/events') return 'stream';
    if (path === '/today' || /^\/patient-workspaces\/[^/]+$/.test(path)) return 'read';
    return null;
  };
  window.fetch = (input, init) => {
    const url = String(input instanceof Request ? input.url : input);
    const kind = kindOf(url);
    const bearer = bearerOf(input, init);
    if (kind === 'sign-out') {
      coda.signingOut++;
      return send(input, init).finally(() => { coda.signingOut--; });
    }
    if (kind === 'stream') return send(input, init).then((response) => {
      if (!response.ok || !response.body) return response;
      const reader = response.body.getReader();
      let end = () => {};
      const body = new ReadableStream({
        start(controller) { end = () => { try { controller.close(); } catch (_) {} reader.cancel().catch(() => {}); }; },
        pull(controller) {
          return reader.read().then(({ done, value }) => done ? controller.close() : controller.enqueue(value),
            (error) => controller.error(error));
        },
        cancel() { return reader.cancel(); },
      });
      coda.streams.push({ bearer, end });
      return new Response(body, { status: response.status, headers: response.headers });
    });
    if (!kind) return send(input, init);
    return new Promise((resolve, reject) => {
      const held = { kind, bearer, input, init, resolve, reject };
      coda.log.push(`hold ${kind} ${bearer}`);
      coda.held.push(held);
      // A request the page drops is no longer waited on.
      const signal = (init && init.signal) || (input instanceof Request ? input.signal : null);
      if (signal) signal.addEventListener('abort', () => {
        coda.log.push(`abort ${kind}`);
        coda.held = coda.held.filter((h) => h !== held);
        reject(new DOMException('aborted', 'AbortError'));
      });
    });
  };
  coda.release = (kind, bearer, how) => {
    // A link request or verification is answered whatever bearer it carries.
    const now = coda.held.filter((h) => h.kind === kind
      && (kind === 'request' || kind === 'verify' || h.bearer === bearer));
    coda.held = coda.held.filter((h) => !now.includes(h));
    for (const h of now) {
      coda.log.push(`release ${kind} ${bearer} ${JSON.stringify(how).slice(0, 60)}`);
      if (how.fail) h.reject(new TypeError('Failed to fetch'));
      else if (how.status) {
        const response = new Response(JSON.stringify(how.body),
          { status: how.status, headers: { 'content-type': 'application/json' } });
        // reqwest reads the answer's URL.
        Object.defineProperty(response, 'url', { value: new URL(h.input instanceof Request ? h.input.url : String(h.input), location.href).href });
        h.resolve(response);
      }
      else h.resolve(send(h.input, h.init));
    }
    return now.length;
  };
  coda.endStreams = (bearer) => {
    coda.streams.filter((s) => s.bearer === bearer).forEach((s) => s.end());
    coda.streams = coda.streams.filter((s) => s.bearer !== bearer);
  };
  const Channel = window.BroadcastChannel;
  window.BroadcastChannel = class extends Channel {
    postMessage(data) { coda.sent.add(String(data)); return super.postMessage(data); }
    set onmessage(handler) {
      this.handler = handler;
      super.onmessage = handler ? (event) => {
        if (!coda.sent.has(String(event.data))) coda.news.push(() => this.handler && this.handler(event));
      } : null;
    }
    get onmessage() { return this.handler; }
  };
  coda.hear = () => { const now = coda.news; coda.news = []; now.forEach((deliver) => deliver()); return now.length; };
})()"#;

/// What a tab shows and still waits on, in the model's words.
const OBSERVE: &str = r#"(() => {
  const coda = window.__coda;
  if (!coda || location.href === 'about:blank') return { shows: 'Closed', held: [], news: 0, signingOut: 0, patient: false };
  const text = document.body ? document.body.innerText : '';
  const has = (selector) => !!document.querySelector(selector);
  const heading = (document.querySelector('.sign-in-content h1') || {}).innerText || '';
  const shows = has('.session-prompt') ? 'Sign-in prompt over your work'
    : has('.glass-app') ? (text.includes('Loading Today…') || text.includes('Opening the patient…') ? 'Opening your workspace' : 'Your workspace')
    : heading.startsWith('Sign in to Coda') ? 'Enter your work email'
    : heading.startsWith('Check your email') ? 'Check your email'
    : heading.startsWith('Signing you in') ? 'Signing you in'
    : heading.startsWith('No access') ? 'No access to this practice'
    : heading.startsWith('Let') ? (text.includes('no longer valid') ? 'This link no longer works' : "Coda couldn't reach sign-in")
    : 'Something else: ' + heading;
  return { shows, held: coda.held.map((h) => [h.kind, h.bearer]), news: coda.news.length,
    signingOut: coda.signingOut, patient: text.includes('Patient summary') };
})()"#;

/// Whether the page that just opened has drawn Coda's workspace, prompt or
/// sign-in page.
const DRAWN: &str =
    "!window.__left && !!document.querySelector('.glass-app, .session-prompt, .sign-in-content')";

/// Mary Louis, local-product's MARY_WORKSPACE_ID.
const MARY: &str = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
const LIVE: [&str; 2] = ["Yours, live", "Another physician's, live"];

/// How the driver lets a held request through.
enum Answer {
    /// The sample server answers.
    Coda,
    /// Coda can't be reached.
    Unreachable,
    /// What Coda would answer and the sample server can't.
    Says(u16, serde_json::Value),
}

struct Walker {
    _browser: Browser,
    _control: devtools::Page,
    a: devtools::Page,
    b: devtools::Page,
    address: std::net::SocketAddr,
    base: String,
    /// The real session behind each of the model's sessions.
    slots: [Option<String>; 3],
}

impl Walker {
    fn new(address: std::net::SocketAddr) -> Self {
        let browser = Browser::launch();
        // Both tabs open in one browser context of their own, so they share
        // storage and news as two tabs of one browser do.
        let port = browser.port();
        let (_, version) = http(
            ([127, 0, 0, 1], port).into(),
            "GET",
            "/json/version",
            None,
            None,
        );
        let endpoint = version["webSocketDebuggerUrl"]
            .as_str()
            .expect("the browser endpoint");
        let endpoint = &endpoint[endpoint.find("/devtools/").expect("a devtools path")..];
        let mut control = devtools::connect(port, endpoint).expect("the browser endpoint");
        let context = control.call_session("Target.createBrowserContext", serde_json::json!({}));
        let context = context
            .pointer("/result/browserContextId")
            .cloned()
            .expect("a browser context");
        let mut open = || {
            let created = control.call_session(
                "Target.createTarget",
                serde_json::json!({ "url": "about:blank", "browserContextId": context }),
            );
            let id = created
                .pointer("/result/targetId")
                .and_then(|v| v.as_str())
                .expect("a tab");
            let mut page = devtools::connect(port, &format!("/devtools/page/{id}")).expect("a tab");
            page.call_session("Page.enable", serde_json::json!({}));
            page.call_session(
                "Emulation.setDeviceMetricsOverride",
                serde_json::json!({ "width": DESKTOP.0, "height": DESKTOP.1, "deviceScaleFactor": 1, "mobile": false }),
            );
            page.call_session(
                "Page.addScriptToEvaluateOnNewDocument",
                serde_json::json!({ "source": HOLD_CODA }),
            );
            page
        };
        let (a, b) = (open(), open());
        Self {
            _browser: browser,
            _control: control,
            a,
            b,
            address,
            base: format!("http://{address}"),
            slots: Default::default(),
        }
    }

    fn tab(&mut self, t: &str) -> &mut devtools::Page {
        if t == "A" { &mut self.a } else { &mut self.b }
    }

    fn walk(mut self, start: &ModelState, steps: &[WalkStep]) {
        let mut before = start.clone();
        for (n, step) in steps.iter().enumerate() {
            self.act(&step.event, &before, &step.state);
            if let Err(difference) = self.settle(&step.state) {
                let path: Vec<&str> = steps[..=n].iter().map(|s| s.event.as_str()).collect();
                let log = [
                    self.tab("A")
                        .evaluate("JSON.stringify((window.__coda || {}).log || [])"),
                    self.tab("B")
                        .evaluate("JSON.stringify((window.__coda || {}).log || [])"),
                ];
                panic!("after {}:\n{difference}\nlog: {log:?}", path.join(" → "));
            }
            before = step.state.clone();
        }
        // Coda knows each session the model knows, live or ended.
        for (i, slot) in self.slots.clone().iter().enumerate() {
            if let Some(token) = slot {
                let live = LIVE.contains(&before[&format!("Session {}", i + 1)].as_str());
                let (status, _) = http(self.address, "GET", "/auth/whoami", Some(token), None);
                assert_eq!(
                    status == 200,
                    live,
                    "Session {} is {} but Coda answered {status}",
                    i + 1,
                    before[&format!("Session {}", i + 1)]
                );
            }
        }
    }

    fn act(&mut self, event: &str, before: &ModelState, after: &ModelState) {
        let (what, t) = match event.rsplit_once(" in tab ") {
            Some((what, t)) => (what, t.to_owned()),
            None => (event, String::new()),
        };
        let c = |region: &str| format!("Tab {t} {region}");
        match what {
            "open Coda" => { let url = self.base.clone(); self.open(&t, &url) }
            "open a patient link" => { let url = format!("{}/?patient={MARY}", self.base); self.open(&t, &url) }
            "open your sign-in link" => {
                let url = format!("{}/?email={STAGING_EMAIL}&code=codas-walk", self.base);
                self.open(&t, &url)
            }
            "open another physician's sign-in link" => {
                let url = format!("{}/?email={}&code=codas-walk", self.base, ANOTHER_EMAIL.replace('@', "%40"));
                self.open(&t, &url)
            }
            "request a sign-in link" => self.request(&t, Answer::Says(200, serde_json::json!({
                "status": "unavailable",
                "message": "If that address has an account, a sign-in link is on its way.",
            }))),
            "use another email" => self.click_text(&t, "Use a different email"),
            "Coda checks the link" | "Coda answers who signed in" => self.answer(&t, "whoami", before),
            "Coda signs in the link" => {
                let slot = (0..3).find(|&i| before[&format!("Session {}", i + 1)] == "Not signed in yet"
                    && after[&format!("Session {}", i + 1)] != "Not signed in yet").expect("a new session");
                let verified = mint(self.address);
                self.slots[slot] = Some(verified["session"].as_str().unwrap().to_owned());
                self.release(&t, "verify", None, &Answer::Says(200, verified));
            }
            "the link was used or has expired" => self.release(&t, "verify", None, &Answer::Says(401, serde_json::json!({
                "detail": "That sign-in link is no longer valid.", "reason": "invalid" }))),
            "the link no longer grants access" => self.release(&t, "verify", None, &Answer::Says(403, serde_json::json!({
                "detail": "This sign-in link no longer gives access to this practice.", "reason": "no_access" }))),
            "Coda can't reach sign-in" => match before[&c("signs in")].as_str() {
                s if s.starts_with("Verifying") => self.release(&t, "verify", None, &Answer::Unreachable),
                s if s.starts_with("Checking the link") => {
                    let bearer = self.slot_token(s);
                    self.release(&t, "whoami", bearer.as_deref(), &Answer::Unreachable)
                }
                _ => self.request(&t, Answer::Unreachable),
            },
            "sign out" => {
                self.tab(&t).evaluate(
                    "(document.querySelector('button[aria-label=\"Sign out of Coda\"]') \
                     || [...document.querySelectorAll('.session-prompt button')].find((b) => b.innerText.trim() === 'Sign out')).click(); 0",
                );
            }
            "Coda answers the read" => self.answer(&t, "read", before),
            "this tab hears the news" => { self.tab(&t).evaluate("window.__coda.hear()"); }
            "the practice removes your access" => {
                // Coda ends the event stream of every workspace on your live sessions.
                for i in 0..3 {
                    if before[&format!("Session {}", i + 1)] == "Yours, live" && let Some(token) = self.slots[i].clone() {
                        for t in ["A", "B"] {
                            let page = self.tab(t);
                            if page.evaluate("!!window.__coda").as_bool() == Some(true) {
                                page.evaluate(&format!("window.__coda.endStreams('{token}'); 0"));
                            }
                        }
                    }
                }
            }
            expires if expires.starts_with("Session ") && expires.ends_with(" expires") => {
                let i: usize = expires["Session ".len()..expires.len() - " expires".len()].parse().unwrap();
                let token = self.slots[i - 1].clone().expect("a session to expire");
                http(self.address, "POST", "/auth/sign-out", Some(&token), Some(&serde_json::json!({})));
            }
            other => panic!("the driver has no step for \"{other}\""),
        }
    }

    /// Asks for a link from whatever this tab shows, and answers the request.
    /// Opens `url` in tab `t`; the step ends once Coda draws its first screen
    /// there. Compiling the page can outlast a step's settle while several
    /// browsers load at once.
    fn open(&mut self, t: &str, url: &str) {
        let page = self.tab(t);
        page.evaluate("window.__left = true");
        page.navigate(url);
        let deadline = Instant::now() + Duration::from_secs(60);
        while page.evaluate(DRAWN).as_bool() != Some(true) {
            assert!(
                Instant::now() < deadline,
                "Coda never drew a screen in tab {t}"
            );
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    fn request(&mut self, t: &str, answer: Answer) {
        let page = self.tab(t);
        page.evaluate(&format!(
            "(() => {{ const input = (document.querySelector('.session-prompt') || document).querySelector('.sign-in-input'); \
               if (input) {{ input.value = '{STAGING_EMAIL}'; input.dispatchEvent(new Event('input', {{ bubbles: true }})); }} return 0; }})()"
        ));
        let press = "(() => { const root = document.querySelector('.session-prompt') || document; \
             const button = [...root.querySelectorAll('button')].find((b) => \
               ['Send sign-in link', 'Email me a sign-in link', 'Send a new link'].includes(b.innerText.trim()) && !b.disabled); \
             if (!button) return false; button.click(); return true; })()";
        wait_for_page(page, press, "a button that sends the link");
        wait_for_page(
            page,
            "window.__coda.held.some((h) => h.kind === 'request')",
            "the link request",
        );
        self.release(t, "request", None, &answer);
    }

    fn click_text(&mut self, t: &str, text: &str) {
        let script = format!(
            "(() => {{ const button = [...document.querySelectorAll('button')].find((b) => b.innerText.trim() === '{text}'); \
               if (!button) return false; button.click(); return true; }})()"
        );
        assert_eq!(
            self.tab(t).evaluate(&script).as_bool(),
            Some(true),
            "no \"{text}\" button"
        );
    }

    /// Answers every held `kind` of tab `t` as Coda would in `state`.
    fn answer(&mut self, t: &str, kind: &str, state: &ModelState) {
        let held = self.held(t);
        let bearers: std::collections::BTreeSet<Option<String>> = held
            .into_iter()
            .filter(|(k, _)| k == kind)
            .map(|(_, b)| b)
            .collect();
        assert!(!bearers.is_empty(), "tab {t} has no {kind} to answer");
        for bearer in bearers {
            let answer = self.coda(kind, bearer.as_deref(), state);
            self.release(t, kind, bearer.as_deref(), &answer);
        }
    }

    /// What Coda answers `kind` presented with `bearer` in `state`.
    fn coda(&self, kind: &str, bearer: Option<&str>, state: &ModelState) -> Answer {
        let Some(i) = bearer.and_then(|b| self.slots.iter().position(|s| s.as_deref() == Some(b)))
        else {
            return Answer::Coda;
        };
        match state[&format!("Session {}", i + 1)].as_str() {
            "Yours, live" if state["Your access"] != "Granted" => Answer::Says(
                401,
                serde_json::json!({
                "detail": "The practice removed your access.", "reason": "access_removed" }),
            ),
            "Another physician's, live" if kind == "whoami" => {
                let (_, mut account) = http(self.address, "GET", "/auth/whoami", bearer, None);
                account["email"] = ANOTHER_EMAIL.into();
                account["display_name"] = "Another Physician".into();
                Answer::Says(200, account)
            }
            _ => Answer::Coda,
        }
    }

    fn release(&mut self, t: &str, kind: &str, bearer: Option<&str>, answer: &Answer) {
        let how = match answer {
            Answer::Coda => serde_json::json!({}),
            Answer::Unreachable => serde_json::json!({ "fail": true }),
            Answer::Says(status, body) => serde_json::json!({ "status": status, "body": body }),
        };
        let script = format!(
            "window.__coda.release('{kind}', {}, {how})",
            bearer.map_or("null".to_owned(), |b| format!("'{b}'"))
        );
        let released = self.tab(t).evaluate(&script).as_u64().unwrap_or(0);
        if released == 0 {
            let log = self
                .tab(t)
                .evaluate("JSON.stringify((window.__coda || {}).log || [])");
            panic!("tab {t} held no {kind} for {bearer:?}\nlog: {log}");
        }
    }

    fn held(&mut self, t: &str) -> Vec<(String, Option<String>)> {
        let seen = self.tab(t).evaluate(OBSERVE);
        seen["held"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|h| {
                (
                    h[0].as_str().unwrap_or_default().to_owned(),
                    h[1].as_str().map(str::to_owned),
                )
            })
            .collect()
    }

    fn slot_token(&self, words: &str) -> Option<String> {
        let i: usize = words.rsplit(' ').next()?.parse().ok()?;
        self.slots[i - 1].clone()
    }

    fn slot_of(&self, token: Option<&str>) -> String {
        match token {
            None => "No session".to_owned(),
            Some(token) => match self.slots.iter().position(|s| s.as_deref() == Some(token)) {
                Some(i) => format!("Session {}", i + 1),
                None => format!("an unknown session {token}"),
            },
        }
    }

    /// Waits until both tabs look as `state` says. A whoami the model doesn't
    /// wait on, like the workspace's own, is answered as Coda would.
    fn settle(&mut self, state: &ModelState) -> Result<(), String> {
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            let mut differences = Vec::new();
            for t in ["A", "B"] {
                let signing = state[&format!("Tab {t} signs in")].clone();
                if !signing.starts_with("Checking") && !signing.starts_with("Asking") {
                    let stray: std::collections::BTreeSet<_> = self
                        .held(t)
                        .into_iter()
                        .filter(|(k, _)| k == "whoami")
                        .collect();
                    for (_, bearer) in stray {
                        {
                            let answer = self.coda("whoami", bearer.as_deref(), state);
                            self.release(t, "whoami", bearer.as_deref(), &answer);
                        }
                    }
                }
                differences.extend(self.differences(t, state));
            }
            // The browser's storage, read from a tab that has opened Coda.
            let mut stored = None;
            for t in ["A", "B"] {
                let page = self.tab(t);
                if page.evaluate("location.protocol").as_str() == Some("http:") {
                    stored = Some(page.evaluate("localStorage.getItem('coda.physician.session')"));
                    break;
                }
            }
            let stored = stored.map_or(state["This browser stores"].clone(), |stored| {
                self.slot_of(stored.as_str())
            });
            if stored != state["This browser stores"] {
                differences.push(format!(
                    "the browser stores {stored}, not {}",
                    state["This browser stores"]
                ));
            }
            if differences.is_empty() {
                return Ok(());
            }
            if Instant::now() > deadline {
                return Err(differences.join("\n"));
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    fn differences(&mut self, t: &str, state: &ModelState) -> Vec<String> {
        let seen = self.tab(t).evaluate(OBSERVE);
        let region = |name: &str| state[&format!("Tab {t} {name}")].clone();
        let mut out = Vec::new();
        let shows = seen["shows"].as_str().unwrap_or_default().to_owned();
        if shows != region("shows") {
            out.push(format!("tab {t} shows {shows}, not {}", region("shows")));
        }
        if region("shows") == "Your workspace" {
            let patient = seen["patient"].as_bool() == Some(true);
            if patient != (region("opens at") == "The linked patient") {
                out.push(format!(
                    "tab {t} opens at {}, not {}",
                    if patient { "the patient" } else { "Today" },
                    region("opens at")
                ));
            }
        }
        let held: Vec<(String, String)> = seen["held"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|h| {
                (
                    h[0].as_str().unwrap_or_default().to_owned(),
                    self.slot_of(h[1].as_str()),
                )
            })
            .collect();
        let of = |kind: &str| {
            held.iter()
                .filter(|(k, _)| k == kind)
                .map(|(_, s)| s.clone())
                .collect::<std::collections::BTreeSet<_>>()
        };
        let reads = of("read");
        let want_reads = match region("reads").as_str() {
            "No read out" => Default::default(),
            "Reading with no session" => {
                std::collections::BTreeSet::from(["No session".to_owned()])
            }
            reading => std::collections::BTreeSet::from([reading
                .trim_start_matches("Reading with ")
                .to_owned()]),
        };
        if reads != want_reads {
            out.push(format!(
                "tab {t} waits on reads with {reads:?}, not {}",
                region("reads")
            ));
        }
        let signing = region("signs in");
        let verifying = !of("verify").is_empty();
        if verifying != signing.starts_with("Verifying") {
            out.push(format!(
                "tab {t} {} link verification, but the model says {signing}",
                if verifying { "waits on a" } else { "has no" }
            ));
        }
        let whoami = of("whoami");
        let want_whoami = signing.starts_with("Checking") || signing.starts_with("Asking");
        if !whoami.is_empty() != want_whoami
            || (signing.starts_with("Checking")
                && whoami
                    != std::collections::BTreeSet::from([signing
                        .rsplit(" with ")
                        .next()
                        .unwrap()
                        .to_owned()]))
        {
            out.push(format!(
                "tab {t} waits on whoami with {whoami:?}, but the model says {signing}"
            ));
        }
        if !of("request").is_empty() {
            out.push(format!("tab {t} still waits on a link request"));
        }
        let news = seen["news"].as_u64().unwrap_or(0) > 0;
        // A closed tab has no page to hear news.
        if region("shows") != "Closed" && news != (state[&format!("News for tab {t}")] != "No news")
        {
            out.push(format!(
                "tab {t} {} news, but the model says {}",
                if news { "has" } else { "has no" },
                state[&format!("News for tab {t}")]
            ));
        }
        if seen["signingOut"].as_u64().unwrap_or(0) > 0 {
            out.push(format!("tab {t} is still signing out at Coda"));
        }
        out
    }
}

fn wait_for_page(page: &mut devtools::Page, condition: &str, what: &str) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if page.evaluate(condition).as_bool() == Some(true) {
            return;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    panic!("{what} never came");
}

/// A fresh session for the staging account, as `/auth/verify` answers it.
fn mint(address: std::net::SocketAddr) -> serde_json::Value {
    // One link at a time: a newer link for the account replaces the last.
    static MINTING: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _one = MINTING
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let (_, issued) = http(
        address,
        "POST",
        "/auth/sign-in",
        None,
        Some(&serde_json::json!({ "email": STAGING_EMAIL })),
    );
    let token = issued["token"]
        .as_str()
        .expect("proof delivery returns the token");
    let (status, verified) = http(
        address,
        "POST",
        "/auth/verify",
        None,
        Some(&serde_json::json!({ "email": STAGING_EMAIL, "token": token })),
    );
    assert_eq!(status, 200, "{verified}");
    verified
}

/// One request to the sample server, outside any tab.
fn http(
    address: std::net::SocketAddr,
    method: &str,
    path: &str,
    bearer: Option<&str>,
    body: Option<&serde_json::Value>,
) -> (u16, serde_json::Value) {
    use std::io::{Read, Write};
    let mut stream = std::net::TcpStream::connect(address).expect("the sample server");
    let body = body.map(|b| b.to_string()).unwrap_or_default();
    let auth = bearer
        .map(|b| format!("Authorization: Bearer {b}\r\n"))
        .unwrap_or_default();
    write!(
        stream,
        "{method} {path} HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\nContent-Type: application/json\r\n{auth}Content-Length: {}\r\n\r\n{body}",
        body.len()
    )
    .unwrap();
    // Read one answer, whether or not the other side closes the connection.
    stream
        .set_read_timeout(Some(Duration::from_secs(30)))
        .unwrap();
    let mut response = Vec::new();
    let mut buf = [0; 8192];
    loop {
        let read = stream.read(&mut buf).unwrap_or(0);
        response.extend_from_slice(&buf[..read]);
        let text = String::from_utf8_lossy(&response);
        let complete = text.split_once("\r\n\r\n").is_some_and(|(head, body)| {
            let head = head.to_ascii_lowercase();
            match head
                .lines()
                .find_map(|line| line.strip_prefix("content-length:"))
            {
                Some(length) => body.len() >= length.trim().parse().unwrap_or(0),
                None => !head.contains("chunked") || body.ends_with("0\r\n\r\n"),
            }
        });
        if read == 0 || complete {
            break;
        }
    }
    let response = String::from_utf8_lossy(&response);
    let (head, mut rest) = response.split_once("\r\n\r\n").expect("an HTTP answer");
    let status = head
        .split(' ')
        .nth(1)
        .and_then(|s| s.parse().ok())
        .expect("a status");
    let mut text = String::new();
    if head
        .to_ascii_lowercase()
        .contains("transfer-encoding: chunked")
    {
        while let Some((size, after)) = rest.split_once("\r\n") {
            let size = usize::from_str_radix(size.trim(), 16).unwrap_or(0);
            if size == 0 {
                break;
            }
            text.push_str(&after[..size]);
            rest = &after[size + 2..];
        }
    } else {
        text = rest.to_owned();
    }
    (
        status,
        serde_json::from_str(&text).unwrap_or(serde_json::Value::Null),
    )
}
