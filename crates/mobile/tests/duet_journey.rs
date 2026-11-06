//! Suite 2: mobile Duet journeys, with separate stub and durable-service proofs.
//!
//! `durable_doctor` uses isolated PostgreSQL, real durable API routes, compiled
//! patient UI and an independent unauthenticated WebCrypto SHL reader. Its
//! identity is synthetic and proposals are staged directly by the real owner;
//! it does not qualify hosted login or live model generation.
//!
//! The client mode talks to `duet-stub` mounted in the same process under
//! `/api`: the conversation tab shows the recorded thread, a send is accepted
//! and the message lands on the page, and the Today and Shared tabs render
//! their live data. The mode chooser walks both shells in the same browser.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

#[path = "common/devtools.rs"]
mod devtools;

use devtools::{Browser, DESKTOP, PHONE};

const SEND_TEXT: &str = "How is my sleep this week?";

#[path = "common/durable_doctor.rs"]
mod durable_doctor;

#[path = "common/session_refusal.rs"]
mod session_refusal;

// Durable-service D2 proof (no browser): isolated PostgreSQL + real owners + duet-client.
#[path = "common/durable_d2.rs"]
mod durable_d2;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn the_duet_journey_runs_on_a_live_stub_backend() {
    let address = spawn_site().await;
    let base = format!("http://{address}");
    let http = reqwest::Client::new();
    for credential in [None, Some("invalid-synthetic-credential")] {
        for path in ["/api/auth/account", "/api/auth/account/context"] {
            let mut request = if path.ends_with("/context") {
                http.get(format!("{base}{path}"))
            } else {
                http.put(format!("{base}{path}"))
            };
            if let Some(credential) = credential {
                request = request.bearer_auth(credential);
            }
            assert_eq!(request.send().await.unwrap().status(), 401);
        }
    }
    let mut browser = Browser::launch();
    browser.resize(PHONE);

    // Client mode: the shell opens on the live stub, landing on Today.
    let text = browser.text_until(&format!("{base}/?mode=client&api={base}"), "Today");
    assert!(
        text.contains("Duet") && text.contains("Shared"),
        "client mode did not show the Duet tabs:\n{text}"
    );
    assert!(
        !text.contains("Find patient"),
        "client mode leaked the physician shell:\n{text}"
    );

