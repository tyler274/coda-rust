//! Hosted acceptance survives a broker outage through the recovery sweep.
//!
//! `DurableChat::accept` saves the turn before it publishes the wakeup. When
//! the publish fails, the turn stays pending and due in Postgres, and
//! `recover_page` republishes exactly that delivery and marks it dispatched.
//! A duplicate accept is never published again; the first wakeup, or
//! recovery, owns delivery. Each test uses its own disposable database.

use std::sync::{Arc, Mutex};

use local_product::runtime::delivery::Delivery;
use local_product::runtime::{Accepted, Input, Scope, SendMode, Store};
use physician_sample::durable::DurableChat;
use physician_sample::queue::ChatQueue;
#[path = "scale/mod.rs"]
mod support;

fn lock() -> &'static tokio::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

/// Refuses its first `refusals` publishes, then records every accepted one.
struct FlakyQueue {
    state: Mutex<FlakyState>,
}

struct FlakyState {
    refusals: usize,
    attempts: usize,
    published: Vec<Delivery>,
}

impl FlakyQueue {
    fn new(refusals: usize) -> Arc<Self> {
        Arc::new(Self {
            state: Mutex::new(FlakyState {
                refusals,
                attempts: 0,
                published: Vec::new(),
            }),
        })
    }

    fn attempts(&self) -> usize {
        self.state.lock().expect("queue state").attempts
    }

    fn published(&self) -> Vec<Delivery> {
        self.state.lock().expect("queue state").published.clone()
    }
}

#[local_product::runtime::async_trait]
impl ChatQueue for FlakyQueue {
    async fn publish(&self, delivery: &Delivery) -> Result<(), String> {
        let mut state = self.state.lock().expect("queue state");
        state.attempts += 1;
        if state.refusals > 0 {
            state.refusals -= 1;
            return Err("broker unavailable".into());
        }
        state.published.push(delivery.clone());
        Ok(())
    }
}

struct Harness {
    // Dropped last: the database outlives the store's pool users below.
    _database: support::Database,
    store: Store,
    queue: Arc<FlakyQueue>,
    chat: DurableChat,
}

async fn harness(label: &str, refusals: usize) -> Option<Harness> {
    let database = support::Database::create(label).await?;
    let store = Store::connect(&database.url).await.expect("runtime store");
    let queue = FlakyQueue::new(refusals);
    let chat = DurableChat::new(store.clone(), queue.clone());
    Some(Harness {
        _database: database,
        store,
        queue,
        chat,
    })
}

fn turn() -> (Scope, Input, Delivery) {
    let unique = uuid::Uuid::new_v4().simple().to_string();
    let scope = Scope {
        practice: format!("practice-{unique}"),
        physician: format!("physician-{unique}"),
        workspace: format!("workspace-{unique}"),
        chat: format!("chat-{unique}"),
    };
    let input = Input {
        key: format!("input-{unique}"),
        text: "Hello".into(),
        timezone: "America/New_York".into(),
    };
    let delivery = Delivery::chat_turn(scope.clone(), input.key.clone());
    (scope, input, delivery)
}

/// Accepted session reference; the runtime only requires 32 bytes.
fn session() -> Vec<u8> {
    vec![7; 32]
}

