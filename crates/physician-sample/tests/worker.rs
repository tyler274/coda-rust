//! Physician worker against the Pub/Sub emulator and a disposable Postgres.
//!
//! The emulator pushes deliveries. A duplicate runs the fixed provider once,
//! a failed run is redelivered and can then succeed, a request the provider
//! rejects holds the turn and is not sent again, and the fifth failure is
//! forwarded to the dead-letter topic. The worker does not ack that fifth try,
//! and the turn ends as failed so the site's recovery sweep leaves it alone.
//! Concurrent pushes on one worker run side by side.

use std::time::{Duration, Instant};

use local_product::physician_chat::PhysicianSkills;
use local_product::runtime::delivery::Delivery;
use local_product::runtime::{AttemptStatus, Conversation, Input, Scope, Store, WorkStatus};
use physician_auth::{AuthStore, Membership, PostgresAuthStore};
#[path = "scale/mod.rs"]
mod support;

type ExportedSpan = opentelemetry_proto::tonic::trace::v1::Span;

/// A loopback OTLP/HTTP receiver: the decoded spans, and every request body
/// as received (traces and logs), so a test can search all of it.
struct TraceCapture {
    endpoint: String,
    port: u16,
    spans: std::sync::Arc<std::sync::Mutex<Vec<ExportedSpan>>>,
    bodies: std::sync::Arc<std::sync::Mutex<Vec<Vec<u8>>>>,
    task: tokio::task::JoinHandle<()>,
}

#[derive(Clone)]
struct Captured {
    spans: std::sync::Arc<std::sync::Mutex<Vec<ExportedSpan>>>,
    bodies: std::sync::Arc<std::sync::Mutex<Vec<Vec<u8>>>>,
}

