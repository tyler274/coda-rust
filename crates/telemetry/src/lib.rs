//! Shared tracing, logging, and OpenTelemetry setup for the Rust services.
//!
//! [`init`] installs one global `tracing` subscriber:
//!
//! - an `EnvFilter` from `RUST_LOG` (default `info`);
//! - an OpenTelemetry layer exporting OTLP/HTTP protobuf spans, only when
//!   `OTEL_EXPORTER_OTLP_ENDPOINT` is set (tests and the sample server need no
//!   infrastructure);
//! - a bounded OTLP log layer when an endpoint is set: fixed body and validated
//!   operational fields only, correlated to the event's tracing parent;
//! - a local log layer: human-readable by default, JSON lines when
//!   `CODA_LOG_FORMAT=json` or an OTLP endpoint is set. JSON lines carry
//!   `dd.trace_id`/`dd.span_id` for Datadog log-to-trace correlation.
//!
//! JSON log lines carry the event's own fields and the current span's name,
//! never span fields, so model content recorded on a generation span does not
//! reach log storage.
//!
//! [`http_layer`] traces inbound HTTP requests, and [`genai`] opens model-call
//! spans in Langfuse's OpenTelemetry mapping plus the ENG-74 run attributes.

use std::env;
use std::fmt;
use std::time::Duration;

use axum::extract::MatchedPath;
use http::{HeaderMap, Request, Response};
use opentelemetry::propagation::Extractor;
use opentelemetry::trace::{SpanId, TraceContextExt, TraceId, TracerProvider as _};
use opentelemetry::{KeyValue, global};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::SdkTracerProvider;
use serde_json::{Map, Value};
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::trace::{MakeSpan, OnResponse, TraceLayer};
use tracing::field::{Field, Visit};
use tracing::{Event, Span, Subscriber};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::{FormatTime, SystemTime};
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;

pub mod genai;
mod logs;
pub mod propagation;

/// Flushes and shuts down the span and log exporters when dropped.
///
/// Hold it for the life of `main`. Without an OTLP endpoint it holds nothing.
#[must_use = "dropping the guard shuts down telemetry export"]
pub struct TelemetryGuard {
    provider: Option<SdkTracerProvider>,
    logs: Option<opentelemetry_sdk::logs::SdkLoggerProvider>,
}

impl TelemetryGuard {
    /// Whether spans are exported over OTLP.
    pub fn exporting(&self) -> bool {
        self.provider.is_some()
    }
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        if let Some(provider) = self.logs.take() {
            // Shutdown drains the bounded queue; no request ever waits for it.
            let _ = provider.shutdown_with_timeout(Duration::from_secs(3));
        }
        if let Some(provider) = self.provider.take() {
            let _ = provider.force_flush();
            let _ = provider.shutdown();
        }
    }
}

/// Installs logging and, when `OTEL_EXPORTER_OTLP_ENDPOINT` is set, span
/// and content-safe operational log export for `service`. Logs use a bounded
/// 2048-record queue, dropping on saturation rather than waiting for export.
///
/// Resource attributes: `service.name` = `service`; `deployment.environment`
/// (and `deployment.environment.name`) from `DD_ENV`, else `CODA_ENV`;
/// `service.version` from `DD_VERSION`, else Cloud Run's immutable `K_REVISION`.
/// The W3C trace-context propagator is
/// installed globally. Safe to call when a global subscriber already exists:
/// the second install is ignored.
pub fn init(service: &'static str) -> TelemetryGuard {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let endpoint = non_empty_env("OTEL_EXPORTER_OTLP_ENDPOINT");
    let json = endpoint.is_some()
        || env::var("CODA_LOG_FORMAT").is_ok_and(|format| format.eq_ignore_ascii_case("json"));
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let log_provider = endpoint
        .as_ref()
        .and_then(|_| match logs::provider(service) {
            Ok(provider) => Some(provider),
            Err(_) => {
                eprintln!("telemetry: OTLP log exporter disabled (configuration error)");
                None
            }
        });
    let otlp_logs = log_provider
        .as_ref()
        .map(|provider| logs::SafeLogs::new(provider, service));
    let provider = match endpoint {
        Some(_) => match tracer_provider(service) {
            Ok(provider) => Some(provider),
            Err(error) => {
                eprintln!("telemetry: OTLP exporter disabled ({error})");
                None
            }
        },
        None => None,
    };
    let otel = provider
        .as_ref()
        .map(|provider| tracing_opentelemetry::layer().with_tracer(provider.tracer(service)));
    let json_logs = json.then(|| {
        tracing_subscriber::fmt::layer().event_format(DatadogJson {
            service,
            env: environment(),
            version: version(),
        })
    });
    let human_logs = (!json).then(|| tracing_subscriber::fmt::layer().with_writer(std::io::stderr));

    let installed = tracing_subscriber::registry()
        .with(filter)
        .with(otel)
        .with(otlp_logs)
        .with(json_logs)
        .with(human_logs)
        .try_init()
        .is_ok();
    if installed && let Some(provider) = &provider {
        global::set_tracer_provider(provider.clone());
    }
    TelemetryGuard {
        provider: if installed { provider } else { None },
        logs: if installed { log_provider } else { None },
    }
}

