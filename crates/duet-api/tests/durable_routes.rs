//! Controlled verifier + real canonical Postgres owner. No provider/cloud calls.
use async_trait::async_trait;
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use duet_api::{
    durable::{self, IdentityProvider},
    identity::IdentityError,
};
use front_door::{
    PostgresFrontDoorStore,
    identity::{LUNA_VERSION, Principal, TERMS_VERSION},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tower::ServiceExt;

struct Verified {
    alice: Principal,
    bob: Principal,
    disabled: AtomicBool,
    revoked: AtomicBool,
}
#[async_trait]
impl IdentityProvider for Verified {
    async fn verify(&self, token: &str) -> Result<Principal, IdentityError> {
        match token {
            "alice"
                if !self.disabled.load(Ordering::SeqCst)
                    && !self.revoked.load(Ordering::SeqCst) =>
            {
                Ok(self.alice.clone())
            }
            "bob" => Ok(self.bob.clone()),
            "foreign" => {
                let mut p = self.alice.clone();
                p.project = "foreign-project".into();
                Ok(p)
            }
            "collision" => {
                let mut p = self.alice.clone();
                p.subject = "separate-identity".into();
                Ok(p)
            }
            "offline" => Err(IdentityError::Unavailable),
            // Even a buggy injected verifier cannot enable dev-token fallback.
            t if t.starts_with("dev:") => Ok(self.alice.clone()),
            _ => Err(IdentityError::Invalid),
        }
    }
}
async fn call(
    router: &Router,
    method: &str,
    path: &str,
    token: &str,
    body: Value,
) -> (StatusCode, Value) {
    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .header("authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    (status, serde_json::from_slice(&body).unwrap_or(Value::Null))
}

#[tokio::test]
async fn canonical_route_isolation_revocation_readback_and_unsupported_effects() {
    let url = match std::env::var("DUET_IDENTITY_TEST_DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            assert!(
                std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
                "DUET_IDENTITY_TEST_DATABASE_URL is required"
            );
            return;
        }
    };
    let store = Arc::new(PostgresFrontDoorStore::connect(&url).await.unwrap());
    store.migrate().await.unwrap();
    let now = chrono::Utc::now().timestamp();
    let suffix = uuid::Uuid::new_v4();
    let principal = |name: &str| Principal {
        project: "duet-test".into(),
        subject: format!("{name}-{suffix}"),
        email: format!("{name}-{suffix}@example.test"),
        authenticated_at: now - 10,
    };
    let verifier = Arc::new(Verified {
        alice: principal("alice"),
        bob: principal("bob"),
        disabled: AtomicBool::new(false),
        revoked: AtomicBool::new(false),
    });
    let router = durable::router(store.clone(), verifier.clone());
    assert_eq!(
        call(&router, "GET", "/api/auth/account", "alice", json!({}))
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert!(
        store.identity_access(&verifier.alice).await.is_err(),
        "reads never provision"
    );
    for token in ["dev:alice", "invalid", "", "alice extra"] {
        assert_eq!(
            call(&router, "PUT", "/api/auth/account", token, json!({}))
                .await
                .0,
            StatusCode::UNAUTHORIZED
        );
    }
    assert_eq!(
        call(&router, "GET", "/api/auth/account", "offline", json!({}))
            .await
            .0,
        StatusCode::SERVICE_UNAVAILABLE
    );
    for (token, name) in [("alice", "Alice"), ("bob", "Bob")] {
        let (status, view) = call(
            &router,
            "PUT",
            "/api/auth/account",
            token,
            json!({"display_name":name}),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(view["display_name"], name);
        assert_eq!(view["intelligence_available"], false);
    }
    let alice = store
        .identity_access(&verifier.alice)
        .await
        .unwrap()
        .account_id;
    let bob = store
        .identity_access(&verifier.bob)
        .await
        .unwrap()
        .account_id;
    assert_ne!(alice, bob);
    assert_eq!(
        call(&router, "GET", "/api/auth/account", "foreign", json!({}))
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        call(&router, "PUT", "/api/auth/account", "collision", json!({}))
            .await
            .0,
        StatusCode::SERVICE_UNAVAILABLE,
        "same email must not merge identities"
    );
    assert_eq!(
        call(&router, "GET", "/api/auth/account", "collision", json!({}))
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/auth/account")
                .header("authorization", "Bearer alice")
                .header("authorization", "Bearer bob")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        call(
            &router,
            "PUT",
            "/api/auth/account",
            "alice",
            json!({"account_id":bob,"display_name":"stolen"})
        )
        .await
        .0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(
        call(
            &router,
            "PUT",
            "/api/auth/account/terms",
            "alice",
            json!({"version":TERMS_VERSION,"account_id":bob})
        )
        .await
        .0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(
        call(
            &router,
            "PUT",
            "/api/auth/account/terms",
            "alice",
            json!({"version":"wrong"})
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    let (_, view) = call(
        &router,
        "PUT",
        "/api/auth/account/terms",
        "alice",
        json!({"version":TERMS_VERSION}),
    )
    .await;
    assert_eq!(view["terms_accepted"], true);
    assert_eq!(
        call(&router, "GET", "/api/auth/account", "bob", json!({}))
            .await
            .1["terms_accepted"],
        false
    );
    assert_eq!(
        call(&router, "GET", "/api/auth/account", "bob", json!({}))
            .await
            .1["display_name"],
        "Bob"
    );
    // Permission owner changes are observed on the next request. No route can
    // run a model, clear restriction_pending, or claim privacy completion.
    store
        .set_identity_luna(&verifier.alice, LUNA_VERSION, true, now)
        .await
        .unwrap();
    assert_eq!(
        call(&router, "GET", "/api/auth/account", "alice", json!({}))
            .await
            .1["luna_consent"],
        "accepted"
    );
    store
        .set_identity_luna(&verifier.alice, LUNA_VERSION, false, now)
        .await
        .unwrap();
    let (_, view) = call(&router, "GET", "/api/auth/account", "alice", json!({})).await;
    assert_eq!(view["luna_consent"], "declined");
    assert_eq!(view["intelligence_enabled"], false);
    for (method, path) in [
        ("GET", "/api/moments"),
        ("DELETE", "/api/moments/foreign"),
        ("PUT", "/api/intelligence/consent"),
        ("POST", "/api/join"),
        ("GET", "/api/consent"),
        ("GET", "/front-door/account"),
        ("POST", "/front-door/sign-in"),
        ("GET", "/api/doctor-plan"),
        ("POST", "/api/physician-sharing/records"),
        ("POST", "/api/conversation/messages"),
        ("PUT", "/api/auth/account/today-stats"),
        ("GET", "/api/export"),
        ("DELETE", "/api/auth/account"),
    ] {
        let status = call(&router, method, path, "alice", json!({"account_id":bob}))
            .await
            .0;
        assert!(
            status == StatusCode::NOT_FOUND || status == StatusCode::METHOD_NOT_ALLOWED,
            "{method} {path}: {status}"
        );
    }
    assert_eq!(
        call(
            &router,
            "GET",
            &format!("/api/auth/account/{bob}"),
            "alice",
            json!({})
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    for flag in [&verifier.disabled, &verifier.revoked] {
        flag.store(true, Ordering::SeqCst);
        for method in ["GET", "PUT"] {
            assert_eq!(
                call(&router, method, "/api/auth/account", "alice", json!({}))
                    .await
                    .0,
                StatusCode::UNAUTHORIZED
            );
        }
        flag.store(false, Ordering::SeqCst);
    }
    let (status, body) = call(
        &router,
        "POST",
        "/api/auth/session/revoke",
        "alice",
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(
        body["status"], "pending",
        "local fence is not global confirmation"
    );
    for (method, path, body) in [
        ("GET", "/api/auth/account", json!({})),
        ("PUT", "/api/auth/account", json!({"display_name":"late"})),
        (
            "PUT",
            "/api/auth/account/terms",
            json!({"version":TERMS_VERSION}),
        ),
    ] {
        assert_eq!(
            call(&router, method, path, "alice", body).await.0,
            StatusCode::UNAUTHORIZED
        );
    }
    let reopened = Arc::new(PostgresFrontDoorStore::connect(&url).await.unwrap());
    let router = durable::router(reopened, verifier);
    assert_eq!(
        call(&router, "GET", "/api/auth/account", "alice", json!({}))
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        call(&router, "GET", "/api/auth/account", "bob", json!({}))
            .await
            .1["display_name"],
        "Bob"
    );
    assert_eq!(
        call(&router, "GET", "/readyz", "bob", json!({})).await.0,
        StatusCode::SERVICE_UNAVAILABLE
    );
}
