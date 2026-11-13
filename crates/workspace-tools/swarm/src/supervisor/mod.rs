//! Runs swarm members. Each member gets one thread that loops: mark it
//! working, run one turn through the [`WorkerAdapter`], give it one park
//! guard turn if it left mentions unread, park it waiting, and poll the
//! board until [`policy::rings`] admits a post for it or, for a narrator or
//! monitor, its timer ticks.
//!
//! The MCP server owns the roster and briefs; the supervisor runs the
//! members it is asked to start, as the coordinator's [`Spawner`]. It owns
//! every member it started and every worktree it created. A worktree whose
//! removal failed stays owned as pending cleanup and is retried on the next
//! [`Supervisor::kill`] or [`Supervisor::shutdown`].

mod observers;
mod workspaces;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};

pub use workspaces::{SandboxWorkspaces, WorkspaceError, Workspaces, WorktreeMode};

use crate::adapter::{Cancel, Catalog, ForkError, TurnOutcome, TurnRequest, WorkerAdapter};
use crate::mcp::{SpawnError, SpawnSpec, Spawner, Workspace};
use crate::message::{Body, Draft, Message, MessageKind, Seq};
use crate::names::MemberName;
use crate::policy;
use crate::roster::{MemberState, RosterError, WakeMode};
use crate::store::{Store, StoreError, UpdateError};
use observers::Observer;

#[derive(Debug, Clone)]
pub struct SupervisorConfig {
    /// The caller's directory, used by [`WorkspaceMode::Inherit`].
    pub cwd: PathBuf,
    /// The board directory each member's MCP server opens.
    pub swarm_dir: PathBuf,
    /// The `coda-swarm` executable members run as their MCP server.
    pub mcp_program: PathBuf,
    /// Prefix of every worktree slug, so swarms do not collide.
    pub slug_prefix: String,
    pub wake_mode: WakeMode,
    /// How often a waiting member polls the board.
    pub poll: Duration,
    /// Board messages read per store call while polling.
    pub read_batch: usize,
    /// How often a narrator gets a tick while others post.
    pub narrator_every: Duration,
    /// How often a monitor gets a tick while workers are live.
    pub monitor_every: Duration,
}

impl SupervisorConfig {
    /// `coda-swarm mcp --dir <swarm-dir> --agent <member>`: how `member` reaches
    /// the board.
    pub fn mcp_command(&self, member: &MemberName) -> Vec<String> {
        vec![
            self.mcp_program.display().to_string(),
            "mcp".to_owned(),
            "--dir".to_owned(),
            self.swarm_dir.display().to_string(),
            "--agent".to_owned(),
            member.to_string(),
        ]
    }

    pub fn new(
        cwd: impl Into<PathBuf>,
        swarm_dir: impl Into<PathBuf>,
        slug_prefix: impl Into<String>,
    ) -> Self {
        Self {
            cwd: cwd.into(),
            swarm_dir: swarm_dir.into(),
            mcp_program: PathBuf::from("coda-swarm"),
            slug_prefix: slug_prefix.into(),
            wake_mode: WakeMode::Mentions,
            poll: Duration::from_millis(200),
            read_batch: 256,
            narrator_every: Duration::from_secs(300),
            monitor_every: Duration::from_secs(180),
        }
    }
}
/// What the supervisor last saw of a member's run.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemberStatus {
    pub last_outcome: Option<TurnOutcome>,
    /// Set while the member is paused for quota.
    pub paused_until: Option<DateTime<Utc>>,
    /// The latest store or notice failure the member's loop recovered from.
    pub last_error: Option<String>,
}

#[derive(Debug, Default)]
pub struct KillReport {
    pub killed: Vec<MemberName>,
    /// Worktree slugs still owned because removal failed.
    pub pending_cleanup: Vec<String>,
    pub cleanup_errors: Vec<(String, WorkspaceError)>,
}

