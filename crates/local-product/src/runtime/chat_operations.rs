//! Physician chat recipe operations: starting context assembly, distinct
//! evidence limit classification (`missing`, `unreadable`, `unavailable`),
//! authorized chat admission, durable patient memory with physician provenance,
//! cross-conversation work retrieval, source refresh, and unified tool routing.

use std::fmt;
use std::sync::Arc;

use async_trait::async_trait;
use harness::engine::{self, Invocation, Item, Tool, ToolOutcome};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set, TransactionTrait, Unchanged,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::concerns::{ConcernUrgency, ExactRecordReference, SourceWork};
use super::entities::{command, concern, patient_dispatch, patient_memory, practice_job};
use super::jobs::PracticeJobKind;
use super::sources::SourcePull;
use super::{
    Accepted, Authority, Conversation, Input, RuntimeError, Scope, Store, WorkStatus,
    conversion_error, error, now, persistence_failure, receipt,
};
use crate::physician_chat::PhysicianSkills;
use crate::physician_sources::{
    ChartRequest, ChartScope, PatientSource, READ_TOOL, SEARCH_TOOL, SourcePatient, SourceRevision,
};

/// Tool name offered in physician chat to refresh patient source records on demand.
pub const REFRESH_SOURCES_TOOL: &str = "refresh_patient_sources";

/// Tool name offered in physician chat to read, record, or retire durable patient memory.
pub const PATIENT_MEMORY_TOOL: &str = "patient_memory";

/// Tool name offered in physician chat to list and read earlier conversations and saved work.
pub const RETRIEVE_PATIENT_WORK_TOOL: &str = "retrieve_patient_work";

/// Maximum length of a memory topic label, in characters.
pub const MAX_MEMORY_TOPIC_CHARS: usize = 200;

/// Maximum length of a durable patient memory statement, in characters.
pub const MAX_MEMORY_STATEMENT_CHARS: usize = 4000;

/// Maximum length of a single-line memory retirement reason, in characters.
pub const MAX_MEMORY_RETIREMENT_CHARS: usize = 500;

/// Maximum number of recent work items listed in one `retrieve_patient_work` call.
pub const MAX_RETRIEVED_WORK_ITEMS: usize = 25;

/// Reserved receipt actor namespace for chat memory mutations so tool command keys
/// never collide with client message submission keys.
const MEMORY_RECEIPT_ACTOR: &str = "coda:patient-memory";

/// Whether `name` is the patient source refresh tool.
pub fn is_refresh_tool(name: &str) -> bool {
    name == REFRESH_SOURCES_TOOL
}

/// Whether `name` is the durable patient memory tool.
pub fn is_memory_tool(name: &str) -> bool {
    name == PATIENT_MEMORY_TOOL
}

/// Whether `name` is the cross-conversation work retrieval tool.
pub fn is_retrieve_work_tool(name: &str) -> bool {
    name == RETRIEVE_PATIENT_WORK_TOOL
}

/// Distinct reason why patient or clinical evidence could not establish a fact.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordLimitKind {
    /// Expected or requested record is absent from the patient's retained chart
    /// observation (or was withdrawn).
    Missing,
    /// Record is present in the chart, but an attachment, part, or stored payload
    /// lacks readable text and cannot support claims about its unread content.
    Unreadable,
    /// Upstream patient source or clinical literature service failed or could not
    /// be reached, so current evidence from that source is unavailable.
    Unavailable,
}

impl RecordLimitKind {
    /// Canonical snake_case name exposed in structured tool receipts.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Unreadable => "unreadable",
            Self::Unavailable => "unavailable",
        }
    }
}

/// One named evidence limit returned by chart, refresh, or clinical library tools.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordLimit {
    /// Distinct limit category (`missing`, `unreadable`, or `unavailable`).
    pub kind: RecordLimitKind,
    /// Record candidate ID, source ID, or service name affected by the limit.
    pub subject: String,
    /// Physician-legible explanation of what could not be read or confirmed.
    pub detail: String,
}

/// Inspects one retained [`SourceRevision`] for unreadable attachments, empty parts,
/// or source-reported readability limits. Stated once so every chart read uses the
/// same rule.
pub fn inspect_record_readability(source: &SourceRevision) -> Option<RecordLimit> {
    if let Some(reason) = source
        .provenance
        .get("unreadable_reason")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|text| !text.is_empty())
    {
        return Some(RecordLimit {
            kind: RecordLimitKind::Unreadable,
            subject: source.candidate_id.clone(),
            detail: format!(
                "Record `{}` ({}) has unreadable content reported by `{}`: {reason}.",
                source.candidate_id, source.record_type, source.source_id
            ),
        });
    }

    if let Some(empty_part) = source
        .readable_parts
        .iter()
        .find(|part| part.text.trim().is_empty())
    {
        return Some(RecordLimit {
            kind: RecordLimitKind::Unreadable,
            subject: source.candidate_id.clone(),
            detail: format!(
                "Record `{}` ({}) includes part `{}` with no readable text.",
                source.candidate_id, source.record_type, empty_part.label
            ),
        });
    }

    let has_readable_parts = source
        .readable_parts
        .iter()
        .any(|part| !part.text.trim().is_empty());
    if !has_readable_parts
        && let Ok(parsed) = serde_json::from_str::<Value>(&source.content)
        && let Some(attachment_desc) = detect_unextracted_attachment(&parsed)
    {
        return Some(RecordLimit {
            kind: RecordLimitKind::Unreadable,
            subject: source.candidate_id.clone(),
            detail: format!(
                "Record `{}` ({}) carries an encoded or binary attachment ({attachment_desc}) without extracted readable text.",
                source.candidate_id, source.record_type
            ),
        });
    }

    None
}

fn detect_unextracted_attachment(value: &Value) -> Option<String> {
    let inspect_attachment = |attachment: &Value| -> Option<String> {
        let content_type = attachment
            .get("contentType")
            .or_else(|| attachment.get("content_type"))
            .and_then(Value::as_str)
            .map(str::trim)
            .unwrap_or("");
        let has_binary_data = attachment
            .get("data")
            .and_then(Value::as_str)
            .is_some_and(|data| !data.trim().is_empty());
        let non_text_mime = !content_type.is_empty()
            && !content_type.starts_with("text/")
            && content_type != "application/json"
            && content_type != "application/fhir+json";
        if non_text_mime {
            return Some(content_type.to_owned());
        }
        if has_binary_data {
            return Some("encoded binary payload".to_owned());
        }
        None
    };

    if let Some(items) = value.get("content").and_then(Value::as_array) {
        for item in items {
            if let Some(attachment) = item.get("attachment")
                && let Some(found) = inspect_attachment(attachment)
            {
                return Some(found);
            }
        }
    }
    if let Some(forms) = value.get("presentedForm").and_then(Value::as_array) {
        for attachment in forms {
            if let Some(found) = inspect_attachment(attachment) {
                return Some(found);
            }
        }
    }
    if let Some(attachment) = value.get("attachment")
        && let Some(found) = inspect_attachment(attachment)
    {
        return Some(found);
    }
    None
}

/// Validation failure on a patient memory identifier, field, or tool payload.
#[derive(Debug, thiserror::Error)]
pub enum MemoryInputError {
    /// Required identifier or scope field was blank or contained control characters.
    #[error("{0} must be non-empty without control characters")]
    InvalidField(&'static str),
    /// Memory identifier failed boundary validation.
    #[error("invalid memory identifier `{0}`: supply 1 to 256 trimmed visible characters")]
    InvalidIdentifier(String),
    /// Topic label was empty or exceeded [`MAX_MEMORY_TOPIC_CHARS`].
    #[error("memory topic must be a single line of 1 to {MAX_MEMORY_TOPIC_CHARS} characters")]
    InvalidTopic,
    /// Memory statement was empty or exceeded [`MAX_MEMORY_STATEMENT_CHARS`].
    #[error("memory statement must be 1 to {MAX_MEMORY_STATEMENT_CHARS} characters")]
    InvalidStatement,
    /// Retirement reason was empty, multiline, or exceeded [`MAX_MEMORY_RETIREMENT_CHARS`].
    #[error(
        "retirement reason must be a single non-empty line of at most {MAX_MEMORY_RETIREMENT_CHARS} characters"
    )]
    InvalidRetirementReason,
    /// Tool JSON arguments did not match the offered schema.
    #[error(
        "supply a valid operation (`read`, `record_correction`, or `retire`) and its required fields ({0})"
    )]
    MalformedArguments(#[source] serde_json::Error),
    /// Tool invocation named an unknown tool.
    #[error("unknown patient memory tool `{0}`")]
    UnknownTool(String),
}

/// Validated identifier of one durable memory entry within `(practice, workspace)`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PatientMemoryId(String);

impl PatientMemoryId {
    /// Parses a trimmed, non-empty identifier of 1 to 256 visible characters.
    pub fn new(value: impl Into<String>) -> Result<Self, MemoryInputError> {
        let value = value.into();
        let count = value.chars().count();
        if !(1..=256).contains(&count)
            || value.trim() != value
            || value.chars().any(char::is_control)
        {
            return Err(MemoryInputError::InvalidIdentifier(value));
        }
        Ok(Self(value))
    }

    /// Identifier string as stored.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PatientMemoryId {
    type Error = MemoryInputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PatientMemoryId> for String {
    fn from(id: PatientMemoryId) -> Self {
        id.0
    }
}

impl fmt::Display for PatientMemoryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn validate_non_empty(field: &'static str, value: &str) -> Result<String, MemoryInputError> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.chars().any(char::is_control) {
        return Err(MemoryInputError::InvalidField(field));
    }
    Ok(trimmed.to_owned())
}

fn validate_topic(topic: &str) -> Result<String, MemoryInputError> {
    if topic.contains('\n') || topic.contains('\r') {
        return Err(MemoryInputError::InvalidTopic);
    }
    let trimmed = topic.trim();
    let count = trimmed.chars().count();
    if !(1..=MAX_MEMORY_TOPIC_CHARS).contains(&count) || trimmed.chars().any(char::is_control) {
        return Err(MemoryInputError::InvalidTopic);
    }
    Ok(trimmed.to_owned())
}

fn validate_statement(statement: &str) -> Result<String, MemoryInputError> {
    let trimmed = statement.trim();
    let count = trimmed.chars().count();
    if !(1..=MAX_MEMORY_STATEMENT_CHARS).contains(&count)
        || trimmed
            .chars()
            .any(|ch| ch.is_control() && ch != '\n' && ch != '\t')
    {
        return Err(MemoryInputError::InvalidStatement);
    }
    Ok(trimmed.to_owned())
}

fn validate_retirement_reason(reason: &str) -> Result<String, MemoryInputError> {
    if reason.contains('\n') || reason.contains('\r') {
        return Err(MemoryInputError::InvalidRetirementReason);
    }
    let trimmed = reason.trim();
    let count = trimmed.chars().count();
    if !(1..=MAX_MEMORY_RETIREMENT_CHARS).contains(&count) || trimmed.chars().any(char::is_control)
    {
        return Err(MemoryInputError::InvalidRetirementReason);
    }
    Ok(trimmed.to_owned())
}

/// Origin of a durable patient memory entry, keeping physician-supplied facts
/// distinct from chart-derived clinical summaries.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "origin", rename_all = "snake_case")]
pub enum MemoryProvenance {
    /// Fact or correction supplied by a signed-in physician during chat, tied to
    /// the conversation and authored message.
    PhysicianCorrection {
        /// Signed-in physician who supplied or confirmed the correction.
        physician: String,
        /// Conversation identifier where the correction was given.
        chat_id: String,
        /// Accepted physician message identifier carrying the correction.
        message_id: String,
        /// Work identifier that recorded the correction.
        work_id: String,
        /// Attempt identifier that recorded the correction.
        attempt_id: String,
    },
    /// Clinical summary synthesized from retained chart records by a background job.
    ClinicalSummary {
        /// Practice background job identifier.
        job_id: String,
        /// Work identifier of the background job.
        work_id: String,
        /// Attempt identifier of the background job.
        attempt_id: String,
        /// Cited exact chart record revisions supporting the summary.
        records: Vec<ExactRecordReference>,
    },
}

