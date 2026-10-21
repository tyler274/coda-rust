use harness::execution::*;
use local_product::{ModelBinding, execution::*};
use serde_json::json;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
fn path() -> PathBuf {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    std::env::temp_dir().join(format!(
        "physician-execution-{}-{}.json",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ))
}
fn binding() -> ModelBinding {
    ModelBinding {
        model: "controlled".into(),
        effort: "high".into(),
        adapter: "test".into(),
    }
}
fn request() -> Request {
    Request {
        model: "controlled".into(),
        options: json!({}),
        instructions: "saved revision and instruction".into(),
        tools: vec![],
        items: vec![],
    }
}
fn response(calls: Vec<Call>) -> Response {
    Response {
        text: String::new(),
        calls,
        usage: Usage {
            input_tokens: Some(2),
            output_tokens: Some(3),
        },
        provenance: json!({"mode": "controlled"}),
    }
}
fn cleanup(path: &std::path::Path) {
    std::fs::remove_file(path).unwrap();
    std::fs::remove_file(path.with_extension("lock")).unwrap();
}
#[test]
fn durable_admission_restart_and_late_completion_use_one_owner() {
    let path = path();
    let mut owner = PhysicianSession::open(&path).unwrap();
    assert!(
        PhysicianSession::open(&path).is_err(),
        "single writer is enforced"
    );
    let old = owner
        .submit(&binding(), "one", "workspace", "chat", "original")
        .unwrap()
        .unwrap();
    assert!(
        owner
            .submit(&binding(), "one", "workspace", "chat", "original")
            .unwrap()
            .is_none()
    );
    assert!(
        owner
            .submit(&binding(), "one", "workspace", "chat", "changed")
            .is_err()
    );
    let newer = owner
        .submit(&binding(), "two", "workspace", "chat", "new input")
        .unwrap()
        .unwrap();
    assert!(owner.complete(&old, "late reply").is_err());
    owner
        .record(
            &old,
            Record::Exchange {
                sequence: 1,
                response: response(vec![]),
            },
        )
        .unwrap();
    owner.complete(&newer, "current reply").unwrap();
    let pending = owner
        .submit(&binding(), "three", "workspace", "chat", "unfinished")
        .unwrap()
        .unwrap();
    drop(owner);
    let mut owner = PhysicianSession::open(&path).unwrap();
    assert_eq!(owner.work(&pending).unwrap().status, Status::Interrupted);
    assert_eq!(owner.work(&newer).unwrap().status, Status::Completed);
    assert_eq!(owner.work(&old).unwrap().records.len(), 1);
    assert!(owner.complete(&pending, "late after restart").is_err());
    assert_eq!(
        owner.conversation("workspace", "chat"),
        ["original", "new input", "current reply", "unfinished"]
    );
    drop(owner);
    cleanup(&path);
}
#[tokio::test]
async fn revision_denies_memory_and_delegation_and_only_commits_current_terminal() {
    let path = path();
    let mut session = PhysicianSession::open(&path).unwrap();
    session
        .save_draft("workspace", "physician/reply", 0, "original draft")
        .unwrap();
    let work = session
        .revise("revise", "workspace", "physician/reply", 1, request())
        .unwrap();
    let terminal = response(vec![Call {
        id: "finish".into(),
        name: "finish_reply_revision".into(),
        arguments: json!({"text": "revised draft", "review_basis": "source basis"}),
    }]);
    {
        let mut owner = PhysicianOwner {
            session: &mut session,
            work: work.clone(),
        };
        for name in ["memory_write", "delegate", "working_notes"] {
            let result = owner
                .dispatch(
                    &Call {
                        id: name.into(),
                        name: name.into(),
                        arguments: json!({}),
                    },
                    &Cancellation::default(),
                )
                .await
                .unwrap();
            assert_eq!(result["error"], "tool_not_authorized");
        }
        assert!(owner.complete(&response(vec![])).await.is_err());
        assert!(owner.terminal(&terminal));
        owner.complete(&terminal).await.unwrap();
    }
    assert_eq!(
        session.draft("workspace", "physician/reply").unwrap(),
        (2, "revised draft".into(), Some("original draft".into()))
    );
    let stale = session
        .revise("later", "workspace", "physician/reply", 2, request())
        .unwrap();
    session
        .save_draft("workspace", "physician/reply", 2, "new physician wording")
        .unwrap();
    assert!(
        PhysicianOwner {
            session: &mut session,
            work: stale
        }
        .complete(&terminal)
        .await
        .is_err()
    );
    drop(session);
    let session = PhysicianSession::open(&path).unwrap();
    assert_eq!(
        session.draft("workspace", "physician/reply").unwrap().1,
        "new physician wording"
    );
    assert_eq!(session.work(&work).unwrap().status, Status::Completed);
    drop(session);
    cleanup(&path);
}
#[test]
fn reply_close_fences_active_revision_without_replacing_saved_text() {
    let mut session = PhysicianSession::memory();
    session.save_draft("w", "p/r", 0, "saved").unwrap();
    let work = session
        .revise("revision", "w", "p/r", 1, request())
        .unwrap();
    session.close_reply("w", "p/r").unwrap();
    assert!(session.complete(&work, "late").is_err());
    assert_eq!(session.work(&work).unwrap().status, Status::Failed);
    assert_eq!(session.draft("w", "p/r").unwrap().1, "saved");
}

#[tokio::test]
async fn shared_loop_commits_persistent_physician_work_and_keeps_provider_evidence() {
    let path = path();
    let mut session = PhysicianSession::open(&path).unwrap();
    let work = session
        .submit(&binding(), "engine", "w", "c", "hello")
        .unwrap()
        .unwrap();
    let mut answer = response(vec![]);
    answer.text = "authored reply".into();
    let mut replay = harness::transport::Replay::new(harness::transport::Cassette {
        schema_version: 2,
        adapter: "controlled".into(),
        authorization: "synthetic test".into(),
        synthetic_non_phi: true,
        exchanges: vec![(work.request.clone(), answer)],
    })
    .unwrap();
    run(
        &mut replay,
        &mut PhysicianOwner {
            session: &mut session,
            work: work.clone(),
        },
        work.request.clone(),
        &mut Progress::default(),
        Limits::default(),
        &Cancellation::default(),
        &mut |_| Ok(()),
    )
    .await
    .unwrap();
    drop(session);
    let session = PhysicianSession::open(&path).unwrap();
    assert_eq!(session.work(&work).unwrap().status, Status::Completed);
    assert!(
        session
            .work(&work)
            .unwrap()
            .records
            .iter()
            .any(|record| matches!(record, Record::Exchange { .. }))
    );
    assert_eq!(session.conversation("w", "c"), ["hello", "authored reply"]);
    drop(session);
    cleanup(&path);
}

#[test]
fn corrupt_owner_never_resets_and_cancelled_work_never_resumes() {
    let path = path();
    let mut session = PhysicianSession::open(&path).unwrap();
    let work = session
        .submit(&binding(), "one", "w", "c", "hello")
        .unwrap()
        .unwrap();
    session.settle(&work, Status::Cancelled).unwrap();
    drop(session);
    let session = PhysicianSession::open(&path).unwrap();
    assert_eq!(session.work(&work).unwrap().status, Status::Cancelled);
    drop(session);
    std::fs::write(&path, b"corrupt").unwrap();
    assert!(PhysicianSession::open(&path).is_err());
    cleanup(&path);
}
