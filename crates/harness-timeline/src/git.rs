//! The Git reads the collector needs, through the `git` command.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::CommandError;
use crate::key::Revision;

/// Runs `git` in `dir` and returns its trimmed standard output.
pub fn git(dir: &Path, args: &[&str]) -> Result<String, CommandError> {
    crate::error::output(Command::new("git").arg("-C").arg(dir).args(args))
}

/// One linked or main worktree with a branch checked out.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Worktree {
    pub path: PathBuf,
    pub branch: String,
    pub head: Revision,
}

/// Reads `git worktree list --porcelain` output. Detached and bare worktrees have
/// no branch to place on a timeline and are left out.
pub fn parse_worktrees(porcelain: &str) -> Vec<Worktree> {
    porcelain
        .split("\n\n")
        .filter_map(|block| {
            let mut path = None;
            let mut head = None;
            let mut branch = None;
            for line in block.lines() {
                if let Some(value) = line.strip_prefix("worktree ") {
                    path = Some(PathBuf::from(value));
                } else if let Some(value) = line.strip_prefix("HEAD ") {
                    head = Revision::parse(value);
                } else if let Some(value) = line.strip_prefix("branch ") {
                    branch = value.strip_prefix("refs/heads/").map(str::to_owned);
                }
            }
            Some(Worktree {
                path: path?,
                branch: branch?,
                head: head?,
            })
        })
        .collect()
}

/// The repository's worktrees with a branch checked out.
pub fn worktrees(repo: &Path) -> Result<Vec<Worktree>, CommandError> {
    Ok(parse_worktrees(&git(
        repo,
        &["worktree", "list", "--porcelain"],
    )?))
}

/// Tracked files differ from the worktree's head, as capture-proof counts it.
pub fn dirty(worktree: &Path) -> Result<bool, CommandError> {
    Ok(!git(worktree, &["status", "--porcelain", "--untracked-files=no"])?.is_empty())
}

/// Commits on `HEAD` that `origin/main` lacks.
pub fn ahead_of_main(worktree: &Path) -> Result<u32, CommandError> {
    let count = git(worktree, &["rev-list", "--count", "origin/main..HEAD"])?;
    count.parse().map_err(|source| CommandError::Unreadable {
        command: "git rev-list --count origin/main..HEAD".to_owned(),
        output: count,
        source: Box::new(source),
    })
}

/// The folder name capture-proof gives a branch's bundle, which the review and
/// local CI folders share.
pub fn branch_slug(branch: &str) -> String {
    let mut slug = String::with_capacity(branch.len());
    let mut gap = false;
    for c in branch.chars() {
        if c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-') {
            if gap && !slug.is_empty() {
                slug.push('-');
            }
            gap = false;
            slug.push(c);
        } else {
            gap = true;
        }
    }
    let slug = slug.trim_matches('-').to_owned();
    if slug.is_empty() {
        "detached".to_owned()
    } else {
        slug
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugs_match_capture_proof() {
        assert_eq!(branch_slug("dev/eng-141-remove"), "dev-eng-141-remove");
        assert_eq!(branch_slug("harness-timeline"), "harness-timeline");
        assert_eq!(branch_slug("a//b"), "a-b");
        assert_eq!(branch_slug("/"), "detached");
    }

    #[test]
    fn worktree_list_keeps_only_branches() {
        let porcelain = "worktree /r\nHEAD 681f52ec00000000000000000000000000000000\nbranch refs/heads/main\n\n\
                         worktree /w/ci-restack-384\nHEAD 8262380900000000000000000000000000000000\ndetached\n\n\
                         worktree /w/eng-159\nHEAD f5d9fe7000000000000000000000000000000000\nbranch refs/heads/eng-159-chart-evidence\n";
        let found = parse_worktrees(porcelain);
        assert_eq!(found.len(), 2);
        assert_eq!(found[1].branch, "eng-159-chart-evidence");
        assert_eq!(found[1].path, PathBuf::from("/w/eng-159"));
    }
}
