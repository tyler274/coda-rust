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
//!   `CODA_LOG_FORMAT=json` or an OTLP endpoint is set. JSON lines carry the
//!   W3C lowercase hex `trace_id`/`span_id` that Grafana (Loki and Tempo)
//!   uses for log-to-trace correlation.
//!
//! `CODA_TELEMETRY_BACKEND` names the operations backend: `datadog` (the
//! default) or `grafana`. Under `datadog`, JSON lines also carry
//! `dd.trace_id`, `dd.span_id`, `dd.service`, `dd.env` and `dd.version` for
//! Datadog's log-to-trace correlation. An unknown value prints one warning and
//! falls back to `datadog`. OTLP export is the same for both backends; the
//! collector decides where the data goes.
//!
//! JSON log lines carry the event's own fields and the current span's name,
//! never span fields, so model content recorded on a generation span does not
//! reach log storage.
//!
//! [`http_layer`] traces inbound HTTP requests, and [`genai`] opens model-call
//! spans in Langfuse's OpenTelemetry mapping plus the ENG-74 run attributes.

use std::env;
use std::fmt;
use std::sync::{Arc, OnceLock};
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
use tracing::dispatcher::WeakDispatch;
use tracing::field::{Field, Visit};
use tracing::{Event, Span, Subscriber};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::{FormatTime, SystemTime};
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields, MakeWriter};
use tracing_subscriber::layer::{Layer, SubscriberExt};
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
    let otlp_logs = logs::SafeLogs::for_provider(log_provider.as_ref(), service);
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
    let json_logs = JsonLogs::layer(
        json.then(|| JsonLogs::new(service, backend(), environment(), version())),
        std::io::stdout,
    );
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

/// The operations backend named by `CODA_TELEMETRY_BACKEND`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Backend {
    /// Datadog: JSON lines also carry the `dd.*` correlation keys.
    #[default]
    Datadog,
    /// Grafana (Tempo, Loki, Prometheus): W3C hex ids only.
    Grafana,
}

impl Backend {
    /// Parses a `CODA_TELEMETRY_BACKEND` value. Unset or blank is `datadog`.
    /// An unknown value falls back to `datadog` and returns a warning to print.
    fn parse(value: Option<&str>) -> (Self, Option<String>) {
        let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) else {
            return (Self::Datadog, None);
        };
        if value.eq_ignore_ascii_case("datadog") {
            (Self::Datadog, None)
        } else if value.eq_ignore_ascii_case("grafana") {
            (Self::Grafana, None)
        } else {
            (
                Self::Datadog,
                Some(format!(
                    "telemetry: unknown CODA_TELEMETRY_BACKEND {value:?}; using datadog"
                )),
            )
        }
    }
}

