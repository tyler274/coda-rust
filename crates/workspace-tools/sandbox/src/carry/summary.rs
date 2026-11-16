//! `summary.txt`: one line per carried worktree, which `carry` writes on the
//! host and `carry import` reads in the container.
//!
//! `<slug>\t<head>\tdirty=<commit>|dirty=no\tbranch=<name>\tcheckout=main|linked\t<host path>`,
//! with an empty branch name for a detached worktree. The commits are named
//! here rather than read from the bundle's refs, because the bundle leaves
//! out a worktree whose commits `origin` already has.

use std::path::{Path, PathBuf};

use super::naming::CarrySlug;
use crate::error::Error;

/// One carried worktree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Carried {
    pub slug: CarrySlug,
    pub head: String,
    pub dirty: Dirty,
    /// The branch the worktree had checked out, without `refs/heads/`;
    /// `None` when it was detached.
    pub branch: Option<String>,
    pub checkout: Checkout,
    /// Where it was on the host.
    pub path: PathBuf,
}

/// Whether the worktree had changes, and so a snapshot commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dirty {
    Clean,
    /// The snapshot commit: `head` plus uncommitted and untracked files.
    Snapshot(String),
}

/// Which of the host repository's checkouts it was.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Checkout {
    /// The main checkout, which the container's own checkout stands for.
    Main,
    /// A linked worktree, which `carry import` makes.
    Linked,
}

impl Checkout {
    fn as_str(self) -> &'static str {
        match self {
            Self::Main => "main",
            Self::Linked => "linked",
        }
    }

    fn parse(text: &str) -> Option<Self> {
        match text {
            "main" => Some(Self::Main),
            "linked" => Some(Self::Linked),
            _ => None,
        }
    }
}

impl Carried {
    pub fn summary_line(&self) -> String {
        let dirty = match &self.dirty {
            Dirty::Clean => "no",
            Dirty::Snapshot(commit) => commit,
        };
        format!(
            "{}\t{}\tdirty={dirty}\tbranch={}\tcheckout={}\t{}\n",
            self.slug,
            self.head,
            self.branch.as_deref().unwrap_or_default(),
            self.checkout.as_str(),
            self.path.display()
        )
    }

    fn parse_line(line: &str) -> Option<Self> {
        let mut fields = line.splitn(6, '\t');
        let slug = CarrySlug::parse(fields.next()?)?;
        let head = fields.next()?.to_owned();
        let dirty = match fields.next()?.strip_prefix("dirty=")? {
            "no" => Dirty::Clean,
            commit => Dirty::Snapshot(commit.to_owned()),
        };
        let branch = match fields.next()?.strip_prefix("branch=")? {
            "" => None,
            name => Some(name.to_owned()),
        };
        let checkout = Checkout::parse(fields.next()?.strip_prefix("checkout=")?)?;
        let path = PathBuf::from(fields.next()?);
        Some(Self {
            slug,
            head,
            dirty,
            branch,
            checkout,
            path,
        })
    }
}

/// Every line of a summary, or which line is not one.
pub fn parse(text: &str, source: &Path) -> Result<Vec<Carried>, Error> {
    text.lines()
        .map(|line| {
            Carried::parse_line(line).ok_or_else(|| Error::UnexpectedOutput {
                what: "carry summary line",
                source_name: source.display().to_string(),
                found: line.to_owned(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(dirty: Dirty, branch: Option<&str>, checkout: Checkout) -> Carried {
        Carried {
            slug: CarrySlug::parse("eng-7").expect("slug"),
            head: "a".repeat(40),
            dirty,
            branch: branch.map(str::to_owned),
            checkout,
            path: PathBuf::from("/w/eng 7"),
        }
    }

    #[test]
    fn lines_read_back_as_written() {
        let carried = [
            sample(Dirty::Clean, Some("tyler/eng-7"), Checkout::Linked),
            sample(Dirty::Snapshot("b".repeat(40)), None, Checkout::Main),
        ];
        let text: String = carried.iter().map(Carried::summary_line).collect();
        assert_eq!(parse(&text, Path::new("s")).expect("parses"), carried);
    }

    #[test]
    fn a_line_in_an_old_format_is_refused() {
        for old in [
            "eng-7\taaaa\tdirty=yes\t/w/eng-7\n",
            "eng-7\taaaa\tdirty=no\tbranch=x\t/w/eng-7\n",
        ] {
            let found = parse(old, Path::new("s"));
            assert!(
                matches!(found, Err(Error::UnexpectedOutput { .. })),
                "{found:?}"
            );
        }
    }
}
