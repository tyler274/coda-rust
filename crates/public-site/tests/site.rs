//! Chromium proof of the actual executable, including startup asset wiring.
use axum::{
    Router,
    body::Body,
    http::{StatusCode, Uri, header},
    response::Response,
};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::Arc,
    time::Duration,
};

#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn desktop_mobile_motion_reduced_motion_and_static_fallback() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let child = Command::new(env!("CARGO_BIN_EXE_public-site"))
        .env("PUBLIC_SITE_LISTEN_ADDR", address.to_string())
        .env("APP_ENV", "test")
        .env_remove("PUBLIC_SITE_INTERNAL_PROBE_ADDR")
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    struct Server(std::process::Child);
    impl Drop for Server {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    let mut server = Server(child);
    for attempt in 0..100 {
        assert!(
            server.0.try_wait().unwrap().is_none(),
            "public-site exited before serving"
        );
        if std::net::TcpStream::connect(address).is_ok() {
            break;
        }
        assert!(attempt < 99, "public-site did not start");
        std::thread::sleep(Duration::from_millis(100));
    }
    let result = Command::new("node")
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/browser.mjs"))
        .arg(format!("http://{address}"))
        .status()
        .expect("Node and Chromium are required for browser proof");
    assert!(result.success(), "Chromium public-site proof failed");
}

#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn exported_subpath_site_serves_zero_404s_and_renders_in_chromium() {
    struct TempDir(PathBuf);
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    let root = TempDir(std::env::temp_dir().join(format!(
        "coda-public-site-browser-export-{}",
        std::process::id()
    )));
    let _ = fs::remove_dir_all(&root.0);
    let mount = root.0.join("coda-rust");
    fs::create_dir_all(&mount).unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_public-site"))
        .args([
            "--export-dir",
            mount.to_str().unwrap(),
            "--base-path",
            "/coda-rust",
            "--origin",
            "https://tyler274.github.io/coda-rust",
        ])
        .status()
        .expect("run public-site static export");
    assert!(status.success(), "public-site --export-dir failed");

    let book_status = Command::new("mdbook")
        .args([
            "build",
            concat!(env!("CARGO_MANIFEST_DIR"), "/../../book"),
            "-d",
            mount.join("book").to_str().unwrap(),
        ])
        .env("MDBOOK_OUTPUT__HTML__SITE_URL", "/coda-rust/book/")
        .status();
    let has_book = book_status.is_ok_and(|s| s.success());

    let rt = tokio::runtime::Runtime::new().unwrap();
    let (addr, server_handle) = rt.block_on(async {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let serve_root = Arc::new(root.0.clone());
        let app = Router::new().fallback(move |uri: Uri| {
            let serve_root = Arc::clone(&serve_root);
            async move { serve_static(&serve_root, uri.path()) }
        });
        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        (addr, handle)
    });

    let mut cmd = Command::new("node");
    cmd.arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/browser.mjs"))
        .arg(format!("http://{addr}"))
        .arg("/coda-rust");
    if has_book {
        cmd.env("PUBLIC_SITE_BOOK_TEST", "1");
    }
    let result = cmd
        .status()
        .expect("Node and Chromium are required for browser proof");
    server_handle.abort();
    assert!(
        result.success(),
        "Chromium exported /coda-rust/ proof failed"
    );
}

#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn exported_subpath_browser_sql_demo_e2e() {
    struct TempDir(PathBuf);
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    let root = TempDir(std::env::temp_dir().join(format!(
        "coda-public-site-demo-export-{}",
        std::process::id()
    )));
    let _ = fs::remove_dir_all(&root.0);
    let mount = root.0.join("coda-rust");
    fs::create_dir_all(&mount).unwrap();

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let build_status = Command::new("bash")
        .arg(repo_root.join("scripts/build-pages-site.sh"))
        .current_dir(&repo_root)
        .env("OUT_DIR", &mount)
        .env("BASE_PATH", "/coda-rust")
        .env("ORIGIN", "https://tyler274.github.io/coda-rust")
        .status()
        .expect("run scripts/build-pages-site.sh");
    assert!(build_status.success(), "build-pages-site.sh failed");

    let rt = tokio::runtime::Runtime::new().unwrap();
    let (addr, server_handle) = rt.block_on(async {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let serve_root = Arc::new(root.0.clone());
        let app = Router::new().fallback(move |uri: Uri| {
            let serve_root = Arc::clone(&serve_root);
            async move { serve_static(&serve_root, uri.path()) }
        });
        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        (addr, handle)
    });

    let result = Command::new("node")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/demo_browser.mjs"
        ))
        .arg(format!("http://{addr}"))
        .arg("/coda-rust")
        .status()
        .expect("Node and Chromium are required for browser demo proof");
    server_handle.abort();
    assert!(
        result.success(),
        "Chromium exported /coda-rust/ browser SQLite demo E2E failed"
    );
}

fn serve_static(root: &Path, uri_path: &str) -> Response {
    let trimmed = uri_path.trim_start_matches('/');
    if trimmed.contains("..") || trimmed.contains('\\') {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap();
    }
    let clean = trimmed.trim_end_matches('/');
    let direct = root.join(clean);
    let candidate = if direct.is_file() {
        Some((StatusCode::OK, direct))
    } else if root.join(format!("{clean}.html")).is_file() {
        Some((StatusCode::OK, root.join(format!("{clean}.html"))))
    } else if root.join(clean).join("index.html").is_file() {
        Some((StatusCode::OK, root.join(clean).join("index.html")))
    } else {
        None
    };

    let (status, file_path) = match candidate {
        Some(hit) => hit,
        None => (StatusCode::NOT_FOUND, root.join("coda-rust/404.html")),
    };
    let bytes = fs::read(&file_path).unwrap_or_default();
    let content_type = if file_path.extension().and_then(|e| e.to_str()) == Some("html") {
        "text/html; charset=utf-8".to_owned()
    } else {
        mime_guess::from_path(&file_path)
            .first_or_octet_stream()
            .to_string()
    };
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, content_type)
        .body(Body::from(bytes))
        .unwrap()
}
