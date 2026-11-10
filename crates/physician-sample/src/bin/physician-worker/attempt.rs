//! One `physician.worker.attempt` consumer span per push delivery, correlated
//! with the product's canonical saved attempt.
//!
//! The Pub/Sub message ID is only a messaging attribute. `coda.run_id` is the
//! attempt this delivery claimed, `coda.work_id` the work it belongs to, and
//! `coda.predecessor_run_id` the attempt it succeeds, all read from the
//! product's own records; nothing here is a second run ledger. Failure
//! signals carry fixed descriptions and a bounded `error.type`, never an
//! error payload, prompt, instruction, or output.
use std::collections::HashMap;

use harness::engine::{Event, EventKind};
use harness::execution::Record;
use local_product::runtime::delivery::Delivery;
use serde_json::Value;
use telemetry::propagation::{self, Relation};
use tracing::Span;
use tracing::field::Empty;

/// A decoded push delivery and the broker metadata it arrived with.
pub(super) struct Push {
    pub delivery: Delivery,
    pub message_id: String,
    /// Publisher attributes, where the W3C trace context travels.
    pub attributes: HashMap<String, String>,
    /// Present when the subscription dead-letters; 1 on the first delivery.
    pub delivery_attempt: Option<u32>,
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
    /// A rehearsal turn's `[fail:K]` marker failed this attempt on purpose.
    Rehearsed,
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
            Self::Rehearsed => "rehearsed",
        }
    }

    /// The fixed, content-free span status description, also the failed
    /// generation's error message.
    pub(super) fn description(self) -> &'static str {
        match self {
            Self::ProviderInit => "model provider unavailable",
            Self::ModelCall => "model execution failed",
            Self::Claim => "attempt claim failed",
            Self::Timeout => "attempt deadline exceeded",
            Self::RetryPersist => "failed attempt not returned to pending",
            Self::Revision => "reply revision failed",
            Self::Rehearsed => "rehearsed attempt failure",
        }
    }
}

/// How the delivery ended for the broker and the product.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Outcome {
    /// The attempt published its result.
    Committed,
    /// Stop, a newer input, or completion already owned the work.
    Settled,
    /// The attempt failed and the delivery was refused for redelivery.
    Retry,
    /// Nothing was claimable for this delivery.
    Absent,
    /// The claim itself failed.
    ClaimError,
}

impl Outcome {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Committed => "committed",
            Self::Settled => "settled",
            Self::Retry => "retry",
            Self::Absent => "absent",
            Self::ClaimError => "claim_error",
        }
    }
}

/// The span's relation to the publisher's context. Only a first delivery
/// continues the publishing trace; a redelivery is a distinct attempt that
/// links to it. A missing counter also links: subscriptions without a
/// dead-letter policy (and some emulators) omit it even on redelivery.
/// Without a valid context the span is a root with no relation.
pub(super) fn relation(remote_valid: bool, delivery_attempt: Option<u32>) -> Relation {
    match (remote_valid, delivery_attempt) {
        (false, _) => Relation::Root,
        (true, Some(1)) => Relation::Child,
        (true, None | Some(_)) => Relation::Link,
    }
}

