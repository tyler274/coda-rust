//! Operator-only, staging-only recovery drill. Cloud restoration is not product qualification.
mod cloud;
mod receipt;
#[cfg(test)]
mod tests;
mod types;

use crate::failure::Failure;
use cloud::{Cloud, Gcloud, Instance, Operation};
use receipt::{Cleanup, Phase, Receipt, ReceiptStore};
use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use types::{BackupId, OperationKind, OperationStatus, RestoreTarget, SqlOperationId};

const PROJECT: &str = "coda-rust-staging";
const NETWORK: &str = "projects/coda-rust-staging/global/networks/rust-private";
/// Cloud SQL's minimum through the target's auto-resize cap (`--storage-auto-increase-limit`).
const STORAGE_GB: std::ops::RangeInclusive<u64> = 10..=50;
const WAIT_LIMIT: Duration = Duration::from_secs(1800);
const POLL_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Debug, thiserror::Error)]
pub(super) enum Error {
    #[error("restore refused: {0}")]
    Unsafe(String),
    #[error(transparent)]
    UnknownOperationStatus(#[from] types::UnknownOperationStatus),
    #[error("restore I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid cloud/receipt JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("cloud command failed ({code:?}): {stderr}")]
    Cloud { code: Option<i32>, stderr: String },
    #[error("cloud operation failed: {details}")]
    Operation { details: serde_json::Value },
    #[error(
        "cloud operation exceeded its bounded wait; inspect the receipt and operation before retrying"
    )]
    Timeout,
    #[error("invalid backup timestamp: {0}")]
    Timestamp(#[from] chrono::ParseError),
    #[error("system clock precedes Unix epoch: {0}")]
    Clock(#[from] std::time::SystemTimeError),
    #[error("backup timestamp precedes Unix epoch: {0}")]
    Epoch(#[from] std::num::TryFromIntError),
    /// The operation failed and its receipt could not record that failure.
    #[error("restore failed and its receipt could not be saved ({persistence})")]
    Unrecorded {
        #[source]
        primary: Box<Error>,
        persistence: Box<Error>,
    },
}
impl Error {
    fn receipt_code(&self) -> receipt::FailureCode {
        use receipt::FailureCode;
        match self {
            Self::Unsafe(_) | Self::UnknownOperationStatus(_) => FailureCode::ValidationRefused,
            Self::Io(_) => FailureCode::Io,
            Self::Json(_) => FailureCode::InvalidJson,
            Self::Cloud { .. } => FailureCode::CloudCommand,
            Self::Operation { .. } => FailureCode::CloudOperation,
            Self::Timeout => FailureCode::Timeout,
            Self::Timestamp(_) | Self::Clock(_) | Self::Epoch(_) => FailureCode::InvalidTime,
            Self::Unrecorded { primary, .. } => primary.receipt_code(),
        }
    }
}
type Result<T> = std::result::Result<T, Error>;
fn refuse(message: impl Into<String>) -> Error {
    Error::Unsafe(message.into())
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub(super) enum Product {
    Physician,
    Duet,
}
impl Product {
    fn name(self) -> &'static str {
        match self {
            Self::Physician => "physician",
            Self::Duet => "duet",
        }
    }
    fn source(self) -> String {
        format!("rust-{}", self.name())
    }
}

struct Request {
    product: Product,
    backup: String,
    target: String,
}
impl Request {
    fn validate(&self) -> Result<(RestoreTarget, BackupId)> {
        Ok((
            RestoreTarget::new(self.product, self.target.clone())?,
            BackupId::try_from(self.backup.clone())?,
        ))
    }
}

impl Error {
    /// Allowlisted provider status of the primary cloud command failure, if any.
    fn api_status(&self) -> Option<&'static str> {
        match self {
            Self::Cloud { stderr, .. } => crate::failure::api_status(stderr.as_bytes()),
            Self::Unrecorded { primary, .. } => primary.api_status(),
            _ => None,
        }
    }
}
fn failure_message(error: &Error) -> String {
    let status = error
        .api_status()
        .map(|status| format!(" (API status {status})"))
        .unwrap_or_default();
    format!(
        "staging restore failed: {:?}{status}; inspect the receipt phase and operation with the authorized operator",
        error.receipt_code()
    )
}
pub fn from_args(args: &[String]) -> std::result::Result<(), Failure> {
    run_cli(args).map_err(|error| Failure::with_source(1, failure_message(&error), error))
}

const USAGE: &str = "staging-restore --product physician|duet --backup ID --target rust-restore-<product>-<run> --receipt PATH [--execute --authorize-staging-restore]\n\
Default is read-only cloud preflight. A fresh private target is created only with both execution flags. Existing targets are always refused.\n\
staging-restore --cleanup --receipt PATH --authorize-staging-delete\n\
Cloud completion is NOT recovery qualification. Run the product-owned migration, schema, readback and permission checks on the isolated target; Duet additionally requires duet-privacy-replay with NO IDs against the complete independent journal inventory, and a synthetic post-backup deletion proof. Keep serving, workers, queues and messaging disconnected. Receipts always retain these validation gaps. Cleanup is explicit and identity-checked.";

#[derive(Default)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "execute, cleanup and their authorizations are one mode not yet modelled as an enum"
)]
struct Options {
    product: Option<Product>,
    backup: Option<String>,
    target: Option<String>,
    receipt: Option<PathBuf>,
    execute: bool,
    authorize: bool,
    cleanup: bool,
    delete: bool,
}
impl Options {
    fn parse(args: &[String]) -> Result<Self> {
        let mut result = Self::default();
        let mut seen = std::collections::BTreeSet::new();
        let mut args = args.iter();
        while let Some(arg) = args.next() {
            if !seen.insert(arg) {
                return Err(refuse(format!("duplicate argument {arg}")));
            }
            match arg.as_str() {
                "--execute" => result.execute = true,
                "--authorize-staging-restore" => result.authorize = true,
                "--cleanup" => result.cleanup = true,
                "--authorize-staging-delete" => result.delete = true,
                "--product" => {
                    result.product = Some(match args.next().map(String::as_str) {
                        Some("physician") => Product::Physician,
                        Some("duet") => Product::Duet,
                        _ => return Err(refuse("product must be physician or duet")),
                    })
                }
                "--backup" => {
                    result.backup =
                        Some(args.next().ok_or_else(|| refuse("missing backup"))?.clone())
                }
                "--target" => {
                    result.target =
                        Some(args.next().ok_or_else(|| refuse("missing target"))?.clone())
                }
                "--receipt" => {
                    result.receipt = Some(PathBuf::from(
                        args.next().ok_or_else(|| refuse("missing receipt path"))?,
                    ))
                }
                _ => return Err(refuse(USAGE)),
            }
        }
        Ok(result)
    }
}

