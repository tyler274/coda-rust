//! The request that accepts a turn is linked from the attempt that runs it.
//!
//! A site and a worker share a disposable Postgres and export their spans to
//! one loopback collector. The site saves its request's trace context with
//! the accepted turn; the worker's `physician.worker.attempt` is a trace of
//! its own that links to that request. Nothing carries the context between
//! the two processes but the conversation row.

use std::time::{Duration, Instant};

#[path = "scale/mod.rs"]
mod support;
use support::trace_attribute;

#[tokio::test]
async fn the_worker_attempt_links_to_the_request_that_accepted_the_turn() {
    let Some(database) = support::Database::create("queue_trace").await else {
        return;
    };
    support::staging_account(&database.url).await;
    let home = std::env::temp_dir().join(format!(
        "physician-queue-trace-{}",
        uuid::Uuid::new_v4().simple()
    ));
    std::fs::create_dir_all(&home).expect("home");
    let collector = support::TraceCapture::start().await;
    let telemetry = [
        ("OTEL_EXPORTER_OTLP_ENDPOINT", collector.endpoint.as_str()),
        ("OTEL_BSP_SCHEDULE_DELAY", "50"),
    ];
    let worker_port = support::free_port();
    let worker_listen = format!("127.0.0.1:{worker_port}");
    let stub = support::ResponsesStub::start(&home, 0, 0, "[]").await;
    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", worker_listen.as_str()),
            ("PHYSICIAN_RESPONSES_STUB_PORT", stub.port.as_str()),
            telemetry[0],
            telemetry[1],
        ],
        &home,
    );
    support::wait_ready(&mut worker, worker_port).await;
    let site_port = support::free_port();
    let site_listen = format!("127.0.0.1:{site_port}");
    let mut site = support::Process::spawn(
        "site",
        support::site_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            support::DURABLE,
            ("PHYSICIAN_AUTH", "required"),
            ("PHYSICIAN_LISTEN_ADDR", site_listen.as_str()),
            telemetry[0],
            telemetry[1],
        ],
        &home,
    );
    support::wait_ready(&mut site, site_port).await;

    let session = support::sign_in(&site_listen).await;
    let workspace = physician_sample::WORKSPACE_IDS[0];
    let chat = format!("chat-{}", uuid::Uuid::new_v4().simple());
    // The page's own trace context, as a browser or load balancer sends it.
    let page_trace = "0af7651916cd43dd8448eb211c80319c";
    let body = serde_json::to_vec(&support::submit(workspace, &chat, "Hello")).unwrap();
    let authorization = format!("Bearer {session}");
    let traceparent = format!("00-{page_trace}-b7ad6b7169203331-01");
    let (status, text) = support::exchange(
        &site_listen,
        "POST",
        "/commands",
        &[
            ("content-type", "application/json"),
            ("authorization", authorization.as_str()),
            ("traceparent", traceparent.as_str()),
        ],
        Some(&body),
    )
    .await
    .unwrap_or_else(|error| panic!("{error}\n{}", site.tail()));
    assert_eq!(status, 200, "{text}\n{}", site.tail());
    assert!(text.contains("\"accepted\""), "{text}");

    let deadline = Instant::now() + Duration::from_secs(30);
    let (attempt, spans) = loop {
        let spans = collector.spans.lock().unwrap().clone();
        let attempt = spans
            .iter()
            .find(|span| {
                span.name == "physician.worker.attempt"
                    && trace_attribute(span, "coda.outcome").as_deref() == Some("committed")
            })
            .cloned();
        if let Some(attempt) = attempt {
            break (attempt, spans);
        }
        assert!(
            Instant::now() < deadline,
            "no committed worker attempt was exported\n{}\n{}",
            site.tail(),
            worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    };

    // The accepting request: the site's server span for this command, which
    // continues the page's trace.
    let page_trace = opentelemetry::trace::TraceId::from_hex(page_trace)
        .unwrap()
        .to_bytes();
    let requests: Vec<_> = spans
        .iter()
        .filter(|span| span.trace_id == page_trace && span.name.contains("/commands"))
        .collect();
    assert_eq!(
        requests.len(),
        1,
        "one accepting request span: {:?}",
        spans.iter().map(|span| &span.name).collect::<Vec<_>>()
    );
    let request = requests[0];

    assert_eq!(attempt.links.len(), 1, "{attempt:?}");
    let link = &attempt.links[0];
    assert_eq!(link.trace_id, request.trace_id);
    assert!(
        spans
            .iter()
            .any(|span| span.trace_id == link.trace_id && span.span_id == link.span_id),
        "the link names no exported span of the accepting request: {link:?}"
    );
    // The attempt is its own trace: it ran later, and may run again.
    assert_ne!(attempt.trace_id, request.trace_id);
    assert!(attempt.parent_span_id.is_empty(), "{attempt:?}");
    for (key, value) in [
        ("coda.trace_relation", "link"),
        ("messaging.system", "postgresql"),
        ("coda.delivery_kind", "chat_turn"),
        ("coda.job", "physician_chat"),
    ] {
        assert_eq!(
            trace_attribute(&attempt, key).as_deref(),
            Some(value),
            "{key}"
        );
    }
    // The model call belongs to the attempt's trace.
    assert!(spans.iter().any(|span| {
        span.name == "gen_ai.generation"
            && span.trace_id == attempt.trace_id
            && span.parent_span_id == attempt.span_id
    }));
    // Trace context is identity only: no span carries the message.
    assert!(!format!("{spans:?}").contains("Hello"));

    worker.kill_now();
    site.kill_now();
    let _ = std::fs::remove_dir_all(&home);
}
