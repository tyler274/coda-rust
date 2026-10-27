//! HTTPS access to the same SMTP transport used by the physician server.
//!
//! Cloud Run IAM owns caller authentication. Products own recipients, message
//! text, codes, and their send records. This adapter has no queue or retry store.

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde::Deserialize;

use crate::{Error, SmtpRelay};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Message {
    to: String,
    subject: String,
    text: String,
    html: Option<String>,
}

/// The relay API. Deploy behind Cloud Run's IAM invoker check, never anonymously.
/// `204` confirms SMTP acceptance, `422` a refusal, and `503` an unknown outcome.
/// Acceptance is not proof of delivery to the recipient's inbox.
pub fn router(relay: SmtpRelay) -> Router {
    Router::new()
        .route("/readyz", get(|| async { StatusCode::NO_CONTENT }))
        .route("/send", post(send))
        .with_state(Arc::new(relay))
}

async fn send(State(relay): State<Arc<SmtpRelay>>, Json(message): Json<Message>) -> StatusCode {
    let result = match message.html {
        Some(html) => {
            relay
                .send_html(&message.to, &message.subject, &message.text, &html)
                .await
        }
        None => {
            relay
                .send(&message.to, &message.subject, &message.text)
                .await
        }
    };
    match result {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(Error::Headers | Error::RelayRefused(_)) => StatusCode::UNPROCESSABLE_ENTITY,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}
