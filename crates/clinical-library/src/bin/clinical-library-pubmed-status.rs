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
//! It runs through the shared night runner, which asks the platform about a
//! run holding the slot. This machine can't see other runs, so it answers
//! that their state is unknown, and the night waits. If an earlier run on
//! this machine was killed while holding the slot, `--previous-run-ended`
//! says so, and this run takes the slot over and records that run as
//! failed. Never pass it while another run may be alive.

use std::process::ExitCode;

use async_trait::async_trait;
use clinical_library::pubmed::{self, Eutils, Pacing, Pmid, PubmedStatus};
use clinical_library::{
    FailureDetail, HolderState, Night, OutcomeKind, Platform, RunId, SlotName, Store, run_night,
};

const SLOT: &str = "pubmed-status";

/// What this machine can say about a run holding the slot.
struct ThisMachine {
    previous_run_ended: bool,
}

#[async_trait]
impl Platform for ThisMachine {
    async fn holder_state(&self, _: RunId) -> HolderState {
        if self.previous_run_ended {
            HolderState::Ended
        } else {
            HolderState::Unknown
        }
    }
}

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
    let job = PubmedStatus::new(ncbi, pmids);
    let platform = ThisMachine { previous_run_ended };
    let night = run_night(&store, &platform, &slot, RunId::new(), &job)
        .await
        .map_err(|error| {
            format!(
                "the night couldn't run: {}",
                FailureDetail::of(&error).as_str()
            )
        })?;

    let outcome = match night {
        Night::Finished(outcome) => outcome,
        Night::Waited(wait) => {
            eprintln!("another run holds the {SLOT} slot ({wait:?}); this night deferred");
            return Ok(OutcomeKind::Deferred);
        }
        Night::Stopped(why) => {
            eprintln!("automatic attempts have stopped: {why}. Record a resolution to resume.");
            return Ok(OutcomeKind::Deferred);
        }
        Night::LostSlot => {
            eprintln!("another run took the {SLOT} slot; this run recorded nothing");
            return Ok(OutcomeKind::Failed);
        }
    };
    if let Some(report) = job.report() {
        println!(
            "{} done, {} set aside, {} not due",
            report.done.len(),
            report.set_aside.len(),
            report.not_due.len()
        );
        for (pmid, why) in &report.set_aside {
            println!("set aside {pmid}: {}", FailureDetail::of(why).as_str());
        }
    }
    if outcome.kind() == OutcomeKind::Failed {
        match store.failure_details(&slot).await {
            Ok(details) => {
                if let Some((code, detail)) = details.into_iter().next() {
                    eprintln!("the night failed ({code}): {detail}");
                }
            }
            Err(error) => eprintln!(
                "the night failed, and its words couldn't be read: {}",
                FailureDetail::of(&error).as_str()
            ),
        }
    }
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
