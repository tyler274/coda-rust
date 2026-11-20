//! Email authentication, non-expiring sessions, Terms, and account settings transport.
use crate::DuetClient;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Verified internal account context returned by `/api/auth/account/context`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountContext {
    /// Internal owner; never inferred from email.
    pub account_id: Uuid,
    /// Restored/deleted/recreated owner fence.
    pub incarnation: Uuid,
    /// Current permission fence.
    pub permission_revision: i64,
    /// Session or authentication creation time.
    pub authenticated_at: String,
}

impl AccountContext {
    /// Protected cache namespace includes account incarnation.
    pub fn owner(&self) -> String {
        format!("{}:{}", self.account_id, self.incarnation)
    }
}

/// Caller proof submitted for email sign-in/sign-up or verified email change.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum ChallengeCredential {
    /// Single-use URL token from the emailed link.
    LinkToken(String),
    /// Six-digit fallback code from the email.
    ShortCode(String),
}

impl std::fmt::Debug for ChallengeCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LinkToken(_) => f.write_str("ChallengeCredential::LinkToken(<redacted>)"),
            Self::ShortCode(_) => f.write_str("ChallengeCredential::ShortCode(<redacted>)"),
        }
    }
}

/// User-chosen Duet appearance preference.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Appearance {
    /// Follow the device's light or dark setting.
    #[default]
    System,
    /// Always use light surfaces.
    Light,
    /// Always use dark surfaces.
    Dark,
}

impl Appearance {
    /// Canonical wire/storage name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    /// Parse stored preference string, defaulting to `None` when unknown.
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim() {
            "system" => Some(Self::System),
            "light" => Some(Self::Light),
            "dark" => Some(Self::Dark),
            _ => None,
        }
    }
}

/// Signed-in account email and appearance preference.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountProfile {
    /// Verified email address on file for this account.
    pub email: Option<String>,
    /// Selected appearance preference.
    pub appearance: Appearance,
}

/// Exported account profile and consent standing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountProfileExport {
    /// Internal account ID.
    pub account_id: Uuid,
    /// Verified email address.
    pub email: Option<String>,
    /// Chosen display name.
    pub display_name: Option<String>,
    /// Chosen appearance (`system`, `light`, or `dark`).
    pub appearance: String,
    /// Sharing label.
    pub sharing_label: String,
    /// Accepted Terms version, if any.
    pub terms_version: Option<String>,
    /// When Terms were accepted.
    pub terms_accepted_at: Option<String>,
    /// Current AI permission flag.
    pub ai_enabled: bool,
    /// Accepted AI consent version, if any.
    pub ai_consent_version: Option<String>,
    /// AI consent decision, if recorded.
    pub ai_consent_decision: Option<String>,
    /// When AI consent was recorded.
    pub ai_consent_at: Option<String>,
    /// Apple Health source generation.
    pub apple_health_generation: i32,
    /// Apple Health disconnect timestamp, if disconnected.
    pub apple_health_disconnected_at: Option<String>,
    /// Health Records source generation.
    pub health_records_generation: i32,
    /// Health Records disconnect timestamp, if disconnected.
    pub health_records_disconnected_at: Option<String>,
    /// Health Connect source generation.
    pub health_connect_generation: i32,
    /// Health Connect disconnect timestamp, if disconnected.
    pub health_connect_disconnected_at: Option<String>,
    /// Account creation timestamp.
    pub created_at: String,
}

/// Exported immutable canonical revision.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedEvidenceRevision {
    /// Monotonic account revision number.
    pub revision: i64,
    /// Date Duet retained this revision.
    pub retained_at: String,
    /// Original literal payload for this revision.
    pub payload: serde_json::Value,
}

