//! The queue's health as Cloud Monitoring gauges.
//!
//! Every minute a hosted worker reads [`QueueSignals`] from Postgres, where
//! the work lives, and writes three gauges on the `global` resource, the way
//! `availability-probe` writes its own: how long the longest-waiting due
//! work has been runnable, how much work is due, and how much ended failed.
//! The observability monitors alert on these. Each instance labels its
//! points with its own id, since two writers on one series would reject each
//! other's points; every instance reads the same rows, so monitors take the
//! maximum. The points carry counts and an age only, never a scope or text.
//! A worker outside Cloud Run writes nothing.
use std::time::Duration;

use local_product::runtime::Store;
use local_product::runtime::delivery::QueueSignals;
use reqwest::{Client, StatusCode};
use sea_orm::DbErr;
use serde_json::{Value, json};
use tokio::sync::watch;

use super::pull::stopping;

/// Seconds the longest-waiting due work has been runnable; 0 when none is due.
pub(super) const OLDEST_DUE_METRIC: &str =
    "custom.googleapis.com/coda/physician/queue/oldest_due_seconds";
/// Chat turns and reply revisions runnable now.
pub(super) const DUE_METRIC: &str = "custom.googleapis.com/coda/physician/queue/due";
/// Work that ended failed and waits for the physician's Try again.
pub(super) const FAILED_METRIC: &str = "custom.googleapis.com/coda/physician/queue/failed";

/// How often a worker reads and writes the gauges.
const INTERVAL: Duration = Duration::from_secs(60);
const METADATA: &str = "http://metadata.google.internal/computeMetadata/v1";
const MONITORING: &str = "https://monitoring.googleapis.com/v3/projects";

