//! `worktree seed [<ref>]`: build the third-party output worktree targets are
//! cloned from, with `dev/session-seed.sh` in the checkout's dev shell.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

use super::Layout;
use super::clone::{link_target, seed_key};
use super::walk::{self, create_dir, remove_tree};
use super::{MARKER, new::DEFAULT_FROM};
use crate::error::Error;
use crate::git::git;
use crate::ids::SeedKey;

/// The seed's build recipe, run from the seed worktree.
pub const RECIPE: &str = "dev/session-seed.sh";

/// Builds the seed for `reference` unless it is already built, makes it
/// current, and drops seeds beyond `keep_seeds`, newest first.
///
/// Converges after a crash: the seed worktree is checked out afresh, and a
/// seed directory without its marker (an interrupted build) is rebuilt from
/// empty; the marker is written only after the recipe succeeds.
pub fn seed(layout: &Layout, reference: Option<&str>) -> Result<SeedKey, Error> {
    let reference = reference.unwrap_or(DEFAULT_FROM);
    layout.need_build_root()?;
    let tree = layout.seed_tree();
    let registered = super::new::registered(layout, &tree)?.is_some();
    if registered && tree.is_dir() {
        git(&tree)
            .args(["checkout", "-q", "--detach", reference])
            .run()?;
    } else {
        if registered {
            git(&layout.main).args(["worktree", "prune"]).run()?;
        }
        create_dir(&layout.trees)?;
        git(&layout.main)
            .args(["worktree", "add", "-q", "--detach"])
            .arg(&tree)
            .arg(reference)
            .run()?;
    }
    let key = seed_key(&tree)?;
    let dest = layout.seeds.join(key.as_str());
    if dest.join(MARKER).exists() {
        println!("Seed {key} is already built.");
    } else {
        remove_tree(&dest)?;
        create_dir(&dest)?;
        link_target(&tree, &dest)?;
        build(&tree)?;
        walk::write(&dest.join(MARKER), &format!("{key}\n"))?;
    }
    // Newest first when pruning below, even when it was built long ago.
    touch(&dest)?;
    walk::replace_symlink(Path::new(key.as_str()), &layout.seeds.join("current"))?;
    prune(layout)?;
    println!("Seed {key} is current.");
    Ok(key)
}

fn build(tree: &Path) -> Result<(), Error> {
    let status = Command::new("nix")
        .args(["develop", ".#agent", "--command", RECIPE])
        .current_dir(tree)
        .status()
        .map_err(|source| Error::Spawn {
            program: "nix",
            source,
        })?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::SeedBuildFailed {
            tree: tree.to_owned(),
            status,
        })
    }
}

fn touch(dir: &Path) -> Result<(), Error> {
    let file = std::fs::File::open(dir).map_err(Error::fs("open", dir))?;
    file.set_modified(SystemTime::now())
        .map_err(Error::fs("touch", dir))
}

/// Seed directories, newest first by modification time.
fn seeds_newest_first(seeds: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(seeds).map_err(Error::fs("list", seeds))? {
        let entry = entry.map_err(Error::fs("list", seeds))?;
        let path = entry.path();
        let meta = std::fs::symlink_metadata(&path).map_err(Error::fs("inspect", &path))?;
        if meta.is_dir() {
            let modified = meta
                .modified()
                .map_err(Error::fs("read the time of", &path))?;
            found.push((modified, path));
        }
    }
    found.sort_by_key(|(modified, _)| std::cmp::Reverse(*modified));
    Ok(found.into_iter().map(|(_, path)| path).collect())
}

/// Older seeds go; clones keep their blocks until they are reset.
fn prune(layout: &Layout) -> Result<(), Error> {
    for old in seeds_newest_first(&layout.seeds)?
        .into_iter()
        .skip(layout.keep_seeds.get())
    {
        remove_tree(&old)?;
    }
    Ok(())
}
