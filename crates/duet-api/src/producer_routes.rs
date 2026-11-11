//! Authenticated health-context producer entrypoints. Inputs name captures;
//! no public API accepts authored observations.
use crate::{
    ApiError,
    durable::DurableState,
    health_context::{HealthContextProducer, Upload},
};
use axum::{
    Json, Router,
    extract::State,
    http::HeaderMap,
    routing::{get, post},
};
use serde_json::{Value, json};

/// Distinct producer credentials are private to their owning services.
#[derive(Clone)]
pub struct ProducerState {
    /// Shared verified identity and independent restriction journal.
    pub durable: DurableState,
    /// Canonical arithmetic write owner, never a doctor snapshot importer.
    pub context: HealthContextProducer,
}
/// Routes for the durable health-context producer.
pub fn router(state: ProducerState) -> Router {
    Router::new()
        .route(
            "/api/health-context",
            get(context_read).post(context_upload),
        )
        .route("/api/health-context/refresh", post(context_refresh))
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
    nullable_portal(&mut response);
    Ok(Json(response))
}
async fn context_refresh(
    State(s): State<ProducerState>,
    h: HeaderMap,
) -> Result<Json<Value>, ApiError> {
    let c = s.durable.authenticate(&h).await?;
    Ok(Json(json!(s.context.refresh(&c).await?)))
}

fn nullable_portal(value: &mut Value) {
    match value {
        Value::Object(map) => {
            if let Some(Value::Array(evidence)) = map.get_mut("evidence") {
                for item in evidence {
                    if let Some(item) = item.as_object_mut() {
                        item.entry("source_portal").or_insert(Value::Null);
                    }
                }
            }
            for child in map.values_mut() {
                nullable_portal(child);
            }
        }
        Value::Array(items) => {
            for item in items {
                nullable_portal(item);
            }
        }
        _ => {}
    }
}
