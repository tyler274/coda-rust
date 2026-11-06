//! W3C trace context over string maps, such as Pub/Sub message attributes.
//!
//! A broker message carries `traceparent` (and `tracestate` when present)
//! among its attributes, never in its payload. The publisher injects the
//! context of its current span; the consumer extracts it and decides with
//! [`Relation`] whether its own span continues that context, links to it, or
//! starts a new trace. The codec is always W3C trace context, whatever global
//! propagator the process installed, so the attribute names stay stable.

use std::collections::HashMap;

use opentelemetry::Context;
use opentelemetry::propagation::TextMapPropagator;
use opentelemetry::trace::{SpanContext, TraceContextExt};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// The W3C attribute naming the remote parent span.
pub const TRACEPARENT: &str = "traceparent";
/// The W3C attribute carrying vendor trace state.
pub const TRACESTATE: &str = "tracestate";

/// Writes the current span's trace context into `carrier`.
///
/// Writes nothing when the current span has no valid OpenTelemetry context,
/// for example when span export is off.
pub fn inject_current(carrier: &mut HashMap<String, String>) {
    inject_span(&Span::current(), carrier);
}

/// Writes `span`'s trace context into `carrier`. See [`inject_current`].
pub fn inject_span(span: &Span, carrier: &mut HashMap<String, String>) {
    // Reusing a carrier must not retain a previous span's parent or state.
    carrier.remove(TRACEPARENT);
    carrier.remove(TRACESTATE);
    let context = span.context();
    TraceContextPropagator::new().inject_context(&context, carrier);
}

/// The valid remote span context in `carrier`, or `None` when it is absent or
/// malformed. A `None` must never be replaced by a fabricated parent.
pub fn remote_context(carrier: &HashMap<String, String>) -> Option<SpanContext> {
    if !carrier
        .get(TRACEPARENT)
        .is_some_and(|value| w3c_shape(value))
    {
        return None;
    }
    let context = TraceContextPropagator::new().extract_with_context(&Context::new(), carrier);
    let remote = context.span().span_context().clone();
    (remote.is_valid() && remote.is_remote()).then_some(remote)
}

/// `version-traceid-parentid-flags` with the exact W3C field widths. The
/// OpenTelemetry parser alone accepts short ids, which W3C forbids.
fn w3c_shape(traceparent: &str) -> bool {
    let fields: Vec<&str> = traceparent.trim().split('-').collect();
    fields.len() >= 4
        && [2, 32, 16, 2].iter().zip(&fields).all(|(width, field)| {
            field.len() == *width
                && field
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        })
}

/// How a consumer span relates to the context its message carried.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Relation {
    /// The span continues the remote trace as its child.
    Child,
    /// The span starts a new trace and links to the remote span.
    Link,
    /// The span starts a new trace with no remote relation.
    Root,
}

impl Relation {
    /// `child`, `link` or `root`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Child => "child",
            Self::Link => "link",
            Self::Root => "root",
        }
    }
}

