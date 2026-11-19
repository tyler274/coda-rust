use std::collections::HashSet;

use serde_json::{Value, json};

use super::fake_github::{Call, FakeGitHub};
use super::*;
use crate::ci_controller::github::{JobId, LABEL};

fn runs() -> String {
    repo_path("actions/runs?status=queued&per_page=50")
}

fn jitconfig() -> String {
    repo_path("actions/runners/generate-jitconfig")
}

fn jobs_path(run: u64) -> String {
    repo_path(&format!(
        "actions/runs/{run}/jobs?filter=latest&per_page=100"
    ))
}

fn job(id: u64, status: &str, label: &str) -> Value {
    json!({"id": id, "name": "ci", "status": status, "labels": [label]})
}

#[test]
fn routes_only_jobs_labelled_for_cyrene() {
    let fake = FakeGitHub::builder()
        .route(
            "GET",
            &runs(),
            200,
            json!({"workflow_runs": [
                {"id": 2, "created_at": "2026-01-02T00:00:00Z"},
                {"id": 1, "created_at": "2026-01-01T00:00:00Z"},
            ]}),
        )
        .route(
            "GET",
            &jobs_path(1),
            200,
            json!({"jobs": [
                job(10, "queued", "blacksmith-4vcpu-ubuntu-2404"),
                job(11, "in_progress", "cyrene-coda"),
            ]}),
        )
        .route(
            "GET",
            &jobs_path(2),
            200,
            json!({"jobs": [job(20, "queued", "cyrene-coda")]}),
        )
        .start();
    let found = api(&fake.base).next_job().unwrap().expect("a labelled job");
    assert_eq!(found.id, JobId(20));
    // The older run is read first.
    assert_eq!(fake.paths()[1], Call::new("GET", &jobs_path(1)));
}

#[test]
fn no_labelled_job_means_none() {
    let fake = FakeGitHub::builder()
        .route(
            "GET",
            &runs(),
            200,
            json!({"workflow_runs": [{"id": 1, "created_at": "x"}]}),
        )
        .route(
            "GET",
            &jobs_path(1),
            200,
            json!({"jobs": [job(10, "queued", "ubuntu-latest")]}),
        )
        .start();
    assert_eq!(api(&fake.base).next_job().unwrap(), None);
}

#[test]
fn a_failed_queue_read_is_an_error_not_an_empty_queue() {
    let fake = FakeGitHub::builder()
        .route("GET", &runs(), 500, json!({}))
        .start();
    assert!(api(&fake.base).next_job().is_err());
}

#[test]
fn registration_uses_the_shared_label_and_a_unique_name() {
    let fake = FakeGitHub::builder()
        .route(
            "POST",
            &jitconfig(),
            201,
            json!({"runner": {"id": 7}, "encoded_jit_config": "jit"}),
        )
        .start();
    let client = api(&fake.base);
    client.register(JobId(42)).unwrap();
    client.register(JobId(42)).unwrap();
    let calls = fake.calls();
    assert!(
        calls
            .iter()
            .all(|call| call.method == "POST" && call.path == jitconfig())
    );
    let bodies: Vec<&Value> = calls
        .iter()
        .map(|call| call.body.as_ref().expect("a body"))
        .collect();
    assert_eq!(bodies[0]["labels"], json!([LABEL]));
    let names: Vec<&str> = bodies
        .iter()
        .map(|body| body["name"].as_str().expect("a name"))
        .collect();
    for name in &names {
        let suffix = name
            .strip_prefix("cyrene-coda-42-")
            .expect("job id in the name");
        assert!(
            suffix.len() == 6 && suffix.bytes().all(|b| b.is_ascii_hexdigit()),
            "{name}"
        );
    }
    assert_eq!(names.iter().collect::<HashSet<_>>().len(), 2, "{names:?}");
}

#[test]
fn github_runs_before_local_requests() {
    let _serial = serial();
    let mut setup = Setup::new();
    setup.guest_script("CODA_JOB_EXIT=0", 0);
    add_request(&setup.requests(), &a12(), "ci", b"bundle", None);
    let fake = FakeGitHub::builder()
        .route(
            "GET",
            &runs(),
            200,
            json!({"workflow_runs": [{"id": 1, "created_at": "x"}]}),
        )
        .route(
            "GET",
            &jobs_path(1),
            200,
            json!({"jobs": [job(5, "queued", "cyrene-coda")]}),
        )
        .route(
            "POST",
            &jitconfig(),
            201,
            json!({"runner": {"id": 7}, "encoded_jit_config": "jit"}),
        )
        .route("DELETE", &repo_path("actions/runners/7"), 204, Value::Null)
        .start();
    setup
        .controller(&EmptyDisks)
        .one_pass(&github(&fake.base))
        .unwrap();
    assert!(setup.config.logs().join("github-5.log").exists());
    assert_eq!(
        listing(&setup.requests()).len(),
        2,
        "the local request still waits"
    );
    assert!(listing(&setup.config.results()).is_empty());
}

#[test]
fn without_a_token_only_local_requests_run() {
    let _serial = serial();
    let mut setup = Setup::new();
    setup.guest_script("CODA_JOB_EXIT=0", 0);
    add_request(&setup.requests(), &a12(), "ci", b"bundle", None);
    setup
        .controller(&EmptyDisks)
        .one_pass(&Queues::LocalOnly)
        .unwrap();
    let result = setup.config.results().join(a12());
    assert_eq!(fs::read_to_string(result.join("status")).unwrap(), "0\n");
    assert!(listing(&setup.requests()).is_empty());
}
