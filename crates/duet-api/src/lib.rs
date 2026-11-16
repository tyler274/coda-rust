//! Duet-owned account, privacy, evidence, workspace and publication services.
//! `main.rs` composes the durable route families and recovery workers.
//! This module also holds the front-door join, consent and premium routes,
//! which the binary does not mount.
//! Book: `book/src/backend-architecture.md`.
#![deny(missing_docs)]
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

pub mod background;
#[expect(
    clippy::disallowed_methods,
    reason = "converting raw SQL to the typed database API"
)]
pub mod database;
pub mod doctor_routes;
#[expect(
    clippy::disallowed_methods,
    reason = "converting raw SQL to the typed database API"
)]
pub mod durable;
pub mod evidence;
mod evidence_entity;
pub mod evidence_paging;
pub mod evidence_routes;
mod follow_up_entity;
#[expect(
    clippy::disallowed_methods,
    reason = "converting raw SQL to the typed database API"
)]
pub mod health_context;
mod health_context_entity;
pub mod hosting;
#[expect(
    clippy::disallowed_methods,
    reason = "converting raw SQL to the typed database API"
)]
pub mod identity;
pub mod journal_gcs;
#[expect(
    clippy::disallowed_methods,
    reason = "converting raw SQL to the typed database API"
)]
pub mod privacy;
pub mod privacy_archive;
#[expect(
    clippy::disallowed_methods,
    reason = "converting raw SQL to the typed database API"
)]
pub mod producer_database;
pub mod producer_routes;
#[expect(
    clippy::disallowed_methods,
    reason = "converting raw SQL to the typed database API"
)]
pub mod publication;
pub mod session;
pub mod site;
#[expect(
    clippy::disallowed_methods,
    reason = "converting raw SQL to the typed database API"
)]
pub mod workspace;

use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use front_door::http::{ConsentView, FrontDoorState, SessionView};
use front_door::{FrontDoorStore, LinkError, Premium, SessionError, SignInError};
use uuid::Uuid;

/// The `REQUIRED_MIGRATION` constant.
pub const REQUIRED_MIGRATION: &str = "0120_one_review_bookmark";

#[derive(Debug)]
/// The `ApiError` cases.
pub enum ApiError {
    /// The `Unauthorized` case.
    Unauthorized,
    /// A bearer token failed validation; renders exactly as `Unauthorized`.
    InvalidToken(jsonwebtoken::errors::Error),
    /// A stale or mismatched operation cannot replace newer state.
    Conflict(&'static str),
    /// The bearer was valid, and sign out everywhere ended that session.
    SessionRevoked,
    /// The bearer verified, and its lifetime has ended. The client refreshes
    /// before anything local is removed.
    SessionExpired,
    /// The bearer is valid and this user has no account yet.
    AbsentAccount,
    /// A data fault. It is not a session refusal and removes nothing.
    Internal(&'static str),
    /// The `Forbidden` case.
    Forbidden,
    /// Today stats need the current terms on an existing account.
    TermsRequired,
    /// The `NotFound` case.
    NotFound,
    /// The invitation or token is unknown, used, or expired.
    JoinInvalid,
    /// The account holds no front-door link, so consent cannot exist yet.
    ConsentUnlinked,
    /// The `Unavailable` case.
    Unavailable(&'static str),
    /// Identity provider response decoding failed; never a product session refusal.
    IdentityResponse(reqwest::Error),
    /// The `Storage` case.
    Storage(String),
    /// The presented terms version is not the current one.
    AgreementChanged,
    /// The request body failed the route's validation.
    Unprocessable(&'static str),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, detail) = match self {
            Self::Conflict(message) => (
                StatusCode::CONFLICT,
                serde_json::json!({"code":match message {
                    "source_authority_refused" => message,
                    _ => "stale_state",
                },"message":message}),
            ),
            Self::Unauthorized | Self::InvalidToken(_) => (
                StatusCode::UNAUTHORIZED,
                refusal("session_invalid", "unverified"),
            ),
            Self::SessionRevoked => (
                StatusCode::UNAUTHORIZED,
                refusal("session_revoked", "revoked"),
            ),
            Self::SessionExpired => (
                StatusCode::UNAUTHORIZED,
                refusal("session_expired", "expired"),
            ),
            Self::AbsentAccount => (
                StatusCode::UNAUTHORIZED,
                refusal("account_absent", "absent"),
            ),
            Self::Internal(cause) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                refusal("internal", cause),
            ),
            Self::Forbidden => (
                StatusCode::FORBIDDEN,
                serde_json::json!("Accept the current Duet Terms of Service to continue."),
            ),
            Self::TermsRequired => (
                StatusCode::FORBIDDEN,
                serde_json::json!({
                    "code": "terms_acceptance_required",
                    "message": "Accept the current Duet Terms of Service to continue.",
                }),
            ),
            Self::NotFound => (StatusCode::NOT_FOUND, serde_json::json!("Moment not found")),
            Self::JoinInvalid => (
                StatusCode::UNAUTHORIZED,
                serde_json::json!("That invitation isn't valid, or it has already been used."),
            ),
            Self::ConsentUnlinked => (
                StatusCode::NOT_FOUND,
                serde_json::json!("Join Duet with your practice invitation first."),
            ),
            Self::AgreementChanged => (
                StatusCode::CONFLICT,
                serde_json::json!({
                    "code": "agreement_version_changed",
                    "message": "Update Duet to review the current terms.",
                }),
            ),
            Self::Unprocessable(detail) => {
                (StatusCode::UNPROCESSABLE_ENTITY, serde_json::json!(detail))
            }
            Self::Unavailable(detail) => {
                (StatusCode::SERVICE_UNAVAILABLE, serde_json::json!(detail))
            }
            Self::IdentityResponse(_) => (
                StatusCode::SERVICE_UNAVAILABLE,
                serde_json::json!("Identity provider response unavailable"),
            ),
            Self::Storage(_) => (
                StatusCode::SERVICE_UNAVAILABLE,
                serde_json::json!("Backend database is not configured."),
            ),
        };
        (status, Json(serde_json::json!({ "detail": detail }))).into_response()
    }
}

/// A refusal body names only a code and an opaque id. The cause stays in the log.
fn refusal(code: &'static str, cause: &'static str) -> serde_json::Value {
    let correlation_id = Uuid::new_v4().simple().to_string();
    tracing::warn!(correlation_id, cause, code, "duet session refusal");
    serde_json::json!({ "code": code, "correlation_id": correlation_id })
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        Self::Storage(error.to_string())
    }
}

