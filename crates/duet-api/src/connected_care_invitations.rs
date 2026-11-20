//! Duet Connected Care invitation inspection, DOB check, and one-tap `Share` routes
//! (`intent/connected-care.md` C1.1–C1.2, `COD-53`).
//!
//! Enforces that:
//! - inspecting an invitation reveals zero practice/physician/sharing details before DOB (`Rule 6`);
//! - entering wrong birth dates increments the practice invitation strike counter atomically
//!   and locks at 5 (`COD-48`, `Rule 7`);
//! - patients under 18 are refused (`Rule 22`);
//! - checking DOB grants no sharing consent (`Rule 3`); only `POST .../share` connects the
//!   currently signed-in Duet account (`Rule 2`, `Rule 9`).

use std::fmt;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use front_door::{
    AccountConnection, FrontDoorStore, InvitationAcceptError, InvitationCheckError,
    InvitationCredential, InvitationGateState, InvitationSharingPreview,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryTrait};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::ApiError;
use crate::database::orm;
use crate::durable::DurableState;
use crate::identity::{AccessLevel, AccountContext, check_current};

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

/// Transport reaching `front-door` invitation endpoints (direct Postgres store or internal HTTP).
#[derive(Clone)]
pub enum FrontDoorInvitationPort {
    /// Direct in-process `FrontDoorStore` (when `FRONT_DOOR_DATABASE_URL` or test store is used).
    Direct(Arc<dyn FrontDoorStore>),
    /// Internal HTTP client to `front-door` (when `FRONT_DOOR_URL` + operator token are used).
    Http(HttpFrontDoorInvitations),
}

impl fmt::Debug for FrontDoorInvitationPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Direct(_) => f.write_str("FrontDoorInvitationPort::Direct(..)"),
            Self::Http(http) => f
                .debug_tuple("FrontDoorInvitationPort::Http")
                .field(http)
                .finish(),
        }
    }
}

/// Hardened HTTP client calling `front-door` invitation endpoints.
#[derive(Clone)]
pub struct HttpFrontDoorInvitations {
    client: reqwest::Client,
    base_url: String,
    operator_token: String,
}

impl fmt::Debug for HttpFrontDoorInvitations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpFrontDoorInvitations")
            .field("base_url", &self.base_url)
            .field("operator_token", &"<redacted>")
            .finish()
    }
}

impl HttpFrontDoorInvitations {
    /// Validates `base_url` (no userinfo) and non-empty operator secret with redirects disabled.
    pub fn new(
        base_url: impl Into<String>,
        operator_token: impl Into<String>,
    ) -> Result<Self, ApiError> {
        let base_url = base_url.into().trim().trim_end_matches('/').to_owned();
        let parsed = reqwest::Url::parse(&base_url)
            .map_err(|_| ApiError::Internal("invalid front-door base URL"))?;
        if !matches!(parsed.scheme(), "http" | "https")
            || !parsed.username().is_empty()
            || parsed.password().is_some()
        {
            return Err(ApiError::Internal("invalid front-door base URL"));
        }
        let operator_token = operator_token.into().trim().to_owned();
        if operator_token.is_empty() {
            return Err(ApiError::Internal("missing front-door operator token"));
        }
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|_| ApiError::Internal("front-door invitation HTTP client build failed"))?;
        Ok(Self {
            client,
            base_url,
            operator_token,
        })
    }
}

/// Router state for `/api/connected-care/invitations/*` and `/api/connected-care/connections`.
#[derive(Clone)]
pub struct ConnectedCareInvitationState {
    /// Duet durable account, auth, and database owners.
    pub durable: DurableState,
    /// Optional configured front-door invitation port (`None` fails closed with 503).
    pub front_door: Option<FrontDoorInvitationPort>,
}

