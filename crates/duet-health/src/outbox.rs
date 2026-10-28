//! Duet-owned transactional acquisition outbox.
//!
//! One outbox exists per (account, installation, source). It records the
//! current source generation, the committed acquisition anchor, pending
//! canonical mutations, the server identities learned from acknowledgements,
//! retained rejections and fenced stale work. Every state transition is a
//! single atomic replacement of one serialized document, so acquired records
//! and their next anchor are committed together, and an acknowledgement is
//! reconciled (ledger updated) in the same write that removes the pending item.
//!
//! The outbox is a pure state machine over [`OutboxStorage`]; it never talks to
//! the network. Callers take a batch with [`Outbox::begin_upload`], deliver it,
//! then report the result with [`Outbox::finish_upload`]. A crash between
//! those steps leaves the items in flight; the next batch resends the exact
//! frozen mutation with the same `operation_id`.

use crate::acquisition::{
    AcquisitionPage, AdapterError, Anchor, CoverageError, CoverageOutcome, HealthSourceAdapter,
    PageRequest, PhoneSource,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

/// Serialized document version.
const VERSION: u32 = 1;

/// Durable bytes for exactly one outbox scope.
///
/// `save` must atomically replace the previous document (write-then-rename,
/// Keychain update, ...): a failed save leaves the old document readable.
pub trait OutboxStorage {
    /// Read the current document. `Ok(None)` means none was ever written.
    fn load(&self) -> Result<Option<Vec<u8>>, StorageError>;
    /// Atomically replace the document.
    fn save(&self, bytes: &[u8]) -> Result<(), StorageError>;
}

/// Why durable storage could not be used. Work stays retained either way.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StorageError {
    /// Protected storage is locked, damaged or unavailable; retry later.
    Unavailable(String),
    /// The account session that owned this access is no longer current.
    AccountChanged,
}

/// Identity of an outbox. The account is the verified internal owner.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutboxScope {
    /// Verified Duet owner (never an email or credential).
    pub account: String,
    /// Stable per-install identifier.
    pub installation: String,
    /// Phone source.
    pub source: PhoneSource,
}

/// Bounds for pages, batches, storage and retries.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutboxLimits {
    /// Maximum records + deletions requested per adapter page.
    pub page_size: usize,
    /// Maximum items per upload batch (server accepts 1..=100).
    pub batch_items: usize,
    /// Maximum serialized bytes per batch.
    pub batch_bytes: usize,
    /// Maximum serialized bytes per item (server accepts up to 2 MB).
    pub item_bytes: usize,
    /// Acquisition stops (without dropping anything) once this many items are pending.
    pub max_pending: usize,
    /// Uncertain deliveries before an item is parked for explicit resume.
    pub max_attempts: u32,
    /// First retry delay.
    pub base_backoff_ms: u64,
    /// Maximum retry delay.
    pub max_backoff_ms: u64,
}

impl Default for OutboxLimits {
    fn default() -> Self {
        Self {
            page_size: 200,
            batch_items: 100,
            batch_bytes: 4 * 1024 * 1024,
            item_bytes: 2 * 1024 * 1024,
            max_pending: 5_000,
            max_attempts: 8,
            base_backoff_ms: 2_000,
            max_backoff_ms: 6 * 60 * 60 * 1000,
        }
    }
}

/// Outbox failures. None of them drop work.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutboxError {
    /// Protected storage is unavailable; pause and retry later.
    Storage(StorageError),
    /// The stored document is unreadable or belongs to another scope. It is
    /// left untouched for explicit handling.
    Damaged,
    /// The source is not connected for this outbox (signed out of the source,
    /// disconnected, or never connected).
    NotConnected,
    /// The source generation presented is older than the stored one.
    StaleGeneration,
}

impl From<StorageError> for OutboxError {
    fn from(error: StorageError) -> Self {
        Self::Storage(error)
    }
}

/// Mutation kind.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum PendingKind {
    /// Create or correct the record with this exact payload.
    Upsert {
        /// Exact canonical payload.
        payload: Value,
    },
    /// Explicit source deletion.
    Delete,
}