impl MemoryProvenance {
    fn validate(&self) -> Result<Self, MemoryInputError> {
        match self {
            Self::PhysicianCorrection {
                physician,
                chat_id,
                message_id,
                work_id,
                attempt_id,
            } => Ok(Self::PhysicianCorrection {
                physician: validate_non_empty("physician", physician)?,
                chat_id: validate_non_empty("chat_id", chat_id)?,
                message_id: validate_non_empty("message_id", message_id)?,
                work_id: validate_non_empty("work_id", work_id)?,
                attempt_id: validate_non_empty("attempt_id", attempt_id)?,
            }),
            Self::ClinicalSummary {
                job_id,
                work_id,
                attempt_id,
                records,
            } => Ok(Self::ClinicalSummary {
                job_id: validate_non_empty("job_id", job_id)?,
                work_id: validate_non_empty("work_id", work_id)?,
                attempt_id: validate_non_empty("attempt_id", attempt_id)?,
                records: records.clone(),
            }),
        }
    }
}

/// Lifecycle standing of a durable patient memory entry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum MemoryStanding {
    /// Active in patient starting context and memory reads.
    Active,
    /// Retired when superseded or withdrawn, kept for audit and readback.
    Retired {
        /// Single-line explanation of why this memory entry was retired.
        reason: String,
        /// Provenance of the turn or job that retired the entry.
        retired_by: MemoryProvenance,
        /// Database-clock Unix seconds when retired.
        retired_at: i64,
    },
}

/// One durable patient memory entry for `(practice, workspace)`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatientMemoryEntry {
    /// Practice tenant owning the patient link.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Stable memory entry identifier within `(practice, workspace)`.
    pub id: PatientMemoryId,
    /// Short clinical topic label.
    pub topic: String,
    /// Durable clinical statement or physician correction.
    pub statement: String,
    /// Provenance distinguishing physician corrections from chart summaries.
    pub provenance: MemoryProvenance,
    /// Monotonic revision (1 when created; incremented on update or retirement).
    pub revision: u64,
    /// Database-clock Unix seconds when first recorded.
    pub created_at: i64,
    /// Database-clock Unix seconds when last updated.
    pub updated_at: i64,
    /// Current lifecycle standing (`Active` or `Retired`).
    pub standing: MemoryStanding,
}

/// Which memory entries a read selects.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemorySelection {
    /// Only active entries carried into current clinical reasoning.
    ActiveOnly,
    /// Active and retired entries alike.
    All,
}

/// Typed input for recording or updating a durable patient memory entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordMemoryCorrection {
    /// Practice tenant.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Stable memory entry identifier.
    pub id: PatientMemoryId,
    /// Clinical topic label.
    pub topic: String,
    /// Durable statement to carry forward across sessions.
    pub statement: String,
    /// Provenance of the correction or summary.
    pub provenance: MemoryProvenance,
}

/// Typed input for retiring an active patient memory entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetirePatientMemory {
    /// Practice tenant.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Target memory entry identifier.
    pub id: PatientMemoryId,
    /// Single-line retirement reason.
    pub reason: String,
    /// Provenance of the retiring turn or job.
    pub retired_by: MemoryProvenance,
}

/// Outcome of a once-only patient memory mutation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MemoryMutation {
    /// Mutation committed to Postgres.
    Applied(PatientMemoryEntry),
    /// Exact command key already committed earlier; existing state returned.
    Duplicate(PatientMemoryEntry),
}

impl MemoryMutation {
    /// Durable entry after the mutation.
    pub const fn entry(&self) -> &PatientMemoryEntry {
        match self {
            Self::Applied(entry) | Self::Duplicate(entry) => entry,
        }
    }
}

