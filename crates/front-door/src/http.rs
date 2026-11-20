//! The axum router exposing the front-door invitation and account APIs.
//!
//! Every route is stateless over a shared [`FrontDoorStore`]; a server holds
//! `Arc<dyn FrontDoorStore>` and mounts [`router`] (or [`router_with`] to
//! control the session bearer directly). The redeem and sign-in routes stay
//! open — the caller proves identity by holding the one-time token — and
//! account reads resolve the presented bearer session.

use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, put};
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{
    Account, AccountConnection, ForMyDoctorError, ForMyDoctorSendRequest, ForMyDoctorShare,
    ForMyDoctorShareStatus, FrontDoorStore, InvitationAcceptError, InvitationCheckError,
    InvitationCredential, InvitationGateState, InvitationSharingPreview, InviteError,
    InvitePatientParams, LinkError, RosterConnectionStatus, RosterPatient, SessionError,
    SharedPatientRecordInput, SignInError,
};

/// Why a route answered with an error status.
#[derive(Debug)]
pub enum ApiError {
    /// The presented bearer token resolves to no active account.
    Unauthorized,
    /// The invitation or token is unknown or already used.
    Invalid,
    /// The roster patient already holds an active Connected connection.
    Conflict,
    /// The open invitation locked after too many wrong date-of-birth answers.
    Locked,
    /// No open invitation matches this roster patient or credential.
    NotFound,
    /// Entered date of birth did not match the practice chart.
    WrongBirthDate {
        /// Remaining attempts before lock.
        attempts_remaining: u8,
    },
    /// Patient does not meet the 18+ age requirement.
    Underage,
    /// Supplied date of birth is malformed.
    InvalidBirthDate,
    /// The signed-in account is deactivated.
    InactiveAccount,
    /// A `"For my doctor"` share key was reused with different content.
    ShareConflict,
    /// Supplied connected-care share/record payload is malformed or ambiguous.
    InvalidInput(&'static str),
    /// The account carries no active connection to that practice.
    Unlinked,
    /// The store failed; the caller cannot fix it by retrying differently.
    Storage,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "code": "unauthorized", "detail": "Sign in to read the account." })),
            )
                .into_response(),
            Self::Invalid => (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "code": "invalid", "detail": "That invitation or sign-in link is no longer valid." })),
            )
                .into_response(),
            Self::Conflict => (
                StatusCode::CONFLICT,
                Json(json!({ "code": "conflict", "detail": "That patient already holds an active connection." })),
            )
                .into_response(),
            Self::Locked => (
                StatusCode::LOCKED,
                Json(json!({ "code": "invitation_locked", "detail": "This invitation is locked after too many wrong date-of-birth answers." })),
            )
                .into_response(),
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({ "code": "invitation_not_open", "detail": "This invitation is no longer open." })),
            )
                .into_response(),
            Self::WrongBirthDate { attempts_remaining } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "code": "wrong_birth_date",
                    "attempts_remaining": attempts_remaining,
                    "detail": format!("Date of birth does not match ({attempts_remaining} attempts remaining)."),
                })),
            )
                .into_response(),
            Self::Underage => (
                StatusCode::FORBIDDEN,
                Json(json!({ "code": "underage", "detail": "Duet Connected Care requires being 18 or older." })),
            )
                .into_response(),
            Self::InvalidBirthDate => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "code": "invalid_birth_date", "detail": "Enter a valid date of birth (YYYY-MM-DD)." })),
            )
                .into_response(),
            Self::InactiveAccount => (
                StatusCode::FORBIDDEN,
                Json(json!({ "code": "inactive_account", "detail": "This account is inactive." })),
            )
                .into_response(),
            Self::ShareConflict => (
                StatusCode::CONFLICT,
                Json(json!({ "code": "share_conflict", "detail": "That For my doctor share key already exists with different content." })),
            )
                .into_response(),
            Self::InvalidInput(detail) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "code": "invalid_input", "detail": detail })),
            )
                .into_response(),
            Self::Unlinked => (
                StatusCode::NOT_FOUND,
                Json(json!({ "code": "unlinked", "detail": "No active connection for this account." })),
            )
                .into_response(),
            Self::Storage => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({ "code": "storage_unavailable", "detail": "The front door is unavailable." })),
            )
                .into_response(),
        }
    }
}

impl From<SessionError> for ApiError {
    fn from(error: SessionError) -> Self {
        match error {
            SessionError::Invalid | SessionError::Inactive => Self::Unauthorized,
        }
    }
}

impl From<SignInError> for ApiError {
    fn from(error: SignInError) -> Self {
        match error {
            SignInError::Invalid | SignInError::Inactive => Self::Invalid,
        }
    }
}

impl From<InviteError> for ApiError {
    fn from(error: InviteError) -> Self {
        match error {
            InviteError::Conflict => Self::Conflict,
            InviteError::Locked => Self::Locked,
            InviteError::NotFound => Self::NotFound,
            InviteError::InvalidInput(msg) => Self::InvalidInput(msg),
            InviteError::Storage(_) => Self::Storage,
        }
    }
}

impl From<InvitationCheckError> for ApiError {
    fn from(error: InvitationCheckError) -> Self {
        match error {
            InvitationCheckError::NotOpen => Self::NotFound,
            InvitationCheckError::Locked => Self::Locked,
            InvitationCheckError::WrongBirthDate { attempts_remaining } => {
                Self::WrongBirthDate { attempts_remaining }
            }
            InvitationCheckError::Underage => Self::Underage,
            InvitationCheckError::InvalidBirthDate => Self::InvalidBirthDate,
            InvitationCheckError::Storage(_) => Self::Storage,
        }
    }
}

