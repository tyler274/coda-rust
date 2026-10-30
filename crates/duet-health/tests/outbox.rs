//! Deterministic outbox proofs with storage and delivery fault injection.
use duet_health::acquisition::*;
use duet_health::outbox::*;
use serde_json::{Value, json};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

/// Shared bytes survive "process restarts" (new `Outbox` over a clone).
#[derive(Clone, Default)]
struct Disk {
    bytes: Rc<RefCell<Option<Vec<u8>>>>,
    locked: Rc<RefCell<bool>>,
    /// Fail the n-th save from now (0 = next save), simulating a crash.
    crash_on_save: Rc<RefCell<Option<usize>>>,
    saves: Rc<RefCell<usize>>,
}
impl OutboxStorage for Disk {
    fn load(&self) -> Result<Option<Vec<u8>>, StorageError> {
        if *self.locked.borrow() {
            return Err(StorageError::Unavailable("locked".into()));
        }
        Ok(self.bytes.borrow().clone())
    }
    fn save(&self, bytes: &[u8]) -> Result<(), StorageError> {
        if *self.locked.borrow() {
            return Err(StorageError::Unavailable("locked".into()));
        }
        let mut crash = self.crash_on_save.borrow_mut();
        if let Some(n) = *crash {
            if n == 0 {
                *crash = None;
                return Err(StorageError::Unavailable("crash".into()));
            }
            *crash = Some(n - 1);
        }
        *self.bytes.borrow_mut() = Some(bytes.to_vec());
        *self.saves.borrow_mut() += 1;
        Ok(())
    }
}

fn scope() -> OutboxScope {
    OutboxScope {
        account: "owner-a".into(),
        installation: "install-1".into(),
        source: PhoneSource::AppleHealth,
    }
}
fn limits() -> OutboxLimits {
    OutboxLimits {
        page_size: 100,
        batch_items: 100,
        max_attempts: 3,
        base_backoff_ms: 1_000,
        max_backoff_ms: 10_000,
        ..OutboxLimits::default()
    }
}
fn open(disk: &Disk) -> Outbox<Disk> {
    Outbox::new(disk.clone(), scope(), limits())
}

fn record(id: usize, literal: &str) -> AcquisitionRecord {
    AcquisitionRecord {
        native_id: format!("HKQuantitySample/{id}"),
        data_type: "HKQuantityTypeIdentifierHeartRate".into(),
        concept_id: "vital:heart_rate".into(),
        value: ExactValue {
            literal: literal.into(),
            unit: Some("count/min".into()),
        },
        time: SourceTime::Instant {
            at: "2026-10-20T07:30:00+09:00".into(),
        },
        time_zone: Some("Asia/Tokyo".into()),
        provenance: Provenance {
            source_name: Some("Apple Watch".into()),
            writer_id: Some("com.apple.health.ABC".into()),
            device: Some("Watch7,1".into()),
        },
        original: None,
    }
}

const HR: &str = "HKQuantityTypeIdentifierHeartRate";

/// A paged fake store: anchor is the offset into a change log.
struct FakeAdapter {
    log: RefCell<Vec<Change>>,
    fail: RefCell<Option<AdapterError>>,
    coverage_override: RefCell<Option<CoverageOutcome>>,
    reads: RefCell<usize>,
}
#[derive(Clone)]
enum Change {
    Put(Box<AcquisitionRecord>),
    Delete(String),
}
impl FakeAdapter {
    fn new(changes: Vec<Change>) -> Self {
        Self {
            log: RefCell::new(changes),
            fail: RefCell::new(None),
            coverage_override: RefCell::new(None),
            reads: RefCell::new(0),
        }
    }
}
impl HealthSourceAdapter for FakeAdapter {
    fn source(&self) -> PhoneSource {
        PhoneSource::AppleHealth
    }
    fn read_page(&self, request: &PageRequest) -> Result<AcquisitionPage, AdapterError> {
        *self.reads.borrow_mut() += 1;
        if let Some(error) = self.fail.borrow().clone() {
            return Err(error);
        }
        let start: usize = request.anchor.as_ref().map_or(0, |a| a.0.parse().unwrap());
        let log = self.log.borrow();
        let end = (start + request.limit).min(log.len());
        let mut page = AcquisitionPage::default();
        for change in &log[start..end] {
            match change {
                Change::Put(r) => page.records.push((**r).clone()),
                Change::Delete(id) => page.deletions.push(DeletionRecord {
                    native_id: id.clone(),
                    data_type: HR.into(),
                }),
            }
        }
        let outcome = self
            .coverage_override
            .borrow()
            .clone()
            .unwrap_or(if end > start {
                CoverageOutcome::Data
            } else {
                CoverageOutcome::Empty
            });
        page.coverage = vec![TypeCoverage {
            data_type: HR.into(),
            outcome,
        }];
        page.next_anchor = Some(Anchor(end.to_string()));
        page.has_more = end < log.len();
        Ok(page)
    }
}

