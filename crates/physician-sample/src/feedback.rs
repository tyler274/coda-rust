//! Feedback on Coda's responses in a hosted chat:
//! `PUT /physician-state/feedback/{workspace}/{chat}/{response}` saves the rating
//! and details together, and the chat snapshot carries each response's saved
//! feedback ([`wire`]).
//!
//! Feedback belongs to one signed-in physician, one patient workspace and one
//! chat, and names a response by its response id, as the snapshot's
//! `response_id` does. Only its author reads it over HTTP; Coda staff
//! read it with `physician-migrate feedback`. The fixture site mounts no
//! feedback route.
use crate::{
    auth::{AuthContext, allows_workspace},
    durable::RuntimeAuthority,
    live::Live,
};
use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use local_product::runtime::{
    ResponseId, Scope,
    feedback::{Details, Feedback, FeedbackSaved, Rating, Reason},
};
use physician_wire::{
    FeedbackDetails, FeedbackRating, FeedbackReason, ResponseFeedback, SaveFeedback,
};
use std::sync::Arc;

/// Saved feedback as the snapshot and a save's answer carry it.
pub(crate) fn wire(feedback: Feedback) -> ResponseFeedback {
    ResponseFeedback {
        revision: feedback.revision,
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

/// Saves the signed-in physician’s feedback on one response: 404 off the
/// panel, or when the chat holds no published reply by that id; 401 without a session; 409 when the auth owner refuses the session;
/// 422 for details over the length limit.
pub(crate) async fn save(
    State(live): State<Arc<Live>>,
    Path((workspace, chat, response)): Path<(String, String, String)>,
    ctx: Option<Extension<AuthContext>>,
    Json(body): Json<SaveFeedback>,
) -> Response {
    let durable = &live.chat;
    let Some(Extension(ctx)) = ctx else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    if !allows_workspace(&ctx.auth, &ctx.identity, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    let details = match body.details.map(details) {
        None => None,
        Some(Some(details)) => Some(details),
        Some(None) => return StatusCode::UNPROCESSABLE_ENTITY.into_response(),
    };
    let rating = match body.rating {
        FeedbackRating::Up => Rating::Up,
        FeedbackRating::Down => Rating::Down,
    };
    let scope = Scope {
        practice: ctx.identity.practice_id.clone(),
        physician: ctx.identity.physician_id.clone(),
        workspace,
        chat,
    };
    let mut authority = RuntimeAuthority;
    let hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
    match durable
        .store
        .save_feedback(
            &scope,
            &ResponseId::from(response),
            rating,
            details.as_ref(),
            &hash,
            &mut authority,
        )
        .await
    {
        Ok(FeedbackSaved::Saved(feedback)) => Json(wire(feedback)).into_response(),
        Ok(FeedbackSaved::NotAResponse) => StatusCode::NOT_FOUND.into_response(),
        Ok(FeedbackSaved::Refused) => StatusCode::CONFLICT.into_response(),
        Err(error) => {
            tracing::error!(%error, "physician feedback save failed");
            StatusCode::SERVICE_UNAVAILABLE.into_response()
        }
    }
}
