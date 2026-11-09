//! The timeline collector's commands.
//!
//! ```text
//! harness-timeline watch [--repo PATH] [--team KEY]... [--ticket KEY]... [--local-every SECS]
//!                        [--remote-every SECS] [--pull-requests N] [--once]
//! harness-timeline run-act [--worktree PATH] [-- ACT_ARGS...]
//! ```
//!
//! `watch` prints one JSON line per changed source (see `event.rs`). `run-act`
//! runs local CI for a worktree's head and records it where `watch` reads it.

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use harness_timeline::key::TicketKey;
use harness_timeline::{Config, act, error::chain, watch};

const USAGE: &str = "usage: harness-timeline watch [--repo PATH] [--team KEY]... [--ticket KEY]... \
[--local-every SECS] [--remote-every SECS] [--pull-requests N] [--once]\n       \
harness-timeline run-act [--worktree PATH] [-- ACT_ARGS...]";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.split_first() {
        Some((command, rest)) if command == "watch" => run_watch(rest),
        Some((command, rest)) if command == "run-act" => run_act(rest),
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn usage(problem: &str) -> ExitCode {
    eprintln!("{problem}\n{USAGE}");
    ExitCode::from(2)
}

fn run_watch(args: &[String]) -> ExitCode {
    let mut config = Config {
        repo: PathBuf::from("."),
        teams: Vec::new(),
        tickets: Vec::new(),
        local_every: Duration::from_secs(3),
        remote_every: Duration::from_secs(60),
        pull_request_limit: 100,
    };
    let mut once = false;
    let mut args = args.iter();
    while let Some(flag) = args.next() {
        if flag == "--once" {
            once = true;
            continue;
        }
        let Some(value) = args.next() else {
            return usage(&format!("{flag} needs a value"));
        };
        let seconds = || value.parse().map(Duration::from_secs);
        match flag.as_str() {
            "--repo" => config.repo = PathBuf::from(value),
            "--team" => config.teams.push(value.to_ascii_uppercase()),
            "--ticket" => match TicketKey::parse(value) {
                Some(key) => config.tickets.push(key),
                None => return usage(&format!("not a ticket key: {value}")),
            },
            "--local-every" => match seconds() {
                Ok(every) => config.local_every = every,
                Err(error) => return usage(&format!("--local-every {value}: {error}")),
            },
            "--remote-every" => match seconds() {
                Ok(every) => config.remote_every = every,
                Err(error) => return usage(&format!("--remote-every {value}: {error}")),
            },
            "--pull-requests" => match value.parse() {
                Ok(limit) => config.pull_request_limit = limit,
                Err(error) => return usage(&format!("--pull-requests {value}: {error}")),
            },
            _ => return usage(&format!("unknown option {flag}")),
        }
    }
    if config.teams.is_empty() {
        return usage("give at least one --team, such as --team ENG");
    }
    let mut out = std::io::stdout().lock();
    match watch(&config, &mut out, once) {
        Ok(()) => ExitCode::SUCCESS,
        // The reader closed its end: the panel stopped listening.
        Err(error) if error.kind() == std::io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("writing the timeline: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run_act(args: &[String]) -> ExitCode {
    let (ours, act_args) = match args.iter().position(|arg| arg == "--") {
        Some(split) => (&args[..split], &args[split + 1..]),
        None => (args, &[][..]),
    };
    let mut worktree = PathBuf::from(".");
    let mut ours = ours.iter();
    while let Some(flag) = ours.next() {
        match (flag.as_str(), ours.next()) {
            ("--worktree", Some(path)) => worktree = PathBuf::from(path),
            _ => return usage(&format!("unknown or incomplete option {flag}")),
        }
    }
    match act::run(&worktree, act_args) {
        Ok(0) => ExitCode::SUCCESS,
        Ok(code) => ExitCode::from(u8::try_from(code).unwrap_or(1).max(1)),
        Err(error) => {
            eprintln!("{}", chain(&error));
            ExitCode::FAILURE
        }
    }
}