/// Delivery state of a pending item.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryState {
    /// Waiting for (re)delivery.
    Pending,
    /// Handed to the transport; the outcome is unknown until reconciled.
    InFlight,
    /// Retry budget exhausted; kept until [`Outbox::resume_parked`].
    Parked,
}

/// One queued mutation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PendingItem {
    /// Enqueue order.
    pub seq: u64,
    /// Stable idempotency key assigned at enqueue and never changed.
    pub operation_id: Uuid,
    /// Native record identity.
    pub native_id: String,
    /// Mutation.
    pub kind: PendingKind,
    /// Delivery state.
    pub state: DeliveryState,
    /// Uncertain deliveries so far.
    pub attempts: u32,
    /// Earliest retry time (milliseconds, caller clock).
    pub not_before_ms: u64,
    /// Exact wire mutation, frozen at first dispatch so retries are byte-identical.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frozen: Option<Value>,
}

/// Typed server or local refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RejectionCode {
    /// Revision or source generation changed.
    Conflict,
    /// The device may not mutate this source.
    SourceAuthorityRefused,
    /// The item is malformed or too large.
    Invalid,
    /// The addressed record does not exist.
    NotFound,
    /// A deletion could not name the exact current revision because its upload
    /// was never acknowledged.
    UnknownRevision,
}

/// A refused item, retained with its exact mutation for inspection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RejectedItem {
    /// The refused item.
    pub item: PendingItem,
    /// Typed reason.
    pub code: RejectionCode,
    /// Diagnostic message.
    pub message: String,
}

/// Why work was fenced.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FenceReason {
    /// The source was disconnected.
    Disconnected,
    /// The source generation advanced.
    GenerationChanged,
}

/// Stale work retained but never uploaded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FencedItem {
    /// The stale item.
    pub item: PendingItem,
    /// Generation it was queued under.
    pub source_generation: i32,
    /// Why it was fenced.
    pub reason: FenceReason,
}

/// Server identity learned for a native record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerEntry {
    /// Canonical record id assigned at first enqueue.
    pub canonical_record_id: Uuid,
    /// Last acknowledged revision, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    /// SHA-256 of the last acknowledged payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_sha256: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Document {
    version: u32,
    scope: OutboxScope,
    connected: bool,
    source_generation: i32,
    #[serde(default)]
    anchor: Option<Anchor>,
    next_seq: u64,
    #[serde(default)]
    items: Vec<PendingItem>,
    #[serde(default)]
    ledger: BTreeMap<String, LedgerEntry>,
    #[serde(default)]
    rejected: Vec<RejectedItem>,
    #[serde(default)]
    fenced: Vec<FencedItem>,
    #[serde(default)]
    coverage: BTreeMap<String, CoverageOutcome>,
    #[serde(default)]
    last_read_error: Option<String>,
}

impl Document {
    fn new(scope: OutboxScope) -> Self {
        Self {
            version: VERSION,
            scope,
            connected: false,
            source_generation: 0,
            anchor: None,
            next_seq: 0,
            items: Vec::new(),
            ledger: BTreeMap::new(),
            rejected: Vec::new(),
            fenced: Vec::new(),
            coverage: BTreeMap::new(),
            last_read_error: None,
        }
    }

    fn fence_all(&mut self, reason: FenceReason) {
        let generation = self.source_generation;
        for item in self.items.drain(..) {
            self.fenced.push(FencedItem {
                item,
                source_generation: generation,
                reason,
            });
        }
        self.anchor = None;
        self.coverage.clear();
    }

    fn push(&mut self, native_id: &str, kind: PendingKind) {
        let seq = self.next_seq;
        self.next_seq += 1;
        self.items.push(PendingItem {
            seq,
            operation_id: Uuid::new_v4(),
            native_id: native_id.to_owned(),
            kind,
            state: DeliveryState::Pending,
            attempts: 0,
            not_before_ms: 0,
            frozen: None,
        });
    }

    fn ensure_ledger(&mut self, native_id: &str) {
        self.ledger
            .entry(native_id.to_owned())
            .or_insert_with(|| LedgerEntry {
                canonical_record_id: Uuid::new_v4(),
                revision: None,
                content_sha256: None,
            });
    }
}

