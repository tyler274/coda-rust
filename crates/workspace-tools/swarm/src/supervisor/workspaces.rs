//! Where a member's turns run. A member either inherits the caller's
//! directory or owns a worktree, which the supervisor removes on kill.

use std::path::PathBuf;
use std::process::Command;

/// The workspaces that own a worktree. `Share` is one worktree for a whole
/// `add` group, removed after its last member; `Branch` is one per member.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorktreeMode {
    Share,
    Branch,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum WorkspaceError {
    #[error("start {program}: {reason}")]
    Start { program: String, reason: String },
    #[error("{command} failed: {detail}")]
    Failed { command: String, detail: String },
    #[error("{command} printed no worktree path")]
    NoPath { command: String },
}

pub trait Workspaces: Send + Sync {
    /// Creates the worktree `slug` and returns its path.
    fn create(&self, slug: &str, mode: WorktreeMode) -> Result<PathBuf, WorkspaceError>;
    fn remove(&self, slug: &str) -> Result<(), WorkspaceError>;
}

/// Worktrees managed by `coda-sandbox worktree new|rm <slug>`. Both owned
/// modes create the same kind of worktree; they differ only in how many
/// members hold it.
#[derive(Debug, Clone)]
pub struct SandboxWorkspaces {
    pub program: PathBuf,
}

impl Default for SandboxWorkspaces {
    fn default() -> Self {
        Self {
            program: PathBuf::from("coda-sandbox"),
        }
    }
}

impl SandboxWorkspaces {
    fn run(&self, verb: &str, slug: &str) -> Result<String, WorkspaceError> {
        let command = format!("{} worktree {verb} {slug}", self.program.display());
        let output = Command::new(&self.program)
            .args(["worktree", verb, slug])
            .output()
            .map_err(|error| WorkspaceError::Start {
                program: self.program.display().to_string(),
                reason: error.to_string(),
            })?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(WorkspaceError::Failed {
                command,
                detail: format!("{}: {}", output.status, stderr.trim()),
            });
        }
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}

impl Workspaces for SandboxWorkspaces {
    /// The path is the last non-empty line `worktree new` prints.
    fn create(&self, slug: &str, _mode: WorktreeMode) -> Result<PathBuf, WorkspaceError> {
        let stdout = self.run("new", slug)?;
        stdout
            .lines()
            .map(str::trim)
            .rfind(|line| !line.is_empty())
            .map(PathBuf::from)
            .ok_or_else(|| WorkspaceError::NoPath {
                command: format!("{} worktree new {slug}", self.program.display()),
            })
    }

    fn remove(&self, slug: &str) -> Result<(), WorkspaceError> {
        self.run("rm", slug).map(drop)
    }
}