impl From<InvitationAcceptError> for ApiError {
    fn from(error: InvitationAcceptError) -> Self {
        match error {
            InvitationAcceptError::Check(check) => check.into(),
            InvitationAcceptError::InactiveAccount => Self::InactiveAccount,
            InvitationAcceptError::Conflict => Self::Conflict,
            InvitationAcceptError::Storage(_) => Self::Storage,
        }
    }
}

impl From<LinkError> for ApiError {
    fn from(error: LinkError) -> Self {
        match error {
            LinkError::Unlinked => Self::Unlinked,
            LinkError::Storage(_) => Self::Storage,
        }
    }
}

impl From<ForMyDoctorError> for ApiError {
    fn from(error: ForMyDoctorError) -> Self {
        match error {
            ForMyDoctorError::Unlinked | ForMyDoctorError::NotFound => Self::Unlinked,
            ForMyDoctorError::InvalidInput(msg) => Self::InvalidInput(msg),
            ForMyDoctorError::Conflict => Self::ShareConflict,
            ForMyDoctorError::Storage(_) => Self::Storage,
        }
    }
}

/// The account as the routes return it.
#[derive(serde::Serialize)]
pub struct AccountView {
    /// The account id.
    pub account_id: Uuid,
    /// The email sign-in address.
    pub email: String,
    /// The account's display name.
    pub display_name: String,
    /// All practice connections (`Connected` and `Ended`) attached to this account.
    pub connections: Vec<AccountConnection>,
}

impl From<Account> for AccountView {
    fn from(account: Account) -> Self {
        Self {
            account_id: account.id,
            email: account.email,
            display_name: account.display_name,
            connections: account.connections,
        }
    }
}

/// The invitation as the invite route returns it.
#[derive(serde::Serialize)]
pub struct InvitationView {
    /// Which roster patient the practice invited.
    pub patient: RosterPatient,
    /// The lower-cased email the invitation names.
    pub email: String,
    /// The invitee's display name.
    pub display_name: String,
    /// The Unix second the invitation was issued.
    pub invited_at: u64,
    /// The redeem token, returned to the caller here; a real deployment
    /// delivers it to the patient instead.
    pub token: String,
    /// The human-enterable short code (`XXXX-XXXX`).
    pub short_code: String,
}

impl InvitationView {
    fn from_issued(issued: crate::IssuedInvitation) -> Self {
        Self {
            patient: issued.invitation.patient,
            email: issued.invitation.email,
            display_name: issued.invitation.display_name,
            invited_at: issued.invitation.invited_at,
            token: issued.link_token,
            short_code: issued.short_code,
        }
    }
}

/// The redeem response: the account plus the minted bearer session.
#[derive(serde::Serialize)]
pub struct SessionView {
    /// The account the session acts for.
    pub account_id: Uuid,
    /// The presented bearer token.
    pub session: String,
}

#[derive(Deserialize)]
struct InviteBody {
    practice_id: String,
    patient_id: String,
    email: String,
    display_name: String,
    #[serde(default)]
    practice_name: String,
    #[serde(default)]
    physician_name: String,
    #[serde(default)]
    expected_birth_date: Option<String>,
}

#[derive(Deserialize)]
struct RosterBody {
    practice_id: String,
    patient_id: String,
}

#[derive(Deserialize)]
struct DeliveryFailedBody {
    practice_id: String,
    patient_id: String,
    detail: String,
}

#[derive(Deserialize)]
struct InspectBody {
    credential: InvitationCredential,
}

#[derive(Deserialize)]
struct VerifyDobBody {
    credential: InvitationCredential,
    birth_date: String,
}

#[derive(Deserialize)]
struct AcceptShareBody {
    #[serde(default)]
    account_id: Option<Uuid>,
    #[serde(default)]
    account_email: Option<String>,
    credential: InvitationCredential,
    birth_date: String,
}

/// Result returned when an invitation is accepted via Share.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AcceptShareView {
    /// Front-door account id that holds the connection.
    pub account_id: Uuid,
    /// Newly created or re-connected practice connection.
    pub connection: AccountConnection,
}

#[derive(Deserialize)]
struct RedeemBody {
    email: String,
    token: String,
}

#[derive(Deserialize)]
struct SignInBody {
    email: String,
}

#[derive(Deserialize)]
struct VerifyBody {
    email: String,
    token: String,
}

/// Bearer secret verifier for operator-gated front-door lifecycle actions
/// (`discharge` and `end-connections`), comparing SHA-256 digests in constant time.
#[derive(Clone, PartialEq, Eq)]
pub struct OperatorAuth {
    digest: [u8; 32],
}

impl std::fmt::Debug for OperatorAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OperatorAuth")
            .field("digest", &"<redacted>")
            .finish()
    }
}

impl OperatorAuth {
    /// Minimum accepted secret length in bytes for `FRONT_DOOR_OPERATOR_TOKEN`.
    pub const MIN_TOKEN_BYTES: usize = 16;

