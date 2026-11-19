//! The GitHub calls the controller makes: the queue, JIT runners, cleanup.

use std::time::Duration;

use serde::de::DeserializeOwned;

mod wire;

use crate::broker::Secret;

pub use self::wire::{Job, JobId, JobStatus, RegisteredRunner, Registration, Runner, RunnerId};
use self::wire::{JobState, Jobs, Runners, Runs};

pub const REPO: &str = "tyler274/coda-rust";
pub const LABEL: &str = "cyrene-coda";
pub const DEFAULT_API: &str = "https://api.github.com";

/// Six hex digits that tell two registrations for one job apart. Each
/// `RandomState` gets fresh keys from the process's random seed.
fn unique_suffix() -> String {
    use std::hash::{BuildHasher as _, Hasher as _};
    let mut hasher = std::collections::hash_map::RandomState::new().build_hasher();
    hasher.write_u128(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos(),
    );
    format!("{:06x}", hasher.finish() & 0xff_ffff)
}

fn runners_path() -> String {
    format!("repos/{REPO}/actions/runners")
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("{method} {path} returned HTTP {status}")]
    Status {
        method: &'static str,
        path: String,
        status: u16,
    },
    #[error("{method} {path} failed")]
    Transport {
        method: &'static str,
        path: String,
        #[source]
        source: Box<ureq::Error>,
    },
    #[error("{method} {path} returned an unexpected body")]
    Body {
        method: &'static str,
        path: String,
        #[source]
        source: serde_json::Error,
    },
}

/// The name of a runner this controller registered: `cyrene-coda-<job>-<hex>`.
pub fn is_controller_runner(name: &str) -> bool {
    let Some(rest) = name.strip_prefix("cyrene-coda-") else {
        return false;
    };
    let Some((job, suffix)) = rest.split_once('-') else {
        return false;
    };
    !job.is_empty()
        && job.bytes().all(|b| b.is_ascii_digit())
        && !suffix.is_empty()
        && suffix
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// An authenticated client for the GitHub REST API.
pub struct Api {
    agent: ureq::Agent,
    base: String,
    token: Secret,
}

impl Api {
    pub fn new(base: &str, token: Secret) -> Self {
        let agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(30)))
            .http_status_as_error(false)
            .build()
            .into();
        Self {
            agent,
            base: base.trim_end_matches('/').to_owned(),
            token,
        }
    }

    fn call<T: DeserializeOwned>(
        &self,
        method: &'static str,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<Option<T>, ApiError> {
        let url = format!("{}/{path}", self.base);
        let request = ureq::http::Request::builder()
            .method(method)
            .uri(&url)
            .header("Authorization", format!("Bearer {}", self.token.expose()))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28");
        let transport = |source| ApiError::Transport {
            method,
            path: path.to_owned(),
            source: Box::new(source),
        };
        let sent = match body {
            Some(value) => request
                .header("Content-Type", "application/json")
                .body(value.to_string())
                .map_err(ureq::Error::from)
                .and_then(|request| self.agent.run(request)),
            None => request
                .body(())
                .map_err(ureq::Error::from)
                .and_then(|request| self.agent.run(request)),
        };
        let mut response = sent.map_err(transport)?;
        let status = response.status().as_u16();
        if !(200..300).contains(&status) {
            return Err(ApiError::Status {
                method,
                path: path.to_owned(),
                status,
            });
        }
        let text = response.body_mut().read_to_string().map_err(transport)?;
        if text.is_empty() {
            return Ok(None);
        }
        serde_json::from_str(&text)
            .map(Some)
            .map_err(|source| ApiError::Body {
                method,
                path: path.to_owned(),
                source,
            })
    }

    fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiError> {
        let body = self.call::<T>("GET", path, None)?;
        body.ok_or_else(|| ApiError::Body {
            method: "GET",
            path: path.to_owned(),
            source: serde::de::Error::custom("empty body"),
        })
    }

    /// Every runner on the repository, page by page.
    pub fn runners(&self) -> Result<Vec<Runner>, ApiError> {
        let mut all = Vec::new();
        for page in 1.. {
            let runners = self
                .get::<Runners>(&format!("{}?per_page=100&page={page}", runners_path()))?
                .runners;
            let last = runners.len() < 100;
            all.extend(runners);
            if last {
                break;
            }
        }
        Ok(all)
    }

    /// Delete a runner; one already gone counts as deleted.
    pub fn delete_runner(&self, id: RunnerId) -> Result<(), ApiError> {
        match self.call::<serde_json::Value>("DELETE", &format!("{}/{id}", runners_path()), None) {
            Ok(_) | Err(ApiError::Status { status: 404, .. }) => Ok(()),
            Err(error) => Err(error),
        }
    }

    /// The oldest queued job whose labels ask for this machine.
    pub fn next_job(&self) -> Result<Option<Job>, ApiError> {
        let mut runs = self
            .get::<Runs>(&format!(
                "repos/{REPO}/actions/runs?status=queued&per_page=50"
            ))?
            .workflow_runs;
        runs.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        for run in runs {
            let path = format!(
                "repos/{REPO}/actions/runs/{}/jobs?filter=latest&per_page=100",
                run.id
            );
            let found = self.get::<Jobs>(&path)?.jobs.into_iter().find(|job| {
                job.status == JobStatus::Queued && job.labels.iter().any(|label| label == LABEL)
            });
            if found.is_some() {
                return Ok(found);
            }
        }
        Ok(None)
    }

    /// Register a one-job runner under a fresh name.
    pub fn register(&self, job_id: JobId) -> Result<Registration, ApiError> {
        let body = serde_json::json!({
            "name": format!("cyrene-coda-{job_id}-{}", unique_suffix()),
            "runner_group_id": 1,
            "labels": [LABEL],
            "work_folder": "_work",
        });
        let path = format!("{}/generate-jitconfig", runners_path());
        let registration = self.call::<Registration>("POST", &path, Some(body))?;
        registration.ok_or_else(|| ApiError::Body {
            method: "POST",
            path,
            source: serde::de::Error::custom("empty body"),
        })
    }

    pub fn runner_busy(&self, id: RunnerId) -> Result<bool, ApiError> {
        Ok(self
            .get::<Runner>(&format!("{}/{id}", runners_path()))?
            .busy)
    }

    pub fn job_queued(&self, id: JobId) -> Result<bool, ApiError> {
        let state = self.get::<JobState>(&format!("repos/{REPO}/actions/jobs/{id}"))?;
        Ok(state.status == JobStatus::Queued)
    }
}
