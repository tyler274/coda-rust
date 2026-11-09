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
        .submit(
            &binding(),
            "one",
            ChatScope {
                owner: None,
                workspace: "workspace",
                chat: "chat",
            },
            "original",
        )
        .unwrap()
        .unwrap();
    let physician = ChatOwner {
        practice: "practice-a".into(),
        physician: "me".into(),
    };
    let other_practice = ChatOwner {
        practice: "practice-b".into(),
        physician: "me".into(),
    };
    for (identity, question) in [
        (&physician, "private question"),
        (&other_practice, "other practice question"),
    ] {
        let scoped = owner
            .submit(
                &binding(),
                "one",
                ChatScope {
                    owner: Some(identity),
                    workspace: "workspace",
                    chat: "chat",
                },
                question,
            )
            .unwrap()
            .unwrap();
        owner.complete(&scoped, "scoped reply").unwrap();
    }
    assert!(
        owner
            .submit(
                &binding(),
                "one",
                ChatScope {
                    owner: None,
                    workspace: "workspace",
                    chat: "chat"
                },
                "original"
            )
            .unwrap()
            .is_none()
    );
    assert!(
        owner
            .submit(
                &binding(),
                "one",
                ChatScope {
                    owner: None,
                    workspace: "workspace",
                    chat: "chat"
                },
                "changed"
            )
            .is_err()
    );
    let newer = owner
        .submit(
            &binding(),
            "two",
            ChatScope {
                owner: None,
                workspace: "workspace",
                chat: "chat",
            },
            "new input",
        )
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
        .submit(
            &binding(),
            "three",
            ChatScope {
                owner: None,
                workspace: "workspace",
                chat: "chat",
            },
            "unfinished",
        )
        .unwrap()
        .unwrap();
    drop(owner);
    let mut owner = PhysicianSession::open(&path).unwrap();
    assert_eq!(owner.work(&pending).unwrap().status, Status::Interrupted);
    assert_eq!(owner.work(&newer).unwrap().status, Status::Completed);
    assert_eq!(owner.work(&old).unwrap().records.len(), 1);
    assert!(owner.complete(&pending, "late after restart").is_err());
    assert_eq!(
        owner.conversation(ChatScope {
            owner: None,
            workspace: "workspace",
            chat: "chat"
        }),
        said(&[
            ("user", "original"),
            ("user", "new input"),
            ("assistant", "current reply"),
            ("user", "unfinished"),
        ])
    );
    for (identity, question) in [
        (&physician, "private question"),
        (&other_practice, "other practice question"),
    ] {
        assert_eq!(
            owner.conversation(ChatScope {
                owner: Some(identity),
                workspace: "workspace",
                chat: "chat",
            }),
            said(&[("user", question), ("assistant", "scoped reply")])
        );
        assert_eq!(owner.chats(Some(identity), "workspace")[0].1, question);
    }
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

#[test]
fn opening_reply_work_is_idempotent_and_keeps_edits_and_runs() {
    let path = path();
    let mut session = PhysicianSession::open(&path).unwrap();
    for blank in [("", "p/r", "text"), ("w", " ", "text"), ("w", "p/r", " ")] {
        assert!(session.open_reply_draft(blank.0, blank.1, blank.2).is_err());
    }
    assert!(session.draft("w", "p/r").is_none());
    assert_eq!(session.open_reply_draft("w", "p/r", "first").unwrap(), 1);
    assert_eq!(session.open_reply_draft("w", "p/r", "other").unwrap(), 1);
    assert_eq!(
        session.draft("w", "p/r").unwrap(),
        (1, "first".into(), None)
    );
    assert!(session.reply_run("w", "p/r").is_none());

    assert_eq!(session.save_draft("w", "p/r", 1, "edited").unwrap(), 2);
    let work = session.revise("run", "w", "p/r", 2, request()).unwrap();
    assert_eq!(session.open_reply_draft("w", "p/r", "first").unwrap(), 2);
    assert_eq!(
        session.draft("w", "p/r").unwrap(),
        (2, "edited".into(), Some("first".into()))
    );
    assert_eq!(
        session.reply_run("w", "p/r"),
        Some(("run".into(), Status::Active))
    );
    session.complete(&work, "revised").unwrap();
    assert_eq!(
        session.reply_run("w", "p/r"),
        Some(("run".into(), Status::Completed))
    );
    // Another workspace or physician target is separate reply work.
    assert_eq!(session.open_reply_draft("w2", "p/r", "first").unwrap(), 1);
    assert_eq!(session.open_reply_draft("w", "q/r", "first").unwrap(), 1);
    assert!(session.reply_run("w2", "p/r").is_none());
    drop(session);
    let session = PhysicianSession::open(&path).unwrap();
    assert_eq!(session.draft("w", "p/r").unwrap().0, 3);
    drop(session);
    cleanup(&path);
}

