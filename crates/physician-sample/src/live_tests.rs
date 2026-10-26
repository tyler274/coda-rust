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
    let auth = auth::Auth::on(
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
        candidate_id: "source-one".into(),
        revision: "immutable-7".into(),
        content: "Potassium 4.1 mmol/L".into(),
        sha256: local_product::sha256_hex(b"Potassium 4.1 mmol/L"),
        provenance: json!({"native_record":"lab-7"}),
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
    let auth = auth::Auth::on(
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
        .await;
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
