//! HTTP handlers and durable `Store` composition for the patient-page
//! `Follow-ups` section: listing, creating, editing, `Review now`, `Stop`,
//! `Stop and discuss`, `Resume`, `Complete`, `Cancel`, and `"Can't restart"`
//! resolution.

use std::collections::BTreeMap;
use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::body::Body;
use axum::extract::{Extension, Path};
use axum::http::{Extensions, Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use local_product::runtime::concerns::WorkspaceAccess;
use local_product::runtime::follow_ups::{
    CannotRestartResolution, FollowUp, FollowUpActor, FollowUpChangeNote, FollowUpCommandKey,
    FollowUpError, FollowUpId, FollowUpReviewSchedule, FollowUpStanding, FollowUpSummary,
    FollowUpUpdateEntry, FollowUpUpdateKind, PhysicianChangeFollowUp, PhysicianCreateFollowUp,
};
use local_product::runtime::reviews::{AdmittedPatientReview, PatientReviewError};
use local_product::runtime::{Input, Scope, Store};
use physician_auth::PostgresAuthStore;
use physician_bus::Event;
use physician_wire::follow_ups::{
    CannotRestartResolutionWire, CreateFollowUpBody, FollowUpActionCommandBody,
    FollowUpMutationResponseWire, FollowUpOriginWire, FollowUpStandingWire, FollowUpWire,
    PatientFollowUpsResponseWire, ResolveCannotRestartBody, SettleFollowUpBody, StopFollowUpBody,
    UpdateFollowUpBody,
};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::auth::{self, AuthContext};
use crate::care::ConnectedCare;
use crate::live::Live;
use crate::matching_reply::{
    MatchingReplyFollowUpsFault, SampleRuntimeAuthority, is_faulted, publish_bus_resync,
    runtime_store_from_extensions, unix_to_rfc3339, with_review_receiver_setup,
};

pub(crate) fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route(
            "/patient-workspaces/{id}/follow-ups",
            get(list_patient_follow_ups).post(create_patient_follow_up),
        )
        .route(
            "/patient-workspaces/{id}/follow-ups/{follow_up_id}",
            axum::routing::put(update_patient_follow_up),
        )
        .route(
            "/patient-workspaces/{id}/follow-ups/{follow_up_id}/review-now",
            post(review_follow_up_now),
        )
        .route(
            "/patient-workspaces/{id}/follow-ups/{follow_up_id}/stop",
            post(stop_follow_up),
        )
        .route(
            "/patient-workspaces/{id}/follow-ups/{follow_up_id}/stop-and-discuss",
            post(stop_and_discuss_follow_up),
        )
        .route(
            "/patient-workspaces/{id}/follow-ups/{follow_up_id}/resume",
            post(resume_follow_up),
        )
        .route(
            "/patient-workspaces/{id}/follow-ups/{follow_up_id}/complete",
            post(complete_follow_up),
        )
        .route(
            "/patient-workspaces/{id}/follow-ups/{follow_up_id}/cancel",
            post(cancel_follow_up),
        )
        .route(
            "/patient-workspaces/{id}/follow-ups/{follow_up_id}/resolve-cannot-restart",
            post(resolve_cannot_restart),
        )
}

