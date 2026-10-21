//! Loads both modes in Chromium. A physician page is not the Duet tabs.

use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn physician_and_client_modes_render() {
    let site = build_site();
    let address = serve(site);
    let chrome = chromium();
    let client = dump_dom(&chrome, &format!("http://{address}/?mode=client")).await;
    // Patient sign-in is independent of practice invitations. Without an
    // authenticated account the tabs remain gated; duet_journey proves the
    // authenticated shell against the live synthetic identity adapter.
    assert!(
        client.contains("Sign in to your Duet account")
            && client.contains("A practice invitation is not required")
            && !client.contains("Find patient"),
        "client mode did not show the account sign-in gate:\n{client}"
    );
    assert!(
        !client.contains("Find patient"),
        "client mode leaked the physician shell:\n{client}"
    );
    let physician = dump_dom(&chrome, &format!("http://{address}/?mode=physician")).await;
    assert!(
        physician.contains("Today")
            && physician.contains("Find patient")
            && !physician.contains("Shared"),
        "physician mode did not show the physician shell:\n{physician}"
    );
}

fn serve(dir: PathBuf) -> std::net::SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let dir = dir.clone();
            std::thread::spawn(move || {
                let _ = write_file(stream, &dir);
            });
        }
    });
    address
}

fn write_file(mut stream: TcpStream, dir: &Path) -> std::io::Result<()> {
    let mut request = Vec::new();
    let mut buf = [0; 1024];
    loop {
        let read = stream.read(&mut buf)?;
        if read == 0 {
            return Ok(());
        }
        request.extend_from_slice(&buf[..read]);
        if request.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
    }
    let text = String::from_utf8_lossy(&request);
    let path = text
        .split_whitespace()
        .nth(1)
        .unwrap_or("/")
        .split('?')
        .next()
        .unwrap_or("/");
    let relative = path.trim_start_matches('/');
    let file = if relative.is_empty() {
        dir.join("index.html")
    } else if relative.contains("..") || relative.contains('\\') {
        dir.join("missing")
    } else {
        dir.join(relative)
    };
    let bytes = fs::read(&file).ok().filter(|_| file.starts_with(dir));
    let Some(bytes) = bytes else {
        return stream.write_all(
            b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
    };
    let mime = match file.extension().and_then(|ext| ext.to_str()) {
        Some("js") => "text/javascript",
        Some("wasm") => "application/wasm",
        Some("css") => "text/css",
        _ => "text/html; charset=utf-8",
    };
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: {mime}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        bytes.len()
    )?;
    stream.write_all(&bytes)
}

async fn dump_dom(chrome: &Path, url: &str) -> String {
    let child = Command::new(chrome)
        .env_remove("LD_LIBRARY_PATH")
        .args([
            "--headless=new",
            "--disable-gpu",
            "--no-sandbox",
            "--virtual-time-budget=15000",
            "--timeout=20000",
            "--dump-dom",
            url,
        ])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("chromium");
    let pid = child.id();
    let wait = tokio::task::spawn_blocking(move || child.wait_with_output());
    let output = match tokio::time::timeout(Duration::from_secs(90), wait).await {
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
    dom.into_owned()
}

fn build_site() -> PathBuf {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let target = env::temp_dir().join("coda-mobile-wasm");
    let wasm = target.join("wasm32-unknown-unknown/debug/mobile.wasm");
    let mut cargo = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    cargo.env_clear();
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
        if let Some(value) = env::var_os(key) {
            cargo.env(key, value);
        }
    }
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
    let site = workspace.join("target/mobile-web");
    fs::create_dir_all(&site).unwrap();
    fs::copy(
        workspace.join("crates/mobile/index.html"),
        site.join("index.html"),
    )
    .unwrap();
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
    panic!("wasm-bindgen 0.2.129 is required");
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
    panic!("chromium is required");
}

fn find_on_path(name: &str) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    env::split_paths(&paths)
        .map(|dir| dir.join(name))
        .find(|path| path.is_file())
}
