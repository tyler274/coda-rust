//! Runs patient rounds once for one patient against the local database, with
//! the model the physician worker uses: `physician-rounds <practice>
//! <workspace>`, from the checkout's root. It is one run and then it exits;
//! nothing schedules it. Memory records when rounds finished: a finished run
//! saves a rounds mark. The command writes the run's record as JSON evidence,
//! with the newest mark before and after the run as the store reads it. It
//! exits 0 for a finished run, 2 for a run that did not finish, and 1 when it
//! could not run.
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use local_product::physician_chat::{PhysicianSkills, SkillsError};
use local_product::runtime::Store;
use local_product::runtime::engine::Event;
use local_product::runtime::memory::{PatientKey, RoundsMark};
use local_product::runtime::rounds::{self, Ending, RoundsRun};
use local_product::step_projection::{
    ProjectedStepState, extract_reasoning_summaries, project_steps_from_events,
};
use physician_sample::hosting::{self, StartupError};
use physician_sample::model::{self, ModelSource, ModelTransport, configured};
use sea_orm::{DbErr, TransactionTrait};
use serde_json::{Value, json};

/// The folder each run's record is written to.
const EVIDENCE: &str = "target/verification/physician-rounds";

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(2),
        Err(reason) => {
            eprintln!("{reason}");
            ExitCode::FAILURE
        }
    }
}

/// Why the command could not run.
#[derive(Debug, thiserror::Error)]
enum RoundsError {
    /// The command was not given a valid practice/workspace pair or inspect path.
    #[error(
        "usage: physician-rounds <practice> <workspace> | physician-rounds inspect <record.json>"
    )]
    Usage,
    /// The command runs only on a developer's machine.
    #[error("physician-rounds runs only on a developer's machine")]
    Hosted,
    /// The environment does not configure the model.
    #[error(transparent)]
    Model(#[from] StartupError),
    /// No database is configured.
    #[error("PHYSICIAN_DATABASE_URL is required")]
    NoDatabase,
    /// The database could not be reached.
    #[error("cannot connect to the physician database: {0}")]
    Connect(#[source] DbErr),
    /// The skills could not be loaded.
    #[error(transparent)]
    Skills(#[from] SkillsError),
    /// The database failed an operation.
    #[error(transparent)]
    Database(#[from] DbErr),
    /// The model's transport may not read this workspace.
    #[error("{0}")]
    SourceBoundary(String),
    /// The model could not be reached.
    #[error("cannot reach the model: {0}")]
    Provider(#[source] harness::engine::Cause),
    /// A saved run record could not be read or parsed for staff inspection.
    #[error("cannot inspect {}: {reason}", path.display())]
    Inspect {
        /// Path to the saved run record file.
        path: PathBuf,
        /// Underlying file or JSON failure.
        reason: String,
    },
    /// The run's record could not be written.
    #[error("cannot write {}: {source}", path.display())]
    Evidence {
        /// Where the record was to go.
        path: PathBuf,
        /// The write's failure.
        #[source]
        source: std::io::Error,
    },
}

/// Runs rounds once (`true` when finished) or inspects a saved rounds record.
async fn run() -> Result<bool, RoundsError> {
    let get = |key: &str| std::env::var(key).ok();
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    if let [cmd, path] = &arguments[..]
        && cmd == "inspect"
    {
        if hosting::is_hosted(get) {
            return Err(RoundsError::Hosted);
        }
        inspect_record(Path::new(path))?;
        return Ok(true);
    }
    let [practice, workspace] = &arguments[..] else {
        return Err(RoundsError::Usage);
    };
    if hosting::is_hosted(get) {
        return Err(RoundsError::Hosted);
    }
    let (transport, _) = ModelTransport::from_env(&get, false)?;
    let model = ModelSource::from_env(transport, &get)?;
    let database = configured(&get, "PHYSICIAN_DATABASE_URL").ok_or(RoundsError::NoDatabase)?;
    let store = Store::connect(&database)
        .await
        .map_err(RoundsError::Connect)?;
    let skills =
        PhysicianSkills::configured(configured(&get, "PHYSICIAN_SKILLS_DIR").as_deref(), false)?;
    let patient = PatientKey::for_rounds(practice, workspace)?;
    // A developer's own ChatGPT plan keeps the worker's boundary: it reads
    // only the synthetic practice.
    let tx = store.connection().begin().await?;
    model::source_boundary(model.synthetic_only(), &tx, workspace)
        .await
        .map_err(RoundsError::SourceBoundary)?;
    tx.commit().await?;

    let before = store.last_rounds(&patient).await?;
    let now = Utc::now();
    let quota: Arc<dyn harness::provider::QuotaGate> = Arc::new(store.clone());
    let mut provider = model.provider(quota).await.map_err(RoundsError::Provider)?;
    let done = rounds::run(
        &store,
        &mut provider,
        &patient,
        now,
        &skills,
        &model.roles.background,
    )
    .await;
    let after = store.last_rounds(&patient).await?;
    let unfinished = match &done.ending {
        Ending::Finished { .. } => ProjectedStepState::Done,
        Ending::Limit { .. } | Ending::Failed(_) => ProjectedStepState::Failed,
    };
    let steps = project_steps_from_events([(unfinished, done.events.as_slice())], &|_| None);
    let evidence = write_evidence(practice, workspace, now, &done, &steps, before, after)?;
    println!("{}", ending_summary(&done.ending));
    for step in &steps {
        println!("step: {}", step.display_line());
    }
    println!("run record: {}", evidence.display());
    match after {
        Some(mark) => println!(
            "rounds last finished: {}, for a run that started at {}",
            mark.saved.to_rfc3339(),
            mark.started.to_rfc3339()
        ),
        None => println!("rounds last finished: never"),
    }
    Ok(done.finished())
}

fn inspect_record(path: &Path) -> Result<(), RoundsError> {
    let raw = std::fs::read_to_string(path).map_err(|err| RoundsError::Inspect {
        path: path.to_path_buf(),
        reason: err.to_string(),
    })?;
    let record: Value = serde_json::from_str(&raw).map_err(|err| RoundsError::Inspect {
        path: path.to_path_buf(),
        reason: err.to_string(),
    })?;
    let events: Vec<Event> = record
        .get("events")
        .cloned()
        .map(serde_json::from_value)
        .transpose()
        .map_err(|err| RoundsError::Inspect {
            path: path.to_path_buf(),
            reason: format!("invalid events array: {err}"),
        })?
        .unwrap_or_default();
    let unfinished = if record
        .get("ending")
        .and_then(|v| v.get("finished"))
        .is_some()
    {
        ProjectedStepState::Done
    } else {
        ProjectedStepState::Failed
    };
    let steps = project_steps_from_events([(unfinished, events.as_slice())], &|_| None);
    let reasoning = extract_reasoning_summaries(&events);
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "practice": record.get("practice"),
            "workspace": record.get("workspace"),
            "started_at": record.get("started_at"),
            "ending": record.get("ending"),
            "projected_steps": steps,
            "reasoning_summaries": reasoning,
            "event_count": events.len(),
        }))
        .map_err(|err| RoundsError::Inspect {
            path: path.to_path_buf(),
            reason: err.to_string(),
        })?
    );
    Ok(())
}

