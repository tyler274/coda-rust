//! Account-fenced phone acquisition and upload.
//!
//! The outbox state machine lives in `duet_health::outbox`; this module binds
//! it to protected, owner-scoped retention and to the canonical evidence batch
//! route. Every storage access presents the session fence captured when the
//! sync started, so sign-out or an account switch stops acquisition and upload
//! at the next step, and another account can never read or consume this queue.
use crate::retention::RetentionOwner;
use crate::session::SessionFence;
use duet_health::acquisition::HealthSourceAdapter;
use duet_health::outbox::{
    AcquireReport, FinishReport, ItemAck, Outbox, OutboxError, OutboxLimits, OutboxScope,
    OutboxStorage, RejectionCode, StorageError, UploadBatch, UploadOutcome,
};
use std::future::Future;

/// Outbox storage bound to one verified owner and session fence.
pub struct FencedHealthStorage {
    retention: RetentionOwner,
    fence: SessionFence,
    scope: String,
}

impl OutboxStorage for FencedHealthStorage {
    fn load(&self) -> Result<Option<Vec<u8>>, StorageError> {
        self.retention.read_health_outbox(&self.fence, &self.scope)
    }
    fn save(&self, bytes: &[u8]) -> Result<(), StorageError> {
        self.retention
            .write_health_outbox(&self.fence, &self.scope, bytes)
    }
}

/// Open the outbox for `fence`'s owner. The scope account is always the
/// fence owner; a caller cannot name another account.
pub fn open_outbox(
    retention: &RetentionOwner,
    fence: &SessionFence,
    installation: &str,
    source: duet_health::acquisition::PhoneSource,
    limits: OutboxLimits,
) -> Outbox<FencedHealthStorage> {
    let scope = OutboxScope {
        account: fence.owner().to_owned(),
        installation: installation.to_owned(),
        source,
    };
    let key = format!("{}:{installation}:{}", installation.len(), source.kind());
    Outbox::new(
        FencedHealthStorage {
            retention: retention.clone(),
            fence: fence.clone(),
            scope: key,
        },
        scope,
        limits,
    )
}

/// Delivers one canonical evidence batch.
pub trait EvidenceTransport {
    /// POST the batch body to `/api/canonical-evidence/batch`.
    fn send(&self, batch: &UploadBatch) -> impl Future<Output = UploadOutcome>;
}

/// Converts a generated batch result into per-item acknowledgements.
pub fn acks_from_wire(result: &duet_wire::models::CanonicalEvidenceBatchResult) -> Vec<ItemAck> {
    use duet_wire::models::canonical_evidence_item_outcome::Status;
    use duet_wire::models::canonical_evidence_rejection::Code;
    result
        .results
        .iter()
        .filter_map(|outcome| match outcome.status {
            Status::Accepted => outcome.address.as_ref().map(|address| ItemAck::Accepted {
                operation_id: outcome.operation_id,
                canonical_record_id: address.canonical_record_id,
                revision: i64::from(address.revision),
            }),
            Status::Rejected => outcome
                .rejection
                .as_ref()
                .map(|rejection| ItemAck::Rejected {
                    operation_id: outcome.operation_id,
                    code: match rejection.code {
                        Code::Conflict => RejectionCode::Conflict,
                        Code::SourceAuthorityRefused => RejectionCode::SourceAuthorityRefused,
                        Code::Invalid => RejectionCode::Invalid,
                        Code::NotFound => RejectionCode::NotFound,
                    },
                    message: rejection.message.clone(),
                }),
        })
        .collect()
}

pub(crate) fn authentication_refused(status: u16) -> bool {
    matches!(status, 401 | 403)
}

/// Map an HTTP status and body to an upload outcome. Only a 2xx body that
/// decodes completely is a delivery; anything ambiguous is uncertain.
pub fn outcome_from_http(status: u16, body: &[u8]) -> UploadOutcome {
    match status {
        200..=299 => {
            serde_json::from_slice::<duet_wire::models::CanonicalEvidenceBatchResult>(body)
                .map(|result| UploadOutcome::Delivered(acks_from_wire(&result)))
                .unwrap_or(UploadOutcome::Uncertain)
        }
        status if authentication_refused(status) => UploadOutcome::Unauthorized,
        _ => UploadOutcome::Uncertain,
    }
}

/// Real transport over the generated client.
impl EvidenceTransport for crate::DuetClient {
    async fn send(&self, batch: &UploadBatch) -> UploadOutcome {
        let Ok(body) = serde_json::from_value(batch.body()) else {
            return UploadOutcome::Uncertain;
        };
        match duet_wire::apis::default_api::ingest_canonical_evidence_batch(
            &self.configuration,
            body,
        )
        .await
        {
            Ok(result) => UploadOutcome::Delivered(acks_from_wire(&result)),
            Err(duet_wire::apis::Error::ResponseError(response)) => {
                outcome_from_http(response.status.as_u16(), response.content.as_bytes())
            }
            Err(_) => UploadOutcome::Uncertain,
        }
    }
}

