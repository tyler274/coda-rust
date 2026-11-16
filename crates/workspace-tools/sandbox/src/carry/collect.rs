//! Records each worktree as refs and writes the bundle and lists.

use std::path::{Path, PathBuf};

use super::naming::{CarrySlug, Names, under_target};
use super::summary::{Carried, Checkout, Dirty};
use crate::error::Error;
use crate::git::git;
use crate::porcelain::{self, EntryState, Head};
use crate::worktree::same_path;

/// The commits the container already has from `origin`, so the bundle need
/// not carry them. A worktree whose commits are all in it is left out of the
/// bundle by git, which is why the summary, not the bundle's refs, names
/// each worktree's commits.
const BASIS: &str = "refs/remotes/origin/main";

/// Which worktrees to carry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Select {
    All,
    /// Only these, by their carry slug; each must name a worktree.
    Named(Vec<String>),
}

impl Select {
    pub fn from_names(names: Vec<String>) -> Self {
        if names.is_empty() {
            Self::All
        } else {
            Self::Named(names)
        }
    }

    pub fn takes(&self, slug: &str) -> bool {
        match self {
            Self::All => true,
            Self::Named(names) => names.iter().any(|name| name == slug),
        }
    }

    /// The first named slug none of `found` answers to.
    pub fn missing<'a>(&'a self, found: &[&str]) -> Option<&'a str> {
        match self {
            Self::All => None,
            Self::Named(names) => names
                .iter()
                .map(String::as_str)
                .find(|name| !found.contains(name)),
        }
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

/// Records one worktree's refs and appends its ignored files.
fn record(
    repo: &Path,
    path: PathBuf,
    slug: CarrySlug,
    head: &str,
    branch: Option<&str>,
    ignored: &mut String,
) -> Result<Carried, Error> {
    let checkout = if same_path(&path, repo) {
        Checkout::Main
    } else {
        Checkout::Linked
    };
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
        Dirty::Snapshot(commit)
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
    Ok(Carried {
        slug,
        head: head.to_owned(),
        dirty,
        branch: branch.map(|name| name.strip_prefix("refs/heads/").unwrap_or(name).to_owned()),
        checkout,
        path,
    })
}

/// Writes `carry.bundle` with every `refs/carry` ref, less what [`BASIS`]
/// holds when the repository has it. Writes nothing when that leaves no
/// commit to carry, as git refuses an empty bundle.
fn bundle(repo: &Path, to: &Path) -> Result<(), Error> {
    let basis = git(repo)
        .args(["rev-parse", "--verify", "--quiet", BASIS])
        .line_if_ok()?;
    let Some(basis) = basis else {
        return git(repo)
            .args(["bundle", "create", "--quiet"])
            .arg(to)
            .arg("--glob=refs/carry/*")
            .run();
    };
    let exclude = format!("^{basis}");
    let new = git(repo)
        .args(["rev-list", "-n1", "--glob=refs/carry/*", &exclude])
        .line()?;
    if new.is_empty() {
        return Ok(());
    }
    git(repo)
        .args(["bundle", "create", "--quiet"])
        .arg(to)
        .args(["--glob=refs/carry/*", &exclude])
        .run()
}

/// Records each present worktree of `repo` that `select` takes and writes
/// `carry.bundle`, `ignored.txt` and `summary.txt` into `staging`. Every
/// worktree claims its slug, carried or not, so a name means the same
/// worktree whichever are selected.
pub fn collect(repo: &Path, staging: &Path, select: &Select) -> Result<Vec<Carried>, Error> {
    clear(repo)?;
    let listing = git(repo)
        .args(["worktree", "list", "--porcelain"])
        .output()?;
    let mut names = Names::default();
    let mut carried = Vec::new();
    let mut ignored = String::new();
    for entry in porcelain::parse(&listing) {
        let (EntryState::Present, Head::Commit { oid: head, branch }) = (&entry.state, &entry.head)
        else {
            continue;
        };
        if !entry.path.is_dir() {
            continue;
        }
        let slug = names.claim(&entry.path);
        if !select.takes(slug.as_str()) {
            continue;
        }
        carried.push(record(
            repo,
            entry.path.clone(),
            slug,
            head,
            branch.as_deref(),
            &mut ignored,
        )?);
    }
    let found: Vec<&str> = carried.iter().map(|one| one.slug.as_str()).collect();
    if let Some(name) = select.missing(&found) {
        return Err(Error::NotCarried {
            slug: name.to_owned(),
            source_name: repo.display().to_string(),
        });
    }
    write(&staging.join("ignored.txt"), &ignored)?;
    let summary: String = carried.iter().map(Carried::summary_line).collect();
    write(&staging.join("summary.txt"), &summary)?;
    bundle(repo, &staging.join("carry.bundle"))?;
    Ok(carried)
}
