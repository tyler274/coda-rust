use std::path::{Path, PathBuf};

use super::fake::healthy;
use super::{Check, Fix, Outcome, SpoolDir, Verdict, render, run, verdict};
use crate::config::{Config, Loaded};
use crate::editor_settings::Editor;
use crate::error::Error;
use crate::probe::{
    ActiveState, FsType, LoadState, PoolFeature, SshReach, UnitState, ZFS_BCLONE_PARAMETER,
};

const CONFIG: &str = include_str!("../../config/cyrene.toml");

fn config() -> Result<Loaded, Error> {
    Config::parse_with_unknown_keys(Path::new("cyrene.toml"), CONFIG)
}

fn outcome<'a>(findings: &'a [super::Finding], check: &Check) -> &'a Outcome {
    &findings
        .iter()
        .find(|f| &f.check == check)
        .expect("check ran")
        .outcome
}

#[test]
fn healthy_machine_passes() {
    let findings = run(&healthy(), &config());
    assert_eq!(
        verdict(&findings),
        Verdict::Healthy,
        "{}",
        render(&findings)
    );
    assert!(matches!(
        outcome(&findings, &Check::Reflinks),
        Outcome::Pass { detail } if detail.contains("pool agents feature active")
    ));
    assert!(matches!(
        outcome(&findings, &Check::EditorSettings(Editor::Cursor)),
        Outcome::Pass { .. }
    ));
    assert!(matches!(
        outcome(&findings, &Check::EditorSettings(Editor::VsCode)),
        Outcome::Skip { .. }
    ));
    assert_eq!(Verdict::Healthy.exit_code(), 0);
}

#[test]
fn missing_config_fails_and_says_how_to_pass_one() {
    let missing = Config::load_with_unknown_keys(Path::new("/nonexistent/coda-sandbox.toml"));
    let findings = run(&healthy(), &missing);
    assert_eq!(verdict(&findings), Verdict::Failing);
    let Outcome::Fail { fix, .. } = outcome(&findings, &Check::Config) else {
        panic!("config should fail")
    };
    assert!(
        fix.to_string()
            .contains("--config crates/workspace-tools/sandbox/config/cyrene.toml")
    );
    assert!(
        findings
            .iter()
            .any(|f| f.check == Check::EditorSettings(Editor::Cursor))
    );
}

#[test]
fn unknown_config_keys_warn_and_the_machine_is_still_checked() {
    let newer = CONFIG.replace("[ci]\n", "[ci]\nqueue = \"/x\"\n");
    let findings = run(
        &healthy(),
        &Config::parse_with_unknown_keys(Path::new("t"), &newer),
    );
    assert!(matches!(
        outcome(&findings, &Check::Config),
        Outcome::Warn { detail, .. } if detail.contains("ci.queue") && detail.contains("older")
    ));
    assert!(matches!(
        outcome(&findings, &Check::ContainerUnit),
        Outcome::Pass { .. }
    ));
    assert_eq!(verdict(&findings), Verdict::Healthy);
}

#[test]
fn permission_denied_is_cannot_check_and_makes_the_verdict_incomplete() {
    let mut probe = healthy();
    probe.denied.push(PathBuf::from("/agents/coda/ci/spool"));
    let findings = run(&probe, &config());
    let spool = outcome(&findings, &Check::SpoolDir(SpoolDir::Requests));
    let Outcome::CannotCheck { cause } = spool else {
        panic!("expected CannotCheck, got {spool:?}")
    };
    assert!(
        cause.chain().contains("permission denied"),
        "{}",
        cause.chain()
    );
    assert_eq!(verdict(&findings), Verdict::Incomplete);
    assert_eq!(Verdict::Incomplete.exit_code(), 2);

    probe.ssh = SshReach::TimedOut;
    let findings = run(&probe, &config());
    assert_eq!(verdict(&findings), Verdict::Failing, "a failure outranks");
}

#[test]
fn present_legacy_token_dir_warns_with_a_sudo_ls_fix() {
    let mut probe = healthy();
    probe.dirs.push(PathBuf::from("/var/lib/coda-ci"));
    let findings = run(&probe, &config());
    let check = Check::LegacyToken(PathBuf::from("/var/lib/coda-ci"));
    let Outcome::Warn { fix, detail } = outcome(&findings, &check) else {
        panic!("expected Warn")
    };
    assert_eq!(
        fix,
        &Fix::command("sudo", ["ls", "-la", "/var/lib/coda-ci"])
    );
    assert!(detail.contains("broker"), "{detail}");
    let absent = Check::LegacyToken(PathBuf::from("/var/lib/agent-secrets/coda-agent"));
    assert!(matches!(outcome(&findings, &absent), Outcome::Pass { .. }));
    assert_eq!(verdict(&findings), Verdict::Healthy);
}

#[test]
fn stopped_container_and_zfs_without_bclone_fail() {
    let mut probe = healthy();
    probe.units.insert(
        "container@coda-agent.service".to_owned(),
        UnitState {
            load: LoadState::Loaded,
            active: ActiveState::Inactive,
        },
    );
    probe
        .files
        .insert(PathBuf::from(ZFS_BCLONE_PARAMETER), "0\n".to_owned());
    let findings = run(&probe, &config());
    assert!(matches!(
        outcome(&findings, &Check::ContainerUnit),
        Outcome::Fail { fix, .. } if fix.to_string() == "sudo systemctl start container@coda-agent.service"
    ));
    assert!(matches!(
        outcome(&findings, &Check::Reflinks),
        Outcome::Fail { .. }
    ));
    assert_eq!(verdict(&findings), Verdict::Failing);
}

#[test]
fn reflink_pool_feature_and_xfs() {
    let mut probe = healthy();
    probe.pool_feature = Some(PoolFeature::Disabled);
    let findings = run(&probe, &config());
    assert!(matches!(
        outcome(&findings, &Check::Reflinks),
        Outcome::Fail {
            fix: Fix::Command { .. },
            ..
        }
    ));

    probe.pool_feature = None;
    let findings = run(&probe, &config());
    assert!(matches!(
        outcome(&findings, &Check::Reflinks),
        Outcome::Pass { detail } if detail.contains("pool feature not checked")
    ));

    probe.fs = FsType::Xfs;
    let findings = run(&probe, &config());
    assert!(matches!(
        outcome(&findings, &Check::Reflinks),
        Outcome::Warn { detail, .. } if detail.contains("assumed reflink=1")
    ));
}

#[test]
fn unset_and_enabled_forwarding_both_fail() {
    let path = Editor::Cursor.settings_path(Path::new("/home/me"));
    let mut probe = healthy();
    for text in [
        r#"{ "git.terminalAuthentication": false }"#,
        r#"{ "remote.SSH.enableAgentForwarding": true }"#,
    ] {
        probe.files.insert(path.clone(), text.to_owned());
        let findings = run(&probe, &config());
        assert!(
            matches!(
                outcome(&findings, &Check::EditorSettings(Editor::Cursor)),
                Outcome::Fail { .. }
            ),
            "{text}"
        );
        assert_eq!(verdict(&findings), Verdict::Failing);
    }
}

#[test]
fn failed_and_timed_out_ssh_fail() {
    let mut probe = healthy();
    for reach in [
        SshReach::Failed {
            exit_code: Some(255),
            stderr: "Connection refused".to_owned(),
        },
        SshReach::TimedOut,
    ] {
        probe.ssh = reach;
        let findings = run(&probe, &config());
        assert!(matches!(
            outcome(&findings, &Check::Ssh),
            Outcome::Fail { .. }
        ));
    }
}
