//! Patient concerns raised, revised, and retired by Coda, and cleared `Done`
//! by authorized physicians sharing the patient.
//!
//! One store holds every concern for `(practice, workspace)`. Urgent concerns
//! carry [`ConcernUrgency::Now`]; routine concerns carry
//! [`ConcernUrgency::Today`]. Every concern cites verified exact chart record
//! revisions from [`super::sources`] and names the [`SourceWork`] that wrote
//! it. When a physician marks an open concern `Done`, it clears for every
//! physician sharing the patient and records durable evaluation feedback in the
//! same transaction. When Coda retires an open concern, its one-line
//! [`RetirementReason`] reads back in Updates and patient history.

use std::collections::BTreeMap;
use std::fmt;

use harness::engine::{self, Invocation, Tool, ToolOutcome};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, ExprTrait, QueryFilter,
    QueryOrder, QuerySelect, Set, TransactionTrait, Unchanged, sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::entities::{concern, concern_feedback};
use super::{Authority, Scope, Store, conversion_error, error, now, persistence_failure, receipt};

/// Longest clinical concern statement accepted, in characters.
pub const MAX_CONCERN_REASON_CHARS: usize = 2000;

/// Longest one-line retirement reason accepted for Updates, in characters.
pub const MAX_RETIREMENT_REASON_CHARS: usize = 500;

/// Maximum exact chart record references cited on one concern.
pub const MAX_CONCERN_RECORDS: usize = 16;

/// Tool name offered to background patient-review jobs for raising, revising,
/// and retiring concerns.
pub const CONCERN_TOOL: &str = "patient_concern";

/// Reserved receipt actor namespace for background job concern mutations so job
/// command keys never collide with physician client command keys.
const JOB_RECEIPT_ACTOR: &str = "coda:concern-job";

/// Sentinel scope chat slot used when checking patient workspace authority.
const AUTHORITY_CHAT_SLOT: &str = "concerns";

/// Whether `name` is the background job concern tool.
pub fn is_concern_tool(name: &str) -> bool {
    name == CONCERN_TOOL
}

/// Validation failure on a concern identifier, command key, or tool payload.
#[derive(Debug, thiserror::Error)]
pub enum ConcernInputError {
    /// A required scope or work field was blank or contained control characters.
    #[error("{0} must be non-empty without control characters")]
    InvalidField(&'static str),
    /// A concern identifier or command key failed validation.
    #[error("invalid identifier `{0}`: supply 1 to 256 trimmed visible characters")]
    InvalidIdentifier(String),
    /// A clinical concern statement was empty or exceeded [`MAX_CONCERN_REASON_CHARS`].
    #[error("concern reason must be 1 to {MAX_CONCERN_REASON_CHARS} characters")]
    InvalidReason,
    /// A retirement reason was empty, multiline, or exceeded [`MAX_RETIREMENT_REASON_CHARS`].
    #[error(
        "retirement reason must be a single non-empty line of at most {MAX_RETIREMENT_REASON_CHARS} characters"
    )]
    InvalidRetirementReason,
    /// A concern cited zero or too many chart records.
    #[error("cite between 1 and {MAX_CONCERN_RECORDS} exact chart record references")]
    InvalidRecordCount,
    /// Two cited references named the same candidate with different revisions.
    #[error("cited record `{candidate_id}` appears with conflicting revisions")]
    ConflictingRecordRevision {
        /// Candidate ID cited more than once with different revisions.
        candidate_id: String,
    },
    /// Tool JSON arguments did not match the offered schema.
    #[error(
        "supply a valid operation (`raise`, `revise`, or `retire`) and its required fields ({0})"
    )]
    MalformedArguments(#[source] serde_json::Error),
    /// Tool invocation named an unknown tool.
    #[error("unknown concern tool `{0}`")]
    UnknownTool(String),
}

/// Failure on a concern read or mutation. Each variant keeps its underlying cause.
#[derive(Debug, thiserror::Error)]
pub enum ConcernError {
    /// Input failed boundary validation.
    #[error("{0}")]
    InvalidInput(#[from] ConcernInputError),
    /// Physician session or patient panel authority was refused.
    #[error("patient workspace access refused: {reason}")]
    AccessDenied {
        /// Refusal reason reported by the authority owner.
        reason: String,
    },
    /// Command key was already used for a different mutation payload.
    #[error("command key `{key}` already recorded a different command")]
    IdempotencyCollision {
        /// Colliding command key.
        key: String,
    },
    /// Raising a concern reused an existing concern identifier in the workspace.
    #[error("concern `{id}` already exists in this patient workspace")]
    AlreadyExists {
        /// Existing concern identifier.
        id: ConcernId,
    },
    /// Revising, retiring, or clearing named a concern absent from the workspace.
    #[error("concern `{id}` does not exist in this patient workspace")]
    NotFound {
        /// Missing concern identifier.
        id: ConcernId,
    },
    /// Revising or retiring targeted a concern that is already cleared or retired.
    #[error("concern `{id}` is no longer open")]
    NotOpen {
        /// Closed concern identifier.
        id: ConcernId,
    },
    /// Cited chart record revision is absent or not current in the workspace.
    #[error(
        "cited chart record `{candidate_id}` at revision `{revision}` is not in the workspace's current retained sources"
    )]
    UnverifiedRecord {
        /// Cited candidate ID.
        candidate_id: String,
        /// Cited revision identity.
        revision: String,
    },
    /// Database error from SeaORM.
    #[error("{0}")]
    Database(#[from] DbErr),
}

fn valid_token(value: &str) -> bool {
    let count = value.chars().count();
    (1..=256).contains(&count) && value.trim() == value && !value.chars().any(char::is_control)
}

fn validate_non_empty_field(field: &'static str, value: &str) -> Result<String, ConcernInputError> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.chars().any(char::is_control) {
        return Err(ConcernInputError::InvalidField(field));
    }
    Ok(trimmed.to_owned())
}

fn validate_concern_reason(reason: &str) -> Result<String, ConcernInputError> {
    let trimmed = reason.trim();
    let count = trimmed.chars().count();
    if !(1..=MAX_CONCERN_REASON_CHARS).contains(&count)
        || trimmed
            .chars()
            .any(|ch| ch.is_control() && ch != '\n' && ch != '\t')
    {
        return Err(ConcernInputError::InvalidReason);
    }
    Ok(trimmed.to_owned())
}

/// Stable identifier of one concern within `(practice, workspace)`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ConcernId(String);

impl ConcernId {
    /// Parses a trimmed, non-empty concern identifier of at most 256 characters.
    pub fn new(value: impl Into<String>) -> Result<Self, ConcernInputError> {
        let value = value.into();
        if !valid_token(&value) {
            return Err(ConcernInputError::InvalidIdentifier(value));
        }
        Ok(Self(value))
    }

