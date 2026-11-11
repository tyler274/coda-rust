use std::io;
use std::path::PathBuf;
use std::process::ExitStatus;

/// Everything that can go wrong in `coda-sandbox`, each keeping its cause.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no config at {path}")]
    ConfigMissing {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("cannot read config {path}")]
    ConfigRead {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("config {path} is not valid")]
    ConfigParse {
        path: PathBuf,
        #[source]
        source: Box<toml::de::Error>,
    },
    #[error("cannot inspect {path}")]
    Inspect {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("HOME is not set, so editor settings cannot be found")]
    NoHome,
    #[error("cannot run {program}")]
    Spawn {
        program: &'static str,
        #[source]
        source: io::Error,
    },
    #[error("cannot wait for {program}")]
    Wait {
        program: &'static str,
        #[source]
        source: io::Error,
    },
    #[error("{program} {status}: {stderr}")]
    CommandFailed {
        program: &'static str,
        status: ExitStatus,
        stderr: String,
    },
    #[error("unexpected {what} from {source_name}: {found:?}")]
    UnexpectedOutput {
        what: &'static str,
        source_name: String,
        found: String,
    },
    #[error("{path} is not valid JSON with comments")]
    SettingsParse {
        path: PathBuf,
        #[source]
        source: json5::Error,
    },
    #[error("cannot encode status as JSON")]
    Json(#[source] serde_json::Error),
    #[error("cannot write to standard output")]
    Output(#[source] io::Error),
}

impl Error {
    /// The error and each of its causes, joined for one line of output.
    pub fn chain(&self) -> String {
        let mut text = self.to_string();
        let mut cause = std::error::Error::source(self);
        while let Some(current) = cause {
            text.push_str(": ");
            text.push_str(&current.to_string());
            cause = current.source();
        }
        text
    }
}
