//! Transport-only proxy mounts: `/physician/`, and in staging `/grafana/` and
//! `/langfuse/` through the observability relay. No identities or application
//! state live here.
#[cfg(test)]
#[path = "proxy_tests.rs"]
mod tests;
use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode, header},
    response::{IntoResponse, Response},
};
use reqwest::{Client, Url};
use std::{
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::sync::Mutex;

pub const REQUEST_LIMIT: usize = 8 * 1024 * 1024;
const METADATA_TOKEN: &str =
    "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
const IAM_BASE: &str = "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/";
const CREDENTIAL_LIMIT: usize = 32 * 1024;

/// Fixed startup values; never derived from client headers or request parameters.
#[derive(Clone)]
pub struct ProxyConfig {
    pub upstream: Url,
    pub public_origin: String,
    pub service_account: Option<String>,
}
impl ProxyConfig {
    pub fn new(
        upstream: &str,
        public_origin: &str,
        service_account: Option<String>,
        deployed: bool,
        staging: bool,
    ) -> Result<Self, &'static str> {
        let upstream = validate_origin(upstream, deployed)?;
        let public_origin = validate_origin(public_origin, deployed)?
            .origin()
            .ascii_serialization();
        if staging && service_account.is_none() {
            return Err("PHYSICIAN_PROXY_SERVICE_ACCOUNT is required in staging");
        }
        if let Some(email) = &service_account {
            let Some((name, domain)) = email.split_once('@') else {
                return Err("invalid PHYSICIAN_PROXY_SERVICE_ACCOUNT");
            };
            if name.is_empty()
                || domain.contains('@')
                || !domain.ends_with(".iam.gserviceaccount.com")
                || !email
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"-._@".contains(&b))
            {
                return Err("invalid PHYSICIAN_PROXY_SERVICE_ACCOUNT");
            }
        }
        Ok(Self {
            upstream,
            public_origin,
            service_account,
        })
    }
}

pub fn validate_origin(value: &str, deployed: bool) -> Result<Url, &'static str> {
    let url = Url::parse(value).map_err(|_| "proxy URL must be an absolute origin")?;
    let loopback = url.host_str().is_some_and(|host| {
        host == "localhost"
            || host
                .trim_matches(['[', ']'])
                .parse::<std::net::IpAddr>()
                .is_ok_and(|ip| ip.is_loopback())
    });
    if value.trim() != value
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.path() != "/"
        || url.host_str().is_none()
        || !(url.scheme() == "https" || (!deployed && loopback && url.scheme() == "http"))
    {
        return Err("proxy URL must be a credential-free HTTPS origin (local loopback HTTP only)");
    }
    Ok(url)
}

/// One site path family and how it maps onto its upstream. Each mount's
/// policy is fixed here; nothing about it comes from configuration or clients.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mount {
    /// Physician product: the prefix is removed and no cookies cross.
    Physician,
    /// Staging Grafana through the observability relay. JWT auth reads the
    /// forwarded assertion on every request, so no cookies cross.
    Grafana,
    /// Staging Langfuse through the observability relay. Only NextAuth's
    /// cookies cross. Under [`LangfuseSignIn::Sso`], page navigations without
    /// a session start its SSO sign-in.
    Langfuse,
    /// The authorize step of `langfuse-sso`, the issuer that turns the
    /// forwarded assertion into Langfuse's sign-in. No cookies cross.
    LangfuseSso,
}
/// How staging Langfuse signs people in, which decides what public-site
/// mounts for it. It must match the `observability` root's
/// `langfuse_sso_image`: only with the issuer deployed does Langfuse offer
/// the `custom` provider that [`Self::Sso`] sends people to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LangfuseSignIn {
    /// Langfuse's own login form, passed through unchanged.
    #[default]
    Password,
    /// `langfuse-sso` from the IAP identity: the issuer's authorize step is
    /// mounted, and page navigations without a session start SSO.
    Sso,
}

