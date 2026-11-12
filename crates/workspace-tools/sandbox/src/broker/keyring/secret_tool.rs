//! The real keyring through `secret-tool`, and the `gh auth token` fallback.

use std::ffi::OsString;

use super::command::{self, Finished};
use super::{Fallback, Keyring, KeyringError};
use crate::broker::{Item, Secret};

const SECRET_TOOL: &str = "secret-tool";
const SERVICE: &str = "coda-sandbox";

/// libsecret's `secret-tool`, found on `PATH` unless a path is given.
#[derive(Debug, Clone)]
pub struct SecretTool {
    pub program: OsString,
}

impl Default for SecretTool {
    fn default() -> Self {
        Self {
            program: SECRET_TOOL.into(),
        }
    }
}

impl SecretTool {
    fn run(&self, args: &[&str], stdin: Option<&[u8]>) -> Result<Finished, KeyringError> {
        command::run(SECRET_TOOL, &self.program, args, stdin)
    }
}

fn failed(program: &'static str, finished: Finished) -> KeyringError {
    KeyringError::Failed {
        program,
        status: finished.status,
        stderr: finished.stderr,
    }
}

impl Keyring for SecretTool {
    fn lookup(&self, item: Item) -> Result<Option<Secret>, KeyringError> {
        let finished = self.run(&["lookup", "service", SERVICE, "token", item.name()], None)?;
        if finished.status.success() {
            let text = String::from_utf8_lossy(&finished.stdout);
            return Secret::new(&text)
                .map(Some)
                .map_err(|source| KeyringError::NotAToken {
                    program: SECRET_TOOL,
                    source,
                });
        }
        // `secret-tool lookup` exits 1 silently when no item matches; any
        // message on stderr means the keyring itself did not answer.
        if finished.status.code() == Some(1)
            && finished.stdout.is_empty()
            && finished.stderr.is_empty()
        {
            return Ok(None);
        }
        Err(failed(SECRET_TOOL, finished))
    }

    fn store(&self, item: Item, secret: &Secret) -> Result<(), KeyringError> {
        let label = format!("coda-sandbox {item}");
        let finished = self.run(
            &[
                "store",
                "--label",
                &label,
                "service",
                SERVICE,
                "token",
                item.name(),
            ],
            Some(secret.expose().as_bytes()),
        )?;
        if finished.status.success() {
            Ok(())
        } else {
            Err(failed(SECRET_TOOL, finished))
        }
    }

    fn delete(&self, item: Item) -> Result<(), KeyringError> {
        let finished = self.run(&["clear", "service", SERVICE, "token", item.name()], None)?;
        // `clear` exits 1 silently when nothing matched, which is the goal.
        let nothing_matched = finished.status.code() == Some(1) && finished.stderr.is_empty();
        if finished.status.success() || nothing_matched {
            Ok(())
        } else {
            Err(failed(SECRET_TOOL, finished))
        }
    }
}

/// `gh auth token`, found on `PATH`.
#[derive(Debug, Clone)]
pub struct GhCli {
    pub program: OsString,
}

impl Default for GhCli {
    fn default() -> Self {
        Self {
            program: "gh".into(),
        }
    }
}

impl Fallback for GhCli {
    fn gh_token(&self) -> Result<Secret, KeyringError> {
        let finished = command::run("gh", &self.program, &["auth", "token"], None)?;
        if !finished.status.success() {
            return Err(failed("gh", finished));
        }
        Secret::new(&String::from_utf8_lossy(&finished.stdout)).map_err(|source| {
            KeyringError::NotAToken {
                program: "gh",
                source,
            }
        })
    }
}
