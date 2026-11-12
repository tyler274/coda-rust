//! The editors that open the sandbox from the host, and the settings that
//! would hand the host's credentials to the container over their remote.
//! Each such setting must be explicitly off, since the editors default it on.

use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::Value;

use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Editor {
    Cursor,
    #[value(name = "code")]
    VsCode,
    Antigravity,
    Zed,
}

/// How an editor reaches the container.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Remote {
    /// VS Code's Remote-SSH, or the fork's own extension that answers the
    /// same `ssh-remote` authority.
    SshRemote,
    /// Zed's `ssh://` project URL.
    ZedSsh,
}

impl Editor {
    pub const ALL: [Self; 4] = [Self::Cursor, Self::VsCode, Self::Antigravity, Self::Zed];

    pub fn name(self) -> &'static str {
        match self {
            Self::Cursor => "Cursor",
            Self::VsCode => "VS Code",
            Self::Antigravity => "Antigravity",
            Self::Zed => "Zed",
        }
    }

    pub fn remote(self) -> Remote {
        match self {
            Self::Cursor | Self::VsCode | Self::Antigravity => Remote::SshRemote,
            Self::Zed => Remote::ZedSsh,
        }
    }

    /// The command-line launcher. Nixpkgs names Zed's `zeditor`, and `zed`
    /// is never tried, since on a ZFS host it is the ZFS event daemon. The
    /// Antigravity IDE is `antigravity-ide`, apart from the agent hub.
    pub fn program(self) -> &'static str {
        match self {
            Self::Cursor => "cursor",
            Self::VsCode => "code",
            Self::Antigravity => "antigravity-ide",
            Self::Zed => "zeditor",
        }
    }

    /// The settings file with keys that forward host credentials, for the
    /// editors that have such keys. Zed has none: it forwards only what the
    /// SSH config does, which doctor checks on its own.
    pub fn settings(self) -> Option<SettingsFile> {
        let (app, keys): (_, &'static [ForwardingKey]) = match self {
            Self::Cursor => ("Cursor", &ForwardingKey::ALL),
            Self::VsCode => ("Code", &ForwardingKey::ALL),
            // Its SSH extension has no agent-forwarding key and follows the
            // SSH config's ForwardAgent.
            Self::Antigravity => (
                "Antigravity IDE",
                &[ForwardingKey::GitTerminal, ForwardingKey::GithubGit],
            ),
            Self::Zed => return None,
        };
        Some(SettingsFile { app, keys })
    }
}

/// An editor's user settings file and the forwarding keys it must turn off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingsFile {
    /// The directory under `~/.config`.
    app: &'static str,
    pub keys: &'static [ForwardingKey],
}

impl SettingsFile {
    pub fn path(self, home: &Path) -> PathBuf {
        home.join(".config")
            .join(self.app)
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
