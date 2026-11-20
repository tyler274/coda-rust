//! Duet-owned account, privacy, evidence and workspace services.
//! `main.rs` composes the durable route families and recovery workers.
//! Book: `book/src/backend-architecture.md`.
#![deny(missing_docs)]
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

pub mod account_export;
mod assistant_entity;
pub mod assistant_harness;
pub mod background;
pub mod connected_care_invitations;
pub mod connected_care_sync;
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
pub mod email_auth;
mod email_auth_entity;
pub mod evidence;
mod evidence_entity;
pub mod evidence_paging;
pub mod evidence_routes;
mod follow_up_entity;
pub mod for_my_doctor;
pub mod for_my_doctor_entity;
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
mod portal_entity;
pub mod portals;
#[expect(
    clippy::disallowed_methods,
    reason = "converting raw SQL to the typed database API"
)]
pub mod privacy;
pub mod privacy_archive;
mod privacy_entity;
#[expect(
    clippy::disallowed_methods,
    reason = "converting raw SQL to the typed database API"
)]
pub mod producer_database;
pub mod producer_routes;
pub mod push;
mod push_entity;
pub mod session;
#[expect(
    clippy::disallowed_methods,
    reason = "converting raw SQL to the typed database API"
)]
pub mod workspace;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
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
    /// The `Unavailable` case.
    Unavailable(&'static str),
    /// Identity provider response decoding failed; never a product session refusal.
    IdentityResponse(reqwest::Error),
    /// The `Storage` case.
    Storage(String),
    /// The presented terms version is not the current one.
    AgreementChanged,
    /// A sign-in or email-change verification link or short code is invalid, reused, or expired.
    InvalidChallenge(&'static str),
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
            Self::AgreementChanged => (
                StatusCode::CONFLICT,
                serde_json::json!({
                    "code": "agreement_version_changed",
                    "message": "Update Duet to review the current terms.",
                }),
            ),
            Self::InvalidChallenge(detail) => (
                StatusCode::UNAUTHORIZED,
                serde_json::json!({
                    "code": "challenge_invalid",
                    "message": detail,
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

/// The `Authorization: Bearer <token>` value, when the header carries one.
fn bearer(headers: &axum::http::HeaderMap) -> Option<String> {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(str::to_owned)
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

    #[test]
    fn the_schema_readiness_check_names_the_required_migration() {
        assert_eq!(REQUIRED_MIGRATION, "0120_one_review_bookmark");
    }
}