/// Exported health evidence record with provenance, dates, and retained revisions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedEvidenceRecord {
    /// Canonical record ID.
    pub id: Uuid,
    /// Concept identifier.
    pub concept_id: String,
    /// Source kind.
    pub source_kind: String,
    /// Connection identifier, when imported from a source connection.
    pub source_connection_id: Option<String>,
    /// Source-native record ID.
    pub source_record_id: Option<String>,
    /// Measurement date declared by the source, if known.
    pub measured_at: Option<String>,
    /// Date Duet received the latest revision.
    pub received_at: String,
    /// Date the record was first saved.
    pub created_at: String,
    /// Date the record was last updated.
    pub updated_at: String,
    /// Latest literal JSON payload.
    pub payload: serde_json::Value,
    /// Complete revision history retained for this record.
    pub revisions: Vec<ExportedEvidenceRevision>,
}

/// Exported clinical note with source and dates.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedClinicalNote {
    /// Canonical note ID.
    pub id: Uuid,
    /// Human-readable source label.
    pub source_label: String,
    /// Source-native note identifier.
    pub source_id: Option<String>,
    /// Note title.
    pub title: String,
    /// Note preview, if any.
    pub preview: Option<String>,
    /// Full note body, if any.
    pub body: Option<String>,
    /// Date declared by the source, if known.
    pub recorded_at: Option<String>,
    /// Date Duet stored the note.
    pub created_at: String,
    /// Original source payload, if retained.
    pub source_payload: Option<serde_json::Value>,
    /// Complete revision history retained for this note.
    pub revisions: Vec<ExportedEvidenceRevision>,
}

/// Exported device snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedDeviceSnapshot {
    /// Snapshot ID.
    pub id: Uuid,
    /// Date the snapshot was generated on the device.
    pub generated_at: String,
    /// Date Duet received the snapshot.
    pub received_at: String,
    /// Snapshot JSON payload.
    pub payload: serde_json::Value,
}

/// Exported `For my doctor` workspace and saved items.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedDoctorPage {
    /// Saved patient note.
    pub note: Option<String>,
    /// Current note/page generation.
    pub note_generation: i64,
    /// Saved page structure.
    pub page: serde_json::Value,
    /// Date the page was last updated.
    pub page_updated_at: String,
    /// Saved items on `For my doctor`.
    pub items: Vec<ExportedDoctorItem>,
}

/// One exported item from `For my doctor`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedDoctorItem {
    /// Item ID.
    pub id: Uuid,
    /// Item kind.
    pub kind: String,
    /// Source identifier.
    pub source_id: String,
    /// Display title.
    pub title: String,
    /// Detail text.
    pub detail: String,
    /// Linked evidence entries.
    pub evidence: serde_json::Value,
    /// Range start, if any.
    pub range_start: Option<String>,
    /// Range end, if any.
    pub range_end: Option<String>,
    /// Date saved onto the page.
    pub saved_at: String,
    /// Order index.
    pub position: i32,
    /// Rendered entry structure, if saved.
    pub page_entry: Option<serde_json::Value>,
    /// Whether the patient confirmed this medication.
    pub medication_confirmed: bool,
}

/// Exported connected portal organization metadata (zero OAuth secrets).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportedPortalConnection {
    /// Connection ID.
    pub id: Uuid,
    /// Portal vendor.
    pub vendor: String,
    /// Organization display name.
    pub organization_name: String,
    /// Brand name.
    pub brand: String,
    /// Connection state.
    pub state: String,
    /// Last successful sync date.
    pub last_synced_at: Option<String>,
    /// First failing date if broken.
    pub failing_since: Option<String>,
    /// Non-secret failure reason.
    pub failure_reason: Option<String>,
    /// Disconnected date, if disconnected.
    pub disconnected_at: Option<String>,
    /// Connection created date.
    pub created_at: String,
    /// Connection updated date.
    pub updated_at: String,
}

/// Exported conversation turn.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedConversationTurn {
    /// Turn ID.
    pub id: Uuid,
    /// Patient message.
    pub input: String,
    /// Turn completion state.
    pub state: String,
    /// Assistant reply payload, if completed.
    pub reply: Option<serde_json::Value>,
    /// Turn creation date.
    pub created_at: String,
}

