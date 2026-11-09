//! Staging-only OpenID Connect issuer that signs Langfuse in as the Google
//! account public-site's IAP already admitted. Langfuse's generic OIDC
//! provider discovers this issuer, sends the browser to `authorize` through
//! public-site (which forwards the verified IAP assertion as
//! `x-coda-iap-assertion`), and redeems the code at `token` over the private
//! network. Assertions, codes, secrets and tokens are never logged.
use axum::{
    Json, Router,
    body::Bytes,
    extract::{DefaultBodyLimit, RawQuery, State},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use base64::Engine;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use http::iap::{IapIdentity, IapVerifier, single_header, validate_cloud_run_audience};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use ring::{
    digest::{SHA256, digest},
    rand::{SecureRandom, SystemRandom},
    signature::{ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair, KeyPair},
};
use serde::Serialize;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, HashMap, hash_map::Entry},
    sync::{Arc, Mutex, PoisonError},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use subtle::ConstantTimeEq;
use url::{Url, form_urlencoded};

/// The header public-site sets to the IAP assertion it verified. Never
/// `x-goog-iap-jwt-assertion`: a request that skipped public-site carries that.
pub const ASSERTION_HEADER: &str = "x-coda-iap-assertion";
/// How long an authorization code stays redeemable.
pub const CODE_LIFETIME: Duration = Duration::from_secs(60);
/// Outstanding codes beyond this are refused until some expire or are redeemed.
pub const MAX_GRANTS: usize = 1024;
/// Lifetime of the ID and access tokens, in seconds.
pub const TOKEN_LIFETIME: u64 = 300;
const TOKEN_BODY_LIMIT: usize = 16 * 1024;
const MIN_SECRET_BYTES: usize = 32;
const MAX_STATE_BYTES: usize = 1024;

/// Fixed at startup from the environment; nothing here comes from requests.
#[derive(Clone)]
pub struct Config {
    issuer: Url,
    authorize_url: Url,
    client_id: String,
    client_secret: String,
    redirect_uri: Url,
    signing_key: SigningKey,
    iap_audience: String,
    code_lifetime: Duration,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("issuer", &self.issuer.as_str())
            .field("authorize_url", &self.authorize_url.as_str())
            .field("client_id", &self.client_id)
            .field("client_secret", &"<redacted>")
            .field("redirect_uri", &self.redirect_uri.as_str())
            .field("signing_key", &self.signing_key.kid)
            .field("iap_audience", &self.iap_audience)
            .field("code_lifetime", &self.code_lifetime)
            .finish()
    }
}

/// Why the issuer refused its startup configuration. `setting` names the
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
    #[error("{setting} must be an absolute URL")]
    Unparsable {
        setting: &'static str,
        #[source]
        source: url::ParseError,
    },
    #[error("{setting} {reason}")]
    Url {
        setting: &'static str,
        reason: &'static str,
    },
    #[error("{setting} must be at least {MIN_SECRET_BYTES} bytes")]
    ShortSecret { setting: &'static str },
    #[error("{setting} must be standard base64")]
    KeyEncoding {
        setting: &'static str,
        #[source]
        source: base64::DecodeError,
    },
    #[error("{setting} must be a PKCS#8 DER P-256 private key")]
    Key {
        setting: &'static str,
        #[source]
        source: ring::error::KeyRejected,
    },
}

impl Config {
    /// Reads the `LANGFUSE_SSO_*` settings.
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_lookup(|name| std::env::var(name).ok())
    }

    /// Every value is required; a missing or malformed one stops startup.
    pub fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let required = |setting: &'static str| {
            lookup(setting)
                .filter(|value| !value.trim().is_empty())
                .ok_or(ConfigError::Missing { setting })
        };
        let setting = "LANGFUSE_SSO_ISSUER";
        let issuer = url(setting, &required(setting)?)?;
        if issuer.path() == "/" || issuer.path().ends_with('/') {
            return Err(ConfigError::Url {
                setting,
                reason: "must have a path without a trailing slash",
            });
        }
        let setting = "LANGFUSE_SSO_AUTHORIZE_URL";
        let authorize_url = url(setting, &required(setting)?)?;
        let setting = "LANGFUSE_SSO_CLIENT_ID";
        let client_id = required(setting)?;
        let setting = "LANGFUSE_SSO_CLIENT_SECRET";
        let client_secret = required(setting)?;
        if client_secret.len() < MIN_SECRET_BYTES {
            return Err(ConfigError::ShortSecret { setting });
        }
        let setting = "LANGFUSE_SSO_REDIRECT_URI";
        let redirect_uri = url(setting, &required(setting)?)?;
        let setting = "LANGFUSE_SSO_SIGNING_KEY";
        let signing_key = SigningKey::from_base64(setting, &required(setting)?)?;
        let setting = "LANGFUSE_SSO_IAP_AUDIENCE";
        let iap_audience = required(setting)?;
        validate_cloud_run_audience(&iap_audience)
            .map_err(|reason| ConfigError::Audience { setting, reason })?;
        Ok(Self {
            issuer,
            authorize_url,
            client_id,
            client_secret,
            redirect_uri,
            signing_key,
            iap_audience,
            code_lifetime: CODE_LIFETIME,
        })
    }

    /// The same configuration with codes redeemable for `lifetime` instead of
    /// [`CODE_LIFETIME`].
    pub fn with_code_lifetime(mut self, lifetime: Duration) -> Self {
        self.code_lifetime = lifetime;
        self
    }

    fn endpoint(&self, suffix: &str) -> String {
        format!("{}{suffix}", self.issuer)
    }
}

