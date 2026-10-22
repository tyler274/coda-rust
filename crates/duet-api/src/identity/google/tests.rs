use super::*;
use axum::{
    Router,
    body::Body,
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    response::Response,
    routing::{get, post},
};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde_json::{Value, json};
use std::sync::{
    Mutex as StdMutex,
    atomic::{AtomicUsize, Ordering},
};

struct Fixture {
    keys: StdMutex<Value>,
    users: StdMutex<Value>,
    key_hits: AtomicUsize,
    metadata_hits: AtomicUsize,
    admin_hits: AtomicUsize,
    key_status: StdMutex<StatusCode>,
    admin_status: StdMutex<StatusCode>,
    delay_ms: AtomicUsize,
    admin_delay_ms: AtomicUsize,
    metadata_delay_ms: AtomicUsize,
    metadata: StdMutex<Value>,
}
struct Server {
    state: Arc<Fixture>,
    adapter: Arc<GoogleIdentity>,
    task: tokio::task::JoinHandle<()>,
}
impl Drop for Server {
    fn drop(&mut self) {
        self.task.abort();
    }
}
async fn keys(State(s): State<Arc<Fixture>>) -> Response {
    s.key_hits.fetch_add(1, Ordering::SeqCst);
    tokio::time::sleep(Duration::from_millis(
        s.delay_ms.load(Ordering::SeqCst) as u64
    ))
    .await;
    Response::builder()
        .status(*s.key_status.lock().unwrap())
        .header("cache-control", "public, max-age=3600")
        .header("location", "/metadata")
        .body(Body::from(s.keys.lock().unwrap().to_string()))
        .unwrap()
}
async fn metadata(State(s): State<Arc<Fixture>>, headers: HeaderMap) -> Response {
    s.metadata_hits.fetch_add(1, Ordering::SeqCst);
    tokio::time::sleep(Duration::from_millis(
        s.metadata_delay_ms.load(Ordering::SeqCst) as u64,
    ))
    .await;
    assert_eq!(headers["metadata-flavor"], "Google");
    assert!(headers.get("authorization").is_none());
    Response::builder()
        .header("metadata-flavor", "Google")
        .body(Body::from(s.metadata.lock().unwrap().to_string()))
        .unwrap()
}
async fn admin(State(s): State<Arc<Fixture>>, req: Request<Body>) -> Response {
    s.admin_hits.fetch_add(1, Ordering::SeqCst);
    tokio::time::sleep(Duration::from_millis(
        s.admin_delay_ms.load(Ordering::SeqCst) as u64,
    ))
    .await;
    assert_eq!(
        req.headers()["authorization"],
        "Bearer fixture-access-secret"
    );
    assert_eq!(req.uri().path(), "/v1/projects/duet-test/accounts:lookup");
    let body = axum::body::to_bytes(req.into_body(), 4096).await.unwrap();
    assert_eq!(
        serde_json::from_slice::<Value>(&body).unwrap(),
        json!({"localId":["alice"]})
    );
    Response::builder()
        .status(*s.admin_status.lock().unwrap())
        .body(Body::from(s.users.lock().unwrap().to_string()))
        .unwrap()
}
async fn server() -> Server {
    let state = Arc::new(Fixture {
        keys: StdMutex::new(
            json!({"one":include_str!("../../../tests/fixtures/identity-test-only.crt")}),
        ),
        users: StdMutex::new(json!({"users":[{"localId":"alice","validSince":"1"}]})),
        key_hits: AtomicUsize::new(0),
        metadata_hits: AtomicUsize::new(0),
        admin_hits: AtomicUsize::new(0),
        key_status: StdMutex::new(StatusCode::OK),
        admin_status: StdMutex::new(StatusCode::OK),
        delay_ms: AtomicUsize::new(0),
        admin_delay_ms: AtomicUsize::new(0),
        metadata_delay_ms: AtomicUsize::new(0),
        metadata: StdMutex::new(
            json!({"access_token":"fixture-access-secret","token_type":"Bearer","expires_in":3600}),
        ),
    });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let mut adapter = GoogleIdentity::new("duet-test".into()).unwrap();
    adapter.endpoints = Some((
        format!("{base}/keys"),
        format!("{base}/v1/projects/duet-test/accounts:lookup"),
        format!("{base}/metadata"),
    ));
    let router = Router::new()
        .route("/keys", get(keys))
        .route("/metadata", get(metadata))
        .route("/v1/projects/duet-test/accounts:lookup", post(admin))
        .with_state(state.clone());
    let task = tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });
    Server {
        state,
        adapter: Arc::new(adapter),
        task,
    }
}
fn token(kid: &str, auth_time: i64) -> String {
    let now = unix_now().unwrap();
    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some(kid.into());
    // Untrusted URLs are ignored, never used as key endpoints.
    header.jku = Some("http://127.0.0.1:1/do-not-fetch".into());
    encode(&header, &json!({"iss":"https://securetoken.google.com/duet-test", "aud":"duet-test","sub":"alice","exp":now+600,"iat":now,"auth_time":auth_time,"email":"alice@example.test","email_verified":true,"firebase":{"sign_in_provider":"google.com"}}), &EncodingKey::from_rsa_pem(include_bytes!("../../../tests/fixtures/identity-test-only.pem")).unwrap()).unwrap()
}

