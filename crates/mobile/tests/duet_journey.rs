//! Suite 2: the mobile shell's Duet journey on a live in-process backend.
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

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_duet_journey_runs_on_a_live_stub_backend() {
    let address = spawn_site().await;
    let base = format!("http://{address}");

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

    // The Duet tab renders the recorded thread through the live backend.
    browser.click(r#"nav.tabs button:nth-of-type(1)"#);
    let text = browser.wait_for("About seven hours.");
    assert!(
        text.contains("How did I sleep?"),
        "the recorded thread did not render:\n{text}"
    );

    // A send is accepted by the stub and the user line lands on the page.
    browser.type_into(".duet-composer-field", SEND_TEXT);
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

    // The Shared tab reports the stub's sharing overview.
    browser.click(r#"nav.tabs button[aria-current="false"]:nth-of-type(4)"#);
    let text = browser.wait_for("Ada");
    assert!(
        text.contains("Shared"),
        "the Shared tab did not render:\n{text}"
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

/// One site per spawn: the mobile page at `/` and the stub router mounted
/// under `/api` in this same process. The WASM build is shared through the
/// process-wide cell; the two servers stay independent.
async fn spawn_site() -> std::net::SocketAddr {
    let site = shared_site();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let store = Arc::new(Mutex::new(
        duet_stub::Store::open(
            std::env::temp_dir().join(format!("duet-stub-browser-{}.json", std::process::id())),
        )
        .expect("stub store"),
    ));
    let app = site_router(site, duet_stub::router(store));
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    wait_until_ready(address).await;
    address
}

/// The mobile page at `/` and the stub under `/api`. The stub keeps its own
/// `/api/...` route paths, so the request is forwarded unchanged.
fn site_router(site_root: PathBuf, api: axum::Router) -> axum::Router {
    axum::Router::new().fallback(move |request: axum::http::Request<axum::body::Body>| {
        let site_root = site_root.clone();
        let mut api = api.clone();
        async move {
            if request.uri().path().starts_with("/api/") {
                use tower::Service as _;
                return api
                    .call(request)
                    .await
                    .unwrap_or_else(|never| match never {});
            }
            match static_file(&site_root, request.uri().path()).await {
                Ok(response) => response,
                Err(_) => axum::http::Response::builder()
                    .status(axum::http::StatusCode::NOT_FOUND)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            }
        }
    })
}

async fn static_file(
    root: &std::path::Path,
    path: &str,
) -> Result<axum::response::Response, std::io::Error> {
    use tokio::io::AsyncReadExt;

    let relative = path.trim_start_matches('/');
    let relative = if relative.is_empty() {
        "index.html"
    } else {
        relative
    };
    let file = root.join(relative);
    if !file.starts_with(root) || !file.is_file() {
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
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let target = std::env::temp_dir().join("coda-mobile-wasm");
    let wasm = target.join("wasm32-unknown-unknown/debug/mobile.wasm");
    let mut cargo =
        std::process::Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    cargo_without_parent_package(&mut cargo);
    let status = cargo
        .current_dir(&workspace)
        .env("CARGO_TARGET_DIR", &target)
        .args([
            "build",
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
    std::fs::create_dir_all(workspace.join("target/mobile-web")).unwrap();
    std::fs::copy(
        workspace.join("crates/mobile/index.html"),
        workspace.join("target/mobile-web/index.html"),
    )
    .unwrap();
    let status = std::process::Command::new(wasm_bindgen())
        .args([
            "--target",
            "web",
            "--out-dir",
            workspace.join("target/mobile-web").to_str().unwrap(),
            wasm.to_str().unwrap(),
        ])
        .status()
        .expect("wasm-bindgen");
    assert!(status.success(), "wasm-bindgen failed");
    workspace.join("target/mobile-web")
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
