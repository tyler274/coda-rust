//! Operator-only reconciliation of the `rust-environments` staging root:
//! a private saved plan, a separately authorized exact-plan apply, and a
//! no-change plan plus physician readback. Infrastructure only; not database,
//! journey or restore qualification.
mod artifacts;
mod credentials;
mod readback;
#[cfg(test)]
mod tests;
mod tools;

use crate::failure::Failure;
use artifacts::{
    ApplyState, Backend, Manifest, PLAN_FILE, PlanDir, PlanOutcome, REVIEW_FILE, SCHEMA_VERSION,
    Sha256Hex, StageRecord,
};
use credentials::{CredentialFailure, Role};
use readback::Finding;
use std::{
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tools::{Exit, Invocation, ProcessRunner, Program, Run, Runner, Stage};

const PROJECT: &str = "coda-rust-staging";
const REGION: &str = "us-east1";
const ROOT: &str = "infra/gcp/rust-environments";
const VERIFICATION_FILE: &str = "verification.json";
const INIT_TIMEOUT: Duration = Duration::from_secs(300);
const PLAN_TIMEOUT: Duration = Duration::from_secs(900);
const SHOW_TIMEOUT: Duration = Duration::from_secs(120);
const APPLY_TIMEOUT: Duration = Duration::from_secs(2700);
const READ_TIMEOUT: Duration = Duration::from_secs(120);
const READY_POLLS: u32 = 30;
const READY_INTERVAL: Duration = Duration::from_secs(10);

/// Where a failed stage's raw output went. Never printed inline.
#[derive(Debug)]
enum Diagnostics {
    None,
    Retained(PathBuf),
    /// The primary failure stands; its log could not be written.
    Unavailable(Box<Error>),
}
impl std::fmt::Display for Diagnostics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => Ok(()),
            Self::Retained(path) => write!(f, "; private diagnostics: {}", path.display()),
            Self::Unavailable(error) => write!(f, "; diagnostics could not be saved ({error})"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("staging reconcile refused: {0}")]
    Refused(String),
    #[error("staging reconcile I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("system clock precedes the Unix epoch: {0}")]
    Clock(#[from] std::time::SystemTimeError),
    #[error(
        "Doppler lookup for {role} ({config}) failed: {reason}. No OpenTofu command ran{diagnostics}",
        config = role.doppler_config()
    )]
    Credential {
        role: Role,
        #[source]
        reason: CredentialFailure,
        diagnostics: Diagnostics,
    },
    #[error("{stage} failed ({exit:?}){diagnostics}", stage = stage.name())]
    Stage {
        stage: Stage,
        exit: Exit,
        diagnostics: Diagnostics,
    },
    #[error(
        "apply outcome is uncertain ({exit:?}). Do not retry or roll back: inspect state, the \
         lock and live resources with the authorized operator, then plan again{diagnostics}"
    )]
    ApplyUncertain {
        exit: Exit,
        diagnostics: Diagnostics,
    },
    #[error("verification found {} difference(s); see {}", findings.len(), receipt.display())]
    Findings {
        findings: Vec<Finding>,
        receipt: PathBuf,
    },
    #[error("{primary}; additionally the manifest could not record it ({persistence})")]
    Unrecorded {
        #[source]
        primary: Box<Error>,
        persistence: Box<Error>,
    },
}
type Result<T> = std::result::Result<T, Error>;
fn refuse(message: impl Into<String>) -> Error {
    Error::Refused(message.into())
}

const USAGE: &str = "\
staging-reconcile plan --output-dir NEW_PRIVATE_DIR
staging-reconcile apply --plan-dir DIR --authorize-staging-apply
staging-reconcile verify --plan-dir DIR

Run from the Doppler-authorized default `nix develop` shell. `plan` reads the four
staging database passwords from Doppler, saves a private plan and its review text
outside the repository, and stops. Review it and obtain operator approval before
`apply`, which applies exactly that saved plan once. `verify` runs a fresh plan that
must report no changes and compares the live physician service with the reviewed
plan. Plan directories contain credentials: never publish them.";

