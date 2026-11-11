//! Model-call spans in Langfuse's OpenTelemetry mapping.
//!
//! [`generation_span`] opens a `gen_ai.generation` span that Langfuse reads as
//! a generation observation, and that carries the ENG-74 run contract:
//! `coda.run_id`, `coda.job`, `coda.recipe_version`, `coda.origin` (also
//! written to `langfuse.environment`), `coda.scenario_id`,
//! `langfuse.session.id`, and `langfuse.trace.tags`.
//!
//! # Content policy
//!
//! Prompt and completion text is attached only when
//! `CODA_TELEMETRY_CONTENT=full`. The default, `none`, records neither, for
//! both origins. Simulation runs may run with `full` anywhere; real runs may
//! be switched to `full` only when spans go to the self-hosted, in-project
//! Langfuse. The operations backend, Datadog or Grafana, must never receive
//! content: the collector pipeline routing spans to either backend drops
//! `langfuse.observation.input` and `langfuse.observation.output`.
//! [`record_error`] never records content.

use std::env;

use opentelemetry::{Array, StringValue, Value};
use tracing::Span;
use tracing::field::Empty;
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Where a run's input came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Origin {
    /// Synthetic or simulation traffic, kept in its own Langfuse environment.
    Simulation,
    /// Real physician or patient traffic.
    Real,
}

impl Origin {
    /// `simulation` or `real`, the value of `coda.origin` and
    /// `langfuse.environment`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Simulation => "simulation",
            Self::Real => "real",
        }
    }
}

/// The ENG-74 attribute contract for one model call.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunContext {
    /// The canonical run id. Until a run record exists it is the provider
    /// call id.
    pub run_id: String,
    /// The job, for example `physician_chat`.
    pub job: &'static str,
    /// The git SHA of `skills/` plus the recipe files. See [`recipe_version`].
    pub recipe_version: String,
    /// Simulation or real traffic.
    pub origin: Origin,
    /// The simulation scenario, when one is running.
    pub scenario_id: Option<String>,
    /// The chat id, used as the Langfuse session.
    pub session_id: Option<String>,
}

/// Whether prompt and completion text may be attached to spans.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ContentPolicy {
    /// No content. The default.
    #[default]
    None,
    /// Prompt and completion text are attached.
    Full,
}

impl ContentPolicy {
    /// Parses `none` or `full`. Anything else is `None`.
    pub fn parse(raw: Option<&str>) -> Self {
        match raw.map(str::trim) {
            Some(value) if value.eq_ignore_ascii_case("full") => Self::Full,
            _ => Self::None,
        }
    }
}

/// The policy from `CODA_TELEMETRY_CONTENT` (`none` by default, or `full`).
pub fn content_policy() -> ContentPolicy {
    ContentPolicy::parse(env::var("CODA_TELEMETRY_CONTENT").ok().as_deref())
}

/// The recipe version: `CODA_RECIPE_VERSION` at run time, else at build
/// time, else `unknown`.
pub fn recipe_version() -> String {
    env::var("CODA_RECIPE_VERSION")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| option_env!("CODA_RECIPE_VERSION").map(str::to_owned))
        .unwrap_or_else(|| "unknown".to_owned())
}

/// Opens a `gen_ai.generation` span for one model call.
///
/// `system` is the provider (`openai`, `codex`) and `adapter` the Coda adapter
/// (`openai-responses`, `codex`). The caller records `gen_ai.request.model`
/// when it dispatches the actual request. The span's parent is the current span.
/// Usage, input, output, and status are recorded later through [`record_usage`],
/// [`record_input`], [`record_output`], and [`record_error`].
pub fn generation_span(ctx: &RunContext, system: &str, adapter: &str) -> Span {
    let origin = ctx.origin.as_str();
    let span = tracing::info_span!(
        "gen_ai.generation",
        otel.kind = "client",
        langfuse.observation.type = "generation",
        gen_ai.system = %system,
        gen_ai.request.model = Empty,
        coda.adapter = %adapter,
        coda.run_id = %ctx.run_id,
        coda.job = ctx.job,
        coda.recipe_version = %ctx.recipe_version,
        coda.origin = origin,
        langfuse.environment = origin,
        coda.scenario_id = Empty,
        langfuse.session.id = Empty,
        gen_ai.usage.input_tokens = Empty,
        gen_ai.usage.output_tokens = Empty,
        langfuse.observation.input = Empty,
        langfuse.observation.output = Empty,
        otel.status_code = Empty,
        error.message = Empty,
    );
    if let Some(scenario) = &ctx.scenario_id {
        span.record("coda.scenario_id", scenario.as_str());
    }
    if let Some(session) = &ctx.session_id {
        span.record("langfuse.session.id", session.as_str());
    }
    let mut tags: Vec<StringValue> = vec![ctx.job.into(), origin.into()];
    if let Some(scenario) = &ctx.scenario_id {
        tags.push(scenario.clone().into());
    }
    span.set_attribute("langfuse.trace.tags", Value::Array(Array::String(tags)));
    span
}

