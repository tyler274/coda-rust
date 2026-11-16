//! The command line's shape: each subcommand and its typed arguments.

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use coda_sandbox::broker::Item;
use coda_sandbox::broker::client::Want;
use coda_sandbox::config;
use coda_sandbox::credential::Operation;
use coda_sandbox::editor_settings::Editor;
use coda_sandbox::ids::{JobName, Slug};
use coda_sandbox::worktree::gc::DEFAULT_IDLE_DAYS;
use coda_sandbox::worktree::new::DEFAULT_FROM;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    /// The sandbox config file.
    #[arg(long, global = true, env = "CODA_SANDBOX_CONFIG", default_value = config::DEFAULT_PATH)]
    pub config: PathBuf,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Check the sandbox on this machine and name the fix for each problem.
    /// Exits 1 when any check fails, else 2 when any check could not run.
    Doctor,
    /// Show the container, build root and CI state.
    Status {
        /// Print the versioned JSON schema instead of text.
        #[arg(long)]
        json: bool,
    },
    /// Worktrees under .worktrees/<slug> whose target/ is a reflink clone of
    /// a seed of third-party build output. They belong to the checkout
    /// [container] repo names; run from any other repository, they refuse.
    #[command(subcommand)]
    Worktree(Worktree),
    /// The Firecracker CI.
    #[command(subcommand)]
    Ci(Ci),
    /// On the host: record the named worktrees, or every one, as
    /// refs/carry/<slug>/{head,dirty} and bundle them into the carry
    /// directory with their branch names. Runs as the checkout's owner,
    /// never as root. `carry import` in the container makes them.
    #[command(args_conflicts_with_subcommands = true)]
    Carry {
        #[command(subcommand)]
        command: Option<Carry>,
        /// Worktrees by directory name, as the summary lists them.
        slugs: Vec<String>,
    },
    /// Run one Coda CI pass, or build a fresh seed with `seed`. Run by the
    /// coda-ci units, configured by their CODA_CI_* environment.
    CiController {
        #[arg(value_enum)]
        mode: Option<ControllerMode>,
    },
    /// The token broker on the host: it serves the container's tokens from
    /// your keyring over [broker] socket.
    #[command(subcommand)]
    Broker(Broker),
    /// git's credential helper in the container, answered by the broker.
    Credential {
        #[arg(value_enum)]
        operation: Operation,
    },
    /// A shell in a container checkout over SSH, with nothing forwarded: a
    /// login shell, or the command after `--` run in the checkout's dev
    /// shell, such as `coda-sandbox shell eng-7 -- claude`.
    Shell {
        /// The worktree; the main checkout without one.
        slug: Option<Slug>,
        #[arg(last = true)]
        command: Vec<String>,
    },
    /// Open a container checkout in an editor on this machine, over its
    /// SSH remote.
    Open {
        /// The worktree; the main checkout without one.
        slug: Option<Slug>,
        /// The editor; the config's `[open] editor` without it, else Cursor.
        #[arg(long, value_enum)]
        editor: Option<Editor>,
    },
    /// In the container: ask the model proxy whether it takes the broker's
    /// model-proxy key, and print what it said as JSON. `doctor` on the
    /// host runs this over SSH.
    ProxyCheck,
    /// Print a token from the broker, for the container's gh and doppler
    /// wrappers and the agent CLIs' key commands.
    Token {
        #[arg(value_enum)]
        want: Want,
        /// Print even when standard output is a terminal.
        #[arg(long)]
        force: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum Carry {
    /// In the container: make each carried worktree, or the named ones,
    /// from its carried commit on its host branch, with its uncommitted
    /// files back and tracking origin's branch when there is one. A
    /// worktree already there is left alone.
    Import {
        /// Carried worktrees by the summary's names.
        slugs: Vec<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum Broker {
    /// Serve the socket; run by the coda-sandbox-broker user service.
    Serve,
    /// Store a token in your keyring, read from standard input without echo
    /// and never from the command line, then reload the running broker.
    Store {
        #[arg(value_enum)]
        item: Item,
    },
    /// Have the service that issues a token mint a new one, store it in
    /// your keyring without showing it, and reload the running broker.
    Mint {
        #[arg(value_enum)]
        item: Mintable,
    },
    /// Remove a token from your keyring and reload the running broker.
    Delete {
        #[arg(value_enum)]
        item: Item,
    },
    /// Store, read back and delete a throwaway keyring item.
    SelfTest,
}

/// The items a local service can mint.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum Mintable {
    /// A client key from the model proxy's loopback admin route, labelled
    /// with the container's name. The key it replaces is revoked.
    ModelProxy,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum ControllerMode {
    Seed,
}

#[derive(Debug, Subcommand)]
pub enum Worktree {
    /// A worktree plus a seeded target; rerunning finishes a partial one.
    New {
        slug: Slug,
        /// The ref a new branch starts from.
        #[arg(long, default_value = DEFAULT_FROM)]
        from: String,
    },
    /// Build a seed from <ref> with dev/session-seed.sh and make it current.
    Seed {
        #[arg(value_name = "REF")]
        reference: Option<String>,
    },
    /// Replace a worktree's target with a fresh clone of the seed.
    Reset { slug: Slug },
    /// Reset idle targets to a seed clone and remove orphaned ones.
    Gc {
        /// Days without a build before a target is reset.
        #[arg(long, value_name = "DAYS", default_value_t = DEFAULT_IDLE_DAYS)]
        idle: u32,
    },
    /// Remove a worktree and its target; the branch is kept.
    Rm { slug: Slug },
    /// Remove every clean worktree merged into origin/main.
    Prune {
        /// Remove without asking.
        #[arg(long)]
        yes: bool,
    },
    /// Worktrees, targets and disk use.
    Status,
}

#[derive(Debug, Subcommand)]
pub enum Ci {
    /// Queue one CI job of a commit through the spool, print its log, and
    /// exit with its status. Uncommitted changes are not sent. Exits 124
    /// when no result arrives in time and 2 when there is no spool.
    Run {
        /// A repository path (its HEAD runs) or a commit in the current
        /// repository; defaults to the current directory.
        #[arg(value_name = "REPO_PATH_OR_COMMIT")]
        target: Option<String>,
        /// The workflow job to run with `act -j`.
        #[arg(long, default_value = "ci")]
        job: JobName,
    },
}
