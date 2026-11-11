//! Pure logic behind `worktree`, `ci run` and `carry`.

use std::path::{Path, PathBuf};

use crate::ci::request::Spool;
use crate::config::Config;
use crate::ids::{RequestId, SeedKey, Slug};
use crate::worktree::Layout;
use crate::worktree::clone::seed_key_of;
use crate::worktree::new::branch_ref;

const CYRENE: &str = include_str!("../config/cyrene.toml");

#[test]
fn seed_key_is_the_first_16_hex_digits_of_the_concatenated_sha256() {
    // sha256("") and sha256("abc"), as `sha256sum | cut -c1-16` prints them.
    assert_eq!(seed_key_of(&[]).as_str(), "e3b0c44298fc1c14");
    assert_eq!(seed_key_of(&[b"abc"]).as_str(), "ba7816bf8f01cfea");
    // Concatenation, not per-file hashing: the split does not matter.
    assert_eq!(seed_key_of(&[b"a", b"", b"bc"]), seed_key_of(&[b"abc"]));
    assert!(seed_key_of(&[b"x"]).as_str().parse::<SeedKey>().is_ok());
}

#[test]
fn request_ids_from_bytes_are_valid_ids() {
    let id = RequestId::from_bytes([0, 1, 0xab, 0xcd, 0xef, 0xff]);
    assert_eq!(id.as_str(), "0001abcdefff");
    assert_eq!(id.as_str().parse::<RequestId>(), Ok(id));
}

#[test]
fn spool_paths_follow_the_request_and_result_layout() {
    let spool = Spool {
        requests: PathBuf::from("/s/requests"),
        results: PathBuf::from("/s/results"),
    };
    let id: RequestId = "0123456789ab".parse().expect("id");
    assert_eq!(
        spool.bundle(&id),
        Path::new("/s/requests/0123456789ab.bundle")
    );
    assert_eq!(
        spool.job_file(&id),
        Path::new("/s/requests/0123456789ab.job")
    );
    assert_eq!(
        spool.job_part(&id),
        Path::new("/s/requests/.0123456789ab.job.part")
    );
    assert_eq!(spool.log(&id), Path::new("/s/results/0123456789ab/log"));
    assert_eq!(
        spool.status(&id),
        Path::new("/s/results/0123456789ab/status")
    );
}

#[test]
fn cyrene_config_reads_the_port_settings() {
    let config = Config::parse(Path::new("cyrene.toml"), CYRENE).expect("config");
    assert_eq!(config.build.keep_seeds.get(), 2);
    assert_eq!(config.ci.timeout_seconds, 14_400);
    let carry = config.carry.expect("Cyrene carries its worktrees");
    assert_eq!(carry.repo, Path::new("/home/luluco/code/coda/coda-rust"));
    assert_eq!(carry.out, Path::new("/var/lib/coda-carry"));

    let without_carry = CYRENE.split("[carry]").next().expect("text before [carry]");
    let config = Config::parse(Path::new("t"), without_carry).expect("no carry");
    assert!(config.carry.is_none());
}

#[test]
fn defaults_fill_keep_seeds_and_timeout() {
    let trimmed = CYRENE
        .replace("keep_seeds = 2\n", "")
        .replace("timeout_seconds = 14400\n", "");
    let config = Config::parse(Path::new("t"), &trimmed).expect("defaults");
    assert_eq!(config.build.keep_seeds.get(), 2);
    assert_eq!(config.ci.timeout_seconds, 4 * 3600);

    let none_kept = CYRENE.replace("keep_seeds = 2", "keep_seeds = 0");
    assert!(Config::parse(Path::new("t"), &none_kept).is_err());
}

#[test]
fn layout_names_trees_targets_and_branches() {
    let config = Config::parse(Path::new("cyrene.toml"), CYRENE).expect("config");
    let layout = Layout::new(PathBuf::from("/r"), &config.build);
    let slug: Slug = "eng-60".parse().expect("slug");
    assert_eq!(layout.tree(&slug), Path::new("/r/.worktrees/eng-60"));
    assert_eq!(
        layout.target(&slug),
        Path::new("/agents/coda/build/wt/eng-60")
    );
    assert_eq!(layout.seeds, Path::new("/agents/coda/build/seed"));
    assert_eq!(layout.seed_tree(), Path::new("/r/.worktrees/.seed"));
    assert_eq!(branch_ref(&slug), "refs/heads/eng-60");
}
