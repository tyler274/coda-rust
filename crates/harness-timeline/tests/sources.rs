//! Each source read from files shaped like the ones the repository's tools write,
//! including the ways each can fail.

use std::path::{Path, PathBuf};

use harness_timeline::act::{self, RunRecord};
use harness_timeline::event::{
    CheckRollup, PrState, ProofState, RunState, ScenarioOutcome, StatusCategory,
};
use harness_timeline::key::{Revision, TicketKey};
use harness_timeline::{TicketIndex, github, linear, proof, review};

const HEAD: &str = "83ea8e4823cc9177f630757b4e1ee190ac2824e7";
const OLD: &str = "855a435b8477f0779bac609ed8e13e4804786504";

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn revision(text: &str) -> Revision {
    Revision::parse(text).unwrap()
}

fn write(path: &Path, contents: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, contents).unwrap();
}

#[test]
fn proof_entries_are_judged_as_proof_py_status_judges_them() {
    let dir = tempfile::tempdir().unwrap();
    let manifest = proof::manifest_path(dir.path(), "eng-225");
    write(
        &manifest,
        &serde_json::json!({
            "branch": "eng-225",
            "entries": [
                {"id": 1, "claim": "AC1", "label": "passes at head", "kind": "run", "revision": HEAD,
                 "dirty": false, "ok": true, "files": ["01-pass.log"]},
                {"id": 2, "claim": "AC1", "label": "before the fix", "kind": "run", "revision": OLD,
                 "dirty": false, "ok": true, "files": ["02-before.log"], "kept": {"revision": HEAD, "reason": "deliberate"}},
                {"id": 3, "claim": "AC2", "label": "older run", "kind": "run", "revision": OLD,
                 "dirty": false, "ok": true, "files": []},
                {"id": 4, "claim": "AC2", "label": "uncommitted", "kind": "run", "revision": HEAD,
                 "dirty": true, "ok": true, "files": []},
                {"id": 5, "claim": "AC3", "label": "screenshot", "kind": "screenshot", "revision": null,
                 "dirty": false, "ok": true, "files": ["05-shot.png"]},
                {"id": 6, "claim": "AC4", "label": "", "kind": "gap", "revision": HEAD, "dirty": false,
                 "ok": false, "reason": "needs staging", "files": []},
                {"id": 7, "claim": "AC5", "label": "failed", "kind": "run", "revision": HEAD, "dirty": false,
                 "ok": false, "files": []},
                {"id": 8, "claim": "AC5", "label": "replaced", "kind": "run", "revision": OLD, "dirty": false,
                 "ok": false, "files": [], "superseded": "AC5 rerun"}
            ]
        })
        .to_string(),
    );
    let criteria = proof::read_manifest(&manifest, &revision(HEAD)).unwrap();
    let states: Vec<(&str, ProofState)> = criteria
        .iter()
        .map(|c| (c.label.as_str(), c.state))
        .collect();
    assert_eq!(
        states,
        vec![
            ("passes at head", ProofState::Passed),
            ("before the fix", ProofState::Passed),
            ("older run", ProofState::Stale),
            ("uncommitted", ProofState::Stale),
            ("screenshot", ProofState::RevisionUnknown),
            ("", ProofState::Gap),
            ("failed", ProofState::Failed),
        ]
    );
    assert_eq!(
        criteria[0].artifact,
        Some(manifest.parent().unwrap().join("01-pass.log"))
    );
}

#[test]
fn an_unreadable_manifest_is_an_error_not_an_empty_lane() {
    let dir = tempfile::tempdir().unwrap();
    let manifest = dir.path().join("manifest.json");
    write(&manifest, "{\"entries\": [ {\"claim\": ");
    let error = proof::read_manifest(&manifest, &revision(HEAD)).unwrap_err();
    assert!(
        error.to_string().contains("not the expected JSON"),
        "{error}"
    );
}

#[test]
fn the_newest_finished_scenario_receipt_is_the_one_shown() {
    let dir = tempfile::tempdir().unwrap();
    let runs = dir.path().join("target/verification/runs");
    write(
        &runs.join("scenarios-20261105-212231-1/receipt.json"),
        &serde_json::json!({"outcome": "failed", "sources": {"git_sha": OLD}, "missing": []})
            .to_string(),
    );
    write(
        &runs.join("scenarios-20261105-214023-2/receipt.json"),
        &serde_json::json!({"outcome": "incomplete", "sources": {"git_sha": HEAD},
                            "missing": [{"capability": "real model"}, {"capability": "staging"}]})
        .to_string(),
    );
    write(&runs.join("physician-scale-stack-1/receipt.json"), "{}");
    let scenario = proof::latest_scenario(dir.path()).unwrap().unwrap();
    assert_eq!(scenario.outcome, ScenarioOutcome::Incomplete);
    assert_eq!(scenario.missing, 2);
    assert_eq!(scenario.revision, Some(revision(HEAD)));

    write(
        &runs.join("scenarios-20261106-000000-3/receipt.json"),
        &serde_json::json!({"started_at": "2026-11-06T00:00:00Z", "sources": {}}).to_string(),
    );
    assert_eq!(
        proof::latest_scenario(dir.path()).unwrap(),
        None,
        "a run in progress has no outcome yet"
    );
    assert_eq!(
        proof::latest_scenario(&dir.path().join("none")).unwrap(),
        None
    );
}

