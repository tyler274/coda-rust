use super::credentials::Secret;
use super::*;
use std::{
    collections::{BTreeMap, VecDeque},
    fs,
    os::unix::fs::PermissionsExt,
};

const PASSWORDS: [(&str, &str); 4] = [
    ("stg_physician_runtime", "pr-synthetic-secret"),
    ("stg_physician_migration", "pm-synthetic-secret"),
    ("stg_duet_runtime", "dr-synthetic-secret"),
    ("stg_duet_migration", "dm-synthetic-secret"),
];

struct Sandbox {
    base: PathBuf,
}
impl Sandbox {
    fn new(name: &str) -> Self {
        let base = std::env::temp_dir().join(format!(
            "staging-reconcile-{name}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = fs::remove_dir_all(&base);
        let root = base.join("ws").join(ROOT);
        fs::create_dir_all(root.join("backend")).unwrap();
        fs::create_dir_all(root.join("env")).unwrap();
        fs::write(root.join("main.tf"), "# synthetic\n").unwrap();
        fs::write(root.join(".terraform.lock.hcl"), "# lock\n").unwrap();
        fs::write(
            root.join(artifacts::BACKEND_FILE),
            "bucket = \"coda-rust-staging-tofu-state-us-east1\"\nprefix = \"rust-environments/stg\"\n",
        )
        .unwrap();
        fs::write(root.join(artifacts::VAR_FILE), "environment = \"stg\"\n").unwrap();
        Self { base }
    }
    fn workspace(&self) -> PathBuf {
        self.base.join("ws")
    }
    fn root(&self) -> PathBuf {
        self.workspace().join(ROOT)
    }
    fn out(&self) -> PathBuf {
        self.base.join("plan")
    }
}
impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.base);
    }
}

fn planned(min: u64, cpu_idle: bool, egress: &str) -> String {
    serde_json::json!({"planned_values": {"root_module": {"resources": [
        {"address": "google_cloud_run_v2_service.app[\"duet\"]", "values": {}},
        {"address": "google_cloud_run_v2_service.app[\"physician\"]", "values": {"template": [{
            "scaling": [{"min_instance_count": min, "max_instance_count": 2}],
            "vpc_access": [{"egress": egress}],
            "containers": [{"resources": [{"cpu_idle": cpu_idle}]}]
        }]}}
    ]}}})
    .to_string()
}

fn service(
    annotations: serde_json::Value,
    ready: &str,
    created: &str,
    traffic: serde_json::Value,
) -> String {
    serde_json::json!({
        "spec": {"template": {"metadata": {"annotations": annotations}}},
        "status": {
            "conditions": [{"type": "Ready", "status": "True"}],
            "latestCreatedRevisionName": created,
            "latestReadyRevisionName": ready,
            "traffic": traffic
        }
    })
    .to_string()
}

fn healthy() -> String {
    service(
        serde_json::json!({
            "autoscaling.knative.dev/minScale": "1",
            "autoscaling.knative.dev/maxScale": "2",
            "run.googleapis.com/cpu-throttling": "false",
            "run.googleapis.com/vpc-access-egress": "all-traffic"
        }),
        "physician-00190",
        "physician-00190",
        serde_json::json!([{"revisionName": "physician-00190", "percent": 100, "latestRevision": true}]),
    )
}

fn run_of(exit: Exit, stdout: &str, stderr: &str) -> Run {
    Run {
        exit,
        stdout: stdout.as_bytes().to_vec(),
        stderr: stderr.as_bytes().to_vec(),
        elapsed: Duration::from_millis(5),
    }
}

