use std::io;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;
use std::thread;

use chrono::Utc;
use clap::{Parser, Subcommand};
use coda_swarm::adapter::{Registry, RegistryAdapter, RegistryError};
use coda_swarm::mcp::{RosterOnlySpawner, Server};
use coda_swarm::supervisor::{SandboxKeys, SandboxWorkspaces, Supervisor, SupervisorConfig};
use coda_swarm::{
    FileStore, Member, MemberName, Policy, Role, RosterError, Store, SwarmId, UpdateError,
};
use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;

#[derive(Parser)]
#[command(
    name = "coda-swarm",
    about = "Shared swarm board for cooperating agents"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Serve one member's board connection as a stdio MCP server.
    Mcp {
        /// The swarm directory.
        #[arg(long, env = "CODA_SWARM_DIR")]
        dir: PathBuf,
        /// The roster member every tool call acts as.
        #[arg(long, env = "CODA_SWARM_AGENT", value_parser = MemberName::parse)]
        agent: MemberName,
        /// Create the swarm if it does not exist, with the agent as coordinator.
        #[arg(long)]
        create: bool,
        /// The model registry. With it, the coordinator's server runs the
        /// members it adds until stdin closes or SIGTERM/SIGINT; other
        /// members' servers ignore it. A missing file means no registry.
        #[arg(long, env = "CODA_SWARM_MODELS")]
        models: Option<PathBuf>,
    },
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Store(#[from] coda_swarm::StoreError),
    #[error("{agent} cannot coordinate this swarm")]
    Coordinator {
        agent: MemberName,
        #[source]
        source: UpdateError<RosterError>,
    },
    #[error(transparent)]
    Registry(#[from] RegistryError),
    #[error("stop members")]
    Shutdown(#[source] coda_swarm::StoreError),
    #[error("serve stdio")]
    Io(#[from] io::Error),
}

fn open(dir: PathBuf, agent: &MemberName, create: bool) -> Result<FileStore, Error> {
    if !create {
        return Ok(FileStore::open(dir, Policy::default())?);
    }
    let now = Utc::now();
    let id = SwarmId(format!("{}-{}", now.timestamp_millis(), std::process::id()));
    let store = FileStore::create(dir, id, now, Policy::default())?;
    store
        .update_roster(|roster| {
            if roster.members.contains_key(agent) {
                return match roster.member(agent)?.role {
                    Role::Coordinator => Ok(()),
                    _ => Err(RosterError::Exists(agent.clone())),
                };
            }
            roster.add_member(agent.clone(), Member::new(Role::Coordinator, None))
        })
        .map_err(|source| Error::Coordinator {
            agent: agent.clone(),
            source,
        })?;
    Ok(store)
}

/// The coordinator's supervisor, if it has a registry. Workers' servers
/// never get one, even when they inherit `CODA_SWARM_MODELS`.
fn supervisor(
    store: &FileStore,
    agent: &MemberName,
    models: Option<PathBuf>,
) -> Result<Option<Arc<Supervisor<FileStore>>>, Error> {
    let Some(models) = models else {
        return Ok(None);
    };
    let role = store.roster()?.member(agent).map(|member| member.role);
    if role != Ok(Role::Coordinator) {
        return Ok(None);
    }
    let registry = match Registry::load(&models) {
        Ok(registry) => registry,
        Err(RegistryError::Read { source, .. }) if source.kind() == io::ErrorKind::NotFound => {
            return Ok(None);
        }
        Err(error) => return Err(error.into()),
    };
    let adapter = RegistryAdapter::new(registry);
    let mut config = SupervisorConfig::new(
        std::env::current_dir()?,
        store.dir(),
        store.meta()?.swarm_id.0,
    );
    config.mcp_program = std::env::current_exe()?;
    Ok(Some(Arc::new(Supervisor::new(
        store.clone(),
        agent.clone(),
        Box::new(adapter),
        Box::new(SandboxWorkspaces::default()),
        Box::new(SandboxKeys::default()),
        config,
    ))))
}

/// Stops every member on SIGTERM or SIGINT, then exits. Closing stdin stops
/// them through [`run`]'s return path instead.
fn stop_on_signal(supervisor: Arc<Supervisor<FileStore>>) -> Result<(), Error> {
    let mut signals = Signals::new([SIGTERM, SIGINT])?;
    thread::spawn(move || {
        if let Some(signal) = signals.forever().next() {
            let code = match supervisor.shutdown() {
                Ok(_) => 128 + signal,
                Err(error) => {
                    eprintln!("coda-swarm: stop members: {error}");
                    1
                }
            };
            std::process::exit(code);
        }
    });
    Ok(())
}

fn run(cli: Cli) -> Result<(), Error> {
    match cli.command {
        Command::Mcp {
            dir,
            agent,
            create,
            models,
        } => {
            let store = open(dir, &agent, create)?;
            let (stdin, stdout) = (io::stdin().lock(), io::stdout().lock());
            match supervisor(&store, &agent, models)? {
                None => Server::new(store, RosterOnlySpawner, agent).serve(stdin, stdout)?,
                Some(supervisor) => {
                    stop_on_signal(Arc::clone(&supervisor))?;
                    let served =
                        Server::new(store, Arc::clone(&supervisor), agent).serve(stdin, stdout);
                    supervisor.shutdown().map_err(Error::Shutdown)?;
                    served?;
                }
            }
            Ok(())
        }
    }
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let mut text = error.to_string();
            let mut source = std::error::Error::source(&error);
            while let Some(cause) = source {
                text.push_str(": ");
                text.push_str(&cause.to_string());
                source = cause.source();
            }
            eprintln!("coda-swarm: {text}");
            ExitCode::FAILURE
        }
    }
}
