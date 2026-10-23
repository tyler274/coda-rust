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
        if !topic.starts_with("projects/") || !topic.contains("/topics/") {
            return Err("Pub/Sub requires a fully qualified topic".into());
        }
        let mut builder = google_cloud_pubsub::client::Publisher::builder(topic);
        if let Some(endpoint) = emulator {
            validate_emulator(endpoint, hosted)?;
            builder = builder
                .with_endpoint(format!("http://{endpoint}"))
                .with_credentials(
                    google_cloud_auth::credentials::anonymous::Builder::new().build(),
                );
        }
        let publisher = builder
            .build()
            .await
            .map_err(|_| "Pub/Sub publisher initialization failed")?;
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

/// Accept only literal loopback IP endpoints; no DNS or URL credential tricks.
pub fn validate_emulator(endpoint: &str, hosted: bool) -> Result<(), String> {
    if hosted {
        return Err("Pub/Sub emulator is forbidden in hosted execution".into());
    }
    let address: std::net::SocketAddr = endpoint
        .parse()
        .map_err(|_| "emulator must be a loopback IP:port")?;
    if !address.ip().is_loopback() || address.port() == 0 {
        return Err("emulator must be a loopback IP:port".into());
    }
    Ok(())
}
