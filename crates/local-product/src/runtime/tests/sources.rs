//! Postgres proof that a reply revision takes in sources arriving before it
//! finishes: the versioned source store, per-attempt pins, refreshes recorded
//! on the attempt, and ingestion serialized against publication.
use super::*;
use crate::physician_sources::adapter::file_fhir_record;
use crate::physician_sources::{
    Bucket, ChartPullReceipt, ChartRequest, ChartScope, DateRole, FiledDate, READ_TOOL,
    ReadablePart, RecordDate, RecordVersion, SEARCH_TOOL, SourcePatient, SourceRecordKind,
    SourceRevision, exact_read,
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
        envelope: None,
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
        pulled_at: Some(pulled_at),
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
async fn a_chart_delivery_is_one_version_and_retains_missing_records() {
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

    // The source omitted the note and corrected the lab. Both the omitted note
    // and the other source's record stay saved, and the pull records `note` as missing.
    let corrected = source(&scope, "lab", "r2", "Potassium 4.3");
    let receipt = store
        .record_chart_with_receipt(
            &scope.workspace,
            std::slice::from_ref(&corrected),
            &delivery,
        )
        .await
        .unwrap();
    assert_eq!(
        receipt,
        ChartPullReceipt {
            changed: 1,
            missing_candidates: vec!["note".into()],
        }
    );
    let after = store.sources(&scope.workspace).await.unwrap();
    assert_eq!(after.version, 3);
    assert_eq!(after.sources, [corrected, message, note]);
    assert_eq!(
        store
            .pull_missing_candidates(&scope.workspace, "ehr")
            .await
            .unwrap(),
        vec!["note".to_owned()]
    );

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
            .withdraw_source(&scope.workspace, "note")
            .await
            .unwrap(),
        4
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
    assert_eq!(
        store
            .withdraw_source(&scope.workspace, "lab")
            .await
            .unwrap(),
        5
    );
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
    assert_eq!(
        store
            .withdraw_source(&scope.workspace, "lab")
            .await
            .unwrap(),
        3
    );
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
                .withdraw_source(&scope.workspace, "lab")
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
        .withdraw_source(&scope.workspace, "note")
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
        .settle_revision_attempt(
            &first,
            harness::engine::AttemptFailure::Failed,
            Backoff::IMMEDIATE,
        )
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
        .settle_revision_attempt(
            &first,
            harness::engine::AttemptFailure::Failed,
            Backoff::IMMEDIATE,
        )
        .await
        .unwrap();
    let delivery = Delivery::reply_revision(scope.clone(), "first".into());
    let second = store.claim_revision(&delivery, 60).await.unwrap().unwrap();
    // Losing a successor before dispatch must not erase the earlier context
    // or its obligation to incorporate the newly pinned source.
    store
        .settle_revision_attempt(
            &second,
            harness::engine::AttemptFailure::Failed,
            Backoff::IMMEDIATE,
        )
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
                .withdraw_source(&scope.workspace, "note")
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

    // Withdrawn, then delivered again with the same bytes.
    store
        .withdraw_source(&scope.workspace, "lab")
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
            .settle_revision_attempt(
                &old,
                engine::AttemptFailure::Interrupted,
                Backoff::IMMEDIATE
            )
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
    let lost = store.assert_revision_current(&old).await.unwrap_err();
    assert!(
        matches!(
            lost,
            engine::ProductError::NotCurrent(engine::NotCurrent::Lost)
        ),
        "a replaced attempt has lost ownership: {lost}"
    );
    assert_eq!(
        store
            .settle_revision_attempt(&old, engine::AttemptFailure::Failed, Backoff::IMMEDIATE)
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
async fn a_record_missing_from_a_later_pull_stays_saved_and_is_recorded_missing() {
    let (store, scope) = setup().await;
    let lab = source(&scope, "lab", "r1", "Potassium 4.1");
    let note = source(&scope, "note", "r1", "Seen in clinic");
    let first_pull = pull("ehr", 100);
    let initial = store
        .record_chart_with_receipt(&scope.workspace, &[lab.clone(), note.clone()], &first_pull)
        .await
        .unwrap();
    assert_eq!(
        initial,
        ChartPullReceipt {
            changed: 2,
            missing_candidates: vec![],
        }
    );

    // Later pull delivers corrected lab and omits `note`: `note` stays saved and is recorded missing.
    let corrected_lab = source(&scope, "lab", "r2", "Potassium 4.3");
    let second_pull = pull("ehr", 200);
    let receipt = store
        .record_chart_with_receipt(
            &scope.workspace,
            std::slice::from_ref(&corrected_lab),
            &second_pull,
        )
        .await
        .unwrap();
    assert_eq!(
        receipt,
        ChartPullReceipt {
            changed: 1,
            missing_candidates: vec!["note".into()],
        }
    );

    let after = store.sources(&scope.workspace).await.unwrap();
    assert_eq!(
        after.sources,
        vec![corrected_lab.clone(), note.clone()],
        "a record missing from a later pull must stay saved"
    );
    assert_eq!(
        store
            .pull_missing_candidates(&scope.workspace, "ehr")
            .await
            .unwrap(),
        vec!["note".to_owned()]
    );

    // When a subsequent pull includes `note` again, `missing_candidates` clears.
    let third_pull = pull("ehr", 300);
    let recovered = store
        .record_chart_with_receipt(&scope.workspace, &[corrected_lab, note], &third_pull)
        .await
        .unwrap();
    assert_eq!(
        recovered,
        ChartPullReceipt {
            changed: 0,
            missing_candidates: vec![],
        }
    );
    assert!(
        store
            .pull_missing_candidates(&scope.workspace, "ehr")
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn filed_fhir_record_envelope_round_trips_through_postgres_unchanged() {
    let (store, scope) = setup().await;
    let raw_ndjson = r#"{"resourceType":"Observation" , "id":"potassium-9", "meta":{"versionId":"3"}, "status":"final", "category":[{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/observation-category","code":"laboratory"}]}], "code":{"coding":[{"system":"http://loinc.org","code":"2823-3","display":"Potassium"}]}, "effectiveDateTime":"2026-08-21T06:15:00-04:00"}"#;
    let filed = file_fhir_record(
        &scope.workspace,
        "ehr",
        raw_ndjson,
        Some("https://ehr.example/Observation/potassium-9".into()),
    )
    .unwrap();

    store
        .record_chart(
            &scope.workspace,
            std::slice::from_ref(&filed),
            &pull("ehr", 100),
        )
        .await
        .unwrap();

    let observed = store.sources(&scope.workspace).await.unwrap();
    assert_eq!(observed.sources.len(), 1);
    let saved = &observed.sources[0];
    assert_eq!(saved, &filed);
    assert_eq!(saved.content.as_bytes(), raw_ndjson.as_bytes());
    let env = saved.envelope.as_ref().expect("persisted envelope");
    assert_eq!(env.bucket, Bucket::Results);
    assert_eq!(
        env.source_kind,
        SourceRecordKind {
            kind: "Observation".into(),
            category: Some("laboratory".into()),
        }
    );
    assert_eq!(env.version, RecordVersion::Source("3".into()));
    assert_eq!(env.status.as_deref(), Some("final"));
    assert_eq!(
        env.dates,
        vec![FiledDate {
            role: DateRole::Effective,
            field: "effectiveDateTime".into(),
            value: "2026-08-21T06:15:00-04:00".into(),
        }]
    );
    assert_eq!(
        env.native_link.as_deref(),
        Some("https://ehr.example/Observation/potassium-9")
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn table_filed_delivery_reconciles_legacy_unprefixed_aliases_and_keeps_omitted_legacy_records()
 {
    let (store, scope) = setup().await;
    let raw_lab_v1 = r#"{"resourceType":"Observation","id":"potassium-9","meta":{"versionId":"1"},"status":"preliminary","category":[{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/observation-category","code":"laboratory"}]}],"code":{"text":"Potassium 6.1 hemolyzed"},"effectiveDateTime":"2026-08-21T06:15:00-04:00"}"#;
    let raw_med = r#"{"resourceType":"MedicationRequest","id":"med-1","meta":{"versionId":"1"},"status":"active","medicationCodeableConcept":{"text":"Lisinopril 10 mg"},"authoredOn":"2026-08-01"}"#;
    let raw_omitted_note = r#"{"resourceType":"DocumentReference","id":"note-omitted","status":"current","description":"Outside clinic note"}"#;

    let legacy_record = |record_type: &str, id: &str, raw: &str| {
        let candidate_id = format!("{record_type}/{id}");
        SourceRevision {
            workspace: scope.workspace.clone(),
            source_id: "ehr".into(),
            candidate_id: candidate_id.clone(),
            revision: crate::sha256_hex(raw.as_bytes()),
            record_type: record_type.into(),
            dates: Vec::new(),
            content: raw.into(),
            sha256: crate::sha256_hex(raw.as_bytes()),
            readable_parts: Vec::new(),
            provenance: serde_json::json!({"native_record": candidate_id}),
            envelope: None,
        }
    };

    let legacy_lab = legacy_record("Observation", "potassium-9", raw_lab_v1);
    let legacy_med = legacy_record("MedicationRequest", "med-1", raw_med);
    let legacy_omitted = legacy_record("DocumentReference", "note-omitted", raw_omitted_note);

    let initial = store
        .record_chart_with_receipt(
            &scope.workspace,
            &[
                legacy_lab.clone(),
                legacy_med.clone(),
                legacy_omitted.clone(),
            ],
            &pull("ehr", 100),
        )
        .await
        .unwrap();
    assert_eq!(initial.changed, 3);
    assert!(initial.missing_candidates.is_empty());

    let pinned_v1 = store.sources(&scope.workspace).await.unwrap();
    assert_eq!(pinned_v1.version, 1);
    let claim_v1 = accepted_claim(&store, &scope, "pinned-v1").await;
    assert_eq!(claim_v1.sources_version, 1);

    // Upgrade to table-filed records (`ehr/{kind}/{id}`), genuinely omitting `DocumentReference/note-omitted`.
    let filed_lab_v1 = file_fhir_record(&scope.workspace, "ehr", raw_lab_v1, None).unwrap();
    let filed_med = file_fhir_record(&scope.workspace, "ehr", raw_med, None).unwrap();

    let upgraded = store
        .record_chart_with_receipt(
            &scope.workspace,
            &[filed_lab_v1.clone(), filed_med.clone()],
            &pull("ehr", 200),
        )
        .await
        .unwrap();
    assert_eq!(
        upgraded,
        ChartPullReceipt {
            changed: 2,
            missing_candidates: vec!["DocumentReference/note-omitted".into()],
        },
        "reconciled unprefixed aliases are retired cleanly while genuinely omitted legacy records stay reported"
    );
    assert_eq!(
        store
            .pull_missing_candidates(&scope.workspace, "ehr")
            .await
            .unwrap(),
        vec!["DocumentReference/note-omitted".to_owned()]
    );

    let at_v2 = store.sources(&scope.workspace).await.unwrap();
    assert_eq!(at_v2.version, 2);
    let active_ids: Vec<&str> = at_v2
        .sources
        .iter()
        .map(|s| s.candidate_id.as_str())
        .collect();
    assert_eq!(
        active_ids,
        vec![
            "DocumentReference/note-omitted",
            "ehr/MedicationRequest/med-1",
            "ehr/Observation/potassium-9",
        ]
    );

    // Historical reads at v1 still resolve the unprefixed row byte-for-byte.
    let historical_v1 = store.sources_at(&scope.workspace, 1).await.unwrap();
    assert_eq!(historical_v1.sources.len(), 3);
    let read_v1 = exact_read(
        &historical_v1.sources,
        &scope.workspace,
        &["Observation/potassium-9".into()],
        16384,
    )
    .unwrap();
    let read_v1_json: serde_json::Value = serde_json::from_str(read_v1.text().unwrap()).unwrap();
    assert_eq!(read_v1_json["results"][0]["outcome"], "available");
    assert_eq!(read_v1_json["results"][0]["content"], raw_lab_v1);

    // Repeat pull is a zero-change no-op.
    let repeat = store
        .record_chart_with_receipt(
            &scope.workspace,
            &[filed_lab_v1, filed_med.clone()],
            &pull("ehr", 250),
        )
        .await
        .unwrap();
    assert_eq!(
        repeat,
        ChartPullReceipt {
            changed: 0,
            missing_candidates: vec!["DocumentReference/note-omitted".into()],
        }
    );
    assert_eq!(store.sources(&scope.workspace).await.unwrap().version, 2);

    // Subsequent correction updates only the prefixed record, leaving zero stale clinical content.
    let raw_lab_v2 = r#"{"resourceType":"Observation","id":"potassium-9","meta":{"versionId":"2"},"status":"corrected","category":[{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/observation-category","code":"laboratory"}]}],"code":{"text":"Potassium 4.2 repeat"},"effectiveDateTime":"2026-08-21T06:15:00-04:00"}"#;
    let filed_lab_v2 = file_fhir_record(&scope.workspace, "ehr", raw_lab_v2, None).unwrap();
    let corrected = store
        .record_chart_with_receipt(
            &scope.workspace,
            &[filed_lab_v2.clone(), filed_med],
            &pull("ehr", 300),
        )
        .await
        .unwrap();
    assert_eq!(
        corrected,
        ChartPullReceipt {
            changed: 1,
            missing_candidates: vec!["DocumentReference/note-omitted".into()],
        }
    );

    let at_v3 = store.sources(&scope.workspace).await.unwrap();
    assert_eq!(at_v3.version, 3);
    assert!(
        !at_v3
            .sources
            .iter()
            .any(|s| str::contains(&s.content, "6.1")),
        "superseded hemolyzed value must not remain active under legacy candidate_id"
    );

    let search = ChartRequest::parse(
        SEARCH_TOOL,
        &serde_json::json!({"query": "potassium", "bucket": null, "record_type": null, "after": null}),
        ChartScope::Chat,
    )
    .unwrap();
    let search_json: serde_json::Value = serde_json::from_str(
        search
            .run(&scope.workspace, at_v3.version, &at_v3.sources)
            .unwrap()
            .text()
            .unwrap(),
    )
    .unwrap();
    let hits = search_json["results"].as_array().unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(
        hits[0]["reference"]["candidate_id"],
        "ehr/Observation/potassium-9"
    );

    let read = ChartRequest::parse(
        READ_TOOL,
        &serde_json::json!({"references": [hits[0]["reference"].clone()], "max_result_bytes": 16384}),
        ChartScope::Chat,
    )
    .unwrap();
    let read_json: serde_json::Value = serde_json::from_str(
        read.run(&scope.workspace, at_v3.version, &at_v3.sources)
            .unwrap()
            .text()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(read_json["results"][0]["content"], raw_lab_v2);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn heartbeat_renewal_does_not_deadlock_when_fenced_delivery_waits_on_database_lock() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
    use std::time::{Duration, Instant};

    use crate::physician_sources::{
        PatientSource, ReconnectReceipt, SourceAdapterError, SourceCommandKey, SourcePullOutput,
    };
    use crate::runtime::entities::patient_source_status;
    use crate::source_status::{
        LinkTarget, ReconnectCommand, ReconnectCommandOutcome, ReconnectLeasePolicy,
        SourceAccessState, SourceCoordinator,
    };

    struct RowLockingAdapter {
        source_id: String,
        locker_store: Store,
        practice_id: String,
        workspace_id: String,
        arm_status_lock: AtomicBool,
        lock_held_during_delivery: AtomicBool,
    }

    #[async_trait]
    impl PatientSource for RowLockingAdapter {
        fn source_id(&self) -> &str {
            &self.source_id
        }

        fn source_label(&self) -> &str {
            "Row-Locking Test EHR"
        }

        async fn patient_ids(&self) -> Result<Vec<String>, SourceAdapterError> {
            Ok(vec!["pat-1".to_owned()])
        }

        async fn patient(
            &self,
            source_patient_id: &str,
        ) -> Result<SourcePatient, SourceAdapterError> {
            Ok(SourcePatient {
                source_patient_id: source_patient_id.to_owned(),
                display_name: "Mary Louis".to_owned(),
                birth_date: Some("1972-03-18".to_owned()),
                sex_or_gender: None,
                medical_record_numbers: vec![],
            })
        }

        async fn records(
            &self,
            _patient: &SourcePatient,
            _workspace: &str,
        ) -> Result<Vec<SourceRevision>, SourceAdapterError> {
            Ok(vec![])
        }

        async fn pull_patient(
            &self,
            _practice_id: &str,
            source_patient_id: &str,
            workspace: &str,
        ) -> SourcePullOutput {
            if self.arm_status_lock.swap(false, SeqCst) {
                // Acquire `FOR UPDATE` on the patient's `physician_runtime_patient_source_status`
                // row on an independent connection and hold it for 80ms while returning `Complete`
                // immediately. `apply_pull_output -> record_complete_pull_fenced` will immediately
                // acquire `FOR UPDATE` on `physician_runtime_reconnect_commands` and then block in
                // Postgres for 80ms waiting on `hold_tx` while ~7-8 10ms heartbeat ticks fire.
                let hold_tx = self.locker_store.db.begin().await.unwrap();
                let locked = patient_source_status::Entity::find_by_id((
                    self.practice_id.clone(),
                    self.workspace_id.clone(),
                    self.source_id.clone(),
                ))
                .lock_exclusive()
                .one(&hold_tx)
                .await
                .unwrap();
                assert!(
                    locked.is_some(),
                    "seeded patient_source_status row must exist"
                );
                self.lock_held_during_delivery.store(true, SeqCst);
                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_millis(80)).await;
                    hold_tx.commit().await.unwrap();
                });
            }
            let scope = Scope {
                practice: self.practice_id.clone(),
                physician: "dr-lock".to_owned(),
                workspace: workspace.to_owned(),
                chat: "reply".to_owned(),
            };
            let mut rev = source(&scope, "lab-lock", "r1", "Potassium 4.2");
            rev.source_id = self.source_id.clone();
            SourcePullOutput::Complete {
                patient: self.patient(source_patient_id).await.unwrap(),
                records: vec![rev],
            }
        }

        async fn reconnect(
            &self,
            practice_id: &str,
            now: i64,
            _command_key: Option<SourceCommandKey<'_>>,
        ) -> Result<ReconnectReceipt, SourceAdapterError> {
            Ok(ReconnectReceipt {
                source_id: self.source_id.clone(),
                practice_id: practice_id.to_owned(),
                reconnected_at: now,
            })
        }
    }

    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let locker_store = Store::connect(&url).await.unwrap();

    let run = Uuid::new_v4().to_string();
    let practice = format!("practice-hb-lock-{run}");
    let workspace = format!("pw-hb-lock-{run}");
    let source_id = format!("ehr-hb-lock-{run}");
    let adapter = Arc::new(RowLockingAdapter {
        source_id: source_id.clone(),
        locker_store,
        practice_id: practice.clone(),
        workspace_id: workspace.clone(),
        arm_status_lock: AtomicBool::new(false),
        lock_held_during_delivery: AtomicBool::new(false),
    });

    // Seed the patient link and `patient_source_status` row at t = 1_000.
    SourceCoordinator::check_patient(
        &store,
        &*adapter,
        &practice,
        "pat-1",
        LinkTarget::Exact(&workspace),
        1_000,
    )
    .await
    .unwrap();

    store
        .set_practice_source_access(
            &practice,
            &source_id,
            &SourceAccessState::ReconnectNeeded {
                since: 1_500,
                reason: "token expired".to_owned(),
            },
            1_500,
        )
        .await
        .unwrap();

    adapter.arm_status_lock.store(true, SeqCst);
    let idem_key = format!("idem-hb-lock-{run}");
    let started = Instant::now();

    let outcome = tokio::time::timeout(
        Duration::from_secs(5),
        SourceCoordinator::reconnect_source_command_with_lease(
            &store,
            ReconnectCommand {
                practice_id: &practice,
                physician_id: "dr-lock",
                idempotency_key: &idem_key,
                source_id: &source_id,
                source: Some(&*adapter),
                now: 2_000,
            },
            ReconnectLeasePolicy {
                lease_seconds: 10,
                renew_interval: Duration::from_millis(10),
            },
        ),
    )
    .await
    .expect("fenced transaction waiting on Postgres row lock must not self-deadlock with heartbeat")
    .unwrap();

    assert!(adapter.lock_held_during_delivery.load(SeqCst));
    assert!(
        started.elapsed() >= Duration::from_millis(75),
        "fenced transaction must have waited on the 80ms Postgres lock across multiple 10ms ticks"
    );
    match outcome {
        ReconnectCommandOutcome::Accepted {
            receipt,
            access,
            summary,
        } => {
            assert_eq!(receipt.reconnected_at, 2_000);
            assert_eq!(
                access,
                SourceAccessState::Connected {
                    connected_at: 2_000
                }
            );
            assert_eq!(
                summary,
                crate::source_status::PracticeSourceSummary::Healthy {
                    source_id: source_id.clone(),
                    total_patients: 1,
                }
            );
        }
        other => panic!("expected Accepted outcome, got {other:?}"),
    }

    // Replaying the completed key proves `complete_reconnect_command` committed cleanly after
    // joining the spawned heartbeat task without racing an unblocked renewal query.
    let replayed = SourceCoordinator::reconnect_source_command(
        &store,
        ReconnectCommand {
            practice_id: &practice,
            physician_id: "dr-lock",
            idempotency_key: &idem_key,
            source_id: &source_id,
            source: Some(&*adapter),
            now: 2_999,
        },
    )
    .await
    .unwrap();
    assert!(matches!(
        replayed,
        ReconnectCommandOutcome::Accepted { receipt, .. } if receipt.reconnected_at == 2_000
    ));
}
