//! Stateless public serving boundary. Every body/encoding is prepared at startup.
use crate::{
    PageTable,
    access::{AccessGate, secret_matches},
    assets::AssetTable,
    representation::{HTML_CACHE, IMMUTABLE_CACHE, Representation},
    runtime::RuntimeConfig,
};
use axum::{
    Router,
    body::{Body, Bytes},
    extract::{Request, State},
    http::{HeaderMap, Method, StatusCode, Uri, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::sync::Semaphore;
use tower_http::limit::RequestBodyLimitLayer;

#[derive(Clone, Default)]
pub struct Readiness(Arc<AtomicBool>);
impl Readiness {
    pub fn ready() -> Self {
        Self(Arc::new(AtomicBool::new(true)))
    }
    pub fn stop(&self) {
        self.0.store(false, Ordering::Release);
    }
    pub fn is_ready(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
}
struct Document {
    representation: Representation,
    noindex: bool,
}
struct SiteState {
    pages: HashMap<String, Document>,
    not_found: Document,
    assets: AssetTable,
    sitemap: Representation,
    robots: Representation,
    health: Representation,
    ready: Readiness,
    runtime: RuntimeConfig,
    gate: AccessGate,
    permits: Arc<Semaphore>,
}

/// Explicit settings and generated asset injection for embedding/tests/motion.
pub fn router_with_options(
    pages: PageTable,
    runtime: RuntimeConfig,
    extra_assets: impl IntoIterator<Item = (String, Bytes)>,
    ready: Readiness,
) -> Router {
    let gate = AccessGate::new(runtime.access.clone());
    router_with_gate(pages, runtime, extra_assets, ready, gate)
}

fn router_with_gate(
    pages: PageTable,
    runtime: RuntimeConfig,
    extra_assets: impl IntoIterator<Item = (String, Bytes)>,
    ready: Readiness,
    gate: AccessGate,
) -> Router {
    assert!(runtime.max_concurrency > 0, "concurrency must be positive");
    if runtime.staging {
        runtime
            .access
            .validate()
            .expect("invalid staging IAP configuration");
    }
    let assets = AssetTable::embedded_with(extra_assets);
    let document = |page: crate::Page| Document {
        representation: Representation::new(
            assets.rewrite(std::str::from_utf8(&page.html).expect("UTF-8 rendered page")),
            "text/html; charset=utf-8",
        ),
        noindex: page.meta.noindex,
    };
    // Each location resolves its path against the origin, as the original's sitemap
    // does, so the homepage is listed with its slash.
    let sitemap = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">{}</urlset>",
        crate::Route::sitemap()
            .iter()
            .map(|route| format!(
                "<url><loc>{}</loc></url>",
                xml_escape(&format!("{}{route}", crate::site::SITE_ORIGIN))
            ))
            .collect::<String>()
    );
    let origin = reqwest::Url::parse(crate::site::SITE_ORIGIN)
        .expect("site origin")
        .origin()
        .ascii_serialization();
    // The original's app/robots.ts as Next serializes it: allow all, with host and sitemap.
    let robots = if runtime.staging {
        "User-agent: *\nDisallow: /\n".to_owned()
    } else {
        format!("User-Agent: *\nAllow: /\n\nHost: {origin}\nSitemap: {origin}/sitemap.xml\n")
    };
    let health = serde_json::to_vec(&serde_json::json!({"ok": true, "app": "coda-health-public", "env": runtime.environment, "release_sha": runtime.release_sha})).unwrap();
    let state = Arc::new(SiteState {
        pages: pages
            .pages
            .into_iter()
            .map(|(path, page)| (path, document(page)))
            .collect(),
        not_found: document(pages.not_found),
        assets,
        sitemap: Representation::new(sitemap, "application/xml; charset=utf-8"),
        robots: Representation::new(robots, "text/plain; charset=utf-8"),
        health: Representation::new(health, "application/json"),
        gate,
        permits: Arc::new(Semaphore::new(runtime.max_concurrency)),
        runtime,
        ready,
    });
    let mut app = Router::new()
        .fallback(serve)
        .layer(RequestBodyLimitLayer::new(16 * 1024));
    // Without a physician upstream there is no mount: /physician/ is an ordinary 404.
    if let Some(config) = state.runtime.physician_proxy.clone() {
        app = app.merge(
            Router::new()
                .route("/physician", axum::routing::any(serve_physician))
                .route("/physician/", axum::routing::any(serve_physician))
                .route("/physician/{*path}", axum::routing::any(serve_physician))
                .layer(RequestBodyLimitLayer::new(crate::proxy::REQUEST_LIMIT))
                .with_state(crate::proxy::PhysicianProxy::new(config)),
        );
    }
    app.with_state(state.clone())
        // Outermost middleware gates even health, assets, rejected methods and
        // body-limit failures, and forcibly removes every public cache policy.
        .layer(middleware::from_fn_with_state(state, boundary))
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

async fn serve_physician(
    State(proxy): State<crate::proxy::PhysicianProxy>,
    request: Request,
) -> Response {
    proxy.serve(request).await
}

async fn boundary(State(state): State<Arc<SiteState>>, request: Request, next: Next) -> Response {
    let physician = state.runtime.physician_proxy.is_some()
        && (request.uri().path() == "/physician"
            || request.uri().path().starts_with("/physician/"));
    // This is the single deadline for response headers. Physician requests can
    // include a Cloud Run cold start and mail delivery; the static-page budget
    // must not cut off their acknowledgement. Streaming bodies stay unbounded.
    let response_timeout = if physician {
        std::time::Duration::from_secs(60)
    } else {
        state.runtime.request_timeout
    };
    let head = request.method() == Method::HEAD;
    let mut response = if let Ok(_permit) = state.permits.clone().try_acquire_owned() {
        match tokio::time::timeout(response_timeout, async {
            if state.runtime.staging && !state.gate.allows(request.headers()).await {
                return (StatusCode::FORBIDDEN, "Google IAP assertion required").into_response();
            }
            next.run(request).await
        })
        .await
        {
            Ok(response) => response,
            Err(_) => (StatusCode::GATEWAY_TIMEOUT, "request timed out").into_response(),
        }
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            [(header::RETRY_AFTER, "1")],
            "server busy",
        )
            .into_response()
    };
    if response.status().is_client_error() || response.status().is_server_error() {
        response
            .headers_mut()
            .insert(header::CACHE_CONTROL, "no-store".parse().unwrap());
    }
    response
        .headers_mut()
        .insert("x-content-type-options", "nosniff".parse().unwrap());
    if state.runtime.staging {
        let headers = response.headers_mut();
        headers.insert("x-robots-tag", "noindex, nofollow".parse().unwrap());
        headers.insert(
            header::CACHE_CONTROL,
            "private, no-store, max-age=0".parse().unwrap(),
        );
        headers.insert("cdn-cache-control", "no-store".parse().unwrap());
        headers.insert("cloudflare-cdn-cache-control", "no-store".parse().unwrap());
        headers.insert("surrogate-control", "no-store".parse().unwrap());
        // Vary:* additionally forbids shared-cache reuse across credentials.
        // Deployment must still disable forced CDN caching on the staging host;
        // origin headers cannot defend against an edge configured to ignore them.
        headers.insert(header::VARY, "*".parse().unwrap());
    }
    if physician {
        let headers = response.headers_mut();
        headers.insert(
            header::CACHE_CONTROL,
            "private, no-store, max-age=0".parse().unwrap(),
        );
        headers.insert("cdn-cache-control", "no-store".parse().unwrap());
        headers.insert("cloudflare-cdn-cache-control", "no-store".parse().unwrap());
        headers.insert("surrogate-control", "no-store".parse().unwrap());
        headers.insert(header::VARY, "*".parse().unwrap());
    }
    if head {
        *response.body_mut() = Body::empty();
    }
    response
}

async fn serve(
    State(state): State<Arc<SiteState>>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
) -> Response {
    if method != Method::GET && method != Method::HEAD {
        return (
            StatusCode::METHOD_NOT_ALLOWED,
            [(header::ALLOW, "GET, HEAD")],
        )
            .into_response();
    }
    // A trailing slash is removed with a permanent redirect that keeps the query,
    // before any route or asset is matched. The target must be one same-origin
    // path: a second leading slash, a backslash, or a scheme is not a redirect.
    if let Some(path) = uri.path().strip_suffix('/').filter(|path| !path.is_empty())
        && same_origin_path(path)
    {
        let location = match uri.query() {
            Some(query) => format!("{path}?{query}"),
            None => path.to_owned(),
        };
        return (
            StatusCode::PERMANENT_REDIRECT,
            [(header::LOCATION, location)],
        )
            .into_response();
    }
    match uri.path() {
        "/api/health" => {
            if state.runtime.staging
                && state
                    .runtime
                    .health_token
                    .as_ref()
                    .is_some_and(|token| !secret_matches(&headers, "x-coda-staging-token", token))
            {
                return (
                    StatusCode::UNAUTHORIZED,
                    axum::Json(serde_json::json!({"error": "health token required"})),
                )
                    .into_response();
            }
            return state
                .health
                .respond(&headers, &method, StatusCode::OK, "no-store");
        }
        "/healthz" => return probe(true, &method),
        "/readyz" => return probe(state.ready.is_ready(), &method),
        "/sitemap.xml" => {
            return state
                .sitemap
                .respond(&headers, &method, StatusCode::OK, HTML_CACHE);
        }
        "/robots.txt" => {
            return state
                .robots
                .respond(&headers, &method, StatusCode::OK, HTML_CACHE);
        }
        _ => {}
    }
    if let Some(asset) = state.assets.assets.get(uri.path()) {
        return asset.representation.respond(
            &headers,
            &method,
            StatusCode::OK,
            if asset.immutable {
                IMMUTABLE_CACHE
            } else {
                HTML_CACHE
            },
        );
    }
    let (status, page) = match state.pages.get(uri.path()) {
        Some(page) => (StatusCode::OK, page),
        None => (StatusCode::NOT_FOUND, &state.not_found),
    };
    let mut response = page.representation.respond(
        &headers,
        &method,
        status,
        if status == StatusCode::OK {
            HTML_CACHE
        } else {
            "no-store"
        },
    );
    if page.noindex {
        response
            .headers_mut()
            .insert("x-robots-tag", "noindex, nofollow".parse().unwrap());
    }
    response
}

fn probe(ready: bool, method: &Method) -> Response {
    let text = if ready { "ok\n" } else { "not ready\n" };
    Response::builder()
        .status(if ready {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        })
        .header(header::CACHE_CONTROL, "no-store")
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .header(header::CONTENT_LENGTH, text.len())
        .body(if *method == Method::HEAD {
            Body::empty()
        } else {
            Body::from(text)
        })
        .unwrap()
}

/// A redirect target is one same-origin path. A second leading slash, a
/// backslash, or a scheme would leave this origin.
fn same_origin_path(path: &str) -> bool {
    path.starts_with('/')
        && !path.starts_with("//")
        && !path.contains('\\')
        && !path.contains("://")
}

/// Only the binary's optional loopback listener mounts these endpoints. Public
/// probes are always subject to Access; no spoofable header/IP bypass exists.
pub fn internal_probe_router(ready: Readiness) -> Router {
    Router::new()
        .route(
            "/healthz",
            axum::routing::get(|method: Method| async move { probe(true, &method) }),
        )
        .route(
            "/readyz",
            axum::routing::get(move |method: Method| {
                let ready = ready.clone();
                async move { probe(ready.is_ready(), &method) }
            }),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower::ServiceExt;
    // Route tests use the same signature verifier with a public local key cache.
    fn router_with_options(
        pages: PageTable,
        runtime: RuntimeConfig,
        assets: impl IntoIterator<Item = (String, Bytes)>,
        ready: Readiness,
    ) -> Router {
        super::router_with_gate(
            pages,
            runtime,
            assets,
            ready,
            crate::access::tests::cached_gate(),
        )
    }
    fn pages() -> PageTable {
        let meta = crate::Route::Home {}.meta();
        let page = crate::Page {
            html: Bytes::from_static(b"<!DOCTYPE html><img src=\"/favicon.ico\">"),
            meta,
        };
        PageTable {
            pages: [("/".into(), page.clone())].into_iter().collect(),
            not_found: page,
        }
    }
    fn staged() -> RuntimeConfig {
        RuntimeConfig {
            staging: true,
            environment: "staging".into(),
            access: crate::access::AccessConfig {
                audience: Some(crate::access::tests::AUDIENCE.into()),
            },
            ..Default::default()
        }
    }
    fn request(path: &str, authorized: bool) -> Request {
        let mut request = Request::builder().uri(path);
        if authorized {
            request = request.header(
                "x-goog-iap-jwt-assertion",
                crate::access::tests::assertion(),
            );
        }
        request.body(Body::empty()).unwrap()
    }
    #[tokio::test]
    async fn trailing_slash_redirect_stays_on_this_origin() {
        let app = router_with_options(pages(), RuntimeConfig::default(), [], Readiness::ready());
        let pricing = app
            .clone()
            .oneshot(request("/pricing/?plan=1", false))
            .await
            .unwrap();
        assert_eq!(pricing.status(), StatusCode::PERMANENT_REDIRECT);
        assert_eq!(pricing.headers()["location"], "/pricing?plan=1");
        assert!(same_origin_path("/pricing"));
        assert!(same_origin_path("/duet/"));
        assert!(!same_origin_path("//example.test"));
        assert!(!same_origin_path("/\\\\example.test"));
        assert!(!same_origin_path("/http://example.test"));

        let off = app
            .oneshot(request("//example.test/", false))
            .await
            .unwrap();
        let location = off
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");
        assert!(
            !location.contains("example.test"),
            "off-origin location: {location}"
        );
    }
    #[tokio::test]
    async fn staging_gates_every_public_resource_and_never_shared_caches() {
        let assets = AssetTable::embedded_with([]);
        let hashed = assets.aliases["/favicon.ico"].clone();
        let app = router_with_options(pages(), staged(), [], Readiness::ready());
        for path in [
            "/",
            "/api/health",
            "/healthz",
            "/readyz",
            "/favicon.ico",
            "/robots.txt",
            "/sitemap.xml",
            "/missing",
            &hashed,
        ] {
            let denied = app.clone().oneshot(request(path, false)).await.unwrap();
            assert_eq!(denied.status(), StatusCode::FORBIDDEN, "{path}");
            let response = app.clone().oneshot(request(path, true)).await.unwrap();
            assert_eq!(
                response.status(),
                if path == "/missing" {
                    StatusCode::NOT_FOUND
                } else {
                    StatusCode::OK
                },
                "{path}"
            );
            for response in [response, denied] {
                assert_eq!(
                    response.headers()[header::CACHE_CONTROL],
                    "private, no-store, max-age=0"
                );
                assert_eq!(
                    response.headers()["cloudflare-cdn-cache-control"],
                    "no-store"
                );
                assert_eq!(response.headers()[header::VARY], "*");
                assert_eq!(response.headers()["x-robots-tag"], "noindex, nofollow");
            }
        }
    }
    #[tokio::test]
    async fn physician_proxy_preserves_access_and_waits_for_upstream_responses() {
        use std::sync::atomic::AtomicUsize;
        let calls = Arc::new(AtomicUsize::new(0));
        let observed = calls.clone();
        let upstream = Router::new().fallback(move |request: Request| {
            let calls = observed.clone();
            async move {
                calls.fetch_add(1, Ordering::SeqCst);
                if request.uri().path() == "/auth/sign-in" {
                    assert_eq!(request.method(), Method::POST);
                    let body = axum::body::to_bytes(request.into_body(), 1024)
                        .await
                        .unwrap();
                    assert_eq!(body, r#"{"email":"doctor@example.test"}"#);
                    // An upstream cold start and mail delivery can outlast the
                    // static site's deadline. Its acknowledgement must survive.
                    tokio::time::sleep(std::time::Duration::from_millis(250)).await;
                    return axum::Json(serde_json::json!({"status": "accepted"})).into_response();
                }
                "upstream".into_response()
            }
        });
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let origin = format!("http://{}", listener.local_addr().unwrap());
        let task = tokio::spawn(async move {
            axum::serve(listener, upstream).await.unwrap();
        });
        let mut runtime = staged();
        runtime.request_timeout = std::time::Duration::from_millis(100);
        runtime.physician_proxy = Some(
            crate::proxy::ProxyConfig::new(&origin, "http://127.0.0.1:8791", None, false, false)
                .unwrap(),
        );
        let app = router_with_options(pages(), runtime, [], Readiness::ready());
        for path in ["/physician", "/physician/", "/physician/api/messages"] {
            assert_eq!(
                app.clone()
                    .oneshot(request(path, false))
                    .await
                    .unwrap()
                    .status(),
                StatusCode::FORBIDDEN
            );
        }
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert_eq!(
            app.clone()
                .oneshot(request("/physician", true))
                .await
                .unwrap()
                .status(),
            StatusCode::PERMANENT_REDIRECT
        );
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert_eq!(
            app.clone()
                .oneshot(request("/physician/", true))
                .await
                .unwrap()
                .status(),
            StatusCode::OK
        );
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        let mut sign_in = request("/physician/auth/sign-in", true);
        *sign_in.method_mut() = Method::POST;
        sign_in
            .headers_mut()
            .insert(header::ORIGIN, "http://127.0.0.1:8791".parse().unwrap());
        *sign_in.body_mut() = Body::from(r#"{"email":"doctor@example.test"}"#);
        let response = app.oneshot(sign_in).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers()[header::CACHE_CONTROL],
            "private, no-store, max-age=0"
        );
        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&body).unwrap()["status"],
            "accepted"
        );
        assert_eq!(
            calls.load(Ordering::SeqCst),
            2,
            "the proxy must not retry a POST"
        );
        task.abort();
    }

    #[tokio::test]
    async fn health_requires_access_and_optional_health_token() {
        let mut runtime = staged();
        runtime.health_token = Some("probe".into());
        let app = router_with_options(pages(), runtime, [], Readiness::ready());
        let mut only_health = request("/api/health", false);
        only_health
            .headers_mut()
            .insert("x-coda-staging-token", "probe".parse().unwrap());
        assert_eq!(
            app.clone().oneshot(only_health).await.unwrap().status(),
            StatusCode::FORBIDDEN
        );
        assert_eq!(
            app.clone()
                .oneshot(request("/api/health", true))
                .await
                .unwrap()
                .status(),
            StatusCode::UNAUTHORIZED
        );
        let mut both = request("/api/health", true);
        both.headers_mut()
            .insert("x-coda-staging-token", "probe".parse().unwrap());
        assert_eq!(app.oneshot(both).await.unwrap().status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn rewritten_assets_cache_and_readiness() {
        let ready = Readiness::ready();
        let app = router_with_options(pages(), RuntimeConfig::default(), [], ready.clone());
        let response = app.clone().oneshot(request("/", false)).await.unwrap();
        let body = axum::body::to_bytes(response.into_body(), 10000)
            .await
            .unwrap();
        assert!(std::str::from_utf8(&body).unwrap().contains("/a/"));
        let assets = AssetTable::embedded_with([]);
        let hashed = &assets.aliases["/favicon.ico"];
        let response = app.clone().oneshot(request(hashed, false)).await.unwrap();
        assert_eq!(response.headers()[header::CACHE_CONTROL], IMMUTABLE_CACHE);
        let response = app
            .clone()
            .oneshot(request("/favicon.ico", false))
            .await
            .unwrap();
        assert_eq!(response.headers()[header::CACHE_CONTROL], HTML_CACHE);
        ready.stop();
        assert_eq!(
            app.clone()
                .oneshot(request("/readyz", false))
                .await
                .unwrap()
                .status(),
            StatusCode::SERVICE_UNAVAILABLE
        );
        assert_eq!(
            app.clone()
                .oneshot(request("/healthz", false))
                .await
                .unwrap()
                .status(),
            StatusCode::OK
        );
        let response = app.oneshot(request("/api/health", false)).await.unwrap();
        let body = axum::body::to_bytes(response.into_body(), 10000)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["app"], "coda-health-public");
        assert!(body["release_sha"].is_null());
    }
    #[tokio::test]
    async fn head_assets_are_empty() {
        let app = router_with_options(pages(), RuntimeConfig::default(), [], Readiness::ready());
        let mut req = request("/favicon.ico", false);
        *req.method_mut() = Method::HEAD;
        req.headers_mut()
            .insert(header::ACCEPT_ENCODING, "br".parse().unwrap());
        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()[header::CONTENT_ENCODING], "br");
        assert!(
            response.headers()[header::CONTENT_LENGTH]
                .to_str()
                .unwrap()
                .parse::<usize>()
                .unwrap()
                > 0
        );
        assert!(
            axum::body::to_bytes(response.into_body(), 1000)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn overload_and_timeout_are_bounded_and_private() {
        let runtime = RuntimeConfig {
            request_timeout: std::time::Duration::from_millis(10),
            max_concurrency: 1,
            ..staged()
        };
        let state = Arc::new(SiteState {
            pages: HashMap::new(),
            not_found: Document {
                representation: Representation::new("missing", "text/plain"),
                noindex: true,
            },
            assets: AssetTable::default(),
            sitemap: Representation::new("", "text/plain"),
            robots: Representation::new("", "text/plain"),
            health: Representation::new("", "text/plain"),
            ready: Readiness::ready(),
            gate: crate::access::tests::cached_gate(),
            permits: Arc::new(Semaphore::new(1)),
            runtime,
        });
        let app = Router::new()
            .fallback(|| async { std::future::pending::<Response>().await })
            .layer(middleware::from_fn_with_state(state.clone(), boundary));
        let permit = state.permits.acquire().await.unwrap();
        let response = app.clone().oneshot(request("/", true)).await.unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(response.headers()[header::RETRY_AFTER], "1");
        assert_eq!(
            response.headers()[header::CACHE_CONTROL],
            "private, no-store, max-age=0"
        );
        drop(permit);
        let response = app.oneshot(request("/", true)).await.unwrap();
        assert_eq!(response.status(), StatusCode::GATEWAY_TIMEOUT);
        assert_eq!(
            response.headers()[header::CACHE_CONTROL],
            "private, no-store, max-age=0"
        );
        assert_eq!(state.permits.available_permits(), 1);
    }

    #[tokio::test]
    async fn no_methods_or_conditional_requests_bypass_staging() {
        let app = router_with_options(pages(), staged(), [], Readiness::ready());
        let mut req = request("/", false);
        *req.method_mut() = Method::HEAD;
        req.headers_mut()
            .insert(header::IF_NONE_MATCH, "*".parse().unwrap());
        let response = app.clone().oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        assert!(
            axum::body::to_bytes(response.into_body(), 1000)
                .await
                .unwrap()
                .is_empty()
        );
        let mut req = request("/", true);
        *req.method_mut() = Method::POST;
        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(
            response.headers()[header::CACHE_CONTROL],
            "private, no-store, max-age=0"
        );
    }
}
