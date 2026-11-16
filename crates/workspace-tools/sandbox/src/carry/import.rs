//! `coda-sandbox carry import`: in the container, recreate carried host
//! worktrees from the bundle and summary the host's `carry` wrote.

use std::path::{Path, PathBuf};

use super::collect::Select;
use super::summary::{self, Carried, Checkout, Dirty};
use crate::error::Error;
use crate::git::git;
use crate::ids::Slug;
use crate::worktree::Layout;
use crate::worktree::new::{allow_direnv, branch_commit, new_on, registered};

/// What import did with one carried worktree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Imported {
    pub slug: Slug,
    pub tree: PathBuf,
    pub branch: String,
    pub outcome: Outcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// Made now, or finished after an interrupted import, with its
    /// uncommitted files back when it had some.
    Made { restored: bool, upstream: Upstream },
    /// Import made it from this carry already, or something else made a
    /// worktree there, so import left it alone.
    Present,
    /// The host's main checkout, which the container's own checkout stands
    /// for; its commits are in `refs/carry/<slug>`.
    MainCheckout { dirty: bool },
}

/// Whether the branch tracks `origin`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Upstream {
    Tracking(String),
    /// `origin` has no branch by that name yet.
    None,
}

/// How far import got with one worktree, kept in the repository's git
/// directory so a rerun knows an interrupted import from a worktree made
/// some other way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Progress {
    /// Started: the worktree may lack its files, direnv or upstream.
    Started,
    Done,
}

impl Progress {
    fn as_str(self) -> &'static str {
        match self {
            Self::Started => "started",
            Self::Done => "done",
        }
    }
}

/// Where import records [`Progress`] for one worktree.
struct Mark {
    path: PathBuf,
    /// What the mark is about: this carry of this worktree.
    carry: String,
}

impl Mark {
    fn new(layout: &Layout, carried: &Carried) -> Result<Self, Error> {
        let common = git(&layout.main)
            .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
            .line()?;
        let dirty = match &carried.dirty {
            Dirty::Clean => "no",
            Dirty::Snapshot(commit) => commit,
        };
        Ok(Self {
            path: PathBuf::from(common)
                .join("coda-carry")
                .join(carried.slug.as_str()),
            carry: format!("{}\t{dirty}", carried.head),
        })
    }

    /// The progress recorded for this carry, if any.
    fn read(&self) -> Result<Option<Progress>, Error> {
        let text = match std::fs::read_to_string(&self.path) {
            Ok(text) => text,
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(source) => return Err(Error::fs("read", &self.path)(source)),
        };
        let Some((carry, progress)) = text.trim_end().rsplit_once('\t') else {
            return Ok(None);
        };
        if carry != self.carry {
            return Ok(None);
        }
        Ok([Progress::Started, Progress::Done]
            .into_iter()
            .find(|known| known.as_str() == progress))
    }

    fn write(&self, progress: Progress) -> Result<(), Error> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent).map_err(Error::fs("create", parent))?;
        }
        std::fs::write(
            &self.path,
            format!("{}\t{}\n", self.carry, progress.as_str()),
        )
        .map_err(Error::fs("write", &self.path))
    }
}

/// The carried worktrees `select` takes.
fn chosen(carried: Vec<Carried>, select: &Select) -> Result<Vec<Carried>, Error> {
    let found: Vec<&str> = carried.iter().map(|one| one.slug.as_str()).collect();
    if let Some(name) = select.missing(&found) {
        return Err(Error::NotCarried {
            slug: name.to_owned(),
            source_name: "the carry summary".to_owned(),
        });
    }
    Ok(carried
        .into_iter()
        .filter(|one| select.takes(one.slug.as_str()))
        .collect())
}

/// Brings the bundle's objects and `origin`'s branches into the checkout.
/// The bundle leaves out what the host's `origin/main` has, so `origin` is
/// fetched first. Old `refs/carry` refs are replaced by the bundle's.
fn fetch(layout: &Layout, bundle: &Path) -> Result<(), Error> {
    let has_origin = git(&layout.main)
        .args(["remote", "get-url", "origin"])
        .succeeds()?;
    if has_origin {
        git(&layout.main)
            .args(["fetch", "--quiet", "origin"])
            .run()?;
    }
    let refs = git(&layout.main)
        .args(["for-each-ref", "--format=delete %(refname)", "refs/carry"])
        .output()?;
    if !refs.is_empty() {
        git(&layout.main)
            .args(["update-ref", "--stdin"])
            .stdin(refs)
            .run()?;
    }
    // No bundle: every carried commit is already on origin/main.
    if !bundle.is_file() {
        return Ok(());
    }
    git(&layout.main)
        .args(["fetch", "--quiet"])
        .arg(bundle)
        .arg("refs/carry/*:refs/carry/*")
        .run()
}