/// Read-only view of an outbox for inspection and tests.
#[derive(Clone, Debug, PartialEq)]
pub struct OutboxView {
    /// Whether acquisition and upload are enabled.
    pub connected: bool,
    /// Current source generation.
    pub source_generation: i32,
    /// Committed acquisition anchor.
    pub anchor: Option<Anchor>,
    /// Pending, in-flight and parked items in order.
    pub items: Vec<PendingItem>,
    /// Retained refusals.
    pub rejected: Vec<RejectedItem>,
    /// Retained stale work.
    pub fenced: Vec<FencedItem>,
    /// Learned server identities.
    pub ledger: BTreeMap<String, LedgerEntry>,
    /// Latest per-type coverage.
    pub coverage: BTreeMap<String, CoverageOutcome>,
    /// Last whole-read failure, if the last read failed.
    pub last_read_error: Option<String>,
}

/// Result of one acquisition step.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AcquireReport {
    /// Upserts enqueued.
    pub upserts: usize,
    /// Deletions enqueued.
    pub deletions: usize,
    /// Records skipped because their exact content was already acknowledged or queued.
    pub unchanged: usize,
    /// Deletions ignored because the type's read did not succeed (adapter contract).
    pub ignored_deletions: usize,
    /// Items rejected locally (oversized).
    pub rejected: usize,
    /// More pages are available now.
    pub has_more: bool,
    /// The pending bound was reached; nothing was read.
    pub backpressure: bool,
    /// The whole read failed; anchor unchanged, nothing deleted.
    pub read_error: Option<String>,
}

/// One batch handed to the transport.
#[derive(Clone, Debug, PartialEq)]
pub struct UploadBatch {
    /// Generation the batch was frozen under.
    pub source_generation: i32,
    /// `(operation_id, exact wire mutation)` in order.
    pub items: Vec<(Uuid, Value)>,
}

impl UploadBatch {
    /// The `CanonicalEvidenceBatch` body.
    pub fn body(&self) -> Value {
        json!({"items": self.items.iter().map(|(_, m)| m.clone()).collect::<Vec<_>>()})
    }
    /// Operation ids in this batch.
    pub fn operation_ids(&self) -> Vec<Uuid> {
        self.items.iter().map(|(id, _)| *id).collect()
    }
}

/// Per-item server acknowledgement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ItemAck {
    /// Accepted at this canonical address.
    Accepted {
        /// Echoed operation id.
        operation_id: Uuid,
        /// Canonical record id.
        canonical_record_id: Uuid,
        /// Revision now current.
        revision: i64,
    },
    /// Refused with a typed reason.
    Rejected {
        /// Echoed operation id.
        operation_id: Uuid,
        /// Typed reason.
        code: RejectionCode,
        /// Diagnostic message.
        message: String,
    },
}

/// Result of delivering a batch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UploadOutcome {
    /// The server answered with per-item results.
    Delivered(Vec<ItemAck>),
    /// Network failure, timeout, 5xx or an unreadable answer: the server may
    /// or may not have applied the batch. Retry with the same operation ids.
    Uncertain,
    /// Authentication is required; retry after sign-in without spending budget.
    Unauthorized,
}

/// Summary of reconciling one batch.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct FinishReport {
    /// Items acknowledged and removed.
    pub acknowledged: usize,
    /// Items refused and retained.
    pub rejected: usize,
    /// Items rescheduled.
    pub retrying: usize,
    /// Items parked after exhausting retries.
    pub parked: usize,
}

/// Transactional outbox over one storage scope.
pub struct Outbox<S> {
    storage: S,
    scope: OutboxScope,
    limits: OutboxLimits,
}

