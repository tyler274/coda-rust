//! Luna's bounded privacy owner, not a general lifecycle or recovery service.
//! SQL commits the restriction and immutable event together. Only exact independent
//! readback may clear pending; upload acknowledgments are insufficient. No runtime
//! storage adapter is installed here, and hosted activation remains refused.
use crate::{
    PostgresFrontDoorStore,
    identity::{self, AuthorityError, Principal},
    migration::{account, permission, restriction},
};
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, DbBackend, EntityTrait,
    IntoActiveModel, QueryFilter, QuerySelect, Set, Statement, TransactionTrait,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

static DELIVERY_SLOTS: tokio::sync::Semaphore = tokio::sync::Semaphore::const_new(4);

/// Independent object-storage boundary. Implementations must bound actual I/O,
/// create with a non-overwrite precondition, and read at most 4097 bytes.
/// Futures must cancel their I/O when dropped; detached blocking work is forbidden.
/// No SQL connection is held while these methods run.
#[async_trait]
pub trait ReceiptArchive: Send + Sync {
    /// Create once at this stable key; an error may mean an ambiguous write.
    async fn create(&self, key: Uuid, bytes: &[u8]) -> Result<(), ArchiveUnavailable>;
    /// Read the exact independently retained object, including after ambiguity.
    async fn read(&self, key: Uuid) -> Result<Vec<u8>, ArchiveUnavailable>;
}
/// Archive failure, intentionally containing no credentials or payload.
#[derive(Debug)]
pub struct ArchiveUnavailable;

/// Minimal versioned receipt. This Rust namespace does not masquerade as the
/// reference SQL owner's wire schema. It contains no identity, prose or health data.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LunaReceipt {
    schema: String,
    event_id: Uuid,
    account_id: Uuid,
    revision: i64,
    occurred_at: i64,
}
impl LunaReceipt {
    fn from_row(row: &restriction::Model) -> Self {
        Self {
            schema: "duet-rust-luna-withdraw-v1".into(),
            event_id: row.id,
            account_id: row.account_id,
            revision: row.revision,
            occurred_at: row.occurred_at,
        }
    }
    /// Parse one independently retrieved receipt before isolated replay. Whole
    /// archive validation and coverage-marker qualification remain operator gates.
    pub fn parse(bytes: &[u8]) -> Result<Self, ArchiveUnavailable> {
        if bytes.len() > 4096 {
            return Err(ArchiveUnavailable);
        }
        let value: Self = serde_json::from_slice(bytes).map_err(|_| ArchiveUnavailable)?;
        if value.schema != "duet-rust-luna-withdraw-v1"
            || value.revision <= 0
            || value.event_id.is_nil()
            || value.account_id.is_nil()
        {
            return Err(ArchiveUnavailable);
        }
        Ok(value)
    }
}

pub(crate) async fn record(
    tx: &DatabaseTransaction,
    account_id: Uuid,
    revision: i64,
    now: i64,
) -> Result<(), sea_orm::DbErr> {
    restriction::ActiveModel {
        id: Set(Uuid::new_v4()),
        account_id: Set(account_id),
        revision: Set(revision),
        occurred_at: Set(now),
        confirmed: Set(false),
        lease: Set(None),
        due_at: Set(0),
        attempts: Set(0),
    }
    .insert(tx)
    .await?;
    Ok(())
}

