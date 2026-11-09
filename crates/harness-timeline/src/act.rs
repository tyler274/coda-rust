//! Local CI through `act`: starting a run and reading it back.
//!
//! `act` copies the working directory into its container, and a linked worktree's
//! `.git` file points outside that copy, so a job that compares revisions (the CI
//! scope check) sees no history there. A run therefore starts from a standalone
//! local clone of the worktree's head, with `origin/main` fetched beside it, and
//! a `pull_request` event comparing the two. Uncommitted changes are not in the
//! clone; the run records the revision it tested.
//!
//! Each run lives in `target/verification/act/<branch>/<run>/` of the worktree:
//! `run.json` (the [`RunRecord`]), `events.jsonl` (act's `--json` output) and
//! `checkout/` (the clone, removed when the run ends).

use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::error::{CommandError, FileError, read_json};
use crate::event::RunState;
use crate::git::{branch_slug, git};
use crate::key::Revision;

/// The folder holding a branch's local CI runs.
pub fn act_dir(worktree: &Path, slug: &str) -> PathBuf {
    worktree.join("target/verification/act").join(slug)
}

/// What `run.json` says about a run.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunRecord {
    pub head: Revision,
    pub base: Revision,
    /// Tracked files differed from `head` when the run started; the run did not test them.
    pub dirty: bool,
    pub command: Vec<String>,
    pub pid: Option<u32>,
    /// `act`'s exit status once it has ended.
    pub exit: Option<i32>,
}

