//! Authenticated Hint Health Chrome connector bridge HTTP routes
//! (`Scope.Now.EhrReadOnly`, `P4.1`, `P4.Undecided.Scheduler`, `P4.Undecided.Roster`).
//!
//! Operator routes (`/connections/hint/bridge/pair`, `/connections/hint/bridge/jobs`)
//! require a signed-in active practice admin.
//! Connector routes (`/connections/hint/bridge/{lease,capture,roster,appointments,deliver}`)
//! authenticate via `X-Coda-Pair: <pair_id>:<pair_key>` verified in constant time
//! against SHA-256 hashes stored in Postgres.

use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::http::{Extensions, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use local_product::runtime::Store;
use local_product::runtime::hint_bridge::{HintBridgeError, HintBridgeJobKind, HintPairHeader};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::auth::now;
use crate::care::ConnectedCare;
use crate::live::Live;

/// Header carrying `<pair_id>:<pair_key>` for paired Hint Chrome extensions.
pub const PAIR_HEADER: &str = "x-coda-pair";

const DEFAULT_LEASE_SECONDS: i64 = 120;

/// Whether `path` is a Hint Chrome connector route authenticated by `X-Coda-Pair`
/// rather than a physician bearer session.
pub(crate) fn is_connector_route(path: &str) -> bool {
    matches!(
        path,
        "/connections/hint/bridge/lease"
            | "/connections/hint/bridge/capture"
            | "/connections/hint/bridge/roster"
            | "/connections/hint/bridge/appointments"
            | "/connections/hint/bridge/deliver"
    )
}

/// Mounts all Hint Chrome bridge routes.
pub(crate) fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/connections/hint/bridge/pair", post(create_pairing))
        .route("/connections/hint/bridge/jobs", post(queue_job))
        .route("/connections/hint/bridge/lease", post(lease_job))
        .route("/connections/hint/bridge/capture", post(ingest_capture))
        .route("/connections/hint/bridge/roster", post(ingest_roster))
        .route(
            "/connections/hint/bridge/appointments",
            post(ingest_appointments),
        )
        .route("/connections/hint/bridge/deliver", post(deliver_job))
}

#[derive(Debug, Default, Deserialize)]
struct CreatePairingBody {
    #[serde(default)]
    source_label: Option<String>,
}

#[derive(Debug, Deserialize)]
struct QueueJobBody {
    read: HintBridgeJobKind,
    #[serde(default)]
    patient_id: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct LeaseJobBody {
    #[serde(default)]
    lease_seconds: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct CaptureBody {
    patient_id: String,
    capture: Value,
}

#[derive(Debug, Deserialize)]
struct PayloadBody {
    payload: Value,
}

#[derive(Debug, Deserialize)]
struct DeliverJobBody {
    job_id: String,
    lease: String,
    read: HintBridgeJobKind,
    #[serde(default)]
    patient_id: Option<String>,
    #[serde(default)]
    payload: Option<Value>,
    #[serde(default)]
    error: Option<String>,
}

async fn require_admin_store(
    connected: &ConnectedCare,
    extensions: &Extensions,
) -> Result<(String, Store), Box<Response>> {
    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let is_admin = auth_store
        .account_by_identity(&identity)
        .await
        .is_some_and(|account| account.active && account.admin);
    if !is_admin {
        return Err(Box::new(
            (
                StatusCode::FORBIDDEN,
                Json(
                    json!({ "detail": "Practice admin access is required for Hint bridge setup." }),
                ),
            )
                .into_response(),
        ));
    }
    let Some(store) = extensions.get::<Store>().cloned() else {
        return Err(Box::new(
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({ "detail": "Runtime store is not configured for Hint bridge." })),
            )
                .into_response(),
        ));
    };
    Ok((identity.practice_id, store))
}