#[tokio::test]
async fn recovery_failures_are_named_correlated_errors_without_content() {
    use opentelemetry::trace::{Status, TracerProvider as _};
    use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider};
    use tracing_subscriber::layer::SubscriberExt;
    let _guard = lock().lock().await;
    let Some(h) = harness("recovery_telemetry", 2).await else {
        return;
    };
    let exporter = InMemorySpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let subscriber = tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(provider.tracer("recovery-test")));
    let guard = tracing::subscriber::set_default(subscriber);
    let (scope, mut input, _) = turn();
    input.text = "SECRET-CLINICAL-CONTENT".into();
    let request = tracing::info_span!("accepted-request");
    use tracing::Instrument as _;
    assert_eq!(
        h.chat
            .accept(&scope, &input, session(), SendMode::Now)
            .instrument(request)
            .await
            .unwrap(),
        Accepted::New
    );
    h.chat
        .recover_page(None)
        .await
        .expect("partial failure remains recoverable");
    h.chat
        .recover_page(None)
        .await
        .expect("successful recovery");
    h.chat.recover_page(None).await.expect("empty sweep");
    h.chat
        .recover_revision_page(None)
        .await
        .expect("empty revision sweep");

    // Closing this pool deterministically fails real database reads without
    // changing schemas or relying on an external server outage.
    let connection = sea_orm::Database::connect(&h._database.url).await.unwrap();
    let unavailable = DurableChat::new(Store::from_connection(connection.clone()), h.queue.clone());
    connection.close().await.unwrap();
    assert!(unavailable.recover_page(None).await.is_err());
    assert!(unavailable.recover_revision_page(None).await.is_err());
    drop(guard);
    provider.force_flush().unwrap();
    let spans = exporter.get_finished_spans().unwrap();
    let accepted = spans.iter().find(|s| s.name == "accepted-request").unwrap();
    assert!(
        accepted.events.iter().any(|event| {
            [
                ("event", "physician.queue.publish_failed"),
                ("error.type", "publish"),
                ("level", "ERROR"),
            ]
            .iter()
            .all(|(key, value)| {
                event
                    .attributes
                    .iter()
                    .any(|kv| kv.key.as_str() == *key && kv.value.to_string() == *value)
            })
        }),
        "accepted work must still report publication failure"
    );
    let sweeps: Vec<_> = spans
        .iter()
        .filter(|s| s.name == "physician.recovery.sweep")
        .collect();
    assert_eq!(sweeps.len(), 6);
    for (span, failure) in sweeps.iter().zip([
        Some("publish"),
        None,
        None,
        None,
        Some("database"),
        Some("database"),
    ]) {
        assert_eq!(span.parent_span_id, opentelemetry::SpanId::INVALID);
        if let Some(class) = failure {
            assert!(matches!(span.status, Status::Error { .. }), "{span:?}");
            let event = span
                .events
                .iter()
                .find(|event| {
                    event.attributes.iter().any(|kv| {
                        kv.key.as_str() == "event"
                            && kv.value.to_string() == "physician.recovery.failed"
                    })
                })
                .expect("named recovery failure");
            for (key, value) in [("level", "ERROR"), ("error.type", class)] {
                assert!(
                    event
                        .attributes
                        .iter()
                        .any(|kv| kv.key.as_str() == key && kv.value.to_string() == value),
                    "{event:?}"
                );
            }
        } else {
            assert_eq!(span.status, Status::Unset);
            assert!(
                span.events.is_empty(),
                "successful or empty recovery stays quiet"
            );
        }
    }
    let dump = format!("{spans:?}");
    for secret in [
        &scope.practice,
        &scope.physician,
        &scope.workspace,
        &scope.chat,
        &input.key,
        &input.text,
    ] {
        assert!(!dump.contains(secret), "telemetry leaked {secret}");
    }
}

async fn due(store: &Store) -> Vec<Delivery> {
    store
        .dispatch_due(100, None)
        .await
        .expect("dispatch_due")
        .deliveries
}

#[tokio::test]
async fn failed_publish_is_republished_once_by_the_recovery_sweep() {
    let _guard = lock().lock().await;
    let Some(h) = harness("recover", 1).await else {
        return;
    };
    let (scope, input, delivery) = turn();

    let accepted = h
        .chat
        .accept(&scope, &input, session(), SendMode::Now)
        .await
        .expect("a failed publish must not fail acceptance");
    assert_eq!(accepted, Accepted::New);
    assert_eq!(h.queue.attempts(), 1, "accept should try the broker once");
    assert!(h.queue.published().is_empty(), "the broker refused it");

    let saved = h.chat.snapshot(&scope).await.expect("snapshot");
    assert_eq!(saved.status, "pending");
    assert_eq!(saved.input_id.as_deref(), Some(input.key.as_str()));
    assert_eq!(saved.dispatch_after, 0, "an unpublished turn stays due");
    assert_eq!(due(&h.store).await, std::slice::from_ref(&delivery));

    let next = h.chat.recover_page(None).await.expect("recovery sweep");
    assert_eq!(next, None, "one row fits in a single page");
    assert_eq!(
        h.queue.published(),
        std::slice::from_ref(&delivery),
        "recovery must republish exactly the delivery whose publish failed"
    );
    let dispatched = h.chat.snapshot(&scope).await.expect("snapshot");
    assert_eq!(dispatched.status, "pending", "recovery never runs the turn");
    assert!(
        dispatched.dispatch_after > 0,
        "a published recovery wakeup must be marked dispatched"
    );
    assert!(
        due(&h.store).await.is_empty(),
        "a dispatched turn is not due again until its retry window"
    );

    let next = h.chat.recover_page(None).await.expect("second sweep");
    assert_eq!(next, None);
    assert_eq!(
        h.queue.published(),
        [delivery],
        "an immediate second sweep must not republish a dispatched turn"
    );
    assert_eq!(h.queue.attempts(), 2);
}

