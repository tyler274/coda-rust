//! Fail-closed Cloudflare Access verification. Secrets/tokens are never logged.
use axum::http::HeaderMap;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header, jwk::JwkSet};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use subtle::ConstantTimeEq;
use tokio::sync::Mutex;

/// Intentionally no Debug implementation: this contains service credentials.
#[derive(Clone, Default)]
pub struct AccessConfig {
    pub issuer: Option<String>,
    pub audience: Option<String>,
    pub service_client_id: Option<String>,
    pub service_client_secret: Option<String>,
}
impl AccessConfig {
    pub fn normalize_issuer(value: &str) -> Result<String, &'static str> {
        let value = value.trim().trim_end_matches('/');
        let value = if value.starts_with("https://") {
            value.to_owned()
        } else {
            format!("https://{value}")
        };
        let url =
            reqwest::Url::parse(&value).map_err(|_| "invalid Cloudflare Access team domain")?;
        let host = url
            .host_str()
            .ok_or("invalid Cloudflare Access team domain")?;
        if url.scheme() != "https"
            || !host.ends_with(".cloudflareaccess.com")
            || host == ".cloudflareaccess.com"
            || !url.username().is_empty()
            || url.password().is_some()
            || url.port().is_some()
            || url.path() != "/"
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return Err(
                "Cloudflare Access team domain must be an HTTPS *.cloudflareaccess.com origin",
            );
        }
        Ok(format!("https://{host}"))
    }
}

#[derive(Default)]
struct Cache {
    keys: Option<JwkSet>,
    fetched: Option<Instant>,
    attempted: Option<Instant>,
}

#[derive(Clone)]
pub struct AccessGate {
    config: AccessConfig,
    client: reqwest::Client,
    cache: Arc<Mutex<Cache>>,
}

#[derive(Deserialize)]
struct Claims {
    exp: u64,
    iss: String,
    aud: serde_json::Value,
}

impl AccessGate {
    pub fn new(mut config: AccessConfig) -> Self {
        // Explicit/programmatic configurations must obey the same trusted origin
        // restriction as environment-loaded configurations.
        config.issuer = config
            .issuer
            .as_deref()
            .and_then(|value| AccessConfig::normalize_issuer(value).ok());
        config.audience = config.audience.filter(|value| !value.trim().is_empty());
        Self {
            config,
            client: reqwest::Client::builder()
                .https_only(true)
                .redirect(reqwest::redirect::Policy::none())
                .timeout(Duration::from_secs(3))
                .connect_timeout(Duration::from_secs(2))
                .build()
                .expect("TLS HTTP client"),
            cache: Arc::new(Mutex::new(Cache::default())),
        }
    }

    pub async fn allows(&self, headers: &HeaderMap) -> bool {
        // Preserve source semantics: either a JWT OR the exact configured service
        // credential pair is sufficient. A health token alone never grants Access.
        if let (Some(id), Some(secret)) = (
            &self.config.service_client_id,
            &self.config.service_client_secret,
        ) {
            let id_ok = secret_matches(headers, "cf-access-client-id", id);
            let secret_ok = secret_matches(headers, "cf-access-client-secret", secret);
            if id_ok & secret_ok {
                return true;
            }
        }
        let Some(token) = single_header(headers, "cf-access-jwt-assertion") else {
            return false;
        };
        if token.len() > 16 * 1024 {
            return false;
        }
        let (Some(issuer), Some(audience)) = (&self.config.issuer, &self.config.audience) else {
            return false;
        };
        let Ok(header) = decode_header(token) else {
            return false;
        };
        if header.alg != Algorithm::RS256 {
            return false;
        }
        let Some(kid) = header.kid else {
            return false;
        };
        if kid.len() > 256 {
            return false;
        }
        let Some(key) = self.key(&kid).await else {
            return false;
        };
        verify(token, issuer, audience, &key)
    }

