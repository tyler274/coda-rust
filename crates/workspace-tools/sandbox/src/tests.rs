use std::path::{Path, PathBuf};

use crate::config::{Config, UnitKind, UnitName, Version};
use crate::doctor::Fix;
use crate::editor_settings::{ForwardingKey, Setting, Settings};
use crate::error::Error;
use crate::ids::{IdKind, JobName, RequestId, SLUG_MAX_BYTES, SeedKey, Slug};
use crate::probe::{ActiveState, LoadState, UnitState};

const CYRENE: &str = include_str!("../config/cyrene.toml");

#[test]
fn cyrene_config_parses() {
    let config = Config::parse(Path::new("cyrene.toml"), CYRENE).expect("cyrene.toml parses");
    assert_eq!(
        config.container.unit.as_str(),
        "container@coda-agent.service"
    );
    assert_eq!(config.container.agent_uid, 656_360);
    assert_eq!(
        config.ci.requests(),
        PathBuf::from("/agents/coda/ci/spool/requests")
    );
    assert_eq!(config.version, Version::V1);
    assert_eq!(config.ci.timer.kind(), UnitKind::Timer);
}

#[test]
fn config_keeps_unknown_keys_and_rejects_bad_units_versions_and_missing_sections() {
    let newer = format!(
        "{}[broker]\nsocket = \"/y\"\n",
        CYRENE.replace("[ci]\n", "[ci]\nqueue = \"/x\"\n")
    );
    let loaded = Config::parse_with_unknown_keys(Path::new("t"), &newer).expect("newer config");
    assert_eq!(loaded.unknown_keys, ["broker", "ci.queue"]);
    let typo = CYRENE.replace("ssh_host", "ssh_hots");
    assert!(matches!(
        Config::parse(Path::new("t"), &typo),
        Err(Error::ConfigParse { .. })
    ));
    let future = CYRENE.replace("version = 1", "version = 2");
    let error = Config::parse(Path::new("t"), &future).expect_err("version 2");
    assert!(error.chain().contains("version 2"), "{}", error.chain());
    let bad_unit = CYRENE.replace("coda-ci.timer", "coda-ci");
    let error = Config::parse(Path::new("t"), &bad_unit).expect_err("unit without suffix");
    assert!(
        error.chain().contains("not a systemd unit name"),
        "{}",
        error.chain()
    );
    assert!(matches!(
        Config::parse(Path::new("t"), "[build]\nroot = \"/x\"\n"),
        Err(Error::ConfigParse { .. })
    ));
}

#[test]
fn missing_config_is_distinct_from_unreadable() {
    let error = Config::load(Path::new("/nonexistent/config.toml")).expect_err("missing");
    assert!(matches!(error, Error::ConfigMissing { .. }));
    let error = Config::load(Path::new("/")).expect_err("a directory");
    assert!(matches!(error, Error::ConfigRead { .. }));
}

#[test]
fn unit_names_need_a_type_suffix() {
    assert!("coda-ci.timer".parse::<UnitName>().is_ok());
    assert!("container@coda-agent.service".parse::<UnitName>().is_ok());
    for (name, kind) in [
        ("coda-sandbox-broker.socket", UnitKind::Socket),
        ("x.path", UnitKind::Path),
        ("multi-user.target", UnitKind::Target),
    ] {
        assert_eq!(name.parse::<UnitName>().expect(name).kind(), kind);
    }
    assert!("x.mount".parse::<UnitName>().is_err());
    assert!(".service".parse::<UnitName>().is_err());
    assert!("coda ci.service".parse::<UnitName>().is_err());
}

#[test]
fn slugs() {
    let longest = "a".repeat(SLUG_MAX_BYTES);
    for good in [
        "eng-60-guardrails",
        "a",
        "0.x_y-z",
        "a.b",
        "x.locks",
        &longest,
    ] {
        assert_eq!(good.parse::<Slug>().expect(good).as_str(), good);
    }
    let too_long = "a".repeat(SLUG_MAX_BYTES + 1);
    for bad in [
        "", "-lead", ".hidden", "Upper", "a/b", "a b", "a..b", "foo.", "x.lock", &too_long,
    ] {
        assert_eq!(bad.parse::<Slug>().expect_err(bad).kind, IdKind::Slug);
    }
}

#[test]
fn fix_commands_render_shell_quoted() {
    let fix = Fix::command("sudo", ["ls", "-la", "/var/lib/coda ci", "it's"]);
    assert_eq!(fix.to_string(), r"sudo ls -la '/var/lib/coda ci' 'it'\''s'");
    assert_eq!(Fix::command("true", [""; 1]).to_string(), "true ''");
    assert_eq!(Fix::instruction("rebuild").to_string(), "rebuild");
}

#[test]
fn job_names() {
    assert!("Lint_and-test".parse::<JobName>().is_ok());
    assert!("a".repeat(64).parse::<JobName>().is_ok());
    assert!("a".repeat(65).parse::<JobName>().is_err());
    assert!("".parse::<JobName>().is_err());
    assert!("job.name".parse::<JobName>().is_err());
}

#[test]
fn request_ids_and_seed_keys_are_fixed_length_lowercase_hex() {
    assert!("0123456789ab".parse::<RequestId>().is_ok());
    assert!("0123456789AB".parse::<RequestId>().is_err());
    assert!("0123456789a".parse::<RequestId>().is_err());
    assert!("0123456789abcdef".parse::<SeedKey>().is_ok());
    assert!("0123456789abcdeg".parse::<SeedKey>().is_err());
    assert!("0123456789ab".parse::<SeedKey>().is_err());
}

#[test]
fn settings_accept_comments_and_trailing_commas() {
    let text = r#"// user settings
    {
        /* block */ "remote.SSH.enableAgentForwarding": false, // trailing
        "git.terminalAuthentication": true,
        "github.gitAuthentication": "no",
        "url": "https://example.com/not//a/comment",
    }"#;
    let settings = Settings::parse(Path::new("settings.json"), text).expect("JSONC parses");
    assert_eq!(settings.get(ForwardingKey::SshAgent), Setting::Off);
    assert_eq!(settings.get(ForwardingKey::GitTerminal), Setting::On);
    assert!(matches!(
        settings.get(ForwardingKey::GithubGit),
        Setting::NotBoolean(_)
    ));
}

#[test]
fn settings_report_absent_keys_and_reject_broken_files() {
    let settings = Settings::parse(Path::new("s"), "{}").expect("empty object");
    assert_eq!(settings.get(ForwardingKey::SshAgent), Setting::Absent);
    assert!(matches!(
        Settings::parse(Path::new("s"), "{ \"a\": "),
        Err(Error::SettingsParse { .. })
    ));
    assert!(Settings::parse(Path::new("s"), "[]").is_err());
}

#[test]
fn systemctl_show_output_parses() {
    let unit: UnitName = "coda-ci.timer".parse().expect("unit");
    let state = UnitState::parse(&unit, "LoadState=loaded\nActiveState=active\n").expect("parses");
    assert_eq!(
        state,
        UnitState {
            load: LoadState::Loaded,
            active: ActiveState::Active
        }
    );
    let state =
        UnitState::parse(&unit, "ActiveState=inactive\nLoadState=not-found\n").expect("parses");
    assert_eq!(state.load, LoadState::NotFound);
    assert!(UnitState::parse(&unit, "LoadState=loaded\n").is_err());
    assert!(UnitState::parse(&unit, "LoadState=sideways\nActiveState=active\n").is_err());
}
