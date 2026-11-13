//! Real PostgreSQL integration tests for the Today and Updates read model,
//! per-physician seen state, live panel authorization, practice-wide source
//! outage collapsing, unhandled-work collapsing and handoff, and partial-failure
//! completeness rules.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex as StdMutex, Weak};

use harness::engine::AttemptFailure;
use local_product::ModelRoles;
use local_product::initial_understanding::SummaryClaimInput;
use local_product::patient_reviews::{
    QuietReviewReason, ReviewCitation, ReviewOutcome, ReviewTrigger, UnfinishedReviewWork,
};
use local_product::physician_chat::PhysicianSkills;
use local_product::physician_sources::{
    MedicalRecordNumber, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
use local_product::runtime::chat_operations::{RecordLimit, RecordLimitKind};
use local_product::runtime::concerns::{
    CommandKey, ConcernId, ConcernUrgency, ExactRecordReference, RaiseConcern, RetireConcern,
    RetirementReason, ReviseConcern, SourceWork,
};
use local_product::runtime::jobs::{Assignment, PracticeScope};
use local_product::runtime::refresh::{
    FixedCheckStep, FixedSourceCheck, RefreshCommandKey, RefreshRequest, RefreshTarget,
    SourceCheckOutcome, SourceId,
};
use local_product::runtime::reviews::{ReviewHoldReason, ReviewLifecycleState};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::today_updates::{
    ClearItemDoneRequest, OpenItemRequest, ProducerKind, ReadCompleteness, RecognizableRunKind,
    SeenItemKey, SeenStanding, TodayDoneResult, TodayRowPayload, TodayUpdatesError,
    TodayUpdatesProbe, TodayVisitSlot, UnhandledWorkKind, UpdateEntryKind,
};
use local_product::runtime::understanding::{PatientSummary, SummaryCitation, SummarySegment};
use local_product::runtime::{Authority, Input, PublicationTransaction, Scope, Store, async_trait};
use local_product::sha256_hex;
use local_product::visit_preparation::VisitPreparationDecision;
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use sea_orm::sea_query::{Alias, Expr, ExprTrait, OnConflict, Query};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection};
use serde_json::json;
use uuid::Uuid;

/// Clinical reference time at `15:00:00Z` of the current UTC calendar day so
/// 12-hour physician sessions stay valid against Postgres `clock_timestamp()`
/// while intra-day appointment offsets share one UTC day.
fn base_now_unix() -> u64 {
    let real_now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock after epoch")
        .as_secs();
    (real_now / 86_400) * 86_400 + 15 * 3_600
}
const DAY_SECONDS: i64 = 86_400;

struct LocalCluster {
    cluster_dir: PathBuf,
    data_dir: PathBuf,
    admin_url: String,
}