impl Mount {
    /// The staging mounts behind the observability relay.
    pub fn observability(sign_in: LangfuseSignIn) -> &'static [Self] {
        match sign_in {
            LangfuseSignIn::Password => &[Self::Grafana, Self::Langfuse],
            LangfuseSignIn::Sso => &[Self::Grafana, Self::Langfuse, Self::LangfuseSso],
        }
    }

    /// The site path, without a trailing slash.
    pub fn prefix(self) -> &'static str {
        match self {
            Self::Physician => "/physician",
            Self::Grafana => "/grafana",
            Self::Langfuse => "/langfuse",
            Self::LangfuseSso => "/langfuse-sso",
        }
    }
    /// True when `path` is this mount's root or anything below it.
    pub fn owns(self, path: &str) -> bool {
        path.strip_prefix(self.prefix())
            .is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
    }
    /// The physician upstream serves at its root. The observability apps are
    /// configured with their site prefix, so the path passes through unchanged.
    fn strips_prefix(self) -> bool {
        self == Self::Physician
    }
    /// Sends the site's verified IAP assertion as `X-Coda-IAP-Assertion`.
    fn forwards_assertion(self) -> bool {
        self != Self::Physician
    }
    fn allows_cookie(self, name: &str) -> bool {
        self == Self::Langfuse
            && (name.starts_with("__Secure-next-auth.") || name == "__Host-next-auth.csrf-token")
    }
    /// Browsers reach only the issuer's authorize step; Langfuse's server
    /// calls the rest inside the cluster.
    fn serves(self, path: &str) -> bool {
        self != Self::LangfuseSso || path == LANGFUSE_SSO_AUTHORIZE
    }
    /// Where this mount's redirects may point, besides its own prefix.
    fn allows_redirect(self, path: &str) -> bool {
        self == Self::LangfuseSso && path == LANGFUSE_CALLBACK
    }
}

const LANGFUSE_SSO_AUTHORIZE: &str = "/langfuse-sso/authorize";
const LANGFUSE_CALLBACK: &str = "/langfuse/api/auth/callback/custom";
const LANGFUSE_SSO_INITIATE: &str = "/langfuse/auth/sso-initiate";
const LANGFUSE_SIGN_IN: &str = "/langfuse/auth/sign-in";

/// Sends a Langfuse page navigation into SSO, or out of it once signed in.
///
/// Langfuse's `sso-initiate` page signs in with itself as the callback URL,
/// so it would sign in again forever. Every initiate URL therefore carries
/// the page the user asked for as `target`, and a navigation to it with a
/// session cookie goes there instead. A sign-in page with an `error` stays,
/// so a failing sign-in cannot loop.
fn langfuse_navigation(
    headers: &HeaderMap,
    method: &Method,
    uri: &axum::http::Uri,
) -> Option<String> {
    if *method != Method::GET
        || headers
            .get("sec-fetch-mode")
            .is_none_or(|mode| mode != "navigate")
    {
        return None;
    }
    let signed_in = request_cookie(headers, Mount::Langfuse).is_some_and(|cookie| {
        cookie.to_str().is_ok_and(|cookie| {
            cookie
                .split("; ")
                .any(|pair| pair.starts_with("__Secure-next-auth.session-token"))
        })
    });
    let path = uri.path();
    let query = || {
        url::form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes())
            .map(|(name, value)| (name.into_owned(), value.into_owned()))
    };
    let page = |target: Option<String>| {
        target
            .filter(|target| langfuse_page(target))
            .unwrap_or_else(|| "/langfuse/".into())
    };
    let initiate = |target: String| {
        let query = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("provider", "custom")
            .append_pair("target", &target)
            .finish();
        format!("{LANGFUSE_SSO_INITIATE}?{query}")
    };
    let param = |wanted: &str| {
        query()
            .find(|(name, _)| name == wanted)
            .map(|(_, value)| value)
    };
    if path == LANGFUSE_SSO_INITIATE {
        return signed_in.then(|| page(param("target")));
    }
    if path == LANGFUSE_SIGN_IN {
        return param("error")
            .is_none()
            .then(|| initiate(page(param("callbackUrl"))));
    }
    if signed_in || path.starts_with("/langfuse/api/") || path.starts_with("/langfuse/auth/") {
        return None;
    }
    Some(initiate(page(
        uri.path_and_query()
            .map(|target| target.as_str().to_owned()),
    )))
}

/// A same-site Langfuse page path that cannot leave the mount.
fn langfuse_page(target: &str) -> bool {
    Mount::Langfuse.owns(target.split(['?', '#']).next().unwrap_or_default())
        && !target.contains(['\\', '\r', '\n'])
        && !target.contains("//")
        && !target.contains("/../")
}

/// The verified user assertion, re-sent to the observability apps. Clients can
/// never set it: every inbound and upstream copy is removed first.
pub const FORWARDED_ASSERTION: &str = "x-coda-iap-assertion";

