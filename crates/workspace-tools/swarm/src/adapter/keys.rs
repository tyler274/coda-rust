//! Provider keys for HTTP models. A key stays in the adapter's process: it
//! is sent as a request header and never put in a child env, error, log or
//! `Debug` output.

use std::fmt;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};

/// A token broker item an HTTP model authenticates with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum KeyItem {
    /// The host model proxy's client key, valid for every model it routes.
    #[serde(rename = "model-proxy")]
    ModelProxy,
}

impl KeyItem {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ModelProxy => "model-proxy",
        }
    }
}

impl fmt::Display for KeyItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A key value. `Debug` and errors never show it.
#[derive(Clone, PartialEq, Eq)]
pub struct Secret(String);

impl Secret {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret(<redacted>)")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum KeyError {
    #[error("start {program}: {reason}")]
    Start { program: String, reason: String },
    #[error("key item {item} is unavailable ({status})")]
    Unavailable { item: KeyItem, status: String },
    #[error("key item {item} is empty")]
    Empty { item: KeyItem },
}

pub trait KeySource: Send + Sync + fmt::Debug {
    fn get(&self, item: KeyItem) -> Result<Secret, KeyError>;
}

/// Keys from `coda-sandbox token <item>`, which prints the key on stdout.
#[derive(Debug, Clone)]
pub struct SandboxKeys {
    pub program: PathBuf,
}

impl Default for SandboxKeys {
    fn default() -> Self {
        Self {
            program: PathBuf::from("coda-sandbox"),
        }
    }
}

impl KeySource for SandboxKeys {
    fn get(&self, item: KeyItem) -> Result<Secret, KeyError> {
        // Stderr is dropped rather than quoted: it could echo the key.
        let output = Command::new(&self.program)
            .args(["token", item.as_str()])
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .map_err(|error| KeyError::Start {
                program: self.program.display().to_string(),
                reason: error.to_string(),
            })?;
        if !output.status.success() {
            return Err(KeyError::Unavailable {
                item,
                status: output.status.to_string(),
            });
        }
        let key = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        if key.is_empty() {
            return Err(KeyError::Empty { item });
        }
        Ok(Secret(key))
    }
}