    /// Identifier text as stored.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ConcernId {
    type Error = ConcernInputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ConcernId> for String {
    fn from(id: ConcernId) -> Self {
        id.0
    }
}

impl fmt::Display for ConcernId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Once-only idempotency key for a concern command.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CommandKey(String);

impl CommandKey {
    /// Parses a trimmed, non-empty command key of at most 256 characters.
    pub fn new(value: impl Into<String>) -> Result<Self, ConcernInputError> {
        let value = value.into();
        if !valid_token(&value) {
            return Err(ConcernInputError::InvalidIdentifier(value));
        }
        Ok(Self(value))
    }

    /// Key text as stored in `physician_runtime_commands`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CommandKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Priority tier of a concern on the physician's Today queue.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConcernUrgency {
    /// Urgent clinical finding requiring immediate physician attention.
    Now,
    /// Actionable clinical finding queued for Today.
    Today,
}

impl ConcernUrgency {
    /// Stored column representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Now => "now",
            Self::Today => "today",
        }
    }

    fn parse(column: &str) -> Result<Self, DbErr> {
        match column {
            "now" => Ok(Self::Now),
            "today" => Ok(Self::Today),
            _ => Err(error("corrupt concern urgency")),
        }
    }

    const fn sort_rank(self) -> u8 {
        match self {
            Self::Now => 0,
            Self::Today => 1,
        }
    }
}

/// One-line physician-readable reason Coda retired a concern, shown in Updates.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct RetirementReason(String);

impl RetirementReason {
    /// Validates a non-empty single-line retirement reason.
    pub fn new(value: impl Into<String>) -> Result<Self, ConcernInputError> {
        let raw = value.into();
        if raw.contains('\n') || raw.contains('\r') {
            return Err(ConcernInputError::InvalidRetirementReason);
        }
        let trimmed = raw.trim();
        let count = trimmed.chars().count();
        if !(1..=MAX_RETIREMENT_REASON_CHARS).contains(&count)
            || trimmed.chars().any(char::is_control)
        {
            return Err(ConcernInputError::InvalidRetirementReason);
        }
        Ok(Self(trimmed.to_owned()))
    }

    /// Reason text as stored and displayed in Updates.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for RetirementReason {
    type Error = ConcernInputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<RetirementReason> for String {
    fn from(reason: RetirementReason) -> Self {
        reason.0
    }
}

impl fmt::Display for RetirementReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Exact chart record revision cited by a concern, matching `read_patient_sources`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExactRecordReference {
    /// Source owner's opaque candidate identifier.
    pub candidate_id: String,
    /// Source owner's immutable revision identity.
    pub revision: String,
}

impl ExactRecordReference {
    /// Constructs a validated exact chart record reference.
    pub fn new(
        candidate_id: impl Into<String>,
        revision: impl Into<String>,
    ) -> Result<Self, ConcernInputError> {
        Ok(Self {
            candidate_id: validate_non_empty_field("candidate_id", &candidate_id.into())?,
            revision: validate_non_empty_field("revision", &revision.into())?,
        })
    }
}

/// Background work execution that raised, revised, or retired a concern.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceWork {
    /// Kind or name of the background job.
    pub job: String,
    /// Durable work item identifier.
    pub work_id: String,
    /// Attempt identifier that executed the tool call.
    pub attempt_id: String,
}

impl SourceWork {
    /// Constructs a validated originating work reference.
    pub fn new(
        job: impl Into<String>,
        work_id: impl Into<String>,
        attempt_id: impl Into<String>,
    ) -> Result<Self, ConcernInputError> {
        Ok(Self {
            job: validate_non_empty_field("job", &job.into())?,
            work_id: validate_non_empty_field("work_id", &work_id.into())?,
            attempt_id: validate_non_empty_field("attempt_id", &attempt_id.into())?,
        })
    }

    fn validate(&self) -> Result<Self, ConcernInputError> {
        Self::new(&self.job, &self.work_id, &self.attempt_id)
    }
}

/// Lifecycle standing of a concern, holding each state's own fields.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ConcernStanding {
    /// Active on Today for every authorized physician sharing the patient.
    Open,
    /// Cleared `Done` by an authorized physician sharing the patient.
    Done {
        /// Physician who cleared the concern.
        physician: String,
        /// Database-clock seconds when `Done` was saved.
        cleared_at: i64,
        /// Revision of the saved [`ConcernFeedback`] record.
        feedback_revision: u64,
    },
    /// Retired by Coda with a one-line reason exposed in Updates.
    Retired {
        /// Single-line reason shown in Updates and patient history.
        reason: RetirementReason,
        /// Background work that retired the concern.
        source_work: SourceWork,
        /// Database-clock seconds when the concern was retired.
        retired_at: i64,
    },
}

/// One durable concern Coda saved for a patient workspace.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Concern {
    /// Practice tenant owning the patient link.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Stable concern identifier within `(practice, workspace)`.
    pub id: ConcernId,
    /// Priority tier (`Now` or `Today`).
    pub urgency: ConcernUrgency,
    /// Clinical statement explaining why the physician should see this concern.
    pub reason: String,
    /// Verified exact chart record revisions supporting the concern.
    pub records: Vec<ExactRecordReference>,
    /// Background work that raised or most recently revised the concern's statement.
    pub source_work: SourceWork,
    /// Monotonic revision counter (1 when raised; incremented on revise, retire, or `Done`).
    pub revision: u64,
    /// Database-clock seconds when first raised.
    pub created_at: i64,
    /// Database-clock seconds of the latest mutation.
    pub updated_at: i64,
    /// Current lifecycle standing (`Open`, `Done`, or `Retired`).
    pub standing: ConcernStanding,
}

/// Projection of a Coda-retired concern for the physician's Updates view.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetiredConcernUpdate {
    /// Concern identifier.
    pub id: ConcernId,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Statement the concern carried before retirement.
    pub concern_reason: String,
    /// One-line explanation of why Coda retired it.
    pub retirement_reason: RetirementReason,
    /// Cited chart record revisions retained with the concern.
    pub records: Vec<ExactRecordReference>,
    /// Work that last wrote the concern statement.
    pub raised_by_work: SourceWork,
    /// Work that retired the concern.
    pub retired_by_work: SourceWork,
    /// Database-clock seconds when retired.
    pub retired_at: i64,
    /// Concern revision at retirement.
    pub revision: u64,
}

impl Concern {
    /// Single rule deciding whether this concern appears on the Today queue.
    pub const fn shows_on_today(&self) -> bool {
        matches!(self.standing, ConcernStanding::Open)
    }

