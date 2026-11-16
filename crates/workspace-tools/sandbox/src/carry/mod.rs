//! `coda-sandbox carry`: record host worktrees of the checkout as refs and
//! bundle them for the sandbox, publishing nothing; and `carry import`,
//! which recreates them in the container.
//!
//! - `refs/carry/<slug>/head`: the worktree's HEAD
//! - `refs/carry/<slug>/dirty`: HEAD plus uncommitted and untracked changes,
//!   built with a temporary index; the worktree, its index and its stash are
//!   untouched
//! - `carry.bundle`: the `refs/carry` refs, less what the host's
//!   `origin/main` holds; absent when that leaves nothing
//! - `ignored.txt`: ignored files outside `target/`, per worktree
//! - `summary.txt`: one line per worktree, with its commits and branch
//!
//! It runs as the checkout's owner, never as root, so no hook or config in
//! the trees runs with privilege, and writes into the carry directory that
//! user owns.

mod collect;
pub mod import;
pub mod naming;
pub mod summary;

use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};

use crate::config::Carry;
use crate::error::Error;

pub use collect::{Select, collect};
pub use summary::{Carried, Dirty};

/// The files carry writes, in the order it installs them.
pub const OUTPUTS: [&str; 3] = ["carry.bundle", "ignored.txt", "summary.txt"];

/// Collects in a scratch directory, then installs each output into
/// `carry.out` by rename, so a reader sees the old file or the new one,
/// never half of one. Refs are replaced wholesale on every run, so an
/// interrupted run is finished by the next. The summary goes last, so an
/// importer never reads a summary newer than its bundle.
pub fn carry(config: &Carry, select: &Select) -> Result<Vec<Carried>, Error> {
    if rustix::process::geteuid().is_root() {
        return Err(Error::CarryAsRoot);
    }
    std::fs::create_dir_all(&config.out).map_err(Error::fs("create", &config.out))?;
    let staging = tempfile::Builder::new()
        .prefix(".carry.")
        .tempdir_in(&config.out)
        .map_err(Error::fs("create a staging directory in", &config.out))?;
    let carried = collect(&config.repo, staging.path(), select)?;
    for name in OUTPUTS {
        let from = staging.path().join(name);
        let to: PathBuf = config.out.join(name);
        if !from.exists() {
            // No bundle: every carried commit is on origin/main.
            match std::fs::remove_file(&to) {
                Ok(()) => {}
                Err(source) if source.kind() == std::io::ErrorKind::NotFound => {}
                Err(source) => return Err(Error::fs("remove", &to)(source)),
            }
            continue;
        }
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
        "coda-sandbox: {} worktrees in {} ({size} bytes); \
         `coda-sandbox carry import` in the container makes them\n",
        carried.len(),
        bundle.display()
    ));
    text
}
