//! `coda-sandbox ci-controller`: run Coda CI jobs one at a time, each in a
//! disposable Firecracker guest.
//!
//! One pass takes at most one job: a queued GitHub job labelled `cyrene-coda`
//! first, then the oldest local act request from the spool. The guest gets a
//! reflink clone of the seed work disk and a read-only inputs disk; its serial
//! console is the only thing that comes back.

pub mod config;
pub mod disk;
pub mod error;
pub mod github;
pub mod guest;
pub mod jobs;
pub mod memory;
pub mod results;
pub mod spool;
pub mod termination;

#[cfg(test)]
mod tests;

use std::fs::{self, File};
use std::io;
use std::os::unix::fs::PermissionsExt as _;
use std::path::Path;
use std::time::SystemTime;

use self::config::ControllerConfig;
use self::disk::Disks;
use self::error::Error;
use self::github::{Api, Job, Token, is_controller_runner};
use self::guest::Timers;
use self::memory::{Admission, MemoryFiles};
use self::termination::Termination;
use crate::ids::RequestId;

/// Which queues this pass may take from. Without a token only local
/// requests run.
pub enum Queues {
    GitHubThenLocal(Api),
    LocalOnly,
}

/// The one job a pass would run; a GitHub job carries the client that found it.
pub enum Next<'q> {
    GitHub(&'q Api, Job),
    Local(RequestId),
    Idle,
}

/// What one invocation does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Prune old results, then run at most one job.
    Pass,
    /// Build a fresh seed work disk.
    Seed,
}

pub struct Controller<'a> {
    pub config: ControllerConfig,
    pub disks: &'a dyn Disks,
    pub memory: MemoryFiles,
    /// Records SIGTERM; a running guest is stopped and the pass ends.
    pub termination: Termination,
    pub timers: Timers,
}

/// The GitHub token systemd passes with `LoadCredential=github-token:…`.
/// An unset credentials directory, an absent file or an empty one means none.
pub fn read_token(credentials_directory: Option<&Path>) -> Result<Option<Token>, Error> {
    let Some(directory) = credentials_directory else {
        return Ok(None);
    };
    let path = directory.join("github-token");
    match fs::read_to_string(&path) {
        Ok(text) => Ok(Token::new(&text)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(Error::Read { path, source }),
    }
}

impl Controller<'_> {
    /// Remove what an interrupted pass left: idle runners and job directories.
    pub fn reconcile(&self, queues: &Queues) -> Result<(), Error> {
        if let Queues::GitHubThenLocal(api) = queues {
            let mut stale = Vec::new();
            for runner in api.runners()? {
                if is_controller_runner(&runner.name) {
                    if runner.busy {
                        return Err(Error::RunnerBusy);
                    }
                    stale.push(runner.id);
                }
            }
            for id in stale {
                api.delete_runner(id)?;
            }
        }
        let jobs = self.config.jobs();
        for entry in fs::read_dir(&jobs).map_err(Error::io("list", &jobs))? {
            let path = entry.map_err(Error::io("list", &jobs))?.path();
            let meta = fs::symlink_metadata(&path).map_err(Error::io("inspect", &path))?;
            let removed = if meta.is_dir() {
                fs::remove_dir_all(&path)
            } else {
                fs::remove_file(&path)
            };
            removed.map_err(Error::io("remove", &path))?;
        }
        Ok(())
    }

    /// The GitHub queue first, then the oldest complete local request.
    pub fn next<'q>(&self, queues: &'q Queues) -> Result<Next<'q>, Error> {
        if let Queues::GitHubThenLocal(api) = queues
            && let Some(job) = api.next_job()?
        {
            return Ok(Next::GitHub(api, job));
        }
        Ok(match spool::next_local_request(&self.config.requests())? {
            Some(id) => Next::Local(id),
            None => Next::Idle,
        })
    }

    pub fn admission(&self) -> Result<Admission, Error> {
        let available = memory::available(&self.memory)?;
        Ok(memory::admit(
            self.config.guest_mem_mib,
            self.config.margin_mib,
            available,
        ))
    }

    /// Run at most one job; a job that does not fit waits for the next pass.
    pub fn one_pass(&self, queues: &Queues) -> Result<(), Error> {
        let next = self.next(queues)?;
        let which = match &next {
            Next::Idle => return Ok(()),
            Next::GitHub(..) => "GitHub job",
            Next::Local(_) => "local request",
        };
        if self.admission()? == Admission::Wait {
            println!("Not enough memory for a guest; {which} waits.");
            return Ok(());
        }
        match next {
            Next::GitHub(api, job) => self.run_github(api, &job).map(drop),
            Next::Local(id) => self.run_local(&id),
            Next::Idle => Ok(()),
        }
    }

    /// Prepare the directories, take the lock, reconcile, then run `mode`.
    pub fn run(&self, mode: Mode, token: Option<Token>) -> Result<(), Error> {
        for directory in [
            self.config.jobs(),
            self.config.results(),
            self.config.logs(),
        ] {
            fs::create_dir_all(&directory).map_err(Error::io("create", &directory))?;
        }
        let results = self.config.results();
        fs::set_permissions(&results, fs::Permissions::from_mode(0o755))
            .map_err(Error::io("chmod", &results))?;
        let lock_path = self.config.lock();
        let lock = File::create(&lock_path).map_err(Error::io("create", &lock_path))?;
        rustix::fs::flock(&lock, rustix::fs::FlockOperation::LockExclusive)
            .map_err(|errno| Error::io("lock", &lock_path)(errno.into()))?;
        let queues = match token {
            Some(token) => Queues::GitHubThenLocal(Api::new(&self.config.api, token)),
            None => Queues::LocalOnly,
        };
        let outcome = self.termination.deferred(|| {
            self.reconcile(&queues)?;
            match mode {
                Mode::Seed => self.seed(),
                Mode::Pass => {
                    results::prune(&results, &self.config.logs(), SystemTime::now())?;
                    self.one_pass(&queues)
                }
            }
        });
        match outcome {
            Ok(()) if self.termination.requested() => Err(Error::Terminated),
            outcome => outcome,
        }
    }
}
