//! The library's records against a real Postgres.
//!
//! Each test runs in its own schema of the database at
//! `CLINICAL_LIBRARY_TEST_DATABASE_URL`, which must be local. A test never
//! skips silently: without the variable it fails.

use clinical_library::{
    Acquired, ArticleEvent, ArticleState, Candidate, DeferralReason, FailureCode, HistoryEntry,
    HolderState, Next, NightOutcome, RETRY_LIMIT, RunId, SlotName, Source, StopReason, Store,
    StoreError, VersionId, WaitReason,
};
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement, Value,
};

struct Scratch {
    db: DatabaseConnection,
    schema: String,
}

impl Scratch {
    async fn new() -> Self {
        let url = std::env::var("CLINICAL_LIBRARY_TEST_DATABASE_URL")
            .expect("CLINICAL_LIBRARY_TEST_DATABASE_URL must name a local Postgres");
        assert!(url.contains("localhost") || url.contains("127.0.0.1"));
        // Postgres can't bind an identifier as a parameter, so the schema's
        // name is spliced in; it is generated here from hex digits only.
        let schema = format!("clinical_proof_{}", uuid::Uuid::new_v4().simple());
        let admin = Database::connect(&url).await.unwrap();
        admin
            .execute_unprepared(&format!("create schema {schema}"))
            .await
            .unwrap();
        // One connection, so the schema's search_path holds for every query.
        let mut options = ConnectOptions::new(url);
        options.max_connections(1).min_connections(1);
        let db = Database::connect(options).await.unwrap();
        db.execute_unprepared(&format!("set search_path to {schema}"))
            .await
            .unwrap();
        Self { db, schema }
    }

    async fn store(&self) -> Store {
        let store = Store::from_connection(self.db.clone());
        store.migrate().await.unwrap();
        store
    }

    async fn refused(&self, sql: &str) -> String {
        self.refused_with(sql, []).await
    }

    /// Runs `sql` with its values bound, expecting the database to refuse it.
    async fn refused_with(&self, sql: &str, values: impl IntoIterator<Item = Value>) -> String {
        let statement = Statement::from_sql_and_values(DbBackend::Postgres, sql, values);
        match self.db.execute_raw(statement).await {
            Ok(_) => panic!("the database accepted: {sql}"),
            Err(error) => error.to_string(),
        }
    }

    async fn drop(self) {
        self.db
            .execute_unprepared(&format!("drop schema {} cascade", self.schema))
            .await
            .unwrap();
    }
}

fn slot() -> SlotName {
    SlotName::new("nightly").unwrap()
}

fn code(value: &str) -> FailureCode {
    FailureCode::new(value).unwrap()
}

async fn hold(store: &Store, run: RunId) -> clinical_library::Lease {
    match store.acquire(&slot(), run, None).await.unwrap() {
        Acquired::Holds(lease) => lease,
        Acquired::Waits(waiting) => panic!("expected the slot, waited: {waiting:?}"),
    }
}

/// AC6: the chain applies to a fresh schema and a second run changes
/// nothing.
#[tokio::test]
async fn migrations_apply_to_a_fresh_database() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    store.migrate().await.unwrap();
    let ledger = scratch
        .db
        .query_one_raw(sea_orm::Statement::from_string(
            sea_orm::DbBackend::Postgres,
            "select count(*) as n from clinical_library_migrations",
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get::<i64>("", "n")
        .unwrap();
    assert_eq!(ledger, 1);
    assert_eq!(store.active_version().await.unwrap(), None);
    scratch.drop().await;
}

/// AC3: only a checked version activates, the switch leaves exactly one
/// active, and the database itself refuses a second active version or an
/// unchecked active one.
#[tokio::test]
async fn one_checked_version_is_active() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let lease = hold(&store, RunId::new()).await;

    for name in ["v1", "v2"] {
        let built = Candidate::built(VersionId::new(name).unwrap());
        let checked = built.check(|_| Ok::<(), String>(())).unwrap();
        let activation = store.activate(&lease, checked).await.unwrap();
        assert_eq!(activation.version().as_str(), name);
        assert_eq!(
            store.active_version().await.unwrap(),
            Some(VersionId::new(name).unwrap())
        );
    }

    let error = scratch
        .refused(
            "insert into clinical_library_versions(version, checked, active, activated_by_run)
             values ('v3', true, true, gen_random_uuid())",
        )
        .await;
    assert!(
        error.contains("clinical_library_one_active_version"),
        "{error}"
    );
    scratch
        .db
        .execute_unprepared("update clinical_library_versions set active = false")
        .await
        .unwrap();
    let error = scratch
        .refused(
            "insert into clinical_library_versions(version, checked, active, activated_by_run)
             values ('v4', false, true, gen_random_uuid())",
        )
        .await;
    assert!(error.contains("check"), "{error}");
    scratch.drop().await;
}