/// Why a local CI run could not start or finish.
#[derive(Debug, Error)]
pub enum ActError {
    #[error("reading the worktree's Git state")]
    Git(#[from] CommandError),
    #[error("writing {path}")]
    Write {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("starting act")]
    Start(#[source] std::io::Error),
}

fn write(path: &Path, contents: &[u8]) -> Result<(), ActError> {
    std::fs::write(path, contents).map_err(|source| ActError::Write {
        path: path.to_owned(),
        source,
    })
}

fn save(path: &Path, record: &RunRecord) -> Result<(), ActError> {
    let mut text = serde_json::to_vec_pretty(record).expect("run records serialize");
    text.push(b'\n');
    write(path, &text)
}

/// Runs `act` for the worktree's head and returns act's exit status. `act_args`
/// are passed through, so `-j ci` or `--pull=false` work as on the command line;
/// with none, the repository's `.actrc` chooses the job.
pub fn run(worktree: &Path, act_args: &[String]) -> Result<i32, ActError> {
    // act runs inside the clone, so every path it is given must be absolute.
    let worktree = &git(worktree, &["rev-parse", "--show-toplevel"]).map(PathBuf::from)?;
    let branch = git(worktree, &["rev-parse", "--abbrev-ref", "HEAD"])?;
    let head_text = git(worktree, &["rev-parse", "HEAD"])?;
    let base_text = git(worktree, &["merge-base", "origin/main", "HEAD"])?;
    let unreadable = |text: String| CommandError::Unreadable {
        command: "git rev-parse".to_owned(),
        output: text.clone(),
        source: format!("not a full revision: {text}").into(),
    };
    let head = Revision::parse(&head_text).ok_or_else(|| unreadable(head_text.clone()))?;
    let base = Revision::parse(&base_text).ok_or_else(|| unreadable(base_text.clone()))?;
    let dirty = crate::git::dirty(worktree)?;

    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .unwrap_or_default();
    let dir =
        act_dir(worktree, &branch_slug(&branch)).join(format!("{stamp}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).map_err(|source| ActError::Write {
        path: dir.clone(),
        source,
    })?;

    let checkout = dir.join("checkout");
    let checkout_text = checkout.to_string_lossy().into_owned();
    git(
        worktree,
        &[
            "clone",
            "--quiet",
            "--local",
            "--no-checkout",
            ".",
            &checkout_text,
        ],
    )?;
    git(
        &checkout,
        &["checkout", "--quiet", "--detach", head.as_str()],
    )?;
    git(
        &checkout,
        &["update-ref", "refs/remotes/origin/main", base.as_str()],
    )?;

    let event = serde_json::json!({
        "pull_request": {
            "base": { "sha": base.as_str(), "ref": "main" },
            "head": { "sha": head.as_str(), "ref": branch },
        }
    });
    let event_path = dir.join("event.json");
    write(&event_path, event.to_string().as_bytes())?;

    let mut command = vec![
        "act".to_owned(),
        "pull_request".to_owned(),
        "--json".to_owned(),
        "--rm".to_owned(),
        "-e".to_owned(),
        event_path.to_string_lossy().into_owned(),
    ];
    command.extend(act_args.iter().cloned());
    let record_path = dir.join("run.json");
    let mut record = RunRecord {
        head,
        base,
        dirty,
        command: command.clone(),
        pid: None,
        exit: None,
    };
    save(&record_path, &record)?;

    let events_path = dir.join("events.jsonl");
    let events = File::create(&events_path).map_err(|source| ActError::Write {
        path: events_path.clone(),
        source,
    })?;
    let stderr = events.try_clone().map_err(|source| ActError::Write {
        path: events_path.clone(),
        source,
    })?;
    eprintln!("local CI run: {}", dir.display());
    let mut child = Command::new(&command[0])
        .args(&command[1..])
        .current_dir(&checkout)
        .stdin(Stdio::null())
        .stdout(events)
        .stderr(stderr)
        .spawn()
        .map_err(ActError::Start)?;
    record.pid = Some(child.id());
    save(&record_path, &record)?;
    let status = child.wait().map_err(ActError::Start)?;
    let exit = status.code().unwrap_or(-1);
    record.exit = Some(exit);
    save(&record_path, &record)?;
    if let Err(error) = std::fs::remove_dir_all(&checkout) {
        eprintln!("could not remove {}: {error}", checkout.display());
    }
    Ok(exit)
}

/// One job as the latest run reports it.
#[derive(Debug, PartialEq)]
pub struct Job {
    pub run: String,
    pub id: String,
    pub name: String,
    pub step: Option<String>,
    pub state: RunState,
    pub head: Revision,
    pub seconds: Option<u64>,
    pub log: PathBuf,
}

#[derive(Deserialize)]
struct ActLine {
    #[serde(rename = "jobID")]
    job_id: Option<String>,
    job: Option<String>,
    step: Option<String>,
    #[serde(rename = "jobResult")]
    job_result: Option<String>,
    time: Option<String>,
}

/// The jobs of the newest run in `dir`, or none when the branch has no run.
/// Run folders start with the second they began, so the last name is the newest.
pub fn latest_jobs(dir: &Path) -> Result<Vec<Job>, FileError> {
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
    let mut runs: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.join("run.json").is_file())
        .collect();
    runs.sort_by_key(|path| run_order(path));
    let Some(latest) = runs.pop() else {
        return Ok(Vec::new());
    };
    let record: RunRecord = read_json(&latest.join("run.json"))?;
    let alive = record.exit.is_none() && record.pid.is_some_and(process_alive);
    read_jobs(&latest, &record, alive)
}

fn run_order(path: &Path) -> (u64, String) {
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    let stamp = name
        .split('-')
        .next()
        .and_then(|stamp| stamp.parse().ok())
        .unwrap_or(0);
    (stamp, name)
}

/// Reads a run's events. `alive` says whether act is still running, which decides
/// whether a job without a result is running or was interrupted.
pub fn read_jobs(run_dir: &Path, record: &RunRecord, alive: bool) -> Result<Vec<Job>, FileError> {
    let log = run_dir.join("events.jsonl");
    let text = match std::fs::read_to_string(&log) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(source) => return Err(FileError::Read { path: log, source }),
    };
    let run = run_dir
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    struct Progress {
        name: String,
        step: Option<String>,
        result: Option<String>,
        first: Option<i64>,
        last: Option<i64>,
    }
    let mut jobs: Vec<(String, Progress)> = Vec::new();
    // act also prints lines that belong to no job (daemon notices, upgrade hints).
    for line in text
        .lines()
        .filter_map(|line| serde_json::from_str::<ActLine>(line).ok())
    {
        let Some(id) = line.job_id else { continue };
        let index = match jobs.iter().position(|(known, _)| *known == id) {
            Some(index) => index,
            None => {
                jobs.push((
                    id.clone(),
                    Progress {
                        name: line.job.clone().unwrap_or_else(|| id.clone()),
                        step: None,
                        result: None,
                        first: None,
                        last: None,
                    },
                ));
                jobs.len() - 1
            }
        };
        let progress = &mut jobs[index].1;
        if let Some(step) = line.step {
            progress.step = Some(step);
        }
        if let Some(result) = line.job_result {
            progress.result = Some(result);
        }
        if let Some(seconds) = line.time.as_deref().and_then(epoch_seconds) {
            progress.first.get_or_insert(seconds);
            progress.last = Some(seconds);
        }
    }
    Ok(jobs
        .into_iter()
        .map(|(id, progress)| Job {
            run: run.clone(),
            id,
            name: progress.name,
            step: progress.step,
            state: match progress.result.as_deref() {
                Some("success") => RunState::Passed,
                Some(_) => RunState::Failed,
                None if alive => RunState::Running,
                None => RunState::Interrupted,
            },
            head: record.head.clone(),
            seconds: progress
                .first
                .zip(progress.last)
                .and_then(|(first, last)| u64::try_from(last - first).ok()),
            log: log.clone(),
        })
        .collect())
}

