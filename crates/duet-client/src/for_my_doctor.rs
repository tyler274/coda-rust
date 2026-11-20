//! Typed HTTP client and local unsent/pending-send state for `"For my doctor"`
//! (`COD-58`, `COD-59`, `COD-50`, `COD-51`).

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::DuetClient;

/// HTTP header forwarding the patient's active `front-door` bearer token.
pub const FRONT_DOOR_SESSION_HEADER: &str = "x-front-door-session";

/// Closed target categories that can be marked in `You` for `To send`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarkTargetKind {
    /// Whole record (`Medication`, `Lab result`, `Vitals`, `Condition`, `Note`).
    Record,
    /// Single value inside a record (e.g. Systolic or Diastolic reading).
    Value,
    /// Single time-series reading inside a trend view.
    Reading,
    /// Trend series snapshot capturing the currently viewed range.
    Trend,
}

/// Closed canonical entity families for marked or linked records.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordFamily {
    /// Health evidence record (`health_evidence`).
    HealthEvidence,
    /// Clinical note (`clinical_notes`).
    ClinicalNote,
}

/// Marked record, value, reading, or trend waiting in `To send`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForMyDoctorMark {
    /// Mark identifier.
    pub id: Uuid,
    /// Target kind (`record`, `value`, `reading`, `trend`).
    pub target_kind: MarkTargetKind,
    /// Deduplicating key.
    pub target_key: String,
    /// Linked canonical record id, if any.
    pub record_id: Option<Uuid>,
    /// Canonical entity family, if linked.
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
    #[serde(default)]
    pub payload: Value,
    /// Timestamp marked.
    pub marked_at: String,
    /// Timestamp last updated.
    pub updated_at: String,
}

/// Patient-written free-text note waiting in `To send`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForMyDoctorNote {
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
    /// Monotonic edit version for `409 version_conflict`.
    pub version: i64,
    /// Creation timestamp.
    pub created_at: String,
    /// Last updated timestamp.
    pub updated_at: String,
}

/// Standing of one practice connection attached to the account.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum PracticeStanding {
    /// Actively sharing.
    Connected {
        /// Unix second connected.
        connected_at: u64,
    },
    /// Connection ended; historical sends keep `ended_at`.
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
pub struct PracticeConnection {
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

/// Frozen record entry included in a committed send.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SentRecordEntry {
    /// Canonical record identifier, if backed by canonical evidence.
    pub record_id: Option<Uuid>,
    /// Title at send time.
    pub title: String,
    /// Summary text without lab printed reference ranges.
    pub summary_text: String,
    /// Plain source label.
    pub source_label: Option<String>,
    /// Frozen revision number at send time.
    pub version: Option<i32>,
    /// Date of source correction if corrected before send.
    pub correction_date: Option<String>,
    /// Viewed trend range label, if from a marked trend.
    pub range_label: Option<String>,
}

/// Frozen note entry included in a committed send.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SentNoteEntry {
    /// Original note id.
    pub id: Uuid,
    /// Frozen note words.
    pub body: String,
    /// Linked record title if written from a record.
    pub linked_record_title: Option<String>,
}

/// Delivery standing shown on a send row in `Shared`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum SendStanding {
    /// Saved by Duet and awaiting practice agent pickup.
    Sent,
    /// Picked up by the practice's agents (`"Received by the care team"`).
    Received {
        /// Timestamp picked up.
        received_at: String,
    },
}

/// Immutable send listed in `Shared`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForMyDoctorSend {
    /// Local send id.
    pub id: Uuid,
    /// Idempotency key.
    pub share_key: String,
    /// Practice identifier.
    pub practice_id: String,
    /// Practice display name.
    pub practice_name: String,
    /// Patient roster identifier.
    pub patient_id: String,
    /// Headline shown in the list.
    pub title: String,
    /// Plain summary sent to the practice.
    pub summary_text: String,
    /// Frozen records carried in this send.
    #[serde(default)]
    pub records: Vec<SentRecordEntry>,
    /// Frozen notes carried in this send.
    #[serde(default)]
    pub notes: Vec<SentNoteEntry>,
    /// `Sent` or `Received`.
    #[serde(flatten)]
    pub standing: SendStanding,
    /// Timestamp sent.
    pub sent_at: String,
    /// Date connection ended if ended before pickup.
    pub connection_ended_at: Option<String>,
}

