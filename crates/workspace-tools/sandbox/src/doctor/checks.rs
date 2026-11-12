use std::path::Path;

use serde::Serialize;

use super::{Check, Finding, Fix, Outcome, SpoolDir, broker, reflink};
use crate::config::{Config, Loaded, UnitName};
use crate::editor_settings::{
    DEFAULT_EXTENSIONS_KEY, Editor, ForwardingKey, RemoteExtensions, Setting, Settings,
    SettingsFile,
};
use crate::error::Error;
use crate::probe::{
    ActiveState, ForwardAgent, LoadState, PathKind, Probe, SSH_DEADLINE, SshReach, UnitScope,
};

const REBUILD: &str = "declare it in the NixOS configuration, then `sudo nixos-rebuild switch`";

/// Directories that hold a root-owned GitHub token in a file. They are not
/// machine layout, so they live here rather than in config. Their parents
/// keep the host user out, so the check reports each directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LegacyTokenDir {
    /// The container's token, which the broker replaces; nothing reads it.
    Agent,
    /// The CI controller's token file, which the broker replaces; nothing
    /// reads it.
    Ci,
}

impl LegacyTokenDir {
    pub const ALL: [Self; 2] = [Self::Agent, Self::Ci];

    pub fn path(self) -> &'static Path {
        Path::new(match self {
            Self::Agent => "/var/lib/agent-secrets/coda-agent",
            Self::Ci => "/var/lib/coda-ci",
        })
    }
}

/// Runs every check. Editor checks run even when the config does not load.
pub fn run(probe: &dyn Probe, config: &Result<Loaded, Error>) -> Vec<Finding> {
    let mut findings = vec![Finding {
        check: Check::Config,
        outcome: config_outcome(config),
    }];
    if let Ok(loaded) = config {
        findings.extend(machine(probe, &loaded.config));
    }
    findings.extend(Editor::ALL.into_iter().filter_map(|editor| {
        Some(Finding {
            check: Check::EditorSettings(editor),
            outcome: editor_outcome(probe, editor)?,
        })
    }));
    findings
}

fn config_outcome(config: &Result<Loaded, Error>) -> Outcome {
    match config {
        Ok(Loaded { unknown_keys, .. }) if unknown_keys.is_empty() => Outcome::Pass {
            detail: "loaded".to_owned(),
        },
        Ok(Loaded { unknown_keys, .. }) => Outcome::Warn {
            detail: format!(
                "config has keys this binary does not know ({}); binary older than config?",
                unknown_keys.join(", ")
            ),
            fix: Fix::instruction(
                "run a coda-sandbox built from the revision that wrote the config, \
                 or remove the keys if they are typos",
            ),
        },
        Err(error @ Error::ConfigMissing { .. }) => Outcome::Fail {
            detail: error.chain(),
            fix: Fix::instruction(
                "pass `--config crates/workspace-tools/sandbox/config/cyrene.toml` \
                 or set CODA_SANDBOX_CONFIG; \
                 the NixOS module will write the default path",
            ),
        },
        Err(error) => Outcome::Fail {
            detail: error.chain(),
            fix: Fix::instruction("correct the config file"),
        },
    }
}

fn machine(probe: &dyn Probe, config: &Config) -> Vec<Finding> {
    let root = &config.build.root;
    let root_kind = probe.path_kind(root);
    let reflinks = match &root_kind {
        Ok(PathKind::Directory) => reflink::outcome(probe, root),
        _ => Outcome::Skip {
            reason: "needs the build root".to_owned(),
        },
    };
    let mut findings = vec![
        (Check::BuildRoot, build_root_outcome(root, root_kind)),
        (Check::Reflinks, reflinks),
        (
            Check::ContainerUnit,
            unit_outcome(probe, UnitScope::System, &config.container.unit),
        ),
        (Check::Ssh, ssh_outcome(probe, &config.container.ssh_host)),
        (
            Check::SshForwarding,
            ssh_forwarding_outcome(probe, &config.container.ssh_host),
        ),
        (
            Check::SpoolDir(SpoolDir::Requests),
            spool_outcome(probe, &config.ci.requests()),
        ),
        (
            Check::SpoolDir(SpoolDir::Results),
            spool_outcome(probe, &config.ci.results()),
        ),
        (
            Check::CiTimer,
            unit_outcome(probe, UnitScope::System, &config.ci.timer),
        ),
        (Check::Broker, broker::outcome(probe, &config.broker)),
    ];
    for dir in LegacyTokenDir::ALL {
        findings.push((Check::LegacyToken(dir), legacy_token_outcome(probe, dir)));
    }
    findings
        .into_iter()
        .map(|(check, outcome)| Finding { check, outcome })
        .collect()
}

