//! `"For my doctor"` (`Shared` tab) durable backend owner.
//!
//! Owns marks from `You`, unlimited free-text notes (standalone or linked to a
//! record), source-correction / source-removal reconciliation, and immutable
//! sends forwarded idempotently (`share_key`) to `front-door` (`ForMyDoctorShare`).
//! Printed lab reference ranges are stripped at every projection boundary.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use chrono::{DateTime, TimeZone, Utc};
use front_door::{
    AccountConnection, ConnectionStanding, ForMyDoctorError, ForMyDoctorSendRequest,
    ForMyDoctorShare, ForMyDoctorShareStatus, FrontDoorStore, RosterPatient,
};
use sea_orm::sea_query::OnConflict;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QueryTrait, Set};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::ApiError;
use crate::connected_care_sync::{FRONT_DOOR_SESSION_HEADER, front_door_account_id};
use crate::database::orm;
use crate::durable::DurableState;
use crate::evidence_entity::{clinical_note, health_evidence, membership, revision};
use crate::for_my_doctor_entity::{
    MarkTargetKind, RecordFamily, StoredSendStanding, mark, note, notice, send,
};
use crate::identity::{AccessLevel, AccountContext, check_current};
use crate::workspace::strip_reference_ranges;

/// Typed error returned by [`ForMyDoctorFrontDoorTransport`].
#[derive(Debug)]
pub enum FrontDoorCallError {
    /// Missing, invalid, or cross-account `front-door` session credential.
    Unauthenticated,
    /// Patient has no active `Connected` connection with the target practice.
    ConnectionEnded {
        /// Plain-language end reason.
        reason: String,
        /// Optional Unix second the connection ended.
        ended_at: Option<u64>,
    },
    /// Same `share_key` was reused with different payload bytes.
    Conflict(String),
    /// Transient transport/timeout/5xx fault; caller may hold `"Couldn't confirm the send"`.
    Unavailable(String),
}

impl From<FrontDoorCallError> for ApiError {
    fn from(error: FrontDoorCallError) -> Self {
        match error {
            FrontDoorCallError::Unauthenticated => Self::Unauthorized,
            FrontDoorCallError::ConnectionEnded { .. } => {
                Self::Conflict("practice connection is not active")
            }
            FrontDoorCallError::Conflict(_) => {
                Self::Conflict("share key already used with different send details")
            }
            FrontDoorCallError::Unavailable(_) => {
                Self::Unavailable("Duet could not confirm the send with the practice")
            }
        }
    }
}

/// Snapshot of a patient's `front-door` practice connections and share standings.
#[derive(Clone, Debug, Default)]
pub struct FrontDoorOverview {
    /// Active and ended practice connections for the mapped account.
    pub connections: Vec<AccountConnection>,
    /// Existing `"For my doctor"` shares held at `front-door`.
    pub shares: Vec<ForMyDoctorShare>,
}

/// Transport boundary from `duet-api` to `front-door` for `"For my doctor"` sends.
#[async_trait]
pub trait ForMyDoctorFrontDoorTransport: Send + Sync {
    /// Reads active/ended practice connections and current share delivery standings.
    async fn account_overview(
        &self,
        expected_front_door_account_id: Uuid,
        front_door_bearer: &str,
    ) -> Result<FrontDoorOverview, FrontDoorCallError>;

    /// Forwards one idempotent `"For my doctor"` send keyed by `request.share_key`.
    async fn send_share(
        &self,
        expected_front_door_account_id: Uuid,
        front_door_bearer: &str,
        request: &ForMyDoctorSendRequest,
    ) -> Result<ForMyDoctorShare, FrontDoorCallError>;
}

/// In-process transport backed directly by [`FrontDoorStore`] (tests and colocated runs).
#[derive(Clone)]
pub struct DirectFrontDoorShareTransport {
    store: Arc<dyn FrontDoorStore>,
    now: Arc<dyn Fn() -> u64 + Send + Sync>,
}

impl DirectFrontDoorShareTransport {
    /// Constructs a direct transport against `store`.
    pub fn new(store: Arc<dyn FrontDoorStore>, now: Arc<dyn Fn() -> u64 + Send + Sync>) -> Self {
        Self { store, now }
    }

    async fn verify_session(
        &self,
        expected_front_door_account_id: Uuid,
        front_door_bearer: &str,
    ) -> Result<Uuid, FrontDoorCallError> {
        let token = front_door_bearer.trim();
        if token.is_empty() {
            return Err(FrontDoorCallError::Unauthenticated);
        }
        let now = (self.now)();
        let account_id = self
            .store
            .session(token, now)
            .await
            .map_err(|_| FrontDoorCallError::Unauthenticated)?;
        if account_id != expected_front_door_account_id {
            return Err(FrontDoorCallError::Unauthenticated);
        }
        Ok(account_id)
    }
}

#[async_trait]
impl ForMyDoctorFrontDoorTransport for DirectFrontDoorShareTransport {
    async fn account_overview(
        &self,
        expected_front_door_account_id: Uuid,
        front_door_bearer: &str,
    ) -> Result<FrontDoorOverview, FrontDoorCallError> {
        let account_id = self
            .verify_session(expected_front_door_account_id, front_door_bearer)
            .await?;
        let account = self
            .store
            .account(account_id)
            .await
            .ok_or(FrontDoorCallError::Unauthenticated)?;
        let shares = self
            .store
            .for_my_doctor_shares_for_account(account_id)
            .await
            .map_err(|err| FrontDoorCallError::Unavailable(err.to_string()))?;
        Ok(FrontDoorOverview {
            connections: account.connections,
            shares,
        })
    }

    async fn send_share(
        &self,
        expected_front_door_account_id: Uuid,
        front_door_bearer: &str,
        request: &ForMyDoctorSendRequest,
    ) -> Result<ForMyDoctorShare, FrontDoorCallError> {
        let account_id = self
            .verify_session(expected_front_door_account_id, front_door_bearer)
            .await?;
        let now = (self.now)();
        match self
            .store
            .send_for_my_doctor(account_id, request.clone(), now)
            .await
        {
            Ok(share) => Ok(share),
            Err(ForMyDoctorError::Unlinked) => {
                let account = self.store.account(account_id).await;
                let ended_at = account.and_then(|acc| {
                    acc.connections.iter().find_map(|c| {
                        if request.target.as_ref() == Some(&c.patient)
                            && let ConnectionStanding::Ended { ended_at, .. } = c.standing
                        {
                            Some(ended_at)
                        } else {
                            None
                        }
                    })
                });
                Err(FrontDoorCallError::ConnectionEnded {
                    reason: "Connection to this practice has ended.".to_owned(),
                    ended_at,
                })
            }
            Err(ForMyDoctorError::Conflict) => Err(FrontDoorCallError::Conflict(
                "Share key already exists with different content.".to_owned(),
            )),
            Err(ForMyDoctorError::InvalidInput(msg)) => {
                Err(FrontDoorCallError::Conflict(msg.into()))
            }
            Err(ForMyDoctorError::NotFound) => Err(FrontDoorCallError::ConnectionEnded {
                reason: "Connection to this practice has ended.".to_owned(),
                ended_at: None,
            }),
            Err(ForMyDoctorError::Storage(err)) => {
                Err(FrontDoorCallError::Unavailable(err.to_string()))
            }
        }
    }
}

/// Bounded HTTPS transport calling `front-door` HTTP endpoints.
#[derive(Clone)]
pub struct HttpFrontDoorShareTransport {
    client: reqwest::Client,
    account_endpoint: reqwest::Url,
    shares_endpoint: reqwest::Url,
}

#[derive(Deserialize)]
struct HttpFrontDoorAccountView {
    account_id: Uuid,
    connections: Vec<AccountConnection>,
}

#[derive(Deserialize)]
struct HttpFrontDoorShareView {
    share_id: String,
    patient: RosterPatient,
    share_key: String,
    title: String,
    summary_text: String,
    origin_label: String,
    sent_at: u64,
    #[serde(flatten)]
    standing: ForMyDoctorShareStatus,
}

impl HttpFrontDoorShareTransport {
    /// Builds an HTTP transport pointing at `front-door` (HTTPS required off loopback).
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
        let shares_endpoint = parsed
            .join("/front-door/for-my-doctor")
            .map_err(|_| ApiError::Unprocessable("invalid front-door for-my-doctor endpoint"))?;
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(15))
            .build()
            .map_err(|_| ApiError::Unavailable("front-door HTTP client unavailable"))?;
        Ok(Self {
            client,
            account_endpoint,
            shares_endpoint,
        })
    }

    async fn verified_account(
        &self,
        expected_front_door_account_id: Uuid,
        front_door_bearer: &str,
    ) -> Result<HttpFrontDoorAccountView, FrontDoorCallError> {
        let token = front_door_bearer.trim();
        if token.is_empty() {
            return Err(FrontDoorCallError::Unauthenticated);
        }
        let resp = self
            .client
            .get(self.account_endpoint.clone())
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| FrontDoorCallError::Unavailable(e.without_url().to_string()))?;
        match resp.status() {
            s if s.is_success() => {
                let account = resp
                    .json::<HttpFrontDoorAccountView>()
                    .await
                    .map_err(|e| FrontDoorCallError::Unavailable(e.without_url().to_string()))?;
                if account.account_id != expected_front_door_account_id {
                    return Err(FrontDoorCallError::Unauthenticated);
                }
                Ok(account)
            }
            reqwest::StatusCode::UNAUTHORIZED | reqwest::StatusCode::FORBIDDEN => {
                Err(FrontDoorCallError::Unauthenticated)
            }
            status => Err(FrontDoorCallError::Unavailable(format!(
                "front-door account returned HTTP {}",
                status.as_u16()
            ))),
        }
    }
}