/// Why a sync step stopped. Pending work is always retained.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SyncPause {
    /// The session changed (sign-out, account switch) or cleanup is pending.
    #[error("account changed")]
    AccountChanged,
    /// Protected storage is locked; resume when it unlocks.
    #[error("protected storage unavailable: {0}")]
    StorageUnavailable(String),
    /// The source is not connected for this account.
    #[error("source disconnected")]
    NotConnected,
    /// The outbox document is damaged and left untouched.
    #[error("damaged outbox")]
    Damaged,
    /// The server asked for authentication.
    #[error("authentication refused")]
    Unauthorized,
    /// The patient must reconnect to the current source generation.
    #[error("source generation changed")]
    StaleGeneration,
}

/// Why a foreground refresh could not drain all available work.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefreshStop {
    /// The bounded sync pass could not run to completion.
    #[error("sync pass paused")]
    Paused(#[from] SyncPause),
    /// The adapter could not complete its read.
    #[error("source read failed: {0}")]
    ReadFailed(String),
    /// One source type could not be read.
    #[error("incomplete coverage: {0:?}")]
    Coverage(duet_health::acquisition::TypeCoverage),
    /// A refused record is retained with its diagnostic.
    #[error("record rejected ({0:?}): {1}")]
    Rejected(RejectionCode, String),
    /// Work retained from a previous source generation.
    #[error("record fenced: {0:?}")]
    Fenced(duet_health::outbox::FenceReason),
    /// Delivery exhausted its retry budget.
    #[error("upload retry limit reached")]
    Parked,
    /// Accepted locally; awaiting another upload attempt.
    #[error("upload pending")]
    WaitingForUpload,
}

impl From<OutboxError> for SyncPause {
    fn from(error: OutboxError) -> Self {
        match error {
            OutboxError::Storage(StorageError::AccountChanged) => Self::AccountChanged,
            OutboxError::Storage(StorageError::Unavailable(cause)) => {
                Self::StorageUnavailable(cause)
            }
            OutboxError::NotConnected => Self::NotConnected,
            OutboxError::StaleGeneration => Self::StaleGeneration,
            OutboxError::Damaged => Self::Damaged,
        }
    }
}

/// Result of one sync pass.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SyncReport {
    /// Acquisition totals.
    pub acquired: AcquireReport,
    /// Upload totals.
    pub uploaded: FinishReport,
    /// Batches sent.
    pub batches: usize,
    /// Why the pass stopped early, if it did.
    pub paused: Option<SyncPause>,
}

/// Bounded work per pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SyncBudget {
    /// Maximum adapter pages read.
    pub pages: usize,
    /// Maximum batches sent.
    pub batches: usize,
}

impl Default for SyncBudget {
    fn default() -> Self {
        Self {
            pages: 20,
            batches: 20,
        }
    }
}

/// Drain the outbox: send ready batches and reconcile each acknowledgement
/// before the next batch. Stops on pause or when nothing is ready.
pub async fn upload<S: OutboxStorage, T: EvidenceTransport>(
    outbox: &Outbox<S>,
    transport: &T,
    now_ms: u64,
    max_batches: usize,
) -> Result<(FinishReport, usize), SyncPause> {
    let mut total = FinishReport::default();
    let mut batches = 0;
    while batches < max_batches {
        let Some(batch) = outbox.begin_upload(now_ms)? else {
            break;
        };
        batches += 1;
        let outcome = transport.send(&batch).await;
        let unauthorized = outcome == UploadOutcome::Unauthorized;
        let uncertain = outcome == UploadOutcome::Uncertain;
        let report = outbox.finish_upload(&batch, outcome, now_ms)?;
        total.acknowledged += report.acknowledged;
        total.rejected += report.rejected;
        total.retrying += report.retrying;
        total.parked += report.parked;
        if unauthorized {
            return Err(SyncPause::Unauthorized);
        }
        if uncertain {
            // Back off: the next pass resends the same operation ids.
            break;
        }
    }
    Ok((total, batches))
}

/// Result of one acquisition pass into the outbox.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AcquisitionOutcome {
    /// Acquisition totals. Everything counted here is durably in the outbox.
    pub acquired: AcquireReport,
    /// Why the pass stopped before writing, if it did. Nothing is lost.
    pub paused: Option<SyncPause>,
}

