use super::tools::{Exit, Invocation, Program, Runner, Stage};
use super::{Diagnostics, Error, Result, artifacts::PlanDir};
use std::{fmt, time::Duration};

pub(super) const DOPPLER_PROJECT: &str = "coda-rust";
const LOOKUP_TIMEOUT: Duration = Duration::from_secs(60);

/// The four application database users whose passwords the staging root writes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Role {
    PhysicianRuntime,
    PhysicianMigration,
    DuetRuntime,
    DuetMigration,
}
impl Role {
    pub const ALL: [Self; 4] = [
        Self::PhysicianRuntime,
        Self::PhysicianMigration,
        Self::DuetRuntime,
        Self::DuetMigration,
    ];
    /// Key in the root's `database_passwords` variable.
    pub fn key(self) -> &'static str {
        match self {
            Self::PhysicianRuntime => "physician-runtime",
            Self::PhysicianMigration => "physician-migration",
            Self::DuetRuntime => "duet-runtime",
            Self::DuetMigration => "duet-migration",
        }
    }
    pub fn doppler_config(self) -> &'static str {
        match self {
            Self::PhysicianRuntime => "stg_physician_runtime",
            Self::PhysicianMigration => "stg_physician_migration",
            Self::DuetRuntime => "stg_duet_runtime",
            Self::DuetMigration => "stg_duet_migration",
        }
    }
}
impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.key())
    }
}

#[derive(Debug, thiserror::Error)]
pub(super) enum CredentialFailure {
    #[error("doppler exited with {0:?}")]
    Lookup(Option<i32>),
    #[error("doppler exceeded its bound")]
    Timeout,
    #[error("the value is empty")]
    Empty,
    #[error("the value is not UTF-8: {0}")]
    Encoding(#[source] std::str::Utf8Error),
}

/// A credential value. It has no `Display` and its `Debug` is redacted, so it
/// cannot reach argv, receipts, or terminal output by formatting.
pub(super) struct Secret(String);
impl Secret {
    pub fn expose(&self) -> &str {
        &self.0
    }
    #[cfg(test)]
    pub fn for_test(value: &str) -> Self {
        Self(value.into())
    }
}
impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret(<redacted>)")
    }
}

/// `TF_VAR_database_passwords`, serialized in memory for the child environment.
pub(super) fn fetch(runner: &mut impl Runner, dir: &PlanDir) -> Result<Secret> {
    let mut passwords = serde_json::Map::new();
    for role in Role::ALL {
        let invocation = Invocation::new(
            Program::Doppler,
            Stage::Credentials,
            [
                "secrets",
                "get",
                "DB_PASSWORD",
                "--plain",
                "--project",
                DOPPLER_PROJECT,
                "--config",
                role.doppler_config(),
            ],
            LOOKUP_TIMEOUT,
        );
        let output = runner.run(&invocation, None)?;
        let failure = match output.exit {
            Exit::Success => None,
            Exit::Timeout => Some(CredentialFailure::Timeout),
            Exit::Code(code) => Some(CredentialFailure::Lookup(code)),
        };
        if let Some(reason) = failure {
            // stdout would be the secret; only stderr is retained.
            let diagnostics = dir.append_log(Stage::Credentials, &output, false).into();
            return Err(Error::Credential {
                role,
                reason,
                diagnostics,
            });
        }
        let value = std::str::from_utf8(&output.stdout)
            .map_err(|source| Error::Credential {
                role,
                reason: CredentialFailure::Encoding(source),
                diagnostics: Diagnostics::None,
            })?
            .trim_end_matches(['\r', '\n'])
            .to_owned();
        if value.trim().is_empty() {
            return Err(Error::Credential {
                role,
                reason: CredentialFailure::Empty,
                diagnostics: Diagnostics::None,
            });
        }
        passwords.insert(role.key().into(), value.into());
    }
    Ok(Secret(serde_json::Value::Object(passwords).to_string()))
}
