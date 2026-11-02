//! The phone outbox drained into the real canonical evidence owner on
//! disposable loopback PostgreSQL. Requires `DUET_TEST_DATABASE_URL`.
use async_trait::async_trait;
use chrono::{Duration, Utc};
use duet_api::{
    ApiError,
    database::{AccountDatabase, migrate},
    evidence::{EvidenceBatch, EvidenceItemOutcome, EvidenceRejectionCode, EvidenceStore},
    identity::{AccountContext, IdentityStore, IdentityVerifier, VerifiedIdentity},
};
use duet_client::health_upload::{EvidenceTransport, SyncBudget, SyncPause, sync, upload};
use duet_health::acquisition::*;
use duet_health::outbox::*;
use sqlx::{AssertSqlSafe, ConnectOptions, Connection, PgConnection};
use std::cell::RefCell;
use std::sync::Arc;
use uuid::Uuid;

struct Verifier;
#[async_trait]
impl IdentityVerifier for Verifier {
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, ApiError> {
        Ok(VerifiedIdentity {
            subject: format!("provider-subject-{bearer}"),
            authenticated_at: Utc::now() - Duration::minutes(5),
        })
    }
}

#[derive(Clone, Default)]
struct Disk(std::rc::Rc<RefCell<Option<Vec<u8>>>>);
impl OutboxStorage for Disk {
    fn load(&self) -> Result<Option<Vec<u8>>, StorageError> {
        Ok(self.0.borrow().clone())
    }
    fn save(&self, bytes: &[u8]) -> Result<(), StorageError> {
        *self.0.borrow_mut() = Some(bytes.to_vec());
        Ok(())
    }
}

/// Calls the device batch owner directly; can drop the answer after commit.
struct Direct {
    store: EvidenceStore,
    context: AccountContext,
    lose_answer: RefCell<bool>,
}
impl EvidenceTransport for Direct {
    async fn send(&self, batch: &UploadBatch) -> UploadOutcome {
        let body: EvidenceBatch = serde_json::from_value(batch.body()).unwrap();
        let result = match self.store.ingest_batch(&self.context, &body).await {
            Ok(result) => result,
            Err(_) => return UploadOutcome::Uncertain,
        };
        if std::mem::take(&mut *self.lose_answer.borrow_mut()) {
            return UploadOutcome::Uncertain;
        }
        UploadOutcome::Delivered(
            result
                .results
                .into_iter()
                .map(|outcome| match outcome {
                    EvidenceItemOutcome::Accepted {
                        operation_id,
                        address,
                        ..
                    } => ItemAck::Accepted {
                        operation_id,
                        canonical_record_id: address.canonical_record_id,
                        revision: i64::from(address.revision),
                    },
                    EvidenceItemOutcome::Rejected {
                        operation_id,
                        rejection,
                        ..
                    } => ItemAck::Rejected {
                        operation_id,
                        code: match rejection.code {
                            EvidenceRejectionCode::Conflict => RejectionCode::Conflict,
                            EvidenceRejectionCode::SourceAuthorityRefused => {
                                RejectionCode::SourceAuthorityRefused
                            }
                            EvidenceRejectionCode::Invalid => RejectionCode::Invalid,
                            EvidenceRejectionCode::NotFound => RejectionCode::NotFound,
                        },
                        message: rejection.message,
                    },
                })
                .collect(),
        )
    }
}