#[derive(Debug, thiserror::Error)]
pub enum StartError {
    #[error("fork {from}")]
    Fork {
        from: MemberName,
        #[source]
        source: ForkError,
    },
    #[error("create worktree {slug}")]
    Workspace {
        slug: String,
        #[source]
        source: WorkspaceError,
    },
    #[error(transparent)]
    Store(#[from] StoreError),
}

/// Runs the members of one swarm.
pub struct Supervisor<S: Store + Send + Sync + 'static> {
    shared: Arc<Shared<S>>,
    workspaces: Box<dyn Workspaces>,
    /// Spawn, kill and shutdown hold this for their whole run, so a cleanup
    /// is never observed half done.
    owned: Mutex<Owned>,
}

struct Shared<S> {
    store: S,
    adapter: Box<dyn WorkerAdapter>,
    config: SupervisorConfig,
    coordinator: MemberName,
    status: Mutex<BTreeMap<MemberName, MemberStatus>>,
}

#[derive(Default)]
struct Owned {
    runs: BTreeMap<MemberName, Run>,
    worktrees: BTreeMap<String, Hold>,
    pending: BTreeSet<String>,
}

struct Run {
    cancel: Cancel,
    thread: JoinHandle<()>,
    worktree: Option<String>,
}

/// An owned worktree and the members still using it.
struct Hold {
    path: PathBuf,
    holders: BTreeSet<MemberName>,
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

impl<S: Store + Send + Sync + 'static> Supervisor<S> {
    pub fn new(
        store: S,
        coordinator: MemberName,
        adapter: Box<dyn WorkerAdapter>,
        workspaces: Box<dyn Workspaces>,
        config: SupervisorConfig,
    ) -> Self {
        Self {
            shared: Arc::new(Shared {
                store,
                adapter,
                config,
                coordinator,
                status: Mutex::new(BTreeMap::new()),
            }),
            workspaces,
            owned: Mutex::new(Owned::default()),
        }
    }

    pub fn store(&self) -> &S {
        &self.shared.store
    }

    /// Runs `spec.name`, which the caller has already put on the roster and
    /// briefed: seeds its session from `spec.fork`, gives it a directory and
    /// starts its loop. Starting a member that already runs does nothing.
    pub fn start(&self, spec: &SpawnSpec) -> Result<(), StartError> {
        let mut owned = lock(&self.owned);
        if owned.runs.contains_key(&spec.name) {
            return Ok(());
        }
        if let Some(from) = &spec.fork {
            self.shared
                .adapter
                .fork(from, &spec.name, spec.model.as_deref())
                .map_err(|source| StartError::Fork {
                    from: from.clone(),
                    source,
                })?;
        }
        let since = self.shared.store.head()?;
        let observer = self
            .shared
            .store
            .roster()?
            .members
            .get(&spec.name)
            .and_then(|member| Observer::of(member.role));
        let (cwd, worktree) = self.place(&mut owned, spec)?;
        let cancel = Cancel::default();
        let thread = {
            let shared = Arc::clone(&self.shared);
            let name = spec.name.clone();
            let prompt = match observer {
                Some(observer) => format!(
                    "{}\n\n{}",
                    observer.role_prompt(&shared.coordinator, observer.every(&shared.config)),
                    spec.brief
                ),
                None => spec.brief.clone(),
            };
            let cancel = cancel.clone();
            thread::spawn(move || {
                shared.run_member(&name, observer, cwd, prompt, since, &cancel);
            })
        };
        lock(&self.shared.status).insert(spec.name.clone(), MemberStatus::default());
        owned.runs.insert(
            spec.name.clone(),
            Run {
                cancel,
                thread,
                worktree,
            },
        );
        Ok(())
    }