/// Server double with slice-1 semantics: idempotent by operation id (same
/// content replays the ack; different content conflicts), exact revision and
/// source generation fences, explicit deletion.
#[derive(Default)]
struct Server {
    generation: i32,
    ops: HashMap<Uuid, (Value, ItemAck)>,
    records: HashMap<Uuid, (i64, bool, Value)>,
    applied: usize,
}
impl Server {
    fn apply(&mut self, batch: &UploadBatch) -> Vec<ItemAck> {
        let mut acks = Vec::new();
        for (op, m) in &batch.items {
            if let Some((prior, ack)) = self.ops.get(op) {
                assert_eq!(prior, m, "retry must resend identical content");
                acks.push(ack.clone());
                continue;
            }
            let id: Uuid = m["canonical_record_id"].as_str().unwrap().parse().unwrap();
            let expected = m.get("expected_revision").and_then(Value::as_i64);
            let current = self.records.get(&id).map(|r| r.0);
            let reject = |code| ItemAck::Rejected {
                operation_id: *op,
                code,
                message: "refused".into(),
            };
            let ack = if m["source_generation"].as_i64() != Some(self.generation.into())
                || expected != current
            {
                reject(RejectionCode::Conflict)
            } else {
                let revision = current.unwrap_or(0) + 1;
                let deleted = m["operation"] == "delete";
                self.records
                    .insert(id, (revision, deleted, m["payload"].clone()));
                self.applied += 1;
                ItemAck::Accepted {
                    operation_id: *op,
                    canonical_record_id: id,
                    revision,
                }
            };
            self.ops.insert(*op, (m.clone(), ack.clone()));
            acks.push(ack);
        }
        acks
    }
}

fn drain(outbox: &Outbox<Disk>, server: &mut Server) {
    while let Some(batch) = outbox.begin_upload(u64::MAX / 2).unwrap() {
        let acks = server.apply(&batch);
        outbox
            .finish_upload(&batch, UploadOutcome::Delivered(acks), 0)
            .unwrap();
    }
}

#[test]
fn full_multi_page_pagination_commits_each_page_with_its_anchor() {
    let disk = Disk::default();
    let outbox = open(&disk);
    outbox.connect(0).unwrap();
    let adapter = FakeAdapter::new(
        (0..250)
            .map(|i| Change::Put(Box::new(record(i, "60"))))
            .collect(),
    );
    let report = outbox.acquire_all(&adapter, 10).unwrap();
    assert_eq!(report.upserts, 250);
    assert!(!report.has_more);
    assert_eq!(*adapter.reads.borrow(), 3);
    let view = outbox.view().unwrap();
    assert_eq!(view.anchor, Some(Anchor("250".into())));
    assert_eq!(view.items.len(), 250);
    let mut server = Server::default();
    let mut batches = 0;
    while let Some(batch) = outbox.begin_upload(0).unwrap() {
        assert!(batch.items.len() <= 100);
        batches += 1;
        let acks = server.apply(&batch);
        outbox
            .finish_upload(&batch, UploadOutcome::Delivered(acks), 0)
            .unwrap();
    }
    assert_eq!(batches, 3);
    assert_eq!(server.applied, 250);
    assert!(outbox.view().unwrap().items.is_empty());
    // Rereading finds nothing new; an empty read is not a deletion.
    let again = outbox.acquire(&adapter).unwrap();
    assert_eq!((again.upserts, again.deletions), (0, 0));
    assert_eq!(outbox.view().unwrap().coverage[HR], CoverageOutcome::Empty);
}

