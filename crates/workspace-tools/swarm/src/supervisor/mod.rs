//! Runs swarm members. Each member gets one thread that loops: mark it
//! working, run one turn through the [`WorkerAdapter`], park it waiting, and
//! poll the board until [`policy::rings`] admits a post for it.
//!
//! The supervisor owns every member it spawned and every worktree it
//! created. A worktree whose removal failed stays owned as pending cleanup
//! and is retried on the next [`Supervisor::kill`] or
//! [`Supervisor::shutdown`].

mod keys;
mod workspaces;

use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};

pub use keys::{KeyError, KeySource, SandboxKeys, Secret};
pub use workspaces::{SandboxWorkspaces, WorkspaceError, WorkspaceMode, Workspaces, WorktreeMode};

pub use crate::adapter::KeyEnv;
use crate::adapter::{Cancel, Catalog, ForkError, TurnOutcome, TurnRequest, WorkerAdapter};
use crate::message::{Control, Draft, Message, MessageKind, Seq};
use crate::names::{ChannelName, MemberName, NameError};
use crate::policy;
use crate::roster::{Member, MemberState, Role, RosterError, WakeMode};
use crate::store::{AppendError, Store, StoreError, UpdateError};

#[derive(Debug, Clone)]
pub struct SupervisorConfig {
    /// The caller's directory, used by [`WorkspaceMode::Inherit`].
    pub cwd: PathBuf,
    /// The board directory each member's MCP server opens.
    pub swarm_dir: PathBuf,
    /// The `coda-swarm` executable members run as their MCP server.
    pub mcp_program: PathBuf,
    pub keys: Vec<KeyEnv>,
    /// Prefix of every worktree slug, so swarms do not collide.
    pub slug_prefix: String,
    pub wake_mode: WakeMode,
    /// How often a waiting member polls the board.
    pub poll: Duration,
    /// Board messages read per store call while polling.
    pub read_batch: usize,
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
            keys: Vec::new(),
            slug_prefix: slug_prefix.into(),
            wake_mode: WakeMode::Mentions,
            poll: Duration::from_millis(200),
            read_batch: 256,
        }
    }
}

