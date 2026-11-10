//! Physician worker against the Pub/Sub emulator and a disposable Postgres.
//!
//! The emulator pushes deliveries. A duplicate runs the fixed provider once,
//! a failed run is redelivered and can then succeed, and the fifth failure is
//! forwarded to the dead-letter topic. The worker does not ack that fifth try,
//! and the turn ends as failed so the site's recovery sweep leaves it alone.
//! Concurrent pushes on one worker run side by side.

use std::time::{Duration, Instant};

use local_product::runtime::delivery::Delivery;
use local_product::runtime::{Conversation, Input, Scope, Store};
use physician_auth::{AuthStore, Membership, PostgresAuthStore};
#[path = "scale/mod.rs"]
mod support;

type ExportedSpan = opentelemetry_proto::tonic::trace::v1::Span;

struct TraceCapture {
    endpoint: String,
    spans: std::sync::Arc<std::sync::Mutex<Vec<ExportedSpan>>>,
    task: tokio::task::JoinHandle<()>,
}

impl TraceCapture {
    async fn start() -> Self {
        use axum::{Router, body::Bytes, extract::State, routing::post};
        use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
        use prost::Message as _;
        let spans = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        async fn receive(
            State(spans): State<std::sync::Arc<std::sync::Mutex<Vec<ExportedSpan>>>>,
            body: Bytes,
        ) -> axum::http::StatusCode {
            let request = ExportTraceServiceRequest::decode(body).expect("OTLP protobuf");
            spans.lock().unwrap().extend(
                request
                    .resource_spans
                    .into_iter()
                    .flat_map(|r| r.scope_spans)
                    .flat_map(|s| s.spans),
            );
            axum::http::StatusCode::OK
        }
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let app = Router::new()
            .route("/v1/traces", post(receive))
            .with_state(spans.clone());
        let task = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        Self {
            endpoint,
            spans,
            task,
        }
    }
}

impl Drop for TraceCapture {
    fn drop(&mut self) {
        self.task.abort();
    }
}

fn trace_attribute(span: &ExportedSpan, key: &str) -> Option<String> {
    use opentelemetry_proto::tonic::common::v1::any_value::Value;
    span.attributes
        .iter()
        .find(|kv| kv.key == key)
        .and_then(|kv| kv.value.as_ref())
        .and_then(|value| match &value.value {
            Some(Value::StringValue(text)) => Some(text.clone()),
            _ => None,
        })
}

fn lock() -> &'static tokio::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

struct Running {
    database: support::Database,
    store: Store,
    emulator: support::Emulator,
    worker: support::Process,
    worker_host: String,
    home: std::path::PathBuf,
}

impl Drop for Running {
    fn drop(&mut self) {
        self.worker.kill_now();
        let _ = std::fs::remove_dir_all(&self.home);
    }
}

async fn running(label: &str, failures: Option<u32>, delay_ms: Option<u64>) -> Option<Running> {
    running_with(label, failures, delay_ms, false, &[]).await
}

async fn running_with(
    label: &str,
    failures: Option<u32>,
    delay_ms: Option<u64>,
    fail_marker: bool,
    extra: &[(&str, &str)],
) -> Option<Running> {
    let database = support::Database::create(label).await?;
    let store = Store::connect(&database.url).await.expect("runtime store");
    let worker_port = support::free_port();
    let emulator = support::Emulator::start().await;
    emulator.provision(worker_port).await;
    let home = std::env::temp_dir().join(format!(
        "physician-scale-worker-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4().simple()
    ));
    std::fs::create_dir_all(&home).expect("worker home");
    let listen = format!("127.0.0.1:{worker_port}");
    let failure_count = failures.map(|count| count.to_string());
    let delay = delay_ms.map(|ms| ms.to_string());
    let mut env = vec![
        ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
        ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
        ("PHYSICIAN_WORKER_LISTEN_ADDR", listen.as_str()),
        ("PHYSICIAN_WORKER_FIXED_TEXT", support::FIXED_TEXT),
    ];
    if let Some(count) = failure_count.as_deref() {
        env.push(("PHYSICIAN_WORKER_FIXED_FAILURES", count));
    }
    if let Some(ms) = delay.as_deref() {
        env.push(("PHYSICIAN_WORKER_FIXED_DELAY_MS", ms));
    }
    if fail_marker {
        env.push(("PHYSICIAN_WORKER_FIXED_FAIL_MARKER", "1"));
    }
    env.extend_from_slice(extra);
    let mut worker = support::Process::spawn("worker", support::worker_bin(), &[], &env, &home);
    support::wait_ready(&mut worker, worker_port).await;
    Some(Running {
        database,
        store,
        emulator,
        worker,
        worker_host: listen,
        home,
    })
}

