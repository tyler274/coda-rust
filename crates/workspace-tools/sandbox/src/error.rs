use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitStatus;

use crate::ids::{ParseIdError, RequestId};

/// Everything that can go wrong in `coda-sandbox`, each keeping its cause.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no config at {path}")]
    ConfigMissing {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("cannot read config {path}")]
    ConfigRead {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("config {path} is not valid")]
    ConfigParse {
        path: PathBuf,
        #[source]
        source: Box<toml::de::Error>,
    },
    #[error("cannot inspect {path}")]
    Inspect {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("HOME is not set, so editor settings cannot be found")]
    NoHome,
    #[error("cannot run {program}")]
    Spawn {
        program: &'static str,
        #[source]
        source: io::Error,
    },
    #[error("cannot write to the standard input of {program}")]
    Stdin {
        program: &'static str,
        #[source]
        source: io::Error,
    },
    #[error("cannot wait for {program}")]
    Wait {
        program: &'static str,
        #[source]
        source: io::Error,
    },
    #[error("{program} {status}: {stderr}")]
    CommandFailed {
        program: &'static str,
        status: ExitStatus,
        stderr: String,
    },
    #[error("unexpected {what} from {source_name}: {found:?}")]
    UnexpectedOutput {
        what: &'static str,
        source_name: String,
        found: String,
    },
    #[error("{path} is not valid JSON with comments")]
    SettingsParse {
        path: PathBuf,
        #[source]
        source: json5::Error,
    },
    #[error("cannot encode status as JSON")]
    Json(#[source] serde_json::Error),
    #[error("cannot write to standard output")]
    Output(#[source] io::Error),
    #[error("cannot {op} {path}")]
    Fs {
        op: &'static str,
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("git {args} in {dir} {status}: {stderr}")]
    Git {
        args: String,
        dir: PathBuf,
        status: ExitStatus,
        stderr: String,
    },
    #[error(transparent)]
    Id(#[from] ParseIdError),
    #[error(
        "build root {path} does not exist; set [build] root to a directory on a filesystem \
         with reflinks (ZFS, btrfs, XFS)"
    )]
    BuildRootMissing { path: PathBuf },
    #[error(
        "build root {path} is not writable by this user; on Cyrene it belongs to the coda-agent sandbox"
    )]
    BuildRootNotWritable {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error(
        "reflink clone of {seed} into {dest} failed; the build root must support reflinks: {stderr}"
    )]
    ReflinkFailed {
        seed: PathBuf,
        dest: PathBuf,
        stderr: String,
    },
    #[error(
        "{kept} holds proof an interrupted reset kept, and {proof} holds newer proof; merge them by hand"
    )]
    ProofConflict { kept: PathBuf, proof: PathBuf },
    #[error("{path} exists and is not the worktree of branch {branch}")]
    WorktreeConflict { path: PathBuf, branch: String },
    #[error("no worktree {path}")]
    NoWorktree { path: PathBuf },
    #[error("{path} is not a worktree git knows; remove it by hand if it is not wanted")]
    UnknownWorktree { path: PathBuf },
    #[error(
        "{found} is not the checkout {configured} that [container] repo names; the build root \
         holds that checkout's targets, so run worktree commands from it or one of its worktrees"
    )]
    ForeignRepo { found: PathBuf, configured: PathBuf },
    #[error("{path} is a real directory; move it away before linking the shared build root")]
    RealTarget { path: PathBuf },
    #[error("the seed build in {tree} {status}")]
    SeedBuildFailed { tree: PathBuf, status: ExitStatus },
    #[error("cannot read the answer to the prompt")]
    Prompt(#[source] io::Error),
    #[error("standard input is not a terminal; rerun with --yes to remove them")]
    NeedsYes,
    #[error("no CI spool at {path}; run this where the CI spool is mounted")]
    NoSpool { path: PathBuf },
    #[error("{rev} is not a commit in {repo}")]
    NotACommit { rev: String, repo: PathBuf },
    #[error("no result for CI request {id} after {seconds}s")]
    CiTimeout { id: RequestId, seconds: u64 },
    #[error("this machine's config has no [carry] section, so it has no worktrees to carry")]
    NoCarry,
    #[error("carry runs git in your worktrees, so it runs as their owner, never as root")]
    CarryAsRoot,
}

impl Error {
    /// Wraps a filesystem error with the operation and the path it was on,
    /// for `map_err`.
    pub fn fs(op: &'static str, path: &Path) -> impl FnOnce(io::Error) -> Self + use<> {
        let path = path.to_owned();
        move |source| Self::Fs { op, path, source }
    }

    /// The error and each of its causes, joined for one line of output.
    pub fn chain(&self) -> String {
        let mut text = self.to_string();
        let mut cause = std::error::Error::source(self);
        while let Some(current) = cause {
            text.push_str(": ");
            text.push_str(&current.to_string());
            cause = current.source();
        }
        text
    }
}
