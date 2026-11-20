//! Restriction + SQL outbox, independent create-only journal, and fenced restore replay.
use crate::{
    ApiError,
    database::{AccountDatabase, orm},
    evidence_entity::{account, clinical_note, health_evidence, membership, revision},
    follow_up_entity::{command, conversation_event, follow_up, occurrence, settings, tool_effect},
    identity::{AccountContext, check_current},
    privacy_entity::{
        NoteSourceLabel, WithdrawnSourceKind, deleted_identity, device_snapshot, doctor_item,
        doctor_proposal, doctor_save, legacy_account, legacy_replay, privacy_event, publication,
        user_privacy,
    },
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveValue::Set,
    ColumnTrait, DeriveActiveEnum, EntityName, EntityTrait, EnumIter, FromQueryResult, QueryFilter,
    QueryOrder, QuerySelect, QueryTrait,
    sea_query::{
        Cond, Expr, ExprTrait, Func, OnConflict, Query, SimpleExpr, UnionType,
        extension::postgres::{PgBinOper, PgExpr},
    },
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::{path::PathBuf, sync::Arc};
use uuid::Uuid;

/// Metadata-only restriction operations. Patient prose and bearer credentials cannot enter receipts.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum Restriction {
    /// Disable Luna and invalidate accepted work.
    #[sea_orm(string_value = "luna_withdraw")]
    LunaWithdraw,
    /// Fence every session authenticated at or before the receipt timestamp.
    #[sea_orm(string_value = "sessions_revoke")]
    SessionsRevoke,
    /// Withdraw one canonical record, including its retained historical content.
    #[sea_orm(string_value = "evidence_delete")]
    EvidenceDelete,
    /// Clear the approved page and selected items through its generation.
    #[sea_orm(string_value = "doctor_page_delete")]
    DoctorPageDelete,
    /// Clear only the patient's concern through its workspace generation.
    #[sea_orm(string_value = "doctor_note_delete")]
    DoctorNoteDelete,
    /// Remove one stable selected-item incarnation.
    #[sea_orm(string_value = "doctor_item_delete")]
    DoctorItemDelete,
    /// Revoke one pinned publication incarnation.
    #[sea_orm(string_value = "publication_revoke")]
    PublicationRevoke,
    /// Withdraw all device snapshots currently held by this account.
    #[sea_orm(string_value = "device_withdraw")]
    DeviceWithdraw,
    /// Withdraw imported Health Records through the account evidence cursor.
    #[sea_orm(string_value = "records_withdraw")]
    RecordsWithdraw,
    /// Pause accepted follow-through without granting later execution authority.
    #[sea_orm(string_value = "follow_up_pause")]
    FollowUpPause,
    /// Permanently cancel one accepted follow-up incarnation.
    #[sea_orm(string_value = "follow_up_cancel")]
    FollowUpCancel,
    /// Disable proactive work until fresh patient authorization after restore.
    #[sea_orm(string_value = "proactive_withdraw")]
    ProactiveWithdraw,
    /// Remove product account state and block stale subject reprovisioning.
    #[sea_orm(string_value = "account_delete")]
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
        // Account-deletion retries run after the `users` row is deleted, so absence is allowed here.
        let _ = orm::row(
            &mut tx,
            &account::Entity::find_by_id(context.account_id)
                .select_only()
                .column(account::Column::Id)
                .lock_exclusive()
                .into_query(),
        )
        .await?;
        // Readback retry remains possible even after this very action revoked the session.
        let existing = find_receipt_bytes(&mut tx, event_id, context.account_id).await?;
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
                let current = active_publication_ids(&mut tx, context.account_id)
                    .await?
                    .into_iter()
                    .next();
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
                let publications = active_publication_ids(&mut tx, context.account_id).await?;
                for id in publications {
                    record_in_transaction(&mut tx, context, Restriction::PublicationRevoke, id, 0)
                        .await?;
                }
                orm::execute(
                    &mut tx,
                    &publication::Entity::update_many()
                        .col_expr(publication::Column::RevokedAt, now())
                        .filter(publication::Column::UserId.eq(context.account_id))
                        .filter(publication::Column::RevokedAt.is_null())
                        .into_query(),
                )
                .await?;
            }
            if let Some(source_kind) = match operation {
                Restriction::DeviceWithdraw => Some(WithdrawnSourceKind::AppleHealth),
                Restriction::RecordsWithdraw => Some(WithdrawnSourceKind::HealthRecords),
                _ => None,
            } {
                for source in withdrawn_revisions(&mut tx, context.account_id, source_kind).await? {
                    record_in_transaction(
                        &mut tx,
                        context,
                        Restriction::EvidenceDelete,
                        source.id,
                        source.revision,
                    )
                    .await?;
                }
            }
            if !apply(&mut tx, &r, ApplyMode::Live).await? {
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
        // SeaQuery has no typed builder for PostgreSQL power() / interval backoff arithmetic.
        sqlx::query("update public.privacy_recovery_events set attempts=attempts+1, next_attempt_at=now()+least(3600, (30 * power(2,least(attempts,7)))::int)*interval '1 second', journaled_at=case when $2 then coalesce(journaled_at,now()) else journaled_at end where id=$1 and receipt_bytes=$3")
            .bind(receipt.event_id)
            .bind(confirmed)
            .bind(&bytes)
            .execute(&mut *tx)
            .await?;
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
        let rows =
            pending_receipt_bytes(&mut tx, account, PendingWindow::AllUnjournaled, 65).await?;
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
        let pending = pending_receipt_bytes(&mut tx, account, PendingWindow::DueNow, 64).await?;
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
    async fn replay_legacy(
        &self,
        legacy: LegacyReceipt,
        validated: ValidatedLegacyOp,
        bytes: &[u8],
    ) -> Result<bool, ApiError> {
        let mut tx = self.database.begin(legacy.account_id).await?;
        if let Some(existing) = orm::model::<LegacyBytesRow>(
            &mut tx,
            &legacy_replay::Entity::find_by_id(legacy.event_id)
                .select_only()
                .column(legacy_replay::Column::ReceiptBytes)
                .into_query(),
        )
        .await?
            && existing.receipt_bytes != bytes
        {
            return Err(ApiError::Conflict("legacy receipt identity collision"));
        }
        let fences = lock_account_fences(&mut tx, legacy.account_id).await?;
        let mut applied = false;
        if let Some(fences) = fences {
            let admitted = orm::model::<AdmittedIncarnationRow>(
                &mut tx,
                &legacy_account::Entity::find_by_id(legacy.account_id)
                    .select_only()
                    .column(legacy_account::Column::Incarnation)
                    .into_query(),
            )
            .await?
            .map(|row| row.incarnation);
            match validated {
                ValidatedLegacyOp::Retired => {}
                ValidatedLegacyOp::Active {
                    restriction,
                    parsed_version,
                } => {
                    if admitted != Some(fences.duet_incarnation) {
                        return Err(ApiError::Unavailable(
                            "legacy receipt account incarnation is not admitted by migration",
                        ));
                    }
                    let scope_version = fences.restore_scope_version(restriction, parsed_version);
                    let r = Receipt {
                        schema_version: 1,
                        event_id: legacy.event_id,
                        account_id: legacy.account_id,
                        account_incarnation: fences.duet_incarnation,
                        operation: restriction,
                        scope_id: legacy.scope_id,
                        scope_version,
                        occurred_at: legacy.occurred_at,
                    };
                    applied = apply(&mut tx, &r, ApplyMode::Restore).await?;
                }
            }
        }
        orm::execute(
            &mut tx,
            &legacy_replay::Entity::insert(legacy_replay::ActiveModel {
                event_id: Set(legacy.event_id),
                account_id: Set(legacy.account_id),
                receipt_bytes: Set(bytes.to_vec()),
            })
            .on_conflict(
                OnConflict::column(legacy_replay::Column::EventId)
                    .do_nothing()
                    .to_owned(),
            )
            .into_query(),
        )
        .await?;
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
        let Some(fences) = lock_account_fences(&mut tx, receipt.account_id).await? else {
            return Ok(false);
        };
        if fences.duet_incarnation != receipt.account_incarnation {
            return Ok(false);
        }
        receipt.scope_version =
            fences.restore_scope_version(receipt.operation, receipt.scope_version);
        let applied = apply(&mut tx, &receipt, ApplyMode::Restore).await?;
        // Never rewrite the independently retained original receipt's bytes.
        let original: Receipt = serde_json::from_slice(&bytes).map_err(|_| unavailable())?;
        insert_receipt(&mut tx, &original).await?;
        mark_receipt_journaled(&mut tx, event_id, &bytes).await?;
        tx.commit().await?;
        Ok(applied)
    }
    /// Restore replay reads the independent journal, validates its closed schema, and applies only to matching incarnations.
    /// Operators enumerate journal IDs before reopening restored serving; no patient data is accepted as a receipt.
    pub async fn replay(&self, event_id: Uuid) -> Result<bool, ApiError> {
        let bytes = self.journal.read(event_id).await?.ok_or_else(unavailable)?;
        if let Ok(legacy) = serde_json::from_slice::<LegacyReceipt>(&bytes) {
            let validated = legacy.validate(event_id)?;
            return self.replay_legacy(legacy, validated, &bytes).await;
        }
        let r: Receipt = serde_json::from_slice(&bytes).map_err(|_| unavailable())?;
        if r.schema_version != 1
            || r.event_id != event_id
            || serde_json::to_vec(&r).map_err(|_| unavailable())? != bytes
        {
            return Err(unavailable());
        }
        let mut tx = self.database.begin(r.account_id).await?;
        let applied = apply(&mut tx, &r, ApplyMode::Restore).await?;
        insert_receipt(&mut tx, &r).await?;
        mark_receipt_journaled(&mut tx, event_id, &bytes).await?;
        tx.commit().await?;
        Ok(applied)
    }
}

