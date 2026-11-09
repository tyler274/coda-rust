//! Collects where each ticket's work stands, for the developer's timeline panel.
//!
//! The collector reads the repository's worktrees, each worktree's proof,
//! scenario receipts, review rounds and local `act` runs, Linear tickets with
//! their sub-tickets, and GitHub pull requests. It prints one [`SourceUpdate`]
//! line of JSON whenever a source's items change. The panel that draws them
//! knows this schema and nothing about where the items come from.
//!
//! This is a developer tool. Neither product depends on it.

pub mod act;
pub mod error;
pub mod event;
pub mod git;
pub mod github;
pub mod key;
pub mod linear;
pub mod proof;
pub mod review;

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};

use error::chain;
use event::{SourceKind, SourceUpdate, TimelineEvent};
use git::{Worktree, branch_slug};
use key::TicketKey;

/// What to watch and how often.
#[derive(Clone, Debug)]
pub struct Config {
    /// Any checkout of the repository; its worktrees are found from here.
    pub repo: PathBuf,
    /// Linear team keys, which decide what in a branch name is a ticket key.
    pub teams: Vec<String>,
    /// Tickets to show even with no worktree, such as a parent being fanned out.
    pub tickets: Vec<TicketKey>,
    /// How often local files are read again.
    pub local_every: Duration,
    /// How often Linear and GitHub are asked again.
    pub remote_every: Duration,
    /// How many recent pull requests to ask GitHub for.
    pub pull_request_limit: u32,
}

/// A file changed within the last two minutes, so a run writing it still counts as running.
pub(crate) fn recently_written(path: &Path) -> bool {
    std::fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()
        .and_then(|modified| SystemTime::now().duration_since(modified).ok())
        .is_some_and(|age| age < Duration::from_secs(120))
}