#[test]
fn crash_between_record_persist_and_cursor_commit_rereads_without_duplicates() {
    let disk = Disk::default();
    open(&disk).connect(0).unwrap();
    let adapter = FakeAdapter::new(
        (0..150)
            .map(|i| Change::Put(Box::new(record(i, "61"))))
            .collect(),
    );
    // Page 1 commits; the page 2 write crashes. Records and anchor are one
    // write, so neither page 2's records nor its anchor are visible.
    *disk.crash_on_save.borrow_mut() = Some(1);
    let outbox = open(&disk);
    outbox.acquire(&adapter).unwrap();
    assert!(matches!(
        outbox.acquire(&adapter),
        Err(OutboxError::Storage(_))
    ));
    let restarted = open(&disk);
    let view = restarted.view().unwrap();
    assert_eq!(view.anchor, Some(Anchor("100".into())));
    assert_eq!(view.items.len(), 100);
    restarted.acquire_all(&adapter, 5).unwrap();
    let view = restarted.view().unwrap();
    assert_eq!(view.items.len(), 150);
    let ids: std::collections::BTreeSet<_> = view.items.iter().map(|i| &i.native_id).collect();
    assert_eq!(ids.len(), 150);
    // Rereading from an invalidated anchor skips acknowledged content.
    let mut server = Server::default();
    drain(&restarted, &mut server);
    *adapter.fail.borrow_mut() = Some(AdapterError::AnchorInvalidated);
    restarted.acquire(&adapter).unwrap();
    *adapter.fail.borrow_mut() = None;
    let report = restarted.acquire_all(&adapter, 5).unwrap();
    assert_eq!((report.upserts, report.unchanged), (0, 150));
}

#[test]
fn crash_after_dispatch_and_uncertain_ack_retry_the_same_operation_id_and_bytes() {
    let disk = Disk::default();
    let outbox = open(&disk);
    outbox.connect(0).unwrap();
    outbox
        .acquire(&FakeAdapter::new(vec![Change::Put(Box::new(record(
            1, "72",
        )))]))
        .unwrap();
    let mut server = Server::default();

    // Crash after marking in flight, before the transport ran.
    let first = outbox.begin_upload(0).unwrap().unwrap();
    let restarted = open(&disk);
    let resent = restarted.begin_upload(0).unwrap().unwrap();
    assert_eq!(first, resent);

    // Server applied it but the answer was lost.
    server.apply(&resent);
    let report = restarted
        .finish_upload(&resent, UploadOutcome::Uncertain, 0)
        .unwrap();
    assert_eq!(report.retrying, 1);
    assert!(restarted.begin_upload(500).unwrap().is_none(), "backoff");
    let retry = restarted.begin_upload(1_000).unwrap().unwrap();
    assert_eq!(retry, first, "same operation id and exact content");

    // The ack arrives but the reconciling write crashes: still in flight.
    let acks = server.apply(&retry);
    *disk.crash_on_save.borrow_mut() = Some(0);
    assert!(
        restarted
            .finish_upload(&retry, UploadOutcome::Delivered(acks), 1_000)
            .is_err()
    );
    let after_crash = open(&disk);
    let again = after_crash.begin_upload(1_000).unwrap().unwrap();
    assert_eq!(again, first);
    let acks = server.apply(&again);
    after_crash
        .finish_upload(&again, UploadOutcome::Delivered(acks), 1_000)
        .unwrap();
    assert_eq!(server.applied, 1, "server applied the record exactly once");
    let view = after_crash.view().unwrap();
    assert!(view.items.is_empty());
    assert_eq!(view.ledger["HKQuantitySample/1"].revision, Some(1));
}

