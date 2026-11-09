//! Bounded CLI boundary. Orchestration never spawns processes directly.
use super::{Result, credentials::Secret};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, Instant},
};

pub(super) const PASSWORDS_ENV: &str = "TF_VAR_database_passwords";
/// Exit status of coreutils `timeout` when it stopped the command.
const TIMED_OUT: [i32; 2] = [124, 137];

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum Program {
    Doppler,
    Tofu,
    Gcloud,
}
impl Program {
    pub fn binary(self) -> &'static str {
        match self {
            Self::Doppler => "doppler",
            Self::Tofu => "tofu",
            Self::Gcloud => "gcloud",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum Stage {
    Credentials,
    Init,
    Plan,
    Show,
    Apply,
    DriftPlan,
    Readback,
}
impl Stage {
    pub fn name(self) -> &'static str {
        match self {
            Self::Credentials => "credentials",
            Self::Init => "init",
            Self::Plan => "plan",
            Self::Show => "show",
            Self::Apply => "apply",
            Self::DriftPlan => "drift-plan",
            Self::Readback => "readback",
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct Invocation {
    pub program: Program,
    pub stage: Stage,
    pub args: Vec<String>,
    pub timeout: Duration,
}
impl Invocation {
    pub fn new<I, S>(program: Program, stage: Stage, args: I, timeout: Duration) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            program,
            stage,
            args: args.into_iter().map(Into::into).collect(),
            timeout,
        }
    }
    pub fn tofu<I, S>(root: &Path, stage: Stage, args: I, timeout: Duration) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut all = vec![format!("-chdir={}", root.display())];
        all.extend(args.into_iter().map(Into::into));
        Self::new(Program::Tofu, stage, all, timeout)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind", content = "code")]
pub(super) enum Exit {
    Success,
    Code(Option<i32>),
    Timeout,
}

pub(super) struct Run {
    pub exit: Exit,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub elapsed: Duration,
}

impl std::fmt::Debug for Run {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Run")
            .field("exit", &self.exit)
            .field("elapsed", &self.elapsed)
            .finish_non_exhaustive()
    }
}

pub(super) trait Runner {
    /// `secret`, when present, reaches only this child as `TF_VAR_database_passwords`.
    fn run(&mut self, invocation: &Invocation, secret: Option<&Secret>) -> Result<Run>;
    fn pause(&mut self, duration: Duration) {
        std::thread::sleep(duration);
    }
}

/// Spawns the dev-shell tools under coreutils `timeout`, which interrupts
/// OpenTofu with SIGINT first so it can release its state lock.
#[derive(Default)]
pub(super) struct ProcessRunner {
    /// Prepended to `PATH`; tests point it at synthetic tools.
    pub path_prefix: Option<PathBuf>,
}
impl Runner for ProcessRunner {
    fn run(&mut self, invocation: &Invocation, secret: Option<&Secret>) -> Result<Run> {
        let mut command = Command::new("timeout");
        command
            .args([
                "--signal=INT".into(),
                "--kill-after=30s".into(),
                format!("{}s", invocation.timeout.as_secs().max(1)),
                invocation.program.binary().into(),
            ])
            .args(&invocation.args)
            .stdin(std::process::Stdio::null())
            .env("CLOUDSDK_CORE_DISABLE_PROMPTS", "1")
            .env("TF_IN_AUTOMATION", "1")
            .env("TF_INPUT", "0")
            .env_remove(PASSWORDS_ENV);
        for (key, _) in std::env::vars_os() {
            let name = key.to_string_lossy();
            if name.starts_with("TF_") || name.starts_with("TOFU_") {
                command.env_remove(key);
            }
        }
        command
            .env("TF_WORKSPACE", "default")
            .env("TF_IN_AUTOMATION", "1")
            .env("TF_INPUT", "0")
            .env("CLOUDSDK_CORE_DISABLE_FILE_LOGGING", "true");
        if let Some(prefix) = &self.path_prefix {
            let mut path = OsString::from(prefix);
            if let Some(existing) = std::env::var_os("PATH") {
                path.push(":");
                path.push(existing);
            }
            command.env("PATH", path);
        }
        if let Some(secret) = secret {
            command.env(PASSWORDS_ENV, secret.expose());
        }
        let started = Instant::now();
        let output = command.output()?;
        let exit = match output.status.code() {
            Some(0) => Exit::Success,
            Some(code) if TIMED_OUT.contains(&code) => Exit::Timeout,
            code => Exit::Code(code),
        };
        Ok(Run {
            exit,
            stdout: output.stdout,
            stderr: output.stderr,
            elapsed: started.elapsed(),
        })
    }
}