    /// Single rule projecting a retired concern into Updates.
    pub fn retired_update(&self) -> Option<RetiredConcernUpdate> {
        match &self.standing {
            ConcernStanding::Retired {
                reason,
                source_work,
                retired_at,
            } => Some(RetiredConcernUpdate {
                id: self.id.clone(),
                workspace: self.workspace.clone(),
                concern_reason: self.reason.clone(),
                retirement_reason: reason.clone(),
                records: self.records.clone(),
                raised_by_work: self.source_work.clone(),
                retired_by_work: source_work.clone(),
                retired_at: *retired_at,
                revision: self.revision,
            }),
            ConcernStanding::Open | ConcernStanding::Done { .. } => None,
        }
    }
}

/// Physician action recorded as concern feedback for evaluation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConcernFeedbackAction {
    /// Physician marked the concern `Done`.
    Done,
}

impl ConcernFeedbackAction {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Done => "done",
        }
    }

    fn parse(column: &str) -> Result<Self, DbErr> {
        match column {
            "done" => Ok(Self::Done),
            _ => Err(error("corrupt concern feedback action")),
        }
    }
}

/// Durable physician feedback recorded when a concern is marked `Done`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConcernFeedback {
    /// Practice tenant.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Cleared concern identifier.
    pub concern_id: ConcernId,
    /// Physician who cleared the concern `Done`.
    pub physician: String,
    /// Action recorded (`Done`).
    pub action: ConcernFeedbackAction,
    /// Revision of the concern after clearing.
    pub concern_revision: u64,
    /// Urgency snapshot of the concern when cleared.
    pub urgency: ConcernUrgency,
    /// Statement snapshot of the concern when cleared.
    pub concern_reason: String,
    /// Cited record references on the concern when cleared.
    pub records: Vec<ExactRecordReference>,
    /// Background work that produced the concern statement.
    pub source_work: SourceWork,
    /// Database-clock seconds when the feedback was saved.
    pub saved_at: i64,
    /// Feedback row revision (starts at 1).
    pub revision: u64,
}

/// Signed-in physician context required to read concerns or mark one `Done`.
#[derive(Clone, Copy, Debug)]
pub struct WorkspaceAccess<'a> {
    /// Practice tenant of the signed-in physician.
    pub practice: &'a str,
    /// Signed-in physician identifier.
    pub physician: &'a str,
    /// Target patient workspace identifier.
    pub workspace: &'a str,
    /// 32-byte SHA-256 session reference checked by [`Authority`].
    pub session_hash: &'a [u8],
}

impl WorkspaceAccess<'_> {
    fn authority_scope(&self) -> Result<Scope, ConcernError> {
        let practice = validate_non_empty_field("practice", self.practice)?;
        let physician = validate_non_empty_field("physician", self.physician)?;
        let workspace = validate_non_empty_field("workspace", self.workspace)?;
        if self.session_hash.len() != 32 {
            return Err(ConcernError::AccessDenied {
                reason: "invalid session reference".into(),
            });
        }
        Ok(Scope {
            practice,
            physician,
            workspace,
            chat: AUTHORITY_CHAT_SLOT.into(),
        })
    }
}

/// Typed input for raising a new patient concern.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RaiseConcern {
    /// Practice tenant owning the patient link.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// New concern identifier.
    pub id: ConcernId,
    /// Urgency (`Now` or `Today`).
    pub urgency: ConcernUrgency,
    /// Clinical statement for the physician.
    pub reason: String,
    /// Exact retained chart record revisions cited by the concern.
    pub records: Vec<ExactRecordReference>,
    /// Originating background work.
    pub source_work: SourceWork,
}

/// Typed input for revising an open patient concern.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReviseConcern {
    /// Practice tenant owning the patient link.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Open concern identifier to revise.
    pub id: ConcernId,
    /// Updated urgency (`Now` or `Today`).
    pub urgency: ConcernUrgency,
    /// Updated clinical statement for the physician.
    pub reason: String,
    /// Updated exact retained chart record revisions cited by the concern.
    pub records: Vec<ExactRecordReference>,
    /// Originating background work performing the revision.
    pub source_work: SourceWork,
}

/// Typed input for retiring an open patient concern with a one-line reason.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetireConcern {
    /// Practice tenant owning the patient link.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Open concern identifier to retire.
    pub id: ConcernId,
    /// Single-line explanation shown in Updates.
    pub reason: RetirementReason,
    /// Originating background work retiring the concern.
    pub source_work: SourceWork,
}

/// Outcome of a once-only job mutation (`raise`, `revise`, or `retire`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConcernMutation {
    /// New command applied and committed.
    Applied(Concern),
    /// Exact command key already committed earlier; no second change was made.
    Duplicate(Concern),
}

impl ConcernMutation {
    /// The durable concern state after this command.
    pub const fn concern(&self) -> &Concern {
        match self {
            Self::Applied(concern) | Self::Duplicate(concern) => concern,
        }
    }
}

/// Outcome of a physician marking a concern `Done`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConcernDoneOutcome {
    /// The open concern was cleared for every physician sharing the patient and
    /// saved as evaluation feedback.
    Cleared {
        /// Updated concern in [`ConcernStanding::Done`].
        concern: Box<Concern>,
        /// Durable feedback saved in the same transaction.
        feedback: Box<ConcernFeedback>,
    },
    /// Another physician sharing the patient had already cleared the concern.
    AlreadyCleared {
        /// Existing cleared concern.
        concern: Box<Concern>,
    },
    /// Exact command key was already accepted earlier; no second change was made.
    Duplicate {
        /// Current durable concern state.
        concern: Box<Concern>,
    },
}

/// Parsed call to [`CONCERN_TOOL`], discriminated by `operation`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum ConcernToolCall {
    /// Raise a new concern in the patient workspace.
    Raise {
        /// Concern identifier.
        concern_id: ConcernId,
        /// `now` or `today`.
        urgency: ConcernUrgency,
        /// Clinical statement for the physician.
        reason: String,
        /// Exact chart record revisions supporting the concern.
        records: Vec<ExactRecordReference>,
    },
    /// Revise an existing open concern's urgency, statement, or cited records.
    Revise {
        /// Open concern identifier.
        concern_id: ConcernId,
        /// Updated `now` or `today` urgency.
        urgency: ConcernUrgency,
        /// Updated clinical statement.
        reason: String,
        /// Updated exact chart record revisions supporting the concern.
        records: Vec<ExactRecordReference>,
    },
    /// Retire an existing open concern with a one-line explanation for Updates.
    Retire {
        /// Open concern identifier.
        concern_id: ConcernId,
        /// Single-line reason for Updates.
        reason: RetirementReason,
    },
}