/// Validated legacy receipt disposition, decided once for validation and replay.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ValidatedLegacyOp {
    Active {
        restriction: Restriction,
        parsed_version: i64,
    },
    Retired,
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
    fn validate(&self, id: Uuid) -> Result<ValidatedLegacyOp, ApiError> {
        if self.schema_version != 1 || self.event_id != id || self.state != "restriction_committed"
        {
            return Err(unavailable());
        }
        let parse_digits = || -> Result<i64, ApiError> {
            if self.scope_version.is_empty()
                || !self.scope_version.bytes().all(|b| b.is_ascii_digit())
            {
                return Err(unavailable());
            }
            self.scope_version.parse::<i64>().map_err(|_| unavailable())
        };
        let require_empty = || -> Result<(), ApiError> {
            if !self.scope_version.is_empty() {
                return Err(unavailable());
            }
            Ok(())
        };
        match self.operation.as_str() {
            "doctor_note_delete" => Ok(ValidatedLegacyOp::Active {
                restriction: Restriction::DoctorNoteDelete,
                parsed_version: parse_digits()?,
            }),
            "doctor_page_delete" => Ok(ValidatedLegacyOp::Active {
                restriction: Restriction::DoctorPageDelete,
                parsed_version: parse_digits()?,
            }),
            "account_delete" => {
                require_empty()?;
                Ok(ValidatedLegacyOp::Active {
                    restriction: Restriction::AccountDelete,
                    parsed_version: 0,
                })
            }
            "luna_withdraw" => {
                require_empty()?;
                Ok(ValidatedLegacyOp::Active {
                    restriction: Restriction::LunaWithdraw,
                    parsed_version: 0,
                })
            }
            "apple_health_withdraw" => {
                require_empty()?;
                Ok(ValidatedLegacyOp::Active {
                    restriction: Restriction::DeviceWithdraw,
                    parsed_version: 0,
                })
            }
            "health_records_withdraw" => {
                require_empty()?;
                Ok(ValidatedLegacyOp::Active {
                    restriction: Restriction::RecordsWithdraw,
                    parsed_version: 0,
                })
            }
            "doctor_item_delete" => {
                require_empty()?;
                Ok(ValidatedLegacyOp::Active {
                    restriction: Restriction::DoctorItemDelete,
                    parsed_version: 0,
                })
            }
            "follow_up_pause" => {
                require_empty()?;
                Ok(ValidatedLegacyOp::Active {
                    restriction: Restriction::FollowUpPause,
                    parsed_version: 0,
                })
            }
            "follow_up_cancel" => {
                require_empty()?;
                Ok(ValidatedLegacyOp::Active {
                    restriction: Restriction::FollowUpCancel,
                    parsed_version: 0,
                })
            }
            "proactive_withdraw" => {
                require_empty()?;
                Ok(ValidatedLegacyOp::Active {
                    restriction: Restriction::ProactiveWithdraw,
                    parsed_version: 0,
                })
            }
            "doctor_preparation_delete" => {
                self.scope_version
                    .parse::<DateTime<Utc>>()
                    .map_err(|_| unavailable())?;
                Ok(ValidatedLegacyOp::Retired)
            }
            "conversation_forget" | "conversation_forget_restore" => Ok(ValidatedLegacyOp::Retired),
            _ => Err(ApiError::Unavailable(
                "legacy receipt operation requires its scoped migration owner",
            )),
        }
    }
}

#[derive(FromQueryResult)]
struct IdRow {
    id: Uuid,
}

#[derive(FromQueryResult)]
struct ReceiptBytesOptionRow {
    receipt_bytes: Option<Vec<u8>>,
}

#[derive(FromQueryResult)]
struct LegacyBytesRow {
    receipt_bytes: Vec<u8>,
}

#[derive(FromQueryResult)]
struct AdmittedIncarnationRow {
    incarnation: Uuid,
}

#[derive(FromQueryResult)]
struct AccountFences {
    duet_incarnation: Uuid,
    intelligence_control_revision: i64,
    doctor_note_generation: i64,
    evidence_revision: i64,
    apple_health_generation: i32,
}

impl AccountFences {
    fn restore_scope_version(&self, restriction: Restriction, fallback: i64) -> i64 {
        match restriction {
            Restriction::LunaWithdraw => self.intelligence_control_revision,
            Restriction::DeviceWithdraw => i64::from(self.apple_health_generation),
            Restriction::RecordsWithdraw => self.evidence_revision,
            _ => fallback,
        }
    }
}

#[derive(FromQueryResult)]
struct WithdrawnRecordRevision {
    id: Uuid,
    revision: i64,
}

#[derive(FromQueryResult)]
struct FollowUpRevisionRow {
    revision: i64,
}

#[derive(FromQueryResult)]
struct MaxRevisionRow {
    max_revision: Option<i64>,
}

