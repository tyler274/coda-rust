//! Staff-only inspection over persisted harness runs, projected steps, and
//! model reasoning summaries across chat, practice jobs, and reply revisions.

use std::collections::{BTreeMap, BTreeSet};

use harness::engine::{Event, EventKind};
use sea_orm::{ColumnTrait, Condition, DbErr, EntityTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

use super::entities::{
    chat_attempt, conversation, execution_event, practice_job, practice_job_attempt,
    revision_attempt, source,
};
use super::history::cited_record_title;
use super::records::Owner;
use super::{AttemptStatus, Store, conversion_error};
use crate::physician_sources::SourceRevision;
use crate::step_projection::{
    ProjectedStep, ProjectedStepState, ReasoningSummary, extract_reasoning_summaries,
    project_steps_from_events,
};

/// Owning runtime lane of a saved execution run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunOwnerKind {
    /// Interactive physician chat conversation (`Owner::Chat`).
    Chat,
    /// Practice-owned background job (`Owner::PracticeJob`).
    PracticeJob,
    /// Reply draft revision workflow (`Owner::Revision`).
    Revision,
}

impl From<Owner> for RunOwnerKind {
    fn from(value: Owner) -> Self {
        match value {
            Owner::Chat => Self::Chat,
            Owner::PracticeJob => Self::PracticeJob,
            Owner::Revision => Self::Revision,
        }
    }
}

/// Optional filters narrowing which saved runs [`Store::inspect_saved_runs`] returns.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SavedRunFilter {
    /// Restrict to one owning practice ID.
    pub practice: Option<String>,
    /// Restrict to one patient workspace ID.
    pub workspace: Option<String>,
    /// Restrict to one chat, reply, or practice job target ID.
    pub target: Option<String>,
}

/// Persisted attempt summary included in a [`SavedRunInspection`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedAttemptInspection {
    /// Attempt identifier.
    pub attempt_id: String,
    /// Work item identifier this attempt executed for.
    pub work: String,
    /// Monotonic work revision number.
    pub revision: u64,
    /// Terminal or current attempt status.
    pub status: AttemptStatus,
    /// Number of harness events saved by this attempt.
    pub event_count: usize,
}

/// Staff-only view of one saved runtime run, including raw events, projected
/// physician-facing steps, and model reasoning summaries.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SavedRunInspection {
    /// Owning lane (`chat`, `practice_job`, or `revision`).
    pub owner: RunOwnerKind,
    /// Owning practice identifier.
    pub practice: String,
    /// Owning physician identifier (empty string for practice-scoped jobs).
    pub physician: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Target chat, reply, or job identifier.
    pub target: String,
    /// Background job kind (`patient_review`, `spruce_matching`, etc.) when applicable.
    pub kind: Option<String>,
    /// Current saved workflow status (`active`, `held`, `completed`, `failed`, etc.).
    pub status: Option<String>,
    /// Ordered attempts known for this run.
    pub attempts: Vec<SavedAttemptInspection>,
    /// Physician-readable steps projected from all recorded tool calls.
    pub projected_steps: Vec<ProjectedStep>,
    /// Staff-only model reasoning summaries extracted from `ProviderReturned` items.
    pub reasoning_summaries: Vec<ReasoningSummary>,
    /// Complete ordered harness events saved for this run.
    pub raw_events: Vec<Event>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RunKey {
    owner: RunOwnerKind,
    practice: String,
    physician: String,
    workspace: String,
    target: String,
}

struct AttemptMeta {
    position: i64,
    work: String,
    revision: u64,
    status: AttemptStatus,
}

fn unfinished_state_for_attempt(status: AttemptStatus) -> ProjectedStepState {
    match status {
        AttemptStatus::Active => ProjectedStepState::Running,
        AttemptStatus::Cancelled => ProjectedStepState::Stopped,
        AttemptStatus::Failed => ProjectedStepState::Failed,
        AttemptStatus::Completed => ProjectedStepState::Done,
        AttemptStatus::Interrupted | AttemptStatus::Held => ProjectedStepState::Interrupted,
    }
}

