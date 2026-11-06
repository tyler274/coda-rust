//! Private, owner-only artifact directory and its plan identity manifest.
use super::tools::{Exit, Run, Stage};
use super::{Result, refuse};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, DirBuilder, File, OpenOptions},
    io::Write,
    os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
};

pub(super) const PLAN_FILE: &str = "staging.tfplan";
pub(super) const REVIEW_FILE: &str = "review.txt";
pub(super) const DIAGNOSTICS_FILE: &str = "diagnostics.log";
pub(super) const MANIFEST_FILE: &str = "manifest.json";
pub(super) const BACKEND_FILE: &str = "backend/staging.gcs.tfbackend";
pub(super) const VAR_FILE: &str = "env/staging.tfvars";
const BACKEND_BUCKET: &str = "coda-rust-staging-tofu-state-us-east1";
const BACKEND_PREFIX: &str = "rust-environments/stg";
const SENSITIVE: &str =
    "SENSITIVE: private OpenTofu diagnostics. Never publish or attach as proof.\n";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub(super) struct Sha256Hex(String);
impl Sha256Hex {
    pub fn of(bytes: &[u8]) -> Self {
        Self(
            Sha256::digest(bytes)
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect(),
        )
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct Backend {
    pub bucket: String,
    pub prefix: String,
}
impl Backend {
    /// The fixed staging state; any other backend file is refused.
    pub fn read(root: &Path) -> Result<Self> {
        let text = fs::read_to_string(root.join(BACKEND_FILE))?;
        let value = |key: &str| {
            text.lines().find_map(|line| {
                let (name, value) = line.split_once('=')?;
                (name.trim() == key).then(|| value.trim().trim_matches('"').to_owned())
            })
        };
        let backend = Self {
            bucket: value("bucket").unwrap_or_default(),
            prefix: value("prefix").unwrap_or_default(),
        };
        if backend.bucket != BACKEND_BUCKET
            || backend.prefix != BACKEND_PREFIX
            || text.lines().filter(|line| line.contains('=')).count() != 2
        {
            return Err(refuse(
                "backend is not the fixed rust-environments staging state",
            ));
        }
        Ok(backend)
    }
}

/// Digest of every reviewed input in the root: `*.tf`, the provider lock,
/// the backend and the staging tfvars. Ordered by relative path.
pub(super) fn config_digest(root: &Path) -> Result<Sha256Hex> {
    let mut files: Vec<String> = fs::read_dir(root)?
        .map(|entry| entry.map(|entry| entry.file_name().to_string_lossy().into_owned()))
        .collect::<std::io::Result<_>>()?;
    files.retain(|name| name.ends_with(".tf") || name == ".terraform.lock.hcl");
    files.extend([BACKEND_FILE.to_owned(), VAR_FILE.to_owned()]);
    files.sort();
    let mut hasher = Sha256::new();
    for name in files {
        let bytes = fs::read(root.join(&name))?;
        hasher.update(name.as_bytes());
        hasher.update([0]);
        hasher.update((bytes.len() as u64).to_be_bytes());
        hasher.update(&bytes);
    }
    Ok(Sha256Hex::of(&hasher.finalize()))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum PlanOutcome {
    NoChanges,
    Changes,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum ApplyState {
    /// Recorded before `tofu apply` starts; seen alone, the outcome is unknown.
    Started,
    Succeeded,
    Failed,
    TimedOut,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(super) struct StageRecord {
    pub stage: Stage,
    pub exit: Exit,
    pub elapsed_ms: u64,
}
impl StageRecord {
    pub fn of(stage: Stage, run: &Run) -> Self {
        Self {
            stage,
            exit: run.exit,
            elapsed_ms: u64::try_from(run.elapsed.as_millis()).unwrap_or(u64::MAX),
        }
    }
}

/// Safe to read aloud: identities, digests, stage exits, no tool output.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct Manifest {
    pub schema_version: u32,
    pub backend: Backend,
    pub config_digest: Sha256Hex,
    pub plan_digest: Sha256Hex,
    pub outcome: PlanOutcome,
    pub planned_at_unix_seconds: u64,
    pub stages: Vec<StageRecord>,
    pub apply: Option<ApplyState>,
}
pub(super) const SCHEMA_VERSION: u32 = 1;

/// An owner-only directory this command created or validated.
#[derive(Debug)]
pub(super) struct PlanDir {
    path: PathBuf,
}
impl PlanDir {
    /// Create a fresh private directory outside the workspace checkout.
    pub fn create(path: &Path, workspace: &Path) -> Result<Self> {
        if path.as_os_str().is_empty() || fs::symlink_metadata(path).is_ok() {
            return Err(refuse(
                "output directory must be a new path; never overwrite plans",
            ));
        }
        let parent = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        let parent = parent.canonicalize()?;
        if parent.starts_with(workspace.canonicalize()?) {
            return Err(refuse(
                "output directory is inside the repository; plans contain credentials",
            ));
        }
        DirBuilder::new().mode(0o700).create(path)?;
        let dir = Self {
            path: parent.join(
                path.file_name()
                    .ok_or_else(|| refuse("output directory has no name"))?,
            ),
        };
        dir.check()?;
        Ok(dir)
    }
    pub fn open(path: &Path) -> Result<Self> {
        let dir = Self {
            path: path.canonicalize()?,
        };
        dir.check()?;
        Ok(dir)
    }
    fn check(&self) -> Result<()> {
        let meta = fs::symlink_metadata(&self.path)?;
        if !meta.is_dir() || meta.mode() & 0o077 != 0 || meta.uid() != current_uid()? {
            return Err(refuse(
                "artifact directory must be an owner-only (0700) directory you own",
            ));
        }
        Ok(())
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn file(&self, name: &str) -> PathBuf {
        self.path.join(name)
    }
    /// A private regular file in this directory, never a symlink or shared file.
    pub fn read_private(&self, name: &str) -> Result<Vec<u8>> {
        let path = self.file(name);
        let meta = fs::symlink_metadata(&path)?;
        if !meta.is_file() || meta.permissions().mode() & 0o077 != 0 || meta.uid() != current_uid()?
        {
            return Err(refuse(format!(
                "{name} must be a private regular file you own"
            )));
        }
        Ok(fs::read(path)?)
    }
    /// OpenTofu creates its plan under the process umask; narrow it to the owner.
    pub fn restrict(&self, name: &str) -> Result<()> {
        let path = self.file(name);
        if !fs::symlink_metadata(&path)?.is_file() {
            return Err(refuse(format!("{name} must be a regular file")));
        }
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
        Ok(())
    }
    pub fn write_new(&self, name: &str, bytes: &[u8]) -> Result<PathBuf> {
        let path = self.file(name);
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&path)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        Ok(path)
    }
    /// Append one stage's output to the private diagnostics log.
    /// `include_stdout` is false for credential lookups, whose stdout is the secret.
    pub fn append_log(&self, stage: Stage, run: &Run, include_stdout: bool) -> Result<PathBuf> {
        let path = self.file(DIAGNOSTICS_FILE);
        let fresh = match fs::symlink_metadata(&path) {
            Ok(meta) if meta.is_file() => false,
            Ok(_) => return Err(refuse("diagnostics log must be a regular file")),
            Err(_) => true,
        };
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .mode(0o600)
            .open(&path)?;
        let mut text = Vec::new();
        if fresh {
            text.extend_from_slice(SENSITIVE.as_bytes());
        }
        text.extend_from_slice(format!("== {} exit={:?}\n", stage.name(), run.exit).as_bytes());
        if include_stdout {
            text.extend_from_slice(b"-- stdout\n");
            text.extend_from_slice(&run.stdout);
        }
        text.extend_from_slice(b"\n-- stderr\n");
        text.extend_from_slice(&run.stderr);
        text.push(b'\n');
        file.write_all(&text)?;
        file.sync_all()?;
        Ok(path)
    }
    pub fn load_manifest(&self) -> Result<Manifest> {
        let manifest: Manifest = serde_json::from_slice(&self.read_private(MANIFEST_FILE)?)?;
        if manifest.schema_version != SCHEMA_VERSION {
            return Err(refuse("unsupported manifest schema"));
        }
        Ok(manifest)
    }
    /// Atomic replace: a crash leaves either the old or the new manifest.
    pub fn save_manifest(&self, manifest: &Manifest) -> Result<()> {
        let temporary = format!("{MANIFEST_FILE}.tmp");
        let _ = fs::remove_file(self.file(&temporary));
        self.write_new(&temporary, &serde_json::to_vec_pretty(manifest)?)?;
        fs::rename(self.file(&temporary), self.file(MANIFEST_FILE))?;
        File::open(&self.path)?.sync_all()?;
        Ok(())
    }
}

fn current_uid() -> Result<u32> {
    Ok(fs::metadata("/proc/self")?.uid())
}

impl From<Result<PathBuf>> for super::Diagnostics {
    fn from(result: Result<PathBuf>) -> Self {
        match result {
            Ok(path) => Self::Retained(path),
            Err(error) => Self::Unavailable(Box::new(error)),
        }
    }
}
