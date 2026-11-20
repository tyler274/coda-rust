//! Duet Connected Care invitation inspection, DOB verification, and `Share` client calls.

use crate::DuetClient;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// How a Duet patient presents an open practice invitation (`link_token` or `short_code`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum InvitationCredential {
    /// Long link token delivered in `/invite?token=...` or `duet://invite?token=...`.
    LinkToken(String),
    /// Human-enterable `XXXX-XXXX` short code.
    ShortCode(String),
}

impl InvitationCredential {
    /// Classifies an incoming invitation string (`codaf-...` or deep-link token vs manual short code).
    pub fn from_input(raw: &str) -> Self {
        let trimmed = raw.trim();
        if trimmed.starts_with("codaf-") || trimmed.len() > 16 {
            Self::LinkToken(trimmed.to_owned())
        } else {
            Self::ShortCode(trimmed.to_owned())
        }
    }
}

/// Lock standing of an open invitation before DOB is verified (`Rule 6`, `Rule 7`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum InvitationOpenStanding {
    /// Open and accepting date-of-birth checks.
    Active {
        /// Wrong DOB answers recorded so far.
        wrong_dob_attempts: u8,
        /// Wrong DOB answers remaining before lock.
        attempts_remaining: u8,
    },
    /// Locked after 5 wrong DOB answers (`COD-48`).
    Locked {
        /// Unix second the invitation locked.
        locked_at: u64,
    },
}

/// Pre-DOB gate status for an open invitation (reveals zero practice or sharing details).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvitationGateState {
    /// Whether the invitation can still accept DOB attempts or is locked.
    pub standing: InvitationOpenStanding,
}

/// Plain preview shown only after the patient's entered DOB matches (`Rule 6`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvitationSharingPreview {
    /// Practice identifier.
    pub practice_id: String,
    /// Practice display name.
    pub practice_name: String,
    /// Physician display name.
    pub physician_name: String,
    /// Patient display name on the invitation.
    pub display_name: String,
    /// Plain summary of what tapping `Share` connects.
    pub shares_summary: String,
}

/// One roster patient connected to or ended with a practice.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectedCareRosterPatient {
    /// Practice identifier.
    pub practice_id: String,
    /// Practice-assigned patient workspace identifier.
    pub patient_id: String,
}

/// Standing of one Duet <-> practice connection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ConnectedCareConnectionStanding {
    /// Actively sharing.
    Connected {
        /// Unix second the connection started.
        connected_at: u64,
    },
    /// Previously connected; sharing stopped at `ended_at`.
    Ended {
        /// Unix second the most recent connection began.
        connected_at: u64,
        /// Unix second the connection ended.
        ended_at: u64,
        /// Why the connection ended (`patient_left`, `practice_discharged`, `account_deleted`).
        reason: String,
    },
}

/// One Connected Care connection belonging to the signed-in Duet account.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectedCareConnection {
    /// Practice and roster patient identifiers.
    pub patient: ConnectedCareRosterPatient,
    /// Active vs ended standing.
    #[serde(flatten)]
    pub standing: ConnectedCareConnectionStanding,
}

/// Response returned after tapping `Share` (`POST /api/connected-care/invitations/share`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectedCareShareResult {
    /// Front-door account id mapped to the signed-in Duet account.
    pub front_door_account_id: Uuid,
    /// Newly created or re-connected practice connection.
    pub connection: ConnectedCareConnection,
}

/// Actionable failure states returned by Connected Care invitation routes.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ConnectedCareError {
    /// Session expired or invalid.
    #[error("Sign in again to continue.")]
    Unauthorized,
    /// Current Duet Terms of Service must be accepted before sharing.
    #[error("Accept the current Duet Terms of Service to connect.")]
    TermsRequired,
    /// Invitation link or short code is unknown, already used, or cancelled (`Rule 5`).
    #[error("This invitation is not open.")]
    NotOpen,
    /// Locked after 5 wrong birth-date answers (`COD-48`, `Rule 7`).
    #[error("This invitation is locked. Ask your practice to cancel and send a new invitation.")]
    Locked,
    /// Entered date of birth did not match the practice chart.
    #[error("That date of birth doesn't match ({attempts_remaining} tries left).")]
    WrongBirthDate {
        /// Wrong-answer budget remaining before lock.
        attempts_remaining: u8,
    },
    /// Invitee is under 18 (`Rule 22`).
    #[error("Duet Connected Care requires being 18 or older.")]
    Underage,
    /// Date of birth was malformed.
    #[error("Enter a valid date of birth (YYYY-MM-DD).")]
    InvalidBirthDate,
    /// Front-door account is inactive.
    #[error("This Connected Care account is inactive.")]
    InactiveAccount,
    /// Another account already holds an active connection for this roster patient.
    #[error("This patient is already connected to that practice.")]
    Conflict,
    /// Network or service failure.
    #[error("Connected Care is unavailable right now.")]
    Unavailable,
}

#[derive(Deserialize)]
struct ApiErrorEnvelope {
    #[serde(default)]
    code: String,
    #[serde(default)]
    attempts_remaining: Option<u8>,
}