fn run_cli(args: &[String]) -> Result<()> {
    if args == ["--help"] {
        println!("{USAGE}");
        return Ok(());
    }
    let options = Options::parse(args)?;
    let path = options.receipt.ok_or_else(|| refuse(USAGE))?;
    let mut cloud = Gcloud;
    let mut store = ReceiptStore::new(path);
    if options.cleanup {
        if !options.delete
            || options.execute
            || options.authorize
            || options.product.is_some()
            || options.backup.is_some()
            || options.target.is_some()
        {
            return Err(refuse(
                "cleanup accepts only --receipt and --authorize-staging-delete",
            ));
        }
        return cleanup(&mut cloud, &mut store);
    }
    if options.delete || options.execute != options.authorize {
        return Err(refuse(
            "execution needs both --execute and --authorize-staging-restore",
        ));
    }
    let request = Request {
        product: options.product.ok_or_else(|| refuse(USAGE))?,
        backup: options.backup.ok_or_else(|| refuse(USAGE))?,
        target: options.target.ok_or_else(|| refuse(USAGE))?,
    };
    restore(&mut cloud, &mut store, request, options.execute)
}

fn epoch() -> Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .map_err(Error::from)
}

fn source_instance(instances: &[Instance], product: Product) -> Result<&Instance> {
    let source = instances
        .iter()
        .find(|i| i.name == product.source())
        .ok_or_else(|| refuse("canonical staging source does not exist"))?;
    if source.project != PROJECT
        || source.state != "RUNNABLE"
        || source.database_version != "POSTGRES_17"
        || source.instance_type != "CLOUD_SQL_INSTANCE"
        || !source.private_only()
        || !source.staging_network()
    {
        return Err(refuse(
            "source is not the expected private staging PostgreSQL instance",
        ));
    }
    Ok(source)
}

