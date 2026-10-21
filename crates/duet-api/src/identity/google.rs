//! Google adapters, intentionally not wired into hosted routing.
//!
//! Contracts: https://firebase.google.com/docs/auth/admin/verify-id-tokens,
//! https://cloud.google.com/identity-platform/docs/reference/rest/v1/projects.accounts/lookup,
//! https://cloud.google.com/identity-platform/docs/reference/rest/v1/UserInfo,
//! https://cloud.google.com/run/docs/securing/service-identity.
//! Uses the attached Cloud Run service identity, not JSON keys, user ADC or gcloud.
//! Operators must grant firebaseauth.users.get on the explicit target project;
//! the metadata access token has cloud-platform scope, with IAM limiting authority.

use super::{IdentityError, ProviderState, RevocationCheck, Verifier};
use async_trait::async_trait;
use front_door::identity::Principal;
use jsonwebtoken::{Algorithm, DecodingKey, decode_header};
use reqwest::{Client, Response, header};
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tokio::sync::{Mutex, Semaphore};

const KEYS: &str =
    "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";
const ADMIN: &str = "https://identitytoolkit.googleapis.com/v1/projects/";
const METADATA: &str =
    "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
const TIMEOUT: Duration = Duration::from_secs(5);
const REFRESH_INTERVAL: Duration = Duration::from_secs(30);
const MAX_BODY: usize = 256 * 1024;
const MAX_KEY_AGE: u64 = 3600;

struct KeyCache {
    verifier: Option<(Arc<Verifier>, Instant)>,
    attempted: Option<Instant>,
}
struct Credential {
    // Deliberately no Debug implementation on credentials, requests or adapters.
    token: String,
    until: Instant,
}

/// Bounded Google transport with fixed endpoints and an explicit target project.
/// Construction performs no I/O. Production endpoint overrides are not supported.
/// Reuse one instance per process; caches and admission bounds are per instance.
pub struct GoogleIdentity {
    project: String,
    client: Client,
    keys: Mutex<KeyCache>,
    credential: Mutex<Option<Credential>>,
    requests: Semaphore,
    #[cfg(test)]
    endpoints: Option<(String, String, String)>,
}

impl GoogleIdentity {
    /// Configure a Google project ID (not a project number or arbitrary URL).
    /// Does not activate any router or change hosted readiness.
    pub fn new(project: String) -> Result<Self, IdentityError> {
        if !(6..=30).contains(&project.len())
            || !project.starts_with(|c: char| c.is_ascii_lowercase())
            || !project.ends_with(|c: char| c.is_ascii_lowercase() || c.is_ascii_digit())
            || !project
                .bytes()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
        {
            return Err(IdentityError::Unavailable);
        }
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .connect_timeout(Duration::from_secs(2))
            .timeout(TIMEOUT)
            .build()
            .map_err(|_| IdentityError::Unavailable)?;
        Ok(Self {
            project,
            client,
            keys: Mutex::new(KeyCache {
                verifier: None,
                attempted: None,
            }),
            credential: Mutex::new(None),
            requests: Semaphore::new(16),
            #[cfg(test)]
            endpoints: None,
        })
    }

    fn endpoints(&self) -> (String, String, String) {
        #[cfg(test)]
        if let Some(endpoints) = &self.endpoints {
            return endpoints.clone();
        }
        (
            KEYS.into(),
            format!("{ADMIN}{}/accounts:lookup", self.project),
            METADATA.into(),
        )
    }

    /// Verify signed identity and fresh provider revocation state. No durable
    /// account is created here; callers must still use the retained account owner.
    /// Unknown keys can cause at most one refresh per 30 seconds, shared with
    /// concurrent callers and failed refreshes. Expired keys never authorize.
    pub async fn verify(&self, token: &str) -> Result<Principal, IdentityError> {
        let _permit = self
            .requests
            .try_acquire()
            .map_err(|_| IdentityError::Unavailable)?;
        tokio::time::timeout(TIMEOUT, async {
            if token.len() > 16384 {
                return Err(IdentityError::Invalid);
            }
            let h = decode_header(token).map_err(|_| IdentityError::Invalid)?;
            if h.alg != Algorithm::RS256 {
                return Err(IdentityError::Invalid);
            }
            let kid = h
                .kid
                .filter(|k| !k.is_empty() && k.len() <= 128)
                .ok_or(IdentityError::Invalid)?;
            let verifier = self.verifier(&kid).await?;
            verifier.verify(token, unix_now()?, &AdminCheck(self)).await
        })
        .await
        .map_err(|_| IdentityError::Unavailable)?
    }

