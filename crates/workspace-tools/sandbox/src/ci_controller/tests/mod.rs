//! Behavior tests of the controller against temp directories, fixture memory
//! reports and a fake GitHub API.

mod cleanup;
mod fake_github;
mod github_queue;
mod guest_failure;
mod local_requests;
mod reconcile;
mod spool_entries;

use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, SystemTime};

use tempfile::TempDir;

use super::config::ControllerConfig;
use super::disk::{DiskSize, Disks};
use super::error::Error;
use super::github::{Api, Token};
use super::guest::Timers;
use super::memory::{MIB, MemoryFiles};
use super::termination::Termination;
use super::{Controller, Queues};

/// Tests that write a script and run it hold this, so no other test forks
/// while the script is still open for writing (ETXTBSY).
static PROCESSES: Mutex<()> = Mutex::new(());

pub fn serial() -> MutexGuard<'static, ()> {
    PROCESSES
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Disks without reflinks or `mkfs`: empty files where the images go.
pub struct EmptyDisks;

impl Disks for EmptyDisks {
    fn clone_seed(&self, _seed: &Path, destination: &Path) -> Result<(), Error> {
        fs::write(destination, "").map_err(Error::io("create", destination))
    }

    fn make_work(&self, path: &Path, _size: DiskSize) -> Result<(), Error> {
        fs::write(path, "").map_err(Error::io("create", path))
    }

    fn make_inputs(&self, path: &Path, _source: &Path) -> Result<(), Error> {
        fs::write(path, "").map_err(Error::io("create", path))
    }
}

pub struct Setup {
    pub dir: TempDir,
    pub config: ControllerConfig,
    pub memory: MemoryFiles,
    pub timers: Timers,
}

impl Setup {
    /// A controller root with its directories, a 12 GiB guest and a 4 GiB
    /// margin, a 1024-byte bundle limit, and 64 GiB available.
    pub fn new() -> Self {
        let dir = tempfile::tempdir().expect("temp dir");
        let config = ControllerConfig {
            root: dir.path().to_owned(),
            runner: PathBuf::from("/bin/false"),
            guest_mem_mib: 12288,
            margin_mib: 4096,
            max_bundle_bytes: 1024,
            work_disk_size: "1G".parse().expect("size"),
            api: "http://127.0.0.1:9".to_owned(),
        };
        for directory in [
            config.jobs(),
            config.requests(),
            config.results(),
            config.logs(),
        ] {
            fs::create_dir_all(directory).expect("controller directories");
        }
        let memory = MemoryFiles {
            meminfo: dir.path().join("meminfo"),
            arcstats: dir.path().join("arcstats-absent"),
        };
        let setup = Self {
            dir,
            config,
            memory,
            timers: Timers::PRODUCTION,
        };
        setup.set_available(64 * 1024 * MIB);
        setup
    }

    pub fn set_available(&self, bytes: u64) {
        let text = format!("MemTotal: 1 kB\nMemAvailable: {} kB\n", bytes / 1024);
        fs::write(&self.memory.meminfo, text).expect("meminfo fixture");
    }

    /// A guest that prints `console` and exits with `status`.
    pub fn guest_script(&mut self, console: &str, status: u8) {
        let path = self.dir.path().join("guest");
        fs::write(
            &path,
            format!("#!/bin/sh\nprintf '%s' '{console}'\nexit {status}\n"),
        )
        .expect("script");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("chmod script");
        self.config.runner = path;
    }

    pub fn controller<'a>(&self, disks: &'a dyn Disks) -> Controller<'a> {
        Controller {
            config: self.config.clone(),
            disks,
            memory: self.memory.clone(),
            termination: Termination::untracked(),
            timers: self.timers,
        }
    }

    pub fn requests(&self) -> PathBuf {
        self.config.requests()
    }
}

pub fn github(base: &str) -> Queues {
    Queues::GitHubThenLocal(api(base))
}

pub fn api(base: &str) -> Api {
    Api::new(base, Token::new("token").expect("token"))
}

/// Write a request as a client does: the bundle, then the sidecar.
pub fn add_request(requests: &Path, id: &str, job: &str, bundle: &[u8], mtime: Option<u64>) {
    fs::write(requests.join(format!("{id}.bundle")), bundle).expect("bundle");
    let sidecar = requests.join(format!("{id}.job"));
    fs::write(&sidecar, format!("{job}\n")).expect("sidecar");
    if let Some(seconds) = mtime {
        let when = SystemTime::UNIX_EPOCH + Duration::from_secs(seconds);
        fs::File::options()
            .write(true)
            .open(&sidecar)
            .expect("sidecar")
            .set_modified(when)
            .expect("mtime");
    }
}

pub fn listing(directory: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(directory)
        .expect("list")
        .map(|entry| {
            entry
                .expect("entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort();
    names
}

pub fn a12() -> String {
    "a".repeat(12)
}