fn deterministic_hex_id(prefix: &str, material: &str) -> String {
    let digest = Sha256::digest(material.as_bytes());
    let mut hex = String::with_capacity(24);
    for byte in &digest[..12] {
        use std::fmt::Write as _;
        let _ = write!(&mut hex, "{byte:02x}");
    }
    format!("{prefix}-{hex}")
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn unix_days_from_civil(year: i32, month: u32, day: u32) -> Option<i64> {
    if !(1970..=9999).contains(&year) || !(1..=12).contains(&month) {
        return None;
    }
    let max_day = days_in_month(year, month);
    if day < 1 || day > max_day {
        return None;
    }
    let adjusted_year = i64::from(year) - i64::from(month <= 2);
    let era = if adjusted_year >= 0 {
        adjusted_year
    } else {
        adjusted_year - 399
    } / 400;
    let yoe = (adjusted_year - era * 400) as u64;
    let m = u64::from(month);
    let d = u64::from(day);
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some(era * 146_097 + doe as i64 - 719_468)
}

/// Parses a `YYYY-MM-DD` calendar date (normalized to `13:00:00Z`) or an
/// RFC 3339 UTC timestamp (`YYYY-MM-DDTHH:MM:SS...Z`) into positive Unix seconds.
pub(crate) fn parse_due_date_unix(raw: &str) -> Result<i64, String> {
    let trimmed = raw.trim();
    if trimmed.len() < 10 {
        return Err("due date must be YYYY-MM-DD".to_owned());
    }
    let date_part = &trimmed[..10];
    let mut parts = date_part.split('-');
    let year: i32 = parts
        .next()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| "invalid year in due date".to_owned())?;
    let month: u32 = parts
        .next()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| "invalid month in due date".to_owned())?;
    let day: u32 = parts
        .next()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| "invalid day in due date".to_owned())?;
    if parts.next().is_some() {
        return Err("invalid due date format".to_owned());
    }
    let days = unix_days_from_civil(year, month, day)
        .ok_or_else(|| "due date is out of range".to_owned())?;

    if trimmed.len() == 10 {
        let unix = days * 86_400 + 13 * 3_600;
        if unix <= 0 {
            return Err("due date must be after 1970-01-01".to_owned());
        }
        return Ok(unix);
    }

    let rest = &trimmed[10..];
    let Some(time_part) = rest.strip_prefix('T').or_else(|| rest.strip_prefix('t')) else {
        return Err("invalid due timestamp separator".to_owned());
    };
    let time_no_tz = time_part
        .strip_suffix('Z')
        .or_else(|| time_part.strip_suffix('z'))
        .ok_or_else(|| "timestamp due dates must use UTC Z suffix or YYYY-MM-DD".to_owned())?;
    let hms = time_no_tz.split('.').next().unwrap_or(time_no_tz);
    let mut tparts = hms.split(':');
    let hour: i64 = tparts
        .next()
        .and_then(|s| s.parse().ok())
        .filter(|h: &i64| (0..=23).contains(h))
        .ok_or_else(|| "invalid hour in due timestamp".to_owned())?;
    let minute: i64 = tparts
        .next()
        .and_then(|s| s.parse().ok())
        .filter(|m: &i64| (0..=59).contains(m))
        .ok_or_else(|| "invalid minute in due timestamp".to_owned())?;
    let second: i64 = tparts
        .next()
        .and_then(|s| s.parse().ok())
        .filter(|s: &i64| (0..=59).contains(s))
        .ok_or_else(|| "invalid second in due timestamp".to_owned())?;
    if tparts.next().is_some() {
        return Err("invalid due timestamp".to_owned());
    }
    let unix = days * 86_400 + hour * 3_600 + minute * 60 + second;
    if unix <= 0 {
        return Err("due timestamp must be positive".to_owned());
    }
    Ok(unix)
}

fn format_due_date_input(unix_secs: i64) -> String {
    let iso = unix_to_rfc3339(unix_secs);
    iso.get(..10).unwrap_or("1970-01-01").to_owned()
}

fn follow_up_error_response(err: FollowUpError) -> Response {
    match err {
        FollowUpError::AccessDenied { .. } => (
            StatusCode::FORBIDDEN,
            Json(json!({ "code": "patient_unavailable", "detail": "Patient access is no longer available." })),
        )
            .into_response(),
        FollowUpError::NotFound { .. } => (
            StatusCode::NOT_FOUND,
            Json(json!({ "code": "follow_up_not_found", "detail": err.to_string() })),
        )
            .into_response(),
        FollowUpError::InvalidInput(_) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "detail": format!("Not saved: {err}") })),
        )
            .into_response(),
        FollowUpError::AlreadyExists { .. }
        | FollowUpError::Terminal { .. }
        | FollowUpError::InvalidTransition { .. }
        | FollowUpError::UnverifiedRecord { .. }
        | FollowUpError::IdempotencyCollision { .. }
        | FollowUpError::WithdrawnFence(_)
        | FollowUpError::CancelledDuplicateWithoutNewInfo { .. } => (
            StatusCode::CONFLICT,
            Json(json!({ "detail": format!("Not saved: {err}") })),
        )
            .into_response(),
        FollowUpError::Database(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not save that follow-up right now." })),
        )
            .into_response(),
    }
}

fn map_follow_up_wire(item: FollowUp, latest_note: Option<String>) -> FollowUpWire {
    let standing = match item.standing {
        FollowUpStanding::Active {
            schedule: FollowUpReviewSchedule::Scheduled { due_at },
        } => FollowUpStandingWire::Scheduled {
            due_at: unix_to_rfc3339(due_at),
            due_date_input: format_due_date_input(due_at),
        },
        FollowUpStanding::Active {
            schedule:
                FollowUpReviewSchedule::DueNow {
                    due_at,
                    requested_at,
                    ..
                },
        } => FollowUpStandingWire::DueNow {
            requested_at: unix_to_rfc3339(requested_at),
            due_date_input: format_due_date_input(due_at),
        },
        FollowUpStanding::Held {
            paused_due_at,
            held_at,
            reason,
            ..
        } => FollowUpStandingWire::Held {
            paused_at: unix_to_rfc3339(held_at),
            reason: reason.map(|r| r.as_str().to_owned()),
            due_date_input: format_due_date_input(paused_due_at),
        },
        FollowUpStanding::CannotRestart {
            due_at,
            failed_at,
            reason,
            ..
        } => FollowUpStandingWire::CannotRestart {
            reason,
            recorded_at: unix_to_rfc3339(failed_at),
            due_date_input: format_due_date_input(due_at),
        },
        FollowUpStanding::Completed {
            resolution,
            completed_at,
            ..
        } => FollowUpStandingWire::Completed {
            completed_at: unix_to_rfc3339(completed_at),
            resolution: resolution.as_str().to_owned(),
        },
        FollowUpStanding::Cancelled {
            reason,
            cancelled_at,
            ..
        } => FollowUpStandingWire::Cancelled {
            cancelled_at: unix_to_rfc3339(cancelled_at),
            reason: reason.as_str().to_owned(),
        },
        FollowUpStanding::WithdrawnWrongPatient {
            reason,
            withdrawn_at,
            ..
        } => FollowUpStandingWire::Cancelled {
            cancelled_at: unix_to_rfc3339(withdrawn_at),
            reason: reason.as_str().to_owned(),
        },
    };

    let origin = match item.origin {
        FollowUpActor::Physician { physician_id } => {
            FollowUpOriginWire::PhysicianCreated { physician_id }
        }
        FollowUpActor::CodaWork { source_work } => FollowUpOriginWire::CodaCreated {
            run_id: source_work.work_id,
        },
    };

    FollowUpWire {
        follow_up_id: item.id.as_str().to_owned(),
        patient_workspace_id: item.workspace,
        version: item.version,
        summary: item.summary.as_str().to_owned(),
        standing,
        origin,
        latest_note,
        updated_at: unix_to_rfc3339(item.updated_at),
    }
}

