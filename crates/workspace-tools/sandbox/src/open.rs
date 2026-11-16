//! `coda-sandbox shell` and `coda-sandbox open`: reach a checkout in the
//! container from the host, as a terminal or in an editor. Both name the
//! checkout by worktree slug, or the main checkout without one, and go
//! through the user's SSH config entry for the container with nothing
//! forwarded.

use std::ffi::OsString;
use std::io::{self, IsTerminal as _};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};

use crate::config::Config;
use crate::editor_settings::{Editor, Remote, RemoteExtensions, RemoteServer, ServerExtensions};
use crate::error::Error;
use crate::ids::Slug;
use crate::probe::{self, FromContainer};
use crate::worktree::Layout;

/// Options for every SSH session these commands start: no agent, X11 or
/// port forwarding, whatever the user's config says for other hosts.
const SSH_OPTIONS: [&str; 6] = [
    "-o",
    "ForwardAgent=no",
    "-o",
    "ForwardX11=no",
    "-o",
    "ClearAllForwardings=yes",
];

/// Quotes `word` for a POSIX shell when it holds anything but plain characters.
pub fn shell_quote(word: &str) -> String {
    let plain = |c: char| c.is_ascii_alphanumeric() || "@%+=:,./-_".contains(c);
    if !word.is_empty() && word.chars().all(plain) {
        word.to_owned()
    } else {
        format!("'{}'", word.replace('\'', r"'\''"))
    }
}

/// The checkout a slug names inside the container: the worktree
/// `<repo>/.worktrees/<slug>`, or the main checkout without a slug.
pub fn checkout(config: &Config, slug: Option<&Slug>) -> PathBuf {
    let layout = Layout::new(config.container.repo.clone(), &config.build);
    match slug {
        Some(slug) => layout.tree(slug),
        None => layout.main,
    }
}

/// What `shell` runs in the checkout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Run {
    /// The agent's login shell, whose direnv hook loads the dev shell.
    Login,
    /// One command, run in the dev shell through `direnv exec`.
    Command(Vec<String>),
}

/// The shell line the container runs: change to the checkout, then the
/// login shell or the command in the checkout's dev shell.
pub fn remote_line(dir: &Path, run: &Run) -> String {
    let cd = format!("cd {}", shell_quote(&dir.to_string_lossy()));
    match run {
        Run::Login => format!("{cd} && exec \"$SHELL\" -l"),
        Run::Command(words) => {
            let words: Vec<String> = words.iter().map(|word| shell_quote(word)).collect();
            format!("{cd} && exec direnv exec . {}", words.join(" "))
        }
    }
}

/// The `ssh` arguments for `shell`. A terminal is allocated for the login
/// shell, and for a command when this side is a terminal, so full-screen
/// programs such as `claude` and `codex` work.
pub fn shell_args(host: &str, dir: &Path, run: &Run, terminal: bool) -> Vec<String> {
    let mut args: Vec<String> = SSH_OPTIONS.map(str::to_owned).to_vec();
    if terminal || *run == Run::Login {
        args.push("-t".to_owned());
    }
    args.push(host.to_owned());
    args.push(remote_line(dir, run));
    args
}

/// Runs `shell` and returns ssh's status, which is the remote command's.
pub fn shell(config: &Config, slug: Option<&Slug>, run: &Run) -> Result<ExitStatus, Error> {
    let dir = checkout(config, slug);
    let terminal = io::stdin().is_terminal() && io::stdout().is_terminal();
    let args = shell_args(&config.container.ssh_host, &dir, run, terminal);
    Command::new("ssh")
        .args(args)
        .status()
        .map_err(|source| Error::Spawn {
            program: "ssh",
            source,
        })
}

/// The editor's launcher and its arguments to open `dir` on `host`.
pub fn editor_command(editor: Editor, host: &str, dir: &Path) -> (&'static str, Vec<OsString>) {
    let args = match editor.remote() {
        Remote::SshRemote => vec![
            "--remote".into(),
            format!("ssh-remote+{host}").into(),
            dir.as_os_str().to_owned(),
        ],
        Remote::ZedSsh => vec![format!("ssh://{host}{}", dir.display()).into()],
    };
    (editor.program(), args)
}

/// What `open` did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opened {
    pub dir: PathBuf,
    /// The remote extensions it installed into the editor's server first.
    pub installed: Vec<&'static str>,
}

/// Opens the checkout in `editor`, after checking over SSH that it exists,
/// since an editor opens a missing folder without saying so. For an editor
/// whose settings cannot install the remote extensions, it installs the
/// missing ones into the editor's server in the container first; before
/// the editor's first connection there is no server yet, and the next
/// `open` installs them.
pub fn open(config: &Config, slug: Option<&Slug>, editor: Editor) -> Result<Opened, Error> {
    let host = &config.container.ssh_host;
    let dir = checkout(config, slug);
    need_dir(host, &dir)?;
    let installed = match editor.settings().map(|file| file.extensions) {
        Some(RemoteExtensions::Server(server)) => install_missing(host, server)?,
        Some(RemoteExtensions::Setting) | None => Vec::new(),
    };
    launch(editor, host, &dir)?;
    Ok(Opened { dir, installed })
}

