//! Suite 1: the authenticated physician journeys in a real browser.
//!
//! `PHYSICIAN_AUTH=required` on the recorded practice: an unauthenticated page
//! shows the sign-in screen, a proof token signs in through the Dioxus form,
//! Today and a panel workspace render, sign-out returns to the sign-in screen,
//! and the steps hold at desktop, phone, and reduced-motion viewports.
//! Book: `book/src/physician-site.md`.

use std::path::PathBuf;
use std::sync::OnceLock;

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
    let text = browser.wait_for("Paste the code from the sign-in link");
    assert!(
        text.contains("Sign-in code"),
        "the verify stage did not appear:\n{text}"
    );

    // The proof token: sign-in mints one for this account on demand.
    let token = sign_in_token(&base).await;

    browser.type_into(".sign-in-input", &token);
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

/// Mints one sign-in token for the staging account over HTTP.
async fn sign_in_token(base: &str) -> String {
    let response = reqwest::Client::new()
        .post(format!("{base}/auth/sign-in"))
        .json(&serde_json::json!({ "email": STAGING_EMAIL }))
        .send()
        .await
        .expect("sign-in call");
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let body: serde_json::Value = response.json().await.expect("sign-in body");
    body["token"].as_str().expect("issued token").to_owned()
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
