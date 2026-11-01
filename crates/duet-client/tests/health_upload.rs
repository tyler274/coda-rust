//! Account-fenced acquisition/upload over protected retention.
use duet_client::health_upload::*;
use duet_client::retention::*;
use duet_health::acquisition::*;
use duet_health::outbox::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct Store {
    values: Mutex<HashMap<String, Vec<u8>>>,
    locked: Mutex<bool>,
    deny: Mutex<bool>,
}
impl ProtectedStore for Store {
    fn cleanup_pending(&self) -> Result<bool, String> {
        Ok(*self.deny.lock().unwrap())
    }
    fn set_cleanup_pending(&self, value: bool) -> Result<(), String> {
        *self.deny.lock().unwrap() = value;
        Ok(())
    }
    fn read(&self, key: &str) -> Result<Option<Vec<u8>>, String> {
        if *self.locked.lock().unwrap() {
            return Err("locked".into());
        }
        Ok(self.values.lock().unwrap().get(key).cloned())
    }
    fn write(&self, key: &str, value: &[u8]) -> Result<(), String> {
        if *self.locked.lock().unwrap() {
            return Err("locked".into());
        }
        self.values.lock().unwrap().insert(key.into(), value.into());
        Ok(())
    }
    fn remove(&self, key: &str) -> Result<(), String> {
        if *self.locked.lock().unwrap() {
            return Err("locked".into());
        }
        self.values.lock().unwrap().remove(key);
        Ok(())
    }
}
fn identity(owner: &str) -> RetainedIdentity {
    RetainedIdentity {
        subject: format!("subject-{owner}"),
        owner: owner.into(),
        refresh_token: "secret".into(),
    }
}

struct Adapter {
    count: usize,
}
impl HealthSourceAdapter for Adapter {
    fn source(&self) -> PhoneSource {
        PhoneSource::AppleHealth
    }
    fn read_page(&self, request: &PageRequest) -> Result<AcquisitionPage, AdapterError> {
        let start: usize = request.anchor.as_ref().map_or(0, |a| a.0.parse().unwrap());
        let end = (start + request.limit).min(self.count);
        Ok(AcquisitionPage {
            records: (start..end)
                .map(|i| AcquisitionRecord {
                    native_id: format!("HKQuantitySample/{i}"),
                    data_type: "HKQuantityTypeIdentifierStepCount".into(),
                    concept_id: "activity:steps".into(),
                    value: ExactValue {
                        literal: "1200".into(),
                        unit: Some("count".into()),
                    },
                    time: SourceTime::Interval {
                        start: Some("2026-10-20T00:00:00-07:00".into()),
                        end: Some("2026-10-21T00:00:00-07:00".into()),
                    },
                    time_zone: Some("America/Los_Angeles".into()),
                    provenance: Provenance::default(),
                    original: None,
                })
                .collect(),
            deletions: Vec::new(),
            coverage: vec![TypeCoverage {
                data_type: "HKQuantityTypeIdentifierStepCount".into(),
                outcome: if end > start {
                    CoverageOutcome::Data
                } else {
                    CoverageOutcome::Empty
                },
            }],
            next_anchor: Some(Anchor(end.to_string())),
            has_more: end < self.count,
        })
    }
}

/// Accepts everything; can be told to lose the answer once.
#[derive(Default)]
struct Transport {
    sent: RefCell<Vec<UploadBatch>>,
    lose_next: RefCell<bool>,
    revisions: RefCell<HashMap<String, i64>>,
}
impl EvidenceTransport for Transport {
    async fn send(&self, batch: &UploadBatch) -> UploadOutcome {
        self.sent.borrow_mut().push(batch.clone());
        let acks = batch
            .items
            .iter()
            .map(|(op, m)| {
                let id = m["canonical_record_id"].as_str().unwrap().to_owned();
                let mut revisions = self.revisions.borrow_mut();
                let revision = revisions.entry(op.to_string()).or_insert_with(|| {
                    m.get("expected_revision")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0)
                        + 1
                });
                ItemAck::Accepted {
                    operation_id: *op,
                    canonical_record_id: id.parse().unwrap(),
                    revision: *revision,
                }
            })
            .collect();
        if std::mem::take(&mut *self.lose_next.borrow_mut()) {
            return UploadOutcome::Uncertain;
        }
        UploadOutcome::Delivered(acks)
    }
}

fn limits() -> OutboxLimits {
    OutboxLimits {
        page_size: 10,
        batch_items: 10,
        base_backoff_ms: 0,
        ..OutboxLimits::default()
    }
}