impl TraceCapture {
    async fn start() -> Self {
        use axum::{Router, body::Bytes, extract::State, routing::post};
        use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
        use prost::Message as _;
        async fn traces(State(captured): State<Captured>, body: Bytes) -> axum::http::StatusCode {
            let request = ExportTraceServiceRequest::decode(body.clone()).expect("OTLP protobuf");
            captured.bodies.lock().unwrap().push(body.to_vec());
            captured.spans.lock().unwrap().extend(
                request
                    .resource_spans
                    .into_iter()
                    .flat_map(|r| r.scope_spans)
                    .flat_map(|s| s.spans),
            );
            axum::http::StatusCode::OK
        }
        async fn logs(State(captured): State<Captured>, body: Bytes) -> axum::http::StatusCode {
            captured.bodies.lock().unwrap().push(body.to_vec());
            axum::http::StatusCode::OK
        }
        let captured = Captured {
            spans: Default::default(),
            bodies: Default::default(),
        };
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let app = Router::new()
            .route("/v1/traces", post(traces))
            .route("/v1/logs", post(logs))
            .with_state(captured.clone());
        let task = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        Self {
            endpoint: format!("http://127.0.0.1:{port}"),
            port,
            spans: captured.spans,
            bodies: captured.bodies,
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
    stub: support::ResponsesStub,
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
    running_with(label, failures, delay_ms, "[]", support::FIXED_TEXT, &[]).await
}

async fn running_with(
    label: &str,
    failures: Option<u32>,
    delay_ms: Option<u64>,
    rounds: &str,
    reply: &str,
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
    let stub = support::ResponsesStub::start_with_reply(
        &home,
        failures.unwrap_or(0),
        delay_ms.unwrap_or(0),
        rounds,
        reply,
    )
    .await;
    let mut env = vec![
        ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
        ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
        ("PHYSICIAN_WORKER_LISTEN_ADDR", listen.as_str()),
        ("PHYSICIAN_RESPONSES_STUB_PORT", stub.port.as_str()),
    ];
    env.extend_from_slice(extra);
    let mut worker = support::Process::spawn("worker", support::worker_bin(), &[], &env, &home);
    support::wait_ready(&mut worker, worker_port).await;
    Some(Running {
        stub,
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

/// [`accept_text`], also returning the physician's session.
async fn accept_signed_in(url: &str, store: &Store, text: &str) -> (Scope, Delivery, String) {
    accept_signed_in_with(
        url,
        store,
        text,
        &PhysicianSkills::configured(None, false).unwrap(),
        None,
        None,
    )
    .await
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
    accept_with_skills(
        url,
        store,
        text,
        &PhysicianSkills::configured(None, false).unwrap(),
        practice,
        scenario,
    )
    .await
}

async fn accept_with_skills(
    url: &str,
    store: &Store,
    text: &str,
    skills: &PhysicianSkills,
    practice: Option<&str>,
    scenario: Option<&str>,
) -> (Scope, Delivery) {
    let (scope, delivery, _) =
        accept_signed_in_with(url, store, text, skills, practice, scenario).await;
    (scope, delivery)
}

async fn accept_signed_in_with(
    url: &str,
    store: &Store,
    text: &str,
    skills: &PhysicianSkills,
    practice: Option<&str>,
    scenario: Option<&str>,
) -> (Scope, Delivery, String) {
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
            skills,
            &local_product::physician_chat::recipe(
                &support::models::roles().chat,
                local_product::physician_chat::INSTRUCTIONS,
                skills,
            ),
        )
        .await
        .expect("accept");
    (scope.clone(), Delivery::chat_turn(scope, key), session)
}

async fn wait_assistant(store: &Store, scope: &Scope, worker: &support::Process) -> Conversation {
    wait_assistant_for(store, scope, worker, Duration::from_secs(30)).await
}

async fn wait_assistant_for(
    store: &Store,
    scope: &Scope,
    worker: &support::Process,
    wait: Duration,
) -> Conversation {
    let deadline = Instant::now() + wait;
    loop {
        if let Some(state) = store.read(scope).await.expect("read conversation")
            && state
                .messages
                .iter()
                .any(|message| message.role.as_str() == "assistant")
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
        .filter(|message| message.role.as_str() == "assistant")
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
            .filter(|attempt| attempt.status == AttemptStatus::Completed)
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
            .filter(|attempt| attempt.status == AttemptStatus::Completed)
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
        "[]",
        support::FIXED_TEXT,
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
    assert_eq!(state.status, WorkStatus::Completed);
    assert_eq!(assistant_texts(&state), [support::FIXED_TEXT]);
    assert_eq!(attempt_statuses(&state), ["failed", "completed"]);
    assert_eq!(state.attempts.len(), 2);
    let first = &state.attempts[0];
    let second = &state.attempts[1];
    assert_ne!(
        first.id, second.id,
        "redelivery claims a new canonical attempt"
    );
    assert_eq!(first.work, second.work);
    assert_eq!(second.predecessor.as_deref(), Some(first.id.as_str()));
    // The stub's broken response ends the first exchange as a failure: it is
    // neither retried within the run nor held as a provider block.
    use harness::engine::EventKind;
    assert!(
        first
            .events
            .iter()
            .any(|event| matches!(event.kind, EventKind::ProviderFailed { .. })),
        "{:?}",
        first.events
    );
    assert!(!first.events.iter().any(|event| matches!(
        event.kind,
        EventKind::TransportRetry { .. } | EventKind::ProviderBlocked { .. }
    )));
    assert_eq!(
        running.stub.requests().len(),
        2,
        "one request for each attempt"
    );
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
/// generation; a turn in any other practice reports no scenario, even if its
/// input somehow carries one. This worker's model is the local Responses stub,
/// so its own origin is `simulation` for every turn; `hosting`'s unit tests
/// cover a real practice staying `real`.
#[tokio::test]
async fn a_request_the_provider_rejects_holds_the_turn_and_is_not_sent_again() {
    let _guard = lock().lock().await;
    let Some(running) = running("rejected", None, None).await else {
        return;
    };
    running.stub.reject_next(1);
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;
    running.emulator.publish(&delivery).await;
    let deadline = Instant::now() + Duration::from_secs(30);
    let held = loop {
        let state = running.store.conversation(&scope).await.unwrap();
        if state.status == WorkStatus::Held {
            break state;
        }
        assert!(
            Instant::now() < deadline,
            "the rejected turn was not held: {state:?}\n{}",
            running.worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    };
    assert_eq!(attempt_statuses(&held), ["held"]);
    assert_eq!(
        held.run().unwrap().outcome,
        Some(harness::engine::runs::RunOutcome::Held {
            reason: harness::engine::runs::HoldReason::ProviderBlocked
        })
    );
    assert_eq!(
        held.failures, 0,
        "a rejected request is not a failed attempt"
    );
    assert_eq!(running.stub.requests().len(), 1);

    // A redelivery of the same turn does not send the request again.
    let body = support::push_body(&delivery, "rejected-redelivery");
    let (status, text) = support::exchange(
        &running.worker_host,
        "POST",
        "/pubsub/chat-turn",
        &[("content-type", "application/json")],
        Some(&body),
    )
    .await
    .unwrap_or_else(|error| panic!("{error}\n{}", running.worker.tail()));
    assert_eq!(status, 204, "a redelivered held turn should ack: {text}");
    tokio::time::sleep(Duration::from_secs(1)).await;
    let after = running.store.conversation(&scope).await.unwrap();
    assert_eq!(attempt_statuses(&after), ["held"]);
    assert!(assistant_texts(&after).is_empty());
    assert_eq!(
        running.stub.requests().len(),
        1,
        "a held turn must not send the rejected request again"
    );
    // The site's recovery sweep leaves the held turn alone.
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
        "the recovery sweep republished a held turn"
    );
}

#[tokio::test]
async fn origin_follows_the_practice_of_the_claimed_turn() {
    let _guard = lock().lock().await;
    let collector = TraceCapture::start().await;
    let rehearsal = format!("rehearsal-{}", uuid::Uuid::new_v4().simple());
    let Some(running) = running_with(
        "origin",
        None,
        None,
        "[]",
        support::FIXED_TEXT,
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
    let (other_scope, other_delivery) =
        accept_in(url, &running.store, "Hello", None, Some("drill.rehearse")).await;
    running.emulator.publish(&simulated_delivery).await;
    running.emulator.publish(&other_delivery).await;
    let simulated = wait_assistant(&running.store, &simulated_scope, &running.worker).await;
    let other = wait_assistant(&running.store, &other_scope, &running.worker).await;
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
    let (simulated_span, other_span) = loop {
        let spans = collector.spans.lock().unwrap().clone();
        if let (Some(a), Some(b)) = (generation(&spans, &simulated), generation(&spans, &other)) {
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
        trace_attribute(&other_span, "langfuse.environment").as_deref(),
        Some("simulation")
    );
    assert_eq!(trace_attribute(&other_span, "coda.scenario_id"), None);
    assert_eq!(tags(&other_span), ["physician_chat", "simulation"]);
}

/// The full export path with content on: the worker's generation goes
/// through the real collector config to a real Langfuse, which holds the
/// prompt and reply under the rehearsal's environment, session and run; the
/// operations pipelines deliver the same generation, with its model, to the
/// test's sink and carry neither text anywhere. Needs Docker with the images
/// in `dev/observability/test/langfuse.yaml` pulled, plus the Postgres gate.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "starts Langfuse and the collector in Docker"]
async fn content_reaches_langfuse_and_never_the_operations_backend() {
    use telemetry::testing::{Langfuse, PUBLIC_KEY, SECRET_KEY, Shape};
    let _guard = lock().lock().await;
    if support::gate_url().is_none() {
        return;
    }
    let unique = uuid::Uuid::new_v4().simple().to_string();
    let prompt_canary = format!("full-path-prompt-{unique}");
    let reply_canary = format!("full-path-reply-{unique}");
    let reply = format!("Noted, {reply_canary}.");
    let ops = TraceCapture::start().await;
    let langfuse = tokio::task::spawn_blocking({
        let port = ops.port;
        move || {
            Langfuse::start(
                "physician-full-path",
                Shape::Ingest {
                    ops_sink_port: port,
                },
            )
        }
    })
    .await
    .expect("compose");
    let otlp = langfuse.otlp_http.clone().expect("collector receiver");
    let rehearsal = format!("rehearsal-{unique}");
    let Some(running) = running_with(
        "full_path",
        None,
        None,
        "[]",
        reply.as_str(),
        &[
            ("CODA_HOSTED_PROFILE", "staging-preview"),
            ("PHYSICIAN_SIMULATION_PRACTICES", rehearsal.as_str()),
            ("CODA_TELEMETRY_CONTENT", "full"),
            ("OTEL_EXPORTER_OTLP_ENDPOINT", otlp.as_str()),
            ("OTEL_BSP_SCHEDULE_DELAY", "50"),
        ],
    )
    .await
    else {
        return;
    };
    let (scope, delivery) = accept_in(
        &running.database.url,
        &running.store,
        &format!("Rehearse {prompt_canary}"),
        Some(&rehearsal),
        Some("drill.full-path"),
    )
    .await;
    running.emulator.publish(&delivery).await;
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(assistant_texts(&state), [reply.as_str()]);
    let run = state.attempts.last().expect("an attempt").id.clone();

    // The operations sink: the generation, its run and model, no content.
    let deadline = Instant::now() + Duration::from_secs(60);
    let generation = loop {
        let found = ops
            .spans
            .lock()
            .unwrap()
            .iter()
            .find(|span| {
                span.name == "gen_ai.generation"
                    && trace_attribute(span, "coda.run_id").as_deref() == Some(run.as_str())
            })
            .cloned();
        if let Some(span) = found {
            break span;
        }
        assert!(
            Instant::now() < deadline,
            "no generation reached the operations sink\n{}\n{}",
            running.worker.tail(),
            langfuse.logs()
        );
        tokio::time::sleep(Duration::from_millis(200)).await;
    };
    assert!(trace_attribute(&generation, "gen_ai.request.model").is_some());
    assert_eq!(
        trace_attribute(&generation, "coda.scenario_id").as_deref(),
        Some("drill.full-path")
    );
    for key in [
        "langfuse.observation.input",
        "langfuse.observation.output",
        "langfuse.session.id",
        "langfuse.trace.tags",
    ] {
        assert_eq!(trace_attribute(&generation, key), None, "{key} reached ops");
    }
    let trace_id: String = generation
        .trace_id
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();

    // Langfuse: the same trace, with the rehearsal's labels and the content.
    let client = reqwest::Client::new();
    let deadline = Instant::now() + Duration::from_secs(180);
    let trace: serde_json::Value = loop {
        let response = client
            .get(format!("{}/api/public/traces/{trace_id}", langfuse.url))
            .basic_auth(PUBLIC_KEY, Some(SECRET_KEY))
            .send()
            .await;
        if let Ok(response) = response
            && response.status().is_success()
        {
            let body: serde_json::Value = response.json().await.expect("trace JSON");
            let generated = body["observations"]
                .as_array()
                .is_some_and(|all| all.iter().any(|o| o["name"] == "gen_ai.generation"));
            if generated {
                break body;
            }
        }
        assert!(
            Instant::now() < deadline,
            "Langfuse never held trace {trace_id}\n{}",
            langfuse.logs()
        );
        tokio::time::sleep(Duration::from_secs(2)).await;
    };
    let observation = trace["observations"]
        .as_array()
        .and_then(|all| all.iter().find(|o| o["name"] == "gen_ai.generation"))
        .expect("generation observation");
    assert_eq!(trace["sessionId"], scope.chat.as_str(), "{trace}");
    assert_eq!(observation["environment"], "simulation", "{observation}");
    assert_eq!(observation["type"], "GENERATION", "{observation}");
    let metadata = observation["metadata"].to_string();
    assert!(metadata.contains(&run), "run id not in {metadata}");
    assert!(
        observation["input"].to_string().contains(&prompt_canary),
        "prompt missing from {observation}"
    );
    assert!(
        observation["output"].to_string().contains(&reply_canary),
        "reply missing from {observation}"
    );

    // Everything the operations pipelines delivered, traces and logs, raw.
    tokio::time::sleep(Duration::from_secs(2)).await;
    let bodies = ops.bodies.lock().unwrap().clone();
    assert!(!bodies.is_empty());
    for body in &bodies {
        for canary in [&prompt_canary, &reply_canary] {
            assert!(
                !body
                    .windows(canary.len())
                    .any(|window| window == canary.as_bytes()),
                "{canary} reached the operations sink"
            );
        }
    }
    drop(running);
    drop(langfuse);
}

/// Each attempt's status as its saved text.
fn attempt_statuses(state: &Conversation) -> Vec<String> {
    state
        .attempts
        .iter()
        .map(|attempt| sea_orm::ActiveEnum::to_value(&attempt.status))
        .collect()
}

/// A rehearsal practice's `[fail:1]` fails its first attempt on purpose,
/// before any model call: the attempt span says `rehearsed`, and the
/// redelivery completes the same work. A real practice's `[fail:1]` answers
/// on its first attempt.
#[tokio::test]
async fn a_rehearsal_turn_marked_to_fail_fails_once_and_its_successor_answers() {
    let _guard = lock().lock().await;
    let collector = TraceCapture::start().await;
    let rehearsal = format!("rehearsal-{}", uuid::Uuid::new_v4().simple());
    let Some(running) = running_with(
        "rehearsed_failure",
        None,
        None,
        "[]",
        support::FIXED_TEXT,
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
    let (scope, marked) = accept_in(
        url,
        &running.store,
        "Drill [fail:1]",
        Some(&rehearsal),
        Some("drill.chat-failure"),
    )
    .await;
    running.emulator.publish(&marked).await;
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(state.status, WorkStatus::Completed);
    assert_eq!(assistant_texts(&state), [support::FIXED_TEXT]);
    assert_eq!(attempt_statuses(&state), ["failed", "completed"]);
    let (failed, successor) = (&state.attempts[0], &state.attempts[1]);
    assert_eq!(failed.work, successor.work, "one work across both attempts");
    assert_eq!(successor.predecessor.as_deref(), Some(failed.id.as_str()));

    let deadline = Instant::now() + Duration::from_secs(10);
    let failed_span = loop {
        let found = collector
            .spans
            .lock()
            .unwrap()
            .iter()
            .find(|span| {
                span.name == "physician.worker.attempt"
                    && trace_attribute(span, "coda.run_id").as_deref() == Some(failed.id.as_str())
            })
            .cloned();
        if let Some(span) = found {
            break span;
        }
        assert!(
            Instant::now() < deadline,
            "failed attempt span not exported\n{}",
            running.worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    };
    assert_eq!(
        trace_attribute(&failed_span, "error.type").as_deref(),
        Some("rehearsed")
    );
    assert_eq!(
        trace_attribute(&failed_span, "coda.work_id").as_deref(),
        Some(failed.work.as_str())
    );

    let (real_scope, real) = accept_in(url, &running.store, "Real [fail:1]", None, None).await;
    running.emulator.publish(&real).await;
    let state = wait_assistant(&running.store, &real_scope, &running.worker).await;
    assert_eq!(
        attempt_statuses(&state),
        ["completed"],
        "a real practice never honours the marker"
    );
}

#[tokio::test]
async fn fifth_failure_is_forwarded_to_the_dead_letter_topic() {
    let _guard = lock().lock().await;
    let Some(mut running) = running("dlq", Some(100), None).await else {
        return;
    };
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;
    running.emulator.publish(&delivery).await;

    let restart_deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let state = running.store.conversation(&scope).await.unwrap();
        if state.failures == 2 {
            assert_eq!(attempt_statuses(&state), ["failed", "failed"]);
            running.worker.restart();
            let port = running
                .worker_host
                .rsplit_once(':')
                .unwrap()
                .1
                .parse()
                .unwrap();
            support::wait_ready(&mut running.worker, port).await;
            break;
        }
        assert!(
            state.failures < 2,
            "missed the second failed attempt: {state:?}"
        );
        assert!(
            Instant::now() < restart_deadline,
            "second failure did not arrive"
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

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
    assert_eq!(attempt_statuses(&state), ["failed"; 5]);
    assert_eq!(
        state.status,
        WorkStatus::Failed,
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
/// as the admitting site sees it.
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

/// Each saved receipt: its original output, or the reason of its refusal.
fn tool_returned(state: &Conversation) -> Vec<(String, String, bool)> {
    use harness::engine::results::ToolResult;
    state
        .attempts
        .iter()
        .flat_map(|attempt| &attempt.events)
        .filter_map(|event| match &event.kind {
            harness::engine::EventKind::ToolReturned {
                call_id,
                result: ToolResult::Original(output),
            } => Some((call_id.clone(), output.clone(), true)),
            harness::engine::EventKind::ToolReturned {
                call_id,
                result: ToolResult::Refused(refusal),
            } => Some((call_id.clone(), refusal.reason().to_owned(), false)),
            _ => None,
        })
        .collect()
}

/// The real worker binary, its host and Postgres, with a scripted provider:
/// round one reads a skill body next to malformed, unknown and unavailable calls; round two reads a supporting resource; then it answers.
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
    let resource = skills.join("patient-record-retrieval/references/authoring-guide.md");
    std::fs::create_dir_all(resource.parent().unwrap()).unwrap();
    std::fs::write(
        &resource,
        "Only the admitting site saw these guidance bytes.",
    )
    .unwrap();
    let admitted = PhysicianSkills::load(&skills).unwrap();
    let catalog: harness::skills::Package =
        serde_json::from_value(serde_json::to_value(&admitted).unwrap()).unwrap();
    let content = r#"{"code":"2160-0","value":0.9,"unit":"mg/dL"}"#;
    let revision = local_product::sha256_hex(content.as_bytes());
    let rounds = serde_json::json!([
        [
            {"name": "skill_view", "arguments": {"name": "patient-record-retrieval", "file_path": null}},
            {"name": "skill_view", "arguments": {"name": 0}},
            {"name": "skill_view", "arguments": "not json"},
            {"name": "write_patient_memory", "arguments": {}},
            {"name": "skill_view", "arguments": {"name": "not-a-skill", "file_path": null}},
            {"name": "skill_view", "arguments": {"name": "patient-record-retrieval", "file_path": "references/absent.md"}},
            {"name": "search_patient_information", "arguments": {"query":"2160-0","record_type":"Observation","after":null}}
        ],
        [
            {"name": "skill_view", "arguments": {"name": "patient-record-retrieval", "file_path": "references/authoring-guide.md"}},
            {"name":"read_patient_sources","arguments":{"references":[{"candidate_id":"Observation/result","revision":revision,"cursor":null}],"max_result_bytes":4096}}
        ],
        [
            {"name": "publish_citations", "arguments": {"previews": [{"reference": "Observation/result", "revision": revision, "preview": "Creatinine 1.1 mg/dL on lab observation."}], "citations": []}}
        ]
    ])
    .to_string();
    let Some(running) = running_with("skills", None, None, &rounds, support::FIXED_TEXT, &[]).await
    else {
        let _ = std::fs::remove_dir_all(&staging);
        return;
    };
    let (scope, delivery) = accept_with_skills(
        &running.database.url,
        &running.store,
        "Hello",
        &admitted,
        None,
        None,
    )
    .await;
    std::fs::remove_dir_all(&skills).unwrap();
    let record = local_product::physician_sources::SourceRevision {
        workspace: scope.workspace.clone(),
        source_id: "synthetic-charts".into(),
        candidate_id: "Observation/result".into(),
        revision: revision.clone(),
        record_type: "Observation".into(),
        dates: vec![],
        content: content.into(),
        sha256: revision.clone(),
        readable_parts: vec![],
        provenance: serde_json::json!({"native_record":"Observation/result"}),
        native: None,
    };
    running.store.record_source(&record).await.unwrap();
    running.emulator.publish(&delivery).await;
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(state.status, WorkStatus::Completed);
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
    let recipe = store
        .frozen_binding(&scope, saved.bindings.get(&saved.work).unwrap())
        .await
        .unwrap();
    let requests: Vec<_> = saved.attempts[0]
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            harness::engine::EventKind::ProviderStarted {
                request: context, ..
            } => Some(
                recipe.request(
                    context
                        .restore(&saved.attempts[0].events, &saved.message_sources())
                        .unwrap()
                        .expect("saved request sources remain available"),
                ),
            ),
            _ => None,
        })
        .collect();
    assert_eq!(requests.len(), 4, "three tool rounds and one answer");
    let offered = local_product::physician_chat::recipe(
        &support::models::roles().chat,
        local_product::physician_chat::INSTRUCTIONS,
        &admitted,
    )
    .tools;
    for context in &requests {
        assert!(
            context
                .instructions
                .contains(local_product::physician_chat::INSTRUCTIONS)
        );
        assert!(
            context
                .instructions
                .contains(&catalog.catalog_instructions())
        );
        assert_eq!(context.tools, offered);
    }
    let returned = tool_returned(&saved);
    assert_eq!(returned.len(), 10);
    let output = |id: &str| {
        returned
            .iter()
            .find(|(call, ..)| call == id)
            .unwrap_or_else(|| panic!("no result for {id}"))
    };
    let found: serde_json::Value = serde_json::from_str(&output("fixed-1-6").1).unwrap();
    assert_eq!(found["results"][0]["reference"]["revision"], revision);
    assert!(!output("fixed-1-6").1.contains("mg/dL"));
    let opened: serde_json::Value = serde_json::from_str(&output("fixed-2-1").1).unwrap();
    assert_eq!(opened["results"][0]["content"], content);
    let exact = |name: &str, path: Option<&str>| {
        catalog
            .execute_skill_view(&serde_json::json!({"name":name,"file_path":path}))
            .unwrap()
            .to_json()
    };
    assert_eq!(
        output("fixed-1-0"),
        &(
            "fixed-1-0".into(),
            exact("patient-record-retrieval", None),
            true
        )
    );
    assert_eq!(output("fixed-1-4").1, exact("not-a-skill", None));
    assert_eq!(
        output("fixed-1-5").1,
        exact("patient-record-retrieval", Some("references/absent.md"))
    );
    assert!(output("fixed-1-5").1.contains("\"unavailable\""));
    assert_eq!(
        output("fixed-2-0").1,
        exact(
            "patient-record-retrieval",
            Some("references/authoring-guide.md")
        )
    );
    for refused in ["fixed-1-1", "fixed-1-2", "fixed-1-3"] {
        let (_, _, executed) = output(refused);
        assert!(!executed, "{refused}");
    }
    drop(running);
    let _ = std::fs::remove_dir_all(&staging);
}

/// A whole-record read reaches the model with its `citation_id`; the answer's
/// marker publishes the exact bytes read, and its summary line leaves the text.
#[tokio::test]
async fn chart_reads_and_exact_citations_survive_readback_and_duplicate_delivery() {
    let _guard = lock().lock().await;
    let content = "{\"potassium\":4.1}";
    let reply = "Controlled source read [[patient_source:{{citation_id}}]]\n\n[[basis]]\nRests on one potassium result.\n[[/basis]]";
    let Some(running) = running_with("chart_citations", None, None, "[]", reply, &[]).await else {
        return;
    };
    let (scope, delivery, session) =
        accept_signed_in(&running.database.url, &running.store, "Hello").await;
    let mut source = local_product::physician_sources::SourceRevision {
        workspace: scope.workspace.clone(),
        source_id: "test-ehr".into(),
        candidate_id: "lab".into(),
        revision: "r1".into(),
        record_type: "Observation".into(),
        dates: vec![],
        content: content.into(),
        sha256: local_product::sha256_hex(content.as_bytes()),
        readable_parts: vec![],
        provenance: serde_json::json!({"native_record":"lab"}),
        native: None,
    };
    running.store.record_source(&source).await.unwrap();
    let mut other = source.clone();
    other.workspace = "another-patient".into();
    other.candidate_id = "other-lab".into();
    running.store.record_source(&other).await.unwrap();
    running.emulator.publish(&delivery).await;
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    let answer = state.messages.last().unwrap();
    let citation = &answer.citations[0];
    assert_eq!(citation.content, content);
    assert!(
        answer
            .text
            .starts_with("Controlled source read [[patient_source:")
    );
    assert_eq!(citation.summary.as_deref(), Some("Records potassium 4.1."));
    assert!(!answer.text.contains("basis"), "{}", answer.text);
    assert_eq!(
        answer.basis.as_deref(),
        Some("Rests on one potassium result.")
    );
    let answered_at = answer
        .sources_version
        .expect("the answer names its sources version");
    assert_eq!(
        running
            .store
            .source_version(&scope.workspace)
            .await
            .unwrap(),
        answered_at
    );
    assert_eq!(
        citation.id,
        local_product::physician_sources::citation_id(&source)
    );
    assert!(!format!("{state:?}").contains("other-lab"));
    read_citation_evidence(&running, &scope, &session, citation).await;
    let original = serde_json::to_value(&state.messages).unwrap();
    source.revision = "r2".into();
    source.content = "{\"potassium\":6.8}".into();
    source.sha256 = local_product::sha256_hex(source.content.as_bytes());
    running.store.record_source(&source).await.unwrap();
    assert!(
        running
            .store
            .source_version(&scope.workspace)
            .await
            .unwrap()
            > answered_at
    );
    let body = support::push_body(&delivery, "duplicate-citation-delivery");
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
    let fresh = Store::connect(&running.database.url).await.unwrap();
    let saved = fresh.read(&scope).await.unwrap().unwrap();
    assert_eq!(serde_json::to_value(saved.messages).unwrap(), original);
    assert_eq!(saved.attempts.len(), 1);
}

/// Opening the answer's citation reads the exact revision it retained, through a real site on the
/// same Postgres, under the same workspace authority as the conversation itself: a same-clinic
/// colleague without the patient on their panel, no session, and an id the answer never cited
/// read nothing.
async fn read_citation_evidence(
    running: &Running,
    scope: &Scope,
    session: &str,
    citation: &local_product::physician_chat::Citation,
) {
    let port = support::free_port();
    let listen = format!("127.0.0.1:{port}");
    let topic = support::topic();
    let mut site = support::Process::spawn(
        "site",
        support::site_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", running.database.url.as_str()),
            ("PUBSUB_EMULATOR_HOST", running.emulator.host.as_str()),
            ("GOOGLE_CLOUD_PROJECT", support::PROJECT),
            ("PHYSICIAN_CHAT_TOPIC", topic.as_str()),
            ("PHYSICIAN_AUTH", "required"),
            ("PHYSICIAN_LISTEN_ADDR", listen.as_str()),
        ],
        &running.home,
    );
    support::wait_ready(&mut site, port).await;
    let path = |id: &str| {
        format!(
            "/patient-workspaces/{}/chats/{}/citations/{id}",
            scope.workspace, scope.chat
        )
    };
    let (status, body) = support::get_json(&listen, &path(&citation.id), session).await;
    assert_eq!(status, 200, "{body}\n{}", site.tail());
    let read: physician_wire::SourceEvidence = serde_json::from_value(body).unwrap();
    let mut source = physician_sample::source_inspection::inspection(citation);
    assert!(source.coda_preview.is_some(), "this answer summarized it");
    // The summary is this answer's, and comes with it; the read is the source alone.
    source.coda_preview = None;
    assert_eq!(read.inspection, source);
    assert_eq!(read.inspection.technical.original, citation.content);
    assert_eq!(read.inspection.technical.revision, "r1");

    let (status, body) = support::get_json(&listen, &path("never-cited"), session).await;
    assert_eq!(status, 404, "{body}");

    let colleague = colleague_session(&running.database.url, scope).await;
    let (status, body) = support::get_json(&listen, &path(&citation.id), &colleague).await;
    assert_eq!(
        status, 404,
        "a colleague off the panel reads no evidence: {body}"
    );

    let (status, text) = support::exchange(&listen, "GET", &path(&citation.id), &[], None)
        .await
        .unwrap();
    assert_eq!(status, 401, "{text}");
    site.kill_now();
}

/// A second physician in `practice`, signed in, without the patient on their panel.
async fn colleague_session(url: &str, scope: &Scope) -> String {
    let practice = scope.practice.as_str();
    let auth = PostgresAuthStore::connect_without_migrations(url)
        .await
        .expect("auth store");
    let now = support::unix_now();
    let unique = uuid::Uuid::new_v4().simple().to_string();
    let email = format!("colleague-{unique}@example.test");
    auth.ensure_account(
        practice,
        &format!("colleague-{unique}"),
        &email,
        "Colleague",
        false,
        now,
    )
    .await
    .expect("colleague account");
    let token = auth
        .begin_sign_in(&email, now)
        .await
        .expect("sign-in token");
    let (identity, session) = auth
        .complete_sign_in(&email, &token, now)
        .await
        .expect("colleague session");
    assert_eq!(
        auth.membership(&identity, &scope.workspace).await,
        Membership::ClinicDemographic
    );
    session
}

#[tokio::test]
async fn hosting_cancels_a_chat_whose_claim_has_been_replaced() {
    use harness::engine::{AttemptFailure, EventKind};
    let _guard = lock().lock().await;
    let Some(running) = running("claim_replaced", None, Some(200_000)).await else {
        return;
    };
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;
    let host = running.worker_host.clone();
    let body = support::push_body(&delivery, "claim-replaced");
    let push = tokio::spawn(async move {
        reqwest::Client::new()
            .post(format!("http://{host}/pubsub/chat-turn"))
            .header("content-type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap()
    });
    let started = Instant::now();
    let abandoned = loop {
        let state = running.store.conversation(&scope).await.unwrap();
        if let Some(attempt) = state.attempts.first()
            && attempt
                .events
                .iter()
                .any(|event| matches!(event.kind, EventKind::ProviderStarted { .. }))
        {
            break local_product::runtime::Claim {
                scope: scope.clone(),
                attempt: attempt.id.clone(),
                revision: attempt.revision,
                lease: local_product::runtime::Lease::new(540).unwrap(),
            };
        }
        assert!(started.elapsed() < Duration::from_secs(10));
        tokio::time::sleep(Duration::from_millis(50)).await;
    };
    running
        .store
        .settle_chat_attempt(&abandoned, AttemptFailure::Interrupted)
        .await
        .unwrap();
    let successor = running.store.claim(&scope, 540).await.unwrap().unwrap();
    assert_ne!(successor.attempt, abandoned.attempt);
    let response = tokio::time::timeout(Duration::from_secs(115), push)
        .await
        .expect("hosting must cancel the old run at its 108-second renewal cadence")
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);
    let saved = running.store.conversation(&scope).await.unwrap();
    assert_eq!(saved.attempts.len(), 2);
    assert_eq!(saved.attempts[1].id, successor.attempt);
    assert_eq!(saved.attempts[1].status, AttemptStatus::Active);
    assert!(assistant_texts(&saved).is_empty());
    assert_eq!(saved.attempts[0].status, AttemptStatus::Interrupted);
    println!(
        "Replaced hosted claim stopped after {} seconds; successor remains current",
        started.elapsed().as_secs()
    );
}

/// The summary contract against a real model, not a fixture: one allergy question over the
/// patient bank's no-known-drug-allergies record. Run explicitly with an OpenAI key; it writes
/// what the model answered and summarized to `PHYSICIAN_FUNNEL_EVIDENCE` when set.
#[tokio::test]
#[ignore = "calls OpenAI; run with CODA_ALLOW_MODEL_CALLS=1 and OPENAI_API_KEY"]
async fn a_real_model_summarizes_the_record_it_cites() {
    let _guard = lock().lock().await;
    let (Ok(allow), Ok(key)) = (
        std::env::var("CODA_ALLOW_MODEL_CALLS"),
        std::env::var("OPENAI_API_KEY"),
    ) else {
        panic!("set CODA_ALLOW_MODEL_CALLS=1 and OPENAI_API_KEY");
    };
    let Some(running) = running_with(
        "funnel_live",
        None,
        None,
        "[]",
        support::FIXED_TEXT,
        &[
            // An empty stub port is unset, so the worker calls the API.
            ("PHYSICIAN_RESPONSES_STUB_PORT", ""),
            ("CODA_LOCAL_MODEL_EXECUTION", "api"),
            ("CODA_ALLOW_MODEL_CALLS", allow.as_str()),
            ("OPENAI_API_KEY", key.as_str()),
        ],
    )
    .await
    else {
        return;
    };
    let (scope, delivery) = accept_text(
        &running.database.url,
        &running.store,
        "Does this patient have any drug allergies on record?",
    )
    .await;
    let content = r#"{"resourceType":"AllergyIntolerance","id":"allergy-0493010f2a16","clinicalStatus":{"coding":[{"code":"active","display":"Active"}]},"verificationStatus":{"coding":[{"code":"confirmed","display":"Confirmed"}]},"type":"allergy","category":["medication"],"criticality":"low","code":{"coding":[{"system":"http://snomed.info/sct","code":"409137002","display":"No known drug allergy"}],"text":"No known drug allergies"},"recordedDate":"2018-09-11T12:20:00+00:00"}"#;
    let source = local_product::physician_sources::SourceRevision {
        workspace: scope.workspace.clone(),
        source_id: "synthetic-ehr".into(),
        candidate_id: "AllergyIntolerance/allergy-0493010f2a16".into(),
        revision: "1".into(),
        record_type: "AllergyIntolerance".into(),
        dates: vec![local_product::physician_sources::RecordDate {
            name: "recordedDate".into(),
            value: "2018-09-11T12:20:00+00:00".into(),
        }],
        content: content.into(),
        sha256: local_product::sha256_hex(content.as_bytes()),
        readable_parts: vec![],
        provenance: serde_json::json!({"native_record":"AllergyIntolerance/allergy-0493010f2a16"}),
        native: None,
    };
    running.store.record_source(&source).await.unwrap();
    running.emulator.publish(&delivery).await;
    let deadline = Instant::now() + Duration::from_secs(240);
    let state = loop {
        if let Some(state) = running.store.read(&scope).await.unwrap()
            && (state
                .messages
                .iter()
                .any(|m| m.role.as_str() == "assistant")
                || state.status == WorkStatus::Failed)
        {
            break state;
        }
        assert!(
            Instant::now() < deadline,
            "no answer\n{}",
            running.worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(500)).await;
    };
    let raw: Vec<String> = state
        .attempts
        .iter()
        .flat_map(|attempt| &attempt.events)
        .filter_map(|event| match &event.kind {
            harness::engine::EventKind::Candidate { text } => Some(text.clone()),
            _ => None,
        })
        .collect();
    let answer = state
        .messages
        .iter()
        .find(|m| m.role.as_str() == "assistant");
    let evidence = serde_json::json!({
        "instructions_sha256": local_product::sha256_hex(
            local_product::physician_chat::INSTRUCTIONS.as_bytes()
        ),
        "status": format!("{:?}", state.status),
        "model_candidate": raw,
        "published_text": answer.map(|m| m.text.clone()),
        "citations": answer.map(|m| m.citations.iter().map(|c| serde_json::json!({
            "id": c.id, "reference": c.reference, "summary": c.summary,
        })).collect::<Vec<_>>()),
    });
    println!("{}", serde_json::to_string_pretty(&evidence).unwrap());
    if let Ok(path) = std::env::var("PHYSICIAN_FUNNEL_EVIDENCE") {
        std::fs::write(path, serde_json::to_string_pretty(&evidence).unwrap()).unwrap();
    }
    let answer = answer.unwrap_or_else(|| panic!("no answer\n{}", running.worker.tail()));
    assert!(!answer.text.contains("source_summaries"), "{evidence}");
    assert!(!answer.citations.is_empty(), "{evidence}");
}

/// One cited lab record for the layout proofs.
const OBSERVATION: &str = r#"{"resourceType":"Observation","id":"k-1","status":"final","code":{"text":"Potassium"},"valueQuantity":{"value":4.1,"unit":"mmol/L"},"referenceRange":[{"low":{"value":3.5,"unit":"mmol/L"},"high":{"value":5.1,"unit":"mmol/L"}}]}"#;

/// A layout that reads [`OBSERVATION`].
const OBSERVATION_LAYOUT: &str = r#"{"record_type":"Observation","statuses":[{"role":"status","pointer":"/status"}],"sections":[{"heading":"result","slots":[{"role":"code","pointer":"/code/text"},{"role":"result","pointer":"/valueQuantity/value"},{"role":"unit","pointer":"/valueQuantity/unit"}]}]}"#;

const CITING_TEXT: &str = "Controlled source read [[patient_source:{{citation_id}}]]";

/// A worker whose stubbed answer cites the chart's first record, and whose fixed layout model is
/// configured by `layout`.
async fn citing_worker(label: &str, layout: (&str, &str)) -> Option<Running> {
    running_with(label, None, None, "[]", CITING_TEXT, &[layout]).await
}

/// Records [`OBSERVATION`] from a new source in `scope`'s chart and returns that source's id.
async fn record_observation(store: &Store, scope: &Scope) -> String {
    let source_id = format!("ehr-{}", uuid::Uuid::new_v4().simple());
    let source = local_product::physician_sources::SourceRevision {
        workspace: scope.workspace.clone(),
        source_id: source_id.clone(),
        candidate_id: "Observation/k-1".into(),
        revision: "1".into(),
        record_type: "Observation".into(),
        dates: vec![],
        content: OBSERVATION.into(),
        sha256: local_product::sha256_hex(OBSERVATION.as_bytes()),
        readable_parts: vec![],
        provenance: serde_json::json!({"native_record":"Observation/k-1"}),
        native: None,
    };
    store.record_source(&source).await.unwrap();
    source_id
}

fn observation_key(source_id: &str) -> local_product::physician_layouts::LayoutKey<'_> {
    local_product::physician_layouts::LayoutKey {
        source_id,
        record_type: "Observation",
    }
}

/// Waits until the worker log shows `line`, the end of its layout work for an answer.
async fn wait_log(worker: &support::Process, line: &str) {
    let deadline = Instant::now() + Duration::from_secs(30);
    while !worker.tail().contains(line) {
        assert!(
            Instant::now() < deadline,
            "worker never logged {line:?}\n{}",
            worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

/// Answers one cited turn and returns the published conversation.
async fn answer_citing(running: &Running, scope: &Scope, delivery: &Delivery) -> Conversation {
    running.emulator.publish(delivery).await;
    let state = wait_assistant(&running.store, scope, &running.worker).await;
    let answer = state.messages.last().unwrap();
    assert!(
        answer
            .text
            .starts_with("Controlled source read [[patient_source:"),
        "{:?}",
        answer.text
    );
    assert_eq!(answer.citations.len(), 1);
    state
}

/// The first answer citing a source's Observation makes that source's layout, which then reads
/// the record; a second answer citing the same key asks no model and changes nothing.
#[tokio::test]
async fn a_first_cited_record_type_gets_a_layout_that_later_citations_read() {
    let _guard = lock().lock().await;
    let Some(running) = citing_worker(
        "layout_stored",
        ("PHYSICIAN_WORKER_FIXED_LAYOUT", OBSERVATION_LAYOUT),
    )
    .await
    else {
        return;
    };
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;
    let source_id = record_observation(&running.store, &scope).await;
    let state = answer_citing(&running, &scope, &delivery).await;
    wait_log(&running.worker, "source layout stored").await;
    let stored = running
        .store
        .layout_for(observation_key(&source_id))
        .await
        .unwrap()
        .expect("layout stored");
    assert_eq!(stored.layout, OBSERVATION_LAYOUT);
    let layout = physician_sample::source_layout::Layout::parse(&stored.layout).unwrap();
    let cited = &state.messages.last().unwrap().citations[0];
    let view = physician_sample::source_inspection::inspection_with_layout(cited, Some(&layout));
    assert_eq!(view.layout_sections[0].heading, "Result");

    // The same source and type cited by a second answer: the stored layout stays, unasked.
    let (second_scope, second) = accept_turn(&running.database.url, &running.store).await;
    let again = local_product::physician_sources::SourceRevision {
        workspace: second_scope.workspace.clone(),
        source_id: source_id.clone(),
        candidate_id: "Observation/k-1".into(),
        revision: "1".into(),
        record_type: "Observation".into(),
        dates: vec![],
        content: OBSERVATION.into(),
        sha256: local_product::sha256_hex(OBSERVATION.as_bytes()),
        readable_parts: vec![],
        provenance: serde_json::json!({"native_record":"Observation/k-1"}),
        native: None,
    };
    running.store.record_source(&again).await.unwrap();
    answer_citing(&running, &second_scope, &second).await;
    assert_eq!(
        running
            .store
            .layout_for(observation_key(&source_id))
            .await
            .unwrap(),
        Some(stored)
    );
    assert_eq!(
        running
            .worker
            .tail()
            .matches("source layout stored")
            .count(),
        1,
        "{}",
        running.worker.tail()
    );
}

/// A layout the checks refuse, and a layout model that fails, each save nothing; the answer is
/// published either way and the record keeps the general view.
#[tokio::test]
async fn a_refused_or_failed_layout_saves_nothing_and_the_answer_still_publishes() {
    let _guard = lock().lock().await;
    // The planted presentation error: a valid pointer at the range's low limit read as the result.
    let planted = OBSERVATION_LAYOUT.replace("/valueQuantity/value", "/referenceRange/0/low/value");
    for (label, layout) in [
        (
            "layout_refused",
            ("PHYSICIAN_WORKER_FIXED_LAYOUT", planted.as_str()),
        ),
        (
            "layout_failed",
            ("PHYSICIAN_WORKER_FIXED_LAYOUT_FAILS", "1"),
        ),
    ] {
        let Some(running) = citing_worker(label, layout).await else {
            return;
        };
        let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;
        let source_id = record_observation(&running.store, &scope).await;
        let state = answer_citing(&running, &scope, &delivery).await;
        assert_eq!(state.status, WorkStatus::Completed, "{label}");
        wait_log(&running.worker, "source layout not saved").await;
        assert_eq!(
            running
                .store
                .layout_for(observation_key(&source_id))
                .await
                .unwrap(),
            None,
            "{label}"
        );
        let tail = running.worker.tail();
        let causes: &[&str] = if label == "layout_refused" {
            &[
                "role_mismatch",
                "a slot's role does not fit the value it binds: it lies inside a reference range",
            ]
        } else {
            &["fixed layout model failed"]
        };
        for cause in causes {
            assert!(tail.contains(cause), "{label}: {cause}\n{tail}");
        }
        // The refusal names its place by position; the pointer the layout wrote stays out.
        assert!(
            !tail.contains("/referenceRange/0/low/value"),
            "{label}\n{tail}"
        );
    }
}

/// The layout contract against a real model: one potassium question over a cited lab record from
/// a new source. The worker asks its background model for that source's Observation layout after
/// the answer publishes. Run explicitly with an OpenAI key; it writes the record, the model's
/// layout (or why none was saved) and the rendered fields to `PHYSICIAN_LAYOUT_EVIDENCE` when
/// set, for the label-meaning comparison the mechanical checks cannot make.
#[tokio::test]
#[ignore = "calls OpenAI; run with CODA_ALLOW_MODEL_CALLS=1 and OPENAI_API_KEY"]
async fn a_real_model_lays_out_the_record_it_cites() {
    let _guard = lock().lock().await;
    let (Ok(allow), Ok(key)) = (
        std::env::var("CODA_ALLOW_MODEL_CALLS"),
        std::env::var("OPENAI_API_KEY"),
    ) else {
        panic!("set CODA_ALLOW_MODEL_CALLS=1 and OPENAI_API_KEY");
    };
    let Some(running) = running_with(
        "layout_live",
        None,
        None,
        "[]",
        support::FIXED_TEXT,
        &[
            ("PHYSICIAN_RESPONSES_STUB_PORT", ""),
            ("CODA_LOCAL_MODEL_EXECUTION", "api"),
            ("CODA_ALLOW_MODEL_CALLS", allow.as_str()),
            ("OPENAI_API_KEY", key.as_str()),
        ],
    )
    .await
    else {
        return;
    };
    let (scope, delivery) = accept_text(
        &running.database.url,
        &running.store,
        "What was this patient's most recent potassium?",
    )
    .await;
    let source_id = record_observation(&running.store, &scope).await;
    running.emulator.publish(&delivery).await;
    let deadline = Instant::now() + Duration::from_secs(400);
    let outcome = loop {
        let tail = running.worker.tail();
        if let Some(line) = ["source layout stored", "source layout not saved"]
            .into_iter()
            .find(|line| tail.contains(line))
        {
            break line;
        }
        assert!(Instant::now() < deadline, "no layout outcome\n{tail}");
        tokio::time::sleep(Duration::from_millis(500)).await;
    };
    let state = running.store.read(&scope).await.unwrap().unwrap();
    let answer = state
        .messages
        .iter()
        .find(|m| m.role.as_str() == "assistant");
    let stored = running
        .store
        .layout_for(observation_key(&source_id))
        .await
        .unwrap();
    let fields = stored.as_ref().and_then(|stored| {
        let layout = physician_sample::source_layout::Layout::parse(&stored.layout).ok()?;
        let cited = answer?.citations.first()?;
        let view =
            physician_sample::source_inspection::inspection_with_layout(cited, Some(&layout));
        Some(serde_json::to_value(view).unwrap())
    });
    let evidence = serde_json::json!({
        "record": serde_json::from_str::<serde_json::Value>(OBSERVATION).unwrap(),
        "outcome": outcome,
        "layout": stored.as_ref().map(|s| serde_json::from_str::<serde_json::Value>(&s.layout).unwrap()),
        "layout_sha256": stored.as_ref().map(|s| s.sha256.clone()),
        "rendered": fields,
        "worker_log": running.worker.tail(),
    });
    println!("{}", serde_json::to_string_pretty(&evidence).unwrap());
    if let Ok(path) = std::env::var("PHYSICIAN_LAYOUT_EVIDENCE") {
        std::fs::write(path, serde_json::to_string_pretty(&evidence).unwrap()).unwrap();
    }
    assert!(
        answer.is_some_and(|a| !a.citations.is_empty()),
        "{evidence}"
    );
    assert_eq!(outcome, "source layout stored", "{evidence}");
}
