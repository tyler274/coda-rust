//! The rehearsal practice: synthetic staging traffic that telemetry labels
//! `simulation`.
//!
//! Real staging accounts stay in the synthetic practice
//! ([`crate::STAGING_PRACTICE_ID`]) and their chats stay `real`. Rehearsal
//! chats run in this separate practice, with its own physician and its own
//! fixture panel on the same fixed clock, so naming the practice in
//! `PHYSICIAN_SIMULATION_PRACTICES` relabels only rehearsal traffic. Its
//! patients are the fixture patients under rehearsal workspace ids: a
//! workspace belongs to one practice, so the two practices never share one.

use local_product::physician_sources::workspace_id;
use local_product::runtime::Store;
use physician_auth::{AuthStore, PostgresAuthStore};

use crate::auth::SYNTHETIC_PRACTICE_CLOCK;

/// The rehearsal practice id.
pub const REHEARSAL_PRACTICE_ID: &str = "coda-rehearsal-practice";

/// The one physician who works in the rehearsal practice.
pub const REHEARSAL_PHYSICIAN_ID: &str = "coda-rehearsal-physician";

/// The rehearsal physician's account email. Nobody signs in with it by link;
/// rehearsal sessions come from the rehearsal sign-in route.
pub const REHEARSAL_EMAIL: &str = "rehearsal@codahealth.test";

/// The source the rehearsal panel's charts come from.
const REHEARSAL_SOURCE: &str = "rehearsal-fixture";

/// The rehearsal panel's workspace ids, one per fixture patient, in fixture
/// order.
pub fn rehearsal_workspaces() -> Vec<String> {
    crate::WORKSPACE_IDS
        .iter()
        .map(|fixture| workspace_id(REHEARSAL_SOURCE, fixture))
        .collect()
}

/// Creates the rehearsal practice on the synthetic clock, its physician
/// account, and its fixture panel. Running it again changes nothing: an
/// existing account is left as an administrator set it, and a patient already
/// on the panel stays.
pub async fn seed_rehearsal_practice(
    auth: &PostgresAuthStore,
    runtime: &Store,
    now: u64,
) -> Result<(), String> {
    auth.ensure_practice(REHEARSAL_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK)
        .await
        .map_err(|_| "create the rehearsal practice".to_owned())?;
    auth.ensure_account(
        REHEARSAL_PRACTICE_ID,
        REHEARSAL_PHYSICIAN_ID,
        REHEARSAL_EMAIL,
        "Rehearsal Physician",
        false,
        now,
    )
    .await
    .map_err(|error| format!("rehearsal account: {error}"))?;
    let physician = physician_auth::PhysicianIdentity {
        practice_id: REHEARSAL_PRACTICE_ID.to_owned(),
        physician_id: REHEARSAL_PHYSICIAN_ID.to_owned(),
    };
    for (fixture, workspace) in crate::WORKSPACE_IDS.iter().zip(rehearsal_workspaces()) {
        let owner = auth
            .patient_practice(&workspace)
            .await
            .map_err(|_| format!("{workspace}: read the patient's practice"))?;
        if owner
            .as_deref()
            .is_some_and(|id| id != REHEARSAL_PRACTICE_ID)
        {
            return Err(format!("{workspace}: belongs to another practice"));
        }
        let mut pull = crate::patients::fixture_pull(fixture);
        pull.source_id = REHEARSAL_SOURCE.to_owned();
        pull.source_label = "Rehearsal fixture".to_owned();
        runtime
            .record_chart(&workspace, &[], &pull)
            .await
            .map_err(|error| format!("{workspace}: {error}"))?;
        auth.register_patient(REHEARSAL_PRACTICE_ID, &workspace)
            .await
            .map_err(|_| format!("{workspace}: register the patient"))?;
        auth.place_on_panel(&physician, &workspace)
            .await
            .map_err(|_| format!("{workspace}: place the patient on the panel"))?;
    }
    Ok(())
}

/// The rehearsal sign-in: `POST /auth/rehearsal`.
///
/// IAP has already admitted the caller; this route reads IAP's signed
/// assertion again, checks it against physician's own audience, and requires
/// the admitted account to be the one configured rehearsal service account.
/// Only then does it mint a bearer session, always for the rehearsal
/// physician, so the session can reach nothing but the rehearsal panel. A
/// human IAP user, any other service account, or a request without a valid
/// assertion gets no session.
pub(crate) fn sign_in_routes(
    auth: std::sync::Arc<crate::auth::Auth>,
    config: crate::hosting::RehearsalSignIn,
) -> axum::Router {
    let state = std::sync::Arc::new(RehearsalState {
        verifier: http::iap::IapVerifier::new(Some(config.audience)),
        principal: config.principal,
        auth,
    });
    sign_in_router(state)
}

fn sign_in_router(state: std::sync::Arc<RehearsalState>) -> axum::Router {
    axum::Router::new()
        .route("/auth/rehearsal", axum::routing::post(sign_in))
        .with_state(state)
}

