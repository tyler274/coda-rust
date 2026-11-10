//! Physician worker against a disposable Postgres, with no broker.
//!
//! Accepting a turn notifies the worker in the same transaction. Hearing of
//! settled work again runs the fixed provider once, a failed attempt is due
//! again and can then succeed, and the fifth failure ends the turn as failed
//! so nothing starts it again. A worker whose listener is gone still finds
//! accepted work by polling. Concurrent turns on one worker run side by side.

use std::time::{Duration, Instant};

use local_product::runtime::delivery::{Delivery, DeliveryState, TraceContext};
use local_product::runtime::{Conversation, Input, Scope, Store};
use physician_auth::{AuthStore, Membership, PostgresAuthStore};
#[path = "scale/mod.rs"]
mod support;

use support::{ExportedSpan, TraceCapture, trace_attribute};

fn lock() -> &'static tokio::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

struct Running {
    database: support::Database,
    store: Store,
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
        ("PHYSICIAN_WORKER_LISTEN_ADDR", listen.as_str()),
        ("PHYSICIAN_WORKER_FIXED_TEXT", support::FIXED_TEXT),
        support::RETRY_NOW,
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
    accept_as(url, store, text, practice, scenario, None).await
}

/// Accepts `text` whose accepting request carried the trace context `origin`.
async fn accept_from(
    url: &str,
    store: &Store,
    text: &str,
    origin: Option<TraceContext>,
) -> (Scope, Delivery) {
    accept_as(url, store, text, None, None, origin).await
}

/// Accepts one authorized turn, as the site does: the store notifies the
/// worker in the accepting transaction. `origin` is the accepting request's
/// trace context.
async fn accept_as(
    url: &str,
    store: &Store,
    text: &str,
    practice: Option<&str>,
    scenario: Option<&str>,
    origin: Option<TraceContext>,
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
            origin,
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

/// Tells listening workers about `delivery` again, as a second announcement
/// of the same work would.
async fn announce_again(url: &str, delivery: &Delivery) {
    use sea_orm::ConnectionTrait;
    let connection = sea_orm::Database::connect(url).await.expect("connection");
    let payload = serde_json::to_string(delivery).expect("delivery");
    connection
        .execute_raw(sea_orm::Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT pg_notify($1, $2)",
            [
                local_product::runtime::delivery::WORK_CHANNEL.into(),
                payload.into(),
            ],
        ))
        .await
        .expect("notify");
    connection.close().await.expect("close");
}

#[tokio::test]
async fn hearing_of_settled_work_again_runs_the_fixed_provider_once() {
    let _guard = lock().lock().await;
    let Some(running) = running("dup", None, None).await else {
        return;
    };
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;
    let first = wait_assistant(&running.store, &scope, &running.worker).await;
    assert_eq!(assistant_texts(&first), [support::FIXED_TEXT]);
    assert_eq!(attempt_statuses(&first), ["completed"]);

    for _ in 0..3 {
        announce_again(&running.database.url, &delivery).await;
    }
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
        "settled work heard of again must not call the provider again"
    );
    assert_eq!(attempt_statuses(&again), ["completed"]);
}

/// The worker serves health and readiness only: the route a broker used to
/// push work to is gone, so nothing can hand it work over HTTP.
#[tokio::test]
async fn the_worker_serves_no_work_route() {
    let _guard = lock().lock().await;
    let Some(running) = running("no_route", None, None).await else {
        return;
    };
    let (_, delivery) = accept_turn(&running.database.url, &running.store).await;
    let body = serde_json::to_vec(&delivery).expect("delivery");
    for path in ["/pubsub/chat-turn", "/"] {
        let (status, text) = support::exchange(
            &running.worker_host,
            "POST",
            path,
            &[("content-type", "application/json")],
            Some(&body),
        )
        .await
        .unwrap_or_else(|error| panic!("{error}\n{}", running.worker.tail()));
        assert_eq!(status, 404, "POST {path}: {text}");
    }
    for path in ["/health", "/readyz"] {
        let (status, text) = support::exchange(&running.worker_host, "GET", path, &[], None)
            .await
            .unwrap_or_else(|error| panic!("{error}\n{}", running.worker.tail()));
        assert_eq!(status, 200, "GET {path}: {text}");
    }
}