#[test]
fn empty_error_and_permission_loss_never_delete_but_explicit_deletion_does() {
    let disk = Disk::default();
    let outbox = open(&disk);
    outbox.connect(0).unwrap();
    let adapter = FakeAdapter::new(vec![Change::Put(Box::new(record(1, "70")))]);
    outbox.acquire(&adapter).unwrap();
    let mut server = Server::default();
    drain(&outbox, &mut server);

    // Empty read.
    let report = outbox.acquire(&adapter).unwrap();
    assert_eq!(report.deletions, 0);
    // Permission lost and store errors leave the anchor and ledger alone.
    for error in [
        AdapterError::PermissionLost,
        AdapterError::StoreUnavailable,
        AdapterError::Failed("boom".into()),
    ] {
        *adapter.fail.borrow_mut() = Some(error);
        let report = outbox.acquire(&adapter).unwrap();
        assert!(report.read_error.is_some());
        let view = outbox.view().unwrap();
        assert!(view.items.is_empty());
        assert_eq!(view.anchor, Some(Anchor("1".into())));
        assert!(view.ledger.contains_key("HKQuantitySample/1"));
    }
    *adapter.fail.borrow_mut() = Some(AdapterError::PermissionLost);
    outbox.acquire(&adapter).unwrap();
    assert_eq!(
        outbox.view().unwrap().coverage[HR],
        CoverageOutcome::PermissionLost
    );
    *adapter.fail.borrow_mut() = None;

    // A deletion reported alongside a failed type read is ignored.
    adapter
        .log
        .borrow_mut()
        .push(Change::Delete("HKQuantitySample/1".into()));
    *adapter.coverage_override.borrow_mut() = Some(CoverageOutcome::Error {
        error: CoverageError::QueryFailed {
            message: "x".into(),
        },
    });
    let report = outbox.acquire(&adapter).unwrap();
    assert_eq!((report.deletions, report.ignored_deletions), (0, 1));
    assert!(outbox.view().unwrap().items.is_empty());

    // An explicit deletion from a successful read deletes the exact revision.
    adapter
        .log
        .borrow_mut()
        .push(Change::Delete("HKQuantitySample/1".into()));
    *adapter.coverage_override.borrow_mut() = None;
    let report = outbox.acquire(&adapter).unwrap();
    assert_eq!(report.deletions, 1);
    let batch = outbox.begin_upload(0).unwrap().unwrap();
    let (_, mutation) = &batch.items[0];
    assert_eq!(mutation["operation"], "delete");
    assert_eq!(mutation["expected_revision"], 1);
    assert_eq!(
        mutation["payload"],
        json!({"source":{"kind":"appleHealth"}})
    );
    let acks = server.apply(&batch);
    outbox
        .finish_upload(&batch, UploadOutcome::Delivered(acks), 0)
        .unwrap();
    let view = outbox.view().unwrap();
    assert!(view.items.is_empty() && view.rejected.is_empty());
    assert!(!view.ledger.contains_key("HKQuantitySample/1"));
}

