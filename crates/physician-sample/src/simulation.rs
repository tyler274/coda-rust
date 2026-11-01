use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use serde_json::{Value, json};

use local_product::{
    DEFAULT_SIMULATION_SCENARIO, SimulationSession, local_simulation_scenarios,
    roger_panel_patients,
};

#[derive(Clone)]
pub(crate) struct SimulationState {
    inner: Arc<Mutex<SimulationInner>>,
}

/// One chat turn: who wrote it, when someone did, and the words.
type Turn = (Option<&'static str>, String);

struct SimulationInner {
    session: SimulationSession,
    /// Each chat's turns, by workspace and chat.
    chats: HashMap<(String, String), Vec<Turn>>,
    as_of: i64,
}

impl SimulationState {
    pub(crate) fn new(scenario: &str) -> Self {
        Self {
            inner: Arc::new(Mutex::new(SimulationInner {
                session: SimulationSession::new(scenario),
                chats: HashMap::new(),
                as_of: 100,
            })),
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct SimulationTodayQuery {
    patient_workspace_id: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct RunScenarioRequest {
    scenario: Option<String>,
}

pub(crate) async fn today(
    State(state): State<SimulationState>,
    Query(query): Query<SimulationTodayQuery>,
) -> Response {
    let guard = state.inner.lock().expect("simulation lock");
    let status = status_json(&guard.session);

    let rows: Vec<Value> = guard
        .session
        .overlays
        .values()
        .filter(|ov| {
            query
                .patient_workspace_id
                .as_deref()
                .is_none_or(|target| target == ov.patient_workspace_id)
        })
        .flat_map(|ov| {
            ov.active_replies.iter().map(|reply| {
                json!({
                    "patient_workspace_id": reply.patient_workspace_id,
                    "patient_display_name": reply.patient_display_name,
                    "item_id": reply.reply_work_id,
                    "version_id": format!("v-{}", guard.as_of),
                    "waiting_since": reply.created_at,
                    "urgency": "today",
                    "kind": "reply",
                    "reason": format!("{} — {}", reply.headline, reply.draft_text),
                })
            })
        })
        .collect();

    if let Some(id) = query.patient_workspace_id
        && !super::WORKSPACE_IDS.contains(&id.as_str())
    {
        return StatusCode::NOT_FOUND.into_response();
    }

    Json(json!({
        "date": "2026-08-27",
        "as_of": guard.as_of,
        "connections": [],
        "rows": rows,
        "seen": [],
        "simulation": status,
    }))
    .into_response()
}

pub(crate) async fn workspace(
    State(state): State<SimulationState>,
    Path(id): Path<String>,
) -> Response {
    let Some(raw) = super::workspace_body(&id) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let mut doc: Value = serde_json::from_str(raw).expect("workspace fixture JSON");
    let guard = state.inner.lock().expect("simulation lock");

    if let Some(overlay) = guard.session.overlays.get(&id) {
        if let Some(extra) = &overlay.synopsis_append
            && let Some(synopsis) = doc
                .get_mut("patient_synopsis")
                .and_then(Value::as_object_mut)
        {
            let existing = synopsis
                .get("authored_text")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_owned();
            synopsis.insert(
                "authored_text".to_owned(),
                Value::String(format!("{existing} {extra}")),
            );
            if let Some(segments) = synopsis.get_mut("segments").and_then(Value::as_array_mut) {
                segments.push(json!({
                    "kind": "text",
                    "text": format!(" {extra}"),
                }));
            }
        }

        let follow_ups: Vec<Value> = overlay
            .active_plan_items
            .iter()
            .map(|item| {
                json!({
                    "follow_up_id": item.item_id,
                    "description": format!("{} — {}", item.title, item.detail),
                })
            })
            .collect();

        if let Some(obj) = doc.as_object_mut() {
            obj.insert("active_follow_ups".to_owned(), Value::Array(follow_ups));
            obj.insert(
                "active_plan_items".to_owned(),
                serde_json::to_value(&overlay.active_plan_items).unwrap_or_default(),
            );
            obj.insert(
                "undecided_recommendations".to_owned(),
                serde_json::to_value(&overlay.undecided_recommendations).unwrap_or_default(),
            );
            obj.insert(
                "replies".to_owned(),
                serde_json::to_value(&overlay.active_replies).unwrap_or_default(),
            );
            obj.insert(
                "superseded_replies".to_owned(),
                serde_json::to_value(&overlay.superseded_replies).unwrap_or_default(),
            );
            obj.insert("simulation".to_owned(), status_json(&guard.session));
        }
    }

    Json(doc).into_response()
}

pub(crate) async fn snapshot(
    State(state): State<SimulationState>,
    Path((id, chat)): Path<(String, String)>,
) -> Response {
    if super::workspace_body(&id).is_none() {
        return StatusCode::NOT_FOUND.into_response();
    }
    let guard = state.inner.lock().expect("simulation lock");
    let turns = guard
        .chats
        .get(&(id, chat))
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|(kind, text)| match kind {
            Some(kind) => json!({ "kind": kind, "text": text }),
            None => json!({ "text": text }),
        })
        .collect::<Vec<_>>();
    Json(json!({
        "snapshot": { "conversation": turns }
    }))
    .into_response()
}

pub(crate) async fn simulations_index(State(state): State<SimulationState>) -> Response {
    let guard = state.inner.lock().expect("simulation lock");
    Json(json!({
        "simulation": status_json(&guard.session),
        "runs": guard.session.run_history,
    }))
    .into_response()
}

pub(crate) async fn run_scenario_endpoint(
    State(state): State<SimulationState>,
    body: axum::body::Bytes,
) -> Response {
    let scenario = serde_json::from_slice::<RunScenarioRequest>(&body)
        .ok()
        .and_then(|b| b.scenario)
        .unwrap_or_else(|| DEFAULT_SIMULATION_SCENARIO.to_owned());
    let mut guard = state.inner.lock().expect("simulation lock");
    match guard.session.run_scenario(&scenario) {
        Ok(record) => {
            guard.as_of += 1;
            Json(json!({
                "outcome": "accepted",
                "run": record,
                "simulation": status_json(&guard.session),
            }))
            .into_response()
        }
        Err(message) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "outcome": "rejected",
                "message": message,
            })),
        )
            .into_response(),
    }
}