#[tokio::test]
async fn sign_out_and_account_switch_pause_and_isolate_the_queue() {
    let store = Arc::new(Store::default());
    let retention = RetentionOwner::new(store.clone());
    let a = retention.activate(identity("owner-a")).unwrap();
    let outbox_a = open_outbox(
        &retention,
        &a,
        "install-1",
        PhoneSource::AppleHealth,
        limits(),
    );
    outbox_a.connect(0).unwrap();
    let adapter = Adapter { count: 25 };
    let transport = Transport::default();
    // Acquire only; the upload is lost.
    *transport.lose_next.borrow_mut() = true;
    let report = sync(&outbox_a, &adapter, &transport, 0, SyncBudget::default()).await;
    assert_eq!(report.acquired.upserts, 25);
    assert_eq!(report.batches, 1);
    let pending_a = outbox_a.view().unwrap().items;
    assert_eq!(pending_a.len(), 25);

    // Switch to B: A's fence is dead, so A's outbox handle can neither read
    // nor upload, and B's outbox does not see A's work.
    let b = retention.activate(identity("owner-b")).unwrap();
    let late = sync(&outbox_a, &adapter, &transport, 0, SyncBudget::default()).await;
    assert_eq!(late.paused, Some(SyncPause::AccountChanged));
    assert_eq!(transport.sent.borrow().len(), 1);
    let outbox_b = open_outbox(
        &retention,
        &b,
        "install-1",
        PhoneSource::AppleHealth,
        limits(),
    );
    let view_b = outbox_b.view().unwrap();
    assert!(view_b.items.is_empty() && !view_b.connected);
    let report = sync(&outbox_b, &adapter, &transport, 0, SyncBudget::default()).await;
    assert_eq!(report.paused, Some(SyncPause::NotConnected));
    assert_eq!(transport.sent.borrow().len(), 1);
    // Sign out pauses everything.
    retention.sign_out().unwrap();
    let paused = sync(&outbox_b, &adapter, &transport, 0, SyncBudget::default()).await;
    assert_eq!(paused.paused, Some(SyncPause::AccountChanged));
    assert_eq!(transport.sent.borrow().len(), 1);

    // A signs back in: A's queue was retained (paused, not consumed) and
    // resumes with the original operation ids.
    let a_again = retention.activate(identity("owner-a")).unwrap();
    let outbox_a = open_outbox(
        &retention,
        &a_again,
        "install-1",
        PhoneSource::AppleHealth,
        limits(),
    );
    assert_eq!(outbox_a.view().unwrap().items, pending_a);
    let first_ops = transport.sent.borrow()[0].operation_ids();
    let report = sync(&outbox_a, &adapter, &transport, 0, SyncBudget::default()).await;
    assert_eq!(report.paused, None);
    assert_eq!(report.uploaded.acknowledged, 25);
    assert_eq!(transport.sent.borrow()[1].operation_ids(), first_ops);
}

#[tokio::test]
async fn same_account_resumes_after_restart_with_the_same_operation_ids() {
    let store = Arc::new(Store::default());
    let retention = RetentionOwner::new(store.clone());
    let fence = retention.activate(identity("owner-a")).unwrap();
    let outbox = open_outbox(
        &retention,
        &fence,
        "install-1",
        PhoneSource::AppleHealth,
        limits(),
    );
    outbox.connect(0).unwrap();
    let adapter = Adapter { count: 12 };
    let transport = Transport::default();
    *transport.lose_next.borrow_mut() = true;
    sync(&outbox, &adapter, &transport, 0, SyncBudget::default()).await;
    let first_ops = transport.sent.borrow()[0].operation_ids();

    // Process restart: a new RetentionOwner over the same protected store.
    let restarted = RetentionOwner::new(store.clone());
    let fence = restarted.activate(identity("owner-a")).unwrap();
    let outbox = open_outbox(
        &restarted,
        &fence,
        "install-1",
        PhoneSource::AppleHealth,
        limits(),
    );
    let report = sync(&outbox, &adapter, &transport, 1, SyncBudget::default()).await;
    assert_eq!(report.paused, None);
    assert_eq!(report.uploaded.acknowledged, 12);
    assert_eq!(transport.sent.borrow()[1].operation_ids(), first_ops);
    assert!(outbox.view().unwrap().items.is_empty());
}

#[tokio::test]
async fn locked_protected_storage_pauses_then_resumes() {
    let store = Arc::new(Store::default());
    let retention = RetentionOwner::new(store.clone());
    let fence = retention.activate(identity("owner-a")).unwrap();
    let outbox = open_outbox(
        &retention,
        &fence,
        "install-1",
        PhoneSource::AppleHealth,
        limits(),
    );
    outbox.connect(0).unwrap();
    let adapter = Adapter { count: 3 };
    let transport = Transport::default();
    *store.locked.lock().unwrap() = true;
    let report = sync(&outbox, &adapter, &transport, 0, SyncBudget::default()).await;
    assert_eq!(report.paused, Some(SyncPause::StorageUnavailable));
    assert!(transport.sent.borrow().is_empty());
    *store.locked.lock().unwrap() = false;
    let report = sync(&outbox, &adapter, &transport, 0, SyncBudget::default()).await;
    assert_eq!(report.paused, None);
    assert_eq!(report.uploaded.acknowledged, 3);
}

