//! The night runner against a real Postgres: resume, the stop rule, one
//! outcome per night, crashes at every step, a stale writer, a person acting
//! mid-night, and the alerts the records raise.
//!
//! Each test runs in its own schema of the database at
//! `CLINICAL_LIBRARY_TEST_DATABASE_URL`, which must be local. A test never
//! skips silently: without the variable it fails.
//!
//! A crash is a panic inside the night's task: the run stops between two
//! store writes and records nothing, as a killed process would.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use clinical_library::stop_rule::{self, HistoryEntry, Next};
use clinical_library::storage::{
    CandidateFolder, ObjectKey, ObjectPath, ObjectStore, StorageError,
};
use clinical_library::{
    Acquired, Alert, ArticleEvent, ArticleState, Candidate, Completion, DeferralReason, Ending,
    Failure, FailureCode, FailureDetail, HolderState, ItemKey, Job, JobError, Lease, Night,
    NightOutcome, OtherSource, RunId, SlotName, SlotWait, Source, StopReason, Store, StoreError,
    VersionId, WaitReason, run_night,
};

mod common;

use common::{Platforms, Scratch};

fn slot() -> SlotName {
    SlotName::new("nightly").unwrap()
}

fn pmc(key: &str) -> ItemKey {
    ItemKey::other(OtherSource::Pmc, key).unwrap()
}

fn code(value: &str) -> FailureCode {
    FailureCode::new(value).unwrap()
}

/// What tonight's job does with one item.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Plan {
    Admit,
    Fail(FailureCode),
    Refuse,
}

impl Plan {
    fn event(&self) -> ArticleEvent {
        match self {
            Self::Admit => ArticleEvent::Admitted,
            Self::Fail(code) => ArticleEvent::OutsideFailure(code.clone()),
            Self::Refuse => ArticleEvent::Refused(WaitReason::NoReadableText),
        }
    }
}

/// Object storage that records every write and who made it.
#[derive(Default)]
struct Recording {
    writes: Mutex<Vec<ObjectKey>>,
}

impl Recording {
    fn writes(&self) -> Vec<ObjectKey> {
        self.writes.lock().unwrap().clone()
    }
}