fn build_root_outcome(root: &Path, kind: Result<PathKind, Error>) -> Outcome {
    let shown = root.display();
    match kind {
        Ok(PathKind::Directory) => Outcome::Pass {
            detail: format!("{shown}"),
        },
        Ok(PathKind::Missing) => Outcome::Fail {
            detail: format!("{shown} does not exist"),
            fix: Fix::instruction(format!(
                "mount a reflink filesystem (ZFS with block cloning, Btrfs or XFS) at {shown}"
            )),
        },
        Ok(other) => Outcome::Fail {
            detail: format!("{shown} is a {other:?}, not a directory"),
            fix: Fix::instruction(format!(
                "move it aside and mount a reflink filesystem at {shown}"
            )),
        },
        Err(cause) => Outcome::CannotCheck { cause },
    }
}

/// What a fix asks systemd to do. The module's polkit rule lets the sandbox
/// user start the container and CI units, the only system units doctor
/// checks, without sudo; unmasking still needs root.
#[derive(Debug, Clone, Copy)]
enum Verb {
    Start,
    Unmask,
}

fn systemctl(scope: UnitScope, verb: Verb, unit: &UnitName) -> Fix {
    let (verb, needs_root) = match verb {
        Verb::Start => ("start", false),
        Verb::Unmask => ("unmask", true),
    };
    let args = [verb, unit.as_str()].map(str::to_owned);
    match scope {
        UnitScope::System if needs_root => {
            Fix::command("sudo", ["systemctl".to_owned()].into_iter().chain(args))
        }
        UnitScope::System => Fix::command("systemctl", args),
        UnitScope::User => Fix::command("systemctl", ["--user".to_owned()].into_iter().chain(args)),
    }
}

fn unit_outcome(probe: &dyn Probe, scope: UnitScope, unit: &UnitName) -> Outcome {
    let state = match probe.unit(scope, unit) {
        Ok(state) => state,
        Err(cause) => return Outcome::CannotCheck { cause },
    };
    let detail = format!("{unit} {:?}/{:?}", state.load, state.active);
    let status = || match scope {
        UnitScope::System => Fix::command("systemctl", ["status", unit.as_str()]),
        UnitScope::User => Fix::command("systemctl", ["--user", "status", unit.as_str()]),
    };
    match (state.load, state.active) {
        (
            LoadState::Loaded,
            ActiveState::Active | ActiveState::Reloading | ActiveState::Refreshing,
        ) => Outcome::Pass {
            detail: format!("{unit} {:?}", state.active).to_lowercase(),
        },
        (
            LoadState::Loaded,
            ActiveState::Inactive
            | ActiveState::Failed
            | ActiveState::Activating
            | ActiveState::Deactivating
            | ActiveState::Maintenance,
        ) => Outcome::Fail {
            detail,
            fix: systemctl(scope, Verb::Start, unit),
        },
        (LoadState::NotFound, _) => Outcome::Fail {
            detail: format!("{unit} not found"),
            fix: Fix::instruction(REBUILD),
        },
        (LoadState::Masked, _) => Outcome::Fail {
            detail,
            fix: systemctl(scope, Verb::Unmask, unit),
        },
        (LoadState::Stub | LoadState::BadSetting | LoadState::Error | LoadState::Merged, _) => {
            Outcome::Fail {
                detail,
                fix: status(),
            }
        }
    }
}

