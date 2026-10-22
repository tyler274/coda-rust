use super::*;
use axum::{
    Router,
    body::Bytes,
    extract::{OriginalUri, State},
    http::{HeaderMap, Method, StatusCode},
    response::{IntoResponse, Response as HttpResponse},
    routing::any,
};
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

static TEST_GATE: Mutex<()> = Mutex::const_new(());

#[derive(Default)]
struct Fixture {
    objects: Mutex<HashMap<String, Vec<u8>>>,
    metadata_calls: AtomicUsize,
    storage_calls: AtomicUsize,
    mode: AtomicUsize,
}
async fn handle(
    State(state): State<Arc<Fixture>>,
    OriginalUri(uri): OriginalUri,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> HttpResponse {
    let mode = state.mode.load(Ordering::SeqCst);
    if uri.path() == "/token" {
        state.metadata_calls.fetch_add(1, Ordering::SeqCst);
        assert_eq!(headers["metadata-flavor"], "Google");
        assert!(!headers.contains_key("authorization"));
        if mode == 4 {
            return (StatusCode::FOUND, [("location", "/must-not-follow")]).into_response();
        }
        if mode == 5 {
            return ([("metadata-flavor", "Google")], "not-json-secret").into_response();
        }
        if mode == 6 {
            return (
                [("metadata-flavor", "Google")],
                r#"{"access_token":"secret","token_type":"Bearer","expires_in":1}"#,
            )
                .into_response();
        }
        if mode == 7 {
            return r#"{"access_token":"secret","token_type":"Bearer","expires_in":3600}"#
                .into_response();
        }
        if mode == 8 {
            return ([("metadata-flavor", "Google")], "x".repeat(17000)).into_response();
        }
        return (
            [("metadata-flavor", "Google")],
            r#"{"access_token":"fixture-only-secret","token_type":"Bearer","expires_in":3600}"#,
        )
            .into_response();
    }
    state.storage_calls.fetch_add(1, Ordering::SeqCst);
    assert_eq!(headers["authorization"], "Bearer fixture-only-secret");
    let url = Url::parse(&format!("http://localhost{uri}")).unwrap();
    let query: HashMap<_, _> = url.query_pairs().collect();
    if mode == 9 {
        return (StatusCode::FOUND, [("location", "/must-not-follow")]).into_response();
    }
    if mode == 10 {
        return (StatusCode::FORBIDDEN, "provider-secret").into_response();
    }
    if mode == 11 {
        std::future::pending::<()>().await;
    }
    if method == Method::POST {
        assert_eq!(uri.path(), "/upload/storage/v1/b/fixture-bucket/o");
        assert_eq!(query["ifGenerationMatch"], "0");
        assert_eq!(query["uploadType"], "media");
        assert_eq!(headers["content-type"], "application/json");
        let name = query["name"].to_string();
        let key = name
            .strip_prefix("receipts/duet-rust-luna-v1/")
            .unwrap()
            .strip_suffix(".json")
            .unwrap();
        Uuid::parse_str(key).unwrap();
        let mut objects = state.objects.lock().await;
        if objects.contains_key(&name) {
            return StatusCode::PRECONDITION_FAILED.into_response();
        }
        objects.insert(name, body.to_vec());
        // Ambiguous outcome: upload committed but server returns failure.
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    assert_eq!(method, Method::GET);
    assert_eq!(query["alt"], "media");
    assert!(
        uri.path()
            .starts_with("/storage/v1/b/fixture-bucket/o/receipts%2Fduet-rust-luna-v1%2F")
    );
    if mode == 2 {
        return "corrupt".into_response();
    }
    if mode == 3 {
        return "x".repeat(4097).into_response();
    }
    let encoded = uri
        .path()
        .strip_prefix("/storage/v1/b/fixture-bucket/o/")
        .unwrap();
    let name = encoded.replace("%2F", "/");
    state.objects.lock().await.get(&name).cloned().map_or_else(
        || StatusCode::NOT_FOUND.into_response(),
        |b| b.into_response(),
    )
}
async fn fixture(mode: usize) -> (GcsReceiptArchive, Arc<Fixture>, tokio::task::JoinHandle<()>) {
    let state = Arc::new(Fixture::default());
    state.mode.store(mode, Ordering::SeqCst);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin = format!("http://{}/", listener.local_addr().unwrap());
    let router = Router::new()
        .fallback(any(handle))
        .with_state(state.clone());
    let server = tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });
    let mut archive = GcsReceiptArchive::new("fixture-bucket".into()).unwrap();
    archive.endpoints = Some((format!("{origin}token"), origin));
    (archive, state, server)
}
#[tokio::test]
async fn conditional_creation_ambiguous_readback_corruption_and_credentials() {
    let _gate = TEST_GATE.lock().await;
    let (archive, state, server) = fixture(0).await;
    let key = Uuid::new_v4();
    assert!(archive.create(key, b"original").await.is_err());
    assert_eq!(archive.read(key).await.unwrap(), b"original");
    assert!(archive.create(key, b"replacement").await.is_err());
    assert_eq!(archive.read(key).await.unwrap(), b"original");
    assert_eq!(state.metadata_calls.load(Ordering::SeqCst), 1);
    state.mode.store(3, Ordering::SeqCst);
    assert!(archive.read(key).await.is_err());
    state.mode.store(9, Ordering::SeqCst);
    let before = state.storage_calls.load(Ordering::SeqCst);
    assert!(archive.read(key).await.is_err());
    assert_eq!(state.storage_calls.load(Ordering::SeqCst), before + 1);
    state.mode.store(10, Ordering::SeqCst);
    assert_eq!(
        format!("{:?}", archive.read(key).await.unwrap_err()),
        "ArchiveUnavailable"
    );
    // Saturation refuses immediately, without even reaching metadata/storage.
    let permits = REQUESTS.acquire_many(4).await.unwrap();
    let before = state.storage_calls.load(Ordering::SeqCst);
    assert!(archive.read(key).await.is_err());
    assert_eq!(state.storage_calls.load(Ordering::SeqCst), before);
    drop(permits);
    let before = state.storage_calls.load(Ordering::SeqCst);
    assert!(archive.create(key, &vec![0; 4097]).await.is_err());
    assert!(archive.read(Uuid::nil()).await.is_err());
    assert_eq!(state.storage_calls.load(Ordering::SeqCst), before);
    // Expired credential isn't reused; failed refresh has bounded retry rate.
    archive
        .credentials
        .lock()
        .await
        .value
        .as_mut()
        .unwrap()
        .until = Instant::now();
    assert!(archive.read(key).await.is_err());
    assert_eq!(state.metadata_calls.load(Ordering::SeqCst), 1);
    server.abort();
}
#[tokio::test]
async fn metadata_failures_never_forward_credentials_or_follow_redirects() {
    let _gate = TEST_GATE.lock().await;
    for mode in 4..=8 {
        let (archive, state, server) = fixture(mode).await;
        assert!(archive.read(Uuid::new_v4()).await.is_err());
        assert!(archive.read(Uuid::new_v4()).await.is_err());
        assert_eq!(state.metadata_calls.load(Ordering::SeqCst), 1);
        assert_eq!(state.storage_calls.load(Ordering::SeqCst), 0);
        server.abort();
    }
}
#[test]
fn production_trust_and_object_namespace_cannot_be_configured_as_urls() {
    for bad in [
        "",
        "https://evil.test",
        "../bucket",
        "bucket/path",
        "bucket?x=y",
        "bucket\n",
        "Bucket",
    ] {
        assert!(GcsReceiptArchive::new(bad.into()).is_err());
    }
    let archive = GcsReceiptArchive::new("fixture-bucket".into()).unwrap();
    assert_eq!(archive.endpoints(), (METADATA, STORAGE));
    let url = archive.object_url(Uuid::new_v4(), true).unwrap();
    assert_eq!(url.host_str(), Some("storage.googleapis.com"));
    assert_eq!(url.scheme(), "https");
}
#[tokio::test]
async fn transport_timeout_and_cancellation_release_capacity() {
    let _gate = TEST_GATE.lock().await;
    let (archive, state, server) = fixture(11).await;
    assert!(
        tokio::time::timeout(Duration::from_secs(12), archive.read(Uuid::new_v4()))
            .await
            .unwrap()
            .is_err()
    );
    // Cancellation drops async I/O rather than leaving detached blocking workers.
    assert!(
        tokio::time::timeout(Duration::from_millis(50), archive.read(Uuid::new_v4()))
            .await
            .is_err()
    );
    state.mode.store(0, Ordering::SeqCst);
    let key = Uuid::new_v4();
    assert!(archive.create(key, b"after-cancel").await.is_err());
    assert_eq!(archive.read(key).await.unwrap(), b"after-cancel");
    server.abort();
}

