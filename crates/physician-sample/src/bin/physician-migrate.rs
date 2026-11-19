//! Runs the physician product's existing workspace-owned schema chains.
//! Auth, front-door, and durable conversation chains run here, never on startup.
//!
//! Under the staging-preview profile, `PHYSICIAN_STAGING_ACCOUNTS` also
//! ensures the preview sign-in accounts exist in the synthetic practice. Each
//! email gets its own stable physician id. Existing accounts are left as they
//! are; every other profile refuses the list.
//!
//! When `PHYSICIAN_SIMULATION_PRACTICES` names the rehearsal practice (again
//! staging-preview only), the job also seeds that practice, its rehearsal
//! physician and its fixture panel ([`physician_sample::rehearsal`]).
//!
//! `physician-migrate load-charts` loads the built synthetic charts and, when
//! `calendar/Appointment.ndjson` is present, syncs the synthetic practice
//! schedule in `PHYSICIAN_SYNTHETIC_CHARTS` into that practice and does nothing
//! else. A deploy never runs it. It needs the schema and takes the accounts
//! already there, whose panels start with every patient. It tries every patient
//! and exits failed if any patient or schedule appointment did not load.
//! Loading again changes only what the charts or calendar changed and leaves
//! panel choices alone.
//!
//! `physician-migrate feedback` prints every physician's feedback on Coda's
//! responses, one JSON line per rated response, for evaluation by Coda staff.
//! It only reads, and it refuses to run as a Cloud Run job.
//!
//! `physician-migrate cadence` lets operators inspect or update per-patient
//! source-check and review cadence (`get <practice_id> [patient_workspace_id]`
//! or `set <practice_id> <patient_workspace_id> <check_every_seconds>
//! <review_every_seconds>`).
//!
//! `physician-migrate inspect-runs` prints saved run records, projected steps,
//! and model reasoning summaries as JSON lines for Coda staff (`[practice_id]
//! [workspace_id] [target_id]`). It refuses to run as a Cloud Run job.

use physician_sample::STAGING_PRACTICE_ID;
use physician_sample::chart_load::{
    load_synthetic_practice, load_synthetic_practice_with_understanding,
};
use physician_sample::hosting::{
    SimulationPractices, StagingAccount, staging_accounts, staging_physician_id, synthetic_charts,
};
use physician_sample::rehearsal::{REHEARSAL_PRACTICE_ID, seed_rehearsal_practice};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

async fn migration_connection(url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut options = ConnectOptions::new(url);
    // The job runs each store operation in sequence. Share one connection so
    // independent store pools cannot exhaust the migration role's four slots.
    options
        .max_connections(1)
        .connect_timeout(std::time::Duration::from_secs(10))
        .sqlx_logging(false);
    Database::connect(options).await
}