#[tokio::test]
async fn signed_identity_single_flight_and_live_revocation_boundary() {
    let s = server().await;
    let now = unix_now().unwrap();
    let token = token("one", now - 20);
    let mut tasks = Vec::new();
    for _ in 0..8 {
        let a = s.adapter.clone();
        let token = token.clone();
        tasks.push(tokio::spawn(async move { a.verify(&token).await }));
    }
    for task in tasks {
        assert_eq!(task.await.unwrap().unwrap().subject, "alice");
    }
    assert_eq!(s.state.key_hits.load(Ordering::SeqCst), 1);
    assert_eq!(s.state.metadata_hits.load(Ordering::SeqCst), 1);
    assert_eq!(s.state.admin_hits.load(Ordering::SeqCst), 8);
    *s.state.users.lock().unwrap() =
        json!({"users":[{"localId":"alice","validSince":(now-20).to_string()}]});
    assert!(
        s.adapter.verify(&token).await.is_ok(),
        "provider cutoff equality remains valid"
    );
    *s.state.users.lock().unwrap() =
        json!({"users":[{"localId":"alice","validSince":(now-19).to_string()}]});
    assert!(
        matches!(s.adapter.verify(&token).await, Err(IdentityError::Revoked)),
        "fresh iat cannot replace original auth_time"
    );
    *s.state.users.lock().unwrap() =
        json!({"users":[{"localId":"alice","validSince":"1","disabled":true}]});
    assert!(matches!(
        s.adapter.verify(&token).await,
        Err(IdentityError::Revoked)
    ));
}

#[tokio::test]
async fn unknown_key_rotation_expiry_and_failed_refresh_are_bounded() {
    let s = server().await;
    let now = unix_now().unwrap();
    assert!(s.adapter.verify(&token("one", now)).await.is_ok());
    *s.state.keys.lock().unwrap() =
        json!({"two":include_str!("../../../tests/fixtures/identity-test-only.crt")});
    for _ in 0..5 {
        assert!(s.adapter.verify(&token("two", now)).await.is_err());
    }
    assert_eq!(s.state.key_hits.load(Ordering::SeqCst), 1);
    s.adapter.keys.lock().await.attempted = Some(Instant::now() - REFRESH_INTERVAL);
    assert!(s.adapter.verify(&token("two", now)).await.is_ok());
    assert!(
        s.adapter.verify(&token("one", now)).await.is_err(),
        "removed key no longer trusted"
    );
    {
        let mut cache = s.adapter.keys.lock().await;
        cache.verifier.as_mut().unwrap().1 = Instant::now();
        cache.attempted = Some(Instant::now() - REFRESH_INTERVAL);
    }
    *s.state.key_status.lock().unwrap() = StatusCode::SERVICE_UNAVAILABLE;
    for _ in 0..5 {
        assert!(s.adapter.verify(&token("two", now)).await.is_err());
    }
    assert_eq!(s.state.key_hits.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn lookup_rejects_deleted_foreign_tenant_malformed_and_oversized_users() {
    let s = server().await;
    assert!(s.adapter.current("other-project", "alice").await.is_err());
    assert_eq!(s.state.metadata_hits.load(Ordering::SeqCst), 0);
    for users in [
        json!({}),
        json!({"users":[]}),
        json!({"users":[{"localId":"bob","validSince":"1"}]}),
        json!({"users":[{"localId":"alice"}]}),
        json!({"users":[{"localId":"alice","validSince":"-1"}]}),
        json!({"users":[{"localId":"alice","validSince":"bad"}]}),
        json!({"users":[{"localId":"alice","validSince":"1","tenantId":"foreign"}]}),
        json!({"users":[{"localId":"alice","validSince":"1"},{"localId":"alice","validSince":"1"}]}),
        json!({"oversized":"x".repeat(MAX_BODY)}),
    ] {
        *s.state.users.lock().unwrap() = users;
        assert!(s.adapter.current("duet-test", "alice").await.is_err());
    }
    for status in [
        StatusCode::FORBIDDEN,
        StatusCode::UNAUTHORIZED,
        StatusCode::FOUND,
        StatusCode::TOO_MANY_REQUESTS,
        StatusCode::INTERNAL_SERVER_ERROR,
    ] {
        *s.state.admin_status.lock().unwrap() = status;
        assert!(s.adapter.current("duet-test", "alice").await.is_err());
    }
}

#[tokio::test]
async fn timeout_bad_keys_and_credentials_do_not_leak_secrets_or_authorize() {
    let s = server().await;
    for keys in [
        json!({}),
        json!({"one":"not-a-certificate"}),
        json!({"one":"x".repeat(MAX_BODY)}),
        json!([]),
    ] {
        *s.state.keys.lock().unwrap() = keys;
        s.adapter.keys.lock().await.attempted = None;
        let err = s
            .adapter
            .verify(&token("one", unix_now().unwrap()))
            .await
            .err()
            .unwrap();
        assert_eq!(format!("{err:?}"), "Unavailable");
    }
    for metadata in [
        json!({"access_token":"secret","token_type":"Basic","expires_in":3600}),
        json!({"access_token":"secret","token_type":"Bearer","expires_in":30}),
        json!({"access_token":"\r\nsecret","token_type":"Bearer","expires_in":3600}),
        json!({"error":"fixture-access-secret"}),
    ] {
        *s.state.metadata.lock().unwrap() = metadata;
        let err = s.adapter.current("duet-test", "alice").await.err().unwrap();
        assert_eq!(format!("{err:?}"), "Unavailable");
    }
    assert_eq!(s.state.admin_hits.load(Ordering::SeqCst), 0);
    s.adapter.keys.lock().await.attempted = None;
    s.state.delay_ms.store(6000, Ordering::SeqCst);
    let start = Instant::now();
    assert!(
        s.adapter
            .verify(&token("one", unix_now().unwrap()))
            .await
            .is_err()
    );
    assert!(start.elapsed() < Duration::from_secs(6));
    let count = s.state.key_hits.load(Ordering::SeqCst);
    assert!(
        s.adapter
            .verify(&token("one", unix_now().unwrap()))
            .await
            .is_err()
    );
    assert_eq!(s.state.key_hits.load(Ordering::SeqCst), count);
}

#[tokio::test]
async fn redirects_and_malformed_json_do_not_reach_credentials() {
    let s = server().await;
    *s.state.key_status.lock().unwrap() = StatusCode::FOUND;
    assert!(
        s.adapter
            .verify(&token("one", unix_now().unwrap()))
            .await
            .is_err()
    );
    assert_eq!(s.state.metadata_hits.load(Ordering::SeqCst), 0);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        axum::serve(
            listener,
            Router::new().route(
                "/",
                get(|| async {
                    (
                        [("cache-control", "max-age=3600")],
                        "{broken fixture-secret",
                    )
                }),
            ),
        )
        .await
        .unwrap();
    });
    let mut adapter = GoogleIdentity::new("duet-test".into()).unwrap();
    adapter.endpoints = Some((base.clone(), base.clone(), base));
    let result = adapter.verify(&token("one", unix_now().unwrap())).await;
    assert_eq!(format!("{:?}", result.err().unwrap()), "Unavailable");
    task.abort();
}