struct Fake {
    calls: Vec<(Invocation, Option<String>)>,
    doppler: BTreeMap<&'static str, Run>,
    plan_exit: Exit,
    drift_exit: Exit,
    apply_exit: Exit,
    show_json: String,
    services: VecDeque<String>,
    pauses: u32,
}
impl Default for Fake {
    fn default() -> Self {
        Self {
            calls: vec![],
            doppler: PASSWORDS
                .iter()
                .map(|(config, value)| (*config, run_of(Exit::Success, &format!("{value}\n"), "")))
                .collect(),
            plan_exit: Exit::Code(Some(2)),
            drift_exit: Exit::Success,
            apply_exit: Exit::Success,
            show_json: planned(1, false, "ALL_TRAFFIC"),
            services: VecDeque::from([healthy()]),
            pauses: 0,
        }
    }
}
impl Fake {
    fn stages(&self) -> Vec<Stage> {
        self.calls.iter().map(|(call, _)| call.stage).collect()
    }
}
impl Runner for Fake {
    fn run(&mut self, invocation: &Invocation, secret: Option<&Secret>) -> Result<Run> {
        self.calls.push((
            invocation.clone(),
            secret.map(|secret| secret.expose().to_owned()),
        ));
        let arg = |index: usize| invocation.args.get(index).map(String::as_str);
        Ok(match (invocation.program, invocation.stage) {
            (Program::Doppler, _) => {
                let config = invocation.args.last().unwrap().as_str();
                let run = &self.doppler[config];
                run_of(
                    run.exit,
                    std::str::from_utf8(&run.stdout).unwrap(),
                    std::str::from_utf8(&run.stderr).unwrap(),
                )
            }
            (Program::Tofu, Stage::Init) => run_of(Exit::Success, "init ok", ""),
            (Program::Tofu, Stage::Plan) => {
                let out = invocation
                    .args
                    .iter()
                    .find_map(|arg| arg.strip_prefix("-out="))
                    .unwrap();
                fs::write(out, "synthetic-plan-bytes").unwrap();
                fs::set_permissions(out, fs::Permissions::from_mode(0o600)).unwrap();
                run_of(self.plan_exit, "plan text", "plan stderr")
            }
            (Program::Tofu, Stage::Show) if arg(2) == Some("-json") => {
                run_of(Exit::Success, &self.show_json, "")
            }
            (Program::Tofu, Stage::Show) => run_of(Exit::Success, "~ physician min 0 -> 1", ""),
            (Program::Tofu, Stage::Apply) => run_of(self.apply_exit, "apply text", "apply stderr"),
            (Program::Tofu, Stage::DriftPlan) => run_of(self.drift_exit, "drift text", ""),
            (Program::Gcloud, Stage::Readback) => {
                let next = if self.services.len() > 1 {
                    self.services.pop_front().unwrap()
                } else {
                    self.services.front().cloned().unwrap()
                };
                run_of(Exit::Success, &next, "")
            }
            other => panic!("unexpected call {other:?}"),
        })
    }
    fn pause(&mut self, _duration: Duration) {
        self.pauses += 1;
    }
}

fn plan_in(sandbox: &Sandbox, fake: &mut Fake) -> Result<()> {
    run(
        fake,
        &sandbox.workspace(),
        Request::Plan {
            output: sandbox.out(),
        },
    )
}
fn apply_in(sandbox: &Sandbox, fake: &mut Fake) -> Result<()> {
    run(
        fake,
        &sandbox.workspace(),
        Request::Apply {
            plan_dir: sandbox.out(),
        },
    )
}
fn verify_in(sandbox: &Sandbox, fake: &mut Fake) -> Result<()> {
    run(
        fake,
        &sandbox.workspace(),
        Request::Verify {
            plan_dir: sandbox.out(),
        },
    )
}
fn manifest(sandbox: &Sandbox) -> Manifest {
    PlanDir::open(&sandbox.out())
        .unwrap()
        .load_manifest()
        .unwrap()
}
fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[test]
fn cli_requires_explicit_apply_authorization_and_rejects_unknown_flags() {
    assert!(matches!(
        Request::parse(&args(&["apply", "--plan-dir", "/p"])),
        Err(Error::Refused(message)) if message.contains("--authorize-staging-apply")
    ));
    assert_eq!(
        Request::parse(&args(&[
            "apply",
            "--plan-dir",
            "/p",
            "--authorize-staging-apply"
        ]))
        .unwrap(),
        Request::Apply {
            plan_dir: "/p".into()
        }
    );
    for bad in [
        &["plan", "--output-dir", "/a", "--output-dir", "/b"][..],
        &["plan", "--output-dir", "/a", "--authorize-staging-apply"],
        &["verify", "--plan-dir", "/a", "--authorize-staging-apply"],
        &["plan", "--plan-dir", "/a"],
        &["apply", "--plan-dir"],
        &["destroy"],
        &[],
    ] {
        assert!(Request::parse(&args(bad)).is_err(), "{bad:?}");
    }
}

