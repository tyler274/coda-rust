//! Physician chat recipe operations: starting context assembly, distinct
//! evidence limit classification (`missing`, `unreadable`, `unavailable`),
//! authorized chat admission, durable patient memory with physician provenance,
//! cross-conversation work retrieval, source refresh, and unified tool routing.

use std::fmt;
use std::sync::Arc;

use async_trait::async_trait;
use harness::engine::{Item, Tool};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set, TransactionTrait, Unchanged,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::concerns::{ConcernUrgency, ExactRecordReference};
use super::entities::{command, patient_memory};
use super::jobs::PracticeJobKind;
use super::sources::SourcePull;
use super::{RuntimeError, Scope, Store, conversion_error, error, now, receipt};
use crate::physician_sources::{PatientSource, SourcePatient, SourceRevision};

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
    /// Database failure while reading or committing refreshed observations.
    #[error("{0}")]
    Database(#[from] DbErr),
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

#[async_trait]
impl PatientRefreshOperation for RuntimeSourceRefreshOperation {
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
        let target = match source_id.map(str::trim).filter(|id| !id.is_empty()) {
            None => super::refresh::RefreshTarget::AllSources,
            Some(raw_id) => {
                let parsed = super::refresh::SourceId::new(raw_id)
                    .map_err(|cause| SourceRefreshError::InvalidInput(cause.to_string()))?;
                super::refresh::RefreshTarget::selected([parsed])
                    .map_err(|cause| SourceRefreshError::InvalidInput(cause.to_string()))?
            }
        };
        let command_key = super::refresh::RefreshCommandKey::new(format!(
            "chat:{}:{}",
            scope.chat.trim(),
            uuid::Uuid::new_v4()
        ))
        .map_err(|cause| SourceRefreshError::InvalidInput(cause.to_string()))?;
        let request = super::refresh::RefreshRequest::new(command_key, target);

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
                return Err(SourceRefreshError::Unavailable {
                    subject: scope.workspace.clone(),
                    reason: cause.to_string(),
                });
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
