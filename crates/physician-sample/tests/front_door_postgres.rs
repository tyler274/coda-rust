//! The practice invitation and front-door read against Postgres.
//!
//! Gated on `PHYSICIAN_DATABASE_URL`, the same gate the auth routes use in
//! a deployment: when it is set, the practice server builds both stores on
//! that database and this suite proves the invite → redeem → link →
//! premium journey through them. Without the gate the memory-store unit
//! tests carry the contract and these stay quiet.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use front_door::{CADENCE_WEEKLY, CADENCE_WEEKLY_DAILY_CHECK, FrontDoorStore, RosterPatient};
use physician_sample::{STAGING_PRACTICE_ID, WORKSPACE_IDS};
use sea_orm::ConnectionTrait;
use serde_json::Value;
use tower::ServiceExt;

const NOW: u64 = 1_760_000_000;

/// Builds the practice's authed router on one Postgres database, returning
/// the router, the auth store's email lookup, and the front-door store.
///
/// Each test gets its own database — named after the gate URL's database
/// plus the test's unique suffix — so parallel tests and repeated runs
/// never share migration state. The databases are dropped on a later run's
/// `DROP DATABASE IF EXISTS`, so a crashed test never wedges the suite.
async fn router(
    name: &str,
) -> Option<(
    axum::Router,
    std::sync::Arc<dyn physician_auth::AuthStore>,
    std::sync::Arc<dyn FrontDoorStore>,
)> {
    let url = std::env::var("PHYSICIAN_DATABASE_URL").ok()?;
    let url = url.trim();
    if url.is_empty() {
        return None;
    }
    let mut split: Vec<_> = url.rsplitn(2, '/').collect();
    let db = split.remove(0);
    let base = split.remove(0);
    let test_db = format!("{db}_t_fd_{name}");
    let admin = sea_orm::Database::connect(&format!("{base}/postgres"))
        .await
        .expect("admin connection");
    admin
        .execute_unprepared(&format!("DROP DATABASE IF EXISTS {test_db}"))
        .await
        .expect("drop stale test database");
    admin
        .execute_unprepared(&format!("CREATE DATABASE {test_db}"))
        .await
        .expect("create test database");
    let url = format!("{base}/{test_db}");
    let auth: std::sync::Arc<dyn physician_auth::AuthStore> = std::sync::Arc::new(
        physician_auth::PostgresAuthStore::connect_with_migrations(&url)
            .await
            .expect("auth store"),
    );
    let front_door: std::sync::Arc<dyn FrontDoorStore> = std::sync::Arc::new(
        front_door::PostgresFrontDoorStore::connect_with_migrations(&url)
            .await
            .expect("front-door store"),
    );
    let router = physician_sample::authed_router_on(
        physician_sample::recorded(),
        std::sync::Arc::clone(&auth),
        std::sync::Arc::clone(&front_door),
    )
    .await;
    admin.close().await.expect("close admin connection");
    Some((router, auth, front_door))
}

/// Signs the staging physician in over HTTP, returning the bearer session.
async fn session_for(app: &axum::Router, email: &str) -> String {
    let (status, body) = send(
        app,
        "POST",
        "/auth/sign-in",
        None,
        Some(serde_json::json!({ "email": email }).to_string()),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "sign-in: {body}");
    let token = serde_json::from_str::<Value>(&body).expect("sign-in body")["token"]
        .as_str()
        .expect("issued token")
        .to_owned();
    let (status, body) = send(
        app,
        "POST",
        "/auth/verify",
        None,
        Some(serde_json::json!({ "email": email, "token": token }).to_string()),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "verify: {body}");
    serde_json::from_str::<Value>(&body).expect("verify body")["session"]
        .as_str()
        .expect("bearer session")
        .to_owned()
}

async fn send(
    app: &axum::Router,
    method: &str,
    path: &str,
    token: Option<&str>,
    body: Option<String>,
) -> (StatusCode, String) {
    let mut builder = Request::builder().method(method).uri(path);
    if let Some(token) = token {
        builder = builder.header(axum::http::header::AUTHORIZATION, format!("Bearer {token}"));
    }
    if body.is_some() {
        builder = builder.header(axum::http::header::CONTENT_TYPE, "application/json");
    }
    let request = builder
        .body(Body::from(body.unwrap_or_default()))
        .expect("request");
    let response = app.clone().oneshot(request).await.expect("response");
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), 2_000_000)
        .await
        .unwrap();
    (status, String::from_utf8(bytes.to_vec()).unwrap())
}