#[test]
fn review_rounds_follow_the_codex_event_stream() {
    let dir = tempfile::tempdir().unwrap();
    let rounds = review::review_dir(dir.path(), "eng-379");

    let passed = rounds.join("01-initial");
    std::fs::create_dir_all(&passed).unwrap();
    std::fs::copy(fixture("codex-completed.jsonl"), passed.join("stdout.log")).unwrap();
    write(&passed.join("review.md"), "No blocking findings.\n");
    write(&passed.join("head.txt"), &format!("{OLD}\n"));

    let failed = rounds.join("02-correction");
    std::fs::create_dir_all(&failed).unwrap();
    std::fs::copy(fixture("codex-failed.jsonl"), failed.join("stdout.log")).unwrap();
    write(&failed.join("head.txt"), HEAD);

    let running = rounds.join("03-running");
    write(
        &running.join("stdout.log"),
        "{\"type\":\"thread.started\",\"thread_id\":\"t-3\"}\n",
    );

    let silent = rounds.join("04-completed-without-report");
    std::fs::create_dir_all(&silent).unwrap();
    std::fs::copy(fixture("codex-completed.jsonl"), silent.join("stdout.log")).unwrap();

    let pr = rounds.join("05-pr");
    write(
        &pr.join("run.json"),
        &serde_json::json!({"status": "stale", "engine": "codex", "model": "gpt-6-astra",
                            "session_id": "s-5", "targets": [{"head_sha": HEAD}]})
        .to_string(),
    );

    let found = review::read_rounds(&rounds).unwrap();
    let summary: Vec<(&str, RunState, Option<&str>)> = found
        .iter()
        .map(|round| (round.name.as_str(), round.state, round.session.as_deref()))
        .collect();
    assert_eq!(
        summary,
        vec![
            (
                "01-initial",
                RunState::Passed,
                Some("019a5c1e-7d2b-7c11-9a3e-0c4f5b6a7d8e")
            ),
            (
                "02-correction",
                RunState::Failed,
                Some("01a1aafe-e2cb-7c43-95ea-b191940ff7fe")
            ),
            ("03-running", RunState::Running, Some("t-3")),
            (
                "04-completed-without-report",
                RunState::Failed,
                Some("019a5c1e-7d2b-7c11-9a3e-0c4f5b6a7d8e")
            ),
            ("05-pr", RunState::Failed, Some("s-5")),
        ]
    );
    assert_eq!(found[0].head, Some(revision(OLD)));
    assert_eq!(found[4].head, Some(revision(HEAD)));
    assert_eq!(found[4].model.as_deref(), Some("gpt-6-astra"));
    assert!(
        review::read_rounds(&dir.path().join("none"))
            .unwrap()
            .is_empty()
    );
}

fn act_record(exit: Option<i32>) -> RunRecord {
    RunRecord {
        head: revision(HEAD),
        base: revision(OLD),
        dirty: false,
        command: vec!["act".into()],
        pid: None,
        exit,
    }
}

#[test]
fn act_runs_report_each_job_its_last_step_and_duration() {
    let dir = tempfile::tempdir().unwrap();
    let run = dir.path().join("1794256256-1");
    std::fs::create_dir_all(&run).unwrap();
    std::fs::copy(fixture("act-scope-passed.jsonl"), run.join("events.jsonl")).unwrap();
    let jobs = act::read_jobs(&run, &act_record(Some(0)), false).unwrap();
    assert_eq!(jobs.len(), 1);
    assert_eq!(jobs[0].id, "scope");
    assert_eq!(jobs[0].name, "CI/Select CI scope");
    assert_eq!(jobs[0].state, RunState::Passed);
    assert_eq!(jobs[0].step.as_deref(), Some("Complete job"));
    assert!(jobs[0].seconds.is_some());

    std::fs::copy(fixture("act-scope-failed.jsonl"), run.join("events.jsonl")).unwrap();
    let jobs = act::read_jobs(&run, &act_record(Some(1)), false).unwrap();
    assert_eq!(jobs[0].state, RunState::Failed);
    assert_eq!(jobs[0].step.as_deref(), Some("Set up job"));
}