#[tokio::test]
async fn pending_secure_cleanup_refuses_queue_access() {
    let store = Arc::new(Store::default());
    let retention = RetentionOwner::new(store.clone());
    let fence = retention.activate(identity("owner-a")).unwrap();
    let outbox = open_outbox(
        &retention,
        &fence,
        "install-1",
        PhoneSource::AppleHealth,
        limits(),
    );
    *store.deny.lock().unwrap() = true;
    assert_eq!(
        outbox.connect(0),
        Err(OutboxError::Storage(StorageError::AccountChanged))
    );
}

#[test]
fn http_outcomes_are_classified_without_guessing() {
    let op = uuid::Uuid::new_v4();
    let record = uuid::Uuid::new_v4();
    let body = serde_json::json!({"results":[
        {"status":"accepted","index":0,"operation_id":op,
         "address":{"canonical_record_id":record,"record_id":"r","entity":"health_evidence","revision":3}},
        {"status":"rejected","index":1,"operation_id":op,
         "rejection":{"code":"source_authority_refused","message":"no"}}
    ]});
    let outcome = outcome_from_http(200, &serde_json::to_vec(&body).unwrap());
    assert_eq!(
        outcome,
        UploadOutcome::Delivered(vec![
            ItemAck::Accepted {
                operation_id: op,
                canonical_record_id: record,
                revision: 3
            },
            ItemAck::Rejected {
                operation_id: op,
                code: RejectionCode::SourceAuthorityRefused,
                message: "no".into()
            }
        ])
    );
    assert_eq!(
        outcome_from_http(200, b"{garbled"),
        UploadOutcome::Uncertain
    );
    assert_eq!(outcome_from_http(503, b""), UploadOutcome::Uncertain);
    assert_eq!(outcome_from_http(401, b""), UploadOutcome::Unauthorized);
}

/// The real client transport posts the exact frozen mutations to the batch
/// route and decodes per-item results.
#[tokio::test]
async fn duet_client_transport_posts_the_canonical_batch() {
    use axum::{Json, Router, routing::post};
    let seen: Arc<Mutex<Vec<serde_json::Value>>> = Arc::default();
    let captured = seen.clone();
    let app = Router::new().route(
        "/api/canonical-evidence/batch",
        post(move |Json(body): Json<serde_json::Value>| {
            let captured = captured.clone();
            async move {
                captured.lock().unwrap().push(body.clone());
                let results: Vec<_> = body["items"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .enumerate()
                    .map(|(index, m)| {
                        serde_json::json!({
                        "status":"accepted","index":index,"operation_id":m["operation_id"],
                        "address":{"canonical_record_id":m["canonical_record_id"],
                            "record_id":m["record_id"],"entity":"health_evidence","revision":1}})
                    })
                    .collect();
                Json(serde_json::json!({ "results": results }))
            }
        }),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

    let store = Arc::new(Store::default());
    let retention = RetentionOwner::new(store);
    let fence = retention.activate(identity("owner-a")).unwrap();
    let outbox = open_outbox(
        &retention,
        &fence,
        "install-1",
        PhoneSource::AppleHealth,
        limits(),
    );
    outbox.connect(2).unwrap();
    let client =
        duet_client::DuetClient::new(format!("http://{address}"), "token", "1", "1.0").unwrap();
    let report = sync(
        &outbox,
        &Adapter { count: 2 },
        &client,
        0,
        SyncBudget::default(),
    )
    .await;
    assert_eq!(report.paused, None);
    assert_eq!(report.uploaded.acknowledged, 2);
    let body = seen.lock().unwrap()[0].clone();
    let item = &body["items"][0];
    assert_eq!(item["operation"], "upsert");
    assert_eq!(item["source_generation"], 2);
    assert_eq!(item["entity"], "health_evidence");
    assert_eq!(item["payload"]["source"]["kind"], "appleHealth");
    assert_eq!(
        item["payload"]["measured_interval"]["start"],
        "2026-10-20T00:00:00-07:00"
    );
    assert_eq!(item["payload"]["timezone"], "America/Los_Angeles");
    let ledger = outbox.view().unwrap().ledger;
    assert_eq!(ledger["HKQuantitySample/0"].revision, Some(1));
    server.abort();
}