#[tokio::test]
async fn shared_loop_commits_persistent_physician_work_and_keeps_provider_evidence() {
    let path = path();
    let mut session = PhysicianSession::open(&path).unwrap();
    let work = session
        .submit(
            &binding(),
            "engine",
            ChatScope {
                owner: None,
                workspace: "w",
                chat: "c",
            },
            "hello",
        )
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
    assert_eq!(
        session.conversation(ChatScope {
            owner: None,
            workspace: "w",
            chat: "c"
        }),
        said(&[("user", "hello"), ("assistant", "authored reply")])
    );
    drop(session);
    cleanup(&path);
}

#[test]
fn corrupt_owner_never_resets_and_cancelled_work_never_resumes() {
    let path = path();
    let mut session = PhysicianSession::open(&path).unwrap();
    let work = session
        .submit(
            &binding(),
            "one",
            ChatScope {
                owner: None,
                workspace: "w",
                chat: "c",
            },
            "hello",
        )
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

#[test]
fn chats_list_conversations_newest_first_and_skip_reply_drafts() {
    let mut session = PhysicianSession::memory();
    session
        .submit(
            &binding(),
            "k1",
            ChatScope {
                owner: None,
                workspace: "ws",
                chat: "chat-old",
            },
            "First question",
        )
        .unwrap();
    session
        .submit(
            &binding(),
            "k2",
            ChatScope {
                owner: None,
                workspace: "ws",
                chat: "chat-old",
            },
            "Follow-up",
        )
        .unwrap();
    session
        .submit(
            &binding(),
            "k3",
            ChatScope {
                owner: None,
                workspace: "other",
                chat: "chat-elsewhere",
            },
            "Not here",
        )
        .unwrap();
    session.save_draft("ws", "reply-draft", 0, "draft").unwrap();
    session
        .submit(
            &binding(),
            "k4",
            ChatScope {
                owner: None,
                workspace: "ws",
                chat: "chat-new",
            },
            "Second chat",
        )
        .unwrap();
    let chats = session.chats(None, "ws");
    let listed: Vec<_> = chats
        .iter()
        .map(|(chat, first, _, archived)| {
            assert!(!archived);
            (chat.as_str(), first.as_str())
        })
        .collect();
    assert_eq!(
        listed,
        [("chat-new", "Second chat"), ("chat-old", "First question")]
    );
    assert!(chats.iter().all(|(_, _, opened_at, _)| *opened_at > 0));
    assert!(session.chats(None, "missing").is_empty());
}

/// A conversation as the session returns it: who wrote each turn, and its words.
fn said(turns: &[(&str, &str)]) -> Vec<(String, String)> {
    turns
        .iter()
        .map(|(role, text)| ((*role).to_owned(), (*text).to_owned()))
        .collect()
}

#[test]
fn archive_and_undo_each_take_effect_once_and_wait_for_running_work() {
    use local_product::runtime::{Archive, Archived};
    let mut session = PhysicianSession::memory();
    let scope = || ChatScope {
        owner: None,
        workspace: "ws",
        chat: "chat",
    };
    let archived = |session: &PhysicianSession| session.chats(None, "ws")[0].3;
    let work = session
        .submit(&binding(), "send-1", scope(), "Question")
        .unwrap()
        .unwrap();
    assert_eq!(
        session.archive("archive-1", scope(), Archive::Archive),
        Ok(Archived::Busy),
        "an answer being written keeps the conversation active"
    );
    assert!(!archived(&session));
    session.complete(&work, "Answer").unwrap();

    assert_eq!(
        session.archive("archive-1", scope(), Archive::Archive),
        Ok(Archived::Done),
        "the same key is asked again once the answer ends"
    );
    assert!(archived(&session));
    assert_eq!(
        session.archive("undo-1", scope(), Archive::Restore),
        Ok(Archived::Done)
    );
    assert!(!archived(&session));
    // The Archive's answer was lost and it is asked again after Undo: it was
    // already taken, so the Undo stands.
    assert_eq!(
        session.archive("archive-1", scope(), Archive::Archive),
        Ok(Archived::Duplicate)
    );
    assert!(!archived(&session));
    assert_eq!(
        session.archive("archive-2", scope(), Archive::Archive),
        Ok(Archived::Done)
    );
    // Undo's lost answer re-asked after a newer Archive leaves that Archive.
    assert_eq!(
        session.archive("undo-1", scope(), Archive::Restore),
        Ok(Archived::Duplicate)
    );
    assert!(archived(&session));
    assert_eq!(
        session.archive("archive-2", scope(), Archive::Restore),
        Ok(Archived::Refused),
        "one key never names both directions"
    );
    assert_eq!(
        session.archive(
            "archive-3",
            ChatScope {
                chat: "missing",
                ..scope()
            },
            Archive::Archive
        ),
        Ok(Archived::Refused)
    );

    // Sending brings it back.
    session
        .submit(&binding(), "send-2", scope(), "Another")
        .unwrap();
    assert!(!archived(&session));
}
