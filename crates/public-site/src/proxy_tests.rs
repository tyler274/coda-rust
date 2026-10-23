use super::*;
use axum::{
    Router,
    extract::State,
    routing::{get, post},
};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[test]
fn fixed_origins_reject_credentials_and_nonlocal_plaintext() {
    for url in [
        "http://example.test",
        "https://user:pass@example.test",
        "https://example.test/physician",
        "https://example.test/?x=y",
        "https://example.test/#x",
        "//example.test",
        "file:///tmp/x",
    ] {
        assert!(validate_origin(url, false).is_err(), "{url}");
    }
    assert!(validate_origin("http://127.0.0.1:8790", false).is_ok());
    assert!(validate_origin("http://[::1]:8790", false).is_ok());
    assert!(validate_origin("http://127.0.0.1:8790", true).is_err());
    assert!(
        ProxyConfig::new(
            "https://physician.test",
            "https://staging.codahealth.ai",
            None,
            true,
            true
        )
        .is_err()
    );
    for email in [
        "",
        "a@evil.test",
        "a/b@project.iam.gserviceaccount.com",
        "a@x.iam.gserviceaccount.com?x",
    ] {
        assert!(
            ProxyConfig::new(
                "https://physician.test",
                "https://staging.codahealth.ai",
                Some(email.into()),
                true,
                true
            )
            .is_err()
        );
    }
}

#[test]
fn location_policy_never_exposes_an_upstream_origin() {
    let origin = Url::parse("https://physician.test").unwrap();
    let target = origin.join("/folder/start").unwrap();
    for (value, expected) in [
        ("/login?x=%2F#done", "/physician/login?x=%2F#done"),
        ("next", "/physician/folder/next"),
        ("https://physician.test/login", "/physician/login"),
    ] {
        assert_eq!(
            rewrite_location(&value.parse().unwrap(), &target, &origin).unwrap(),
            expected
        );
    }
    for value in [
        "https://accounts.google.com/login",
        "//evil.test",
        "https://user@physician.test/",
        "javascript:alert(1)",
    ] {
        assert!(rewrite_location(&value.parse().unwrap(), &target, &origin).is_none());
    }
}

#[derive(Clone)]
struct Mock {
    metadata: Arc<AtomicUsize>,
    signing: Arc<AtomicUsize>,
    fail: Arc<AtomicBool>,
}

#[tokio::test]
async fn jwt_claims_cache_serialization_and_refresh_failure_are_closed() {
    let mock = Mock {
        metadata: Arc::new(AtomicUsize::new(0)),
        signing: Arc::new(AtomicUsize::new(0)),
        fail: Arc::new(AtomicBool::new(false)),
    };
    let app = Router::new()
        .route(
            "/upstream",
            get(|headers: HeaderMap| async move {
                assert_eq!(headers[header::AUTHORIZATION], "Bearer physician-session");
                assert_eq!(
                    headers[header::PROXY_AUTHORIZATION],
                    "Bearer signed.payload.signature"
                );
                assert!(!headers.contains_key("x-goog-iap-jwt-assertion"));
                "ok"
            }),
        )
        .route(
            "/metadata",
            get(|State(mock): State<Mock>, headers: HeaderMap| async move {
                assert_eq!(headers["metadata-flavor"], "Google");
                mock.metadata.fetch_add(1, Ordering::SeqCst);
                axum::Json(serde_json::json!({"access_token": "metadata-access-token"}))
            }),
        )
        .route(
            "/sign",
            post(
                |State(mock): State<Mock>,
                 headers: HeaderMap,
                 axum::Json(body): axum::Json<serde_json::Value>| async move {
                    mock.signing.fetch_add(1, Ordering::SeqCst);
                    assert_eq!(
                        headers[header::AUTHORIZATION],
                        "Bearer metadata-access-token"
                    );
                    let claims: serde_json::Value =
                        serde_json::from_str(body["payload"].as_str().unwrap()).unwrap();
                    assert_eq!(claims["iss"], "public-site@project.iam.gserviceaccount.com");
                    assert_eq!(claims["sub"], claims["iss"]);
                    assert_eq!(claims["aud"], "https://physician.test/*");
                    assert_eq!(
                        claims["exp"].as_u64().unwrap() - claims["iat"].as_u64().unwrap(),
                        3600
                    );
                    if mock.fail.load(Ordering::SeqCst) {
                        return StatusCode::FORBIDDEN.into_response();
                    }
                    axum::Json(serde_json::json!({"signedJwt": "signed.payload.signature"}))
                        .into_response()
                },
            ),
        )
        .with_state(mock.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    let credentials = Arc::new(Credentials {
        client: Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap(),
        email: "public-site@project.iam.gserviceaccount.com".into(),
        audience: "https://physician.test/*".into(),
        cache: Mutex::new(None),
        metadata_url: format!("{origin}/metadata"),
        signing_url: format!("{origin}/sign"),
    });
    let mut requests = Vec::new();
    for _ in 0..10 {
        let credentials = credentials.clone();
        requests.push(tokio::spawn(
            async move { credentials.token().await.unwrap() },
        ));
    }
    for request in requests {
        assert_eq!(request.await.unwrap(), "Bearer signed.payload.signature");
    }
    assert_eq!(mock.metadata.load(Ordering::SeqCst), 1);
    assert_eq!(mock.signing.load(Ordering::SeqCst), 1);
    let proxy = PhysicianProxy {
        config: ProxyConfig::new(&origin, "http://127.0.0.1:8791", None, false, false).unwrap(),
        client: Client::new(),
        credentials: Some(credentials.clone()),
    };
    let response = proxy
        .serve(
            Request::builder()
                .uri("/physician/upstream")
                .header(header::AUTHORIZATION, "Bearer physician-session")
                .header(header::PROXY_AUTHORIZATION, "Bearer attacker")
                .header("x-goog-iap-jwt-assertion", "outer-assertion")
                .body(Body::empty())
                .unwrap(),
        )
        .await;
    assert_eq!(response.status(), StatusCode::OK);
    credentials.cache.lock().await.as_mut().unwrap().expires = 1;
    mock.fail.store(true, Ordering::SeqCst);
    assert!(credentials.token().await.is_err());
    assert!(credentials.cache.lock().await.is_none());
    // A physician request must not reach any target after signing fails.
    let proxy = PhysicianProxy {
        config: ProxyConfig::default(),
        client: Client::new(),
        credentials: Some(credentials),
    };
    let response = proxy
        .serve(
            Request::builder()
                .uri("/physician/")
                .body(Body::empty())
                .unwrap(),
        )
        .await;
    assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
    server.abort();
}

#[tokio::test]
async fn credential_responses_are_size_bounded_and_never_follow_redirects() {
    let app = Router::new()
        .route("/huge", get(|| async { "x".repeat(CREDENTIAL_LIMIT + 1) }))
        .route(
            "/redirect",
            get(|| async { (StatusCode::FOUND, [(header::LOCATION, "/valid")]) }),
        )
        .route(
            "/valid",
            get(|| async { axum::Json(serde_json::json!({"ok": true})) }),
        );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    for path in ["huge", "redirect"] {
        assert!(
            bounded_json(client.get(format!("{origin}/{path}")))
                .await
                .is_err()
        );
    }
    server.abort();
}