/// Mounts Duet Connected Care invitation & connection routes.
pub fn router(state: ConnectedCareInvitationState) -> Router {
    Router::new()
        .route("/api/connected-care/invitations/inspect", post(inspect))
        .route(
            "/api/connected-care/invitations/verify-dob",
            post(verify_dob),
        )
        .route("/api/connected-care/invitations/share", post(share))
        .route("/api/connected-care/connections", get(connections))
        .with_state(state)
}

#[derive(Debug, Deserialize)]
struct InspectRequest {
    credential: InvitationCredential,
}

#[derive(Debug, Deserialize)]
struct DobRequest {
    credential: InvitationCredential,
    birth_date: String,
}

#[derive(Debug, Serialize)]
struct ShareResponse {
    front_door_account_id: Uuid,
    connection: AccountConnection,
}

#[derive(Debug)]
enum InvitationApiError {
    Api(ApiError),
    NotOpen,
    Locked,
    WrongBirthDate { attempts_remaining: u8 },
    Underage,
    InvalidBirthDate,
    InactiveAccount,
    Conflict,
    Unavailable(String),
}

impl From<ApiError> for InvitationApiError {
    fn from(value: ApiError) -> Self {
        Self::Api(value)
    }
}

impl From<InvitationCheckError> for InvitationApiError {
    fn from(value: InvitationCheckError) -> Self {
        match value {
            InvitationCheckError::NotOpen => Self::NotOpen,
            InvitationCheckError::Locked => Self::Locked,
            InvitationCheckError::WrongBirthDate { attempts_remaining } => {
                Self::WrongBirthDate { attempts_remaining }
            }
            InvitationCheckError::Underage => Self::Underage,
            InvitationCheckError::InvalidBirthDate => Self::InvalidBirthDate,
            InvitationCheckError::Storage(err) => Self::Unavailable(err.to_string()),
        }
    }
}

impl From<InvitationAcceptError> for InvitationApiError {
    fn from(value: InvitationAcceptError) -> Self {
        match value {
            InvitationAcceptError::Check(err) => err.into(),
            InvitationAcceptError::InactiveAccount => Self::InactiveAccount,
            InvitationAcceptError::Conflict => Self::Conflict,
            InvitationAcceptError::Storage(err) => Self::Unavailable(err.to_string()),
        }
    }
}

impl IntoResponse for InvitationApiError {
    fn into_response(self) -> Response {
        match self {
            Self::Api(err) => err.into_response(),
            Self::NotOpen => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "code": "invitation_not_open",
                    "detail": "This invitation link or code is not open.",
                })),
            )
                .into_response(),
            Self::Locked => (
                StatusCode::LOCKED,
                Json(json!({
                    "code": "invitation_locked",
                    "detail": "Too many wrong birth-date answers locked this invitation. Ask your practice to cancel and send a new invitation.",
                })),
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
                Json(json!({
                    "code": "underage",
                    "detail": "Duet Connected Care requires being 18 or older.",
                })),
            )
                .into_response(),
            Self::InvalidBirthDate => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "code": "invalid_birth_date",
                    "detail": "Enter a valid date of birth (YYYY-MM-DD).",
                })),
            )
                .into_response(),
            Self::InactiveAccount => (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "code": "inactive_account",
                    "detail": "This Connected Care account is inactive.",
                })),
            )
                .into_response(),
            Self::Conflict => (
                StatusCode::CONFLICT,
                Json(json!({
                    "code": "connection_conflict",
                    "detail": "This roster patient is already connected.",
                })),
            )
                .into_response(),
            Self::Unavailable(detail) => {
                tracing::warn!(%detail, "front-door invitation upstream unavailable");
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(json!({
                        "code": "front_door_unavailable",
                        "detail": "Connected Care is unavailable right now.",
                    })),
                )
                    .into_response()
            }
        }
    }
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[derive(Deserialize)]
struct FrontDoorErrorPayload {
    #[serde(default)]
    code: String,
    #[serde(default)]
    attempts_remaining: Option<u8>,
    #[serde(default)]
    detail: Option<String>,
}