/// Refuses to replace a file, as a bucket's create-only write does.
#[async_trait]
impl ObjectStore for Recording {
    async fn create(&self, key: &ObjectKey, _: Vec<u8>) -> Result<(), StorageError> {
        let mut writes = self.writes.lock().unwrap();
        if writes.contains(key) {
            return Err(StorageError::Exists(key.clone()));
        }
        writes.push(key.clone());
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
enum TestError {
    #[error("the records failed")]
    Store(#[from] StoreError),
    #[error("the candidate's files couldn't be written")]
    Storage(#[from] StorageError),
    #[error("the build image couldn't start")]
    Failed(FailureCode),
}

impl JobError for TestError {
    fn failure(&self) -> Failure {
        match self {
            Self::Store(error) => error.failure(),
            Self::Storage(_) => Failure::Code(code("storage_failed")),
            Self::Failed(code) => Failure::Code(code.clone()),
        }
    }
}

/// A PMC job driven by a script: which items it qualifies and what it does
/// with each, whether it builds a version, where it crashes, and what it
/// leaves undone. Every store write is a step boundary.
struct TestJob {
    items: Vec<(ItemKey, Plan)>,
    build: bool,
    crash_at: Option<usize>,
    skip: Option<ItemKey>,
    fail: Option<FailureCode>,
    storage: Arc<Recording>,
    steps: AtomicUsize,
    applied: Mutex<Vec<ItemKey>>,
}

impl TestJob {
    fn new(items: &[(&str, Plan)], storage: &Arc<Recording>) -> Self {
        Self {
            items: items
                .iter()
                .map(|(key, plan)| (pmc(key), plan.clone()))
                .collect(),
            build: true,
            crash_at: None,
            skip: None,
            fail: None,
            storage: Arc::clone(storage),
            steps: AtomicUsize::new(0),
            applied: Mutex::new(Vec::new()),
        }
    }

    fn crashing_at(mut self, step: usize) -> Self {
        self.crash_at = Some(step);
        self
    }

    /// Passes one boundary between store writes, crashing there if asked.
    fn step(&self) {
        let step = self.steps.fetch_add(1, Ordering::SeqCst);
        assert_ne!(self.crash_at, Some(step), "crash at boundary {step}");
    }

    fn plan(&self, key: &ItemKey) -> Plan {
        self.items
            .iter()
            .find(|(item, _)| item == key)
            .map_or(Plan::Admit, |(_, plan)| plan.clone())
    }
}

#[async_trait]
impl Job for TestJob {
    type Error = TestError;

    fn sources(&self) -> Vec<Source> {
        vec![Source::Pmc]
    }

    async fn run(&self, store: &Store, lease: &Lease) -> Result<Completion, TestError> {
        if let Some(code) = &self.fail {
            return Err(TestError::Failed(code.clone()));
        }
        for (key, _) in &self.items {
            self.step();
            store.qualify(lease, key).await?;
        }
        for key in store.due_tonight(lease, Source::Pmc).await? {
            if self.skip.as_ref() == Some(&key) {
                continue;
            }
            self.step();
            store.apply(lease, &key, &self.plan(&key).event()).await?;
            self.applied.lock().unwrap().push(key);
        }
        if !self.build {
            return Ok(Completion::Done { activation: None });
        }
        let folder = CandidateFolder::of(lease);
        self.step();
        folder
            .create(
                &*self.storage,
                &ObjectPath::new("index/part-0.lance").unwrap(),
                b"vectors".to_vec(),
            )
            .await?;
        let version = VersionId::new(format!("v-{}", lease.run())).unwrap();
        let checked = Candidate::built(version)
            .check(|_| Ok::<(), std::convert::Infallible>(()))
            .unwrap();
        self.step();
        let activation = store.activate(lease, checked).await?;
        // After activation, before the night is finalized.
        self.step();
        Ok(Completion::Done {
            activation: Some(activation),
        })
    }
}

/// Runs one night in its own task. `None` when the run crashed.
async fn night(
    store: &Store,
    platform: &Arc<Platforms>,
    job: Arc<TestJob>,
    run: RunId,
) -> Option<Night> {
    let (store, platform) = (store.clone(), Arc::clone(platform));
    let task = tokio::spawn(async move {
        run_night(&store, &*platform, &slot(), run, &*job)
            .await
            .unwrap()
    });
    match task.await {
        Ok(night) => Some(night),
        Err(error) => {
            assert!(error.is_panic(), "the night's task didn't crash: {error}");
            None
        }
    }
}

fn finished(night: Option<Night>) -> NightOutcome {
    match night {
        Some(Night::Finished(outcome)) => outcome,
        other => panic!("expected a finished night, got {other:?}"),
    }
}

async fn states(store: &Store, keys: &[&str]) -> Vec<ArticleState> {
    let mut states = Vec::new();
    for key in keys {
        states.push(store.item(&pmc(key)).await.unwrap().unwrap());
    }
    states
}

fn ended() -> HistoryEntry {
    HistoryEntry::Night(NightOutcome::failed(FailureCode::ended_without_outcome()))
}

fn is_success(entry: &HistoryEntry) -> bool {
    matches!(entry, HistoryEntry::Night(night) if night.success().is_some())
}

const ITEMS: [&str; 3] = ["PMC1", "PMC2", "PMC3"];

fn admit_all() -> Vec<(&'static str, Plan)> {
    ITEMS.iter().map(|key| (*key, Plan::Admit)).collect()
}

/// AC1: a run killed partway through is taken over the next night, which
/// resumes from what the first admitted and loses none of it.
#[tokio::test]
async fn a_killed_night_resumes_without_losing_admitted_work() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let platform = Arc::new(Platforms::default());
    let storage = Arc::new(Recording::default());

    // Boundaries 0 to 2 qualify; 3 and 4 admit two items; 5 is the third.
    let first = RunId::new();
    let job = Arc::new(TestJob::new(&admit_all(), &storage).crashing_at(5));
    assert!(night(&store, &platform, job, first).await.is_none());
    assert_eq!(
        states(&store, &ITEMS).await[..2],
        [ArticleState::Admitted, ArticleState::Admitted]
    );
    assert_eq!(store.holder(&slot()).await.unwrap(), Some(first));

    platform.set(first, HolderState::Ended);
    let job = Arc::new(TestJob::new(&admit_all(), &storage));
    let outcome = finished(night(&store, &platform, Arc::clone(&job), RunId::new()).await);
    assert!(outcome.success().is_some(), "{outcome:?}");
    // The resumed night tried only what was still due.
    assert_eq!(*job.applied.lock().unwrap(), [pmc("PMC3")]);
    assert_eq!(
        states(&store, &ITEMS).await,
        vec![ArticleState::Admitted; 3]
    );
    let history = store.history(&slot()).await.unwrap();
    assert_eq!(history[0], ended());
    assert!(is_success(&history[1]));
    assert_eq!(history.len(), 2);
    scratch.drop().await;
}

/// AC2 and AC10: the same failure twice stops automatic attempts. The next
/// night doesn't run, though nothing asked the stop rule, and records the
/// reason a person reads; a resolution lets nights run again.
#[tokio::test]
async fn the_same_failure_twice_stops_and_says_why() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let platform = Arc::new(Platforms::default());
    let storage = Arc::new(Recording::default());
    let failing = || {
        let mut job = TestJob::new(&admit_all(), &storage);
        job.fail = Some(code("build_failed"));
        Arc::new(job)
    };
    for _ in 0..2 {
        let outcome = finished(night(&store, &platform, failing(), RunId::new()).await);
        assert_eq!(outcome, NightOutcome::failed(code("build_failed")));
    }
    let (failure, detail) = store.failure_details(&slot()).await.unwrap().remove(0);
    assert_eq!(failure, code("build_failed"));
    assert_eq!(detail, "the build image couldn't start");

    let why = StopReason::SameFailureTwice(code("build_failed"));
    match night(&store, &platform, failing(), RunId::new()).await {
        Some(Night::Stopped(stopped)) => assert_eq!(stopped, why),
        other => panic!("the nightly didn't stop: {other:?}"),
    }
    assert_eq!(
        store.history(&slot()).await.unwrap().last(),
        Some(&HistoryEntry::Night(NightOutcome::deferred(
            DeferralReason::Stopped(why.clone())
        )))
    );
    assert!(why.to_string().contains("build_failed"));
    assert!(
        store
            .alerts(&slot(), &[Source::Pmc])
            .await
            .unwrap()
            .contains(&Alert::Stopped(why))
    );
    // A stopped night took no work: nothing was qualified.
    assert_eq!(store.item(&pmc("PMC1")).await.unwrap(), None);

    store
        .record_resolution(&slot(), "operator@example.test", "Rebuilt the image")
        .await
        .unwrap();
    let job = Arc::new(TestJob::new(&admit_all(), &storage));
    assert!(
        finished(night(&store, &platform, job, RunId::new()).await)
            .success()
            .is_some()
    );
    scratch.drop().await;
}

/// AC2: three different failures in a row stop automatic attempts too.
#[tokio::test]
async fn three_failed_nights_in_a_row_stop() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let platform = Arc::new(Platforms::default());
    let storage = Arc::new(Recording::default());
    for failure in ["a_failed", "b_failed", "c_failed"] {
        let mut job = TestJob::new(&admit_all(), &storage);
        job.fail = Some(code(failure));
        finished(night(&store, &platform, Arc::new(job), RunId::new()).await);
    }
    let job = Arc::new(TestJob::new(&admit_all(), &storage));
    assert!(matches!(
        night(&store, &platform, job, RunId::new()).await,
        Some(Night::Stopped(StopReason::FailedNightsInARow))
    ));
    scratch.drop().await;
}

/// AC3: each night records one outcome, and only a complete night is a
/// success.
#[tokio::test]
async fn each_night_records_one_outcome_and_only_complete_nights_succeed() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let platform = Arc::new(Platforms::default());
    let storage = Arc::new(Recording::default());

