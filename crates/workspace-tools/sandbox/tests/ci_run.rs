//! `coda-sandbox ci run` against a scratch spool, with a thread playing the
//! controller: it waits for a complete request, checks the bundle, and
//! publishes a result the way the controller does.

mod common;

use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

use common::{Scratch, read, write};

/// Waits for `<id>.job`, returns the id and the job name, and checks that
/// the bundle came first.
fn take_request(requests: &Path) -> (String, String) {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let job = std::fs::read_dir(requests)
            .unwrap()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .find(|path| path.extension().is_some_and(|ext| ext == "job"));
        if let Some(job) = job {
            let id = job.file_stem().unwrap().to_string_lossy().into_owned();
            assert!(requests.join(format!("{id}.bundle")).is_file());
            return (id, read(&job));
        }
        assert!(Instant::now() < deadline, "no request appeared");
        thread::sleep(Duration::from_millis(50));
    }
}

fn spool(s: &Scratch) -> PathBuf {
    let spool = s.path().join("spool");
    std::fs::create_dir_all(spool.join("requests")).unwrap();
    std::fs::create_dir_all(spool.join("results")).unwrap();
    spool
}

#[test]
fn run_queues_a_bundle_and_exits_with_the_job_status() {
    let s = Scratch::new();
    let repo = s.repo("coda-rust");
    let spool = spool(&s);
    let config = s.config(&s.path().join("build"), &repo, &spool);
    let head = s.git(&repo, &["rev-parse", "HEAD"]);
    let refs_before = s.git(&repo, &["for-each-ref"]);

    let requests = spool.join("requests");
    let results = spool.join("results");
    let controller = thread::spawn(move || {
        let (id, job) = take_request(&requests);
        let bundle = requests.join(format!("{id}.bundle"));
        let bundle_copy = requests.join(format!("{id}.seen"));
        std::fs::copy(&bundle, &bundle_copy).unwrap();
        write(&results.join(format!("{id}/log")), "job log line\n");
        write(&results.join(format!("{id}/status.tmp")), "3\n");
        std::fs::rename(
            results.join(format!("{id}/status.tmp")),
            results.join(format!("{id}/status")),
        )
        .unwrap();
        (job, bundle_copy)
    });

    let output = s.sandbox(
        &config,
        &repo,
        &["ci", "run", repo.to_str().unwrap(), "--job", "lightweight"],
    );
    let (job, bundle) = controller.join().unwrap();
    assert_eq!(job, "lightweight\n");
    assert_eq!(output.status.code(), Some(3), "{output:?}");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "job log line\n");
    let heads = s.git(
        s.path(),
        &["bundle", "list-heads", bundle.to_str().unwrap()],
    );
    assert_eq!(heads, format!("{head} refs/coda-ci/head"));
    // The source repository gains no refs.
    assert_eq!(s.git(&repo, &["for-each-ref"]), refs_before);
}

#[test]
fn run_refuses_unknown_commits_bad_jobs_and_missing_spools() {
    let s = Scratch::new();
    let repo = s.repo("coda-rust");
    let config = s.config(&s.path().join("build"), &repo, &s.path().join("absent"));
    let no_spool = s.sandbox(&config, &repo, &["ci", "run"]);
    assert_eq!(no_spool.status.code(), Some(2), "{no_spool:?}");
    assert!(String::from_utf8_lossy(&no_spool.stderr).contains("no CI spool"));

    let spool = spool(&s);
    let config = s.config(&s.path().join("build"), &repo, &spool);
    let text = std::fs::read_to_string(&config).unwrap();
    let impatient = s.path().join("impatient.toml");
    write(
        &impatient,
        &text.replace("timeout_seconds = 60", "timeout_seconds = 0"),
    );
    let timed_out = s.sandbox(&impatient, &repo, &["ci", "run"]);
    assert_eq!(timed_out.status.code(), Some(124), "{timed_out:?}");
    for entry in std::fs::read_dir(spool.join("requests")).unwrap() {
        std::fs::remove_file(entry.unwrap().path()).unwrap();
    }

    let bad_rev = s.sandbox(&config, &repo, &["ci", "run", "no-such-rev"]);
    assert!(String::from_utf8_lossy(&bad_rev.stderr).contains("is not a commit"));
    let bad_job = s.sandbox(&config, &repo, &["ci", "run", "--job", "a.b"]);
    assert!(!bad_job.status.success());
    assert!(
        std::fs::read_dir(spool.join("requests"))
            .unwrap()
            .next()
            .is_none()
    );
}
