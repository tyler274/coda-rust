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

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;

use harness::engine::recipe::{Finish, Observer, Product, Provider, ReplayEvent, ToolFuture};
use harness::engine::recovery::Recovered;
use harness::engine::runs::RunOutcome;
use harness::engine::{
    AcceptedInput, AttemptFailure, Cancellation, Event, EventKind, Invocation, Item, LimitBackstop,
    ProductMessage, RunLimits, ToolOutcome, Turn, history,
};
use local_product::physician_chat::PhysicianSkills;
use local_product::physician_sources::{
    MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
use local_product::runtime::chat_operations::{
    PatientRefreshOperation, PatientStartingContext, REFRESH_SOURCES_TOOL,
    RuntimeSourceRefreshOperation, SavedChatRefresh, SourceRefreshError, SourceRefreshReport,
};
use local_product::runtime::concerns::{
    CONCERN_TOOL, ConcernStanding, ConcernUrgency, WorkspaceAccess,
};
use local_product::runtime::jobs::{
    Assignment, PracticeJobClaim, PracticeJobCompletion, PracticeJobHost, PracticeJobKind,
    PracticeScope,
};
use local_product::runtime::refresh::{
    FixedSourceCheck, REFRESH_TOOL, SourceCheckOutcome, SourceId,
};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::{
    Accepted, AttemptStatus, Authority, Claim, Conversation, Input, MessageRole, PhysicianHost,
    PublicationTransaction, Scope, Store, WorkStatus, async_trait, persistence_failure,
};
use local_product::{ModelBinding, ModelRoles};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use sea_orm::{ConnAcquireErr, DbErr};
use serde_json::{Value, json};
use tokio::sync::Notify;
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
                pulled_at: Some(1_760_500_000),
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
                pulled_at: Some(1_760_500_010),
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
        envelope: None,
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
    ) -> Result<Option<Vec<ProductMessage>>, harness::engine::ProductError> {
        self.inner.message_sources().await
    }

    async fn admit_binding(
        &mut self,
        recipe: &harness::engine::recipe::RequestBinding,
    ) -> Result<(), harness::engine::ProductError> {
        self.inner.admit_binding(recipe).await
    }

    fn read_only(&self, name: &str) -> bool {
        self.inner.read_only(name)
    }

    fn terminal(&self, turn: &Turn) -> bool {
        self.inner.terminal(turn)
    }

    fn read_result(&mut self, id: &str) -> ToolFuture<Option<String>> {
        self.inner.read_result(id)
    }

    fn saved_outcome(&mut self, call: &Invocation) -> ToolFuture<Option<ToolOutcome>> {
        self.inner.saved_outcome(call)
    }

    async fn assert_current(&mut self) -> Result<(), harness::engine::ProductError> {
        self.inner.assert_current().await
    }

    async fn persist(&mut self, event: Event) -> Result<(), harness::engine::ProductError> {
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

    async fn finish(&mut self, turn: &Turn) -> Result<Finish, harness::engine::ProductError> {
        if matches!(self.fault, PersistenceFault::FailFinish) {
            return Err(DbErr::Custom("injected finish commit failure".into()).into());
        }
        self.inner.finish(turn).await
    }

    async fn finish_partial(
        &mut self,
        turn: &Turn,
        backstops: &[LimitBackstop],
    ) -> Result<(), harness::engine::ProductError> {
        self.inner.finish_partial(turn, backstops).await
    }

    async fn window_seed(
        &mut self,
    ) -> Result<harness::engine::WindowSeed, harness::engine::ProductError> {
        self.inner.window_seed().await
    }
}

