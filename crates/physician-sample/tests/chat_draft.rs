//! Hosted composer drafts through two authenticated site instances and Postgres.
//! No worker, reply revision, model call, Valkey or Pub/Sub service is needed:
//! draft saves never publish work. The unused publisher points only at loopback.
use std::path::PathBuf;

use local_product::runtime::{Scope, Store};
use physician_auth::{AuthStore, Membership, PostgresAuthStore};
use serde_json::{Value, json};

#[path = "scale/mod.rs"]
mod support;

struct Running {
    database: support::Database,
    store: Store,
    sites: Vec<support::Process>,
    hosts: [String; 2],
    session: String,
    home: PathBuf,
}

impl Drop for Running {
    fn drop(&mut self) {
        for site in &mut self.sites {
            site.kill_now();
        }
        let _ = std::fs::remove_dir_all(&self.home);
    }
}

impl Running {
    async fn start() -> Option<Self> {
        let database = support::Database::create("chat_draft").await?;
        support::staging_account(&database.url).await;
        let store = Store::connect(&database.url).await.expect("runtime store");
        let home = std::env::temp_dir().join(format!(
            "physician-chat-draft-{}",
            uuid::Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(&home).expect("home");
        let topic = support::topic();
        let unused_publisher = format!("127.0.0.1:{}", support::free_port());
        let mut sites = Vec::new();
        let mut hosts = [String::new(), String::new()];
        for (index, host) in hosts.iter_mut().enumerate() {
            let port = support::free_port();
            *host = format!("127.0.0.1:{port}");
            let mut site = support::Process::spawn(
                &format!("site-{index}"),
                support::site_bin(),
                &[],
                &[
                    ("PHYSICIAN_DATABASE_URL", &database.url),
                    ("PUBSUB_EMULATOR_HOST", &unused_publisher),
                    ("GOOGLE_CLOUD_PROJECT", support::PROJECT),
                    ("PHYSICIAN_CHAT_TOPIC", &topic),
                    ("PHYSICIAN_AUTH", "required"),
                    ("PHYSICIAN_LISTEN_ADDR", host),
                ],
                &home,
            );
            // Start sequentially, as each instance seeds the practice panel.
            support::wait_ready(&mut site, port).await;
            sites.push(site);
        }
        let session = support::sign_in(&hosts[0]).await;
        Some(Self {
            database,
            store,
            sites,
            hosts,
            session,
            home,
        })
    }

    fn logs(&self) -> String {
        format!("{}\n{}", self.sites[0].tail(), self.sites[1].tail())
    }
}

async fn put_chat_draft(
    host: &str,
    session: &str,
    path: &str,
    expected: u64,
    text: &str,
) -> (u16, Value) {
    let authorization = format!("Bearer {session}");
    let bytes = serde_json::to_vec(&json!({"expected_revision": expected, "text": text})).unwrap();
    let (status, body) = support::exchange(
        host,
        "PUT",
        path,
        &[
            ("authorization", &authorization),
            ("content-type", "application/json"),
        ],
        Some(&bytes),
    )
    .await
    .expect("chat draft save");
    (
        status,
        serde_json::from_str(&body).unwrap_or(json!({"raw": body})),
    )
}

/// The composer's Save on a hosted site: saved through one instance, read
/// through the other, fenced by revision, and owned by one physician.
#[tokio::test]
async fn chat_draft_saves_on_one_site_and_reads_on_the_other() {
    let Some(running) = Running::start().await else {
        return;
    };
    let workspace = physician_sample::WORKSPACE_IDS[0];
    let path = format!("/physician-state/drafts/{workspace}/draft");
    let (status, body) = support::get_json(&running.hosts[1], &path, &running.session).await;
    assert_eq!((status, body), (200, json!({"revision": 0, "text": ""})));
    let saved = put_chat_draft(
        &running.hosts[0],
        &running.session,
        &path,
        0,
        "Check potassium",
    )
    .await;
    assert_eq!(
        saved,
        (200, json!({"revision": 1, "text": "Check potassium"})),
        "{}",
        running.logs()
    );
    let (status, body) = support::get_json(&running.hosts[1], &path, &running.session).await;
    assert_eq!(
        (status, body),
        (200, json!({"revision": 1, "text": "Check potassium"}))
    );
    let saved = put_chat_draft(
        &running.hosts[1],
        &running.session,
        &path,
        1,
        "Check potassium and sodium",
    )
    .await;
    assert_eq!(saved.0, 200, "{}", saved.1);
    assert_eq!(saved.1["revision"], 2);
    // Site A still holds revision 1 in its page; its save is stale.
    let stale = put_chat_draft(
        &running.hosts[0],
        &running.session,
        &path,
        1,
        "lost wording",
    )
    .await;
    assert_eq!(
        stale,
        (
            409,
            json!({"outcome": "conflict", "current": {"revision": 2, "text": "Check potassium and sodium"}})
        )
    );
    let scope = Scope {
        practice: physician_sample::STAGING_PRACTICE_ID.into(),
        physician: "staging-physician".into(),
        workspace: workspace.into(),
        chat: "draft".into(),
    };
    let stored = running.store.chat_draft(&scope).await.unwrap();
    assert_eq!(
        (stored.revision, stored.text.as_str()),
        (2, "Check potassium and sodium")
    );

    // Another physician: outside the panel the patient is not found; inside
    // it, they see and save only their own draft.
    let auth = PostgresAuthStore::connect_without_migrations(&running.database.url)
        .await
        .unwrap();
    let email = "other-draft-physician@example.test";
    auth.ensure_account(
        physician_sample::STAGING_PRACTICE_ID,
        "other-draft-physician",
        email,
        "Other physician",
        false,
        support::unix_now(),
    )
    .await
    .unwrap();
    let token = auth
        .begin_sign_in(email, support::unix_now())
        .await
        .unwrap();
    let (identity, other) = auth
        .complete_sign_in(email, &token, support::unix_now())
        .await
        .unwrap();
    auth.set_membership(&identity, workspace, Membership::Denied)
        .await;
    let (status, _) = support::get_json(&running.hosts[1], &path, &other).await;
    assert_eq!(status, 404);
    let (status, _) =
        put_chat_draft(&running.hosts[1], &other, &path, 2, "foreign overwrite").await;
    assert_eq!(status, 404);
    auth.set_membership(&identity, workspace, Membership::Allowed)
        .await;
    let (status, body) = support::get_json(&running.hosts[0], &path, &other).await;
    assert_eq!((status, body), (200, json!({"revision": 0, "text": ""})));
    let (status, _) = put_chat_draft(&running.hosts[0], &other, &path, 0, "Their own note").await;
    assert_eq!(status, 200);
    let stored = running.store.chat_draft(&scope).await.unwrap();
    assert_eq!(
        (stored.revision, stored.text.as_str()),
        (2, "Check potassium and sodium")
    );
    let (status, _) = support::exchange(&running.hosts[0], "GET", &path, &[], None)
        .await
        .unwrap();
    assert_eq!(status, 401);
}
