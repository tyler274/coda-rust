//! Why a source could not be read, with the cause kept as the error's source.

use std::path::PathBuf;
use std::process::Command;

use thiserror::Error;

/// A command the collector runs did not give a usable answer.
#[derive(Debug, Error)]
pub enum CommandError {
    #[error("could not start {command}")]
    Start {
        command: String,
        #[source]
        source: std::io::Error,
    },
    #[error("{command} exited with {status}: {stderr}")]
    Failed {
        command: String,
        status: std::process::ExitStatus,
        stderr: String,
    },
    #[error("{command} printed something unreadable: {output}")]
    Unreadable {
        command: String,
        output: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

/// Runs a command and returns its trimmed standard output, or why it failed.
pub fn output(command: &mut Command) -> Result<String, CommandError> {
    let name = describe(command);
    let result = command.output().map_err(|source| CommandError::Start {
        command: name.clone(),
        source,
    })?;
    if !result.status.success() {
        return Err(CommandError::Failed {
            command: name,
            status: result.status,
            stderr: String::from_utf8_lossy(&result.stderr).trim().to_owned(),
        });
    }
    Ok(String::from_utf8_lossy(&result.stdout).trim().to_owned())
}

fn describe(command: &Command) -> String {
    let mut words = vec![command.get_program().to_string_lossy().into_owned()];
    words.extend(
        command
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned()),
    );
    words.join(" ")
}

/// A file a source reads was missing its expected shape.
#[derive(Debug, Error)]
pub enum FileError {
    #[error("could not read {path}")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("{path} is not the expected JSON")]
    Parse {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
}

/// Reads and parses a JSON file.
pub fn read_json<T: serde::de::DeserializeOwned>(path: &std::path::Path) -> Result<T, FileError> {
    let text = std::fs::read_to_string(path).map_err(|source| FileError::Read {
        path: path.to_owned(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| FileError::Parse {
        path: path.to_owned(),
        source,
    })
}

/// The error and each cause beneath it, joined for a person to read in the panel.
pub fn chain(error: &(dyn std::error::Error + 'static)) -> String {
    let mut text = error.to_string();
    let mut cause = error.source();
    while let Some(next) = cause {
        text.push_str(": ");
        text.push_str(&next.to_string());
        cause = next.source();
    }
    text
}