impl Store {
    /// Inspects saved run records, projected physician steps, and model
    /// reasoning summaries for Coda staff without altering any state.
    pub async fn inspect_saved_runs(
        &self,
        filter: &SavedRunFilter,
    ) -> Result<Vec<SavedRunInspection>, DbErr> {
        let mut cond = Condition::all();
        if let Some(practice) = filter.practice.as_deref() {
            cond = cond.add(execution_event::Column::Practice.eq(practice));
        }
        if let Some(workspace) = filter.workspace.as_deref() {
            cond = cond.add(execution_event::Column::Workspace.eq(workspace));
        }
        if let Some(target) = filter.target.as_deref() {
            cond = cond.add(execution_event::Column::Target.eq(target));
        }

        let event_rows = execution_event::Entity::find()
            .filter(cond)
            .order_by_asc(execution_event::Column::Practice)
            .order_by_asc(execution_event::Column::Workspace)
            .order_by_asc(execution_event::Column::Target)
            .order_by_asc(execution_event::Column::Sequence)
            .order_by_asc(execution_event::Column::Ordinal)
            .all(&self.db)
            .await?;

        let mut grouped_events: BTreeMap<RunKey, Vec<Event>> = BTreeMap::new();
        let mut workspaces: BTreeSet<String> = BTreeSet::new();
        for row in event_rows {
            let kind: EventKind = serde_json::from_value(row.record)
                .map_err(conversion_error("execution_event.record", "EventKind"))?;
            let sequence = u64::try_from(row.sequence)
                .map_err(conversion_error("execution_event.sequence", "u64"))?;
            let ordinal = u64::try_from(row.ordinal)
                .map_err(conversion_error("execution_event.ordinal", "u64"))?;
            workspaces.insert(row.workspace.clone());
            grouped_events
                .entry(RunKey {
                    owner: RunOwnerKind::from(row.owner),
                    practice: row.practice,
                    physician: row.physician,
                    workspace: row.workspace,
                    target: row.target,
                })
                .or_default()
                .push(Event {
                    attempt_id: row.attempt,
                    sequence,
                    ordinal,
                    kind,
                });
        }

        if grouped_events.is_empty() {
            return Ok(Vec::new());
        }

        let titles_by_workspace = load_workspace_titles(&self.db, &workspaces).await?;
        let job_meta = load_practice_job_meta(&self.db, filter).await?;
        let conv_meta = load_conversation_meta(&self.db, filter).await?;
        let chat_attempts = load_chat_attempt_meta(&self.db, filter).await?;
        let revision_attempts = load_revision_attempt_meta(&self.db, filter).await?;
        let job_attempts = load_job_attempt_meta(&self.db, filter).await?;

        let mut inspections = Vec::with_capacity(grouped_events.len());
        for (key, raw_events) in grouped_events {
            let (kind, status, attempts_map) = match key.owner {
                RunOwnerKind::Chat => {
                    let status = conv_meta.get(&key).cloned();
                    (
                        Some("physician_chat".to_owned()),
                        status,
                        chat_attempts.get(&key),
                    )
                }
                RunOwnerKind::PracticeJob => {
                    let (kind, status) = job_meta
                        .get(&(
                            key.practice.clone(),
                            key.workspace.clone(),
                            key.target.clone(),
                        ))
                        .cloned()
                        .unwrap_or((None, None));
                    (kind, status, job_attempts.get(&key))
                }
                RunOwnerKind::Revision => (
                    Some("reply_revision".to_owned()),
                    None,
                    revision_attempts.get(&key),
                ),
            };

            let mut events_by_attempt: BTreeMap<String, Vec<Event>> = BTreeMap::new();
            let mut event_attempt_order: Vec<String> = Vec::new();
            for event in &raw_events {
                if !events_by_attempt.contains_key(&event.attempt_id) {
                    event_attempt_order.push(event.attempt_id.clone());
                }
                events_by_attempt
                    .entry(event.attempt_id.clone())
                    .or_default()
                    .push(event.clone());
            }

            let mut ordered_attempt_ids: Vec<String> = attempts_map
                .map(|m| {
                    let mut pairs: Vec<(&String, &AttemptMeta)> = m.iter().collect();
                    pairs.sort_by_key(|(_, meta)| meta.position);
                    pairs.into_iter().map(|(id, _)| id.clone()).collect()
                })
                .unwrap_or_default();
            for id in event_attempt_order {
                if !ordered_attempt_ids.contains(&id) {
                    ordered_attempt_ids.push(id);
                }
            }

            let mut attempts = Vec::with_capacity(ordered_attempt_ids.len());
            let mut projection_inputs: Vec<(ProjectedStepState, Vec<Event>)> =
                Vec::with_capacity(ordered_attempt_ids.len());

            for attempt_id in &ordered_attempt_ids {
                let events = events_by_attempt.remove(attempt_id).unwrap_or_default();
                let meta = attempts_map.and_then(|m| m.get(attempt_id));
                let attempt_status = meta.map_or(AttemptStatus::Interrupted, |m| m.status);
                attempts.push(SavedAttemptInspection {
                    attempt_id: attempt_id.clone(),
                    work: meta.map_or_else(String::new, |m| m.work.clone()),
                    revision: meta.map_or(0, |m| m.revision),
                    status: attempt_status,
                    event_count: events.len(),
                });
                projection_inputs.push((unfinished_state_for_attempt(attempt_status), events));
            }

            let ws_titles = titles_by_workspace.get(&key.workspace);
            let resolve_title = |candidate_id: &str| -> Option<String> {
                ws_titles.and_then(|titles| titles.get(candidate_id).cloned())
            };
            let projected_steps = project_steps_from_events(
                projection_inputs
                    .iter()
                    .map(|(state, events)| (*state, events.as_slice())),
                &resolve_title,
            );
            let reasoning_summaries = extract_reasoning_summaries(&raw_events);

            inspections.push(SavedRunInspection {
                owner: key.owner,
                practice: key.practice,
                physician: key.physician,
                workspace: key.workspace,
                target: key.target,
                kind,
                status,
                attempts,
                projected_steps,
                reasoning_summaries,
                raw_events,
            });
        }

        Ok(inspections)
    }
}

