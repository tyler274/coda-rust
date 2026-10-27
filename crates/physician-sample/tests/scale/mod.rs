//! Disposable loopback processes for the physician scale proofs.
//!
//! Postgres comes from `PHYSICIAN_DATABASE_URL` or
//! `PHYSICIAN_RUNTIME_TEST_DATABASE_URL`. Each test creates its own database
//! on that server. Valkey and the Pub/Sub emulator bind `127.0.0.1:0`.
#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::net::TcpListener;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use base64::Engine;
use local_product::runtime::delivery::Delivery;
use sea_orm::ConnectionTrait;
use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub const PROJECT: &str = "coda-local-physician";
pub const FIXED_TEXT: &str = "Noted.";

pub fn topic() -> String {
    format!("projects/{PROJECT}/topics/physician-chat-turns")
}

pub fn dead_letter_topic() -> String {
    format!("projects/{PROJECT}/topics/physician-chat-turns-dead-letter")
}

pub fn site_bin() -> &'static str {
    env!("CARGO_BIN_EXE_physician-sample")
}

pub fn worker_bin() -> &'static str {
    env!("CARGO_BIN_EXE_physician-worker")
}

/// The admin URL of a disposable server, or a skip when the gate is unset.
pub fn gate_url() -> Option<String> {
    for key in [
        "PHYSICIAN_DATABASE_URL",
        "PHYSICIAN_RUNTIME_TEST_DATABASE_URL",
    ] {
        if let Ok(value) = std::env::var(key) {
            let value = value.trim().to_owned();
            if !value.is_empty() {
                return Some(value);
            }
        }
    }
    assert!(
        std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
        "PHYSICIAN_DATABASE_URL or PHYSICIAN_RUNTIME_TEST_DATABASE_URL is required for the Postgres CI proof"
    );
    None
}

pub struct Database {
    admin: String,
    name: String,
    pub url: String,
}

impl Database {
    pub async fn create(label: &str) -> Option<Self> {
        let gate = gate_url()?;
        let (base, _) = gate
            .rsplit_once('/')
            .expect("database URL must name a database");
        let name = format!("scale_{label}_{}", uuid::Uuid::new_v4().simple());
        assert!(
            name.chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '_'),
            "database name {name}"
        );
        let admin = format!("{base}/postgres");
        let connection = sea_orm::Database::connect(&admin)
            .await
            .expect("admin connection");
        connection
            .execute_unprepared(&format!("CREATE DATABASE {name}"))
            .await
            .expect("create disposable database");
        connection.close().await.expect("close admin connection");
        let url = format!("{base}/{name}");
        physician_auth::PostgresAuthStore::connect_with_migrations(&url)
            .await
            .expect("physician auth migrations");
        front_door::PostgresFrontDoorStore::connect_with_migrations(&url)
            .await
            .expect("front-door migrations");
        local_product::runtime::Store::connect(&url)
            .await
            .expect("runtime store")
            .migrate()
            .await
            .expect("runtime migrations");
        Some(Self { admin, name, url })
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        let sql = format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}' AND pid <> pg_backend_pid()",
            self.name
        );
        let _ = Command::new("psql")
            .args([&self.admin, "-c", &sql])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        let _ = Command::new("dropdb")
            .args(["--maintenance-db", &self.admin, "--if-exists", &self.name])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}

/// A loopback port for a child process to bind.
///
/// Binding `127.0.0.1:0` and handing the number to a child is racy: the
/// kernel draws it from the ephemeral range, which every outbound connection
/// (Postgres, Valkey, the emulator) also draws from, so the port can be taken
/// before the child binds it. Ports here come from below that range
/// (Linux defaults to 32768-60999), are never handed out twice by this test
/// process, and are probed before use.
pub fn free_port() -> u16 {
    use std::sync::Mutex;
    const LOW: u16 = 20_000;
    const HIGH: u16 = 32_000;
    static GIVEN: Mutex<Vec<u16>> = Mutex::new(Vec::new());
    let mut given = GIVEN.lock().expect("port ledger");
    for _ in 0..1000 {
        let random = uuid::Uuid::new_v4().as_u128();
        let port = LOW + (random % u128::from(HIGH - LOW)) as u16;
        if given.contains(&port) || TcpListener::bind(("127.0.0.1", port)).is_err() {
            continue;
        }
        given.push(port);
        return port;
    }
    panic!("no free loopback port in {LOW}..{HIGH}");
}