#[test]
fn credentials_map_each_role_into_one_child_environment_value() {
    let sandbox = Sandbox::new("mapping");
    let mut fake = Fake::default();
    plan_in(&sandbox, &mut fake).unwrap();
    let configs: Vec<_> = fake
        .calls
        .iter()
        .filter(|(call, _)| call.program == Program::Doppler)
        .map(|(call, _)| call.args.clone())
        .collect();
    assert_eq!(configs.len(), 4);
    for (role, args) in Role::ALL.iter().zip(&configs) {
        assert_eq!(
            args,
            &[
                "secrets",
                "get",
                "DB_PASSWORD",
                "--plain",
                "--project",
                "coda-rust",
                "--config",
                role.doppler_config()
            ]
        );
    }
    let secrets: Vec<_> = fake
        .calls
        .iter()
        .filter_map(|(call, secret)| secret.as_ref().map(|secret| (call.stage, secret)))
        .collect();
    assert_eq!(secrets.len(), 1, "only tofu plan receives the passwords");
    assert_eq!(secrets[0].0, Stage::Plan);
    let value: BTreeMap<String, String> = serde_json::from_str(secrets[0].1).unwrap();
    assert_eq!(
        value,
        BTreeMap::from([
            ("physician-runtime".into(), "pr-synthetic-secret".into()),
            ("physician-migration".into(), "pm-synthetic-secret".into()),
            ("duet-runtime".into(), "dr-synthetic-secret".into()),
            ("duet-migration".into(), "dm-synthetic-secret".into()),
        ])
    );
    for (call, _) in &fake.calls {
        assert!(!call.args.iter().any(|arg| arg.contains("synthetic-secret")));
    }
    assert!(std::env::var_os(tools::PASSWORDS_ENV).is_none());
    assert_eq!(
        format!("{:?}", Secret::for_test("pr-synthetic-secret")),
        "Secret(<redacted>)"
    );
}

#[test]
fn failed_or_empty_lookup_stops_before_any_opentofu_command_without_leaking() {
    for (failing, run) in [
        (
            "stg_duet_migration",
            run_of(
                Exit::Code(Some(1)),
                "leaked-stdout-secret",
                "Doppler Error: unauthorized",
            ),
        ),
        ("stg_physician_runtime", run_of(Exit::Success, " \n", "")),
        ("stg_duet_runtime", run_of(Exit::Timeout, "", "")),
    ] {
        let sandbox = Sandbox::new("lookup");
        let mut fake = Fake::default();
        fake.doppler.insert(failing, run);
        let error = plan_in(&sandbox, &mut fake).unwrap_err();
        let message = error.to_string();
        assert!(matches!(error, Error::Credential { .. }), "{message}");
        assert!(message.contains(failing) && message.contains("No OpenTofu command ran"));
        assert!(
            fake.calls
                .iter()
                .all(|(call, _)| call.program == Program::Doppler)
        );
        assert!(!message.contains("secret") && !format!("{error:?}").contains("synthetic"));
        let log =
            fs::read_to_string(sandbox.out().join(artifacts::DIAGNOSTICS_FILE)).unwrap_or_default();
        assert!(!log.contains("leaked-stdout-secret") && !log.contains("synthetic"));
        if failing == "stg_duet_migration" {
            assert!(
                log.contains("unauthorized"),
                "stderr retained for diagnosis"
            );
        }
    }
}