async fn decode_http_error(resp: reqwest::Response) -> InvitationApiError {
    let status = resp.status();
    let payload = resp.json::<FrontDoorErrorPayload>().await.ok();
    let code = payload.as_ref().map(|p| p.code.as_str()).unwrap_or("");
    match (status, code) {
        (StatusCode::NOT_FOUND, "not_open") | (StatusCode::NOT_FOUND, "") => {
            InvitationApiError::NotOpen
        }
        (StatusCode::LOCKED, _) => InvitationApiError::Locked,
        (StatusCode::FORBIDDEN, "underage") => InvitationApiError::Underage,
        (StatusCode::FORBIDDEN, "inactive_account") => InvitationApiError::InactiveAccount,
        (StatusCode::UNPROCESSABLE_ENTITY, "wrong_birth_date") => {
            InvitationApiError::WrongBirthDate {
                attempts_remaining: payload.and_then(|p| p.attempts_remaining).unwrap_or(0),
            }
        }
        (StatusCode::UNPROCESSABLE_ENTITY, _) => InvitationApiError::InvalidBirthDate,
        (StatusCode::CONFLICT, _) => InvitationApiError::Conflict,
        _ => InvitationApiError::Unavailable(
            payload
                .and_then(|p| p.detail)
                .unwrap_or_else(|| format!("front-door returned {status}")),
        ),
    }
}

impl FrontDoorInvitationPort {
    async fn inspect(
        &self,
        credential: &InvitationCredential,
    ) -> Result<InvitationGateState, InvitationApiError> {
        match self {
            Self::Direct(store) => Ok(store.inspect_invitation(credential).await?),
            Self::Http(http) => {
                let resp = http
                    .client
                    .post(format!("{}/front-door/invitations/inspect", http.base_url))
                    .json(&json!({ "credential": credential }))
                    .send()
                    .await
                    .map_err(|err| InvitationApiError::Unavailable(err.to_string()))?;
                if !resp.status().is_success() {
                    return Err(decode_http_error(resp).await);
                }
                resp.json::<InvitationGateState>()
                    .await
                    .map_err(|err| InvitationApiError::Unavailable(err.to_string()))
            }
        }
    }

    async fn verify_dob(
        &self,
        credential: &InvitationCredential,
        birth_date: &str,
        now: u64,
    ) -> Result<InvitationSharingPreview, InvitationApiError> {
        match self {
            Self::Direct(store) => Ok(store
                .verify_invitation_dob(credential, birth_date, now)
                .await?),
            Self::Http(http) => {
                let resp = http
                    .client
                    .post(format!(
                        "{}/front-door/invitations/verify-dob",
                        http.base_url
                    ))
                    .json(&json!({ "credential": credential, "birth_date": birth_date }))
                    .send()
                    .await
                    .map_err(|err| InvitationApiError::Unavailable(err.to_string()))?;
                if !resp.status().is_success() {
                    return Err(decode_http_error(resp).await);
                }
                resp.json::<InvitationSharingPreview>()
                    .await
                    .map_err(|err| InvitationApiError::Unavailable(err.to_string()))
            }
        }
    }

    async fn accept_share(
        &self,
        account_id: Option<Uuid>,
        account_email: Option<&str>,
        credential: &InvitationCredential,
        birth_date: &str,
        now: u64,
    ) -> Result<(Uuid, AccountConnection), InvitationApiError> {
        match self {
            Self::Direct(store) => Ok(store
                .accept_invitation_share(account_id, account_email, credential, birth_date, now)
                .await?),
            Self::Http(http) => {
                #[derive(Deserialize)]
                struct AcceptResponse {
                    account_id: Uuid,
                    connection: AccountConnection,
                }
                let resp = http
                    .client
                    .post(format!("{}/front-door/invitations/accept", http.base_url))
                    .bearer_auth(&http.operator_token)
                    .json(&json!({
                        "account_id": account_id,
                        "account_email": account_email,
                        "credential": credential,
                        "birth_date": birth_date,
                    }))
                    .send()
                    .await
                    .map_err(|err| InvitationApiError::Unavailable(err.to_string()))?;
                if !resp.status().is_success() {
                    return Err(decode_http_error(resp).await);
                }
                let ok = resp
                    .json::<AcceptResponse>()
                    .await
                    .map_err(|err| InvitationApiError::Unavailable(err.to_string()))?;
                Ok((ok.account_id, ok.connection))
            }
        }
    }