/// Failure on a durable patient memory read or mutation.
#[derive(Debug, thiserror::Error)]
pub enum PatientMemoryError {
    /// Input failed validation before mutating state.
    #[error("{0}")]
    InvalidInput(#[from] MemoryInputError),
    /// Command key was already used with a different payload.
    #[error("command key `{key}` already recorded a different memory command")]
    IdempotencyCollision {
        /// Colliding command key.
        key: String,
    },
    /// Retiring named an entry absent from the workspace.
    #[error("memory entry `{id}` does not exist in this patient workspace")]
    NotFound {
        /// Missing memory identifier.
        id: PatientMemoryId,
    },
    /// Retiring targeted an entry that is already retired.
    #[error("memory entry `{id}` is already retired")]
    AlreadyRetired {
        /// Retired memory identifier.
        id: PatientMemoryId,
    },
    /// Backing memory operation failed without saving changes.
    #[error("patient memory operation failed: {reason}")]
    Unavailable {
        /// Operational failure reason.
        reason: String,
    },
    /// Database error from SeaORM.
    #[error("{0}")]
    Database(#[from] DbErr),
}

/// Pluggable owner for patient memory reads and mutations so chat and background
/// understanding share one boundary.
#[async_trait]
pub trait PatientMemoryOperation: Send + Sync {
    /// Lists durable memory entries for `(practice, workspace)`.
    async fn list_memories(
        &self,
        store: &Store,
        practice: &str,
        workspace: &str,
        selection: MemorySelection,
    ) -> Result<Vec<PatientMemoryEntry>, PatientMemoryError>;

    /// Records or updates one durable patient memory entry under `command_key`.
    async fn record_correction(
        &self,
        store: &Store,
        command_key: &str,
        input: &RecordMemoryCorrection,
    ) -> Result<MemoryMutation, PatientMemoryError>;

    /// Retires one active patient memory entry under `command_key`.
    async fn retire_memory(
        &self,
        store: &Store,
        command_key: &str,
        input: &RetirePatientMemory,
    ) -> Result<MemoryMutation, PatientMemoryError>;

    /// Reads back an already-committed memory mutation receipt read-only when an
    /// execution attempt was interrupted before acknowledging its tool outcome.
    async fn saved_mutation(
        &self,
        _store: &Store,
        _practice: &str,
        _workspace: &str,
        _command_key: &str,
        _id: &PatientMemoryId,
    ) -> Result<Option<PatientMemoryEntry>, PatientMemoryError> {
        Ok(None)
    }
}

/// Durable SeaORM-backed [`PatientMemoryOperation`] storing entries in
/// `physician_runtime_patient_memory`.
#[derive(Clone, Copy, Debug, Default)]
pub struct PostgresPatientMemoryOperation;

fn decode_memory_entry(row: patient_memory::Model) -> Result<PatientMemoryEntry, DbErr> {
    let id = PatientMemoryId::new(row.id)
        .map_err(|cause| error(&format!("corrupt memory id: {cause}")))?;
    let provenance: MemoryProvenance = serde_json::from_value(row.provenance).map_err(
        conversion_error("patient_memory.provenance", "MemoryProvenance"),
    )?;
    let revision =
        u64::try_from(row.revision).map_err(conversion_error("patient_memory.revision", "u64"))?;
    let standing = match row.status.as_str() {
        "active" => MemoryStanding::Active,
        "retired" => {
            let reason = row
                .retirement_reason
                .ok_or_else(|| error("corrupt retired memory missing reason"))?;
            let retired_by_raw = row
                .retired_by
                .ok_or_else(|| error("corrupt retired memory missing provenance"))?;
            let retired_by: MemoryProvenance = serde_json::from_value(retired_by_raw).map_err(
                conversion_error("patient_memory.retired_by", "MemoryProvenance"),
            )?;
            let retired_at = row
                .retired_at
                .ok_or_else(|| error("corrupt retired memory missing retired_at"))?;
            MemoryStanding::Retired {
                reason,
                retired_by,
                retired_at,
            }
        }
        _ => return Err(error("corrupt patient memory status")),
    };
    Ok(PatientMemoryEntry {
        practice: row.practice,
        workspace: row.workspace,
        id,
        topic: row.topic,
        statement: row.statement,
        provenance,
        revision,
        created_at: row.created_at,
        updated_at: row.updated_at,
        standing,
    })
}

async fn load_memory_entry(
    tx: &impl ConnectionTrait,
    practice: &str,
    workspace: &str,
    id: &PatientMemoryId,
) -> Result<Option<PatientMemoryEntry>, DbErr> {
    patient_memory::Entity::find_by_id((
        practice.to_owned(),
        workspace.to_owned(),
        id.as_str().to_owned(),
    ))
    .one(tx)
    .await?
    .map(decode_memory_entry)
    .transpose()
}

#[async_trait]
impl PatientMemoryOperation for PostgresPatientMemoryOperation {
    async fn list_memories(
        &self,
        store: &Store,
        practice: &str,
        workspace: &str,
        selection: MemorySelection,
    ) -> Result<Vec<PatientMemoryEntry>, PatientMemoryError> {
        let practice = validate_non_empty("practice", practice)?;
        let workspace = validate_non_empty("workspace", workspace)?;
        let mut query = patient_memory::Entity::find()
            .filter(patient_memory::Column::Practice.eq(practice))
            .filter(patient_memory::Column::Workspace.eq(workspace));
        match selection {
            MemorySelection::ActiveOnly => {
                query = query.filter(patient_memory::Column::Status.eq("active"));
            }
            MemorySelection::All => {}
        }
        let rows = query
            .order_by_desc(patient_memory::Column::UpdatedAt)
            .order_by_asc(patient_memory::Column::Id)
            .all(&store.db)
            .await?;
        let mut entries = Vec::with_capacity(rows.len());
        for row in rows {
            entries.push(decode_memory_entry(row)?);
        }
        Ok(entries)
    }

    async fn record_correction(
        &self,
        store: &Store,
        command_key: &str,
        input: &RecordMemoryCorrection,
    ) -> Result<MemoryMutation, PatientMemoryError> {
        let key = validate_non_empty("command_key", command_key)?;
        let practice = validate_non_empty("practice", &input.practice)?;
        let workspace = validate_non_empty("workspace", &input.workspace)?;
        let topic = validate_topic(&input.topic)?;
        let statement = validate_statement(&input.statement)?;
        let provenance = input.provenance.validate()?;

        let command = json!({
            "kind": "memory_record",
            "practice": &practice,
            "workspace": &workspace,
            "memory_id": input.id.as_str(),
            "topic": &topic,
            "statement": &statement,
            "provenance": &provenance,
        });

        let tx = store.db.begin().await?;
        match receipt::record(&tx, &practice, MEMORY_RECEIPT_ACTOR, &key, &command).await? {
            receipt::Receipt::New => {}
            receipt::Receipt::Same => {
                let existing = load_memory_entry(&tx, &practice, &workspace, &input.id)
                    .await?
                    .ok_or_else(|| error("missing memory entry for accepted receipt"))?;
                tx.commit().await?;
                return Ok(MemoryMutation::Duplicate(existing));
            }
            receipt::Receipt::Different => {
                return Err(PatientMemoryError::IdempotencyCollision { key });
            }
        }

        let existing = patient_memory::Entity::find_by_id((
            practice.clone(),
            workspace.clone(),
            input.id.as_str().to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?;
        let timestamp = now(&tx).await?;
        let provenance_json = serde_json::to_value(&provenance)
            .map_err(conversion_error("MemoryProvenance", "json"))?;

        let (revision, created_at) = match existing {
            None => {
                patient_memory::ActiveModel {
                    practice: Set(practice.clone()),
                    workspace: Set(workspace.clone()),
                    id: Set(input.id.as_str().to_owned()),
                    status: Set("active".into()),
                    topic: Set(topic.clone()),
                    statement: Set(statement.clone()),
                    provenance: Set(provenance_json),
                    retirement_reason: Set(None),
                    retired_by: Set(None),
                    retired_at: Set(None),
                    created_at: Set(timestamp),
                    updated_at: Set(timestamp),
                    revision: Set(1),
                }
                .insert(&tx)
                .await?;
                (1_u64, timestamp)
            }
            Some(row) => {
                let current_rev = u64::try_from(row.revision)
                    .map_err(conversion_error("patient_memory.revision", "u64"))?;
                let next_rev = current_rev + 1;
                let next_rev_i64 = i64::try_from(next_rev)
                    .map_err(conversion_error("patient_memory.revision", "i64"))?;
                let created = row.created_at;
                patient_memory::ActiveModel {
                    practice: Unchanged(row.practice),
                    workspace: Unchanged(row.workspace),
                    id: Unchanged(row.id),
                    status: Set("active".into()),
                    topic: Set(topic.clone()),
                    statement: Set(statement.clone()),
                    provenance: Set(provenance_json),
                    retirement_reason: Set(None),
                    retired_by: Set(None),
                    retired_at: Set(None),
                    created_at: Unchanged(created),
                    updated_at: Set(timestamp),
                    revision: Set(next_rev_i64),
                }
                .update(&tx)
                .await?;
                (next_rev, created)
            }
        };

        tx.commit().await?;
        Ok(MemoryMutation::Applied(PatientMemoryEntry {
            practice,
            workspace,
            id: input.id.clone(),
            topic,
            statement,
            provenance,
            revision,
            created_at,
            updated_at: timestamp,
            standing: MemoryStanding::Active,
        }))
    }

    async fn retire_memory(
        &self,
        store: &Store,
        command_key: &str,
        input: &RetirePatientMemory,
    ) -> Result<MemoryMutation, PatientMemoryError> {
        let key = validate_non_empty("command_key", command_key)?;
        let practice = validate_non_empty("practice", &input.practice)?;
        let workspace = validate_non_empty("workspace", &input.workspace)?;
        let reason = validate_retirement_reason(&input.reason)?;
        let retired_by = input.retired_by.validate()?;

        let command = json!({
            "kind": "memory_retire",
            "practice": &practice,
            "workspace": &workspace,
            "memory_id": input.id.as_str(),
            "reason": &reason,
            "retired_by": &retired_by,
        });

        let tx = store.db.begin().await?;
        match receipt::record(&tx, &practice, MEMORY_RECEIPT_ACTOR, &key, &command).await? {
            receipt::Receipt::New => {}
            receipt::Receipt::Same => {
                let existing = load_memory_entry(&tx, &practice, &workspace, &input.id)
                    .await?
                    .ok_or_else(|| error("missing memory entry for accepted receipt"))?;
                tx.commit().await?;
                return Ok(MemoryMutation::Duplicate(existing));
            }
            receipt::Receipt::Different => {
                return Err(PatientMemoryError::IdempotencyCollision { key });
            }
        }

        let Some(row) = patient_memory::Entity::find_by_id((
            practice.clone(),
            workspace.clone(),
            input.id.as_str().to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        else {
            return Err(PatientMemoryError::NotFound {
                id: input.id.clone(),
            });
        };
        let current = decode_memory_entry(row.clone())?;
        if matches!(current.standing, MemoryStanding::Retired { .. }) {
            return Err(PatientMemoryError::AlreadyRetired {
                id: input.id.clone(),
            });
        }

        let timestamp = now(&tx).await?;
        let next_rev = current.revision + 1;
        let next_rev_i64 =
            i64::try_from(next_rev).map_err(conversion_error("patient_memory.revision", "i64"))?;
        let retired_by_json = serde_json::to_value(&retired_by)
            .map_err(conversion_error("MemoryProvenance", "json"))?;

        patient_memory::ActiveModel {
            practice: Unchanged(row.practice),
            workspace: Unchanged(row.workspace),
            id: Unchanged(row.id),
            status: Set("retired".into()),
            topic: Unchanged(row.topic),
            statement: Unchanged(row.statement),
            provenance: Unchanged(row.provenance),
            retirement_reason: Set(Some(reason.clone())),
            retired_by: Set(Some(retired_by_json)),
            retired_at: Set(Some(timestamp)),
            created_at: Unchanged(row.created_at),
            updated_at: Set(timestamp),
            revision: Set(next_rev_i64),
        }
        .update(&tx)
        .await?;
        tx.commit().await?;

        Ok(MemoryMutation::Applied(PatientMemoryEntry {
            standing: MemoryStanding::Retired {
                reason,
                retired_by,
                retired_at: timestamp,
            },
            revision: next_rev,
            updated_at: timestamp,
            ..current
        }))
    }

    async fn saved_mutation(
        &self,
        store: &Store,
        practice: &str,
        workspace: &str,
        command_key: &str,
        id: &PatientMemoryId,
    ) -> Result<Option<PatientMemoryEntry>, PatientMemoryError> {
        let key = validate_non_empty("command_key", command_key)?;
        let practice = validate_non_empty("practice", practice)?;
        let workspace = validate_non_empty("workspace", workspace)?;
        let receipt_row =
            command::Entity::find_by_id((practice.clone(), MEMORY_RECEIPT_ACTOR.to_owned(), key))
                .one(&store.db)
                .await?;
        if receipt_row.is_none() {
            return Ok(None);
        }
        let existing = load_memory_entry(&store.db, &practice, &workspace, id)
            .await?
            .ok_or_else(|| error("missing memory entry for accepted receipt"))?;
        Ok(Some(existing))
    }
}

/// Summary of one refreshed source returned by [`PatientRefreshOperation`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefreshedSourceSummary {
    /// Stable source identifier.
    pub source_id: String,
    /// Physician-facing source label.
    pub source_label: String,
    /// Number of retained records added, corrected, or withdrawn by the pull.
    pub changed_records: usize,
    /// Unix second recorded for the completed pull.
    pub pulled_at: i64,
}

/// Report returned when a patient source refresh succeeds.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRefreshReport {
    /// Workspace source version after applying the refresh.
    pub workspace_version: u64,
    /// Sources refreshed during this call.
    pub sources: Vec<RefreshedSourceSummary>,
    /// Any partial or per-source unavailability limits encountered during a mixed refresh.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limits: Vec<RecordLimit>,
}

/// Failure while refreshing patient sources on demand.
#[derive(Debug, thiserror::Error)]
pub enum SourceRefreshError {
    /// Refresh request arguments were malformed or empty.
    #[error("{0}")]
    InvalidInput(String),
    /// No live source adapter is configured or reachable for the requested source.
    #[error("source `{subject}` is unavailable for live refresh: {reason}")]
    Unavailable {
        /// Source identifier or workspace affected.
        subject: String,
        /// Why live refresh could not run.
        reason: String,
    },
    /// Upstream source pull failed; retained chart and pull timestamps remain untouched.
    #[error("refresh from source `{source_id}` failed: {reason}")]
    Upstream {
        /// Failing source identifier.
        source_id: String,
        /// Upstream error detail.
        reason: String,
    },
    /// Source check was interrupted mid-pull before settling every requested source.
    #[error("{0}")]
    Interrupted(#[from] super::refresh::SourceRefreshAdapterError),
    /// Database failure while reading or committing refreshed observations.
    #[error("{0}")]
    Database(#[from] DbErr),
}

/// Read-only crash reconciliation status for a work-scoped chat source refresh.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SavedChatRefresh {
    /// No complete command receipt has been recorded for this work and target yet.
    NotYetSaved,
    /// Every requested source settled and at least one source succeeded or partially succeeded.
    Refreshed(SourceRefreshReport),
    /// Every requested source settled with a source-level failure outcome.
    Failed {
        /// First failing source identifier.
        source_id: String,
        /// Recorded failure detail.
        reason: String,
    },
}

/// Pluggable owner for on-demand patient source refresh in physician chat.
#[async_trait]
pub trait PatientRefreshOperation: Send + Sync {
    /// Pulls fresh records for `scope.workspace` (restricted to `source_id` when supplied)
    /// and commits them atomically. Never advances pull timestamps on failure.
    async fn refresh_sources(
        &self,
        store: &Store,
        scope: &Scope,
        source_id: Option<&str>,
        reason: &str,
    ) -> Result<SourceRefreshReport, SourceRefreshError>;

    /// Executes an on-demand chat source refresh scoped to `work_id` so retries
    /// within the same work item reuse already-settled per-source receipts.
    async fn refresh_sources_for_work(
        &self,
        store: &Store,
        scope: &Scope,
        _work_id: &str,
        source_id: Option<&str>,
        reason: &str,
    ) -> Result<SourceRefreshReport, SourceRefreshError> {
        self.refresh_sources(store, scope, source_id, reason).await
    }

    /// Reconciles a previously completed refresh for `work_id` without invoking
    /// any live source adapter.
    async fn saved_refresh_for_work(
        &self,
        _store: &Store,
        _scope: &Scope,
        _work_id: &str,
        _source_id: Option<&str>,
        _reason: &str,
    ) -> Result<SavedChatRefresh, SourceRefreshError> {
        Ok(SavedChatRefresh::NotYetSaved)
    }
}

/// Adapter-backed [`PatientRefreshOperation`] that pulls from configured
/// [`PatientSource`] implementations and commits through [`Store::record_chart`].
#[derive(Clone, Default)]
pub struct SourceAdapterRefreshOperation {
    adapters: Vec<Arc<dyn PatientSource>>,
}

impl SourceAdapterRefreshOperation {
    /// Creates a refresh operation backed by the supplied [`PatientSource`] adapters.
    pub fn new(adapters: Vec<Arc<dyn PatientSource>>) -> Self {
        Self { adapters }
    }

    async fn resolve_patient(
        adapter: &dyn PatientSource,
        store: &Store,
        workspace: &str,
    ) -> Result<SourcePatient, SourceRefreshError> {
        let pulls = store
            .source_pulls(&[workspace.to_owned()])
            .await?
            .remove(workspace)
            .unwrap_or_default();
        if let Some(existing) = pulls
            .into_iter()
            .find(|pull| pull.source_id == adapter.source_id())
        {
            return adapter
                .patient(&existing.patient.source_patient_id)
                .await
                .map_err(|reason| SourceRefreshError::Upstream {
                    source_id: adapter.source_id().to_owned(),
                    reason: reason.to_string(),
                });
        }

        let ids = adapter
            .patient_ids()
            .await
            .map_err(|reason| SourceRefreshError::Upstream {
                source_id: adapter.source_id().to_owned(),
                reason: reason.to_string(),
            })?;
        for pid in ids {
            if crate::physician_sources::workspace_id(adapter.source_id(), &pid) == workspace {
                return adapter.patient(&pid).await.map_err(|reason| {
                    SourceRefreshError::Upstream {
                        source_id: adapter.source_id().to_owned(),
                        reason: reason.to_string(),
                    }
                });
            }
        }
        Err(SourceRefreshError::Unavailable {
            subject: adapter.source_id().to_owned(),
            reason: format!(
                "source `{}` holds no patient mapped to workspace `{workspace}`",
                adapter.source_id()
            ),
        })
    }
}

#[async_trait]
impl PatientRefreshOperation for SourceAdapterRefreshOperation {
    async fn refresh_sources(
        &self,
        store: &Store,
        scope: &Scope,
        source_id: Option<&str>,
        reason: &str,
    ) -> Result<SourceRefreshReport, SourceRefreshError> {
        if reason.trim().is_empty() {
            return Err(SourceRefreshError::InvalidInput(
                "Supply a non-empty clinical reason for refreshing patient sources.".into(),
            ));
        }
        let selected: Vec<&Arc<dyn PatientSource>> =
            match source_id.map(str::trim).filter(|id| !id.is_empty()) {
                Some(target) => {
                    let matching: Vec<_> = self
                        .adapters
                        .iter()
                        .filter(|adapter| adapter.source_id() == target)
                        .collect();
                    if matching.is_empty() {
                        return Err(SourceRefreshError::Unavailable {
                            subject: target.to_owned(),
                            reason: "no live adapter is configured for the requested source".into(),
                        });
                    }
                    matching
                }
                None => {
                    if self.adapters.is_empty() {
                        return Err(SourceRefreshError::Unavailable {
                            subject: scope.workspace.clone(),
                            reason:
                                "no live patient source adapters are configured for this workspace"
                                    .into(),
                        });
                    }
                    self.adapters.iter().collect()
                }
            };

        let mut refreshed = Vec::with_capacity(selected.len());
        for adapter in selected {
            let patient = Self::resolve_patient(adapter.as_ref(), store, &scope.workspace).await?;
            let records = adapter
                .records(&patient, &scope.workspace)
                .await
                .map_err(|cause| SourceRefreshError::Upstream {
                    source_id: adapter.source_id().to_owned(),
                    reason: cause.to_string(),
                })?;
            let pulled_at = now(&store.db).await?;
            let pull = SourcePull {
                source_id: adapter.source_id().to_owned(),
                source_label: adapter.source_label().to_owned(),
                patient,
                pulled_at: Some(pulled_at),
            };
            let changed_records = store
                .record_chart(&scope.workspace, &records, &pull)
                .await?;
            refreshed.push(RefreshedSourceSummary {
                source_id: pull.source_id,
                source_label: pull.source_label,
                changed_records,
                pulled_at,
            });
        }
        let observation = store.sources(&scope.workspace).await?;
        if refreshed.iter().any(|summary| summary.changed_records > 0)
            && let Some(binding) = &store.understanding_trigger
            && let Err(super::understanding::UnderstandingError::Database(db_err)) = store
                .trigger_patient_understanding_authorized(
                    &scope.practice,
                    &scope.workspace,
                    &binding.roles,
                    &binding.skills,
                )
                .await
        {
            return Err(SourceRefreshError::Database(db_err));
        }
        Ok(SourceRefreshReport {
            workspace_version: observation.version,
            sources: refreshed,
            limits: Vec::new(),
        })
    }
}

/// Default [`PatientRefreshOperation`] delegating on-demand chat source refreshes
/// directly to [`super::refresh`] so physician chat and background jobs share one
/// once-only status and incremental evidence delivery owner.
#[derive(Clone, Copy, Debug, Default)]
pub struct RuntimeSourceRefreshOperation;

impl RuntimeSourceRefreshOperation {
    fn chat_refresh_request(
        work_id: &str,
        source_id: Option<&str>,
        reason: &str,
    ) -> Result<super::refresh::RefreshRequest, SourceRefreshError> {
        if reason.trim().is_empty() {
            return Err(SourceRefreshError::InvalidInput(
                "Supply a non-empty clinical reason for refreshing patient sources.".into(),
            ));
        }
        let target = match source_id.map(str::trim).filter(|id| !id.is_empty()) {
            None => super::refresh::RefreshTarget::AllSources,
            Some(raw_id) => {
                let parsed = super::refresh::SourceId::new(raw_id)
                    .map_err(|cause| SourceRefreshError::InvalidInput(cause.to_string()))?;
                super::refresh::RefreshTarget::selected([parsed])
                    .map_err(|cause| SourceRefreshError::InvalidInput(cause.to_string()))?
            }
        };
        super::refresh::RefreshToolCall {
            target,
            request_key: None,
        }
        .into_request(work_id)
        .map_err(|cause| SourceRefreshError::InvalidInput(cause.to_string()))
    }

    fn classify_refresh_report(
        report: super::refresh::RefreshReport,
    ) -> Result<SourceRefreshReport, SourceRefreshError> {
        let mut refreshed = Vec::new();
        let mut limits = Vec::new();
        let mut first_failed: Option<(String, String)> = None;
        for saved in report.sources {
            match saved.outcome {
                super::refresh::SourceRefreshOutcome::Updated {
                    changed_records,
                    pulled_at,
                    ..
                } => {
                    refreshed.push(RefreshedSourceSummary {
                        source_id: saved.source_id.as_str().to_owned(),
                        source_label: saved.source_label,
                        changed_records,
                        pulled_at,
                    });
                }
                super::refresh::SourceRefreshOutcome::PartlyUpdated {
                    changed_records,
                    pulled_at,
                    reason: partial_reason,
                    ..
                } => {
                    limits.push(RecordLimit {
                        kind: RecordLimitKind::Unavailable,
                        subject: saved.source_id.as_str().to_owned(),
                        detail: format!(
                            "Source `{}` updated partially ({changed_records} changed records): {partial_reason}",
                            saved.source_label
                        ),
                    });
                    refreshed.push(RefreshedSourceSummary {
                        source_id: saved.source_id.as_str().to_owned(),
                        source_label: saved.source_label,
                        changed_records,
                        pulled_at,
                    });
                }
                super::refresh::SourceRefreshOutcome::Failed {
                    reason: fail_reason,
                    ..
                } => {
                    let sid = saved.source_id.as_str().to_owned();
                    if first_failed.is_none() {
                        first_failed = Some((sid.clone(), fail_reason.clone()));
                    }
                    limits.push(RecordLimit {
                        kind: RecordLimitKind::Unavailable,
                        subject: sid,
                        detail: fail_reason,
                    });
                }
            }
        }

        if refreshed.is_empty()
            && let Some((source_id, fail_reason)) = first_failed
        {
            return Err(SourceRefreshError::Upstream {
                source_id,
                reason: fail_reason,
            });
        }

        Ok(SourceRefreshReport {
            workspace_version: report.workspace_version,
            sources: refreshed,
            limits,
        })
    }
}

#[async_trait]
impl PatientRefreshOperation for RuntimeSourceRefreshOperation {
    async fn refresh_sources(
        &self,
        store: &Store,
        scope: &Scope,
        source_id: Option<&str>,
        reason: &str,
    ) -> Result<SourceRefreshReport, SourceRefreshError> {
        let standalone_work = format!("chat:{}:{}", scope.chat.trim(), uuid::Uuid::new_v4());
        self.refresh_sources_for_work(store, scope, &standalone_work, source_id, reason)
            .await
    }

    async fn refresh_sources_for_work(
        &self,
        store: &Store,
        scope: &Scope,
        work_id: &str,
        source_id: Option<&str>,
        reason: &str,
    ) -> Result<SourceRefreshReport, SourceRefreshError> {
        let request = Self::chat_refresh_request(work_id, source_id, reason)?;

        let report = match store
            .refresh_sources_for_claimed_chat(scope, &request)
            .await
        {
            Ok(report) => report,
            Err(super::refresh::RefreshError::InvalidInput(cause)) => {
                return Err(SourceRefreshError::InvalidInput(cause.to_string()));
            }
            Err(super::refresh::RefreshError::NoConfiguredSources { workspace }) => {
                return Err(SourceRefreshError::Unavailable {
                    subject: workspace,
                    reason: "no live patient source adapters are configured for this workspace"
                        .into(),
                });
            }
            Err(super::refresh::RefreshError::UnknownSource {
                source_id,
                configured,
                ..
            }) => {
                return Err(SourceRefreshError::Unavailable {
                    subject: source_id.as_str().to_owned(),
                    reason: format!(
                        "no live adapter is configured for the requested source (configured: {configured})"
                    ),
                });
            }
            Err(super::refresh::RefreshError::Interrupted(cause)) => {
                return Err(SourceRefreshError::Interrupted(cause));
            }
            Err(super::refresh::RefreshError::IdempotencyCollision { key }) => {
                return Err(SourceRefreshError::InvalidInput(format!(
                    "duplicate refresh command key `{key}`"
                )));
            }
            Err(super::refresh::RefreshError::PracticeAccess(cause)) => {
                return Err(SourceRefreshError::Unavailable {
                    subject: scope.workspace.clone(),
                    reason: cause.to_string(),
                });
            }
            Err(super::refresh::RefreshError::PhysicianAccess(db_err))
            | Err(super::refresh::RefreshError::Database(db_err)) => {
                return Err(SourceRefreshError::Database(db_err));
            }
            Err(super::refresh::RefreshError::Encoding(cause)) => {
                return Err(SourceRefreshError::InvalidInput(cause.to_string()));
            }
        };

        Self::classify_refresh_report(report)
    }

    async fn saved_refresh_for_work(
        &self,
        store: &Store,
        scope: &Scope,
        work_id: &str,
        source_id: Option<&str>,
        reason: &str,
    ) -> Result<SavedChatRefresh, SourceRefreshError> {
        let request = Self::chat_refresh_request(work_id, source_id, reason)?;
        let saved = match store.saved_physician_refresh_report(scope, &request).await {
            Ok(None) => return Ok(SavedChatRefresh::NotYetSaved),
            Ok(Some(report)) => report,
            Err(super::refresh::RefreshError::InvalidInput(cause)) => {
                return Err(SourceRefreshError::InvalidInput(cause.to_string()));
            }
            Err(super::refresh::RefreshError::IdempotencyCollision { key }) => {
                return Err(SourceRefreshError::InvalidInput(format!(
                    "duplicate refresh command key `{key}`"
                )));
            }
            Err(super::refresh::RefreshError::PhysicianAccess(db_err))
            | Err(super::refresh::RefreshError::Database(db_err)) => {
                return Err(SourceRefreshError::Database(db_err));
            }
            Err(other) => {
                return Err(SourceRefreshError::InvalidInput(other.to_string()));
            }
        };
        match Self::classify_refresh_report(saved) {
            Ok(report) => Ok(SavedChatRefresh::Refreshed(report)),
            Err(SourceRefreshError::Upstream { source_id, reason }) => {
                Ok(SavedChatRefresh::Failed { source_id, reason })
            }
            Err(other) => Err(other),
        }
    }
}

/// Per-source freshness summary included in [`PatientStartingContext`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceFreshnessSummary {
    /// Stable source identifier.
    pub source_id: String,
    /// Physician-facing source name.
    pub source_label: String,
    /// Unix second when the source's last successful pull finished.
    pub pulled_at: i64,
}

/// Standing of the practice dispatcher's source check for this patient workspace.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum DispatchCheckStanding {
    /// No background source check has been recorded yet.
    NotRecorded,
    /// Most recent source check succeeded at `checked_at`.
    Current {
        /// Unix second of the latest successful source check.
        checked_at: i64,
    },
    /// A source check failed after the last successful check (or before any succeeded).
    FailedSinceLastSuccess {
        /// Unix second of the failed check attempt.
        last_attempt_at: i64,
        /// Unix second of the prior successful check, if one ever succeeded.
        last_success_at: Option<i64>,
    },
}

/// Concise open concern projection included in [`PatientStartingContext`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenConcernSummary {
    /// Concern identifier.
    pub concern_id: String,
    /// `now` or `today`.
    pub urgency: ConcernUrgency,
    /// Clinical statement.
    pub reason: String,
    /// Cited chart record references.
    pub records: Vec<ExactRecordReference>,
}

/// Kind of saved earlier work available in the patient workspace.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PriorWorkKind {
    /// Earlier physician chat conversation in this workspace.
    Conversation {
        /// Conversation identifier.
        chat_id: String,
    },
    /// Completed practice background job for this workspace.
    BackgroundJob {
        /// Practice job identifier.
        job_id: String,
        /// Background job kind.
        job_kind: PracticeJobKind,
    },
    /// Saved patient-message reply draft in this workspace.
    ReplyDraft {
        /// Reply work identifier.
        reply_id: String,
    },
}

/// Pointer to one earlier conversation or saved work item in the patient workspace.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PriorWorkPointer {
    /// Target work kind and identifier.
    pub target: PriorWorkKind,
    /// Content-distinguishing title or question.
    pub title: String,
    /// Short outcome label.
    pub outcome: String,
    /// Unix second when last updated.
    pub updated_at: i64,
}