    async fn verifier(&self, kid: &str) -> Result<Arc<Verifier>, IdentityError> {
        // Hold the mutex through refresh: a single flight, bounded by the outer
        // deadline. A cancelled refresh keeps its attempt timestamp (no storm).
        let mut cache = self.keys.lock().await;
        if let Some((v, until)) = &cache.verifier
            && Instant::now() < *until
            && v.keys.contains_key(kid)
        {
            return Ok(v.clone());
        }
        if cache
            .attempted
            .is_some_and(|t| t.elapsed() < REFRESH_INTERVAL)
        {
            return Err(IdentityError::Unavailable);
        }
        cache.attempted = Some(Instant::now());
        let start = Instant::now();
        let now = unix_now()?;
        let response = self
            .client
            .get(self.endpoints().0)
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        let age = freshness(response.headers())?;
        let bytes = bounded(response, MAX_BODY).await?;
        let pems: BTreeMap<String, String> =
            serde_json::from_slice(&bytes).map_err(|_| IdentityError::Unavailable)?;
        if pems.is_empty() || pems.len() > 16 {
            return Err(IdentityError::Unavailable);
        }
        let mut keys = BTreeMap::new();
        for (id, pem) in pems {
            if id.is_empty() || id.len() > 128 || !pem.starts_with("-----BEGIN CERTIFICATE-----") {
                return Err(IdentityError::Unavailable);
            }
            keys.insert(
                id,
                DecodingKey::from_rsa_pem(pem.as_bytes())
                    .map_err(|_| IdentityError::Unavailable)?,
            );
        }
        let verifier = Arc::new(Verifier::new(self.project.clone(), keys, now + age as i64)?);
        let until = start + Duration::from_secs(age);
        if Instant::now() >= until {
            return Err(IdentityError::Unavailable);
        }
        cache.verifier = Some((verifier.clone(), until));
        Ok(verifier)
    }

    async fn access_token(&self) -> Result<String, IdentityError> {
        let mut cache = self.credential.lock().await;
        if let Some(c) = &*cache
            && Instant::now() < c.until
        {
            return Ok(c.token.clone());
        }
        let start = Instant::now();
        let response = self
            .client
            .get(self.endpoints().2)
            .header("Metadata-Flavor", "Google")
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        if response
            .headers()
            .get("Metadata-Flavor")
            .and_then(|v| v.to_str().ok())
            != Some("Google")
        {
            return Err(IdentityError::Unavailable);
        }
        #[derive(Deserialize)]
        struct Token {
            access_token: String,
            token_type: String,
            expires_in: u64,
        }
        let token: Token = serde_json::from_slice(&bounded(response, 16384).await?)
            .map_err(|_| IdentityError::Unavailable)?;
        if token.token_type != "Bearer"
            || token.access_token.is_empty()
            || token.access_token.len() > 8192
            || !token.access_token.bytes().all(|b| b.is_ascii_graphic())
            || token.expires_in <= 60
        {
            return Err(IdentityError::Unavailable);
        }
        let until = start + Duration::from_secs(token.expires_in.min(3600) - 60);
        if Instant::now() >= until {
            return Err(IdentityError::Unavailable);
        }
        let value = token.access_token.clone();
        *cache = Some(Credential {
            token: token.access_token,
            until,
        });
        Ok(value)
    }

