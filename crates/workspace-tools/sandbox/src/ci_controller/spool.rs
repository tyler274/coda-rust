//! Local requests: finding the next one in the spool and claiming it.
//!
//! A client writes `<id>.bundle`, then the `<id>.job` sidecar last, into a
//! sticky world-writable directory. The controller copies both into the job's
//! inputs without following links and takes them out of the spool; the
//! bundle is never parsed on the host, only the guest opens it.
//!
//! Anything may sit under a request's name: a link, a directory, a FIFO. Each
//! claim takes its entries out of the spool whatever they are, so one bad
//! request is rejected once and never holds up the requests behind it.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read as _};
use std::os::unix::fs::{MetadataExt as _, OpenOptionsExt as _};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use rustix::fs::{CWD, OFlags, RenameFlags, renameat_with};

use super::error::Error;
use crate::ids::{JobName, RequestId};

/// Whether `path` names an entry of any kind, without following a link.
fn present(path: &Path) -> io::Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error),
    }
}

/// The id of the oldest complete request, by its sidecar's own mtime.
pub fn next_local_request(requests: &Path) -> Result<Option<RequestId>, Error> {
    let entries = fs::read_dir(requests).map_err(Error::io("list", requests))?;
    let mut oldest: Option<(SystemTime, RequestId)> = None;
    for entry in entries {
        let entry = entry.map_err(Error::io("list", requests))?;
        let name = entry.file_name();
        let Some(id) = name.to_str().and_then(|name| name.strip_suffix(".job")) else {
            continue;
        };
        let Ok(id) = id.parse::<RequestId>() else {
            continue;
        };
        let bundle = requests.join(format!("{id}.bundle"));
        if !present(&bundle).map_err(Error::io("inspect", &bundle))? {
            continue;
        }
        let sidecar = entry.path();
        let modified = match fs::symlink_metadata(&sidecar).and_then(|meta| meta.modified()) {
            Ok(modified) => modified,
            Err(error) if error.kind() == io::ErrorKind::NotFound => continue,
            Err(error) => return Err(Error::io("inspect", &sidecar)(error)),
        };
        let candidate = (modified, id);
        if oldest.as_ref().is_none_or(|current| candidate < *current) {
            oldest = Some(candidate);
        }
    }
    Ok(oldest.map(|(_, id)| id))
}

/// Why a local request cannot run; the reason goes to its result.
#[derive(Debug, thiserror::Error)]
pub enum Rejection {
    #[error("{file} cannot be opened: {source}")]
    CannotOpen {
        file: String,
        #[source]
        source: io::Error,
    },
    #[error("{file} is not a regular file")]
    NotRegular { file: String },
    #[error("{file} has {links} hard links; a request's files must have one")]
    HardLinked { file: String, links: u64 },
    #[error("{file} is {size} bytes; the limit is {limit}")]
    TooLarge { file: String, size: u64, limit: u64 },
    #[error("{file} grew past the limit of {limit} bytes")]
    GrewPastLimit { file: String, limit: u64 },
    #[error("job name {found:?} is not [A-Za-z0-9_-]{{1,64}}")]
    BadJobName { found: String },
}

/// What claiming a request produced.
#[derive(Debug)]
pub enum Claim {
    Ready(JobName),
    Rejected(Rejection),
}

/// Copy a file a client wrote without following links or exceeding `limit`.
/// `Ok(Err(_))` is the client's fault; `Err(_)` is the controller's.
fn copy_regular(
    path: &Path,
    destination: &Path,
    limit: u64,
) -> Result<Result<(), Rejection>, Error> {
    let file = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    let flags = (OFlags::NOFOLLOW | OFlags::NONBLOCK).bits();
    let mut source = match OpenOptions::new()
        .read(true)
        .custom_flags(flags.cast_signed())
        .open(path)
    {
        Ok(source) => source,
        Err(source) => return Ok(Err(Rejection::CannotOpen { file, source })),
    };
    let info = source.metadata().map_err(Error::io("inspect", path))?;
    if !info.file_type().is_file() {
        return Ok(Err(Rejection::NotRegular { file }));
    }
    // A hard link could carry a controller-owned file into the guest.
    if info.nlink() != 1 {
        let links = info.nlink();
        return Ok(Err(Rejection::HardLinked { file, links }));
    }
    if info.len() > limit {
        return Ok(Err(Rejection::TooLarge {
            file,
            size: info.len(),
            limit,
        }));
    }
    let mut target = File::create(destination).map_err(Error::io("create", destination))?;
    // The client may still be writing; never copy past the limit.
    let copied = io::copy(&mut (&mut source).take(limit + 1), &mut target)
        .map_err(Error::io("copy", path))?;
    if copied > limit {
        drop(target);
        fs::remove_file(destination).map_err(Error::io("remove", destination))?;
        return Ok(Err(Rejection::GrewPastLimit { file, limit }));
    }
    Ok(Ok(()))
}

