//! Content-safe OTLP logs. Message/debug/error payloads and span fields never
//! cross this boundary. String fields must match the operational vocabulary
//! below; canonical run IDs must be UUID-shaped. Keep collector allowlists in
//! sync when adding a field. Stdout remains a separate local diagnostic sink.

use std::{fmt, sync::OnceLock, time::Duration};

use opentelemetry::logs::{LogRecord as _, Logger as _, LoggerProvider as _, Severity};
use opentelemetry::trace::TraceContextExt as _;
use opentelemetry_sdk::logs::{
    BatchConfigBuilder, BatchLogProcessor, SdkLogRecord, SdkLogger, SdkLoggerProvider,
};
use tracing::{
    Event, Subscriber,
    field::{Field, Visit},
};
use tracing_subscriber::{Layer, layer::Context, registry::LookupSpan};

const QUEUE_SIZE: usize = 2048;
const BATCH_SIZE: usize = 256;
const EXPORT_TIMEOUT: Duration = Duration::from_secs(2);
const BODY: &str = "operational event";

pub(crate) fn provider(service: &'static str) -> Result<SdkLoggerProvider, String> {
    use opentelemetry_otlp::{Protocol, WithExportConfig};

    let exporter = opentelemetry_otlp::LogExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .with_timeout(EXPORT_TIMEOUT)
        .build()
        .map_err(|error| error.to_string())?;
    let processor = BatchLogProcessor::builder(exporter)
        .with_batch_config(batch_config(QUEUE_SIZE, BATCH_SIZE))
        .build();
    Ok(SdkLoggerProvider::builder()
        .with_resource(super::resource(service))
        .with_log_processor(processor)
        .build())
}

fn batch_config(queue: usize, batch: usize) -> opentelemetry_sdk::logs::BatchConfig {
    BatchConfigBuilder::default()
        .with_max_queue_size(queue)
        .with_max_export_batch_size(batch)
        .with_scheduled_delay(Duration::from_secs(1))
        .build()
}

/// Install it directly, never as `Option<SafeLogs>`: `Option<Layer>` does not
/// forward `on_register_dispatch`, which would leave every record without
/// trace context. [`SafeLogs::for_provider`] covers the no-exporter case.
pub(crate) struct SafeLogs {
    logger: Option<SdkLogger>,
    dispatch: OnceLock<tracing::dispatcher::WeakDispatch>,
}

impl SafeLogs {
    pub(crate) fn new(provider: &SdkLoggerProvider, service: &'static str) -> Self {
        Self {
            logger: Some(provider.logger(service)),
            dispatch: OnceLock::new(),
        }
    }

    /// The layer `init` installs: exporting through `provider`, or nothing.
    pub(crate) fn for_provider(
        provider: Option<&SdkLoggerProvider>,
        service: &'static str,
    ) -> Self {
        match provider {
            Some(provider) => Self::new(provider, service),
            None => Self {
                logger: None,
                dispatch: OnceLock::new(),
            },
        }
    }
}

impl<S> Layer<S> for SafeLogs
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_register_dispatch(&self, dispatch: &tracing::Dispatch) {
        // The ambient dispatch is disabled during subscriber callbacks. Keep a
        // weak handle to our actual subscriber, without a reference cycle.
        let _ = self.dispatch.set(dispatch.downgrade());
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        // SDK diagnostics and the HTTP exporter must never feed back into
        // their own queue, including queue-full diagnostics on request threads.
        let Some(logger) = &self.logger else {
            return;
        };
        let target = event.metadata().target();
        if opentelemetry::Context::is_current_telemetry_suppressed()
            || [
                "opentelemetry",
                "tracing_opentelemetry",
                "reqwest",
                "hyper",
                "h2",
            ]
            .iter()
            .any(|prefix| target.starts_with(prefix))
        {
            return;
        }
        let mut record = logger.create_log_record();
        record.set_body(BODY.into());
        record.set_severity_text(event.metadata().level().as_str());
        record.set_severity_number(match *event.metadata().level() {
            tracing::Level::ERROR => Severity::Error,
            tracing::Level::WARN => Severity::Warn,
            tracing::Level::INFO => Severity::Info,
            tracing::Level::DEBUG => Severity::Debug,
            tracing::Level::TRACE => Severity::Trace,
        });
        // Set even an invalid context: otherwise the SDK may fall back to an
        // unrelated ambient OTel context for explicitly parentless events.
        let context = ctx
            .event_span(event)
            .and_then(|span| {
                let dispatch = self.dispatch.get()?.upgrade()?;
                tracing_opentelemetry::get_otel_context(&span.id(), &dispatch)
            })
            .unwrap_or_default();
        let span = context.span();
        let span_context = span.span_context();
        record.set_trace_context(
            span_context.trace_id(),
            span_context.span_id(),
            Some(span_context.trace_flags()),
        );
        event.record(&mut SafeFields(&mut record));
        logger.emit(record);
    }
}

struct SafeFields<'a>(&'a mut SdkLogRecord);

impl Visit for SafeFields<'_> {
    fn record_str(&mut self, field: &Field, value: &str) {
        let allowed = match field.name() {
            "event" => matches!(
                value,
                "physician.worker.attempt_failed"
                    | "physician.worker.retry_not_persisted"
                    | "physician.queue.publish_failed"
                    | "physician.queue.dispatch_failed"
                    | "physician.recovery.failed"
                    | "physician.receipt_compaction.failed"
                    | "duet.privacy_recovery.failed"
            ),
            "error.type" => matches!(
                value,
                "provider_init"
                    | "model_call"
                    | "claim"
                    | "timeout"
                    | "retry_persist"
                    | "revision"
                    | "rehearsed"
                    | "lost"
                    | "publish"
                    | "database"
            ),
            "coda.job" => matches!(value, "physician_chat" | "physician_reply_revision"),
            "coda.outcome" => matches!(
                value,
                "committed" | "settled" | "retry" | "absent" | "claim_error"
            ),
            "step" => matches!(value, "pending" | "recover"),
            "error_class" => matches!(
                value,
                "database" | "identity" | "conflict" | "not_found" | "unavailable" | "invalid"
            ),
            "coda.run_id" => uuid_shaped(value),
            _ => false,
        };
        if allowed {
            self.0.add_attribute(field.name(), value.to_owned());
        }
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        if matches!(field.name(), "status" | "http.response.status_code")
            && (100..=599).contains(&value)
        {
            self.0.add_attribute(field.name(), value as i64);
        } else if field.name() == "latency_ms"
            && let Ok(value) = i64::try_from(value)
        {
            self.0.add_attribute(field.name(), value);
        }
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        if let Ok(value) = u64::try_from(value) {
            self.record_u64(field, value);
        }
    }

    // Never format arbitrary Display/Debug payloads: even a nominally safe key
    // could contain a provider error or clinical text, and formatting can block.
    fn record_debug(&mut self, _: &Field, _: &dyn fmt::Debug) {}
}

fn uuid_shaped(value: &str) -> bool {
    value.len() == 36
        && value.bytes().enumerate().all(|(index, byte)| {
            if matches!(index, 8 | 13 | 18 | 23) {
                byte == b'-'
            } else {
                byte.is_ascii_hexdigit()
            }
        })
}

#[cfg(test)]
mod tests;
