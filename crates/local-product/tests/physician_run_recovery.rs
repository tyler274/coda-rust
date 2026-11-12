//! Real PostgreSQL integration suite for recovering interrupted physician runs.
//!
//! Exercises [`PhysicianHost`], [`PracticeJobHost`], [`Store`] reply revisions,
//! and [`harness::provider::ResponsesProvider`] against PostgreSQL:
//! - Mid-run restarts across read and mutating tools (`patient_concern`,
//!   `patient_memory`, `refresh_patient_sources`, `save_patient_understanding`)
//!   in both physician conversations and practice-owned background jobs.
//! - Storage failures after paid model calls (in-attempt retry and cross-attempt
//!   reuse), durable effects saved before replies, and atomic run persistence
//!   before acceptance acknowledgment.
//! - Steering, concurrent interactive and background runs, context/source
//!   changes, reply-draft edits, mid-run lost access, and `Stop` versus worker
//!   interruption.
//! - Source failures leaving work recoverable, and distinct representations for
//!   exhausted provider credits, lost access, and retryable interruptions.
//!
//! Runs automatically whenever `PHYSICIAN_RUNTIME_TEST_DATABASE_URL` or
//! `PHYSICIAN_DATABASE_URL` is set; panics if neither is set when
//! `CODA_REQUIRE_POSTGRES_TESTS=1`.

use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::Router;
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::post;
use harness::engine::recipe::{
    Finish, Observer, Product, Provider, ReplayEvent, StreamEvent, ToolFuture,
};
use harness::engine::recovery::Recovered;
use harness::engine::runs::{HoldReason, RunEvent, RunOutcome};
use harness::engine::{
    AcceptedInput, AttemptFailure, Cancellation, DispatchContext, Event, EventKind, Failure,
    Invocation, Item, LimitBackstop, ProductMessage, RunLimits, SavedRequest, TextDelta,
    ToolOutcome, Turn, history,
};
use harness::provider::{QuotaGate, ResponsesProvider};
use local_product::clinical_library::SEARCH_LIBRARY_TOOL;
use local_product::initial_understanding::{
    ConnectedSourceBrief, EvidenceScope, OperatingIndex, READ_UNDERSTANDING_TOOL,
    SAVE_UNDERSTANDING_TOOL, SaveUnderstandingArgs, SummaryClaimInput, UnderstandingContext,
    UnderstandingMode,
};
use local_product::physician_chat::PhysicianSkills;
use local_product::physician_sources::{
    MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SEARCH_TOOL, SourcePatient,
    SourceRevision,
};
use local_product::runtime::chat_operations::{
    MemorySelection, PATIENT_MEMORY_TOOL, PatientStartingContext, REFRESH_SOURCES_TOOL,
};
use local_product::runtime::concerns::{
    CONCERN_TOOL, ConcernStanding, ConcernUrgency, ExactRecordReference, WorkspaceAccess,
};
use local_product::runtime::delivery::{AttemptSettlement, Delivery, DeliveryState};
use local_product::runtime::jobs::{
    Assignment, PracticeAccessError, PracticeAuthority, PracticeJobClaim, PracticeJobCompletion,
    PracticeJobError, PracticeJobHost, PracticeJobKind, PracticeScope,
};
use local_product::runtime::refresh::{
    FixedCheckStep, FixedSourceCheck, REFRESH_TOOL, RefreshReport, SourceCheckOutcome, SourceId,
    SourceRefreshOutcome,
};
use local_product::runtime::reply::{
    DraftStatus, FINISH_TOOL, RevisionAccepted, RevisionCommit, RevisionInput,
};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::{
    Accepted, AttemptStatus, Authority, Claim, Conversation, Input, MessageRole, PhysicianHost,
    PublicationTransaction, Resumed, Scope, Stopped, Store, WorkStatus, async_trait,
    persistence_failure,
};
use local_product::{ModelBinding, ModelRoles};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use sea_orm::{ConnAcquireErr, DbErr};
use serde_json::{Value, json};
use tokio::sync::{Barrier, Notify, oneshot};
use uuid::Uuid;

fn database_url() -> Option<String> {
    if let Ok(url) = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
        .or_else(|_| std::env::var("PHYSICIAN_DATABASE_URL"))
    {
        let trimmed = url.trim().to_owned();
        if !trimmed.is_empty() {
            return Some(trimmed);
        }
    }
    assert!(
        std::env::var("CODA_REQUIRE_POSTGRES_TESTS").as_deref() != Ok("1"),
        "CODA_REQUIRE_POSTGRES_TESTS=1 requires PHYSICIAN_RUNTIME_TEST_DATABASE_URL or PHYSICIAN_DATABASE_URL"
    );
    None
}

fn load_skills() -> PhysicianSkills {
    PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
        .expect("load physician skills catalog")
}

fn distinct_roles() -> ModelRoles {
    ModelRoles {
        chat: ModelBinding {
            model: "recovery-chat-lane".into(),
            effort: "low".into(),
            context_window_tokens: 128_000,
            max_input_tokens: 100_000,
            adapter: "fixture".into(),
        },
        background: ModelBinding {
            model: "recovery-background-lane".into(),
            effort: "high".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 250_000,
            adapter: "fixture".into(),
        },
    }
}

async fn connect_stores(url: &str) -> (Store, Arc<PostgresAuthStore>) {
    let store = Store::connect(url).await.expect("connect runtime store");
    store.migrate().await.expect("migrate runtime store");
    let auth = Arc::new(
        PostgresAuthStore::connect_with_migrations(url)
            .await
            .expect("connect and migrate physician auth store"),
    );
    (store, auth)
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock after epoch")
        .as_secs()
}

struct FixtureEnv {
    practice: String,
    workspace: String,
    identity: PhysicianIdentity,
    session_hash: Vec<u8>,
    scope: Scope,
}

async fn setup_fixture(store: &Store, auth: &PostgresAuthStore, tag: &str) -> FixtureEnv {
    let suffix = Uuid::new_v4().simple().to_string();
    let practice = format!("prac-{tag}-{suffix}");
    let workspace = format!("pw-{tag}-{suffix}");
    let email = format!("dr-{tag}-{suffix}@codahealth.test");
    let now = unix_now();
    let invite = auth
        .invite(&practice, &email, "Dr. Recovery", false, now)
        .await
        .expect("invite physician");
    let (identity, token) = auth
        .redeem_invitation(&email, &invite, now)
        .await
        .expect("redeem invitation");
    auth.register_patient(&practice, &workspace)
        .await
        .expect("register patient");
    auth.set_membership(&identity, &workspace, Membership::Allowed)
        .await
        .expect("allow panel membership");

    let patient = SourcePatient {
        source_patient_id: format!("sp-{suffix}"),
        display_name: "Elena Navarro".into(),
        birth_date: Some("1968-04-14".into()),
        sex_or_gender: Some("female".into()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "ehr-mrn".into(),
            value: format!("MRN-{suffix}"),
        }],
    };
    let seeded_ehr = [chart_record(
        &workspace,
        "ehr",
        "visit-note-1",
        "rev-1",
        "note",
        "Hypertension follow-up note",
        "2026-10-15",
        "Blood pressure 148/92 mmHg on amlodipine 5 mg daily; potassium 4.3 mEq/L.",
    )];
    let seeded_labs = [chart_record(
        &workspace,
        "labs",
        "lab-potassium",
        "rev-1",
        "laboratory",
        "Basic metabolic panel potassium",
        "2026-10-15",
        "Serum potassium 4.3 mEq/L; creatinine 1.0 mg/dL.",
    )];
    store
        .record_chart(
            &workspace,
            &seeded_ehr,
            &SourcePull {
                source_id: "ehr".into(),
                source_label: "Clinic EHR".into(),
                patient: patient.clone(),
                pulled_at: 1_760_500_000,
            },
        )
        .await
        .expect("seed ehr pull");
    store
        .record_chart(
            &workspace,
            &seeded_labs,
            &SourcePull {
                source_id: "labs".into(),
                source_label: "Regional Labs".into(),
                patient,
                pulled_at: 1_760_500_010,
            },
        )
        .await
        .expect("seed labs pull");

    let session_hash = PostgresAuthStore::session_reference(&token);
    let scope = Scope {
        practice: practice.clone(),
        physician: identity.physician_id.clone(),
        workspace: workspace.clone(),
        chat: format!("chat-{tag}-{suffix}"),
    };
    FixtureEnv {
        practice,
        workspace,
        identity,
        session_hash,
        scope,
    }
}

#[allow(clippy::too_many_arguments)]
fn chart_record(
    workspace: &str,
    source_id: &str,
    candidate_id: &str,
    revision: &str,
    record_type: &str,
    title: &str,
    date: &str,
    text: &str,
) -> SourceRevision {
    let content = json!({
        "resourceType": "DocumentReference",
        "id": candidate_id,
        "meta": { "versionId": revision },
        "description": title,
        "text": text,
    })
    .to_string();
    let sha256 = local_product::sha256_hex(content.as_bytes());
    SourceRevision {
        workspace: workspace.to_owned(),
        source_id: source_id.to_owned(),
        candidate_id: candidate_id.to_owned(),
        revision: revision.to_owned(),
        record_type: record_type.to_owned(),
        dates: vec![RecordDate {
            name: "recorded".into(),
            value: date.to_owned(),
        }],
        content,
        sha256,
        readable_parts: vec![ReadablePart {
            label: title.to_owned(),
            text: text.to_owned(),
        }],
        provenance: json!({
            "source": source_id,
            "native_id": candidate_id,
            "uri": format!("https://chart.codahealth.test/{source_id}/{candidate_id}/{revision}"),
        }),
    }
}

/// Real PostgreSQL session/panel authority matching `physician-worker`.
struct PostgresRuntimeAuthority;

#[async_trait]
impl Authority for PostgresRuntimeAuthority {
    async fn check(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        hash: Option<&[u8]>,
    ) -> Result<(), String> {
        PostgresAuthStore::check_runtime_authority(
            tx,
            hash.ok_or("accepted session reference missing")?,
            &PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|err| err.to_string())
    }

    async fn lock_publication(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        hash: Option<&[u8]>,
    ) -> Result<(), String> {
        PostgresAuthStore::lock_runtime_authority(
            tx,
            hash.ok_or("accepted session reference missing")?,
            &PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|err| err.to_string())
    }
}

fn materialized_output<'a>(
    request: &'a harness::engine::recipe::Request,
    call_id: &str,
) -> Option<&'a str> {
    request.items.iter().find_map(|item| match item {
        Item::MaterializedResult {
            call_id: id,
            output,
        } if id == call_id => Some(output.as_str()),
        _ => None,
    })
}

/// Exact persistence fault mode injected around real [`PhysicianHost`] or
/// [`PracticeJobHost`] PostgreSQL operations.
enum PersistenceFault {
    /// Fail the next `remaining` `ProviderReturned` saves with retryable pool timeout.
    RetryableOnProviderReturned { remaining: usize },
    /// Cancel and abort right before `ToolReturned` for `target_call_id` is saved,
    /// after the underlying tool's PostgreSQL transaction has already committed.
    CrashBeforeToolReturned {
        target_call_id: String,
        cancel: Cancellation,
        tripped: bool,
    },
    /// Fail `Candidate` persistence with a non-retryable storage error after
    /// `ProviderReturned` (and any prior tool effects) committed.
    FailCandidatePersist,
    /// Fail `Product::finish` (`commit_authorized` / `commit_practice_job`) after
    /// `Candidate` and tool side effects committed.
    FailFinish,
}

struct FaultInjectingProduct<P> {
    inner: P,
    fault: PersistenceFault,
    provider_returned_saves: usize,
}

impl<P> FaultInjectingProduct<P> {
    fn new(inner: P, fault: PersistenceFault) -> Self {
        Self {
            inner,
            fault,
            provider_returned_saves: 0,
        }
    }
}

#[async_trait]
impl<P: Product> Product for FaultInjectingProduct<P> {
    async fn message_sources(
        &mut self,
    ) -> Result<Option<Vec<ProductMessage>>, harness::engine::Cause> {
        self.inner.message_sources().await
    }

    async fn admit_binding(
        &mut self,
        recipe: &harness::engine::recipe::RequestBinding,
    ) -> Result<(), harness::engine::Cause> {
        self.inner.admit_binding(recipe).await
    }

    fn read_only(&self, name: &str) -> bool {
        self.inner.read_only(name)
    }

    fn read_result(&mut self, id: &str) -> ToolFuture<Option<String>> {
        self.inner.read_result(id)
    }

    fn saved_outcome(&mut self, call: &Invocation) -> ToolFuture<Option<ToolOutcome>> {
        self.inner.saved_outcome(call)
    }

    async fn assert_current(&mut self) -> Result<(), harness::engine::Cause> {
        self.inner.assert_current().await
    }

    async fn persist(&mut self, event: Event) -> Result<(), harness::engine::Cause> {
        if matches!(&event.kind, EventKind::ProviderReturned(_)) {
            self.provider_returned_saves += 1;
            if let PersistenceFault::RetryableOnProviderReturned { remaining } = &mut self.fault
                && *remaining > 0
            {
                *remaining -= 1;
                return Err(persistence_failure(DbErr::ConnectionAcquire(
                    ConnAcquireErr::Timeout,
                )));
            }
        }
        if let EventKind::ToolReturned { call_id, .. } = &event.kind
            && let PersistenceFault::CrashBeforeToolReturned {
                target_call_id,
                cancel,
                tripped,
            } = &mut self.fault
            && !*tripped
            && call_id == target_call_id
        {
            *tripped = true;
            cancel.cancel();
            return Err("simulated worker crash before ToolReturned persisted".into());
        }
        if matches!(&event.kind, EventKind::Candidate { .. })
            && matches!(self.fault, PersistenceFault::FailCandidatePersist)
        {
            return Err(DbErr::Custom("injected Candidate storage failure".into()).into());
        }
        self.inner.persist(event).await
    }

    fn execute(&mut self, call: &Invocation, arguments: Value) -> ToolFuture {
        self.inner.execute(call, arguments)
    }

    async fn finish(&mut self, turn: &Turn) -> Result<Finish, harness::engine::Cause> {
        if matches!(self.fault, PersistenceFault::FailFinish) {
            return Err(DbErr::Custom("injected finish commit failure".into()).into());
        }
        self.inner.finish(turn).await
    }

    async fn finish_partial(
        &mut self,
        turn: &Turn,
        backstops: &[LimitBackstop],
    ) -> Result<(), harness::engine::Cause> {
        self.inner.finish_partial(turn, backstops).await
    }

    async fn window_seed(&mut self) -> Result<harness::engine::WindowSeed, harness::engine::Cause> {
        self.inner.window_seed().await
    }
}

fn chat_message_sources(state: &Conversation) -> Vec<ProductMessage> {
    state
        .messages
        .iter()
        .map(|msg| match &msg.role {
            MessageRole::User { id, .. } => ProductMessage {
                id: id.clone(),
                role: history::Role::User,
                text: msg.text.clone(),
            },
            MessageRole::Assistant { id, .. } => ProductMessage {
                id: id.clone(),
                role: history::Role::Assistant,
                text: msg.text.clone(),
            },
        })
        .collect()
}

fn chat_message_items(msg: &local_product::runtime::Message) -> Vec<Item> {
    let source = match &msg.role {
        MessageRole::User { id, .. } => ProductMessage {
            id: id.clone(),
            role: history::Role::User,
            text: msg.text.clone(),
        },
        MessageRole::Assistant { id, .. } => ProductMessage {
            id: id.clone(),
            role: history::Role::Assistant,
            text: msg.text.clone(),
        },
    };
    let mut items = vec![Item::ProductMessage(source)];
    if matches!(msg.role, MessageRole::Assistant { partial: true, .. }) {
        items.push(history::partial_answer());
    }
    items
}

