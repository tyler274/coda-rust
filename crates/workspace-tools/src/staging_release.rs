//! Durable checkpoints around the existing staging-only, tested-image rollout.
//! Cloud output is projected into allowlisted fields; stderr and environment are never receipts.
use crate::failure::Failure;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Outcome {
    Initialized,
    InProgress,
    NotAttempted,
    Unknown,
    Succeeded,
    Failed,
    PartialOrUnknown,
    BootstrapSkippedAbsent,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Phase {
    Publication,
    BeforeSnapshot,
    PhysicianMigrationUpdate,
    PhysicianMigrationExecute,
    PhysicianMigrationWait,
    DuetMigrationUpdate,
    DuetMigrationExecute,
    DuetMigrationWait,
    PhysicianRollout,
    DuetRollout,
    WorkerRollout,
}
fn migration_phase(app: &str, step: u8) -> Phase {
    match (app, step) {
        ("physician", 0) => Phase::PhysicianMigrationUpdate,
        ("physician", 1) => Phase::PhysicianMigrationExecute,
        ("physician", _) => Phase::PhysicianMigrationWait,
        (_, 0) => Phase::DuetMigrationUpdate,
        (_, 1) => Phase::DuetMigrationExecute,
        _ => Phase::DuetMigrationWait,
    }
}

/// Polls after an async service update: 120 x 5 s covers image pull plus the
/// 120 s startup probe budget, and the shared `Gcloud` deadline still applies.
const ROLLOUT_POLLS: usize = 120;
const PROJECT: &str = "coda-rust-staging";
const REGION: &str = "us-east1";
const SERVICES: [&str; 3] = ["physician", "duet", "physician-worker"];
const IMAGES: [(&str, &str); 5] = [
    ("physician-runtime", "coda-physician-sample"),
    ("physician-worker", "coda-physician-worker"),
    ("physician-migration", "coda-physician-migrate"),
    ("duet-runtime", "coda-duet-api"),
    ("duet-migration", "coda-duet-migrate"),
];

type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("invalid release input: {0}")]
    Input(&'static str),
    #[error("release receipt I/O failed")]
    Io(#[from] std::io::Error),
    #[error("invalid structured release data")]
    Json(#[from] serde_json::Error),
    #[error("staging operation failed: {0}")]
    Cloud(&'static str),
    #[error("cloud command failed ({code})")]
    Command {
        code: &'static str,
        #[source]
        source: Arc<CommandFailure>,
    },
    #[error("cloud observations failed")]
    Observations(#[source] Arc<Error>),
    #[error("release failed with related diagnostic causes")]
    Related {
        #[source]
        primary: Box<Error>,
        causes: Vec<Arc<Error>>,
    },
    /// The operation failed and its receipt could not record that failure.
    #[error("release failed and its receipt could not be saved ({})", persistence)]
    Unrecorded {
        #[source]
        primary: Box<Error>,
        persistence: Box<Error>,
    },
}
#[derive(Debug, thiserror::Error)]
#[error("gcloud exited with {status:?}")]
struct CommandFailure {
    status: Option<i32>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}
impl Error {
    fn safe_code(&self) -> &'static str {
        match self {
            Self::Command { code, .. } | Self::Cloud(code) => code,
            Self::Input(_) => "invalid-input",
            Self::Io(_) => "io-failed",
            Self::Json(_) => "json-invalid",
            Self::Observations(_) => "observations-failed",
            Self::Related { primary, .. } | Self::Unrecorded { primary, .. } => primary.safe_code(),
        }
    }
    /// Allowlisted provider status of the primary cloud command failure, if any.
    fn api_status(&self) -> Option<&'static str> {
        match self {
            Self::Command { source, .. } => crate::failure::api_status(&source.stderr),
            Self::Observations(inner) => inner.api_status(),
            Self::Related { primary, causes } => primary
                .api_status()
                .or_else(|| causes.iter().rev().find_map(|cause| cause.api_status())),
            Self::Unrecorded { primary, .. } => primary.api_status(),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Traffic {
    revision: String,
    percent: u64,
    latest: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Service {
    created_revision: String,
    ready_revision: String,
    image: String,
    ready_image: String,
    traffic: Vec<Traffic>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "state", content = "summary", rename_all = "snake_case")]
enum Observation {
    Known(Service),
    Absent,
    Unknown,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ServiceRecord {
    before: Observation,
    final_state: Observation,
    outcome: Outcome,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Migration {
    execution_id: Option<String>,
    executions_before: BTreeSet<String>,
    outcome: Outcome,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Diagnostic {
    phase: Phase,
    code: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Receipt {
    schema_version: u32,
    project: String,
    region: String,
    source_sha: String,
    workflow: String,
    run_id: String,
    run_attempt: String,
    tested_artifact_id: String,
    images: BTreeMap<String, Option<String>>,
    services: BTreeMap<String, ServiceRecord>,
    migrations: BTreeMap<String, Migration>,
    phase: Phase,
    outcome: Outcome,
    diagnostics: Vec<Diagnostic>,
    #[serde(skip)]
    causes: Vec<Arc<Error>>,
}

trait Store {
    fn save(&mut self, receipt: &Receipt) -> Result<()>;
}
struct DiskStore(PathBuf);
impl Store for DiskStore {
    fn save(&mut self, receipt: &Receipt) -> Result<()> {
        let temporary = self.0.with_extension("json.tmp");
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temporary)?;
        file.write_all(&serde_json::to_vec_pretty(receipt)?)?;
        file.write_all(b"\n")?;
        file.sync_all()?;
        fs::rename(temporary, &self.0)?;
        File::open(self.0.parent().ok_or(Error::Input("receipt directory"))?)?.sync_all()?;
        Ok(())
    }
}

trait Cloud {
    fn service(&mut self, name: &str) -> Result<Observation>;
    fn update_job(&mut self, name: &str, image: &str) -> Result<()>;
    fn job_image(&mut self, name: &str) -> Result<String>;
    fn executions(&mut self, name: &str) -> Result<BTreeSet<String>>;
    fn execute(&mut self, name: &str) -> Result<String>;
    fn execution(&mut self, id: &str, image: &str) -> Result<Execution>;
    fn update_service(&mut self, name: &str, image: &str) -> Result<()>;
    fn pause(&mut self);
}
#[derive(Debug, PartialEq, Eq)]
enum Execution {
    Pending,
    Succeeded,
    Failed,
}

fn checkpoint(receipt: &mut Receipt, store: &mut impl Store, phase: Phase) -> Result<()> {
    receipt.phase = phase.to_owned();
    store.save(receipt)
}
fn diagnostic(receipt: &mut Receipt, code: &str) {
    receipt.diagnostics.push(Diagnostic {
        phase: receipt.phase,
        code: code.to_owned(),
    });
}
fn retain_cause(receipt: &mut Receipt, error: Error) {
    diagnostic(receipt, error.safe_code());
    if let Some(status) = error.api_status() {
        diagnostic(receipt, status);
    }
    receipt.causes.push(Arc::new(error));
}
fn observe(receipt: &mut Receipt, cloud: &mut impl Cloud, name: &str) -> Observation {
    match cloud.service(name) {
        Ok(state) => state,
        Err(error) => {
            retain_cause(receipt, error);
            Observation::Unknown
        }
    }
}
fn budget_exhausted(receipt: &Receipt) -> bool {
    receipt
        .causes
        .last()
        .is_some_and(|cause| cause.safe_code() == command::BUDGET_EXHAUSTED)
}
fn snapshot(receipt: &mut Receipt, cloud: &mut impl Cloud, before: bool) -> bool {
    let mut complete = true;
    for name in SERVICES {
        let observation = observe(receipt, cloud, name);
        complete &= observation != Observation::Unknown;
        let record = receipt.services.get_mut(name).expect("validated services");
        if before {
            record.before = observation.clone();
        }
        record.final_state = observation;
    }
    complete
}
fn serving(service: &Observation, image: &str) -> bool {
    let Observation::Known(service) = service else {
        return false;
    };
    !service.ready_revision.is_empty()
        && service.ready_revision == service.created_revision
        && service.image == image
        && service.ready_image == image
        && service
            .traffic
            .iter()
            .filter(|traffic| traffic.revision == service.ready_revision)
            .map(|traffic| traffic.percent)
            .sum::<u64>()
            == 100
        && service
            .traffic
            .iter()
            .map(|traffic| traffic.percent)
            .sum::<u64>()
            == 100
}

fn migrate(
    receipt: &mut Receipt,
    store: &mut impl Store,
    cloud: &mut impl Cloud,
    app: &str,
) -> Result<()> {
    let job = format!("{app}-migrate");
    let key = format!("{app}-migration");
    let image = receipt.images[&key]
        .clone()
        .ok_or(Error::Input("missing migration digest"))?;
    checkpoint(receipt, store, migration_phase(app, 0))?;
    if let Err(error) = cloud.update_job(&job, &image) {
        retain_cause(receipt, error);
        if cloud.job_image(&job)? != image {
            return Err(Error::Cloud("migration-update-not-confirmed"));
        }
        diagnostic(receipt, "job-update-response-reconciled");
        store.save(receipt)?;
    }
    let previous = cloud.executions(&job)?;
    let migration = receipt.migrations.get_mut(app).expect("migration");
    migration.outcome = Outcome::Unknown;
    migration.executions_before = previous.clone();
    checkpoint(receipt, store, migration_phase(app, 1))?;
    let execution = match cloud.execute(&job) {
        Ok(id) => id,
        Err(error) => {
            retain_cause(receipt, error);
            // Never retry an ambiguous execute: it may already have changed the database.
            let current = cloud.executions(&job)?;
            let added: Vec<_> = current.difference(&previous).collect();
            if added.len() != 1 {
                return Err(Error::Cloud("migration-execution-unknown"));
            }
            diagnostic(receipt, "execute-response-reconciled");
            added[0].clone()
        }
    };
    receipt
        .migrations
        .get_mut(app)
        .expect("migration")
        .execution_id = Some(execution.clone());
    checkpoint(receipt, store, migration_phase(app, 2))?;
    for _ in 0..120 {
        match cloud.execution(&execution, &image)? {
            Execution::Succeeded => {
                receipt.migrations.get_mut(app).expect("migration").outcome = Outcome::Succeeded;
                store.save(receipt)?;
                return Ok(());
            }
            Execution::Failed => {
                receipt.migrations.get_mut(app).expect("migration").outcome = Outcome::Failed;
                return save_settled(store, receipt, Err(Error::Cloud("migration-failed")));
            }
            Execution::Pending => cloud.pause(),
        }
    }
    Err(Error::Cloud("migration-timeout"))
}
fn rollout(
    receipt: &mut Receipt,
    store: &mut impl Store,
    cloud: &mut impl Cloud,
    name: &str,
    key: &str,
) -> Result<()> {
    let image = receipt.images[key]
        .clone()
        .ok_or(Error::Input("missing runtime digest"))?;
    receipt.services.get_mut(name).expect("service").outcome = Outcome::Unknown;
    checkpoint(
        receipt,
        store,
        match name {
            "physician" => Phase::PhysicianRollout,
            "duet" => Phase::DuetRollout,
            _ => Phase::WorkerRollout,
        },
    )?;
    // The update is submitted asynchronously; readiness is confirmed by bounded
    // polling rather than by a synchronous call that a per-call cap could kill.
    let update = cloud.update_service(name, &image);
    let mut observed = observe(receipt, cloud, name);
    for _ in 0..ROLLOUT_POLLS {
        if serving(&observed, &image) || budget_exhausted(receipt) {
            break;
        }
        cloud.pause();
        observed = observe(receipt, cloud, name);
    }
    let ready = serving(&observed, &image);
    let record = receipt.services.get_mut(name).expect("service");
    record.final_state = observed;
    record.outcome = if ready {
        Outcome::Succeeded
    } else {
        Outcome::Unknown
    };
    if let Err(error) = update {
        retain_cause(receipt, error);
        diagnostic(receipt, "update-response-failed-reconciled");
    }
    let result = if ready {
        Ok(())
    } else {
        Err(Error::Cloud("rollout-not-confirmed"))
    };
    save_settled(store, receipt, result)
}
fn deploy(receipt: &mut Receipt, store: &mut impl Store, cloud: &mut impl Cloud) -> Result<()> {
    if receipt.outcome != Outcome::Initialized {
        return Err(Error::Input(
            "receipt already attempted; initialize a new run attempt",
        ));
    }
    if receipt.images.values().any(Option::is_none) {
        return Err(Error::Input("all five published images required"));
    }
    receipt.outcome = Outcome::InProgress;
    checkpoint(receipt, store, Phase::BeforeSnapshot)?;
    let before = snapshot(receipt, cloud, true);
    store.save(receipt)?;
    let result = (|| {
        if !before {
            return Err(Error::Cloud("before-state-unknown"));
        }
        for app in ["physician", "duet"] {
            if receipt.services[app].before == Observation::Absent {
                return Err(Error::Cloud("required-service-absent"));
            }
            migrate(receipt, store, cloud, app)?;
            rollout(receipt, store, cloud, app, &format!("{app}-runtime"))?;
        }
        if receipt.services["physician-worker"].before == Observation::Absent {
            receipt
                .services
                .get_mut("physician-worker")
                .expect("worker")
                .outcome = Outcome::BootstrapSkippedAbsent;
            store.save(receipt)?;
        } else {
            rollout(
                receipt,
                store,
                cloud,
                "physician-worker",
                "physician-worker",
            )?;
        }
        Ok(())
    })();
    if let Err(error) = &result {
        diagnostic(
            receipt,
            match error {
                Error::Cloud(code) => code,
                _ => "release-stopped",
            },
        );
        if let Some(status) = error.api_status() {
            diagnostic(receipt, status);
        }
    }
    let observed = snapshot(receipt, cloud, false);
    receipt.outcome = if result.is_ok() && observed && final_matches(receipt) {
        Outcome::Succeeded
    } else {
        Outcome::PartialOrUnknown
    };
    let failure = match result {
        Err(error) => Some(Error::Related {
            primary: Box::new(error),
            causes: receipt.causes.clone(),
        }),
        Ok(()) if receipt.outcome != Outcome::Succeeded => Some(
            receipt
                .causes
                .last()
                .cloned()
                .map_or(Error::Cloud("final-state-unknown"), Error::Observations),
        ),
        Ok(()) => None,
    };
    save_settled(store, receipt, failure.map_or(Ok(()), Err))
}
/// Save a receipt that records an already-settled result. A save failure never
/// replaces the operation error it was meant to record.
fn save_settled(store: &mut impl Store, receipt: &Receipt, result: Result<()>) -> Result<()> {
    match (result, store.save(receipt)) {
        (Ok(()), saved) => saved,
        (Err(primary), Ok(())) => Err(primary),
        (Err(primary), Err(persistence)) => Err(Error::Unrecorded {
            primary: Box::new(primary),
            persistence: Box::new(persistence),
        }),
    }
}
fn reconcile_migrations(receipt: &mut Receipt, cloud: &mut impl Cloud) {
    let mut causes = Vec::new();
    for app in ["physician", "duet"] {
        let migration = receipt
            .migrations
            .get_mut(app)
            .expect("validated migration");
        if migration.outcome != Outcome::Unknown {
            continue;
        }
        if migration.execution_id.is_none() {
            match cloud.executions(&format!("{app}-migrate")) {
                Ok(current) => {
                    let added: Vec<_> = current.difference(&migration.executions_before).collect();
                    if added.len() == 1 {
                        migration.execution_id = Some(added[0].clone());
                    }
                }
                Err(error) => causes.push(error),
            }
        }
        if let (Some(id), Some(image)) = (
            &migration.execution_id,
            receipt
                .images
                .get(&format!("{app}-migration"))
                .and_then(Option::as_deref),
        ) {
            migration.outcome = match cloud.execution(id, image) {
                Ok(Execution::Succeeded) => Outcome::Succeeded,
                Ok(Execution::Failed) => Outcome::Failed,
                Ok(Execution::Pending) => Outcome::Unknown,
                Err(error) => {
                    causes.push(error);
                    Outcome::Unknown
                }
            };
        }
    }
    for error in causes {
        retain_cause(receipt, error);
    }
}
fn final_matches(receipt: &Receipt) -> bool {
    SERVICES.iter().all(|name| {
        let record = &receipt.services[*name];
        if *name == "physician-worker" && record.outcome == Outcome::BootstrapSkippedAbsent {
            return record.final_state == Observation::Absent;
        }
        let key = if *name == "physician-worker" {
            (*name).into()
        } else {
            format!("{name}-runtime")
        };
        receipt
            .images
            .get(&key)
            .and_then(Option::as_deref)
            .is_some_and(|image| serving(&record.final_state, image))
    })
}

mod command;
use command::Gcloud;
fn identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 255
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"-_.".contains(&byte))
}
fn text(value: &Value, pointer: &str) -> Result<String> {
    let value = value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or(Error::Cloud("missing-cloud-field"))?;
    if !identifier(value) {
        return Err(Error::Cloud("invalid-cloud-identifier"));
    }
    Ok(value.to_owned())
}
fn cloud_image(value: &Value, pointer: &str) -> Result<String> {
    let image = value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or(Error::Cloud("missing-cloud-image"))?;
    // Only image references are retained, never arbitrary cloud messages or container settings.
    if image.len() > 512
        || !image
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"-._/:@".contains(&byte))
    {
        return Err(Error::Cloud("invalid-cloud-image"));
    }
    Ok(image.to_owned())
}
impl<R: command::Runner> Cloud for Gcloud<R> {
    fn service(&mut self, name: &str) -> Result<Observation> {
        let output = self.call(&["services", "describe", name])?;
        if !output.status.success() {
            if command::structured_not_found(&output.stderr) {
                return Ok(Observation::Absent);
            }
            return Err(command::command_error(output));
        }
        let service: Value = serde_json::from_slice(&output.stdout)?;
        let ready = text(&service, "/status/latestReadyRevisionName")?;
        let revision = self.json(&["revisions", "describe", &ready])?;
        let traffic = service
            .pointer("/status/traffic")
            .and_then(Value::as_array)
            .ok_or(Error::Cloud("missing-traffic"))?
            .iter()
            .map(|entry| {
                Ok(Traffic {
                    revision: text(entry, "/revisionName")?,
                    percent: entry.get("percent").and_then(Value::as_u64).unwrap_or(0),
                    latest: entry
                        .get("latestRevision")
                        .and_then(Value::as_bool)
                        .unwrap_or(false),
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Observation::Known(Service {
            created_revision: text(&service, "/status/latestCreatedRevisionName")?,
            ready_revision: ready,
            image: cloud_image(&service, "/spec/template/spec/containers/0/image")?,
            ready_image: cloud_image(&revision, "/spec/containers/0/image")?,
            traffic,
        }))
    }
    fn update_job(&mut self, name: &str, image: &str) -> Result<()> {
        self.json(&["jobs", "update", name, "--image", image])
            .map(|_| ())
    }
    fn job_image(&mut self, name: &str) -> Result<String> {
        cloud_image(
            &self.json(&["jobs", "describe", name])?,
            "/spec/template/spec/template/spec/containers/0/image",
        )
    }
    fn executions(&mut self, name: &str) -> Result<BTreeSet<String>> {
        self.json(&["jobs", "executions", "list", "--job", name])?
            .as_array()
            .ok_or(Error::Cloud("invalid-executions"))?
            .iter()
            .map(|value| text(value, "/metadata/name"))
            .collect()
    }
    fn execute(&mut self, name: &str) -> Result<String> {
        let output = self.call(&["jobs", "execute", name, "--async"])?;
        if !output.status.success() {
            return Err(command::command_error(output));
        }
        let value: Value = serde_json::from_slice(&output.stdout)?;
        text(&value, "/metadata/name")
    }
    fn execution(&mut self, id: &str, image: &str) -> Result<Execution> {
        let value = self.json(&["jobs", "executions", "describe", id])?;
        if cloud_image(&value, "/spec/template/spec/containers/0/image")? != image {
            return Err(Error::Cloud("execution-image-mismatch"));
        }
        let conditions = value
            .pointer("/status/conditions")
            .and_then(Value::as_array)
            .ok_or(Error::Cloud("missing-execution-status"))?;
        match conditions
            .iter()
            .find(|condition| condition.get("type").and_then(Value::as_str) == Some("Completed"))
            .and_then(|condition| condition.get("status"))
            .and_then(Value::as_str)
        {
            Some("True") => Ok(Execution::Succeeded),
            Some("False") => Ok(Execution::Failed),
            _ => Ok(Execution::Pending),
        }
    }
    fn update_service(&mut self, name: &str, image: &str) -> Result<()> {
        self.json(&["services", "update", name, "--image", image, "--async"])
            .map(|_| ())
    }
    fn pause(&mut self) {
        std::thread::sleep(Duration::from_secs(5));
    }
}

fn options(args: &[String]) -> Result<BTreeMap<String, String>> {
    if !args.len().is_multiple_of(2) {
        return Err(Error::Input("expected --name value pairs"));
    }
    let mut options = BTreeMap::new();
    for pair in args.as_chunks::<2>().0 {
        if ![
            "--directory",
            "--project",
            "--region",
            "--source-sha",
            "--workflow",
            "--run-id",
            "--run-attempt",
            "--artifact-id",
        ]
        .contains(&pair[0].as_str())
            || options.insert(pair[0].clone(), pair[1].clone()).is_some()
        {
            return Err(Error::Input("unknown or duplicate option"));
        }
    }
    Ok(options)
}
fn required<'a>(options: &'a BTreeMap<String, String>, name: &str) -> Result<&'a str> {
    options
        .get(name)
        .map(String::as_str)
        .ok_or(Error::Input("missing required option"))
}
fn initialize(options: &BTreeMap<String, String>) -> Result<Receipt> {
    if required(options, "--project")? != PROJECT || required(options, "--region")? != REGION {
        return Err(Error::Input(
            "only coda-rust-staging/us-east1 is authorized",
        ));
    }
    let sha = required(options, "--source-sha")?;
    if sha.len() != 40
        || !sha
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(Error::Input("full lowercase source SHA required"));
    }
    for key in ["--run-id", "--run-attempt", "--artifact-id"] {
        let value = required(options, key)?;
        if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(Error::Input("numeric workflow identity required"));
        }
    }
    let workflow = required(options, "--workflow")?;
    if workflow != ".github/workflows/ci.yml" {
        return Err(Error::Input("unexpected workflow identity"));
    }
    Ok(Receipt {
        schema_version: 1,
        project: PROJECT.into(),
        region: REGION.into(),
        source_sha: sha.into(),
        workflow: workflow.into(),
        run_id: required(options, "--run-id")?.into(),
        run_attempt: required(options, "--run-attempt")?.into(),
        tested_artifact_id: required(options, "--artifact-id")?.into(),
        images: IMAGES
            .iter()
            .map(|(key, _)| ((*key).into(), None))
            .collect(),
        services: SERVICES
            .iter()
            .map(|name| {
                (
                    (*name).into(),
                    ServiceRecord {
                        before: Observation::Unknown,
                        final_state: Observation::Unknown,
                        outcome: Outcome::NotAttempted,
                    },
                )
            })
            .collect(),
        migrations: ["physician", "duet"]
            .iter()
            .map(|name| {
                (
                    (*name).into(),
                    Migration {
                        execution_id: None,
                        executions_before: BTreeSet::new(),
                        outcome: Outcome::NotAttempted,
                    },
                )
            })
            .collect(),
        phase: Phase::Publication,
        outcome: Outcome::Initialized,
        diagnostics: Vec::new(),
        causes: Vec::new(),
    })
}
fn published_images(receipt: &mut Receipt, directory: &Path) -> Result<()> {
    receipt.images = IMAGES
        .iter()
        .map(|(key, _)| ((*key).into(), None))
        .collect();
    let manifest = directory.join("tested-images.json");
    if manifest.exists() {
        let manifest: Value = serde_json::from_slice(&fs::read(manifest)?)?;
        if manifest.get("source_revision").and_then(Value::as_str) != Some(&receipt.source_sha) {
            return Err(Error::Input(
                "published bundle differs from receipt source SHA",
            ));
        }
    } else if IMAGES.iter().any(|(key, _)| directory.join(key).exists()) {
        return Err(Error::Input(
            "published digest has no tested bundle identity",
        ));
    }
    for (key, repository) in IMAGES {
        let path = directory.join(key);
        if !path.exists() {
            continue;
        }
        let image = fs::read_to_string(path)?;
        let image = image.trim();
        let prefix = format!("{REGION}-docker.pkg.dev/{PROJECT}/apps/{repository}@sha256:");
        let digest = image
            .strip_prefix(&prefix)
            .ok_or(Error::Input("unexpected image repository"))?;
        if digest.len() != 64
            || !digest
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err(Error::Input("immutable digest required"));
        }
        receipt.images.insert(key.into(), Some(image.into()));
    }
    Ok(())
}
fn run(args: &[String]) -> Result<()> {
    let (command, args) = args
        .split_first()
        .ok_or(Error::Input("expected init, deploy, or finalize"))?;
    let options = options(args)?;
    let directory = Path::new(required(&options, "--directory")?);
    let mut store = DiskStore(directory.join("receipt.json"));
    if command == "init" {
        let mut receipt = initialize(&options)?;
        fs::create_dir_all(directory)?;
        // A rerun must use a fresh directory/attempt rather than erase evidence.
        let reservation = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&store.0)?;
        reservation.sync_all()?;
        store.save(&receipt)?;
        let mut cloud = Gcloud::new(command::ProcessRunner, Duration::from_secs(300));
        let observed = snapshot(&mut receipt, &mut cloud, true);
        store.save(&receipt)?;
        return if observed {
            Ok(())
        } else {
            Err(receipt
                .causes
                .last()
                .cloned()
                .map_or(Error::Cloud("initial-state-unknown"), Error::Observations))
        };
    }
    let mut receipt: Receipt = serde_json::from_slice(&fs::read(&store.0)?)?;
    if receipt.schema_version != 1
        || receipt.project != PROJECT
        || receipt.region != REGION
        || receipt.services.len() != 3
        || SERVICES
            .iter()
            .any(|name| !receipt.services.contains_key(*name))
        || ["physician", "duet"]
            .iter()
            .any(|name| !receipt.migrations.contains_key(*name))
    {
        return Err(Error::Input("invalid receipt contract"));
    }
    match command.as_str() {
        "deploy" => {
            let images = published_images(&mut receipt, directory);
            if images.is_err() {
                diagnostic(&mut receipt, "invalid-publication-evidence");
            }
            save_settled(&mut store, &receipt, images)?;
            deploy(&mut receipt, &mut store, &mut Gcloud::default())
        }
        "finalize" => {
            let images = published_images(&mut receipt, directory);
            let mut cloud = Gcloud::new(command::ProcessRunner, Duration::from_secs(300));
            reconcile_migrations(&mut receipt, &mut cloud);
            let observed = snapshot(&mut receipt, &mut cloud, false);
            if receipt.outcome != Outcome::Succeeded
                || !observed
                || images.is_err()
                || !final_matches(&receipt)
            {
                receipt.outcome = Outcome::PartialOrUnknown;
                diagnostic(&mut receipt, "finalized-incomplete-release");
            }
            let result = images.and_then(|()| {
                if receipt.outcome == Outcome::Succeeded {
                    Ok(())
                } else {
                    Err(receipt
                        .causes
                        .last()
                        .cloned()
                        .map_or(Error::Cloud("release-incomplete"), Error::Observations))
                }
            });
            if let Err(persistence) = store.save(&receipt) {
                return Err(match result {
                    Ok(()) => persistence,
                    Err(primary) => Error::Unrecorded {
                        primary: Box::new(primary),
                        persistence: Box::new(persistence),
                    },
                });
            }
            if let Some(path) = std::env::var_os("GITHUB_STEP_SUMMARY") {
                let mut file = OpenOptions::new().append(true).create(true).open(path)?;
                write_summary(&receipt, &mut file)?;
            }
            result
        }
        _ => Err(Error::Input("expected init, deploy, or finalize")),
    }
}
fn write_summary(receipt: &Receipt, writer: &mut impl Write) -> Result<()> {
    writeln!(
        writer,
        "## Staging release\n\nOutcome: `{:?}`; last phase: `{:?}`.",
        receipt.outcome, receipt.phase
    )?;
    for name in SERVICES {
        let record = &receipt.services[name];
        writeln!(
            writer,
            "- {name}: `{:?}`; final observation: `{}`",
            record.outcome,
            match record.final_state {
                Observation::Known(_) => "known",
                Observation::Absent => "absent",
                Observation::Unknown => "unknown",
            }
        )?;
    }
    for name in ["physician", "duet"] {
        writeln!(
            writer,
            "- {name} migration: `{:?}`",
            receipt.migrations[name].outcome
        )?;
    }
    writeln!(
        writer,
        "\nSee the release receipt artifact for digest, execution, revision and traffic evidence."
    )?;
    Ok(())
}
fn failure_message(error: &Error) -> String {
    match error.api_status() {
        Some(status) => format!(
            "staging release failed: {} (API status {status})",
            error.safe_code()
        ),
        None => format!("staging release failed: {}", error.safe_code()),
    }
}
pub fn from_args(args: &[String]) -> std::result::Result<(), Failure> {
    run(args).map_err(|error| Failure::with_source(1, failure_message(&error), error))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MemoryStore {
        receipts: Vec<Receipt>,
        fail_at: Option<usize>,
        fail_final_partial: bool,
        fail_failed_migration: bool,
    }
    impl Store for MemoryStore {
        fn save(&mut self, receipt: &Receipt) -> Result<()> {
            if self.fail_at == Some(self.receipts.len())
                || (self.fail_final_partial && receipt.outcome == Outcome::PartialOrUnknown)
                || (self.fail_failed_migration
                    && receipt
                        .migrations
                        .values()
                        .any(|migration| migration.outcome == Outcome::Failed))
            {
                return Err(Error::Cloud("receipt-unwritable"));
            }
            self.receipts.push(receipt.clone());
            Ok(())
        }
    }
    struct FakeCloud {
        services: BTreeMap<String, Observation>,
        writes: Vec<String>,
        fail_migration: bool,
        fail_duet_migration: bool,
        ambiguous_execute: bool,
        ambiguous_update: bool,
        unknown_execution: bool,
        wrong_ready_image: bool,
        worker_failure: bool,
        split_traffic: bool,
        // Observations of an updated service that still show the previous revision.
        not_ready_observations: usize,
        pending: BTreeMap<String, (Observation, usize)>,
        pauses: usize,
        denied_execution: bool,
        // Service reads fail once this many have succeeded.
        reads_before_failure: Option<usize>,
    }
    fn denied() -> Error {
        Error::Command {
            code: "gcloud-command-failed",
            source: Arc::new(CommandFailure {
                status: Some(1),
                stdout: b"secret-stdout".to_vec(),
                stderr: b"ERROR: PERMISSION_DENIED: secret-stderr".to_vec(),
            }),
        }
    }
    impl FakeCloud {
        fn new() -> Self {
            Self {
                services: SERVICES
                    .iter()
                    .map(|name| ((*name).into(), known("old-image")))
                    .collect(),
                writes: Vec::new(),
                fail_migration: false,
                fail_duet_migration: false,
                ambiguous_execute: false,
                ambiguous_update: false,
                unknown_execution: false,
                wrong_ready_image: false,
                worker_failure: false,
                split_traffic: false,
                not_ready_observations: 0,
                pending: BTreeMap::new(),
                pauses: 0,
                denied_execution: false,
                reads_before_failure: None,
            }
        }
    }
    fn known(image: &str) -> Observation {
        Observation::Known(Service {
            created_revision: "ready-revision".into(),
            ready_revision: "ready-revision".into(),
            image: image.into(),
            ready_image: image.into(),
            traffic: vec![Traffic {
                revision: "ready-revision".into(),
                percent: 100,
                latest: true,
            }],
        })
    }
    impl Cloud for FakeCloud {
        fn service(&mut self, name: &str) -> Result<Observation> {
            if let Some(remaining) = &mut self.reads_before_failure {
                if *remaining == 0 {
                    return Err(denied());
                }
                *remaining -= 1;
            }
            if let Some((state, remaining)) = self.pending.get_mut(name) {
                if *remaining > 0 {
                    *remaining -= 1;
                } else {
                    let state = state.clone();
                    self.pending.remove(name);
                    self.services.insert(name.into(), state);
                }
            }
            Ok(self.services[name].clone())
        }
        fn update_job(&mut self, name: &str, _: &str) -> Result<()> {
            self.writes.push(format!("job:{name}"));
            Ok(())
        }
        fn job_image(&mut self, _: &str) -> Result<String> {
            Err(Error::Cloud("unused"))
        }
        fn executions(&mut self, name: &str) -> Result<BTreeSet<String>> {
            Ok(
                if !self.unknown_execution && self.writes.contains(&format!("execute:{name}")) {
                    BTreeSet::from([format!("{name}-execution")])
                } else {
                    BTreeSet::new()
                },
            )
        }
        fn execute(&mut self, name: &str) -> Result<String> {
            self.writes.push(format!("execute:{name}"));
            if self.ambiguous_execute {
                Err(Error::Cloud("lost-response"))
            } else {
                Ok(format!("{name}-execution"))
            }
        }
        fn execution(&mut self, id: &str, _: &str) -> Result<Execution> {
            if self.denied_execution {
                return Err(denied());
            }
            Ok(
                if self.fail_migration || (self.fail_duet_migration && id.starts_with("duet-")) {
                    Execution::Failed
                } else {
                    Execution::Succeeded
                },
            )
        }
        fn update_service(&mut self, name: &str, image: &str) -> Result<()> {
            self.writes.push(format!("service:{name}"));
            if name == "physician-worker" && self.worker_failure {
                return Err(Error::Cloud("worker-update-failed"));
            }
            let mut state = known(image);
            if let Observation::Known(service) = &mut state {
                if self.wrong_ready_image {
                    service.ready_image = "unrelated-image".into();
                }
                if self.split_traffic {
                    service.traffic[0].percent = 50;
                }
            }
            if self.not_ready_observations > 0 {
                self.pending
                    .insert(name.into(), (state, self.not_ready_observations));
            } else {
                self.services.insert(name.into(), state);
            }
            if self.ambiguous_update {
                Err(Error::Cloud("lost-response"))
            } else {
                Ok(())
            }
        }
        fn pause(&mut self) {
            self.pauses += 1;
        }
    }
    pub(super) fn receipt() -> Receipt {
        let options = BTreeMap::from([
            ("--project".into(), PROJECT.into()),
            ("--region".into(), REGION.into()),
            ("--source-sha".into(), "a".repeat(40)),
            ("--workflow".into(), ".github/workflows/ci.yml".into()),
            ("--run-id".into(), "123".into()),
            ("--run-attempt".into(), "1".into()),
            ("--artifact-id".into(), "456".into()),
        ]);
        let mut receipt = initialize(&options).expect("receipt");
        for (key, repository) in IMAGES {
            receipt.images.insert(
                key.into(),
                Some(format!(
                    "{REGION}-docker.pkg.dev/{PROJECT}/apps/{repository}@sha256:{}",
                    "b".repeat(64)
                )),
            );
        }
        receipt
    }
    #[test]
    fn successful_release_has_all_images_migrations_and_service_states() {
        let mut receipt = receipt();
        let mut store = MemoryStore::default();
        let mut cloud = FakeCloud::new();
        deploy(&mut receipt, &mut store, &mut cloud).expect("release");
        assert_eq!(receipt.outcome, Outcome::Succeeded);
        assert_eq!(receipt.images.len(), 5);
        assert!(
            receipt
                .migrations
                .values()
                .all(|migration| migration.outcome == Outcome::Succeeded
                    && migration.execution_id.is_some())
        );
        assert!(
            receipt
                .services
                .values()
                .all(|service| service.before != Observation::Unknown
                    && service.final_state != Observation::Unknown)
        );
        assert_eq!(
            cloud.writes,
            [
                "job:physician-migrate",
                "execute:physician-migrate",
                "service:physician",
                "job:duet-migrate",
                "execute:duet-migrate",
                "service:duet",
                "service:physician-worker"
            ]
        );
        assert!(
            store
                .receipts
                .iter()
                .any(|receipt| receipt.phase == Phase::PhysicianMigrationExecute)
        );
    }
    #[test]
    fn failed_migration_keeps_execution_and_stops_all_later_writes() {
        let mut receipt = receipt();
        let mut cloud = FakeCloud::new();
        cloud.fail_migration = true;
        assert!(deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).is_err());
        assert_eq!(
            cloud.writes,
            ["job:physician-migrate", "execute:physician-migrate"]
        );
        assert_eq!(
            receipt.migrations["physician"].execution_id.as_deref(),
            Some("physician-migrate-execution")
        );
        assert_eq!(receipt.migrations["physician"].outcome, Outcome::Failed);
        assert_eq!(receipt.outcome, Outcome::PartialOrUnknown);
        assert!(
            receipt
                .services
                .values()
                .all(|service| service.final_state != Observation::Unknown)
        );
    }
    #[test]
    fn ambiguous_execute_and_update_are_reconciled_without_retries() {
        let mut receipt = receipt();
        let mut cloud = FakeCloud::new();
        cloud.ambiguous_execute = true;
        cloud.ambiguous_update = true;
        deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).expect("reconciled");
        assert_eq!(cloud.writes.len(), 7);
        assert_eq!(receipt.outcome, Outcome::Succeeded);
        assert_eq!(receipt.diagnostics.len(), 10);
        assert_eq!(receipt.causes.len(), 5);
    }
    #[test]
    fn unidentifiable_execution_is_unknown_and_never_reexecuted() {
        let mut receipt = receipt();
        let mut cloud = FakeCloud::new();
        cloud.ambiguous_execute = true;
        cloud.unknown_execution = true;
        assert!(deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).is_err());
        assert_eq!(cloud.writes.len(), 2);
        assert_eq!(receipt.migrations["physician"].outcome, Outcome::Unknown);
        assert!(receipt.migrations["physician"].execution_id.is_none());
    }
    #[test]
    fn bootstrap_skip_requires_confirmed_absence() {
        for observation in [Observation::Absent, Observation::Unknown] {
            let mut receipt = receipt();
            let mut cloud = FakeCloud::new();
            cloud
                .services
                .insert("physician-worker".into(), observation.clone());
            let result = deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud);
            if observation == Observation::Absent {
                result.expect("bootstrap");
                assert_eq!(
                    receipt.services["physician-worker"].outcome,
                    Outcome::BootstrapSkippedAbsent
                );
                assert_eq!(cloud.writes.len(), 6);
            } else {
                assert!(result.is_err());
                assert!(cloud.writes.is_empty());
            }
        }
    }
    #[test]
    fn unwritable_checkpoint_never_permits_next_cloud_mutation() {
        let mut successful_store = MemoryStore::default();
        deploy(&mut receipt(), &mut successful_store, &mut FakeCloud::new()).expect("baseline");
        for checkpoint in 0..successful_store.receipts.len() {
            let mut cloud = FakeCloud::new();
            let mut store = MemoryStore {
                fail_at: Some(checkpoint),
                ..MemoryStore::default()
            };
            assert!(deploy(&mut receipt(), &mut store, &mut cloud).is_err());
            if checkpoint < 3 {
                assert!(cloud.writes.is_empty());
            }
            assert_eq!(store.receipts.len(), checkpoint);
        }
    }
    #[test]
    fn slow_rollout_is_polled_until_ready_without_repeating_the_update() {
        let mut receipt = receipt();
        let mut cloud = FakeCloud::new();
        cloud.not_ready_observations = 3;
        deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).expect("release");
        assert_eq!(receipt.outcome, Outcome::Succeeded);
        assert_eq!(
            cloud
                .writes
                .iter()
                .filter(|write| write.starts_with("service:"))
                .count(),
            3
        );
        assert_eq!(cloud.pauses, 9);
    }
    #[test]
    fn never_ready_rollout_stops_after_bounded_polls() {
        let mut receipt = receipt();
        let mut cloud = FakeCloud::new();
        cloud.not_ready_observations = usize::MAX;
        let error =
            deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).expect_err("not ready");
        assert_eq!(error.safe_code(), "rollout-not-confirmed");
        assert_eq!(cloud.pauses, ROLLOUT_POLLS);
        assert_eq!(cloud.writes.len(), 3);
        assert_eq!(receipt.services["physician"].outcome, Outcome::Unknown);
    }
    #[test]
    fn final_receipt_failure_keeps_the_operation_error() {
        for scenario in 0..2 {
            let mut cloud = FakeCloud::new();
            cloud.fail_migration = scenario == 0;
            cloud.worker_failure = scenario == 1;
            let mut store = MemoryStore {
                fail_final_partial: true,
                ..MemoryStore::default()
            };
            let error = deploy(&mut receipt(), &mut store, &mut cloud).expect_err("failed");
            let Error::Unrecorded {
                primary,
                persistence,
            } = &error
            else {
                panic!("persistence failure replaced the operation error: {error:?}");
            };
            assert!(matches!(**primary, Error::Related { .. }));
            assert_eq!(persistence.safe_code(), "receipt-unwritable");
            assert_eq!(
                error.safe_code(),
                if scenario == 0 {
                    "migration-failed"
                } else {
                    "rollout-not-confirmed"
                }
            );
        }
    }
    #[test]
    fn failed_migration_survives_an_unwritable_receipt() {
        let mut cloud = FakeCloud::new();
        cloud.fail_migration = true;
        let mut store = MemoryStore {
            fail_failed_migration: true,
            ..MemoryStore::default()
        };
        let error = deploy(&mut receipt(), &mut store, &mut cloud).expect_err("failed");
        assert!(matches!(error, Error::Unrecorded { .. }), "{error:?}");
        assert_eq!(error.safe_code(), "migration-failed");
        assert!(
            !cloud
                .writes
                .iter()
                .any(|write| write.starts_with("service:"))
        );
    }
    #[test]
    fn propagated_command_failure_records_only_its_api_status() {
        let mut receipt = receipt();
        let mut cloud = FakeCloud::new();
        cloud.denied_execution = true;
        let error =
            deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).expect_err("denied");
        assert_eq!(
            failure_message(&error),
            "staging release failed: gcloud-command-failed (API status PERMISSION_DENIED)"
        );
        let encoded = serde_json::to_string(&receipt).expect("receipt");
        assert!(encoded.contains("\"PERMISSION_DENIED\""));
        assert!(!encoded.contains("secret"));
    }
    #[test]
    fn final_snapshot_failure_survives_an_unwritable_receipt() {
        let mut cloud = FakeCloud::new();
        // Three initial reads, then one per rollout; the final snapshot fails.
        cloud.reads_before_failure = Some(6);
        let mut store = MemoryStore {
            fail_final_partial: true,
            ..MemoryStore::default()
        };
        let error = deploy(&mut receipt(), &mut store, &mut cloud).expect_err("unknown");
        let Error::Unrecorded { primary, .. } = &error else {
            panic!("persistence failure replaced the observation error: {error:?}");
        };
        assert!(matches!(**primary, Error::Observations(_)));
        assert_eq!(error.safe_code(), "observations-failed");
        assert_eq!(error.api_status(), Some("PERMISSION_DENIED"));
    }
    #[test]
    fn wrong_ready_digest_and_split_traffic_fail_before_duet() {
        for split in [true, false] {
            let mut receipt = receipt();
            let mut cloud = FakeCloud::new();
            cloud.split_traffic = split;
            cloud.wrong_ready_image = !split;
            assert!(deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).is_err());
            assert_eq!(cloud.writes.len(), 3);
            assert_eq!(receipt.outcome, Outcome::PartialOrUnknown);
        }
    }
    #[test]
    fn partial_publication_cannot_start_migrations() {
        let mut receipt = receipt();
        receipt.images.insert("duet-migration".into(), None);
        let mut cloud = FakeCloud::new();
        assert!(deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).is_err());
        assert!(cloud.writes.is_empty());
    }
    #[test]
    fn duet_failure_preserves_successful_physician_and_never_rolls_back() {
        let mut receipt = receipt();
        let mut cloud = FakeCloud::new();
        cloud.fail_duet_migration = true;
        assert!(deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).is_err());
        assert_eq!(cloud.writes.len(), 5);
        assert_eq!(receipt.services["physician"].outcome, Outcome::Succeeded);
        assert!(serving(
            &receipt.services["physician"].final_state,
            receipt.images["physician-runtime"]
                .as_deref()
                .expect("image")
        ));
        assert_eq!(receipt.migrations["duet"].outcome, Outcome::Failed);
        assert_eq!(
            receipt.services["duet"].before,
            receipt.services["duet"].final_state
        );
    }
    #[test]
    fn finalization_recovers_execution_id_after_interrupted_response() {
        let mut receipt = receipt();
        receipt
            .migrations
            .get_mut("physician")
            .expect("migration")
            .outcome = Outcome::Unknown;
        let mut cloud = FakeCloud::new();
        cloud.writes.push("execute:physician-migrate".into());
        cloud.fail_migration = true;
        reconcile_migrations(&mut receipt, &mut cloud);
        assert_eq!(
            receipt.migrations["physician"].execution_id.as_deref(),
            Some("physician-migrate-execution")
        );
        assert_eq!(receipt.migrations["physician"].outcome, Outcome::Failed);
        assert_eq!(cloud.writes.len(), 1);
    }
    #[test]
    fn worker_failure_preserves_both_runtime_successes_and_cause() {
        let mut receipt = receipt();
        let mut cloud = FakeCloud::new();
        cloud.worker_failure = true;
        assert!(deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).is_err());
        assert_eq!(receipt.outcome, Outcome::PartialOrUnknown);
        assert_eq!(cloud.writes.len(), 7);
        for app in ["physician", "duet"] {
            assert_eq!(receipt.services[app].outcome, Outcome::Succeeded);
        }
        assert_eq!(
            receipt.services["physician-worker"].before,
            receipt.services["physician-worker"].final_state
        );
        assert_eq!(receipt.causes[0].safe_code(), "worker-update-failed");
        let mut summary = Vec::new();
        write_summary(&receipt, &mut summary).expect("summary");
        assert!(
            String::from_utf8(summary)
                .expect("utf8")
                .contains("PartialOrUnknown")
        );
    }
    #[test]
    fn unknown_lifecycle_states_fail_deserialization() {
        let mut value = serde_json::to_value(receipt()).expect("json");
        value["outcome"] = Value::String("invented".into());
        assert!(serde_json::from_value::<Receipt>(value).is_err());
        let mut value = serde_json::to_value(receipt()).expect("json");
        value["phase"] = Value::String("invented".into());
        assert!(serde_json::from_value::<Receipt>(value).is_err());
    }
    #[test]
    fn refuses_replaying_an_attempted_release() {
        let mut receipt = receipt();
        receipt.outcome = Outcome::PartialOrUnknown;
        let mut cloud = FakeCloud::new();
        assert!(deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).is_err());
        assert!(cloud.writes.is_empty());
    }
    #[test]
    fn rejects_production_and_duplicate_options() {
        let args = ["--project", "production", "--region", REGION].map(str::to_owned);
        assert!(initialize(&options(&args).expect("options")).is_err());
        assert!(options(&["--directory", "one", "--directory", "two"].map(str::to_owned)).is_err());
    }
    #[test]
    fn receipts_only_include_allowlisted_diagnostics_and_summaries() {
        let receipt = receipt();
        let value = serde_json::to_value(&receipt).expect("json");
        assert_eq!(value["source_sha"], "a".repeat(40));
        assert_eq!(value["tested_artifact_id"], "456");
        assert!(
            !serde_json::to_string(&receipt)
                .expect("json")
                .contains("containers")
        );
    }
}
