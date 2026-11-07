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
    let auth = PostgresAuthStore::connect_without_migrations(url)
        .await
        .expect("auth store");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or_default();
    let unique = uuid::Uuid::new_v4().simple().to_string();
    let email = format!("worker-{unique}@example.test");
    let practice = format!("practice-{unique}");
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
    let Some(running) = running("retry", Some(1), None).await else {
        return;
    };
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;
    running.emulator.publish(&delivery).await;
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
            [
                "search_patient_sources",
                "read_patient_sources",
                "skill_view"
            ]
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

#[tokio::test]
async fn chart_reads_and_exact_citations_survive_readback_and_duplicate_delivery() {
    let _guard = lock().lock().await;
    let Some(running) = running_with(
        "chart_citations",
        None,
        None,
        false,
        &[(
            "PHYSICIAN_WORKER_FIXED_TEXT",
            "Controlled source read {{patient_source}}\n\n[[source_summaries]]\n{{patient_source_id}}: Records potassium 4.1.\n[[/source_summaries]]",
        )],
    )
    .await
    else {
        return;
    };
    let (scope, delivery) = accept_turn(&running.database.url, &running.store).await;
    let content = "{\"potassium\":4.1}";
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
    assert!(!answer.text.contains("source_summaries"), "{}", answer.text);
    assert!(
        answer
            .text
            .starts_with("Controlled source read [[patient_source:")
    );
    assert_eq!(citation.summary.as_deref(), Some("Records potassium 4.1."));
    assert_eq!(
        citation.id,
        local_product::physician_sources::citation_id(&source)
    );
    assert!(!serde_json::to_string(&state).unwrap().contains("other-lab"));
    let original = serde_json::to_value(&state.messages).unwrap();
    source.revision = "r2".into();
    source.content = "{\"potassium\":6.8}".into();
    source.sha256 = local_product::sha256_hex(source.content.as_bytes());
    running.store.record_source(&source).await.unwrap();
    running.emulator.publish(&delivery).await;
    let fresh = Store::connect(&running.database.url).await.unwrap();
    let saved = fresh.read(&scope).await.unwrap().unwrap();
    assert_eq!(serde_json::to_value(saved.messages).unwrap(), original);
    assert_eq!(saved.attempts.len(), 1);
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
        false,
        &[
            ("PHYSICIAN_WORKER_FIXED_TEXT", ""),
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
            && (state.messages.iter().any(|m| m.role == "assistant") || state.status == "failed")
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
    let answer = state.messages.iter().find(|m| m.role == "assistant");
    let evidence = serde_json::json!({
        "instructions_version": local_product::physician_chat::INSTRUCTIONS_VERSION,
        "status": state.status,
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