#[derive(Debug, PartialEq, Eq)]
enum Request {
    Plan { output: PathBuf },
    Apply { plan_dir: PathBuf },
    Verify { plan_dir: PathBuf },
}
impl Request {
    fn parse(args: &[String]) -> Result<Self> {
        let (command, rest) = args.split_first().ok_or_else(|| refuse(USAGE))?;
        let mut path = None;
        let mut authorized = false;
        let flag = match command.as_str() {
            "plan" => "--output-dir",
            "apply" | "verify" => "--plan-dir",
            _ => return Err(refuse(USAGE)),
        };
        let mut rest = rest.iter();
        while let Some(arg) = rest.next() {
            match arg.as_str() {
                name if name == flag && path.is_none() => {
                    path = Some(PathBuf::from(
                        rest.next()
                            .ok_or_else(|| refuse(format!("{flag} needs a path")))?,
                    ));
                }
                "--authorize-staging-apply" if command == "apply" && !authorized => {
                    authorized = true;
                }
                _ => return Err(refuse(format!("unexpected argument {arg}\n{USAGE}"))),
            }
        }
        let path = path.ok_or_else(|| refuse(format!("{flag} is required\n{USAGE}")))?;
        match command.as_str() {
            "plan" => Ok(Self::Plan { output: path }),
            "apply" if authorized => Ok(Self::Apply { plan_dir: path }),
            "apply" => Err(refuse(
                "apply needs --authorize-staging-apply after the saved plan was reviewed and approved",
            )),
            _ => Ok(Self::Verify { plan_dir: path }),
        }
    }
}

pub fn from_args(args: &[String]) -> std::result::Result<(), Failure> {
    if args == ["--help"] {
        println!("{USAGE}");
        return Ok(());
    }
    let result = Request::parse(args).and_then(|request| {
        require_tools()?;
        let workspace = crate::workspace_root().canonicalize()?;
        run(&mut ProcessRunner::default(), &workspace, request)
    });
    result.map_err(|error| {
        let code = match error {
            Error::Findings { .. } => 3,
            Error::Refused(_) => 2,
            _ => 1,
        };
        Failure::with_source(code, error.to_string(), error)
    })
}

fn require_tools() -> Result<()> {
    let path = std::env::var_os("PATH").unwrap_or_default();
    for tool in ["timeout", "doppler", "tofu", "gcloud"] {
        if !std::env::split_paths(&path).any(|dir| dir.join(tool).is_file()) {
            return Err(refuse(format!(
                "{tool} is not on PATH; run inside the repository's `nix develop` shell"
            )));
        }
    }
    Ok(())
}

fn run(runner: &mut impl Runner, workspace: &Path, request: Request) -> Result<()> {
    let root = workspace.join(ROOT);
    match request {
        Request::Plan { output } => {
            let dir = PlanDir::create(&output, workspace)?;
            let manifest = plan(runner, &root, &dir)?;
            print_plan(&dir, &manifest);
            Ok(())
        }
        Request::Apply { plan_dir } => apply(runner, &root, &PlanDir::open(&plan_dir)?),
        Request::Verify { plan_dir } => verify(runner, &root, &PlanDir::open(&plan_dir)?),
    }
}

/// Run one stage, report its safe summary, and fail with retained diagnostics.
fn stage(
    runner: &mut impl Runner,
    dir: &PlanDir,
    invocation: &Invocation,
    secret: Option<&credentials::Secret>,
    accept: &[Exit],
) -> Result<Run> {
    let run = runner.run(invocation, secret)?;
    eprintln!(
        "staging-reconcile: {} {:?} in {:.1}s",
        invocation.stage.name(),
        run.exit,
        run.elapsed.as_secs_f64()
    );
    if run.exit == Exit::Success || accept.contains(&run.exit) {
        return Ok(run);
    }
    Err(Error::Stage {
        stage: invocation.stage,
        exit: run.exit,
        diagnostics: dir.append_log(invocation.stage, &run, true).into(),
    })
}