fn preflight(cloud: &mut impl Cloud, request: &Request) -> Result<Receipt> {
    let (target, backup_id) = request.validate()?;
    let instances = cloud.instances()?;
    let source = source_instance(&instances, request.product)?;
    if instances
        .iter()
        .any(|instance| instance.name == request.target)
    {
        return Err(refuse(
            "target already exists: never overwrite or trust a name prefix/label as ownership",
        ));
    }
    let backup = cloud.backup(request.product, &backup_id)?;
    if backup.id != backup_id
        || backup.instance != request.product.source()
        || backup.status != "SUCCESSFUL"
    {
        return Err(refuse(
            "backup identity/status does not match the canonical product source",
        ));
    }
    let backup_epoch = chrono::DateTime::parse_from_rfc3339(&backup.end_time)?.timestamp();
    let now = epoch()?;
    let backup_epoch = u64::try_from(backup_epoch)?;
    let age = now
        .checked_sub(backup_epoch)
        .ok_or_else(|| refuse("backup timestamp is in the future"))?;
    // Cloud SQL refuses to restore into a smaller disk than the source's.
    let storage_size_gb = source
        .settings
        .data_disk_size_gb
        .parse()
        .ok()
        .filter(|size| STORAGE_GB.contains(size))
        .ok_or_else(|| {
            refuse("source disk size is unknown or outside the drill's 10-50 GB bound")
        })?;
    Ok(Receipt::new(
        request.product,
        target,
        backup,
        source.region.clone(),
        storage_size_gb,
        now,
        age,
    ))
}

fn restore(
    cloud: &mut impl Cloud,
    store: &mut impl receipt::Persistence,
    request: Request,
    execute: bool,
) -> Result<()> {
    let mut receipt = preflight(cloud, &request)?;
    store.create(&receipt)?;
    if !execute {
        println!("Preflight passed; no cloud writes. Product validation remains pending.");
        return Ok(());
    }
    let Err(error) = execute_restore(cloud, store, &mut receipt) else {
        return Ok(());
    };
    if matches!(error, Error::Unrecorded { .. }) {
        return Err(error);
    }
    receipt.failure = Some(error.receipt_code());
    // Persistence failure is an error too; never continue cloud writes after it,
    // and keep the operation error that the receipt failed to record.
    save_settled(store, &receipt, Err(error))
}

/// Save a receipt that records an already-settled result. A save failure never
/// replaces the operation error it was meant to record.
fn save_settled(
    store: &mut impl receipt::Persistence,
    receipt: &Receipt,
    result: Result<()>,
) -> Result<()> {
    match (result, store.save(receipt)) {
        (Ok(()), saved) => saved,
        (Err(primary), Ok(())) => Err(primary),
        (Err(primary), Err(persistence)) => Err(Error::Unrecorded {
            primary: Box::new(primary),
            persistence: Box::new(persistence),
        }),
    }
}

