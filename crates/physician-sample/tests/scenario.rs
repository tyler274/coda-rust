//! Scenario tags on hosted chat: only a rehearsal practice's session may send
//! one. The same configured site serves the rehearsal sign-in, whose
//! assertion checks the crate's unit tests cover with fixture-signed tokens. One authenticated durable site
//! accepts into a disposable Postgres; no worker runs, since acceptance is
//! what is judged.

use std::path::PathBuf;

use local_product::runtime::{Scope, Store};
use physician_auth::{AuthStore, PostgresAuthStore};
use physician_sample::rehearsal::{
    REHEARSAL_EMAIL, REHEARSAL_PHYSICIAN_ID, REHEARSAL_PRACTICE_ID, rehearsal_workspaces,
    seed_rehearsal_practice,
};
use serde_json::{Value, json};

#[path = "scale/mod.rs"]
mod support;

struct Home(PathBuf);

impl Drop for Home {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn submit(workspace: &str, chat: &str, scenario: Option<&str>) -> Value {
    let mut command = support::submit(workspace, chat, "Rehearse a reply");
    if let Some(scenario) = scenario {
        command["payload"]["scenario"] = json!(scenario);
    }
    command
}

#[tokio::test]
async fn only_a_rehearsal_session_may_tag_a_message_with_a_scenario() {
    let Some(database) = support::Database::create("scenario").await else {
        return;
    };
    support::staging_account(&database.url).await;
    let auth = PostgresAuthStore::connect_without_migrations(&database.url)
        .await
        .expect("auth store");
    let store = Store::connect(&database.url).await.expect("runtime store");
    seed_rehearsal_practice(&auth, &store, support::unix_now())
        .await
        .expect("seed the rehearsal practice");
    // Seeding again changes nothing.
    seed_rehearsal_practice(&auth, &store, support::unix_now())
        .await
        .expect("seed the rehearsal practice again");

    let home = Home(std::env::temp_dir().join(format!(
        "physician-scenario-{}",
        uuid::Uuid::new_v4().simple()
    )));
    std::fs::create_dir_all(&home.0).expect("home");
    let port = support::free_port();
    let host = format!("127.0.0.1:{port}");
    let mut site = support::Process::spawn(
        "site",
        support::site_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            support::DURABLE,
            ("PHYSICIAN_AUTH", "required"),
            ("PHYSICIAN_LISTEN_ADDR", host.as_str()),
            ("CODA_HOSTED_PROFILE", "staging-preview"),
            ("PHYSICIAN_SIMULATION_PRACTICES", REHEARSAL_PRACTICE_ID),
            (
                "PHYSICIAN_IAP_AUDIENCE",
                "/projects/123/locations/us-east1/services/physician",
            ),
            (
                "PHYSICIAN_REHEARSAL_PRINCIPAL",
                "physician-rehearsal@coda-rust-staging.iam.gserviceaccount.com",
            ),
        ],
        &home.0,
    );
    support::wait_ready(&mut site, port).await;

    // The configured site serves the rehearsal sign-in, and a caller without
    // IAP's assertion gets no session from it.
    let (status, body) = support::post(&host, "/auth/rehearsal", None, &json!({})).await;
    assert_eq!(status, 401, "{body}");
    assert!(body.get("session").is_none(), "{body}");

    // A real practice's session: any scenario is refused and nothing is saved.
    let staging = support::sign_in(&host).await;
    let workspace = physician_sample::WORKSPACE_IDS[0];
    let chat = format!("chat-{}", uuid::Uuid::new_v4().simple());
    let (status, body) = support::post(
        &host,
        "/commands",
        Some(&staging),
        &submit(workspace, &chat, Some("drill.rehearse")),
    )
    .await;
    assert_eq!((status, body["outcome"].as_str()), (403, Some("rejected")));
    let staging_scope = Scope {
        practice: physician_sample::STAGING_PRACTICE_ID.into(),
        physician: "staging-physician".into(),
        workspace: workspace.into(),
        chat: chat.clone(),
    };
    assert!(store.read(&staging_scope).await.unwrap().is_none());
    // Without a scenario the same message is accepted and stays untagged.
    let (status, body) = support::post(
        &host,
        "/commands",
        Some(&staging),
        &submit(workspace, &chat, None),
    )
    .await;
    assert_eq!(
        (status, body["outcome"].as_str()),
        (200, Some("accepted")),
        "{body}\n{}",
        site.tail()
    );
    let saved = store.read(&staging_scope).await.unwrap().expect("saved");
    assert_eq!(saved.scenario, None);

    // The rehearsal physician: a slug is accepted and saved with the input;
    // a malformed one is refused.
    let now = support::unix_now();
    let token = auth.begin_sign_in(REHEARSAL_EMAIL, now).await.unwrap();
    let (_, rehearsal) = auth
        .complete_sign_in(REHEARSAL_EMAIL, &token, now)
        .await
        .unwrap();
    let rehearsal_workspace = rehearsal_workspaces().remove(0);
    let rehearsal_chat = format!("chat-{}", uuid::Uuid::new_v4().simple());
    let (status, body) = support::post(
        &host,
        "/commands",
        Some(&rehearsal),
        &submit(&rehearsal_workspace, &rehearsal_chat, Some("Not A Slug")),
    )
    .await;
    assert_eq!((status, body["outcome"].as_str()), (403, Some("rejected")));
    let (status, body) = support::post(
        &host,
        "/commands",
        Some(&rehearsal),
        &submit(
            &rehearsal_workspace,
            &rehearsal_chat,
            Some("drill.rehearse"),
        ),
    )
    .await;
    assert_eq!(
        (status, body["outcome"].as_str()),
        (200, Some("accepted")),
        "{body}\n{}",
        site.tail()
    );
    let rehearsal_scope = Scope {
        practice: REHEARSAL_PRACTICE_ID.into(),
        physician: REHEARSAL_PHYSICIAN_ID.into(),
        workspace: rehearsal_workspace.clone(),
        chat: rehearsal_chat,
    };
    let saved = store.read(&rehearsal_scope).await.unwrap().expect("saved");
    assert_eq!(saved.scenario.as_deref(), Some("drill.rehearse"));

    // The rehearsal physician sees only the rehearsal panel.
    let (status, body) = support::get_json(&host, "/patient-workspaces", &rehearsal).await;
    assert_eq!(status, 200, "{body}");
    let listed = body.to_string();
    for workspace in rehearsal_workspaces() {
        assert!(listed.contains(&workspace), "{workspace} missing: {listed}");
    }
    for workspace in physician_sample::WORKSPACE_IDS {
        assert!(!listed.contains(workspace), "{workspace} leaked: {listed}");
    }
    let (status, _) = support::get_json(
        &host,
        &format!("/patient-workspaces/{workspace}"),
        &rehearsal,
    )
    .await;
    assert_eq!(status, 404);
}
