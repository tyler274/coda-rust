//! Runs git, the owner of repositories and worktrees. Every value travels as
//! its own argument; nothing is spliced into a shell string.

use std::ffi::{OsStr, OsString};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use crate::error::Error;

/// One git invocation in a directory, built up argument by argument.
pub struct Git {
    dir: PathBuf,
    args: Vec<OsString>,
    index: Option<PathBuf>,
    input: Option<Vec<u8>>,
}

/// Starts `git -C <dir>`.
pub fn git(dir: &Path) -> Git {
    Git {
        dir: dir.to_owned(),
        args: Vec::new(),
        index: None,
        input: None,
    }
}

impl Git {
    pub fn arg(mut self, arg: impl AsRef<OsStr>) -> Self {
        self.args.push(arg.as_ref().to_owned());
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.args
            .extend(args.into_iter().map(|arg| arg.as_ref().to_owned()));
        self
    }

    /// Uses another index file, leaving the worktree's own index untouched.
    pub fn index(mut self, index: &Path) -> Self {
        self.index = Some(index.to_owned());
        self
    }

    /// Feeds `input` to git's standard input, as `update-ref --stdin` reads.
    pub fn stdin(mut self, input: impl Into<Vec<u8>>) -> Self {
        self.input = Some(input.into());
        self
    }

    fn spawn(&self) -> Result<Output, Error> {
        let mut command = Command::new("git");
        command.arg("-C").arg(&self.dir).args(&self.args);
        if let Some(index) = &self.index {
            command.env("GIT_INDEX_FILE", index);
        }
        let spawn_error = |source| Error::Spawn {
            program: "git",
            source,
        };
        let Some(input) = &self.input else {
            return command.stdin(Stdio::null()).output().map_err(spawn_error);
        };
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(spawn_error)?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(input).map_err(|source| Error::Stdin {
                program: "git",
                source,
            })?;
        }
        child.wait_with_output().map_err(|source| Error::Wait {
            program: "git",
            source,
        })
    }

    fn failed(&self, output: &Output) -> Error {
        Error::Git {
            args: self
                .args
                .iter()
                .map(|arg| arg.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" "),
            dir: self.dir.clone(),
            status: output.status,
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        }
    }

    /// Standard output, or an error carrying git's status and message.
    pub fn output(&self) -> Result<String, Error> {
        let output = self.spawn()?;
        if !output.status.success() {
            return Err(self.failed(&output));
        }
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    /// Standard output with its trailing newline removed.
    pub fn line(&self) -> Result<String, Error> {
        Ok(self.output()?.trim_end().to_owned())
    }

    pub fn run(&self) -> Result<(), Error> {
        self.output().map(drop)
    }

    /// The first line of output when git exits 0, `None` when it answers no,
    /// as `rev-parse --verify --quiet` does for an unknown revision.
    pub fn line_if_ok(&self) -> Result<Option<String>, Error> {
        let output = self.spawn()?;
        Ok(output.status.success().then(|| {
            String::from_utf8_lossy(&output.stdout)
                .trim_end()
                .to_owned()
        }))
    }

    /// Whether git exited 0, for questions such as `show-ref --verify --quiet`
    /// whose answer is the exit status. Only a failure to start git is an error.
    pub fn succeeds(&self) -> Result<bool, Error> {
        Ok(self.spawn()?.status.success())
    }
}

fn rev_parse_common_dir(dir: &Path) -> Git {
    git(dir).args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
}

/// The git directory the repository at `dir` shares across its worktrees,
/// which names the repository whichever worktree `dir` is in.
pub fn common_dir(dir: &Path) -> Result<PathBuf, Error> {
    Ok(PathBuf::from(rev_parse_common_dir(dir).line()?))
}

/// Like [`common_dir`], `None` when `dir` is in no repository.
pub fn common_dir_if_any(dir: &Path) -> Result<Option<PathBuf>, Error> {
    Ok(rev_parse_common_dir(dir).line_if_ok()?.map(PathBuf::from))
}

/// The repository's object directory, shared by all its worktrees.
pub fn objects_dir(dir: &Path) -> Result<PathBuf, Error> {
    Ok(common_dir(dir)?.join("objects"))
}
