//! Scratch repositories and configs for the integration tests.

#![allow(dead_code, clippy::missing_panics_doc)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A temporary directory with an isolated git identity and config.
pub struct Scratch {
    pub dir: tempfile::TempDir,
}

impl Scratch {
    pub fn new() -> Self {
        let dir = tempfile::tempdir().expect("temp dir");
        std::fs::write(
            dir.path().join("gitconfig"),
            "[user]\n\tname = test\n\temail = test@test\n[init]\n\tdefaultBranch = main\n",
        )
        .expect("gitconfig");
        Self { dir }
    }

    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    fn env(&self, command: &mut Command) {
        command
            .env("GIT_CONFIG_GLOBAL", self.path().join("gitconfig"))
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("HOME", self.path());
    }

    /// Runs git in `dir` and returns its trimmed standard output.
    pub fn git(&self, dir: &Path, args: &[&str]) -> String {
        let mut command = Command::new("git");
        command.arg("-C").arg(dir).args(args);
        self.env(&mut command);
        let output = command.output().expect("run git");
        assert!(
            output.status.success(),
            "git {args:?} in {}: {}",
            dir.display(),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).trim().to_owned()
    }

    /// A repository with one commit, `/target` (as the real repository
    /// ignores it, which also matches the `target` symlink) and `*.log`
    /// ignored, and the seed key's three input files.
    pub fn repo(&self, name: &str) -> PathBuf {
        let repo = self.path().join(name);
        std::fs::create_dir_all(&repo).expect("repo dir");
        self.git(&repo, &["init", "-q"]);
        write(&repo.join(".gitignore"), "/target\n*.log\n/.worktrees\n");
        write(&repo.join("a.txt"), "one\n");
        write(&repo.join("Cargo.lock"), "cargo\n");
        write(&repo.join("flake.lock"), "flake\n");
        write(&repo.join("rust-toolchain.toml"), "toolchain\n");
        self.git(&repo, &["add", "-A"]);
        self.git(&repo, &["commit", "-qm", "one"]);
        repo
    }

    /// Writes a config whose build root, carry repo and spool are given.
    pub fn config(&self, build: &Path, carry_repo: &Path, spool: &Path) -> PathBuf {
        let path = self.path().join("config.toml");
        let text = format!(
            "version = 1\n\
             [container]\nname = \"coda-agent\"\nunit = \"container@coda-agent.service\"\n\
             ssh_host = \"coda-agent\"\nagent_uid = 1\nrepo = {repo:?}\n\
             [build]\nroot = {build:?}\n\
             [ci]\nspool = {spool:?}\ntimeout_seconds = 60\n\
             timer = \"coda-ci.timer\"\nservice = \"coda-ci.service\"\n\
             [carry]\nrepo = {repo:?}\nout = {out:?}\n",
            repo = carry_repo.display().to_string(),
            build = build.display().to_string(),
            spool = spool.display().to_string(),
            out = self.path().join("carry-out").display().to_string(),
        );
        write(&path, &text);
        path
    }

    /// Runs `coda-sandbox --config <config> <args>` in `dir`.
    pub fn sandbox(&self, config: &Path, dir: &Path, args: &[&str]) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_coda-sandbox"));
        command
            .arg("--config")
            .arg(config)
            .args(args)
            .current_dir(dir);
        self.env(&mut command);
        command.output().expect("run coda-sandbox")
    }

    /// Like [`Self::sandbox`], asserting success and returning stdout.
    pub fn ok(&self, config: &Path, dir: &Path, args: &[&str]) -> String {
        let output = self.sandbox(config, dir, args);
        assert!(
            output.status.success(),
            "coda-sandbox {args:?}: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).into_owned()
    }
}

pub fn write(path: &Path, text: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("parent dir");
    }
    std::fs::write(path, text).expect("write file");
}

pub fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}
