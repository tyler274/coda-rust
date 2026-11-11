//! The controller's settings, read from the environment the NixOS unit sets.

use std::path::{Path, PathBuf};

use super::disk::DiskSize;
use super::error::Error;
use super::github::DEFAULT_API;
use super::memory::MIB;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControllerConfig {
    /// `CODA_CI_ROOT`: holds the seed, jobs, spool, logs and the lock.
    pub root: PathBuf,
    /// `CODA_CI_RUNNER`: the program that boots one guest in its working directory.
    pub runner: PathBuf,
    /// `CODA_CI_GUEST_MEM_MIB`.
    pub guest_mem_mib: u64,
    /// `CODA_CI_MARGIN_MIB`, default 4096.
    pub margin_mib: u64,
    /// `CODA_CI_MAX_BUNDLE_MIB` in bytes, default 4096 MiB.
    pub max_bundle_bytes: u64,
    /// `CODA_CI_WORK_DISK_SIZE`, as `truncate -s` reads it; default `200G`.
    pub work_disk_size: DiskSize,
    /// `CODA_CI_API`, default the public GitHub API.
    pub api: String,
}

impl ControllerConfig {
    pub fn from_env() -> Result<Self, Error> {
        Self::from_lookup(|name| std::env::var(name).ok())
    }

    pub fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Result<Self, Error> {
        let required = |name: &'static str| lookup(name).ok_or(Error::EnvMissing { name });
        let number = |name: &'static str, value: String| {
            value
                .trim()
                .parse::<u64>()
                .map_err(|source| Error::EnvInvalid {
                    name,
                    value,
                    source,
                })
        };
        let optional =
            |name: &'static str, default: &str| lookup(name).unwrap_or_else(|| default.to_owned());
        let root = required("CODA_CI_ROOT")?.into();
        let runner = required("CODA_CI_RUNNER")?.into();
        let guest_mem_mib = number("CODA_CI_GUEST_MEM_MIB", required("CODA_CI_GUEST_MEM_MIB")?)?;
        let margin_mib = number("CODA_CI_MARGIN_MIB", optional("CODA_CI_MARGIN_MIB", "4096"))?;
        let bundle_mib = number(
            "CODA_CI_MAX_BUNDLE_MIB",
            optional("CODA_CI_MAX_BUNDLE_MIB", "4096"),
        )?;
        let max_bundle_bytes = bundle_mib.checked_mul(MIB).ok_or(Error::EnvTooLarge {
            name: "CODA_CI_MAX_BUNDLE_MIB",
            mib: bundle_mib,
        })?;
        Ok(Self {
            root,
            runner,
            guest_mem_mib,
            margin_mib,
            max_bundle_bytes,
            work_disk_size: optional("CODA_CI_WORK_DISK_SIZE", "200G").parse().map_err(
                |source| Error::EnvDiskSize {
                    name: "CODA_CI_WORK_DISK_SIZE",
                    source,
                },
            )?,
            api: optional("CODA_CI_API", DEFAULT_API),
        })
    }

    pub fn seed(&self) -> PathBuf {
        self.root.join("seed").join("work.img")
    }

    pub fn jobs(&self) -> PathBuf {
        self.root.join("jobs")
    }

    pub fn requests(&self) -> PathBuf {
        self.root.join("spool").join("requests")
    }

    pub fn results(&self) -> PathBuf {
        self.root.join("spool").join("results")
    }

    pub fn logs(&self) -> PathBuf {
        self.root.join("logs")
    }

    pub fn lock(&self) -> PathBuf {
        self.root.join("lock")
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_units_variables_with_the_python_defaults() {
        let config = ControllerConfig::from_lookup(|name| match name {
            "CODA_CI_ROOT" => Some("/agents/coda/ci".into()),
            "CODA_CI_RUNNER" => Some("/nix/store/x/bin/microvm-run".into()),
            "CODA_CI_GUEST_MEM_MIB" => Some("12288".into()),
            _ => None,
        })
        .unwrap();
        assert_eq!(config.margin_mib, 4096);
        assert_eq!(config.max_bundle_bytes, 4096 * MIB);
        assert_eq!(config.work_disk_size, "200G".parse().unwrap());
        assert_eq!(config.api, DEFAULT_API);
        assert_eq!(
            config.requests(),
            Path::new("/agents/coda/ci/spool/requests")
        );
    }

    #[test]
    fn missing_or_malformed_numbers_are_errors() {
        let missing = ControllerConfig::from_lookup(|_| None);
        assert!(matches!(
            missing,
            Err(Error::EnvMissing {
                name: "CODA_CI_ROOT"
            })
        ));
        let bad = ControllerConfig::from_lookup(|name| match name {
            "CODA_CI_GUEST_MEM_MIB" => Some("lots".into()),
            _ => Some("/x".into()),
        });
        assert!(matches!(
            bad,
            Err(Error::EnvInvalid {
                name: "CODA_CI_GUEST_MEM_MIB",
                ..
            })
        ));
        let huge = ControllerConfig::from_lookup(|name| match name {
            "CODA_CI_ROOT" | "CODA_CI_RUNNER" => Some("/x".into()),
            "CODA_CI_GUEST_MEM_MIB" => Some("1".into()),
            "CODA_CI_MAX_BUNDLE_MIB" => Some(u64::MAX.to_string()),
            _ => None,
        });
        assert!(matches!(
            huge,
            Err(Error::EnvTooLarge {
                name: "CODA_CI_MAX_BUNDLE_MIB",
                ..
            })
        ));
    }
}
