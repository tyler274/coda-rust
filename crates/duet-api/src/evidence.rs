//! Canonical source mutation and exact account-bound reads; never a search or summary substitute.
//!
//! Authority: every mutation is applied under an [`IngestAuthority`]. The public
//! HTTP routes only ever use [`IngestAuthority::Device`]; server-owned portal
//! originals (`epic`, `oracleHealth`, `healow`, `googleHealth`) can only be
//! created, corrected or removed in-process through
//! [`EvidenceStore::ingest_with_authority`] with a matching
//! [`IngestAuthority::Connector`]. A stored record's source kind, connection and
//! note label are fixed at creation.
//!
//! Removal: [`EvidenceOperation::Delete`] is an explicit source removal. It keeps
//! the base row and every retained revision, marks membership
//! `deleted = source_removed = true`, and leaves earlier revisions readable as
//! non-current citations. Privacy erasure (`privacy.rs`) deletes the base rows,
//! cascading the retained revisions, and stays authoritative. Missing records,
//! failed reads and lost permission are never deletions.
//!
//! Dates: declared instants are projected into `measured_at` / `recorded_at`
//! only when the source states an RFC 3339 instant; otherwise the column is NULL
//! and the literal stays in the payload. Receipt time is never a source date.
use crate::{
    ApiError,
    database::AccountDatabase,
    evidence_entity::{
        self as orm, account, clinical_note, health_evidence, ingestion, membership, revision,
    },
    identity::{AccountContext, check_current},
};
use chrono::{DateTime, Utc};
use duet_wire::models::addressed_record_revision::Entity;
use sea_orm::{
    ActiveValue::Set,
    ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait,
    sea_query::{Expr, ExprTrait, OnConflict},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::{Acquire, Postgres, Transaction};
use uuid::Uuid;

/// Exact canonical address retaining arbitrary original source identifiers.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AddressedRecordRevision {
    /// Immutable canonical UUID.
    pub canonical_record_id: Uuid,
    /// Source family.
    pub entity: Entity,
    /// Literal original source identity (not constrained to UUID).
    pub record_id: String,
    /// Immutable revision.
    pub revision: i32,
}
impl AddressedRecordRevision {
    /// Construct an exact source address.
    pub fn new(
        canonical_record_id: Uuid,
        entity: Entity,
        record_id: impl Into<String>,
        revision: i32,
    ) -> Self {
        Self {
            canonical_record_id,
            entity,
            record_id: record_id.into(),
            revision,
        }
    }
}

/// Source kinds whose originals only server-side connectors may write.
pub const SERVER_SOURCE_KINDS: [&str; 4] = ["epic", "oracleHealth", "healow", "googleHealth"];
/// Health evidence source kinds a device upload may write.
pub const DEVICE_SOURCE_KINDS: [&str; 4] = ["appleHealth", "healthRecords", "physical", "manual"];
/// Device kinds whose native source may explicitly remove a record.
pub const DEVICE_REMOVABLE_KINDS: [&str; 2] = ["appleHealth", "healthRecords"];
/// Clinical note labels a device upload may create. Fixed at creation.
pub const DEVICE_NOTE_LABELS: [&str; 2] = ["Health Records", "Added in Duet"];
/// Patient-authored note label; such wording must carry its own date.
pub const PATIENT_NOTE_LABEL: &str = "Added in Duet";
/// Maximum mutations in one batch.
pub const MAX_EVIDENCE_BATCH_ITEMS: usize = 100;
/// Maximum serialized bytes of one source object.
pub const MAX_EVIDENCE_ITEM_BYTES: usize = 2_000_000;
/// Maximum request body of the batch route.
pub const MAX_EVIDENCE_BATCH_BYTES: usize = 8 * 1024 * 1024;
/// Conflict message (and stable error code) for an unauthorized source mutation.
pub const SOURCE_AUTHORITY_REFUSED: &str = "source_authority_refused";

/// Mutation kind. Defaults to `upsert`; `delete` is an explicit source removal.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceOperation {
    /// Create, or correct against `expected_revision`.
    #[default]
    Upsert,
    /// Explicit removal by the owning source, fenced by exact revision and generation.
    Delete,
}
impl EvidenceOperation {
    fn is_upsert(&self) -> bool {
        *self == Self::Upsert
    }
}

/// Who is writing. Only in-process server code can construct a connector authority;
/// HTTP routes always ingest as [`IngestAuthority::Device`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum IngestAuthority {
    /// Patient device or client upload.
    Device,
    /// Server-side portal connector publishing originals it acquired.
    Connector {
        /// One of [`SERVER_SOURCE_KINDS`].
        source_kind: String,
        /// Stable connection identity; must equal `payload.source.connection_id`.
        connection_id: String,
    },
}
impl IngestAuthority {
    /// Validated connector authority.
    pub fn connector(
        source_kind: &str,
        connection_id: impl Into<String>,
    ) -> Result<Self, ApiError> {
        let connection_id = connection_id.into();
        if !SERVER_SOURCE_KINDS.contains(&source_kind) || connection_id.trim().is_empty() {
            return Err(ApiError::Unprocessable("invalid connector authority"));
        }
        Ok(Self::Connector {
            source_kind: source_kind.to_owned(),
            connection_id,
        })
    }
}

