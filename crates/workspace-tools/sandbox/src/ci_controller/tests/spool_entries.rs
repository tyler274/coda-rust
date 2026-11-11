//! Entries in the world-writable spool that are not regular files: each is
//! rejected and taken out of the spool, and the queue keeps moving.

use std::os::unix::fs::symlink;

use super::*;
use crate::ci_controller::results::REJECTED;

/// Newer than anything planted without an explicit mtime.
const LATER: u64 = 4_000_000_000;

fn status(setup: &Setup, id: &str) -> String {
    fs::read_to_string(setup.config.results().join(id).join("status")).expect("a result")
}

/// Run passes until the honest request `b…` has a result, at most three.
fn drain(setup: &Setup) {
    let controller = setup.controller(&EmptyDisks);
    for _ in 0..3 {
        controller.one_pass(&Queues::LocalOnly).expect("a pass");
        if setup.config.results().join("b".repeat(12)).exists() {
            return;
        }
    }
}

fn assert_spool_moves_on(setup: &Setup) {
    drain(setup);
    assert_eq!(status(setup, &a12()), format!("{REJECTED}\n"));
    assert_eq!(status(setup, &"b".repeat(12)), "0\n");
    let left: Vec<String> = listing(&setup.requests())
        .into_iter()
        .filter(|name| !name.starts_with('.'))
        .collect();
    assert!(left.is_empty(), "{left:?}");
}

#[test]
fn a_directory_request_is_rejected_and_a_later_request_still_runs() {
    let _serial = serial();
    let mut setup = Setup::new();
    setup.guest_script("CODA_JOB_EXIT=0\n", 0);
    let requests = setup.requests();
    fs::create_dir(requests.join(format!("{}.bundle", a12()))).unwrap();
    fs::create_dir(requests.join(format!("{}.job", a12()))).unwrap();
    add_request(&requests, &"b".repeat(12), "ci", b"bundle", Some(LATER));
    assert_spool_moves_on(&setup);
}

#[test]
fn a_directory_that_cannot_be_emptied_is_quarantined() {
    let _serial = serial();
    let mut setup = Setup::new();
    setup.guest_script("CODA_JOB_EXIT=0\n", 0);
    let requests = setup.requests();
    for name in [format!("{}.bundle", a12()), format!("{}.job", a12())] {
        fs::create_dir(requests.join(&name)).unwrap();
        fs::write(requests.join(&name).join("inside"), "x").unwrap();
    }
    add_request(&requests, &"b".repeat(12), "ci", b"bundle", Some(LATER));
    assert_spool_moves_on(&setup);
    let quarantined: Vec<String> = listing(&requests)
        .into_iter()
        .filter(|name| name.contains(".rejected-"))
        .collect();
    assert_eq!(quarantined.len(), 2, "{quarantined:?}");
}

#[test]
fn fifo_unreadable_and_hard_linked_requests_do_not_hold_up_the_queue() {
    use rustix::fs::{CWD, FileType, Mode, mknodat};
    let _serial = serial();
    let mut setup = Setup::new();
    setup.guest_script("CODA_JOB_EXIT=0\n", 0);
    let requests = setup.requests();
    let plant = |id: &str, mtime: u64| add_request(&requests, id, "ci", b"bundle", Some(mtime));
    // A FIFO sidecar no writer ever opens.
    plant(&a12(), 100);
    let fifo = requests.join(format!("{}.job", a12()));
    fs::remove_file(&fifo).unwrap();
    mknodat(CWD, &fifo, FileType::Fifo, Mode::from_raw_mode(0o622), 0).unwrap();
    // A bundle nobody may read; root's tests read it anyway, so it may run.
    let unreadable = "c".repeat(12);
    plant(&unreadable, 200);
    let bundle = requests.join(format!("{unreadable}.bundle"));
    fs::set_permissions(&bundle, fs::Permissions::from_mode(0o000)).unwrap();
    // A bundle hard-linked to a file outside the spool.
    let linked = "d".repeat(12);
    plant(&linked, 300);
    let outside = setup.dir.path().join("controller-owned");
    fs::write(&outside, "host secret").unwrap();
    let bundle = requests.join(format!("{linked}.bundle"));
    fs::remove_file(&bundle).unwrap();
    fs::hard_link(&outside, &bundle).unwrap();
    add_request(&requests, &"b".repeat(12), "ci", b"bundle", Some(LATER));
    let controller = setup.controller(&EmptyDisks);
    for _ in 0..4 {
        controller.one_pass(&Queues::LocalOnly).expect("a pass");
    }
    assert_eq!(status(&setup, &a12()), format!("{REJECTED}\n"));
    assert!(
        setup
            .config
            .results()
            .join(&unreadable)
            .join("status")
            .exists()
    );
    assert_eq!(status(&setup, &linked), format!("{REJECTED}\n"));
    assert_eq!(status(&setup, &"b".repeat(12)), "0\n");
    assert!(listing(&requests).is_empty(), "{:?}", listing(&requests));
    assert_eq!(fs::read_to_string(&outside).unwrap(), "host secret");
}

#[test]
fn symlinked_requests_are_rejected_without_following_them() {
    let _serial = serial();
    let mut setup = Setup::new();
    setup.guest_script("CODA_JOB_EXIT=0\n", 0);
    let requests = setup.requests();
    let elsewhere = setup.dir.path().join("elsewhere");
    fs::create_dir(&elsewhere).unwrap();
    fs::write(elsewhere.join("secret"), "host secret").unwrap();
    symlink(&elsewhere, requests.join(format!("{}.bundle", a12()))).unwrap();
    symlink(
        elsewhere.join("secret"),
        requests.join(format!("{}.job", a12())),
    )
    .unwrap();
    symlink(
        elsewhere.join("absent"),
        requests.join(format!("{}.bundle", "c".repeat(12))),
    )
    .unwrap();
    fs::write(requests.join(format!("{}.job", "c".repeat(12))), "ci").unwrap();
    add_request(&requests, &"b".repeat(12), "ci", b"bundle", Some(LATER));
    assert_spool_moves_on(&setup);
    assert_eq!(status(&setup, &"c".repeat(12)), format!("{REJECTED}\n"));
    assert_eq!(
        fs::read_to_string(elsewhere.join("secret")).unwrap(),
        "host secret"
    );
}