fn process_alive(pid: u32) -> bool {
    Command::new("kill")
        .args(["-0", &pid.to_string()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

/// Seconds since the epoch for act's RFC 3339 times, such as `2026-11-09T13:31:00-07:00`.
fn epoch_seconds(text: &str) -> Option<i64> {
    let (date, rest) = text.split_once('T')?;
    let mut date = date.splitn(3, '-').map(str::parse::<i64>);
    let (year, month, day) = (date.next()?.ok()?, date.next()?.ok()?, date.next()?.ok()?);
    let (clock, offset) = if let Some(clock) = rest.strip_suffix('Z') {
        (clock, 0)
    } else {
        let split = rest.rfind(['+', '-'])?;
        let (clock, zone) = rest.split_at(split);
        let sign = if zone.starts_with('-') { -1 } else { 1 };
        let (hours, minutes) = zone[1..].split_once(':')?;
        (
            clock,
            sign * (hours.parse::<i64>().ok()? * 3600 + minutes.parse::<i64>().ok()? * 60),
        )
    };
    let mut clock = clock.split(':');
    let hour: i64 = clock.next()?.parse().ok()?;
    let minute: i64 = clock.next()?.parse().ok()?;
    let second: f64 = clock.next()?.parse().ok()?;
    // Days from the civil date (Howard Hinnant's algorithm).
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let day_of_year = (153 * ((month + 9) % 12) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    let days = era * 146_097 + day_of_era - 719_468;
    #[allow(
        clippy::cast_possible_truncation,
        reason = "whole seconds are enough for durations"
    )]
    let second = second as i64;
    Some(days * 86_400 + hour * 3600 + minute * 60 + second - offset)
}

#[cfg(test)]
mod tests {
    use super::epoch_seconds;

    #[test]
    fn act_times_read_as_epoch_seconds() {
        assert_eq!(epoch_seconds("1970-01-01T00:00:00Z"), Some(0));
        assert_eq!(
            epoch_seconds("2026-11-09T13:31:00-07:00"),
            Some(1_794_256_260)
        );
        assert_eq!(
            epoch_seconds("2026-11-09T13:31:01-07:00")
                .zip(epoch_seconds("2026-11-09T13:30:56-07:00"))
                .map(|(a, b)| a - b),
            Some(5)
        );
    }
}