fn backend() -> Backend {
    let value = env::var("CODA_TELEMETRY_BACKEND").ok();
    let (backend, warning) = Backend::parse(value.as_deref());
    if let Some(warning) = warning {
        eprintln!("{warning}");
    }
    backend
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

/// JSON log lines with W3C hex `trace_id`/`span_id`, plus the `dd.*`
/// correlation keys under [`Backend::Datadog`].
struct JsonLogs {
    service: &'static str,
    backend: Backend,
    env: Option<String>,
    version: Option<String>,
    dispatch: DispatchCell,
}

impl JsonLogs {
    fn new(
        service: &'static str,
        backend: Backend,
        env: Option<String>,
        version: Option<String>,
    ) -> Self {
        Self {
            service,
            backend,
            env,
            version,
            dispatch: DispatchCell::default(),
        }
    }

    /// The JSON fmt layer writing to `writer` when `json` is set, plus the
    /// always-installed layer that hands the formatter its subscriber for
    /// trace-context lookup. `Option<Layer>` does not forward
    /// `on_register_dispatch`, so the handle layer sits outside the `Option`.
    fn layer<S, W>(json: Option<Self>, writer: W) -> impl Layer<S> + Send + Sync
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
        W: for<'w> MakeWriter<'w> + Send + Sync + 'static,
    {
        let dispatch = json
            .as_ref()
            .map(|json| json.dispatch.clone())
            .unwrap_or_default();
        let fmt = json.map(|json| {
            tracing_subscriber::fmt::layer()
                .event_format(json)
                .with_writer(writer)
        });
        Layer::and_then(fmt, dispatch)
    }

    fn otel_context(&self, span: &tracing::span::Id) -> Option<opentelemetry::Context> {
        let dispatch = self.dispatch.0.get()?.upgrade()?;
        tracing_opentelemetry::get_otel_context(span, &dispatch)
    }
}

/// A weak handle to the subscriber [`JsonLogs`] is installed in. The ambient
/// dispatch is disabled during subscriber callbacks under a scoped default, so
/// the formatter keeps its own handle, as [`logs::SafeLogs`] does.
#[derive(Clone, Default)]
struct DispatchCell(Arc<OnceLock<WeakDispatch>>);

impl<S: Subscriber> Layer<S> for DispatchCell {
    fn on_register_dispatch(&self, dispatch: &tracing::Dispatch) {
        let _ = self.0.set(dispatch.downgrade());
    }
}

impl<S, N> FormatEvent<S, N> for JsonLogs
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
        let datadog = self.backend == Backend::Datadog;
        if datadog {
            line.insert("dd.service".into(), Value::String(self.service.to_owned()));
            if let Some(env) = &self.env {
                line.insert("dd.env".into(), Value::String(env.clone()));
            }
            if let Some(version) = &self.version {
                line.insert("dd.version".into(), Value::String(version.clone()));
            }
        }
        if let Some(span) = ctx.lookup_current() {
            let name = span.name().to_owned();
            let span_id = span.id();
            drop(span);
            line.insert("span".into(), Value::String(name));
            if let Some(context) = self.otel_context(&span_id) {
                let otel_span = context.span();
                let span_context = otel_span.span_context();
                if span_context.is_valid() {
                    let trace_id = span_context.trace_id();
                    let span_id = span_context.span_id();
                    line.insert("trace_id".into(), Value::String(trace_id.to_string()));
                    line.insert("span_id".into(), Value::String(span_id.to_string()));
                    if datadog {
                        line.insert("dd.trace_id".into(), Value::String(dd_trace_id(trace_id)));
                        line.insert("dd.span_id".into(), Value::String(dd_span_id(span_id)));
                    }
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
/// status 500 or above set `otel.status_code=ERROR`, which Datadog Error
/// Tracking and Tempo span metrics both count as errors. Latency is the span
/// duration.
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

    const TRACE: &str = "0af7651916cd43dd8448eb211c80319c";

    #[derive(Clone, Default)]
    struct Buffer(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);

    impl std::io::Write for Buffer {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for Buffer {
        type Writer = Self;

        fn make_writer(&'a self) -> Self::Writer {
            self.clone()
        }
    }

    /// Formats one event inside a span continuing [`TRACE`] and returns the
    /// JSON line. The layers are composed as `init` composes them, each
    /// optional, under a scoped default (where the ambient dispatch is
    /// disabled during subscriber callbacks).
    fn json_line(backend: Backend) -> Map<String, Value> {
        use opentelemetry::trace::{SpanContext, TraceFlags, TraceState};

        let buffer = Buffer::default();
        let provider = SdkTracerProvider::builder().build();
        let subscriber = tracing_subscriber::registry()
            .with(Some(
                tracing_opentelemetry::layer().with_tracer(provider.tracer("test")),
            ))
            .with(logs::SafeLogs::for_provider(None, "physician-sample"))
            .with(JsonLogs::layer(
                Some(JsonLogs::new(
                    "physician-sample",
                    backend,
                    Some("staging".into()),
                    Some("rev-7".into()),
                )),
                buffer.clone(),
            ));
        tracing::subscriber::with_default(subscriber, || {
            let span = tracing::info_span!("attempt");
            let remote = SpanContext::new(
                TraceId::from_hex(TRACE).unwrap(),
                SpanId::from_hex("b7ad6b7169203331").unwrap(),
                TraceFlags::SAMPLED,
                true,
                TraceState::default(),
            );
            let _ = span.set_parent(opentelemetry::Context::new().with_remote_span_context(remote));
            span.in_scope(|| tracing::info!(work = "w-1", "attempt failed"));
        });
        let bytes = buffer.0.lock().unwrap().clone();
        let text = String::from_utf8(bytes).unwrap();
        let mut lines = text.lines();
        let line = lines.next().expect("one JSON line");
        assert!(lines.next().is_none(), "{text}");
        serde_json::from_str::<Value>(line)
            .unwrap()
            .as_object()
            .unwrap()
            .clone()
    }

    fn assert_common_fields(line: &Map<String, Value>) -> SpanId {
        assert_eq!(line["service"], "physician-sample");
        assert_eq!(line["level"], "INFO");
        assert_eq!(line["target"], module_path!());
        assert_eq!(line["span"], "attempt");
        assert_eq!(line["work"], "w-1");
        assert_eq!(line["message"], "attempt failed");
        assert!(line["timestamp"].is_string());
        assert_eq!(line["trace_id"], TRACE);
        let span_id = line["span_id"].as_str().unwrap();
        assert_eq!(span_id.len(), 16);
        assert!(
            span_id
                .chars()
                .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
        );
        let span_id = SpanId::from_hex(span_id).unwrap();
        assert_ne!(span_id, SpanId::INVALID);
        span_id
    }

    #[test]
    fn datadog_lines_carry_hex_ids_and_the_dd_correlation_keys() {
        let line = json_line(Backend::Datadog);
        let span_id = assert_common_fields(&line);
        assert_eq!(
            line["dd.trace_id"],
            dd_trace_id(TraceId::from_hex(TRACE).unwrap())
        );
        assert_eq!(line["dd.span_id"], dd_span_id(span_id));
        assert_eq!(line["dd.service"], "physician-sample");
        assert_eq!(line["dd.env"], "staging");
        assert_eq!(line["dd.version"], "rev-7");
    }

    #[test]
    fn grafana_lines_carry_hex_ids_and_no_dd_keys() {
        let line = json_line(Backend::Grafana);
        assert_common_fields(&line);
        let dd: Vec<_> = line.keys().filter(|key| key.starts_with("dd.")).collect();
        assert!(dd.is_empty(), "{dd:?}");
    }

    #[test]
    fn lines_outside_a_trace_carry_no_ids() {
        let buffer = Buffer::default();
        let subscriber = tracing_subscriber::registry().with(JsonLogs::layer(
            Some(JsonLogs::new(
                "physician-sample",
                Backend::Datadog,
                None,
                None,
            )),
            buffer.clone(),
        ));
        tracing::subscriber::with_default(subscriber, || tracing::info!("ready"));
        let bytes = buffer.0.lock().unwrap().clone();
        let line: Map<String, Value> = serde_json::from_slice(&bytes).unwrap();
        for key in ["trace_id", "span_id", "dd.trace_id", "dd.span_id"] {
            assert!(!line.contains_key(key), "{key}: {line:?}");
        }
        assert_eq!(line["dd.service"], "physician-sample");
    }

    #[test]
    fn backend_defaults_to_datadog_and_falls_back_on_unknown_values() {
        assert_eq!(Backend::parse(None), (Backend::Datadog, None));
        assert_eq!(Backend::parse(Some("  ")), (Backend::Datadog, None));
        assert_eq!(Backend::parse(Some("datadog")), (Backend::Datadog, None));
        assert_eq!(Backend::parse(Some("Grafana")), (Backend::Grafana, None));
        assert_eq!(Backend::parse(Some(" grafana ")), (Backend::Grafana, None));
        let (backend, warning) = Backend::parse(Some("honeycomb"));
        assert_eq!(backend, Backend::Datadog);
        let warning = warning.expect("unknown value warns");
        assert!(warning.contains("CODA_TELEMETRY_BACKEND"), "{warning}");
        assert!(warning.contains("datadog"), "{warning}");
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
