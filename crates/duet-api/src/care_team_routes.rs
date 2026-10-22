//! Authenticated account-private directory, care-team and review-template routes.
use crate::{
    ApiError,
    care_team::{CareTeamStore, MemberPatch, ReleaseInput},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, patch, post, put},
};
use serde::Deserialize;
use uuid::Uuid;
/// Mount with an injectable public directory and existing durable account authority.
pub fn router(state: CareTeamStore) -> Router {
    Router::new()
        .route("/api/clinicians/search", post(search))
        .route("/api/care-team", get(list).post(add))
        .route("/api/care-team/{id}", patch(edit).delete(remove))
        .route("/api/care-team/{id}/refresh", post(refresh))
        .route("/api/release-authorizations", post(create_release))
        .route("/api/release-authorizations/{id}", put(update_release))
        .route("/api/release-authorizations/{id}/pdf", get(pdf))
        .route("/api/release-authorizations/{id}/preview", get(preview))
        .layer(axum::middleware::map_response(private_response))
        .with_state(state)
}
async fn private_response(mut response: Response) -> Response {
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, "private, no-store".parse().unwrap());
    response
        .headers_mut()
        .insert(header::VARY, "Authorization".parse().unwrap());
    response
        .headers_mut()
        .insert(header::X_CONTENT_TYPE_OPTIONS, "nosniff".parse().unwrap());
    response
}
async fn search(
    State(s): State<CareTeamStore>,
    h: HeaderMap,
    Json(q): Json<npi_registry::SearchQuery>,
) -> Result<Json<Vec<npi_registry::Clinician>>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    s.check(&c).await?;
    let result = s
        .directory
        .search(&q)
        .await
        .map_err(crate::care_team::registry_error)?;
    s.check(&c).await?;
    Ok(Json(result))
}
async fn list(
    State(s): State<CareTeamStore>,
    h: HeaderMap,
) -> Result<Json<Vec<crate::care_team::CareTeamMember>>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(s.list(&c).await?))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Add {
    npi: String,
}
async fn add(
    State(s): State<CareTeamStore>,
    h: HeaderMap,
    Json(b): Json<Add>,
) -> Result<Json<crate::care_team::CareTeamMember>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(s.add(&c, &b.npi).await?))
}
async fn edit(
    State(s): State<CareTeamStore>,
    h: HeaderMap,
    Path(id): Path<Uuid>,
    Json(b): Json<MemberPatch>,
) -> Result<Json<crate::care_team::CareTeamMember>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(s.patch(&c, id, b).await?))
}
async fn remove(
    State(s): State<CareTeamStore>,
    h: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    s.delete(&c, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
async fn refresh(
    State(s): State<CareTeamStore>,
    h: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<crate::care_team::CareTeamMember>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(s.refresh(&c, id).await?))
}
async fn create_release(
    State(s): State<CareTeamStore>,
    h: HeaderMap,
    Json(b): Json<ReleaseInput>,
) -> Result<Json<crate::care_team::ReleaseView>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(s.save_release(&c, None, b).await?))
}
async fn update_release(
    State(s): State<CareTeamStore>,
    h: HeaderMap,
    Path(id): Path<Uuid>,
    Json(b): Json<ReleaseInput>,
) -> Result<Json<crate::care_team::ReleaseView>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(s.save_release(&c, Some(id), b).await?))
}
async fn pdf(
    State(s): State<CareTeamStore>,
    h: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Response, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    let bytes = s.render_release(&c, id).await?.pdf().map_err(|_| {
        ApiError::Unprocessable("Template cannot faithfully render; review unsupported text")
    })?;
    Ok((
        [
            (header::CONTENT_TYPE, "application/pdf"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=release-authorization.pdf",
            ),
        ],
        bytes,
    )
        .into_response())
}
async fn preview(
    State(s): State<CareTeamStore>,
    h: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Response, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok((
        [(header::CONTENT_TYPE, "image/svg+xml")],
        s.render_release(&c, id).await?.preview_svg(),
    )
        .into_response())
}
