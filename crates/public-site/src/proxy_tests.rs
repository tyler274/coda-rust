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
    let proxy = Proxy {
        mount: Mount::Physician,
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
    let proxy = Proxy {
        mount: Mount::Physician,
        config: ProxyConfig::new(
            "http://127.0.0.1:8790",
            "http://127.0.0.1:8791",
            None,
            false,
            false,
        )
        .unwrap(),
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

#[test]
fn mounts_own_only_their_path_family() {
    for (mount, inside, outside) in [
        (Mount::Physician, "/physician/x", "/physicians"),
        (Mount::Grafana, "/grafana/d/abc", "/grafanax"),
        (Mount::Langfuse, "/langfuse/api/auth", "/langfuse-old/"),
    ] {
        assert!(mount.owns(mount.prefix()));
        assert!(mount.owns(&format!("{}/", mount.prefix())));
        assert!(mount.owns(inside));
        assert!(!mount.owns(outside));
        assert!(!mount.owns("/"));
    }
}

#[test]
fn mounted_location_stays_inside_its_prefix_on_known_origins() {
    let config = ProxyConfig::new(
        "https://relay.test",
        "https://staging.codahealth.ai",
        None,
        true,
        false,
    )
    .unwrap();
    let target = Url::parse("https://relay.test/langfuse/auth/sign-in").unwrap();
    for (value, expected) in [
        ("/langfuse/project/1?x=%2F#t", "/langfuse/project/1?x=%2F#t"),
        ("next", "/langfuse/auth/next"),
        ("https://relay.test/langfuse/", "/langfuse/"),
        (
            "https://staging.codahealth.ai/langfuse/api/auth/callback",
            "/langfuse/api/auth/callback",
        ),
        ("/langfuse", "/langfuse"),
    ] {
        assert_eq!(
            mounted_location(&value.parse().unwrap(), &target, &config, "/langfuse").unwrap(),
            expected,
            "{value}"
        );
    }
    for value in [
        "/",
        "/grafana/",
        "/langfuse-other/",
        "../../physician/",
        "https://accounts.google.com/o/oauth2",
        "https://staging.codahealth.ai/physician/",
        "//evil.test/langfuse/",
        "https://user@relay.test/langfuse/",
        "javascript:alert(1)",
    ] {
        assert!(
            mounted_location(&value.parse().unwrap(), &target, &config, "/langfuse").is_none(),
            "{value}"
        );
    }
}

#[test]
fn cookie_allowlist_is_per_mount_and_in_both_directions() {
    let mut request = HeaderMap::new();
    request.append(
        header::COOKIE,
        "grafana_session=g; __Secure-next-auth.session-token=s"
            .parse()
            .unwrap(),
    );
    request.append(
        header::COOKIE,
        "__Host-next-auth.csrf-token=c; next-auth.session-token=plain; IAP=x"
            .parse()
            .unwrap(),
    );
    assert!(request_cookie(&request, Mount::Physician).is_none());
    assert!(request_cookie(&request, Mount::Grafana).is_none());
    assert_eq!(
        request_cookie(&request, Mount::Langfuse).unwrap(),
        "__Secure-next-auth.session-token=s; __Host-next-auth.csrf-token=c"
    );
    let mut response = HeaderMap::new();
    for value in [
        "__Secure-next-auth.session-token=s; Path=/langfuse; Secure; HttpOnly",
        "__Host-next-auth.csrf-token=c; Path=/; Secure",
        "__Secure-next-auth.callback-url=u; Domain=codahealth.ai; Secure",
        "grafana_session=g; Path=/grafana",
        "next-auth.session-token=plain",
    ] {
        response.append(header::SET_COOKIE, value.parse().unwrap());
    }
    assert!(response_cookies(&response, Mount::Grafana).is_empty());
    assert!(response_cookies(&response, Mount::Physician).is_empty());
    let kept: Vec<_> = response_cookies(&response, Mount::Langfuse)
        .into_iter()
        .map(|value| value.to_str().unwrap().to_owned())
        .collect();
    assert_eq!(
        kept,
        [
            "__Secure-next-auth.session-token=s; Path=/langfuse; Secure; HttpOnly",
            "__Host-next-auth.csrf-token=c; Path=/; Secure",
        ]
    );
}

#[tokio::test]
async fn observability_mounts_keep_prefix_drop_spoofed_assertion_and_refuse_off_origin_redirects() {
    let app = Router::new().fallback(|request: Request| async move {
        let path = request.uri().path().to_owned();
        assert!(!request.headers().contains_key(FORWARDED_ASSERTION));
        match path.as_str() {
            "/grafana/login" => (
                StatusCode::FOUND,
                [(header::LOCATION, "https://accounts.google.com/")],
            )
                .into_response(),
            "/grafana/home" => {
                (StatusCode::FOUND, [(header::LOCATION, "/grafana/d/1")]).into_response()
            }
            // Next.js with a base path and no trailing slash.
            "/langfuse/" => (
                StatusCode::PERMANENT_REDIRECT,
                [(header::LOCATION, "/langfuse")],
            )
                .into_response(),
            _ => (
                [(
                    header::SET_COOKIE,
                    "__Secure-next-auth.session-token=s; Path=/langfuse; Secure",
                )],
                path,
            )
                .into_response(),
        }
    });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    let relay =
        Proxy::new(ProxyConfig::new(&origin, "http://127.0.0.1:8791", None, false, false).unwrap());
    let send = |mount: Mount, uri: &str| {
        let proxy = relay.mount(mount);
        let request = Request::builder()
            .uri(uri)
            // No verified assertion arrives, so the client copy must not pass.
            .header(FORWARDED_ASSERTION, "client-spoof")
            .body(Body::empty())
            .unwrap();
        async move { proxy.serve(request).await }
    };
    let response = send(Mount::Langfuse, "/langfuse/api/public/health?x=1").await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()[header::SET_COOKIE],
        "__Secure-next-auth.session-token=s; Path=/langfuse; Secure"
    );
    let body = axum::body::to_bytes(response.into_body(), 1024)
        .await
        .unwrap();
    assert_eq!(body, "/langfuse/api/public/health");
    // The bare prefix reaches the app unchanged; the app owns its root's
    // trailing-slash rule, so the proxy adds no redirect that could loop.
    let response = send(Mount::Langfuse, "/langfuse?next=1").await;
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), 1024)
        .await
        .unwrap();
    assert_eq!(body, "/langfuse");
    let response = send(Mount::Langfuse, "/langfuse/").await;
    assert_eq!(response.status(), StatusCode::PERMANENT_REDIRECT);
    assert_eq!(response.headers()[header::LOCATION], "/langfuse");
    let response = send(Mount::Grafana, "/grafana").await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        send(Mount::Langfuse, "/langfusex").await.status(),
        StatusCode::NOT_FOUND
    );
    let response = send(Mount::Grafana, "/grafana/home").await;
    assert_eq!(response.headers()[header::LOCATION], "/grafana/d/1");
    assert_eq!(
        send(Mount::Grafana, "/grafana/login").await.status(),
        StatusCode::BAD_GATEWAY
    );
    assert_eq!(
        send(Mount::Grafana, "/langfuse/").await.status(),
        StatusCode::NOT_FOUND
    );
    server.abort();
}
