//! What the collector says, one JSON line per [`SourceUpdate`].
//!
//! Each line carries every item one source currently knows. A reader replaces
//! that source's earlier items with the new ones, so items that disappear (a
//! removed worktree, a merged PR leaving the list) disappear from the reader too,
//! and a reader can start at any time.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::key::{Revision, TicketKey};

/// The places the collector reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    /// `git worktree list` for the repository.
    Worktrees,
    /// Each worktree's capture-proof manifest.
    Proof,
    /// Each worktree's latest scenario receipt.
    Scenarios,
    /// Each worktree's independent review rounds.
    Review,
    /// Each worktree's latest local `act` run.
    LocalCi,
    /// Linear tickets, their parents and their sub-tickets.
    Linear,
    /// GitHub pull requests.
    GitHub,
}

/// One line of output: everything `source` currently knows.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceUpdate {
    /// The source these events replace.
    pub source: SourceKind,
    /// Its current items, including a [`TimelineEvent::SourceError`] for any part it could not read.
    pub events: Vec<TimelineEvent>,
}

/// One item on the timeline. Local items name the branch they came from; the
/// collector fills in the ticket the branch belongs to.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TimelineEvent {
    /// A Linear ticket. Sub-tickets name their parent.
    Ticket {
        key: TicketKey,
        parent: Option<TicketKey>,
        title: String,
        status: TicketStatus,
        assignee: Option<String>,
        url: String,
    },
    /// A worktree with a branch checked out.
    Worktree {
        ticket: Option<TicketKey>,
        branch: String,
        path: PathBuf,
        head: Revision,
        /// Tracked files differ from `head`.
        dirty: bool,
        /// Commits on `head` that `origin/main` lacks.
        ahead_of_main: u32,
    },
    /// One current capture-proof entry.
    Criterion {
        ticket: Option<TicketKey>,
        branch: String,
        claim: String,
        label: String,
        state: ProofState,
        revision: Option<Revision>,
        /// The first artifact the entry recorded, when it recorded one.
        artifact: Option<PathBuf>,
    },
    /// The latest scenario receipt.
    Scenario {
        ticket: Option<TicketKey>,
        branch: String,
        outcome: ScenarioOutcome,
        revision: Option<Revision>,
        /// Capabilities the journeys needed that the run did not exercise.
        missing: u32,
        receipt: PathBuf,
    },
    /// One independent review round.
    Review {
        ticket: Option<TicketKey>,
        branch: String,
        round: String,
        engine: Option<String>,
        model: Option<String>,
        session: Option<String>,
        state: RunState,
        /// The revision the reviewer was given.
        head: Option<Revision>,
        /// `head` is the worktree's current head.
        at_head: bool,
        report: PathBuf,
    },
    /// One job of the latest local `act` run.
    LocalCi {
        ticket: Option<TicketKey>,
        branch: String,
        run: String,
        job: String,
        /// The step running now, or the last one that ran.
        step: Option<String>,
        state: RunState,
        head: Revision,
        at_head: bool,
        seconds: Option<u64>,
        log: PathBuf,
    },
    /// A GitHub pull request.
    PullRequest {
        ticket: Option<TicketKey>,
        branch: String,
        number: u64,
        title: String,
        state: PrState,
        checks: CheckRollup,
        review: Option<String>,
        head: Revision,
        url: String,
    },
    /// Part of a source that could not be read. It never reads as an empty lane.
    SourceError {
        source: SourceKind,
        /// The worktree branch the failure belongs to, when it belongs to one.
        branch: Option<String>,
        message: String,
    },
}

impl TimelineEvent {
    /// The branch a local item came from.
    pub fn branch(&self) -> Option<&str> {
        match self {
            Self::Worktree { branch, .. }
            | Self::Criterion { branch, .. }
            | Self::Scenario { branch, .. }
            | Self::Review { branch, .. }
            | Self::LocalCi { branch, .. }
            | Self::PullRequest { branch, .. } => Some(branch),
            Self::SourceError { branch, .. } => branch.as_deref(),
            Self::Ticket { .. } => None,
        }
    }

    /// The ticket slot the collector fills in, for items that have one.
    pub fn ticket_mut(&mut self) -> Option<&mut Option<TicketKey>> {
        match self {
            Self::Worktree { ticket, .. }
            | Self::Criterion { ticket, .. }
            | Self::Scenario { ticket, .. }
            | Self::Review { ticket, .. }
            | Self::LocalCi { ticket, .. }
            | Self::PullRequest { ticket, .. } => Some(ticket),
            Self::Ticket { .. } | Self::SourceError { .. } => None,
        }
    }
}

/// A Linear workflow state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TicketStatus {
    pub name: String,
    pub category: StatusCategory,
}

/// Linear's fixed state types, which every team's named states belong to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusCategory {
    Triage,
    Backlog,
    Unstarted,
    Started,
    Completed,
    Canceled,
}

/// Where a proof entry stands against the worktree's head.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofState {
    Passed,
    Failed,
    /// Ran against another revision, or with uncommitted changes, and not kept.
    Stale,
    /// Added without the revision it was made from.
    RevisionUnknown,
    /// A criterion recorded as having no proof.
    Gap,
}

/// A scenario receipt's outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScenarioOutcome {
    Complete,
    Incomplete,
    Failed,
}

/// A review round or CI job.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunState {
    Running,
    Passed,
    Failed,
    /// Stopped without a result: the process is gone, or nothing has arrived for too long.
    Interrupted,
}

/// A pull request's place in its life.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrState {
    Draft,
    Open,
    Merged,
    Closed,
}

/// Hosted checks on a pull request's head, taken together.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckRollup {
    None,
    Pending,
    Passed,
    Failed,
}
