//! Two physician-sample processes on one Valkey and one Postgres.
//!
//! A turn accepted on instance A is completed by the worker and shows up on
//! an SSE stream held by instance B. Restarting Valkey makes that stream emit
//! `event: resync`.

use std::path::PathBuf;
use std::time::Duration;

use serde_json::{Value, json};

#[path = "scale/mod.rs"]
mod support;

struct Home(PathBuf);

impl Drop for Home {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[tokio::test]
async fn cross_instance_turn_is_visible_on_the_other_sse_stream_and_valkey_restart_resyncs() {
    let Some(database) = support::Database::create("fanout").await else {
        return;
    };
    let auth = physician_auth::PostgresAuthStore::connect_without_migrations(&database.url)
        .await
        .expect("auth store");
    auth.ensure_account(
        physician_sample::STAGING_PRACTICE_ID,
        "staging-physician",
        "physician@codahealth.test",
        "Staging Physician",
        true,
        unix_now(),
    )
    .await
    .expect("staging account");
    let runtime = local_product::runtime::Store::connect(&database.url)
        .await
        .expect("runtime store");
    physician_sample::patients::enroll_fixture(&auth, &runtime)
        .await
        .expect("fixture patients");
    drop(auth);

    let home = Home(std::env::temp_dir().join(format!(
        "physician-scale-fanout-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4().simple()
    )));
    std::fs::create_dir_all(&home.0).expect("home");
    let mut valkey = support::Valkey::start().await;
    let worker_port = support::free_port();
    let site_a_port = support::free_port();
    let site_b_port = support::free_port();

    let redis = valkey.url();
    let worker_listen = format!("127.0.0.1:{worker_port}");
    let site_a_listen = format!("127.0.0.1:{site_a_port}");
    let site_b_listen = format!("127.0.0.1:{site_b_port}");
    let stub = support::ResponsesStub::start(&home.0, 0, 0, "[]").await;
    let mut worker = support::Process::spawn(
        "worker",
        support::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", worker_listen.as_str()),
            ("PHYSICIAN_RESPONSES_STUB_PORT", &stub.port),
        ],
        &home.0,
    );
    support::wait_ready(&mut worker, worker_port).await;

    let site_env = [
        ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
        ("PHYSICIAN_REDIS_URL", redis.as_str()),
        support::DURABLE,
        ("PHYSICIAN_AUTH", "required"),
    ];
    let mut site_a = support::Process::spawn(
        "site-a",
        support::site_bin(),
        &[],
        &[
            site_env[0],
            site_env[1],
            site_env[2],
            site_env[3],
            ("PHYSICIAN_LISTEN_ADDR", site_a_listen.as_str()),
        ],
        &home.0,
    );
    support::wait_ready(&mut site_a, site_a_port).await;
    let mut site_b = support::Process::spawn(
        "site-b",
        support::site_bin(),
        &[],
        &[
            site_env[0],
            site_env[1],
            site_env[2],
            site_env[3],
            ("PHYSICIAN_LISTEN_ADDR", site_b_listen.as_str()),
        ],
        &home.0,
    );
    support::wait_ready(&mut site_b, site_b_port).await;

    let deadline = std::time::Instant::now() + Duration::from_secs(15);
    while valkey.subscribers() < 3 {
        assert!(
            std::time::Instant::now() < deadline,
            "expected both sites and the worker on Redis, saw {}\n{}\n{}\n{}",
            valkey.subscribers(),
            site_a.tail(),
            site_b.tail(),
            worker.tail()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    let session = sign_in(&site_a_listen).await;
    let workspace = physician_sample::WORKSPACE_IDS[0];
    let chat = format!("chat-{}", uuid::Uuid::new_v4().simple());
    let mut sse = support::Sse::open(
        &site_b_listen,
        &format!("/patient-workspaces/events?patient_workspace_id={workspace}"),
        &session,
    )
    .await
    .unwrap_or_else(|error| panic!("{error}\n{}", site_b.tail()));

    let (status, body) = post_json(
        &site_a_listen,
        "/commands",
        Some(&session),
        &json!({
            "type": "message.submit",
            "idempotency_key": format!("key-{}", uuid::Uuid::new_v4().simple()),
            "payload": {
                "patient_workspace_id": workspace,
                "chat_id": chat,
                "text": "Hello from instance A",
                "physician_time_zone": "America/New_York"
            }
        }),
    )
    .await;
    assert_eq!(status, 200, "{body}\n{}", site_a.tail());
    assert_eq!(body["outcome"], "accepted", "{body}");

    sse.wait_for("event: workspace_changed", Duration::from_secs(20))
        .await
        .unwrap_or_else(|error| {
            panic!(
                "{error}\n{}\n{}\n{}",
                site_a.tail(),
                site_b.tail(),
                worker.tail()
            )
        });
    assert!(
        sse.buffer.contains(workspace),
        "workspace missing from {}",
        sse.buffer
    );
    assert!(
        !sse.buffer.contains("event: resync"),
        "resync arrived before Valkey restarted: {}",
        sse.buffer
    );

    let (status, snapshot) = get_json(
        &site_b_listen,
        &format!("/patient-workspaces/{workspace}/chats/{chat}/snapshot"),
        &session,
    )
    .await;
    assert_eq!(status, 200, "{snapshot}\n{}", site_b.tail());
    let texts = support::chat_texts(&snapshot["snapshot"]);
    assert!(
        texts.iter().any(|text| text == "Hello from instance A"),
        "instance B snapshot missing the turn: {texts:?}"
    );
    assert!(
        texts.iter().any(|text| text == support::FIXED_TEXT),
        "instance B snapshot missing the worker reply: {texts:?}"
    );

    valkey.restart().await;
    sse.wait_for("event: resync", Duration::from_secs(20))
        .await
        .unwrap_or_else(|error| panic!("{error}\n{}", valkey.tail()));
    assert!(
        sse.buffer.contains("event: resync\ndata: {}"),
        "resync frame: {}",
        sse.buffer
    );
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or_default()
}

async fn sign_in(host: &str) -> String {
    let (status, body) = post_json(
        host,
        "/auth/sign-in",
        None,
        &json!({ "email": "physician@codahealth.test" }),
    )
    .await;
    assert_eq!(status, 200, "{body}");
    let token = body["token"].as_str().expect("sign-in token").to_owned();
    let (status, body) = post_json(
        host,
        "/auth/verify",
        None,
        &json!({ "email": "physician@codahealth.test", "token": token }),
    )
    .await;
    assert_eq!(status, 200, "{body}");
    body["session"].as_str().expect("bearer session").to_owned()
}

async fn post_json(host: &str, path: &str, bearer: Option<&str>, body: &Value) -> (u16, Value) {
    let bytes = serde_json::to_vec(body).expect("json");
    let mut headers = vec![("content-type", "application/json")];
    let authorization;
    if let Some(bearer) = bearer {
        authorization = format!("Bearer {bearer}");
        headers.push(("authorization", authorization.as_str()));
    }
    let (status, text) = support::exchange(host, "POST", path, &headers, Some(&bytes))
        .await
        .expect("post");
    let value = serde_json::from_str(&text).unwrap_or(json!({ "raw": text }));
    (status, value)
}

async fn get_json(host: &str, path: &str, bearer: &str) -> (u16, Value) {
    let authorization = format!("Bearer {bearer}");
    let (status, text) = support::exchange(
        host,
        "GET",
        path,
        &[("authorization", authorization.as_str())],
        None,
    )
    .await
    .expect("get");
    let value = serde_json::from_str(&text).unwrap_or(json!({ "raw": text }));
    (status, value)
}
