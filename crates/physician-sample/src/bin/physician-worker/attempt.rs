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
use local_product::runtime::RuntimeError;
use local_product::runtime::delivery::{
    AfterFailure, AttemptSettlement, DeliveryKind, TraceContext,
};
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
    /// The failed attempt could not be returned to pending.
    RetryPersist,
    /// The reply revision's execution failed.
    Revision,
    /// A rehearsal turn's `[fail:K]` marker failed this attempt on purpose.
    Rehearsed,
    /// The run stopped moving and hosting dropped it unsettled.
    Lost,
}

impl Failure {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::ProviderInit => "provider_init",
            Self::ModelCall => "model_call",
            Self::Claim => "claim",
            Self::RetryPersist => "retry_persist",
            Self::Revision => "revision",
            Self::Rehearsed => "rehearsed",
            Self::Lost => "lost",
        }
    }

    /// The fixed, content-free span status description, also the failed
    /// generation's error message.
    pub(super) fn description(self) -> &'static str {
        match self {
            Self::ProviderInit => "model provider unavailable",
            Self::ModelCall => "model execution failed",
            Self::Claim => "attempt claim failed",
            Self::RetryPersist => "failed attempt not returned to pending",
            Self::Revision => "reply revision failed",
            Self::Rehearsed => "rehearsed attempt failure",
            Self::Lost => "run stopped moving and was dropped",
        }
    }
}

/// How the attempt ended for the product.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Outcome {
    /// The attempt published its result.
    Committed,
    /// Stop, a newer input, completion or a hold already owned the work.
    Settled,
    /// The attempt failed and the work is due again after its backoff, or,
    /// when the failure could not be saved, once its lease runs out.
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

