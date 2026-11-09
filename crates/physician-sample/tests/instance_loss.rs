//! Instances that die mid-turn, against Valkey and a disposable Postgres.
//!
//! A site instance accepts a turn while no worker is running, and dies. A
//! worker that starts later finds the turn in Postgres and answers; no one
//! reloads or presses anything. A worker told to stop hands its turn back and
//! another worker runs it at once; a worker that is killed leaves its turn to
//! another worker once its lease runs out. A turn longer than the lease stays
//! with the worker that keeps renewing it. A turn whose worker leases all run
//! out ends as failed, and open pages are told to refetch.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use local_product::runtime::delivery::{Delivery, DeliveryState};
use local_product::runtime::{Conversation, MAX_FAILED_ATTEMPTS, Store};
use physician_auth::AuthStore;
use physician_bus::{Bus, Event};
use serde_json::Value;

#[path = "scale/mod.rs"]
mod support;

fn lock() -> &'static tokio::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

struct Home(PathBuf);

impl Home {
    fn new(label: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "physician-scale-{label}-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(&path).expect("home");
        Self(path)
    }
}

impl Drop for Home {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// A worker on its own port, ready to serve. `extra` adds to its settings.
async fn spawn_worker(
    name: &str,
    home: &Home,
    database: &str,
    redis: Option<&str>,
    extra: &[(&str, &str)],
) -> support::Process {
    let port = support::free_port();
    let listen = format!("127.0.0.1:{port}");
    let mut env = vec![
        ("PHYSICIAN_DATABASE_URL", database),
        ("PHYSICIAN_WORKER_LISTEN_ADDR", listen.as_str()),
        ("PHYSICIAN_WORKER_FIXED_TEXT", support::FIXED_TEXT),
    ];
    if let Some(redis) = redis {
        env.push(("PHYSICIAN_REDIS_URL", redis));
    }
    env.extend_from_slice(extra);
    let mut worker = support::Process::spawn(name, support::worker_bin(), &[], &env, &home.0);
    support::wait_ready(&mut worker, port).await;
    worker
}

fn spawn_site(
    name: &str,
    home: &Home,
    database: &str,
    redis: &str,
    listen: &str,
) -> support::Process {
    support::Process::spawn(
        name,
        support::site_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database),
            ("PHYSICIAN_REDIS_URL", redis),
            support::DURABLE,
            ("PHYSICIAN_AUTH", "required"),
            ("PHYSICIAN_LISTEN_ADDR", listen),
        ],
        &home.0,
    )
}

fn port(listen: &str) -> u16 {
    listen
        .rsplit_once(':')
        .and_then(|(_, port)| port.parse().ok())
        .expect("listen port")
}

fn assistant_texts(snapshot: &Value) -> Vec<&str> {
    snapshot["conversation"]
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter(|item| item["kind"] != "mechanical")
                .filter_map(|item| item["text"].as_str())
                .collect()
        })
        .unwrap_or_default()
}

fn statuses(state: &Conversation) -> Vec<&str> {
    state
        .attempts
        .iter()
        .map(|attempt| attempt.status.as_str())
        .collect()
}