    // One item set aside: published part of what was due.
    let partial = [
        ("PMC1", Plan::Admit),
        ("PMC2", Plan::Fail(code("http_503"))),
    ];
    let outcome = finished(
        night(
            &store,
            &platform,
            Arc::new(TestJob::new(&partial, &storage)),
            RunId::new(),
        )
        .await,
    );
    assert_eq!(
        outcome,
        NightOutcome::deferred(DeferralReason::PublishedWithSetAside)
    );
    // The retry succeeds: everything due was done and published.
    let outcome = finished(
        night(
            &store,
            &platform,
            Arc::new(TestJob::new(&admit_all(), &storage)),
            RunId::new(),
        )
        .await,
    );
    assert!(outcome.success().and_then(|s| s.activated()).is_some());
    // Nothing due: a success with no version.
    let mut idle = TestJob::new(&[], &storage);
    idle.build = false;
    let outcome = finished(night(&store, &platform, Arc::new(idle), RunId::new()).await);
    assert_eq!(outcome.success().map(|s| s.activated()), Some(None));

    let history = store.history(&slot()).await.unwrap();
    assert_eq!(history.len(), 3);
    assert_eq!(history.iter().filter(|entry| is_success(entry)).count(), 2);
    scratch.drop().await;
}

/// The boundaries a full night over three items passes.
const STEPS: usize = 9;

/// The boundaries a full, crash-free run passes, for the crash suite.
async fn boundaries() -> usize {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let platform = Arc::new(Platforms::default());
    let job = Arc::new(TestJob::new(&admit_all(), &Arc::new(Recording::default())));
    finished(night(&store, &platform, Arc::clone(&job), RunId::new()).await);
    scratch.drop().await;
    job.steps.load(Ordering::SeqCst)
}

/// The rules that must hold after any sequence of nights.
async fn rules_hold(store: &Store) {
    let history = store.history(&slot()).await.unwrap();
    // A success never follows work left undone: every success leaves
    // nothing due of what it qualified.
    if let Some(HistoryEntry::Night(night)) = history.last()
        && night.success().is_some()
    {
        assert_eq!(states(store, &ITEMS).await, vec![ArticleState::Admitted; 3]);
    }
    // At most one version is active (the database's index also holds this).
    let active = store.active_version().await.unwrap();
    if let Some(version) = &active {
        assert!(version.as_str().starts_with("v-"));
    }
}

/// AC4: crashing at each step boundary, then resuming, keeps every rule:
/// the crashed night is recorded failed, admitted work stays, and the
/// resumed night succeeds.
#[tokio::test]
async fn a_crash_at_any_boundary_resumes_and_keeps_the_rules() {
    let steps = boundaries().await;
    assert_eq!(
        steps, STEPS,
        "3 qualifies, 3 admits, a write, activation, finalization"
    );
    for crash in 0..steps {
        let scratch = Scratch::new().await;
        let store = scratch.store().await;
        let platform = Arc::new(Platforms::default());
        let storage = Arc::new(Recording::default());
        let crashed = RunId::new();
        let job = Arc::new(TestJob::new(&admit_all(), &storage).crashing_at(crash));
        assert!(night(&store, &platform, job, crashed).await.is_none());
        rules_hold(&store).await;

        platform.set(crashed, HolderState::Ended);
        let job = Arc::new(TestJob::new(&admit_all(), &storage));
        let outcome = finished(night(&store, &platform, job, RunId::new()).await);
        assert!(outcome.success().is_some(), "crash at {crash}: {outcome:?}");
        rules_hold(&store).await;
        let history = store.history(&slot()).await.unwrap();
        assert_eq!(history[0], ended(), "crash at {crash}");
        assert_eq!(history.len(), 2, "crash at {crash}");
        scratch.drop().await;
    }
}