/// A credential-free http(s) URL with no query or fragment, written exactly
/// as it serializes so string comparisons by relying parties hold.
fn url(setting: &'static str, value: &str) -> Result<Url, ConfigError> {
    let url = Url::parse(value).map_err(|source| ConfigError::Unparsable { setting, source })?;
    let reason = if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
        "must be an http(s) URL"
    } else if !url.username().is_empty() || url.password().is_some() {
        "must not carry credentials"
    } else if url.query().is_some() || url.fragment().is_some() {
        "must not have a query or fragment"
    } else if url.as_str() != value {
        "must be written in canonical form"
    } else {
        return Ok(url);
    };
    Err(ConfigError::Url { setting, reason })
}

/// The ES256 key that signs ID tokens, with its published JWK.
#[derive(Clone)]
struct SigningKey {
    encoding: EncodingKey,
    kid: String,
    jwk: Value,
}

impl SigningKey {
    fn from_base64(setting: &'static str, value: &str) -> Result<Self, ConfigError> {
        let der = STANDARD
            .decode(value.trim())
            .map_err(|source| ConfigError::KeyEncoding { setting, source })?;
        let pair =
            EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &der, &SystemRandom::new())
                .map_err(|source| ConfigError::Key { setting, source })?;
        // An uncompressed SEC1 point: 0x04, then 32 bytes each of x and y.
        let point = pair.public_key().as_ref();
        let x = URL_SAFE_NO_PAD.encode(&point[1..33]);
        let y = URL_SAFE_NO_PAD.encode(&point[33..65]);
        // RFC 7638 thumbprint: the required members in lexicographic order,
        // which a BTreeMap serializes in, with no whitespace.
        let thumbprint = serde_json::to_vec(&BTreeMap::from([
            ("crv", "P-256"),
            ("kty", "EC"),
            ("x", x.as_str()),
            ("y", y.as_str()),
        ]))
        .expect("string map serializes");
        let kid = URL_SAFE_NO_PAD.encode(digest(&SHA256, &thumbprint));
        let jwk = json!({
            "kty": "EC",
            "crv": "P-256",
            "x": x,
            "y": y,
            "kid": kid,
            "alg": "ES256",
            "use": "sig",
        });
        Ok(Self {
            encoding: EncodingKey::from_ec_der(&der),
            kid,
            jwk,
        })
    }
}

/// A single-use authorization code: 32 random bytes, base64url.
#[derive(Clone, PartialEq, Eq, Hash)]
struct AuthorizationCode(String);

/// A PKCE S256 `code_challenge`: base64url SHA-256, 43 characters.
struct S256Challenge([u8; 32]);

impl S256Challenge {
    fn parse(value: &str) -> Option<Self> {
        if value.len() != 43 {
            return None;
        }
        URL_SAFE_NO_PAD
            .decode(value)
            .ok()?
            .try_into()
            .ok()
            .map(Self)
    }

    fn verifies(&self, verifier: &CodeVerifier) -> bool {
        digest(&SHA256, verifier.0.as_bytes())
            .as_ref()
            .ct_eq(&self.0)
            .into()
    }
}

/// A PKCE `code_verifier`: 43 to 128 unreserved characters (RFC 7636).
struct CodeVerifier(String);

