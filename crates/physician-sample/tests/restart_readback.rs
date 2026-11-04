//! Accepted physician work reads back after the site and the worker restart.
//!
//! One site and one worker share a disposable Postgres, Valkey and the Pub/Sub
//! emulator. One chat gets its reply; the other's message is marked to fail
//! five times, so it ends as "couldn't finish". Both processes then restart.
//! The same session reads back the same conversations, run and chat list,
//! and the saved records are unchanged. After the restart, the same submit,
//! a redelivered wakeup for the answered turn, and the same Try again each
//! leave one accepted command and one effect.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use local_product::runtime::delivery::Delivery;
use local_product::runtime::{Conversation, Scope, Store};
use serde_json::{Value, json};

#[path = "scale/mod.rs"]
mod support;
use support::{get_json, sign_in};

const FAILING: &str = "Check the labs [fail:5]";

struct Home(PathBuf);

impl Drop for Home {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[tokio::test]
async fn accepted_work_reads_back_after_a_restart_and_repeats_take_effect_once() {
    let Some(database) = support::Database::create("restart_readback").await else {
        return;
    };
    support::staging_account(&database.url).await;
    let store = Store::connect(&database.url).await.expect("runtime store");

    let home = Home(std::env::temp_dir().join(format!(
        "physician-scale-restart-readback-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4().simple()
    )));
    std::fs::create_dir_all(&home.0).expect("home");
    let valkey = support::Valkey::start().await;
    let redis = valkey.url();
    let worker_port = support::free_port();
    let site_port = support::free_port();
    let emulator = support::Emulator::start().await;
    emulator.provision(worker_port).await;

    let topic = support::topic();
    let worker_listen = format!("127.0.0.1:{worker_port}");
    let site_listen = format!("127.0.0.1:{site_port}");
    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", worker_listen.as_str()),
            ("PHYSICIAN_WORKER_FIXED_TEXT", support::FIXED_TEXT),
            // `[fail:5]` fails the first five saved attempts on that message,
            // counted from Postgres, so a restarted worker agrees.
            ("PHYSICIAN_WORKER_FIXED_FAIL_MARKER", "1"),
        ],
        &home.0,
    );
    support::wait_ready(&mut worker, worker_port).await;
    let mut site = support::Process::spawn(
        "site",
        support::site_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
            ("GOOGLE_CLOUD_PROJECT", support::PROJECT),
            ("PHYSICIAN_CHAT_TOPIC", topic.as_str()),
            ("PHYSICIAN_AUTH", "required"),
            ("PHYSICIAN_LISTEN_ADDR", site_listen.as_str()),
        ],
        &home.0,
    );
    support::wait_ready(&mut site, site_port).await;
    let tails = |site: &support::Process, worker: &support::Process| {
        format!("{}\n{}", site.tail(), worker.tail())
    };

    let session = sign_in(&site_listen).await;
    let workspace = physician_sample::WORKSPACE_IDS[0];
    let answered_chat = format!("chat-{}", uuid::Uuid::new_v4().simple());
    let failed_chat = format!("chat-{}", uuid::Uuid::new_v4().simple());
    let scope = |chat: &str| Scope {
        practice: physician_sample::STAGING_PRACTICE_ID.to_owned(),
        physician: support::STAGING_PHYSICIAN.to_owned(),
        workspace: workspace.to_owned(),
        chat: chat.to_owned(),
    };

    // One chat is answered.
    let submit_key = format!("key-{}", uuid::Uuid::new_v4().simple());
    let first_submit = submit(workspace, &answered_chat, "Hello", &submit_key);
    let (status, body) = post_json(&site_listen, &session, &first_submit).await;
    assert_eq!(status, 200, "{body}\n{}", tails(&site, &worker));
    assert_eq!(body["outcome"], "accepted", "{body}");
    wait_snapshot(
        &site_listen,
        &session,
        workspace,
        &answered_chat,
        |snapshot| texts(snapshot).contains(&support::FIXED_TEXT),
    )
    .await
    .unwrap_or_else(|last| panic!("no reply: {last}\n{}", tails(&site, &worker)));

    // The other fails on all five deliveries and reaches the dead-letter topic.
    let (status, body) = post_json(
        &site_listen,
        &session,
        &submit(
            workspace,
            &failed_chat,
            FAILING,
            &format!("key-{}", uuid::Uuid::new_v4().simple()),
        ),
    )
    .await;
    assert_eq!(status, 200, "{body}\n{}", tails(&site, &worker));
    assert_eq!(body["outcome"], "accepted", "{body}");
    let deadline = Instant::now() + Duration::from_secs(90);
    loop {
        let forwarded = emulator.pull_dead_letters().await;
        if forwarded.iter().any(|bytes| {
            serde_json::from_slice::<Delivery>(bytes)
                .is_ok_and(|delivery| delivery.scope.chat == failed_chat)
        }) {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "dead-letter topic did not receive the failing turn\n{}\n{}",
            tails(&site, &worker),
            emulator.tail()
        );
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    wait_snapshot(
        &site_listen,
        &session,
        workspace,
        &failed_chat,
        |snapshot| mechanical(snapshot).is_some_and(|line| line["resume_target"].is_object()),
    )
    .await
    .unwrap_or_else(|last| panic!("never failed: {last}\n{}", tails(&site, &worker)));

    let before = Readback::take(
        &site_listen,
        &session,
        workspace,
        &answered_chat,
        &failed_chat,
    )
    .await;
    assert_eq!(texts(&before.answered), ["Hello", support::FIXED_TEXT]);
    assert_eq!(texts(&before.failed), [FAILING]);
    let answered_record = record(&store, &scope(&answered_chat)).await;
    let failed_record = record(&store, &scope(&failed_chat)).await;
    assert_eq!(answered_record.status, "completed");
    assert_eq!(failed_record.status, "failed");
    assert_eq!(failed_record.attempts.len(), 5, "{failed_record:?}");

    // Both processes restart: the next deploy, or a crash and a replacement.
    worker.restart();
    site.restart();
    support::wait_ready(&mut worker, worker_port).await;
    support::wait_ready(&mut site, site_port).await;

    // The same session reads back the same conversations, the failed run
    // Try again names, and the chat list; the saved records match too.
    let after = Readback::take(
        &site_listen,
        &session,
        workspace,
        &answered_chat,
        &failed_chat,
    )
    .await;
    assert_eq!(after, before, "read back after the restart");
    assert_eq!(
        saved(&record(&store, &scope(&answered_chat)).await),
        saved(&answered_record)
    );
    assert_eq!(
        saved(&record(&store, &scope(&failed_chat)).await),
        saved(&failed_record)
    );

    // The same submit after the restart is the same command. It is accepted
    // again and changes nothing.
    let (status, body) = post_json(&site_listen, &session, &first_submit).await;
    assert_eq!(status, 200, "{body}\n{}", tails(&site, &worker));
    assert_eq!(body["outcome"], "accepted", "{body}");

    // The answered turn's wakeup arrives again at the restarted worker: its
    // reply was saved before the restart, so the push is acknowledged and
    // the provider does not run again.
    let wakeup = Delivery::chat_turn(scope(&answered_chat), submit_key.clone());
    let (status, text) = support::exchange(
        &worker_listen,
        "POST",
        "/pubsub/chat-turn",
        &[("content-type", "application/json")],
        Some(&support::push_body(&wakeup, "redelivered-after-restart")),
    )
    .await
    .unwrap_or_else(|error| panic!("{error}\n{}", worker.tail()));
    assert_eq!(status, 204, "a settled turn's redelivery is acked: {text}");
    tokio::time::sleep(Duration::from_secs(1)).await;
    assert_eq!(
        saved(&record(&store, &scope(&answered_chat)).await),
        saved(&answered_record),
        "a repeated submit or wakeup must not add input, attempts or replies"
    );
    let (_, page) = snapshot(&site_listen, &session, workspace, &answered_chat).await;
    assert_eq!(page["snapshot"], before.answered);

    // Try again on the run read back after the restart, pressed twice with
    // the same key: one resumed attempt and one reply.
    let line = mechanical(&after.failed).expect("couldn't finish line");
    let work = line["resume_target"]["work_item_id"]
        .as_str()
        .expect("work_item_id")
        .to_owned();
    let run = line["resume_target"]["expected_run_id"]
        .as_str()
        .expect("expected_run_id")
        .to_owned();
    assert_eq!(
        after.failed["runs"],
        json!([{ "run_id": run, "state": "failed" }])
    );
    let try_again = resume(
        workspace,
        &failed_chat,
        &work,
        &run,
        &format!("resume-{run}"),
    );
    let (status, body) = post_json(&site_listen, &session, &try_again).await;
    assert_eq!(status, 200, "{body}\n{}", tails(&site, &worker));
    assert_eq!(body["outcome"], "accepted", "{body}");
    let (status, body) = post_json(&site_listen, &session, &try_again).await;
    assert_eq!(status, 200, "{body}\n{}", tails(&site, &worker));
    assert_eq!(
        body["outcome"], "accepted",
        "the same press is the same request: {body}"
    );
    let resumed = wait_snapshot(
        &site_listen,
        &session,
        workspace,
        &failed_chat,
        |snapshot| texts(snapshot).contains(&support::FIXED_TEXT),
    )
    .await
    .unwrap_or_else(|last| panic!("Try again got no reply: {last}\n{}", tails(&site, &worker)));
    assert_eq!(texts(&resumed), [FAILING, support::FIXED_TEXT]);
    assert!(mechanical(&resumed).is_none(), "{resumed}");
    assert_eq!(resumed["runs"], json!([]), "{resumed}");

    tokio::time::sleep(Duration::from_secs(1)).await;
    let state = record(&store, &scope(&failed_chat)).await;
    assert_eq!(state.status, "completed");
    assert_eq!(
        state.revision, failed_record.revision,
        "Try again keeps the input"
    );
    assert_eq!(
        state.attempts.len(),
        6,
        "five failed attempts and one resumed attempt: {state:?}"
    );
    let completed: Vec<_> = state
        .attempts
        .iter()
        .filter(|attempt| attempt.status == "completed")
        .collect();
    assert_eq!(completed.len(), 1, "{state:?}");
    assert_eq!(completed[0].predecessor.as_deref(), Some(run.as_str()));
    assert_eq!(
        state
            .messages
            .iter()
            .filter(|message| message.role == "assistant")
            .count(),
        1,
        "{state:?}"
    );
}

