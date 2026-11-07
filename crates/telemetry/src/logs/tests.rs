use super::*;
use opentelemetry::{Key, KeyValue, logs::AnyValue, trace::TracerProvider as _};
use opentelemetry_sdk::{
    Resource,
    error::{OTelSdkError, OTelSdkResult},
    logs::{InMemoryLogExporter, LogBatch, LogExporter},
    trace::SdkTracerProvider,
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
    mpsc,
};
use tracing_opentelemetry::OpenTelemetrySpanExt as _;
use tracing_subscriber::prelude::*;

#[test]
fn correlated_safe_logs_respect_explicit_current_and_absent_parents() {
    let exporter = InMemoryLogExporter::default();
    let resource = Resource::builder()
        .with_service_name("worker")
        .with_attributes([
            KeyValue::new("deployment.environment.name", "staging"),
            KeyValue::new("service.version", "immutable-revision"),
        ])
        .build();
    let provider = SdkLoggerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(exporter.clone())
        .build();
    let traces = SdkTracerProvider::builder().build();
    let subscriber = tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(traces.tracer("test")))
        .with(SafeLogs::new(&provider, "worker"));
    let (explicit, ambient) = tracing::subscriber::with_default(subscriber, || {
        let explicit = tracing::info_span!("attempt", prompt = "SECRET-SPAN-CONTENT");
        let expected = explicit.context().span().span_context().clone();
        let ambient = tracing::info_span!("unrelated");
        let other = ambient.context().span().span_context().clone();
        let _entered = ambient.enter();
        tracing::error!(parent: &explicit,
            event = "physician.worker.attempt_failed", error.type = "provider_init",
            coda.job = "physician_chat", coda.outcome = "retry",
            coda.run_id = "12345678-1234-1234-1234-123456789abc",
            error = "SECRET-PROVIDER-ERROR", gen_ai.input.messages = "SECRET-PROMPT",
            "SECRET-MESSAGE");
        tracing::warn!(
            event = "duet.recovery.failed",
            step = "launch",
            error_class = "database",
            "SECRET-DUET"
        );
        // Even an attached unrelated OTel context cannot override parent: None.
        let _otel = ambient.context().attach();
        tracing::info!(parent: None, status = 503_u64, latency_ms = 42_u64, "SECRET-NO-PARENT");
        (expected, other)
    });
    provider.force_flush().unwrap();
    let logs = exporter.get_emitted_logs().unwrap();
    assert_eq!(logs.len(), 3);
    let first = &logs[0].record;
    assert_eq!(first.trace_context().unwrap().trace_id, explicit.trace_id());
    assert_eq!(first.trace_context().unwrap().span_id, explicit.span_id());
    assert_eq!(
        logs[1].record.trace_context().unwrap().span_id,
        ambient.span_id()
    );
    assert_eq!(
        logs[2].record.trace_context().unwrap().trace_id,
        opentelemetry::TraceId::INVALID
    );
    assert_eq!(first.body(), Some(&AnyValue::from(BODY)));
    assert_eq!(first.severity_number(), Some(Severity::Error));
    assert!(
        first
            .attributes_iter()
            .any(|(k, v)| k.as_str() == "error.type" && *v == AnyValue::from("provider_init"))
    );
    assert!(
        first
            .attributes_iter()
            .any(|(k, _)| k.as_str() == "coda.run_id")
    );
    assert_eq!(
        logs[0].resource.get(&Key::from_static_str("service.name")),
        Some("worker".into())
    );
    assert_eq!(
        logs[0]
            .resource
            .get(&Key::from_static_str("deployment.environment.name")),
        Some("staging".into())
    );
    assert_eq!(
        logs[0]
            .resource
            .get(&Key::from_static_str("service.version")),
        Some("immutable-revision".into())
    );
    assert!(!format!("{logs:?}").contains("SECRET"));
    provider.shutdown().unwrap();
    traces.shutdown().unwrap();
}

