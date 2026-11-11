//! `coda-sandbox carry`: record every host worktree of the checkout as refs
//! and bundle them for the sandbox, publishing nothing.
//!
//! - `refs/carry/<slug>/head`: the worktree's HEAD
//! - `refs/carry/<slug>/dirty`: HEAD plus uncommitted and untracked changes,
//!   built with a temporary index; the worktree, its index and its stash are
//!   untouched
//! - `carry.bundle`: all `refs/carry` refs
//! - `ignored.txt`: ignored files outside `target/`, per worktree
//! - `summary.txt`: one line per worktree
//!
//! It runs as the checkout's owner, never as root, so no hook or config in
//! the trees runs with privilege, and writes into the carry directory that
//! user owns.

mod collect;
pub mod naming;

use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};

use crate::config::Carry;
use crate::error::Error;

pub use collect::{Carried, collect};

/// The files carry writes, in the order it installs them.
pub const OUTPUTS: [&str; 3] = ["carry.bundle", "ignored.txt", "summary.txt"];

/// Collects in a scratch directory, then installs each output into
/// `carry.out` by rename, so a reader sees the old file or the new one,
/// never half of one. Refs are replaced wholesale on every run, so an
/// interrupted run is finished by the next.
pub fn carry(config: &Carry) -> Result<Vec<Carried>, Error> {
    if rustix::process::geteuid().is_root() {
        return Err(Error::CarryAsRoot);
    }
    std::fs::create_dir_all(&config.out).map_err(Error::fs("create", &config.out))?;
    let staging = tempfile::Builder::new()
        .prefix(".carry.")
        .tempdir_in(&config.out)
        .map_err(Error::fs("create a staging directory in", &config.out))?;
    let carried = collect(&config.repo, staging.path())?;
    for name in OUTPUTS {
        let from = staging.path().join(name);
        let to: PathBuf = config.out.join(name);
        std::fs::set_permissions(&from, std::fs::Permissions::from_mode(0o644))
            .map_err(Error::fs("set the mode of", &from))?;
        std::fs::rename(&from, &to).map_err(Error::fs("install", &to))?;
    }
    Ok(carried)
}

/// What `carry` prints: the summary, then a total line.
pub fn render(carried: &[Carried], out: &Path) -> String {
    let mut text: String = carried.iter().map(Carried::summary_line).collect();
    let bundle = out.join("carry.bundle");
    let size = std::fs::metadata(&bundle).map_or(0, |meta| meta.len());
    text.push_str(&format!(
        "coda-sandbox: {} worktrees in {} ({size} bytes)\n",
        carried.len(),
        bundle.display()
    ));
    text
}