    async fn connections(
        &self,
        account_id: Uuid,
    ) -> Result<Vec<AccountConnection>, InvitationApiError> {
        match self {
            Self::Direct(store) => match store.connections(account_id).await {
                Ok(list) => Ok(list),
                Err(front_door::LinkError::Unlinked) => Ok(Vec::new()),
                Err(front_door::LinkError::Storage(err)) => {
                    Err(InvitationApiError::Unavailable(err.to_string()))
                }
            },
            Self::Http(http) => {
                let resp = http
                    .client
                    .get(format!(
                        "{}/front-door/accounts/{account_id}/connections",
                        http.base_url
                    ))
                    .bearer_auth(&http.operator_token)
                    .send()
                    .await
                    .map_err(|err| InvitationApiError::Unavailable(err.to_string()))?;
                if !resp.status().is_success() {
                    return Err(decode_http_error(resp).await);
                }
                resp.json::<Vec<AccountConnection>>()
                    .await
                    .map_err(|err| InvitationApiError::Unavailable(err.to_string()))
            }
        }
    }
}

async fn current_front_door_mapping(
    state: &ConnectedCareInvitationState,
    context: &AccountContext,
    level: AccessLevel,
) -> Result<Option<Uuid>, ApiError> {
    let mut tx = state
        .durable
        .identity
        .database
        .begin(context.account_id)
        .await?;
    check_current(&mut tx, context, level).await?;
    let mapped = orm::model::<front_door_mapping::Model>(
        &mut tx,
        &front_door_mapping::Entity::find()
            .filter(front_door_mapping::Column::UserId.eq(context.account_id))
            .into_query(),
    )
    .await?;
    tx.commit().await?;
    Ok(mapped.map(|row| row.front_door_account_id))
}

fn require_port(
    state: &ConnectedCareInvitationState,
) -> Result<&FrontDoorInvitationPort, InvitationApiError> {
    state.front_door.as_ref().ok_or_else(|| {
        InvitationApiError::Unavailable("front-door is not configured for this server".to_owned())
    })
}

async fn inspect(
    State(state): State<ConnectedCareInvitationState>,
    headers: HeaderMap,
    Json(body): Json<InspectRequest>,
) -> Result<Json<InvitationGateState>, InvitationApiError> {
    let context = state.durable.authenticate(&headers).await?;
    let _ = current_front_door_mapping(&state, &context, AccessLevel::RightsOnly).await?;
    let port = require_port(&state)?;
    Ok(Json(port.inspect(&body.credential).await?))
}

async fn verify_dob(
    State(state): State<ConnectedCareInvitationState>,
    headers: HeaderMap,
    Json(body): Json<DobRequest>,
) -> Result<Json<InvitationSharingPreview>, InvitationApiError> {
    let context = state.durable.authenticate(&headers).await?;
    let _ = current_front_door_mapping(&state, &context, AccessLevel::RightsOnly).await?;
    let port = require_port(&state)?;
    Ok(Json(
        port.verify_dob(&body.credential, &body.birth_date, unix_now())
            .await?,
    ))
}