#[tokio::test]
async fn the_practice_invite_redeems_once_and_reads_premium_on_postgres() {
    let Some((app, auth, front_door)) = router("invite").await else {
        return;
    };
    let account = auth
        .account_by_email("physician@codahealth.test")
        .await
        .expect("staging account");
    assert!(account.admin, "the staging physician administers");
    let session = session_for(&app, "physician@codahealth.test").await;
    let workspace = WORKSPACE_IDS[0];
    let (status, body) = send(
        &app,
        "POST",
        "/auth/invitations",
        Some(&session),
        Some(
            serde_json::json!({
                "patient_workspace_id": workspace,
                "email": format!("{}@pg-proof.test", uuid::Uuid::new_v4().simple()),
                "display_name": "PG Proof",
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let invited: Value = serde_json::from_str(&body).expect("invitation body");
    let token = invited["token"].as_str().expect("invite token").to_owned();
    let email = invited["email"].as_str().unwrap().to_owned();

    // The pending invitation names the staging practice's roster patient.
    let pending = front_door
        .invitation(&email)
        .await
        .expect("pending invitation");
    assert_eq!(
        pending.patient,
        RosterPatient {
            practice_id: STAGING_PRACTICE_ID.to_owned(),
            patient_id: workspace.to_owned(),
        }
    );

    // The patient redeems once; the account, link, and session are minted.
    let (account_id, redeem_session) = front_door
        .redeem_invitation(&email, &token, NOW)
        .await
        .expect("first redeem");
    assert_eq!(
        front_door.session(&redeem_session, NOW).await.unwrap(),
        account_id
    );
    assert_eq!(
        front_door
            .redeem_invitation(&email, &token, NOW)
            .await
            .unwrap_err(),
        front_door::SignInError::Invalid,
        "an invitation redeems once"
    );
    assert_eq!(
        front_door.link(account_id).await.unwrap().patient_id,
        workspace
    );

    // The practice reads the linked account and its premium status, which
    // starts free and follows the setting without storing billing.
    let (status, body) = send(
        &app,
        "GET",
        &format!("/auth/front-door/{workspace}"),
        Some(&session),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let read: Value = serde_json::from_str(&body).expect("front-door read");
    assert_eq!(read["account_id"], account_id.to_string());
    assert_eq!(read["email"], email);
    assert_eq!(read["premium"], false);
    assert_eq!(read["cadence"], CADENCE_WEEKLY);
    front_door
        .set_premium(
            &RosterPatient {
                practice_id: STAGING_PRACTICE_ID.to_owned(),
                patient_id: workspace.to_owned(),
            },
            true,
            NOW,
        )
        .await
        .unwrap();
    let (_, body) = send(
        &app,
        "GET",
        &format!("/auth/front-door/{workspace}"),
        Some(&session),
        None,
    )
    .await;
    let read: Value = serde_json::from_str(&body).expect("front-door read");
    assert_eq!(read["premium"], true);
    assert_eq!(read["cadence"], CADENCE_WEEKLY_DAILY_CHECK);
}

#[tokio::test]
async fn an_existing_account_signs_in_again_without_redeeming() {
    let Some((_app, _auth, front_door)) = router("sign_in").await else {
        return;
    };
    let workspace = WORKSPACE_IDS[1];
    let patient = RosterPatient {
        practice_id: STAGING_PRACTICE_ID.to_owned(),
        patient_id: workspace.to_owned(),
    };
    let token = front_door
        .invite(&patient, "returning@pg-proof.test", "Returning", NOW)
        .await
        .expect("invitation");
    let (account_id, _) = front_door
        .redeem_invitation("returning@pg-proof.test", &token, NOW)
        .await
        .expect("redeem");
    // The redeemed account signs in with a fresh token, never a second
    // redeem: the front door's email identity survives the first visit.
    let sign_in = front_door
        .begin_sign_in("returning@pg-proof.test", NOW)
        .await
        .expect("sign-in token");
    let (who, session) = front_door
        .complete_sign_in("returning@pg-proof.test", &sign_in, NOW)
        .await
        .expect("verify");
    assert_eq!(who, account_id);
    assert_eq!(front_door.session(&session, NOW).await.unwrap(), account_id);
    // The practice reads the returning patient's link through the same
    // front-door store the server holds.
    let linked = front_door
        .linked_accounts(STAGING_PRACTICE_ID)
        .await
        .into_iter()
        .any(|(account, linked)| account.id == account_id && linked == patient);
    assert!(linked, "the returning account stays linked");
}