/// Applies `relation` to `span`, which must not have started yet.
///
/// Returns the relation actually applied: [`Relation::Root`] when `remote`
/// is `None`, whatever was asked. A span with no OpenTelemetry layer is left
/// unchanged.
pub fn relate(span: &Span, remote: Option<&SpanContext>, relation: Relation) -> Relation {
    let Some(remote) = remote.filter(|remote| remote.is_valid()) else {
        return Relation::Root;
    };
    match relation {
        Relation::Child => {
            let _ = span.set_parent(Context::new().with_remote_span_context(remote.clone()));
        }
        Relation::Link => span.add_link(remote.clone()),
        Relation::Root => {}
    }
    relation
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::capture;
    use opentelemetry::trace::{SpanId, TraceId};

    const TRACE: &str = "0af7651916cd43dd8448eb211c80319c";
    const PARENT: &str = "b7ad6b7169203331";

    fn carrier(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
            .collect()
    }

    #[test]
    fn a_valid_traceparent_and_its_tracestate_are_extracted() {
        let remote = remote_context(&carrier(&[
            (TRACEPARENT, &format!("00-{TRACE}-{PARENT}-01")),
            (TRACESTATE, "dd=s:1"),
        ]))
        .expect("valid context");
        assert_eq!(remote.trace_id(), TraceId::from_hex(TRACE).unwrap());
        assert_eq!(remote.span_id(), SpanId::from_hex(PARENT).unwrap());
        assert!(remote.is_remote());
        assert!(remote.is_sampled());
        assert_eq!(remote.trace_state().get("dd"), Some("s:1"));
    }

    #[test]
    fn absent_or_malformed_context_is_none() {
        assert_eq!(remote_context(&HashMap::new()), None);
        for traceparent in [
            "",
            "garbage",
            "00-0AF7651916CD43DD8448EB211C80319C-b7ad6b7169203331-01",
            "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01-extra",
            "00-00000000000000000000000000000000-b7ad6b7169203331-01",
            "00-0af7651916cd43dd8448eb211c80319c-0000000000000000-01",
            "00-0af7651916cd43dd8448eb211c80319c-b7ad6b71692033-01",
            "ff-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01",
        ] {
            assert_eq!(
                remote_context(&carrier(&[(TRACEPARENT, traceparent)])),
                None,
                "{traceparent:?}"
            );
        }
        // Trace state alone names no parent.
        assert_eq!(remote_context(&carrier(&[(TRACESTATE, "dd=s:1")])), None);
        // The OTel parser must not fall back to an ambient remote context
        // when a correctly shaped header has an invalid zero trace id.
        let valid = remote_context(&carrier(&[(
            TRACEPARENT,
            &format!("00-{TRACE}-{PARENT}-01"),
        )]))
        .unwrap();
        let _guard = Context::new().with_remote_span_context(valid).attach();
        assert_eq!(
            remote_context(&carrier(&[(
                TRACEPARENT,
                &format!("00-00000000000000000000000000000000-{PARENT}-01")
            )])),
            None
        );
    }

    #[test]
    fn injection_writes_the_current_span_and_round_trips() {
        let mut injected = HashMap::new();
        let spans = capture(|| {
            let span = tracing::info_span!("publisher");
            relate(
                &span,
                remote_context(&carrier(&[
                    (TRACEPARENT, &format!("00-{TRACE}-{PARENT}-01")),
                    (TRACESTATE, "dd=s:1"),
                ]))
                .as_ref(),
                Relation::Child,
            );
            span.in_scope(|| inject_current(&mut injected));
        });
        let publisher = &spans[0];
        let extracted = remote_context(&injected).expect("round trip");
        assert_eq!(extracted.trace_id(), TraceId::from_hex(TRACE).unwrap());
        assert_eq!(extracted.span_id(), publisher.span_context.span_id());
        assert_eq!(
            injected.get(TRACEPARENT),
            Some(&format!(
                "00-{TRACE}-{}-01",
                publisher.span_context.span_id()
            ))
        );
        assert_eq!(injected.get(TRACESTATE).map(String::as_str), Some("dd=s:1"));
    }

    #[test]
    fn nothing_is_injected_without_a_span_context() {
        let mut injected = carrier(&[(TRACEPARENT, "stale"), (TRACESTATE, "dd=s:1")]);
        inject_span(&Span::none(), &mut injected);
        assert!(injected.is_empty(), "{injected:?}");
    }

    #[test]
    fn child_link_and_root_relations_never_fabricate_a_parent() {
        let remote = remote_context(&carrier(&[(
            TRACEPARENT,
            &format!("00-{TRACE}-{PARENT}-01"),
        )]));
        let mut applied = Vec::new();
        let spans = capture(|| {
            for (name, remote, relation) in [
                ("child", remote.as_ref(), Relation::Child),
                ("link", remote.as_ref(), Relation::Link),
                ("absent", None, Relation::Child),
            ] {
                let span = tracing::info_span!(parent: None, "consumer", otel.name = name);
                applied.push(relate(&span, remote, relation));
            }
        });
        assert_eq!(applied, [Relation::Child, Relation::Link, Relation::Root]);
        let trace = TraceId::from_hex(TRACE).unwrap();
        let find = |name: &str| spans.iter().find(|span| span.name == name).unwrap();

        let child = find("child");
        assert_eq!(child.span_context.trace_id(), trace);
        assert_eq!(child.parent_span_id, SpanId::from_hex(PARENT).unwrap());
        assert!(child.links.is_empty());

        let link = find("link");
        assert_ne!(link.span_context.trace_id(), trace);
        assert_eq!(link.parent_span_id, SpanId::INVALID);
        assert_eq!(link.links.len(), 1);
        assert_eq!(link.links[0].span_context.trace_id(), trace);

        let absent = find("absent");
        assert_ne!(absent.span_context.trace_id(), trace);
        assert_eq!(absent.parent_span_id, SpanId::INVALID);
        assert!(absent.links.is_empty());
    }
}