#[async_trait]
impl ForMyDoctorFrontDoorTransport for HttpFrontDoorShareTransport {
    async fn account_overview(
        &self,
        expected_front_door_account_id: Uuid,
        front_door_bearer: &str,
    ) -> Result<FrontDoorOverview, FrontDoorCallError> {
        let account = self
            .verified_account(expected_front_door_account_id, front_door_bearer)
            .await?;
        let token = front_door_bearer.trim();
        let shares_resp = self
            .client
            .get(self.shares_endpoint.clone())
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| FrontDoorCallError::Unavailable(e.without_url().to_string()))?;
        let shares = match shares_resp.status() {
            s if s.is_success() => shares_resp
                .json::<Vec<HttpFrontDoorShareView>>()
                .await
                .map_err(|e| FrontDoorCallError::Unavailable(e.without_url().to_string()))?
                .into_iter()
                .map(|view| ForMyDoctorShare {
                    share_id: view.share_id,
                    account_id: account.account_id,
                    patient: view.patient,
                    share_key: view.share_key,
                    title: view.title,
                    summary_text: view.summary_text,
                    origin_label: view.origin_label,
                    sent_at: view.sent_at,
                    consent_at: 0,
                    standing: view.standing,
                })
                .collect(),
            reqwest::StatusCode::UNAUTHORIZED | reqwest::StatusCode::FORBIDDEN => {
                return Err(FrontDoorCallError::Unauthenticated);
            }
            status => {
                return Err(FrontDoorCallError::Unavailable(format!(
                    "front-door share list returned HTTP {}",
                    status.as_u16()
                )));
            }
        };
        Ok(FrontDoorOverview {
            connections: account.connections,
            shares,
        })
    }

    async fn send_share(
        &self,
        expected_front_door_account_id: Uuid,
        front_door_bearer: &str,
        request: &ForMyDoctorSendRequest,
    ) -> Result<ForMyDoctorShare, FrontDoorCallError> {
        let account = self
            .verified_account(expected_front_door_account_id, front_door_bearer)
            .await?;
        let token = front_door_bearer.trim();
        let resp = self
            .client
            .post(self.shares_endpoint.clone())
            .bearer_auth(token)
            .json(request)
            .send()
            .await
            .map_err(|e| FrontDoorCallError::Unavailable(e.without_url().to_string()))?;
        match resp.status() {
            s if s.is_success() => {
                let view = resp
                    .json::<HttpFrontDoorShareView>()
                    .await
                    .map_err(|e| FrontDoorCallError::Unavailable(e.without_url().to_string()))?;
                Ok(ForMyDoctorShare {
                    share_id: view.share_id,
                    account_id: account.account_id,
                    patient: view.patient,
                    share_key: view.share_key,
                    title: view.title,
                    summary_text: view.summary_text,
                    origin_label: view.origin_label,
                    sent_at: view.sent_at,
                    consent_at: 0,
                    standing: view.standing,
                })
            }
            reqwest::StatusCode::UNAUTHORIZED | reqwest::StatusCode::FORBIDDEN => {
                Err(FrontDoorCallError::Unauthenticated)
            }
            reqwest::StatusCode::NOT_FOUND => {
                let ended_at = account.connections.iter().find_map(|c| {
                    if request.target.as_ref() == Some(&c.patient)
                        && let ConnectionStanding::Ended { ended_at, .. } = c.standing
                    {
                        Some(ended_at)
                    } else {
                        None
                    }
                });
                Err(FrontDoorCallError::ConnectionEnded {
                    reason: "Connection to this practice has ended.".to_owned(),
                    ended_at,
                })
            }
            reqwest::StatusCode::CONFLICT => Err(FrontDoorCallError::Conflict(
                "Share key already exists with different content.".to_owned(),
            )),
            status => Err(FrontDoorCallError::Unavailable(format!(
                "front-door share returned HTTP {}",
                status.as_u16()
            ))),
        }
    }
}

/// Wire view of one marked item in `To send`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForMyDoctorMarkView {
    /// Mark identifier.
    pub id: Uuid,
    /// Kind (`record`, `value`, `reading`, `trend`).
    pub target_kind: MarkTargetKind,
    /// Deduplicating key.
    pub target_key: String,
    /// Linked canonical record id, if any.
    pub record_id: Option<Uuid>,
    /// Canonical family, if linked.
    pub entity: Option<RecordFamily>,
    /// Current canonical revision observed for `record_id`.
    pub record_revision: Option<i32>,
    /// Canonical revision when originally marked.
    pub marked_revision: Option<i32>,
    /// Title shown to patient and practice.
    pub title: String,
    /// Plain summary (never contains lab printed reference ranges).
    pub summary_text: String,
    /// Viewed date range label when `target_kind == Trend`.
    pub range_label: Option<String>,
    /// Source display label.
    pub source_label: Option<String>,
    /// True when the source corrected the record after it was marked.
    pub source_corrected: bool,
    /// Date of the post-mark source correction, if corrected.
    pub correction_date: Option<String>,
    /// Reference-range-free payload snapshot.
    pub payload: Value,
    /// Timestamp marked.
    pub marked_at: String,
    /// Timestamp last updated.
    pub updated_at: String,
}

/// Wire view of one patient note in `To send`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForMyDoctorNoteView {
    /// Note identifier.
    pub id: Uuid,
    /// Free-text note body.
    pub body: String,
    /// Optional linked canonical record id.
    pub linked_record_id: Option<Uuid>,
    /// Plain label of the linked record.
    pub linked_record_title: Option<String>,
    /// Canonical family of the linked record.
    pub linked_entity: Option<RecordFamily>,
    /// Monotonic edit version.
    pub version: i64,
    /// Creation timestamp.
    pub created_at: String,
    /// Last updated timestamp.
    pub updated_at: String,
}

/// Wire view of one practice connection attached to the signed-in account.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum PracticeStanding {
    /// Actively sharing.
    Connected {
        /// Unix second connected.
        connected_at: u64,
    },
    /// Connection ended; historical sends stay listed with `ended_at`.
    Ended {
        /// Unix second connected.
        connected_at: u64,
        /// Unix second ended.
        ended_at: u64,
        /// End reason label.
        reason: String,
    },
}

/// One connected or ended practice shown in `Shared`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticeConnectionView {
    /// Practice identifier.
    pub practice_id: String,
    /// Human-readable practice name.
    pub practice_name: String,
    /// Patient identifier on the practice's roster.
    pub patient_id: String,
    /// Whether connected or ended.
    #[serde(flatten)]
    pub standing: PracticeStanding,
}

/// Frozen record entry included in a committed send snapshot.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SentRecordEntry {
    /// Canonical record identifier, if backed by canonical evidence.
    pub record_id: Option<Uuid>,
    /// Title at send time.
    pub title: String,
    /// Summary text (without lab printed reference ranges) at send time.
    pub summary_text: String,
    /// Plain source label.
    pub source_label: Option<String>,
    /// Frozen revision number at send time.
    pub version: Option<i32>,
    /// Date of source correction if corrected before send.
    pub correction_date: Option<String>,
    /// Viewed trend range label, if this entry came from a marked trend.
    pub range_label: Option<String>,
}

/// Frozen note entry included in a committed send snapshot.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SentNoteEntry {
    /// Original note id.
    pub id: Uuid,
    /// Frozen note words.
    pub body: String,
    /// Linked record label if written from a record.
    pub linked_record_title: Option<String>,
}

/// Delivery standing shown on a send row in `Shared`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum SendStandingView {
    /// Saved by Duet and waiting practice agent pickup.
    Sent,
    /// Picked up by the practice's agents (`"Received by the care team"`).
    Received {
        /// Timestamp picked up.
        received_at: String,
    },
}

/// Wire view of one immutable send in `Shared`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForMyDoctorSendView {
    /// Local send id.
    pub id: Uuid,
    /// Idempotency key.
    pub share_key: String,
    /// Practice identifier.
    pub practice_id: String,
    /// Practice name.
    pub practice_name: String,
    /// Patient roster identifier.
    pub patient_id: String,
    /// Headline shown in the list.
    pub title: String,
    /// Plain summary sent to the practice.
    pub summary_text: String,
    /// Frozen records carried in this send.
    pub records: Vec<SentRecordEntry>,
    /// Frozen notes carried in this send.
    pub notes: Vec<SentNoteEntry>,
    /// `Sent` or `Received`.
    #[serde(flatten)]
    pub standing: SendStandingView,
    /// Timestamp sent.
    pub sent_at: String,
    /// Date the practice connection ended if ended before pickup.
    pub connection_ended_at: Option<String>,
}