/// Why one round of gauges was not written.
#[derive(Debug, thiserror::Error)]
pub(super) enum SignalError {
    /// Postgres did not answer the signal read.
    #[error("queue signal read failed")]
    Read(#[source] DbErr),
    /// A metadata or monitoring request did not complete.
    #[error("queue signal request failed")]
    Request(#[source] reqwest::Error),
    /// The metadata server or Cloud Monitoring refused the request.
    #[error("{0} answered {1}")]
    Status(&'static str, StatusCode),
    /// The metadata token response named no access token.
    #[error("metadata token response has no access_token")]
    Token,
}

impl SignalError {
    /// The bounded `error.type` logged for this failure.
    fn kind(&self) -> &'static str {
        match self {
            Self::Read(_) => "database",
            Self::Request(_) => "request",
            Self::Status(..) => "status",
            Self::Token => "token",
        }
    }
}

/// The project and instance the gauges are written for.
struct Target {
    client: Client,
    project: String,
    instance: String,
}

/// Writes the gauges every [`INTERVAL`] until the worker stops. A round that
/// fails is logged and the next one tries again; nothing here stops the
/// worker.
pub(super) async fn run(store: Store, stop: watch::Receiver<bool>) {
    let target = loop {
        match Target::discover().await {
            Ok(target) => break target,
            Err(error) => {
                tracing::warn!(
                    event = "physician.queue.signals_failed",
                    error.type = error.kind(),
                    "physician worker could not find its project for queue signals"
                );
            }
        }
        tokio::select! {
            () = stopping(stop.clone()) => return,
            () = tokio::time::sleep(INTERVAL) => {}
        }
    };
    let mut tick = tokio::time::interval(INTERVAL);
    tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    loop {
        tokio::select! {
            () = stopping(stop.clone()) => return,
            _ = tick.tick() => {}
        }
        if let Err(error) = target.write(&store).await {
            tracing::warn!(
                event = "physician.queue.signals_failed",
                error.type = error.kind(),
                "physician worker queue signals not written"
            );
        }
    }
}

impl Target {
    async fn discover() -> Result<Self, SignalError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(SignalError::Request)?;
        let project = metadata(&client, "project/project-id").await?;
        let instance = metadata(&client, "instance/id").await?;
        Ok(Self {
            client,
            project,
            instance,
        })
    }

    async fn write(&self, store: &Store) -> Result<(), SignalError> {
        let signals = store.queue_signals().await.map_err(SignalError::Read)?;
        let token: Value = self
            .client
            .get(format!(
                "{METADATA}/instance/service-accounts/default/token"
            ))
            .header("Metadata-Flavor", "Google")
            .send()
            .await
            .map_err(SignalError::Request)?
            .error_for_status()
            .map_err(SignalError::Request)?
            .json()
            .await
            .map_err(SignalError::Request)?;
        let token = token["access_token"]
            .as_str()
            .filter(|token| !token.is_empty())
            .ok_or(SignalError::Token)?;
        let end = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let response = self
            .client
            .post(format!("{MONITORING}/{}/timeSeries", self.project))
            .bearer_auth(token)
            .json(&time_series(&self.project, &self.instance, signals, &end))
            .send()
            .await
            .map_err(SignalError::Request)?;
        if !response.status().is_success() {
            return Err(SignalError::Status("timeSeries.create", response.status()));
        }
        Ok(())
    }
}

async fn metadata(client: &Client, path: &str) -> Result<String, SignalError> {
    let response = client
        .get(format!("{METADATA}/{path}"))
        .header("Metadata-Flavor", "Google")
        .send()
        .await
        .map_err(SignalError::Request)?;
    if !response.status().is_success() {
        return Err(SignalError::Status("metadata", response.status()));
    }
    response.text().await.map_err(SignalError::Request)
}

/// The `timeSeries.create` body for one round of signals.
fn time_series(project: &str, instance: &str, signals: QueueSignals, end: &str) -> Value {
    let (due, oldest) = signals
        .due
        .map_or((0, 0), |due| (due.count.get(), due.oldest.as_secs()));
    let series = [
        (OLDEST_DUE_METRIC, oldest),
        (DUE_METRIC, due),
        (FAILED_METRIC, signals.failed),
    ]
    .map(|(metric, value)| {
        json!({
            "metric": { "type": metric, "labels": { "instance_id": instance } },
            "resource": { "type": "global", "labels": { "project_id": project } },
            "metricKind": "GAUGE",
            "valueType": "INT64",
            "points": [{
                "interval": { "endTime": end },
                "value": { "int64Value": value.to_string() },
            }],
        })
    });
    json!({ "timeSeries": series })
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use local_product::runtime::delivery::DueWork;

    use super::*;

    fn point(body: &Value, metric: &str) -> Value {
        body["timeSeries"]
            .as_array()
            .and_then(|series| series.iter().find(|s| s["metric"]["type"] == metric))
            .cloned()
            .unwrap_or(Value::Null)
    }

    #[test]
    fn gauges_carry_the_signals_on_the_global_resource_per_instance() {
        let signals = QueueSignals {
            due: Some(DueWork {
                count: NonZeroU64::new(3).unwrap(),
                oldest: Duration::from_millis(1_250_900),
            }),
            failed: 2,
        };
        let body = time_series("stg-project", "instance-7", signals, "2026-11-10T04:00:00Z");
        assert_eq!(body["timeSeries"].as_array().map(Vec::len), Some(3));
        for (metric, value) in [
            (OLDEST_DUE_METRIC, "1250"),
            (DUE_METRIC, "3"),
            (FAILED_METRIC, "2"),
        ] {
            let series = point(&body, metric);
            assert_eq!(
                series["points"][0]["value"]["int64Value"], value,
                "{metric}"
            );
            assert_eq!(
                series["points"][0]["interval"]["endTime"],
                "2026-11-10T04:00:00Z"
            );
            assert_eq!(series["metricKind"], "GAUGE");
            assert_eq!(series["valueType"], "INT64");
            assert_eq!(
                series["resource"],
                json!({ "type": "global", "labels": { "project_id": "stg-project" } })
            );
            // The instance is the only label: no scope, chat or patient.
            assert_eq!(
                series["metric"]["labels"],
                json!({ "instance_id": "instance-7" })
            );
        }
    }

    #[test]
    fn nothing_due_reports_an_age_of_zero() {
        let body = time_series("p", "i", QueueSignals::default(), "2026-11-10T04:00:00Z");
        for metric in [OLDEST_DUE_METRIC, DUE_METRIC, FAILED_METRIC] {
            assert_eq!(
                point(&body, metric)["points"][0]["value"]["int64Value"],
                "0"
            );
        }
    }
}