    /// The member's directory and the owned worktree slug it holds. A
    /// `share` worktree is keyed by the group, so later members reuse it.
    fn place(
        &self,
        owned: &mut Owned,
        spec: &SpawnSpec,
    ) -> Result<(PathBuf, Option<String>), StartError> {
        let prefix = &self.shared.config.slug_prefix;
        let (slug, mode) = match spec.workspace {
            Workspace::Inherit => return Ok((self.shared.config.cwd.clone(), None)),
            Workspace::Share => (format!("{prefix}-{}", spec.group), WorktreeMode::Share),
            Workspace::Branch => (format!("{prefix}-{}", spec.name), WorktreeMode::Branch),
        };
        let path = match owned.worktrees.get(&slug) {
            Some(hold) => hold.path.clone(),
            None => {
                self.workspaces
                    .create(&slug, mode)
                    .map_err(|source| StartError::Workspace {
                        slug: slug.clone(),
                        source,
                    })?
            }
        };
        owned
            .worktrees
            .entry(slug.clone())
            .or_insert_with(|| Hold {
                path: path.clone(),
                holders: BTreeSet::new(),
            })
            .holders
            .insert(spec.name.clone());
        Ok((path, Some(slug)))
    }

    /// Kills `name`. A member this supervisor does not run has nothing to
    /// stop.
    pub fn kill(&self, name: &MemberName) -> Result<KillReport, StoreError> {
        let mut owned = lock(&self.owned);
        let mut report = KillReport::default();
        self.retry_pending(&mut owned, &mut report);
        self.stop(&mut owned, name, MemberState::Killed, &mut report)?;
        report.pending_cleanup = owned.pending.iter().cloned().collect();
        Ok(report)
    }

    /// Stops every member this supervisor runs, leaves them stopped on the
    /// board and retries pending cleanup.
    pub fn shutdown(&self) -> Result<KillReport, StoreError> {
        let mut owned = lock(&self.owned);
        let mut report = KillReport::default();
        self.retry_pending(&mut owned, &mut report);
        let names: Vec<MemberName> = owned.runs.keys().cloned().collect();
        for name in names {
            self.stop(&mut owned, &name, MemberState::Stopped, &mut report)?;
        }
        report.pending_cleanup = owned.pending.iter().cloned().collect();
        Ok(report)
    }

    /// Stops the member's turn (SIGTERM, then SIGKILL after the adapter's
    /// grace), releases its worktree and leaves it in `state`.
    fn stop(
        &self,
        owned: &mut Owned,
        name: &MemberName,
        state: MemberState,
        report: &mut KillReport,
    ) -> Result<(), StoreError> {
        let Some(run) = owned.runs.remove(name) else {
            return Ok(());
        };
        run.cancel.cancel();
        // A panicked member loop has nothing left running to stop.
        let _ = run.thread.join();
        if let Some(slug) = &run.worktree {
            self.release(owned, slug, name, report);
        }
        let marked = self.shared.store.update_roster(|roster| {
            roster.member_mut(name)?.state = state;
            Ok::<_, RosterError>(())
        });
        match marked {
            Ok(()) | Err(UpdateError::Rejected(RosterError::Unknown(_))) => {}
            Err(UpdateError::Rejected(other)) => {
                lock(&self.shared.status)
                    .entry(name.clone())
                    .or_default()
                    .last_error = Some(other.to_string());
            }
            Err(UpdateError::Store(error)) => return Err(error),
        }
        if let Some(status) = lock(&self.shared.status).get_mut(name) {
            status.paused_until = None;
        }
        report.killed.push(name.clone());
        Ok(())
    }

    /// Drops `holder` from the worktree and removes it once no one holds it.
    /// A failed removal keeps the slug as pending cleanup.
    fn release(&self, owned: &mut Owned, slug: &str, holder: &MemberName, report: &mut KillReport) {
        if let Some(hold) = owned.worktrees.get_mut(slug) {
            hold.holders.remove(holder);
            if !hold.holders.is_empty() {
                return;
            }
            owned.worktrees.remove(slug);
        }
        if let Err(error) = self.workspaces.remove(slug) {
            owned.pending.insert(slug.to_owned());
            report.cleanup_errors.push((slug.to_owned(), error));
        }
    }

    fn retry_pending(&self, owned: &mut Owned, report: &mut KillReport) {
        let pending = std::mem::take(&mut owned.pending);
        for slug in pending {
            if let Err(error) = self.workspaces.remove(&slug) {
                report.cleanup_errors.push((slug.clone(), error));
                owned.pending.insert(slug);
            }
        }
    }

    /// The models members can run.
    pub fn models(&self) -> Catalog {
        self.shared.adapter.models()
    }