/// Installs the [`crate::editor_settings::REMOTE_EXTENSIONS`] `server` lacks
/// and returns them.
fn install_missing(host: &str, server: RemoteServer) -> Result<Vec<&'static str>, Error> {
    let Some(missing) = missing_on(host, server)? else {
        return Ok(Vec::new());
    };
    if missing.is_empty() {
        return Ok(missing);
    }
    // No deadline: the server downloads each extension from its gallery.
    let output = Command::new("ssh")
        .args(SSH_OPTIONS)
        .args(["-o", "BatchMode=yes", "-o", "ConnectTimeout=5", host])
        .arg(server.install_line(&missing))
        .stdin(Stdio::null())
        .output()
        .map_err(|source| Error::Spawn {
            program: "ssh",
            source,
        })?;
    // Antigravity's server can exit 1 from an unrelated error after it
    // installed everything, so what it now lists is the verdict.
    let still = missing_on(host, server)?.unwrap_or_else(|| missing.clone());
    if !still.is_empty() {
        return Err(Error::ContainerFailed {
            host: host.to_owned(),
            what: "install the editor server's extensions",
            exit_code: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        });
    }
    Ok(missing)
}

/// The remote extensions `server` lacks, or `None` before the editor has
/// installed it.
fn missing_on(host: &str, server: RemoteServer) -> Result<Option<Vec<&'static str>>, Error> {
    const WHAT: &str = "list the editor server's extensions";
    match probe::ssh_server_extensions(host, server)? {
        FromContainer::Answered(ServerExtensions::NoServer) => Ok(None),
        FromContainer::Answered(ServerExtensions::Missing(missing)) => Ok(Some(missing)),
        FromContainer::Failed { exit_code, stderr } => Err(Error::ContainerFailed {
            host: host.to_owned(),
            what: WHAT,
            exit_code,
            stderr,
        }),
        FromContainer::TimedOut => Err(Error::ContainerTimedOut {
            host: host.to_owned(),
            what: WHAT,
            seconds: probe::SSH_DEADLINE.as_secs(),
        }),
    }
}

fn launch(editor: Editor, host: &str, dir: &Path) -> Result<(), Error> {
    let (program, args) = editor_command(editor, host, dir);
    let status = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .status()
        .map_err(|source| match source.kind() {
            io::ErrorKind::NotFound => Error::NoEditor {
                editor,
                program,
                source,
            },
            _ => Error::Spawn { program, source },
        })?;
    if !status.success() {
        return Err(Error::EditorFailed { program, status });
    }
    Ok(())
}

fn need_dir(host: &str, dir: &Path) -> Result<(), Error> {
    let output = Command::new("ssh")
        .args(SSH_OPTIONS)
        .args(["-o", "BatchMode=yes", "-o", "ConnectTimeout=5", host])
        .arg(format!("test -d {}", shell_quote(&dir.to_string_lossy())))
        .stdin(Stdio::null())
        .output()
        .map_err(|source| Error::Spawn {
            program: "ssh",
            source,
        })?;
    match output.status.code() {
        Some(0) => Ok(()),
        Some(1) => Err(Error::NoCheckout {
            host: host.to_owned(),
            path: dir.to_owned(),
        }),
        _ => Err(Error::CommandFailed {
            program: "ssh",
            status: output.status,
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    const CONFIG: &str = include_str!("../config/cyrene.toml");

    fn config() -> Config {
        Config::parse(Path::new("cyrene.toml"), CONFIG).expect("the Cyrene config parses")
    }

    #[test]
    fn a_slug_names_its_worktree_and_none_the_main_checkout() {
        let config = config();
        let repo = config.container.repo.clone();
        assert_eq!(checkout(&config, None), repo);
        let slug: Slug = "eng-7".parse().expect("a slug");
        assert_eq!(
            checkout(&config, Some(&slug)),
            repo.join(".worktrees/eng-7")
        );
    }

    #[test]
    fn the_shell_forwards_nothing_and_runs_commands_in_the_dev_shell() {
        let dir = Path::new("/agents/coda/src/coda-rust/.worktrees/x");
        let login = shell_args("coda-agent", dir, &Run::Login, false);
        assert!(login.windows(2).any(|w| w == ["-o", "ForwardAgent=no"]));
        assert!(login.contains(&"-t".to_owned()));
        assert_eq!(
            login.last().map(String::as_str),
            Some("cd /agents/coda/src/coda-rust/.worktrees/x && exec \"$SHELL\" -l")
        );
        let run = Run::Command(vec!["cargo".into(), "test".into(), "it's".into()]);
        let piped = shell_args("coda-agent", dir, &run, false);
        assert!(!piped.contains(&"-t".to_owned()));
        assert_eq!(
            piped.last().map(String::as_str),
            Some(
                r"cd /agents/coda/src/coda-rust/.worktrees/x && exec direnv exec . cargo test 'it'\''s'"
            )
        );
        assert!(shell_args("coda-agent", dir, &run, true).contains(&"-t".to_owned()));
    }

    #[test]
    fn each_editor_opens_the_checkout_over_its_own_remote() {
        let dir = Path::new("/agents/coda/src/coda-rust");
        let shown = |editor| {
            let (program, args) = editor_command(editor, "coda-agent", dir);
            let args: Vec<String> = args
                .iter()
                .map(|arg| arg.to_string_lossy().into_owned())
                .collect();
            format!("{program} {}", args.join(" "))
        };
        assert_eq!(
            shown(Editor::Cursor),
            "cursor --remote ssh-remote+coda-agent /agents/coda/src/coda-rust"
        );
        assert_eq!(
            shown(Editor::VsCode),
            "code --remote ssh-remote+coda-agent /agents/coda/src/coda-rust"
        );
        assert_eq!(
            shown(Editor::Antigravity),
            "antigravity-ide --remote ssh-remote+coda-agent /agents/coda/src/coda-rust"
        );
        assert_eq!(
            shown(Editor::Zed),
            "zeditor ssh://coda-agent/agents/coda/src/coda-rust"
        );
    }

    #[test]
    fn quoting_leaves_plain_words_and_wraps_the_rest() {
        assert_eq!(shell_quote("a/b-c.d"), "a/b-c.d");
        assert_eq!(shell_quote(""), "''");
        assert_eq!(shell_quote("$HOME"), "'$HOME'");
    }
}
