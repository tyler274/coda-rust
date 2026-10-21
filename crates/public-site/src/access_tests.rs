//! Local, deliberately public fixtures; never fetch Google keys during tests.
use super::*;
use base64::Engine;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde_json::{Value, json};
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) const AUDIENCE: &str =
    "/projects/134570912572/locations/us-east1/services/public-site-staging";
const KID: &str = "public-test-key";
fn fixture() -> Value {
    serde_json::from_str(include_str!("../tests/fixtures/iap-public-test-key.json")).unwrap()
}
fn jwks() -> JwkSet {
    serde_json::from_value(fixture()).unwrap()
}
fn claims(now: u64) -> Value {
    json!({"iss": ISSUER, "aud": AUDIENCE, "iat": now - 10, "exp": now + 590})
}
fn sign(claims: &Value, kid: Option<&str>) -> String {
    let der = base64::engine::general_purpose::STANDARD
        .decode(fixture()["private_pkcs8_base64"].as_str().unwrap())
        .unwrap();
    let mut header = Header::new(Algorithm::ES256);
    header.kid = kid.map(str::to_owned);
    encode(&header, claims, &EncodingKey::from_ec_der(&der)).unwrap()
}
pub(crate) fn assertion() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    sign(&claims(now), Some(KID))
}
pub(crate) fn cached_gate() -> AccessGate {
    let mut gate = AccessGate::new(AccessConfig {
        audience: Some(AUDIENCE.into()),
    });
    gate.cache = Arc::new(Mutex::new(Cache {
        keys: Some(jwks()),
        fetched: Some(Instant::now()),
        attempted: Some(Instant::now()),
    }));
    // A network attempt from a cached fixture can only hit this closed local port.
    gate.test_key_url = Some("http://127.0.0.1:1".into());
    gate
}
fn headers(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("x-goog-iap-jwt-assertion", token.parse().unwrap());
    headers
}

#[test]
fn signature_claim_types_and_exact_time_boundaries() {
    let now = 2_000_000_000;
    let key = decoding_key(&jwks(), KID).unwrap();
    let good = claims(now);
    assert!(verify(&sign(&good, Some(KID)), AUDIENCE, &key, now));
    for (name, value) in [
        ("iss", json!("https://attacker.test")),
        ("iss", json!(format!("{ISSUER}/"))),
        ("aud", json!("wrong")),
        ("aud", json!([AUDIENCE])),
        ("aud", json!(null)),
        ("iat", json!(now + 30)),
        ("iat", json!(now + 31)),
        ("exp", json!(now - 30)),
        ("exp", json!(now - 31)),
        ("exp", json!(now + 651)), // lifetime 661
        ("exp", json!(now - 10)),  // exp == iat
        ("exp", json!(now - 11)),
        ("nbf", json!(now + 31)),
        ("nbf", json!(null)),
        ("nbf", json!("123")),
        ("nbf", json!(-1)),
        ("nbf", json!(1.5)),
        ("iat", json!("123")),
        ("exp", json!("123")),
        ("iat", json!(null)),
        ("exp", json!(null)),
        ("iat", json!(-1)),
        ("exp", json!(-1)),
        ("iat", json!(1.5)),
        ("exp", json!(1.5)),
        ("iat", json!(u64::MAX)),
        ("exp", json!(u64::MAX)),
    ] {
        let mut bad = good.clone();
        bad[name] = value;
        assert!(
            !verify(&sign(&bad, Some(KID)), AUDIENCE, &key, now),
            "{bad}"
        );
    }
    for field in ["iss", "aud", "iat", "exp"] {
        let mut bad = good.clone();
        bad.as_object_mut().unwrap().remove(field);
        assert!(
            !verify(&sign(&bad, Some(KID)), AUDIENCE, &key, now),
            "missing {field}"
        );
    }
    for (iat, exp, nbf) in [
        (now + 29, now + 600, now + 30),
        (now - 600, now - 29, now - 600),
        (now - 10, now + 650, now),
    ] {
        let mut valid = good.clone();
        valid["iat"] = json!(iat);
        valid["exp"] = json!(exp);
        valid["nbf"] = json!(nbf);
        assert!(
            verify(&sign(&valid, Some(KID)), AUDIENCE, &key, now),
            "{valid}"
        );
    }
    let token = sign(&good, Some(KID));
    let mut parts: Vec<String> = token.split('.').map(str::to_owned).collect();
    let replacement = if parts[2].starts_with('A') { "B" } else { "A" };
    parts[2].replace_range(0..1, replacement);
    assert!(!verify(&parts.join("."), AUDIENCE, &key, now));
}

