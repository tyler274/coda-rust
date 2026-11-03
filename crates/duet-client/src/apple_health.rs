//! A foreground Apple Health connection belongs to the patient who requested it.
use crate::{
    DuetClient,
    health_upload::{
        FencedHealthStorage, RefreshStop, SyncBudget, SyncPause, acquire, authentication_refused,
        open_outbox, sync_acquired,
    },
    retention::RetentionOwner,
    session::SessionFence,
};
use duet_health::{
    AuthorizationError,
    acquisition::PhoneSource,
    native_adapters::{HealthKitAdapter, HealthKitBridge},
    outbox::{Outbox, OutboxError, OutboxLimits},
};
use std::{future::Future, sync::Arc};

type GenerationError =
    duet_wire::apis::Error<duet_wire::apis::default_api::ReadAppleHealthGenerationError>;

/// Failure crossing a platform or service boundary during an Apple Health operation.
#[derive(Debug, thiserror::Error)]
pub enum AppleHealthError {
    /// The sync owner stopped, with retained work or incomplete coverage.
    #[error("Apple Health sync stopped")]
    Stopped(#[from] RefreshStop),
    /// The server refused the session during the generation read.
    #[error("authentication refused for Apple Health generation")]
    SessionRefused(#[source] GenerationError),
    /// The authenticated generation read failed.
    #[error("reading the account’s Apple Health generation failed")]
    Generation(#[source] GenerationError),
    /// The native permission request did not finish successfully.
    #[error("HealthKit authorization failed")]
    Permission(#[from] AuthorizationError),
    /// The reading thread ended without returning its acquisition result.
    #[error("HealthKit reading thread exited")]
    Reader(#[source] futures_channel::oneshot::Canceled),
    /// The device clock could not provide an upload timestamp.
    #[error("device clock precedes Unix epoch")]
    Clock(#[from] std::time::SystemTimeError),
}

impl From<SyncPause> for AppleHealthError {
    fn from(pause: SyncPause) -> Self {
        Self::Stopped(pause.into())
    }
}

impl From<OutboxError> for AppleHealthError {
    fn from(error: OutboxError) -> Self {
        SyncPause::from(error).into()
    }
}

/// What a connection needs from the device: the bounded reads, and the
/// permission request that precedes them.
pub trait AppleHealthDevice: HealthKitBridge + Clone + Send + 'static {
    /// Wait for the permission sheet. Completion does not disclose which types were granted.
    fn authorize(&self) -> impl Future<Output = Result<(), AuthorizationError>>;
}

#[cfg(target_os = "ios")]
impl AppleHealthDevice for duet_health::native_adapters::IosHealthKitBridge {
    fn authorize(&self) -> impl Future<Output = Result<(), AuthorizationError>> {
        duet_health::request_apple_health()
    }
}

/// The connection on a real iPhone.
#[cfg(target_os = "ios")]
pub type IosAppleHealth = AppleHealth<duet_health::native_adapters::IosHealthKitBridge>;

#[cfg(target_os = "ios")]
impl IosAppleHealth {
    /// Resolve the current patient's queue on this iPhone.
    pub fn open_on_device(
        retention: &RetentionOwner,
        fence: &SessionFence,
    ) -> Result<Self, OutboxError> {
        Self::open(
            retention,
            fence,
            duet_health::native_adapters::IosHealthKitBridge,
        )
    }
}

/// The queue opened for the initiating session. Even a late HealthKit answer
/// must pass its storage fence before it can start an import.
#[derive(Clone)]
pub struct AppleHealth<D> {
    queue: Arc<Outbox<FencedHealthStorage>>,
    device: D,
}
impl<D: AppleHealthDevice> AppleHealth<D> {
    /// Resolve the current patient's queue on this installation.
    pub fn open(
        retention: &RetentionOwner,
        fence: &SessionFence,
        device: D,
    ) -> Result<Self, OutboxError> {
        let installation = retention.health_installation(fence)?;
        Ok(Self {
            device,
            queue: Arc::new(open_outbox(
                retention,
                fence,
                &installation.to_string(),
                PhoneSource::AppleHealth,
                OutboxLimits::default(),
            )),
        })
    }
    /// Only an explicit patient choice enables this queue. The OS grant alone never does.
    pub fn connected(&self) -> Result<bool, OutboxError> {
        self.queue.view().map(|v| v.connected)
    }
    /// Capture the generation before the permission sheet so a withdrawal
    /// while the sheet is open cannot authorize uploads under a later generation.
    pub async fn connect(&self, client: &DuetClient) -> Result<(), AppleHealthError> {
        let generation = generation(client).await?;
        self.device.authorize().await?;
        self.queue
            .connect(generation)
            .map_err(AppleHealthError::from)
    }
    /// Disconnect this installation without erasing previously imported records.
    pub fn disconnect(&self) -> Result<(), AppleHealthError> {
        self.queue.disconnect().map_err(AppleHealthError::from)
    }
    /// Read on a worker thread because HealthKit's bounded read is synchronous.
    /// Each durable write and each batch rechecks the original session.
    pub async fn refresh(&self, client: &DuetClient) -> Result<(), AppleHealthError> {
        let generation = generation(client).await?;
        let state = self.queue.view()?;
        if state.source_generation != generation {
            self.disconnect()?;
            return Err(SyncPause::StaleGeneration.into());
        }
        let budget = SyncBudget::default();
        loop {
            let queue = self.queue.clone();
            let device = self.device.clone();
            let (tx, rx) = futures_channel::oneshot::channel();
            std::thread::spawn(move || {
                let adapter = HealthKitAdapter::new(device);
                let _ = tx.send(acquire(&*queue, &adapter, budget.pages));
            });
            let acquired = rx.await.map_err(AppleHealthError::Reader)?;
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

async fn generation(client: &DuetClient) -> Result<i32, AppleHealthError> {
    duet_wire::apis::default_api::read_apple_health_generation(&client.configuration)
        .await
        .map_err(|error| match &error {
            duet_wire::apis::Error::ResponseError(response)
                if authentication_refused(response.status.as_u16()) =>
            {
                AppleHealthError::SessionRefused(error)
            }
            _ => AppleHealthError::Generation(error),
        })
}