fn tracer_provider(service: &'static str) -> Result<SdkTracerProvider, String> {
    use opentelemetry_otlp::{Protocol, WithExportConfig};

    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .build()
        .map_err(|error| error.to_string())?;
    Ok(SdkTracerProvider::builder()
        .with_resource(resource(service))
        .with_batch_exporter(exporter)
        .build())
}

/// The OpenTelemetry resource for `service`, from the environment.
pub fn resource(service: &'static str) -> Resource {
    let mut attributes = Vec::new();
    if let Some(environment) = environment() {
        attributes.push(KeyValue::new("deployment.environment", environment.clone()));
        attributes.push(KeyValue::new("deployment.environment.name", environment));
    }
    if let Some(version) = version() {
        attributes.push(KeyValue::new("service.version", version));
    }
    Resource::builder()
        .with_service_name(service)
        .with_attributes(attributes)
        .build()
}

fn version() -> Option<String> {
    non_empty_env("DD_VERSION").or_else(|| non_empty_env("K_REVISION"))
}

fn environment() -> Option<String> {
    non_empty_env("DD_ENV").or_else(|| non_empty_env("CODA_ENV"))
}

fn non_empty_env(key: &str) -> Option<String> {
    env::var(key).ok().filter(|value| !value.trim().is_empty())
}

/// Datadog's 64-bit trace id: the lower half of the OpenTelemetry trace id,
/// in decimal.
pub fn dd_trace_id(trace_id: TraceId) -> String {
    let bytes = trace_id.to_bytes();
    let mut lower = [0_u8; 8];
    lower.copy_from_slice(&bytes[8..]);
    u64::from_be_bytes(lower).to_string()
}

/// Datadog's span id: the OpenTelemetry span id in decimal.
pub fn dd_span_id(span_id: SpanId) -> String {
    u64::from_be_bytes(span_id.to_bytes()).to_string()
}

/// JSON log lines with Datadog correlation ids.
struct DatadogJson {
    service: &'static str,
    env: Option<String>,
    version: Option<String>,
}

impl<S, N> FormatEvent<S, N> for DatadogJson
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let mut line = Map::new();
        let mut timestamp = String::new();
        SystemTime.format_time(&mut Writer::new(&mut timestamp))?;
        line.insert("timestamp".into(), Value::String(timestamp));
        let metadata = event.metadata();
        line.insert("level".into(), Value::String(metadata.level().to_string()));
        line.insert("target".into(), Value::String(metadata.target().to_owned()));
        line.insert("service".into(), Value::String(self.service.to_owned()));
        line.insert("dd.service".into(), Value::String(self.service.to_owned()));
        if let Some(env) = &self.env {
            line.insert("dd.env".into(), Value::String(env.clone()));
        }
        if let Some(version) = &self.version {
            line.insert("dd.version".into(), Value::String(version.clone()));
        }
        if let Some(span) = ctx.lookup_current() {
            let name = span.name().to_owned();
            let span_id = span.id();
            drop(span);
            line.insert("span".into(), Value::String(name));
            let context = tracing::dispatcher::get_default(|dispatch| {
                tracing_opentelemetry::get_otel_context(&span_id, dispatch)
            });
            if let Some(context) = context {
                let otel_span = context.span();
                let span_context = otel_span.span_context();
                let trace_id = span_context.trace_id();
                if trace_id != TraceId::INVALID {
                    line.insert("dd.trace_id".into(), Value::String(dd_trace_id(trace_id)));
                    line.insert(
                        "dd.span_id".into(),
                        Value::String(dd_span_id(span_context.span_id())),
                    );
                }
            }
        }
        let mut fields = Map::new();
        event.record(&mut JsonFields(&mut fields));
        for (key, value) in fields {
            line.entry(key).or_insert(value);
        }
        writeln!(writer, "{}", Value::Object(line))
    }
}

struct JsonFields<'a>(&'a mut Map<String, Value>);

impl Visit for JsonFields<'_> {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.0
            .insert(field.name().to_owned(), Value::String(value.to_owned()));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.0.insert(field.name().to_owned(), Value::Bool(value));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.0.insert(field.name().to_owned(), value.into());
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.0.insert(field.name().to_owned(), value.into());
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        self.0.insert(field.name().to_owned(), value.into());
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.0
            .insert(field.name().to_owned(), Value::String(format!("{value:?}")));
    }
}

/// The tower layer returned by [`http_layer`].
pub type HttpLayer =
    TraceLayer<SharedClassifier<ServerErrorsAsFailures>, HttpSpan, (), RecordStatus, (), (), ()>;