/// Standing of durable patient memory in [`PatientStartingContext`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum PatientMemoryStanding {
    /// Durable patient memory read succeeded.
    Available {
        /// Active durable memory entries (physician corrections and clinical summaries).
        entries: Vec<PatientMemoryEntry>,
    },
    /// Durable patient memory could not be read because the memory operation was unavailable.
    Unavailable {
        /// Operational reason why durable patient memory could not be read.
        reason: String,
    },
}

/// Starting clinical context assembled for one patient workspace when a chat turn
/// begins or opens a fresh window.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatientStartingContext {
    /// Target patient workspace identifier.
    pub workspace: String,
    /// Patient demographic identity from the latest recorded source pull, when present.
    pub patient: Option<SourcePatient>,
    /// Current workspace chart version.
    pub chart_version: u64,
    /// Count of active retained chart records in the workspace.
    pub chart_record_count: usize,
    /// Per-source successful pull timestamps.
    pub source_pulls: Vec<SourceFreshnessSummary>,
    /// Standing of background source checks for this workspace.
    pub check_standing: DispatchCheckStanding,
    /// Saved patient understanding (summary and Coda's private operating index), when initialized.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub understanding: Option<super::understanding::PatientUnderstanding>,
    /// Standing of durable patient memory for this workspace.
    pub memory_standing: PatientMemoryStanding,
    /// Open patient concerns on Today.
    pub open_concerns: Vec<OpenConcernSummary>,
    /// Recent earlier conversations and work in this workspace (excluding the active chat).
    pub recent_work: Vec<PriorWorkPointer>,
}