/// AC4: an alive holder keeps its slot, an unreadable one is waited on, an
/// ended one is taken over, and the old run's writes are refused after.
#[tokio::test]
async fn a_run_that_lost_its_slot_cannot_write() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let old = RunId::new();
    let old_lease = hold(&store, old).await;
    assert_eq!(store.holder(&slot()).await.unwrap(), Some(old));

    for (state, reason) in [
        (HolderState::Alive, DeferralReason::SlotHeldByLiveRun),
        (HolderState::Unknown, DeferralReason::OwnerUnreadable),
    ] {
        let waiting = match store
            .acquire(&slot(), RunId::new(), Some((old, state)))
            .await
            .unwrap()
        {
            Acquired::Waits(waiting) => waiting,
            Acquired::Holds(_) => panic!("took a slot whose holder is {state:?}"),
        };
        assert_eq!(waiting.reason(), reason);
        store.record_wait(waiting).await.unwrap();
    }
    // A holder the caller didn't ask about is treated as alive.
    let other = RunId::new();
    assert!(matches!(
        store
            .acquire(&slot(), RunId::new(), Some((other, HolderState::Ended)))
            .await
            .unwrap(),
        Acquired::Waits(_)
    ));

    let new = RunId::new();
    let new_lease = match store
        .acquire(&slot(), new, Some((old, HolderState::Ended)))
        .await
        .unwrap()
    {
        Acquired::Holds(lease) => lease,
        Acquired::Waits(waiting) => panic!("didn't take an ended holder's slot: {waiting:?}"),
    };
    assert!(new_lease.token() > old_lease.token());

    // The old run wakes up: every write it tries is refused.
    let lost = |result: Result<_, StoreError>| {
        assert!(
            matches!(result, Err(StoreError::LeaseLost { run, .. }) if run == old),
            "the old run's write wasn't refused"
        );
    };
    lost(
        store
            .apply(&old_lease, Source::PubMed, "12345", &ArticleEvent::Admitted)
            .await
            .map(drop),
    );
    let checked = Candidate::built(VersionId::new("stale").unwrap())
        .check(|_| Ok::<(), String>(()))
        .unwrap();
    lost(store.activate(&old_lease, checked).await.map(drop));
    lost(store.finish(old_lease, &NightOutcome::nothing_due()).await);
    assert_eq!(store.active_version().await.unwrap(), None);
    assert_eq!(store.item(Source::PubMed, "12345").await.unwrap(), None);

    // The new holder writes normally.
    store
        .apply(&new_lease, Source::PubMed, "12345", &ArticleEvent::Admitted)
        .await
        .unwrap();
    store
        .finish(new_lease, &NightOutcome::nothing_due())
        .await
        .unwrap();

    // The two waits, the taken-over night as a failure, then the success.
    assert_eq!(
        store.history(&slot()).await.unwrap(),
        vec![
            HistoryEntry::Night(NightOutcome::deferred(DeferralReason::SlotHeldByLiveRun)),
            HistoryEntry::Night(NightOutcome::deferred(DeferralReason::OwnerUnreadable)),
            HistoryEntry::Night(NightOutcome::failed(FailureCode::ended_without_outcome())),
            HistoryEntry::Night(NightOutcome::nothing_due()),
        ]
    );
    scratch.drop().await;
}

