//! Physician chat on the Rust local product.
//!
//! `message.submit` records the physician turn, calls the chat role, and
//! publishes `workspace_changed` when that call finishes. Codex `live`,
//! `record`, and `replay` do not use an API key. A scripted model stands in
//! for the CLI in tests.

use std::convert::Infallible;
use std::sync::{Arc, Mutex};

use axum::Json;
use axum::body::Body;
use axum::extract::{Extension, Path, State};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use futures_util::stream;
use local_product::codex::CodexTransport;
use local_product::{ChatSession, ModelBinding, Submit};
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::sync::broadcast;

use crate::auth::{AuthContext, allows_workspace, now};
use crate::rejection_message;

/// The chat role's caller. `Fixed` never leaves the process.
pub(crate) enum ChatModel {
    /// Returns this text and does not call OpenAI.
    #[cfg(test)]
    Fixed(String),
    /// Calls OpenAI Responses with this key and the chat binding.
    OpenAi {
        /// The API key. It is not written to logs.
        key: String,
        /// The chat role.
        binding: ModelBinding,
    },
    /// Codex CLI live, record, or replay. No API key.
    Codex {
        /// The chat role.
        binding: ModelBinding,
        /// The Codex transport. Record and replay mutate the cassette.
        transport: Arc<Mutex<CodexTransport>>,
    },
}

/// In-memory chats and the event other subscribers are waiting on.
pub(crate) struct Live {
    model: ChatModel,
    session: Mutex<ChatSession>,
    events: broadcast::Sender<String>,
}

impl Live {
    /// Starts an empty chat session for the model's role.
    pub(crate) fn new(model: ChatModel) -> Arc<Self> {
        let binding = match &model {
            #[cfg(test)]
            ChatModel::Fixed(_) => ModelBinding {
                model: "gpt-6-sol".to_owned(),
                effort: "high".to_owned(),
                adapter: "openai-responses".to_owned(),
            },
            ChatModel::OpenAi { binding, .. } | ChatModel::Codex { binding, .. } => binding.clone(),
        };
        let (events, _) = broadcast::channel(64);
        Arc::new(Self {
            model,
            session: Mutex::new(ChatSession::new(binding)),
            events,
        })
    }
}

#[derive(Deserialize)]
struct CommandBody {
    #[serde(rename = "type")]
    kind: String,
    idempotency_key: Option<String>,
    payload: Option<SubmitPayload>,
}

#[derive(Deserialize)]
struct SubmitPayload {
    patient_workspace_id: String,
    chat_id: String,
    text: String,
    physician_time_zone: String,
}

/// Accepts `message.submit` and rejects every other command.
pub(crate) async fn submit(
    State(live): State<Arc<Live>>,
    auth_ctx: Option<Extension<AuthContext>>,
    body: axum::body::Bytes,
) -> Response {
    let Ok(body) = serde_json::from_slice::<CommandBody>(&body) else {
        return rejected_submit();
    };
    if body.kind != "message.submit" {
        return Json(json!({
            "outcome": "rejected",
            "message": rejection_message(&body.kind),
        }))
        .into_response();
    }
    let Some(payload) = body.payload else {
        return rejected_submit();
    };
    if payload.patient_workspace_id.trim().is_empty()
        || payload.chat_id.trim().is_empty()
        || payload.physician_time_zone.trim().is_empty()
    {
        return rejected_submit();
    }
    let Some(idempotency_key) = body.idempotency_key.filter(|key| !key.trim().is_empty()) else {
        return rejected_submit();
    };
    let request = {
        let mut session = live.session.lock().expect("chat session");
        match session.submit(
            &idempotency_key,
            &payload.patient_workspace_id,
            &payload.chat_id,
            &payload.text,
        ) {
            Submit::Accepted { request } => Some(request),
            Submit::Duplicate => None,
            Submit::Rejected => return rejected_submit(),
        }
    };
    if let Some(request) = request {
        let live = Arc::clone(&live);
        let workspace = payload.patient_workspace_id.clone();
        let chat = payload.chat_id.clone();
        let auth_ctx = auth_ctx.map(|Extension(ctx)| ctx);
        tokio::spawn(async move {
            match complete(&live.model, &request).await {
                Ok(text) => {
                    if let Some(ref ctx) = auth_ctx {
                        let session_ok = ctx.auth.store.session(&ctx.token, now()).await.is_ok();
                        let allowed = allows_workspace(&ctx.auth, &ctx.identity, &workspace).await;
                        if !session_ok || !allowed {
                            return;
                        }
                    }
                    live.session
                        .lock()
                        .expect("chat session")
                        .record_reply(&workspace, &chat, &text);
                    let _ = live.events.send(workspace);
                }
                Err(error) => eprintln!("physician chat: {error}"),
            }
        });
    }
    Json(json!({
        "outcome": "accepted",
        "patient_workspace_id": payload.patient_workspace_id,
        "chat_id": payload.chat_id,
    }))
    .into_response()
}

/// The turns recorded for this chat.
pub(crate) async fn snapshot(
    State(live): State<Arc<Live>>,
    Path((workspace, chat)): Path<(String, String)>,
) -> Json<Value> {
    let conversation = live
        .session
        .lock()
        .expect("chat session")
        .conversation(&workspace, &chat);
    Json(json!({
        "snapshot": {
            "conversation": conversation
                .into_iter()
                .map(|text| json!({ "text": text }))
                .collect::<Vec<_>>()
        }
    }))
}

/// `workspace_changed` frames. The body stays open.
pub(crate) async fn events(State(live): State<Arc<Live>>) -> Response {
    let rx = live.events.subscribe();
    let stream = stream::unfold(rx, |mut rx| async move {
        let workspace = rx.recv().await.ok()?;
        let data = json!({ "patient_workspace_id": workspace }).to_string();
        let frame = format!("event: workspace_changed\ndata: {data}\n\n");
        Some((Ok::<String, Infallible>(frame), rx))
    });
    (
        StatusCode::OK,
        [(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("text/event-stream"),
        )],
        Body::from_stream(stream),
    )
        .into_response()
}

fn rejected_submit() -> Response {
    Json(json!({
        "outcome": "rejected",
        "message": rejection_message("message.submit"),
    }))
    .into_response()
}

async fn complete(model: &ChatModel, request: &Value) -> Result<String, String> {
    match model {
        #[cfg(test)]
        ChatModel::Fixed(text) => Ok(text.clone()),
        ChatModel::OpenAi { key, .. } => {
            let response = reqwest::Client::new()
                .post("https://api.openai.com/v1/responses")
                .bearer_auth(key)
                .json(request)
                .send()
                .await
                .map_err(|_| "OpenAI responses request failed".to_owned())?;
            if !response.status().is_success() {
                return Err(format!("OpenAI responses returned {}", response.status()));
            }
            let body: Value = response
                .json()
                .await
                .map_err(|_| "OpenAI responses returned unreadable JSON".to_owned())?;
            local_product::authored_text(&body)
                .ok_or_else(|| "OpenAI responses returned no authored text".to_owned())
        }
        ChatModel::Codex { transport, .. } => {
            let transport = Arc::clone(transport);
            let request = request.clone();
            match tokio::task::spawn_blocking(move || {
                transport.lock().expect("codex").complete(&request)
            })
            .await
            {
                Ok(result) => result,
                Err(_) => Err("Codex exec task stopped".to_owned()),
            }
        }
    }
}
