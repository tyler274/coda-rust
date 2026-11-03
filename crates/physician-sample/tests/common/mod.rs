//! An in-process sample server that cannot reach a model or the user's state.
//!
//! `physician_sample::serve` reads its execution mode and its physician
//! session from the process environment. Unset, those are `live` (Codex CLI)
//! and the persistent XDG session, whose exclusive lock admits one server.
//! Parallel browser tests would contend for that lock and write into the
//! operator's real session. Each server started here gets its own temporary
//! session file and `replay` against an empty synthetic cassette: no Codex
//! process, no API key, and any chat turn fails closed.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// An empty schema-1 Codex cassette. Replay answers no request from it.
pub const EMPTY_CASSETTE: &str = r#"{
  "schema_version": 1,
  "cassette_id": "physician-sample-browser-tests",
  "recorded_at": "2026-01-01T00:00:00Z",
  "provenance": {
    "adapter_id": "codex-exec",
    "authorization_reference": "physician-sample browser tests; no model exchange",
    "synthetic_non_phi": true
  },
  "exchanges": []
}"#;

/// A running sample server and the temporary state it owns.
pub struct IsolatedServer {
    pub address: SocketAddr,
    state: PathBuf,
}

impl Drop for IsolatedServer {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.state);
    }
}

/// Starts the site on a fresh port with an isolated session, replay execution
/// and the fixture patients. `configure` sets the test's own variables. Startup is
/// serialized and waits for an HTTP answer, so the server has read the
/// environment before another test changes it.
pub async fn serve_isolated(configure: impl FnOnce()) -> IsolatedServer {
    static START: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let _start = START
        .get_or_init(|| tokio::sync::Mutex::new(()))
        .lock()
        .await;

    let state = std::env::temp_dir().join(format!(
        "coda-physician-sample-test-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&state);
    std::fs::create_dir_all(&state).expect("temporary physician state");
    let cassette = state.join("cassette.json");
    std::fs::write(&cassette, EMPTY_CASSETTE).expect("empty replay cassette");

    configure();
    // SAFETY: every server start in this binary holds `START` while it
    // mutates the environment and until the server has read it.
    unsafe {
        std::env::set_var("CODA_PHYSICIAN_SESSION_PATH", state.join("session.json"));
        std::env::set_var("CODA_LOCAL_MODEL_EXECUTION", "replay");
        std::env::set_var("CODA_CASSETTE_PATH", &cassette);
        for key in [
            "CODA_ALLOW_MODEL_CALLS",
            "OPENAI_API_KEY",
            "CODA_CASSETTE_OUTPUT",
            "CODA_CASSETTE_AUTHORIZATION",
            "CODA_PHYSICIAN_SOURCE_SNAPSHOT",
            "CODA_LOCAL_SIMULATION",
            // Browser tests sign in from the proof token. A shell that already
            // has the Gmail sender must not flip this server into hosted mode.
            "PHYSICIAN_GMAIL_FROM",
            "PHYSICIAN_MAIL_TRANSPORT",
            "PHYSICIAN_MAIL_FROM",
            "PHYSICIAN_GMAIL_CLIENT_ID",
            "PHYSICIAN_GMAIL_CLIENT_SECRET",
            "PHYSICIAN_GMAIL_REFRESH_TOKEN",
            "PHYSICIAN_GMAIL_SERVICE_ACCOUNT_KEY",
            "PHYSICIAN_GMAIL_SEND_URL",
            "PHYSICIAN_GMAIL_TOKEN_URL",
            "PHYSICIAN_PUBLIC_ORIGIN",
        ] {
            std::env::remove_var(key);
        }
    }

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let patients = physician_sample::patients::MemoryPatients::fixture();
    tokio::spawn(async move { physician_sample::serve_holding(listener, patients).await });
    wait_until_serving(address).await;
    IsolatedServer { address, state }
}

/// A bound listener accepts before the router exists; an HTTP answer does not.
async fn wait_until_serving(address: SocketAddr) {
    let client = reqwest::Client::new();
    let started = Instant::now();
    loop {
        if started.elapsed() > Duration::from_secs(20) {
            panic!("physician sample server did not answer /health on {address}");
        }
        if let Ok(response) = client
            .get(format!("http://{address}/health"))
            .timeout(Duration::from_secs(2))
            .send()
            .await
            && response.status().is_success()
        {
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
