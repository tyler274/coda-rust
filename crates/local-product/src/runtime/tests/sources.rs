//! Postgres proof that a reply revision takes in sources arriving before it
//! finishes: the versioned source store, per-attempt pins, refreshes recorded
//! on the attempt, and ingestion serialized against publication.
use super::*;
use crate::physician_sources::{
    ReadablePart, RecordDate, SourcePatient, SourceRevision, exact_read,
};
use crate::runtime::reply::{DraftStatus, FINISH_TOOL, RevisionCommit, RevisionInput};
use crate::runtime::sources::SourcePull;
use harness::engine::recipe::Usage;
use harness::engine::{Event, EventKind};
use harness::engine::{Invocation, Item, Turn};

async fn setup() -> (Store, Scope) {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    // The source version is per workspace, so each test gets its own.
    let scope = Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "physician".into(),
        workspace: uuid::Uuid::new_v4().to_string(),
        chat: "reply".into(),
    };
    (store, scope)
}

fn source(scope: &Scope, id: &str, revision: &str, content: &str) -> SourceRevision {
    SourceRevision {
        workspace: scope.workspace.clone(),
        source_id: "ehr".into(),
        candidate_id: id.into(),
        revision: revision.into(),
        record_type: "Observation".into(),
        dates: vec![],
        content: content.into(),
        sha256: crate::sha256_hex(content.as_bytes()),
        readable_parts: vec![],
        provenance: serde_json::json!({"native_record": id}),
        native: None,
    }
}

fn input(key: &str, revision: u64) -> RevisionInput {
    RevisionInput {
        key: key.into(),
        expected_revision: revision,
        instruction: "Mention the new results".into(),
        timezone: "UTC".into(),
        scenario: None,
    }
}

fn finish(id: &str, text: &str) -> Turn {
    Turn {
        items: vec![
            Invocation {
                id: id.into(),
                name: FINISH_TOOL.into(),
                arguments: serde_json::json!({"text": text, "review_basis": "Read sources"}),
            }
            .into(),
        ],
        usage: Usage::default(),
        receipt: serde_json::json!({"synthetic": true}),
    }
}

/// A model call whose context holds an exact read of `read`, then a finish.
async fn read_then_finish(
    store: &Store,
    claim: &reply::RevisionClaim,
    sequence: u64,
    read: &[SourceRevision],
    references: &[String],
    output: &Turn,
) {
    let outcome = exact_read(read, &claim.scope.workspace, references, 65536).unwrap();
    let value: Value = serde_json::from_str(outcome.text().unwrap()).unwrap();
    let count = store
        .draft(&claim.scope)
        .await
        .unwrap()
        .unwrap()
        .attempts
        .last()
        .unwrap()
        .events
        .len() as u64;
    store
        .record_revision(
            claim,
            Event {
                attempt_id: claim.attempt.clone(),
                sequence: count + 1,
                ordinal: sequence,
                kind: EventKind::ToolReturned {
                    call_id: format!("read-{sequence}"),
                    result: harness::engine::results::ToolResult::Original(value.to_string()),
                },
            },
        )
        .await
        .unwrap();
    store
        .record_revision(
            claim,
            Event {
                attempt_id: claim.attempt.clone(),
                sequence: count + 2,
                ordinal: sequence,
                kind: EventKind::ProviderStarted {
                    dispatched_at_unix_ms: 1,
                    request: harness::engine::SavedRequest {
                        added: vec![Item::Result {
                            call_id: format!("read-{sequence}"),
                            output: harness::engine::results::ResultView::whole(
                                format!("read-{sequence}"),
                                &value.to_string(),
                            ),
                        }],
                        items: vec![harness::engine::RequestItem::Event {
                            attempt: claim.attempt.clone(),
                            sequence: count + 2,
                            index: 0,
                        }],
                    },
                    context: Default::default(),
                },
            },
        )
        .await
        .unwrap();
    let previews: Vec<Value> = value["results"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|entry| entry["outcome"] == "available")
        .map(|entry| {
            serde_json::json!({
                "reference": entry["reference"],
                "revision": entry["revision"],
                "preview": format!("Reviewed {} ({}).", entry["reference"], entry["revision"]),
            })
        })
        .collect();
    if !previews.is_empty() {
        let all_events = store
            .revision_records(
                &claim.scope,
                &store.draft(&claim.scope).await.unwrap().unwrap(),
            )
            .await
            .unwrap();
        let evidence = crate::citations::WindowEvidence::from_records(
            all_events.iter(),
            &[],
            &claim.scope.workspace,
        )
        .unwrap();
        let publish = crate::citations::execute_publish_citations(
            &serde_json::json!({
                "previews": previews,
                "citations": [],
            }),
            &evidence,
            read,
        );
        store
            .record_revision(
                claim,
                Event {
                    attempt_id: claim.attempt.clone(),
                    sequence: count + 3,
                    ordinal: sequence,
                    kind: EventKind::ToolReturned {
                        call_id: format!("publish-{sequence}"),
                        result: harness::engine::results::ToolResult::Original(
                            publish.text().unwrap().to_owned(),
                        ),
                    },
                },
            )
            .await
            .unwrap();
    }
    exchange(store, claim, sequence, output).await;
}

