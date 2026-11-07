use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

pub(super) const MAX_EXERCISE_SECONDS: i64 = 40 * 60;
pub(super) const MAX_REMINDERS: usize = 2;

const SOURCE_SYNTHETIC: &str = "synthetic-fixture";
const SOURCE_EVALUATOR: &str = "evaluator-history";
const SOURCE_PROVIDER: &str = "provider-metadata";
const SOURCE_SLACK: &str = "human-confirmed-slack";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DrillRoute {
    Datadog,
    Grafana,
    Watchdog,
}

impl DrillRoute {
    pub(super) fn parse(value: &str) -> Option<Self> {
        match value {
            "datadog" => Some(Self::Datadog),
            "grafana" => Some(Self::Grafana),
            "watchdog" => Some(Self::Watchdog),
            _ => None,
        }
    }

    pub(super) fn slug(self) -> &'static str {
        match self {
            Self::Datadog => "datadog",
            Self::Grafana => "grafana",
            Self::Watchdog => "watchdog",
        }
    }

    pub(super) fn observation_route(self) -> ObservationRoute {
        match self {
            Self::Datadog => ObservationRoute::Datadog,
            Self::Grafana => ObservationRoute::GrafanaTest,
            Self::Watchdog => ObservationRoute::CloudMonitoring,
        }
    }

    fn default_group(self) -> &'static str {
        match self {
            Self::Datadog => "staging/synthetic-qualification/datadog",
            Self::Grafana => "staging/synthetic-qualification/grafana",
            Self::Watchdog => "staging/synthetic-qualification/watchdog",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DrillAction {
    Fail,
    Recover,
    Status,
}

impl DrillAction {
    pub(super) fn parse(value: &str) -> Option<Self> {
        match value {
            "fail" => Some(Self::Fail),
            "recover" => Some(Self::Recover),
            "status" => Some(Self::Status),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct DrillRequest {
    pub(super) route: DrillRoute,
    pub(super) action: DrillAction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ObservationRoute {
    Datadog,
    GrafanaTest,
    CloudMonitoring,
}

impl ObservationRoute {
    fn as_str(self) -> &'static str {
        match self {
            Self::Datadog => "datadog",
            Self::GrafanaTest => "grafana-test",
            Self::CloudMonitoring => "cloud-monitoring",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value {
            "datadog" => Some(Self::Datadog),
            "grafana-test" => Some(Self::GrafanaTest),
            "cloud-monitoring" => Some(Self::CloudMonitoring),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SignalKind {
    Failure,
    Restored,
}

impl SignalKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Failure => "signal_failure",
            Self::Restored => "signal_restored",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum EvalKind {
    Pending,
    Firing,
    Recovered,
}

impl EvalKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Firing => "firing",
            Self::Recovered => "recovered",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum NoticeKind {
    SlackAlert,
    SlackRecovery,
    Reminder,
}

impl NoticeKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::SlackAlert => "slack_alert",
            Self::SlackRecovery => "slack_recovery",
            Self::Reminder => "reminder",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MaintenanceKind {
    Start,
    End,
}

impl MaintenanceKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "maintenance_start",
            Self::End => "maintenance_end",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Observation {
    Signal {
        route: ObservationRoute,
        kind: SignalKind,
        at: DateTime<Utc>,
        group: String,
        evidence: String,
        source: String,
    },
    Evaluator {
        route: ObservationRoute,
        kind: EvalKind,
        at: DateTime<Utc>,
        group: String,
        evidence: String,
    },
    Notice {
        route: ObservationRoute,
        kind: NoticeKind,
        at: DateTime<Utc>,
        group: String,
        evidence: String,
        confirmed_by: String,
        message_ref: String,
    },
    Provider {
        route: ObservationRoute,
        at: DateTime<Utc>,
        group: String,
        evidence: String,
    },
    Maintenance {
        route: ObservationRoute,
        kind: MaintenanceKind,
        at: DateTime<Utc>,
        group: String,
        evidence: String,
        source: String,
    },
}

impl Observation {
    pub(super) fn signal(
        route: ObservationRoute,
        kind: SignalKind,
        at: DateTime<Utc>,
        group: impl Into<String>,
        evidence: impl Into<String>,
    ) -> Self {
        Self::Signal {
            route,
            kind,
            at,
            group: group.into(),
            evidence: evidence.into(),
            source: SOURCE_SYNTHETIC.to_string(),
        }
    }

    #[cfg(test)]
    pub(super) fn evaluator(
        route: ObservationRoute,
        kind: EvalKind,
        at: DateTime<Utc>,
        group: impl Into<String>,
        evidence: impl Into<String>,
    ) -> Self {
        Self::Evaluator {
            route,
            kind,
            at,
            group: group.into(),
            evidence: evidence.into(),
        }
    }

    #[cfg(test)]
    pub(super) fn notice(
        route: ObservationRoute,
        kind: NoticeKind,
        at: DateTime<Utc>,
        group: impl Into<String>,
        evidence: impl Into<String>,
        confirmed_by: impl Into<String>,
        message_ref: impl Into<String>,
    ) -> Self {
        Self::Notice {
            route,
            kind,
            at,
            group: group.into(),
            evidence: evidence.into(),
            confirmed_by: confirmed_by.into(),
            message_ref: message_ref.into(),
        }
    }

    #[cfg(test)]
    pub(super) fn provider(
        route: ObservationRoute,
        at: DateTime<Utc>,
        group: impl Into<String>,
        evidence: impl Into<String>,
    ) -> Self {
        Self::Provider {
            route,
            at,
            group: group.into(),
            evidence: evidence.into(),
        }
    }

    fn at(&self) -> DateTime<Utc> {
        match self {
            Self::Signal { at, .. }
            | Self::Evaluator { at, .. }
            | Self::Notice { at, .. }
            | Self::Provider { at, .. }
            | Self::Maintenance { at, .. } => *at,
        }
    }

    fn route(&self) -> ObservationRoute {
        match self {
            Self::Signal { route, .. }
            | Self::Evaluator { route, .. }
            | Self::Notice { route, .. }
            | Self::Provider { route, .. }
            | Self::Maintenance { route, .. } => *route,
        }
    }

    fn group(&self) -> &str {
        match self {
            Self::Signal { group, .. }
            | Self::Evaluator { group, .. }
            | Self::Notice { group, .. }
            | Self::Provider { group, .. }
            | Self::Maintenance { group, .. } => group,
        }
    }

    fn is_failure(&self) -> bool {
        matches!(
            self,
            Self::Signal {
                kind: SignalKind::Failure,
                ..
            }
        )
    }

    fn is_restored(&self) -> bool {
        matches!(
            self,
            Self::Signal {
                kind: SignalKind::Restored,
                ..
            }
        )
    }

    fn is_reminder(&self) -> bool {
        matches!(
            self,
            Self::Notice {
                kind: NoticeKind::Reminder,
                ..
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Breach {
    Duration,
    Reminders,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Exercise {
    route: DrillRoute,
    group: String,
    observations: Vec<Observation>,
}

impl Exercise {
    pub(super) fn idle(route: DrillRoute) -> Self {
        Self {
            route,
            group: route.default_group().to_string(),
            observations: Vec::new(),
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }

    pub(super) fn is_open(&self) -> bool {
        self.open_since().is_some()
    }

    pub(super) fn breach(&self, now: DateTime<Utc>) -> Option<Breach> {
        if self.reminder_count() > MAX_REMINDERS {
            return Some(Breach::Reminders);
        }
        let over = self
            .open_since()
            .is_some_and(|opened| over_duration(opened, now));
        over.then_some(Breach::Duration)
    }

    pub(super) fn record_signal(&mut self, kind: SignalKind, at: DateTime<Utc>, evidence: &str) {
        self.observations.push(Observation::signal(
            self.route.observation_route(),
            kind,
            at,
            self.group.clone(),
            evidence,
        ));
    }

    #[cfg(test)]
    pub(super) fn record_reminder(
        &mut self,
        at: DateTime<Utc>,
        evidence: &str,
        confirmed_by: &str,
        message_ref: &str,
    ) -> Result<(), Breach> {
        if self.reminder_count() >= MAX_REMINDERS {
            return Err(Breach::Reminders);
        }
        self.observations.push(Observation::notice(
            self.route.observation_route(),
            NoticeKind::Reminder,
            at,
            self.group.clone(),
            evidence,
            confirmed_by,
            message_ref,
        ));
        Ok(())
    }

    pub(super) fn to_json(&self) -> Result<String, serde_json::Error> {
        observations_to_json(&self.observations)
    }

    fn open_since(&self) -> Option<DateTime<Utc>> {
        let mut opened = None;
        for observation in &self.observations {
            if observation.is_failure() {
                opened = Some(observation.at());
            } else if observation.is_restored() {
                opened = None;
            }
        }
        opened
    }

    fn reminder_count(&self) -> usize {
        self.observations
            .iter()
            .filter(|observation| observation.is_reminder())
            .count()
    }
}

fn over_duration(opened: DateTime<Utc>, now: DateTime<Utc>) -> bool {
    now.signed_duration_since(opened) > Duration::seconds(MAX_EXERCISE_SECONDS)
}

pub(super) fn observations_to_json(
    observations: &[Observation],
) -> Result<String, serde_json::Error> {
    let wires: Vec<Wire> = observations.iter().map(Wire::from).collect();
    serde_json::to_string_pretty(&wires)
}

pub(super) fn parse_exercise(route: DrillRoute, text: &str) -> Result<Exercise, String> {
    let wires: Vec<Wire> = serde_json::from_str(text)
        .map_err(|error| format!("observation JSON is invalid ({error})"))?;
    if wires.is_empty() {
        return Ok(Exercise::idle(route));
    }
    let mut observations = Vec::with_capacity(wires.len());
    for wire in wires {
        observations.push(Observation::try_from(wire)?);
    }
    let expected = route.observation_route();
    if observations
        .iter()
        .any(|observation| observation.route() != expected)
    {
        return Err(format!("observation route is not {}", expected.as_str()));
    }
    let group = observations[0].group().to_string();
    if observations
        .iter()
        .any(|observation| observation.group() != group)
    {
        return Err("observation group changed within the exercise".to_string());
    }
    let mut previous = None;
    for observation in &observations {
        if let Some(earlier) = previous
            && observation.at() < earlier
        {
            return Err("observations must be chronological".to_string());
        }
        previous = Some(observation.at());
    }
    Ok(Exercise {
        route,
        group,
        observations,
    })
}

pub(super) fn format_stamp(at: DateTime<Utc>) -> String {
    let nanos = at.timestamp_subsec_nanos();
    let base = at.format("%Y-%m-%dT%H:%M:%S");
    if nanos == 0 {
        format!("{base}Z")
    } else {
        format!("{base}.{nanos:09}Z")
    }
}

pub(super) fn parse_stamp(value: &str) -> Result<DateTime<Utc>, String> {
    if !value.ends_with('Z') {
        return Err("timestamps must be exact UTC RFC3339".to_string());
    }
    let parsed = DateTime::parse_from_rfc3339(value)
        .map_err(|_error| "timestamps must be exact UTC RFC3339".to_string())?;
    Ok(parsed.with_timezone(&Utc))
}

#[derive(Debug)]
pub(super) struct SignalError(String);

impl SignalError {
    pub(super) fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl std::fmt::Display for SignalError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

pub(super) trait Signal {
    fn fail(&mut self) -> Result<(), SignalError>;
    fn recover(&mut self) -> Result<(), SignalError>;
}

#[derive(Serialize, Deserialize)]
struct Wire {
    route: String,
    kind: String,
    at: String,
    source: String,
    group: String,
    evidence: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    confirmed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    message_ref: Option<String>,
}

impl From<&Observation> for Wire {
    fn from(observation: &Observation) -> Self {
        match observation {
            Observation::Signal {
                route,
                kind,
                at,
                group,
                evidence,
                source,
            } => Self {
                route: route.as_str().to_string(),
                kind: kind.as_str().to_string(),
                at: format_stamp(*at),
                source: source.clone(),
                group: group.clone(),
                evidence: evidence.clone(),
                confirmed_by: None,
                message_ref: None,
            },
            Observation::Evaluator {
                route,
                kind,
                at,
                group,
                evidence,
            } => Self {
                route: route.as_str().to_string(),
                kind: kind.as_str().to_string(),
                at: format_stamp(*at),
                source: SOURCE_EVALUATOR.to_string(),
                group: group.clone(),
                evidence: evidence.clone(),
                confirmed_by: None,
                message_ref: None,
            },
            Observation::Notice {
                route,
                kind,
                at,
                group,
                evidence,
                confirmed_by,
                message_ref,
            } => Self {
                route: route.as_str().to_string(),
                kind: kind.as_str().to_string(),
                at: format_stamp(*at),
                source: SOURCE_SLACK.to_string(),
                group: group.clone(),
                evidence: evidence.clone(),
                confirmed_by: Some(confirmed_by.clone()),
                message_ref: Some(message_ref.clone()),
            },
            Observation::Provider {
                route,
                at,
                group,
                evidence,
            } => Self {
                route: route.as_str().to_string(),
                kind: "provider_notified".to_string(),
                at: format_stamp(*at),
                source: SOURCE_PROVIDER.to_string(),
                group: group.clone(),
                evidence: evidence.clone(),
                confirmed_by: None,
                message_ref: None,
            },
            Observation::Maintenance {
                route,
                kind,
                at,
                group,
                evidence,
                source,
            } => Self {
                route: route.as_str().to_string(),
                kind: kind.as_str().to_string(),
                at: format_stamp(*at),
                source: source.clone(),
                group: group.clone(),
                evidence: evidence.clone(),
                confirmed_by: None,
                message_ref: None,
            },
        }
    }
}

impl TryFrom<Wire> for Observation {
    type Error = String;

    fn try_from(wire: Wire) -> Result<Self, Self::Error> {
        let route = ObservationRoute::parse(&wire.route)
            .ok_or_else(|| format!("unknown observation route: {}", wire.route))?;
        let at = parse_stamp(&wire.at)?;
        let kind = wire.kind;
        let group = nonempty("group", wire.group)?;
        let evidence = nonempty("evidence", wire.evidence)?;
        let source = nonempty("source", wire.source)?;
        let confirmed_by = wire.confirmed_by;
        let message_ref = wire.message_ref;
        if confirmed_by.as_ref().is_some_and(|value| value.is_empty())
            || message_ref.as_ref().is_some_and(|value| value.is_empty())
        {
            return Err("slack receipt is missing a confirmer or message reference".to_string());
        }
        let receipt = confirmed_by.is_some() || message_ref.is_some();
        match kind.as_str() {
            "signal_failure" => signal(
                route,
                SignalKind::Failure,
                at,
                group,
                evidence,
                source,
                receipt,
            ),
            "signal_restored" => signal(
                route,
                SignalKind::Restored,
                at,
                group,
                evidence,
                source,
                receipt,
            ),
            "pending" => evaluator(
                route,
                EvalKind::Pending,
                at,
                group,
                evidence,
                &source,
                receipt,
            ),
            "firing" => evaluator(
                route,
                EvalKind::Firing,
                at,
                group,
                evidence,
                &source,
                receipt,
            ),
            "recovered" => evaluator(
                route,
                EvalKind::Recovered,
                at,
                group,
                evidence,
                &source,
                receipt,
            ),
            "slack_alert" => notice(
                route,
                NoticeKind::SlackAlert,
                at,
                group,
                evidence,
                &source,
                (confirmed_by, message_ref),
            ),
            "slack_recovery" => notice(
                route,
                NoticeKind::SlackRecovery,
                at,
                group,
                evidence,
                &source,
                (confirmed_by, message_ref),
            ),
            "reminder" => notice(
                route,
                NoticeKind::Reminder,
                at,
                group,
                evidence,
                &source,
                (confirmed_by, message_ref),
            ),
            "provider_notified" => {
                if source != SOURCE_PROVIDER {
                    return Err(
                        "provider notification requires provider-metadata source".to_string()
                    );
                }
                if receipt {
                    return Err(
                        "only a slack receipt carries a confirmer and message reference"
                            .to_string(),
                    );
                }
                Ok(Self::Provider {
                    route,
                    at,
                    group,
                    evidence,
                })
            }
            "maintenance_start" => maintenance(
                route,
                MaintenanceKind::Start,
                at,
                group,
                evidence,
                source,
                receipt,
            ),
            "maintenance_end" => maintenance(
                route,
                MaintenanceKind::End,
                at,
                group,
                evidence,
                source,
                receipt,
            ),
            other => Err(format!("unknown observation kind: {other}")),
        }
    }
}

fn signal(
    route: ObservationRoute,
    kind: SignalKind,
    at: DateTime<Utc>,
    group: String,
    evidence: String,
    source: String,
    receipt: bool,
) -> Result<Observation, String> {
    if receipt {
        return Err("only a slack receipt carries a confirmer and message reference".to_string());
    }
    Ok(Observation::Signal {
        route,
        kind,
        at,
        group,
        evidence,
        source,
    })
}

fn evaluator(
    route: ObservationRoute,
    kind: EvalKind,
    at: DateTime<Utc>,
    group: String,
    evidence: String,
    source: &str,
    receipt: bool,
) -> Result<Observation, String> {
    if source != SOURCE_EVALUATOR {
        return Err(format!(
            "{}: requires actual evaluator history, not inferred sample state",
            kind.as_str()
        ));
    }
    if receipt {
        return Err("only a slack receipt carries a confirmer and message reference".to_string());
    }
    Ok(Observation::Evaluator {
        route,
        kind,
        at,
        group,
        evidence,
    })
}

fn notice(
    route: ObservationRoute,
    kind: NoticeKind,
    at: DateTime<Utc>,
    group: String,
    evidence: String,
    source: &str,
    receipt: (Option<String>, Option<String>),
) -> Result<Observation, String> {
    let (confirmed_by, message_ref) = receipt;
    if source != SOURCE_SLACK {
        return Err(format!(
            "{}: needs human-confirmed Slack receipt, confirmer and message reference",
            kind.as_str()
        ));
    }
    let confirmed_by = confirmed_by
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            format!(
                "{}: needs human-confirmed Slack receipt, confirmer and message reference",
                kind.as_str()
            )
        })?;
    let message_ref = message_ref
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            format!(
                "{}: needs human-confirmed Slack receipt, confirmer and message reference",
                kind.as_str()
            )
        })?;
    Ok(Observation::Notice {
        route,
        kind,
        at,
        group,
        evidence,
        confirmed_by,
        message_ref,
    })
}

fn maintenance(
    route: ObservationRoute,
    kind: MaintenanceKind,
    at: DateTime<Utc>,
    group: String,
    evidence: String,
    source: String,
    receipt: bool,
) -> Result<Observation, String> {
    if receipt {
        return Err("only a slack receipt carries a confirmer and message reference".to_string());
    }
    Ok(Observation::Maintenance {
        route,
        kind,
        at,
        group,
        evidence,
        source,
    })
}

fn nonempty(field: &str, value: String) -> Result<String, String> {
    if value.is_empty() {
        Err(format!("{field} is empty"))
    } else {
        Ok(value)
    }
}