/// Literal source mutation. Unknown fields, units, dates and statuses remain in `payload`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceMutation {
    /// Stable ingestion identity; retry uses identical bytes/values.
    pub operation_id: Uuid,
    /// Canonical identity retained across corrections.
    pub canonical_record_id: Uuid,
    /// Original client/source identity, never replaced by a summary identity.
    pub record_id: String,
    /// Original entity family.
    pub entity: Entity,
    /// Expected previous immutable revision; None means create only.
    pub expected_revision: Option<i64>,
    /// Source generation observed when upload was prepared; omitted only for an initial source generation.
    #[serde(default)]
    pub source_generation: Option<i32>,
    /// `upsert` (default) or an explicit source `delete`. Omitted from the
    /// idempotency fingerprint when `upsert`, so earlier acknowledgements replay.
    #[serde(default, skip_serializing_if = "EvidenceOperation::is_upsert")]
    pub operation: EvidenceOperation,
    /// Exact source object, including provenance and declared date roles.
    /// A `delete` carries only `{"source":{"kind":..}}` (plus `connection_id` for connectors).
    pub payload: Value,
}
/// An exact read receipt and unmodified source object.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExactEvidence {
    /// Address for this precise revision.
    pub address: AddressedRecordRevision,
    /// Literal source payload (not a summary).
    pub payload: Value,
    /// Whether this revision is the currently visible source revision.
    pub current: bool,
}
/// Incremental membership, including deletion visibility without erased prose.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvidenceChange {
    /// Source family.
    pub entity: Entity,
    /// Canonical identity.
    pub canonical_record_id: Uuid,
    /// Original source identity.
    pub record_id: String,
    /// Account-serialized mutation cursor, not a sequence allocation order.
    pub revision: i64,
    /// Whether the record is no longer current (source removal or privacy erasure).
    pub deleted: bool,
    /// Whether the owning source removed it; retained revisions stay citable.
    #[serde(default)]
    pub source_removed: bool,
}
/// One transactionally consistent snapshot/delta. Read all members before advancing the cursor.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvidenceSnapshot {
    /// Account mutation fence captured under the account lock.
    pub cursor: i64,
    /// Source withdrawal generation captured in the same account transaction.
    pub health_records_generation: i32,
    /// Changed membership since requested cursor (all members for zero).
    pub changes: Vec<EvidenceChange>,
    /// Current exact source bodies for the non-deleted members.
    pub records: Vec<ExactEvidence>,
}
/// Bounded batch of independent mutations.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceBatch {
    /// One to [`MAX_EVIDENCE_BATCH_ITEMS`] mutations, applied in order.
    pub items: Vec<EvidenceMutation>,
}
/// Typed reason one batch item was refused. Nothing is skipped silently.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceRejectionCode {
    /// Stale revision/generation, reused identity or withdrawn identity.
    Conflict,
    /// The authority may not write this source or record.
    SourceAuthorityRefused,
    /// The mutation is malformed or unbounded.
    Invalid,
    /// The removal names no current record.
    NotFound,
}
/// One refused batch item.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EvidenceRejection {
    /// Typed category.
    pub code: EvidenceRejectionCode,
    /// Literal refusal message.
    pub message: String,
}
/// Outcome of one batch item, in request order.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum EvidenceItemOutcome {
    /// Committed (or idempotently replayed) at this exact address.
    Accepted {
        /// Request index.
        index: usize,
        /// Item operation identity.
        operation_id: Uuid,
        /// Exact acknowledged address.
        address: AddressedRecordRevision,
    },
    /// Rolled back to its savepoint; siblings are unaffected.
    Rejected {
        /// Request index.
        index: usize,
        /// Item operation identity.
        operation_id: Uuid,
        /// Typed refusal.
        rejection: EvidenceRejection,
    },
}
/// Per-item results for a batch.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EvidenceBatchResult {
    /// One outcome per request item, in order.
    pub results: Vec<EvidenceItemOutcome>,
}

/// Canonical evidence owner, reusable by selection/export/grounding within their transaction.
#[derive(Clone)]
pub struct EvidenceStore {
    /// Shared account database boundary.
    pub database: AccountDatabase,
}
fn entity_name(entity: Entity) -> &'static str {
    match entity {
        Entity::ClinicalNote => "clinical_note",
        Entity::HealthEvidence => "health_evidence",
    }
}
fn hash(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect()
}
fn wire_revision(revision: i64) -> Result<i32, ApiError> {
    i32::try_from(revision).map_err(|_| ApiError::Unavailable("revision exceeds wire range"))
}
fn refused() -> ApiError {
    ApiError::Conflict(SOURCE_AUTHORITY_REFUSED)
}
/// A declared instant: RFC 3339 → Some; absent/null/partial literal → None.
fn declared_instant(value: Option<&Value>) -> Result<Option<DateTime<Utc>>, ApiError> {
    match value {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(text)) => Ok(text.parse().ok()),
        Some(_) => Err(ApiError::Unprocessable("declared dates must be strings")),
    }
}
fn unique_conflict(error: sqlx::Error) -> ApiError {
    match &error {
        sqlx::Error::Database(database) if database.code().as_deref() == Some("23505") => {
            ApiError::Conflict("source identity was withdrawn or differs")
        }
        _ => error.into(),
    }
}
/// Map a per-item refusal to a typed rejection; infrastructure errors abort the batch.
fn rejection(error: ApiError) -> Result<EvidenceRejection, ApiError> {
    let (code, message) = match error {
        ApiError::Conflict(SOURCE_AUTHORITY_REFUSED) => (
            EvidenceRejectionCode::SourceAuthorityRefused,
            SOURCE_AUTHORITY_REFUSED,
        ),
        ApiError::Conflict(message) => (EvidenceRejectionCode::Conflict, message),
        ApiError::Unprocessable(message) => (EvidenceRejectionCode::Invalid, message),
        ApiError::NotFound => (EvidenceRejectionCode::NotFound, "source record not found"),
        other => return Err(other),
    };
    Ok(EvidenceRejection {
        code,
        message: message.to_owned(),
    })
}
fn source_of(payload: &Value) -> (Option<&str>, Option<&str>) {
    (
        payload.pointer("/source/kind").and_then(Value::as_str),
        payload
            .pointer("/source/connection_id")
            .and_then(Value::as_str),
    )
}