/// Notice surfaced in `You` when a clinical source removes a marked record (`COD-50`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForMyDoctorNotice {
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ForMyDoctorState {
    /// Active marked items in `To send`.
    #[serde(default)]
    pub marks: Vec<ForMyDoctorMark>,
    /// Unsent free-text notes in `To send`.
    #[serde(default)]
    pub notes: Vec<ForMyDoctorNote>,
    /// Immutable flat list of sends.
    #[serde(default)]
    pub sends: Vec<ForMyDoctorSend>,
    /// Active notices in `You` (source removed a marked item).
    #[serde(default)]
    pub notices: Vec<ForMyDoctorNotice>,
    /// Connected and ended practices known to `front-door`.
    #[serde(default)]
    pub practices: Vec<PracticeConnection>,
}

/// Request to mark a record, value, reading, or trend.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    /// Optional structured payload.
    #[serde(default)]
    pub payload: Value,
}

/// Request to create an unsent note.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

/// Request to update an existing note.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateDoctorNoteRequest {
    /// Updated free-text note body.
    pub body: String,
    /// Optimistic concurrency version expected by the client.
    pub expected_version: i64,
}

/// Single open record passed directly from `You` -> `Choosing` -> `Send`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SendForMyDoctorPayload {
    /// Client-generated idempotency key reused on `Try again`.
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

/// Local draft held while a free-text note failed to save (`"Not saved yet"`) or hit `409 version_conflict`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetainedDoctorNoteDraft {
    /// Existing server note id (`None` when editing a new unsaved note, `Some` with client id).
    pub note_id: Uuid,
    /// True if `note_id` has not yet been confirmed on the server.
    pub is_new: bool,
    /// Base server version when last loaded (`None` for a new note).
    pub expected_version: Option<i64>,
    /// Patient's unsaved text preserved verbatim (`Rule 9`).
    pub unsaved_body: String,
    /// Optional linked record id.
    pub linked_record_id: Option<Uuid>,
    /// Optional linked record title.
    pub linked_record_title: Option<String>,
    /// Optional linked entity.
    pub linked_entity: Option<RecordFamily>,
    /// Latest server note surfaced alongside `unsaved_body` on `409 version_conflict`.
    pub conflict_latest: Option<ForMyDoctorNote>,
}

/// Local send attempt held across `"Couldn't confirm the send"` -> `"Try again"` or `COD-51`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingDoctorSend {
    /// Exact frozen payload + `share_key` to replay on `"Try again"`.
    pub payload: SendForMyDoctorPayload,
    /// True once the server reported `connection_ended` after an uncertain send (`COD-51`).
    pub ended_before_confirm: bool,
    /// Plain-language end reason if ended.
    pub ended_reason: Option<String>,
}

/// Typed client error distinguishing `409 version_conflict`, `connection_ended`, and `Uncertain`.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ForMyDoctorClientError {
    /// Account/session authentication refused.
    #[error("Sign in again to access For my doctor.")]
    Unauthorized,
    /// Optimistic concurrency conflict returning the newer saved server note (`Rule 9`).
    #[error("This note was edited on another device.")]
    VersionConflict(Box<ForMyDoctorNote>),
    /// Practice connection ended during `Sending` or while `Couldn't confirm the send` (`COD-51`).
    #[error("The practice connection ended.")]
    ConnectionEnded {
        /// Plain-language reason.
        reason: String,
        /// Optional Unix second the connection ended.
        ended_at: Option<u64>,
    },
    /// Validation or explicit conflict error.
    #[error("{0}")]
    Invalid(String),
    /// Network or 5xx uncertainty: keep unsaved words / offer `"Try again"` with exact same `share_key`.
    #[error("Couldn't confirm with Duet right now.")]
    Uncertain(String),
}

#[derive(Deserialize)]
struct ErrorEnvelope {
    code: Option<String>,
    detail: Option<String>,
    ended_at: Option<u64>,
    latest: Option<ForMyDoctorNote>,
}

impl DuetClient {
    fn apply_front_door_header(
        req: reqwest::RequestBuilder,
        front_door_session: Option<&str>,
    ) -> reqwest::RequestBuilder {
        match front_door_session.map(str::trim).filter(|s| !s.is_empty()) {
            Some(token) => req.header(FRONT_DOOR_SESSION_HEADER, token),
            None => req,
        }
    }

    fn bearer(&self) -> &str {
        self.configuration
            .bearer_access_token
            .as_deref()
            .unwrap_or_default()
    }

    async fn parse_fmd_json<T: serde::de::DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<T, ForMyDoctorClientError> {
        let response = request
            .send()
            .await
            .map_err(|err| ForMyDoctorClientError::Uncertain(err.to_string()))?;
        let status = response.status();
        let bytes = response
            .bytes()
            .await
            .map_err(|err| ForMyDoctorClientError::Uncertain(err.to_string()))?;
        if status.is_success() {
            return serde_json::from_slice(&bytes)
                .map_err(|err| ForMyDoctorClientError::Uncertain(err.to_string()));
        }
        Err(classify_fmd_error(status, &bytes))
    }

