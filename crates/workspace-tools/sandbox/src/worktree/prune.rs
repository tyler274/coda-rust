//! `worktree prune [--yes]`: remove every clean worktree merged into
//! origin/main, with its target.

use std::io::{BufRead as _, IsTerminal as _, Write as _};

use super::Layout;
use super::new::{registered, rm};
use crate::error::Error;
use crate::git::git;
use crate::ids::Slug;
use crate::porcelain::EntryState;

/// Whether to ask before removing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Confirm {
    Ask,
    Yes,
}

/// The worktrees whose HEAD is in origin/main and that have no changes. A
/// directory under `.worktrees` git does not know is skipped: run there, git
/// would answer for the main checkout instead.
pub fn merged(layout: &Layout) -> Result<Vec<Slug>, Error> {
    git(&layout.main)
        .args(["fetch", "-q", "origin", "main"])
        .run()?;
    let mut merged = Vec::new();
    for slug in layout.tree_slugs()? {
        let tree = layout.tree(&slug);
        let state = registered(layout, &tree)?.map(|entry| entry.state);
        if state != Some(EntryState::Present) {
            continue;
        }
        let in_main = git(&tree)
            .args(["merge-base", "--is-ancestor", "HEAD", "origin/main"])
            .succeeds()?;
        if !in_main {
            continue;
        }
        if git(&tree)
            .args(["status", "--porcelain"])
            .output()?
            .is_empty()
        {
            merged.push(slug);
        }
    }
    Ok(merged)
}

/// Each removal is whole, so a rerun after an interruption removes the rest.
pub fn prune(layout: &Layout, confirm: Confirm) -> Result<(), Error> {
    let merged = merged(layout)?;
    if merged.is_empty() {
        println!("No clean worktree is merged into origin/main.");
        return Ok(());
    }
    for slug in &merged {
        println!("Merged and clean: {slug}");
    }
    if confirm == Confirm::Ask {
        let stdin = std::io::stdin();
        if !stdin.is_terminal() {
            return Err(Error::NeedsYes);
        }
        print!("Remove these worktrees and their targets? [y/N] ");
        std::io::stdout().flush().map_err(Error::Output)?;
        let mut answer = String::new();
        stdin.lock().read_line(&mut answer).map_err(Error::Prompt)?;
        if answer.trim() != "y" {
            return Ok(());
        }
    }
    for slug in &merged {
        rm(layout, slug)?;
    }
    Ok(())
}