/// Opens the consumer span for one delivery. It is always a new span with no
/// ambient parent; [`relation`] decides whether it continues or links to the
/// publisher's context.
pub(super) fn span(push: &Push, job: &'static str) -> Span {
    let span = tracing::info_span!(
        parent: None,
        "physician.worker.attempt",
        otel.kind = "consumer",
        messaging.system = "gcp_pubsub",
        messaging.operation.type = "process",
        messaging.message.id = %push.message_id,
        messaging.gcp_pubsub.message.delivery_attempt = Empty,
        coda.job = job,
        coda.delivery_kind = physician_sample::queue::delivery_kind(push.delivery.kind),
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
    if let Some(attempt) = push.delivery_attempt {
        span.record("messaging.gcp_pubsub.message.delivery_attempt", attempt);
    }
    let remote = propagation::remote_context(&push.attributes);
    let applied = propagation::relate(
        &span,
        remote.as_ref(),
        relation(remote.is_some(), push.delivery_attempt),
    );
    span.record("coda.trace_relation", applied.as_str());
    span
}

/// The canonical attempt this delivery claimed.
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

/// Records how the delivery ended. A failure also marks the span `ERROR`
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

    #[test]
    fn attempt_spans_keep_canonical_ids_and_safe_failures_across_redelivery() {
        use opentelemetry::trace::{SpanId, TracerProvider as _};
        use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider};
        use tracing_subscriber::layer::SubscriberExt;
        let exporter = InMemorySpanExporter::default();
        let provider = SdkTracerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();
        let subscriber = tracing_subscriber::registry()
            .with(tracing_opentelemetry::layer().with_tracer(provider.tracer("attempt-test")));
        tracing::subscriber::with_default(subscriber, || {
            let delivery = Delivery::chat_turn(
                local_product::runtime::Scope {
                    practice: "private-practice-marker".into(),
                    physician: "private-physician-marker".into(),
                    workspace: "private-workspace-marker".into(),
                    chat: "private-chat-marker".into(),
                },
                "private-input-marker".into(),
            );
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
                let push = Push {
                    delivery: delivery.clone(),
                    message_id: "same-broker-message".into(),
                    delivery_attempt: Some(number),
                    attributes: [(
                        "traceparent".into(),
                        "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01".into(),
                    )]
                    .into(),
                };
                let span = span(&push, "physician_chat");
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
            // Neither invalid nor missing attributes inherit an ambient HTTP span.
            let ambient = tracing::info_span!("http.server");
            ambient.in_scope(|| {
                for attributes in [
                    HashMap::new(),
                    [("traceparent".into(), "invalid".into())].into(),
                ] {
                    drop(span(
                        &Push {
                            delivery: delivery.clone(),
                            message_id: "unclaimed".into(),
                            delivery_attempt: None,
                            attributes,
                        },
                        "physician_chat",
                    ));
                }
            });
        });
        provider.force_flush().unwrap();
        let spans = exporter.get_finished_spans().unwrap();
        let value = |span: &opentelemetry_sdk::trace::SpanData, key: &str| {
            span.attributes
                .iter()
                .find(|kv| kv.key.as_str() == key)
                .map(|kv| kv.value.to_string())
        };
        let attempts: Vec<_> = spans
            .iter()
            .filter(|span| span.name == "physician.worker.attempt")
            .collect();
        assert_eq!(attempts.len(), 5);
        for (index, run) in [
            "saved-attempt-one",
            "saved-attempt-two",
            "saved-attempt-three",
        ]
        .iter()
        .enumerate()
        {
            let span = attempts[index];
            assert_eq!(value(span, "coda.run_id").as_deref(), Some(*run));
            assert_eq!(
                value(span, "messaging.message.id").as_deref(),
                Some("same-broker-message")
            );
            if index == 0 {
                assert_eq!(
                    span.parent_span_id,
                    SpanId::from_hex("b7ad6b7169203331").unwrap()
                );
            } else {
                assert_eq!(span.parent_span_id, SpanId::INVALID);
                assert_eq!(span.links.len(), 1);
                assert_ne!(
                    span.span_context.trace_id(),
                    attempts[index - 1].span_context.trace_id()
                );
            }
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
            assert!(value(span, "coda.run_id").is_none());
        }
        assert!(!format!("{spans:?}").contains("private-"));
    }

    #[test]
    fn only_a_first_delivery_with_valid_context_continues_the_trace() {
        assert_eq!(relation(true, None), Relation::Link);
        assert_eq!(relation(true, Some(0)), Relation::Link);
        assert_eq!(relation(true, Some(1)), Relation::Child);
        assert_eq!(relation(true, Some(2)), Relation::Link);
        assert_eq!(relation(true, Some(5)), Relation::Link);
        assert_eq!(relation(false, None), Relation::Root);
        assert_eq!(relation(false, Some(1)), Relation::Root);
        assert_eq!(relation(false, Some(3)), Relation::Root);
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
            Failure::Rehearsed,
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
                "revision",
                "rehearsed"
            ]
        );
        let outcomes = [
            Outcome::Committed,
            Outcome::Settled,
            Outcome::Retry,
            Outcome::Absent,
            Outcome::ClaimError,
        ]
        .map(Outcome::as_str);
        assert_eq!(
            outcomes,
            ["committed", "settled", "retry", "absent", "claim_error"]
        );
    }

    fn push(attributes: &[(&str, &str)], delivery_attempt: Option<u32>) -> Push {
        let delivery: Delivery = serde_json::from_value(json!({
            "scope": {"practice": "p", "physician": "d", "workspace": "w", "chat": "c"},
            "input_id": "k"
        }))
        .unwrap();
        Push {
            delivery,
            message_id: "message-1".into(),
            attributes: attributes
                .iter()
                .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
                .collect(),
            delivery_attempt,
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

    const TRACE: &str = "0af7651916cd43dd8448eb211c80319c";
    const PARENT: &str = "b7ad6b7169203331";

    #[test]
    fn a_first_delivery_continues_the_trace_and_a_failure_is_content_free() {
        use opentelemetry::trace::{SpanId, Status, TraceId};
        let traceparent = format!("00-{TRACE}-{PARENT}-01");
        let spans = capture(|| {
            let span = span(
                &push(&[("traceparent", &traceparent)], Some(1)),
                "physician_chat",
            );
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
        assert_eq!(span.span_kind, opentelemetry::trace::SpanKind::Consumer);
        assert_eq!(
            span.span_context.trace_id(),
            TraceId::from_hex(TRACE).unwrap()
        );
        assert_eq!(span.parent_span_id, SpanId::from_hex(PARENT).unwrap());
        assert!(span.links.is_empty());
        for (key, value) in [
            ("coda.job", "physician_chat"),
            ("coda.delivery_kind", "chat_turn"),
            ("messaging.message.id", "message-1"),
            ("messaging.gcp_pubsub.message.delivery_attempt", "1"),
            ("coda.trace_relation", "child"),
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
