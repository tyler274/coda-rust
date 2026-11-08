use std::path::{Path, PathBuf};
use std::process::Command;

use chrono::{DateTime, Duration, Utc};
use serde_json::Value;

use super::live::{
    Captured, QUALIFICATION_BELOW_RECOVERY, QUALIFICATION_CRITICAL, SYNTHETIC_METRIC,
    SYNTHETIC_TAG, SchedulerState, Shell, describe_args, doppler_args, loki_body, mutate_args,
    reconcile_scheduler, secret_from_capture, series_body,
};
use super::model::{
    Breach, DrillAction, DrillRequest, DrillRoute, EvalKind, Exercise, MAX_EXERCISE_SECONDS,
    MAX_REMINDERS, NoticeKind, Observation, ObservationRoute, Signal, SignalError, SignalKind,
    format_stamp, observations_to_json, parse_exercise, parse_stamp,
};
use super::{RunErrorKind, parse_args, run};

fn at(value: &str) -> DateTime<Utc> {
    parse_stamp(value).unwrap()
}

fn temp_dir() -> PathBuf {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let dir = std::env::temp_dir().join(format!(
        "observability-drill-{}-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

struct Temp(PathBuf);

impl Temp {
    fn new() -> Self {
        Self(temp_dir())
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

struct Fake {
    refuse_fail: bool,
    refuse_recover: bool,
    fails: u32,
    recovers: u32,
}

impl Fake {
    fn ok() -> Self {
        Self {
            refuse_fail: false,
            refuse_recover: false,
            fails: 0,
            recovers: 0,
        }
    }
}

impl Signal for Fake {
    fn fail(&mut self) -> Result<(), SignalError> {
        self.fails += 1;
        if self.refuse_fail {
            Err(SignalError::new("fail refused"))
        } else {
            Ok(())
        }
    }

    fn recover(&mut self) -> Result<(), SignalError> {
        self.recovers += 1;
        if self.refuse_recover {
            Err(SignalError::new("recover refused"))
        } else {
            Ok(())
        }
    }
}

fn request(route: DrillRoute, action: DrillAction) -> DrillRequest {
    DrillRequest { route, action }
}

fn write_prior(dir: &Path, route: DrillRoute, body: &str) {
    let path = dir.join(format!(
        "observations-{}-20260101T000000Z.json",
        route.slug()
    ));
    std::fs::write(path, body).unwrap();
}

fn latest_body(dir: &Path, route: DrillRoute) -> String {
    let prefix = format!("observations-{}-", route.slug());
    let mut paths: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(&prefix) && name.ends_with(".json"))
        })
        .collect();
    paths.sort();
    std::fs::read_to_string(paths.last().unwrap()).unwrap()
}

fn kinds(body: &str) -> Vec<String> {
    serde_json::from_str::<Vec<Value>>(body)
        .unwrap()
        .into_iter()
        .map(|record| record["kind"].as_str().unwrap().to_string())
        .collect()
}

fn open_failure(route: DrillRoute) -> String {
    let mut exercise = Exercise::idle(route);
    exercise.record_signal(
        SignalKind::Failure,
        at("2026-01-01T00:00:00Z"),
        "prior.json",
    );
    let mut body = exercise.to_json().unwrap();
    body.push('\n');
    body
}

#[test]
fn fail_records_one_signal_and_does_not_restore() {
    let dir = Temp::new();
    let mut signal = Fake::ok();
    let wrote = run(
        request(DrillRoute::Datadog, DrillAction::Fail),
        dir.path(),
        at("2026-01-01T00:00:00Z"),
        &mut signal,
    )
    .unwrap();
    assert!(wrote.is_some());
    assert_eq!(signal.fails, 1);
    assert_eq!(signal.recovers, 0);
    let body = latest_body(dir.path(), DrillRoute::Datadog);
    assert_eq!(kinds(&body), ["signal_failure"]);
    let records: Vec<Value> = serde_json::from_str(&body).unwrap();
    assert_eq!(records[0]["route"], "datadog");
    assert_eq!(records[0]["source"], "synthetic-fixture");
    assert_eq!(
        records[0]["group"],
        "staging/synthetic-qualification/datadog"
    );
    assert!(!body.contains("DD_API_KEY"));
}

#[test]
fn each_route_writes_the_checker_route() {
    let cases = [
        (DrillRoute::Datadog, "datadog"),
        (DrillRoute::Grafana, "grafana-test"),
        (DrillRoute::Watchdog, "cloud-monitoring"),
    ];
    for (route, ledger_route) in cases {
        let dir = Temp::new();
        let mut signal = Fake::ok();
        run(
            request(route, DrillAction::Fail),
            dir.path(),
            at("2026-01-01T00:00:00Z"),
            &mut signal,
        )
        .unwrap();
        let records: Vec<Value> = serde_json::from_str(&latest_body(dir.path(), route)).unwrap();
        assert_eq!(records[0]["route"], ledger_route);
    }
}

#[test]
fn forty_minutes_is_still_inside_the_cap() {
    let dir = Temp::new();
    write_prior(
        dir.path(),
        DrillRoute::Datadog,
        &open_failure(DrillRoute::Datadog),
    );
    let mut signal = Fake::ok();
    let now = at("2026-01-01T00:00:00Z") + Duration::seconds(MAX_EXERCISE_SECONDS);
    run(
        request(DrillRoute::Datadog, DrillAction::Status),
        dir.path(),
        now,
        &mut signal,
    )
    .unwrap();
    assert_eq!(signal.recovers, 0);
    assert_eq!(signal.fails, 0);
}

#[test]
fn a_run_past_forty_minutes_restores_and_does_not_extend_the_failure() {
    let dir = Temp::new();
    write_prior(
        dir.path(),
        DrillRoute::Watchdog,
        &open_failure(DrillRoute::Watchdog),
    );
    let mut signal = Fake::ok();
    let now = at("2026-01-01T00:00:00Z") + Duration::seconds(MAX_EXERCISE_SECONDS + 1);
    let error = run(
        request(DrillRoute::Watchdog, DrillAction::Fail),
        dir.path(),
        now,
        &mut signal,
    )
    .unwrap_err();
    assert!(matches!(error.kind, RunErrorKind::Cap(Breach::Duration)));
    assert_eq!(signal.fails, 0);
    assert_eq!(signal.recovers, 1);
    assert_eq!(
        kinds(&latest_body(dir.path(), DrillRoute::Watchdog)),
        ["signal_failure", "signal_restored"]
    );
    assert!(error.to_string().contains("40 minutes"));
}

#[test]
fn a_third_reminder_is_refused_and_a_run_that_already_has_one_restores() {
    let mut exercise = Exercise::idle(DrillRoute::Datadog);
    let stamp = at("2026-01-01T00:10:00Z");
    for index in 0..MAX_REMINDERS {
        exercise
            .record_reminder(
                stamp + Duration::minutes(index as i64),
                "prior.json",
                "fixture-operator",
                "fixture-message-not-real",
            )
            .unwrap();
    }
    let refused = exercise.record_reminder(
        stamp + Duration::minutes(30),
        "prior.json",
        "fixture-operator",
        "fixture-message-not-real",
    );
    assert_eq!(refused, Err(Breach::Reminders));

    let group = "staging/synthetic-qualification/grafana";
    let route = ObservationRoute::GrafanaTest;
    let ledger = observations_to_json(&[
        Observation::signal(
            route,
            SignalKind::Failure,
            at("2026-01-01T00:00:00Z"),
            group,
            "prior.json",
        ),
        Observation::notice(
            route,
            NoticeKind::Reminder,
            at("2026-01-01T00:16:00Z"),
            group,
            "prior.json",
            "fixture-operator",
            "fixture-message-not-real",
        ),
        Observation::notice(
            route,
            NoticeKind::Reminder,
            at("2026-01-01T00:32:00Z"),
            group,
            "prior.json",
            "fixture-operator",
            "fixture-message-not-real",
        ),
        Observation::notice(
            route,
            NoticeKind::Reminder,
            at("2026-01-01T00:33:00Z"),
            group,
            "prior.json",
            "fixture-operator",
            "fixture-message-not-real",
        ),
    ])
    .unwrap();
    let dir = Temp::new();
    write_prior(dir.path(), DrillRoute::Grafana, &format!("{ledger}\n"));
    let mut signal = Fake::ok();
    let error = run(
        request(DrillRoute::Grafana, DrillAction::Status),
        dir.path(),
        at("2026-01-01T00:34:00Z"),
        &mut signal,
    )
    .unwrap_err();
    assert!(matches!(error.kind, RunErrorKind::Cap(Breach::Reminders)));
    assert_eq!(signal.recovers, 1);
    assert!(
        kinds(&latest_body(dir.path(), DrillRoute::Grafana))
            .contains(&"signal_restored".to_string())
    );
}

#[test]
fn a_failed_effect_restores_even_when_nothing_was_open() {
    let dir = Temp::new();
    let mut signal = Fake::ok();
    signal.refuse_fail = true;
    let error = run(
        request(DrillRoute::Datadog, DrillAction::Fail),
        dir.path(),
        at("2026-01-01T00:00:00Z"),
        &mut signal,
    )
    .unwrap_err();
    assert!(matches!(error.kind, RunErrorKind::Effect(_)));
    assert_eq!(signal.fails, 1);
    assert_eq!(signal.recovers, 1);
    assert!(error.to_string().contains("fail refused"));
    assert!(!error.to_string().contains("planted-value"));
}

#[test]
fn a_failed_effect_on_an_open_exercise_records_the_restore() {
    let dir = Temp::new();
    write_prior(
        dir.path(),
        DrillRoute::Datadog,
        &open_failure(DrillRoute::Datadog),
    );
    let mut signal = Fake::ok();
    signal.refuse_fail = true;
    let error = run(
        request(DrillRoute::Datadog, DrillAction::Fail),
        dir.path(),
        at("2026-01-01T00:05:00Z"),
        &mut signal,
    )
    .unwrap_err();
    assert!(matches!(error.kind, RunErrorKind::Effect(_)));
    assert_eq!(signal.recovers, 1);
    assert_eq!(
        kinds(&latest_body(dir.path(), DrillRoute::Datadog)),
        ["signal_failure", "signal_restored"]
    );
}

#[test]
fn restore_failure_is_reported_and_not_retried() {
    let dir = Temp::new();
    write_prior(
        dir.path(),
        DrillRoute::Datadog,
        &open_failure(DrillRoute::Datadog),
    );
    let mut signal = Fake::ok();
    signal.refuse_recover = true;
    let error = run(
        request(DrillRoute::Datadog, DrillAction::Recover),
        dir.path(),
        at("2026-01-01T00:05:00Z"),
        &mut signal,
    )
    .unwrap_err();
    assert!(matches!(error.kind, RunErrorKind::Restore(_)));
    assert_eq!(signal.recovers, 1);
    assert_eq!(
        kinds(&latest_body(dir.path(), DrillRoute::Datadog)),
        ["signal_failure"]
    );
}

#[test]
fn recover_closes_an_open_exercise_once() {
    let dir = Temp::new();
    write_prior(
        dir.path(),
        DrillRoute::Grafana,
        &open_failure(DrillRoute::Grafana),
    );
    let mut signal = Fake::ok();
    run(
        request(DrillRoute::Grafana, DrillAction::Recover),
        dir.path(),
        at("2026-01-01T00:05:00Z"),
        &mut signal,
    )
    .unwrap();
    assert_eq!(signal.recovers, 1);
    assert_eq!(
        kinds(&latest_body(dir.path(), DrillRoute::Grafana)),
        ["signal_failure", "signal_restored"]
    );
}

#[test]
fn a_restored_ledger_starts_a_new_exercise() {
    let dir = Temp::new();
    let mut exercise = Exercise::idle(DrillRoute::Datadog);
    exercise.record_signal(
        SignalKind::Failure,
        at("2026-01-01T00:00:00Z"),
        "prior.json",
    );
    exercise.record_signal(
        SignalKind::Restored,
        at("2026-01-01T00:02:00Z"),
        "prior.json",
    );
    write_prior(
        dir.path(),
        DrillRoute::Datadog,
        &format!("{}\n", exercise.to_json().unwrap()),
    );
    let mut signal = Fake::ok();
    run(
        request(DrillRoute::Datadog, DrillAction::Fail),
        dir.path(),
        at("2026-01-01T00:10:00Z"),
        &mut signal,
    )
    .unwrap();
    assert_eq!(
        kinds(&latest_body(dir.path(), DrillRoute::Datadog)),
        ["signal_failure"]
    );
}

#[test]
fn an_unreadable_ledger_still_restores() {
    let dir = Temp::new();
    write_prior(dir.path(), DrillRoute::Watchdog, "not json\n");
    let mut signal = Fake::ok();
    let error = run(
        request(DrillRoute::Watchdog, DrillAction::Status),
        dir.path(),
        at("2026-01-01T00:05:00Z"),
        &mut signal,
    )
    .unwrap_err();
    assert!(matches!(error.kind, RunErrorKind::Ledger(_)));
    assert_eq!(signal.recovers, 1);
}

#[test]
fn slack_notice_without_a_confirmer_cannot_be_loaded() {
    let invalid = r#"[{
        "route": "datadog",
        "kind": "slack_alert",
        "at": "2026-01-01T00:01:00Z",
        "source": "human-confirmed-slack",
        "group": "staging/synthetic-qualification/datadog",
        "evidence": "prior.json"
    }]"#;
    let error = parse_exercise(DrillRoute::Datadog, invalid).unwrap_err();
    assert!(error.contains("human-confirmed"));
}