/// AC5 in storage: moves round-trip through the database, a refused move
/// changes nothing, and the database refuses rows the types can't produce.
#[tokio::test]
async fn item_states_keep_their_rules_in_storage() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let lease = hold(&store, RunId::new()).await;
    let failure = ArticleEvent::OutsideFailure(code("ncbi_timeout"));

    let state = store
        .apply(&lease, Source::Pmc, "PMC1", &failure)
        .await
        .unwrap();
    assert_eq!(store.item(Source::Pmc, "PMC1").await.unwrap(), Some(state));
    for _ in 1..RETRY_LIMIT {
        store
            .apply(&lease, Source::Pmc, "PMC1", &failure)
            .await
            .unwrap();
    }
    let needs_person = ArticleState::NeedsPerson {
        last_failure: code("ncbi_timeout"),
    };
    assert_eq!(
        store.item(Source::Pmc, "PMC1").await.unwrap(),
        Some(needs_person.clone())
    );
    let refused = store.apply(&lease, Source::Pmc, "PMC1", &failure).await;
    assert!(matches!(refused, Err(StoreError::InvalidMove(_))));
    assert_eq!(
        store.item(Source::Pmc, "PMC1").await.unwrap(),
        Some(needs_person)
    );

    let waiting = store
        .apply(
            &lease,
            Source::Pmc,
            "PMC2",
            &ArticleEvent::Refused(WaitReason::NoReadableText),
        )
        .await
        .unwrap();
    assert_eq!(
        store.item(Source::Pmc, "PMC2").await.unwrap(),
        Some(waiting)
    );

    // The database's retry bound is the Rust one.
    let limit = i16::from(RETRY_LIMIT);
    let retry =
        "insert into clinical_library_items(source, item_key, state, failures, last_failure)
                 values ('pmc', $1, 'waiting_to_retry', $2, 'x')";
    scratch
        .db
        .execute_raw(Statement::from_sql_and_values(
            DbBackend::Postgres,
            retry,
            ["bound".into(), (limit - 1).into()],
        ))
        .await
        .unwrap();
    scratch
        .refused_with(retry, ["over".into(), limit.into()])
        .await;
    for sql in [
        "insert into clinical_library_items(source, item_key, state)
         values ('pmc', 'no-reason', 'waiting_for_change')",
        "insert into clinical_library_items(source, item_key, state)
         values ('pmc', 'no-failure', 'needs_person')",
        "insert into clinical_library_items(source, item_key, state)
         values ('elsewhere', 'x', 'admitted')",
    ] {
        scratch.refused(sql).await;
    }
    scratch.drop().await;
}

/// AC2 in storage: each run records one outcome, the history can't be
/// rewritten, and a stored outcome reads back as the same outcome.
#[tokio::test]
async fn each_night_records_one_outcome() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let run = RunId::new();
    let lease = hold(&store, run).await;
    let outcome = NightOutcome::failed(code("build_failed"));
    store.finish(lease, &outcome).await.unwrap();
    assert_eq!(
        store.history(&slot()).await.unwrap(),
        vec![HistoryEntry::Night(outcome)]
    );

    let error = scratch
        .refused_with(
            "insert into clinical_library_nights(slot, run_id, outcome)
             values ('nightly', $1, 'succeeded')",
            [run.0.into()],
        )
        .await;
    assert!(error.contains("unique"), "{error}");
    let error = scratch
        .refused("update clinical_library_nights set outcome = 'succeeded', failure_code = null")
        .await;
    assert!(error.contains("append-only"), "{error}");
    scratch.refused("delete from clinical_library_nights").await;
    // A deferred night can't be stored as a success.
    scratch
        .refused(
            "insert into clinical_library_nights(slot, run_id, outcome, deferral_reason)
             values ('nightly', gen_random_uuid(), 'succeeded', 'waiting_for_capacity')",
        )
        .await;
    scratch.drop().await;
}

/// Decision table, read from storage: the same failure twice stops the
/// nightly, and a person's resolution starts the count over.
#[tokio::test]
async fn the_stop_rule_reads_the_stored_history() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    for _ in 0..2 {
        let lease = hold(&store, RunId::new()).await;
        store
            .finish(lease, &NightOutcome::failed(code("build_failed")))
            .await
            .unwrap();
    }
    assert_eq!(
        store.next(&slot()).await.unwrap(),
        Next::Stop(StopReason::SameFailureTwice(code("build_failed")))
    );
    store
        .record_resolution(&slot(), "operator@example.test", "Fixed the build image")
        .await
        .unwrap();
    assert_eq!(store.next(&slot()).await.unwrap(), Next::Continue);
    scratch.drop().await;
}