fn ssh_outcome(probe: &dyn Probe, host: &str) -> Outcome {
    let why = || Fix::command("ssh", ["-v", host, "true"]);
    match probe.ssh(host) {
        Ok(SshReach::Reachable) => Outcome::Pass {
            detail: format!("{host} answers"),
        },
        Ok(SshReach::Failed { exit_code, stderr }) => Outcome::Fail {
            detail: match exit_code {
                Some(code) => format!("`ssh {host} true` exited {code}: {stderr}"),
                None => format!("`ssh {host} true` was killed by a signal: {stderr}"),
            },
            fix: why(),
        },
        Ok(SshReach::TimedOut) => Outcome::Fail {
            detail: format!(
                "`ssh {host} true` did not finish in {} s",
                SSH_DEADLINE.as_secs()
            ),
            fix: why(),
        },
        Err(cause) => Outcome::CannotCheck { cause },
    }
}

fn spool_outcome(probe: &dyn Probe, path: &Path) -> Outcome {
    match probe.path_kind(path) {
        Ok(PathKind::Directory) => Outcome::Pass {
            detail: path.display().to_string(),
        },
        Ok(kind) => Outcome::Fail {
            detail: format!("{} is {kind:?}, not a directory", path.display()),
            fix: Fix::instruction(format!("the CI module creates it: {REBUILD}")),
        },
        Err(cause) => Outcome::CannotCheck { cause },
    }
}

fn legacy_token_outcome(probe: &dyn Probe, dir: LegacyTokenDir) -> Outcome {
    let path = dir.path();
    match probe.path_kind(path) {
        Ok(PathKind::Missing) => Outcome::Pass {
            detail: "absent".to_owned(),
        },
        Ok(_) => match dir {
            LegacyTokenDir::Agent => Outcome::Warn {
                detail: "present; it may hold the container's old root-owned GitHub token, \
                         which the token broker replaces"
                    .to_owned(),
                fix: Fix::instruction(format!(
                    "once the token broker check passes, revoke that token on github.com, \
                     then `sudo rm -rf {}`",
                    path.display()
                )),
            },
            LegacyTokenDir::Ci => Outcome::Warn {
                detail: "present; it may hold the CI controller's old root-owned GitHub token, \
                         which the token broker replaces"
                    .to_owned(),
                fix: Fix::instruction(format!(
                    "once the token broker check shows github-ci, revoke that token on \
                     github.com, then `sudo rm -rf {}`",
                    path.display()
                )),
            },
        },
        Err(cause) => Outcome::CannotCheck { cause },
    }
}

/// Every editor's remote, and Claude Code's and Codex's shells, are SSH
/// sessions under the user's SSH config, so its `ForwardAgent` decides
/// whether the host's keys reach the container.
fn ssh_forwarding_outcome(probe: &dyn Probe, host: &str) -> Outcome {
    let forward = match probe
        .ssh_config(host)
        .and_then(|resolved| resolved.forward_agent())
    {
        Ok(forward) => forward,
        Err(cause) => return Outcome::CannotCheck { cause },
    };
    let agent = match forward {
        ForwardAgent::No => {
            return Outcome::Pass {
                detail: format!("{host}: ForwardAgent no"),
            };
        }
        ForwardAgent::Yes => "yes".to_owned(),
        ForwardAgent::Socket(socket) => socket,
    };
    Outcome::Fail {
        detail: format!(
            "{host}: ForwardAgent {agent}, so the editors' SSH sessions carry the host's keys"
        ),
        fix: Fix::instruction(format!(
            "set `ForwardAgent no` under `Host {host}` in ~/.ssh/config"
        )),
    }
}