/// Plain notice surfaced in `You` when a clinical source removes a marked record (`COD-50`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForMyDoctorNoticeView {
    /// Notice id.
    pub id: Uuid,
    /// Title of the removed record.
    pub title: String,
    /// Plain-language notice sentence.
    pub message: String,
    /// Timestamp created.
    pub created_at: String,
}

/// Complete `"For my doctor"` state returned by `GET /api/for-my-doctor`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForMyDoctorStateView {
    /// Active marked items in `To send`.
    pub marks: Vec<ForMyDoctorMarkView>,
    /// Unsent free-text notes in `To send`.
    pub notes: Vec<ForMyDoctorNoteView>,
    /// Immutable flat list of sends (newest first).
    pub sends: Vec<ForMyDoctorSendView>,
    /// Active notices in `You` (source removed a marked item).
    pub notices: Vec<ForMyDoctorNoticeView>,
    /// Connected and ended practices known to `front-door`.
    pub practices: Vec<PracticeConnectionView>,
}

/// Request to mark a record, value, reading, or trend (`PUT /api/for-my-doctor/marks`).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarkForDoctorRequest {
    /// Target kind (`record`, `value`, `reading`, `trend`).
    pub target_kind: MarkTargetKind,
    /// Deduplicating key scoped to the account.
    pub target_key: String,
    /// Optional canonical record id.
    pub record_id: Option<Uuid>,
    /// Optional canonical entity family (`health_evidence` or `clinical_note`).
    pub entity: Option<RecordFamily>,
    /// Human-readable title.
    pub title: String,
    /// Plain summary (reference ranges stripped server-side).
    pub summary_text: String,
    /// Viewed range label for trends.
    pub range_label: Option<String>,
    /// Plain source label.
    pub source_label: Option<String>,
    /// Optional structured payload (reference ranges stripped server-side).
    #[serde(default)]
    pub payload: Value,
}

/// Request to remove one mark (`DELETE /api/for-my-doctor/marks`).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnmarkRequest {
    /// Canonical `target_key` to unmark.
    pub target_key: String,
}

/// Request to create an unsent note (`POST /api/for-my-doctor/notes`).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateDoctorNoteRequest {
    /// Client-generated note id (idempotent on retry).
    pub id: Uuid,
    /// Free-text note body.
    pub body: String,
    /// Optional canonical record linked when written from an open record in `You`.
    pub linked_record_id: Option<Uuid>,
    /// Optional linked record display title.
    pub linked_record_title: Option<String>,
    /// Optional linked record family.
    pub linked_entity: Option<RecordFamily>,
}

/// Request to update an existing note (`PUT /api/for-my-doctor/notes/{id}`).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateDoctorNoteRequest {
    /// Updated free-text note body.
    pub body: String,
    /// Optimistic concurrency version expected by the client.
    pub expected_version: i64,
}

/// Single open record passed directly from `You` -> `Choosing` -> `Send`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdHocRecordSend {
    /// Optional canonical record id.
    pub record_id: Option<Uuid>,
    /// Canonical entity family if canonical.
    pub entity: Option<RecordFamily>,
    /// Display title.
    pub title: String,
    /// Plain summary text.
    pub summary_text: String,
    /// Plain source label.
    pub source_label: Option<String>,
}

/// Request to send selected `To send` items (or a single open record) to one connected practice.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendForMyDoctorPayload {
    /// Client-generated idempotency key.
    pub share_key: String,
    /// Target connected practice identifier.
    pub practice_id: String,
    /// Target practice display name.
    pub practice_name: String,
    /// Roster patient identifier within the target practice.
    pub patient_id: String,
    /// Patient confirmed the mandatory non-urgent / cannot-be-taken-back disclosure.
    pub disclosure_accepted: bool,
    /// Selected marks from `To send`.
    #[serde(default)]
    pub selected_mark_ids: Vec<Uuid>,
    /// Selected notes from `To send`.
    #[serde(default)]
    pub selected_note_ids: Vec<Uuid>,
    /// Optional single open record sent directly from `You`.
    #[serde(default)]
    pub ad_hoc_record: Option<AdHocRecordSend>,
}

/// Structured send error returned so the UI distinguishes uncertain vs connection-ended vs conflict.
#[derive(Debug)]
pub enum ForMyDoctorSendError {
    /// Underlying account/session/terms refusal (`session_revoked`, `account_absent`, `TermsRequired`, etc.).
    Api(ApiError),
    /// Missing disclosure or empty selection.
    Invalid(&'static str),
    /// Practice connection ended (during initial `Sending` or while `Couldn't confirm the send`).
    ConnectionEnded {
        /// Plain-language reason.
        reason: String,
        /// Optional Unix second the connection ended.
        ended_at: Option<u64>,
    },
    /// Optimistic conflict (e.g., reused `share_key` with different payload).
    Conflict(String),
    /// Transport/storage uncertainty (`503`): hold `"Couldn't confirm the send"` + `"Try again"`.
    Uncertain(String),
}

impl From<ApiError> for ForMyDoctorSendError {
    fn from(err: ApiError) -> Self {
        match err {
            ApiError::Unauthorized
            | ApiError::InvalidToken(_)
            | ApiError::SessionExpired
            | ApiError::SessionRevoked
            | ApiError::AbsentAccount
            | ApiError::Forbidden
            | ApiError::TermsRequired
            | ApiError::AgreementChanged => Self::Api(err),
            ApiError::Conflict(msg) => Self::Conflict(msg.to_owned()),
            ApiError::Unprocessable(msg) => Self::Invalid(msg),
            other => Self::Uncertain(format!("{other:?}")),
        }
    }
}

impl axum::response::IntoResponse for ForMyDoctorSendError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Api(err) => err.into_response(),
            Self::Invalid(msg) => ApiError::Unprocessable(msg).into_response(),
            Self::ConnectionEnded { reason, ended_at } => (
                StatusCode::CONFLICT,
                Json(json!({
                    "code": "connection_ended",
                    "detail": reason,
                    "ended_at": ended_at,
                })),
            )
                .into_response(),
            Self::Conflict(msg) => (
                StatusCode::CONFLICT,
                Json(json!({
                    "code": "conflict",
                    "detail": msg,
                })),
            )
                .into_response(),
            Self::Uncertain(msg) => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "code": "uncertain",
                    "detail": msg,
                })),
            )
                .into_response(),
        }
    }
}

/// Structured note update conflict carrying the latest saved server version (`Rule 9`).
#[derive(Debug)]
pub enum NoteUpdateError {
    /// Underlying API error.
    Api(ApiError),
    /// Newer server version exists; return it so the client displays it while keeping unsaved text.
    VersionConflict(Box<ForMyDoctorNoteView>),
}

impl From<ApiError> for NoteUpdateError {
    fn from(err: ApiError) -> Self {
        Self::Api(err)
    }
}

impl axum::response::IntoResponse for NoteUpdateError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Api(err) => err.into_response(),
            Self::VersionConflict(latest) => (
                StatusCode::CONFLICT,
                Json(json!({
                    "code": "version_conflict",
                    "detail": "Note was edited on another device.",
                    "latest": latest,
                })),
            )
                .into_response(),
        }
    }
}

/// Account-scoped owner for `"For my doctor"` marks, notes, sends, and source-removal notices.
#[derive(Clone)]
pub struct ForMyDoctorStore {
    database: crate::database::AccountDatabase,
    front_door: Option<Arc<dyn ForMyDoctorFrontDoorTransport>>,
}

impl ForMyDoctorStore {
    /// Builds a store with optional `front-door` transport.
    pub fn new(
        database: crate::database::AccountDatabase,
        front_door: Option<Arc<dyn ForMyDoctorFrontDoorTransport>>,
    ) -> Self {
        Self {
            database,
            front_door,
        }
    }

    /// Reads the current `"For my doctor"` state (`To send` marks/notes, `Sent` list, notices, practices).
    pub async fn read_state(
        &self,
        context: &AccountContext,
        front_door_bearer: Option<&str>,
    ) -> Result<ForMyDoctorStateView, ApiError> {
        let now = Utc::now();
        let overview = self.fetch_overview(context, front_door_bearer).await?;

        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, AccessLevel::AcceptedTerms).await?;

        reconcile_marks_in_tx(&mut tx, context.account_id, now).await?;
        sync_sends_with_overview_in_tx(&mut tx, context.account_id, &overview).await?;