/// What the physician's page reads for the two chats and their list.
#[derive(Debug, PartialEq)]
struct Readback {
    answered: Value,
    failed: Value,
    chats: Value,
}

impl Readback {
    async fn take(
        host: &str,
        session: &str,
        workspace: &str,
        answered_chat: &str,
        failed_chat: &str,
    ) -> Self {
        let (status, answered) = snapshot(host, session, workspace, answered_chat).await;
        assert_eq!(status, 200, "{answered}");
        let (status, failed) = snapshot(host, session, workspace, failed_chat).await;
        assert_eq!(status, 200, "{failed}");
        let (status, listed) =
            get_json(host, &format!("/patient-workspaces/{workspace}"), session).await;
        assert_eq!(status, 200, "{listed}");
        let chats = listed["chats"].clone();
        let mut ids: Vec<_> = chats
            .as_array()
            .expect("chats")
            .iter()
            .filter_map(|chat| chat["chat_id"].as_str())
            .collect();
        // Activity is kept in whole seconds, so two chats can tie on it.
        ids.sort_unstable();
        let mut expected = [answered_chat, failed_chat];
        expected.sort_unstable();
        assert_eq!(ids, expected, "{chats}");
        Self {
            answered: answered["snapshot"].clone(),
            failed: failed["snapshot"].clone(),
            chats,
        }
    }
}

