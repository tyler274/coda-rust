//! Reflink clones of a seed into worktree targets, and the seed key.

use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest as _, Sha256};

use super::walk::{self, create_dir, remove_tree, rename};
use super::{Layout, MARKER, current_seed};
use crate::error::Error;
use crate::ids::{SeedKey, Slug};

/// The files that decide whether third-party build output can be reused.
pub const KEY_INPUTS: [&str; 3] = ["Cargo.lock", "flake.lock", "rust-toolchain.toml"];

/// The first 16 hex digits of the sha256 of the key inputs' contents, in
/// order, as `cat Cargo.lock flake.lock rust-toolchain.toml | sha256sum`.
pub fn seed_key_of(contents: &[&[u8]]) -> SeedKey {
    let mut hasher = Sha256::new();
    for content in contents {
        hasher.update(content);
    }
    SeedKey::from_digest(&hasher.finalize().into())
}

/// The seed key of a checkout.
pub fn seed_key(tree: &Path) -> Result<SeedKey, Error> {
    let mut contents = Vec::new();
    for name in KEY_INPUTS {
        let path = tree.join(name);
        contents.push(std::fs::read(&path).map_err(|source| Error::Fs {
            op: "read",
            path,
            source,
        })?);
    }
    let slices: Vec<&[u8]> = contents.iter().map(Vec::as_slice).collect();
    Ok(seed_key_of(&slices))
}

/// Where a target's retained proof waits while the target is replaced.
fn kept_proof(layout: &Layout, dest: &Path) -> PathBuf {
    let name = dest
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    layout.targets.join(format!(".verification.{name}"))
}

/// Replaces `dest` with a clone of the current seed, or an empty directory
/// when there is no seed yet. Never falls back to a copy: a full copy per
/// worktree is the disk use this exists to avoid. `verification/` holds
/// retained proof, not build output, so it survives.
///
/// Converges after a crash: proof left in `.verification.<slug>` by an
/// interrupted run is restored by the next one.
pub fn clone_seed(layout: &Layout, dest: &Path) -> Result<(), Error> {
    let kept = kept_proof(layout, dest);
    let proof = dest.join("verification");
    let stash = kept.join("verification");
    match (proof.is_dir(), stash.is_dir()) {
        (true, true) => return Err(Error::ProofConflict { kept: stash, proof }),
        (true, false) => {
            create_dir(&kept)?;
            rename(&proof, &stash)?;
        }
        _ => {}
    }
    remove_tree(dest)?;
    create_dir(dest)?;
    let seed = current_seed(layout)?;
    let cloned = match &seed {
        None => {
            eprintln!(
                "No seed yet; {} starts empty. Run `coda-sandbox worktree seed` to share third-party builds.",
                dest.display()
            );
            Ok(())
        }
        Some(seed) => reflink(seed, dest),
    };
    if cloned.is_err() {
        remove_tree(dest)?;
        create_dir(dest)?;
    }
    if stash.is_dir() {
        remove_tree(&proof)?;
        rename(&stash, &proof)?;
        remove_tree(&kept)?;
    }
    cloned
}

/// `cp -a --reflink=always <seed>/. <dest>/`, then the marker naming the seed.
fn reflink(seed: &Path, dest: &Path) -> Result<(), Error> {
    let mut into = dest.as_os_str().to_owned();
    into.push("/");
    let output = Command::new("cp")
        .arg("-a")
        .arg("--reflink=always")
        .arg(seed.join("."))
        .arg(into)
        .output()
        .map_err(|source| Error::Spawn {
            program: "cp",
            source,
        })?;
    if !output.status.success() {
        return Err(Error::ReflinkFailed {
            seed: seed.to_owned(),
            dest: dest.to_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        });
    }
    let name = seed
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    walk::write(&dest.join(MARKER), &format!("{name}\n"))
}

/// Points `<tree>/target` at `dest`. A real `target/` directory stops it
/// rather than being replaced.
pub fn link_target(tree: &Path, dest: &Path) -> Result<(), Error> {
    let link = tree.join("target");
    if let Ok(meta) = std::fs::symlink_metadata(&link)
        && !meta.file_type().is_symlink()
    {
        return Err(Error::RealTarget { path: link });
    }
    walk::replace_symlink(dest, &link)
}

/// A slug's target is pristine when it is an untouched clone of the current
/// seed: its marker names that seed and nothing below it is newer than the
/// marker.
pub fn pristine(layout: &Layout, slug: &Slug) -> Result<bool, Error> {
    let dir = layout.target(slug);
    let Some(seed) = current_seed(layout)? else {
        return Ok(false);
    };
    let marker = dir.join(MARKER);
    let Some(text) = walk::read(&marker)? else {
        return Ok(false);
    };
    let seed_name = seed.file_name().map(|n| n.to_string_lossy().into_owned());
    if Some(text.trim().to_owned()) != seed_name {
        return Ok(false);
    }
    let marked = std::fs::metadata(&marker)
        .and_then(|meta| meta.modified())
        .map_err(|source| Error::Fs {
            op: "read the time of",
            path: marker.clone(),
            source,
        })?;
    Ok(walk::newest_below(&dir, 3)?.is_none_or(|newest| newest <= marked))
}