fn latest_notes_by_follow_up(updates: Vec<FollowUpUpdateEntry>) -> BTreeMap<String, String> {
    let mut notes = BTreeMap::new();
    for entry in updates {
        if matches!(
            entry.kind,
            FollowUpUpdateKind::Changed
                | FollowUpUpdateKind::Stopped
                | FollowUpUpdateKind::Resumed
                | FollowUpUpdateKind::ReviewRequestedNow
        ) {
            notes.insert(entry.follow_up_id.as_str().to_owned(), entry.detail);
        }
    }
    notes
}

struct RequestAccessContext {
    token: String,
    session_hash: Vec<u8>,
    real_now_unix: u64,
    practice_id: String,
    physician_id: String,
}

fn resolve_access_context(extensions: &Extensions) -> Option<RequestAccessContext> {
    if let Some(ctx) = extensions.get::<AuthContext>() {
        return Some(RequestAccessContext {
            token: ctx.token.clone(),
            session_hash: PostgresAuthStore::session_reference(&ctx.token),
            real_now_unix: auth::now(),
            practice_id: ctx.identity.practice_id.clone(),
            physician_id: ctx.identity.physician_id.clone(),
        });
    }
    None
}

async fn list_patient_follow_ups(
    Path(workspace): Path<String>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let extensions = request.extensions();
    if let Err(status) =
        crate::require_on_panel_in_practice(&connected, extensions, &workspace).await
    {
        return status.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::FollowUpRead) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Follow-ups are temporarily unavailable." })),
        )
            .into_response();
    }
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return Json(PatientFollowUpsResponseWire {
            patient_workspace_id: workspace,
            durable: false,
            follow_ups: Vec::new(),
        })
        .into_response();
    };
    let Some(req_ctx) = resolve_access_context(extensions) else {
        return Json(PatientFollowUpsResponseWire {
            patient_workspace_id: workspace,
            durable: false,
            follow_ups: Vec::new(),
        })
        .into_response();
    };

    let auth_store = connected.auth_store(extensions);
    let access = WorkspaceAccess {
        session_hash: &req_ctx.session_hash,
        practice: &req_ctx.practice_id,
        physician: &req_ctx.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &req_ctx.token,
        now_unix: req_ctx.real_now_unix,
    };

    let items = match store.physician_follow_ups(&access, &mut authority).await {
        Ok(items) => items,
        Err(err) => return follow_up_error_response(err),
    };
    let notes = if items.is_empty() {
        BTreeMap::new()
    } else {
        let updates = match store
            .physician_follow_up_updates(&access, &mut authority)
            .await
        {
            Ok(updates) => updates,
            Err(err) => return follow_up_error_response(err),
        };
        latest_notes_by_follow_up(updates)
    };

    let mut follow_ups: Vec<FollowUpWire> = items
        .into_iter()
        .map(|item| {
            let note = notes.get(item.id.as_str()).cloned();
            map_follow_up_wire(item, note)
        })
        .collect();
    follow_ups.sort_by_key(|item| (!item.standing.is_open(), item.follow_up_id.clone()));

    Json(PatientFollowUpsResponseWire {
        patient_workspace_id: workspace,
        durable: true,
        follow_ups,
    })
    .into_response()
}

