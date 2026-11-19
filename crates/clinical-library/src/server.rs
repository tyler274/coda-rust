//! Axum HTTP router serving `/v1/search`, `/v1/read`, and `/v1/freshness`
//! from Postgres-backed clinical library snapshots.

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;

use crate::corpus::{DEFAULT_SEARCH_LIMIT, ReadItemOutcome};
use crate::store::{Store, StoreError};

/// Builds the Axum router serving clinical evidence search, exact read, and freshness endpoints.
pub fn router(store: Store) -> Router {
    Router::new()
        .route("/v1/search", post(handle_search))
        .route("/v1/read", post(handle_read))
        .route("/v1/freshness", get(handle_freshness))
        .with_state(store)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SearchRequest {
    mode: String,
    #[serde(default)]
    query: Option<String>,
    #[serde(default)]
    scheme: Option<String>,
    #[serde(default)]
    value: Option<String>,
    #[serde(default)]
    candidate_id: Option<String>,
    #[serde(default)]
    relationship: Option<String>,
    #[serde(default)]
    cursor: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ReadRequest {
    candidate_ids: Vec<String>,
}

#[derive(serde::Serialize)]
struct ReadResponse {
    results: Vec<ReadItemOutcome>,
}

struct ServerError(StatusCode, String);

impl From<StoreError> for ServerError {
    fn from(err: StoreError) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        (
            self.0,
            Json(serde_json::json!({
                "error": self.1,
            })),
        )
            .into_response()
    }
}

async fn handle_search(
    State(store): State<Store>,
    Json(req): Json<SearchRequest>,
) -> Result<Response, ServerError> {
    if req.mode != "query"
        || req.scheme.is_some()
        || req.value.is_some()
        || req.candidate_id.is_some()
        || req.relationship.is_some()
    {
        return Err(ServerError(
            StatusCode::BAD_REQUEST,
            "search endpoint accepts only query mode".into(),
        ));
    }
    let query = req.query.as_deref().map(str::trim).unwrap_or_default();
    if query.is_empty() {
        return Err(ServerError(
            StatusCode::BAD_REQUEST,
            "search query cannot be blank".into(),
        ));
    }

    let result = store
        .search_active(query, req.cursor.as_deref(), DEFAULT_SEARCH_LIMIT)
        .await?;
    Ok(Json(result).into_response())
}

async fn handle_read(
    State(store): State<Store>,
    Json(req): Json<ReadRequest>,
) -> Result<Response, ServerError> {
    let results = store.read_active(&req.candidate_ids).await?;
    Ok(Json(ReadResponse { results }).into_response())
}

async fn handle_freshness(State(store): State<Store>) -> Result<Response, ServerError> {
    let report = store.freshness_report().await?;
    Ok(Json(report).into_response())
}
