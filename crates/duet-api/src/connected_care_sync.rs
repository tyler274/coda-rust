//! Live Duet -> `front-door` shared-record uploader with original source attribution
//! and visible sync-failure reporting (`intent/connected-care.md` P8, P13, P15, C1.2.6,
//! C1.3.1, C1.3.4, C1.3.8, R1.3, Rule 10, Rule 12, Scope "Failures").
//!
//! Reads only the signed-in account's canonical evidence (`health_evidence` and
//! `clinical_notes`) through [`EvidenceStore`]; Duet's private AI memory never crosses.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use duet_wire::models::addressed_record_revision::Entity as EvidenceEntity;
use front_door::{ForMyDoctorError, FrontDoorStore, SharedPatientRecordInput};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::ApiError;
use crate::background;
use crate::database::{AccountDatabase, orm};
use crate::durable::DurableState;
use crate::evidence::{EvidenceChange, EvidenceSnapshot, EvidenceStore, ExactEvidence};
use crate::identity::{AccountContext, check_current};

mod front_door_mapping {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, sqlx::FromRow)]
    #[sea_orm(schema_name = "public", table_name = "duet_front_door_mapping")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub front_door_account_id: Uuid,
        pub user_id: Uuid,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// HTTP header carrying the patient's active `front-door` session token for sync routes.
pub const FRONT_DOOR_SESSION_HEADER: &str = "x-front-door-session";

/// Content-free telemetry event name recorded when a connected-care upload fails.
const CONNECTED_CARE_SYNC_FAILED: &str = "duet.connected_care_sync.failed";

/// Closed set of patient-facing source systems mapped once at the Duet boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DuetSourceOrigin {
    /// Epic / MyChart patient portal.
    Epic,
    /// Oracle Health (Cerner) patient portal.
    OracleHealth,
    /// Apple Health (HealthKit) phone readings.
    AppleHealth,
    /// Android Health Connect phone readings.
    HealthConnect,
    /// Apple Health Records clinical imports.
    AppleHealthRecords,
    /// healow patient portal.
    Healow,
    /// Google Health portal.
    GoogleHealth,
    /// Patient-entered medications in Duet.
    PatientMedications,
    /// Other patient-entered health record in Duet.
    PatientEntry,
}

impl DuetSourceOrigin {
    /// Human-readable source system label shown in `"from <source>, via the patient's Duet"`.
    pub const fn source_name(self) -> &'static str {
        match self {
            Self::Epic => "Epic",
            Self::OracleHealth => "Oracle Health",
            Self::AppleHealth => "Apple Health",
            Self::HealthConnect => "Health Connect",
            Self::AppleHealthRecords => "Apple Health Records",
            Self::Healow => "healow",
            Self::GoogleHealth => "Google Health",
            Self::PatientMedications => "patient medications",
            Self::PatientEntry => "patient entry",
        }
    }

    /// Formats the exact Connected Care origin attribution (`intent/connected-care.md` C1.3.4).
    pub fn via_duet_label(self) -> String {
        format!("from {}, via the patient's Duet", self.source_name())
    }

    /// Classifies a canonical Duet evidence payload's source system once at the boundary.
    pub fn from_payload(entity: EvidenceEntity, payload: &Value) -> Self {
        let source_kind = payload
            .get("source")
            .and_then(|source| source.get("kind"))
            .and_then(Value::as_str)
            .or_else(|| payload.get("source_kind").and_then(Value::as_str))
            .map(str::trim)
            .unwrap_or_default();

        match source_kind {
            "epic" => return Self::Epic,
            "oracleHealth" => return Self::OracleHealth,
            "appleHealth" => return Self::AppleHealth,
            "healthConnect" => return Self::HealthConnect,
            "healthRecords" => return Self::AppleHealthRecords,
            "healow" => return Self::Healow,
            "googleHealth" => return Self::GoogleHealth,
            _ => {}
        }

        let source_label = payload
            .get("source_label")
            .and_then(Value::as_str)
            .map(str::trim)
            .unwrap_or_default();
        if source_label.eq_ignore_ascii_case("Health Records") {
            return Self::AppleHealthRecords;
        }

        let concept = payload
            .get("concept_id")
            .and_then(Value::as_str)
            .map(str::trim)
            .unwrap_or_default()
            .to_ascii_lowercase();
        let resource_type = payload
            .get("resourceType")
            .or_else(|| payload.get("record_type"))
            .and_then(Value::as_str)
            .map(str::trim)
            .unwrap_or_default()
            .to_ascii_lowercase();
        if concept.contains("medication") || resource_type.contains("medication") {
            return Self::PatientMedications;
        }
        if entity == EvidenceEntity::ClinicalNote && source_label.eq_ignore_ascii_case("Epic") {
            return Self::Epic;
        }
        Self::PatientEntry
    }
}