async fn accept_turn(url: &str, store: &Store) -> (Scope, Delivery) {
    accept_text(url, store, "Hello").await
}

async fn accept_text(url: &str, store: &Store, text: &str) -> (Scope, Delivery) {
    accept_in(url, store, text, None, None).await
}

/// Accepts `text` from a new physician in `practice` (a new one when `None`),
/// tagged with `scenario`.
async fn accept_in(
    url: &str,
    store: &Store,
    text: &str,
    practice: Option<&str>,
    scenario: Option<&str>,
) -> (Scope, Delivery) {
    let auth = PostgresAuthStore::connect_without_migrations(url)
        .await
        .expect("auth store");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or_default();
    let unique = uuid::Uuid::new_v4().simple().to_string();
    let email = format!("worker-{unique}@example.test");
    let practice = practice.map_or_else(|| format!("practice-{unique}"), str::to_owned);
    let physician = format!("physician-{unique}");
    let workspace = format!("workspace-{unique}");
    let chat = format!("chat-{unique}");
    auth.ensure_account(
        &practice,
        &physician,
        &email,
        "Synthetic Physician",
        false,
        now,
    )
    .await
    .expect("account");
    let token = auth
        .begin_sign_in(&email, now)
        .await
        .expect("sign-in token");
    let (identity, session) = auth
        .complete_sign_in(&email, &token, now)
        .await
        .expect("session");
    assert_eq!(&identity.practice_id, &practice);
    assert_eq!(&identity.physician_id, &physician);
    auth.register_patient(&identity.practice_id, &workspace)
        .await
        .unwrap();
    auth.set_membership(&identity, &workspace, Membership::Allowed)
        .await
        .unwrap();
    let scope = Scope {
        practice: identity.practice_id,
        physician: identity.physician_id,
        workspace,
        chat,
    };
    let key = format!("input-{unique}");
    store
        .accept_authorized(
            &scope,
            &Input {
                key: key.clone(),
                text: text.into(),
                timezone: "America/New_York".into(),
                scenario: scenario.map(str::to_owned),
            },
            PostgresAuthStore::session_reference(&session),
        )
        .await
        .expect("accept");
    (scope.clone(), Delivery::chat_turn(scope, key))
}