#[test]
fn original_value_unit_and_time_zone_are_sent_exactly_and_unknown_dates_stay_absent() {
    let disk = Disk::default();
    let outbox = open(&disk);
    outbox.connect(0).unwrap();
    let mut undated = record(2, "< 5.7");
    undated.time = SourceTime::Unknown;
    undated.time_zone = None;
    undated.value.unit = Some("%".into());
    let mut interval = record(3, "1200");
    interval.time = SourceTime::Interval {
        start: Some("2026-10-20T00:00:00-07:00".into()),
        end: None,
    };
    interval.original = Some(json!({"HKMetadataKeyTimeZone":"America/Los_Angeles"}));
    outbox
        .acquire(&FakeAdapter::new(vec![
            Change::Put(Box::new(record(1, "072"))),
            Change::Put(Box::new(undated)),
            Change::Put(Box::new(interval)),
        ]))
        .unwrap();
    let batch = outbox.begin_upload(0).unwrap().unwrap();
    let p = |i: usize| batch.items[i].1["payload"].clone();
    assert_eq!(
        p(0),
        json!({
            "concept_id":"vital:heart_rate",
            "measured_at":"2026-10-20T07:30:00+09:00",
            "timezone":"Asia/Tokyo",
            "source":{"kind":"appleHealth","record_id":"HKQuantitySample/1","data_type":HR,
                "source_name":"Apple Watch","writer_id":"com.apple.health.ABC","device":"Watch7,1"},
            "value":{"literal":"072","unit":"count/min"}
        })
    );
    assert!(p(1).get("measured_at").is_none());
    assert!(p(1).get("measured_interval").is_none());
    assert!(p(1).get("timezone").is_none());
    assert_eq!(p(1)["value"], json!({"literal":"< 5.7","unit":"%"}));
    assert_eq!(
        p(2)["measured_interval"],
        json!({"start":"2026-10-20T00:00:00-07:00"})
    );
    assert_eq!(
        p(2)["source_record"],
        json!({"HKMetadataKeyTimeZone":"America/Los_Angeles"})
    );
    let m = &batch.items[0].1;
    assert_eq!(m["entity"], "health_evidence");
    assert_eq!(m["record_id"], "HKQuantitySample/1");
    assert_eq!(m["source_generation"], 0);
    assert!(m.get("expected_revision").is_none());
}

#[test]
fn corrections_follow_the_acknowledged_revision_in_order() {
    let disk = Disk::default();
    let outbox = open(&disk);
    outbox.connect(0).unwrap();
    let adapter = FakeAdapter::new(vec![Change::Put(Box::new(record(1, "70")))]);
    outbox.acquire(&adapter).unwrap();
    // First upsert is in flight when a correction arrives.
    let first = outbox.begin_upload(0).unwrap().unwrap();
    adapter
        .log
        .borrow_mut()
        .push(Change::Put(Box::new(record(1, "71"))));
    outbox.acquire(&adapter).unwrap();
    assert_eq!(outbox.view().unwrap().items.len(), 2);
    // Only one item per record is ever in flight.
    assert!(outbox.begin_upload(0).unwrap().unwrap() == first);
    let mut server = Server::default();
    let acks = server.apply(&first);
    outbox
        .finish_upload(&first, UploadOutcome::Delivered(acks), 0)
        .unwrap();
    let correction = outbox.begin_upload(0).unwrap().unwrap();
    assert_eq!(correction.items[0].1["expected_revision"], 1);
    assert_eq!(correction.items[0].1["payload"]["value"]["literal"], "71");
    let acks = server.apply(&correction);
    outbox
        .finish_upload(&correction, UploadOutcome::Delivered(acks), 0)
        .unwrap();
    let view = outbox.view().unwrap();
    assert!(view.items.is_empty());
    assert_eq!(view.ledger["HKQuantitySample/1"].revision, Some(2));
}

#[test]
fn bounded_retries_park_work_without_dropping_it() {
    let disk = Disk::default();
    let outbox = open(&disk);
    outbox.connect(0).unwrap();
    outbox
        .acquire(&FakeAdapter::new(vec![Change::Put(Box::new(record(
            1, "70",
        )))]))
        .unwrap();
    let mut now = 0;
    let first = outbox.begin_upload(now).unwrap().unwrap();
    let mut delays = Vec::new();
    for _ in 0..3 {
        let batch = outbox.begin_upload(now).unwrap().unwrap();
        assert_eq!(batch, first);
        outbox
            .finish_upload(&batch, UploadOutcome::Uncertain, now)
            .unwrap();
        let view = outbox.view().unwrap();
        delays.push(view.items[0].not_before_ms.saturating_sub(now));
        now = view.items[0].not_before_ms;
    }
    assert_eq!(&delays[..2], &[1_000, 2_000]);
    let view = outbox.view().unwrap();
    assert_eq!(view.items.len(), 1);
    assert_eq!(view.items[0].state, DeliveryState::Parked);
    assert!(outbox.begin_upload(u64::MAX).unwrap().is_none());
    assert_eq!(outbox.resume_parked().unwrap(), 1);
    let resumed = outbox.begin_upload(now).unwrap().unwrap();
    assert_eq!(resumed, first, "same operation id after resume");
    // Unauthorized does not spend the retry budget.
    outbox
        .finish_upload(&resumed, UploadOutcome::Unauthorized, now)
        .unwrap();
    assert_eq!(outbox.view().unwrap().items[0].attempts, 0);
}