struct Log(
    RefCell<Vec<Option<AcquisitionRecord>>>,
    RefCell<Vec<String>>,
);
impl HealthSourceAdapter for Log {
    fn source(&self) -> PhoneSource {
        PhoneSource::AppleHealth
    }
    fn read_page(&self, request: &PageRequest) -> Result<AcquisitionPage, AdapterError> {
        let start: usize = request.anchor.as_ref().map_or(0, |a| a.0.parse().unwrap());
        let log = self.0.borrow();
        let ids = self.1.borrow();
        let end = (start + request.limit).min(log.len());
        let mut page = AcquisitionPage::default();
        for (index, entry) in log[start..end].iter().enumerate() {
            match entry {
                Some(record) => page.records.push(record.clone()),
                None => page.deletions.push(DeletionRecord {
                    native_id: ids[start + index].clone(),
                    data_type: "HKQuantityTypeIdentifierHeartRate".into(),
                }),
            }
        }
        page.coverage = vec![TypeCoverage {
            data_type: "HKQuantityTypeIdentifierHeartRate".into(),
            outcome: if end > start {
                CoverageOutcome::Data
            } else {
                CoverageOutcome::Empty
            },
        }];
        page.next_anchor = Some(Anchor(end.to_string()));
        page.has_more = end < log.len();
        Ok(page)
    }
}
impl Log {
    fn put(&self, id: usize, literal: &str) {
        self.0.borrow_mut().push(Some(AcquisitionRecord {
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
                writer_id: Some("com.apple.health".into()),
                device: None,
            },
            original: None,
        }));
        self.1.borrow_mut().push(String::new());
    }
    fn delete(&self, id: usize) {
        self.0.borrow_mut().push(None);
        self.1.borrow_mut().push(format!("HKQuantitySample/{id}"));
    }
}