#[derive(Clone)]
pub(crate) struct Proxy {
    mount: Mount,
    config: ProxyConfig,
    client: Client,
    credentials: Option<Arc<Credentials>>,
    langfuse_sign_in: LangfuseSignIn,
}
impl Proxy {
    pub(crate) fn new(config: ProxyConfig) -> Self {
        // Disable decompression even when other workspace crates enable reqwest features.
        let client = Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .no_gzip()
            .no_brotli()
            .no_deflate()
            .no_zstd()
            .connect_timeout(Duration::from_secs(2))
            .build()
            .expect("proxy HTTP client");
        let credentials = config.service_account.as_ref().map(|email| {
            Arc::new(Credentials {
                client: client.clone(),
                email: email.clone(),
                audience: format!("{}/*", config.upstream.origin().ascii_serialization()),
                cache: Mutex::new(None),
                metadata_url: METADATA_TOKEN.into(),
                signing_url: format!("{IAM_BASE}{email}:signJwt"),
            })
        });
        Self {
            mount: Mount::Physician,
            config,
            client,
            credentials,
            langfuse_sign_in: LangfuseSignIn::default(),
        }
    }

    /// The same upstream and signer, serving another mount.
    pub(crate) fn mount(&self, mount: Mount) -> Self {
        Self {
            mount,
            ..self.clone()
        }
    }

    /// The same proxy, with the Langfuse mount signing people in this way.
    pub(crate) fn langfuse_sign_in(self, langfuse_sign_in: LangfuseSignIn) -> Self {
        Self {
            langfuse_sign_in,
            ..self
        }
    }

    pub(crate) fn prefix(&self) -> &'static str {
        self.mount.prefix()
    }

    pub(crate) async fn serve(&self, request: Request) -> Response {
        if request.method() == Method::CONNECT || request.method() == Method::TRACE {
            return failure(StatusCode::METHOD_NOT_ALLOWED);
        }
        if !matches!(
            *request.method(),
            Method::GET | Method::HEAD | Method::OPTIONS
        ) && (request.headers().get_all(header::ORIGIN).iter().count() != 1
            || request
                .headers()
                .get(header::ORIGIN)
                .and_then(|v| v.to_str().ok())
                != Some(self.config.public_origin.as_str())
            || request
                .headers()
                .get("sec-fetch-site")
                .is_some_and(|v| v == "cross-site"))
        {
            return failure(StatusCode::FORBIDDEN);
        }
        let prefix = self.mount.prefix();
        if !self.mount.serves(request.uri().path()) {
            return failure(StatusCode::NOT_FOUND);
        }
        if self.mount == Mount::Langfuse
            && self.langfuse_sign_in == LangfuseSignIn::Sso
            && let Some(location) =
                langfuse_navigation(request.headers(), request.method(), request.uri())
        {
            return (StatusCode::SEE_OTHER, [(header::LOCATION, location)]).into_response();
        }
        // Only the stripping mount picks the canonical root. The others pass
        // the bare prefix through, because each app has its own trailing-slash
        // rule (Next.js redirects `/langfuse/` to `/langfuse`, Grafana the
        // reverse) and a redirect here would loop against it.
        if self.mount.strips_prefix() && request.uri().path() == prefix {
            let location = match request.uri().query() {
                Some(query) => format!("{prefix}/?{query}"),
                None => format!("{prefix}/"),
            };
            return (
                StatusCode::PERMANENT_REDIRECT,
                [(header::LOCATION, location)],
            )
                .into_response();
        }
        let Some(path_and_query) = request.uri().path_and_query().map(|p| p.as_str()) else {
            return failure(StatusCode::NOT_FOUND);
        };
        let path = if self.mount.strips_prefix() {
            let Some(below) = path_and_query
                .strip_prefix(prefix)
                .and_then(|rest| rest.strip_prefix('/'))
            else {
                return failure(StatusCode::NOT_FOUND);
            };
            below.to_owned()
        } else {
            let Some(rest) = path_and_query
                .strip_prefix(prefix)
                .filter(|rest| rest.is_empty() || rest.starts_with('/') || rest.starts_with('?'))
            else {
                return failure(StatusCode::NOT_FOUND);
            };
            format!("{}{rest}", &prefix[1..])
        };
        // Concatenate, never Url::join an untrusted path (which permits authority replacement).
        let Ok(target) = Url::parse(&format!(
            "{}/{path}",
            self.config.upstream.origin().ascii_serialization()
        )) else {
            return failure(StatusCode::BAD_REQUEST);
        };
        if target.origin() != self.config.upstream.origin() {
            return failure(StatusCode::BAD_REQUEST);
        }
        let (parts, body) = request.into_parts();
        let mut headers = parts.headers;
        // Read before sanitizing: the boundary has already verified it in staging.
        let assertion = self
            .mount
            .forwards_assertion()
            .then(|| http::iap::single_header(&headers, http::iap::ASSERTION_HEADER))
            .flatten()
            .and_then(|value| HeaderValue::from_str(value).ok());
        let cookie = request_cookie(&headers, self.mount);
        sanitize(&mut headers);
        if let Some(mut assertion) = assertion {
            assertion.set_sensitive(true);
            headers.insert(FORWARDED_ASSERTION, assertion);
        }
        if let Some(cookie) = cookie {
            headers.insert(header::COOKIE, cookie);
        }
        if let Some(credentials) = &self.credentials {
            let Ok(token) = credentials.token().await else {
                return failure(StatusCode::BAD_GATEWAY);
            };
            headers.insert(header::PROXY_AUTHORIZATION, token);
        }
        let outgoing = self
            .client
            .request(parts.method, target.clone())
            .headers(headers)
            .body(reqwest::Body::wrap_stream(body.into_data_stream()));
        // The server boundary owns the response deadline, including credentials
        // and upstream startup. Do not impose a second, shorter deadline here.
        let mut upstream = match outgoing.send().await {
            Ok(response) => response,
            Err(_) => return failure(StatusCode::BAD_GATEWAY),
        };
        let status = upstream.status();
        let mut headers = std::mem::take(upstream.headers_mut());
        if headers.get_all(header::LOCATION).iter().count() > 1 {
            return failure(StatusCode::BAD_GATEWAY);
        }
        if let Some(location) = headers.get(header::LOCATION) {
            let location = if self.mount.strips_prefix() {
                rewrite_location(location, &target, &self.config.upstream)
            } else {
                mounted_location(location, &target, &self.config, self.mount)
            };
            let Some(location) = location else {
                return failure(StatusCode::BAD_GATEWAY);
            };
            headers.insert(header::LOCATION, location);
        }
        let set_cookies = response_cookies(&headers, self.mount);
        sanitize(&mut headers);
        for cookie in set_cookies {
            headers.append(header::SET_COOKIE, cookie);
        }
        // Refresh can navigate without Location and Link can disclose the private origin.
        headers.remove("refresh");
        headers.remove(header::LINK);
        let mut response = Response::new(Body::from_stream(upstream.bytes_stream()));
        *response.status_mut() = status;
        *response.headers_mut() = headers;
        response
    }
}