#[test]
fn stamps_keep_fractional_seconds() {
    let stamp = "2026-01-01T00:03:02.123456790Z";
    assert_eq!(format_stamp(at(stamp)), stamp);
}

#[test]
fn parse_rejects_an_unknown_route_and_a_reversed_clock() {
    let unknown = r#"[{"route":"unknown","kind":"signal_failure","at":"2026-01-01T00:00:00Z","source":"synthetic-fixture","group":"g","evidence":"e"}]"#;
    assert!(
        parse_exercise(DrillRoute::Datadog, unknown)
            .unwrap_err()
            .contains("unknown observation route")
    );
    let reversed = r#"[
        {"route":"datadog","kind":"signal_restored","at":"2026-01-01T00:02:00Z","source":"synthetic-fixture","group":"g","evidence":"e"},
        {"route":"datadog","kind":"signal_failure","at":"2026-01-01T00:01:00Z","source":"synthetic-fixture","group":"g","evidence":"e"}
    ]"#;
    assert!(
        parse_exercise(DrillRoute::Datadog, reversed)
            .unwrap_err()
            .contains("chronological")
    );
}

#[test]
fn command_arguments_reject_unknown_pairs() {
    let error = parse_args(&[
        "other".into(),
        "fail".into(),
        "--output-dir".into(),
        "out".into(),
    ])
    .unwrap_err();
    assert!(error.contains("unknown route"));
    let error = parse_args(&[
        "datadog".into(),
        "pause".into(),
        "--output-dir".into(),
        "out".into(),
    ])
    .unwrap_err();
    assert!(error.contains("unknown action"));
    let (request, output) = parse_args(&[
        "watchdog".into(),
        "recover".into(),
        "--output-dir".into(),
        "out".into(),
    ])
    .unwrap();
    assert_eq!(request.route, DrillRoute::Watchdog);
    assert_eq!(request.action, DrillAction::Recover);
    assert_eq!(output, PathBuf::from("out"));
}

