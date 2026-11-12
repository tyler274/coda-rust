//! `worktree gc [--idle <days>]`: reset targets unbuilt for `days` to a seed
//! clone, and remove targets whose worktree is gone. A reset keeps
//! `target/verification`, which holds retained proof.

use std::time::{Duration, SystemTime};

use super::Layout;
use super::clone::{clone_seed, pristine};
use super::walk::{newest_below, remove_tree};
use crate::error::Error;
use crate::ids::Slug;

pub const DEFAULT_IDLE_DAYS: u32 = 7;

/// What gc does with one target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Its worktree is gone.
    Remove,
    /// Idle for the threshold and not already a fresh clone.
    Reset,
    Keep,
}

/// The decision for one target, from what was read about it.
#[expect(
    clippy::fn_params_excessive_bools,
    reason = "three readings of one target that the match turns into one Action; a typed target state is left to this tool's owner"
)]
pub fn decide(has_tree: bool, idle: bool, pristine: bool) -> Action {
    match (has_tree, idle, pristine) {
        (false, _, _) => Action::Remove,
        (true, true, false) => Action::Reset,
        (true, _, _) => Action::Keep,
    }
}

/// True when nothing within three levels of the target changed since
/// `cutoff`.
fn idle(layout: &Layout, slug: &Slug, cutoff: SystemTime) -> Result<bool, Error> {
    Ok(newest_below(&layout.target(slug), 3)?.is_none_or(|newest| newest < cutoff))
}

/// Each step is whole on its own, so an interrupted gc is finished by the
/// next run; an interrupted reset's kept proof is restored by `clone_seed`.
pub fn gc(layout: &Layout, idle_days: u32) -> Result<(), Error> {
    layout.need_build_root()?;
    let cutoff = SystemTime::now()
        .checked_sub(Duration::from_secs(u64::from(idle_days) * 86_400))
        .unwrap_or(SystemTime::UNIX_EPOCH);
    for slug in layout.target_slugs()? {
        let has_tree = layout.tree(&slug).is_dir();
        let action = if has_tree {
            let idle = idle(layout, &slug, cutoff)?;
            let pristine = idle && pristine(layout, &slug)?;
            decide(true, idle, pristine)
        } else {
            decide(false, false, false)
        };
        match action {
            Action::Remove => {
                println!(
                    "Removing {}: its worktree is gone.",
                    layout.target(&slug).display()
                );
                remove_tree(&layout.target(&slug))?;
            }
            Action::Reset => {
                println!("Resetting {slug}: no build for {idle_days} days.");
                clone_seed(layout, &layout.target(&slug))?;
            }
            Action::Keep => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orphans_go_idle_targets_reset_and_fresh_clones_stay() {
        assert_eq!(decide(false, false, false), Action::Remove);
        assert_eq!(decide(false, true, true), Action::Remove);
        assert_eq!(decide(true, true, false), Action::Reset);
        assert_eq!(decide(true, true, true), Action::Keep);
        assert_eq!(decide(true, false, false), Action::Keep);
    }
}