async fn create_patient_follow_up(
    Path(workspace): Path<String>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if let Err(status) =
        crate::require_on_panel_in_practice(&connected, extensions, &workspace).await
    {
        return status.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::FollowUpCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not create that follow-up right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<CreateFollowUpBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Some(req_ctx) = resolve_access_context(extensions) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let due_at = match parse_due_date_unix(&payload.due_date) {
        Ok(ts) => ts,
        Err(msg) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "detail": format!("Not saved: {msg}") })),
            )
                .into_response();
        }
    };
    let summary = match FollowUpSummary::new(&payload.summary) {
        Ok(s) => s,
        Err(err) => return follow_up_error_response(err.into()),
    };
    let command_key = match FollowUpCommandKey::new(&payload.idempotency_key) {
        Ok(k) => k,
        Err(err) => return follow_up_error_response(err.into()),
    };
    let follow_up_id_raw = deterministic_hex_id(
        "fu",
        &format!(
            "{}:{}:{}",
            req_ctx.practice_id, workspace, payload.idempotency_key
        ),
    );
    let follow_up_id = match FollowUpId::new(follow_up_id_raw) {
        Ok(id) => id,
        Err(err) => return follow_up_error_response(err.into()),
    };

    let auth_store = connected.auth_store(extensions);
    let access = WorkspaceAccess {
        session_hash: &req_ctx.session_hash,
        practice: &req_ctx.practice_id,
        physician: &req_ctx.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &req_ctx.token,
        now_unix: req_ctx.real_now_unix,
    };

    let outcome = match store
        .create_follow_up_for_physician(
            &access,
            &command_key,
            &PhysicianCreateFollowUp {
                id: follow_up_id,
                summary,
                due_at,
                records: Vec::new(),
            },
            &mut authority,
        )
        .await
    {
        Ok(outcome) => outcome,
        Err(err) => return follow_up_error_response(err),
    };
    publish_bus_resync(extensions).await;

    Json(FollowUpMutationResponseWire {
        follow_up: map_follow_up_wire(outcome.follow_up().clone(), None),
        review_started: false,
        discuss_chat_id: None,
    })
    .into_response()
}

async fn update_patient_follow_up(
    Path((workspace, follow_up_id_raw)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if let Err(status) =
        crate::require_on_panel_in_practice(&connected, extensions, &workspace).await
    {
        return status.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::FollowUpCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(
                json!({ "detail": "Not saved. Coda could not save follow-up changes right now." }),
            ),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<UpdateFollowUpBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Some(req_ctx) = resolve_access_context(extensions) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let follow_up_id = match FollowUpId::new(&follow_up_id_raw) {
        Ok(id) => id,
        Err(err) => return follow_up_error_response(err.into()),
    };
    let command_key = match FollowUpCommandKey::new(&payload.idempotency_key) {
        Ok(k) => k,
        Err(err) => return follow_up_error_response(err.into()),
    };

    let auth_store = connected.auth_store(extensions);
    let access = WorkspaceAccess {
        session_hash: &req_ctx.session_hash,
        practice: &req_ctx.practice_id,
        physician: &req_ctx.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &req_ctx.token,
        now_unix: req_ctx.real_now_unix,
    };

    let existing_list = match store.physician_follow_ups(&access, &mut authority).await {
        Ok(list) => list,
        Err(err) => return follow_up_error_response(err),
    };
    let Some(current) = existing_list
        .into_iter()
        .find(|item| item.id == follow_up_id)
    else {
        return follow_up_error_response(FollowUpError::NotFound { id: follow_up_id });
    };

    let summary = match payload.summary.as_deref() {
        Some(raw) => match FollowUpSummary::new(raw) {
            Ok(s) => s,
            Err(err) => return follow_up_error_response(err.into()),
        },
        None => current.summary.clone(),
    };
    let due_at = match payload.due_date.as_deref() {
        Some(raw) => match parse_due_date_unix(raw) {
            Ok(ts) => ts,
            Err(msg) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "detail": format!("Not saved: {msg}") })),
                )
                    .into_response();
            }
        },
        None => current.due_at,
    };
    let note_raw = match payload.note.as_deref() {
        Some(n) if !n.trim().is_empty() => n,
        _ => "Updated follow-up",
    };
    let note = match FollowUpChangeNote::new(note_raw) {
        Ok(n) => n,
        Err(err) => return follow_up_error_response(err.into()),
    };

    let outcome = match store
        .change_follow_up_for_physician(
            &access,
            &command_key,
            &PhysicianChangeFollowUp {
                id: follow_up_id,
                summary,
                due_at,
                records: current.records,
                note: note.clone(),
            },
            &mut authority,
        )
        .await
    {
        Ok(outcome) => outcome,
        Err(err) => return follow_up_error_response(err),
    };
    publish_bus_resync(extensions).await;

    Json(FollowUpMutationResponseWire {
        follow_up: map_follow_up_wire(outcome.follow_up().clone(), Some(note.as_str().to_owned())),
        review_started: false,
        discuss_chat_id: None,
    })
    .into_response()
}

