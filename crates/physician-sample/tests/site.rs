//! Loads the physician page in Chromium and fails when the document is empty.
//! Dioxus mounts into `#main`. A missing element leaves the visible page blank.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

#[tokio::test]
async fn the_recorded_practice_is_visible() {
    let site = build_site();
    // SAFETY: this test is the only process that reads the variable, and it
    // sets it before the server starts.
    unsafe { env::set_var("PHYSICIAN_SITE_DIR", &site) };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { physician_sample::serve(listener).await });
    wait_until_ready(address).await;

    let url = format!("http://{address}/");
    let chrome = chromium();
    let child = Command::new(&chrome)
        .args([
            "--headless=new",
            "--disable-gpu",
            "--no-sandbox",
            "--virtual-time-budget=15000",
            "--timeout=20000",
            "--dump-dom",
            &url,
        ])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("chromium");
    let pid = child.id();
    let wait = tokio::task::spawn_blocking(move || child.wait_with_output());
    let output = match tokio::time::timeout(Duration::from_secs(40), wait).await {
        Ok(Ok(output)) => output.expect("chromium"),
        _ => {
            let _ = Command::new("kill").arg(pid.to_string()).status();
            panic!("chromium did not finish");
        }
    };
    let dom = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "chromium exited {:?}\n{dom}\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        dom.contains("Today")
            && dom.contains("Coda hasn’t read Gmail yet.")
            && dom.contains("Coda hasn’t read Spruce yet.")
            && dom.contains("3:30 PM")
            && dom.contains("Mary Louis")
            && dom.contains("m3 11 9-8 9 8")
            && dom.contains("viewBox=\"0 0 24 24\"")
            && dom.contains("stroke-width=\"1.8\""),
        "the page was blank or incomplete:\n{dom}"
    );
    let logo = tokio::task::spawn_blocking(move || read_logo(address))
        .await
        .expect("logo read");
    let head = String::from_utf8_lossy(&logo[..logo.len().min(80)]);
    assert!(
        head.contains("200") && logo.windows(8).any(|window| window == b"\x89PNG\r\n\x1a\n"),
        "the reference logo was not a PNG:\n{head}"
    );
}

fn read_logo(address: std::net::SocketAddr) -> Vec<u8> {
    use std::io::{Read, Write};
    let mut stream = std::net::TcpStream::connect(address).expect("logo connection");
    stream
        .write_all(
            b"GET /assets/coda-health-logo-transparent.png HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        )
        .unwrap();
    let mut body = Vec::new();
    stream.read_to_end(&mut body).unwrap();
    body
}

fn build_site() -> PathBuf {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let target = env::temp_dir().join("coda-physician-wasm");
    let wasm = target.join("wasm32-unknown-unknown/debug/physician-app.wasm");
    let mut cargo = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
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
    let site = workspace.join("target/physician-web");
    fs::create_dir_all(&site).unwrap();
    let status = Command::new(wasm_bindgen())
        .args([
            "--target",
            "web",
            "--out-dir",
            site.to_str().unwrap(),
            wasm.to_str().unwrap(),
        ])
        .status()
        .expect("wasm-bindgen");
    assert!(status.success(), "wasm-bindgen failed");
    site
}

fn cargo_without_parent_package(command: &mut Command) {
    command.env_clear();
    for key in [
        "PATH",
        "HOME",
        "USER",
        "RUSTUP_HOME",
        "CARGO_HOME",
        "RUSTUP_TOOLCHAIN",
        "TMPDIR",
    ] {
        if let Some(value) = env::var_os(key) {
            command.env(key, value);
        }
    }
}

fn wasm_bindgen() -> PathBuf {
    if let Some(path) = env::var_os("WASM_BINDGEN") {
        return PathBuf::from(path);
    }
    if let Some(path) = find_on_path("wasm-bindgen") {
        return path;
    }
    let local = env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/bin/wasm-bindgen"));
    if let Some(path) = local.filter(|path| path.is_file()) {
        return path;
    }
    panic!("wasm-bindgen 0.2.129 is required to build the physician page");
}

fn chromium() -> PathBuf {
    if let Some(path) = env::var_os("CHROMIUM") {
        return PathBuf::from(path);
    }
    for name in [
        "chromium",
        "chromium-browser",
        "google-chrome",
        "google-chrome-stable",
    ] {
        if let Some(path) = find_on_path(name) {
            return path;
        }
    }
    panic!("chromium is required to validate the physician page");
}

fn find_on_path(name: &str) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    env::split_paths(&paths)
        .map(|dir| dir.join(name))
        .find(|path| path.is_file())
}

async fn wait_until_ready(address: std::net::SocketAddr) {
    let started = Instant::now();
    loop {
        if started.elapsed() > Duration::from_secs(10) {
            panic!("physician site did not accept connections on {address}");
        }
        if std::net::TcpStream::connect(address).is_ok() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
