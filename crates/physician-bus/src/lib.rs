//! Physician-only, lossy projection invalidation. This is not a durable work queue.
//!
//! Consumers must refetch on `Resync` and on broadcast `RecvError::Lagged`.
//! Workspace IDs are routing hints, not authorization: filter them against the
//! authenticated physician's current authority before sending anything to a client.

use async_trait::async_trait;
use fred::{clients::SubscriberClient, prelude::*, types::RespVersion};
use serde::{Deserialize, Serialize};
use std::{
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::{sync::broadcast, task::JoinHandle};

/// No patient data, messages, or authorization claims belong on this channel.
pub const CHANNEL: &str = "physician:workspace-events:v1";
const CAPACITY: usize = 1024;
const MAX_PAYLOAD: usize = 4096;
const COMMAND_TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Event {
    WorkspaceChanged { workspace_id: String },
    WorkspaceSettled { workspace_id: String },
    Resync,
}

impl Event {
    fn validate(&self) -> Result<(), String> {
        match self {
            Self::WorkspaceChanged { workspace_id } | Self::WorkspaceSettled { workspace_id }
                if workspace_id.is_empty()
                    || workspace_id.len() > 512
                    || workspace_id.chars().any(char::is_whitespace)
                    || workspace_id.chars().any(char::is_control) =>
            {
                Err("invalid physician workspace ID".into())
            }
            _ => Ok(()),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope {
    version: u8,
    event: Event,
}

fn encode(event: Event) -> Result<String, String> {
    event.validate()?;
    serde_json::to_string(&Envelope { version: 1, event })
        .map_err(|_| "cannot encode physician bus event".into())
}

fn decode(payload: &[u8]) -> Result<Event, String> {
    if payload.len() > MAX_PAYLOAD {
        return Err("physician bus event too large".into());
    }
    let envelope: Envelope =
        serde_json::from_slice(payload).map_err(|_| "invalid physician bus event".to_string())?;
    if envelope.version != 1 {
        return Err("unsupported physician bus event version".into());
    }
    envelope.event.validate()?;
    Ok(envelope.event)
}

#[async_trait]
pub trait Bus: Send + Sync {
    /// Best-effort invalidation only. A successful publish is not durable delivery.
    async fn publish(&self, event: Event) -> Result<(), String>;
    /// A new receiver gets a resync hint; callers must also handle receiver lag.
    fn subscribe(&self) -> broadcast::Receiver<Event>;
    /// Whether the subscriber is connected and its subscription acknowledged.
    fn live(&self) -> bool;
}

pub struct LocalBus {
    events: broadcast::Sender<Event>,
}

impl LocalBus {
    pub fn new(capacity: usize) -> Self {
        Self {
            events: broadcast::channel(capacity.max(1)).0,
        }
    }
}

#[async_trait]
impl Bus for LocalBus {
    async fn publish(&self, event: Event) -> Result<(), String> {
        event.validate()?;
        let _ = self.events.send(event);
        Ok(())
    }

    fn subscribe(&self) -> broadcast::Receiver<Event> {
        subscribe_with_resync(&self.events)
    }

    fn live(&self) -> bool {
        true
    }
}

fn subscribe_with_resync(events: &broadcast::Sender<Event>) -> broadcast::Receiver<Event> {
    let receiver = events.subscribe();
    // Broadcast cannot seed just one receiver. Existing receivers may therefore
    // see an extra resync when another stream opens; resync is idempotent.
    let _ = events.send(Event::Resync);
    receiver
}

#[derive(Default)]
struct Tasks(Vec<JoinHandle<()>>);

impl Drop for Tasks {
    fn drop(&mut self) {
        for task in &self.0 {
            task.abort();
        }
    }
}

pub struct RedisBus {
    publisher: Client,
    subscriber: SubscriberClient,
    events: broadcast::Sender<Event>,
    live: Arc<AtomicBool>,
    // Abort on drop, including cancellation/failure during connect. No detached
    // retry loops or sockets should outlive the owning bus.
    _tasks: Tasks,
}

fn config(url: &str, ca: Option<&Path>) -> Result<Config, String> {
    // Restrict to one ordinary Redis endpoint, never a cluster/sentinel URL.
    if !url.starts_with("redis://") && !url.starts_with("rediss://") {
        return Err("physician Redis URL must use redis or rediss".into());
    }
    let mut config =
        Config::from_url(url).map_err(|_| "invalid physician Redis URL".to_string())?;
    // RESP3 allows a subscriber heartbeat without another subscriber connection.
    config.version = RespVersion::RESP3;
    if let Some(ca) = ca {
        if !url.starts_with("rediss://") {
            return Err("physician Redis CA requires rediss TLS".into());
        }
        use fred::rustls::{
            ClientConfig, RootCertStore,
            pki_types::{CertificateDer, pem::PemObject},
        };
        let pem = std::fs::read(ca).map_err(|_| "cannot read physician Redis CA".to_string())?;
        let mut roots = RootCertStore::empty();
        for cert in CertificateDer::pem_slice_iter(&pem) {
            roots
                .add(cert.map_err(|_| "invalid physician Redis CA".to_string())?)
                .map_err(|_| "invalid physician Redis CA".to_string())?;
        }
        if roots.is_empty() {
            return Err("empty physician Redis CA bundle".into());
        }
        config.tls = Some(
            ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth()
                .into(),
        );
    }
    // AUTH (including ACL username) is parsed by fred. Never expose its errors,
    // URL, credentials, or payload in application logs or returned diagnostics.
    Ok(config)
}

impl RedisBus {
    /// Connect publisher and subscriber, authenticating from the URL. `rediss`
    /// verifies the server certificate and name using `ca` (PEM bundle) or system
    /// roots. Hosted configuration must use `rediss` and the Memorystore CA.
    /// Initial failure returns an error; established connections retry forever
    /// with bounded exponential backoff. No credentials appear in our errors.
    pub async fn connect(url: &str, ca: Option<&Path>) -> Result<Self, String> {
        let config = config(url, ca)?;
        let mut builder = Builder::from_config(config);
        builder.set_policy(ReconnectPolicy::new_exponential(0, 100, 5_000, 2));
        builder.with_performance_config(|config| config.default_command_timeout = COMMAND_TIMEOUT);
        builder.with_connection_config(|config| config.connection_timeout = COMMAND_TIMEOUT);
        let publisher = builder
            .build()
            .map_err(|_| "cannot build physician Redis publisher")?;
        let subscriber = builder
            .build_subscriber_client()
            .map_err(|_| "cannot build physician Redis subscriber")?;
        let events = broadcast::channel(CAPACITY).0;
        let live = Arc::new(AtomicBool::new(false));
        let mut tasks = Tasks::default();
        // Register notification receivers before connecting: even a very short
        // disconnect must invalidate projections, not disappear between polls.
        let mut reconnects = subscriber.reconnect_rx();
        let mut errors = subscriber.error_rx();
        let mut messages = subscriber.message_rx();
        for connection in [publisher.connect(), subscriber.connect()] {
            // Aborting a wrapper would detach its child. Store a task that owns
            // an abort-on-drop guard for the actual fred connection handle.
            struct AbortConnection(fred::types::ConnectHandle);
            impl Drop for AbortConnection {
                fn drop(&mut self) {
                    self.0.abort();
                }
            }
            let mut guard = AbortConnection(connection);
            tasks.0.push(tokio::spawn(async move {
                let _ = (&mut guard.0).await;
            }));
        }
        tokio::time::timeout(Duration::from_secs(10), async {
            publisher.wait_for_connect().await?;
            subscriber.wait_for_connect().await?;
            subscriber.subscribe(CHANNEL).await
        })
        .await
        .map_err(|_| "physician Redis connection timed out")?
        .map_err(|_| "physician Redis connection or subscription failed")?;
        live.store(true, Ordering::Release);
        let listener = subscriber.clone();
        let output = events.clone();
        let healthy = live.clone();
        tasks.0.push(tokio::spawn(async move {
            let mut heartbeat = tokio::time::interval(Duration::from_secs(2));
            loop {
                tokio::select! {
                    result = reconnects.recv() => {
                        if matches!(result, Err(broadcast::error::RecvError::Closed)) { break; }
                        healthy.store(false, Ordering::Release);
                        let _ = output.send(Event::Resync);
                        // Restore explicitly, so live becomes true only after ACK.
                        if listener.subscribe(CHANNEL).await.is_ok() {
                            healthy.store(true, Ordering::Release);
                            let _ = output.send(Event::Resync);
                        }
                    }
                    result = errors.recv() => {
                        if matches!(result, Err(broadcast::error::RecvError::Closed)) { break; }
                        healthy.store(false, Ordering::Release);
                        let _ = output.send(Event::Resync);
                    }
                    result = messages.recv() => {
                        match result {
                            Ok(message) if message.channel == CHANNEL => {
                                let event = message.value.as_bytes().and_then(|value| decode(&value).ok())
                                    .unwrap_or(Event::Resync);
                                let _ = output.send(event);
                            }
                            Ok(_) => {}
                            Err(broadcast::error::RecvError::Lagged(_)) => { let _ = output.send(Event::Resync); }
                            Err(broadcast::error::RecvError::Closed) => break,
                        }
                    }
                    _ = heartbeat.tick() => {
                        if listener.ping::<String>(None).await.is_err() {
                            healthy.store(false, Ordering::Release);
                            let _ = output.send(Event::Resync);
                            let _ = listener.force_reconnection().await;
                        } else if !healthy.load(Ordering::Acquire)
                            && listener.subscribe(CHANNEL).await.is_ok() {
                            healthy.store(true, Ordering::Release);
                            let _ = output.send(Event::Resync);
                        }
                    }
                }
            }
            healthy.store(false, Ordering::Release);
            let _ = output.send(Event::Resync);
        }));
        Ok(Self {
            publisher,
            subscriber,
            events,
            live,
            _tasks: tasks,
        })
    }
}

#[async_trait]
impl Bus for RedisBus {
    async fn publish(&self, event: Event) -> Result<(), String> {
        let payload = encode(event)?;
        if !self.publisher.is_connected() {
            return Err("physician Redis publisher disconnected".into());
        }
        self.publisher
            .publish::<i64, _, _>(CHANNEL, payload)
            .await
            .map(|_| ())
            .map_err(|_| "physician Redis publish failed".into())
    }

    fn subscribe(&self) -> broadcast::Receiver<Event> {
        subscribe_with_resync(&self.events)
    }

    fn live(&self) -> bool {
        self.live.load(Ordering::Acquire) && self.subscriber.is_connected()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn local_fanout_initial_resync_and_lag() {
        let bus: Arc<dyn Bus> = Arc::new(LocalBus::new(2));
        let mut first = bus.subscribe();
        assert_eq!(first.recv().await.unwrap(), Event::Resync);
        let mut second = bus.subscribe();
        assert_eq!(first.recv().await.unwrap(), Event::Resync);
        assert_eq!(second.recv().await.unwrap(), Event::Resync);
        let event = Event::WorkspaceChanged {
            workspace_id: "workspace-1".into(),
        };
        bus.publish(event.clone()).await.unwrap();
        assert_eq!(first.recv().await.unwrap(), event);
        assert_eq!(second.recv().await.unwrap(), event);
        for _ in 0..4 {
            bus.publish(Event::Resync).await.unwrap();
        }
        assert!(matches!(
            first.recv().await,
            Err(broadcast::error::RecvError::Lagged(_))
        ));
        assert!(bus.live());
    }

    #[test]
    fn strict_versioned_wire_contract() {
        for event in [
            Event::Resync,
            Event::WorkspaceChanged {
                workspace_id: "workspace-1".into(),
            },
            Event::WorkspaceSettled {
                workspace_id: "workspace-2".into(),
            },
        ] {
            assert_eq!(
                decode(encode(event.clone()).unwrap().as_bytes()).unwrap(),
                event
            );
        }
        for payload in [
            r#"{"version":2,"event":{"type":"resync"}}"#,
            r#"{"version":1,"event":{"type":"workspace_changed","workspace_id":""}}"#,
            r#"{"version":1,"event":{"type":"resync","clinical_text":"not allowed"}}"#,
            r#"{"version":1,"event":{"type":"unknown"}}"#,
            r#"{"version":1,"event":{"type":"resync"},"extra":true}"#,
        ] {
            assert!(decode(payload.as_bytes()).is_err(), "{payload}");
        }
        assert!(decode(&vec![b' '; MAX_PAYLOAD + 1]).is_err());
        assert!(
            encode(Event::WorkspaceChanged {
                workspace_id: "a\nb".into()
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_unsafe_ca_and_redacts_url_errors() {
        assert!(config("redis://localhost", Some(Path::new("missing"))).is_err());
        let error = config("not-a-url:secret", None).err().unwrap();
        assert!(!error.contains("secret"));
        assert!(config("rediss://localhost", Some(Path::new("missing"))).is_err());
        let config = config(
            "redis://default:synthetic-test-password@localhost:6379",
            None,
        )
        .unwrap();
        assert_eq!(config.username.as_deref(), Some("default"));
        assert_eq!(config.password.as_deref(), Some("synthetic-test-password"));
    }
}
