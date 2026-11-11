//! `coda-sandbox`: set up, check and run the Coda agent sandbox.

use std::io::Write as _;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use coda_sandbox::config::{self, Config};
use coda_sandbox::error::Error;
use coda_sandbox::probe::SystemProbe;
use coda_sandbox::{doctor, status};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// The sandbox config file.
    #[arg(long, global = true, env = "CODA_SANDBOX_CONFIG", default_value = config::DEFAULT_PATH)]
    config: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Check the sandbox on this machine and name the fix for each problem.
    /// Exits 1 when any check fails, else 2 when any check could not run.
    Doctor,
    /// Show the container, build root and CI state.
    Status {
        /// Print the versioned JSON schema instead of text.
        #[arg(long)]
        json: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match dispatch(&cli) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("coda-sandbox: {}", error.chain());
            ExitCode::FAILURE
        }
    }
}

fn dispatch(cli: &Cli) -> Result<ExitCode, Error> {
    let probe = SystemProbe;
    match &cli.command {
        Command::Doctor => {
            let config = Config::load_with_unknown_keys(&cli.config);
            let findings = doctor::run(&probe, &config);
            print(&doctor::render(&findings))?;
            Ok(ExitCode::from(doctor::verdict(&findings).exit_code()))
        }
        Command::Status { json } => {
            let config = Config::load(&cli.config)?;
            let status = status::collect(&probe, &cli.config, &config);
            let text = if *json {
                serde_json::to_string_pretty(&status).map_err(Error::Json)? + "\n"
            } else {
                status.render()
            };
            print(&text)?;
            Ok(ExitCode::SUCCESS)
        }
    }
}

fn print(text: &str) -> Result<(), Error> {
    std::io::stdout()
        .lock()
        .write_all(text.as_bytes())
        .map_err(Error::Output)
}
