use std::io;
use std::path::PathBuf;
use std::process::ExitCode;

use chrono::Utc;
use clap::{Parser, Subcommand};
use coda_swarm::mcp::{RosterOnlySpawner, Server};
use coda_swarm::{
    FileStore, Member, MemberName, Policy, Role, RosterError, Store, SwarmId, UpdateError,
};

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

fn run(cli: Cli) -> Result<(), Error> {
    match cli.command {
        Command::Mcp { dir, agent, create } => {
            let store = open(dir, &agent, create)?;
            let server = Server::new(store, RosterOnlySpawner, agent);
            server.serve(io::stdin().lock(), io::stdout().lock())?;
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