fn init(runner: &mut impl Runner, root: &Path, dir: &PlanDir) -> Result<StageRecord> {
    let invocation = Invocation::tofu(
        root,
        Stage::Init,
        [
            "init",
            "-input=false",
            "-no-color",
            &format!("-backend-config={}", artifacts::BACKEND_FILE),
        ],
        INIT_TIMEOUT,
    );
    let run = stage(runner, dir, &invocation, None, &[])?;
    Ok(StageRecord::of(Stage::Init, &run))
}

/// `-detailed-exitcode`: 0 no changes, 2 changes, anything else failure.
fn plan_outcome(exit: Exit) -> Option<PlanOutcome> {
    match exit {
        Exit::Success => Some(PlanOutcome::NoChanges),
        Exit::Code(Some(2)) => Some(PlanOutcome::Changes),
        _ => None,
    }
}

fn plan(runner: &mut impl Runner, root: &Path, dir: &PlanDir) -> Result<Manifest> {
    let backend = Backend::read(root)?;
    let config_digest = artifacts::config_digest(root)?;
    let secret = credentials::fetch(runner, dir)?;
    let mut stages = vec![init(runner, root, dir)?];
    let plan_path = dir.file(PLAN_FILE);
    let invocation = Invocation::tofu(
        root,
        Stage::Plan,
        [
            "plan".to_owned(),
            "-input=false".into(),
            "-no-color".into(),
            "-detailed-exitcode".into(),
            format!("-var-file={}", artifacts::VAR_FILE),
            format!("-out={}", plan_path.display()),
        ],
        PLAN_TIMEOUT,
    );
    let run = stage(
        runner,
        dir,
        &invocation,
        Some(&secret),
        &[Exit::Code(Some(2))],
    )?;
    drop(secret);
    dir.restrict(PLAN_FILE)?;
    let outcome = plan_outcome(run.exit).ok_or_else(|| refuse("unexpected plan exit"))?;
    stages.push(StageRecord::of(Stage::Plan, &run));
    let show = Invocation::tofu(
        root,
        Stage::Show,
        [
            "show".to_owned(),
            "-no-color".into(),
            plan_path.display().to_string(),
        ],
        SHOW_TIMEOUT,
    );
    let shown = stage(runner, dir, &show, None, &[])?;
    stages.push(StageRecord::of(Stage::Show, &shown));
    dir.write_new(REVIEW_FILE, &shown.stdout)?;
    if artifacts::config_digest(root)? != config_digest {
        return Err(refuse(
            "the root changed while planning; plan again from a stable checkout",
        ));
    }
    let manifest = Manifest {
        schema_version: SCHEMA_VERSION,
        backend,
        config_digest,
        plan_digest: Sha256Hex::of(&dir.read_private(PLAN_FILE)?),
        outcome,
        planned_at_unix_seconds: now()?,
        stages,
        apply: None,
    };
    dir.save_manifest(&manifest)?;
    Ok(manifest)
}

fn print_plan(dir: &PlanDir, manifest: &Manifest) {
    println!(
        "Saved {:?} plan sha256:{} for config sha256:{}.",
        manifest.outcome,
        manifest.plan_digest.as_str(),
        manifest.config_digest.as_str()
    );
    println!(
        "Review (SENSITIVE, never publish): {}",
        dir.file(REVIEW_FILE).display()
    );
    match manifest.outcome {
        PlanOutcome::NoChanges => println!(
            "No changes. Run `staging-reconcile verify --plan-dir {}` for the live readback.",
            dir.path().display()
        ),
        PlanOutcome::Changes => println!(
            "STOP: review every change and obtain operator approval, then run \
             `staging-reconcile apply --plan-dir {} --authorize-staging-apply`.",
            dir.path().display()
        ),
    }
}

