//! Canonical source mutation and exact account-bound reads; never a search or summary substitute.
use crate::{
    ApiError,
    database::AccountDatabase,
    identity::{AccountContext, check_current},
};
use chrono::{DateTime, Utc};
use duet_wire::models::addressed_record_revision::Entity;
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
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::Row;
use uuid::Uuid;

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
    /// Exact source object, including provenance and declared date roles.
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
    /// Whether the record was withdrawn.
    pub deleted: bool,
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
impl EvidenceStore {
    /// Construct an evidence owner over the shared boundary.
    pub fn new(database: AccountDatabase) -> Self {
        Self { database }
    }
    /// Idempotent, optimistic source mutation. Locks the account so cursor order equals commit order.
    pub async fn ingest(
        &self,
        context: &AccountContext,
        mutation: &EvidenceMutation,
    ) -> Result<AddressedRecordRevision, ApiError> {
        if !mutation.payload.is_object()
            || serde_json::to_vec(&mutation.payload)
                .map_err(|_| ApiError::Unprocessable("invalid evidence"))?
                .len()
                > 2_000_000
        {
            return Err(ApiError::Unprocessable(
                "evidence must be a bounded source object",
            ));
        }
        let fingerprint = hash(
            &serde_json::to_vec(mutation)
                .map_err(|_| ApiError::Unprocessable("invalid evidence"))?,
        );
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        if let Some(row)=sqlx::query("select request_sha256,address from public.duet_evidence_ingestions where user_id=$1 and operation_id=$2").bind(context.account_id).bind(mutation.operation_id).fetch_optional(&mut *tx).await? {
            if row.get::<String,_>("request_sha256")!=fingerprint {return Err(ApiError::Conflict("ingestion identity reused with different content"));}
            return serde_json::from_value(row.get("address")).map_err(|_|ApiError::Storage("invalid stored address".into()));
        }
        let source_kind = mutation
            .payload
            .pointer("/source/kind")
            .and_then(Value::as_str);
        let generation_column = if source_kind == Some("appleHealth") {
            Some("apple_health_generation")
        } else if source_kind == Some("healthRecords")
            || (mutation.entity == Entity::ClinicalNote
                && mutation.payload["source_label"] == "Health Records")
        {
            Some("health_records_generation")
        } else {
            None
        };
        if let Some(column) = generation_column {
            let generation: i32 =
                sqlx::query_scalar(&format!("select {column} from users where id=$1"))
                    .bind(context.account_id)
                    .fetch_one(&mut *tx)
                    .await?;
            if mutation.source_generation.unwrap_or(0) != generation {
                return Err(ApiError::Conflict(
                    "source generation changed; reconnect before upload",
                ));
            }
        }
        let latest:Option<i64>=sqlx::query_scalar("select max(revision) from public.canonical_evidence_revisions where user_id=$1 and coalesce(evidence_id,note_id)=$2").bind(context.account_id).bind(mutation.canonical_record_id).fetch_one(&mut *tx).await?;
        if latest != mutation.expected_revision {
            return Err(ApiError::Conflict("source revision changed"));
        }
        let prior=sqlx::query("select entity,client_record_id,deleted from public.evidence_sync_changes where user_id=$1 and record_id=$2").bind(context.account_id).bind(mutation.canonical_record_id).fetch_optional(&mut *tx).await?;
        if let Some(prior) = prior
            && (prior.get::<String, _>("entity") != entity_name(mutation.entity)
                || prior.get::<String, _>("client_record_id") != mutation.record_id
                || prior.get::<bool, _>("deleted"))
        {
            return Err(ApiError::Conflict(
                "source identity was withdrawn or differs",
            ));
        }
        let p = &mutation.payload;
        match mutation.entity {
            Entity::HealthEvidence => {
                let source = p
                    .get("source")
                    .ok_or(ApiError::Unprocessable("source provenance required"))?;
                let source_kind = source["kind"]
                    .as_str()
                    .ok_or(ApiError::Unprocessable("source kind required"))?;
                if !["appleHealth", "healthRecords", "physical", "manual"].contains(&source_kind) {
                    return Err(ApiError::Unprocessable("invalid source kind"));
                }
                let measured: DateTime<Utc> = p
                    .get("measured_at")
                    .or_else(|| p.pointer("/measured_interval/start"))
                    .and_then(Value::as_str)
                    .ok_or(ApiError::Unprocessable("dated evidence required"))?
                    .parse()
                    .map_err(|_| ApiError::Unprocessable("explicit source timestamp required"))?;
                let concept = p["concept_id"]
                    .as_str()
                    .ok_or(ApiError::Unprocessable("concept identity required"))?;
                sqlx::query("insert into public.health_evidence(id,user_id,concept_id,source_kind,source_connection_id,source_record_id,measured_at,received_at,payload) values($1,$2,$3,$4,$5,$6,$7,now(),$8) on conflict(id) do update set concept_id=excluded.concept_id,source_kind=excluded.source_kind,source_connection_id=excluded.source_connection_id,source_record_id=excluded.source_record_id,measured_at=excluded.measured_at,received_at=now(),payload=excluded.payload,updated_at=now() where health_evidence.user_id=excluded.user_id")
                    .bind(mutation.canonical_record_id).bind(context.account_id).bind(concept).bind(source_kind).bind(source["connection_id"].as_str()).bind(source["record_id"].as_str()).bind(measured).bind(p).execute(&mut *tx).await?;
            }
            Entity::ClinicalNote => {
                sqlx::query("insert into public.clinical_notes(id,user_id,source_id,title,preview,body,source_label,duet_source_payload) values($1,$2,$3,$4,$5,$6,$7,$8) on conflict(id) do update set source_id=excluded.source_id,title=excluded.title,preview=excluded.preview,body=excluded.body,source_label=excluded.source_label,duet_source_payload=excluded.duet_source_payload where clinical_notes.user_id=excluded.user_id")
                    .bind(mutation.canonical_record_id).bind(context.account_id).bind(p["source_id"].as_str()).bind(p["title"].as_str().unwrap_or("")).bind(p["preview"].as_str()).bind(p["body"].as_str()).bind(p["source_label"].as_str().unwrap_or("Health Records")).bind(p).execute(&mut *tx).await?;
            }
        }
        let (evidence_id, note_id) = match mutation.entity {
            Entity::HealthEvidence => (Some(mutation.canonical_record_id), None),
            Entity::ClinicalNote => (None, Some(mutation.canonical_record_id)),
        };
        let revision:i64=sqlx::query_scalar("insert into public.canonical_evidence_revisions(user_id,evidence_id,note_id,payload,content_sha256) values($1,$2,$3,$4,$5) returning revision")
            .bind(context.account_id).bind(evidence_id).bind(note_id).bind(p).bind(hash(&serde_json::to_vec(p).map_err(|_|ApiError::Unprocessable("invalid source object"))?)).fetch_one(&mut *tx).await?;
        let cursor:i64=sqlx::query_scalar("update public.users set evidence_revision=evidence_revision+1 where id=$1 returning evidence_revision").bind(context.account_id).fetch_one(&mut *tx).await?;
        sqlx::query("insert into public.evidence_sync_changes(user_id,entity,record_id,client_record_id,revision,deleted) values($1,$2,$3,$4,$5,false) on conflict(user_id,entity,record_id) do update set revision=excluded.revision,deleted=false")
            .bind(context.account_id).bind(entity_name(mutation.entity)).bind(mutation.canonical_record_id).bind(mutation.record_id.to_string()).bind(cursor).execute(&mut *tx).await?;
        let address = AddressedRecordRevision::new(
            mutation.canonical_record_id,
            mutation.entity,
            mutation.record_id.clone(),
            i32::try_from(revision)
                .map_err(|_| ApiError::Unavailable("revision exceeds wire range"))?,
        );
        sqlx::query("insert into public.duet_evidence_ingestions(user_id,operation_id,request_sha256,address) values($1,$2,$3,$4)").bind(context.account_id).bind(mutation.operation_id).bind(fingerprint).bind(serde_json::to_value(&address).map_err(|_|ApiError::Unprocessable("invalid address"))?).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(address)
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
        let mut tx = self.database.begin(context.account_id).await?;
        check_current(&mut tx, context, false).await?;
        let cursor: i64 =
            sqlx::query_scalar("select evidence_revision from public.users where id=$1")
                .bind(context.account_id)
                .fetch_one(&mut *tx)
                .await?;
        if after < 0 || after > cursor {
            return Err(ApiError::Conflict("evidence cursor is not current"));
        }
        let rows=sqlx::query("select entity,record_id,client_record_id,revision,deleted from public.evidence_sync_changes where user_id=$1 and revision>$2 order by revision,entity,record_id").bind(context.account_id).bind(after).fetch_all(&mut *tx).await?;
        let mut changes = Vec::new();
        let mut records = Vec::new();
        for row in rows {
            let entity = if row.get::<String, _>("entity") == "clinical_note" {
                Entity::ClinicalNote
            } else {
                Entity::HealthEvidence
            };
            let id: Uuid = row.get("record_id");
            let original: String = row.get("client_record_id");
            let deleted: bool = row.get("deleted");
            changes.push(EvidenceChange {
                entity,
                canonical_record_id: id,
                record_id: original.clone(),
                revision: row.get("revision"),
                deleted,
            });
            if !deleted {
                let revision:i64=sqlx::query_scalar("select max(revision) from public.canonical_evidence_revisions where user_id=$1 and coalesce(evidence_id,note_id)=$2").bind(context.account_id).bind(id).fetch_one(&mut *tx).await?;
                let address = AddressedRecordRevision::new(
                    id,
                    entity,
                    original,
                    i32::try_from(revision)
                        .map_err(|_| ApiError::Unavailable("revision exceeds wire range"))?,
                );
                records
                    .push(exact_in_transaction(&mut tx, context.account_id, &address, true).await?);
            }
        }
        let health_records_generation: i32 =
            sqlx::query_scalar("select health_records_generation from users where id=$1")
                .bind(context.account_id)
                .fetch_one(&mut *tx)
                .await?;
        tx.commit().await?;
        Ok(EvidenceSnapshot {
            cursor,
            health_records_generation,
            changes,
            records,
        })
    }
}
/// Exact/currentness hook for workspace save, publication validation and Luna grounding.
/// Caller must first hold `identity::check_current`'s account lock in this same transaction.
pub async fn exact_in_transaction(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    account: Uuid,
    address: &AddressedRecordRevision,
    require_current: bool,
) -> Result<ExactEvidence, ApiError> {
    let row=sqlx::query("select r.payload,r.revision=(select max(newer.revision) from public.canonical_evidence_revisions newer where newer.user_id=r.user_id and coalesce(newer.evidence_id,newer.note_id)=coalesce(r.evidence_id,r.note_id)) as current from public.canonical_evidence_revisions r join public.evidence_sync_changes m on m.user_id=r.user_id and m.record_id=coalesce(r.evidence_id,r.note_id) and not m.deleted where r.user_id=$1 and r.revision=$2 and coalesce(r.evidence_id,r.note_id)=$3 and m.entity=$4 and m.client_record_id=$5 and (($4='clinical_note' and r.note_id is not null) or ($4='health_evidence' and r.evidence_id is not null))")
        .bind(account).bind(i64::from(address.revision)).bind(address.canonical_record_id).bind(entity_name(address.entity)).bind(address.record_id.to_string()).fetch_optional(&mut **tx).await?.ok_or(ApiError::NotFound)?;
    let current: bool = row.get("current");
    if require_current && !current {
        return Err(ApiError::Conflict("selected evidence was corrected"));
    }
    Ok(ExactEvidence {
        address: address.clone(),
        payload: row.get("payload"),
        current,
    })
}