#[test]
fn plan_saves_private_review_and_identity_and_reports_outcome() {
    let sandbox = Sandbox::new("plan");
    let mut fake = Fake::default();
    plan_in(&sandbox, &mut fake).unwrap();
    assert_eq!(
        fake.stages(),
        [
            Stage::Credentials,
            Stage::Credentials,
            Stage::Credentials,
            Stage::Credentials,
            Stage::Init,
            Stage::Plan,
            Stage::Show
        ]
    );
    let plan_args = &fake.calls[5].0.args;
    assert!(plan_args.contains(&"-detailed-exitcode".to_owned()));
    assert!(plan_args.contains(&"-var-file=env/staging.tfvars".to_owned()));
    assert!(
        fake.calls[4]
            .0
            .args
            .contains(&"-backend-config=backend/staging.gcs.tfbackend".to_owned())
    );
    let mode = |name: &str| {
        fs::metadata(sandbox.out().join(name))
            .unwrap()
            .permissions()
            .mode()
            & 0o777
    };
    assert_eq!(mode(""), 0o700);
    for file in [PLAN_FILE, REVIEW_FILE, artifacts::MANIFEST_FILE] {
        assert_eq!(mode(file), 0o600, "{file}");
    }
    let saved = manifest(&sandbox);
    assert_eq!(saved.outcome, PlanOutcome::Changes);
    assert_eq!(saved.plan_digest, Sha256Hex::of(b"synthetic-plan-bytes"));
    assert_eq!(saved.apply, None);
    let encoded = fs::read_to_string(sandbox.out().join(artifacts::MANIFEST_FILE)).unwrap();
    assert!(!encoded.contains("synthetic-secret") && !encoded.contains("plan text"));

    let again = plan_in(&sandbox, &mut Fake::default()).unwrap_err();
    assert!(matches!(again, Error::Refused(message) if message.contains("never overwrite")));

    let quiet = Sandbox::new("no-changes");
    let mut fake = Fake {
        plan_exit: Exit::Success,
        ..Fake::default()
    };
    plan_in(&quiet, &mut fake).unwrap();
    assert_eq!(manifest(&quiet).outcome, PlanOutcome::NoChanges);
    let refused = apply_in(&quiet, &mut Fake::default()).unwrap_err();
    assert!(matches!(refused, Error::Refused(message) if message.contains("no changes")));
}

#[test]
fn plan_failure_retains_private_diagnostics_and_writes_no_manifest() {
    let sandbox = Sandbox::new("plan-fail");
    let mut fake = Fake {
        plan_exit: Exit::Code(Some(1)),
        ..Fake::default()
    };
    let error = plan_in(&sandbox, &mut fake).unwrap_err();
    assert!(matches!(
        error,
        Error::Stage {
            stage: Stage::Plan,
            exit: Exit::Code(Some(1)),
            ..
        }
    ));
    assert!(error.to_string().contains(artifacts::DIAGNOSTICS_FILE));
    let log = fs::read_to_string(sandbox.out().join(artifacts::DIAGNOSTICS_FILE)).unwrap();
    assert!(log.starts_with("SENSITIVE") && log.contains("plan stderr"));
    assert!(!sandbox.out().join(artifacts::MANIFEST_FILE).exists());
}

#[test]
fn unsafe_output_paths_and_backends_are_refused() {
    let sandbox = Sandbox::new("paths");
    let inside = run(
        &mut Fake::default(),
        &sandbox.workspace(),
        Request::Plan {
            output: sandbox.workspace().join("plan"),
        },
    );
    assert!(
        matches!(inside, Err(Error::Refused(message)) if message.contains("inside the repository"))
    );
    fs::write(
        sandbox.root().join(artifacts::BACKEND_FILE),
        "bucket = \"coda-health-prod-state\"\nprefix = \"rust-environments/stg\"\n",
    )
    .unwrap();
    let mut fake = Fake::default();
    assert!(matches!(
        plan_in(&sandbox, &mut fake),
        Err(Error::Refused(_))
    ));
    assert!(fake.calls.is_empty(), "refused before credentials");
}

