//! Non-clinical wakeup transports. Postgres, not Pub/Sub, owns accepted work.
use local_product::runtime::{async_trait, delivery::Delivery};
use std::sync::Arc;

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
        tokio::spawn(async move {
            if handler.handle(delivery).await.is_err() {
                tracing::warn!("inline physician execution requires recovery");
            }
        });
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
        let bytes = serde_json::to_vec(delivery).map_err(|_| "invalid queue identity")?;
        self.publisher
            .publish(google_cloud_pubsub::model::Message::new().set_data(bytes))
            .await
            .map_err(|_| "Pub/Sub publication failed")?;
        Ok(())
    }
}

/// Accept only literal loopback IP endpoints; owned by `crates/pubsub`.
pub use pubsub::validate_emulator;
