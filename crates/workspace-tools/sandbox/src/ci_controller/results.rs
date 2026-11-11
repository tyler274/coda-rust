//! Local results and logs: publishing one and pruning old ones.

use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::os::unix::fs::PermissionsExt as _;
use std::path::Path;
use std::time::{Duration, SystemTime};

use super::error::Error;
use crate::ci::request;
use crate::ids::RequestId;

/// The status of a request the guest started but never reported on.
pub const GUEST_FAILED: u32 = request::GUEST_FAILED.0 as u32;
/// The status of a request that could not run.
pub const REJECTED: u32 = request::REJECTED.0 as u32;
/// How long a local result stays readable.
pub const RESULT_RETENTION: Duration = Duration::from_secs(7 * 86_400);
/// How many GitHub job logs stay on the host.
pub const GITHUB_LOGS_KEPT: usize = 20;

fn set_mode(path: &Path, mode: u32) -> Result<(), Error> {
    fs::set_permissions(path, fs::Permissions::from_mode(mode)).map_err(Error::io("chmod", path))
}

/// Publish the log, then the status; the client treats `status` as completion.
///
/// `log` moves into the result when it exists; `message` is appended to it.
pub fn write_result(
    results: &Path,
    id: &RequestId,
    status: u32,
    log: Option<&Path>,
    message: Option<&str>,
) -> Result<(), Error> {
    let directory = results.join(id.as_str());
    match fs::create_dir(&directory) {
        Err(error) if error.kind() != std::io::ErrorKind::AlreadyExists => {
            return Err(Error::io("create", &directory)(error));
        }
        _ => {}
    }
    set_mode(&directory, 0o755)?;
    let target = directory.join("log");
    if let Some(log) = log.filter(|log| log.exists()) {
        move_file(log, &target)?;
    }
    if let Some(message) = message {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&target)
            .and_then(|mut file| write!(file, "\ncoda-ci: {message}\n"))
            .map_err(Error::io("append to", &target))?;
    }
    if target.exists() {
        set_mode(&target, 0o644)?;
    }
    let temporary = directory.join("status.tmp");
    fs::write(&temporary, format!("{status}\n")).map_err(Error::io("write", &temporary))?;
    set_mode(&temporary, 0o644)?;
    let done = directory.join("status");
    fs::rename(&temporary, &done).map_err(Error::io("publish", &done))
}

/// Rename, or copy then remove when the two sit on different filesystems.
fn move_file(from: &Path, to: &Path) -> Result<(), Error> {
    match fs::rename(from, to) {
        Ok(()) => Ok(()),
        Err(error) if error.raw_os_error() == Some(rustix::io::Errno::XDEV.raw_os_error()) => {
            fs::copy(from, to).map_err(Error::io("copy", from))?;
            fs::remove_file(from).map_err(Error::io("remove", from))
        }
        Err(error) => Err(Error::io("move", from)(error)),
    }
}

/// Remove results older than the retention and all but the newest GitHub logs.
pub fn prune(results: &Path, logs: &Path, now: SystemTime) -> Result<(), Error> {
    let cutoff = now
        .checked_sub(RESULT_RETENTION)
        .unwrap_or(SystemTime::UNIX_EPOCH);
    for entry in fs::read_dir(results).map_err(Error::io("list", results))? {
        let path = entry.map_err(Error::io("list", results))?.path();
        let meta = fs::symlink_metadata(&path).map_err(Error::io("inspect", &path))?;
        let modified = meta.modified().map_err(Error::io("inspect", &path))?;
        if meta.is_dir() && modified < cutoff {
            fs::remove_dir_all(&path).map_err(Error::io("remove", &path))?;
        }
    }
    let mut github_logs = Vec::new();
    for entry in fs::read_dir(logs).map_err(Error::io("list", logs))? {
        let path = entry.map_err(Error::io("list", logs))?.path();
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();
        if name.starts_with("github-") && name.ends_with(".log") {
            let modified = fs::metadata(&path)
                .and_then(|meta| meta.modified())
                .map_err(Error::io("inspect", &path))?;
            github_logs.push((modified, path));
        }
    }
    github_logs.sort();
    let excess = github_logs.len().saturating_sub(GITHUB_LOGS_KEPT);
    for (_, old) in github_logs.into_iter().take(excess) {
        fs::remove_file(&old).map_err(Error::io("remove", &old))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_mtime(path: &Path, when: SystemTime) {
        fs::File::options()
            .write(true)
            .open(path)
            .unwrap()
            .set_modified(when)
            .unwrap();
    }

    #[test]
    fn result_publishes_log_then_status_readable_by_all() {
        let dir = tempfile::tempdir().unwrap();
        let id: RequestId = "a".repeat(12).parse().unwrap();
        let log = dir.path().join("local.log");
        fs::write(&log, "booting\n").unwrap();
        write_result(
            dir.path(),
            &id,
            GUEST_FAILED,
            Some(&log),
            Some("guest failed"),
        )
        .unwrap();
        let result = dir.path().join(id.as_str());
        assert_eq!(fs::read_to_string(result.join("status")).unwrap(), "255\n");
        assert_eq!(
            fs::read_to_string(result.join("log")).unwrap(),
            "booting\n\ncoda-ci: guest failed\n"
        );
        assert!(!log.exists());
        assert!(!result.join("status.tmp").exists());
        for path in [result.join("log"), result.join("status")] {
            assert_eq!(
                fs::metadata(path).unwrap().permissions().mode() & 0o777,
                0o644
            );
        }
    }

    #[test]
    fn prune_keeps_recent_results_and_the_newest_github_logs() {
        let dir = tempfile::tempdir().unwrap();
        let (results, logs) = (dir.path().join("results"), dir.path().join("logs"));
        fs::create_dir_all(results.join("old")).unwrap();
        fs::create_dir_all(results.join("new")).unwrap();
        fs::create_dir_all(&logs).unwrap();
        let now = SystemTime::now();
        let old = now - RESULT_RETENTION - Duration::from_secs(60);
        fs::File::open(results.join("old"))
            .unwrap()
            .set_modified(old)
            .unwrap();
        for index in 0..(GITHUB_LOGS_KEPT + 2) {
            let path = logs.join(format!("github-{index}.log"));
            fs::write(&path, "").unwrap();
            set_mtime(&path, now - Duration::from_secs(1000 - index as u64));
        }
        fs::write(logs.join("seed.log"), "").unwrap();
        prune(&results, &logs, now).unwrap();
        assert!(!results.join("old").exists());
        assert!(results.join("new").exists());
        assert!(!logs.join("github-0.log").exists());
        assert!(!logs.join("github-1.log").exists());
        assert!(logs.join("github-2.log").exists());
        assert!(logs.join("seed.log").exists());
    }
}
