//! The composer's saved draft belongs to one physician, patient workspace and chat.
//! Postgres owns its text and revision; stale saves return the current draft.
use crate::{
    auth::{AuthContext, allows_workspace},
    live::Live,
};
use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use local_product::runtime::{
    Scope,
    chat_draft::{ChatDraft, ChatDraftSaved},
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

fn scope(ctx: &AuthContext, workspace: &str, chat: &str) -> Scope {
    Scope {
        practice: ctx.identity.practice_id.clone(),
        physician: ctx.identity.physician_id.clone(),
        workspace: workspace.into(),
        chat: chat.into(),
    }
}

fn body(draft: &ChatDraft) -> serde_json::Value {
    json!({"revision": draft.revision, "text": draft.text})
}

fn conflict(current: &ChatDraft) -> Response {
    (
        StatusCode::CONFLICT,
        Json(json!({"outcome": "conflict", "current": body(current)})),
    )
        .into_response()
}

#[derive(Deserialize)]
pub(crate) struct Save {
    expected_revision: u64,
    text: String,
}

pub(crate) async fn read(
    State(live): State<Arc<Live>>,
    Path((workspace, chat)): Path<(String, String)>,
    ctx: Option<Extension<AuthContext>>,
) -> Response {
    let ctx = ctx.map(|Extension(ctx)| ctx);
    let durable = &live.chat;
    // Chat routes always carry a session; without one there is no physician.
    let Some(ctx) = ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    if !allows_workspace(&ctx.auth, &ctx.identity, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    match durable
        .store
        .chat_draft(&scope(&ctx, &workspace, &chat))
        .await
    {
        Ok(draft) => Json(body(&draft)).into_response(),
        Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}

pub(crate) async fn save(
    State(live): State<Arc<Live>>,
    Path((workspace, chat)): Path<(String, String)>,
    ctx: Option<Extension<AuthContext>>,
    Json(save): Json<Save>,
) -> Response {
    let ctx = ctx.map(|Extension(ctx)| ctx);
    let durable = &live.chat;
    let Some(ctx) = ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    if !allows_workspace(&ctx.auth, &ctx.identity, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    let mut authority = crate::durable::RuntimeAuthority;
    let hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
    match durable
        .store
        .save_chat_draft(
            &scope(&ctx, &workspace, &chat),
            save.expected_revision,
            &save.text,
            &hash,
            &mut authority,
        )
        .await
    {
        Ok(ChatDraftSaved::Saved(draft)) => Json(body(&draft)).into_response(),
        Ok(ChatDraftSaved::Conflict(current)) => conflict(&current),
        // The auth owner refused the session or the panel in the save's
        // own transaction: nothing was written.
        Err(sea_orm::DbErr::Custom(_)) => StatusCode::CONFLICT.into_response(),
        Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}