/// AC4: a crash after activation and a second crash before the resumed
/// night finalizes are two takeovers, which stop the nightly; a resolution
/// resumes it, and the version search serves stays one a night checked.
#[tokio::test]
async fn two_crashes_around_activation_stop_until_resolved() {
    let steps = STEPS;
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let platform = Arc::new(Platforms::default());
    let storage = Arc::new(Recording::default());

    // Crash just after activation, before finalization.
    let first = RunId::new();
    let job = Arc::new(TestJob::new(&admit_all(), &storage).crashing_at(steps - 1));
    assert!(night(&store, &platform, job, first).await.is_none());
    let served = store.active_version().await.unwrap().unwrap();
    assert_eq!(served.as_str(), format!("v-{first}"));

    // The resumed night has nothing due, rebuilds, and crashes after its own
    // activation too.
    platform.set(first, HolderState::Ended);
    let second = RunId::new();
    let job = Arc::new(TestJob::new(&admit_all(), &storage).crashing_at(3 + 2));
    assert!(night(&store, &platform, job, second).await.is_none());
    rules_hold(&store).await;

    platform.set(second, HolderState::Ended);
    let job = Arc::new(TestJob::new(&admit_all(), &storage));
    assert!(matches!(
        night(&store, &platform, job, RunId::new()).await,
        Some(Night::Stopped(StopReason::SameFailureTwice(code)))
            if code == FailureCode::ended_without_outcome()
    ));
    // The stopped night released the dead run's hold.
    assert_eq!(store.holder(&slot()).await.unwrap(), None);
    assert_eq!(
        store.active_version().await.unwrap().unwrap().as_str(),
        format!("v-{second}")
    );

    store
        .record_resolution(&slot(), "operator@example.test", "Found the OOM")
        .await
        .unwrap();
    let job = Arc::new(TestJob::new(&admit_all(), &storage));
    assert!(
        finished(night(&store, &platform, job, RunId::new()).await)
            .success()
            .is_some()
    );
    rules_hold(&store).await;
    scratch.drop().await;
}

/// AC5: a run that wakes after losing its slot writes only into its own
/// candidate's folder, and can't activate or finish, so the version search
/// serves is the new holder's.
#[tokio::test]
async fn a_run_that_wakes_after_losing_its_slot_cannot_touch_the_active_version() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let platform = Platforms::default();
    let storage = Recording::default();
    let part = ObjectPath::new("index/part-0.lance").unwrap();
    let late = ObjectPath::new("index/part-1.lance").unwrap();

    let stale_run = RunId::new();
    let stale = match store
        .acquire(&slot(), &[Source::Pmc], stale_run, &platform)
        .await
        .unwrap()
    {
        Acquired::Holds(lease) => lease,
        other => panic!("the slot is free: {other:?}"),
    };
    let stale_folder = CandidateFolder::of(&stale);
    stale_folder.create(&storage, &part, vec![1]).await.unwrap();

    // The platform wrongly says the stale run ended, and another takes over.
    platform.set(stale_run, HolderState::Ended);
    let fresh = match store
        .acquire(&slot(), &[Source::Pmc], RunId::new(), &platform)
        .await
        .unwrap()
    {
        Acquired::Holds(lease) => lease,
        other => panic!("an ended holder's slot is taken: {other:?}"),
    };
    let fresh_folder = CandidateFolder::of(&fresh);
    fresh_folder.create(&storage, &part, vec![2]).await.unwrap();
    let fresh_version = VersionId::new("v-fresh").unwrap();
    let checked = Candidate::built(fresh_version.clone())
        .check(|_| Ok::<(), String>(()))
        .unwrap();
    let activation = store.activate(&fresh, checked).await.unwrap();

    // The stale run wakes: it writes into its own folder, then tries to
    // activate and finish.
    stale_folder.create(&storage, &late, vec![3]).await.unwrap();
    let checked = Candidate::built(VersionId::new("v-stale").unwrap())
        .check(|_| Ok::<(), String>(()))
        .unwrap();
    assert!(matches!(
        store.activate(&stale, checked).await,
        Err(StoreError::LeaseLost { run, .. }) if run == stale_run
    ));
    assert!(matches!(
        store
            .finish(stale, Ending::Completed { activation: None })
            .await,
        Err(StoreError::LeaseLost { .. })
    ));

    let stale_prefix = format!("candidates/{stale_run}/");
    let fresh_prefix = format!("candidates/{}/", fresh.run());
    let writes = storage.writes();
    assert_eq!(writes.len(), 3);
    assert_eq!(
        writes
            .iter()
            .filter(|key| key.as_str().starts_with(&fresh_prefix))
            .count(),
        1,
        "only the fresh run wrote into the fresh run's folder"
    );
    assert!(
        writes
            .iter()
            .all(|key| key.as_str().starts_with(&stale_prefix)
                || key.as_str().starts_with(&fresh_prefix))
    );
    assert_eq!(store.active_version().await.unwrap(), Some(fresh_version));
    let outcome = store
        .finish(
            fresh,
            Ending::Completed {
                activation: Some(activation),
            },
        )
        .await
        .unwrap();
    assert!(outcome.success().is_some());
    scratch.drop().await;
}

