//! Makes the layout of each `(source, record type)` an answer cites for the first time, after
//! that answer is published and announced. The answer never waits on it: the physician already
//! reads the answer, and its citations read through the general view until a layout is saved.
//!
//! It runs inside the push request, before the push is acknowledged, because the worker has
//! request-based CPU: a task left running after the response would be throttled. A failed or
//! refused layout only means no layout; it is logged with its cause and the push is still
//! acknowledged, since the turn is committed. A key another task on this instance is already
//! making is skipped, and writers on other instances settle through the store, where the first
//! layout stored wins.
use std::time::{Duration, Instant};

use harness::engine::{Item, Provider, Request, Turn};
use local_product::ModelBinding;
use local_product::physician_layouts::LayoutKey;
use local_product::runtime::{Scope, async_trait};
use physician_sample::layout_generation::{Outcome, Unsaved, ensure_layout, first_of_each_key};
use serde_json::json;

use super::{Worker, configured};

/// The longest the layout phase for one answer may run: the conversation read, every layout's
/// store reads and writes, and its model calls.
const DEADLINE: Duration = Duration::from_secs(100);

/// The push subscription's ack deadline: a push not acknowledged within it is redelivered.
const PUSH_DEADLINE: Duration = Duration::from_secs(600);

/// Time kept back from the push deadline for the response to reach Pub/Sub after the phase.
const ACK_RESERVE: Duration = Duration::from_secs(10);

/// How long the layout phase may run when the push arrived `elapsed` ago: [`DEADLINE`], cut
/// short so the phase ends [`ACK_RESERVE`] before the push deadline, and zero once that is past.
fn phase_budget(elapsed: Duration) -> Duration {
    PUSH_DEADLINE
        .saturating_sub(ACK_RESERVE)
        .saturating_sub(elapsed)
        .min(DEADLINE)
}

/// Who makes layouts on this worker.
pub(super) enum LayoutSource {
    /// The fixed proof worker was given no layout reply, so it makes none.
    Off,
    /// The configured background model, through the Responses adapter.
    Api { key: String, binding: ModelBinding },
    /// Non-hosted proof only: every layout call gets this reply or fails.
    Fixed(FixedLayout),
}

/// The fixed proof worker's layout model.
#[derive(Clone, Debug)]
pub(super) enum FixedLayout {
    /// Each call replies with this text.
    Reply(String),
    /// Each call fails, as a provider error would.
    Fails,
}

/// Reads the fixed layout model from `PHYSICIAN_WORKER_FIXED_LAYOUT` (the reply text) or
/// `PHYSICIAN_WORKER_FIXED_LAYOUT_FAILS=1`. Both are non-hosted proof knobs that need the fixed
/// chat reply, and they exclude each other.
pub(super) fn fixed_from_env(
    get: &impl Fn(&str) -> Option<String>,
    hosted: bool,
    fixed_text: bool,
) -> Result<Option<FixedLayout>, String> {
    let reply = configured(get, "PHYSICIAN_WORKER_FIXED_LAYOUT");
    let fails = configured(get, "PHYSICIAN_WORKER_FIXED_LAYOUT_FAILS");
    if hosted && (reply.is_some() || fails.is_some()) {
        return Err(
            "PHYSICIAN_WORKER_FIXED_LAYOUT variables are refused in hosted execution".into(),
        );
    }
    let fails = match fails.as_deref() {
        None | Some("0") => false,
        Some("1") => true,
        Some(_) => return Err("PHYSICIAN_WORKER_FIXED_LAYOUT_FAILS must be 0 or 1".into()),
    };
    let fixed = match (reply, fails) {
        (Some(_), true) => {
            return Err(
                "PHYSICIAN_WORKER_FIXED_LAYOUT and PHYSICIAN_WORKER_FIXED_LAYOUT_FAILS exclude each other"
                    .into(),
            );
        }
        (Some(reply), false) => Some(FixedLayout::Reply(reply)),
        (None, true) => Some(FixedLayout::Fails),
        (None, false) => None,
    };
    if fixed.is_some() && !fixed_text {
        return Err(
            "PHYSICIAN_WORKER_FIXED_LAYOUT variables require PHYSICIAN_WORKER_FIXED_TEXT".into(),
        );
    }
    Ok(fixed)
}

