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
    assert_eq!(
        report.progress(&outbox_a),
        Err(RefreshStop::WaitingForUpload)
    );
    assert_eq!(outbox_a.view().unwrap().items.len(), 25);

    // Switch to B: A's fence is dead, so A's outbox handle can neither read
    // nor upload, and B's outbox does not see A's work.
    let b = retention.activate(identity("owner-b")).unwrap();
    // A permission sheet opened by A may finish only after B has signed in.
    assert!(outbox_a.connect(0).is_err());
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
    // Sign out pauses everything and removes the signed-in account's queue.
    retention.sign_out().unwrap();
    let paused = sync(&outbox_b, &adapter, &transport, 0, SyncBudget::default()).await;
    assert_eq!(paused.paused, Some(SyncPause::AccountChanged));
    assert_eq!(transport.sent.borrow().len(), 1);

    // A signs back in to an empty queue. Switching accounts removed A's
    // unsent health uploads instead of keeping them for the next person.
    let a_again = retention.activate(identity("owner-a")).unwrap();
    let outbox_a = open_outbox(
        &retention,
        &a_again,
        "install-1",
        PhoneSource::AppleHealth,
        limits(),
    );
    assert!(outbox_a.view().unwrap().items.is_empty());
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
    assert_eq!(
        retention.health_installation(&fence),
        Err(HealthStorageError::Unavailable("locked".into()))
    );
    assert_eq!(
        report.progress(&outbox),
        Err(RefreshStop::Paused(SyncPause::StorageUnavailable(
            "locked".into()
        )))
    );
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
        post(
            move |headers: axum::http::HeaderMap, Json(body): Json<serde_json::Value>| {
                let captured = captured.clone();
                async move {
                    captured.lock().unwrap().push(body.clone());
                    if headers["authorization"] == "Bearer expired" {
                        return Err(axum::http::StatusCode::UNAUTHORIZED);
                    }
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
                    Ok(Json(serde_json::json!({ "results": results })))
                }
            },
        ),
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
    let expired =
        duet_client::DuetClient::new(format!("http://{address}"), "expired", "1", "1.0").unwrap();
    let report = sync(
        &outbox,
        &Adapter { count: 2 },
        &expired,
        0,
        SyncBudget::default(),
    )
    .await;
    assert_eq!(
        report.progress(&outbox),
        Err(RefreshStop::Paused(SyncPause::Unauthorized))
    );
    assert!(
        outbox
            .view()
            .unwrap()
            .items
            .iter()
            .all(|item| item.attempts == 0)
    );
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
    assert!(report.progress(&outbox).unwrap().is_break());
    let requests = seen.lock().unwrap();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0], requests[1]);
    let body = &requests[1];
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

/// Refuses the first item it ever sees and accepts the rest.
#[derive(Default)]
struct RefusesFirst {
    refused: RefCell<bool>,
}
impl EvidenceTransport for RefusesFirst {
    async fn send(&self, batch: &UploadBatch) -> UploadOutcome {
        let acks = batch
            .items
            .iter()
            .map(|(op, m)| {
                if !std::mem::replace(&mut *self.refused.borrow_mut(), true) {
                    return ItemAck::Rejected {
                        operation_id: *op,
                        code: RejectionCode::Invalid,
                        message: "refused".into(),
                    };
                }
                ItemAck::Accepted {
                    operation_id: *op,
                    canonical_record_id: m["canonical_record_id"]
                        .as_str()
                        .unwrap()
                        .parse()
                        .unwrap(),
                    revision: 1,
                }
            })
            .collect();
        UploadOutcome::Delivered(acks)
    }
}

#[tokio::test]
async fn a_retained_rejection_reports_only_after_the_queue_drains() {
    let retention = RetentionOwner::new(Arc::new(Store::default()));
    let fence = retention.activate(identity("owner-a")).unwrap();
    let outbox = open_outbox(
        &retention,
        &fence,
        "install-1",
        PhoneSource::AppleHealth,
        limits(),
    );
    outbox.connect(1).unwrap();
    let adapter = Adapter { count: 25 };
    let transport = RefusesFirst::default();
    let budget = SyncBudget {
        pages: 1,
        batches: 1,
    };
    let mut acknowledged = 0;
    let stop = loop {
        let report = sync(&outbox, &adapter, &transport, 0, budget).await;
        acknowledged += report.uploaded.acknowledged;
        match report.progress(&outbox) {
            Ok(flow) if flow.is_continue() => {}
            other => break other,
        }
    };
    assert_eq!(acknowledged, 24);
    assert_eq!(
        stop,
        Err(RefreshStop::Rejected(
            RejectionCode::Invalid,
            "refused".into()
        ))
    );
}