/// AC5: a run that activated its own version, then lost the slot before
/// finalizing, can't replace a file of the version search still serves, even
/// while the run that took over hasn't activated a version of its own.
#[tokio::test]
async fn a_run_that_lost_its_slot_cannot_replace_the_files_search_serves() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let platform = Platforms::default();
    let storage = Recording::default();
    let part = ObjectPath::new("index/part-0.lance").unwrap();

    let first_run = RunId::new();
    let Acquired::Holds(first) = acquire_as(&store, &platform, first_run).await else {
        panic!("the slot is free")
    };
    let folder = CandidateFolder::of(&first);
    folder.create(&storage, &part, vec![1]).await.unwrap();
    let served = VersionId::new("v-first").unwrap();
    let checked = Candidate::built(served.clone())
        .check(|_| Ok::<(), String>(()))
        .unwrap();
    store.activate(&first, checked).await.unwrap();

    // It loses the slot before finalizing; the run that takes over fails
    // without activating anything, so the first run's version still serves.
    platform.set(first_run, HolderState::Ended);
    let Acquired::Holds(second) = acquire_as(&store, &platform, RunId::new()).await else {
        panic!("an ended holder's slot is taken")
    };
    store
        .finish(
            second,
            Ending::Failed {
                code: code("build_failed"),
                detail: FailureDetail::new("the build image couldn't start"),
            },
        )
        .await
        .unwrap();
    assert_eq!(store.active_version().await.unwrap(), Some(served));

    // The first run wakes and tries to replace the file search serves.
    assert!(matches!(
        folder.create(&storage, &part, vec![9]).await,
        Err(StorageError::Exists(key)) if key == folder.key(&part)
    ));
    assert_eq!(storage.writes(), [folder.key(&part)]);
    assert!(matches!(
        store
            .finish(first, Ending::Completed { activation: None })
            .await,
        Err(StoreError::LeaseLost { .. })
    ));
    scratch.drop().await;
}

/// AC1, AC4 and AC8: content a night admitted but never published stays the
/// next night's to publish. A night that crashed after admitting and before
/// activating leaves the next one unable to succeed without activating.
#[tokio::test]
async fn admitted_content_left_unpublished_is_the_next_nights_to_publish() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let platform = Arc::new(Platforms::default());
    let storage = Arc::new(Recording::default());

    // Boundaries 0 to 5 qualify and admit all three; 6 is the write.
    let crashed = RunId::new();
    let job = Arc::new(TestJob::new(&admit_all(), &storage).crashing_at(6));
    assert!(night(&store, &platform, job, crashed).await.is_none());
    assert_eq!(
        states(&store, &ITEMS).await,
        vec![ArticleState::Admitted; 3]
    );
    platform.set(crashed, HolderState::Ended);

    // Nothing is due, but the admitted content still awaits a version.
    let mut idle = TestJob::new(&admit_all(), &storage);
    idle.build = false;
    let outcome = finished(night(&store, &platform, Arc::new(idle), RunId::new()).await);
    assert_eq!(
        outcome,
        NightOutcome::failed(FailureCode::new(FailureCode::DUE_WORK_LEFT_UNDONE).unwrap())
    );
    let (_, detail) = store.failure_details(&slot()).await.unwrap().remove(0);
    assert!(detail.contains("without activating a version"), "{detail}");

    // A night that builds and activates publishes it and succeeds.
    let job = Arc::new(TestJob::new(&admit_all(), &storage));
    let outcome = finished(night(&store, &platform, job, RunId::new()).await);
    assert!(outcome.success().and_then(|s| s.activated()).is_some());
    let mut idle = TestJob::new(&admit_all(), &storage);
    idle.build = false;
    let outcome = finished(night(&store, &platform, Arc::new(idle), RunId::new()).await);
    assert_eq!(outcome.success().map(|s| s.activated()), Some(None));
    scratch.drop().await;
}

