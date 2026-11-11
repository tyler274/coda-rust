//! Editor settings that would hand the host's credentials to the container
//! over Remote-SSH. Each must be explicitly off, since the editors default it on.

use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::Value;

use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Editor {
    Cursor,
    VsCode,
}

impl Editor {
    pub const ALL: [Self; 2] = [Self::Cursor, Self::VsCode];

    pub fn name(self) -> &'static str {
        match self {
            Self::Cursor => "Cursor",
            Self::VsCode => "VS Code",
        }
    }

    /// The user settings file, relative to the home directory.
    pub fn settings_path(self, home: &Path) -> PathBuf {
        let app = match self {
            Self::Cursor => "Cursor",
            Self::VsCode => "Code",
        };
        home.join(".config")
            .join(app)
            .join("User")
            .join("settings.json")
    }
}

/// The settings that forward host credentials, all on by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ForwardingKey {
    #[serde(rename = "remote.SSH.enableAgentForwarding")]
    SshAgent,
    #[serde(rename = "git.terminalAuthentication")]
    GitTerminal,
    #[serde(rename = "github.gitAuthentication")]
    GithubGit,
}

impl ForwardingKey {
    pub const ALL: [Self; 3] = [Self::SshAgent, Self::GitTerminal, Self::GithubGit];

    pub fn key(self) -> &'static str {
        match self {
            Self::SshAgent => "remote.SSH.enableAgentForwarding",
            Self::GitTerminal => "git.terminalAuthentication",
            Self::GithubGit => "github.gitAuthentication",
        }
    }
}

/// How one forwarding setting reads in the file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Setting {
    Off,
    On,
    /// Not set, so the editor's default (on) applies.
    Absent,
    NotBoolean(Value),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings(serde_json::Map<String, Value>);

impl Settings {
    /// Parses a settings file: JSON with comments and trailing commas.
    pub fn parse(path: &Path, text: &str) -> Result<Self, Error> {
        let value: Value = json5::from_str(text).map_err(|source| Error::SettingsParse {
            path: path.to_owned(),
            source,
        })?;
        match value {
            Value::Object(map) => Ok(Self(map)),
            other => Err(Error::UnexpectedOutput {
                what: "top-level object",
                source_name: path.display().to_string(),
                found: other.to_string(),
            }),
        }
    }

    pub fn get(&self, key: ForwardingKey) -> Setting {
        match self.0.get(key.key()) {
            None => Setting::Absent,
            Some(Value::Bool(false)) => Setting::Off,
            Some(Value::Bool(true)) => Setting::On,
            Some(other) => Setting::NotBoolean(other.clone()),
        }
    }
}