/// Take one spool entry out without following it: unlink, or rmdir, or,
/// when neither works (a directory the controller cannot empty), rename it
/// aside. Only a failed rename fails the pass.
fn take_out(path: &Path) -> Result<(), Error> {
    let removed = match fs::remove_file(path) {
        Err(error) if error.kind() == io::ErrorKind::IsADirectory => fs::remove_dir(path),
        removed => removed,
    };
    let error = match removed {
        Ok(()) => return Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => error,
    };
    let aside = quarantine(path)?;
    eprintln!(
        "coda-ci: cannot remove {} ({error}); moved it to {}",
        path.display(),
        aside.display()
    );
    Ok(())
}

/// Rename `path` to a fresh dot name beside it, which no request id matches.
/// The name is unpredictable and never replaces an entry already there.
fn quarantine(path: &Path) -> Result<PathBuf, Error> {
    use std::hash::{BuildHasher as _, Hasher as _};
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let mut last = None;
    for _ in 0..3 {
        let mut hasher = std::collections::hash_map::RandomState::new().build_hasher();
        hasher.write(name.as_bytes());
        let aside = path.with_file_name(format!(".{name}.rejected-{:016x}", hasher.finish()));
        match renameat_with(CWD, path, CWD, &aside, RenameFlags::NOREPLACE) {
            Ok(()) => return Ok(aside),
            Err(errno) => last = Some(io::Error::from(errno)),
        }
    }
    let source = last.unwrap_or_else(|| io::Error::other("no attempt was made"));
    Err(Error::io("move aside", path)(source))
}

/// Copy a request into `destination` as `job` and `repo.bundle`, then take
/// it out of the spool whatever happened.
pub fn claim_request(
    requests: &Path,
    id: &RequestId,
    destination: &Path,
    max_bundle_bytes: u64,
) -> Result<Claim, Error> {
    let bundle = requests.join(format!("{id}.bundle"));
    let sidecar = requests.join(format!("{id}.job"));
    let claimed = copy_request(&bundle, &sidecar, destination, max_bundle_bytes);
    let taken_out = take_out(&bundle).and(take_out(&sidecar));
    Error::with_cleanup(claimed, taken_out)
}

fn copy_request(
    bundle: &Path,
    sidecar: &Path,
    destination: &Path,
    max_bundle_bytes: u64,
) -> Result<Claim, Error> {
    let job_file = destination.join("job");
    if let Err(rejection) = copy_regular(sidecar, &job_file, 128)? {
        return Ok(Claim::Rejected(rejection));
    }
    let raw = fs::read(&job_file).map_err(Error::io("read", &job_file))?;
    let text: String = raw
        .iter()
        .map(|&b| {
            if b.is_ascii() {
                char::from(b)
            } else {
                char::REPLACEMENT_CHARACTER
            }
        })
        .collect();
    let text = text.trim();
    let Ok(job) = text.parse::<JobName>() else {
        fs::remove_file(&job_file).map_err(Error::io("remove", &job_file))?;
        let found = text.chars().take(64).collect();
        return Ok(Claim::Rejected(Rejection::BadJobName { found }));
    };
    fs::write(&job_file, format!("{job}\n")).map_err(Error::io("write", &job_file))?;
    let bundle_copy = destination.join("repo.bundle");
    if let Err(rejection) = copy_regular(bundle, &bundle_copy, max_bundle_bytes)? {
        return Ok(Claim::Rejected(rejection));
    }
    Ok(Claim::Ready(job))
}