#[test]
fn apply_runs_exactly_the_reviewed_plan_once() {
    let sandbox = Sandbox::new("apply");
    plan_in(&sandbox, &mut Fake::default()).unwrap();
    let mut fake = Fake::default();
    apply_in(&sandbox, &mut fake).unwrap();
    assert_eq!(fake.stages(), [Stage::Init, Stage::Apply]);
    let (apply, secret) = &fake.calls[1];
    assert!(secret.is_none(), "a saved plan needs no passwords");
    assert_eq!(apply.args[1], "apply");
    assert_eq!(
        apply.args.last().unwrap(),
        &sandbox
            .out()
            .canonicalize()
            .unwrap()
            .join(PLAN_FILE)
            .display()
            .to_string()
    );
    assert!(
        !apply
            .args
            .iter()
            .any(|arg| arg.starts_with("-var") || arg == "-auto-approve")
    );
    assert_eq!(manifest(&sandbox).apply, Some(ApplyState::Succeeded));
    let mut again = Fake::default();
    assert!(
        matches!(apply_in(&sandbox, &mut again), Err(Error::Refused(message)) if message.contains("at most once"))
    );
    assert!(again.calls.is_empty());
}

#[test]
fn modified_plans_and_changed_roots_are_refused_before_apply() {
    type Tamper = (&'static str, fn(&Sandbox));
    let tamper: [Tamper; 4] = [
        ("plan bytes", |s| {
            fs::write(s.out().join(PLAN_FILE), "other").unwrap()
        }),
        ("tfvars", |s| {
            fs::write(
                s.root().join(artifacts::VAR_FILE),
                "environment = \"prd\"\n",
            )
            .unwrap()
        }),
        ("new tf file", |s| {
            fs::write(s.root().join("extra.tf"), "").unwrap()
        }),
        ("plan permissions", |s| {
            fs::set_permissions(s.out().join(PLAN_FILE), fs::Permissions::from_mode(0o644)).unwrap()
        }),
    ];
    for (name, change) in tamper {
        let sandbox = Sandbox::new("tamper");
        plan_in(&sandbox, &mut Fake::default()).unwrap();
        change(&sandbox);
        let mut fake = Fake::default();
        let error = apply_in(&sandbox, &mut fake).unwrap_err();
        assert!(matches!(error, Error::Refused(_)), "{name}: {error}");
        assert!(fake.calls.is_empty(), "{name}: no command ran");
        assert_eq!(manifest(&sandbox).apply, None);
        let error = verify_in(&sandbox, &mut fake).unwrap_err();
        assert!(matches!(error, Error::Refused(_)), "{name}: {error}");
        assert!(fake.calls.is_empty(), "{name}: verify ran no command");
    }
}

#[test]
fn failed_or_timed_out_apply_is_uncertain_and_never_retried() {
    for (exit, state) in [
        (Exit::Code(Some(1)), ApplyState::Failed),
        (Exit::Timeout, ApplyState::TimedOut),
    ] {
        let sandbox = Sandbox::new("apply-fail");
        plan_in(&sandbox, &mut Fake::default()).unwrap();
        let mut fake = Fake {
            apply_exit: exit,
            ..Fake::default()
        };
        let error = apply_in(&sandbox, &mut fake).unwrap_err();
        assert!(matches!(error, Error::ApplyUncertain { .. }));
        let message = error.to_string();
        assert!(message.contains("Do not retry") && message.contains(artifacts::DIAGNOSTICS_FILE));
        assert_eq!(
            fake.stages()
                .iter()
                .filter(|stage| **stage == Stage::Apply)
                .count(),
            1
        );
        assert_eq!(manifest(&sandbox).apply, Some(state));
        let log = fs::read_to_string(sandbox.out().join(artifacts::DIAGNOSTICS_FILE)).unwrap();
        assert!(log.contains("apply stderr"));
        assert!(matches!(
            apply_in(&sandbox, &mut Fake::default()),
            Err(Error::Refused(_))
        ));
    }
}

#[test]
fn interrupted_apply_leaves_started_state_that_refuses_reapply() {
    let sandbox = Sandbox::new("interrupted");
    plan_in(&sandbox, &mut Fake::default()).unwrap();
    let dir = PlanDir::open(&sandbox.out()).unwrap();
    let mut recorded = dir.load_manifest().unwrap();
    recorded.apply = Some(ApplyState::Started);
    dir.save_manifest(&recorded).unwrap();
    let error = apply_in(&sandbox, &mut Fake::default()).unwrap_err();
    assert!(matches!(error, Error::Refused(message) if message.contains("Started")));
}

#[test]
fn verify_passes_with_no_change_plan_and_reviewed_settings() {
    let sandbox = Sandbox::new("verify");
    plan_in(&sandbox, &mut Fake::default()).unwrap();
    apply_in(&sandbox, &mut Fake::default()).unwrap();
    let mut fake = Fake::default();
    verify_in(&sandbox, &mut fake).unwrap();
    assert_eq!(&fake.stages()[..4], &[Stage::Credentials; 4]);
    assert_eq!(fake.stages()[4], Stage::Show);
    assert!(fake.stages().contains(&Stage::DriftPlan) && fake.stages().contains(&Stage::Readback));
    let drift = fake
        .calls
        .iter()
        .find(|(call, _)| call.stage == Stage::DriftPlan)
        .unwrap();
    assert!(drift.1.is_some() && !drift.0.args.iter().any(|arg| arg.starts_with("-out")));
    let receipt = fs::read_dir(sandbox.out())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| path.to_string_lossy().ends_with(VERIFICATION_FILE))
        .unwrap();
    let text = fs::read_to_string(receipt).unwrap();
    assert!(text.contains("\"findings\": []") && text.contains("physician-00190"));
    assert!(text.contains("not database, journey or restore qualification"));
    assert!(!text.contains("synthetic-secret"));
}