/// AC2. The worker's listening connection is ended, as a dropped connection
/// or a database failover would end it, and a turn is accepted at once. The
/// worker still claims it within its poll interval, here the hosted 10 s.
#[tokio::test]
async fn a_worker_whose_listener_is_gone_claims_accepted_work_within_the_poll_interval() {
    let _guard = lock().lock().await;
    let Some(running) = running("listener_gone", None, None).await else {
        return;
    };
    // The listener is up once a first turn is answered or polled; either way
    // a listening session exists before it is ended.
    let deadline = Instant::now() + Duration::from_secs(10);
    let ended = loop {
        let ended = running.database.terminate_listeners();
        if ended > 0 {
            break ended;
        }
        assert!(Instant::now() < deadline, "the worker never listened");
        tokio::time::sleep(Duration::from_millis(50)).await;
    };
    assert_eq!(ended, 1, "one worker, one listening session");
    let accepted = Instant::now();
    let (scope, _) = accept_turn(&running.database.url, &running.store).await;
    let state = wait_assistant(&running.store, &scope, &running.worker).await;
    let waited = accepted.elapsed();
    println!("listener ended; accepted turn answered after {waited:?} (poll interval 10 s)");
    assert_eq!(attempt_statuses(&state), ["completed"]);
    assert!(
        waited < Duration::from_secs(11),
        "the turn waited {waited:?}, longer than the poll interval"
    );
    assert!(
        running
            .worker
            .tail()
            .contains("physician work listener disconnected"),
        "the worker never noticed its listener was gone\n{}",
        running.worker.tail()
    );
}

/// AC2, the poll on its own. Work becomes due with no notification at all: a
/// worker claimed the turn and died, and its lease ran out. Only the timed
/// read of the due work can find it.
#[tokio::test]
async fn work_that_becomes_due_unannounced_is_found_by_the_poll() {
    let _guard = lock().lock().await;
    let Some(database) = support::Database::create("poll_only").await else {
        return;
    };
    let store = Store::connect(&database.url).await.expect("runtime store");
    let (scope, delivery) = accept_turn(&database.url, &store).await;
    // The dead worker's claim. Nothing announces its lease running out.
    let lease = Duration::from_secs(3);
    store
        .claim_delivery(&delivery, lease.as_secs() as i64)
        .await
        .expect("claim")
        .expect("the accepted turn is claimable");
    let claimed = Instant::now();
    let home = std::env::temp_dir().join(format!(
        "physician-worker-poll-{}",
        uuid::Uuid::new_v4().simple()
    ));
    std::fs::create_dir_all(&home).expect("home");
    let port = support::free_port();
    let listen = format!("127.0.0.1:{port}");
    let poll = Duration::from_millis(500);
    let poll_ms = poll.as_millis().to_string();
    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", listen.as_str()),
            ("PHYSICIAN_WORKER_FIXED_TEXT", support::FIXED_TEXT),
            ("PHYSICIAN_WORKER_POLL_MS", poll_ms.as_str()),
        ],
        &home,
    );
    support::wait_ready(&mut worker, port).await;
    // The worker's first reads of the due work fall inside the live lease.
    assert!(claimed.elapsed() < lease, "the worker started too late");
    assert_eq!(
        store.delivery_state(&delivery).await.expect("state"),
        DeliveryState::Claimed
    );
    let state = wait_assistant(&store, &scope, &worker).await;
    let waited = claimed.elapsed();
    println!(
        "unannounced work answered {waited:?} after its {lease:?} lease began (poll {poll:?})"
    );
    assert_eq!(attempt_statuses(&state), ["interrupted", "completed"]);
    // The database clock counts whole seconds, so allow one beyond the lease.
    assert!(
        waited < lease + Duration::from_secs(1) + poll * 2,
        "the poll took {waited:?} to find work due after {lease:?}"
    );
    worker.kill_now();
    let _ = std::fs::remove_dir_all(&home);
}

