//! Running one job of each kind: a GitHub job, a local request, the seed.

use std::fs;
use std::path::Path;
use std::time::Instant;

use tempfile::TempDir;

use super::Controller;
use super::error::Error;
use super::github::{Api, Job};
use super::guest::{GuestFailure, Watch, Watcher, run_guest};
use super::results::{GUEST_FAILED, REJECTED, write_result};
use super::spool::{Claim, claim_request};
use crate::ids::RequestId;

fn remove_dir(path: &Path) -> Result<(), Error> {
    fs::remove_dir_all(path).map_err(Error::io("remove", path))
}

fn write(path: &Path, text: &str) -> Result<(), Error> {
    fs::write(path, text).map_err(Error::io("write", path))
}

impl Controller<'_> {
    fn inputs_dir(&self) -> Result<TempDir, Error> {
        tempfile::Builder::new()
            .tempdir_in(self.config.root())
            .map_err(Error::io(
                "create an inputs directory in",
                self.config.root(),
            ))
    }

    /// Run one job in its own directory under `jobs/`, and always remove it.
    pub fn run_job(
        &self,
        name: &str,
        inputs: &Path,
        log: &Path,
        watch: Option<Watcher<'_>>,
    ) -> Result<u32, Error> {
        if self.termination.requested() {
            return Err(Error::Terminated);
        }
        let job_dir = self.config.jobs().join(name);
        fs::create_dir(&job_dir).map_err(Error::io("create", &job_dir))?;
        let ran = (|| {
            self.disks
                .clone_seed(&self.config.seed(), &job_dir.join("work.img"))?;
            self.disks
                .make_inputs(&job_dir.join("inputs.img"), inputs)?;
            run_guest(
                &self.config.runner,
                &job_dir,
                log,
                &self.termination,
                &self.timers,
                watch,
            )
        })();
        Error::with_cleanup(ran, remove_dir(&job_dir))
    }

    /// Run a GitHub job on a fresh JIT runner, and always delete the runner.
    pub fn run_github(&self, api: &Api, job: &Job) -> Result<u32, Error> {
        let registration = api.register(job.id)?;
        let runner_id = registration.runner.id;
        let started = Instant::now();
        let ran = (|| {
            let inputs = self.inputs_dir()?;
            write(&inputs.path().join("mode"), "github\n")?;
            write(&inputs.path().join("jit"), &registration.encoded_jit_config)?;
            let log = self.config.logs().join(format!("github-{}.log", job.id));
            println!("Running GitHub job {} ({})", job.id, job.name);
            let mut keep_going = || -> Result<Watch, Error> {
                if started.elapsed() < self.timers.idle_grace
                    || api.runner_busy(runner_id)?
                    || api.job_queued(job.id)?
                {
                    Ok(Watch::KeepGoing)
                } else {
                    Ok(Watch::Abandoned)
                }
            };
            let status = self.run_job(
                &format!("github-{}", job.id),
                inputs.path(),
                &log,
                Some(&mut keep_going),
            )?;
            println!("GitHub job {} exited {status}", job.id);
            Ok(status)
        })();
        Error::with_cleanup(ran, api.delete_runner(runner_id).map_err(Error::from))
    }

    /// Claim and run a local request, and always try to leave it a result.
    pub fn run_local(&self, id: &RequestId) -> Result<(), Error> {
        let inputs = self.inputs_dir()?;
        let results = self.config.results();
        let claimed = claim_request(
            &self.config.requests(),
            id,
            inputs.path(),
            self.config.max_bundle_bytes,
        );
        let job = match claimed {
            Ok(Claim::Ready(job)) => job,
            // A client whose request failed to copy or leave the spool must not wait forever.
            Err(error) => {
                let message = "the controller could not claim this request; see its journal";
                return Error::with_cleanup(
                    Err(error),
                    write_result(&results, id, GUEST_FAILED, None, Some(message)),
                );
            }
            Ok(Claim::Rejected(rejection)) => {
                eprintln!("coda-ci: rejected local request {id}: {rejection}");
                return write_result(&results, id, REJECTED, None, Some(&rejection.to_string()));
            }
        };
        write(&inputs.path().join("mode"), "act\n")?;
        let log = self.config.root().join(format!("local-{id}.log"));
        println!("Running local request {id} (act -j {job})");
        match self.run_job(&format!("local-{id}"), inputs.path(), &log, None) {
            Ok(status) => {
                write_result(&results, id, status, Some(&log), None)?;
                println!("Local request {id} exited {status}");
                Ok(())
            }
            Err(Error::Guest(failure)) => write_result(
                &results,
                id,
                GUEST_FAILED,
                Some(&log),
                Some(&failure.to_string()),
            ),
            Err(error) => {
                let message = "the controller failed; see its journal";
                Error::with_cleanup(
                    Err(error),
                    write_result(&results, id, GUEST_FAILED, Some(&log), Some(message)),
                )
            }
        }
    }

    /// Pull the pinned images into a fresh work disk and make it the seed.
    pub fn seed(&self) -> Result<(), Error> {
        let job_dir = self.config.jobs().join("seed");
        fs::create_dir(&job_dir).map_err(Error::io("create", &job_dir))?;
        let seeded = (|| {
            let log = self.config.logs().join("seed.log");
            let status = {
                let inputs = self.inputs_dir()?;
                write(&inputs.path().join("mode"), "seed\n")?;
                self.disks
                    .make_work(&job_dir.join("work.img"), self.config.work_disk_size)?;
                self.disks
                    .make_inputs(&job_dir.join("inputs.img"), inputs.path())?;
                run_guest(
                    &self.config.runner,
                    &job_dir,
                    &log,
                    &self.termination,
                    &self.timers,
                    None,
                )?
            };
            if status != 0 {
                let log = log.display().to_string();
                return Err(GuestFailure::SeedFailed { status, log }.into());
            }
            let seed = self.config.seed();
            if let Some(parent) = seed.parent() {
                fs::create_dir_all(parent).map_err(Error::io("create", parent))?;
            }
            let work = job_dir.join("work.img");
            fs::rename(&work, &seed).map_err(Error::io("install the seed from", &work))?;
            println!("New seed at {}", seed.display());
            Ok(())
        })();
        Error::with_cleanup(seeded, remove_dir(&job_dir))
    }
}