/// One spawn call: `count` members named `name` or `name-1..count`.
#[derive(Debug, Clone)]
pub struct SpawnSpec {
    pub name: String,
    pub brief: String,
    pub model: Option<String>,
    pub workspace: WorkspaceMode,
    pub channels: Vec<ChannelName>,
    pub count: NonZeroUsize,
    /// A member whose history each new member starts from.
    pub fork: Option<MemberName>,
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
pub enum SpawnError {
    #[error(transparent)]
    Name(#[from] NameError),
    #[error("{0} already exists; member names are never reused")]
    Exists(MemberName),
    #[error("fork {from} into {to}")]
    Fork {
        from: MemberName,
        to: MemberName,
        #[source]
        source: ForkError,
    },
    #[error("post brief")]
    Brief(#[source] AppendError),
    #[error("create worktree {slug}")]
    Workspace {
        slug: String,
        #[source]
        source: WorkspaceError,
    },
    #[error("add members")]
    Roster(#[source] UpdateError<RosterError>),
    #[error(transparent)]
    Store(#[from] StoreError),
}

#[derive(Debug, thiserror::Error)]
pub enum KillError {
    #[error("{0} is the coordinator and cannot be killed")]
    Coordinator(MemberName),
    #[error("{0} is already killed")]
    AlreadyKilled(MemberName),
    #[error("no member or group spawned by this supervisor is named {0}")]
    Unknown(String),
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
    keys: Box<dyn KeySource>,
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
        keys: Box<dyn KeySource>,
        config: SupervisorConfig,
    ) -> Self {
        Self {
            shared: Arc::new(Shared {
                store,
                adapter,
                keys,
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

    /// Adds `spec.count` workers added by `by`, posts the brief from `by`,
    /// and starts each member's loop. Returns the new names.
    pub fn spawn(&self, by: &MemberName, spec: SpawnSpec) -> Result<Vec<MemberName>, SpawnError> {
        let mut owned = lock(&self.owned);
        let names = MemberName::group(&spec.name, spec.count)?;
        let roster = self.shared.store.roster()?;
        if let Some(taken) = names.iter().find(|name| roster.members.contains_key(*name)) {
            return Err(SpawnError::Exists(taken.clone()));
        }
        if let Some(from) = &spec.fork {
            for to in &names {
                self.shared
                    .adapter
                    .fork(from, to, spec.model.as_deref())
                    .map_err(|source| SpawnError::Fork {
                        from: from.clone(),
                        to: to.clone(),
                        source,
                    })?;
            }
        }
        let brief = Draft {
            author: by.clone(),
            text: format!("@{} {}", spec.name, spec.brief),
            channel: None,
            kind: MessageKind::Brief,
            request_id: None,
        };
        let since = self
            .shared
            .store
            .append(brief, Utc::now())
            .map_err(SpawnError::Brief)?
            .seq;

        let places = self.create_worktrees(&mut owned, &spec, &names)?;
        let added = self.shared.store.update_roster(|roster| {
            for name in &names {
                let mut member = Member::new(Role::Worker, Some(by.clone()));
                member.model.clone_from(&spec.model);
                roster.add_member(name.clone(), member)?;
            }
            Ok::<_, RosterError>(())
        });
        let added = added.and_then(|()| {
            spec.channels.iter().try_for_each(|channel| {
                let control = Control::Subscribe {
                    channel: channel.clone(),
                    members: names.clone(),
                };
                self.shared
                    .store
                    .record(by.clone(), control, Utc::now())
                    .map(drop)
            })
        });
        if let Err(error) = added {
            let slugs: BTreeSet<String> = places.into_iter().filter_map(|(_, slug)| slug).collect();
            for slug in slugs {
                if let Some(hold) = owned.worktrees.get_mut(&slug) {
                    hold.holders.retain(|holder| !names.contains(holder));
                }
                self.release(&mut owned, &slug, None);
            }
            return Err(SpawnError::Roster(error));
        }

        for (name, (cwd, slug)) in names.iter().zip(places) {
            let cancel = Cancel::default();
            let shared = Arc::clone(&self.shared);
            let prompt = spec.brief.clone();
            let thread = {
                let name = name.clone();
                let cancel = cancel.clone();
                thread::spawn(move || shared.run_member(&name, cwd, prompt, since, &cancel))
            };
            lock(&self.shared.status).insert(name.clone(), MemberStatus::default());
            owned.runs.insert(
                name.clone(),
                Run {
                    cancel,
                    thread,
                    worktree: slug,
                },
            );
        }
        Ok(names)
    }

    /// The directory and owned worktree slug for each name, in order.
    fn create_worktrees(
        &self,
        owned: &mut Owned,
        spec: &SpawnSpec,
        names: &[MemberName],
    ) -> Result<Vec<(PathBuf, Option<String>)>, SpawnError> {
        let Some(mode) = spec.workspace.worktree() else {
            return Ok(names
                .iter()
                .map(|_| (self.shared.config.cwd.clone(), None))
                .collect());
        };
        let prefix = &self.shared.config.slug_prefix;
        let slugs: Vec<String> = match mode {
            WorktreeMode::Share => vec![format!("{prefix}-{}", spec.name)],
            WorktreeMode::Branch => names
                .iter()
                .map(|name| format!("{prefix}-{name}"))
                .collect(),
        };
        let mut paths = Vec::with_capacity(slugs.len());
        for slug in &slugs {
            match self.workspaces.create(slug, mode) {
                Ok(path) => paths.push(path),
                Err(source) => {
                    for created in &slugs[..paths.len()] {
                        self.release(owned, created, None);
                    }
                    return Err(SpawnError::Workspace {
                        slug: slug.clone(),
                        source,
                    });
                }
            }
        }
        let places = names
            .iter()
            .enumerate()
            .map(|(index, name)| {
                let at = if mode == WorktreeMode::Share {
                    0
                } else {
                    index
                };
                let slug = slugs[at].clone();
                owned
                    .worktrees
                    .entry(slug.clone())
                    .or_insert_with(|| Hold {
                        holders: BTreeSet::new(),
                    })
                    .holders
                    .insert(name.clone());
                (paths[at].clone(), Some(slug))
            })
            .collect();
        Ok(places)
    }

    /// Kills the member `target`, or every member of the group `target`
    /// (`target-1`, `target-2`, ...). Members they added keep running.
    pub fn kill(&self, target: &str) -> Result<KillReport, KillError> {
        let mut owned = lock(&self.owned);
        let names = self.resolve(&owned, target)?;
        let mut report = KillReport::default();
        self.retry_pending(&mut owned, &mut report);
        for name in names {
            self.stop(&mut owned, &name, &mut report)?;
        }
        report.pending_cleanup = owned.pending.iter().cloned().collect();
        Ok(report)
    }

    /// Kills every member this supervisor runs and retries pending cleanup.
    pub fn shutdown(&self) -> Result<KillReport, KillError> {
        let mut owned = lock(&self.owned);
        let mut report = KillReport::default();
        self.retry_pending(&mut owned, &mut report);
        let names: Vec<MemberName> = owned.runs.keys().cloned().collect();
        for name in names {
            self.stop(&mut owned, &name, &mut report)?;
        }
        report.pending_cleanup = owned.pending.iter().cloned().collect();
        Ok(report)
    }

    fn resolve(&self, owned: &Owned, target: &str) -> Result<Vec<MemberName>, KillError> {
        let roster = self.shared.store.roster()?;
        let named = |name: &MemberName| {
            name.as_str() == target
                || name
                    .as_str()
                    .strip_prefix(target)
                    .and_then(|rest| rest.strip_prefix('-'))
                    .is_some_and(|index| {
                        !index.is_empty() && index.bytes().all(|b| b.is_ascii_digit())
                    })
        };
        for (name, member) in roster.members.iter().filter(|(name, _)| named(name)) {
            if *name == self.shared.coordinator || member.role == Role::Coordinator {
                return Err(KillError::Coordinator(name.clone()));
            }
        }
        let names: Vec<MemberName> = owned
            .runs
            .keys()
            .filter(|name| named(name))
            .cloned()
            .collect();
        if names.is_empty() {
            let killed = roster
                .members
                .iter()
                .find(|(name, member)| named(name) && member.state == MemberState::Killed);
            return Err(match killed {
                Some((name, _)) => KillError::AlreadyKilled(name.clone()),
                None => KillError::Unknown(target.to_owned()),
            });
        }
        Ok(names)
    }

    /// Stops the member's turn (SIGTERM, then SIGKILL after the adapter's
    /// grace), releases its worktree and marks it killed.
    fn stop(
        &self,
        owned: &mut Owned,
        name: &MemberName,
        report: &mut KillReport,
    ) -> Result<(), KillError> {
        let Some(run) = owned.runs.remove(name) else {
            return Ok(());
        };
        run.cancel.cancel();
        // A panicked member loop has nothing left running to stop.
        let _ = run.thread.join();
        if let Some(slug) = &run.worktree {
            self.release(owned, slug, Some((name, report)));
        }
        let marked = self.shared.store.update_roster(|roster| {
            roster.member_mut(name)?.state = MemberState::Killed;
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
            Err(UpdateError::Store(error)) => return Err(KillError::Store(error)),
        }
        if let Some(status) = lock(&self.shared.status).get_mut(name) {
            status.paused_until = None;
        }
        report.killed.push(name.clone());
        Ok(())
    }

    /// Drops `holder` from the worktree and removes it once no one holds it.
    /// A failed removal keeps the slug as pending cleanup.
    fn release(
        &self,
        owned: &mut Owned,
        slug: &str,
        holder: Option<(&MemberName, &mut KillReport)>,
    ) {
        let (name, report) = match holder {
            Some((name, report)) => (Some(name), Some(report)),
            None => (None, None),
        };
        if let Some(hold) = owned.worktrees.get_mut(slug) {
            if let Some(name) = name {
                hold.holders.remove(name);
            }
            if !hold.holders.is_empty() {
                return;
            }
            owned.worktrees.remove(slug);
        }
        if let Err(error) = self.workspaces.remove(slug) {
            owned.pending.insert(slug.to_owned());
            if let Some(report) = report {
                report.cleanup_errors.push((slug.to_owned(), error));
            }
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

    /// Sets the model the member's next turn uses.
    pub fn set_model(
        &self,
        name: &MemberName,
        model: Option<String>,
    ) -> Result<(), UpdateError<RosterError>> {
        self.shared.store.update_roster(|roster| {
            roster.member_mut(name)?.model = model;
            Ok(())
        })
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
        cwd: PathBuf,
        first_prompt: String,
        mut since: Seq,
        cancel: &Cancel,
    ) {
        let mut prompt = first_prompt;
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
            let outcome = match self.turn_env(name, model.as_deref()) {
                Ok(env) => self.adapter.run_turn(
                    TurnRequest {
                        member: name.clone(),
                        model,
                        prompt: prompt.clone(),
                        cwd: cwd.clone(),
                        mcp_command: self.config.mcp_command(name),
                        env,
                    },
                    cancel,
                ),
                Err(error) => TurnOutcome::Failed {
                    reason: error.to_string(),
                },
            };
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
                TurnOutcome::Completed | TurnOutcome::Failed { .. } => {}
            }
            let Some(message) = self.wait_for_ring(name, since, cancel) else {
                return;
            };
            since = message.seq;
            prompt = format!(
                "New post #{} from {} in #{}:\n{}",
                message.seq, message.agent, message.channel, message.text
            );
        }
    }

    /// Marks the member working and returns the model for this turn.
    fn begin_turn(&self, name: &MemberName) -> Result<Option<String>, UpdateError<RosterError>> {
        self.store.update_roster(|roster| {
            let member = roster.member_mut(name)?;
            member.state = MemberState::Working;
            Ok(member.model.clone())
        })
    }

    /// The member's board identity, then every key the turn needs.
    fn turn_env(
        &self,
        name: &MemberName,
        model: Option<&str>,
    ) -> Result<Vec<(String, String)>, KeyError> {
        let identity = [
            (
                "CODA_SWARM_DIR".to_owned(),
                self.config.swarm_dir.display().to_string(),
            ),
            ("CODA_SWARM_AGENT".to_owned(), name.to_string()),
        ];
        let model_keys = self.adapter.keys(model);
        let keys = self
            .config
            .keys
            .iter()
            .chain(&model_keys)
            .map(|key| {
                let secret = self.keys.get(&key.item)?;
                Ok((key.var.clone(), secret.expose().to_owned()))
            })
            .collect::<Result<Vec<_>, KeyError>>()?;
        Ok(identity.into_iter().chain(keys).collect())
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

    /// Parks the member and polls the board until a post rings it.
    fn wait_for_ring(&self, name: &MemberName, since: Seq, cancel: &Cancel) -> Option<Message> {
        let mut cursor = loop {
            let parked = self.store.update_roster(|roster| {
                let member = roster.member_mut(name)?;
                let wait_since = since.max(member.last_read_seq);
                member.state = MemberState::Waiting {
                    wake_mode: self.config.wake_mode,
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
                Ok(Some(message)) => return Some(message),
                Ok(None) => {}
                Err(error) => self.note_error(name, &error),
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

#[cfg(test)]
mod tests;
