//! Real canonical Postgres + HTTP router + controlled archive. No live provider.
use async_trait::async_trait;
use axum::{
    Router,
    body::Body,
    http::{HeaderMap, Request, StatusCode},
};
use duet_api::{
    durable::{self, IdentityProvider},
    identity::IdentityError,
};
use front_door::{
    PostgresFrontDoorStore,
    identity::{LUNA_VERSION, Principal, TERMS_VERSION},
    journal::{ArchiveUnavailable, ReceiptArchive},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU8, Ordering},
    },
};
use tokio::sync::Notify;
use tower::ServiceExt;
use uuid::Uuid;
struct Verified {
    alice: Principal,
    bob: Principal,
}
#[async_trait]
impl IdentityProvider for Verified {
    async fn verify(&self, token: &str) -> Result<Principal, IdentityError> {
        match token {
            "alice" => Ok(self.alice.clone()),
            "bob" => Ok(self.bob.clone()),
            "offline" => Err(IdentityError::Unavailable),
            t if t.starts_with("dev:") => Ok(self.alice.clone()),
            _ => Err(IdentityError::Invalid),
        }
    }
}
#[derive(Default)]
struct Archive {
    objects: Mutex<HashMap<Uuid, Vec<u8>>>,
    mode: AtomicU8,
    entered: Notify,
    release: Notify,
}
#[async_trait]
impl ReceiptArchive for Archive {
    async fn create(&self, id: Uuid, body: &[u8]) -> Result<(), ArchiveUnavailable> {
        if self.mode.load(Ordering::SeqCst) == 1 {
            return Ok(());
        } // upload success alone is NOT evidence
        self.objects
            .lock()
            .unwrap()
            .entry(id)
            .or_insert_with(|| body.to_vec());
        self.entered.notify_one();
        if self.mode.load(Ordering::SeqCst) == 2 {
            self.release.notified().await;
        }
        Err(ArchiveUnavailable) // committed upload, lost acknowledgment
    }
    async fn read(&self, id: Uuid) -> Result<Vec<u8>, ArchiveUnavailable> {
        self.objects
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or(ArchiveUnavailable)
    }
}
async fn call(
    router: &Router,
    method: &str,
    path: &str,
    token: &str,
    body: Value,
) -> (StatusCode, HeaderMap, Value) {
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
    let headers = response.headers().clone();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        headers,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}
