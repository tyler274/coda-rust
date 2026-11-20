//! Account-owned Android Health Connect connection, acquisition, and disconnect sync.

use crate::DuetClient;
use crate::health_upload::{
    FencedHealthStorage, RefreshStop, SyncBudget, SyncPause, acquire, open_outbox, sync_acquired,
};
use crate::portals::{PhoneSourceKind, PortalClientError};
use crate::retention::RetentionOwner;
use crate::session::SessionFence;
use duet_health::acquisition::PhoneSource;
use duet_health::native_adapters::{HealthConnectAdapter, HealthConnectBridge, HealthConnectScope};
use duet_health::outbox::{Outbox, OutboxError, OutboxLimits};
use std::future::Future;
use std::sync::Arc;

/// Failure crossing a platform or service boundary during a Health Connect operation.
#[derive(Debug, thiserror::Error)]
pub enum HealthConnectError {
    /// The sync owner stopped with retained work or incomplete coverage.
    #[error("Health Connect sync stopped")]
    Stopped(#[from] RefreshStop),
    /// The server refused the session.
    #[error("authentication refused for Health Connect")]
    SessionRefused(#[source] PortalClientError),
    /// Server-side generation or disconnect synchronization failed.
    #[error("Health Connect server synchronization failed")]
    Server(#[source] PortalClientError),
    /// Native Android permission launcher failed.
    #[error("Health Connect permission request failed: {0}")]
    Permission(String),
    /// The reading thread ended without returning its acquisition result.
    #[error("Health Connect reading thread exited")]
    Reader(#[source] futures_channel::oneshot::Canceled),
    /// The device clock could not provide an upload timestamp.
    #[error("device clock precedes Unix epoch")]
    Clock(#[from] std::time::SystemTimeError),
}

impl From<SyncPause> for HealthConnectError {
    fn from(pause: SyncPause) -> Self {
        Self::Stopped(pause.into())
    }
}

impl From<OutboxError> for HealthConnectError {
    fn from(error: OutboxError) -> Self {
        SyncPause::from(error).into()
    }
}

impl From<PortalClientError> for HealthConnectError {
    fn from(error: PortalClientError) -> Self {
        match error {
            PortalClientError::SessionRefused(_) => Self::SessionRefused(error),
            other => Self::Server(other),
        }
    }
}

/// Device contract needed by a Health Connect connection: bounded reads plus permission request.
pub trait HealthConnectDevice: HealthConnectBridge + Clone + Send + 'static {
    /// Request Health Connect permissions from Android's system sheet.
    fn authorize(&self) -> impl Future<Output = Result<(), String>>;
}

#[cfg(target_os = "android")]
impl HealthConnectDevice for duet_health::native_adapters::AndroidHealthConnectBridge {
    async fn authorize(&self) -> Result<(), String> {
        duet_health::request_health_connect_permission()
    }
}

/// Account-fenced Health Connect queue on an Android installation.
#[derive(Clone)]
pub struct HealthConnect<D> {
    queue: Arc<Outbox<FencedHealthStorage>>,
    device: D,
    scope: HealthConnectScope,
}

impl<D: HealthConnectDevice> HealthConnect<D> {
    /// Resolve the current patient's Health Connect queue on this installation
    /// using the default Google-approved type scope.
    pub fn open(
        retention: &RetentionOwner,
        fence: &SessionFence,
        device: D,
    ) -> Result<Self, OutboxError> {
        let installation = retention.health_installation(fence)?;
        Ok(Self {
            device,
            scope: HealthConnectScope::approved(),
            queue: Arc::new(open_outbox(
                retention,
                fence,
                &installation.to_string(),
                PhoneSource::HealthConnect,
                OutboxLimits::default(),
            )),
        })
    }

    /// Override the acquisition scope (e.g. narrowed to granted types or disabled).
    pub fn with_scope(mut self, scope: HealthConnectScope) -> Self {
        self.scope = scope;
        self
    }

    /// Whether the patient explicitly enabled this Health Connect queue.
    pub fn connected(&self) -> Result<bool, OutboxError> {
        self.queue.view().map(|v| v.connected)
    }

    /// Capture the current server generation before opening Android's permission
    /// sheet, connect locally, and notify `duet-api`.
    pub async fn connect(&self, client: &DuetClient) -> Result<(), HealthConnectError> {
        let generation = client
            .phone_source_generation(PhoneSourceKind::HealthConnect)
            .await?;
        self.device
            .authorize()
            .await
            .map_err(HealthConnectError::Permission)?;
        self.queue
            .connect(generation)
            .map_err(HealthConnectError::from)?;
        let _ = client
            .connect_phone_source(PhoneSourceKind::HealthConnect)
            .await?;
        Ok(())
    }

    /// Disconnect locally without erasing previously imported history.
    pub fn disconnect(&self) -> Result<(), HealthConnectError> {
        self.queue.disconnect().map_err(HealthConnectError::from)
    }

    /// Disconnect locally AND synchronize disconnect state to `duet-api` while
    /// keeping imported history intact (`intent/duet.md` D2.11).
    pub async fn disconnect_and_sync(&self, client: &DuetClient) -> Result<(), HealthConnectError> {
        self.queue.disconnect().map_err(HealthConnectError::from)?;
        let _ = client
            .disconnect_phone_source(PhoneSourceKind::HealthConnect)
            .await?;
        Ok(())
    }

    /// Acquire granted Health Connect records into the durable outbox and upload
    /// them to `/api/canonical-evidence/batch`.
    pub async fn refresh(&self, client: &DuetClient) -> Result<(), HealthConnectError> {
        let generation = client
            .phone_source_generation(PhoneSourceKind::HealthConnect)
            .await?;
        let state = self.queue.view()?;
        if state.source_generation != generation {
            self.disconnect()?;
            return Err(SyncPause::StaleGeneration.into());
        }
        let budget = SyncBudget::default();
        loop {
            let queue = self.queue.clone();
            let device = self.device.clone();
            let scope = self.scope.clone();
            let (tx, rx) = futures_channel::oneshot::channel();
            std::thread::spawn(move || {
                let adapter = HealthConnectAdapter::new(device).with_scope(scope);
                let _ = tx.send(acquire(&*queue, &adapter, budget.pages));
            });
            let acquired = rx.await.map_err(HealthConnectError::Reader)?;
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_millis() as u64;
            let report = sync_acquired(&self.queue, acquired, client, now, budget.batches).await;
            if report.progress(&self.queue)?.is_break() {
                return Ok(());
            }
        }
    }
}
