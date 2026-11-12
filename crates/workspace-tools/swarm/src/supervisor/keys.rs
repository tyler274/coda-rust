//! API keys for member turns. A key only ever goes into the child env; it
//! is never formatted into errors, logs or `Debug` output.

use std::fmt;
use std::path::PathBuf;
use std::process::{Command, Stdio};

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
    Unavailable { item: String, status: String },
    #[error("key item {item} is empty")]
    Empty { item: String },
}

pub trait KeySource: Send + Sync {
    fn get(&self, item: &str) -> Result<Secret, KeyError>;
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
    fn get(&self, item: &str) -> Result<Secret, KeyError> {
        // Stderr is dropped rather than quoted: it could echo the key.
        let output = Command::new(&self.program)
            .args(["token", item])
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .map_err(|error| KeyError::Start {
                program: self.program.display().to_string(),
                reason: error.to_string(),
            })?;
        if !output.status.success() {
            return Err(KeyError::Unavailable {
                item: item.to_owned(),
                status: output.status.to_string(),
            });
        }
        let key = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        if key.is_empty() {
            return Err(KeyError::Empty {
                item: item.to_owned(),
            });
        }
        Ok(Secret(key))
    }
}
