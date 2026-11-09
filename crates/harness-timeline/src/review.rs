//! Reads independent review rounds from `target/verification/review/<branch>/<round>/`.
//!
//! The local-review and review-pr references put each round there: `request.md`,
//! the CLI's `stdout.log`, `review.md`, and `head.txt` holding the revision the
//! reviewer was given. A review-pr round also has the launcher's `run.json`.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::{FileError, read_json};
use crate::event::RunState;
use crate::key::Revision;

/// The folder holding a branch's review rounds.
pub fn review_dir(worktree: &Path, slug: &str) -> PathBuf {
    worktree.join("target/verification/review").join(slug)
}

/// One review round.
#[derive(Debug, PartialEq)]
pub struct Round {
    pub name: String,
    pub engine: Option<String>,
    pub model: Option<String>,
    pub session: Option<String>,
    pub state: RunState,
    pub head: Option<Revision>,
    pub report: PathBuf,
}

#[derive(Debug, Default, Deserialize)]
struct RunRecord {
    status: Option<String>,
    engine: Option<String>,
    model: Option<String>,
    session_id: Option<String>,
    resume_session_id: Option<String>,
    #[serde(default)]
    targets: Vec<RunTarget>,
}

#[derive(Debug, Deserialize)]
struct RunTarget {
    head_sha: Option<Revision>,
}

/// Every round under `dir`, in name order. A missing folder means no rounds yet.
pub fn read_rounds(dir: &Path) -> Result<Vec<Round>, FileError> {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(source) => {
            return Err(FileError::Read {
                path: dir.to_owned(),
                source,
            });
        }
    };
    let mut folders: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect();
    folders.sort();
    folders.iter().map(|folder| read_round(folder)).collect()
}

fn read_round(folder: &Path) -> Result<Round, FileError> {
    let run_path = folder.join("run.json");
    let run: RunRecord = if run_path.is_file() {
        read_json(&run_path)?
    } else {
        RunRecord::default()
    };
    let head = match std::fs::read_to_string(folder.join("head.txt")) {
        Ok(text) => Revision::parse(&text),
        Err(_) => run
            .targets
            .first()
            .and_then(|target| target.head_sha.clone()),
    };
    let events = CodexEvents::read(&folder.join("stdout.log"))?;
    let report = folder.join("review.md");
    let state = match run.status.as_deref() {
        Some("completed") => RunState::Passed,
        Some(
            "failed"
            | "stale"
            | "access_unverified"
            | "freshness_unverified"
            | "publication_unverified",
        ) => RunState::Failed,
        _ => events.state(report.is_file(), folder),
    };
    Ok(Round {
        name: folder
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default(),
        engine: run
            .engine
            .or_else(|| events.thread.as_ref().map(|_| "codex".to_owned())),
        model: run.model,
        session: run.session_id.or(run.resume_session_id).or(events.thread),
        state,
        head,
        report,
    })
}

/// What a Codex `--json` event stream says about its turn.
#[derive(Debug, Default)]
struct CodexEvents {
    thread: Option<String>,
    completed: bool,
    failed: bool,
    present: bool,
}

#[derive(Deserialize)]
struct CodexEvent {
    #[serde(rename = "type")]
    kind: String,
    thread_id: Option<String>,
}

impl CodexEvents {
    fn read(path: &Path) -> Result<Self, FileError> {
        let text = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::default());
            }
            Err(source) => {
                return Err(FileError::Read {
                    path: path.to_owned(),
                    source,
                });
            }
        };
        let mut events = Self {
            present: true,
            ..Self::default()
        };
        // Lines that are not events (CLI banners) are not part of the turn.
        for event in text
            .lines()
            .filter_map(|line| serde_json::from_str::<CodexEvent>(line).ok())
        {
            match event.kind.as_str() {
                "thread.started" => events.thread = event.thread_id,
                "turn.completed" => events.completed = true,
                "turn.failed" | "error" => events.failed = true,
                _ => {}
            }
        }
        Ok(events)
    }

    /// The local-review reference accepts a round only with `turn.completed`, no
    /// failed turn and a report. A stream that stopped without either is still
    /// running only while it keeps growing.
    fn state(&self, has_report: bool, folder: &Path) -> RunState {
        if self.failed {
            return RunState::Failed;
        }
        if self.completed {
            return if has_report {
                RunState::Passed
            } else {
                RunState::Failed
            };
        }
        if !self.present {
            return if has_report {
                RunState::Passed
            } else {
                RunState::Interrupted
            };
        }
        if crate::recently_written(&folder.join("stdout.log")) {
            RunState::Running
        } else {
            RunState::Interrupted
        }
    }
}
