//! The composer's saved chat draft: `GET` and `PUT
//! /physician-state/drafts/{workspace}/{chat}`.
//!
//! A draft belongs to one signed-in physician, one patient workspace and one
//! chat (the page saves under the chat id `draft` before a chat is open).
//! Hosted sites keep it in Postgres through [`local_product::runtime::Store`];
//! inline sites keep it in the [`PhysicianSession`](local_product::execution::PhysicianSession).
//! A save names the revision it replaces; a stale one answers 409 with the
//! current draft and writes nothing. With sign-in off (a local inline site)
//! there is one anonymous draft per workspace and chat, as for anonymous chats;
//! hosted sites always require a session.
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

/// The inline session target. Its 4-tuple JSON key never equals a chat's own
/// target (a bare chat id with an owner) or a reply's 3-tuple target. With
/// sign-in off there is no physician: the owner is `null`, the same anonymous
/// local scope inline chats use.
fn target(ctx: Option<&AuthContext>, chat: &str) -> String {
    serde_json::to_string(&(
        "chat",
        ctx.map(|ctx| &ctx.identity.practice_id),
        ctx.map(|ctx| &ctx.identity.physician_id),
        chat,
    ))
    .unwrap()
}

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

fn inline_draft(live: &Live, ctx: Option<&AuthContext>, workspace: &str, chat: &str) -> ChatDraft {
    live.session
        .lock()
        .expect("physician session")
        .draft(workspace, &target(ctx, chat))
        .map(|(revision, text, _)| ChatDraft { revision, text })
        .unwrap_or_default()
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
    if let Some(durable) = &live.durable {
        // Hosted routes always carry a session; without one there is no physician.
        let Some(ctx) = ctx else {
            return StatusCode::UNAUTHORIZED.into_response();
        };
        if !allows_workspace(&ctx.auth, &ctx.identity, &workspace).await {
            return StatusCode::NOT_FOUND.into_response();
        }
        return match durable
            .store
            .chat_draft(&scope(&ctx, &workspace, &chat))
            .await
        {
            Ok(draft) => Json(body(&draft)).into_response(),
            Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
        };
    }
    if let Some(ctx) = &ctx
        && !allows_workspace(&ctx.auth, &ctx.identity, &workspace).await
    {
        return StatusCode::NOT_FOUND.into_response();
    }
    Json(body(&inline_draft(&live, ctx.as_ref(), &workspace, &chat))).into_response()
}

pub(crate) async fn save(
    State(live): State<Arc<Live>>,
    Path((workspace, chat)): Path<(String, String)>,
    ctx: Option<Extension<AuthContext>>,
    Json(save): Json<Save>,
) -> Response {
    let ctx = ctx.map(|Extension(ctx)| ctx);
    if let Some(durable) = &live.durable {
        let Some(ctx) = ctx else {
            return StatusCode::UNAUTHORIZED.into_response();
        };
        if !allows_workspace(&ctx.auth, &ctx.identity, &workspace).await {
            return StatusCode::NOT_FOUND.into_response();
        }
        let mut authority = crate::durable::RuntimeAuthority {
            store: durable.store.clone(),
        };
        let hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
        return match durable
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
        };
    }
    let target = target(ctx.as_ref(), &chat);
    let mut saved = None;
    let mut stale = false;
    let mut write = || {
        let mut session = live.session.lock().expect("physician session");
        match session.save_draft(&workspace, &target, save.expected_revision, &save.text) {
            Ok(revision) => {
                saved = Some(revision);
                Ok(())
            }
            Err(error) => {
                stale = true;
                Err(error)
            }
        }
    };
    let result = match &ctx {
        Some(ctx) => {
            if !allows_workspace(&ctx.auth, &ctx.identity, &workspace).await {
                return StatusCode::NOT_FOUND.into_response();
            }
            let _guard = ctx.auth.publication.lock().await;
            crate::publication::publish(ctx, &workspace, &mut write).await
        }
        // Sign-in is off: one local anonymous draft, as for anonymous chats.
        None => write(),
    };
    match (result, saved) {
        (Ok(()), Some(revision)) => Json(body(&ChatDraft {
            revision,
            text: save.text,
        }))
        .into_response(),
        _ if stale => conflict(&inline_draft(&live, ctx.as_ref(), &workspace, &chat)),
        _ => StatusCode::CONFLICT.into_response(),
    }
}