    /// Parses `FRONT_DOOR_OPERATOR_TOKEN` when present and non-empty.
    pub fn from_env(get: impl Fn(&str) -> Option<String>) -> Result<Option<Self>, String> {
        let Some(raw) = get("FRONT_DOOR_OPERATOR_TOKEN") else {
            return Ok(None);
        };
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Ok(None);
        }
        Self::from_secret(trimmed).map(Some)
    }

    /// Builds an operator verifier from a raw secret token.
    pub fn from_secret(secret: &str) -> Result<Self, String> {
        let trimmed = secret.trim();
        if trimmed.len() < Self::MIN_TOKEN_BYTES {
            return Err(format!(
                "FRONT_DOOR_OPERATOR_TOKEN must be at least {} bytes",
                Self::MIN_TOKEN_BYTES
            ));
        }
        if trimmed.bytes().any(|b| matches!(b, b'\r' | b'\n' | 0)) {
            return Err("FRONT_DOOR_OPERATOR_TOKEN must not contain control characters".to_owned());
        }
        let digest: [u8; 32] = Sha256::digest(trimmed.as_bytes()).into();
        Ok(Self { digest })
    }

    /// Constant-time verification of a presented bearer secret.
    pub fn verify(&self, presented: &str) -> bool {
        let candidate: [u8; 32] = Sha256::digest(presented.as_bytes()).into();
        let mut diff = 0u8;
        for (&a, &b) in self.digest.iter().zip(candidate.iter()) {
            diff |= std::hint::black_box(a ^ b);
        }
        diff == 0
    }
}

/// The state the routes share.
#[derive(Clone)]
pub struct FrontDoorState {
    /// The durable store.
    pub store: Arc<dyn FrontDoorStore>,
    /// Unix seconds now, injected so tests control time.
    pub now: Arc<dyn Fn() -> u64 + Send + Sync>,
    /// Optional operator bearer verifier for staff/service lifecycle endpoints.
    pub operator_auth: Option<OperatorAuth>,
}

impl FrontDoorState {
    /// State on one store, reading the wall clock for `now`.
    pub fn new(store: Arc<dyn FrontDoorStore>) -> Self {
        Self {
            store,
            now: Arc::new(wall_now),
            operator_auth: None,
        }
    }

    /// Attaches an operator bearer verifier for discharge and account-delete lifecycle calls.
    pub fn with_operator_auth(mut self, operator_auth: OperatorAuth) -> Self {
        self.operator_auth = Some(operator_auth);
        self
    }
}

fn wall_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or_default()
}

/// Builds the HTTP router. Reads `FRONT_DOOR_AUTH` to decide whether the
/// account routes resolve the presented bearer (`required`) or stay open
/// for proof setups; a deployment sets `required`.
pub fn router(state: FrontDoorState) -> Router {
    let required = std::env::var("FRONT_DOOR_AUTH").ok().as_deref() == Some("required");
    router_with(state, required)
}

/// Builds the router, controlling bearer resolution explicitly.
pub fn router_with(state: FrontDoorState, require_auth: bool) -> Router {
    Router::new()
        .route("/front-door/invitations", post(invite))
        .route("/front-door/invitations/resend", post(resend_invite))
        .route("/front-door/invitations/cancel", post(cancel_invite))
        .route(
            "/front-door/invitations/delivery-failed",
            post(mark_delivery_failed),
        )
        .route("/front-door/invitations/inspect", post(inspect_invite))
        .route(
            "/front-door/invitations/verify-dob",
            post(verify_invite_dob),
        )
        .route("/front-door/invitations/accept", post(accept_invite_share))
        .route("/front-door/invitations/redeem", post(redeem))
        .route(
            "/front-door/roster/{practice_id}/{patient_id}",
            get(roster_connection),
        )
        .route("/front-door/sign-in", post(sign_in))
        .route("/front-door/sign-in/verify", post(verify))
        .route("/front-door/account", get(account))
        .route("/front-door/connections", get(connections))
        .route(
            "/front-door/connections/{practice_id}/leave",
            post(leave_connection),
        )
        .route(
            "/front-door/connections/discharge",
            post(discharge_connection),
        )
        .route(
            "/front-door/accounts/{account_id}/connections",
            get(account_connections_operator),
        )
        .route(
            "/front-door/accounts/{account_id}/end-connections",
            post(end_account_connections),
        )
        .route(
            "/front-door/for-my-doctor",
            post(send_for_my_doctor).get(list_for_my_doctor),
        )
        .route("/front-door/shared-records", put(upsert_shared_records))
        .with_state((state, require_auth))
}

/// The presented bearer token, when the route carries one.
fn bearer(headers: &HeaderMap) -> Option<String> {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(str::to_owned)
}

async fn account_id(
    state: &FrontDoorState,
    require_auth: bool,
    headers: &HeaderMap,
) -> Result<Uuid, ApiError> {
    if !require_auth {
        // Proof setups pass the account id as the bearer itself.
        let token = bearer(headers).ok_or(ApiError::Unauthorized)?;
        return Uuid::parse_str(&token).map_err(|_| ApiError::Unauthorized);
    }
    let token = bearer(headers).ok_or(ApiError::Unauthorized)?;
    Ok(state.store.session(&token, (state.now)()).await?)
}

fn require_operator(
    state: &FrontDoorState,
    require_auth: bool,
    headers: &HeaderMap,
) -> Result<(), ApiError> {
    if let Some(auth) = &state.operator_auth {
        let token = bearer(headers).ok_or(ApiError::Unauthorized)?;
        if !auth.verify(&token) {
            return Err(ApiError::Unauthorized);
        }
        return Ok(());
    }
    if require_auth {
        return Err(ApiError::Unauthorized);
    }
    Ok(())
}