#[tokio::test]
async fn a_failed_attempt_is_retried_and_links_to_the_accepting_request() {
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
    let (scope, _) = accept_from(
        &running.database.url,
        &running.store,
        "Hello",
        Some(TraceContext {
            traceparent: "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01".into(),
            tracestate: Some("dd=s:1".into()),
        }),
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
        "the first attempt should fail before the retry succeeds: {:?}",
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
        "the retry should complete an attempt"
    );
    assert_eq!(state.attempts.len(), 2);
    let first = &state.attempts[0];
    let second = &state.attempts[1];
    assert_ne!(
        first.id, second.id,
        "the retry claims a new canonical attempt"
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
        let accepting_trace =
            opentelemetry::trace::TraceId::from_hex("0af7651916cd43dd8448eb211c80319c")
                .unwrap()
                .to_bytes();
        // Every attempt is its own trace and links to the request that
        // accepted the turn; one accepted input can have several attempts.
        assert_ne!(
            attempt.trace_id, accepting_trace,
            "attempt span: {attempt:?}"
        );
        assert_eq!(
            trace_attribute(attempt, "coda.trace_relation").as_deref(),
            Some("link")
        );
        assert_eq!(attempt.links.len(), 1);
        assert_eq!(attempt.links[0].trace_id, accepting_trace);
        assert_eq!(
            attempt.links[0].span_id,
            opentelemetry::trace::SpanId::from_hex("b7ad6b7169203331")
                .unwrap()
                .to_bytes()
        );
        assert_eq!(attempt.links[0].trace_state, "dd=s:1");
        assert_eq!(
            trace_attribute(attempt, "messaging.system").as_deref(),
            Some("postgresql")
        );
        assert_eq!(
            trace_attribute(attempt, "coda.work_id").as_deref(),
            Some(saved.work.as_str())
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
        spans.iter().all(|span| !span.name.contains("pubsub")),
        "no work arrives over HTTP"
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
    let (simulated_scope, _) = accept_in(
        url,
        &running.store,
        "Rehearse",
        Some(&rehearsal),
        Some("drill.rehearse"),
    )
    .await;
    let (real_scope, _) =
        accept_in(url, &running.store, "Hello", None, Some("drill.rehearse")).await;
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
        false,
        &[
            ("CODA_HOSTED_PROFILE", "staging-preview"),
            ("PHYSICIAN_SIMULATION_PRACTICES", rehearsal.as_str()),
            ("PHYSICIAN_WORKER_FIXED_TEXT", reply.as_str()),
            ("CODA_TELEMETRY_CONTENT", "full"),
            ("OTEL_EXPORTER_OTLP_ENDPOINT", otlp.as_str()),
            ("OTEL_BSP_SCHEDULE_DELAY", "50"),
        ],
    )
    .await
    else {
        return;
    };
    let (scope, _) = accept_in(
        &running.database.url,
        &running.store,
        &format!("Rehearse {prompt_canary}"),
        Some(&rehearsal),
        Some("drill.full-path"),
    )
    .await;
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

fn attempt_statuses(state: &Conversation) -> Vec<&str> {
    state
        .attempts
        .iter()
        .map(|attempt| attempt.status.as_str())
        .collect()
}

/// One worker, both kinds of turn: `[fail:1]` fails once and the retry
/// answers; a plain turn on the same worker never fails.
#[tokio::test]
async fn fail_marker_fails_only_the_marked_message() {
    let _guard = lock().lock().await;
    let Some(running) = running_with("marker", None, None, true, &[]).await else {
        return;
    };
    let (plain_scope, _) = accept_turn(&running.database.url, &running.store).await;
    let state = wait_assistant(&running.store, &plain_scope, &running.worker).await;
    assert_eq!(state.status, "completed");
    assert_eq!(attempt_statuses(&state), ["completed"]);

    let (marked_scope, _) =
        accept_text(&running.database.url, &running.store, "Retry [fail:1]").await;
    let state = wait_assistant(&running.store, &marked_scope, &running.worker).await;
    assert_eq!(state.status, "completed");
    assert_eq!(assistant_texts(&state), [support::FIXED_TEXT]);
    assert_eq!(attempt_statuses(&state), ["failed", "completed"]);

    let (again_scope, _) = accept_turn(&running.database.url, &running.store).await;
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
    // The retry waits, so the restart lands between the first attempt and
    // the second instead of killing the second mid-run.
    let wait = [("PHYSICIAN_WORKER_RETRY_SECONDS", "3")];
    let Some(mut running) = running_with("marker_restart", None, None, true, &wait).await else {
        return;
    };
    let (scope, _) = accept_text(&running.database.url, &running.store, "Restart [fail:2]").await;
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
async fn the_fifth_failure_ends_the_turn_and_nothing_starts_it_again() {
    let _guard = lock().lock().await;
    let Some(running) = running("gave_up", Some(100), None).await else {
        return;
    };
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;

    let deadline = Instant::now() + Duration::from_secs(60);
    let state = loop {
        let state = running
            .store
            .read(&scope)
            .await
            .expect("read")
            .expect("conversation");
        if state.status == "failed" {
            break state;
        }
        assert!(
            Instant::now() < deadline,
            "the turn never ended as failed: {:?}\n{}",
            attempt_statuses(&state),
            running.worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    };
    assert_eq!(attempt_statuses(&state), ["failed"; 5]);
    assert!(
        assistant_texts(&state).is_empty(),
        "a turn that gave up must not publish an assistant reply"
    );
    assert_eq!(
        running
            .store
            .delivery_state(&delivery)
            .await
            .expect("state"),
        DeliveryState::Failed
    );
    // Neither the poll nor another announcement starts a sixth attempt.
    let start = Scope {
        practice: scope.practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    let due = running
        .store
        .dispatch_due(100, Some(&start))
        .await
        .expect("due work");
    assert!(
        !due.deliveries.contains(&delivery),
        "a turn that gave up is still due"
    );
    announce_again(&running.database.url, &delivery).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let later = running
        .store
        .read(&scope)
        .await
        .expect("read")
        .expect("conversation");
    assert_eq!(later.status, "failed");
    assert_eq!(later.attempts.len(), 5);
}

/// AC3. Hosted pacing: each failure makes the turn due later than the last,
/// 10 s then 20 s, and the worker does not run it before then.
#[tokio::test]
async fn failed_attempts_wait_longer_each_time() {
    let _guard = lock().lock().await;
    let Some(database) = support::Database::create("backoff").await else {
        return;
    };
    let store = Store::connect(&database.url).await.expect("runtime store");
    let home = std::env::temp_dir().join(format!(
        "physician-worker-backoff-{}",
        uuid::Uuid::new_v4().simple()
    ));
    std::fs::create_dir_all(&home).expect("home");
    let port = support::free_port();
    let listen = format!("127.0.0.1:{port}");
    // No retry setting: this worker waits the hosted 10 s, 20 s, 40 s.
    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", listen.as_str()),
            ("PHYSICIAN_WORKER_FIXED_TEXT", support::FIXED_TEXT),
            ("PHYSICIAN_WORKER_FIXED_FAILURES", "2"),
        ],
        &home,
    );
    support::wait_ready(&mut worker, port).await;
    let (scope, _) = accept_turn(&database.url, &store).await;
    let started = Instant::now();
    let mut seen = vec![];
    let deadline = Instant::now() + Duration::from_secs(60);
    let state = loop {
        let state = store.read(&scope).await.expect("read").expect("row");
        while seen.len() < state.attempts.len() {
            seen.push(started.elapsed());
        }
        if state.status == "completed" {
            break state;
        }
        assert!(
            Instant::now() < deadline,
            "the turn never completed: {:?}\n{}",
            attempt_statuses(&state),
            worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    };
    assert_eq!(attempt_statuses(&state), ["failed", "failed", "completed"]);
    let first_wait = seen[1] - seen[0];
    let second_wait = seen[2] - seen[1];
    println!("attempts began at {seen:?}: waits of {first_wait:?} then {second_wait:?}");
    // The database clock counts whole seconds; a poll may add its interval.
    for (wait, seconds) in [(first_wait, 10), (second_wait, 20)] {
        assert!(
            wait >= Duration::from_secs(seconds - 1),
            "retried after {wait:?}, before its {seconds} s delay"
        );
        assert!(
            wait < Duration::from_secs(seconds + 11),
            "retried after {wait:?}, long after its {seconds} s delay"
        );
    }
    worker.kill_now();
    let _ = std::fs::remove_dir_all(&home);
}

#[tokio::test]
async fn concurrent_turns_on_one_worker_run_side_by_side() {
    let _guard = lock().lock().await;
    let delay = Duration::from_secs(2);
    let Some(running) = running("parallel", None, Some(delay.as_millis() as u64)).await else {
        return;
    };
    let started = Instant::now();
    let (first_scope, _) = accept_turn(&running.database.url, &running.store).await;
    let (second_scope, _) = accept_turn(&running.database.url, &running.store).await;
    for scope in [&first_scope, &second_scope] {
        let state = wait_assistant(&running.store, scope, &running.worker).await;
        assert_eq!(assistant_texts(&state), [support::FIXED_TEXT]);
    }
    let elapsed = started.elapsed();
    assert!(
        elapsed >= delay,
        "both turns answered after {elapsed:?}, before a {delay:?} turn could finish"
    );
    assert!(
        elapsed < delay * 2 - Duration::from_millis(500),
        "two {delay:?} turns took {elapsed:?}; the worker ran them one after the other"
    );
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
    let (scope, _) = accept_turn(&running.database.url, &running.store).await;
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