fn failure(status: StatusCode) -> Response {
    (status, "proxy request failed").into_response()
}

fn rewrite_location(value: &HeaderValue, target: &Url, origin: &Url) -> Option<HeaderValue> {
    let resolved = target.join(value.to_str().ok()?).ok()?;
    if resolved.origin() != origin.origin()
        || !resolved.username().is_empty()
        || resolved.password().is_some()
    {
        return None;
    }
    let mut location = format!("/physician{}", resolved.path());
    push_query_and_fragment(&mut location, &resolved);
    HeaderValue::from_str(&location).ok()
}

/// A mount that keeps its prefix may redirect only within that prefix, or to
/// the one path its policy allows, on the upstream or the public origin. The
/// browser sees a path on this site.
fn mounted_location(
    value: &HeaderValue,
    target: &Url,
    config: &ProxyConfig,
    mount: Mount,
) -> Option<HeaderValue> {
    let resolved = target.join(value.to_str().ok()?).ok()?;
    let origin = resolved.origin().ascii_serialization();
    if (resolved.origin() != config.upstream.origin() && origin != config.public_origin)
        || !resolved.username().is_empty()
        || resolved.password().is_some()
        || !(mount.owns(resolved.path()) || mount.allows_redirect(resolved.path()))
    {
        return None;
    }
    let mut location = resolved.path().to_owned();
    push_query_and_fragment(&mut location, &resolved);
    HeaderValue::from_str(&location).ok()
}

fn push_query_and_fragment(location: &mut String, resolved: &Url) {
    if let Some(query) = resolved.query() {
        location.push('?');
        location.push_str(query);
    }
    if let Some(fragment) = resolved.fragment() {
        location.push('#');
        location.push_str(fragment);
    }
}