impl PostgresFrontDoorStore {
    /// Attempt one account-scoped receipt. Caller retries through this same owner;
    /// ordinary reads and unrelated accounts never drain the journal. Concurrent
    /// workers are excluded by a five-minute persisted lease. Failed/abandoned
    /// attempts retain identity and back off; no live provider is implied by a fixture.
    pub async fn confirm_luna_restriction(
        &self,
        principal: &Principal,
        archive: &dyn ReceiptArchive,
        now: i64,
    ) -> Result<(), AuthorityError> {
        let _slot = DELIVERY_SLOTS
            .try_acquire()
            .map_err(|_| AuthorityError::RestrictionPending)?;
        let tx = self.db.begin().await?;
        let (account, permissions) = identity::locked(&tx, principal).await?;
        let row = restriction::Entity::find()
            .filter(restriction::Column::AccountId.eq(account.id))
            .filter(restriction::Column::Confirmed.eq(false))
            .lock_exclusive()
            .one(&tx)
            .await?;
        let Some(row) = row else {
            // Legacy pending state without an outbox is not evidence of delivery.
            return if permissions.restriction_pending {
                Err(AuthorityError::RestrictionPending)
            } else {
                Ok(())
            };
        };
        if row.due_at > now {
            return Err(AuthorityError::RestrictionPending);
        }
        let lease = Uuid::new_v4();
        let mut claim = row.clone().into_active_model();
        claim.lease = Set(Some(lease));
        claim.due_at = Set(now.saturating_add(300));
        claim.attempts = Set(row.attempts.saturating_add(1));
        claim.update(&tx).await?;
        tx.commit().await?;
        let body = serde_json::to_vec(&LunaReceipt::from_row(&row))
            .map_err(|_| AuthorityError::RestrictionPending)?;
        let timeout = std::time::Duration::from_secs(10);
        let _ambiguous = tokio::time::timeout(timeout, archive.create(row.id, &body)).await;
        let verified = matches!(tokio::time::timeout(timeout, archive.read(row.id)).await, Ok(Ok(actual)) if actual == body);
        let tx = self.db.begin().await?;
        // Same lock order as mutation/claim; do not require an unrevoked session to
        // acknowledge delivery of a restriction already committed by its owner.
        let owner = account::Entity::find_by_id(account.id)
            .lock_exclusive()
            .one(&tx)
            .await?;
        let current = restriction::Entity::find_by_id(row.id)
            .lock_exclusive()
            .one(&tx)
            .await?
            .ok_or(AuthorityError::RestrictionPending)?;
        if current.lease != Some(lease) {
            return Err(AuthorityError::RestrictionPending);
        }
        let mut update = current.into_active_model();
        update.lease = Set(None);
        update.confirmed = Set(verified);
        let delay = 30_i64
            .saturating_mul(1_i64 << row.attempts.clamp(0, 7))
            .min(3600);
        update.due_at = Set(now.saturating_add(delay));
        update.update(&tx).await?;
        let mut scope_complete = verified;
        if verified && owner.is_some() {
            let pending = restriction::Entity::find()
                .filter(restriction::Column::AccountId.eq(account.id))
                .filter(restriction::Column::Confirmed.eq(false))
                .one(&tx)
                .await?
                .is_some();
            scope_complete = !pending;
            if !pending {
                let permission = permission::Entity::find_by_id(account.id)
                    .one(&tx)
                    .await?
                    .ok_or(AuthorityError::Refused)?;
                let mut update = permission.into_active_model();
                update.restriction_pending = Set(false);
                update.update(&tx).await?;
            }
        }
        tx.commit().await?;
        if scope_complete {
            Ok(())
        } else {
            Err(AuthorityError::RestrictionPending)
        }
    }

    /// Reapply one validated withdrawal on an ISOLATED restore with consumers off.
    /// Matches account incarnation, never email/UID. Even a later positive grant
    /// in the snapshot requires fresh authorization. This is not a restore driver,
    /// coverage proof, external confirmation, or permission to enable traffic.
    pub async fn replay_luna_restriction(
        &self,
        receipt: &LunaReceipt,
    ) -> Result<(), AuthorityError> {
        if receipt.schema != "duet-rust-luna-withdraw-v1"
            || receipt.revision <= 0
            || receipt.event_id.is_nil()
            || receipt.account_id.is_nil()
        {
            return Err(AuthorityError::Refused);
        }
        let tx = self.db.begin().await?;
        let database = tx
            .query_one_raw(Statement::from_string(
                DbBackend::Postgres,
                "SELECT current_database() AS name",
            ))
            .await?
            .ok_or(AuthorityError::Refused)?;
        let name: String = database.try_get("", "name")?;
        if !name.starts_with("duet_recovery_") {
            return Err(AuthorityError::Refused);
        }
        if account::Entity::find_by_id(receipt.account_id)
            .lock_exclusive()
            .one(&tx)
            .await?
            .is_some()
        {
            let permission = permission::Entity::find_by_id(receipt.account_id)
                .one(&tx)
                .await?
                .ok_or(AuthorityError::Refused)?;
            if permission.luna_allowed || permission.luna_version.is_some() {
                let revision = permission
                    .revision
                    .checked_add(1)
                    .ok_or(AuthorityError::Refused)?;
                let mut update = permission.into_active_model();
                update.luna_allowed = Set(false);
                // The replayed independent receipt is already evidence. Invalidate
                // the old grant rather than inventing a new undeliverable event;
                // preserve any genuinely pending local outbox restriction.
                update.luna_version = Set(None);
                update.luna_at = Set(None);
                update.revision = Set(revision);
                update.update(&tx).await?;
            }
        }
        tx.commit().await?;
        Ok(())
    }
}