#[test]
fn verify_reports_drift_settings_readiness_and_traffic_separately_from_failure() {
    let sandbox = Sandbox::new("findings");
    plan_in(&sandbox, &mut Fake::default()).unwrap();
    let drifted = service(
        serde_json::json!({"autoscaling.knative.dev/maxScale": "2"}),
        "physician-00189",
        "physician-00190",
        serde_json::json!([
            {"revisionName": "physician-00189", "percent": 90},
            {"revisionName": "physician-00190", "percent": 10, "tag": "canary"}
        ]),
    );
    let mut fake = Fake {
        drift_exit: Exit::Code(Some(2)),
        services: VecDeque::from([drifted]),
        ..Fake::default()
    };
    let Err(Error::Findings { findings, .. }) = verify_in(&sandbox, &mut fake) else {
        panic!("findings expected");
    };
    assert_eq!(findings[0], Finding::Drift);
    let settings: Vec<_> = findings
        .iter()
        .filter_map(|finding| match finding {
            Finding::Setting {
                setting, observed, ..
            } => Some((*setting, observed.clone())),
            _ => None,
        })
        .collect();
    assert_eq!(
        settings,
        [
            (readback::Setting::MinInstances, Some("0".into())),
            (readback::Setting::CpuThrottling, Some("true".into())),
            (
                readback::Setting::VpcEgress,
                Some("private-ranges-only".into())
            ),
        ]
    );
    assert!(
        findings
            .iter()
            .any(|finding| matches!(finding, Finding::NotReady { .. }))
    );
    assert!(
        findings
            .iter()
            .any(|finding| matches!(finding, Finding::Traffic { .. }))
    );
    assert_eq!(fake.pauses, READY_POLLS - 1, "readiness polling is bounded");

    let mut failing = Fake {
        drift_exit: Exit::Code(Some(1)),
        ..Fake::default()
    };
    assert!(matches!(
        verify_in(&sandbox, &mut failing),
        Err(Error::Stage {
            stage: Stage::DriftPlan,
            ..
        })
    ));
}