        let state = load_state_in_tx(&mut tx, context.account_id, &overview.connections).await?;
        tx.commit().await?;
        Ok(state)
    }

    /// Marks (upserts) one record, value, reading, or trend in `To send`.
    pub async fn mark(
        &self,
        context: &AccountContext,
        request: &MarkForDoctorRequest,
    ) -> Result<ForMyDoctorMarkView, ApiError> {
        let key = request.target_key.trim();
        let title = request.title.trim();
        if key.is_empty() || title.is_empty() {
            return Err(ApiError::Unprocessable("mark key and title are required"));
        }

        let now = Utc::now();
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, AccessLevel::AcceptedTerms).await?;

        let mut resolved_title = title.to_owned();
        let mut resolved_summary = sanitize_summary(&request.summary_text);
        let mut resolved_payload = strip_reference_ranges(&request.payload);
        let mut resolved_rev: Option<i32> = None;

        if let Some(record_id) = request.record_id {
            let snap = load_current_canonical_record(
                &mut tx,
                context.account_id,
                record_id,
                request.entity,
            )
            .await?
            .ok_or(ApiError::Conflict(
                "cannot mark a record that was removed or deleted",
            ))?;
            resolved_rev = Some(snap.revision);
            if request.target_kind == MarkTargetKind::Record {
                resolved_title = snap.title;
                if !snap.summary_text.is_empty() {
                    resolved_summary = snap.summary_text;
                }
                resolved_payload = snap.payload;
            }
        }

        let existing = orm::model::<mark::Model>(
            &mut tx,
            &mark::Entity::find()
                .filter(mark::Column::UserId.eq(context.account_id))
                .filter(mark::Column::TargetKey.eq(key))
                .into_query(),
        )
        .await?;

        let row = if let Some(existing) = existing {
            orm::execute(
                &mut tx,
                &mark::Entity::update_many()
                    .col_expr(mark::Column::Title, resolved_title.clone().into())
                    .col_expr(mark::Column::SummaryText, resolved_summary.clone().into())
                    .col_expr(mark::Column::RangeLabel, request.range_label.clone().into())
                    .col_expr(
                        mark::Column::SourceLabel,
                        request.source_label.clone().into(),
                    )
                    .col_expr(mark::Column::RecordRevision, resolved_rev.into())
                    .col_expr(mark::Column::Payload, resolved_payload.clone().into())
                    .col_expr(mark::Column::UpdatedAt, now.into())
                    .filter(mark::Column::Id.eq(existing.id))
                    .filter(mark::Column::UserId.eq(context.account_id))
                    .into_query(),
            )
            .await?;
            mark::Model {
                title: resolved_title,
                summary_text: resolved_summary,
                range_label: request.range_label.clone(),
                source_label: request.source_label.clone(),
                record_revision: resolved_rev,
                payload: resolved_payload,
                updated_at: now,
                ..existing
            }
        } else {
            let model = mark::Model {
                id: Uuid::new_v4(),
                user_id: context.account_id,
                target_kind: request.target_kind,
                target_key: key.to_owned(),
                record_id: request.record_id,
                entity: request.entity,
                record_revision: resolved_rev,
                marked_revision: resolved_rev,
                title: resolved_title,
                summary_text: resolved_summary,
                range_label: request.range_label.clone(),
                source_label: request.source_label.clone(),
                source_corrected: false,
                correction_date: None,
                payload: resolved_payload,
                marked_at: now,
                updated_at: now,
            };
            orm::execute(
                &mut tx,
                &mark::Entity::insert(mark::ActiveModel {
                    id: Set(model.id),
                    user_id: Set(model.user_id),
                    target_kind: Set(model.target_kind),
                    target_key: Set(model.target_key.clone()),
                    record_id: Set(model.record_id),
                    entity: Set(model.entity),
                    record_revision: Set(model.record_revision),
                    marked_revision: Set(model.marked_revision),
                    title: Set(model.title.clone()),
                    summary_text: Set(model.summary_text.clone()),
                    range_label: Set(model.range_label.clone()),
                    source_label: Set(model.source_label.clone()),
                    source_corrected: Set(false),
                    correction_date: Set(None),
                    payload: Set(model.payload.clone()),
                    marked_at: Set(model.marked_at),
                    updated_at: Set(model.updated_at),
                })
                .into_query(),
            )
            .await?;
            model
        };

        tx.commit().await?;
        Ok(mark_to_view(row))
    }

    /// Removes one mark from `To send` by `target_key`.
    pub async fn unmark(&self, context: &AccountContext, target_key: &str) -> Result<(), ApiError> {
        let key = target_key.trim();
        if key.is_empty() {
            return Err(ApiError::Unprocessable("target_key is required"));
        }
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, AccessLevel::AcceptedTerms).await?;
        orm::execute(
            &mut tx,
            &mark::Entity::delete_many()
                .filter(mark::Column::UserId.eq(context.account_id))
                .filter(mark::Column::TargetKey.eq(key))
                .into_query(),
        )
        .await?;
        tx.commit().await?;
        Ok(())
    }

    /// Creates an unsent free-text note in `To send`, idempotent by `request.id`.
    pub async fn create_note(
        &self,
        context: &AccountContext,
        request: &CreateDoctorNoteRequest,
    ) -> Result<ForMyDoctorNoteView, ApiError> {
        let body = request.body.trim();
        if body.is_empty() {
            return Err(ApiError::Unprocessable("note text is required"));
        }
        let now = Utc::now();
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, AccessLevel::AcceptedTerms).await?;

        if let Some(existing) = orm::model::<note::Model>(
            &mut tx,
            &note::Entity::find()
                .filter(note::Column::UserId.eq(context.account_id))
                .filter(note::Column::Id.eq(request.id))
                .into_query(),
        )
        .await?
        {
            tx.commit().await?;
            return Ok(note_to_view(existing));
        }

        let model = note::Model {
            id: request.id,
            user_id: context.account_id,
            body: body.to_owned(),
            linked_record_id: request.linked_record_id,
            linked_record_title: request.linked_record_title.clone(),
            linked_entity: request.linked_entity,
            version: 1,
            created_at: now,
            updated_at: now,
        };
        orm::execute(
            &mut tx,
            &note::Entity::insert(note::ActiveModel {
                id: Set(model.id),
                user_id: Set(model.user_id),
                body: Set(model.body.clone()),
                linked_record_id: Set(model.linked_record_id),
                linked_record_title: Set(model.linked_record_title.clone()),
                linked_entity: Set(model.linked_entity),
                version: Set(1),
                created_at: Set(now),
                updated_at: Set(now),
            })
            .into_query(),
        )
        .await?;

        tx.commit().await?;
        Ok(note_to_view(model))
    }

    /// Updates an existing note when `expected_version` matches; otherwise returns `VersionConflict(latest)`.
    pub async fn update_note(
        &self,
        context: &AccountContext,
        note_id: Uuid,
        request: &UpdateDoctorNoteRequest,
    ) -> Result<ForMyDoctorNoteView, NoteUpdateError> {
        let body = request.body.trim();
        if body.is_empty() {
            return Err(NoteUpdateError::Api(ApiError::Unprocessable(
                "note text is required",
            )));
        }
        let now = Utc::now();
        let mut tx = self
            .database
            .begin(context.account_id)
            .await
            .map_err(ApiError::from)?;
        check_current(&mut tx, context, AccessLevel::AcceptedTerms).await?;

        let existing = orm::model::<note::Model>(
            &mut tx,
            &note::Entity::find()
                .filter(note::Column::UserId.eq(context.account_id))
                .filter(note::Column::Id.eq(note_id))
                .into_query(),
        )
        .await
        .map_err(ApiError::from)?
        .ok_or(ApiError::NotFound)?;

        if existing.version != request.expected_version {
            tx.commit().await.map_err(ApiError::from)?;
            return Err(NoteUpdateError::VersionConflict(Box::new(note_to_view(
                existing,
            ))));
        }

        let next_version = existing.version.saturating_add(1);
        orm::execute(
            &mut tx,
            &note::Entity::update_many()
                .col_expr(note::Column::Body, body.to_owned().into())
                .col_expr(note::Column::Version, next_version.into())
                .col_expr(note::Column::UpdatedAt, now.into())
                .filter(note::Column::UserId.eq(context.account_id))
                .filter(note::Column::Id.eq(note_id))
                .into_query(),
        )
        .await
        .map_err(ApiError::from)?;

        tx.commit().await.map_err(ApiError::from)?;
        Ok(note_to_view(note::Model {
            body: body.to_owned(),
            version: next_version,
            updated_at: now,
            ..existing
        }))
    }

    /// Deletes one note from `To send`. Earlier sends retain their frozen copies.
    pub async fn delete_note(
        &self,
        context: &AccountContext,
        note_id: Uuid,
    ) -> Result<(), ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, AccessLevel::AcceptedTerms).await?;
        orm::execute(
            &mut tx,
            &note::Entity::delete_many()
                .filter(note::Column::UserId.eq(context.account_id))
                .filter(note::Column::Id.eq(note_id))
                .into_query(),
        )
        .await?;
        tx.commit().await?;
        Ok(())
    }

    /// Dismisses one source-removal notice in `You`.
    pub async fn dismiss_notice(
        &self,
        context: &AccountContext,
        notice_id: Uuid,
    ) -> Result<(), ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, AccessLevel::AcceptedTerms).await?;
        orm::execute(
            &mut tx,
            &notice::Entity::update_many()
                .col_expr(notice::Column::DismissedAt, Some(Utc::now()).into())
                .filter(notice::Column::UserId.eq(context.account_id))
                .filter(notice::Column::Id.eq(notice_id))
                .into_query(),
        )
        .await?;
        tx.commit().await?;
        Ok(())
    }

    /// Freezes the chosen items at current version, forwards idempotently to `front-door`,
    /// records `Sent`, and clears only the marks & notes carried by this send.
    pub async fn send(
        &self,
        context: &AccountContext,
        front_door_bearer: Option<&str>,
        payload: &SendForMyDoctorPayload,
    ) -> Result<ForMyDoctorSendView, ForMyDoctorSendError> {
        let share_key = payload.share_key.trim();
        let practice_id = payload.practice_id.trim();
        let patient_id = payload.patient_id.trim();
        let practice_name = payload.practice_name.trim();
        if share_key.is_empty() || practice_id.is_empty() || patient_id.is_empty() {
            return Err(ForMyDoctorSendError::Invalid(
                "share_key, practice_id, and patient_id are required",
            ));
        }
        if !payload.disclosure_accepted {
            return Err(ForMyDoctorSendError::Invalid(
                "Please confirm the non-urgent, permanent send disclosure first.",
            ));
        }
        if payload.selected_mark_ids.is_empty()
            && payload.selected_note_ids.is_empty()
            && payload.ad_hoc_record.is_none()
        {
            return Err(ForMyDoctorSendError::Invalid(
                "Choose at least one record or note to send.",
            ));
        }

        // Fast idempotency read before touching front-door or mutating marks.
        {
            let mut tx = self
                .database
                .begin(context.account_id)
                .await
                .map_err(ApiError::from)?;
            check_current(&mut tx, context, AccessLevel::AcceptedTerms).await?;
            if let Some(existing) = orm::model::<send::Model>(
                &mut tx,
                &send::Entity::find()
                    .filter(send::Column::UserId.eq(context.account_id))
                    .filter(send::Column::ShareKey.eq(share_key))
                    .into_query(),
            )
            .await
            .map_err(ApiError::from)?
            {
                tx.commit().await.map_err(ApiError::from)?;
                return Ok(send_to_view(existing));
            }
            tx.commit().await.map_err(ApiError::from)?;
        }

        let transport = self.front_door.as_ref().ok_or_else(|| {
            ForMyDoctorSendError::Uncertain("Connected practice service is unavailable.".to_owned())
        })?;
        let bearer = front_door_bearer
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or(ForMyDoctorSendError::Api(ApiError::Unauthorized))?;

        let mapped_account_id = {
            let mut tx = self
                .database
                .begin(context.account_id)
                .await
                .map_err(ApiError::from)?;
            check_current(&mut tx, context, AccessLevel::AcceptedTerms).await?;
            let id = front_door_account_id(&mut tx, context.account_id)
                .await?
                .ok_or(ForMyDoctorSendError::ConnectionEnded {
                    reason: "No connected practice was found for this account.".to_owned(),
                    ended_at: None,
                })?;
            tx.commit().await.map_err(ApiError::from)?;
            id
        };

        // Reconcile marks & assemble frozen snapshot at current versions.
        let now = Utc::now();
        let (title, summary_text, records, notes, clear_mark_ids, clear_note_ids) = {
            let mut tx = self
                .database
                .begin(context.account_id)
                .await
                .map_err(ApiError::from)?;
            check_current(&mut tx, context, AccessLevel::AcceptedTerms).await?;
            reconcile_marks_in_tx(&mut tx, context.account_id, now).await?;

            let mut selected_marks = Vec::new();
            if !payload.selected_mark_ids.is_empty() {
                let all_marks = orm::models::<mark::Model>(
                    &mut tx,
                    &mark::Entity::find()
                        .filter(mark::Column::UserId.eq(context.account_id))
                        .order_by_asc(mark::Column::MarkedAt)
                        .into_query(),
                )
                .await
                .map_err(ApiError::from)?;
                let want: HashSet<Uuid> = payload.selected_mark_ids.iter().copied().collect();
                selected_marks.extend(all_marks.into_iter().filter(|m| want.contains(&m.id)));
            }

            let mut selected_notes = Vec::new();
            if !payload.selected_note_ids.is_empty() {
                let all_notes = orm::models::<note::Model>(
                    &mut tx,
                    &note::Entity::find()
                        .filter(note::Column::UserId.eq(context.account_id))
                        .order_by_asc(note::Column::CreatedAt)
                        .into_query(),
                )
                .await
                .map_err(ApiError::from)?;
                let want: HashSet<Uuid> = payload.selected_note_ids.iter().copied().collect();
                selected_notes.extend(all_notes.into_iter().filter(|n| want.contains(&n.id)));
            }

            if selected_marks.is_empty()
                && selected_notes.is_empty()
                && payload.ad_hoc_record.is_none()
            {
                return Err(ForMyDoctorSendError::Invalid(
                    "Chosen items are no longer in To send.",
                ));
            }

            let mut records: Vec<SentRecordEntry> = Vec::new();
            let mut included_record_ids: HashSet<Uuid> = HashSet::new();

            for m in &selected_marks {
                if let Some(rid) = m.record_id {
                    included_record_ids.insert(rid);
                }
                records.push(SentRecordEntry {
                    record_id: m.record_id,
                    title: m.title.clone(),
                    summary_text: sanitize_summary(&m.summary_text),
                    source_label: m.source_label.clone(),
                    version: m.record_revision,
                    correction_date: m.correction_date.map(|dt| dt.to_rfc3339()),
                    range_label: m.range_label.clone(),
                });
            }

            if let Some(ad_hoc) = &payload.ad_hoc_record {
                let mut entry = SentRecordEntry {
                    record_id: ad_hoc.record_id,
                    title: ad_hoc.title.trim().to_owned(),
                    summary_text: sanitize_summary(&ad_hoc.summary_text),
                    source_label: ad_hoc.source_label.clone(),
                    version: None,
                    correction_date: None,
                    range_label: None,
                };
                if let Some(rid) = ad_hoc.record_id {
                    included_record_ids.insert(rid);
                    if let Some(snap) = load_current_canonical_record(
                        &mut tx,
                        context.account_id,
                        rid,
                        ad_hoc.entity,
                    )
                    .await?
                    {
                        entry.title = snap.title;
                        if !snap.summary_text.is_empty() {
                            entry.summary_text = snap.summary_text;
                        }
                        entry.version = Some(snap.revision);
                        entry.correction_date = snap.correction_date;
                    }
                }
                records.push(entry);
            }

            let mut frozen_notes = Vec::new();
            for n in &selected_notes {
                if let Some(rid) = n.linked_record_id
                    && !included_record_ids.contains(&rid)
                {
                    // Rule 3: a note written from a record takes that record with it.
                    if let Some(snap) = load_current_canonical_record(
                        &mut tx,
                        context.account_id,
                        rid,
                        n.linked_entity,
                    )
                    .await?
                    {
                        included_record_ids.insert(rid);
                        records.push(SentRecordEntry {
                            record_id: Some(rid),
                            title: snap.title,
                            summary_text: snap.summary_text,
                            source_label: snap.source_label,
                            version: Some(snap.revision),
                            correction_date: snap.correction_date,
                            range_label: None,
                        });
                    } else if let Some(title) = &n.linked_record_title {
                        included_record_ids.insert(rid);
                        records.push(SentRecordEntry {
                            record_id: Some(rid),
                            title: title.clone(),
                            summary_text: String::new(),
                            source_label: None,
                            version: None,
                            correction_date: None,
                            range_label: None,
                        });
                    }
                }
                frozen_notes.push(SentNoteEntry {
                    id: n.id,
                    body: n.body.clone(),
                    linked_record_title: n.linked_record_title.clone(),
                });
            }

            let title = summarize_send_title(&records, &frozen_notes);
            let summary_text = render_send_summary(&records, &frozen_notes);
            let clear_mark_ids: Vec<Uuid> = selected_marks.iter().map(|m| m.id).collect();
            let clear_note_ids: Vec<Uuid> = selected_notes.iter().map(|n| n.id).collect();
            tx.commit().await.map_err(ApiError::from)?;
            (
                title,
                summary_text,
                records,
                frozen_notes,
                clear_mark_ids,
                clear_note_ids,
            )
        };

        // Verify whether an earlier uncertain attempt already landed at front-door before failing as ended.
        let fd_req = ForMyDoctorSendRequest {
            share_key: share_key.to_owned(),
            title: title.clone(),
            summary_text: summary_text.clone(),
            origin_label: Some("Duet — For my doctor".to_owned()),
            target: Some(RosterPatient {
                practice_id: practice_id.to_owned(),
                patient_id: patient_id.to_owned(),
            }),
        };

        let share = match transport
            .send_share(mapped_account_id, bearer, &fd_req)
            .await
        {
            Ok(share) => share,
            Err(FrontDoorCallError::ConnectionEnded { reason, ended_at }) => {
                // Check if an earlier attempt with this exact share_key succeeded before disconnect.
                if let Ok(overview) = transport.account_overview(mapped_account_id, bearer).await
                    && let Some(existing_share) = overview.shares.into_iter().find(|s| {
                        s.share_key == share_key && s.patient == fd_req.target.clone().unwrap()
                    })
                {
                    return self
                        .persist_committed_send(
                            context,
                            existing_share,
                            practice_name,
                            ended_at.and_then(|ts| Utc.timestamp_opt(ts as i64, 0).single()),
                            records,
                            notes,
                            clear_mark_ids,
                            clear_note_ids,
                        )
                        .await;
                }
                return Err(ForMyDoctorSendError::ConnectionEnded { reason, ended_at });
            }
            Err(FrontDoorCallError::Unauthenticated) => {
                return Err(ForMyDoctorSendError::Api(ApiError::Unauthorized));
            }
            Err(FrontDoorCallError::Conflict(msg)) => {
                return Err(ForMyDoctorSendError::Conflict(msg));
            }
            Err(FrontDoorCallError::Unavailable(msg)) => {
                return Err(ForMyDoctorSendError::Uncertain(msg));
            }
        };

        self.persist_committed_send(
            context,
            share,
            practice_name,
            None,
            records,
            notes,
            clear_mark_ids,
            clear_note_ids,
        )
        .await
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "commits one frozen send with its cleared To send IDs in a single transaction"
    )]
    async fn persist_committed_send(
        &self,
        context: &AccountContext,
        share: ForMyDoctorShare,
        practice_name: &str,
        connection_ended_at: Option<DateTime<Utc>>,
        records: Vec<SentRecordEntry>,
        notes: Vec<SentNoteEntry>,
        clear_mark_ids: Vec<Uuid>,
        clear_note_ids: Vec<Uuid>,
    ) -> Result<ForMyDoctorSendView, ForMyDoctorSendError> {
        let resolved_practice_name = if practice_name.trim().is_empty() {
            share.patient.practice_id.clone()
        } else {
            practice_name.trim().to_owned()
        };
        let sent_at = Utc
            .timestamp_opt(share.sent_at as i64, 0)
            .single()
            .unwrap_or_else(Utc::now);
        let (standing, received_at) = match share.standing {
            ForMyDoctorShareStatus::Received { received_at } => {
                let rec = Utc.timestamp_opt(received_at as i64, 0).single();
                let ended_first = match (rec, connection_ended_at) {
                    (Some(r), Some(e)) => r > e,
                    (None, Some(_)) => true,
                    _ => false,
                };
                if ended_first {
                    (StoredSendStanding::Sent, None)
                } else {
                    (StoredSendStanding::Received, rec)
                }
            }
            ForMyDoctorShareStatus::Sent => (StoredSendStanding::Sent, None),
        };
        let snapshot = json!({
            "records": records,
            "notes": notes,
        });
        let row = send::Model {
            id: Uuid::new_v4(),
            user_id: context.account_id,
            share_key: share.share_key.clone(),
            practice_id: share.patient.practice_id.clone(),
            practice_name: resolved_practice_name,
            patient_id: share.patient.patient_id.clone(),
            title: share.title.clone(),
            summary_text: share.summary_text.clone(),
            snapshot,
            mark_ids: clear_mark_ids.clone(),
            note_ids: clear_note_ids.clone(),
            front_door_share_id: Some(share.share_id),
            standing,
            sent_at,
            received_at,
            connection_ended_at,
        };

        let mut tx = self
            .database
            .begin(context.account_id)
            .await
            .map_err(ApiError::from)?;
        check_current(&mut tx, context, AccessLevel::AcceptedTerms).await?;

        if let Some(existing) = orm::model::<send::Model>(
            &mut tx,
            &send::Entity::find()
                .filter(send::Column::UserId.eq(context.account_id))
                .filter(send::Column::ShareKey.eq(&row.share_key))
                .into_query(),
        )
        .await
        .map_err(ApiError::from)?
        {
            tx.commit().await.map_err(ApiError::from)?;
            return Ok(send_to_view(existing));
        }

        orm::execute(
            &mut tx,
            &send::Entity::insert(send::ActiveModel {
                id: Set(row.id),
                user_id: Set(row.user_id),
                share_key: Set(row.share_key.clone()),
                practice_id: Set(row.practice_id.clone()),
                practice_name: Set(row.practice_name.clone()),
                patient_id: Set(row.patient_id.clone()),
                title: Set(row.title.clone()),
                summary_text: Set(row.summary_text.clone()),
                snapshot: Set(row.snapshot.clone()),
                mark_ids: Set(row.mark_ids.clone()),
                note_ids: Set(row.note_ids.clone()),
                front_door_share_id: Set(row.front_door_share_id.clone()),
                standing: Set(row.standing),
                sent_at: Set(row.sent_at),
                received_at: Set(row.received_at),
                connection_ended_at: Set(row.connection_ended_at),
            })
            .on_conflict(
                OnConflict::columns([send::Column::UserId, send::Column::ShareKey])
                    .do_nothing()
                    .to_owned(),
            )
            .into_query(),
        )
        .await
        .map_err(ApiError::from)?;

        if !clear_mark_ids.is_empty() {
            orm::execute(
                &mut tx,
                &mark::Entity::delete_many()
                    .filter(mark::Column::UserId.eq(context.account_id))
                    .filter(mark::Column::Id.is_in(clear_mark_ids))
                    .into_query(),
            )
            .await
            .map_err(ApiError::from)?;
        }
        if !clear_note_ids.is_empty() {
            orm::execute(
                &mut tx,
                &note::Entity::delete_many()
                    .filter(note::Column::UserId.eq(context.account_id))
                    .filter(note::Column::Id.is_in(clear_note_ids))
                    .into_query(),
            )
            .await
            .map_err(ApiError::from)?;
        }

        tx.commit().await.map_err(ApiError::from)?;
        Ok(send_to_view(row))
    }

    async fn fetch_overview(
        &self,
        context: &AccountContext,
        front_door_bearer: Option<&str>,
    ) -> Result<FrontDoorOverview, ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, AccessLevel::AcceptedTerms).await?;
        let Some(transport) = &self.front_door else {
            return Ok(FrontDoorOverview::default());
        };
        let Some(token) = front_door_bearer.map(str::trim).filter(|s| !s.is_empty()) else {
            return Ok(FrontDoorOverview::default());
        };
        let mapped = front_door_account_id(&mut tx, context.account_id).await?;
        tx.commit().await?;
        let Some(fd_id) = mapped else {
            return Ok(FrontDoorOverview::default());
        };
        Ok(transport
            .account_overview(fd_id, token)
            .await
            .unwrap_or_default())
    }
}

