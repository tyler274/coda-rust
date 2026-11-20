//! Isolated PostgreSQL proof for email link/code auth, non-expiring sessions,
//! verified email change, appearance preferences, Terms enforcement, Rights-never-wait
//! actions, and retired web/doctor-preparation endpoints.
#![allow(
    clippy::disallowed_methods,
    reason = "provisions disposable databases and roles before running typed product owners"
)]

use async_trait::async_trait;
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use chrono::Utc;
use duet_api::{
    ApiError,
    database::{AccountDatabase, migrate},
    doctor_routes::{self, DoctorState},
    durable::{self, DurableState},
    email_auth::{Appearance, ChallengeCredential, EmailAuthStore, MemoryMailer},
    evidence::EvidenceStore,
    evidence_routes,
    identity::{IdentityStore, IdentityVerifier, VerifiedIdentity},
    privacy::{FileJournal, PrivacyStore, Restriction},
    publication::PublicationStore,
    workspace::WorkspaceStore,
};
use serde_json::{Value, json};
use sqlx::{AssertSqlSafe, ConnectOptions, Connection, PgConnection};
use std::sync::Arc;
use tower::ServiceExt;
use uuid::Uuid;

struct UnusedExternalVerifier;
#[async_trait]
impl IdentityVerifier for UnusedExternalVerifier {
    async fn verify(&self, _bearer: &str) -> Result<VerifiedIdentity, ApiError> {
        Err(ApiError::Unauthorized)
    }
}

async fn http(
    app: &Router,
    method: &str,
    path: &str,
    bearer: Option<&str>,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(path);
    if let Some(token) = bearer {
        builder = builder.header("authorization", format!("Bearer {token}"));
    }
    let body = match body {
        Some(val) => {
            builder = builder.header("content-type", "application/json");
            Body::from(val.to_string())
        }
        None => Body::empty(),
    };
    let response = app
        .clone()
        .oneshot(builder.body(body).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| json!({ "raw": String::from_utf8_lossy(&bytes) }))
    };
    (status, json)
}