impl CodeVerifier {
    fn parse(value: &str) -> Option<Self> {
        let unreserved =
            |c: u8| c.is_ascii_alphanumeric() || matches!(c, b'-' | b'.' | b'_' | b'~');
        ((43..=128).contains(&value.len()) && value.bytes().all(unreserved))
            .then(|| Self(value.to_owned()))
    }
}

/// What a code redeems for, fixed when the browser was authorized.
struct Grant {
    identity: IapIdentity,
    challenge: S256Challenge,
    nonce: Option<String>,
    authenticated_at: u64,
    expires_at: Instant,
}

struct Issuer {
    config: Config,
    verifier: IapVerifier,
    grants: Mutex<HashMap<AuthorizationCode, Grant>>,
    random: SystemRandom,
}

/// A fault in the issuer itself, reported to the operator on stderr and to
/// the client only as a server error.
#[derive(Debug, thiserror::Error)]
enum Fault {
    #[error("the system random source failed")]
    Random(#[source] ring::error::Unspecified),
    #[error("the system clock is before 1970")]
    Clock(#[source] std::time::SystemTimeError),
    #[error("signing the ID token failed")]
    Signing(#[source] jsonwebtoken::errors::Error),
}

impl Fault {
    fn report(&self) {
        let mut message = self.to_string();
        let mut cause = std::error::Error::source(self);
        while let Some(source) = cause {
            message.push_str(&format!(": {source}"));
            cause = source.source();
        }
        eprintln!("langfuse-sso: {message}");
    }
}

impl IntoResponse for Fault {
    fn into_response(self) -> Response {
        self.report();
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

fn unix_now() -> Result<u64, Fault> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(Fault::Clock)?
        .as_secs())
}

impl Issuer {
    fn random_token(&self) -> Result<String, Fault> {
        let mut bytes = [0; 32];
        self.random.fill(&mut bytes).map_err(Fault::Random)?;
        Ok(URL_SAFE_NO_PAD.encode(bytes))
    }

    fn grants(&self) -> std::sync::MutexGuard<'_, HashMap<AuthorizationCode, Grant>> {
        // No critical section can leave the map half-updated, so poison is moot.
        self.grants.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// The issuer's whole HTTP surface.
pub fn router(config: Config) -> Router {
    let verifier = IapVerifier::new(Some(config.iap_audience.clone()));
    router_with_verifier(config, verifier)
}

/// The router with an injected verifier, for tests with fixture keys.
pub fn router_with_verifier(config: Config, verifier: IapVerifier) -> Router {
    let base = config.issuer.path().to_owned();
    let issuer = Arc::new(Issuer {
        config,
        verifier,
        grants: Mutex::new(HashMap::new()),
        random: SystemRandom::new(),
    });
    Router::new()
        .route(
            &format!("{base}/.well-known/openid-configuration"),
            get(discovery),
        )
        .route(&format!("{base}/.well-known/jwks.json"), get(jwks))
        .route(&format!("{base}/authorize"), get(authorize))
        .route(
            &format!("{base}/token"),
            post(token).layer(DefaultBodyLimit::max(TOKEN_BODY_LIMIT)),
        )
        .fallback(not_found)
        .method_not_allowed_fallback(not_found)
        .with_state(issuer)
}

async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}

async fn discovery(State(issuer): State<Arc<Issuer>>) -> Json<Value> {
    let config = &issuer.config;
    Json(json!({
        "issuer": config.issuer.as_str(),
        "authorization_endpoint": config.authorize_url.as_str(),
        "token_endpoint": config.endpoint("/token"),
        "jwks_uri": config.endpoint("/.well-known/jwks.json"),
        "response_types_supported": ["code"],
        "subject_types_supported": ["public"],
        "id_token_signing_alg_values_supported": ["ES256"],
        "token_endpoint_auth_methods_supported": ["client_secret_basic"],
        "code_challenge_methods_supported": ["S256"],
        "scopes_supported": ["openid", "email", "profile"],
        "grant_types_supported": ["authorization_code"],
    }))
}

async fn jwks(State(issuer): State<Arc<Issuer>>) -> Json<Value> {
    Json(json!({"keys": [issuer.config.signing_key.jwk]}))
}

/// Query or form parameters, refusing any name given twice.
fn parameters(encoded: &[u8]) -> Option<HashMap<String, String>> {
    let mut parameters = HashMap::new();
    for (name, value) in form_urlencoded::parse(encoded) {
        match parameters.entry(name.into_owned()) {
            Entry::Occupied(_) => return None,
            Entry::Vacant(slot) => slot.insert(value.into_owned()),
        };
    }
    Some(parameters)
}

/// Why `authorize` refused a request it would otherwise have redirected.
#[derive(Debug, thiserror::Error)]
enum AuthorizeError {
    #[error("parameters must be well-formed and appear once")]
    Malformed,
    #[error("response_type must be code")]
    ResponseType,
    #[error("unknown client_id")]
    ClientId,
    #[error("redirect_uri is not registered")]
    RedirectUri,
    #[error("scope must include openid")]
    Scope,
    #[error("state is required")]
    State,
    #[error("code_challenge_method must be S256")]
    ChallengeMethod,
    #[error("code_challenge must be a base64url SHA-256 digest")]
    Challenge,
    #[error("nonce is too long")]
    Nonce,
}

/// A request `authorize` accepted, ready for a code.
struct Authorization {
    state: String,
    challenge: S256Challenge,
    nonce: Option<String>,
}

fn authorization(config: &Config, query: Option<&str>) -> Result<Authorization, AuthorizeError> {
    let parameters =
        parameters(query.unwrap_or_default().as_bytes()).ok_or(AuthorizeError::Malformed)?;
    let get = |name: &str| parameters.get(name).map(String::as_str);
    if get("response_type") != Some("code") {
        return Err(AuthorizeError::ResponseType);
    }
    if get("client_id") != Some(config.client_id.as_str()) {
        return Err(AuthorizeError::ClientId);
    }
    if get("redirect_uri") != Some(config.redirect_uri.as_str()) {
        return Err(AuthorizeError::RedirectUri);
    }
    if !get("scope").is_some_and(|scope| scope.split(' ').any(|scope| scope == "openid")) {
        return Err(AuthorizeError::Scope);
    }
    let state = get("state")
        .filter(|state| !state.is_empty() && state.len() <= MAX_STATE_BYTES)
        .ok_or(AuthorizeError::State)?;
    if get("code_challenge_method") != Some("S256") {
        return Err(AuthorizeError::ChallengeMethod);
    }
    let challenge = get("code_challenge")
        .and_then(S256Challenge::parse)
        .ok_or(AuthorizeError::Challenge)?;
    let nonce = match get("nonce") {
        Some(nonce) if nonce.len() > MAX_STATE_BYTES => return Err(AuthorizeError::Nonce),
        nonce => nonce.filter(|nonce| !nonce.is_empty()).map(str::to_owned),
    };
    Ok(Authorization {
        state: state.to_owned(),
        challenge,
        nonce,
    })
}

async fn authorize(
    State(issuer): State<Arc<Issuer>>,
    headers: HeaderMap,
    RawQuery(query): RawQuery,
) -> Response {
    let identity = match single_header(&headers, ASSERTION_HEADER) {
        Some(token) => issuer.verifier.identity(token).await,
        None => None,
    };
    let Some(identity) = identity else {
        return (StatusCode::FORBIDDEN, "IAP identity required").into_response();
    };
    let request = match authorization(&issuer.config, query.as_deref()) {
        Ok(request) => request,
        Err(error) => return (StatusCode::BAD_REQUEST, error.to_string()).into_response(),
    };
    let (code, authenticated_at) = match (issuer.random_token(), unix_now()) {
        (Ok(code), Ok(now)) => (AuthorizationCode(code), now),
        (Err(fault), _) | (_, Err(fault)) => return fault.into_response(),
    };
    let now = Instant::now();
    {
        let mut grants = issuer.grants();
        grants.retain(|_, grant| grant.expires_at > now);
        if grants.len() >= MAX_GRANTS {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "too many sign-ins in progress",
            )
                .into_response();
        }
        grants.insert(
            code.clone(),
            Grant {
                identity,
                challenge: request.challenge,
                nonce: request.nonce,
                authenticated_at,
                expires_at: now + issuer.config.code_lifetime,
            },
        );
    }
    let mut target = issuer.config.redirect_uri.clone();
    target
        .query_pairs_mut()
        .append_pair("code", &code.0)
        .append_pair("state", &request.state);
    (StatusCode::FOUND, [(header::LOCATION, target.as_str())]).into_response()
}

/// The OAuth token endpoint's error responses (RFC 6749 section 5.2).
#[derive(Debug)]
enum TokenError {
    InvalidClient,
    InvalidRequest,
    UnsupportedGrantType,
    InvalidGrant,
    Server(Fault),
}

impl From<Fault> for TokenError {
    fn from(fault: Fault) -> Self {
        Self::Server(fault)
    }
}

impl IntoResponse for TokenError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            Self::InvalidClient => (StatusCode::UNAUTHORIZED, "invalid_client"),
            Self::InvalidRequest => (StatusCode::BAD_REQUEST, "invalid_request"),
            Self::UnsupportedGrantType => (StatusCode::BAD_REQUEST, "unsupported_grant_type"),
            Self::InvalidGrant => (StatusCode::BAD_REQUEST, "invalid_grant"),
            Self::Server(fault) => {
                fault.report();
                (StatusCode::INTERNAL_SERVER_ERROR, "server_error")
            }
        };
        let mut response = (
            status,
            [(header::CACHE_CONTROL, "no-store")],
            Json(json!({"error": code})),
        )
            .into_response();
        if matches!(self, Self::InvalidClient) {
            response.headers_mut().insert(
                header::WWW_AUTHENTICATE,
                header::HeaderValue::from_static("Basic"),
            );
        }
        response
    }
}