struct CanonicalSnapshot {
    revision: i32,
    title: String,
    summary_text: String,
    source_label: Option<String>,
    correction_date: Option<String>,
    payload: Value,
}

async fn load_current_canonical_record(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    account_id: Uuid,
    record_id: Uuid,
    hint_entity: Option<RecordFamily>,
) -> Result<Option<CanonicalSnapshot>, ApiError> {
    let mut members = orm::models::<membership::Model>(
        tx,
        &membership::Entity::find()
            .filter(membership::Column::UserId.eq(account_id))
            .filter(membership::Column::RecordId.eq(record_id))
            .into_query(),
    )
    .await?;
    if let Some(hint) = hint_entity {
        members.retain(|m| m.entity_kind == hint.as_str());
    }
    let Some(member) = members.into_iter().next() else {
        return Ok(None);
    };
    if member.deleted {
        return Ok(None);
    }

    let is_note = member.entity_kind == RecordFamily::ClinicalNote.as_str();
    let rev_row = if is_note {
        orm::model::<revision::Model>(
            tx,
            &revision::Entity::find()
                .filter(revision::Column::UserId.eq(account_id))
                .filter(revision::Column::NoteId.eq(record_id))
                .order_by_desc(revision::Column::Revision)
                .into_query(),
        )
        .await?
    } else {
        orm::model::<revision::Model>(
            tx,
            &revision::Entity::find()
                .filter(revision::Column::UserId.eq(account_id))
                .filter(revision::Column::EvidenceId.eq(record_id))
                .order_by_desc(revision::Column::Revision)
                .into_query(),
        )
        .await?
    };

    let (revision, raw_payload, updated_at) = if let Some(rev) = rev_row {
        let updated = if is_note {
            None
        } else {
            orm::model::<health_evidence::Model>(
                tx,
                &health_evidence::Entity::find()
                    .filter(health_evidence::Column::UserId.eq(account_id))
                    .filter(health_evidence::Column::Id.eq(record_id))
                    .into_query(),
            )
            .await?
            .map(|e| e.updated_at.to_rfc3339())
        };
        (rev.revision as i32, rev.payload, updated)
    } else if is_note {
        let Some(note_row) = orm::model::<clinical_note::Model>(
            tx,
            &clinical_note::Entity::find()
                .filter(clinical_note::Column::UserId.eq(account_id))
                .filter(clinical_note::Column::Id.eq(record_id))
                .into_query(),
        )
        .await?
        else {
            return Ok(None);
        };
        let payload = note_row.duet_source_payload.clone().unwrap_or_else(|| {
            json!({
                "title": note_row.title,
                "body": note_row.body,
                "source_label": note_row.source_label,
            })
        });
        (member.revision as i32, payload, None)
    } else {
        let Some(ev) = orm::model::<health_evidence::Model>(
            tx,
            &health_evidence::Entity::find()
                .filter(health_evidence::Column::UserId.eq(account_id))
                .filter(health_evidence::Column::Id.eq(record_id))
                .into_query(),
        )
        .await?
        else {
            return Ok(None);
        };
        (
            member.revision as i32,
            ev.payload,
            Some(ev.updated_at.to_rfc3339()),
        )
    };

    let clean_payload = strip_reference_ranges(&raw_payload);
    let (title, summary_text, source_label, is_corrected) = project_clean_record(&clean_payload);
    Ok(Some(CanonicalSnapshot {
        revision,
        title,
        summary_text,
        source_label,
        correction_date: is_corrected.then_some(updated_at).flatten(),
        payload: clean_payload,
    }))
}

