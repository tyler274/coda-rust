//! One physician chat turn produces an `http.server` span and a child
//! `gen_ai.generation` span on the same trace, carrying the ENG-74 run
//! contract, and no prompt text under the default content policy.

use std::time::Duration;

use axum::body::Body;
use axum::http::Request;
use local_product::codex::{CodexExecutor, CodexProcessResult, CodexRequest, CodexTransport};
use opentelemetry::trace::{SpanKind, TraceId, TracerProvider as _};
use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider, SpanData};
use tower::ServiceExt;
use tracing_subscriber::layer::SubscriberExt;

const PROMPT: &str = "Hello, telemetry-prompt-marker";
const REPLY: &str = "Noted, telemetry-reply-marker";
const TRACE: &str = "0af7651916cd43dd8448eb211c80319c";

struct Scripted;

impl CodexExecutor for Scripted {
    fn run(&mut self, _request: &CodexRequest) -> Result<CodexProcessResult, String> {
        Ok(CodexProcessResult::authored(REPLY))
    }
}

fn attribute(span: &SpanData, key: &str) -> Option<String> {
    span.attributes
        .iter()
        .find(|kv| kv.key.as_str() == key)
        .map(|kv| kv.value.to_string())
}

fn queue_context_round_trips(exporter: &InMemorySpanExporter, provider: &SdkTracerProvider) {
    use local_product::runtime::{Scope, delivery::Delivery};
    use telemetry::propagation::{self, Relation, TRACEPARENT};
    let delivery = Delivery::chat_turn(
        Scope {
            practice: "practice".into(),
            physician: "physician".into(),
            workspace: "workspace".into(),
            chat: "chat".into(),
        },
        "accepted-input".into(),
    );
    let publisher = tracing::info_span!(parent: None, "queue-test-publisher");
    let message = publisher.in_scope(|| physician_sample::queue::message(&delivery).unwrap());
    let remote = propagation::remote_context(&message.attributes).expect("publisher context");
    assert_eq!(
        serde_json::from_slice::<Delivery>(&message.data).unwrap(),
        delivery
    );
    assert!(!String::from_utf8_lossy(&message.data).contains(TRACEPARENT));
    drop(publisher);
    for (name, attributes, expected) in [
        ("valid", message.attributes.clone(), Relation::Child),
        ("missing", Default::default(), Relation::Root),
        (
            "invalid",
            [(TRACEPARENT.into(), "not-a-parent".into())].into(),
            Relation::Root,
        ),
    ] {
        let span = tracing::info_span!(parent: None, "queue-test-consumer", otel.name = name);
        assert_eq!(
            propagation::relate(
                &span,
                propagation::remote_context(&attributes).as_ref(),
                Relation::Child
            ),
            expected,
        );
    }
    provider.force_flush().unwrap();
    let spans = exporter.get_finished_spans().unwrap();
    let valid = spans.iter().find(|span| span.name == "valid").unwrap();
    assert_eq!(valid.span_context.trace_id(), remote.trace_id());
    assert_eq!(valid.parent_span_id, remote.span_id());
    for name in ["missing", "invalid"] {
        let span = spans.iter().find(|span| span.name == name).unwrap();
        assert_eq!(span.parent_span_id, opentelemetry::trace::SpanId::INVALID);
        assert_ne!(span.span_context.trace_id(), remote.trace_id());
        assert!(span.links.is_empty());
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_chat_turn_is_one_trace_with_the_run_contract_and_no_content() {
    // SAFETY: this test binary has one test; nothing else reads the env.
    unsafe {
        std::env::remove_var("CODA_TELEMETRY_CONTENT");
        std::env::remove_var("CODA_LOCAL_SIMULATION");
        std::env::set_var("CODA_RECIPE_VERSION", "recipe-sha-test");
    }
    opentelemetry::global::set_text_map_propagator(
        opentelemetry_sdk::propagation::TraceContextPropagator::new(),
    );
    let exporter = InMemorySpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let subscriber = tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(provider.tracer("test")));
    tracing::subscriber::set_global_default(subscriber).expect("one subscriber");
    queue_context_round_trips(&exporter, &provider);
    exporter.reset();

    let dir = std::env::temp_dir().join(format!("coda-telemetry-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    // The physician session is durable; a fresh file keeps this turn new on
    // every run instead of replaying an idempotency key from the user's state.
    // SAFETY: single-test binary, set before the router reads it.
    unsafe {
        std::env::set_var("CODA_PHYSICIAN_SESSION_PATH", dir.join("session.json"));
    }
    let transport = CodexTransport::recording(
        Box::new(Scripted),
        "synthetic telemetry test",
        dir.join("cassette.json"),
    )
    .unwrap();
    let app = physician_sample::codex_chat_router(transport);

    let submit = format!(
        r#"{{"type":"message.submit","idempotency_key":"k-telemetry","payload":{{"patient_workspace_id":"pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e","chat_id":"chat-telemetry","text":"{PROMPT}","physician_time_zone":"America/New_York"}}}}"#
    );
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/commands")
                .header("content-type", "application/json")
                .header("traceparent", format!("00-{TRACE}-b7ad6b7169203331-01"))
                .body(Body::from(submit))
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(response.status().is_success(), "{}", response.status());
    // The `http.server` span ends with the response body.
    let body = axum::body::to_bytes(response.into_body(), 4_000)
        .await
        .unwrap();
    assert!(String::from_utf8_lossy(&body).contains("accepted"));

    let mut spans = Vec::new();
    for _ in 0..200 {
        provider.force_flush().unwrap();
        spans = exporter.get_finished_spans().unwrap();
        if spans.iter().any(|span| span.name == "gen_ai.generation")
            && spans.iter().any(|span| span.span_kind == SpanKind::Server)
        {
            break;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    let names: Vec<_> = spans.iter().map(|span| span.name.to_string()).collect();
    let server = spans
        .iter()
        .find(|span| span.span_kind == SpanKind::Server)
        .unwrap_or_else(|| panic!("no server span in {names:?}"));
    let generation = spans
        .iter()
        .find(|span| span.name == "gen_ai.generation")
        .unwrap_or_else(|| panic!("no generation span in {names:?}"));

    assert_eq!(server.name, "POST /commands");
    assert_eq!(
        attribute(server, "http.route").as_deref(),
        Some("/commands")
    );
    assert_eq!(
        attribute(server, "http.response.status_code").as_deref(),
        Some("200")
    );
    let trace = TraceId::from_hex(TRACE).unwrap();
    assert_eq!(server.span_context.trace_id(), trace);
    assert_eq!(generation.span_context.trace_id(), trace);
    assert_eq!(generation.parent_span_id, server.span_context.span_id());

    for (key, value) in [
        ("langfuse.observation.type", "generation"),
        ("gen_ai.system", "codex"),
        ("coda.adapter", "codex"),
        ("coda.run_id", "physician-chat-1"),
        ("coda.job", "physician_chat"),
        ("coda.recipe_version", "recipe-sha-test"),
        ("coda.origin", "real"),
        ("langfuse.environment", "real"),
        ("langfuse.session.id", "chat-telemetry"),
    ] {
        assert_eq!(attribute(generation, key).as_deref(), Some(value), "{key}");
    }
    assert!(attribute(generation, "gen_ai.request.model").is_some());

    for span in &spans {
        let dump = format!("{:?} {:?}", span.attributes, span.events);
        assert!(
            !dump.contains("telemetry-prompt-marker"),
            "{}: {dump}",
            span.name
        );
        assert!(
            !dump.contains("telemetry-reply-marker"),
            "{}: {dump}",
            span.name
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}