/// The client in `Authorization: Basic`, per RFC 6749 section 2.3.1: each
/// half form-urlencoded before base64.
fn authenticates(config: &Config, headers: &HeaderMap) -> bool {
    let decode = |half: &str| {
        percent_encoding::percent_decode_str(&half.replace('+', " "))
            .decode_utf8()
            .ok()
            .map(|half| half.into_owned())
    };
    let credentials = single_header(headers, header::AUTHORIZATION.as_str())
        .and_then(|value| value.strip_prefix("Basic "))
        .and_then(|encoded| STANDARD.decode(encoded.trim()).ok())
        .and_then(|bytes| String::from_utf8(bytes).ok());
    let Some((id, secret)) = credentials.as_deref().and_then(|pair| pair.split_once(':')) else {
        return false;
    };
    let (Some(id), Some(secret)) = (decode(id), decode(secret)) else {
        return false;
    };
    let id = id.as_bytes().ct_eq(config.client_id.as_bytes());
    let secret = secret.as_bytes().ct_eq(config.client_secret.as_bytes());
    (id & secret).into()
}

#[derive(Serialize)]
struct IdTokenClaims<'a> {
    iss: &'a str,
    sub: &'a str,
    aud: &'a str,
    email: &'a str,
    email_verified: bool,
    name: &'a str,
    iat: u64,
    auth_time: u64,
    exp: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<&'a str>,
}

