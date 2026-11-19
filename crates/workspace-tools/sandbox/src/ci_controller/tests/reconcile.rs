use serde_json::{Value, json};

use super::fake_github::{Call, FakeGitHub};
use super::*;

fn page_1() -> String {
    repo_path("actions/runners?per_page=100&page=1")
}

#[test]
fn removes_owned_idle_runners_and_all_job_directories() {
    let setup = Setup::new();
    let jobs = setup.config.jobs();
    fs::create_dir(jobs.join("local-old")).unwrap();
    fs::write(jobs.join("local-old").join("work.img"), "").unwrap();
    fs::write(jobs.join("stray"), "").unwrap();
    let fake = FakeGitHub::builder()
        .route(
            "GET",
            &page_1(),
            200,
            json!({"runners": [
                {"id": 1, "name": "cyrene-coda-1-abcdef", "busy": false},
                {"id": 2, "name": "other", "busy": false},
            ]}),
        )
        .route("DELETE", &repo_path("actions/runners/1"), 204, Value::Null)
        .start();
    setup
        .controller(&EmptyDisks)
        .reconcile(&github(&fake.base))
        .unwrap();
    assert_eq!(
        fake.paths(),
        [
            Call::new("GET", &page_1()),
            Call::new("DELETE", &repo_path("actions/runners/1")),
        ]
    );
    assert!(listing(&jobs).is_empty());
}

#[test]
fn busy_runner_prevents_cleanup() {
    let setup = Setup::new();
    let jobs = setup.config.jobs();
    fs::create_dir(jobs.join("github-1")).unwrap();
    let fake = FakeGitHub::builder()
        .route(
            "GET",
            &page_1(),
            200,
            json!({"runners": [
                {"id": 1, "name": "cyrene-coda-1-abcdef", "busy": true},
            ]}),
        )
        .start();
    let error = setup
        .controller(&EmptyDisks)
        .reconcile(&github(&fake.base))
        .unwrap_err();
    assert!(error.to_string().contains("still busy"), "{error}");
    assert!(jobs.join("github-1").exists());
}

#[test]
fn runners_are_read_page_by_page_and_a_gone_runner_counts_as_deleted() {
    let setup = Setup::new();
    let full: Vec<Value> = (1..=100)
        .map(|id| json!({"id": id, "name": format!("someone-{id}"), "busy": false}))
        .collect();
    let page_2 = repo_path("actions/runners?per_page=100&page=2");
    let fake = FakeGitHub::builder()
        .route("GET", &page_1(), 200, json!({ "runners": full }))
        .route(
            "GET",
            &page_2,
            200,
            json!({"runners": [
                {"id": 101, "name": "cyrene-coda-9-0a1b2c", "busy": false},
            ]}),
        )
        .start();
    // DELETE runners/101 has no route, so the fake answers 404.
    setup
        .controller(&EmptyDisks)
        .reconcile(&github(&fake.base))
        .unwrap();
    assert_eq!(
        fake.paths().last(),
        Some(&Call::new("DELETE", &repo_path("actions/runners/101")))
    );
}

#[test]
fn without_a_token_reconcile_still_clears_job_directories() {
    let setup = Setup::new();
    fs::create_dir(setup.config.jobs().join("local-old")).unwrap();
    setup
        .controller(&EmptyDisks)
        .reconcile(&Queues::LocalOnly)
        .unwrap();
    assert!(listing(&setup.config.jobs()).is_empty());
}

#[test]
fn runner_names_follow_the_controllers_pattern() {
    use crate::ci_controller::github::is_controller_runner;
    assert!(is_controller_runner("cyrene-coda-1-abcdef"));
    assert!(is_controller_runner("cyrene-coda-123-0"));
    for name in [
        "other",
        "cyrene-coda-1-ABCDEF",
        "cyrene-coda-x-abc",
        "cyrene-coda--abc",
        "cyrene-coda-1-",
        "cyrene-coda-1-ab-c",
    ] {
        assert!(!is_controller_runner(name), "{name}");
    }
}
