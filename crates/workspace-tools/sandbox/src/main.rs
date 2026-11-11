//! `coda-sandbox`: set up, check and run the Coda agent sandbox.

mod cli;

use std::io::Write as _;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser as _;
use cli::{Ci, Cli, Command, ControllerMode, Worktree};
use coda_sandbox::config::Config;
use coda_sandbox::error::Error;
use coda_sandbox::probe::SystemProbe;
use coda_sandbox::worktree::{self, Layout, prune::Confirm};
use coda_sandbox::{carry, ci, doctor, status};

fn main() -> ExitCode {
    let cli = Cli::parse();
    match dispatch(&cli) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("coda-sandbox: {}", error.chain());
            exit_code(&error)
        }
    }
}

/// `ci run` keeps the exit codes of the script it replaced, which callers
/// tell apart: 124 for a timeout, as `timeout(1)` uses, and 2 for no spool.
fn exit_code(error: &Error) -> ExitCode {
    match error {
        Error::CiTimeout { .. } => ExitCode::from(124),
        Error::NoSpool { .. } => ExitCode::from(2),
        _ => ExitCode::FAILURE,
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
        Command::Worktree(command) => {
            let config = Config::load(&cli.config)?;
            worktree_command(&config, command)?;
            Ok(ExitCode::SUCCESS)
        }
        Command::Ci(Ci::Run { target, job }) => {
            let config = Config::load(&cli.config)?;
            let target = ci::Target::from_arg(target.as_deref());
            let status = ci::run(&config.ci, &target, job)?;
            Ok(ExitCode::from(status.0))
        }
        Command::Carry => {
            let config = Config::load(&cli.config)?;
            let carry_config = config.carry.as_ref().ok_or(Error::NoCarry)?;
            let carried = carry::carry(carry_config)?;
            print(&carry::render(&carried, &carry_config.out))?;
            Ok(ExitCode::SUCCESS)
        }
        Command::CiController { mode } => Ok(ci_controller(*mode)),
    }
}

fn worktree_command(config: &Config, command: &Worktree) -> Result<(), Error> {
    let cwd = std::env::current_dir().map_err(|source| Error::Inspect {
        path: ".".into(),
        source,
    })?;
    let layout = Layout::discover(&cwd, &config.container.repo, &config.build)?;
    match command {
        Worktree::New { slug, from } => {
            let tree = worktree::new::new(&layout, slug, from)?;
            print(&format!("{}\n", tree.display()))
        }
        Worktree::Seed { reference } => {
            worktree::seed::seed(&layout, reference.as_deref()).map(drop)
        }
        Worktree::Reset { slug } => worktree::new::reset(&layout, slug),
        Worktree::Gc { idle } => worktree::gc::gc(&layout, *idle),
        Worktree::Rm { slug } => worktree::new::rm(&layout, slug),
        Worktree::Prune { yes } => {
            let confirm = if *yes { Confirm::Yes } else { Confirm::Ask };
            worktree::prune::prune(&layout, confirm)
        }
        Worktree::Status => print(&worktree::status::status(&layout)?),
    }
}

/// The controller reads its own environment, not the sandbox config, and
/// exits 143 on SIGTERM as the Python controller did.
fn ci_controller(mode: Option<ControllerMode>) -> ExitCode {
    use coda_sandbox::ci_controller::{self as ctl, Controller, Mode};
    use ctl::termination::{TERMINATED_EXIT, Termination};
    let termination = match Termination::install() {
        Ok(termination) => termination,
        Err(error) => {
            eprintln!("coda-sandbox ci-controller: cannot handle SIGTERM: {error}");
            return ExitCode::FAILURE;
        }
    };
    let run = || -> Result<(), ctl::error::Error> {
        let config = ctl::config::ControllerConfig::from_env()?;
        let credentials = std::env::var_os("CREDENTIALS_DIRECTORY").map(PathBuf::from);
        let token = ctl::read_token(credentials.as_deref())?;
        let controller = Controller {
            config,
            disks: &ctl::disk::SystemDisks,
            memory: ctl::memory::MemoryFiles::system(),
            termination: termination.clone(),
            timers: ctl::guest::Timers::PRODUCTION,
        };
        let mode = match mode {
            Some(ControllerMode::Seed) => Mode::Seed,
            None => Mode::Pass,
        };
        controller.run(mode, token)
    };
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(ctl::error::Error::Terminated) => std::process::exit(TERMINATED_EXIT),
        Err(error) => {
            eprintln!("coda-sandbox ci-controller: {}", ctl::error::chain(&error));
            if error.is_terminated() {
                std::process::exit(TERMINATED_EXIT);
            }
            ExitCode::FAILURE
        }
    }
}

fn print(text: &str) -> Result<(), Error> {
    std::io::stdout()
        .lock()
        .write_all(text.as_bytes())
        .map_err(Error::Output)
}
