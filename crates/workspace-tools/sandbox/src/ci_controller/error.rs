use std::io;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::process::ExitStatus;

use super::disk::ParseDiskSizeError;
use super::github::ApiError;
use super::guest::GuestFailure;

/// Everything that ends a controller pass early, each keeping its cause.
///
/// A rejected local request is not here: it is an outcome the controller
/// reports in the request's result, not a failure of the pass.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{name} is not set")]
    EnvMissing { name: &'static str },
    #[error("{name}={value:?} is not a whole number")]
    EnvInvalid {
        name: &'static str,
        value: String,
        #[source]
        source: ParseIntError,
    },
    #[error("{name}={mib} MiB does not fit in 64-bit bytes")]
    EnvTooLarge { name: &'static str, mib: u64 },
    #[error("{name} is not valid")]
    EnvDiskSize {
        name: &'static str,
        #[source]
        source: ParseDiskSizeError,
    },
    #[error("cannot read {path}")]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("cannot {action} {path}")]
    Io {
        action: &'static str,
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("{path} has no MemAvailable line")]
    MemAvailableMissing { path: PathBuf },
    #[error("GitHub API")]
    Api(#[from] ApiError),
    #[error("cannot get the GitHub token from the token broker")]
    Broker(#[from] crate::broker::error::BrokerError),
    #[error("a controller-owned runner is still busy; refusing concurrent execution")]
    RunnerBusy,
    #[error("no seed work disk at {path}; run coda-ci-seed first")]
    SeedMissing { path: PathBuf },
    #[error("cannot run {program}")]
    Spawn {
        program: String,
        #[source]
        source: io::Error,
    },
    #[error("{program} {status}")]
    CommandFailed { program: String, status: ExitStatus },
    #[error(transparent)]
    Guest(#[from] GuestFailure),
    #[error("stopped by SIGTERM")]
    Terminated,
    /// The work failed and so did the cleanup after it; both are kept.
    #[error("{}; cleanup failed too: {}", chain(.error.as_ref()), chain(.cleanup.as_ref()))]
    CleanupAfterFailure {
        error: Box<Error>,
        cleanup: Box<Error>,
    },
}

/// An error and every cause under it, joined by `: `.
pub fn chain(error: &dyn std::error::Error) -> String {
    let mut text = error.to_string();
    let mut cause = error.source();
    while let Some(current) = cause {
        text.push_str(": ");
        text.push_str(&current.to_string());
        cause = current.source();
    }
    text
}

impl Error {
    /// The outcome of work followed by a cleanup that runs either way. A
    /// cleanup failure is never dropped: alone it is the error, and after a
    /// failed `work` both are reported.
    pub(crate) fn with_cleanup<T>(
        work: Result<T, Self>,
        cleanup: Result<(), Self>,
    ) -> Result<T, Self> {
        match (work, cleanup) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(error), Ok(())) | (Ok(_), Err(error)) => Err(error),
            (Err(error), Err(cleanup)) => Err(Self::CleanupAfterFailure {
                error: Box::new(error),
                cleanup: Box::new(cleanup),
            }),
        }
    }

    /// Whether SIGTERM ended this pass, alone or before a failed cleanup.
    pub fn is_terminated(&self) -> bool {
        matches!(self, Self::Terminated)
            || matches!(self, Self::CleanupAfterFailure { error, .. } if error.is_terminated())
    }

    pub(crate) fn io(
        action: &'static str,
        path: impl Into<PathBuf>,
    ) -> impl FnOnce(io::Error) -> Self {
        let path = path.into();
        move |source| Self::Io {
            action,
            path,
            source,
        }
    }
}