fn project_clean_record(payload: &Value) -> (String, String, Option<String>, bool) {
    let title = ["display_name", "title", "name", "concept_id"]
        .into_iter()
        .find_map(|k| payload.get(k).and_then(Value::as_str))
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("Health record")
        .to_owned();

    let source_label = payload
        .get("source_label")
        .and_then(Value::as_str)
        .or_else(|| {
            payload
                .get("source")
                .and_then(|s| s.get("source_name").or_else(|| s.get("kind")))
                .and_then(Value::as_str)
        })
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned);

    let summary_text = if let Some(body) = payload
        .get("body")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        body.to_owned()
    } else if let Some(summary) = payload
        .get("summary")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        summary.to_owned()
    } else {
        let value = payload
            .get("value")
            .and_then(|v| {
                v.as_str()
                    .map(str::to_owned)
                    .or_else(|| v.as_f64().map(|n| n.to_string()))
                    .or_else(|| v.get("literal").and_then(Value::as_str).map(str::to_owned))
            })
            .unwrap_or_default();
        let unit = payload
            .get("unit")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if !value.is_empty() && !unit.is_empty() {
            format!("{value} {unit}")
        } else {
            value
        }
    };

    let status = payload
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_ascii_lowercase();
    let corrected =
        status == "corrected" || status == "amended" || payload.get("corrects").is_some();

    (
        title,
        sanitize_summary(&summary_text),
        source_label,
        corrected,
    )
}