/// Exported assistant conversation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedConversation {
    /// Conversation ID.
    pub id: Uuid,
    /// Conversation created date.
    pub created_at: String,
    /// Turns belonging to this conversation in chronological order.
    pub turns: Vec<ExportedConversationTurn>,
}

/// Exported Duet private memory state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedPrivateMemory {
    /// Current memory revision.
    pub revision: i64,
    /// Structured memory content.
    pub content: serde_json::Value,
}

/// Exported privacy restriction receipt metadata.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportedPrivacyReceipt {
    /// Idempotent event identity.
    pub event_id: Uuid,
    /// Privacy operation name.
    pub operation: String,
    /// Target resource or account scope ID.
    pub scope_id: Uuid,
    /// Date committed.
    pub occurred_at: String,
    /// Date confirmed in independent privacy journal.
    pub journaled_at: Option<String>,
}

/// Complete portable personal data export returned by `/api/auth/account/export`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountExport {
    /// Export schema version.
    pub schema_version: u32,
    /// UTC timestamp when Duet produced this export.
    pub exported_at: String,
    /// Account identity profile, preferences, and consent standing.
    pub account: AccountProfileExport,
    /// Imported and user-entered canonical health records and retained source revisions.
    pub health_evidence: Vec<ExportedEvidenceRecord>,
    /// Clinical notes with source label, date, and retained revisions.
    pub clinical_notes: Vec<ExportedClinicalNote>,
    /// Raw device snapshots retained on the account.
    pub device_snapshots: Vec<ExportedDeviceSnapshot>,
    /// Current saved `For my doctor` page and selected items.
    pub doctor_page: ExportedDoctorPage,
    /// Connected or disconnected portal accounts (metadata only, zero OAuth secrets).
    pub portal_connections: Vec<ExportedPortalConnection>,
    /// Assistant conversations and turn records.
    pub conversations: Vec<ExportedConversation>,
    /// Duet's current understanding/memory of the person.
    pub private_memory: Option<ExportedPrivateMemory>,
    /// Committed privacy restriction receipts.
    pub privacy_receipts: Vec<ExportedPrivacyReceipt>,
}

impl AccountExport {
    /// Human-readable one-line summary of exported record categories.
    pub fn summary_text(&self) -> String {
        format!(
            "Export ready: {} health record(s), {} note(s), {} portal(s), {} doctor item(s), {} conversation(s), {} privacy receipt(s).",
            self.health_evidence.len(),
            self.clinical_notes.len(),
            self.portal_connections.len(),
            self.doctor_page.items.len(),
            self.conversations.len(),
            self.privacy_receipts.len(),
        )
    }

    /// Encode this export as pretty-printed UTF-8 JSON for saving or download.
    pub fn to_pretty_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Non-expiring opaque session returned after email link or code verification.
#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct EmailSignInResult {
    /// Opaque `ds1.` session token.
    pub bearer: String,
    /// Internal Duet account UUID.
    pub account_id: Uuid,
    /// Active account incarnation UUID.
    pub incarnation: Uuid,
    /// Verified email address.
    pub email: String,
    /// Selected appearance preference.
    pub appearance: Appearance,
}

impl std::fmt::Debug for EmailSignInResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EmailSignInResult")
            .field("bearer", &"<redacted>")
            .field("account_id", &self.account_id)
            .field("incarnation", &self.incarnation)
            .field("email", &self.email)
            .field("appearance", &self.appearance)
            .finish()
    }
}

impl EmailSignInResult {
    /// Cache/retention owner namespace (`<account_id>:<incarnation>`).
    pub fn owner(&self) -> String {
        format!("{}:{}", self.account_id, self.incarnation)
    }
}

