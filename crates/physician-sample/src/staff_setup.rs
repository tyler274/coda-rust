//! Authenticated Coda staff routes for practice creation and physician account lifecycle.
//!
//! Mounted under `/auth/staff/*` only when `PHYSICIAN_AUTH=required` and
//! `CODA_STAFF_SETUP_TOKEN` is configured ([`crate::hosting::StaffSetupAuth`]).
//! Completely separate from `PHYSICIAN_STAGING_ACCOUNTS` and `POST /auth/rehearsal`:
//! staff authenticate with a constant-time-verified bearer secret to create a
//! practice with its clinical clock and validated IANA time zone, invite physicians
//! through the standard sign-in/invitation delivery flow, create physician or
//! practice-admin accounts, and deactivate or reactivate accounts in that practice.

use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::body::Body;
use axum::extract::{Path, Request, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use physician_auth::{
    Account, AccountCreateOutcome, AccountSetupError, PracticeClock, PracticeRecord,
    PracticeSetupError, PracticeSetupOutcome, PracticeTimeZone, SignInError,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::auth::{self, Auth};
use crate::hosting::StaffSetupAuth;

const STAFF_BODY_LIMIT: usize = 64 * 1024;

#[derive(Clone)]
struct StaffState {
    auth: Arc<Auth>,
    staff: StaffSetupAuth,
}

/// Wire representation of a practice's clinical clock on `/auth/staff/practices/{practice_id}`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum PracticeClockWire {
    /// Real-time practice clock (`PracticeClock::RealTime`).
    #[default]
    RealTime,
    /// Synthetic fixed clinical clock (`PracticeClock::Fixed(unix_seconds)`).
    Fixed {
        /// Fixed Unix timestamp in seconds.
        unix_seconds: u64,
    },
}

impl From<PracticeClock> for PracticeClockWire {
    fn from(clock: PracticeClock) -> Self {
        match clock {
            PracticeClock::RealTime => Self::RealTime,
            PracticeClock::Fixed(unix_seconds) => Self::Fixed { unix_seconds },
        }
    }
}

impl From<PracticeClockWire> for PracticeClock {
    fn from(wire: PracticeClockWire) -> Self {
        match wire {
            PracticeClockWire::RealTime => Self::RealTime,
            PracticeClockWire::Fixed { unix_seconds } => Self::Fixed(unix_seconds),
        }
    }
}

/// Role assigned to an invited or created physician account.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaffAccountRole {
    /// Practice administrator (panel assignment and roster invitation permission).
    Admin,
    /// Standard practice physician.
    #[default]
    Physician,
}

impl StaffAccountRole {
    fn is_admin(self) -> bool {
        matches!(self, Self::Admin)
    }

    fn from_admin(admin: bool) -> Self {
        if admin { Self::Admin } else { Self::Physician }
    }
}

#[derive(Debug, Serialize)]
struct StaffAccountWire {
    practice_id: String,
    physician_id: String,
    email: String,
    display_name: String,
    role: StaffAccountRole,
    active: bool,
    sessions_ended_before: u64,
}

impl From<Account> for StaffAccountWire {
    fn from(account: Account) -> Self {
        Self {
            practice_id: account.identity.practice_id,
            physician_id: account.identity.physician_id,
            email: account.email,
            display_name: account.display_name,
            role: StaffAccountRole::from_admin(account.admin),
            active: account.active,
            sessions_ended_before: account.sessions_ended_before,
        }
    }
}

#[derive(Debug, Deserialize)]
struct PutPracticeBody {
    #[serde(default)]
    clock: PracticeClockWire,
    time_zone: String,
}

#[derive(Debug, Deserialize)]
struct StaffInviteBody {
    email: String,
    display_name: String,
    #[serde(default)]
    role: StaffAccountRole,
}

#[derive(Debug, Deserialize)]
struct CreateAccountBody {
    #[serde(default)]
    physician_id: Option<String>,
    email: String,
    display_name: String,
    #[serde(default)]
    role: StaffAccountRole,
}

#[derive(Debug, Deserialize)]
struct AccountLifecycleBody {
    email: String,
}

/// Builds the `/auth/staff/*` router guarded by `staff`.
pub(crate) fn routes(auth: Arc<Auth>, staff: StaffSetupAuth) -> Router {
    let state = StaffState { auth, staff };
    Router::new()
        .route(
            "/auth/staff/practices/{practice_id}",
            get(get_practice).put(put_practice),
        )
        .route(
            "/auth/staff/practices/{practice_id}/invitations",
            post(invite_physician),
        )
        .route(
            "/auth/staff/practices/{practice_id}/accounts",
            post(create_account),
        )
        .route(
            "/auth/staff/practices/{practice_id}/accounts/deactivate",
            post(deactivate_account),
        )
        .route(
            "/auth/staff/practices/{practice_id}/accounts/reactivate",
            post(reactivate_account),
        )
        .with_state(state)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("Valid Coda staff authorization is required.")]
struct StaffUnauthorized;

impl IntoResponse for StaffUnauthorized {
    fn into_response(self) -> Response {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "detail": self.to_string(),
                "code": "staff_unauthorized",
            })),
        )
            .into_response()
    }
}