#[tokio::test]
async fn failed_recovery_publish_stays_due_for_the_next_sweep() {
    let _guard = lock().lock().await;
    let Some(h) = harness("recover_retry", 2).await else {
        return;
    };
    let (scope, input, delivery) = turn();

    assert_eq!(
        h.chat
            .accept(&scope, &input, session(), SendMode::Now)
            .await
            .expect("accept"),
        Accepted::New
    );
    h.chat.recover_page(None).await.expect("refused sweep");
    assert_eq!(h.queue.attempts(), 2, "the sweep should try the broker");
    assert!(h.queue.published().is_empty());
    let saved = h.chat.snapshot(&scope).await.expect("snapshot");
    assert_eq!(
        saved.dispatch_after, 0,
        "a refused recovery publish must not be marked dispatched"
    );
    assert_eq!(due(&h.store).await, std::slice::from_ref(&delivery));

    h.chat.recover_page(None).await.expect("next sweep");
    assert_eq!(h.queue.published(), [delivery]);
    assert!(due(&h.store).await.is_empty());
}

#[tokio::test]
async fn duplicate_accept_is_never_published_again() {
    let _guard = lock().lock().await;
    let Some(h) = harness("recover_dup", 0).await else {
        return;
    };

    // First publish succeeds: the duplicate must not wake the worker again.
    let (scope, input, delivery) = turn();
    assert_eq!(
        h.chat
            .accept(&scope, &input, session(), SendMode::Now)
            .await
            .expect("accept"),
        Accepted::New
    );
    assert_eq!(h.queue.published(), std::slice::from_ref(&delivery));
    assert_eq!(
        h.chat
            .accept(&scope, &input, session(), SendMode::Now)
            .await
            .expect("duplicate accept"),
        Accepted::Duplicate
    );
    assert_eq!(h.queue.attempts(), 1, "a duplicate must not publish");
    assert_eq!(h.queue.published(), std::slice::from_ref(&delivery));
    let saved = h.chat.snapshot(&scope).await.expect("snapshot");
    assert_eq!(
        saved
            .messages
            .iter()
            .filter(|message| message.role == "user")
            .count(),
        1,
        "a duplicate must not append the input again"
    );
    h.chat.recover_page(None).await.expect("sweep");
    assert_eq!(
        h.queue.published(),
        [delivery],
        "a dispatched turn is not republished by recovery either"
    );
}

#[tokio::test]
async fn duplicate_accept_after_failed_publish_leaves_delivery_to_recovery() {
    let _guard = lock().lock().await;
    let Some(h) = harness("recover_dup_failed", 1).await else {
        return;
    };
    let (scope, input, delivery) = turn();
    assert_eq!(
        h.chat
            .accept(&scope, &input, session(), SendMode::Now)
            .await
            .expect("accept"),
        Accepted::New
    );
    assert_eq!(h.queue.attempts(), 1);
    assert_eq!(
        h.chat
            .accept(&scope, &input, session(), SendMode::Now)
            .await
            .expect("duplicate accept"),
        Accepted::Duplicate
    );
    assert_eq!(
        h.queue.attempts(),
        1,
        "a duplicate must not publish, even when the first publish failed"
    );
    assert_eq!(due(&h.store).await, std::slice::from_ref(&delivery));

    h.chat.recover_page(None).await.expect("sweep");
    assert_eq!(h.queue.published(), std::slice::from_ref(&delivery));
    h.chat.recover_page(None).await.expect("second sweep");
    assert_eq!(
        h.queue.published(),
        [delivery],
        "the turn is published exactly once overall"
    );
}