/// Makes one carried worktree: from its head commit, on its host branch,
/// with its uncommitted files back in the work tree (the index is not
/// restored, so staged work comes back unstaged) and tracking `origin`'s
/// branch of the same name when there is one. A worktree whose import was
/// interrupted is finished; one made otherwise is left alone.
fn make(layout: &Layout, carried: &Carried) -> Result<Imported, Error> {
    let slug = carried.slug.slug().clone();
    let branch = carried
        .branch
        .clone()
        .unwrap_or_else(|| slug.as_str().to_owned());
    if carried.checkout == Checkout::Main {
        return Ok(Imported {
            slug,
            tree: layout.main.clone(),
            branch,
            outcome: Outcome::MainCheckout {
                dirty: matches!(carried.dirty, Dirty::Snapshot(_)),
            },
        });
    }
    let tree = layout.tree(&slug);
    let mark = Mark::new(layout, carried)?;
    match (registered(layout, &tree)?, mark.read()?) {
        (Some(_), None | Some(Progress::Done)) => {
            return Ok(Imported {
                slug,
                tree,
                branch,
                outcome: Outcome::Present,
            });
        }
        (_, Some(Progress::Started)) => {}
        (None, None | Some(Progress::Done)) => {
            if let Some(found) = branch_commit(layout, &branch)?
                && found != carried.head
            {
                return Err(Error::CarriedBranchMoved {
                    branch,
                    found,
                    carried: carried.head.clone(),
                });
            }
            mark.write(Progress::Started)?;
        }
    }
    let tree = new_on(layout, &slug, &branch, &carried.head)?;
    let restored = match &carried.dirty {
        Dirty::Clean => false,
        Dirty::Snapshot(commit) => {
            git(&tree)
                .args(["restore", "--source", commit, "--worktree", "--", "."])
                .run()?;
            // A restored `.envrc` differs from the one `new_on` allowed.
            allow_direnv(&tree)?;
            true
        }
    };
    let remote = format!("refs/remotes/origin/{branch}");
    let tracked = git(&layout.main)
        .args(["show-ref", "--verify", "--quiet", &remote])
        .succeeds()?;
    let upstream = if tracked {
        let name = format!("origin/{branch}");
        git(&tree)
            .args(["branch", "--quiet", "--set-upstream-to", &name])
            .run()?;
        Upstream::Tracking(name)
    } else {
        Upstream::None
    };
    mark.write(Progress::Done)?;
    Ok(Imported {
        slug,
        tree,
        branch,
        outcome: Outcome::Made { restored, upstream },
    })
}

/// Reads `summary.txt` and `carry.bundle` from `dir` and makes each
/// selected worktree that is not there yet. A worktree already made is
/// left untouched and one interrupted is finished, so rerunning after a
/// failure finishes the rest.
pub fn import(layout: &Layout, dir: &Path, select: &Select) -> Result<Vec<Imported>, Error> {
    let summary_path = dir.join("summary.txt");
    let text = std::fs::read_to_string(&summary_path).map_err(Error::fs("read", &summary_path))?;
    let chosen = chosen(summary::parse(&text, &summary_path)?, select)?;
    fetch(layout, &dir.join("carry.bundle"))?;
    chosen.iter().map(|carried| make(layout, carried)).collect()
}

/// One line per worktree, for the terminal.
pub fn render(imported: &[Imported]) -> String {
    imported
        .iter()
        .map(|one| {
            let what = match &one.outcome {
                Outcome::Present => "already there, left alone".to_owned(),
                Outcome::MainCheckout { dirty: false } => {
                    "the host's main checkout; this checkout stands for it".to_owned()
                }
                Outcome::MainCheckout { dirty: true } => format!(
                    "the host's main checkout, not made; `git restore --source \
                     refs/carry/{}/dirty --worktree -- .` brings its uncommitted files here",
                    one.slug
                ),
                Outcome::Made { restored, upstream } => {
                    let files = if *restored {
                        "uncommitted files restored"
                    } else {
                        "clean"
                    };
                    let tracking = match upstream {
                        Upstream::Tracking(name) => format!("tracking {name}"),
                        Upstream::None => "no upstream yet".to_owned(),
                    };
                    format!("{files}, {tracking}")
                }
            };
            format!(
                "{}\t{}\ton {}: {what}\n",
                one.slug,
                one.tree.display(),
                one.branch
            )
        })
        .collect()
}
