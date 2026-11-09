//! Reads capture-proof's manifest and the latest scenario receipt in a worktree.
//!
//! The manifest is written by `.agents/skills/capture-proof/scripts/proof.py`; the
//! stale rule here is that script's `stale`, so the panel and `proof.py status`
//! agree.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::{FileError, read_json};
use crate::event::{ProofState, ScenarioOutcome};
use crate::key::Revision;

#[derive(Debug, Deserialize)]
struct Manifest {
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    claim: String,
    #[serde(default)]
    label: String,
    kind: String,
    revision: Option<Revision>,
    #[serde(default)]
    dirty: bool,
    #[serde(default)]
    ok: bool,
    #[serde(default)]
    files: Vec<String>,
    superseded: Option<String>,
    kept: Option<Kept>,
}

#[derive(Debug, Deserialize)]
struct Kept {
    revision: Revision,
}

/// One current entry, judged against the worktree's head.
#[derive(Debug, PartialEq)]
pub struct Criterion {
    pub claim: String,
    pub label: String,
    pub state: ProofState,
    pub revision: Option<Revision>,
    pub artifact: Option<PathBuf>,
}

/// The manifest a worktree's branch writes, when there is one.
pub fn manifest_path(worktree: &Path, slug: &str) -> PathBuf {
    worktree
        .join("target/verification/proof")
        .join(slug)
        .join("manifest.json")
}

/// The current (not superseded) entries of a manifest, as `proof.py status` judges them.
pub fn read_manifest(path: &Path, head: &Revision) -> Result<Vec<Criterion>, FileError> {
    let manifest: Manifest = read_json(path)?;
    let bundle = path.parent().unwrap_or(path);
    Ok(manifest
        .entries
        .into_iter()
        .filter(|entry| entry.superseded.is_none())
        .map(|entry| {
            let state = judge(&entry, head);
            Criterion {
                artifact: entry.files.first().map(|file| bundle.join(file)),
                claim: entry.claim,
                label: entry.label,
                state,
                revision: entry.revision,
            }
        })
        .collect())
}

fn judge(entry: &Entry, head: &Revision) -> ProofState {
    if entry.kind == "gap" {
        return ProofState::Gap;
    }
    if !entry.ok {
        return ProofState::Failed;
    }
    if entry
        .kept
        .as_ref()
        .is_some_and(|kept| &kept.revision == head)
    {
        return ProofState::Passed;
    }
    match &entry.revision {
        None => ProofState::RevisionUnknown,
        Some(revision) if revision != head || entry.dirty => ProofState::Stale,
        Some(_) => ProofState::Passed,
    }
}

#[derive(Debug, Deserialize)]
struct Receipt {
    outcome: Option<ScenarioOutcome>,
    #[serde(default)]
    sources: ReceiptSources,
    #[serde(default)]
    missing: Vec<serde_json::Value>,
}

#[derive(Debug, Default, Deserialize)]
struct ReceiptSources {
    git_sha: Option<Revision>,
}

/// The newest scenario receipt's result.
#[derive(Debug, PartialEq)]
pub struct Scenario {
    pub outcome: ScenarioOutcome,
    pub revision: Option<Revision>,
    pub missing: u32,
    pub receipt: PathBuf,
}

/// The newest `target/verification/runs/scenarios-*/receipt.json`, or `None` when
/// the worktree has never run the scenarios. Run folders are named for the time
/// they started, so the last name is the newest. A receipt without an outcome is a
/// run still in progress and reads as not finished rather than complete.
pub fn latest_scenario(worktree: &Path) -> Result<Option<Scenario>, FileError> {
    let runs = worktree.join("target/verification/runs");
    let entries = match std::fs::read_dir(&runs) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(source) => return Err(FileError::Read { path: runs, source }),
    };
    let mut names: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("scenarios-"))
                && path.join("receipt.json").is_file()
        })
        .collect();
    names.sort();
    let Some(latest) = names.pop() else {
        return Ok(None);
    };
    let path = latest.join("receipt.json");
    let receipt: Receipt = read_json(&path)?;
    let Some(outcome) = receipt.outcome else {
        return Ok(None);
    };
    Ok(Some(Scenario {
        outcome,
        revision: receipt.sources.git_sha,
        missing: u32::try_from(receipt.missing.len()).unwrap_or(u32::MAX),
        receipt: path,
    }))
}
