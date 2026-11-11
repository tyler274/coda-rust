//! `coda-sandbox worktree`: worktrees whose build output costs nothing until
//! it diverges. Each worktree under `<main>/.worktrees/<slug>` keeps only its
//! source; its `target/` links to `<build root>/wt/<slug>`, a reflink clone of
//! a seed under `<build root>/seed/` that holds the third-party build output.
//!
//! The build root must support reflinks (ZFS 2.2 or later, btrfs, XFS); a
//! clone never falls back to a full copy. `seed` runs `dev/session-seed.sh`,
//! keyed by Cargo.lock, flake.lock and rust-toolchain.toml.

pub mod clone;
pub mod gc;
pub mod new;
pub mod prune;
pub mod seed;
pub mod status;
pub mod walk;

use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};

use crate::config::Build;
use crate::error::Error;
use crate::git::{common_dir, common_dir_if_any};
use crate::ids::Slug;

/// Whether two paths name the same file. A path that is gone (a stale
/// worktree record) resolves through its parent directory, so a symlink above
/// it still compares equal.
pub fn same_path(a: &Path, b: &Path) -> bool {
    resolve(a) == resolve(b)
}

fn resolve(path: &Path) -> PathBuf {
    path.canonicalize()
        .ok()
        .or_else(|| {
            let parent = path.parent()?.canonicalize().ok()?;
            Some(parent.join(path.file_name()?))
        })
        .unwrap_or_else(|| path.to_owned())
}

/// Written into a seed and each clone of it, holding the seed's key.
pub const MARKER: &str = ".coda-seed";

/// Where one checkout's worktrees and build output live.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    /// The main checkout, which owns the repository.
    pub main: PathBuf,
    /// `<main>/.worktrees`, ignored by git.
    pub trees: PathBuf,
    pub build: PathBuf,
    pub seeds: PathBuf,
    pub targets: PathBuf,
    pub keep_seeds: NonZeroUsize,
}

impl Layout {
    pub fn new(main: PathBuf, build: &Build) -> Self {
        Self {
            trees: main.join(".worktrees"),
            main,
            seeds: build.root.join("seed"),
            targets: build.root.join("wt"),
            build: build.root.clone(),
            keep_seeds: build.keep_seeds,
        }
    }

    /// The configured checkout, whose worktrees the build root's targets
    /// belong to. Run from inside a repository, that repository must be the
    /// configured one (the same common git directory, so any of its worktrees
    /// will do); any other repository, a submodule included, is refused
    /// before a command can delete targets it does not own. Outside a
    /// repository, as a system unit runs, the configured checkout is used.
    /// Either way the configured checkout must be a repository, or every
    /// target would look orphaned to `gc`.
    pub fn discover(cwd: &Path, repo: &Path, build: &Build) -> Result<Self, Error> {
        let configured = common_dir(repo)?;
        if let Some(found) = common_dir_if_any(cwd)?
            && !same_path(&found, &configured)
        {
            return Err(Error::ForeignRepo {
                found: found.parent().unwrap_or(&found).to_owned(),
                configured: repo.to_owned(),
            });
        }
        Ok(Self::new(repo.to_owned(), build))
    }

    pub fn tree(&self, slug: &Slug) -> PathBuf {
        self.trees.join(slug.as_str())
    }

    pub fn target(&self, slug: &Slug) -> PathBuf {
        self.targets.join(slug.as_str())
    }

    /// The detached worktree a seed is built in.
    pub fn seed_tree(&self) -> PathBuf {
        self.trees.join(".seed")
    }

    /// Stops unless the build root exists and this user may write to it, then
    /// makes its `seed/` and `wt/` directories.
    pub fn need_build_root(&self) -> Result<(), Error> {
        if !self.build.is_dir() {
            return Err(Error::BuildRootMissing {
                path: self.build.clone(),
            });
        }
        rustix::fs::access(&self.build, rustix::fs::Access::WRITE_OK).map_err(|errno| {
            Error::BuildRootNotWritable {
                path: self.build.clone(),
                source: errno.into(),
            }
        })?;
        for dir in [&self.seeds, &self.targets] {
            std::fs::create_dir_all(dir).map_err(|source| Error::Fs {
                op: "create",
                path: dir.clone(),
                source,
            })?;
        }
        Ok(())
    }

    /// The slugs of the worktrees under `.worktrees`, skipping hidden entries
    /// such as the seed's own worktree.
    pub fn tree_slugs(&self) -> Result<Vec<Slug>, Error> {
        walk::visible_dirs(&self.trees)
    }

    /// The slugs of the targets under the build root, skipping hidden
    /// entries such as preserved proof awaiting its clone.
    pub fn target_slugs(&self) -> Result<Vec<Slug>, Error> {
        walk::visible_dirs(&self.targets)
    }
}

/// The current seed directory, `None` when `seed/current` names none.
pub fn current_seed(layout: &Layout) -> Result<Option<PathBuf>, Error> {
    let link = layout.seeds.join("current");
    match std::fs::read_link(&link) {
        Ok(target) => {
            let seed = layout.seeds.join(target);
            Ok(seed.is_dir().then_some(seed))
        }
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(Error::Fs {
            op: "read the link",
            path: link,
            source,
        }),
    }
}