#[tokio::test]
async fn only_one_signed_iap_header_authorizes() {
    let gate = cached_gate();
    let token = assertion();
    assert!(gate.allows(&headers(&token)).await);
    let mut legacy = HeaderMap::new();
    for (name, value) in [
        ("cf-access-client-id", "id"),
        ("cf-access-client-secret", "secret"),
        ("cf-access-jwt-assertion", token.as_str()),
        (
            "x-goog-authenticated-user-email",
            "accounts.google.com:engineer@example.test",
        ),
        ("x-goog-authenticated-user-id", "accounts.google.com:123"),
        ("x-coda-staging-token", "probe"),
        ("authorization", "Bearer unsigned"),
    ] {
        legacy.insert(name, value.parse().unwrap());
    }
    assert!(!gate.allows(&legacy).await);
    for invalid in [
        "",
        " ",
        "garbage",
        &format!("{token}, {token}"),
        &"a".repeat(16385),
    ] {
        assert!(!gate.allows(&headers(invalid)).await);
    }
    let mut duplicate = headers(&token);
    duplicate.append("x-goog-iap-jwt-assertion", token.parse().unwrap());
    assert!(!gate.allows(&duplicate).await);
    for kid in [None, Some(""), Some("unknown"), Some(&"k".repeat(257))] {
        assert!(
            !gate
                .allows(&headers(&sign(&claims(2_000_000_000), kid)))
                .await
        );
    }
    for algorithm in [Algorithm::HS256, Algorithm::HS384, Algorithm::HS512] {
        let mut header = Header::new(algorithm);
        header.kid = Some(KID.into());
        let token = encode(
            &header,
            &claims(2_000_000_000),
            &EncodingKey::from_secret(b"not-a-key"),
        )
        .unwrap();
        assert!(!gate.allows(&headers(&token)).await);
    }
    for algorithm in ["none", "RS256", "ES384"] {
        let mut parts: Vec<String> = token.split('.').map(str::to_owned).collect();
        parts[0] = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(json!({"alg": algorithm, "kid": KID}).to_string());
        assert!(!gate.allows(&headers(&parts.join("."))).await);
    }
    assert!(
        !AccessGate::new(AccessConfig::default())
            .allows(&headers(&token))
            .await
    );
}

#[test]
fn rejects_wrong_key_type_curve_usage_algorithm_and_ambiguous_kid() {
    for (name, value) in [
        ("alg", json!("RS256")),
        ("crv", json!("P-384")),
        ("use", json!("enc")),
        ("key_ops", json!(["sign"])),
    ] {
        let mut set = fixture();
        set["keys"][0][name] = value;
        assert!(decoding_key(&serde_json::from_value(set).unwrap(), KID).is_none());
    }
    let rsa = json!({"keys":[{"kty":"RSA","kid":KID,"n":"AQAB","e":"AQAB"}]});
    assert!(decoding_key(&serde_json::from_value(rsa).unwrap(), KID).is_none());
    let mut keys = jwks();
    keys.keys.push(keys.keys[0].clone());
    assert!(decoding_key(&keys, KID).is_none());
}

// Real reqwest transport, injectable only under cfg(test). This proves redirects,
// status/size bounds and cache behavior without changing the production trust URL.
async fn source(
    status: u16,
    body: String,
) -> (AccessGate, Arc<AtomicUsize>, tokio::task::JoinHandle<()>) {
    use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
    let calls = Arc::new(AtomicUsize::new(0));
    let count = calls.clone();
    let app = Router::new().route(
        "/keys",
        get(move || {
            let count = count.clone();
            let body = body.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                (
                    StatusCode::from_u16(status).unwrap(),
                    [("location", "/keys")],
                    body,
                )
                    .into_response()
            }
        }),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/keys", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    let mut gate = AccessGate::new(AccessConfig {
        audience: Some(AUDIENCE.into()),
    });
    gate.client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(3))
        .connect_timeout(Duration::from_secs(2))
        .build()
        .unwrap();
    gate.test_key_url = Some(url);
    (gate, calls, task)
}

