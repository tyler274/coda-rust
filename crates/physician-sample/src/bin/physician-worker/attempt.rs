//! One `physician.worker.attempt` consumer span per claimed attempt,
//! correlated with the product's canonical saved attempt.
//!
//! `coda.run_id` is the attempt this worker claimed, `coda.work_id` the work
//! it belongs to, and `coda.predecessor_run_id` the attempt it succeeds, all
//! read from the product's own records; nothing here is a second run ledger.
//! Failure signals carry fixed descriptions and a bounded `error.type`, never
//! an error payload, prompt, instruction, or output.
use std::collections::HashMap;
use std::time::Duration;

use harness::engine::{Event, EventKind};
use harness::execution::Record;
use local_product::runtime::delivery::{DeliveryKind, Retry, TraceContext};
use sea_orm::DbErr;
use serde_json::Value;
use telemetry::propagation::{self, Relation};
use tracing::Span;
use tracing::field::Empty;

/// What the pull loop does with a delivery once this worker is done with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Next {
    /// Nothing more from this worker: the work is settled, failed, released,
    /// or left to the poll.
    Done,
    /// The attempt failed and the work is due again after this long.
    RetryAfter(Duration),
}

/// The bounded `error.type` of an asynchronous worker failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Failure {
    /// The model provider could not be constructed for this attempt.
    ProviderInit,
    /// The chat model call or its execution failed.
    ModelCall,
    /// The durable claim could not be read or written.
    Claim,
    /// The revision attempt ran past its deadline.
    Timeout,
    /// The failed attempt could not be returned to pending.
    RetryPersist,
    /// The reply revision's execution failed.
    Revision,
}

impl Failure {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::ProviderInit => "provider_init",
            Self::ModelCall => "model_call",
            Self::Claim => "claim",
            Self::Timeout => "timeout",
            Self::RetryPersist => "retry_persist",
            Self::Revision => "revision",
        }
    }

    /// The fixed, content-free span status description.
    fn description(self) -> &'static str {
        match self {
            Self::ProviderInit => "model provider unavailable",
            Self::ModelCall => "model execution failed",
            Self::Claim => "attempt claim failed",
            Self::Timeout => "attempt deadline exceeded",
            Self::RetryPersist => "failed attempt not returned to pending",
            Self::Revision => "reply revision failed",
        }
    }
}

/// How the attempt ended for the product.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Outcome {
    /// The attempt published its result.
    Committed,
    /// Stop, a newer input, or completion already owned the work.
    Settled,
    /// The attempt failed and the work is due again after its backoff.
    Retry,
    /// The attempt failed and was the last one: the work ended as failed.
    GaveUp,
    /// The worker was stopping and handed its claim back.
    Released,
    /// The claim itself failed.
    ClaimError,
}

impl Outcome {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Committed => "committed",
            Self::Settled => "settled",
            Self::Retry => "retry",
            Self::GaveUp => "gave_up",
            Self::Released => "released",
            Self::ClaimError => "claim_error",
        }
    }
}

/// Opens the consumer span for one attempt. It is always a new trace with no
/// ambient parent. Accepting the work and running it are separate in time and
/// one accepted input can have several attempts, so the span links to
/// `origin`, the request that made the work runnable, and never continues it.
/// Without a valid `origin` the span has no relation.
pub(super) fn span(kind: DeliveryKind, job: &'static str, origin: Option<&TraceContext>) -> Span {
    let span = tracing::info_span!(
        parent: None,
        "physician.worker.attempt",
        otel.kind = "consumer",
        messaging.system = "postgresql",
        messaging.operation.type = "process",
        coda.job = job,
        coda.delivery_kind = kind.as_str(),
        coda.trace_relation = Empty,
        coda.run_id = Empty,
        coda.work_id = Empty,
        coda.predecessor_run_id = Empty,
        coda.input_revision = Empty,
        coda.attempt_number = Empty,
        coda.outcome = Empty,
        otel.status_code = Empty,
        otel.status_description = Empty,
        error.type = Empty,
        error.message = Empty,
    );
    let remote = origin.and_then(|origin| {
        let mut carrier = HashMap::from([(
            propagation::TRACEPARENT.to_owned(),
            origin.traceparent.clone(),
        )]);
        if let Some(state) = &origin.tracestate {
            carrier.insert(propagation::TRACESTATE.to_owned(), state.clone());
        }
        propagation::remote_context(&carrier)
    });
    let applied = propagation::relate(&span, remote.as_ref(), Relation::Link);
    span.record("coda.trace_relation", applied.as_str());
    span
}