impl Drop for LocalCluster {
    fn drop(&mut self) {
        let _ = Command::new("pg_ctl")
            .args([
                "-D",
                self.data_dir.to_str().unwrap_or_default(),
                "-m",
                "immediate",
                "stop",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        let _ = std::fs::remove_dir_all(&self.cluster_dir);
    }
}

static LOCAL_CLUSTER: StdMutex<Weak<LocalCluster>> = StdMutex::new(Weak::new());
static MIGRATE_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

fn acquire_database() -> Option<(String, Option<Arc<LocalCluster>>)> {
    for key in [
        "PHYSICIAN_RUNTIME_TEST_DATABASE_URL",
        "PHYSICIAN_DATABASE_URL",
        "PHYSICIAN_TEST_DATABASE_URL",
    ] {
        if let Ok(url) = std::env::var(key) {
            let trimmed = url.trim().to_owned();
            if !trimmed.is_empty() {
                return Some((trimmed, None));
            }
        }
    }

    let mut slot = LOCAL_CLUSTER
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(existing) = slot.upgrade() {
        return Some((existing.admin_url.clone(), Some(existing)));
    }

    let port = {
        use std::net::TcpListener;
        (0..500)
            .find_map(|_| {
                let candidate = 22_000 + (Uuid::new_v4().as_u128() % 9_000) as u16;
                TcpListener::bind(("127.0.0.1", candidate))
                    .ok()
                    .map(|_| candidate)
            })
            .expect("free loopback port for local postgres")
    };
    let cluster_dir =
        std::env::temp_dir().join(format!("coda-tu-pg-{}-{}", std::process::id(), port));
    let _ = std::fs::remove_dir_all(&cluster_dir);
    let data_dir = cluster_dir.join("data");
    let sock_dir = cluster_dir.join("sock");
    std::fs::create_dir_all(&data_dir).ok()?;
    std::fs::create_dir_all(&sock_dir).ok()?;

    let init = Command::new("initdb")
        .args([
            "-D",
            data_dir.to_str()?,
            "-U",
            "postgres",
            "-A",
            "trust",
            "--no-locale",
            "-E",
            "UTF8",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok()?;
    if !init.success() {
        let _ = std::fs::remove_dir_all(&cluster_dir);
        return None;
    }

    let log_file = cluster_dir.join("postgres.log");
    let opts = format!("-F -p {port} -h 127.0.0.1 -k {}", sock_dir.to_str()?);
    let start = Command::new("pg_ctl")
        .args([
            "-D",
            data_dir.to_str()?,
            "-l",
            log_file.to_str()?,
            "-o",
            &opts,
            "-w",
            "start",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok()?;
    if !start.success() {
        let _ = std::fs::remove_dir_all(&cluster_dir);
        return None;
    }

    let cluster = Arc::new(LocalCluster {
        cluster_dir,
        data_dir,
        admin_url: format!("postgresql://postgres@127.0.0.1:{port}/postgres"),
    });
    *slot = Arc::downgrade(&cluster);
    Some((cluster.admin_url.clone(), Some(cluster)))
}

async fn connect_stores() -> Option<(
    Store,
    DatabaseConnection,
    Arc<PostgresAuthStore>,
    String,
    Option<Arc<LocalCluster>>,
)> {
    let Some((url, cluster)) = acquire_database() else {
        assert!(
            std::env::var("CODA_REQUIRE_POSTGRES_TESTS").as_deref() != Ok("1"),
            "CODA_REQUIRE_POSTGRES_TESTS=1 requires PHYSICIAN_RUNTIME_TEST_DATABASE_URL"
        );
        return None;
    };
    let _guard = MIGRATE_LOCK.lock().await;
    let store = Store::connect(&url).await.expect("connect runtime store");
    store.migrate().await.expect("migrate runtime store");
    let db = Database::connect(&url).await.expect("connect sea_orm db");
    let auth = Arc::new(
        PostgresAuthStore::connect_with_migrations(&url)
            .await
            .expect("connect and migrate physician auth store"),
    );
    drop(_guard);
    Some((store, db, auth, url, cluster))
}

struct PostgresWorkspaceAuthority;

#[async_trait]
impl Authority for PostgresWorkspaceAuthority {
    async fn check(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        PostgresAuthStore::check_runtime_authority(
            tx,
            session_hash.ok_or("missing session reference")?,
            &PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|cause| cause.to_string())
    }

    async fn lock_publication(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        PostgresAuthStore::lock_runtime_authority(
            tx,
            session_hash.ok_or("missing session reference")?,
            &PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|cause| cause.to_string())
    }
}

async fn invite_and_sign_in(
    auth: &PostgresAuthStore,
    practice_id: &str,
    label: &str,
    now_unix: u64,
) -> (PhysicianIdentity, String) {
    let email = format!("{label}-{}@codahealth.test", Uuid::new_v4().simple());
    let invite = auth
        .invite(practice_id, &email, label, true, now_unix)
        .await
        .expect("invite physician");
    auth.redeem_invitation(&email, &invite, now_unix)
        .await
        .expect("redeem invitation")
}

fn jsonb_expr(value: &serde_json::Value) -> sea_orm::sea_query::SimpleExpr {
    Expr::val(serde_json::to_string(value).expect("serialize jsonb")).cast_as(Alias::new("jsonb"))
}

async fn exec_query(db: &DatabaseConnection, stmt: impl sea_orm::StatementBuilder) {
    db.execute(&stmt).await.expect("execute test setup query");
}

fn test_model_roles() -> ModelRoles {
    local_product::model_roles(
        "test",
        &[
            ("CODA_CHAT_MODEL", "gpt-5.4"),
            ("CODA_CHAT_EFFORT", "medium"),
            ("CODA_CHAT_CONTEXT_WINDOW_TOKENS", "128000"),
            ("CODA_CHAT_INPUT_LIMIT_TOKENS", "64000"),
            ("CODA_BACKGROUND_MODEL", "gpt-5.4"),
            ("CODA_BACKGROUND_EFFORT", "medium"),
            ("CODA_BACKGROUND_CONTEXT_WINDOW_TOKENS", "128000"),
            ("CODA_BACKGROUND_INPUT_LIMIT_TOKENS", "64000"),
        ],
    )
    .expect("valid test model roles")
}

fn make_source_patient(workspace: &str, patient_name: &str) -> SourcePatient {
    SourcePatient {
        source_patient_id: workspace.to_owned(),
        display_name: patient_name.to_owned(),
        birth_date: Some("1968-05-14".to_owned()),
        sex_or_gender: Some("female".to_owned()),
        medical_record_numbers: vec![MedicalRecordNumber {
            system: "MRN".to_owned(),
            value: format!("MRN-{workspace}"),
        }],
    }
}

async fn seed_chart_record(
    store: &Store,
    workspace: &str,
    patient_name: &str,
    candidate_id: &str,
    revision: &str,
    excerpt: &str,
    pulled_at: i64,
) -> ExactRecordReference {
    seed_chart_record_full(
        store,
        workspace,
        patient_name,
        candidate_id,
        revision,
        vec![ReadablePart {
            label: "Result summary".to_owned(),
            text: excerpt.to_owned(),
        }],
        json!({"system": "regional-ehr", "id": candidate_id}),
        pulled_at,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
async fn seed_chart_record_full(
    store: &Store,
    workspace: &str,
    patient_name: &str,
    candidate_id: &str,
    revision: &str,
    readable_parts: Vec<ReadablePart>,
    provenance: serde_json::Value,
    pulled_at: i64,
) -> ExactRecordReference {
    let combined_text = readable_parts
        .iter()
        .map(|part| part.text.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let content = json!({
        "resourceType": "Observation",
        "id": candidate_id,
        "note": combined_text,
    })
    .to_string();
    let pull = SourcePull {
        source_id: "regional-ehr".to_owned(),
        source_label: "Regional EHR".to_owned(),
        patient: make_source_patient(workspace, patient_name),
        pulled_at: Some(pulled_at),
    };
    let rev = SourceRevision {
        workspace: workspace.to_owned(),
        source_id: "regional-ehr".to_owned(),
        candidate_id: candidate_id.to_owned(),
        revision: revision.to_owned(),
        record_type: "Laboratory result".to_owned(),
        dates: vec![RecordDate {
            name: "effectiveDate".to_owned(),
            value: "2026-11-12".to_owned(),
        }],
        sha256: sha256_hex(content.as_bytes()),
        content,
        readable_parts,
        provenance,
        envelope: None,
    };
    store
        .record_chart(workspace, &[rev], &pull)
        .await
        .expect("record chart");
    ExactRecordReference::new(candidate_id, revision).expect("valid ref")
}

async fn ensure_scheduled_appointment(
    db: &DatabaseConnection,
    practice: &str,
    workspace: &str,
    slot: &TodayVisitSlot,
    updated_at: i64,
) {
    let content = "{}";
    let mut appt = Query::insert();
    appt.into_table(Alias::new("physician_runtime_appointments"))
        .columns([
            Alias::new("practice"),
            Alias::new("source_id"),
            Alias::new("appointment_id"),
            Alias::new("saved_version"),
            Alias::new("source_revision"),
            Alias::new("starts_at"),
            Alias::new("starts_at_unix"),
            Alias::new("ends_at"),
            Alias::new("ends_at_unix"),
            Alias::new("status"),
            Alias::new("source_status"),
            Alias::new("label"),
            Alias::new("match_state"),
            Alias::new("workspace"),
            Alias::new("source_patient_id"),
            Alias::new("dates"),
            Alias::new("content"),
            Alias::new("sha256"),
            Alias::new("provenance"),
            Alias::new("as_of"),
            Alias::new("updated_at"),
        ])
        .values_panic([
            practice.into(),
            slot.source_id.as_str().into(),
            slot.appointment_id.as_str().into(),
            1_i64.into(),
            "rev-1".into(),
            "2026-11-12T16:00:00Z".into(),
            slot.starts_at_unix.into(),
            "2026-11-12T16:30:00Z".into(),
            slot.ends_at_unix.into(),
            "scheduled".into(),
            "booked".into(),
            "Follow-up visit".into(),
            "matched".into(),
            workspace.into(),
            workspace.into(),
            jsonb_expr(&json!([])),
            content.into(),
            sha256_hex(content.as_bytes()).into(),
            jsonb_expr(&json!({"system": slot.source_id})),
            updated_at.into(),
            updated_at.into(),
        ])
        .on_conflict(
            OnConflict::columns([
                Alias::new("practice"),
                Alias::new("source_id"),
                Alias::new("appointment_id"),
            ])
            .update_columns([
                Alias::new("starts_at_unix"),
                Alias::new("ends_at_unix"),
                Alias::new("status"),
                Alias::new("workspace"),
                Alias::new("updated_at"),
            ])
            .to_owned(),
        );
    exec_query(db, appt).await;
}

enum TestConcernStanding {
    Open,
    Retired { reason: String, retired_at: i64 },
}

struct TestConcern {
    practice: String,
    workspace: String,
    concern_id: String,
    urgency: ConcernUrgency,
    standing: TestConcernStanding,
    reason: String,
    records: Vec<ExactRecordReference>,
    source_work: SourceWork,
    created_at: i64,
    updated_at: i64,
    visit_slot: Option<TodayVisitSlot>,
}

async fn seed_concern(store: &Store, db: &DatabaseConnection, spec: TestConcern) {
    let id = ConcernId::new(&spec.concern_id).expect("valid concern id");
    let raise_key =
        CommandKey::new(format!("test-raise:{}:{}", spec.workspace, spec.concern_id)).unwrap();
    store
        .raise_concern(
            &raise_key,
            &RaiseConcern {
                practice: spec.practice.clone(),
                workspace: spec.workspace.clone(),
                id: id.clone(),
                urgency: spec.urgency,
                reason: spec.reason.clone(),
                records: spec.records.clone(),
                source_work: spec.source_work.clone(),
            },
        )
        .await
        .expect("raise concern through Store::raise_concern");

    let mut retired_at_val: Option<i64> = None;
    if let TestConcernStanding::Retired { reason, retired_at } = &spec.standing {
        let retire_key = CommandKey::new(format!(
            "test-retire:{}:{}",
            spec.workspace, spec.concern_id
        ))
        .unwrap();
        store
            .retire_concern(
                &retire_key,
                &RetireConcern {
                    practice: spec.practice.clone(),
                    workspace: spec.workspace.clone(),
                    id: id.clone(),
                    reason: RetirementReason::new(reason.clone()).expect("valid retirement reason"),
                    source_work: spec.source_work.clone(),
                },
            )
            .await
            .expect("retire concern through Store::retire_concern");
        retired_at_val = Some(*retired_at);
    }

    let mut update = Query::update();
    update
        .table(Alias::new("physician_runtime_concerns"))
        .value(Alias::new("created_at"), spec.created_at)
        .value(Alias::new("updated_at"), spec.updated_at)
        .and_where(Expr::col(Alias::new("practice")).eq(spec.practice.as_str()))
        .and_where(Expr::col(Alias::new("workspace")).eq(spec.workspace.as_str()))
        .and_where(Expr::col(Alias::new("id")).eq(spec.concern_id.as_str()));
    if let Some(ts) = retired_at_val {
        update.value(Alias::new("retired_at"), ts);
    }
    exec_query(db, update).await;

    if let Some(slot) = spec.visit_slot {
        ensure_scheduled_appointment(db, &spec.practice, &spec.workspace, &slot, spec.updated_at)
            .await;
        let mut insert = Query::insert();
        insert
            .into_table(Alias::new("physician_runtime_visit_concerns"))
            .columns([
                Alias::new("practice"),
                Alias::new("workspace"),
                Alias::new("concern_id"),
                Alias::new("source_id"),
                Alias::new("appointment_id"),
                Alias::new("starts_at_unix"),
                Alias::new("ends_at_unix"),
                Alias::new("urgency"),
                Alias::new("created_at"),
                Alias::new("updated_at"),
            ])
            .values_panic([
                spec.practice.into(),
                spec.workspace.into(),
                spec.concern_id.into(),
                slot.source_id.into(),
                slot.appointment_id.into(),
                slot.starts_at_unix.into(),
                slot.ends_at_unix.into(),
                spec.urgency.as_str().into(),
                spec.created_at.into(),
                spec.updated_at.into(),
            ])
            .on_conflict(
                OnConflict::columns([
                    Alias::new("practice"),
                    Alias::new("workspace"),
                    Alias::new("concern_id"),
                ])
                .update_columns([
                    Alias::new("source_id"),
                    Alias::new("appointment_id"),
                    Alias::new("starts_at_unix"),
                    Alias::new("ends_at_unix"),
                    Alias::new("urgency"),
                    Alias::new("updated_at"),
                ])
                .to_owned(),
            );
        exec_query(db, insert).await;
    }
}

#[allow(clippy::too_many_arguments)]
async fn revise_concern_at(
    store: &Store,
    db: &DatabaseConnection,
    practice: &str,
    workspace: &str,
    concern_id: &str,
    urgency: ConcernUrgency,
    reason: &str,
    records: Vec<ExactRecordReference>,
    updated_at: i64,
) {
    let key = CommandKey::new(format!("test-revise:{workspace}:{concern_id}:{updated_at}"))
        .expect("valid revise key");
    store
        .revise_concern(
            &key,
            &ReviseConcern {
                practice: practice.to_owned(),
                workspace: workspace.to_owned(),
                id: ConcernId::new(concern_id).expect("valid concern id"),
                urgency,
                reason: reason.to_owned(),
                records,
                source_work: SourceWork::new("patient_review", "work-revise", "att-revise")
                    .unwrap(),
            },
        )
        .await
        .expect("revise concern through Store::revise_concern");

    let mut update = Query::update();
    update
        .table(Alias::new("physician_runtime_concerns"))
        .value(Alias::new("updated_at"), updated_at)
        .and_where(Expr::col(Alias::new("practice")).eq(practice))
        .and_where(Expr::col(Alias::new("workspace")).eq(workspace))
        .and_where(Expr::col(Alias::new("id")).eq(concern_id));
    exec_query(db, update).await;
}

struct TestReview {
    practice: String,
    workspace: String,
    review_id: String,
    triggers: Vec<ReviewTrigger>,
    unfinished_work: Vec<UnfinishedReviewWork>,
    state: ReviewLifecycleState,
    evidence_read: Vec<ExactRecordReference>,
    queued_at: i64,
    updated_at: i64,
}

async fn seed_review(db: &DatabaseConnection, spec: TestReview) {
    let status = spec.state.status_str();
    let completed_at = match &spec.state {
        ReviewLifecycleState::Completed { completed_at, .. } => Some(*completed_at),
        _ => None,
    };
    let completed_source_version = match &spec.state {
        ReviewLifecycleState::Completed {
            completed_source_version,
            ..
        } => Some(i64::try_from(*completed_source_version).unwrap_or(1)),
        _ => None,
    };
    let started_at = match &spec.state {
        ReviewLifecycleState::Running { started_at, .. } => Some(*started_at),
        ReviewLifecycleState::Completed { completed_at, .. } => Some(*completed_at - 10),
        ReviewLifecycleState::Held { held_at, .. } => Some(*held_at - 10),
        ReviewLifecycleState::Failed { failed_at, .. } => Some(*failed_at - 10),
        ReviewLifecycleState::Pending | ReviewLifecycleState::Superseded { .. } => None,
    };
    let recipe_version = json!({
        "model": "gpt-5.4",
        "skill_sha256": "deadbeef",
        "instructions_sha256": "cafebabe"
    });
    let usage = json!({"kind": "unknown"});
    let mut insert = Query::insert();
    insert
        .into_table(Alias::new("physician_runtime_patient_reviews"))
        .columns([
            Alias::new("practice"),
            Alias::new("workspace"),
            Alias::new("review_id"),
            Alias::new("command_key"),
            Alias::new("job_id"),
            Alias::new("work_id"),
            Alias::new("status"),
            Alias::new("triggers"),
            Alias::new("accepted_inputs"),
            Alias::new("recipe_version"),
            Alias::new("initial_source_version"),
            Alias::new("initial_sources"),
            Alias::new("completed_source_version"),
            Alias::new("evidence_read"),
            Alias::new("actions"),
            Alias::new("unfinished_work"),
            Alias::new("state"),
            Alias::new("usage"),
            Alias::new("queued_at"),
            Alias::new("started_at"),
            Alias::new("completed_at"),
            Alias::new("updated_at"),
        ])
        .values_panic([
            spec.practice.into(),
            spec.workspace.into(),
            spec.review_id.clone().into(),
            format!("cmd-rev-{}", spec.review_id).into(),
            format!("job-rev-{}", spec.review_id).into(),
            format!("work-rev-{}", spec.review_id).into(),
            status.into(),
            jsonb_expr(&serde_json::to_value(&spec.triggers).unwrap()),
            jsonb_expr(&json!([])),
            jsonb_expr(&recipe_version),
            1_i64.into(),
            jsonb_expr(&json!([])),
            completed_source_version.into(),
            jsonb_expr(&serde_json::to_value(&spec.evidence_read).unwrap()),
            jsonb_expr(&json!([])),
            jsonb_expr(&serde_json::to_value(&spec.unfinished_work).unwrap()),
            jsonb_expr(&serde_json::to_value(&spec.state).unwrap()),
            jsonb_expr(&usage),
            spec.queued_at.into(),
            started_at.into(),
            completed_at.into(),
            spec.updated_at.into(),
        ]);
    exec_query(db, insert).await;
}

enum TestVisitPrepState {
    CouldNotFinish { reason: String },
    Skipped { reason: String },
}

struct TestVisitPreparation {
    practice: String,
    workspace: String,
    source_id: String,
    appointment_id: String,
    starts_at_unix: i64,
    ends_at_unix: i64,
    state: TestVisitPrepState,
    updated_at: i64,
}

async fn seed_visit_preparation(db: &DatabaseConnection, spec: TestVisitPreparation) {
    let (state_str, decision, failed_reason, briefing_job_id) = match spec.state {
        TestVisitPrepState::CouldNotFinish { reason } => (
            "could_not_finish",
            VisitPreparationDecision::PrepareNow {
                reason: "Upcoming clinical visit requires chart synthesis.".to_owned(),
            },
            Some(reason),
            Some(format!("job-vp-{}", spec.appointment_id)),
        ),
        TestVisitPrepState::Skipped { reason } => (
            "skipped",
            VisitPreparationDecision::Skip { reason },
            None,
            None,
        ),
    };
    let mut insert = Query::insert();
    insert
        .into_table(Alias::new("physician_runtime_visit_preparations"))
        .columns([
            Alias::new("practice"),
            Alias::new("source_id"),
            Alias::new("appointment_id"),
            Alias::new("starts_at_unix"),
            Alias::new("workspace"),
            Alias::new("ends_at_unix"),
            Alias::new("appointment_saved_version"),
            Alias::new("observed_source_version"),
            Alias::new("decision"),
            Alias::new("decided_at"),
            Alias::new("selection_job_id"),
            Alias::new("selection_work_id"),
            Alias::new("selection_idempotency_key"),
            Alias::new("state"),
            Alias::new("briefing_job_id"),
            Alias::new("failed_reason"),
            Alias::new("updated_at"),
        ])
        .values_panic([
            spec.practice.into(),
            spec.source_id.into(),
            spec.appointment_id.clone().into(),
            spec.starts_at_unix.into(),
            spec.workspace.into(),
            spec.ends_at_unix.into(),
            1_i64.into(),
            1_i64.into(),
            jsonb_expr(&serde_json::to_value(&decision).unwrap()),
            spec.updated_at.into(),
            format!("sel-job-{}", spec.appointment_id).into(),
            format!("sel-work-{}", spec.appointment_id).into(),
            format!("sel-key-{}", spec.appointment_id).into(),
            state_str.into(),
            briefing_job_id.into(),
            failed_reason.into(),
            spec.updated_at.into(),
        ]);
    exec_query(db, insert).await;
}

struct TestVisitBriefing {
    practice: String,
    workspace: String,
    source_id: String,
    appointment_id: String,
    starts_at_unix: i64,
    ends_at_unix: i64,
    briefing_id: String,
    orientation: String,
    narrative: String,
    basis_and_limitations: String,
    citations: Vec<SummaryCitation>,
    prepared_at: i64,
    source_version: i64,
}

async fn seed_visit_briefing(db: &DatabaseConnection, spec: TestVisitBriefing) {
    ensure_scheduled_appointment(
        db,
        &spec.practice,
        &spec.workspace,
        &TodayVisitSlot {
            source_id: spec.source_id.clone(),
            appointment_id: spec.appointment_id.clone(),
            starts_at_unix: spec.starts_at_unix,
            ends_at_unix: spec.ends_at_unix,
        },
        spec.prepared_at,
    )
    .await;
    let origin = json!({
        "kind": "practice_run",
        "job_id": format!("job-{}", spec.briefing_id),
        "work_id": format!("work-{}", spec.briefing_id),
        "attempt_id": "att-1"
    });
    let mut insert = Query::insert();
    insert
        .into_table(Alias::new("physician_runtime_visit_briefings"))
        .columns([
            Alias::new("practice"),
            Alias::new("source_id"),
            Alias::new("appointment_id"),
            Alias::new("starts_at_unix"),
            Alias::new("briefing_id"),
            Alias::new("workspace"),
            Alias::new("idempotency_key"),
            Alias::new("starts_at"),
            Alias::new("ends_at"),
            Alias::new("ends_at_unix"),
            Alias::new("appointment_saved_version"),
            Alias::new("prepared_at"),
            Alias::new("source_version"),
            Alias::new("source_versions"),
            Alias::new("source_freshness"),
            Alias::new("limits"),
            Alias::new("visit_card_orientation"),
            Alias::new("trajectory"),
            Alias::new("meaningful_changes"),
            Alias::new("unresolved_issues"),
            Alias::new("visit_decisions"),
            Alias::new("basis_and_limitations"),
            Alias::new("basis_citations"),
            Alias::new("citations"),
            Alias::new("narrative"),
            Alias::new("visit_standing"),
            Alias::new("origin"),
            Alias::new("updated_at"),
        ])
        .values_panic([
            spec.practice.into(),
            spec.source_id.into(),
            spec.appointment_id.into(),
            spec.starts_at_unix.into(),
            spec.briefing_id.clone().into(),
            spec.workspace.into(),
            format!("idem-{}", spec.briefing_id).into(),
            "2026-11-12T16:00:00Z".into(),
            "2026-11-12T16:30:00Z".into(),
            spec.ends_at_unix.into(),
            1_i64.into(),
            spec.prepared_at.into(),
            spec.source_version.into(),
            jsonb_expr(&json!({})),
            jsonb_expr(&json!([])),
            jsonb_expr(&json!([])),
            spec.orientation.into(),
            jsonb_expr(&json!([])),
            jsonb_expr(&json!([])),
            jsonb_expr(&json!([])),
            jsonb_expr(&json!([])),
            spec.basis_and_limitations.into(),
            jsonb_expr(&json!([])),
            jsonb_expr(&serde_json::to_value(&spec.citations).unwrap()),
            spec.narrative.into(),
            "current".into(),
            jsonb_expr(&origin),
            spec.prepared_at.into(),
        ]);
    exec_query(db, insert).await;
}

async fn seed_patient_summary(
    db: &DatabaseConnection,
    practice: &str,
    workspace: &str,
    revision: i64,
    source_version: i64,
    summary: &PatientSummary,
    updated_at: i64,
) {
    let operating_index = json!({
        "sentinel": "INTERNAL-OPERATING-INDEX-NEVER-SHOWN",
        "clinical_snapshot": "Private operating index only for Coda runs."
    });
    let origin = json!({
        "kind": "practice_run",
        "job_id": format!("job-understanding-{workspace}-{revision}"),
        "work_id": format!("work-understanding-{workspace}-{revision}"),
        "attempt_id": "att-1"
    });
    let mut insert = Query::insert();
    insert
        .into_table(Alias::new("physician_runtime_patient_understanding"))
        .columns([
            Alias::new("practice"),
            Alias::new("workspace"),
            Alias::new("revision"),
            Alias::new("source_version"),
            Alias::new("evidence_scope"),
            Alias::new("operating_index"),
            Alias::new("summary"),
            Alias::new("origin"),
            Alias::new("updated_at"),
        ])
        .values_panic([
            practice.into(),
            workspace.into(),
            revision.into(),
            source_version.into(),
            jsonb_expr(&json!({"sources": []})),
            jsonb_expr(&operating_index),
            jsonb_expr(&serde_json::to_value(summary).unwrap()),
            jsonb_expr(&origin),
            updated_at.into(),
        ]);
    exec_query(db, insert).await;
}

async fn seed_failed_initial_understanding(
    store: &Store,
    db: &DatabaseConnection,
    auth: &PostgresAuthStore,
    practice: &str,
    workspace: &str,
    job_id: &str,
    failed_at: i64,
) {
    let skills = PhysicianSkills::configured(None, false).expect("load workspace skills");
    let assignment = Assignment::synthetic_patient_stub(workspace, &skills).expect("stub");
    let scope = PracticeScope {
        practice: practice.to_owned(),
        workspace: workspace.to_owned(),
        job_id: job_id.to_owned(),
    };
    store
        .accept_practice_job(&scope, &assignment, &test_model_roles(), &skills, auth)
        .await
        .expect("accept practice job");
    while let Some(claim) = store
        .claim_practice_job(&scope, 60, auth)
        .await
        .expect("claim practice job")
    {
        store
            .settle_practice_job_attempt(
                &claim,
                AttemptFailure::Failed,
                local_product::runtime::delivery::Backoff::IMMEDIATE,
            )
            .await
            .expect("settle failed practice job attempt");
    }

    let mut update = Query::update();
    update
        .table(Alias::new("physician_runtime_practice_jobs"))
        .value(Alias::new("created_at"), failed_at - 30)
        .value(Alias::new("updated_at"), failed_at)
        .and_where(Expr::col(Alias::new("practice")).eq(practice))
        .and_where(Expr::col(Alias::new("workspace")).eq(workspace))
        .and_where(Expr::col(Alias::new("job_id")).eq(job_id));
    exec_query(db, update).await;
}

async fn seed_running_conversation(
    store: &Store,
    db: &DatabaseConnection,
    scope: &Scope,
    question: &str,
    activity_at: i64,
) {
    let skills = PhysicianSkills::configured(None, false).expect("load workspace skills");
    let roles = test_model_roles();
    let recipe = store
        .chat_recipe_with_understanding(
            &roles.chat,
            "Answer clinical questions from chart evidence.",
            &skills,
            &scope.practice,
            &scope.workspace,
        )
        .await
        .expect("build chat recipe");
    store
        .accept_authorized(
            scope,
            &Input {
                key: format!("chat-input-{}", scope.chat),
                text: question.to_owned(),
                timezone: "America/New_York".to_owned(),
                scenario: None,
            },
            vec![7_u8; 32],
            &skills,
            &recipe,
            None,
        )
        .await
        .expect("accept running chat");

    let mut select = Query::select();
    select
        .column(Alias::new("state"))
        .from(Alias::new("physician_runtime_conversations"))
        .and_where(Expr::col(Alias::new("practice")).eq(scope.practice.as_str()))
        .and_where(Expr::col(Alias::new("physician")).eq(scope.physician.as_str()))
        .and_where(Expr::col(Alias::new("workspace")).eq(scope.workspace.as_str()))
        .and_where(Expr::col(Alias::new("chat")).eq(scope.chat.as_str()));
    let row = db
        .query_one(&select)
        .await
        .expect("query seeded conversation state")
        .expect("seeded conversation row");
    let mut state_val: serde_json::Value = row
        .try_get("", "state")
        .expect("decode conversation state jsonb");
    if let Some(obj) = state_val.as_object_mut() {
        obj.insert("opened_at".to_owned(), json!(activity_at));
        obj.insert("last_activity_at".to_owned(), json!(activity_at));
    }
    let mut update = Query::update();
    update
        .table(Alias::new("physician_runtime_conversations"))
        .value(Alias::new("state"), jsonb_expr(&state_val))
        .value(Alias::new("last_activity_at"), Expr::val(activity_at))
        .and_where(Expr::col(Alias::new("practice")).eq(scope.practice.as_str()))
        .and_where(Expr::col(Alias::new("physician")).eq(scope.physician.as_str()))
        .and_where(Expr::col(Alias::new("workspace")).eq(scope.workspace.as_str()))
        .and_where(Expr::col(Alias::new("chat")).eq(scope.chat.as_str()));
    exec_query(db, update).await;
}

struct ForcedFaultProbe {
    fail_producers: BTreeSet<ProducerKind>,
    fail_open_keys: BTreeSet<String>,
}

impl TodayUpdatesProbe for ForcedFaultProbe {
    fn check_producer(&self, producer: ProducerKind) -> Result<(), String> {
        if self.fail_producers.contains(&producer) {
            return Err(format!("simulated {} storage timeout", producer.label()));
        }
        Ok(())
    }

    fn check_open_item(&self, _workspace: &str, item_key: &str) -> Result<(), String> {
        if self.fail_open_keys.contains(item_key) {
            return Err("simulated detail query timeout".to_owned());
        }
        Ok(())
    }
}

#[tokio::test]
async fn ac1_every_producer_kind_reaches_the_right_list_collapses_unhandled_per_patient_and_hands_off_on_done()
 {
    let Some((store, db, auth, _url, _cluster)) = connect_stores().await else {
        return;
    };

    let now_u64 = base_now_unix();
    let now_i64 = now_u64 as i64;
    let practice = format!("prac-ac1-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_u64))
        .await
        .unwrap();

    let (dr, token) = invite_and_sign_in(&auth, &practice, "Dr. Rivera", now_u64).await;
    let ws_now = format!("pw-now-{}", Uuid::new_v4().simple());
    let ws_nudge = format!("pw-nudge-{}", Uuid::new_v4().simple());
    let ws_quiet = format!("pw-quiet-{}", Uuid::new_v4().simple());
    let ws_unhandled_rev = format!("pw-unh-rev-{}", Uuid::new_v4().simple());
    let ws_unhandled_limit = format!("pw-unh-lim-{}", Uuid::new_v4().simple());
    let ws_unhandled_visit = format!("pw-unh-vis-{}", Uuid::new_v4().simple());
    let ws_unhandled_summary = format!("pw-unh-sum-{}", Uuid::new_v4().simple());

    for ws in [
        &ws_now,
        &ws_nudge,
        &ws_quiet,
        &ws_unhandled_rev,
        &ws_unhandled_limit,
        &ws_unhandled_visit,
        &ws_unhandled_summary,
    ] {
        auth.register_patient(&practice, ws).await.unwrap();
        auth.set_membership(&dr, ws, Membership::Allowed)
            .await
            .unwrap();
    }

    // Multi-section >500-char chart record on `ws_now` with `external_url` and a generic
    // `"attachment"` structural part label to verify full readable body + preserved `native_uri`.
    let long_bmp_section = "Serum potassium 6.1 mmol/L (critical high; reference range 3.5-5.1 mmol/L) \
        drawn at 06:42 AM from non-hemolyzed peripheral venipuncture. Serum sodium 137 mmol/L, \
        chloride 104 mmol/L, bicarbonate 19 mmol/L, blood urea nitrogen 38 mg/dL, serum creatinine \
        2.1 mg/dL (prior baseline 1.3 mg/dL three months ago), estimated GFR 28 mL/min/1.73m2.";
    let long_plan_tail = "TAIL_MARKER_BEYOND_320_CHARS: Repeat stat venous basic metabolic panel \
        and 12-lead electrocardiogram ordered; hold lisinopril 20 mg and spironolactone 25 mg \
        pending physician review and potassium binder initiation.";
    let ref_now = seed_chart_record_full(
        &store,
        &ws_now,
        "Elena Vance",
        "obs-k-1",
        "rev-1",
        vec![
            ReadablePart {
                label: "attachment".to_owned(),
                text: long_bmp_section.to_owned(),
            },
            ReadablePart {
                label: "Clinical comment".to_owned(),
                text: long_plan_tail.to_owned(),
            },
        ],
        json!({
            "system": "regional-ehr",
            "id": "obs-k-1",
            "title": "Morning Basic Metabolic Panel",
            "external_url": "https://ehr.codahealth.test/chart/obs-k-1"
        }),
        now_i64 - 600,
    )
    .await;

    let ref_nudge = seed_chart_record(
        &store,
        &ws_nudge,
        "Marcus Sterling",
        "obs-a1c-1",
        "rev-1",
        "Hemoglobin A1c 8.7%, up from 7.2% six months ago.",
        now_i64 - 3_600,
    )
    .await;
    let ref_quiet = seed_chart_record(
        &store,
        &ws_quiet,
        "Hannah Cho",
        "obs-bp-1",
        "rev-1",
        "Home blood pressure log averaging 118/74 mmHg on current regimen.",
        now_i64 - 1_800,
    )
    .await;
    let _ref_unhandled_rev = seed_chart_record(
        &store,
        &ws_unhandled_rev,
        "Devon Brooks",
        "obs-renal-1",
        "rev-1",
        "External nephrology consult pending scanned attachment.",
        now_i64 - 8_000,
    )
    .await;
    let _ref_unhandled_limit = seed_chart_record(
        &store,
        &ws_unhandled_limit,
        "claire-bennett",
        "obs-onc-1",
        "rev-1",
        "Multi-hospital oncology flowsheet awaiting synthesis.",
        now_i64 - 7_500,
    )
    .await;
    let _ref_unhandled_visit = seed_chart_record(
        &store,
        &ws_unhandled_visit,
        "Owen Alvarez",
        "obs-cards-1",
        "rev-1",
        "Cardiology stress echo report pending attachment fetch.",
        now_i64 - 7_000,
    )
    .await;
    let _ref_unhandled_summary = seed_chart_record(
        &store,
        &ws_unhandled_summary,
        "Nadia Kim",
        "obs-intake-1",
        "rev-1",
        "Transfer intake packet imported from outside clinic.",
        now_i64 - 6_000,
    )
    .await;

    let origin_work = SourceWork::new("patient_review", "work-origin-1", "attempt-1").unwrap();

    // 1. Open Now concern (waited 5 minutes -> still ranks FIRST ahead of multi-hour Today rows!).
    seed_concern(
        &store,
        &db,
        TestConcern {
            practice: practice.clone(),
            workspace: ws_now.clone(),
            concern_id: "concern-now-k".to_owned(),
            urgency: ConcernUrgency::Now,
            standing: TestConcernStanding::Open,
            reason: "Critical hyperkalemia (K 6.1 mmol/L) on morning BMP.".to_owned(),
            records: vec![ref_now.clone()],
            source_work: origin_work.clone(),
            created_at: now_i64 - 300,
            updated_at: now_i64 - 300,
            visit_slot: None,
        },
    )
    .await;

    // 2. Open Today pre-visit nudge (waited 1 hour) + saved VisitBriefing.
    seed_visit_briefing(
        &db,
        TestVisitBriefing {
            practice: practice.clone(),
            workspace: ws_nudge.clone(),
            source_id: "practice-sched".to_owned(),
            appointment_id: "appt-sterling-1".to_owned(),
            starts_at_unix: now_i64 + 3_600,
            ends_at_unix: now_i64 + 5_400,
            briefing_id: "briefing-sterling-today".to_owned(),
            orientation: "Follow-up for rising A1c (8.7%) ahead of afternoon visit.".to_owned(),
            narrative: "A1c rose from 7.2% to 8.7% over 6 months despite metformin.".to_owned(),
            basis_and_limitations: "Regional EHR labs through 2026-11-12.".to_owned(),
            citations: vec![SummaryCitation {
                reference: ref_nudge.clone(),
                source_id: "regional-ehr".to_owned(),
                source_label: "Regional EHR".to_owned(),
                sha256: "deadbeef".to_owned(),
                display_label: "Hemoglobin A1c".to_owned(),
                as_of: Some("2026-11-12".to_owned()),
                excerpt: "Hemoglobin A1c 8.7%.".to_owned(),
                native_uri: None,
            }],
            prepared_at: now_i64 - 3_600,
            source_version: 1,
        },
    )
    .await;

    seed_concern(
        &store,
        &db,
        TestConcern {
            practice: practice.clone(),
            workspace: ws_nudge.clone(),
            concern_id: "concern-visit-a1c".to_owned(),
            urgency: ConcernUrgency::Today,
            standing: TestConcernStanding::Open,
            reason: "A1c rose to 8.7% ahead of today's diabetes follow-up.".to_owned(),
            records: vec![ref_nudge.clone()],
            source_work: origin_work.clone(),
            created_at: now_i64 - 3_600,
            updated_at: now_i64 - 3_600,
            visit_slot: Some(TodayVisitSlot {
                source_id: "practice-sched".to_owned(),
                appointment_id: "appt-sterling-1".to_owned(),
                starts_at_unix: now_i64 + 3_600,
                ends_at_unix: now_i64 + 5_400,
            }),
        },
    )
    .await;

    // 3. Retired concern via `Store::retire_concern` -> ONLY in Updates, NEVER on Today.
    seed_concern(
        &store,
        &db,
        TestConcern {
            practice: practice.clone(),
            workspace: ws_quiet.clone(),
            concern_id: "concern-retired-bp".to_owned(),
            urgency: ConcernUrgency::Today,
            standing: TestConcernStanding::Retired {
                reason: "Repeat clinic and home blood pressures normalized.".to_owned(),
                retired_at: now_i64 - 900,
            },
            reason: "Borderline elevated office blood pressure last month.".to_owned(),
            records: vec![ref_quiet.clone()],
            source_work: origin_work,
            created_at: now_i64 - 10_000,
            updated_at: now_i64 - 900,
            visit_slot: None,
        },
    )
    .await;

    // 4. Quiet completed review today -> NO Today row; reaches Updates `reviewed_nothing_for_you`.
    seed_review(
        &db,
        TestReview {
            practice: practice.clone(),
            workspace: ws_quiet.clone(),
            review_id: "rev-quiet-today".to_owned(),
            triggers: vec![ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                last_reviewed_at: None,
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Completed {
                outcome: ReviewOutcome::Quiet {
                    reason: QuietReviewReason::NothingToDo,
                    explanation: "Home BP log values remain at goal.".to_owned(),
                    citations: vec![ReviewCitation {
                        reference: ref_quiet.clone(),
                        source_id: "regional-ehr".to_owned(),
                        source_label: "Regional EHR".to_owned(),
                        sha256: "abcd".to_owned(),
                        display_label: "Home BP Log".to_owned(),
                        as_of: Some("2026-11-12".to_owned()),
                        excerpt: "118/74 mmHg average.".to_owned(),
                    }],
                    basis_and_limitations: "Regional EHR flowsheet.".to_owned(),
                },
                completed_at: now_i64 - 1_000,
                completed_source_version: 1,
            },
            evidence_read: vec![ref_quiet.clone()],
            queued_at: now_i64 - 1_200,
            updated_at: now_i64 - 1_000,
        },
    )
    .await;

    // 5. Assessed completed review today -> Updates `today_items`, no duplicate Today row.
    seed_review(
        &db,
        TestReview {
            practice: practice.clone(),
            workspace: ws_now.clone(),
            review_id: "rev-assessed-today".to_owned(),
            triggers: vec![ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                changed_candidates: vec!["obs-k-1".to_owned()],
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Completed {
                outcome: ReviewOutcome::Assessed {
                    summary: "Flagged critical potassium 6.1 mmol/L and raised Now concern."
                        .to_owned(),
                    citations: Vec::new(),
                    basis_and_limitations: "Regional EHR lab result.".to_owned(),
                },
                completed_at: now_i64 - 290,
                completed_source_version: 1,
            },
            evidence_read: vec![ref_now.clone()],
            queued_at: now_i64 - 310,
            updated_at: now_i64 - 290,
        },
    )
    .await;

    // 6. Earlier-day completed review (Yesterday) -> reaches Updates `earlier_days`.
    seed_review(
        &db,
        TestReview {
            practice: practice.clone(),
            workspace: ws_quiet.clone(),
            review_id: "rev-yesterday-1".to_owned(),
            triggers: vec![ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                last_reviewed_at: None,
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Completed {
                outcome: ReviewOutcome::Assessed {
                    summary: "Updated lipid trend note after fasting panel.".to_owned(),
                    citations: Vec::new(),
                    basis_and_limitations: "Fasting lipid panel.".to_owned(),
                },
                completed_at: now_i64 - DAY_SECONDS,
                completed_source_version: 1,
            },
            evidence_read: Vec::new(),
            queued_at: now_i64 - DAY_SECONDS - 600,
            updated_at: now_i64 - DAY_SECONDS,
        },
    )
    .await;

    // 7. Failed background review WITHOUT unfinished_work -> NO Today row!
    seed_review(
        &db,
        TestReview {
            practice: practice.clone(),
            workspace: ws_quiet.clone(),
            review_id: "rev-failed-clean".to_owned(),
            triggers: vec![ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                last_reviewed_at: None,
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Failed {
                reason: "Transient provider error on routine check".to_owned(),
                failed_at: now_i64 - 1_900,
                attempts: 3,
            },
            evidence_read: Vec::new(),
            queued_at: now_i64 - 2_000,
            updated_at: now_i64 - 1_900,
        },
    )
    .await;

    // 8. Physician-STOPPED review (`ReviewHoldReason::Stopped`) WITH unfinished_work ->
    //    MUST NOT create an unhandled Today row (`intent/physician.md:121`).
    seed_review(
        &db,
        TestReview {
            practice: practice.clone(),
            workspace: ws_quiet.clone(),
            review_id: "rev-stopped-by-physician".to_owned(),
            triggers: vec![ReviewTrigger::PhysicianRequest {
                physician_id: dr.physician_id.clone(),
                request_text: "Check home BP log".to_owned(),
                workspace_version: 1,
                record_versions: BTreeMap::new(),
            }],
            unfinished_work: vec![UnfinishedReviewWork {
                topic: "Paused BP comparison".to_owned(),
                detail: "Stopped intentionally by physician on patient page".to_owned(),
                limit: None,
            }],
            state: ReviewLifecycleState::Held {
                reason: ReviewHoldReason::Stopped,
                held_at: now_i64 - 9_000,
                partial_text: Some("Partial review paused.".to_owned()),
            },
            evidence_read: Vec::new(),
            queued_at: now_i64 - 9_100,
            updated_at: now_i64 - 9_000,
        },
    )
    .await;

    // 9. TWO unhandled items on `ws_unhandled_rev`:
    //    (a) Failed review WITH unfinished_work at `now - 7_200`
    //    (b) CouldNotFinish visit preparation at `now - 6_800`
    //    Must collapse into AT MOST ONE `unhandled:<ws_unhandled_rev>` row ordered at `now - 7_200`!
    seed_review(
        &db,
        TestReview {
            practice: practice.clone(),
            workspace: ws_unhandled_rev.clone(),
            review_id: "rev-failed-unhandled".to_owned(),
            triggers: vec![ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                changed_candidates: vec!["obs-renal-1".to_owned()],
            }],
            unfinished_work: vec![UnfinishedReviewWork {
                topic: "External nephrology consult".to_owned(),
                detail: "Verify potassium binder dose in scanned consult attachment".to_owned(),
                limit: None,
            }],
            state: ReviewLifecycleState::Failed {
                reason: "Provider budget exhausted".to_owned(),
                failed_at: now_i64 - 7_200,
                attempts: 3,
            },
            evidence_read: Vec::new(),
            queued_at: now_i64 - 7_300,
            updated_at: now_i64 - 7_200,
        },
    )
    .await;

    seed_visit_preparation(
        &db,
        TestVisitPreparation {
            practice: practice.clone(),
            workspace: ws_unhandled_rev.clone(),
            source_id: "practice-sched".to_owned(),
            appointment_id: "appt-brooks-collapse".to_owned(),
            starts_at_unix: now_i64 + 8_400,
            ends_at_unix: now_i64 + 10_200,
            state: TestVisitPrepState::CouldNotFinish {
                reason: "External dialysis flowsheet timed out before visit".to_owned(),
            },
            updated_at: now_i64 - 6_800,
        },
    )
    .await;

    // 10. Limit-held review (`ReviewHoldReason::Limit`) WITH unfinished_work on `ws_unhandled_limit` (waited 6300s).
    seed_review(
        &db,
        TestReview {
            practice: practice.clone(),
            workspace: ws_unhandled_limit.clone(),
            review_id: "rev-limit-held".to_owned(),
            triggers: vec![ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                changed_candidates: vec!["obs-onc-1".to_owned()],
            }],
            unfinished_work: vec![UnfinishedReviewWork {
                topic: "Outside oncology staging scan".to_owned(),
                detail: "Turn budget reached before reconciling PET-CT addendum".to_owned(),
                limit: Some(RecordLimit {
                    kind: RecordLimitKind::Unavailable,
                    subject: "obs-onc-1".to_owned(),
                    detail: "PET-CT addendum pending".to_owned(),
                }),
            }],
            state: ReviewLifecycleState::Held {
                reason: ReviewHoldReason::Limit,
                held_at: now_i64 - 6_300,
                partial_text: Some(
                    "Reviewed initial pathology; PET-CT addendum pending.".to_owned(),
                ),
            },
            evidence_read: Vec::new(),
            queued_at: now_i64 - 6_400,
            updated_at: now_i64 - 6_300,
        },
    )
    .await;

    // 11. Visit preparation CouldNotFinish on `ws_unhandled_visit` (waited 5400s);
    //     Skipped visit preparation on `ws_quiet` -> Updates `EndedNothingToDo`.
    seed_visit_preparation(
        &db,
        TestVisitPreparation {
            practice: practice.clone(),
            workspace: ws_unhandled_visit.clone(),
            source_id: "practice-sched".to_owned(),
            appointment_id: "appt-alvarez-upcoming".to_owned(),
            starts_at_unix: now_i64 + 7_200,
            ends_at_unix: now_i64 + 9_000,
            state: TestVisitPrepState::CouldNotFinish {
                reason: "Could not synthesize cardiology stress echo before visit".to_owned(),
            },
            updated_at: now_i64 - 5_400,
        },
    )
    .await;

    seed_visit_preparation(
        &db,
        TestVisitPreparation {
            practice: practice.clone(),
            workspace: ws_quiet.clone(),
            source_id: "practice-sched".to_owned(),
            appointment_id: "appt-cho-flu".to_owned(),
            starts_at_unix: now_i64 + 10_800,
            ends_at_unix: now_i64 + 11_700,
            state: TestVisitPrepState::Skipped {
                reason: "Nurse-only influenza vaccination visit needs no chart briefing."
                    .to_owned(),
            },
            updated_at: now_i64 - 800,
        },
    )
    .await;

    // 12. Saved patient summary on `ws_quiet` + failed InitialUnderstanding on `ws_unhandled_summary` (waited 4500s).
    seed_patient_summary(
        &db,
        &practice,
        &ws_quiet,
        1,
        1,
        &PatientSummary {
            narrative: "Well-controlled hypertension on lisinopril 10 mg daily.".to_owned(),
            claims: vec![SummaryClaimInput {
                text: "Home BP averaging 118/74 mmHg.".to_owned(),
                citations: vec![ref_quiet],
            }],
            material_gaps: Vec::new(),
            scope_note: None,
            citations: Vec::new(),
            segments: vec![SummarySegment::Prose {
                text: "Well-controlled hypertension on lisinopril 10 mg daily.".to_owned(),
            }],
        },
        now_i64 - 700,
    )
    .await;

    seed_failed_initial_understanding(
        &store,
        &db,
        auth.as_ref(),
        &practice,
        &ws_unhandled_summary,
        "init-understanding-kim",
        now_i64 - 4_500,
    )
    .await;

    // 13. Routine background running review -> Updates `active_runs` ONLY (not Today).
    seed_review(
        &db,
        TestReview {
            practice: practice.clone(),
            workspace: ws_nudge.clone(),
            review_id: "rev-routine-running".to_owned(),
            triggers: vec![ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                last_reviewed_at: None,
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Running {
                attempt_id: "att-bg-1".to_owned(),
                started_at: now_i64 - 100,
            },
            evidence_read: Vec::new(),
            queued_at: now_i64 - 120,
            updated_at: now_i64 - 100,
        },
    )
    .await;

    // 14. Active physician chat response on `ws_now` via `Store::accept_authorized` -> Today `RunningWork`.
    seed_running_conversation(
        &store,
        &db,
        &Scope {
            practice: practice.clone(),
            physician: dr.physician_id.clone(),
            workspace: ws_now.clone(),
            chat: "chat-hyperkalemia-1".to_owned(),
        },
        "Did Elena take patiromer this week?",
        now_i64 - 120,
    )
    .await;

    // --- READ TODAY ---
    let names = BTreeMap::new();
    let today = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .expect("read today");

    assert_eq!(today.completeness, ReadCompleteness::Complete);
    assert!(!today.nothing_needs_physician());
    assert_eq!(
        today.also_reviewed_today_count, 1,
        "quiet review today counts in also_reviewed_today_count without adding a Today row"
    );

    let today_keys: Vec<&str> = today.rows.iter().map(|r| r.item_key.as_str()).collect();
    // Verify quiet review, clean-failed review, physician-stopped review, retired concern,
    // and routine running review NEVER appear on Today:
    assert!(
        !today_keys.iter().any(|k| {
            k.contains("rev-quiet-today")
                || k.contains("rev-failed-clean")
                || k.contains("rev-stopped-by-physician")
                || k.contains(&ws_quiet)
                || k.contains("concern-retired-bp")
                || k.contains("rev-routine-running")
        }),
        "quiet/clean-failed/physician-stopped/retired/routine-running items must never create Today rows: {today_keys:?}"
    );

    // Verify `ws_unhandled_rev` has TWO underlying unhandled failures but collapses into AT MOST ONE Today row!
    let unhandled_rev_rows: Vec<_> = today
        .rows
        .iter()
        .filter(|r| r.workspace == ws_unhandled_rev)
        .collect();
    assert_eq!(
        unhandled_rev_rows.len(),
        1,
        "multiple unhandled items on one patient must collapse into one unhandled Today row: {unhandled_rev_rows:?}"
    );
    assert_eq!(
        unhandled_rev_rows[0].item_key.as_str(),
        format!("unhandled:{ws_unhandled_rev}")
    );
    assert_eq!(
        unhandled_rev_rows[0].waiting_since_unix,
        now_i64 - 7_200,
        "collapsed unhandled row must keep the earliest waiting_since_unix"
    );

    // Verify Now comes FIRST, then remaining rows ordered strictly by longest wait:
    // 0: Now concern (-300, urgency Now)
    // 1: unhandled:<ws_unhandled_rev> (-7200, PatientReview + VisitPreparation collapsed)
    // 2: unhandled:<ws_unhandled_limit> (-6300, Limit-held PatientReview)
    // 3: unhandled:<ws_unhandled_visit> (-5400, VisitPreparation CouldNotFinish)
    // 4: unhandled:<ws_unhandled_summary> (-4500, InitialUnderstanding Failed)
    // 5: Today pre-visit nudge (-3600)
    // 6: Running physician chat (-120)
    assert_eq!(today.rows.len(), 7, "unexpected Today rows: {today_keys:?}");
    assert_eq!(
        today.rows[0].item_key.as_str(),
        format!("concern:{ws_now}:concern-now-k")
    );
    assert!(today.rows[0].is_now());
    assert_eq!(today.rows[0].patient_display_name, "Elena Vance");

    assert!(matches!(
        today.rows[1].payload,
        TodayRowPayload::UnhandledWork {
            work_kind: UnhandledWorkKind::PatientReview,
            ..
        }
    ));
    assert_eq!(today.rows[1].workspace, ws_unhandled_rev);

    assert!(matches!(
        today.rows[2].payload,
        TodayRowPayload::UnhandledWork {
            work_kind: UnhandledWorkKind::PatientReview,
            ..
        }
    ));
    assert_eq!(today.rows[2].workspace, ws_unhandled_limit);

    assert!(matches!(
        today.rows[3].payload,
        TodayRowPayload::UnhandledWork {
            work_kind: UnhandledWorkKind::VisitPreparation,
            ..
        }
    ));
    assert_eq!(today.rows[3].workspace, ws_unhandled_visit);

    assert!(matches!(
        today.rows[4].payload,
        TodayRowPayload::UnhandledWork {
            work_kind: UnhandledWorkKind::InitialUnderstanding,
            ..
        }
    ));
    assert_eq!(today.rows[4].workspace, ws_unhandled_summary);

    assert!(matches!(
        today.rows[5].payload,
        TodayRowPayload::OpenConcern {
            urgency: ConcernUrgency::Today,
            visit_slot: Some(_),
            ..
        }
    ));
    assert!(matches!(
        today.rows[6].payload,
        TodayRowPayload::RunningWork {
            run_kind: RecognizableRunKind::PhysicianChatResponse,
            ..
        }
    ));

    // Verify opening Elena's Now concern returns FULL untruncated readable body (without raw
    // `"attachment"` structural label) AND preserves `native_uri`!
    let mut authority = PostgresWorkspaceAuthority;
    let open_key = SeenItemKey::new(format!("concern:{ws_now}:concern-now-k")).unwrap();
    let evidence = store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut authority,
            &OpenItemRequest {
                token: &token,
                real_now_unix: now_u64,
                workspace: &ws_now,
                item_key: &open_key,
                command_key: &CommandKey::new("cmd-open-now-1").unwrap(),
                patient_names: &names,
            },
        )
        .await
        .expect("open Now concern");
    assert_eq!(evidence.citations.len(), 1);
    let cit = &evidence.citations[0];
    assert!(cit.excerpt.contains("6.1 mmol/L"));
    assert!(
        cit.excerpt.chars().count() <= 325 && !cit.excerpt.contains("TAIL_MARKER_BEYOND_320_CHARS"),
        "excerpt remains concise summary: len={}",
        cit.excerpt.chars().count()
    );
    assert!(
        cit.body.len() > 450 && cit.body.contains("TAIL_MARKER_BEYOND_320_CHARS"),
        "citation body must carry full untruncated readable record text, got len={}: {}",
        cit.body.len(),
        cit.body
    );
    assert!(
        !cit.body.lines().any(|line| line.trim() == "attachment"),
        "generic structural part label `attachment` must not appear as a standalone line:\n{}",
        cit.body
    );
    assert_eq!(
        cit.native_uri.as_deref(),
        Some("https://ehr.codahealth.test/chart/obs-k-1"),
        "citation must preserve native_uri from source provenance"
    );
    assert!(matches!(evidence.seen_standing, SeenStanding::Seen { .. }));

    // Verify opening collapsed `unhandled:<ws_unhandled_rev>` shows both constituent unhandled items:
    let unhandled_row = today.rows[1].clone();
    let unhandled_evidence = store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut authority,
            &OpenItemRequest {
                token: &token,
                real_now_unix: now_u64,
                workspace: &unhandled_row.workspace,
                item_key: &unhandled_row.item_key,
                command_key: &CommandKey::new("cmd-open-unhandled-1").unwrap(),
                patient_names: &names,
            },
        )
        .await
        .expect("open collapsed unhandled row");
    assert!(
        unhandled_evidence
            .clinical_body
            .contains("External nephrology consult")
            && unhandled_evidence
                .clinical_body
                .contains("External dialysis flowsheet timed out"),
        "collapsed unhandled detail must surface every constituent item:\n{}",
        unhandled_evidence.clinical_body
    );

    // Verify BEFORE `Done`, `next_review_starting_context` does NOT carry `unhandled_handed_off_on_done`:
    let ctx_before_done = store
        .next_review_starting_context(
            &practice,
            &ws_unhandled_rev,
            "rev-next-check",
            ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                last_reviewed_at: None,
            },
        )
        .await
        .expect("starting context before Done");
    assert!(
        !ctx_before_done
            .prior_reviews
            .iter()
            .any(|p| p.disposition == "unhandled_handed_off_on_done"),
        "unhandled review must only hand forward on Done clearance"
    );

    // Clear `unhandled:<ws_unhandled_rev>` with `Done` -> leaves Today AND hands unreviewed
    // changed candidates + unfinished work to the next review (`intent/physician.md:130`)!
    let done_res = store
        .clear_today_item_done(
            auth.as_ref(),
            &mut authority,
            &ClearItemDoneRequest {
                token: &token,
                real_now_unix: now_u64,
                workspace: &unhandled_row.workspace,
                item_key: &unhandled_row.item_key,
                version_id: &unhandled_row.version_id,
                command_key: &CommandKey::new("cmd-done-unhandled-1").unwrap(),
            },
        )
        .await
        .expect("clear unhandled work Done");
    assert!(matches!(done_res, TodayDoneResult::Cleared { .. }));

    let today_after_done = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert!(
        !today_after_done
            .rows
            .iter()
            .any(|r| r.workspace == ws_unhandled_rev),
        "cleared unhandled row must leave Today"
    );

    let ctx_after_done = store
        .next_review_starting_context(
            &practice,
            &ws_unhandled_rev,
            "rev-next-1",
            ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                last_reviewed_at: None,
            },
        )
        .await
        .expect("starting context after Done");
    let handed_off = ctx_after_done
        .prior_reviews
        .iter()
        .find(|p| p.disposition == "unhandled_handed_off_on_done")
        .expect("expected handed-off unhandled review summary in next_review_starting_context");
    assert!(
        handed_off
            .unfinished_work
            .iter()
            .any(|u| u.detail.contains("obs-renal-1")),
        "handed-off unfinished_work must include unreviewed changed candidate `obs-renal-1`: {:?}",
        handed_off.unfinished_work
    );
    assert!(
        handed_off
            .unfinished_work
            .iter()
            .any(|u| u.topic == "External nephrology consult"),
        "handed-off summary must carry unfinished_work forward: {:?}",
        handed_off.unfinished_work
    );
    let dev_text = ctx_after_done.developer_text();
    assert!(
        dev_text.contains("obs-renal-1") && dev_text.contains("External nephrology consult"),
        "developer_text() must expose handed-off unreviewed results to the next review:\n{dev_text}"
    );

    // Seeding a NEW unhandled failure on `ws_unhandled_rev` after `Done` resurfaces `unhandled:<ws>`!
    seed_review(
        &db,
        TestReview {
            practice: practice.clone(),
            workspace: ws_unhandled_rev.clone(),
            review_id: "rev-failed-after-done".to_owned(),
            triggers: vec![ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                changed_candidates: vec!["obs-renal-1".to_owned()],
            }],
            unfinished_work: vec![UnfinishedReviewWork {
                topic: "New dialysis access ultrasound".to_owned(),
                detail: "Interrupted while fetching vascular surgery addendum".to_owned(),
                limit: None,
            }],
            state: ReviewLifecycleState::Failed {
                reason: "Upstream timeout".to_owned(),
                failed_at: now_i64 - 60,
                attempts: 3,
            },
            evidence_read: Vec::new(),
            queued_at: now_i64 - 90,
            updated_at: now_i64 - 60,
        },
    )
    .await;
    let today_resurfaced = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    let resurfaced = today_resurfaced
        .rows
        .iter()
        .find(|r| r.workspace == ws_unhandled_rev)
        .expect("new unhandled failure after Done must resurface unhandled:<workspace>");
    assert_ne!(
        resurfaced.version_id, unhandled_row.version_id,
        "resurfaced unhandled row must carry a distinct version_id"
    );

    // --- READ UPDATES ---
    let updates = store
        .read_updates(auth.as_ref(), &token, now_u64, &names)
        .await
        .expect("read updates");
    assert_eq!(updates.completeness, ReadCompleteness::Complete);
    assert_eq!(
        updates.active_runs.len(),
        1,
        "routine running review surfaces in Updates active_runs"
    );
    assert_eq!(
        updates.reviewed_nothing_for_you.len(),
        1,
        "quiet review today reaches Reviewed, nothing for you"
    );
    assert!(matches!(
        updates.reviewed_nothing_for_you[0].kind,
        UpdateEntryKind::QuietReview { .. }
    ));

    // Verify clean non-duplicated `title` strings across ALL Updates groups (no category prefixes in `title`!).
    for entry in updates
        .today_items
        .iter()
        .chain(updates.reviewed_nothing_for_you.iter())
        .chain(updates.earlier_days.iter().flat_map(|g| g.items.iter()))
    {
        for forbidden_prefix in [
            "Concern raised:",
            "Retired concern:",
            "Reviewed chart",
            "Chart review:",
            "Visit briefing prepared:",
            "Prepared patient summary",
        ] {
            assert!(
                !entry.title.starts_with(forbidden_prefix),
                "UpdateEntry.title must not repeat category badge prefix {forbidden_prefix:?}: {:?}",
                entry.title
            );
        }
    }

    // Check every producer kind in `today_items`: AssessedReview, VisitBriefing, PatientSummary, ConcernRaised, ConcernRetired, EndedNothingToDo!
    let has_assessed = updates
        .today_items
        .iter()
        .any(|i| matches!(i.kind, UpdateEntryKind::AssessedReview { .. }));
    let has_briefing = updates
        .today_items
        .iter()
        .any(|i| matches!(i.kind, UpdateEntryKind::VisitBriefing { .. }));
    let has_summary = updates
        .today_items
        .iter()
        .any(|i| matches!(i.kind, UpdateEntryKind::PatientSummary { .. }));
    let has_raised = updates
        .today_items
        .iter()
        .any(|i| matches!(i.kind, UpdateEntryKind::ConcernRaised { .. }));
    let has_retired = updates.today_items.iter().any(|i| {
        matches!(
            &i.kind,
            UpdateEntryKind::ConcernRetired {
                retirement_reason,
                ..
            } if retirement_reason.contains("normalized")
        )
    });
    let has_skipped = updates.today_items.iter().any(|i| {
        matches!(
            &i.kind,
            UpdateEntryKind::EndedNothingToDo { reason, .. } if reason.contains("influenza vaccination")
        )
    });
    assert!(
        has_assessed && has_briefing && has_summary && has_raised && has_retired && has_skipped,
        "every producer kind must reach Updates today_items: {:?}",
        updates.today_items
    );

    // Verify OperatingIndex sentinel never leaks into Updates JSON:
    let updates_serialized = serde_json::to_string(&updates).unwrap();
    assert!(
        !updates_serialized.contains("INTERNAL-OPERATING-INDEX-NEVER-SHOWN"),
        "OperatingIndex must never leak into Updates projection"
    );

    // Verify Earlier days group contains yesterday's review:
    assert_eq!(updates.earlier_days.len(), 1);
    assert_eq!(updates.earlier_days[0].day_label, "Yesterday");
    assert_eq!(updates.earlier_days[0].items.len(), 1);
    assert_eq!(
        updates.earlier_days[0].items[0].item_key.as_str(),
        format!("review:{ws_quiet}:rev-yesterday-1")
    );
}

#[tokio::test]
async fn ac2_seen_state_is_saved_per_physician_survives_restart_and_changed_item_returns_as_new_with_what_changed()
 {
    let Some((store, db, auth, url, _cluster)) = connect_stores().await else {
        return;
    };
    let now_u64 = base_now_unix();
    let now_i64 = now_u64 as i64;
    let practice = format!("prac-ac2-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_u64))
        .await
        .unwrap();

    let (dr_a, token_a) = invite_and_sign_in(&auth, &practice, "Dr. Patel", now_u64).await;
    let (dr_b, token_b) = invite_and_sign_in(&auth, &practice, "Dr. Nguyen", now_u64).await;
    let ws = format!("pw-shared-{}", Uuid::new_v4().simple());
    auth.register_patient(&practice, &ws).await.unwrap();
    auth.set_membership(&dr_a, &ws, Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&dr_b, &ws, Membership::Allowed)
        .await
        .unwrap();

    let rec_ref = seed_chart_record(
        &store,
        &ws,
        "Samira Al-Mansoor",
        "obs-cr-1",
        "rev-1",
        "Serum creatinine 1.6 mg/dL (baseline 1.0 mg/dL).",
        now_i64 - 1_200,
    )
    .await;

    seed_concern(
        &store,
        &db,
        TestConcern {
            practice: practice.clone(),
            workspace: ws.clone(),
            concern_id: "concern-creatinine".to_owned(),
            urgency: ConcernUrgency::Today,
            standing: TestConcernStanding::Open,
            reason: "Creatinine rose from 1.0 to 1.6 mg/dL.".to_owned(),
            records: vec![rec_ref.clone()],
            source_work: SourceWork::new("patient_review", "work-cr-1", "att-1").unwrap(),
            created_at: now_i64 - 1_000,
            updated_at: now_i64 - 1_000,
            visit_slot: None,
        },
    )
    .await;

    let names = BTreeMap::new();
    let item_key = SeenItemKey::new(format!("concern:{ws}:concern-creatinine")).unwrap();

    // Initially Unseen for both physicians.
    let today_a0 = store
        .read_today(auth.as_ref(), &token_a, now_u64, &names)
        .await
        .unwrap();
    let today_b0 = store
        .read_today(auth.as_ref(), &token_b, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(today_a0.rows[0].seen_standing, SeenStanding::Unseen);
    assert_eq!(today_b0.rows[0].seen_standing, SeenStanding::Unseen);

    // Dr. Patel opens the item -> Seen for Dr. Patel, still Unseen for Dr. Nguyen.
    let mut authority = PostgresWorkspaceAuthority;
    store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut authority,
            &OpenItemRequest {
                token: &token_a,
                real_now_unix: now_u64,
                workspace: &ws,
                item_key: &item_key,
                command_key: &CommandKey::new("cmd-patel-open-1").unwrap(),
                patient_names: &names,
            },
        )
        .await
        .unwrap();

    // Simulate full process restart by dropping `store` and reconnecting a fresh `Store`.
    drop(store);
    let restarted_store = Store::connect(&url).await.expect("reconnect runtime store");

    let today_a1 = restarted_store
        .read_today(auth.as_ref(), &token_a, now_u64 + 10, &names)
        .await
        .unwrap();
    let today_b1 = restarted_store
        .read_today(auth.as_ref(), &token_b, now_u64 + 10, &names)
        .await
        .unwrap();
    assert!(matches!(
        today_a1.rows[0].seen_standing,
        SeenStanding::Seen { .. }
    ));
    assert_eq!(
        today_b1.rows[0].seen_standing,
        SeenStanding::Unseen,
        "Dr. Nguyen's seen state must remain independent of Dr. Patel"
    );

    // Now Coda revises the concern via `Store::revise_concern` (escalates to Now + repeat Cr 2.3).
    revise_concern_at(
        &restarted_store,
        &db,
        &practice,
        &ws,
        "concern-creatinine",
        ConcernUrgency::Now,
        "Repeat creatinine 2.3 mg/dL with hyperkalemia.",
        vec![rec_ref],
        now_i64 + 30,
    )
    .await;

    let today_a2 = restarted_store
        .read_today(auth.as_ref(), &token_a, now_u64 + 40, &names)
        .await
        .unwrap();
    let today_b2 = restarted_store
        .read_today(auth.as_ref(), &token_b, now_u64 + 40, &names)
        .await
        .unwrap();

    assert!(
        today_a2.rows[0].seen_standing.needs_physician_eye(),
        "changed item must return as needing physician eye"
    );
    match &today_a2.rows[0].seen_standing {
        SeenStanding::ChangedSinceSeen { what_changed, .. } => {
            assert!(
                what_changed.contains("Urgency raised to Now"),
                "expected urgency escalation in what_changed, got: {what_changed}"
            );
            assert!(
                what_changed.contains("2.3 mg/dL"),
                "expected updated clinical finding in what_changed, got: {what_changed}"
            );
        }
        other => panic!("expected ChangedSinceSeen for Dr. Patel, got {other:?}"),
    }
    assert_eq!(today_b2.rows[0].seen_standing, SeenStanding::Unseen);

    let updates_a2 = restarted_store
        .read_updates(auth.as_ref(), &token_a, now_u64 + 40, &names)
        .await
        .unwrap();
    assert!(matches!(
        updates_a2.today_items[0].seen_standing,
        SeenStanding::ChangedSinceSeen { .. }
    ));
}

#[tokio::test]
async fn ac3_physician_sees_only_authorized_patients_including_after_access_changes() {
    let Some((store, db, auth, _url, _cluster)) = connect_stores().await else {
        return;
    };
    let now_u64 = base_now_unix();
    let now_i64 = now_u64 as i64;
    let practice = format!("prac-ac3-{}", Uuid::new_v4().simple());
    let other_practice = format!("prac-other-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_u64))
        .await
        .unwrap();
    auth.ensure_practice(&other_practice, PracticeClock::Fixed(now_u64))
        .await
        .unwrap();

    let (dr, token) = invite_and_sign_in(&auth, &practice, "Dr. Lin", now_u64).await;
    let ws_allowed = format!("pw-allowed-{}", Uuid::new_v4().simple());
    let ws_off_panel = format!("pw-offpanel-{}", Uuid::new_v4().simple());
    let ws_other_prac = format!("pw-otherprac-{}", Uuid::new_v4().simple());

    auth.register_patient(&practice, &ws_allowed).await.unwrap();
    auth.register_patient(&practice, &ws_off_panel)
        .await
        .unwrap();
    auth.register_patient(&other_practice, &ws_other_prac)
        .await
        .unwrap();

    auth.set_membership(&dr, &ws_allowed, Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&dr, &ws_off_panel, Membership::ClinicDemographic)
        .await
        .unwrap();

    for (target_prac, ws, name) in [
        (&practice, &ws_allowed, "Panel Patient"),
        (&practice, &ws_off_panel, "Off-Panel Clinic Patient"),
        (&other_practice, &ws_other_prac, "Other Practice Patient"),
    ] {
        let rec =
            seed_chart_record(&store, ws, name, "obs-1", "rev-1", "Lab finding", now_i64).await;
        seed_concern(
            &store,
            &db,
            TestConcern {
                practice: target_prac.clone(),
                workspace: ws.clone(),
                concern_id: "concern-1".to_owned(),
                urgency: ConcernUrgency::Now,
                standing: TestConcernStanding::Open,
                reason: format!("Urgent finding for {name}"),
                records: vec![rec],
                source_work: SourceWork::new("patient_review", "work-1", "att-1").unwrap(),
                created_at: now_i64 - 100,
                updated_at: now_i64 - 100,
                visit_slot: None,
            },
        )
        .await;
    }

    let names = BTreeMap::new();
    let today_initial = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(today_initial.rows.len(), 1);
    assert_eq!(today_initial.rows[0].workspace, ws_allowed);

    let updates_initial = store
        .read_updates(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(updates_initial.today_items.len(), 1);
    assert_eq!(updates_initial.today_items[0].workspace, ws_allowed);

    assert!(matches!(
        store
            .read_today_for_workspace(auth.as_ref(), &token, now_u64, &ws_off_panel, &names)
            .await,
        Err(TodayUpdatesError::AccessDenied { .. })
    ));
    assert!(matches!(
        store
            .read_today_for_workspace(auth.as_ref(), &token, now_u64, &ws_other_prac, &names)
            .await,
        Err(TodayUpdatesError::AccessDenied { .. })
    ));

    auth.set_membership(&dr, &ws_off_panel, Membership::Allowed)
        .await
        .unwrap();
    let today_granted = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(today_granted.rows.len(), 2);

    auth.set_membership(&dr, &ws_allowed, Membership::ClinicDemographic)
        .await
        .unwrap();
    let today_revoked = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(today_revoked.rows.len(), 1);
    assert_eq!(today_revoked.rows[0].workspace, ws_off_panel);

    let updates_revoked = store
        .read_updates(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(updates_revoked.today_items.len(), 1);
    assert_eq!(updates_revoked.today_items[0].workspace, ws_off_panel);

    let mut authority = PostgresWorkspaceAuthority;
    let revoked_key = SeenItemKey::new(format!("concern:{ws_allowed}:concern-1")).unwrap();
    let open_err = store
        .open_today_or_update_item(
            auth.as_ref(),
            &mut authority,
            &OpenItemRequest {
                token: &token,
                real_now_unix: now_u64,
                workspace: &ws_allowed,
                item_key: &revoked_key,
                command_key: &CommandKey::new("cmd-open-revoked").unwrap(),
                patient_names: &names,
            },
        )
        .await;
    assert!(matches!(
        open_err,
        Err(TodayUpdatesError::AccessDenied { .. })
    ));
}

#[tokio::test]
async fn ac4_partial_producer_failure_practice_source_outage_collapse_and_empty_completeness_rule()
{
    let Some((store, db, auth, _url, _cluster)) = connect_stores().await else {
        return;
    };
    let now_u64 = base_now_unix();
    let now_i64 = now_u64 as i64;
    let practice = format!("prac-ac4-{}", Uuid::new_v4().simple());
    auth.ensure_practice(&practice, PracticeClock::Fixed(now_u64))
        .await
        .unwrap();

    let (dr, token) = invite_and_sign_in(&auth, &practice, "Dr. Okafor", now_u64).await;
    let ws_1 = format!("pw-p1-{}", Uuid::new_v4().simple());
    let ws_2 = format!("pw-p2-{}", Uuid::new_v4().simple());
    for ws in [&ws_1, &ws_2] {
        auth.register_patient(&practice, ws).await.unwrap();
        auth.set_membership(&dr, ws, Membership::Allowed)
            .await
            .unwrap();
    }

    let names = BTreeMap::new();

    // 1. Clean empty state when EVERY producer answers -> `nothing_needs_physician() == true`.
    let clean_empty = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(clean_empty.completeness, ReadCompleteness::Complete);
    assert!(clean_empty.rows.is_empty());
    assert!(clean_empty.practice_outages.is_empty());
    assert!(clean_empty.nothing_needs_physician());

    // 2. Empty rows when ONE producer fails -> `nothing_needs_physician()` MUST be FALSE!
    let concerns_down_probe = ForcedFaultProbe {
        fail_producers: BTreeSet::from([ProducerKind::Concerns]),
        fail_open_keys: BTreeSet::new(),
    };
    let partial_empty = store
        .read_today_with_probe(
            auth.as_ref(),
            &token,
            now_u64,
            None,
            &names,
            &concerns_down_probe,
        )
        .await
        .unwrap();
    assert!(!partial_empty.completeness.is_complete());
    assert!(!partial_empty.nothing_needs_physician());
    assert_eq!(
        partial_empty.completeness.failed_parts()[0].producer,
        ProducerKind::Concerns
    );

    // 3. Seed an open concern on `ws_1`, then force `PatientReviews` to fail:
    let rec = seed_chart_record(
        &store,
        &ws_1,
        "Patient One",
        "obs-1",
        "rev-1",
        "INR 4.8",
        now_i64,
    )
    .await;
    seed_concern(
        &store,
        &db,
        TestConcern {
            practice: practice.clone(),
            workspace: ws_1.clone(),
            concern_id: "concern-inr".to_owned(),
            urgency: ConcernUrgency::Now,
            standing: TestConcernStanding::Open,
            reason: "Elevated INR 4.8 on warfarin.".to_owned(),
            records: vec![rec],
            source_work: SourceWork::new("patient_review", "work-inr", "att-1").unwrap(),
            created_at: now_i64 - 200,
            updated_at: now_i64 - 200,
            visit_slot: None,
        },
    )
    .await;

    let reviews_down_probe = ForcedFaultProbe {
        fail_producers: BTreeSet::from([ProducerKind::PatientReviews]),
        fail_open_keys: BTreeSet::new(),
    };
    let partial_with_concern = store
        .read_today_with_probe(
            auth.as_ref(),
            &token,
            now_u64,
            None,
            &names,
            &reviews_down_probe,
        )
        .await
        .unwrap();
    assert_eq!(partial_with_concern.rows.len(), 1);
    assert_eq!(partial_with_concern.completeness.failed_parts().len(), 1);
    assert_eq!(
        partial_with_concern.completeness.failed_parts()[0].producer,
        ProducerKind::PatientReviews
    );

    // 4. Single-patient source failure via `Store::refresh_patient_sources_for_practice`:
    //    `ws_1` failed, `ws_2` updated -> NO practice outage line!
    let quest_id = SourceId::new("quest-labs").unwrap();
    let checker = FixedSourceCheck::new();
    checker.push_step(
        &practice,
        &ws_1,
        quest_id.clone(),
        FixedCheckStep::Outcome(SourceCheckOutcome::Failed {
            source_label: Some("Quest Diagnostics".to_owned()),
            checked_at: now_i64 - 120,
            reason: "patient match timeout".to_owned(),
        }),
    );
    checker.push_step(
        &practice,
        &ws_2,
        quest_id.clone(),
        FixedCheckStep::Outcome(SourceCheckOutcome::Updated {
            source_label: "Quest Diagnostics".to_owned(),
            patient: make_source_patient(&ws_2, "Patient Two"),
            pulled_at: now_i64 - 60,
            records: Vec::new(),
        }),
    );

    store
        .refresh_patient_sources_for_practice(
            &PracticeScope {
                practice: practice.clone(),
                workspace: ws_1.clone(),
                job_id: "job-refresh-ws1-1".to_owned(),
            },
            auth.as_ref(),
            &RefreshRequest::new(
                RefreshCommandKey::new("cmd-refresh-ws1-1").unwrap(),
                RefreshTarget::AllSources,
            ),
            &checker,
        )
        .await
        .expect("refresh ws_1");
    store
        .refresh_patient_sources_for_practice(
            &PracticeScope {
                practice: practice.clone(),
                workspace: ws_2.clone(),
                job_id: "job-refresh-ws2-1".to_owned(),
            },
            auth.as_ref(),
            &RefreshRequest::new(
                RefreshCommandKey::new("cmd-refresh-ws2-1").unwrap(),
                RefreshTarget::AllSources,
            ),
            &checker,
        )
        .await
        .expect("refresh ws_2");

    let single_fail_today = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert!(
        single_fail_today.practice_outages.is_empty(),
        "single-patient source failure must not report a practice-wide outage"
    );

    // 5. Whole source down across the practice (`ws_1` AND `ws_2` failed on `quest-labs`) ->
    //    ONE outage line with clean `failure_detail` (never repeating `"is unavailable across the practice"`).
    checker.push_step(
        &practice,
        &ws_2,
        quest_id,
        FixedCheckStep::Outcome(SourceCheckOutcome::Failed {
            source_label: Some("Quest Diagnostics".to_owned()),
            checked_at: now_i64 - 30,
            reason: "upstream gateway 503".to_owned(),
        }),
    );
    store
        .refresh_patient_sources_for_practice(
            &PracticeScope {
                practice: practice.clone(),
                workspace: ws_2.clone(),
                job_id: "job-refresh-ws2-2".to_owned(),
            },
            auth.as_ref(),
            &RefreshRequest::new(
                RefreshCommandKey::new("cmd-refresh-ws2-2").unwrap(),
                RefreshTarget::AllSources,
            ),
            &checker,
        )
        .await
        .expect("refresh ws_2 failed");

    let practice_outage_today = store
        .read_today(auth.as_ref(), &token, now_u64, &names)
        .await
        .unwrap();
    assert_eq!(
        practice_outage_today.practice_outages.len(),
        1,
        "practice-wide source failure must collapse into one outage line"
    );
    assert_eq!(
        practice_outage_today.practice_outages[0].source_id,
        "quest-labs"
    );
    assert_eq!(
        practice_outage_today.practice_outages[0].source_label,
        "Quest Diagnostics"
    );
    assert!(
        !practice_outage_today.practice_outages[0]
            .failure_detail
            .contains("is unavailable across the practice"),
        "failure_detail must hold only the underlying failure reason, got: {:?}",
        practice_outage_today.practice_outages[0].failure_detail
    );
    assert!(
        practice_outage_today.practice_outages[0]
            .failure_detail
            .contains("patient match timeout")
            || practice_outage_today.practice_outages[0]
                .failure_detail
                .contains("upstream gateway 503"),
        "expected underlying failure reason in failure_detail, got: {:?}",
        practice_outage_today.practice_outages[0].failure_detail
    );
}
