//! Cloud Scheduler OIDC -> Cloud Run IAM -> this adapter -> Jobs run override.
//! Scheduler's documented ScheduleTime is constant across retries. Cloud Run
//! Jobs itself provides no schedule timestamp; never infer one from its clock.
use crate::{Credentials, Endpoints, Error};
use axum::{
    Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::post,
};
use serde_json::{Value, json};
use std::sync::Arc;

/// Scheduler documents the header only as "the job name". Accept the full
/// `projects/P/locations/L/jobs/J` resource name or its final `J` segment, and
/// name what arrived otherwise. A job name is not secret.
fn is_expected_job(received: &str, expected_job: &str) -> bool {
    received == expected_job
        || expected_job.rsplit_once("/jobs/").map(|(_, id)| id) == Some(received)
}
pub fn scheduled_slot(headers: &HeaderMap, expected_job: &str) -> Result<u64, Error> {
    let received = headers
        .get("x-cloudscheduler-jobname")
        .ok_or("missing scheduler job name")?
        .to_str()
        .map_err(|e| Error::caused("scheduler job name is not text", e))?;
    if !is_expected_job(received, expected_job) {
        let shown: String = received.chars().take(200).collect();
        return Err(format!("unexpected scheduler job {shown:?}").into());
    }
    let value = headers
        .get("x-cloudscheduler-scheduletime")
        .and_then(|v| v.to_str().ok())
        .ok_or("missing scheduler time")?;
    let timestamp = chrono::DateTime::parse_from_rfc3339(value)
        .map_err(|e| Error::caused("invalid scheduler time", e))?;
    if timestamp.timestamp_subsec_nanos() != 0 {
        return Err("scheduler time must be a whole minute".into());
    }
    let slot = u64::try_from(timestamp.timestamp())
        .map_err(|e| Error::caused("invalid scheduler epoch", e))?;
    if !slot.is_multiple_of(60) {
        return Err("scheduler time must be a whole minute".into());
    }
    Ok(slot)
}
pub fn overrides(slot: u64) -> Value {
    json!({"overrides": {"containerOverrides": [{"env": [{"name": "AVAILABILITY_PROBE_SLOT", "value": slot.to_string()}]}]}})
}
struct Dispatch {
    credentials: Credentials,
    scheduler: String,
    job_url: String,
}
async fn launch(State(state): State<Arc<Dispatch>>, headers: HeaderMap) -> StatusCode {
    let result = async {
        let slot = scheduled_slot(&headers, &state.scheduler)?;
        let access = state.credentials.access_token().await?;
        let response = state
            .credentials
            .client
            .post(&state.job_url)
            .bearer_auth(access)
            .json(&overrides(slot))
            .send()
            .await
            .map_err(|e| Error::caused("job launch failed", e))?;
        if !response.status().is_success() {
            return Err(Error::status("job launch", response.status()));
        }
        Ok::<(), Error>(())
    }
    .await;
    match result {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(error) => {
            eprintln!(
                "{}",
                json!({"severity": "ERROR", "message": error.to_string()})
            );
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}
pub async fn serve() -> Result<(), Error> {
    let required =
        |key: &str| std::env::var(key).map_err(|e| Error::caused(format!("{key} is required"), e));
    let state = Dispatch {
        credentials: Credentials::new(String::new(), Endpoints::default())?,
        scheduler: required("AVAILABILITY_SCHEDULER_JOB")?,
        job_url: required("AVAILABILITY_JOB_RUN_URL")?,
    };
    let port = required("PORT")?
        .parse::<u16>()
        .map_err(|e| Error::caused("invalid listener port", e))?;
    let listener = tokio::net::TcpListener::bind((std::net::Ipv4Addr::UNSPECIFIED, port))
        .await
        .map_err(|e| Error::caused("dispatcher listener failed", e))?;
    axum::serve(
        listener,
        Router::new()
            .route("/dispatch", post(launch))
            .with_state(Arc::new(state)),
    )
    .await
    .map_err(|e| Error::caused("dispatcher server failed", e))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn retry_stable_header_becomes_explicit_job_override() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-cloudscheduler-jobname",
            "projects/p/locations/r/jobs/probe".parse().unwrap(),
        );
        headers.insert(
            "x-cloudscheduler-scheduletime",
            "2026-11-06T00:01:00Z".parse().unwrap(),
        );
        let slot = scheduled_slot(&headers, "projects/p/locations/r/jobs/probe").unwrap();
        assert_eq!(
            scheduled_slot(&headers, "projects/p/locations/r/jobs/probe").unwrap(),
            slot
        );
        assert_eq!(
            overrides(slot)["overrides"]["containerOverrides"][0]["env"][0]["value"],
            slot.to_string()
        );
        assert!(scheduled_slot(&headers, "other").is_err());
        headers.insert("x-cloudscheduler-jobname", "probe".parse().unwrap());
        assert_eq!(
            scheduled_slot(&headers, "projects/p/locations/r/jobs/probe").unwrap(),
            slot
        );
        headers.insert("x-cloudscheduler-jobname", "jobs/probe".parse().unwrap());
        let error = scheduled_slot(&headers, "projects/p/locations/r/jobs/probe").unwrap_err();
        assert_eq!(error.to_string(), "unexpected scheduler job \"jobs/probe\"");
        headers.insert("x-cloudscheduler-jobname", "other-probe".parse().unwrap());
        assert!(scheduled_slot(&headers, "projects/p/locations/r/jobs/probe").is_err());
        headers.remove("x-cloudscheduler-jobname");
        let error = scheduled_slot(&headers, "projects/p/locations/r/jobs/probe").unwrap_err();
        assert_eq!(error.to_string(), "missing scheduler job name");
        headers.insert(
            "x-cloudscheduler-jobname",
            "projects/p/locations/r/jobs/probe".parse().unwrap(),
        );
        headers.remove("x-cloudscheduler-scheduletime");
        assert!(scheduled_slot(&headers, "projects/p/locations/r/jobs/probe").is_err());
    }
}