impl PatientStartingContext {
    /// Renders a compact, model-facing developer context summary with zero
    /// implementation labels or ticket identifiers.
    pub fn developer_text(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("Patient workspace context ({})", self.workspace));
        match &self.patient {
            Some(patient) => {
                let dob = patient.birth_date.as_deref().unwrap_or("not recorded");
                let mrns = if patient.medical_record_numbers.is_empty() {
                    "not recorded".to_owned()
                } else {
                    patient
                        .medical_record_numbers
                        .iter()
                        .map(|mrn| format!("{}: {}", mrn.system, mrn.value))
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                lines.push(format!(
                    "Patient identity: {} | birth date: {} | MRN: {}",
                    patient.display_name, dob, mrns
                ));
            }
            None => {
                lines.push(
                    "Patient identity: no source demographic pull recorded yet for this workspace."
                        .to_owned(),
                );
            }
        }

        let pulls_summary = if self.source_pulls.is_empty() {
            "no completed source pulls recorded".to_owned()
        } else {
            self.source_pulls
                .iter()
                .map(|pull| {
                    format!(
                        "{} ({}) as of unix {}",
                        pull.source_label, pull.source_id, pull.pulled_at
                    )
                })
                .collect::<Vec<_>>()
                .join("; ")
        };
        lines.push(format!(
            "Retained chart: version {}, {} active record(s); source pulls: {}",
            self.chart_version, self.chart_record_count, pulls_summary
        ));

        if let DispatchCheckStanding::FailedSinceLastSuccess {
            last_attempt_at,
            last_success_at,
        } = self.check_standing
        {
            match last_success_at {
                Some(ok_at) => lines.push(format!(
                    "Source limit (unavailable): latest source check at unix {last_attempt_at} failed; retained chart reflects earlier records as of unix {ok_at}."
                )),
                None => lines.push(format!(
                    "Source limit (unavailable): latest source check at unix {last_attempt_at} failed before any source pull succeeded."
                )),
            }
        }

        if let Some(saved) = &self.understanding {
            lines.push(format!(
                "Saved patient understanding (revision {}, source version {}; internal working overview is never medication truth and must never be disclosed as internal state to the physician):",
                saved.revision, saved.source_version
            ));
            lines.push(format!("- Patient summary: {}", saved.summary.narrative));
            if let Some(scope_note) = &saved.summary.scope_note {
                lines.push(format!("- Evidence scope note: {scope_note}"));
            }
            if !saved.summary.material_gaps.is_empty() {
                lines.push(format!(
                    "- Material gaps: {}",
                    saved.summary.material_gaps.join("; ")
                ));
            }
            lines.push(format!(
                "- Internal working overview: {}",
                saved.operating_index.working_overview
            ));
            if !saved.operating_index.open_threads.is_empty() {
                lines.push(format!(
                    "- Internal open threads: {}",
                    saved.operating_index.open_threads.join("; ")
                ));
            }
        }

        match &self.memory_standing {
            PatientMemoryStanding::Available { entries } => {
                if !entries.is_empty() {
                    lines.push("Durable patient memory:".to_owned());
                    for entry in entries {
                        let origin = match &entry.provenance {
                            MemoryProvenance::PhysicianCorrection {
                                physician,
                                chat_id,
                                message_id,
                                ..
                            } => format!(
                                "physician-supplied correction by {physician} in {chat_id} ({message_id}) at unix {}",
                                entry.updated_at
                            ),
                            MemoryProvenance::ClinicalSummary { job_id, .. } => format!(
                                "clinical summary from {job_id} at unix {}",
                                entry.updated_at
                            ),
                        };
                        lines.push(format!(
                            "- [{}] {}: {} ({origin})",
                            entry.id, entry.topic, entry.statement
                        ));
                    }
                }
            }
            PatientMemoryStanding::Unavailable { reason } => {
                lines.push(format!(
                    "Memory limit (unavailable): durable patient memory could not be read ({reason}); do not assume prior physician corrections are absent."
                ));
            }
        }

        if !self.open_concerns.is_empty() {
            lines.push("Open patient concerns:".to_owned());
            for concern in &self.open_concerns {
                lines.push(format!(
                    "- [{}] ({}): {}",
                    concern.concern_id,
                    concern.urgency.as_str(),
                    concern.reason
                ));
            }
        }

        if !self.recent_work.is_empty() {
            lines.push(
                "Earlier saved work in this workspace (use retrieve_patient_work to inspect details):"
                    .to_owned(),
            );
            for item in &self.recent_work {
                let target = match &item.target {
                    PriorWorkKind::Conversation { chat_id } => format!("conversation {chat_id}"),
                    PriorWorkKind::BackgroundJob { job_id, job_kind } => {
                        format!("{} {job_id}", job_kind.as_str())
                    }
                    PriorWorkKind::ReplyDraft { reply_id } => format!("reply draft {reply_id}"),
                };
                lines.push(format!("- {target}: {} ({})", item.title, item.outcome));
            }
        }

        lines.join("\n")
    }

    /// Developer context item injected into the initial chat request and fresh windows.
    pub fn developer_item(&self) -> Item {
        Item::Feedback {
            text: self.developer_text(),
        }
    }
}

/// Failure when starting a physician chat turn through [`Store::start_chat_authorized`].
#[derive(Debug, thiserror::Error)]
pub enum ChatStartError {
    /// Physician session, practice membership, or panel access was refused.
    #[error("physician lacks access to patient workspace: {reason}")]
    AccessDenied {
        /// Refusal reason reported by the authority owner.
        reason: String,
    },
    /// Command input or runtime persistence failed.
    #[error("{0}")]
    Runtime(#[from] RuntimeError),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RefreshToolArgs {
    source_id: Option<String>,
    reason: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
enum MemoryToolCall {
    Read {
        #[serde(default)]
        include_retired: bool,
    },
    RecordCorrection {
        memory_id: PatientMemoryId,
        topic: String,
        statement: String,
    },
    Retire {
        memory_id: PatientMemoryId,
        reason: String,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
enum RetrieveWorkToolCall {
    List { limit: Option<usize> },
    ReadConversation { chat_id: String },
    ReadBackgroundJob { job_id: String },
}

/// Tool schema offered in physician chat to refresh patient sources on demand.
pub fn refresh_sources_tool() -> Tool {
    Tool {
        name: REFRESH_SOURCES_TOOL.into(),
        description: "Pull fresh records from connected patient sources for this patient workspace when recency matters or retained sources appear stale. Supply source_id to refresh one source or null to refresh all configured sources for this patient, along with a brief clinical reason. If a refresh fails or no live source is available, retained records stay unchanged and an explicit unavailable limit is returned.".into(),
        parameters: json!({
            "type": "object",
            "properties": {
                "source_id": { "type": ["string", "null"] },
                "reason": { "type": "string", "minLength": 1, "maxLength": 500 }
            },
            "required": ["source_id", "reason"],
            "additionalProperties": false
        }),
    }
}

/// Tool schema offered in physician chat to read, record, or retire durable patient memory.
pub fn patient_memory_tool() -> Tool {
    Tool {
        name: PATIENT_MEMORY_TOOL.into(),
        description: "Read, record, or retire durable patient memory carried across conversations for this patient. Use `record_correction` when the physician supplies or corrects a durable patient fact so future conversations keep that physician-attributed fact distinct from chart records without overwriting unrelated memory. Use `retire` with a single-line reason when a saved memory entry is superseded or withdrawn. Use `read` to inspect saved memory entries.".into(),
        parameters: json!({
            "anyOf": [
                {
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string", "enum": ["read"] },
                        "include_retired": { "type": "boolean" }
                    },
                    "required": ["operation"],
                    "additionalProperties": false
                },
                {
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string", "enum": ["record_correction"] },
                        "memory_id": { "type": "string", "minLength": 1, "maxLength": 256 },
                        "topic": { "type": "string", "minLength": 1, "maxLength": MAX_MEMORY_TOPIC_CHARS },
                        "statement": { "type": "string", "minLength": 1, "maxLength": MAX_MEMORY_STATEMENT_CHARS }
                    },
                    "required": ["operation", "memory_id", "topic", "statement"],
                    "additionalProperties": false
                },
                {
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string", "enum": ["retire"] },
                        "memory_id": { "type": "string", "minLength": 1, "maxLength": 256 },
                        "reason": { "type": "string", "minLength": 1, "maxLength": MAX_MEMORY_RETIREMENT_CHARS }
                    },
                    "required": ["operation", "memory_id", "reason"],
                    "additionalProperties": false
                }
            ]
        }),
    }
}

/// Tool schema offered in physician chat to list and read earlier conversations and background work.
pub fn retrieve_patient_work_tool() -> Tool {
    Tool {
        name: RETRIEVE_PATIENT_WORK_TOOL.into(),
        description: "List or read earlier saved conversations, reply drafts, and completed background clinical reviews for this patient workspace. Use `list` to see saved conversations and work across sessions; use `read_conversation` with `chat_id` or `read_background_job` with `job_id` to inspect exact saved questions, answers, citations, and clinical outputs.".into(),
        parameters: json!({
            "anyOf": [
                {
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string", "enum": ["list"] },
                        "limit": { "type": ["integer", "null"], "minimum": 1, "maximum": MAX_RETRIEVED_WORK_ITEMS }
                    },
                    "required": ["operation", "limit"],
                    "additionalProperties": false
                },
                {
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string", "enum": ["read_conversation"] },
                        "chat_id": { "type": "string", "minLength": 1, "maxLength": 256 }
                    },
                    "required": ["operation", "chat_id"],
                    "additionalProperties": false
                },
                {
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string", "enum": ["read_background_job"] },
                        "job_id": { "type": "string", "minLength": 1, "maxLength": 256 }
                    },
                    "required": ["operation", "job_id"],
                    "additionalProperties": false
                }
            ]
        }),
    }
}

impl Store {
    /// Accepts new physician chat input only after verifying and locking the
    /// physician's session, practice membership, and patient panel access in the
    /// same transaction, then delegating to the single [`Store::accept_in_tx`]
    /// owner. Unauthorized callers fail closed without saving any receipt,
    /// conversation row, or pending run.
    pub async fn start_chat_authorized(
        &self,
        scope: &Scope,
        input: &Input,
        session_hash: Vec<u8>,
        skills: &PhysicianSkills,
        recipe: &engine::recipe::RequestRecipe,
        authority: &mut dyn Authority,
    ) -> Result<Accepted, ChatStartError> {
        if session_hash.len() != 32 {
            return Err(ChatStartError::AccessDenied {
                reason: "invalid session reference".into(),
            });
        }
        Self::validate_accept_fields(scope, input)?;

        let tx = self.db.begin().await.map_err(RuntimeError::from)?;
        if let Err(reason) = authority.check(&tx, scope, Some(&session_hash)).await {
            tx.rollback().await.map_err(RuntimeError::from)?;
            return Err(ChatStartError::AccessDenied { reason });
        }
        if let Err(reason) = authority
            .lock_publication(&tx, scope, Some(&session_hash))
            .await
        {
            tx.rollback().await.map_err(RuntimeError::from)?;
            return Err(ChatStartError::AccessDenied { reason });
        }

        Self::accept_in_tx(tx, scope, input, session_hash, skills, recipe, None)
            .await
            .map_err(ChatStartError::from)
    }