fn sanitize_summary(raw: &str) -> String {
    raw.lines()
        .map(str::trim)
        .filter(|line| {
            let lower = line.to_ascii_lowercase();
            !line.is_empty()
                && !lower.contains("reference range")
                && !lower.contains("normal range")
                && !lower.contains("ref range")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Reconciles every mark linked to a canonical record against source updates and
/// source removals (`Rule 10` and `COD-50`) inside the caller's RLS transaction.
pub(crate) async fn reconcile_marks_in_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    account_id: Uuid,
    now: DateTime<Utc>,
) -> Result<(), ApiError> {
    let marks = orm::models::<mark::Model>(
        tx,
        &mark::Entity::find()
            .filter(mark::Column::UserId.eq(account_id))
            .into_query(),
    )
    .await?;

    for m in marks {
        let Some(record_id) = m.record_id else {
            continue;
        };
        let mut members = orm::models::<membership::Model>(
            tx,
            &membership::Entity::find()
                .filter(membership::Column::UserId.eq(account_id))
                .filter(membership::Column::RecordId.eq(record_id))
                .into_query(),
        )
        .await?;
        if let Some(hint) = m.entity {
            members.retain(|row| row.entity_kind == hint.as_str());
        }
        let member = members.into_iter().next();

        let deleted = member.as_ref().is_none_or(|row| row.deleted);
        if deleted {
            orm::execute(
                tx,
                &mark::Entity::delete_many()
                    .filter(mark::Column::UserId.eq(account_id))
                    .filter(mark::Column::Id.eq(m.id))
                    .into_query(),
            )
            .await?;

            if member.is_some_and(|row| row.source_removed) {
                let message = format!(
                    "{} was removed by its source and cleared from To send.",
                    m.title
                );
                orm::execute(
                    tx,
                    &notice::Entity::insert(notice::ActiveModel {
                        id: Set(Uuid::new_v4()),
                        user_id: Set(account_id),
                        title: Set(m.title),
                        message: Set(message),
                        dismissed_at: Set(None),
                        created_at: Set(now),
                    })
                    .into_query(),
                )
                .await?;
            }
            continue;
        }

        if let Some(snap) =
            load_current_canonical_record(tx, account_id, record_id, m.entity).await?
            && m.record_revision.is_some_and(|prev| snap.revision > prev)
        {
            orm::execute(
                tx,
                &mark::Entity::update_many()
                    .col_expr(mark::Column::RecordRevision, Some(snap.revision).into())
                    .col_expr(mark::Column::Title, snap.title.into())
                    .col_expr(mark::Column::SummaryText, snap.summary_text.into())
                    .col_expr(mark::Column::Payload, snap.payload.into())
                    .col_expr(mark::Column::SourceCorrected, true.into())
                    .col_expr(mark::Column::CorrectionDate, Some(now).into())
                    .col_expr(mark::Column::UpdatedAt, now.into())
                    .filter(mark::Column::UserId.eq(account_id))
                    .filter(mark::Column::Id.eq(m.id))
                    .into_query(),
            )
            .await?;
        }
    }
    Ok(())
}

async fn sync_sends_with_overview_in_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    account_id: Uuid,
    overview: &FrontDoorOverview,
) -> Result<(), ApiError> {
    if overview.connections.is_empty() && overview.shares.is_empty() {
        return Ok(());
    }
    let conn_map: HashMap<(&str, &str), ConnectionStanding> = overview
        .connections
        .iter()
        .map(|c| {
            (
                (
                    c.patient.practice_id.as_str(),
                    c.patient.patient_id.as_str(),
                ),
                c.standing,
            )
        })
        .collect();
    let share_map: HashMap<&str, &ForMyDoctorShare> = overview
        .shares
        .iter()
        .map(|s| (s.share_key.as_str(), s))
        .collect();

    let sends = orm::models::<send::Model>(
        tx,
        &send::Entity::find()
            .filter(send::Column::UserId.eq(account_id))
            .into_query(),
    )
    .await?;

    for s in sends {
        if s.standing == StoredSendStanding::Received || s.connection_ended_at.is_some() {
            continue;
        }
        let ended_at = conn_map
            .get(&(s.practice_id.as_str(), s.patient_id.as_str()))
            .and_then(|standing| match standing {
                ConnectionStanding::Ended { ended_at, .. } => {
                    Utc.timestamp_opt(*ended_at as i64, 0).single()
                }
                ConnectionStanding::Connected { .. } => None,
            });

        let received_at = share_map.get(s.share_key.as_str()).and_then(|fd| {
            if let ForMyDoctorShareStatus::Received { received_at } = fd.standing {
                Utc.timestamp_opt(received_at as i64, 0).single()
            } else {
                None
            }
        });

        match (received_at, ended_at) {
            (Some(rec), Some(end)) if rec <= end => {
                orm::execute(
                    tx,
                    &send::Entity::update_many()
                        .col_expr(send::Column::Standing, StoredSendStanding::Received.into())
                        .col_expr(send::Column::ReceivedAt, Some(rec).into())
                        .col_expr(send::Column::ConnectionEndedAt, Some(end).into())
                        .filter(send::Column::UserId.eq(account_id))
                        .filter(send::Column::Id.eq(s.id))
                        .into_query(),
                )
                .await?;
            }
            (_, Some(end)) => {
                // Rule 7: if connection ends before pickup, stay Sent with the end date.
                orm::execute(
                    tx,
                    &send::Entity::update_many()
                        .col_expr(send::Column::ConnectionEndedAt, Some(end).into())
                        .filter(send::Column::UserId.eq(account_id))
                        .filter(send::Column::Id.eq(s.id))
                        .into_query(),
                )
                .await?;
            }
            (Some(rec), None) => {
                orm::execute(
                    tx,
                    &send::Entity::update_many()
                        .col_expr(send::Column::Standing, StoredSendStanding::Received.into())
                        .col_expr(send::Column::ReceivedAt, Some(rec).into())
                        .filter(send::Column::UserId.eq(account_id))
                        .filter(send::Column::Id.eq(s.id))
                        .into_query(),
                )
                .await?;
            }
            (None, None) => {}
        }
    }
    Ok(())
}

async fn load_state_in_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    account_id: Uuid,
    connections: &[AccountConnection],
) -> Result<ForMyDoctorStateView, ApiError> {
    let marks = orm::models::<mark::Model>(
        tx,
        &mark::Entity::find()
            .filter(mark::Column::UserId.eq(account_id))
            .order_by_asc(mark::Column::MarkedAt)
            .order_by_asc(mark::Column::Id)
            .into_query(),
    )
    .await?
    .into_iter()
    .map(mark_to_view)
    .collect();

    let notes = orm::models::<note::Model>(
        tx,
        &note::Entity::find()
            .filter(note::Column::UserId.eq(account_id))
            .order_by_desc(note::Column::UpdatedAt)
            .order_by_asc(note::Column::Id)
            .into_query(),
    )
    .await?
    .into_iter()
    .map(note_to_view)
    .collect();

    let sends = orm::models::<send::Model>(
        tx,
        &send::Entity::find()
            .filter(send::Column::UserId.eq(account_id))
            .order_by_desc(send::Column::SentAt)
            .order_by_asc(send::Column::Id)
            .into_query(),
    )
    .await?
    .into_iter()
    .map(send_to_view)
    .collect();

    let notices = orm::models::<notice::Model>(
        tx,
        &notice::Entity::find()
            .filter(notice::Column::UserId.eq(account_id))
            .filter(notice::Column::DismissedAt.is_null())
            .order_by_desc(notice::Column::CreatedAt)
            .into_query(),
    )
    .await?
    .into_iter()
    .map(|row| ForMyDoctorNoticeView {
        id: row.id,
        title: row.title,
        message: row.message,
        created_at: row.created_at.to_rfc3339(),
    })
    .collect();

    let mut latest_by_practice: BTreeMap<String, PracticeConnectionView> = BTreeMap::new();
    for conn in connections {
        let standing = match conn.standing {
            ConnectionStanding::Connected { connected_at } => {
                PracticeStanding::Connected { connected_at }
            }
            ConnectionStanding::Ended {
                connected_at,
                ended_at,
                reason,
            } => PracticeStanding::Ended {
                connected_at,
                ended_at,
                reason: reason.as_str().to_owned(),
            },
        };
        latest_by_practice.insert(
            format!("{}:{}", conn.patient.practice_id, conn.patient.patient_id),
            PracticeConnectionView {
                practice_id: conn.patient.practice_id.clone(),
                practice_name: conn.patient.practice_id.clone(),
                patient_id: conn.patient.patient_id.clone(),
                standing,
            },
        );
    }

    Ok(ForMyDoctorStateView {
        marks,
        notes,
        sends,
        notices,
        practices: latest_by_practice.into_values().collect(),
    })
}