async fn share(
    State(state): State<ConnectedCareInvitationState>,
    headers: HeaderMap,
    Json(body): Json<DobRequest>,
) -> Result<Json<ShareResponse>, InvitationApiError> {
    let context = state.durable.authenticate(&headers).await?;
    let existing_mapping =
        current_front_door_mapping(&state, &context, AccessLevel::AcceptedTerms).await?;
    let profile_email = if existing_mapping.is_none() {
        let profile = state
            .durable
            .identity
            .email_auth()
            .read_profile(&context)
            .await?;
        Some(
            profile
                .email
                .unwrap_or_else(|| format!("account-{}@duet.local", context.account_id)),
        )
    } else {
        None
    };

    let port = require_port(&state)?;
    let (front_door_account_id, connection) = port
        .accept_share(
            existing_mapping,
            profile_email.as_deref(),
            &body.credential,
            &body.birth_date,
            unix_now(),
        )
        .await?;

    if existing_mapping.is_none() {
        state
            .durable
            .identity
            .map_front_door(&context, front_door_account_id)
            .await?;
    }

    Ok(Json(ShareResponse {
        front_door_account_id,
        connection,
    }))
}

async fn connections(
    State(state): State<ConnectedCareInvitationState>,
    headers: HeaderMap,
) -> Result<Json<Vec<AccountConnection>>, InvitationApiError> {
    let context = state.durable.authenticate(&headers).await?;
    let Some(front_door_account_id) =
        current_front_door_mapping(&state, &context, AccessLevel::RightsOnly).await?
    else {
        return Ok(Json(Vec::new()));
    };
    let port = require_port(&state)?;
    Ok(Json(port.connections(front_door_account_id).await?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use front_door::{
        InvitePatientParams, MemoryFrontDoorStore, RosterPatient,
        http::{FrontDoorState, OperatorAuth, router_with},
    };
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn http_port_preserves_dob_strike_lock_and_accept_without_creating_duplicate_account() {
        let mem = Arc::new(MemoryFrontDoorStore::new());
        let now = 1_760_000_000;
        let issued = mem
            .invite_patient(InvitePatientParams {
                patient: RosterPatient {
                    practice_id: "prac-1".into(),
                    patient_id: "pat-1".into(),
                },
                email: "chart@example.test".into(),
                display_name: "Tamika".into(),
                practice_name: "River Bend Clinic".into(),
                physician_name: "Dr. Roger".into(),
                expected_birth_date: Some("1987-02-18".into()),
                now,
            })
            .await
            .unwrap();

        let fd_state = FrontDoorState::new(mem.clone()).with_operator_auth(
            OperatorAuth::from_secret("operator-secret-token-for-tests").unwrap(),
        );
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, router_with(fd_state, true))
                .await
                .unwrap();
        });

        let port = FrontDoorInvitationPort::Http(
            HttpFrontDoorInvitations::new(
                format!("http://{addr}"),
                "operator-secret-token-for-tests",
            )
            .unwrap(),
        );
        let cred = InvitationCredential::ShortCode(issued.short_code);

        let gate = port.inspect(&cred).await.unwrap();
        assert!(matches!(
            gate.standing,
            front_door::InvitationOpenStanding::Active {
                attempts_remaining: 5,
                ..
            }
        ));

        let wrong = port
            .verify_dob(&cred, "1987-02-19", now + 1)
            .await
            .unwrap_err();
        assert!(matches!(
            wrong,
            InvitationApiError::WrongBirthDate {
                attempts_remaining: 4
            }
        ));

        let preview = port.verify_dob(&cred, "1987-02-18", now + 2).await.unwrap();
        assert_eq!(preview.practice_name, "River Bend Clinic");

        let (fd_id, conn) = port
            .accept_share(
                None,
                Some("different@duet.test"),
                &cred,
                "1987-02-18",
                now + 3,
            )
            .await
            .unwrap();
        assert_eq!(conn.patient.practice_id, "prac-1");
        assert_eq!(port.connections(fd_id).await.unwrap().len(), 1);
    }
}