fn execute_restore(
    cloud: &mut impl Cloud,
    store: &mut impl receipt::Persistence,
    receipt: &mut Receipt,
) -> Result<()> {
    // Repeat the preflight immediately before the first mutation; create is exclusive in Cloud SQL.
    let instances = cloud.instances()?;
    source_instance(&instances, receipt.product)?;
    if instances.iter().any(|i| i.name == receipt.target.as_str()) {
        return Err(refuse("target appeared after preflight"));
    }
    receipt.phase = Phase::CreateRequested;
    receipt.cleanup = Cleanup::RequiredOrCreationUncertain;
    store.save(receipt)?;
    let operation = cloud.create(receipt)?;
    check_operation_identity(&operation, receipt, OperationKind::Create)?;
    receipt.create_operation = Some(operation.name.clone());
    let checked = check_operation(&operation, receipt, OperationKind::Create);
    save_settled(store, receipt, checked)?;
    wait(cloud, &operation.name, receipt, OperationKind::Create)?;
    let target = cloud.instance(&receipt.target)?;
    validate_target(&target, receipt, false)?;
    receipt.target_created_at = Some(target.create_time.clone());
    receipt.phase = Phase::Created;
    store.save(receipt)?;
    // No service, worker, queue, message integration or IAM binding is provisioned.
    let target = cloud.instance(&receipt.target)?;
    validate_target(&target, receipt, true)?;
    receipt.phase = Phase::RestoreRequested;
    receipt.restore_started_at = Some(epoch()?);
    store.save(receipt)?;
    let start = Instant::now();
    let operation = cloud.restore(receipt)?;
    check_operation_identity(&operation, receipt, OperationKind::RestoreVolume)?;
    receipt.restore_operation = Some(operation.name.clone());
    let checked = check_operation(&operation, receipt, OperationKind::RestoreVolume);
    save_settled(store, receipt, checked)?;
    wait(
        cloud,
        &operation.name,
        receipt,
        OperationKind::RestoreVolume,
    )?;
    let target = cloud.instance(&receipt.target)?;
    validate_target(&target, receipt, true)?;
    receipt.restore_elapsed_seconds = Some(start.elapsed().as_secs());
    receipt.recovery_age_seconds = epoch()?
        .checked_sub(receipt.backup_epoch()?)
        .ok_or_else(|| refuse("clock moved before backup"))?;
    receipt.phase = Phase::RestoredUnqualified;
    store.save(receipt)?;
    println!(
        "Cloud restore completed. Recovery remains UNQUALIFIED; product validation and explicit cleanup required."
    );
    Ok(())
}

fn validate_target(instance: &Instance, receipt: &Receipt, require_identity: bool) -> Result<()> {
    receipt.validate()?;
    if instance.name != receipt.target.as_str()
        || instance.project != PROJECT
        || instance.region != receipt.region
        || instance.state != "RUNNABLE"
        || instance.database_version != "POSTGRES_17"
        || instance.instance_type != "CLOUD_SQL_INSTANCE"
        || !instance.private_only()
        || !instance.staging_network()
        || instance.settings.ip_configuration.ssl_mode != "ENCRYPTED_ONLY"
        || !instance.settings.storage_auto_resize
        || instance.settings.storage_auto_resize_limit != "50"
        || instance.settings.deletion_protection_enabled
        || instance.master_instance_name.is_some()
        || !instance.replica_names.is_empty()
        || instance
            .settings
            .user_labels
            .get("coda-purpose")
            .map(String::as_str)
            != Some("restore-drill")
        || instance
            .settings
            .user_labels
            .get("coda-product")
            .map(String::as_str)
            != Some(receipt.product.name())
        || instance
            .settings
            .user_labels
            .get("coda-run")
            .map(String::as_str)
            != Some(receipt.run_id.as_str())
        || instance.create_time.is_empty()
    {
        return Err(refuse(
            "target is not the isolated private disposable instance created by this drill",
        ));
    }
    if require_identity
        && receipt.target_created_at.as_deref() != Some(instance.create_time.as_str())
    {
        return Err(refuse(
            "target creation identity changed or is missing; manual investigation required",
        ));
    }
    Ok(())
}