/// Polls the saved conversation until `done`, and returns it.
async fn wait_state(
    store: &Store,
    delivery: &Delivery,
    within: Duration,
    what: &str,
    logs: &dyn Fn() -> String,
    done: impl Fn(&Conversation) -> bool,
) -> Conversation {
    let deadline = Instant::now() + within;
    loop {
        let state = store
            .read(&delivery.scope)
            .await
            .expect("read")
            .expect("conversation");
        if done(&state) {
            return state;
        }
        assert!(
            Instant::now() < deadline,
            "{what}: {:?}\n{}",
            statuses(&state),
            logs()
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

/// Work accepted while no worker runs, by a site instance that then dies,
/// must still finish: the notification reached no one, and the worker that
/// starts later reads the due work from Postgres.
#[tokio::test]
async fn a_turn_accepted_by_a_dead_site_instance_with_no_worker_is_finished_by_a_later_worker() {
    let _guard = lock().lock().await;
    let Some(database) = support::Database::create("instance_loss").await else {
        return;
    };
    support::staging_account(&database.url).await;
    let home = Home::new("instance-loss");
    let valkey = support::Valkey::start().await;
    let redis = valkey.url();

    let a_listen = format!("127.0.0.1:{}", support::free_port());
    let mut site_a = spawn_site("site-a", &home, &database.url, &redis, &a_listen);
    support::wait_ready(&mut site_a, port(&a_listen)).await;

    let session = support::sign_in(&a_listen).await;
    let workspace = physician_sample::WORKSPACE_IDS[0];
    let chat = format!("chat-{}", uuid::Uuid::new_v4().simple());
    let snapshot_path = format!("/patient-workspaces/{workspace}/chats/{chat}/snapshot");
    let (status, body) = support::post(
        &a_listen,
        "/commands",
        Some(&session),
        &support::submit(workspace, &chat, "Hello"),
    )
    .await;
    assert_eq!(status, 200, "{body}\n{}", site_a.tail());
    assert_eq!(body["outcome"], "accepted", "{body}");

    // Instance A dies. Its acceptance is saved; nothing heard it announced.
    site_a.kill_now();
    let store = Store::connect(&database.url).await.expect("runtime store");
    let delivery = {
        let start = local_product::runtime::Scope {
            practice: physician_sample::STAGING_PRACTICE_ID.to_owned(),
            physician: String::new(),
            workspace: String::new(),
            chat: String::new(),
        };
        let due = store
            .dispatch_due(100, Some(&start))
            .await
            .expect("dispatch_due");
        let mut matching = due
            .deliveries
            .into_iter()
            .filter(|delivery| delivery.scope.chat == chat);
        let delivery = matching
            .next()
            .expect("the dead instance's turn should be due");
        assert!(matching.next().is_none());
        delivery
    };
    assert_eq!(
        store.delivery_state(&delivery).await.expect("state"),
        DeliveryState::Due
    );

    // Instance B serves the page, which reconnects; it sends nothing and
    // presses nothing. B itself runs no work and republishes nothing.
    let b_listen = format!("127.0.0.1:{}", support::free_port());
    let mut site_b = spawn_site("site-b", &home, &database.url, &redis, &b_listen);
    support::wait_ready(&mut site_b, port(&b_listen)).await;
    let tails = |site: &support::Process, worker: &support::Process| {
        format!("{}\n{}", site.tail(), worker.tail())
    };
    let mut sse = support::Sse::open(
        &b_listen,
        &format!("/patient-workspaces/events?patient_workspace_id={workspace}"),
        &session,
    )
    .await
    .unwrap_or_else(|error| panic!("{error}\n{}", site_b.tail()));
    tokio::time::sleep(Duration::from_secs(1)).await;
    assert_eq!(
        store.delivery_state(&delivery).await.expect("state"),
        DeliveryState::Due,
        "a site instance never runs or claims durable work"
    );

    // Subscribe first: invalidations are live notices, not a replay log for
    // pages that were disconnected when the reply landed.
    let started = Instant::now();
    let worker = spawn_worker("worker", &home, &database.url, Some(&redis), &[]).await;

    let deadline = Instant::now() + Duration::from_secs(45);
    let answered = loop {
        let (status, body) = support::get_json(&b_listen, &snapshot_path, &session).await;
        assert_eq!(status, 200, "{body}");
        let snapshot = body["snapshot"].clone();
        if assistant_texts(&snapshot).contains(&support::FIXED_TEXT) {
            break snapshot;
        }
        assert!(
            Instant::now() < deadline,
            "the later worker never finished the dead instance's turn: {snapshot}\n{}",
            tails(&site_b, &worker)
        );
        tokio::time::sleep(Duration::from_millis(200)).await;
    };
    println!(
        "turn accepted with no worker answered {:?} after a worker began starting",
        started.elapsed()
    );
    assert_eq!(assistant_texts(&answered), ["Hello", support::FIXED_TEXT]);
    let state = store
        .read(&delivery.scope)
        .await
        .expect("read")
        .expect("conversation");
    assert_eq!(state.status, "completed");
    assert_eq!(
        statuses(&state),
        ["completed"],
        "the turn is found once and never run twice"
    );
    assert_eq!(
        store.delivery_state(&delivery).await.expect("state"),
        DeliveryState::Settled
    );
    sse.wait_for("event: workspace_changed", Duration::from_secs(10))
        .await
        .unwrap_or_else(|error| {
            panic!(
                "the page on instance B was never told the reply landed: {error}\n{}",
                tails(&site_b, &worker)
            )
        });
    drop(sse);
    let _reconnected = support::Sse::open(
        &b_listen,
        &format!("/patient-workspaces/events?patient_workspace_id={workspace}"),
        &session,
    )
    .await
    .expect("reconnect after completion");
    let (status, snapshot) = support::get_json(&b_listen, &snapshot_path, &session).await;
    assert_eq!(status, 200);
    assert_eq!(
        assistant_texts(&snapshot["snapshot"]),
        ["Hello", support::FIXED_TEXT],
        "a reconnect reads the completed reply without waiting for a past invalidation"
    );
}

/// How the first worker is stopped mid-turn.
#[derive(Clone, Copy, Debug)]
enum Stop {
    /// SIGTERM, as Cloud Run stops an instance: the worker hands its claim
    /// back.
    Terminate,
    /// SIGKILL, as a crash: the claim is left to run out.
    Kill,
}

/// Worker A is stopped while it runs a turn; worker B, already serving,
/// finishes it. Returns how long after the signal B claimed the turn.
async fn another_worker_finishes(label: &str, stop: Stop, pace: &[(&str, &str)]) -> Duration {
    let database = support::Database::create(label)
        .await
        .expect("the caller checked the gate");
    let home = Home::new(label);
    let store = Store::connect(&database.url).await.expect("runtime store");
    // A's turn outlasts the test; B answers at once.
    let mut slow = vec![("PHYSICIAN_WORKER_FIXED_DELAY_MS", "600000")];
    slow.extend_from_slice(pace);
    let mut a = spawn_worker("worker-a", &home, &database.url, None, &slow).await;
    let delivery = accepted(&database.url, &store).await;
    let logs = |a: &support::Process| a.tail();
    let first = wait_state(
        &store,
        &delivery,
        Duration::from_secs(15),
        "worker A never claimed the turn",
        &|| logs(&a),
        |state| state.status == "active",
    )
    .await;
    assert_eq!(statuses(&first), ["active"]);
    // B starts while A holds the turn, and must not take it.
    let b = spawn_worker("worker-b", &home, &database.url, None, pace).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let held = store.read(&delivery.scope).await.unwrap().unwrap();
    assert_eq!(statuses(&held), ["active"], "B took a turn A still holds");

    let signalled = Instant::now();
    match stop {
        Stop::Terminate => a.terminate(),
        Stop::Kill => a.kill_hard(),
    }
    let both = || format!("{}\n{}", a.tail(), b.tail());
    let taken = wait_state(
        &store,
        &delivery,
        Duration::from_secs(90),
        "worker B never claimed the stopped worker's turn",
        &both,
        |state| state.attempts.len() == 2,
    )
    .await;
    let recovered = signalled.elapsed();
    assert_eq!(taken.attempts[0].status, "interrupted");
    let done = wait_state(
        &store,
        &delivery,
        Duration::from_secs(15),
        "worker B never finished the turn",
        &both,
        |state| state.status == "completed",
    )
    .await;
    assert_eq!(statuses(&done), ["interrupted", "completed"]);
    // A handed-back turn is not a failure. A lease that ran out is one: a
    // turn that keeps killing its worker must still reach the failure bound.
    let failures = match stop {
        Stop::Terminate => 0,
        Stop::Kill => 1,
    };
    assert_eq!(done.failures, failures);
    assert_eq!(
        done.messages
            .iter()
            .filter(|message| message.role == "assistant")
            .map(|message| message.text.as_str())
            .collect::<Vec<_>>(),
        [support::FIXED_TEXT]
    );
    if matches!(stop, Stop::Terminate) {
        // A exits on its own after handing the claim back.
        let deadline = Instant::now() + Duration::from_secs(10);
        while a.exited().is_none() {
            assert!(
                Instant::now() < deadline,
                "worker A never exited\n{}",
                a.tail()
            );
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        assert_eq!(a.exited(), Some(0), "{}", a.tail());
    }
    println!("{label}: {stop:?} -> another worker claimed the turn after {recovered:?}");
    recovered
}

/// AC4. SIGTERM: the stopping worker hands the turn back and announces it,
/// so the other worker claims it at once, well inside the lease and without
/// waiting for a poll. Both workers use the hosted 60 s lease and 10 s poll.
#[tokio::test]
async fn a_turn_whose_worker_is_told_to_stop_is_taken_by_another_worker_at_once() {
    let _guard = lock().lock().await;
    if support::gate_url().is_none() {
        return;
    }
    let recovered = another_worker_finishes("sigterm", Stop::Terminate, &[]).await;
    assert!(
        recovered < Duration::from_secs(3),
        "a released turn waited {recovered:?} for another worker"
    );
}

/// AC4. SIGKILL: nothing hands the turn back, so the other worker's poll
/// takes it once the lease runs out. A 5 s lease and a 500 ms poll keep the
/// run short; `..._within_the_hosted_lease` uses the hosted values.
#[tokio::test]
async fn a_turn_whose_worker_is_killed_is_taken_by_another_worker_when_its_lease_runs_out() {
    let _guard = lock().lock().await;
    if support::gate_url().is_none() {
        return;
    }
    let pace = [
        ("PHYSICIAN_WORKER_LEASE_SECONDS", "5"),
        ("PHYSICIAN_WORKER_POLL_MS", "500"),
    ];
    let recovered = another_worker_finishes("sigkill", Stop::Kill, &pace).await;
    // Renewed at most a third of a lease before the kill, the claim has more
    // than three seconds left; it never outlives the lease, one whole-second
    // tick of the database clock, and two polls.
    assert!(
        recovered > Duration::from_secs(2),
        "the turn was taken after {recovered:?}, while its lease was live"
    );
    assert!(
        recovered < Duration::from_secs(5 + 1 + 1),
        "the turn waited {recovered:?}, longer than its 5 s lease"
    );
}

/// AC4 at hosted pacing: a killed worker's turn is taken within the 60 s
/// lease plus one 10 s poll. Slow, so it runs when asked for.
#[tokio::test]
#[ignore = "takes about a minute: waits out the hosted 60 s lease"]
async fn a_turn_whose_worker_is_killed_is_taken_within_the_hosted_lease() {
    let _guard = lock().lock().await;
    if support::gate_url().is_none() {
        return;
    }
    let recovered = another_worker_finishes("sigkill_hosted", Stop::Kill, &[]).await;
    assert!(
        recovered > Duration::from_secs(39),
        "the turn was taken after {recovered:?}, while its lease was live"
    );
    assert!(
        recovered < Duration::from_secs(60 + 1 + 10 + 1),
        "the turn waited {recovered:?}, longer than the lease and one poll"
    );
}

/// A turn that takes longer than the lease stays with its worker, which
/// renews the lease as it runs: a second worker polling throughout never
/// takes it, and it completes in one attempt.
#[tokio::test]
async fn a_turn_longer_than_its_lease_is_kept_by_the_worker_that_renews_it() {
    let _guard = lock().lock().await;
    let Some(database) = support::Database::create("renewed").await else {
        return;
    };
    let home = Home::new("renewed");
    let store = Store::connect(&database.url).await.expect("runtime store");
    let pace = [
        ("PHYSICIAN_WORKER_LEASE_SECONDS", "3"),
        ("PHYSICIAN_WORKER_POLL_MS", "300"),
    ];
    let mut slow = vec![("PHYSICIAN_WORKER_FIXED_DELAY_MS", "8000")];
    slow.extend_from_slice(&pace);
    let a = spawn_worker("worker-a", &home, &database.url, None, &slow).await;
    let delivery = accepted(&database.url, &store).await;
    let first = wait_state(
        &store,
        &delivery,
        Duration::from_secs(15),
        "worker A never claimed the turn",
        &|| a.tail(),
        |state| state.status == "active",
    )
    .await;
    let claimed_until = first.attempts[0].expires;
    let b = spawn_worker("worker-b", &home, &database.url, None, &pace).await;
    let both = || format!("{}\n{}", a.tail(), b.tail());
    let done = wait_state(
        &store,
        &delivery,
        Duration::from_secs(30),
        "the long turn never completed",
        &both,
        |state| state.status == "completed",
    )
    .await;
    assert_eq!(
        statuses(&done),
        ["completed"],
        "an 8 s turn on a 3 s lease must stay with its worker"
    );
    assert!(
        done.attempts[0].expires >= claimed_until + 4,
        "the lease was never renewed: {} then {}",
        claimed_until,
        done.attempts[0].expires
    );
}

/// A turn whose worker leases all run out (each worker died mid-turn) ends
/// as failed. The worker that finds it so records the last failure and
/// announces the change, so a held page shows "couldn't finish".
#[tokio::test]
async fn exhausted_leases_fail_the_turn_and_announce_it() {
    let _guard = lock().lock().await;
    let Some(database) = support::Database::create("lease_exhausted").await else {
        return;
    };
    let home = Home::new("lease-exhausted");
    let valkey = support::Valkey::start().await;
    let redis = valkey.url();
    let store = Store::connect(&database.url).await.expect("runtime store");
    let delivery = accepted(&database.url, &store).await;

    // Each claim stands for a worker instance that took the turn and died
    // before it finished: nothing returns it, and its one-second lease runs
    // out. The worker's own lease is 60 s; only the length differs.
    for _ in 0..MAX_FAILED_ATTEMPTS {
        let claim = store
            .claim_delivery(&delivery, 1)
            .await
            .expect("claim")
            .expect("the turn should be claimable while its leases remain");
        assert_eq!(claim.scope, delivery.scope);
        // The database clock counts whole seconds and a lease ends once
        // `expires <= now`, so one second always ends it; 200 ms is margin.
        tokio::time::sleep(Duration::from_millis(1200)).await;
    }
    assert_eq!(
        store.delivery_state(&delivery).await.expect("state"),
        DeliveryState::Due,
        "an expired lease is still runnable until a claim records it"
    );

    // A page on another instance listens for this workspace.
    let bus = physician_bus::RedisBus::connect(&redis, None)
        .await
        .expect("valkey bus");
    let mut events = bus.subscribe();
    let deadline = Instant::now() + Duration::from_secs(10);
    while !bus.live() {
        assert!(Instant::now() < deadline, "test bus never subscribed");
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    // The worker's first read of the due work finds the last lease run out,
    // and its claim records the fifth failure.
    let worker = spawn_worker("worker", &home, &database.url, Some(&redis), &[]).await;
    let state = wait_state(
        &store,
        &delivery,
        Duration::from_secs(15),
        "the turn never ended as failed",
        &|| worker.tail(),
        |state| state.status == "failed",
    )
    .await;
    assert!(
        state
            .messages
            .iter()
            .all(|message| message.role != "assistant"),
        "the model never ran"
    );
    assert_eq!(
        store.delivery_state(&delivery).await.expect("state"),
        DeliveryState::Failed
    );
    let expected = Event::WorkspaceChanged {
        workspace_id: delivery.scope.workspace.clone(),
    };
    loop {
        let event = tokio::time::timeout(Duration::from_secs(10), events.recv())
            .await
            .unwrap_or_else(|_| {
                panic!("no workspace change for the failed turn\n{}", worker.tail())
            })
            .expect("bus");
        if event == expected {
            break;
        }
    }
    // Nothing finds it again: Try again is the only way back.
    let due = store.dispatch_due(100, None).await.expect("due work");
    assert!(!due.deliveries.contains(&delivery));
}

/// One authorized, accepted turn saved straight to the runtime store.
async fn accepted(url: &str, store: &Store) -> Delivery {
    let auth = physician_auth::PostgresAuthStore::connect_without_migrations(url)
        .await
        .expect("auth store");
    let now = support::unix_now();
    let unique = uuid::Uuid::new_v4().simple().to_string();
    let email = format!("lease-{unique}@example.test");
    let practice = format!("practice-{unique}");
    let physician = format!("physician-{unique}");
    let workspace = format!("workspace-{unique}");
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
    let token = auth.begin_sign_in(&email, now).await.expect("token");
    let (identity, session) = auth
        .complete_sign_in(&email, &token, now)
        .await
        .expect("session");
    auth.register_patient(&identity.practice_id, &workspace)
        .await
        .unwrap();
    auth.set_membership(&identity, &workspace, physician_auth::Membership::Allowed)
        .await
        .unwrap();
    let scope = local_product::runtime::Scope {
        practice: identity.practice_id,
        physician: identity.physician_id,
        workspace,
        chat: format!("chat-{unique}"),
    };
    let key = format!("input-{unique}");
    store
        .accept_authorized(
            &scope,
            &local_product::runtime::Input {
                key: key.clone(),
                text: "Hello".into(),
                timezone: "America/New_York".into(),
                scenario: None,
            },
            physician_auth::PostgresAuthStore::session_reference(&session),
            None,
        )
        .await
        .expect("accept");
    Delivery::chat_turn(scope, key)
}
