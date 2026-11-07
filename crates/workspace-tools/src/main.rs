//! Workspace commands that used to live under `scripts/`.
//!
//! ```text
//! cargo run -p workspace-tools -- generate-duet-wire [--check]
//! cargo run -p workspace-tools -- install-doppler OUTPUT_PATH
//! cargo run -p workspace-tools -- apply-migrations [--] [psql args...]
//! cargo run -p workspace-tools -- pg-docker initdb|pg_ctl|postgres [args...]
//! cargo run -p workspace-tools -- pg-docker link DEST
//! cargo run -p workspace-tools -- clinical-serving-startup --data-root PATH --image IMAGE --env-file PATH
//! cargo run -p workspace-tools -- staging-reconcile plan|apply|verify [options]
//! cargo run -p workspace-tools -- observability-drill datadog|grafana|watchdog fail|recover|status --output-dir DIR
//! ```

mod doppler;
mod failure;
mod migrations;
mod observability_drill;
mod pg_docker;
mod serving;
mod staging_reconcile;
mod staging_release;
mod staging_restore;
mod wire;

use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use failure::Failure;

fn main() -> ExitCode {
    match dispatch() {
        Ok(()) => ExitCode::SUCCESS,
        Err(failure) => {
            eprintln!("{}", failure.message);
            ExitCode::from(failure.code)
        }
    }
}

fn dispatch() -> Result<(), Failure> {
    let mut args = env::args();
    let argv0 = args.next().unwrap_or_default();
    let rest: Vec<String> = args.collect();
    if let Some(tool) = pg_docker::tool_from_argv0(&argv0) {
        return pg_docker::launch(tool, &rest);
    }
    if Path::new(&argv0).file_name().and_then(|name| name.to_str())
        == Some("clinical-serving-startup")
    {
        return serving::from_args(&rest);
    }
    let (command, rest) = rest
        .split_first()
        .map_or((None, &[][..]), |(head, tail)| (Some(head.as_str()), tail));
    match command {
        Some("generate-duet-wire") => wire::from_args(rest),
        Some("install-doppler") => doppler::from_args(rest),
        Some("apply-migrations") => migrations::from_args(rest),
        Some("pg-docker") => pg_docker::from_args(rest),
        Some("clinical-serving-startup") => serving::from_args(rest),
        Some("staging-release") => staging_release::from_args(rest),
        Some("staging-restore") => staging_restore::from_args(rest),
        Some("staging-reconcile") => staging_reconcile::from_args(rest),
        Some("observability-drill") => observability_drill::from_args(rest),
        _ => Err(usage()),
    }
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn usage() -> Failure {
    Failure::new(
        2,
        "\
workspace-tools runs the workspace commands that used to be shell scripts.

  workspace-tools generate-duet-wire [--check]
  workspace-tools install-doppler OUTPUT_PATH
  workspace-tools apply-migrations [--] [psql args...]
  workspace-tools pg-docker initdb|pg_ctl|postgres [args...]
  workspace-tools pg-docker link DEST
  workspace-tools clinical-serving-startup --data-root PATH --image IMAGE --env-file PATH
  workspace-tools staging-release init|deploy|finalize [options]
  workspace-tools staging-restore --help
  workspace-tools staging-reconcile plan|apply|verify (--help for options)
  workspace-tools observability-drill datadog|grafana|watchdog fail|recover|status --output-dir DIR",
    )
}
