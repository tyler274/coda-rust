//! Practice-owned background jobs in Chords.
//!
//! Reviews, visit preparation, and initial understanding execute for one
//! practice and one registered patient workspace with no physician session.
//! Every background job runs on the shared run store and harness background
//! lane (`ModelRoles.background`), preloads its assignment skills at acceptance,
//! refuses physician-only actions, and records every event and saved result
//! under the practice and job.

use async_trait::async_trait;
use harness::engine::recipe::{Finish, Provider, RequestBinding, RequestRecipe};
use harness::engine::{
    self, Event, EventKind, Invocation, Item, Product, Tool, ToolOutcome, Turn,
    runs::{HoldReason, RunHistory, RunOutcome},
};
use sea_orm::sea_query::StringLen;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect, Set, TransactionTrait, sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

use super::delivery::{AttemptSettlement, Delivery, DeliveryKind, DeliveryState};
use super::entities::{practice_job, practice_job_attempt};
use super::receipt::{self, Receipt};
use super::records::{self, Owner, OwnerKey};
use super::{
    Accepted, AttemptFailure, AttemptStatus, BindingRef, Lease, Resumed, RuntimeError, Scope,
    SkillPackageId, Stopped, Store, WorkStatus, control, conversion_error, error, frozen, now,
    persistence_failure, settle_attempt, working_notes,
};
use crate::physician_chat::{Job, PhysicianSkills, SKILL_VIEW, work_tools};
use crate::physician_sources::{
    ChartScope, READ_TOOL, SEARCH_TOOL, SourceRevision, read_tool, search_tool,
};

/// Reserved non-physician actor key for shared runtime recipe, skill, receipt,
/// and event tables. Physician-scoped entry points refuse this actor.
pub const PRACTICE_OWNER: &str = "__practice__";

fn terminal_answer(turn: &Turn) -> Result<String, DbErr> {
    let mut answer = String::new();
    for item in &turn.items {
        match item {
            Item::Call(_) => return Err(error("terminal turn requested a tool")),
            Item::Message { role, text } if role == "assistant" => answer.push_str(text),
            _ => {}
        }
    }
    if answer.trim().is_empty() {
        return Err(error("terminal turn has no authored answer"));
    }
    Ok(answer)
}

/// Clinical instructions for practice-owned background runs.
pub const INSTRUCTIONS: &str = "\
You are running a practice-owned clinical background job for one patient in this practice. \
No physician is signed in to this run.

Carry out the assigned clinical review, visit preparation, or initial patient understanding \
using only the dated patient source records in this patient workspace and the approved skill \
guidance loaded for this assignment. Discover candidate records in the patient workspace and \
read the exact source revisions before stating patient-specific facts. Cite dated sources \
plainly and distinguish partial reads or missing records from confirmed facts.

You act only as a practice background analyst, never as a physician. You cannot send messages, \
edit physician drafts, record physician feedback, approve plans, or perform any physician-only \
action. Leave clinical decisions and patient communication to the physician.

When you rely on general medical knowledge, state that plainly and never present it as retrieved \
chart evidence. Use unambiguous calendar dates with the year. Close with a concise clinical \
basis naming the dated source records and skill guidance used and any material limitation. \
Use clinical language, never runtime terminology, model names, schemas, or internal identifiers.";

/// Which practice-owned background job an assignment runs.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    sea_orm::EnumIter,
    sea_orm::DeriveActiveEnum,
)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum PracticeJobKind {
    /// Background patient chart review triggered by source changes or periodic schedule.
    #[sea_orm(string_value = "patient_review")]
    PatientReview,
    /// Pre-visit chart synthesis for an upcoming encounter.
    #[sea_orm(string_value = "visit_preparation")]
    VisitPreparation,
    /// Initial clinical understanding when a patient chart enters the practice.
    #[sea_orm(string_value = "initial_understanding")]
    InitialUnderstanding,
}

impl PracticeJobKind {
    /// Canonical snake_case identifier for cache keys and attribution records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PatientReview => "patient_review",
            Self::VisitPreparation => "visit_preparation",
            Self::InitialUnderstanding => "initial_understanding",
        }
    }
}

/// Practice-scoped identity of one background job for one patient workspace.
/// Carries no physician identity and no session reference.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PracticeScope {
    /// Owning practice identifier.
    pub practice: String,
    /// Patient workspace identifier within `practice`.
    pub workspace: String,
    /// Practice-unique background job identifier within `workspace`.
    pub job_id: String,
}

impl PracticeScope {
    /// Refuses blank practice, workspace, or job identifiers.
    pub fn validate(&self) -> Result<(), DbErr> {
        if [&self.practice, &self.workspace, &self.job_id]
            .iter()
            .any(|part| part.trim().is_empty())
        {
            return Err(error("empty practice job scope"));
        }
        Ok(())
    }

    /// Stable prompt-cache key partitioned strictly to this practice, patient,
    /// job kind, and job identity.
    pub fn cache_key(&self, kind: PracticeJobKind) -> harness::PromptCacheKey {
        harness::PromptCacheKey::scoped(
            "chords-practice-job",
            &[&self.practice, &self.workspace, kind.as_str(), &self.job_id],
        )
    }

    /// Control-plane [`Scope`] representation for calling the shared run store's
    /// [`Store::stop_authorized`] and [`Store::resume_authorized`] controls.
    /// Physician-only endpoints refuse [`PRACTICE_OWNER`].
    pub fn as_control_scope(&self) -> Scope {
        Scope {
            practice: self.practice.clone(),
            physician: PRACTICE_OWNER.to_owned(),
            workspace: self.workspace.clone(),
            chat: self.job_id.clone(),
        }
    }

    /// Parses a [`DeliveryKind::PracticeJob`] control scope back into its
    /// [`PracticeScope`], refusing any scope whose `physician` is not
    /// [`PRACTICE_OWNER`] or whose identifiers are blank.
    pub fn from_delivery_scope(scope: &Scope) -> Result<Self, DbErr> {
        if scope.physician != PRACTICE_OWNER {
            return Err(error("practice job delivery carries non-practice actor"));
        }
        let parsed = Self {
            practice: scope.practice.clone(),
            workspace: scope.workspace.clone(),
            job_id: scope.chat.clone(),
        };
        parsed.validate()?;
        Ok(parsed)
    }

    fn owner_key(&self) -> OwnerKey<'_> {
        OwnerKey {
            practice: &self.practice,
            actor: PRACTICE_OWNER,
            workspace: &self.workspace,
            target: &self.job_id,
            owner: Owner::PracticeJob,
        }
    }
}

/// One skill preloaded at the start of a practice background job and recorded
/// durably with the run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreloadedSkill {
    /// Approved catalog skill name.
    pub name: String,
    /// Lowercase SHA-256 hex digest of the preloaded `SKILL.md` content.
    pub sha256: String,
    /// Full skill body loaded at assignment start.
    pub content: String,
}

/// Why an assignment's inputs or preloaded skills were refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AssignmentError {
    /// The command key or prompt text is blank.
    #[error("practice job assignment requires a non-empty key and prompt")]
    EmptyAssignment,
    /// A requested skill name could not be loaded from the approved catalog.
    #[error("assignment skill {name} could not be preloaded: {reason}")]
    UnavailableSkill {
        /// Skill name requested by the assignment.
        name: String,
        /// Catalog refusal or unavailability message.
        reason: String,
    },
}

/// The work and preloaded skills assigned to one practice background run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assignment {
    /// Which kind of practice background job this assignment runs.
    pub kind: PracticeJobKind,
    /// Once-only command key for accepting this job.
    pub key: String,
    /// Assignment instructions naming the clinical goal for this patient.
    pub prompt: String,
    /// Skills loaded from [`PhysicianSkills`] at assignment creation.
    pub preloaded_skills: Vec<PreloadedSkill>,
    /// Optional structured context when this assignment runs initial or revised
    /// patient understanding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub understanding: Option<crate::initial_understanding::UnderstandingContext>,
    /// Optional structured context when this assignment runs a background or
    /// physician-requested patient review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<crate::patient_reviews::ReviewAssignmentContext>,
    /// Optional structured context when this assignment selects upcoming visits
    /// and preparation times from a practice schedule wake-up.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visit_selection: Option<crate::visit_preparation::VisitSelectionContext>,
    /// Optional structured context when this assignment prepares and saves one
    /// visit's clinical briefing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visit_briefing: Option<crate::visit_preparation::VisitBriefingContext>,
}

impl Assignment {
    /// Validates `key` and `prompt`, loads each named skill from `skills` right
    /// away, and records its exact content and digest on the assignment.
    pub fn new(
        kind: PracticeJobKind,
        key: impl Into<String>,
        prompt: impl Into<String>,
        skill_names: &[&str],
        skills: &PhysicianSkills,
    ) -> Result<Self, AssignmentError> {
        let key = key.into();
        let prompt = prompt.into();
        if key.trim().is_empty() || prompt.trim().is_empty() {
            return Err(AssignmentError::EmptyAssignment);
        }
        let mut seen = std::collections::BTreeSet::new();
        let mut preloaded_skills = Vec::with_capacity(skill_names.len());
        for raw_name in skill_names {
            let name = raw_name.trim();
            if name.is_empty() {
                return Err(AssignmentError::UnavailableSkill {
                    name: (*raw_name).to_owned(),
                    reason: "skill name cannot be empty".to_owned(),
                });
            }
            if !seen.insert(name.to_owned()) {
                continue;
            }
            preloaded_skills.push(preload_skill(skills, name)?);
        }
        Ok(Self {
            kind,
            key,
            prompt,
            preloaded_skills,
            understanding: None,
            review: None,
            visit_selection: None,
            visit_briefing: None,
        })
    }

    /// Fixed synthetic-patient assignment stub that preloads chart retrieval
    /// guidance and summarizes one synthetic patient's dated chart records.
    pub fn synthetic_patient_stub(
        workspace_id: &str,
        skills: &PhysicianSkills,
    ) -> Result<Self, AssignmentError> {
        let workspace = workspace_id.trim();
        if workspace.is_empty() {
            return Err(AssignmentError::EmptyAssignment);
        }
        Self::new(
            PracticeJobKind::InitialUnderstanding,
            format!("synthetic-initial-understanding:{workspace}"),
            format!(
                "Build an initial clinical understanding of patient workspace {workspace} \
                 from the practice's retained source records, citing each dated record."
            ),
            &["patient-record-retrieval"],
            skills,
        )
    }

    pub(super) fn message(&self) -> engine::ProductMessage {
        engine::ProductMessage {
            id: format!("{}:assignment", self.key),
            role: engine::history::Role::User,
            text: self.prompt.clone(),
        }
    }
}