/// One `Cookie` header with only the mount's allowlisted pairs, or none.
fn request_cookie(headers: &HeaderMap, mount: Mount) -> Option<HeaderValue> {
    let kept: Vec<&str> = headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(';'))
        .map(str::trim)
        .filter(|pair| {
            pair.split_once('=')
                .is_some_and(|(name, _)| mount.allows_cookie(name.trim()))
        })
        .collect();
    if kept.is_empty() {
        return None;
    }
    let mut value = HeaderValue::from_str(&kept.join("; ")).ok()?;
    value.set_sensitive(true);
    Some(value)
}

/// Allowlisted `Set-Cookie` values. A `Domain` attribute could widen the
/// cookie beyond this host, so those are dropped too.
fn response_cookies(headers: &HeaderMap, mount: Mount) -> Vec<HeaderValue> {
    headers
        .get_all(header::SET_COOKIE)
        .iter()
        .filter(|value| {
            let Ok(text) = value.to_str() else {
                return false;
            };
            let mut attributes = text.split(';');
            let named = attributes
                .next()
                .and_then(|pair| pair.split_once('='))
                .is_some_and(|(name, _)| mount.allows_cookie(name.trim()));
            named
                && !attributes.any(|attribute| {
                    attribute
                        .trim()
                        .get(..6)
                        .is_some_and(|name| name.eq_ignore_ascii_case("domain"))
                })
        })
        .cloned()
        .collect()
}

/// Applied symmetrically, including every Connection-nominated extension header.
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
                    | "cookie"
                    | "cookie2"
                    | "set-cookie"
                    | "set-cookie2"
                    | "forwarded"
                    | "via"
                    | "x-real-ip"
                    | "x-coda-staging-token"
                    | FORWARDED_ASSERTION
            ) || name.starts_with("x-goog-")
                || name.starts_with("x-google-")
                || name.starts_with("x-serverless-")
                || name.starts_with("x-forwarded-")
                || name.starts_with("proxy-")
                || name.starts_with("cf-access-")
        })
        .cloned()
        .collect();
    for name in remove {
        headers.remove(name);
    }
}

struct CachedJwt {
    token: HeaderValue,
    expires: u64,
}
struct Credentials {
    client: Client,
    email: String,
    audience: String,
    cache: Mutex<Option<CachedJwt>>,
    // Fixed production endpoints; fields permit isolated unit tests, not runtime overrides.
    metadata_url: String,
    signing_url: String,
}
impl Credentials {
    async fn token(&self) -> Result<HeaderValue, ()> {
        let mut cache = self.cache.lock().await;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| ())?
            .as_secs();
        if let Some(cached) = cache.as_ref().filter(|cached| cached.expires > now + 120) {
            return Ok(cached.token.clone());
        }
        // Invalidate before refreshing: failed refresh never falls back to an old credential.
        *cache = None;
        let token = bounded_json(
            self.client
                .get(&self.metadata_url)
                .header("Metadata-Flavor", "Google"),
        )
        .await?;
        let access_token = token
            .get("access_token")
            .and_then(|v| v.as_str())
            .filter(|v| !v.is_empty())
            .ok_or(())?;
        let expires = now + 3600;
        let payload = serde_json::json!({ "iss": self.email, "sub": self.email, "aud": self.audience, "iat": now, "exp": expires }).to_string();
        let signed = bounded_json(
            self.client
                .post(&self.signing_url)
                .bearer_auth(access_token)
                .json(&serde_json::json!({ "payload": payload })),
        )
        .await?;
        let jwt = signed
            .get("signedJwt")
            .and_then(|v| v.as_str())
            .filter(|v| !v.is_empty() && v.split('.').count() == 3)
            .ok_or(())?;
        let mut token = HeaderValue::from_str(&format!("Bearer {jwt}")).map_err(|_| ())?;
        token.set_sensitive(true);
        *cache = Some(CachedJwt {
            token: token.clone(),
            expires,
        });
        Ok(token)
    }
}
async fn bounded_json(request: reqwest::RequestBuilder) -> Result<serde_json::Value, ()> {
    tokio::time::timeout(Duration::from_secs(3), async {
        let mut response = request.send().await.map_err(|_| ())?;
        if !response.status().is_success()
            || response
                .content_length()
                .is_some_and(|n| n > CREDENTIAL_LIMIT as u64)
        {
            return Err(());
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|_| ())? {
            if bytes.len() + chunk.len() > CREDENTIAL_LIMIT {
                return Err(());
            }
            bytes.extend_from_slice(&chunk);
        }
        serde_json::from_slice(&bytes).map_err(|_| ())
    })
    .await
    .map_err(|_| ())?
}