/// Closed privacy operations callable from Settings without waiting on Terms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrivacyRestrictionKind {
    /// Disconnect Apple Health / device snapshots.
    DeviceWithdraw,
    /// Disconnect clinical Health Records imports.
    RecordsWithdraw,
    /// Revoke all sessions on every device.
    SessionsRevoke,
    /// Permanently delete this account and all associated state.
    AccountDelete,
}

/// Errors never expose raw credential-bearing URLs or raw server payloads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdentityError {
    /// No valid credential; do not infer account deletion.
    Unauthorized,
    /// The server refused this session because it was revoked (another device or sign-out everywhere).
    Ended,
    /// The server confirmed this account is absent (already deleted).
    AccountAbsent,
    /// A single-use email link or code was invalid, expired, or already used.
    InvalidChallenge(String),
    /// Current Duet Terms must be accepted before accessing `You` or `Shared`.
    TermsRequired,
    /// App must be updated before reviewing newer Terms.
    AgreementChanged,
    /// Request conflicted with existing server state (for example duplicate email).
    Conflict(String),
    /// Caller supplied invalid input (for example malformed email).
    InvalidInput(String),
    /// Network or service outage; preserve saved local state.
    Unavailable,
    /// Unreadable HTTP response with redacted transport cause.
    Response(crate::refusal::TransportCause),
}

impl IdentityError {
    /// Whether the server ended either this session (`Ended`) or the account itself (`AccountAbsent`).
    pub fn is_ended(&self) -> bool {
        matches!(self, Self::Ended | Self::AccountAbsent)
    }
}

impl std::fmt::Display for IdentityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unauthorized => f.write_str("No valid session. Sign in again."),
            Self::Ended => f.write_str("This session has ended."),
            Self::AccountAbsent => f.write_str("This account was deleted."),
            Self::InvalidChallenge(message) => f.write_str(message),
            Self::TermsRequired => f.write_str("Accept the current Duet Terms to continue."),
            Self::AgreementChanged => f.write_str("Update Duet to review the current terms."),
            Self::Conflict(message) | Self::InvalidInput(message) => f.write_str(message),
            Self::Unavailable | Self::Response(_) => {
                f.write_str("Duet couldn’t reach the server right now. Saved information on this phone stays available.")
            }
        }
    }
}

impl std::error::Error for IdentityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Response(cause) => Some(cause),
            _ => None,
        }
    }
}

impl DuetClient {
    fn authed(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let mut req = self
            .configuration
            .client
            .request(method, format!("{}{path}", self.configuration.base_path));
        if let Some(token) = &self.configuration.bearer_access_token
            && !token.is_empty()
        {
            req = req.bearer_auth(token);
        }
        req
    }

    /// Start the unified email sign-up / sign-in challenge flow.
    pub async fn begin_email_sign_in(
        &self,
        email: &str,
        invite: Option<&str>,
    ) -> Result<String, IdentityError> {
        #[derive(Deserialize)]
        struct Started {
            message: String,
        }
        let response = self
            .configuration
            .client
            .post(format!(
                "{}/api/auth/email/start",
                self.configuration.base_path
            ))
            .json(&serde_json::json!({ "email": email, "invite": invite }))
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        let started: Started = self.decode_checked(response, false).await?;
        Ok(started.message)
    }

    /// Complete email sign-up / sign-in using a single-use link token or 6-digit fallback code.
    pub async fn complete_email_sign_in(
        &self,
        email: Option<&str>,
        credential: &ChallengeCredential,
    ) -> Result<EmailSignInResult, IdentityError> {
        let response = self
            .configuration
            .client
            .post(format!(
                "{}/api/auth/email/verify",
                self.configuration.base_path
            ))
            .json(&serde_json::json!({ "email": email, "credential": credential }))
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        self.decode_checked(response, false).await
    }

    /// Sign out ONLY the current device session without revoking other signed-in devices.
    pub async fn sign_out_device(&self) -> Result<(), IdentityError> {
        let response = self
            .authed(reqwest::Method::POST, "/api/auth/session/sign-out")
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        let _: serde_json::Value = self.decode_checked(response, true).await?;
        Ok(())
    }