/// The single acquisition owner shared by foreground and background
/// triggers: read bounded adapter pages and commit each page (records,
/// deletions, coverage, anchor) to the outbox in one durable write. It never
/// touches the network.
pub fn acquire<S: OutboxStorage, A: HealthSourceAdapter + ?Sized>(
    outbox: &Outbox<S>,
    adapter: &A,
    max_pages: usize,
) -> AcquisitionOutcome {
    match outbox.acquire_all(adapter, max_pages) {
        Ok(acquired) => AcquisitionOutcome {
            acquired,
            paused: None,
        },
        Err(error) => AcquisitionOutcome {
            acquired: AcquireReport::default(),
            paused: Some(error.into()),
        },
    }
}

/// Background entry (e.g. an `HKObserverQuery` update). Runs [`acquire`] and
/// returns only after the durable outbox write, or after a typed pause such
/// as locked storage. It does not upload: the caller may complete the OS
/// callback immediately, and upload happens on the next foreground or
/// scheduled [`sync`].
pub fn background_acquire<S: OutboxStorage, A: HealthSourceAdapter + ?Sized>(
    outbox: &Outbox<S>,
    adapter: &A,
    max_pages: usize,
) -> (
    duet_health::native_adapters::BackgroundSignal,
    AcquisitionOutcome,
) {
    use duet_health::native_adapters::BackgroundSignal;
    let outcome = acquire(outbox, adapter, max_pages);
    let signal = if outcome.paused.is_some() {
        BackgroundSignal::Paused
    } else {
        BackgroundSignal::Durable
    };
    (signal, outcome)
}

/// One bounded acquire-then-upload pass for the current session.
pub async fn sync<S: OutboxStorage, A: HealthSourceAdapter + ?Sized, T: EvidenceTransport>(
    outbox: &Outbox<S>,
    adapter: &A,
    transport: &T,
    now_ms: u64,
    budget: SyncBudget,
) -> SyncReport {
    let acquired = acquire(outbox, adapter, budget.pages);
    sync_acquired(outbox, acquired, transport, now_ms, budget.batches).await
}

/// Finish one bounded pass after acquisition on the platform's reading thread.
pub async fn sync_acquired<S: OutboxStorage, T: EvidenceTransport>(
    outbox: &Outbox<S>,
    acquired: AcquisitionOutcome,
    transport: &T,
    now_ms: u64,
    max_batches: usize,
) -> SyncReport {
    let mut report = SyncReport {
        acquired: acquired.acquired,
        ..SyncReport::default()
    };
    if let Some(pause) = acquired.paused {
        report.paused = Some(pause);
        return report;
    }
    match upload(outbox, transport, now_ms, max_batches).await {
        Ok((uploaded, batches)) => {
            report.uploaded = uploaded;
            report.batches = batches;
        }
        Err(pause) => report.paused = Some(pause),
    }
    report
}

impl SyncReport {
    /// Decide whether a foreground refresh has drained its queue, needs another
    /// bounded pass, or stopped for a specific reason.
    pub fn progress<S: OutboxStorage>(
        self,
        outbox: &Outbox<S>,
    ) -> Result<std::ops::ControlFlow<()>, RefreshStop> {
        if let Some(reason) = self.paused {
            return Err(reason.into());
        }
        if let Some(reason) = self.acquired.read_error {
            return Err(RefreshStop::ReadFailed(reason));
        }
        let view = outbox.view().map_err(SyncPause::from)?;
        if let Some((data_type, outcome)) = view
            .coverage
            .into_iter()
            .find(|(_, outcome)| !outcome.read_succeeded())
        {
            return Err(RefreshStop::Coverage(
                duet_health::acquisition::TypeCoverage { data_type, outcome },
            ));
        }
        if self.uploaded.retrying > 0 {
            return Err(RefreshStop::WaitingForUpload);
        }
        let settled = self.uploaded.acknowledged + self.uploaded.rejected;
        if !view.items.is_empty() && settled == 0 {
            let parked = view
                .items
                .iter()
                .any(|item| item.state == duet_health::outbox::DeliveryState::Parked);
            return Err(if parked {
                RefreshStop::Parked
            } else {
                RefreshStop::WaitingForUpload
            });
        }
        if self.acquired.has_more || self.acquired.backpressure || !view.items.is_empty() {
            return Ok(std::ops::ControlFlow::Continue(()));
        }
        // Retained items outlive this pass, so they report only after the drain.
        if let Some(item) = view.rejected.into_iter().next() {
            return Err(RefreshStop::Rejected(item.code, item.message));
        }
        if let Some(item) = view.fenced.first() {
            return Err(RefreshStop::Fenced(item.reason));
        }
        Ok(std::ops::ControlFlow::Break(()))
    }
}
