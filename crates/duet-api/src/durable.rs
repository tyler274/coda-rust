//! Explicit, not-yet-hosted Duet route composition over the canonical owner.
//!
//! Never merge this router with the local/dev or legacy front-door router. Only
//! account provisioning/readback, terms acceptance and LOCAL session fencing are
//! converted. All clinical, privacy, linking, preference and execution operations
//! remain absent. A permission snapshot never grants an effect or makes us ready.
use crate::identity::{IdentityError, google::GoogleIdentity};
use async_trait::async_trait;
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post, put},
};
use duet_wire::models::{AccountState, LunaConsentDecision, SessionRevocationResponse};
use front_door::{
    FrontDoorStore, PostgresFrontDoorStore,
    identity::{Access, AuthorityError, LUNA_VERSION, Principal, TERMS_VERSION},
};
use std::sync::Arc;

/// Trusted authentication boundary. Implementations must check signatures and
/// current provider revocation for each request, never merely decode JWT claims.
#[async_trait]
pub trait IdentityProvider: Send + Sync {
    /// Return a verified principal; failures carry no token/provider diagnostics.
    async fn verify(&self, token: &str) -> Result<Principal, IdentityError>;
}
#[async_trait]
impl IdentityProvider for GoogleIdentity {
    async fn verify(&self, token: &str) -> Result<Principal, IdentityError> {
        GoogleIdentity::verify(self, token).await
    }
}

#[derive(Clone)]
struct Durable {
    identity: Arc<dyn IdentityProvider>,
    store: Arc<PostgresFrontDoorStore>,
}

/// Construct the narrow durable composition. Construction does not migrate,
/// enable hosted startup, or assert readiness. Runtime pools must be nonmigrating.
/// Injectable identity is a trusted dependency, not an endpoint/config override.
pub fn router(store: Arc<PostgresFrontDoorStore>, identity: Arc<dyn IdentityProvider>) -> Router {
    Router::new()
        .route("/api/auth/account", get(read).put(provision))
        .route("/api/auth/account/terms", put(terms))
        .route("/api/auth/session/revoke", post(revoke))
        .route("/readyz", get(crate::hosting::ready))
        .layer(DefaultBodyLimit::max(4096))
        .with_state(Durable { identity, store })
}

async fn principal(state: &Durable, headers: &HeaderMap) -> Result<Principal, StatusCode> {
    let mut values = headers.get_all(axum::http::header::AUTHORIZATION).iter();
    let value = values
        .next()
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    if values.next().is_some() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let token = value
        .strip_prefix("Bearer ")
        .filter(|s| {
            !s.is_empty()
                && s.len() <= 16384
                && !s.chars().any(char::is_whitespace)
                && !s.starts_with("dev:")
        })
        .ok_or(StatusCode::UNAUTHORIZED)?;
    state.identity.verify(token).await.map_err(|e| match e {
        IdentityError::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
        IdentityError::Account(e) => authority(e),
        _ => StatusCode::UNAUTHORIZED,
    })
}
fn authority(e: AuthorityError) -> StatusCode {
    match e {
        AuthorityError::Storage(_) => StatusCode::SERVICE_UNAVAILABLE,
        AuthorityError::AgreementChanged | AuthorityError::RestrictionPending => {
            StatusCode::CONFLICT
        }
        AuthorityError::Absent | AuthorityError::Refused => StatusCode::UNAUTHORIZED,
    }
}
fn now() -> i64 {
    chrono::Utc::now().timestamp()
}

async fn view(
    state: &Durable,
    principal: &Principal,
    access: Access,
) -> Result<Json<AccountState>, StatusCode> {
    let account = state
        .store
        .account(access.account_id)
        .await
        .ok_or(StatusCode::SERVICE_UNAVAILABLE)?;
    // Recheck the local fence after projection read; each request resolves anew.
    let access = state
        .store
        .identity_access(principal)
        .await
        .map_err(authority)?;
    let p = access.permissions;
    let mut result = AccountState::new();
    result.display_name = Some((!account.display_name.is_empty()).then_some(account.display_name));
    result.intelligence_available = Some(false);
    result.intelligence_enabled = Some(false);
    result.setup_complete = Some(false);
    result.terms_version = Some(TERMS_VERSION.into());
    result.terms_accepted = Some(p.terms_version.as_deref() == Some(TERMS_VERSION));
    result.terms_accepted_at = Some(
        p.terms_at
            .and_then(|t| chrono::DateTime::from_timestamp(t, 0))
            .map(|t| t.fixed_offset()),
    );
    result.luna_consent_version = Some(LUNA_VERSION.into());
    result.luna_consent = Some(if p.luna_version.is_none() {
        LunaConsentDecision::Undecided
    } else if p.luna_allowed && !p.restriction_pending {
        LunaConsentDecision::Accepted
    } else {
        LunaConsentDecision::Declined
    });
    result.luna_consent_at = Some(
        p.luna_at
            .and_then(|t| chrono::DateTime::from_timestamp(t, 0))
            .map(|t| t.fixed_offset()),
    );
    Ok(Json(result))
}
async fn read(
    State(state): State<Durable>,
    headers: HeaderMap,
) -> Result<Json<AccountState>, StatusCode> {
    let p = principal(&state, &headers).await?;
    let access = state.store.identity_access(&p).await.map_err(authority)?;
    view(&state, &p, access).await
}

// Explicit provisioning matches the retained PUT account contract. Unknown
// fields (including supplied account IDs) are refused, never silently trusted.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Provision {
    display_name: Option<String>,
}
async fn provision(
    State(state): State<Durable>,
    headers: HeaderMap,
    Json(body): Json<Provision>,
) -> Result<Json<AccountState>, StatusCode> {
    let p = principal(&state, &headers).await?;
    if body
        .display_name
        .as_ref()
        .is_some_and(|s| s.chars().count() > 80)
    {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    let access = state
        .store
        .provision_identity(&p, now())
        .await
        .map_err(authority)?;
    if let Some(name) = body.display_name {
        let name = name.split_whitespace().collect::<Vec<_>>().join(" ");
        if !name.is_empty() {
            state
                .store
                .set_identity_display_name(&p, &name)
                .await
                .map_err(authority)?;
        }
    }
    view(&state, &p, access).await
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Terms {
    version: String,
}
async fn terms(
    State(state): State<Durable>,
    headers: HeaderMap,
    Json(body): Json<Terms>,
) -> Result<Json<AccountState>, StatusCode> {
    let p = principal(&state, &headers).await?;
    let access = state
        .store
        .accept_identity_terms(&p, &body.version, now())
        .await
        .map_err(authority)?;
    view(&state, &p, access).await
}
async fn revoke(
    State(state): State<Durable>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<SessionRevocationResponse>), StatusCode> {
    let p = principal(&state, &headers).await?;
    state
        .store
        .revoke_identity_sessions(&p, now())
        .await
        .map_err(authority)?;
    // Only the local fence is durable. Provider revocation and lifecycle journal
    // confirmation are not implemented; never claim global sign-out confirmed.
    Ok((
        StatusCode::ACCEPTED,
        Json(SessionRevocationResponse::new(
            duet_wire::models::session_revocation_response::Status::Pending,
        )),
    ))
}
