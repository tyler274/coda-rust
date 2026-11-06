//! Wakeup publication carries its trace context on the wire, and a failed
//! publication is recorded without content.
//!
//! Runs the real `PubSubQueue` against the Pub/Sub emulator from the Nix dev
//! shell. It skips when the emulator component is unavailable, and fails
//! under `CODA_REQUIRE_POSTGRES_TESTS`.

use local_product::runtime::Scope;
use local_product::runtime::delivery::Delivery;
use opentelemetry::trace::{SpanKind, Status, TracerProvider as _};
use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider, SpanData};
use physician_sample::queue::{ChatQueue, PubSubQueue};
use pubsub::testing::{Emulator, Subscription};
use tracing_subscriber::layer::SubscriberExt;

const PROJECT: &str = "coda-queue-trace";
/// Stands in for clinical content: it must reach the message data only.
const MARKER: &str = "synthetic-content-marker";

fn attribute(span: &SpanData, key: &str) -> Option<String> {
    span.attributes
        .iter()
        .find(|kv| kv.key.as_str() == key)
        .map(|kv| kv.value.to_string())
}

fn delivery() -> Delivery {
    Delivery::chat_turn(
        Scope {
            practice: format!("{MARKER}-practice"),
            physician: format!("{MARKER}-physician"),
            workspace: format!("{MARKER}-workspace"),
            chat: format!("{MARKER}-chat"),
        },
        format!("{MARKER}-input"),
    )
}

#[tokio::test]
async fn publication_injects_its_trace_context_and_fails_without_content() {
    let Some(emulator) = Emulator::start(PROJECT).await else {
        return;
    };
    let topic = emulator.create_topic("wakeups").await;
    let subscription = emulator
        .create_subscription("wakeups-pull", &Subscription::pull("wakeups"))
        .await;
    let published = PubSubQueue::connect(&topic, Some(&emulator.host), false)
        .await
        .expect("emulator publisher");
    let missing =
        PubSubQueue::connect(&emulator.topic_path("missing"), Some(&emulator.host), false)
            .await
            .expect("publisher for a missing topic");

    let exporter = InMemorySpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let subscriber = tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(provider.tracer("queue-trace")));
    let guard = tracing::subscriber::set_default(subscriber);
    let delivery = delivery();
    published.publish(&delivery).await.expect("published");
    let refused = missing.publish(&delivery).await;
    drop(guard);
    provider.force_flush().unwrap();
    let spans = exporter.get_finished_spans().unwrap();

    let received = emulator.pull(&subscription, 10).await;
    assert_eq!(received.len(), 1, "one wakeup: {}", emulator.tail());
    let message = &received[0].message;
    assert_eq!(
        serde_json::from_slice::<Delivery>(&message.data).unwrap(),
        delivery
    );

    let publishes: Vec<_> = spans
        .iter()
        .filter(|span| span.name == "physician.queue.publish")
        .collect();
    assert_eq!(publishes.len(), 2, "{spans:?}");
    let ok = publishes
        .iter()
        .find(|span| span.status == Status::Unset)
        .expect("successful publication span");
    assert_eq!(ok.span_kind, SpanKind::Producer);
    for (key, value) in [
        ("messaging.system", "gcp_pubsub"),
        ("messaging.operation.type", "publish"),
        ("coda.delivery_kind", "chat_turn"),
    ] {
        assert_eq!(attribute(ok, key).as_deref(), Some(value), "{key}");
    }
    // The broker attribute names the publication span itself.
    let expected = format!(
        "00-{}-{}-01",
        ok.span_context.trace_id(),
        ok.span_context.span_id()
    );
    assert_eq!(
        message.attributes.get("traceparent"),
        Some(&expected),
        "{:?}",
        message.attributes
    );
    let attributes: std::collections::HashMap<_, _> =
        message.attributes.clone().into_iter().collect();
    let remote = telemetry::propagation::remote_context(&attributes).expect("valid context");
    assert_eq!(remote.span_id(), ok.span_context.span_id());
    // Trace context never travels in the payload.
    assert!(!String::from_utf8_lossy(&message.data).contains("traceparent"));

    // A refused publication is an error with fixed fields only.
    assert_eq!(refused, Err("Pub/Sub publication failed".to_owned()));
    let failed = publishes
        .iter()
        .find(|span| span.status != Status::Unset)
        .expect("failed publication span");
    assert_eq!(failed.status, Status::error("queue publication failed"));
    assert_eq!(attribute(failed, "error.type").as_deref(), Some("publish"));
    assert!(
        failed
            .events
            .iter()
            .any(|event| event.attributes.iter().any(|kv| {
                kv.key.as_str() == "event"
                    && kv.value.to_string() == "physician.queue.publish_failed"
            }))
    );
    assert!(
        failed.events.iter().any(|event| event
            .attributes
            .iter()
            .any(|kv| { kv.key.as_str() == "level" && kv.value.to_string() == "ERROR" })),
        "publication failures must be alertable errors"
    );
    let recorded = format!("{spans:?}");
    assert!(
        !recorded.contains(MARKER),
        "publication telemetry carried delivery content: {recorded}"
    );
}