/// AC7: nights that didn't update the library alert once there are three
/// in a row, even when a live run holding the slot keeps the stop rule from
/// counting them; an item that needs a person alerts too.
#[tokio::test]
async fn a_library_that_stops_updating_alerts() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let platform = Arc::new(Platforms::default());
    let storage = Arc::new(Recording::default());
    let refused = [("PMC1", Plan::Refuse), ("PMC2", Plan::Admit)];
    finished(
        night(
            &store,
            &platform,
            Arc::new(TestJob::new(&refused, &storage)),
            RunId::new(),
        )
        .await,
    );
    let sources = [Source::Pmc];
    // PMC1 has no readable text, which waits for a change without telling
    // anyone.
    assert_eq!(store.alerts(&slot(), &sources).await.unwrap(), []);

    // A live run holds the slot for days.
    let stuck = RunId::new();
    let lease = common::hold_slot(&store, &slot(), stuck).await;
    platform.set(stuck, HolderState::Alive);
    for nights in 1..=3 {
        let job = Arc::new(TestJob::new(&[], &storage));
        assert!(matches!(
            night(&store, &platform, job, RunId::new()).await,
            Some(Night::Waited(SlotWait::HolderAlive))
        ));
        let alerts = store.alerts(&slot(), &sources).await.unwrap();
        let not_updating = alerts
            .iter()
            .any(|alert| matches!(alert, Alert::NotUpdating { .. }));
        // The first night published part of what was due, which updated the
        // library; the waits since didn't.
        assert_eq!(
            not_updating,
            nights >= 3,
            "after {nights} waits: {alerts:?}"
        );
    }
    assert_eq!(store.next(&slot()).await.unwrap(), Next::Continue);

    // A new reader that can't read PMC1 is our own defect, which tells a
    // person at once.
    store
        .apply(&lease, &pmc("PMC1"), &ArticleEvent::ReaderChanged)
        .await
        .unwrap();
    store
        .apply(
            &lease,
            &pmc("PMC1"),
            &ArticleEvent::Refused(WaitReason::OwnDefect),
        )
        .await
        .unwrap();
    assert!(
        store
            .alerts(&slot(), &sources)
            .await
            .unwrap()
            .contains(&Alert::ItemsNeedPerson { count: 1 })
    );
    scratch.drop().await;
}

/// AC8: a night that says it finished but left due work undone, or admitted
/// content without publishing a version, fails with the reason; a person
/// resolving an item mid-night doesn't count against the night.
#[tokio::test]
async fn due_work_left_undone_cannot_record_success() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let platform = Arc::new(Platforms::default());
    let storage = Arc::new(Recording::default());
    let undone = FailureCode::new(FailureCode::DUE_WORK_LEFT_UNDONE).unwrap();

    let mut skipping = TestJob::new(&admit_all(), &storage);
    skipping.skip = Some(pmc("PMC2"));
    let outcome = finished(night(&store, &platform, Arc::new(skipping), RunId::new()).await);
    assert_eq!(outcome, NightOutcome::failed(undone.clone()));
    let (_, detail) = store.failure_details(&slot()).await.unwrap().remove(0);
    assert!(detail.contains("1 due items"), "{detail}");

    // Admitting PMC content without activating a version publishes nothing.
    let mut unpublished = TestJob::new(&admit_all(), &storage);
    unpublished.build = false;
    let outcome = finished(night(&store, &platform, Arc::new(unpublished), RunId::new()).await);
    assert_eq!(outcome, NightOutcome::failed(undone));
    // Two such nights stop the nightly, like any repeated failure.
    assert!(matches!(store.next(&slot()).await.unwrap(), Next::Stop(_)));
    store
        .record_resolution(&slot(), "operator@example.test", "Fixed the job")
        .await
        .unwrap();

    // Put PMC9 in front of a person, then start a night and resolve it
    // while the night runs: it isn't tonight's, so the night still succeeds.
    let lease = common::hold_slot(&store, &slot(), RunId::new()).await;
    store.qualify(&lease, &pmc("PMC9")).await.unwrap();
    for _ in 0..3 {
        store
            .apply(
                &lease,
                &pmc("PMC9"),
                &ArticleEvent::OutsideFailure(code("http_503")),
            )
            .await
            .unwrap();
    }
    store
        .finish(
            lease,
            Ending::Failed {
                code: code("http_503"),
                detail: FailureDetail::new("NCBI was down"),
            },
        )
        .await
        .unwrap();
    store
        .record_resolution(&slot(), "operator@example.test", "Reset the test")
        .await
        .unwrap();
    let lease = common::hold_slot(&store, &slot(), RunId::new()).await;
    assert!(matches!(
        store.resolve_item(&pmc("PMC9")).await.unwrap(),
        ArticleState::WaitingToRetry(_)
    ));
    // Qualifying it again, as a source's list does, doesn't make it
    // tonight's either.
    assert!(store.qualify(&lease, &pmc("PMC9")).await.unwrap().is_due());
    assert_eq!(store.tally(&lease).await.unwrap().due_left(), 0);
    // PMC2, admitted two nights ago without a version, is still this night's
    // to publish.
    assert!(store.tally(&lease).await.unwrap().needs_version());
    let checked = Candidate::built(VersionId::new("v-catch-up").unwrap())
        .check(|_| Ok::<(), String>(()))
        .unwrap();
    let activation = store.activate(&lease, checked).await.unwrap();
    let outcome = store
        .finish(
            lease,
            Ending::Completed {
                activation: Some(activation),
            },
        )
        .await
        .unwrap();
    assert!(outcome.success().is_some(), "{outcome:?}");
    // The next night counts it.
    let lease = common::hold_slot(&store, &slot(), RunId::new()).await;
    assert_eq!(store.tally(&lease).await.unwrap().due_left(), 1);
    scratch.drop().await;
}