struct RehearsalState {
    verifier: http::iap::IapVerifier,
    principal: String,
    auth: std::sync::Arc<crate::auth::Auth>,
}

async fn sign_in(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<RehearsalState>>,
    headers: axum::http::HeaderMap,
) -> axum::response::Response {
    use axum::Json;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    let identity = match http::iap::single_header(&headers, http::iap::ASSERTION_HEADER) {
        Some(token) => state.verifier.identity(token).await,
        None => None,
    };
    let Some(identity) = identity else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "detail": "No valid IAP assertion." })),
        )
            .into_response();
    };
    if !identity.email.eq_ignore_ascii_case(&state.principal) {
        tracing::warn!(
            "rehearsal sign-in refused an IAP account that is not the rehearsal principal"
        );
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({ "detail": "This account may not rehearse." })),
        )
            .into_response();
    }
    let now = crate::auth::now();
    let store = &state.auth.store;
    let minted = match store.begin_sign_in(REHEARSAL_EMAIL, now).await {
        Ok(token) => store.complete_sign_in(REHEARSAL_EMAIL, &token, now).await,
        Err(error) => Err(error),
    };
    match minted {
        Ok((identity, session)) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "session": session,
                "physician_id": identity.physician_id,
                "practice_id": identity.practice_id,
            })),
        )
            .into_response(),
        Err(error) => {
            tracing::warn!(?error, "the rehearsal physician could not be signed in");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({
                    "detail": "The rehearsal physician is not available; run the migration job with the rehearsal practice."
                })),
            )
                .into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use physician_auth::{Membership, MemoryAuthStore, PhysicianIdentity};
    use std::sync::Arc;
    use tower::ServiceExt;

    const AUDIENCE: &str = "/projects/123/locations/us-east1/services/physician";
    const PRINCIPAL: &str = "physician-rehearsal@coda-rust-staging.iam.gserviceaccount.com";
    const SA_SUB: &str = "accounts.google.com:2000";

    /// The rehearsal route on a memory store seeded with the staging and
    /// rehearsal practices, plus the auth state behind it.
    async fn rehearsal_app() -> (axum::Router, Arc<crate::auth::Auth>) {
        let store = Arc::new(MemoryAuthStore::new());
        store.seed_account(
            crate::STAGING_PRACTICE_ID,
            "staging-physician",
            "physician@codahealth.test",
            "Staging Physician",
            true,
        );
        store.seed_account(
            REHEARSAL_PRACTICE_ID,
            REHEARSAL_PHYSICIAN_ID,
            REHEARSAL_EMAIL,
            "Rehearsal Physician",
            false,
        );
        store
            .ensure_practice(REHEARSAL_PRACTICE_ID, SYNTHETIC_PRACTICE_CLOCK)
            .await
            .unwrap();
        let rehearsal = PhysicianIdentity {
            practice_id: REHEARSAL_PRACTICE_ID.to_owned(),
            physician_id: REHEARSAL_PHYSICIAN_ID.to_owned(),
        };
        for workspace in rehearsal_workspaces() {
            store
                .register_patient(REHEARSAL_PRACTICE_ID, &workspace)
                .await
                .unwrap();
            store
                .set_membership(&rehearsal, &workspace, Membership::Allowed)
                .await
                .unwrap();
        }
        let auth = crate::auth::Auth::over_fixture(
            Arc::clone(&store) as Arc<dyn AuthStore>,
            Arc::new(front_door::MemoryFrontDoorStore::new()),
        )
        .await;
        let state = Arc::new(RehearsalState {
            verifier: http::iap::fixture::verifier(AUDIENCE),
            principal: PRINCIPAL.to_owned(),
            auth: Arc::clone(&auth),
        });
        let app = crate::auth::required_router(
            crate::fixture_site().merge(sign_in_router(state)),
            Arc::clone(&auth),
        );
        (app, auth)
    }

    async fn rehearse(
        app: &axum::Router,
        assertion: Option<&str>,
    ) -> (StatusCode, serde_json::Value) {
        let mut request = Request::post("/auth/rehearsal");
        if let Some(assertion) = assertion {
            request = request.header(http::iap::ASSERTION_HEADER, assertion);
        }
        let response = app
            .clone()
            .oneshot(request.body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let body = axum::body::to_bytes(response.into_body(), 1_000_000)
            .await
            .unwrap();
        (status, serde_json::from_slice(&body).unwrap_or_default())
    }

    async fn get(app: &axum::Router, path: &str, session: &str) -> (StatusCode, String) {
        let response = app
            .clone()
            .oneshot(
                Request::get(path)
                    .header("authorization", format!("Bearer {session}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let body = axum::body::to_bytes(response.into_body(), 2_000_000)
            .await
            .unwrap();
        (status, String::from_utf8_lossy(&body).into_owned())
    }

    #[test]
    fn rehearsal_workspaces_never_collide_with_the_staging_panel() {
        let rehearsal = rehearsal_workspaces();
        assert_eq!(rehearsal.len(), crate::WORKSPACE_IDS.len());
        for workspace in &rehearsal {
            assert!(workspace.starts_with("pw-"));
            assert!(!crate::WORKSPACE_IDS.contains(&workspace.as_str()));
        }
    }

    #[tokio::test]
    async fn only_a_valid_assertion_for_this_audience_is_considered() {
        let (app, _) = rehearsal_app().await;
        assert_eq!(rehearse(&app, None).await.0, StatusCode::UNAUTHORIZED);
        assert_eq!(
            rehearse(&app, Some("not-a-jwt")).await.0,
            StatusCode::UNAUTHORIZED
        );
        // Another service's audience, even for the right principal.
        let elsewhere = http::iap::fixture::identity_assertion(
            "/projects/123/locations/us-east1/services/duet",
            SA_SUB,
            PRINCIPAL,
        );
        assert_eq!(
            rehearse(&app, Some(&elsewhere)).await.0,
            StatusCode::UNAUTHORIZED
        );
        // A forged signature: the right claims, the signature of other claims.
        let genuine = http::iap::fixture::identity_assertion(AUDIENCE, SA_SUB, PRINCIPAL);
        let other =
            http::iap::fixture::identity_assertion(AUDIENCE, SA_SUB, "x@y.iam.gserviceaccount.com");
        let mut parts: Vec<&str> = genuine.split('.').collect();
        parts[2] = other.split('.').nth(2).unwrap();
        let forged = parts.join(".");
        assert_eq!(
            rehearse(&app, Some(&forged)).await.0,
            StatusCode::UNAUTHORIZED
        );
        // Expired.
        let mut claims = http::iap::fixture::claims(AUDIENCE, crate::auth::now() - 3_600);
        claims["email"] = serde_json::json!(PRINCIPAL);
        let expired = http::iap::fixture::sign(&claims, Some(http::iap::fixture::KID));
        assert_eq!(
            rehearse(&app, Some(&expired)).await.0,
            StatusCode::UNAUTHORIZED
        );
    }

    #[tokio::test]
    async fn without_the_settings_there_is_no_rehearsal_route() {
        let store = Arc::new(MemoryAuthStore::new());
        let auth = crate::auth::Auth::over_fixture(
            store as Arc<dyn AuthStore>,
            Arc::new(front_door::MemoryFrontDoorStore::new()),
        )
        .await;
        let app = crate::auth::required_router(crate::fixture_site(), auth);
        let assertion = http::iap::fixture::identity_assertion(AUDIENCE, SA_SUB, PRINCIPAL);
        let (status, body) = rehearse(&app, Some(&assertion)).await;
        assert_ne!(status, StatusCode::OK, "{body}");
        assert!(body.get("session").is_none(), "{body}");
    }

    #[tokio::test]
    async fn only_the_configured_service_account_may_rehearse() {
        let (app, _) = rehearsal_app().await;
        for email in [
            http::iap::fixture::EMAIL,
            "physician@codahealth.test",
            "availability-probe@coda-rust-staging.iam.gserviceaccount.com",
            "physician-rehearsal@other-project.iam.gserviceaccount.com",
        ] {
            let assertion = http::iap::fixture::identity_assertion(AUDIENCE, SA_SUB, email);
            let (status, body) = rehearse(&app, Some(&assertion)).await;
            assert_eq!(status, StatusCode::FORBIDDEN, "{email}");
            assert!(body.get("session").is_none(), "{email}: {body}");
        }
    }

    #[tokio::test]
    async fn the_rehearsal_session_reaches_only_the_rehearsal_panel() {
        let (app, _) = rehearsal_app().await;
        let assertion = http::iap::fixture::identity_assertion(AUDIENCE, SA_SUB, PRINCIPAL);
        let (status, body) = rehearse(&app, Some(&assertion)).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(body["practice_id"], REHEARSAL_PRACTICE_ID);
        assert_eq!(body["physician_id"], REHEARSAL_PHYSICIAN_ID);
        let session = body["session"].as_str().expect("session").to_owned();

        let (status, whoami) = get(&app, "/auth/whoami", &session).await;
        assert_eq!(status, StatusCode::OK, "{whoami}");
        assert!(whoami.contains(REHEARSAL_EMAIL), "{whoami}");
        let (status, listed) = get(&app, "/patient-workspaces", &session).await;
        assert_eq!(status, StatusCode::OK, "{listed}");
        for workspace in crate::WORKSPACE_IDS {
            assert!(!listed.contains(workspace), "{workspace} leaked: {listed}");
            let (status, body) =
                get(&app, &format!("/patient-workspaces/{workspace}"), &session).await;
            assert_eq!(status, StatusCode::NOT_FOUND, "{workspace}: {body}");
        }
        let (status, _) = get(&app, "/auth/panels/staging-physician", &session).await;
        assert_ne!(
            status,
            StatusCode::OK,
            "another physician's panel stays closed"
        );
    }
}