#[test]
fn verify_waits_for_a_settling_rollout() {
    let sandbox = Sandbox::new("settling");
    plan_in(&sandbox, &mut Fake::default()).unwrap();
    let settling = service(
        serde_json::json!({}),
        "physician-00189",
        "physician-00190",
        serde_json::json!([]),
    );
    let mut fake = Fake {
        services: VecDeque::from([settling, healthy()]),
        ..Fake::default()
    };
    verify_in(&sandbox, &mut fake).unwrap();
    assert_eq!(fake.pauses, 1);
}

#[test]
fn expected_settings_follow_the_reviewed_plan_not_a_second_source() {
    let expected = readback::expected(planned(0, true, "PRIVATE_RANGES_ONLY").as_bytes()).unwrap();
    assert_eq!(expected[&readback::Setting::MinInstances], "0");
    assert_eq!(expected[&readback::Setting::CpuThrottling], "true");
    assert_eq!(
        expected[&readback::Setting::VpcEgress],
        "private-ranges-only"
    );
    let parsed = readback::Service::parse(
        service(
            serde_json::json!({"autoscaling.knative.dev/maxScale": "2"}),
            "r",
            "r",
            serde_json::json!([{"revisionName": "r", "percent": 100}]),
        )
        .as_bytes(),
    )
    .unwrap();
    assert!(readback::compare(&expected, &parsed).is_empty());
    assert!(readback::expected(b"{\"planned_values\":{\"root_module\":{}}}").is_err());
}

/// Real processes: synthetic `doppler`, `tofu` and `gcloud` on PATH record
/// their argv and whether they received the password variable.
#[test]
fn workflow_through_real_processes_keeps_passwords_out_of_argv() {
    let sandbox = Sandbox::new("process");
    let bin = sandbox.base.join("bin");
    fs::create_dir_all(&bin).unwrap();
    let log = sandbox.base.join("calls.log");
    let show = sandbox.base.join("show.json");
    fs::write(&show, planned(1, false, "ALL_TRAFFIC")).unwrap();
    let service = sandbox.base.join("service.json");
    fs::write(&service, healthy()).unwrap();
    for name in ["doppler", "tofu", "gcloud"] {
        let path = bin.join(name);
        fs::write(
            &path,
            include_bytes!("../../tests/fixtures/staging-reconcile/tool.sh"),
        )
        .unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    }
    let mut runner = ProcessRunner {
        path_prefix: Some(bin.clone()),
    };
    let workspace = sandbox.workspace();
    run(
        &mut runner,
        &workspace,
        Request::Plan {
            output: sandbox.out(),
        },
    )
    .unwrap();
    run(
        &mut runner,
        &workspace,
        Request::Apply {
            plan_dir: sandbox.out(),
        },
    )
    .unwrap();
    run(
        &mut runner,
        &workspace,
        Request::Verify {
            plan_dir: sandbox.out(),
        },
    )
    .unwrap();
    let calls = fs::read_to_string(&log).unwrap();
    for line in calls.lines() {
        let (head, env) = line.split_once(" env=").unwrap();
        assert!(!head.contains("synthetic-secret"), "argv leak: {head}");
        let receives = head.starts_with("tofu argv=-chdir=") && head.contains(" plan ");
        assert_eq!(env != "none", receives, "{line}");
    }
    assert!(calls.contains("\"duet-migration\":\"stg_duet_migration-synthetic-secret\""));
    assert_eq!(calls.matches("tofu argv=").count(), 8);
    assert_eq!(manifest(&sandbox).apply, Some(ApplyState::Succeeded));
}