/// How the run ended, as the evidence holds it.
fn ending(ending: &Ending) -> Value {
    match ending {
        Ending::Finished { account } => json!({"finished": {"account": account}}),
        Ending::Limit { backstops, text } => {
            json!({"limit": {"backstops": backstops, "text": text}})
        }
        Ending::Failed(failure) => json!({"failed": failure.to_string()}),
    }
}

fn ending_summary(ending: &Ending) -> String {
    match ending {
        Ending::Finished { account } => format!("finished: {account}"),
        Ending::Limit { text, .. } => format!("stopped at a limit: {text}"),
        Ending::Failed(failure) => format!("failed: {failure}"),
    }
}

/// A rounds mark as the evidence holds it; null when rounds never finished.
fn mark(mark: Option<RoundsMark>) -> Value {
    match mark {
        None => Value::Null,
        Some(mark) => json!({
            "started_at": mark.started.to_rfc3339(),
            "saved_at": mark.saved.to_rfc3339(),
        }),
    }
}

/// Writes the run's record, with the newest rounds mark before and after
/// the run, and returns where.
fn write_evidence(
    practice: &str,
    workspace: &str,
    start: DateTime<Utc>,
    done: &RoundsRun,
    steps: &[local_product::step_projection::ProjectedStep],
    before: Option<RoundsMark>,
    after: Option<RoundsMark>,
) -> Result<PathBuf, RoundsError> {
    let record = json!({
        "practice": practice,
        "workspace": workspace,
        "started_at": start.to_rfc3339(),
        "ending": ending(&done.ending),
        "last_rounds_before": mark(before),
        "last_rounds_after": mark(after),
        "usage": done.usage(),
        "steps": steps,
        "events": done.events,
    });
    let file: String = workspace
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let path = Path::new(EVIDENCE).join(format!("{file}-{}.json", start.format("%Y%m%dT%H%M%SZ")));
    let write = || -> std::io::Result<()> {
        std::fs::create_dir_all(EVIDENCE)?;
        std::fs::write(&path, serde_json::to_string_pretty(&record)?)
    };
    match write() {
        Ok(()) => Ok(path),
        Err(source) => Err(RoundsError::Evidence { path, source }),
    }
}