#[test]
fn rejections_are_typed_and_retained() {
    let disk = Disk::default();
    let outbox = open(&disk);
    outbox.connect(0).unwrap();
    outbox
        .acquire(&FakeAdapter::new(vec![Change::Put(Box::new(record(
            1, "70",
        )))]))
        .unwrap();
    let mut server = Server {
        generation: 4,
        ..Server::default()
    };
    drain(&outbox, &mut server);
    let view = outbox.view().unwrap();
    assert!(view.items.is_empty());
    assert_eq!(view.rejected.len(), 1);
    assert_eq!(view.rejected[0].code, RejectionCode::Conflict);
    assert_eq!(
        view.rejected[0].item.frozen.as_ref().unwrap()["payload"]["value"]["literal"],
        "70"
    );
}

#[test]
fn generation_bump_and_disconnect_fence_stale_work() {
    let disk = Disk::default();
    let outbox = open(&disk);
    outbox.connect(0).unwrap();
    let adapter = FakeAdapter::new(vec![
        Change::Put(Box::new(record(1, "70"))),
        Change::Put(Box::new(record(2, "71"))),
    ]);
    outbox.acquire(&adapter).unwrap();
    let in_flight = outbox.begin_upload(0).unwrap().unwrap();

    // Reconnect raced the upload: the old batch is fenced and its late
    // acknowledgement changes nothing.
    outbox.connect(1).unwrap();
    let view = outbox.view().unwrap();
    assert!(view.items.is_empty());
    assert_eq!(view.fenced.len(), 2);
    assert!(
        view.fenced
            .iter()
            .all(|f| f.source_generation == 0 && f.reason == FenceReason::GenerationChanged)
    );
    assert_eq!(view.anchor, None);
    let mut server = Server::default();
    let late = server.apply(&in_flight);
    let report = outbox
        .finish_upload(&in_flight, UploadOutcome::Delivered(late), 0)
        .unwrap();
    assert_eq!(report, FinishReport::default());
    assert!(matches!(
        outbox.connect(0),
        Err(OutboxError::StaleGeneration)
    ));

    // Reacquire under generation 1.
    outbox.acquire(&adapter).unwrap();
    let batch = outbox.begin_upload(0).unwrap().unwrap();
    assert!(batch.items.iter().all(|(_, m)| m["source_generation"] == 1));
    assert!(
        batch
            .operation_ids()
            .iter()
            .all(|id| !in_flight.operation_ids().contains(id))
    );

    // Ordinary disconnect keeps the queue, frozen ids, ledger and anchor.
    // Generation-change fences stay where they are.
    let before = outbox.view().unwrap();
    outbox.disconnect().unwrap();
    let view = outbox.view().unwrap();
    assert!(!view.connected);
    assert_eq!(view.items, before.items);
    assert_eq!(view.anchor, before.anchor);
    assert_eq!(view.ledger, before.ledger);
    assert!(view.items.iter().all(|item| item.frozen.is_some()));
    assert_eq!(view.fenced.len(), 2);
    assert!(
        view.fenced
            .iter()
            .all(|f| f.source_generation == 0 && f.reason == FenceReason::GenerationChanged)
    );
    assert_eq!(outbox.acquire(&adapter), Err(OutboxError::NotConnected));
    assert_eq!(outbox.begin_upload(0), Err(OutboxError::NotConnected));
    // A late acknowledgement still finds the original operation.
    let mut resumed = Server {
        generation: 1,
        ..Server::default()
    };
    let acks = resumed.apply(&batch);
    let report = outbox
        .finish_upload(&batch, UploadOutcome::Delivered(acks), 0)
        .unwrap();
    assert_eq!(report.acknowledged, 2);
    assert!(outbox.view().unwrap().items.is_empty());
    assert_eq!(outbox.view().unwrap().fenced.len(), 2);
    outbox.discard_fenced().unwrap();
    assert!(outbox.view().unwrap().fenced.is_empty());
}