#[tokio::test]
async fn email_auth_sessions_terms_rights_and_retired_routes() {
    let admin_url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("DUET_TEST_DATABASE_URL must point at disposable loopback PostgreSQL");
    let options: sqlx::postgres::PgConnectOptions = admin_url.parse().unwrap();
    assert!(matches!(
        options.get_host(),
        "localhost" | "127.0.0.1" | "::1"
    ));
    let mut admin = PgConnection::connect(&admin_url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let db_name = format!("duet_email_{suffix}");
    let role = format!("duet_email_role_{suffix}");
    let password = Uuid::new_v4().simple().to_string();
    sqlx::raw_sql(AssertSqlSafe(format!(
        "create role {role} login password '{password}'"
    )))
    .execute(&mut admin)
    .await
    .unwrap();
    sqlx::raw_sql(AssertSqlSafe(format!("create database {db_name}")))
        .execute(&mut admin)
        .await
        .unwrap();

    let owner_url = options
        .clone()
        .database(&db_name)
        .to_url_lossy()
        .to_string();
    let runtime_url = options
        .clone()
        .database(&db_name)
        .username(&role)
        .password(&password)
        .to_url_lossy()
        .to_string();
    migrate(&owner_url, &role).await.unwrap();

    let mailer = Arc::new(MemoryMailer::new());
    let journal_dir = std::env::temp_dir().join(format!("duet-email-journal-{suffix}"));
    let make_router = |mailer: Arc<MemoryMailer>| async {
        let db = AccountDatabase::connect(&runtime_url).await.unwrap();
        let email_auth = EmailAuthStore::new(db.clone(), mailer, "https://codahealth.ai");
        let state = DurableState {
            identity: IdentityStore::with_email_auth(
                db.clone(),
                Arc::new(UnusedExternalVerifier),
                email_auth,
            ),
            privacy: PrivacyStore::new(
                db.clone(),
                Arc::new(FileJournal::open(journal_dir.clone()).await.unwrap()),
            ),
            evidence: EvidenceStore::new(db.clone()),
        };
        let doctor = DoctorState {
            durable: state.clone(),
            workspace: WorkspaceStore::new(db.clone()),
            publications: PublicationStore::new(
                db,
                "https://codahealth.ai".into(),
                "https://viewer.codahealth.test".into(),
                [7u8; 32],
            )
            .unwrap(),
        };
        durable::router(state.clone())
            .merge(evidence_routes::router(state))
            .merge(doctor_routes::router(doctor))
    };

    let app = make_router(mailer.clone()).await;

    // 1. Retired Duet web surface and legacy /api/doctor-preparation return 404.
    for path in [
        "/",
        "/duet-app.js",
        "/duet-app_bg.wasm",
        "/api/doctor-preparation",
    ] {
        let (status, _) = http(&app, "GET", path, None, None).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{path} must be absent");
    }

    // 2. Single email sign-up flow issues link + short code; link works once and fails on reuse.
    let (status, body) = http(
        &app,
        "POST",
        "/api/auth/email/start",
        None,
        Some(json!({ "email": "Ada@Example.Test" })),
    )
    .await;
    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(body["message"], "Check your email");
    let first_msg = mailer.take().pop().unwrap();
    assert_eq!(first_msg.to, "ada@example.test");
    assert_eq!(first_msg.short_code.len(), 6);

    let (status, sign_in_1) = http(
        &app,
        "POST",
        "/api/auth/email/verify",
        None,
        Some(json!({
            "credential": ChallengeCredential::LinkToken(first_msg.link_token.clone())
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let bearer_1 = sign_in_1["bearer"].as_str().unwrap().to_owned();
    let account_id_1 = sign_in_1["account_id"].as_str().unwrap().to_owned();

    // Reusing the same magic link is refused with "This link no longer works".
    let (status, reuse) = http(
        &app,
        "POST",
        "/api/auth/email/verify",
        None,
        Some(json!({
            "credential": ChallengeCredential::LinkToken(first_msg.link_token)
        })),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(reuse["detail"]["message"], "This link no longer works");

    // 3. Second sign-in using short-code fallback resolves to the SAME account, and keeps first session alive.
    let _ = http(
        &app,
        "POST",
        "/api/auth/email/start",
        None,
        Some(json!({ "email": "ada@example.test" })),
    )
    .await;
    let second_msg = mailer.take().pop().unwrap();
    // Wrong code fails cleanly.
    let (status, wrong) = http(
        &app,
        "POST",
        "/api/auth/email/verify",
        None,
        Some(json!({
            "email": "ada@example.test",
            "credential": ChallengeCredential::ShortCode("000000".into())
        })),
    )
    .await;
    if second_msg.short_code != "000000" {
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(wrong["detail"]["message"], "This link no longer works");
    }
    let (status, sign_in_2) = http(
        &app,
        "POST",
        "/api/auth/email/verify",
        None,
        Some(json!({
            "email": "ada@example.test",
            "credential": ChallengeCredential::ShortCode(second_msg.short_code)
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let bearer_2 = sign_in_2["bearer"].as_str().unwrap().to_owned();
    assert_eq!(sign_in_2["account_id"], account_id_1);

    // 4. Restarting the backend keeps sessions valid (no time-based expiry).
    let restarted = make_router(mailer.clone()).await;
    for bearer in [&bearer_1, &bearer_2] {
        let (status, acct) = http(&restarted, "GET", "/api/auth/account", Some(bearer), None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(acct["terms_accepted"], false);
    }

    // 5. Before Terms acceptance: You and Shared routes are blocked (403 terms_acceptance_required).
    for (method, path, payload) in [
        ("GET", "/api/evidence/library", None),
        ("GET", "/api/doctor-collection", None),
        (
            "PUT",
            "/api/auth/account/today-stats",
            Some(json!({ "stat_ids": ["metric:steps"] })),
        ),
    ] {
        let (status, err) = http(&restarted, method, path, Some(&bearer_1), payload).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{path} must require terms");
        assert_eq!(err["detail"]["code"], "terms_acceptance_required");
    }

    // 6. Rights-never-wait while Terms wait: display name, appearance, verified email change, source withdrawal.
    let (status, prof) = http(
        &restarted,
        "PUT",
        "/api/auth/account/appearance",
        Some(&bearer_1),
        Some(json!({ "appearance": Appearance::Dark })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(prof["appearance"], "dark");

    let (status, acct) = http(
        &restarted,
        "PUT",
        "/api/auth/account",
        Some(&bearer_1),
        Some(json!({ "display_name": "Ada Lovelace" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(acct["display_name"], "Ada Lovelace");

    // Verified email change works before Terms acceptance and updates verified email cleanly.
    let (status, _) = http(
        &restarted,
        "POST",
        "/api/auth/email-change/start",
        Some(&bearer_1),
        Some(json!({ "new_email": "ada.new@example.test" })),
    )
    .await;
    assert_eq!(status, StatusCode::ACCEPTED);
    let change_msg = mailer.take().pop().unwrap();
    let (status, updated_prof) = http(
        &restarted,
        "POST",
        "/api/auth/email-change/confirm",
        Some(&bearer_1),
        Some(json!({
            "new_email": "ada.new@example.test",
            "credential": ChallengeCredential::ShortCode(change_msg.short_code)
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated_prof["email"], "ada.new@example.test");

    // Device withdrawal stays accessible while Terms wait.
    let (status, receipt) = http(
        &restarted,
        "POST",
        "/api/auth/privacy-restrictions",
        Some(&bearer_1),
        Some(json!({
            "event_id": Uuid::new_v4(),
            "operation": Restriction::DeviceWithdraw,
            "scope_version": 0
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{receipt}");

    // 7. Stale Terms version yields 409 agreement_version_changed; current version unlocks You & Shared.
    let (status, stale) = http(
        &restarted,
        "PUT",
        "/api/auth/account/terms",
        Some(&bearer_1),
        Some(json!({ "version": "1999-01-01" })),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(stale["detail"]["code"], "agreement_version_changed");

    let (status, accepted) = http(
        &restarted,
        "PUT",
        "/api/auth/account/terms",
        Some(&bearer_1),
        Some(json!({ "version": "2026-09-23" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(accepted["terms_accepted"], true);

    let (status, _) = http(
        &restarted,
        "GET",
        "/api/evidence/library",
        Some(&bearer_1),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (status, _) = http(
        &restarted,
        "GET",
        "/api/doctor-collection",
        Some(&bearer_1),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    // 8. Device sign-out ends bearer_1 only; bearer_2 stays alive until SessionsRevoke (sign-out everywhere).
    let (status, _) = http(
        &restarted,
        "POST",
        "/api/auth/session/sign-out",
        Some(&bearer_1),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (status, _) = http(
        &restarted,
        "GET",
        "/api/auth/account",
        Some(&bearer_1),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let (status, _) = http(
        &restarted,
        "GET",
        "/api/auth/account",
        Some(&bearer_2),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    // Sign out everywhere revokes bearer_2 across restarts.
    let (status, _) = http(
        &restarted,
        "POST",
        "/api/auth/privacy-restrictions",
        Some(&bearer_2),
        Some(json!({
            "event_id": Uuid::new_v4(),
            "operation": Restriction::SessionsRevoke,
            "scope_version": 0
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (status, _) = http(
        &restarted,
        "GET",
        "/api/auth/account",
        Some(&bearer_2),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // 9. Sign back in, delete account, and re-sign-up with the SAME email -> fresh empty account!
    let _ = http(
        &restarted,
        "POST",
        "/api/auth/email/start",
        None,
        Some(json!({ "email": "ada.new@example.test" })),
    )
    .await;
    let third_msg = mailer.take().pop().unwrap();
    let (_, sign_in_3) = http(
        &restarted,
        "POST",
        "/api/auth/email/verify",
        None,
        Some(json!({
            "credential": ChallengeCredential::LinkToken(third_msg.link_token)
        })),
    )
    .await;
    let bearer_3 = sign_in_3["bearer"].as_str().unwrap().to_owned();
    let (status, _) = http(
        &restarted,
        "POST",
        "/api/auth/privacy-restrictions",
        Some(&bearer_3),
        Some(json!({
            "event_id": Uuid::new_v4(),
            "operation": Restriction::AccountDelete,
            "scope_version": 0
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let (status, _) = http(
        &restarted,
        "GET",
        "/api/auth/account",
        Some(&bearer_3),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let _ = http(
        &restarted,
        "POST",
        "/api/auth/email/start",
        None,
        Some(json!({ "email": "ada.new@example.test" })),
    )
    .await;
    let fourth_msg = mailer.take().pop().unwrap();
    let (status, recreated) = http(
        &restarted,
        "POST",
        "/api/auth/email/verify",
        None,
        Some(json!({
            "credential": ChallengeCredential::LinkToken(fourth_msg.link_token)
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_ne!(
        recreated["account_id"].as_str().unwrap(),
        account_id_1,
        "deleted email must receive a new empty account on later sign-up"
    );
    let _ = Utc::now();

    drop(app);
    drop(restarted);
    let _ = sqlx::raw_sql(AssertSqlSafe(format!(
        "drop database if exists {db_name} with (force)"
    )))
    .execute(&mut admin)
    .await;
    let _ = sqlx::raw_sql(AssertSqlSafe(format!("drop role if exists {role}")))
        .execute(&mut admin)
        .await;
    let _ = std::fs::remove_dir_all(&journal_dir);
}
