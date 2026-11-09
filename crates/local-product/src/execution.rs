//! Physician-owned accepted work and completion fences. The shared engine
//! stores nothing. This local persistent owner replaces the volatile chat map;
//! its exclusive file lock deliberately bounds it to one server process.
//! Production multi-instance storage remains a separate qualification gate.

use crate::ModelBinding;
use harness::execution::{Call, Cancellation, Item, Owner, Record, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
};

/// Signed-in owner of an inline conversation, supplied by the server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatOwner {
    /// Practice tenant.
    pub practice: String,
    /// Physician within that practice.
    pub physician: String,
}

/// The conversation addressed by an inline read or submission. `None` is the
/// explicit local anonymous session; it never selects signed-in conversations.
#[derive(Clone, Copy)]
pub struct ChatScope<'a> {
    /// Authenticated owner, absent only for local anonymous use.
    pub owner: Option<&'a ChatOwner>,
    /// Authorized patient workspace.
    pub workspace: &'a str,
    /// Client conversation ID, scoped to this owner and workspace.
    pub chat: &'a str,
}

/// Product execution surface; reply revisions cannot acquire write authority.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Surface {
    /// Ordinary physician conversation.
    Chat,
    /// A physician-owned saved reply draft revision.
    ReplyRevision,
}
/// Persisted terminal status, selected by the physician adapter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    /// Accepted, may be executing.
    Active,
    /// Atomically published.
    Completed,
    /// Process restart; physician asks again, never resumes this work.
    Interrupted,
    /// Explicitly stopped or permission withdrawn.
    Cancelled,
    /// Failed transport, evidence or stale completion.
    Failed,
}
/// One accepted physician work identity and immutable input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Work {
    /// Idempotency key scoped to workspace and conversation/draft.
    pub id: String,
    /// Physician workspace.
    pub workspace: String,
    /// Conversation or physician-specific draft identity.
    pub target: String,
    // Old unscoped chat work remains local; it is never assigned to a physician
    // merely because that physician signs in after an upgrade.
    #[serde(default)]
    owner: Option<ChatOwner>,
    /// Expected target revision at admission.
    pub revision: u64,
    /// Product authority surface.
    pub surface: Surface,
    /// Immutable provider context, including original instructions.
    pub request: Request,
    /// Product-owned execution status.
    pub status: Status,
    /// Provider evidence, including late usage and failures.
    pub records: Vec<Record>,
    /// Exact physician-source observation this work currently pins: from
    /// admission, replaced by each refresh.
    #[serde(default)]
    pub sources: Vec<crate::physician_sources::SourceRevision>,
    /// Source observation version of `sources`.
    #[serde(default)]
    pub sources_version: u64,
    /// Each time a finish found the sources moved, in order, with the
    /// earlier revisions, so the record shows what the work could read when.
    #[serde(default)]
    pub refreshes: Vec<crate::physician_sources::SourceRefresh>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Target {
    workspace: String,
    id: String,
    #[serde(default)]
    owner: Option<ChatOwner>,
    revision: u64,
    open: bool,
    turns: Vec<(String, String)>,
    draft: Option<String>,
    restore_text: Option<String>,
    /// Unix seconds when the target was created; 0 on older session files.
    #[serde(default)]
    opened_at: u64,
    /// Listed under Archived until a new message or Undo.
    #[serde(default)]
    archived: bool,
    /// Each archive command taken on this chat: its key and whether it
    /// archived (`true`) or restored, so a re-asked key changes nothing.
    #[serde(default)]
    archive_keys: Vec<(String, bool)>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct State {
    targets: Vec<Target>,
    work: Vec<Work>,
}

/// Physician accepted-work owner. No independent engine run ledger exists.
#[derive(Debug)]
pub struct PhysicianSession {
    state: State,
    path: Option<PathBuf>,
    _lock: Option<File>,
    // A publication error can follow rename: no further writes may use the
    // pre-write snapshot. Reopen/readback through this owner is required.
    blocked: bool,
}
fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |since| since.as_secs())
}
impl PhysicianSession {
    /// Ephemeral controlled tests only; server selects `open` when configured.
    pub fn memory() -> Self {
        Self {
            state: State::default(),
            path: None,
            _lock: None,
            blocked: false,
        }
    }
    /// Opens the single-writer local persistent owner and settles unfinished
    /// work as interrupted. Corrupt evidence fails closed, never resets history.
    pub fn open(path: &Path) -> Result<Self, String> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let lock = options
            .open(path.with_extension("lock"))
            .map_err(|e| e.to_string())?;
        lock.try_lock()
            .map_err(|_| "physician session is already owned by another process")?;
        let state = match std::fs::read(path) {
            Ok(bytes) => serde_json::from_slice(&bytes)
                .map_err(|_| "physician session evidence is unreadable")?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => State::default(),
            Err(e) => return Err(e.to_string()),
        };
        let mut session = Self {
            state,
            path: Some(path.into()),
            _lock: Some(lock),
            blocked: false,
        };
        session.change(|state| {
            for work in &mut state.work {
                if work.status == Status::Active {
                    work.status = Status::Interrupted;
                }
            }
            Ok(())
        })?;
        Ok(session)
    }
    fn change<T>(
        &mut self,
        edit: impl FnOnce(&mut State) -> Result<T, String>,
    ) -> Result<T, String> {
        if self.blocked {
            return Err("physician owner requires recovery readback".into());
        }
        let mut next = self.state.clone();
        let result = edit(&mut next)?;
        if let Some(path) = &self.path
            && let Err(error) = harness::transport::write_atomic(
                path,
                &serde_json::to_vec(&next).map_err(|e| e.to_string())?,
            )
        {
            self.blocked = true;
            return Err(error);
        }
        self.state = next;
        Ok(result)
    }
    /// Admits a chat input durably before dispatch. Same key and different
    /// bytes is a conflict, not a successful duplicate.
    pub fn submit(
        &mut self,
        binding: &ModelBinding,
        id: &str,
        scope: ChatScope<'_>,
        text: &str,
    ) -> Result<Option<Work>, String> {
        let ChatScope {
            owner,
            workspace,
            chat: target,
        } = scope;
        if owner.is_some_and(|owner| {
            owner.practice.trim().is_empty() || owner.physician.trim().is_empty()
        }) {
            return Err("invalid physician identity".into());
        }
        if [id, workspace, target, text]
            .iter()
            .any(|s| s.trim().is_empty())
        {
            return Err("invalid physician input".into());
        }
        self.change(|state| {
            if let Some(work) = state.work.iter().find(|w| {
                w.owner.as_ref() == owner
                    && w.id == id
                    && w.workspace == workspace
                    && w.target == target
            }) {
                let same = work.surface == Surface::Chat
                    && work.request.items.last()
                        == Some(&Item::Message {
                            role: "user".into(),
                            text: text.into(),
                        });
                return if same {
                    Ok(None)
                } else {
                    Err("idempotency input conflict".into())
                };
            }
            let index = state
                .targets
                .iter()
                .position(|t| {
                    t.owner.as_ref() == owner && t.workspace == workspace && t.id == target
                })
                .unwrap_or_else(|| {
                    state.targets.push(Target {
                        workspace: workspace.into(),
                        id: target.into(),
                        owner: owner.cloned(),
                        revision: 0,
                        open: true,
                        turns: vec![],
                        draft: None,
                        restore_text: None,
                        opened_at: unix_now(),
                        archived: false,
                        archive_keys: vec![],
                    });
                    state.targets.len() - 1
                });
            let target_state = &mut state.targets[index];
            target_state.archived = false;
            target_state.revision += 1;
            target_state.turns.push(("user".into(), text.into()));
            let work = Work {
                id: id.into(),
                workspace: workspace.into(),
                target: target.into(),
                owner: owner.cloned(),
                revision: target_state.revision,
                surface: Surface::Chat,
                status: Status::Active,
                records: vec![],
                sources: vec![],
                sources_version: 0,
                refreshes: vec![],
                request: Request {
                    model: binding.model.clone(),
                    options: json!({"effort": binding.effort}),
                    instructions: crate::codex::PHYSICIAN_CHAT_INSTRUCTIONS.into(),
                    tools: vec![],
                    items: target_state
                        .turns
                        .iter()
                        .map(|(role, text)| Item::Message {
                            role: role.clone(),
                            text: text.clone(),
                        })
                        .collect(),
                },
            };
            state.work.push(work.clone());
            Ok(Some(work))
        })
    }
    /// Saves a physician-specific draft under an exact revision fence.
    pub fn save_draft(
        &mut self,
        workspace: &str,
        target: &str,
        expected: u64,
        text: &str,
    ) -> Result<u64, String> {
        self.change(|state| {
            if !state
                .targets
                .iter()
                .any(|t| t.owner.is_none() && t.workspace == workspace && t.id == target)
            {
                if expected != 0 {
                    return Err("stale draft".into());
                }
                state.targets.push(Target {
                    workspace: workspace.into(),
                    id: target.into(),
                    owner: None,
                    revision: 0,
                    open: true,
                    turns: vec![],
                    draft: None,
                    restore_text: None,
                    opened_at: unix_now(),
                    archived: false,
                    archive_keys: vec![],
                });
            }
            let t = state
                .targets
                .iter_mut()
                .find(|t| t.owner.is_none() && t.workspace == workspace && t.id == target)
                .unwrap();
            if !t.open || t.revision != expected {
                return Err("stale draft".into());
            }
            t.restore_text = t.draft.replace(text.into());
            t.revision += 1;
            Ok(t.revision)
        })
    }
    /// Opens trusted, server-owned reply work's first saved draft at the
    /// first revision. The caller holds the physician publication fence.
    /// Opening again leaves an existing draft, its edits and revision unchanged.
    pub fn open_reply_draft(
        &mut self,
        workspace: &str,
        target: &str,
        text: &str,
    ) -> Result<u64, String> {
        if [workspace, target, text]
            .iter()
            .any(|s| s.trim().is_empty())
        {
            return Err("invalid reply draft".into());
        }
        if let Some((revision, _, _)) = self.draft(workspace, target) {
            return Ok(revision);
        }
        self.change(|state| {
            if state
                .targets
                .iter()
                .any(|t| t.owner.is_none() && t.workspace == workspace && t.id == target)
            {
                return Err("reply target exists without a draft".into());
            }
            state.targets.push(Target {
                workspace: workspace.into(),
                id: target.into(),
                owner: None,
                revision: 1,
                open: true,
                turns: vec![],
                draft: Some(text.into()),
                restore_text: None,
                opened_at: unix_now(),
                archived: false,
                archive_keys: vec![],
            });
            Ok(1)
        })
    }
    /// The latest reply revision admitted on this draft and its status.
    pub fn reply_run(&self, workspace: &str, target: &str) -> Option<(String, Status)> {
        self.state
            .work
            .iter()
            .rev()
            .find(|w| {
                w.owner.is_none()
                    && w.surface == Surface::ReplyRevision
                    && w.workspace == workspace
                    && w.target == target
            })
            .map(|w| (w.id.clone(), w.status.clone()))
    }
    /// Admits an already assembled immutable reply revision request. The caller
    /// binds target to the asking physician and exact reply work identity.
    pub fn revise(
        &mut self,
        id: &str,
        workspace: &str,
        target: &str,
        expected: u64,
        request: Request,
    ) -> Result<Work, String> {
        self.change(|state| {
            if state.work.iter().any(|w| {
                w.owner.is_none()
                    && w.workspace == workspace
                    && w.target == target
                    && (w.id == id || w.status == Status::Active)
            }) {
                return Err("reply revision already admitted".into());
            }
            let t = state
                .targets
                .iter()
                .find(|t| t.owner.is_none() && t.workspace == workspace && t.id == target)
                .ok_or("missing saved draft")?;
            if !t.open || t.revision != expected || t.draft.is_none() {
                return Err("stale draft".into());
            }
            let work = Work {
                id: id.into(),
                workspace: workspace.into(),
                target: target.into(),
                owner: None,
                revision: expected,
                surface: Surface::ReplyRevision,
                request,
                status: Status::Active,
                records: vec![],
                sources: vec![],
                sources_version: 0,
                refreshes: vec![],
            };
            state.work.push(work.clone());
            Ok(work)
        })
    }
    /// Pins the exact source observation at `version` before the first
    /// dispatch. A source candidate cannot name two different revisions
    /// within one observation.
    pub fn pin_sources(
        &mut self,
        key: &Work,
        version: u64,
        sources: Vec<crate::physician_sources::SourceRevision>,
    ) -> Result<Work, String> {
        self.change(|state| {
            check(state, key)?;
            let work = work_mut(state, key)?;
            if !work.records.is_empty() {
                return Err("source observation already dispatched".into());
            }
            scoped(&work.workspace, &sources)?;
            work.sources = sources;
            work.sources_version = version;
            Ok(work.clone())
        })
    }
    /// Re-pins active work whose sources moved before it finished, keeping
    /// the change on the work. Returns the recorded refresh, or `None` when
    /// the observation is still the pinned one or nothing readable changed.
    pub fn refresh_sources(
        &mut self,
        key: &Work,
        version: u64,
        sources: Vec<crate::physician_sources::SourceRevision>,
    ) -> Result<Option<crate::physician_sources::SourceRefresh>, String> {
        self.change(|state| {
            check(state, key)?;
            let work = work_mut(state, key)?;
            if version == work.sources_version {
                return Ok(None);
            }
            scoped(&work.workspace, &sources)?;
            let mut refresh = crate::physician_sources::SourceRefresh::between(
                work.sources_version,
                &work.sources,
                version,
                &sources,
            );
            work.sources = sources;
            work.sources_version = version;
            if refresh.is_empty() {
                return Ok(None);
            }
            refresh.after_records = work.records.len();
            work.refreshes.push(refresh.clone());
            Ok(Some(refresh))
        })
    }
    /// Closes this physician reply target. Active revisions fail and cannot
    /// later publish or resume; retained draft text is not replaced.
    pub fn close_reply(&mut self, workspace: &str, target: &str) -> Result<(), String> {
        self.change(|state| {
            let t = state
                .targets
                .iter_mut()
                .find(|t| t.owner.is_none() && t.workspace == workspace && t.id == target)
                .ok_or("missing reply target")?;
            t.open = false;
            for work in &mut state.work {
                if work.owner.is_none()
                    && work.workspace == workspace
                    && work.target == target
                    && work.status == Status::Active
                {
                    work.status = Status::Failed;
                }
            }
            Ok(())
        })
    }
    /// Reads retained status/evidence for the same accepted identity.
    pub fn work(&self, key: &Work) -> Option<&Work> {
        self.state.work.iter().find(|w| {
            w.owner == key.owner
                && w.id == key.id
                && w.workspace == key.workspace
                && w.target == key.target
        })
    }
    /// Currentness before dispatch; completion checks it again in the write.
    pub fn check(&self, key: &Work) -> Result<(), String> {
        if self.blocked {
            return Err("physician owner requires recovery readback".into());
        }
        check(&self.state, key)
    }
    /// Retains provider observations even if their target became stale.
    pub fn record(&mut self, key: &Work, record: Record) -> Result<(), String> {
        self.change(|state| {
            work_mut(state, key)?.records.push(record);
            Ok(())
        })
    }
    /// Settles without publishing. Completed work cannot be overwritten.
    pub fn settle(&mut self, key: &Work, status: Status) -> Result<(), String> {
        if matches!(status, Status::Active | Status::Completed) {
            return Err("invalid settlement".into());
        }
        self.change(|state| {
            let w = work_mut(state, key)?;
            if w.status == Status::Active {
                w.status = status;
            }
            Ok(())
        })
    }
    /// Completion and draft/chat publication are one synced owner transaction.
    pub fn complete(&mut self, key: &Work, text: &str) -> Result<(), String> {
        if text.trim().is_empty() {
            return Err("empty physician completion".into());
        }
        self.change(|state| {
            check(state, key)?;
            let t = state
                .targets
                .iter_mut()
                .find(|t| {
                    t.owner == key.owner && t.workspace == key.workspace && t.id == key.target
                })
                .unwrap();
            match key.surface {
                Surface::Chat => t.turns.push(("assistant".into(), text.into())),
                Surface::ReplyRevision => {
                    t.restore_text = t.draft.replace(text.into());
                    t.revision += 1;
                }
            }
            work_mut(state, key)?.status = Status::Completed;
            Ok(())
        })
    }
    /// Physician-visible authored turns in order, each with who wrote it (`user` for the
    /// physician, `assistant` for Coda), never provider/tool evidence.
    pub fn conversation(&self, scope: ChatScope<'_>) -> Vec<(String, String)> {
        self.state
            .targets
            .iter()
            .find(|t| {
                t.owner.as_ref() == scope.owner
                    && t.workspace == scope.workspace
                    && t.id == scope.chat
            })
            .map(|t| t.turns.clone())
            .unwrap_or_default()
    }
    /// This owner's chats in a workspace that have a physician turn, newest opened first,
    /// each with its first physician message, opening Unix second, and whether
    /// it is archived. Reply drafts have no turns and are not chats.
    pub fn chats(
        &self,
        owner: Option<&ChatOwner>,
        workspace: &str,
    ) -> Vec<(String, String, u64, bool)> {
        self.state
            .targets
            .iter()
            .rev()
            .filter(|t| t.owner.as_ref() == owner && t.workspace == workspace)
            .filter_map(|t| {
                let (_, first) = t.turns.iter().find(|(role, _)| role == "user")?;
                Some((t.id.clone(), first.clone(), t.opened_at, t.archived))
            })
            .collect()
    }
    /// Archive, or its Undo, on one of this owner's chats, once per key: a
    /// re-asked key answers [`Archived::Duplicate`] and changes nothing, and
    /// the other direction under that key is refused. Archive is
    /// [`Archived::Busy`] while an answer is being written.
    pub fn archive(
        &mut self,
        key: &str,
        scope: ChatScope<'_>,
        direction: crate::runtime::Archive,
    ) -> Result<crate::runtime::Archived, String> {
        use crate::runtime::{Archive, Archived};
        let ChatScope {
            owner,
            workspace,
            chat,
        } = scope;
        if [key, workspace, chat].iter().any(|s| s.trim().is_empty()) {
            return Ok(Archived::Refused);
        }
        let archive = direction == Archive::Archive;
        self.change(|state| {
            let running = state.work.iter().any(|w| {
                w.owner.as_ref() == owner
                    && w.workspace == workspace
                    && w.target == chat
                    && w.status == Status::Active
            });
            let Some(target) = state.targets.iter_mut().find(|t| {
                t.owner.as_ref() == owner
                    && t.workspace == workspace
                    && t.id == chat
                    && !t.turns.is_empty()
            }) else {
                return Ok(Archived::Refused);
            };
            if let Some((_, taken)) = target.archive_keys.iter().find(|(k, _)| k == key) {
                return Ok(if *taken == archive {
                    Archived::Duplicate
                } else {
                    Archived::Refused
                });
            }
            // An accepted command keeps its key, even one that found the chat
            // already where it asked, so a late retry never undoes a later one.
            if target.archived == archive {
                target.archive_keys.push((key.to_owned(), archive));
                return Ok(Archived::Duplicate);
            }
            if archive && running {
                return Ok(Archived::Busy);
            }
            target.archived = archive;
            target.archive_keys.push((key.to_owned(), archive));
            Ok(Archived::Done)
        })
    }
    /// Saved draft revision, current text, and replaced text.
    pub fn draft(&self, workspace: &str, target: &str) -> Option<(u64, String, Option<String>)> {
        let t = self
            .state
            .targets
            .iter()
            .find(|t| t.owner.is_none() && t.workspace == workspace && t.id == target)?;
        Some((t.revision, t.draft.clone()?, t.restore_text.clone()))
    }
}
fn check(state: &State, key: &Work) -> Result<(), String> {
    let w = state
        .work
        .iter()
        .find(|w| {
            w.owner == key.owner
                && w.id == key.id
                && w.workspace == key.workspace
                && w.target == key.target
        })
        .ok_or("missing accepted work")?;
    let t = state
        .targets
        .iter()
        .find(|t| t.owner == key.owner && t.workspace == key.workspace && t.id == key.target)
        .ok_or("missing target")?;
    if w.status != Status::Active
        || w.revision != key.revision
        || w.surface != key.surface
        || !t.open
        || t.revision != key.revision
    {
        return Err("stale physician work".into());
    }
    Ok(())
}
/// One workspace's sources, each candidate once.
fn scoped(
    workspace: &str,
    sources: &[crate::physician_sources::SourceRevision],
) -> Result<(), String> {
    let mut ids = std::collections::HashSet::new();
    for source in sources {
        source.validate()?;
        if source.workspace != workspace || !ids.insert(source.candidate_id.clone()) {
            return Err("invalid source observation scope".into());
        }
    }
    Ok(())
}

