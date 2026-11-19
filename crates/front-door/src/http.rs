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
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, put};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::{
    Account, Consent, ForMyDoctorError, ForMyDoctorSendRequest, ForMyDoctorShare,
    ForMyDoctorShareStatus, FrontDoorStore, Invitation, InviteError, LinkError, Premium,
    RosterPatient, SessionError, SharedPatientRecordInput, SignInError,
};

/// Why a route answered with an error status.
#[derive(Debug)]
pub enum ApiError {
    /// The presented bearer token resolves to no active account.
    Unauthorized,
    /// The invitation or token is unknown, used, or expired.
    Invalid,
    /// The practice already invited this email, or the email holds an account.
    Conflict,
    /// A `"For my doctor"` share key was reused with different content.
    ShareConflict,
    /// Sharing consent is missing or withdrawn.
    ConsentRequired,
    /// Supplied connected-care share/record payload is malformed.
    InvalidInput,
    /// The account carries no link to a roster patient.
    Unlinked,
    /// The store failed; the caller cannot fix it by retrying differently.
    Storage,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, detail) = match self {
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Sign in to read the account."),
            Self::Invalid => (
                StatusCode::UNAUTHORIZED,
                "That invitation or sign-in link is no longer valid.",
            ),
            Self::Conflict => (
                StatusCode::CONFLICT,
                "That email is already invited or has an account.",
            ),
            Self::ShareConflict => (
                StatusCode::CONFLICT,
                "That For my doctor share key already exists with different content.",
            ),
            Self::ConsentRequired => (
                StatusCode::PRECONDITION_REQUIRED,
                "Turn on sharing consent before sending to your doctor.",
            ),
            Self::InvalidInput => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "Supply non-empty share details.",
            ),
            Self::Unlinked => (StatusCode::NOT_FOUND, "No linked patient for this account."),
            Self::Storage => (
                StatusCode::SERVICE_UNAVAILABLE,
                "The front door is unavailable.",
            ),
        };
        (status, Json(json!({ "detail": detail }))).into_response()
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
        }
    }
}

impl From<LinkError> for ApiError {
    fn from(error: LinkError) -> Self {
        match error {
            LinkError::Unlinked => Self::Unlinked,
        }
    }
}