async fn queue_physician_follow_up_review(
    store: &Store,
    connected: &ConnectedCare,
    extensions: &Extensions,
    workspace: &str,
    follow_up: &FollowUp,
    idempotency_key: &str,
) -> Result<bool, PatientReviewError> {
    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    let (roles, skills) = with_review_receiver_setup(store, extensions).await;
    let review_cmd_key = deterministic_hex_id(
        "furev",
        &format!(
            "{}:{}:{}:{}",
            identity.practice_id,
            workspace,
            follow_up.id.as_str(),
            idempotency_key
        ),
    );
    match store
        .submit_physician_follow_up_review(
            auth_store.as_ref(),
            &identity,
            workspace,
            &review_cmd_key,
            follow_up,
            &roles,
            &skills,
        )
        .await
    {
        Ok(AdmittedPatientReview::New { .. }) => {
            if let Some(live) = extensions.get::<Arc<Live>>() {
                let _ = live
                    .bus
                    .publish(physician_bus::Event::WorkspaceChanged {
                        workspace_id: workspace.to_owned(),
                    })
                    .await;
            }
            Ok(true)
        }
        Ok(AdmittedPatientReview::Duplicate { .. }) => Ok(true),
        Err(err) => {
            tracing::warn!(%err, "physician follow-up review submission failed");
            Err(err)
        }
    }
}

async fn review_follow_up_now(
    Path((workspace, follow_up_id_raw)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if let Err(status) =
        crate::require_on_panel_in_practice(&connected, extensions, &workspace).await
    {
        return status.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::FollowUpCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not start that review right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<FollowUpActionCommandBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Some(req_ctx) = resolve_access_context(extensions) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let follow_up_id = match FollowUpId::new(&follow_up_id_raw) {
        Ok(id) => id,
        Err(err) => return follow_up_error_response(err.into()),
    };
    let command_key = match FollowUpCommandKey::new(&payload.idempotency_key) {
        Ok(k) => k,
        Err(err) => return follow_up_error_response(err.into()),
    };

    let auth_store = connected.auth_store(extensions);
    let access = WorkspaceAccess {
        session_hash: &req_ctx.session_hash,
        practice: &req_ctx.practice_id,
        physician: &req_ctx.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &req_ctx.token,
        now_unix: req_ctx.real_now_unix,
    };

    let outcome = match store
        .review_follow_up_now_for_physician(&access, &command_key, &follow_up_id, &mut authority)
        .await
    {
        Ok(outcome) => outcome,
        Err(err) => return follow_up_error_response(err),
    };

    let saved = outcome.follow_up().clone();
    let review_started = match queue_physician_follow_up_review(
        &store,
        &connected,
        extensions,
        &workspace,
        &saved,
        &payload.idempotency_key,
    )
    .await
    {
        Ok(started) => started,
        Err(err) => {
            publish_bus_resync(extensions).await;
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "detail": format!(
                        "Follow-up was marked due for immediate review, but Coda could not start the review right now ({err})."
                    )
                })),
            )
                .into_response();
        }
    };
    publish_bus_resync(extensions).await;

    Json(FollowUpMutationResponseWire {
        follow_up: map_follow_up_wire(saved, Some("Immediate review requested".to_owned())),
        review_started,
        discuss_chat_id: None,
    })
    .into_response()
}