/// AC8: an activation earlier in the night doesn't publish content the
/// night admitted after it, so the night can't finish with it as a success.
#[tokio::test]
async fn content_admitted_after_activation_keeps_the_night_from_succeeding() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let lease = common::hold_slot(&store, &slot(), RunId::new()).await;
    store.qualify(&lease, &pmc("PMC1")).await.unwrap();
    store
        .apply(&lease, &pmc("PMC1"), &ArticleEvent::Admitted)
        .await
        .unwrap();
    let checked = Candidate::built(VersionId::new("v1").unwrap())
        .check(|_| Ok::<(), String>(()))
        .unwrap();
    let activation = store.activate(&lease, checked).await.unwrap();
    // Admitted after the version was built and activated.
    store.qualify(&lease, &pmc("PMC2")).await.unwrap();
    store
        .apply(&lease, &pmc("PMC2"), &ArticleEvent::Admitted)
        .await
        .unwrap();
    let outcome = store
        .finish(
            lease,
            Ending::Completed {
                activation: Some(activation),
            },
        )
        .await
        .unwrap();
    assert_eq!(
        outcome,
        NightOutcome::failed(FailureCode::new(FailureCode::DUE_WORK_LEFT_UNDONE).unwrap())
    );
    scratch.drop().await;
}

/// AC9: an activation finishes only the night that made it, and only while
/// its version is the active one.
#[tokio::test]
async fn an_activation_finishes_only_its_own_night() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let checked = |name: &str| {
        Candidate::built(VersionId::new(name).unwrap())
            .check(|_| Ok::<(), String>(()))
            .unwrap()
    };

    let first = common::hold_slot(&store, &slot(), RunId::new()).await;
    let earlier = store.activate(&first, checked("v1")).await.unwrap();
    let kept = store.activate(&first, checked("v2")).await.unwrap();
    // v1 is no longer active, so its activation can't finish the night.
    assert!(matches!(
        store
            .finish(
                first.clone(),
                Ending::Completed {
                    activation: Some(earlier),
                },
            )
            .await,
        Err(StoreError::ForeignActivation { version, .. }) if version.as_str() == "v1"
    ));
    let outcome = store
        .finish(
            first,
            Ending::Completed {
                activation: Some(kept),
            },
        )
        .await
        .unwrap();
    assert!(outcome.success().is_some());

    // A later night can't claim success with an earlier night's activation.
    let second = common::hold_slot(&store, &slot(), RunId::new()).await;
    let third_activation = store.activate(&second, checked("v3")).await.unwrap();
    store.finish(second, Ending::NoCapacity).await.unwrap();
    let third = common::hold_slot(&store, &slot(), RunId::new()).await;
    assert!(matches!(
        store
            .finish(
                third,
                Ending::Completed {
                    activation: Some(third_activation),
                },
            )
            .await,
        Err(StoreError::ForeignActivation { .. })
    ));
    scratch.drop().await;
}

/// AC11: the store asks the platform about the holder itself, and acts on
/// each answer; a free slot asks nothing.
#[tokio::test]
async fn the_store_asks_the_platform_about_the_holder() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let platform = Platforms::default();
    let holder = RunId::new();

    assert!(matches!(
        acquire_as(&store, &platform, holder).await,
        Acquired::Holds(_)
    ));
    assert!(
        platform.asked.lock().unwrap().is_empty(),
        "a free slot asks nothing"
    );

    for (state, wait) in [
        (HolderState::Alive, SlotWait::HolderAlive),
        (HolderState::Unknown, SlotWait::HolderUnreadable),
    ] {
        platform.set(holder, state);
        match acquire_as(&store, &platform, RunId::new()).await {
            Acquired::Waited(waited) => assert_eq!(waited, wait),
            other => panic!("{state:?} gave {other:?}"),
        }
    }
    platform.set(holder, HolderState::Ended);
    assert!(matches!(
        acquire_as(&store, &platform, RunId::new()).await,
        Acquired::Holds(_)
    ));
    assert_eq!(*platform.asked.lock().unwrap(), [holder; 3]);
    assert_eq!(
        store.history(&slot()).await.unwrap(),
        [
            HistoryEntry::Night(NightOutcome::deferred(DeferralReason::SlotHeldByLiveRun)),
            HistoryEntry::Night(NightOutcome::deferred(DeferralReason::OwnerUnreadable)),
            ended(),
        ]
    );
    scratch.drop().await;
}

async fn acquire_as(store: &Store, platform: &Platforms, run: RunId) -> Acquired {
    store
        .acquire(&slot(), &[Source::Pmc], run, platform)
        .await
        .unwrap()
}