#[test]
fn an_act_job_without_a_result_is_running_only_while_act_is() {
    let dir = tempfile::tempdir().unwrap();
    let run = dir.path().join("1794256256-1");
    let partial: String = std::fs::read_to_string(fixture("act-scope-passed.jsonl"))
        .unwrap()
        .lines()
        .take(5)
        .map(|line| format!("{line}\n"))
        .collect();
    write(&run.join("events.jsonl"), &partial);
    assert_eq!(
        act::read_jobs(&run, &act_record(None), true).unwrap()[0].state,
        RunState::Running
    );
    assert_eq!(
        act::read_jobs(&run, &act_record(None), false).unwrap()[0].state,
        RunState::Interrupted
    );
}

#[test]
fn the_newest_act_run_is_the_one_shown() {
    let dir = tempfile::tempdir().unwrap();
    for (name, events) in [
        ("999-1", "act-scope-passed.jsonl"),
        ("1794256256-2", "act-scope-failed.jsonl"),
    ] {
        let run = dir.path().join(name);
        std::fs::create_dir_all(&run).unwrap();
        std::fs::copy(fixture(events), run.join("events.jsonl")).unwrap();
        write(
            &run.join("run.json"),
            &serde_json::to_string(&act_record(Some(0))).unwrap(),
        );
    }
    let jobs = act::latest_jobs(dir.path()).unwrap();
    assert_eq!(jobs[0].run, "1794256256-2");
    assert_eq!(jobs[0].state, RunState::Failed);

    write(&dir.path().join("1794256257-3/run.json"), "not json");
    assert!(
        act::latest_jobs(dir.path()).is_err(),
        "an unreadable run record is an error"
    );
}

#[test]
fn linear_answers_give_the_ticket_its_parent_and_its_sub_tickets() {
    let body = std::fs::read_to_string(fixture("linear-ticket.json")).unwrap();
    let tickets = linear::parse_answer("ENG-379", &body).unwrap();
    let keys: Vec<(String, Option<String>)> = tickets
        .iter()
        .map(|t| {
            (
                t.key.to_string(),
                t.parent.as_ref().map(ToString::to_string),
            )
        })
        .collect();
    assert_eq!(
        keys,
        vec![
            ("ENG-413".into(), None),
            ("ENG-379".into(), Some("ENG-413".into())),
            ("ENG-420".into(), Some("ENG-379".into())),
        ]
    );
    assert_eq!(
        tickets[1].pull_requests,
        vec!["https://github.com/Coda-Health/coda-rust/pull/401"]
    );
    assert_eq!(tickets[2].status.category, StatusCategory::Canceled);
}

#[test]
fn linear_errors_and_missing_tickets_are_reported() {
    let error = linear::parse_answer(
        "ENG-1",
        r#"{"data":null,"errors":[{"message":"Authentication required"}]}"#,
    )
    .unwrap_err();
    assert!(
        error.to_string().contains("Authentication required"),
        "{error}"
    );
    let error = linear::parse_answer("ENG-2", r#"{"data":{"issue":null}}"#).unwrap_err();
    assert_eq!(error.to_string(), "Linear has no ticket ENG-2");
}

#[test]
fn pull_requests_roll_up_their_checks() {
    let prs =
        github::parse_list(&std::fs::read_to_string(fixture("gh-prs.json")).unwrap()).unwrap();
    let summary: Vec<(u64, PrState, CheckRollup, Option<&str>)> = prs
        .iter()
        .map(|pr| (pr.number, pr.state, pr.checks, pr.review.as_deref()))
        .collect();
    assert_eq!(
        summary,
        vec![
            (401, PrState::Open, CheckRollup::Pending, None),
            (
                402,
                PrState::Draft,
                CheckRollup::Failed,
                Some("CHANGES_REQUESTED")
            ),
            (408, PrState::Merged, CheckRollup::Passed, None),
        ]
    );
    assert!(github::parse_list("{\"message\": \"Bad credentials\"}").is_err());
}

#[test]
fn branches_and_pull_requests_find_their_ticket_through_linear_first() {
    let body = std::fs::read_to_string(fixture("linear-ticket.json")).unwrap();
    let tickets = linear::parse_answer("ENG-379", &body).unwrap();
    let index = TicketIndex::new(&["ENG".to_owned()], &tickets);
    let key = |text: &str| TicketKey::parse(text);
    // Linear's branch name has no ticket-shaped word the fallback would miss, but it is matched exactly.
    assert_eq!(
        index.branch("tyler/eng-379-finish-eng-255-ac1ac4-sources-control-basis"),
        key("ENG-379")
    );
    // A PR whose branch names no ticket still belongs to the ticket it is attached to.
    assert_eq!(
        index.pull_request(
            "https://github.com/Coda-Health/coda-rust/pull/401",
            "dev/sources"
        ),
        key("ENG-379")
    );
    assert_eq!(index.branch("eng-420-evidence-fetch"), key("ENG-420"));
    assert_eq!(index.branch("harness-timeline"), None);
}
