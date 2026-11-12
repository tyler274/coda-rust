//! `coda-sandbox doctor`: read-only checks of the sandbox on this machine.
//! Every warning and failure names the fix.

pub mod broker;
mod checks;
#[cfg(test)]
mod fake;
mod reflink;
#[cfg(test)]
mod tests;

use std::fmt::{self, Write as _};

pub use checks::{LegacyTokenDir, run};

use crate::editor_settings::Editor;
use crate::error::Error;

/// What one check looks at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Check {
    Config,
    BuildRoot,
    Reflinks,
    ContainerUnit,
    Ssh,
    SpoolDir(SpoolDir),
    CiTimer,
    /// The token broker answers and which source serves each token.
    Broker,
    /// A directory that holds a root-owned GitHub token in a file.
    LegacyToken(LegacyTokenDir),
    EditorSettings(Editor),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpoolDir {
    Requests,
    Results,
}

impl Check {
    pub fn label(&self) -> String {
        match self {
            Self::Config => "config".to_owned(),
            Self::BuildRoot => "build root".to_owned(),
            Self::Reflinks => "reflinks".to_owned(),
            Self::ContainerUnit => "container".to_owned(),
            Self::Ssh => "ssh".to_owned(),
            Self::SpoolDir(SpoolDir::Requests) => "spool requests".to_owned(),
            Self::SpoolDir(SpoolDir::Results) => "spool results".to_owned(),
            Self::CiTimer => "ci timer".to_owned(),
            Self::Broker => "token broker".to_owned(),
            Self::LegacyToken(dir) => format!("token dir {}", dir.path().display()),
            Self::EditorSettings(editor) => format!("{} settings", editor.name()),
        }
    }
}

/// How to resolve a warning or failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fix {
    /// One command to run as given; shown shell-quoted.
    Command { program: String, args: Vec<String> },
    /// Something a person does that no single command covers.
    Instruction(String),
}

impl Fix {
    pub fn command<I, S>(program: &str, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::Command {
            program: program.to_owned(),
            args: args.into_iter().map(Into::into).collect(),
        }
    }

    pub fn instruction(text: impl Into<String>) -> Self {
        Self::Instruction(text.into())
    }
}

/// Quotes `word` for a POSIX shell when it holds anything but plain characters.
fn shell_quote(word: &str) -> String {
    let plain = |c: char| c.is_ascii_alphanumeric() || "@%+=:,./-_".contains(c);
    if !word.is_empty() && word.chars().all(plain) {
        word.to_owned()
    } else {
        format!("'{}'", word.replace('\'', r"'\''"))
    }
}

impl fmt::Display for Fix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Command { program, args } => {
                f.write_str(&shell_quote(program))?;
                args.iter()
                    .try_for_each(|arg| write!(f, " {}", shell_quote(arg)))
            }
            Self::Instruction(text) => f.write_str(text),
        }
    }
}

/// What a check found.
#[derive(Debug)]
pub enum Outcome {
    Pass {
        detail: String,
    },
    Warn {
        detail: String,
        fix: Fix,
    },
    Fail {
        detail: String,
        fix: Fix,
    },
    /// The check could not read what it needs; nothing is known either way.
    CannotCheck {
        cause: Error,
    },
    /// The check does not apply here, such as an editor that is not installed.
    Skip {
        reason: String,
    },
}

impl Outcome {
    fn tag(&self) -> &'static str {
        match self {
            Self::Pass { .. } => "pass",
            Self::Warn { .. } => "WARN",
            Self::Fail { .. } => "FAIL",
            Self::CannotCheck { .. } => "????",
            Self::Skip { .. } => "skip",
        }
    }
}

#[derive(Debug)]
pub struct Finding {
    pub check: Check,
    pub outcome: Outcome,
}

/// The overall result. A failure outranks a check that could not run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// Every check passed, warned or did not apply. Exit 0.
    Healthy,
    /// At least one check failed. Exit 1.
    Failing,
    /// Nothing failed, but at least one check could not run. Exit 2.
    Incomplete,
}

impl Verdict {
    /// The process exit code, a contract for scripts and editor tasks.
    pub fn exit_code(self) -> u8 {
        match self {
            Self::Healthy => 0,
            Self::Failing => 1,
            Self::Incomplete => 2,
        }
    }
}

pub fn verdict(findings: &[Finding]) -> Verdict {
    let any = |pick: fn(&Outcome) -> bool| findings.iter().any(|f| pick(&f.outcome));
    if any(|o| matches!(o, Outcome::Fail { .. })) {
        Verdict::Failing
    } else if any(|o| matches!(o, Outcome::CannotCheck { .. })) {
        Verdict::Incomplete
    } else {
        Verdict::Healthy
    }
}

/// One line per finding, the fix indented beneath, then a count.
pub fn render(findings: &[Finding]) -> String {
    let width = findings
        .iter()
        .map(|f| f.check.label().len())
        .max()
        .unwrap_or(0);
    let mut out = String::new();
    let mut counts = [0_usize; 5];
    for finding in findings {
        let label = finding.check.label();
        let tag = finding.outcome.tag();
        let (index, detail, fix) = match &finding.outcome {
            Outcome::Pass { detail } => (0, detail.clone(), None),
            Outcome::Warn { detail, fix } => (1, detail.clone(), Some(fix)),
            Outcome::Fail { detail, fix } => (2, detail.clone(), Some(fix)),
            Outcome::CannotCheck { cause } => (3, format!("cannot check: {}", cause.chain()), None),
            Outcome::Skip { reason } => (4, reason.clone(), None),
        };
        counts[index] += 1;
        let _ = writeln!(out, "{tag}  {label:<width$}  {detail}");
        if let Some(fix) = fix {
            let _ = writeln!(out, "      {:<width$}  fix: {fix}", "");
        }
    }
    let [pass, warn, fail, unknown, skip] = counts;
    let _ = writeln!(
        out,
        "\n{pass} passed, {warn} warned, {fail} failed, {unknown} could not be checked, {skip} skipped"
    );
    out
}