async fn verify_connector(
    connected: &ConnectedCare,
    extensions: &Extensions,
    headers: &HeaderMap,
) -> Result<(String, Store, i64), Box<Response>> {
    let Some(raw) = headers
        .get(PAIR_HEADER)
        .and_then(|value| value.to_str().ok())
    else {
        return Err(Box::new(
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "detail": "Missing X-Coda-Pair header." })),
            )
                .into_response(),
        ));
    };
    let Some(parsed) = HintPairHeader::parse(raw) else {
        return Err(Box::new(
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "detail": "Malformed X-Coda-Pair header." })),
            )
                .into_response(),
        ));
    };
    let Some(store) = extensions.get::<Store>().cloned() else {
        return Err(Box::new(
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({ "detail": "Runtime store is not configured for Hint bridge." })),
            )
                .into_response(),
        ));
    };
    let practice = match store.verify_hint_pairing(&parsed).await {
        Ok(Some(practice)) => practice,
        Ok(None) => {
            return Err(Box::new(
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "detail": "Invalid or revoked Hint pairing." })),
                )
                    .into_response(),
            ));
        }
        Err(error) => {
            tracing::error!(%error, "hint bridge pairing lookup failed");
            return Err(Box::new(StatusCode::SERVICE_UNAVAILABLE.into_response()));
        }
    };
    let auth_store = connected.auth_store(extensions);
    let clock = auth_store
        .practice_clock(&practice)
        .await
        .unwrap_or(physician_auth::PracticeClock::RealTime);
    let read_at = clock.now(now()) as i64;
    Ok((practice, store, read_at))
}

fn bridge_error_response(error: HintBridgeError) -> Response {
    match error {
        HintBridgeError::InvalidInput | HintBridgeError::InvalidPayload(_) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "detail": error.to_string() })),
        )
            .into_response(),
        HintBridgeError::FilingFailed(detail) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({ "detail": detail })),
        )
            .into_response(),
        HintBridgeError::Schedule(_)
        | HintBridgeError::AuthStore(_)
        | HintBridgeError::Database(_) => {
            tracing::error!(%error, "hint bridge persistence error");
            StatusCode::SERVICE_UNAVAILABLE.into_response()
        }
    }
}

async fn publish_resync(extensions: &Extensions, workspace_id: Option<String>) {
    if let Some(live) = extensions.get::<Arc<Live>>() {
        if let Some(workspace_id) = workspace_id {
            let _ = live
                .bus
                .publish(physician_bus::Event::WorkspaceChanged { workspace_id })
                .await;
        }
        let _ = live.bus.publish(physician_bus::Event::Resync).await;
    }
}

async fn create_pairing(
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    extensions: Extensions,
    body: Option<Json<CreatePairingBody>>,
) -> Response {
    let (practice, store) = match require_admin_store(&connected, &extensions).await {
        Ok(ok) => ok,
        Err(resp) => return *resp,
    };
    let label = body.and_then(|Json(b)| b.source_label);
    match store.create_hint_pairing(&practice, label.as_deref()).await {
        Ok(created) => (StatusCode::CREATED, Json(created)).into_response(),
        Err(err) => bridge_error_response(err),
    }
}

async fn queue_job(
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    extensions: Extensions,
    Json(body): Json<QueueJobBody>,
) -> Response {
    let (practice, store) = match require_admin_store(&connected, &extensions).await {
        Ok(ok) => ok,
        Err(resp) => return *resp,
    };
    match store
        .queue_hint_job(&practice, body.read, body.patient_id.as_deref())
        .await
    {
        Ok(job_id) => (
            StatusCode::ACCEPTED,
            Json(json!({
                "job_id": job_id,
                "read": body.read,
                "patient_id": body.patient_id,
            })),
        )
            .into_response(),
        Err(err) => bridge_error_response(err),
    }
}

async fn lease_job(
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    extensions: Extensions,
    headers: HeaderMap,
    body: Option<Json<LeaseJobBody>>,
) -> Response {
    let (practice, store, _) = match verify_connector(&connected, &extensions, &headers).await {
        Ok(ok) => ok,
        Err(resp) => return *resp,
    };
    let lease_seconds = body
        .and_then(|Json(b)| b.lease_seconds)
        .unwrap_or(DEFAULT_LEASE_SECONDS);
    match store.lease_hint_job(&practice, lease_seconds).await {
        Ok(Some(job)) => (StatusCode::OK, Json(json!({ "job": job }))).into_response(),
        Ok(None) => (StatusCode::OK, Json(json!({ "job": Value::Null }))).into_response(),
        Err(error) => {
            tracing::error!(%error, "hint bridge lease failed");
            StatusCode::SERVICE_UNAVAILABLE.into_response()
        }
    }
}