async fn stop_follow_up(
    Path((workspace, follow_up_id_raw)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if let Err(status) =
        crate::require_on_panel_in_practice(&connected, extensions, &workspace).await
    {
        return status.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::FollowUpCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not pause that follow-up right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<StopFollowUpBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Some(req_ctx) = resolve_access_context(extensions) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let follow_up_id = match FollowUpId::new(&follow_up_id_raw) {
        Ok(id) => id,
        Err(err) => return follow_up_error_response(err.into()),
    };
    let command_key = match FollowUpCommandKey::new(&payload.idempotency_key) {
        Ok(k) => k,
        Err(err) => return follow_up_error_response(err.into()),
    };
    let reason = match payload.reason.as_deref().filter(|s| !s.trim().is_empty()) {
        Some(r) => match FollowUpChangeNote::new(r) {
            Ok(note) => Some(note),
            Err(err) => return follow_up_error_response(err.into()),
        },
        None => None,
    };

    let auth_store = connected.auth_store(extensions);
    let access = WorkspaceAccess {
        session_hash: &req_ctx.session_hash,
        practice: &req_ctx.practice_id,
        physician: &req_ctx.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &req_ctx.token,
        now_unix: req_ctx.real_now_unix,
    };

    let outcome = match store
        .stop_follow_up_for_physician(
            &access,
            &command_key,
            &follow_up_id,
            reason.clone(),
            &mut authority,
        )
        .await
    {
        Ok(outcome) => outcome,
        Err(err) => return follow_up_error_response(err),
    };
    publish_bus_resync(extensions).await;

    Json(FollowUpMutationResponseWire {
        follow_up: map_follow_up_wire(
            outcome.follow_up().clone(),
            reason.map(|r| r.as_str().to_owned()),
        ),
        review_started: false,
        discuss_chat_id: None,
    })
    .into_response()
}

async fn stop_and_discuss_follow_up(
    Path((workspace, follow_up_id_raw)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if let Err(status) =
        crate::require_on_panel_in_practice(&connected, extensions, &workspace).await
    {
        return status.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::FollowUpCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not pause and open discussion right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<StopFollowUpBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Some(req_ctx) = resolve_access_context(extensions) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let follow_up_id = match FollowUpId::new(&follow_up_id_raw) {
        Ok(id) => id,
        Err(err) => return follow_up_error_response(err.into()),
    };
    let command_key = match FollowUpCommandKey::new(&payload.idempotency_key) {
        Ok(k) => k,
        Err(err) => return follow_up_error_response(err.into()),
    };
    let reason = match payload.reason.as_deref().filter(|s| !s.trim().is_empty()) {
        Some(r) => match FollowUpChangeNote::new(r) {
            Ok(note) => Some(note),
            Err(err) => return follow_up_error_response(err.into()),
        },
        None => None,
    };

    let auth_store = connected.auth_store(extensions);
    let access = WorkspaceAccess {
        session_hash: &req_ctx.session_hash,
        practice: &req_ctx.practice_id,
        physician: &req_ctx.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &req_ctx.token,
        now_unix: req_ctx.real_now_unix,
    };

    let outcome = match store
        .stop_follow_up_for_physician(
            &access,
            &command_key,
            &follow_up_id,
            reason.clone(),
            &mut authority,
        )
        .await
    {
        Ok(outcome) => outcome,
        Err(err) => return follow_up_error_response(err),
    };
    let saved = outcome.follow_up().clone();

    let chat_id = deterministic_hex_id(
        "followup-discuss",
        &format!(
            "{}:{}:{}:{}",
            req_ctx.practice_id,
            workspace,
            saved.id.as_str(),
            payload.idempotency_key
        ),
    );
    let scope = Scope {
        practice: req_ctx.practice_id.clone(),
        physician: req_ctx.physician_id.clone(),
        workspace: workspace.clone(),
        chat: chat_id.clone(),
    };

    let accepted_chat_result: Result<String, String> =
        if is_faulted(extensions, MatchingReplyFollowUpsFault::StopAndDiscussChat) {
            Err("discussion chat unavailable".to_owned())
        } else if let Some(physician_question) = &reason {
            // Physician composed an explicit question/note: start the discussion
            // conversation with the physician's own words and practice timezone.
            let timezone = match payload
                .physician_time_zone
                .as_deref()
                .map(str::trim)
                .filter(|tz| !tz.is_empty())
            {
                Some(tz) => tz.to_owned(),
                None => match connected.practice_moment(extensions).await {
                    Ok(moment) => moment.time_zone.as_str().to_owned(),
                    Err(_) => "UTC".to_owned(),
                },
            };
            let turn_input = Input {
                key: format!("fudisc-turn:{}", payload.idempotency_key),
                text: physician_question.as_str().to_owned(),
                timezone,
                scenario: None,
            };
            if let Some(live) = extensions.get::<Arc<Live>>() {
                match live
                    .chat
                    .accept(&scope, &turn_input, req_ctx.session_hash.clone())
                    .await
                {
                    Ok(_) => {
                        let _ = live
                            .bus
                            .publish(Event::WorkspaceChanged {
                                workspace_id: workspace.clone(),
                            })
                            .await;
                        Ok(chat_id)
                    }
                    Err(err) => {
                        tracing::warn!(%err, "follow-up discuss chat accept failed");
                        Err(err.to_string())
                    }
                }
            } else {
                let (roles, skills) = with_review_receiver_setup(&store, extensions).await;
                match store
                    .chat_recipe_with_understanding(
                        &roles.chat,
                        local_product::physician_chat::INSTRUCTIONS,
                        &skills,
                        &scope.practice,
                        &scope.workspace,
                    )
                    .await
                {
                    Ok(recipe) => store
                        .accept_authorized(
                            &scope,
                            &turn_input,
                            req_ctx.session_hash.clone(),
                            &skills,
                            &recipe,
                            crate::durable::request_context(),
                        )
                        .await
                        .map(|_| chat_id)
                        .map_err(|err| err.to_string()),
                    Err(err) => Err(err.to_string()),
                }
            }
        } else {
            // No physician message supplied yet: open a fresh discussion composer
            // with an editable draft instead of auto-sending a canned AI turn.
            let draft_text = format!("Regarding held follow-up \"{}\": ", saved.summary.as_str());
            match store
                .save_chat_draft(
                    &scope,
                    0,
                    &draft_text,
                    &req_ctx.session_hash,
                    &mut authority,
                )
                .await
            {
                Ok(_) => {
                    if let Some(live) = extensions.get::<Arc<Live>>() {
                        let _ = live
                            .bus
                            .publish(Event::WorkspaceChanged {
                                workspace_id: workspace.clone(),
                            })
                            .await;
                    }
                    Ok(chat_id)
                }
                Err(err) => {
                    tracing::warn!(%err, "follow-up discuss draft save failed");
                    Err(err.to_string())
                }
            }
        };

    publish_bus_resync(extensions).await;

    let accepted_chat_id = match accepted_chat_result {
        Ok(id) => id,
        Err(err) => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "detail": format!(
                        "Not saved: Follow-up was paused, but Coda could not start the discussion conversation ({err})."
                    )
                })),
            )
                .into_response();
        }
    };

    Json(FollowUpMutationResponseWire {
        follow_up: map_follow_up_wire(saved, reason.map(|r| r.as_str().to_owned())),
        review_started: false,
        discuss_chat_id: Some(accepted_chat_id),
    })
    .into_response()
}