/// What a failed attempt's saved retry means for the span and the pull loop.
/// A retry that could not be saved is logged; the attempt's lease then runs
/// out and the next claim records it.
pub(super) fn retried(
    span: &Span,
    job: &'static str,
    run_id: &str,
    saved: Result<Retry, DbErr>,
) -> (Outcome, Next) {
    match saved {
        Ok(Retry::After(delay)) => (Outcome::Retry, Next::RetryAfter(delay)),
        Ok(Retry::GaveUp) => (Outcome::GaveUp, Next::Done),
        Ok(Retry::NotCurrent) => (Outcome::Settled, Next::Done),
        Err(_) => {
            retry_not_persisted(span, job, run_id);
            (Outcome::Retry, Next::Done)
        }
    }
}

/// The canonical attempt this worker claimed.
pub(super) struct Claimed<'a> {
    pub run_id: &'a str,
    pub work_id: Option<&'a str>,
    pub predecessor_run_id: Option<&'a str>,
    pub input_revision: Option<u64>,
    pub attempt_number: u32,
}

/// Records the claimed attempt's identity on the attempt span.
pub(super) fn record_claim(span: &Span, claimed: &Claimed<'_>) {
    span.record("coda.run_id", claimed.run_id);
    if let Some(work) = claimed.work_id {
        span.record("coda.work_id", work);
    }
    if let Some(predecessor) = claimed.predecessor_run_id {
        span.record("coda.predecessor_run_id", predecessor);
    }
    if let Some(revision) = claimed.input_revision {
        span.record("coda.input_revision", revision);
    }
    span.record("coda.attempt_number", claimed.attempt_number);
}

/// Records how the attempt ended. A failure also marks the span `ERROR`
/// with fixed fields and logs `physician.worker.attempt_failed`; the log line
/// carries the trace and span ids, and `coda.run_id` once an attempt was
/// claimed. Nothing recorded here comes from an error payload.
pub(super) fn finish(
    span: &Span,
    job: &'static str,
    run_id: Option<&str>,
    outcome: Outcome,
    failure: Option<Failure>,
) {
    span.record("coda.outcome", outcome.as_str());
    let Some(failure) = failure else {
        return;
    };
    span.record("otel.status_code", "ERROR");
    span.record("otel.status_description", failure.description());
    span.record("error.type", failure.as_str());
    span.record("error.message", failure.description());
    tracing::error!(
        parent: span,
        event = "physician.worker.attempt_failed",
        error.type = failure.as_str(),
        coda.job = job,
        coda.outcome = outcome.as_str(),
        coda.run_id = run_id.unwrap_or(""),
        "physician worker attempt failed"
    );
}

/// Logs a failure to return a failed attempt to pending. The attempt span
/// keeps its primary failure.
pub(super) fn retry_not_persisted(span: &Span, job: &'static str, run_id: &str) {
    let failure = Failure::RetryPersist;
    tracing::error!(
        parent: span,
        event = "physician.worker.retry_not_persisted",
        error.type = failure.as_str(),
        coda.job = job,
        coda.run_id = run_id,
        "physician worker failed attempt not returned to pending"
    );
}

/// Token usage the provider reported for an attempt, summed over its calls.
/// Unknown stays unknown: one call without both counts makes the total
/// unknown rather than an undercount.
#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct Tally {
    input: u64,
    output: u64,
    calls: u32,
    unknown: bool,
}

impl Tally {
    pub(super) fn add(&mut self, input: Option<u64>, output: Option<u64>) {
        self.calls += 1;
        match (input, output) {
            (Some(input), Some(output)) => {
                self.input = self.input.saturating_add(input);
                self.output = self.output.saturating_add(output);
            }
            _ => self.unknown = true,
        }
    }

    /// `(input, output)` when every call reported both counts.
    pub(super) fn known(&self) -> Option<(u64, u64)> {
        (self.calls > 0 && !self.unknown).then_some((self.input, self.output))
    }

    /// Usage from a chat attempt's retained provider receipts.
    pub(super) fn from_events(events: &[Event]) -> Self {
        let mut tally = Self::default();
        for event in events {
            if let EventKind::ProviderReturned(turn) = &event.kind {
                let count = |key: &str| {
                    turn.receipt
                        .pointer(&format!("/usage/{key}"))
                        .and_then(Value::as_u64)
                };
                tally.add(count("input_tokens"), count("output_tokens"));
            }
        }
        tally
    }

