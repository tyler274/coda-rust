//! An in-process sample server that cannot reach a model or the user's state.
//!
//! Each server renders fixture patients without a model transport.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

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

/// Starts the site on a fresh port with the fixture patients. `configure` sets the test's own variables. Startup is
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
    let prev_physician_database_url = std::env::var_os("PHYSICIAN_DATABASE_URL");
    // SAFETY: every server start in this binary holds `START` while it
    // mutates the environment and until the server has read it.
    unsafe {
        for key in [
            "CODA_LOCAL_MODEL_EXECUTION",
            "PHYSICIAN_DATABASE_URL",
            "CODA_ALLOW_MODEL_CALLS",
            "OPENAI_API_KEY",
            "CODA_PHYSICIAN_SOURCE_SNAPSHOT",
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
            // A scale stack exports durable chat to child commands; this server runs chat inline.
            "PHYSICIAN_DURABLE_CHAT",
        ] {
            std::env::remove_var(key);
        }
    }

    configure();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let patients = physician_sample::patients::MemoryPatients::fixture();
    tokio::spawn(async move { physician_sample::serve_holding(listener, patients).await });
    wait_until_serving(address).await;
    // Restore the caller's process database gate after the spawned server has
    // captured its env snapshot, so isolated fixture tests do not strip
    // PHYSICIAN_DATABASE_URL from subsequent Postgres browser tests.
    // SAFETY: `_start` is still held here until this function returns.
    unsafe {
        match prev_physician_database_url {
            Some(url) => std::env::set_var("PHYSICIAN_DATABASE_URL", url),
            None => std::env::remove_var("PHYSICIAN_DATABASE_URL"),
        }
    }
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
            && let Ok(ready_probe) = client
                .get(format!("http://{address}/auth/whoami"))
                .timeout(Duration::from_secs(2))
                .send()
                .await
            && ready_probe.status() != reqwest::StatusCode::SERVICE_UNAVAILABLE
        {
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