fn chat_finish_turn(
    call_id: &str,
    answer: &str,
    basis: &str,
    citations: Value,
    cites_none: bool,
    receipt: Value,
    usage: harness::engine::recipe::Usage,
) -> Turn {
    Turn {
        items: vec![Item::Call(Invocation {
            id: call_id.into(),
            name: local_product::physician_sources::citations::FINISH_TOOL.into(),
            arguments: json!({
                "answer": answer,
                "basis": basis,
                "descriptions": [], "quotes": citations,
                "cites_none": cites_none,
            })
            .to_string(),
        })],
        receipt,
        usage,
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
    if matches!(
        msg.role,
        MessageRole::Assistant {
            kind: local_product::runtime::AssistantKind::Kept,
            ..
        }
    ) {
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
            MessageRole::Assistant {
                id,
                kind: local_product::runtime::AssistantKind::Kept,
            } => Some(id.clone()),
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
            None,
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
            shown_steps: local_product::runtime::ShownSteps::new(()),
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
    let reason_1 = AttemptFailure::of(&err_1, &cancel_1);
    assert_eq!(reason_1, AttemptFailure::Interrupted);
    store
        .settle_chat_attempt(
            &claim_1,
            reason_1,
            local_product::runtime::delivery::Backoff::IMMEDIATE,
        )
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
    // cancels inside a slow refresh operation before committing.
    struct SlowAbortingRefreshOp {
        cancel: Cancellation,
        abort_next: AtomicBool,
    }

    #[async_trait]
    impl PatientRefreshOperation for SlowAbortingRefreshOp {
        async fn refresh_sources(
            &self,
            _store: &Store,
            _scope: &Scope,
            _source_id: Option<&str>,
            _reason: &str,
        ) -> Result<SourceRefreshReport, SourceRefreshError> {
            unreachable!("Attempt 2 uses refresh_sources_for_work")
        }

        async fn refresh_sources_for_work(
            &self,
            store: &Store,
            scope: &Scope,
            work_id: &str,
            source_id: Option<&str>,
            reason: &str,
        ) -> Result<SourceRefreshReport, SourceRefreshError> {
            if self.abort_next.swap(false, Ordering::SeqCst) {
                self.cancel.cancel();
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
            RuntimeSourceRefreshOperation
                .refresh_sources_for_work(store, scope, work_id, source_id, reason)
                .await
        }

        async fn saved_refresh_for_work(
            &self,
            store: &Store,
            scope: &Scope,
            work_id: &str,
            source_id: Option<&str>,
            reason: &str,
        ) -> Result<SavedChatRefresh, SourceRefreshError> {
            RuntimeSourceRefreshOperation
                .saved_refresh_for_work(store, scope, work_id, source_id, reason)
                .await
        }
    }

    let cancel_2 = Cancellation::default();
    let store_with_slow_refresh =
        store
            .clone()
            .with_refresh_operation(Arc::new(SlowAbortingRefreshOp {
                cancel: cancel_2.clone(),
                abort_next: AtomicBool::new(true),
            }));

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
                    id: "call-refresh-uncommitted".into(),
                    name: REFRESH_SOURCES_TOOL.into(),
                    arguments: json!({
                        "source_id": "ehr",
                        "reason": "Re-check EHR before committing reply",
                    })
                    .to_string(),
                })],
                receipt: json!({ "attempt": 2 }),
                usage: Default::default(),
            })
        }
    }

    let mut host_2 = PhysicianHost {
        store: store_with_slow_refresh.clone(),
        claim: claim_2.clone(),
        authority: PostgresRuntimeAuthority,
        shown_steps: local_product::runtime::ShownSteps::new(()),
    };
    let err_2 = host_2
        .run(&mut Attempt2ChatProvider, RunLimits::default(), &cancel_2)
        .await
        .expect_err("attempt 2 cancels before uncommitted refresh finishes");
    let reason_2 = AttemptFailure::of(&err_2, &cancel_2);
    assert_eq!(reason_2, AttemptFailure::Interrupted);
    store
        .settle_chat_attempt(
            &claim_2,
            reason_2,
            local_product::runtime::delivery::Backoff::IMMEDIATE,
        )
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
                    let unknown_refresh = materialized_output(request, "call-refresh-uncommitted")
                        .expect("unknown output for uncommitted refresh call");
                    assert!(
                        unknown_refresh.contains("\"outcome\":\"unknown\""),
                        "uncommitted tool call must report unknown outcome: {unknown_refresh}"
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
                                name: "add_patient_memory_note".into(),
                                arguments: json!({
                                    "text": "Prior cough on lisinopril reported by patient; avoid ACE inhibitors.",
                                    "quote": null,
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
                    Ok(chat_finish_turn(
                        "finish-attempt-3",
                        "Blood pressure rose to 156/96 mmHg on 2026-11-10 [S2] from 148/92 mmHg on 2026-10-15 [S1]. Raised an urgent blood pressure concern and recorded ACE inhibitor intolerance.",
                        "Clinic EHR notes dated 2026-10-15 and 2026-11-10 and physician-supplied allergy correction.",
                        json!([
                            { "label": "S1", "quote": "148/92 mmHg" },
                            { "label": "S2", "quote": "156/96 mmHg" }
                        ]),
                        false,
                        json!({ "attempt": 3, "round": 2 }),
                        Default::default(),
                    ))
                }
                other => panic!("unexpected round {other}"),
            }
        }
    }

    let mut host_3 = PhysicianHost {
        store: store_with_slow_refresh.clone(),
        claim: claim_3.clone(),
        authority: PostgresRuntimeAuthority,
        shown_steps: local_product::runtime::ShownSteps::new(()),
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
    let reason_1 = AttemptFailure::of(&err_1, &cancel_1);
    assert_eq!(reason_1, AttemptFailure::Interrupted);
    store
        .settle_practice_job_attempt(
            &claim_1,
            reason_1,
            local_product::runtime::delivery::Backoff::IMMEDIATE,
        )
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
            None,
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
        is_chat: bool,
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
            let usage = harness::engine::recipe::Usage {
                input_tokens: Some(420),
                output_tokens: Some(55),
                cached_input_tokens: Some(100),
            };
            if self.is_chat {
                Ok(chat_finish_turn(
                    "finish-counting",
                    self.answer,
                    "Seeded chart context.",
                    json!([]),
                    true,
                    json!({ "paid": true }),
                    usage,
                ))
            } else {
                Ok(Turn {
                    items: vec![Item::assistant(self.answer)],
                    receipt: json!({ "paid": true }),
                    usage,
                })
            }
        }
    }

    let mut fault_inrun = FaultInjectingProduct::new(
        PhysicianHost {
            store: store.clone(),
            claim: claim_inrun,
            authority: PostgresRuntimeAuthority,
            shown_steps: local_product::runtime::ShownSteps::new(()),
        },
        PersistenceFault::RetryableOnProviderReturned { remaining: 2 },
    );
    work_inrun
        .run(
            &mut CountingAnswerProvider {
                calls: inrun_provider_calls.clone(),
                answer: "Blood pressure was 148/92 mmHg on 2026-10-15.",
                is_chat: true,
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
            None,
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
            shown_steps: local_product::runtime::ShownSteps::new(()),
        },
        PersistenceFault::FailCandidatePersist,
    );
    let err_cross_1 = work_cross_1
        .run(
            &mut CountingAnswerProvider {
                calls: cross_provider_calls.clone(),
                answer: "Serum potassium was 4.3 mEq/L on 2026-10-15.",
                is_chat: true,
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
            AttemptFailure::of(&err_cross_1, &Cancellation::default()),
            local_product::runtime::delivery::Backoff::IMMEDIATE,
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
        shown_steps: local_product::runtime::ShownSteps::new(()),
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
                is_chat: false,
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
            AttemptFailure::of(&job_err_1, &Cancellation::default()),
            local_product::runtime::delivery::Backoff::IMMEDIATE,
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
            None,
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
                            name: "add_patient_memory_note".into(),
                            arguments: json!({
                                "text": "Preferred pharmacy: Uses Mission St Pharmacy.",
                                "quote": null,
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
                    Ok(chat_finish_turn(
                        "finish-eff-reply",
                        "Recorded concern and pharmacy note.",
                        "Clinic EHR note rev-1 and physician input.",
                        json!([]),
                        true,
                        json!({ "round": 2 }),
                        Default::default(),
                    ))
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
            shown_steps: local_product::runtime::ShownSteps::new(()),
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
            AttemptFailure::of(&err_1, &Cancellation::default()),
            local_product::runtime::delivery::Backoff::IMMEDIATE,
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
        shown_steps: local_product::runtime::ShownSteps::new(()),
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
                None,
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