/// One step of the random-sequence test.
#[derive(Clone, Debug)]
enum Step {
    /// A night runs with these plans for the three items, crashing at the
    /// boundary given, if any.
    Night {
        plans: Vec<Plan>,
        crash: Option<usize>,
    },
    /// A person resolves an item that needs one.
    ResolveItem(usize),
    /// A person records that they resolved what stopped the nightly.
    Resolve,
}

fn plan() -> impl proptest::strategy::Strategy<Value = Plan> {
    use proptest::prelude::*;
    prop_oneof![
        4 => Just(Plan::Admit),
        2 => prop::sample::select(vec!["http_503", "ncbi_timeout"])
            .prop_map(|value| Plan::Fail(code(value))),
        1 => Just(Plan::Refuse),
    ]
}

fn step() -> impl proptest::strategy::Strategy<Value = Step> {
    use proptest::prelude::*;
    prop_oneof![
        6 => (prop::collection::vec(plan(), 3), prop::option::weighted(0.25, 0..STEPS))
            .prop_map(|(plans, crash)| Step::Night { plans, crash }),
        1 => (0..3_usize).prop_map(Step::ResolveItem),
        1 => Just(Step::Resolve),
    ]
}

/// Runs one sequence against a fresh schema, checking the rules after each
/// step against the records.
async fn run_sequence(steps: Vec<Step>) {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let platform = Arc::new(Platforms::default());
    let storage = Arc::new(Recording::default());
    let mut admitted: HashMap<&str, bool> = HashMap::new();

    for step in steps {
        let before = store.history(&slot()).await.unwrap();
        let states_before = items(&store).await;
        match step {
            Step::Night { plans, crash } => {
                let script: Vec<(&str, Plan)> = ITEMS.iter().copied().zip(plans).collect();
                let mut job = TestJob::new(&script, &storage);
                job.crash_at = crash;
                let run = RunId::new();
                let night = night(&store, &platform, Arc::new(job), run).await;
                // A crashed run is ended the next time anyone asks.
                platform.set(run, HolderState::Ended);
                if let Some(night) = &night {
                    // The rule as it stood when the night asked: the history
                    // before its own entry, including any takeover it made.
                    let after = store.history(&slot()).await.unwrap();
                    let prior = &after[..after.len() - 1];
                    let stopped = matches!(stop_rule::decide(prior), Next::Stop(_));
                    match night {
                        Night::Stopped(_) => assert!(stopped, "stopped without the rule"),
                        Night::Finished(_) => assert!(!stopped, "ran though stopped"),
                        other => panic!("unexpected night {other:?}"),
                    }
                }
                if let Some(Night::Finished(outcome)) = &night
                    && outcome.success().is_some()
                {
                    // Every item due before the night is admitted now.
                    for (key, state) in &states_before {
                        if state.as_ref().is_none_or(ArticleState::is_due) {
                            assert_eq!(
                                store.item(&pmc(key)).await.unwrap(),
                                Some(ArticleState::Admitted),
                                "{key} was due and the night succeeded"
                            );
                        }
                    }
                }
            }
            Step::ResolveItem(index) => {
                let key = pmc(ITEMS[index]);
                let state = store.item(&key).await.unwrap();
                let result = store.resolve_item(&key).await;
                match state {
                    Some(ArticleState::NeedsPerson { .. }) => {
                        assert!(result.unwrap().is_due());
                    }
                    _ => assert!(result.is_err(), "resolved an item that needed no one"),
                }
            }
            Step::Resolve => {
                store
                    .record_resolution(&slot(), "operator@example.test", "Looked into it")
                    .await
                    .unwrap();
            }
        }

        // Admitted work is never lost: nothing here changes a source.
        for (key, state) in items(&store).await {
            if admitted.get(key).copied().unwrap_or(false) {
                assert_eq!(
                    state,
                    Some(ArticleState::Admitted),
                    "{key} lost its admission"
                );
            }
            admitted.insert(key, state == Some(ArticleState::Admitted));
        }
        // Each run recorded at most one outcome, and the history reads back.
        let history = store.history(&slot()).await.unwrap();
        assert!(history.len() >= before.len());
        assert_eq!(
            history[..before.len()],
            before[..],
            "the history was rewritten"
        );
        rules_hold_on_versions(&store).await;
    }
    scratch.drop().await;
}

async fn items(store: &Store) -> Vec<(&'static str, Option<ArticleState>)> {
    let mut items = Vec::new();
    for key in ITEMS {
        items.push((key, store.item(&pmc(key)).await.unwrap()));
    }
    items
}

async fn rules_hold_on_versions(store: &Store) {
    // The database refuses a second active version; reading it must work.
    store.active_version().await.unwrap();
}

/// AC6: random sequences of nights, crashes and a person acting keep the
/// rules against real Postgres. A fixed number of cases keeps it in routine
/// CI.
#[test]
fn random_sequences_keep_the_rules() {
    use proptest::prelude::*;
    use proptest::test_runner::{Config, TestRunner};

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let mut runner = TestRunner::new(Config {
        cases: 24,
        failure_persistence: None,
        ..Config::default()
    });
    runner
        .run(&prop::collection::vec(step(), 1..14), |steps| {
            runtime.block_on(run_sequence(steps));
            Ok(())
        })
        .unwrap();
}