fn source_error(
    source: SourceKind,
    branch: Option<&str>,
    error: &(dyn std::error::Error + 'static),
) -> TimelineEvent {
    TimelineEvent::SourceError {
        source,
        branch: branch.map(str::to_owned),
        message: chain(error),
    }
}

/// The local sources' items for the given worktrees, before tickets are filled in.
pub fn local_updates(worktrees: &[Worktree]) -> Vec<SourceUpdate> {
    let mut trees = Vec::new();
    let mut proof = Vec::new();
    let mut scenarios = Vec::new();
    let mut reviews = Vec::new();
    let mut local_ci = Vec::new();
    for tree in worktrees {
        let branch = tree.branch.as_str();
        let slug = branch_slug(branch);

        match git::dirty(&tree.path).and_then(|dirty| Ok((dirty, git::ahead_of_main(&tree.path)?)))
        {
            Ok((dirty, ahead_of_main)) => trees.push(TimelineEvent::Worktree {
                ticket: None,
                branch: branch.to_owned(),
                path: tree.path.clone(),
                head: tree.head.clone(),
                dirty,
                ahead_of_main,
            }),
            Err(error) => trees.push(source_error(SourceKind::Worktrees, Some(branch), &error)),
        }

        let manifest = proof::manifest_path(&tree.path, &slug);
        if manifest.is_file() {
            match proof::read_manifest(&manifest, &tree.head) {
                Ok(criteria) => {
                    proof.extend(
                        criteria
                            .into_iter()
                            .map(|criterion| TimelineEvent::Criterion {
                                ticket: None,
                                branch: branch.to_owned(),
                                claim: criterion.claim,
                                label: criterion.label,
                                state: criterion.state,
                                revision: criterion.revision,
                                artifact: criterion.artifact,
                            }),
                    )
                }
                Err(error) => proof.push(source_error(SourceKind::Proof, Some(branch), &error)),
            }
        }

        match proof::latest_scenario(&tree.path) {
            Ok(Some(scenario)) => scenarios.push(TimelineEvent::Scenario {
                ticket: None,
                branch: branch.to_owned(),
                outcome: scenario.outcome,
                revision: scenario.revision,
                missing: scenario.missing,
                receipt: scenario.receipt,
            }),
            Ok(None) => {}
            Err(error) => scenarios.push(source_error(SourceKind::Scenarios, Some(branch), &error)),
        }

        match review::read_rounds(&review::review_dir(&tree.path, &slug)) {
            Ok(rounds) => reviews.extend(rounds.into_iter().map(|round| TimelineEvent::Review {
                ticket: None,
                branch: branch.to_owned(),
                at_head: round.head.as_ref() == Some(&tree.head),
                round: round.name,
                engine: round.engine,
                model: round.model,
                session: round.session,
                state: round.state,
                head: round.head,
                report: round.report,
            })),
            Err(error) => reviews.push(source_error(SourceKind::Review, Some(branch), &error)),
        }

        match act::latest_jobs(&act::act_dir(&tree.path, &slug)) {
            Ok(jobs) => local_ci.extend(jobs.into_iter().map(|job| TimelineEvent::LocalCi {
                ticket: None,
                branch: branch.to_owned(),
                at_head: job.head == tree.head,
                run: job.run,
                job: job.name,
                step: job.step,
                state: job.state,
                head: job.head,
                seconds: job.seconds,
                log: job.log,
            })),
            Err(error) => local_ci.push(source_error(SourceKind::LocalCi, Some(branch), &error)),
        }
    }
    vec![
        SourceUpdate {
            source: SourceKind::Worktrees,
            events: trees,
        },
        SourceUpdate {
            source: SourceKind::Proof,
            events: proof,
        },
        SourceUpdate {
            source: SourceKind::Scenarios,
            events: scenarios,
        },
        SourceUpdate {
            source: SourceKind::Review,
            events: reviews,
        },
        SourceUpdate {
            source: SourceKind::LocalCi,
            events: local_ci,
        },
    ]
}

/// Decides which ticket a branch or pull request belongs to.
#[derive(Debug, Default)]
pub struct TicketIndex {
    teams: Vec<String>,
    by_branch: BTreeMap<String, TicketKey>,
    by_pull_request: BTreeMap<String, TicketKey>,
}

impl TicketIndex {
    /// Builds the index from Linear's tickets. Linear's own branch name and pull
    /// request attachments come first; the key written in a branch name is the fallback.
    pub fn new(teams: &[String], tickets: &[linear::Ticket]) -> Self {
        let mut index = Self {
            teams: teams.to_vec(),
            ..Self::default()
        };
        for ticket in tickets {
            if let Some(branch) = &ticket.branch {
                index.by_branch.insert(branch.clone(), ticket.key.clone());
            }
            for url in &ticket.pull_requests {
                index
                    .by_pull_request
                    .insert(url.clone(), ticket.key.clone());
            }
        }
        index
    }

    /// The ticket a branch is for.
    pub fn branch(&self, branch: &str) -> Option<TicketKey> {
        self.by_branch
            .get(branch)
            .cloned()
            .or_else(|| TicketKey::in_branch(branch, &self.teams))
    }

    /// The ticket a pull request is for.
    pub fn pull_request(&self, url: &str, branch: &str) -> Option<TicketKey> {
        self.by_pull_request
            .get(url)
            .cloned()
            .or_else(|| self.branch(branch))
    }

    /// Fills in the ticket of each local item from its branch.
    pub fn fill(&self, events: &mut [TimelineEvent]) {
        for event in events {
            let ticket = event.branch().and_then(|branch| self.branch(branch));
            if let Some(slot) = event.ticket_mut() {
                *slot = ticket;
            }
        }
    }
}

/// The Linear and GitHub sources' items, with tickets filled in. Pull requests are
/// kept when they belong to a known ticket or a local worktree's branch.
pub fn remote_updates(config: &Config, worktrees: &[Worktree]) -> (TicketIndex, Vec<SourceUpdate>) {
    let pull_requests = github::list(&config.repo, config.pull_request_limit);

    let mut wanted: BTreeSet<TicketKey> = config.tickets.iter().cloned().collect();
    wanted.extend(
        worktrees
            .iter()
            .filter_map(|tree| TicketKey::in_branch(&tree.branch, &config.teams)),
    );
    let (mut tickets, mut errors) = linear::fetch(&wanted.iter().cloned().collect::<Vec<_>>());
    // A sub-ticket's answer names its parent but not its siblings; ask for each
    // parent too, so the fan-out shows every sub-ticket.
    let parents: Vec<TicketKey> = tickets
        .iter()
        .filter_map(|ticket| ticket.parent.clone())
        .filter(|parent| !wanted.contains(parent))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    if !parents.is_empty() {
        let (more, more_errors) = linear::fetch(&parents);
        for ticket in more {
            match tickets.iter_mut().find(|known| known.key == ticket.key) {
                Some(known) if ticket.parent.is_some() || known.parent.is_none() => *known = ticket,
                Some(_) => {}
                None => tickets.push(ticket),
            }
        }
        errors.extend(more_errors);
    }
    let index = TicketIndex::new(&config.teams, &tickets);

    let mut linear_events: Vec<TimelineEvent> = tickets.iter().map(linear::Ticket::event).collect();
    linear_events.extend(
        errors
            .iter()
            .map(|error| source_error(SourceKind::Linear, None, error)),
    );

    let known: BTreeSet<TicketKey> = tickets.iter().map(|ticket| ticket.key.clone()).collect();
    let local: BTreeSet<&str> = worktrees.iter().map(|tree| tree.branch.as_str()).collect();
    let github_events = match pull_requests {
        Ok(pull_requests) => pull_requests
            .into_iter()
            .filter_map(|pr| {
                let ticket = index.pull_request(&pr.url, &pr.branch);
                let ours = ticket.as_ref().is_some_and(|ticket| known.contains(ticket))
                    || local.contains(pr.branch.as_str());
                ours.then_some(TimelineEvent::PullRequest {
                    ticket,
                    branch: pr.branch,
                    number: pr.number,
                    title: pr.title,
                    state: pr.state,
                    checks: pr.checks,
                    review: pr.review,
                    head: pr.head,
                    url: pr.url,
                })
            })
            .collect(),
        Err(error) => vec![source_error(SourceKind::GitHub, None, &error)],
    };
    (
        index,
        vec![
            SourceUpdate {
                source: SourceKind::Linear,
                events: linear_events,
            },
            SourceUpdate {
                source: SourceKind::GitHub,
                events: github_events,
            },
        ],
    )
}

/// Prints each source's line when its items change, until writing fails (the
/// reader has gone) or, with `once`, after the first round.
pub fn watch(config: &Config, out: &mut impl Write, once: bool) -> std::io::Result<()> {
    let mut last: BTreeMap<SourceKind, Vec<TimelineEvent>> = BTreeMap::new();
    let mut index = TicketIndex::new(&config.teams, &[]);
    let mut remote: Vec<SourceUpdate> = Vec::new();
    let mut remote_at: Option<Instant> = None;
    loop {
        let (worktrees, mut updates) = match git::worktrees(&config.repo) {
            Ok(worktrees) => {
                let updates = local_updates(&worktrees);
                (worktrees, updates)
            }
            Err(error) => (
                Vec::new(),
                vec![SourceUpdate {
                    source: SourceKind::Worktrees,
                    events: vec![source_error(SourceKind::Worktrees, None, &error)],
                }],
            ),
        };
        if remote_at.is_none_or(|at| at.elapsed() >= config.remote_every) {
            (index, remote) = remote_updates(config, &worktrees);
            remote_at = Some(Instant::now());
        }
        for update in &mut updates {
            index.fill(&mut update.events);
        }
        updates.extend(remote.iter().cloned());
        for update in updates {
            if last.get(&update.source) == Some(&update.events) {
                continue;
            }
            serde_json::to_writer(&mut *out, &update)?;
            out.write_all(b"\n")?;
            out.flush()?;
            last.insert(update.source, update.events);
        }
        if once {
            return Ok(());
        }
        std::thread::sleep(config.local_every);
    }
}
