//! Staging-only hop from public-site to the private Grafana load balancer.
//! Cloud Run IAP admits only public-site's service account and the
//! engineering group; this service checks that IAP assertion again, forwards
//! the fixed `/grafana` path family unchanged and knows nothing about the app.
//! Book: `book/src/gcp.md`.
use axum::{
    Router,
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, HeaderName, Method, StatusCode, header},
    response::{IntoResponse, Response},
};
use http::iap::{IapVerifier, validate_cloud_run_audience};
use std::{sync::Arc, time::Duration};
use tower_http::limit::RequestBodyLimitLayer;
use url::Url;

/// The same request body ceiling as public-site's proxy mounts.
pub const REQUEST_LIMIT: usize = 8 * 1024 * 1024;

/// Fixed at startup from the environment; nothing here comes from requests.
#[derive(Clone, Debug)]
pub struct Config {
    /// This service's own Cloud Run IAP audience.
    pub audience: String,
    /// Internal Grafana origin, which serves under `/grafana/`.
    pub grafana: Url,
}

impl Config {
    /// The upstream that owns `path`, if any. The path family is fixed.
    fn upstream_for(&self, path: &str) -> Option<&Url> {
        owns(path, "/grafana").then_some(&self.grafana)
    }
}

/// Why the relay refused its startup configuration. `setting` names the
/// environment variable.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("{setting} is required")]
    Missing { setting: &'static str },
    #[error("{setting} must be a Cloud Run IAP audience: {reason}")]
    Audience {
        setting: &'static str,
        reason: &'static str,
    },
    #[error("{setting} must be an absolute origin")]
    Unparsable {
        setting: &'static str,
        #[source]
        source: url::ParseError,
    },
    #[error("{setting} must be a credential-free http(s) origin with no path, query or fragment")]
    NotOrigin { setting: &'static str },
}

impl Config {
    /// Reads `RELAY_IAP_AUDIENCE` and `GRAFANA_UPSTREAM`.
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_lookup(|name| std::env::var(name).ok())
    }

    /// Both values are required; a missing or invalid one stops startup.
    pub fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let required = |setting: &'static str| {
            lookup(setting)
                .filter(|value| !value.trim().is_empty())
                .ok_or(ConfigError::Missing { setting })
        };
        let setting = "RELAY_IAP_AUDIENCE";
        let audience = required(setting)?;
        validate_cloud_run_audience(&audience)
            .map_err(|reason| ConfigError::Audience { setting, reason })?;
        Ok(Self {
            audience,
            grafana: upstream("GRAFANA_UPSTREAM", &required("GRAFANA_UPSTREAM")?)?,
        })
    }
}

/// An origin only: no credentials, path, query or fragment. Plain HTTP is
/// expected, because the hop is a private address inside the VPC.
fn upstream(setting: &'static str, value: &str) -> Result<Url, ConfigError> {
    let url = Url::parse(value).map_err(|source| ConfigError::Unparsable { setting, source })?;
    if value.trim() != value
        || !matches!(url.scheme(), "http" | "https")
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.path() != "/"
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(ConfigError::NotOrigin { setting });
    }
    Ok(url)
}

#[derive(Clone)]
struct Relay {
    config: Arc<Config>,
    verifier: IapVerifier,
    client: reqwest::Client,
}

/// The relay's whole HTTP surface.
pub fn router(config: Config) -> Router {
    let verifier = IapVerifier::new(Some(config.audience.clone()));
    router_with_verifier(config, verifier)
}

/// The router with an injected verifier, for tests with fixture keys.
pub fn router_with_verifier(config: Config, verifier: IapVerifier) -> Router {
    // No redirects, no decompression and no connection reuse across a proxy.
    let client = reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(Duration::from_secs(2))
        .build()
        .expect("relay HTTP client");
    let relay = Relay {
        config: Arc::new(config),
        verifier,
        client,
    };
    Router::new()
        .fallback(relay_request)
        .layer(RequestBodyLimitLayer::new(REQUEST_LIMIT))
        .with_state(relay)
}

fn failure(status: StatusCode) -> Response {
    (status, "relay request failed").into_response()
}

fn owns(path: &str, prefix: &str) -> bool {
    path.strip_prefix(prefix)
        .is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
}

async fn relay_request(State(relay): State<Relay>, request: Request) -> Response {
    // Defense in depth: Cloud Run IAP already admitted this request.
    if !relay.verifier.allows(request.headers()).await {
        return failure(StatusCode::FORBIDDEN);
    }
    if request.method() == Method::CONNECT || request.method() == Method::TRACE {
        return failure(StatusCode::METHOD_NOT_ALLOWED);
    }
    let Some(origin) = relay.config.upstream_for(request.uri().path()) else {
        return failure(StatusCode::NOT_FOUND);
    };
    let Some(path_and_query) = request.uri().path_and_query() else {
        return failure(StatusCode::NOT_FOUND);
    };
    // Concatenate, never Url::join an untrusted path (which permits authority replacement).
    let Ok(target) = Url::parse(&format!(
        "{}{}",
        origin.origin().ascii_serialization(),
        path_and_query.as_str()
    )) else {
        return failure(StatusCode::BAD_REQUEST);
    };
    if target.origin() != origin.origin() {
        return failure(StatusCode::BAD_REQUEST);
    }
    let (parts, body) = request.into_parts();
    let mut headers = parts.headers;
    sanitize(&mut headers);
    let outgoing = relay
        .client
        .request(parts.method, target)
        .headers(headers)
        .body(reqwest::Body::wrap_stream(body.into_data_stream()));
    let mut upstream = match outgoing.send().await {
        Ok(response) => response,
        Err(_) => return failure(StatusCode::BAD_GATEWAY),
    };
    let status = upstream.status();
    let mut headers = std::mem::take(upstream.headers_mut());
    sanitize(&mut headers);
    let mut response = Response::new(Body::from_stream(upstream.bytes_stream()));
    *response.status_mut() = status;
    *response.headers_mut() = headers;
    response
}

/// Hop-by-hop headers and Google's front-end headers never cross. Everything
/// else, including the forwarded user assertion, passes unchanged:
/// public-site owns that policy.
fn sanitize(headers: &mut HeaderMap) {
    let nominated: Vec<HeaderName> = headers
        .get_all(header::CONNECTION)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(','))
        .filter_map(|v| HeaderName::from_bytes(v.trim().as_bytes()).ok())
        .collect();
    for name in nominated {
        headers.remove(name);
    }
    let remove: Vec<HeaderName> = headers
        .keys()
        .filter(|name| {
            let name = name.as_str();
            matches!(
                name,
                "host"
                    | "connection"
                    | "keep-alive"
                    | "proxy-authenticate"
                    | "proxy-authorization"
                    | "te"
                    | "trailer"
                    | "transfer-encoding"
                    | "upgrade"
                    | "forwarded"
                    | "via"
                    | "x-real-ip"
            ) || name.starts_with("x-goog-")
                || name.starts_with("x-google-")
                || name.starts_with("x-serverless-")
                || name.starts_with("x-forwarded-")
                || name.starts_with("proxy-")
        })
        .cloned()
        .collect();
    for name in remove {
        headers.remove(name);
    }
}