/// Coarse, actionable reason why a Duet -> `front-door` sharing sync failed (`P15`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharingFailureReason {
    /// Patient's `front-door` session token was missing, invalid, or revoked.
    Unauthenticated,
    /// No active practice connection exists (left, discharged, or not yet joined).
    ConnectionEnded,
    /// `front-door` transport failed or timed out; safe to retry.
    TransportUnavailable,
    /// Reading canonical evidence from Duet storage failed.
    EvidenceReadFailed,
}

impl SharingFailureReason {
    /// Content-free telemetry error class matching [`background::failed`].
    pub const fn telemetry_class(self) -> &'static str {
        match self {
            Self::Unauthenticated => "unauthenticated",
            Self::ConnectionEnded => "connection_ended",
            Self::TransportUnavailable => "transport_unavailable",
            Self::EvidenceReadFailed => "evidence_read",
        }
    }
}

/// Failure returned by [`ConnectedCareUploader`] or [`FrontDoorSyncTransport`].
#[derive(Debug)]
pub struct ConnectedCareSyncError {
    /// Typed failure category.
    pub reason: SharingFailureReason,
    /// Plain-language operational reason suitable for status display.
    pub message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl ConnectedCareSyncError {
    /// Creates an explicit sync failure preserving an optional underlying cause.
    pub fn new(
        reason: SharingFailureReason,
        message: impl Into<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    ) -> Self {
        Self {
            reason,
            message: message.into(),
            source,
        }
    }
}

impl fmt::Display for ConnectedCareSyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.reason, self.message)
    }
}

impl std::error::Error for ConnectedCareSyncError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_deref()
            .map(|err| err as &(dyn std::error::Error + 'static))
    }
}

impl From<ConnectedCareSyncError> for ApiError {
    fn from(error: ConnectedCareSyncError) -> Self {
        match error.reason {
            SharingFailureReason::Unauthenticated => Self::Unauthorized,
            SharingFailureReason::ConnectionEnded => {
                Self::Conflict("no active practice connection for sharing")
            }
            SharingFailureReason::TransportUnavailable => {
                Self::Unavailable("Connected care sharing service is temporarily unavailable")
            }
            SharingFailureReason::EvidenceReadFailed => {
                Self::Unavailable("Duet evidence could not be read for sharing")
            }
        }
    }
}

/// Per-account sharing health surfaced in Duet (`intent/connected-care.md` P15, C1.3.8).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum SharingSyncStanding {
    /// No sharing upload has run yet for this account in this process.
    NeverSynced,
    /// Latest upload succeeded with all current consented items reflected in `front-door`.
    Current {
        /// When the last successful sync completed.
        synced_at: DateTime<Utc>,
        /// Duet evidence cursor included in that sync.
        cursor: i64,
        /// Number of shared records uploaded in that sync.
        record_count: usize,
    },
    /// Latest upload failed; carries the last known good date and plain reason.
    Degraded {
        /// When sharing last succeeded, if ever.
        last_succeeded_at: Option<DateTime<Utc>>,
        /// Evidence cursor from the last successful sync, if any.
        last_cursor: Option<i64>,
        /// When the latest failure occurred.
        failed_at: DateTime<Utc>,
        /// Number of consecutive failed sync attempts.
        consecutive_failures: u32,
        /// Typed reason for the failure.
        reason: SharingFailureReason,
        /// Plain explanation of why sharing is behind.
        message: String,
    },
}

/// Content-free operational alert emitted for Coda ops when sharing is degraded (`P15`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DegradedSharingAlert {
    /// Owning account identifier (no patient clinical content).
    pub account_id: Uuid,
    /// When the latest failure occurred.
    pub failed_at: DateTime<Utc>,
    /// Number of consecutive failures.
    pub consecutive_failures: u32,
    /// Typed failure category.
    pub reason: SharingFailureReason,
    /// When sharing last succeeded, if ever.
    pub last_succeeded_at: Option<DateTime<Utc>>,
}

/// Result returned when an upload pass succeeds.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharingSyncReport {
    /// Duet evidence cursor captured in this sync pass.
    pub cursor: i64,
    /// Number of consented records (including source corrections/retractions) uploaded.
    pub uploaded_count: usize,
    /// Number of non-shareable/private items excluded by `Rule 10`.
    pub excluded_private_count: usize,
    /// Timestamp when `front-door` acknowledged the upload.
    pub synced_at: DateTime<Utc>,
}