/// `init` composes layers that may be absent; the log layer must still learn
/// its dispatch, or Loki receives records without trace context.
#[test]
fn init_layer_composition_keeps_trace_context() {
    let exporter = InMemoryLogExporter::default();
    let provider = SdkLoggerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let traces = SdkTracerProvider::builder().build();
    let subscriber = tracing_subscriber::registry()
        .with(Some(
            tracing_opentelemetry::layer().with_tracer(traces.tracer("test")),
        ))
        .with(SafeLogs::for_provider(Some(&provider), "test"))
        .with(None::<SafeLogs>);
    let expected = tracing::subscriber::with_default(subscriber, || {
        let span = tracing::info_span!("http.server");
        let expected = span.context().span().span_context().trace_id();
        span.in_scope(|| tracing::info!(status = 200_u64, "request"));
        expected
    });
    let logs = exporter.get_emitted_logs().unwrap();
    assert_eq!(logs.len(), 1);
    assert_ne!(expected, opentelemetry::TraceId::INVALID);
    assert_eq!(logs[0].record.trace_context().unwrap().trace_id, expected);

    let disabled = tracing_subscriber::registry().with(SafeLogs::for_provider(None, "test"));
    tracing::subscriber::with_default(disabled, || tracing::error!("nothing exported"));
    assert_eq!(exporter.get_emitted_logs().unwrap().len(), 1);
    provider.shutdown().unwrap();
    traces.shutdown().unwrap();
}

#[test]
fn every_http_response_exports_one_record_under_the_default_filter() {
    use tower_http::trace::{MakeSpan as _, OnResponse as _};

    let exporter = InMemoryLogExporter::default();
    let provider = SdkLoggerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let traces = SdkTracerProvider::builder().build();
    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(crate::DEFAULT_FILTER))
        .with(tracing_opentelemetry::layer().with_tracer(traces.tracer("test")))
        .with(SafeLogs::for_provider(Some(&provider), "test"));
    let expected = tracing::subscriber::with_default(subscriber, || {
        let request = http::Request::get("/health").body(()).unwrap();
        let span = crate::HttpSpan.make_span(&request);
        let response = http::Response::builder().status(302).body(()).unwrap();
        crate::RecordStatus.on_response(&response, Duration::from_millis(7), &span);
        span.context().span().span_context().trace_id()
    });
    let logs = exporter.get_emitted_logs().unwrap();
    assert_eq!(logs.len(), 1, "one record per response");
    let record = &logs[0].record;
    assert_eq!(record.trace_context().unwrap().trace_id, expected);
    assert!(
        record
            .attributes_iter()
            .any(|(k, v)| k.as_str() == "status" && *v == AnyValue::from(302_i64))
    );
    assert!(
        record
            .attributes_iter()
            .any(|(k, v)| k.as_str() == "latency_ms" && *v == AnyValue::from(7_i64))
    );
    provider.shutdown().unwrap();
    traces.shutdown().unwrap();
}

#[test]
fn queue_publish_failure_keeps_identifying_fields_without_content() {
    let exporter = InMemoryLogExporter::default();
    let provider = SdkLoggerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let subscriber = tracing_subscriber::registry().with(SafeLogs::new(&provider, "test"));
    tracing::subscriber::with_default(subscriber, || {
        tracing::error!(
            event = "physician.queue.publish_failed",
            error.type = "publish",
            error = "SECRET-PUBLISH-PAYLOAD",
            "SECRET-PUBLISH-MESSAGE"
        );
    });
    let logs = exporter.get_emitted_logs().unwrap();
    assert_eq!(logs.len(), 1);
    for (key, value) in [
        ("event", "physician.queue.publish_failed"),
        ("error.type", "publish"),
    ] {
        assert!(
            logs[0]
                .record
                .attributes_iter()
                .any(|(k, v)| k.as_str() == key && *v == AnyValue::from(value)),
            "missing {key}"
        );
    }
    assert!(!format!("{logs:?}").contains("SECRET"));
    provider.shutdown().unwrap();
}