impl ConcernToolCall {
    /// Parses and validates tool arguments before touching Postgres.
    pub fn parse(name: &str, arguments: &Value) -> Result<Self, ConcernInputError> {
        if !is_concern_tool(name) {
            return Err(ConcernInputError::UnknownTool(name.to_owned()));
        }
        let call: Self = serde_json::from_value(arguments.clone())
            .map_err(ConcernInputError::MalformedArguments)?;
        match &call {
            Self::Raise {
                reason, records, ..
            }
            | Self::Revise {
                reason, records, ..
            } => {
                validate_concern_reason(reason)?;
                normalize_records(records)?;
            }
            Self::Retire { .. } => {}
        }
        Ok(call)
    }
}

/// Tool definition offered to background patient jobs so Coda can raise, revise,
/// or retire patient concerns grounded in exact chart record revisions.
pub fn concern_tool() -> Tool {
    let record_ref = json!({
        "type": "object",
        "properties": {
            "candidate_id": { "type": "string" },
            "revision": { "type": "string" }
        },
        "required": ["candidate_id", "revision"],
        "additionalProperties": false
    });
    let urgency = json!({
        "type": "string",
        "enum": ["now", "today"]
    });
    Tool {
        name: CONCERN_TOOL.into(),
        description: "Raise, revise, or retire a clinical concern for this patient's Today queue. Mark urgent findings `now` and routine actionable findings `today`. Every raised or revised concern must cite the exact `candidate_id` and `revision` of each supporting chart record read in this workspace. Unrelated clinical issues belong in separate concerns. When new evidence resolves or supersedes an open concern, retire it with a single-line physician-readable reason for Updates.".into(),
        parameters: json!({
            "anyOf": [
                {
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string", "enum": ["raise"] },
                        "concern_id": { "type": "string" },
                        "urgency": urgency,
                        "reason": { "type": "string" },
                        "records": {
                            "type": "array",
                            "items": record_ref,
                            "minItems": 1,
                            "maxItems": MAX_CONCERN_RECORDS
                        }
                    },
                    "required": ["operation", "concern_id", "urgency", "reason", "records"],
                    "additionalProperties": false
                },
                {
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string", "enum": ["revise"] },
                        "concern_id": { "type": "string" },
                        "urgency": urgency,
                        "reason": { "type": "string" },
                        "records": {
                            "type": "array",
                            "items": record_ref,
                            "minItems": 1,
                            "maxItems": MAX_CONCERN_RECORDS
                        }
                    },
                    "required": ["operation", "concern_id", "urgency", "reason", "records"],
                    "additionalProperties": false
                },
                {
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string", "enum": ["retire"] },
                        "concern_id": { "type": "string" },
                        "reason": { "type": "string" }
                    },
                    "required": ["operation", "concern_id", "reason"],
                    "additionalProperties": false
                }
            ]
        }),
    }
}

fn normalize_records(
    records: &[ExactRecordReference],
) -> Result<Vec<ExactRecordReference>, ConcernInputError> {
    if !(1..=MAX_CONCERN_RECORDS).contains(&records.len()) {
        return Err(ConcernInputError::InvalidRecordCount);
    }
    let mut seen_revisions: BTreeMap<String, String> = BTreeMap::new();
    let mut normalized = Vec::with_capacity(records.len());
    for record in records {
        let checked = ExactRecordReference::new(&record.candidate_id, &record.revision)?;
        if let Some(existing) = seen_revisions.get(&checked.candidate_id) {
            if existing != &checked.revision {
                return Err(ConcernInputError::ConflictingRecordRevision {
                    candidate_id: checked.candidate_id,
                });
            }
            continue;
        }
        seen_revisions.insert(checked.candidate_id.clone(), checked.revision.clone());
        normalized.push(checked);
    }
    Ok(normalized)
}

async fn verify_cited_records(
    tx: &impl ConnectionTrait,
    workspace: &str,
    records: &[ExactRecordReference],
) -> Result<(), ConcernError> {
    let observation = super::sources::observed(tx, workspace).await?;
    for cited in records {
        let present = observation.sources.iter().any(|source| {
            source.workspace == workspace
                && source.candidate_id == cited.candidate_id
                && source.revision == cited.revision
        });
        if !present {
            return Err(ConcernError::UnverifiedRecord {
                candidate_id: cited.candidate_id.clone(),
                revision: cited.revision.clone(),
            });
        }
    }
    Ok(())
}

fn positive_u64(value: i64, what: &'static str) -> Result<u64, DbErr> {
    u64::try_from(value)
        .map_err(conversion_error(what, "u64"))?
        .checked_sub(0)
        .filter(|revision| *revision > 0)
        .ok_or_else(|| error(what))
}

fn decode_concern(row: concern::Model) -> Result<Concern, DbErr> {
    let id = ConcernId::new(row.id).map_err(conversion_error("stored concern id", "ConcernId"))?;
    let urgency = ConcernUrgency::parse(&row.urgency)?;
    let records: Vec<ExactRecordReference> = serde_json::from_value(row.records).map_err(
        conversion_error("stored concern records", "ExactRecordReference"),
    )?;
    let source_work: SourceWork = serde_json::from_value(row.source_work)
        .map_err(conversion_error("stored concern source_work", "SourceWork"))?;
    let revision = positive_u64(row.revision, "corrupt concern revision")?;
    let standing = match row.status.as_str() {
        "open" => ConcernStanding::Open,
        "done" => {
            let physician = row
                .cleared_by_physician
                .ok_or_else(|| error("corrupt done concern missing physician"))?;
            let cleared_at = row
                .cleared_at
                .ok_or_else(|| error("corrupt done concern missing cleared_at"))?;
            let feedback_revision = positive_u64(
                row.feedback_revision
                    .ok_or_else(|| error("corrupt done concern missing feedback_revision"))?,
                "corrupt done concern feedback_revision",
            )?;
            ConcernStanding::Done {
                physician,
                cleared_at,
                feedback_revision,
            }
        }
        "retired" => {
            let reason_raw = row
                .retirement_reason
                .ok_or_else(|| error("corrupt retired concern missing reason"))?;
            let reason = RetirementReason::new(reason_raw).map_err(conversion_error(
                "stored retirement reason",
                "RetirementReason",
            ))?;
            let work_json = row
                .retired_by_work
                .ok_or_else(|| error("corrupt retired concern missing source_work"))?;
            let retired_work: SourceWork = serde_json::from_value(work_json)
                .map_err(conversion_error("stored retired_by_work", "SourceWork"))?;
            let retired_at = row
                .retired_at
                .ok_or_else(|| error("corrupt retired concern missing retired_at"))?;
            ConcernStanding::Retired {
                reason,
                source_work: retired_work,
                retired_at,
            }
        }
        _ => return Err(error("corrupt concern status")),
    };
    Ok(Concern {
        practice: row.practice,
        workspace: row.workspace,
        id,
        urgency,
        reason: row.reason,
        records,
        source_work,
        revision,
        created_at: row.created_at,
        updated_at: row.updated_at,
        standing,
    })
}

