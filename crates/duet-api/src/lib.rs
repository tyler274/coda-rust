//! Axum owner for Duet's moments, account, and session routes. Other patient
//! routers stay on FastAPI until each one moves. The physician Node server is
//! not this crate.
//! The front door is embedded here for Duet: joining redeems a practice
//! invitation, consent shares with the physician through the front-door
//! store, and premium reads name only which features are on.
//! Book: `book/src/backend-architecture.md`.
#![deny(missing_docs)]

pub mod care;
pub mod hosting;
pub mod identity;
pub mod session;
pub mod site;

use std::env;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use axum::Json;
use axum::Router;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post, put};
use duet_wire::models::{AccountMoment, MomentKind};
use front_door::http::{ConsentView, FrontDoorState, SessionView};
use front_door::{FrontDoorStore, LinkError, Premium, SessionError, SignInError};
use uuid::Uuid;

/// The `REQUIRED_MIGRATION` constant.
pub const REQUIRED_MIGRATION: &str = "0120_one_review_bookmark";

const LIST_SQL: &str = include_str!("sql/list_moments.sql");
const DELETE_SQL: &str = include_str!("sql/delete_moment.sql");

#[derive(Debug)]
/// The `ApiError` cases.
pub enum ApiError {
    /// The `Unauthorized` case.
    Unauthorized,
    /// The bearer was valid, and sign out everywhere ended that session.
    SessionRevoked,
    /// The bearer is valid and this user has no account yet.
    AbsentAccount,
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
            Self::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                serde_json::json!("Invalid bearer token."),
            ),
            Self::SessionRevoked => (
                StatusCode::UNAUTHORIZED,
                serde_json::json!({
                    "code": "session_revoked",
                    "message": "Sign in to Duet again.",
                }),
            ),
            Self::AbsentAccount => (
                StatusCode::UNAUTHORIZED,
                serde_json::json!("Duet account does not exist."),
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
            Self::Storage(_) => (
                StatusCode::SERVICE_UNAVAILABLE,
                serde_json::json!("Backend database is not configured."),
            ),
        };
        (status, Json(serde_json::json!({ "detail": detail }))).into_response()
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

#[async_trait]
/// Behavior implemented by `Moments`.
pub trait Moments: Send + Sync {
    /// The `user_id` value.
    async fn list(&self, user_id: Uuid) -> Result<Vec<AccountMoment>, ApiError>;
    /// The `user_id` value.
    async fn remove(&self, user_id: Uuid, moment_id: Uuid) -> Result<bool, ApiError>;
}

/// The `PgMoments` value exchanged by this crate.
pub struct PgMoments {
    pool: sqlx::PgPool,
}

impl PgMoments {
    /// Creates the client.
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Moments for PgMoments {
    async fn list(&self, user_id: Uuid) -> Result<Vec<AccountMoment>, ApiError> {
        let rows = sqlx::query_as::<_, MomentRow>(LIST_SQL)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|error| ApiError::Storage(error.to_string()))?;
        rows.into_iter().map(MomentRow::into_moment).collect()
    }

    async fn remove(&self, user_id: Uuid, moment_id: Uuid) -> Result<bool, ApiError> {
        if env::var("DUET_PRIVACY_JOURNAL_BUCKET")
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
        {
            return Err(ApiError::Unavailable(
                "Privacy journal confirmation is not served by this process.",
            ));
        }
        let deleted = sqlx::query(DELETE_SQL)
            .bind(moment_id)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|error| ApiError::Storage(error.to_string()))?;
        Ok(deleted.rows_affected() == 1)
    }
}

#[derive(sqlx::FromRow)]
struct MomentRow {
    id: Uuid,
    kind: String,
    occurred_at: chrono::DateTime<chrono::Utc>,
}

impl MomentRow {
    fn into_moment(self) -> Result<AccountMoment, ApiError> {
        let kind = match self.kind.as_str() {
            "first_summary_prepared" => MomentKind::FirstSummaryPrepared,
            _ => {
                return Err(ApiError::Storage(
                    "account moment kind is not a known value".to_owned(),
                ));
            }
        };
        Ok(AccountMoment::new(
            self.id,
            kind,
            self.occurred_at.fixed_offset(),
        ))
    }
}

