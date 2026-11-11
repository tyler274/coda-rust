//! The parts of GitHub's REST responses the controller reads.

use std::fmt;

use serde::Deserialize;

/// The controller only asks whether a job still waits for a runner. Any other
/// status GitHub reports, today's or a future one, keeps its text for logs.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(from = "String")]
pub enum JobStatus {
    Queued,
    NotQueued(String),
}

impl From<String> for JobStatus {
    fn from(status: String) -> Self {
        if status == "queued" {
            Self::Queued
        } else {
            Self::NotQueued(status)
        }
    }
}

/// A GitHub Actions job id; never a runner's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct JobId(pub u64);

/// A self-hosted runner id; never a job's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct RunnerId(pub u64);

impl fmt::Display for JobId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for RunnerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Job {
    pub id: JobId,
    pub name: String,
    pub status: JobStatus,
    pub labels: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Runner {
    pub id: RunnerId,
    pub name: String,
    pub busy: bool,
}

#[derive(Debug, Deserialize)]
pub struct RegisteredRunner {
    pub id: RunnerId,
}

/// A JIT registration. Its `Debug` never shows the runner's credential.
#[derive(Deserialize)]
pub struct Registration {
    pub runner: RegisteredRunner,
    pub encoded_jit_config: String,
}

impl fmt::Debug for Registration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Registration")
            .field("runner", &self.runner)
            .field("encoded_jit_config", &"..")
            .finish()
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct Runners {
    pub runners: Vec<Runner>,
}

#[derive(Debug, Deserialize)]
pub(super) struct Run {
    pub id: u64,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct Runs {
    pub workflow_runs: Vec<Run>,
}

#[derive(Debug, Deserialize)]
pub(super) struct Jobs {
    pub jobs: Vec<Job>,
}

#[derive(Debug, Deserialize)]
pub(super) struct JobState {
    pub status: JobStatus,
}
