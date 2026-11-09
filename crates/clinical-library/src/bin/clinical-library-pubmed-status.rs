//! Refreshes PubMed's retraction and correction status for a list of PMIDs.
//!
//! ```sh
//! doppler run -- clinical-library-pubmed-status pmids.txt
//! ```
//!
//! The file lists one PMID per line; blank lines and lines starting with `#`
//! are skipped. It reads `CLINICAL_LIBRARY_DATABASE_URL`, `NCBI_EMAIL` and
//! `NCBI_API_KEY` (optional: without it NCBI allows 3 requests a second, not
//! 10). It takes the `pubmed-status` slot, records each article's result and
//! the night's one outcome, and exits 0 when the night succeeded, 2 when it
//! deferred and 1 when it failed or couldn't run.
//!
//! If an earlier run on this machine was killed while holding the slot,
//! `--previous-run-ended` says so, and this run takes the slot over and
//! records that run as failed. Never pass it while another run may be alive.

use std::process::ExitCode;

use clinical_library::pubmed::{self, Eutils, Pacing, Pmid};
use clinical_library::{Acquired, HolderState, NightOutcome, OutcomeKind, RunId, SlotName, Store};

const SLOT: &str = "pubmed-status";

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(kind) => match kind {
            OutcomeKind::Succeeded => ExitCode::SUCCESS,
            OutcomeKind::Deferred => ExitCode::from(2),
            OutcomeKind::Failed => ExitCode::FAILURE,
        },
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<OutcomeKind, String> {
    let mut path = None;
    let mut previous_run_ended = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--previous-run-ended" => previous_run_ended = true,
            _ if path.is_none() => path = Some(arg),
            _ => return Err(format!("unexpected argument {arg:?}")),
        }
    }
    let path =
        path.ok_or("usage: clinical-library-pubmed-status [--previous-run-ended] <pmids-file>")?;
    let pmids = read_pmids(&path)?;

    let database_url = required("CLINICAL_LIBRARY_DATABASE_URL")?;
    let email = required("NCBI_EMAIL")?;
    let api_key = std::env::var("NCBI_API_KEY")
        .ok()
        .filter(|key| !key.is_empty());
    if api_key.is_none() {
        eprintln!("NCBI_API_KEY isn't set; NCBI allows 3 requests a second without it");
    }
    let ncbi = Eutils::new(
        pubmed::client::EUTILS,
        &email,
        api_key.as_deref(),
        Pacing::ncbi(api_key.is_some()),
    )
    .map_err(|error| format!("couldn't set up the NCBI client: {error}"))?;

    let store = Store::connect(&database_url)
        .await
        .map_err(|error| format!("couldn't reach the clinical library database: {error}"))?;
    let slot = SlotName::new(SLOT).map_err(|error| error.to_string())?;
    let run = RunId::new();
    let holder_state = if previous_run_ended {
        store
            .holder(&slot)
            .await
            .map_err(|error| format!("couldn't read the slot: {error}"))?
            .map(|holder| (holder, HolderState::Ended))
    } else {
        None
    };
    let lease = match store
        .acquire(&slot, run, holder_state)
        .await
        .map_err(|error| format!("couldn't take the slot: {error}"))?
    {
        Acquired::Holds(lease) => lease,
        Acquired::Waits(waiting) => {
            let reason = waiting.reason();
            store
                .record_wait(waiting)
                .await
                .map_err(|error| format!("couldn't record the wait: {error}"))?;
            eprintln!(
                "another run holds the {SLOT} slot ({}); this night deferred",
                reason.as_str()
            );
            return Ok(OutcomeKind::Deferred);
        }
    };

    let outcome = match pubmed::refresh(&store, &lease, &ncbi, &pmids).await {
        Ok(report) => {
            println!(
                "{} done, {} set aside, {} not due",
                report.done.len(),
                report.set_aside.len(),
                report.not_due.len()
            );
            for (pmid, why) in &report.set_aside {
                println!("set aside {pmid}: {}", chain(why));
            }
            report.outcome()
        }
        Err(error) => {
            eprintln!("the refresh stopped: {}", chain(&error));
            match error.code() {
                Some(code) => NightOutcome::failed(code),
                // The slot is another run's now; its night isn't ours to record.
                None => return Ok(OutcomeKind::Failed),
            }
        }
    };
    store
        .finish(lease, &outcome)
        .await
        .map_err(|error| format!("couldn't record the night: {error}"))?;
    println!("night {}", outcome.kind().as_str());
    Ok(outcome.kind())
}

fn required(name: &str) -> Result<String, String> {
    std::env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| format!("{name} is required"))
}

fn read_pmids(path: &str) -> Result<Vec<Pmid>, String> {
    let text =
        std::fs::read_to_string(path).map_err(|error| format!("couldn't read {path}: {error}"))?;
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| Pmid::parse(line).map_err(|error| format!("{path}: {error}")))
        .collect()
}

/// An error and each of its causes, for a person reading the terminal.
fn chain(error: &dyn std::error::Error) -> String {
    let mut words = error.to_string();
    let mut cause = error.source();
    while let Some(next) = cause {
        words.push_str(": ");
        words.push_str(&next.to_string());
        cause = next.source();
    }
    words
}