async fn ingest_capture(
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    extensions: Extensions,
    headers: HeaderMap,
    Json(body): Json<CaptureBody>,
) -> Response {
    let (practice, store, read_at) = match verify_connector(&connected, &extensions, &headers).await
    {
        Ok(ok) => ok,
        Err(resp) => return *resp,
    };
    let auth_store = connected.auth_store(&extensions);
    match store
        .ingest_hint_chart_capture(
            &*auth_store,
            &practice,
            &body.patient_id,
            body.capture,
            read_at,
        )
        .await
    {
        Ok(summary) => {
            publish_resync(&extensions, Some(summary.workspace.clone())).await;
            (StatusCode::OK, Json(summary)).into_response()
        }
        Err(err) => bridge_error_response(err),
    }
}

async fn ingest_roster(
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    extensions: Extensions,
    headers: HeaderMap,
    Json(body): Json<PayloadBody>,
) -> Response {
    let (practice, store, read_at) = match verify_connector(&connected, &extensions, &headers).await
    {
        Ok(ok) => ok,
        Err(resp) => return *resp,
    };
    let auth_store = connected.auth_store(&extensions);
    match store
        .ingest_hint_roster(&*auth_store, &practice, &body.payload, read_at)
        .await
    {
        Ok((summary, _workspaces)) => {
            publish_resync(&extensions, None).await;
            (
                StatusCode::OK,
                Json(json!({
                    "roster_patients": summary.roster_patients,
                    "newly_registered": summary.newly_registered,
                    "seeded_links": summary.seeded_links,
                })),
            )
                .into_response()
        }
        Err(err) => bridge_error_response(err),
    }
}

async fn ingest_appointments(
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    extensions: Extensions,
    headers: HeaderMap,
    Json(body): Json<PayloadBody>,
) -> Response {
    let (practice, store, read_at) = match verify_connector(&connected, &extensions, &headers).await
    {
        Ok(ok) => ok,
        Err(resp) => return *resp,
    };
    let auth_store = connected.auth_store(&extensions);
    match store
        .ingest_hint_appointments(&*auth_store, &practice, &body.payload, read_at)
        .await
    {
        Ok(outcome) => {
            publish_resync(&extensions, None).await;
            match outcome {
                local_product::runtime::schedule::ScheduleSyncOutcome::Applied(summary) => (
                    StatusCode::OK,
                    Json(json!({
                        "outcome": "applied",
                        "saved_appointments": summary.saved_appointments,
                        "changed_appointments": summary.changed_appointments,
                        "matched_appointments": summary.matched_appointments,
                        "unmatched_appointments": summary.unmatched_appointments,
                    })),
                )
                    .into_response(),
                local_product::runtime::schedule::ScheduleSyncOutcome::FailedRead {
                    retained_appointments,
                    ..
                } => (
                    StatusCode::BAD_GATEWAY,
                    Json(json!({
                        "outcome": "failed_read",
                        "retained_appointments": retained_appointments,
                    })),
                )
                    .into_response(),
            }
        }
        Err(err) => bridge_error_response(err),
    }
}

