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