#[tokio::test]
async fn credentials_expire_and_metadata_and_admin_timeouts_fail_closed() {
    let s = server().await;
    assert!(s.adapter.current("duet-test", "alice").await.is_ok());
    s.adapter.credential.lock().await.as_mut().unwrap().until = Instant::now();
    assert!(s.adapter.current("duet-test", "alice").await.is_ok());
    assert_eq!(s.state.metadata_hits.load(Ordering::SeqCst), 2);
    s.state.admin_delay_ms.store(6000, Ordering::SeqCst);
    assert!(s.adapter.current("duet-test", "alice").await.is_err());
    s.state.admin_delay_ms.store(0, Ordering::SeqCst);
    *s.adapter.credential.lock().await = None;
    s.state.metadata_delay_ms.store(6000, Ordering::SeqCst);
    assert!(s.adapter.current("duet-test", "alice").await.is_err());
    assert_eq!(s.state.admin_hits.load(Ordering::SeqCst), 3);
    let permits = s.adapter.requests.acquire_many(16).await.unwrap();
    assert!(s.adapter.current("duet-test", "alice").await.is_err());
    drop(permits);
}

#[test]
fn freshness_and_configuration_are_fail_closed() {
    for project in [
        "../escape",
        "https://evil.test",
        "Uppercase",
        "short",
        "test-project/",
        "test-project?",
    ] {
        assert!(GoogleIdentity::new(project.into()).is_err());
    }
    for cache in [
        "",
        "no-cache, max-age=300",
        "no-cache=\"field\", max-age=300",
        "no-store, max-age=300",
        "max-age=0",
        "max-age=abc",
        "max-age=1, max-age=2",
    ] {
        let mut h = HeaderMap::new();
        h.insert(header::CACHE_CONTROL, cache.parse().unwrap());
        assert!(freshness(&h).is_err());
    }
    let mut h = HeaderMap::new();
    h.insert(
        header::CACHE_CONTROL,
        "public,max-age=7200".parse().unwrap(),
    );
    assert_eq!(freshness(&h).unwrap(), 3600);
    h.insert(header::AGE, "7199".parse().unwrap());
    assert_eq!(freshness(&h).unwrap(), 1);
    h.insert(header::AGE, "7200".parse().unwrap());
    assert!(freshness(&h).is_err());
}

#[tokio::test]
async fn bounded_body_rejects_chunked_overflow() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let mut buf = [0; 2048];
        let _ = stream.read(&mut buf).await.unwrap();
        stream.write_all(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n8\r\n12345678\r\n8\r\n12345678\r\n0\r\n\r\n").await.unwrap();
    });
    let response = GoogleIdentity::new("duet-test".into())
        .unwrap()
        .client
        .get(url)
        .send()
        .await
        .unwrap();
    assert!(bounded(response, 10).await.is_err());
    task.await.unwrap();
}
