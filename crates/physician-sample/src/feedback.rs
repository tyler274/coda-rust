//! Feedback on Coda's responses in a hosted chat:
//! `GET /physician-state/feedback/{workspace}/{chat}` and `PUT
//! /physician-state/feedback/{workspace}/{chat}/{response}/rating` and
//! `.../details`.
//!
//! Feedback belongs to one signed-in physician, one patient workspace and one
//! chat, and names a response by its place among the chat's messages, as the
//! snapshot's `response` does. Only its author reads it over HTTP; Coda staff
//! read it with `physician-migrate feedback`. Hosted sites only: an inline
//! site mounts none of these routes.
use crate::{
    auth::{AuthContext, allows_workspace},
    durable::{DurableChat, RuntimeAuthority},
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
    feedback::{Details, DetailsSaved, Feedback, Rating, RatingSaved, Reason},
};
use physician_wire::{
    ChatFeedback, FeedbackDetails, FeedbackRating, FeedbackReason, RateResponse, ResponseFeedback,
};
use std::sync::Arc;

fn scope(ctx: &AuthContext, workspace: String, chat: String) -> Scope {
    Scope {
        practice: ctx.identity.practice_id.clone(),
        physician: ctx.identity.physician_id.clone(),
        workspace,
        chat,
    }
}

fn wire(response: u32, feedback: Feedback) -> ResponseFeedback {
    ResponseFeedback {
        response: response as usize,
        rating: match feedback.rating {
            Rating::Up => FeedbackRating::Up,
            Rating::Down => FeedbackRating::Down,
        },
        details: feedback.details.map(|details| FeedbackDetails {
            reasons: details
                .reasons()
                .iter()
                .map(|reason| match reason {
                    Reason::IncorrectInformation => FeedbackReason::IncorrectInformation,
                    Reason::MissingContext => FeedbackReason::MissingContext,
                    Reason::HardToFollow => FeedbackReason::HardToFollow,
                    Reason::NotUseful => FeedbackReason::NotUseful,
                })
                .collect(),
            text: details.text().to_owned(),
        }),
    }
}

fn details(body: FeedbackDetails) -> Option<Details> {
    let reasons = body
        .reasons
        .into_iter()
        .map(|reason| match reason {
            FeedbackReason::IncorrectInformation => Reason::IncorrectInformation,
            FeedbackReason::MissingContext => Reason::MissingContext,
            FeedbackReason::HardToFollow => Reason::HardToFollow,
            FeedbackReason::NotUseful => Reason::NotUseful,
        })
        .collect();
    Details::new(reasons, body.text)
}

/// The signed-in physician with the patient on their panel, or the answer
/// that ends the request: 404 off a hosted site, 401 without a session, 404
/// off the panel.
async fn authorized<'a>(
    live: &'a Live,
    ctx: Option<Extension<AuthContext>>,
    workspace: &str,
) -> Result<(&'a DurableChat, AuthContext), StatusCode> {
    let Some(durable) = &live.durable else {
        return Err(StatusCode::NOT_FOUND);
    };
    let Some(Extension(ctx)) = ctx else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    if !allows_workspace(&ctx.auth, &ctx.identity, workspace).await {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok((durable, ctx))
}

pub(crate) async fn read(
    State(live): State<Arc<Live>>,
    Path((workspace, chat)): Path<(String, String)>,
    ctx: Option<Extension<AuthContext>>,
) -> Response {
    let (durable, ctx) = match authorized(&live, ctx, &workspace).await {
        Ok(found) => found,
        Err(status) => return status.into_response(),
    };
    match durable
        .store
        .chat_feedback(&scope(&ctx, workspace, chat))
        .await
    {
        Ok(saved) => Json(ChatFeedback {
            feedback: saved
                .into_iter()
                .map(|(response, feedback)| wire(response, feedback))
                .collect(),
        })
        .into_response(),
        Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}

pub(crate) async fn rate(
    State(live): State<Arc<Live>>,
    Path((workspace, chat, response)): Path<(String, String, u32)>,
    ctx: Option<Extension<AuthContext>>,
    Json(body): Json<RateResponse>,
) -> Response {
    let (durable, ctx) = match authorized(&live, ctx, &workspace).await {
        Ok(found) => found,
        Err(status) => return status.into_response(),
    };
    let rating = match body.rating {
        FeedbackRating::Up => Rating::Up,
        FeedbackRating::Down => Rating::Down,
    };
    let mut authority = RuntimeAuthority {
        store: durable.store.clone(),
    };
    let hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
    match durable
        .store
        .rate_response(
            &scope(&ctx, workspace, chat),
            response,
            rating,
            &hash,
            &mut authority,
        )
        .await
    {
        Ok(RatingSaved::Saved(feedback)) => Json(wire(response, feedback)).into_response(),
        Ok(RatingSaved::NotAResponse) => StatusCode::NOT_FOUND.into_response(),
        Ok(RatingSaved::Refused) => StatusCode::CONFLICT.into_response(),
        Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}

pub(crate) async fn save_details(
    State(live): State<Arc<Live>>,
    Path((workspace, chat, response)): Path<(String, String, u32)>,
    ctx: Option<Extension<AuthContext>>,
    Json(body): Json<FeedbackDetails>,
) -> Response {
    let (durable, ctx) = match authorized(&live, ctx, &workspace).await {
        Ok(found) => found,
        Err(status) => return status.into_response(),
    };
    let Some(details) = details(body) else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };
    let mut authority = RuntimeAuthority {
        store: durable.store.clone(),
    };
    let hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
    match durable
        .store
        .save_feedback_details(
            &scope(&ctx, workspace, chat),
            response,
            &details,
            &hash,
            &mut authority,
        )
        .await
    {
        Ok(DetailsSaved::Saved(feedback)) => Json(wire(response, feedback)).into_response(),
        Ok(DetailsSaved::NotRated) => StatusCode::NOT_FOUND.into_response(),
        Ok(DetailsSaved::Refused) => StatusCode::CONFLICT.into_response(),
        Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}
