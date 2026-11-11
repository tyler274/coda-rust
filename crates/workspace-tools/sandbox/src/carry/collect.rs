//! Records each worktree as refs and writes the bundle and lists.

use std::path::{Path, PathBuf};

use super::naming::{CarrySlug, Names, under_target};
use crate::error::Error;
use crate::git::git;
use crate::porcelain::{self, EntryState, Head};

/// One carried worktree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Carried {
    pub slug: CarrySlug,
    pub head: String,
    pub dirty: Dirty,
    pub path: PathBuf,
}

/// Whether the worktree had changes, and so a `dirty` ref.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dirty {
    Clean,
    Snapshot,
}

impl Carried {
    pub fn summary_line(&self) -> String {
        let dirty = match self.dirty {
            Dirty::Clean => "no",
            Dirty::Snapshot => "yes",
        };
        format!(
            "{}\t{}\tdirty={dirty}\t{}\n",
            self.slug,
            self.head,
            self.path.display()
        )
    }
}

fn write(path: &Path, text: &str) -> Result<(), Error> {
    std::fs::write(path, text).map_err(|source| Error::Fs {
        op: "write",
        path: path.to_owned(),
        source,
    })
}

/// Deletes every `refs/carry` ref, so refs never accumulate across runs.
fn clear(repo: &Path) -> Result<(), Error> {
    let refs = git(repo)
        .args(["for-each-ref", "--format=delete %(refname)", "refs/carry"])
        .output()?;
    if refs.is_empty() {
        return Ok(());
    }
    git(repo).args(["update-ref", "--stdin"]).stdin(refs).run()
}

/// The snapshot commit of a dirty worktree, on top of `head`, built with a
/// temporary index that starts from the real one so unchanged files are not
/// rehashed.
fn snapshot(path: &Path, head: &str, slug: &CarrySlug) -> Result<String, Error> {
    let scratch = tempfile::tempdir().map_err(|source| Error::Fs {
        op: "create a scratch directory in",
        path: std::env::temp_dir(),
        source,
    })?;
    let index = scratch.path().join("index");
    let real = PathBuf::from(
        git(path)
            .args(["rev-parse", "--path-format=absolute", "--git-path", "index"])
            .line()?,
    );
    if real.is_file() {
        std::fs::copy(&real, &index).map_err(|source| Error::Fs {
            op: "copy the index to",
            path: index.clone(),
            source,
        })?;
    }
    git(path).index(&index).args(["add", "-A"]).run()?;
    let tree = git(path).index(&index).arg("write-tree").line()?;
    git(path)
        .args(["commit-tree", &tree, "-p", head, "-m"])
        .arg(format!("carry: uncommitted work in {slug}"))
        .line()
}

/// Records each present worktree of `repo` and writes `carry.bundle`,
/// `ignored.txt` and `summary.txt` into `staging`.
pub fn collect(repo: &Path, staging: &Path) -> Result<Vec<Carried>, Error> {
    clear(repo)?;
    let listing = git(repo)
        .args(["worktree", "list", "--porcelain"])
        .output()?;
    let mut names = Names::default();
    let mut carried = Vec::new();
    let mut ignored = String::new();
    for entry in porcelain::parse(&listing) {
        let (EntryState::Present, Head::Commit { oid: head, .. }) = (&entry.state, &entry.head)
        else {
            continue;
        };
        if !entry.path.is_dir() {
            continue;
        }
        let path = entry.path.clone();
        let slug = names.claim(&path);
        git(repo)
            .args(["update-ref", &slug.head_ref(), head])
            .run()?;
        let changes = git(&path)
            .args(["status", "--porcelain", "--ignore-submodules=dirty"])
            .output()?;
        let dirty = if changes.is_empty() {
            Dirty::Clean
        } else {
            let commit = snapshot(&path, head, &slug)?;
            git(repo)
                .args(["update-ref", &slug.dirty_ref(), &commit])
                .run()?;
            Dirty::Snapshot
        };
        let others = git(&path)
            .args([
                "ls-files",
                "--others",
                "--ignored",
                "--exclude-standard",
                "--directory",
            ])
            .output()?;
        for file in others.lines().filter(|file| !under_target(file)) {
            ignored.push_str(&format!("{slug}: {file}\n"));
        }
        carried.push(Carried {
            slug,
            head: head.clone(),
            dirty,
            path,
        });
    }
    write(&staging.join("ignored.txt"), &ignored)?;
    let summary: String = carried.iter().map(Carried::summary_line).collect();
    write(&staging.join("summary.txt"), &summary)?;
    git(repo)
        .args(["bundle", "create", "--quiet"])
        .arg(staging.join("carry.bundle"))
        .arg("--glob=refs/carry/*")
        .run()?;
    Ok(carried)
}