impl EvidenceStore {
    /// Construct an evidence owner over the shared boundary.
    pub fn new(database: AccountDatabase) -> Self {
        Self { database }
    }
    /// Device upload. Idempotent, optimistic; locks the account so cursor order equals commit order.
    pub async fn ingest(
        &self,
        context: &AccountContext,
        mutation: &EvidenceMutation,
    ) -> Result<AddressedRecordRevision, ApiError> {
        self.ingest_with_authority(context, &IngestAuthority::Device, mutation)
            .await
    }
    /// In-process entry point for server-owned writers (portal connectors).
    /// Never exposed through an HTTP route.
    pub async fn ingest_with_authority(
        &self,
        context: &AccountContext,
        authority: &IngestAuthority,
        mutation: &EvidenceMutation,
    ) -> Result<AddressedRecordRevision, ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let address = apply(&mut tx, context.account_id, authority, mutation).await?;
        tx.commit().await?;
        Ok(address)
    }
    /// Device batch upload; see [`EvidenceStore::ingest_batch_with_authority`].
    pub async fn ingest_batch(
        &self,
        context: &AccountContext,
        batch: &EvidenceBatch,
    ) -> Result<EvidenceBatchResult, ApiError> {
        self.ingest_batch_with_authority(context, &IngestAuthority::Device, batch)
            .await
    }
    /// Apply each item in its own savepoint inside one account transaction. A
    /// refused item rolls back alone and is reported; infrastructure failure
    /// aborts the whole batch so no acknowledgement is ambiguous.
    pub async fn ingest_batch_with_authority(
        &self,
        context: &AccountContext,
        authority: &IngestAuthority,
        batch: &EvidenceBatch,
    ) -> Result<EvidenceBatchResult, ApiError> {
        if batch.items.is_empty() || batch.items.len() > MAX_EVIDENCE_BATCH_ITEMS {
            return Err(ApiError::Unprocessable(
                "evidence batch must hold 1 to 100 items",
            ));
        }
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let mut results = Vec::with_capacity(batch.items.len());
        for (index, item) in batch.items.iter().enumerate() {
            let mut savepoint = tx.begin().await?;
            match apply(&mut savepoint, context.account_id, authority, item).await {
                Ok(address) => {
                    savepoint.commit().await?;
                    results.push(EvidenceItemOutcome::Accepted {
                        index,
                        operation_id: item.operation_id,
                        address,
                    });
                }
                Err(error) => {
                    savepoint.rollback().await?;
                    results.push(EvidenceItemOutcome::Rejected {
                        index,
                        operation_id: item.operation_id,
                        rejection: rejection(error)?,
                    });
                }
            }
        }
        tx.commit().await?;
        Ok(EvidenceBatchResult { results })
    }
    /// Read exactly the addressed revision, optionally requiring that it remains current.
    pub async fn exact(
        &self,
        context: &AccountContext,
        address: &AddressedRecordRevision,
        require_current: bool,
    ) -> Result<ExactEvidence, ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        exact_in_transaction(&mut tx, context.account_id, address, require_current).await
    }
    /// Account-consistent library/snapshot/delta; no relevance filtering and no mutable hash shortcut.
    pub async fn snapshot(
        &self,
        context: &AccountContext,
        after: i64,
    ) -> Result<EvidenceSnapshot, ApiError> {
        Ok(self.collect(context, after, false).await?.0)
    }
    /// Complete browse inventory: the current snapshot plus, from the same account
    /// transaction, the last retained revision of every source-removed record
    /// (`current=false`). Privacy-erased records are absent from both.
    pub async fn inventory(
        &self,
        context: &AccountContext,
    ) -> Result<(EvidenceSnapshot, Vec<ExactEvidence>), ApiError> {
        self.collect(context, 0, true).await
    }
    async fn collect(
        &self,
        context: &AccountContext,
        after: i64,
        include_removed: bool,
    ) -> Result<(EvidenceSnapshot, Vec<ExactEvidence>), ApiError> {
        let mut removed = Vec::new();
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let fences = account_fences(&mut tx, context.account_id).await?;
        if after < 0 || after > fences.evidence_revision {
            return Err(ApiError::Conflict("evidence cursor is not current"));
        }
        let members: Vec<membership::Model> = orm::models(
            &mut tx,
            &membership::Entity::find()
                .filter(membership::Column::UserId.eq(context.account_id))
                .filter(membership::Column::Revision.gt(after))
                .order_by_asc(membership::Column::Revision)
                .order_by_asc(membership::Column::EntityKind)
                .order_by_asc(membership::Column::RecordId)
                .into_query(),
        )
        .await?;
        let mut changes = Vec::new();
        let mut records = Vec::new();
        for member in members {
            let entity = if member.entity_kind == "clinical_note" {
                Entity::ClinicalNote
            } else {
                Entity::HealthEvidence
            };
            if !member.deleted {
                let latest = latest_revision(&mut tx, context.account_id, member.record_id)
                    .await?
                    .ok_or(ApiError::Storage("current member lacks a revision".into()))?;
                records.push(ExactEvidence {
                    address: AddressedRecordRevision::new(
                        member.record_id,
                        entity,
                        member.client_record_id.clone(),
                        wire_revision(latest.revision)?,
                    ),
                    payload: latest.payload,
                    current: true,
                });
            } else if include_removed
                && member.source_removed
                && let Some(latest) =
                    latest_revision(&mut tx, context.account_id, member.record_id).await?
            {
                removed.push(ExactEvidence {
                    address: AddressedRecordRevision::new(
                        member.record_id,
                        entity,
                        member.client_record_id.clone(),
                        wire_revision(latest.revision)?,
                    ),
                    payload: latest.payload,
                    current: false,
                });
            }
            changes.push(EvidenceChange {
                entity,
                canonical_record_id: member.record_id,
                record_id: member.client_record_id,
                revision: member.revision,
                deleted: member.deleted,
                source_removed: member.source_removed,
            });
        }
        tx.commit().await?;
        Ok((
            EvidenceSnapshot {
                cursor: fences.evidence_revision,
                health_records_generation: fences.health_records_generation,
                changes,
                records,
            },
            removed,
        ))
    }
    /// Exact revision with its truthful status; see [`revision_view_in_transaction`].
    pub async fn revision_view(
        &self,
        context: &AccountContext,
        address: &AddressedRecordRevision,
    ) -> Result<Value, ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        revision_view_in_transaction(&mut tx, context.account_id, address).await
    }
}