    assert_eq!(
        browser
            .page()
            .evaluate("Object.keys(localStorage).filter(k => k !== 'coda-mode')"),
        serde_json::json!([])
    );
    // The Duet tab renders the recorded thread through the live backend.
    browser.click(r#"nav.tabs button:nth-of-type(1)"#);
    let text = browser.wait_for("About seven hours.");
    assert!(
        text.contains("How did I sleep?"),
        "the recorded thread did not render:\n{text}"
    );

    // The message's one Sources button opens its sources in a sheet; a source opens the
    // retained note from the stub in a sheet over it, and closing that returns to the list.
    browser.click("button.sources-button");
    wait_for_selector(
        &mut browser,
        r#"dialog.dialog-sheet[open] button[aria-label="Open source Sleep note"]"#,
    );
    browser.click(r#"button[aria-label="Open source Sleep note"]"#);
    let text = browser.wait_for("About seven hours, from the sleep note.");
    assert!(
        text.contains("Source: Sleep note") && !text.contains("\"body\""),
        "the source sheet did not show the cited note:\n{text}"
    );
    browser.click(r#"dialog.dialog-sheet:last-of-type button[aria-label="Close"]"#);
    wait_for_gone(&mut browser, "p.source-text");
    browser.click(r#"dialog.dialog-sheet button[aria-label="Close"]"#);
    wait_for_gone(&mut browser, "dialog.dialog-sheet");

    // For my doctor lays the saved page out with the letter layout.
    browser.click(r#"button[aria-label="Open For my doctor"]"#);
    browser.wait_for("Ask about sleep");
    browser.click(r#"button[aria-label="Edit page"]"#);
    browser.wait_for("Add a question");
    browser.type_into(r#"input[aria-label="Name"]"#, "Ada Lovelace");
    browser.type_into(r#"input[aria-label="Date of birth"]"#, "1990-04-02");
    browser.click(r#"button[aria-label="Add a question"]"#);
    wait_for_selector(&mut browser, r#"input[aria-label="Question"]"#);
    browser.type_into(r#"input[aria-label="Question"]"#, "What changed this week?");
    browser.click(r#"button[aria-label="Save page"]"#);
    let text = browser.wait_for("Saved page preview");
    assert!(
        text.contains("What changed this week?"),
        "the doctor page did not show the letter layout:\n{text}"
    );
    let saved: serde_json::Value = reqwest::get(format!("{base}/api/doctor-collection"))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(saved["page"]["name"], "Ada Lovelace");
    assert_eq!(saved["page"]["date_of_birth"], "1990-04-02");
    assert_eq!(saved["page"]["questions"][0], "What changed this week?");
    browser.click(r#"button[aria-label="Close page"]"#);
    browser.wait_for("About seven hours.");

    // A send is accepted by the stub and the user line lands on the page.
    browser.type_into(".composer-input", SEND_TEXT);
    browser.click(".duet-send");
    let text = browser.wait_for(SEND_TEXT);
    assert!(
        text.contains("How did I sleep?") && text.contains("About seven hours."),
        "the accepted send did not join the thread:\n{text}"
    );

    // The Today tab reads the stub's live insights.
    browser.click(r#"nav.tabs button[aria-current="false"]:nth-of-type(2)"#);
    let text = browser.wait_for("Steady walks");
    assert!(
        text.contains("Walking has been steady this week."),
        "the Today tab did not show the stub insights:\n{text}"
    );

    // You can correct one library row and delete another.
    browser.click(r#"nav.tabs button:nth-of-type(3)"#);
    browser.wait_for("Lisinopril");
    browser.click(r#"button[aria-label="Correct Lisinopril"]"#);
    browser.type_into(r#"input[aria-label="Corrected value"]"#, "Lisinopril 5 mg");
    browser.click(r#"button[aria-label="Save correction"]"#);
    let text = wait_until(&mut browser, |text| {
        text.contains("Saved the correction.") && text.contains("Lisinopril 5 mg")
    });
    assert!(
        text.contains("Lisinopril 5 mg"),
        "the corrected medication did not replace the library row:\n{text}"
    );
    browser.click(r#"button[aria-label="Delete LDL cholesterol"]"#);
    let text = wait_until(&mut browser, |text| {
        text.contains("Removed from your records.") && !text.contains("LDL cholesterol")
    });
    assert!(
        text.contains("Lisinopril 5 mg"),
        "the corrected medication disappeared with the deleted lab:\n{text}"
    );

    // Stub/browser D2 proof: You lists each record as a person would read it,
    // and "Ask about this" hands that exact revision to the composer.
    browser.click("details.record-browser summary");
    let text = wait_until(&mut browser, |text| text.contains("Showing "));
    assert!(
        text.contains("Source: Pharmacy") && text.contains("2026-10-01T15:00:00+00:00"),
        "the record browser did not show the source and date:\n{text}"
    );
    assert!(
        !text.contains("Version 1") && !text.contains("Show original"),
        "the record browser still shows a technical inspector:\n{text}"
    );
    wait_for_selector(
        &mut browser,
        r#"button[aria-label="Ask about Lisinopril 5 mg"]"#,
    );
    browser.click(r#"button[aria-label="Ask about Lisinopril 5 mg"]"#);
    let text = wait_until(&mut browser, |text| {
        text.contains("Asking about Lisinopril 5 mg") && !text.contains("(version 1)")
    });
    assert!(
        text.contains("Asking about"),
        "the selected record did not reach the composer:\n{text}"
    );
    browser.click(r#"button[aria-label="Clear selected record"]"#);
    wait_until(&mut browser, |text| !text.contains("Asking about"));

    // The retired person-to-person endpoint stays absent. The Shared tab does not
    // expose physician consent controls.
    assert_eq!(
        http.get(format!("{base}/api/sharing/overview"))
            .send()
            .await
            .unwrap()
            .status(),
        404
    );
    browser.click(r#"nav.tabs button:nth-of-type(4)"#);
    wait_for_selector(
        &mut browser,
        r#"nav.tabs button:nth-of-type(4)[aria-current="page"]"#,
    );
    let text = browser.wait_for("Shared");
    assert!(
        !text.contains("People")
            && !text.contains("Share with my physician")
            && !text.contains("Stop sharing")
            && !text.contains("Awaiting acceptance")
            && !text.contains("not sharing yet")
            && !text.contains("Saved values stay visible"),
        "{text}"
    );

    // The journey holds at the desktop viewport.
    browser.resize(DESKTOP);
    let text = browser.wait_for("Duet");
    assert!(
        text.contains("Today") && text.contains("You") && text.contains("Shared"),
        "the desktop viewport lost the Duet tabs:\n{text}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn the_mode_switch_walks_the_tabs_in_the_browser() {
    let address = spawn_site().await;
    let base = format!("http://{address}");

    let mut browser = Browser::launch();

    // The chooser offers both modes.
    let text = browser.text_until(&base, "Coda");
    assert!(
        text.contains("Physician") && text.contains("Client"),
        "the chooser did not offer both modes:\n{text}"
    );

    // The physician mode opens the physician shell.
    browser.click("main.mode-chooser button:first-of-type");
    let text = browser.wait_for("Today");
    assert!(
        text.contains("Find patient"),
        "the physician mode did not open the physician shell:\n{text}"
    );

    // Switching back clears the mode; the chooser returns.
    browser.click(".mode-switch");
    let text = browser.wait_for("Coda");
    assert!(
        text.contains("Client"),
        "switching back did not restore the chooser:\n{text}"
    );

    // The client mode opens the Duet tabs on the live backend; the thread
    // arrives after selecting the Duet tab.
    browser.click("main.mode-chooser button:nth-of-type(2)");
    let text = browser.wait_for("Today");
    assert!(
        text.contains("Duet") && !text.contains("Find patient"),
        "the client mode did not open the Duet shell:\n{text}"
    );
    browser.click(r#"nav.tabs button:nth-of-type(1)"#);
    let text = browser.wait_for("About seven hours.");
    assert!(
        text.contains("How did I sleep?"),
        "the client mode did not load the thread:\n{text}"
    );
}

fn wait_for_gone(browser: &mut Browser, selector: &str) {
    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    while std::time::Instant::now() < deadline {
        let found = browser.page().run("has", serde_json::json!([selector]));
        if found.as_bool() == Some(false) {
            return;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    panic!("{selector} stayed open");
}

fn wait_for_selector(browser: &mut Browser, selector: &str) {
    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    while std::time::Instant::now() < deadline {
        let found = browser.page().run("has", serde_json::json!([selector]));
        if found.as_bool() == Some(true) {
            return;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    panic!(
        "missing {selector}:\n{}",
        browser.body_text().unwrap_or_default()
    );
}

fn wait_until(browser: &mut Browser, ready: impl Fn(&str) -> bool) -> String {
    let deadline = std::time::Instant::now() + Duration::from_secs(30);
    let mut last = String::new();
    while std::time::Instant::now() < deadline {
        if let Some(text) = browser.body_text() {
            last = text;
            if ready(&last) {
                return last;
            }
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    panic!("the page did not reach the expected state:\n{last}");
}

/// One site per spawn: the mobile page at `/` and the stub router plus the
/// in-process front door mounted under `/api` in this same process. The
/// browser uses an explicitly verified synthetic standalone account adapter.
/// The WASM build is shared through the process-wide cell; the two servers
/// stay independent.
async fn spawn_site() -> std::net::SocketAddr {
    spawn_site_with_patient(false).await.0
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn patient_tabs_do_not_offer_or_request_physician_sharing() {
    let (address, session) = spawn_site_with_patient(true).await;
    let base = format!("http://{address}");
    let mut browser = Browser::launch();
    browser.resize(PHONE);
    browser
        .page()
        .call_session("Network.enable", serde_json::json!({}));
    browser.page().call_session(
        "Network.setExtraHTTPHeaders",
        serde_json::json!({"headers":{"Authorization":format!("Bearer {session}")}}),
    );
    browser.text_until(&format!("{base}/?mode=client"), "Today");
    let evidence = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/verification/physician-sharing");
    std::fs::create_dir_all(&evidence).unwrap();
    for (tab, loaded, absent, image) in [
        (
            3,
            "Add health information",
            "Sharing with my physician",
            "you.json",
        ),
        (4, "Shared", "Share with my physician", "shared.json"),
    ] {
        browser.click(&format!("nav.tabs button:nth-of-type({tab})"));
        wait_for_selector(
            &mut browser,
            &format!(r#"nav.tabs button:nth-of-type({tab})[aria-current="page"]"#),
        );
        let text = browser.wait_for(loaded);
        assert!(!text.contains(absent), "{text}");
        let screenshot = browser.page().call_session(
            "Page.captureScreenshot",
            serde_json::json!({"format":"png"}),
        );
        std::fs::write(
            evidence.join(image),
            serde_json::to_vec(&screenshot).unwrap(),
        )
        .unwrap();
    }
    let paths: Vec<String> = serde_json::from_value(browser.page().evaluate(
        "performance.getEntriesByType('resource').map(entry => new URL(entry.name).pathname)",
    ))
    .unwrap();
    assert!(
        paths.iter().all(|path| path != "/api/consent"
            && !path.starts_with("/api/physician-sharing/")
            && !path.starts_with("/api/sharing/")),
        "{paths:?}"
    );
}

async fn spawn_site_with_patient(linked: bool) -> (std::net::SocketAddr, String) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let store = Arc::new(Mutex::new(
        duet_stub::Store::open(
            std::env::temp_dir().join(format!("duet-stub-browser-{}.json", uuid::Uuid::new_v4())),
        )
        .expect("stub store"),
    ));
    let front_door_store = Arc::new(front_door::MemoryFrontDoorStore::new());
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("unix clock")
        .as_secs();
    let patient = front_door::RosterPatient {
        practice_id: "roger-synthetic-practice".to_owned(),
        patient_id: "pat-elena".to_owned(),
    };
    let invite_token = front_door::FrontDoorStore::invite(
        front_door_store.as_ref(),
        &patient,
        "elena@codahealth.test",
        "Elena Rostova",
        now,
    )
    .await
    .expect("front-door invite");
    let (account_id, session) = front_door::FrontDoorStore::redeem_invitation(
        front_door_store.as_ref(),
        "elena@codahealth.test",
        &invite_token,
        now,
    )
    .await
    .expect("front-door redeem");
    let site = shared_site();
    let expected = format!(
        "Bearer {}",
        if linked {
            session.as_str()
        } else {
            "synthetic-stub-browser-only"
        }
    );
    let account_header = expected.clone();
    // Test-only identity adapter. It verifies the synthetic build credential, then
    // supplies a stable account/incarnation. It never modifies production routing.
    let context = serde_json::json!({"account_id":account_id,"incarnation":uuid::Uuid::new_v4(),"permission_revision":0,"authenticated_at":"2026-10-20T00:00:00Z"});
    let identity = axum::Router::new()
        .route(
            "/api/auth/account",
            axum::routing::put(move |headers: axum::http::HeaderMap| {
                let expected = account_header.clone();
                async move {
                    if headers.get("authorization").and_then(|h| h.to_str().ok())
                        != Some(expected.as_str())
                    {
                        return Err(axum::http::StatusCode::UNAUTHORIZED);
                    }
                    Ok(axum::Json(serde_json::json!({})))
                }
            }),
        )
        .route(
            "/api/auth/account/context",
            axum::routing::get(move |headers: axum::http::HeaderMap| {
                let context = context.clone();
                let expected = expected.clone();
                async move {
                    if headers.get("authorization").and_then(|h| h.to_str().ok())
                        != Some(expected.as_str())
                    {
                        return Err(axum::http::StatusCode::UNAUTHORIZED);
                    }
                    Ok(axum::Json(context))
                }
            }),
        );
    let api = duet_stub::router(store)
        .merge(duet_api::front_door_router(duet_api::DuetApi::new(
            front_door_store,
        )))
        .merge(identity);
    let app = site_router(site, api);
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    wait_until_ready(address).await;
    (address, session)
}

/// The mobile page at `/` and the stub under `/api`. The stub keeps its own
/// `/api/...` route paths, so the request is forwarded unchanged.
fn site_router(site_root: PathBuf, api: axum::Router) -> axum::Router {
    axum::Router::new().fallback(move |request: axum::http::Request<axum::body::Body>| {
        let site_root = site_root.clone();
        let mut api = api.clone();
        async move {
            if request.uri().path().starts_with("/api/")
                || request.uri().path().starts_with("/front-door/")
            {
                use tower::Service as _;
                return api
                    .call(request)
                    .await
                    .unwrap_or_else(|never| match never {});
            }
            match static_file(&site_root, request.uri().path(), "").await {
                Ok(response) => response,
                Err(_) => axum::http::Response::builder()
                    .status(axum::http::StatusCode::NOT_FOUND)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            }
        }
    })
}

#[tokio::test]
async fn static_site_serves_assets_without_escaping_its_root() {
    use tower::ServiceExt;

    let directory =
        std::env::temp_dir().join(format!("coda-site-boundary-{}", uuid::Uuid::new_v4()));
    let root = directory.join("site");
    std::fs::create_dir_all(root.join("assets")).unwrap();
    std::fs::write(root.join("index.html"), "test index").unwrap();
    std::fs::write(root.join("assets/app.js"), "test script").unwrap();
    std::fs::write(directory.join("outside.txt"), "outside the site root").unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(directory.join("outside.txt"), root.join("escaped.txt")).unwrap();
    let app = site_router(root, axum::Router::new());
    let mut observed = Vec::new();
    for path in [
        "/",
        "/assets/app.js",
        "/../outside.txt",
        "/assets/../../outside.txt",
        "/%2e%2e/outside.txt",
        "/escaped.txt",
    ] {
        // Construct the raw URI: browsers and URL clients normalize dot segments.
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri(path)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        observed.push((path, status, body));
    }
    std::fs::remove_dir_all(directory).unwrap();
    for (path, status, body) in observed {
        match path {
            "/" => {
                assert_eq!(status, 200);
                assert_eq!(&body[..], b"test index");
            }
            "/assets/app.js" => {
                assert_eq!(status, 200);
                assert_eq!(&body[..], b"test script");
            }
            _ => assert_eq!(status, 404, "{path} escaped the site root"),
        }
    }
}

async fn static_file(
    root: &std::path::Path,
    path: &str,
    _session: &str,
) -> Result<axum::response::Response, std::io::Error> {
    use tokio::io::AsyncReadExt;

    let relative = path.trim_start_matches('/');
    let relative = if relative.is_empty() {
        "index.html"
    } else {
        relative
    };
    // Compare resolved paths so dot segments and symlinks cannot leave the
    // compiled site tree. Both journey routers use this same boundary.
    let root = root.canonicalize()?;
    let file = root.join(relative).canonicalize()?;
    if !file.starts_with(&root) || !file.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "no such site file",
        ));
    }
    let mut bytes = Vec::new();
    let mut opened = tokio::fs::File::open(&file).await?;
    opened.read_to_end(&mut bytes).await?;
    let mime = match file.extension().and_then(|ext| ext.to_str()) {
        Some("js") => "text/javascript",
        Some("wasm") => "application/wasm",
        Some("css") => "text/css",
        Some("html") => "text/html; charset=utf-8",
        _ => "application/octet-stream",
    };
    Ok(axum::response::Response::builder()
        .header("content-type", mime)
        .body(axum::body::Body::from(bytes))
        .unwrap())
}

/// Builds the WASM site once for this test binary.
fn shared_site() -> PathBuf {
    static SITE: OnceLock<PathBuf> = OnceLock::new();
    SITE.get_or_init(build_site).clone()
}

fn build_site() -> PathBuf {
    build_site_with_bearer(None)
}

fn build_site_with_bearer(bearer: Option<&str>) -> PathBuf {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let target = workspace.join("target").join(if bearer.is_some() {
        "coda-mobile-durable-wasm"
    } else {
        "coda-mobile-wasm"
    });
    let output = workspace.join(if bearer.is_some() {
        "target/mobile-durable-web"
    } else {
        "target/mobile-web"
    });
    let wasm = target.join("wasm32-unknown-unknown/debug/mobile.wasm");
    let mut cargo =
        std::process::Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    cargo_without_parent_package(&mut cargo);
    cargo.env(
        "DUET_BEARER",
        bearer.unwrap_or("synthetic-stub-browser-only"),
    );
    let status = cargo
        .current_dir(&workspace)
        .env("CARGO_TARGET_DIR", &target)
        .args([
            "build",
            "--locked",
            "--offline",
            "-p",
            "mobile",
            "--target",
            "wasm32-unknown-unknown",
            "--features",
            "web",
        ])
        .status()
        .expect("cargo build");
    assert!(status.success(), "mobile wasm build failed");
    std::fs::create_dir_all(output.clone()).unwrap();
    std::fs::copy(
        workspace.join("crates/mobile/index.html"),
        output.join("index.html"),
    )
    .unwrap();
    let status = std::process::Command::new(wasm_bindgen())
        .args([
            "--target",
            "web",
            "--out-dir",
            output.clone().to_str().unwrap(),
            wasm.to_str().unwrap(),
        ])
        .status()
        .expect("wasm-bindgen");
    assert!(status.success(), "wasm-bindgen failed");
    output.clone()
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
    panic!("wasm-bindgen 0.2.129 is required to build the mobile page");
}

fn find_on_path(name: &str) -> Option<PathBuf> {
    let paths = std::env::var_os("PATH")?;
    std::env::split_paths(&paths)
        .map(|dir| dir.join(name))
        .find(|path| path.is_file())
}

async fn wait_until_ready(address: std::net::SocketAddr) {
    let started = std::time::Instant::now();
    loop {
        if started.elapsed() > Duration::from_secs(10) {
            panic!("mobile site did not accept connections on {address}");
        }
        if tokio::net::TcpStream::connect(address).await.is_ok() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