async fn resume_follow_up(
    Path((workspace, follow_up_id_raw)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if let Err(status) =
        crate::require_on_panel_in_practice(&connected, extensions, &workspace).await
    {
        return status.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::FollowUpCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not resume that follow-up right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<FollowUpActionCommandBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Some(req_ctx) = resolve_access_context(extensions) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let follow_up_id = match FollowUpId::new(&follow_up_id_raw) {
        Ok(id) => id,
        Err(err) => return follow_up_error_response(err.into()),
    };
    let command_key = match FollowUpCommandKey::new(&payload.idempotency_key) {
        Ok(k) => k,
        Err(err) => return follow_up_error_response(err.into()),
    };

    let auth_store = connected.auth_store(extensions);
    let access = WorkspaceAccess {
        session_hash: &req_ctx.session_hash,
        practice: &req_ctx.practice_id,
        physician: &req_ctx.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &req_ctx.token,
        now_unix: req_ctx.real_now_unix,
    };

    let outcome = match store
        .resume_follow_up_for_physician(&access, &command_key, &follow_up_id, &mut authority)
        .await
    {
        Ok(outcome) => outcome,
        Err(err) => return follow_up_error_response(err),
    };
    let saved = outcome.follow_up().clone();
    let review_started = match queue_physician_follow_up_review(
        &store,
        &connected,
        extensions,
        &workspace,
        &saved,
        &payload.idempotency_key,
    )
    .await
    {
        Ok(started) => started,
        Err(err) => {
            publish_bus_resync(extensions).await;
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "detail": format!(
                        "Follow-up was resumed, but Coda could not start the review right now ({err})."
                    )
                })),
            )
                .into_response();
        }
    };
    publish_bus_resync(extensions).await;

    Json(FollowUpMutationResponseWire {
        follow_up: map_follow_up_wire(
            saved,
            Some("Follow-up resumed and marked due for review".to_owned()),
        ),
        review_started,
        discuss_chat_id: None,
    })
    .into_response()
}