    async fn key(&self, kid: &str) -> Option<DecodingKey> {
        // Single flight, bounded by the HTTP timeout. Unknown kids may request a
        // refresh, but at most once per 30s across all callers (DoS protection).
        let mut cache = self.cache.lock().await;
        let fresh = cache
            .fetched
            .is_some_and(|at| at.elapsed() < Duration::from_secs(300));
        if fresh && let Some(key) = cache.keys.as_ref().and_then(|keys| decoding_key(keys, kid)) {
            return Some(key);
        }
        if cache
            .attempted
            .is_some_and(|at| at.elapsed() < Duration::from_secs(30))
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
        let issuer = self.config.issuer.as_ref()?;
        let mut response = self
            .client
            .get(format!("{issuer}/cdn-cgi/access/certs"))
            .send()
            .await
            .ok()?
            .error_for_status()
            .ok()?;
        if response
            .content_length()
            .is_some_and(|len| len > 256 * 1024)
        {
            return None;
        }
        let mut body = Vec::new();
        while let Some(chunk) = response.chunk().await.ok()? {
            if body.len() + chunk.len() > 256 * 1024 {
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
    let key = keys.find(kid)?;
    if key
        .common
        .key_algorithm
        .is_some_and(|alg| alg != jsonwebtoken::jwk::KeyAlgorithm::RS256)
    {
        return None;
    }
    if key
        .common
        .public_key_use
        .as_ref()
        .is_some_and(|usage| *usage != jsonwebtoken::jwk::PublicKeyUse::Signature)
    {
        return None;
    }
    if key
        .common
        .key_operations
        .as_ref()
        .is_some_and(|ops| !ops.contains(&jsonwebtoken::jwk::KeyOperations::Verify))
    {
        return None;
    }
    if !matches!(
        key.algorithm,
        jsonwebtoken::jwk::AlgorithmParameters::RSA(_)
    ) {
        return None;
    }
    DecodingKey::from_jwk(key).ok()
}

fn verify(token: &str, issuer: &str, audience: &str, key: &DecodingKey) -> bool {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[issuer]);
    validation.set_audience(&[audience]);
    validation.set_required_spec_claims(&["exp", "iss", "aud"]);
    validation.validate_nbf = true;
    validation.leeway = 0;
    let Ok(token) = decode::<Claims>(token, key, &validation) else {
        return false;
    };
    // Keep required claim types explicit as well as library validation.
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    token.claims.exp > now && token.claims.iss == issuer && !token.claims.aud.is_null()
}

fn single_header<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    let mut values = headers.get_all(name).iter();
    let value = values.next()?.to_str().ok()?;
    if values.next().is_some() || value.is_empty() {
        return None;
    }
    Some(value)
}

pub fn secret_matches(headers: &HeaderMap, name: &str, expected: &str) -> bool {
    if expected.is_empty() {
        return false;
    }
    let Some(actual) = single_header(headers, name) else {
        return false;
    };
    // Fixed-size digests avoid leaking credential length via the comparison.
    bool::from(Sha256::digest(actual.as_bytes()).ct_eq(&Sha256::digest(expected.as_bytes())))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn service_pair_is_an_alternative_not_a_health_bypass() {
        let gate = AccessGate::new(AccessConfig {
            service_client_id: Some("id".into()),
            service_client_secret: Some("secret".into()),
            ..Default::default()
        });
        let mut headers = HeaderMap::new();
        assert!(!gate.allows(&headers).await);
        headers.insert("cf-access-client-id", "id".parse().unwrap());
        assert!(!gate.allows(&headers).await);
        headers.insert("cf-access-client-secret", "secret".parse().unwrap());
        assert!(gate.allows(&headers).await);
        headers.append("cf-access-client-secret", "secret".parse().unwrap());
        assert!(!gate.allows(&headers).await);
    }
    #[tokio::test]
    async fn absent_configuration_fails_closed() {
        let gate = AccessGate::new(AccessConfig::default());
        let mut headers = HeaderMap::new();
        headers.insert("cf-access-jwt-assertion", "anything".parse().unwrap());
        assert!(!gate.allows(&headers).await);
    }
    #[tokio::test]
    async fn validates_signature_issuer_audience_expiry_and_algorithm_from_cached_jwks() {
        use base64::Engine;
        use rsa::{pkcs1::EncodeRsaPrivateKey, traits::PublicKeyParts};
        let private = rsa::RsaPrivateKey::new(
            &mut <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(42),
            2048,
        )
        .unwrap();
        let public = private.to_public_key();
        let b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD;
        let keys: JwkSet = serde_json::from_value(serde_json::json!({"keys": [{"kty": "RSA", "alg": "RS256", "use": "sig", "kid": "test", "n": b64.encode(public.n().to_bytes_be()), "e": b64.encode(public.e().to_bytes_be())}]})).unwrap();
        let issuer = "https://test.cloudflareaccess.com";
        let gate = AccessGate::new(AccessConfig {
            issuer: Some(issuer.into()),
            audience: Some("audience".into()),
            ..Default::default()
        });
        {
            let mut cache = gate.cache.lock().await;
            cache.keys = Some(keys);
            cache.fetched = Some(Instant::now());
            cache.attempted = Some(Instant::now());
        }
        let private_der = private.to_pkcs1_der().unwrap();
        let key = jsonwebtoken::EncodingKey::from_rsa_der(private_der.as_bytes());
        let mut header = jsonwebtoken::Header::new(Algorithm::RS256);
        header.kid = Some("test".into());
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let valid = serde_json::json!({"iss": issuer, "aud": ["audience"], "exp": now + 60});
        let check = |claims: serde_json::Value, header: &jsonwebtoken::Header| {
            let token = jsonwebtoken::encode(header, &claims, &key).unwrap();
            let mut headers = HeaderMap::new();
            headers.insert("cf-access-jwt-assertion", token.parse().unwrap());
            headers
        };
        assert!(gate.allows(&check(valid.clone(), &header)).await);
        let mut tampered = check(valid.clone(), &header);
        let token = tampered["cf-access-jwt-assertion"].to_str().unwrap();
        let mut pieces: Vec<String> = token.split('.').map(str::to_owned).collect();
        pieces[1] = b64.encode(
            serde_json::to_vec(
                &serde_json::json!({"iss": issuer, "aud": ["audience"], "exp": now + 3600}),
            )
            .unwrap(),
        );
        tampered.insert("cf-access-jwt-assertion", pieces.join(".").parse().unwrap());
        assert!(!gate.allows(&tampered).await);
        for (name, value) in [
            (
                "iss",
                serde_json::json!("https://wrong.cloudflareaccess.com"),
            ),
            ("aud", serde_json::json!("wrong")),
            ("exp", serde_json::json!(now - 1)),
            ("nbf", serde_json::json!(now + 60)),
        ] {
            let mut invalid = valid.clone();
            invalid[name] = value;
            assert!(!gate.allows(&check(invalid, &header)).await, "{name}");
        }
        for name in ["iss", "aud", "exp"] {
            let mut invalid = valid.clone();
            invalid.as_object_mut().unwrap().remove(name);
            assert!(!gate.allows(&check(invalid, &header)).await);
        }
        header.kid = Some("unknown".into());
        assert!(!gate.allows(&check(valid.clone(), &header)).await);
        let mut hs = jsonwebtoken::Header::new(Algorithm::HS256);
        hs.kid = Some("test".into());
        let token = jsonwebtoken::encode(
            &hs,
            &valid,
            &jsonwebtoken::EncodingKey::from_secret(b"secret"),
        )
        .unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("cf-access-jwt-assertion", token.parse().unwrap());
        assert!(!gate.allows(&headers).await);
        {
            let mut cache = gate.cache.lock().await;
            cache.fetched = Some(Instant::now() - Duration::from_secs(301));
        }
        header.kid = Some("test".into());
        assert!(
            !gate.allows(&check(valid, &header)).await,
            "expired cache must fail closed during refresh cooldown"
        );
    }
    #[test]
    fn issuer_is_an_https_cloudflare_origin() {
        assert_eq!(
            AccessConfig::normalize_issuer(" team.cloudflareaccess.com/ ").unwrap(),
            "https://team.cloudflareaccess.com"
        );
        for value in [
            "",
            "http://team.cloudflareaccess.com",
            "localhost",
            "team.cloudflareaccess.com/evil",
            "team.cloudflareaccess.com?x=1",
            "evil.example",
            "user@team.cloudflareaccess.com",
        ] {
            assert!(AccessConfig::normalize_issuer(value).is_err(), "{value}");
        }
    }
}