fn work_mut<'a>(state: &'a mut State, key: &Work) -> Result<&'a mut Work, String> {
    state
        .work
        .iter_mut()
        .find(|w| {
            w.owner == key.owner
                && w.id == key.id
                && w.workspace == key.workspace
                && w.target == key.target
        })
        .ok_or("missing accepted work".into())
}

/// Minimal synchronous physician adapter used by focused persistent tests.
/// Real serving checks session authorization too; tools here fail closed rather
/// than granting memory/delegation authority to a reply revision.
pub struct PhysicianOwner<'a> {
    /// Existing physician state owner.
    pub session: &'a mut PhysicianSession,
    /// Accepted immutable work.
    pub work: Work,
}
impl Owner for PhysicianOwner<'_> {
    fn terminal(&self, response: &Response) -> bool {
        self.work.surface == Surface::ReplyRevision
            && response
                .calls
                .iter()
                .any(|c| c.name == "finish_reply_revision")
    }
    async fn check(&mut self) -> Result<(), String> {
        self.session.check(&self.work)
    }
    async fn record(&mut self, record: Record) -> Result<(), String> {
        self.session.record(&self.work, record)
    }
    async fn dispatch(&mut self, call: &Call, cancel: &Cancellation) -> Result<Value, String> {
        cancel.check()?;
        self.session.check(&self.work)?;
        Ok(json!({"error": "tool_not_authorized", "name": call.name}))
    }
    async fn complete(&mut self, response: &Response) -> Result<(), String> {
        if self.work.surface == Surface::ReplyRevision {
            let [call] = response.calls.as_slice() else {
                return Err("reply revision requires one terminal call".into());
            };
            if call.name != "finish_reply_revision" || call.arguments.get("review_basis").is_none()
            {
                return Err("invalid reply revision completion".into());
            }
            let text = call
                .arguments
                .get("text")
                .and_then(Value::as_str)
                .ok_or("missing revised text")?;
            return self.session.complete(&self.work, text);
        }
        self.session.complete(&self.work, &response.text)
    }
}
