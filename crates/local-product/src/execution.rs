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
    /// Exact physician-source observation pinned at admission.
    #[serde(default)]
    pub sources: Vec<crate::physician_sources::SourceRevision>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Target {
    workspace: String,
    id: String,
    revision: u64,
    open: bool,
    turns: Vec<(String, String)>,
    draft: Option<String>,
    restore_text: Option<String>,
    /// Unix seconds when the target was created; 0 on older session files.
    #[serde(default)]
    opened_at: u64,
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
        workspace: &str,
        target: &str,
        text: &str,
    ) -> Result<Option<Work>, String> {
        if [id, workspace, target, text]
            .iter()
            .any(|s| s.trim().is_empty())
        {
            return Err("invalid physician input".into());
        }
        self.change(|state| {
            if let Some(work) = state
                .work
                .iter()
                .find(|w| w.id == id && w.workspace == workspace && w.target == target)
            {
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
                .position(|t| t.workspace == workspace && t.id == target)
                .unwrap_or_else(|| {
                    state.targets.push(Target {
                        workspace: workspace.into(),
                        id: target.into(),
                        revision: 0,
                        open: true,
                        turns: vec![],
                        draft: None,
                        restore_text: None,
                        opened_at: unix_now(),
                    });
                    state.targets.len() - 1
                });
            let target_state = &mut state.targets[index];
            target_state.revision += 1;
            target_state.turns.push(("user".into(), text.into()));
            let work = Work {
                id: id.into(),
                workspace: workspace.into(),
                target: target.into(),
                revision: target_state.revision,
                surface: Surface::Chat,
                status: Status::Active,
                records: vec![],
                sources: vec![],
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
                .any(|t| t.workspace == workspace && t.id == target)
            {
                if expected != 0 {
                    return Err("stale draft".into());
                }
                state.targets.push(Target {
                    workspace: workspace.into(),
                    id: target.into(),
                    revision: 0,
                    open: true,
                    turns: vec![],
                    draft: None,
                    restore_text: None,
                    opened_at: unix_now(),
                });
            }
            let t = state
                .targets
                .iter_mut()
                .find(|t| t.workspace == workspace && t.id == target)
                .unwrap();
            if !t.open || t.revision != expected {
                return Err("stale draft".into());
            }
            t.restore_text = t.draft.replace(text.into());
            t.revision += 1;
            Ok(t.revision)
        })
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
                w.workspace == workspace
                    && w.target == target
                    && (w.id == id || w.status == Status::Active)
            }) {
                return Err("reply revision already admitted".into());
            }
            let t = state
                .targets
                .iter()
                .find(|t| t.workspace == workspace && t.id == target)
                .ok_or("missing saved draft")?;
            if !t.open || t.revision != expected || t.draft.is_none() {
                return Err("stale draft".into());
            }
            let work = Work {
                id: id.into(),
                workspace: workspace.into(),
                target: target.into(),
                revision: expected,
                surface: Surface::ReplyRevision,
                request,
                status: Status::Active,
                records: vec![],
                sources: vec![],
            };
            state.work.push(work.clone());
            Ok(work)
        })
    }
    /// Pins the exact source observation before the first dispatch. A source
    /// candidate cannot name two different revisions within one observation.
    pub fn pin_sources(
        &mut self,
        key: &Work,
        sources: Vec<crate::physician_sources::SourceRevision>,
    ) -> Result<Work, String> {
        self.change(|state| {
            check(state, key)?;
            let work = work_mut(state, key)?;
            if !work.records.is_empty() {
                return Err("source observation already dispatched".into());
            }
            let mut ids = std::collections::HashSet::new();
            for source in &sources {
                source.validate()?;
                if source.workspace != work.workspace || !ids.insert(source.candidate_id.clone()) {
                    return Err("invalid source observation scope".into());
                }
            }
            work.sources = sources;
            Ok(work.clone())
        })
    }
    /// Closes this physician reply target. Active revisions fail and cannot
    /// later publish or resume; retained draft text is not replaced.
    pub fn close_reply(&mut self, workspace: &str, target: &str) -> Result<(), String> {
        self.change(|state| {
            let t = state
                .targets
                .iter_mut()
                .find(|t| t.workspace == workspace && t.id == target)
                .ok_or("missing reply target")?;
            t.open = false;
            for work in &mut state.work {
                if work.workspace == workspace
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
        self.state
            .work
            .iter()
            .find(|w| w.id == key.id && w.workspace == key.workspace && w.target == key.target)
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
                .find(|t| t.workspace == key.workspace && t.id == key.target)
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
    /// Physician-visible authored texts, never provider/tool evidence.
    pub fn conversation(&self, workspace: &str, target: &str) -> Vec<String> {
        self.state
            .targets
            .iter()
            .find(|t| t.workspace == workspace && t.id == target)
            .map(|t| t.turns.iter().map(|(_, text)| text.clone()).collect())
            .unwrap_or_default()
    }
    /// Chats of a workspace that have a physician turn, newest opened first,
    /// each with its first physician message and opening Unix second. Reply
    /// drafts have no turns and are not chats.
    pub fn chats(&self, workspace: &str) -> Vec<(String, String, u64)> {
        self.state
            .targets
            .iter()
            .rev()
            .filter(|t| t.workspace == workspace)
            .filter_map(|t| {
                let (_, first) = t.turns.iter().find(|(role, _)| role == "user")?;
                Some((t.id.clone(), first.clone(), t.opened_at))
            })
            .collect()
    }
    /// Saved draft revision, current text, and replaced text.
    pub fn draft(&self, workspace: &str, target: &str) -> Option<(u64, String, Option<String>)> {
        let t = self
            .state
            .targets
            .iter()
            .find(|t| t.workspace == workspace && t.id == target)?;
        Some((t.revision, t.draft.clone()?, t.restore_text.clone()))
    }
}
fn check(state: &State, key: &Work) -> Result<(), String> {
    let w = state
        .work
        .iter()
        .find(|w| w.id == key.id && w.workspace == key.workspace && w.target == key.target)
        .ok_or("missing accepted work")?;
    let t = state
        .targets
        .iter()
        .find(|t| t.workspace == key.workspace && t.id == key.target)
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
fn work_mut<'a>(state: &'a mut State, key: &Work) -> Result<&'a mut Work, String> {
    state
        .work
        .iter_mut()
        .find(|w| w.id == key.id && w.workspace == key.workspace && w.target == key.target)
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