fn preload_skill(skills: &PhysicianSkills, name: &str) -> Result<PreloadedSkill, AssignmentError> {
    match skills.execute(SKILL_VIEW, &json!({ "name": name })) {
        ToolOutcome::Returned(payload) => {
            let value: Value = serde_json::from_str(&payload).map_err(|err| {
                AssignmentError::UnavailableSkill {
                    name: name.to_owned(),
                    reason: err.to_string(),
                }
            })?;
            let status = value
                .get("status")
                .and_then(Value::as_str)
                .unwrap_or("unavailable");
            if status != "available" {
                let reason = value
                    .get("message")
                    .and_then(Value::as_str)
                    .unwrap_or("skill is not available in catalog")
                    .to_owned();
                return Err(AssignmentError::UnavailableSkill {
                    name: name.to_owned(),
                    reason,
                });
            }
            let content = value
                .get("content")
                .and_then(Value::as_str)
                .filter(|body| !body.trim().is_empty())
                .ok_or_else(|| AssignmentError::UnavailableSkill {
                    name: name.to_owned(),
                    reason: "skill body is empty".to_owned(),
                })?
                .to_owned();
            Ok(PreloadedSkill {
                name: name.to_owned(),
                sha256: crate::sha256_hex(content.as_bytes()),
                content,
            })
        }
        ToolOutcome::Refused(refusal) => Err(AssignmentError::UnavailableSkill {
            name: name.to_owned(),
            reason: refusal.reason().to_owned(),
        }),
        ToolOutcome::History(_) | ToolOutcome::Retained(_) => {
            Err(AssignmentError::UnavailableSkill {
                name: name.to_owned(),
                reason: "unexpected non-inline outcome from skill catalog".to_owned(),
            })
        }
    }
}

/// Practice and job attribution recorded on every saved background job effect.
/// Carries no physician identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticeEffectAttribution {
    /// Owning practice identifier.
    pub practice: String,
    /// Patient workspace identifier within `practice`.
    pub workspace: String,
    /// Background job identifier.
    pub job_id: String,
    /// Background job kind.
    pub kind: PracticeJobKind,
}

/// Whether a saved practice background output represents final completion or a
/// partial answer retained when the run held at a budget limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PracticeJobCompletion {
    /// The run finished normally and committed its terminal clinical output.
    Complete,
    /// The run held at a turn or budget limit and retained its partial output.
    Partial,
}

/// Durable result saved when a practice background run finishes or holds a
/// partial output.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticeJobResult {
    /// Practice and job ownership attribution (never a physician).
    pub attribution: PracticeEffectAttribution,
    /// Attempt that produced this output.
    pub attempt_id: String,
    /// Saved clinical output text.
    pub output_text: String,
    /// Whether this output completed the job or held partially at a limit.
    pub completion: PracticeJobCompletion,
    /// Workspace source version pinned when this output was recorded.
    pub sources_version: u64,
    /// Database-clock Unix seconds when the output was recorded.
    pub recorded_at: i64,
}

/// One attempt at a practice background job, with its retained execution events.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PracticeJobAttempt {
    /// Attempt identifier.
    pub id: String,
    /// Immediate predecessor attempt when recovered.
    pub predecessor: Option<String>,
    /// Work item identity this attempt serves.
    pub work: String,
    /// Job revision fenced by this attempt.
    pub revision: u64,
    /// Current attempt lifecycle status.
    pub status: AttemptStatus,
    /// Database-clock lease expiry in Unix seconds.
    pub expires: i64,
    /// Ordered execution events recorded under [`Owner::PracticeJob`].
    pub events: Vec<Event>,
}

/// Durable practice-owned background run record in the Chords run store.
#[derive(Clone, Debug)]
pub struct PracticeJob {
    /// Practice, patient workspace, and job identity.
    pub scope: PracticeScope,
    /// Which background job kind this run serves.
    pub kind: PracticeJobKind,
    /// Current run-store work status.
    pub status: WorkStatus,
    /// Work item identity acted on by Stop and Resume.
    pub work: String,
    /// Monotonic job revision.
    pub revision: u64,
    /// Accepted assignment for this run.
    pub assignment: Assignment,
    /// Preloaded skill bodies and digests recorded at start.
    pub preloaded_skills: Vec<PreloadedSkill>,
    /// Immutable skill package identity retained in the run store.
    pub skills: SkillPackageId,
    /// Frozen background-lane provider recipe binding.
    pub binding: BindingRef,
    /// Run history shared with Stop, Resume, and recovery.
    pub runs: RunHistory,
    /// Unsuccessful attempts in the current run.
    pub failures: u32,
    /// Allowance boundary recorded across Resume.
    pub allowance_after: u64,
    /// Pinned patient source version.
    pub sources_version: u64,
    /// Pinned patient source revisions.
    pub sources: Vec<SourceRevision>,
    /// Saved output, once completed or held with partial text.
    pub result: Option<PracticeJobResult>,
    /// Earliest database-clock Unix second when recovery may republish this wakeup.
    pub dispatch_after: i64,
    /// Ordered attempts on this practice job.
    pub attempts: Vec<PracticeJobAttempt>,
    /// Database-clock Unix seconds when accepted.
    pub created_at: i64,
    /// Database-clock Unix seconds when last updated.
    pub updated_at: i64,
}

impl PracticeJob {
    /// Current run state from the shared run history.
    pub fn current_run(&self) -> Option<harness::engine::runs::RunState<'_>> {
        self.runs.current()
    }

    pub(super) fn message_sources(&self) -> Vec<engine::ProductMessage> {
        let mut sources = vec![self.assignment.message()];
        if let Some(saved) = &self.result {
            sources.push(engine::ProductMessage {
                id: format!("{}:result", saved.attempt_id),
                role: engine::history::Role::Assistant,
                text: saved.output_text.clone(),
            });
        }
        sources
    }

    fn attempt_ids(&self) -> Vec<String> {
        self.attempts
            .iter()
            .map(|attempt| attempt.id.clone())
            .collect()
    }
}

/// Fenced lease claim on a practice background job.
#[derive(Clone, Debug)]
pub struct PracticeJobClaim {
    /// Ownership duration and renewal cadence.
    pub lease: Lease,
    /// Practice, patient workspace, and job identity.
    pub scope: PracticeScope,
    /// Background job kind.
    pub kind: PracticeJobKind,
    /// Claimed attempt identifier.
    pub attempt: String,
    /// Work item identity.
    pub work: String,
    /// Fenced job revision.
    pub revision: u64,
}

/// Why practice-only patient access was refused.
#[derive(Debug, thiserror::Error)]
pub enum PracticeAccessError {
    /// Practice, workspace, or job identifier was blank.
    #[error("practice job scope cannot contain blank identifiers")]
    InvalidScope,
    /// The auth store could not be reached.
    #[error("practice patient directory is unavailable: {0}")]
    StoreUnavailable(#[from] physician_auth::StoreUnavailable),
    /// The patient workspace is not registered in any practice.
    #[error("patient workspace {workspace} is not registered in practice {practice}")]
    UnregisteredPatient {
        /// Practice attempting the background run.
        practice: String,
        /// Unregistered workspace identifier.
        workspace: String,
    },
    /// The patient workspace belongs to a different practice.
    #[error(
        "practice {requested_practice} cannot access patient workspace {workspace} owned by another practice"
    )]
    ForeignPractice {
        /// Practice attempting the background run.
        requested_practice: String,
        /// Actual owning practice of `workspace`.
        owner_practice: String,
        /// Patient workspace identifier.
        workspace: String,
    },
}

/// Practice-level patient membership authority requiring no physician session.
#[async_trait]
pub trait PracticeAuthority: Send + Sync {
    /// Confirms that `scope.workspace` is registered to `scope.practice`.
    async fn check_practice_patient(
        &self,
        scope: &PracticeScope,
    ) -> Result<(), PracticeAccessError>;
}

#[async_trait]
impl<T: physician_auth::AuthStore + ?Sized> PracticeAuthority for T {
    async fn check_practice_patient(
        &self,
        scope: &PracticeScope,
    ) -> Result<(), PracticeAccessError> {
        if scope.validate().is_err() {
            return Err(PracticeAccessError::InvalidScope);
        }
        match self.patient_practice(&scope.workspace).await? {
            None => Err(PracticeAccessError::UnregisteredPatient {
                practice: scope.practice.clone(),
                workspace: scope.workspace.clone(),
            }),
            Some(owner) if owner != scope.practice => Err(PracticeAccessError::ForeignPractice {
                requested_practice: scope.practice.clone(),
                owner_practice: owner,
                workspace: scope.workspace.clone(),
            }),
            Some(_) => Ok(()),
        }
    }
}