#[tokio::main]
async fn main() {
    let url = std::env::var("PHYSICIAN_DATABASE_URL").unwrap_or_default();
    if url.trim().is_empty() {
        eprintln!("PHYSICIAN_DATABASE_URL is required");
        std::process::exit(1);
    }
    match std::env::args().nth(1).as_deref() {
        None => {}
        Some("load-charts") => {
            if let Err(error) = load_charts(&url).await {
                eprintln!("synthetic charts not loaded: {error}");
                std::process::exit(1);
            }
            return;
        }
        Some("feedback") => {
            if let Err(error) = print_feedback(&url).await {
                eprintln!("feedback not read: {error}");
                let mut cause = std::error::Error::source(&error);
                while let Some(inner) = cause {
                    eprintln!("  caused by: {inner}");
                    cause = inner.source();
                }
                std::process::exit(1);
            }
            return;
        }
        Some("cadence") => {
            let args: Vec<String> = std::env::args().skip(2).collect();
            if let Err(error) = manage_cadence(&url, &args).await {
                eprintln!("cadence command failed: {error}");
                std::process::exit(1);
            }
            return;
        }
        Some("inspect-runs") => {
            let args: Vec<String> = std::env::args().skip(2).collect();
            if let Err(error) = inspect_runs(&url, &args).await {
                eprintln!("inspect-runs not read: {error}");
                let mut cause = std::error::Error::source(&error);
                while let Some(inner) = cause {
                    eprintln!("  caused by: {inner}");
                    cause = inner.source();
                }
                std::process::exit(1);
            }
            return;
        }
        Some(other) => {
            eprintln!("physician-migrate has no command {other}");
            std::process::exit(1);
        }
    }
    let accounts = match staging_accounts(|key| std::env::var(key).ok()) {
        Ok(accounts) => accounts,
        Err(error) => {
            eprintln!("physician migrations refused: {error}");
            std::process::exit(1);
        }
    };
    let rehearsal = match SimulationPractices::from_env(|key| std::env::var(key).ok()) {
        Ok(practices) => practices.contains(REHEARSAL_PRACTICE_ID),
        Err(error) => {
            eprintln!("physician migrations refused: {error}");
            std::process::exit(1);
        }
    };
    if let Err(error) = migrate(&url, &accounts, rehearsal).await {
        eprintln!("physician migrations failed: {error}");
        std::process::exit(1);
    }
}

async fn migrate(url: &str, accounts: &[StagingAccount], rehearsal: bool) -> Result<(), String> {
    let db = migration_connection(url)
        .await
        .map_err(|error| format!("physician database connection: {error}"))?;
    let auth = physician_auth::PostgresAuthStore::from_connection(db.clone());
    auth.migrate()
        .await
        .map_err(|error| format!("physician-auth: {error}"))?;
    front_door::PostgresFrontDoorStore::from_connection(db.clone())
        .migrate()
        .await
        .map_err(|error| format!("front-door: {error}"))?;
    let runtime = local_product::runtime::Store::from_connection(db);
    runtime
        .migrate()
        .await
        .map_err(|error| format!("physician runtime migration: {error}"))?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or_default();
    for account in accounts {
        let created = auth
            .ensure_account(
                STAGING_PRACTICE_ID,
                &staging_physician_id(&account.email),
                &account.email,
                &account.display_name,
                account.admin,
                now,
            )
            .await
            .map_err(|error| format!("staging account {}: {error}", account.email))?;
        println!(
            "staging account {} {}",
            account.email,
            if created {
                "created"
            } else {
                "already present"
            }
        );
    }
    if rehearsal {
        seed_rehearsal_practice(&auth, &runtime, now).await?;
        println!("rehearsal practice {REHEARSAL_PRACTICE_ID} seeded");
    }
    Ok(())
}