    /// Ensure an external-provider account exists (`PUT /api/auth/account`).
    pub async fn ensure_account(&self) -> Result<duet_wire::models::AccountState, IdentityError> {
        let response = self
            .authed(reqwest::Method::PUT, "/api/auth/account")
            .json(&duet_wire::models::AccountIdentityRequest::new())
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        self.decode_checked(response, true).await
    }

    /// Read current account state (`GET /api/auth/account`), allowed while Terms wait.
    pub async fn account_state(&self) -> Result<duet_wire::models::AccountState, IdentityError> {
        let response = self
            .authed(reqwest::Method::GET, "/api/auth/account")
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        self.decode_checked(response, true).await
    }

    /// Update or clear display name (`PUT /api/auth/account`), allowed while Terms wait.
    pub async fn update_display_name(
        &self,
        display_name: Option<&str>,
    ) -> Result<duet_wire::models::AccountState, IdentityError> {
        let mut body = duet_wire::models::AccountIdentityRequest::new();
        body.display_name = Some(display_name.map(str::to_owned));
        let response = self
            .authed(reqwest::Method::PUT, "/api/auth/account")
            .json(&body)
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        self.decode_checked(response, true).await
    }

    /// Record explicit Terms acceptance for `version`.
    pub async fn accept_terms(
        &self,
        version: &str,
    ) -> Result<duet_wire::models::AccountState, IdentityError> {
        let response = self
            .authed(reqwest::Method::PUT, "/api/auth/account/terms")
            .json(&duet_wire::models::TermsAcceptanceRequest::new(
                version.to_owned(),
            ))
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        self.decode_checked(response, true).await
    }

    /// Read verified account context before rendering private routes.
    pub async fn account_context(&self) -> Result<AccountContext, IdentityError> {
        let response = self
            .authed(reqwest::Method::GET, "/api/auth/account/context")
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        self.decode_checked(response, true).await
    }

    /// Read email and appearance settings, accessible while Terms wait.
    pub async fn account_profile(&self) -> Result<AccountProfile, IdentityError> {
        let response = self
            .authed(reqwest::Method::GET, "/api/auth/account/profile")
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        self.decode_checked(response, true).await
    }

    /// Save appearance preference (`system`, `light`, or `dark`).
    pub async fn set_appearance(
        &self,
        appearance: Appearance,
    ) -> Result<AccountProfile, IdentityError> {
        let response = self
            .authed(reqwest::Method::PUT, "/api/auth/account/appearance")
            .json(&serde_json::json!({ "appearance": appearance }))
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        self.decode_checked(response, true).await
    }

    /// Send verification link + code to confirm a new verified email address.
    pub async fn begin_email_change(&self, new_email: &str) -> Result<String, IdentityError> {
        #[derive(Deserialize)]
        struct Started {
            message: String,
        }
        let response = self
            .authed(reqwest::Method::POST, "/api/auth/email-change/start")
            .json(&serde_json::json!({ "new_email": new_email }))
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        let started: Started = self.decode_checked(response, true).await?;
        Ok(started.message)
    }

    /// Confirm verified email change with single-use link token or 6-digit fallback code.
    pub async fn confirm_email_change(
        &self,
        new_email: Option<&str>,
        credential: &ChallengeCredential,
    ) -> Result<AccountProfile, IdentityError> {
        let response = self
            .authed(reqwest::Method::POST, "/api/auth/email-change/confirm")
            .json(&serde_json::json!({ "new_email": new_email, "credential": credential }))
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        self.decode_checked(response, true).await
    }

    /// Commit a Rights-never-wait privacy restriction (source disconnect, sign-out everywhere, or account delete).
    pub async fn commit_privacy_restriction(
        &self,
        event_id: Uuid,
        operation: PrivacyRestrictionKind,
        scope_id: Option<Uuid>,
        scope_version: i64,
    ) -> Result<serde_json::Value, IdentityError> {
        let response = self
            .authed(reqwest::Method::POST, "/api/auth/privacy-restrictions")
            .json(&serde_json::json!({
                "event_id": event_id,
                "operation": operation,
                "scope_id": scope_id,
                "scope_version": scope_version,
            }))
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        self.decode_checked(response, true).await
    }