impl LayoutSource {
    fn provider(&self) -> Result<Option<LayoutModel>, String> {
        match self {
            Self::Off => Ok(None),
            Self::Api { key, binding } => Ok(Some(LayoutModel::Api(
                harness::responses::Responses::new(key.clone(), binding.model.clone())?
                    .reasoning_effort(binding.effort.clone()),
            ))),
            Self::Fixed(fixed) => Ok(Some(LayoutModel::Fixed(fixed.clone()))),
        }
    }
}

enum LayoutModel {
    Api(harness::responses::Responses),
    Fixed(FixedLayout),
}

#[async_trait]
impl Provider for LayoutModel {
    fn request_bytes(&self, request: &Request<'_>) -> Result<usize, String> {
        match self {
            Self::Api(provider) => provider.request_bytes(request),
            Self::Fixed(_) => Ok(1),
        }
    }

    async fn complete(&mut self, request: Request<'_>) -> Result<Turn, String> {
        match self {
            Self::Api(provider) => provider.complete(request).await,
            Self::Fixed(FixedLayout::Fails) => Err("fixed layout model failed".into()),
            Self::Fixed(FixedLayout::Reply(text)) => Ok(Turn {
                items: vec![Item::Message {
                    role: "assistant".into(),
                    text: text.clone(),
                }],
                receipt: json!({ "provider": "fixed-layout" }),
            }),
        }
    }
}

/// Makes a layout for each `(source, record type)` the published answer in `scope` cites that
/// has none yet. `run_id` is the attempt that published it, and `received` is when its push
/// arrived; the whole phase ends within [`phase_budget`] of now.
pub(super) async fn after_answer(worker: &Worker, scope: &Scope, run_id: &str, received: Instant) {
    if matches!(worker.layouts, LayoutSource::Off) {
        return;
    }
    let budget = phase_budget(received.elapsed());
    if tokio::time::timeout(budget, phase(worker, scope, run_id))
        .await
        .is_err()
    {
        tracing::warn!(
            budget_seconds = budget.as_secs(),
            "source layouts ran past their deadline; the rest are made on a later citation"
        );
    }
}

/// The layout phase itself: the published answer's read, then each new key in citation order.
async fn phase(worker: &Worker, scope: &Scope, run_id: &str) {
    let conversation = match worker.store.read(scope).await {
        Ok(Some(conversation)) => conversation,
        Ok(None) => {
            tracing::warn!("published conversation not found; no source layouts made");
            return;
        }
        Err(error) => {
            tracing::warn!(
                error = &error as &dyn std::error::Error,
                "published conversation unreadable; no source layouts made"
            );
            return;
        }
    };
    let Some(answer) = conversation
        .messages
        .iter()
        .rev()
        .find(|m| m.role == "assistant")
    else {
        return;
    };
    for (key, cited) in first_of_each_key(&answer.citations) {
        one(worker, key, cited, run_id).await;
    }
}

async fn one(
    worker: &Worker,
    key: LayoutKey<'_>,
    cited: &local_product::physician_chat::Citation,
    run_id: &str,
) {
    let LayoutKey {
        source_id,
        record_type,
    } = key;
    let Some(_held) = worker.layouts_in_flight.hold(key) else {
        tracing::info!(
            source_id,
            record_type,
            "source layout already being made on this instance"
        );
        return;
    };
    let mut provider = match worker.layouts.provider() {
        Ok(Some(provider)) => provider,
        Ok(None) => return,
        Err(reason) => {
            tracing::warn!(
                source_id,
                record_type,
                reason,
                "layout model unavailable; source layout not made"
            );
            return;
        }
    };
    match ensure_layout(&worker.store, &mut provider, key, cited, run_id).await {
        Ok(Outcome::AlreadyStored) => {}
        Ok(Outcome::Stored { sha256 }) => {
            tracing::info!(
                source_id,
                record_type,
                layout_sha256 = sha256,
                "source layout stored"
            );
        }
        Ok(Outcome::Unchanged) => {
            tracing::info!(
                source_id,
                record_type,
                "source layout was already stored with these bytes"
            );
        }
        Ok(Outcome::LostRace { stored_sha256 }) => {
            tracing::info!(
                source_id,
                record_type,
                layout_sha256 = stored_sha256,
                "another writer stored this source layout first; it stays"
            );
        }
        Err(error) => {
            tracing::warn!(
                source_id,
                record_type,
                error = causes(&error),
                "source layout not saved; its records read through the general view"
            );
        }
    }
}

