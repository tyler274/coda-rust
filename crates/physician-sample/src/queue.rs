//! Non-clinical wakeup transports. Postgres, not Pub/Sub, owns accepted work.
//!
//! Each publication opens a `physician.queue.publish` producer span and
//! carries its W3C trace context in the message attributes (`traceparent`,
//! `tracestate`), never in the payload, so the worker can continue or link to
//! the trace that published the wakeup.
use local_product::runtime::{
    async_trait,
    delivery::{Delivery, DeliveryKind},
};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::Instrument;

/// Transport of accepted input identities, never messages or session secrets.
#[async_trait]
pub trait ChatQueue: Send + Sync {
    /// Resolve only after the transport accepted the wakeup.
    async fn publish(&self, delivery: &Delivery) -> Result<(), String>;
}

/// Local execution adapter; durable recovery still owns process-crash retries.
#[async_trait]
pub trait InlineHandler: Send + Sync {
    /// Execute a wakeup through the same fenced worker as push delivery.
    async fn handle(&self, delivery: Delivery) -> Result<(), String>;
}

/// In-process fallback for local runs, not a hosted durability mechanism.
pub struct InlineQueue {
    handler: Arc<dyn InlineHandler>,
}
impl InlineQueue {
    /// Builds the explicitly selected local fallback.
    pub fn new(handler: Arc<dyn InlineHandler>) -> Self {
        Self { handler }
    }
}
#[async_trait]
impl ChatQueue for InlineQueue {
    async fn publish(&self, delivery: &Delivery) -> Result<(), String> {
        let handler = self.handler.clone();
        let delivery = delivery.clone();
        let span = publish_span(&delivery, "inline");
        // The spawned execution stays a child of the publishing span.
        tokio::spawn(
            async move {
                if handler.handle(delivery).await.is_err() {
                    tracing::warn!("inline physician execution requires recovery");
                }
            }
            .instrument(span),
        );
        Ok(())
    }
}

/// Official Google publisher using workload ADC; waits for server acceptance.
pub struct PubSubQueue {
    publisher: google_cloud_pubsub::client::Publisher,
}
impl PubSubQueue {
    /// Connects to Google using ADC, or an explicitly allowed loopback emulator.
    /// Hosted markers always prohibit emulator endpoints, including false/empty.
    pub async fn connect(
        topic: &str,
        emulator: Option<&str>,
        hosted: bool,
    ) -> Result<Self, String> {
        let publisher = pubsub::publisher(topic, emulator, hosted).await?;
        Ok(Self { publisher })
    }
}
#[async_trait]
impl ChatQueue for PubSubQueue {
    async fn publish(&self, delivery: &Delivery) -> Result<(), String> {
        let span = publish_span(delivery, "gcp_pubsub");
        let result = async {
            let message = message(delivery)?;
            self.publisher
                .publish(message)
                .await
                .map_err(|_| "Pub/Sub publication failed")?;
            Ok::<_, &'static str>(())
        }
        .instrument(span.clone())
        .await;
        if let Err(reason) = result {
            record_publish_failure(&span);
            return Err(reason.into());
        }
        Ok(())
    }
}

/// The broker message for `delivery`: its identity as data, and the current
/// span's W3C trace context as attributes.
pub fn message(delivery: &Delivery) -> Result<google_cloud_pubsub::model::Message, &'static str> {
    let bytes = serde_json::to_vec(delivery).map_err(|_| "invalid queue identity")?;
    let mut attributes = HashMap::new();
    telemetry::propagation::inject_current(&mut attributes);
    Ok(google_cloud_pubsub::model::Message::new()
        .set_data(bytes)
        .set_attributes(attributes))
}

/// `coda.delivery_kind`: `chat_turn` or `reply_revision`.
pub fn delivery_kind(kind: DeliveryKind) -> &'static str {
    match kind {
        DeliveryKind::ChatTurn => "chat_turn",
        DeliveryKind::ReplyRevision => "reply_revision",
    }
}

/// The producer span for one wakeup publication. Content-free: it names the
/// delivery kind, never the scope or input.
fn publish_span(delivery: &Delivery, system: &'static str) -> tracing::Span {
    tracing::info_span!(
        "physician.queue.publish",
        otel.kind = "producer",
        messaging.system = system,
        messaging.operation.type = "publish",
        coda.delivery_kind = delivery_kind(delivery.kind),
        otel.status_code = tracing::field::Empty,
        otel.status_description = tracing::field::Empty,
        error.type = tracing::field::Empty,
    )
}

/// Marks a publication failure with fixed, content-free fields. The caller
/// still treats the wakeup as recoverable from Postgres.
fn record_publish_failure(span: &tracing::Span) {
    span.record("otel.status_code", "ERROR");
    span.record("otel.status_description", "queue publication failed");
    span.record("error.type", "publish");
    tracing::error!(
        parent: span,
        event = "physician.queue.publish_failed",
        error.type = "publish",
        "physician queue publication failed"
    );
}

/// Accept only literal loopback IP endpoints; owned by `crates/pubsub`.
pub use pubsub::validate_emulator;