/// `EvidenceRevisionView` for one exact address: the addressed payload, and whether
/// it is `current`, `superseded` by a later correction, or `removed_by_source`.
/// Privacy-erased, withdrawn, foreign or absent addresses are `NotFound`; callers that
/// already proved ownership (a retained citation) render that as `unavailable`.
pub async fn revision_view_in_transaction(
    tx: &mut Transaction<'_, Postgres>,
    account: Uuid,
    address: &AddressedRecordRevision,
) -> Result<Value, ApiError> {
    let exact = exact_in_transaction(tx, account, address, false).await?;
    let removed = orm::model(
        tx,
        &membership::Entity::find()
            .filter(membership::Column::UserId.eq(account))
            .filter(membership::Column::RecordId.eq(address.canonical_record_id))
            .into_query(),
    )
    .await?
    .is_some_and(|m: membership::Model| m.source_removed);
    let latest = latest_revision(tx, account, address.canonical_record_id)
        .await?
        .map(|r| r.revision);
    let status = if exact.current {
        "current"
    } else if removed {
        "removed_by_source"
    } else {
        "superseded"
    };
    Ok(serde_json::json!({
        "address": exact.address,
        "status": status,
        "current": exact.current,
        "newer_revision_exists": latest.is_some_and(|l| l > i64::from(address.revision)),
        "current_revision": latest,
        "payload": exact.payload,
    }))
}

/// `EvidenceRevisionView` for a retained citation whose record was privacy-erased.
pub fn unavailable_view(address: &AddressedRecordRevision) -> Value {
    serde_json::json!({"address": address, "status": "unavailable", "current": false, "payload": null})
}

async fn account_fences(
    tx: &mut Transaction<'_, Postgres>,
    account_id: Uuid,
) -> Result<account::Model, ApiError> {
    orm::model(tx, &account::Entity::find_by_id(account_id).into_query())
        .await?
        .ok_or(ApiError::AbsentAccount)
}
async fn latest_revision(
    tx: &mut Transaction<'_, Postgres>,
    account_id: Uuid,
    record: Uuid,
) -> Result<Option<revision::Model>, ApiError> {
    Ok(orm::model(
        tx,
        &revision::Entity::find()
            .filter(revision::Column::UserId.eq(account_id))
            .filter(
                Condition::any()
                    .add(revision::Column::EvidenceId.eq(record))
                    .add(revision::Column::NoteId.eq(record)),
            )
            .order_by_desc(revision::Column::Revision)
            .limit(1)
            .into_query(),
    )
    .await?)
}

/// Fields of an authorized upsert, parsed before any write.
enum Prepared {
    Health {
        concept: String,
        source_kind: String,
        connection: Option<String>,
        source_record: Option<String>,
        measured_at: Option<DateTime<Utc>>,
        existing: bool,
    },
    Note {
        label: String,
        recorded_at: Option<DateTime<Utc>>,
        existing: bool,
    },
}