    /// Reads durable patient memory entries for `(practice, workspace)` through
    /// the configured [`PatientMemoryOperation`].
    pub async fn patient_memories(
        &self,
        practice: &str,
        workspace: &str,
        selection: MemorySelection,
    ) -> Result<Vec<PatientMemoryEntry>, PatientMemoryError> {
        self.memory_operation
            .list_memories(self, practice, workspace, selection)
            .await
    }

    /// Records or updates a durable patient memory entry through the configured
    /// [`PatientMemoryOperation`].
    pub async fn record_patient_memory(
        &self,
        command_key: &str,
        input: &RecordMemoryCorrection,
    ) -> Result<MemoryMutation, PatientMemoryError> {
        self.memory_operation
            .record_correction(self, command_key, input)
            .await
    }

    /// Retires a durable patient memory entry through the configured
    /// [`PatientMemoryOperation`].
    pub async fn retire_patient_memory(
        &self,
        command_key: &str,
        input: &RetirePatientMemory,
    ) -> Result<MemoryMutation, PatientMemoryError> {
        self.memory_operation
            .retire_memory(self, command_key, input)
            .await
    }

    /// Reads active durable patient memory standing for `(practice, workspace)`,
    /// propagating database errors while representing upstream memory outages as
    /// [`PatientMemoryStanding::Unavailable`].
    pub(crate) async fn active_memory_standing(
        &self,
        practice: &str,
        workspace: &str,
    ) -> Result<PatientMemoryStanding, DbErr> {
        match self
            .patient_memories(practice, workspace, MemorySelection::ActiveOnly)
            .await
        {
            Ok(entries) => Ok(PatientMemoryStanding::Available { entries }),
            Err(PatientMemoryError::Unavailable { reason }) => {
                Ok(PatientMemoryStanding::Unavailable { reason })
            }
            Err(PatientMemoryError::Database(db_err)) => Err(db_err),
            Err(
                cause @ (PatientMemoryError::InvalidInput(_)
                | PatientMemoryError::IdempotencyCollision { .. }
                | PatientMemoryError::NotFound { .. }
                | PatientMemoryError::AlreadyRetired { .. }),
            ) => Err(error(&cause.to_string())),
        }
    }

    /// Reads the practice dispatcher's source-check standing for `(practice, workspace)`.
    pub async fn dispatch_check_standing(
        &self,
        practice: &str,
        workspace: &str,
    ) -> Result<DispatchCheckStanding, DbErr> {
        let Some(row) =
            patient_dispatch::Entity::find_by_id((practice.to_owned(), workspace.to_owned()))
                .one(&self.db)
                .await?
        else {
            return Ok(DispatchCheckStanding::NotRecorded);
        };
        let standing = match (row.last_attempt_at, row.last_success_at) {
            (None, None) => DispatchCheckStanding::NotRecorded,
            (None, Some(checked_at)) => DispatchCheckStanding::Current { checked_at },
            (Some(attempt_at), Some(success_at)) if attempt_at <= success_at => {
                DispatchCheckStanding::Current {
                    checked_at: success_at,
                }
            }
            (Some(attempt_at), last_success_at) => DispatchCheckStanding::FailedSinceLastSuccess {
                last_attempt_at: attempt_at,
                last_success_at,
            },
        };
        Ok(standing)
    }

    /// Assembles [`PatientStartingContext`] for `scope` from durable source pulls,
    /// chart observations, dispatch check status, patient memory, open concerns,
    /// and saved workspace history.
    pub async fn patient_starting_context(
        &self,
        scope: &Scope,
    ) -> Result<PatientStartingContext, DbErr> {
        scope.validate()?;
        let observation = self.sources_snapshot(&scope.workspace).await?;
        let pulls = self
            .source_pulls(std::slice::from_ref(&scope.workspace))
            .await?
            .remove(&scope.workspace)
            .unwrap_or_default();
        let patient = pulls
            .iter()
            .max_by_key(|pull| pull.pulled_at)
            .map(|pull| pull.patient.clone());
        let source_pulls = pulls
            .into_iter()
            .filter_map(|pull| {
                Some(SourceFreshnessSummary {
                    source_id: pull.source_id,
                    source_label: pull.source_label,
                    pulled_at: pull.pulled_at?,
                })
            })
            .collect();
        let check_standing = self
            .dispatch_check_standing(&scope.practice, &scope.workspace)
            .await?;

        let memory_standing = self
            .active_memory_standing(&scope.practice, &scope.workspace)
            .await?;

        let mut raw_open_concerns = concern::Entity::find()
            .filter(concern::Column::Practice.eq(&scope.practice))
            .filter(concern::Column::Workspace.eq(&scope.workspace))
            .filter(concern::Column::Status.eq("open"))
            .all(&self.db)
            .await?;
        raw_open_concerns.sort_by(|left, right| {
            let rank = |urgency: &str| if urgency == "now" { 0_u8 } else { 1_u8 };
            rank(&left.urgency)
                .cmp(&rank(&right.urgency))
                .then_with(|| left.created_at.cmp(&right.created_at))
                .then_with(|| left.id.cmp(&right.id))
        });
        let mut open_concerns = Vec::with_capacity(raw_open_concerns.len());
        for row in raw_open_concerns {
            let urgency: ConcernUrgency = serde_json::from_value(Value::String(row.urgency))
                .map_err(conversion_error("stored concern urgency", "ConcernUrgency"))?;
            let records: Vec<ExactRecordReference> = serde_json::from_value(row.records).map_err(
                conversion_error("stored concern records", "ExactRecordReference"),
            )?;
            open_concerns.push(OpenConcernSummary {
                concern_id: row.id,
                urgency,
                reason: row.reason,
                records,
            });
        }

        let past_work = self
            .workspace_past_work(&scope.practice, &scope.physician, &scope.workspace)
            .await
            .map_err(|cause| match cause {
                super::history::HistoryError::EmptyScope => error("empty physician scope"),
                super::history::HistoryError::Database(db_err) => db_err,
            })?;
        let mut recent_work: Vec<PriorWorkPointer> = past_work
            .into_iter()
            .filter_map(|item| match item.target {
                super::history::HistoryWorkTarget::Conversation { chat_id }
                    if chat_id == scope.chat =>
                {
                    None
                }
                super::history::HistoryWorkTarget::Conversation { chat_id } => {
                    Some(PriorWorkPointer {
                        target: PriorWorkKind::Conversation { chat_id },
                        title: item.title,
                        outcome: item.outcome.label().to_owned(),
                        updated_at: item.updated_at,
                    })
                }
                super::history::HistoryWorkTarget::ReplyDraft { reply_id, .. } => {
                    Some(PriorWorkPointer {
                        target: PriorWorkKind::ReplyDraft { reply_id },
                        title: item.title,
                        outcome: item.outcome.label().to_owned(),
                        updated_at: item.updated_at,
                    })
                }
            })
            .take(5)
            .collect();

        let completed_jobs = practice_job::Entity::find()
            .filter(practice_job::Column::Practice.eq(&scope.practice))
            .filter(practice_job::Column::Workspace.eq(&scope.workspace))
            .filter(practice_job::Column::Status.eq(WorkStatus::Completed))
            .order_by_desc(practice_job::Column::UpdatedAt)
            .all(&self.db)
            .await?;
        for job in completed_jobs.into_iter().take(3) {
            recent_work.push(PriorWorkPointer {
                target: PriorWorkKind::BackgroundJob {
                    job_id: job.job_id.clone(),
                    job_kind: job.kind,
                },
                title: format!("{} ({})", job.kind.as_str(), job.job_id),
                outcome: "Completed".to_owned(),
                updated_at: job.updated_at,
            });
        }

        let understanding = self
            .patient_understanding_internal(&scope.practice, &scope.workspace)
            .await?;

        Ok(PatientStartingContext {
            workspace: scope.workspace.clone(),
            patient,
            chart_version: observation.version,
            chart_record_count: observation.sources.len(),
            source_pulls,
            check_standing,
            understanding,
            memory_standing,
            open_concerns,
            recent_work,
        })
    }

    /// Executes `search_patient_information` or `read_patient_sources` for a chat
    /// turn, classifying distinct `missing`, `unreadable`, and `unavailable`
    /// limits while preserving exact `ReadReceipt` shapes for citation extraction.
    pub async fn execute_chat_chart_tool(
        &self,
        scope: &Scope,
        name: &str,
        arguments: &Value,
    ) -> Result<ToolOutcome, engine::ProductError> {
        self.execute_chat_chart_tool_with_labels(scope, name, arguments, None)
            .await
    }