/// Transport sending Duet shared records to `PUT /front-door/shared-records`.
#[async_trait]
pub trait FrontDoorSyncTransport: Send + Sync {
    /// Verifies `front_door_bearer` belongs to `expected_front_door_account_id` and uploads
    /// `records` to that account only (`Rule 21`).
    async fn upload_shared_records(
        &self,
        expected_front_door_account_id: Uuid,
        front_door_bearer: &str,
        records: &[SharedPatientRecordInput],
    ) -> Result<(), ConnectedCareSyncError>;
}

#[derive(Deserialize)]
struct FrontDoorAccountIdentity {
    account_id: Uuid,
}

/// Bounded, no-redirect HTTPS transport to `GET /front-door/account` and `PUT /front-door/shared-records`.
#[derive(Clone)]
pub struct HttpFrontDoorSyncTransport {
    client: reqwest::Client,
    account_endpoint: reqwest::Url,
    endpoint: reqwest::Url,
}

impl fmt::Debug for HttpFrontDoorSyncTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpFrontDoorSyncTransport")
            .field("account_endpoint", &self.account_endpoint.as_str())
            .field("endpoint", &self.endpoint.as_str())
            .finish()
    }
}

impl HttpFrontDoorSyncTransport {
    /// Builds an HTTP transport pointing at a `front-door` base URL (HTTPS required except loopback).
    pub fn new(base_url: &str) -> Result<Self, ApiError> {
        let parsed = reqwest::Url::parse(base_url.trim())
            .map_err(|_| ApiError::Unprocessable("invalid front-door base URL"))?;
        if !parsed.username().is_empty() || parsed.password().is_some() {
            return Err(ApiError::Unprocessable(
                "front-door URL must not embed credentials",
            ));
        }
        let loopback = matches!(
            parsed.host_str(),
            Some("127.0.0.1" | "localhost" | "::1" | "[::1]")
        );
        if parsed.scheme() != "https" && !(parsed.scheme() == "http" && loopback) {
            return Err(ApiError::Unprocessable("front-door URL must use HTTPS"));
        }
        let account_endpoint = parsed
            .join("/front-door/account")
            .map_err(|_| ApiError::Unprocessable("invalid front-door account endpoint"))?;
        let endpoint = parsed
            .join("/front-door/shared-records")
            .map_err(|_| ApiError::Unprocessable("invalid front-door shared-records endpoint"))?;
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(15))
            .build()
            .map_err(|_| ApiError::Unavailable("front-door HTTP client unavailable"))?;
        Ok(Self {
            client,
            account_endpoint,
            endpoint,
        })
    }
}

#[async_trait]
impl FrontDoorSyncTransport for HttpFrontDoorSyncTransport {
    async fn upload_shared_records(
        &self,
        expected_front_door_account_id: Uuid,
        front_door_bearer: &str,
        records: &[SharedPatientRecordInput],
    ) -> Result<(), ConnectedCareSyncError> {
        let token = front_door_bearer.trim();
        if token.is_empty() {
            return Err(ConnectedCareSyncError::new(
                SharingFailureReason::Unauthenticated,
                "missing front-door session token",
                None,
            ));
        }
        // Rule 21: verify that the presented front-door session resolves to the exact
        // `front_door_account_id` mapped to the authenticated Duet account before sending PHI.
        let whoami = self
            .client
            .get(self.account_endpoint.clone())
            .bearer_auth(token)
            .send()
            .await
            .map_err(|error| {
                let sanitized = error.without_url();
                ConnectedCareSyncError::new(
                    SharingFailureReason::TransportUnavailable,
                    format!("front-door account verification request failed: {sanitized}"),
                    Some(Box::new(sanitized)),
                )
            })?;
        match whoami.status() {
            status if status.is_success() => {
                let identity =
                    whoami
                        .json::<FrontDoorAccountIdentity>()
                        .await
                        .map_err(|error| {
                            let sanitized = error.without_url();
                            ConnectedCareSyncError::new(
                                SharingFailureReason::TransportUnavailable,
                                format!(
                                    "front-door account verification response invalid: {sanitized}"
                                ),
                                Some(Box::new(sanitized)),
                            )
                        })?;
                if identity.account_id != expected_front_door_account_id {
                    return Err(ConnectedCareSyncError::new(
                        SharingFailureReason::Unauthenticated,
                        "front-door session does not belong to this Duet account",
                        None,
                    ));
                }
            }
            reqwest::StatusCode::UNAUTHORIZED | reqwest::StatusCode::FORBIDDEN => {
                return Err(ConnectedCareSyncError::new(
                    SharingFailureReason::Unauthenticated,
                    "front-door rejected the patient session",
                    None,
                ));
            }
            status => {
                return Err(ConnectedCareSyncError::new(
                    SharingFailureReason::TransportUnavailable,
                    format!(
                        "front-door account verification returned HTTP {}",
                        status.as_u16()
                    ),
                    None,
                ));
            }
        }

        let response = self
            .client
            .put(self.endpoint.clone())
            .bearer_auth(token)
            .json(&serde_json::json!({ "records": records }))
            .send()
            .await
            .map_err(|error| {
                let sanitized = error.without_url();
                ConnectedCareSyncError::new(
                    SharingFailureReason::TransportUnavailable,
                    format!("front-door upload request failed: {sanitized}"),
                    Some(Box::new(sanitized)),
                )
            })?;
        match response.status() {
            status if status.is_success() => Ok(()),
            reqwest::StatusCode::UNAUTHORIZED | reqwest::StatusCode::FORBIDDEN => {
                Err(ConnectedCareSyncError::new(
                    SharingFailureReason::Unauthenticated,
                    "front-door rejected the patient session",
                    None,
                ))
            }
            reqwest::StatusCode::NOT_FOUND | reqwest::StatusCode::CONFLICT => {
                Err(ConnectedCareSyncError::new(
                    SharingFailureReason::ConnectionEnded,
                    "no active practice connection for sharing",
                    None,
                ))
            }
            status => Err(ConnectedCareSyncError::new(
                SharingFailureReason::TransportUnavailable,
                format!("front-door returned HTTP {}", status.as_u16()),
                None,
            )),
        }
    }
}