/// Errors produced when accepting or claiming a practice background job.
#[derive(Debug, thiserror::Error)]
pub enum PracticeJobError {
    /// Practice patient access was refused.
    #[error("{0}")]
    Access(#[from] PracticeAccessError),
    /// Assignment validation failed.
    #[error("{0}")]
    Assignment(#[from] AssignmentError),
    /// Runtime persistence or lifecycle transition failed.
    #[error("{0}")]
    Runtime(#[from] RuntimeError),
}

impl From<DbErr> for PracticeJobError {
    fn from(source: DbErr) -> Self {
        Self::Runtime(RuntimeError::Database(source))
    }
}

/// Capabilities offered to a practice-owned background run: patient chart
/// search/read, on-demand source refresh, approved skill views, retained result
/// reads, and history.
pub fn tools() -> Vec<Tool> {
    let mut offered = work_tools();
    offered.push(search_tool());
    offered.push(read_tool(ChartScope::Chat));
    offered.push(super::refresh::refresh_tool());
    offered
}

/// Returns `true` when `name` is a physician-only action that a practice
/// background run must never execute.
pub fn is_physician_only_tool(name: &str) -> bool {
    matches!(
        name,
        super::reply::FINISH_TOOL
            | "reply_draft"
            | "open_reply_draft"
            | "save_reply_draft"
            | "chat_draft"
            | "save_chat_draft"
            | "response_feedback"
            | "save_feedback"
            | "send_patient_message"
            | "approve_doctor_plan"
            | "invite_patient"
    )
}

/// Builds the immutable [`RequestRecipe`] for a practice background job on the
/// harness background lane (`roles.background`), embedding the assignment's
/// preloaded skills at the start of the run.
pub fn recipe(
    roles: &crate::ModelRoles,
    assignment: &Assignment,
    skills: &PhysicianSkills,
) -> RequestRecipe {
    if assignment.kind == PracticeJobKind::InitialUnderstanding {
        return crate::initial_understanding::recipe(&roles.background, assignment, skills);
    }
    if assignment.kind == PracticeJobKind::PatientReview && assignment.review.is_some() {
        return crate::patient_reviews::recipe(&roles.background, assignment, skills);
    }
    if assignment.kind == PracticeJobKind::VisitPreparation {
        return crate::visit_preparation::recipe(&roles.background, assignment, skills);
    }
    let background = &roles.background;
    let mut instructions = skills.instructions(INSTRUCTIONS, Job::PracticeBackground);
    if !assignment.preloaded_skills.is_empty() {
        let preloaded: Vec<Value> = assignment
            .preloaded_skills
            .iter()
            .map(|skill| {
                json!({
                    "name": skill.name,
                    "sha256": skill.sha256,
                    "content": skill.content,
                })
            })
            .collect();
        let encoded = serde_json::to_string(&json!({
            "kind": "preloaded_assignment_skills",
            "skills": preloaded,
        }))
        .expect("preloaded skills serialize as JSON");
        instructions.push_str("\n\nPreloaded assignment skills (already open in this run; do not reload via skill_view):\n");
        instructions.push_str(&encoded);
    }
    let mut offered = tools();
    offered.push(super::concerns::concern_tool());
    let mut built = RequestRecipe {
        model: background.model.clone(),
        context_window_tokens: background.context_window_tokens,
        max_input_tokens: background.max_input_tokens,
        options: json!({ "effort": background.effort }),
        instructions,
        tools: offered.into_iter().map(|tool| tool.function()).collect(),
    };
    engine::offer_working_notes(&mut built.instructions, &mut built.tools);
    built
}

fn job_keys(scope: &PracticeScope) -> (String, String, String) {
    (
        scope.practice.clone(),
        scope.workspace.clone(),
        scope.job_id.clone(),
    )
}

fn attempt_scope_filter(scope: &PracticeScope) -> Condition {
    use practice_job_attempt::Column as C;
    Condition::all()
        .add(C::Practice.eq(scope.practice.as_str()))
        .add(C::Workspace.eq(scope.workspace.as_str()))
        .add(C::JobId.eq(scope.job_id.as_str()))
}

fn decode_job(
    row: practice_job::Model,
    attempts: Vec<PracticeJobAttempt>,
) -> Result<PracticeJob, DbErr> {
    let assignment: Assignment = serde_json::from_value(row.assignment)
        .map_err(conversion_error("practice_job.assignment", "Assignment"))?;
    let preloaded_skills: Vec<PreloadedSkill> = serde_json::from_value(row.preloaded_skills)
        .map_err(conversion_error(
            "practice_job.preloaded_skills",
            "Vec<PreloadedSkill>",
        ))?;
    let skills: SkillPackageId = serde_json::from_value(Value::String(row.skills))
        .map_err(conversion_error("practice_job.skills", "SkillPackageId"))?;
    let binding: BindingRef = serde_json::from_value(row.binding)
        .map_err(conversion_error("practice_job.binding", "BindingRef"))?;
    let runs: RunHistory = serde_json::from_value(row.runs)
        .map_err(conversion_error("practice_job.runs", "RunHistory"))?;
    let sources: Vec<SourceRevision> = serde_json::from_value(row.sources).map_err(
        conversion_error("practice_job.sources", "Vec<SourceRevision>"),
    )?;
    let result: Option<PracticeJobResult> = row
        .result
        .map(|value| {
            serde_json::from_value(value)
                .map_err(conversion_error("practice_job.result", "PracticeJobResult"))
        })
        .transpose()?;
    Ok(PracticeJob {
        scope: PracticeScope {
            practice: row.practice,
            workspace: row.workspace,
            job_id: row.job_id,
        },
        kind: row.kind,
        status: row.status,
        work: row.work,
        revision: u64::try_from(row.revision)
            .map_err(conversion_error("practice_job.revision", "u64"))?,
        assignment,
        preloaded_skills,
        skills,
        binding,
        runs,
        failures: u32::try_from(row.failures)
            .map_err(conversion_error("practice_job.failures", "u32"))?,
        allowance_after: u64::try_from(row.allowance_after)
            .map_err(conversion_error("practice_job.allowance_after", "u64"))?,
        sources_version: u64::try_from(row.sources_version)
            .map_err(conversion_error("practice_job.sources_version", "u64"))?,
        sources,
        result,
        dispatch_after: row.dispatch_after,
        attempts,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

async fn load_attempts(
    db: &impl ConnectionTrait,
    scope: &PracticeScope,
) -> Result<Vec<PracticeJobAttempt>, DbErr> {
    let rows = practice_job_attempt::Entity::find()
        .filter(attempt_scope_filter(scope))
        .order_by_asc(practice_job_attempt::Column::Position)
        .all(db)
        .await?;
    rows.into_iter()
        .map(|row| {
            Ok(PracticeJobAttempt {
                id: row.id,
                predecessor: row.predecessor,
                work: row.work,
                revision: u64::try_from(row.revision)
                    .map_err(conversion_error("practice_job_attempt.revision", "u64"))?,
                status: row.status,
                expires: row.expires,
                events: Vec::new(),
            })
        })
        .collect()
}

async fn load_attempt_events(
    db: &impl ConnectionTrait,
    scope: &PracticeScope,
    attempts: &mut [PracticeJobAttempt],
) -> Result<(), DbErr> {
    let ids: Vec<String> = attempts.iter().map(|attempt| attempt.id.clone()).collect();
    let mut by_attempt = records::load_for(db, &scope.owner_key(), &ids, |event, _| event).await?;
    for attempt in attempts {
        attempt.events = by_attempt.remove(&attempt.id).unwrap_or_default();
    }
    Ok(())
}

async fn save_attempts(
    tx: &impl ConnectionTrait,
    scope: &PracticeScope,
    attempts: &[PracticeJobAttempt],
) -> Result<(), DbErr> {
    for (position, attempt) in attempts.iter().enumerate() {
        let position =
            i64::try_from(position).map_err(conversion_error("attempt position", "i64"))?;
        let revision =
            i64::try_from(attempt.revision).map_err(conversion_error("attempt revision", "i64"))?;
        practice_job_attempt::Entity::insert(practice_job_attempt::ActiveModel {
            practice: Set(scope.practice.clone()),
            workspace: Set(scope.workspace.clone()),
            job_id: Set(scope.job_id.clone()),
            id: Set(attempt.id.clone()),
            position: Set(position),
            predecessor: Set(attempt.predecessor.clone()),
            work: Set(attempt.work.clone()),
            revision: Set(revision),
            status: Set(attempt.status),
            expires: Set(attempt.expires),
        })
        .on_conflict(
            OnConflict::columns([
                practice_job_attempt::Column::Practice,
                practice_job_attempt::Column::Workspace,
                practice_job_attempt::Column::JobId,
                practice_job_attempt::Column::Id,
            ])
            .update_columns([
                practice_job_attempt::Column::Status,
                practice_job_attempt::Column::Expires,
            ])
            .to_owned(),
        )
        .exec_without_returning(tx)
        .await?;
    }
    Ok(())
}

fn job_active_model(job: &PracticeJob) -> Result<practice_job::ActiveModel, DbErr> {
    let skills_str = serde_json::to_value(&job.skills)
        .map_err(conversion_error("SkillPackageId", "practice_job.skills"))?
        .as_str()
        .ok_or_else(|| error("skill package id must serialize as text"))?
        .to_owned();
    Ok(practice_job::ActiveModel {
        practice: Set(job.scope.practice.clone()),
        workspace: Set(job.scope.workspace.clone()),
        job_id: Set(job.scope.job_id.clone()),
        kind: Set(job.kind),
        status: Set(job.status),
        work: Set(job.work.clone()),
        revision: Set(i64::try_from(job.revision).map_err(conversion_error("job.revision", "i64"))?),
        assignment: Set(serde_json::to_value(&job.assignment)
            .map_err(conversion_error("Assignment", "practice_job.assignment"))?),
        preloaded_skills: Set(serde_json::to_value(&job.preloaded_skills).map_err(
            conversion_error("Vec<PreloadedSkill>", "practice_job.preloaded_skills"),
        )?),
        skills: Set(skills_str),
        binding: Set(serde_json::to_value(&job.binding)
            .map_err(conversion_error("BindingRef", "practice_job.binding"))?),
        runs: Set(serde_json::to_value(&job.runs)
            .map_err(conversion_error("RunHistory", "practice_job.runs"))?),
        failures: Set(i64::from(job.failures)),
        allowance_after: Set(i64::try_from(job.allowance_after)
            .map_err(conversion_error("job.allowance_after", "i64"))?),
        sources_version: Set(i64::try_from(job.sources_version)
            .map_err(conversion_error("job.sources_version", "i64"))?),
        sources: Set(serde_json::to_value(&job.sources).map_err(conversion_error(
            "Vec<SourceRevision>",
            "practice_job.sources",
        ))?),
        result: Set(job
            .result
            .as_ref()
            .map(|res| {
                serde_json::to_value(res)
                    .map_err(conversion_error("PracticeJobResult", "practice_job.result"))
            })
            .transpose()?),
        dispatch_after: Set(job.dispatch_after),
        created_at: Set(job.created_at),
        updated_at: Set(job.updated_at),
    })
}

fn job_current(
    job: &PracticeJob,
    claim: &PracticeJobClaim,
    now: i64,
) -> Result<(), engine::NotCurrent> {
    control::current(
        job.attempts
            .last()
            .map(|attempt| (attempt.id.as_str(), attempt.status, attempt.expires)),
        &claim.attempt,
        job.revision == claim.revision && job.work == claim.work,
        now,
    )
}

pub(super) async fn decode_job_for_reviews(
    tx: &impl ConnectionTrait,
    scope: &PracticeScope,
    row: practice_job::Model,
) -> Result<PracticeJob, DbErr> {
    let attempts = load_attempts(tx, scope).await?;
    decode_job(row, attempts)
}

pub(super) async fn load_attempt_events_for_reviews(
    db: &impl ConnectionTrait,
    scope: &PracticeScope,
    attempts: &mut [PracticeJobAttempt],
) -> Result<(), DbErr> {
    load_attempt_events(db, scope, attempts).await
}

pub(super) async fn write_practice_job_in_tx(
    tx: &impl ConnectionTrait,
    job: &PracticeJob,
) -> Result<(), DbErr> {
    save_attempts(tx, &job.scope, &job.attempts).await?;
    job_active_model(job)?.update(tx).await?;
    Ok(())
}

impl Store {
    pub(super) async fn locked_practice_job(
        &self,
        scope: &PracticeScope,
    ) -> Result<(sea_orm::DatabaseTransaction, PracticeJob, i64), DbErr> {
        scope.validate()?;
        let tx = self.db.begin().await?;
        let row = practice_job::Entity::find_by_id(job_keys(scope))
            .lock_exclusive()
            .one(&tx)
            .await?
            .ok_or_else(|| error("practice job not found"))?;
        let attempts = load_attempts(&tx, scope).await?;
        let job = decode_job(row, attempts)?;
        let clock = now(&tx).await?;
        Ok((tx, job, clock))
    }

    async fn save_practice_job(
        tx: sea_orm::DatabaseTransaction,
        job: &PracticeJob,
    ) -> Result<(), DbErr> {
        write_practice_job_in_tx(&tx, job).await?;
        super::briefings::sync_visit_preparation_job_in_tx(&tx, job).await?;
        tx.commit().await
    }

    pub(super) async fn save_practice_job_in_tx(
        tx: sea_orm::DatabaseTransaction,
        job: &PracticeJob,
    ) -> Result<(), DbErr> {
        Self::save_practice_job(tx, job).await
    }

    /// Accepts one practice-owned background job for `(scope.practice, scope.workspace)`
    /// with no physician session, verifying practice patient membership, freezing
    /// the background-lane recipe (`roles.background`), and recording the
    /// assignment's preloaded skills with the run.
    pub async fn accept_practice_job<A: PracticeAuthority + ?Sized>(
        &self,
        scope: &PracticeScope,
        assignment: &Assignment,
        roles: &crate::ModelRoles,
        skills: &PhysicianSkills,
        authority: &A,
    ) -> Result<Accepted, PracticeJobError> {
        authority.check_practice_patient(scope).await?;
        if assignment.key.trim().is_empty() || assignment.prompt.trim().is_empty() {
            return Err(AssignmentError::EmptyAssignment.into());
        }
        // Re-verify every skill recorded on the assignment against the supplied
        // skill package so forged or mismatched preloads cannot enter the store.
        let mut verified_preloads = Vec::with_capacity(assignment.preloaded_skills.len());
        for skill in &assignment.preloaded_skills {
            let loaded = preload_skill(skills, &skill.name)?;
            if loaded != *skill {
                return Err(AssignmentError::UnavailableSkill {
                    name: skill.name.clone(),
                    reason: "preloaded skill content does not match skill package".to_owned(),
                }
                .into());
            }
            verified_preloads.push(loaded);
        }

        let built_recipe =
            recipe(roles, assignment, skills).bind(Some(scope.cache_key(assignment.kind)));
        let tx = self.db.begin().await?;
        let command = json!({
            "practice_scope": scope,
            "assignment": {
                "kind": assignment.kind,
                "key": assignment.key,
                "prompt": assignment.prompt,
                "preloaded_skill_digests": verified_preloads
                    .iter()
                    .map(|s| (&s.name, &s.sha256))
                    .collect::<Vec<_>>(),
            },
        });
        match receipt::record(
            &tx,
            &scope.practice,
            PRACTICE_OWNER,
            &assignment.key,
            &command,
        )
        .await?
        {
            Receipt::New => {}
            Receipt::Same => {
                tx.commit().await?;
                return Ok(Accepted::Duplicate);
            }
            Receipt::Different => {
                return Err(RuntimeError::Database(error("idempotency collision")).into());
            }
        }
        if practice_job::Entity::find_by_id(job_keys(scope))
            .lock_exclusive()
            .one(&tx)
            .await?
            .is_some()
        {
            return Err(RuntimeError::Database(error("practice job id already exists")).into());
        }
        let observation = super::sources::observed(&tx, &scope.workspace).await?;
        let clock = now(&tx).await?;
        let binding =
            frozen::keep_owner_binding(&tx, &scope.practice, PRACTICE_OWNER, &built_recipe).await?;
        let skills_id =
            frozen::keep_owner_skills(&tx, &scope.practice, PRACTICE_OWNER, skills).await?;
        let mut runs = RunHistory::default();
        let failures = runs
            .start(Uuid::new_v4().to_string())
            .map_err(RuntimeError::from)?;
        let job = PracticeJob {
            scope: scope.clone(),
            kind: assignment.kind,
            status: WorkStatus::Pending,
            work: scope.job_id.clone(),
            revision: 1,
            assignment: assignment.clone(),
            preloaded_skills: verified_preloads,
            skills: skills_id,
            binding,
            runs,
            failures,
            allowance_after: 0,
            sources_version: observation.version,
            sources: observation.sources,
            result: None,
            dispatch_after: 0,
            attempts: Vec::new(),
            created_at: clock,
            updated_at: clock,
        };
        practice_job::Entity::insert(job_active_model(&job)?)
            .exec_without_returning(&tx)
            .await?;
        super::delivery::notify(&tx, &Delivery::practice_job(scope, assignment.key.clone()))
            .await?;
        tx.commit().await?;
        Ok(Accepted::New)
    }

    /// Reads one practice background job with its attempts and ordered events.
    pub async fn practice_job(&self, scope: &PracticeScope) -> Result<Option<PracticeJob>, DbErr> {
        scope.validate()?;
        let tx = self.snapshot().await?;
        let Some(row) = practice_job::Entity::find_by_id(job_keys(scope))
            .one(&tx)
            .await?
        else {
            tx.commit().await?;
            return Ok(None);
        };
        let mut attempts = load_attempts(&tx, scope).await?;
        load_attempt_events(&tx, scope, &mut attempts).await?;
        let job = decode_job(row, attempts)?;
        tx.commit().await?;
        Ok(Some(job))
    }

    /// Reads the frozen provider binding recorded with `job`.
    pub async fn practice_job_binding(
        &self,
        scope: &PracticeScope,
        reference: &BindingRef,
    ) -> Result<RequestBinding, DbErr> {
        scope.validate()?;
        frozen::owner_binding(&self.db, &scope.practice, PRACTICE_OWNER, reference).await
    }

    /// Lists pending or lease-expired practice jobs in `practice`, oldest update first.
    pub async fn practice_jobs_due(
        &self,
        practice: &str,
        maximum: usize,
    ) -> Result<Vec<PracticeScope>, DbErr> {
        if practice.trim().is_empty() || !(1..=100).contains(&maximum) {
            return Err(error("invalid practice job due query"));
        }
        let clock = now(&self.db).await?;
        let rows = practice_job::Entity::find()
            .filter(practice_job::Column::Practice.eq(practice))
            .filter(
                Condition::any()
                    .add(practice_job::Column::Status.eq(WorkStatus::Pending))
                    .add(practice_job::Column::Status.eq(WorkStatus::Active)),
            )
            .order_by_asc(practice_job::Column::UpdatedAt)
            .order_by_asc(practice_job::Column::Workspace)
            .order_by_asc(practice_job::Column::JobId)
            .limit(maximum as u64)
            .all(&self.db)
            .await?;
        let mut due = Vec::new();
        for row in rows {
            let scope = PracticeScope {
                practice: row.practice,
                workspace: row.workspace,
                job_id: row.job_id,
            };
            if row.status == WorkStatus::Pending {
                due.push(scope);
                continue;
            }
            let attempts = load_attempts(&self.db, &scope).await?;
            if attempts
                .last()
                .is_some_and(|attempt| attempt.expires <= clock)
            {
                due.push(scope);
            }
        }
        Ok(due)
    }

    /// Keyset-paginated recovery sweep of due practice-job wakeups across practices.
    pub async fn practice_jobs_dispatch_due(
        &self,
        maximum: usize,
        after: Option<&PracticeScope>,
    ) -> Result<(Vec<Delivery>, Option<PracticeScope>), DbErr> {
        if !(1..=100).contains(&maximum) {
            return Err(error("invalid dispatch bounds"));
        }
        let mut query = practice_job::Entity::find()
            .filter(
                Condition::any()
                    .add(practice_job::Column::Status.eq(WorkStatus::Pending))
                    .add(practice_job::Column::Status.eq(WorkStatus::Active)),
            )
            .order_by_asc(practice_job::Column::Practice)
            .order_by_asc(practice_job::Column::Workspace)
            .order_by_asc(practice_job::Column::JobId)
            .limit(maximum as u64);
        if let Some(cursor) = after {
            query = query.filter(
                Condition::any()
                    .add(practice_job::Column::Practice.gt(cursor.practice.as_str()))
                    .add(
                        Condition::all()
                            .add(practice_job::Column::Practice.eq(cursor.practice.as_str()))
                            .add(practice_job::Column::Workspace.gt(cursor.workspace.as_str())),
                    )
                    .add(
                        Condition::all()
                            .add(practice_job::Column::Practice.eq(cursor.practice.as_str()))
                            .add(practice_job::Column::Workspace.eq(cursor.workspace.as_str()))
                            .add(practice_job::Column::JobId.gt(cursor.job_id.as_str())),
                    ),
            );
        }
        let rows = query.all(&self.db).await?;
        let scopes: Vec<PracticeScope> = rows
            .into_iter()
            .map(|row| PracticeScope {
                practice: row.practice,
                workspace: row.workspace,
                job_id: row.job_id,
            })
            .collect();
        let next = (scopes.len() == maximum).then(|| scopes.last().unwrap().clone());
        let mut deliveries = Vec::new();
        for scope in scopes {
            let (tx, job, clock) = self.locked_practice_job(&scope).await?;
            if job.dispatch_after <= clock
                && (job.status == WorkStatus::Pending
                    || (job.status == WorkStatus::Active
                        && job.attempts.last().is_some_and(|a| a.expires <= clock)))
            {
                deliveries.push(Delivery::practice_job(&scope, job.assignment.key.clone()));
            }
            tx.commit().await?;
        }
        Ok((deliveries, next))
    }

    /// One keyset page of due practice jobs, shaped as [`super::delivery::DispatchPage`]
    /// for the worker pull loop.
    pub async fn practice_jobs_due_page(
        &self,
        maximum: usize,
        after: Option<&Scope>,
    ) -> Result<super::delivery::DispatchPage, DbErr> {
        let cursor = after.map(PracticeScope::from_delivery_scope).transpose()?;
        let (deliveries, next) = self
            .practice_jobs_dispatch_due(maximum, cursor.as_ref())
            .await?;
        Ok(super::delivery::DispatchPage {
            deliveries,
            next: next.map(|scope| scope.as_control_scope()),
        })
    }

    pub(super) async fn practice_job_delivery_state(
        &self,
        delivery: &Delivery,
    ) -> Result<DeliveryState, DbErr> {
        let scope = PracticeScope::from_delivery_scope(&delivery.scope)?;
        let (tx, job, clock) = self.locked_practice_job(&scope).await?;
        tx.commit().await?;
        if job.assignment.key != delivery.input_id {
            return Ok(DeliveryState::Settled);
        }
        Ok(match job.status {
            WorkStatus::Completed | WorkStatus::Held | WorkStatus::Cancelled => {
                DeliveryState::Settled
            }
            WorkStatus::Failed => DeliveryState::Failed,
            WorkStatus::Active
                if job.attempts.last().is_some_and(|attempt| {
                    attempt.status == AttemptStatus::Active && attempt.expires > clock
                }) =>
            {
                DeliveryState::Claimed
            }
            WorkStatus::Active | WorkStatus::Pending => DeliveryState::Due,
        })
    }

    /// Claims the practice-job attempt named by a [`DeliveryKind::PracticeJob`]
    /// broker wakeup after verifying practice patient membership.
    pub async fn claim_practice_job_delivery(
        &self,
        delivery: &Delivery,
        lease_seconds: i64,
        authority: &dyn PracticeAuthority,
    ) -> Result<Option<PracticeJobClaim>, PracticeJobError> {
        if delivery.kind != DeliveryKind::PracticeJob {
            return Err(RuntimeError::Database(error("not a practice job delivery")).into());
        }
        let scope = PracticeScope::from_delivery_scope(&delivery.scope)?;
        self.claim_practice_job_inner(&scope, lease_seconds, Some(&delivery.input_id), authority)
            .await
    }

    /// Claims an attempt lease on a pending (or lease-expired active) practice
    /// background job after verifying practice patient membership.
    pub async fn claim_practice_job(
        &self,
        scope: &PracticeScope,
        lease_seconds: i64,
        authority: &dyn PracticeAuthority,
    ) -> Result<Option<PracticeJobClaim>, PracticeJobError> {
        self.claim_practice_job_inner(scope, lease_seconds, None, authority)
            .await
    }

    async fn claim_practice_job_inner(
        &self,
        scope: &PracticeScope,
        lease_seconds: i64,
        expected_input_id: Option<&str>,
        authority: &dyn PracticeAuthority,
    ) -> Result<Option<PracticeJobClaim>, PracticeJobError> {
        authority.check_practice_patient(scope).await?;
        let lease = Lease::new(lease_seconds)?;
        let (tx, mut job, clock) = self.locked_practice_job(scope).await?;
        if expected_input_id.is_some_and(|input_id| job.assignment.key != input_id) {
            tx.commit().await?;
            return Ok(None);
        }
        if job.status == WorkStatus::Active
            && job
                .attempts
                .last()
                .is_some_and(|attempt| attempt.expires <= clock)
        {
            let attempt = job
                .attempts
                .last_mut()
                .expect("expired practice job attempt");
            let outcome = settle_attempt(
                AttemptFailure::Abandoned,
                &mut job.failures,
                &mut attempt.status,
                &mut job.runs,
            )?;
            job.status = outcome.map_or(WorkStatus::Pending, WorkStatus::from);
            if job.status == WorkStatus::Pending {
                job.dispatch_after = clock;
            }
            job.updated_at = clock;
            Self::sync_review_on_settlement_in_tx(
                &tx,
                &job,
                "abandoned attempt lease expired",
                clock,
            )
            .await?;
            if job.status == WorkStatus::Failed {
                Self::save_practice_job(tx, &job).await?;
                return Ok(None);
            }
        }
        if job.status != WorkStatus::Pending || job.dispatch_after > clock {
            tx.commit().await?;
            return Ok(None);
        }
        let observation = super::sources::observed(&tx, &scope.workspace).await?;
        job.sources_version = observation.version;
        job.sources = observation.sources;
        let predecessor = job.attempts.last().map(|attempt| attempt.id.clone());
        let id = Uuid::new_v4().to_string();
        job.attempts.push(PracticeJobAttempt {
            id: id.clone(),
            predecessor,
            work: job.work.clone(),
            revision: job.revision,
            status: AttemptStatus::Active,
            expires: lease.expires(clock),
            events: Vec::new(),
        });
        job.status = WorkStatus::Active;
        job.updated_at = clock;
        Self::sync_review_on_claim_in_tx(&tx, &job, &id, clock).await?;
        let claim = PracticeJobClaim {
            lease,
            scope: scope.clone(),
            kind: job.kind,
            attempt: id,
            work: job.work.clone(),
            revision: job.revision,
        };
        Self::save_practice_job(tx, &job).await?;
        Ok(Some(claim))
    }

    /// Checks without taking a row lock whether `claim` still owns the active attempt.
    pub async fn assert_practice_job_current(
        &self,
        claim: &PracticeJobClaim,
    ) -> Result<(), engine::ProductError> {
        claim.scope.validate()?;
        let row = practice_job::Entity::find_by_id(job_keys(&claim.scope))
            .select_only()
            .columns([practice_job::Column::Revision, practice_job::Column::Work])
            .into_tuple::<(i64, String)>()
            .one(&self.db)
            .await?
            .ok_or_else(|| error("practice job not found"))?;
        let revision =
            u64::try_from(row.0).map_err(conversion_error("practice_job.revision", "u64"))?;
        let last = practice_job_attempt::Entity::find()
            .select_only()
            .columns([
                practice_job_attempt::Column::Id,
                practice_job_attempt::Column::Status,
            ])
            .filter(attempt_scope_filter(&claim.scope))
            .order_by_desc(practice_job_attempt::Column::Position)
            .into_tuple::<(String, AttemptStatus)>()
            .one(&self.db)
            .await?;
        control::attempt_currency(
            last.as_ref().map(|(id, status)| (id.as_str(), *status)),
            &claim.attempt,
            revision == claim.revision && row.1 == claim.work,
        )?;
        Ok(())
    }

    /// Extends the lease of a still-current practice job attempt.
    pub async fn renew_practice_job_claim(&self, claim: &PracticeJobClaim) -> Result<(), DbErr> {
        let (tx, mut job, clock) = self.locked_practice_job(&claim.scope).await?;
        if job_current(&job, claim, clock).is_err() {
            return Err(error("practice job attempt ownership lost"));
        }
        if let Some(attempt) = job.attempts.last_mut() {
            attempt.expires = claim.lease.expires(clock);
        }
        job.updated_at = clock;
        Self::save_practice_job(tx, &job).await
    }

    /// Confirms the provider binding matches the frozen background-lane recipe
    /// saved when the practice job was accepted.
    pub async fn admit_practice_job_binding(
        &self,
        claim: &PracticeJobClaim,
        recipe: &RequestBinding,
    ) -> Result<(), engine::ProductError> {
        let (tx, job, clock) = self.locked_practice_job(&claim.scope).await?;
        job_current(&job, claim, clock)?;
        let saved =
            frozen::owner_binding(&tx, &claim.scope.practice, PRACTICE_OWNER, &job.binding).await?;
        if saved != *recipe {
            return Err(error("dispatch changed the accepted practice job recipe").into());
        }
        Ok(tx.commit().await?)
    }

    /// Appends one execution event under [`Owner::PracticeJob`] for `claim`.
    pub async fn persist_practice_job_event(
        &self,
        claim: &PracticeJobClaim,
        event: Event,
    ) -> Result<(), engine::ProductError> {
        if event.attempt_id != claim.attempt {
            return Err(error("practice job event belongs to another attempt").into());
        }
        let (tx, mut job, clock) = self.locked_practice_job(&claim.scope).await?;
        let current = job_current(&job, claim, clock);
        if !event.kind.survives_loss_of_currency() {
            current?;
        }
        let is_current = current.is_ok();
        if !job
            .attempts
            .iter()
            .any(|attempt| attempt.id == claim.attempt)
        {
            return Err(error("unknown practice job attempt").into());
        }
        records::append_for(&tx, &claim.scope.owner_key(), &event, is_current).await?;
        if is_current && matches!(event.kind, EventKind::ProviderBlocked { .. }) {
            let attempt = job
                .attempts
                .last_mut()
                .ok_or_else(|| error("unknown practice job attempt"))?;
            let outcome = settle_attempt(
                AttemptFailure::ProviderBlocked,
                &mut job.failures,
                &mut attempt.status,
                &mut job.runs,
            )?;
            job.status = outcome.map_or(WorkStatus::Pending, WorkStatus::from);
            if job.status == WorkStatus::Pending {
                job.dispatch_after = clock;
                super::delivery::notify(
                    &tx,
                    &Delivery::practice_job(&claim.scope, job.assignment.key.clone()),
                )
                .await?;
            }
            job.updated_at = clock;
            Self::sync_review_on_settlement_in_tx(&tx, &job, "provider blocked", clock).await?;
            Self::save_practice_job(tx, &job).await?;
            return Ok(());
        }
        tx.commit().await?;
        Ok(())
    }

    /// Holds a practice job when a run hits a turn/budget limit, retaining any
    /// partial output attributed to the practice and job.
    pub async fn retain_practice_job_partial(
        &self,
        claim: &PracticeJobClaim,
        turn: &Turn,
        authority: &dyn PracticeAuthority,
    ) -> Result<(), engine::ProductError> {
        authority.check_practice_patient(&claim.scope).await?;
        let (tx, mut job, clock) = self.locked_practice_job(&claim.scope).await?;
        load_attempt_events(&tx, &claim.scope, &mut job.attempts).await?;
        job_current(&job, claim, clock)?;
        let attempt = job
            .attempts
            .last()
            .ok_or_else(|| error("missing attempt"))?;
        if !matches!(
            attempt.events.last().map(|event| &event.kind),
            Some(EventKind::Candidate { text }) if text == &turn.text()
        ) {
            return Err(error("partial result lacks durable evidence").into());
        }
        let ordered =
            records::ordered_for(&tx, &claim.scope.owner_key(), &job.attempt_ids()).await?;
        let recipe =
            frozen::owner_binding(&tx, &claim.scope.practice, PRACTICE_OWNER, &job.binding).await?;
        let messages = Self::review_message_sources_in_tx(&tx, &job).await?;
        if !engine::pending_final(&recipe, ordered.iter(), job.allowance_after, &messages)
            .map_err(RuntimeError::PendingFinal)?
            .is_some_and(|saved| saved.turn == *turn)
        {
            return Err(error("partial result differs from recorded terminal turn").into());
        }
        let observation = super::sources::observed(&tx, &claim.scope.workspace).await?;
        job.sources_version = observation.version;
        job.sources = observation.sources;
        let text = turn.text();
        let partial_copy = text.clone();
        if let Some(attempt) = job.attempts.last_mut() {
            attempt.status = AttemptStatus::Held;
        }
        job.runs
            .end(RunOutcome::Held {
                reason: HoldReason::Limit,
            })
            .map_err(RuntimeError::from)?;
        job.status = WorkStatus::Held;
        if !text.trim().is_empty() {
            job.result = Some(PracticeJobResult {
                attribution: PracticeEffectAttribution {
                    practice: claim.scope.practice.clone(),
                    workspace: claim.scope.workspace.clone(),
                    job_id: claim.scope.job_id.clone(),
                    kind: job.kind,
                },
                attempt_id: claim.attempt.clone(),
                output_text: text,
                completion: PracticeJobCompletion::Partial,
                sources_version: job.sources_version,
                recorded_at: clock,
            });
        }
        job.updated_at = clock;
        Self::sync_review_on_partial_hold_in_tx(&tx, &job, &partial_copy, clock).await?;
        Self::save_practice_job(tx, &job).await?;
        Ok(())
    }

    /// Commits the completed practice job output under the practice and job,
    /// verifying practice patient authority and retained terminal provider evidence.
    pub async fn commit_practice_job(
        &self,
        claim: &PracticeJobClaim,
        turn: &Turn,
        authority: &dyn PracticeAuthority,
    ) -> Result<PracticeJobResult, engine::ProductError> {
        authority.check_practice_patient(&claim.scope).await?;
        let (tx, mut job, clock) = self.locked_practice_job(&claim.scope).await?;
        load_attempt_events(&tx, &claim.scope, &mut job.attempts).await?;
        job_current(&job, claim, clock)?;
        let attempt = job
            .attempts
            .last()
            .ok_or_else(|| error("missing attempt"))?;
        if !matches!(
            attempt.events.last().map(|event| &event.kind),
            Some(EventKind::Candidate { text }) if text == &turn.text()
        ) {
            return Err(error("candidate lacks durable evidence").into());
        }
        let terminal = job
            .attempts
            .iter()
            .filter(|saved| saved.revision == claim.revision)
            .flat_map(|saved| &saved.events)
            .rev()
            .find_map(|event| match &event.kind {
                EventKind::ProviderStarted { .. } => Some(None),
                EventKind::ProviderReturned(retained) => Some(Some(retained)),
                EventKind::ProviderObservation { .. }
                | EventKind::ProviderFailed { .. }
                | EventKind::TransportRetry { .. }
                | EventKind::ToolStarted(_)
                | EventKind::ToolReturned { .. }
                | EventKind::ProviderBlocked { .. }
                | EventKind::ContextRejected
                | EventKind::Candidate { .. }
                | EventKind::CallBegan { .. }
                | EventKind::WindowOpened { .. } => None,
            })
            .flatten()
            .ok_or_else(|| error("candidate lacks a retained terminal turn"))?;
        if terminal != turn {
            return Err(error("completion differs from recorded terminal turn").into());
        }
        let output_text = terminal_answer(terminal)?;
        let ordered =
            records::ordered_for(&tx, &claim.scope.owner_key(), &job.attempt_ids()).await?;
        let messages = Self::review_message_sources_in_tx(&tx, &job).await?;
        let observation = super::sources::observed(&tx, &claim.scope.workspace).await?;
        Self::finalize_patient_review_in_tx(&tx, &job, &ordered, &messages, &observation, clock)
            .await?;
        job.sources_version = observation.version;
        job.sources = observation.sources;
        if let Some(attempt) = job.attempts.last_mut() {
            attempt.status = AttemptStatus::Completed;
        }
        job.runs
            .end(RunOutcome::Completed)
            .map_err(RuntimeError::from)?;
        job.status = WorkStatus::Completed;
        let saved_result = PracticeJobResult {
            attribution: PracticeEffectAttribution {
                practice: claim.scope.practice.clone(),
                workspace: claim.scope.workspace.clone(),
                job_id: claim.scope.job_id.clone(),
                kind: job.kind,
            },
            attempt_id: claim.attempt.clone(),
            output_text,
            completion: PracticeJobCompletion::Complete,
            sources_version: job.sources_version,
            recorded_at: clock,
        };
        job.result = Some(saved_result.clone());
        job.updated_at = clock;
        Self::save_practice_job(tx, &job).await?;
        Ok(saved_result)
    }

    /// Settles an unsuccessful practice job attempt under the shared failure rule.
    pub async fn settle_practice_job_attempt(
        &self,
        claim: &PracticeJobClaim,
        reason: AttemptFailure,
        backoff: super::delivery::Backoff,
    ) -> Result<AttemptSettlement, RuntimeError> {
        let (tx, mut job, clock) = self.locked_practice_job(&claim.scope).await?;
        let Some(attempt) = job.attempts.last_mut().filter(|a| a.id == claim.attempt) else {
            tx.rollback().await?;
            return Ok(AttemptSettlement::Superseded);
        };
        attempt.expires = clock;
        if job.revision == claim.revision
            && job.work == claim.work
            && job.status == WorkStatus::Active
            && attempt.status == AttemptStatus::Active
        {
            let before = job.failures;
            let outcome = settle_attempt(
                reason,
                &mut job.failures,
                &mut attempt.status,
                &mut job.runs,
            )?;
            let settlement = if job.failures > before {
                let retry_clock = super::delivery::RetryClock {
                    failures: job.failures,
                    dispatch_after: &mut job.dispatch_after,
                };
                let after = super::delivery::failed_once(retry_clock, clock, backoff);
                job.status = after.work_status();
                AttemptSettlement::Failed(after)
            } else {
                job.status = outcome.map_or(WorkStatus::Pending, WorkStatus::from);
                if job.status == WorkStatus::Pending {
                    job.dispatch_after = clock;
                }
                AttemptSettlement::Applied
            };
            if job.status == WorkStatus::Pending && job.dispatch_after <= clock {
                super::delivery::notify(
                    &tx,
                    &Delivery::practice_job(&claim.scope, job.assignment.key.clone()),
                )
                .await?;
            }
            job.updated_at = clock;
            Self::sync_review_on_settlement_in_tx(&tx, &job, &format!("{reason:?}"), clock).await?;
            Self::save_practice_job(tx, &job).await?;
            Ok(settlement)
        } else {
            Self::save_practice_job(tx, &job).await?;
            Ok(AttemptSettlement::Superseded)
        }
    }

    /// Stops a pending or active practice job with no physician session,
    /// delegating to the same run-store hold transition used by [`Store::stop_authorized`].
    pub async fn stop_practice_job(
        &self,
        scope: &PracticeScope,
        key: &str,
        work_item_id: &str,
    ) -> Result<Stopped, RuntimeError> {
        if [
            key,
            work_item_id,
            &scope.practice,
            &scope.workspace,
            &scope.job_id,
        ]
        .iter()
        .any(|part| part.trim().is_empty())
        {
            return Ok(Stopped::Refused);
        }
        let tx = self.db.begin().await?;
        let control_scope = scope.as_control_scope();
        let command = json!({
            "scope": control_scope,
            "stop": { "work_item_id": work_item_id },
        });
        match receipt::record(&tx, &scope.practice, PRACTICE_OWNER, key, &command).await? {
            Receipt::New => {}
            Receipt::Same => {
                tx.commit().await?;
                return Ok(Stopped::Duplicate);
            }
            Receipt::Different => {
                tx.commit().await?;
                return Ok(Stopped::Refused);
            }
        }
        Self::stop_practice_job_in_tx(tx, scope, work_item_id).await
    }

    /// Resumes a held or failed practice job with no physician session,
    /// delegating to the same run-store resume transition used by [`Store::resume_authorized`].
    pub async fn resume_practice_job(
        &self,
        scope: &PracticeScope,
        key: &str,
        work_item_id: &str,
        expected_run_id: &str,
    ) -> Result<Resumed, RuntimeError> {
        if [
            key,
            work_item_id,
            expected_run_id,
            &scope.practice,
            &scope.workspace,
            &scope.job_id,
        ]
        .iter()
        .any(|part| part.trim().is_empty())
        {
            return Ok(Resumed::Refused);
        }
        let tx = self.db.begin().await?;
        let control_scope = scope.as_control_scope();
        let command = json!({
            "scope": control_scope,
            "resume": { "work_item_id": work_item_id, "expected_run_id": expected_run_id },
        });
        match receipt::record(&tx, &scope.practice, PRACTICE_OWNER, key, &command).await? {
            Receipt::New => {}
            Receipt::Same => {
                tx.commit().await?;
                return Ok(Resumed::Duplicate);
            }
            Receipt::Different => {
                tx.commit().await?;
                return Ok(Resumed::Refused);
            }
        }
        Self::resume_practice_job_in_tx(tx, &control_scope, work_item_id, expected_run_id).await
    }

    /// Shared run-store Stop transition for practice jobs, invoked both by
    /// [`Store::stop_authorized`] and [`Store::stop_practice_job`].
    pub(super) async fn stop_practice_job_in_tx(
        tx: sea_orm::DatabaseTransaction,
        scope: &PracticeScope,
        work_item_id: &str,
    ) -> Result<Stopped, RuntimeError> {
        let Some(row) = practice_job::Entity::find_by_id(job_keys(scope))
            .lock_exclusive()
            .one(&tx)
            .await?
        else {
            return Ok(Stopped::Refused);
        };
        let attempts = load_attempts(&tx, scope).await?;
        let mut job = decode_job(row, attempts)?;
        if job.status == WorkStatus::Held && job.work == work_item_id {
            return Ok(Stopped::Duplicate);
        }
        if !job.status.running() || job.work != work_item_id {
            return Ok(Stopped::Refused);
        }
        if let Some(attempt) = job
            .attempts
            .last_mut()
            .filter(|attempt| attempt.status == AttemptStatus::Active)
        {
            attempt.status = AttemptStatus::Cancelled;
        }
        job.runs.end(RunOutcome::Held {
            reason: HoldReason::Stopped,
        })?;
        job.status = WorkStatus::Held;
        job.updated_at = now(&tx).await?;
        Self::sync_review_on_stop_in_tx(&tx, &job, job.updated_at).await?;
        Self::save_practice_job(tx, &job).await?;
        Ok(Stopped::Held)
    }

    /// Shared run-store Resume transition for practice jobs, invoked both by
    /// [`Store::resume_authorized`] and [`Store::resume_practice_job`].
    pub(super) async fn resume_practice_job_in_tx(
        tx: sea_orm::DatabaseTransaction,
        control_scope: &Scope,
        work_item_id: &str,
        expected_run_id: &str,
    ) -> Result<Resumed, RuntimeError> {
        let practice_scope = PracticeScope {
            practice: control_scope.practice.clone(),
            workspace: control_scope.workspace.clone(),
            job_id: control_scope.chat.clone(),
        };
        let Some(row) = practice_job::Entity::find_by_id(job_keys(&practice_scope))
            .lock_exclusive()
            .one(&tx)
            .await?
        else {
            return Ok(Resumed::Refused);
        };
        let mut attempts = load_attempts(&tx, &practice_scope).await?;
        load_attempt_events(&tx, &practice_scope, &mut attempts).await?;
        let mut job = decode_job(row, attempts)?;
        let resumable = job.status.resumable()
            && job.work == work_item_id
            && job
                .runs
                .current()
                .is_some_and(|run| run.id == expected_run_id);
        if !resumable {
            return Ok(Resumed::Refused);
        }
        job.allowance_after = engine::recovery::resume_boundary(
            job.attempts.iter().flat_map(|attempt| &attempt.events),
        );
        job.failures = job.runs.start(Uuid::new_v4().to_string())?;
        job.status = WorkStatus::Pending;
        job.dispatch_after = 0;
        job.updated_at = now(&tx).await?;
        let input_id = job.assignment.key.clone();
        Self::sync_review_on_resume_in_tx(&tx, &job, job.updated_at).await?;
        let delivery = Delivery::practice_job(&practice_scope, input_id);
        super::delivery::notify(&tx, &delivery).await?;
        Self::save_practice_job(tx, &job).await?;
        Ok(Resumed::Pending { delivery })
    }

    /// Cancels a pending, active, or held practice job inside caller-supplied
    /// transaction `tx` without committing `tx`. Returns `true` when the job
    /// transitioned to [`WorkStatus::Cancelled`].
    pub(super) async fn cancel_practice_job_in_tx(
        tx: &sea_orm::DatabaseTransaction,
        scope: &PracticeScope,
        now_unix: i64,
    ) -> Result<bool, RuntimeError> {
        let Some(row) = practice_job::Entity::find_by_id(job_keys(scope))
            .lock_exclusive()
            .one(tx)
            .await?
        else {
            return Ok(false);
        };
        let attempts = load_attempts(tx, scope).await?;
        let mut job = decode_job(row, attempts)?;
        if matches!(
            job.status,
            WorkStatus::Completed | WorkStatus::Failed | WorkStatus::Cancelled
        ) {
            return Ok(false);
        }
        if let Some(attempt) = job
            .attempts
            .last_mut()
            .filter(|attempt| attempt.status == AttemptStatus::Active)
        {
            attempt.status = AttemptStatus::Cancelled;
        }
        if let Some(current) = job.runs.current() {
            match current.outcome {
                None => {
                    job.runs.end(RunOutcome::Cancelled)?;
                }
                Some(RunOutcome::Held { .. }) => {
                    job.runs.start(Uuid::new_v4().to_string())?;
                    job.runs.end(RunOutcome::Cancelled)?;
                }
                Some(RunOutcome::Completed | RunOutcome::Failed | RunOutcome::Cancelled) => {}
            }
        }
        job.status = WorkStatus::Cancelled;
        job.updated_at = now_unix;
        save_attempts(tx, &job.scope, &job.attempts).await?;
        job_active_model(&job)?.update(tx).await?;
        super::briefings::sync_visit_preparation_job_in_tx(tx, &job).await?;
        Ok(true)
    }

    pub(super) async fn practice_job_snapshot(
        &self,
        scope: &PracticeScope,
    ) -> Result<(PracticeJob, Vec<(Event, bool)>, PhysicianSkills), DbErr> {
        let tx = self.snapshot().await?;
        let row = practice_job::Entity::find_by_id(job_keys(scope))
            .one(&tx)
            .await?
            .ok_or_else(|| error("practice job not found"))?;
        let mut attempts = load_attempts(&tx, scope).await?;
        load_attempt_events(&tx, scope, &mut attempts).await?;
        let job = decode_job(row, attempts)?;
        let ordered = records::ordered_for(&tx, &scope.owner_key(), &job.attempt_ids()).await?;
        let skills =
            frozen::owner_skills(&tx, &scope.practice, PRACTICE_OWNER, &job.skills).await?;
        tx.commit().await?;
        Ok((job, ordered, skills))
    }
}

pub(crate) fn practice_job_tool_is_read_only(_kind: PracticeJobKind, name: &str) -> bool {
    crate::physician_chat::read_only(name)
        || crate::initial_understanding::read_only_understanding_tool(name)
        || crate::patient_reviews::read_only_review_tool(name)
}

/// Shared-harness host for one claimed practice background job. Enforces
/// practice-only chart access, refuses physician-only actions, and records all
/// effects under the practice and job.
pub struct PracticeJobHost<'a> {
    /// Durable run store.
    pub store: Store,
    /// Claimed practice job attempt.
    pub claim: PracticeJobClaim,
    /// Practice patient membership authority (no physician session).
    pub authority: &'a dyn PracticeAuthority,
}

#[async_trait]
impl Product for PracticeJobHost<'_> {
    async fn message_sources(
        &mut self,
    ) -> Result<Option<Vec<engine::ProductMessage>>, engine::ProductError> {
        let (job, _, _) = self.store.practice_job_snapshot(&self.claim.scope).await?;
        let messages = self.store.review_message_sources_for_job(&job).await?;
        Ok(Some(messages))
    }

    async fn admit_binding(&mut self, recipe: &RequestBinding) -> Result<(), engine::ProductError> {
        self.authority
            .check_practice_patient(&self.claim.scope)
            .await?;
        self.store
            .admit_practice_job_binding(&self.claim, recipe)
            .await?;
        Ok(())
    }

    fn read_only(&self, name: &str) -> bool {
        practice_job_tool_is_read_only(self.claim.kind, name)
    }

    fn read_result(&mut self, id: &str) -> engine::recipe::ToolFuture<Option<String>> {
        let store = self.store.clone();
        let claim = self.claim.clone();
        let id = id.to_owned();
        Box::pin(async move {
            store.assert_practice_job_current(&claim).await?;
            let (job, ordered, _) = store.practice_job_snapshot(&claim.scope).await?;
            let messages = store.review_message_sources_for_job(&job).await?;
            engine::results::resolve(ordered.iter(), &id, &messages).map_err(Into::into)
        })
    }

    async fn assert_current(&mut self) -> Result<(), engine::ProductError> {
        self.authority
            .check_practice_patient(&self.claim.scope)
            .await?;
        self.store.assert_practice_job_current(&self.claim).await
    }

    async fn persist(&mut self, event: Event) -> Result<(), engine::ProductError> {
        self.store
            .persist_practice_job_event(&self.claim, event)
            .await
            .map_err(persistence_failure)
    }

    fn saved_outcome(
        &mut self,
        call: &Invocation,
    ) -> engine::recipe::ToolFuture<Option<ToolOutcome>> {
        let is_understanding = call.name == crate::initial_understanding::SAVE_UNDERSTANDING_TOOL;
        let is_refresh = super::refresh::is_refresh_tool(&call.name);
        let is_concern = super::concerns::is_concern_tool(&call.name);
        let is_memory = super::memory::tools::MemoryTool::named(&call.name).is_some();
        let is_complete_review = call.name == crate::patient_reviews::COMPLETE_PATIENT_REVIEW_TOOL;
        if !is_understanding && !is_refresh && !is_concern && !is_memory && !is_complete_review {
            return Box::pin(async { Ok(None) });
        }
        if (is_concern || is_memory || is_complete_review)
            && self.claim.kind != PracticeJobKind::PatientReview
        {
            return Box::pin(async { Ok(None) });
        }
        let store = self.store.clone();
        let claim = self.claim.clone();
        let call = call.clone();
        Box::pin(async move {
            if is_understanding {
                return store
                    .saved_practice_understanding_outcome(&claim, &call)
                    .await;
            }
            store.assert_practice_job_current(&claim).await?;
            let (job, _, _) = store.practice_job_snapshot(&claim.scope).await?;
            if is_refresh {
                return store
                    .saved_practice_refresh_outcome(&claim.scope, &job.work, &call)
                    .await;
            }
            if is_concern {
                return store
                    .saved_concern_tool_outcome(
                        &claim.scope.practice,
                        &claim.scope.workspace,
                        &job.work,
                        &call,
                    )
                    .await;
            }
            if is_memory {
                let key = super::memory::PatientKey::for_practice_job(&claim.scope)?;
                return super::memory::tools::saved_outcome_for_key(&store, &key, &job.work, &call)
                    .await
                    .map_err(Into::into);
            }
            store
                .saved_complete_review_outcome(&claim.scope, &job.work, &call)
                .await
        })
    }

    fn execute(&mut self, call: &Invocation, arguments: Value) -> engine::recipe::ToolFuture {
        let store = self.store.clone();
        let claim = self.claim.clone();
        let call = call.clone();
        Box::pin(async move {
            store.assert_practice_job_current(&claim).await?;
            if is_physician_only_tool(&call.name) {
                return Ok(ToolOutcome::refused(format!(
                    "practice background job cannot act as a physician: {} is refused",
                    call.name
                )));
            }
            let (job, ordered, skills) = store.practice_job_snapshot(&claim.scope).await?;
            if working_notes::is_tool(&call.name) {
                let note_call = Invocation {
                    id: call.id.clone(),
                    name: call.name.clone(),
                    arguments,
                };
                let attempt = job
                    .attempts
                    .iter()
                    .find(|attempt| attempt.id == claim.attempt)
                    .ok_or("missing claimed practice job attempt")?;
                return working_notes::execute(
                    &attempt.work,
                    &note_call,
                    job.attempts.iter().flat_map(|attempt| {
                        attempt
                            .events
                            .iter()
                            .map(|event| (attempt.work.as_str(), event))
                    }),
                )?
                .ok_or_else(|| "working note tool was not handled".into());
            }
            if engine::history::is_tool(&call.name) {
                let raw_messages = store.review_message_sources_for_job(&job).await?;
                let messages: Vec<engine::history::Message<Value>> = raw_messages
                    .into_iter()
                    .map(|msg| engine::history::Message {
                        id: msg.id,
                        role: msg.role,
                        text: msg.text,
                        metadata: json!({
                            "practice": claim.scope.practice,
                            "job_id": claim.scope.job_id,
                            "kind": claim.kind,
                        }),
                    })
                    .collect();
                let page = match engine::history::execute(
                    &call.name,
                    &arguments,
                    &messages,
                    ordered.iter(),
                ) {
                    Ok(page) => page,
                    Err(reason) => return Ok(ToolOutcome::refused(reason)),
                };
                return Ok(ToolOutcome::History(page));
            }
            if call.name == SKILL_VIEW {
                return Ok(skills.execute(&call.name, &arguments));
            }
            if matches!(call.name.as_str(), SEARCH_TOOL | READ_TOOL) {
                return store
                    .execute_practice_chart_tool(&claim.scope, &call.name, &arguments)
                    .await;
            }
            if super::refresh::is_refresh_tool(&call.name) {
                let refresh_call = Invocation {
                    id: call.id.clone(),
                    name: call.name.clone(),
                    arguments,
                };
                return store
                    .execute_claimed_practice_refresh_tool(&claim, &job.work, &refresh_call)
                    .await;
            }
            if crate::initial_understanding::is_understanding_tool(&call.name) {
                return store
                    .execute_practice_understanding_tool_fenced(&claim, &call, &arguments)
                    .await;
            }
            if crate::patient_reviews::is_patient_review_tool(&call.name) {
                if claim.kind != PracticeJobKind::PatientReview {
                    return Ok(ToolOutcome::refused(format!(
                        "tool {} is only available to patient review jobs",
                        call.name
                    )));
                }
                return store
                    .execute_patient_review_tool(&claim, &job, &ordered, &call, &arguments)
                    .await;
            }
            if super::concerns::is_concern_tool(&call.name) {
                if claim.kind == PracticeJobKind::InitialUnderstanding {
                    return Ok(ToolOutcome::refused(format!(
                        "initial understanding job cannot modify concerns: {} is refused",
                        call.name
                    )));
                }
                let source_work = super::concerns::SourceWork::new(
                    claim.kind.as_str(),
                    &job.work,
                    &claim.attempt,
                )
                .map_err(|cause| -> engine::Cause { Box::new(cause) })?;
                let invocation = Invocation {
                    id: call.id.clone(),
                    name: call.name.clone(),
                    arguments,
                };
                return store
                    .execute_concern_tool(
                        &claim.scope.practice,
                        &claim.scope.workspace,
                        &source_work,
                        &invocation,
                    )
                    .await;
            }
            if let Some(mem_tool) = super::memory::tools::MemoryTool::named(&call.name) {
                if !mem_tool.reads() && claim.kind != PracticeJobKind::PatientReview {
                    return Ok(ToolOutcome::refused(format!(
                        "{} job cannot modify patient memory: {} is refused",
                        claim.kind.as_str(),
                        call.name
                    )));
                }
                return mem_tool
                    .execute_for_practice(&store, &claim.scope, &arguments, || {
                        Ok(super::memory::NoteOrigin {
                            work: job.work.clone(),
                            call: call.id.clone(),
                            message: job.work.clone(),
                        })
                    })
                    .await
                    .map_err(Into::into);
            }
            if super::chat_operations::is_retrieve_work_tool(&call.name) {
                if claim.kind == PracticeJobKind::InitialUnderstanding {
                    return Ok(ToolOutcome::refused(
                        "retrieve_patient_work is not available to initial understanding jobs",
                    ));
                }
                return store
                    .execute_practice_retrieve_work_tool(&claim.scope, &arguments)
                    .await;
            }
            if crate::clinical_library::is_library_tool(&call.name) {
                if claim.kind == PracticeJobKind::InitialUnderstanding {
                    return Ok(ToolOutcome::refused(format!(
                        "tool {} is not available to initial understanding jobs",
                        call.name
                    )));
                }
                return store
                    .execute_practice_clinical_library_tool(&call.name, &arguments)
                    .await;
            }
            Ok(ToolOutcome::refused(format!(
                "tool {} is not available to a practice background job",
                call.name
            )))
        })
    }

    async fn finish(&mut self, turn: &Turn) -> Result<Finish, engine::ProductError> {
        self.assert_current().await?;
        let (job, ordered, _) = self.store.practice_job_snapshot(&self.claim.scope).await?;
        if job.assignment.understanding.is_some()
            && !self
                .store
                .has_saved_understanding_for_work(
                    &self.claim.scope.practice,
                    &self.claim.scope.workspace,
                    &job.work,
                )
                .await?
        {
            return Ok(Finish::Continue(json!({
                "outcome": "understanding_not_saved",
                "message": "Call save_patient_understanding to record the operating index and physician-facing summary before finishing.",
            })));
        }
        match self
            .store
            .check_patient_review_ready_to_commit(&self.claim, &job, &ordered)
            .await?
        {
            super::reviews::ReviewCommitCheck::NotApplicable
            | super::reviews::ReviewCommitCheck::Ready => {}
            super::reviews::ReviewCommitCheck::Continue(payload) => {
                return Ok(Finish::Continue(payload));
            }
        }
        self.store
            .commit_practice_job(&self.claim, turn, self.authority)
            .await?;
        Ok(Finish::Committed)
    }

    async fn finish_partial(
        &mut self,
        turn: &Turn,
        _backstops: &[engine::LimitBackstop],
    ) -> Result<(), engine::ProductError> {
        self.assert_current().await?;
        self.store
            .retain_practice_job_partial(&self.claim, turn, self.authority)
            .await
    }

    async fn window_seed(&mut self) -> Result<engine::WindowSeed, engine::ProductError> {
        self.assert_current().await?;
        let (job, _, _) = self.store.practice_job_snapshot(&self.claim.scope).await?;
        let messages = self.store.review_message_sources_for_job(&job).await?;
        let partial_id = job
            .result
            .as_ref()
            .filter(|saved| saved.completion == PracticeJobCompletion::Partial)
            .map(|saved| format!("{}:result", saved.attempt_id));
        let mut items = Vec::with_capacity(messages.len() + 1);
        for message in messages {
            let is_partial = partial_id.as_deref() == Some(message.id.as_str());
            items.push(Item::ProductMessage(message));
            if is_partial {
                items.push(engine::history::partial_answer());
            }
        }
        let note_references = working_notes::references(job.attempts.iter().flat_map(|attempt| {
            attempt
                .events
                .iter()
                .map(|event| (attempt.work.as_str(), event))
        }))?;
        Ok(engine::WindowSeed {
            items,
            note_references,
        })
    }
}

impl PracticeJobHost<'_> {
    /// Hydrates the accepted background-lane recipe and runs or recovers the
    /// claimed practice job attempt through the shared harness engine.
    pub async fn run<P: Provider>(
        &mut self,
        provider: &mut P,
        limits: engine::RunLimits,
        cancel: &engine::Cancellation,
    ) -> Result<(), engine::Failure> {
        self.assert_current().await.map_err(engine::Failure::from)?;
        let (job, ordered, _) = self
            .store
            .practice_job_snapshot(&self.claim.scope)
            .await
            .map_err(|err| engine::Failure::Host(err.into()))?;
        let mut request = self
            .store
            .practice_job_binding(&self.claim.scope, &job.binding)
            .await
            .map_err(|err| engine::Failure::Host(err.into()))?
            .request(Vec::new());
        request.items.clear();
        let partial_id = job
            .result
            .as_ref()
            .filter(|saved| saved.completion == PracticeJobCompletion::Partial)
            .map(|saved| format!("{}:result", saved.attempt_id));
        let messages = self
            .store
            .review_message_sources_for_job(&job)
            .await
            .map_err(|err| engine::Failure::Host(err.into()))?;
        let (accepted, source_feedback) = self
            .store
            .practice_job_recovery_inputs(&job, &ordered)
            .await
            .map_err(|err| engine::Failure::Host(err.into()))?;
        let work = request.freeze()?.recover(
            &self.claim.attempt,
            engine::recovery::Records {
                events: ordered.iter().map(Into::into).collect(),
                allowance_after: job.allowance_after,
            },
            engine::recovery::Messages {
                sources: &messages,
                partial: partial_id,
            },
            engine::recovery::Inputs {
                accepted,
                source_feedback,
            },
        )?;
        work.run(provider, self, limits, cancel).await.map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::{ModelBinding, ModelRoles};

    fn load_skills() -> PhysicianSkills {
        PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
            .expect("physician skills catalog loads")
    }

    fn distinct_roles() -> ModelRoles {
        ModelRoles {
            chat: ModelBinding {
                model: "chat-lane-fixture".into(),
                effort: "low".into(),
                context_window_tokens: 128_000,
                max_input_tokens: 100_000,
                adapter: "fixture".into(),
            },
            background: ModelBinding {
                model: "background-lane-fixture".into(),
                effort: "high".into(),
                context_window_tokens: 272_000,
                max_input_tokens: 250_000,
                adapter: "fixture".into(),
            },
        }
    }

    #[test]
    fn background_recipe_uses_background_lane_preloads_skills_and_keeps_clean_model_text() {
        let skills = load_skills();
        let roles = distinct_roles();
        let assignment = Assignment::synthetic_patient_stub("pw-synthetic-1", &skills)
            .expect("synthetic patient assignment preloads skill");

        assert_eq!(assignment.kind, PracticeJobKind::InitialUnderstanding);
        assert_eq!(assignment.preloaded_skills.len(), 1);
        let preloaded = &assignment.preloaded_skills[0];
        assert_eq!(preloaded.name, "patient-record-retrieval");
        assert!(!preloaded.content.trim().is_empty());
        assert_eq!(
            preloaded.sha256,
            crate::sha256_hex(preloaded.content.as_bytes())
        );

        let built = recipe(&roles, &assignment, &skills);
        assert_eq!(built.model, "background-lane-fixture");
        assert_eq!(built.context_window_tokens, 272_000);
        assert_eq!(built.max_input_tokens, 250_000);
        assert_eq!(built.options, json!({ "effort": "high" }));
        assert!(built.instructions.contains("preloaded_assignment_skills"));
        assert!(built.instructions.contains("patient-record-retrieval"));
        assert!(built.instructions.contains(&preloaded.sha256));
        assert!(built.tools.iter().any(|tool| tool["name"] == SEARCH_TOOL));
        assert!(built.tools.iter().any(|tool| tool["name"] == READ_TOOL));
        assert!(
            built
                .tools
                .iter()
                .any(|tool| tool["name"] == super::super::refresh::REFRESH_TOOL)
        );
        assert!(
            !built
                .tools
                .iter()
                .any(|tool| tool["name"] == super::super::concerns::CONCERN_TOOL),
            "initial patient understanding recipe must not offer patient_concern"
        );
        assert!(
            !built
                .tools
                .iter()
                .any(|tool| { tool["name"].as_str().is_some_and(is_physician_only_tool) }),
            "practice background recipe must never offer physician-only tools"
        );

        let review_assignment = Assignment::new(
            PracticeJobKind::PatientReview,
            "review-1",
            "Review updated labs.",
            &["patient-record-retrieval"],
            &skills,
        )
        .expect("patient review assignment");
        let review_recipe = recipe(&roles, &review_assignment, &skills);
        assert!(
            review_recipe
                .tools
                .iter()
                .any(|tool| tool["name"] == super::super::concerns::CONCERN_TOOL),
            "patient review recipe must offer patient_concern"
        );
        assert!(
            !review_recipe
                .tools
                .iter()
                .any(|tool| { tool["name"].as_str().is_some_and(is_physician_only_tool) })
        );
    }

    #[test]
    fn assignment_refuses_unknown_skills_and_blank_inputs() {
        let skills = load_skills();
        assert_eq!(
            Assignment::new(PracticeJobKind::PatientReview, " ", "prompt", &[], &skills),
            Err(AssignmentError::EmptyAssignment)
        );
        assert_eq!(
            Assignment::new(PracticeJobKind::PatientReview, "key-1", " ", &[], &skills),
            Err(AssignmentError::EmptyAssignment)
        );
        assert!(matches!(
            Assignment::new(
                PracticeJobKind::PatientReview,
                "key-1",
                "Review chart",
                &["unknown-skill-name"],
                &skills,
            ),
            Err(AssignmentError::UnavailableSkill { name, .. }) if name == "unknown-skill-name"
        ));
    }

    #[test]
    fn practice_control_scope_is_rejected_by_physician_scope_validation() {
        let scope = PracticeScope {
            practice: "practice-a".into(),
            workspace: "pw-a".into(),
            job_id: "job-1".into(),
        };
        let err = scope
            .as_control_scope()
            .validate()
            .expect_err("practice owner must never pass physician Scope::validate");
        assert!(
            err.to_string()
                .contains("practice run cannot act as a physician"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn memory_reads_are_read_only_while_notes_are_not() {
        assert!(super::practice_job_tool_is_read_only(
            PracticeJobKind::VisitPreparation,
            "read_patient_memory"
        ));
        assert!(super::practice_job_tool_is_read_only(
            PracticeJobKind::PatientReview,
            "list_patient_memory"
        ));
        assert!(!super::practice_job_tool_is_read_only(
            PracticeJobKind::PatientReview,
            "add_patient_memory_note"
        ));
        assert!(!super::practice_job_tool_is_read_only(
            PracticeJobKind::InitialUnderstanding,
            "edit_patient_memory"
        ));
        assert!(super::practice_job_tool_is_read_only(
            PracticeJobKind::PatientReview,
            crate::physician_sources::READ_TOOL
        ));
    }
}