fn decode_feedback(row: concern_feedback::Model) -> Result<ConcernFeedback, DbErr> {
    let concern_id = ConcernId::new(row.concern)
        .map_err(conversion_error("stored feedback concern id", "ConcernId"))?;
    let action = ConcernFeedbackAction::parse(&row.action)?;
    let concern_revision = positive_u64(row.concern_revision, "corrupt feedback concern_revision")?;
    let urgency = ConcernUrgency::parse(&row.urgency)?;
    let records: Vec<ExactRecordReference> = serde_json::from_value(row.records).map_err(
        conversion_error("stored feedback records", "ExactRecordReference"),
    )?;
    let source_work: SourceWork = serde_json::from_value(row.source_work).map_err(
        conversion_error("stored feedback source_work", "SourceWork"),
    )?;
    let revision = positive_u64(row.revision, "corrupt concern feedback revision")?;
    Ok(ConcernFeedback {
        practice: row.practice,
        workspace: row.workspace,
        concern_id,
        physician: row.physician,
        action,
        concern_revision,
        urgency,
        concern_reason: row.concern_reason,
        records,
        source_work,
        saved_at: row.saved_at,
        revision,
    })
}

fn sort_today(concerns: &mut [Concern]) {
    concerns.sort_by(|left, right| {
        left.urgency
            .sort_rank()
            .cmp(&right.urgency.sort_rank())
            .then_with(|| left.created_at.cmp(&right.created_at))
            .then_with(|| left.id.cmp(&right.id))
    });
}

async fn load_concern(
    tx: &impl ConnectionTrait,
    practice: &str,
    workspace: &str,
    id: &ConcernId,
) -> Result<Option<Concern>, DbErr> {
    concern::Entity::find_by_id((
        practice.to_owned(),
        workspace.to_owned(),
        id.as_str().to_owned(),
    ))
    .one(tx)
    .await?
    .map(decode_concern)
    .transpose()
}

