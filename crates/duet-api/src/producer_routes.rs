//! Authenticated producer entrypoints. Inputs name captures and calendar slots;
//! no public API accepts authored Today observations or trusted follow-up imports.
use crate::{
    ApiError,
    conversation_routes::ConversationState,
    durable::DurableState,
    health_context::{HealthContextProducer, Upload},
    today::{SlotRequest, TodayStore},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::HeaderMap,
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

/// Distinct producer credentials are private to their owning services.
#[derive(Clone)]
pub struct ProducerState {
    /// Shared verified identity and independent restriction journal.
    pub durable: DurableState,
    /// Canonical arithmetic write owner, never a doctor snapshot importer.
    pub context: HealthContextProducer,
    /// Accepted-work facade using the same command executor and effect ledger.
    pub conversation: ConversationState,
    /// Today assignment/publication owner.
    pub today: TodayStore,
}
/// Routes for durable producers and person-controlled follow-through.
pub fn router(state: ProducerState) -> Router {
    Router::new()
        .route(
            "/api/health-context",
            get(context_read).post(context_upload),
        )
        .route("/api/health-context/refresh", post(context_refresh))
        .route("/api/insight-cycles", post(today_admit))
        .route("/api/insight-cycles/status", post(today_status))
        .route(
            "/api/insight-cycles/{cycle}/insights/{insight}/correction",
            post(correct),
        )
        .route("/api/follow-ups", get(follow_list))
        .route("/api/follow-ups/{id}", get(follow_read))
        .route("/api/follow-ups/{id}/resume", post(follow_resume))
        .route("/api/follow-ups/{id}/pause", post(follow_pause))
        .route("/api/follow-ups/{id}/cancel", post(follow_cancel))
        .with_state(state)
}
async fn context_read(
    State(s): State<ProducerState>,
    h: HeaderMap,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(json!(s.context.read(&c).await?)))
}
async fn context_upload(
    State(s): State<ProducerState>,
    h: HeaderMap,
    Json(body): Json<Upload>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    let stored = s.context.upload(&c, &body).await?;
    let mut response = json!(stored);
    response["accepted"] = json!(true);
    response["snapshot_id"] = response["snapshot"]["id"].clone();
    crate::conversation_context::nullable_portal(&mut response);
    Ok(Json(response))
}
async fn context_refresh(
    State(s): State<ProducerState>,
    h: HeaderMap,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(json!(s.context.refresh(&c).await?)))
}
async fn today_admit(
    State(s): State<ProducerState>,
    h: HeaderMap,
    Json(body): Json<SlotRequest>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    if s.conversation.provider.is_none() {
        return Err(ApiError::Unavailable("assessment provider not enabled"));
    }
    let value = s
        .today
        .admit(
            &c,
            &body,
            &s.conversation.model,
            crate::conversation::INSTRUCTIONS,
        )
        .await?;
    if let Some(id) = value["run_id"].as_str() {
        crate::conversation_routes::launch(
            &s.conversation,
            &c,
            id.parse()
                .map_err(|_| ApiError::Unavailable("invalid accepted assessment identity"))?,
        )
        .await?;
    }
    Ok(Json(value))
}
async fn today_status(
    State(s): State<ProducerState>,
    h: HeaderMap,
    Json(body): Json<SlotRequest>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(s.today.status(&c, &body).await?))
}
async fn correct(
    State(s): State<ProducerState>,
    h: HeaderMap,
    Path((cycle, insight)): Path<(Uuid, Uuid)>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    s.today.correct(&c, cycle, insight).await?;
    Ok(Json(json!({"corrected":true})))
}
async fn follow_list(
    State(s): State<ProducerState>,
    h: HeaderMap,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    let mut tx = s.durable.identity.database.begin(c.account_id).await?;
    crate::identity::check_current(&mut tx, &c, false).await?;
    let ids: Vec<Uuid> =
        sqlx::query_scalar("select id from follow_ups where user_id=$1 order by created_at,id")
            .bind(c.account_id)
            .fetch_all(&mut *tx)
            .await?;
    let mut items = Vec::new();
    for id in ids {
        items.push(crate::follow_up::read(&mut tx, c.account_id, id).await?);
    }
    Ok(Json(json!({"follow_ups":items})))
}
async fn follow_read(
    State(s): State<ProducerState>,
    h: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    let mut tx = s.durable.identity.database.begin(c.account_id).await?;
    crate::identity::check_current(&mut tx, &c, false).await?;
    Ok(Json(
        crate::follow_up::read(&mut tx, c.account_id, id).await?,
    ))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RestrictFollowUp {
    #[serde(default)]
    event_id: Option<Uuid>,
    #[serde(alias = "revision")]
    expected_revision: i64,
}
async fn follow_pause(
    State(s): State<ProducerState>,
    h: HeaderMap,
    Path(id): Path<Uuid>,
    Json(body): Json<RestrictFollowUp>,
) -> Result<Json<Value>, ApiError> {
    follow_restrict(s, h, id, body, crate::privacy::Restriction::FollowUpPause).await
}
async fn follow_cancel(
    State(s): State<ProducerState>,
    h: HeaderMap,
    Path(id): Path<Uuid>,
    Json(body): Json<RestrictFollowUp>,
) -> Result<Json<Value>, ApiError> {
    follow_restrict(s, h, id, body, crate::privacy::Restriction::FollowUpCancel).await
}
async fn follow_restrict(
    s: ProducerState,
    h: HeaderMap,
    id: Uuid,
    body: RestrictFollowUp,
    operation: crate::privacy::Restriction,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(
        format!(
            "{}:{}:{id}:{}:{operation:?}",
            c.account_id, c.incarnation, body.expected_revision
        )
        .as_bytes(),
    );
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    let event = body.event_id.unwrap_or_else(|| Uuid::from_bytes(bytes));
    s.durable
        .privacy
        .restrict(&c, event, operation, id, body.expected_revision)
        .await?;
    let mut tx = s.durable.identity.database.begin(c.account_id).await?;
    Ok(Json(
        crate::follow_up::read(&mut tx, c.account_id, id).await?,
    ))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Resume {
    #[serde(alias = "revision")]
    expected_revision: i64,
}
async fn follow_resume(
    State(s): State<ProducerState>,
    h: HeaderMap,
    Path(id): Path<Uuid>,
    Json(body): Json<Resume>,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    let mut tx = s.durable.privacy.database.begin(c.account_id).await?;
    let result = crate::follow_up::person_control(
        &mut tx,
        &c,
        id,
        body.expected_revision,
        crate::follow_up::Control::Resume,
    )
    .await?;
    tx.commit().await?;
    Ok(Json(result))
}