fn require_staff(staff: &StaffSetupAuth, request: &Request<Body>) -> Result<(), StaffUnauthorized> {
    let Some(token) = auth::bearer(request) else {
        return Err(StaffUnauthorized);
    };
    if !staff.verify(&token) {
        return Err(StaffUnauthorized);
    }
    Ok(())
}

async fn get_practice(
    State(state): State<StaffState>,
    Path(practice_id): Path<String>,
    request: Request<Body>,
) -> Response {
    if let Err(refused) = require_staff(&state.staff, &request) {
        return refused.into_response();
    }
    let record = match state.auth.store.practice(&practice_id).await {
        Ok(Some(record)) => record,
        Ok(None) => return auth::not_found(),
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };
    let Ok(accounts) = state
        .auth
        .store
        .practice_accounts(&record.practice_id)
        .await
    else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Ok(patients) = state
        .auth
        .store
        .practice_patients(&record.practice_id)
        .await
    else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let accounts: Vec<StaffAccountWire> =
        accounts.into_iter().map(StaffAccountWire::from).collect();
    (
        StatusCode::OK,
        Json(json!({
            "practice_id": record.practice_id,
            "clock": PracticeClockWire::from(record.clock),
            "time_zone": record.time_zone.as_str(),
            "patient_count": patients.len(),
            "accounts": accounts,
        })),
    )
        .into_response()
}

async fn put_practice(
    State(state): State<StaffState>,
    Path(practice_id): Path<String>,
    request: Request<Body>,
) -> Response {
    if let Err(refused) = require_staff(&state.staff, &request) {
        return refused.into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(request.into_body(), STAFF_BODY_LIMIT).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(body) = serde_json::from_slice::<PutPracticeBody>(&bytes) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "detail": "Invalid practice setup request body.",
                "code": "invalid_body",
            })),
        )
            .into_response();
    };
    let time_zone = match PracticeTimeZone::parse(&body.time_zone) {
        Ok(zone) => zone,
        Err(error) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "detail": error.to_string(),
                    "code": "invalid_time_zone",
                })),
            )
                .into_response();
        }
    };
    let record = PracticeRecord {
        practice_id,
        clock: PracticeClock::from(body.clock),
        time_zone,
    };
    let _publication = state.auth.publication.lock().await;
    match state.auth.store.upsert_practice(&record).await {
        Ok(outcome) => {
            let status = match outcome {
                PracticeSetupOutcome::Created => StatusCode::CREATED,
                PracticeSetupOutcome::Unchanged => StatusCode::OK,
            };
            (
                status,
                Json(json!({
                    "practice_id": record.practice_id.trim(),
                    "outcome": outcome,
                    "clock": PracticeClockWire::from(record.clock),
                    "time_zone": record.time_zone.as_str(),
                })),
            )
                .into_response()
        }
        Err(PracticeSetupError::InvalidPracticeId(detail)) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "detail": format!("Practice identifier `{detail}` is invalid."),
                "code": "invalid_practice_id",
            })),
        )
            .into_response(),
        Err(PracticeSetupError::Conflict {
            practice_id,
            existing_clock,
            existing_time_zone,
        }) => (
            StatusCode::CONFLICT,
            Json(json!({
                "detail": format!("Practice `{practice_id}` already exists with different clock or time zone settings."),
                "code": "practice_conflict",
                "existing_clock": PracticeClockWire::from(existing_clock),
                "existing_time_zone": existing_time_zone.as_str(),
            })),
        )
            .into_response(),
        Err(PracticeSetupError::Unavailable(_)) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}

