//! `coda-sandbox worktree` against a scratch repository.
//!
//! Cloning a seed needs a filesystem with reflinks, which tmpfs lacks, so the
//! seeded test takes its build root from `CODA_SANDBOX_TEST_BUILD_ROOT` (a
//! directory on ZFS 2.2+, btrfs or XFS) and skips, saying so, when it is
//! unset. The unseeded test runs anywhere.

mod common;

use std::path::{Path, PathBuf};

use common::{Scratch, read, write};

const BUILD_ROOT_VAR: &str = "CODA_SANDBOX_TEST_BUILD_ROOT";

fn link(path: &Path) -> PathBuf {
    std::fs::read_link(path).unwrap_or_else(|e| panic!("read link {}: {e}", path.display()))
}

#[test]
fn new_without_a_seed_makes_an_empty_target_and_rm_removes_it() {
    let s = Scratch::new();
    let repo = s.repo("coda-rust");
    let build = s.path().join("build");
    std::fs::create_dir(&build).unwrap();
    let config = s.config(&build, &repo, &s.path().join("spool"));

    let printed = s.ok(
        &config,
        &repo,
        &["worktree", "new", "eng-1", "--from", "main"],
    );
    let tree = repo.join(".worktrees/eng-1");
    assert_eq!(printed.trim(), tree.display().to_string());
    assert_eq!(link(&tree.join("target")), build.join("wt/eng-1"));
    assert!(build.join("wt/eng-1").is_dir());
    assert_eq!(s.git(&tree, &["symbolic-ref", "--short", "HEAD"]), "eng-1");

    // A rerun finishes rather than failing, and so does one after the
    // target was lost mid-way.
    std::fs::remove_dir(build.join("wt/eng-1")).unwrap();
    s.ok(
        &config,
        &repo,
        &["worktree", "new", "eng-1", "--from", "main"],
    );
    assert!(build.join("wt/eng-1").is_dir());

    // From inside the worktree, the layout is still the main checkout's.
    let status = s.ok(&config, &tree, &["worktree", "status"]);
    assert!(status.contains("Seed: none"), "{status}");
    assert!(
        status.lines().any(|line| line.starts_with("eng-1 ")),
        "{status}"
    );

    // A real target/ directory stops new rather than being replaced.
    let other = s.ok(
        &config,
        &repo,
        &["worktree", "new", "eng-2", "--from", "main"],
    );
    let other = PathBuf::from(other.trim());
    std::fs::remove_file(other.join("target")).unwrap();
    std::fs::create_dir(other.join("target")).unwrap();
    let refused = s.sandbox(&config, &repo, &["worktree", "reset", "eng-2"]);
    assert!(!refused.status.success());
    assert!(String::from_utf8_lossy(&refused.stderr).contains("is a real directory"));
    std::fs::remove_dir(other.join("target")).unwrap();

    s.ok(&config, &repo, &["worktree", "rm", "eng-1"]);
    assert!(!tree.exists());
    assert!(!build.join("wt/eng-1").exists());
    // The branch is kept. With nothing left, a second rm says so.
    s.git(&repo, &["rev-parse", "--verify", "refs/heads/eng-1"]);
    let again = s.sandbox(&config, &repo, &["worktree", "rm", "eng-1"]);
    assert!(!again.status.success());
    assert!(String::from_utf8_lossy(&again.stderr).contains("no worktree"));

    // A target whose worktree a crash already removed is finished off.
    std::fs::create_dir_all(build.join("wt/eng-1")).unwrap();
    s.ok(&config, &repo, &["worktree", "rm", "eng-1"]);
    assert!(!build.join("wt/eng-1").exists());

    // gc drops a target whose worktree is gone.
    std::fs::create_dir_all(build.join("wt/orphan")).unwrap();
    s.ok(&config, &repo, &["worktree", "gc"]);
    assert!(!build.join("wt/orphan").exists());
}

