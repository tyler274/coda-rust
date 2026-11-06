//! A single durable pending publication, not a queue. GCS generations serialize
//! claims; original metric points survive retries. No new observation is taken
//! until the preceding publication (including heartbeat) is acknowledged.
use crate::{Config, Credentials, Error, Observation, Target, state::State, *};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub const LEASE_SECONDS: u64 = 240;
pub const MAX_START_DELAY: u64 = 180;
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ledger {
    pub state: State,
    pub claim: Option<Claim>,
    pub pending: Option<Pending>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claim {
    pub slot: u64,
    pub owner: String,
    pub expires: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pending {
    pub slot: u64,
    pub points: Vec<Value>,
    pub acknowledged: usize,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Observe,
    Publish,
    Done,
}
impl Ledger {
    pub fn claim(&mut self, slot: u64, now: u64, owner: &str) -> Result<Action, Error> {
        if let Some(claim) = &self.claim
            && claim.expires > now
        {
            return Err("another execution owns the durable claim".into());
        }
        // Publication is recovered even by a stale invocation. It never creates
        // new observations or invents a fresh timestamp for an old heartbeat.
        if let Some(pending) = &self.pending {
            self.claim = Some(Claim {
                slot: pending.slot,
                owner: owner.into(),
                expires: now + LEASE_SECONDS,
            });
            return Ok(Action::Publish);
        }
        if self.state.minute.is_some_and(|minute| slot / 60 <= minute) {
            return Ok(Action::Done);
        }
        // An abandoned probe cannot be re-observed: that would replace its
        // scheduled measurement with a later one. Consume it as missing.
        if let Some(abandoned) = self.claim.take() {
            self.state.advance(abandoned.slot / 60, &[]);
            if slot <= abandoned.slot {
                return Ok(Action::Done);
            }
        }
        if !slot.is_multiple_of(60) || slot > now || now - slot > MAX_START_DELAY {
            return Err("scheduled slot is stale, future or not minute-aligned".into());
        }
        self.claim = Some(Claim {
            slot,
            owner: owner.into(),
            expires: now + LEASE_SECONDS,
        });
        Ok(Action::Observe)
    }
    pub fn observed(
        &mut self,
        project: &str,
        observations: &[Observation],
        timestamp: u64,
    ) -> Result<(), Error> {
        let slot = self.claim.as_ref().ok_or("missing claim")?.slot;
        if !self.state.advance(slot / 60, observations) {
            return Err("out-of-order observation".into());
        }
        let end = chrono::DateTime::from_timestamp(
            i64::try_from(slot).map_err(|e| Error::caused("invalid scheduled timestamp", e))?,
            0,
        )
        .ok_or("invalid observation timestamp")?
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let mut points = Vec::new();
        for (index, target) in Target::ALL.iter().enumerate() {
            if let Some(o) = observations.iter().find(|o| o.target == *target) {
                points.push(metric_point(
                    project,
                    READINESS_METRIC,
                    json!({"target": target.label(), "outcome": o.outcome.label()}),
                    i64::from(o.outcome.failed()),
                    &end,
                ));
                for (metric, value) in [
                    (ALERT_METRIC, i64::from(self.state.targets[index].firing)),
                    (
                        LATENCY_METRIC,
                        i64::try_from(o.latency_ms).unwrap_or(i64::MAX),
                    ),
                    (
                        FRESHNESS_METRIC,
                        i64::try_from(timestamp).unwrap_or(i64::MAX),
                    ),
                ] {
                    points.push(metric_point(
                        project,
                        metric,
                        json!({"target": target.label()}),
                        value,
                        &end,
                    ));
                }
            }
        }
        points.push(metric_point(project, HEARTBEAT_METRIC, json!({}), 1, &end));
        self.pending = Some(Pending {
            slot,
            points,
            acknowledged: 0,
        });
        Ok(())
    }
}

struct Store<'a> {
    config: &'a Config,
    credentials: &'a Credentials,
    access: &'a str,
    bucket: &'a str,
}
impl Store<'_> {
    async fn load(&self) -> Result<(Ledger, String), Error> {
        let response = self
            .credentials
            .client
            .get(format!(
                "{}/storage/v1/b/{}/o/state.json",
                self.config.endpoints.storage, self.bucket
            ))
            .query(&[("alt", "media")])
            .bearer_auth(self.access)
            .send()
            .await
            .map_err(|e| Error::caused("state read failed", e))?;
        if response.status() == StatusCode::NOT_FOUND {
            return Ok((Ledger::default(), "0".into()));
        }
        let generation = response
            .headers()
            .get("x-goog-generation")
            .and_then(|v| v.to_str().ok())
            .ok_or("state generation missing")?
            .to_owned();
        let bytes = bounded_body(response, 128 * 1024).await?;
        let ledger = serde_json::from_slice(&bytes)
            .map_err(|e| Error::caused("state decoding failed", e))?;
        Ok((ledger, generation))
    }
    async fn save(&self, ledger: &Ledger, generation: &str) -> Result<String, Error> {
        // GCS limits replacement of one object to once per second. Include
        // pacing on the first write too, since a prior invocation may have
        // just released its claim. Retrying keeps the SAME generation guard.
        for attempt in 0..4u32 {
            tokio::time::sleep(std::time::Duration::from_millis(1100 * (1u64 << attempt))).await;
            match self.save_once(ledger, generation).await? {
                Some(generation) => return Ok(generation),
                None => continue,
            }
        }
        Err("state write throttled after bounded retries".into())
    }
    async fn save_once(&self, ledger: &Ledger, generation: &str) -> Result<Option<String>, Error> {
        let response = self
            .credentials
            .client
            .post(format!(
                "{}/upload/storage/v1/b/{}/o",
                self.config.endpoints.storage, self.bucket
            ))
            .query(&[
                ("uploadType", "media"),
                ("name", "state.json"),
                ("ifGenerationMatch", generation),
            ])
            .bearer_auth(self.access)
            .json(ledger)
            .send()
            .await
            .map_err(|e| Error::caused("state write failed", e))?;
        if response.status() == StatusCode::PRECONDITION_FAILED {
            return Err("state CAS conflict; retry without recounting".into());
        }
        if response.status() == StatusCode::TOO_MANY_REQUESTS || response.status().is_server_error()
        {
            return Ok(None);
        }
        let bytes = bounded_body(response, 128 * 1024).await?;
        let metadata: Value = serde_json::from_slice(&bytes)
            .map_err(|e| Error::caused("state metadata decoding failed", e))?;
        metadata["generation"]
            .as_str()
            .map(|value| Some(value.to_owned()))
            .ok_or_else(|| "saved state generation missing".into())
    }
}