#[test]
fn exclusive_apply_claim_blocks_a_second_invocation_before_tools() {
    let sandbox = Sandbox::new("claimed");
    plan_in(&sandbox, &mut Fake::default()).unwrap();
    let dir = PlanDir::open(&sandbox.out()).unwrap();
    dir.write_new("apply-claimed", b"another invocation")
        .unwrap();
    let mut fake = Fake::default();
    assert!(apply_in(&sandbox, &mut fake).is_err());
    assert!(fake.calls.is_empty());
}

#[test]
fn unsupported_automatic_inputs_cannot_bypass_configuration_identity() {
    for name in [
        "override.tf.json",
        "main.tofu",
        "terraform.tfvars",
        "secrets.auto.tfvars.json",
    ] {
        let sandbox = Sandbox::new("unsupported");
        fs::write(sandbox.root().join(name), "{}").unwrap();
        assert!(artifacts::config_digest(&sandbox.root()).is_err(), "{name}");
    }
}

#[test]
fn pre_epoch_clock_error_preserves_its_typed_cause() {
    use std::error::Error as _;
    let error = unix_seconds(UNIX_EPOCH - Duration::from_secs(1)).unwrap_err();
    let source = error.source().expect("typed clock error source");
    assert_eq!(
        source
            .downcast_ref::<std::time::SystemTimeError>()
            .unwrap()
            .duration(),
        Duration::from_secs(1)
    );
    assert_eq!(
        unix_seconds(UNIX_EPOCH + Duration::from_secs(42)).unwrap(),
        42
    );
}

#[test]
fn credential_encoding_error_preserves_safe_source_chain() {
    use std::error::Error as _;
    let bytes = vec![0xff, 0xfe];
    let source = std::str::from_utf8(&bytes).unwrap_err();
    let error = Error::Credential {
        role: Role::PhysicianRuntime,
        reason: CredentialFailure::Encoding(source),
        diagnostics: Diagnostics::None,
    };
    let reason = error.source().expect("credential failure cause");
    assert!(reason.source().unwrap().is::<std::str::Utf8Error>());
    assert!(!format!("{error:?}").contains("255, 254"));
}

#[test]
fn raw_process_output_debug_is_redacted() {
    let output = run_of(Exit::Success, "synthetic-password", "synthetic-password");
    let debug = format!("{output:?}");
    assert!(!debug.contains("synthetic-password"));
    assert!(!debug.contains("115, 121, 110"));
}

#[test]
fn verify_lookup_failure_precedes_opentofu() {
    let sandbox = Sandbox::new("verify-credentials");
    plan_in(&sandbox, &mut Fake::default()).unwrap();
    let mut fake = Fake::default();
    fake.doppler.insert(
        "stg_physician_runtime",
        run_of(Exit::Code(Some(1)), "", "lookup refused"),
    );
    assert!(matches!(
        verify_in(&sandbox, &mut fake),
        Err(Error::Credential { .. })
    ));
    assert!(
        fake.calls
            .iter()
            .all(|(call, _)| call.program == Program::Doppler)
    );
}

#[test]
fn process_runner_bounds_a_hung_tool() {
    let sandbox = Sandbox::new("timeout");
    let bin = sandbox.base.join("bin");
    fs::create_dir_all(&bin).unwrap();
    fs::write(
        bin.join("gcloud"),
        include_bytes!("../../tests/fixtures/staging-reconcile/hung.sh"),
    )
    .unwrap();
    fs::set_permissions(bin.join("gcloud"), fs::Permissions::from_mode(0o755)).unwrap();
    let started = std::time::Instant::now();
    let run = ProcessRunner {
        path_prefix: Some(bin),
    }
    .run(
        &Invocation::new(
            Program::Gcloud,
            Stage::Readback,
            ["x"],
            Duration::from_secs(1),
        ),
        None,
    )
    .unwrap();
    assert_eq!(run.exit, Exit::Timeout);
    assert!(started.elapsed() < Duration::from_secs(10));
}