/// Apply one mutation inside the caller's account transaction (account already locked).
async fn apply(
    tx: &mut Transaction<'_, Postgres>,
    account_id: Uuid,
    authority: &IngestAuthority,
    mutation: &EvidenceMutation,
) -> Result<AddressedRecordRevision, ApiError> {
    if let IngestAuthority::Connector {
        source_kind,
        connection_id,
    } = authority
    {
        IngestAuthority::connector(source_kind, connection_id.clone())?;
    }
    let payload_bytes = serde_json::to_vec(&mutation.payload)
        .map_err(|_| ApiError::Unprocessable("invalid evidence"))?;
    if !mutation.payload.is_object() || payload_bytes.len() > MAX_EVIDENCE_ITEM_BYTES {
        return Err(ApiError::Unprocessable(
            "evidence must be a bounded source object",
        ));
    }
    let request = match authority {
        IngestAuthority::Device => serde_json::to_vec(mutation),
        IngestAuthority::Connector { .. } => {
            serde_json::to_vec(&serde_json::json!({"authority": authority, "mutation": mutation}))
        }
    }
    .map_err(|_| ApiError::Unprocessable("invalid evidence"))?;
    let fingerprint = hash(&request);
    if let Some(prior) = orm::model::<ingestion::Model>(
        tx,
        &ingestion::Entity::find_by_id((account_id, mutation.operation_id)).into_query(),
    )
    .await?
    {
        if prior.request_sha256 != fingerprint {
            return Err(ApiError::Conflict(
                "ingestion identity reused with different content",
            ));
        }
        return serde_json::from_value(prior.address)
            .map_err(|_| ApiError::Storage("invalid stored address".into()));
    }
    fence_generation(tx, account_id, authority, mutation).await?;
    let member: Option<membership::Model> = orm::model(
        tx,
        &membership::Entity::find()
            .filter(membership::Column::UserId.eq(account_id))
            .filter(membership::Column::RecordId.eq(mutation.canonical_record_id))
            .into_query(),
    )
    .await?;
    if let Some(member) = &member
        && (member.entity_kind != entity_name(mutation.entity)
            || member.client_record_id != mutation.record_id)
    {
        return Err(ApiError::Conflict(
            "source identity was withdrawn or differs",
        ));
    }
    let latest = latest_revision(tx, account_id, mutation.canonical_record_id).await?;
    let address = match mutation.operation {
        EvidenceOperation::Upsert => {
            upsert(
                tx,
                account_id,
                authority,
                mutation,
                member,
                latest,
                &payload_bytes,
            )
            .await?
        }
        EvidenceOperation::Delete => {
            remove(tx, account_id, authority, mutation, member, latest).await?
        }
    };
    orm::execute(
        tx,
        &ingestion::Entity::insert(ingestion::ActiveModel {
            user_id: Set(account_id),
            operation_id: Set(mutation.operation_id),
            request_sha256: Set(fingerprint),
            address: Set(serde_json::to_value(&address)
                .map_err(|_| ApiError::Unprocessable("invalid address"))?),
        })
        .into_query(),
    )
    .await
    .map_err(|error| match unique_conflict(error) {
        ApiError::Conflict(_) => ApiError::Conflict("ingestion identity reused concurrently"),
        other => other,
    })?;
    Ok(address)
}

/// Apple Health and Health Records uploads must match the current source generation.
async fn fence_generation(
    tx: &mut Transaction<'_, Postgres>,
    account_id: Uuid,
    authority: &IngestAuthority,
    mutation: &EvidenceMutation,
) -> Result<(), ApiError> {
    if *authority != IngestAuthority::Device {
        return Ok(());
    }
    let (kind, _) = source_of(&mutation.payload);
    let health_records_note = mutation.entity == Entity::ClinicalNote
        && mutation.operation == EvidenceOperation::Upsert
        && mutation
            .payload
            .get("source_label")
            .is_none_or(|label| label == "Health Records");
    let fences = account_fences(tx, account_id).await?;
    let generation = if kind == Some("appleHealth") {
        fences.apple_health_generation
    } else if kind == Some("healthRecords") || health_records_note {
        fences.health_records_generation
    } else {
        return Ok(());
    };
    if mutation.source_generation.unwrap_or(0) != generation {
        return Err(ApiError::Conflict(
            "source generation changed; reconnect before upload",
        ));
    }
    Ok(())
}

async fn existing_health(
    tx: &mut Transaction<'_, Postgres>,
    account_id: Uuid,
    id: Uuid,
) -> Result<Option<health_evidence::Model>, ApiError> {
    Ok(orm::model(
        tx,
        &health_evidence::Entity::find_by_id(id)
            .filter(health_evidence::Column::UserId.eq(account_id))
            .into_query(),
    )
    .await?)
}
async fn existing_note(
    tx: &mut Transaction<'_, Postgres>,
    account_id: Uuid,
    id: Uuid,
) -> Result<Option<clinical_note::Model>, ApiError> {
    Ok(orm::model(
        tx,
        &clinical_note::Entity::find_by_id(id)
            .filter(clinical_note::Column::UserId.eq(account_id))
            .into_query(),
    )
    .await?)
}
/// Whether `authority` owns a stored note. Device notes carry a device label;
/// connector notes carry the connector's source kind and connection.
fn owns_note(authority: &IngestAuthority, note: &clinical_note::Model) -> bool {
    match authority {
        IngestAuthority::Device => DEVICE_NOTE_LABELS.contains(&note.source_label.as_str()),
        IngestAuthority::Connector {
            source_kind,
            connection_id,
        } => note.duet_source_payload.as_ref().is_some_and(|payload| {
            source_of(payload) == (Some(source_kind.as_str()), Some(connection_id.as_str()))
        }),
    }
}