impl From<ForMyDoctorError> for ApiError {
    fn from(error: ForMyDoctorError) -> Self {
        match error {
            ForMyDoctorError::Unlinked | ForMyDoctorError::NotFound => Self::Unlinked,
            ForMyDoctorError::ConsentRequired => Self::ConsentRequired,
            ForMyDoctorError::InvalidInput(_) => Self::InvalidInput,
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
    /// The linked roster patient, when the account carries one.
    pub link: Option<RosterPatient>,
}

impl From<Account> for AccountView {
    fn from(account: Account) -> Self {
        Self {
            account_id: account.id,
            email: account.email,
            display_name: account.display_name,
            link: account.link,
        }
    }
}

/// The consent state as the routes return it.
#[derive(serde::Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ConsentView {
    /// Sharing is on.
    Granted {
        /// The Unix second consent was given.
        at: u64,
    },
    /// Consent was withdrawn; what was shared stays dated evidence.
    Withdrawn {
        /// The Unix second consent was given.
        at: u64,
        /// The Unix second consent was withdrawn.
        withdrawn_at: u64,
    },
    /// The patient was never asked, or never answered.
    None,
}

impl From<Consent> for ConsentView {
    fn from(consent: Consent) -> Self {
        match consent {
            Consent::Granted { at } => Self::Granted { at },
            Consent::Withdrawn { at, withdrawn_at } => Self::Withdrawn { at, withdrawn_at },
            Consent::None => Self::None,
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
    /// The Unix second the invitation stops redeeming.
    pub expires_at: u64,
    /// The redeem token, returned to the caller here; a real deployment
    /// delivers it to the patient instead.
    pub token: String,
}

impl InvitationView {
    fn from_pending(invitation: Invitation, token: String) -> Self {
        Self {
            patient: invitation.patient,
            email: invitation.email,
            display_name: invitation.display_name,
            expires_at: invitation.expires_at,
            token,
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

/// The state the routes share.
#[derive(Clone)]
pub struct FrontDoorState {
    /// The durable store.
    pub store: Arc<dyn FrontDoorStore>,
    /// Unix seconds now, injected so tests control time.
    pub now: Arc<dyn Fn() -> u64 + Send + Sync>,
}

impl FrontDoorState {
    /// State on one store, reading the wall clock for `now`.
    pub fn new(store: Arc<dyn FrontDoorStore>) -> Self {
        Self {
            store,
            now: Arc::new(wall_now),
        }
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
        .route("/front-door/invitations/redeem", post(redeem))
        .route("/front-door/sign-in", post(sign_in))
        .route("/front-door/sign-in/verify", post(verify))
        .route("/front-door/account", get(account))
        .route("/front-door/link", get(link))
        .route(
            "/front-door/consent",
            get(consent).post(grant_consent).delete(withdraw_consent),
        )
        .route("/front-door/premium", get(premium))
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

/// Records one invitation to a roster patient and returns the redeem token.
async fn invite(
    State((state, _)): State<(FrontDoorState, bool)>,
    Json(body): Json<InviteBody>,
) -> Result<Json<InvitationView>, ApiError> {
    let patient = RosterPatient {
        practice_id: body.practice_id,
        patient_id: body.patient_id,
    };
    let token = state
        .store
        .invite(&patient, &body.email, &body.display_name, (state.now)())
        .await?;
    let invitation = state
        .store
        .invitation(&body.email)
        .await
        .ok_or(ApiError::Storage)?;
    Ok(Json(InvitationView::from_pending(invitation, token)))
}

/// Redeems an invitation, creating the account and minting one session.
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
    // The token itself is the response's proof; the account id lands on verify.
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

/// Resolves the presented session's link to its roster patient.
async fn link(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
) -> Result<Json<RosterPatient>, ApiError> {
    let account_id = account_id(&state, require_auth, &headers).await?;
    Ok(Json(state.store.link(account_id).await?))
}

/// Reads the presented session's consent state.
async fn consent(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
) -> Result<Json<ConsentView>, ApiError> {
    let account_id = account_id(&state, require_auth, &headers).await?;
    Ok(Json(state.store.consent(account_id).await?.into()))
}

/// Records the presented session's consent to share.
async fn grant_consent(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
) -> Result<StatusCode, ApiError> {
    let account_id = account_id(&state, require_auth, &headers).await?;
    state.store.grant_consent(account_id, (state.now)()).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Withdraws the presented session's consent; one step.
async fn withdraw_consent(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
) -> Result<StatusCode, ApiError> {
    let account_id = account_id(&state, require_auth, &headers).await?;
    state
        .store
        .withdraw_consent(account_id, (state.now)())
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Reads the presented session's linked patient's premium status. Duet
/// reads only which premium features are on.
async fn premium(
    State((state, require_auth)): State<(FrontDoorState, bool)>,
    headers: HeaderMap,
) -> Result<Json<Premium>, ApiError> {
    let account_id = account_id(&state, require_auth, &headers).await?;
    let patient = state.store.link(account_id).await?;
    Ok(Json(state.store.premium(&patient).await))
}

#[derive(serde::Serialize)]
struct ForMyDoctorShareView {
    share_id: String,
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
    use crate::{INVITATION_TTL_SECONDS, MemoryFrontDoorStore};
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

    /// Invites and redeems through the router, returning the bearer session.
    async fn invited_session(app: &Router, _store: &MemoryFrontDoorStore) -> String {
        let response = post(
            app,
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
        assert_eq!(
            body["expires_at"].as_u64().unwrap(),
            NOW + INVITATION_TTL_SECONDS
        );
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
    async fn a_conflicting_invitation_answers_conflict() {
        let (state, store) = state(MemoryFrontDoorStore::new());
        let app = router_with(state, false);
        // The email already holds a front-door account.
        store.seed_linked_account(
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
                "patient_id": "patient-2",
                "email": "LUNA@example.test",
                "display_name": "Luna Park",
            }),
        )
        .await;
        assert_eq!(
            conflict.status(),
            StatusCode::CONFLICT,
            "the practice cannot invite an email that already holds an account"
        );
    }

    #[tokio::test]
    async fn the_account_read_resolves_the_session_bearer() {
        let (state, store) = state(MemoryFrontDoorStore::new());
        let app = router_with(state, true);
        let session = invited_session(&app, &store).await;
        let response = get(&app, "/front-door/account", &session).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(body["email"], "luna@example.test");
        assert_eq!(body["link"]["practice_id"], "practice");
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
    async fn the_link_and_consent_and_premium_reads_follow_the_session() {
        let (state, store) = state(MemoryFrontDoorStore::new());
        let app = router_with(state, true);
        let session = invited_session(&app, &store).await;
        let link = get(&app, "/front-door/link", &session).await;
        assert_eq!(link.status(), StatusCode::OK);
        let consent = get(&app, "/front-door/consent", &session).await;
        assert_eq!(consent.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(consent.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(body["state"], "none", "consent is granted, never assumed");
        assert_eq!(
            app.clone()
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/front-door/consent")
                        .header("authorization", format!("Bearer {session}"))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap()
                .status(),
            StatusCode::NO_CONTENT
        );
        let premium = get(&app, "/front-door/premium", &session).await;
        assert_eq!(premium.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(premium.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(body["premium"], false);
        assert!(body.get("cadence").is_none());
    }

    #[tokio::test]
    async fn withdrawal_answers_no_content_and_stays_one_step() {
        let (state, store) = state(MemoryFrontDoorStore::new());
        let app = router_with(state, true);
        let session = invited_session(&app, &store).await;
        // The grant that the withdrawal withdraws.
        assert_eq!(
            app.clone()
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/front-door/consent")
                        .header("authorization", format!("Bearer {session}"))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap()
                .status(),
            StatusCode::NO_CONTENT
        );
        assert_eq!(
            app.clone()
                .oneshot(
                    Request::builder()
                        .method("DELETE")
                        .uri("/front-door/consent")
                        .header("authorization", format!("Bearer {session}"))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap()
                .status(),
            StatusCode::NO_CONTENT,
            "stopping sharing is one step"
        );
        // The grant second is preserved through the withdrawal.
        let consent = get(&app, "/front-door/consent", &session).await;
        assert_eq!(consent.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(consent.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(body["state"], "withdrawn");
    }

    #[tokio::test]
    async fn sign_in_issues_and_verifies_one_token() {
        let (state, store) = state(MemoryFrontDoorStore::new());
        let app = router_with(state, true);
        let session = invited_session(&app, &store).await;
        let _ = session;
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
    async fn for_my_doctor_route_requires_consent_and_tracks_sent_status() {
        let (state, store) = state(MemoryFrontDoorStore::new());
        let app = router_with(state, true);
        let session = invited_session(&app, &store).await;
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
                            "share_key": "share-1",
                            "title": "Pre-visit summary",
                            "summary_text": "Home blood pressure averages 138/86."
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(denied.status(), StatusCode::PRECONDITION_REQUIRED);

        // Grant consent -> send works -> idempotent resend -> status shows "sent" -> practice marks "received".
        let consent_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/front-door/consent")
                    .header("authorization", format!("Bearer {session}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(consent_resp.status(), StatusCode::NO_CONTENT);

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
    }
}