async fn wait_assistant(store: &Store, scope: &Scope, worker: &support::Process) -> Conversation {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        if let Some(state) = store.read(scope).await.expect("read conversation")
            && state
                .messages
                .iter()
                .any(|message| message.role == "assistant")
        {
            return state;
        }
        assert!(
            Instant::now() < deadline,
            "assistant reply did not arrive\n{}",
            worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

fn assistant_texts(state: &Conversation) -> Vec<&str> {
    state
        .messages
        .iter()
        .filter(|message| message.role == "assistant")
        .map(|message| message.text.as_str())
        .collect()
}

#[tokio::test]
async fn duplicate_delivery_runs_the_fixed_provider_once() {
    let _guard = lock().lock().await;
    let Some(running) = running("dup", None, None).await else {
        return;
    };
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;
    running.emulator.publish(&delivery).await;
    let first = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(assistant_texts(&first), [support::FIXED_TEXT]);
    assert_eq!(
        first
            .attempts
            .iter()
            .filter(|attempt| attempt.status == "completed")
            .count(),
        1,
        "first push should complete one attempt"
    );

    running.emulator.publish(&delivery).await;
    let body = support::push_body(&delivery, "duplicate-delivery");
    let (status, text) = support::exchange(
        &running.worker_host,
        "POST",
        "/pubsub/chat-turn",
        &[("content-type", "application/json")],
        Some(&body),
    )
    .await
    .unwrap_or_else(|error| panic!("{error}\n{}", running.worker.tail()));
    assert_eq!(status, 204, "duplicate push should ack: {text}");
    tokio::time::sleep(Duration::from_secs(1)).await;
    let again = running
        .store
        .read(&scope)
        .await
        .expect("reread")
        .expect("conversation");
    assert_eq!(
        assistant_texts(&again),
        [support::FIXED_TEXT],
        "a duplicate delivery must not call the provider again"
    );
    assert_eq!(
        again
            .attempts
            .iter()
            .filter(|attempt| attempt.status == "completed")
            .count(),
        1
    );
}

#[tokio::test]
async fn failed_delivery_is_redelivered_and_can_succeed() {
    let _guard = lock().lock().await;
    let collector = TraceCapture::start().await;
    let Some(running) = running_with(
        "retry",
        Some(1),
        None,
        false,
        &[
            ("CODA_LOG_FORMAT", "json"),
            ("OTEL_EXPORTER_OTLP_ENDPOINT", &collector.endpoint),
            ("OTEL_BSP_SCHEDULE_DELAY", "50"),
        ],
    )
    .await
    else {
        return;
    };
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;
    running
        .emulator
        .publish_with_attributes(
            &delivery,
            [
                (
                    "traceparent".into(),
                    "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01".into(),
                ),
                ("tracestate".into(), "dd=s:1".into()),
            ]
            .into(),
        )
        .await;
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(state.status, "completed");
    assert_eq!(assistant_texts(&state), [support::FIXED_TEXT]);
    assert!(
        state
            .attempts
            .iter()
            .any(|attempt| attempt.status == "failed" || attempt.status == "interrupted"),
        "the first delivery should fail before the redelivery succeeds: {:?}",
        state
            .attempts
            .iter()
            .map(|attempt| attempt.status.as_str())
            .collect::<Vec<_>>()
    );
    assert!(
        state
            .attempts
            .iter()
            .any(|attempt| attempt.status == "completed"),
        "redelivery should complete an attempt"
    );
    assert_eq!(state.attempts.len(), 2);
    let first = &state.attempts[0];
    let second = &state.attempts[1];
    assert_ne!(
        first.id, second.id,
        "redelivery claims a new canonical attempt"
    );
    assert_eq!(first.work, second.work);
    assert_eq!(second.predecessor.as_deref(), Some(first.id.as_str()));
    let logs = running.worker.tail();
    let failure: serde_json::Value = logs
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .find(|event| event["event"] == "physician.worker.attempt_failed")
        .unwrap_or_else(|| panic!("missing asynchronous failure event: {logs}"));
    assert_eq!(failure["coda.run_id"], first.id);
    assert_eq!(failure["error.type"], "model_call");
    let deadline = Instant::now() + Duration::from_secs(10);
    let spans = loop {
        let spans = collector.spans.lock().unwrap().clone();
        if spans
            .iter()
            .filter(|span| span.name == "gen_ai.generation")
            .count()
            == 2
        {
            break spans;
        }
        assert!(
            Instant::now() < deadline,
            "worker traces not exported: {spans:?}\n{logs}"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    };
    for saved in [first, second] {
        let find = |name| {
            spans
                .iter()
                .find(|span| {
                    span.name == name
                        && trace_attribute(span, "coda.run_id").as_deref()
                            == Some(saved.id.as_str())
                })
                .unwrap()
        };
        let attempt = find("physician.worker.attempt");
        let generation = find("gen_ai.generation");
        assert_eq!(generation.trace_id, attempt.trace_id);
        assert_eq!(generation.parent_span_id, attempt.span_id);
        let publisher_trace =
            opentelemetry::trace::TraceId::from_hex("0af7651916cd43dd8448eb211c80319c")
                .unwrap()
                .to_bytes();
        // The emulator omits deliveryAttempt. An unknown delivery count
        // conservatively links, never assumes this is the first execution.
        assert_ne!(attempt.trace_id, publisher_trace, "retry span: {attempt:?}");
        assert_eq!(
            trace_attribute(attempt, "coda.trace_relation").as_deref(),
            Some("link")
        );
        assert_eq!(attempt.links.len(), 1);
        assert_eq!(attempt.links[0].trace_id, publisher_trace);
        assert_eq!(
            trace_attribute(attempt, "coda.work_id").as_deref(),
            Some(saved.work.as_str())
        );
        assert_ne!(
            trace_attribute(attempt, "messaging.message.id").as_deref(),
            Some(saved.id.as_str())
        );
    }
    let failed = spans
        .iter()
        .find(|span| {
            span.name == "physician.worker.attempt"
                && trace_attribute(span, "coda.run_id").as_deref() == Some(first.id.as_str())
        })
        .unwrap();
    assert_eq!(
        failure["dd.trace_id"],
        telemetry::dd_trace_id(opentelemetry::trace::TraceId::from_bytes(
            failed.trace_id.as_slice().try_into().unwrap()
        ))
    );
    assert!(
        spans
            .iter()
            .any(|span| span.name == "POST /pubsub/chat-turn")
    );
}

/// A turn in a rehearsal practice is `simulation` with its scenario on the
/// generation; a turn in any other practice stays `real`, even if its input
/// somehow carries a scenario.
#[tokio::test]
async fn origin_follows_the_practice_of_the_claimed_turn() {
    let _guard = lock().lock().await;
    let collector = TraceCapture::start().await;
    let rehearsal = format!("rehearsal-{}", uuid::Uuid::new_v4().simple());
    let Some(running) = running_with(
        "origin",
        None,
        None,
        false,
        &[
            ("CODA_HOSTED_PROFILE", "staging-preview"),
            ("PHYSICIAN_SIMULATION_PRACTICES", rehearsal.as_str()),
            ("OTEL_EXPORTER_OTLP_ENDPOINT", &collector.endpoint),
            ("OTEL_BSP_SCHEDULE_DELAY", "50"),
        ],
    )
    .await
    else {
        return;
    };
    let url = &running.database.url;
    let (simulated_scope, simulated_delivery) = accept_in(
        url,
        &running.store,
        "Rehearse",
        Some(&rehearsal),
        Some("drill.rehearse"),
    )
    .await;
    let (real_scope, real_delivery) =
        accept_in(url, &running.store, "Hello", None, Some("drill.rehearse")).await;
    running.emulator.publish(&simulated_delivery).await;
    running.emulator.publish(&real_delivery).await;
    let simulated = wait_assistant(&running.store, &simulated_scope, &running.worker).await;
    let real = wait_assistant(&running.store, &real_scope, &running.worker).await;
    let deadline = Instant::now() + Duration::from_secs(10);
    let generation = |spans: &[ExportedSpan], state: &Conversation| {
        let run = state.attempts.last().map(|attempt| attempt.id.clone());
        spans
            .iter()
            .find(|span| {
                span.name == "gen_ai.generation" && trace_attribute(span, "coda.run_id") == run
            })
            .cloned()
    };
    let (simulated_span, real_span) = loop {
        let spans = collector.spans.lock().unwrap().clone();
        if let (Some(a), Some(b)) = (generation(&spans, &simulated), generation(&spans, &real)) {
            break (a, b);
        }
        assert!(
            Instant::now() < deadline,
            "generations not exported\n{}",
            running.worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    };
    let tags = |span: &ExportedSpan| -> Vec<String> {
        use opentelemetry_proto::tonic::common::v1::any_value::Value;
        let Some(Value::ArrayValue(array)) = span
            .attributes
            .iter()
            .find(|kv| kv.key == "langfuse.trace.tags")
            .and_then(|kv| kv.value.as_ref())
            .and_then(|value| value.value.clone())
        else {
            return Vec::new();
        };
        array
            .values
            .into_iter()
            .filter_map(|value| match value.value {
                Some(Value::StringValue(text)) => Some(text),
                _ => None,
            })
            .collect()
    };
    assert_eq!(
        trace_attribute(&simulated_span, "langfuse.environment").as_deref(),
        Some("simulation")
    );
    assert_eq!(
        trace_attribute(&simulated_span, "coda.scenario_id").as_deref(),
        Some("drill.rehearse")
    );
    assert_eq!(
        trace_attribute(&simulated_span, "langfuse.session.id").as_deref(),
        Some(simulated_scope.chat.as_str())
    );
    assert_eq!(
        tags(&simulated_span),
        ["physician_chat", "simulation", "drill.rehearse"]
    );
    assert_eq!(
        trace_attribute(&real_span, "langfuse.environment").as_deref(),
        Some("real")
    );
    assert_eq!(trace_attribute(&real_span, "coda.scenario_id"), None);
    assert_eq!(tags(&real_span), ["physician_chat", "real"]);
}

fn attempt_statuses(state: &Conversation) -> Vec<&str> {
    state
        .attempts
        .iter()
        .map(|attempt| attempt.status.as_str())
        .collect()
}

/// One worker, both kinds of turn: `[fail:1]` fails once and the redelivery
/// answers; a plain turn on the same worker never fails.
#[tokio::test]
async fn fail_marker_fails_only_the_marked_message() {
    let _guard = lock().lock().await;
    let Some(running) = running_with("marker", None, None, true, &[]).await else {
        return;
    };
    let (plain_scope, plain) = accept_turn(&running.database.url, &running.store).await;
    running.emulator.publish(&plain).await;
    let state = wait_assistant(&running.store, &plain_scope, &running.worker).await;
    assert_eq!(state.status, "completed");
    assert_eq!(attempt_statuses(&state), ["completed"]);

    let (marked_scope, marked) =
        accept_text(&running.database.url, &running.store, "Retry [fail:1]").await;
    running.emulator.publish(&marked).await;
    let state = wait_assistant(&running.store, &marked_scope, &running.worker).await;
    assert_eq!(state.status, "completed");
    assert_eq!(assistant_texts(&state), [support::FIXED_TEXT]);
    assert_eq!(attempt_statuses(&state), ["failed", "completed"]);

    let (again_scope, again) = accept_turn(&running.database.url, &running.store).await;
    running.emulator.publish(&again).await;
    let state = wait_assistant(&running.store, &again_scope, &running.worker).await;
    assert_eq!(
        attempt_statuses(&state),
        ["completed"],
        "a plain turn after a marked one must not fail"
    );
}

/// `[fail:K]` counts the conversation's saved attempts, not worker memory: a
/// worker restarted after the first failure fails the second attempt only,
/// and the third answers.
#[tokio::test]
async fn fail_marker_count_survives_a_worker_restart() {
    let _guard = lock().lock().await;
    let Some(mut running) = running_with("marker_restart", None, None, true, &[]).await else {
        return;
    };
    let (scope, marked) =
        accept_text(&running.database.url, &running.store, "Restart [fail:2]").await;
    running.emulator.publish(&marked).await;
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let state = running
            .store
            .read(&scope)
            .await
            .expect("read")
            .expect("row");
        if attempt_statuses(&state).contains(&"failed") {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "the first marked attempt never failed\n{}",
            running.worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    running.worker.restart();
    let port = running
        .worker_host
        .rsplit_once(':')
        .and_then(|(_, port)| port.parse().ok())
        .expect("worker port");
    support::wait_ready(&mut running.worker, port).await;
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(assistant_texts(&state), [support::FIXED_TEXT]);
    assert_eq!(
        attempt_statuses(&state),
        ["failed", "failed", "completed"],
        "a restarted worker must keep counting the saved attempts"
    );
}

#[tokio::test]
async fn fifth_failure_is_forwarded_to_the_dead_letter_topic() {
    let _guard = lock().lock().await;
    let Some(running) = running("dlq", Some(100), None).await else {
        return;
    };
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;
    running.emulator.publish(&delivery).await;

    let deadline = Instant::now() + Duration::from_secs(60);
    let forwarded = loop {
        let messages = running.emulator.pull_dead_letters().await;
        if let Some(bytes) = messages.into_iter().find(|bytes| {
            serde_json::from_slice::<Delivery>(bytes).ok().as_ref() == Some(&delivery)
        }) {
            break bytes;
        }
        assert!(
            Instant::now() < deadline,
            "dead-letter topic did not receive the delivery\n{}\n{}",
            running.worker.tail(),
            running.emulator.tail()
        );
        tokio::time::sleep(Duration::from_millis(200)).await;
    };
    let decoded: Delivery = serde_json::from_slice(&forwarded).expect("dead-letter payload");
    assert_eq!(decoded, delivery);

    let mut state = running
        .store
        .read(&scope)
        .await
        .expect("read")
        .expect("conversation");
    let attempts_deadline = Instant::now() + Duration::from_secs(5);
    while state.attempts.len() < 5 && Instant::now() < attempts_deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
        state = running
            .store
            .read(&scope)
            .await
            .expect("read")
            .expect("conversation");
    }
    assert!(
        state.attempts.len() >= 5,
        "the worker should have failed the delivery five times before the emulator forwarded it, saw {:?}",
        state
            .attempts
            .iter()
            .map(|attempt| attempt.status.as_str())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        state.status, "failed",
        "recovery gives up with the dead letter, so the turn ends as failed"
    );
    assert!(
        assistant_texts(&state).is_empty(),
        "a dead-lettered turn must not publish an assistant reply"
    );
    // The site's recovery sweep reads the same store. It must not republish
    // the turn, which would restart the five-delivery budget forever.
    let start = Scope {
        practice: scope.practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    let swept = running
        .store
        .dispatch_due(100, Some(&start))
        .await
        .expect("recovery sweep");
    assert!(
        !swept.deliveries.contains(&delivery),
        "the recovery sweep republished a turn that recovery gave up on"
    );
}

#[tokio::test]
async fn concurrent_pushes_on_one_worker_run_side_by_side() {
    let _guard = lock().lock().await;
    let delay = Duration::from_secs(2);
    let Some(running) = running("parallel", None, Some(delay.as_millis() as u64)).await else {
        return;
    };
    let (first_scope, first) = accept_turn(&running.database.url, &running.store).await;
    let (second_scope, second) = accept_turn(&running.database.url, &running.store).await;
    let first_body = support::push_body(&first, "parallel-first");
    let second_body = support::push_body(&second, "parallel-second");
    let push = |body: Vec<u8>| {
        let host = running.worker_host.clone();
        async move {
            support::exchange(
                &host,
                "POST",
                "/pubsub/chat-turn",
                &[("content-type", "application/json")],
                Some(&body),
            )
            .await
        }
    };
    let started = Instant::now();
    let (a, b) = tokio::join!(push(first_body), push(second_body));
    let elapsed = started.elapsed();
    for result in [a, b] {
        let (status, text) =
            result.unwrap_or_else(|error| panic!("{error}\n{}", running.worker.tail()));
        assert_eq!(
            status, 204,
            "push should ack after the turn completes: {text}"
        );
    }
    assert!(
        elapsed >= delay,
        "pushes acked after {elapsed:?}, before the {delay:?} turn could finish"
    );
    assert!(
        elapsed < delay * 2 - Duration::from_millis(500),
        "two {delay:?} turns took {elapsed:?}; the worker ran them one after the other"
    );
    for scope in [&first_scope, &second_scope] {
        let state = wait_assistant(&running.store, scope, &running.worker).await;
        assert_eq!(assistant_texts(&state), [support::FIXED_TEXT]);
    }
}

/// Copies the workspace catalog to a directory outside the source checkout,
/// as the packaged worker sees it.
fn copied_skills(home: &std::path::Path) -> std::path::PathBuf {
    fn copy(from: &std::path::Path, to: &std::path::Path) {
        std::fs::create_dir_all(to).expect("skills directory");
        for entry in std::fs::read_dir(from).expect("read skills") {
            let entry = entry.expect("skills entry");
            let target = to.join(entry.file_name());
            if entry.file_type().expect("file type").is_dir() {
                copy(&entry.path(), &target);
            } else {
                std::fs::copy(entry.path(), target).expect("copy skill file");
            }
        }
    }
    let target = home.join("packaged-skills");
    copy(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../skills"),
        &target,
    );
    target
}

fn tool_returned(state: &Conversation) -> Vec<(String, String, bool)> {
    state
        .attempts
        .iter()
        .flat_map(|attempt| &attempt.events)
        .filter_map(|event| match &event.kind {
            harness::engine::EventKind::ToolReturned {
                call_id,
                output,
                executed,
            } => Some((call_id.clone(), output.clone(), *executed)),
            _ => None,
        })
        .collect()
}

/// The real worker binary, its host and Postgres, with a scripted provider:
/// round one reads a skill body next to malformed, unknown, unavailable and
/// over-budget calls; round two reads a supporting resource; then it answers.
/// Each round's results must reach the next request or the fixed provider
/// fails the attempt. Only the final text is published; every exact result
/// stays in the saved attempt.
#[tokio::test]
async fn skill_reads_run_through_the_worker_and_only_the_answer_is_published() {
    let _guard = lock().lock().await;
    let staging = std::env::temp_dir().join(format!(
        "physician-worker-skills-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4().simple()
    ));
    let skills = copied_skills(&staging);
    let skills_text = skills.display().to_string();
    let rounds = serde_json::json!([
        [
            {"name": "skill_view", "arguments": {"name": "pre-visit-preparation", "file_path": null, "max_result_bytes": 65536}},
            {"name": "skill_view", "arguments": {"name": "pre-visit-preparation"}},
            {"name": "skill_view", "arguments": "not json"},
            {"name": "write_patient_memory", "arguments": {}},
            {"name": "skill_view", "arguments": {"name": "not-a-skill", "file_path": null, "max_result_bytes": 4096}},
            {"name": "skill_view", "arguments": {"name": "pre-visit-preparation", "file_path": null, "max_result_bytes": 4096}}
        ],
        [
            {"name": "skill_view", "arguments": {"name": "patient-memory-maintenance", "file_path": "references/authoring-guide.md", "max_result_bytes": 65536}}
        ]
    ])
    .to_string();
    let extra = [
        ("PHYSICIAN_SKILLS_DIR", skills_text.as_str()),
        ("PHYSICIAN_WORKER_FIXED_TOOL_ROUNDS", rounds.as_str()),
    ];
    let Some(running) = running_with("skills", None, None, false, &extra).await else {
        let _ = std::fs::remove_dir_all(&staging);
        return;
    };
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;
    running.emulator.publish(&delivery).await;
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(state.status, "completed");
    assert_eq!(attempt_statuses(&state), ["completed"]);
    assert_eq!(
        assistant_texts(&state),
        [support::FIXED_TEXT],
        "tool-round narration must stay internal"
    );

    // Durable readback through a fresh connection.
    let store = Store::connect(&running.database.url)
        .await
        .expect("fresh store");
    let saved = store
        .read(&scope)
        .await
        .expect("read")
        .expect("conversation");
    let catalog = harness::Harness::load(&skills).expect("copied catalog");
    let requests: Vec<_> = saved.attempts[0]
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            harness::engine::EventKind::ProviderStarted { context, .. } => Some(context),
            _ => None,
        })
        .collect();
    assert_eq!(requests.len(), 3, "two tool rounds and one answer");
    for context in &requests {
        assert!(
            context
                .instructions
                .contains(local_product::physician_chat::INSTRUCTIONS_VERSION)
        );
        assert!(context.instructions.contains(&catalog.catalog()));
        assert_eq!(
            context
                .tools
                .iter()
                .map(|tool| tool.name.as_str())
                .collect::<Vec<_>>(),
            ["skill_view"]
        );
    }
    let returned = tool_returned(&saved);
    assert_eq!(returned.len(), 7);
    let output = |id: &str| {
        returned
            .iter()
            .find(|(call, ..)| call == id)
            .unwrap_or_else(|| panic!("no result for {id}"))
    };
    let exact = |name: &str, path: Option<&str>, bytes: u32| {
        catalog.skill_view(name, path, bytes).unwrap().to_json()
    };
    assert_eq!(
        output("fixed-1-0"),
        &(
            "fixed-1-0".into(),
            exact("pre-visit-preparation", None, 65536),
            true
        )
    );
    assert_eq!(output("fixed-1-4").1, exact("not-a-skill", None, 4096));
    assert_eq!(
        output("fixed-1-5").1,
        exact("pre-visit-preparation", None, 4096)
    );
    assert!(output("fixed-1-5").1.contains("\"over_budget\""));
    assert_eq!(
        output("fixed-2-0").1,
        exact(
            "patient-memory-maintenance",
            Some("references/authoring-guide.md"),
            65536
        )
    );
    for refused in ["fixed-1-1", "fixed-1-2", "fixed-1-3"] {
        let (_, text, executed) = output(refused);
        assert!(!executed, "{refused}");
        assert!(text.contains("\"executed\":false"), "{refused}: {text}");
    }
    // The next request carried each exact result.
    for (call, text, _) in &returned {
        let round: usize = if call.starts_with("fixed-1-") { 1 } else { 2 };
        assert!(requests[round].items.iter().any(|item| matches!(
            item,
            harness::engine::Item::Result { call_id, output } if call_id == call && output == text
        )));
    }
    drop(running);
    let _ = std::fs::remove_dir_all(&staging);
}

/// A worker pointed at a directory without the catalog exits at startup
/// instead of serving with an empty one.
#[tokio::test]
async fn a_worker_without_its_catalog_never_serves() {
    let _guard = lock().lock().await;
    let missing = std::env::temp_dir().join(format!(
        "physician-worker-no-skills-{}",
        uuid::Uuid::new_v4().simple()
    ));
    let missing_text = missing.display().to_string();
    let home = std::env::temp_dir().join(format!(
        "physician-worker-no-skills-home-{}",
        uuid::Uuid::new_v4().simple()
    ));
    std::fs::create_dir_all(&home).expect("home");
    let port = support::free_port().to_string();
    let listen = format!("127.0.0.1:{port}");
    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_WORKER_FIXED_TEXT", support::FIXED_TEXT),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", listen.as_str()),
            ("PUBSUB_EMULATOR_HOST", "127.0.0.1:1"),
            (
                "PHYSICIAN_DATABASE_URL",
                "postgres://unused@127.0.0.1:1/unused",
            ),
            ("PHYSICIAN_SKILLS_DIR", missing_text.as_str()),
        ],
        &home,
    );
    let deadline = Instant::now() + Duration::from_secs(20);
    let code = loop {
        if let Some(code) = worker.exited() {
            break code;
        }
        assert!(
            Instant::now() < deadline,
            "worker kept running\n{}",
            worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    };
    assert_ne!(code, 0);
    let log = worker.tail();
    assert!(log.contains("physician skill catalog at"), "{log}");
    assert!(log.contains(&missing_text), "{log}");
    let _ = std::fs::remove_dir_all(&home);
}