async fn load_workspace_titles(
    db: &impl sea_orm::ConnectionTrait,
    workspaces: &BTreeSet<String>,
) -> Result<BTreeMap<String, BTreeMap<String, String>>, DbErr> {
    let ws_list: Vec<String> = workspaces.iter().cloned().collect();
    if ws_list.is_empty() {
        return Ok(BTreeMap::new());
    }
    let rows = source::Entity::find()
        .filter(source::Column::Workspace.is_in(ws_list))
        .order_by_asc(source::Column::ObservedAtVersion)
        .all(db)
        .await?;
    let mut out: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    for row in rows {
        let Some(raw) = row.revision else {
            continue;
        };
        let rev: SourceRevision = serde_json::from_value(raw)
            .map_err(conversion_error("source.revision", "SourceRevision"))?;
        let title = cited_record_title(&rev.provenance, &rev.record_type, &rev.candidate_id);
        out.entry(row.workspace)
            .or_default()
            .insert(row.candidate, title);
    }
    Ok(out)
}

async fn load_practice_job_meta(
    db: &impl sea_orm::ConnectionTrait,
    filter: &SavedRunFilter,
) -> Result<BTreeMap<(String, String, String), (Option<String>, Option<String>)>, DbErr> {
    let mut cond = Condition::all();
    if let Some(p) = filter.practice.as_deref() {
        cond = cond.add(practice_job::Column::Practice.eq(p));
    }
    if let Some(w) = filter.workspace.as_deref() {
        cond = cond.add(practice_job::Column::Workspace.eq(w));
    }
    if let Some(t) = filter.target.as_deref() {
        cond = cond.add(practice_job::Column::JobId.eq(t));
    }
    let rows = practice_job::Entity::find().filter(cond).all(db).await?;
    let mut map = BTreeMap::new();
    for row in rows {
        let kind = serde_json::to_value(row.kind)
            .ok()
            .and_then(|v| v.as_str().map(str::to_owned));
        let status = serde_json::to_value(row.status)
            .ok()
            .and_then(|v| v.as_str().map(str::to_owned));
        map.insert((row.practice, row.workspace, row.job_id), (kind, status));
    }
    Ok(map)
}

async fn load_conversation_meta(
    db: &impl sea_orm::ConnectionTrait,
    filter: &SavedRunFilter,
) -> Result<BTreeMap<RunKey, String>, DbErr> {
    let mut cond = Condition::all();
    if let Some(p) = filter.practice.as_deref() {
        cond = cond.add(conversation::Column::Practice.eq(p));
    }
    if let Some(w) = filter.workspace.as_deref() {
        cond = cond.add(conversation::Column::Workspace.eq(w));
    }
    if let Some(t) = filter.target.as_deref() {
        cond = cond.add(conversation::Column::Chat.eq(t));
    }
    let rows = conversation::Entity::find().filter(cond).all(db).await?;
    let mut map = BTreeMap::new();
    for row in rows {
        if let Some(status) = serde_json::to_value(row.status)
            .ok()
            .and_then(|v| v.as_str().map(str::to_owned))
        {
            map.insert(
                RunKey {
                    owner: RunOwnerKind::Chat,
                    practice: row.practice,
                    physician: row.physician,
                    workspace: row.workspace,
                    target: row.chat,
                },
                status,
            );
        }
    }
    Ok(map)
}

