use super::*;
use crate::ci_controller::results::REJECTED;
use crate::ci_controller::spool::{Claim, Rejection, claim_request, next_local_request};
use crate::ids::RequestId;

fn id(text: &str) -> RequestId {
    text.parse().expect("request id")
}

/// A spool and an inputs directory side by side.
fn spool() -> (TempDir, PathBuf, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let requests = dir.path().join("requests");
    let inputs = dir.path().join("inputs");
    fs::create_dir(&requests).unwrap();
    fs::create_dir(&inputs).unwrap();
    (dir, requests, inputs)
}

#[test]
fn oldest_complete_request_is_next() {
    let (_dir, requests, _) = spool();
    add_request(&requests, &"b".repeat(12), "ci", b"bundle", Some(200));
    add_request(&requests, &a12(), "ci", b"bundle", Some(300));
    fs::write(
        requests.join(format!("{}.bundle", "c".repeat(12))),
        "no sidecar yet",
    )
    .unwrap();
    assert_eq!(
        next_local_request(&requests).unwrap(),
        Some(id(&"b".repeat(12)))
    );
}

#[test]
fn names_outside_the_pattern_are_ignored() {
    let (_dir, requests, _) = spool();
    let traversal = format!("..{}", "a".repeat(10));
    for name in [
        "A".repeat(12),
        "a".repeat(11),
        "a".repeat(13),
        traversal,
        "g".repeat(12),
    ] {
        fs::write(requests.join(format!("{name}.bundle")), "x").unwrap();
        fs::write(requests.join(format!("{name}.job")), "ci").unwrap();
    }
    assert_eq!(next_local_request(&requests).unwrap(), None);
}

#[test]
fn an_unreadable_spool_is_an_error_not_an_empty_queue() {
    let (dir, _, _) = spool();
    assert!(next_local_request(&dir.path().join("absent")).is_err());
}

#[test]
fn claim_copies_and_removes_the_request() {
    let (_dir, requests, inputs) = spool();
    add_request(&requests, &a12(), "build-wasm", b"bundle", None);
    let claim = claim_request(&requests, &id(&a12()), &inputs, 1024).unwrap();
    assert!(matches!(claim, Claim::Ready(job) if job.as_str() == "build-wasm"));
    assert_eq!(fs::read(inputs.join("repo.bundle")).unwrap(), b"bundle");
    assert_eq!(
        fs::read_to_string(inputs.join("job")).unwrap(),
        "build-wasm\n"
    );
    assert!(listing(&requests).is_empty());
}

#[test]
fn claim_rejects_bad_job_names_sizes_and_links() {
    let long = "a".repeat(65);
    let cases: [(&str, &[u8]); 3] = [
        ("ci; rm -rf /", b"bundle"),
        (&long, b"bundle"),
        ("ci", &[b'x'; 2048]),
    ];
    for (job, bundle) in cases {
        let (_dir, requests, inputs) = spool();
        add_request(&requests, &a12(), job, bundle, None);
        let claim = claim_request(&requests, &id(&a12()), &inputs, 1024).unwrap();
        assert!(
            matches!(claim, Claim::Rejected(_)),
            "{job:?} {}",
            bundle.len()
        );
        assert!(listing(&requests).is_empty());
        assert!(!inputs.join("repo.bundle").exists());
    }
    let (dir, requests, inputs) = spool();
    let secret = dir.path().join("secret");
    fs::write(&secret, "host secret").unwrap();
    std::os::unix::fs::symlink(&secret, requests.join(format!("{}.bundle", a12()))).unwrap();
    fs::write(requests.join(format!("{}.job", a12())), "ci").unwrap();
    let claim = claim_request(&requests, &id(&a12()), &inputs, 1024).unwrap();
    let Claim::Rejected(rejection) = claim else {
        panic!("a link must be rejected")
    };
    assert!(matches!(rejection, Rejection::CannotOpen { .. }));
    assert!(
        rejection.to_string().contains("cannot be opened"),
        "{rejection}"
    );
    assert!(secret.exists());
    assert!(listing(&requests).is_empty());
}

#[test]
fn claim_rejects_a_fifo_without_blocking() {
    use rustix::fs::{CWD, FileType, Mode, mknodat};
    let (_dir, requests, inputs) = spool();
    let bundle = requests.join(format!("{}.bundle", a12()));
    mknodat(CWD, &bundle, FileType::Fifo, Mode::from_raw_mode(0o600), 0).unwrap();
    fs::write(requests.join(format!("{}.job", a12())), "ci").unwrap();
    let claim = claim_request(&requests, &id(&a12()), &inputs, 1024).unwrap();
    assert!(
        matches!(claim, Claim::Rejected(Rejection::NotRegular { .. })),
        "{claim:?}"
    );
    assert!(listing(&requests).is_empty());
}

#[test]
fn rejected_request_gets_a_result() {
    let setup = Setup::new();
    add_request(&setup.requests(), &a12(), "bad job", b"bundle", None);
    // No guest may run: the runner does not exist.
    setup
        .controller(&EmptyDisks)
        .run_local(&id(&a12()))
        .unwrap();
    assert!(listing(&setup.config.jobs()).is_empty());
    let result = setup.config.results().join(a12());
    assert_eq!(
        fs::read_to_string(result.join("status")).unwrap(),
        format!("{REJECTED}\n")
    );
    assert!(
        fs::read_to_string(result.join("log"))
            .unwrap()
            .contains("job name")
    );
}

#[test]
fn unadmitted_request_stays_queued() {
    let setup = Setup::new();
    add_request(&setup.requests(), &a12(), "ci", b"bundle", None);
    setup.set_available(8 * 1024 * MIB);
    setup
        .controller(&EmptyDisks)
        .one_pass(&Queues::LocalOnly)
        .unwrap();
    assert!(setup.requests().join(format!("{}.job", a12())).exists());
    assert!(listing(&setup.config.results()).is_empty());
}
