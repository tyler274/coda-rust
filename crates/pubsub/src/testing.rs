//! A disposable Pub/Sub emulator for another crate's tests.
//!
//! [`Emulator::start`] reuses a loopback `PUBSUB_EMULATOR_HOST` when one is set,
//! so a suite can run under
//! `scripts/physician-scale-stack.py --dependencies-only -- <command>`, and
//! otherwise starts `gcloud beta emulators pubsub` from the Nix dev shell on a
//! loopback port. Without `gcloud` it returns `None`; when
//! `CODA_REQUIRE_POSTGRES_TESTS` is set (the Nix Postgres CI job) it panics
//! instead, so the proof cannot silently skip.
//!
//! Callers name their own topics and subscriptions. On a reused emulator,
//! everything an [`Emulator`] created is deleted when it drops. An emulator this
//! process started is killed with its process group, and its data directory is
//! removed.
//!
//! The emulator has no IAM: push requests carry no OIDC token, and nothing here
//! proves push authentication.
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use base64::Engine;
use serde_json::{Value, json};

use crate::envelope::{self, PushMessage};

const REQUIRE: &str = "CODA_REQUIRE_POSTGRES_TESTS";

/// One message to publish over the emulator's REST API.
#[derive(Debug, Clone, Default)]
pub struct Outgoing {
    /// Bytes to publish.
    pub data: Vec<u8>,
    /// Ordering key; the subscription must enable message ordering to honour it.
    pub ordering_key: Option<String>,
    /// Publisher attributes.
    pub attributes: BTreeMap<String, String>,
}

impl Outgoing {
    /// A message carrying only `data`.
    pub fn data(data: impl Into<Vec<u8>>) -> Self {
        Self {
            data: data.into(),
            ..Self::default()
        }
    }

    /// The same message with an ordering key.
    pub fn ordered(mut self, key: impl Into<String>) -> Self {
        self.ordering_key = Some(key.into());
        self
    }
}

/// How a subscription delivers.
#[derive(Debug, Clone)]
pub struct Subscription {
    /// Short topic name, in the emulator's project.
    pub topic: String,
    /// Push to this URL; `None` makes a pull subscription.
    pub push_endpoint: Option<String>,
    /// Ack deadline in seconds (Pub/Sub's minimum is 10).
    pub ack_deadline_seconds: u32,
    /// Retry backoff `(minimum, maximum)` in seconds.
    pub retry_backoff: Option<(u32, u32)>,
    /// Dead-letter `(short topic name, maxDeliveryAttempts)`. Pub/Sub's minimum
    /// is 5 attempts.
    pub dead_letter: Option<(String, u32)>,
    /// Deliver messages that share an ordering key in publish order.
    pub ordering: bool,
}

impl Subscription {
    /// A pull subscription with a 10 second ack deadline.
    pub fn pull(topic: impl Into<String>) -> Self {
        Self {
            topic: topic.into(),
            push_endpoint: None,
            ack_deadline_seconds: 10,
            retry_backoff: None,
            dead_letter: None,
            ordering: false,
        }
    }

    /// A push subscription to `endpoint` with a 10 second ack deadline.
    pub fn push(topic: impl Into<String>, endpoint: impl Into<String>) -> Self {
        Self {
            push_endpoint: Some(endpoint.into()),
            ..Self::pull(topic)
        }
    }

    /// Retry backoff bounds in seconds.
    pub fn retry(mut self, minimum: u32, maximum: u32) -> Self {
        self.retry_backoff = Some((minimum, maximum));
        self
    }

    /// Forward to `topic` after `max_attempts` deliveries.
    pub fn dead_letter(mut self, topic: impl Into<String>, max_attempts: u32) -> Self {
        self.dead_letter = Some((topic.into(), max_attempts));
        self
    }

    /// Enable message ordering.
    pub fn ordered(mut self) -> Self {
        self.ordering = true;
        self
    }

    /// Ack deadline in seconds.
    pub fn ack_deadline(mut self, seconds: u32) -> Self {
        self.ack_deadline_seconds = seconds;
        self
    }
}

/// One pulled message and the id that acks it.
#[derive(Debug, Clone)]
pub struct Received {
    /// Acks or extends this delivery only.
    pub ack_id: String,
    /// Which delivery this is, present when the subscription dead-letters.
    pub delivery_attempt: Option<u32>,
    /// The decoded message.
    pub message: PushMessage,
}

