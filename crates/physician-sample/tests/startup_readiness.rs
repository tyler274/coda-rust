//! Startup readiness proof: `physician-sample` and `physician-worker` bind immediately
//! when configuration is valid and report degraded `/readyz` while retrying unavailable runtime
//! dependencies instead of exiting at boot.

use std::net::TcpListener;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

#[path = "scale/mod.rs"]
mod support;

struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    listener.local_addr().expect("local addr").port()
}

#[tokio::test]
async fn sample_and_worker_start_serving_degraded_readyz_when_postgres_is_unreachable() {
    let dead_pg_port = free_port();
    let sample_port = free_port();
    let worker_port = free_port();
    let db_url = format!("postgres://nobody:nothing@127.0.0.1:{dead_pg_port}/none");

    let mut sample = ChildGuard(
        Command::new(env!("CARGO_BIN_EXE_physician-sample"))
            .env_clear()
            .envs(support::models::environment())
            .env("PATH", std::env::var_os("PATH").unwrap_or_default())
            .env("PHYSICIAN_LISTEN_ADDR", format!("127.0.0.1:{sample_port}"))
            .env("PHYSICIAN_AUTH", "required")
            .env("PHYSICIAN_DURABLE_CHAT", "1")
            .env("PHYSICIAN_DATABASE_URL", &db_url)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn physician-sample"),
    );

    let client = reqwest::Client::new();
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        if let Some(status) = sample.0.try_wait().expect("poll physician-sample") {
            panic!(
                "physician-sample exited prematurely ({status}) instead of starting and retrying"
            );
        }
        if let Ok(resp) = client
            .get(format!("http://127.0.0.1:{sample_port}/readyz"))
            .send()
            .await
        {
            assert_eq!(resp.status(), reqwest::StatusCode::SERVICE_UNAVAILABLE);
            break;
        }
        assert!(
            Instant::now() < deadline,
            "physician-sample did not answer /readyz before deadline"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let health = client
        .get(format!("http://127.0.0.1:{sample_port}/health"))
        .send()
        .await
        .expect("sample /health");
    assert_eq!(health.status(), reqwest::StatusCode::OK);

    let mut worker = ChildGuard(
        Command::new(env!("CARGO_BIN_EXE_physician-worker"))
            .env_clear()
            .envs(support::models::environment())
            .env("PATH", std::env::var_os("PATH").unwrap_or_default())
            .env(
                "PHYSICIAN_WORKER_LISTEN_ADDR",
                format!("127.0.0.1:{worker_port}"),
            )
            .env("PHYSICIAN_DATABASE_URL", &db_url)
            .env("PHYSICIAN_RESPONSES_STUB_PORT", "1")
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn physician-worker"),
    );
    let worker_deadline = Instant::now() + Duration::from_secs(3);
    loop {
        if let Some(status) = worker.0.try_wait().expect("poll physician-worker") {
            panic!(
                "physician-worker exited prematurely ({status}) instead of starting and retrying"
            );
        }
        if let Ok(resp) = client
            .get(format!("http://127.0.0.1:{worker_port}/readyz"))
            .send()
            .await
        {
            assert_eq!(resp.status(), reqwest::StatusCode::SERVICE_UNAVAILABLE);
            break;
        }
        assert!(
            Instant::now() < worker_deadline,
            "physician-worker did not answer /readyz before deadline"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let worker_health = client
        .get(format!("http://127.0.0.1:{worker_port}/health"))
        .send()
        .await
        .expect("worker /health");
    assert_eq!(worker_health.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn sample_and_worker_recover_readyz_from_503_to_200_when_postgres_recovers() {
    let Some(database) = support::Database::create("startup_recover").await else {
        return;
    };
    let parsed_url: reqwest::Url = database.url.parse().expect("valid postgres url");
    let pg_host = parsed_url.host_str().expect("pg host").to_owned();
    let pg_port = parsed_url.port().unwrap_or(5432);
    let upstream_addr = format!("{pg_host}:{pg_port}");

    let gate = Arc::new(AtomicBool::new(false));
    let proxy_listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind tcp proxy");
    let proxy_port = proxy_listener.local_addr().expect("proxy addr").port();
    let gate_clone = Arc::clone(&gate);
    let proxy_task = tokio::spawn(async move {
        while let Ok((mut inbound, _)) = proxy_listener.accept().await {
            if !gate_clone.load(Ordering::SeqCst) {
                drop(inbound);
                continue;
            }
            let target = upstream_addr.clone();
            tokio::spawn(async move {
                if let Ok(mut outbound) = tokio::net::TcpStream::connect(&target).await {
                    let _ = tokio::io::copy_bidirectional(&mut inbound, &mut outbound).await;
                }
            });
        }
    });

    let mut proxied_url = parsed_url.clone();
    proxied_url
        .set_host(Some("127.0.0.1"))
        .expect("set proxy host");
    proxied_url
        .set_port(Some(proxy_port))
        .expect("set proxy port");
    let proxied_db = proxied_url.to_string();

    let sample_port = free_port();
    let worker_port = free_port();
    let mut sample = ChildGuard(
        Command::new(env!("CARGO_BIN_EXE_physician-sample"))
            .env_clear()
            .envs(support::models::environment())
            .env("PATH", std::env::var_os("PATH").unwrap_or_default())
            .env("PHYSICIAN_LISTEN_ADDR", format!("127.0.0.1:{sample_port}"))
            .env("PHYSICIAN_AUTH", "required")
            .env("PHYSICIAN_DURABLE_CHAT", "1")
            .env("PHYSICIAN_DATABASE_URL", &proxied_db)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn physician-sample"),
    );
    let mut worker = ChildGuard(
        Command::new(env!("CARGO_BIN_EXE_physician-worker"))
            .env_clear()
            .envs(support::models::environment())
            .env("PATH", std::env::var_os("PATH").unwrap_or_default())
            .env(
                "PHYSICIAN_WORKER_LISTEN_ADDR",
                format!("127.0.0.1:{worker_port}"),
            )
            .env("PHYSICIAN_DATABASE_URL", &proxied_db)
            .env("PHYSICIAN_RESPONSES_STUB_PORT", "1")
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn physician-worker"),
    );

    let client = reqwest::Client::new();
    let initial_deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if let Some(status) = sample.0.try_wait().expect("poll sample") {
            panic!("physician-sample exited early while DB gated: {status}");
        }
        if let Some(status) = worker.0.try_wait().expect("poll worker") {
            panic!("physician-worker exited early while DB gated: {status}");
        }
        let s_ready = client
            .get(format!("http://127.0.0.1:{sample_port}/readyz"))
            .send()
            .await;
        let w_ready = client
            .get(format!("http://127.0.0.1:{worker_port}/readyz"))
            .send()
            .await;
        if let (Ok(sr), Ok(wr)) = (s_ready, w_ready) {
            assert_eq!(sr.status(), reqwest::StatusCode::SERVICE_UNAVAILABLE);
            assert_eq!(wr.status(), reqwest::StatusCode::SERVICE_UNAVAILABLE);
            break;
        }
        assert!(
            Instant::now() < initial_deadline,
            "initial /readyz 503 timeout"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    // Open the TCP gate to Postgres: both binaries recover automatically and flip /readyz to 200.
    gate.store(true, Ordering::SeqCst);
    let recover_deadline = Instant::now() + Duration::from_secs(15);
    loop {
        if let Some(status) = sample.0.try_wait().expect("poll sample") {
            let mut err = String::new();
            if let Some(mut stderr) = sample.0.stderr.take() {
                use std::io::Read as _;
                let _ = stderr.read_to_string(&mut err);
            }
            panic!("physician-sample exited while recovering ({status}): {err}");
        }
        if let Some(status) = worker.0.try_wait().expect("poll worker") {
            let mut err = String::new();
            if let Some(mut stderr) = worker.0.stderr.take() {
                use std::io::Read as _;
                let _ = stderr.read_to_string(&mut err);
            }
            panic!("physician-worker exited while recovering ({status}): {err}");
        }
        let s_ready = client
            .get(format!("http://127.0.0.1:{sample_port}/readyz"))
            .send()
            .await;
        let w_ready = client
            .get(format!("http://127.0.0.1:{worker_port}/readyz"))
            .send()
            .await;
        let last_state = match (s_ready, w_ready) {
            (Ok(sr), Ok(wr)) => {
                let ss = sr.status();
                let ws = wr.status();
                let sb = sr.text().await.unwrap_or_default();
                let wb = wr.text().await.unwrap_or_default();
                if ss == reqwest::StatusCode::OK && ws == reqwest::StatusCode::OK {
                    break;
                }
                format!("sample=({ss}, {sb:?}), worker=({ws}, {wb:?})")
            }
            (sr, wr) => {
                format!("sample_err={:?}, worker_err={:?}", sr.err(), wr.err())
            }
        };
        assert!(
            Instant::now() < recover_deadline,
            "recovery /readyz 200 timeout: {last_state}"
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    proxy_task.abort();
}

#[test]
fn invalid_hosted_config_refuses_immediately_without_serving() {
    for bin in [
        env!("CARGO_BIN_EXE_physician-sample"),
        env!("CARGO_BIN_EXE_physician-worker"),
    ] {
        let port = free_port();
        let output = Command::new(bin)
            .env_clear()
            .env("PATH", std::env::var_os("PATH").unwrap_or_default())
            .env("CODA_HOSTED", "1")
            .env("PHYSICIAN_LISTEN_ADDR", format!("127.0.0.1:{port}"))
            .env("PHYSICIAN_WORKER_LISTEN_ADDR", format!("127.0.0.1:{port}"))
            .output()
            .expect("run binary with invalid hosted config");
        assert!(
            !output.status.success(),
            "binary {bin} must refuse invalid hosted config"
        );
    }
}
