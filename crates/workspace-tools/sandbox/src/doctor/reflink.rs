//! Whether the build root's filesystem can clone files, which worktree build
//! directories depend on.

use std::path::Path;

use super::{Fix, Outcome};
use crate::probe::{FsType, PoolFeature, Probe, ZFS_BCLONE_PARAMETER};

pub(super) fn outcome(probe: &dyn Probe, root: &Path) -> Outcome {
    let fs = match probe.fs_type(root) {
        Ok(fs) => fs,
        Err(cause) => return Outcome::CannotCheck { cause },
    };
    match fs {
        FsType::Btrfs => Outcome::Pass {
            detail: "btrfs".to_owned(),
        },
        FsType::Xfs => Outcome::Warn {
            detail: "xfs, assumed reflink=1 (the default since xfsprogs 5.1)".to_owned(),
            fix: Fix::command("xfs_info", [root.display().to_string()]),
        },
        FsType::Other => Outcome::Fail {
            detail: format!("{} is not on ZFS, Btrfs or XFS", root.display()),
            fix: Fix::instruction("move the build root onto a reflink filesystem"),
        },
        FsType::Zfs => zfs(probe, root),
    }
}

fn zfs(probe: &dyn Probe, root: &Path) -> Outcome {
    match probe.read_file(Path::new(ZFS_BCLONE_PARAMETER)) {
        Ok(Some(value)) if value.trim() == "1" => zfs_pool(probe, root),
        Ok(Some(value)) => Outcome::Fail {
            detail: format!(
                "zfs, block cloning off (zfs_bclone_enabled = {})",
                value.trim()
            ),
            fix: Fix::instruction(format!(
                "add `boot.kernelParams = [ \"zfs.zfs_bclone_enabled=1\" ];` and rebuild, \
                 or now: `echo 1 | sudo tee {ZFS_BCLONE_PARAMETER}`"
            )),
        },
        Ok(None) => Outcome::Fail {
            detail: format!("zfs without {ZFS_BCLONE_PARAMETER}; block cloning needs OpenZFS 2.2"),
            fix: Fix::instruction("upgrade ZFS to 2.2 or later"),
        },
        Err(cause) => Outcome::CannotCheck { cause },
    }
}

/// The kernel parameter is on; the pool must also have the feature. When the
/// pool cannot be read the parameter's verdict stands, and the detail says so.
fn zfs_pool(probe: &dyn Probe, root: &Path) -> Outcome {
    let source = match probe.mount_source(root) {
        Ok(source) => source,
        Err(cause) => return unchecked(&cause.chain()),
    };
    let Some(pool) = pool_of(&source) else {
        return unchecked(&format!("no pool in mount source {source:?}"));
    };
    match probe.pool_block_cloning(pool) {
        Ok(feature @ (PoolFeature::Enabled | PoolFeature::Active)) => Outcome::Pass {
            detail: format!("zfs, block cloning on; pool {pool} feature {feature:?}")
                .to_lowercase(),
        },
        Ok(PoolFeature::Disabled) => Outcome::Fail {
            detail: format!("zfs, pool {pool} has feature@block_cloning disabled"),
            fix: Fix::command(
                "sudo",
                ["zpool", "set", "feature@block_cloning=enabled", pool],
            ),
        },
        Err(cause) => unchecked(&cause.chain()),
    }
}

fn unchecked(reason: &str) -> Outcome {
    Outcome::Pass {
        detail: format!("zfs, block cloning on; pool feature not checked ({reason})"),
    }
}

/// `agents/coda/build` → `agents`.
fn pool_of(source: &str) -> Option<&str> {
    source.split('/').next().filter(|pool| !pool.is_empty())
}
