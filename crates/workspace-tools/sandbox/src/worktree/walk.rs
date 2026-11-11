//! Small filesystem readings the worktree commands share.

use std::fs;
use std::io;
use std::path::Path;
use std::time::SystemTime;

use crate::error::Error;
use crate::ids::Slug;

/// Directories in `dir` whose names are slugs, sorted; a missing `dir` has
/// none. Hidden entries (`.seed`, `.verification.*`) are not slugs.
pub fn visible_dirs(dir: &Path) -> Result<Vec<Slug>, Error> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(source) if source.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(source) => return Err(Error::fs("list", dir)(source)),
    };
    let mut slugs = Vec::new();
    for entry in entries {
        let entry = entry.map_err(Error::fs("list", dir))?;
        let is_dir = entry
            .file_type()
            .map_err(Error::fs("inspect", &entry.path()))?
            .is_dir();
        let name = entry.file_name();
        if let (true, Some(Ok(slug))) = (is_dir, name.to_str().map(str::parse::<Slug>)) {
            slugs.push(slug);
        }
    }
    slugs.sort();
    Ok(slugs)
}

/// The newest modification time of any entry within `depth` levels below
/// `dir`, not counting `dir` itself. Cargo adds entries to `deps/`,
/// `.fingerprint/` and `incremental/` on every build, so three levels are
/// enough to tell when a target was last built.
pub fn newest_below(dir: &Path, depth: usize) -> Result<Option<SystemTime>, Error> {
    if depth == 0 {
        return Ok(None);
    }
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(source) if source.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(source) => return Err(Error::fs("list", dir)(source)),
    };
    let mut newest: Option<SystemTime> = None;
    for entry in entries {
        let entry = entry.map_err(Error::fs("list", dir))?;
        let path = entry.path();
        let meta = fs::symlink_metadata(&path).map_err(Error::fs("inspect", &path))?;
        let modified = meta
            .modified()
            .map_err(Error::fs("read the time of", &path))?;
        newest = newest.max(Some(modified));
        if meta.is_dir() {
            newest = newest.max(newest_below(&path, depth - 1)?);
        }
    }
    Ok(newest)
}

/// Removes `path` and everything under it; a missing path is already gone.
pub fn remove_tree(path: &Path) -> Result<(), Error> {
    match fs::remove_dir_all(path) {
        Ok(()) => Ok(()),
        Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(Error::fs("remove", path)(source)),
    }
}

pub fn create_dir(path: &Path) -> Result<(), Error> {
    fs::create_dir_all(path).map_err(Error::fs("create", path))
}

pub fn rename(from: &Path, to: &Path) -> Result<(), Error> {
    fs::rename(from, to).map_err(Error::fs("move into place", to))
}

pub fn write(path: &Path, text: &str) -> Result<(), Error> {
    fs::write(path, text).map_err(Error::fs("write", path))
}

/// The file's text, `None` when it does not exist.
pub fn read(path: &Path) -> Result<Option<String>, Error> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(Error::fs("read", path)(source)),
    }
}

/// Points the symlink `link` at `target`, replacing any symlink there by an
/// atomic rename, as `ln -sfn` then `mv -T` would.
pub fn replace_symlink(target: &Path, link: &Path) -> Result<(), Error> {
    let name = link
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    let staged = link.with_file_name(format!(".{name}.new"));
    remove_file(&staged)?;
    std::os::unix::fs::symlink(target, &staged).map_err(Error::fs("create the link", &staged))?;
    rename(&staged, link)
}

/// Removes a file or symlink; a missing one is already gone.
pub fn remove_file(path: &Path) -> Result<(), Error> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(Error::fs("remove", path)(source)),
    }
}
