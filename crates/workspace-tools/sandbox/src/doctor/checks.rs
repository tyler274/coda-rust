use std::path::Path;

use serde::Serialize;

use super::{Check, Finding, Fix, Outcome, SpoolDir, broker, reflink};
use crate::config::{Config, Loaded, UnitName};
use crate::editor_settings::{
    DEFAULT_EXTENSIONS_KEY, Editor, ForwardingKey, RemoteExtensions, RemoteServer,
    ServerExtensions, Setting, Settings, SettingsFile,
};
use crate::error::Error;
use crate::model_proxy::ProxyCheck;
use crate::probe::{
    ActiveState, ForwardAgent, FromContainer, LoadState, PROXY_CHECK, PathKind, Probe,
    SSH_DEADLINE, SshReach, UnitScope,
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
    let host = config
        .as_ref()
        .ok()
        .map(|loaded| loaded.config.container.ssh_host.as_str());
    findings.extend(Editor::ALL.into_iter().filter_map(|editor| {
        Some(Finding {
            check: Check::EditorSettings(editor),
            outcome: editor_outcome(probe, host, editor)?,
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
        (Check::ArpIgnore, arp_ignore_outcome(probe)),
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
        (Check::Broker, broker::outcome(probe, config)),
        (Check::ModelProxy, model_proxy_outcome(probe, config)),
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

/// The host's `net.ipv4.conf.all.arp_ignore`.
pub(super) const ARP_IGNORE: &str = "/proc/sys/net/ipv4/conf/all/arp_ignore";

/// The kernel's `arp_ignore` modes (Documentation/networking/ip-sysctl.rst).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArpIgnore {
    /// 0: answer for any local address on any interface.
    Any,
    /// 1: answer only for addresses on the interface asked.
    Interface,
    /// 2: as 1, and only when the sender is in the target's subnet. Each
    /// veth end is a /32, so neither end answers the other.
    Subnet,
    /// 3 to 8: scope or never-answer modes the sandbox does not use.
    Other(u8),
}

impl ArpIgnore {
    fn parse(text: &str) -> Option<Self> {
        Some(match text.trim().parse::<u8>().ok()? {
            0 => Self::Any,
            1 => Self::Interface,
            2 => Self::Subnet,
            other => Self::Other(other),
        })
    }
}

/// The module sets 1 at boot, but `nixos-rebuild switch` does not reapply
/// sysctls, and the Mullvad daemon raises 0 to 2 when it starts. A host at
/// 2 cannot ARP its containers' /32 addresses, and a container started then
/// copies the value.
fn arp_ignore_outcome(probe: &dyn Probe) -> Outcome {
    let path = Path::new(ARP_IGNORE);
    let text = match probe.read_file(path) {
        Ok(Some(text)) => text,
        Ok(None) => {
            return Outcome::Skip {
                reason: format!("no {ARP_IGNORE}"),
            };
        }
        Err(cause) => return Outcome::CannotCheck { cause },
    };
    let fix = || Fix::command("sudo", ["sysctl", "-w", "net.ipv4.conf.all.arp_ignore=1"]);
    match ArpIgnore::parse(&text) {
        Some(ArpIgnore::Interface) => Outcome::Pass {
            detail: "1".to_owned(),
        },
        Some(ArpIgnore::Any) => Outcome::Warn {
            detail: "0, not the module's 1; a daemon such as Mullvad may raise it to 2 \
                     and cut the host off from the container"
                .to_owned(),
            fix: fix(),
        },
        Some(ArpIgnore::Subnet) => Outcome::Fail {
            detail: "2, so the host gets no ARP answer from the container's /32 address, \
                     and a container started now copies it"
                .to_owned(),
            fix: fix(),
        },
        Some(ArpIgnore::Other(mode)) => Outcome::Fail {
            detail: format!("{mode}, not the module's 1"),
            fix: fix(),
        },
        None => Outcome::CannotCheck {
            cause: Error::UnexpectedOutput {
                what: "arp_ignore",
                source_name: ARP_IGNORE.to_owned(),
                found: text,
            },
        },
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

fn mint() -> Fix {
    broker::fill(crate::broker::Item::ModelProxy)
}

/// Asks from inside the container, so the check crosses the same firewall
/// and presents the same key as Claude Code and Codex do.
fn model_proxy_outcome(probe: &dyn Probe, config: &Config) -> Outcome {
    let Some(proxy) = &config.model_proxy else {
        return Outcome::Skip {
            reason: "modelProxy is off; the agent CLIs keep their own logins".to_owned(),
        };
    };
    let host = &config.container.ssh_host;
    let url = &proxy.url;
    let found = match probe.proxy_check(host) {
        Ok(FromContainer::Answered(found)) => found,
        Ok(FromContainer::Failed { stderr, .. }) => {
            return Outcome::Fail {
                detail: format!("`ssh {host} {PROXY_CHECK}` failed: {stderr}"),
                fix: Fix::command("ssh", [host.as_str(), PROXY_CHECK]),
            };
        }
        Ok(FromContainer::TimedOut) => {
            return Outcome::Fail {
                detail: format!(
                    "`ssh {host} {PROXY_CHECK}` did not finish in {} s",
                    SSH_DEADLINE.as_secs()
                ),
                fix: Fix::command("ssh", [host.as_str(), PROXY_CHECK]),
            };
        }
        Err(cause) => return Outcome::CannotCheck { cause },
    };
    match found {
        ProxyCheck::Accepted {
            client: Some(client),
        } => Outcome::Pass {
            detail: format!("{url} takes the key as client {client}"),
        },
        ProxyCheck::Accepted { client: None } => Outcome::Warn {
            detail: format!(
                "{url} has no client keys, so it serves anything that reaches it \
                 and cannot tell the container's usage apart"
            ),
            fix: mint(),
        },
        ProxyCheck::Refused { status: 401 | 403 } => Outcome::Fail {
            detail: format!("{url} refuses the broker's model-proxy key"),
            fix: mint(),
        },
        ProxyCheck::Refused { status } => Outcome::Fail {
            detail: format!("{url}/v1/client answered {status}"),
            fix: Fix::instruction("see the proxy's log on the host"),
        },
        ProxyCheck::NotTheProxy { error } => Outcome::Fail {
            detail: format!("{url} answers, but not as the model proxy: {error}"),
            fix: Fix::instruction(
                "run a proxy with `GET /v1/client`, such as Coda's ungate, on that port",
            ),
        },
        ProxyCheck::Unreachable { error } => Outcome::Fail {
            detail: format!("the container cannot reach {url}: {error}"),
            fix: Fix::instruction(
                "start the proxy on the host listening on hostAddress, \
                 such as `systemctl --user status ungate`",
            ),
        },
        ProxyCheck::NoKey { reason } => Outcome::Fail {
            detail: format!("no model-proxy key to present: {reason}"),
            fix: mint(),
        },
        ProxyCheck::Broker { error } => Outcome::Fail {
            detail: format!("the container's broker did not answer: {error}"),
            fix: Fix::command("systemctl", ["--user", "status", broker::UNIT]),
        },
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

/// `host` is the container's SSH alias, when the config loaded.
fn editor_outcome(probe: &dyn Probe, host: Option<&str>, editor: Editor) -> Option<Outcome> {
    let file = editor.settings()?;
    let Some(home) = probe.home() else {
        return Some(Outcome::CannotCheck {
            cause: Error::NoHome,
        });
    };
    Some(settings_outcome(
        probe,
        host,
        editor,
        &file.path(&home),
        file,
    ))
}

fn settings_outcome(
    probe: &dyn Probe,
    host: Option<&str>,
    editor: Editor,
    path: &Path,
    file: SettingsFile,
) -> Outcome {
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
    extensions_outcome(probe, host, editor, &settings, file.extensions, path)
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
/// nothing, but rust-analyzer and the dev shell wait for an install.
fn extensions_outcome(
    probe: &dyn Probe,
    host: Option<&str>,
    editor: Editor,
    settings: &Settings,
    extensions: RemoteExtensions,
    path: &Path,
) -> Outcome {
    match extensions {
        RemoteExtensions::Setting => setting_extensions_outcome(settings, path),
        RemoteExtensions::Server(server) => match host {
            Some(host) => server_extensions_outcome(probe, host, editor, server),
            None => Outcome::Pass {
                detail: "credential forwarding off; its server's extensions need the config"
                    .to_owned(),
            },
        },
    }
}

fn setting_extensions_outcome(settings: &Settings, path: &Path) -> Outcome {
    let missing = settings.missing_remote_extensions();
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

/// The extensions in the editor's server in the container, which
/// `coda-sandbox open` installs.
fn server_extensions_outcome(
    probe: &dyn Probe,
    host: &str,
    editor: Editor,
    server: RemoteServer,
) -> Outcome {
    const WHAT: &str = "list the editor server's extensions";
    let open = || Fix::command("coda-sandbox", ["open", "--editor", editor.value_name()]);
    let missing = match probe.server_extensions(host, server) {
        Ok(FromContainer::Answered(ServerExtensions::NoServer)) => {
            return Outcome::Pass {
                detail: format!(
                    "credential forwarding off; no {} server in the container yet",
                    editor.name()
                ),
            };
        }
        Ok(FromContainer::Answered(ServerExtensions::Missing(missing))) => missing,
        Ok(FromContainer::Failed { exit_code, stderr }) => {
            return Outcome::CannotCheck {
                cause: Error::ContainerFailed {
                    host: host.to_owned(),
                    what: WHAT,
                    exit_code,
                    stderr,
                },
            };
        }
        Ok(FromContainer::TimedOut) => {
            return Outcome::CannotCheck {
                cause: Error::ContainerTimedOut {
                    host: host.to_owned(),
                    what: WHAT,
                    seconds: SSH_DEADLINE.as_secs(),
                },
            };
        }
        Err(cause) => return Outcome::CannotCheck { cause },
    };
    if missing.is_empty() {
        return Outcome::Pass {
            detail: "credential forwarding off; its server has the remote extensions".to_owned(),
        };
    }
    Outcome::Warn {
        detail: format!(
            "credential forwarding off; its server in {host} lacks {}, so the container side has no dev shell or rust-analyzer",
            missing.join(", ")
        ),
        fix: open(),
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