/// `error` and its causes, outermost first: a refused pointer and role, or a store failure. A
/// JSON error names only its kind and position, since its text can quote the record or the reply,
/// and worker logs carry no clinical content.
fn causes(error: &Unsaved) -> String {
    if let Unsaved::RecordNotJson(json) | Unsaved::NotALayout(json) = error {
        return format!(
            "{error}: {:?} error at line {}, column {}",
            json.classify(),
            json.line(),
            json.column()
        );
    }
    let mut text = error.to_string();
    let mut source = std::error::Error::source(error);
    while let Some(cause) = source {
        text.push_str(": ");
        text.push_str(&cause.to_string());
        source = cause.source();
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The phase gets its full bound early in a push, is cut short so it ends before the push
    /// deadline with time left to acknowledge, and gets nothing once that time is gone.
    #[test]
    fn the_layout_phase_ends_before_the_push_deadline() {
        let secs = Duration::from_secs;
        assert_eq!(phase_budget(Duration::ZERO), DEADLINE);
        assert_eq!(phase_budget(secs(480)), DEADLINE);
        assert_eq!(phase_budget(secs(530)), secs(60));
        assert_eq!(phase_budget(secs(590)), Duration::ZERO);
        assert_eq!(phase_budget(secs(900)), Duration::ZERO);
        for elapsed in (0..=700).map(secs) {
            let ends = elapsed + phase_budget(elapsed);
            assert!(
                ends <= (PUSH_DEADLINE - ACK_RESERVE).max(elapsed),
                "{elapsed:?}"
            );
        }
    }

    #[test]
    fn the_fixed_layout_knobs_are_proof_only_and_need_the_fixed_reply() {
        let env = |pairs: &'static [(&'static str, &'static str)]| {
            move |name: &str| {
                pairs
                    .iter()
                    .find(|(key, _)| *key == name)
                    .map(|(_, value)| (*value).to_owned())
            }
        };
        let reply = env(&[("PHYSICIAN_WORKER_FIXED_LAYOUT", "{}")]);
        let fails = env(&[("PHYSICIAN_WORKER_FIXED_LAYOUT_FAILS", "1")]);
        let both = env(&[
            ("PHYSICIAN_WORKER_FIXED_LAYOUT", "{}"),
            ("PHYSICIAN_WORKER_FIXED_LAYOUT_FAILS", "1"),
        ]);
        assert!(
            fixed_from_env(&reply, true, true)
                .unwrap_err()
                .contains("hosted")
        );
        assert!(
            fixed_from_env(&fails, true, true)
                .unwrap_err()
                .contains("hosted")
        );
        assert!(
            fixed_from_env(&reply, false, false)
                .unwrap_err()
                .contains("require")
        );
        assert!(
            fixed_from_env(&both, false, true)
                .unwrap_err()
                .contains("exclude")
        );
        let bad = env(&[("PHYSICIAN_WORKER_FIXED_LAYOUT_FAILS", "yes")]);
        assert!(fixed_from_env(&bad, false, true).is_err());
        assert!(matches!(
            fixed_from_env(&reply, false, true),
            Ok(Some(FixedLayout::Reply(text))) if text == "{}"
        ));
        assert!(matches!(
            fixed_from_env(&fails, false, true),
            Ok(Some(FixedLayout::Fails))
        ));
        assert!(matches!(fixed_from_env(&env(&[]), false, true), Ok(None)));
        assert!(matches!(fixed_from_env(&env(&[]), true, false), Ok(None)));
    }
}