/// Traces each inbound request as an `http.server` span.
///
/// The span continues an inbound W3C `traceparent` and carries
/// `otel.kind=server`, `http.request.method`, `http.route` (the matched axum
/// route, never the raw path), and `http.response.status_code`. Responses with
/// status 500 or above set `otel.status_code=ERROR` for Datadog Error
/// Tracking. Latency is the span duration.
pub fn http_layer() -> HttpLayer {
    TraceLayer::new_for_http()
        .make_span_with(HttpSpan)
        .on_request(())
        .on_response(RecordStatus)
        .on_body_chunk(())
        .on_eos(())
        .on_failure(())
}

/// Builds the `http.server` span. See [`http_layer`].
#[derive(Clone, Copy, Debug, Default)]
pub struct HttpSpan;

impl<B> MakeSpan<B> for HttpSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let method = request.method().as_str();
        let route = request
            .extensions()
            .get::<MatchedPath>()
            .map(|path| path.as_str().to_owned());
        let name = match &route {
            Some(route) => format!("{method} {route}"),
            None => method.to_owned(),
        };
        let span = tracing::info_span!(
            "http.server",
            otel.kind = "server",
            otel.name = %name,
            http.request.method = %method,
            http.route = tracing::field::Empty,
            http.response.status_code = tracing::field::Empty,
            otel.status_code = tracing::field::Empty,
        );
        if let Some(route) = &route {
            span.record("http.route", route.as_str());
        }
        let parent = global::get_text_map_propagator(|propagator| {
            propagator.extract(&HeaderExtractor(request.headers()))
        });
        let _ = span.set_parent(parent);
        span
    }
}

/// Records the response status on the `http.server` span.
#[derive(Clone, Copy, Debug, Default)]
pub struct RecordStatus;

impl<B> OnResponse<B> for RecordStatus {
    fn on_response(self, response: &Response<B>, latency: Duration, span: &Span) {
        let status = response.status().as_u16();
        span.record("http.response.status_code", status);
        if status >= 500 {
            span.record("otel.status_code", "ERROR");
        }
        tracing::debug!(
            parent: span,
            status,
            latency_ms = u64::try_from(latency.as_millis()).unwrap_or(u64::MAX),
            "request finished"
        );
    }
}

struct HeaderExtractor<'a>(&'a HeaderMap);

impl Extractor for HeaderExtractor<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|value| value.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|key| key.as_str()).collect()
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use opentelemetry::trace::TracerProvider as _;
    use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider, SpanData};
    use tracing_subscriber::layer::SubscriberExt;

    /// Runs `body` under an in-memory OTel subscriber and returns the spans.
    pub(crate) fn capture(body: impl FnOnce()) -> Vec<SpanData> {
        let exporter = InMemorySpanExporter::default();
        let provider = SdkTracerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();
        let subscriber = tracing_subscriber::registry()
            .with(tracing_opentelemetry::layer().with_tracer(provider.tracer("test")));
        tracing::subscriber::with_default(subscriber, body);
        provider.force_flush().unwrap();
        exporter.get_finished_spans().unwrap()
    }

    pub(crate) fn attribute(span: &SpanData, key: &str) -> Option<String> {
        span.attributes
            .iter()
            .find(|kv| kv.key.as_str() == key)
            .map(|kv| kv.value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{attribute, capture};

    #[test]
    fn datadog_ids_are_the_lower_64_bits_in_decimal() {
        let trace_id = TraceId::from_hex("0af7651916cd43dd8448eb211c80319c").unwrap();
        assert_eq!(
            dd_trace_id(trace_id),
            u64::from_str_radix("8448eb211c80319c", 16)
                .unwrap()
                .to_string()
        );
        let span_id = SpanId::from_hex("b7ad6b7169203331").unwrap();
        assert_eq!(dd_span_id(span_id), "13235353014750950193");
        assert_eq!(
            dd_trace_id(TraceId::from_hex("00000000000000000000000000000001").unwrap()),
            "1"
        );
    }

    #[test]
    fn http_spans_continue_the_inbound_traceparent_and_mark_server_errors() {
        global::set_text_map_propagator(TraceContextPropagator::new());
        let spans = capture(|| {
            let request = Request::builder()
                .method("POST")
                .uri("/commands")
                .header(
                    "traceparent",
                    "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01",
                )
                .body(())
                .unwrap();
            let span = HttpSpan.make_span(&request);
            let response = Response::builder().status(503).body(()).unwrap();
            RecordStatus.on_response(&response, Duration::from_millis(3), &span);
        });
        let span = spans
            .iter()
            .find(|span| span.name == "POST")
            .expect("http span");
        assert_eq!(
            span.span_context.trace_id(),
            TraceId::from_hex("0af7651916cd43dd8448eb211c80319c").unwrap()
        );
        assert_eq!(
            span.parent_span_id,
            SpanId::from_hex("b7ad6b7169203331").unwrap()
        );
        assert_eq!(span.span_kind, opentelemetry::trace::SpanKind::Server);
        assert_eq!(
            attribute(span, "http.request.method").as_deref(),
            Some("POST")
        );
        assert_eq!(
            attribute(span, "http.response.status_code").as_deref(),
            Some("503")
        );
        assert!(matches!(
            span.status,
            opentelemetry::trace::Status::Error { .. }
        ));
    }
}
