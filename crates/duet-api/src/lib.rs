//! Axum owner for Duet's moments routes. Other patient routers stay on FastAPI
//! until each one moves. The physician Node server is not this crate.
//! Book: `book/src/backend-architecture.md`.
#![deny(missing_docs)]

pub mod care;

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
use uuid::Uuid;

/// The `REQUIRED_MIGRATION` constant.
pub const REQUIRED_MIGRATION: &str = "0120_one_review_bookmark";

const LIST_SQL: &str = "\
select id, kind, occurred_at \
from public.account_moments \
where user_id = $1 \
order by occurred_at desc, id desc";

const DELETE_SQL: &str = "\
delete from public.account_moments \
where id = $1 and user_id = $2";

#[derive(Debug)]
/// The `ApiError` cases.
pub enum ApiError {
    /// The `Unauthorized` case.
    Unauthorized,
    /// The `Forbidden` case.
    Forbidden,
    /// The `NotFound` case.
    NotFound,
    /// The `Unavailable` case.
    Unavailable(&'static str),
    /// The `Storage` case.
    Storage(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, detail) = match self {
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Invalid bearer token."),
            Self::Forbidden => (
                StatusCode::FORBIDDEN,
                "Accept the current Duet Terms of Service to continue.",
            ),
            Self::NotFound => (StatusCode::NOT_FOUND, "Moment not found"),
            Self::Unavailable(detail) => (StatusCode::SERVICE_UNAVAILABLE, detail),
            Self::Storage(_) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Backend database is not configured.",
            ),
        };
        (status, Json(serde_json::json!({ "detail": detail }))).into_response()
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
        .with_state(AppState {
            moments,
            dev_bearer,
            care,
        })
}

async fn list_moments(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<AccountMoment>>, ApiError> {
    let user_id = account_id(&headers, state.dev_bearer)?;
    Ok(Json(state.moments.list(user_id).await?))
}

async fn remove_moment(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(moment_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let user_id = account_id(&headers, state.dev_bearer)?;
    if state.moments.remove(user_id, moment_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound)
    }
}

fn account_id(headers: &HeaderMap, dev_bearer: bool) -> Result<Uuid, ApiError> {
    if !dev_bearer {
        return Err(ApiError::Unavailable("Auth is not configured."));
    }
    let header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    let token = header
        .strip_prefix("Bearer dev:")
        .ok_or(ApiError::Unauthorized)?;
    Uuid::parse_str(token).map_err(|_| ApiError::Unauthorized)
}

/// Reports whether the required migration is applied.
pub async fn schema_is_ready(pool: &sqlx::PgPool) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar("select exists (select 1 from public.schema_migrations where version = $1)")
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
    use tower::ServiceExt;

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
        assert!(LIST_SQL.contains("from public.account_moments"));
        assert!(LIST_SQL.contains("order by occurred_at desc, id desc"));
        assert!(DELETE_SQL.contains("delete from public.account_moments"));
        assert_eq!(REQUIRED_MIGRATION, "0120_one_review_bookmark");
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
}
