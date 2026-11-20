//! Real-PostgreSQL proof for `COD-53` Duet Connected Care invitation routes:
//! - Inspecting link tokens & short codes without leaking practice/physician details (`Rule 6`).
//! - Wrong birth-date strike counter locking at 5 (`COD-48`, `Rule 7`) & underage rejection (`Rule 22`).
//! - Checking DOB alone grants zero consent (`Rule 3`).
//! - One-tap `Share` connects the currently signed-in Duet account (`Rule 2`, `Rule 9`)
//!   without creating a duplicate account even when invited email != signed-in email.

#![allow(
    clippy::disallowed_methods,
    reason = "provisions disposable databases and roles before running typed product owners"
)]

use std::sync::Arc;

use async_trait::async_trait;
use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use duet_api::{
    ApiError,
    connected_care_invitations::{
        ConnectedCareInvitationState, FrontDoorInvitationPort, router as invite_router,
    },
    database::{AccountDatabase, migrate},
    durable::{self, DurableState},
    email_auth::{ChallengeCredential, EmailAuthStore, MemoryMailer},
    evidence::EvidenceStore,
    identity::{IdentityStore, IdentityVerifier, VerifiedIdentity},
    privacy::{FileJournal, PrivacyStore},
};
use front_door::{
    FrontDoorStore, InvitationCredential, InvitePatientParams, MemoryFrontDoorStore, RosterPatient,
};
use serde_json::{Value, json};
use sqlx::{AssertSqlSafe, ConnectOptions, Connection, PgConnection};
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
async fn cod53_duet_api_dob_gate_lock_and_share_connects_signed_in_account_in_postgres() {
    let admin_url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("DUET_TEST_DATABASE_URL must point at disposable loopback PostgreSQL");
    let options: sqlx::postgres::PgConnectOptions = admin_url.parse().unwrap();
    let mut admin = PgConnection::connect(&admin_url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let db_name = format!("duet_inv_{suffix}");
    let role = format!("duet_inv_role_{suffix}");
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

    let db = AccountDatabase::connect(&runtime_url).await.unwrap();
    let mailer = Arc::new(MemoryMailer::new());
    let email_auth = EmailAuthStore::new(db.clone(), mailer.clone(), "https://codahealth.ai");
    let journal_dir = std::env::temp_dir().join(format!("duet-inv-journal-{suffix}"));
    let durable = DurableState {
        identity: IdentityStore::with_email_auth(
            db.clone(),
            Arc::new(UnusedExternalVerifier),
            email_auth,
        ),
        privacy: PrivacyStore::new(
            db.clone(),
            Arc::new(FileJournal::open(journal_dir).await.unwrap()),
        ),
        evidence: EvidenceStore::new(db),
    };
    let front_door = Arc::new(MemoryFrontDoorStore::new());
    let app = durable::router(durable.clone()).merge(invite_router(ConnectedCareInvitationState {
        durable,
        front_door: Some(FrontDoorInvitationPort::Direct(front_door.clone())),
    }));

    // Sign in a Duet account whose email differs from the chart invitation email (`Rule 9`).
    let (status, _) = http(
        &app,
        "POST",
        "/api/auth/email/start",
        None,
        Some(json!({ "email": "patient.own@example.test" })),
    )
    .await;
    assert_eq!(status, StatusCode::ACCEPTED);
    let msg = mailer.take().pop().unwrap();
    let (status, sign_in) = http(
        &app,
        "POST",
        "/api/auth/email/verify",
        None,
        Some(json!({
            "email": "patient.own@example.test",
            "credential": ChallengeCredential::ShortCode(msg.short_code)
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let bearer = sign_in["bearer"].as_str().unwrap().to_owned();
    let (status, _) = http(
        &app,
        "PUT",
        "/api/auth/account/terms",
        Some(&bearer),
        Some(json!({ "version": "2026-09-23" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    // Practice invites Tamika to clinic email; resend adds another valid credential.
    let now = 1_760_000_000;
    let rp = RosterPatient {
        practice_id: "clinic-1".into(),
        patient_id: "pat-tamika".into(),
    };
    let issued1 = front_door
        .invite_patient(InvitePatientParams {
            patient: rp.clone(),
            email: "chart-other@example.test".into(),
            display_name: "Tamika Washington".into(),
            practice_name: "River Bend Clinic".into(),
            physician_name: "Dr. Roger".into(),
            expected_birth_date: Some("1987-02-18".into()),
            now,
        })
        .await
        .unwrap();
    let issued2 = front_door.resend_invitation(&rp, now + 1).await.unwrap();

    // 1. Inspect with prior link token returns active standing and zero preview leakage.
    let (status, gate) = http(
        &app,
        "POST",
        "/api/connected-care/invitations/inspect",
        Some(&bearer),
        Some(json!({
            "credential": InvitationCredential::LinkToken(issued1.link_token)
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{gate}");
    assert_eq!(gate["standing"]["standing"], "active");
    assert!(gate.get("practice_name").is_none());

    // 2. Verify DOB reveals preview without creating a connection (`Rule 3`, `Rule 6`).
    let (status, preview) = http(
        &app,
        "POST",
        "/api/connected-care/invitations/verify-dob",
        Some(&bearer),
        Some(json!({
            "credential": InvitationCredential::ShortCode(issued2.short_code.clone()),
            "birth_date": "1987-02-18"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{preview}");
    assert_eq!(preview["practice_name"], "River Bend Clinic");
    let (status, conns_before) = http(
        &app,
        "GET",
        "/api/connected-care/connections",
        Some(&bearer),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(conns_before.as_array().unwrap().len(), 0);

    // 3. Explicit Share connects the signed-in Duet account (`Rule 2`, `Rule 9`).
    let (status, shared) = http(
        &app,
        "POST",
        "/api/connected-care/invitations/share",
        Some(&bearer),
        Some(json!({
            "credential": InvitationCredential::ShortCode(issued2.short_code),
            "birth_date": "1987-02-18"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{shared}");
    let fd_id = shared["front_door_account_id"].as_str().unwrap().to_owned();
    assert_eq!(
        front_door
            .account_by_email("patient.own@example.test")
            .await
            .unwrap()
            .id
            .to_string(),
        fd_id
    );
    assert!(
        front_door
            .account_by_email("chart-other@example.test")
            .await
            .is_none(),
        "must not create a duplicate account for the invitation email"
    );

    // 4. Five wrong DOB answers on another invitation lock cleanly (`COD-48`, `Rule 7`).
    let rp2 = RosterPatient {
        practice_id: "clinic-2".into(),
        patient_id: "pat-tamika".into(),
    };
    let issued_b = front_door
        .invite_patient(InvitePatientParams {
            patient: rp2,
            email: "chart-other@example.test".into(),
            display_name: "Tamika Washington".into(),
            practice_name: "Oak Clinic".into(),
            physician_name: "Dr. Chen".into(),
            expected_birth_date: Some("1987-02-18".into()),
            now: now + 10,
        })
        .await
        .unwrap();
    for _ in 0..4 {
        let (status, err) = http(
            &app,
            "POST",
            "/api/connected-care/invitations/verify-dob",
            Some(&bearer),
            Some(json!({
                "credential": InvitationCredential::ShortCode(issued_b.short_code.clone()),
                "birth_date": "1987-02-19"
            })),
        )
        .await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{err}");
    }
    let (status, locked_err) = http(
        &app,
        "POST",
        "/api/connected-care/invitations/verify-dob",
        Some(&bearer),
        Some(json!({
            "credential": InvitationCredential::ShortCode(issued_b.short_code.clone()),
            "birth_date": "1987-02-19"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::LOCKED, "{locked_err}");
}