/// The reviewer's scratch-repository cases: the build root holds the
/// configured checkout's targets, proof included, so no command run from
/// another repository may delete them, and `rm` of a directory git does not
/// know as a worktree deletes nothing.
#[test]
fn deleting_commands_refuse_other_repositories_and_unknown_worktrees() {
    let s = Scratch::new();
    let repo = s.repo("coda-rust");
    let other = s.repo("other");
    let build = s.path().join("build");
    std::fs::create_dir(&build).unwrap();
    let config = s.config(&build, &repo, &s.path().join("spool"));
    s.ok(
        &config,
        &repo,
        &["worktree", "new", "foo", "--from", "main"],
    );
    write(&build.join("wt/foo/verification/proof.txt"), "proof\n");
    let proof = build.join("wt/foo/verification/proof.txt");

    for args in [&["worktree", "rm", "foo"][..], &["worktree", "gc"]] {
        let refused = s.sandbox(&config, &other, args);
        assert!(!refused.status.success(), "{args:?} from another repo");
        let stderr = String::from_utf8_lossy(&refused.stderr);
        assert!(stderr.contains("is not the checkout"), "{stderr}");
        assert_eq!(read(&proof), "proof\n", "{args:?} deleted the target");
    }
    assert!(repo.join(".worktrees/foo/target").is_dir());

    // A submodule is another repository too, though it sits inside this one.
    let sub = repo.join(".worktrees/foo/sub");
    s.git(&repo.join(".worktrees/foo"), &["init", "-q", "sub"]);
    let refused = s.sandbox(&config, &sub, &["worktree", "gc"]);
    assert!(!refused.status.success());
    assert_eq!(read(&proof), "proof\n");

    // Outside any repository, as a system unit runs, the configured checkout
    // is used, and from one of its worktrees the layout is the same.
    s.ok(&config, s.path(), &["worktree", "gc"]);
    s.ok(&config, &repo.join(".worktrees/foo"), &["worktree", "gc"]);
    assert_eq!(read(&proof), "proof\n");

    // A configured checkout that is not a repository orphans nothing.
    let missing = s.config(&build, &s.path().join("absent"), &s.path().join("spool"));
    let refused = s.sandbox(&missing, s.path(), &["worktree", "gc"]);
    assert!(!refused.status.success());
    assert_eq!(read(&proof), "proof\n");
    let config = s.config(&build, &repo, &s.path().join("spool"));

    std::fs::create_dir_all(repo.join(".worktrees/plain")).unwrap();
    std::fs::create_dir_all(build.join("wt/plain")).unwrap();
    let unknown = s.sandbox(&config, &repo, &["worktree", "rm", "plain"]);
    assert!(!unknown.status.success());
    let stderr = String::from_utf8_lossy(&unknown.stderr);
    assert!(stderr.contains("is not a worktree git knows"), "{stderr}");
    assert!(!String::from_utf8_lossy(&unknown.stdout).contains("Removed"));
    assert!(build.join("wt/plain").is_dir());
    assert!(repo.join(".worktrees/plain").is_dir());
}

#[test]
fn bad_slugs_and_missing_build_roots_are_refused() {
    let s = Scratch::new();
    let repo = s.repo("coda-rust");
    let config = s.config(&s.path().join("absent"), &repo, &s.path().join("spool"));
    let bad = s.sandbox(&config, &repo, &["worktree", "new", "Bad/Slug"]);
    assert!(!bad.status.success());
    let missing = s.sandbox(&config, &repo, &["worktree", "new", "ok", "--from", "main"]);
    assert!(!missing.status.success());
    assert!(String::from_utf8_lossy(&missing.stderr).contains("does not exist"));
}

#[test]
fn seeded_clone_reset_and_gc_on_a_reflink_build_root() {
    let Some(root) = std::env::var_os(BUILD_ROOT_VAR) else {
        eprintln!("skipped: set {BUILD_ROOT_VAR} to a directory on a reflink filesystem");
        return;
    };
    let s = Scratch::new();
    let repo = s.repo("coda-rust");
    let build = tempfile::tempdir_in(root).expect("build root under the reflink dir");
    let build = build.path();
    let config = s.config(build, &repo, &s.path().join("spool"));

    // A seed as `worktree seed` leaves it, keyed by the three lock files.
    let key = "0123456789abcdef";
    write(&build.join(format!("seed/{key}/debug/dep.rlib")), "built\n");
    write(
        &build.join(format!("seed/{key}/.coda-seed")),
        &format!("{key}\n"),
    );
    std::os::unix::fs::symlink(key, build.join("seed/current")).unwrap();

    s.ok(
        &config,
        &repo,
        &["worktree", "new", "eng-3", "--from", "main"],
    );
    let target = build.join("wt/eng-3");
    assert_eq!(read(&target.join("debug/dep.rlib")), "built\n");
    assert_eq!(read(&target.join(".coda-seed")).trim(), key);

    // reset keeps retained proof and drops build output.
    write(&target.join("verification/proof.txt"), "proof\n");
    write(&target.join("debug/new.rlib"), "diverged\n");
    s.ok(&config, &repo, &["worktree", "reset", "eng-3"]);
    assert_eq!(read(&target.join("verification/proof.txt")), "proof\n");
    assert!(!target.join("debug/new.rlib").exists());

    // Proof an interrupted reset left aside is restored by the next one.
    let aside = build.join("wt/.verification.eng-3/verification");
    std::fs::create_dir_all(aside.parent().unwrap()).unwrap();
    std::fs::rename(target.join("verification"), &aside).unwrap();
    s.ok(&config, &repo, &["worktree", "reset", "eng-3"]);
    assert_eq!(read(&target.join("verification/proof.txt")), "proof\n");
    assert!(!build.join("wt/.verification.eng-3").exists());

    // gc --idle 0 resets a target built since its clone, keeping proof. File
    // times come from the kernel's coarse clock, so let it tick past the
    // marker the reset wrote first.
    std::thread::sleep(std::time::Duration::from_millis(50));
    write(&target.join("debug/later.rlib"), "diverged\n");
    s.ok(&config, &repo, &["worktree", "gc", "--idle", "0"]);
    assert!(!target.join("debug/later.rlib").exists());
    assert_eq!(read(&target.join("verification/proof.txt")), "proof\n");

    let status = s.ok(&config, &repo, &["worktree", "status"]);
    assert!(status.contains(&format!("Seed: {key}")), "{status}");
    s.ok(&config, &repo, &["worktree", "rm", "eng-3"]);
    assert!(!target.exists());
}