async fn prepare(
    tx: &mut Transaction<'_, Postgres>,
    account_id: Uuid,
    authority: &IngestAuthority,
    mutation: &EvidenceMutation,
) -> Result<Prepared, ApiError> {
    let p = &mutation.payload;
    let (kind, connection) = source_of(p);
    match mutation.entity {
        Entity::HealthEvidence => {
            if !p.get("source").is_some_and(Value::is_object) {
                return Err(ApiError::Unprocessable("source provenance required"));
            }
            let kind = kind.ok_or(ApiError::Unprocessable("source kind required"))?;
            match authority {
                IngestAuthority::Device => {
                    if SERVER_SOURCE_KINDS.contains(&kind) {
                        return Err(refused());
                    }
                    if !DEVICE_SOURCE_KINDS.contains(&kind) {
                        return Err(ApiError::Unprocessable("invalid source kind"));
                    }
                }
                IngestAuthority::Connector {
                    source_kind,
                    connection_id,
                } => {
                    if kind != source_kind || connection != Some(connection_id.as_str()) {
                        return Err(refused());
                    }
                }
            }
            let existing = existing_health(tx, account_id, mutation.canonical_record_id).await?;
            if let Some(existing) = &existing {
                // Source kind and connection are fixed at creation: no takeover, no revival
                // under another writer.
                if existing.source_kind != kind
                    || existing.source_connection_id.as_deref() != connection
                {
                    return Err(refused());
                }
                // An older client cannot replace a richer retained source object.
                if existing
                    .payload
                    .get("source_record")
                    .is_some_and(Value::is_object)
                    && !p.get("source_record").is_some_and(Value::is_object)
                {
                    return Err(ApiError::Conflict("source-preserving client required"));
                }
            }
            let concept = p["concept_id"]
                .as_str()
                .filter(|concept| !concept.trim().is_empty())
                .ok_or(ApiError::Unprocessable("concept identity required"))?;
            let measured_at = declared_instant(
                p.get("measured_at")
                    .filter(|value| !value.is_null())
                    .or_else(|| p.pointer("/measured_interval/start")),
            )?;
            Ok(Prepared::Health {
                concept: concept.to_owned(),
                source_kind: kind.to_owned(),
                connection: connection.map(str::to_owned),
                source_record: p
                    .pointer("/source/record_id")
                    .and_then(Value::as_str)
                    .map(str::to_owned),
                measured_at,
                existing: existing.is_some(),
            })
        }
        Entity::ClinicalNote => {
            let label = match (authority, p.get("source_label")) {
                (IngestAuthority::Device, None | Some(Value::Null)) => "Health Records",
                (_, Some(Value::String(label))) if !label.trim().is_empty() => label.as_str(),
                _ => {
                    return Err(ApiError::Unprocessable(
                        "clinical note requires a source label",
                    ));
                }
            };
            match authority {
                IngestAuthority::Device => {
                    if !DEVICE_NOTE_LABELS.contains(&label)
                        || kind.is_some_and(|kind| SERVER_SOURCE_KINDS.contains(&kind))
                    {
                        return Err(refused());
                    }
                }
                IngestAuthority::Connector {
                    source_kind,
                    connection_id,
                } => {
                    if DEVICE_NOTE_LABELS.contains(&label) {
                        return Err(ApiError::Unprocessable(
                            "connector notes carry their provider label",
                        ));
                    }
                    if kind != Some(source_kind.as_str())
                        || connection != Some(connection_id.as_str())
                    {
                        return Err(refused());
                    }
                }
            }
            let existing = existing_note(tx, account_id, mutation.canonical_record_id).await?;
            if let Some(existing) = &existing
                && (existing.source_label != label || !owns_note(authority, existing))
            {
                return Err(refused());
            }
            let recorded_at = declared_instant(p.get("recorded_at"))?;
            if label == PATIENT_NOTE_LABEL && recorded_at.is_none() {
                return Err(ApiError::Unprocessable(
                    "patient-authored notes require an explicit recorded_at instant",
                ));
            }
            Ok(Prepared::Note {
                label: label.to_owned(),
                recorded_at,
                existing: existing.is_some(),
            })
        }
    }
}

