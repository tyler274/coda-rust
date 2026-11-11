//! `worktree new <slug> [--from <ref>]`, `worktree reset <slug>` and
//! `worktree rm <slug>`.

use std::path::{Path, PathBuf};
use std::process::Command;

use super::clone::{clone_seed, link_target};
use super::walk::{create_dir, remove_tree};
use super::{Layout, same_path};
use crate::error::Error;
use crate::git::git;
use crate::ids::Slug;
use crate::porcelain::{self, Entry, EntryState, Head};

/// The ref a new branch starts from when `--from` is not given.
pub const DEFAULT_FROM: &str = "origin/main";

/// The branch ref a slug's worktree checks out.
pub fn branch_ref(slug: &Slug) -> String {
    format!("refs/heads/{slug}")
}

/// The repository's record of the worktree at `tree`, if it has one.
pub fn registered(layout: &Layout, tree: &Path) -> Result<Option<Entry>, Error> {
    let text = git(&layout.main)
        .args(["worktree", "list", "--porcelain"])
        .output()?;
    Ok(porcelain::parse(&text)
        .into_iter()
        .find(|entry| same_path(&entry.path, tree)))
}

/// Makes the worktree, its seeded target and the `target` link, converging
/// from whatever an interrupted earlier run left: an existing worktree on the
/// slug's branch is kept, a target the link already names is kept, and
/// anything else is (re)made. Returns the worktree's path.
pub fn new(layout: &Layout, slug: &Slug, from: &str) -> Result<PathBuf, Error> {
    layout.need_build_root()?;
    let tree = layout.tree(slug);
    let dest = layout.target(slug);
    let branch = branch_ref(slug);
    match registered(layout, &tree)? {
        Some(Entry {
            state: EntryState::Present,
            head:
                Head::Commit {
                    branch: Some(found),
                    ..
                },
            ..
        }) if found == branch => {
            eprintln!("{} already exists on {slug}; finishing it.", tree.display());
        }
        Some(Entry {
            state: EntryState::Prunable,
            ..
        }) => {
            // Registered but its directory is gone: git's own record is stale.
            git(&layout.main).args(["worktree", "prune"]).run()?;
            add(layout, &tree, slug, from)?;
        }
        Some(_) => {
            return Err(Error::WorktreeConflict {
                path: tree,
                branch: slug.to_string(),
            });
        }
        None if tree.exists() => {
            return Err(Error::WorktreeConflict {
                path: tree,
                branch: slug.to_string(),
            });
        }
        None => add(layout, &tree, slug, from)?,
    }
    let linked = std::fs::read_link(tree.join("target")).is_ok_and(|to| to == dest);
    if !(linked && dest.is_dir()) {
        clone_seed(layout, &dest)?;
        link_target(&tree, &dest)?;
    }
    allow_direnv(&tree)?;
    Ok(tree)
}

fn add(layout: &Layout, tree: &Path, slug: &Slug, from: &str) -> Result<(), Error> {
    create_dir(&layout.trees)?;
    let exists = git(&layout.main)
        .args(["show-ref", "--verify", "--quiet"])
        .arg(branch_ref(slug))
        .succeeds()?;
    if exists {
        git(&layout.main)
            .args(["worktree", "add"])
            .arg(tree)
            .arg(slug.as_str())
            .run()
    } else {
        // No upstream: the first push names the branch's own remote ref.
        git(&layout.main)
            .args(["worktree", "add", "--no-track", "-b", slug.as_str()])
            .arg(tree)
            .arg(from)
            .run()
    }
}

/// `direnv allow` when the worktree has an `.envrc` and direnv is installed.
fn allow_direnv(tree: &Path) -> Result<(), Error> {
    if !tree.join(".envrc").is_file() {
        return Ok(());
    }
    match Command::new("direnv").arg("allow").arg(tree).status() {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(Error::CommandFailed {
            program: "direnv",
            status,
            stderr: String::new(),
        }),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(Error::Spawn {
            program: "direnv",
            source,
        }),
    }
}

/// Replaces a worktree's target with a fresh clone of the current seed.
pub fn reset(layout: &Layout, slug: &Slug) -> Result<(), Error> {
    layout.need_build_root()?;
    let tree = layout.tree(slug);
    if !tree.is_dir() {
        return Err(Error::NoWorktree { path: tree });
    }
    let dest = layout.target(slug);
    clone_seed(layout, &dest)?;
    link_target(&tree, &dest)
}

/// Removes a worktree and its target; the branch is kept. git refuses a
/// worktree with changes. A worktree already gone whose target remains has
/// the target removed, so a rerun after a crash finishes the job. A directory
/// under `.worktrees` that git does not know, or a slug with neither a
/// worktree nor a target, is an error and nothing is deleted.
pub fn rm(layout: &Layout, slug: &Slug) -> Result<(), Error> {
    let tree = layout.tree(slug);
    let target = layout.target(slug);
    match registered(layout, &tree)?.map(|entry| entry.state) {
        Some(EntryState::Present) => git(&layout.main)
            .args(["worktree", "remove"])
            .arg(&tree)
            .run()?,
        Some(EntryState::Prunable) => git(&layout.main).args(["worktree", "prune"]).run()?,
        None if tree.exists() => return Err(Error::UnknownWorktree { path: tree }),
        None if target.exists() => {}
        None => return Err(Error::NoWorktree { path: tree }),
    }
    remove_tree(&target)?;
    println!("Removed {slug}; branch {slug} is kept.");
    Ok(())
}