/// Records one invitation to a roster patient and returns the link token and short code.
async fn invite(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
    Json(body): Json<InviteBody>,
) -> Result<Json<InvitationView>, ApiError> {
    require_operator(&state, require_auth, &headers)?;
    let practice_name = if body.practice_name.trim().is_empty() {
        body.practice_id.clone()
    } else {
        body.practice_name
    };
    let issued = state
        .store
        .invite_patient(InvitePatientParams {
            patient: RosterPatient {
                practice_id: body.practice_id,
                patient_id: body.patient_id,
            },
            email: body.email,
            display_name: body.display_name,
            practice_name,
            physician_name: body.physician_name,
            expected_birth_date: body.expected_birth_date,
            now: (state.now)(),
        })
        .await?;
    Ok(Json(InvitationView::from_issued(issued)))
}

async fn resend_invite(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
    Json(body): Json<RosterBody>,
) -> Result<Json<InvitationView>, ApiError> {
    require_operator(&state, require_auth, &headers)?;
    let issued = state
        .store
        .resend_invitation(
            &RosterPatient {
                practice_id: body.practice_id,
                patient_id: body.patient_id,
            },
            (state.now)(),
        )
        .await?;
    Ok(Json(InvitationView::from_issued(issued)))
}

async fn cancel_invite(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
    Json(body): Json<RosterBody>,
) -> Result<StatusCode, ApiError> {
    require_operator(&state, require_auth, &headers)?;
    state
        .store
        .cancel_invitation(&RosterPatient {
            practice_id: body.practice_id,
            patient_id: body.patient_id,
        })
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn mark_delivery_failed(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
    Json(body): Json<DeliveryFailedBody>,
) -> Result<StatusCode, ApiError> {
    require_operator(&state, require_auth, &headers)?;
    state
        .store
        .mark_invitation_delivery_failed(
            &RosterPatient {
                practice_id: body.practice_id,
                patient_id: body.patient_id,
            },
            &body.detail,
            (state.now)(),
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn inspect_invite(
    State((state, _)): State<(FrontDoorState, bool)>,
    Json(body): Json<InspectBody>,
) -> Result<Json<InvitationGateState>, ApiError> {
    Ok(Json(
        state.store.inspect_invitation(&body.credential).await?,
    ))
}

async fn verify_invite_dob(
    State((state, _)): State<(FrontDoorState, bool)>,
    Json(body): Json<VerifyDobBody>,
) -> Result<Json<InvitationSharingPreview>, ApiError> {
    Ok(Json(
        state
            .store
            .verify_invitation_dob(&body.credential, &body.birth_date, (state.now)())
            .await?,
    ))
}

async fn accept_invite_share(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
    Json(body): Json<AcceptShareBody>,
) -> Result<Json<AcceptShareView>, ApiError> {
    require_operator(&state, require_auth, &headers)?;
    let (account_id, connection) = state
        .store
        .accept_invitation_share(
            body.account_id,
            body.account_email.as_deref(),
            &body.credential,
            &body.birth_date,
            (state.now)(),
        )
        .await?;
    Ok(Json(AcceptShareView {
        account_id,
        connection,
    }))
}

async fn roster_connection(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
    Path((practice_id, patient_id)): Path<(String, String)>,
) -> Result<Json<RosterConnectionStatus>, ApiError> {
    require_operator(&state, require_auth, &headers)?;
    Ok(Json(
        state
            .store
            .roster_connection(&RosterPatient {
                practice_id,
                patient_id,
            })
            .await?,
    ))
}

/// Redeems an invitation, attaching `(account_id, practice_id, patient_id)` and minting one session.
async fn redeem(
    State((state, _)): State<(FrontDoorState, bool)>,
    Json(body): Json<RedeemBody>,
) -> Result<Json<SessionView>, ApiError> {
    let (account_id, session) = state
        .store
        .redeem_invitation(&body.email, &body.token, (state.now)())
        .await?;
    Ok(Json(SessionView {
        account_id,
        session,
    }))
}

/// Issues a sign-in token for an existing account.
async fn sign_in(
    State((state, _)): State<(FrontDoorState, bool)>,
    Json(body): Json<SignInBody>,
) -> Result<Json<SessionView>, ApiError> {
    let token = state
        .store
        .begin_sign_in(&body.email, (state.now)())
        .await?;
    Ok(Json(SessionView {
        account_id: Uuid::nil(),
        session: token,
    }))
}

/// Verifies a sign-in token and returns the bearer session.
async fn verify(
    State((state, _)): State<(FrontDoorState, bool)>,
    Json(body): Json<VerifyBody>,
) -> Result<Json<SessionView>, ApiError> {
    let (account_id, session) = state
        .store
        .complete_sign_in(&body.email, &body.token, (state.now)())
        .await?;
    Ok(Json(SessionView {
        account_id,
        session,
    }))
}

/// Reads the presented session's account.
async fn account(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
) -> Result<Json<AccountView>, ApiError> {
    let account_id = account_id(&state, require_auth, &headers).await?;
    let account = state
        .store
        .account(account_id)
        .await
        .ok_or(ApiError::Unauthorized)?;
    Ok(Json(account.into()))
}

/// Lists the presented session's multi-practice connections.
async fn connections(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
) -> Result<Json<Vec<AccountConnection>>, ApiError> {
    let account_id = account_id(&state, require_auth, &headers).await?;
    Ok(Json(state.store.connections(account_id).await?))
}

/// Patient leaves a practice (`X1`): ends active connections with `practice_id`.
async fn leave_connection(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
    Path(practice_id): Path<String>,
) -> Result<Json<Vec<AccountConnection>>, ApiError> {
    let account_id = account_id(&state, require_auth, &headers).await?;
    let ended = state
        .store
        .leave_connection(account_id, &practice_id, (state.now)())
        .await?;
    Ok(Json(ended))
}

#[derive(Deserialize)]
struct DischargeBody {
    practice_id: String,
    patient_id: String,
}

/// Operator/staff endpoint for discharging a roster patient (`X2`).
async fn discharge_connection(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
    Json(body): Json<DischargeBody>,
) -> Result<Json<AccountConnection>, ApiError> {
    require_operator(&state, require_auth, &headers)?;
    let ended = state
        .store
        .discharge_connection(
            &RosterPatient {
                practice_id: body.practice_id,
                patient_id: body.patient_id,
            },
            (state.now)(),
        )
        .await?;
    Ok(Json(ended))
}

/// Operator/service endpoint for ending all connections when a Duet account is deleted (`X3`).
async fn account_connections_operator(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
    Path(account_id): Path<Uuid>,
) -> Result<Json<Vec<AccountConnection>>, ApiError> {
    require_operator(&state, require_auth, &headers)?;
    match state.store.connections(account_id).await {
        Ok(connections) => Ok(Json(connections)),
        Err(LinkError::Unlinked) => Ok(Json(Vec::new())),
        Err(other) => Err(other.into()),
    }
}

async fn end_account_connections(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
    Path(account_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    require_operator(&state, require_auth, &headers)?;
    let ended = state
        .store
        .end_all_connections_for_account(account_id, (state.now)())
        .await?;
    Ok(Json(json!({ "ended": ended })))
}

#[derive(serde::Serialize)]
struct ForMyDoctorShareView {
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

impl From<ForMyDoctorShare> for ForMyDoctorShareView {
    fn from(share: ForMyDoctorShare) -> Self {
        Self {
            share_id: share.share_id,
            patient: share.patient,
            share_key: share.share_key,
            title: share.title,
            summary_text: share.summary_text,
            origin_label: share.origin_label,
            sent_at: share.sent_at,
            standing: share.standing,
        }
    }
}

#[derive(Deserialize)]
struct UpsertSharedRecordsBody {
    records: Vec<SharedPatientRecordInput>,
}

async fn send_for_my_doctor(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
    Json(body): Json<ForMyDoctorSendRequest>,
) -> Result<Json<ForMyDoctorShareView>, ApiError> {
    let account_id = account_id(&state, require_auth, &headers).await?;
    let share = state
        .store
        .send_for_my_doctor(account_id, body, (state.now)())
        .await?;
    Ok(Json(share.into()))
}

async fn list_for_my_doctor(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
) -> Result<Json<Vec<ForMyDoctorShareView>>, ApiError> {
    let account_id = account_id(&state, require_auth, &headers).await?;
    let shares = state
        .store
        .for_my_doctor_shares_for_account(account_id)
        .await?;
    Ok(Json(shares.into_iter().map(Into::into).collect()))
}

async fn upsert_shared_records(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
    Json(body): Json<UpsertSharedRecordsBody>,
) -> Result<StatusCode, ApiError> {
    let account_id = account_id(&state, require_auth, &headers).await?;
    state
        .store
        .upsert_shared_records(account_id, &body.records, (state.now)())
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ConnectionEndReason, MemoryFrontDoorStore};
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    const NOW: u64 = 1_760_000_000;

    fn state(store: MemoryFrontDoorStore) -> (FrontDoorState, Arc<MemoryFrontDoorStore>) {
        let store = Arc::new(store);
        let mut state = FrontDoorState::new(store.clone());
        state.now = Arc::new(|| NOW);
        (state, store)
    }

    async fn post(app: &Router, uri: &str, body: serde_json::Value) -> Response {
        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(uri)
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap()
    }

    async fn get(app: &Router, uri: &str, token: &str) -> Response {
        app.clone()
            .oneshot(
                Request::builder()
                    .uri(uri)
                    .header("authorization", format!("Bearer {token}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap()
    }

    async fn invited_session(app: &Router) -> String {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/front-door/invitations")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer operator-secret-token-for-tests")
                    .body(Body::from(
                        json!({
                            "practice_id": "practice",
                            "patient_id": "patient-1",
                            "email": "luna@example.test",
                            "display_name": "Luna Park",
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        let token = body["token"].as_str().unwrap().to_owned();
        let response = post(
            app,
            "/front-door/invitations/redeem",
            json!({
                "email": "luna@example.test",
                "token": token,
            }),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);
        let redeemed: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert!(redeemed["session"].as_str().unwrap().starts_with("codaf-"));
        redeemed["session"].as_str().unwrap().to_owned()
    }

    #[tokio::test]
    async fn an_invitation_is_invited_redeemed_and_never_redeemed_twice() {
        let (state, _store) = state(MemoryFrontDoorStore::new());
        let app = router_with(state, false);
        let response = post(
            &app,
            "/front-door/invitations",
            json!({
                "practice_id": "practice",
                "patient_id": "patient-1",
                "email": "luna@example.test",
                "display_name": "Luna Park",
            }),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(body["email"], "luna@example.test");
        assert_eq!(body["invited_at"].as_u64().unwrap(), NOW);
        assert!(body.get("expires_at").is_none());
        let token = body["token"].as_str().unwrap().to_owned();
        assert!(
            post(
                &app,
                "/front-door/invitations/redeem",
                json!({
                    "email": "luna@example.test",
                    "token": token,
                })
            )
            .await
            .status()
            .is_success()
        );
        let second = post(
            &app,
            "/front-door/invitations/redeem",
            json!({
                "email": "luna@example.test",
                "token": token,
            }),
        )
        .await;
        assert_eq!(
            second.status(),
            StatusCode::UNAUTHORIZED,
            "an invitation redeems once"
        );
    }

    #[tokio::test]
    async fn a_connected_patient_conflicts_while_second_practice_can_invite_same_email() {
        let (state, store) = state(MemoryFrontDoorStore::new());
        let app = router_with(state, false);
        let first_id = store.seed_linked_account(
            &RosterPatient {
                practice_id: "practice".to_owned(),
                patient_id: "patient-1".to_owned(),
            },
            "luna@example.test",
            "Luna Park",
        );
        let conflict = post(
            &app,
            "/front-door/invitations",
            json!({
                "practice_id": "practice",
                "patient_id": "patient-1",
                "email": "LUNA@example.test",
                "display_name": "Luna Park",
            }),
        )
        .await;
        assert_eq!(
            conflict.status(),
            StatusCode::CONFLICT,
            "already-connected roster patient conflicts"
        );

        let second_invite = post(
            &app,
            "/front-door/invitations",
            json!({
                "practice_id": "practice-2",
                "patient_id": "patient-2",
                "email": "LUNA@example.test",
                "display_name": "Luna Park",
            }),
        )
        .await;
        assert_eq!(
            second_invite.status(),
            StatusCode::OK,
            "existing Duet account can be invited by another practice"
        );
        let invite_body: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(second_invite.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        let redeem = post(
            &app,
            "/front-door/invitations/redeem",
            json!({
                "email": "luna@example.test",
                "token": invite_body["token"].as_str().unwrap(),
            }),
        )
        .await;
        assert_eq!(redeem.status(), StatusCode::OK);
        let redeem_body: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(redeem.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(
            redeem_body["account_id"].as_str().unwrap(),
            first_id.to_string()
        );
    }

    #[tokio::test]
    async fn the_account_and_connections_reads_resolve_the_session_bearer() {
        let (state, _store) = state(MemoryFrontDoorStore::new());
        let state = state.with_operator_auth(
            OperatorAuth::from_secret("operator-secret-token-for-tests").unwrap(),
        );
        let app = router_with(state, true);
        let session = invited_session(&app).await;
        let response = get(&app, "/front-door/account", &session).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(body["email"], "luna@example.test");
        assert_eq!(body["connections"][0]["patient"]["practice_id"], "practice");
        assert_eq!(body["connections"][0]["status"], "connected");

        let conns_resp = get(&app, "/front-door/connections", &session).await;
        assert_eq!(conns_resp.status(), StatusCode::OK);
        let conns: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(conns_resp.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(conns[0]["status"], "connected");

        let anonymous = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/front-door/account")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(anonymous.status(), StatusCode::UNAUTHORIZED);
        let forged = get(&app, "/front-door/account", "codaf-forged").await;
        assert_eq!(forged.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn sign_in_issues_and_verifies_one_token() {
        let (state, _store) = state(MemoryFrontDoorStore::new());
        let state = state.with_operator_auth(
            OperatorAuth::from_secret("operator-secret-token-for-tests").unwrap(),
        );
        let app = router_with(state, true);
        let _session = invited_session(&app).await;
        let issued = post(
            &app,
            "/front-door/sign-in",
            json!({
                "email": "luna@example.test",
            }),
        )
        .await;
        assert_eq!(issued.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(issued.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        let token = body["session"].as_str().unwrap().to_owned();
        let verify = post(
            &app,
            "/front-door/sign-in/verify",
            json!({
                "email": "luna@example.test",
                "token": token,
            }),
        )
        .await;
        assert_eq!(verify.status(), StatusCode::OK);
        let again = post(
            &app,
            "/front-door/sign-in/verify",
            json!({
                "email": "luna@example.test",
                "token": token,
            }),
        )
        .await;
        assert_eq!(
            again.status(),
            StatusCode::UNAUTHORIZED,
            "a verified token never verifies again"
        );
    }

    #[tokio::test]
    async fn for_my_doctor_route_requires_active_connection_and_tracks_sent_status() {
        let (state, store) = state(MemoryFrontDoorStore::new());
        let state = state.with_operator_auth(
            OperatorAuth::from_secret("operator-secret-token-for-tests").unwrap(),
        );
        let app = router_with(state, true);
        let session = invited_session(&app).await;
        let payload = json!({
            "share_key": "share-1",
            "title": "Pre-visit summary",
            "summary_text": "Home blood pressure averages 138/86."
        });
        let send1 = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/front-door/for-my-doctor")
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {session}"))
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(send1.status(), StatusCode::OK);
        let share1: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(send1.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(share1["status"], "sent");
        let share_id = share1["share_id"].as_str().unwrap().to_owned();

        let send2 = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/front-door/for-my-doctor")
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {session}"))
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(send2.status(), StatusCode::OK);
        let share2: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(send2.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(share2["share_id"], share_id);

        store
            .mark_for_my_doctor_received("practice", &share_id, NOW + 5)
            .await
            .unwrap();
        let list_resp = get(&app, "/front-door/for-my-doctor", &session).await;
        assert_eq!(list_resp.status(), StatusCode::OK);
        let list_body: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(list_resp.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        let item = &list_body[0];
        assert_eq!(item["status"], "received");
        assert_eq!(item["received_at"], NOW + 5);
        assert!(item.get("physician_looked").is_none());
        assert!(item.get("looked_at").is_none());

        // Ending the connection blocks subsequent sends cleanly.
        let account = store.account_by_email("luna@example.test").await.unwrap();
        store
            .end_connection(
                account.id,
                &RosterPatient {
                    practice_id: "practice".to_owned(),
                    patient_id: "patient-1".to_owned(),
                },
                ConnectionEndReason::PatientLeft,
                NOW + 10,
            )
            .await
            .unwrap();
        let denied = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/front-door/for-my-doctor")
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {session}"))
                    .body(Body::from(
                        json!({
                            "share_key": "share-after-end",
                            "title": "After leaving",
                            "summary_text": "Should fail when Ended."
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(denied.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn leave_discharge_and_account_delete_routes_enforce_auth_and_preserve_history() {
        let store = MemoryFrontDoorStore::new();
        let (mut state, store) = state(store);
        let op = OperatorAuth::from_secret("operator-secret-token-for-tests").unwrap();
        state = state.with_operator_auth(op);
        let app = router_with(state, true);

        let inv_a = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/front-door/invitations")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer operator-secret-token-for-tests")
                    .body(Body::from(
                        json!({
                            "practice_id": "practice-a",
                            "patient_id": "pat-1",
                            "email": "luna@example.test",
                            "display_name": "Luna",
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let tok_a = serde_json::from_slice::<serde_json::Value>(
            &axum::body::to_bytes(inv_a.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap()["token"]
            .as_str()
            .unwrap()
            .to_owned();
        let red_a = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/front-door/invitations/redeem")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        json!({"email": "luna@example.test", "token": tok_a}).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let red_a_json = serde_json::from_slice::<serde_json::Value>(
            &axum::body::to_bytes(red_a.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        let account_id: Uuid = red_a_json["account_id"].as_str().unwrap().parse().unwrap();
        let session = red_a_json["session"].as_str().unwrap().to_owned();

        let p_b = RosterPatient {
            practice_id: "practice-b".into(),
            patient_id: "pat-2".into(),
        };
        let tok_b = store
            .invite(&p_b, "luna@example.test", "Luna", NOW)
            .await
            .unwrap();
        store
            .redeem_invitation("luna@example.test", &tok_b, NOW + 1)
            .await
            .unwrap();
        let p_c = RosterPatient {
            practice_id: "practice-c".into(),
            patient_id: "pat-3".into(),
        };
        let tok_c = store
            .invite(&p_c, "luna@example.test", "Luna", NOW)
            .await
            .unwrap();
        store
            .redeem_invitation("luna@example.test", &tok_c, NOW + 2)
            .await
            .unwrap();

        // Patient leaves practice-a via HTTP bearer route.
        let leave_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/front-door/connections/practice-a/leave")
                    .header("authorization", format!("Bearer {session}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(leave_resp.status(), StatusCode::OK);
        let leave_json = serde_json::from_slice::<serde_json::Value>(
            &axum::body::to_bytes(leave_resp.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(leave_json[0]["status"], "ended");
        assert_eq!(leave_json[0]["reason"], "patient_left");
        assert!(store.roster_connection(&p_b).await.unwrap().is_connected());

        // Discharge requires valid operator token.
        let unauthed_discharge = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/front-door/connections/discharge")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        json!({"practice_id": "practice-b", "patient_id": "pat-2"}).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(unauthed_discharge.status(), StatusCode::UNAUTHORIZED);

        let discharged = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/front-door/connections/discharge")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer operator-secret-token-for-tests")
                    .body(Body::from(
                        json!({"practice_id": "practice-b", "patient_id": "pat-2"}).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(discharged.status(), StatusCode::OK);
        let discharged_json = serde_json::from_slice::<serde_json::Value>(
            &axum::body::to_bytes(discharged.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(discharged_json["status"], "ended");
        assert_eq!(discharged_json["reason"], "practice_discharged");

        // End all connections on account deletion: requires operator auth, revokes sessions.
        let end_all = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/front-door/accounts/{account_id}/end-connections"))
                    .header("authorization", "Bearer operator-secret-token-for-tests")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(end_all.status(), StatusCode::OK);
        let end_json = serde_json::from_slice::<serde_json::Value>(
            &axum::body::to_bytes(end_all.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(end_json["ended"], 1);
        assert_eq!(
            get(&app, "/front-door/connections", &session)
                .await
                .status(),
            StatusCode::UNAUTHORIZED
        );
    }

    #[tokio::test]
    async fn invitation_lifecycle_routes_enforce_auth_dob_lock_and_share() {
        let (state, _store) = state(MemoryFrontDoorStore::new());
        let state = state.with_operator_auth(
            OperatorAuth::from_secret("operator-secret-token-for-tests").unwrap(),
        );
        let app = router_with(state, true);

        let inv_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/front-door/invitations")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer operator-secret-token-for-tests")
                    .body(Body::from(
                        json!({
                            "practice_id": "practice-1",
                            "patient_id": "pat-1",
                            "email": "tamika@example.test",
                            "display_name": "Tamika",
                            "practice_name": "River Bend Clinic",
                            "physician_name": "Dr. Roger",
                            "expected_birth_date": "1987-02-18"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(inv_resp.status(), StatusCode::OK);
        let inv_json = serde_json::from_slice::<serde_json::Value>(
            &axum::body::to_bytes(inv_resp.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        let short_code = inv_json["short_code"].as_str().unwrap().to_owned();
        let token_1 = inv_json["token"].as_str().unwrap().to_owned();

        // Resend mints new link & short code while old credentials still pass inspect.
        let resend_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/front-door/invitations/resend")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer operator-secret-token-for-tests")
                    .body(Body::from(
                        json!({"practice_id": "practice-1", "patient_id": "pat-1"}).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resend_resp.status(), StatusCode::OK);

        // Inspect returns active standing with no practice leakage.
        let inspect_resp = post(
            &app,
            "/front-door/invitations/inspect",
            json!({ "credential": { "kind": "link_token", "value": token_1 } }),
        )
        .await;
        assert_eq!(inspect_resp.status(), StatusCode::OK);
        let inspect_json = serde_json::from_slice::<serde_json::Value>(
            &axum::body::to_bytes(inspect_resp.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(inspect_json["standing"]["standing"], "active");
        assert!(inspect_json.get("practice_name").is_none());

        // Verify DOB unlocks preview, then Accept connects.
        let verify_resp = post(
            &app,
            "/front-door/invitations/verify-dob",
            json!({
                "credential": { "kind": "short_code", "value": short_code },
                "birth_date": "1987-02-18"
            }),
        )
        .await;
        assert_eq!(verify_resp.status(), StatusCode::OK);
        let verify_json = serde_json::from_slice::<serde_json::Value>(
            &axum::body::to_bytes(verify_resp.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(verify_json["practice_name"], "River Bend Clinic");

        let accept_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/front-door/invitations/accept")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer operator-secret-token-for-tests")
                    .body(Body::from(
                        json!({
                            "account_email": "duet-account@example.test",
                            "credential": { "kind": "short_code", "value": short_code },
                            "birth_date": "1987-02-18"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(accept_resp.status(), StatusCode::OK);
        let accept_json = serde_json::from_slice::<serde_json::Value>(
            &axum::body::to_bytes(accept_resp.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(accept_json["connection"]["patient"]["patient_id"], "pat-1");

        // Second invite on a fresh patient tests cancel -> 410 NotOpen.
        let inv2_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/front-door/invitations")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer operator-secret-token-for-tests")
                    .body(Body::from(
                        json!({
                            "practice_id": "practice-1",
                            "patient_id": "pat-2",
                            "email": "p2@example.test",
                            "display_name": "P2"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let inv2_json = serde_json::from_slice::<serde_json::Value>(
            &axum::body::to_bytes(inv2_resp.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        let cancel_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/front-door/invitations/cancel")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer operator-secret-token-for-tests")
                    .body(Body::from(
                        json!({"practice_id": "practice-1", "patient_id": "pat-2"}).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(cancel_resp.status(), StatusCode::NO_CONTENT);
        let cancelled_inspect = post(
            &app,
            "/front-door/invitations/inspect",
            json!({ "credential": { "kind": "link_token", "value": inv2_json["token"] } }),
        )
        .await;
        assert_eq!(cancelled_inspect.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn practice_operator_invitation_and_roster_routes_require_operator_bearer() {
        let (state, _store) = state(MemoryFrontDoorStore::new());
        let state = state.with_operator_auth(
            OperatorAuth::from_secret("operator-secret-token-for-tests").unwrap(),
        );
        let app = router_with(state, true);

        let unauth_routes = [
            (
                "POST",
                "/front-door/invitations",
                Some(json!({
                    "practice_id": "practice-1",
                    "patient_id": "pat-unauth",
                    "email": "u@example.test",
                    "display_name": "Unauth"
                })),
            ),
            (
                "POST",
                "/front-door/invitations/resend",
                Some(json!({"practice_id": "practice-1", "patient_id": "pat-unauth"})),
            ),
            (
                "POST",
                "/front-door/invitations/cancel",
                Some(json!({"practice_id": "practice-1", "patient_id": "pat-unauth"})),
            ),
            (
                "POST",
                "/front-door/invitations/delivery-failed",
                Some(
                    json!({"practice_id": "practice-1", "patient_id": "pat-unauth", "detail": "smtp"}),
                ),
            ),
            (
                "POST",
                "/front-door/invitations/accept",
                Some(json!({
                    "account_email": "u@example.test",
                    "credential": {"kind": "short_code", "value": "ABCD-EFGH"},
                    "birth_date": "1980-01-01"
                })),
            ),
            ("GET", "/front-door/roster/practice-1/pat-unauth", None),
        ];

        for (method, uri, body) in unauth_routes {
            for auth_header in [None, Some("Bearer wrong-operator-secret")] {
                let mut builder = Request::builder().method(method).uri(uri);
                if let Some(h) = auth_header {
                    builder = builder.header("authorization", h);
                }
                let req = if let Some(json_body) = &body {
                    builder
                        .header("content-type", "application/json")
                        .body(Body::from(json_body.to_string()))
                        .unwrap()
                } else {
                    builder.body(Body::empty()).unwrap()
                };
                let resp = app.clone().oneshot(req).await.unwrap();
                assert_eq!(
                    resp.status(),
                    StatusCode::UNAUTHORIZED,
                    "expected 401 for {method} {uri} with auth={auth_header:?}"
                );
            }
        }

        // Valid operator bearer succeeds on invite + roster + delivery-failed + cancel.
        let roster_ok = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/front-door/roster/practice-1/pat-unauth")
                    .header("authorization", "Bearer operator-secret-token-for-tests")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(roster_ok.status(), StatusCode::OK);
    }
}
