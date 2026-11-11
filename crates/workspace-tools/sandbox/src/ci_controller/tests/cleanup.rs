//! Every way a job ends still deletes its runner and answers its request,
//! and a cleanup that fails is reported beside the failure before it.

use serde_json::{Value, json};

use super::fake_github::{Call, FakeGitHub};
use super::*;
use crate::ci_controller::github::{Job, JobId, JobStatus};
use crate::ci_controller::guest::GuestFailure;
use crate::ci_controller::results::GUEST_FAILED;

const JITCONFIG: &str = "repos/Coda-Health/coda-rust/actions/runners/generate-jitconfig";
const RUNNER: &str = "repos/Coda-Health/coda-rust/actions/runners/7";

fn job() -> Job {
    Job {
        id: JobId(9),
        name: "ci".into(),
        status: JobStatus::Queued,
        labels: vec!["cyrene-coda".into()],
    }
}

fn github_with_delete(delete_status: u16) -> FakeGitHub {
    FakeGitHub::builder()
        .route(
            "POST",
            JITCONFIG,
            201,
            json!({"runner": {"id": 7}, "encoded_jit_config": "jit"}),
        )
        .route("DELETE", RUNNER, delete_status, Value::Null)
        .route(
            "GET",
            RUNNER,
            200,
            json!({"id": 7, "name": "cyrene-coda-9-abcdef", "busy": false}),
        )
        .route(
            "GET",
            "repos/Coda-Health/coda-rust/actions/jobs/9",
            200,
            json!({"status": "completed"}),
        )
        .start()
}

/// A guest that runs until it is stopped.
fn endless_guest(setup: &mut Setup) {
    let script = setup.dir.path().join("guest");
    fs::write(&script, "#!/bin/sh\nsleep 300 &\nwait\n").unwrap();
    fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
    setup.config.runner = script;
}

fn request_sigterm_soon(controller: &Controller<'_>) {
    let flag = controller.termination.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(300));
        flag.request();
    });
}

fn assert_runner_deleted(fake: &FakeGitHub) {
    assert_eq!(fake.paths().last(), Some(&Call::new("DELETE", RUNNER)));
}

#[test]
fn sigterm_during_a_github_job_still_deletes_its_runner() {
    let _serial = serial();
    let mut setup = Setup::new();
    endless_guest(&mut setup);
    let fake = github_with_delete(204);
    let controller = setup.controller(&EmptyDisks);
    request_sigterm_soon(&controller);
    let error = controller.run_github(&api(&fake.base), &job()).unwrap_err();
    assert!(matches!(error, Error::Terminated), "{error}");
    assert_runner_deleted(&fake);
    assert!(listing(&setup.config.jobs()).is_empty());
}

#[test]
fn an_abandoned_runner_stops_the_guest_and_is_deleted() {
    let _serial = serial();
    let mut setup = Setup::new();
    endless_guest(&mut setup);
    setup.timers = Timers {
        check_every: Duration::from_millis(50),
        idle_grace: Duration::ZERO,
        stop_grace: Duration::from_secs(2),
        ..Timers::PRODUCTION
    };
    let fake = github_with_delete(204);
    let error = setup
        .controller(&EmptyDisks)
        .run_github(&api(&fake.base), &job())
        .unwrap_err();
    assert!(
        matches!(error, Error::Guest(GuestFailure::RunnerAbandoned)),
        "{error}"
    );
    assert!(fake.paths().contains(&Call::new("GET", RUNNER)));
    assert_runner_deleted(&fake);
    assert!(listing(&setup.config.jobs()).is_empty());
}

#[test]
fn sigterm_during_a_local_request_leaves_it_a_failed_result() {
    let _serial = serial();
    let mut setup = Setup::new();
    endless_guest(&mut setup);
    add_request(&setup.requests(), &a12(), "ci", b"bundle", None);
    let controller = setup.controller(&EmptyDisks);
    request_sigterm_soon(&controller);
    let error = controller.run_local(&a12().parse().unwrap()).unwrap_err();
    assert!(matches!(error, Error::Terminated), "{error}");
    let result = setup.config.results().join(a12());
    assert_eq!(
        fs::read_to_string(result.join("status")).unwrap(),
        format!("{GUEST_FAILED}\n")
    );
    assert!(listing(&setup.config.jobs()).is_empty());
}

#[test]
fn a_failed_runner_deletion_is_reported_with_the_guest_failure() {
    let _serial = serial();
    let mut setup = Setup::new();
    setup.guest_script("booting\n", 1);
    let fake = github_with_delete(500);
    let error = setup
        .controller(&EmptyDisks)
        .run_github(&api(&fake.base), &job())
        .unwrap_err();
    let Error::CleanupAfterFailure { error, cleanup } = &error else {
        panic!("both failures must be kept: {error}");
    };
    assert!(matches!(**error, Error::Guest(_)), "{error}");
    assert!(matches!(**cleanup, Error::Api(_)), "{cleanup}");
    let text = crate::ci_controller::error::chain(cleanup.as_ref());
    assert!(text.contains("HTTP 500"), "{text}");
    assert_runner_deleted(&fake);
}
