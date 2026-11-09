//! Hosted response feedback through two authenticated site instances and
//! Postgres. A chat is seeded through the runtime store, so no worker, model,
//! Valkey or Pub/Sub service is needed; the unused publisher points only at
//! loopback.
use std::path::PathBuf;

use physician_auth::{AuthStore, Membership, PostgresAuthStore};
use serde_json::{Value, json};

#[path = "scale/mod.rs"]
mod support;

struct Running {
    database: support::Database,
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
    async fn start(chat: &str) -> Option<Self> {
        let database = support::Database::create("feedback").await?;
        support::staging_account(&database.url).await;
        let store = local_product::runtime::Store::connect(&database.url)
            .await
            .expect("runtime store");
        support::answered_chat(
            &store,
            physician_sample::WORKSPACE_IDS[0],
            chat,
            &[
                ("Latest potassium?", "Potassium was 4.1 on Monday."),
                ("And sodium?", "Sodium was 139 on Monday."),
            ],
        )
        .await;
        let home = std::env::temp_dir().join(format!(
            "physician-feedback-{}",
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
            support::wait_ready(&mut site, port).await;
            sites.push(site);
        }
        let session = support::sign_in(&hosts[0]).await;
        Some(Self {
            database,
            sites,
            hosts,
            session,
            home,
        })
    }

    fn logs(&self) -> String {
        format!("{}\n{}", self.sites[0].tail(), self.sites[1].tail())
    }

    /// Another physician of the practice, with the patient on their panel or
    /// not; returns their bearer.
    async fn colleague(&self, membership: Membership) -> String {
        let auth = PostgresAuthStore::connect_without_migrations(&self.database.url)
            .await
            .unwrap();
        let email = "colleague@example.test";
        auth.ensure_account(
            physician_sample::STAGING_PRACTICE_ID,
            "colleague",
            email,
            "Colleague",
            false,
            support::unix_now(),
        )
        .await
        .unwrap();
        let token = auth
            .begin_sign_in(email, support::unix_now())
            .await
            .unwrap();
        let (identity, bearer) = auth
            .complete_sign_in(email, &token, support::unix_now())
            .await
            .unwrap();
        auth.set_membership(&identity, physician_sample::WORKSPACE_IDS[0], membership)
            .await
            .unwrap();
        bearer
    }
}

async fn put(host: &str, bearer: Option<&str>, path: &str, body: &Value) -> (u16, Value) {
    let bytes = serde_json::to_vec(body).unwrap();
    let authorization = bearer.map(|bearer| format!("Bearer {bearer}"));
    let mut headers = vec![("content-type", "application/json")];
    if let Some(authorization) = &authorization {
        headers.push(("authorization", authorization));
    }
    let (status, text) = support::exchange(host, "PUT", path, &headers, Some(&bytes))
        .await
        .expect("feedback save");
    (
        status,
        serde_json::from_str(&text).unwrap_or(json!({"raw": text})),
    )
}

/// A rating lands on the response it names, reads back through the other
/// instance, keeps its details across a new rating, and belongs to its author.
#[tokio::test]
async fn feedback_stays_on_its_response_and_with_its_author() {
    let chat = "chat-feedback";
    let Some(running) = Running::start(chat).await else {
        return;
    };
    let (a, b, session) = (&running.hosts[0], &running.hosts[1], &running.session);
    let workspace = physician_sample::WORKSPACE_IDS[0];
    let base = format!("/physician-state/feedback/{workspace}/{chat}");

    // The snapshot names each of Coda's responses by its place.
    let (status, snapshot) = support::get_json(
        b,
        &format!("/patient-workspaces/{workspace}/chats/{chat}/snapshot"),
        session,
    )
    .await;
    assert_eq!(status, 200, "{snapshot}\n{}", running.logs());
    let places: Vec<_> = snapshot["snapshot"]["conversation"]
        .as_array()
        .unwrap()
        .iter()
        .map(|turn| turn.get("response").cloned())
        .collect();
    assert_eq!(places, [None, Some(json!(1)), None, Some(json!(3))]);

    assert_eq!(
        support::get_json(b, &base, session).await,
        (200, json!({"feedback": []}))
    );
    let rated = put(
        a,
        Some(session),
        &format!("{base}/3/rating"),
        &json!({"rating": "down"}),
    )
    .await;
    assert_eq!(
        rated,
        (
            200,
            json!({"response": 3, "rating": "down", "details": null})
        ),
        "{}",
        running.logs()
    );
    let details = json!({"reasons": ["missing_context", "incorrect_information"], "text": "Sodium was rechecked Tuesday."});
    let saved = put(b, Some(session), &format!("{base}/3/details"), &details).await;
    assert_eq!(saved.0, 200, "{}", saved.1);
    assert_eq!(
        saved.1["details"]["reasons"],
        json!(["incorrect_information", "missing_context"])
    );
    let rated = put(
        b,
        Some(session),
        &format!("{base}/3/rating"),
        &json!({"rating": "up"}),
    )
    .await;
    assert_eq!(rated.1["rating"], "up");
    assert_eq!(rated.1["details"]["text"], "Sodium was rechecked Tuesday.");
    let up = put(
        a,
        Some(session),
        &format!("{base}/1/rating"),
        &json!({"rating": "up"}),
    )
    .await;
    assert_eq!(up.0, 200, "{}", up.1);
    let (status, read) = support::get_json(b, &base, session).await;
    assert_eq!(status, 200);
    let ratings: Vec<_> = read["feedback"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| (f["response"].clone(), f["rating"].clone()))
        .collect();
    assert_eq!(ratings, [(json!(1), json!("up")), (json!(3), json!("up"))]);

    // Only Coda's responses take a rating; details need one first.
    for place in [0, 2, 4, 99] {
        let refused = put(
            a,
            Some(session),
            &format!("{base}/{place}/rating"),
            &json!({"rating": "up"}),
        )
        .await;
        assert_eq!(refused.0, 404, "place {place}: {}", refused.1);
    }
    let other_chat = format!("/physician-state/feedback/{workspace}/no-such-chat");
    let refused = put(
        a,
        Some(session),
        &format!("{other_chat}/1/details"),
        &details,
    )
    .await;
    assert_eq!(refused.0, 404);
    let too_long = json!({"reasons": [], "text": "x".repeat(4001)});
    let refused = put(a, Some(session), &format!("{base}/1/details"), &too_long).await;
    assert_eq!(refused.0, 422);

    // Without a session: 401. Off the panel: 404 and nothing written.
    let (status, _) = support::exchange(a, "GET", &base, &[], None).await.unwrap();
    assert_eq!(status, 401);
    let refused = put(
        a,
        None,
        &format!("{base}/1/rating"),
        &json!({"rating": "down"}),
    )
    .await;
    assert_eq!(refused.0, 401);
    let outsider = running.colleague(Membership::Denied).await;
    assert_eq!(support::get_json(a, &base, &outsider).await.0, 404);
    let refused = put(
        a,
        Some(&outsider),
        &format!("{base}/1/rating"),
        &json!({"rating": "down"}),
    )
    .await;
    assert_eq!(refused.0, 404);

    // A colleague with the patient on their panel reads none of the author's
    // feedback, and cannot rate a chat that is not theirs.
    let colleague = running.colleague(Membership::Allowed).await;
    assert_eq!(
        support::get_json(b, &base, &colleague).await,
        (200, json!({"feedback": []}))
    );
    let refused = put(
        b,
        Some(&colleague),
        &format!("{base}/1/rating"),
        &json!({"rating": "down"}),
    )
    .await;
    assert_eq!(refused.0, 404);
    let (_, read) = support::get_json(a, &base, session).await;
    assert_eq!(read["feedback"][0]["rating"], "up");
}
