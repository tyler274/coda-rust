use serde_json::{Value, json};

use super::*;
use crate::ci_controller::disk::SystemDisks;
use crate::ci_controller::github::{Job, JobId, JobStatus};
use crate::ci_controller::guest::run_guest;
use crate::ci_controller::results::GUEST_FAILED;
use crate::fake_http::{Call, FakeHttp};

/// Disks that record the job directory they were asked to fill.
#[derive(Default)]
struct Recording(Mutex<Vec<PathBuf>>);

impl Disks for Recording {
    fn clone_seed(&self, seed: &Path, destination: &Path) -> Result<(), Error> {
        self.0
            .lock()
            .unwrap()
            .push(destination.parent().unwrap().to_owned());
        EmptyDisks.clone_seed(seed, destination)
    }

    fn make_work(&self, path: &Path, size: DiskSize) -> Result<(), Error> {
        EmptyDisks.make_work(path, size)
    }

    fn make_inputs(&self, path: &Path, source: &Path) -> Result<(), Error> {
        EmptyDisks.make_inputs(path, source)
    }
}

#[test]
fn guest_failure_removes_the_job_directory_and_reports() {
    let _serial = serial();
    let mut setup = Setup::new();
    setup.guest_script("booting\n", 1);
    add_request(&setup.requests(), &a12(), "ci", b"bundle", None);
    let disks = Recording::default();
    setup
        .controller(&disks)
        .run_local(&a12().parse().unwrap())
        .unwrap();
    assert_eq!(
        *disks.0.lock().unwrap(),
        [setup.config.jobs().join(format!("local-{}", a12()))]
    );
    assert!(listing(&setup.config.jobs()).is_empty());
    let result = setup.config.results().join(a12());
    assert_eq!(
        fs::read_to_string(result.join("status")).unwrap(),
        format!("{GUEST_FAILED}\n")
    );
    let log = fs::read_to_string(result.join("log")).unwrap();
    assert!(log.contains("booting"), "{log}");
    assert!(log.contains("without reporting a status"), "{log}");
}

#[test]
fn missing_seed_fails_without_leaving_a_job() {
    let setup = Setup::new();
    let controller = setup.controller(&SystemDisks);
    let error = controller
        .run_job(
            "local-x",
            setup.dir.path(),
            &setup.dir.path().join("log"),
            None,
        )
        .unwrap_err();
    assert!(error.to_string().contains("coda-ci-seed"), "{error}");
    assert!(listing(&setup.config.jobs()).is_empty());
}

#[test]
fn exit_marker_is_the_job_status() {
    let _serial = serial();
    let mut setup = Setup::new();
    setup.guest_script("noise\nCODA_JOB_EXIT=3\n", 0);
    let job_dir = setup.config.jobs().join("j");
    fs::create_dir(&job_dir).unwrap();
    let log = setup.dir.path().join("log");
    assert_eq!(
        run_guest(
            &setup.config.runner,
            &job_dir,
            &log,
            &Termination::untracked(),
            &setup.timers,
            None
        )
        .unwrap(),
        3
    );
}

#[test]
fn runner_is_deleted_after_guest_failure() {
    let _serial = serial();
    let mut setup = Setup::new();
    setup.guest_script("booting\n", 1);
    let fake = FakeHttp::builder()
        .route(
            "POST",
            "repos/Coda-Health/coda-rust/actions/runners/generate-jitconfig",
            201,
            json!({"runner": {"id": 7}, "encoded_jit_config": "jit"}),
        )
        .route(
            "DELETE",
            "repos/Coda-Health/coda-rust/actions/runners/7",
            204,
            Value::Null,
        )
        .start();
    let job = Job {
        id: JobId(9),
        name: "ci".into(),
        status: JobStatus::Queued,
        labels: vec!["cyrene-coda".into()],
    };
    let error = setup
        .controller(&EmptyDisks)
        .run_github(&api(&fake.base), &job)
        .unwrap_err();
    assert!(matches!(error, Error::Guest(_)), "{error}");
    assert_eq!(
        fake.paths().last(),
        Some(&Call::new(
            "DELETE",
            "repos/Coda-Health/coda-rust/actions/runners/7"
        ))
    );
    assert!(listing(&setup.config.jobs()).is_empty());
}

#[test]
fn sigterm_stops_the_guest_group_and_ends_the_pass() {
    let _serial = serial();
    let mut setup = Setup::new();
    let script = setup.dir.path().join("guest");
    fs::write(&script, "#!/bin/sh\nsleep 300 &\nwait\n").unwrap();
    fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
    setup.config.runner = script;
    let job_dir = setup.config.jobs().join("j");
    fs::create_dir(&job_dir).unwrap();
    let termination = Termination::untracked();
    let flag = termination.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(300));
        flag.request();
    });
    let started = std::time::Instant::now();
    let log = setup.dir.path().join("log");
    let error = run_guest(
        &setup.config.runner,
        &job_dir,
        &log,
        &termination,
        &setup.timers,
        None,
    )
    .unwrap_err();
    assert!(matches!(error, Error::Terminated), "{error}");
    assert!(started.elapsed() < Duration::from_secs(15));
}

#[test]
fn seed_installs_the_work_disk_only_when_seeding_succeeds() {
    let _serial = serial();
    let mut setup = Setup::new();
    setup.guest_script("CODA_JOB_EXIT=1\n", 0);
    let error = setup.controller(&EmptyDisks).seed().unwrap_err();
    assert!(error.to_string().contains("seeding exited 1"), "{error}");
    assert!(!setup.config.seed().exists());
    assert!(listing(&setup.config.jobs()).is_empty());
    setup.guest_script("CODA_JOB_EXIT=0\n", 0);
    setup.controller(&EmptyDisks).seed().unwrap();
    assert!(setup.config.seed().exists());
    assert!(listing(&setup.config.jobs()).is_empty());
}
