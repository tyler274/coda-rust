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
mod domain;
use domain::*;
mod shape;
use shape::Shape;

/// Polls after an async service update: 120 x 5 s covers image pull plus the
/// 120 s startup probe budget, and the shared `Gcloud` deadline still applies.
const ROLLOUT_POLLS: usize = 120;
const PROJECT: &str = "coda-rust-staging";
const REGION: &str = "us-east1";
const SERVICES: [ReleaseService; 3] = ReleaseService::ALL;

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
    Cloud(LocalCode),
    #[error("cloud command failed ({code})")]
    Command {
        code: LocalCode,
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
    fn safe_code(&self) -> LocalCode {
        match self {
            Self::Command { code, .. } | Self::Cloud(code) => *code,
            Self::Input(_) => LocalCode::InvalidInput,
            Self::Io(_) => LocalCode::IoFailed,
            Self::Json(_) => LocalCode::JsonInvalid,
            Self::Observations(_) => LocalCode::ObservationsFailed,
            Self::Related { primary, .. } | Self::Unrecorded { primary, .. } => primary.safe_code(),
        }
    }
    /// Allowlisted provider status of the primary cloud command failure, if any.
    fn api_status(&self) -> Option<ApiStatus> {
        match self {
            Self::Command { source, .. } => ApiStatus::from_stderr(&source.stderr),
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
    revision: RevisionId,
    percent: u64,
    latest: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Service {
    created_revision: RevisionId,
    ready_revision: RevisionId,
    image: ObservedImageRef,
    ready_image: ObservedImageRef,
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
    execution_id: Option<ExecutionId>,
    executions_before: BTreeSet<ExecutionId>,
    outcome: Outcome,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Diagnostic {
    phase: Phase,
    code: DiagnosticCode,
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
    images: Images,
    services: Services,
    migrations: Migrations,
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
    fn service(&mut self, service: ReleaseService) -> Result<Observation>;
    /// The service's current template shape; refuses one infrastructure
    /// would not produce.
    fn shape(&mut self, service: ReleaseService) -> Result<Shape>;
    fn update_job(&mut self, product: Product, images: &Images) -> Result<()>;
    fn job_image(&mut self, product: Product) -> Result<ObservedImageRef>;
    fn executions(&mut self, product: Product) -> Result<BTreeSet<ExecutionId>>;
    fn execute(&mut self, product: Product) -> Result<ExecutionId>;
    fn execution(&mut self, id: &ExecutionId, image: &PublishedImage) -> Result<Execution>;
    /// Updates only the image of `container`, or of the single unnamed
    /// container when it is `None`.
    fn update_service(
        &mut self,
        service: ReleaseService,
        container: Option<&str>,
        images: &Images,
    ) -> Result<()>;
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
fn diagnostic(receipt: &mut Receipt, code: DiagnosticCode) {
    receipt.diagnostics.push(Diagnostic {
        phase: receipt.phase,
        code,
    });
}
fn local_diagnostic(receipt: &mut Receipt, code: LocalCode) {
    diagnostic(receipt, DiagnosticCode::Local(code));
}
fn retain_cause(receipt: &mut Receipt, error: Error) {
    local_diagnostic(receipt, error.safe_code());
    if let Some(status) = error.api_status() {
        diagnostic(receipt, DiagnosticCode::Api(status));
    }
    receipt.causes.push(Arc::new(error));
}
fn observe(receipt: &mut Receipt, cloud: &mut impl Cloud, name: ReleaseService) -> Observation {
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
        let record = receipt.services.get_mut(name);
        if before {
            record.before = observation.clone();
        }
        record.final_state = observation;
    }
    complete
}
/// Whether a known service still specifies an image other than the target.
/// An accepted update records the new spec image at once.
fn spec_not_updated(service: &Observation, image: &PublishedImage) -> bool {
    matches!(service, Observation::Known(service) if !service.image.matches(image))
}
fn serving(service: &Observation, image: &PublishedImage) -> bool {
    let Observation::Known(service) = service else {
        return false;
    };
    service.ready_revision == service.created_revision
        && service.image.matches(image)
        && service.ready_image.matches(image)
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
    app: Product,
) -> Result<()> {
    let image = receipt
        .images
        .get(app.migration_image())
        .cloned()
        .ok_or(Error::Input("missing migration digest"))?;
    checkpoint(receipt, store, app.phase(MigrationStep::Update))?;
    if let Err(error) = cloud.update_job(app, &receipt.images) {
        retain_cause(receipt, error);
        if !cloud.job_image(app)?.matches(&image) {
            return Err(Error::Cloud(LocalCode::MigrationUpdateNotConfirmed));
        }
        local_diagnostic(receipt, LocalCode::JobUpdateResponseReconciled);
        store.save(receipt)?;
    }
    let previous = cloud.executions(app)?;
    let migration = receipt.migrations.get_mut(app);
    migration.outcome = Outcome::Unknown;
    migration.executions_before = previous.clone();
    checkpoint(receipt, store, app.phase(MigrationStep::Execute))?;
    let execution = match cloud.execute(app) {
        Ok(id) => id,
        Err(error) => {
            retain_cause(receipt, error);
            // Never retry an ambiguous execute: it may already have changed the database.
            let current = cloud.executions(app)?;
            let added: Vec<_> = current.difference(&previous).collect();
            if added.len() != 1 {
                return Err(Error::Cloud(LocalCode::MigrationExecutionUnknown));
            }
            local_diagnostic(receipt, LocalCode::ExecuteResponseReconciled);
            added[0].clone()
        }
    };
    receipt.migrations.get_mut(app).execution_id = Some(execution.clone());
    checkpoint(receipt, store, app.phase(MigrationStep::Wait))?;
    for _ in 0..120 {
        match cloud.execution(&execution, &image)? {
            Execution::Succeeded => {
                receipt.migrations.get_mut(app).outcome = Outcome::Succeeded;
                store.save(receipt)?;
                return Ok(());
            }
            Execution::Failed => {
                receipt.migrations.get_mut(app).outcome = Outcome::Failed;
                return save_settled(
                    store,
                    receipt,
                    Err(Error::Cloud(LocalCode::MigrationFailed)),
                );
            }
            Execution::Pending => cloud.pause(),
        }
    }
    Err(Error::Cloud(LocalCode::MigrationTimeout))
}
fn rollout(
    receipt: &mut Receipt,
    store: &mut impl Store,
    cloud: &mut impl Cloud,
    name: ReleaseService,
    before: &Shape,
) -> Result<()> {
    let image = receipt
        .images
        .get(name.image())
        .cloned()
        .ok_or(Error::Input("missing runtime digest"))?;
    receipt.services.get_mut(name).outcome = Outcome::Unknown;
    checkpoint(receipt, store, name.phase())?;
    // The update is submitted asynchronously; readiness is confirmed by bounded
    // polling rather than by a synchronous call that a per-call cap could kill.
    let update = cloud.update_service(name, before.container(), &receipt.images);
    let mut observed = observe(receipt, cloud, name);
    // A failed submit may still land shortly after it returns, so the update is
    // judged unapplied only after two consecutive reads, a pause apart, both
    // show a known service specifying another image.
    let mut unapplied = false;
    for _ in 0..ROLLOUT_POLLS {
        if serving(&observed, &image) || budget_exhausted(receipt) {
            break;
        }
        let stale = update.is_err() && spec_not_updated(&observed, &image);
        cloud.pause();
        observed = observe(receipt, cloud, name);
        if stale && spec_not_updated(&observed, &image) {
            unapplied = true;
            break;
        }
    }
    let ready = serving(&observed, &image);
    // Serving the image is not enough: nothing infrastructure owns may move.
    let preserved = ready.then(|| cloud.shape(name).and_then(|after| after.preserves(before)));
    let record = receipt.services.get_mut(name);
    record.final_state = observed;
    record.outcome = match &preserved {
        Some(Ok(())) => Outcome::Succeeded,
        Some(Err(Error::Cloud(code))) if code.is_shape_refusal() => Outcome::Failed,
        Some(Err(_)) | None => Outcome::Unknown,
    };
    if let Err(error) = update {
        retain_cause(receipt, error);
        local_diagnostic(
            receipt,
            if unapplied {
                LocalCode::UpdateFailedSpecUnchanged
            } else {
                LocalCode::UpdateResponseFailedReconciled
            },
        );
    }
    let result = match preserved {
        Some(result) => result,
        None => Err(Error::Cloud(LocalCode::RolloutNotConfirmed)),
    };
    save_settled(store, receipt, result)
}
/// Every present service's shape, read before any migration so a malformed
/// one stops the release while nothing has changed.
fn before_shapes(
    receipt: &Receipt,
    cloud: &mut impl Cloud,
) -> Result<Vec<(ReleaseService, Shape)>> {
    SERVICES
        .into_iter()
        .filter(|name| matches!(receipt.services.get(*name).before, Observation::Known(_)))
        .map(|name| Ok((name, cloud.shape(name)?)))
        .collect()
}
fn shape_of(shapes: &[(ReleaseService, Shape)], name: ReleaseService) -> Result<&Shape> {
    shapes
        .iter()
        .find(|(service, _)| *service == name)
        .map(|(_, shape)| shape)
        .ok_or(Error::Cloud(LocalCode::RequiredServiceAbsent))
}
fn deploy(receipt: &mut Receipt, store: &mut impl Store, cloud: &mut impl Cloud) -> Result<()> {
    if receipt.outcome != Outcome::Initialized {
        return Err(Error::Input(
            "receipt already attempted; initialize a new run attempt",
        ));
    }
    if !receipt.images.complete() {
        return Err(Error::Input("all five published images required"));
    }
    receipt.outcome = Outcome::InProgress;
    checkpoint(receipt, store, Phase::BeforeSnapshot)?;
    let before = snapshot(receipt, cloud, true);
    store.save(receipt)?;
    let result = (|| {
        if !before {
            return Err(Error::Cloud(LocalCode::BeforeStateUnknown));
        }
        for app in Product::ALL {
            if receipt.services.get(app.service()).before == Observation::Absent {
                return Err(Error::Cloud(LocalCode::RequiredServiceAbsent));
            }
        }
        let shapes = before_shapes(receipt, cloud)?;
        for app in Product::ALL {
            migrate(receipt, store, cloud, app)?;
            rollout(
                receipt,
                store,
                cloud,
                app.service(),
                shape_of(&shapes, app.service())?,
            )?;
        }
        if receipt.services.get(ReleaseService::PhysicianWorker).before == Observation::Absent {
            receipt
                .services
                .get_mut(ReleaseService::PhysicianWorker)
                .outcome = Outcome::BootstrapSkippedAbsent;
            store.save(receipt)?;
        } else {
            let worker = shape_of(&shapes, ReleaseService::PhysicianWorker)?;
            rollout(
                receipt,
                store,
                cloud,
                ReleaseService::PhysicianWorker,
                worker,
            )?;
        }
        Ok(())
    })();
    if let Err(error) = &result {
        local_diagnostic(
            receipt,
            match error {
                Error::Cloud(code) => *code,
                _ => LocalCode::ReleaseStopped,
            },
        );
        if let Some(status) = error.api_status() {
            diagnostic(receipt, DiagnosticCode::Api(status));
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
        Ok(()) if receipt.outcome != Outcome::Succeeded => {
            Some(receipt.causes.last().cloned().map_or(
                Error::Cloud(LocalCode::FinalStateUnknown),
                Error::Observations,
            ))
        }
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
    for app in Product::ALL {
        let migration = receipt.migrations.get_mut(app);
        if migration.outcome != Outcome::Unknown {
            continue;
        }
        if migration.execution_id.is_none() {
            match cloud.executions(app) {
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
            receipt.images.get(app.migration_image()),
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
    SERVICES.into_iter().all(|name| {
        let record = receipt.services.get(name);
        if name == ReleaseService::PhysicianWorker
            && record.outcome == Outcome::BootstrapSkippedAbsent
        {
            return record.final_state == Observation::Absent;
        }
        receipt
            .images
            .get(name.image())
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
        .ok_or(Error::Cloud(LocalCode::MissingCloudField))?;
    if !identifier(value) {
        return Err(Error::Cloud(LocalCode::InvalidCloudIdentifier));
    }
    Ok(value.to_owned())
}
fn cloud_image(value: &Value, pointer: &str) -> Result<ObservedImageRef> {
    let image = value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or(Error::Cloud(LocalCode::MissingCloudImage))?;
    ObservedImageRef::try_from(image.to_owned())
}
impl<R: command::Runner> Cloud for Gcloud<R> {
    fn service(&mut self, name: ReleaseService) -> Result<Observation> {
        let output = self.call(&["services", "describe", name.name()])?;
        if !output.status.success() {
            if command::structured_not_found(&output.stderr) {
                return Ok(Observation::Absent);
            }
            return Err(command::command_error(output));
        }
        let service: Value = serde_json::from_slice(&output.stdout)?;
        let ready = RevisionId::try_from(text(&service, "/status/latestReadyRevisionName")?)?;
        let revision = self.json(&["revisions", "describe", ready.as_str()])?;
        let template = service
            .pointer("/spec/template/spec")
            .ok_or(Error::Cloud(LocalCode::MissingCloudField))?;
        let ready_spec = revision
            .get("spec")
            .ok_or(Error::Cloud(LocalCode::MissingCloudField))?;
        let traffic = service
            .pointer("/status/traffic")
            .and_then(Value::as_array)
            .ok_or(Error::Cloud(LocalCode::MissingTraffic))?
            .iter()
            .map(|entry| {
                Ok(Traffic {
                    revision: RevisionId::try_from(text(entry, "/revisionName")?)?,
                    percent: entry.get("percent").and_then(Value::as_u64).unwrap_or(0),
                    latest: entry
                        .get("latestRevision")
                        .and_then(Value::as_bool)
                        .unwrap_or(false),
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Observation::Known(Service {
            created_revision: RevisionId::try_from(text(
                &service,
                "/status/latestCreatedRevisionName",
            )?)?,
            ready_revision: ready,
            image: cloud_image(shape::product(template, name.name())?, "/image")?,
            ready_image: cloud_image(shape::product(ready_spec, name.name())?, "/image")?,
            traffic,
        }))
    }
    fn update_job(&mut self, product: Product, images: &Images) -> Result<()> {
        let image = images
            .get(product.migration_image())
            .ok_or(Error::Input("missing migration digest"))?;
        self.json(&["jobs", "update", product.job(), "--image", image.as_str()])
            .map(|_| ())
    }
    fn job_image(&mut self, product: Product) -> Result<ObservedImageRef> {
        cloud_image(
            &self.json(&["jobs", "describe", product.job()])?,
            "/spec/template/spec/template/spec/containers/0/image",
        )
    }
    fn executions(&mut self, product: Product) -> Result<BTreeSet<ExecutionId>> {
        self.json(&["jobs", "executions", "list", "--job", product.job()])?
            .as_array()
            .ok_or(Error::Cloud(LocalCode::InvalidExecutions))?
            .iter()
            .map(|value| ExecutionId::try_from(text(value, "/metadata/name")?))
            .collect()
    }
    fn execute(&mut self, product: Product) -> Result<ExecutionId> {
        let output = self.call(&["jobs", "execute", product.job(), "--async"])?;
        if !output.status.success() {
            return Err(command::command_error(output));
        }
        let value: Value = serde_json::from_slice(&output.stdout)?;
        ExecutionId::try_from(text(&value, "/metadata/name")?)
    }
    fn execution(&mut self, id: &ExecutionId, image: &PublishedImage) -> Result<Execution> {
        let value = self.json(&["jobs", "executions", "describe", id.as_str()])?;
        if !cloud_image(&value, "/spec/template/spec/containers/0/image")?.matches(image) {
            return Err(Error::Cloud(LocalCode::ExecutionImageMismatch));
        }
        let conditions = value
            .pointer("/status/conditions")
            .and_then(Value::as_array)
            .ok_or(Error::Cloud(LocalCode::MissingExecutionStatus))?;
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
    fn shape(&mut self, name: ReleaseService) -> Result<Shape> {
        let service = self.json(&["services", "describe", name.name()])?;
        let template = service
            .pointer("/spec/template")
            .ok_or(Error::Cloud(LocalCode::MissingCloudField))?;
        shape::read(template, name.name())
    }
    fn update_service(
        &mut self,
        name: ReleaseService,
        container: Option<&str>,
        images: &Images,
    ) -> Result<()> {
        let image = images
            .get(name.image())
            .ok_or(Error::Input("missing runtime digest"))?;
        let mut args = vec!["services", "update", name.name()];
        // `--image` follows `--container`, so it never lands on the sidecar.
        // Service flags stay in front. The GA CLI assigns every later flag
        // to that container and rejects them before the API call.
        if let Some(container) = container {
            args.extend(["--async", "--container", container]);
        }
        args.extend(["--image", image.as_str()]);
        if container.is_none() {
            args.push("--async");
        }
        self.json(&args).map(|_| ())
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
        images: Images::default(),
        services: Services::new(ServiceRecord {
            before: Observation::Unknown,
            final_state: Observation::Unknown,
            outcome: Outcome::NotAttempted,
        }),
        migrations: Migrations::new(Migration {
            execution_id: None,
            executions_before: BTreeSet::new(),
            outcome: Outcome::NotAttempted,
        }),
        phase: Phase::Publication,
        outcome: Outcome::Initialized,
        diagnostics: Vec::new(),
        causes: Vec::new(),
    })
}
fn published_images(receipt: &mut Receipt, directory: &Path) -> Result<()> {
    receipt.images = Images::default();
    let manifest = directory.join("tested-images.json");
    if manifest.exists() {
        let manifest: Value = serde_json::from_slice(&fs::read(manifest)?)?;
        if manifest.get("source_revision").and_then(Value::as_str) != Some(&receipt.source_sha) {
            return Err(Error::Input(
                "published bundle differs from receipt source SHA",
            ));
        }
    } else if ImageRole::ALL
        .iter()
        .any(|role| directory.join(role.key()).exists())
    {
        return Err(Error::Input(
            "published digest has no tested bundle identity",
        ));
    }
    for role in ImageRole::ALL {
        let path = directory.join(role.key());
        if !path.exists() {
            continue;
        }
        let image = fs::read_to_string(path)?;
        receipt
            .images
            .insert(PublishedImage::parse(role, image.trim())?);
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
        return record_initial_state(&mut receipt, &mut store, &mut cloud);
    }
    let mut receipt: Receipt = serde_json::from_slice(&fs::read(&store.0)?)?;
    if receipt.schema_version != 1 || receipt.project != PROJECT || receipt.region != REGION {
        return Err(Error::Input("invalid receipt contract"));
    }
    match command.as_str() {
        "deploy" => {
            let images = published_images(&mut receipt, directory);
            if images.is_err() {
                local_diagnostic(&mut receipt, LocalCode::InvalidPublicationEvidence);
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
                local_diagnostic(&mut receipt, LocalCode::FinalizedIncompleteRelease);
            }
            let result = images.and_then(|()| {
                if receipt.outcome == Outcome::Succeeded {
                    Ok(())
                } else {
                    Err(receipt.causes.last().cloned().map_or(
                        Error::Cloud(LocalCode::ReleaseIncomplete),
                        Error::Observations,
                    ))
                }
            });
            settle_finalize(
                &mut store,
                &receipt,
                result,
                |receipt| match std::env::var_os("GITHUB_STEP_SUMMARY") {
                    Some(path) => write_summary(
                        receipt,
                        &mut OpenOptions::new().append(true).create(true).open(path)?,
                    ),
                    None => Ok(()),
                },
            )
        }
        _ => Err(Error::Input("expected init, deploy, or finalize")),
    }
}
/// Records the release's state before any change; a failed save keeps the
/// snapshot failure it was recording.
fn record_initial_state(
    receipt: &mut Receipt,
    store: &mut impl Store,
    cloud: &mut impl Cloud,
) -> Result<()> {
    let result = if snapshot(receipt, cloud, true) {
        Ok(())
    } else {
        Err(receipt.causes.last().cloned().map_or(
            Error::Cloud(LocalCode::InitialStateUnknown),
            Error::Observations,
        ))
    };
    save_settled(store, receipt, result)
}
/// Saves the final receipt and, only once it is saved, appends the step
/// summary that points to it. Neither failure replaces an already-settled
/// release failure.
fn settle_finalize(
    store: &mut impl Store,
    receipt: &Receipt,
    result: Result<()>,
    summarize: impl FnOnce(&Receipt) -> Result<()>,
) -> Result<()> {
    if let Err(persistence) = store.save(receipt) {
        return Err(match result {
            Ok(()) => persistence,
            Err(primary) => Error::Unrecorded {
                primary: Box::new(primary),
                persistence: Box::new(persistence),
            },
        });
    }
    let summarized = summarize(receipt);
    result.and(summarized)
}
fn write_summary(receipt: &Receipt, writer: &mut impl Write) -> Result<()> {
    writeln!(
        writer,
        "## Staging release\n\nOutcome: `{:?}`; last phase: `{:?}`.",
        receipt.outcome, receipt.phase
    )?;
    for name in SERVICES {
        let record = receipt.services.get(name);
        let name = name.name();
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
    for product in Product::ALL {
        let migration = receipt.migrations.get(product);
        writeln!(
            writer,
            "- {} migration: `{:?}`; execution: `{}`",
            product.name(),
            migration.outcome,
            migration
                .execution_id
                .as_ref()
                .map_or("not identified", ExecutionId::as_str)
        )?;
    }
    for diagnostic in &receipt.diagnostics {
        let code = match diagnostic.code {
            DiagnosticCode::Local(code) => code.as_str(),
            DiagnosticCode::Api(code) => code.as_str(),
        };
        writeln!(writer, "- Diagnostic: `{:?}` / `{code}`", diagnostic.phase)?;
    }
    writeln!(
        writer,
        "\nSee the release receipt artifact for digest, execution, revision and traffic evidence."
    )?;
    if receipt.outcome != Outcome::Succeeded {
        writeln!(
            writer,
            "\nInspect the recorded migration execution and final traffic in `{PROJECT}` / `{REGION}` before recovery. Do not reexecute an uncertain migration or assume a failed release left services unchanged. Fix forward; previous-digest rollback requires schema compatibility proof and operator approval."
        )?;
    }
    Ok(())
}
fn failure_message(error: &Error) -> String {
    let mut message = match error.api_status() {
        Some(status) => format!(
            "staging release failed: {} (API status {status})",
            error.safe_code()
        ),
        None => format!("staging release failed: {}", error.safe_code()),
    };
    if let Error::Unrecorded { persistence, .. } = error {
        message.push_str(&format!(
            "; receipt persistence failed: {} (receipt may be incomplete)",
            persistence.safe_code()
        ));
    }
    message
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
                return Err(Error::Cloud(LocalCode::ReceiptUnwritable));
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
        // The service update is refused and leaves the service unchanged.
        denied_update: bool,
        // The submit times out, but the update lands after the first read.
        late_update: bool,
        // Service reads fail once this many have succeeded.
        reads_before_failure: Option<usize>,
        // Each service's template, and the container each update targeted.
        templates: BTreeMap<String, Value>,
        targets: Vec<Option<String>>,
        // The update also moves the otel-collector sidecar's image.
        sidecar_drift: bool,
    }
    fn denied() -> Error {
        Error::Command {
            code: LocalCode::GcloudCommandFailed,
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
                    .map(|name| (name.name().into(), known("old-image")))
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
                denied_update: false,
                late_update: false,
                reads_before_failure: None,
                templates: SERVICES
                    .iter()
                    .map(|name| {
                        (
                            name.name().into(),
                            shape::tests::observed(name.name(), "old-image"),
                        )
                    })
                    .collect(),
                targets: Vec::new(),
                sidecar_drift: false,
            }
        }
    }
    fn known(image: &str) -> Observation {
        Observation::Known(Service {
            created_revision: RevisionId::try_from("ready-revision".to_owned()).expect("revision"),
            ready_revision: RevisionId::try_from("ready-revision".to_owned()).expect("revision"),
            image: ObservedImageRef::try_from(image.to_owned()).expect("image"),
            ready_image: ObservedImageRef::try_from(image.to_owned()).expect("image"),
            traffic: vec![Traffic {
                revision: RevisionId::try_from("ready-revision".to_owned()).expect("revision"),
                percent: 100,
                latest: true,
            }],
        })
    }
    impl Cloud for FakeCloud {
        fn service(&mut self, name: ReleaseService) -> Result<Observation> {
            let name = name.name();
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
        fn shape(&mut self, name: ReleaseService) -> Result<Shape> {
            shape::read(&self.templates[name.name()], name.name())
        }
        fn update_job(&mut self, product: Product, images: &Images) -> Result<()> {
            images
                .get(product.migration_image())
                .ok_or(Error::Input("missing migration digest"))?;
            self.writes.push(format!("job:{}", product.job()));
            Ok(())
        }
        fn job_image(&mut self, _: Product) -> Result<ObservedImageRef> {
            Err(Error::Cloud(LocalCode::Unused))
        }
        fn executions(&mut self, product: Product) -> Result<BTreeSet<ExecutionId>> {
            let name = product.job();
            Ok(
                if !self.unknown_execution && self.writes.contains(&format!("execute:{name}")) {
                    BTreeSet::from([
                        ExecutionId::try_from(format!("{name}-execution")).expect("execution")
                    ])
                } else {
                    BTreeSet::new()
                },
            )
        }
        fn execute(&mut self, product: Product) -> Result<ExecutionId> {
            let name = product.job();
            self.writes.push(format!("execute:{name}"));
            if self.ambiguous_execute {
                Err(Error::Cloud(LocalCode::LostResponse))
            } else {
                ExecutionId::try_from(format!("{name}-execution"))
            }
        }
        fn execution(&mut self, id: &ExecutionId, _: &PublishedImage) -> Result<Execution> {
            if self.denied_execution {
                return Err(denied());
            }
            Ok(
                if self.fail_migration
                    || (self.fail_duet_migration && id.as_str().starts_with("duet-"))
                {
                    Execution::Failed
                } else {
                    Execution::Succeeded
                },
            )
        }
        fn update_service(
            &mut self,
            service: ReleaseService,
            container: Option<&str>,
            images: &Images,
        ) -> Result<()> {
            let name = service.name();
            let image = images
                .get(service.image())
                .ok_or(Error::Input("missing runtime digest"))?
                .as_str();
            self.writes.push(format!("service:{name}"));
            self.targets.push(container.map(str::to_owned));
            // gcloud sets the image of the container it was told to target.
            let template = self.templates.get_mut(name).expect("template");
            *template = shape::tests::released(template, container.unwrap_or(""), image);
            if self.sidecar_drift {
                template["spec"]["containers"][0]["image"] = image.into();
            }
            if self.denied_update {
                return Err(denied());
            }
            if self.late_update {
                self.pending.insert(name.into(), (known(image), 1));
                return Err(Error::Cloud(LocalCode::GcloudTimeout));
            }
            if name == "physician-worker" && self.worker_failure {
                return Err(Error::Cloud(LocalCode::WorkerUpdateFailed));
            }
            let mut state = known(image);
            if let Observation::Known(service) = &mut state {
                if self.wrong_ready_image {
                    service.ready_image =
                        ObservedImageRef::try_from("unrelated-image".to_owned()).expect("image");
                }
                if self.split_traffic {
                    service.traffic[0].percent = 50;
                }
            }
            if self.not_ready_observations > 0 {
                // Cloud Run records the new spec at once; readiness lags behind it.
                if let Some(Observation::Known(current)) = self.services.get_mut(name) {
                    current.image = ObservedImageRef::try_from(image.to_owned()).expect("image");
                }
                self.pending
                    .insert(name.into(), (state, self.not_ready_observations));
            } else {
                self.services.insert(name.into(), state);
            }
            if self.ambiguous_update {
                Err(Error::Cloud(LocalCode::LostResponse))
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
        for role in ImageRole::ALL {
            receipt.images.insert(
                PublishedImage::parse(
                    role,
                    &format!(
                        "{REGION}-docker.pkg.dev/{PROJECT}/apps/{}@sha256:{}",
                        role.repository(),
                        "b".repeat(64)
                    ),
                )
                .expect("published image"),
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
        assert!(receipt.images.complete());
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
    fn two_container_services_update_only_the_named_product_container() {
        let mut receipt = receipt();
        let mut cloud = FakeCloud::new();
        cloud
            .templates
            .insert("duet".into(), shape::tests::single("old-image"));
        deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).expect("release");
        assert_eq!(receipt.outcome, Outcome::Succeeded);
        assert_eq!(
            cloud.targets,
            [
                Some("physician".to_owned()),
                None,
                Some("physician-worker".to_owned())
            ]
        );
        for name in SERVICES {
            let template = &cloud.templates[name.name()];
            let image = receipt.images.get(name.image()).expect("image").as_str();
            assert_eq!(
                shape::product(&template["spec"], name.name()).expect("product")["image"],
                image
            );
        }
        // The sidecar keeps its own image.
        assert_eq!(
            cloud.templates["physician"]["spec"]["containers"][0]["image"],
            "otel/opentelemetry-collector-contrib:0.155.0"
        );
    }
    #[test]
    fn malformed_sidecar_shape_is_refused_before_any_migration() {
        for service in ["duet", "physician-worker"] {
            let mut receipt = receipt();
            let mut cloud = FakeCloud::new();
            // The product container is not named after its service.
            cloud.templates.get_mut(service).expect("template")["spec"]["containers"][1]["name"] =
                "app".into();
            let error =
                deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).expect_err("refused");
            assert_eq!(error.safe_code().as_str(), "unexpected-container-set");
            assert!(cloud.writes.is_empty(), "{service}: {:?}", cloud.writes);
            assert_eq!(receipt.outcome, Outcome::PartialOrUnknown);
        }
    }
    #[test]
    fn sidecar_drift_after_rollout_fails_and_stops_later_services() {
        let mut receipt = receipt();
        let mut cloud = FakeCloud::new();
        cloud.sidecar_drift = true;
        let error =
            deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).expect_err("drift");
        assert_eq!(error.safe_code().as_str(), "infrastructure-field-changed");
        assert_eq!(
            cloud.writes,
            [
                "job:physician-migrate",
                "execute:physician-migrate",
                "service:physician"
            ]
        );
        assert_eq!(
            receipt.services.get(ReleaseService::Physician).outcome,
            Outcome::Failed
        );
        assert_eq!(receipt.outcome, Outcome::PartialOrUnknown);
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
            receipt
                .migrations
                .get(Product::Physician)
                .execution_id
                .as_ref()
                .map(ExecutionId::as_str),
            Some("physician-migrate-execution")
        );
        assert_eq!(
            receipt.migrations.get(Product::Physician).outcome,
            Outcome::Failed
        );
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
        assert_eq!(
            receipt.migrations.get(Product::Physician).outcome,
            Outcome::Unknown
        );
        assert!(
            receipt
                .migrations
                .get(Product::Physician)
                .execution_id
                .is_none()
        );
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
                    receipt
                        .services
                        .get(ReleaseService::PhysicianWorker)
                        .outcome,
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
        assert_eq!(error.safe_code().as_str(), "rollout-not-confirmed");
        assert_eq!(cloud.pauses, ROLLOUT_POLLS);
        assert_eq!(cloud.writes.len(), 3);
        assert_eq!(
            receipt.services.get(ReleaseService::Physician).outcome,
            Outcome::Unknown
        );
    }
    #[test]
    fn refused_update_that_left_the_old_spec_stops_after_one_confirming_read() {
        let mut receipt = receipt();
        let mut cloud = FakeCloud::new();
        cloud.denied_update = true;
        let error =
            deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).expect_err("denied");
        assert_eq!(error.safe_code().as_str(), "rollout-not-confirmed");
        // One confirming read a pause after the first, then no more polling.
        assert_eq!(cloud.pauses, 1);
        assert_eq!(
            receipt.services.get(ReleaseService::Physician).outcome,
            Outcome::Unknown
        );
        let encoded = serde_json::to_string(&receipt).expect("receipt");
        assert!(encoded.contains("\"PERMISSION_DENIED\""));
        assert!(encoded.contains("\"update-failed-spec-unchanged\""));
        assert!(!encoded.contains("\"update-response-failed-reconciled\""));
        assert!(!encoded.contains("secret"));
    }
    #[test]
    fn failed_initial_snapshot_survives_an_unwritable_receipt() {
        let mut cloud = FakeCloud::new();
        cloud.reads_before_failure = Some(0);
        let mut store = MemoryStore {
            fail_at: Some(0),
            ..MemoryStore::default()
        };
        let error = record_initial_state(&mut receipt(), &mut store, &mut cloud).unwrap_err();
        let Error::Unrecorded { primary, .. } = &error else {
            panic!("save error replaced the snapshot failure: {error:?}");
        };
        assert_eq!(primary.safe_code().as_str(), "observations-failed");
        let Error::Unrecorded { persistence, .. } = &error else {
            unreachable!()
        };
        assert_eq!(persistence.safe_code().as_str(), "receipt-unwritable");
    }
    fn broken_summary(_: &Receipt) -> Result<()> {
        Err(std::io::Error::other("summary unwritable").into())
    }
    #[test]
    fn failed_summary_identifies_migration_and_sanitized_cause() {
        let mut receipt = receipt();
        let mut cloud = FakeCloud::new();
        cloud.fail_migration = true;
        deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).unwrap_err();
        let mut output = Vec::new();
        write_summary(&receipt, &mut output).unwrap();
        let output = String::from_utf8(output).unwrap();
        let execution = receipt
            .migrations
            .get(Product::Physician)
            .execution_id
            .as_ref()
            .unwrap();
        assert!(
            output.contains(execution.as_str()),
            "missing migration execution identity"
        );
        assert!(
            output.contains("migration-failed"),
            "missing sanitized diagnostic"
        );
        assert!(
            output.contains("Do not reexecute"),
            "missing ambiguous-execution guidance"
        );
    }

    #[test]
    fn failure_message_exposes_receipt_loss_without_raw_diagnostics() {
        let error = Error::Unrecorded {
            primary: Box::new(denied()),
            persistence: Box::new(
                std::io::Error::other("private path and credential sentinel").into(),
            ),
        };
        let message = failure_message(&error);
        assert!(message.contains("PERMISSION_DENIED"));
        assert!(
            message.contains("receipt persistence failed: io-failed"),
            "missing receipt persistence failure"
        );
        assert!(!message.contains("sentinel"));
        assert!(!message.contains("private"));
    }

    #[test]
    fn failed_release_still_writes_its_step_summary() {
        let mut written = false;
        let failed = Err(Error::Cloud(LocalCode::ReleaseIncomplete));
        let error = settle_finalize(&mut MemoryStore::default(), &receipt(), failed, |_| {
            written = true;
            Ok(())
        })
        .unwrap_err();
        assert_eq!(error.safe_code().as_str(), "release-incomplete");
        assert!(written);
    }
    #[test]
    fn unwritable_final_receipt_skips_the_summary_and_keeps_both_errors() {
        for (result, code) in [
            (Ok(()), "receipt-unwritable"),
            (
                Err(Error::Cloud(LocalCode::ReleaseIncomplete)),
                "release-incomplete",
            ),
        ] {
            let failed = result.is_err();
            let mut store = MemoryStore {
                fail_at: Some(0),
                ..MemoryStore::default()
            };
            let mut written = false;
            let error = settle_finalize(&mut store, &receipt(), result, |_| {
                written = true;
                Ok(())
            })
            .unwrap_err();
            assert_eq!(error.safe_code().as_str(), code);
            assert_eq!(matches!(error, Error::Unrecorded { .. }), failed);
            assert!(!written, "summary points to a receipt that was not saved");
        }
    }
    #[test]
    fn unwritable_step_summary_keeps_the_release_result() {
        let mut store = MemoryStore::default();
        let failed = Err(Error::Cloud(LocalCode::ReleaseIncomplete));
        let error = settle_finalize(&mut store, &receipt(), failed, broken_summary).unwrap_err();
        assert_eq!(error.safe_code().as_str(), "release-incomplete");
        assert_eq!(store.receipts.len(), 1);
        let error = settle_finalize(&mut store, &receipt(), Ok(()), broken_summary).unwrap_err();
        assert!(matches!(error, Error::Io(_)), "{error:?}");
    }
    #[test]
    fn timed_out_update_that_lands_after_the_first_read_is_still_confirmed() {
        let mut receipt = receipt();
        let mut cloud = FakeCloud::new();
        cloud.late_update = true;
        deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).expect("release");
        assert_eq!(receipt.outcome, Outcome::Succeeded);
        assert_eq!(cloud.pauses, 3);
    }
    #[test]
    fn applied_update_with_lost_response_keeps_polling_until_ready() {
        let mut receipt = receipt();
        let mut cloud = FakeCloud::new();
        cloud.ambiguous_update = true;
        cloud.not_ready_observations = 3;
        deploy(&mut receipt, &mut MemoryStore::default(), &mut cloud).expect("release");
        assert_eq!(receipt.outcome, Outcome::Succeeded);
        assert_eq!(cloud.pauses, 9);
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
            assert_eq!(persistence.safe_code().as_str(), "receipt-unwritable");
            assert_eq!(
                error.safe_code().as_str(),
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
        assert_eq!(error.safe_code().as_str(), "migration-failed");
        assert!(
            !cloud
                .writes
                .iter()
                .any(|write| write.starts_with("service:"))
        );
    }
    #[test]
    fn reviewer_reproduction_failed_migration_with_sixth_save_unwritable() {
        // Saves: before-snapshot, snapshot, three migration checkpoints, then
        // the save that records the failed execution (index 5).
        let mut cloud = FakeCloud::new();
        cloud.fail_migration = true;
        let mut store = MemoryStore {
            fail_at: Some(5),
            ..MemoryStore::default()
        };
        let error = deploy(&mut receipt(), &mut store, &mut cloud).expect_err("failed");
        assert!(matches!(error, Error::Unrecorded { .. }), "{error:?}");
        assert_eq!(error.safe_code().as_str(), "migration-failed");
        assert_eq!(store.receipts.len(), 5);
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
        assert_eq!(error.safe_code().as_str(), "observations-failed");
        assert_eq!(error.api_status(), Some(ApiStatus::PermissionDenied));
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
        let mut images = Images::default();
        for role in ImageRole::ALL {
            if role != ImageRole::DuetMigration {
                images.insert(receipt.images.get(role).expect("image").clone());
            }
        }
        receipt.images = images;
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
        assert_eq!(
            receipt.services.get(ReleaseService::Physician).outcome,
            Outcome::Succeeded
        );
        assert!(serving(
            &receipt.services.get(ReleaseService::Physician).final_state,
            receipt
                .images
                .get(ImageRole::PhysicianRuntime)
                .expect("image")
        ));
        assert_eq!(
            receipt.migrations.get(Product::Duet).outcome,
            Outcome::Failed
        );
        assert_eq!(
            receipt.services.get(ReleaseService::Duet).before,
            receipt.services.get(ReleaseService::Duet).final_state
        );
    }
    #[test]
    fn finalization_recovers_execution_id_after_interrupted_response() {
        let mut receipt = receipt();
        receipt.migrations.get_mut(Product::Physician).outcome = Outcome::Unknown;
        let mut cloud = FakeCloud::new();
        cloud.writes.push("execute:physician-migrate".into());
        cloud.fail_migration = true;
        reconcile_migrations(&mut receipt, &mut cloud);
        assert_eq!(
            receipt
                .migrations
                .get(Product::Physician)
                .execution_id
                .as_ref()
                .map(ExecutionId::as_str),
            Some("physician-migrate-execution")
        );
        assert_eq!(
            receipt.migrations.get(Product::Physician).outcome,
            Outcome::Failed
        );
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
        for app in Product::ALL {
            assert_eq!(
                receipt.services.get(app.service()).outcome,
                Outcome::Succeeded
            );
        }
        assert_eq!(
            receipt.services.get(ReleaseService::PhysicianWorker).before,
            receipt
                .services
                .get(ReleaseService::PhysicianWorker)
                .final_state
        );
        assert_eq!(
            receipt.causes[0].safe_code().as_str(),
            "worker-update-failed"
        );
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
    fn receipt_wire_shape_round_trips_without_changing_keys() {
        let mut receipt = receipt();
        deploy(
            &mut receipt,
            &mut MemoryStore::default(),
            &mut FakeCloud::new(),
        )
        .expect("deploy");
        local_diagnostic(&mut receipt, LocalCode::UpdateResponseFailedReconciled);
        diagnostic(
            &mut receipt,
            DiagnosticCode::Api(ApiStatus::PermissionDenied),
        );
        let value = serde_json::to_value(&receipt).expect("json");
        let keys = |value: &Value| {
            value
                .as_object()
                .expect("object")
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>()
        };
        assert_eq!(
            keys(&value["services"]),
            ["duet", "physician", "physician-worker"]
                .map(str::to_owned)
                .into()
        );
        assert_eq!(
            keys(&value["migrations"]),
            ["duet", "physician"].map(str::to_owned).into()
        );
        assert_eq!(
            keys(&value["images"]),
            [
                "duet-migration",
                "duet-runtime",
                "physician-migration",
                "physician-runtime",
                "physician-worker"
            ]
            .map(str::to_owned)
            .into()
        );
        assert_eq!(value["schema_version"], 1);
        assert_eq!(
            value["migrations"]["physician"]["execution_id"],
            "physician-migrate-execution"
        );
        assert_eq!(
            value["services"]["physician"]["final_state"]["summary"]["ready_revision"],
            "ready-revision"
        );
        assert_eq!(
            value["diagnostics"][0]["code"],
            "update-response-failed-reconciled"
        );
        assert_eq!(value["diagnostics"][1]["code"], "PERMISSION_DENIED");
        let decoded: Receipt = serde_json::from_value(value.clone()).expect("receipt");
        assert_eq!(serde_json::to_value(decoded).expect("json"), value);
    }
    #[test]
    fn fixed_containers_require_every_key_but_allow_null_images() {
        let mut receipt = receipt();
        receipt.images = Images::default();
        let value = serde_json::to_value(receipt).expect("json");
        let decoded: Receipt = serde_json::from_value(value.clone()).expect("null images");
        assert!(!decoded.images.complete());
        for container in ["images", "services", "migrations"] {
            for key in value[container].as_object().expect("object").keys() {
                let mut missing = value.clone();
                missing[container]
                    .as_object_mut()
                    .expect("object")
                    .remove(key);
                assert!(
                    serde_json::from_value::<Receipt>(missing).is_err(),
                    "{container}.{key} required"
                );
            }
            let mut extra = value.clone();
            extra[container]["invented"] = Value::Null;
            assert!(serde_json::from_value::<Receipt>(extra).is_err());
        }
    }
    #[test]
    fn receipt_rejects_misassigned_or_invalid_images_and_diagnostics() {
        let value = serde_json::to_value(receipt()).expect("json");
        for invalid in [
            "arbitrary-text",
            "PERMISSION_DENIED: secret",
            "permission_denied",
        ] {
            let mut bad = value.clone();
            bad["diagnostics"] = serde_json::json!([{"phase":"publication", "code": invalid}]);
            assert!(serde_json::from_value::<Receipt>(bad).is_err());
        }
        for role in ImageRole::ALL {
            let mut swapped = value.clone();
            let other = if role == ImageRole::DuetRuntime {
                ImageRole::PhysicianRuntime
            } else {
                ImageRole::DuetRuntime
            };
            swapped["images"][role.key()] = value["images"][other.key()].clone();
            assert!(serde_json::from_value::<Receipt>(swapped).is_err());
        }
        let mut invalid = value;
        invalid["images"]["physician-runtime"] = "image:tag".into();
        assert!(serde_json::from_value::<Receipt>(invalid).is_err());
    }
    #[test]
    fn publication_still_requires_matching_tested_bundle_identity() {
        let directory = std::env::temp_dir().join(format!("release-domain-{}", std::process::id()));
        fs::create_dir_all(&directory).expect("directory");
        let mut receipt = receipt();
        let image = receipt
            .images
            .get(ImageRole::PhysicianRuntime)
            .expect("image")
            .clone();
        fs::write(directory.join("physician-runtime"), image.as_str()).expect("digest");
        assert!(published_images(&mut receipt, &directory).is_err());
        fs::write(
            directory.join("tested-images.json"),
            serde_json::json!({"source_revision": "c".repeat(40)}).to_string(),
        )
        .expect("manifest");
        assert!(published_images(&mut receipt, &directory).is_err());
        fs::write(
            directory.join("tested-images.json"),
            serde_json::json!({"source_revision": receipt.source_sha}).to_string(),
        )
        .expect("manifest");
        published_images(&mut receipt, &directory).expect("partial publication");
        assert_eq!(
            receipt.images.get(ImageRole::PhysicianRuntime),
            Some(&image)
        );
        assert!(receipt.images.get(ImageRole::DuetRuntime).is_none());
        assert!(!receipt.images.complete());
        fs::remove_dir_all(directory).expect("cleanup");
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