#[derive(Clone)]
pub(crate) struct AppState {
    moments: Arc<dyn Moments>,
    dev_bearer: bool,
    pub(crate) care: Arc<Mutex<care::ApiConnectedCare>>,
    pub(crate) accounts: Arc<Mutex<session::Accounts>>,
}

/// Builds the HTTP router.
pub fn router(moments: Arc<dyn Moments>) -> Router {
    let dev_bearer = env::var("DUET_API_AUTH").ok().as_deref() == Some("dev-bearer");
    router_with(moments, dev_bearer)
}

/// Builds the router, including the dev bearer when asked.
pub fn router_with(moments: Arc<dyn Moments>, dev_bearer: bool) -> Router {
    router_with_care(
        moments,
        dev_bearer,
        Arc::new(Mutex::new(care::ApiConnectedCare::default())),
    )
}

/// Builds the router with an explicit Connected Care store.
pub fn router_with_care(
    moments: Arc<dyn Moments>,
    dev_bearer: bool,
    care: Arc<Mutex<care::ApiConnectedCare>>,
) -> Router {
    Router::new()
        .route("/api/moments", get(list_moments))
        .route("/api/moments/{moment_id}", delete(remove_moment))
        .route(
            "/api/doctor-collection/share-link",
            post(care::post_create_share_link).delete(care::delete_share_link),
        )
        .route(
            "/shared/for-my-doctor/{token}",
            get(care::get_shared_for_my_doctor),
        )
        .route(
            "/api/physician-sharing/status",
            get(care::get_sharing_status),
        )
        .route(
            "/api/physician-sharing/accept-invitation",
            post(care::post_accept_invitation),
        )
        .route(
            "/api/physician-sharing/consent",
            put(care::put_grant_consent),
        )
        .route(
            "/api/physician-sharing/stop-sharing",
            post(care::post_stop_sharing),
        )
        .route(
            "/api/physician-sharing/records",
            post(care::post_patient_record),
        )
        .route(
            "/api/physician-sharing/consented-records",
            get(care::get_consented_records),
        )
        .route(
            "/api/luna-notes/propose",
            post(care::post_propose_luna_note),
        )
        .route(
            "/api/luna-notes/proposals",
            get(care::get_luna_note_proposals),
        )
        .route(
            "/api/luna-notes/{id}/approve",
            post(care::post_approve_luna_note),
        )
        .route(
            "/api/doctor-plan",
            get(care::get_doctor_plan).put(care::put_doctor_plan),
        )
        .route("/api/auth/account", put(session::put_ensure_account))
        .route("/api/auth/account/terms", put(session::put_accept_terms))
        .route(
            "/api/auth/session/revoke",
            post(session::post_revoke_sessions),
        )
        .route(
            "/api/auth/account/today-stats",
            put(session::put_today_stats),
        )
        .with_state(AppState {
            moments,
            dev_bearer,
            care,
            accounts: Arc::new(Mutex::new(session::Accounts::default())),
        })
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

/// Builds the router with the front door embedded: join, consent, premium,
/// plus the front-door account routes and the existing moments routes.
pub fn router_with_front_door(api: DuetApi, moments: Arc<dyn Moments>) -> Router {
    front_door_router(api).merge(router_with(moments, dev_bearer_from_env()))
}

fn dev_bearer_from_env() -> bool {
    env::var("DUET_API_AUTH").ok().as_deref() == Some("dev-bearer")
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

async fn list_moments(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<AccountMoment>>, ApiError> {
    let user_id = account_id(&state, &headers)?;
    Ok(Json(state.moments.list(user_id).await?))
}

async fn remove_moment(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(moment_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let user_id = account_id(&state, &headers)?;
    if state.moments.remove(user_id, moment_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound)
    }
}

pub(crate) fn account_id(state: &AppState, headers: &HeaderMap) -> Result<Uuid, ApiError> {
    if !state.dev_bearer {
        return Err(ApiError::Unavailable("Auth is not configured."));
    }
    let header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    let token = header
        .strip_prefix("Bearer dev:")
        .ok_or(ApiError::Unauthorized)?;
    let user_id = Uuid::parse_str(token).map_err(|_| ApiError::Unauthorized)?;
    if state
        .accounts
        .lock()
        .expect("accounts lock")
        .is_revoked(user_id)
    {
        return Err(ApiError::SessionRevoked);
    }
    Ok(user_id)
}

/// Reports whether the required migration is applied.
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
    use chrono::TimeZone;
    use front_door::MemoryFrontDoorStore;
    use tower::ServiceExt;

    const NOW: u64 = 1_760_000_000;

    struct Memory {
        rows: std::sync::Mutex<Vec<AccountMoment>>,
    }

    #[async_trait]
    impl Moments for Memory {
        async fn list(&self, _user_id: Uuid) -> Result<Vec<AccountMoment>, ApiError> {
            Ok(self.rows.lock().unwrap().clone())
        }

        async fn remove(&self, _user_id: Uuid, moment_id: Uuid) -> Result<bool, ApiError> {
            let mut rows = self.rows.lock().unwrap();
            let before = rows.len();
            rows.retain(|row| row.id != moment_id);
            Ok(rows.len() != before)
        }
    }

    fn moment() -> AccountMoment {
        AccountMoment::new(
            Uuid::nil(),
            MomentKind::FirstSummaryPrepared,
            chrono::Utc
                .with_ymd_and_hms(2026, 1, 2, 3, 4, 5)
                .unwrap()
                .fixed_offset(),
        )
    }

    #[tokio::test]
    async fn lists_and_deletes_one_account_moment() {
        let store = Arc::new(Memory {
            rows: std::sync::Mutex::new(vec![moment()]),
        });
        let app = router_with(store, true);
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/moments")
                    .header(
                        "authorization",
                        "Bearer dev:00000000-0000-0000-0000-000000000001",
                    )
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let deleted = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/moments/00000000-0000-0000-0000-000000000000")
                    .header(
                        "authorization",
                        "Bearer dev:00000000-0000-0000-0000-000000000001",
                    )
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(deleted.status(), StatusCode::NO_CONTENT);
    }

    #[test]
    fn sql_matches_the_python_moment_store() {
        let list = LIST_SQL.split_whitespace().collect::<Vec<_>>().join(" ");
        let delete = DELETE_SQL.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(list.contains("from public.account_moments"));
        assert!(list.contains("order by occurred_at desc, id desc"));
        assert!(delete.contains("delete from public.account_moments"));
        assert_eq!(REQUIRED_MIGRATION, "0120_one_review_bookmark");
    }

    fn front_door_app() -> (Router, Arc<MemoryFrontDoorStore>) {
        let store = Arc::new(MemoryFrontDoorStore::new());
        let mut front_door = FrontDoorState::new(store.clone());
        front_door.now = Arc::new(|| NOW);
        let moments: Arc<dyn Moments> = Arc::new(Memory {
            rows: std::sync::Mutex::new(Vec::new()),
        });
        let app = router_with_front_door(DuetApi { front_door }, moments);
        (app, store)
    }

    /// Invites through the front door (the practice side, as ENG-48 will own
    /// it) and posts the join through Duet's route.
    async fn joined(app: &Router, email: &str) -> (serde_json::Value, String) {
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
                            "patient_id": "patient-1",
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
        let (body, session) = joined(&app, "luna@example.test").await;
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
        let (_, _) = joined(&app, "luna@example.test").await;
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
        let (_, session) = joined(&app, "luna@example.test").await;
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
        let (_, _session) = joined(&app, "luna@example.test").await;
        let anonymous = request(&app, "GET", "/api/consent", None).await;
        assert_eq!(anonymous.status(), StatusCode::UNAUTHORIZED);
        let forged = request(&app, "GET", "/api/consent", Some("codaf-forged")).await;
        assert_eq!(forged.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn premium_reads_free_until_the_front_door_says_otherwise() {
        let (app, store) = front_door_app();
        let (_, session) = joined(&app, "luna@example.test").await;
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

    #[tokio::test]
    async fn connected_care_routes_enforce_consent_zero_private_leakage_and_read_only_plan() {
        let store = Arc::new(Memory {
            rows: std::sync::Mutex::new(vec![moment()]),
        });
        let app = router_with(store, true);
        let auth = "Bearer dev:00000000-0000-0000-0000-000000000001";

        // Accept invitation & grant consent.
        let accept = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/physician-sharing/accept-invitation")
                    .header("authorization", auth)
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"practice_id":"practice-coda-1","practice_name":"Coda Internal Medicine","patient_workspace_id":"pat-elena","invitation_token":"inv-1"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(accept.status(), StatusCode::OK);

        let propose = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/luna-notes/propose")
                    .header("authorization", auth)
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"date":"2026-10-13","title":"Sleep consistency","summary":"Sleeping 7.5 hours consistently.","private_operating_note":"private_luna_operating_index_do_not_leak"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(propose.status(), StatusCode::OK);
        let prop_bytes = axum::body::to_bytes(propose.into_body(), 64_000)
            .await
            .unwrap();
        let prop_json: serde_json::Value = serde_json::from_slice(&prop_bytes).unwrap();
        let prop_id = prop_json["id"].as_str().unwrap();

        let approve = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/api/luna-notes/{prop_id}/approve"))
                    .header("authorization", auth)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(approve.status(), StatusCode::OK);

        let consent = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/api/physician-sharing/consent")
                    .header("authorization", auth)
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"consent_version":"v1"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(consent.status(), StatusCode::OK);

        let read = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/physician-sharing/consented-records")
                    .header("authorization", auth)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(read.status(), StatusCode::OK);
        let read_bytes = axum::body::to_bytes(read.into_body(), 64_000)
            .await
            .unwrap();
        let read_str = String::from_utf8(read_bytes.to_vec()).unwrap();
        assert!(!read_str.contains("private_luna_operating_index_do_not_leak"));
        let read_json: serde_json::Value = serde_json::from_str(&read_str).unwrap();
        assert_eq!(read_json["items"].as_array().unwrap().len(), 1);
        assert_eq!(
            read_json["items"][0]["category"],
            "patient_approved_luna_note"
        );

        // Stop sharing in one step.
        let stop = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/physician-sharing/stop-sharing")
                    .header("authorization", auth)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(stop.status(), StatusCode::OK);

        let blocked = app
            .oneshot(
                Request::builder()
                    .uri("/api/physician-sharing/consented-records")
                    .header("authorization", auth)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(blocked.status(), StatusCode::FORBIDDEN);
    }

    fn session_app() -> Router {
        router_with(
            Arc::new(Memory {
                rows: std::sync::Mutex::new(Vec::new()),
            }),
            true,
        )
    }

    async fn auth_call(
        app: &Router,
        method: &str,
        uri: &str,
        auth: Option<&str>,
        body: Option<&str>,
    ) -> (StatusCode, serde_json::Value) {
        let mut builder = Request::builder().method(method).uri(uri);
        if let Some(auth) = auth {
            builder = builder.header("authorization", auth);
        }
        let body = match body {
            Some(body) => {
                builder = builder.header("content-type", "application/json");
                Body::from(body.to_owned())
            }
            None => Body::empty(),
        };
        let response = app
            .clone()
            .oneshot(builder.body(body).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), 64_000)
            .await
            .unwrap();
        let value = if bytes.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::from_slice(&bytes).unwrap()
        };
        (status, value)
    }

    #[tokio::test]
    async fn session_routes_ensure_terms_stats_revoke_then_bearer_fails() {
        let app = session_app();
        let ada = "Bearer dev:00000000-0000-0000-0000-000000000001";
        let other = "Bearer dev:00000000-0000-0000-0000-000000000002";
        let terms = format!(r#"{{"version":"{}"}}"#, session::CURRENT_TERMS_VERSION);

        let (status, body) = auth_call(&app, "PUT", "/api/auth/account", None, Some("{}")).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body["detail"], "Invalid bearer token.");

        let (status, _) = auth_call(
            &app,
            "PUT",
            "/api/auth/account",
            Some("Bearer not-a-dev-token"),
            Some("{}"),
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        let (status, _) = auth_call(
            &app,
            "PUT",
            "/api/auth/account",
            Some("Bearer dev:not-a-uuid"),
            Some("{}"),
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        let (status, body) = auth_call(
            &app,
            "PUT",
            "/api/auth/account/terms",
            Some(ada),
            Some(&terms),
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body["detail"], "Duet account does not exist.");

        let (status, body) = auth_call(
            &app,
            "PUT",
            "/api/auth/account/today-stats",
            Some(ada),
            Some(r#"{"stat_ids":["metric:steps"]}"#),
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body["detail"], "Duet account does not exist.");

        let (status, ensured) = auth_call(
            &app,
            "PUT",
            "/api/auth/account",
            Some(ada),
            Some(r#"{"display_name":"  Ada   Lovelace  "}"#),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(ensured["display_name"], "Ada Lovelace");
        assert_eq!(ensured["setup_complete"], false);
        assert_eq!(ensured["terms_accepted"], false);
        assert_eq!(ensured["terms_accepted_at"], serde_json::Value::Null);
        assert_eq!(ensured["terms_version"], session::CURRENT_TERMS_VERSION);
        assert_eq!(ensured["intelligence_enabled"], true);
        assert_eq!(ensured["intelligence_available"], false);
        assert_eq!(ensured["luna_consent"], "undecided");
        assert_eq!(ensured["today_stat_ids"], serde_json::Value::Null);

        let (status, kept) =
            auth_call(&app, "PUT", "/api/auth/account", Some(ada), Some("{}")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(kept["display_name"], "Ada Lovelace");

        let (status, body) = auth_call(
            &app,
            "PUT",
            "/api/auth/account/today-stats",
            Some(ada),
            Some(r#"{"stat_ids":["metric:steps"]}"#),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(body["detail"]["code"], "terms_acceptance_required");

        let (status, accepted) = auth_call(
            &app,
            "PUT",
            "/api/auth/account/terms",
            Some(ada),
            Some(&terms),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(accepted["terms_accepted"], true);
        let accepted_at = accepted["terms_accepted_at"].as_str().unwrap().to_owned();

        let (status, again) = auth_call(
            &app,
            "PUT",
            "/api/auth/account/terms",
            Some(ada),
            Some(&terms),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(again["terms_accepted_at"], accepted_at);

        let (status, stale) = auth_call(
            &app,
            "PUT",
            "/api/auth/account/terms",
            Some(ada),
            Some(r#"{"version":"2026-01-01"}"#),
        )
        .await;
        assert_eq!(status, StatusCode::CONFLICT);
        assert_eq!(stale["detail"]["code"], "agreement_version_changed");

        let (status, still) =
            auth_call(&app, "PUT", "/api/auth/account", Some(ada), Some("{}")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(still["terms_accepted"], true);
        assert_eq!(still["terms_accepted_at"], accepted_at);

        let (status, _) = auth_call(
            &app,
            "PUT",
            "/api/auth/account/today-stats",
            Some(ada),
            Some(r#"{"stat_ids":["metric:not_a_real_metric"]}"#),
        )
        .await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

        let (status, saved) = auth_call(
            &app,
            "PUT",
            "/api/auth/account/today-stats",
            Some(ada),
            Some(r#"{"stat_ids":["metric:steps","signal:hrv_overnight"]}"#),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            saved["stat_ids"],
            serde_json::json!(["metric:steps", "signal:hrv_overnight"])
        );

        let (status, with_stats) =
            auth_call(&app, "PUT", "/api/auth/account", Some(ada), Some("{}")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            with_stats["today_stat_ids"],
            serde_json::json!(["metric:steps", "signal:hrv_overnight"])
        );

        let (status, revoked) =
            auth_call(&app, "POST", "/api/auth/session/revoke", Some(ada), None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(revoked["status"], "confirmed");

        let (status, refused) =
            auth_call(&app, "PUT", "/api/auth/account", Some(ada), Some("{}")).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(refused["detail"]["code"], "session_revoked");

        let (status, _) = auth_call(&app, "GET", "/api/moments", Some(ada), None).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        let (status, other_account) =
            auth_call(&app, "PUT", "/api/auth/account", Some(other), Some("{}")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(other_account["terms_accepted"], false);
        assert_eq!(other_account["display_name"], serde_json::Value::Null);
    }
}