#[test]
fn recovery_failures_keep_named_safe_classes_and_trace_identity() {
    let exporter = InMemoryLogExporter::default();
    let provider = SdkLoggerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let traces = SdkTracerProvider::builder().build();
    let subscriber = tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(traces.tracer("test")))
        .with(SafeLogs::new(&provider, "physician-sample"));
    let expected = tracing::subscriber::with_default(subscriber, || {
        let span = tracing::info_span!("physician.recovery.sweep");
        let expected = span.context().span().span_context().clone();
        let _entered = span.enter();
        for (event, class) in [
            ("physician.recovery.failed", "database"),
            ("physician.recovery.failed", "publish"),
            ("physician.queue.dispatch_failed", "database"),
            ("physician.receipt_compaction.failed", "database"),
        ] {
            tracing::error!(event, error.type = class,
                error = "SECRET-DATABASE-ERROR", "SECRET-CLINICAL-CONTENT");
        }
        expected
    });
    let logs = exporter.get_emitted_logs().unwrap();
    assert_eq!(logs.len(), 4);
    for (log, (event, class)) in logs.iter().zip([
        ("physician.recovery.failed", "database"),
        ("physician.recovery.failed", "publish"),
        ("physician.queue.dispatch_failed", "database"),
        ("physician.receipt_compaction.failed", "database"),
    ]) {
        assert_eq!(log.record.severity_number(), Some(Severity::Error));
        assert_eq!(
            log.record.trace_context().unwrap().trace_id,
            expected.trace_id()
        );
        assert_eq!(
            log.record.trace_context().unwrap().span_id,
            expected.span_id()
        );
        for (key, value) in [("event", event), ("error.type", class)] {
            assert!(
                log.record
                    .attributes_iter()
                    .any(|(k, v)| k.as_str() == key && *v == AnyValue::from(value)),
                "missing {key}"
            );
        }
    }
    assert!(!format!("{logs:?}").contains("SECRET"));
    provider.shutdown().unwrap();
    traces.shutdown().unwrap();
}

#[test]
fn conversation_failures_keep_safe_steps_and_trace_identity() {
    let exporter = InMemoryLogExporter::default();
    let provider = SdkLoggerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let traces = SdkTracerProvider::builder().build();
    let subscriber = tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(traces.tracer("test")))
        .with(SafeLogs::new(&provider, "duet-api"));
    let expected = tracing::subscriber::with_default(subscriber, || {
        let span = tracing::info_span!("duet.conversation.execute");
        let expected = span.context().span().span_context().clone();
        let _entered = span.enter();
        for step in ["execute", "steered", "claim", "confirm"] {
            tracing::error!(
                event = "duet.conversation.failed",
                step,
                error_class = "execution",
                error = "SECRET-PROVIDER",
                "SECRET-CONTENT"
            );
        }
        expected
    });
    let logs = exporter.get_emitted_logs().unwrap();
    assert_eq!(logs.len(), 4);
    for (log, step) in logs.iter().zip(["execute", "steered", "claim", "confirm"]) {
        assert_eq!(
            log.record.trace_context().unwrap().trace_id,
            expected.trace_id()
        );
        assert_eq!(
            log.record.trace_context().unwrap().span_id,
            expected.span_id()
        );
        for (key, value) in [
            ("event", "duet.conversation.failed"),
            ("step", step),
            ("error_class", "execution"),
        ] {
            assert!(
                log.record
                    .attributes_iter()
                    .any(|(k, v)| k.as_str() == key && *v == AnyValue::from(value)),
                "missing {key}"
            );
        }
    }
    assert!(!format!("{logs:?}").contains("SECRET"));
    provider.shutdown().unwrap();
    traces.shutdown().unwrap();
}

#[test]
fn resource_uses_release_version_or_cloud_run_revision() {
    let resource = crate::resource("telemetry-version-test");
    let expected =
        crate::non_empty_env("DD_VERSION").or_else(|| crate::non_empty_env("K_REVISION"));
    assert_eq!(
        resource.get(&Key::from_static_str("service.version")),
        expected.map(Into::into)
    );
    assert_eq!(
        resource.get(&Key::from_static_str("service.name")),
        Some("telemetry-version-test".into())
    );
}