impl Store {
    /// Raises a new patient concern once under `command_key`, verifying every
    /// cited chart record revision against the workspace's current observation.
    pub async fn raise_concern(
        &self,
        command_key: &CommandKey,
        input: &RaiseConcern,
    ) -> Result<ConcernMutation, ConcernError> {
        let practice = validate_non_empty_field("practice", &input.practice)?;
        let workspace = validate_non_empty_field("workspace", &input.workspace)?;
        let reason = validate_concern_reason(&input.reason)?;
        let records = normalize_records(&input.records)?;
        let source_work = input.source_work.validate()?;

        let command = json!({
            "kind": "concern_raise",
            "practice": &practice,
            "workspace": &workspace,
            "concern_id": input.id.as_str(),
            "urgency": input.urgency,
            "reason": &reason,
            "records": &records,
            "source_work": &source_work,
        });
        let tx = self.db.begin().await?;
        match receipt::record(
            &tx,
            &practice,
            JOB_RECEIPT_ACTOR,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            receipt::Receipt::New => {}
            receipt::Receipt::Same => {
                let existing = load_concern(&tx, &practice, &workspace, &input.id)
                    .await?
                    .ok_or_else(|| error("missing concern for accepted receipt"))?;
                tx.commit().await?;
                return Ok(ConcernMutation::Duplicate(existing));
            }
            receipt::Receipt::Different => {
                return Err(ConcernError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
        }

        if concern::Entity::find_by_id((
            practice.clone(),
            workspace.clone(),
            input.id.as_str().to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        .is_some()
        {
            return Err(ConcernError::AlreadyExists {
                id: input.id.clone(),
            });
        }

        verify_cited_records(&tx, &workspace, &records).await?;
        let timestamp = now(&tx).await?;
        let records_json = serde_json::to_value(&records)
            .map_err(conversion_error("ExactRecordReference", "json"))?;
        let work_json =
            serde_json::to_value(&source_work).map_err(conversion_error("SourceWork", "json"))?;
        concern::Entity::insert(concern::ActiveModel {
            practice: Set(practice.clone()),
            workspace: Set(workspace.clone()),
            id: Set(input.id.as_str().to_owned()),
            urgency: Set(input.urgency.as_str().to_owned()),
            status: Set("open".into()),
            reason: Set(reason.clone()),
            records: Set(records_json),
            source_work: Set(work_json),
            cleared_by_physician: Set(None),
            cleared_at: Set(None),
            feedback_revision: Set(None),
            retirement_reason: Set(None),
            retired_by_work: Set(None),
            retired_at: Set(None),
            created_at: Set(timestamp),
            updated_at: Set(timestamp),
            revision: Set(1),
        })
        .exec_without_returning(&tx)
        .await?;
        tx.commit().await?;

        Ok(ConcernMutation::Applied(Concern {
            practice,
            workspace,
            id: input.id.clone(),
            urgency: input.urgency,
            reason,
            records,
            source_work,
            revision: 1,
            created_at: timestamp,
            updated_at: timestamp,
            standing: ConcernStanding::Open,
        }))
    }

    /// Revises an open patient concern once under `command_key`, verifying the
    /// updated exact chart record citations against the workspace's sources.
    pub async fn revise_concern(
        &self,
        command_key: &CommandKey,
        input: &ReviseConcern,
    ) -> Result<ConcernMutation, ConcernError> {
        let practice = validate_non_empty_field("practice", &input.practice)?;
        let workspace = validate_non_empty_field("workspace", &input.workspace)?;
        let reason = validate_concern_reason(&input.reason)?;
        let records = normalize_records(&input.records)?;
        let source_work = input.source_work.validate()?;

        let command = json!({
            "kind": "concern_revise",
            "practice": &practice,
            "workspace": &workspace,
            "concern_id": input.id.as_str(),
            "urgency": input.urgency,
            "reason": &reason,
            "records": &records,
            "source_work": &source_work,
        });
        let tx = self.db.begin().await?;
        match receipt::record(
            &tx,
            &practice,
            JOB_RECEIPT_ACTOR,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            receipt::Receipt::New => {}
            receipt::Receipt::Same => {
                let existing = load_concern(&tx, &practice, &workspace, &input.id)
                    .await?
                    .ok_or_else(|| error("missing concern for accepted receipt"))?;
                tx.commit().await?;
                return Ok(ConcernMutation::Duplicate(existing));
            }
            receipt::Receipt::Different => {
                return Err(ConcernError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
        }

        let Some(row) = concern::Entity::find_by_id((
            practice.clone(),
            workspace.clone(),
            input.id.as_str().to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        else {
            return Err(ConcernError::NotFound {
                id: input.id.clone(),
            });
        };
        let current = decode_concern(row.clone())?;
        if !current.shows_on_today() {
            return Err(ConcernError::NotOpen {
                id: input.id.clone(),
            });
        }

        verify_cited_records(&tx, &workspace, &records).await?;
        let timestamp = now(&tx).await?;
        let next_revision = current.revision + 1;
        let next_revision_i64 =
            i64::try_from(next_revision).map_err(conversion_error("concern revision", "i64"))?;
        let records_json = serde_json::to_value(&records)
            .map_err(conversion_error("ExactRecordReference", "json"))?;
        let work_json =
            serde_json::to_value(&source_work).map_err(conversion_error("SourceWork", "json"))?;

        concern::ActiveModel {
            practice: Unchanged(row.practice),
            workspace: Unchanged(row.workspace),
            id: Unchanged(row.id),
            urgency: Set(input.urgency.as_str().to_owned()),
            status: Unchanged(row.status),
            reason: Set(reason.clone()),
            records: Set(records_json),
            source_work: Set(work_json),
            cleared_by_physician: Unchanged(row.cleared_by_physician),
            cleared_at: Unchanged(row.cleared_at),
            feedback_revision: Unchanged(row.feedback_revision),
            retirement_reason: Unchanged(row.retirement_reason),
            retired_by_work: Unchanged(row.retired_by_work),
            retired_at: Unchanged(row.retired_at),
            created_at: Unchanged(row.created_at),
            updated_at: Set(timestamp),
            revision: Set(next_revision_i64),
        }
        .update(&tx)
        .await?;
        tx.commit().await?;

        Ok(ConcernMutation::Applied(Concern {
            practice,
            workspace,
            id: input.id.clone(),
            urgency: input.urgency,
            reason,
            records,
            source_work,
            revision: next_revision,
            created_at: current.created_at,
            updated_at: timestamp,
            standing: ConcernStanding::Open,
        }))
    }

    /// Retires an open patient concern once under `command_key` with a one-line
    /// explanation exposed in Updates and patient history.
    pub async fn retire_concern(
        &self,
        command_key: &CommandKey,
        input: &RetireConcern,
    ) -> Result<ConcernMutation, ConcernError> {
        let practice = validate_non_empty_field("practice", &input.practice)?;
        let workspace = validate_non_empty_field("workspace", &input.workspace)?;
        let source_work = input.source_work.validate()?;

        let command = json!({
            "kind": "concern_retire",
            "practice": &practice,
            "workspace": &workspace,
            "concern_id": input.id.as_str(),
            "reason": input.reason.as_str(),
            "source_work": &source_work,
        });
        let tx = self.db.begin().await?;
        match receipt::record(
            &tx,
            &practice,
            JOB_RECEIPT_ACTOR,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            receipt::Receipt::New => {}
            receipt::Receipt::Same => {
                let existing = load_concern(&tx, &practice, &workspace, &input.id)
                    .await?
                    .ok_or_else(|| error("missing concern for accepted receipt"))?;
                tx.commit().await?;
                return Ok(ConcernMutation::Duplicate(existing));
            }
            receipt::Receipt::Different => {
                return Err(ConcernError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
        }

        let Some(row) = concern::Entity::find_by_id((
            practice.clone(),
            workspace.clone(),
            input.id.as_str().to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        else {
            return Err(ConcernError::NotFound {
                id: input.id.clone(),
            });
        };
        let current = decode_concern(row.clone())?;
        if !current.shows_on_today() {
            return Err(ConcernError::NotOpen {
                id: input.id.clone(),
            });
        }

        let timestamp = now(&tx).await?;
        let next_revision = current.revision + 1;
        let next_revision_i64 =
            i64::try_from(next_revision).map_err(conversion_error("concern revision", "i64"))?;
        let retired_work_json =
            serde_json::to_value(&source_work).map_err(conversion_error("SourceWork", "json"))?;

        concern::ActiveModel {
            practice: Unchanged(row.practice),
            workspace: Unchanged(row.workspace),
            id: Unchanged(row.id),
            urgency: Unchanged(row.urgency),
            status: Set("retired".into()),
            reason: Unchanged(row.reason),
            records: Unchanged(row.records),
            source_work: Unchanged(row.source_work),
            cleared_by_physician: Set(None),
            cleared_at: Set(None),
            feedback_revision: Set(None),
            retirement_reason: Set(Some(input.reason.as_str().to_owned())),
            retired_by_work: Set(Some(retired_work_json)),
            retired_at: Set(Some(timestamp)),
            created_at: Unchanged(row.created_at),
            updated_at: Set(timestamp),
            revision: Set(next_revision_i64),
        }
        .update(&tx)
        .await?;
        tx.commit().await?;

        Ok(ConcernMutation::Applied(Concern {
            standing: ConcernStanding::Retired {
                reason: input.reason.clone(),
                source_work,
                retired_at: timestamp,
            },
            revision: next_revision,
            updated_at: timestamp,
            ..current
        }))
    }

    /// Executes one [`CONCERN_TOOL`] call for a background patient job.
    ///
    /// Model-correctable domain errors (such as citing an unverified record,
    /// raising an existing ID, or retiring a closed concern) return
    /// [`ToolOutcome::refused`] so the job can adjust; Postgres failures
    /// propagate through [`persistence_failure`].
    pub async fn execute_concern_tool(
        &self,
        practice: &str,
        workspace: &str,
        source_work: &SourceWork,
        call: &Invocation<Value>,
    ) -> Result<ToolOutcome, engine::ProductError> {
        let parsed = match ConcernToolCall::parse(&call.name, &call.arguments) {
            Ok(parsed) => parsed,
            Err(reason) => return Ok(ToolOutcome::refused(reason.to_string())),
        };
        let validated_work = match source_work.validate() {
            Ok(work) => work,
            Err(reason) => return Err((Box::new(reason) as engine::Cause).into()),
        };
        let raw_key = format!("{}:{}", validated_work.work_id, call.id.trim());
        let command_key = match CommandKey::new(raw_key) {
            Ok(key) => key,
            Err(reason) => return Err((Box::new(reason) as engine::Cause).into()),
        };

        let outcome = match parsed {
            ConcernToolCall::Raise {
                concern_id,
                urgency,
                reason,
                records,
            } => {
                self.raise_concern(
                    &command_key,
                    &RaiseConcern {
                        practice: practice.to_owned(),
                        workspace: workspace.to_owned(),
                        id: concern_id,
                        urgency,
                        reason,
                        records,
                        source_work: validated_work,
                    },
                )
                .await
            }
            ConcernToolCall::Revise {
                concern_id,
                urgency,
                reason,
                records,
            } => {
                self.revise_concern(
                    &command_key,
                    &ReviseConcern {
                        practice: practice.to_owned(),
                        workspace: workspace.to_owned(),
                        id: concern_id,
                        urgency,
                        reason,
                        records,
                        source_work: validated_work,
                    },
                )
                .await
            }
            ConcernToolCall::Retire { concern_id, reason } => {
                self.retire_concern(
                    &command_key,
                    &RetireConcern {
                        practice: practice.to_owned(),
                        workspace: workspace.to_owned(),
                        id: concern_id,
                        reason,
                        source_work: validated_work,
                    },
                )
                .await
            }
        };

        match outcome {
            Ok(ConcernMutation::Applied(concern)) => Ok(ToolOutcome::Returned(
                json!({
                    "outcome": "applied",
                    "concern": concern,
                })
                .to_string(),
            )),
            Ok(ConcernMutation::Duplicate(concern)) => Ok(ToolOutcome::Returned(
                json!({
                    "outcome": "duplicate",
                    "concern": concern,
                })
                .to_string(),
            )),
            Err(
                error @ (ConcernError::InvalidInput(_)
                | ConcernError::AlreadyExists { .. }
                | ConcernError::NotFound { .. }
                | ConcernError::NotOpen { .. }
                | ConcernError::UnverifiedRecord { .. }
                | ConcernError::IdempotencyCollision { .. }),
            ) => Ok(ToolOutcome::refused(error.to_string())),
            Err(ConcernError::AccessDenied { reason }) => Err(engine::Cause::from(reason).into()),
            Err(ConcernError::Database(db_err)) => Err(persistence_failure(db_err)),
        }
    }

    /// Marks an open concern `Done` on behalf of an authorized physician,
    /// clearing it for every physician sharing the patient and recording
    /// evaluation feedback in the same transaction.
    pub async fn mark_concern_done(
        &self,
        access: &WorkspaceAccess<'_>,
        command_key: &CommandKey,
        concern_id: &ConcernId,
        authority: &mut dyn Authority,
    ) -> Result<ConcernDoneOutcome, ConcernError> {
        use concern_feedback::Column as F;

        let scope = access.authority_scope()?;
        let tx = self.db.begin().await?;
        if let Err(reason) = authority
            .lock_publication(&tx, &scope, Some(access.session_hash))
            .await
        {
            tx.rollback().await?;
            return Err(ConcernError::AccessDenied { reason });
        }

        let command = json!({
            "kind": "concern_done",
            "practice": &scope.practice,
            "physician": &scope.physician,
            "workspace": &scope.workspace,
            "concern_id": concern_id.as_str(),
        });
        match receipt::record(
            &tx,
            &scope.practice,
            &scope.physician,
            command_key.as_str(),
            &command,
        )
        .await?
        {
            receipt::Receipt::New => {}
            receipt::Receipt::Same => {
                let concern = load_concern(&tx, &scope.practice, &scope.workspace, concern_id)
                    .await?
                    .ok_or_else(|| error("missing concern for accepted Done receipt"))?;
                tx.commit().await?;
                return Ok(ConcernDoneOutcome::Duplicate {
                    concern: Box::new(concern),
                });
            }
            receipt::Receipt::Different => {
                tx.rollback().await?;
                return Err(ConcernError::IdempotencyCollision {
                    key: command_key.to_string(),
                });
            }
        }

        let Some(row) = concern::Entity::find_by_id((
            scope.practice.clone(),
            scope.workspace.clone(),
            concern_id.as_str().to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        else {
            tx.rollback().await?;
            return Err(ConcernError::NotFound {
                id: concern_id.clone(),
            });
        };
        let current = decode_concern(row.clone())?;
        match &current.standing {
            ConcernStanding::Done { .. } => {
                tx.commit().await?;
                return Ok(ConcernDoneOutcome::AlreadyCleared {
                    concern: Box::new(current),
                });
            }
            ConcernStanding::Retired { .. } => {
                tx.rollback().await?;
                return Err(ConcernError::NotOpen {
                    id: concern_id.clone(),
                });
            }
            ConcernStanding::Open => {}
        }

        let timestamp = now(&tx).await?;
        let next_revision = current.revision + 1;
        let next_revision_i64 =
            i64::try_from(next_revision).map_err(conversion_error("concern revision", "i64"))?;

        concern_feedback::Entity::insert(concern_feedback::ActiveModel {
            practice: Set(scope.practice.clone()),
            workspace: Set(scope.workspace.clone()),
            concern: Set(concern_id.as_str().to_owned()),
            physician: Set(scope.physician.clone()),
            action: Set(ConcernFeedbackAction::Done.as_str().to_owned()),
            concern_revision: Set(next_revision_i64),
            urgency: Set(current.urgency.as_str().to_owned()),
            concern_reason: Set(current.reason.clone()),
            records: Set(row.records.clone()),
            source_work: Set(row.source_work.clone()),
            saved_at: Set(timestamp),
            revision: Set(1),
        })
        .on_conflict(
            OnConflict::columns([F::Practice, F::Workspace, F::Concern])
                .update_columns([
                    F::Physician,
                    F::Action,
                    F::ConcernRevision,
                    F::Urgency,
                    F::ConcernReason,
                    F::Records,
                    F::SourceWork,
                    F::SavedAt,
                ])
                .value(
                    F::Revision,
                    sea_orm::sea_query::Expr::col((concern_feedback::Entity, F::Revision)).add(1),
                )
                .to_owned(),
        )
        .exec_without_returning(&tx)
        .await?;

        let feedback_row = concern_feedback::Entity::find_by_id((
            scope.practice.clone(),
            scope.workspace.clone(),
            concern_id.as_str().to_owned(),
        ))
        .one(&tx)
        .await?
        .ok_or_else(|| error("missing saved concern feedback"))?;
        let feedback = decode_feedback(feedback_row)?;
        let feedback_revision_i64 = i64::try_from(feedback.revision)
            .map_err(conversion_error("feedback revision", "i64"))?;

        concern::ActiveModel {
            practice: Unchanged(row.practice),
            workspace: Unchanged(row.workspace),
            id: Unchanged(row.id),
            urgency: Unchanged(row.urgency),
            status: Set("done".into()),
            reason: Unchanged(row.reason),
            records: Unchanged(row.records),
            source_work: Unchanged(row.source_work),
            cleared_by_physician: Set(Some(scope.physician.clone())),
            cleared_at: Set(Some(timestamp)),
            feedback_revision: Set(Some(feedback_revision_i64)),
            retirement_reason: Set(None),
            retired_by_work: Set(None),
            retired_at: Set(None),
            created_at: Unchanged(row.created_at),
            updated_at: Set(timestamp),
            revision: Set(next_revision_i64),
        }
        .update(&tx)
        .await?;
        tx.commit().await?;

        let cleared = Concern {
            standing: ConcernStanding::Done {
                physician: scope.physician,
                cleared_at: timestamp,
                feedback_revision: feedback.revision,
            },
            revision: next_revision,
            updated_at: timestamp,
            ..current
        };
        Ok(ConcernDoneOutcome::Cleared {
            concern: Box::new(cleared),
            feedback: Box::new(feedback),
        })
    }

    /// Reads open Today concerns for one patient workspace after checking the
    /// signed-in physician's workspace authority. Ordered with [`ConcernUrgency::Now`]
    /// ahead of [`ConcernUrgency::Today`], then by longest wait (`created_at` ascending).
    pub async fn open_concerns(
        &self,
        access: &WorkspaceAccess<'_>,
        authority: &mut dyn Authority,
    ) -> Result<Vec<Concern>, ConcernError> {
        let all = self.workspace_concerns(access, authority).await?;
        let mut open: Vec<Concern> = all.into_iter().filter(Concern::shows_on_today).collect();
        sort_today(&mut open);
        Ok(open)
    }

    /// Reads retired concern entries for the physician's Updates view in one
    /// patient workspace, newest retirement first, after checking workspace authority.
    pub async fn retired_concern_updates(
        &self,
        access: &WorkspaceAccess<'_>,
        authority: &mut dyn Authority,
    ) -> Result<Vec<RetiredConcernUpdate>, ConcernError> {
        let all = self.workspace_concerns(access, authority).await?;
        let mut retired: Vec<RetiredConcernUpdate> =
            all.iter().filter_map(Concern::retired_update).collect();
        retired.sort_by(|left, right| {
            right
                .retired_at
                .cmp(&left.retired_at)
                .then_with(|| left.id.cmp(&right.id))
        });
        Ok(retired)
    }

    /// Reads every concern (`Open`, `Done`, and `Retired`) in one patient
    /// workspace after checking the physician's session and panel authority.
    pub async fn workspace_concerns(
        &self,
        access: &WorkspaceAccess<'_>,
        authority: &mut dyn Authority,
    ) -> Result<Vec<Concern>, ConcernError> {
        use concern::Column as C;

        let scope = access.authority_scope()?;
        let tx = self.db.begin().await?;
        if let Err(reason) = authority
            .check(&tx, &scope, Some(access.session_hash))
            .await
        {
            tx.rollback().await?;
            return Err(ConcernError::AccessDenied { reason });
        }
        let rows = concern::Entity::find()
            .filter(C::Practice.eq(scope.practice.as_str()))
            .filter(C::Workspace.eq(scope.workspace.as_str()))
            .order_by_asc(C::Id)
            .all(&tx)
            .await?;
        tx.commit().await?;
        rows.into_iter()
            .map(decode_concern)
            .collect::<Result<Vec<_>, _>>()
            .map_err(ConcernError::from)
    }

    /// Reads all saved concern `Done` feedback rows for evaluation, ordered by
    /// practice, workspace, and concern identifier.
    pub async fn concern_feedback_for_evaluation(&self) -> Result<Vec<ConcernFeedback>, DbErr> {
        use concern_feedback::Column as F;

        concern_feedback::Entity::find()
            .order_by_asc(F::Practice)
            .order_by_asc(F::Workspace)
            .order_by_asc(F::Concern)
            .all(&self.db)
            .await?
            .into_iter()
            .map(decode_feedback)
            .collect()
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn concern_tool_schema_parses_each_discriminated_operation() {
        let tool = concern_tool();
        assert_eq!(tool.name, CONCERN_TOOL);

        let raised = ConcernToolCall::parse(
            CONCERN_TOOL,
            &json!({
                "operation": "raise",
                "concern_id": "k-hyperkalemia",
                "urgency": "now",
                "reason": "Potassium 6.1 mmol/L on basic metabolic panel.",
                "records": [{"candidate_id": "lab-bmp-1", "revision": "rev-1"}]
            }),
        )
        .unwrap();
        assert!(matches!(
            raised,
            ConcernToolCall::Raise {
                urgency: ConcernUrgency::Now,
                ..
            }
        ));

        let retired = ConcernToolCall::parse(
            CONCERN_TOOL,
            &json!({
                "operation": "retire",
                "concern_id": "k-hyperkalemia",
                "reason": "Repeat serum potassium normalized to 4.4 mmol/L without hemolysis."
            }),
        )
        .unwrap();
        assert!(matches!(retired, ConcernToolCall::Retire { .. }));

        // Multiline retirement reason is rejected for Updates.
        assert!(
            ConcernToolCall::parse(
                CONCERN_TOOL,
                &json!({
                    "operation": "retire",
                    "concern_id": "k-hyperkalemia",
                    "reason": "Line one\nLine two"
                }),
            )
            .is_err()
        );

        // Conflicting revisions for the same candidate ID are rejected.
        assert!(
            ConcernToolCall::parse(
                CONCERN_TOOL,
                &json!({
                    "operation": "revise",
                    "concern_id": "k-hyperkalemia",
                    "urgency": "today",
                    "reason": "Updated potassium check.",
                    "records": [
                        {"candidate_id": "lab-bmp-1", "revision": "rev-1"},
                        {"candidate_id": "lab-bmp-1", "revision": "rev-2"}
                    ]
                }),
            )
            .is_err()
        );
    }

    #[test]
    fn sort_today_orders_urgency_then_longest_wait_created_at() {
        let sample =
            |id: &str, urgency: ConcernUrgency, created_at: i64, updated_at: i64| Concern {
                practice: "p1".into(),
                workspace: "pw-1".into(),
                id: ConcernId::new(id).unwrap(),
                urgency,
                reason: "Check lab.".into(),
                records: vec![ExactRecordReference::new("c1", "r1").unwrap()],
                source_work: SourceWork::new("job", "w1", "a1").unwrap(),
                revision: 1,
                created_at,
                updated_at,
                standing: ConcernStanding::Open,
            };

        let mut list = vec![
            sample("a-newer-today", ConcernUrgency::Today, 200, 900),
            sample("z-older-today", ConcernUrgency::Today, 100, 150),
            sample("m-newest-now", ConcernUrgency::Now, 500, 500),
            sample("b-older-now", ConcernUrgency::Now, 300, 300),
        ];
        sort_today(&mut list);
        let ordered_ids: Vec<&str> = list.iter().map(|c| c.id.as_str()).collect();
        assert_eq!(
            ordered_ids,
            vec![
                "b-older-now",
                "m-newest-now",
                "z-older-today",
                "a-newer-today"
            ]
        );
    }
}