/// In-process transport calling [`FrontDoorStore`] directly (used for tests and colocated runs).
#[derive(Clone)]
pub struct DirectFrontDoorSyncTransport {
    store: Arc<dyn FrontDoorStore>,
    now: Arc<dyn Fn() -> u64 + Send + Sync>,
}

impl DirectFrontDoorSyncTransport {
    /// Creates an in-process transport against `store`.
    pub fn new(store: Arc<dyn FrontDoorStore>, now: Arc<dyn Fn() -> u64 + Send + Sync>) -> Self {
        Self { store, now }
    }
}

#[async_trait]
impl FrontDoorSyncTransport for DirectFrontDoorSyncTransport {
    async fn upload_shared_records(
        &self,
        expected_front_door_account_id: Uuid,
        front_door_bearer: &str,
        records: &[SharedPatientRecordInput],
    ) -> Result<(), ConnectedCareSyncError> {
        let now = (self.now)();
        let account_id = self
            .store
            .session(front_door_bearer.trim(), now)
            .await
            .map_err(|error| {
                ConnectedCareSyncError::new(
                    SharingFailureReason::Unauthenticated,
                    format!("front-door session is not valid: {error:?}"),
                    None,
                )
            })?;
        if account_id != expected_front_door_account_id {
            return Err(ConnectedCareSyncError::new(
                SharingFailureReason::Unauthenticated,
                "front-door session does not belong to this Duet account",
                None,
            ));
        }
        self.store
            .upsert_shared_records(account_id, records, now)
            .await
            .map_err(|error| {
                let reason = match error {
                    ForMyDoctorError::Unlinked => SharingFailureReason::ConnectionEnded,
                    ForMyDoctorError::Storage(_) => SharingFailureReason::TransportUnavailable,
                    _ => SharingFailureReason::TransportUnavailable,
                };
                ConnectedCareSyncError::new(
                    reason,
                    format!("front-door shared-record upsert failed: {error}"),
                    Some(Box::new(error)),
                )
            })
    }
}

/// Live uploader streaming consented Duet canonical evidence to `front-door` while tracking
/// honest sync health and operational alerts (`intent/connected-care.md` P15, C1.3.8).
#[derive(Clone)]
pub struct ConnectedCareUploader {
    database: AccountDatabase,
    evidence: EvidenceStore,
    transport: Arc<dyn FrontDoorSyncTransport>,
    standings: Arc<Mutex<BTreeMap<Uuid, SharingSyncStanding>>>,
}