fn kept_env(home: &Path) -> HashMap<String, String> {
    const KEEP: [&str; 10] = [
        "PATH",
        "LD_LIBRARY_PATH",
        "LIBRARY_PATH",
        "SSL_CERT_FILE",
        "SSL_CERT_DIR",
        "NIX_SSL_CERT_FILE",
        "LANG",
        "TZ",
        "TMPDIR",
        "LOCALE_ARCHIVE",
    ];
    let mut env = HashMap::new();
    for key in KEEP {
        if let Ok(value) = std::env::var(key) {
            env.insert(key.to_owned(), value);
        }
    }
    env.insert("HOME".to_owned(), home.display().to_string());
    env.insert(
        "CLOUDSDK_CONFIG".to_owned(),
        home.join("gcloud").display().to_string(),
    );
    env.insert(
        "XDG_CONFIG_HOME".to_owned(),
        home.join("config").display().to_string(),
    );
    env.insert("RUST_BACKTRACE".to_owned(), "1".to_owned());
    env
}

pub struct Process {
    pub name: String,
    program: PathBuf,
    args: Vec<String>,
    env: HashMap<String, String>,
    log_path: PathBuf,
    child: Option<Child>,
}

impl Process {
    pub fn spawn(
        name: &str,
        program: &str,
        args: &[&str],
        extra: &[(&str, &str)],
        directory: &Path,
    ) -> Self {
        let mut env = kept_env(directory);
        for (key, value) in extra {
            env.insert((*key).to_owned(), (*value).to_owned());
        }
        let mut process = Self {
            name: name.to_owned(),
            program: PathBuf::from(program),
            args: args.iter().map(|arg| (*arg).to_owned()).collect(),
            env,
            log_path: directory.join(format!("{name}.log")),
            child: None,
        };
        process.start();
        process
    }