#[tokio::test]
async fn transport_bounds_chunked_body_and_stalled_response() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    for stalled in [false, true] {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let task = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut request = [0; 4096];
            // One read is enough to know the client sent its request line.
            let read = socket.read(&mut request).await.unwrap();
            assert!(read > 0);
            if stalled {
                std::future::pending::<()>().await;
            } else {
                socket
                    .write_all(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n")
                    .await
                    .unwrap();
                let chunk = " ".repeat(16 * 1024);
                // No Content-Length: the incremental limit must stop this body.
                for _ in 0..17 {
                    if socket
                        .write_all(format!("4000\r\n{chunk}\r\n").as_bytes())
                        .await
                        .is_err()
                    {
                        return;
                    }
                }
                let _ = socket.write_all(b"0\r\n\r\n").await;
            }
        });
        let mut gate = cached_gate();
        gate.client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(3))
            .connect_timeout(Duration::from_secs(2))
            .build()
            .unwrap();
        gate.test_key_url = Some(url);
        let started = Instant::now();
        assert!(
            tokio::time::timeout(Duration::from_secs(5), gate.fetch_keys())
                .await
                .unwrap()
                .is_none()
        );
        if stalled {
            assert!(started.elapsed() >= Duration::from_secs(2));
        }
        task.abort();
    }
}

#[tokio::test]
async fn fetch_rotation_throttle_and_singleflight() {
    let (gate, calls, task) = source(200, fixture().to_string()).await;
    let mut callers = Vec::new();
    for _ in 0..20 {
        let gate = gate.clone();
        callers.push(tokio::spawn(async move {
            gate.allows(&headers(&assertion())).await
        }));
    }
    for caller in callers {
        assert!(caller.await.unwrap());
    }
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(gate.key("unknown").await.is_none());
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    {
        let mut cache = gate.cache.lock().await;
        cache.keys.as_mut().unwrap().keys[0].common.key_id = Some("old-key".into());
        cache.attempted = Some(Instant::now() - REFRESH_INTERVAL);
    }
    assert!(gate.key(KID).await.is_some()); // rotation refresh for new kid
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    assert!(gate.key("old-key").await.is_none());
    {
        let mut cache = gate.cache.lock().await;
        cache.fetched = Some(Instant::now() - CACHE_TTL);
        cache.attempted = Some(Instant::now() - REFRESH_INTERVAL);
    }
    assert!(gate.key(KID).await.is_some()); // expired cache must fetch again
    assert_eq!(calls.load(Ordering::SeqCst), 3);
    task.abort();
}

#[tokio::test]
async fn failed_refresh_never_uses_stale_keys_and_is_throttled() {
    let (gate, calls, task) = source(503, "unavailable".into()).await;
    {
        let mut cache = gate.cache.lock().await;
        cache.keys = Some(jwks());
        cache.fetched = Some(Instant::now() - CACHE_TTL);
    }
    for _ in 0..3 {
        assert!(gate.key(KID).await.is_none());
    }
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    task.abort();
    gate.cache.lock().await.attempted = None;
    assert!(gate.key(KID).await.is_none()); // connection failure also fails closed
}

#[tokio::test]
async fn fetch_rejects_redirects_invalid_json_empty_and_oversized_sets() {
    let mut excessive = fixture();
    excessive["keys"] = Value::Array(vec![excessive["keys"][0].clone(); 33]);
    for (status, body) in [
        (302, fixture().to_string()),
        (500, fixture().to_string()),
        (200, "not json".into()),
        (200, json!({"keys":[]}).to_string()),
        (200, excessive.to_string()),
        (200, " ".repeat(MAX_KEY_BYTES + 1)),
    ] {
        let (gate, calls, task) = source(status, body).await;
        assert!(gate.key(KID).await.is_none());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        task.abort();
    }
}