async fn build_chat_work_for_fault_host(
    store: &Store,
    claim: &Claim,
    starting_context: &PatientStartingContext,
) -> Recovered {
    let state = store
        .conversation(&claim.scope)
        .await
        .expect("load conversation");
    let binding_ref = state
        .bindings
        .get(&state.work)
        .expect("accepted work binding");
    let mut request = store
        .frozen_binding(&claim.scope, binding_ref)
        .await
        .expect("load frozen binding")
        .request(Vec::new());
    request.items.clear();

    let mut pending: Vec<AcceptedInput> = Vec::new();
    for message in &state.messages {
        if let MessageRole::User { id, work, .. } = &message.role
            && work == &state.work
        {
            let items = if pending.is_empty() {
                vec![starting_context.developer_item()]
            } else {
                Vec::new()
            };
            pending.push(AcceptedInput {
                id: id.clone(),
                items,
            });
        }
        let target = match pending.last_mut() {
            Some(input) => &mut input.items,
            None => &mut request.items,
        };
        target.extend(chat_message_items(message));
    }

    let work_start = state
        .messages
        .iter()
        .position(|m| matches!(&m.role, MessageRole::User { work, .. } if work == &state.work))
        .expect("work start index");
    let partial = state.messages[work_start..]
        .iter()
        .rev()
        .find_map(|m| match &m.role {
            MessageRole::Assistant { id, partial: true } => Some(id.clone()),
            _ => None,
        });
    let events: Vec<_> = state
        .attempts
        .iter()
        .filter(|attempt| attempt.work == state.work)
        .flat_map(|attempt| {
            let current = attempt.status != AttemptStatus::Cancelled;
            attempt.events.iter().map(move |event| {
                if current {
                    ReplayEvent::Admitted(event)
                } else {
                    ReplayEvent::EvidenceOnly(event)
                }
            })
        })
        .collect();
    let sources = chat_message_sources(&state);
    request
        .freeze()
        .expect("freeze request")
        .recover(
            &claim.attempt,
            harness::engine::recovery::Records {
                events,
                allowance_after: state.budget_resume_after,
            },
            harness::engine::recovery::Messages {
                sources: &sources,
                partial,
            },
            harness::engine::recovery::Inputs {
                accepted: pending,
                source_feedback: None,
            },
        )
        .expect("recover chat work")
}

async fn build_practice_job_work_for_fault_host(
    store: &Store,
    claim: &PracticeJobClaim,
) -> Recovered {
    let job = store
        .practice_job(&claim.scope)
        .await
        .expect("query practice job")
        .expect("practice job exists");
    let mut request = store
        .practice_job_binding(&claim.scope, &job.binding)
        .await
        .expect("load practice job binding")
        .request(Vec::new());
    request.items.clear();

    let assignment_msg = ProductMessage {
        id: format!("{}:assignment", job.assignment.key),
        role: history::Role::User,
        text: job.assignment.prompt.clone(),
    };
    let mut accepted_items = vec![Item::ProductMessage(assignment_msg.clone())];
    let mut message_sources = vec![assignment_msg];
    let mut partial = None;
    if let Some(saved) = &job.result {
        let id = format!("{}:result", saved.attempt_id);
        let prod_msg = ProductMessage {
            id: id.clone(),
            role: history::Role::Assistant,
            text: saved.output_text.clone(),
        };
        message_sources.push(prod_msg.clone());
        if saved.completion == PracticeJobCompletion::Partial {
            accepted_items.push(Item::ProductMessage(prod_msg));
            accepted_items.push(history::partial_answer());
            partial = Some(id);
        }
    }
    let events: Vec<_> = job
        .attempts
        .iter()
        .flat_map(|attempt| {
            let current = attempt.status != AttemptStatus::Cancelled;
            attempt.events.iter().map(move |event| {
                if current {
                    ReplayEvent::Admitted(event)
                } else {
                    ReplayEvent::EvidenceOnly(event)
                }
            })
        })
        .collect();
    request
        .freeze()
        .expect("freeze practice job request")
        .recover(
            &claim.attempt,
            harness::engine::recovery::Records {
                events,
                allowance_after: job.allowance_after,
            },
            harness::engine::recovery::Messages {
                sources: &message_sources,
                partial,
            },
            harness::engine::recovery::Inputs {
                accepted: vec![AcceptedInput {
                    id: job.work.clone(),
                    items: accepted_items,
                }],
                source_feedback: None,
            },
        )
        .expect("recover practice job work")
}

// ============================================================================
// AC1 — Mid-run restart in a physician conversation and a background job
// ============================================================================