#[test]
fn same_generation_disconnect_resumes_a_lost_acknowledgement() {
    let disk = Disk::default();
    let outbox = open(&disk);
    outbox.connect(0).unwrap();
    let adapter = FakeAdapter::new(vec![Change::Put(Box::new(record(1, "70")))]);
    outbox.acquire(&adapter).unwrap();
    let anchor = outbox.view().unwrap().anchor.clone();
    let create = outbox.begin_upload(0).unwrap().unwrap();
    let operation_id = create.operation_ids()[0];
    let canonical: Uuid = create.items[0].1["canonical_record_id"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    let mut server = Server::default();
    server.apply(&create);
    outbox.disconnect().unwrap();
    // The answer was lost. Reconciling it after disconnect still names this
    // operation; the queue was not fenced.
    let lost = outbox
        .finish_upload(&create, UploadOutcome::Uncertain, 0)
        .unwrap();
    assert_eq!(lost.retrying, 1);
    let view = outbox.view().unwrap();
    assert!(!view.connected);
    assert!(view.fenced.is_empty());
    assert_eq!(view.anchor, anchor);
    assert_eq!(view.items.len(), 1);
    assert_eq!(view.items[0].operation_id, operation_id);
    assert_eq!(view.items[0].frozen.as_ref(), Some(&create.items[0].1));
    assert_eq!(
        view.ledger["HKQuantitySample/1"].canonical_record_id,
        canonical
    );
    assert_eq!(view.ledger["HKQuantitySample/1"].revision, None);
    assert_eq!(outbox.acquire(&adapter), Err(OutboxError::NotConnected));
    assert_eq!(outbox.begin_upload(1_000), Err(OutboxError::NotConnected));

    outbox.connect(0).unwrap();
    let retry = outbox.begin_upload(1_000).unwrap().unwrap();
    assert_eq!(retry, create);
    let acks = server.apply(&retry);
    assert!(matches!(
        &acks[0],
        ItemAck::Accepted {
            operation_id: id,
            canonical_record_id,
            revision: 1,
        } if *id == operation_id && *canonical_record_id == canonical
    ));
    outbox
        .finish_upload(&retry, UploadOutcome::Delivered(acks), 1_000)
        .unwrap();
    assert_eq!(server.applied, 1);
    assert_eq!(
        outbox.view().unwrap().ledger["HKQuantitySample/1"].revision,
        Some(1)
    );

    adapter
        .log
        .borrow_mut()
        .push(Change::Put(Box::new(record(1, "71"))));
    outbox.acquire(&adapter).unwrap();
    let correction = outbox.begin_upload(2_000).unwrap().unwrap();
    assert_ne!(correction.operation_ids()[0], operation_id);
    assert_eq!(
        correction.items[0].1["canonical_record_id"]
            .as_str()
            .unwrap()
            .parse::<Uuid>()
            .unwrap(),
        canonical
    );
    assert_eq!(correction.items[0].1["expected_revision"], 1);
    assert_eq!(correction.items[0].1["payload"]["value"]["literal"], "71");
    let acks = server.apply(&correction);
    outbox
        .finish_upload(&correction, UploadOutcome::Delivered(acks), 2_000)
        .unwrap();

    adapter
        .log
        .borrow_mut()
        .push(Change::Delete("HKQuantitySample/1".into()));
    outbox.acquire(&adapter).unwrap();
    let deletion = outbox.begin_upload(3_000).unwrap().unwrap();
    assert_eq!(deletion.items[0].1["operation"], "delete");
    assert_eq!(
        deletion.items[0].1["canonical_record_id"]
            .as_str()
            .unwrap()
            .parse::<Uuid>()
            .unwrap(),
        canonical
    );
    assert_eq!(deletion.items[0].1["expected_revision"], 2);
    let acks = server.apply(&deletion);
    outbox
        .finish_upload(&deletion, UploadOutcome::Delivered(acks), 3_000)
        .unwrap();

    let view = outbox.view().unwrap();
    assert!(view.items.is_empty() && view.rejected.is_empty());
    assert!(!view.ledger.contains_key("HKQuantitySample/1"));
    assert_eq!(server.records.len(), 1);
    let (revision, deleted, _) = server.records[&canonical];
    assert_eq!((revision, deleted, server.applied), (3, true, 3));
}

#[test]
fn locked_storage_pauses_and_resumes_without_losing_work() {
    let disk = Disk::default();
    let outbox = open(&disk);
    outbox.connect(0).unwrap();
    let adapter = FakeAdapter::new(
        (0..3)
            .map(|i| Change::Put(Box::new(record(i, "70"))))
            .collect(),
    );
    outbox.acquire(&adapter).unwrap();
    *disk.locked.borrow_mut() = true;
    let reads = *adapter.reads.borrow();
    assert!(matches!(
        outbox.acquire(&adapter),
        Err(OutboxError::Storage(StorageError::Unavailable(_)))
    ));
    assert_eq!(*adapter.reads.borrow(), reads, "no read while locked");
    assert!(matches!(
        outbox.begin_upload(0),
        Err(OutboxError::Storage(_))
    ));
    *disk.locked.borrow_mut() = false;
    let mut server = Server::default();
    drain(&outbox, &mut server);
    assert_eq!(server.applied, 3);
}

#[test]
fn pending_bound_applies_backpressure_and_oversized_items_are_rejected_locally() {
    let disk = Disk::default();
    let outbox = Outbox::new(
        disk.clone(),
        scope(),
        OutboxLimits {
            page_size: 10,
            max_pending: 5,
            item_bytes: 1_000,
            ..limits()
        },
    );
    outbox.connect(0).unwrap();
    let mut huge = record(99, "1");
    huge.original = Some(json!({"blob":"x".repeat(2_000)}));
    let mut log: Vec<_> = vec![Change::Put(Box::new(huge))];
    log.extend((0..20).map(|i| Change::Put(Box::new(record(i, "70")))));
    let adapter = FakeAdapter::new(log);
    let report = outbox.acquire(&adapter).unwrap();
    assert_eq!(report.rejected, 1);
    assert_eq!(report.upserts, 4);
    // Only the remaining room is requested.
    assert_eq!(outbox.acquire(&adapter).unwrap().upserts, 1);
    let reads = *adapter.reads.borrow();
    let report = outbox.acquire(&adapter).unwrap();
    assert!(report.backpressure);
    assert_eq!(*adapter.reads.borrow(), reads, "no read under backpressure");
    let view = outbox.view().unwrap();
    assert_eq!(view.items.len(), 5);
    assert_eq!(view.rejected[0].code, RejectionCode::Invalid);
    let mut server = Server::default();
    drain(&outbox, &mut server);
    // Draining makes room; nothing was dropped while acquisition waited.
    loop {
        let report = outbox.acquire_all(&adapter, 10).unwrap();
        drain(&outbox, &mut server);
        if !report.backpressure && !report.has_more {
            break;
        }
    }
    assert_eq!(server.applied, 20);
}

#[test]
fn another_scope_cannot_open_the_document() {
    let disk = Disk::default();
    open(&disk).connect(0).unwrap();
    let other = Outbox::new(
        disk.clone(),
        OutboxScope {
            account: "owner-b".into(),
            ..scope()
        },
        limits(),
    );
    assert_eq!(other.view(), Err(OutboxError::Damaged));
    assert_eq!(other.connect(0), Err(OutboxError::Damaged));
}
