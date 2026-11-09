//! Fail-closed Google IAP signed-header verification for Cloud Run services
//! behind native IAP. Assertions are never logged.
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header, jwk::JwkSet};
use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

/// The header Cloud Run IAP adds to every request it admits.
pub const ASSERTION_HEADER: &str = "x-goog-iap-jwt-assertion";
const ISSUER: &str = "https://cloud.google.com/iap";
const JWKS_URL: &str = "https://www.gstatic.com/iap/verify/public_key-jwk";
const MAX_KEY_BYTES: usize = 256 * 1024;
const MAX_ASSERTION_BYTES: usize = 16 * 1024;
const CACHE_TTL: Duration = Duration::from_secs(300);
const REFRESH_INTERVAL: Duration = Duration::from_secs(30);

/// Checks a direct Cloud Run signed-header audience,
/// `/projects/NUMBER/locations/REGION/services/NAME`. It is not an OAuth
/// client ID or a URL.
pub fn validate_cloud_run_audience(audience: &str) -> Result<(), &'static str> {
    let parts: Vec<_> = audience.split('/').collect();
    let resource_name = |value: &str| {
        !value.is_empty()
            && value.len() <= 63
            && value.starts_with(|c: char| c.is_ascii_lowercase())
            && value.ends_with(|c: char| c.is_ascii_lowercase() || c.is_ascii_digit())
            && value
                .bytes()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
    };
    if parts.len() != 7
        || !parts[0].is_empty()
        || parts[1] != "projects"
        || parts[2].is_empty()
        || parts[2].len() > 20
        || !parts[2].bytes().all(|c| c.is_ascii_digit())
        || parts[2].starts_with('0')
        || parts[3] != "locations"
        || !resource_name(parts[4])
        || parts[5] != "services"
        || !resource_name(parts[6])
    {
        return Err("IAP audience must be /projects/NUMBER/locations/REGION/services/NAME");
    }
    Ok(())
}

/// The value of a header that appears exactly once and is not blank.
pub fn single_header<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    let mut values = headers.get_all(name).iter();
    let value = values.next()?.to_str().ok()?;
    if values.next().is_some() || value.trim().is_empty() {
        return None;
    }
    Some(value)
}

#[derive(Default)]
struct Cache {
    keys: Option<JwkSet>,
    fetched: Option<Instant>,
    attempted: Option<Instant>,
}

/// Verifies `x-goog-iap-jwt-assertion` against Google's IAP keys and one
/// fixed audience. A missing or malformed audience denies every request.
#[derive(Clone)]
pub struct IapVerifier {
    audience: Option<String>,
    client: reqwest::Client,
    cache: Arc<Mutex<Cache>>,
    // Always Google's key set in production; only test constructors change it.
    key_url: String,
}

#[derive(Deserialize)]
struct Claims {
    exp: u64,
    iat: u64,
    iss: String,
    aud: String,
    #[serde(default, deserialize_with = "present_numeric_date")]
    nbf: Option<u64>,
}

// An absent nbf is allowed; an explicit null or a nonnumeric value is not.
fn present_numeric_date<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<u64>, D::Error> {
    u64::deserialize(deserializer).map(Some)
}

impl IapVerifier {
    /// A verifier for one Cloud Run service audience.
    pub fn new(audience: Option<String>) -> Self {
        Self {
            audience,
            client: reqwest::Client::builder()
                .https_only(true)
                .redirect(reqwest::redirect::Policy::none())
                .timeout(Duration::from_secs(3))
                .connect_timeout(Duration::from_secs(2))
                .build()
                .expect("TLS HTTP client"),
            cache: Arc::new(Mutex::new(Cache::default())),
            key_url: JWKS_URL.into(),
        }
    }

    /// True only for exactly one valid assertion for this audience.
    pub async fn allows(&self, headers: &HeaderMap) -> bool {
        match single_header(headers, ASSERTION_HEADER) {
            Some(token) => self.verifies(token).await,
            None => false,
        }
    }

    /// True only when `token` is a valid assertion for this audience.
    pub async fn verifies(&self, token: &str) -> bool {
        let Some(audience) = self
            .audience
            .as_deref()
            .filter(|audience| validate_cloud_run_audience(audience).is_ok())
        else {
            return false;
        };
        if token.len() > MAX_ASSERTION_BYTES {
            return false;
        }
        let Ok(header) = decode_header(token) else {
            return false;
        };
        if header.alg != Algorithm::ES256 {
            return false;
        }
        let Some(kid) = header.kid else {
            return false;
        };
        if kid.is_empty() || kid.len() > 256 {
            return false;
        }
        let Some(key) = self.key(&kid).await else {
            return false;
        };
        let Ok(now) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) else {
            return false;
        };
        verify(token, audience, &key, now.as_secs())
    }

    async fn key(&self, kid: &str) -> Option<DecodingKey> {
        // Single flight, bounded by the HTTP timeout. Unknown kids may request a
        // refresh, but at most once per 30s across all callers (DoS protection).
        let mut cache = self.cache.lock().await;
        let fresh = cache.fetched.is_some_and(|at| at.elapsed() < CACHE_TTL);
        if fresh && let Some(key) = cache.keys.as_ref().and_then(|keys| decoding_key(keys, kid)) {
            return Some(key);
        }
        if cache
            .attempted
            .is_some_and(|at| at.elapsed() < REFRESH_INTERVAL)
        {
            return None;
        }
        cache.attempted = Some(Instant::now());
        // Expired keys never authorize requests after a failed refresh.
        let keys = self.fetch_keys().await?;
        cache.keys = Some(keys);
        cache.fetched = Some(Instant::now());
        cache.keys.as_ref().and_then(|keys| decoding_key(keys, kid))
    }

    async fn fetch_keys(&self) -> Option<JwkSet> {
        let mut response = self.client.get(&self.key_url).send().await.ok()?;
        // Explicit success check also rejects redirects (which are never followed).
        if !response.status().is_success()
            || response
                .content_length()
                .is_some_and(|len| len > MAX_KEY_BYTES as u64)
        {
            return None;
        }
        let mut body = Vec::new();
        while let Some(chunk) = response.chunk().await.ok()? {
            if body.len() + chunk.len() > MAX_KEY_BYTES {
                return None;
            }
            body.extend_from_slice(&chunk);
        }
        let keys: JwkSet = serde_json::from_slice(&body).ok()?;
        if keys.keys.is_empty() || keys.keys.len() > 32 {
            return None;
        }
        Some(keys)
    }
}

