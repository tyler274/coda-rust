//! The guest's two raw ext4 disks: the work disk and the read-only inputs.

use std::fmt;
use std::fs::{self, File};
use std::path::Path;
use std::process::{Command, Stdio};
use std::str::FromStr;

use super::error::Error;
use super::memory::MIB;

/// A disk's apparent size, as `truncate -s` reads it with binary suffixes:
/// `200G`, `512M`, `1T`, or plain bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiskSize(u64);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{0:?} is not a disk size such as 200G (bytes, or K, M, G, T, P or E)")]
pub struct ParseDiskSizeError(String);

impl DiskSize {
    pub fn bytes(self) -> u64 {
        self.0
    }
}

impl FromStr for DiskSize {
    type Err = ParseDiskSizeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let error = || ParseDiskSizeError(value.to_owned());
        let text = value.strip_suffix("iB").unwrap_or(value);
        let split = text
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(text.len());
        let (digits, suffix) = text.split_at(split);
        let power = match suffix {
            "" => 0,
            "K" => 1,
            "M" => 2,
            "G" => 3,
            "T" => 4,
            "P" => 5,
            "E" => 6,
            _ => return Err(error()),
        };
        // `digits` holds only ASCII digits, so parsing fails only when it is
        // empty or too large, which the message's value already shows.
        digits
            .parse::<u64>()
            .ok()
            .zip(1024u64.checked_pow(power))
            .and_then(|(count, unit)| count.checked_mul(unit))
            .map(Self)
            .ok_or_else(error)
    }
}

impl fmt::Display for DiskSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} bytes", self.0)
    }
}

/// How the controller makes a job's disks. `SystemDisks` is the machine's
/// reflinks and `mkfs.ext4`; tests that need neither substitute their own.
pub trait Disks {
    fn clone_seed(&self, seed: &Path, destination: &Path) -> Result<(), Error>;
    fn make_work(&self, path: &Path, size: DiskSize) -> Result<(), Error>;
    fn make_inputs(&self, path: &Path, source: &Path) -> Result<(), Error>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemDisks;

impl Disks for SystemDisks {
    fn clone_seed(&self, seed: &Path, destination: &Path) -> Result<(), Error> {
        clone_seed(seed, destination)
    }

    fn make_work(&self, path: &Path, size: DiskSize) -> Result<(), Error> {
        make_disk(path, size.bytes(), None, "work")
    }

    fn make_inputs(&self, path: &Path, source: &Path) -> Result<(), Error> {
        make_inputs(path, source)
    }
}

/// Give the job a reflink clone of the seed work disk.
pub fn clone_seed(seed: &Path, destination: &Path) -> Result<(), Error> {
    let source = match File::open(seed) {
        Ok(file) if file.metadata().is_ok_and(|meta| meta.is_file()) => file,
        Ok(_) => {
            return Err(Error::SeedMissing {
                path: seed.to_owned(),
            });
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(Error::SeedMissing {
                path: seed.to_owned(),
            });
        }
        Err(error) => return Err(Error::io("open", seed)(error)),
    };
    let target = File::create(destination).map_err(Error::io("create", destination))?;
    rustix::fs::ioctl_ficlone(&target, &source)
        .map_err(|errno| Error::io("reflink-clone the seed into", destination)(errno.into()))
}

/// A sparse ext4 image of `size` bytes, filled from `source` when given.
pub fn make_disk(path: &Path, size: u64, source: Option<&Path>, label: &str) -> Result<(), Error> {
    File::create(path)
        .and_then(|file| file.set_len(size))
        .map_err(Error::io("size", path))?;
    let mut command = Command::new("mkfs.ext4");
    command.args(["-q", "-F", "-m", "0", "-L", label]);
    if let Some(source) = source {
        command.arg("-d").arg(source);
    }
    command.arg(path).stdin(Stdio::null());
    let status = command.status().map_err(|source| Error::Spawn {
        program: "mkfs.ext4".to_owned(),
        source,
    })?;
    if !status.success() {
        return Err(Error::CommandFailed {
            program: "mkfs.ext4".to_owned(),
            status,
        });
    }
    Ok(())
}

/// The inputs disk: the payload, a tenth more, and 32 MiB for ext4 itself.
pub fn make_inputs(path: &Path, source: &Path) -> Result<(), Error> {
    let payload = payload_size(source)?;
    make_disk(
        path,
        payload + payload / 10 + 32 * MIB,
        Some(source),
        "inputs",
    )
}

fn payload_size(directory: &Path) -> Result<u64, Error> {
    let mut total = 0;
    for entry in fs::read_dir(directory).map_err(Error::io("list", directory))? {
        let path = entry.map_err(Error::io("list", directory))?.path();
        let meta = fs::metadata(&path).map_err(Error::io("inspect", &path))?;
        if meta.is_dir() {
            total += payload_size(&path)?;
        } else if meta.is_file() {
            total += meta.len();
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disk_sizes_read_like_truncate() {
        assert_eq!(
            "200G".parse::<DiskSize>().unwrap().bytes(),
            200 * 1024 * MIB
        );
        assert_eq!("1GiB".parse::<DiskSize>().unwrap().bytes(), 1024 * MIB);
        assert_eq!("4096".parse::<DiskSize>().unwrap().bytes(), 4096);
        for bad in ["", "G", "1.5G", "1X", "-1G", "99999999999E"] {
            assert!(bad.parse::<DiskSize>().is_err(), "{bad:?}");
        }
    }

    #[test]
    fn missing_seed_names_the_seed_unit() {
        let dir = tempfile::tempdir().unwrap();
        let error = clone_seed(&dir.path().join("work.img"), &dir.path().join("x")).unwrap_err();
        assert!(error.to_string().contains("coda-ci-seed"), "{error}");
        assert!(!dir.path().join("x").exists());
    }
}