    fn start(&mut self) {
        if let Some(parent) = self.log_path.parent() {
            std::fs::create_dir_all(parent).expect("log directory");
        }
        let log = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)
            .expect("log file");
        let mut command = Command::new(&self.program);
        command
            .args(&self.args)
            .env_clear()
            .envs(&self.env)
            .stdin(Stdio::null())
            .stdout(Stdio::from(log.try_clone().expect("clone log")))
            .stderr(Stdio::from(log))
            .process_group(0);
        self.child = Some(command.spawn().unwrap_or_else(|error| {
            panic!("spawn {} ({}): {error}", self.name, self.program.display())
        }));
    }

    /// Stops the process and starts it again with the same arguments and
    /// environment, appending to the same log.
    #[allow(dead_code)]
    pub fn restart(&mut self) {
        self.kill_now();
        self.start();
    }

    pub fn exited(&mut self) -> Option<i32> {
        self.child.as_mut().and_then(|child| {
            child
                .try_wait()
                .expect("child status")
                .map(|status| status.code().unwrap_or(1))
        })
    }

    pub fn tail(&self) -> String {
        let bytes = std::fs::read(&self.log_path).unwrap_or_default();
        let start = bytes.len().saturating_sub(4000);
        String::from_utf8_lossy(&bytes[start..]).into_owned()
    }

    pub fn kill_now(&mut self) {
        let Some(mut child) = self.child.take() else {
            return;
        };
        signal_group(child.id(), "-TERM");
        let deadline = Instant::now() + Duration::from_secs(2);
        while Instant::now() < deadline {
            if matches!(child.try_wait(), Ok(Some(_))) {
                return;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        signal_group(child.id(), "-KILL");
        let _ = child.wait();
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        self.kill_now();
    }
}

fn signal_group(pid: u32, signal: &str) {
    let _ = Command::new("kill")
        .args([signal, "--", &format!("-{pid}")])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

pub struct Valkey {
    port: u16,
    directory: PathBuf,
    process: Process,
}

impl Valkey {
    pub async fn start() -> Self {
        let port = free_port();
        let directory = std::env::temp_dir().join(format!(
            "physician-scale-valkey-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(&directory).expect("valkey directory");
        let mut server = Self {
            port,
            directory: directory.clone(),
            process: Self::launch(port, &directory),
        };
        server.wait_ready().await;
        server
    }

    pub fn url(&self) -> String {
        format!("redis://127.0.0.1:{}", self.port)
    }

    pub fn tail(&self) -> String {
        self.process.tail()
    }

    fn launch(port: u16, directory: &Path) -> Process {
        let port = port.to_string();
        Process::spawn(
            "valkey",
            "valkey-server",
            &[
                "--bind",
                "127.0.0.1",
                "--port",
                &port,
                "--save",
                "",
                "--appendonly",
                "no",
                "--protected-mode",
                "yes",
                "--dir",
                directory.to_str().expect("utf-8 temp dir"),
            ],
            &[],
            directory,
        )
    }

    fn spawn(&mut self) {
        self.process = Self::launch(self.port, &self.directory);
    }

    async fn wait_ready(&mut self) {
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            if let Some(code) = self.process.exited() {
                panic!("valkey exited ({code}): {}", self.process.tail());
            }
            if TcpStream::connect(("127.0.0.1", self.port)).await.is_ok() {
                let pong = Command::new("valkey-cli")
                    .args(["-h", "127.0.0.1", "-p", &self.port.to_string(), "ping"])
                    .output();
                if pong
                    .ok()
                    .is_some_and(|output| output.stdout.windows(4).any(|bytes| bytes == b"PONG"))
                {
                    return;
                }
            }
            assert!(
                Instant::now() < deadline,
                "valkey startup: {}",
                self.process.tail()
            );
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }

    pub async fn restart(&mut self) {
        self.process.kill_now();
        let deadline = Instant::now() + Duration::from_secs(5);
        while TcpStream::connect(("127.0.0.1", self.port)).await.is_ok() {
            assert!(Instant::now() < deadline, "valkey port stayed open");
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        self.spawn();
        self.wait_ready().await;
    }

    pub fn subscribers(&self) -> u64 {
        let output = Command::new("valkey-cli")
            .args([
                "-h",
                "127.0.0.1",
                "-p",
                &self.port.to_string(),
                "--raw",
                "PUBSUB",
                "NUMSUB",
                physician_bus::CHANNEL,
            ])
            .output()
            .expect("valkey-cli");
        output
            .stdout
            .split(|byte| *byte == b'\n')
            .nth(1)
            .and_then(|line| std::str::from_utf8(line).ok())
            .and_then(|line| line.trim().parse().ok())
            .unwrap_or(0)
    }
}

impl Drop for Valkey {
    fn drop(&mut self) {
        self.process.kill_now();
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

/// The physician chat-turn topology on the shared `pubsub::testing` emulator.
///
/// Always a fresh emulator on a port from this process's ledger, never a
/// reused `PUBSUB_EMULATOR_HOST`: these suites use fixed resource names.
pub struct Emulator {
    pub host: String,
    inner: pubsub::testing::Emulator,
}

impl Emulator {
    pub async fn start() -> Self {
        let inner = pubsub::testing::Emulator::spawn(PROJECT, free_port()).await;
        inner.create_topic("physician-chat-turns").await;
        Self {
            host: inner.host.clone(),
            inner,
        }
    }

    pub fn tail(&self) -> String {
        self.inner.tail()
    }

    pub async fn provision(&self, worker_port: u16) {
        use pubsub::testing::Subscription;
        let dead_letter = "physician-chat-turns-dead-letter";
        self.inner.create_topic(dead_letter).await;
        self.inner
            .create_subscription(dead_letter, &Subscription::pull(dead_letter))
            .await;
        self.inner
            .create_subscription(
                "physician-chat-turns",
                &Subscription::push(
                    "physician-chat-turns",
                    format!("http://127.0.0.1:{worker_port}/pubsub/chat-turn"),
                )
                .ack_deadline(60)
                .retry(1, 2)
                .dead_letter(dead_letter, 5),
            )
            .await;
    }

    pub async fn publish(&self, delivery: &Delivery) {
        let data = serde_json::to_vec(delivery).expect("delivery");
        self.inner
            .publish(&topic(), &[pubsub::testing::Outgoing::data(data)])
            .await;
    }

    pub async fn pull_dead_letters(&self) -> Vec<Vec<u8>> {
        self.inner
            .pull(
                &self
                    .inner
                    .subscription_path("physician-chat-turns-dead-letter"),
                10,
            )
            .await
            .into_iter()
            .map(|received| received.message.data)
            .collect()
    }
}

pub async fn exchange(
    host: &str,
    method: &str,
    path: &str,
    headers: &[(&str, &str)],
    body: Option<&[u8]>,
) -> Result<(u16, String), String> {
    let mut stream = TcpStream::connect(host)
        .await
        .map_err(|error| format!("connect {host}: {error}"))?;
    let mut request = format!("{method} {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n");
    for (name, value) in headers {
        request.push_str(name);
        request.push_str(": ");
        request.push_str(value);
        request.push_str("\r\n");
    }
    if let Some(body) = body {
        request.push_str(&format!("Content-Length: {}\r\n", body.len()));
    }
    request.push_str("\r\n");
    stream
        .write_all(request.as_bytes())
        .await
        .map_err(|error| error.to_string())?;
    if let Some(body) = body {
        stream
            .write_all(body)
            .await
            .map_err(|error| error.to_string())?;
    }
    let (status, bytes) = read_response(&mut stream, Duration::from_secs(20)).await?;
    Ok((status, String::from_utf8_lossy(&bytes).into_owned()))
}

async fn read_response(
    stream: &mut TcpStream,
    timeout: Duration,
) -> Result<(u16, Vec<u8>), String> {
    let deadline = Instant::now() + timeout;
    let mut buf = Vec::new();
    let mut tmp = [0u8; 4096];
    let header_end = loop {
        if Instant::now() >= deadline {
            return Err(format!("http timeout: {}", String::from_utf8_lossy(&buf)));
        }
        let remaining = deadline.saturating_duration_since(Instant::now());
        let read = tokio::time::timeout(remaining, stream.read(&mut tmp))
            .await
            .map_err(|_| "http timeout".to_owned())?;
        let n = read.map_err(|error| error.to_string())?;
        if n == 0 {
            return Err(format!("http closed: {}", String::from_utf8_lossy(&buf)));
        }
        buf.extend_from_slice(&tmp[..n]);
        if let Some(pos) = buf.windows(4).position(|window| window == b"\r\n\r\n") {
            break pos;
        }
    };
    let headers = String::from_utf8_lossy(&buf[..header_end]).into_owned();
    let status = headers
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse().ok())
        .ok_or_else(|| format!("bad status line: {headers}"))?;
    let mut body = buf[header_end + 4..].to_vec();
    let length = headers.lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        name.eq_ignore_ascii_case("content-length")
            .then(|| value.trim().parse::<usize>().ok())
            .flatten()
    });
    if let Some(length) = length {
        while body.len() < length {
            if Instant::now() >= deadline {
                return Err("http body timeout".into());
            }
            let n = stream
                .read(&mut tmp)
                .await
                .map_err(|error| error.to_string())?;
            if n == 0 {
                break;
            }
            body.extend_from_slice(&tmp[..n]);
        }
        body.truncate(length);
    }
    Ok((status, body))
}

pub struct Sse {
    stream: TcpStream,
    pub buffer: String,
}

impl Sse {
    pub async fn open(host: &str, path: &str, bearer: &str) -> Result<Self, String> {
        let mut stream = TcpStream::connect(host)
            .await
            .map_err(|error| error.to_string())?;
        let request = format!(
            "GET {path} HTTP/1.1\r\nHost: {host}\r\nAuthorization: Bearer {bearer}\r\nAccept: text/event-stream\r\n\r\n"
        );
        stream
            .write_all(request.as_bytes())
            .await
            .map_err(|error| error.to_string())?;
        let mut buf = Vec::new();
        let mut tmp = [0u8; 2048];
        let header_end = loop {
            let n = tokio::time::timeout(Duration::from_secs(10), stream.read(&mut tmp))
                .await
                .map_err(|_| "sse header timeout".to_owned())?
                .map_err(|error| error.to_string())?;
            if n == 0 {
                return Err(format!("sse closed: {}", String::from_utf8_lossy(&buf)));
            }
            buf.extend_from_slice(&tmp[..n]);
            if let Some(pos) = buf.windows(4).position(|window| window == b"\r\n\r\n") {
                break pos;
            }
        };
        let headers = String::from_utf8_lossy(&buf[..header_end]);
        let status: u16 = headers
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|code| code.parse().ok())
            .ok_or_else(|| format!("bad sse status: {headers}"))?;
        if status != 200 {
            return Err(format!("sse status {status}: {headers}"));
        }
        Ok(Self {
            stream,
            buffer: String::from_utf8_lossy(&buf[header_end + 4..]).into_owned(),
        })
    }

    pub async fn wait_for(&mut self, needle: &str, timeout: Duration) -> Result<(), String> {
        let deadline = Instant::now() + timeout;
        let mut tmp = [0u8; 2048];
        while !self.buffer.contains(needle) {
            if Instant::now() >= deadline {
                return Err(format!("missing {needle} in {}", self.buffer));
            }
            let remaining = deadline.saturating_duration_since(Instant::now());
            let n = tokio::time::timeout(remaining, self.stream.read(&mut tmp))
                .await
                .map_err(|_| format!("sse timeout; saw {}", self.buffer))?
                .map_err(|error| error.to_string())?;
            if n == 0 {
                return Err(format!("sse closed; saw {}", self.buffer));
            }
            self.buffer.push_str(&String::from_utf8_lossy(&tmp[..n]));
        }
        Ok(())
    }
}

pub async fn wait_ready(process: &mut Process, port: u16) {
    let host = format!("127.0.0.1:{port}");
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        if let Some(code) = process.exited() {
            panic!("{} exited ({code}): {}", process.name, process.tail());
        }
        if exchange(&host, "GET", "/readyz", &[], None)
            .await
            .is_ok_and(|(status, _)| status == 200)
        {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "timed out waiting for {} /readyz: {}",
            process.name,
            process.tail()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

pub fn push_body(delivery: &Delivery, message_id: &str) -> Vec<u8> {
    let data = base64::engine::general_purpose::STANDARD
        .encode(serde_json::to_vec(delivery).expect("delivery"));
    serde_json::to_vec(&json!({
        "message": {"data": data, "messageId": message_id},
        "subscription": format!("projects/{PROJECT}/subscriptions/physician-chat-turns")
    }))
    .expect("push body")
}

pub fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or_default()
}

/// Creates the staging physician the site's local sign-in accepts.
pub async fn staging_account(url: &str) {
    let auth = physician_auth::PostgresAuthStore::connect_without_migrations(url)
        .await
        .expect("auth store");
    auth.ensure_account(
        physician_sample::STAGING_PRACTICE_ID,
        "staging-physician",
        "physician@codahealth.test",
        "Staging Physician",
        true,
        unix_now(),
    )
    .await
    .expect("staging account");
}

/// Signs the staging physician in on a non-hosted site; returns the bearer.
pub async fn sign_in(host: &str) -> String {
    let (status, body) = post(
        host,
        "/auth/sign-in",
        None,
        &json!({ "email": "physician@codahealth.test" }),
    )
    .await;
    assert_eq!(status, 200, "{body}");
    let token = body["token"].as_str().expect("sign-in token").to_owned();
    let (status, body) = post(
        host,
        "/auth/verify",
        None,
        &json!({ "email": "physician@codahealth.test", "token": token }),
    )
    .await;
    assert_eq!(status, 200, "{body}");
    body["session"].as_str().expect("bearer session").to_owned()
}

pub async fn post(host: &str, path: &str, bearer: Option<&str>, body: &Value) -> (u16, Value) {
    let bytes = serde_json::to_vec(body).expect("json");
    let mut headers = vec![("content-type", "application/json")];
    let authorization;
    if let Some(bearer) = bearer {
        authorization = format!("Bearer {bearer}");
        headers.push(("authorization", authorization.as_str()));
    }
    let (status, text) = exchange(host, "POST", path, &headers, Some(&bytes))
        .await
        .expect("post");
    let value = serde_json::from_str(&text).unwrap_or(json!({ "raw": text }));
    (status, value)
}

pub async fn get_json(host: &str, path: &str, bearer: &str) -> (u16, Value) {
    let authorization = format!("Bearer {bearer}");
    let (status, text) = exchange(
        host,
        "GET",
        path,
        &[("authorization", authorization.as_str())],
        None,
    )
    .await
    .expect("get");
    let value = serde_json::from_str(&text).unwrap_or(json!({ "raw": text }));
    (status, value)
}

/// A `message.submit` command body for one new chat line.
pub fn submit(workspace: &str, chat: &str, text: &str) -> Value {
    json!({
        "type": "message.submit",
        "idempotency_key": format!("key-{}", uuid::Uuid::new_v4().simple()),
        "payload": {
            "patient_workspace_id": workspace,
            "chat_id": chat,
            "text": text,
            "physician_time_zone": "America/New_York"
        }
    })
}