/// A loopback Pub/Sub emulator, started here or reused from the environment.
pub struct Emulator {
    /// `127.0.0.1:port` of the emulator.
    pub host: String,
    project: String,
    http: reqwest::Client,
    owned: Option<Owned>,
    created: Mutex<Vec<String>>,
}

struct Owned {
    child: Option<Child>,
    directory: PathBuf,
    log: PathBuf,
}

impl Emulator {
    /// Reuses a loopback `PUBSUB_EMULATOR_HOST`, or starts an emulator when
    /// `gcloud` is on `PATH`. `None` means skip; under
    /// `CODA_REQUIRE_POSTGRES_TESTS` a missing `gcloud` panics instead.
    pub async fn start(project: &str) -> Option<Self> {
        if let Some(host) = std::env::var("PUBSUB_EMULATOR_HOST")
            .ok()
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
        {
            crate::validate_emulator(&host, false)
                .unwrap_or_else(|error| panic!("PUBSUB_EMULATOR_HOST={host}: {error}"));
            let mut emulator = Self {
                host,
                project: project.to_owned(),
                http: http_client(),
                owned: None,
                created: Mutex::new(Vec::new()),
            };
            emulator.wait_ready().await;
            return Some(emulator);
        }
        if !on_path("gcloud") {
            assert!(
                std::env::var_os(REQUIRE).is_none(),
                "gcloud with the pubsub-emulator component from the Nix dev shell is required for the Pub/Sub proof"
            );
            return None;
        }
        Some(Self::spawn(project, free_port()).await)
    }