/// What an attempt's saved settlement means for the span and the pull loop.
/// A settlement that could not be saved is logged; the attempt's lease then
/// runs out and the next claim records it.
pub(super) fn settled(
    span: &Span,
    job: &'static str,
    run_id: &str,
    saved: &Result<AttemptSettlement, RuntimeError>,
) -> (Outcome, Next) {
    match saved {
        Ok(AttemptSettlement::Failed(AfterFailure::RetryAt(delay))) => {
            (Outcome::Retry, Next::RetryAfter(*delay))
        }
        Ok(AttemptSettlement::Failed(AfterFailure::GaveUp)) => (Outcome::GaveUp, Next::Done),
        Ok(AttemptSettlement::Applied | AttemptSettlement::Superseded) => {
            (Outcome::Settled, Next::Done)
        }
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

/// Records how the attempt ended. A claimed attempt that finished without
/// failure logs `physician.worker.attempt_finished`; a failure marks the
/// span `ERROR` with fixed fields and logs `physician.worker.attempt_failed`.
/// Both log lines carry the trace and span ids, and `coda.run_id` once an
/// attempt was claimed. Nothing recorded here comes from a turn or error
/// payload.
pub(super) fn finish(
    span: &Span,
    job: &'static str,
    run_id: Option<&str>,
    outcome: Outcome,
    failure: Option<Failure>,
) {
    span.record("coda.outcome", outcome.as_str());
    let Some(failure) = failure else {
        if let Some(run_id) = run_id {
            tracing::info!(
                parent: span,
                event = "physician.worker.attempt_finished",
                coda.job = job,
                coda.outcome = outcome.as_str(),
                coda.run_id = run_id,
                "physician worker attempt finished"
            );
        }
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

    /// Usage from a chat attempt's retained provider returns.
    pub(super) fn from_events(events: &[Event]) -> Self {
        let mut tally = Self::default();
        for event in events {
            tally.event(event);
        }
        tally
    }

    /// Usage from one retained event. Only a provider return carries a call's
    /// reported counts.
    pub(super) fn event(&mut self, event: &Event) {
        if let EventKind::ProviderReturned(turn) = &event.kind {
            self.add(turn.usage.input_tokens, turn.usage.output_tokens);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use harness::engine::Turn;
    use sea_orm::DbErr;
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
        assert!(attempts[2].events.iter().any(|event| {
            event.attributes.iter().any(|kv| {
                kv.key.as_str() == "event"
                    && kv.value.to_string() == "physician.worker.attempt_finished"
            })
        }));
        for span in &attempts[3..] {
            assert_eq!(span.parent_span_id, SpanId::INVALID);
            assert!(span.links.is_empty());
            assert!(span.events.is_empty());
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
    fn a_saved_settlement_decides_the_outcome_and_what_the_loop_does_next() {
        let span = Span::none();
        let delay = Duration::from_secs(20);
        for (saved, expected) in [
            (
                Ok(AttemptSettlement::Failed(AfterFailure::RetryAt(delay))),
                (Outcome::Retry, Next::RetryAfter(delay)),
            ),
            (
                Ok(AttemptSettlement::Failed(AfterFailure::GaveUp)),
                (Outcome::GaveUp, Next::Done),
            ),
            (
                Ok(AttemptSettlement::Applied),
                (Outcome::Settled, Next::Done),
            ),
            (
                Ok(AttemptSettlement::Superseded),
                (Outcome::Settled, Next::Done),
            ),
            (
                Err(RuntimeError::Database(DbErr::Custom("unavailable".into()))),
                (Outcome::Retry, Next::Done),
            ),
        ] {
            assert_eq!(settled(&span, "physician_chat", "run", &saved), expected);
        }
    }

    #[test]
    fn failure_types_and_outcomes_are_a_fixed_vocabulary() {
        let failures = [
            Failure::ProviderInit,
            Failure::ModelCall,
            Failure::Claim,
            Failure::RetryPersist,
            Failure::Revision,
            Failure::Rehearsed,
            Failure::Lost,
        ]
        .map(Failure::as_str);
        assert_eq!(
            failures,
            [
                "provider_init",
                "model_call",
                "claim",
                "retry_persist",
                "revision",
                "rehearsed",
                "lost"
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

    fn returned(input_tokens: Option<u64>, output_tokens: Option<u64>) -> Event {
        Event {
            attempt_id: "a".into(),
            sequence: 1,
            ordinal: 1,
            kind: EventKind::ProviderReturned(Turn {
                items: vec![],
                receipt: json!({"id": "r"}),
                usage: harness::engine::recipe::Usage {
                    input_tokens,
                    cached_input_tokens: None,
                    output_tokens,
                },
            }),
        }
    }

    #[test]
    fn usage_is_reported_only_when_every_call_reported_it() {
        let both = [returned(Some(10), Some(2)), returned(Some(5), Some(1))];
        assert_eq!(Tally::from_events(&both).known(), Some((15, 3)));
        // A provider that reports no usage: nothing is invented.
        let unreported = [returned(None, None)];
        assert_eq!(Tally::from_events(&unreported).known(), None);
        let partial = [returned(Some(10), Some(2)), returned(Some(4), None)];
        assert_eq!(Tally::from_events(&partial).known(), None);
        assert_eq!(Tally::from_events(&[]).known(), None);
    }

    mod otlp_logs_proto {
        use opentelemetry_proto::tonic::{
            common::v1::{AnyValue, InstrumentationScope, KeyValue},
            resource::v1::Resource,
        };

        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ExportLogsServiceRequest {
            #[prost(message, repeated, tag = "1")]
            pub resource_logs: Vec<ResourceLogs>,
        }

        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ResourceLogs {
            #[prost(message, optional, tag = "1")]
            pub resource: Option<Resource>,
            #[prost(message, repeated, tag = "2")]
            pub scope_logs: Vec<ScopeLogs>,
            #[prost(string, tag = "3")]
            pub schema_url: String,
        }

        #[derive(Clone, PartialEq, prost::Message)]
        pub struct ScopeLogs {
            #[prost(message, optional, tag = "1")]
            pub scope: Option<InstrumentationScope>,
            #[prost(message, repeated, tag = "2")]
            pub log_records: Vec<LogRecord>,
            #[prost(string, tag = "3")]
            pub schema_url: String,
        }

        #[derive(Clone, PartialEq, prost::Message)]
        pub struct LogRecord {
            #[prost(fixed64, tag = "1")]
            pub time_unix_nano: u64,
            #[prost(int32, tag = "2")]
            pub severity_number: i32,
            #[prost(string, tag = "3")]
            pub severity_text: String,
            #[prost(message, optional, tag = "5")]
            pub body: Option<AnyValue>,
            #[prost(message, repeated, tag = "6")]
            pub attributes: Vec<KeyValue>,
            #[prost(uint32, tag = "7")]
            pub dropped_attributes_count: u32,
            #[prost(fixed32, tag = "8")]
            pub flags: u32,
            #[prost(bytes = "vec", tag = "9")]
            pub trace_id: Vec<u8>,
            #[prost(bytes = "vec", tag = "10")]
            pub span_id: Vec<u8>,
            #[prost(fixed64, tag = "11")]
            pub observed_time_unix_nano: u64,
            #[prost(string, tag = "12")]
            pub event_name: String,
        }
    }

    struct EphemeralPostgres {
        data_dir: Option<std::path::PathBuf>,
        admin_url: String,
    }

    impl EphemeralPostgres {
        fn open_or_start() -> Self {
            if let Some(trimmed) = [
                "PHYSICIAN_RUNTIME_TEST_DATABASE_URL",
                "PHYSICIAN_DATABASE_URL",
                "PHYSICIAN_TEST_DATABASE_URL",
            ]
            .into_iter()
            .find_map(|name| {
                std::env::var(name)
                    .ok()
                    .map(|value| value.trim().to_owned())
                    .filter(|value| !value.is_empty())
            }) {
                let (server, _) = trimmed.rsplit_once('/').expect("database URL");
                return Self {
                    data_dir: None,
                    admin_url: format!("{server}/postgres"),
                };
            }
            let port = {
                let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind free port");
                listener.local_addr().expect("local addr").port()
            };
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("unix clock")
                .as_nanos();
            let data_dir = std::env::temp_dir().join(format!(
                "coda-worker-attempt-pg-{}-{nanos:x}",
                std::process::id()
            ));
            std::fs::create_dir_all(&data_dir).expect("create PGDATA");
            let init = std::process::Command::new("initdb")
                .args([
                    "-U",
                    "postgres",
                    "-A",
                    "trust",
                    "--no-locale",
                    "--encoding=UTF8",
                    "-D",
                ])
                .arg(&data_dir)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::piped())
                .output()
                .expect("run initdb");
            assert!(
                init.status.success(),
                "initdb failed: {}",
                String::from_utf8_lossy(&init.stderr)
            );
            let log_file = data_dir.join("postgres.log");
            let sock_dir = data_dir.to_str().expect("utf8 path");
            let pg_opts = format!("-p {port} -h 127.0.0.1 -k {sock_dir}");
            let start = std::process::Command::new("pg_ctl")
                .arg("-D")
                .arg(&data_dir)
                .arg("-l")
                .arg(&log_file)
                .args(["-o", &pg_opts, "-w", "start"])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::piped())
                .output()
                .expect("run pg_ctl start");
            assert!(
                start.status.success(),
                "pg_ctl start failed: {}",
                String::from_utf8_lossy(&start.stderr)
            );
            Self {
                data_dir: Some(data_dir),
                admin_url: format!("postgresql://postgres@127.0.0.1:{port}/postgres"),
            }
        }
    }

    impl Drop for EphemeralPostgres {
        fn drop(&mut self) {
            if let Some(data_dir) = &self.data_dir {
                if data_dir.join("postmaster.pid").exists() {
                    let _ = std::process::Command::new("pg_ctl")
                        .arg("-D")
                        .arg(data_dir)
                        .args(["-m", "fast", "-w", "stop"])
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .status();
                }
                let _ = std::fs::remove_dir_all(data_dir);
            }
        }
    }

    fn hex_bytes(bytes: &[u8]) -> String {
        use std::fmt::Write as _;
        let mut out = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            let _ = write!(&mut out, "{byte:02x}");
        }
        out
    }

    fn any_string(
        value: &Option<opentelemetry_proto::tonic::common::v1::AnyValue>,
    ) -> Option<String> {
        use opentelemetry_proto::tonic::common::v1::any_value::Value;
        match value.as_ref()?.value.as_ref()? {
            Value::StringValue(text) => Some(text.clone()),
            _ => None,
        }
    }

    fn kv_string(
        attributes: &[opentelemetry_proto::tonic::common::v1::KeyValue],
        key: &str,
    ) -> Option<String> {
        attributes
            .iter()
            .find(|kv| kv.key == key)
            .and_then(|kv| any_string(&kv.value))
    }

    fn run_worker_attempt_log_child() {
        let guard = telemetry::init("physician-worker");
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("worker child runtime");
        rt.block_on(async {
            let worker = std::sync::Arc::new(
                crate::Worker::from_env(|key| std::env::var(key).ok())
                    .await
                    .expect("worker from env"),
            );
            let delivery: local_product::runtime::delivery::Delivery = serde_json::from_str(
                &std::env::var("CODA_DELIVERY_JSON").expect("CODA_DELIVERY_JSON"),
            )
            .expect("decode delivery");
            let (_stop_tx, stop) = physician_auth::watch::channel(false);
            let next = crate::run_delivery(&worker, &delivery, stop.clone()).await;
            assert_eq!(next, Next::Done);

            // Duplicate delivery (`claim_delivery` returns `Ok(None)`) must not
            // emit a second `physician.worker.attempt_finished` record.
            let dup_next = crate::run_delivery(&worker, &delivery, stop).await;
            assert_eq!(dup_next, Next::Done);
        });
        drop(guard);
    }

    #[test]
    fn finished_chat_attempt_exports_traced_content_free_log() {
        if std::env::var_os("CODA_WORKER_ATTEMPT_LOG_CHILD").is_some() {
            run_worker_attempt_log_child();
            return;
        }

        use axum::{Router, body::Bytes, extract::State, routing::post};
        use local_product::physician_chat::PhysicianSkills;
        use local_product::runtime::{Input, Scope, Store};
        use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
        use physician_auth::{AuthStore, Membership, PostgresAuthStore};
        use prost::Message as _;

        const INPUT_CANARY: &str = "canary-turn-input-9f4e2a";
        const OUTPUT_CANARY: &str = "canary-turn-output-7c1b8d";

        let pg = EphemeralPostgres::open_or_start();
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("parent runtime");

        rt.block_on(async {
            let (server, _) = pg.admin_url.rsplit_once('/').expect("admin url");
            let db_name = format!(
                "worker_attempt_log_{}",
                &uuid::Uuid::new_v4().simple().to_string()[..12]
            );
            let admin = sea_orm::Database::connect(&pg.admin_url)
                .await
                .expect("connect admin");
            crate::disposable::replace_database(&admin, &db_name).await;
            admin.close().await.expect("close admin");
            let db_url = format!("{server}/{db_name}");

            let store = Store::connect(&db_url).await.expect("connect store");
            store.migrate().await.expect("migrate store");
            let auth = PostgresAuthStore::connect_with_migrations(&db_url)
                .await
                .expect("migrate auth");

            let unique = uuid::Uuid::new_v4().simple().to_string();
            let practice = format!("practice-{unique}");
            let physician = format!("physician-{unique}");
            let workspace = format!("workspace-{unique}");
            let chat = format!("chat-{unique}");
            let email = format!("worker-{unique}@codahealth.test");
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("unix clock")
                .as_secs();
            auth.ensure_account(&practice, &physician, &email, "Synthetic Physician", false, now)
                .await
                .expect("ensure account");
            let token = auth.begin_sign_in(&email, now).await.expect("begin sign in");
            let (identity, session) = auth
                .complete_sign_in(&email, &token, now)
                .await
                .expect("complete sign in");
            auth.register_patient(&identity.practice_id, &workspace)
                .await
                .expect("register patient");
            auth.set_membership(&identity, &workspace, Membership::Allowed)
                .await
                .expect("set membership");

            let scope = Scope {
                practice: identity.practice_id,
                physician: identity.physician_id,
                workspace,
                chat,
            };
            let skills = PhysicianSkills::configured(None, false).expect("skills");
            let model_env = [
                ("CODA_CHAT_MODEL", "gpt-5.4"),
                ("CODA_CHAT_EFFORT", "medium"),
                ("CODA_CHAT_CONTEXT_WINDOW_TOKENS", "128000"),
                ("CODA_CHAT_INPUT_LIMIT_TOKENS", "64000"),
                ("CODA_BACKGROUND_MODEL", "gpt-5.4"),
                ("CODA_BACKGROUND_EFFORT", "medium"),
                ("CODA_BACKGROUND_CONTEXT_WINDOW_TOKENS", "128000"),
                ("CODA_BACKGROUND_INPUT_LIMIT_TOKENS", "64000"),
            ];
            let roles =
                local_product::model_roles("openai-responses", &model_env).expect("model roles");
            let input_key = format!("input-{unique}");
            store
                .accept_authorized(
                    &scope,
                    &Input {
                        key: input_key.clone(),
                        text: format!("Summarize pre-visit labs [{INPUT_CANARY}]"),
                        timezone: "America/New_York".into(),
                        scenario: Some("langfuse-drill".into()),
                    },
                    PostgresAuthStore::session_reference(&session),
                    &skills,
                    &local_product::physician_chat::recipe(
                        &roles.chat,
                        local_product::physician_chat::INSTRUCTIONS,
                        &skills,
                    ),
                    Some(TraceContext {
                        traceparent: "00-11111111111111111111111111111111-2222222222222222-01"
                            .into(),
                        tracestate: None,
                    }),
                )
                .await
                .expect("accept turn");
            let delivery =
                local_product::runtime::delivery::Delivery::chat_turn(scope.clone(), input_key);

            // Start loopback SSE Responses stub returning the output canary.
            let reply_text = format!("Labs reviewed safely [{OUTPUT_CANARY}]");
            let response = json!({
                "id": "resp-canary-1",
                "status": "completed",
                "model": "gpt-5.4",
                "output": [{
                    "id": "item-0",
                    "type": "message",
                    "role": "assistant",
                    "status": "completed",
                    "content": [{ "type": "output_text", "text": reply_text }]
                }],
                "usage": { "input_tokens": 42, "output_tokens": 17, "total_tokens": 59 }
            });
            let sse_events = [
                json!({ "sequence_number": 0, "type": "response.created", "response": { "id": "resp-canary-1" } }),
                json!({ "sequence_number": 1, "type": "response.output_item.added", "output_index": 0, "item": response["output"][0] }),
                json!({ "sequence_number": 2, "type": "response.content_part.added", "output_index": 0, "item_id": "item-0", "content_index": 0, "part": { "type": "output_text", "text": "" } }),
                json!({ "sequence_number": 3, "type": "response.output_text.delta", "output_index": 0, "item_id": "item-0", "content_index": 0, "delta": reply_text }),
                json!({ "sequence_number": 4, "type": "response.output_text.done", "output_index": 0, "item_id": "item-0", "content_index": 0, "text": reply_text }),
                json!({ "sequence_number": 5, "type": "response.content_part.done", "output_index": 0, "item_id": "item-0", "content_index": 0, "part": response["output"][0]["content"][0] }),
                json!({ "sequence_number": 6, "type": "response.output_item.done", "output_index": 0, "item": response["output"][0] }),
                json!({ "sequence_number": 7, "type": "response.completed", "response": response }),
            ];
            let sse_body: String = sse_events
                .iter()
                .map(|event| format!("data: {event}\n\n"))
                .collect();
            let json_body = response.to_string();
            let stub_listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .expect("bind stub");
            let stub_port = stub_listener.local_addr().expect("stub addr").port();
            let stub_app = Router::new().route(
                "/responses",
                post(move |req_body: Bytes| {
                    let sse = sse_body.clone();
                    let plain = json_body.clone();
                    async move {
                        let stream = serde_json::from_slice::<serde_json::Value>(&req_body)
                            .ok()
                            .and_then(|v| v["stream"].as_bool())
                            .unwrap_or(false);
                        if stream {
                            ([("content-type", "text/event-stream")], sse)
                        } else {
                            ([("content-type", "application/json")], plain)
                        }
                    }
                }),
            );
            let stub_task = tokio::spawn(async move {
                let _ = axum::serve(stub_listener, stub_app).await;
            });

            // Start loopback OTLP/HTTP collector capturing `/v1/traces` and `/v1/logs`.
            #[derive(Clone, Default)]
            struct OtlpSink {
                spans: std::sync::Arc<std::sync::Mutex<Vec<opentelemetry_proto::tonic::trace::v1::Span>>>,
                logs: std::sync::Arc<std::sync::Mutex<Vec<otlp_logs_proto::LogRecord>>>,
                log_bodies: std::sync::Arc<std::sync::Mutex<Vec<Vec<u8>>>>,
            }
            let sink = OtlpSink::default();
            let otel_listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .expect("bind otel sink");
            let otel_port = otel_listener.local_addr().expect("otel addr").port();
            let otel_app = Router::new()
                .route(
                    "/v1/traces",
                    post(|State(sink): State<OtlpSink>, body: Bytes| async move {
                        let req = ExportTraceServiceRequest::decode(body).expect("decode OTLP traces");
                        sink.spans.lock().unwrap().extend(
                            req.resource_spans
                                .into_iter()
                                .flat_map(|r| r.scope_spans)
                                .flat_map(|s| s.spans),
                        );
                        axum::http::StatusCode::OK
                    }),
                )
                .route(
                    "/v1/logs",
                    post(|State(sink): State<OtlpSink>, body: Bytes| async move {
                        let req = otlp_logs_proto::ExportLogsServiceRequest::decode(body.clone())
                            .expect("decode OTLP logs");
                        sink.log_bodies.lock().unwrap().push(body.to_vec());
                        sink.logs.lock().unwrap().extend(
                            req.resource_logs
                                .into_iter()
                                .flat_map(|r| r.scope_logs)
                                .flat_map(|s| s.log_records),
                        );
                        axum::http::StatusCode::OK
                    }),
                )
                .with_state(sink.clone());
            let otel_task = tokio::spawn(async move {
                let _ = axum::serve(otel_listener, otel_app).await;
            });

            let sim_practice = scope.practice.clone();
            let child = tokio::task::spawn_blocking(move || {
                std::process::Command::new(std::env::current_exe().expect("current test binary"))
                    .args([
                        "--exact",
                        "attempt::tests::finished_chat_attempt_exports_traced_content_free_log",
                        "--nocapture",
                    ])
                    .env("CODA_WORKER_ATTEMPT_LOG_CHILD", "1")
                    .envs(model_env)
                    .env("PHYSICIAN_DATABASE_URL", &db_url)
                    .env("PUBSUB_EMULATOR_HOST", "127.0.0.1:9")
                    .env("PHYSICIAN_WORKER_LISTEN_ADDR", "127.0.0.1:0")
                    .env("PHYSICIAN_RESPONSES_STUB_PORT", stub_port.to_string())
                    .env("CODA_HOSTED_PROFILE", "staging-preview")
                    .env("PHYSICIAN_SIMULATION_PRACTICES", &sim_practice)
                    .env("CODA_TELEMETRY_CONTENT", "full")
                    .env(
                        "OTEL_EXPORTER_OTLP_ENDPOINT",
                        format!("http://127.0.0.1:{otel_port}"),
                    )
                    .env("OTEL_BSP_SCHEDULE_DELAY", "50")
                    .env(
                        "CODA_DELIVERY_JSON",
                        serde_json::to_string(&delivery).expect("encode delivery"),
                    )
                    .output()
                    .expect("spawn worker child")
            })
            .await
            .expect("join worker child");

            stub_task.abort();
            otel_task.abort();

            assert!(
                child.status.success(),
                "worker child failed:\nstdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&child.stdout),
                String::from_utf8_lossy(&child.stderr)
            );

            let conversation = store
                .read(&scope)
                .await
                .expect("read conversation")
                .expect("conversation exists");
            let saved_attempt_id = conversation
                .attempts
                .last()
                .expect("saved attempt")
                .id
                .clone();

            let spans = sink.spans.lock().unwrap().clone();
            let attempt_span = spans
                .iter()
                .find(|s| {
                    s.name == "physician.worker.attempt"
                        && kv_string(&s.attributes, "coda.run_id").as_deref()
                            == Some(saved_attempt_id.as_str())
                })
                .expect("exported physician.worker.attempt span");
            let gen_span = spans
                .iter()
                .find(|s| {
                    s.name == "gen_ai.generation"
                        && kv_string(&s.attributes, "coda.run_id").as_deref()
                            == Some(saved_attempt_id.as_str())
                })
                .expect("exported gen_ai.generation span");

            let attempt_trace_hex = hex_bytes(&attempt_span.trace_id);
            let attempt_span_hex = hex_bytes(&attempt_span.span_id);
            assert_ne!(
                attempt_trace_hex, "11111111111111111111111111111111",
                "worker attempt should own a distinct trace linked to the upstream traceparent"
            );
            assert_eq!(hex_bytes(&gen_span.trace_id), attempt_trace_hex);

            // Positive control: `CODA_TELEMETRY_CONTENT=full` exported both canaries on `gen_ai.generation`.
            let gen_input = kv_string(&gen_span.attributes, "langfuse.observation.input")
                .expect("generation input captured under CODA_TELEMETRY_CONTENT=full");
            let gen_output = kv_string(&gen_span.attributes, "langfuse.observation.output")
                .expect("generation output captured under CODA_TELEMETRY_CONTENT=full");
            assert!(gen_input.contains(INPUT_CANARY), "{gen_input}");
            assert!(gen_output.contains(OUTPUT_CANARY), "{gen_output}");

            // Inspect exported OTLP log records and raw `/v1/logs` protobuf payloads.
            let raw_logs = sink.log_bodies.lock().unwrap().clone();
            for payload in &raw_logs {
                let lossy = String::from_utf8_lossy(payload);
                assert!(
                    !lossy.contains(INPUT_CANARY) && !lossy.contains(OUTPUT_CANARY),
                    "OTLP /v1/logs payload leaked canary content: {lossy}"
                );
            }

            let logs = sink.logs.lock().unwrap().clone();
            let finished: Vec<_> = logs
                .iter()
                .filter(|record| {
                    kv_string(&record.attributes, "event").as_deref()
                        == Some("physician.worker.attempt_finished")
                })
                .collect();
            assert_eq!(
                finished.len(),
                1,
                "expected exactly one physician.worker.attempt_finished OTLP log record, got {logs:?}"
            );
            let record = finished[0];
            let mut attrs = serde_json::Map::new();
            for kv in &record.attributes {
                if let Some(val) = any_string(&kv.value) {
                    attrs.insert(kv.key.clone(), json!(val));
                }
            }
            let rendered = json!({
                "severity_text": record.severity_text,
                "body": any_string(&record.body),
                "trace_id": hex_bytes(&record.trace_id),
                "span_id": hex_bytes(&record.span_id),
                "attributes": attrs,
            });
            println!(
                "EXPORTED_OTLP_LOG_RECORD={}",
                serde_json::to_string_pretty(&rendered).unwrap()
            );

            assert_eq!(record.severity_text, "INFO");
            assert_eq!(any_string(&record.body).as_deref(), Some("operational event"));
            assert_eq!(hex_bytes(&record.trace_id), attempt_trace_hex);
            assert_eq!(hex_bytes(&record.span_id), attempt_span_hex);
            assert_eq!(
                kv_string(&record.attributes, "coda.run_id").as_deref(),
                Some(saved_attempt_id.as_str())
            );
            assert_eq!(
                kv_string(&record.attributes, "coda.job").as_deref(),
                Some("physician_chat")
            );
            assert_eq!(
                kv_string(&record.attributes, "coda.outcome").as_deref(),
                Some("committed")
            );
            let rendered_str = rendered.to_string();
            assert!(!rendered_str.contains(INPUT_CANARY));
            assert!(!rendered_str.contains(OUTPUT_CANARY));
        });
    }
}