    async fn lookup(&self, project: &str, subject: &str) -> Result<ProviderState, IdentityError> {
        if project != self.project
            || subject.is_empty()
            || subject.len() > 128
            || subject.trim() != subject
        {
            return Err(IdentityError::Invalid);
        }
        let token = self.access_token().await?;
        let response = self
            .client
            .post(self.endpoints().1)
            .bearer_auth(token)
            .json(&serde_json::json!({"localId": [subject]}))
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            *self.credential.lock().await = None;
        }
        #[derive(Deserialize)]
        struct Lookup {
            #[serde(default)]
            users: Vec<User>,
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct User {
            local_id: String,
            valid_since: String,
            // Proto JSON omits false scalar defaults; omission means enabled.
            #[serde(default)]
            disabled: bool,
            tenant_id: Option<String>,
        }
        let lookup: Lookup = serde_json::from_slice(&bounded(response, MAX_BODY).await?)
            .map_err(|_| IdentityError::Unavailable)?;
        if lookup.users.is_empty() {
            return Err(IdentityError::Revoked);
        }
        if lookup.users.len() != 1 {
            return Err(IdentityError::Unavailable);
        }
        let user = lookup
            .users
            .into_iter()
            .next()
            .ok_or(IdentityError::Unavailable)?;
        if user.local_id != subject || user.tenant_id.is_some() {
            return Err(IdentityError::Unavailable);
        }
        let valid_since = user
            .valid_since
            .parse::<i64>()
            .map_err(|_| IdentityError::Unavailable)?;
        if valid_since < 0 {
            return Err(IdentityError::Unavailable);
        }
        // The response has no project field. Its authority is the fixed TLS host
        // plus explicit project in the request path; never invent a response claim.
        Ok(ProviderState {
            project: self.project.clone(),
            subject: user.local_id,
            disabled: user.disabled,
            valid_since,
        })
    }
}

// Avoid reacquiring admission in verify; standalone RevocationCheck calls below
// have the same bounds. No provider state cache: revocation is read per request.
struct AdminCheck<'a>(&'a GoogleIdentity);
#[async_trait]
impl RevocationCheck for AdminCheck<'_> {
    async fn current(&self, project: &str, subject: &str) -> Result<ProviderState, IdentityError> {
        self.0.lookup(project, subject).await
    }
}
#[async_trait]
impl RevocationCheck for GoogleIdentity {
    async fn current(&self, project: &str, subject: &str) -> Result<ProviderState, IdentityError> {
        let _permit = self
            .requests
            .try_acquire()
            .map_err(|_| IdentityError::Unavailable)?;
        tokio::time::timeout(TIMEOUT, self.lookup(project, subject))
            .await
            .map_err(|_| IdentityError::Unavailable)?
    }
}

fn unix_now() -> Result<i64, IdentityError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .map_err(|_| IdentityError::Unavailable)
}

fn freshness(headers: &header::HeaderMap) -> Result<u64, IdentityError> {
    let raw = headers
        .get(header::CACHE_CONTROL)
        .and_then(|v| v.to_str().ok())
        .ok_or(IdentityError::Unavailable)?;
    let mut max_age = None;
    for directive in raw.split(',').map(str::trim) {
        if ["no-store", "no-cache"].iter().any(|d| {
            directive
                .split('=')
                .next()
                .unwrap_or("")
                .trim()
                .eq_ignore_ascii_case(d)
        }) {
            return Err(IdentityError::Unavailable);
        }
        if let Some((name, value)) = directive.split_once('=')
            && name.eq_ignore_ascii_case("max-age")
        {
            if max_age.is_some() {
                return Err(IdentityError::Unavailable);
            }
            max_age = Some(
                value
                    .trim_matches('"')
                    .parse::<u64>()
                    .map_err(|_| IdentityError::Unavailable)?,
            );
        }
    }
    let age = match headers.get(header::AGE) {
        None => 0,
        Some(v) => v
            .to_str()
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .ok_or(IdentityError::Unavailable)?,
    };
    let remaining = max_age
        .ok_or(IdentityError::Unavailable)?
        .saturating_sub(age)
        .min(MAX_KEY_AGE);
    if remaining == 0 {
        return Err(IdentityError::Unavailable);
    }
    Ok(remaining)
}

async fn bounded(mut response: Response, limit: usize) -> Result<Vec<u8>, IdentityError> {
    if !response.status().is_success()
        || response.content_length().is_some_and(|n| n > limit as u64)
    {
        return Err(IdentityError::Unavailable);
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| IdentityError::Unavailable)?
    {
        if chunk.len() > limit - bytes.len() {
            return Err(IdentityError::Unavailable);
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests;