impl ConnectedCareUploader {
    /// Creates a live uploader backed by Duet's [`AccountDatabase`], [`EvidenceStore`], and a `front-door` transport.
    pub fn new(
        database: AccountDatabase,
        evidence: EvidenceStore,
        transport: Arc<dyn FrontDoorSyncTransport>,
    ) -> Self {
        Self {
            database,
            evidence,
            transport,
            standings: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    async fn mapped_front_door_account(
        &self,
        context: &AccountContext,
    ) -> Result<Uuid, ConnectedCareSyncError> {
        let mut tx = self
            .database
            .begin(context.account_id)
            .await
            .map_err(|err| {
                ConnectedCareSyncError::new(
                    SharingFailureReason::EvidenceReadFailed,
                    "failed to open account transaction for front-door mapping check",
                    Some(Box::new(err)),
                )
            })?;
        if let Err(err) = check_current(&mut tx, context, false).await {
            let reason = match err {
                ApiError::Unauthorized | ApiError::SessionRevoked | ApiError::AbsentAccount => {
                    SharingFailureReason::Unauthenticated
                }
                _ => SharingFailureReason::EvidenceReadFailed,
            };
            return Err(ConnectedCareSyncError::new(
                reason,
                format!("Duet account session could not be verified: {err:?}"),
                None,
            ));
        }
        let mapped = orm::model::<front_door_mapping::Model>(
            &mut tx,
            &front_door_mapping::Entity::find()
                .filter(front_door_mapping::Column::UserId.eq(context.account_id))
                .into_query(),
        )
        .await
        .map_err(|err| {
            ConnectedCareSyncError::new(
                SharingFailureReason::EvidenceReadFailed,
                format!("failed to query Duet front-door identity mapping: {err:?}"),
                None,
            )
        })?;
        tx.commit().await.map_err(|err| {
            ConnectedCareSyncError::new(
                SharingFailureReason::EvidenceReadFailed,
                "failed to complete front-door mapping verification transaction",
                Some(Box::new(err)),
            )
        })?;
        mapped.map(|row| row.front_door_account_id).ok_or_else(|| {
            ConnectedCareSyncError::new(
                SharingFailureReason::Unauthenticated,
                "Duet account is not mapped to a front-door account",
                None,
            )
        })
    }

    /// Reads the current sharing standing for `account_id`.
    pub fn standing(&self, account_id: Uuid) -> SharingSyncStanding {
        self.standings
            .lock()
            .ok()
            .and_then(|map| map.get(&account_id).cloned())
            .unwrap_or(SharingSyncStanding::NeverSynced)
    }

    /// Returns content-free operational alerts for every account currently in [`SharingSyncStanding::Degraded`].
    pub fn degraded_alerts(&self) -> Vec<DegradedSharingAlert> {
        let Ok(map) = self.standings.lock() else {
            return Vec::new();
        };
        map.iter()
            .filter_map(|(account_id, standing)| match standing {
                SharingSyncStanding::Degraded {
                    last_succeeded_at,
                    failed_at,
                    consecutive_failures,
                    reason,
                    ..
                } => Some(DegradedSharingAlert {
                    account_id: *account_id,
                    failed_at: *failed_at,
                    consecutive_failures: *consecutive_failures,
                    reason: *reason,
                    last_succeeded_at: *last_succeeded_at,
                }),
                SharingSyncStanding::NeverSynced | SharingSyncStanding::Current { .. } => None,
            })
            .collect()
    }

    /// Streams all current consented Duet evidence items (plus source-removal events) for
    /// `context` to `PUT /front-door/shared-records`.
    ///
    /// Enforces `Rule 21` (caller must have an explicit `duet_front_door_mapping` matching the
    /// presented `front_door_bearer`) and never crosses private AI memory or private notes
    /// (`Rule 10`). Fails closed on error, recording degraded standing and emitting `ERROR`
    /// telemetry instead of a silent warning.
    pub async fn sync_account(
        &self,
        context: &AccountContext,
        front_door_bearer: &str,
        now: DateTime<Utc>,
    ) -> Result<SharingSyncReport, ConnectedCareSyncError> {
        let expected_front_door_account_id = match self.mapped_front_door_account(context).await {
            Ok(id) => id,
            Err(sync_err) => {
                self.record_failure(context.account_id, now, &sync_err);
                return Err(sync_err);
            }
        };

        let snapshot = match self.evidence.snapshot(context, 0).await {
            Ok(snapshot) => snapshot,
            Err(error) => {
                let sync_err = ConnectedCareSyncError::new(
                    SharingFailureReason::EvidenceReadFailed,
                    format!("failed to read Duet evidence snapshot: {error:?}"),
                    None,
                );
                self.record_failure(context.account_id, now, &sync_err);
                return Err(sync_err);
            }
        };

        let (records, excluded_private_count) = build_shared_record_batch(&snapshot, now);
        if let Err(sync_err) = self
            .transport
            .upload_shared_records(expected_front_door_account_id, front_door_bearer, &records)
            .await
        {
            self.record_failure(context.account_id, now, &sync_err);
            return Err(sync_err);
        }

        let report = SharingSyncReport {
            cursor: snapshot.cursor,
            uploaded_count: records.len(),
            excluded_private_count,
            synced_at: now,
        };
        if let Ok(mut map) = self.standings.lock() {
            map.insert(
                context.account_id,
                SharingSyncStanding::Current {
                    synced_at: now,
                    cursor: snapshot.cursor,
                    record_count: records.len(),
                },
            );
        }
        Ok(report)
    }

    fn record_failure(
        &self,
        account_id: Uuid,
        failed_at: DateTime<Utc>,
        error: &ConnectedCareSyncError,
    ) {
        if let Ok(mut map) = self.standings.lock() {
            let (last_succeeded_at, last_cursor, consecutive_failures) = match map.get(&account_id)
            {
                Some(SharingSyncStanding::Current {
                    synced_at, cursor, ..
                }) => (Some(*synced_at), Some(*cursor), 1),
                Some(SharingSyncStanding::Degraded {
                    last_succeeded_at,
                    last_cursor,
                    consecutive_failures,
                    ..
                }) => (
                    *last_succeeded_at,
                    *last_cursor,
                    consecutive_failures.saturating_add(1),
                ),
                Some(SharingSyncStanding::NeverSynced) | None => (None, None, 1),
            };
            map.insert(
                account_id,
                SharingSyncStanding::Degraded {
                    last_succeeded_at,
                    last_cursor,
                    failed_at,
                    consecutive_failures,
                    reason: error.reason,
                    message: error.message.clone(),
                },
            );
        }
        background::failed(
            CONNECTED_CARE_SYNC_FAILED,
            "upload",
            error.reason.telemetry_class(),
        );
    }
}

/// Converts a Duet [`EvidenceSnapshot`] into `front-door` shared-record inputs carrying exact
/// `"from <source>, via the patient's Duet"` origin attribution and literal source dates.
///
/// Private AI notes and non-shareable drafts are excluded (`Rule 10`).
pub fn build_shared_record_batch(
    snapshot: &EvidenceSnapshot,
    fallback_now: DateTime<Utc>,
) -> (Vec<SharedPatientRecordInput>, usize) {
    let mut output = Vec::with_capacity(snapshot.records.len());
    let mut excluded_private = 0usize;

    for record in &snapshot.records {
        if !record.current || is_private_ai_or_unshared_note(record) {
            excluded_private += 1;
            continue;
        }
        output.push(project_active_record(record, fallback_now));
    }

    // Explicit source removals arrive as distinct retraction revisions so unfinished practice
    // work receives the update and never publishes a replaced/removed item (`Rule 13`).
    for change in &snapshot.changes {
        if change.deleted && change.source_removed {
            output.push(project_source_removal(change, fallback_now));
        }
    }

    (output, excluded_private)
}

fn is_private_ai_or_unshared_note(record: &ExactEvidence) -> bool {
    if record
        .payload
        .get("private_ai_note")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return true;
    }
    let kind = record
        .payload
        .get("kind")
        .or_else(|| record.payload.get("note_kind"))
        .and_then(Value::as_str)
        .map(str::trim)
        .unwrap_or_default()
        .to_ascii_lowercase();
    if matches!(
        kind.as_str(),
        "private_ai_note" | "duet_private_memory" | "for_my_doctor_draft"
    ) {
        return true;
    }
    let source_label = record
        .payload
        .get("source_label")
        .and_then(Value::as_str)
        .map(str::trim)
        .unwrap_or_default();
    if record.address.entity == EvidenceEntity::ClinicalNote
        && source_label.eq_ignore_ascii_case("Added in Duet")
    {
        // Patient free-text notes in For my doctor cross only via explicit Send (`Rule 11`),
        // not via continuous record streaming unless they are structured medications.
        let concept = record
            .payload
            .get("concept_id")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_ascii_lowercase();
        if !concept.contains("medication") {
            return true;
        }
    }
    false
}

fn project_active_record(
    record: &ExactEvidence,
    fallback_now: DateTime<Utc>,
) -> SharedPatientRecordInput {
    let origin = DuetSourceOrigin::from_payload(record.address.entity, &record.payload);
    let record_type = infer_record_type(record.address.entity, &record.payload);
    let title = infer_title(&record.payload, &record.address.record_id);
    let recorded_at = extract_recorded_unix(&record.payload)
        .unwrap_or_else(|| u64::try_from(fallback_now.timestamp().max(0)).unwrap_or(0));
    let source_status = record
        .payload
        .get("status")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let concept = record
        .payload
        .get("concept_id")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let summary_body = render_literal_record_body(&record.payload);

    let mut header = format!(
        "Source-Origin: {}\nOrigin: {}\nSource-Record-Id: {}",
        origin.source_name(),
        origin.via_duet_label(),
        sanitize_header_value(&record.address.record_id)
    );
    if let Some(concept) = concept {
        let clean = sanitize_header_value(concept);
        if !clean.is_empty() {
            header.push_str(&format!("\nSource-Concept: {clean}"));
        }
    }
    if let Some(status) = source_status {
        let clean = sanitize_header_value(status);
        if !clean.is_empty() {
            header.push_str(&format!("\nSource-Status: {clean}"));
        }
    }
    let content_text = format!("{header}\n---\n{summary_body}");

    SharedPatientRecordInput {
        record_id: record.address.canonical_record_id.to_string(),
        revision: format!("rev-{}", record.address.revision),
        record_type,
        title,
        content_text,
        recorded_at,
    }
}

fn sanitize_header_value(raw: &str) -> String {
    raw.chars()
        .map(|ch| if matches!(ch, '\r' | '\n') { ' ' } else { ch })
        .collect::<String>()
        .trim()
        .to_owned()
}

fn project_source_removal(change: &EvidenceChange, now: DateTime<Utc>) -> SharedPatientRecordInput {
    let recorded_at = u64::try_from(now.timestamp().max(0)).unwrap_or(0);
    let clean_record_id = sanitize_header_value(&change.record_id);
    let content_text = format!(
        "Source-Origin: source update\nOrigin: from source update, via the patient's Duet\nSource-Record-Id: {clean_record_id}\nSource-Status: removed\n---\nSource removed this record at revision {}.",
        change.revision
    );
    SharedPatientRecordInput {
        record_id: change.canonical_record_id.to_string(),
        revision: format!("removed-{}", change.revision),
        record_type: match change.entity {
            EvidenceEntity::ClinicalNote => "DocumentReference".to_owned(),
            EvidenceEntity::HealthEvidence => "Observation".to_owned(),
        },
        title: format!("Removed record {}", change.record_id),
        content_text,
        recorded_at,
    }
}

fn infer_record_type(entity: EvidenceEntity, payload: &Value) -> String {
    if let Some(rt) = payload
        .get("resourceType")
        .or_else(|| payload.get("record_type"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        return rt.to_owned();
    }
    let concept = payload
        .get("concept_id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_ascii_lowercase();
    if concept.contains("medication") {
        return "MedicationStatement".to_owned();
    }
    match entity {
        EvidenceEntity::ClinicalNote => "DocumentReference".to_owned(),
        EvidenceEntity::HealthEvidence => "Observation".to_owned(),
    }
}

fn infer_title(payload: &Value, fallback_record_id: &str) -> String {
    for key in ["display_name", "title", "name", "concept_id"] {
        if let Some(value) = payload
            .get(key)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            return value.to_owned();
        }
    }
    fallback_record_id.to_owned()
}

fn extract_recorded_unix(payload: &Value) -> Option<u64> {
    let candidates = [
        payload
            .get("measured_interval")
            .and_then(|v| v.get("start"))
            .and_then(Value::as_str),
        payload.get("measured_at").and_then(Value::as_str),
        payload.get("recorded_at").and_then(Value::as_str),
        payload.get("effectiveDateTime").and_then(Value::as_str),
    ];
    for raw in candidates.into_iter().flatten() {
        if let Ok(dt) = DateTime::parse_from_rfc3339(raw.trim()) {
            return u64::try_from(dt.timestamp().max(0)).ok();
        }
    }
    None
}

fn render_literal_record_body(payload: &Value) -> String {
    if let Some(body) = payload
        .get("body")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|v| !v.is_empty())
    {
        return body.to_owned();
    }
    if let Some(summary) = payload
        .get("summary")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|v| !v.is_empty())
    {
        return summary.to_owned();
    }
    if let Some(qty) = payload.get("value").and_then(|v| v.get("quantity"))
        && let Some(val) = qty.get("canonical_value").and_then(Value::as_f64)
    {
        let unit = qty
            .get("canonical_unit")
            .and_then(Value::as_str)
            .unwrap_or("");
        let name = payload
            .get("display_name")
            .and_then(Value::as_str)
            .unwrap_or("Observation");
        return if unit.is_empty() {
            format!("{name}: {val}")
        } else {
            format!("{name}: {val} {unit}")
        };
    }
    payload.to_string()
}

