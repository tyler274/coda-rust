//! A chat turn the worker gives up on, then Try again.
//!
//! One site and one worker share a disposable Postgres, Valkey and the Pub/Sub
//! emulator. The worker's fixed provider fails five times, so the delivery
//! goes to the dead-letter topic and the turn ends as failed. A stream held on
//! the site is told to refetch, and the snapshot then shows the reference
//! "couldn't finish" line with the run Try again resumes. `work.resume` for
//! that run is accepted, and the reply arrives.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use local_product::runtime::delivery::Delivery;
use serde_json::{Value, json};

#[path = "scale/mod.rs"]
mod support;
use support::{get_json, sign_in};

const COULDNT_FINISH: &str = "Coda couldn’t finish this response. Your message is saved.";

struct Home(PathBuf);

impl Drop for Home {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[tokio::test]
async fn a_dead_lettered_turn_shows_couldnt_finish_and_try_again_gets_the_reply() {
    let Some(database) = support::Database::create("try_again").await else {
        return;
    };
    support::staging_account(&database.url).await;

    let home = Home(std::env::temp_dir().join(format!(
        "physician-scale-try-again-{}-{}",
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
            // Exactly the subscription's delivery budget: every first-round
            // delivery fails, and the resumed one succeeds.
            ("PHYSICIAN_WORKER_FIXED_FAILURES", "5"),
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
    let chat = format!("chat-{}", uuid::Uuid::new_v4().simple());
    let snapshot_path = format!("/patient-workspaces/{workspace}/chats/{chat}/snapshot");
    let deadline = Instant::now() + Duration::from_secs(15);
    while valkey.subscribers() < 2 {
        assert!(
            Instant::now() < deadline,
            "expected the site and the worker on Valkey\n{}",
            tails(&site, &worker)
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    // The page's stream, held from before the send. Acceptance publishes
    // nothing, so only the worker can tell it the turn ended.
    let mut sse = support::Sse::open(
        &site_listen,
        &format!("/patient-workspaces/events?patient_workspace_id={workspace}"),
        &session,
    )
    .await
    .unwrap_or_else(|error| panic!("{error}\n{}", site.tail()));
    let (status, body) = post_json(
        &site_listen,
        "/commands",
        &session,
        &json!({
            "type": "message.submit",
            "idempotency_key": format!("key-{}", uuid::Uuid::new_v4().simple()),
            "payload": {
                "patient_workspace_id": workspace,
                "chat_id": chat,
                "text": "Hello",
                "physician_time_zone": "America/New_York"
            }
        }),
    )
    .await;
    assert_eq!(status, 200, "{body}\n{}", tails(&site, &worker));
    assert_eq!(body["outcome"], "accepted", "{body}");

    // Five failed deliveries reach the dead-letter topic.
    let deadline = Instant::now() + Duration::from_secs(90);
    loop {
        let forwarded = emulator.pull_dead_letters().await;
        if forwarded.iter().any(|bytes| {
            serde_json::from_slice::<Delivery>(bytes)
                .is_ok_and(|delivery| delivery.scope.chat == chat)
        }) {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "dead-letter topic did not receive the turn\n{}\n{}",
            tails(&site, &worker),
            emulator.tail()
        );
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    // A page held open learns of the failure without reloading.
    sse.wait_for("event: workspace_changed", Duration::from_secs(20))
        .await
        .unwrap_or_else(|error| {
            panic!(
                "the held stream was never told the turn failed: {error}\n{}",
                tails(&site, &worker)
            )
        });

    // The snapshot keeps the message and names the failed run.
    let failed = wait_snapshot(&site_listen, &snapshot_path, &session, |snapshot| {
        mechanical(snapshot).is_some()
    })
    .await
    .unwrap_or_else(|last| {
        panic!(
            "snapshot never showed the failed turn: {last}\n{}",
            tails(&site, &worker)
        )
    });
    let items = failed["conversation"].as_array().expect("conversation");
    assert_eq!(items[0]["text"], "Hello", "{failed}");
    assert!(
        !texts(&failed).contains(&support::FIXED_TEXT),
        "a failed turn has no reply: {failed}"
    );
    let line = mechanical(&failed).expect("mechanical line");
    assert_eq!(line["text"], COULDNT_FINISH, "{failed}");
    let work = line["resume_target"]["work_item_id"]
        .as_str()
        .expect("work_item_id")
        .to_owned();
    let run = line["resume_target"]["expected_run_id"]
        .as_str()
        .expect("expected_run_id")
        .to_owned();
    assert_eq!(
        failed["runs"],
        json!([{ "run_id": run, "state": "failed" }]),
        "{failed}"
    );

    // A run that is not the failed one is refused and changes nothing.
    let (status, body) = post_json(
        &site_listen,
        "/commands",
        &session,
        &resume(workspace, &chat, &work, "not-the-run", "resume-wrong"),
    )
    .await;
    assert_eq!(status, 200, "{body}");
    assert_eq!(body["outcome"], "rejected", "{body}");

    // Try again on the named run.
    let key = format!("resume-{work}-{run}");
    let (status, body) = post_json(
        &site_listen,
        "/commands",
        &session,
        &resume(workspace, &chat, &work, &run, &key),
    )
    .await;
    assert_eq!(status, 200, "{body}\n{}", tails(&site, &worker));
    assert_eq!(body["outcome"], "accepted", "{body}");

    let answered = wait_snapshot(&site_listen, &snapshot_path, &session, |snapshot| {
        texts(snapshot).contains(&support::FIXED_TEXT)
    })
    .await
    .unwrap_or_else(|last| {
        panic!(
            "the resumed turn never got its reply: {last}\n{}",
            tails(&site, &worker)
        )
    });
    assert_eq!(texts(&answered), ["Hello", support::FIXED_TEXT]);
    assert!(mechanical(&answered).is_none(), "{answered}");
    let runs = answered["runs"].as_array().expect("runs");
    assert_eq!(runs.len(), 1, "{answered}");
    assert_eq!(runs[0]["state"], "completed", "{answered}");
    assert_ne!(runs[0]["run_id"], run, "the answer is a successor attempt");

    // A reload without `?chat=` still finds this chat: the workspace lists it,
    // titled by its first message, ahead of the recording's empty list.
    let (status, listed) = get_json(
        &site_listen,
        &format!("/patient-workspaces/{workspace}"),
        &session,
    )
    .await;
    assert_eq!(status, 200, "{listed}");
    let listed = &listed["chats"];
    let chats = listed.as_array().expect("chats");
    assert_eq!(chats.len(), 1, "{listed}");
    assert_eq!(chats[0]["chat_id"], chat, "{listed}");
    assert_eq!(chats[0]["title"], "Hello", "{listed}");
    assert!(
        chats[0]["created_at"]
            .as_str()
            .is_some_and(|at| at.ends_with('Z') && !at.starts_with("1970")),
        "{listed}"
    );

    // The same press again is the same request; completed work never reopens.
    let (_, body) = post_json(
        &site_listen,
        "/commands",
        &session,
        &resume(workspace, &chat, &work, &run, &key),
    )
    .await;
    assert_eq!(body["outcome"], "accepted", "{body}");
    let (_, body) = post_json(
        &site_listen,
        "/commands",
        &session,
        &resume(workspace, &chat, &work, &run, "resume-after-completion"),
    )
    .await;
    assert_eq!(body["outcome"], "rejected", "{body}");
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

/// Polls the snapshot until `done`; on timeout, returns the last body seen.
async fn wait_snapshot(
    host: &str,
    path: &str,
    session: &str,
    done: impl Fn(&Value) -> bool,
) -> Result<Value, Value> {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        let (status, body) = get_json(host, path, session).await;
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

async fn post_json(host: &str, path: &str, bearer: &str, body: &Value) -> (u16, Value) {
    support::post(host, path, Some(bearer), body).await
}