fn check_operation_identity(
    operation: &Operation,
    receipt: &Receipt,
    expected: OperationKind,
) -> Result<()> {
    if operation.target_project != PROJECT
        || operation.target_id != receipt.target.as_str()
        || operation.operation_type != expected
    {
        return Err(refuse("operation identity/type does not match this target"));
    }
    Ok(())
}

fn check_operation(
    operation: &Operation,
    receipt: &Receipt,
    expected: OperationKind,
) -> Result<()> {
    check_operation_identity(operation, receipt, expected)?;
    if let Some(details) = &operation.error {
        return Err(Error::Operation {
            details: details.clone(),
        });
    }
    operation.status?;
    Ok(())
}

fn wait(
    cloud: &mut impl Cloud,
    id: &SqlOperationId,
    receipt: &Receipt,
    expected: OperationKind,
) -> Result<()> {
    wait_bounded(cloud, id, receipt, expected, WAIT_LIMIT)
}

fn wait_bounded(
    cloud: &mut impl Cloud,
    id: &SqlOperationId,
    receipt: &Receipt,
    expected: OperationKind,
    limit: Duration,
) -> Result<()> {
    let started = Instant::now();
    loop {
        let operation = cloud.operation(id)?;
        check_operation(&operation, receipt, expected)?;
        if &operation.name != id {
            return Err(refuse("operation ID changed"));
        }
        match operation.status? {
            OperationStatus::Done => return Ok(()),
            OperationStatus::Pending | OperationStatus::Running => {}
        }
        if started.elapsed() >= limit {
            return Err(Error::Timeout);
        }
        cloud.pause(POLL_INTERVAL);
    }
}

fn cleanup(cloud: &mut impl Cloud, store: &mut impl receipt::Persistence) -> Result<()> {
    let mut receipt = store.load()?;
    receipt.validate()?;
    if receipt.target_created_at.is_none() || receipt.create_operation.is_none() {
        return Err(refuse(
            "no persisted creation identity; do not delete by name, investigate manually",
        ));
    }
    let target = cloud.instance(&receipt.target)?;
    validate_target(&target, &receipt, true)?;
    let (operation_id, kind) = if let Some(id) = receipt.restore_operation.as_ref() {
        (id, OperationKind::RestoreVolume)
    } else if receipt.phase == Phase::RestoreRequested {
        return Err(refuse(
            "restore submission outcome unknown; inspect cloud operations before cleanup",
        ));
    } else {
        (
            receipt
                .create_operation
                .as_ref()
                .ok_or_else(|| refuse("missing create operation"))?,
            OperationKind::Create,
        )
    };
    let prior_operation = cloud.operation(operation_id)?;
    // A failed operation is terminal too; it still needs explicit target cleanup.
    let mut identity_only = prior_operation.clone();
    identity_only.error = None;
    check_operation(&identity_only, &receipt, kind)?;
    if &prior_operation.name != operation_id || prior_operation.status? != OperationStatus::Done {
        return Err(refuse("target operation is not terminal; cleanup refused"));
    }
    receipt.cleanup = Cleanup::DeleteRequested;
    store.save(&receipt)?;
    let operation = cloud.delete(&receipt.target)?;
    check_operation_identity(&operation, &receipt, OperationKind::Delete)?;
    receipt.delete_operation = Some(operation.name.clone());
    let checked = check_operation(&operation, &receipt, OperationKind::Delete);
    save_settled(store, &receipt, checked)?;
    wait(cloud, &operation.name, &receipt, OperationKind::Delete)?;
    if cloud
        .instances()?
        .iter()
        .any(|instance| instance.name == receipt.target.as_str())
    {
        return Err(refuse("delete completed but target still present"));
    }
    receipt.cleanup = Cleanup::Deleted;
    store.save(&receipt)?;
    println!("Disposable target deletion verified; source was not modified.");
    Ok(())
}