/// Prints every feedback row as one JSON line, with the rated response's
/// text. The output is clinical text, so it is read on an operator's own
/// terminal: inside a Cloud Run job, where stdout reaches Cloud Logging, it
/// refuses and reads nothing.
async fn print_feedback(url: &str) -> Result<(), FeedbackReadError> {
    use std::io::Write;
    if std::env::var_os("CLOUD_RUN_JOB").is_some() {
        return Err(FeedbackReadError::InCloudRunJob);
    }
    let db = migration_connection(url)
        .await
        .map_err(FeedbackReadError::Connect)?;
    let store = local_product::runtime::Store::from_connection(db);
    let rows = store
        .feedback_for_evaluation()
        .await
        .map_err(FeedbackReadError::Read)?;
    let mut out = std::io::stdout().lock();
    for row in rows {
        let details = row.feedback.details.as_ref();
        let line = serde_json::json!({
            "practice": row.scope.practice,
            "physician": row.scope.physician,
            "workspace": row.scope.workspace,
            "chat": row.scope.chat,
            "response": row.response,
            "rating": row.feedback.rating,
            "reasons": details.map(|details| details.reasons()),
            "details": details.map(|details| details.text()),
            "saved_at": row.feedback.saved_at,
            "revision": row.feedback.revision,
            "response_text": row.response_text,
        });
        writeln!(out, "{line}").map_err(FeedbackReadError::Print)?;
    }
    let concern_rows = store
        .concern_feedback_for_evaluation()
        .await
        .map_err(FeedbackReadError::Read)?;
    for row in concern_rows {
        let line = serde_json::json!({
            "kind": "concern_done",
            "practice": row.practice,
            "workspace": row.workspace,
            "concern": row.concern_id.as_str(),
            "physician": row.physician,
            "action": row.action,
            "concern_revision": row.concern_revision,
            "urgency": row.urgency,
            "concern_reason": row.concern_reason,
            "records": row.records,
            "source_work": row.source_work,
            "saved_at": row.saved_at,
            "revision": row.revision,
        });
        writeln!(out, "{line}").map_err(FeedbackReadError::Print)?;
    }
    let reply_rows = store
        .reply_feedback_for_evaluation()
        .await
        .map_err(FeedbackReadError::Read)?;
    for row in reply_rows {
        let line = serde_json::json!({
            "kind": "reply_feedback",
            "practice": row.practice,
            "workspace": row.workspace,
            "conversation_id": row.conversation_id,
            "physician": row.physician,
            "reply_id": row.reply_id,
            "proposed_text": row.proposed_text,
            "revised_text": row.revised_text,
            "draft_status": row.draft_status,
            "draft_revision": row.draft_revision,
            "revision_instructions": row.revision_instructions,
            "copied_texts": row.copied_texts,
            "updated_at": row.updated_at,
        });
        writeln!(out, "{line}").map_err(FeedbackReadError::Print)?;
    }
    Ok(())
}