    /// Usage from one retained revision record.
    pub(super) fn record(&mut self, record: &Record) {
        if let Record::Exchange { response, .. } = record {
            self.add(response.usage.input_tokens, response.usage.output_tokens);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use harness::engine::Turn;
    use serde_json::json;

    const TRACE: &str = "0af7651916cd43dd8448eb211c80319c";
    const PARENT: &str = "b7ad6b7169203331";

    fn origin(traceparent: &str) -> TraceContext {
        TraceContext {
            traceparent: traceparent.into(),
            tracestate: Some("dd=s:1".into()),
        }
    }

    fn capture(body: impl FnOnce()) -> Vec<opentelemetry_sdk::trace::SpanData> {
        use opentelemetry::trace::TracerProvider as _;
        use tracing_subscriber::layer::SubscriberExt;
        let exporter = opentelemetry_sdk::trace::InMemorySpanExporter::default();
        let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();
        let subscriber = tracing_subscriber::registry()
            .with(tracing_opentelemetry::layer().with_tracer(provider.tracer("test")));
        tracing::subscriber::with_default(subscriber, body);
        provider.force_flush().unwrap();
        exporter.get_finished_spans().unwrap()
    }

    fn attribute(span: &opentelemetry_sdk::trace::SpanData, key: &str) -> Option<String> {
        span.attributes
            .iter()
            .find(|kv| kv.key.as_str() == key)
            .map(|kv| kv.value.to_string())
    }

    #[test]
    fn every_attempt_links_to_the_accepting_request_in_its_own_trace() {
        use opentelemetry::trace::{SpanId, TraceId};
        let accepted = origin(&format!("00-{TRACE}-{PARENT}-01"));
        let spans = capture(|| {
            for (number, run, previous, failure) in [
                (1, "saved-attempt-one", None, Some(Failure::ProviderInit)),
                (
                    2,
                    "saved-attempt-two",
                    Some("saved-attempt-one"),
                    Some(Failure::Revision),
                ),
                (3, "saved-attempt-three", Some("saved-attempt-two"), None),
            ] {
                let span = span(DeliveryKind::ChatTurn, "physician_chat", Some(&accepted));
                record_claim(
                    &span,
                    &Claimed {
                        run_id: run,
                        work_id: Some("saved-work"),
                        predecessor_run_id: previous,
                        input_revision: Some(1),
                        attempt_number: number,
                    },
                );
                finish(
                    &span,
                    "physician_chat",
                    Some(run),
                    if failure.is_some() {
                        Outcome::Retry
                    } else {
                        Outcome::Committed
                    },
                    failure,
                );
            }
            // Neither an invalid nor a missing origin inherits an ambient span.
            let ambient = tracing::info_span!("http.server");
            ambient.in_scope(|| {
                drop(span(DeliveryKind::ChatTurn, "physician_chat", None));
                drop(span(
                    DeliveryKind::ReplyRevision,
                    "physician_reply_revision",
                    Some(&origin("invalid")),
                ));
            });
        });
        let attempts: Vec<_> = spans
            .iter()
            .filter(|span| span.name == "physician.worker.attempt")
            .collect();
        assert_eq!(attempts.len(), 5);
        let mut traces = std::collections::HashSet::new();
        for (span, run) in attempts.iter().zip([
            "saved-attempt-one",
            "saved-attempt-two",
            "saved-attempt-three",
        ]) {
            assert_eq!(attribute(span, "coda.run_id").as_deref(), Some(run));
            assert_eq!(span.span_kind, opentelemetry::trace::SpanKind::Consumer);
            assert_eq!(span.parent_span_id, SpanId::INVALID);
            assert_eq!(
                attribute(span, "coda.trace_relation").as_deref(),
                Some("link")
            );
            assert_eq!(span.links.len(), 1);
            let link = &span.links[0].span_context;
            assert_eq!(link.trace_id(), TraceId::from_hex(TRACE).unwrap());
            assert_eq!(link.span_id(), SpanId::from_hex(PARENT).unwrap());
            assert_eq!(link.trace_state().get("dd"), Some("s:1"));
            assert_ne!(span.span_context.trace_id(), link.trace_id());
            assert!(traces.insert(span.span_context.trace_id()));
        }
        for span in &attempts[..2] {
            assert!(matches!(
                span.status,
                opentelemetry::trace::Status::Error { .. }
            ));
            assert!(span.events.iter().any(|event| {
                event.attributes.iter().any(|kv| {
                    kv.key.as_str() == "event"
                        && kv.value.to_string() == "physician.worker.attempt_failed"
                })
            }));
        }
        for span in &attempts[3..] {
            assert_eq!(span.parent_span_id, SpanId::INVALID);
            assert!(span.links.is_empty());
            assert_eq!(
                attribute(span, "coda.trace_relation").as_deref(),
                Some("root")
            );
            assert!(attribute(span, "coda.run_id").is_none());
        }
        assert_eq!(
            attribute(attempts[4], "coda.delivery_kind").as_deref(),
            Some("reply_revision")
        );
    }

    #[test]
    fn a_failed_attempt_is_recorded_without_content() {
        use opentelemetry::trace::Status;
        let spans = capture(|| {
            let span = span(DeliveryKind::ChatTurn, "physician_chat", None);
            record_claim(
                &span,
                &Claimed {
                    run_id: "attempt-2",
                    work_id: Some("work-1"),
                    predecessor_run_id: Some("attempt-1"),
                    input_revision: Some(3),
                    attempt_number: 2,
                },
            );
            finish(
                &span,
                "physician_chat",
                Some("attempt-2"),
                Outcome::Retry,
                Some(Failure::ModelCall),
            );
        });
        let span = &spans[0];
        assert_eq!(span.name, "physician.worker.attempt");
        for (key, value) in [
            ("coda.job", "physician_chat"),
            ("coda.delivery_kind", "chat_turn"),
            ("messaging.system", "postgresql"),
            ("coda.trace_relation", "root"),
            ("coda.run_id", "attempt-2"),
            ("coda.work_id", "work-1"),
            ("coda.predecessor_run_id", "attempt-1"),
            ("coda.input_revision", "3"),
            ("coda.attempt_number", "2"),
            ("coda.outcome", "retry"),
            ("error.type", "model_call"),
            ("error.message", "model execution failed"),
        ] {
            assert_eq!(attribute(span, key).as_deref(), Some(value), "{key}");
        }
        assert_eq!(span.status, Status::error("model execution failed"));
        let event = span
            .events
            .iter()
            .find(|event| event.name == "physician worker attempt failed")
            .expect("failure event");
        let event_attribute = |key: &str| {
            event
                .attributes
                .iter()
                .find(|kv| kv.key.as_str() == key)
                .map(|kv| kv.value.to_string())
        };
        assert_eq!(
            event_attribute("event").as_deref(),
            Some("physician.worker.attempt_failed")
        );
        assert_eq!(event_attribute("error.type").as_deref(), Some("model_call"));
        assert_eq!(event_attribute("coda.outcome").as_deref(), Some("retry"));
    }

    #[test]
    fn a_saved_retry_decides_the_outcome_and_what_the_loop_does_next() {
        let span = Span::none();
        let delay = Duration::from_secs(20);
        for (saved, expected) in [
            (
                Ok(Retry::After(delay)),
                (Outcome::Retry, Next::RetryAfter(delay)),
            ),
            (Ok(Retry::GaveUp), (Outcome::GaveUp, Next::Done)),
            (Ok(Retry::NotCurrent), (Outcome::Settled, Next::Done)),
            (
                Err(DbErr::Custom("unavailable".into())),
                (Outcome::Retry, Next::Done),
            ),
        ] {
            assert_eq!(retried(&span, "physician_chat", "run", saved), expected);
        }
    }

    #[test]
    fn failure_types_and_outcomes_are_a_fixed_vocabulary() {
        let failures = [
            Failure::ProviderInit,
            Failure::ModelCall,
            Failure::Claim,
            Failure::Timeout,
            Failure::RetryPersist,
            Failure::Revision,
        ]
        .map(Failure::as_str);
        assert_eq!(
            failures,
            [
                "provider_init",
                "model_call",
                "claim",
                "timeout",
                "retry_persist",
                "revision"
            ]
        );
        let outcomes = [
            Outcome::Committed,
            Outcome::Settled,
            Outcome::Retry,
            Outcome::GaveUp,
            Outcome::Released,
            Outcome::ClaimError,
        ]
        .map(Outcome::as_str);
        assert_eq!(
            outcomes,
            [
                "committed",
                "settled",
                "retry",
                "gave_up",
                "released",
                "claim_error"
            ]
        );
    }

    fn returned(receipt: Value) -> Event {
        Event {
            attempt_id: "a".into(),
            sequence: 1,
            kind: EventKind::ProviderReturned(Turn {
                items: vec![],
                receipt,
            }),
        }
    }

    #[test]
    fn usage_is_reported_only_when_every_call_reported_it() {
        let usage = |input: u64, output: u64| json!({"id": "r", "usage": {"input_tokens": input, "output_tokens": output}});
        let both = [returned(usage(10, 2)), returned(usage(5, 1))];
        assert_eq!(Tally::from_events(&both).known(), Some((15, 3)));
        // The fixed provider reports no usage: nothing is invented.
        let fixed = [returned(json!({"provider": "fixed"}))];
        assert_eq!(Tally::from_events(&fixed).known(), None);
        let partial = [returned(usage(10, 2)), returned(json!({"usage": null}))];
        assert_eq!(Tally::from_events(&partial).known(), None);
        assert_eq!(Tally::from_events(&[]).known(), None);
    }
}
