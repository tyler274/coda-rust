//! `coda-sandbox ci run [<repo-path-or-commit>] [--job <job>]`.

use std::fs;
use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::ci::request::{BUNDLE_REF, JobStatus, Request, Spool};
use crate::config::Ci;
use crate::error::Error;
use crate::git::{git, objects_dir};
use crate::ids::{JobName, RequestId};

const POLL: Duration = Duration::from_secs(5);

/// What to test: a repository's HEAD, or a commit in the current repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    Repo(PathBuf),
    Commit(String),
}

impl Target {
    /// A path to an existing directory is a repository; anything else is a
    /// revision in the current directory's repository.
    pub fn from_arg(arg: Option<&str>) -> Self {
        match arg {
            None => Self::Repo(PathBuf::from(".")),
            Some(arg) if Path::new(arg).is_dir() => Self::Repo(PathBuf::from(arg)),
            Some(arg) => Self::Commit(arg.to_owned()),
        }
    }

    fn repo_and_rev(&self) -> (PathBuf, &str) {
        match self {
            Self::Repo(path) => (path.clone(), "HEAD"),
            Self::Commit(rev) => (PathBuf::from("."), rev),
        }
    }
}

/// A fresh request id from the kernel's random source.
pub fn new_request_id() -> Result<RequestId, Error> {
    let path = Path::new("/dev/urandom");
    let mut bytes = [0u8; 6];
    fs::File::open(path)
        .and_then(|mut file| file.read_exact(&mut bytes))
        .map_err(|source| Error::Fs {
            op: "read",
            path: path.to_owned(),
            source,
        })?;
    Ok(RequestId::from_bytes(bytes))
}

/// Bundles `commit` from a scratch repository that borrows `objects`, so the
/// source repository gains no refs.
fn bundle(scratch: &Path, objects: &Path, commit: &str) -> Result<PathBuf, Error> {
    let repo = scratch.join("repo");
    git(scratch)
        .args(["init", "--quiet", "--bare"])
        .arg(&repo)
        .run()?;
    let alternates = repo.join("objects/info/alternates");
    fs::write(&alternates, format!("{}\n", objects.display())).map_err(|source| Error::Fs {
        op: "write",
        path: alternates.clone(),
        source,
    })?;
    git(&repo).args(["update-ref", BUNDLE_REF, commit]).run()?;
    let bundle = scratch.join("repo.bundle");
    git(&repo)
        .args(["bundle", "create", "--quiet"])
        .arg(&bundle)
        .arg(BUNDLE_REF)
        .run()?;
    Ok(bundle)
}

/// Queues the job, waits for its result, prints the log, and returns the
/// job's exit status.
pub fn run(ci: &Ci, target: &Target, job: &JobName) -> Result<JobStatus, Error> {
    let spool = Spool {
        requests: ci.requests(),
        results: ci.results(),
    };
    for dir in [&spool.requests, &spool.results] {
        if !dir.is_dir() {
            return Err(Error::NoSpool { path: dir.clone() });
        }
    }
    let (repo, rev) = target.repo_and_rev();
    let commit = git(&repo)
        .args(["rev-parse", "--verify", "--quiet"])
        .arg(format!("{rev}^{{commit}}"))
        .line_if_ok()?
        .ok_or_else(|| Error::NotACommit {
            rev: rev.to_owned(),
            repo: repo.clone(),
        })?;
    if matches!(target, Target::Repo(_)) {
        let dirty = git(&repo)
            .args(["status", "--porcelain", "--untracked-files=no"])
            .output()?;
        if !dirty.trim().is_empty() {
            eprintln!("coda-sandbox: uncommitted changes are not sent; testing {commit}");
        }
    }
    let scratch = tempfile::tempdir().map_err(|source| Error::Fs {
        op: "create a scratch directory in",
        path: std::env::temp_dir(),
        source,
    })?;
    let bundle = bundle(scratch.path(), &objects_dir(&repo)?, &commit)?;
    let request = Request {
        id: new_request_id()?,
        job: job.clone(),
    };
    spool.submit(&request, &bundle)?;
    drop(scratch);
    let id = &request.id;
    eprintln!("coda-sandbox: queued {id}: act -j {job} at {commit}");

    let deadline = Instant::now() + Duration::from_secs(ci.timeout_seconds);
    let status = loop {
        if let Some(status) = spool.finished(id)? {
            break status;
        }
        if Instant::now() >= deadline {
            return Err(Error::CiTimeout {
                id: id.clone(),
                seconds: ci.timeout_seconds,
            });
        }
        sleep(POLL);
    };
    match fs::read_to_string(spool.log(id)) {
        Ok(log) => print!("{log}"),
        Err(error) => eprintln!("coda-sandbox: {id} has no log: {error}"),
    }
    eprintln!("coda-sandbox: {id} exited {}", status.0);
    Ok(status)
}