async fn find_receipt_bytes(
    tx: &mut Tx<'_>,
    event_id: Uuid,
    account_id: Uuid,
) -> Result<Option<Vec<u8>>, ApiError> {
    Ok(orm::model::<ReceiptBytesOptionRow>(
        tx,
        &privacy_event::Entity::find()
            .select_only()
            .column(privacy_event::Column::ReceiptBytes)
            .filter(privacy_event::Column::Id.eq(event_id))
            .filter(privacy_event::Column::AccountId.eq(account_id))
            .filter(privacy_event::Column::ReceiptBytes.is_not_null())
            .into_query(),
    )
    .await?
    .and_then(|row| row.receipt_bytes))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PendingWindow {
    AllUnjournaled,
    DueNow,
}

async fn pending_receipt_bytes(
    tx: &mut Tx<'_>,
    account_id: Uuid,
    window: PendingWindow,
    limit: u64,
) -> Result<Vec<Vec<u8>>, ApiError> {
    let mut query = privacy_event::Entity::find()
        .select_only()
        .column(privacy_event::Column::ReceiptBytes)
        .filter(privacy_event::Column::AccountId.eq(account_id))
        .filter(privacy_event::Column::JournaledAt.is_null())
        .filter(privacy_event::Column::ReceiptBytes.is_not_null());
    if window == PendingWindow::DueNow {
        query = query.filter(Expr::col(privacy_event::Column::NextAttemptAt).lte(now()));
    }
    let rows = orm::models::<ReceiptBytesOptionRow>(
        tx,
        &query
            .order_by_asc(privacy_event::Column::OccurredAt)
            .order_by_asc(privacy_event::Column::Id)
            .limit(limit)
            .into_query(),
    )
    .await?;
    Ok(rows
        .into_iter()
        .filter_map(|row| row.receipt_bytes)
        .collect())
}

async fn active_publication_ids(tx: &mut Tx<'_>, account_id: Uuid) -> Result<Vec<Uuid>, ApiError> {
    Ok(orm::models::<IdRow>(
        tx,
        &publication::Entity::find()
            .select_only()
            .column(publication::Column::Id)
            .filter(publication::Column::UserId.eq(account_id))
            .filter(publication::Column::RevokedAt.is_null())
            .into_query(),
    )
    .await?
    .into_iter()
    .map(|row| row.id)
    .collect())
}

fn revision_record_id() -> SimpleExpr {
    Func::coalesce([
        Expr::col(revision::Column::EvidenceId),
        Expr::col(revision::Column::NoteId),
    ])
    .into()
}

async fn withdrawn_revisions(
    tx: &mut Tx<'_>,
    account_id: Uuid,
    source_kind: WithdrawnSourceKind,
) -> Result<Vec<WithdrawnRecordRevision>, ApiError> {
    let record_id = revision_record_id();
    let evidence_ids = health_evidence::Entity::find()
        .select_only()
        .column(health_evidence::Column::Id)
        .filter(health_evidence::Column::UserId.eq(account_id))
        .filter(health_evidence::Column::SourceKind.eq(source_kind))
        .into_query();
    let mut source_match = Cond::any().add(revision::Column::EvidenceId.in_subquery(evidence_ids));
    if source_kind == WithdrawnSourceKind::HealthRecords {
        let note_ids = clinical_note::Entity::find()
            .select_only()
            .column(clinical_note::Column::Id)
            .filter(clinical_note::Column::UserId.eq(account_id))
            .filter(clinical_note::Column::SourceLabel.eq(NoteSourceLabel::HealthRecords))
            .into_query();
        source_match = source_match.add(revision::Column::NoteId.in_subquery(note_ids));
    }
    let mut query = revision::Entity::find()
        .select_only()
        .expr_as(record_id.clone(), "id")
        .expr_as(Expr::col(revision::Column::Revision).max(), "revision")
        .filter(revision::Column::UserId.eq(account_id))
        .filter(source_match)
        .into_query();
    query.add_group_by([record_id]);
    Ok(orm::models::<WithdrawnRecordRevision>(tx, &query).await?)
}

async fn lock_account_fences(
    tx: &mut Tx<'_>,
    account_id: Uuid,
) -> Result<Option<AccountFences>, ApiError> {
    Ok(orm::model::<AccountFences>(
        tx,
        &user_privacy::Entity::find_by_id(account_id)
            .select_only()
            .column(user_privacy::Column::DuetIncarnation)
            .column(user_privacy::Column::IntelligenceControlRevision)
            .column(user_privacy::Column::DoctorNoteGeneration)
            .column(user_privacy::Column::EvidenceRevision)
            .column(user_privacy::Column::AppleHealthGeneration)
            .lock_exclusive()
            .into_query(),
    )
    .await?)
}

async fn mark_receipt_journaled(
    tx: &mut Tx<'_>,
    event_id: Uuid,
    bytes: &[u8],
) -> Result<(), ApiError> {
    orm::execute(
        tx,
        &privacy_event::Entity::update_many()
            .col_expr(
                privacy_event::Column::JournaledAt,
                Func::coalesce([Expr::col(privacy_event::Column::JournaledAt), now()]).into(),
            )
            .filter(privacy_event::Column::Id.eq(event_id))
            .filter(privacy_event::Column::ReceiptBytes.eq(bytes.to_vec()))
            .into_query(),
    )
    .await?;
    Ok(())
}

fn coalesce_revoked_at(occurred_at: DateTime<Utc>) -> SimpleExpr {
    Func::coalesce([
        Expr::col(publication::Column::RevokedAt),
        Expr::val(occurred_at),
    ])
    .into()
}

async fn revoke_account_publications(
    tx: &mut Tx<'_>,
    account_id: Uuid,
    occurred_at: DateTime<Utc>,
) -> Result<(), ApiError> {
    orm::execute(
        tx,
        &publication::Entity::update_many()
            .col_expr(
                publication::Column::RevokedAt,
                coalesce_revoked_at(occurred_at),
            )
            .filter(publication::Column::UserId.eq(account_id))
            .into_query(),
    )
    .await?;
    Ok(())
}

async fn advance_evidence_revision(tx: &mut Tx<'_>, account_id: Uuid) -> Result<i64, ApiError> {
    let mut update = user_privacy::Entity::update_many()
        .col_expr(
            user_privacy::Column::EvidenceRevision,
            Expr::col(user_privacy::Column::EvidenceRevision).add(1),
        )
        .filter(user_privacy::Column::Id.eq(account_id))
        .into_query();
    update.returning_col(user_privacy::Column::EvidenceRevision);
    Ok(orm::row(tx, &update)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?
        .try_get(0)?)
}

async fn mark_records_privacy_deleted(
    tx: &mut Tx<'_>,
    account_id: Uuid,
    record_ids: &[Uuid],
    revision: i64,
) -> Result<(), ApiError> {
    if record_ids.is_empty() {
        return Ok(());
    }
    orm::execute(
        tx,
        &membership::Entity::update_many()
            .col_expr(membership::Column::Deleted, Expr::val(true))
            .col_expr(membership::Column::SourceRemoved, Expr::val(false))
            .col_expr(membership::Column::Revision, Expr::val(revision))
            .filter(membership::Column::UserId.eq(account_id))
            .filter(membership::Column::RecordId.is_in(record_ids.iter().copied()))
            .into_query(),
    )
    .await?;
    Ok(())
}

fn greatest_expr(lhs: impl Into<SimpleExpr>, rhs: impl Into<SimpleExpr>) -> SimpleExpr {
    Func::cust("greatest").args([lhs.into(), rhs.into()]).into()
}

fn json_items_contain<C: ColumnTrait>(column: C, item_needle: &serde_json::Value) -> SimpleExpr {
    Expr::col(column)
        .get_json_field("items")
        .binary(PgBinOper::Contains, Expr::val(item_needle.clone()))
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
    orm::execute(
        tx,
        &privacy_event::Entity::insert(privacy_event::ActiveModel {
            id: Set(r.event_id),
            account_id: Set(r.account_id),
            operation: Set(r.operation),
            scope_id: Set(r.scope_id),
            scope_version: Set(r.scope_version.to_string()),
            occurred_at: Set(r.occurred_at),
            account_incarnation: Set(Some(r.account_incarnation)),
            receipt_bytes: Set(Some(bytes.clone())),
            ..Default::default()
        })
        .on_conflict(
            OnConflict::column(privacy_event::Column::Id)
                .do_nothing()
                .to_owned(),
        )
        .into_query(),
    )
    .await?;
    let stored = orm::model::<ReceiptBytesOptionRow>(
        tx,
        &privacy_event::Entity::find_by_id(r.event_id)
            .select_only()
            .column(privacy_event::Column::ReceiptBytes)
            .into_query(),
    )
    .await?
    .ok_or(sqlx::Error::RowNotFound)?;
    if stored.receipt_bytes.as_deref() != Some(bytes.as_slice()) {
        return Err(ApiError::Conflict("receipt identity collision"));
    }
    Ok(())
}

async fn purge_record_selection(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    account: Uuid,
    record: Uuid,
) -> Result<(), ApiError> {
    let items: Vec<Uuid> = orm::models::<IdRow>(
        tx,
        &doctor_item::Entity::find()
            .select_only()
            .column(doctor_item::Column::Id)
            .filter(doctor_item::Column::UserId.eq(account))
            .filter(
                Expr::col(doctor_item::Column::RecordSelection)
                    .cast_json_field("canonical_record_id")
                    .eq(record.to_string()),
            )
            .into_query(),
    )
    .await?
    .into_iter()
    .map(|row| row.id)
    .collect();
    purge_selected(tx, account, &items).await
}

async fn purge_selected(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    account: Uuid,
    items: &[Uuid],
) -> Result<(), ApiError> {
    if !items.is_empty() {
        orm::execute(
            tx,
            &doctor_item::Entity::delete_many()
                .filter(doctor_item::Column::UserId.eq(account))
                .filter(doctor_item::Column::Id.is_in(items.iter().copied()))
                .into_query(),
        )
        .await?;
        orm::execute(
            tx,
            &doctor_save::Entity::delete_many()
                .filter(doctor_save::Column::UserId.eq(account))
                .into_query(),
        )
        .await?;
        orm::execute(
            tx,
            &doctor_proposal::Entity::delete_many()
                .filter(doctor_proposal::Column::UserId.eq(account))
                .into_query(),
        )
        .await?;
        orm::execute(
            tx,
            &user_privacy::Entity::update_many()
                .col_expr(user_privacy::Column::DoctorProposal, Expr::null())
                .col_expr(
                    user_privacy::Column::DoctorNoteGeneration,
                    Expr::col(user_privacy::Column::DoctorNoteGeneration).add(1),
                )
                .filter(user_privacy::Column::Id.eq(account))
                .into_query(),
        )
        .await?;
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ApplyMode {
    Live,
    Restore,
}

async fn apply(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    r: &Receipt,
    mode: ApplyMode,
) -> Result<bool, ApiError> {
    let restoring = mode == ApplyMode::Restore;
    let Some(fences) = lock_account_fences(tx, r.account_id).await? else {
        return Ok(false);
    };
    if fences.duet_incarnation != r.account_incarnation {
        return Ok(false);
    }
    match r.operation {
        Restriction::LunaWithdraw => {
            if r.scope_id != r.account_id
                || !(fences.intelligence_control_revision == r.scope_version
                    || (restoring && fences.intelligence_control_revision < r.scope_version))
            {
                return Ok(false);
            }
            orm::execute(
                tx,
                &user_privacy::Entity::update_many()
                    .col_expr(user_privacy::Column::IntelligenceEnabled, Expr::val(false))
                    .col_expr(
                        user_privacy::Column::LunaConsentDecision,
                        Expr::val(user_privacy::LunaConsentDecision::Declined),
                    )
                    .col_expr(
                        user_privacy::Column::LunaConsentVersion,
                        Expr::val(user_privacy::LUNA_CONSENT_VERSION),
                    )
                    .col_expr(
                        user_privacy::Column::LunaConsentAt,
                        Expr::val(r.occurred_at),
                    )
                    .col_expr(
                        user_privacy::Column::IntelligenceControlRevision,
                        greatest_expr(
                            Expr::col(user_privacy::Column::IntelligenceControlRevision),
                            Expr::val(r.scope_version),
                        )
                        .add(1),
                    )
                    .filter(user_privacy::Column::Id.eq(r.account_id))
                    .into_query(),
            )
            .await?;
            orm::execute(
                tx,
                &command::Entity::update_many()
                    .col_expr(command::Column::State, Expr::val(command::State::Cancelled))
                    .col_expr(command::Column::StagedMemory, Expr::null())
                    .col_expr(command::Column::LeaseUntil, Expr::null())
                    .filter(command::Column::UserId.eq(r.account_id))
                    .filter(
                        command::Column::State
                            .is_in([command::State::Accepted, command::State::Running]),
                    )
                    .into_query(),
            )
            .await?;
        }
        Restriction::SessionsRevoke => {
            if r.scope_id != r.account_id {
                return Ok(false);
            }
            orm::execute(
                tx,
                &user_privacy::Entity::update_many()
                    .col_expr(
                        user_privacy::Column::SessionsRevokedBefore,
                        greatest_expr(
                            Expr::col(user_privacy::Column::SessionsRevokedBefore),
                            Expr::val(r.occurred_at),
                        ),
                    )
                    .filter(user_privacy::Column::Id.eq(r.account_id))
                    .into_query(),
            )
            .await?;
            let _ = crate::push::revoke_devices_before(tx, r.account_id, r.occurred_at).await?;
        }
        Restriction::DoctorNoteDelete => {
            if r.scope_id != r.account_id
                || (if restoring {
                    fences.doctor_note_generation > r.scope_version
                } else {
                    fences.doctor_note_generation != r.scope_version
                })
            {
                return Ok(false);
            }
            orm::execute(
                tx,
                &doctor_save::Entity::delete_many()
                    .filter(doctor_save::Column::UserId.eq(r.account_id))
                    .filter(doctor_save::Column::ExpectedGeneration.lte(r.scope_version))
                    .into_query(),
            )
            .await?;
            orm::execute(
                tx,
                &user_privacy::Entity::update_many()
                    .col_expr(user_privacy::Column::DoctorNote, Expr::null())
                    .col_expr(user_privacy::Column::DoctorProposal, Expr::null())
                    .col_expr(
                        user_privacy::Column::DoctorNoteGeneration,
                        Expr::val(r.scope_version).add(1),
                    )
                    .filter(user_privacy::Column::Id.eq(r.account_id))
                    .into_query(),
            )
            .await?;
        }
        Restriction::DoctorPageDelete => {
            if r.scope_id != r.account_id
                || (if restoring {
                    fences.doctor_note_generation > r.scope_version
                } else {
                    fences.doctor_note_generation != r.scope_version
                })
            {
                return Ok(false);
            }
            orm::execute(
                tx,
                &doctor_item::Entity::delete_many()
                    .filter(doctor_item::Column::UserId.eq(r.account_id))
                    .into_query(),
            )
            .await?;
            orm::execute(
                tx,
                &doctor_proposal::Entity::delete_many()
                    .filter(doctor_proposal::Column::UserId.eq(r.account_id))
                    .into_query(),
            )
            .await?;
            orm::execute(
                tx,
                &doctor_save::Entity::delete_many()
                    .filter(doctor_save::Column::UserId.eq(r.account_id))
                    .into_query(),
            )
            .await?;
            orm::execute(
                tx,
                &user_privacy::Entity::update_many()
                    .col_expr(user_privacy::Column::DoctorNote, Expr::null())
                    .col_expr(
                        user_privacy::Column::DoctorPage,
                        Expr::val(serde_json::json!({})),
                    )
                    .col_expr(user_privacy::Column::DoctorProposal, Expr::null())
                    .col_expr(
                        user_privacy::Column::DoctorNoteGeneration,
                        Expr::val(r.scope_version).add(1),
                    )
                    .col_expr(user_privacy::Column::DoctorPageUpdatedAt, now())
                    .filter(user_privacy::Column::Id.eq(r.account_id))
                    .into_query(),
            )
            .await?;
            revoke_account_publications(tx, r.account_id, r.occurred_at).await?;
        }
        Restriction::DoctorItemDelete => {
            let deleted = orm::execute(
                tx,
                &doctor_item::Entity::delete_many()
                    .filter(doctor_item::Column::UserId.eq(r.account_id))
                    .filter(doctor_item::Column::Id.eq(r.scope_id))
                    .into_query(),
            )
            .await?;
            let item_needle = serde_json::json!([{ "id": r.scope_id }]);
            orm::execute(
                tx,
                &doctor_save::Entity::delete_many()
                    .filter(doctor_save::Column::UserId.eq(r.account_id))
                    .filter(json_items_contain(
                        doctor_save::Column::Result,
                        &item_needle,
                    ))
                    .into_query(),
            )
            .await?;
            orm::execute(
                tx,
                &doctor_proposal::Entity::update_many()
                    .col_expr(
                        doctor_proposal::Column::State,
                        Expr::val(doctor_proposal::State::Superseded),
                    )
                    .filter(doctor_proposal::Column::UserId.eq(r.account_id))
                    .filter(json_items_contain(
                        doctor_proposal::Column::Proposal,
                        &item_needle,
                    ))
                    .into_query(),
            )
            .await?;
            if deleted > 0 {
                orm::execute(
                    tx,
                    &user_privacy::Entity::update_many()
                        .col_expr(user_privacy::Column::DoctorProposal, Expr::null())
                        .col_expr(
                            user_privacy::Column::DoctorNoteGeneration,
                            Expr::col(user_privacy::Column::DoctorNoteGeneration).add(1),
                        )
                        .col_expr(user_privacy::Column::DoctorPageUpdatedAt, now())
                        .filter(user_privacy::Column::Id.eq(r.account_id))
                        .into_query(),
                )
                .await?;
            }
            orm::execute(
                tx,
                &publication::Entity::update_many()
                    .col_expr(
                        publication::Column::RevokedAt,
                        coalesce_revoked_at(r.occurred_at),
                    )
                    .filter(publication::Column::UserId.eq(r.account_id))
                    .filter(json_items_contain(
                        publication::Column::Snapshot,
                        &item_needle,
                    ))
                    .into_query(),
            )
            .await?;
        }
        Restriction::PublicationRevoke => {
            orm::execute(
                tx,
                &publication::Entity::update_many()
                    .col_expr(
                        publication::Column::RevokedAt,
                        coalesce_revoked_at(r.occurred_at),
                    )
                    .filter(publication::Column::UserId.eq(r.account_id))
                    .filter(publication::Column::Id.eq(r.scope_id))
                    .into_query(),
            )
            .await?;
        }
        Restriction::DeviceWithdraw => {
            let generation = i64::from(fences.apple_health_generation);
            if r.scope_id != r.account_id
                || (if restoring {
                    generation > r.scope_version
                } else {
                    generation != r.scope_version
                })
            {
                return Ok(false);
            }
            revoke_account_publications(tx, r.account_id, r.occurred_at).await?;
            let items: Vec<Uuid> = orm::models::<IdRow>(
                tx,
                &doctor_item::Entity::find()
                    .select_only()
                    .column(doctor_item::Column::Id)
                    .filter(doctor_item::Column::UserId.eq(r.account_id))
                    .filter(doctor_item::Column::FactSnapshotId.is_not_null())
                    .into_query(),
            )
            .await?
            .into_iter()
            .map(|row| row.id)
            .collect();
            purge_selected(tx, r.account_id, &items).await?;
            let sources: Vec<Uuid> = orm::models::<IdRow>(
                tx,
                &health_evidence::Entity::find()
                    .select_only()
                    .column(health_evidence::Column::Id)
                    .filter(health_evidence::Column::UserId.eq(r.account_id))
                    .filter(
                        health_evidence::Column::SourceKind.eq(WithdrawnSourceKind::AppleHealth),
                    )
                    .into_query(),
            )
            .await?
            .into_iter()
            .map(|row| row.id)
            .collect();
            for id in sources {
                purge_record_selection(tx, r.account_id, id).await?;
                orm::execute(
                    tx,
                    &health_evidence::Entity::delete_many()
                        .filter(health_evidence::Column::UserId.eq(r.account_id))
                        .filter(health_evidence::Column::Id.eq(id))
                        .into_query(),
                )
                .await?;
                let revision = advance_evidence_revision(tx, r.account_id).await?;
                mark_records_privacy_deleted(tx, r.account_id, &[id], revision).await?;
            }
            orm::execute(
                tx,
                &device_snapshot::Entity::delete_many()
                    .filter(device_snapshot::Column::UserId.eq(r.account_id))
                    .into_query(),
            )
            .await?;
            orm::execute(
                tx,
                &user_privacy::Entity::update_many()
                    .col_expr(
                        user_privacy::Column::AppleHealthGeneration,
                        greatest_expr(
                            Expr::col(user_privacy::Column::AppleHealthGeneration),
                            Expr::val(r.scope_version as i32),
                        )
                        .add(1),
                    )
                    .col_expr(
                        user_privacy::Column::AppleHealthDisconnectedAt,
                        Expr::val(r.occurred_at),
                    )
                    .filter(user_privacy::Column::Id.eq(r.account_id))
                    .into_query(),
            )
            .await?;
        }
        Restriction::RecordsWithdraw => {
            let revision = fences.evidence_revision;
            if r.scope_id != r.account_id
                || (if restoring {
                    revision > r.scope_version
                } else {
                    revision != r.scope_version
                })
            {
                return Ok(false);
            }
            revoke_account_publications(tx, r.account_id, r.occurred_at).await?;
            let mut records_query = health_evidence::Entity::find()
                .select_only()
                .column_as(health_evidence::Column::Id, "id")
                .filter(health_evidence::Column::UserId.eq(r.account_id))
                .filter(health_evidence::Column::SourceKind.eq(WithdrawnSourceKind::HealthRecords))
                .into_query();
            records_query.union(
                UnionType::All,
                clinical_note::Entity::find()
                    .select_only()
                    .column_as(clinical_note::Column::Id, "id")
                    .filter(clinical_note::Column::UserId.eq(r.account_id))
                    .filter(clinical_note::Column::SourceLabel.eq(NoteSourceLabel::HealthRecords))
                    .into_query(),
            );
            let records: Vec<Uuid> = orm::models::<IdRow>(tx, &records_query)
                .await?
                .into_iter()
                .map(|row| row.id)
                .collect();
            for id in &records {
                purge_record_selection(tx, r.account_id, *id).await?;
            }
            orm::execute(
                tx,
                &health_evidence::Entity::delete_many()
                    .filter(health_evidence::Column::UserId.eq(r.account_id))
                    .filter(
                        health_evidence::Column::SourceKind.eq(WithdrawnSourceKind::HealthRecords),
                    )
                    .into_query(),
            )
            .await?;
            orm::execute(
                tx,
                &clinical_note::Entity::delete_many()
                    .filter(clinical_note::Column::UserId.eq(r.account_id))
                    .filter(clinical_note::Column::SourceLabel.eq(NoteSourceLabel::HealthRecords))
                    .into_query(),
            )
            .await?;
            let mut cursor_update = user_privacy::Entity::update_many()
                .col_expr(
                    user_privacy::Column::EvidenceRevision,
                    greatest_expr(
                        Expr::col(user_privacy::Column::EvidenceRevision),
                        Expr::val(r.scope_version),
                    )
                    .add(1),
                )
                .col_expr(
                    user_privacy::Column::HealthRecordsGeneration,
                    Expr::col(user_privacy::Column::HealthRecordsGeneration).add(1),
                )
                .col_expr(
                    user_privacy::Column::HealthRecordsDisconnectedAt,
                    Expr::val(r.occurred_at),
                )
                .filter(user_privacy::Column::Id.eq(r.account_id))
                .into_query();
            cursor_update.returning_col(user_privacy::Column::EvidenceRevision);
            let cursor: i64 = orm::row(tx, &cursor_update)
                .await?
                .ok_or(sqlx::Error::RowNotFound)?
                .try_get(0)?;
            mark_records_privacy_deleted(tx, r.account_id, &records, cursor).await?;
        }
        Restriction::FollowUpPause | Restriction::FollowUpCancel => {
            let revision = orm::model::<FollowUpRevisionRow>(
                tx,
                &follow_up::Entity::find()
                    .select_only()
                    .column(follow_up::Column::Revision)
                    .filter(follow_up::Column::UserId.eq(r.account_id))
                    .filter(follow_up::Column::Id.eq(r.scope_id))
                    .into_query(),
            )
            .await?
            .map(|row| row.revision);
            if !restoring && revision != Some(r.scope_version) {
                return Ok(false);
            }
            let restriction = if r.operation == Restriction::FollowUpCancel {
                FollowUpRestriction::Cancel
            } else {
                FollowUpRestriction::Pause
            };
            replay_restriction(
                tx,
                r.account_id,
                r.account_incarnation,
                r.scope_id,
                restriction,
            )
            .await?;
        }
        Restriction::ProactiveWithdraw => {
            if r.scope_id != r.account_id {
                return Ok(false);
            }
            replay_settings_withdrawal(tx, r.account_id, r.account_incarnation).await?;
            orm::execute(
                tx,
                &command::Entity::update_many()
                    .col_expr(command::Column::State, Expr::val(command::State::Cancelled))
                    .col_expr(command::Column::LeaseId, Expr::null())
                    .col_expr(command::Column::LeaseUntil, Expr::null())
                    .col_expr(command::Column::StagedMemory, Expr::null())
                    .filter(command::Column::UserId.eq(r.account_id))
                    .filter(
                        command::Column::WorkKind
                            .is_in([command::WorkKind::Today, command::WorkKind::FollowUp]),
                    )
                    .filter(
                        command::Column::State
                            .is_in([command::State::Accepted, command::State::Running]),
                    )
                    .into_query(),
            )
            .await?;
        }
        Restriction::AccountDelete => {
            if r.scope_id != r.account_id {
                return Ok(false);
            }
            let select_user = user_privacy::Entity::find()
                .select_only()
                .column(user_privacy::Column::AuthUserId)
                .column(user_privacy::Column::Id)
                .column(user_privacy::Column::DuetIncarnation)
                .expr(Expr::val(r.occurred_at))
                .filter(user_privacy::Column::Id.eq(r.account_id))
                .into_query();
            let insert_deleted = Query::insert()
                .into_table(deleted_identity::Entity.table_ref())
                .columns([
                    deleted_identity::Column::AuthUserId,
                    deleted_identity::Column::AccountId,
                    deleted_identity::Column::Incarnation,
                    deleted_identity::Column::DeletedAt,
                ])
                .select_from(select_user)
                .map_err(|error| ApiError::Storage(error.to_string()))?
                .on_conflict(
                    OnConflict::column(deleted_identity::Column::AuthUserId)
                        .do_nothing()
                        .to_owned(),
                )
                .to_owned();
            orm::execute(tx, &insert_deleted).await?;
            orm::execute(
                tx,
                &doctor_item::Entity::delete_many()
                    .filter(doctor_item::Column::UserId.eq(r.account_id))
                    .into_query(),
            )
            .await?;
            orm::execute(
                tx,
                &doctor_save::Entity::delete_many()
                    .filter(doctor_save::Column::UserId.eq(r.account_id))
                    .into_query(),
            )
            .await?;
            orm::execute(
                tx,
                &doctor_proposal::Entity::delete_many()
                    .filter(doctor_proposal::Column::UserId.eq(r.account_id))
                    .into_query(),
            )
            .await?;
            orm::execute(
                tx,
                &device_snapshot::Entity::delete_many()
                    .filter(device_snapshot::Column::UserId.eq(r.account_id))
                    .into_query(),
            )
            .await?;
            orm::execute(
                tx,
                &publication::Entity::delete_many()
                    .filter(publication::Column::UserId.eq(r.account_id))
                    .into_query(),
            )
            .await?;
            orm::execute(
                tx,
                &health_evidence::Entity::delete_many()
                    .filter(health_evidence::Column::UserId.eq(r.account_id))
                    .into_query(),
            )
            .await?;
            orm::execute(
                tx,
                &clinical_note::Entity::delete_many()
                    .filter(clinical_note::Column::UserId.eq(r.account_id))
                    .into_query(),
            )
            .await?;
            orm::execute(
                tx,
                &user_privacy::Entity::delete_many()
                    .filter(user_privacy::Column::Id.eq(r.account_id))
                    .into_query(),
            )
            .await?;
        }
        Restriction::EvidenceDelete => {
            let latest = orm::model::<MaxRevisionRow>(
                tx,
                &revision::Entity::find()
                    .select_only()
                    .expr_as(Expr::col(revision::Column::Revision).max(), "max_revision")
                    .filter(revision::Column::UserId.eq(r.account_id))
                    .filter(Expr::expr(revision_record_id()).eq(r.scope_id))
                    .into_query(),
            )
            .await?
            .and_then(|row| row.max_revision);
            // Restored SQL may predate the final correction that was deleted.
            // The same canonical incarnation at an earlier revision is still covered;
            // later source incarnations/revisions are never erased by an old receipt.
            if !latest.is_some_and(|v| v == r.scope_version || (restoring && v < r.scope_version)) {
                return Ok(false);
            }
            purge_record_selection(tx, r.account_id, r.scope_id).await?;
            orm::execute(
                tx,
                &health_evidence::Entity::delete_many()
                    .filter(health_evidence::Column::UserId.eq(r.account_id))
                    .filter(health_evidence::Column::Id.eq(r.scope_id))
                    .into_query(),
            )
            .await?;
            orm::execute(
                tx,
                &clinical_note::Entity::delete_many()
                    .filter(clinical_note::Column::UserId.eq(r.account_id))
                    .filter(clinical_note::Column::Id.eq(r.scope_id))
                    .into_query(),
            )
            .await?;
            let revision = advance_evidence_revision(tx, r.account_id).await?;
            mark_records_privacy_deleted(tx, r.account_id, &[r.scope_id], revision).await?;
        }
    }
    Ok(true)
}

// Follow-up restriction replay, kept with the journal that owns it. The
// follow-up tables stay, so every receipt kind still replays its effects,
// through typed statements on the entities in `follow_up_entity`.
type Tx<'a> = sqlx::Transaction<'a, sqlx::Postgres>;

/// What a follow-up restriction receipt does to its follow-up.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FollowUpRestriction {
    /// Hold an active follow-up.
    Pause,
    /// End the follow-up for good.
    Cancel,
}

async fn lock_account(tx: &mut Tx<'_>, account: Uuid) -> Result<(), ApiError> {
    orm::row(
        tx,
        &account::Entity::find_by_id(account)
            .select_only()
            .column(account::Column::Id)
            .lock_exclusive()
            .into_query(),
    )
    .await?
    .ok_or(ApiError::NotFound)?;
    Ok(())
}
fn now() -> Expr {
    Func::cust("now").into()
}
async fn notify(tx: &mut Tx<'_>, account: Uuid, id: Uuid) -> Result<(), ApiError> {
    let changed = Func::cust("jsonb_build_object").args([
        Expr::val("type"),
        Expr::val("follow_up_changed"),
        Expr::val("follow_up_id"),
        Expr::val(id),
    ]);
    let commands = tool_effect::Entity::find()
        .select_only()
        .distinct()
        .column(tool_effect::Column::UserId)
        .column(tool_effect::Column::CommandId)
        .expr(changed)
        .filter(tool_effect::Column::UserId.eq(account))
        .filter(
            Expr::col((tool_effect::Entity, tool_effect::Column::Call))
                .cast_json_field("name")
                .is_in(tool_effect::FOLLOW_UP_TOOLS),
        )
        .filter(
            Expr::col((tool_effect::Entity, tool_effect::Column::Result))
                .cast_json_field("id")
                .eq(Expr::val(id).cast_as("text")),
        )
        .into_query();
    let insert = Query::insert()
        .into_table(conversation_event::Entity.table_ref())
        .columns([
            conversation_event::Column::UserId,
            conversation_event::Column::CommandId,
            conversation_event::Column::Event,
        ])
        .select_from(commands)
        .map_err(|error| ApiError::Storage(error.to_string()))?
        .to_owned();
    orm::execute(tx, &insert).await?;
    Ok(())
}
async fn interrupt(tx: &mut Tx<'_>, account: Uuid, id: Uuid) -> Result<(), ApiError> {
    // Wake clients of original action messages without rewriting historical text.
    notify(tx, account, id).await?;
    let carries_this_follow_up = occurrence::Entity::find()
        .select_only()
        .expr(Expr::val(1))
        .filter(
            Expr::col((occurrence::Entity, occurrence::Column::UserId))
                .equals((command::Entity, command::Column::UserId)),
        )
        .filter(
            Expr::col((occurrence::Entity, occurrence::Column::CommandId))
                .equals((command::Entity, command::Column::Id)),
        )
        .filter(occurrence::Column::FollowUpId.eq(id))
        .into_query();
    orm::execute(
        tx,
        &command::Entity::update_many()
            .col_expr(command::Column::State, Expr::val(command::State::Cancelled))
            .col_expr(command::Column::LeaseId, Expr::null())
            .col_expr(command::Column::LeaseUntil, Expr::null())
            .col_expr(command::Column::StagedMemory, Expr::null())
            .filter(command::Column::UserId.eq(account))
            .filter(
                command::Column::State.is_in([command::State::Accepted, command::State::Running]),
            )
            .filter(Expr::exists(carries_this_follow_up))
            .into_query(),
    )
    .await?;
    orm::execute(
        tx,
        &occurrence::Entity::update_many()
            .col_expr(
                occurrence::Column::Status,
                Expr::val(occurrence::Status::Interrupted),
            )
            .col_expr(occurrence::Column::CompletedAt, now())
            .col_expr(occurrence::Column::UpdatedAt, now())
            .filter(occurrence::Column::UserId.eq(account))
            .filter(occurrence::Column::FollowUpId.eq(id))
            .filter(
                occurrence::Column::Status
                    .is_in([occurrence::Status::Queued, occurrence::Status::Running]),
            )
            .into_query(),
    )
    .await?;
    Ok(())
}
/// Called ONLY by privacy replay after receipt byte verification and incarnation
/// validation. It never grants authority, resumes work, or reopens ended work.
pub async fn replay_restriction(
    tx: &mut Tx<'_>,
    account: Uuid,
    incarnation: Uuid,
    id: Uuid,
    restriction: FollowUpRestriction,
) -> Result<(), ApiError> {
    lock_account(tx, account).await?;
    let current: Uuid = orm::row(
        tx,
        &account::Entity::find_by_id(account)
            .select_only()
            .column(account::Column::DuetIncarnation)
            .into_query(),
    )
    .await?
    .ok_or(sqlx::Error::RowNotFound)?
    .try_get(0)?;
    if current != incarnation {
        return Ok(());
    }
    // Cancel ends anything not already cancelled; pause holds only active work.
    let restrict = match restriction {
        FollowUpRestriction::Cancel => follow_up::Entity::update_many()
            .col_expr(
                follow_up::Column::State,
                Expr::val(follow_up::State::Cancelled),
            )
            .col_expr(follow_up::Column::ConfirmationRequired, Expr::val(false))
            .filter(follow_up::Column::State.ne(follow_up::State::Cancelled)),
        FollowUpRestriction::Pause => follow_up::Entity::update_many()
            .col_expr(
                follow_up::Column::State,
                Expr::val(follow_up::State::Paused),
            )
            .filter(follow_up::Column::State.eq(follow_up::State::Active)),
    };
    orm::execute(
        tx,
        &restrict
            .col_expr(
                follow_up::Column::Revision,
                Expr::col(follow_up::Column::Revision).add(1),
            )
            .col_expr(follow_up::Column::NextOccurrenceAt, Expr::null())
            .col_expr(follow_up::Column::UpdatedAt, now())
            .filter(follow_up::Column::UserId.eq(account))
            .filter(follow_up::Column::Id.eq(id))
            .into_query(),
    )
    .await?;
    interrupt(tx, account, id).await
}
/// Privacy-only monotonic settings replay after independent receipt validation.
pub async fn replay_settings_withdrawal(
    tx: &mut Tx<'_>,
    account: Uuid,
    incarnation: Uuid,
) -> Result<(), ApiError> {
    lock_account(tx, account).await?;
    let this_incarnation = account::Entity::find()
        .select_only()
        .column(account::Column::Id)
        .expr(Expr::val(false))
        .filter(account::Column::Id.eq(account))
        .filter(account::Column::DuetIncarnation.eq(incarnation))
        .into_query();
    // The revision rises only when the hold actually changes.
    let withdraw = OnConflict::column(settings::Column::UserId)
        .value(settings::Column::Enabled, false)
        .value(
            settings::Column::Revision,
            Expr::col((settings::Entity, settings::Column::Revision))
                .add(Expr::col((settings::Entity, settings::Column::Enabled)).cast_as("int")),
        )
        .to_owned();
    let insert = Query::insert()
        .into_table(settings::Entity.table_ref())
        .columns([settings::Column::UserId, settings::Column::Enabled])
        .select_from(this_incarnation)
        .map_err(|error| ApiError::Storage(error.to_string()))?
        .on_conflict(withdraw)
        .to_owned();
    orm::execute(tx, &insert).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        database::migrate,
        evidence::{EvidenceMutation, EvidenceStore},
        identity::{IdentityStore, IdentityVerifier, VerifiedIdentity},
        producer_database::{ProducerKind, grant},
        publication::{PublicationStore, PublishRequest},
        workspace::WorkspaceStore,
    };
    use duet_wire::models::{
        CanonicalRecordSelection, DoctorPageContent, DoctorPageSave,
        addressed_record_revision::Entity, canonical_record_selection::Kind,
    };
    use sea_orm::{ActiveEnum, Iterable};
    use serde_json::json;
    use sqlx::{ConnectOptions, Connection, PgConnection};
    use tokio::sync::{Mutex, MutexGuard};

    static TEST_GATE: Mutex<()> = Mutex::const_new(());
    const TEST_DB_NAME: &str = "privacy_unit_db";
    const TEST_ROLE_NAME: &str = "privacy_unit_role";

    struct TestVerifier;
    #[async_trait]
    impl IdentityVerifier for TestVerifier {
        async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, ApiError> {
            Ok(VerifiedIdentity {
                subject: format!("privacy-test-{bearer}"),
                authenticated_at: Utc::now(),
            })
        }
    }

    struct PrivacyFixture {
        _gate: MutexGuard<'static, ()>,
        admin: PgConnection,
        owner: PgConnection,
        journal_dir: PathBuf,
        db: AccountDatabase,
        identities: Arc<IdentityStore>,
        privacy: PrivacyStore,
        a: AccountContext,
        b: AccountContext,
    }

    impl PrivacyFixture {
        async fn setup(grant_accepted_work: bool) -> Option<Self> {
            let url = std::env::var("DUET_TEST_DATABASE_URL")
                .or_else(|_| std::env::var("MIGRATION_DATABASE_URL"))
                .ok()
                .filter(|v| !v.trim().is_empty());
            let Some(url) = url else {
                assert!(
                    std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
                    "DUET_TEST_DATABASE_URL or MIGRATION_DATABASE_URL is required"
                );
                return None;
            };
            let gate = TEST_GATE.lock().await;
            let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
            assert!(matches!(
                options.get_host(),
                "127.0.0.1" | "localhost" | "::1"
            ));
            let mut admin = PgConnection::connect(&url).await.unwrap();
            sqlx::raw_sql("drop database if exists privacy_unit_db with (force)")
                .execute(&mut admin)
                .await
                .unwrap();
            sqlx::raw_sql("drop role if exists privacy_unit_role")
                .execute(&mut admin)
                .await
                .unwrap();
            sqlx::raw_sql("create role privacy_unit_role login")
                .execute(&mut admin)
                .await
                .unwrap();
            sqlx::raw_sql("create database privacy_unit_db")
                .execute(&mut admin)
                .await
                .unwrap();
            let db_options = options.database(TEST_DB_NAME);
            let owner_url = db_options.to_url_lossy().to_string();
            let runtime_url = db_options
                .clone()
                .username(TEST_ROLE_NAME)
                .to_url_lossy()
                .to_string();
            let mut owner = PgConnection::connect(&owner_url).await.unwrap();
            sqlx::raw_sql("revoke create on schema public from public")
                .execute(&mut owner)
                .await
                .unwrap();
            migrate(&owner_url, TEST_ROLE_NAME).await.unwrap();
            if grant_accepted_work {
                grant(&owner_url, TEST_ROLE_NAME, ProducerKind::AcceptedWork)
                    .await
                    .unwrap();
            }
            let suffix = Uuid::new_v4().simple().to_string();
            let db = AccountDatabase::connect(&runtime_url).await.unwrap();
            let identities = Arc::new(IdentityStore::new(db.clone(), Arc::new(TestVerifier)));
            let a = identities.authenticate("a", true).await.unwrap();
            let b = identities.authenticate("b", true).await.unwrap();
            let journal_dir = std::env::temp_dir().join(format!("privacy-unit-journal-{suffix}"));
            let journal = Arc::new(FileJournal::open(journal_dir.clone()).await.unwrap());
            let privacy = PrivacyStore::new(db.clone(), journal);
            Some(Self {
                _gate: gate,
                admin,
                owner,
                journal_dir,
                db,
                identities,
                privacy,
                a,
                b,
            })
        }

        async fn cleanup(mut self) {
            drop(self.privacy);
            drop(self.identities);
            drop(self.db);
            self.owner.close().await.unwrap();
            sqlx::raw_sql("drop database privacy_unit_db with (force)")
                .execute(&mut self.admin)
                .await
                .unwrap();
            sqlx::raw_sql("drop role privacy_unit_role")
                .execute(&mut self.admin)
                .await
                .unwrap();
            let _ = tokio::fs::remove_dir_all(self.journal_dir).await;
        }
    }

    #[test]
    fn restriction_enum_matches_wire_snake_case() {
        for variant in Restriction::iter() {
            let wire = serde_json::to_value(variant).unwrap();
            assert_eq!(wire.as_str(), Some(variant.to_value().as_str()));
        }
    }

    #[tokio::test]
    async fn receipt_luna_withdraw() {
        let Some(mut fx) = PrivacyFixture::setup(false).await else {
            return;
        };
        let conv = Uuid::new_v4();
        let cmd = Uuid::new_v4();
        sqlx::query("update users set intelligence_enabled=true,luna_consent_decision='accepted',luna_consent_version='2026-09-24',intelligence_control_revision=1 where id=$1")
            .bind(fx.a.account_id)
            .execute(&mut fx.owner)
            .await
            .unwrap();
        sqlx::query("insert into duet_conversations(user_id,id) values($1,$2)")
            .bind(fx.a.account_id)
            .bind(conv)
            .execute(&mut fx.owner)
            .await
            .unwrap();
        sqlx::query("insert into duet_commands(user_id,id,conversation_id,context,input,base_revision,request,state,lease_until,staged_memory) values($1,$2,$3,'{}','hi',0,'{}','running',now(),'{\"k\":1}')")
            .bind(fx.a.account_id)
            .bind(cmd)
            .bind(conv)
            .execute(&mut fx.owner)
            .await
            .unwrap();

        let event = Uuid::new_v4();
        fx.privacy
            .restrict(&fx.a, event, Restriction::LunaWithdraw, fx.a.account_id, 1)
            .await
            .unwrap();

        let (enabled, decision, rev): (bool, Option<String>, i64) = sqlx::query_as(
            "select intelligence_enabled,luna_consent_decision,intelligence_control_revision from users where id=$1",
        )
        .bind(fx.a.account_id)
        .fetch_one(&mut fx.owner)
        .await
        .unwrap();
        let (cmd_state, cleared): (String, bool) = sqlx::query_as(
            "select state,lease_until is null and staged_memory is null from duet_commands where id=$1",
        )
        .bind(cmd)
        .fetch_one(&mut fx.owner)
        .await
        .unwrap();
        assert!(
            !enabled
                && decision.as_deref() == Some("declined")
                && rev == 2
                && cmd_state == "cancelled"
                && cleared,
            "luna_withdraw must disable intelligence, decline consent, advance revision, and cancel commands"
        );
        fx.cleanup().await;
    }

    #[tokio::test]
    async fn receipt_sessions_revoke() {
        let Some(mut fx) = PrivacyFixture::setup(false).await else {
            return;
        };
        let prior = fx.identities.authenticate("a", false).await.unwrap();
        assert_eq!(prior.account_id, fx.a.account_id);
        let event = Uuid::new_v4();
        fx.privacy
            .restrict(
                &fx.a,
                event,
                Restriction::SessionsRevoke,
                fx.a.account_id,
                0,
            )
            .await
            .unwrap();
        let cutoff_set: bool =
            sqlx::query_scalar("select sessions_revoked_before is not null from users where id=$1")
                .bind(fx.a.account_id)
                .fetch_one(&mut fx.owner)
                .await
                .unwrap();
        assert!(
            cutoff_set,
            "sessions_revoke must record sessions_revoked_before cutoff"
        );
        fx.cleanup().await;
    }

    #[tokio::test]
    async fn receipt_evidence_delete() {
        let Some(mut fx) = PrivacyFixture::setup(false).await else {
            return;
        };
        let evidence = EvidenceStore::new(fx.db.clone());
        let workspace = WorkspaceStore::new(fx.db.clone());
        let addr = evidence
            .ingest(
                &fx.a,
                &EvidenceMutation {
                    operation_id: Uuid::new_v4(),
                    canonical_record_id: Uuid::new_v4(),
                    record_id: "src/ev-1".to_owned(),
                    entity: Entity::HealthEvidence,
                    expected_revision: None,
                    source_generation: None,
                    operation: Default::default(),
                    payload: json!({"concept_id":"lab:k","source":{"kind":"manual"},"value":{"literal":"4.2"}}),
                },
            )
            .await
            .unwrap();
        workspace
            .select_record(
                &fx.a,
                &CanonicalRecordSelection::new(
                    Kind::Record,
                    addr.canonical_record_id,
                    addr.revision,
                ),
            )
            .await
            .unwrap();

        fx.privacy
            .restrict(
                &fx.a,
                Uuid::new_v4(),
                Restriction::EvidenceDelete,
                addr.canonical_record_id,
                i64::from(addr.revision),
            )
            .await
            .unwrap();

        let remaining: i64 = sqlx::query_scalar(
            "select (select count(*) from health_evidence where user_id=$1 and id=$2) + (select count(*) from canonical_evidence_revisions where user_id=$1 and evidence_id=$2) + (select count(*) from doctor_collection_items where user_id=$1)",
        )
        .bind(fx.a.account_id)
        .bind(addr.canonical_record_id)
        .fetch_one(&mut fx.owner)
        .await
        .unwrap();
        let (deleted, source_removed): (bool, bool) = sqlx::query_as(
            "select deleted,source_removed from evidence_sync_changes where user_id=$1 and record_id=$2",
        )
        .bind(fx.a.account_id)
        .bind(addr.canonical_record_id)
        .fetch_one(&mut fx.owner)
        .await
        .unwrap();
        assert!(
            remaining == 0 && deleted && !source_removed,
            "evidence_delete must erase canonical evidence, purge selections, and mark tombstoned membership"
        );
        fx.cleanup().await;
    }

    #[tokio::test]
    async fn receipt_doctor_note_delete() {
        let Some(mut fx) = PrivacyFixture::setup(false).await else {
            return;
        };
        sqlx::query("update users set doctor_note='Private concern',doctor_proposal='{\"id\":\"p\"}',doctor_note_generation=2 where id=$1")
            .bind(fx.a.account_id)
            .execute(&mut fx.owner)
            .await
            .unwrap();
        sqlx::query("insert into duet_doctor_saves(user_id,expected_generation,request,result) values($1,2,'{}','{}'),($1,3,'{}','{}')")
            .bind(fx.a.account_id)
            .execute(&mut fx.owner)
            .await
            .unwrap();

        fx.privacy
            .restrict(
                &fx.a,
                Uuid::new_v4(),
                Restriction::DoctorNoteDelete,
                fx.a.account_id,
                2,
            )
            .await
            .unwrap();

        let (note_cleared, proposal_cleared, generation): (bool, bool, i64) = sqlx::query_as(
            "select doctor_note is null,doctor_proposal is null,doctor_note_generation from users where id=$1",
        )
        .bind(fx.a.account_id)
        .fetch_one(&mut fx.owner)
        .await
        .unwrap();
        let saves_left: Vec<i64> = sqlx::query_scalar(
            "select expected_generation from duet_doctor_saves where user_id=$1 order by expected_generation",
        )
        .bind(fx.a.account_id)
        .fetch_all(&mut fx.owner)
        .await
        .unwrap();
        assert!(
            note_cleared && proposal_cleared && generation == 3 && saves_left == vec![3],
            "doctor_note_delete must clear doctor_note, proposal, and saves through scope_version"
        );
        fx.cleanup().await;
    }

    #[tokio::test]
    async fn receipt_doctor_page_delete() {
        let Some(mut fx) = PrivacyFixture::setup(false).await else {
            return;
        };
        let evidence = EvidenceStore::new(fx.db.clone());
        let workspace = WorkspaceStore::new(fx.db.clone());
        let publications = PublicationStore::new(
            fx.db.clone(),
            "https://duet.example".into(),
            "https://viewer.example/".into(),
            [9; 32],
        )
        .unwrap();
        let addr = evidence
            .ingest(
                &fx.a,
                &EvidenceMutation {
                    operation_id: Uuid::new_v4(),
                    canonical_record_id: Uuid::new_v4(),
                    record_id: "src/page-1".to_owned(),
                    entity: Entity::HealthEvidence,
                    expected_revision: None,
                    source_generation: None,
                    operation: Default::default(),
                    payload: json!({"concept_id":"lab:na","source":{"kind":"manual"},"value":{"literal":"140"}}),
                },
            )
            .await
            .unwrap();
        let page = workspace
            .select_record(
                &fx.a,
                &CanonicalRecordSelection::new(
                    Kind::Record,
                    addr.canonical_record_id,
                    addr.revision,
                ),
            )
            .await
            .unwrap();
        let item_id: Uuid = serde_json::from_value(page["items"][0]["id"].clone()).unwrap();
        let mut content = DoctorPageContent::new();
        content.name = Some(Some("Alex".into()));
        content.cover_note = Some(Some("Cover".into()));
        let saved = workspace
            .save(
                &fx.a,
                &DoctorPageSave::new(
                    page["note_generation"].as_i64().unwrap() as i32,
                    vec![item_id],
                    content,
                ),
            )
            .await
            .unwrap();
        let generation = saved["note_generation"].as_i64().unwrap();
        let pub_view = publications
            .publish(
                &fx.a,
                &PublishRequest {
                    operation_id: Uuid::new_v4(),
                    expected_generation: generation,
                    expires_in_seconds: 3600,
                },
            )
            .await
            .unwrap();

        fx.privacy
            .restrict(
                &fx.a,
                Uuid::new_v4(),
                Restriction::DoctorPageDelete,
                fx.a.account_id,
                generation,
            )
            .await
            .unwrap();

        let leftover: i64 = sqlx::query_scalar(
            "select (select count(*) from doctor_collection_items where user_id=$1) + (select count(*) from duet_doctor_saves where user_id=$1) + (select count(*) from duet_doctor_proposals where user_id=$1)",
        )
        .bind(fx.a.account_id)
        .fetch_one(&mut fx.owner)
        .await
        .unwrap();
        let (note_null, page_json, next_gen): (bool, serde_json::Value, i64) = sqlx::query_as(
            "select doctor_note is null,doctor_page,doctor_note_generation from users where id=$1",
        )
        .bind(fx.a.account_id)
        .fetch_one(&mut fx.owner)
        .await
        .unwrap();
        let revoked: bool =
            sqlx::query_scalar("select revoked_at is not null from duet_publications where id=$1")
                .bind(pub_view.id)
                .fetch_one(&mut fx.owner)
                .await
                .unwrap();
        assert!(
            leftover == 0
                && note_null
                && page_json == json!({})
                && next_gen == generation + 1
                && revoked,
            "doctor_page_delete must clear workspace items, saves, page json, and revoke publications"
        );
        fx.cleanup().await;
    }

    #[tokio::test]
    async fn receipt_doctor_item_delete() {
        let Some(mut fx) = PrivacyFixture::setup(false).await else {
            return;
        };
        let target_item = Uuid::new_v4();
        let keep_item = Uuid::new_v4();
        let prop_match = Uuid::new_v4();
        let prop_keep = Uuid::new_v4();
        let pub_id = Uuid::new_v4();
        sqlx::query("insert into doctor_collection_items(id,user_id,kind,source_id,title,detail,position) values($1,$3,'record','s1','T1','D1',0),($2,$3,'record','s2','T2','D2',1)")
            .bind(target_item)
            .bind(keep_item)
            .bind(fx.a.account_id)
            .execute(&mut fx.owner)
            .await
            .unwrap();
        sqlx::query("insert into duet_doctor_saves(user_id,expected_generation,request,result) values($1,1,'{}',$2),($1,2,'{}',$3)")
            .bind(fx.a.account_id)
            .bind(json!({"items":[{"id":target_item}]}))
            .bind(json!({"items":[{"id":keep_item}]}))
            .execute(&mut fx.owner)
            .await
            .unwrap();
        sqlx::query("insert into duet_doctor_proposals(user_id,id,run_id,proposal,state) values($1,$2,$3,$4,'pending'),($1,$5,$6,$7,'pending')")
            .bind(fx.a.account_id)
            .bind(prop_match)
            .bind(Uuid::new_v4())
            .bind(json!({"items":[{"id":target_item}]}))
            .bind(prop_keep)
            .bind(Uuid::new_v4())
            .bind(json!({"items":[{"id":keep_item}]}))
            .execute(&mut fx.owner)
            .await
            .unwrap();
        sqlx::query("insert into duet_publications(id,user_id,operation_id,request,generation,token_hash,shl_hash,protected_capabilities,snapshot,pdf,digest,expires_at) values($1,$2,$3,'{}',1,'th1','sh1','\\x00',$4,'\\x00','d1',now()+interval '1 hour')")
            .bind(pub_id)
            .bind(fx.a.account_id)
            .bind(Uuid::new_v4())
            .bind(json!({"items":[{"id":target_item}]}))
            .execute(&mut fx.owner)
            .await
            .unwrap();

        fx.privacy
            .restrict(
                &fx.a,
                Uuid::new_v4(),
                Restriction::DoctorItemDelete,
                target_item,
                0,
            )
            .await
            .unwrap();

        let remaining_items: Vec<Uuid> = sqlx::query_scalar(
            "select id from doctor_collection_items where user_id=$1 order by position",
        )
        .bind(fx.a.account_id)
        .fetch_all(&mut fx.owner)
        .await
        .unwrap();
        let remaining_saves: Vec<i64> = sqlx::query_scalar(
            "select expected_generation from duet_doctor_saves where user_id=$1",
        )
        .bind(fx.a.account_id)
        .fetch_all(&mut fx.owner)
        .await
        .unwrap();
        let (matched_state, kept_state): (String, String) = sqlx::query_as(
            "select (select state from duet_doctor_proposals where id=$1),(select state from duet_doctor_proposals where id=$2)",
        )
        .bind(prop_match)
        .bind(prop_keep)
        .fetch_one(&mut fx.owner)
        .await
        .unwrap();
        let pub_revoked: bool =
            sqlx::query_scalar("select revoked_at is not null from duet_publications where id=$1")
                .bind(pub_id)
                .fetch_one(&mut fx.owner)
                .await
                .unwrap();
        assert!(
            remaining_items == vec![keep_item]
                && remaining_saves == vec![2]
                && matched_state == "superseded"
                && kept_state == "pending"
                && pub_revoked,
            "doctor_item_delete must delete target item, supersede matching proposals, and revoke matching publications"
        );
        fx.cleanup().await;
    }

    #[tokio::test]
    async fn receipt_publication_revoke() {
        let Some(mut fx) = PrivacyFixture::setup(false).await else {
            return;
        };
        let pub_id = Uuid::new_v4();
        sqlx::query("insert into duet_publications(id,user_id,operation_id,request,generation,token_hash,shl_hash,protected_capabilities,snapshot,pdf,digest,expires_at) values($1,$2,$3,'{}',1,'th-pub','sh-pub','\\x00','{}','\\x00','d1',now()+interval '1 hour')")
            .bind(pub_id)
            .bind(fx.a.account_id)
            .bind(Uuid::new_v4())
            .execute(&mut fx.owner)
            .await
            .unwrap();

        fx.privacy
            .restrict(
                &fx.a,
                Uuid::new_v4(),
                Restriction::PublicationRevoke,
                pub_id,
                0,
            )
            .await
            .unwrap();

        let revoked: bool =
            sqlx::query_scalar("select revoked_at is not null from duet_publications where id=$1")
                .bind(pub_id)
                .fetch_one(&mut fx.owner)
                .await
                .unwrap();
        assert!(
            revoked,
            "publication_revoke must set revoked_at on target publication"
        );
        fx.cleanup().await;
    }

    #[tokio::test]
    async fn receipt_device_withdraw() {
        let Some(mut fx) = PrivacyFixture::setup(false).await else {
            return;
        };
        let evidence = EvidenceStore::new(fx.db.clone());
        let workspace = WorkspaceStore::new(fx.db.clone());
        let apple = evidence
            .ingest(
                &fx.a,
                &EvidenceMutation {
                    operation_id: Uuid::new_v4(),
                    canonical_record_id: Uuid::new_v4(),
                    record_id: "ah/1".to_owned(),
                    entity: Entity::HealthEvidence,
                    expected_revision: None,
                    source_generation: Some(0),
                    operation: Default::default(),
                    payload: json!({"concept_id":"vital:hr","source":{"kind":"appleHealth"},"value":{"literal":"64"}}),
                },
            )
            .await
            .unwrap();
        let manual = evidence
            .ingest(
                &fx.a,
                &EvidenceMutation {
                    operation_id: Uuid::new_v4(),
                    canonical_record_id: Uuid::new_v4(),
                    record_id: "man/1".to_owned(),
                    entity: Entity::HealthEvidence,
                    expected_revision: None,
                    source_generation: None,
                    operation: Default::default(),
                    payload: json!({"concept_id":"vital:wt","source":{"kind":"manual"},"value":{"literal":"70"}}),
                },
            )
            .await
            .unwrap();
        workspace
            .upload_device(
                &fx.a,
                &json!({"snapshot_id":Uuid::new_v4(),"generated_at":"2026-10-20T12:00:00Z","facts":[{"id":"watch/steps","title":"Steps","value":1200,"unit":"count","range_start":"2026-10-19T00:00:00Z","range_end":"2026-10-20T00:00:00Z"}]}),
            )
            .await
            .unwrap();
        workspace.select_fact(&fx.a, "watch/steps").await.unwrap();

        fx.privacy
            .restrict(
                &fx.a,
                Uuid::new_v4(),
                Restriction::DeviceWithdraw,
                fx.a.account_id,
                0,
            )
            .await
            .unwrap();

        let remaining_evidence: Vec<Uuid> =
            sqlx::query_scalar("select id from health_evidence where user_id=$1")
                .bind(fx.a.account_id)
                .fetch_all(&mut fx.owner)
                .await
                .unwrap();
        let snapshots_left: i64 =
            sqlx::query_scalar("select count(*) from duet_device_snapshots where user_id=$1")
                .bind(fx.a.account_id)
                .fetch_one(&mut fx.owner)
                .await
                .unwrap();
        let (generation, disconnected): (i32, bool) = sqlx::query_as(
            "select apple_health_generation,apple_health_disconnected_at is not null from users where id=$1",
        )
        .bind(fx.a.account_id)
        .fetch_one(&mut fx.owner)
        .await
        .unwrap();
        let apple_tombstone: (bool, bool) = sqlx::query_as(
            "select deleted,source_removed from evidence_sync_changes where user_id=$1 and record_id=$2",
        )
        .bind(fx.a.account_id)
        .bind(apple.canonical_record_id)
        .fetch_one(&mut fx.owner)
        .await
        .unwrap();
        assert!(
            remaining_evidence == vec![manual.canonical_record_id]
                && snapshots_left == 0
                && generation == 1
                && disconnected
                && apple_tombstone == (true, false),
            "device_withdraw must delete appleHealth records, device snapshots, and advance apple_health_generation"
        );
        fx.cleanup().await;
    }

    #[tokio::test]
    async fn receipt_records_withdraw() {
        let Some(mut fx) = PrivacyFixture::setup(false).await else {
            return;
        };
        let evidence = EvidenceStore::new(fx.db.clone());
        let hr_ev = evidence
            .ingest(
                &fx.a,
                &EvidenceMutation {
                    operation_id: Uuid::new_v4(),
                    canonical_record_id: Uuid::new_v4(),
                    record_id: "hr/ev-1".to_owned(),
                    entity: Entity::HealthEvidence,
                    expected_revision: None,
                    source_generation: Some(0),
                    operation: Default::default(),
                    payload: json!({"concept_id":"lab:a1c","source":{"kind":"healthRecords"},"value":{"literal":"5.4"}}),
                },
            )
            .await
            .unwrap();
        let hr_note = evidence
            .ingest(
                &fx.a,
                &EvidenceMutation {
                    operation_id: Uuid::new_v4(),
                    canonical_record_id: Uuid::new_v4(),
                    record_id: "hr/note-1".to_owned(),
                    entity: Entity::ClinicalNote,
                    expected_revision: None,
                    source_generation: Some(0),
                    operation: Default::default(),
                    payload: json!({"title":"Clinic Summary","body":"Imported","recorded_at":"2026-10-01T00:00:00Z"}),
                },
            )
            .await
            .unwrap();
        let duet_note = evidence
            .ingest(
                &fx.a,
                &EvidenceMutation {
                    operation_id: Uuid::new_v4(),
                    canonical_record_id: Uuid::new_v4(),
                    record_id: "duet/note-1".to_owned(),
                    entity: Entity::ClinicalNote,
                    expected_revision: None,
                    source_generation: None,
                    operation: Default::default(),
                    payload: json!({"title":"My Note","body":"Patient authored","source_label":"Added in Duet","recorded_at":"2026-10-02T00:00:00Z"}),
                },
            )
            .await
            .unwrap();

        let cursor: i64 = sqlx::query_scalar("select evidence_revision from users where id=$1")
            .bind(fx.a.account_id)
            .fetch_one(&mut fx.owner)
            .await
            .unwrap();

        fx.privacy
            .restrict(
                &fx.a,
                Uuid::new_v4(),
                Restriction::RecordsWithdraw,
                fx.a.account_id,
                cursor,
            )
            .await
            .unwrap();

        let ev_left: i64 =
            sqlx::query_scalar("select count(*) from health_evidence where user_id=$1")
                .bind(fx.a.account_id)
                .fetch_one(&mut fx.owner)
                .await
                .unwrap();
        let notes_left: Vec<Uuid> =
            sqlx::query_scalar("select id from clinical_notes where user_id=$1")
                .bind(fx.a.account_id)
                .fetch_all(&mut fx.owner)
                .await
                .unwrap();
        let (hr_gen, disconnected): (i32, bool) = sqlx::query_as(
            "select health_records_generation,health_records_disconnected_at is not null from users where id=$1",
        )
        .bind(fx.a.account_id)
        .fetch_one(&mut fx.owner)
        .await
        .unwrap();
        let tombstoned: i64 = sqlx::query_scalar(
            "select count(*) from evidence_sync_changes where user_id=$1 and record_id=any($2) and deleted and not source_removed",
        )
        .bind(fx.a.account_id)
        .bind([hr_ev.canonical_record_id, hr_note.canonical_record_id])
        .fetch_one(&mut fx.owner)
        .await
        .unwrap();
        assert!(
            ev_left == 0
                && notes_left == vec![duet_note.canonical_record_id]
                && hr_gen == 1
                && disconnected
                && tombstoned == 2,
            "records_withdraw must erase healthRecords evidence and notes while preserving patient-authored notes"
        );
        fx.cleanup().await;
    }

    #[tokio::test]
    async fn receipt_follow_up_pause() {
        let Some(mut fx) = PrivacyFixture::setup(true).await else {
            return;
        };
        let follow = Uuid::new_v4();
        let conv = Uuid::new_v4();
        let cmd = Uuid::new_v4();
        sqlx::query("insert into follow_ups(id,user_id,accepted_instruction,schedule,schedule_summary,state,source_message_ids,next_occurrence_at,confirmation_required) values($1,$2,'Check in','{}','Daily','active','{}',now(),true)")
            .bind(follow)
            .bind(fx.a.account_id)
            .execute(&mut fx.owner)
            .await
            .unwrap();
        sqlx::query("insert into duet_conversations(user_id,id) values($1,$2)")
            .bind(fx.a.account_id)
            .bind(conv)
            .execute(&mut fx.owner)
            .await
            .unwrap();
        sqlx::query("insert into duet_commands(user_id,id,conversation_id,context,input,base_revision,request,state,work_kind,lease_id,lease_until,staged_memory) values($1,$2,$3,'{}','',0,'{}','running','follow_up',$2,now(),'{}')")
            .bind(fx.a.account_id)
            .bind(cmd)
            .bind(conv)
            .execute(&mut fx.owner)
            .await
            .unwrap();
        sqlx::query("insert into duet_follow_up_occurrences(user_id,id,follow_up_id,follow_up_revision,scheduled_for,command_id,operation,accepted_instruction,status) values($1,$2,$3,1,now(),$4,'check_in','Check in','running')")
            .bind(fx.a.account_id)
            .bind(Uuid::new_v4())
            .bind(follow)
            .bind(cmd)
            .execute(&mut fx.owner)
            .await
            .unwrap();
        sqlx::query("insert into duet_tool_effects(user_id,command_id,invocation_id,call,result) values($1,$2,'c1','{\"name\":\"create_follow_up\"}',jsonb_build_object('id',$3::text))")
            .bind(fx.a.account_id)
            .bind(cmd)
            .bind(follow)
            .execute(&mut fx.owner)
            .await
            .unwrap();

        fx.privacy
            .restrict(&fx.a, Uuid::new_v4(), Restriction::FollowUpPause, follow, 1)
            .await
            .unwrap();

        let (state, rev, next_null, confirm_req): (String, i64, bool, bool) = sqlx::query_as(
            "select state,revision,next_occurrence_at is null,confirmation_required from follow_ups where id=$1",
        )
        .bind(follow)
        .fetch_one(&mut fx.owner)
        .await
        .unwrap();
        let occ_status: String =
            sqlx::query_scalar("select status from duet_follow_up_occurrences where command_id=$1")
                .bind(cmd)
                .fetch_one(&mut fx.owner)
                .await
                .unwrap();
        assert!(
            state == "paused"
                && rev == 2
                && next_null
                && confirm_req
                && occ_status == "interrupted",
            "follow_up_pause must pause active follow_up and interrupt running occurrence"
        );
        fx.cleanup().await;
    }

    #[tokio::test]
    async fn receipt_follow_up_cancel() {
        let Some(mut fx) = PrivacyFixture::setup(true).await else {
            return;
        };
        let follow = Uuid::new_v4();
        sqlx::query("insert into follow_ups(id,user_id,accepted_instruction,schedule,schedule_summary,revision,state,source_message_ids,next_occurrence_at,confirmation_required) values($1,$2,'Check in','{}','Daily',2,'paused','{}',now(),true)")
            .bind(follow)
            .bind(fx.a.account_id)
            .execute(&mut fx.owner)
            .await
            .unwrap();

        fx.privacy
            .restrict(
                &fx.a,
                Uuid::new_v4(),
                Restriction::FollowUpCancel,
                follow,
                2,
            )
            .await
            .unwrap();

        let (state, rev, next_null, confirm_req): (String, i64, bool, bool) = sqlx::query_as(
            "select state,revision,next_occurrence_at is null,confirmation_required from follow_ups where id=$1",
        )
        .bind(follow)
        .fetch_one(&mut fx.owner)
        .await
        .unwrap();
        assert!(
            state == "cancelled" && rev == 3 && next_null && !confirm_req,
            "follow_up_cancel must cancel follow_up and clear confirmation_required"
        );
        fx.cleanup().await;
    }

    #[tokio::test]
    async fn receipt_proactive_withdraw() {
        let Some(mut fx) = PrivacyFixture::setup(true).await else {
            return;
        };
        let conv_today = Uuid::new_v4();
        let conv_msg = Uuid::new_v4();
        let cmd_today = Uuid::new_v4();
        let cmd_msg = Uuid::new_v4();
        sqlx::query("insert into duet_follow_up_settings(user_id,enabled) values($1,true)")
            .bind(fx.a.account_id)
            .execute(&mut fx.owner)
            .await
            .unwrap();
        sqlx::query("insert into duet_conversations(user_id,id) values($1,$2),($1,$3)")
            .bind(fx.a.account_id)
            .bind(conv_today)
            .bind(conv_msg)
            .execute(&mut fx.owner)
            .await
            .unwrap();
        sqlx::query("insert into duet_commands(user_id,id,conversation_id,context,input,base_revision,request,state,work_kind,today_assignment,lease_id,lease_until,staged_memory) values($1,$2,$3,'{}','',0,'{}','running','today','{\"slot\":1}',$2,now(),'{}')")
            .bind(fx.a.account_id)
            .bind(cmd_today)
            .bind(conv_today)
            .execute(&mut fx.owner)
            .await
            .unwrap();
        sqlx::query("insert into duet_commands(user_id,id,conversation_id,context,input,base_revision,request,state,work_kind,lease_id,lease_until) values($1,$2,$3,'{}','hello',0,'{}','running','message',$2,now())")
            .bind(fx.a.account_id)
            .bind(cmd_msg)
            .bind(conv_msg)
            .execute(&mut fx.owner)
            .await
            .unwrap();

        fx.privacy
            .restrict(
                &fx.a,
                Uuid::new_v4(),
                Restriction::ProactiveWithdraw,
                fx.a.account_id,
                0,
            )
            .await
            .unwrap();

        let (enabled, rev): (bool, i64) =
            sqlx::query_as("select enabled,revision from duet_follow_up_settings where user_id=$1")
                .bind(fx.a.account_id)
                .fetch_one(&mut fx.owner)
                .await
                .unwrap();
        let (today_state, msg_state): (String, String) = sqlx::query_as(
            "select (select state from duet_commands where id=$1),(select state from duet_commands where id=$2)",
        )
        .bind(cmd_today)
        .bind(cmd_msg)
        .fetch_one(&mut fx.owner)
        .await
        .unwrap();
        assert!(
            !enabled && rev == 2 && today_state == "cancelled" && msg_state == "running",
            "proactive_withdraw must disable follow_up_settings and cancel proactive commands only"
        );
        fx.cleanup().await;
    }

    #[tokio::test]
    async fn receipt_account_delete() {
        let Some(mut fx) = PrivacyFixture::setup(false).await else {
            return;
        };
        let evidence = EvidenceStore::new(fx.db.clone());
        evidence
            .ingest(
                &fx.a,
                &EvidenceMutation {
                    operation_id: Uuid::new_v4(),
                    canonical_record_id: Uuid::new_v4(),
                    record_id: "del/a".to_owned(),
                    entity: Entity::HealthEvidence,
                    expected_revision: None,
                    source_generation: None,
                    operation: Default::default(),
                    payload: json!({"concept_id":"lab:glu","source":{"kind":"manual"},"value":{"literal":"90"}}),
                },
            )
            .await
            .unwrap();
        evidence
            .ingest(
                &fx.b,
                &EvidenceMutation {
                    operation_id: Uuid::new_v4(),
                    canonical_record_id: Uuid::new_v4(),
                    record_id: "keep/b".to_owned(),
                    entity: Entity::HealthEvidence,
                    expected_revision: None,
                    source_generation: None,
                    operation: Default::default(),
                    payload: json!({"concept_id":"lab:glu","source":{"kind":"manual"},"value":{"literal":"95"}}),
                },
            )
            .await
            .unwrap();

        fx.privacy
            .restrict(
                &fx.a,
                Uuid::new_v4(),
                Restriction::AccountDelete,
                fx.a.account_id,
                0,
            )
            .await
            .unwrap();

        let a_user_left: i64 = sqlx::query_scalar("select count(*) from users where id=$1")
            .bind(fx.a.account_id)
            .fetch_one(&mut fx.owner)
            .await
            .unwrap();
        let tombstone: Option<(Uuid, Uuid)> = sqlx::query_as(
            "select account_id,incarnation from duet_deleted_identities where auth_user_id='privacy-test-a'",
        )
        .fetch_optional(&mut fx.owner)
        .await
        .unwrap();
        let a_ev_left: i64 =
            sqlx::query_scalar("select count(*) from health_evidence where user_id=$1")
                .bind(fx.a.account_id)
                .fetch_one(&mut fx.owner)
                .await
                .unwrap();
        let b_ev_left: i64 =
            sqlx::query_scalar("select count(*) from health_evidence where user_id=$1")
                .bind(fx.b.account_id)
                .fetch_one(&mut fx.owner)
                .await
                .unwrap();
        assert!(
            a_user_left == 0
                && tombstone == Some((fx.a.account_id, fx.a.incarnation))
                && a_ev_left == 0
                && b_ev_left == 1,
            "account_delete must record deleted identity tombstone and erase only target account rows"
        );
        fx.cleanup().await;
    }
}