async fn load_chat_attempt_meta(
    db: &impl sea_orm::ConnectionTrait,
    filter: &SavedRunFilter,
) -> Result<BTreeMap<RunKey, BTreeMap<String, AttemptMeta>>, DbErr> {
    let mut cond = Condition::all();
    if let Some(p) = filter.practice.as_deref() {
        cond = cond.add(chat_attempt::Column::Practice.eq(p));
    }
    if let Some(w) = filter.workspace.as_deref() {
        cond = cond.add(chat_attempt::Column::Workspace.eq(w));
    }
    if let Some(t) = filter.target.as_deref() {
        cond = cond.add(chat_attempt::Column::Chat.eq(t));
    }
    let rows = chat_attempt::Entity::find().filter(cond).all(db).await?;
    let mut map: BTreeMap<RunKey, BTreeMap<String, AttemptMeta>> = BTreeMap::new();
    for row in rows {
        let rev = u64::try_from(row.revision)
            .map_err(conversion_error("chat_attempt.revision", "u64"))?;
        map.entry(RunKey {
            owner: RunOwnerKind::Chat,
            practice: row.practice,
            physician: row.physician,
            workspace: row.workspace,
            target: row.chat,
        })
        .or_default()
        .insert(
            row.id,
            AttemptMeta {
                position: row.position,
                work: row.work,
                revision: rev,
                status: row.status,
            },
        );
    }
    Ok(map)
}

async fn load_revision_attempt_meta(
    db: &impl sea_orm::ConnectionTrait,
    filter: &SavedRunFilter,
) -> Result<BTreeMap<RunKey, BTreeMap<String, AttemptMeta>>, DbErr> {
    let mut cond = Condition::all();
    if let Some(p) = filter.practice.as_deref() {
        cond = cond.add(revision_attempt::Column::Practice.eq(p));
    }
    if let Some(w) = filter.workspace.as_deref() {
        cond = cond.add(revision_attempt::Column::Workspace.eq(w));
    }
    if let Some(t) = filter.target.as_deref() {
        cond = cond.add(revision_attempt::Column::Reply.eq(t));
    }
    let rows = revision_attempt::Entity::find()
        .filter(cond)
        .all(db)
        .await?;
    let mut map: BTreeMap<RunKey, BTreeMap<String, AttemptMeta>> = BTreeMap::new();
    for row in rows {
        let rev = u64::try_from(row.position)
            .map_err(conversion_error("revision_attempt.position", "u64"))?;
        map.entry(RunKey {
            owner: RunOwnerKind::Revision,
            practice: row.practice,
            physician: row.physician,
            workspace: row.workspace,
            target: row.reply,
        })
        .or_default()
        .insert(
            row.id,
            AttemptMeta {
                position: row.position,
                work: row.input_id,
                revision: rev,
                status: row.status,
            },
        );
    }
    Ok(map)
}

async fn load_job_attempt_meta(
    db: &impl sea_orm::ConnectionTrait,
    filter: &SavedRunFilter,
) -> Result<BTreeMap<RunKey, BTreeMap<String, AttemptMeta>>, DbErr> {
    let mut cond = Condition::all();
    if let Some(p) = filter.practice.as_deref() {
        cond = cond.add(practice_job_attempt::Column::Practice.eq(p));
    }
    if let Some(w) = filter.workspace.as_deref() {
        cond = cond.add(practice_job_attempt::Column::Workspace.eq(w));
    }
    if let Some(t) = filter.target.as_deref() {
        cond = cond.add(practice_job_attempt::Column::JobId.eq(t));
    }
    let rows = practice_job_attempt::Entity::find()
        .filter(cond)
        .all(db)
        .await?;
    let mut map: BTreeMap<RunKey, BTreeMap<String, AttemptMeta>> = BTreeMap::new();
    for row in rows {
        let rev = u64::try_from(row.revision)
            .map_err(conversion_error("practice_job_attempt.revision", "u64"))?;
        map.entry(RunKey {
            owner: RunOwnerKind::PracticeJob,
            practice: row.practice,
            physician: String::new(),
            workspace: row.workspace,
            target: row.job_id,
        })
        .or_default()
        .insert(
            row.id,
            AttemptMeta {
                position: row.position,
                work: row.work,
                revision: rev,
                status: row.status,
            },
        );
    }
    Ok(map)
}