    /// The model of every live member of the swarm.
    pub fn member_models(&self) -> Result<BTreeMap<MemberName, Option<String>>, StoreError> {
        Ok(self
            .shared
            .store
            .roster()?
            .members
            .into_iter()
            .filter(|(_, member)| member.state.is_live())
            .map(|(name, member)| (name, member.model))
            .collect())
    }

    pub fn status(&self, name: &MemberName) -> Option<MemberStatus> {
        lock(&self.shared.status).get(name).cloned()
    }
}

/// Roster entries, briefs and model checks stay with the server. A model
/// change needs nothing here: each turn reads the member's model from the
/// roster.
impl<S: Store + Send + Sync + 'static> Spawner for Supervisor<S> {
    fn models(&self) -> Catalog {
        Supervisor::models(self)
    }

    fn probe_models(&self) -> Catalog {
        self.shared.adapter.probe_models()
    }

    fn spawn(&self, spec: &SpawnSpec) -> Result<(), SpawnError> {
        self.start(spec).map_err(|source| SpawnError::Failed {
            action: "start",
            member: spec.name.clone(),
            source: Box::new(source),
        })
    }

    fn kill(&self, name: &MemberName) -> Result<(), SpawnError> {
        let report = Supervisor::kill(self, name).map_err(|source| SpawnError::Failed {
            action: "kill",
            member: name.clone(),
            source: Box::new(source),
        })?;
        match report.cleanup_errors.into_iter().next() {
            None => Ok(()),
            Some((_, source)) => Err(SpawnError::Failed {
                action: "remove the worktree of",
                member: name.clone(),
                source: Box::new(source),
            }),
        }
    }

    fn set_model(&self, _name: &MemberName, _model: &str) -> Result<(), SpawnError> {
        Ok(())
    }
}

impl<S: Store + Send + Sync + 'static> Drop for Supervisor<S> {
    fn drop(&mut self) {
        // Nothing can report a failure from drop; pending worktrees stay on disk.
        let _ = self.shutdown();
    }
}

impl<S: Store> Shared<S> {
    fn run_member(
        &self,
        name: &MemberName,
        observer: Option<Observer>,
        cwd: PathBuf,
        first_prompt: String,
        mut since: Seq,
        cancel: &Cancel,
    ) {
        let mut prompt = first_prompt;
        let mut tick_mark = since;
        while !cancel.is_cancelled() {
            let model = match self.begin_turn(name) {
                Ok(model) => model,
                Err(error) => {
                    self.note_error(name, &error);
                    if self.idle(cancel, self.config.poll) {
                        continue;
                    }
                    return;
                }
            };
            let outcome = self.adapter.run_turn(
                TurnRequest {
                    member: name.clone(),
                    model,
                    prompt: prompt.clone(),
                    cwd: cwd.clone(),
                    mcp_command: self.config.mcp_command(name),
                    env: self.turn_env(name),
                },
                cancel,
            );
            lock(&self.status)
                .entry(name.clone())
                .or_default()
                .last_outcome = Some(outcome.clone());
            match outcome {
                TurnOutcome::Cancelled => return,
                TurnOutcome::QuotaExhausted { retry_after } => {
                    self.pause(name, retry_after);
                    if !self.idle(cancel, retry_after) {
                        return;
                    }
                    if let Some(status) = lock(&self.status).get_mut(name) {
                        status.paused_until = None;
                    }
                    continue;
                }
                TurnOutcome::Completed => match self.park_guard(name, since) {
                    Ok(ParkGuard::Clear) => {}
                    Ok(ParkGuard::Spent { head }) => since = since.max(head),
                    Ok(ParkGuard::Nudge { prompt: nudge }) => {
                        prompt = nudge;
                        continue;
                    }
                    Err(error) => self.note_error(name, &error),
                },
                TurnOutcome::Failed { .. } => {}
            }
            let ticks = observer.map(|observer| (observer, &mut tick_mark));
            prompt = match self.wait_for_ring(name, since, ticks, cancel) {
                None => return,
                Some(Wake::Tick { observer, head }) => observer.tick_prompt(head),
                Some(Wake::Ring(message)) => {
                    since = message.seq;
                    format!(
                        "New post #{} from {} in #{}:\n{}",
                        message.seq, message.agent, message.channel, message.text
                    )
                }
            };
        }
    }