#[test]
fn rejects_dynamic_values_debug_and_exporter_diagnostics() {
    let exporter = InMemoryLogExporter::default();
    let provider = SdkLoggerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let subscriber = tracing_subscriber::registry().with(SafeLogs::new(&provider, "test"));
    struct MustNotFormat;
    impl fmt::Debug for MustNotFormat {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            panic!("unsafe payload formatted")
        }
    }
    tracing::subscriber::with_default(subscriber, || {
        tracing::error!(event = "SECRET", error.type = "SECRET", coda.run_id = "SECRET",
            coda.job = "SECRET", coda.outcome = "SECRET", step = "SECRET", error_class = "SECRET",
            error = ?MustNotFormat, "SECRET");
        tracing::warn!(target: "opentelemetry_sdk", "export failed");
        tracing::warn!(target: "reqwest", "transport failed");
        let _suppressed = opentelemetry::Context::enter_telemetry_suppressed_scope();
        tracing::warn!("suppressed");
    });
    let logs = exporter.get_emitted_logs().unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].record.attributes_iter().count(), 0);
    assert_eq!(logs[0].record.body(), Some(&AnyValue::from(BODY)));
    provider.shutdown().unwrap();
}

#[derive(Debug)]
struct StalledExporter {
    started: mpsc::SyncSender<()>,
    release: Mutex<mpsc::Receiver<()>>,
    calls: AtomicUsize,
    records: Arc<AtomicUsize>,
}

impl LogExporter for StalledExporter {
    async fn export(&self, batch: LogBatch<'_>) -> OTelSdkResult {
        self.records
            .fetch_add(batch.iter().count(), Ordering::Relaxed);
        if self.calls.fetch_add(1, Ordering::Relaxed) == 0 {
            self.started.send(()).unwrap();
            // A bounded wait ensures even a failing test cannot leak a thread.
            self.release
                .lock()
                .unwrap()
                .recv_timeout(Duration::from_secs(5))
                .unwrap();
        }
        Err(OTelSdkError::InternalFailure("synthetic outage".into()))
    }
}

#[test]
fn stalled_failed_export_drops_overflow_without_blocking_emitters() {
    let (started_tx, started_rx) = mpsc::sync_channel(1);
    let (release_tx, release_rx) = mpsc::sync_channel(1);
    let records = Arc::new(AtomicUsize::new(0));
    let exporter = StalledExporter {
        started: started_tx,
        release: Mutex::new(release_rx),
        calls: AtomicUsize::new(0),
        records: records.clone(),
    };
    let processor = BatchLogProcessor::builder(exporter)
        .with_batch_config(batch_config(8, 1))
        .build();
    let provider = SdkLoggerProvider::builder()
        .with_log_processor(processor)
        .build();
    let subscriber = tracing_subscriber::registry().with(SafeLogs::new(&provider, "test"));
    let dispatch = tracing::Dispatch::new(subscriber);
    tracing::dispatcher::with_default(&dispatch, || tracing::error!("start export"));
    started_rx.recv_timeout(Duration::from_secs(2)).unwrap();
    let (done_tx, done_rx) = mpsc::sync_channel(1);
    let emitter = std::thread::spawn(move || {
        tracing::dispatcher::with_default(&dispatch, || {
            for _ in 0..10_000 {
                tracing::error!(event = "duet.recovery.failed", "outage");
            }
        });
        done_tx.send(()).unwrap();
    });
    let completed = done_rx.recv_timeout(Duration::from_secs(2));
    release_tx.send(()).unwrap();
    emitter.join().unwrap();
    assert!(
        completed.is_ok(),
        "emission blocked behind stalled exporter"
    );
    // Export errors are intentionally best effort; flush waits for draining,
    // not successful delivery to the failed destination.
    let _ = provider.force_flush();
    let exported = records.load(Ordering::Relaxed);
    assert!(
        (2..=9).contains(&exported),
        "queue was not bounded: {exported}"
    );
    provider.shutdown().unwrap();
}
