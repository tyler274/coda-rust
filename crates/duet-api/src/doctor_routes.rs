//! Durable For my doctor collection and device-snapshot routes.
use crate::{ApiError, durable::DurableState, privacy::Restriction, workspace::WorkspaceStore};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{delete, get, post, put},
};
use serde_json::Value;
use uuid::Uuid;
/// Backend composition for authenticated doctor collection routes.
#[derive(Clone)]
pub struct DoctorState {
    /// Unified account/evidence/privacy owners.
    pub durable: DurableState,
    /// Durable collection workspace.
    pub workspace: WorkspaceStore,
}
/// Compose the durable collection and device-snapshot routes; retired PDF, SHL,
/// public link, and AI-proposal routes remain absent.
pub fn router(state: DoctorState) -> Router {
    Router::new()
        .route("/api/doctor-collection", get(read).delete(clear))
        .route("/api/doctor-preparation", get(legacy_preparation))
        .route("/api/doctor-collection/record", put(select_record))
        .route("/api/doctor-collection/facts/{id}", put(select_fact))
        .route("/api/doctor-collection/page", put(save))
        .route(
            "/api/doctor-collection/save-receipts/{generation}",
            get(save_receipt),
        )
        .route("/api/doctor-collection/order", put(order))
        .route("/api/doctor-collection/note", put(note))
        .route("/api/doctor-collection/page/share", post(check))
        .route("/api/doctor-collection/items/{id}", delete(remove))
        .route(
            "/api/health-context",
            put(upload_device).get(device_snapshot),
        )
        .with_state(state)
}
async fn save_receipt(
    State(s): State<DoctorState>,
    h: HeaderMap,
    Path(generation): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    s.durable.privacy.confirm_account(c.account_id).await?;
    Ok(Json(s.workspace.save_receipt(&c, generation).await?))
}
async fn read(State(s): State<DoctorState>, h: HeaderMap) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(s.workspace.read(&c).await?))
}
async fn legacy_preparation(
    State(s): State<DoctorState>,
    h: HeaderMap,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    let collection = s.workspace.read(&c).await?;
    Ok(Json(crate::workspace::legacy_preparation(&collection)))
}
async fn select_record(
    State(s): State<DoctorState>,
    h: HeaderMap,
    Json(body): Json<duet_wire::models::CanonicalRecordSelection>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(s.workspace.select_record(&c, &body).await?))
}
async fn select_fact(
    State(s): State<DoctorState>,
    h: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(s.workspace.select_fact(&c, &id).await?))
}
async fn device_snapshot(
    State(s): State<DoctorState>,
    h: HeaderMap,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(s.workspace.device_snapshot(&c).await?))
}
async fn upload_device(
    State(s): State<DoctorState>,
    h: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(s.workspace.upload_device(&c, &body).await?))
}
async fn order(
    State(s): State<DoctorState>,
    h: HeaderMap,
    Json(body): Json<duet_wire::models::DoctorCollectionOrderUpdate>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(s.workspace.reorder(&c, &body.item_ids).await?))
}
async fn note(
    State(s): State<DoctorState>,
    h: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    let current = s.workspace.read(&c).await?;
    let generation = body["expected_generation"]
        .as_i64()
        .ok_or(ApiError::Unprocessable("expected_generation required"))?;
    let ids = current["items"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|i| {
            serde_json::from_value(i["id"].clone()).map_err(|_| ApiError::Conflict("invalid item"))
        })
        .collect::<Result<Vec<Uuid>, _>>()?;
    let confirmed = Some(
        current["items"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|i| i["medication_confirmed"] == true)
            .filter_map(|i| serde_json::from_value(i["id"].clone()).ok())
            .collect(),
    );
    let mut request = duet_wire::models::DoctorPageSave::new(
        generation as i32,
        ids,
        serde_json::from_value(current["page"].clone())
            .map_err(|_| ApiError::Conflict("invalid page"))?,
    );
    request.confirmed_medication_ids = confirmed;
    request.note = Some(body["note"].as_str().map(str::to_owned));
    let result = s.workspace.save(&c, &request).await?;
    s.durable.privacy.confirm_account(c.account_id).await?;
    Ok(Json(result))
}
async fn save(
    State(s): State<DoctorState>,
    h: HeaderMap,
    Json(body): Json<duet_wire::models::DoctorPageSave>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    let result = s.workspace.save(&c, &body).await?;
    s.durable.privacy.confirm_account(c.account_id).await?;
    Ok(Json(result))
}
async fn check(
    State(s): State<DoctorState>,
    h: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    let result = s.workspace.approved_read(&c).await?;
    if result["note_generation"] != body["expected_generation"]
        || result["items"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|i| i["review_required"] == true)
    {
        return Err(ApiError::Conflict("page or evidence changed"));
    }
    Ok(Json(result))
}
fn operation(h: &HeaderMap) -> Result<Uuid, ApiError> {
    h.get("Idempotency-Key")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
        .ok_or(ApiError::Unprocessable(
            "Idempotency-Key UUID required for privacy action",
        ))
}
async fn clear(State(s): State<DoctorState>, h: HeaderMap) -> Result<StatusCode, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    let generation = h
        .get("If-Match")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<i64>().ok())
        .ok_or(ApiError::Unprocessable(
            "If-Match generation required for Start fresh",
        ))?;
    s.durable
        .privacy
        .restrict(
            &c,
            operation(&h)?,
            Restriction::DoctorPageDelete,
            c.account_id,
            generation,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
async fn remove(
    State(s): State<DoctorState>,
    h: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    s.durable
        .privacy
        .restrict(&c, operation(&h)?, Restriction::DoctorItemDelete, id, 0)
        .await?;
    Ok(Json(s.workspace.read(&c).await?))
}