#[test]
fn doppler_errors_and_debug_omit_the_secret_value() {
    let planted = "planted-value";
    let error = secret_from_capture(false, Some(1), planted.as_bytes()).unwrap_err();
    assert!(!error.to_string().contains(planted));
    assert!(error.to_string().contains("Doppler exited"));
    let secret = secret_from_capture(true, Some(0), format!("{planted}\n").as_bytes()).unwrap();
    assert!(!format!("{secret:?}").contains(planted));
    assert!(!secret.to_string().contains(planted));
    let args = doppler_args();
    assert_eq!(
        args,
        [
            "secrets",
            "get",
            "DD_API_KEY",
            "--plain",
            "--project",
            "coda-rust",
            "--config",
            "stg",
        ]
    );
    assert!(!args.iter().any(|arg| arg.contains(planted)));
}

#[test]
fn metric_and_loki_bodies_name_the_synthetic_signal() {
    let at = at("2026-01-01T00:00:00Z");
    let firing: Value = serde_json::from_str(&series_body(QUALIFICATION_CRITICAL, at)).unwrap();
    let healthy: Value =
        serde_json::from_str(&series_body(QUALIFICATION_BELOW_RECOVERY, at)).unwrap();
    assert_eq!(firing["series"][0]["metric"], SYNTHETIC_METRIC);
    assert_eq!(firing["series"][0]["tags"][0], SYNTHETIC_TAG);
    assert_eq!(firing["series"][0]["points"][0][1], 1.0);
    assert_eq!(firing["series"][0]["points"][0][1], QUALIFICATION_CRITICAL);
    assert_eq!(healthy["series"][0]["points"][0][1], 0.0);
    assert!(healthy["series"][0]["points"][0][1].as_f64().unwrap() < 0.5);
    assert_eq!(
        healthy["series"][0]["points"][0][1],
        QUALIFICATION_BELOW_RECOVERY
    );
    assert!(!series_body(QUALIFICATION_CRITICAL, at).contains("DD_API_KEY"));
    let fail: Value = serde_json::from_str(&loki_body("fail", at)).unwrap();
    let recover: Value = serde_json::from_str(&loki_body("ok", at)).unwrap();
    assert_eq!(
        fail["streams"][0]["stream"]["qualification"],
        "synthetic-notification-test"
    );
    assert_eq!(fail["streams"][0]["values"][0][1], "fail");
    assert_eq!(recover["streams"][0]["values"][0][1], "ok");
}

