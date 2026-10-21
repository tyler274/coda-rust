//! Existing doctor workspace routes plus pinned publication/SMART Health Link delivery.
use crate::{
    ApiError,
    durable::DurableState,
    privacy::Restriction,
    publication::{PublicationStore, PublishRequest},
    workspace::{WorkspaceStore, approved},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
};
use serde_json::{Value, json};
use uuid::Uuid;
/// Backend composition shared by private controls and public capability routes.
#[derive(Clone)]
pub struct DoctorState {
    /// Unified account/evidence/privacy owners.
    pub durable: DurableState,
    /// Durable approved workspace.
    pub workspace: WorkspaceStore,
    /// Pinned publication owner.
    pub publications: PublicationStore,
}
/// Compose the existing DoctorCollection/DoctorPageSave routes, no model simulation.
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
        .route("/api/doctor-collection/proposals/{id}/reject", post(reject))
        .route("/api/doctor-collection/note", put(note))
        .route("/api/doctor-collection/page/share", post(check))
        .route("/api/doctor-collection/items/{id}", delete(remove))
        .route("/api/doctor-collection/preview", get(preview))
        .route("/api/doctor-collection/pdf", get(pdf))
        .route(
            "/api/doctor-collection/share-link",
            post(publish)
                .get(status)
                .delete(revoke)
                .options(publication_options)
                .layer(axum::middleware::from_fn(publication_cors)),
        )
        .route(
            "/api/health-context",
            put(upload_device).get(device_snapshot),
        )
        .route("/shared/for-my-doctor/{token}", get(web))
        .route("/shared/for-my-doctor/{token}/pdf", get(public_pdf))
        .route("/shl/{token}/manifest", post(manifest).options(shl_options))
        .route("/shl-file/{location}", get(shl_file).options(shl_options))
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
async fn reject(
    State(s): State<DoctorState>,
    h: HeaderMap,
    Path(id): Path<Uuid>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    let generation = body["expected_generation"]
        .as_i64()
        .ok_or(ApiError::Unprocessable("expected_generation required"))?;
    Ok(Json(s.workspace.reject_proposal(&c, id, generation).await?))
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
async fn preview(State(s): State<DoctorState>, h: HeaderMap) -> Result<Response, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    let page = s.workspace.read(&c).await?;
    let layout = duet_page::layout_approved(&approved(&page));
    Ok(private_response(
        "image/svg+xml",
        layout.preview_svg().into_bytes(),
    ))
}
async fn pdf(State(s): State<DoctorState>, h: HeaderMap) -> Result<Response, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    let page = s.workspace.approved_read(&c).await?;
    if page["items"]
        .as_array()
        .into_iter()
        .flatten()
        .any(|i| i["review_required"] == true)
    {
        return Err(ApiError::Conflict("selected evidence changed"));
    }
    let bytes = duet_page::layout_approved(&approved(&page))
        .pdf()
        .map_err(|_| ApiError::Conflict("page cannot export faithfully"))?;
    Ok(private_response("application/pdf", bytes))
}
async fn publish(
    State(s): State<DoctorState>,
    h: HeaderMap,
    Json(body): Json<PublishRequest>,
) -> Result<Json<crate::publication::PublicationView>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    let view = s.publications.publish(&c, &body).await?;
    s.durable.privacy.confirm_account(c.account_id).await?;
    Ok(Json(view))
}
const PUBLICATION_PRECONDITION: &str = "duet-publication-precondition";
fn publication_support(mut response: Response) -> Response {
    response
        .headers_mut()
        .insert(PUBLICATION_PRECONDITION, "uuid-v1".parse().unwrap());
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, "no-store".parse().unwrap());
    response
}
// Bearer-only endpoint: no cookie credentials are allowed. Match the existing
// public SHL cross-origin policy while explicitly allowing conditional headers.
async fn publication_cors(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Response {
    let mut response = next.run(request).await;
    let h = response.headers_mut();
    h.insert("access-control-allow-origin", "*".parse().unwrap());
    h.insert(
        "access-control-allow-methods",
        "GET, POST, DELETE, OPTIONS".parse().unwrap(),
    );
    h.insert(
        "access-control-allow-headers",
        "Authorization, Content-Type, If-Match, Idempotency-Key"
            .parse()
            .unwrap(),
    );
    h.insert(
        "access-control-expose-headers",
        "Duet-Publication-Precondition".parse().unwrap(),
    );
    response
}
async fn publication_options() -> StatusCode {
    StatusCode::NO_CONTENT
}
async fn status(State(s): State<DoctorState>, h: HeaderMap) -> Result<Response, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(publication_support(
        Json(json!(s.publications.status(&c).await?)).into_response(),
    ))
}
async fn revoke(State(s): State<DoctorState>, h: HeaderMap) -> Result<Response, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    let Some(target) = h.get(header::IF_MATCH) else {
        return Ok((
            StatusCode::PRECONDITION_REQUIRED,
            Json(json!({"detail":"If-Match reviewed publication UUID required"})),
        )
            .into_response());
    };
    if h.get_all(header::IF_MATCH).iter().count() != 1 {
        return Err(ApiError::Unprocessable(
            "exactly one If-Match publication UUID required",
        ));
    }
    let target = target
        .to_str()
        .ok()
        .and_then(|s| {
            // This version's contract is a bare canonical UUID, not an ETag list,
            // wildcard, weak validator, or quoted UUID.
            Uuid::parse_str(s)
                .ok()
                .filter(|id| id.to_string().eq_ignore_ascii_case(s))
        })
        .ok_or(ApiError::Unprocessable(
            "If-Match must be a bare publication UUID",
        ))?;
    if h.get_all("Idempotency-Key").iter().count() != 1 {
        return Err(ApiError::Unprocessable(
            "exactly one Idempotency-Key UUID required",
        ));
    }
    s.durable
        .privacy
        .restrict(
            &c,
            operation(&h)?,
            Restriction::PublicationRevoke,
            target,
            0,
        )
        .await?;
    Ok(publication_support(StatusCode::NO_CONTENT.into_response()))
}
async fn web(State(s): State<DoctorState>, Path(token): Path<String>) -> Response {
    match s.publications.retrieve(&token, false).await {
        Ok(p) => private_response(
            "text/html; charset=utf-8",
            crate::publication::html(&p.snapshot).into_bytes(),
        ),
        Err(_) => unavailable(),
    }
}
async fn public_pdf(State(s): State<DoctorState>, Path(token): Path<String>) -> Response {
    match s.publications.retrieve(&token, false).await {
        Ok(p) => private_response("application/pdf", p.pdf),
        Err(_) => unavailable(),
    }
}
fn shl_cors(mut response: Response) -> Response {
    response
        .headers_mut()
        .insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    response.headers_mut().insert(
        "Access-Control-Allow-Methods",
        "POST, GET, OPTIONS".parse().unwrap(),
    );
    response.headers_mut().insert(
        "Access-Control-Allow-Headers",
        "Content-Type".parse().unwrap(),
    );
    response
}
async fn shl_options() -> Response {
    let mut response = private_response("text/plain", Vec::new());
    *response.status_mut() = StatusCode::NO_CONTENT;
    shl_cors(response)
}
async fn manifest(
    State(s): State<DoctorState>,
    Path(token): Path<String>,
    Json(body): Json<Value>,
) -> Response {
    if !body.is_object() {
        return unavailable();
    }
    match s.publications.manifest(&token).await {
        Ok(v) => shl_cors(private_response(
            "application/json",
            v.to_string().into_bytes(),
        )),
        Err(_) => unavailable(),
    }
}
async fn shl_file(State(s): State<DoctorState>, Path(location): Path<String>) -> Response {
    match s.publications.shl_file(&location).await {
        Ok(jwe) => shl_cors(private_response("application/jose", jwe.into_bytes())),
        Err(_) => unavailable(),
    }
}
fn unavailable() -> Response {
    let mut r=private_response("text/html; charset=utf-8",b"<!doctype html><html lang=\"en\"><title>Link unavailable</title><main><h1>This link is unavailable</h1><p>It may have expired, been revoked, or need renewed patient review.</p></main></html>".to_vec());
    *r.status_mut() = StatusCode::NOT_FOUND;
    r
}
fn private_response(content_type: &str, bytes: Vec<u8>) -> Response {
    let mut r = bytes.into_response();
    let h = r.headers_mut();
    h.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
    h.insert(
        header::CACHE_CONTROL,
        "no-store, max-age=0".parse().unwrap(),
    );
    h.insert("Referrer-Policy", "no-referrer".parse().unwrap());
    h.insert(
        "X-Robots-Tag",
        "noindex, nofollow, noarchive".parse().unwrap(),
    );
    h.insert("X-Content-Type-Options", "nosniff".parse().unwrap());
    h.insert("Content-Security-Policy","default-src 'none'; style-src 'unsafe-inline'; sandbox; frame-ancestors 'none'; base-uri 'none'; form-action 'none'".parse().unwrap());
    r
}