async fn invite_physician(
    State(state): State<StaffState>,
    Path(practice_id): Path<String>,
    request: Request<Body>,
) -> Response {
    if let Err(refused) = require_staff(&state.staff, &request) {
        return refused.into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(request.into_body(), STAFF_BODY_LIMIT).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(body) = serde_json::from_slice::<StaffInviteBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let practice = match state.auth.store.practice(&practice_id).await {
        Ok(Some(practice)) => practice,
        Ok(None) => return auth::not_found(),
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };
    let email = body.email.trim().to_lowercase();
    let display_name = body.display_name.trim().to_owned();
    let valid_email = email
        .split_once('@')
        .is_some_and(|(local, domain)| !local.is_empty() && domain.contains('.'))
        && !email.chars().any(char::is_whitespace);
    if !valid_email || display_name.is_empty() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "detail": "Invitation requires a valid email address and display name.",
                "code": "invalid_invitation",
            })),
        )
            .into_response();
    }
    if state.auth.store.account_by_email(&email).await.is_some() {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "detail": "An account with that email address already exists.",
                "code": "account_exists",
            })),
        )
            .into_response();
    }
    let token = match state
        .auth
        .store
        .invite(
            &practice.practice_id,
            &email,
            &display_name,
            body.role.is_admin(),
            auth::now(),
        )
        .await
    {
        Ok(token) => token,
        Err(SignInError::Invalid | SignInError::Inactive) => {
            return (
                StatusCode::CONFLICT,
                Json(json!({
                    "detail": "That email address already has a conflicting invitation.",
                    "code": "invitation_conflict",
                })),
            )
                .into_response();
        }
    };
    match &state.auth.mail {
        crate::gmail::Delivery::Proof => {
            state
                .auth
                .delivery
                .lock()
                .expect("delivery")
                .push(token.clone());
            let sign_in_path = format!(
                "/?code={}&email={}",
                percent_encode(&token),
                percent_encode(&email)
            );
            (
                StatusCode::OK,
                Json(json!({
                    "practice_id": practice.practice_id,
                    "email": email,
                    "display_name": display_name,
                    "role": body.role,
                    "status": "invited",
                    "delivery": "stored",
                    "token": token,
                    "sign_in_path": sign_in_path,
                })),
            )
                .into_response()
        }
        crate::gmail::Delivery::Hosted(sender) => {
            if let Err(error) = sender.send_physician_invite(&email, &token).await {
                tracing::warn!(%error, "physician invitation mail was not sent");
                return (
                    StatusCode::BAD_GATEWAY,
                    Json(json!({
                        "detail": "The physician invitation email could not be sent.",
                        "code": "mail_delivery_failed",
                    })),
                )
                    .into_response();
            }
            (
                StatusCode::OK,
                Json(json!({
                    "practice_id": practice.practice_id,
                    "email": email,
                    "display_name": display_name,
                    "role": body.role,
                    "status": "invited",
                    "delivery": "mailed",
                })),
            )
                .into_response()
        }
    }
}

async fn create_account(
    State(state): State<StaffState>,
    Path(practice_id): Path<String>,
    request: Request<Body>,
) -> Response {
    if let Err(refused) = require_staff(&state.staff, &request) {
        return refused.into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(request.into_body(), STAFF_BODY_LIMIT).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(body) = serde_json::from_slice::<CreateAccountBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let practice = match state.auth.store.practice(&practice_id).await {
        Ok(Some(practice)) => practice,
        Ok(None) => return auth::not_found(),
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };
    let _publication = state.auth.publication.lock().await;
    match state
        .auth
        .store
        .create_account(
            &practice.practice_id,
            body.physician_id.as_deref(),
            &body.email,
            &body.display_name,
            body.role.is_admin(),
            auth::now(),
        )
        .await
    {
        Ok((account, outcome)) => {
            let status = match outcome {
                AccountCreateOutcome::Created => StatusCode::CREATED,
                AccountCreateOutcome::Existing => StatusCode::OK,
            };
            (
                status,
                Json(json!({
                    "outcome": outcome,
                    "account": StaffAccountWire::from(account),
                })),
            )
                .into_response()
        }
        Err(AccountSetupError::InvalidInput(detail)) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "detail": detail,
                "code": "invalid_account",
            })),
        )
            .into_response(),
        Err(AccountSetupError::EmailConflict {
            email,
            existing_practice_id,
        }) => (
            StatusCode::CONFLICT,
            Json(json!({
                "detail": format!("Email `{email}` already belongs to another practice (`{existing_practice_id}`)."),
                "code": "email_conflict",
            })),
        )
            .into_response(),
        Err(AccountSetupError::PhysicianIdConflict {
            practice_id,
            physician_id,
        }) => (
            StatusCode::CONFLICT,
            Json(json!({
                "detail": format!("Physician `{physician_id}` already exists in practice `{practice_id}`."),
                "code": "physician_id_conflict",
            })),
        )
            .into_response(),
        Err(AccountSetupError::Unavailable(_)) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AccountTransition {
    Deactivate,
    Reactivate,
}

async fn deactivate_account(
    State(state): State<StaffState>,
    Path(practice_id): Path<String>,
    request: Request<Body>,
) -> Response {
    change_account_active(state, practice_id, request, AccountTransition::Deactivate).await
}

async fn reactivate_account(
    State(state): State<StaffState>,
    Path(practice_id): Path<String>,
    request: Request<Body>,
) -> Response {
    change_account_active(state, practice_id, request, AccountTransition::Reactivate).await
}

async fn change_account_active(
    state: StaffState,
    practice_id: String,
    request: Request<Body>,
    transition: AccountTransition,
) -> Response {
    if let Err(refused) = require_staff(&state.staff, &request) {
        return refused.into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(request.into_body(), STAFF_BODY_LIMIT).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(body) = serde_json::from_slice::<AccountLifecycleBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let email = body.email.trim().to_lowercase();
    let _publication = state.auth.publication.lock().await;
    let Some(existing) = state.auth.store.account_by_email(&email).await else {
        return auth::not_found();
    };
    if existing.identity.practice_id != practice_id.trim() {
        return auth::not_found();
    }
    let updated = match transition {
        AccountTransition::Reactivate => state.auth.store.reactivate(&email).await,
        AccountTransition::Deactivate => state.auth.store.deactivate(&email).await,
    };
    if !updated {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    let Some(account) = state.auth.store.account_by_email(&email).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    (
        StatusCode::OK,
        Json(json!({
            "account": StaffAccountWire::from(account),
        })),
    )
        .into_response()
}

fn percent_encode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}