/// Records the prompt when [`content_policy`] is `full`.
pub fn record_input(span: &Span, text: &str) {
    record_input_with(content_policy(), span, text);
}

/// Records the completion when [`content_policy`] is `full`.
pub fn record_output(span: &Span, text: &str) {
    record_output_with(content_policy(), span, text);
}

/// Records the prompt when `policy` is `Full`.
pub fn record_input_with(policy: ContentPolicy, span: &Span, text: &str) {
    if policy == ContentPolicy::Full {
        span.record("langfuse.observation.input", text);
    }
}

/// Records the completion when `policy` is `Full`.
pub fn record_output_with(policy: ContentPolicy, span: &Span, text: &str) {
    if policy == ContentPolicy::Full {
        span.record("langfuse.observation.output", text);
    }
}

/// Records token usage.
pub fn record_usage(span: &Span, input_tokens: u64, output_tokens: u64) {
    span.record("gen_ai.usage.input_tokens", input_tokens);
    span.record("gen_ai.usage.output_tokens", output_tokens);
}

/// Marks the span as failed with `message`. `message` must describe the
/// failure, never the prompt or completion.
pub fn record_error(span: &Span, message: &str) {
    span.record("otel.status_code", "ERROR");
    span.record("error.message", message);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{attribute, capture};

    fn context() -> RunContext {
        RunContext {
            run_id: "physician-chat-1".to_owned(),
            job: "physician_chat",
            recipe_version: "abc123".to_owned(),
            origin: Origin::Simulation,
            scenario_id: Some("connected-walk".to_owned()),
            session_id: Some("chat-1".to_owned()),
        }
    }

    #[test]
    fn generation_spans_carry_the_run_contract() {
        let spans = capture(|| {
            let span = generation_span(&context(), "codex", "codex");
            span.record("gen_ai.request.model", "gpt-6-sol");
            record_usage(&span, 12, 34);
        });
        let span = spans
            .iter()
            .find(|span| span.name == "gen_ai.generation")
            .expect("generation span");
        for (key, value) in [
            ("langfuse.observation.type", "generation"),
            ("gen_ai.system", "codex"),
            ("gen_ai.request.model", "gpt-6-sol"),
            ("coda.adapter", "codex"),
            ("coda.run_id", "physician-chat-1"),
            ("coda.job", "physician_chat"),
            ("coda.recipe_version", "abc123"),
            ("coda.origin", "simulation"),
            ("langfuse.environment", "simulation"),
            ("coda.scenario_id", "connected-walk"),
            ("langfuse.session.id", "chat-1"),
            ("gen_ai.usage.input_tokens", "12"),
            ("gen_ai.usage.output_tokens", "34"),
        ] {
            assert_eq!(attribute(span, key).as_deref(), Some(value), "{key}");
        }
        let tags = attribute(span, "langfuse.trace.tags").expect("tags");
        assert!(tags.contains("physician_chat"), "{tags}");
        assert_eq!(span.span_kind, opentelemetry::trace::SpanKind::Client);
    }

    #[test]
    fn content_is_recorded_only_under_the_full_policy() {
        let spans = capture(|| {
            let span = generation_span(&context(), "openai", "openai-responses");
            record_input_with(ContentPolicy::None, &span, "secret prompt");
            record_output_with(ContentPolicy::None, &span, "secret reply");
        });
        let span = &spans[0];
        assert!(attribute(span, "langfuse.observation.input").is_none());
        assert!(attribute(span, "langfuse.observation.output").is_none());
        assert!(
            !format!("{:?}", span.attributes).contains("secret"),
            "{:?}",
            span.attributes
        );

        let spans = capture(|| {
            let span = generation_span(&context(), "openai", "openai-responses");
            record_input_with(ContentPolicy::Full, &span, "prompt");
            record_output_with(ContentPolicy::Full, &span, "reply");
        });
        let span = &spans[0];
        assert_eq!(
            attribute(span, "langfuse.observation.input").as_deref(),
            Some("prompt")
        );
        assert_eq!(
            attribute(span, "langfuse.observation.output").as_deref(),
            Some("reply")
        );
    }

    #[test]
    fn errors_set_the_status_without_content() {
        let spans = capture(|| {
            let span = generation_span(&context(), "codex", "codex");
            record_error(&span, "Codex exec exited 1");
        });
        let span = &spans[0];
        assert!(matches!(
            span.status,
            opentelemetry::trace::Status::Error { .. }
        ));
        assert_eq!(
            attribute(span, "error.message").as_deref(),
            Some("Codex exec exited 1")
        );
    }

    #[test]
    fn the_policy_defaults_to_none() {
        assert_eq!(ContentPolicy::parse(None), ContentPolicy::None);
        assert_eq!(ContentPolicy::parse(Some("bogus")), ContentPolicy::None);
        assert_eq!(ContentPolicy::parse(Some(" FULL ")), ContentPolicy::Full);
        assert_eq!(Origin::Real.as_str(), "real");
    }
}