    pub(super) async fn execute_chat_chart_tool_with_labels(
        &self,
        scope: &Scope,
        name: &str,
        arguments: &Value,
        labels: Option<&crate::physician_sources::Labels>,
    ) -> Result<ToolOutcome, engine::ProductError> {
        let chart = match ChartRequest::parse(name, arguments, ChartScope::Chat) {
            Ok(chart) => chart,
            Err(reason) => return Ok(ToolOutcome::refused(reason.to_string())),
        };
        let requested_candidates: Option<Vec<String>> = chart
            .candidates()
            .map(|ids| ids.into_iter().map(str::to_owned).collect());
        let observation = match self.chart_observation(&scope.workspace, &chart).await {
            Ok(obs) => obs,
            Err(db_err) => {
                if db_err.to_string().contains("corrupt source observation")
                    || db_err
                        .to_string()
                        .contains("invalid retained physician source revision")
                {
                    let subject = requested_candidates
                        .as_ref()
                        .and_then(|ids| ids.first().cloned())
                        .unwrap_or_else(|| scope.workspace.clone());
                    let limit = RecordLimit {
                        kind: RecordLimitKind::Unreadable,
                        subject: subject.clone(),
                        detail: format!(
                            "Stored source observation for `{subject}` failed integrity validation and cannot be read."
                        ),
                    };
                    return Ok(ToolOutcome::Returned(
                        json!({
                            "results": [{
                                "outcome": "unavailable",
                                "reference": subject,
                                "reason": "unreadable_corrupt_observation"
                            }],
                            "limits": [limit],
                        })
                        .to_string(),
                    ));
                }
                return Err(persistence_failure(db_err));
            }
        };

        let check_standing = self
            .dispatch_check_standing(&scope.practice, &scope.workspace)
            .await
            .map_err(persistence_failure)?;
        let mut limits = Vec::new();
        if let DispatchCheckStanding::FailedSinceLastSuccess {
            last_attempt_at,
            last_success_at,
        } = check_standing
        {
            let detail = match last_success_at {
                Some(ok_at) => format!(
                    "Latest patient source pull at unix {last_attempt_at} failed; newer external records are unavailable beyond unix {ok_at}."
                ),
                None => format!(
                    "Latest patient source pull at unix {last_attempt_at} failed before any source records were delivered."
                ),
            };
            limits.push(RecordLimit {
                kind: RecordLimitKind::Unavailable,
                subject: scope.workspace.clone(),
                detail,
            });
        }

        let outcome = match labels {
            Some(labels) => chart.run_labelled(
                &scope.workspace,
                observation.version,
                &observation.sources,
                labels,
            ),
            None => chart.run(&scope.workspace, observation.version, &observation.sources),
        }
        .map_err(engine::Cause::from)?;
        let ToolOutcome::Returned(raw_json) = outcome else {
            return Ok(outcome);
        };
        let Ok(mut payload) = serde_json::from_str::<Value>(&raw_json) else {
            return Ok(ToolOutcome::Returned(raw_json));
        };

        if name == SEARCH_TOOL {
            if observation.version == 0
                && observation.sources.is_empty()
                && !matches!(
                    check_standing,
                    DispatchCheckStanding::FailedSinceLastSuccess { .. }
                )
            {
                limits.push(RecordLimit {
                    kind: RecordLimitKind::Missing,
                    subject: scope.workspace.clone(),
                    detail: "No chart records have been delivered for this patient workspace."
                        .into(),
                });
            }
            if let Some(obj) = payload.as_object_mut() {
                obj.insert("workspace_version".into(), json!(observation.version));
                if !limits.is_empty() {
                    obj.insert("limits".into(), serde_json::to_value(&limits)?);
                }
            }
            return Ok(ToolOutcome::Returned(payload.to_string()));
        }

        if let Some(results) = payload.get("results").and_then(Value::as_array) {
            for item in results {
                let item_outcome = item.get("outcome").and_then(Value::as_str);
                let reference = item
                    .get("reference")
                    .and_then(Value::as_str)
                    .unwrap_or(&scope.workspace);
                if item_outcome == Some("unavailable") {
                    let reason = item.get("reason").and_then(Value::as_str).unwrap_or("");
                    if reason == "not_in_recorded_observation" {
                        limits.push(RecordLimit {
                            kind: RecordLimitKind::Missing,
                            subject: reference.to_owned(),
                            detail: format!(
                                "Requested chart record `{reference}` is missing from this patient's retained chart observation."
                            ),
                        });
                    }
                } else if matches!(item_outcome, Some("available" | "passage"))
                    && let Some(source) = observation
                        .sources
                        .iter()
                        .find(|source| source.candidate_id == reference)
                    && let Some(unreadable) = inspect_record_readability(source)
                {
                    limits.push(unreadable);
                }
            }
        }

        // Preserve exact byte-for-byte ReadReceipt JSON when no limits apply.
        if limits.is_empty() {
            return Ok(ToolOutcome::Returned(raw_json));
        }
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("limits".into(), serde_json::to_value(&limits)?);
        }
        Ok(ToolOutcome::Returned(payload.to_string()))
    }

    /// Executes one chat tool invocation (`refresh`, `memory`, `retrieve_patient_work`,
    /// `patient_concern`, `clinical_library`, `chart`, or `skill_view`) under `claim`.
    pub async fn execute_chat_tool(
        &self,
        claim: &super::Claim,
        state: &Conversation,
        records: &[(engine::Event, bool)],
        call: &Invocation,
        arguments: Value,
    ) -> Result<ToolOutcome, engine::ProductError> {
        if matches!(call.name.as_str(), SEARCH_TOOL | READ_TOOL) {
            let labels = crate::physician_sources::Labels::of_work(records);
            return self
                .execute_chat_chart_tool_with_labels(
                    &claim.scope,
                    &call.name,
                    &arguments,
                    Some(&labels),
                )
                .await;
        }

        if is_refresh_tool(&call.name) {
            if arguments
                .as_object()
                .is_some_and(|obj| !obj.contains_key("reason") && !obj.contains_key("source_id"))
            {
                let refresh_call = Invocation {
                    id: call.id.clone(),
                    name: call.name.clone(),
                    arguments,
                };
                return self
                    .execute_claimed_physician_refresh_tool(claim, &state.work, &refresh_call)
                    .await;
            }
            let parsed: RefreshToolArgs = match serde_json::from_value(arguments) {
                Ok(parsed) => parsed,
                Err(cause) => {
                    return Ok(ToolOutcome::refused(format!(
                        "Supply source_id (string or null) and a non-empty reason ({cause})."
                    )));
                }
            };
            return match self
                .refresh_operation
                .refresh_sources_for_work(
                    self,
                    &claim.scope,
                    &state.work,
                    parsed.source_id.as_deref(),
                    &parsed.reason,
                )
                .await
            {
                Ok(report) => encode_chat_refresh_report(&report),
                Err(SourceRefreshError::InvalidInput(reason)) => Ok(ToolOutcome::refused(reason)),
                Err(SourceRefreshError::Unavailable { subject, reason }) => {
                    Ok(encode_chat_refresh_limit(subject, reason))
                }
                Err(SourceRefreshError::Upstream { source_id, reason }) => {
                    Ok(encode_chat_refresh_limit(source_id, reason))
                }
                Err(SourceRefreshError::Interrupted(cause)) => Err(cause.into()),
                Err(SourceRefreshError::Database(db_err)) => Err(persistence_failure(db_err)),
            };
        }

        if is_memory_tool(&call.name) {
            let parsed: MemoryToolCall = match serde_json::from_value(arguments)
                .map_err(MemoryInputError::MalformedArguments)
            {
                Ok(parsed) => parsed,
                Err(cause) => return Ok(ToolOutcome::refused(cause.to_string())),
            };
            let attempt = state
                .attempts
                .iter()
                .find(|attempt| attempt.id == claim.attempt)
                .ok_or("missing hosted attempt")?;
            let message_id = state
                .input_id()
                .ok_or("missing accepted physician message for memory provenance")?
                .to_owned();
            let provenance = MemoryProvenance::PhysicianCorrection {
                physician: claim.scope.physician.clone(),
                chat_id: claim.scope.chat.clone(),
                message_id,
                work_id: attempt.work.clone(),
                attempt_id: claim.attempt.clone(),
            };
            let command_key = memory_tool_command_key(&attempt.work, &call.id);

            return match parsed {
                MemoryToolCall::Read { include_retired } => {
                    let selection = if include_retired {
                        MemorySelection::All
                    } else {
                        MemorySelection::ActiveOnly
                    };
                    match self
                        .patient_memories(&claim.scope.practice, &claim.scope.workspace, selection)
                        .await
                    {
                        Ok(entries) => Ok(ToolOutcome::Returned(
                            json!({
                                "outcome": "available",
                                "entries": entries,
                            })
                            .to_string(),
                        )),
                        Err(PatientMemoryError::Unavailable { reason }) => {
                            Ok(ToolOutcome::Returned(
                                json!({
                                    "outcome": "failed",
                                    "saved": false,
                                    "limits": [{
                                        "kind": "unavailable",
                                        "subject": "patient_memory",
                                        "detail": reason,
                                    }],
                                })
                                .to_string(),
                            ))
                        }
                        Err(PatientMemoryError::InvalidInput(cause)) => {
                            Ok(ToolOutcome::refused(cause.to_string()))
                        }
                        Err(PatientMemoryError::Database(db_err)) => {
                            Err(persistence_failure(db_err))
                        }
                        Err(
                            cause @ (PatientMemoryError::IdempotencyCollision { .. }
                            | PatientMemoryError::NotFound { .. }
                            | PatientMemoryError::AlreadyRetired { .. }),
                        ) => Ok(ToolOutcome::refused(cause.to_string())),
                    }
                }
                MemoryToolCall::RecordCorrection {
                    memory_id,
                    topic,
                    statement,
                } => {
                    match self
                        .record_patient_memory(
                            &command_key,
                            &RecordMemoryCorrection {
                                practice: claim.scope.practice.clone(),
                                workspace: claim.scope.workspace.clone(),
                                id: memory_id,
                                topic,
                                statement,
                                provenance,
                            },
                        )
                        .await
                    {
                        Ok(MemoryMutation::Applied(entry)) => Ok(ToolOutcome::Returned(
                            json!({
                                "outcome": "applied",
                                "saved": true,
                                "entry": entry,
                            })
                            .to_string(),
                        )),
                        Ok(MemoryMutation::Duplicate(entry)) => Ok(ToolOutcome::Returned(
                            json!({
                                "outcome": "duplicate",
                                "saved": true,
                                "entry": entry,
                            })
                            .to_string(),
                        )),
                        Err(
                            cause @ (PatientMemoryError::InvalidInput(_)
                            | PatientMemoryError::IdempotencyCollision { .. }
                            | PatientMemoryError::NotFound { .. }
                            | PatientMemoryError::AlreadyRetired { .. }),
                        ) => Ok(ToolOutcome::refused(cause.to_string())),
                        Err(PatientMemoryError::Unavailable { reason }) => {
                            Ok(ToolOutcome::Returned(
                                json!({
                                    "outcome": "failed",
                                    "saved": false,
                                    "error": reason,
                                })
                                .to_string(),
                            ))
                        }
                        Err(PatientMemoryError::Database(db_err)) => {
                            Err(persistence_failure(db_err))
                        }
                    }
                }
                MemoryToolCall::Retire { memory_id, reason } => {
                    match self
                        .retire_patient_memory(
                            &command_key,
                            &RetirePatientMemory {
                                practice: claim.scope.practice.clone(),
                                workspace: claim.scope.workspace.clone(),
                                id: memory_id,
                                reason,
                                retired_by: provenance,
                            },
                        )
                        .await
                    {
                        Ok(MemoryMutation::Applied(entry)) => Ok(ToolOutcome::Returned(
                            json!({
                                "outcome": "applied",
                                "saved": true,
                                "entry": entry,
                            })
                            .to_string(),
                        )),
                        Ok(MemoryMutation::Duplicate(entry)) => Ok(ToolOutcome::Returned(
                            json!({
                                "outcome": "duplicate",
                                "saved": true,
                                "entry": entry,
                            })
                            .to_string(),
                        )),
                        Err(
                            cause @ (PatientMemoryError::InvalidInput(_)
                            | PatientMemoryError::IdempotencyCollision { .. }
                            | PatientMemoryError::NotFound { .. }
                            | PatientMemoryError::AlreadyRetired { .. }),
                        ) => Ok(ToolOutcome::refused(cause.to_string())),
                        Err(PatientMemoryError::Unavailable { reason }) => {
                            Ok(ToolOutcome::Returned(
                                json!({
                                    "outcome": "failed",
                                    "saved": false,
                                    "error": reason,
                                })
                                .to_string(),
                            ))
                        }
                        Err(PatientMemoryError::Database(db_err)) => {
                            Err(persistence_failure(db_err))
                        }
                    }
                }
            };
        }

        if is_retrieve_work_tool(&call.name) {
            return self
                .execute_retrieve_work_tool(&claim.scope, &arguments)
                .await;
        }

        if super::concerns::is_concern_tool(&call.name) {
            let attempt = state
                .attempts
                .iter()
                .find(|attempt| attempt.id == claim.attempt)
                .ok_or("missing hosted attempt")?;
            let source_work = SourceWork::new("physician_chat", &attempt.work, &claim.attempt)
                .map_err(|cause| cause.to_string())?;
            let invocation = Invocation {
                id: call.id.clone(),
                name: call.name.clone(),
                arguments,
            };
            return self
                .execute_concern_tool(
                    &claim.scope.practice,
                    &claim.scope.workspace,
                    &source_work,
                    &invocation,
                )
                .await;
        }

        if crate::initial_understanding::is_understanding_tool(&call.name) {
            return self
                .execute_chat_understanding_tool(claim, call, &arguments)
                .await;
        }

        if crate::clinical_library::is_library_tool(&call.name) {
            let outcome = self
                .clinical_library()
                .execute(&call.name, &arguments)
                .await;
            if let ToolOutcome::Returned(raw) = &outcome
                && let Ok(mut parsed) = serde_json::from_str::<Value>(raw)
                && parsed.get("outcome").and_then(Value::as_str) == Some("unavailable")
            {
                let detail = parsed
                    .get("reason")
                    .or_else(|| parsed.get("message"))
                    .and_then(Value::as_str)
                    .unwrap_or("Clinical library service is unavailable.")
                    .to_owned();
                let limit = RecordLimit {
                    kind: RecordLimitKind::Unavailable,
                    subject: "clinical_library".into(),
                    detail,
                };
                if let Some(obj) = parsed.as_object_mut() {
                    obj.insert("limits".into(), serde_json::to_value([limit])?);
                }
                return Ok(ToolOutcome::Returned(parsed.to_string()));
            }
            return Ok(outcome);
        }

        let skills = self.frozen_skills(&claim.scope, &state.skills).await?;
        Ok(skills.execute(&call.name, &arguments))
    }

    async fn execute_retrieve_work_tool(
        &self,
        scope: &Scope,
        arguments: &Value,
    ) -> Result<ToolOutcome, engine::ProductError> {
        let parsed: RetrieveWorkToolCall = match serde_json::from_value(arguments.clone()) {
            Ok(parsed) => parsed,
            Err(cause) => {
                return Ok(ToolOutcome::refused(format!(
                    "Supply a valid operation (`list`, `read_conversation`, or `read_background_job`) and its required arguments ({cause})."
                )));
            }
        };

        match parsed {
            RetrieveWorkToolCall::List { limit } => {
                let cap = limit.unwrap_or(10).clamp(1, MAX_RETRIEVED_WORK_ITEMS);
                let items = self
                    .workspace_past_work(&scope.practice, &scope.physician, &scope.workspace)
                    .await
                    .map_err(|cause| match cause {
                        super::history::HistoryError::EmptyScope => {
                            engine::ProductError::from("empty physician scope")
                        }
                        super::history::HistoryError::Database(db_err) => {
                            persistence_failure(db_err)
                        }
                    })?;
                let filtered: Vec<_> = items
                    .into_iter()
                    .filter(|item| {
                        !matches!(
                            &item.target,
                            super::history::HistoryWorkTarget::Conversation { chat_id }
                                if chat_id.as_str() == scope.chat
                        )
                    })
                    .take(cap)
                    .collect();
                let jobs = practice_job::Entity::find()
                    .filter(practice_job::Column::Practice.eq(&scope.practice))
                    .filter(practice_job::Column::Workspace.eq(&scope.workspace))
                    .filter(practice_job::Column::Status.eq(WorkStatus::Completed))
                    .order_by_desc(practice_job::Column::UpdatedAt)
                    .all(&self.db)
                    .await
                    .map_err(persistence_failure)?;
                let background_jobs: Vec<_> = jobs
                    .into_iter()
                    .take(cap)
                    .map(|job| {
                        json!({
                            "job_id": job.job_id,
                            "kind": job.kind.as_str(),
                            "sources_version": job.sources_version,
                            "updated_at": job.updated_at,
                        })
                    })
                    .collect();
                Ok(ToolOutcome::Returned(
                    json!({
                        "outcome": "available",
                        "past_work": filtered,
                        "background_jobs": background_jobs,
                    })
                    .to_string(),
                ))
            }
            RetrieveWorkToolCall::ReadConversation { chat_id } => {
                let target_chat = chat_id.trim();
                if target_chat.is_empty() {
                    return Ok(ToolOutcome::refused("Supply a non-empty chat_id."));
                }
                let target_scope = Scope {
                    practice: scope.practice.clone(),
                    physician: scope.physician.clone(),
                    workspace: scope.workspace.clone(),
                    chat: target_chat.to_owned(),
                };
                let readback = self
                    .reopen_conversation(&target_scope)
                    .await
                    .map_err(|cause| match cause {
                        super::history::HistoryError::EmptyScope => {
                            engine::ProductError::from("empty physician scope")
                        }
                        super::history::HistoryError::Database(db_err) => {
                            persistence_failure(db_err)
                        }
                    })?;
                let Some(readback) = readback else {
                    return Ok(ToolOutcome::Returned(
                        json!({
                            "outcome": "unavailable",
                            "limits": [{
                                "kind": "missing",
                                "subject": target_chat,
                                "detail": format!("Conversation `{target_chat}` does not exist in this patient's authorized history."),
                            }],
                        })
                        .to_string(),
                    ));
                };
                let turns: Vec<_> = readback
                    .turns
                    .iter()
                    .map(|turn| {
                        let role = match turn.author {
                            super::history::TurnAuthor::Physician => "physician",
                            super::history::TurnAuthor::Assistant { partial: false } => "assistant",
                            super::history::TurnAuthor::Assistant { partial: true } => {
                                "assistant_partial"
                            }
                        };
                        json!({
                            "id": turn.id,
                            "author": role,
                            "text": turn.raw_text,
                            "newer_records": turn.newer_records,
                        })
                    })
                    .collect();
                Ok(ToolOutcome::Returned(
                    json!({
                        "outcome": "available",
                        "chat_id": target_chat,
                        "status": readback.outcome,
                        "turns": turns,
                        "sources": readback.sources,
                    })
                    .to_string(),
                ))
            }
            RetrieveWorkToolCall::ReadBackgroundJob { job_id } => {
                let target_job = job_id.trim();
                if target_job.is_empty() {
                    return Ok(ToolOutcome::refused("Supply a non-empty job_id."));
                }
                let row = practice_job::Entity::find_by_id((
                    scope.practice.clone(),
                    scope.workspace.clone(),
                    target_job.to_owned(),
                ))
                .one(&self.db)
                .await
                .map_err(persistence_failure)?;
                let Some(row) = row else {
                    return Ok(ToolOutcome::Returned(
                        json!({
                            "outcome": "unavailable",
                            "limits": [{
                                "kind": "missing",
                                "subject": target_job,
                                "detail": format!("Background job `{target_job}` does not exist in this patient workspace."),
                            }],
                        })
                        .to_string(),
                    ));
                };
                Ok(ToolOutcome::Returned(
                    json!({
                        "outcome": "available",
                        "job_id": row.job_id,
                        "kind": row.kind.as_str(),
                        "status": row.status,
                        "sources_version": row.sources_version,
                        "result": row.result,
                        "updated_at": row.updated_at,
                    })
                    .to_string(),
                ))
            }
        }
    }

    /// Reconciles an already-committed durable chat tool effect read-only when an
    /// attempt was interrupted after `ToolStarted` and before `ToolReturned`.
    pub(super) async fn saved_chat_tool_outcome(
        &self,
        claim: &super::Claim,
        call: &Invocation,
    ) -> Result<Option<ToolOutcome>, engine::ProductError> {
        let state = self
            .conversation(&claim.scope)
            .await
            .map_err(persistence_failure)?;
        let Some(attempt) = state
            .attempts
            .iter()
            .find(|attempt| attempt.id == claim.attempt)
        else {
            return Ok(None);
        };

        if super::concerns::is_concern_tool(&call.name) {
            return self
                .saved_concern_tool_outcome(
                    &claim.scope.practice,
                    &claim.scope.workspace,
                    &attempt.work,
                    call,
                )
                .await;
        }

        if is_refresh_tool(&call.name) || super::refresh::is_refresh_tool(&call.name) {
            let Ok(arguments) = serde_json::from_str::<Value>(&call.arguments) else {
                return Ok(None);
            };
            if arguments
                .as_object()
                .is_some_and(|obj| !obj.contains_key("reason") && !obj.contains_key("source_id"))
            {
                return self
                    .saved_physician_refresh_outcome(&claim.scope, &attempt.work, call)
                    .await;
            }
            let Ok(parsed) = serde_json::from_value::<RefreshToolArgs>(arguments) else {
                return Ok(None);
            };
            return match self
                .refresh_operation
                .saved_refresh_for_work(
                    self,
                    &claim.scope,
                    &attempt.work,
                    parsed.source_id.as_deref(),
                    &parsed.reason,
                )
                .await
            {
                Ok(SavedChatRefresh::NotYetSaved) => Ok(None),
                Ok(SavedChatRefresh::Refreshed(report)) => {
                    encode_chat_refresh_report(&report).map(Some)
                }
                Ok(SavedChatRefresh::Failed { source_id, reason }) => {
                    Ok(Some(encode_chat_refresh_limit(source_id, reason)))
                }
                Err(SourceRefreshError::InvalidInput(reason)) => {
                    Ok(Some(ToolOutcome::refused(reason)))
                }
                Err(SourceRefreshError::Database(db_err)) => Err(persistence_failure(db_err)),
                Err(other) => Err(other.into()),
            };
        }

        if call.name == crate::initial_understanding::SAVE_UNDERSTANDING_TOOL {
            return self.saved_chat_understanding_outcome(claim, call).await;
        }

        if call.name != PATIENT_MEMORY_TOOL {
            return Ok(None);
        }
        let Ok(parsed) = serde_json::from_str::<MemoryToolCall>(&call.arguments) else {
            return Ok(None);
        };
        let memory_id = match parsed {
            MemoryToolCall::Read { .. } => return Ok(None),
            MemoryToolCall::RecordCorrection { memory_id, .. }
            | MemoryToolCall::Retire { memory_id, .. } => memory_id,
        };
        let command_key = memory_tool_command_key(&attempt.work, &call.id);
        let entry = match self
            .memory_operation
            .saved_mutation(
                self,
                &claim.scope.practice,
                &claim.scope.workspace,
                &command_key,
                &memory_id,
            )
            .await
        {
            Ok(Some(entry)) => entry,
            Ok(None) => return Ok(None),
            Err(PatientMemoryError::Database(db_err)) => return Err(persistence_failure(db_err)),
            Err(cause) => return Err(cause.to_string().into()),
        };
        Ok(Some(ToolOutcome::Returned(
            json!({
                "outcome": "duplicate",
                "saved": true,
                "entry": entry,
            })
            .to_string(),
        )))
    }
}