fn mark_to_view(row: mark::Model) -> ForMyDoctorMarkView {
    ForMyDoctorMarkView {
        id: row.id,
        target_kind: row.target_kind,
        target_key: row.target_key,
        record_id: row.record_id,
        entity: row.entity,
        record_revision: row.record_revision,
        marked_revision: row.marked_revision,
        title: row.title,
        summary_text: row.summary_text,
        range_label: row.range_label,
        source_label: row.source_label,
        source_corrected: row.source_corrected,
        correction_date: row.correction_date.map(|dt| dt.to_rfc3339()),
        payload: row.payload,
        marked_at: row.marked_at.to_rfc3339(),
        updated_at: row.updated_at.to_rfc3339(),
    }
}

fn note_to_view(row: note::Model) -> ForMyDoctorNoteView {
    ForMyDoctorNoteView {
        id: row.id,
        body: row.body,
        linked_record_id: row.linked_record_id,
        linked_record_title: row.linked_record_title,
        linked_entity: row.linked_entity,
        version: row.version,
        created_at: row.created_at.to_rfc3339(),
        updated_at: row.updated_at.to_rfc3339(),
    }
}

fn send_to_view(row: send::Model) -> ForMyDoctorSendView {
    let records: Vec<SentRecordEntry> = row
        .snapshot
        .get("records")
        .cloned()
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    let notes: Vec<SentNoteEntry> = row
        .snapshot
        .get("notes")
        .cloned()
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    let standing = match row.standing {
        StoredSendStanding::Sent => SendStandingView::Sent,
        StoredSendStanding::Received => SendStandingView::Received {
            received_at: row.received_at.unwrap_or(row.sent_at).to_rfc3339(),
        },
    };
    ForMyDoctorSendView {
        id: row.id,
        share_key: row.share_key,
        practice_id: row.practice_id,
        practice_name: row.practice_name,
        patient_id: row.patient_id,
        title: row.title,
        summary_text: row.summary_text,
        records,
        notes,
        standing,
        sent_at: row.sent_at.to_rfc3339(),
        connection_ended_at: row.connection_ended_at.map(|dt| dt.to_rfc3339()),
    }
}

fn summarize_send_title(records: &[SentRecordEntry], notes: &[SentNoteEntry]) -> String {
    match (records.len(), notes.len()) {
        (1, 0) => records[0].title.clone(),
        (0, 1) => "Patient note".to_owned(),
        (r, 0) => format!("{r} records"),
        (0, n) => format!("{n} notes"),
        (r, n) => format!("{r} records and {n} notes"),
    }
}

fn render_send_summary(records: &[SentRecordEntry], notes: &[SentNoteEntry]) -> String {
    let mut parts = Vec::new();
    for r in records {
        let mut line = r.title.clone();
        if !r.summary_text.is_empty() {
            line.push_str(&format!(": {}", r.summary_text));
        }
        if let Some(range) = &r.range_label {
            line.push_str(&format!(" ({range})"));
        }
        if let Some(src) = &r.source_label {
            line.push_str(&format!(" — {src}"));
        }
        if let Some(rev) = r.version {
            line.push_str(&format!(" [v{rev}]"));
        }
        if let Some(corr) = &r.correction_date {
            line.push_str(&format!(" [updated {corr}]"));
        }
        parts.push(line);
    }
    for n in notes {
        if let Some(rec) = &n.linked_record_title {
            parts.push(format!("Note on {rec}: {}", n.body));
        } else {
            parts.push(format!("Note: {}", n.body));
        }
    }
    parts.join("\n")
}

/// Shared router state for `/api/for-my-doctor/*`.
#[derive(Clone)]
pub struct ForMyDoctorState {
    /// Authenticated identity/privacy/evidence store.
    pub durable: DurableState,
    /// `"For my doctor"` persistence and `front-door` delivery owner.
    pub store: ForMyDoctorStore,
}

/// Builds the `/api/for-my-doctor/*` Axum router.
pub fn router(state: ForMyDoctorState) -> Router {
    Router::new()
        .route("/api/for-my-doctor", get(read_route))
        .route(
            "/api/for-my-doctor/marks",
            put(mark_route).delete(unmark_route),
        )
        .route("/api/for-my-doctor/notes", post(create_note_route))
        .route(
            "/api/for-my-doctor/notes/{id}",
            put(update_note_route).delete(delete_note_route),
        )
        .route(
            "/api/for-my-doctor/notices/{id}/dismiss",
            post(dismiss_notice_route),
        )
        .route("/api/for-my-doctor/send", post(send_route))
        .route("/api/for-my-doctor/marks/{id}", delete(unmark_by_id_route))
        .with_state(state)
}

fn front_door_bearer(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(FRONT_DOOR_SESSION_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|s| !s.is_empty())
}

async fn read_route(
    State(s): State<ForMyDoctorState>,
    headers: HeaderMap,
) -> Result<Json<ForMyDoctorStateView>, ApiError> {
    let context = s.durable.authenticate(&headers).await?;
    let state = s
        .store
        .read_state(&context, front_door_bearer(&headers))
        .await?;
    Ok(Json(state))
}

async fn mark_route(
    State(s): State<ForMyDoctorState>,
    headers: HeaderMap,
    Json(body): Json<MarkForDoctorRequest>,
) -> Result<Json<ForMyDoctorMarkView>, ApiError> {
    let context = s.durable.authenticate(&headers).await?;
    Ok(Json(s.store.mark(&context, &body).await?))
}

async fn unmark_route(
    State(s): State<ForMyDoctorState>,
    headers: HeaderMap,
    Json(body): Json<UnmarkRequest>,
) -> Result<StatusCode, ApiError> {
    let context = s.durable.authenticate(&headers).await?;
    s.store.unmark(&context, &body.target_key).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn unmark_by_id_route(
    State(s): State<ForMyDoctorState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let context = s.durable.authenticate(&headers).await?;
    let mut tx = s.store.database.begin(context.account_id).await?;
    check_current(&mut tx, &context, AccessLevel::AcceptedTerms).await?;
    orm::execute(
        &mut tx,
        &mark::Entity::delete_many()
            .filter(mark::Column::UserId.eq(context.account_id))
            .filter(mark::Column::Id.eq(id))
            .into_query(),
    )
    .await?;
    tx.commit().await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn create_note_route(
    State(s): State<ForMyDoctorState>,
    headers: HeaderMap,
    Json(body): Json<CreateDoctorNoteRequest>,
) -> Result<Json<ForMyDoctorNoteView>, ApiError> {
    let context = s.durable.authenticate(&headers).await?;
    Ok(Json(s.store.create_note(&context, &body).await?))
}

async fn update_note_route(
    State(s): State<ForMyDoctorState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateDoctorNoteRequest>,
) -> Result<Json<ForMyDoctorNoteView>, NoteUpdateError> {
    let context = s.durable.authenticate(&headers).await?;
    Ok(Json(s.store.update_note(&context, id, &body).await?))
}

async fn delete_note_route(
    State(s): State<ForMyDoctorState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let context = s.durable.authenticate(&headers).await?;
    s.store.delete_note(&context, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn dismiss_notice_route(
    State(s): State<ForMyDoctorState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let context = s.durable.authenticate(&headers).await?;
    s.store.dismiss_notice(&context, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn send_route(
    State(s): State<ForMyDoctorState>,
    headers: HeaderMap,
    Json(body): Json<SendForMyDoctorPayload>,
) -> Result<Json<ForMyDoctorSendView>, ForMyDoctorSendError> {
    let context = s.durable.authenticate(&headers).await?;
    Ok(Json(
        s.store
            .send(&context, front_door_bearer(&headers), &body)
            .await?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_strips_lab_reference_ranges() {
        let cleaned =
            sanitize_summary("5.4 mmol/L\nReference range: 3.5-5.0 mmol/L\nSource: Quest");
        assert!(!cleaned.contains("3.5"));
        assert!(!cleaned.contains("Reference range"));
        assert!(cleaned.contains("5.4 mmol/L"));
    }
}
