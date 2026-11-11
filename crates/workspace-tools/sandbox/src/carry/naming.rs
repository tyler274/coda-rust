//! Carry's names for worktrees and their refs.

use std::collections::HashSet;
use std::path::Path;

/// A worktree's name in `refs/carry/<name>/...`: its directory's name with
/// anything outside `[A-Za-z0-9._-]` turned into `-` and one leading `.`
/// dropped. Unlike a [`crate::ids::Slug`] it keeps capitals, since host
/// directories are named freely.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CarrySlug(String);

impl CarrySlug {
    pub fn of(path: &Path) -> Self {
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let cleaned: String = name
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-') {
                    c
                } else {
                    '-'
                }
            })
            .collect();
        Self(cleaned.strip_prefix('.').unwrap_or(&cleaned).to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn head_ref(&self) -> String {
        format!("refs/carry/{}/head", self.0)
    }

    pub fn dirty_ref(&self) -> String {
        format!("refs/carry/{}/dirty", self.0)
    }
}

impl std::fmt::Display for CarrySlug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Hands out slugs so two worktrees with the same directory name keep
/// separate refs: the second `clean` becomes `clean-2`, the third `clean-3`.
#[derive(Debug, Default)]
pub struct Names {
    taken: HashSet<String>,
}

impl Names {
    pub fn claim(&mut self, path: &Path) -> CarrySlug {
        let base = CarrySlug::of(path);
        let mut candidate = base.0.clone();
        let mut n = 1;
        while self.taken.contains(&candidate) {
            n += 1;
            candidate = format!("{base}-{n}");
        }
        self.taken.insert(candidate.clone());
        CarrySlug(candidate)
    }
}

/// True for an ignored path carry does not list: anything in a `target/`.
pub fn under_target(path: &str) -> bool {
    path.starts_with("target/") || path.contains("/target/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugs_clean_the_directory_name() {
        assert_eq!(
            CarrySlug::of(Path::new("/w/coda-rust")).as_str(),
            "coda-rust"
        );
        assert_eq!(CarrySlug::of(Path::new("/w/My Tree")).as_str(), "My-Tree");
        assert_eq!(CarrySlug::of(Path::new("/w/.seed")).as_str(), "seed");
        assert_eq!(
            CarrySlug::of(Path::new("/w/x")).dirty_ref(),
            "refs/carry/x/dirty"
        );
    }

    #[test]
    fn duplicate_names_get_numbered() {
        let mut names = Names::default();
        assert_eq!(names.claim(Path::new("/a/clean")).as_str(), "clean");
        assert_eq!(names.claim(Path::new("/b/clean")).as_str(), "clean-2");
        assert_eq!(names.claim(Path::new("/c/clean")).as_str(), "clean-3");
        assert_eq!(names.claim(Path::new("/d/other")).as_str(), "other");
    }

    #[test]
    fn target_paths_are_not_listed() {
        assert!(under_target("target/"));
        assert!(under_target("crates/x/target/debug"));
        assert!(!under_target("notes.log"));
        assert!(!under_target("targets/x"));
    }
}
