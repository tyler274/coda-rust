//! `coda-sandbox carry` against a scratch repository with three worktrees,
//! and `carry import` from a host clone into a container clone of one
//! origin.

mod common;

use std::path::{Path, PathBuf};

use common::{Scratch, read, write};

/// A bare origin, a host clone and a container clone of it.
struct Pair {
    s: Scratch,
    host: PathBuf,
    container: PathBuf,
    host_config: PathBuf,
    container_config: PathBuf,
}

impl Pair {
    fn new() -> Self {
        let s = Scratch::new();
        let seed = s.repo("seed");
        let origin = s.path().join("origin.git");
        let origin_arg = origin.to_str().unwrap();
        s.git(
            s.path(),
            &["clone", "-q", "--bare", seed.to_str().unwrap(), origin_arg],
        );
        let host = s.path().join("host/coda-rust");
        let container = s.path().join("container/coda-rust");
        s.git(
            s.path(),
            &["clone", "-q", origin_arg, host.to_str().unwrap()],
        );
        s.git(
            s.path(),
            &["clone", "-q", origin_arg, container.to_str().unwrap()],
        );
        let build = s.path().join("build");
        std::fs::create_dir(&build).unwrap();
        let spool = s.path().join("spool");
        let host_config = s.config_at("host.toml", &build, &host, &host, &spool);
        let container_config = s.config_at("container.toml", &build, &container, &host, &spool);
        Self {
            s,
            host,
            container,
            host_config,
            container_config,
        }
    }

    /// A host worktree at `<scratch>/host/<dir>` on a new `branch`.
    fn worktree(&self, dir: &str, branch: &str) -> PathBuf {
        let path = self.s.path().join("host").join(dir);
        self.s.git(
            &self.host,
            &[
                "worktree",
                "add",
                "-q",
                path.to_str().unwrap(),
                "-b",
                branch,
            ],
        );
        path
    }

    fn commit(&self, tree: &Path, file: &str, text: &str) {
        write(&tree.join(file), text);
        self.s.git(tree, &["add", "-A"]);
        self.s.git(tree, &["commit", "-qm", file]);
    }
}

#[test]
fn carry_takes_only_the_named_worktrees_and_records_their_branches() {
    let p = Pair::new();
    let feature = p.worktree("feature", "tyler/feature");
    p.commit(&feature, "f.txt", "f\n");
    p.worktree("other", "other");

    let printed = p.s.ok(&p.host_config, p.s.path(), &["carry", "feature"]);
    assert!(printed.contains("1 worktrees"), "{printed}");
    let summary = read(&p.s.path().join("carry-out/summary.txt"));
    let fields: Vec<&str> = summary.trim_end().split('\t').collect();
    assert_eq!(fields[0], "feature", "{summary}");
    assert_eq!(fields[2], "dirty=no", "{summary}");
    assert_eq!(fields[3], "branch=tyler/feature", "{summary}");
    assert_eq!(fields[4], "checkout=linked", "{summary}");
    let bundle = p.s.path().join("carry-out/carry.bundle");
    let heads = p.s.git(
        p.s.path(),
        &["bundle", "list-heads", bundle.to_str().unwrap()],
    );
    assert!(heads.ends_with(" refs/carry/feature/head"), "{heads}");

    // A name no worktree has stops carry before it replaces anything.
    let refused = p.s.sandbox(&p.host_config, p.s.path(), &["carry", "nope"]);
    assert!(!refused.status.success());
    assert!(
        String::from_utf8_lossy(&refused.stderr).contains("no worktree nope"),
        "{}",
        String::from_utf8_lossy(&refused.stderr)
    );
    assert_eq!(read(&p.s.path().join("carry-out/summary.txt")), summary);

    // Worktrees whose commits origin/main already has need no bundle.
    let printed = p.s.ok(&p.host_config, p.s.path(), &["carry", "other"]);
    assert!(printed.contains("1 worktrees"), "{printed}");
    assert!(!bundle.exists(), "a bundle of nothing new was kept");
}

