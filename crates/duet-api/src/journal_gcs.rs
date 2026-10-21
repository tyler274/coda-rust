//! GCS create-only privacy receipt adapter. Production bucket retention/IAM is an operator gate.
use crate::{ApiError, privacy::ReceiptJournal};
use async_trait::async_trait;
use uuid::Uuid;

/// Independent GCS journal using the Cloud Run/Compute attached service account.
/// No bearer, token, patient prose or SDK error text enters the receipt/logs.
pub struct GcsJournal {
    client: reqwest::Client,
    bucket: String,
}
impl GcsJournal {
    /// Configure the existing privacy bucket. No network access happens at construction.
    pub fn new(bucket: String) -> Result<Self, ApiError> {
        if bucket.len() < 3
            || !bucket
                .bytes()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || b"-_.".contains(&c))
        {
            return Err(ApiError::Unavailable(
                "invalid privacy bucket configuration",
            ));
        }
        Ok(Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .map_err(|_| unavailable())?,
            bucket,
        })
    }
    async fn token(&self) -> Result<String, ApiError> {
        let response=self.client.get("http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token").header("Metadata-Flavor","Google").send().await.map_err(|_|unavailable())?;
        if !response.status().is_success()
            || response
                .headers()
                .get("Metadata-Flavor")
                .and_then(|v| v.to_str().ok())
                != Some("Google")
        {
            return Err(unavailable());
        }
        let body: serde_json::Value = response.json().await.map_err(|_| unavailable())?;
        body["access_token"]
            .as_str()
            .map(str::to_owned)
            .ok_or_else(unavailable)
    }
}
fn unavailable() -> ApiError {
    ApiError::Unavailable("Independent privacy journal is unavailable")
}
#[async_trait]
impl ReceiptJournal for GcsJournal {
    async fn inventory(&self) -> Result<Vec<Uuid>, ApiError> {
        let mut page = String::new();
        let mut ids = Vec::new();
        loop {
            let token = self.token().await?;
            let response = self
                .client
                .get(format!(
                    "https://storage.googleapis.com/storage/v1/b/{}/o",
                    self.bucket
                ))
                .query(&[("maxResults", "1000"), ("pageToken", page.as_str())])
                .bearer_auth(token)
                .send()
                .await
                .map_err(|_| unavailable())?;
            if !response.status().is_success() {
                return Err(unavailable());
            }
            let body: serde_json::Value = response.json().await.map_err(|_| unavailable())?;
            for item in body["items"].as_array().into_iter().flatten() {
                let name = item["name"].as_str().ok_or_else(unavailable)?;
                let id = name
                    .strip_prefix("receipts/v1/")
                    .and_then(|s| s.strip_suffix(".json"))
                    .and_then(|s| s.parse().ok())
                    .ok_or_else(unavailable)?;
                ids.push(id);
            }
            match body["nextPageToken"].as_str() {
                Some(next) if !next.is_empty() => page = next.to_owned(),
                _ => break,
            }
        }
        Ok(ids)
    }
    async fn put_if_absent(&self, id: Uuid, bytes: &[u8]) -> Result<(), ApiError> {
        if bytes.len() > 4096 {
            return Err(unavailable());
        }
        let token = self.token().await?;
        let response = self
            .client
            .post(format!(
                "https://storage.googleapis.com/upload/storage/v1/b/{}/o",
                self.bucket
            ))
            .query(&[
                ("uploadType", "media"),
                ("name", &format!("receipts/v1/{id}.json")),
                ("ifGenerationMatch", "0"),
            ])
            .bearer_auth(token)
            .header("Content-Type", "application/json")
            .body(bytes.to_owned())
            .send()
            .await
            .map_err(|_| unavailable())?;
        // Existing object is reconciled by the caller's independent readback, never overwritten.
        if response.status().is_success()
            || response.status() == reqwest::StatusCode::PRECONDITION_FAILED
        {
            Ok(())
        } else {
            Err(unavailable())
        }
    }
    async fn read(&self, id: Uuid) -> Result<Option<Vec<u8>>, ApiError> {
        let token = self.token().await?;
        let response = self
            .client
            .get(format!(
                "https://storage.googleapis.com/storage/v1/b/{}/o/receipts%2Fv1%2F{id}.json",
                self.bucket
            ))
            .query(&[("alt", "media")])
            .bearer_auth(token)
            .header("Range", "bytes=0-4096")
            .send()
            .await
            .map_err(|_| unavailable())?;
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !response.status().is_success() {
            return Err(unavailable());
        }
        let bytes = response.bytes().await.map_err(|_| unavailable())?;
        if bytes.len() > 4096 {
            return Err(unavailable());
        }
        Ok(Some(bytes.to_vec()))
    }
}