/// One point per request avoids ambiguous partial batches. On an unsuccessful
/// write (including a lost response), verify the exact original point through
/// timeSeries.list before acknowledging. Never treat generic 400/409 as success.
pub async fn send_point(
    config: &Config,
    credentials: &Credentials,
    access: &str,
    point: &Value,
) -> Result<(), Error> {
    let response = credentials
        .client
        .post(format!(
            "{}/{}/timeSeries",
            config.endpoints.monitoring, config.project
        ))
        .bearer_auth(access)
        .json(&json!({"timeSeries": [point]}))
        .send()
        .await;
    let error = match response {
        Ok(response) if response.status().is_success() => return Ok(()),
        Ok(response) => Error::status("timeSeries.create", response.status()),
        Err(error) => Error::caused("timeSeries.create failed", error),
    };
    if point_exists(config, credentials, access, point).await? {
        Ok(())
    } else {
        Err(error)
    }
}
async fn point_exists(
    config: &Config,
    credentials: &Credentials,
    access: &str,
    point: &Value,
) -> Result<bool, Error> {
    let metric = point["metric"]["type"].as_str().ok_or("invalid metric")?;
    let end = point["points"][0]["interval"]["endTime"]
        .as_str()
        .ok_or("invalid point timestamp")?;
    let timestamp = chrono::DateTime::parse_from_rfc3339(end)
        .map_err(|e| Error::caused("invalid saved timestamp", e))?;
    let start = (timestamp - chrono::Duration::seconds(1))
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    // All filter components come from fixed metric/label names or validated
    // saved operational state, never service responses or user requests.
    let filter = format!(
        "metric.type = {} AND resource.type = \"global\"",
        serde_json::to_string(metric)
            .map_err(|e| Error::caused("metric filter encoding failed", e))?
    );
    let response = bounded_json(
        credentials
            .client
            .get(format!(
                "{}/{}/timeSeries",
                config.endpoints.monitoring, config.project
            ))
            .bearer_auth(access)
            .query(&[
                ("filter", filter.as_str()),
                ("interval.startTime", start.as_str()),
                ("interval.endTime", end),
                ("view", "FULL"),
                ("pageSize", "100"),
            ]),
    )
    .await?;
    Ok(response["timeSeries"].as_array().is_some_and(|series| {
        series.iter().any(|s| {
            s["metric"] == point["metric"]
                && s["resource"] == point["resource"]
                && s["points"].as_array().is_some_and(|points| {
                    points.iter().any(|p| {
                        p["value"] == point["points"][0]["value"]
                            && p["interval"]["endTime"] == point["points"][0]["interval"]["endTime"]
                    })
                })
        })
    }))
}

pub async fn run_slot(config: Config, slot: u64, owner: String) -> Result<(), Error> {
    // The lease exceeds the complete execution lifetime, including every API
    // timeout. A previous process cannot still publish when a successor claims.
    tokio::time::timeout(
        std::time::Duration::from_secs(120),
        execute(&config, slot, &owner),
    )
    .await
    .map_err(|e| Error::caused("execution deadline exceeded", e))?
}
async fn execute(config: &Config, slot: u64, owner: &str) -> Result<(), Error> {
    let credentials = Credentials::new(config.service_account.clone(), config.endpoints.clone())?;
    let access = credentials.access_token().await?;
    let store = Store {
        config,
        credentials: &credentials,
        access: &access,
        bucket: config
            .state_bucket
            .as_deref()
            .ok_or("state bucket required")?,
    };
    let (mut ledger, mut generation) = store.load().await?;
    let action = ledger.claim(slot, unix_now(), owner)?;
    if action == Action::Done {
        return Ok(());
    }
    generation = store.save(&ledger, &generation).await?;
    if action == Action::Observe {
        let observations = probe_all(config, &credentials, &probe_client(config.timeout)?).await;
        ledger.observed(&config.project, &observations, unix_now())?;
        generation = store.save(&ledger, &generation).await?;
    }
    while let Some(pending) = &ledger.pending {
        let Some(point) = pending.points.get(pending.acknowledged).cloned() else {
            break;
        };
        // Only this lease holder may progress; failed requests retain the exact
        // point and cursor. A later execution resumes, not re-probes.
        if let Err(error) = send_point(config, &credentials, &access, &point).await {
            ledger.claim = None;
            store.save(&ledger, &generation).await?;
            return Err(error);
        }
        ledger
            .pending
            .as_mut()
            .ok_or("pending publication missing")?
            .acknowledged += 1;
        generation = store.save(&ledger, &generation).await?;
    }
    ledger.pending = None;
    ledger.claim = None;
    store.save(&ledger, &generation).await?;
    // Recovering another slot consumes this invocation. Its own slot is not
    // sampled late or counted as success; the resulting gap breaks the streak.
    Ok(())
}
