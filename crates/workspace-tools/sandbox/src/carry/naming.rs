//! Carry's names for worktrees and their refs.

use std::collections::HashSet;
use std::path::Path;

use crate::ids::{SLUG_MAX_BYTES, Slug};

/// A worktree's name in `refs/carry/<name>/...` and, in the container,
/// its [`Slug`]: its directory's name lowercased, with anything a slug
/// cannot hold turned into `-`, so `carry import` can make every carried
/// worktree under the name `carry` gave it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CarrySlug(Slug);

/// The name of a worktree whose directory name leaves nothing a slug holds.
const FALLBACK: &str = "worktree";

/// Room [`Names::claim`] keeps for a `-<n>` suffix.
const SUFFIX_ROOM: usize = 8;

impl CarrySlug {
    /// The slug for `path` before [`Names`] numbers duplicates.
    fn base(path: &Path) -> String {
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        let mut cleaned = String::with_capacity(name.len());
        for c in name.chars() {
            let c = if c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '.' | '_' | '-')
            {
                c
            } else {
                '-'
            };
            // No `..`, which git refuses in a branch name.
            if !(c == '.' && cleaned.ends_with('.')) {
                cleaned.push(c);
            }
        }
        let mut cleaned = cleaned
            .trim_start_matches(|c: char| !c.is_ascii_alphanumeric())
            .to_owned();
        cleaned.truncate(SLUG_MAX_BYTES - SUFFIX_ROOM);
        while let Some(rest) = cleaned
            .strip_suffix('.')
            .or_else(|| cleaned.strip_suffix(".lock"))
        {
            cleaned = rest.to_owned();
        }
        if cleaned.is_empty() {
            FALLBACK.to_owned()
        } else {
            cleaned
        }
    }

    /// A slug read back from the summary.
    pub fn parse(text: &str) -> Option<Self> {
        text.parse().ok().map(Self)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// The container worktree's slug.
    pub fn slug(&self) -> &Slug {
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
        self.0.fmt(f)
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
        let base = CarrySlug::base(path);
        let mut candidate = base.clone();
        let mut n = 1;
        while self.taken.contains(&candidate) {
            n += 1;
            candidate = format!("{base}-{n}");
        }
        self.taken.insert(candidate.clone());
        CarrySlug(
            candidate
                .parse()
                .expect("a cleaned directory name is a valid slug"),
        )
    }
}

/// True for an ignored path carry does not list: anything in a `target/`.
pub fn under_target(path: &str) -> bool {
    path.starts_with("target/") || path.contains("/target/")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn of(path: &str) -> String {
        Names::default().claim(Path::new(path)).as_str().to_owned()
    }

    #[test]
    fn slugs_clean_the_directory_name() {
        assert_eq!(of("/w/coda-rust"), "coda-rust");
        assert_eq!(of("/w/My Tree"), "my-tree");
        assert_eq!(of("/w/.seed"), "seed");
        assert_eq!(of("/w/_x..y.lock."), "x.y");
        assert_eq!(of("/w/..."), FALLBACK);
        assert_eq!(of(&format!("/w/{}", "a".repeat(300))).len(), 92);
        assert_eq!(
            Names::default().claim(Path::new("/w/x")).dirty_ref(),
            "refs/carry/x/dirty"
        );
    }

    #[test]
    fn parse_takes_only_slugs() {
        assert_eq!(
            CarrySlug::parse("my-tree.2").map(|slug| slug.as_str().to_owned()),
            Some("my-tree.2".to_owned())
        );
        assert_eq!(CarrySlug::parse(""), None);
        assert_eq!(CarrySlug::parse(".seed"), None);
        assert_eq!(CarrySlug::parse("My-Tree"), None);
        assert_eq!(CarrySlug::parse("a/b"), None);
    }

    #[test]
    fn duplicate_names_get_numbered() {
        let mut names = Names::default();
        assert_eq!(names.claim(Path::new("/a/clean")).as_str(), "clean");
        assert_eq!(names.claim(Path::new("/b/Clean")).as_str(), "clean-2");
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
