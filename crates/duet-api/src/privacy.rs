//! Restriction + SQL outbox, independent create-only journal, and fenced restore replay.
use crate::{
    ApiError,
    database::AccountDatabase,
    identity::{AccountContext, check_current},
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::{path::PathBuf, sync::Arc};
use uuid::Uuid;

/// Metadata-only restriction operations. Patient prose and bearer credentials cannot enter receipts.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Restriction {
    /// Disable Luna and invalidate accepted work.
    LunaWithdraw,
    /// Fence every session authenticated at or before the receipt timestamp.
    SessionsRevoke,
    /// Withdraw one canonical record, including its retained historical content.
    EvidenceDelete,
    /// Clear the approved page and selected items through its generation.
    DoctorPageDelete,
    /// Clear only the patient's concern through its workspace generation.
    DoctorNoteDelete,
    /// Remove one stable selected-item incarnation.
    DoctorItemDelete,
    /// Revoke one pinned publication incarnation.
    PublicationRevoke,
    /// Withdraw all device snapshots currently held by this account.
    DeviceWithdraw,
    /// Withdraw imported Health Records through the account evidence cursor.
    RecordsWithdraw,
    /// Pause accepted follow-through without granting later execution authority.
    FollowUpPause,
    /// Permanently cancel one accepted follow-up incarnation.
    FollowUpCancel,
    /// Disable proactive work until fresh patient authorization after restore.
    ProactiveWithdraw,
    /// Remove product account state and block stale subject reprovisioning.
    AccountDelete,
}
/// Immutable recovery receipt: only identifiers, revisions and times.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    /// Wire schema version.
    pub schema_version: u32,
    /// Stable caller operation identity.
    pub event_id: Uuid,
    /// Existing account identity.
    pub account_id: Uuid,
    /// Account incarnation, preventing replay into a recreated account.
    pub account_incarnation: Uuid,
    /// Restriction kind.
    pub operation: Restriction,
    /// Account or canonical resource identity.
    pub scope_id: Uuid,
    /// Permission/resource revision being restricted.
    pub scope_version: i64,
    /// Original server time, unchanged on retries.
    pub occurred_at: DateTime<Utc>,
}
/// An independent store must implement create-only writes and authoritative byte readback.
#[async_trait]
pub trait ReceiptJournal: Send + Sync {
    /// Never replace an existing event, even when bytes differ. A write may have an uncertain outcome.
    async fn put_if_absent(&self, id: Uuid, bytes: &[u8]) -> Result<(), ApiError>;
    /// Independent read; callers compare the complete bytes, not only existence or a mutable digest.
    async fn read(&self, id: Uuid) -> Result<Option<Vec<u8>>, ApiError>;
    /// Enumerate the independent complete inventory before restored serving is admitted.
    /// Unsupported adapters fail closed rather than trusting the restored SQL outbox.
    async fn inventory(&self) -> Result<Vec<Uuid>, ApiError> {
        Err(unavailable())
    }
}
/// Durable local journal adapter for isolated service proof and separately mounted receipt volumes.
/// Production operators must provide independent retention/backup protection; this is not a GCS WORM claim.
pub struct FileJournal {
    root: PathBuf,
}
impl FileJournal {
    /// Open/create a dedicated private receipt directory, never the database data directory.
    pub async fn open(root: PathBuf) -> Result<Self, ApiError> {
        tokio::fs::create_dir_all(&root)
            .await
            .map_err(|_| unavailable())?;
        Ok(Self { root })
    }
}
fn unavailable() -> ApiError {
    ApiError::Unavailable("Privacy restriction is effective; receipt confirmation is pending")
}
#[async_trait]
impl ReceiptJournal for FileJournal {
    async fn inventory(&self) -> Result<Vec<Uuid>, ApiError> {
        let mut entries = tokio::fs::read_dir(&self.root)
            .await
            .map_err(|_| unavailable())?;
        let mut ids = Vec::new();
        while let Some(entry) = entries.next_entry().await.map_err(|_| unavailable())? {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with('.') {
                continue;
            }
            let id = name
                .strip_suffix(".json")
                .and_then(|s| s.parse::<Uuid>().ok())
                .ok_or_else(unavailable)?;
            ids.push(id);
        }
        Ok(ids)
    }
    async fn put_if_absent(&self, id: Uuid, bytes: &[u8]) -> Result<(), ApiError> {
        use tokio::io::AsyncWriteExt;
        if bytes.len() > 4096 {
            return Err(unavailable());
        }
        let temporary = self.root.join(format!(".{}-{}", id, Uuid::new_v4()));
        let target = self.root.join(format!("{id}.json"));
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&temporary)
            .await
            .map_err(|_| unavailable())?;
        file.write_all(bytes).await.map_err(|_| unavailable())?;
        file.sync_all().await.map_err(|_| unavailable())?;
        let outcome = tokio::fs::hard_link(&temporary, &target).await;
        let _ = tokio::fs::remove_file(&temporary).await;
        match outcome {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(_) => return Err(unavailable()),
        }
        tokio::fs::File::open(&self.root)
            .await
            .map_err(|_| unavailable())?
            .sync_all()
            .await
            .map_err(|_| unavailable())?;
        Ok(())
    }
    async fn read(&self, id: Uuid) -> Result<Option<Vec<u8>>, ApiError> {
        match tokio::fs::read(self.root.join(format!("{id}.json"))).await {
            Ok(b) if b.len() <= 4096 => Ok(Some(b)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            _ => Err(unavailable()),
        }
    }
}
/// Single privacy owner, shared by HTTP actions and bounded recovery workers.
#[derive(Clone)]
pub struct PrivacyStore {
    /// Same account database as all product owners.
    pub database: AccountDatabase,
    journal: Arc<dyn ReceiptJournal>,
    slots: Arc<tokio::sync::Semaphore>,
}
impl PrivacyStore {
    /// Construct with an independent journal adapter (never the SQL database itself).
    pub fn new(database: AccountDatabase, journal: Arc<dyn ReceiptJournal>) -> Self {
        Self {
            database,
            journal,
            slots: Arc::new(tokio::sync::Semaphore::new(4)),
        }
    }
    /// Reconcile every independently retained receipt before binding a serving socket.
    /// Unknown legacy schemas or malformed inventory leave the entire process quarantined.
    pub async fn reconcile_inventory(&self) -> Result<usize, ApiError> {
        let ids = self.journal.inventory().await?;
        let mut ordered = Vec::new();
        for id in &ids {
            let bytes = self.journal.read(*id).await?.ok_or_else(unavailable)?;
            if let Ok(legacy) = serde_json::from_slice::<LegacyReceipt>(&bytes) {
                legacy.validate(*id)?;
                ordered.push((legacy.occurred_at, *id));
                continue;
            }
            let r: Receipt = serde_json::from_slice(&bytes).map_err(|_| unavailable())?;
            if r.schema_version != 1
                || r.event_id != *id
                || serde_json::to_vec(&r).map_err(|_| unavailable())? != bytes
            {
                return Err(unavailable());
            }
            ordered.push((r.occurred_at, *id));
        }
        ordered.sort_unstable();
        for (_, id) in ordered {
            self.replay_for_restore(id).await?;
        }
        if self.database.has_unsupported_receipts().await? {
            return Err(unavailable());
        }
        Ok(ids.len())
    }
    /// Commit restriction and original receipt together, then confirm independent readback.
    /// Retry the same event identity: mismatching arguments are a conflict, not a new action.
    pub async fn restrict(
        &self,
        context: &AccountContext,
        event_id: Uuid,
        operation: Restriction,
        scope_id: Uuid,
        scope_version: i64,
    ) -> Result<Receipt, ApiError> {
        let mut tx = self.database.begin(context.account_id).await?;
        // Serialize duplicate event submissions with the same lock used by every account mutation.
        sqlx::query("select id from public.users where id=$1 for update")
            .bind(context.account_id)
            .fetch_optional(&mut *tx)
            .await?;
        // Readback retry remains possible even after this very action revoked the session.
        let existing:Option<Vec<u8>>=sqlx::query_scalar("select receipt_bytes from public.privacy_recovery_events where id=$1 and account_id=$2 and receipt_bytes is not null")
            .bind(event_id).bind(context.account_id).fetch_optional(&mut *tx).await?;
        let receipt = if let Some(bytes) = existing {
            let r: Receipt = serde_json::from_slice(&bytes).map_err(|_| unavailable())?;
            if r.operation != operation
                || r.scope_id != scope_id
                || r.scope_version != scope_version
                || r.account_incarnation != context.incarnation
            {
                return Err(ApiError::Conflict(
                    "privacy retry differs from original operation",
                ));
            }
            r
        } else {
            check_current(&mut tx, context, false).await?;
            if operation == Restriction::AccountDelete
                && Utc::now() - context.authenticated_at > chrono::Duration::minutes(5)
            {
                return Err(ApiError::Unauthorized);
            }
            // Publication replacement takes this same account lock. Bind a new
            // revoke operation to the reviewed active publication before applying
            // the restriction and inserting its receipt in this transaction.
            // Existing receipts above deliberately bypass current-target checks:
            // acknowledgement retries must not touch a later replacement.
            if operation == Restriction::PublicationRevoke {
                let current: Option<Uuid> = sqlx::query_scalar(
                    "select id from duet_publications where user_id=$1 and revoked_at is null",
                )
                .bind(context.account_id)
                .fetch_optional(&mut *tx)
                .await?;
                if current != Some(scope_id) {
                    return Err(ApiError::Conflict(
                        "reviewed publication is no longer active",
                    ));
                }
                if scope_version != 0 {
                    return Err(ApiError::Unprocessable(
                        "publication scope version must be zero",
                    ));
                }
            }
            let r = Receipt {
                schema_version: 1,
                event_id,
                account_id: context.account_id,
                account_incarnation: context.incarnation,
                operation,
                scope_id,
                scope_version,
                occurred_at: Utc::now(),
            };
            if matches!(
                operation,
                Restriction::DoctorPageDelete
                    | Restriction::AccountDelete
                    | Restriction::DeviceWithdraw
                    | Restriction::RecordsWithdraw
            ) {
                let publications: Vec<Uuid> = sqlx::query_scalar(
                    "select id from duet_publications where user_id=$1 and revoked_at is null",
                )
                .bind(context.account_id)
                .fetch_all(&mut *tx)
                .await?;
                for id in publications {
                    record_in_transaction(&mut tx, context, Restriction::PublicationRevoke, id, 0)
                        .await?;
                }
                sqlx::query("update duet_publications set revoked_at=now() where user_id=$1 and revoked_at is null").bind(context.account_id).execute(&mut *tx).await?;
            }
            if matches!(
                operation,
                Restriction::DeviceWithdraw | Restriction::RecordsWithdraw
            ) {
                let rows=sqlx::query("select coalesce(evidence_id,note_id) as id,max(revision) as revision from canonical_evidence_revisions where user_id=$1 and (evidence_id in (select id from health_evidence where user_id=$1 and source_kind=$2) or ($2='healthRecords' and note_id in(select id from clinical_notes where user_id=$1 and source_label='Health Records'))) group by coalesce(evidence_id,note_id)").bind(context.account_id).bind(if operation==Restriction::DeviceWithdraw{"appleHealth"}else{"healthRecords"}).fetch_all(&mut *tx).await?;
                for source in rows {
                    record_in_transaction(
                        &mut tx,
                        context,
                        Restriction::EvidenceDelete,
                        source.get("id"),
                        source.get("revision"),
                    )
                    .await?;
                }
            }
            if !apply(&mut tx, &r, false).await? {
                return Err(ApiError::Conflict(
                    "restriction resource incarnation changed",
                ));
            }
            insert_receipt(&mut tx, &r).await?;
            r
        };
        tx.commit().await?;
        self.confirm(&receipt).await?;
        self.confirm_account(receipt.account_id).await?;
        Ok(receipt)
    }
    /// Deliver exactly these retained bytes. Ambiguous writes are reconciled by the same key/readback.
    pub async fn confirm(&self, receipt: &Receipt) -> Result<(), ApiError> {
        let _permit = self.slots.try_acquire().map_err(|_| unavailable())?;
        let bytes = serde_json::to_vec(receipt).map_err(|_| unavailable())?;
        let outcome = tokio::time::timeout(std::time::Duration::from_secs(30), async {
            let _ambiguous = self.journal.put_if_absent(receipt.event_id, &bytes).await;
            if self.journal.read(receipt.event_id).await?.as_deref() != Some(bytes.as_slice()) {
                return Err(unavailable());
            }
            Ok::<_, ApiError>(())
        })
        .await;
        let confirmed = matches!(outcome, Ok(Ok(())));
        let mut tx = self.database.begin(receipt.account_id).await?;
        sqlx::query("update public.privacy_recovery_events set attempts=attempts+1, next_attempt_at=now()+least(3600, (30 * power(2,least(attempts,7)))::int)*interval '1 second', journaled_at=case when $2 then coalesce(journaled_at,now()) else journaled_at end where id=$1 and receipt_bytes=$3")
            .bind(receipt.event_id).bind(confirmed).bind(&bytes).execute(&mut *tx).await?;
        tx.commit().await?;
        if confirmed {
            Ok(())
        } else {
            Err(unavailable())
        }
    }
    /// Confirm all retained account restrictions before an action acknowledges success (including save retries).
    pub async fn confirm_account(&self, account: Uuid) -> Result<(), ApiError> {
        let mut tx = self.database.begin(account).await?;
        let rows:Vec<Vec<u8>>=sqlx::query_scalar("select receipt_bytes from privacy_recovery_events where account_id=$1 and journaled_at is null and receipt_bytes is not null order by occurred_at,id limit 65").bind(account).fetch_all(&mut *tx).await?;
        tx.commit().await?;
        if rows.len() > 64 {
            return Err(unavailable());
        }
        for bytes in rows {
            let r: Receipt = serde_json::from_slice(&bytes).map_err(|_| unavailable())?;
            self.confirm(&r).await?;
        }
        Ok(())
    }
    /// A bounded recovery batch retains failed work and backoff; never spins or marks pending work done.
    pub async fn recover(&self, account: Uuid) -> Result<usize, ApiError> {
        let mut tx = self.database.begin(account).await?;
        let pending:Vec<Vec<u8>>=sqlx::query_scalar("select receipt_bytes from public.privacy_recovery_events where account_id=$1 and receipt_bytes is not null and journaled_at is null and next_attempt_at<=now() order by occurred_at,id limit 64")
            .bind(account).fetch_all(&mut *tx).await?;
        tx.commit().await?;
        let mut completed = 0;
        for bytes in pending {
            let r: Receipt = serde_json::from_slice(&bytes).map_err(|_| unavailable())?;
            if self.confirm(&r).await.is_ok() {
                completed += 1;
            }
        }
        Ok(completed)
    }
    async fn replay_legacy(&self, legacy: LegacyReceipt, bytes: &[u8]) -> Result<bool, ApiError> {
        let mut tx = self.database.begin(legacy.account_id).await?;
        if let Some(existing) = sqlx::query_scalar::<_, Vec<u8>>(
            "select receipt_bytes from duet_legacy_privacy_replays where event_id=$1",
        )
        .bind(legacy.event_id)
        .fetch_optional(&mut *tx)
        .await?
            && existing != bytes
        {
            return Err(ApiError::Conflict("legacy receipt identity collision"));
        }
        let row=sqlx::query("select duet_incarnation,intelligence_control_revision,apple_health_generation,evidence_revision from users where id=$1 for update").bind(legacy.account_id).fetch_optional(&mut *tx).await?;
        let mut applied = false;
        if let Some(row) = row {
            let admitted: Option<Uuid> = sqlx::query_scalar(
                "select incarnation from duet_legacy_privacy_accounts where account_id=$1",
            )
            .bind(legacy.account_id)
            .fetch_optional(&mut *tx)
            .await?;
            if admitted != Some(row.get::<Uuid, _>("duet_incarnation"))
                && !matches!(
                    legacy.operation.as_str(),
                    "conversation_forget"
                        | "conversation_forget_restore"
                        | "genome_withdraw"
                        | "doctor_preparation_delete"
                )
            {
                return Err(ApiError::Unavailable(
                    "legacy receipt account incarnation is not admitted by migration",
                ));
            }
            let operation = match legacy.operation.as_str() {
                "luna_withdraw" => Some(Restriction::LunaWithdraw),
                "apple_health_withdraw" => Some(Restriction::DeviceWithdraw),
                "health_records_withdraw" => Some(Restriction::RecordsWithdraw),
                "account_delete" => Some(Restriction::AccountDelete),
                "doctor_note_delete" => Some(Restriction::DoctorNoteDelete),
                "doctor_page_delete" => Some(Restriction::DoctorPageDelete),
                "doctor_item_delete" => Some(Restriction::DoctorItemDelete),
                "follow_up_pause" => Some(Restriction::FollowUpPause),
                "follow_up_cancel" => Some(Restriction::FollowUpCancel),
                "proactive_withdraw" => Some(Restriction::ProactiveWithdraw),
                _ => None,
            };
            if let Some(operation) = operation {
                let scope_version = match operation {
                    Restriction::LunaWithdraw => row.get("intelligence_control_revision"),
                    Restriction::DeviceWithdraw => {
                        i64::from(row.get::<i32, _>("apple_health_generation"))
                    }
                    Restriction::RecordsWithdraw => row.get("evidence_revision"),
                    _ => legacy.scope_version.parse().unwrap_or(0),
                };
                let r = Receipt {
                    schema_version: 1,
                    event_id: legacy.event_id,
                    account_id: legacy.account_id,
                    account_incarnation: row.get("duet_incarnation"),
                    operation,
                    scope_id: legacy.scope_id,
                    scope_version,
                    occurred_at: legacy.occurred_at,
                };
                applied = apply(&mut tx, &r, true).await?;
            }
        }
        sqlx::query("insert into duet_legacy_privacy_replays(event_id,account_id,receipt_bytes) values($1,$2,$3) on conflict(event_id) do nothing").bind(legacy.event_id).bind(legacy.account_id).bind(bytes).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(applied)
    }
    async fn replay_for_restore(&self, event_id: Uuid) -> Result<bool, ApiError> {
        let bytes = self.journal.read(event_id).await?.ok_or_else(unavailable)?;
        if serde_json::from_slice::<LegacyReceipt>(&bytes).is_ok() {
            return self.replay(event_id).await;
        }
        let mut receipt: Receipt = serde_json::from_slice(&bytes).map_err(|_| unavailable())?;
        // Full restore requires fresh grants even if a later positive grant was
        // backed up. Targeted resource replay below keeps its original fence.
        if !matches!(
            receipt.operation,
            Restriction::LunaWithdraw | Restriction::DeviceWithdraw | Restriction::RecordsWithdraw
        ) {
            return self.replay(event_id).await;
        }
        if receipt.schema_version != 1
            || receipt.event_id != event_id
            || serde_json::to_vec(&receipt).map_err(|_| unavailable())? != bytes
        {
            return Err(unavailable());
        }
        let mut tx = self.database.begin(receipt.account_id).await?;
        let row=sqlx::query("select duet_incarnation,intelligence_control_revision,apple_health_generation,evidence_revision from users where id=$1 for update").bind(receipt.account_id).fetch_optional(&mut *tx).await?;
        let Some(row) = row else { return Ok(false) };
        if row.get::<Uuid, _>("duet_incarnation") != receipt.account_incarnation {
            return Ok(false);
        }
        receipt.scope_version = match receipt.operation {
            Restriction::LunaWithdraw => row.get("intelligence_control_revision"),
            Restriction::DeviceWithdraw => i64::from(row.get::<i32, _>("apple_health_generation")),
            Restriction::RecordsWithdraw => row.get("evidence_revision"),
            _ => unreachable!(),
        };
        let applied = apply(&mut tx, &receipt, true).await?;
        // Never rewrite the independently retained original receipt's bytes.
        let original: Receipt = serde_json::from_slice(&bytes).map_err(|_| unavailable())?;
        insert_receipt(&mut tx, &original).await?;
        sqlx::query("update privacy_recovery_events set journaled_at=coalesce(journaled_at,now()) where id=$1 and receipt_bytes=$2").bind(event_id).bind(bytes).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(applied)
    }
    /// Restore replay reads the independent journal, validates its closed schema, and applies only to matching incarnations.
    /// Operators enumerate journal IDs before reopening restored serving; no patient data is accepted as a receipt.
    pub async fn replay(&self, event_id: Uuid) -> Result<bool, ApiError> {
        let bytes = self.journal.read(event_id).await?.ok_or_else(unavailable)?;
        if let Ok(legacy) = serde_json::from_slice::<LegacyReceipt>(&bytes) {
            legacy.validate(event_id)?;
            return self.replay_legacy(legacy, &bytes).await;
        }
        let r: Receipt = serde_json::from_slice(&bytes).map_err(|_| unavailable())?;
        if r.schema_version != 1
            || r.event_id != event_id
            || serde_json::to_vec(&r).map_err(|_| unavailable())? != bytes
        {
            return Err(unavailable());
        }
        let mut tx = self.database.begin(r.account_id).await?;
        let applied = apply(&mut tx, &r, true).await?;
        insert_receipt(&mut tx, &r).await?;
        sqlx::query("update public.privacy_recovery_events set journaled_at=coalesce(journaled_at,now()) where id=$1 and receipt_bytes=$2").bind(event_id).bind(bytes).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(applied)
    }
}
// Scoped compatibility with the original closed v1 schema. Unsupported active
// product families still quarantine; retired operations intentionally replay no effect.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct LegacyReceipt {
    schema_version: u32,
    event_id: Uuid,
    account_id: Uuid,
    operation: String,
    scope_id: Uuid,
    scope_version: String,
    occurred_at: chrono::DateTime<Utc>,
    state: String,
}
impl LegacyReceipt {
    fn validate(&self, id: Uuid) -> Result<(), ApiError> {
        if self.schema_version != 1 || self.event_id != id || self.state != "restriction_committed"
        {
            return Err(unavailable());
        }
        match self.operation.as_str() {
            "doctor_note_delete" | "doctor_page_delete" => {
                if self.scope_version.is_empty()
                    || !self.scope_version.bytes().all(|b| b.is_ascii_digit())
                {
                    return Err(unavailable());
                }
                self.scope_version
                    .parse::<i64>()
                    .map_err(|_| unavailable())?;
            }
            "account_delete"
            | "luna_withdraw"
            | "apple_health_withdraw"
            | "health_records_withdraw"
            | "doctor_item_delete"
            | "follow_up_pause"
            | "follow_up_cancel"
            | "proactive_withdraw" => {
                if !self.scope_version.is_empty() {
                    return Err(unavailable());
                }
            }
            "doctor_preparation_delete" => {
                self.scope_version
                    .parse::<DateTime<Utc>>()
                    .map_err(|_| unavailable())?;
            }
            "conversation_forget" | "conversation_forget_restore" | "genome_withdraw" => {}
            _ => {
                return Err(ApiError::Unavailable(
                    "legacy receipt operation requires its scoped migration owner",
                ));
            }
        }
        Ok(())
    }
}
/// Record a restriction already applied by its action owner in the same locked transaction.
/// The action must confirm retained bytes after commit before acknowledging privacy success.
pub async fn record_in_transaction(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    c: &AccountContext,
    operation: Restriction,
    scope_id: Uuid,
    scope_version: i64,
) -> Result<Receipt, ApiError> {
    let r = Receipt {
        schema_version: 1,
        event_id: Uuid::new_v4(),
        account_id: c.account_id,
        account_incarnation: c.incarnation,
        operation,
        scope_id,
        scope_version,
        occurred_at: Utc::now(),
    };
    insert_receipt(tx, &r).await?;
    Ok(r)
}
async fn insert_receipt(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    r: &Receipt,
) -> Result<(), ApiError> {
    let bytes = serde_json::to_vec(r).map_err(|_| unavailable())?;
    let operation = serde_json::to_value(r.operation).map_err(|_| unavailable())?;
    sqlx::query("insert into public.privacy_recovery_events(id,account_id,operation,scope_id,scope_version,occurred_at,account_incarnation,receipt_bytes) values($1,$2,$3,$4,$5,$6,$7,$8) on conflict(id) do nothing")
        .bind(r.event_id).bind(r.account_id).bind(operation.as_str().unwrap_or("")).bind(r.scope_id).bind(r.scope_version.to_string()).bind(r.occurred_at).bind(r.account_incarnation).bind(&bytes).execute(&mut **tx).await?;
    let stored: Option<Vec<u8>> =
        sqlx::query_scalar("select receipt_bytes from public.privacy_recovery_events where id=$1")
            .bind(r.event_id)
            .fetch_one(&mut **tx)
            .await?;
    if stored.as_deref() != Some(bytes.as_slice()) {
        return Err(ApiError::Conflict("receipt identity collision"));
    }
    Ok(())
}
async fn purge_record_selection(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    account: Uuid,
    record: Uuid,
) -> Result<(), ApiError> {
    let items:Vec<Uuid>=sqlx::query_scalar("select id from doctor_collection_items where user_id=$1 and record_selection->>'canonical_record_id'=$2").bind(account).bind(record.to_string()).fetch_all(&mut **tx).await?;
    purge_selected(tx, account, &items).await
}
async fn purge_selected(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    account: Uuid,
    items: &[Uuid],
) -> Result<(), ApiError> {
    if !items.is_empty() {
        sqlx::query("delete from doctor_collection_items where user_id=$1 and id=any($2)")
            .bind(account)
            .bind(items)
            .execute(&mut **tx)
            .await?;
        sqlx::query("delete from duet_doctor_saves where user_id=$1")
            .bind(account)
            .execute(&mut **tx)
            .await?;
        sqlx::query("delete from duet_doctor_proposals where user_id=$1")
            .bind(account)
            .execute(&mut **tx)
            .await?;
        sqlx::query("update users set doctor_proposal=null,doctor_note_generation=doctor_note_generation+1 where id=$1").bind(account).execute(&mut **tx).await?;
    }
    Ok(())
}
async fn apply(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    r: &Receipt,
    restoring: bool,
) -> Result<bool, ApiError> {
    let row=sqlx::query("select duet_incarnation,intelligence_control_revision,doctor_note_generation,evidence_revision,apple_health_generation from public.users where id=$1 for update").bind(r.account_id).fetch_optional(&mut **tx).await?;
    let Some(row) = row else { return Ok(false) };
    if row.get::<Uuid, _>("duet_incarnation") != r.account_incarnation {
        return Ok(false);
    }
    match r.operation {
        Restriction::LunaWithdraw => {
            if r.scope_id != r.account_id
                || !(row.get::<i64, _>("intelligence_control_revision") == r.scope_version
                    || (restoring
                        && row.get::<i64, _>("intelligence_control_revision") < r.scope_version))
            {
                return Ok(false);
            }
            sqlx::query("update public.users set intelligence_enabled=false,luna_consent_decision='declined',luna_consent_version='2026-09-24',luna_consent_at=$2,intelligence_control_revision=greatest(intelligence_control_revision,$3)+1 where id=$1").bind(r.account_id).bind(r.occurred_at).bind(r.scope_version).execute(&mut **tx).await?;
            sqlx::query("update duet_commands set state='cancelled',staged_memory=null,lease_until=null where user_id=$1 and state in ('accepted','running')").bind(r.account_id).execute(&mut **tx).await?;
        }
        Restriction::SessionsRevoke => {
            if r.scope_id != r.account_id {
                return Ok(false);
            }
            sqlx::query("update public.users set sessions_revoked_before=greatest(sessions_revoked_before,$2) where id=$1").bind(r.account_id).bind(r.occurred_at).execute(&mut **tx).await?;
        }
        Restriction::DoctorNoteDelete => {
            if r.scope_id != r.account_id
                || (if restoring {
                    row.get::<i64, _>("doctor_note_generation") > r.scope_version
                } else {
                    row.get::<i64, _>("doctor_note_generation") != r.scope_version
                })
            {
                return Ok(false);
            }
            sqlx::query(
                "delete from duet_doctor_saves where user_id=$1 and expected_generation<=$2",
            )
            .bind(r.account_id)
            .bind(r.scope_version)
            .execute(&mut **tx)
            .await?;
            sqlx::query(
                "update users set doctor_note=null,doctor_proposal=null,doctor_note_generation=$2+1 where id=$1",
            )
            .bind(r.account_id)
            .bind(r.scope_version)
            .execute(&mut **tx)
            .await?;
        }
        Restriction::DoctorPageDelete => {
            if r.scope_id != r.account_id
                || (if restoring {
                    row.get::<i64, _>("doctor_note_generation") > r.scope_version
                } else {
                    row.get::<i64, _>("doctor_note_generation") != r.scope_version
                })
            {
                return Ok(false);
            }
            sqlx::query("delete from doctor_collection_items where user_id=$1")
                .bind(r.account_id)
                .execute(&mut **tx)
                .await?;
            sqlx::query("delete from duet_doctor_proposals where user_id=$1")
                .bind(r.account_id)
                .execute(&mut **tx)
                .await?;
            sqlx::query("delete from duet_doctor_saves where user_id=$1")
                .bind(r.account_id)
                .execute(&mut **tx)
                .await?;
            sqlx::query("update users set doctor_note=null,doctor_page='{}',doctor_proposal=null,doctor_note_generation=$2+1,doctor_page_updated_at=now() where id=$1").bind(r.account_id).bind(r.scope_version).execute(&mut **tx).await?;
            sqlx::query(
                "update duet_publications set revoked_at=coalesce(revoked_at,$2) where user_id=$1",
            )
            .bind(r.account_id)
            .bind(r.occurred_at)
            .execute(&mut **tx)
            .await?;
        }
        Restriction::DoctorItemDelete => {
            let deleted =
                sqlx::query("delete from doctor_collection_items where user_id=$1 and id=$2")
                    .bind(r.account_id)
                    .bind(r.scope_id)
                    .execute(&mut **tx)
                    .await?
                    .rows_affected();
            sqlx::query("delete from duet_doctor_saves where user_id=$1 and result->'items' @> jsonb_build_array(jsonb_build_object('id',$2::text))").bind(r.account_id).bind(r.scope_id.to_string()).execute(&mut **tx).await?;
            sqlx::query("update duet_doctor_proposals set state='superseded' where user_id=$1 and proposal->'items' @> jsonb_build_array(jsonb_build_object('id',$2::text))").bind(r.account_id).bind(r.scope_id.to_string()).execute(&mut **tx).await?;
            if deleted > 0 {
                sqlx::query("update users set doctor_proposal=null,doctor_note_generation=doctor_note_generation+1,doctor_page_updated_at=now() where id=$1").bind(r.account_id).execute(&mut **tx).await?;
            }
            sqlx::query("update duet_publications set revoked_at=coalesce(revoked_at,$3) where user_id=$1 and snapshot->'items' @> jsonb_build_array(jsonb_build_object('id',$2::text))").bind(r.account_id).bind(r.scope_id.to_string()).bind(r.occurred_at).execute(&mut **tx).await?;
        }
        Restriction::PublicationRevoke => {
            sqlx::query("update duet_publications set revoked_at=coalesce(revoked_at,$3) where user_id=$1 and id=$2").bind(r.account_id).bind(r.scope_id).bind(r.occurred_at).execute(&mut **tx).await?;
        }
        Restriction::DeviceWithdraw => {
            let generation = i64::from(row.get::<i32, _>("apple_health_generation"));
            if r.scope_id != r.account_id
                || (if restoring {
                    generation > r.scope_version
                } else {
                    generation != r.scope_version
                })
            {
                return Ok(false);
            }
            sqlx::query(
                "update duet_publications set revoked_at=coalesce(revoked_at,$2) where user_id=$1",
            )
            .bind(r.account_id)
            .bind(r.occurred_at)
            .execute(&mut **tx)
            .await?;
            let items:Vec<Uuid>=sqlx::query_scalar("select id from doctor_collection_items where user_id=$1 and fact_snapshot_id is not null").bind(r.account_id).fetch_all(&mut **tx).await?;
            purge_selected(tx, r.account_id, &items).await?;
            let sources: Vec<Uuid> = sqlx::query_scalar(
                "select id from health_evidence where user_id=$1 and source_kind='appleHealth'",
            )
            .bind(r.account_id)
            .fetch_all(&mut **tx)
            .await?;
            for id in sources {
                purge_record_selection(tx, r.account_id, id).await?;
                sqlx::query("delete from health_evidence where user_id=$1 and id=$2")
                    .bind(r.account_id)
                    .bind(id)
                    .execute(&mut **tx)
                    .await?;
                let revision:i64=sqlx::query_scalar("update users set evidence_revision=evidence_revision+1 where id=$1 returning evidence_revision").bind(r.account_id).fetch_one(&mut **tx).await?;
                sqlx::query("update evidence_sync_changes set deleted=true,revision=$3 where user_id=$1 and record_id=$2").bind(r.account_id).bind(id).bind(revision).execute(&mut **tx).await?;
            }
            sqlx::query("delete from duet_device_snapshots where user_id=$1")
                .bind(r.account_id)
                .execute(&mut **tx)
                .await?;
            sqlx::query("update users set apple_health_generation=greatest(apple_health_generation,$2)+1,apple_health_disconnected_at=$3 where id=$1").bind(r.account_id).bind(r.scope_version as i32).bind(r.occurred_at).execute(&mut **tx).await?;
        }
        Restriction::RecordsWithdraw => {
            let revision = row.get::<i64, _>("evidence_revision");
            if r.scope_id != r.account_id
                || (if restoring {
                    revision > r.scope_version
                } else {
                    revision != r.scope_version
                })
            {
                return Ok(false);
            }
            sqlx::query(
                "update duet_publications set revoked_at=coalesce(revoked_at,$2) where user_id=$1",
            )
            .bind(r.account_id)
            .bind(r.occurred_at)
            .execute(&mut **tx)
            .await?;
            let records:Vec<Uuid>=sqlx::query_scalar("select id from health_evidence where user_id=$1 and source_kind='healthRecords' union all select id from clinical_notes where user_id=$1 and source_label='Health Records'").bind(r.account_id).fetch_all(&mut **tx).await?;
            for id in &records {
                purge_record_selection(tx, r.account_id, *id).await?;
            }
            sqlx::query(
                "delete from health_evidence where user_id=$1 and source_kind='healthRecords'",
            )
            .bind(r.account_id)
            .execute(&mut **tx)
            .await?;
            sqlx::query(
                "delete from clinical_notes where user_id=$1 and source_label='Health Records'",
            )
            .bind(r.account_id)
            .execute(&mut **tx)
            .await?;
            let cursor:i64=sqlx::query_scalar("update users set evidence_revision=greatest(evidence_revision,$2)+1,health_records_generation=health_records_generation+1,health_records_disconnected_at=$3 where id=$1 returning evidence_revision").bind(r.account_id).bind(r.scope_version).bind(r.occurred_at).fetch_one(&mut **tx).await?;
            sqlx::query("update evidence_sync_changes set deleted=true,revision=$2 where user_id=$1 and record_id=any($3)").bind(r.account_id).bind(cursor).bind(records).execute(&mut **tx).await?;
        }
        Restriction::FollowUpPause | Restriction::FollowUpCancel => {
            let revision: Option<i64> =
                sqlx::query_scalar("select revision from follow_ups where user_id=$1 and id=$2")
                    .bind(r.account_id)
                    .bind(r.scope_id)
                    .fetch_optional(&mut **tx)
                    .await?;
            if !restoring && revision != Some(r.scope_version) {
                return Ok(false);
            }
            crate::follow_up::replay_restriction(
                tx,
                r.account_id,
                r.account_incarnation,
                r.scope_id,
                r.operation == Restriction::FollowUpCancel,
            )
            .await?;
        }
        Restriction::ProactiveWithdraw => {
            if r.scope_id != r.account_id {
                return Ok(false);
            }
            crate::follow_up::replay_settings_withdrawal(tx, r.account_id, r.account_incarnation)
                .await?;
            sqlx::query("update duet_commands set state='cancelled',lease_id=null,lease_until=null,staged_memory=null where user_id=$1 and work_kind in ('today','follow_up') and state in ('accepted','running')").bind(r.account_id).execute(&mut **tx).await?;
        }
        Restriction::AccountDelete => {
            if r.scope_id != r.account_id {
                return Ok(false);
            }
            sqlx::query("insert into duet_deleted_identities(auth_user_id,account_id,incarnation,deleted_at) select auth_user_id,id,duet_incarnation,$2 from users where id=$1 on conflict(auth_user_id) do nothing").bind(r.account_id).bind(r.occurred_at).execute(&mut **tx).await?;
            for table in [
                "doctor_collection_items",
                "duet_doctor_saves",
                "duet_doctor_proposals",
                "duet_device_snapshots",
                "duet_publications",
                "health_evidence",
                "clinical_notes",
            ] {
                sqlx::query(&format!("delete from {table} where user_id=$1"))
                    .bind(r.account_id)
                    .execute(&mut **tx)
                    .await?;
            }
            sqlx::query("delete from users where id=$1")
                .bind(r.account_id)
                .execute(&mut **tx)
                .await?;
        }
        Restriction::EvidenceDelete => {
            let latest:Option<i64>=sqlx::query_scalar("select max(revision) from public.canonical_evidence_revisions where user_id=$1 and coalesce(evidence_id,note_id)=$2").bind(r.account_id).bind(r.scope_id).fetch_one(&mut **tx).await?;
            // Restored SQL may predate the final correction that was deleted.
            // The same canonical incarnation at an earlier revision is still covered;
            // later source incarnations/revisions are never erased by an old receipt.
            if !latest.is_some_and(|v| v == r.scope_version || (restoring && v < r.scope_version)) {
                return Ok(false);
            }
            purge_record_selection(tx, r.account_id, r.scope_id).await?;
            for table in ["health_evidence", "clinical_notes"] {
                sqlx::query(&format!(
                    "delete from public.{table} where user_id=$1 and id=$2"
                ))
                .bind(r.account_id)
                .bind(r.scope_id)
                .execute(&mut **tx)
                .await?;
            }
            let revision:i64=sqlx::query_scalar("update public.users set evidence_revision=evidence_revision+1 where id=$1 returning evidence_revision").bind(r.account_id).fetch_one(&mut **tx).await?;
            sqlx::query("update public.evidence_sync_changes set deleted=true,revision=$3 where user_id=$1 and record_id=$2").bind(r.account_id).bind(r.scope_id).bind(revision).execute(&mut **tx).await?;
        }
    }
    Ok(true)
}