    /// The park guard, run when a turn completes. Posts after both the
    /// member's read cursor and `delivered` (the last post handed to it in
    /// a prompt) that mention it earn one more turn to read and answer them.
    /// At most one such turn per board head, recorded on the roster so a
    /// restart does not repeat it; once spent, the member parks past them.
    fn park_guard(&self, name: &MemberName, delivered: Seq) -> Result<ParkGuard, GuardError> {
        let head = self.store.head()?;
        let roster = self.store.roster()?;
        let member = roster.member(name)?;
        let mut unread = Vec::new();
        let mut cursor = delivered.max(member.last_read_seq);
        loop {
            let batch = self.store.read_since(cursor, self.config.read_batch)?;
            let Some(last) = batch.last() else { break };
            cursor = last.seq;
            unread.extend(
                batch
                    .iter()
                    .filter(|m| {
                        m.body == Body::Post(MessageKind::Post)
                            && m.agent != *name
                            && roster.is_mentioned(name, member, &m.mentions())
                    })
                    .map(|m| format!("#{}", m.seq)),
            );
        }
        if unread.is_empty() {
            return Ok(ParkGuard::Clear);
        }
        let claimed = self.store.update_roster(|roster| {
            Ok::<_, RosterError>(roster.member_mut(name)?.take_park_block(head))
        })?;
        if !claimed {
            return Ok(ParkGuard::Spent { head });
        }
        Ok(ParkGuard::Nudge {
            prompt: format!(
                "[board] Your turn ended with unread posts that mention you: {}. The board \
                 is at #{head} and you have read up to #{}. Call read_board, answer what \
                 concerns you, then end your turn.",
                unread.join(", "),
                member.last_read_seq
            ),
        })
    }

    /// Marks the member working and returns the model for this turn.
    fn begin_turn(&self, name: &MemberName) -> Result<Option<String>, UpdateError<RosterError>> {
        self.store.update_roster(|roster| {
            let member = roster.member_mut(name)?;
            member.state = MemberState::Working;
            Ok(member.model.clone())
        })
    }

    /// The member's board identity.
    fn turn_env(&self, name: &MemberName) -> Vec<(String, String)> {
        vec![
            (
                "CODA_SWARM_DIR".to_owned(),
                self.config.swarm_dir.display().to_string(),
            ),
            ("CODA_SWARM_AGENT".to_owned(), name.to_string()),
        ]
    }

    fn pause(&self, name: &MemberName, retry_after: Duration) {
        let until =
            Utc::now() + chrono::Duration::from_std(retry_after).unwrap_or(chrono::Duration::MAX);
        lock(&self.status)
            .entry(name.clone())
            .or_default()
            .paused_until = Some(until);
        let stopped = self.store.update_roster(|roster| {
            roster.member_mut(name)?.state = MemberState::Stopped;
            Ok::<_, RosterError>(())
        });
        if let Err(error) = stopped {
            self.note_error(name, &error);
        }
        let notice = Draft::post(
            name.clone(),
            format!(
                "@{} {name} is paused: model quota exhausted; retrying after {}",
                self.coordinator,
                until.to_rfc3339()
            ),
        );
        if let Err(error) = self.store.append(notice, Utc::now()) {
            self.note_error(name, &error);
        }
    }

