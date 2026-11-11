//! `coda-sandbox carry` against a scratch repository with three worktrees.

mod common;

use common::{Scratch, read, write};

#[test]
fn carry_records_head_and_dirty_refs_without_touching_worktrees() {
    let s = Scratch::new();
    let repo = s.repo("coda-rust");
    let feature = s.path().join("feature");
    s.git(
        &repo,
        &[
            "worktree",
            "add",
            "-q",
            feature.to_str().unwrap(),
            "-b",
            "feature",
        ],
    );
    write(&feature.join("a.txt"), "two\n");
    write(&feature.join("staged.txt"), "staged\n");
    s.git(&feature, &["add", "staged.txt"]);
    write(&feature.join("untracked.txt"), "new\n");
    write(&feature.join("target/out"), "big\n");
    write(&feature.join("notes.log"), "keep\n");
    let status_before = s.git(&feature, &["status", "--porcelain"]);
    let index_before = s.git(&feature, &["ls-files", "-s"]);
    let clean = s.path().join("clean");
    s.git(
        &repo,
        &[
            "worktree",
            "add",
            "-q",
            clean.to_str().unwrap(),
            "-b",
            "clean",
        ],
    );
    let other = s.path().join("other/clean");
    s.git(
        &repo,
        &[
            "worktree",
            "add",
            "-q",
            other.to_str().unwrap(),
            "-b",
            "clean2",
        ],
    );

    let config = s.config(
        &s.path().join("unused-build"),
        &repo,
        &s.path().join("spool"),
    );
    let out = s.path().join("carry-out");
    let printed = s.ok(&config, s.path(), &["carry"]);
    assert!(printed.contains("4 worktrees"), "{printed}");

    // The worktree is untouched.
    assert_eq!(s.git(&feature, &["status", "--porcelain"]), status_before);
    assert_eq!(s.git(&feature, &["ls-files", "-s"]), index_before);
    assert_eq!(s.git(&feature, &["stash", "list"]), "");

    // Refs: head for each worktree, dirty only for the dirty one.
    let bundle = out.join("carry.bundle");
    let heads = s.git(
        s.path(),
        &["bundle", "list-heads", bundle.to_str().unwrap()],
    );
    let mut names: Vec<&str> = heads
        .lines()
        .filter_map(|line| line.split(' ').nth(1))
        .collect();
    names.sort_unstable();
    assert_eq!(
        names,
        [
            "refs/carry/clean-2/head",
            "refs/carry/clean/head",
            "refs/carry/coda-rust/head",
            "refs/carry/feature/dirty",
            "refs/carry/feature/head",
        ]
    );

    // The bundle restores the dirty snapshot in a fresh clone.
    let restore = s.path().join("restore");
    std::fs::create_dir(&restore).unwrap();
    s.git(&restore, &["init", "-q"]);
    s.git(
        &restore,
        &[
            "fetch",
            "-q",
            bundle.to_str().unwrap(),
            "refs/carry/*:refs/carry/*",
        ],
    );
    s.git(
        &restore,
        &[
            "-c",
            "advice.detachedHead=false",
            "checkout",
            "-q",
            "refs/carry/feature/dirty",
        ],
    );
    assert_eq!(read(&restore.join("a.txt")), "two\n");
    assert_eq!(read(&restore.join("staged.txt")), "staged\n");
    assert_eq!(read(&restore.join("untracked.txt")), "new\n");
    assert!(
        !restore.join("target").exists(),
        "ignored target/ was carried"
    );
    assert_eq!(
        s.git(&restore, &["rev-parse", "refs/carry/feature/dirty^"]),
        s.git(&repo, &["rev-parse", "feature"])
    );

    // Ignored files outside target/ are listed, target/ is not.
    let ignored = read(&out.join("ignored.txt"));
    assert!(
        ignored.lines().any(|line| line == "feature: notes.log"),
        "{ignored}"
    );
    assert!(!ignored.contains("target"), "{ignored}");
    assert_eq!(read(&out.join("summary.txt")).lines().count(), 4);

    // A second run replaces the refs rather than accumulating them.
    s.git(&feature, &["add", "-A"]);
    s.git(&feature, &["commit", "-qm", "two"]);
    s.ok(&config, s.path(), &["carry"]);
    let heads = s.git(
        s.path(),
        &["bundle", "list-heads", bundle.to_str().unwrap()],
    );
    assert!(
        !heads.contains("feature/dirty"),
        "stale dirty ref kept: {heads}"
    );
    // No staging directory is left behind.
    let leftovers: Vec<_> = std::fs::read_dir(&out)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().starts_with(".carry."))
        .collect();
    assert!(leftovers.is_empty());
}