impl From<SessionError> for ApiError {
    fn from(_: SessionError) -> Self {
        Self::Unauthorized
    }
}

impl From<SignInError> for ApiError {
    fn from(_: SignInError) -> Self {
        Self::JoinInvalid
    }
}

impl From<LinkError> for ApiError {
    fn from(_: LinkError) -> Self {
        Self::ConsentUnlinked
    }
}

/// The front-door side of the Duet API: Duet joins through the front door,
/// keeps the resulting session as its bearer, and reads consent and premium
/// from the front-door store. Duet never stores billing and never sees
/// clinical data beyond what it already has.
#[derive(Clone)]
pub struct DuetApi {
    /// The front-door store and the injectable clock, shared with the
    /// front-door routes this router merges.
    pub front_door: FrontDoorState,
}

impl DuetApi {
    /// One Duet API on one front-door store, reading the wall clock.
    pub fn new(store: Arc<dyn FrontDoorStore>) -> Self {
        Self {
            front_door: FrontDoorState::new(store),
        }
    }
}

/// The `Authorization: Bearer <token>` value, when the header carries one.
fn bearer(headers: &HeaderMap) -> Option<String> {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(str::to_owned)
}

/// Builds the Duet front-door routes (`/api/join`, `/api/consent`,
/// `/api/premium`, and `/front-door/*`) backed by the given front door.
pub fn front_door_router(api: DuetApi) -> Router {
    // The serving Duet binary does not mount this adapter.
    let join_door = api.front_door.clone();
    let read_door = api.front_door.clone();
    let grant_door = api.front_door.clone();
    let withdraw_door = api.front_door.clone();
    let premium_door = api.front_door.clone();
    Router::new()
        .route(
            "/api/join",
            post(move |body: Json<JoinBody>| join(join_door, body)),
        )
        .route(
            "/api/consent",
            get(move |headers: HeaderMap| consent_read(read_door, headers))
                .post(move |headers: HeaderMap| consent_grant(grant_door, headers))
                .delete(move |headers: HeaderMap| consent_withdraw(withdraw_door, headers)),
        )
        .route(
            "/api/premium",
            get(move |headers: HeaderMap| premium_read(premium_door, headers)),
        )
        // The front-door routes on the same origin: redeem and sign-in stay
        // open (the token is the proof), and the account, link, consent, and
        // premium reads resolve the presented bearer session.
        .merge(front_door::http::router_with(api.front_door, true))
}

/// The join request: the invitation email and token the practice delivered.
#[derive(Debug, serde::Deserialize)]
pub struct JoinBody {
    /// The email the invitation names.
    pub email: String,
    /// The redeem token the practice delivered.
    pub token: String,
}