    /// Parks the member and polls the board until a post rings it or, for
    /// an observer, a tick is due.
    fn wait_for_ring(
        &self,
        name: &MemberName,
        since: Seq,
        mut ticks: Option<(Observer, &mut Seq)>,
        cancel: &Cancel,
    ) -> Option<Wake> {
        let mut next_tick = ticks
            .as_ref()
            .map(|(observer, _)| Instant::now() + observer.every(&self.config));
        let mut cursor = loop {
            let parked = self.store.update_roster(|roster| {
                let member = roster.member_mut(name)?;
                let (wake_mode, wait_since) = match member.state {
                    MemberState::Waiting {
                        wake_mode,
                        wait_since,
                    } => (wake_mode, wait_since.max(since)),
                    _ => (self.config.wake_mode, since),
                };
                let wait_since = wait_since.max(member.last_read_seq);
                member.state = MemberState::Waiting {
                    wake_mode,
                    wait_since,
                };
                Ok::<_, RosterError>(wait_since)
            });
            match parked {
                Ok(wait_since) => break wait_since,
                Err(error) => self.note_error(name, &error),
            }
            if !self.idle(cancel, self.config.poll) {
                return None;
            }
        };
        loop {
            match self.ringing(name, &mut cursor) {
                Ok(Some(message)) => return Some(Wake::Ring(message)),
                Ok(None) => {}
                Err(error) => self.note_error(name, &error),
            }
            if let (Some((observer, mark)), Some(at)) = (ticks.as_mut(), next_tick.as_mut())
                && Instant::now() >= *at
            {
                *at = Instant::now() + observer.every(&self.config);
                match self.tick_due(name, *observer, mark) {
                    Ok(Some(head)) => {
                        return Some(Wake::Tick {
                            observer: *observer,
                            head,
                        });
                    }
                    Ok(None) => {}
                    Err(error) => self.note_error(name, &error),
                }
            }
            if !self.idle(cancel, self.config.poll) {
                return None;
            }
        }
    }

    /// The first post after `cursor` that rings the member. Advances
    /// `cursor` past posts that do not.
    fn ringing(&self, name: &MemberName, cursor: &mut Seq) -> Result<Option<Message>, StoreError> {
        let roster = self.store.roster()?;
        let Some(member) = roster.members.get(name) else {
            return Ok(None);
        };
        loop {
            let batch = self.store.read_since(*cursor, self.config.read_batch)?;
            if batch.is_empty() {
                return Ok(None);
            }
            for message in batch {
                if policy::rings(&roster, name, member, &message) {
                    return Ok(Some(message));
                }
                *cursor = message.seq;
            }
        }
    }

    /// The head, when the observer's tick is worth a turn. Advances `mark`
    /// to the head either way, so each post counts toward one tick.
    fn tick_due(
        &self,
        name: &MemberName,
        observer: Observer,
        mark: &mut Seq,
    ) -> Result<Option<Seq>, StoreError> {
        let head = self.store.head()?;
        let roster = self.store.roster()?;
        let mut moved = false;
        let mut cursor = *mark;
        while !moved && cursor < head {
            let batch = self.store.read_since(cursor, self.config.read_batch)?;
            let Some(last) = batch.last() else { break };
            cursor = last.seq;
            moved = batch.iter().any(|m| Observer::moves(name, m));
        }
        *mark = head;
        Ok(observer.due(&roster, moved).then_some(head))
    }

    /// Sleeps up to `duration`; false once cancelled.
    fn idle(&self, cancel: &Cancel, duration: Duration) -> bool {
        let deadline = Instant::now() + duration;
        loop {
            if cancel.is_cancelled() {
                return false;
            }
            let now = Instant::now();
            if now >= deadline {
                return true;
            }
            thread::sleep(self.config.poll.min(deadline - now));
        }
    }

    fn note_error(&self, name: &MemberName, error: &dyn std::error::Error) {
        lock(&self.status)
            .entry(name.clone())
            .or_default()
            .last_error = Some(error.to_string());
    }
}

enum Wake {
    Ring(Message),
    Tick { observer: Observer, head: Seq },
}

enum ParkGuard {
    Clear,
    /// This head's one nudge already ran.
    Spent {
        head: Seq,
    },
    Nudge {
        prompt: String,
    },
}

#[derive(Debug, thiserror::Error)]
enum GuardError {
    #[error(transparent)]
    Store(#[from] StoreError),
    #[error(transparent)]
    Roster(#[from] RosterError),
    #[error(transparent)]
    Update(#[from] UpdateError<RosterError>),
}

#[cfg(test)]
mod tests;