fn consent(decision: &str) -> Value {
    json!({"decision":decision,"version":LUNA_VERSION})
}
const PATH: &str = "/api/intelligence/consent";
fn pending(result: &(StatusCode, HeaderMap, Value)) {
    assert_eq!(result.0, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(result.1["retry-after"], "30");
    assert_eq!(
        result.2,
        json!({"detail":"This privacy change is pending confirmation. Please retry."})
    );
}
#[tokio::test]
async fn truthful_withdrawal_http_identity_receipt_and_revocation_boundaries() {
    let url = match std::env::var("DUET_IDENTITY_TEST_DATABASE_URL") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
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
    let principal = || {
        let id = Uuid::new_v4();
        Principal {
            project: "fixture".into(),
            subject: id.to_string(),
            email: format!("{id}@example.test"),
            authenticated_at: now - 10,
        }
    };
    let identity = Arc::new(Verified {
        alice: principal(),
        bob: principal(),
    });
    let archive = Arc::new(Archive::default());
    let router =
        durable::router_with_permissions(store.clone(), identity.clone(), Some(archive.clone()));
    assert_eq!(
        call(&router, "PUT", PATH, "alice", consent("declined"))
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    for token in ["alice", "bob"] {
        assert_eq!(
            call(&router, "PUT", "/api/auth/account", token, json!({}))
                .await
                .0,
            StatusCode::OK
        );
        assert_eq!(
            call(
                &router,
                "PUT",
                "/api/auth/account/terms",
                token,
                json!({"version":TERMS_VERSION})
            )
            .await
            .0,
            StatusCode::OK
        );
    }
    for token in ["bad", "dev:alice", ""] {
        assert_eq!(
            call(&router, "PUT", PATH, token, consent("declined"))
                .await
                .0,
            StatusCode::UNAUTHORIZED
        );
    }
    assert_eq!(
        call(&router, "PUT", PATH, "offline", consent("declined"))
            .await
            .0,
        StatusCode::SERVICE_UNAVAILABLE
    );
    assert_eq!(
        call(
            &router,
            "PUT",
            PATH,
            "alice",
            json!({"decision":"declined","version":"old"})
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    assert_eq!(
        call(&router, "PUT", PATH, "alice", consent("undecided"))
            .await
            .2,
        json!({"detail":"Luna consent requires a decision"})
    );
    assert_eq!(
        call(
            &router,
            "PUT",
            PATH,
            "alice",
            json!({"decision":"declined","version":LUNA_VERSION,"account_id":Uuid::new_v4()})
        )
        .await
        .0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(
        call(&router, "PUT", PATH, "alice", consent("accepted"))
            .await
            .0,
        StatusCode::SERVICE_UNAVAILABLE
    );
    let original = store
        .identity_access(&identity.alice)
        .await
        .unwrap()
        .permissions;
    assert_eq!(original.luna_version, None);
    // Seed previously accepted permission through its owner, never via route qualification.
    store
        .set_identity_luna(&identity.alice, LUNA_VERSION, true, now)
        .await
        .unwrap();
    store
        .set_identity_luna(&identity.bob, LUNA_VERSION, true, now)
        .await
        .unwrap();
    let unavailable = durable::router_with_permissions(store.clone(), identity.clone(), None);
    pending(&call(&unavailable, "PUT", PATH, "alice", consent("declined")).await);
    let withdrawal = store
        .identity_access(&identity.alice)
        .await
        .unwrap()
        .permissions;
    assert!(!withdrawal.luna_allowed);
    assert!(withdrawal.restriction_pending);
    assert!(
        store
            .identity_access(&identity.bob)
            .await
            .unwrap()
            .allows_luna()
    );
    archive.mode.store(1, Ordering::SeqCst);
    pending(&call(&router, "PUT", PATH, "alice", consent("declined")).await);
    assert_eq!(
        withdrawal,
        store
            .identity_access(&identity.alice)
            .await
            .unwrap()
            .permissions
    );
    archive.mode.store(0, Ordering::SeqCst);
    // Same pending receipt is reconciled by the bounded recovery owner after backoff.
    let reopened = PostgresFrontDoorStore::connect(&url).await.unwrap();
    reopened
        .recover_luna_restrictions(archive.as_ref(), now + 3601)
        .await
        .unwrap();
    let confirmed = call(&router, "PUT", PATH, "alice", consent("declined")).await;
    assert_eq!(confirmed.0, StatusCode::OK);
    let wire: duet_wire::models::AccountState =
        serde_json::from_value(confirmed.2.clone()).unwrap();
    assert_eq!(
        wire.luna_consent,
        Some(duet_wire::models::LunaConsentDecision::Declined)
    );
    assert_eq!(wire.intelligence_available, Some(false));
    assert_eq!(wire.intelligence_enabled, Some(false));
    let count = archive.objects.lock().unwrap().len();
    let retry = call(&router, "PUT", PATH, "alice", consent("declined")).await;
    assert_eq!(retry.2, confirmed.2);
    assert_eq!(archive.objects.lock().unwrap().len(), count);
    assert_eq!(
        call(&router, "PUT", PATH, "alice", consent("accepted"))
            .await
            .0,
        StatusCode::SERVICE_UNAVAILABLE
    );
    assert!(
        !store
            .identity_access(&identity.alice)
            .await
            .unwrap()
            .allows_luna()
    );
    // Direct HTTP confirmation resolves ambiguous creation by exact readback.
    // Parallel retries share one durable transition, never two receipts.
    store
        .set_identity_luna(&identity.alice, LUNA_VERSION, true, now)
        .await
        .unwrap();
    let before = archive.objects.lock().unwrap().len();
    let (left, right) = tokio::join!(
        call(&router, "PUT", PATH, "alice", consent("declined")),
        call(&router, "PUT", PATH, "alice", consent("declined"))
    );
    assert!(left.0 == StatusCode::OK || right.0 == StatusCode::OK);
    for result in [&left, &right] {
        if result.0 != StatusCode::OK {
            pending(result);
        }
    }
    assert_eq!(archive.objects.lock().unwrap().len(), before + 1);
    assert_eq!(
        call(&router, "PUT", PATH, "alice", consent("declined"))
            .await
            .0,
        StatusCode::OK
    );
    // Cancellation after an ambiguous committed upload leaves SQL restriction and
    // stable object. Recovery waits for lease expiry, then reads the same bytes.
    archive.mode.store(2, Ordering::SeqCst);
    let request = call(&router, "PUT", PATH, "bob", consent("declined"));
    assert!(
        tokio::time::timeout(std::time::Duration::from_millis(100), request)
            .await
            .is_err()
    );
    assert!(
        store
            .identity_access(&identity.bob)
            .await
            .unwrap()
            .permissions
            .restriction_pending
    );
    let count = archive.objects.lock().unwrap().len();
    pending(&call(&router, "PUT", PATH, "bob", consent("declined")).await);
    archive.mode.store(0, Ordering::SeqCst);
    reopened
        .recover_luna_restrictions(archive.as_ref(), now + 3601)
        .await
        .unwrap();
    assert_eq!(
        call(&router, "PUT", PATH, "bob", consent("declined"))
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(archive.objects.lock().unwrap().len(), count);
    // Force a new owner transition to exercise revocation during receipt I/O.
    store
        .set_identity_luna(&identity.bob, LUNA_VERSION, true, now)
        .await
        .unwrap();
    archive.mode.store(2, Ordering::SeqCst);
    // Drain an earlier notification before waiting on the new upload.
    while tokio::time::timeout(
        std::time::Duration::from_millis(1),
        archive.entered.notified(),
    )
    .await
    .is_ok()
    {}
    let task_router = router.clone();
    let task =
        tokio::spawn(
            async move { call(&task_router, "PUT", PATH, "bob", consent("declined")).await },
        );
    tokio::time::timeout(
        std::time::Duration::from_secs(5),
        archive.entered.notified(),
    )
    .await
    .unwrap();
    assert_eq!(
        call(
            &router,
            "POST",
            "/api/auth/session/revoke",
            "bob",
            json!({})
        )
        .await
        .0,
        StatusCode::ACCEPTED
    );
    archive.release.notify_one();
    assert_eq!(task.await.unwrap().0, StatusCode::UNAUTHORIZED);
    let mut fresh = identity.bob.clone();
    fresh.authenticated_at = chrono::Utc::now().timestamp() + 2;
    assert!(!store.identity_access(&fresh).await.unwrap().allows_luna());
    assert_eq!(
        call(&router, "GET", "/readyz", "alice", json!({})).await.0,
        StatusCode::SERVICE_UNAVAILABLE
    );
    for path in ["/api/conversation/messages", "/api/intelligence/settings"] {
        assert_eq!(
            call(&router, "PUT", path, "alice", json!({})).await.0,
            StatusCode::NOT_FOUND
        );
    }
    // Reverse race order: a revoked request cannot create another restriction.
    let before = archive.objects.lock().unwrap().len();
    assert_eq!(
        call(&router, "PUT", PATH, "bob", consent("declined"))
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(archive.objects.lock().unwrap().len(), before);
    let base = durable::router(store, identity);
    assert_eq!(
        call(&base, "PUT", PATH, "alice", consent("declined"))
            .await
            .0,
        StatusCode::NOT_FOUND
    );
}
