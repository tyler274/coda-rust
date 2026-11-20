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

#[tokio::test]
async fn client_export_and_sync_pending_before_sign_out() {
    use duet_client::doctor::{
        PendingDoctorOperation, SignOutSyncOutcome, sync_pending_before_sign_out,
    };
    use duet_client::retention::{DoctorDraft, ProtectedStore, RetainedIdentity, RetentionOwner};
    use duet_wire::models::{DoctorCollection, DoctorPageSave};
    use std::collections::HashMap;

    #[derive(Default)]
    struct TestStore(Mutex<HashMap<String, Vec<u8>>>);
    impl ProtectedStore for TestStore {
        fn read(&self, key: &str) -> Result<Option<Vec<u8>>, String> {
            Ok(self.0.lock().unwrap().get(key).cloned())
        }
        fn write(&self, key: &str, bytes: &[u8]) -> Result<(), String> {
            self.0.lock().unwrap().insert(key.into(), bytes.into());
            Ok(())
        }
        fn remove(&self, key: &str) -> Result<(), String> {
            self.0.lock().unwrap().remove(key);
            Ok(())
        }
    }

    let account_id = Uuid::new_v4();
    let export_payload = json!({
        "schema_version": 1,
        "exported_at": "2026-11-19T12:00:00Z",
        "account": {
            "account_id": account_id,
            "email": "ada@example.test",
            "display_name": "Ada",
            "appearance": "system",
            "sharing_label": "Ada",
            "terms_version": null,
            "terms_accepted_at": null,
            "ai_enabled": false,
            "ai_consent_version": null,
            "ai_consent_decision": null,
            "ai_consent_at": null,
            "apple_health_generation": 0,
            "apple_health_disconnected_at": null,
            "health_records_generation": 0,
            "health_records_disconnected_at": null,
            "health_connect_generation": 0,
            "health_connect_disconnected_at": null,
            "created_at": "2026-11-19T12:00:00Z"
        },
        "health_evidence": [],
        "clinical_notes": [],
        "device_snapshots": [],
        "doctor_page": {
            "note": "Check blood pressure",
            "note_generation": 1,
            "page": {},
            "page_updated_at": "2026-11-19T12:00:00Z",
            "items": []
        },
        "portal_connections": [],
        "conversations": [],
        "private_memory": null,
        "privacy_receipts": []
    });

    let cleared = Arc::new(Mutex::new(false));
    let cleared_flag = cleared.clone();
    let mut base_collection = DoctorCollection::new(2);
    base_collection.page = Some(Box::default());
    base_collection.items = Some(Vec::new());
    let mut saved_collection = DoctorCollection::new(3);
    saved_collection.page = Some(Box::default());
    saved_collection.items = Some(Vec::new());
    saved_collection.note = Some(Some("Updated note".into()));
    let get_coll = base_collection.clone();
    let put_coll = saved_collection.clone();

    let app = Router::new()
        .route(
            "/api/auth/account/export",
            get(move || {
                let payload = export_payload.clone();
                async move { Json(payload) }
            }),
        )
        .route(
            "/api/doctor-collection",
            get(move || {
                let c = get_coll.clone();
                async move { Json(c) }
            })
            .delete(move || {
                *cleared_flag.lock().unwrap() = true;
                async move { StatusCode::NO_CONTENT }
            }),
        )
        .route(
            "/api/doctor-collection/page",
            put(move || {
                let c = put_coll.clone();
                async move { Json(c) }
            }),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

    let client = DuetClient::new(format!("http://{addr}"), "ds1.token", "0", "0.1.0").unwrap();
    let exported = client.export_account_data().await.unwrap();
    assert_eq!(exported.account.email.as_deref(), Some("ada@example.test"));
    assert_eq!(
        exported.doctor_page.note.as_deref(),
        Some("Check blood pressure")
    );
    assert!(exported.summary_text().contains("Export ready:"));

    let store = Arc::new(TestStore::default());
    let retention = RetentionOwner::new(store);
    let fence = retention
        .activate(RetainedIdentity {
            owner: format!("{account_id}:inc-1"),
            subject: "email:ada".into(),
            session_token: "ds1.token".into(),
            email: Some("ada@example.test".into()),
        })
        .unwrap();

    retention
        .save(
            &fence,
            DoctorDraft {
                base: base_collection.clone(),
                save: DoctorPageSave {
                    expected_generation: 2,
                    page: Box::default(),
                    note: Some(Some("Updated note".into())),
                    item_ids: Vec::new(),
                    proposal_id: None,
                    confirmed_medication_ids: None,
                },
                uncertain_save: false,
            },
        )
        .unwrap();
    retention
        .retain_operation(
            &fence,
            &PendingDoctorOperation::Clear {
                operation_id: Uuid::new_v4(),
                expected_generation: 3,
            },
        )
        .unwrap();

    // Unreachable client keeps exact count unconfirmed without losing local work.
    let offline = DuetClient::new("http://127.0.0.1:9", "ds1.token", "0", "0.1.0").unwrap();
    let offline_outcome = sync_pending_before_sign_out(&offline, &retention, &fence, 1000)
        .await
        .unwrap();
    match offline_outcome {
        SignOutSyncOutcome::Unconfirmed(leaving) => {
            assert_eq!(leaving.unconfirmed_count(), 2);
            assert!(leaving.warning().contains("2 unconfirmed entries"));
            assert!(leaving.warning().contains("removes them from this phone"));
        }
        SignOutSyncOutcome::AllConfirmed => panic!("expected unconfirmed entries when offline"),
    }

    // Connected client flushes draft & operation cleanly.
    let synced = sync_pending_before_sign_out(&client, &retention, &fence, 2000)
        .await
        .unwrap();
    assert_eq!(synced, SignOutSyncOutcome::AllConfirmed);
    assert!(*cleared.lock().unwrap());
    assert!(retention.account_leaving().unwrap().is_empty());
}