pub(crate) async fn command(
    State(state): State<SimulationState>,
    body: axum::body::Bytes,
) -> Response {
    let Ok(body) = serde_json::from_slice::<Value>(&body) else {
        return super::reject_command(None).await;
    };
    let command_type = body.get("type").and_then(Value::as_str).unwrap_or("");
    let payload = body.get("payload").cloned().unwrap_or_else(|| json!({}));

    let mut guard = state.inner.lock().expect("simulation lock");
    match command_type {
        "simulation.run" => {
            let scenario = payload
                .get("scenario")
                .and_then(Value::as_str)
                .unwrap_or(DEFAULT_SIMULATION_SCENARIO);
            match guard.session.run_scenario(scenario) {
                Ok(record) => {
                    guard.as_of += 1;
                    Json(json!({
                        "outcome": "accepted",
                        "run": record,
                        "simulation": status_json(&guard.session),
                    }))
                    .into_response()
                }
                Err(message) => Json(json!({
                    "outcome": "rejected",
                    "message": message,
                }))
                .into_response(),
            }
        }
        "simulation.admit_corrected_labs" => {
            let ws_id = payload
                .get("patient_workspace_id")
                .and_then(Value::as_str)
                .unwrap_or(local_product::JAKE_WORKSPACE_ID);
            match guard.session.admit_corrected_labs(ws_id) {
                Ok(()) => {
                    guard.as_of += 1;
                    Json(json!({
                        "outcome": "accepted",
                        "simulation": status_json(&guard.session),
                    }))
                    .into_response()
                }
                Err(message) => Json(json!({
                    "outcome": "rejected",
                    "message": message,
                }))
                .into_response(),
            }
        }
        "plan.decide_recommendation" => {
            let ws_id = payload
                .get("patient_workspace_id")
                .and_then(Value::as_str)
                .unwrap_or("");
            let rec_id = payload
                .get("recommendation_id")
                .and_then(Value::as_str)
                .unwrap_or("");
            match guard.session.decide_recommendation(ws_id, rec_id) {
                Ok(plan_item) => {
                    guard.as_of += 1;
                    Json(json!({
                        "outcome": "accepted",
                        "plan_item": plan_item,
                        "simulation": status_json(&guard.session),
                    }))
                    .into_response()
                }
                Err(message) => Json(json!({
                    "outcome": "rejected",
                    "message": message,
                }))
                .into_response(),
            }
        }
        "reply.close" => {
            let ws_id = payload
                .get("patient_workspace_id")
                .and_then(Value::as_str)
                .unwrap_or("");
            let reply_id = payload
                .get("reply_work_id")
                .and_then(Value::as_str)
                .unwrap_or("");
            match guard.session.close_reply(ws_id, reply_id) {
                Ok(()) => {
                    guard.as_of += 1;
                    Json(json!({
                        "outcome": "accepted",
                        "simulation": status_json(&guard.session),
                    }))
                    .into_response()
                }
                Err(message) => Json(json!({
                    "outcome": "rejected",
                    "message": message,
                }))
                .into_response(),
            }
        }
        "message.submit" => {
            let ws_id = payload
                .get("patient_workspace_id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_owned();
            let chat_id = payload
                .get("chat_id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_owned();
            let text = payload
                .get("text")
                .and_then(Value::as_str)
                .unwrap_or("")
                .trim()
                .to_owned();
            if ws_id.is_empty() || chat_id.is_empty() || text.is_empty() {
                drop(guard);
                return super::reject_command(Some(Json(body))).await;
            }
            guard.session.record_physician_touch();
            guard.as_of += 1;
            let entry = guard.chats.entry((ws_id, chat_id)).or_default();
            entry.push((Some("physician"), text));
            // The simulation's own acknowledgement: neither the physician's words nor Coda's.
            entry.push((
                None,
                "Recorded in local simulation. Undecided medication recommendations remain held out of the active plan until approved."
                    .to_owned(),
            ));
            Json(json!({
                "outcome": "accepted",
                "simulation": status_json(&guard.session),
            }))
            .into_response()
        }
        _ => {
            drop(guard);
            super::reject_command(Some(Json(body))).await
        }
    }
}

pub(crate) fn status_json(session: &SimulationSession) -> Value {
    json!({
        "active_scenario_key": session.active_scenario_key,
        "active_scenario_label": session.active_scenario_label,
        "current_time": session.current_time,
        "investor_metric": session.investor_metric,
        "aggregate_metric": session.aggregate_metric,
        "scenarios": local_simulation_scenarios(),
        "panel_patients": roger_panel_patients(),
    })
}
