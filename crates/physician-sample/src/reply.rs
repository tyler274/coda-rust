//! Physician-specific reply revision HTTP boundary. A saved draft is required;
//! model output never creates reply work or sends a patient message.
use crate::{
    auth::{AuthContext, allows_workspace, now},
    live::Live,
};
use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use harness::execution::{Item, Request};
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;

pub(crate) fn target(ctx: &AuthContext, reply: &str) -> String {
    // JSON tuple avoids ambiguous delimiter-based physician identity joins.
    serde_json::to_string(&(&ctx.identity.practice_id, &ctx.identity.physician_id, reply)).unwrap()
}
#[derive(Deserialize)]
pub(crate) struct Revision {
    pub patient_workspace_id: String,
    pub reply_work_id: String,
    pub expected_draft_revision: u64,
    pub instruction: String,
    pub physician_time_zone: String,
}
#[derive(Deserialize)]
pub(crate) struct Save {
    revision: u64,
    text: String,
}

pub(crate) async fn read(
    State(live): State<Arc<Live>>,
    Path((workspace, reply)): Path<(String, String)>,
    ctx: Option<Extension<AuthContext>>,
) -> Response {
    let Some(Extension(ctx)) = ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    if !allows_workspace(&ctx.auth, &ctx.identity, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    match live
        .session
        .lock()
        .expect("physician session")
        .draft(&workspace, &target(&ctx, &reply))
    {
        Some((revision, text, restore_text)) => {
            Json(json!({"revision": revision,"text":text,"restore_text":restore_text}))
                .into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
pub(crate) async fn save(
    State(live): State<Arc<Live>>,
    Path((workspace, reply)): Path<(String, String)>,
    ctx: Option<Extension<AuthContext>>,
    Json(save): Json<Save>,
) -> Response {
    let Some(Extension(ctx)) = ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let _guard = ctx.auth.publication.lock().await;
    let mut revision = None;
    // This slice never invents initial reply work. The initial saved draft must
    // have been admitted by the physician reply owner during owner loading.
    let mut publish = || {
        let mut session = live.session.lock().expect("physician session");
        let target = target(&ctx, &reply);
        if session.draft(&workspace, &target).is_none() {
            return Err("reply draft missing".into());
        }
        revision = Some(session.save_draft(&workspace, &target, save.revision, &save.text)?);
        Ok(())
    };
    let result = crate::publication::publish(&ctx, &workspace, &mut publish).await;
    match result {
        Ok(()) => Json(json!({"revision":revision,"text":save.text})).into_response(),
        Err(_) => StatusCode::CONFLICT.into_response(),
    }
}

pub(crate) async fn admit(
    live: &Arc<Live>,
    ctx: &AuthContext,
    id: &str,
    payload: Value,
) -> Result<local_product::execution::Work, String> {
    let input: Revision =
        serde_json::from_value(payload).map_err(|_| "invalid revision command")?;
    if id.trim().is_empty()
        || input.instruction.trim().is_empty()
        || input.physician_time_zone.trim().is_empty()
    {
        return Err("invalid revision input".into());
    }
    if ctx.auth.store.session(&ctx.token, now()).await.is_err()
        || !allows_workspace(&ctx.auth, &ctx.identity, &input.patient_workspace_id).await
    {
        return Err("not authorized".into());
    }
    let target = target(ctx, &input.reply_work_id);
    let mut session = live.session.lock().expect("physician session");
    let (revision, text, _) = session
        .draft(&input.patient_workspace_id, &target)
        .ok_or("reply work has no saved draft")?;
    if revision != input.expected_draft_revision {
        return Err("stale draft".into());
    }
    let sources: Vec<_> = live
        .sources
        .iter()
        .filter(|s| s.workspace == input.patient_workspace_id)
        .cloned()
        .collect();
    let request = Request {
        model: live.binding.model.clone(), options: json!({"effort":live.binding.effort}),
        instructions: "Revise only this physician's saved patient-facing reply according to their instruction. Read exact sources when needed. Never write memory, delegate, send a message, or modify patient state. Finish with finish_reply_revision containing the complete text and review_basis. Preserve wording the instruction does not change.".into(),
        tools: vec![local_product::physician_sources::read_definition(), json!({"type":"function","name":"finish_reply_revision","description":"Return the complete revised reply and source-based review basis; never sends a message.","parameters":{"type":"object","properties":{"text":{"type":"string"},"review_basis":{"type":"string"}},"required":["text","review_basis"],"additionalProperties":false}})],
        items: vec![Item::Message { role:"user".into(), text:json!({"saved_draft":text,"instruction":input.instruction,"source_references":sources.iter().map(|s| &s.candidate_id).collect::<Vec<_>>(),"physician_time_zone":input.physician_time_zone}).to_string() }],
    };
    let work = session.revise(id, &input.patient_workspace_id, &target, revision, request)?;
    session.pin_sources(&work, sources)
}
