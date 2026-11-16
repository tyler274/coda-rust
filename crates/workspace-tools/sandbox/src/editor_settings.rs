//! The editors that open the sandbox from the host, and the settings that
//! would hand the host's credentials to the container over their remote.
//! Each such setting must be explicitly off, since the editors default it on.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Error;

/// Named as `--editor` and the config's `[open] editor` take it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Editor {
    #[default]
    Cursor,
    #[value(name = "code")]
    #[serde(rename = "code")]
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

    /// The name `--editor` takes.
    pub fn value_name(self) -> &'static str {
        match self {
            Self::Cursor => "cursor",
            Self::VsCode => "code",
            Self::Antigravity => "antigravity",
            Self::Zed => "zed",
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
        let (app, keys, extensions): (_, &'static [ForwardingKey], _) = match self {
            Self::Cursor => ("Cursor", &ForwardingKey::ALL, RemoteExtensions::Setting),
            Self::VsCode => ("Code", &ForwardingKey::ALL, RemoteExtensions::Setting),
            // Its SSH extension has no agent-forwarding key and follows the
            // SSH config's ForwardAgent, and it has no default-extensions key.
            Self::Antigravity => (
                "Antigravity IDE",
                &[ForwardingKey::GitTerminal, ForwardingKey::GithubGit],
                RemoteExtensions::Server(RemoteServer::ANTIGRAVITY),
            ),
            Self::Zed => return None,
        };
        Some(SettingsFile {
            app,
            keys,
            extensions,
        })
    }
}

/// The extensions the container side of the VS Code family needs: direnv
/// loads the checkout's dev shell, and rust-analyzer then runs the
/// flake's server from it.
pub const REMOTE_EXTENSIONS: [&str; 2] = ["mkhl.direnv", "rust-lang.rust-analyzer"];

/// The key that installs extensions on a remote server when it starts.
pub const DEFAULT_EXTENSIONS_KEY: &str = "remote.SSH.defaultExtensions";

/// How the container side of an editor gets [`REMOTE_EXTENSIONS`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteExtensions {
    /// [`DEFAULT_EXTENSIONS_KEY`] in the user settings installs them.
    Setting,
    /// Nothing in the editor does; `coda-sandbox open` installs them into
    /// this server once the editor has put it in the container.
    Server(RemoteServer),
}

/// An editor's server in the container, which the editor installs under
/// the agent's home on its first connection. Every version it installs
/// shares one extensions directory, so any version's command lists and
/// installs them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemoteServer {
    /// The directory under the agent's home.
    dir: &'static str,
    /// The command in each version's `bin/<commit>/bin`.
    command: &'static str,
}

impl RemoteServer {
    pub const ANTIGRAVITY: Self = Self {
        dir: ".antigravity-ide-server",
        command: "antigravity-ide-server",
    };

    /// How [`Self::list_line`] and [`Self::install_line`] exit when the
    /// editor has not installed its server yet.
    pub const ABSENT_EXIT: i32 = 3;

    /// A shell line that runs the server's command with `args`.
    fn line(self, args: &str) -> String {
        format!(
            "set -- \"$HOME\"/{}/bin/*/bin/{}; [ -x \"$1\" ] || exit {}; exec \"$1\" {args}",
            self.dir,
            self.command,
            Self::ABSENT_EXIT
        )
    }

    /// Prints the installed extension ids, one per line.
    pub fn list_line(self) -> String {
        self.line("--list-extensions")
    }

    pub fn install_line(self, ids: &[&str]) -> String {
        let args: Vec<String> = ids
            .iter()
            .map(|id| format!("--install-extension {}", crate::open::shell_quote(id)))
            .collect();
        self.line(&args.join(" "))
    }
}

/// What a remote server holds of [`REMOTE_EXTENSIONS`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerExtensions {
    /// The editor has not connected and installed its server yet.
    NoServer,
    /// The server is there; these are the ones it lacks, possibly none.
    Missing(Vec<&'static str>),
}

impl ServerExtensions {
    /// Reads what [`RemoteServer::list_line`] exited with and printed;
    /// `None` for any other exit, which the caller reports as a failure.
    pub fn read(exit_code: Option<i32>, stdout: &str) -> Option<Self> {
        match exit_code {
            Some(0) => Some(Self::Missing(missing_from(stdout.lines().map(str::trim)))),
            Some(RemoteServer::ABSENT_EXIT) => Some(Self::NoServer),
            _ => None,
        }
    }
}

/// The [`REMOTE_EXTENSIONS`] not among `listed`. Extension ids compare
/// without case, as the editors do.
fn missing_from<'a>(listed: impl Iterator<Item = &'a str>) -> Vec<&'static str> {
    let listed: Vec<&str> = listed.collect();
    REMOTE_EXTENSIONS
        .into_iter()
        .filter(|want| !listed.iter().any(|id| id.eq_ignore_ascii_case(want)))
        .collect()
}

/// An editor's user settings file and the forwarding keys it must turn off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingsFile {
    /// The directory under `~/.config`.
    app: &'static str,
    pub keys: &'static [ForwardingKey],
    pub extensions: RemoteExtensions,
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

    /// The [`REMOTE_EXTENSIONS`] that [`DEFAULT_EXTENSIONS_KEY`] does not
    /// list.
    pub fn missing_remote_extensions(&self) -> Vec<&'static str> {
        match self.0.get(DEFAULT_EXTENSIONS_KEY) {
            Some(Value::Array(ids)) => missing_from(ids.iter().filter_map(Value::as_str)),
            _ => REMOTE_EXTENSIONS.to_vec(),
        }
    }
}