async fn exchange(store: &Store, claim: &reply::RevisionClaim, ordinal: u64, output: &Turn) {
    let count = store
        .draft(&claim.scope)
        .await
        .unwrap()
        .unwrap()
        .attempts
        .last()
        .unwrap()
        .events
        .len() as u64;
    for (offset, kind) in [
        EventKind::ProviderReturned(output.clone()),
        EventKind::Candidate {
            text: output.text(),
        },
    ]
    .into_iter()
    .enumerate()
    {
        store
            .record_revision(
                claim,
                Event {
                    attempt_id: claim.attempt.clone(),
                    sequence: count + offset as u64 + 1,
                    ordinal,
                    kind,
                },
            )
            .await
            .unwrap();
    }
}

async fn accepted_claim(store: &Store, scope: &Scope, key: &str) -> reply::RevisionClaim {
    open_draft(store, scope, "Original").await;
    accept_revision_input(store, scope, &input(key, 1))
        .await
        .unwrap();
    store
        .claim_revision(&Delivery::reply_revision(scope.clone(), key.into()), 60)
        .await
        .unwrap()
        .unwrap()
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_record_comes_back_with_its_type_dates_and_readable_parts() {
    let (store, scope) = setup().await;
    let mut note = source(&scope, "note", "r1", r#"{"data":"U2VlbiBpbiBjbGluaWM="}"#);
    note.record_type = "DocumentReference".into();
    note.dates = vec![RecordDate {
        name: "date".into(),
        value: "2026-08".into(),
    }];
    note.readable_parts = vec![ReadablePart {
        label: "attachment".into(),
        text: "Seen in clinic".into(),
    }];
    store.record_source(&note).await.unwrap();
    let read = store.sources(&scope.workspace).await.unwrap();
    assert_eq!(read.sources, [note.clone()]);

    let mut changed = note;
    changed.record_type = "Observation".into();
    assert!(store.record_source(&changed).await.is_err());
}

fn pull(source_id: &str, pulled_at: i64) -> SourcePull {
    SourcePull {
        source_id: source_id.into(),
        source_label: source_id.to_uppercase(),
        patient: SourcePatient {
            source_patient_id: "patient-1".into(),
            display_name: "Mary Louis".into(),
            birth_date: Some("1972-03-18".into()),
            sex_or_gender: None,
            medical_record_numbers: vec![],
        },
        pulled_at,
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_later_pull_replaces_only_that_sources_last_pull() {
    let (store, scope) = setup().await;
    for delivery in [pull("ehr", 100), pull("mail", 200), pull("ehr", 300)] {
        store
            .record_chart(&scope.workspace, &[], &delivery)
            .await
            .unwrap();
    }
    assert_eq!(
        store
            .source_pulls(std::slice::from_ref(&scope.workspace))
            .await
            .unwrap()[&scope.workspace],
        [pull("ehr", 300), pull("mail", 200)]
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_chart_delivery_is_one_version_and_withdraws_what_the_source_dropped() {
    let (store, scope) = setup().await;
    let lab = source(&scope, "lab", "r1", "Potassium 4.1");
    let note = source(&scope, "note", "r1", "Seen in clinic");
    let mut message = source(&scope, "message", "r1", "From the patient");
    message.source_id = "mail".into();
    store.record_source(&message).await.unwrap();
    let delivery = pull("ehr", 100);
    let chart = [lab.clone(), note.clone()];
    assert_eq!(
        store
            .record_chart(&scope.workspace, &chart, &delivery)
            .await
            .unwrap(),
        2
    );
    let first = store.sources(&scope.workspace).await.unwrap();
    assert_eq!(first.version, 2);

    // A repeat delivery moves nothing.
    assert_eq!(
        store
            .record_chart(&scope.workspace, &chart, &delivery)
            .await
            .unwrap(),
        0
    );
    assert_eq!(store.sources(&scope.workspace).await.unwrap(), first);

    // The source dropped the note and corrected the lab. The other source's
    // record stays.
    let corrected = source(&scope, "lab", "r2", "Potassium 4.3");
    assert_eq!(
        store
            .record_chart(
                &scope.workspace,
                std::slice::from_ref(&corrected),
                &delivery
            )
            .await
            .unwrap(),
        2
    );
    let after = store.sources(&scope.workspace).await.unwrap();
    assert_eq!(after.version, 3);
    assert_eq!(after.sources, [corrected, message]);

    // The same identity with other bytes is refused, and the pull stays.
    let conflict = source(&scope, "lab", "r2", "Potassium 9.9");
    assert!(
        store
            .record_chart(&scope.workspace, &[conflict], &pull("ehr", 200))
            .await
            .is_err()
    );
    assert_eq!(
        store
            .source_pulls(std::slice::from_ref(&scope.workspace))
            .await
            .unwrap()[&scope.workspace],
        [delivery]
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn unchanged_imports_keep_the_version_and_changes_append() {
    let (store, scope) = setup().await;
    let lab = source(&scope, "lab", "r1", "Potassium 4.1");
    let note = source(&scope, "note", "r1", "Seen in clinic");
    assert_eq!(store.sources(&scope.workspace).await.unwrap().version, 0);
    assert!(
        !store
            .has_sources_outside(&scope.workspace, "ehr")
            .await
            .unwrap()
    );
    store
        .import_sources(&[lab.clone(), note.clone()])
        .await
        .unwrap();
    let first = store.sources(&scope.workspace).await.unwrap();
    assert_eq!(first.version, 2);
    assert!(
        !store
            .has_sources_outside(&scope.workspace, "ehr")
            .await
            .unwrap()
    );
    assert!(
        store
            .has_sources_outside(&scope.workspace, "synthetic-charts")
            .await
            .unwrap()
    );
    // A site restart imports the same snapshot again: nothing moves.
    store
        .import_sources(&[lab.clone(), note.clone()])
        .await
        .unwrap();
    assert_eq!(store.sources(&scope.workspace).await.unwrap(), first);
    let corrected = source(&scope, "lab", "r2", "Potassium 4.3");
    assert_eq!(store.record_source(&corrected).await.unwrap(), 3);
    assert_eq!(
        store
            .record_chart(
                &scope.workspace,
                std::slice::from_ref(&corrected),
                &pull("ehr", 100)
            )
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        store
            .record_chart(
                &scope.workspace,
                std::slice::from_ref(&corrected),
                &pull("ehr", 100)
            )
            .await
            .unwrap(),
        0
    );
    let now = store.sources(&scope.workspace).await.unwrap();
    assert_eq!(now.version, 4);
    assert_eq!(now.sources, vec![corrected]);
    assert!(
        store
            .has_sources_outside(&scope.workspace, "synthetic-charts")
            .await
            .unwrap()
    );
    store
        .record_chart(&scope.workspace, &[], &pull("ehr", 100))
        .await
        .unwrap();
    assert!(
        store
            .sources(&scope.workspace)
            .await
            .unwrap()
            .sources
            .is_empty(),
        "every record is withdrawn"
    );
    assert!(
        store
            .has_sources_outside(&scope.workspace, "synthetic-charts")
            .await
            .unwrap(),
        "answers written from withdrawn records stay in the workspace's history"
    );
    assert!(
        !store
            .has_sources_outside(&scope.workspace, "ehr")
            .await
            .unwrap()
    );
    // Another workspace's sources never appear.
    let other = Scope {
        workspace: uuid::Uuid::new_v4().to_string(),
        ..scope.clone()
    };
    assert!(
        store
            .sources(&other.workspace)
            .await
            .unwrap()
            .sources
            .is_empty()
    );
    let mut foreign = lab;
    foreign.sha256 = "0".repeat(64);
    assert!(store.record_source(&foreign).await.is_err());
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn restart_imports_do_not_rollback_corrections_or_restore_withdrawals() {
    let (store, scope) = setup().await;
    let original = source(&scope, "lab", "r1", "Potassium 4.1");
    let corrected = source(&scope, "lab", "r2", "Potassium 6.8");
    store
        .import_sources(std::slice::from_ref(&original))
        .await
        .unwrap();
    store.record_source(&corrected).await.unwrap();
    // A fresh site connects to the same store but still has its old snapshot.
    let restarted = Store::connect(&std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    let before = store.sources(&scope.workspace).await.unwrap();
    restarted
        .import_sources(std::slice::from_ref(&original))
        .await
        .unwrap();
    assert_eq!(restarted.sources(&scope.workspace).await.unwrap(), before);
    store
        .record_chart(&scope.workspace, &[], &pull("ehr", 100))
        .await
        .unwrap();
    let withdrawn = store.sources(&scope.workspace).await.unwrap();
    restarted
        .import_sources(&[original, corrected])
        .await
        .unwrap();
    assert_eq!(
        restarted.sources(&scope.workspace).await.unwrap(),
        withdrawn
    );
    let restored = source(&scope, "lab", "r3", "Potassium 4.2");
    assert_eq!(restarted.record_source(&restored).await.unwrap(), 4);
    assert_eq!(
        store.sources(&scope.workspace).await.unwrap().sources,
        vec![restored]
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn source_identity_conflicts_are_refused_throughout_history() {
    let (store, scope) = setup().await;
    let original = source(&scope, "lab", "r1", "Potassium 4.1");
    store.record_source(&original).await.unwrap();
    for phase in 0..3 {
        let before = store.sources(&scope.workspace).await.unwrap();
        let changed_bytes = source(&scope, "lab", "r1", "Potassium 6.8");
        let mut changed_provenance = original.clone();
        changed_provenance.provenance = serde_json::json!({"native_record": "another-lab"});
        for conflict in [changed_bytes, changed_provenance] {
            assert!(store.record_source(&conflict).await.is_err());
            assert_eq!(store.sources(&scope.workspace).await.unwrap(), before);
        }
        if phase == 0 {
            store
                .record_source(&source(&scope, "lab", "r2", "Corrected"))
                .await
                .unwrap();
        } else {
            store
                .record_chart(&scope.workspace, &[], &pull("ehr", 100))
                .await
                .unwrap();
        }
    }
    // Identity is scoped to the candidate and workspace, not revision alone.
    store
        .record_source(&source(&scope, "other-lab", "r1", "Other bytes"))
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn concurrent_conflicting_revisions_cannot_redefine_identity() {
    let (store, scope) = setup().await;
    let first = source(&scope, "lab", "r1", "Potassium 4.1");
    let second = source(&scope, "lab", "r1", "Potassium 6.8");
    let (a, b) = tokio::join!(store.record_source(&first), store.record_source(&second));
    assert_ne!(a.is_ok(), b.is_ok());
    let observed = store.sources(&scope.workspace).await.unwrap();
    assert_eq!(observed.version, 1);
    assert_eq!(
        observed.sources,
        vec![if a.is_ok() { first } else { second }]
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_source_arriving_mid_revision_must_be_read_before_publication() {
    let (store, scope) = setup().await;
    let lab = source(&scope, "lab-1", "r1", "Potassium 4.1");
    store.record_source(&lab).await.unwrap();
    let mut claim = accepted_claim(&store, &scope, "first").await;
    assert_eq!(claim.sources, vec![lab.clone()]);
    assert_eq!(claim.sources_version, 1);
    let early = finish("finish-1", "Potassium is normal.");
    exchange(&store, &claim, 1, &early).await;
    // A lab arrives after the model finished but before the commit.
    let arrived = source(&scope, "lab-2", "r1", "Creatinine 2.4 (high)");
    store.record_source(&arrived).await.unwrap();
    let RevisionCommit::SourcesMoved { refresh, sources } = store
        .commit_revision(&claim, &early, &mut OpenAuthority)
        .await
        .unwrap()
    else {
        panic!("a finish on an older reading must not publish");
    };
    assert_eq!((refresh.from_version, refresh.to_version), (1, 2));
    assert_eq!(refresh.added, vec![arrived.clone()]);
    assert!(refresh.corrected.is_empty() && refresh.withdrawn.is_empty());
    assert_eq!(sources, vec![lab.clone(), arrived.clone()]);
    let draft = store.draft(&scope).await.unwrap().unwrap();
    assert_eq!((draft.revision, draft.text.as_str()), (1, "Original"));
    assert_eq!(draft.status, DraftStatus::Active);
    assert_eq!(draft.sources_version, 2);
    let attempt = draft.attempts.last().unwrap();
    assert_eq!(attempt.sources, vec![lab.clone()], "the original pin stays");
    assert_eq!(attempt.refreshes, vec![refresh.clone()]);
    claim.sources = sources;
    claim.sources_version = refresh.to_version;
    // Finishing again without reading the new lab still publishes nothing.
    let unread = finish("finish-2", "Potassium is normal.");
    read_then_finish(
        &store,
        &claim,
        2,
        &claim.sources.clone(),
        &["lab-1".into()],
        &unread,
    )
    .await;
    assert_eq!(
        store
            .commit_revision(&claim, &unread, &mut OpenAuthority)
            .await
            .unwrap(),
        RevisionCommit::SourcesUnread(vec!["lab-2".into()])
    );
    assert_eq!(store.draft(&scope).await.unwrap().unwrap().revision, 1);
    // A retained read with the right identity but different bytes or
    // provenance is not exact evidence, even if imported by older code.
    let wrong_bytes = source(&scope, "lab-2", "r1", "Creatinine 0.8 (normal)");
    let mut wrong_provenance = arrived.clone();
    wrong_provenance.provenance = serde_json::json!({"native_record": "other-lab"});
    for (offset, wrong) in [wrong_bytes, wrong_provenance].into_iter().enumerate() {
        read_then_finish(
            &store,
            &claim,
            3 + offset as u64,
            &[wrong],
            &["lab-2".into()],
            &unread,
        )
        .await;
        assert_eq!(
            store
                .commit_revision(&claim, &unread, &mut OpenAuthority)
                .await
                .unwrap(),
            RevisionCommit::SourcesUnread(vec!["lab-2".into()])
        );
    }
    // Reading it, then finishing, publishes revision 2.
    let caught_up = finish("finish-5", "Potassium is normal; creatinine is high.");
    read_then_finish(
        &store,
        &claim,
        5,
        &claim.sources.clone(),
        &["lab-2".into()],
        &caught_up,
    )
    .await;
    // A returned provider turn rejected by the currency fence cannot establish
    // that the model received the refreshed source.
    use crate::runtime::entities::execution_event::{Column as C, Entity};
    use sea_orm::sea_query::Expr;
    let response_sequence = store
        .draft(&scope)
        .await
        .unwrap()
        .unwrap()
        .attempts
        .last()
        .unwrap()
        .events
        .iter()
        .find(|event| event.ordinal == 5 && matches!(event.kind, EventKind::ProviderReturned(_)))
        .unwrap()
        .sequence;
    let response_row = Entity::update_many()
        .filter(C::Practice.eq(scope.practice.as_str()))
        .filter(C::Attempt.eq(claim.attempt.as_str()))
        .filter(C::Sequence.eq(response_sequence as i64));
    response_row
        .clone()
        .col_expr(C::Admissible, Expr::value(false))
        .exec(&store.db)
        .await
        .unwrap();
    assert_eq!(
        store
            .commit_revision(&claim, &caught_up, &mut OpenAuthority)
            .await
            .unwrap(),
        RevisionCommit::SourcesUnread(vec!["lab-2".into()]),
        "a non-admissible provider response cannot establish source delivery"
    );
    response_row
        .col_expr(C::Admissible, Expr::value(true))
        .exec(&store.db)
        .await
        .unwrap();
    assert_eq!(
        store
            .commit_revision(&claim, &caught_up, &mut OpenAuthority)
            .await
            .unwrap(),
        RevisionCommit::Committed
    );
    let draft = store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(draft.revision, 2);
    assert_eq!(draft.text, "Potassium is normal; creatinine is high.");
    assert_eq!(draft.status, DraftStatus::Completed);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_correction_replaces_the_pinned_revision_and_keeps_the_old_one_on_record() {
    let (store, scope) = setup().await;
    let original = source(&scope, "lab", "r1", "Potassium 6.1 (hemolyzed)");
    let gone = source(&scope, "note", "r1", "Draft note");
    store
        .import_sources(&[original.clone(), gone.clone()])
        .await
        .unwrap();
    let mut claim = accepted_claim(&store, &scope, "first").await;
    let output = finish("finish-1", "Potassium is high.");
    exchange(&store, &claim, 1, &output).await;
    let corrected = source(&scope, "lab", "r2", "Potassium 4.2 (repeat)");
    store.record_source(&corrected).await.unwrap();
    store
        .record_chart(
            &scope.workspace,
            std::slice::from_ref(&corrected),
            &pull("ehr", 100),
        )
        .await
        .unwrap();
    let RevisionCommit::SourcesMoved { refresh, sources } = store
        .commit_revision(&claim, &output, &mut OpenAuthority)
        .await
        .unwrap()
    else {
        panic!("a correction must refresh the attempt");
    };
    assert_eq!(sources, vec![corrected.clone()]);
    assert_eq!(refresh.corrected.len(), 1);
    assert_eq!(refresh.corrected[0].previous, original);
    assert_eq!(refresh.corrected[0].current, corrected);
    assert_eq!(refresh.withdrawn, vec![gone]);
    assert_eq!(refresh.to_read(), vec!["lab".to_owned()]);
    claim.sources = sources;
    claim.sources_version = refresh.to_version;
    // A read of the old revision does not count: the read must match r2.
    let stale = finish("finish-2", "Potassium is high.");
    read_then_finish(
        &store,
        &claim,
        2,
        std::slice::from_ref(&original),
        &["lab".into()],
        &stale,
    )
    .await;
    assert_eq!(
        store
            .commit_revision(&claim, &stale, &mut OpenAuthority)
            .await
            .unwrap(),
        RevisionCommit::SourcesUnread(vec!["lab".into()])
    );
    let current = finish("finish-3", "Potassium is normal on repeat.");
    read_then_finish(
        &store,
        &claim,
        3,
        &claim.sources.clone(),
        &["lab".into()],
        &current,
    )
    .await;
    assert_eq!(
        store
            .commit_revision(&claim, &current, &mut OpenAuthority)
            .await
            .unwrap(),
        RevisionCommit::Committed
    );
    let draft = store.draft(&scope).await.unwrap().unwrap();
    let attempt = draft.attempts.last().unwrap();
    assert_eq!(
        attempt.sources,
        vec![original.clone(), source(&scope, "note", "r1", "Draft note")]
    );
    assert_eq!(attempt.refreshes[0].corrected[0].previous, original);
    assert_eq!(draft.sources, vec![corrected]);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn ingestion_waits_for_a_commit_holding_the_source_lock() {
    let (store, scope) = setup().await;
    store
        .record_source(&source(&scope, "lab", "r1", "Potassium 4.1"))
        .await
        .unwrap();
    // A commit holds the version row's shared lock until it ends.
    let tx = store.db.begin().await.unwrap();
    let held = sources::observed(&tx, &scope.workspace).await.unwrap();
    assert_eq!(held.version, 1);
    let arriving = source(&scope, "lab-2", "r1", "Creatinine 2.4");
    let ingest = tokio::spawn({
        let store = store.clone();
        let arriving = arriving.clone();
        async move { store.record_source(&arriving).await }
    });
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    assert!(!ingest.is_finished(), "ingestion must wait for the commit");
    tx.commit().await.unwrap();
    assert_eq!(ingest.await.unwrap().unwrap(), 2);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn ingestion_racing_a_commit_is_caught_or_lands_after_it_never_lost() {
    for round in 0..12 {
        let (store, scope) = setup().await;
        store
            .record_source(&source(&scope, "lab", "r1", "Potassium 4.1"))
            .await
            .unwrap();
        let claim = accepted_claim(&store, &scope, "first").await;
        let output = finish("finish-1", "Potassium is normal.");
        exchange(&store, &claim, 1, &output).await;
        let arriving = source(&scope, "lab-2", "r1", &format!("Creatinine {round}"));
        let mut allow = OpenAuthority;
        let (committed, recorded) = tokio::join!(
            store.commit_revision(&claim, &output, &mut allow),
            store.record_source(&arriving)
        );
        let recorded = recorded.unwrap();
        assert_eq!(recorded, 2);
        let draft = store.draft(&scope).await.unwrap().unwrap();
        match committed.unwrap() {
            // Published first: the source arrived after the reply was done.
            RevisionCommit::Committed => {
                assert_eq!(draft.revision, 2);
                assert_eq!(draft.sources_version, 1);
            }
            // Caught: nothing published, and the attempt now pins the source.
            RevisionCommit::SourcesMoved { refresh, .. } => {
                assert_eq!(draft.revision, 1);
                assert_eq!(refresh.added, vec![arriving.clone()]);
                assert_eq!(draft.sources_version, recorded);
            }
            RevisionCommit::SourcesUnread(_) => panic!("no refresh preceded this finish"),
            RevisionCommit::CitationsRefused(error) => {
                panic!("no sources read in this turn: {error:?}")
            }
        }
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn retries_pin_the_newest_sources_and_duplicates_keep_the_original_pin() {
    let (store, scope) = setup().await;
    let lab = source(&scope, "lab", "r1", "Potassium 4.1");
    store.record_source(&lab).await.unwrap();
    let first = accepted_claim(&store, &scope, "first").await;
    assert_eq!(first.sources_version, 1);
    let arrived = source(&scope, "lab-2", "r1", "Creatinine 2.4");
    store.record_source(&arrived).await.unwrap();
    // A duplicate acceptance answers from the receipt and keeps the pin.
    assert_eq!(
        accept_revision_input(&store, &scope, &input("first", 1))
            .await
            .unwrap(),
        reply::RevisionAccepted::Duplicate
    );
    assert_eq!(
        store.draft(&scope).await.unwrap().unwrap().sources_version,
        1
    );
    store
        .settle_revision_attempt(&first, harness::engine::AttemptFailure::Failed)
        .await
        .unwrap();
    let retry = store
        .claim_revision(&Delivery::reply_revision(scope.clone(), "first".into()), 60)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(retry.sources_version, 2);
    assert_eq!(retry.sources, vec![lab.clone(), arrived.clone()]);
    let draft = store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(draft.attempts[0].sources, vec![lab.clone()]);
    assert_eq!(draft.attempts[1].sources, vec![lab, arrived]);
    // A fresh attempt finishing on its own pin publishes directly.
    let output = finish("finish-1", "Both labs reviewed.");
    exchange(&store, &retry, 1, &output).await;
    assert_eq!(
        store
            .commit_revision(&retry, &output, &mut OpenAuthority)
            .await
            .unwrap(),
        RevisionCommit::Committed
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn retained_model_context_requires_new_sources_after_a_no_dispatch_successor() {
    let (store, scope) = setup().await;
    let lab = source(&scope, "lab", "r1", "Potassium 4.1");
    store.record_source(&lab).await.unwrap();
    let first = accepted_claim(&store, &scope, "first").await;
    let old = finish("finish-old", "Potassium is normal.");
    read_then_finish(&store, &first, 1, &first.sources, &["lab".into()], &old).await;
    let arrived = source(&scope, "lab-2", "r1", "Creatinine 2.4 (high)");
    store.record_source(&arrived).await.unwrap();
    store
        .settle_revision_attempt(&first, harness::engine::AttemptFailure::Failed)
        .await
        .unwrap();
    let delivery = Delivery::reply_revision(scope.clone(), "first".into());
    let second = store.claim_revision(&delivery, 60).await.unwrap().unwrap();
    // Losing a successor before dispatch must not erase the earlier context
    // or its obligation to incorporate the newly pinned source.
    store
        .settle_revision_attempt(&second, harness::engine::AttemptFailure::Failed)
        .await
        .unwrap();
    let third = store.claim_revision(&delivery, 60).await.unwrap().unwrap();
    assert_eq!(third.sources_version, 2);
    exchange(&store, &third, 1, &old).await;
    let refused = store
        .commit_revision(&third, &old, &mut OpenAuthority)
        .await
        .unwrap_err();
    assert!(
        refused
            .to_string()
            .as_str()
            .contains("no model call followed the source refresh")
    );
    let unchanged = store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(
        (unchanged.revision, unchanged.text.as_str()),
        (1, "Original")
    );

    let updated = finish("finish-updated", "Potassium is normal; creatinine is high.");
    read_then_finish(
        &store,
        &third,
        2,
        &third.sources,
        &["lab-2".into()],
        &updated,
    )
    .await;
    assert_eq!(
        store
            .commit_revision(&third, &updated, &mut OpenAuthority)
            .await
            .unwrap(),
        RevisionCommit::Committed
    );
    let published = store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(published.revision, 2);
    assert_eq!(published.text, "Potassium is normal; creatinine is high.");
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_source_change_after_a_lost_revision_reaches_the_model_before_its_saved_finish() {
    for withdrawn in [false, true] {
        let case = if withdrawn { "withdrawn" } else { "corrected" };
        let (store, scope) = setup().await;
        let lab = source(&scope, "lab", "r1", "Potassium 4.1");
        let note = source(&scope, "note", "r1", "Seen in clinic");
        store
            .import_sources(&[lab.clone(), note.clone()])
            .await
            .unwrap();
        let first = accepted_claim(&store, &scope, "first").await;
        let saved = finish("finish-saved", "Potassium is normal; seen in clinic.");
        read_then_finish(
            &store,
            &first,
            1,
            &first.sources,
            &["lab".into(), "note".into()],
            &saved,
        )
        .await;
        // The worker dies before publication; its lease runs out.
        use entities::revision_attempt::{Column, Entity};
        Entity::update_many()
            .col_expr(Column::Expires, Expr::value(0_i64))
            .filter(Column::Id.eq(first.attempt.as_str()))
            .exec(&store.db)
            .await
            .unwrap();
        if withdrawn {
            store
                .record_chart(
                    &scope.workspace,
                    std::slice::from_ref(&lab),
                    &pull("ehr", 100),
                )
                .await
                .unwrap();
        } else {
            store
                .record_source(&source(
                    &scope,
                    "note",
                    "r2",
                    "Seen in clinic; plan changed",
                ))
                .await
                .unwrap();
        }
        let delivery = Delivery::reply_revision(scope.clone(), "first".into());
        let successor = store.claim_revision(&delivery, 60).await.unwrap().unwrap();
        // What the worker passes the model as source feedback on recovery.
        let told = store
            .draft(&scope)
            .await
            .unwrap()
            .unwrap()
            .attempts
            .last()
            .unwrap()
            .refreshes
            .last()
            .is_some_and(|refresh| {
                refresh
                    .withdrawn
                    .iter()
                    .chain(refresh.corrected.iter().map(|c| &c.previous))
                    .any(|source| source.candidate_id == "note")
            });
        // The saved finish, offered again without another model call.
        store
            .record_revision(
                &successor,
                Event {
                    attempt_id: successor.attempt.clone(),
                    sequence: 1,
                    ordinal: 1,
                    kind: EventKind::Candidate { text: saved.text() },
                },
            )
            .await
            .unwrap();
        let committed = matches!(
            store
                .commit_revision(&successor, &saved, &mut OpenAuthority)
                .await,
            Ok(RevisionCommit::Committed)
        );
        let draft = store.draft(&scope).await.unwrap().unwrap();
        assert_eq!(
            (told, committed, draft.revision, draft.text.as_str()),
            (true, false, 1, "Original"),
            "{case}: the model hears of the changed source before its saved finish can publish"
        );
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_chart_delivery_says_what_the_source_holds_now() {
    let (store, scope) = setup().await;
    let first = source(&scope, "lab", "r1", "Potassium 4.1");
    let second = source(&scope, "lab", "r2", "Potassium 4.3");
    let delivery = pull("ehr", 100);
    for chart in [&first, &second, &first] {
        store
            .record_chart(&scope.workspace, std::slice::from_ref(chart), &delivery)
            .await
            .unwrap();
    }
    let rolled_back = store.sources(&scope.workspace).await.unwrap();
    assert_eq!(rolled_back.sources, std::slice::from_ref(&first));
    assert_eq!(rolled_back.version, 3);

    // Dropped, then delivered again with the same bytes.
    store
        .record_chart(&scope.workspace, &[], &delivery)
        .await
        .unwrap();
    assert!(
        store
            .sources(&scope.workspace)
            .await
            .unwrap()
            .sources
            .is_empty()
    );
    store
        .record_chart(&scope.workspace, std::slice::from_ref(&first), &delivery)
        .await
        .unwrap();
    assert_eq!(
        store.sources(&scope.workspace).await.unwrap().sources,
        [first]
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn currency_is_read_only_and_renewal_touches_only_the_current_revision_attempt() {
    use entities::revision_attempt::{Column as C, Entity};
    let (store, scope) = setup().await;
    let old = accepted_claim(&store, &scope, "lease").await;
    assert_eq!(
        store
            .settle_revision_attempt(&old, engine::AttemptFailure::Interrupted)
            .await
            .unwrap(),
        delivery::AttemptSettlement::Applied
    );
    let claim = store
        .claim_revision(&Delivery::reply_revision(scope.clone(), "lease".into()), 60)
        .await
        .unwrap()
        .unwrap();
    let tx = store.db.begin().await.unwrap();
    let near_expiry = now(&tx).await.unwrap() + 5;
    tx.commit().await.unwrap();
    Entity::update_many()
        .col_expr(C::Expires, Expr::value(near_expiry))
        .filter(control::revision_scope(&scope))
        .exec(&store.db)
        .await
        .unwrap();
    let locked = store.db.begin().await.unwrap();
    super::super::entities::reply_draft::Entity::find_by_id((
        scope.practice.clone(),
        scope.physician.clone(),
        scope.workspace.clone(),
        scope.chat.clone(),
    ))
    .lock_exclusive()
    .one(&locked)
    .await
    .unwrap()
    .unwrap();
    let read = tokio::time::timeout(
        std::time::Duration::from_secs(1),
        store.assert_revision_current(&claim),
    )
    .await;
    locked.rollback().await.unwrap();
    assert!(
        read.is_ok(),
        "revision currency check waited for a write lock"
    );
    read.unwrap().unwrap();
    let unchanged = store.draft(&scope).await.unwrap().unwrap();
    assert!(
        unchanged
            .attempts
            .iter()
            .all(|attempt| attempt.expires == near_expiry)
    );
    store.renew_revision_claim(&claim).await.unwrap();
    let state = store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(state.attempts[0].expires, near_expiry);
    assert!(
        state.attempts[1].expires >= near_expiry + 50,
        "current lease was not renewed"
    );
    assert!(store.renew_revision_claim(&old).await.is_err());
    store.assert_revision_current(&old).await.unwrap();
    assert_eq!(
        store
            .settle_revision_attempt(&old, engine::AttemptFailure::Failed)
            .await
            .unwrap(),
        delivery::AttemptSettlement::Superseded
    );
    let preserved = store.draft(&scope).await.unwrap().unwrap();
    assert_eq!(preserved.failures, state.failures);
    assert_eq!(preserved.attempts[1].status, AttemptStatus::Active);
    assert_eq!(preserved.attempts[1].id, claim.attempt);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn cited_source_state_tracks_newer_versions_and_enforces_restrictions() {
    use crate::runtime::sources::{CitedSourceState, CitedSourceVersion, SourceRestriction};
    let (store, scope) = setup().await;
    let r1 = source(&scope, "lab", "r1", "Potassium 4.1");
    let r2 = source(&scope, "lab", "r2", "Potassium 5.4");
    let wrong_patient = source(&scope, "msg-wrong", "m1", "Message for another patient");
    store
        .import_sources(&[r1.clone(), wrong_patient.clone()])
        .await
        .unwrap();

    assert_eq!(
        store
            .cited_source_state(&scope.workspace, "lab", "r1")
            .await
            .unwrap(),
        CitedSourceState::Readable {
            version: CitedSourceVersion::Current,
        }
    );

    // A newer revision arrives: r1 still opens, flagged as having a newer version, while r2 is current.
    store.record_source(&r2).await.unwrap();
    assert_eq!(
        store
            .cited_source_state(&scope.workspace, "lab", "r1")
            .await
            .unwrap(),
        CitedSourceState::Readable {
            version: CitedSourceVersion::NewerAvailable,
        }
    );
    assert_eq!(
        store
            .cited_source_state(&scope.workspace, "lab", "r2")
            .await
            .unwrap(),
        CitedSourceState::Readable {
            version: CitedSourceVersion::Current,
        }
    );

    // Wrong-patient correction withholds msg-wrong from active observations and citation reads,
    // including across snapshot re-imports.
    let before = store.source_version(&scope.workspace).await.unwrap();
    let after = store
        .restrict_source(
            &scope.workspace,
            "msg-wrong",
            SourceRestriction::WithheldWrongPatient,
        )
        .await
        .unwrap();
    assert_eq!(after, before + 1);
    assert_eq!(
        store
            .restrict_source(
                &scope.workspace,
                "msg-wrong",
                SourceRestriction::WithheldWrongPatient,
            )
            .await
            .unwrap(),
        after,
        "repeating the same restriction is idempotent"
    );
    store.import_sources(&[r1, wrong_patient]).await.unwrap();
    assert_eq!(
        store.sources(&scope.workspace).await.unwrap().sources,
        [r2],
        "withheld wrong-patient record never reappears in chart observations"
    );
    assert_eq!(
        store
            .cited_source_state(&scope.workspace, "msg-wrong", "m1")
            .await
            .unwrap(),
        CitedSourceState::Unavailable {
            reason: SourceRestriction::WithheldWrongPatient,
        }
    );

    // Source-level access removal likewise marks the candidate unavailable.
    store
        .restrict_source(&scope.workspace, "lab", SourceRestriction::AccessRemoved)
        .await
        .unwrap();
    assert_eq!(
        store
            .cited_source_state(&scope.workspace, "lab", "r1")
            .await
            .unwrap(),
        CitedSourceState::Unavailable {
            reason: SourceRestriction::AccessRemoved,
        }
    );
    assert!(
        store
            .sources(&scope.workspace)
            .await
            .unwrap()
            .sources
            .is_empty()
    );
}
