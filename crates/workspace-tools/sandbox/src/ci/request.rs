//! The CI spool's format, shared by `ci run` (the client) and the CI
//! controller.
//!
//! A request is `requests/<id>.bundle`, a git bundle whose one ref names the
//! commit, then `requests/<id>.job` holding the job name. The job file appears
//! whole by renaming `.<id>.job.part`, so its presence marks the request
//! complete. A result is `results/<id>/log`, then `results/<id>/status`
//! holding the job's exit status, which marks the result complete.

use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};

use crate::error::Error;
use crate::ids::{JobName, ParseIdError, RequestId};

/// The ref the bundle carries; the guest checks it out.
pub const BUNDLE_REF: &str = "refs/coda-ci/head";
/// The status the controller reports when the guest ended without one.
pub const GUEST_FAILED: JobStatus = JobStatus(255);
/// The status the controller reports for a request it refused to run.
pub const REJECTED: JobStatus = JobStatus(125);

/// The spool's two directories, as this side of the bind mount sees them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spool {
    pub requests: PathBuf,
    pub results: PathBuf,
}

/// One request: the job to run on the bundled commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub id: RequestId,
    pub job: JobName,
}

/// The exit status a finished result reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JobStatus(pub u8);

impl JobStatus {
    /// Reads a status file's text, such as `0\n`.
    pub fn parse(text: &str) -> Option<Self> {
        text.trim().parse::<u8>().ok().map(Self)
    }

    pub fn file_text(self) -> String {
        format!("{}\n", self.0)
    }
}

/// The job file's text for a job.
pub fn job_file_text(job: &JobName) -> String {
    format!("{job}\n")
}

/// Reads a job file's text back into a job name.
pub fn parse_job_file(text: &str) -> Result<JobName, ParseIdError> {
    text.trim().parse()
}

impl Spool {
    pub fn bundle(&self, id: &RequestId) -> PathBuf {
        self.requests.join(format!("{id}.bundle"))
    }

    pub fn job_file(&self, id: &RequestId) -> PathBuf {
        self.requests.join(format!("{id}.job"))
    }

    pub fn job_part(&self, id: &RequestId) -> PathBuf {
        self.requests.join(format!(".{id}.job.part"))
    }

    pub fn result_dir(&self, id: &RequestId) -> PathBuf {
        self.results.join(id.as_str())
    }

    pub fn log(&self, id: &RequestId) -> PathBuf {
        self.result_dir(id).join("log")
    }

    pub fn status(&self, id: &RequestId) -> PathBuf {
        self.result_dir(id).join("status")
    }

    /// Writes the bundle, then the job file by rename, so the controller never
    /// sees a request without its bundle. Both are made readable by the
    /// controller whatever the writer's umask. A failed submit removes what
    /// it wrote, since the controller never looks at a bundle without its job.
    pub fn submit(&self, request: &Request, bundle: &Path) -> Result<(), Error> {
        let dest = self.bundle(&request.id);
        let part = self.job_part(&request.id);
        let written = self.write_request(request, bundle, &dest, &part);
        if written.is_err() {
            for path in [&dest, &part] {
                if let Err(error) = fs::remove_file(path)
                    && error.kind() != io::ErrorKind::NotFound
                {
                    eprintln!("coda-sandbox: cannot remove {}: {error}", path.display());
                }
            }
        }
        written
    }

    fn write_request(
        &self,
        request: &Request,
        bundle: &Path,
        dest: &Path,
        part: &Path,
    ) -> Result<(), Error> {
        let readable = || fs::Permissions::from_mode(0o644);
        fs::copy(bundle, dest).map_err(Error::fs("copy the bundle to", dest))?;
        fs::set_permissions(dest, readable()).map_err(Error::fs("set the mode of", dest))?;
        fs::write(part, job_file_text(&request.job)).map_err(Error::fs("write", part))?;
        fs::set_permissions(part, readable()).map_err(Error::fs("set the mode of", part))?;
        let job = self.job_file(&request.id);
        fs::rename(part, &job).map_err(Error::fs("rename the job file to", &job))
    }

    /// The job's status once its result is complete, `None` while it runs.
    pub fn finished(&self, id: &RequestId) -> Result<Option<JobStatus>, Error> {
        let path = self.status(id);
        let text = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(source) if source.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(source) => return Err(Error::fs("read", &path)(source)),
        };
        JobStatus::parse(&text)
            .map(Some)
            .ok_or_else(|| Error::UnexpectedOutput {
                what: "exit status",
                source_name: path.display().to_string(),
                found: text,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn job_files_and_statuses_round_trip() {
        let job: JobName = "lightweight".parse().expect("job");
        assert_eq!(job_file_text(&job), "lightweight\n");
        assert_eq!(parse_job_file(&job_file_text(&job)), Ok(job));
        assert!(parse_job_file("two words\n").is_err());
        assert_eq!(JobStatus::parse("0\n"), Some(JobStatus(0)));
        assert_eq!(
            JobStatus::parse(&GUEST_FAILED.file_text()),
            Some(GUEST_FAILED)
        );
        assert_eq!(JobStatus::parse("256"), None);
        assert_eq!(JobStatus::parse("ok"), None);
    }
}
