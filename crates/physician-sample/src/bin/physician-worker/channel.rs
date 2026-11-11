//! `POST /wake/spruce-intake`: one bounded intake pass per practice that reads
//! its Spruce channel.
//!
//! A scheduled caller wakes it with the same signed identity the Pub/Sub push
//! uses. The body is ignored; the wake-up carries no practice or message. Each
//! practice's pass holds that practice's lease, so overlapping wake-ups never
//! read one channel twice. Logs name outcomes and counts, never content.

use local_product::runtime::channel::ChannelKey;
use local_product::runtime::channel::intake::{self, Api, Intake, PassOutcome};

use super::{Worker, authorize_push, configured};
use axum::extract::{Request, State};
use axum::http::StatusCode;
use std::sync::Arc;

/// The intake configuration, when this worker reads channels.
pub(super) fn from_env(
    get: &impl Fn(&str) -> Option<String>,
    hosted: bool,
) -> Result<Option<Intake>, String> {
    let Some(text) = configured(get, "PHYSICIAN_CHANNEL_KEY") else {
        return Ok(None);
    };
    let key = ChannelKey::from_hex(&text).ok_or("PHYSICIAN_CHANNEL_KEY must be 64 hex digits")?;
    let api = match configured(get, "PHYSICIAN_SPRUCE_API_BASE") {
        Some(_) if hosted => {
            return Err("PHYSICIAN_SPRUCE_API_BASE is refused in hosted execution".into());
        }
        Some(base) => Api::At(base),
        None => Api::Live,
    };
    // Unique per process start; the lease only needs to tell readers apart.
    let started = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    let holder = format!("physician-worker-{}-{started}", std::process::id());
    Ok(Some(Intake::new(key, api, holder)))
}

pub(super) async fn wake(State(worker): State<Arc<Worker>>, request: Request) -> StatusCode {
    let header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    if let Err(status) = authorize_push(worker.verifier.as_ref(), header.as_deref()).await {
        return status;
    }
    let Some(intake) = &worker.intake else {
        return StatusCode::NOT_FOUND;
    };
    let outcomes = match intake::wake(&worker.store, intake).await {
        Ok(outcomes) => outcomes,
        Err(error) => {
            tracing::error!(error = %error, "spruce intake wake-up could not list practices");
            return StatusCode::SERVICE_UNAVAILABLE;
        }
    };
    let mut failed = 0;
    for (practice, outcome) in &outcomes {
        match outcome {
            Ok(PassOutcome::Read { pages, kept, more }) => tracing::info!(
                practice = %practice, pages, kept, more, "spruce intake pass read"
            ),
            Ok(other) => {
                tracing::info!(practice = %practice, outcome = ?other, "spruce intake pass")
            }
            Err(error) => {
                failed += 1;
                tracing::warn!(practice = %practice, error = %error, "spruce intake pass failed");
            }
        }
    }
    tracing::info!(
        practices = outcomes.len(),
        failed,
        "spruce intake wake-up finished"
    );
    // A failed practice keeps its cursor and shows on its status; the
    // scheduler need not retry the whole wake-up for it.
    StatusCode::NO_CONTENT
}
