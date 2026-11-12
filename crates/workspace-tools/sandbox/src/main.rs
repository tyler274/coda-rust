//! `coda-sandbox`: set up, check and run the Coda agent sandbox.

mod cli;

use std::io::{IsTerminal as _, Write as _};
use std::path::Path;
use std::process::ExitCode;

use clap::Parser as _;
use cli::{Broker, Ci, Cli, Command, ControllerMode, Worktree};
use coda_sandbox::broker::client;
use coda_sandbox::broker::error::BrokerError;
use coda_sandbox::config::Config;
use coda_sandbox::credential;
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
        Command::Broker(command) => {
            broker_command(&cli.config, command)?;
            Ok(ExitCode::SUCCESS)
        }
        Command::Credential { operation } => {
            let config = Config::load(&cli.config)?;
            let stdin = std::io::stdin().lock();
            credential::run(*operation, &config.broker.socket, stdin, std::io::stdout())?;
            Ok(ExitCode::SUCCESS)
        }
        Command::Token { want, force } => {
            if std::io::stdout().is_terminal() && !force {
                return Err(BrokerError::Terminal.into());
            }
            let config = Config::load(&cli.config)?;
            let secret = client::secret(&config.broker.socket, *want)?;
            print(secret.expose())?;
            Ok(ExitCode::SUCCESS)
        }
    }
}

fn broker_command(config_path: &Path, command: &Broker) -> Result<(), Error> {
    use coda_sandbox::broker::commands::{self, Stored};
    let report = |item, stored| match stored {
        Stored::Reloaded => format!("{item} saved; the running broker reads it on next use\n"),
        Stored::BrokerNotRunning => {
            format!("{item} saved; no broker is running, so it reads it when it starts\n")
        }
    };
    match command {
        Broker::Serve => Ok(commands::serve(&Config::load(config_path)?)?),
        Broker::Store { item } => {
            let stored = commands::store(&Config::load(config_path)?, *item)?;
            print(&report(item, stored))
        }
        Broker::Delete { item } => {
            let stored = commands::delete(&Config::load(config_path)?, *item)?;
            print(&report(item, stored).replace("saved", "deleted"))
        }
        Broker::SelfTest => {
            let keyring = coda_sandbox::broker::keyring::SecretTool::default();
            print(&commands::self_test(&keyring)?)
        }
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
/// exits 143 on SIGTERM.
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
        let access = ctl::token::fetch(config.broker_socket.as_deref())?;
        if let ctl::token::Access::LocalOnly(why) = &access {
            eprintln!(
                "coda-sandbox ci-controller: no GitHub token, so only local requests run: {}",
                ctl::error::chain(why)
            );
        }
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
        controller.run(mode, access)
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