async fn upsert(
    tx: &mut Transaction<'_, Postgres>,
    account_id: Uuid,
    authority: &IngestAuthority,
    mutation: &EvidenceMutation,
    member: Option<membership::Model>,
    latest: Option<revision::Model>,
    payload_bytes: &[u8],
) -> Result<AddressedRecordRevision, ApiError> {
    if member.as_ref().is_some_and(|member| member.deleted) {
        return Err(ApiError::Conflict(
            "source identity was withdrawn or differs",
        ));
    }
    let prepared = prepare(tx, account_id, authority, mutation).await?;
    let content_sha256 = hash(payload_bytes);
    let address = |revision: i64| -> Result<AddressedRecordRevision, ApiError> {
        Ok(AddressedRecordRevision::new(
            mutation.canonical_record_id,
            mutation.entity,
            mutation.record_id.clone(),
            wire_revision(revision)?,
        ))
    };
    if latest.as_ref().map(|latest| latest.revision) != mutation.expected_revision {
        return Err(ApiError::Conflict("source revision changed"));
    }
    let now = Utc::now();
    let p = &mutation.payload;
    match prepared {
        Prepared::Health {
            concept,
            source_kind,
            connection,
            source_record,
            measured_at,
            existing,
        } => {
            if existing {
                orm::execute(
                    tx,
                    &health_evidence::Entity::update_many()
                        .col_expr(health_evidence::Column::ConceptId, Expr::val(concept))
                        .col_expr(
                            health_evidence::Column::SourceRecordId,
                            Expr::val(source_record),
                        )
                        .col_expr(health_evidence::Column::MeasuredAt, Expr::val(measured_at))
                        .col_expr(health_evidence::Column::ReceivedAt, Expr::val(now))
                        .col_expr(health_evidence::Column::Payload, Expr::val(p.clone()))
                        .col_expr(health_evidence::Column::UpdatedAt, Expr::val(now))
                        .filter(health_evidence::Column::Id.eq(mutation.canonical_record_id))
                        .filter(health_evidence::Column::UserId.eq(account_id))
                        .into_query(),
                )
                .await?;
            } else {
                orm::execute(
                    tx,
                    &health_evidence::Entity::insert(health_evidence::ActiveModel {
                        id: Set(mutation.canonical_record_id),
                        user_id: Set(account_id),
                        concept_id: Set(concept),
                        source_kind: Set(source_kind),
                        source_connection_id: Set(connection),
                        source_record_id: Set(source_record),
                        measured_at: Set(measured_at),
                        received_at: Set(now),
                        payload: Set(p.clone()),
                        updated_at: Set(now),
                    })
                    .into_query(),
                )
                .await
                .map_err(unique_conflict)?;
            }
        }
        Prepared::Note {
            label,
            recorded_at,
            existing,
        } => {
            let text = |key: &str| p[key].as_str().map(str::to_owned);
            if existing {
                orm::execute(
                    tx,
                    &clinical_note::Entity::update_many()
                        .col_expr(
                            clinical_note::Column::SourceId,
                            Expr::val(text("source_id")),
                        )
                        .col_expr(
                            clinical_note::Column::Title,
                            Expr::val(text("title").unwrap_or_default()),
                        )
                        .col_expr(clinical_note::Column::Preview, Expr::val(text("preview")))
                        .col_expr(clinical_note::Column::Body, Expr::val(text("body")))
                        .col_expr(clinical_note::Column::RecordedAt, Expr::val(recorded_at))
                        .col_expr(
                            clinical_note::Column::DuetSourcePayload,
                            Expr::val(p.clone()),
                        )
                        .filter(clinical_note::Column::Id.eq(mutation.canonical_record_id))
                        .filter(clinical_note::Column::UserId.eq(account_id))
                        .into_query(),
                )
                .await?;
            } else {
                orm::execute(
                    tx,
                    &clinical_note::Entity::insert(clinical_note::ActiveModel {
                        id: Set(mutation.canonical_record_id),
                        user_id: Set(account_id),
                        source_id: Set(text("source_id")),
                        title: Set(text("title").unwrap_or_default()),
                        preview: Set(text("preview")),
                        body: Set(text("body")),
                        recorded_at: Set(recorded_at),
                        source_label: Set(label),
                        duet_source_payload: Set(Some(p.clone())),
                    })
                    .into_query(),
                )
                .await
                .map_err(unique_conflict)?;
            }
        }
    }
    let (evidence_id, note_id) = match mutation.entity {
        Entity::HealthEvidence => (Some(mutation.canonical_record_id), None),
        Entity::ClinicalNote => (None, Some(mutation.canonical_record_id)),
    };
    let mut insert = revision::Entity::insert(revision::ActiveModel {
        user_id: Set(account_id),
        evidence_id: Set(evidence_id),
        note_id: Set(note_id),
        payload: Set(p.clone()),
        content_sha256: Set(content_sha256),
        ..Default::default()
    })
    .into_query();
    insert.returning_col(revision::Column::Revision);
    let retained: i64 = sqlx::Row::try_get(
        &orm::row(tx, &insert)
            .await?
            .ok_or(ApiError::Storage("revision insert returned nothing".into()))?,
        0,
    )?;
    let cursor = advance_cursor(tx, account_id).await?;
    orm::execute(
        tx,
        &membership::Entity::insert(membership::ActiveModel {
            user_id: Set(account_id),
            entity_kind: Set(entity_name(mutation.entity).to_owned()),
            record_id: Set(mutation.canonical_record_id),
            client_record_id: Set(mutation.record_id.clone()),
            revision: Set(cursor),
            deleted: Set(false),
            source_removed: Set(false),
        })
        .on_conflict(
            OnConflict::columns([
                membership::Column::UserId,
                membership::Column::EntityKind,
                membership::Column::RecordId,
            ])
            .update_columns([
                membership::Column::Revision,
                membership::Column::Deleted,
                membership::Column::SourceRemoved,
            ])
            .to_owned(),
        )
        .into_query(),
    )
    .await?;
    address(retained)
}