/// The saved plan still matches what was reviewed, for the same root and state.
fn check_identity(root: &Path, dir: &PlanDir, manifest: &Manifest) -> Result<()> {
    if Backend::read(root)? != manifest.backend {
        return Err(refuse("backend differs from the reviewed plan"));
    }
    if artifacts::config_digest(root)? != manifest.config_digest {
        return Err(refuse(
            "the staging root changed since this plan was reviewed; plan and review again",
        ));
    }
    if Sha256Hex::of(&dir.read_private(PLAN_FILE)?) != manifest.plan_digest {
        return Err(refuse("saved plan does not match its reviewed digest"));
    }
    Ok(())
}

fn apply(runner: &mut impl Runner, root: &Path, dir: &PlanDir) -> Result<()> {
    let mut manifest = dir.load_manifest()?;
    if let Some(state) = manifest.apply {
        return Err(refuse(format!(
            "this plan already has apply state {state:?}; a plan is applied at most once. \
             Inspect, then plan again"
        )));
    }
    if manifest.outcome != PlanOutcome::Changes {
        return Err(refuse("the reviewed plan has no changes to apply"));
    }
    check_identity(root, dir, &manifest)?;
    dir.write_new(
        "apply-claimed",
        b"Apply claimed. Inspect outcome before planning again.\n",
    )?;
    std::fs::File::open(dir.path())?.sync_all()?;
    manifest.stages.push(init(runner, root, dir)?);
    // Recorded before the mutation: a crash leaves `Started`, which means inspect.
    manifest.apply = Some(ApplyState::Started);
    dir.save_manifest(&manifest)?;
    let invocation = Invocation::tofu(
        root,
        Stage::Apply,
        [
            "apply".to_owned(),
            "-input=false".into(),
            "-no-color".into(),
            dir.file(PLAN_FILE).display().to_string(),
        ],
        APPLY_TIMEOUT,
    );
    let run = runner.run(&invocation, None)?;
    eprintln!(
        "staging-reconcile: apply {:?} in {:.1}s",
        run.exit,
        run.elapsed.as_secs_f64()
    );
    let diagnostics: Diagnostics = dir.append_log(Stage::Apply, &run, true).into();
    manifest.stages.push(StageRecord::of(Stage::Apply, &run));
    let (state, result) = match run.exit {
        Exit::Success => (ApplyState::Succeeded, Ok(())),
        Exit::Timeout => (
            ApplyState::TimedOut,
            Err(Error::ApplyUncertain {
                exit: run.exit,
                diagnostics,
            }),
        ),
        Exit::Code(_) => (
            ApplyState::Failed,
            Err(Error::ApplyUncertain {
                exit: run.exit,
                diagnostics,
            }),
        ),
    };
    manifest.apply = Some(state);
    match (result, dir.save_manifest(&manifest)) {
        (Ok(()), saved) => saved?,
        (Err(primary), Ok(())) => return Err(primary),
        (Err(primary), Err(persistence)) => {
            return Err(Error::Unrecorded {
                primary: Box::new(primary),
                persistence: Box::new(persistence),
            });
        }
    }
    println!(
        "Applied the reviewed plan. Run `staging-reconcile verify --plan-dir {}`.",
        dir.path().display()
    );
    Ok(())
}

#[derive(Debug, serde::Serialize)]
struct Verification<'a> {
    plan_digest: &'a Sha256Hex,
    config_digest: &'a Sha256Hex,
    apply: Option<ApplyState>,
    verified_at_unix_seconds: u64,
    drift_plan: StageRecord,
    latest_ready_revision: Option<String>,
    findings: &'a [Finding],
    scope: &'static str,
}