impl DuetClient {
    fn connected_care_post(&self, path: &str, body: serde_json::Value) -> reqwest::RequestBuilder {
        let mut req = self
            .configuration
            .client
            .post(format!("{}{path}", self.configuration.base_path))
            .header(reqwest::header::CACHE_CONTROL, "no-store")
            .json(&body);
        if let Some(token) = &self.configuration.bearer_access_token {
            req = req.bearer_auth(token);
        }
        req
    }

    async fn send_connected_care<T: serde::de::DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<T, ConnectedCareError> {
        let resp = request
            .send()
            .await
            .map_err(|_| ConnectedCareError::Unavailable)?;
        let status = resp.status();
        if status.is_success() {
            return resp
                .json::<T>()
                .await
                .map_err(|_| ConnectedCareError::Unavailable);
        }
        let body = resp.text().await.unwrap_or_default();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            self.observe_refusal(status, &body);
            return Err(ConnectedCareError::Unauthorized);
        }
        if status == reqwest::StatusCode::LOCKED {
            return Err(ConnectedCareError::Locked);
        }
        if status == reqwest::StatusCode::CONFLICT {
            return Err(ConnectedCareError::Conflict);
        }
        if status == reqwest::StatusCode::NOT_FOUND {
            return Err(ConnectedCareError::NotOpen);
        }
        if let Ok(outer) = serde_json::from_str::<serde_json::Value>(&body)
            && matches!(
                outer["detail"]["code"].as_str(),
                Some("terms_acceptance_required" | "agreement_changed")
            )
        {
            return Err(ConnectedCareError::TermsRequired);
        }
        let envelope = serde_json::from_str::<ApiErrorEnvelope>(&body).ok();
        let code = envelope.as_ref().map(|e| e.code.as_str()).unwrap_or("");
        match (status, code) {
            (reqwest::StatusCode::FORBIDDEN, "underage") => Err(ConnectedCareError::Underage),
            (reqwest::StatusCode::FORBIDDEN, _) => Err(ConnectedCareError::InactiveAccount),
            (reqwest::StatusCode::UNPROCESSABLE_ENTITY, "wrong_birth_date") => {
                Err(ConnectedCareError::WrongBirthDate {
                    attempts_remaining: envelope.and_then(|e| e.attempts_remaining).unwrap_or(0),
                })
            }
            (reqwest::StatusCode::UNPROCESSABLE_ENTITY, _) => {
                Err(ConnectedCareError::InvalidBirthDate)
            }
            _ => Err(ConnectedCareError::Unavailable),
        }
    }

    /// Inspects an invitation credential before DOB (`Rule 6`, no practice preview leaked).
    pub async fn inspect_connected_care_invitation(
        &self,
        credential: &InvitationCredential,
    ) -> Result<InvitationGateState, ConnectedCareError> {
        self.send_connected_care(self.connected_care_post(
            "/api/connected-care/invitations/inspect",
            serde_json::json!({ "credential": credential }),
        ))
        .await
    }

    /// Verifies the patient's entered DOB (`YYYY-MM-DD`) and `18+` eligibility without connecting.
    pub async fn verify_connected_care_dob(
        &self,
        credential: &InvitationCredential,
        birth_date: &str,
    ) -> Result<InvitationSharingPreview, ConnectedCareError> {
        self.send_connected_care(self.connected_care_post(
            "/api/connected-care/invitations/verify-dob",
            serde_json::json!({ "credential": credential, "birth_date": birth_date }),
        ))
        .await
    }

    /// Connects the signed-in Duet account when the patient explicitly taps `Share` (`Rule 2`, `Rule 9`).
    pub async fn accept_connected_care_share(
        &self,
        credential: &InvitationCredential,
        birth_date: &str,
    ) -> Result<ConnectedCareShareResult, ConnectedCareError> {
        self.send_connected_care(self.connected_care_post(
            "/api/connected-care/invitations/share",
            serde_json::json!({ "credential": credential, "birth_date": birth_date }),
        ))
        .await
    }

    /// Lists all Connected Care practice connections (`Connected` and `Ended`) for the signed-in account.
    pub async fn connected_care_connections(
        &self,
    ) -> Result<Vec<ConnectedCareConnection>, ConnectedCareError> {
        let mut req = self
            .configuration
            .client
            .get(format!(
                "{}/api/connected-care/connections",
                self.configuration.base_path
            ))
            .header(reqwest::header::CACHE_CONTROL, "no-store");
        if let Some(token) = &self.configuration.bearer_access_token {
            req = req.bearer_auth(token);
        }
        self.send_connected_care(req).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_input_distinguishes_link_tokens_from_short_codes() {
        assert_eq!(
            InvitationCredential::from_input(" codaf-0123456789 "),
            InvitationCredential::LinkToken("codaf-0123456789".to_owned())
        );
        assert_eq!(
            InvitationCredential::from_input(" ABCD-EFGH "),
            InvitationCredential::ShortCode("ABCD-EFGH".to_owned())
        );
    }
}