    /// Starts a new emulator on `127.0.0.1:port` and waits for its REST API.
    /// Panics when it cannot start.
    pub async fn spawn(project: &str, port: u16) -> Self {
        let host = format!("127.0.0.1:{port}");
        let directory = std::env::temp_dir().join(format!(
            "pubsub-emulator-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4().simple()
        ));
        // gcloud writes env.yaml into --data-dir and does not create it.
        std::fs::create_dir_all(directory.join("data")).expect("emulator directory");
        let log = directory.join("pubsub.log");
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log)
            .expect("emulator log");
        let child = Command::new("gcloud")
            .args([
                "beta",
                "emulators",
                "pubsub",
                "start",
                &format!("--host-port={host}"),
                &format!("--project={project}"),
                &format!("--data-dir={}", directory.join("data").display()),
                "--quiet",
            ])
            .env_clear()
            .envs(kept_env(&directory))
            .stdin(Stdio::null())
            .stdout(Stdio::from(file.try_clone().expect("clone log")))
            .stderr(Stdio::from(file))
            .process_group(0)
            .spawn()
            .unwrap_or_else(|error| panic!("spawn gcloud pubsub emulator: {error}"));
        let mut emulator = Self {
            host,
            project: project.to_owned(),
            http: http_client(),
            owned: Some(Owned {
                child: Some(child),
                directory,
                log,
            }),
            created: Mutex::new(Vec::new()),
        };
        emulator.wait_ready().await;
        emulator
    }

    async fn wait_ready(&mut self) {
        let deadline = Instant::now() + Duration::from_secs(90);
        loop {
            if let Some(owned) = &mut self.owned
                && let Some(code) = owned.exited()
            {
                panic!("pubsub emulator exited ({code}): {}", self.tail());
            }
            let probe = self
                .http
                .get(self.url(&format!("projects/{}/topics", self.project)))
                .send()
                .await;
            if probe.is_ok_and(|response| response.status().is_success()) {
                return;
            }
            assert!(
                Instant::now() < deadline,
                "pubsub emulator {} not ready: {}",
                self.host,
                self.tail()
            );
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// The project resources are created in.
    pub fn project(&self) -> &str {
        &self.project
    }

    /// Whether this emulator was reused from `PUBSUB_EMULATOR_HOST`.
    pub fn reused(&self) -> bool {
        self.owned.is_none()
    }

    /// The last few kilobytes of the emulator log, or a note for a reused one.
    pub fn tail(&self) -> String {
        let Some(owned) = &self.owned else {
            return format!("(reused emulator {}; its log is the stack's)", self.host);
        };
        let bytes = std::fs::read(&owned.log).unwrap_or_default();
        let start = bytes.len().saturating_sub(4000);
        String::from_utf8_lossy(&bytes[start..]).into_owned()
    }

    /// `projects/<project>/topics/<name>`.
    pub fn topic_path(&self, name: &str) -> String {
        format!("projects/{}/topics/{name}", self.project)
    }

    /// `projects/<project>/subscriptions/<name>`.
    pub fn subscription_path(&self, name: &str) -> String {
        format!("projects/{}/subscriptions/{name}", self.project)
    }

    fn url(&self, resource: &str) -> String {
        format!("http://{}/v1/{resource}", self.host)
    }

    /// PUTs `body` to a fully qualified resource and asserts success. The
    /// resource is deleted on drop when the emulator is reused.
    pub async fn put(&self, resource: &str, body: &Value) {
        let (status, text) = self.call("PUT", resource, body).await;
        assert!(
            (200..300).contains(&status),
            "create {resource}: {status} {text}\n{}",
            self.tail()
        );
        self.created
            .lock()
            .expect("created resources")
            .push(resource.to_owned());
    }

    async fn call(&self, method: &str, resource: &str, body: &Value) -> (u16, String) {
        let method = reqwest::Method::from_bytes(method.as_bytes()).expect("http method");
        let response = self
            .http
            .request(method, self.url(resource))
            .json(body)
            .send()
            .await
            .unwrap_or_else(|error| panic!("pubsub {resource}: {error}\n{}", self.tail()));
        let status = response.status().as_u16();
        (status, response.text().await.unwrap_or_default())
    }

    /// Creates a topic by short name and returns its full path.
    pub async fn create_topic(&self, name: &str) -> String {
        let path = self.topic_path(name);
        self.put(&path, &json!({})).await;
        path
    }

    /// Creates a subscription by short name and returns its full path.
    pub async fn create_subscription(&self, name: &str, config: &Subscription) -> String {
        let path = self.subscription_path(name);
        let mut body = json!({
            "topic": self.topic_path(&config.topic),
            "ackDeadlineSeconds": config.ack_deadline_seconds,
            "enableMessageOrdering": config.ordering,
        });
        if let Some(endpoint) = &config.push_endpoint {
            body["pushConfig"] = json!({ "pushEndpoint": endpoint });
        }
        if let Some((minimum, maximum)) = config.retry_backoff {
            body["retryPolicy"] = json!({
                "minimumBackoff": format!("{minimum}s"),
                "maximumBackoff": format!("{maximum}s"),
            });
        }
        if let Some((topic, attempts)) = &config.dead_letter {
            body["deadLetterPolicy"] = json!({
                "deadLetterTopic": self.topic_path(topic),
                "maxDeliveryAttempts": attempts,
            });
        }
        self.put(&path, &body).await;
        path
    }

    /// Publishes through the REST API to a fully qualified topic and returns
    /// the message ids in order.
    pub async fn publish(&self, topic: &str, messages: &[Outgoing]) -> Vec<String> {
        let messages: Vec<Value> = messages
            .iter()
            .map(|message| {
                let mut value = json!({
                    "data": base64::engine::general_purpose::STANDARD.encode(&message.data),
                });
                if let Some(key) = &message.ordering_key {
                    value["orderingKey"] = json!(key);
                }
                if !message.attributes.is_empty() {
                    value["attributes"] = json!(message.attributes);
                }
                value
            })
            .collect();
        let (status, text) = self
            .call(
                "POST",
                &format!("{topic}:publish"),
                &json!({ "messages": messages }),
            )
            .await;
        assert!(
            (200..300).contains(&status),
            "publish: {status} {text}\n{}",
            self.tail()
        );
        serde_json::from_str::<Value>(&text)
            .ok()
            .and_then(|value| value.get("messageIds").cloned())
            .and_then(|ids| serde_json::from_value(ids).ok())
            .unwrap_or_default()
    }

    /// Pulls up to `max` messages without waiting, and does not ack them.
    pub async fn pull(&self, subscription: &str, max: u32) -> Vec<Received> {
        let (status, text) = self
            .call(
                "POST",
                &format!("{subscription}:pull"),
                &json!({ "maxMessages": max, "returnImmediately": true }),
            )
            .await;
        assert!(
            (200..300).contains(&status),
            "pull {subscription}: {status} {text}\n{}",
            self.tail()
        );
        let value: Value = serde_json::from_str(&text).unwrap_or(json!({}));
        value
            .get("receivedMessages")
            .and_then(Value::as_array)
            .map(|messages| {
                messages
                    .iter()
                    .filter_map(|received| {
                        Some(Received {
                            ack_id: received.get("ackId")?.as_str()?.to_owned(),
                            delivery_attempt: received
                                .get("deliveryAttempt")
                                .and_then(Value::as_u64)
                                .and_then(|attempt| u32::try_from(attempt).ok()),
                            message: envelope::decode_message(received.get("message")?).ok()?,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Acknowledges deliveries; they are not redelivered.
    pub async fn ack(&self, subscription: &str, ack_ids: &[String]) {
        let (status, text) = self
            .call(
                "POST",
                &format!("{subscription}:acknowledge"),
                &json!({ "ackIds": ack_ids }),
            )
            .await;
        assert!(
            (200..300).contains(&status),
            "ack: {status} {text}\n{}",
            self.tail()
        );
    }

    /// Sets the ack deadline of deliveries. `0` is a nack: redeliver now.
    pub async fn modify_ack_deadline(&self, subscription: &str, ack_ids: &[String], seconds: u32) {
        let (status, text) = self
            .call(
                "POST",
                &format!("{subscription}:modifyAckDeadline"),
                &json!({ "ackIds": ack_ids, "ackDeadlineSeconds": seconds }),
            )
            .await;
        assert!(
            (200..300).contains(&status),
            "modifyAckDeadline: {status} {text}\n{}",
            self.tail()
        );
    }

    /// The official Google publisher for a fully qualified topic on this
    /// emulator, with anonymous credentials (see [`crate::publisher`]).
    pub async fn publisher(&self, topic: &str) -> google_cloud_pubsub::client::Publisher {
        crate::publisher(topic, Some(&self.host), false)
            .await
            .unwrap_or_else(|error| panic!("publisher for {topic}: {error}"))
    }
}

impl Drop for Emulator {
    fn drop(&mut self) {
        if let Some(owned) = &mut self.owned {
            owned.kill();
            let _ = std::fs::remove_dir_all(&owned.directory);
            return;
        }
        // A reused emulator outlives this test; leave it as it was found.
        let created = std::mem::take(&mut *self.created.lock().expect("created resources"));
        for resource in created.iter().rev() {
            delete_blocking(&self.host, &format!("/v1/{resource}"));
        }
    }
}

impl Owned {
    fn exited(&mut self) -> Option<i32> {
        self.child.as_mut().and_then(|child| {
            child
                .try_wait()
                .ok()
                .flatten()
                .map(|status| status.code().unwrap_or(1))
        })
    }

    fn kill(&mut self) {
        let Some(mut child) = self.child.take() else {
            return;
        };
        // gcloud runs the emulator as a Java child; signal the whole group.
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

fn signal_group(pid: u32, signal: &str) {
    let _ = Command::new("kill")
        .args([signal, "--", &format!("-{pid}")])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

/// A synchronous `DELETE` for `Drop`, which cannot await.
fn delete_blocking(host: &str, path: &str) {
    let Ok(mut stream) = std::net::TcpStream::connect(host) else {
        return;
    };
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let request = format!(
        "DELETE {path} HTTP/1.1\r\nHost: {host}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
    );
    if stream.write_all(request.as_bytes()).is_ok() {
        let mut sink = Vec::new();
        let _ = stream.read_to_end(&mut sink);
    }
}

fn http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .no_proxy()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(20))
        .build()
        .expect("pubsub emulator http client")
}

/// Only what the emulator needs; never Doppler, ADC or database variables.
fn kept_env(home: &Path) -> Vec<(String, String)> {
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
    let mut env: Vec<(String, String)> = KEEP
        .iter()
        .filter_map(|key| Some(((*key).to_owned(), std::env::var(key).ok()?)))
        .collect();
    env.push(("HOME".into(), home.display().to_string()));
    env.push((
        "CLOUDSDK_CONFIG".into(),
        home.join("gcloud").display().to_string(),
    ));
    env.push((
        "XDG_CONFIG_HOME".into(),
        home.join("config").display().to_string(),
    ));
    env
}

fn on_path(program: &str) -> bool {
    std::env::var_os("PATH").is_some_and(|path| {
        std::env::split_paths(&path).any(|directory| directory.join(program).is_file())
    })
}

/// A loopback port for the emulator to bind.
///
/// Binding `127.0.0.1:0` and handing the number to a child is racy: the kernel
/// draws it from the ephemeral range, which outbound connections also draw
/// from. Ports here come from below that range (Linux defaults to
/// 32768-60999), are never handed out twice by this process, and are probed
/// before use. A caller with its own port ledger passes its port to
/// [`Emulator::spawn`] instead.
pub fn free_port() -> u16 {
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