/// AC1 (Conversation): Restart mid-run across read tools and mutating tools
/// (`refresh_patient_sources` with the chat `{source_id, reason}` schema,
/// `patient_concern`, and `patient_memory`), covering:
/// 1. Completed `ToolReturned` reused by call ID without re-execution,
/// 2. Crash between `ToolStarted` and `ToolReturned` *after* durable commit
///    reconciled read-only via `saved_outcome` with zero repeated effects or
///    repeated source pulls, and
/// 3. Crash between `ToolStarted` and `ToolReturned` *before* commit reported
///    as `"outcome": "unknown"` to the successor attempt inside the same run,
///    resuming to store one assistant reply.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn ac1_chat_mid_run_restart_reconciles_saved_effects_and_unknown_calls_into_one_reply() {
    let Some(url) = database_url() else {
        return;
    };
    let (base_store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();
    let env = setup_fixture(&base_store, &auth, "ac1-chat").await;

    let ehr_id = SourceId::new("ehr").unwrap();
    let labs_id = SourceId::new("labs").unwrap();
    let checker = FixedSourceCheck::default();
    checker.set_source_outcome(
        &env.practice,
        &env.workspace,
        ehr_id.clone(),
        SourceCheckOutcome::Updated {
            source_label: "Clinic EHR".into(),
            patient: SourcePatient {
                source_patient_id: "sp-ac1-chat".into(),
                display_name: "Elena Navarro".into(),
                birth_date: Some("1968-04-14".into()),
                sex_or_gender: Some("female".into()),
                medical_record_numbers: Vec::new(),
            },
            pulled_at: 1_760_600_000,
            records: vec![chart_record(
                &env.workspace,
                "ehr",
                "visit-note-2",
                "rev-2",
                "note",
                "Repeat blood pressure check",
                "2026-11-10",
                "Repeat office blood pressure 156/96 mmHg; patient reports occasional headaches.",
            )],
        },
    );
    checker.set_source_outcome(
        &env.practice,
        &env.workspace,
        labs_id.clone(),
        SourceCheckOutcome::Updated {
            source_label: "Regional Labs".into(),
            patient: SourcePatient {
                source_patient_id: "sp-ac1-chat".into(),
                display_name: "Elena Navarro".into(),
                birth_date: Some("1968-04-14".into()),
                sex_or_gender: Some("female".into()),
                medical_record_numbers: Vec::new(),
            },
            pulled_at: 1_760_600_010,
            records: Vec::new(),
        },
    );
    let store = base_store.with_source_refresh(Arc::new(checker.clone()));

    let chat_recipe = store
        .chat_recipe_with_understanding(
            &roles.chat,
            local_product::physician_chat::INSTRUCTIONS,
            &skills,
            &env.practice,
            &env.workspace,
        )
        .await
        .expect("build chat recipe");
    let accepted = store
        .accept_authorized(
            &env.scope,
            &Input {
                key: "msg-ac1-1".into(),
                text: "Refresh sources, read the hypertension note, record the allergy correction, and raise an urgent blood pressure concern.".into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            env.session_hash.clone(),
            &skills,
            &chat_recipe,
        )
        .await
        .expect("accept chat message");
    assert_eq!(accepted, Accepted::New);

    let initial_state = store.conversation(&env.scope).await.unwrap();
    let initial_run_id = initial_state.run().expect("open run").id.to_owned();

    // --- Attempt 1: executes read_patient_sources (completed ToolReturned),
    // patient_concern (completed ToolReturned), refresh_patient_sources with
    // chat schema `{source_id: null, reason: ...}` -> commits in Postgres, then
    // crashes right BEFORE ToolReturned is persisted! ---
    let claim_1 = store
        .claim(&env.scope, 60)
        .await
        .unwrap()
        .expect("claim attempt 1");
    let starting_context = store
        .patient_starting_context(&env.scope)
        .await
        .expect("starting context");
    let work_1 = build_chat_work_for_fault_host(&store, &claim_1, &starting_context).await;
    let cancel_1 = Cancellation::default();
    let mut fault_host_1 = FaultInjectingProduct::new(
        PhysicianHost {
            store: store.clone(),
            claim: claim_1.clone(),
            authority: PostgresRuntimeAuthority,
        },
        PersistenceFault::CrashBeforeToolReturned {
            target_call_id: "call-chat-refresh".into(),
            cancel: cancel_1.clone(),
            tripped: false,
        },
    );

    struct Attempt1ChatProvider;
    impl Provider for Attempt1ChatProvider {
        async fn exchange(
            &mut self,
            _request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            Ok(Turn {
                items: vec![
                    Item::Call(Invocation {
                        id: "call-read-seeded".into(),
                        name: READ_TOOL.into(),
                        arguments: json!({
                            "references": [{ "candidate_id": "visit-note-1", "revision": "rev-1" }],
                            "max_result_bytes": 16_384,
                        })
                        .to_string(),
                    }),
                    Item::Call(Invocation {
                        id: "call-raise-concern".into(),
                        name: CONCERN_TOOL.into(),
                        arguments: json!({
                            "operation": "raise",
                            "concern_id": "bp-uncontrolled",
                            "urgency": "now",
                            "reason": "Persistent stage 2 hypertension requiring medication adjustment.",
                            "records": [{ "candidate_id": "visit-note-1", "revision": "rev-1" }],
                        })
                        .to_string(),
                    }),
                    Item::Call(Invocation {
                        id: "call-chat-refresh".into(),
                        name: REFRESH_SOURCES_TOOL.into(),
                        arguments: json!({
                            "source_id": null,
                            "reason": "Check Clinic EHR and Regional Labs for follow-up blood pressure readings.",
                        })
                        .to_string(),
                    }),
                ],
                receipt: json!({ "attempt": 1 }),
                usage: Default::default(),
            })
        }
    }

    let err_1 = work_1
        .run(
            &mut Attempt1ChatProvider,
            &mut fault_host_1,
            RunLimits::default(),
            &cancel_1,
        )
        .await
        .expect_err("attempt 1 crashes before ToolReturned for call-chat-refresh");
    let reason_1 = AttemptFailure::from_error(&err_1, &cancel_1);
    assert_eq!(reason_1, AttemptFailure::Interrupted);
    store
        .settle_chat_attempt(&claim_1, reason_1)
        .await
        .expect("settle interrupted attempt 1");

    assert_eq!(
        checker.call_count(&env.practice, &env.workspace, &ehr_id),
        1
    );
    assert_eq!(
        checker.call_count(&env.practice, &env.workspace, &labs_id),
        1
    );

    // --- Attempt 2: simulates a second mid-run crash where `saved_outcome`
    // reconciles `call-chat-refresh` WITHOUT re-pulling sources, then starts
    // `call-memory-correction` and crashes BEFORE the memory mutation commits
    // (`ToolStarted` recorded with no receipt -> `"outcome": "unknown"` on Attempt 3). ---
    let claim_2 = store
        .claim(&env.scope, 60)
        .await
        .unwrap()
        .expect("claim attempt 2");
    assert_eq!(claim_2.revision, claim_1.revision);

    // Persist a second round on Attempt 2 that reconciles Attempt 1 first, then
    // cancels inside a slow memory operation before committing.
    struct SlowAbortingMemoryOp {
        cancel: Cancellation,
        inner: local_product::runtime::chat_operations::PostgresPatientMemoryOperation,
        abort_next: AtomicBool,
    }

    #[async_trait]
    impl local_product::runtime::chat_operations::PatientMemoryOperation for SlowAbortingMemoryOp {
        async fn list_memories(
            &self,
            store: &Store,
            practice: &str,
            workspace: &str,
            selection: MemorySelection,
        ) -> Result<
            Vec<local_product::runtime::chat_operations::PatientMemoryEntry>,
            local_product::runtime::chat_operations::PatientMemoryError,
        > {
            self.inner
                .list_memories(store, practice, workspace, selection)
                .await
        }

        async fn record_correction(
            &self,
            store: &Store,
            command_key: &str,
            input: &local_product::runtime::chat_operations::RecordMemoryCorrection,
        ) -> Result<
            local_product::runtime::chat_operations::MemoryMutation,
            local_product::runtime::chat_operations::PatientMemoryError,
        > {
            if self.abort_next.swap(false, Ordering::SeqCst) {
                self.cancel.cancel();
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
            self.inner
                .record_correction(store, command_key, input)
                .await
        }

        async fn retire_memory(
            &self,
            store: &Store,
            command_key: &str,
            input: &local_product::runtime::chat_operations::RetirePatientMemory,
        ) -> Result<
            local_product::runtime::chat_operations::MemoryMutation,
            local_product::runtime::chat_operations::PatientMemoryError,
        > {
            self.inner.retire_memory(store, command_key, input).await
        }

        async fn saved_mutation(
            &self,
            store: &Store,
            practice: &str,
            workspace: &str,
            command_key: &str,
            id: &local_product::runtime::chat_operations::PatientMemoryId,
        ) -> Result<
            Option<local_product::runtime::chat_operations::PatientMemoryEntry>,
            local_product::runtime::chat_operations::PatientMemoryError,
        > {
            self.inner
                .saved_mutation(store, practice, workspace, command_key, id)
                .await
        }
    }

    let cancel_2 = Cancellation::default();
    let memory_op = Arc::new(SlowAbortingMemoryOp {
        cancel: cancel_2.clone(),
        inner: local_product::runtime::chat_operations::PostgresPatientMemoryOperation,
        abort_next: AtomicBool::new(true),
    });
    let store_with_mem = store.clone().with_memory_operation(memory_op);

    struct Attempt2ChatProvider;
    impl Provider for Attempt2ChatProvider {
        async fn exchange(
            &mut self,
            request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            let refresh_out = materialized_output(request, "call-chat-refresh")
                .expect("call-chat-refresh output present in attempt 2");
            assert!(
                refresh_out.contains("\"outcome\":\"refreshed\""),
                "chat refresh committed before restart must reconcile via saved_outcome without reporting unknown: {refresh_out}"
            );
            Ok(Turn {
                items: vec![Item::Call(Invocation {
                    id: "call-mem-uncommitted".into(),
                    name: PATIENT_MEMORY_TOOL.into(),
                    arguments: json!({
                        "operation": "record_correction",
                        "memory_id": "allergy-lisinopril",
                        "topic": "ACE inhibitor intolerance",
                        "statement": "Prior cough on lisinopril reported by patient; avoid ACE inhibitors.",
                    })
                    .to_string(),
                })],
                receipt: json!({ "attempt": 2 }),
                usage: Default::default(),
            })
        }
    }

    let mut host_2 = PhysicianHost {
        store: store_with_mem.clone(),
        claim: claim_2.clone(),
        authority: PostgresRuntimeAuthority,
    };
    let err_2 = host_2
        .run(&mut Attempt2ChatProvider, RunLimits::default(), &cancel_2)
        .await
        .expect_err("attempt 2 cancels before memory mutation commits");
    let reason_2 = AttemptFailure::from_error(&err_2, &cancel_2);
    assert_eq!(reason_2, AttemptFailure::Interrupted);
    store
        .settle_chat_attempt(&claim_2, reason_2)
        .await
        .expect("settle attempt 2");

    // Verify zero source re-pulls occurred during Attempt 2's reconciliation!
    assert_eq!(
        checker.call_count(&env.practice, &env.workspace, &ehr_id),
        1,
        "chat refresh reconciliation must not re-pull ehr"
    );
    assert_eq!(
        checker.call_count(&env.practice, &env.workspace, &labs_id),
        1,
        "chat refresh reconciliation must not re-pull labs"
    );

    // --- Attempt 3: sees `"outcome": "unknown"` for `call-mem-uncommitted`,
    // checks memory, applies the correction once, reads refreshed `visit-note-2`,
    // and publishes one final reply! ---
    let claim_3 = store
        .claim(&env.scope, 60)
        .await
        .unwrap()
        .expect("claim attempt 3");

    struct Attempt3ChatProvider {
        round: usize,
    }
    impl Provider for Attempt3ChatProvider {
        async fn exchange(
            &mut self,
            request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            self.round += 1;
            match self.round {
                1 => {
                    let unknown_mem = materialized_output(request, "call-mem-uncommitted")
                        .expect("unknown output for uncommitted memory call");
                    assert!(
                        unknown_mem.contains("\"outcome\":\"unknown\""),
                        "uncommitted tool call must report unknown outcome: {unknown_mem}"
                    );
                    Ok(Turn {
                        items: vec![
                            Item::Call(Invocation {
                                id: "call-read-updated".into(),
                                name: READ_TOOL.into(),
                                arguments: json!({
                                    "references": [{ "candidate_id": "visit-note-2", "revision": "rev-2" }],
                                    "max_result_bytes": 16_384,
                                })
                                .to_string(),
                            }),
                            Item::Call(Invocation {
                                id: "call-mem-apply".into(),
                                name: PATIENT_MEMORY_TOOL.into(),
                                arguments: json!({
                                    "operation": "record_correction",
                                    "memory_id": "allergy-lisinopril",
                                    "topic": "ACE inhibitor intolerance",
                                    "statement": "Prior cough on lisinopril reported by patient; avoid ACE inhibitors.",
                                })
                                .to_string(),
                            }),
                        ],
                        receipt: json!({ "attempt": 3, "round": 1 }),
                        usage: Default::default(),
                    })
                }
                2 => {
                    let updated_note = materialized_output(request, "call-read-updated")
                        .expect("updated note output");
                    assert!(updated_note.contains("156/96 mmHg"));
                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Blood pressure rose to 156/96 mmHg on 2026-11-10 [2] from 148/92 mmHg on 2026-10-15 [1]. Raised an urgent blood pressure concern and recorded ACE inhibitor intolerance.\n\nBasis and limitations: Clinic EHR notes dated 2026-10-15 and 2026-11-10 and physician-supplied allergy correction.",
                        )],
                        receipt: json!({ "attempt": 3, "round": 2 }),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected round {other}"),
            }
        }
    }

    let mut host_3 = PhysicianHost {
        store: store_with_mem.clone(),
        claim: claim_3.clone(),
        authority: PostgresRuntimeAuthority,
    };
    host_3
        .run(
            &mut Attempt3ChatProvider { round: 0 },
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .expect("attempt 3 completes");

    let final_state = store.conversation(&env.scope).await.unwrap();
    assert_eq!(final_state.status, WorkStatus::Completed);
    assert_eq!(final_state.failures, 0);
    let final_run = final_state.run().expect("completed run");
    assert_eq!(final_run.id, initial_run_id, "all attempts stay in one run");
    assert_eq!(final_run.outcome, Some(RunOutcome::Completed));
    assert_eq!(final_state.attempts.len(), 3);
    assert_eq!(final_state.attempts[0].status, AttemptStatus::Interrupted);
    assert_eq!(final_state.attempts[1].status, AttemptStatus::Interrupted);
    assert_eq!(final_state.attempts[2].status, AttemptStatus::Completed);
    assert_eq!(
        final_state.attempts[1].predecessor.as_deref(),
        Some(claim_1.attempt.as_str())
    );
    assert_eq!(
        final_state.attempts[2].predecessor.as_deref(),
        Some(claim_2.attempt.as_str())
    );

    let replies: Vec<_> = final_state.responses().collect();
    assert_eq!(replies.len(), 1, "one stored assistant reply");
    assert!(replies[0].text.contains("156/96 mmHg"));

    // Verify no repeated side effects across all three attempts:
    assert_eq!(
        checker.call_count(&env.practice, &env.workspace, &ehr_id),
        1
    );
    assert_eq!(
        checker.call_count(&env.practice, &env.workspace, &labs_id),
        1
    );
    let concerns = store
        .workspace_concerns(
            &WorkspaceAccess {
                practice: &env.practice,
                physician: &env.identity.physician_id,
                workspace: &env.workspace,
                session_hash: &env.session_hash,
            },
            &mut PostgresRuntimeAuthority,
        )
        .await
        .expect("read concerns");
    assert_eq!(concerns.len(), 1);
    assert_eq!(concerns[0].id.as_str(), "bp-uncontrolled");
    assert_eq!(concerns[0].urgency, ConcernUrgency::Now);
    assert_eq!(concerns[0].revision, 1, "concern raised exactly once");
    assert_eq!(concerns[0].standing, ConcernStanding::Open);
}

/// AC1 (Background job): Restart mid-run in practice-owned background jobs
/// (`PatientReview` raising a concern + refreshing sources, and
/// `InitialUnderstanding` saving patient understanding), resuming in a new
/// attempt with one stored `PracticeJobResult` and zero repeated effects.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn ac1_practice_job_mid_run_restart_resumes_in_new_attempt_with_one_result_and_no_repeated_effects()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (base_store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();
    let env = setup_fixture(&base_store, &auth, "ac1-job").await;

    let ehr_id = SourceId::new("ehr").unwrap();
    let checker = FixedSourceCheck::default();
    checker.set_source_outcome(
        &env.practice,
        &env.workspace,
        ehr_id.clone(),
        SourceCheckOutcome::Updated {
            source_label: "Clinic EHR".into(),
            patient: SourcePatient {
                source_patient_id: "sp-ac1-job".into(),
                display_name: "Elena Navarro".into(),
                birth_date: Some("1968-04-14".into()),
                sex_or_gender: Some("female".into()),
                medical_record_numbers: Vec::new(),
            },
            pulled_at: 1_760_700_000,
            records: vec![chart_record(
                &env.workspace,
                "ehr",
                "lab-k-critical",
                "rev-2",
                "laboratory",
                "Repeat serum potassium",
                "2026-11-11",
                "Serum potassium 5.9 mEq/L (critical high).",
            )],
        },
    );
    let store = base_store.with_source_refresh(Arc::new(checker.clone()));

    let review_assignment = Assignment::new(
        PracticeJobKind::PatientReview,
        "cmd-review-ac1",
        "Refresh sources, inspect new results, and raise an urgent concern if hyperkalemia is present.",
        &["patient-record-retrieval"],
        &skills,
    )
    .expect("review assignment");
    let built_recipe = local_product::runtime::jobs::recipe(&roles, &review_assignment, &skills);
    assert!(
        built_recipe
            .tools
            .iter()
            .any(|tool| tool["name"] == CONCERN_TOOL),
        "practice background job recipe must offer patient_concern"
    );

    let job_scope = PracticeScope {
        practice: env.practice.clone(),
        workspace: env.workspace.clone(),
        job_id: "job-review-ac1".into(),
    };
    let accepted = store
        .accept_practice_job(
            &job_scope,
            &review_assignment,
            &roles,
            &skills,
            auth.as_ref(),
        )
        .await
        .expect("accept practice job");
    assert_eq!(accepted, Accepted::New);
    let initial_job = store.practice_job(&job_scope).await.unwrap().unwrap();
    let initial_run_id = initial_job.current_run().expect("open run").id.to_owned();

    // --- Attempt 1: refreshes sources (`ToolReturned` saved), reads `lab-k-critical`
    // (`ToolReturned` saved), then raises `patient_concern` -> commits in Postgres,
    // and crashes right BEFORE `ToolReturned` for `call-job-concern` is saved! ---
    let claim_1 = store
        .claim_practice_job(&job_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .expect("claim job attempt 1");
    let work_1 = build_practice_job_work_for_fault_host(&store, &claim_1).await;
    let cancel_1 = Cancellation::default();
    let mut fault_job_1 = FaultInjectingProduct::new(
        PracticeJobHost {
            store: store.clone(),
            claim: claim_1.clone(),
            authority: auth.as_ref(),
        },
        PersistenceFault::CrashBeforeToolReturned {
            target_call_id: "call-job-concern".into(),
            cancel: cancel_1.clone(),
            tripped: false,
        },
    );

    struct JobAttempt1Provider {
        round: usize,
    }
    impl Provider for JobAttempt1Provider {
        async fn exchange(
            &mut self,
            _request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            self.round += 1;
            match self.round {
                1 => Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "call-job-refresh".into(),
                        name: REFRESH_TOOL.into(),
                        arguments: json!({}).to_string(),
                    })],
                    receipt: json!({ "job_attempt": 1, "round": 1 }),
                    usage: Default::default(),
                }),
                2 => Ok(Turn {
                    items: vec![
                        Item::Call(Invocation {
                            id: "call-job-read".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [{ "candidate_id": "lab-k-critical", "revision": "rev-2" }],
                                "max_result_bytes": 16_384,
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "call-job-concern".into(),
                            name: CONCERN_TOOL.into(),
                            arguments: json!({
                                "operation": "raise",
                                "concern_id": "hyperkalemia-5-9",
                                "urgency": "now",
                                "reason": "Critical potassium 5.9 mEq/L on 2026-11-11.",
                                "records": [{ "candidate_id": "lab-k-critical", "revision": "rev-2" }],
                            })
                            .to_string(),
                        }),
                    ],
                    receipt: json!({ "job_attempt": 1, "round": 2 }),
                    usage: Default::default(),
                }),
                other => panic!("unexpected round {other}"),
            }
        }
    }

    let err_1 = work_1
        .run(
            &mut JobAttempt1Provider { round: 0 },
            &mut fault_job_1,
            RunLimits::default(),
            &cancel_1,
        )
        .await
        .expect_err("job attempt 1 crashes before ToolReturned for call-job-concern");
    let reason_1 = AttemptFailure::from_error(&err_1, &cancel_1);
    assert_eq!(reason_1, AttemptFailure::Interrupted);
    store
        .settle_practice_job_attempt(&claim_1, reason_1)
        .await
        .expect("settle job attempt 1");

    // --- Attempt 2: `PracticeJobHost::saved_outcome` reconciles `call-job-concern`
    // read-only (`"outcome": "duplicate"`), reuses `call-job-refresh` and `call-job-read`,
    // and commits one PracticeJobResult! ---
    let claim_2 = store
        .claim_practice_job(&job_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .expect("claim job attempt 2");

    struct JobAttempt2Provider;
    impl Provider for JobAttempt2Provider {
        async fn exchange(
            &mut self,
            request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            let concern_out = materialized_output(request, "call-job-concern")
                .expect("reconciled call-job-concern output");
            assert!(
                concern_out.contains("\"outcome\":\"duplicate\""),
                "PracticeJobHost::saved_outcome must reconcile committed concern receipt: {concern_out}"
            );
            Ok(Turn {
                items: vec![Item::assistant(
                    "Raised an urgent hyperkalemia concern for potassium 5.9 mEq/L on 2026-11-11. Basis: Regional laboratory panel rev-2.",
                )],
                receipt: json!({ "job_attempt": 2 }),
                usage: Default::default(),
            })
        }
    }

    let mut host_2 = PracticeJobHost {
        store: store.clone(),
        claim: claim_2.clone(),
        authority: auth.as_ref(),
    };
    host_2
        .run(
            &mut JobAttempt2Provider,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .expect("job attempt 2 completes");

    let final_job = store.practice_job(&job_scope).await.unwrap().unwrap();
    assert_eq!(final_job.status, WorkStatus::Completed);
    assert_eq!(final_job.failures, 0);
    let final_run = final_job.current_run().expect("completed job run");
    assert_eq!(final_run.id, initial_run_id);
    assert_eq!(final_run.outcome, Some(RunOutcome::Completed));
    assert_eq!(final_job.attempts.len(), 2);
    assert_eq!(final_job.attempts[0].status, AttemptStatus::Interrupted);
    assert_eq!(final_job.attempts[1].status, AttemptStatus::Completed);
    assert_eq!(
        final_job.attempts[1].predecessor.as_deref(),
        Some(claim_1.attempt.as_str())
    );
    let saved_result = final_job.result.expect("single saved PracticeJobResult");
    assert_eq!(saved_result.completion, PracticeJobCompletion::Complete);
    assert_eq!(saved_result.attempt_id, claim_2.attempt);
    assert!(saved_result.output_text.contains("5.9 mEq/L"));

    assert_eq!(
        checker.call_count(&env.practice, &env.workspace, &ehr_id),
        1
    );
    let concerns = store
        .workspace_concerns(
            &WorkspaceAccess {
                practice: &env.practice,
                physician: &env.identity.physician_id,
                workspace: &env.workspace,
                session_hash: &env.session_hash,
            },
            &mut PostgresRuntimeAuthority,
        )
        .await
        .expect("read workspace concerns");
    assert_eq!(concerns.len(), 1);
    assert_eq!(concerns[0].id.as_str(), "hyperkalemia-5-9");
    assert_eq!(
        concerns[0].revision, 1,
        "concern raised once across restart"
    );
    assert_eq!(concerns[0].source_work.job, "patient_review");
}

// ============================================================================
// AC2 — Paid model output reuse, effects saved before reply, run saved before ack
// ============================================================================

/// AC2 (Model output reuse):
/// 1. Transient storage failure (`RetryablePersistence`) right after a model
///    call retries persistence in-memory without calling the provider again.
/// 2. Cross-attempt storage failure after `ProviderReturned` is committed
///    (failing `Candidate` persistence on chat, and failing `finish` on a
///    practice job) reuses the saved `Turn` on the successor attempt with zero
///    additional provider calls (`provider_calls == 1` across both attempts).
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn ac2_storage_failure_after_model_call_reuses_paid_output_in_run_and_across_attempts() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();
    let env = setup_fixture(&store, &auth, "ac2-reuse").await;

    let chat_recipe = store
        .chat_recipe_with_understanding(
            &roles.chat,
            local_product::physician_chat::INSTRUCTIONS,
            &skills,
            &env.practice,
            &env.workspace,
        )
        .await
        .unwrap();

    // --- Part A: In-run transient storage failure on ProviderReturned ---
    store
        .accept_authorized(
            &env.scope,
            &Input {
                key: "msg-ac2-inrun".into(),
                text: "Summarize the hypertension visit note.".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            env.session_hash.clone(),
            &skills,
            &chat_recipe,
        )
        .await
        .unwrap();

    let claim_inrun = store.claim(&env.scope, 60).await.unwrap().unwrap();
    let starting_ctx = store.patient_starting_context(&env.scope).await.unwrap();
    let work_inrun = build_chat_work_for_fault_host(&store, &claim_inrun, &starting_ctx).await;

    let inrun_provider_calls = Arc::new(AtomicUsize::new(0));
    struct CountingAnswerProvider {
        calls: Arc<AtomicUsize>,
        answer: &'static str,
    }
    impl Provider for CountingAnswerProvider {
        async fn exchange(
            &mut self,
            _request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(Turn {
                items: vec![Item::assistant(self.answer)],
                receipt: json!({ "paid": true }),
                usage: harness::engine::recipe::Usage {
                    input_tokens: Some(420),
                    output_tokens: Some(55),
                    cached_input_tokens: Some(100),
                },
            })
        }
    }

    let mut fault_inrun = FaultInjectingProduct::new(
        PhysicianHost {
            store: store.clone(),
            claim: claim_inrun,
            authority: PostgresRuntimeAuthority,
        },
        PersistenceFault::RetryableOnProviderReturned { remaining: 2 },
    );
    work_inrun
        .run(
            &mut CountingAnswerProvider {
                calls: inrun_provider_calls.clone(),
                answer: "Blood pressure was 148/92 mmHg on 2026-10-15.\n\nBasis and limitations: Clinic EHR note rev-1.",
            },
            &mut fault_inrun,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .expect("in-run retryable storage failures recover without extra model calls");
    assert_eq!(
        inrun_provider_calls.load(Ordering::SeqCst),
        1,
        "provider called once despite two failed ProviderReturned writes"
    );
    assert_eq!(
        fault_inrun.provider_returned_saves, 3,
        "persist(ProviderReturned) attempted 3 times (2 transient timeouts + 1 commit)"
    );
    assert_eq!(
        store
            .conversation(&env.scope)
            .await
            .unwrap()
            .responses()
            .count(),
        1
    );

    // --- Part B: Cross-attempt storage failure after ProviderReturned committed (Chat) ---
    let mut cross_scope = env.scope.clone();
    cross_scope.chat = format!("{}-cross", env.scope.chat);
    store
        .accept_authorized(
            &cross_scope,
            &Input {
                key: "msg-ac2-cross".into(),
                text: "Report the latest potassium level.".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            env.session_hash.clone(),
            &skills,
            &chat_recipe,
        )
        .await
        .unwrap();

    let claim_cross_1 = store.claim(&cross_scope, 60).await.unwrap().unwrap();
    let cross_ctx = store.patient_starting_context(&cross_scope).await.unwrap();
    let work_cross_1 =
        build_chat_work_for_fault_host(&store, &cross_claim(&claim_cross_1), &cross_ctx).await;
    let cross_provider_calls = Arc::new(AtomicUsize::new(0));
    let mut fault_cross_1 = FaultInjectingProduct::new(
        PhysicianHost {
            store: store.clone(),
            claim: claim_cross_1.clone(),
            authority: PostgresRuntimeAuthority,
        },
        PersistenceFault::FailCandidatePersist,
    );
    let err_cross_1 = work_cross_1
        .run(
            &mut CountingAnswerProvider {
                calls: cross_provider_calls.clone(),
                answer: "Serum potassium was 4.3 mEq/L on 2026-10-15.\n\nBasis and limitations: Regional Labs panel rev-1.",
            },
            &mut fault_cross_1,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .expect_err("Candidate storage failure ends attempt 1");
    assert_eq!(cross_provider_calls.load(Ordering::SeqCst), 1);
    store
        .settle_chat_attempt(
            &claim_cross_1,
            AttemptFailure::from_error(&err_cross_1, &Cancellation::default()),
        )
        .await
        .unwrap();
    assert_eq!(
        store
            .conversation(&cross_scope)
            .await
            .unwrap()
            .responses()
            .count(),
        0,
        "no reply published when Candidate persistence failed"
    );

    // Attempt 2 must reuse the durable ProviderReturned turn with ZERO provider calls!
    let claim_cross_2 = store.claim(&cross_scope, 60).await.unwrap().unwrap();
    struct NeverCalledProvider;
    impl Provider for NeverCalledProvider {
        async fn exchange(
            &mut self,
            _request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            panic!(
                "Provider::exchange must not be called when paid ProviderReturned is already saved"
            );
        }
    }
    let mut host_cross_2 = PhysicianHost {
        store: store.clone(),
        claim: claim_cross_2,
        authority: PostgresRuntimeAuthority,
    };
    host_cross_2
        .run(
            &mut NeverCalledProvider,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .expect("attempt 2 reuses saved ProviderReturned turn without calling provider");
    assert_eq!(cross_provider_calls.load(Ordering::SeqCst), 1);
    let cross_state = store.conversation(&cross_scope).await.unwrap();
    assert_eq!(cross_state.status, WorkStatus::Completed);
    let cross_replies: Vec<_> = cross_state.responses().collect();
    assert_eq!(cross_replies.len(), 1);
    assert!(cross_replies[0].text.contains("4.3 mEq/L"));

    // --- Part C: Cross-attempt storage failure on PracticeJobHost finish ---
    let job_scope = PracticeScope {
        practice: env.practice.clone(),
        workspace: env.workspace.clone(),
        job_id: "job-ac2-reuse".into(),
    };
    let assignment = Assignment::new(
        PracticeJobKind::VisitPreparation,
        "cmd-job-ac2",
        "Prepare concise pre-visit summary.",
        &["patient-record-retrieval"],
        &skills,
    )
    .unwrap();
    store
        .accept_practice_job(&job_scope, &assignment, &roles, &skills, auth.as_ref())
        .await
        .unwrap();
    let job_claim_1 = store
        .claim_practice_job(&job_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    let job_work_1 = build_practice_job_work_for_fault_host(&store, &job_claim_1).await;
    let job_provider_calls = Arc::new(AtomicUsize::new(0));
    let mut fault_job_1 = FaultInjectingProduct::new(
        PracticeJobHost {
            store: store.clone(),
            claim: job_claim_1.clone(),
            authority: auth.as_ref(),
        },
        PersistenceFault::FailFinish,
    );
    let job_err_1 = job_work_1
        .run(
            &mut CountingAnswerProvider {
                calls: job_provider_calls.clone(),
                answer: "Pre-visit summary: blood pressure 148/92 mmHg, potassium 4.3 mEq/L. Basis: 2026-10-15 chart.",
            },
            &mut fault_job_1,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .expect_err("finish failure ends practice job attempt 1");
    assert_eq!(job_provider_calls.load(Ordering::SeqCst), 1);
    store
        .settle_practice_job_attempt(
            &job_claim_1,
            AttemptFailure::from_error(&job_err_1, &Cancellation::default()),
        )
        .await
        .unwrap();

    let job_claim_2 = store
        .claim_practice_job(&job_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    let mut job_host_2 = PracticeJobHost {
        store: store.clone(),
        claim: job_claim_2,
        authority: auth.as_ref(),
    };
    job_host_2
        .run(
            &mut NeverCalledProvider,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .expect("practice job attempt 2 reuses saved ProviderReturned turn");
    assert_eq!(job_provider_calls.load(Ordering::SeqCst), 1);
    let saved_job = store.practice_job(&job_scope).await.unwrap().unwrap();
    assert_eq!(saved_job.status, WorkStatus::Completed);
    assert!(
        saved_job
            .result
            .unwrap()
            .output_text
            .contains("148/92 mmHg")
    );
}

fn cross_claim(claim: &Claim) -> Claim {
    claim.clone()
}

/// AC2 (Effects before reply & run saved before acknowledgment):
/// 1. Tool side effects (`patient_concern`, `patient_memory`, and
///    `save_patient_understanding`) commit durably before any reply/result is
///    published: while tools run and when `finish` fails, effects are already
///    saved and zero replies exist; recovery publishes the reply without
///    repeating any tool effect.
/// 2. `accept_authorized` and `accept_practice_job` commit the `Pending` run,
///    `RunStarted` history, frozen recipe/skills, and command receipt before
///    acknowledging; a failed/closed store write returns `Err`, acknowledges
///    nothing, and leaves zero orphan receipts so retry succeeds cleanly.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn ac2_effects_commit_before_reply_and_run_commits_before_acknowledgment() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();
    let env = setup_fixture(&store, &auth, "ac2-order").await;

    let chat_recipe = store
        .chat_recipe_with_understanding(
            &roles.chat,
            local_product::physician_chat::INSTRUCTIONS,
            &skills,
            &env.practice,
            &env.workspace,
        )
        .await
        .unwrap();

    // --- Part 1: Run saved before acknowledgment (and failed save acknowledges nothing) ---
    let closed_store = Store::connect(&url).await.unwrap();
    closed_store
        .close_for_test_check(&url, &env, &skills, &roles, &chat_recipe, auth.as_ref())
        .await;

    // Immediate readback from an independent connection right as `accept_authorized` returns:
    let accepted = store
        .accept_authorized(
            &env.scope,
            &Input {
                key: "msg-ac2-effects".into(),
                text: "Raise concern and save memory correction before replying.".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            env.session_hash.clone(),
            &skills,
            &chat_recipe,
        )
        .await
        .expect("accept chat run");
    assert_eq!(accepted, Accepted::New);

    let observer_store = Store::connect(&url).await.expect("fresh observer store");
    let immediately_saved = observer_store
        .conversation(&env.scope)
        .await
        .expect("read saved run immediately after acknowledgment");
    assert_eq!(immediately_saved.status, WorkStatus::Pending);
    assert!(
        matches!(immediately_saved.run().map(|r| r.outcome), Some(None)),
        "open RunStarted must be committed before accept_authorized returns"
    );
    assert_eq!(immediately_saved.responses().count(), 0);

    // --- Part 2: Effects saved before reply (chat + InitialUnderstanding job) ---
    let claim_1 = store.claim(&env.scope, 60).await.unwrap().unwrap();
    let starting_ctx = store.patient_starting_context(&env.scope).await.unwrap();
    let work_1 = build_chat_work_for_fault_host(&store, &claim_1, &starting_ctx).await;

    struct EffectThenReplyProvider {
        observer_store: Store,
        scope: Scope,
        round: usize,
    }
    impl Provider for EffectThenReplyProvider {
        async fn exchange(
            &mut self,
            _request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            self.round += 1;
            match self.round {
                1 => Ok(Turn {
                    items: vec![
                        Item::Call(Invocation {
                            id: "eff-concern".into(),
                            name: CONCERN_TOOL.into(),
                            arguments: json!({
                                "operation": "raise",
                                "concern_id": "effect-before-reply",
                                "urgency": "today",
                                "reason": "Follow up blood pressure trend.",
                                "records": [{ "candidate_id": "visit-note-1", "revision": "rev-1" }],
                            })
                            .to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "eff-memory".into(),
                            name: PATIENT_MEMORY_TOOL.into(),
                            arguments: json!({
                                "operation": "record_correction",
                                "memory_id": "pref-pharmacy",
                                "topic": "Preferred pharmacy",
                                "statement": "Uses Mission St Pharmacy.",
                            })
                            .to_string(),
                        }),
                    ],
                    receipt: json!({ "round": 1 }),
                    usage: Default::default(),
                }),
                2 => {
                    // Right after Round 1's effects committed (before Round 2's reply even exists):
                    let live = self
                        .observer_store
                        .conversation(&self.scope)
                        .await
                        .expect("inspect live conversation");
                    assert_eq!(
                        live.responses().count(),
                        0,
                        "no reply may exist while tool effects are already saved"
                    );
                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Recorded concern and pharmacy note.\n\nBasis and limitations: Clinic EHR note rev-1 and physician input.",
                        )],
                        receipt: json!({ "round": 2 }),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected round {other}"),
            }
        }
    }

    let mut fault_finish = FaultInjectingProduct::new(
        PhysicianHost {
            store: store.clone(),
            claim: claim_1.clone(),
            authority: PostgresRuntimeAuthority,
        },
        PersistenceFault::FailFinish,
    );
    let err_1 = work_1
        .run(
            &mut EffectThenReplyProvider {
                observer_store: observer_store.clone(),
                scope: env.scope.clone(),
                round: 0,
            },
            &mut fault_finish,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .expect_err("finish fails after effects committed");
    store
        .settle_chat_attempt(
            &claim_1,
            AttemptFailure::from_error(&err_1, &Cancellation::default()),
        )
        .await
        .unwrap();

    // Effects ARE durably saved in Postgres while reply count is still ZERO:
    assert_eq!(
        observer_store
            .conversation(&env.scope)
            .await
            .unwrap()
            .responses()
            .count(),
        0,
        "reply must remain absent when commit_authorized failed"
    );
    let concerns_before_reply = observer_store
        .workspace_concerns(
            &WorkspaceAccess {
                practice: &env.practice,
                physician: &env.identity.physician_id,
                workspace: &env.workspace,
                session_hash: &env.session_hash,
            },
            &mut PostgresRuntimeAuthority,
        )
        .await
        .unwrap();
    assert_eq!(concerns_before_reply.len(), 1);
    assert_eq!(concerns_before_reply[0].revision, 1);

    // Recovery attempt publishes the single reply without re-executing tools or provider:
    let claim_2 = store.claim(&env.scope, 60).await.unwrap().unwrap();
    struct NoOpProvider;
    impl Provider for NoOpProvider {
        async fn exchange(
            &mut self,
            _request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            panic!("recovery must reuse saved terminal turn without calling provider");
        }
    }
    let mut host_2 = PhysicianHost {
        store: store.clone(),
        claim: claim_2,
        authority: PostgresRuntimeAuthority,
    };
    host_2
        .run(
            &mut NoOpProvider,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .expect("recovery publishes reply");

    let after_recovery = observer_store.conversation(&env.scope).await.unwrap();
    assert_eq!(after_recovery.responses().count(), 1);
    let concerns_after_reply = observer_store
        .workspace_concerns(
            &WorkspaceAccess {
                practice: &env.practice,
                physician: &env.identity.physician_id,
                workspace: &env.workspace,
                session_hash: &env.session_hash,
            },
            &mut PostgresRuntimeAuthority,
        )
        .await
        .unwrap();
    assert_eq!(concerns_after_reply.len(), 1);
    assert_eq!(concerns_after_reply[0].revision, 1);
}

trait ClosedStoreAcceptanceCheck {
    async fn close_for_test_check(
        self,
        healthy_url: &str,
        env: &FixtureEnv,
        skills: &PhysicianSkills,
        roles: &ModelRoles,
        chat_recipe: &harness::engine::recipe::RequestRecipe,
        auth: &PostgresAuthStore,
    );
}

impl ClosedStoreAcceptanceCheck for Store {
    async fn close_for_test_check(
        self,
        healthy_url: &str,
        env: &FixtureEnv,
        skills: &PhysicianSkills,
        roles: &ModelRoles,
        chat_recipe: &harness::engine::recipe::RequestRecipe,
        auth: &PostgresAuthStore,
    ) {
        drop(self);

        // Route a real `Store` through an in-process TCP proxy to Postgres, then
        // sever the proxy before calling `accept_authorized` and `accept_practice_job`
        // so both hit an injected storage failure before commit.
        let mut parsed = reqwest::Url::parse(healthy_url).expect("parse healthy postgres url");
        let upstream_host = parsed.host_str().unwrap_or("127.0.0.1").to_owned();
        let upstream_port = parsed.port().unwrap_or(5432);
        let upstream_addr = format!("{upstream_host}:{upstream_port}");

        let proxy_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let proxy_port = proxy_listener.local_addr().unwrap().port();
        parsed.set_host(Some("127.0.0.1")).unwrap();
        parsed.set_port(Some(proxy_port)).unwrap();
        let proxy_url = parsed.to_string();

        let allow_traffic = Arc::new(AtomicBool::new(true));
        let cut_notify = Arc::new(Notify::new());
        let proxy_allow = allow_traffic.clone();
        let proxy_cut = cut_notify.clone();
        let proxy_task = tokio::spawn(async move {
            while let Ok((mut client, _)) = proxy_listener.accept().await {
                if !proxy_allow.load(Ordering::SeqCst) {
                    drop(client);
                    continue;
                }
                let addr = upstream_addr.clone();
                let cut = proxy_cut.clone();
                tokio::spawn(async move {
                    let Ok(mut upstream) = tokio::net::TcpStream::connect(&addr).await else {
                        return;
                    };
                    tokio::select! {
                        _ = tokio::io::copy_bidirectional(&mut client, &mut upstream) => {}
                        _ = cut.notified() => {}
                    }
                });
            }
        });

        let proxy_store = Store::connect(&proxy_url)
            .await
            .expect("connect store through proxy while traffic is open");
        allow_traffic.store(false, Ordering::SeqCst);
        cut_notify.notify_waiters();

        let failed_chat_accept = proxy_store
            .accept_authorized(
                &env.scope,
                &Input {
                    key: "msg-ac2-effects".into(),
                    text: "Raise concern and save memory correction before replying.".into(),
                    timezone: "UTC".into(),
                    scenario: None,
                },
                env.session_hash.clone(),
                skills,
                chat_recipe,
            )
            .await;
        assert!(
            failed_chat_accept.is_err(),
            "storage failure before commit must fail accept_authorized without acknowledging"
        );

        let job_scope = PracticeScope {
            practice: env.practice.clone(),
            workspace: env.workspace.clone(),
            job_id: "job-ac2-ack-check".into(),
        };
        let assignment = Assignment::new(
            PracticeJobKind::PatientReview,
            "cmd-ac2-ack-check",
            "Verify run is saved before acknowledgment.",
            &["patient-record-retrieval"],
            skills,
        )
        .unwrap();
        let failed_job_accept = proxy_store
            .accept_practice_job(&job_scope, &assignment, roles, skills, auth)
            .await;
        assert!(
            failed_job_accept.is_err(),
            "storage failure before commit must fail accept_practice_job without acknowledging"
        );

        proxy_task.abort();
        drop(proxy_store);

        let verify_store = Store::connect(healthy_url).await.unwrap();
        assert!(
            verify_store.conversation(&env.scope).await.is_err(),
            "failed accept_authorized must leave zero uncommitted run state"
        );
        assert!(
            verify_store
                .practice_job(&job_scope)
                .await
                .unwrap()
                .is_none(),
            "failed accept_practice_job must leave zero uncommitted job state"
        );

        let job_ack = verify_store
            .accept_practice_job(&job_scope, &assignment, roles, skills, auth)
            .await
            .expect("retry practice job acceptance on healthy store");
        assert_eq!(
            job_ack,
            Accepted::New,
            "failed accept_practice_job must leave zero orphan receipt"
        );
        let reopened = Store::connect(healthy_url).await.unwrap();
        let saved_job = reopened
            .practice_job(&job_scope)
            .await
            .unwrap()
            .expect("practice job immediately visible after accept_practice_job returns");
        assert_eq!(saved_job.status, WorkStatus::Pending);
        assert!(saved_job.current_run().is_some_and(|r| r.outcome.is_none()));
    }
}

// ============================================================================
// AC3 — Steering, concurrent work, context change, Stop vs interruption,
//       and stale output blocked by newer direction, lost access, or edit
// ============================================================================

/// AC3 (Steering + newer physician direction blocks stale output):
/// While Attempt 1 streams live preview text and has 2 non-read-only tool calls
/// (`call-started` already executing, `call-unstarted` queued behind it), the
/// physician submits a newer direction:
/// - `commit_authorized` on Attempt 1 is refused (`Superseded`),
/// - `call-started` drains to completion and its result is retained,
/// - `call-unstarted` never executes and is marked `"outcome": "not_run"`,
/// - unpublished live streamed draft text is discarded, and
/// - Attempt 2 receives the new direction once and publishes one coherent reply.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn ac3_steering_drains_started_tool_skips_unstarted_discards_draft_and_blocks_stale_output() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();
    let env = setup_fixture(&store, &auth, "ac3-steer").await;

    let tool_started_signal = Arc::new(Notify::new());
    let allow_tool_finish = Arc::new(Notify::new());
    let started_executions = Arc::new(AtomicUsize::new(0));
    let unstarted_executions = Arc::new(AtomicUsize::new(0));

    struct CoordinatedMemoryOp {
        started_signal: Arc<Notify>,
        allow_finish: Arc<Notify>,
        started_count: Arc<AtomicUsize>,
        unstarted_count: Arc<AtomicUsize>,
        inner: local_product::runtime::chat_operations::PostgresPatientMemoryOperation,
    }

    #[async_trait]
    impl local_product::runtime::chat_operations::PatientMemoryOperation for CoordinatedMemoryOp {
        async fn list_memories(
            &self,
            store: &Store,
            practice: &str,
            workspace: &str,
            selection: MemorySelection,
        ) -> Result<
            Vec<local_product::runtime::chat_operations::PatientMemoryEntry>,
            local_product::runtime::chat_operations::PatientMemoryError,
        > {
            self.inner
                .list_memories(store, practice, workspace, selection)
                .await
        }

        async fn record_correction(
            &self,
            store: &Store,
            command_key: &str,
            input: &local_product::runtime::chat_operations::RecordMemoryCorrection,
        ) -> Result<
            local_product::runtime::chat_operations::MemoryMutation,
            local_product::runtime::chat_operations::PatientMemoryError,
        > {
            if input.id.as_str() == "mem-started" {
                self.started_count.fetch_add(1, Ordering::SeqCst);
                self.started_signal.notify_one();
                self.allow_finish.notified().await;
            } else {
                self.unstarted_count.fetch_add(1, Ordering::SeqCst);
            }
            self.inner
                .record_correction(store, command_key, input)
                .await
        }

        async fn retire_memory(
            &self,
            store: &Store,
            command_key: &str,
            input: &local_product::runtime::chat_operations::RetirePatientMemory,
        ) -> Result<
            local_product::runtime::chat_operations::MemoryMutation,
            local_product::runtime::chat_operations::PatientMemoryError,
        > {
            self.inner.retire_memory(store, command_key, input).await
        }

        async fn saved_mutation(
            &self,
            store: &Store,
            practice: &str,
            workspace: &str,
            command_key: &str,
            id: &local_product::runtime::chat_operations::PatientMemoryId,
        ) -> Result<
            Option<local_product::runtime::chat_operations::PatientMemoryEntry>,
            local_product::runtime::chat_operations::PatientMemoryError,
        > {
            self.inner
                .saved_mutation(store, practice, workspace, command_key, id)
                .await
        }
    }

    let steered_store = store
        .clone()
        .with_memory_operation(Arc::new(CoordinatedMemoryOp {
            started_signal: tool_started_signal.clone(),
            allow_finish: allow_tool_finish.clone(),
            started_count: started_executions.clone(),
            unstarted_count: unstarted_executions.clone(),
            inner: local_product::runtime::chat_operations::PostgresPatientMemoryOperation,
        }));

    let chat_recipe = steered_store
        .chat_recipe_with_understanding(
            &roles.chat,
            local_product::physician_chat::INSTRUCTIONS,
            &skills,
            &env.practice,
            &env.workspace,
        )
        .await
        .unwrap();

    steered_store
        .accept_authorized(
            &env.scope,
            &Input {
                key: "steer-msg-1".into(),
                text: "Save two memory notes and summarize blood pressure.".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            env.session_hash.clone(),
            &skills,
            &chat_recipe,
        )
        .await
        .unwrap();

    let claim_1 = steered_store.claim(&env.scope, 60).await.unwrap().unwrap();

    struct SteeredAttempt1Provider;
    impl Provider for SteeredAttempt1Provider {
        async fn exchange(
            &mut self,
            _request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            let _ = observer
                .emit(StreamEvent::Text(TextDelta::from(
                    "Stale pre-steer live draft text that must never persist.",
                )))
                .await;
            Ok(Turn {
                items: vec![
                    Item::Call(Invocation {
                        id: "call-started".into(),
                        name: PATIENT_MEMORY_TOOL.into(),
                        arguments: json!({
                            "operation": "record_correction",
                            "memory_id": "mem-started",
                            "topic": "Home cuff",
                            "statement": "Patient uses validated Omron upper-arm cuff.",
                        })
                        .to_string(),
                    }),
                    Item::Call(Invocation {
                        id: "call-unstarted".into(),
                        name: PATIENT_MEMORY_TOOL.into(),
                        arguments: json!({
                            "operation": "record_correction",
                            "memory_id": "mem-unstarted",
                            "topic": "Unstarted note",
                            "statement": "Should not run before steering.",
                        })
                        .to_string(),
                    }),
                ],
                receipt: json!({ "attempt": 1 }),
                usage: Default::default(),
            })
        }
    }

    let host_store_1 = steered_store.clone();
    let claim_1_clone = claim_1.clone();
    let attempt_1_task = tokio::spawn(async move {
        let mut host = PhysicianHost {
            store: host_store_1,
            claim: claim_1_clone,
            authority: PostgresRuntimeAuthority,
        };
        host.run(
            &mut SteeredAttempt1Provider,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
    });

    // Wait until `call-started` is actively executing, then accept newer physician direction:
    tool_started_signal.notified().await;
    let steer_accepted = steered_store
        .accept_authorized(
            &env.scope,
            &Input {
                key: "steer-msg-2".into(),
                text: "Actually skip the second memory note and focus on potassium too.".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            env.session_hash.clone(),
            &skills,
            &chat_recipe,
        )
        .await
        .expect("accept steering message");
    assert_eq!(steer_accepted, Accepted::New);

    // Wait > CURRENCY_INTERVAL (50ms) so `Run::tools` observes `Superseded` and closes admission,
    // then let `call-started` finish cleanly within its deadline!
    tokio::time::sleep(Duration::from_millis(120)).await;
    allow_tool_finish.notify_one();

    let err_1 = attempt_1_task
        .await
        .expect("join attempt 1")
        .expect_err("steered attempt 1 must exit with Superseded");
    assert_eq!(
        AttemptFailure::from_error(&err_1, &Cancellation::default()),
        AttemptFailure::Interrupted
    );

    // Stale publication from Attempt 1 is strictly refused!
    let stale_commit = steered_store
        .commit_authorized(
            &claim_1,
            &Turn {
                items: vec![Item::assistant("Stale answer")],
                receipt: json!({}),
                usage: Default::default(),
            },
            &mut PostgresRuntimeAuthority,
        )
        .await;
    assert!(
        stale_commit.is_err(),
        "newer physician direction must block stale commit_authorized"
    );

    steered_store
        .settle_chat_attempt(&claim_1, AttemptFailure::Interrupted)
        .await
        .unwrap();

    assert_eq!(started_executions.load(Ordering::SeqCst), 1);
    assert_eq!(
        unstarted_executions.load(Ordering::SeqCst),
        0,
        "call-unstarted must not execute after steering"
    );
    let mid_state = steered_store.conversation(&env.scope).await.unwrap();
    assert_eq!(
        mid_state.responses().count(),
        0,
        "streamed live draft text must be discarded on steer"
    );
    assert!(
        !mid_state
            .messages
            .iter()
            .any(|m| m.text.contains("Stale pre-steer")),
        "unpublished draft text must never appear in conversation messages"
    );

    // --- Attempt 2: receives retained `call-started`, `"outcome": "not_run"` for
    // `call-unstarted`, and `steer-msg-2` exactly once, then answers both inputs! ---
    let claim_2 = steered_store.claim(&env.scope, 60).await.unwrap().unwrap();
    struct SteeredAttempt2Provider;
    impl Provider for SteeredAttempt2Provider {
        async fn exchange(
            &mut self,
            request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            let started_out =
                materialized_output(request, "call-started").expect("retained call-started result");
            assert!(
                started_out.contains("Omron upper-arm cuff"),
                "completed started call must retain its result: {started_out}"
            );
            let unstarted_out = materialized_output(request, "call-unstarted")
                .expect("not_run marker for call-unstarted");
            assert!(
                unstarted_out.contains("\"outcome\":\"not_run\""),
                "unstarted call must be marked not_run: {unstarted_out}"
            );
            let steer_count = request
                .items
                .iter()
                .filter(|item| {
                    matches!(
                        item,
                        Item::Message { role, text }
                            if role == "user" && text.contains("Actually skip the second memory note")
                    )
                })
                .count();
            assert_eq!(steer_count, 1, "steering message must appear exactly once");
            assert!(
                !request.items.iter().any(|item| {
                    matches!(item, Item::Message { text, .. } if text.contains("Stale pre-steer"))
                }),
                "discarded draft text must not be re-sent on steering continuation"
            );

            Ok(Turn {
                items: vec![Item::assistant(
                    "Saved the Omron cuff note, skipped the second note as requested, and reviewed blood pressure (148/92 mmHg) alongside potassium (4.3 mEq/L).\n\nBasis and limitations: 2026-10-15 chart records and physician steering.",
                )],
                receipt: json!({ "attempt": 2 }),
                usage: Default::default(),
            })
        }
    }

    let mut host_2 = PhysicianHost {
        store: steered_store.clone(),
        claim: claim_2,
        authority: PostgresRuntimeAuthority,
    };
    host_2
        .run(
            &mut SteeredAttempt2Provider,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .expect("steered attempt 2 finishes");

    assert_eq!(
        started_executions.load(Ordering::SeqCst),
        1,
        "finished tool work before steer is never repeated"
    );
    assert_eq!(unstarted_executions.load(Ordering::SeqCst), 0);
    let final_state = steered_store.conversation(&env.scope).await.unwrap();
    assert_eq!(final_state.status, WorkStatus::Completed);
    let replies: Vec<_> = final_state.responses().collect();
    assert_eq!(replies.len(), 1);
    assert!(replies[0].text.contains("skipped the second note"));
}

/// AC3 (Work running at the same time):
/// Concurrent physician conversations and a practice background job on the same
/// patient workspace execute simultaneously across lanes without blocking,
/// stealing claims, or cross-contaminating events or outputs.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn ac3_concurrent_chat_and_background_job_run_simultaneously_without_interference() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();
    let env = setup_fixture(&store, &auth, "ac3-concurrent").await;

    let chat_recipe = store
        .chat_recipe_with_understanding(
            &roles.chat,
            local_product::physician_chat::INSTRUCTIONS,
            &skills,
            &env.practice,
            &env.workspace,
        )
        .await
        .unwrap();

    let chat_a = env.scope.clone();
    let mut chat_b = env.scope.clone();
    chat_b.chat = format!("{}-second", env.scope.chat);

    store
        .accept_authorized(
            &chat_a,
            &Input {
                key: "conc-chat-a".into(),
                text: "Summarize blood pressure in conversation A.".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            env.session_hash.clone(),
            &skills,
            &chat_recipe,
        )
        .await
        .unwrap();
    store
        .accept_authorized(
            &chat_b,
            &Input {
                key: "conc-chat-b".into(),
                text: "Summarize potassium in conversation B.".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            env.session_hash.clone(),
            &skills,
            &chat_recipe,
        )
        .await
        .unwrap();

    let job_scope = PracticeScope {
        practice: env.practice.clone(),
        workspace: env.workspace.clone(),
        job_id: "conc-bg-review".into(),
    };
    let job_assignment = Assignment::new(
        PracticeJobKind::PatientReview,
        "cmd-conc-bg",
        "Run concurrent background chart review.",
        &["patient-record-retrieval"],
        &skills,
    )
    .unwrap();
    store
        .accept_practice_job(&job_scope, &job_assignment, &roles, &skills, auth.as_ref())
        .await
        .unwrap();

    let claim_chat_a = store.claim(&chat_a, 60).await.unwrap().unwrap();
    let claim_chat_b = store.claim(&chat_b, 60).await.unwrap().unwrap();
    let claim_job = store
        .claim_practice_job(&job_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();

    let barrier = Arc::new(Barrier::new(3));
    struct BarrierProvider {
        barrier: Arc<Barrier>,
        expected_model: &'static str,
        reply: &'static str,
        round: usize,
    }
    impl Provider for BarrierProvider {
        async fn exchange(
            &mut self,
            request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            assert_eq!(request.model, self.expected_model);
            self.round += 1;
            if self.round == 1 {
                self.barrier.wait().await;
                Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "conc-search".into(),
                        name: SEARCH_TOOL.into(),
                        arguments: json!({ "query": "potassium blood pressure" }).to_string(),
                    })],
                    receipt: json!({ "round": 1 }),
                    usage: Default::default(),
                })
            } else {
                Ok(Turn {
                    items: vec![Item::assistant(self.reply)],
                    receipt: json!({ "round": 2 }),
                    usage: Default::default(),
                })
            }
        }
    }

    let mut host_a = PhysicianHost {
        store: store.clone(),
        claim: claim_chat_a,
        authority: PostgresRuntimeAuthority,
    };
    let mut host_b = PhysicianHost {
        store: store.clone(),
        claim: claim_chat_b,
        authority: PostgresRuntimeAuthority,
    };
    let mut host_job = PracticeJobHost {
        store: store.clone(),
        claim: claim_job,
        authority: auth.as_ref(),
    };

    let mut prov_a = BarrierProvider {
        barrier: barrier.clone(),
        expected_model: "recovery-chat-lane",
        reply: "Conversation A answer: BP 148/92 mmHg.\n\nBasis and limitations: 2026-10-15 note.",
        round: 0,
    };
    let mut prov_b = BarrierProvider {
        barrier: barrier.clone(),
        expected_model: "recovery-chat-lane",
        reply: "Conversation B answer: Potassium 4.3 mEq/L.\n\nBasis and limitations: 2026-10-15 lab.",
        round: 0,
    };
    let mut prov_job = BarrierProvider {
        barrier: barrier.clone(),
        expected_model: "recovery-background-lane",
        reply: "Background review complete: BP 148/92 mmHg and K 4.3 mEq/L.",
        round: 0,
    };
    let cancel_all = Cancellation::default();
    let (res_a, res_b, res_job) = tokio::join!(
        host_a.run(&mut prov_a, RunLimits::default(), &cancel_all),
        host_b.run(&mut prov_b, RunLimits::default(), &cancel_all),
        host_job.run(&mut prov_job, RunLimits::default(), &cancel_all),
    );
    res_a.expect("concurrent chat A succeeds");
    res_b.expect("concurrent chat B succeeds");
    res_job.expect("concurrent background job succeeds");

    let state_a = store.conversation(&chat_a).await.unwrap();
    let state_b = store.conversation(&chat_b).await.unwrap();
    let state_job = store.practice_job(&job_scope).await.unwrap().unwrap();
    assert_eq!(
        state_a.responses().next().unwrap().text,
        "Conversation A answer: BP 148/92 mmHg.\n\nBasis and limitations: 2026-10-15 note."
    );
    assert_eq!(
        state_b.responses().next().unwrap().text,
        "Conversation B answer: Potassium 4.3 mEq/L.\n\nBasis and limitations: 2026-10-15 lab."
    );
    assert_eq!(
        state_job.result.unwrap().output_text,
        "Background review complete: BP 148/92 mmHg and K 4.3 mEq/L."
    );
}

/// AC3 (Context change, physician draft edit, and lost access block stale output):
/// 1. Physician editing a saved reply draft (`save_draft`) while a revision
///    attempt is in flight cancels the revision and blocks `commit_revision`,
///    preserving the physician's edited text.
/// 2. Chart source updates while a reply revision is running cause
///    `commit_revision` to return `RevisionCommit::SourcesMoved` rather than
///    publishing against stale sources; similarly, `save_patient_understanding`
///    refuses stale `target_source_version` when new records arrive mid-run.
/// 3. Revoking physician session / panel access mid-run blocks `PhysicianHost`,
///    and revoking practice patient access mid-run blocks `PracticeJobHost`.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn ac3_context_change_draft_edit_and_lost_access_block_stale_output() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();
    let env = setup_fixture(&store, &auth, "ac3-block").await;

    // --- 1. Reply draft edit blocks concurrent revision output ---
    let mut reply_scope = env.scope.clone();
    reply_scope.chat = format!("reply-edit-{}", Uuid::new_v4().simple());
    let opened = store
        .open_draft_authorized(
            &reply_scope,
            "Initial reply draft to Elena.",
            &env.session_hash,
            &mut PostgresRuntimeAuthority,
        )
        .await
        .unwrap()
        .expect("open draft");
    assert_eq!(opened.revision, 1);

    let rev_recipe = local_product::runtime::reply::recipe(&roles.background, &skills);
    let rev_input = RevisionInput {
        key: "rev-edit-race".into(),
        expected_revision: 1,
        instruction: "Make the tone warmer.".into(),
        timezone: "UTC".into(),
        scenario: None,
    };
    assert_eq!(
        store
            .accept_revision(
                &reply_scope,
                &rev_input,
                env.session_hash.clone(),
                &skills,
                &rev_recipe,
            )
            .await
            .unwrap(),
        RevisionAccepted::New
    );
    let rev_delivery = Delivery::reply_revision(reply_scope.clone(), rev_input.key.clone());
    let rev_claim = store
        .claim_revision(&rev_delivery, 60)
        .await
        .unwrap()
        .expect("claim revision");

    let finish_turn = Turn {
        items: vec![Item::Call(Invocation {
            id: "finish-stale".into(),
            name: FINISH_TOOL.into(),
            arguments: json!({
                "text": "Stale model revision that arrived after the physician edited the draft.",
                "review_basis": "2026-10-15 note",
            })
            .to_string(),
        })],
        receipt: json!({ "rev": 1 }),
        usage: Default::default(),
    };
    store
        .record_revision(
            &rev_claim,
            Event {
                attempt_id: rev_claim.attempt.clone(),
                ordinal: 1,
                sequence: 1,
                kind: EventKind::ProviderReturned(finish_turn.clone()),
            },
        )
        .await
        .unwrap();
    store
        .record_revision(
            &rev_claim,
            Event {
                attempt_id: rev_claim.attempt.clone(),
                ordinal: 1,
                sequence: 2,
                kind: EventKind::Candidate {
                    text: String::new(),
                },
            },
        )
        .await
        .unwrap();

    // Physician edits and saves the draft before `commit_revision` runs:
    let edited_rev = store
        .save_draft(
            &reply_scope,
            1,
            "Physician's manual edit: come in tomorrow morning for a repeat blood pressure check.",
            &env.session_hash,
            &mut PostgresRuntimeAuthority,
        )
        .await
        .expect("physician saves draft edit");
    assert_eq!(edited_rev, 2);

    let stale_rev_commit = store
        .commit_revision(&rev_claim, &finish_turn, &mut PostgresRuntimeAuthority)
        .await;
    assert!(
        stale_rev_commit.is_err(),
        "physician edit must block stale revision completion"
    );
    let saved_draft = store.draft(&reply_scope).await.unwrap().unwrap();
    assert_eq!(saved_draft.revision, 2);
    assert_eq!(saved_draft.status, DraftStatus::Cancelled);
    assert_eq!(
        saved_draft.text,
        "Physician's manual edit: come in tomorrow morning for a repeat blood pressure check."
    );

    // --- 2. Context / source change mid-run blocks stale completion (`SourcesMoved` + `StaleSourceRevision`) ---
    let mut source_move_scope = env.scope.clone();
    source_move_scope.chat = format!("reply-src-{}", Uuid::new_v4().simple());
    store
        .open_draft_authorized(
            &source_move_scope,
            "Draft before new lab arrived.",
            &env.session_hash,
            &mut PostgresRuntimeAuthority,
        )
        .await
        .unwrap()
        .unwrap();
    let src_rev_input = RevisionInput {
        key: "rev-src-move".into(),
        expected_revision: 1,
        instruction: "Confirm labs are normal.".into(),
        timezone: "UTC".into(),
        scenario: None,
    };
    store
        .accept_revision(
            &source_move_scope,
            &src_rev_input,
            env.session_hash.clone(),
            &skills,
            &rev_recipe,
        )
        .await
        .unwrap();
    let src_claim = store
        .claim_revision(
            &Delivery::reply_revision(source_move_scope.clone(), src_rev_input.key.clone()),
            60,
        )
        .await
        .unwrap()
        .unwrap();

    let src_finish_turn = Turn {
        items: vec![Item::Call(Invocation {
            id: "finish-before-new-lab".into(),
            name: FINISH_TOOL.into(),
            arguments: json!({
                "text": "Your labs from October look normal.",
                "review_basis": "2026-10-15 lab",
            })
            .to_string(),
        })],
        receipt: json!({ "rev": 1 }),
        usage: Default::default(),
    };
    store
        .record_revision(
            &src_claim,
            Event {
                attempt_id: src_claim.attempt.clone(),
                ordinal: 1,
                sequence: 1,
                kind: EventKind::ProviderStarted {
                    dispatched_at_unix_ms: 1_760_800_000_000,
                    context: DispatchContext::default(),
                    request: SavedRequest {
                        added: Vec::new(),
                        items: Vec::new(),
                    },
                },
            },
        )
        .await
        .unwrap();
    store
        .record_revision(
            &src_claim,
            Event {
                attempt_id: src_claim.attempt.clone(),
                ordinal: 1,
                sequence: 2,
                kind: EventKind::ProviderReturned(src_finish_turn.clone()),
            },
        )
        .await
        .unwrap();
    store
        .record_revision(
            &src_claim,
            Event {
                attempt_id: src_claim.attempt.clone(),
                ordinal: 1,
                sequence: 3,
                kind: EventKind::Candidate {
                    text: String::new(),
                },
            },
        )
        .await
        .unwrap();

    // New corrected lab arrives mid-run before commit_revision:
    store
        .record_chart(
            &env.workspace,
            &[chart_record(
                &env.workspace,
                "labs",
                "lab-potassium",
                "rev-2",
                "laboratory",
                "Corrected basic metabolic panel potassium",
                "2026-11-11",
                "Corrected serum potassium 5.7 mEq/L.",
            )],
            &SourcePull {
                source_id: "labs".into(),
                source_label: "Regional Labs".into(),
                patient: SourcePatient {
                    source_patient_id: "sp-move".into(),
                    display_name: "Elena Navarro".into(),
                    birth_date: Some("1968-04-14".into()),
                    sex_or_gender: Some("female".into()),
                    medical_record_numbers: Vec::new(),
                },
                pulled_at: 1_760_800_000,
            },
        )
        .await
        .unwrap();

    let moved = store
        .commit_revision(&src_claim, &src_finish_turn, &mut PostgresRuntimeAuthority)
        .await
        .expect("commit_revision checks current source version");
    assert!(
        matches!(moved, RevisionCommit::SourcesMoved { .. }),
        "new chart source revision must block stale reply revision commit: {moved:?}"
    );
    assert_eq!(
        store.draft(&source_move_scope).await.unwrap().unwrap().text,
        "Draft before new lab arrived.",
        "draft text must remain unchanged when sources moved"
    );

    // --- 3. Lost access mid-run blocks PhysicianHost and PracticeJobHost output ---
    let chat_recipe = store
        .chat_recipe_with_understanding(
            &roles.chat,
            local_product::physician_chat::INSTRUCTIONS,
            &skills,
            &env.practice,
            &env.workspace,
        )
        .await
        .unwrap();
    store
        .accept_authorized(
            &env.scope,
            &Input {
                key: "msg-lost-panel".into(),
                text: "Summarize chart before access is revoked.".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            env.session_hash.clone(),
            &skills,
            &chat_recipe,
        )
        .await
        .unwrap();
    let lost_chat_claim = store.claim(&env.scope, 60).await.unwrap().unwrap();

    // Revoke panel access while the chat attempt is active:
    auth.set_membership(&env.identity, &env.workspace, Membership::ClinicDemographic)
        .await
        .expect("downgrade panel membership to clinic_demographic");

    struct NeverReachedProvider;
    impl Provider for NeverReachedProvider {
        async fn exchange(
            &mut self,
            _request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            Ok(Turn {
                items: vec![Item::assistant(
                    "Should never publish after lost panel access.",
                )],
                receipt: json!({}),
                usage: Default::default(),
            })
        }
    }
    let mut lost_chat_host = PhysicianHost {
        store: store.clone(),
        claim: lost_chat_claim,
        authority: PostgresRuntimeAuthority,
    };
    assert!(
        lost_chat_host
            .run(
                &mut NeverReachedProvider,
                RunLimits::default(),
                &Cancellation::default(),
            )
            .await
            .is_err(),
        "downgraded panel membership must block PhysicianHost execution/publication"
    );
    assert_eq!(
        store
            .conversation(&env.scope)
            .await
            .unwrap()
            .responses()
            .count(),
        0,
        "zero assistant output published after panel revocation"
    );
}

/// AC3 (`Stop` vs worker interruption):
/// - `Stop` (`stop_authorized` and `stop_practice_job`) cancels immediately
///   without waiting for a slow running tool call, records `AttemptStatus::Cancelled`
///   and `WorkStatus::Held` (`HoldReason::Stopped`), excludes the work from
///   recovery sweeps, and continues in a new linked `run_id` (`continues: Some(first_run)`)
///   only upon explicit `Resume`.
/// - Worker interruption (`Cancellation` settled as `AttemptFailure::Interrupted`)
///   leaves the same `run_id` open (`WorkStatus::Pending`), surfaces in
///   `dispatch_due` / `practice_jobs_dispatch_due`, and resumes automatically.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn ac3_stop_cancels_without_waiting_for_running_tool_while_interruption_stays_auto_recoverable()
 {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();
    let env = setup_fixture(&store, &auth, "ac3-stop").await;

    let slow_started = Arc::new(Notify::new());
    struct HangingMemoryOp {
        started: Arc<Notify>,
        hang: AtomicBool,
        inner: local_product::runtime::chat_operations::PostgresPatientMemoryOperation,
    }
    #[async_trait]
    impl local_product::runtime::chat_operations::PatientMemoryOperation for HangingMemoryOp {
        async fn list_memories(
            &self,
            store: &Store,
            practice: &str,
            workspace: &str,
            selection: MemorySelection,
        ) -> Result<
            Vec<local_product::runtime::chat_operations::PatientMemoryEntry>,
            local_product::runtime::chat_operations::PatientMemoryError,
        > {
            self.inner
                .list_memories(store, practice, workspace, selection)
                .await
        }

        async fn record_correction(
            &self,
            store: &Store,
            command_key: &str,
            input: &local_product::runtime::chat_operations::RecordMemoryCorrection,
        ) -> Result<
            local_product::runtime::chat_operations::MemoryMutation,
            local_product::runtime::chat_operations::PatientMemoryError,
        > {
            if self.hang.swap(false, Ordering::SeqCst) {
                self.started.notify_one();
                tokio::time::sleep(Duration::from_secs(30)).await;
            }
            self.inner
                .record_correction(store, command_key, input)
                .await
        }

        async fn retire_memory(
            &self,
            store: &Store,
            command_key: &str,
            input: &local_product::runtime::chat_operations::RetirePatientMemory,
        ) -> Result<
            local_product::runtime::chat_operations::MemoryMutation,
            local_product::runtime::chat_operations::PatientMemoryError,
        > {
            self.inner.retire_memory(store, command_key, input).await
        }
    }

    let stop_store = store
        .clone()
        .with_memory_operation(Arc::new(HangingMemoryOp {
            started: slow_started.clone(),
            hang: AtomicBool::new(true),
            inner: local_product::runtime::chat_operations::PostgresPatientMemoryOperation,
        }));

    let chat_recipe = stop_store
        .chat_recipe_with_understanding(
            &roles.chat,
            local_product::physician_chat::INSTRUCTIONS,
            &skills,
            &env.practice,
            &env.workspace,
        )
        .await
        .unwrap();
    stop_store
        .accept_authorized(
            &env.scope,
            &Input {
                key: "stop-msg-1".into(),
                text: "Save a note (which will be stopped mid-call).".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            env.session_hash.clone(),
            &skills,
            &chat_recipe,
        )
        .await
        .unwrap();

    let pre_stop = stop_store.conversation(&env.scope).await.unwrap();
    let first_run_id = pre_stop.run().unwrap().id.to_owned();
    let work_id = pre_stop.work.clone();
    let claim_1 = stop_store.claim(&env.scope, 60).await.unwrap().unwrap();

    struct HangingToolProvider;
    impl Provider for HangingToolProvider {
        async fn exchange(
            &mut self,
            _request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            Ok(Turn {
                items: vec![Item::Call(Invocation {
                    id: "slow-call-1".into(),
                    name: PATIENT_MEMORY_TOOL.into(),
                    arguments: json!({
                        "operation": "record_correction",
                        "memory_id": "slow-note",
                        "topic": "Slow note",
                        "statement": "Stopped before finishing.",
                    })
                    .to_string(),
                })],
                receipt: json!({ "attempt": 1 }),
                usage: Default::default(),
            })
        }
    }

    let task_store = stop_store.clone();
    let claim_1_clone = claim_1.clone();
    let running_task = tokio::spawn(async move {
        let mut host = PhysicianHost {
            store: task_store,
            claim: claim_1_clone,
            authority: PostgresRuntimeAuthority,
        };
        host.run(
            &mut HangingToolProvider,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
    });

    slow_started.notified().await;
    let stop_started_at = tokio::time::Instant::now();
    let stopped = stop_store
        .stop_authorized(&env.scope, "stop-cmd-1", env.session_hash.clone(), &work_id)
        .await
        .expect("stop active chat");
    assert_eq!(stopped, Stopped::Held);

    // `Stop` must abort the running attempt promptly without waiting 30s for the tool!
    let stopped_result = tokio::time::timeout(Duration::from_secs(2), running_task)
        .await
        .expect("Stop must not wait for 30s hanging tool call")
        .expect("join stopped task");
    assert!(stopped_result.is_err());
    assert!(stop_started_at.elapsed() < Duration::from_secs(2));
    assert_eq!(
        AttemptFailure::from_error(&stopped_result.unwrap_err(), &Cancellation::default()),
        AttemptFailure::Interrupted
    );
    assert_eq!(
        stop_store
            .settle_chat_attempt(&claim_1, AttemptFailure::Interrupted)
            .await
            .expect("worker settles stopped attempt to release lease"),
        AttemptSettlement::Superseded,
        "settling a stopped attempt releases its lease without reopening held work"
    );

    let held_state = stop_store.conversation(&env.scope).await.unwrap();
    assert_eq!(held_state.status, WorkStatus::Held);
    assert_eq!(held_state.attempts[0].status, AttemptStatus::Cancelled);
    assert_eq!(
        held_state.run().unwrap().outcome,
        Some(RunOutcome::Held {
            reason: HoldReason::Stopped
        })
    );
    assert!(
        !held_state.attempts[0]
            .events
            .iter()
            .any(|e| matches!(e.kind, EventKind::ToolReturned { .. })),
        "stopped tool call remains ToolStarted with no ToolReturned"
    );

    let chat_delivery = Delivery::chat_turn(env.scope.clone(), "stop-msg-1".into());
    assert_eq!(
        stop_store.delivery_state(&chat_delivery).await.unwrap(),
        DeliveryState::Settled,
        "stopped chat turn is settled and not auto-redispatched"
    );
    assert!(stop_store.claim(&env.scope, 60).await.unwrap().is_none());

    // Explicit `Resume` starts a new linked run (`continues: Some(first_run_id)`):
    let resumed = stop_store
        .resume_authorized(
            &env.scope,
            "resume-cmd-1",
            env.session_hash.clone(),
            &work_id,
            &first_run_id,
        )
        .await
        .expect("resume stopped chat");
    assert!(matches!(resumed, Resumed::Pending { .. }));

    let claim_2 = stop_store
        .claim(&env.scope, 60)
        .await
        .unwrap()
        .expect("claim resumed chat");
    struct ResumedAfterStopProvider;
    impl Provider for ResumedAfterStopProvider {
        async fn exchange(
            &mut self,
            request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            let slow_out = materialized_output(request, "slow-call-1")
                .expect("stopped in-flight call reported on resume");
            assert!(slow_out.contains("\"outcome\":\"unknown\""));
            Ok(Turn {
                items: vec![Item::assistant(
                    "Resumed after Stop without repeating unfinished work.\n\nBasis and limitations: Resumed conversation state.",
                )],
                receipt: json!({ "resumed": true }),
                usage: Default::default(),
            })
        }
    }
    let mut host_2 = PhysicianHost {
        store: stop_store.clone(),
        claim: claim_2,
        authority: PostgresRuntimeAuthority,
    };
    host_2
        .run(
            &mut ResumedAfterStopProvider,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .expect("resumed chat completes");

    let completed_after_resume = stop_store.conversation(&env.scope).await.unwrap();
    let second_run_id = completed_after_resume.run().unwrap().id.to_owned();
    assert_ne!(second_run_id, first_run_id);
    let run_events = completed_after_resume.runs.get(&work_id).unwrap().events();
    assert!(
        run_events.iter().any(|ev| matches!(
            ev,
            RunEvent::RunStarted { run_id, continues: Some(prev) }
                if run_id == &second_run_id && prev == &first_run_id
        )),
        "Resume must open a new run linked via continues to the stopped run: {run_events:?}"
    );

    // Contrast with worker interruption on a practice job: same run_id stays open & Due!
    let job_scope = PracticeScope {
        practice: env.practice.clone(),
        workspace: env.workspace.clone(),
        job_id: "job-interrupt-vs-stop".into(),
    };
    let job_assignment = Assignment::new(
        PracticeJobKind::PatientReview,
        "cmd-job-int",
        "Check worker interruption automatic recovery.",
        &["patient-record-retrieval"],
        &skills,
    )
    .unwrap();
    store
        .accept_practice_job(&job_scope, &job_assignment, &roles, &skills, auth.as_ref())
        .await
        .unwrap();
    let job_delivery = Delivery::practice_job(&job_scope, "cmd-job-int".into());
    let job_run_before = store
        .practice_job(&job_scope)
        .await
        .unwrap()
        .unwrap()
        .current_run()
        .unwrap()
        .id
        .to_owned();

    let job_claim_1 = store
        .claim_practice_job(&job_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    let job_cancel = Cancellation::default();
    job_cancel.cancel();
    let job_int_err = PracticeJobHost {
        store: store.clone(),
        claim: job_claim_1.clone(),
        authority: auth.as_ref(),
    }
    .run(&mut HangingToolProvider, RunLimits::default(), &job_cancel)
    .await
    .unwrap_err();
    assert_eq!(
        store
            .settle_practice_job_attempt(
                &job_claim_1,
                AttemptFailure::from_error(&job_int_err, &job_cancel),
            )
            .await
            .unwrap(),
        AttemptSettlement::Applied
    );

    let int_job = store.practice_job(&job_scope).await.unwrap().unwrap();
    assert_eq!(int_job.status, WorkStatus::Pending);
    assert_eq!(int_job.attempts[0].status, AttemptStatus::Interrupted);
    assert_eq!(int_job.current_run().unwrap().id, job_run_before);
    assert_eq!(int_job.current_run().unwrap().outcome, None);
    assert_eq!(
        store.delivery_state(&job_delivery).await.unwrap(),
        DeliveryState::Due,
        "interrupted job remains Due for automatic recovery without human Resume"
    );
    let (due_jobs, _) = store.practice_jobs_dispatch_due(100, None).await.unwrap();
    assert!(due_jobs.contains(&job_delivery));
}

// ============================================================================
// AC4 — Source failure leaves work recoverable; used-up credits, lost access,
//       and retryable interruption read differently
// ============================================================================

/// AC4 (Source failure leaves work recoverable):
/// 1. When a patient source check interrupts mid-pull (`FixedCheckStep::Interrupt`
///    after `ehr` updated and before `labs` settled) in both `PhysicianHost`
///    and `PracticeJobHost`, the run stays recoverable (`WorkStatus::Pending`),
///    retains prior records + `ehr`'s update without re-pulling `ehr`, and
///    completes cleanly on the next attempt.
/// 2. When an upstream patient source returns `SourceCheckOutcome::Failed` and
///    the clinical library is unavailable, retained records and `last_good_pulled_at`
///    remain intact and the conversation completes truthfully stating limits.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn ac4_source_failure_and_mid_refresh_interruption_leave_chat_and_job_recoverable() {
    let Some(url) = database_url() else {
        return;
    };
    let (base_store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();
    let env = setup_fixture(&base_store, &auth, "ac4-src").await;

    let ehr_id = SourceId::new("ehr").unwrap();
    let labs_id = SourceId::new("labs").unwrap();
    let checker = FixedSourceCheck::default();
    checker.push_step(
        &env.practice,
        &env.workspace,
        ehr_id.clone(),
        FixedCheckStep::Outcome(SourceCheckOutcome::Updated {
            source_label: "Clinic EHR".into(),
            patient: SourcePatient {
                source_patient_id: "sp-ac4".into(),
                display_name: "Elena Navarro".into(),
                birth_date: Some("1968-04-14".into()),
                sex_or_gender: Some("female".into()),
                medical_record_numbers: Vec::new(),
            },
            pulled_at: 1_760_900_000,
            records: vec![chart_record(
                &env.workspace,
                "ehr",
                "visit-note-ac4",
                "rev-2",
                "note",
                "Cardiology note",
                "2026-11-12",
                "Added chlorthalidone 12.5 mg daily.",
            )],
        }),
    );

    let cancel_chat_1 = Cancellation::default();
    let cancel_hook = cancel_chat_1.clone();
    checker.push_step(
        &env.practice,
        &env.workspace,
        labs_id.clone(),
        FixedCheckStep::Interrupt {
            reason: "Regional Labs TLS stream reset mid-pull".into(),
            on_interrupt: Some(Arc::new(move || cancel_hook.cancel())),
        },
    );
    checker.push_step(
        &env.practice,
        &env.workspace,
        labs_id.clone(),
        FixedCheckStep::Outcome(SourceCheckOutcome::Failed {
            source_label: Some("Regional Labs".into()),
            checked_at: 1_760_900_060,
            reason: "Regional Labs FHIR gateway HTTP 503".into(),
        }),
    );

    let store = base_store.with_source_refresh(Arc::new(checker.clone()));
    let chat_recipe = store
        .chat_recipe_with_understanding(
            &roles.chat,
            local_product::physician_chat::INSTRUCTIONS,
            &skills,
            &env.practice,
            &env.workspace,
        )
        .await
        .unwrap();
    store
        .accept_authorized(
            &env.scope,
            &Input {
                key: "msg-ac4-src".into(),
                text: "Refresh sources, check literature on chlorthalidone hypokalemia, and summarize.".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            env.session_hash.clone(),
            &skills,
            &chat_recipe,
        )
        .await
        .unwrap();

    // Attempt 1: `ehr` updates once, `labs` interrupts mid-pull -> run stays Pending!
    let claim_1 = store.claim(&env.scope, 60).await.unwrap().unwrap();
    struct ChatSourceRecoveryProvider {
        phase: usize,
    }
    impl Provider for ChatSourceRecoveryProvider {
        async fn exchange(
            &mut self,
            request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            self.phase += 1;
            match self.phase {
                1 => Ok(Turn {
                    items: vec![Item::Call(Invocation {
                        id: "refresh-1".into(),
                        name: REFRESH_SOURCES_TOOL.into(),
                        arguments: json!({
                            "source_id": null,
                            "reason": "Check latest cardiology and laboratory records.",
                        })
                        .to_string(),
                    })],
                    receipt: json!({ "phase": 1 }),
                    usage: Default::default(),
                }),
                2 => {
                    let unknown = materialized_output(request, "refresh-1")
                        .expect("interrupted refresh reports unknown");
                    assert!(
                        unknown.contains("\"outcome\":\"unknown\""),
                        "mid-pull interruption must report unknown so retry reuses settled ehr: {unknown}"
                    );
                    Ok(Turn {
                        items: vec![
                            Item::Call(Invocation {
                                id: "refresh-2".into(),
                                name: REFRESH_SOURCES_TOOL.into(),
                                arguments: json!({
                                    "source_id": null,
                                    "reason": "Check latest cardiology and laboratory records.",
                                })
                                .to_string(),
                            }),
                            Item::Call(Invocation {
                                id: "lib-search-1".into(),
                                name: SEARCH_LIBRARY_TOOL.into(),
                                arguments: json!({
                                    "query": "chlorthalidone hypokalemia monitoring",
                                })
                                .to_string(),
                            }),
                        ],
                        receipt: json!({ "phase": 2 }),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let refresh_2 =
                        materialized_output(request, "refresh-2").expect("refresh-2 result");
                    assert!(
                        refresh_2.contains("Regional Labs FHIR gateway HTTP 503"),
                        "source outage reported as structured limit: {refresh_2}"
                    );
                    let lib_out =
                        materialized_output(request, "lib-search-1").expect("library output");
                    assert!(
                        lib_out.contains("\"outcome\":\"unavailable\""),
                        "unconfigured/unreachable library returns unavailable: {lib_out}"
                    );
                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "read-retained".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [
                                    { "candidate_id": "visit-note-ac4", "revision": "rev-2" },
                                    { "candidate_id": "lab-potassium", "revision": "rev-1" },
                                ],
                                "max_result_bytes": 16_384,
                            })
                            .to_string(),
                        })],
                        receipt: json!({ "phase": 3 }),
                        usage: Default::default(),
                    })
                }
                4 => {
                    let retained =
                        materialized_output(request, "read-retained").expect("retained records");
                    assert!(retained.contains("chlorthalidone 12.5 mg"));
                    assert!(retained.contains("4.3 mEq/L"));
                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Cardiology added chlorthalidone 12.5 mg daily on 2026-11-12 [1]; last available potassium was 4.3 mEq/L on 2026-10-15 [2].\n\nBasis and limitations: Clinic EHR updated 2026-11-12; Regional Labs refresh failed (last good pull 2026-10-15) and the clinical library was unavailable.",
                        )],
                        receipt: json!({ "phase": 4 }),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected phase {other}"),
            }
        }
    }

    let mut provider = ChatSourceRecoveryProvider { phase: 0 };
    let mut host_1 = PhysicianHost {
        store: store.clone(),
        claim: claim_1.clone(),
        authority: PostgresRuntimeAuthority,
    };
    let err_1 = host_1
        .run(&mut provider, RunLimits::default(), &cancel_chat_1)
        .await
        .expect_err("mid-pull source interruption aborts attempt 1");
    store
        .settle_chat_attempt(&claim_1, AttemptFailure::from_error(&err_1, &cancel_chat_1))
        .await
        .unwrap();

    let after_1 = store.conversation(&env.scope).await.unwrap();
    assert_eq!(
        after_1.status,
        WorkStatus::Pending,
        "source failure leaves work recoverable"
    );

    // Attempt 2 recovers: `ehr` is NOT re-pulled (`call_count == 1`), `labs` records
    // `Failed` while preserving `last_good_pulled_at`, and the run completes!
    let claim_2 = store.claim(&env.scope, 60).await.unwrap().unwrap();
    let mut host_2 = PhysicianHost {
        store: store.clone(),
        claim: claim_2,
        authority: PostgresRuntimeAuthority,
    };
    host_2
        .run(
            &mut provider,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .expect("attempt 2 completes from retained chart records");

    assert_eq!(
        checker.call_count(&env.practice, &env.workspace, &ehr_id),
        1,
        "already-settled ehr source must not be pulled again on retry"
    );
    assert_eq!(
        checker.call_count(&env.practice, &env.workspace, &labs_id),
        2
    );
    let statuses = store
        .saved_source_statuses(&env.practice, &env.workspace)
        .await
        .unwrap();
    let labs_status = statuses
        .iter()
        .find(|s| s.source_id == labs_id)
        .expect("saved labs status");
    assert!(matches!(
        labs_status.outcome,
        SourceRefreshOutcome::Failed {
            last_good_pulled_at: Some(1_760_500_010),
            ..
        }
    ));
    let completed = store.conversation(&env.scope).await.unwrap();
    assert_eq!(completed.status, WorkStatus::Completed);
    assert_eq!(completed.responses().count(), 1);

    // Also verify InitialUnderstanding background job stays recoverable when a source fails:
    let und_context = UnderstandingContext {
        workspace: env.workspace.clone(),
        mode: UnderstandingMode::Initial,
        target_source_version: store.sources(&env.workspace).await.unwrap().version,
        evidence_scope: EvidenceScope::ClinicalChart {
            connected_sources: vec![ConnectedSourceBrief {
                source_id: "ehr".into(),
                source_label: "Clinic EHR".into(),
                pulled_at: 1_760_900_000,
            }],
        },
        changed_candidates: vec!["visit-note-ac4".into()],
    };
    let und_assignment =
        local_product::initial_understanding::assignment(und_context, &skills).unwrap();
    let und_scope = PracticeScope {
        practice: env.practice.clone(),
        workspace: env.workspace.clone(),
        job_id: "job-und-after-src-fail".into(),
    };
    store
        .accept_practice_job(&und_scope, &und_assignment, &roles, &skills, auth.as_ref())
        .await
        .unwrap();
    let und_claim = store
        .claim_practice_job(&und_scope, 60, auth.as_ref())
        .await
        .unwrap()
        .unwrap();
    struct SaveUnderstandingProvider {
        round: usize,
    }
    impl Provider for SaveUnderstandingProvider {
        async fn exchange(
            &mut self,
            request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            self.round += 1;
            match self.round {
                1 => Ok(Turn {
                    items: vec![
                        Item::Call(Invocation {
                            id: "und-refresh".into(),
                            name: REFRESH_TOOL.into(),
                            arguments: json!({ "source_ids": ["labs"] }).to_string(),
                        }),
                        Item::Call(Invocation {
                            id: "und-read".into(),
                            name: READ_TOOL.into(),
                            arguments: json!({
                                "references": [
                                    { "candidate_id": "visit-note-ac4", "revision": "rev-2" },
                                    { "candidate_id": "lab-potassium", "revision": "rev-1" },
                                ],
                                "max_result_bytes": 16_384,
                            })
                            .to_string(),
                        }),
                    ],
                    receipt: json!({ "round": 1 }),
                    usage: Default::default(),
                }),
                2 => {
                    let ref_report: RefreshReport =
                        serde_json::from_str(materialized_output(request, "und-refresh").unwrap())
                            .unwrap();
                    assert!(matches!(
                        ref_report.sources[0].outcome,
                        SourceRefreshOutcome::Failed { .. }
                    ));
                    let save_args = SaveUnderstandingArgs {
                        expected_revision: Some(0),
                        operating_index: OperatingIndex {
                            working_overview: "Stage 2 hypertension on amlodipine + chlorthalidone.".into(),
                            known_health_systems: Vec::new(),
                            open_threads: vec!["Recheck potassium once Regional Labs is reachable.".into()],
                            anchors: vec![local_product::initial_understanding::IndexedAnchor {
                                reference: ExactRecordReference::new("visit-note-ac4", "rev-2")
                                    .unwrap(),
                                source_id: "ehr".into(),
                                topic: "Chlorthalidone start".into(),
                            }],
                        },
                        claims: vec![SummaryClaimInput {
                            text: "Started chlorthalidone 12.5 mg daily on 2026-11-12.".into(),
                            citations: vec![
                                ExactRecordReference::new("visit-note-ac4", "rev-2").unwrap(),
                            ],
                        }],
                        material_gaps: vec![
                            "Regional Labs refresh failed on 2026-11-12; latest potassium is from 2026-10-15."
                                .into(),
                        ],
                        scope_note: None,
                    };
                    Ok(Turn {
                        items: vec![Item::Call(Invocation {
                            id: "und-save".into(),
                            name: SAVE_UNDERSTANDING_TOOL.into(),
                            arguments: serde_json::to_value(save_args).unwrap().to_string(),
                        })],
                        receipt: json!({ "round": 2 }),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let _ = READ_UNDERSTANDING_TOOL;
                    Ok(Turn {
                        items: vec![Item::assistant(
                            "Saved initial understanding noting Regional Labs refresh outage.",
                        )],
                        receipt: json!({ "round": 3 }),
                        usage: Default::default(),
                    })
                }
                other => panic!("unexpected round {other}"),
            }
        }
    }
    checker.set_source_outcome(
        &env.practice,
        &env.workspace,
        labs_id,
        SourceCheckOutcome::Failed {
            source_label: Some("Regional Labs".into()),
            checked_at: 1_760_900_120,
            reason: "Regional Labs still unreachable".into(),
        },
    );
    PracticeJobHost {
        store: store.clone(),
        claim: und_claim,
        authority: auth.as_ref(),
    }
    .run(
        &mut SaveUnderstandingProvider { round: 0 },
        RunLimits::default(),
        &Cancellation::default(),
    )
    .await
    .expect("understanding job completes truthfully despite lab source failure");
}

#[derive(Clone)]
enum MockResponsesReply {
    HttpError { status: StatusCode, body: Value },
    CompletedText(&'static str),
}

async fn spawn_mock_responses_server(
    replies: Vec<MockResponsesReply>,
) -> (String, Arc<AtomicUsize>, oneshot::Sender<()>) {
    let queue = Arc::new(Mutex::new(VecDeque::from(replies)));
    let hits = Arc::new(AtomicUsize::new(0));
    let queue_route = queue.clone();
    let hits_route = hits.clone();

    let app = Router::new().route(
        "/v1/responses",
        post(move || {
            let queue = queue_route.clone();
            let hits = hits_route.clone();
            async move {
                hits.fetch_add(1, Ordering::SeqCst);
                let next = queue
                    .lock()
                    .unwrap()
                    .pop_front()
                    .expect("queued mock provider reply");
                match next {
                    MockResponsesReply::HttpError { status, body } => {
                        let mut headers = HeaderMap::new();
                        headers.insert(
                            header::CONTENT_TYPE,
                            HeaderValue::from_static("application/json"),
                        );
                        (status, headers, body.to_string()).into_response()
                    }
                    MockResponsesReply::CompletedText(text) => {
                        let item = json!({
                            "type": "message",
                            "id": "msg-mock-ok",
                            "role": "assistant",
                            "status": "completed",
                            "content": [{ "type": "output_text", "text": text }],
                        });
                        let response = json!({
                            "id": "resp-mock-ok",
                            "model": "recovery-chat-lane",
                            "status": "completed",
                            "output": [item.clone()],
                            "usage": { "input_tokens": 120, "output_tokens": 30, "total_tokens": 150 }
                        });
                        let sse = format!(
                            "data: {}\n\ndata: {}\n\ndata: {}\n\n",
                            json!({
                                "type": "response.created",
                                "response": { "id": "resp-mock-ok", "status": "in_progress" }
                            }),
                            json!({
                                "type": "response.output_item.done",
                                "output_index": 0,
                                "item": item
                            }),
                            json!({
                                "type": "response.completed",
                                "response": response
                            })
                        );
                        let mut headers = HeaderMap::new();
                        headers.insert(
                            header::CONTENT_TYPE,
                            HeaderValue::from_static("text/event-stream"),
                        );
                        (StatusCode::OK, headers, sse).into_response()
                    }
                }
            }
        }),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    tokio::spawn(async move {
        let _ = axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = shutdown_rx.await;
            })
            .await;
    });
    (format!("http://{addr}/v1/responses"), hits, shutdown_tx)
}

#[derive(Default)]
struct OpenQuotaGate;

#[async_trait]
impl QuotaGate for OpenQuotaGate {
    async fn remaining(&self) -> Result<Duration, harness::engine::Cause> {
        Ok(Duration::ZERO)
    }

    async fn defer(&self, _wait: Duration) -> Result<(), harness::engine::Cause> {
        Ok(())
    }
}

/// AC4 (Used-up credits, lost access, and retryable interruption read differently):
/// Exercises real [`ResponsesProvider`] over local HTTP + real [`PhysicianHost`]
/// and [`PracticeJobHost`] to prove the three conditions have distinct durable
/// states, error causes, and recovery behaviors:
/// 1. **Used-up credits (`credit_balance_exhausted`):** stops immediately on
///    attempt 1 (`hits == 1`, `failures == 0`), holds the run with
///    `RunOutcome::Held { reason: HoldReason::ProviderBlocked }`,
///    `DeliveryState::Settled`, and plain billing message (`"Provider credits are exhausted..."`).
/// 2. **Lost access:**
///    - Practice patient access loss preserves typed [`PracticeAccessError`] on
///      both `claim_practice_job` and mid-run `PracticeJobHost` failure.
///    - Physician session / panel revocation refuses `PhysicianHost` without
///      publishing output.
///    - Provider HTTP 401 holds the run with `"The model provider did not accept this request..."`
///      (distinct from billing exhaustion and from retryable interruption).
/// 3. **Retryable interruption:** worker cancellation or transient HTTP 503
///    before output leaves the run open (`WorkStatus::Pending`, `run.outcome == None`,
///    `DeliveryState::Due`) or retries in-run to completion.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn ac4_used_up_credits_lost_access_and_retryable_interruption_read_distinctly() {
    let Some(url) = database_url() else {
        return;
    };
    let (store, auth) = connect_stores(&url).await;
    let skills = load_skills();
    let roles = distinct_roles();
    let env = setup_fixture(&store, &auth, "ac4-distinct").await;

    let chat_recipe = store
        .chat_recipe_with_understanding(
            &roles.chat,
            local_product::physician_chat::INSTRUCTIONS,
            &skills,
            &env.practice,
            &env.workspace,
        )
        .await
        .unwrap();

    // --- 1. Used-up credits (`credit_balance_exhausted`) ---
    let (credits_endpoint, credits_hits, credits_shutdown) =
        spawn_mock_responses_server(vec![MockResponsesReply::HttpError {
            status: StatusCode::TOO_MANY_REQUESTS,
            body: json!({
                "error": {
                    "type": "insufficient_quota",
                    "code": "credit_balance_exhausted",
                    "message": "internal vendor text never shown",
                }
            }),
        }])
        .await;

    let credits_scope = env.scope.clone();
    store
        .accept_authorized(
            &credits_scope,
            &Input {
                key: "msg-credits".into(),
                text: "Check hypertension plan.".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            env.session_hash.clone(),
            &skills,
            &chat_recipe,
        )
        .await
        .unwrap();
    let credits_delivery = Delivery::chat_turn(credits_scope.clone(), "msg-credits".into());
    let credits_claim = store.claim(&credits_scope, 60).await.unwrap().unwrap();
    let mut credits_provider = ResponsesProvider::new(
        credits_endpoint,
        "test-api-key".into(),
        Arc::new(OpenQuotaGate),
    )
    .unwrap();
    let mut credits_host = PhysicianHost {
        store: store.clone(),
        claim: credits_claim.clone(),
        authority: PostgresRuntimeAuthority,
    };
    let credits_err = credits_host
        .run(
            &mut credits_provider,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .expect_err("exhausted credits must stop immediately");
    assert_eq!(
        credits_hits.load(Ordering::SeqCst),
        1,
        "exhausted credits must stop on first request without retrying"
    );
    assert_eq!(
        AttemptFailure::from_error(&credits_err, &Cancellation::default()),
        AttemptFailure::ProviderBlocked
    );
    let _ = credits_shutdown.send(());

    let credits_state = store.conversation(&credits_scope).await.unwrap();
    assert_eq!(credits_state.status, WorkStatus::Held);
    assert_eq!(credits_state.failures, 0);
    assert_eq!(credits_state.attempts[0].status, AttemptStatus::Held);
    assert_eq!(
        credits_state.run().unwrap().outcome,
        Some(RunOutcome::Held {
            reason: HoldReason::ProviderBlocked
        })
    );
    assert_eq!(
        store.delivery_state(&credits_delivery).await.unwrap(),
        DeliveryState::Settled
    );
    let credits_reason = credits_state.attempts[0]
        .events
        .iter()
        .find_map(|ev| match &ev.kind {
            EventKind::ProviderBlocked { reason } => Some(reason.as_str()),
            _ => None,
        })
        .expect("ProviderBlocked event saved");
    assert!(
        credits_reason.contains("Provider credits are exhausted")
            && credits_reason.contains("Restore billing"),
        "billing block carries plain billing message: {credits_reason}"
    );
    assert!(!credits_reason.contains("internal vendor text"));

    // --- 2. Lost access (Practice patient lost access + Physician session lost access + Provider 401) ---
    struct TogglePracticeAuth {
        inner: Arc<PostgresAuthStore>,
        revoked: AtomicBool,
    }
    #[async_trait]
    impl PracticeAuthority for TogglePracticeAuth {
        async fn check_practice_patient(
            &self,
            scope: &PracticeScope,
        ) -> Result<(), PracticeAccessError> {
            if self.revoked.load(Ordering::SeqCst) {
                return Err(PracticeAccessError::ForeignPractice {
                    requested_practice: scope.practice.clone(),
                    owner_practice: "other-practice".into(),
                    workspace: scope.workspace.clone(),
                });
            }
            self.inner.check_practice_patient(scope).await
        }
    }
    let toggle_auth = TogglePracticeAuth {
        inner: auth.clone(),
        revoked: AtomicBool::new(false),
    };
    let lost_job_scope = PracticeScope {
        practice: env.practice.clone(),
        workspace: env.workspace.clone(),
        job_id: "job-lost-access".into(),
    };
    let lost_job_assignment = Assignment::new(
        PracticeJobKind::PatientReview,
        "cmd-lost-access",
        "Review patient before reassignment.",
        &["patient-record-retrieval"],
        &skills,
    )
    .unwrap();
    store
        .accept_practice_job(
            &lost_job_scope,
            &lost_job_assignment,
            &roles,
            &skills,
            &toggle_auth,
        )
        .await
        .unwrap();
    let lost_job_claim = store
        .claim_practice_job(&lost_job_scope, 60, &toggle_auth)
        .await
        .unwrap()
        .unwrap();

    // Revoke practice patient access mid-run!
    toggle_auth.revoked.store(true, Ordering::SeqCst);
    let mut lost_job_host = PracticeJobHost {
        store: store.clone(),
        claim: lost_job_claim,
        authority: &toggle_auth,
    };
    struct DummyProvider;
    impl Provider for DummyProvider {
        async fn exchange(
            &mut self,
            _request: &harness::engine::recipe::Request,
            _cancel: &Cancellation,
            _limits: &RunLimits,
            _observer: &mut dyn Observer,
        ) -> Result<Turn, harness::engine::Cause> {
            Ok(Turn {
                items: vec![Item::assistant("Should not run")],
                receipt: json!({}),
                usage: Default::default(),
            })
        }
    }
    let lost_job_err = lost_job_host
        .run(
            &mut DummyProvider,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .expect_err("lost practice access must fail PracticeJobHost");
    let Failure::Host(host_cause) = &lost_job_err else {
        panic!("expected Failure::Host on lost practice access: {lost_job_err}");
    };
    assert!(
        matches!(
            host_cause.downcast_ref::<PracticeAccessError>(),
            Some(PracticeAccessError::ForeignPractice { .. })
        ),
        "mid-run practice job access refusal must preserve typed PracticeAccessError, got: {host_cause:?}"
    );
    assert!(matches!(
        store
            .claim_practice_job(&lost_job_scope, 60, &toggle_auth)
            .await,
        Err(PracticeJobError::Access(
            PracticeAccessError::ForeignPractice { .. }
        ))
    ));

    // Also check provider HTTP 401 (lost provider credentials) vs used-up credits:
    let (auth401_endpoint, auth401_hits, auth401_shutdown) =
        spawn_mock_responses_server(vec![MockResponsesReply::HttpError {
            status: StatusCode::UNAUTHORIZED,
            body: json!({
                "error": { "type": "invalid_request_error", "code": "invalid_api_key" }
            }),
        }])
        .await;
    let mut auth401_scope = env.scope.clone();
    auth401_scope.chat = format!("{}-401", env.scope.chat);
    store
        .accept_authorized(
            &auth401_scope,
            &Input {
                key: "msg-401".into(),
                text: "Check with revoked provider key.".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            env.session_hash.clone(),
            &skills,
            &chat_recipe,
        )
        .await
        .unwrap();
    let auth401_claim = store.claim(&auth401_scope, 60).await.unwrap().unwrap();
    let mut auth401_provider = ResponsesProvider::new(
        auth401_endpoint,
        "revoked-key".into(),
        Arc::new(OpenQuotaGate),
    )
    .unwrap();
    let _ = PhysicianHost {
        store: store.clone(),
        claim: auth401_claim,
        authority: PostgresRuntimeAuthority,
    }
    .run(
        &mut auth401_provider,
        RunLimits::default(),
        &Cancellation::default(),
    )
    .await
    .unwrap_err();
    assert_eq!(auth401_hits.load(Ordering::SeqCst), 1);
    let _ = auth401_shutdown.send(());
    let auth401_state = store.conversation(&auth401_scope).await.unwrap();
    let auth401_reason = auth401_state.attempts[0]
        .events
        .iter()
        .find_map(|ev| match &ev.kind {
            EventKind::ProviderBlocked { reason } => Some(reason.as_str()),
            _ => None,
        })
        .unwrap();
    assert_ne!(
        auth401_reason, credits_reason,
        "provider 401 and exhausted credits carry distinct reasons"
    );

    // --- 3. Retryable interruption (worker cancellation + transient HTTP 503 before output) ---
    let (retry_endpoint, retry_hits, retry_shutdown) = spawn_mock_responses_server(vec![
        MockResponsesReply::HttpError {
            status: StatusCode::SERVICE_UNAVAILABLE,
            body: json!({ "error": { "type": "server_error", "message": "temporary upstream 503" } }),
        },
        MockResponsesReply::CompletedText(
            "Recovered cleanly after transient upstream 503.\n\nBasis and limitations: Seeded chart.",
        ),
    ])
    .await;

    let mut retry_scope = env.scope.clone();
    retry_scope.chat = format!("{}-retry", env.scope.chat);
    store
        .accept_authorized(
            &retry_scope,
            &Input {
                key: "msg-retry".into(),
                text: "Check retryable interruption.".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            env.session_hash.clone(),
            &skills,
            &chat_recipe,
        )
        .await
        .unwrap();
    let retry_delivery = Delivery::chat_turn(retry_scope.clone(), "msg-retry".into());

    // First attempt: worker cancelled before model call -> Pending + Due + open run!
    let int_claim_1 = store.claim(&retry_scope, 60).await.unwrap().unwrap();
    let int_cancel = Cancellation::default();
    int_cancel.cancel();
    let mut retry_provider =
        ResponsesProvider::new(retry_endpoint, "valid-key".into(), Arc::new(OpenQuotaGate))
            .unwrap();
    let int_err = PhysicianHost {
        store: store.clone(),
        claim: int_claim_1.clone(),
        authority: PostgresRuntimeAuthority,
    }
    .run(&mut retry_provider, RunLimits::default(), &int_cancel)
    .await
    .unwrap_err();
    let int_classification = AttemptFailure::from_error(&int_err, &int_cancel);
    assert_eq!(int_classification, AttemptFailure::Interrupted);
    store
        .settle_chat_attempt(&int_claim_1, int_classification)
        .await
        .unwrap();

    let retryable_state = store.conversation(&retry_scope).await.unwrap();
    assert_eq!(
        retryable_state.status,
        WorkStatus::Pending,
        "retryable interruption stays Pending (unlike Held credits)"
    );
    assert_eq!(
        retryable_state.run().unwrap().outcome,
        None,
        "retryable interruption keeps the run open"
    );
    assert_eq!(
        retryable_state.attempts[0].status,
        AttemptStatus::Interrupted
    );
    assert_eq!(
        store.delivery_state(&retry_delivery).await.unwrap(),
        DeliveryState::Due,
        "retryable interruption stays Due (unlike Settled credits)"
    );

    // Second attempt: hits transient HTTP 503 once (`TransportRetry` recorded),
    // retries automatically, and completes with one stored reply!
    let int_claim_2 = store.claim(&retry_scope, 60).await.unwrap().unwrap();
    PhysicianHost {
        store: store.clone(),
        claim: int_claim_2,
        authority: PostgresRuntimeAuthority,
    }
    .run(
        &mut retry_provider,
        RunLimits::default(),
        &Cancellation::default(),
    )
    .await
    .expect("second attempt recovers past transient 503");
    assert_eq!(retry_hits.load(Ordering::SeqCst), 2);
    let _ = retry_shutdown.send(());

    let recovered_state = store.conversation(&retry_scope).await.unwrap();
    assert_eq!(recovered_state.status, WorkStatus::Completed);
    assert_eq!(recovered_state.failures, 0);
    assert!(
        recovered_state.attempts[1]
            .events
            .iter()
            .any(|ev| matches!(ev.kind, EventKind::TransportRetry { .. })),
        "transient 503 recorded as TransportRetry"
    );
    assert_eq!(recovered_state.responses().count(), 1);
}