    async fn parse_fmd_empty(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<(), ForMyDoctorClientError> {
        let response = request
            .send()
            .await
            .map_err(|err| ForMyDoctorClientError::Uncertain(err.to_string()))?;
        let status = response.status();
        let bytes = response
            .bytes()
            .await
            .map_err(|err| ForMyDoctorClientError::Uncertain(err.to_string()))?;
        if status.is_success() {
            return Ok(());
        }
        Err(classify_fmd_error(status, &bytes))
    }

    /// Reads the complete `For my doctor` workspace state (`To send`, `Sent`, `You` notices, practices).
    pub async fn for_my_doctor_state(
        &self,
        front_door_session: Option<&str>,
    ) -> Result<ForMyDoctorState, ForMyDoctorClientError> {
        let req = self
            .configuration
            .client
            .get(format!(
                "{}/api/for-my-doctor",
                self.configuration.base_path
            ))
            .bearer_auth(self.bearer())
            .header(reqwest::header::CACHE_CONTROL, "no-store");
        self.parse_fmd_json(Self::apply_front_door_header(req, front_door_session))
            .await
    }

    /// Marks one record, value, reading, or trend for `To send`.
    pub async fn mark_for_doctor(
        &self,
        request: &MarkForDoctorRequest,
    ) -> Result<ForMyDoctorMark, ForMyDoctorClientError> {
        let req = self
            .configuration
            .client
            .put(format!(
                "{}/api/for-my-doctor/marks",
                self.configuration.base_path
            ))
            .bearer_auth(self.bearer())
            .header(reqwest::header::CACHE_CONTROL, "no-store")
            .json(request);
        self.parse_fmd_json(req).await
    }

    /// Removes one mark from `To send` by `target_key`.
    pub async fn unmark_for_doctor(&self, target_key: &str) -> Result<(), ForMyDoctorClientError> {
        let req = self
            .configuration
            .client
            .delete(format!(
                "{}/api/for-my-doctor/marks",
                self.configuration.base_path
            ))
            .bearer_auth(self.bearer())
            .header(reqwest::header::CACHE_CONTROL, "no-store")
            .json(&serde_json::json!({ "target_key": target_key }));
        self.parse_fmd_empty(req).await
    }

    /// Removes one mark from `To send` by mark `id`.
    pub async fn unmark_for_doctor_by_id(&self, id: Uuid) -> Result<(), ForMyDoctorClientError> {
        let req = self
            .configuration
            .client
            .delete(format!(
                "{}/api/for-my-doctor/marks/{id}",
                self.configuration.base_path
            ))
            .bearer_auth(self.bearer())
            .header(reqwest::header::CACHE_CONTROL, "no-store");
        self.parse_fmd_empty(req).await
    }

    /// Creates a new free-text note in `To send` (idempotent on `request.id`).
    pub async fn create_for_my_doctor_note(
        &self,
        request: &CreateDoctorNoteRequest,
    ) -> Result<ForMyDoctorNote, ForMyDoctorClientError> {
        let req = self
            .configuration
            .client
            .post(format!(
                "{}/api/for-my-doctor/notes",
                self.configuration.base_path
            ))
            .bearer_auth(self.bearer())
            .header(reqwest::header::CACHE_CONTROL, "no-store")
            .json(request);
        self.parse_fmd_json(req).await
    }

    /// Updates an existing note in `To send` with optimistic version checking (`409 version_conflict`).
    pub async fn update_for_my_doctor_note(
        &self,
        id: Uuid,
        request: &UpdateDoctorNoteRequest,
    ) -> Result<ForMyDoctorNote, ForMyDoctorClientError> {
        let req = self
            .configuration
            .client
            .put(format!(
                "{}/api/for-my-doctor/notes/{id}",
                self.configuration.base_path
            ))
            .bearer_auth(self.bearer())
            .header(reqwest::header::CACHE_CONTROL, "no-store")
            .json(request);
        self.parse_fmd_json(req).await
    }

    /// Deletes an unsent note from `To send`.
    pub async fn delete_for_my_doctor_note(&self, id: Uuid) -> Result<(), ForMyDoctorClientError> {
        let req = self
            .configuration
            .client
            .delete(format!(
                "{}/api/for-my-doctor/notes/{id}",
                self.configuration.base_path
            ))
            .bearer_auth(self.bearer())
            .header(reqwest::header::CACHE_CONTROL, "no-store");
        self.parse_fmd_empty(req).await
    }

    /// Dismisses a source-removal notice (`COD-50`) shown in `You`.
    pub async fn dismiss_for_my_doctor_notice(
        &self,
        id: Uuid,
    ) -> Result<(), ForMyDoctorClientError> {
        let req = self
            .configuration
            .client
            .post(format!(
                "{}/api/for-my-doctor/notices/{id}/dismiss",
                self.configuration.base_path
            ))
            .bearer_auth(self.bearer())
            .header(reqwest::header::CACHE_CONTROL, "no-store");
        self.parse_fmd_empty(req).await
    }

    /// Sends selected `To send` items (or a single open record) to one connected practice.
    pub async fn send_for_my_doctor(
        &self,
        payload: &SendForMyDoctorPayload,
        front_door_session: Option<&str>,
    ) -> Result<ForMyDoctorSend, ForMyDoctorClientError> {
        let req = self
            .configuration
            .client
            .post(format!(
                "{}/api/for-my-doctor/send",
                self.configuration.base_path
            ))
            .bearer_auth(self.bearer())
            .header(reqwest::header::CACHE_CONTROL, "no-store")
            .json(payload);
        self.parse_fmd_json(Self::apply_front_door_header(req, front_door_session))
            .await
    }
}

fn classify_fmd_error(status: reqwest::StatusCode, bytes: &[u8]) -> ForMyDoctorClientError {
    if status == reqwest::StatusCode::UNAUTHORIZED {
        return ForMyDoctorClientError::Unauthorized;
    }
    let env = serde_json::from_slice::<ErrorEnvelope>(bytes).ok();
    if status == reqwest::StatusCode::CONFLICT {
        if let Some(env) = env {
            if env.code.as_deref() == Some("version_conflict")
                && let Some(latest) = env.latest
            {
                return ForMyDoctorClientError::VersionConflict(Box::new(latest));
            }
            if env.code.as_deref() == Some("connection_ended") {
                return ForMyDoctorClientError::ConnectionEnded {
                    reason: env
                        .detail
                        .unwrap_or_else(|| "This practice connection ended.".to_owned()),
                    ended_at: env.ended_at,
                };
            }
            if let Some(detail) = env.detail {
                return ForMyDoctorClientError::Invalid(detail);
            }
        }
        return ForMyDoctorClientError::Invalid("That request conflicted with newer state.".into());
    }
    if status == reqwest::StatusCode::UNPROCESSABLE_ENTITY
        || status == reqwest::StatusCode::BAD_REQUEST
    {
        let detail = env
            .and_then(|e| e.detail)
            .unwrap_or_else(|| "Review your selection before sending.".to_owned());
        return ForMyDoctorClientError::Invalid(detail);
    }
    let msg = env
        .and_then(|e| e.detail)
        .unwrap_or_else(|| format!("HTTP {}", status.as_u16()));
    ForMyDoctorClientError::Uncertain(msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_version_conflict_with_latest_saved_note() {
        let note = ForMyDoctorNote {
            id: Uuid::new_v4(),
            body: "Saved on iPad".to_owned(),
            linked_record_id: None,
            linked_record_title: None,
            linked_entity: None,
            version: 2,
            created_at: "2026-11-20T00:00:00Z".to_owned(),
            updated_at: "2026-11-20T00:05:00Z".to_owned(),
        };
        let bytes = serde_json::to_vec(&serde_json::json!({
            "code": "version_conflict",
            "detail": "Note was edited on another device.",
            "latest": note,
        }))
        .unwrap();
        let err = classify_fmd_error(reqwest::StatusCode::CONFLICT, &bytes);
        assert_eq!(err, ForMyDoctorClientError::VersionConflict(Box::new(note)));
    }

    #[test]
    fn classifies_connection_ended_and_uncertain_states() {
        let ended = serde_json::to_vec(&serde_json::json!({
            "code": "connection_ended",
            "detail": "Patient left the practice.",
            "ended_at": 1763300000u64,
        }))
        .unwrap();
        assert_eq!(
            classify_fmd_error(reqwest::StatusCode::CONFLICT, &ended),
            ForMyDoctorClientError::ConnectionEnded {
                reason: "Patient left the practice.".to_owned(),
                ended_at: Some(1_763_300_000),
            }
        );
        assert!(matches!(
            classify_fmd_error(reqwest::StatusCode::SERVICE_UNAVAILABLE, b""),
            ForMyDoctorClientError::Uncertain(_)
        ));
    }
}