fn editor_outcome(probe: &dyn Probe, editor: Editor) -> Option<Outcome> {
    let file = editor.settings()?;
    let Some(home) = probe.home() else {
        return Some(Outcome::CannotCheck {
            cause: Error::NoHome,
        });
    };
    Some(settings_outcome(probe, &file.path(&home), file))
}

fn settings_outcome(probe: &dyn Probe, path: &Path, file: SettingsFile) -> Outcome {
    let text = match probe.read_file(path) {
        Ok(Some(text)) => text,
        Ok(None) => return no_settings_outcome(probe, path, file),
        Err(cause) => return Outcome::CannotCheck { cause },
    };
    let settings = match Settings::parse(path, &text) {
        Ok(settings) => settings,
        Err(cause) => return Outcome::CannotCheck { cause },
    };
    if let Some(failure) = forwarding_failure(&settings, file.keys, path) {
        return failure;
    }
    extensions_outcome(&settings, file.extensions, path)
}

/// An editor that has run keeps its user directory even with no settings
/// file, and then every forwarding key is at its default, on. An editor
/// that never ran on this machine has neither.
fn no_settings_outcome(probe: &dyn Probe, path: &Path, file: SettingsFile) -> Outcome {
    let Some(user_dir) = path.parent() else {
        return Outcome::Skip {
            reason: format!("no {}", path.display()),
        };
    };
    match probe.path_kind(user_dir) {
        Ok(PathKind::Directory) => {
            let keys: Vec<&str> = file.keys.iter().map(|key| key.key()).collect();
            Outcome::Fail {
                detail: format!(
                    "no {}, so {} default to true",
                    path.display(),
                    keys.join(", ")
                ),
                fix: Fix::instruction(format!(
                    "create {} setting {} to false",
                    path.display(),
                    keys.join(", ")
                )),
            }
        }
        Ok(_) => Outcome::Skip {
            reason: format!("no {}", path.display()),
        },
        Err(cause) => Outcome::CannotCheck { cause },
    }
}

/// Missing extensions only warn: the remote still works and forwards
/// nothing, but rust-analyzer and the dev shell wait for a manual install.
fn extensions_outcome(settings: &Settings, extensions: RemoteExtensions, path: &Path) -> Outcome {
    let missing = match extensions {
        RemoteExtensions::Setting => settings.missing_remote_extensions(),
        RemoteExtensions::ByHand => Vec::new(),
    };
    if missing.is_empty() {
        return Outcome::Pass {
            detail: "credential forwarding off".to_owned(),
        };
    }
    Outcome::Warn {
        detail: format!(
            "credential forwarding off; {DEFAULT_EXTENSIONS_KEY} lacks {}, so the container side has no dev shell or rust-analyzer until they are installed",
            missing.join(", ")
        ),
        fix: Fix::instruction(format!(
            "add {} to {DEFAULT_EXTENSIONS_KEY} in {}",
            missing.join(", "),
            path.display()
        )),
    }
}

/// Unset fails like `true`: the editors default these keys on, so an unset
/// key forwards the host's credentials just the same.
fn forwarding_failure(settings: &Settings, keys: &[ForwardingKey], path: &Path) -> Option<Outcome> {
    let not_off: Vec<(ForwardingKey, Setting)> = keys
        .iter()
        .map(|&key| (key, settings.get(key)))
        .filter(|(_, setting)| *setting != Setting::Off)
        .collect();
    if not_off.is_empty() {
        return None;
    }
    let keys: Vec<&str> = not_off.iter().map(|(key, _)| key.key()).collect();
    let described: Vec<String> = not_off
        .iter()
        .map(|(key, setting)| match setting {
            Setting::Absent => format!("{} unset (defaults to true)", key.key()),
            Setting::NotBoolean(value) => format!("{} = {value}", key.key()),
            Setting::On | Setting::Off => format!("{} = true", key.key()),
        })
        .collect();
    Some(Outcome::Fail {
        detail: described.join("; "),
        fix: Fix::instruction(format!(
            "set {} to false in {}",
            keys.join(", "),
            path.display()
        )),
    })
}