struct Scripted {
    answers: Vec<Captured>,
    calls: Vec<Vec<String>>,
}

impl Shell for Scripted {
    fn run(&mut self, args: &[String]) -> Result<Captured, SignalError> {
        self.calls.push(args.to_vec());
        Ok(self.answers.remove(0))
    }
}

#[test]
fn scheduler_pause_and_resume_are_idempotent() {
    let describe = describe_args();
    assert!(describe.iter().any(|arg| arg == "availability-probe"));
    assert!(
        describe
            .iter()
            .any(|arg| arg == "--project=coda-rust-staging")
    );
    assert!(!describe.iter().any(|arg| arg.contains("DD_API_KEY")));

    let mut paused = Scripted {
        answers: vec![Captured {
            success: true,
            code: Some(0),
            stdout: b"PAUSED\n".to_vec(),
            stderr: Vec::new(),
        }],
        calls: Vec::new(),
    };
    reconcile_scheduler(SchedulerState::Paused, &mut paused).unwrap();
    assert_eq!(paused.calls.len(), 1);

    let mut enabled = Scripted {
        answers: vec![
            Captured {
                success: true,
                code: Some(0),
                stdout: b"ENABLED\n".to_vec(),
                stderr: Vec::new(),
            },
            Captured {
                success: true,
                code: Some(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
            },
        ],
        calls: Vec::new(),
    };
    reconcile_scheduler(SchedulerState::Paused, &mut enabled).unwrap();
    assert_eq!(enabled.calls[1], mutate_args("pause"));

    let mut resume = Scripted {
        answers: vec![
            Captured {
                success: true,
                code: Some(0),
                stdout: b"PAUSED\n".to_vec(),
                stderr: Vec::new(),
            },
            Captured {
                success: true,
                code: Some(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
            },
        ],
        calls: Vec::new(),
    };
    reconcile_scheduler(SchedulerState::Enabled, &mut resume).unwrap();
    assert_eq!(resume.calls[1], mutate_args("resume"));

    let mut unknown = Scripted {
        answers: vec![Captured {
            success: true,
            code: Some(0),
            stdout: b"ENABLED planted-value\n".to_vec(),
            stderr: Vec::new(),
        }],
        calls: Vec::new(),
    };
    let error = reconcile_scheduler(SchedulerState::Paused, &mut unknown).unwrap_err();
    assert_eq!(unknown.calls.len(), 1);
    assert!(!error.to_string().contains("planted-value"));
}

fn qualify(body: &str) -> std::process::Output {
    let dir = Temp::new();
    let path = dir.path().join("observations.json");
    std::fs::write(&path, body).unwrap();
    let script =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../dev/observability/qualify-alerts.py");
    Command::new("python3")
        .arg(script)
        .arg("--observations")
        .arg(&path)
        .output()
        .expect("python3")
}

fn complete_observations() -> Vec<Observation> {
    let route = ObservationRoute::Datadog;
    let group = "staging/qualification-test/unavailable";
    let evidence = "fixture-only.json";
    vec![
        Observation::signal(
            route,
            SignalKind::Failure,
            at("2026-01-01T00:00:00.123456789Z"),
            group,
            evidence,
        ),
        Observation::evaluator(
            route,
            EvalKind::Pending,
            at("2026-01-01T00:01:00.123456789Z"),
            group,
            evidence,
        ),
        Observation::evaluator(
            route,
            EvalKind::Firing,
            at("2026-01-01T00:03:00.123456789Z"),
            group,
            evidence,
        ),
        Observation::provider(route, at("2026-01-01T00:03:01.123456789Z"), group, evidence),
        Observation::notice(
            route,
            NoticeKind::SlackAlert,
            at("2026-01-01T00:03:02.123456790Z"),
            group,
            evidence,
            "fixture-operator",
            "fixture-message-not-real",
        ),
        Observation::signal(
            route,
            SignalKind::Restored,
            at("2026-01-01T00:04:00.123456789Z"),
            group,
            evidence,
        ),
        Observation::evaluator(
            route,
            EvalKind::Recovered,
            at("2026-01-01T00:06:00.123456789Z"),
            group,
            evidence,
        ),
        Observation::notice(
            route,
            NoticeKind::SlackRecovery,
            at("2026-01-01T00:06:02.123456789Z"),
            group,
            evidence,
            "fixture-operator",
            "fixture-message-not-real",
        ),
    ]
}

#[test]
fn qualify_alerts_accepts_the_ledger_and_rejects_a_partial_one_as_incomplete() {
    let complete = observations_to_json(&complete_observations()).unwrap();
    let reloaded = parse_exercise(DrillRoute::Datadog, &complete).unwrap();
    let complete = reloaded.to_json().unwrap();
    let output = qualify(&complete);
    assert_eq!(
        output.status.code(),
        Some(0),
        "stdout {}\nstderr {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let dir = Temp::new();
    let mut signal = Fake::ok();
    run(
        request(DrillRoute::Datadog, DrillAction::Fail),
        dir.path(),
        at("2026-01-01T00:00:00Z"),
        &mut signal,
    )
    .unwrap();
    let partial = qualify(&latest_body(dir.path(), DrillRoute::Datadog));
    assert_eq!(
        partial.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&partial.stderr)
    );
    assert!(partial.stderr.is_empty());
    assert!(String::from_utf8_lossy(&partial.stdout).contains("require exactly one"));
}

fn ok(stdout: &str) -> Captured {
    Captured {
        success: true,
        code: Some(0),
        stdout: stdout.as_bytes().to_vec(),
        stderr: Vec::new(),
    }
}

fn readiness(stamp: &str, outcomes: &[(&str, &str)]) -> Value {
    serde_json::json!({
        "timeSeries": outcomes.iter().map(|(target, outcome)| serde_json::json!({
            "metric": {"labels": {"target": target, "outcome": outcome}},
            "points": [{"interval": {"endTime": stamp}, "value": {"int64Value": "0"}}],
        })).collect::<Vec<_>>()
    })
}

fn heartbeat(stamp: &str) -> Value {
    serde_json::json!({"timeSeries": [{"points": [{"interval": {"endTime": stamp}, "value": {"int64Value": "1"}}]}]})
}

fn probe_with(
    readiness_body: Value,
    heartbeat_body: Value,
) -> (super::probe::ProbeReport, Vec<Vec<String>>, u32) {
    let mut shell = Scripted {
        answers: vec![ok("availability-probe-abcde\n"), ok("token\n")],
        calls: Vec::new(),
    };
    let mut pauses = 0;
    let report = super::probe::probe(
        at("2026-11-07T03:08:41Z"),
        &mut shell,
        &mut |token, metric, _slot| {
            assert_eq!(token, "token");
            Ok(if metric.ends_with("/heartbeat") {
                heartbeat_body.clone()
            } else {
                readiness_body.clone()
            })
        },
        &mut || pauses += 1,
    )
    .unwrap();
    (report, shell.calls, pauses)
}

#[test]
fn probe_runs_the_current_slot_as_the_probe_identity_and_passes_when_all_targets_answer() {
    let stamp = "2026-11-07T03:08:00Z";
    let all = [
        ("physician", "success"),
        ("duet", "success"),
        ("physician-worker", "success"),
    ];
    let (report, calls, pauses) = probe_with(readiness(stamp, &all), heartbeat(stamp));
    assert_eq!(report.slot, 1_794_020_880);
    assert_eq!(report.execution, "availability-probe-abcde");
    assert!(
        calls[0]
            .iter()
            .any(|arg| arg == "--update-env-vars=AVAILABILITY_PROBE_SLOT=1794020880")
    );
    assert!(calls[0].iter().any(|arg| arg == "--wait"));
    assert_eq!(pauses, 0);
    assert_eq!(report.verdict(), super::probe::Verdict::Reachable);
}

#[test]
fn probe_names_targets_that_iap_refused() {
    let stamp = "2026-11-07T03:08:00Z";
    let body = readiness(
        stamp,
        &[
            ("physician", "auth_failure"),
            ("duet", "success"),
            ("physician-worker", "success"),
        ],
    );
    let (report, _, _) = probe_with(body, heartbeat(stamp));
    assert_eq!(
        report.verdict(),
        super::probe::Verdict::AuthRefused(vec!["physician".into()])
    );
}

#[test]
fn probe_ignores_other_slots_and_fails_without_a_heartbeat() {
    let stamp = "2026-11-07T03:08:00Z";
    let earlier = "2026-11-07T03:07:00Z";
    let all = [
        ("physician", "success"),
        ("duet", "success"),
        ("physician-worker", "success"),
    ];
    let (report, _, pauses) = probe_with(readiness(earlier, &all), heartbeat(earlier));
    assert!(pauses > 0);
    assert_eq!(
        report.verdict(),
        super::probe::Verdict::Unobserved(vec![
            "physician".into(),
            "duet".into(),
            "physician-worker".into()
        ])
    );
    let (report, _, _) = probe_with(readiness(stamp, &all), heartbeat(earlier));
    assert_eq!(report.verdict(), super::probe::Verdict::NoHeartbeat);
}

#[test]
fn probe_fails_before_readback_when_the_job_fails() {
    let mut shell = Scripted {
        answers: vec![Captured {
            success: false,
            code: Some(1),
            stdout: Vec::new(),
            stderr: b"ERROR: PERMISSION_DENIED planted-secret".to_vec(),
        }],
        calls: Vec::new(),
    };
    let error = super::probe::probe(
        at("2026-11-07T03:08:41Z"),
        &mut shell,
        &mut |_token, _metric, _slot| panic!("no readback after a failed job"),
        &mut || {},
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "gcloud probe job exited Some(1) (PERMISSION_DENIED)"
    );
    assert!(!error.to_string().contains("planted-secret"));
    assert_eq!(shell.calls.len(), 1);
}

#[test]
fn probe_retries_a_transient_read_and_names_each_failing_outcome() {
    let stamp = "2026-11-07T03:08:00Z";
    let body = readiness(
        stamp,
        &[
            ("physician", "success"),
            ("duet", "timeout"),
            ("physician-worker", "success"),
        ],
    );
    let mut shell = Scripted {
        answers: vec![ok("availability-probe-abcde\n"), ok("token\n")],
        calls: Vec::new(),
    };
    let mut reads = 0;
    let report = super::probe::probe(
        at("2026-11-07T03:08:41Z"),
        &mut shell,
        &mut |_token, metric, _slot| {
            reads += 1;
            if reads == 1 {
                return Err(super::probe::ProbeError::Refused(503));
            }
            Ok(if metric.ends_with("/heartbeat") {
                heartbeat(stamp)
            } else {
                body.clone()
            })
        },
        &mut || {},
    )
    .unwrap();
    let verdict = report.verdict();
    assert_eq!(
        verdict,
        super::probe::Verdict::Unhealthy(vec!["duet: timeout".into()])
    );
    assert_eq!(verdict.to_string(), "Readiness failed for duet: timeout.");
}

#[test]
fn probe_reports_the_read_error_when_no_read_ever_succeeds() {
    let mut shell = Scripted {
        answers: vec![ok("availability-probe-abcde\n"), ok("token\n")],
        calls: Vec::new(),
    };
    let error = super::probe::probe(
        at("2026-11-07T03:08:41Z"),
        &mut shell,
        &mut |_token, _metric, _slot| Err(super::probe::ProbeError::Refused(403)),
        &mut || {},
    )
    .unwrap_err();
    assert_eq!(error.to_string(), "Cloud Monitoring refused the read (403)");
}