fn verify(runner: &mut impl Runner, root: &Path, dir: &PlanDir) -> Result<()> {
    let manifest = dir.load_manifest()?;
    if matches!(
        manifest.apply,
        Some(ApplyState::Started | ApplyState::TimedOut | ApplyState::Failed)
    ) {
        eprintln!(
            "staging-reconcile: WARNING apply state {:?}; verification does not resolve it",
            manifest.apply
        );
    }
    check_identity(root, dir, &manifest)?;
    let secret = credentials::fetch(runner, dir)?;
    let shown = stage(
        runner,
        dir,
        &Invocation::tofu(
            root,
            Stage::Show,
            [
                "show".to_owned(),
                "-json".into(),
                dir.file(PLAN_FILE).display().to_string(),
            ],
            SHOW_TIMEOUT,
        ),
        None,
        &[],
    )?;
    let expected = readback::expected(&shown.stdout)?;
    drop(shown);
    init(runner, root, dir)?;
    let drift = Invocation::tofu(
        root,
        Stage::DriftPlan,
        [
            "plan".to_owned(),
            "-input=false".into(),
            "-no-color".into(),
            "-detailed-exitcode".into(),
            format!("-var-file={}", artifacts::VAR_FILE),
        ],
        PLAN_TIMEOUT,
    );
    let drift_run = stage(runner, dir, &drift, Some(&secret), &[Exit::Code(Some(2))])?;
    drop(secret);
    let mut findings = Vec::new();
    if plan_outcome(drift_run.exit) == Some(PlanOutcome::Changes) {
        if let Err(error) = dir.append_log(Stage::DriftPlan, &drift_run, true) {
            eprintln!("staging-reconcile: drift plan text could not be saved ({error})");
        }
        findings.push(Finding::Drift);
    }
    let service = read_settled(runner, dir)?;
    findings.extend(readback::compare(&expected, &service));
    let receipt = Verification {
        plan_digest: &manifest.plan_digest,
        config_digest: &manifest.config_digest,
        apply: manifest.apply,
        verified_at_unix_seconds: now()?,
        drift_plan: StageRecord::of(Stage::DriftPlan, &drift_run),
        latest_ready_revision: service.latest_ready().map(str::to_owned),
        findings: &findings,
        scope: "infrastructure reconciliation only; not database, journey or restore qualification",
    };
    let name = format!("{}-{VERIFICATION_FILE}", receipt.verified_at_unix_seconds);
    let path = dir.write_new(&name, &serde_json::to_vec_pretty(&receipt)?)?;
    if !findings.is_empty() {
        return Err(Error::Findings {
            findings,
            receipt: path,
        });
    }
    println!(
        "Verified: no-change plan; physician {} Ready with all traffic and reviewed settings. \
         Receipt: {}. This is not database or journey qualification.",
        receipt.latest_ready_revision.as_deref().unwrap_or("?"),
        path.display()
    );
    Ok(())
}

/// Poll a bounded number of times while a rollout is still settling.
fn read_settled(runner: &mut impl Runner, dir: &PlanDir) -> Result<readback::Service> {
    let describe = Invocation::new(
        Program::Gcloud,
        Stage::Readback,
        [
            "run",
            "services",
            "describe",
            readback::SERVICE,
            "--project",
            PROJECT,
            "--region",
            REGION,
            "--quiet",
            "--format=json(spec.template.metadata.annotations,status)",
        ],
        READ_TIMEOUT,
    );
    let mut polls = 0;
    loop {
        let run = stage(runner, dir, &describe, None, &[])?;
        let service = readback::Service::parse(&run.stdout)?;
        polls += 1;
        if service.settled() || polls >= READY_POLLS {
            return Ok(service);
        }
        runner.pause(READY_INTERVAL);
    }
}

fn now() -> Result<u64> {
    unix_seconds(SystemTime::now())
}

fn unix_seconds(time: SystemTime) -> Result<u64> {
    time.duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .map_err(Error::from)
}