fn sha256(value: &Value) -> String {
    let bytes = serde_json::to_vec(value).unwrap_or_default();
    Sha256::digest(&bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

impl<S: OutboxStorage> Outbox<S> {
    /// Open an outbox. Nothing is read until the first operation.
    pub fn new(storage: S, scope: OutboxScope, limits: OutboxLimits) -> Self {
        Self {
            storage,
            scope,
            limits,
        }
    }

    /// The outbox scope.
    pub fn scope(&self) -> &OutboxScope {
        &self.scope
    }

    fn load(&self) -> Result<Document, OutboxError> {
        match self.storage.load()? {
            None => Ok(Document::new(self.scope.clone())),
            Some(bytes) => match serde_json::from_slice::<Document>(&bytes) {
                Ok(document) if document.version == VERSION && document.scope == self.scope => {
                    Ok(document)
                }
                _ => Err(OutboxError::Damaged),
            },
        }
    }

    fn save(&self, document: &Document) -> Result<(), OutboxError> {
        let bytes = serde_json::to_vec(document).map_err(|_| OutboxError::Damaged)?;
        self.storage.save(&bytes)?;
        Ok(())
    }

    /// Inspect the committed state.
    pub fn view(&self) -> Result<OutboxView, OutboxError> {
        let d = self.load()?;
        Ok(OutboxView {
            connected: d.connected,
            source_generation: d.source_generation,
            anchor: d.anchor,
            items: d.items,
            rejected: d.rejected,
            fenced: d.fenced,
            ledger: d.ledger,
            coverage: d.coverage,
            last_read_error: d.last_read_error,
        })
    }

    /// Enable acquisition under `source_generation`. A newer generation fences
    /// all pending work from the old generation and restarts acquisition; an
    /// older generation is refused.
    pub fn connect(&self, source_generation: i32) -> Result<(), OutboxError> {
        let mut d = self.load()?;
        if source_generation < d.source_generation {
            return Err(OutboxError::StaleGeneration);
        }
        if source_generation > d.source_generation {
            d.fence_all(FenceReason::GenerationChanged);
            // Server identities belong to the old connection.
            d.ledger.clear();
            d.source_generation = source_generation;
        } else if d.connected {
            return Ok(());
        }
        d.connected = true;
        self.save(&d)
    }

    /// Disconnect the source: fence pending work, forget the anchor, stop.
    pub fn disconnect(&self) -> Result<(), OutboxError> {
        let mut d = self.load()?;
        d.fence_all(FenceReason::Disconnected);
        d.connected = false;
        self.save(&d)
    }

    /// Discard retained stale work after the person reviewed it.
    pub fn discard_fenced(&self) -> Result<(), OutboxError> {
        let mut d = self.load()?;
        d.fenced.clear();
        self.save(&d)
    }

    /// Make parked items eligible again.
    pub fn resume_parked(&self) -> Result<usize, OutboxError> {
        let mut d = self.load()?;
        let mut resumed = 0;
        for item in &mut d.items {
            if item.state == DeliveryState::Parked {
                item.state = DeliveryState::Pending;
                item.attempts = 0;
                item.not_before_ms = 0;
                resumed += 1;
            }
        }
        self.save(&d)?;
        Ok(resumed)
    }

    /// Read one bounded page and commit its records, deletions, coverage and
    /// next anchor in one atomic write. A failed read changes nothing but the
    /// recorded error; it never deletes.
    pub fn acquire<A: HealthSourceAdapter + ?Sized>(
        &self,
        adapter: &A,
    ) -> Result<AcquireReport, OutboxError> {
        let mut d = self.load()?;
        if !d.connected {
            return Err(OutboxError::NotConnected);
        }
        let room = self.limits.max_pending.saturating_sub(d.items.len());
        if room == 0 {
            return Ok(AcquireReport {
                backpressure: true,
                ..AcquireReport::default()
            });
        }
        let request = PageRequest {
            anchor: d.anchor.clone(),
            limit: self.limits.page_size.min(room).max(1),
        };
        let page = match adapter.read_page(&request) {
            Ok(page) => page,
            Err(error) => {
                let message = match &error {
                    AdapterError::PermissionLost => "permission_lost".to_owned(),
                    AdapterError::StoreUnavailable => "store_unavailable".to_owned(),
                    AdapterError::AnchorInvalidated => "anchor_invalidated".to_owned(),
                    AdapterError::Failed(message) => format!("failed: {message}"),
                };
                if error == AdapterError::AnchorInvalidated {
                    // Reread from the start; acknowledged content is skipped by hash.
                    d.anchor = None;
                }
                if error == AdapterError::PermissionLost {
                    for outcome in d.coverage.values_mut() {
                        *outcome = CoverageOutcome::PermissionLost;
                    }
                }
                if error == AdapterError::StoreUnavailable {
                    for outcome in d.coverage.values_mut() {
                        *outcome = CoverageOutcome::Error {
                            error: CoverageError::StoreUnavailable,
                        };
                    }
                }
                d.last_read_error = Some(message.clone());
                self.save(&d)?;
                return Ok(AcquireReport {
                    read_error: Some(message),
                    ..AcquireReport::default()
                });
            }
        };
        let report = self.apply_page(&mut d, page);
        self.save(&d)?;
        Ok(report)
    }

    /// Acquire until the adapter reports no more pages, or `max_pages` pages.
    pub fn acquire_all<A: HealthSourceAdapter + ?Sized>(
        &self,
        adapter: &A,
        max_pages: usize,
    ) -> Result<AcquireReport, OutboxError> {
        let mut total = AcquireReport::default();
        for _ in 0..max_pages {
            let report = self.acquire(adapter)?;
            total.upserts += report.upserts;
            total.deletions += report.deletions;
            total.unchanged += report.unchanged;
            total.ignored_deletions += report.ignored_deletions;
            total.rejected += report.rejected;
            total.has_more = report.has_more;
            total.backpressure = report.backpressure;
            total.read_error = report.read_error.clone();
            if !report.has_more || report.backpressure || report.read_error.is_some() {
                break;
            }
        }
        Ok(total)
    }

    fn apply_page(&self, d: &mut Document, page: AcquisitionPage) -> AcquireReport {
        let source = self.scope.source;
        let mut report = AcquireReport {
            has_more: page.has_more,
            ..AcquireReport::default()
        };
        let succeeded: BTreeSet<&str> = page
            .coverage
            .iter()
            .filter(|coverage| coverage.outcome.read_succeeded())
            .map(|coverage| coverage.data_type.as_str())
            .collect();
        for record in &page.records {
            let payload = record.payload(source);
            let hash = sha256(&payload);
            let size = serde_json::to_vec(&payload).map(|b| b.len()).unwrap_or(0);
            // Unsent upsert for this record: replace its content in place.
            if let Some(item) = d.items.iter_mut().rev().find(|item| {
                item.native_id == record.native_id
                    && item.frozen.is_none()
                    && matches!(item.kind, PendingKind::Upsert { .. })
            }) {
                if let PendingKind::Upsert { payload: old } = &item.kind
                    && sha256(old) == hash
                {
                    report.unchanged += 1;
                    continue;
                }
                item.kind = PendingKind::Upsert { payload };
                report.upserts += 1;
                continue;
            }
            let queued_same = d.items.iter().rev().find(|item| item.native_id == record.native_id).is_some_and(
                |item| matches!(&item.kind, PendingKind::Upsert { payload } if sha256(payload) == hash),
            );
            let acked_same = !d
                .items
                .iter()
                .any(|item| item.native_id == record.native_id)
                && d.ledger
                    .get(&record.native_id)
                    .is_some_and(|entry| entry.content_sha256.as_deref() == Some(hash.as_str()));
            if queued_same || acked_same {
                report.unchanged += 1;
                continue;
            }
            d.ensure_ledger(&record.native_id);
            if size > self.limits.item_bytes {
                let seq = d.next_seq;
                d.next_seq += 1;
                d.rejected.push(RejectedItem {
                    item: PendingItem {
                        seq,
                        operation_id: Uuid::new_v4(),
                        native_id: record.native_id.clone(),
                        kind: PendingKind::Upsert { payload },
                        state: DeliveryState::Pending,
                        attempts: 0,
                        not_before_ms: 0,
                        frozen: None,
                    },
                    code: RejectionCode::Invalid,
                    message: "item exceeds the upload size bound".into(),
                });
                report.rejected += 1;
                continue;
            }
            d.push(&record.native_id, PendingKind::Upsert { payload });
            report.upserts += 1;
        }
        for deletion in &page.deletions {
            if !succeeded.contains(deletion.data_type.as_str()) {
                report.ignored_deletions += 1;
                continue;
            }
            let has_sent = d
                .items
                .iter()
                .any(|item| item.native_id == deletion.native_id && item.frozen.is_some());
            let acknowledged = d
                .ledger
                .get(&deletion.native_id)
                .is_some_and(|entry| entry.revision.is_some());
            if !has_sent && !acknowledged {
                // Never reached the server from this installation: drop unsent upserts.
                let before = d.items.len();
                d.items.retain(|item| item.native_id != deletion.native_id);
                if before == d.items.len() {
                    report.unchanged += 1;
                } else {
                    report.deletions += 1;
                }
                d.ledger.remove(&deletion.native_id);
                continue;
            }
            // Drop unsent upserts superseded by the deletion; keep sent ones so
            // the deletion follows their acknowledged revision.
            d.items.retain(|item| {
                !(item.native_id == deletion.native_id
                    && item.frozen.is_none()
                    && matches!(item.kind, PendingKind::Upsert { .. }))
            });
            if d.items.iter().any(|item| {
                item.native_id == deletion.native_id && item.kind == PendingKind::Delete
            }) {
                report.unchanged += 1;
                continue;
            }
            d.push(&deletion.native_id, PendingKind::Delete);
            report.deletions += 1;
        }
        for coverage in page.coverage {
            d.coverage.insert(coverage.data_type, coverage.outcome);
        }
        if let Some(anchor) = page.next_anchor {
            d.anchor = Some(anchor);
        }
        d.last_read_error = None;
        report
    }

    fn freeze(&self, d: &Document, item: &PendingItem) -> Result<Value, RejectionCode> {
        let entry = d.ledger.get(&item.native_id);
        let canonical_record_id = entry
            .map(|entry| entry.canonical_record_id)
            .ok_or(RejectionCode::UnknownRevision)?;
        let revision = entry.and_then(|entry| entry.revision);
        let mut mutation = json!({
            "operation_id": item.operation_id,
            "canonical_record_id": canonical_record_id,
            "record_id": item.native_id,
            "entity": "health_evidence",
            "source_generation": d.source_generation,
        });
        match &item.kind {
            PendingKind::Upsert { payload } => {
                mutation["operation"] = "upsert".into();
                if let Some(revision) = revision {
                    mutation["expected_revision"] = revision.into();
                }
                mutation["payload"] = payload.clone();
            }
            PendingKind::Delete => {
                let revision = revision.ok_or(RejectionCode::UnknownRevision)?;
                mutation["operation"] = "delete".into();
                mutation["expected_revision"] = revision.into();
                mutation["payload"] = json!({"source": {"kind": self.scope.source.kind()}});
            }
        }
        Ok(mutation)
    }

    /// Select, freeze and mark in flight the next bounded batch. Items left
    /// in flight by a crash are resent with the same frozen mutation. Only the
    /// oldest unresolved item per record is eligible, so a correction or
    /// deletion always names the revision its predecessor produced.
    pub fn begin_upload(&self, now_ms: u64) -> Result<Option<UploadBatch>, OutboxError> {
        let mut d = self.load()?;
        if !d.connected {
            return Err(OutboxError::NotConnected);
        }
        let mut seen = BTreeSet::new();
        let mut chosen = Vec::new();
        let mut bytes = 0usize;
        let mut refused = Vec::new();
        for index in 0..d.items.len() {
            if chosen.len() >= self.limits.batch_items {
                break;
            }
            let item = &d.items[index];
            if !seen.insert(item.native_id.clone()) {
                continue;
            }
            let eligible = match item.state {
                DeliveryState::InFlight => true,
                DeliveryState::Pending => item.not_before_ms <= now_ms,
                DeliveryState::Parked => false,
            };
            if !eligible {
                continue;
            }
            let mutation = match &item.frozen {
                Some(frozen) => frozen.clone(),
                None => match self.freeze(&d, item) {
                    Ok(mutation) => mutation,
                    Err(code) => {
                        refused.push((index, code));
                        continue;
                    }
                },
            };
            let size = serde_json::to_vec(&mutation).map(|b| b.len()).unwrap_or(0);
            if !chosen.is_empty() && bytes + size > self.limits.batch_bytes {
                break;
            }
            bytes += size;
            chosen.push((index, mutation));
        }
        for (index, mutation) in &chosen {
            let item = &mut d.items[*index];
            item.frozen = Some(mutation.clone());
            item.state = DeliveryState::InFlight;
        }
        let batch = UploadBatch {
            source_generation: d.source_generation,
            items: chosen
                .iter()
                .map(|(index, mutation)| (d.items[*index].operation_id, mutation.clone()))
                .collect(),
        };
        for (index, code) in refused.iter().rev() {
            let item = d.items.remove(*index);
            d.rejected.push(RejectedItem {
                item,
                code: *code,
                message: "deletion has no acknowledged revision".into(),
            });
        }
        if chosen.is_empty() && refused.is_empty() {
            return Ok(None);
        }
        self.save(&d)?;
        Ok((!batch.items.is_empty()).then_some(batch))
    }

    /// Reconcile a delivered batch. Acknowledgements update the ledger in the
    /// same write that removes the item; refusals are retained; anything
    /// unanswered is rescheduled with the same operation id.
    pub fn finish_upload(
        &self,
        batch: &UploadBatch,
        outcome: UploadOutcome,
        now_ms: u64,
    ) -> Result<FinishReport, OutboxError> {
        let mut d = self.load()?;
        let mut report = FinishReport::default();
        if batch.source_generation != d.source_generation {
            // Items were fenced; nothing in this batch is pending any more.
            return Ok(report);
        }
        let acks: BTreeMap<Uuid, ItemAck> = match &outcome {
            UploadOutcome::Delivered(acks) => acks
                .iter()
                .map(|ack| match ack {
                    ItemAck::Accepted { operation_id, .. }
                    | ItemAck::Rejected { operation_id, .. } => (*operation_id, ack.clone()),
                })
                .collect(),
            _ => BTreeMap::new(),
        };
        for operation_id in batch.operation_ids() {
            let Some(index) = d
                .items
                .iter()
                .position(|item| item.operation_id == operation_id)
            else {
                continue;
            };
            match acks.get(&operation_id) {
                Some(ItemAck::Accepted {
                    canonical_record_id,
                    revision,
                    ..
                }) => {
                    let item = d.items.remove(index);
                    match &item.kind {
                        PendingKind::Upsert { payload } => {
                            d.ledger.insert(
                                item.native_id.clone(),
                                LedgerEntry {
                                    canonical_record_id: *canonical_record_id,
                                    revision: Some(*revision),
                                    content_sha256: Some(sha256(payload)),
                                },
                            );
                        }
                        PendingKind::Delete => {
                            d.ledger.remove(&item.native_id);
                        }
                    }
                    report.acknowledged += 1;
                }
                Some(ItemAck::Rejected { code, message, .. }) => {
                    let item = d.items.remove(index);
                    d.rejected.push(RejectedItem {
                        item,
                        code: *code,
                        message: message.clone(),
                    });
                    report.rejected += 1;
                }
                None => {
                    let item = &mut d.items[index];
                    if outcome == UploadOutcome::Unauthorized {
                        item.state = DeliveryState::Pending;
                        item.not_before_ms = now_ms;
                        report.retrying += 1;
                        continue;
                    }
                    item.attempts = item.attempts.saturating_add(1);
                    if item.attempts >= self.limits.max_attempts {
                        item.state = DeliveryState::Parked;
                        report.parked += 1;
                    } else {
                        let exponent = item.attempts.saturating_sub(1).min(32);
                        let delay = self
                            .limits
                            .base_backoff_ms
                            .saturating_mul(1u64 << exponent)
                            .min(self.limits.max_backoff_ms);
                        item.state = DeliveryState::Pending;
                        item.not_before_ms = now_ms.saturating_add(delay);
                        report.retrying += 1;
                    }
                }
            }
        }
        self.save(&d)?;
        Ok(report)
    }
}