mod apple_health {
    use super::*;
    use axum::{
        Json, Router,
        http::StatusCode,
        routing::{get, post},
    };
    use duet_client::{
        DuetClient,
        apple_health::{AppleHealth, AppleHealthDevice, AppleHealthError},
    };
    use duet_health::{
        AuthorizationError,
        native_adapters::{
            BridgeError, HealthKitBridge, HealthKitChanges, HealthKitQuery, NativeSample,
        },
    };
    use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};

    /// Reads one heart-rate sample. `on_authorize` runs while the sheet is open.
    #[derive(Clone)]
    struct Device {
        authorized: Arc<AtomicUsize>,
        on_authorize: Arc<dyn Fn() + Send + Sync>,
    }
    impl Device {
        fn new(on_authorize: impl Fn() + Send + Sync + 'static) -> Self {
            Self {
                authorized: Arc::default(),
                on_authorize: Arc::new(on_authorize),
            }
        }
    }
    impl HealthKitBridge for Device {
        fn available(&self) -> bool {
            true
        }
        fn read_type(&self, query: &HealthKitQuery) -> Result<HealthKitChanges, BridgeError> {
            let wanted = query.data_type == "HKQuantityTypeIdentifierHeartRate";
            let samples = if wanted && query.anchor.is_none() {
                vec![NativeSample {
                    id: "a".into(),
                    value: "72".into(),
                    unit: Some("count/min".into()),
                    start: Some("2026-10-20T07:30:00+09:00".into()),
                    end: Some("2026-10-20T07:30:00+09:00".into()),
                    time_zone: Some("Asia/Tokyo".into()),
                    source_name: None,
                    writer_id: Some("com.apple.health".into()),
                    device: None,
                    metadata: None,
                }]
            } else {
                Vec::new()
            };
            Ok(HealthKitChanges {
                samples,
                deleted: Vec::new(),
                anchor: "next".into(),
                has_more: false,
            })
        }
    }
    impl AppleHealthDevice for Device {
        async fn authorize(&self) -> Result<(), AuthorizationError> {
            self.authorized.fetch_add(1, Ordering::SeqCst);
            (self.on_authorize)();
            Ok(())
        }
    }

    /// Local API: the account's generation and an accepting batch route.
    struct Api {
        client: DuetClient,
        generation: Arc<AtomicI32>,
        uploaded: Arc<AtomicUsize>,
    }
    async fn serve(generation: i32, refuse_session: bool) -> Api {
        let shared = Arc::new(AtomicI32::new(generation));
        let uploaded = Arc::new(AtomicUsize::default());
        let (read, count) = (shared.clone(), uploaded.clone());
        let app = Router::new()
            .route(
                "/api/apple-health/generation",
                get(move || async move {
                    if refuse_session {
                        return Err(StatusCode::UNAUTHORIZED);
                    }
                    Ok(Json(read.load(Ordering::SeqCst)))
                }),
            )
            .route(
                "/api/canonical-evidence/batch",
                post(move |Json(body): Json<serde_json::Value>| async move {
                    let items = body["items"].as_array().unwrap();
                    count.fetch_add(items.len(), Ordering::SeqCst);
                    let results: Vec<_> = items
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
                }),
            );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        Api {
            client: DuetClient::new(format!("http://{address}"), "token", "1", "1.0").unwrap(),
            generation: shared,
            uploaded,
        }
    }

    fn signed_in() -> (RetentionOwner, duet_client::session::SessionFence) {
        let retention = RetentionOwner::new(Arc::new(Store::default()));
        let fence = retention.activate(identity("owner-a")).unwrap();
        (retention, fence)
    }

    #[tokio::test]
    async fn refresh_imports_through_the_connected_queue() {
        let api = serve(2, false).await;
        let (retention, fence) = signed_in();
        let health = AppleHealth::open(&retention, &fence, Device::new(|| {})).unwrap();
        assert!(!health.connected().unwrap());
        health.connect(&api.client).await.unwrap();
        assert!(health.connected().unwrap());
        health.refresh(&api.client).await.unwrap();
        assert_eq!(api.uploaded.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn a_withdrawal_during_the_sheet_cannot_authorize_a_later_generation() {
        let api = serve(3, false).await;
        let (retention, fence) = signed_in();
        let bumped = api.generation.clone();
        let device = Device::new(move || bumped.store(4, Ordering::SeqCst));
        let health = AppleHealth::open(&retention, &fence, device).unwrap();
        health.connect(&api.client).await.unwrap();
        let stopped = health.refresh(&api.client).await.unwrap_err();
        assert!(matches!(
            stopped,
            AppleHealthError::Stopped(RefreshStop::Paused(SyncPause::StaleGeneration))
        ));
        assert!(!health.connected().unwrap());
        assert_eq!(api.uploaded.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn a_refused_session_never_opens_the_sheet() {
        let api = serve(1, true).await;
        let (retention, fence) = signed_in();
        let device = Device::new(|| {});
        let health = AppleHealth::open(&retention, &fence, device.clone()).unwrap();
        let refused = health.connect(&api.client).await.unwrap_err();
        assert!(matches!(refused, AppleHealthError::SessionRefused(_)));
        assert_eq!(device.authorized.load(Ordering::SeqCst), 0);
        assert!(!health.connected().unwrap());
    }

    #[tokio::test]
    async fn a_late_permission_answer_cannot_connect_the_next_patient() {
        let api = serve(1, false).await;
        let (retention, fence) = signed_in();
        let switching = retention.clone();
        let device = Device::new(move || {
            switching.activate(identity("owner-b")).unwrap();
        });
        let health = AppleHealth::open(&retention, &fence, device).unwrap();
        let stopped = health.connect(&api.client).await.unwrap_err();
        assert!(matches!(
            stopped,
            AppleHealthError::Stopped(RefreshStop::Paused(SyncPause::AccountChanged))
        ));
        let next = retention.session().fence().unwrap();
        let theirs = AppleHealth::open(&retention, &next, Device::new(|| {})).unwrap();
        assert!(!theirs.connected().unwrap());
    }
}
