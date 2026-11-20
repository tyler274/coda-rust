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
    /// The server refused this session because it was revoked or deleted.
    Ended,
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

impl std::fmt::Display for IdentityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unauthorized => f.write_str("No valid session. Sign in again."),
            Self::Ended => f.write_str("This session has ended."),
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
        Some(crate::refusal::SessionRefusal::Revoked | crate::refusal::SessionRefusal::Absent) => {
            IdentityError::Ended
        }
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
    }
}
