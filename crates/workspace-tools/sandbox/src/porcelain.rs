//! `git worktree list --porcelain`, parsed into one entry per worktree.

use std::path::PathBuf;

/// One worktree as git lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub path: PathBuf,
    pub head: Head,
    pub state: EntryState,
}

/// What the worktree has checked out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Head {
    /// The bare repository itself, which has no checkout.
    Bare,
    /// A commit, on a branch (`refs/heads/...`) or detached.
    Commit { oid: String, branch: Option<String> },
}

/// Whether git still finds the worktree's directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryState {
    Present,
    /// Its directory is gone; `git worktree prune` would remove it.
    Prunable,
}

#[derive(Default)]
struct Partial {
    path: Option<PathBuf>,
    oid: Option<String>,
    branch: Option<String>,
    bare: bool,
    prunable: bool,
}

impl Partial {
    fn finish(self) -> Option<Entry> {
        let path = self.path?;
        let head = if self.bare {
            Head::Bare
        } else {
            Head::Commit {
                oid: self.oid?,
                branch: self.branch,
            }
        };
        let state = if self.prunable {
            EntryState::Prunable
        } else {
            EntryState::Present
        };
        Some(Entry { path, head, state })
    }
}

/// Parses the porcelain output; records without a path or HEAD are skipped.
pub fn parse(text: &str) -> Vec<Entry> {
    let mut entries = Vec::new();
    let mut current = Partial::default();
    for line in text.lines().chain(std::iter::once("")) {
        if line.is_empty() {
            entries.extend(std::mem::take(&mut current).finish());
        } else if let Some(path) = line.strip_prefix("worktree ") {
            current.path = Some(PathBuf::from(path));
        } else if let Some(oid) = line.strip_prefix("HEAD ") {
            current.oid = Some(oid.to_owned());
        } else if let Some(branch) = line.strip_prefix("branch ") {
            current.branch = Some(branch.to_owned());
        } else if line == "bare" {
            current.bare = true;
        } else if line == "prunable" || line.starts_with("prunable ") {
            current.prunable = true;
        }
    }
    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_main_branch_detached_bare_and_prunable() {
        let text = "worktree /r\nHEAD aaa\nbranch refs/heads/main\n\n\
                    worktree /r/.worktrees/x\nHEAD bbb\ndetached\n\n\
                    worktree /bare\nbare\n\n\
                    worktree /gone\nHEAD ccc\nbranch refs/heads/gone\nprunable gitdir file points to non-existent location\n";
        let entries = parse(text);
        assert_eq!(entries.len(), 4);
        assert_eq!(
            entries[0].head,
            Head::Commit {
                oid: "aaa".into(),
                branch: Some("refs/heads/main".into())
            }
        );
        assert_eq!(
            entries[1].head,
            Head::Commit {
                oid: "bbb".into(),
                branch: None
            }
        );
        assert_eq!(entries[2].head, Head::Bare);
        assert_eq!(entries[3].state, EntryState::Prunable);
        assert_eq!(entries[0].state, EntryState::Present);
    }

    #[test]
    fn empty_output_has_no_entries() {
        assert!(parse("").is_empty());
    }
}
