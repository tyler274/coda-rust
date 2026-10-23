use axum::{
    Router,
    body::{Body, to_bytes},
    extract::Request,
    http::{StatusCode, header},
    response::IntoResponse,
    routing::any,
};
use public_site::{
    Config, prerender,
    proxy::ProxyConfig,
    runtime::RuntimeConfig,
    server::{Readiness, router_with_options},
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tower::ServiceExt;

async fn upstream(app: Router) -> (String, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    (origin, handle)
}
fn site(origin: &str) -> Router {
    router_with_options(
        prerender(&Config::default()),
        RuntimeConfig {
            physician_proxy: ProxyConfig::new(origin, "http://127.0.0.1:8791", None, false, false)
                .unwrap(),
            ..Default::default()
        },
        [],
        Readiness::ready(),
    )
}
fn request(path: &str) -> axum::http::request::Builder {
    Request::builder().uri(path)
}

#[tokio::test]
async fn preserves_transport_and_strips_credentials_and_hops() {
    let (origin, handle) = upstream(Router::new().fallback(any(|request: Request| async move {
        assert_eq!(
            request.uri().path_and_query().unwrap().as_str(),
            "/api/messages?escaped=%2F&x=1&x=2"
        );
        assert_eq!(
            request.headers()[header::AUTHORIZATION],
            "Bearer physician-token"
        );
        assert_eq!(
            request.headers()[header::CONTENT_TYPE],
            "application/octet-stream"
        );
        for header in [
            "cookie",
            "proxy-authorization",
            "x-goog-iap-jwt-assertion",
            "x-goog-authenticated-user-email",
            "forwarded",
            "x-forwarded-host",
            "x-real-ip",
            "x-private-hop",
        ] {
            assert!(!request.headers().contains_key(header), "{header}");
        }
        let body = to_bytes(request.into_body(), 100_000).await.unwrap();
        assert_eq!(body.len(), 32_768);
        (
            [
                ("set-cookie", "secret=x"),
                ("connection", "x-hidden"),
                ("x-hidden", "private"),
                ("x-goog-iap-jwt-assertion", "secret"),
                ("cache-control", "public,max-age=600"),
            ],
            body,
        )
    })))
    .await;
    let response = site(&origin)
        .oneshot(
            request("/physician/api/messages?escaped=%2F&x=1&x=2")
                .method("POST")
                .header("origin", "http://127.0.0.1:8791")
                .header("authorization", "Bearer physician-token")
                .header("content-type", "application/octet-stream")
                .header("cookie", "IAP=private")
                .header("proxy-authorization", "Bearer client-proxy")
                .header("x-goog-iap-jwt-assertion", "private")
                .header("x-goog-authenticated-user-email", "private")
                .header("forwarded", "host=evil")
                .header("x-forwarded-host", "evil")
                .header("x-real-ip", "evil")
                .header("connection", "x-private-hop")
                .header("x-private-hop", "private")
                .body(Body::from(vec![42; 32_768]))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()[header::CACHE_CONTROL],
        "private, no-store, max-age=0"
    );
    for header in [
        "set-cookie",
        "connection",
        "x-hidden",
        "x-goog-iap-jwt-assertion",
    ] {
        assert!(!response.headers().contains_key(header));
    }
    assert_eq!(
        to_bytes(response.into_body(), 100_000).await.unwrap().len(),
        32_768
    );
    handle.abort();
}

#[tokio::test]
async fn redirect_rewriting_errors_origin_checks_and_limits() {
    let calls = Arc::new(AtomicUsize::new(0));
    let count = calls.clone();
    let (origin, handle) = upstream(Router::new().fallback(any(move |request: Request| {
        let calls = count.clone();
        async move {
            calls.fetch_add(1, Ordering::SeqCst);
            match request.uri().path() {
                "/safe" => (StatusCode::FOUND, [(header::LOCATION, "/login?q=1")]).into_response(),
                "/unsafe" => (
                    StatusCode::FOUND,
                    [(header::LOCATION, "https://accounts.google.com/login")],
                )
                    .into_response(),
                "/error" => StatusCode::SERVICE_UNAVAILABLE.into_response(),
                _ => StatusCode::OK.into_response(),
            }
        }
    })))
    .await;
    let app = site(&origin);
    let response = app
        .clone()
        .oneshot(request("/physician?x=%2F").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::PERMANENT_REDIRECT);
    assert_eq!(response.headers()[header::LOCATION], "/physician/?x=%2F");
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    for path in ["/app", "/app/", "/app/today", "/physicians/today"] {
        let response = app
            .clone()
            .oneshot(request(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert!(!response.status().is_success());
        assert_eq!(calls.load(Ordering::SeqCst), 0, "unexpected proxy: {path}");
    }
    for origin in [None, Some("https://evil.test"), Some("null")] {
        let mut req = request("/physician/write").method("POST");
        if let Some(origin) = origin {
            req = req.header("origin", origin);
        }
        let response = app
            .clone()
            .oneshot(req.body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let response = app
        .clone()
        .oneshot(
            request("/physician/write")
                .method("POST")
                .header("origin", "http://127.0.0.1:8791")
                .header("content-length", public_site::proxy::REQUEST_LIMIT + 1)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    for (path, status, location) in [
        ("safe", StatusCode::FOUND, Some("/physician/login?q=1")),
        ("unsafe", StatusCode::BAD_GATEWAY, None),
        ("error", StatusCode::SERVICE_UNAVAILABLE, None),
    ] {
        let response = app
            .clone()
            .oneshot(
                request(&format!("/physician/{path}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), status);
        assert_eq!(
            response
                .headers()
                .get(header::LOCATION)
                .map(|v| v.to_str().unwrap()),
            location
        );
        assert_eq!(
            response.headers()[header::CACHE_CONTROL],
            "private, no-store, max-age=0"
        );
    }
    assert_eq!(
        calls.load(Ordering::SeqCst),
        3,
        "redirects must not be followed"
    );
    handle.abort();
    let response = app
        .oneshot(request("/physician/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
}

#[tokio::test]
async fn sse_delivers_first_chunk_while_upstream_body_is_still_open() {
    // A stream whose second event cannot be emitted until the test releases it.
    let (release, wait) = tokio::sync::oneshot::channel::<()>();
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let upstream_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin = format!("http://{}", upstream_listener.local_addr().unwrap());
    let handle = tokio::spawn(async move {
        let (mut stream, _) = upstream_listener.accept().await.unwrap();
        let mut request = [0u8; 4096];
        let bytes_read = stream.read(&mut request).await.unwrap();
        assert!(bytes_read > 0);
        stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\n\r\nd\r\ndata: first\n\n\r\n").await.unwrap();
        wait.await.unwrap();
        stream
            .write_all(b"e\r\ndata: second\n\n\r\n0\r\n\r\n")
            .await
            .unwrap();
    });
    let app = site(&origin);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/physician/events", listener.local_addr().unwrap());
    let proxy = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    let mut response = reqwest::get(url).await.unwrap();
    assert_eq!(
        response.headers()[header::CONTENT_TYPE],
        "text/event-stream"
    );
    let first = tokio::time::timeout(Duration::from_secs(1), response.chunk())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(first, "data: first\n\n");
    release.send(()).unwrap();
    assert_eq!(response.chunk().await.unwrap().unwrap(), "data: second\n\n");
    assert!(response.chunk().await.unwrap().is_none());
    proxy.abort();
    handle.abort();
}