/// Router state for `/api/connected-care/*` live sharing routes.
#[derive(Clone)]
pub struct ConnectedCareSyncState {
    /// Authenticated identity, privacy, and evidence owners.
    pub durable: DurableState,
    /// Live uploader & standing tracker.
    pub uploader: ConnectedCareUploader,
}

/// Builds the authenticated Duet -> `front-door` sharing sync & status router.
pub fn router(state: ConnectedCareSyncState) -> Router {
    Router::new()
        .route("/api/connected-care/sync", post(trigger_sync_route))
        .route("/api/connected-care/status", get(sync_status_route))
        .with_state(state)
}

async fn trigger_sync_route(
    State(state): State<ConnectedCareSyncState>,
    headers: HeaderMap,
) -> Result<Json<SharingSyncReport>, ApiError> {
    let context = state.durable.authenticate(&headers).await?;
    let front_door_session = headers
        .get(FRONT_DOOR_SESSION_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or(ApiError::Unauthorized)?;
    let report = state
        .uploader
        .sync_account(&context, front_door_session, Utc::now())
        .await?;
    Ok(Json(report))
}

async fn sync_status_route(
    State(state): State<ConnectedCareSyncState>,
    headers: HeaderMap,
) -> Result<Json<SharingSyncStanding>, ApiError> {
    let context = state.durable.authenticate(&headers).await?;
    Ok(Json(state.uploader.standing(context.account_id)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence::AddressedRecordRevision;
    use serde_json::json;

    #[test]
    fn source_origin_formats_exact_via_duet_labels() {
        assert_eq!(
            DuetSourceOrigin::Epic.via_duet_label(),
            "from Epic, via the patient's Duet"
        );
        assert_eq!(
            DuetSourceOrigin::OracleHealth.via_duet_label(),
            "from Oracle Health, via the patient's Duet"
        );
        assert_eq!(
            DuetSourceOrigin::AppleHealth.via_duet_label(),
            "from Apple Health, via the patient's Duet"
        );
        assert_eq!(
            DuetSourceOrigin::HealthConnect.via_duet_label(),
            "from Health Connect, via the patient's Duet"
        );
    }

    #[test]
    fn private_ai_notes_and_for_my_doctor_drafts_are_excluded_from_live_streaming() {
        let now = Utc::now();
        let epic_id = Uuid::new_v4();
        let private_ai_id = Uuid::new_v4();
        let fmd_draft_id = Uuid::new_v4();
        let snapshot = EvidenceSnapshot {
            cursor: 3,
            health_records_generation: 0,
            changes: Vec::new(),
            records: vec![
                ExactEvidence {
                    address: AddressedRecordRevision::new(
                        epic_id,
                        EvidenceEntity::HealthEvidence,
                        "epic-lab-a1c-1",
                        1,
                    ),
                    payload: json!({
                        "display_name": "Hemoglobin A1c",
                        "concept_id": "hba1c",
                        "source": { "kind": "epic" },
                        "measured_interval": { "start": "2026-11-15T14:30:00Z" },
                        "value": { "quantity": { "canonical_value": 6.4, "canonical_unit": "%" } }
                    }),
                    current: true,
                },
                ExactEvidence {
                    address: AddressedRecordRevision::new(
                        private_ai_id,
                        EvidenceEntity::ClinicalNote,
                        "ai-mem-1",
                        1,
                    ),
                    payload: json!({
                        "title": "Duet private memory",
                        "private_ai_note": true,
                        "body": "Internal private AI observation"
                    }),
                    current: true,
                },
                ExactEvidence {
                    address: AddressedRecordRevision::new(
                        fmd_draft_id,
                        EvidenceEntity::ClinicalNote,
                        "fmd-note-1",
                        1,
                    ),
                    payload: json!({
                        "title": "Draft note for visit",
                        "source_label": "Added in Duet",
                        "body": "Ask about knees"
                    }),
                    current: true,
                },
            ],
        };

        let (batch, excluded) = build_shared_record_batch(&snapshot, now);
        assert_eq!(excluded, 2);
        assert_eq!(batch.len(), 1);
        assert_eq!(batch[0].record_id, epic_id.to_string());
        assert_eq!(batch[0].revision, "rev-1");
        assert!(
            batch[0]
                .content_text
                .contains("Origin: from Epic, via the patient's Duet")
        );
        assert!(batch[0].content_text.contains("Hemoglobin A1c: 6.4 %"));
    }

    #[test]
    fn http_transport_requires_https_except_loopback() {
        assert!(HttpFrontDoorSyncTransport::new("https://front-door.coda.test").is_ok());
        assert!(HttpFrontDoorSyncTransport::new("http://127.0.0.1:8085").is_ok());
        assert!(HttpFrontDoorSyncTransport::new("http://front-door.coda.test").is_err());
        assert!(HttpFrontDoorSyncTransport::new("https://user:pass@front-door.coda.test").is_err());
    }

    #[test]
    fn header_values_strip_control_newlines() {
        assert_eq!(
            sanitize_header_value("epic-1\r\nOrigin: spoofed"),
            "epic-1  Origin: spoofed"
        );
    }
}