async fn deliver_job(
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    extensions: Extensions,
    headers: HeaderMap,
    Json(body): Json<DeliverJobBody>,
) -> Response {
    let (practice, store, read_at) = match verify_connector(&connected, &extensions, &headers).await
    {
        Ok(ok) => ok,
        Err(resp) => return *resp,
    };
    if let Some(error_text) = body.error.filter(|e| !e.trim().is_empty()) {
        let settled = match store
            .settle_hint_job(&practice, &body.job_id, &body.lease, false)
            .await
        {
            Ok(settled) => settled,
            Err(error) => {
                tracing::error!(%error, "hint bridge settle failed");
                return StatusCode::SERVICE_UNAVAILABLE.into_response();
            }
        };
        if !settled {
            return (
                StatusCode::CONFLICT,
                Json(json!({ "detail": "Hint job lease does not match or has expired." })),
            )
                .into_response();
        }
        return (
            StatusCode::OK,
            Json(json!({ "outcome": "failed", "detail": error_text })),
        )
            .into_response();
    }

    let Some(payload) = body.payload else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "detail": "Delivery requires either payload or error." })),
        )
            .into_response();
    };
    let leased = match store
        .verify_hint_lease(&practice, &body.job_id, &body.lease)
        .await
    {
        Ok(Some(leased)) => leased,
        Ok(None) => {
            return (
                StatusCode::CONFLICT,
                Json(json!({ "detail": "Hint job lease does not match or has expired." })),
            )
                .into_response();
        }
        Err(error) => {
            tracing::error!(%error, "hint bridge lease check failed");
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }
    };
    let requested_patient = body
        .patient_id
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());
    if body.read != leased.read || requested_patient != leased.patient_id.as_deref() {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "detail": "Delivery read kind or patient_id does not match the leased Hint job."
            })),
        )
            .into_response();
    }
    let auth_store = connected.auth_store(&extensions);
    let (status_code, succeeded, response_body) = match leased.read {
        HintBridgeJobKind::Roster => match store
            .ingest_hint_roster(&*auth_store, &practice, &payload, read_at)
            .await
        {
            Ok((summary, _workspaces)) => {
                publish_resync(&extensions, None).await;
                (
                    StatusCode::OK,
                    true,
                    json!({
                        "outcome": "delivered",
                        "read": "roster",
                        "roster_patients": summary.roster_patients,
                        "newly_registered": summary.newly_registered,
                        "seeded_links": summary.seeded_links,
                    }),
                )
            }
            Err(err) => return bridge_error_response(err),
        },
        HintBridgeJobKind::Appointments => match store
            .ingest_hint_appointments(&*auth_store, &practice, &payload, read_at)
            .await
        {
            Ok(local_product::runtime::schedule::ScheduleSyncOutcome::Applied(summary)) => {
                publish_resync(&extensions, None).await;
                (
                    StatusCode::OK,
                    true,
                    json!({
                        "outcome": "delivered",
                        "read": "appointments",
                        "saved_appointments": summary.saved_appointments,
                        "matched_appointments": summary.matched_appointments,
                        "unmatched_appointments": summary.unmatched_appointments,
                    }),
                )
            }
            Ok(local_product::runtime::schedule::ScheduleSyncOutcome::FailedRead {
                retained_appointments,
                ..
            }) => {
                publish_resync(&extensions, None).await;
                (
                    StatusCode::BAD_GATEWAY,
                    false,
                    json!({
                        "outcome": "failed_read",
                        "read": "appointments",
                        "retained_appointments": retained_appointments,
                    }),
                )
            }
            Err(err) => return bridge_error_response(err),
        },
        HintBridgeJobKind::ChartCapture => {
            let Some(patient_id) = leased.patient_id.as_deref() else {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "detail": "chart_capture delivery requires patient_id." })),
                )
                    .into_response();
            };
            match store
                .ingest_hint_chart_capture(&*auth_store, &practice, patient_id, payload, read_at)
                .await
            {
                Ok(summary) => {
                    publish_resync(&extensions, Some(summary.workspace.clone())).await;
                    (
                        StatusCode::OK,
                        true,
                        json!({
                            "outcome": "delivered",
                            "read": "chart_capture",
                            "workspace": summary.workspace,
                            "filed_records": summary.filed_records,
                        }),
                    )
                }
                Err(err) => return bridge_error_response(err),
            }
        }
    };

    match store
        .settle_hint_job(&practice, &body.job_id, &body.lease, succeeded)
        .await
    {
        Ok(true) => (status_code, Json(response_body)).into_response(),
        Ok(false) => (
            StatusCode::CONFLICT,
            Json(json!({ "detail": "Hint job lease does not match or has expired." })),
        )
            .into_response(),
        Err(error) => {
            tracing::error!(%error, "hint bridge settle failed");
            StatusCode::SERVICE_UNAVAILABLE.into_response()
        }
    }
}
