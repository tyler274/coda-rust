use axum::{
    Json, Router,
    http::{HeaderMap, StatusCode},
    routing::{get, post, put},
};
use duet_client::{
    DuetClient,
    identity::{Appearance, ChallengeCredential, IdentityError, PrivacyRestrictionKind},
};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Default)]
struct ServerState {
    email: String,
    appearance: Appearance,
    terms_accepted: bool,
    display_name: Option<String>,
    revoked_sessions: usize,
    device_signed_out: usize,
    deleted: usize,
}

#[tokio::test]
async fn client_email_auth_terms_settings_change_email_and_rights() {
    let state = Arc::new(Mutex::new(ServerState {
        email: "ada@example.test".into(),
        appearance: Appearance::System,
        ..Default::default()
    }));
    let st = state.clone();
    let app = Router::new()
        .route(
            "/api/auth/email/start",
            post(|Json(body): Json<Value>| async move {
                assert_eq!(body["email"], "ada@example.test");
                (StatusCode::ACCEPTED, Json(json!({ "message": "Check your email" })))
            }),
        )
        .route(
            "/api/auth/email/verify",
            post({
                let st = st.clone();
                move |Json(body): Json<Value>| {
                    let st = st.clone();
                    async move {
                        if body["credential"]["value"] == "dl1.used" {
                            return (
                                StatusCode::UNAUTHORIZED,
                                Json(json!({ "detail": { "code": "challenge_invalid", "message": "This link no longer works" } })),
                            )
                                .into_response();
                        }
                        let s = st.lock().unwrap();
                        (
                            StatusCode::OK,
                            Json(json!({
                                "bearer": "ds1.token",
                                "account_id": Uuid::nil(),
                                "incarnation": Uuid::nil(),
                                "email": s.email,
                                "appearance": s.appearance,
                            })),
                        )
                            .into_response()
                    }
                }
            }),
        )
        .route(
            "/api/auth/account/terms",
            put({
                let st = st.clone();
                move |Json(body): Json<Value>| {
                    let st = st.clone();
                    async move {
                        if body["version"] != "2026-09-23" {
                            return (
                                StatusCode::CONFLICT,
                                Json(json!({ "detail": { "code": "agreement_version_changed", "message": "Update Duet to review the current terms." } })),
                            )
                                .into_response();
                        }
                        st.lock().unwrap().terms_accepted = true;
                        (StatusCode::OK, Json(json!({ "terms_accepted": true, "terms_version": "2026-09-23" }))).into_response()
                    }
                }
            }),
        )
        .route(
            "/api/auth/account",
            put({
                let st = st.clone();
                move |Json(body): Json<Value>| {
                    let st = st.clone();
                    async move {
                        let mut s = st.lock().unwrap();
                        s.display_name = body["display_name"].as_str().map(str::to_owned);
                        Json(json!({ "display_name": s.display_name, "terms_accepted": s.terms_accepted }))
                    }
                }
            }),
        )
        .route(
            "/api/auth/account/appearance",
            put({
                let st = st.clone();
                move |Json(body): Json<Value>| {
                    let st = st.clone();
                    async move {
                        let app_pref: Appearance =
                            serde_json::from_value(body["appearance"].clone()).unwrap();
                        let mut s = st.lock().unwrap();
                        s.appearance = app_pref;
                        Json(json!({ "email": s.email, "appearance": s.appearance }))
                    }
                }
            }),
        )
        .route(
            "/api/auth/account/profile",
            get({
                let st = st.clone();
                move || {
                    let st = st.clone();
                    async move {
                        let s = st.lock().unwrap();
                        Json(json!({ "email": s.email, "appearance": s.appearance }))
                    }
                }
            }),
        )
        .route(
            "/api/auth/email-change/start",
            post(|Json(body): Json<Value>| async move {
                assert_eq!(body["new_email"], "ada.new@example.test");
                (StatusCode::ACCEPTED, Json(json!({ "message": "Check your email" })))
            }),
        )
        .route(
            "/api/auth/email-change/confirm",
            post({
                let st = st.clone();
                move |Json(body): Json<Value>| {
                    let st = st.clone();
                    async move {
                        assert_eq!(body["credential"]["value"], "654321");
                        let mut s = st.lock().unwrap();
                        s.email = "ada.new@example.test".into();
                        Json(json!({ "email": s.email, "appearance": s.appearance }))
                    }
                }
            }),
        )
        .route(
            "/api/auth/privacy-restrictions",
            post({
                let st = st.clone();
                move |headers: HeaderMap, Json(body): Json<Value>| {
                    let st = st.clone();
                    async move {
                        assert_eq!(headers["authorization"], "Bearer ds1.token");
                        let mut s = st.lock().unwrap();
                        match body["operation"].as_str().unwrap() {
                            "sessions_revoke" => s.revoked_sessions += 1,
                            "account_delete" => s.deleted += 1,
                            _ => {}
                        }
                        Json(json!({ "event_id": body["event_id"] }))
                    }
                }
            }),
        )
        .route(
            "/api/auth/session/sign-out",
            post({
                let st = st.clone();
                move || {
                    let st = st.clone();
                    async move {
                        st.lock().unwrap().device_signed_out += 1;
                        Json(json!({ "signed_out": true }))
                    }
                }
            }),
        );

    use axum::response::IntoResponse;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    let base = format!("http://{addr}");

    let unauth = DuetClient::new(&base, "", "1", "0.1.0").unwrap();
    assert_eq!(
        unauth
            .begin_email_sign_in("ada@example.test", Some("inv-1"))
            .await
            .unwrap(),
        "Check your email"
    );
    let err = unauth
        .complete_email_sign_in(None, &ChallengeCredential::LinkToken("dl1.used".into()))
        .await
        .err()
        .unwrap();
    assert_eq!(
        err,
        IdentityError::InvalidChallenge("This link no longer works".into())
    );
    let signed = unauth
        .complete_email_sign_in(None, &ChallengeCredential::LinkToken("dl1.good".into()))
        .await
        .unwrap();
    assert_eq!(signed.bearer, "ds1.token");

    let client = DuetClient::new(&base, &signed.bearer, "1", "0.1.0").unwrap();
    assert_eq!(
        client.accept_terms("1999-01-01").await.unwrap_err(),
        IdentityError::AgreementChanged
    );
    let prof = client.set_appearance(Appearance::Dark).await.unwrap();
    assert_eq!(prof.appearance, Appearance::Dark);
    let acct = client.update_display_name(Some("Ada")).await.unwrap();
    assert_eq!(acct.display_name, Some(Some("Ada".into())));

    assert_eq!(
        client
            .begin_email_change("ada.new@example.test")
            .await
            .unwrap(),
        "Check your email"
    );
    let prof = client
        .confirm_email_change(
            Some("ada.new@example.test"),
            &ChallengeCredential::ShortCode("654321".into()),
        )
        .await
        .unwrap();
    assert_eq!(prof.email.as_deref(), Some("ada.new@example.test"));

    let _ = client.accept_terms("2026-09-23").await.unwrap();
    let _ = client
        .commit_privacy_restriction(
            Uuid::new_v4(),
            PrivacyRestrictionKind::SessionsRevoke,
            None,
            0,
        )
        .await
        .unwrap();
    let _ = client
        .commit_privacy_restriction(
            Uuid::new_v4(),
            PrivacyRestrictionKind::AccountDelete,
            None,
            0,
        )
        .await
        .unwrap();
    client.sign_out_device().await.unwrap();

    let s = state.lock().unwrap();
    assert!(s.terms_accepted);
    assert_eq!(s.revoked_sessions, 1);
    assert_eq!(s.deleted, 1);
    assert_eq!(s.device_signed_out, 1);
}
