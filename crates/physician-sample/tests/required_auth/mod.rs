//! The recorded practice with `PHYSICIAN_AUTH=required`, served with the
//! built physician page, for the sign-in browser tests.

use std::path::PathBuf;
use std::sync::OnceLock;

use crate::common;

pub const STAGING_EMAIL: &str = "physician@codahealth.test";

/// What a signed-in Today says while nothing waits.
pub const SIGNED_IN: &str = "Nothing needs you right now.";

/// Ends the page's session on the server, as expiry does; the page is not told.
pub const END_THE_SESSION: &str = "fetch('/auth/sign-out', { method: 'POST', headers: { \
     Authorization: 'Bearer ' + localStorage.getItem('coda.physician.session') } }); 'sent'";

/// Boots the recorded practice with `PHYSICIAN_AUTH=required`. The server
/// seeds its staging account and panel itself on the memory store. Its
/// physician session is a temporary file and its execution is offline replay.
pub async fn spawn_required_server() -> common::IsolatedServer {
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
    let target = workspace.join("target/coda-physician-wasm");
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