async fn token(
    State(issuer): State<Arc<Issuer>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, TokenError> {
    let config = &issuer.config;
    if !authenticates(config, &headers) {
        return Err(TokenError::InvalidClient);
    }
    let form = parameters(&body).ok_or(TokenError::InvalidRequest)?;
    let get = |name: &str| form.get(name).map(String::as_str);
    if get("grant_type") != Some("authorization_code") {
        return Err(TokenError::UnsupportedGrantType);
    }
    let code = AuthorizationCode(get("code").ok_or(TokenError::InvalidRequest)?.to_owned());
    let grant = issuer
        .grants()
        .remove(&code)
        .filter(|grant| grant.expires_at > Instant::now())
        .ok_or(TokenError::InvalidGrant)?;
    if get("redirect_uri") != Some(config.redirect_uri.as_str()) {
        return Err(TokenError::InvalidGrant);
    }
    let verifier = get("code_verifier")
        .and_then(CodeVerifier::parse)
        .ok_or(TokenError::InvalidGrant)?;
    if !grant.challenge.verifies(&verifier) {
        return Err(TokenError::InvalidGrant);
    }
    let issued_at = unix_now()?;
    let claims = IdTokenClaims {
        iss: config.issuer.as_str(),
        sub: &grant.identity.sub,
        aud: &config.client_id,
        email: &grant.identity.email,
        email_verified: true,
        name: &grant.identity.email,
        iat: issued_at,
        auth_time: grant.authenticated_at,
        exp: issued_at + TOKEN_LIFETIME,
        nonce: grant.nonce.as_deref(),
    };
    let mut header = Header::new(Algorithm::ES256);
    header.kid = Some(config.signing_key.kid.clone());
    let id_token = jsonwebtoken::encode(&header, &claims, &config.signing_key.encoding)
        .map_err(Fault::Signing)?;
    let access_token = issuer.random_token()?;
    Ok((
        [
            (header::CACHE_CONTROL, "no-store"),
            (header::PRAGMA, "no-cache"),
        ],
        Json(json!({
            "access_token": access_token,
            "token_type": "Bearer",
            "expires_in": TOKEN_LIFETIME,
            "id_token": id_token,
        })),
    )
        .into_response())
}