fn decoding_key(keys: &JwkSet, kid: &str) -> Option<DecodingKey> {
    // Ambiguous key IDs must not silently select the first entry.
    let mut matching = keys
        .keys
        .iter()
        .filter(|key| key.common.key_id.as_deref() == Some(kid));
    let key = matching.next()?;
    if matching.next().is_some()
        || key
            .common
            .key_algorithm
            .is_some_and(|alg| alg != jsonwebtoken::jwk::KeyAlgorithm::ES256)
        || key
            .common
            .public_key_use
            .as_ref()
            .is_some_and(|usage| *usage != jsonwebtoken::jwk::PublicKeyUse::Signature)
        || key
            .common
            .key_operations
            .as_ref()
            .is_some_and(|ops| !ops.contains(&jsonwebtoken::jwk::KeyOperations::Verify))
    {
        return None;
    }
    match &key.algorithm {
        jsonwebtoken::jwk::AlgorithmParameters::EllipticCurve(parameters)
            if parameters.curve == jsonwebtoken::jwk::EllipticCurve::P256 => {}
        _ => return None,
    }
    DecodingKey::from_jwk(key).ok()
}

fn verify(token: &str, audience: &str, key: &DecodingKey, now: u64) -> bool {
    let mut validation = Validation::new(Algorithm::ES256);
    validation.set_issuer(&[ISSUER]);
    validation.set_audience(&[audience]);
    validation.set_required_spec_claims(&["exp", "iat", "iss", "aud"]);
    // Perform all NumericDate checks together, using one clock snapshot. The
    // Google signed-header contract allows 30s skew and at most 600 + 2*30s
    // lifetime: exp > now-30, iat < now+30, and exp > iat. nbf is inclusive.
    validation.validate_exp = false;
    validation.validate_nbf = false;
    let Ok(token) = decode::<Claims>(token, key, &validation) else {
        return false;
    };
    let claims = token.claims;
    claims.iss == ISSUER
        && claims.aud == audience
        && claims.exp > now.saturating_sub(30)
        && claims.iat < now.saturating_add(30)
        && claims.exp > claims.iat
        && claims.exp - claims.iat <= 660
        && claims
            .nbf
            .is_none_or(|nbf| nbf <= now.saturating_add(30) && nbf < claims.exp)
}

/// Deliberately public fixtures for other crates' tests. They never fetch
/// Google's keys. Enable only from `[dev-dependencies]`.
#[cfg(any(test, feature = "test-util"))]
pub mod fixture {
    use super::*;
    use base64::Engine;
    use jsonwebtoken::{EncodingKey, Header, encode};
    use serde_json::{Value, json};

    /// The fixture key's ID.
    pub const KID: &str = "public-test-key";

    /// The public test key set, including its disclosed private key.
    pub fn key_file() -> Value {
        serde_json::from_str(include_str!("iap/public-test-key.json")).expect("fixture JSON")
    }

    pub(super) fn jwks() -> JwkSet {
        serde_json::from_value(key_file()).expect("fixture key set")
    }

    /// Signs `claims` with the fixture key.
    pub fn sign(claims: &Value, kid: Option<&str>) -> String {
        let der = base64::engine::general_purpose::STANDARD
            .decode(key_file()["private_pkcs8_base64"].as_str().expect("key"))
            .expect("base64 key");
        let mut header = Header::new(Algorithm::ES256);
        header.kid = kid.map(str::to_owned);
        encode(&header, claims, &EncodingKey::from_ec_der(&der)).expect("signed fixture")
    }

    /// Valid claims for `audience`, issued just before `now`.
    pub fn claims(audience: &str, now: u64) -> Value {
        json!({"iss": ISSUER, "aud": audience, "iat": now - 10, "exp": now + 590})
    }

    /// A currently valid assertion for `audience`.
    pub fn assertion(audience: &str) -> String {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_secs();
        sign(&claims(audience, now), Some(KID))
    }

    /// A verifier whose cache already holds the fixture key. Any refresh it
    /// attempts can only reach a closed local port.
    pub fn verifier(audience: &str) -> IapVerifier {
        let mut verifier = IapVerifier::new(Some(audience.into()));
        verifier.cache = Arc::new(Mutex::new(Cache {
            keys: Some(jwks()),
            fetched: Some(Instant::now()),
            attempted: Some(Instant::now()),
        }));
        verifier.key_url = "http://127.0.0.1:1".into();
        verifier
    }
}

#[cfg(test)]
#[path = "iap/tests.rs"]
mod tests;