fn encode_chat_refresh_report(
    report: &SourceRefreshReport,
) -> Result<ToolOutcome, engine::ProductError> {
    let mut out = json!({
        "outcome": "refreshed",
        "saved": true,
        "workspace_version": report.workspace_version,
        "sources": report.sources,
    });
    if !report.limits.is_empty()
        && let Some(obj) = out.as_object_mut()
    {
        obj.insert("limits".into(), serde_json::to_value(&report.limits)?);
    }
    Ok(ToolOutcome::Returned(out.to_string()))
}

fn encode_chat_refresh_limit(subject: String, detail: String) -> ToolOutcome {
    let limit = RecordLimit {
        kind: RecordLimitKind::Unavailable,
        subject,
        detail,
    };
    ToolOutcome::Returned(
        json!({
            "outcome": "failed",
            "saved": false,
            "limits": [limit],
        })
        .to_string(),
    )
}

fn memory_tool_command_key(work_id: &str, call_id: &str) -> String {
    format!("{}:{}", work_id, call_id.trim())
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::physician_sources::{ReadablePart, RecordDate};

    fn sample_source(
        content: &str,
        readable_parts: Vec<ReadablePart>,
        provenance: Value,
    ) -> SourceRevision {
        SourceRevision {
            workspace: "pw-1".into(),
            source_id: "clinic-ehr".into(),
            candidate_id: "doc-echo-1".into(),
            revision: "rev-1".into(),
            record_type: "DocumentReference".into(),
            dates: vec![RecordDate {
                name: "date".into(),
                value: "2026-11-10".into(),
            }],
            sha256: crate::sha256_hex(content.as_bytes()),
            content: content.into(),
            readable_parts,
            provenance,
            envelope: None,
        }
    }

    #[test]
    fn readability_classifies_binary_pdf_without_extracted_text_as_unreadable() {
        let pdf_without_text = sample_source(
            r#"{"resourceType":"DocumentReference","content":[{"attachment":{"contentType":"application/pdf","data":"JVBERi0xLjQK"}}]}"#,
            vec![],
            json!({"id": "doc-echo-1"}),
        );
        let limit = inspect_record_readability(&pdf_without_text).expect("unreadable limit");
        assert_eq!(limit.kind, RecordLimitKind::Unreadable);
        assert_eq!(limit.subject, "doc-echo-1");
        assert!(limit.detail.contains("application/pdf"));

        let pdf_with_extracted_text = sample_source(
            r#"{"resourceType":"DocumentReference","content":[{"attachment":{"contentType":"application/pdf","data":"JVBERi0xLjQK"}}]}"#,
            vec![ReadablePart {
                label: "extracted_text".into(),
                text: "LVEF 58%, normal valvular function.".into(),
            }],
            json!({"id": "doc-echo-1"}),
        );
        assert_eq!(inspect_record_readability(&pdf_with_extracted_text), None);
    }

    #[test]
    fn readability_classifies_explicit_provenance_unreadable_reason() {
        let corrupt_scan = sample_source(
            r#"{"resourceType":"DiagnosticReport","conclusion":"See scanned report"}"#,
            vec![],
            json!({"id": "doc-echo-1", "unreadable_reason": "scanned fax pages 2-3 were blank"}),
        );
        let limit = inspect_record_readability(&corrupt_scan).expect("unreadable limit");
        assert_eq!(limit.kind, RecordLimitKind::Unreadable);
        assert!(limit.detail.contains("pages 2-3 were blank"));
    }
}