/// Why `physician-migrate feedback` read nothing, or stopped part way.
#[derive(Debug, thiserror::Error)]
enum FeedbackReadError {
    /// It prints clinical text, so it never runs as a Cloud Run job.
    #[error("it prints clinical text and never runs as a Cloud Run job")]
    InCloudRunJob,
    #[error("the physician database did not connect")]
    Connect(#[source] DbErr),
    #[error("the feedback did not read")]
    Read(#[source] DbErr),
    #[error("the feedback did not print")]
    Print(#[source] std::io::Error),
}

/// Prints saved run records, projected physician steps, and staff-only model
/// reasoning summaries as JSON lines. Refuses inside a Cloud Run job so
/// clinical text and reasoning summaries never reach Cloud Logging.
async fn inspect_runs(url: &str, args: &[String]) -> Result<(), InspectRunsError> {
    use std::io::Write;
    if std::env::var_os("CLOUD_RUN_JOB").is_some() {
        return Err(InspectRunsError::InCloudRunJob);
    }
    if args.len() > 3 {
        return Err(InspectRunsError::Usage);
    }
    let filter = local_product::runtime::run_inspector::SavedRunFilter {
        practice: args.first().cloned(),
        workspace: args.get(1).cloned(),
        target: args.get(2).cloned(),
    };
    let db = migration_connection(url)
        .await
        .map_err(InspectRunsError::Connect)?;
    let runs = local_product::runtime::Store::from_connection(db)
        .inspect_saved_runs(&filter)
        .await
        .map_err(InspectRunsError::Read)?;
    let mut out = std::io::stdout().lock();
    for run in runs {
        let line = serde_json::to_string(&run).map_err(InspectRunsError::Serialize)?;
        writeln!(out, "{line}").map_err(InspectRunsError::Print)?;
    }
    Ok(())
}

/// Why `physician-migrate inspect-runs` could not run.
#[derive(Debug, thiserror::Error)]
enum InspectRunsError {
    /// Arguments exceeded `[practice_id] [workspace_id] [target_id]`.
    #[error("usage: physician-migrate inspect-runs [practice_id] [workspace_id] [target_id]")]
    Usage,
    /// It prints clinical text and reasoning summaries, so it never runs as a Cloud Run job.
    #[error("it prints clinical text and reasoning summaries and never runs as a Cloud Run job")]
    InCloudRunJob,
    /// The database did not connect.
    #[error("the physician database did not connect")]
    Connect(#[source] DbErr),
    /// The saved runs could not be read.
    #[error("the saved runs did not read")]
    Read(#[source] DbErr),
    /// The inspected run could not be serialized.
    #[error("the inspected run did not serialize")]
    Serialize(#[source] serde_json::Error),
    /// The output stream failed while writing.
    #[error("the inspected runs did not print")]
    Print(#[source] std::io::Error),
}

async fn load_charts(url: &str) -> Result<(), String> {
    use local_product::runtime::schedule::{
        ScheduleFreshness, ScheduleStanding, ScheduleSyncOutcome,
    };
    use physician_sample::schedule::{SyntheticBankCalendar, sync_synthetic_schedule};

    let charts = synthetic_charts(|key| std::env::var(key).ok())?;
    let db = migration_connection(url)
        .await
        .map_err(|error| format!("physician database connection: {error}"))?;
    let auth = physician_auth::PostgresAuthStore::from_connection(db.clone());
    let runtime = local_product::runtime::Store::from_connection(db);
    let model_settings: Vec<_> = local_product::MODEL_SETTING_KEYS
        .into_iter()
        .filter_map(|key| {
            std::env::var(key)
                .ok()
                .filter(|value| !value.trim().is_empty())
                .map(|value| (key, value))
        })
        .collect();
    let summary = if model_settings.is_empty() {
        load_synthetic_practice(&charts, &auth, &runtime).await?
    } else {
        let environment: Vec<_> = model_settings
            .iter()
            .map(|(key, value)| (*key, value.as_str()))
            .collect();
        let roles = local_product::model_roles("responses", &environment)
            .map_err(|error| format!("model roles configuration: {error}"))?;
        let skills = local_product::physician_chat::PhysicianSkills::configured(
            std::env::var("PHYSICIAN_SKILLS_DIR").ok().as_deref(),
            false,
        )
        .map_err(|error| format!("physician skill catalog: {error}"))?;
        load_synthetic_practice_with_understanding(&charts, &auth, &runtime, &roles, &skills)
            .await?
    };
    for failure in &summary.failed {
        eprintln!("synthetic patient failed: {failure}");
    }
    println!(
        "synthetic charts: {} patients, {} records, {} changed, {} failed",
        summary.patients,
        summary.records,
        summary.changed,
        summary.failed.len()
    );
    let mut errors = Vec::new();
    if !summary.failed.is_empty() {
        errors.push(format!("{} patients failed", summary.failed.len()));
    }

    let calendar = SyntheticBankCalendar::open(&charts);
    if calendar.path().is_file() {
        match sync_synthetic_schedule(&calendar, &auth, &runtime, STAGING_PRACTICE_ID).await {
            Ok(ScheduleSyncOutcome::Applied(schedule_summary)) => {
                let schedule_failures = match &schedule_summary.standing {
                    ScheduleStanding::Usable {
                        freshness: ScheduleFreshness::Partial { failures, .. },
                        ..
                    } => {
                        for issue in failures.as_slice() {
                            eprintln!(
                                "synthetic schedule appointment failed: {}: {}",
                                issue.target(),
                                issue.reason()
                            );
                        }
                        failures.len()
                    }
                    ScheduleStanding::Absent
                    | ScheduleStanding::Unavailable { .. }
                    | ScheduleStanding::Usable { .. } => 0,
                };
                println!(
                    "synthetic schedule: {} appointments ({} matched, {} unmatched), {} changed, {} failed",
                    schedule_summary.saved_appointments,
                    schedule_summary.matched_appointments,
                    schedule_summary.unmatched_appointments,
                    schedule_summary.changed_appointments,
                    schedule_failures,
                );
                if schedule_failures > 0 {
                    errors.push(format!("{schedule_failures} schedule appointments failed"));
                }
            }
            Ok(ScheduleSyncOutcome::FailedRead { standing, .. }) => {
                let reason = match standing {
                    ScheduleStanding::Unavailable { failure, .. }
                    | ScheduleStanding::Usable {
                        freshness: ScheduleFreshness::RefreshFailed { failure },
                        ..
                    } => failure.reason,
                    ScheduleStanding::Absent | ScheduleStanding::Usable { .. } => {
                        "schedule source read failed".to_owned()
                    }
                };
                eprintln!("synthetic schedule read failed: {reason}");
                errors.push(format!("synthetic schedule read failed: {reason}"));
            }
            Err(error) => {
                eprintln!("synthetic schedule sync failed: {error}");
                errors.push(format!("synthetic schedule sync failed: {error}"));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("; "))
    }
}

async fn manage_cadence(url: &str, args: &[String]) -> Result<(), String> {
    let db = migration_connection(url)
        .await
        .map_err(|error| format!("physician database connection: {error}"))?;
    let auth = physician_auth::PostgresAuthStore::from_connection(db.clone());
    let runtime = local_product::runtime::Store::from_connection(db);
    let now_sec = runtime
        .database_clock_seconds()
        .await
        .map_err(|error| format!("database clock: {error}"))?;

    match args.first().map(String::as_str) {
        Some("get") => match args.len() {
            2 => {
                let views = runtime
                    .practice_dispatch_statuses(&auth, &args[1], now_sec)
                    .await
                    .map_err(|error| error.to_string())?;
                println!(
                    "{}",
                    serde_json::to_string(&views).map_err(|error| error.to_string())?
                );
                Ok(())
            }
            3 => {
                let view = runtime
                    .patient_dispatch_status(&auth, &args[1], &args[2], now_sec)
                    .await
                    .map_err(|error| error.to_string())?;
                println!(
                    "{}",
                    serde_json::to_string(&view).map_err(|error| error.to_string())?
                );
                Ok(())
            }
            _ => Err(
                "usage: physician-migrate cadence get <practice_id> [patient_workspace_id]"
                    .to_owned(),
            ),
        },
        Some("set") => {
            if args.len() != 5 {
                return Err(
                    "usage: physician-migrate cadence set <practice_id> <patient_workspace_id> <check_every_seconds> <review_every_seconds>"
                        .to_owned(),
                );
            }
            let check_every: u64 = args[3]
                .parse()
                .map_err(|error| format!("invalid check_every_seconds: {error}"))?;
            let review_every: u64 = args[4]
                .parse()
                .map_err(|error| format!("invalid review_every_seconds: {error}"))?;
            let cadence =
                patient_work::Cadence::new(check_every, review_every).map_err(|e| e.to_string())?;
            let view = runtime
                .set_patient_cadence(&auth, &args[1], &args[2], cadence, now_sec)
                .await
                .map_err(|error| error.to_string())?;
            println!(
                "{}",
                serde_json::to_string(&view).map_err(|error| error.to_string())?
            );
            Ok(())
        }
        Some("set-practice") => {
            if args.len() != 4 {
                return Err(
                    "usage: physician-migrate cadence set-practice <practice_id> <check_every_seconds> <review_every_seconds>"
                        .to_owned(),
                );
            }
            let check_every = args[2]
                .parse::<u64>()
                .map_err(|error| format!("check_every_seconds `{}`: {error}", args[2]))?;
            let review_every = args[3]
                .parse::<u64>()
                .map_err(|error| format!("review_every_seconds `{}`: {error}", args[3]))?;
            let cadence = patient_work::Cadence::new(check_every, review_every)
                .map_err(|error| error.to_string())?;
            let view = runtime
                .set_practice_cadence(&auth, &args[1], cadence, now_sec)
                .await
                .map_err(|error| error.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&view).map_err(|error| error.to_string())?
            );
            Ok(())
        }
        _ => Err(
            "usage: physician-migrate cadence <get|set|set-practice> <practice_id> [patient_workspace_id] [check_every_seconds review_every_seconds]"
                .to_owned(),
        ),
    }
}
