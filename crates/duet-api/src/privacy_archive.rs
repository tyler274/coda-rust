//! GCS receipt transport using only the attached runtime service identity.
//! Conditional creation is not a retention lock or bucket/IAM qualification.
//! No list/delete/overwrite APIs, ADC files, endpoint environment overrides,
//! redirects, proxies, retries, or credential-bearing errors are supported.
use async_trait::async_trait;
use front_door::journal::{ArchiveUnavailable, ReceiptArchive};
use reqwest::{Client, Response, Url, header};
use serde::Deserialize;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, Semaphore};
use uuid::Uuid;

const METADATA: &str =
    "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
const STORAGE: &str = "https://storage.googleapis.com/";
const TIMEOUT: Duration = Duration::from_secs(10);
static REQUESTS: Semaphore = Semaphore::const_new(4);

struct Credential {
    token: String,
    until: Instant,
}
struct Credentials {
    value: Option<Credential>,
    attempted: Option<Instant>,
}
/// Fixed trusted GCS transport. Reuse one instance per job/process. Construction
/// performs no I/O. Bucket provisioning/retention/coverage are operator gates.
/// Deliberately does not implement Debug (nor do cached credentials).
pub struct GcsReceiptArchive {
    bucket: String,
    client: Client,
    credentials: Mutex<Credentials>,
    #[cfg(test)]
    endpoints: Option<(String, String)>,
}
impl GcsReceiptArchive {
    /// Accept a conservative DNS-compatible bucket name, never a URL or path.
    /// Uses the attached service account; JSON keys/user ADC are never read.
    pub fn new(bucket: String) -> Result<Self, ArchiveUnavailable> {
        if !(3..=63).contains(&bucket.len())
            || !bucket.starts_with(|c: char| c.is_ascii_lowercase() || c.is_ascii_digit())
            || !bucket.ends_with(|c: char| c.is_ascii_lowercase() || c.is_ascii_digit())
            || !bucket
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        {
            return Err(ArchiveUnavailable);
        }
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .no_proxy()
            .connect_timeout(Duration::from_secs(2))
            .timeout(TIMEOUT)
            .build()
            .map_err(|_| ArchiveUnavailable)?;
        Ok(Self {
            bucket,
            client,
            credentials: Mutex::new(Credentials {
                value: None,
                attempted: None,
            }),
            #[cfg(test)]
            endpoints: None,
        })
    }
    fn endpoints(&self) -> (&str, &str) {
        #[cfg(test)]
        if let Some((metadata, storage)) = &self.endpoints {
            return (metadata, storage);
        }
        (METADATA, STORAGE)
    }
    fn object_url(&self, key: Uuid, create: bool) -> Result<Url, ArchiveUnavailable> {
        let mut url = Url::parse(self.endpoints().1).map_err(|_| ArchiveUnavailable)?;
        let name = format!("receipts/duet-rust-luna-v1/{key}.json");
        {
            let mut segments = url.path_segments_mut().map_err(|_| ArchiveUnavailable)?;
            segments.pop_if_empty();
            if create {
                segments.push("upload");
            }
            segments.extend(["storage", "v1", "b", &self.bucket, "o"]);
            if !create {
                segments.push(&name);
            }
        }
        if create {
            url.query_pairs_mut()
                .append_pair("uploadType", "media")
                .append_pair("ifGenerationMatch", "0")
                .append_pair("name", &name);
        } else {
            url.query_pairs_mut().append_pair("alt", "media");
        }
        Ok(url)
    }
    async fn credential(&self) -> Result<String, ArchiveUnavailable> {
        let mut cache = self.credentials.lock().await;
        if let Some(value) = &cache.value
            && Instant::now() < value.until
        {
            return Ok(value.token.clone());
        }
        if cache
            .attempted
            .is_some_and(|at| at.elapsed() < Duration::from_secs(30))
        {
            return Err(ArchiveUnavailable);
        }
        let start = Instant::now();
        cache.attempted = Some(start);
        let response = self
            .client
            .get(self.endpoints().0)
            .header("Metadata-Flavor", "Google")
            .send()
            .await
            .map_err(|_| ArchiveUnavailable)?;
        if response
            .headers()
            .get("Metadata-Flavor")
            .and_then(|v| v.to_str().ok())
            != Some("Google")
        {
            return Err(ArchiveUnavailable);
        }
        #[derive(Deserialize)]
        struct Token {
            access_token: String,
            token_type: String,
            expires_in: u64,
        }
        let bytes = bounded(response, 16 * 1024).await?;
        let token: Token = serde_json::from_slice(&bytes).map_err(|_| ArchiveUnavailable)?;
        if token.token_type != "Bearer"
            || !(61..=3600).contains(&token.expires_in)
            || token.access_token.is_empty()
            || token.access_token.len() > 8192
            || !token.access_token.bytes().all(|b| b.is_ascii_graphic())
        {
            return Err(ArchiveUnavailable);
        }
        let until = start + Duration::from_secs(token.expires_in - 60);
        if Instant::now() >= until {
            return Err(ArchiveUnavailable);
        }
        cache.value = Some(Credential {
            token: token.access_token.clone(),
            until,
        });
        Ok(token.access_token)
    }
    async fn request(&self, key: Uuid, body: Option<&[u8]>) -> Result<Vec<u8>, ArchiveUnavailable> {
        let _permit = REQUESTS.try_acquire().map_err(|_| ArchiveUnavailable)?;
        if key.is_nil() || body.is_some_and(|b| b.len() > 4096) {
            return Err(ArchiveUnavailable);
        }
        tokio::time::timeout(TIMEOUT, async {
            let token = self.credential().await?;
            let mut authorization = header::HeaderValue::from_str(&format!("Bearer {token}"))
                .map_err(|_| ArchiveUnavailable)?;
            authorization.set_sensitive(true);
            let url = self.object_url(key, body.is_some())?;
            let request = match body {
                Some(bytes) => self
                    .client
                    .post(url)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(bytes.to_vec()),
                None => self.client.get(url),
            };
            let response = request
                .header(header::AUTHORIZATION, authorization)
                .send()
                .await
                .map_err(|_| ArchiveUnavailable)?;
            // Never retain error response bodies, URLs, metadata tokens or provider
            // diagnostics. Any upload uncertainty is resolved by owner readback.
            bounded(response, if body.is_some() { 16 * 1024 } else { 4096 }).await
        })
        .await
        .map_err(|_| ArchiveUnavailable)?
    }
}
async fn bounded(mut response: Response, limit: usize) -> Result<Vec<u8>, ArchiveUnavailable> {
    if !response.status().is_success()
        || response.content_length().is_some_and(|n| n > limit as u64)
    {
        return Err(ArchiveUnavailable);
    }
    let mut result = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|_| ArchiveUnavailable)? {
        if chunk.len() > limit.saturating_sub(result.len()) {
            return Err(ArchiveUnavailable);
        }
        result.extend_from_slice(&chunk);
    }
    Ok(result)
}
#[async_trait]
impl ReceiptArchive for GcsReceiptArchive {
    async fn create(&self, key: Uuid, bytes: &[u8]) -> Result<(), ArchiveUnavailable> {
        self.request(key, Some(bytes)).await.map(|_| ())
    }
    async fn read(&self, key: Uuid) -> Result<Vec<u8>, ArchiveUnavailable> {
        self.request(key, None).await
    }
}

#[cfg(test)]
mod tests;