/// Explicit source removal: keep the base row and retained revisions, withdraw
/// current membership, and advance the account cursor.
async fn remove(
    tx: &mut Transaction<'_, Postgres>,
    account_id: Uuid,
    authority: &IngestAuthority,
    mutation: &EvidenceMutation,
    member: Option<membership::Model>,
    latest: Option<revision::Model>,
) -> Result<AddressedRecordRevision, ApiError> {
    let p = &mutation.payload;
    let (kind, connection) = source_of(p);
    let source_keys_ok = p.as_object().is_some_and(|object| object.len() == 1)
        && p["source"].as_object().is_some_and(|source| {
            source
                .keys()
                .all(|key| key == "kind" || key == "connection_id")
        });
    let expected = mutation.expected_revision.ok_or(ApiError::Unprocessable(
        "source removal requires the exact expected revision",
    ))?;
    let kind = kind
        .filter(|_| source_keys_ok)
        .ok_or(ApiError::Unprocessable("invalid source removal"))?;
    match authority {
        IngestAuthority::Device => {
            if SERVER_SOURCE_KINDS.contains(&kind) {
                return Err(refused());
            }
            if !DEVICE_REMOVABLE_KINDS.contains(&kind)
                || (kind == "appleHealth" && mutation.entity != Entity::HealthEvidence)
            {
                return Err(ApiError::Unprocessable("invalid source removal"));
            }
        }
        IngestAuthority::Connector {
            source_kind,
            connection_id,
        } => {
            if kind != source_kind || connection != Some(connection_id.as_str()) {
                return Err(refused());
            }
        }
    }
    let member = member
        .filter(|member| !member.deleted)
        .ok_or(ApiError::NotFound)?;
    match mutation.entity {
        Entity::HealthEvidence => {
            let existing = existing_health(tx, account_id, mutation.canonical_record_id)
                .await?
                .ok_or(ApiError::NotFound)?;
            let owner_connection = match authority {
                IngestAuthority::Device => existing.source_connection_id.as_deref(),
                IngestAuthority::Connector { .. } => connection,
            };
            if existing.source_kind != kind
                || existing.source_connection_id.as_deref() != owner_connection
            {
                return Err(refused());
            }
        }
        Entity::ClinicalNote => {
            let existing = existing_note(tx, account_id, mutation.canonical_record_id)
                .await?
                .ok_or(ApiError::NotFound)?;
            let owned = match authority {
                IngestAuthority::Device => existing.source_label == "Health Records",
                IngestAuthority::Connector { .. } => owns_note(authority, &existing),
            };
            if !owned {
                return Err(refused());
            }
        }
    }
    let latest = latest.ok_or(ApiError::NotFound)?;
    if latest.revision != expected {
        return Err(ApiError::Conflict("source revision changed"));
    }
    let cursor = advance_cursor(tx, account_id).await?;
    orm::execute(
        tx,
        &membership::Entity::update_many()
            .col_expr(membership::Column::Revision, Expr::val(cursor))
            .col_expr(membership::Column::Deleted, Expr::val(true))
            .col_expr(membership::Column::SourceRemoved, Expr::val(true))
            .filter(membership::Column::UserId.eq(account_id))
            .filter(membership::Column::EntityKind.eq(member.entity_kind))
            .filter(membership::Column::RecordId.eq(member.record_id))
            .into_query(),
    )
    .await?;
    Ok(AddressedRecordRevision::new(
        mutation.canonical_record_id,
        mutation.entity,
        mutation.record_id.clone(),
        wire_revision(latest.revision)?,
    ))
}

async fn advance_cursor(
    tx: &mut Transaction<'_, Postgres>,
    account_id: Uuid,
) -> Result<i64, ApiError> {
    let mut update = account::Entity::update_many()
        .col_expr(
            account::Column::EvidenceRevision,
            Expr::col(account::Column::EvidenceRevision).add(1),
        )
        .filter(account::Column::Id.eq(account_id))
        .into_query();
    update.returning_col(account::Column::EvidenceRevision);
    Ok(sqlx::Row::try_get(
        &orm::row(tx, &update)
            .await?
            .ok_or(ApiError::AbsentAccount)?,
        0,
    )?)
}

/// Exact/currentness hook for workspace save, publication validation and Luna grounding.
/// Caller must first hold `identity::check_current`'s account lock in this same transaction.
///
/// A source-removed record's retained revisions stay readable with `current=false`;
/// `require_current` then fails with a conflict. A privacy-erased or withdrawn
/// record is not found.
pub async fn exact_in_transaction(
    tx: &mut Transaction<'_, Postgres>,
    account: Uuid,
    address: &AddressedRecordRevision,
    require_current: bool,
) -> Result<ExactEvidence, ApiError> {
    let member: membership::Model = orm::model(
        tx,
        &membership::Entity::find()
            .filter(membership::Column::UserId.eq(account))
            .filter(membership::Column::EntityKind.eq(entity_name(address.entity)))
            .filter(membership::Column::RecordId.eq(address.canonical_record_id))
            .filter(membership::Column::ClientRecordId.eq(address.record_id.clone()))
            .into_query(),
    )
    .await?
    .ok_or(ApiError::NotFound)?;
    if member.deleted && !member.source_removed {
        return Err(ApiError::NotFound);
    }
    let family = match address.entity {
        Entity::HealthEvidence => revision::Column::EvidenceId,
        Entity::ClinicalNote => revision::Column::NoteId,
    };
    let retained: revision::Model = orm::model(
        tx,
        &revision::Entity::find()
            .filter(revision::Column::UserId.eq(account))
            .filter(revision::Column::Revision.eq(i64::from(address.revision)))
            .filter(family.eq(address.canonical_record_id))
            .into_query(),
    )
    .await?
    .ok_or(ApiError::NotFound)?;
    let latest = latest_revision(tx, account, address.canonical_record_id).await?;
    let current =
        !member.deleted && latest.is_some_and(|latest| latest.revision == retained.revision);
    if require_current && !current {
        return Err(ApiError::Conflict(if member.source_removed {
            "selected evidence was removed by its source"
        } else {
            "selected evidence was corrected"
        }));
    }
    Ok(ExactEvidence {
        address: address.clone(),
        payload: retained.payload,
        current,
    })
}