#[tokio::test]
async fn chunked_readback_cannot_bypass_body_limit() {
    let _gate = TEST_GATE.lock().await;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin = format!("http://{}/", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut request = [0; 4096];
        let received = socket.read(&mut request).await.unwrap();
        assert!(received > 0);
        socket.write_all(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n1001\r\n").await.unwrap();
        socket.write_all(&vec![b'x'; 4097]).await.unwrap();
        socket.write_all(b"\r\n0\r\n\r\n").await.unwrap();
    });
    let mut archive = GcsReceiptArchive::new("fixture-bucket".into()).unwrap();
    archive.endpoints = Some((format!("{origin}token"), origin));
    archive.credentials.lock().await.value = Some(Credential {
        token: "fixture-only-secret".into(),
        until: Instant::now() + Duration::from_secs(60),
    });
    assert!(archive.read(Uuid::new_v4()).await.is_err());
    server.await.unwrap();
}
#[tokio::test]
async fn gcs_transport_and_postgres_recovery_reconcile_same_receipt_after_restart() {
    let _gate = TEST_GATE.lock().await;
    use front_door::{
        PostgresFrontDoorStore,
        identity::{LUNA_VERSION, Principal, TERMS_VERSION},
    };
    let url = match std::env::var("DUET_IDENTITY_TEST_DATABASE_URL") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            assert!(
                std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
                "DUET_IDENTITY_TEST_DATABASE_URL is required"
            );
            return;
        }
    };
    let store = PostgresFrontDoorStore::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let id = Uuid::new_v4();
    let p = Principal {
        project: "fixture".into(),
        subject: id.to_string(),
        email: format!("{id}@example.test"),
        authenticated_at: 1,
    };
    store.provision_identity(&p, 10).await.unwrap();
    store
        .accept_identity_terms(&p, TERMS_VERSION, 10)
        .await
        .unwrap();
    store
        .set_identity_luna(&p, LUNA_VERSION, true, 10)
        .await
        .unwrap();
    store
        .set_identity_luna(&p, LUNA_VERSION, false, 10)
        .await
        .unwrap();
    let (archive, state, server) = fixture(2).await;
    let report = store.recover_luna_restrictions(&archive, 10).await.unwrap();
    assert!(report.failed >= 1);
    assert!(
        store
            .identity_access(&p)
            .await
            .unwrap()
            .permissions
            .restriction_pending
    );
    drop(store);
    let store = PostgresFrontDoorStore::connect(&url).await.unwrap();
    state.mode.store(0, Ordering::SeqCst);
    let report = store
        .recover_luna_restrictions(&archive, 3600)
        .await
        .unwrap();
    assert!(report.confirmed >= 1);
    let access = store.identity_access(&p).await.unwrap();
    assert!(!access.permissions.restriction_pending);
    assert!(!access.allows_luna());
    assert!(
        store
            .set_identity_luna(&p, LUNA_VERSION, true, 3601)
            .await
            .unwrap()
            .allows_luna()
    );
    server.abort();
}
