use super::{PROJECT, Product, RestoreTarget, Result, SqlOperationId, cloud::Backup, refuse};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    os::unix::fs::OpenOptionsExt,
    path::PathBuf,
};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(super) enum Phase {
    Preflight,
    CreateRequested,
    Created,
    RestoreRequested,
    RestoredUnqualified,
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(super) enum Cleanup {
    NotCreated,
    RequiredOrCreationUncertain,
    DeleteRequested,
    Deleted,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Receipt {
    pub schema_version: u32,
    pub project: String,
    pub product: Product,
    pub source: String,
    pub target: RestoreTarget,
    pub region: String,
    pub run_id: String,
    pub backup: Backup,
    /// Initial target disk, copied from the source so the backup fits.
    pub storage_size_gb: u64,
    pub preflight_at_unix_seconds: u64,
    pub phase: Phase,
    pub create_operation: Option<SqlOperationId>,
    pub target_created_at: Option<String>,
    pub restore_operation: Option<SqlOperationId>,
    pub restore_started_at: Option<u64>,
    pub restore_elapsed_seconds: Option<u64>,
    pub recovery_age_seconds: u64,
    pub cleanup: Cleanup,
    pub delete_operation: Option<SqlOperationId>,
    pub failure: Option<FailureCode>,
    pub qualification: Qualification,
    pub pending_validation: Vec<String>,
}
/// Closed public diagnostic vocabulary; never serialize process stderr or provider error fields.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(super) enum FailureCode {
    ValidationRefused,
    Io,
    InvalidJson,
    CloudCommand,
    CloudOperation,
    Timeout,
    InvalidTime,
}

/// No successful qualification variant exists here. Product owners, not cloud operations,
/// establish recovery; an operator must retain their separate evidence against this target.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Qualification {
    PendingProductValidation,
}
impl Receipt {
    pub fn new(
        product: Product,
        target: RestoreTarget,
        backup: Backup,
        region: String,
        storage_size_gb: u64,
        now: u64,
        age: u64,
    ) -> Self {
        Self {
            schema_version: 2,
            storage_size_gb,
            project: PROJECT.into(),
            source: product.source(),
            product,
            target,
            region,
            run_id: format!("{}-{}", now, std::process::id()),
            backup,
            preflight_at_unix_seconds: now,
            phase: Phase::Preflight,
            create_operation: None,
            target_created_at: None,
            restore_operation: None,
            restore_started_at: None,
            restore_elapsed_seconds: None,
            recovery_age_seconds: age,
            cleanup: Cleanup::NotCreated,
            delete_operation: None,
            failure: None,
            qualification: Qualification::PendingProductValidation,
            pending_validation: match product {
                Product::Physician => vec![
                    "physician-auth/front-door migration and schema readiness".into(),
                    "physician product-owned representative readback and runtime permission checks".into(),
                ],
                Product::Duet => vec![
                    "duet-migrate and AccountDatabase schema/runtime permission checks".into(),
                    "duet product-owned representative readback".into(),
                    "independent journal provenance and complete inventory, including post-backup restrictions".into(),
                    "duet-privacy-replay with no receipt ID arguments (full inventory)".into(),
                    "synthetic post-backup deletion remains unreadable after recovery".into(),
                ],
            },
        }
    }
    pub fn backup_epoch(&self) -> Result<u64> {
        u64::try_from(chrono::DateTime::parse_from_rfc3339(&self.backup.end_time)?.timestamp())
            .map_err(super::Error::from)
    }
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 2
            || self.project != PROJECT
            || self.source != self.product.source()
            || self.target.product() != self.product
            || self.backup.instance != self.source
            || self.backup.status != "SUCCESSFUL"
            || !super::STORAGE_GB.contains(&self.storage_size_gb)
            || self.run_id.is_empty()
            || !self.run_id.bytes().all(|b| b.is_ascii_digit() || b == b'-')
            || self.region.is_empty()
            || !self
                .region
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        {
            return Err(refuse("receipt identity is not a staging restore drill"));
        }
        self.backup_epoch()?;
        if let Some(created) = &self.target_created_at {
            chrono::DateTime::parse_from_rfc3339(created)?;
        }
        Ok(())
    }
}

pub(super) trait Persistence {
    fn create(&mut self, receipt: &Receipt) -> Result<()>;
    fn save(&mut self, receipt: &Receipt) -> Result<()>;
    fn load(&mut self) -> Result<Receipt>;
}
/// Append-only, synced receipt snapshots preserve the intent before each cloud write.
/// Partial trailing records are rejected, not silently discarded during cleanup.
pub(super) struct ReceiptStore {
    path: PathBuf,
    file: Option<File>,
}
impl ReceiptStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path, file: None }
    }
}
impl Persistence for ReceiptStore {
    fn create(&mut self, receipt: &Receipt) -> Result<()> {
        let file = OpenOptions::new()
            .create_new(true)
            .append(true)
            .mode(0o600)
            .open(&self.path)?;
        self.file = Some(file);
        self.save(receipt)?;
        let parent = self
            .path
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| std::path::Path::new("."));
        File::open(parent)?.sync_all()?;
        Ok(())
    }
    fn save(&mut self, receipt: &Receipt) -> Result<()> {
        receipt.validate()?;
        let file = self
            .file
            .as_mut()
            .ok_or_else(|| refuse("receipt must be created or loaded before saving"))?;
        let mut bytes = serde_json::to_vec(receipt)?;
        bytes.push(b'\n');
        file.write_all(&bytes)?;
        file.sync_all()?;
        Ok(())
    }
    fn load(&mut self) -> Result<Receipt> {
        if !std::fs::symlink_metadata(&self.path)?.file_type().is_file() {
            return Err(refuse("receipt must be a regular file, not a symlink"));
        }
        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(&self.path)?;
        let mut last: Option<Receipt> = None;
        let mut reader = BufReader::new(&file);
        let mut line = String::new();
        while reader.read_line(&mut line)? != 0 {
            if !line.ends_with('\n') {
                return Err(refuse("incomplete persisted receipt; investigate manually"));
            }
            let next: Receipt = serde_json::from_str(&line)?;
            next.validate()?;
            if last.as_ref().is_some_and(|old| {
                old.target != next.target
                    || old.run_id != next.run_id
                    || old.source != next.source
                    || old.backup.id != next.backup.id
            }) {
                return Err(refuse("receipt identity changed between snapshots"));
            }
            last = Some(next);
            line.clear();
        }
        self.file = Some(file);
        last.ok_or_else(|| refuse("empty receipt"))
    }
}