    /// Rights-never-wait full personal data export. Accessible before Terms acceptance or refresh.
    pub async fn export_account_data(&self) -> Result<AccountExport, IdentityError> {
        let response = self
            .authed(reqwest::Method::GET, "/api/auth/account/export")
            .send()
            .await
            .map_err(|_| IdentityError::Unavailable)?;
        self.decode_checked(response, true).await
    }

    async fn decode_checked<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
        observe_session: bool,
    ) -> Result<T, IdentityError> {
        let status = response.status();
        if !status.is_success() {
            let body = response
                .text()
                .await
                .map_err(|error| IdentityError::Response(error.into()))?;
            if observe_session {
                self.observe_refusal(status, &body);
            }
            return Err(classify_identity_error(status, &body));
        }
        response
            .json()
            .await
            .map_err(|error| IdentityError::Response(error.into()))
    }
}

fn classify_identity_error(status: reqwest::StatusCode, body: &str) -> IdentityError {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(body) {
        let detail = &value["detail"];
        if let Some(code) = detail["code"].as_str() {
            let msg = detail["message"]
                .as_str()
                .unwrap_or("This link no longer works")
                .to_owned();
            match code {
                "challenge_invalid" => return IdentityError::InvalidChallenge(msg),
                "terms_acceptance_required" => return IdentityError::TermsRequired,
                "agreement_version_changed" => return IdentityError::AgreementChanged,
                "stale_state" => return IdentityError::Conflict(msg),
                _ => {}
            }
        }
        if status == reqwest::StatusCode::UNPROCESSABLE_ENTITY
            && let Some(msg) = detail.as_str()
        {
            return IdentityError::InvalidInput(msg.to_owned());
        }
    }
    match crate::refusal::classify(status, body) {
        Some(crate::refusal::SessionRefusal::Revoked) => IdentityError::Ended,
        Some(crate::refusal::SessionRefusal::Absent) => IdentityError::AccountAbsent,
        Some(crate::refusal::SessionRefusal::Invalid) => IdentityError::Unauthorized,
        None => IdentityError::Unavailable,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_challenge_terms_upgrade_and_conflict_without_triggering_session_end() {
        let challenge = classify_identity_error(
            reqwest::StatusCode::UNAUTHORIZED,
            r#"{"detail":{"code":"challenge_invalid","message":"This link no longer works"}}"#,
        );
        assert_eq!(
            challenge,
            IdentityError::InvalidChallenge("This link no longer works".into())
        );
        assert!(
            crate::refusal::classify(
                reqwest::StatusCode::UNAUTHORIZED,
                r#"{"detail":{"code":"challenge_invalid","message":"This link no longer works"}}"#,
            )
            .is_none()
        );

        assert_eq!(
            classify_identity_error(
                reqwest::StatusCode::FORBIDDEN,
                r#"{"detail":{"code":"terms_acceptance_required"}}"#,
            ),
            IdentityError::TermsRequired
        );
        assert_eq!(
            classify_identity_error(
                reqwest::StatusCode::CONFLICT,
                r#"{"detail":{"code":"agreement_version_changed"}}"#,
            ),
            IdentityError::AgreementChanged
        );
        assert_eq!(
            classify_identity_error(
                reqwest::StatusCode::UNAUTHORIZED,
                r#"{"detail":{"code":"session_revoked"}}"#,
            ),
            IdentityError::Ended
        );
        assert_eq!(
            classify_identity_error(
                reqwest::StatusCode::UNAUTHORIZED,
                r#"{"detail":{"code":"account_absent"}}"#,
            ),
            IdentityError::AccountAbsent
        );
    }
}