async fn complete_follow_up(
    Path((workspace, follow_up_id_raw)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if let Err(status) =
        crate::require_on_panel_in_practice(&connected, extensions, &workspace).await
    {
        return status.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::FollowUpCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(
                json!({ "detail": "Not saved. Coda could not complete that follow-up right now." }),
            ),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<SettleFollowUpBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Some(req_ctx) = resolve_access_context(extensions) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let follow_up_id = match FollowUpId::new(&follow_up_id_raw) {
        Ok(id) => id,
        Err(err) => return follow_up_error_response(err.into()),
    };
    let command_key = match FollowUpCommandKey::new(&payload.idempotency_key) {
        Ok(k) => k,
        Err(err) => return follow_up_error_response(err.into()),
    };
    let note_raw = if payload.note.trim().is_empty() {
        "Completed by physician"
    } else {
        payload.note.as_str()
    };
    let resolution = match FollowUpChangeNote::new(note_raw) {
        Ok(n) => n,
        Err(err) => return follow_up_error_response(err.into()),
    };

    let auth_store = connected.auth_store(extensions);
    let access = WorkspaceAccess {
        session_hash: &req_ctx.session_hash,
        practice: &req_ctx.practice_id,
        physician: &req_ctx.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &req_ctx.token,
        now_unix: req_ctx.real_now_unix,
    };

    let outcome = match store
        .complete_follow_up_for_physician(
            &access,
            &command_key,
            &follow_up_id,
            resolution,
            &mut authority,
        )
        .await
    {
        Ok(outcome) => outcome,
        Err(err) => return follow_up_error_response(err),
    };
    publish_bus_resync(extensions).await;

    Json(FollowUpMutationResponseWire {
        follow_up: map_follow_up_wire(outcome.follow_up().clone(), None),
        review_started: false,
        discuss_chat_id: None,
    })
    .into_response()
}

async fn cancel_follow_up(
    Path((workspace, follow_up_id_raw)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if let Err(status) =
        crate::require_on_panel_in_practice(&connected, extensions, &workspace).await
    {
        return status.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::FollowUpCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not cancel that follow-up right now." })),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<SettleFollowUpBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Some(req_ctx) = resolve_access_context(extensions) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let follow_up_id = match FollowUpId::new(&follow_up_id_raw) {
        Ok(id) => id,
        Err(err) => return follow_up_error_response(err.into()),
    };
    let command_key = match FollowUpCommandKey::new(&payload.idempotency_key) {
        Ok(k) => k,
        Err(err) => return follow_up_error_response(err.into()),
    };
    let note_raw = if payload.note.trim().is_empty() {
        "Cancelled by physician"
    } else {
        payload.note.as_str()
    };
    let reason = match FollowUpChangeNote::new(note_raw) {
        Ok(n) => n,
        Err(err) => return follow_up_error_response(err.into()),
    };

    let auth_store = connected.auth_store(extensions);
    let access = WorkspaceAccess {
        session_hash: &req_ctx.session_hash,
        practice: &req_ctx.practice_id,
        physician: &req_ctx.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &req_ctx.token,
        now_unix: req_ctx.real_now_unix,
    };

    let outcome = match store
        .cancel_follow_up_for_physician(
            &access,
            &command_key,
            &follow_up_id,
            reason,
            &mut authority,
        )
        .await
    {
        Ok(outcome) => outcome,
        Err(err) => return follow_up_error_response(err),
    };
    publish_bus_resync(extensions).await;

    Json(FollowUpMutationResponseWire {
        follow_up: map_follow_up_wire(outcome.follow_up().clone(), None),
        review_started: false,
        discuss_chat_id: None,
    })
    .into_response()
}

async fn resolve_cannot_restart(
    Path((workspace, follow_up_id_raw)): Path<(String, String)>,
    Extension(connected): Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let extensions = &parts.extensions;
    if let Err(status) =
        crate::require_on_panel_in_practice(&connected, extensions, &workspace).await
    {
        return status.into_response();
    }
    if is_faulted(extensions, MatchingReplyFollowUpsFault::FollowUpCommand) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(
                json!({ "detail": "Not saved. Coda could not resolve that follow-up right now." }),
            ),
        )
            .into_response();
    }
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<ResolveCannotRestartBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Some(req_ctx) = resolve_access_context(extensions) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let follow_up_id = match FollowUpId::new(&follow_up_id_raw) {
        Ok(id) => id,
        Err(err) => return follow_up_error_response(err.into()),
    };
    let command_key = match FollowUpCommandKey::new(&payload.idempotency_key) {
        Ok(k) => k,
        Err(err) => return follow_up_error_response(err.into()),
    };

    let (resolution, latest_note) = match payload.resolution {
        CannotRestartResolutionWire::Complete { resolution } => {
            let raw = if resolution.trim().is_empty() {
                "Completed after review failure"
            } else {
                resolution.as_str()
            };
            let note = match FollowUpChangeNote::new(raw) {
                Ok(n) => n,
                Err(err) => return follow_up_error_response(err.into()),
            };
            (CannotRestartResolution::Complete { resolution: note }, None)
        }
        CannotRestartResolutionWire::NewDate { due_date, note } => {
            let due_at = match parse_due_date_unix(&due_date) {
                Ok(ts) => ts,
                Err(msg) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({ "detail": format!("Not saved: {msg}") })),
                    )
                        .into_response();
                }
            };
            let note_raw = match note.as_deref() {
                Some(n) if !n.trim().is_empty() => n,
                _ => "Scheduled a new review date",
            };
            let parsed_note = match FollowUpChangeNote::new(note_raw) {
                Ok(n) => n,
                Err(err) => return follow_up_error_response(err.into()),
            };
            let label = parsed_note.as_str().to_owned();
            (
                CannotRestartResolution::NewDate {
                    due_at,
                    note: parsed_note,
                },
                Some(label),
            )
        }
        CannotRestartResolutionWire::Cancel { reason } => {
            let raw = if reason.trim().is_empty() {
                "Cancelled after review failure"
            } else {
                reason.as_str()
            };
            let parsed_reason = match FollowUpChangeNote::new(raw) {
                Ok(n) => n,
                Err(err) => return follow_up_error_response(err.into()),
            };
            (
                CannotRestartResolution::Cancel {
                    reason: parsed_reason,
                },
                None,
            )
        }
    };

    let auth_store = connected.auth_store(extensions);
    let access = WorkspaceAccess {
        session_hash: &req_ctx.session_hash,
        practice: &req_ctx.practice_id,
        physician: &req_ctx.physician_id,
        workspace: &workspace,
    };
    let mut authority = SampleRuntimeAuthority {
        auth: auth_store.as_ref(),
        token: &req_ctx.token,
        now_unix: req_ctx.real_now_unix,
    };

    let outcome = match store
        .resolve_cannot_restart_for_physician(
            &access,
            &command_key,
            &follow_up_id,
            resolution,
            &mut authority,
        )
        .await
    {
        Ok(outcome) => outcome,
        Err(err) => return follow_up_error_response(err),
    };
    publish_bus_resync(extensions).await;

    Json(FollowUpMutationResponseWire {
        follow_up: map_follow_up_wire(outcome.follow_up().clone(), latest_note),
        review_started: false,
        discuss_chat_id: None,
    })
    .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_date_and_rfc3339_parse_roundtrip() {
        let ts = parse_due_date_unix("2026-09-15").expect("parse YYYY-MM-DD");
        assert_eq!(format_due_date_input(ts), "2026-09-15");
        let ts_rfc =
            parse_due_date_unix("2026-09-15T18:30:00.000Z").expect("parse RFC 3339 timestamp");
        assert_eq!(format_due_date_input(ts_rfc), "2026-09-15");
        assert!(parse_due_date_unix("2026-02-30").is_err());
    }
}
