//! Memory admission: a guest starts only if it fits with the margin left over.
//!
//! The kernel's two reports are read as files, and each parser takes their
//! text, so tests feed fixtures instead of the running machine's numbers.

use std::io;
use std::path::{Path, PathBuf};

use super::error::Error;

pub const MIB: u64 = 1024 * 1024;

/// The ARC lists the kernel can evict under pressure.
const ARC_EVICTABLE: [&str; 4] = [
    "mru_evictable_data",
    "mru_evictable_metadata",
    "mfu_evictable_data",
    "mfu_evictable_metadata",
];

/// Where the kernel reports memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryFiles {
    pub meminfo: PathBuf,
    pub arcstats: PathBuf,
}

impl MemoryFiles {
    pub fn system() -> Self {
        Self {
            meminfo: PathBuf::from("/proc/meminfo"),
            arcstats: PathBuf::from("/proc/spl/kstat/zfs/arcstats"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admission {
    Admit,
    Wait,
}

/// Start a guest only if the guest and the margin fit in `available` bytes.
/// A need too large to count in bytes never fits.
pub fn admit(guest_mem_mib: u64, margin_mib: u64, available: u64) -> Admission {
    let needed = guest_mem_mib
        .checked_add(margin_mib)
        .and_then(|mib| mib.checked_mul(MIB));
    if needed.is_some_and(|needed| needed <= available) {
        Admission::Admit
    } else {
        Admission::Wait
    }
}

/// `MemAvailable` from `/proc/meminfo`, in bytes; `None` when absent or malformed.
pub fn mem_available(meminfo: &str) -> Option<u64> {
    let line = meminfo
        .lines()
        .find(|line| line.starts_with("MemAvailable:"))?;
    let kib: u64 = line.split_whitespace().nth(1)?.parse().ok()?;
    kib.checked_mul(1024)
}

/// ZFS ARC the kernel can give back under pressure, which `MemAvailable` omits.
///
/// Only the evictable lists count, and never below the ARC's floor (`c_min`).
pub fn reclaimable_arc(arcstats: &str) -> u64 {
    let mut stats = std::collections::HashMap::new();
    for line in arcstats.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if let [name, _kind, value] = fields[..]
            && !value.is_empty()
            && value.bytes().all(|b| b.is_ascii_digit())
            && let Ok(value) = value.parse::<u64>()
        {
            stats.insert(name, value);
        }
    }
    let stat = |name: &str| stats.get(name).copied().unwrap_or(0);
    let evictable: u64 = ARC_EVICTABLE.iter().map(|name| stat(name)).sum();
    evictable.min(stat("size").saturating_sub(stat("c_min")))
}

/// Bytes a guest may use now: `MemAvailable` plus reclaimable ARC.
/// A host without ZFS has no arcstats and so no ARC to add.
pub fn available(files: &MemoryFiles) -> Result<u64, Error> {
    let meminfo = read(&files.meminfo)?;
    let mem = mem_available(&meminfo).ok_or_else(|| Error::MemAvailableMissing {
        path: files.meminfo.clone(),
    })?;
    let arc = match std::fs::read_to_string(&files.arcstats) {
        Ok(text) => reclaimable_arc(&text),
        Err(source) if source.kind() == io::ErrorKind::NotFound => 0,
        Err(source) => {
            return Err(Error::Read {
                path: files.arcstats.clone(),
                source,
            });
        }
    };
    Ok(mem.saturating_add(arc))
}

fn read(path: &Path) -> Result<String, Error> {
    std::fs::read_to_string(path).map_err(|source| Error::Read {
        path: path.to_owned(),
        source,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arcstats(size: u64, c_min: u64) -> String {
        let mut lines = vec![
            "13 1 0x01 147 39984 0 0".to_owned(),
            "name                            type data".to_owned(),
        ];
        let values = [
            ("size", size),
            ("c_min", c_min),
            ("mru_evictable_data", MIB),
            ("mru_evictable_metadata", 2 * MIB),
            ("mfu_evictable_data", 3 * MIB),
            ("mfu_evictable_metadata", 4 * MIB),
        ];
        lines.extend(
            values
                .iter()
                .map(|(name, value)| format!("{name:<32}4    {value}")),
        );
        lines.join("\n") + "\n"
    }

    #[test]
    fn guest_and_margin_must_fit_in_available_memory() {
        assert_eq!(admit(12288, 4096, 16384 * MIB), Admission::Admit);
        assert_eq!(admit(12288, 4096, 16384 * MIB - 1), Admission::Wait);
        assert_eq!(admit(u64::MAX / MIB + 1, 0, u64::MAX), Admission::Wait);
    }

    #[test]
    fn reclaimable_arc_counts_evictable_lists_above_the_floor() {
        assert_eq!(reclaimable_arc(&arcstats(100 * MIB, 10 * MIB)), 10 * MIB);
        assert_eq!(reclaimable_arc(&arcstats(14 * MIB, 10 * MIB)), 4 * MIB);
        assert_eq!(reclaimable_arc(&arcstats(5 * MIB, 10 * MIB)), 0);
    }

    #[test]
    fn available_adds_arc_and_treats_absent_arcstats_as_none() {
        let dir = tempfile::tempdir().unwrap();
        let files = MemoryFiles {
            meminfo: dir.path().join("meminfo"),
            arcstats: dir.path().join("absent"),
        };
        std::fs::write(&files.meminfo, "MemTotal: 9 kB\nMemAvailable:    1024 kB\n").unwrap();
        assert_eq!(available(&files).unwrap(), MIB);
        std::fs::write(dir.path().join("arc"), arcstats(100 * MIB, 10 * MIB)).unwrap();
        let with_arc = MemoryFiles {
            arcstats: dir.path().join("arc"),
            ..files
        };
        assert_eq!(available(&with_arc).unwrap(), 11 * MIB);
    }

    #[test]
    fn unreadable_or_incomplete_reports_are_errors_not_zero() {
        let dir = tempfile::tempdir().unwrap();
        let files = MemoryFiles {
            meminfo: dir.path().join("meminfo"),
            arcstats: dir.path().to_owned(),
        };
        std::fs::write(&files.meminfo, "MemTotal: 9 kB\n").unwrap();
        assert!(matches!(
            available(&files),
            Err(Error::MemAvailableMissing { .. })
        ));
        std::fs::write(&files.meminfo, "MemAvailable: 1 kB\n").unwrap();
        assert!(matches!(available(&files), Err(Error::Read { .. })));
    }
}
