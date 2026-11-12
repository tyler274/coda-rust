//! Fresh-install HTTP health-context producer, account isolation and restriction.
//! All product rows originate through authenticated product APIs; SQL only
//! provisions isolated roles/schema and inspects durability/authority proofs.
#![allow(
    clippy::disallowed_methods,
    reason = "converting to the typed database API; the proof also provisions databases"
)]
use async_trait::async_trait;
use axum::{Router, body::Body, http::Request};
use chrono::{Duration, Utc};
use duet_api::{
    ApiError,
    database::{AccountDatabase, migrate},
    durable::DurableState,
    evidence::EvidenceStore,
    identity::{IdentityStore, IdentityVerifier, VerifiedIdentity},
    privacy::{FileJournal, PrivacyStore},
    producer_database::{ProducerKind, grant},
    producer_routes::ProducerState,
};
use serde_json::{Value, json};
use sqlx::AssertSqlSafe;
use sqlx::{ConnectOptions, Connection};
use std::sync::Arc;
use tower::ServiceExt;
use uuid::Uuid;
struct Identity;
#[async_trait]
impl IdentityVerifier for Identity {
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, ApiError> {
        if !matches!(bearer, "a" | "b") {
            return Err(ApiError::Unauthorized);
        }
        Ok(VerifiedIdentity {
            subject: format!("fresh-{bearer}"),
            authenticated_at: Utc::now(),
        })
    }
}
async fn http(app: &Router, method: &str, path: &str, who: &str, body: Value) -> (u16, Value) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .header("authorization", format!("Bearer {who}"))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status().as_u16();
    let bytes = axum::body::to_bytes(response.into_body(), 2_000_000)
        .await
        .unwrap();
    (
        status,
        serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| json!({"text":String::from_utf8_lossy(&bytes)})),
    )
}
async fn ok(app: &Router, method: &str, path: &str, who: &str, body: Value) -> Value {
    let (status, value) = http(app, method, path, who, body).await;
    assert!(
        (200..300).contains(&status),
        "{method} {path}: {status} {value}"
    );
    value
}
#[tokio::test]
async fn fresh_http_health_context_isolates_accounts_and_restricts() {
    let url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("real disposable loopback PostgreSQL required");
    let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(matches!(
        options.get_host(),
        "localhost" | "127.0.0.1" | "::1"
    ));
    let mut admin = sqlx::PgConnection::connect(&url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let name = format!("fresh_{suffix}");
    let reader = format!("reader_{suffix}");
    let context_role = format!("context_{suffix}");
    let executor = format!("executor_{suffix}");
    for role in [&reader, &context_role, &executor] {
        sqlx::query(AssertSqlSafe(format!("create role {role} login")))
            .execute(&mut admin)
            .await
            .unwrap();
    }
    sqlx::query(AssertSqlSafe(format!("create database {name}")))
        .execute(&mut admin)
        .await
        .unwrap();
    let options = options.database(&name);
    let owner_url = options.to_url_lossy().to_string();
    for role in [&reader, &context_role, &executor] {
        migrate(&owner_url, role).await.unwrap();
    }
    grant(&owner_url, &context_role, ProducerKind::HealthContext)
        .await
        .unwrap();
    grant(&owner_url, &executor, ProducerKind::AcceptedWork)
        .await
        .unwrap();
    let reader_url = options.clone().username(&reader).to_url_lossy().to_string();
    let context_url = options
        .clone()
        .username(&context_role)
        .to_url_lossy()
        .to_string();
    let executor_url = options.username(&executor).to_url_lossy().to_string();
    let db = AccountDatabase::connect(&reader_url).await.unwrap();
    let write_db = AccountDatabase::connect(&executor_url).await.unwrap();
    let journal_path = std::env::temp_dir().join(format!("fresh-journal-{suffix}"));
    let durable = DurableState {
        identity: IdentityStore::new(db.clone(), Arc::new(Identity)),
        evidence: EvidenceStore::new(db.clone()),
        privacy: PrivacyStore::new(
            write_db.clone(),
            Arc::new(FileJournal::open(journal_path.clone()).await.unwrap()),
        ),
    };
    let producer = duet_api::health_context::HealthContextProducer::connect(&context_url)
        .await
        .unwrap();
    let app = duet_api::durable::router(durable.clone()).merge(duet_api::producer_routes::router(
        ProducerState {
            durable: durable.clone(),
            context: producer.clone(),
        },
    ));
    for who in ["a", "b"] {
        ok(&app, "PUT", "/api/auth/account", who, json!({})).await;
        ok(
            &app,
            "PUT",
            "/api/auth/account/terms",
            who,
            json!({"version":"2026-09-23"}),
        )
        .await;
        ok(
            &app,
            "PUT",
            "/api/auth/account/luna-consent",
            who,
            json!({"decision":"accepted","version":"2026-09-24","expected_revision":0}),
        )
        .await;
    }
    let a = durable
        .authenticate(
            &[(
                axum::http::header::AUTHORIZATION,
                "Bearer a".parse().unwrap(),
            )]
            .into_iter()
            .collect(),
        )
        .await
        .unwrap();
    assert!(
        ok(&app, "GET", "/api/health-context", "a", json!(null))
            .await
            .is_null()
    );
    let start = Utc::now()
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();
    let record = Uuid::new_v4();
    ok(&app,"POST","/api/canonical-evidence","a",json!({"operation_id":Uuid::new_v4(),"canonical_record_id":record,"record_id":"phone-steps-yesterday","entity":"health_evidence","source_generation":0,"payload":{"id":record,"concept_id":"steps","display_name":"Steps","verification":"verified","method":"daily_aggregate","measured_interval":{"start":start-Duration::hours(12)},"source":{"kind":"appleHealth","connection_id":"phone","issuer":"watch","provenance":"native"},"value":{"type":"quantity","quantity":{"canonical_value":7654.0,"canonical_unit":"count"}},"context":{"aggregation":"daily_sum","day_timezone":"UTC","day_partial":false}}})).await;
    let capture = json!({"expected_revision":null,"snapshot":{"id":Uuid::new_v4(),"source_boundary_id":Uuid::new_v4(),"schema_version":1,"generated_at":Utc::now(),"timezone":"UTC","source_generations":{},"projections":[{"kind":"daily","window_start":start,"window_end":start+Duration::days(1),"timezone":"UTC","first_weekday":7,"facts":[]}]}});
    let context = ok(&app, "POST", "/api/health-context", "a", capture.clone()).await;
    assert_eq!(
        ok(&app, "POST", "/api/health-context", "a", capture).await,
        context
    );
    assert!(
        ok(&app, "GET", "/api/health-context", "b", json!(null))
            .await
            .is_null()
    );
    let stored = ok(&app, "GET", "/api/health-context", "a", json!(null)).await;
    let facts = stored["snapshot"]["projections"][0]["facts"]
        .as_array()
        .unwrap();
    let fact = facts
        .iter()
        .find(|f| f["kind"] == "current_value" && f["window"] == "day")
        .unwrap()
        .clone();
    assert_eq!(fact["value"], 7654.0);
    ok(&app,"POST","/api/auth/privacy-restrictions","a",json!({"event_id":Uuid::new_v4(),"operation":"device_withdraw","scope_id":a.account_id,"scope_version":0})).await;
    assert!(producer.read(&a).await.is_err());
    let refreshed = ok(&app, "POST", "/api/health-context/refresh", "a", json!({})).await;
    assert!(
        refreshed["snapshot"]["projections"][0]["facts"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let mut tx = db.begin(a.account_id).await.unwrap();
    assert!(
        sqlx::query("update follow_ups set state='active' where user_id=$1")
            .bind(a.account_id)
            .execute(&mut *tx)
            .await
            .is_err()
    );
    tx.rollback().await.unwrap();
    drop(app);
    drop(producer);
    drop(durable);
    drop(db);
    drop(write_db);
    sqlx::query(AssertSqlSafe(format!("drop database {name} with (force)")))
        .execute(&mut admin)
        .await
        .unwrap();
    for role in [reader, context_role, executor] {
        sqlx::query(AssertSqlSafe(format!("drop role {role}")))
            .execute(&mut admin)
            .await
            .unwrap();
    }
    std::fs::remove_dir_all(journal_path).unwrap();
}
