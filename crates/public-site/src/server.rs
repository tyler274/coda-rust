//! Stateless public serving boundary. Every body/encoding is prepared at startup.
use crate::{
    Config, PageTable,
    access::{AccessGate, secret_matches},
    assets::AssetTable,
    prerender,
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

pub fn router(config: Config) -> Router {
    router_with_pages(prerender(&config))
}

/// Compatibility entry point still honors environment staging protection.
pub fn router_with_pages(pages: PageTable) -> Router {
    router_with_options(
        pages,
        RuntimeConfig::from_env().expect("invalid public-site runtime configuration"),
        [],
        Readiness::ready(),
    )
}

/// Explicit settings and generated asset injection for embedding/tests/motion.
pub fn router_with_options(
    pages: PageTable,
    runtime: RuntimeConfig,
    extra_assets: impl IntoIterator<Item = (String, Bytes)>,
    ready: Readiness,
) -> Router {
    assert!(runtime.max_concurrency > 0, "concurrency must be positive");
    let generated = crate::paint::assets()
        .into_iter()
        .map(|asset| (asset.path, Bytes::from(asset.bytes)));
    let paint_css = (
        "/css/paint.css".to_owned(),
        Bytes::from(crate::paint::stylesheet(str::to_owned)),
    );
    let assets = AssetTable::embedded_with(generated.chain([paint_css]).chain(extra_assets));
    let document = |page: crate::Page| Document {
        representation: Representation::new(
            assets.rewrite(std::str::from_utf8(&page.html).expect("UTF-8 rendered page")),
            "text/html; charset=utf-8",
        ),
        noindex: page.meta.noindex,
    };
    let mut locations: Vec<_> = pages
        .pages
        .values()
        .filter(|page| !page.meta.noindex)
        .map(|page| page.meta.canonical_url.clone())
        .collect();
    locations.sort();
    locations.dedup();
    let sitemap = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">{}</urlset>",
        locations
            .iter()
            .map(|url| format!("<url><loc>{}</loc></url>", xml_escape(url)))
            .collect::<String>()
    );
    let origin = reqwest::Url::parse(crate::site::SITE_ORIGIN)
        .expect("site origin")
        .origin()
        .ascii_serialization();
    let robots = if runtime.staging {
        "User-agent: *\nDisallow: /\n".to_owned()
    } else {
        format!(
            "User-agent: *\nAllow: /\nDisallow: /scheduling/dev\nSitemap: {origin}/sitemap.xml\n"
        )
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
        gate: AccessGate::new(runtime.access.clone()),
        permits: Arc::new(Semaphore::new(runtime.max_concurrency)),
        runtime,
        ready,
    });
    Router::new()
        .fallback(serve)
        .with_state(state.clone())
        .layer(RequestBodyLimitLayer::new(16 * 1024))
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

async fn boundary(State(state): State<Arc<SiteState>>, request: Request, next: Next) -> Response {
    let head = request.method() == Method::HEAD;
    let mut response = if let Ok(_permit) = state.permits.clone().try_acquire_owned() {
        match tokio::time::timeout(state.runtime.request_timeout, async {
            if state.runtime.staging && !state.gate.allows(request.headers()).await {
                return (StatusCode::FORBIDDEN, "Cloudflare Access required").into_response();
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
                service_client_id: Some("id".into()),
                service_client_secret: Some("secret".into()),
                ..Default::default()
            },
            ..Default::default()
        }
    }
    fn request(path: &str, authorized: bool) -> Request {
        let mut request = Request::builder().uri(path);
        if authorized {
            request = request
                .header("cf-access-client-id", "id")
                .header("cf-access-client-secret", "secret");
        }
        request.body(Body::empty()).unwrap()
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
    async fn sitemap_excludes_noindex_and_head_assets_are_empty() {
        let mut table = pages();
        let mut private = table.not_found.clone();
        private.meta.noindex = true;
        private.meta.canonical_url = "https://codahealth.ai/scheduling/dev".into();
        table.pages.insert("/scheduling/dev".into(), private);
        let app = router_with_options(table, RuntimeConfig::default(), [], Readiness::ready());
        let response = app
            .clone()
            .oneshot(request("/sitemap.xml", false))
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), 10000)
            .await
            .unwrap();
        let body = std::str::from_utf8(&body).unwrap();
        assert!(body.contains("<loc>https://codahealth.ai</loc>"));
        assert!(!body.contains("scheduling/dev"));
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
            gate: AccessGate::new(runtime.access.clone()),
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
