//! Chromium browser qualification over PostgreSQL for Today and Updates (`GET /today`,
//! `GET /updates`, `POST /today-updates/open`, and `POST /today-updates/done`).

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex as StdMutex, OnceLock, Weak};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use base64::Engine as _;
use front_door::PostgresFrontDoorStore;
use harness::engine::AttemptFailure;
use local_product::ModelRoles;
use local_product::initial_understanding::{EvidenceScope, SummaryClaimInput};
use local_product::patient_reviews::{
    QuietReviewReason, ReviewCitation, ReviewOutcome, ReviewTrigger, UnfinishedReviewWork,
};
use local_product::physician_chat::PhysicianSkills;
use local_product::physician_sources::{
    MedicalRecordNumber, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
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
use local_product::runtime::today_updates::{ProducerKind, TodayVisitSlot};
use local_product::runtime::understanding::{PatientSummary, SummaryCitation, SummarySegment};
use local_product::runtime::{Input, Scope, Store};
use local_product::sha256_hex;
use local_product::visit_preparation::VisitPreparationDecision;
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use physician_sample::today_updates::{TodayUpdatesFaultInjection, fault_layer};
use physician_sample::{STAGING_PRACTICE_ID, WORKSPACE_IDS};
use sea_orm::sea_query::{Alias, Expr, ExprTrait, OnConflict, Query};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection};
use serde_json::json;
use tokio::sync::Mutex;
use uuid::Uuid;

#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;

use devtools::Browser;

/// Synthetic practice clock unix timestamp (`2026-08-27T16:00:00Z`) configured by `Auth::over_fixture`.
const CLINICAL_NOW_UNIX: i64 = 1_787_846_400;
const DAY_SECONDS: i64 = 86_400;

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
static BUILT_SITE: OnceLock<PathBuf> = OnceLock::new();

fn test_lock() -> &'static Mutex<()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(()))
}

fn real_now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or(1_787_846_400)
}

/// Ephemeral PostgreSQL cluster started on demand when no external test database URL is configured.
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
        let _ = fs::remove_dir_all(&self.cluster_dir);
    }
}

static LOCAL_CLUSTER: StdMutex<Weak<LocalCluster>> = StdMutex::new(Weak::new());

