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

/// Location and job id of a `projects/P/locations/L/jobs/J` resource name.
fn job_path(name: &str) -> Option<(&str, &str)> {
    match name.split('/').collect::<Vec<_>>()[..] {
        ["projects", project, "locations", location, "jobs", id]
            if !project.is_empty() && !location.is_empty() && !id.is_empty() =>
        {
            Some((location, id))
        }
        _ => None,
    }
}
/// Scheduler documents the header only as "the job name". Accept the bare job
/// id, or a resource name with the same location and id, whether it names the
/// project by id or by number. Cloud Run IAM already limits callers to the
/// scheduler identity, so the project segment adds no protection here.
fn is_expected_job(received: &str, expected_job: &str) -> bool {
    let Some((location, id)) = job_path(expected_job) else {
        return false;
    };
    received == id || job_path(received) == Some((location, id))
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
        headers.remove("x-cloudscheduler-scheduletime");
        assert!(scheduled_slot(&headers, "projects/p/locations/r/jobs/probe").is_err());
    }
    #[test]
    fn scheduler_job_name_matches_id_or_same_location_resource() {
        let expected = "projects/p/locations/r/jobs/probe";
        let slot_for = |name: &[u8]| {
            let mut headers = HeaderMap::new();
            headers.insert(
                "x-cloudscheduler-jobname",
                axum::http::HeaderValue::from_bytes(name).unwrap(),
            );
            headers.insert(
                "x-cloudscheduler-scheduletime",
                "2026-11-06T00:01:00Z".parse().unwrap(),
            );
            scheduled_slot(&headers, expected).map_err(|e| e.to_string())
        };
        for accepted in [
            &b"probe"[..],
            b"projects/p/locations/r/jobs/probe",
            b"projects/123456/locations/r/jobs/probe",
        ] {
            assert!(slot_for(accepted).is_ok(), "{accepted:?}");
        }
        for rejected in [
            &b"jobs/probe"[..],
            b"r/jobs/probe",
            b"other-probe",
            b"projects/p/locations/other/jobs/probe",
            b"projects//locations/r/jobs/probe",
            b"",
        ] {
            assert!(slot_for(rejected).is_err(), "{rejected:?}");
        }
        assert_eq!(
            slot_for(b"jobs/probe").unwrap_err(),
            "unexpected scheduler job \"jobs/probe\""
        );
        assert_eq!(
            slot_for(&[0xff]).unwrap_err(),
            "scheduler job name is not text"
        );
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-cloudscheduler-scheduletime",
            "2026-11-06T00:01:00Z".parse().unwrap(),
        );
        assert_eq!(
            scheduled_slot(&headers, expected).unwrap_err().to_string(),
            "missing scheduler job name"
        );
        assert!(!is_expected_job("probe", "probe"));
    }
}
