//! Real PostgreSQL proof from an empty database; producer rows are never SQL seeded.
use sqlx::AssertSqlSafe;
// Compile the assigned module before coordinator wiring; no shared lib edits required.
pub use duet_api::{ApiError, database, evidence, identity};
#[path = "../src/health_context.rs"]
mod health_context;
use chrono::{Duration, Utc};
use duet_api::{
    database::AccountDatabase,
    evidence::{EvidenceMutation, EvidenceStore},
    identity::{IdentityStore, IdentityVerifier, VerifiedIdentity},
    privacy::{FileJournal, PrivacyStore, Restriction},
};
use duet_wire::models::addressed_record_revision::Entity;
use health_context::{HealthContextProducer, Upload};
use serde_json::json;
use sqlx::{ConnectOptions, Connection};
use std::sync::Arc;
use uuid::Uuid;
struct TestIdentity;
#[async_trait::async_trait]
impl IdentityVerifier for TestIdentity {
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, ApiError> {
        Ok(VerifiedIdentity {
            subject: bearer.to_owned(),
            authenticated_at: Utc::now(),
        })
    }
}
fn upload(expected: Option<i64>) -> Upload {
    let today = Utc::now().date_naive();
    serde_json::from_value(json!({"expected_revision":expected,"snapshot":{"id":Uuid::new_v4(),"source_boundary_id":Uuid::new_v4(),"schema_version":1,"generated_at":Utc::now(),"timezone":"UTC","source_generations":{},"projections":[{"kind":"daily","window_start":today.and_hms_opt(0,0,0).unwrap().and_utc(),"window_end":(today+Duration::days(1)).and_hms_opt(0,0,0).unwrap().and_utc(),"timezone":"UTC","first_weekday":2,"facts":[]}]}})).unwrap()
}
fn measurement(day: i64, value: f64, issuer: &str) -> EvidenceMutation {
    let id = Uuid::new_v4();
    let at = (Utc::now().date_naive() - Duration::days(day))
        .and_hms_opt(12, 0, 0)
        .unwrap()
        .and_utc();
    EvidenceMutation {
        operation_id: Uuid::new_v4(),
        canonical_record_id: id,
        record_id: format!("source-{id}"),
        entity: Entity::HealthEvidence,
        expected_revision: None,
        source_generation: Some(0),
        payload: json!({"id":id,"concept_id":"steps","display_name":"Steps","verification":"verified","method":"daily_aggregate","measured_interval":{"start":at},"source":{"kind":"appleHealth","connection_id":"phone","issuer":issuer,"provenance":"native"},"value":{"type":"quantity","quantity":{"canonical_value":value,"canonical_unit":"count"}},"context":{"aggregation":"daily_sum","day_timezone":"UTC","day_partial":false}}),
    }
}
#[tokio::test]
async fn empty_database_producer_restart_fences_and_withdrawal() {
    let url = std::env::var("DUET_TEST_DATABASE_URL").expect(
        "DUET_TEST_DATABASE_URL must be a disposable local PostgreSQL admin URL; test never skips",
    );
    let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(matches!(
        options.get_host(),
        "localhost" | "127.0.0.1" | "::1"
    ));
    let mut admin = sqlx::PgConnection::connect(&url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let reader = format!("hc_read_{suffix}");
    let writer = format!("hc_write_{suffix}");
    let name = format!("hc_test_{suffix}");
    sqlx::raw_sql(AssertSqlSafe(format!(
        "create role {reader} login; create role {writer} login;"
    )))
    .execute(&mut admin)
    .await
    .unwrap();
    sqlx::raw_sql(AssertSqlSafe(format!("create database {name}")))
        .execute(&mut admin)
        .await
        .unwrap();
    let options = options.database(&name);
    let owner_url = options.to_url_lossy().to_string();
    let reader_url = options.clone().username(&reader).to_url_lossy().to_string();
    let writer_url = options.username(&writer).to_url_lossy().to_string();
    let mut owner = sqlx::PgConnection::connect(&owner_url).await.unwrap();
    sqlx::raw_sql("revoke create on schema public from public")
        .execute(&mut owner)
        .await
        .unwrap();
    database::migrate(&owner_url, &reader).await.unwrap();
    // Until coordinator registers migration 0010, apply only schema (never producer data).
    let installed: bool = sqlx::query_scalar(
        "select to_regclass('public.duet_health_context_operations') is not null",
    )
    .fetch_one(&mut owner)
    .await
    .unwrap();
    if !installed {
        sqlx::raw_sql(include_str!(
            "../migrations/0010_health_context_producer.sql"
        ))
        .execute(&mut owner)
        .await
        .unwrap();
    }
    sqlx::raw_sql(AssertSqlSafe(format!("grant usage on schema public to {writer}; grant select on users,evidence_sync_changes,canonical_evidence_revisions,health_contexts,duet_health_context_operations,privacy_recovery_events to {writer}; grant update(id) on users to {writer}; grant insert,update on health_contexts to {writer}; grant insert on duet_health_context_operations to {writer}; grant execute on function acting_account() to {writer};"))).execute(&mut owner).await.unwrap();
    let db = AccountDatabase::connect(&reader_url).await.unwrap();
    let identities = IdentityStore::new(db.clone(), Arc::new(TestIdentity));
    let a = identities.authenticate("producer-a", true).await.unwrap();
    let b = identities.authenticate("producer-b", true).await.unwrap();
    let source = EvidenceStore::new(db.clone());
    let producer = HealthContextProducer::connect(&writer_url).await.unwrap();
    assert!(HealthContextProducer::connect(&reader_url).await.is_err());
    assert!(producer.read(&a).await.unwrap().is_none());
    let mut mutations = vec![];
    for (day, value, issuer) in [
        (1, 100., "watch-a"),
        (2, 200., "watch-a"),
        (8, 50., "watch-a"),
        (1, 900., "watch-b"),
    ] {
        let m = measurement(day, value, issuer);
        source.ingest(&a, &m).await.unwrap();
        mutations.push(m);
    }
    // Unsupported literal bound remains explicitly uncalculated, not interpreted as a scalar.
    let mut bound = measurement(1, 7., "watch-a");
    bound.payload["value"]["quantity"]["comparator"] = json!("<");
    source.ingest(&a, &bound).await.unwrap();
    // Source-record civil dates and named-zone midnights are PostgreSQL-backed,
    // not fixed offsets: the fall-back day is 25 hours.
    let mut dst = measurement(1, 65., "dst-source");
    dst.payload["concept_id"] = json!("pulse");
    dst.payload["display_name"] = json!("Pulse");
    dst.payload["measured_interval"]["start"] = json!("2025-11-02T12:00:00Z");
    dst.payload["context"]["day_timezone"] = json!("America/Los_Angeles");
    dst.payload["source"]["projected_date_role"] = json!("effectiveDateTime");
    dst.payload["source"]["declared_dates"] = json!({"effectiveDateTime":"2025-11-02"});
    source.ingest(&b, &dst).await.unwrap();
    let dst_context = producer.upload(&b, &upload(None)).await.unwrap();
    let dst_fact = dst_context.snapshot.projections[0]
        .facts
        .iter()
        .find(|f| f.kind == duet_wire::models::HealthFactKind::CurrentValue)
        .unwrap();
    assert_eq!((dst_fact.range_end - dst_fact.range_start).num_hours(), 25);
    let request = upload(None);
    let stored = producer.upload(&a, &request).await.unwrap();
    assert_eq!(stored.revision, 1);
    assert_eq!(stored.uncalculated.len(), 1);
    assert_eq!(stored.dependencies.len(), 4);
    let mut validation = db.begin(a.account_id).await.unwrap();
    identity::check_current(&mut validation, &a, false)
        .await
        .unwrap();
    health_context::validate_current_in_transaction(&mut validation, a.account_id)
        .await
        .unwrap();
    validation.rollback().await.unwrap();
    let facts = &stored.snapshot.projections[0].facts;
    let currents: Vec<_> = facts
        .iter()
        .filter(|f| f.kind == duet_wire::models::HealthFactKind::CurrentValue)
        .collect();
    assert_eq!(currents.len(), 2);
    assert!(currents.iter().any(|f| f.value == Some(Some(100.))));
    assert!(currents.iter().any(|f| f.value == Some(Some(900.))));
    assert!(
        facts
            .iter()
            .any(|f| f.kind == duet_wire::models::HealthFactKind::Variability)
    );
    let restarted = HealthContextProducer::connect(&writer_url).await.unwrap();
    assert_eq!(stored, restarted.upload(&a, &request).await.unwrap());
    assert_eq!(stored, restarted.read(&a).await.unwrap().unwrap());
    assert_eq!(restarted.read(&b).await.unwrap().unwrap(), dst_context);
    let refreshed = restarted.refresh(&a).await.unwrap().unwrap();
    assert_eq!(stored, refreshed);
    let mut reused = request.clone();
    reused.snapshot.timezone = "America/Los_Angeles".into();
    assert!(restarted.upload(&a, &reused).await.is_err());
    // Opaque PreparedContext is bound to the account and canonical revision.
    let candidate = upload(Some(1));
    let prepared = restarted.prepare_upload(&a, &candidate).await.unwrap();
    assert!(restarted.publish(&b, prepared).await.is_err());
    let prepared = restarted.prepare_upload(&a, &candidate).await.unwrap();
    let old = source
        .snapshot(&a, 0)
        .await
        .unwrap()
        .records
        .into_iter()
        .find(|r| r.address.canonical_record_id == mutations[0].canonical_record_id)
        .unwrap();
    let mut correction = mutations[0].clone();
    correction.operation_id = Uuid::new_v4();
    correction.expected_revision = Some(i64::from(old.address.revision));
    correction.payload["value"]["quantity"]["canonical_value"] = json!(321.);
    source.ingest(&a, &correction).await.unwrap();
    assert!(restarted.publish(&a, prepared).await.is_err());
    assert!(restarted.read(&a).await.is_err());
    assert!(restarted.upload(&a, &request).await.is_err());
    let corrected = restarted.refresh(&a).await.unwrap().unwrap();
    assert_eq!(corrected.revision, 2);
    assert_ne!(corrected.snapshot.id, stored.snapshot.id);
    assert_ne!(
        corrected.snapshot.source_boundary_id,
        stored.snapshot.source_boundary_id
    );
    assert!(
        corrected.snapshot.projections[0]
            .facts
            .iter()
            .any(|f| f.value == Some(Some(321.)))
    );
    // Ordinary reader cannot inject facts, even into its own account.
    let mut tx = db.begin(a.account_id).await.unwrap();
    assert!(
        sqlx::query("update health_contexts set facts='[]' where user_id=$1")
            .bind(a.account_id)
            .execute(&mut *tx)
            .await
            .is_err()
    );
    tx.rollback().await.unwrap();
    let journal = FileJournal::open(std::env::temp_dir().join(format!("hc-journal-{suffix}")))
        .await
        .unwrap();
    let privacy = PrivacyStore::new(db.clone(), Arc::new(journal));
    privacy
        .restrict(
            &a,
            Uuid::new_v4(),
            Restriction::DeviceWithdraw,
            a.account_id,
            0,
        )
        .await
        .unwrap();
    assert!(restarted.read(&a).await.is_err());
    assert!(restarted.upload(&a, &request).await.is_err());
    let withdrawn = restarted.refresh(&a).await.unwrap().unwrap();
    assert!(withdrawn.snapshot.projections[0].facts.is_empty());
    assert!(withdrawn.dependencies.is_empty());
    let mut native = upload(Some(withdrawn.revision));
    native
        .snapshot
        .source_generations
        .insert("apple_health_generation".into(), 1);
    let start = native.snapshot.projections[0].window_start;
    let end = native.snapshot.projections[0].window_end;
    native.snapshot.projections[0].facts.push(serde_json::from_value(json!({"id":"native-workout-1","kind":"current_value","concept_ids":["workout"],"title":"Native workout","window":"day","range_start":start,"range_end":end,"value":30.,"unit":"min","evidence":[{"id":"workout:1","kind":"workout","title":"Workout","source_kind":"appleHealth"}]})).unwrap());
    let native_stored = restarted.upload(&a, &native).await.unwrap();
    assert_eq!(native_stored.snapshot.projections[0].facts.len(), 1);
    let mut spoof = upload(Some(native_stored.revision));
    spoof
        .snapshot
        .source_generations
        .insert("apple_health_generation".into(), 1);
    spoof.snapshot.projections[0]
        .facts
        .push(stored.snapshot.projections[0].facts[0].clone());
    assert!(restarted.upload(&a, &spoof).await.is_err());
    drop(owner);
    drop(producer);
    drop(restarted);
    drop(source);
    drop(identities);
    drop(db);
    drop(privacy);
    sqlx::raw_sql(AssertSqlSafe(format!("drop database {name} with (force)")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::raw_sql(AssertSqlSafe(format!(
        "drop role {reader}; drop role {writer}"
    )))
    .execute(&mut admin)
    .await
    .unwrap();
}