#[tokio::test]
async fn phone_outbox_drains_into_canonical_evidence_on_postgres() {
    let url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("DUET_TEST_DATABASE_URL must point at disposable loopback PostgreSQL");
    let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(["localhost", "127.0.0.1", "::1"].contains(&options.get_host()));
    let mut admin = PgConnection::connect(&url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let role = format!("duet_phone_{suffix}");
    let name = format!("duet_phone_{suffix}");
    sqlx::raw_sql(AssertSqlSafe(format!("create role {role} login")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::raw_sql(AssertSqlSafe(format!("create database {name}")))
        .execute(&mut admin)
        .await
        .unwrap();
    let owner_url = options.clone().database(&name).to_url_lossy().to_string();
    let runtime_url = options
        .database(&name)
        .username(&role)
        .to_url_lossy()
        .to_string();
    let mut owner = PgConnection::connect(&owner_url).await.unwrap();
    sqlx::raw_sql("revoke create on schema public from public")
        .execute(&mut owner)
        .await
        .unwrap();
    migrate(&owner_url, &role).await.unwrap();
    let db = AccountDatabase::connect(&runtime_url).await.unwrap();
    let identities = IdentityStore::new(db.clone(), Arc::new(Verifier));
    let a = identities.authenticate("a", true).await.unwrap();
    let store = EvidenceStore::new(db.clone());
    let transport = Direct {
        store: store.clone(),
        context: a.clone(),
        lose_answer: RefCell::new(false),
    };
    let outbox = Outbox::new(
        Disk::default(),
        OutboxScope {
            account: a.account_id.to_string(),
            installation: "install-1".into(),
            source: PhoneSource::AppleHealth,
        },
        OutboxLimits {
            page_size: 40,
            batch_items: 30,
            base_backoff_ms: 0,
            ..OutboxLimits::default()
        },
    );
    outbox.connect(0).unwrap();
    let log = Log(RefCell::default(), RefCell::default());
    for i in 0..70 {
        log.put(i, "072");
    }

    // Multi-page acquisition; first batch commits but its answer is lost.
    *transport.lose_answer.borrow_mut() = true;
    let report = sync(&outbox, &log, &transport, 0, SyncBudget::default()).await;
    assert_eq!(report.acquired.upserts, 70);
    assert_eq!(report.uploaded.retrying, 30);
    // Retry with the same operation ids replays the committed acknowledgement.
    let (finished, _) = upload(&outbox, &transport, 1, 10).await.unwrap();
    assert_eq!(finished.acknowledged, 70);
    assert_eq!(finished.rejected, 0);
    let snapshot = store.snapshot(&a, 0).await.unwrap();
    let count: i64 = sqlx::query_scalar("select count(*) from health_evidence where user_id=$1")
        .bind(a.account_id)
        .fetch_one(&mut owner)
        .await
        .unwrap();
    assert_eq!(count, 70, "{snapshot:?}");
    let (measured, payload): (Option<chrono::DateTime<Utc>>, serde_json::Value) =
        sqlx::query_as("select measured_at,payload from health_evidence where user_id=$1 and payload->'source'->>'record_id'='HKQuantitySample/0'")
            .bind(a.account_id)
            .fetch_one(&mut owner)
            .await
            .unwrap();
    assert_eq!(
        measured.unwrap(),
        "2026-10-20T07:30:00+09:00"
            .parse::<chrono::DateTime<Utc>>()
            .unwrap()
    );
    assert_eq!(payload["value"]["literal"], "072");
    assert_eq!(payload["timezone"], "Asia/Tokyo");

    // A correction and an explicit deletion follow acknowledged revisions.
    let before = outbox.view().unwrap().ledger["HKQuantitySample/1"]
        .revision
        .unwrap();
    log.put(1, "073");
    log.delete(2);
    let report = sync(&outbox, &log, &transport, 2, SyncBudget::default()).await;
    assert_eq!(report.paused, None);
    assert_eq!((report.acquired.upserts, report.acquired.deletions), (1, 1));
    assert_eq!(report.uploaded.acknowledged, 2);
    let view = outbox.view().unwrap();
    assert!(view.items.is_empty() && view.rejected.is_empty());
    assert!(view.ledger["HKQuantitySample/1"].revision.unwrap() > before);
    assert!(!view.ledger.contains_key("HKQuantitySample/2"));

    // Committed upsert, lost answer, same-generation disconnect, reconnect,
    // correction and deletion stay on one canonical id.
    log.put(80, "090");
    *transport.lose_answer.borrow_mut() = true;
    let report = sync(&outbox, &log, &transport, 4, SyncBudget::default()).await;
    assert_eq!(report.acquired.upserts, 1);
    assert_eq!(report.uploaded.retrying, 1);
    assert_eq!(report.uploaded.acknowledged, 0);
    let queued = outbox.view().unwrap();
    let operation_id = queued.items[0].operation_id;
    let canonical = queued.ledger["HKQuantitySample/80"].canonical_record_id;
    let anchor = queued.anchor.clone();
    assert_eq!(queued.ledger["HKQuantitySample/80"].revision, None);
    assert!(queued.items[0].frozen.is_some());

    outbox.disconnect().unwrap();
    let disconnected = outbox.view().unwrap();
    assert!(!disconnected.connected);
    assert!(disconnected.fenced.is_empty());
    assert_eq!(disconnected.anchor, anchor);
    assert_eq!(disconnected.items[0].operation_id, operation_id);
    assert_eq!(
        disconnected.ledger["HKQuantitySample/80"].canonical_record_id,
        canonical
    );
    let paused = sync(&outbox, &log, &transport, 4, SyncBudget::default()).await;
    assert_eq!(paused.paused, Some(SyncPause::NotConnected));
    assert_eq!(outbox.view().unwrap().items[0].operation_id, operation_id);

    outbox.connect(0).unwrap();
    assert!(outbox.view().unwrap().connected);
    assert_eq!(outbox.view().unwrap().items[0].operation_id, operation_id);
    let (finished, _) = upload(&outbox, &transport, 5, 10).await.unwrap();
    assert_eq!((finished.acknowledged, finished.rejected), (1, 0));
    assert_eq!(
        outbox.view().unwrap().ledger["HKQuantitySample/80"].canonical_record_id,
        canonical
    );
    let learned = outbox.view().unwrap().ledger["HKQuantitySample/80"]
        .revision
        .unwrap();
    assert!(learned > 0);
    let replayed: i64 = sqlx::query_scalar(
        "select count(*) from duet_evidence_ingestions where user_id=$1 and operation_id=$2",
    )
    .bind(a.account_id)
    .bind(operation_id)
    .fetch_one(&mut owner)
    .await
    .unwrap();
    assert_eq!(replayed, 1);

    log.put(80, "091");
    let report = sync(&outbox, &log, &transport, 6, SyncBudget::default()).await;
    assert_eq!(
        (report.uploaded.acknowledged, report.uploaded.rejected),
        (1, 0)
    );
    assert_eq!(
        outbox.view().unwrap().ledger["HKQuantitySample/80"].canonical_record_id,
        canonical
    );
    assert!(
        outbox.view().unwrap().ledger["HKQuantitySample/80"]
            .revision
            .unwrap()
            > learned
    );
    let corrected: serde_json::Value =
        sqlx::query_scalar("select payload from health_evidence where user_id=$1 and id=$2")
            .bind(a.account_id)
            .bind(canonical)
            .fetch_one(&mut owner)
            .await
            .unwrap();
    assert_eq!(corrected["value"]["literal"], "091");

    log.delete(80);
    let report = sync(&outbox, &log, &transport, 7, SyncBudget::default()).await;
    assert_eq!(
        (report.uploaded.acknowledged, report.uploaded.rejected),
        (1, 0)
    );
    assert!(
        !outbox
            .view()
            .unwrap()
            .ledger
            .contains_key("HKQuantitySample/80")
    );
    let (stored_id, deleted, source_removed): (Uuid, bool, bool) = sqlx::query_as(
        "select record_id, deleted, source_removed from evidence_sync_changes where user_id=$1 and client_record_id='HKQuantitySample/80'",
    )
    .bind(a.account_id)
    .fetch_one(&mut owner)
    .await
    .unwrap();
    assert_eq!(stored_id, canonical);
    assert!(deleted && source_removed);
    let operations: i64 = sqlx::query_scalar(
        "select count(*) from duet_evidence_ingestions where user_id=$1 and address->>'canonical_record_id'=$2",
    )
    .bind(a.account_id)
    .bind(canonical.to_string())
    .fetch_one(&mut owner)
    .await
    .unwrap();
    assert_eq!(operations, 3);
    let revisions: i64 = sqlx::query_scalar(
        "select count(*) from canonical_evidence_revisions where user_id=$1 and evidence_id=$2",
    )
    .bind(a.account_id)
    .bind(canonical)
    .fetch_one(&mut owner)
    .await
    .unwrap();
    assert_eq!(revisions, 2);
    let snapshot = store.snapshot(&a, 0).await.unwrap();
    assert!(
        snapshot
            .records
            .iter()
            .all(|record| record.address.canonical_record_id != canonical)
    );
    let change = snapshot
        .changes
        .iter()
        .find(|change| change.record_id == "HKQuantitySample/80")
        .unwrap();
    assert_eq!(change.canonical_record_id, canonical);
    assert!(change.deleted && change.source_removed);

    // The server's source generation advances (reconnect elsewhere): stale
    // work is refused as a conflict and retained, not dropped.
    sqlx::query("update users set apple_health_generation=apple_health_generation+1 where id=$1")
        .bind(a.account_id)
        .execute(&mut owner)
        .await
        .unwrap();
    log.put(3, "080");
    let report = sync(&outbox, &log, &transport, 3, SyncBudget::default()).await;
    assert_eq!(report.uploaded.rejected, 1);
    let view = outbox.view().unwrap();
    assert_eq!(view.rejected[0].code, RejectionCode::Conflict);
    // Reconnecting under the new generation fences nothing pending and
    // restarts acquisition from the beginning under fresh identities.
    outbox.connect(1).unwrap();
    assert_eq!(outbox.view().unwrap().anchor, None);
    let _ = sqlx::raw_sql(AssertSqlSafe(format!(
        "drop database if exists {name} with (force)"
    )))
    .execute(&mut admin)
    .await;
}
