//! HTTP handlers for the physician-facing patient memory surface
//! (`Memory.Code.20` and `Memory.Code.22`).
//!
//! `GET /patient-workspaces/{id}/memory` exposes only `summary.md` and
//! `direction.md`; subject pages, `person.md`, and the dated log are Coda's
//! working files and are never served here.
//! `POST /patient-workspaces/{id}/memory` saves the physician's edit as a log
//! note in their words via `Store::save_physician_memory_edit` so patient
//! rounds fold it in, rather than overwriting the versioned files directly.

use axum::Json;
use axum::Router;
use axum::body::Body;
use axum::extract::{Extension, Path};
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use local_product::runtime::Scope;
use local_product::runtime::memory::{
    Day, MemoryError, MemoryPath, PatientKey, PhysicianMemoryDocument, PhysicianMemoryTarget,
};
use physician_wire::{
    MemoryDocumentWire, MemoryEditTarget, PatientMemoryResponse, SavePhysicianMemoryEditRequest,
    SavePhysicianMemoryEditResponse,
};
use serde_json::json;

use crate::care::ConnectedCare;
use crate::matching_reply::{publish_bus_resync, runtime_store_from_extensions};

pub(crate) fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route(
        "/patient-workspaces/{id}/memory",
        get(read_patient_memory).post(save_patient_memory_edit),
    )
}

fn map_document(doc: PhysicianMemoryDocument) -> MemoryDocumentWire {
    MemoryDocumentWire {
        version: doc.version,
        saved_at: doc.saved.to_string(),
        markdown: doc.markdown,
    }
}

fn map_target(target: MemoryEditTarget) -> PhysicianMemoryTarget {
    match target {
        MemoryEditTarget::Summary => PhysicianMemoryTarget::Summary,
        MemoryEditTarget::Direction => PhysicianMemoryTarget::Direction,
    }
}

async fn read_patient_memory(
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
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return Json(PatientMemoryResponse {
            patient_workspace_id: workspace,
            durable: false,
            summary: None,
            direction: None,
        })
        .into_response();
    };
    let identity = ConnectedCare::identity(extensions);
    let scope = Scope {
        practice: identity.practice_id,
        physician: identity.physician_id,
        workspace: workspace.clone(),
        chat: "physician-memory-view".to_owned(),
    };
    let Ok(key) = PatientKey::try_from(&scope) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    match store.read_physician_memory(&key).await {
        Ok(memory) => Json(PatientMemoryResponse {
            patient_workspace_id: workspace,
            durable: true,
            summary: memory.summary.map(map_document),
            direction: memory.direction.map(map_document),
        })
        .into_response(),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Patient memory is temporarily unavailable." })),
        )
            .into_response(),
    }
}

async fn save_patient_memory_edit(
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
    let Some(store) = runtime_store_from_extensions(extensions) else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Patient memory edits require durable storage." })),
        )
            .into_response();
    };
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<SavePhysicianMemoryEditRequest>(&bytes) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "detail": "Invalid patient memory edit request." })),
        )
            .into_response();
    };
    let identity = ConnectedCare::identity(extensions);
    let scope = Scope {
        practice: identity.practice_id,
        physician: identity.physician_id,
        workspace: workspace.clone(),
        chat: "physician-memory-edit".to_owned(),
    };
    let target = map_target(payload.target);
    match store
        .save_physician_memory_edit(&scope, target, &payload.text, &payload.idempotency_key)
        .await
    {
        Ok(note) => {
            publish_bus_resync(extensions).await;
            Json(SavePhysicianMemoryEditResponse {
                note_number: note.number.to_string(),
                log_path: MemoryPath::Log(Day::of(note.time)).to_string(),
                saved_at: note.time.to_rfc3339(),
            })
            .into_response()
        }
        Err(MemoryError::Refused(reason)) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "detail": format!("Not saved: {reason}") })),
        )
            .into_response(),
        Err(MemoryError::WithdrawnFence(reason)) => (
            StatusCode::CONFLICT,
            Json(json!({ "detail": format!("Not saved: {reason}") })),
        )
            .into_response(),
        Err(MemoryError::Database(_)) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "detail": "Not saved. Coda could not save that note right now." })),
        )
            .into_response(),
    }
}
