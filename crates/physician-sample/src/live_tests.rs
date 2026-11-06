//! Focused real-HTTP proof. Synthetic retained source bytes, no model calls.
use super::*;
use harness::execution::{Response as ModelResponse, Usage};
use local_product::execution::PhysicianSession;
use physician_auth::{AuthStore, MemoryAuthStore};
use std::sync::Arc;

#[tokio::test]
async fn reply_http_reads_exact_source_commits_and_survives_owner_restart() {
    let path = std::env::temp_dir().join(format!("physician-http-{}.json", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let store = Arc::new(MemoryAuthStore::new());
    store.seed_account(
        auth::STAGING_PRACTICE_ID,
        "staging-physician",
        "physician@codahealth.test",
        "Physician",
        true,
    );
    let auth = auth::Auth::over_fixture(
        store.clone(),
        Arc::new(front_door::MemoryFrontDoorStore::new()),
    )
    .await;
    let sign = store
        .begin_sign_in("physician@codahealth.test", auth::now())
        .await
        .unwrap();
    let (identity, token) = store
        .complete_sign_in("physician@codahealth.test", &sign, auth::now())
        .await
        .unwrap();
    let ctx = auth::AuthContext {
        auth: auth.clone(),
        token: token.clone(),
        identity,
    };
    let workspace = WORKSPACE_IDS[0];
    let target = reply::target(&ctx, "reply-one");
    let mut session = PhysicianSession::open(&path).unwrap();
    // This is the source owner, not a model or projection-derived fixture.
    session
        .save_draft(workspace, &target, 0, "physician saved wording")
        .unwrap();
    let source = local_product::physician_sources::SourceRevision {
        workspace: workspace.into(),
        source_id: "ehr".into(),
        candidate_id: "source-one".into(),
        revision: "immutable-7".into(),
        record_type: "Observation".into(),
        dates: vec![],
        content: "Potassium 4.1 mmol/L".into(),
        sha256: local_product::sha256_hex(b"Potassium 4.1 mmol/L"),
        readable_parts: vec![],
        provenance: json!({"native_record":"lab-7"}),
        native: None,
    };
    let binding = local_product::ModelBinding {
        model: "controlled".into(),
        effort: "high".into(),
        adapter: "replay".into(),
    };
    let temporary = live::Live::on(
        live::ChatModel::Fixed("unused".into()),
        session,
        vec![source.clone()],
    )
    .unwrap();
    let payload = json!({"patient_workspace_id":workspace,"reply_work_id":"reply-one","expected_draft_revision":1,"instruction":"make concise","physician_time_zone":"UTC"});
    let accepted = reply::admit(&temporary, &ctx, "build-cassette", payload.clone())
        .await
        .unwrap();
    let request = accepted.request.clone();
    temporary
        .session
        .lock()
        .unwrap()
        .settle(&accepted, local_product::execution::Status::Cancelled)
        .unwrap();
    let read = harness::execution::Call {
        id: "read-one".into(),
        name: "read_patient_sources".into(),
        arguments: json!({"references":["source-one"],"max_result_bytes":4096}),
    };
    let first = ModelResponse {
        text: String::new(),
        calls: vec![read.clone()],
        usage: Usage::default(),
        provenance: json!({"mode":"controlled"}),
    };
    let result = local_product::physician_sources::exact_read(
        std::slice::from_ref(&source),
        workspace,
        &["source-one".into()],
        4096,
    )
    .unwrap();
    let mut next = request.clone();
    next.items.extend([
        harness::execution::Item::Call(read),
        harness::execution::Item::Result {
            call_id: "read-one".into(),
            value: result,
        },
    ]);
    let terminal = ModelResponse {
        text: String::new(),
        calls: vec![harness::execution::Call {
            id: "finish".into(),
            name: "finish_reply_revision".into(),
            arguments: json!({"text":"revised physician wording","review_basis":"Read exact lab-7."}),
        }],
        usage: Usage::default(),
        provenance: json!({"mode":"controlled"}),
    };
    drop(temporary);
    let live = live::Live::on(
        live::ChatModel::StructuredReplay {
            binding,
            replay: tokio::sync::Mutex::new(
                harness::transport::Replay::new(harness::transport::Cassette {
                    schema_version: 2,
                    adapter: "controlled".into(),
                    authorization: "synthetic HTTP proof".into(),
                    synthetic_non_phi: true,
                    exchanges: vec![(request, first), (next, terminal)],
                })
                .unwrap(),
            ),
        },
        PhysicianSession::open(&path).unwrap(),
        vec![source],
    )
    .unwrap();
    let app = auth::required_router(live_router_on(live.clone()), auth.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{base}/commands"))
        .bearer_auth(&token)
        .json(&json!({"type":"reply.revise","idempotency_key":"http-revision","payload":payload}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let route = format!("{base}/physician-state/reply-drafts/{workspace}/reply-one");
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            let draft: Value = client
                .get(&route)
                .bearer_auth(&token)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            if draft["revision"] == 2 {
                assert_eq!(draft["text"], "revised physician wording");
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    let signed_out = client
        .post(format!("{base}/auth/sign-out"))
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();
    assert_eq!(signed_out.status(), StatusCode::NO_CONTENT);
    assert_eq!(
        client
            .get(&route)
            .bearer_auth(&token)
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::UNAUTHORIZED
    );
    task.abort();
    let _ = task.await;
    drop(live);
    let reopened = PhysicianSession::open(&path).unwrap();
    assert_eq!(
        reopened.draft(workspace, &target).unwrap().1,
        "revised physician wording"
    );
    drop(reopened);
    std::fs::remove_file(&path).unwrap();
    std::fs::remove_file(path.with_extension("lock")).unwrap();
}

/// A scripted inline revision model. Each exchange sees the request and
/// returns the next call; the first records a lab, as if it arrived while
/// the model was working.
struct Scripted {
    live: Arc<live::Live>,
    lab: local_product::physician_sources::SourceRevision,
    seen: Vec<harness::execution::Request>,
}

impl harness::execution::Provider for Scripted {
    async fn exchange(
        &mut self,
        request: &harness::execution::Request,
        _: &harness::execution::Cancellation,
        _: &mut (dyn FnMut(harness::execution::StreamEvent) -> Result<(), String> + Send),
    ) -> Result<ModelResponse, String> {
        self.seen.push(request.clone());
        let finish = harness::execution::Call {
            id: format!("finish-{}", self.seen.len()),
            name: "finish_reply_revision".into(),
            arguments: json!({"text":"revised with the new lab","review_basis":"Read lab-new."}),
        };
        let call = match self.seen.len() {
            1 => {
                assert!(self.live.record_source(self.lab.clone()).unwrap());
                finish
            }
            // Finishing without reading what changed is refused too.
            2 => finish,
            3 => harness::execution::Call {
                id: "read-new".into(),
                name: "read_patient_sources".into(),
                arguments: json!({"references":["lab-new"],"max_result_bytes":4096}),
            },
            _ => finish,
        };
        Ok(ModelResponse {
            text: String::new(),
            calls: vec![call],
            usage: Usage::default(),
            provenance: json!({"mode":"scripted"}),
        })
    }
}

#[tokio::test]
async fn inline_revision_reads_a_source_that_arrives_before_it_finishes() {
    use harness::execution::{self, Item};
    use local_product::physician_sources::SourceRevision;
    let workspace = WORKSPACE_IDS[0];
    let target = "inline-arrival-target";
    let source = |id: &str, content: &str| SourceRevision {
        workspace: workspace.into(),
        source_id: "ehr".into(),
        candidate_id: id.into(),
        revision: "immutable-1".into(),
        record_type: "Observation".into(),
        dates: vec![],
        content: content.into(),
        sha256: local_product::sha256_hex(content.as_bytes()),
        readable_parts: vec![],
        provenance: json!({"synthetic": true}),
        native: None,
    };
    let original = source("lab-old", "Potassium 4.1 mmol/L");
    let lab = source("lab-new", "Potassium 5.9 mmol/L");
    let mut session = PhysicianSession::memory();
    session
        .save_draft(workspace, target, 0, "physician saved wording")
        .unwrap();
    let live = live::Live::on(
        live::ChatModel::Fixed("unused".into()),
        session,
        vec![original.clone()],
    )
    .unwrap();
    let (version, pinned) = live.observed_sources(workspace);
    assert_eq!(pinned, vec![original.clone()]);
    let work = {
        let mut session = live.session.lock().unwrap();
        let request = local_product::runtime::reply::request(
            "scripted",
            "low",
            "physician saved wording",
            "make concise",
            "UTC",
            &pinned,
        );
        let work = session
            .revise("inline-arrival", workspace, target, 1, request)
            .unwrap();
        session.pin_sources(&work, version, pinned).unwrap()
    };
    let mut provider = Scripted {
        live: live.clone(),
        lab: lab.clone(),
        seen: vec![],
    };
    let mut owner = live::LiveOwner {
        live: live.clone(),
        work: work.clone(),
        auth_ctx: None,
    };
    execution::run(
        &mut provider,
        &mut owner,
        work.request.clone(),
        &mut execution::Progress::default(),
        execution::Limits::default(),
        &execution::Cancellation::default(),
        &mut |_| Ok(()),
    )
    .await
    .unwrap();

    let outcome = |request: &harness::execution::Request| match request.items.last() {
        Some(Item::Result { value, .. }) => value["outcome"].as_str().map(str::to_owned),
        _ => None,
    };
    assert_eq!(provider.seen.len(), 4);
    assert_eq!(
        outcome(&provider.seen[1]).as_deref(),
        Some("sources_changed")
    );
    assert_eq!(
        outcome(&provider.seen[2]).as_deref(),
        Some("sources_unread")
    );
    let read = |request: &harness::execution::Request| {
        request.items.iter().any(|item| {
            matches!(item, Item::Result { value, .. }
                if value["results"][0]["content"] == lab.content.as_str())
        })
    };
    assert!(!read(&provider.seen[2]));
    assert!(
        read(&provider.seen[3]),
        "the lab was read before the finish"
    );

    let session = live.session.lock().unwrap();
    let (revision, text, restore) = session.draft(workspace, target).unwrap();
    assert_eq!(revision, 2);
    assert_eq!(text, "revised with the new lab");
    assert_eq!(restore.as_deref(), Some("physician saved wording"));
    let settled = session.work(&work).unwrap();
    assert_eq!(settled.sources, vec![original.clone(), lab.clone()]);
    assert_eq!(settled.refreshes.len(), 1);
    assert_eq!(settled.refreshes[0].from_version, version);
    assert_eq!(settled.refreshes[0].added, vec![lab]);
    assert_eq!(settled.sources_version, settled.refreshes[0].to_version);
}

#[tokio::test]
async fn inline_workspace_lists_the_chat_a_physician_started() {
    let store = Arc::new(MemoryAuthStore::new());
    store.seed_account(
        auth::STAGING_PRACTICE_ID,
        "staging-physician",
        "physician@codahealth.test",
        "Physician",
        true,
    );
    let auth = auth::Auth::over_fixture(
        store.clone(),
        Arc::new(front_door::MemoryFrontDoorStore::new()),
    )
    .await;
    let sign = store
        .begin_sign_in("physician@codahealth.test", auth::now())
        .await
        .unwrap();
    let (_, token) = store
        .complete_sign_in("physician@codahealth.test", &sign, auth::now())
        .await
        .unwrap();
    store.seed_account(
        auth::STAGING_PRACTICE_ID,
        "colleague",
        "colleague@codahealth.test",
        "Colleague",
        false,
    );
    store
        .set_membership(
            &physician_auth::PhysicianIdentity {
                practice_id: auth::STAGING_PRACTICE_ID.into(),
                physician_id: "colleague".into(),
            },
            WORKSPACE_IDS[0],
            physician_auth::Membership::Allowed,
        )
        .await
        .unwrap();
    let sign = store
        .begin_sign_in("colleague@codahealth.test", auth::now())
        .await
        .unwrap();
    let (_, colleague) = store
        .complete_sign_in("colleague@codahealth.test", &sign, auth::now())
        .await
        .unwrap();
    let live = live::Live::on(
        live::ChatModel::Fixed("Noted.".into()),
        PhysicianSession::memory(),
        vec![],
    )
    .unwrap();
    let app = auth::required_router(live_router_on(live), auth);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let client = reqwest::Client::new();
    let workspace = WORKSPACE_IDS[0];
    let read = || async {
        client
            .get(format!("{base}/patient-workspaces/{workspace}"))
            .bearer_auth(&token)
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap()
    };
    assert_eq!(read().await["chats"], json!([]));
    let long =
        "Please review the potassium trend and tell me whether the latest result changes the plan";
    let sent = client
        .post(format!("{base}/commands"))
        .bearer_auth(&token)
        .json(
            &json!({"type":"message.submit","idempotency_key":"inline-chat-list","payload":{
                "patient_workspace_id": workspace,
                "chat_id": "chat-inline",
                "text": long,
                "physician_time_zone": "UTC"
            }}),
        )
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();
    assert_eq!(sent["outcome"], "accepted", "{sent}");
    let listed = read().await["chats"].clone();
    let chats = listed.as_array().expect("chats");
    assert_eq!(chats.len(), 1, "{listed}");
    assert_eq!(chats[0]["chat_id"], "chat-inline");
    assert_eq!(chats[0]["title"], live::chat_title(long));
    assert!(
        chats[0]["created_at"]
            .as_str()
            .is_some_and(|at| !at.starts_with("1970")),
        "{listed}"
    );
    let colleague_page = client
        .get(format!("{base}/patient-workspaces/{workspace}"))
        .bearer_auth(&colleague)
        .send()
        .await
        .unwrap();
    assert_eq!(colleague_page.status(), StatusCode::OK);
    assert_eq!(
        colleague_page.json::<Value>().await.unwrap()["chats"],
        json!([])
    );
    let snapshot = format!("{base}/patient-workspaces/{workspace}/chats/chat-inline/snapshot");
    assert_eq!(
        client
            .get(&snapshot)
            .bearer_auth(&colleague)
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap()["snapshot"]["conversation"],
        json!([]),
        "patient access must not reveal another physician's conversation"
    );
    // Even the same conversation and idempotency IDs name independent work
    // for another physician. Both snapshots and the list consume that owner.
    let own_text = "My independent question";
    let sent = client.post(format!("{base}/commands")).bearer_auth(&colleague)
        .json(&json!({"type":"message.submit","idempotency_key":"inline-chat-list","payload":{
            "patient_workspace_id":workspace,"chat_id":"chat-inline","text":own_text,"physician_time_zone":"UTC"
        }})).send().await.unwrap().json::<Value>().await.unwrap();
    assert_eq!(sent["outcome"], "accepted", "{sent}");
    for (session, expected, excluded) in [(&token, long, own_text), (&colleague, own_text, long)] {
        let body = client
            .get(&snapshot)
            .bearer_auth(session)
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();
        let texts: Vec<_> = body["snapshot"]["conversation"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|line| line["text"].as_str())
            .collect();
        assert!(texts.contains(&expected), "{body}");
        assert!(!texts.contains(&excluded), "{body}");
    }
    task.abort();
}

#[tokio::test]
async fn inline_chat_draft_saves_under_a_revision_for_one_physician() {
    let store = Arc::new(MemoryAuthStore::new());
    let auth = auth::Auth::over_fixture(
        store.clone(),
        Arc::new(front_door::MemoryFrontDoorStore::new()),
    )
    .await;
    let mut tokens = Vec::new();
    for (physician, email, admin) in [
        ("staging-physician", "physician@codahealth.test", true),
        ("colleague", "colleague@codahealth.test", false),
    ] {
        store.seed_account(auth::STAGING_PRACTICE_ID, physician, email, "Name", admin);
        let sign = store.begin_sign_in(email, auth::now()).await.unwrap();
        tokens.push(
            store
                .complete_sign_in(email, &sign, auth::now())
                .await
                .unwrap()
                .1,
        );
    }
    let workspace = WORKSPACE_IDS[0];
    store
        .set_membership(
            &physician_auth::PhysicianIdentity {
                practice_id: auth::STAGING_PRACTICE_ID.into(),
                physician_id: "colleague".into(),
            },
            workspace,
            physician_auth::Membership::Allowed,
        )
        .await
        .unwrap();
    let (token, colleague) = (&tokens[0], &tokens[1]);
    let live = live::Live::on(
        live::ChatModel::Fixed("Noted.".into()),
        PhysicianSession::memory(),
        vec![],
    )
    .unwrap();
    let app = auth::required_router(live_router_on(live), auth);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let client = reqwest::Client::new();
    let route = format!("{base}/physician-state/drafts/{workspace}/draft");
    let read = |session: &str| client.get(&route).bearer_auth(session).send();
    let save = |session: &str, expected: u64, text: &str| {
        client
            .put(&route)
            .bearer_auth(session)
            .json(&json!({"expected_revision": expected, "text": text}))
            .send()
    };
    let empty = read(token).await.unwrap();
    assert_eq!(empty.status(), StatusCode::OK);
    assert_eq!(
        empty.json::<Value>().await.unwrap(),
        json!({"revision": 0, "text": ""})
    );
    let first = save(token, 0, "Check potassium").await.unwrap();
    assert_eq!(first.status(), StatusCode::OK);
    assert_eq!(
        first.json::<Value>().await.unwrap(),
        json!({"revision": 1, "text": "Check potassium"})
    );
    let second = save(token, 1, "Check potassium and sodium").await.unwrap();
    assert_eq!(
        second.json::<Value>().await.unwrap(),
        json!({"revision": 2, "text": "Check potassium and sodium"})
    );
    // A stale save writes nothing and answers with the current draft.
    let stale = save(token, 1, "lost wording").await.unwrap();
    assert_eq!(stale.status(), StatusCode::CONFLICT);
    assert_eq!(
        stale.json::<Value>().await.unwrap(),
        json!({"outcome": "conflict", "current": {"revision": 2, "text": "Check potassium and sodium"}})
    );
    assert_eq!(
        read(token).await.unwrap().json::<Value>().await.unwrap()["text"],
        "Check potassium and sodium"
    );
    // Another physician on the same patient keeps their own draft.
    assert_eq!(
        read(colleague)
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap(),
        json!({"revision": 0, "text": ""})
    );
    let own = save(colleague, 0, "Colleague note").await.unwrap();
    assert_eq!(own.status(), StatusCode::OK);
    assert_eq!(
        read(token).await.unwrap().json::<Value>().await.unwrap()["text"],
        "Check potassium and sodium"
    );
    // The draft does not fence the chat it belongs to.
    let sent = client
        .post(format!("{base}/commands"))
        .bearer_auth(token)
        .json(
            &json!({"type":"message.submit","idempotency_key":"after-draft","payload":{
                "patient_workspace_id": workspace,
                "chat_id": "draft",
                "text": "Check potassium and sodium",
                "physician_time_zone": "UTC"
            }}),
        )
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();
    assert_eq!(sent["outcome"], "accepted", "{sent}");
    // Outside the colleague's panel, and without a session.
    let outside = format!("{base}/physician-state/drafts/{}/draft", WORKSPACE_IDS[1]);
    for request in [
        client.get(&outside).bearer_auth(colleague),
        client
            .put(&outside)
            .bearer_auth(colleague)
            .json(&json!({"expected_revision": 0, "text": "x"})),
    ] {
        assert_eq!(
            request.send().await.unwrap().status(),
            StatusCode::NOT_FOUND
        );
    }
    assert_eq!(
        client.get(&route).send().await.unwrap().status(),
        StatusCode::UNAUTHORIZED
    );
    task.abort();

    // Sign-in off: one anonymous local draft, apart from any physician's.
    let open = live::Live::on(
        live::ChatModel::Fixed("Noted.".into()),
        PhysicianSession::memory(),
        vec![],
    )
    .unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let task =
        tokio::spawn(async move { axum::serve(listener, live_router_on(open)).await.unwrap() });
    let route = format!("{base}/physician-state/drafts/{workspace}/draft");
    let saved = client
        .put(&route)
        .json(&json!({"expected_revision": 0, "text": "Local note"}))
        .send()
        .await
        .unwrap();
    assert_eq!(saved.status(), StatusCode::OK);
    let again = client
        .put(&route)
        .json(&json!({"expected_revision": 1, "text": "Local note, edited"}))
        .send()
        .await
        .unwrap();
    assert_eq!(
        again.json::<Value>().await.unwrap(),
        json!({"revision": 2, "text": "Local note, edited"})
    );
    let stale = client
        .put(&route)
        .json(&json!({"expected_revision": 1, "text": "lost"}))
        .send()
        .await
        .unwrap();
    assert_eq!(stale.status(), StatusCode::CONFLICT);
    task.abort();
}

#[test]
fn chat_titles_are_one_line_and_cut_at_a_word() {
    assert_eq!(live::chat_title("  About\n the   labs "), "About the labs");
    let title = live::chat_title(
        "Please review the potassium trend and tell me whether the latest result changes the plan",
    );
    assert_eq!(
        title,
        "Please review the potassium trend and tell me whether the…"
    );
    assert!(title.chars().count() <= live::CHAT_TITLE_CHARS + 1);
    let unbroken = "x".repeat(80);
    assert_eq!(
        live::chat_title(&unbroken).chars().count(),
        live::CHAT_TITLE_CHARS + 1
    );
}

/// Signs one seeded account in on the memory auth store.
async fn sign_in_memory(
    store: &MemoryAuthStore,
    email: &str,
) -> (physician_auth::PhysicianIdentity, String) {
    let sign = store.begin_sign_in(email, auth::now()).await.unwrap();
    store
        .complete_sign_in(email, &sign, auth::now())
        .await
        .unwrap()
}

#[tokio::test]
async fn the_synthetic_reply_opens_once_under_authority_and_only_when_enabled() {
    let store = Arc::new(MemoryAuthStore::new());
    store.seed_account(
        auth::STAGING_PRACTICE_ID,
        "staging-physician",
        "physician@codahealth.test",
        "Physician",
        true,
    );
    store.seed_account(
        auth::STAGING_PRACTICE_ID,
        "colleague",
        "colleague@codahealth.test",
        "Colleague",
        false,
    );
    let auth = auth::Auth::over_fixture(
        store.clone(),
        Arc::new(front_door::MemoryFrontDoorStore::new()),
    )
    .await;
    let (_, token) = sign_in_memory(&store, "physician@codahealth.test").await;
    let (colleague_identity, colleague) = sign_in_memory(&store, "colleague@codahealth.test").await;
    let workspace = crate::WORKSPACE_IDS[0];
    let serve = |live: Arc<live::Live>| {
        let app = auth::required_router(live_router_on(live), auth.clone());
        async move {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let base = format!("http://{}", listener.local_addr().unwrap());
            let task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
            (base, task)
        }
    };
    let client = reqwest::Client::new();
    let replies = |base: &str, token: &str| {
        let request = client
            .get(format!("{base}/patient-workspaces/{workspace}"))
            .bearer_auth(token);
        async move {
            let response = request.send().await.unwrap();
            let status = response.status();
            let body = response.json::<Value>().await.unwrap_or_default();
            (status, body["replies"].clone())
        }
    };

    // Off by default: the workspace has no reply and none is opened.
    let off = live::Live::on(
        live::ChatModel::Fixed("unused".into()),
        PhysicianSession::memory(),
        vec![],
    )
    .unwrap();
    let (base, task) = serve(off.clone()).await;
    let (status, listed) = replies(&base, &token).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        listed.as_array().is_none_or(Vec::is_empty),
        "switch off must list no reply: {listed}"
    );
    let target = reply::target(
        &auth::AuthContext {
            auth: auth.clone(),
            token: token.clone(),
            identity: physician_auth::PhysicianIdentity {
                practice_id: auth::STAGING_PRACTICE_ID.into(),
                physician_id: "staging-physician".into(),
            },
        },
        synthetic_reply::REPLY_ID,
    );
    assert!(
        off.session
            .lock()
            .unwrap()
            .draft(workspace, &target)
            .is_none()
    );
    task.abort();

    let live = live::Live::on(
        live::ChatModel::Fixed("unused".into()),
        PhysicianSession::memory(),
        vec![],
    )
    .unwrap();
    live.enable_synthetic_reply(workspace);
    let (base, task) = serve(live.clone()).await;
    let (status, listed) = replies(&base, &token).await;
    assert_eq!(status, StatusCode::OK, "{listed}");
    let reply = &listed[0];
    assert_eq!(reply["reply_work_id"], synthetic_reply::REPLY_ID);
    assert_eq!(reply["draft_text"], synthetic_reply::FIRST_DRAFT);
    assert_eq!(reply["draft_revision"], 1);
    assert_eq!(reply["revision_run"], Value::Null);
    assert_eq!(reply["state"], "ready");

    // A physician save advances the draft; opening again keeps it.
    let route = format!(
        "{base}/physician-state/reply-drafts/{workspace}/{}",
        synthetic_reply::REPLY_ID
    );
    let saved = client
        .put(&route)
        .bearer_auth(&token)
        .json(&json!({"revision": 1, "text": "Physician's own wording."}))
        .send()
        .await
        .unwrap();
    assert_eq!(saved.status(), StatusCode::OK);
    for _ in 0..2 {
        let (_, listed) = replies(&base, &token).await;
        assert_eq!(listed[0]["draft_text"], "Physician's own wording.");
        assert_eq!(listed[0]["draft_revision"], 2);
    }

    // Another physician with patient access gets their own draft, never this one.
    store
        .set_membership(
            &colleague_identity,
            workspace,
            physician_auth::Membership::Allowed,
        )
        .await
        .unwrap();
    let (_, theirs) = replies(&base, &colleague).await;
    assert_eq!(theirs[0]["draft_text"], synthetic_reply::FIRST_DRAFT);
    assert_eq!(theirs[0]["draft_revision"], 1);

    // Off the panel, a same-clinic physician sees demographics only: no
    // reply is projected, and none can be read or saved.
    store
        .set_membership(
            &colleague_identity,
            workspace,
            physician_auth::Membership::Denied,
        )
        .await
        .unwrap();
    let (status, listed) = replies(&base, &colleague).await;
    assert_eq!(status, StatusCode::OK);
    assert!(listed.as_array().is_none_or(Vec::is_empty), "{listed}");
    let route_status = client
        .get(&route)
        .bearer_auth(&colleague)
        .send()
        .await
        .unwrap()
        .status();
    assert_eq!(route_status, StatusCode::NOT_FOUND);
    let refused = client
        .put(&route)
        .bearer_auth(&colleague)
        .json(&json!({"revision": 1, "text": "demographic-only overwrite"}))
        .send()
        .await
        .unwrap();
    assert_ne!(refused.status(), StatusCode::OK);

    // A missing reply cannot be created through a save or a revision.
    let missing = format!("{base}/physician-state/reply-drafts/{workspace}/not-a-reply");
    let refused = client
        .put(&missing)
        .bearer_auth(&token)
        .json(&json!({"revision": 0, "text": "invented"}))
        .send()
        .await
        .unwrap();
    assert_eq!(refused.status(), StatusCode::CONFLICT);
    assert_eq!(
        client
            .get(&missing)
            .bearer_auth(&token)
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::NOT_FOUND
    );
    task.abort();
}