#[test]
fn import_makes_each_carried_worktree_on_its_branch_with_its_changes() {
    let p = Pair::new();
    // On origin, with a further local commit and uncommitted work.
    let pushed = p.worktree("pushed", "tyler/pushed");
    p.commit(&pushed, "p.txt", "one\n");
    p.s.git(&pushed, &["push", "-q", "origin", "tyler/pushed"]);
    p.commit(&pushed, "p.txt", "two\n");
    write(&pushed.join("a.txt"), "changed\n");
    write(&pushed.join("new.txt"), "untracked\n");
    write(&pushed.join("staged.txt"), "staged\n");
    p.s.git(&pushed, &["add", "staged.txt"]);
    // Never pushed.
    let local = p.worktree("local", "local");
    p.commit(&local, "l.txt", "l\n");
    // Only what origin/main has, so the bundle holds none of its commits.
    p.worktree("even", "even");
    let head = p.s.git(&pushed, &["rev-parse", "HEAD"]);

    p.s.ok(
        &p.host_config,
        p.s.path(),
        &["carry", "pushed", "local", "even"],
    );
    let printed =
        p.s.ok(&p.container_config, &p.container, &["carry", "import"]);
    assert_eq!(printed.lines().count(), 3, "{printed}");

    let tree = p.container.join(".worktrees/pushed");
    let git = |dir: &Path, args: &[&str]| p.s.git(dir, args);
    assert_eq!(
        git(&tree, &["symbolic-ref", "--short", "HEAD"]),
        "tyler/pushed"
    );
    assert_eq!(git(&tree, &["rev-parse", "HEAD"]), head);
    assert_eq!(read(&tree.join("p.txt")), "two\n");
    assert_eq!(read(&tree.join("a.txt")), "changed\n");
    assert_eq!(read(&tree.join("new.txt")), "untracked\n");
    assert_eq!(read(&tree.join("staged.txt")), "staged\n");
    assert_eq!(
        git(&tree, &["rev-parse", "--abbrev-ref", "@{upstream}"]),
        "origin/tyler/pushed"
    );
    assert!(tree.join("target").is_symlink());

    let tree = p.container.join(".worktrees/local");
    assert_eq!(git(&tree, &["symbolic-ref", "--short", "HEAD"]), "local");
    assert_eq!(read(&tree.join("l.txt")), "l\n");
    assert_eq!(git(&tree, &["status", "--porcelain"]), "");
    assert!(printed.contains("local\t"), "{printed}");
    assert!(printed.contains("no upstream yet"), "{printed}");

    let tree = p.container.join(".worktrees/even");
    assert_eq!(git(&tree, &["symbolic-ref", "--short", "HEAD"]), "even");

    // A rerun leaves made worktrees alone, uncommitted work included.
    write(&p.container.join(".worktrees/pushed/later.txt"), "mine\n");
    let again = p.s.ok(
        &p.container_config,
        &p.container,
        &["carry", "import", "pushed"],
    );
    assert!(again.contains("already there"), "{again}");
    assert_eq!(
        read(&p.container.join(".worktrees/pushed/later.txt")),
        "mine\n"
    );

    // A name the summary lacks is refused.
    let refused = p.s.sandbox(
        &p.container_config,
        &p.container,
        &["carry", "import", "nope"],
    );
    assert!(!refused.status.success());
}

#[test]
fn a_rerun_finishes_an_import_that_was_interrupted() {
    let p = Pair::new();
    let pushed = p.worktree("Pushed", "pushed");
    p.commit(&pushed, "p.txt", "one\n");
    p.s.git(&pushed, &["push", "-q", "origin", "pushed"]);
    write(&pushed.join("a.txt"), "changed\n");
    p.s.ok(&p.host_config, p.s.path(), &["carry", "pushed"]);
    p.s.ok(&p.container_config, &p.container, &["carry", "import"]);

    // Stop it as if it had died after making the worktree: the files are
    // not restored, the upstream not set and the mark still says started.
    let tree = p.container.join(".worktrees/pushed");
    p.s.git(&tree, &["checkout", "--", "a.txt"]);
    p.s.git(&tree, &["branch", "--unset-upstream"]);
    let mark = p.container.join(".git/coda-carry/pushed");
    let text = read(&mark);
    write(&mark, &text.replace("\tdone\n", "\tstarted\n"));

    let again =
        p.s.ok(&p.container_config, &p.container, &["carry", "import"]);
    assert!(again.contains("uncommitted files restored"), "{again}");
    assert_eq!(read(&tree.join("a.txt")), "changed\n");
    assert_eq!(
        p.s.git(&tree, &["rev-parse", "--abbrev-ref", "@{upstream}"]),
        "origin/pushed"
    );
    assert_eq!(read(&mark), text);
}

#[test]
fn import_leaves_the_host_main_checkout_to_the_container_checkout() {
    let p = Pair::new();
    write(&p.host.join("a.txt"), "main work\n");
    p.worktree("feature", "feature");
    let carried = p.s.ok(&p.host_config, p.s.path(), &["carry"]);
    assert!(carried.contains("checkout=main"), "{carried}");

    let printed =
        p.s.ok(&p.container_config, &p.container, &["carry", "import"]);
    assert!(printed.contains("the host's main checkout"), "{printed}");
    assert!(printed.contains("refs/carry/coda-rust/dirty"), "{printed}");
    assert!(!p.container.join(".worktrees/coda-rust").exists());
    assert!(p.container.join(".worktrees/feature").is_dir());
    assert_eq!(p.s.git(&p.container, &["status", "--porcelain"]), "");
}

#[test]
fn import_refuses_a_branch_that_moved_since_it_was_carried() {
    let p = Pair::new();
    let feature = p.worktree("feature", "feature");
    p.commit(&feature, "f.txt", "f\n");
    p.s.ok(&p.host_config, p.s.path(), &["carry", "feature"]);
    // The container already has its own `feature`, elsewhere.
    p.s.git(&p.container, &["branch", "feature"]);

    let refused =
        p.s.sandbox(&p.container_config, &p.container, &["carry", "import"]);
    assert!(!refused.status.success());
    let stderr = String::from_utf8_lossy(&refused.stderr);
    assert!(stderr.contains("branch feature is at"), "{stderr}");
    assert!(!p.container.join(".worktrees/feature").exists());
}

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