/// Redeems the practice's invitation through the front door: the account is
/// created there, the link is written there, and the minted session becomes
/// Duet's bearer. The route stays open: the invitation token is the proof,
/// so a person whose session expired can still join.
async fn join(
    front_door: FrontDoorState,
    Json(body): Json<JoinBody>,
) -> Result<Json<SessionView>, ApiError> {
    let (account_id, session) = front_door
        .store
        .redeem_invitation(&body.email, &body.token, (front_door.now)())
        .await?;
    Ok(Json(SessionView {
        account_id,
        session,
    }))
}

/// Reads the presented session's consent state.
async fn consent_read(
    front_door: FrontDoorState,
    headers: HeaderMap,
) -> Result<Json<ConsentView>, ApiError> {
    let token = bearer(&headers).ok_or(ApiError::Unauthorized)?;
    let account_id = front_door.store.session(&token, (front_door.now)()).await?;
    Ok(Json(front_door.store.consent(account_id).await?.into()))
}

/// Records the presented session's consent to share with the physician.
async fn consent_grant(
    front_door: FrontDoorState,
    headers: HeaderMap,
) -> Result<StatusCode, ApiError> {
    let token = bearer(&headers).ok_or(ApiError::Unauthorized)?;
    let account_id = front_door.store.session(&token, (front_door.now)()).await?;
    front_door
        .store
        .grant_consent(account_id, (front_door.now)())
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Withdraws the presented session's consent: one step, and what was already
/// shared stays dated evidence.
async fn consent_withdraw(
    front_door: FrontDoorState,
    headers: HeaderMap,
) -> Result<StatusCode, ApiError> {
    let token = bearer(&headers).ok_or(ApiError::Unauthorized)?;
    let account_id = front_door.store.session(&token, (front_door.now)()).await?;
    front_door
        .store
        .withdraw_consent(account_id, (front_door.now)())
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Reads the presented session's linked patient's premium status: Duet reads
/// only which premium features are on, never billing.
async fn premium_read(
    front_door: FrontDoorState,
    headers: HeaderMap,
) -> Result<Json<Premium>, ApiError> {
    let token = bearer(&headers).ok_or(ApiError::Unauthorized)?;
    let account_id = front_door.store.session(&token, (front_door.now)()).await?;
    let patient = front_door.store.link(account_id).await?;
    Ok(Json(front_door.store.premium(&patient).await))
}

/// Reports whether the required migration is applied.
#[expect(
    clippy::disallowed_methods,
    reason = "converting raw SQL to the typed database API"
)]
pub async fn schema_is_ready(pool: &sqlx::PgPool) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar(include_str!("sql/has_migration.sql"))
        .bind(REQUIRED_MIGRATION)
        .fetch_one(pool)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use front_door::MemoryFrontDoorStore;
    use tower::ServiceExt;

    const NOW: u64 = 1_760_000_000;

    #[test]
    fn the_schema_readiness_check_names_the_required_migration() {
        assert_eq!(REQUIRED_MIGRATION, "0120_one_review_bookmark");
    }

    fn front_door_app() -> (Router, Arc<MemoryFrontDoorStore>) {
        let store = Arc::new(MemoryFrontDoorStore::new());
        let mut front_door = FrontDoorState::new(store.clone());
        front_door.now = Arc::new(|| NOW);
        let app = front_door_router(DuetApi { front_door });
        (app, store)
    }

    /// Invites through the front door (the practice side, as ENG-48 will own
    /// it) and posts the join through Duet's route.
    async fn joined(app: &Router, email: &str, patient_id: &str) -> (serde_json::Value, String) {
        let token = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/front-door/invitations")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "practice_id": "roger-synthetic-practice",
                            "patient_id": patient_id,
                            "email": email,
                            "display_name": "Luna Park",
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(token.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(token.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        let redeem = body["token"].as_str().unwrap().to_owned();
        let joined = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/join")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "email": email,
                            "token": redeem,
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(joined.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(joined.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        let session = body["session"].as_str().unwrap().to_owned();
        (body, session)
    }

    async fn request(app: &Router, method: &str, uri: &str, session: Option<&str>) -> Response {
        let mut builder = Request::builder().method(method).uri(uri);
        if let Some(session) = session {
            builder = builder.header("authorization", format!("Bearer {session}"));
        }
        app.clone()
            .oneshot(builder.body(Body::empty()).unwrap())
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn a_joined_patient_holds_a_session_that_reads_the_link() {
        let (app, _store) = front_door_app();
        let (body, session) = joined(&app, "luna@example.test", "patient-1").await;
        assert!(body["account_id"].as_str().is_some());
        assert!(session.starts_with("codaf-"));
        let link = request(&app, "GET", "/front-door/link", Some(&session)).await;
        assert_eq!(link.status(), StatusCode::OK);
        let link: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(link.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(link["practice_id"], "roger-synthetic-practice");
        assert_eq!(link["patient_id"], "patient-1");
    }

    #[tokio::test]
    async fn a_used_or_wrong_invitation_redeems_never_again() {
        let (app, _store) = front_door_app();
        let (_, _) = joined(&app, "luna@example.test", "patient-1").await;
        // A second join on the redeemed invitation is unauthorized.
        let second = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/join")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "email": "luna@example.test",
                            "token": "codaf-reused",
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(second.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn consent_grants_reads_and_withdraws_in_one_step() {
        let (app, _store) = front_door_app();
        let (_, session) = joined(&app, "luna@example.test", "patient-1").await;
        let none = request(&app, "GET", "/api/consent", Some(&session)).await;
        assert_eq!(none.status(), StatusCode::OK);
        let none: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(none.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(none["state"], "none", "consent is granted, never assumed");
        let granted = request(&app, "POST", "/api/consent", Some(&session)).await;
        assert_eq!(granted.status(), StatusCode::NO_CONTENT);
        let granted = request(&app, "GET", "/api/consent", Some(&session)).await;
        let granted: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(granted.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(granted["state"], "granted");
        assert!(granted["at"].as_u64().is_some());
        let (_, other) = joined(&app, "other@example.test", "patient-2").await;
        let state = request(&app, "GET", "/api/consent", Some(&other)).await;
        let state: serde_json::Value =
            serde_json::from_slice(&axum::body::to_bytes(state.into_body(), 4096).await.unwrap())
                .unwrap();
        assert_eq!(state["state"], "none");
        // A second patient's withdrawal acts only on their own link.
        assert_eq!(
            request(&app, "DELETE", "/api/consent", Some(&other))
                .await
                .status(),
            StatusCode::NO_CONTENT
        );
        let original = request(&app, "GET", "/api/consent", Some(&session)).await;
        let original: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(original.into_body(), 4096)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(original["state"], "granted");
        // One step: DELETE ends the sharing, no confirm round trip.
        let withdrawn = request(&app, "DELETE", "/api/consent", Some(&session)).await;
        assert_eq!(withdrawn.status(), StatusCode::NO_CONTENT);
        let withdrawn = request(&app, "GET", "/api/consent", Some(&session)).await;
        let withdrawn: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(withdrawn.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(
            withdrawn["state"], "withdrawn",
            "what was already shared stays dated evidence"
        );
        assert!(withdrawn["at"].as_u64().is_some());
        assert!(withdrawn["withdrawn_at"].as_u64().is_some());
    }

    #[tokio::test]
    async fn consent_follows_the_presented_session_not_a_dev_header() {
        let (app, _store) = front_door_app();
        let (_, _session) = joined(&app, "luna@example.test", "patient-1").await;
        let anonymous = request(&app, "GET", "/api/consent", None).await;
        assert_eq!(anonymous.status(), StatusCode::UNAUTHORIZED);
        let forged = request(&app, "GET", "/api/consent", Some("codaf-forged")).await;
        assert_eq!(forged.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn premium_reads_free_until_the_front_door_says_otherwise() {
        let (app, store) = front_door_app();
        let (_, session) = joined(&app, "luna@example.test", "patient-1").await;
        let free = request(&app, "GET", "/api/premium", Some(&session)).await;
        assert_eq!(free.status(), StatusCode::OK);
        let free: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(free.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(free["premium"], false, "Duet reads only what is on");
        assert_eq!(free["cadence"], "weekly");
        // The front door owns the status; Duet only reads it.
        let luna = store
            .account_by_email("luna@example.test")
            .await
            .unwrap()
            .id;
        let patient = store.link(luna).await.unwrap();
        store.set_premium(&patient, true, NOW).await.unwrap();
        let premium = request(&app, "GET", "/api/premium", Some(&session)).await;
        let premium: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(premium.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(premium["premium"], true);
        assert_eq!(premium["cadence"], "weekly-daily-check");
    }

    #[tokio::test]
    async fn an_unknown_session_cannot_read_premium_or_grant_consent() {
        let (app, _store) = front_door_app();
        let premium = request(&app, "GET", "/api/premium", Some("codaf-forged")).await;
        assert_eq!(premium.status(), StatusCode::UNAUTHORIZED);
        let grant = request(&app, "POST", "/api/consent", Some("codaf-forged")).await;
        assert_eq!(grant.status(), StatusCode::UNAUTHORIZED);
    }
}