async fn record(store: &Store, scope: &Scope) -> Conversation {
    store
        .read(scope)
        .await
        .expect("read conversation")
        .expect("saved conversation")
}

fn saved(conversation: &Conversation) -> Value {
    serde_json::to_value(conversation).expect("conversation json")
}

fn submit(workspace: &str, chat: &str, text: &str, key: &str) -> Value {
    json!({
        "type": "message.submit",
        "idempotency_key": key,
        "payload": {
            "patient_workspace_id": workspace,
            "chat_id": chat,
            "text": text,
            "physician_time_zone": "America/New_York"
        }
    })
}

fn resume(workspace: &str, chat: &str, work: &str, run: &str, key: &str) -> Value {
    json!({
        "type": "work.resume",
        "idempotency_key": key,
        "payload": {
            "patient_workspace_id": workspace,
            "chat_id": chat,
            "work_item_id": work,
            "expected_run_id": run,
        }
    })
}

fn mechanical(snapshot: &Value) -> Option<&Value> {
    snapshot["conversation"]
        .as_array()?
        .iter()
        .find(|item| item["kind"] == "mechanical")
}

fn texts(snapshot: &Value) -> Vec<&str> {
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

async fn snapshot(host: &str, session: &str, workspace: &str, chat: &str) -> (u16, Value) {
    get_json(
        host,
        &format!("/patient-workspaces/{workspace}/chats/{chat}/snapshot"),
        session,
    )
    .await
}

/// Polls the snapshot until `done`; on timeout, returns the last one seen.
async fn wait_snapshot(
    host: &str,
    session: &str,
    workspace: &str,
    chat: &str,
    done: impl Fn(&Value) -> bool,
) -> Result<Value, Value> {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        let (status, body) = snapshot(host, session, workspace, chat).await;
        assert_eq!(status, 200, "{body}");
        let snapshot = body["snapshot"].clone();
        if done(&snapshot) {
            return Ok(snapshot);
        }
        if Instant::now() >= deadline {
            return Err(snapshot);
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

async fn post_json(host: &str, bearer: &str, body: &Value) -> (u16, Value) {
    support::post(host, "/commands", Some(bearer), body).await
}
