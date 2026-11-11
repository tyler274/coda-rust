//! The sandbox's machine description, read from TOML. The NixOS module will
//! write it; until then `--config crates/workspace-tools/sandbox/config/cyrene.toml` points at it.

use std::fmt;
use std::io;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::Error;

pub const DEFAULT_PATH: &str = "/etc/coda-sandbox/config.toml";

/// One machine's sandbox layout, read by the binary on the host and inside
/// the agent container alike.
///
/// **Every path here names the same directory on the host and in the
/// container.** The container's bind mounts keep host paths, so neither side
/// translates a path, and one file serves both.
///
/// Which side reads what:
/// - host only: [`Container`] (the container unit, its SSH alias and the
///   agent's UID) and [`Ci::timer`] and [`Ci::service`];
/// - both: [`Build`], the CI spool ([`Ci::spool`]) and [`Container::repo`].
///
/// Keys this binary does not know are kept as [`Loaded::unknown_keys`] rather
/// than rejected, so a binary older than the file still reads it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Config {
    pub version: Version,
    pub container: Container,
    pub build: Build,
    pub ci: Ci,
    /// Only the host that carries its worktrees into the sandbox has one.
    #[serde(default)]
    pub carry: Option<Carry>,
}

/// `coda-sandbox carry`: which checkout to record and where the bundle goes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Carry {
    /// The host's main checkout; all its worktrees are carried.
    pub repo: PathBuf,
    /// Receives `carry.bundle`, `ignored.txt` and `summary.txt`; the
    /// container binds it read-only.
    pub out: PathBuf,
}

/// The config schema's version, `version = 1` in the file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(try_from = "u32")]
pub enum Version {
    V1,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("config version {0} is not one this binary reads (it reads 1)")]
pub struct UnsupportedVersion(u32);

impl TryFrom<u32> for Version {
    type Error = UnsupportedVersion;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::V1),
            other => Err(UnsupportedVersion(other)),
        }
    }
}

/// The agent container and how the host reaches it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Container {
    pub name: String,
    pub unit: UnitName,
    /// The `Host` alias in the user's SSH config.
    pub ssh_host: String,
    /// The host UID that owns the agent's files.
    pub agent_uid: u32,
    /// The repository checkout the agent works in.
    pub repo: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Build {
    /// Where worktree build output lives; it must support reflinks. Seeds go
    /// under `seed/`, worktree targets under `wt/`.
    pub root: PathBuf,
    /// How many seeds `worktree seed` keeps, newest first; at least the one
    /// just built.
    #[serde(default = "default_keep_seeds")]
    pub keep_seeds: NonZeroUsize,
}

fn default_keep_seeds() -> NonZeroUsize {
    NonZeroUsize::MIN.saturating_add(1)
}

fn default_timeout_seconds() -> u64 {
    4 * 3600
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Ci {
    /// Holds `requests/` and `results/`.
    pub spool: PathBuf,
    /// How long `ci run` waits for a result.
    #[serde(default = "default_timeout_seconds")]
    pub timeout_seconds: u64,
    pub timer: UnitName,
    pub service: UnitName,
}

impl Ci {
    pub fn requests(&self) -> PathBuf {
        self.spool.join("requests")
    }

    pub fn results(&self) -> PathBuf {
        self.spool.join("results")
    }
}

/// A config together with the keys in its file that this binary ignored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loaded {
    pub config: Config,
    /// Dotted paths such as `ci.queue`, sorted.
    pub unknown_keys: Vec<String>,
}

impl Config {
    /// Reads the config, ignoring keys this binary does not know.
    pub fn load(path: &Path) -> Result<Self, Error> {
        Self::load_with_unknown_keys(path).map(|loaded| loaded.config)
    }

    /// Reads the config and reports the keys it ignored, for `doctor`.
    pub fn load_with_unknown_keys(path: &Path) -> Result<Loaded, Error> {
        let text = std::fs::read_to_string(path).map_err(|source| match source.kind() {
            io::ErrorKind::NotFound => Error::ConfigMissing {
                path: path.to_owned(),
                source,
            },
            _ => Error::ConfigRead {
                path: path.to_owned(),
                source,
            },
        })?;
        Self::parse_with_unknown_keys(path, &text)
    }

    pub fn parse(path: &Path, text: &str) -> Result<Self, Error> {
        Self::parse_with_unknown_keys(path, text).map(|loaded| loaded.config)
    }

    pub fn parse_with_unknown_keys(path: &Path, text: &str) -> Result<Loaded, Error> {
        let parse_error = |source: toml::de::Error| Error::ConfigParse {
            path: path.to_owned(),
            source: Box::new(source),
        };
        let deserializer = toml::Deserializer::parse(text).map_err(parse_error)?;
        let mut unknown_keys = Vec::new();
        let config = serde_ignored::deserialize(deserializer, |key| {
            unknown_keys.push(key.to_string());
        })
        .map_err(parse_error)?;
        unknown_keys.sort();
        Ok(Loaded {
            config,
            unknown_keys,
        })
    }
}

/// What a unit is, from its name's suffix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitKind {
    Service,
    Timer,
    Socket,
    Path,
    Target,
}

impl UnitKind {
    const ALL: [Self; 5] = [
        Self::Service,
        Self::Timer,
        Self::Socket,
        Self::Path,
        Self::Target,
    ];

    pub fn suffix(self) -> &'static str {
        match self {
            Self::Service => ".service",
            Self::Timer => ".timer",
            Self::Socket => ".socket",
            Self::Path => ".path",
            Self::Target => ".target",
        }
    }
}

/// A systemd unit name with its type suffix, such as `coda-ci.timer`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct UnitName {
    name: String,
    kind: UnitKind,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{0:?} is not a systemd unit name ending in .service, .timer, .socket, .path or .target")]
pub struct ParseUnitNameError(String);

impl UnitName {
    pub fn as_str(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> UnitKind {
        self.kind
    }
}

impl FromStr for UnitName {
    type Err = ParseUnitNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let plain =
            |c: char| c.is_ascii_alphanumeric() || matches!(c, '@' | ':' | '-' | '_' | '.' | '\\');
        let parsed = UnitKind::ALL.into_iter().find_map(|kind| {
            let stem = value.strip_suffix(kind.suffix())?;
            (!stem.is_empty() && stem.chars().all(plain)).then_some(kind)
        });
        match parsed {
            Some(kind) => Ok(Self {
                name: value.to_owned(),
                kind,
            }),
            None => Err(ParseUnitNameError(value.to_owned())),
        }
    }
}

impl TryFrom<String> for UnitName {
    type Error = ParseUnitNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<UnitName> for String {
    fn from(unit: UnitName) -> Self {
        unit.name
    }
}

impl fmt::Display for UnitName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}
