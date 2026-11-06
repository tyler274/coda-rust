//! Authenticated durable doctor-page controls. Capability URLs never appear in Debug or errors.
use crate::DuetClient;
use duet_wire::models::{CanonicalRecordSelection, DoctorCollection, DoctorPageSave};
use serde::{Deserialize, Serialize};

/// Closed failure states; neither response prose nor credential-bearing URLs are logged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoctorError {
    /// Sign-in expired; never automatically replay mutations after refresh.
    Unauthorized,
    /// Server does not advertise/acknowledge atomic UUID preconditions; never retry unconditionally.
    UnsupportedPrecondition,
    /// Permission or accepted terms required.
    Forbidden,
    /// Generation, proposal, source or idempotency bytes changed.
    Conflict,
    /// No such receipt/resource.
    NotFound,
    /// Invalid selection, expiry or command headers.
    Invalid,
    /// Transport, server, receipt confirmation or decode failure: mutation outcome may be uncertain.
    Uncertain,
}
impl std::fmt::Display for DoctorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::UnsupportedPrecondition => "This server has not confirmed conditional revocation support. No unconditional retry will be sent; check the active publication before proceeding.",
            Self::Unauthorized => "Sign in again. Pending operations were not replayed.",
            Self::Forbidden => "This operation needs current account permission or terms.",
            Self::Conflict => "The page, selected sources, or operation changed. Review the current page; your draft is retained.",
            Self::NotFound => "No receipt or resource was found. This alone does not prove an uncertain write failed.",
            Self::Invalid => "The request is invalid. Review the selection and expiry.",
            Self::Uncertain => "Duet could not confirm the outcome. Retry the same operation, not a new one.",
        })
    }
}
impl std::error::Error for DoctorError {}
/// Exact command retained across uncertain acknowledgements. Contains no capability or patient prose.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishRequest {
    /// Generate once per explicit approval; reuse identical bytes on retry.
    pub operation_id: uuid::Uuid,
    /// Reviewed approved page generation.
    pub expected_generation: i64,
    /// Sixty seconds to thirty days; UI defaults to seven days.
    pub expires_in_seconds: i64,
}
/// Patient-only capability response. Intentionally no Debug implementation.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationView {
    /// Publication identity.
    pub id: uuid::Uuid,
    /// Pinned approved generation.
    pub generation: i64,
    /// Server-authoritative RFC3339 expiry.
    pub expires_at: String,
    /// Revocation state.
    pub revoked: bool,
    /// Account-free web capability; never persist to browser storage/telemetry.
    pub web_url: String,
    /// SMART Health Link, key only in fragment; never persist to logs.
    pub shl_url: String,
    /// Digest of exact published PDF.
    pub content_sha256: String,
}
/// Persistent intent records only, never link URLs or PDF bytes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PendingDoctorOperation {
    /// Explicit pinned publication command.
    Publish(PublishRequest),
    /// Retry the same privacy deletion event and generation.
    Clear {
        /// Stable privacy event UUID.
        operation_id: uuid::Uuid,
        /// Original generation, never rebased on retry.
        expected_generation: i64,
    },
    /// Revoke command and original target guard.
    Revoke {
        /// Stable privacy event UUID.
        operation_id: uuid::Uuid,
        /// Current API revokes active publication; guard readback before retry.
        publication_id: uuid::Uuid,
    },
}
/// Exact canonical address retaining the original arbitrary source identifier.
#[derive(Clone, PartialEq, Deserialize)]
pub struct DoctorSourceRevision {
    /// Stable canonical identity used for selection.
    pub canonical_record_id: uuid::Uuid,
    /// Source family.
    pub entity: duet_wire::models::addressed_record_revision::Entity,
    /// Literal original source identity, not necessarily a UUID.
    pub record_id: String,
    /// Immutable source revision.
    pub revision: i32,
}
/// Exact canonical inventory returned by the durable evidence owner, independent of grouped library projections.
#[derive(Clone, PartialEq, Deserialize)]
pub struct DoctorSourceInventory {
    /// Continuation for the same account/revision/query, absent after completion.
    #[serde(default)]
    pub next_cursor: Option<String>,
    /// Account evidence revision for the complete inventory.
    #[serde(default)]
    pub revision: i64,
    /// Canonical identity and revision of each payload.
    pub canonical_revisions: Vec<DoctorSourceRevision>,
    /// Literal current evidence payloads, in canonical address order for this entity.
    pub health_evidence: Vec<serde_json::Value>,
    /// Literal current clinical-note payloads, in canonical address order for this entity.
    pub clinical_notes: Vec<serde_json::Value>,
}
/// Immutable health-context snapshot. Preserve all literal fact fields, including source coverage.
#[derive(Clone, PartialEq, Deserialize)]
pub struct DoctorDeviceSnapshot {
    /// Server-stored immutable snapshot identity.
    pub snapshot_id: uuid::Uuid,
    /// Server-stored generation time.
    pub generated_at: String,
    /// Complete literal facts, not synthetic replacements or summary-only projections.
    pub facts: Vec<serde_json::Value>,
}
impl DuetClient {
    /// Read the actual latest immutable device snapshot; 404 is explicit absence.
    pub async fn doctor_device_snapshot(&self) -> Result<DoctorDeviceSnapshot, DoctorError> {
        self.doctor_json(
            self.configuration
                .client
                .get(format!(
                    "{}/api/health-context",
                    self.configuration.base_path
                ))
                .bearer_auth(
                    self.configuration
                        .bearer_access_token
                        .as_deref()
                        .unwrap_or_default(),
                )
                .header(reqwest::header::CACHE_CONTROL, "no-store"),
        )
        .await
    }
    /// Existing fact selection is latest-snapshot based, NOT atomically snapshot-conditional.
    /// Preflight and validate the actual returned pinned snapshot; a race must force page review.
    pub async fn select_doctor_device_fact(
        &self,
        reviewed: &DoctorDeviceSnapshot,
        fact_id: &str,
    ) -> Result<DoctorCollection, DoctorError> {
        let fact = reviewed
            .facts
            .iter()
            .find(|f| f["id"].as_str() == Some(fact_id))
            .ok_or(DoctorError::Invalid)?;
        if self.doctor_device_snapshot().await? != *reviewed {
            return Err(DoctorError::Conflict);
        }
        let mut url = reqwest::Url::parse(&format!(
            "{}/api/doctor-collection/facts/",
            self.configuration.base_path
        ))
        .map_err(|_| DoctorError::Invalid)?;
        url.path_segments_mut()
            .map_err(|_| DoctorError::Invalid)?
            .pop_if_empty()
            .push(fact_id);
        let value: serde_json::Value = self
            .doctor_json(
                self.configuration
                    .client
                    .put(url)
                    .bearer_auth(
                        self.configuration
                            .bearer_access_token
                            .as_deref()
                            .unwrap_or_default(),
                    )
                    .header(reqwest::header::CACHE_CONTROL, "no-store"),
            )
            .await?;
        let matches = value["items"].as_array().is_some_and(|items| {
            items.iter().any(|item| {
                item["fact_snapshot_id"].as_str() == Some(reviewed.snapshot_id.to_string().as_str())
                    && item["fact_snapshot"] == *fact
            })
        });
        if !matches {
            return Err(DoctorError::Conflict);
        }
        serde_json::from_value(value).map_err(|_| DoctorError::Uncertain)
    }
    /// Read current canonical revisions for explicit patient selection; never infer revision from a list title.
    pub async fn doctor_source_inventory(&self) -> Result<DoctorSourceInventory, DoctorError> {
        let mut result: Option<DoctorSourceInventory> = None;
        let mut cursor: Option<String> = None;
        let mut seen = std::collections::HashSet::new();
        loop {
            let mut url = reqwest::Url::parse(&format!(
                "{}/api/evidence/records",
                self.configuration.base_path
            ))
            .map_err(|_| DoctorError::Invalid)?;
            if let Some(cursor) = &cursor {
                url.query_pairs_mut().append_pair("cursor", cursor);
            }
            let page: DoctorSourceInventory = self
                .doctor_json(
                    self.configuration
                        .client
                        .get(url)
                        .bearer_auth(
                            self.configuration
                                .bearer_access_token
                                .as_deref()
                                .unwrap_or_default(),
                        )
                        .header(reqwest::header::CACHE_CONTROL, "no-store"),
                )
                .await?;
            cursor = page.next_cursor.clone();
            if let Some(result) = &mut result {
                if result.revision != page.revision {
                    return Err(DoctorError::Conflict);
                }
                result.canonical_revisions.extend(page.canonical_revisions);
                result.health_evidence.extend(page.health_evidence);
                result.clinical_notes.extend(page.clinical_notes);
                result.next_cursor = cursor.clone();
            } else {
                result = Some(page);
            }
            match &cursor {
                None => return result.ok_or(DoctorError::Uncertain),
                Some(cursor) if !seen.insert(cursor.clone()) => return Err(DoctorError::Conflict),
                _ => {}
            }
        }
    }
    /// Resolve every member before offering a grouped selection. The returned
    /// records are exact, never the representative row or a partial first page.
    pub async fn doctor_group_inventory(
        &self,
        kind: &str,
        anchor: uuid::Uuid,
    ) -> Result<DoctorSourceInventory, DoctorError> {
        let mut ids = std::collections::HashSet::new();
        let mut cursor = None::<String>;
        let mut revision = None;
        let mut seen = std::collections::HashSet::new();
        let total = loop {
            let mut url = reqwest::Url::parse(&format!(
                "{}/api/evidence/library",
                self.configuration.base_path
            ))
            .map_err(|_| DoctorError::Invalid)?;
            url.query_pairs_mut()
                .append_pair("scope", "completeRecord")
                .append_pair("group_kind", kind)
                .append_pair("anchor_record_id", &anchor.to_string());
            if let Some(cursor) = &cursor {
                url.query_pairs_mut().append_pair("cursor", cursor);
            }
            let page: serde_json::Value = self
                .doctor_json(
                    self.configuration
                        .client
                        .get(url)
                        .bearer_auth(
                            self.configuration
                                .bearer_access_token
                                .as_deref()
                                .unwrap_or_default(),
                        )
                        .header(reqwest::header::CACHE_CONTROL, "no-store"),
                )
                .await?;
            let current = page["revision"].as_i64().ok_or(DoctorError::Uncertain)?;
            if revision.is_some_and(|r| r != current) {
                return Err(DoctorError::Conflict);
            }
            revision = Some(current);
            for entry in page["entries"].as_array().ok_or(DoctorError::Uncertain)? {
                let id: uuid::Uuid = serde_json::from_value(entry["record_id"].clone())
                    .map_err(|_| DoctorError::Uncertain)?;
                if !ids.insert(id) {
                    return Err(DoctorError::Conflict);
                }
            }
            cursor = page["next_cursor"].as_str().map(str::to_owned);
            if let Some(cursor) = &cursor {
                if !seen.insert(cursor.clone()) {
                    return Err(DoctorError::Conflict);
                }
            } else {
                break page["total_count"].as_u64().ok_or(DoctorError::Uncertain)? as usize;
            }
        };
        if ids.len() != total || !ids.contains(&anchor) {
            return Err(DoctorError::Conflict);
        }
        let mut inventory = self.doctor_source_inventory().await?;
        if Some(inventory.revision) != revision {
            return Err(DoctorError::Conflict);
        }
        // Payload arrays are entity-ordered, matching the address order within
        // each family. Preserve literal original identifiers; never join titles.
        let keep = |entity, values: Vec<serde_json::Value>| {
            inventory
                .canonical_revisions
                .iter()
                .filter(|a| a.entity == entity)
                .zip(values)
                .filter_map(|(a, p)| ids.contains(&a.canonical_record_id).then_some(p))
                .collect()
        };
        inventory.health_evidence = keep(
            duet_wire::models::addressed_record_revision::Entity::HealthEvidence,
            inventory.health_evidence.clone(),
        );
        inventory.clinical_notes = keep(
            duet_wire::models::addressed_record_revision::Entity::ClinicalNote,
            inventory.clinical_notes.clone(),
        );
        inventory
            .canonical_revisions
            .retain(|a| ids.contains(&a.canonical_record_id));
        if inventory.canonical_revisions.len() != total {
            return Err(DoctorError::Conflict);
        }
        Ok(inventory)
    }
    fn doctor_request(&self, method: reqwest::Method, suffix: &str) -> reqwest::RequestBuilder {
        self.configuration
            .client
            .request(
                method,
                format!(
                    "{}/api/doctor-collection{}",
                    self.configuration.base_path, suffix
                ),
            )
            .bearer_auth(
                self.configuration
                    .bearer_access_token
                    .as_deref()
                    .unwrap_or_default(),
            )
            .header(reqwest::header::CACHE_CONTROL, "no-store")
    }
    /// Current durable collection, not an approval.
    pub async fn doctor_workspace(&self) -> Result<DoctorCollection, DoctorError> {
        self.doctor_json(self.doctor_request(reqwest::Method::GET, ""))
            .await
    }
    /// Submit exactly the original save DTO; retries preserve generation/proposal/membership.
    pub async fn save_doctor_workspace(
        &self,
        save: &DoctorPageSave,
    ) -> Result<DoctorCollection, DoctorError> {
        self.doctor_json(
            self.doctor_request(reqwest::Method::PUT, "/page")
                .json(save),
        )
        .await
    }
    /// Immutable result of the particular generation's save, not a mutable collection inference.
    pub async fn doctor_save_receipt(
        &self,
        expected: i32,
    ) -> Result<DoctorCollection, DoctorError> {
        self.doctor_json(
            self.doctor_request(reqwest::Method::GET, &format!("/save-receipts/{expected}")),
        )
        .await
    }
    /// Reject only the reviewed proposal and base generation.
    pub async fn reject_doctor_proposal(
        &self,
        id: uuid::Uuid,
        generation: i32,
    ) -> Result<DoctorCollection, DoctorError> {
        self.doctor_json(
            self.doctor_request(reqwest::Method::POST, &format!("/proposals/{id}/reject"))
                .json(&serde_json::json!({"expected_generation":generation})),
        )
        .await
    }
    /// Select an addressed canonical record using the existing exact revision DTO.
    pub async fn select_doctor_record(
        &self,
        selection: &CanonicalRecordSelection,
    ) -> Result<DoctorCollection, DoctorError> {
        self.doctor_json(
            self.doctor_request(reqwest::Method::PUT, "/record")
                .json(selection),
        )
        .await
    }
    /// Source-currentness check of the reviewed approved generation.
    pub async fn check_doctor_workspace(
        &self,
        generation: i32,
    ) -> Result<DoctorCollection, DoctorError> {
        self.doctor_json(
            self.doctor_request(reqwest::Method::POST, "/page/share")
                .json(&serde_json::json!({"expected_generation":generation})),
        )
        .await
    }
    /// Source-validated server PDF. Caller must compare against reviewed generation/content before sharing.
    pub async fn doctor_pdf(&self) -> Result<Vec<u8>, DoctorError> {
        let response = self
            .doctor_checked(self.doctor_request(reqwest::Method::GET, "/pdf"))
            .await?;
        if response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            != Some("application/pdf")
        {
            return Err(DoctorError::Uncertain);
        }
        response
            .bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|_| DoctorError::Uncertain)
    }
    /// Server SVG preview; never grants export/publication authority.
    pub async fn doctor_preview_svg(&self) -> Result<String, DoctorError> {
        self.doctor_checked(self.doctor_request(reqwest::Method::GET, "/preview"))
            .await?
            .text()
            .await
            .map_err(|_| DoctorError::Uncertain)
    }
    /// Confirmed Start fresh with durable privacy-event retry identity.
    pub async fn clear_doctor_workspace(
        &self,
        operation_id: uuid::Uuid,
        generation: i64,
    ) -> Result<(), DoctorError> {
        self.doctor_checked(
            self.doctor_request(reqwest::Method::DELETE, "")
                .header("Idempotency-Key", operation_id.to_string())
                .header("If-Match", generation.to_string()),
        )
        .await
        .map(|_| ())
    }
    /// Explicit publication. Do not generate an operation ID inside this method.
    pub async fn create_doctor_share_link(
        &self,
        request: &PublishRequest,
    ) -> Result<PublicationView, DoctorError> {
        self.doctor_json(
            self.doctor_request(reqwest::Method::POST, "/share-link")
                .json(request),
        )
        .await
    }
    /// Current active/revoked publication readback, or no publication.
    pub async fn doctor_share_status(&self) -> Result<Option<PublicationView>, DoctorError> {
        self.doctor_json(self.doctor_request(reqwest::Method::GET, "/share-link"))
            .await
    }
    /// Revoke using the same privacy-event UUID after an uncertain acknowledgement.
    pub async fn revoke_doctor_share_link(
        &self,
        operation_id: uuid::Uuid,
        publication_id: uuid::Uuid,
    ) -> Result<(), DoctorError> {
        // Old servers ignore unknown If-Match headers. Do not dispatch a destructive request
        // until the server explicitly advertises the atomic UUID protocol.
        let capability = self
            .doctor_checked(self.doctor_request(reqwest::Method::GET, "/share-link"))
            .await?;
        if capability
            .headers()
            .get("Duet-Publication-Precondition")
            .and_then(|v| v.to_str().ok())
            != Some("uuid-v1")
        {
            return Err(DoctorError::UnsupportedPrecondition);
        }
        let response = self
            .doctor_checked(
                self.doctor_request(reqwest::Method::DELETE, "/share-link")
                    .header("Idempotency-Key", operation_id.to_string())
                    .header("If-Match", publication_id.to_string()),
            )
            .await?;
        if response
            .headers()
            .get("Duet-Publication-Precondition")
            .and_then(|v| v.to_str().ok())
            != Some("uuid-v1")
        {
            return Err(DoctorError::UnsupportedPrecondition);
        }
        Ok(())
    }
}
impl DuetClient {
    async fn doctor_checked(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, DoctorError> {
        let response = request.send().await.map_err(|_| DoctorError::Uncertain)?;
        match response.status().as_u16() {
            200..=299 => Ok(response),
            401 => {
                let status = response.status();
                let body = response.text().await.map_err(|_| DoctorError::Uncertain)?;
                self.observe_refusal(status, &body);
                if crate::refusal::classify(status, &body).is_some() {
                    Err(DoctorError::Unauthorized)
                } else {
                    Err(DoctorError::Uncertain)
                }
            }
            403 => Err(DoctorError::Forbidden),
            404 => Err(DoctorError::NotFound),
            409 | 412 => Err(DoctorError::Conflict),
            400 | 422 => Err(DoctorError::Invalid),
            405 | 428 | 501 => Err(DoctorError::UnsupportedPrecondition),
            _ => Err(DoctorError::Uncertain),
        }
    }
    async fn doctor_json<T: serde::de::DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<T, DoctorError> {
        self.doctor_checked(request)
            .await?
            .json()
            .await
            .map_err(|_| DoctorError::Uncertain)
    }
}