fn ensure_admin_postgres_url() -> (String, Option<Arc<LocalCluster>>) {
    for key in [
        "PHYSICIAN_DATABASE_URL",
        "PHYSICIAN_RUNTIME_TEST_DATABASE_URL",
    ] {
        if let Ok(val) = env::var(key) {
            let trimmed = val.trim();
            if !trimmed.is_empty() {
                let (base, _) = trimmed
                    .rsplit_once('/')
                    .expect("database URL must name a database");
                return (format!("{base}/postgres"), None);
            }
        }
    }

    let mut slot = LOCAL_CLUSTER
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(existing) = slot.upgrade() {
        return (existing.admin_url.clone(), Some(existing));
    }

    let port = free_loopback_port();
    let cluster_dir = env::temp_dir().join(format!(
        "coda-tu-browser-pg-{}-{}",
        std::process::id(),
        port
    ));
    let _ = fs::remove_dir_all(&cluster_dir);
    let data_dir = cluster_dir.join("data");
    let sock_dir = cluster_dir.join("sock");
    fs::create_dir_all(&data_dir).expect("create temp postgres data dir");
    fs::create_dir_all(&sock_dir).expect("create temp postgres sock dir");

    let init = Command::new("initdb")
        .args([
            "-D",
            data_dir.to_str().unwrap(),
            "-U",
            "postgres",
            "-A",
            "trust",
            "--no-locale",
            "-E",
            "UTF8",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .expect("run initdb");
    assert!(
        init.status.success(),
        "initdb failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );

    let log_file = cluster_dir.join("postgres.log");
    let opts = format!(
        "-F -p {port} -h 127.0.0.1 -k {}",
        sock_dir.to_str().unwrap()
    );
    let start = Command::new("pg_ctl")
        .args([
            "-D",
            data_dir.to_str().unwrap(),
            "-l",
            log_file.to_str().unwrap(),
            "-o",
            &opts,
            "-w",
            "start",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .expect("run pg_ctl start");
    assert!(
        start.status.success(),
        "pg_ctl start failed: {}\nlog:\n{}",
        String::from_utf8_lossy(&start.stderr),
        fs::read_to_string(&log_file).unwrap_or_default()
    );

    let cluster = Arc::new(LocalCluster {
        cluster_dir,
        data_dir,
        admin_url: format!("postgresql://postgres@127.0.0.1:{port}/postgres"),
    });
    *slot = Arc::downgrade(&cluster);
    (cluster.admin_url.clone(), Some(cluster))
}

fn free_loopback_port() -> u16 {
    use std::net::TcpListener;
    for _ in 0..500 {
        let port = 22_000 + (Uuid::new_v4().as_u128() % 9_000) as u16;
        if TcpListener::bind(("127.0.0.1", port)).is_ok() {
            return port;
        }
    }
    panic!("no free loopback port for test cluster");
}

struct DisposableDb {
    admin_url: String,
    db_name: String,
    url: String,
    _cluster: Option<Arc<LocalCluster>>,
}

impl DisposableDb {
    async fn create(label: &str) -> Self {
        let (admin_url, cluster) = ensure_admin_postgres_url();
        let (base, _) = admin_url.rsplit_once('/').expect("admin url base");
        let db_name = format!("tu_site_{label}_{}", Uuid::new_v4().simple());
        let admin = sea_orm::Database::connect(&admin_url)
            .await
            .expect("connect admin db");
        admin
            .execute_unprepared(&format!("CREATE DATABASE {db_name}"))
            .await
            .expect("create disposable db");
        admin.close().await.expect("close admin db");

        let url = format!("{base}/{db_name}");
        PostgresAuthStore::connect_with_migrations(&url)
            .await
            .expect("migrate physician auth");
        PostgresFrontDoorStore::connect_with_migrations(&url)
            .await
            .expect("migrate front door");
        Store::connect(&url)
            .await
            .expect("connect runtime store")
            .migrate()
            .await
            .expect("migrate runtime store");

        Self {
            admin_url,
            db_name,
            url,
            _cluster: cluster,
        }
    }
}

impl Drop for DisposableDb {
    fn drop(&mut self) {
        let sql = format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}' AND pid <> pg_backend_pid()",
            self.db_name
        );
        let _ = Command::new("psql")
            .args([&self.admin_url, "-c", &sql])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        let _ = Command::new("dropdb")
            .args([
                "--maintenance-db",
                &self.admin_url,
                "--if-exists",
                &self.db_name,
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}

struct RunningHarness {
    _db: DisposableDb,
    address: SocketAddr,
    store: Store,
    db: DatabaseConnection,
    auth: Arc<PostgresAuthStore>,
    faults: Arc<TodayUpdatesFaultInjection>,
    server_task: tokio::task::JoinHandle<()>,
}

impl Drop for RunningHarness {
    fn drop(&mut self) {
        self.server_task.abort();
    }
}

impl RunningHarness {
    async fn start(label: &str) -> Self {
        let site = build_site();
        // SAFETY: serialized by `test_lock()` held across each test.
        unsafe {
            env::set_var("PHYSICIAN_SITE_DIR", &site);
        }
        let disposable = DisposableDb::create(label).await;
        let store = Store::connect(&disposable.url)
            .await
            .expect("connect store");
        let db = Database::connect(&disposable.url)
            .await
            .expect("connect sea_orm db");
        let auth = Arc::new(
            PostgresAuthStore::connect_without_migrations(&disposable.url)
                .await
                .expect("connect auth store"),
        );
        let front_door = Arc::new(
            PostgresFrontDoorStore::connect_without_migrations(&disposable.url)
                .await
                .expect("connect front-door store"),
        );
        let faults = TodayUpdatesFaultInjection::new();

        let base_router = physician_sample::router()
            .layer(axum::Extension(store.clone()))
            .layer(fault_layer(Arc::clone(&faults)));
        let auth_dyn: Arc<dyn AuthStore> = auth.clone();
        let app = physician_sample::authed_router_on(base_router, auth_dyn, front_door).await;

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind listener");
        let address = listener.local_addr().expect("local addr");
        let server_task = tokio::spawn(async move {
            let _ = axum::serve(listener, app).await;
        });
        wait_until_serving(address).await;

        Self {
            _db: disposable,
            address,
            store,
            db,
            auth,
            faults,
            server_task,
        }
    }

    async fn enroll_physician(
        &self,
        email: &str,
        display_name: &str,
        workspaces: &[&str],
    ) -> PhysicianIdentity {
        let now = real_now_unix();
        let invite = self
            .auth
            .invite(STAGING_PRACTICE_ID, email, display_name, false, now)
            .await
            .expect("invite physician");
        let (identity, _initial_session) = self
            .auth
            .redeem_invitation(email, &invite, now)
            .await
            .expect("redeem physician invitation");
        for ws in workspaces {
            self.auth
                .set_membership(&identity, ws, Membership::Allowed)
                .await
                .expect("grant panel membership");
        }
        identity
    }
}

async fn wait_until_serving(address: SocketAddr) {
    let client = reqwest::Client::new();
    let started = Instant::now();
    loop {
        assert!(
            started.elapsed() < Duration::from_secs(20),
            "server did not answer /health on {address}"
        );
        if let Ok(resp) = client
            .get(format!("http://{address}/health"))
            .timeout(Duration::from_secs(2))
            .send()
            .await
            && resp.status().is_success()
        {
            return;
        }
        tokio::time::sleep(Duration::from_millis(40)).await;
    }
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
            label: "Clinical finding".to_owned(),
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
        record_type: "Clinical observation".to_owned(),
        dates: vec![RecordDate {
            name: "effectiveDate".to_owned(),
            value: "2026-08-27".to_owned(),
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
            "2026-08-27T17:00:00Z".into(),
            slot.starts_at_unix.into(),
            "2026-08-27T17:30:00Z".into(),
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
            "2026-08-27T17:00:00Z".into(),
            "2026-08-27T17:30:00Z".into(),
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
            jsonb_expr(
                &serde_json::to_value(EvidenceScope::ClinicalChart {
                    connected_sources: Vec::new(),
                })
                .unwrap(),
            ),
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
    harness: &RunningHarness,
    scope: &Scope,
    question: &str,
    activity_at: i64,
) {
    let skills = PhysicianSkills::configured(None, false).expect("load workspace skills");
    let roles = test_model_roles();
    let recipe = harness
        .store
        .chat_recipe_with_understanding(
            &roles.chat,
            "Answer clinical questions from chart evidence.",
            &skills,
            &scope.practice,
            &scope.workspace,
        )
        .await
        .expect("build chat recipe");
    harness
        .store
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
    let row = harness
        .db
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
        .and_where(Expr::col(Alias::new("practice")).eq(scope.practice.as_str()))
        .and_where(Expr::col(Alias::new("physician")).eq(scope.physician.as_str()))
        .and_where(Expr::col(Alias::new("workspace")).eq(scope.workspace.as_str()))
        .and_where(Expr::col(Alias::new("chat")).eq(scope.chat.as_str()));
    exec_query(&harness.db, update).await;
}

async fn seed_practice_source_outage(
    harness: &RunningHarness,
    workspaces: &[&str],
    source_id_str: &str,
    source_label: &str,
    checked_at: i64,
    reason: &str,
) {
    let source_id = SourceId::new(source_id_str).expect("valid source id");
    let checker = FixedSourceCheck::new();
    for ws in workspaces {
        checker.push_step(
            STAGING_PRACTICE_ID,
            *ws,
            source_id.clone(),
            FixedCheckStep::Outcome(SourceCheckOutcome::Failed {
                source_label: Some(source_label.to_owned()),
                checked_at,
                reason: reason.to_owned(),
            }),
        );
    }
    for (idx, ws) in workspaces.iter().enumerate() {
        harness
            .store
            .refresh_patient_sources_for_practice(
                &PracticeScope {
                    practice: STAGING_PRACTICE_ID.to_owned(),
                    workspace: (*ws).to_owned(),
                    job_id: format!("job-outage-{source_id_str}-{idx}"),
                },
                harness.auth.as_ref(),
                &RefreshRequest::new(
                    RefreshCommandKey::new(format!("cmd-outage-{source_id_str}-{ws}")).unwrap(),
                    RefreshTarget::AllSources,
                ),
                &checker,
            )
            .await
            .expect("refresh patient sources for outage");
    }
}

fn build_site() -> PathBuf {
    BUILT_SITE
        .get_or_init(|| {
            let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
            let target = workspace.join("target/coda-physician-wasm");
            let wasm = target.join("wasm32-unknown-unknown/debug/physician-app.wasm");
            let mut cargo = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
            cargo_without_parent_package(&mut cargo);
            let status = cargo
                .current_dir(&workspace)
                .env("CARGO_TARGET_DIR", &target)
                .env_remove("PHYSICIAN_API_URL")
                .args([
                    "build",
                    "-p",
                    "physician-app",
                    "--target",
                    "wasm32-unknown-unknown",
                    "--features",
                    "web",
                ])
                .status()
                .expect("cargo build wasm");
            assert!(status.success(), "physician-app wasm build failed");
            let site = workspace.join("target/physician-web");
            fs::create_dir_all(&site).unwrap();
            let status = Command::new(wasm_bindgen())
                .args([
                    "--target",
                    "web",
                    "--out-dir",
                    site.to_str().unwrap(),
                    wasm.to_str().unwrap(),
                ])
                .status()
                .expect("wasm-bindgen");
            assert!(status.success(), "wasm-bindgen failed");
            site
        })
        .clone()
}

fn cargo_without_parent_package(command: &mut Command) {
    command.env_clear();
    for key in [
        "HOME",
        "USER",
        "RUSTUP_HOME",
        "CARGO_HOME",
        "RUSTUP_TOOLCHAIN",
        "TMPDIR",
    ] {
        if let Some(value) = env::var_os(key) {
            command.env(key, value);
        }
    }
    if let Some(paths) = env::var_os("PATH") {
        let filtered = env::join_paths(
            env::split_paths(&paths)
                .filter(|dir| !dir.to_string_lossy().contains(".local/share/swiftly")),
        )
        .expect("join PATH");
        command.env("PATH", filtered);
    }
}

fn wasm_bindgen() -> PathBuf {
    if let Some(path) = env::var_os("WASM_BINDGEN") {
        return PathBuf::from(path);
    }
    if let Some(paths) = env::var_os("PATH")
        && let Some(found) = env::split_paths(&paths)
            .map(|dir| dir.join("wasm-bindgen"))
            .find(|path| path.is_file())
    {
        return found;
    }
    let local = env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/bin/wasm-bindgen"));
    if let Some(path) = local.filter(|path| path.is_file()) {
        return path;
    }
    panic!("wasm-bindgen is required to build the physician page");
}

fn screenshot_dir() -> PathBuf {
    if let Some(folder) = env::var_os("PHYSICIAN_PROOF_SHOTS") {
        return PathBuf::from(folder);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/verification/screenshots/today-updates")
}

struct Tab {
    browser: Browser,
}

impl Tab {
    fn open(url: &str) -> Self {
        let mut browser = Browser::launch();
        browser
            .page()
            .call_session("Page.enable", serde_json::json!({}));
        browser
            .page()
            .call_session("Runtime.enable", serde_json::json!({}));
        let mut tab = Self { browser };
        tab.set_viewport(1440, 900, true);
        tab.browser.page().navigate(url);
        tab
    }

    fn eval(&mut self, expression: &str) -> serde_json::Value {
        self.browser.page().evaluate(expression)
    }

    fn call(&mut self, method: &str, params: serde_json::Value) -> serde_json::Value {
        self.browser.page().call_session(method, params)
    }

    fn step(&mut self, name: &str, args: serde_json::Value) -> serde_json::Value {
        let mut arguments = vec![serde_json::json!(name)];
        arguments.extend(args.as_array().expect("step args").iter().cloned());
        self.browser
            .page()
            .call_function(include_str!("js/site.js"), &arguments)
    }

    fn click(&mut self, selector: &str) {
        self.browser.click(selector);
    }

    fn click_named(&mut self, selector: &str, words: &str) {
        self.step("clickNamed", serde_json::json!([selector, words]));
    }

    fn sign_in_as(&mut self, email: &str) {
        self.wait_for_text("Work email", Duration::from_secs(20));
        self.browser.type_into("input[type=\"email\"]", email);
        std::thread::sleep(Duration::from_millis(60));
        self.click("button.control-primary");
    }

    fn wait_for_text(&mut self, expected: &str, timeout: Duration) -> String {
        let started = Instant::now();
        loop {
            let snapshot = self.eval(
                "({text: document.body?.innerText ?? '', html: document.documentElement?.outerHTML ?? ''})",
            );
            let html = snapshot["html"].as_str().unwrap_or_default();
            if snapshot["text"]
                .as_str()
                .is_some_and(|text| text.contains(expected))
            {
                return html.to_owned();
            }
            assert!(
                started.elapsed() < timeout,
                "timed out waiting for {expected:?} in visible text; DOM:\n{html}"
            );
            std::thread::sleep(Duration::from_millis(60));
        }
    }

    fn wait_until_absent(&mut self, forbidden: &str, timeout: Duration) {
        let started = Instant::now();
        loop {
            let text = self
                .eval("document.body?.innerText ?? ''")
                .as_str()
                .unwrap_or_default()
                .to_owned();
            if !text.contains(forbidden) {
                return;
            }
            assert!(
                started.elapsed() < timeout,
                "timed out waiting for {forbidden:?} to leave visible text; still showing:\n{text}"
            );
            std::thread::sleep(Duration::from_millis(60));
        }
    }

    fn reload(&mut self) {
        let previous = self.document_loader();
        self.call("Page.reload", serde_json::json!({}));
        let started = Instant::now();
        while self.document_loader() == previous {
            assert!(
                started.elapsed() < Duration::from_secs(10),
                "reload did not replace document"
            );
            std::thread::sleep(Duration::from_millis(25));
        }
    }

    fn document_loader(&mut self) -> String {
        self.call("Page.getFrameTree", serde_json::json!({}))
            .pointer("/result/frameTree/frame/loaderId")
            .and_then(|id| id.as_str())
            .expect("main frame loader id")
            .to_owned()
    }

    fn set_viewport(&mut self, width: u32, height: u32, reduced_motion: bool) {
        self.call(
            "Emulation.setDeviceMetricsOverride",
            serde_json::json!({
                "width": width,
                "height": height,
                "deviceScaleFactor": 1,
                "mobile": width < 600,
            }),
        );
        self.call(
            "Emulation.setEmulatedMedia",
            serde_json::json!({
                "features": [{
                    "name": "prefers-reduced-motion",
                    "value": if reduced_motion { "reduce" } else { "no-preference" },
                }]
            }),
        );
    }

    fn save_screenshot(&mut self, name: &str) -> PathBuf {
        let shot = self.call(
            "Page.captureScreenshot",
            serde_json::json!({ "format": "png" }),
        );
        let encoded = shot["result"]["data"].as_str().expect("screenshot data");
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .expect("decode screenshot png");
        let dir = screenshot_dir();
        fs::create_dir_all(&dir).expect("create screenshot dir");
        let path = dir.join(format!("{name}.png"));
        fs::write(&path, bytes).expect("write screenshot png");
        path
    }

    fn assert_no_horizontal_overflow(&mut self, context: &str) {
        let metrics = self.eval(
            "({scrollWidth: document.documentElement.scrollWidth, clientWidth: document.documentElement.clientWidth})",
        );
        let scroll_w = metrics["scrollWidth"].as_f64().unwrap_or(9999.0);
        let client_w = metrics["clientWidth"].as_f64().unwrap_or(0.0);
        assert!(
            scroll_w <= client_w + 1.0,
            "horizontal overflow on {context}: scrollWidth={scroll_w}, clientWidth={client_w}"
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn ac1_every_producer_kind_flat_order_and_open_lands_on_work_and_evidence() {
    let _guard = test_lock().lock().await;
    let harness = RunningHarness::start("ac1").await;

    let ws_elena = WORKSPACE_IDS[0];
    let ws_jake = WORKSPACE_IDS[1];
    let ws_maria = WORKSPACE_IDS[2];
    let ws_robert = WORKSPACE_IDS[3];
    let ws_mary = WORKSPACE_IDS[4];

    let dr_rivera = harness
        .enroll_physician(
            "dr.rivera@codahealth.test",
            "Dr. Rivera",
            &[ws_elena, ws_jake, ws_maria, ws_robert, ws_mary],
        )
        .await;

    // Multi-section >550-char chart record on Elena with `external_url` + structural `"attachment"` label.
    let bmp_head = "Serum potassium 6.2 mmol/L on morning basic metabolic panel (critical high; \
        reference range 3.5-5.1 mmol/L) drawn from non-hemolyzed peripheral venipuncture at 06:35 AM. \
        Serum sodium 136 mmol/L, chloride 103 mmol/L, bicarbonate 18 mmol/L, anion gap 15 mmol/L, \
        blood urea nitrogen 41 mg/dL, serum creatinine 2.2 mg/dL (baseline 1.4 mg/dL), calcium 9.1 mg/dL, \
        glucose 118 mg/dL, eGFR 26 mL/min/1.73m2.";
    let bmp_tail = "TAIL_MARKER_BEYOND_320_CHARS: Stat 12-lead ECG and repeat venous potassium \
        ordered; hold lisinopril 20 mg and spironolactone 25 mg pending immediate physician review.";
    let ref_elena = seed_chart_record_full(
        &harness.store,
        ws_elena,
        "Elena Morales",
        "obs-k-elena",
        "rev-1",
        vec![
            ReadablePart {
                label: "attachment".to_owned(),
                text: bmp_head.to_owned(),
            },
            ReadablePart {
                label: "Clinical comment".to_owned(),
                text: bmp_tail.to_owned(),
            },
        ],
        json!({
            "system": "regional-ehr",
            "id": "obs-k-elena",
            "title": "Morning Basic Metabolic Panel",
            "external_url": "https://ehr.codahealth.test/chart/obs-k-elena"
        }),
        CLINICAL_NOW_UNIX - 600,
    )
    .await;

    let ref_jake = seed_chart_record(
        &harness.store,
        ws_jake,
        "Jake Miller",
        "obs-a1c-jake",
        "rev-1",
        "Hemoglobin A1c 8.9%, increased from 7.1% ahead of afternoon diabetes visit.",
        CLINICAL_NOW_UNIX - 3_600,
    )
    .await;
    let _ref_maria = seed_chart_record(
        &harness.store,
        ws_maria,
        "Maria Santos",
        "obs-renal-maria",
        "rev-1",
        "External nephrology consult attachment pending potassium binder dose verification.",
        CLINICAL_NOW_UNIX - 10_800,
    )
    .await;
    let ref_robert = seed_chart_record(
        &harness.store,
        ws_robert,
        "Robert Chen",
        "obs-bp-robert",
        "rev-1",
        "Home blood pressure log averaging 118/74 mmHg over fourteen days.",
        CLINICAL_NOW_UNIX - 1_800,
    )
    .await;
    let _ref_mary = seed_chart_record(
        &harness.store,
        ws_mary,
        "Mary Johnson",
        "obs-cards-mary",
        "rev-1",
        "Cardiology stress echocardiogram awaiting outside report fetch.",
        CLINICAL_NOW_UNIX - 8_000,
    )
    .await;

    let origin = SourceWork::new("patient_review", "work-ac1", "att-1").unwrap();

    // 1. Open Now concern on Elena Morales (waited only 5 minutes -> STILL first!).
    seed_concern(
        &harness.store,
        &harness.db,
        TestConcern {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_elena.to_owned(),
            concern_id: "concern-now-k".to_owned(),
            urgency: ConcernUrgency::Now,
            standing: TestConcernStanding::Open,
            reason: "Critical hyperkalemia (K 6.2 mmol/L) on morning BMP.".to_owned(),
            records: vec![ref_elena.clone()],
            source_work: origin.clone(),
            created_at: CLINICAL_NOW_UNIX - 300,
            updated_at: CLINICAL_NOW_UNIX - 300,
            visit_slot: None,
        },
    )
    .await;

    // 2. TWO unhandled items on Maria Santos (`ws_maria`):
    //    - Failed review WITH unfinished work (`now - 10_800`)
    //    - CouldNotFinish visit preparation (`now - 9_600`)
    //    Proves Today collapses multiple unhandled failures on one patient into ONE `unhandled:<ws_maria>` row!
    seed_review(
        &harness.db,
        TestReview {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_maria.to_owned(),
            review_id: "rev-failed-unhandled".to_owned(),
            triggers: vec![ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                changed_candidates: vec!["obs-renal-maria".to_owned()],
            }],
            unfinished_work: vec![UnfinishedReviewWork {
                topic: "External nephrology consult".to_owned(),
                detail: "Verify potassium binder dose in scanned consult attachment".to_owned(),
                limit: None,
            }],
            state: ReviewLifecycleState::Failed {
                reason: "Provider budget exhausted".to_owned(),
                failed_at: CLINICAL_NOW_UNIX - 10_800,
                attempts: 3,
            },
            evidence_read: Vec::new(),
            queued_at: CLINICAL_NOW_UNIX - 10_900,
            updated_at: CLINICAL_NOW_UNIX - 10_800,
        },
    )
    .await;

    seed_visit_preparation(
        &harness.db,
        TestVisitPreparation {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_maria.to_owned(),
            source_id: "practice-sched".to_owned(),
            appointment_id: "appt-maria-collapse".to_owned(),
            starts_at_unix: CLINICAL_NOW_UNIX + 9_000,
            ends_at_unix: CLINICAL_NOW_UNIX + 10_800,
            state: TestVisitPrepState::CouldNotFinish {
                reason: "Dialysis flowsheet timed out before nephrology visit".to_owned(),
            },
            updated_at: CLINICAL_NOW_UNIX - 9_600,
        },
    )
    .await;

    // 3. Upcoming visit preparation CouldNotFinish on `ws_mary` (waited 2 hours -> `unhandled:<ws_mary>`).
    seed_visit_preparation(
        &harness.db,
        TestVisitPreparation {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_mary.to_owned(),
            source_id: "practice-sched".to_owned(),
            appointment_id: "appt-mary-upcoming".to_owned(),
            starts_at_unix: CLINICAL_NOW_UNIX + 7_200,
            ends_at_unix: CLINICAL_NOW_UNIX + 9_000,
            state: TestVisitPrepState::CouldNotFinish {
                reason: "Could not synthesize cardiology stress echo before visit".to_owned(),
            },
            updated_at: CLINICAL_NOW_UNIX - 7_200,
        },
    )
    .await;

    // 4. Failed InitialUnderstanding on `ws_robert` (waited 90 minutes -> `unhandled:<ws_robert>`).
    seed_failed_initial_understanding(
        &harness.store,
        &harness.db,
        harness.auth.as_ref(),
        STAGING_PRACTICE_ID,
        ws_robert,
        "init-robert",
        CLINICAL_NOW_UNIX - 5_400,
    )
    .await;

    // 5. Open Today pre-visit nudge + saved VisitBriefing on Jake Miller (waited 1 hour).
    seed_visit_briefing(
        &harness.db,
        TestVisitBriefing {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_jake.to_owned(),
            source_id: "practice-sched".to_owned(),
            appointment_id: "appt-jake-1".to_owned(),
            starts_at_unix: CLINICAL_NOW_UNIX + 3_600,
            ends_at_unix: CLINICAL_NOW_UNIX + 5_400,
            briefing_id: "briefing-jake-1".to_owned(),
            orientation: "Diabetes follow-up with rising A1c (8.9%).".to_owned(),
            narrative: "Hemoglobin A1c rose from 7.1% to 8.9% despite metformin.".to_owned(),
            basis_and_limitations: "Regional EHR laboratory flowsheet through Aug 27, 2026."
                .to_owned(),
            citations: vec![SummaryCitation {
                reference: ref_jake.clone(),
                source_id: "regional-ehr".to_owned(),
                source_label: "Regional EHR".to_owned(),
                sha256: "deadbeef".to_owned(),
                display_label: "Hemoglobin A1c".to_owned(),
                as_of: Some("2026-08-27".to_owned()),
                excerpt:
                    "Hemoglobin A1c 8.9%, increased from 7.1% ahead of afternoon diabetes visit."
                        .to_owned(),
                native_uri: None,
            }],
            prepared_at: CLINICAL_NOW_UNIX - 3_600,
            source_version: 1,
        },
    )
    .await;

    seed_concern(
        &harness.store,
        &harness.db,
        TestConcern {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_jake.to_owned(),
            concern_id: "concern-visit-a1c".to_owned(),
            urgency: ConcernUrgency::Today,
            standing: TestConcernStanding::Open,
            reason: "A1c rose to 8.9% ahead of today's diabetes visit.".to_owned(),
            records: vec![ref_jake.clone()],
            source_work: origin.clone(),
            created_at: CLINICAL_NOW_UNIX - 3_600,
            updated_at: CLINICAL_NOW_UNIX - 3_600,
            visit_slot: Some(TodayVisitSlot {
                source_id: "practice-sched".to_owned(),
                appointment_id: "appt-jake-1".to_owned(),
                starts_at_unix: CLINICAL_NOW_UNIX + 3_600,
                ends_at_unix: CLINICAL_NOW_UNIX + 5_400,
            }),
        },
    )
    .await;

    // 6. Active physician conversation response on Jake Miller (waited 30 minutes).
    seed_running_conversation(
        &harness,
        &Scope {
            practice: STAGING_PRACTICE_ID.to_owned(),
            physician: dr_rivera.physician_id.clone(),
            workspace: ws_jake.to_owned(),
            chat: "chat-jake-renal".to_owned(),
        },
        "Compare eGFR slope over the last year",
        CLINICAL_NOW_UNIX - 1_800,
    )
    .await;

    // 7. Quiet review, clean failed review, physician-stopped review, retired concern,
    //    assessed review, skipped visit prep, patient summary, and routine running review ->
    //    reach Updates (or stay on patient page), NEVER Today!
    seed_review(
        &harness.db,
        TestReview {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_robert.to_owned(),
            review_id: "rev-quiet-robert".to_owned(),
            triggers: vec![ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                last_reviewed_at: None,
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Completed {
                outcome: ReviewOutcome::Quiet {
                    reason: QuietReviewReason::NothingToDo,
                    explanation: "Home BP log remains well controlled at 118/74 mmHg.".to_owned(),
                    citations: vec![ReviewCitation {
                        reference: ref_robert.clone(),
                        source_id: "regional-ehr".to_owned(),
                        source_label: "Regional EHR".to_owned(),
                        sha256: "abcd".to_owned(),
                        display_label: "Home BP Log".to_owned(),
                        as_of: Some("2026-08-27".to_owned()),
                        excerpt:
                            "Home blood pressure log averaging 118/74 mmHg over fourteen days."
                                .to_owned(),
                    }],
                    basis_and_limitations: "Regional EHR home BP flowsheet.".to_owned(),
                },
                completed_at: CLINICAL_NOW_UNIX - 1_200,
                completed_source_version: 1,
            },
            evidence_read: vec![ref_robert.clone()],
            queued_at: CLINICAL_NOW_UNIX - 1_300,
            updated_at: CLINICAL_NOW_UNIX - 1_200,
        },
    )
    .await;

    seed_review(
        &harness.db,
        TestReview {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_robert.to_owned(),
            review_id: "rev-clean-failed-robert".to_owned(),
            triggers: vec![ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                last_reviewed_at: None,
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Failed {
                reason: "ROUTINE_CLEAN_FAILURE_NO_TODAY_ROW".to_owned(),
                failed_at: CLINICAL_NOW_UNIX - 1_500,
                attempts: 3,
            },
            evidence_read: Vec::new(),
            queued_at: CLINICAL_NOW_UNIX - 1_600,
            updated_at: CLINICAL_NOW_UNIX - 1_500,
        },
    )
    .await;

    seed_review(
        &harness.db,
        TestReview {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_elena.to_owned(),
            review_id: "rev-stopped-by-physician".to_owned(),
            triggers: vec![ReviewTrigger::PhysicianRequest {
                physician_id: dr_rivera.physician_id.clone(),
                request_text: "Check outpatient potassium trend".to_owned(),
                workspace_version: 1,
                record_versions: BTreeMap::new(),
            }],
            unfinished_work: vec![UnfinishedReviewWork {
                topic: "PHYSICIAN_STOPPED_REVIEW_NO_TODAY_ROW".to_owned(),
                detail: "Stopped intentionally by physician".to_owned(),
                limit: None,
            }],
            state: ReviewLifecycleState::Held {
                reason: ReviewHoldReason::Stopped,
                held_at: CLINICAL_NOW_UNIX - 12_000,
                partial_text: Some("Paused by physician.".to_owned()),
            },
            evidence_read: Vec::new(),
            queued_at: CLINICAL_NOW_UNIX - 12_100,
            updated_at: CLINICAL_NOW_UNIX - 12_000,
        },
    )
    .await;

    seed_concern(
        &harness.store,
        &harness.db,
        TestConcern {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_robert.to_owned(),
            concern_id: "concern-retired-bp".to_owned(),
            urgency: ConcernUrgency::Today,
            standing: TestConcernStanding::Retired {
                reason: "Repeat clinic and home blood pressures normalized.".to_owned(),
                retired_at: CLINICAL_NOW_UNIX - 900,
            },
            reason: "Borderline elevated office blood pressure last month.".to_owned(),
            records: vec![ref_robert.clone()],
            source_work: origin,
            created_at: CLINICAL_NOW_UNIX - 20_000,
            updated_at: CLINICAL_NOW_UNIX - 900,
            visit_slot: None,
        },
    )
    .await;

    seed_review(
        &harness.db,
        TestReview {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_elena.to_owned(),
            review_id: "rev-assessed-elena".to_owned(),
            triggers: vec![ReviewTrigger::ChangedRecords {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                changed_candidates: vec!["obs-k-elena".to_owned()],
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Completed {
                outcome: ReviewOutcome::Assessed {
                    summary: "Assessed critical morning potassium 6.2 mmol/L.".to_owned(),
                    citations: Vec::new(),
                    basis_and_limitations: "Regional EHR basic metabolic panel.".to_owned(),
                },
                completed_at: CLINICAL_NOW_UNIX - 280,
                completed_source_version: 1,
            },
            evidence_read: vec![ref_elena],
            queued_at: CLINICAL_NOW_UNIX - 295,
            updated_at: CLINICAL_NOW_UNIX - 280,
        },
    )
    .await;

    seed_visit_preparation(
        &harness.db,
        TestVisitPreparation {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_robert.to_owned(),
            source_id: "practice-sched".to_owned(),
            appointment_id: "appt-robert-flu".to_owned(),
            starts_at_unix: CLINICAL_NOW_UNIX + 10_800,
            ends_at_unix: CLINICAL_NOW_UNIX + 11_700,
            state: TestVisitPrepState::Skipped {
                reason: "Nurse-only influenza vaccination visit needs no chart briefing."
                    .to_owned(),
            },
            updated_at: CLINICAL_NOW_UNIX - 700,
        },
    )
    .await;

    let _ = ref_robert;
    seed_patient_summary(
        &harness.db,
        STAGING_PRACTICE_ID,
        ws_jake,
        1,
        1,
        &PatientSummary {
            narrative: "Well-controlled hypertension on lisinopril 10 mg daily.".to_owned(),
            claims: vec![SummaryClaimInput {
                text: "Home BP averaging 118/74 mmHg.".to_owned(),
                citations: vec![ref_jake],
            }],
            material_gaps: Vec::new(),
            scope_note: None,
            citations: Vec::new(),
            segments: vec![SummarySegment::Prose {
                text: "Well-controlled hypertension on lisinopril 10 mg daily.".to_owned(),
            }],
        },
        CLINICAL_NOW_UNIX - 650,
    )
    .await;

    seed_review(
        &harness.db,
        TestReview {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_jake.to_owned(),
            review_id: "rev-routine-running".to_owned(),
            triggers: vec![ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                last_reviewed_at: None,
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Running {
                attempt_id: "att-bg-1".to_owned(),
                started_at: CLINICAL_NOW_UNIX - 90,
            },
            evidence_read: Vec::new(),
            queued_at: CLINICAL_NOW_UNIX - 100,
            updated_at: CLINICAL_NOW_UNIX - 90,
        },
    )
    .await;

    let base = format!("http://{}", harness.address);
    let mut tab = Tab::open(&base);
    tab.sign_in_as("dr.rivera@codahealth.test");
    tab.wait_for_text(
        "Critical hyperkalemia (K 6.2 mmol/L) on morning BMP.",
        Duration::from_secs(20),
    );

    // Verify Today is ONE flat list, every row names its patient, Now comes first, and the rest
    // are ordered strictly by how long they have waited (with Maria's 2 unhandled failures collapsed
    // into one `unhandled:<ws_maria>` row, and Elena's physician-stopped review excluded!).
    let list_count = tab.eval(
        "document.querySelectorAll('section[aria-label=\"Needs you\"] > .today-list').length",
    );
    assert_eq!(list_count, json!(1), "Today must render one flat list");

    let rows_snapshot = tab.eval(
        "[...document.querySelectorAll('section[aria-label=\"Needs you\"] [data-today-item]')].map(el => ({\
            id: el.getAttribute('data-today-item'),\
            who: el.querySelector('.today-row-who')?.textContent?.trim() ?? '',\
            now: !!el.querySelector('.urgency-now')\
        }))",
    );
    let rows = rows_snapshot.as_array().expect("today rows array");
    let item_ids: Vec<&str> = rows
        .iter()
        .map(|r| r["id"].as_str().unwrap_or_default())
        .collect();
    assert_eq!(
        item_ids,
        vec![
            format!("concern:{ws_elena}:concern-now-k"),
            format!("unhandled:{ws_maria}"),
            format!("unhandled:{ws_mary}"),
            format!("unhandled:{ws_robert}"),
            format!("concern:{ws_jake}:concern-visit-a1c"),
            format!("running-chat:{ws_jake}:chat-jake-renal"),
        ],
        "Today rows must place Now first, collapse unhandled work to at most one row per patient, and order by longest wait: {rows_snapshot}"
    );
    assert_eq!(rows[0]["now"], json!(true));
    for row in rows {
        let who = row["who"].as_str().unwrap_or_default();
        assert!(
            !who.is_empty(),
            "every Today row must name its patient: {row}"
        );
    }

    let today_text = tab
        .eval("document.body.innerText")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        !today_text.contains("Home BP log remains well controlled")
            && !today_text.contains("ROUTINE_CLEAN_FAILURE_NO_TODAY_ROW")
            && !today_text.contains("PHYSICIAN_STOPPED_REVIEW_NO_TODAY_ROW")
            && !today_text.contains("Borderline elevated office blood pressure"),
        "quiet review, clean failed review, physician-stopped review, and retired concern must not appear on Today:\n{today_text}"
    );

    // Click Elena's Now row -> lands on Elena's workspace AND shows work + chart evidence!
    let elena_item = format!("concern:{ws_elena}:concern-now-k");
    tab.click(&format!(
        "[data-today-item=\"{elena_item}\"] button.today-row-open"
    ));
    tab.wait_for_text(
        "Serum potassium 6.2 mmol/L on morning basic metabolic panel",
        Duration::from_secs(15),
    );
    let open_patient = tab.eval("new URL(location.href).searchParams.get('patient')");
    assert_eq!(open_patient, json!(ws_elena));
    let evidence_text = tab
        .eval(&format!(
            "document.querySelector('[data-work-evidence=\"{elena_item}\"]')?.innerText ?? ''"
        ))
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        evidence_text.contains("Critical hyperkalemia (K 6.2 mmol/L) on morning BMP.")
            && evidence_text.contains("Chart evidence")
            && evidence_text.contains("Serum potassium 6.2 mmol/L"),
        "opening Today row did not display its work and chart citation:\n{evidence_text}"
    );

    // Click citation `[1]` inside `[data-work-evidence]` -> `ReaderDialog` renders FULL untruncated
    // multi-section record body (>500 chars), omits raw `"attachment"`, AND shows `Open in Regional EHR`!
    tab.click("[data-citation-candidate=\"obs-k-elena\"] button");
    tab.wait_for_text("TAIL_MARKER_BEYOND_320_CHARS", Duration::from_secs(10));
    let reader_snapshot = tab.eval(
        "({text: document.querySelector('.citation-inspection')?.innerText ?? '', \
           href: document.querySelector('.citation-inspection a.citation-open')?.getAttribute('href') ?? '', \
           linkText: document.querySelector('.citation-inspection a.citation-open')?.textContent?.trim() ?? ''})",
    );
    let reader_text = reader_snapshot["text"].as_str().unwrap_or_default();
    assert!(
        reader_text.contains("TAIL_MARKER_BEYOND_320_CHARS")
            && !reader_text.lines().any(|line| line.trim() == "attachment"),
        "ReaderDialog must render full untruncated record text without raw `attachment` label:\n{reader_text}"
    );
    assert_eq!(
        reader_snapshot["href"],
        json!("https://ehr.codahealth.test/chart/obs-k-elena")
    );
    assert_eq!(reader_snapshot["linkText"], json!("Open in Regional EHR"));
    tab.click(".citation-inspection button[aria-label=\"Close\"]");
    tab.wait_until_absent("TAIL_MARKER_BEYOND_320_CHARS", Duration::from_secs(10));

    // Navigate to Updates -> verify every producer kind reached Updates with clean non-duplicated headlines!
    tab.click_named(".sidebar-nav-item", "Updates");
    tab.wait_for_text("What Coda did today", Duration::from_secs(15));
    tab.wait_for_text("Reviewed, nothing for you", Duration::from_secs(10));
    let updates_text = tab
        .eval("document.body.innerText")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    for expected in [
        "Running now",
        "Assessed critical morning potassium 6.2 mmol/L.",
        "Repeat clinic and home blood pressures normalized.",
        "Diabetes follow-up with rising A1c (8.9%).",
        "Nurse-only influenza vaccination visit needs no chart briefing.",
        "Well-controlled hypertension on lisinopril 10 mg daily.",
        "Home BP log remains well controlled at 118/74 mmHg.",
    ] {
        assert!(
            updates_text.contains(expected),
            "Updates surface missing expected producer entry {expected:?}:\n{updates_text}"
        );
    }
    for forbidden_doubled in [
        "Concern raised:",
        "Retired concern:",
        "Reviewed chart — nothing for you",
        "Chart review:",
        "Visit briefing prepared:",
        "Prepared patient summary",
    ] {
        assert!(
            !updates_text.contains(forbidden_doubled),
            "Updates surface must not repeat category prefix {forbidden_doubled:?} in card headline:\n{updates_text}"
        );
    }

    let briefing_item = format!("briefing:{ws_jake}:briefing-jake-1");
    tab.click(&format!(
        "[data-update-item=\"{briefing_item}\"] button.today-row-open"
    ));
    tab.wait_for_text(
        "Hemoglobin A1c 8.9%, increased from 7.1% ahead of afternoon diabetes visit.",
        Duration::from_secs(15),
    );
    assert_eq!(
        tab.eval("new URL(location.href).searchParams.get('patient')"),
        json!(ws_jake)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn ac2_two_physicians_two_concerns_seen_state_revision_and_done() {
    let _guard = test_lock().lock().await;
    let harness = RunningHarness::start("ac2").await;

    let ws_elena = WORKSPACE_IDS[0];
    let ws_jake = WORKSPACE_IDS[1];

    harness
        .enroll_physician(
            "dr.patel@codahealth.test",
            "Dr. Patel",
            &[ws_elena, ws_jake],
        )
        .await;
    harness
        .enroll_physician(
            "dr.nguyen@codahealth.test",
            "Dr. Nguyen",
            &[ws_elena, ws_jake],
        )
        .await;

    let ref_elena = seed_chart_record(
        &harness.store,
        ws_elena,
        "Elena Morales",
        "obs-k-ac2",
        "rev-1",
        "Potassium 6.2 mmol/L on stat basic metabolic panel.",
        CLINICAL_NOW_UNIX - 500,
    )
    .await;
    let ref_jake = seed_chart_record(
        &harness.store,
        ws_jake,
        "Jake Miller",
        "obs-inr-ac2",
        "rev-1",
        "INR 3.9 above target range 2.0-3.0 on warfarin.",
        CLINICAL_NOW_UNIX - 1_200,
    )
    .await;

    let origin = SourceWork::new("patient_review", "work-ac2", "att-1").unwrap();
    seed_concern(
        &harness.store,
        &harness.db,
        TestConcern {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_elena.to_owned(),
            concern_id: "concern-a".to_owned(),
            urgency: ConcernUrgency::Now,
            standing: TestConcernStanding::Open,
            reason: "Stat potassium 6.2 mmol/L requires immediate repeat.".to_owned(),
            records: vec![ref_elena.clone()],
            source_work: origin.clone(),
            created_at: CLINICAL_NOW_UNIX - 300,
            updated_at: CLINICAL_NOW_UNIX - 300,
            visit_slot: None,
        },
    )
    .await;
    seed_concern(
        &harness.store,
        &harness.db,
        TestConcern {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_jake.to_owned(),
            concern_id: "concern-b".to_owned(),
            urgency: ConcernUrgency::Today,
            standing: TestConcernStanding::Open,
            reason: "Supratherapeutic INR 3.9 needs warfarin dose hold review.".to_owned(),
            records: vec![ref_jake],
            source_work: origin,
            created_at: CLINICAL_NOW_UNIX - 1_200,
            updated_at: CLINICAL_NOW_UNIX - 1_200,
            visit_slot: None,
        },
    )
    .await;

    let base = format!("http://{}", harness.address);
    let mut tab_patel = Tab::open(&base);
    tab_patel.sign_in_as("dr.patel@codahealth.test");
    tab_patel.wait_for_text(
        "Stat potassium 6.2 mmol/L requires immediate repeat.",
        Duration::from_secs(20),
    );

    let mut tab_nguyen = Tab::open(&base);
    tab_nguyen.sign_in_as("dr.nguyen@codahealth.test");
    tab_nguyen.wait_for_text(
        "Stat potassium 6.2 mmol/L requires immediate repeat.",
        Duration::from_secs(20),
    );

    assert_eq!(
        tab_patel.eval("!!document.querySelector('.now-banner')"),
        json!(true)
    );
    tab_patel.reload();
    tab_patel.wait_for_text(
        "Stat potassium 6.2 mmol/L requires immediate repeat.",
        Duration::from_secs(20),
    );
    assert_eq!(
        tab_patel.eval("!!document.querySelector('.now-banner')"),
        json!(true),
        "Now concern must remain active across reload until opened"
    );

    let item_a = format!("concern:{ws_elena}:concern-a");
    let item_b = format!("concern:{ws_jake}:concern-b");
    tab_patel.click(&format!(
        "[data-today-item=\"{item_a}\"] button.today-row-open"
    ));
    tab_patel.wait_for_text(
        "Potassium 6.2 mmol/L on stat basic metabolic panel.",
        Duration::from_secs(15),
    );
    assert_eq!(
        tab_patel.eval("!!document.querySelector('.now-banner')"),
        json!(false),
        "Now banner must clear for Dr. Patel once opened"
    );

    tab_patel.reload();
    tab_patel.wait_for_text("Tanvir Chowdhury", Duration::from_secs(20));
    assert_eq!(
        tab_patel.eval("!!document.querySelector('.now-banner')"),
        json!(false),
        "Dr. Patel's seen state must persist across reload"
    );

    tab_nguyen.reload();
    tab_nguyen.wait_for_text(
        "Stat potassium 6.2 mmol/L requires immediate repeat.",
        Duration::from_secs(20),
    );
    assert_eq!(
        tab_nguyen.eval("!!document.querySelector('.now-banner')"),
        json!(true),
        "Opening Concern A as Dr. Patel must not mark Dr. Nguyen's seen state"
    );

    // Revise Concern A through `Store::revise_concern` -> returns as changed for Dr. Patel!
    revise_concern_at(
        &harness.store,
        &harness.db,
        STAGING_PRACTICE_ID,
        ws_elena,
        "concern-a",
        ConcernUrgency::Now,
        "Repeat potassium rose to 6.5 mmol/L with peaked T waves on ECG.",
        vec![ref_elena],
        CLINICAL_NOW_UNIX - 30,
    )
    .await;

    tab_patel.click("button[aria-label=\"Open Today\"]");
    tab_patel.reload();
    tab_patel.wait_for_text(
        "Repeat potassium rose to 6.5 mmol/L with peaked T waves on ECG.",
        Duration::from_secs(20),
    );
    assert_eq!(
        tab_patel.eval("!!document.querySelector('.now-banner')"),
        json!(true),
        "A seen Now concern that changes must surface again"
    );
    let changed_badge = tab_patel
        .eval(&format!(
            "document.querySelector('[data-today-item=\"{item_a}\"] [data-changed-since-seen=\"true\"]')?.textContent?.trim() ?? ''"
        ))
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        changed_badge.contains("Updated ·") && changed_badge.contains("6.5 mmol/L"),
        "Revised concern must state what changed since Dr. Patel opened it; got: {changed_badge:?}"
    );

    tab_patel.click(&format!("button[data-today-done=\"{item_a}\"]"));
    tab_patel.wait_until_absent(
        "Repeat potassium rose to 6.5 mmol/L with peaked T waves on ECG.",
        Duration::from_secs(15),
    );
    let remaining_ids = tab_patel.eval(
        "[...document.querySelectorAll('section[aria-label=\"Needs you\"] [data-today-item]')].map(e => e.getAttribute('data-today-item'))",
    );
    assert_eq!(
        remaining_ids,
        json!([item_b]),
        "Done on Concern A must clear only Concern A and keep Concern B"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn ac3_partial_failure_practice_outage_failed_open_retry_and_nothing_needs_you_guard() {
    let _guard = test_lock().lock().await;
    let harness = RunningHarness::start("ac3").await;

    let ws_elena = WORKSPACE_IDS[0];
    let ws_jake = WORKSPACE_IDS[1];
    let ws_mary = WORKSPACE_IDS[4];

    harness
        .enroll_physician(
            "dr.okafor@codahealth.test",
            "Dr. Okafor",
            &[ws_elena, ws_jake],
        )
        .await;
    harness
        .enroll_physician("dr.empty@codahealth.test", "Dr. Empty", &[ws_mary])
        .await;

    let ref_elena = seed_chart_record(
        &harness.store,
        ws_elena,
        "Elena Morales",
        "obs-troponin-1",
        "rev-1",
        "High-sensitivity troponin I 142 ng/L (elevated).",
        CLINICAL_NOW_UNIX - 400,
    )
    .await;
    let ref_jake = seed_chart_record(
        &harness.store,
        ws_jake,
        "Jake Miller",
        "obs-bnp-1",
        "rev-1",
        "NT-proBNP 1,850 pg/mL with mild lower-extremity edema.",
        CLINICAL_NOW_UNIX - 800,
    )
    .await;

    let origin = SourceWork::new("patient_review", "work-ac3", "att-1").unwrap();
    seed_concern(
        &harness.store,
        &harness.db,
        TestConcern {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_elena.to_owned(),
            concern_id: "concern-troponin".to_owned(),
            urgency: ConcernUrgency::Today,
            standing: TestConcernStanding::Open,
            reason: "Elevated hs-troponin I 142 ng/L on chest pain panel.".to_owned(),
            records: vec![ref_elena],
            source_work: origin.clone(),
            created_at: CLINICAL_NOW_UNIX - 400,
            updated_at: CLINICAL_NOW_UNIX - 400,
            visit_slot: None,
        },
    )
    .await;
    seed_concern(
        &harness.store,
        &harness.db,
        TestConcern {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_jake.to_owned(),
            concern_id: "concern-bnp".to_owned(),
            urgency: ConcernUrgency::Today,
            standing: TestConcernStanding::Open,
            reason: "Elevated NT-proBNP 1,850 pg/mL ahead of cardiology check.".to_owned(),
            records: vec![ref_jake],
            source_work: origin,
            created_at: CLINICAL_NOW_UNIX - 800,
            updated_at: CLINICAL_NOW_UNIX - 800,
            visit_slot: None,
        },
    )
    .await;

    // 1. Force `PatientReviews` producer failure BEFORE opening Today.
    harness.faults.fail_producer(ProducerKind::PatientReviews);

    let base = format!("http://{}", harness.address);
    let mut tab_okafor = Tab::open(&base);
    tab_okafor.sign_in_as("dr.okafor@codahealth.test");
    tab_okafor.wait_for_text(
        "Elevated hs-troponin I 142 ng/L on chest pain panel.",
        Duration::from_secs(20),
    );
    tab_okafor.wait_for_text("patient reviews", Duration::from_secs(10));
    assert_eq!(
        tab_okafor
            .eval("document.querySelectorAll('[data-today-partial-failure=\"true\"]').length"),
        json!(1),
        "partial producer failure must render one partial-failure line"
    );
    assert_eq!(
        tab_okafor.eval("document.querySelectorAll('[data-today-partial-retry=\"true\"]').length"),
        json!(1),
        "partial producer failure must offer one Retry button"
    );

    let mut tab_empty = Tab::open(&base);
    tab_empty.sign_in_as("dr.empty@codahealth.test");
    tab_empty.wait_for_text("patient reviews", Duration::from_secs(20));
    let empty_partial_text = tab_empty
        .eval("document.body.innerText")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        !empty_partial_text.contains("Nothing needs you right now."),
        "\"Nothing needs you\" must NOT show when a producer failed:\n{empty_partial_text}"
    );

    harness.faults.clear_producer_failures();
    tab_empty.click("button[data-today-partial-retry=\"true\"]");
    tab_empty.wait_for_text("Nothing needs you right now.", Duration::from_secs(15));
    assert_eq!(
        tab_empty.eval("document.querySelectorAll('[data-today-partial-failure=\"true\"]').length"),
        json!(0)
    );

    tab_okafor.click("button[data-today-partial-retry=\"true\"]");
    tab_okafor.wait_until_absent("couldn’t load (patient reviews)", Duration::from_secs(15));
    assert_eq!(
        tab_okafor
            .eval("document.querySelectorAll('[data-today-partial-failure=\"true\"]').length"),
        json!(0)
    );

    // 2. Seed a practice-wide source outage (`quest-labs`) via `Store::refresh_patient_sources_for_practice`!
    seed_practice_source_outage(
        &harness,
        &[ws_elena, ws_jake, ws_mary],
        "quest-labs",
        "Quest Diagnostics",
        CLINICAL_NOW_UNIX - 60,
        "upstream gateway 503",
    )
    .await;

    tab_okafor.reload();
    tab_okafor.wait_for_text("Quest Diagnostics", Duration::from_secs(20));
    assert_eq!(
        tab_okafor.eval("document.querySelectorAll('[data-practice-outage]').length"),
        json!(1),
        "practice-wide outage across multiple patients must render as one line"
    );
    let outage_text = tab_okafor
        .eval("document.querySelector('[data-practice-outage]')?.innerText ?? ''")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert_eq!(
        outage_text
            .matches("is unavailable across the practice")
            .count(),
        1,
        "practice outage banner must state `is unavailable across the practice` once without doubling: {outage_text:?}"
    );
    assert!(
        outage_text.contains("Quest Diagnostics is unavailable across the practice (2 patients): upstream gateway 503"),
        "unexpected outage banner sentence: {outage_text:?}"
    );

    tab_empty.reload();
    tab_empty.wait_for_text("Quest Diagnostics", Duration::from_secs(20));
    assert!(
        !tab_empty
            .eval("document.body.innerText")
            .as_str()
            .unwrap_or_default()
            .contains("Nothing needs you right now."),
        "\"Nothing needs you\" must not render during a practice-wide source outage"
    );

    // 3. Failed item open preserves older content on screen while offering `Retry`!
    let item_elena = format!("concern:{ws_elena}:concern-troponin");
    let item_jake = format!("concern:{ws_jake}:concern-bnp");
    tab_okafor.click(&format!(
        "[data-today-item=\"{item_elena}\"] button.today-row-open"
    ));
    tab_okafor.wait_for_text(
        "High-sensitivity troponin I 142 ng/L (elevated).",
        Duration::from_secs(15),
    );

    harness.faults.fail_open_item(&item_jake);
    tab_okafor.click("button[aria-label=\"Open Today\"]");
    tab_okafor.wait_for_text(
        "Elevated NT-proBNP 1,850 pg/mL ahead of cardiology check.",
        Duration::from_secs(15),
    );
    tab_okafor.click(&format!(
        "[data-today-item=\"{item_jake}\"] button.today-row-open"
    ));
    tab_okafor.wait_for_text("Existing content is still shown.", Duration::from_secs(15));
    assert_eq!(
        tab_okafor.eval("!!document.querySelector('[data-item-open-failure=\"true\"]')"),
        json!(true)
    );
    assert_eq!(
        tab_okafor.eval("!!document.querySelector('[data-item-open-retry=\"true\"]')"),
        json!(true)
    );
    assert!(
        tab_okafor
            .eval("document.body.innerText")
            .as_str()
            .unwrap_or_default()
            .contains("Elevated hs-troponin I 142 ng/L on chest pain panel."),
        "failed item open must preserve existing content on screen"
    );

    harness.faults.clear_open_failures();
    tab_okafor.click("button[data-item-open-retry=\"true\"]");
    tab_okafor.wait_for_text(
        "NT-proBNP 1,850 pg/mL with mild lower-extremity edema.",
        Duration::from_secs(15),
    );
    assert_eq!(
        tab_okafor.eval("new URL(location.href).searchParams.get('patient')"),
        json!(ws_jake)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn ac4_and_ac5_patient_page_rows_updates_groups_and_desktop_phone_screenshots() {
    let _guard = test_lock().lock().await;
    let harness = RunningHarness::start("ac4_ac5").await;

    let ws_elena = WORKSPACE_IDS[0];
    let ws_jake = WORKSPACE_IDS[1];
    let ws_maria = WORKSPACE_IDS[2];
    let ws_robert = WORKSPACE_IDS[3];

    harness
        .enroll_physician(
            "dr.chen@codahealth.test",
            "Dr. Lin",
            &[ws_elena, ws_jake, ws_maria, ws_robert],
        )
        .await;

    let ref_elena = seed_chart_record(
        &harness.store,
        ws_elena,
        "Elena Morales",
        "obs-k-ac4",
        "rev-1",
        "Potassium 5.8 mmol/L with lisinopril 20 mg active medication.",
        CLINICAL_NOW_UNIX - 900,
    )
    .await;
    let ref_jake = seed_chart_record(
        &harness.store,
        ws_jake,
        "Jake Miller",
        "obs-ldl-ac4",
        "rev-1",
        "LDL cholesterol 164 mg/dL off statin therapy.",
        CLINICAL_NOW_UNIX - 2_400,
    )
    .await;
    let ref_robert = seed_chart_record(
        &harness.store,
        ws_robert,
        "Robert Chen",
        "obs-bp-ac5",
        "rev-1",
        "Ambulatory blood pressure average 119/75 mmHg.",
        CLINICAL_NOW_UNIX - 1_500,
    )
    .await;

    // Seed a practice-wide external lab outage via `Store::refresh_patient_sources_for_practice`
    // so `09-today-desktop.png` and `10-today-phone.png` visually prove clean non-duplicated outage prose!
    seed_practice_source_outage(
        &harness,
        &[ws_elena, ws_jake, ws_maria, ws_robert],
        "quest-labs",
        "Quest Diagnostics",
        CLINICAL_NOW_UNIX - 180,
        "upstream gateway 503",
    )
    .await;

    let origin = SourceWork::new("patient_review", "work-ac4", "att-1").unwrap();

    // Two Today rows on Elena Morales (1 open concern + 1 collapsed unhandled row) + one Today row on Jake Miller.
    seed_concern(
        &harness.store,
        &harness.db,
        TestConcern {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_elena.to_owned(),
            concern_id: "concern-elena-k".to_owned(),
            urgency: ConcernUrgency::Today,
            standing: TestConcernStanding::Open,
            reason: "Potassium 5.8 mmol/L on ACE inhibitor; recheck BMP this week.".to_owned(),
            records: vec![ref_elena.clone()],
            source_work: origin.clone(),
            created_at: CLINICAL_NOW_UNIX - 1_800,
            updated_at: CLINICAL_NOW_UNIX - 1_800,
            visit_slot: None,
        },
    )
    .await;
    seed_visit_preparation(
        &harness.db,
        TestVisitPreparation {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_elena.to_owned(),
            source_id: "practice-sched".to_owned(),
            appointment_id: "appt-elena-pm".to_owned(),
            starts_at_unix: CLINICAL_NOW_UNIX + 5_400,
            ends_at_unix: CLINICAL_NOW_UNIX + 7_200,
            state: TestVisitPrepState::CouldNotFinish {
                reason: "Outside cardiology discharge summary still awaiting fax OCR".to_owned(),
            },
            updated_at: CLINICAL_NOW_UNIX - 3_600,
        },
    )
    .await;
    seed_concern(
        &harness.store,
        &harness.db,
        TestConcern {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_jake.to_owned(),
            concern_id: "concern-jake-ldl".to_owned(),
            urgency: ConcernUrgency::Today,
            standing: TestConcernStanding::Open,
            reason: "LDL 164 mg/dL after statin hold; discuss re-challenge.".to_owned(),
            records: vec![ref_jake],
            source_work: origin.clone(),
            created_at: CLINICAL_NOW_UNIX - 2_400,
            updated_at: CLINICAL_NOW_UNIX - 2_400,
            visit_slot: None,
        },
    )
    .await;

    // Seed retired concern + assessed review + patient summary so `13-updates-groups-desktop.png`
    // and `14-updates-groups-phone.png` visually prove clean non-duplicated card headlines!
    seed_concern(
        &harness.store,
        &harness.db,
        TestConcern {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_robert.to_owned(),
            concern_id: "concern-robert-retired".to_owned(),
            urgency: ConcernUrgency::Today,
            standing: TestConcernStanding::Retired {
                reason: "Repeat ambulatory blood pressures normalized.".to_owned(),
                retired_at: CLINICAL_NOW_UNIX - 1_400,
            },
            reason: "Borderline office systolic blood pressure last month.".to_owned(),
            records: vec![ref_robert.clone()],
            source_work: origin,
            created_at: CLINICAL_NOW_UNIX - 15_000,
            updated_at: CLINICAL_NOW_UNIX - 1_400,
            visit_slot: None,
        },
    )
    .await;

    // Quiet reviews today (`Reviewed, nothing for you` group) on Robert Chen and Maria Santos.
    seed_review(
        &harness.db,
        TestReview {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_robert.to_owned(),
            review_id: "rev-quiet-robert-ac5".to_owned(),
            triggers: vec![ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                last_reviewed_at: None,
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Completed {
                outcome: ReviewOutcome::Quiet {
                    reason: QuietReviewReason::NothingToDo,
                    explanation: "Ambulatory BP remains at target 119/75 mmHg.".to_owned(),
                    citations: vec![ReviewCitation {
                        reference: ref_robert,
                        source_id: "regional-ehr".to_owned(),
                        source_label: "Regional EHR".to_owned(),
                        sha256: "ef01".to_owned(),
                        display_label: "Ambulatory BP".to_owned(),
                        as_of: Some("2026-08-27".to_owned()),
                        excerpt: "Ambulatory blood pressure average 119/75 mmHg.".to_owned(),
                    }],
                    basis_and_limitations: "Regional EHR vitals.".to_owned(),
                },
                completed_at: CLINICAL_NOW_UNIX - 1_200,
                completed_source_version: 1,
            },
            evidence_read: Vec::new(),
            queued_at: CLINICAL_NOW_UNIX - 1_300,
            updated_at: CLINICAL_NOW_UNIX - 1_200,
        },
    )
    .await;
    seed_review(
        &harness.db,
        TestReview {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_maria.to_owned(),
            review_id: "rev-quiet-maria-ac5".to_owned(),
            triggers: vec![ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                last_reviewed_at: None,
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Completed {
                outcome: ReviewOutcome::Quiet {
                    reason: QuietReviewReason::NothingToDo,
                    explanation: "Routine thyroid panel stable (TSH 2.1 mIU/L).".to_owned(),
                    citations: Vec::new(),
                    basis_and_limitations: "Regional EHR labs.".to_owned(),
                },
                completed_at: CLINICAL_NOW_UNIX - 1_000,
                completed_source_version: 1,
            },
            evidence_read: Vec::new(),
            queued_at: CLINICAL_NOW_UNIX - 1_100,
            updated_at: CLINICAL_NOW_UNIX - 1_000,
        },
    )
    .await;

    // Earlier days (`Yesterday` 2026-08-26 and `Aug 25, 2026`).
    seed_review(
        &harness.db,
        TestReview {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_elena.to_owned(),
            review_id: "rev-yesterday-elena".to_owned(),
            triggers: vec![ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                last_reviewed_at: None,
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Completed {
                outcome: ReviewOutcome::Assessed {
                    summary: "Reconciled discharge medication list after urgent care visit."
                        .to_owned(),
                    citations: Vec::new(),
                    basis_and_limitations: "Regional EHR medication history.".to_owned(),
                },
                completed_at: CLINICAL_NOW_UNIX - DAY_SECONDS,
                completed_source_version: 1,
            },
            evidence_read: Vec::new(),
            queued_at: CLINICAL_NOW_UNIX - DAY_SECONDS - 300,
            updated_at: CLINICAL_NOW_UNIX - DAY_SECONDS,
        },
    )
    .await;
    seed_review(
        &harness.db,
        TestReview {
            practice: STAGING_PRACTICE_ID.to_owned(),
            workspace: ws_jake.to_owned(),
            review_id: "rev-two-days-ago-jake".to_owned(),
            triggers: vec![ReviewTrigger::Periodic {
                workspace_version: 1,
                record_versions: BTreeMap::new(),
                last_reviewed_at: None,
            }],
            unfinished_work: Vec::new(),
            state: ReviewLifecycleState::Completed {
                outcome: ReviewOutcome::Assessed {
                    summary: "Prepared quarterly retinopathy screening reminder.".to_owned(),
                    citations: Vec::new(),
                    basis_and_limitations: "Ophthalmology registry.".to_owned(),
                },
                completed_at: CLINICAL_NOW_UNIX - 2 * DAY_SECONDS,
                completed_source_version: 1,
            },
            evidence_read: Vec::new(),
            queued_at: CLINICAL_NOW_UNIX - 2 * DAY_SECONDS - 300,
            updated_at: CLINICAL_NOW_UNIX - 2 * DAY_SECONDS,
        },
    )
    .await;

    let base = format!("http://{}", harness.address);
    let mut tab = Tab::open(&base);
    tab.sign_in_as("dr.chen@codahealth.test");
    tab.wait_for_text(
        "Potassium 5.8 mmol/L on ACE inhibitor; recheck BMP this week.",
        Duration::from_secs(20),
    );
    tab.wait_for_text(
        "Coda also reviewed 2 patients today — nothing needed you",
        Duration::from_secs(10),
    );
    tab.wait_for_text(
        "Quest Diagnostics is unavailable across the practice (4 patients): upstream gateway 503",
        Duration::from_secs(10),
    );

    // 1. Today desktop & phone screenshots + responsive geometry check.
    tab.set_viewport(1440, 900, true);
    std::thread::sleep(Duration::from_millis(250));
    tab.assert_no_horizontal_overflow("Today desktop");
    let shot_today_desktop = tab.save_screenshot("today-desktop");

    tab.set_viewport(390, 844, true);
    std::thread::sleep(Duration::from_millis(250));
    tab.assert_no_horizontal_overflow("Today phone");
    let shot_today_phone = tab.save_screenshot("today-phone");

    // 2. Open Elena's row -> patient page shows ONLY Elena's 2 Today rows + opened work & evidence!
    tab.set_viewport(1440, 900, true);
    std::thread::sleep(Duration::from_millis(150));
    let elena_concern_id = format!("concern:{ws_elena}:concern-elena-k");
    tab.click(&format!(
        "[data-today-item=\"{elena_concern_id}\"] button.today-row-open"
    ));
    tab.wait_for_text(
        "Potassium 5.8 mmol/L with lisinopril 20 mg active medication.",
        Duration::from_secs(15),
    );
    tab.wait_for_text(
        "Outside cardiology discharge summary still awaiting fax OCR",
        Duration::from_secs(10),
    );

    let patient_rows = tab.eval(
        "[...document.querySelectorAll('[data-patient-today-rows=\"true\"] [data-today-item]')].map(e => e.getAttribute('data-today-item'))",
    );
    assert_eq!(
        patient_rows,
        json!([format!("unhandled:{ws_elena}"), elena_concern_id,]),
        "patient page must display that patient's Today rows ordered by wait time, and no other patient's rows: {patient_rows}"
    );
    let patient_rows_text = tab
        .eval("document.querySelector('[data-patient-today-rows=\"true\"]')?.innerText ?? ''")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        !patient_rows_text.contains("Jake Smith") && !patient_rows_text.contains("LDL 164 mg/dL"),
        "patient page must never include another patient's Today row:\n{patient_rows_text}"
    );

    std::thread::sleep(Duration::from_millis(250));
    tab.assert_no_horizontal_overflow("Patient page desktop");
    let shot_patient_desktop = tab.save_screenshot("patient-rows-desktop");

    tab.set_viewport(390, 844, true);
    std::thread::sleep(Duration::from_millis(250));
    tab.assert_no_horizontal_overflow("Patient page phone");
    let shot_patient_phone = tab.save_screenshot("patient-rows-phone");

    // 3. Navigate to Updates -> verify `What Coda did today`, `Reviewed, nothing for you`, and `Earlier days`!
    tab.set_viewport(1440, 900, true);
    std::thread::sleep(Duration::from_millis(150));
    tab.click_named(".sidebar-nav-item", "Updates");
    tab.wait_for_text("What Coda did today", Duration::from_secs(15));
    tab.wait_for_text("Reviewed, nothing for you", Duration::from_secs(10));
    tab.wait_for_text("Earlier days", Duration::from_secs(10));
    tab.wait_for_text("Yesterday", Duration::from_secs(10));
    tab.wait_for_text(
        "Prepared quarterly retinopathy screening reminder.",
        Duration::from_secs(10),
    );

    assert_eq!(
        tab.eval("!!document.querySelector('[data-updates-group=\"today-items\"]')"),
        json!(true)
    );
    assert_eq!(
        tab.eval("!!document.querySelector('[data-updates-group=\"reviewed-nothing-for-you\"]')"),
        json!(true)
    );
    assert_eq!(
        tab.eval("!!document.querySelector('[data-updates-group=\"earlier-days\"]')"),
        json!(true)
    );
    assert_eq!(
        tab.eval("[...document.querySelectorAll('[data-earlier-day]')].map(e => e.getAttribute('data-earlier-day'))"),
        json!(["2026-08-26", "2026-08-25"])
    );

    std::thread::sleep(Duration::from_millis(250));
    tab.assert_no_horizontal_overflow("Updates desktop");
    let shot_updates_desktop = tab.save_screenshot("updates-groups-desktop");

    tab.set_viewport(390, 844, true);
    std::thread::sleep(Duration::from_millis(250));
    tab.assert_no_horizontal_overflow("Updates phone");
    let shot_updates_phone = tab.save_screenshot("updates-groups-phone");

    for shot in [
        &shot_today_desktop,
        &shot_today_phone,
        &shot_patient_desktop,
        &shot_patient_phone,
        &shot_updates_desktop,
        &shot_updates_phone,
    ] {
        assert!(
            Path::new(shot)
                .metadata()
                .map(|m| m.len() > 1_000)
                .unwrap_or(false),
            "expected non-empty screenshot at {}",
            shot.display()
        );
    }
}
