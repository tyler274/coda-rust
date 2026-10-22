//! Bounded Duet-owned ORM adoption for private care teams and release templates.
//! SeaORM 2 and the account boundary both use SQLx 0.9 and the same runtime pool.
//! No clinical entities or account state are imported from the physician product.
mod entity;
mod releases;
use crate::{ApiError, durable::DurableState, identity::AccountContext};
use chrono::{DateTime, Utc};
pub use releases::{
    ReleaseInput, ReleaseParty, ReleasePatient, ReleaseRecipient, ReleaseScope, ReleaseView,
    SensitiveDisclosure,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, DbBackend, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder, Set, Statement,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub(crate) fn database_error(_: sea_orm::DbErr) -> ApiError {
    // Database errors may contain private values; never expose them to responses/logs.
    ApiError::Unavailable("Care-team storage unavailable")
}
/// Account-scoped operations, injectable public directory for fixture proof.
#[derive(Clone)]
pub struct CareTeamStore {
    /// Existing account/privacy authority.
    pub durable: DurableState,
    /// Public NPPES adapter; no user data is passed to it besides explicit search terms.
    pub directory: std::sync::Arc<dyn npi_registry::Directory>,
}
impl CareTeamStore {
    async fn begin(&self, c: &AccountContext) -> Result<DatabaseTransaction, ApiError> {
        self.durable.privacy.confirm_account(c.account_id).await?;
        let tx = self
            .durable
            .identity
            .database
            .begin_orm(c.account_id)
            .await?;
        // Account locking and session/incarnation fencing are PostgreSQL-specific
        // boundary operations. All care-team/release CRUD below uses typed ORM.
        let row = tx.query_one_raw(Statement::from_sql_and_values(DbBackend::Postgres,
            "select duet_incarnation,sessions_revoked_before from public.users where id=$1 for update", [c.account_id.into()]))
            .await.map_err(database_error)?.ok_or(ApiError::AbsentAccount)?;
        let incarnation: Uuid = row
            .try_get("", "duet_incarnation")
            .map_err(database_error)?;
        let revoked: Option<DateTime<Utc>> = row
            .try_get("", "sessions_revoked_before")
            .map_err(database_error)?;
        if incarnation != c.incarnation || revoked.is_some_and(|at| c.authenticated_at <= at) {
            return Err(ApiError::SessionRevoked);
        }
        // Recheck after the account lock: a restriction could commit between
        // confirm_account and this transaction, including during directory I/O.
        let pending = tx.query_one_raw(Statement::from_sql_and_values(DbBackend::Postgres,
            "select exists(select 1 from public.privacy_recovery_events where account_id=$1 and journaled_at is null) as pending", [c.account_id.into()]))
            .await.map_err(database_error)?.ok_or(ApiError::Unavailable("Privacy confirmation unavailable"))?;
        if pending
            .try_get::<bool>("", "pending")
            .map_err(database_error)?
        {
            return Err(ApiError::Unavailable("Privacy confirmation pending"));
        }
        Ok(tx)
    }
    /// Verify fresh account fences without retaining a lock during network I/O.
    pub async fn check(&self, c: &AccountContext) -> Result<(), ApiError> {
        self.begin(c).await?.commit().await.map_err(database_error)
    }
    /// List only members of the verified account.
    pub async fn list(&self, c: &AccountContext) -> Result<Vec<CareTeamMember>, ApiError> {
        let tx = self.begin(c).await?;
        let result = entity::member::Entity::find()
            .filter(entity::member::Column::UserId.eq(c.account_id))
            .order_by_asc(entity::member::Column::AddedAt)
            .order_by_asc(entity::member::Column::Id)
            .all(&tx)
            .await
            .map_err(database_error)?
            .into_iter()
            .map(member_view)
            .collect::<Result<_, _>>()?;
        tx.commit().await.map_err(database_error)?;
        Ok(result)
    }
    /// Add from a server-fetched NPPES record; retries return the existing member.
    pub async fn add(&self, c: &AccountContext, npi: &str) -> Result<CareTeamMember, ApiError> {
        self.check(c).await?;
        let npi =
            npi_registry::Npi::parse(npi).map_err(|_| ApiError::Unprocessable("Invalid NPI"))?;
        let clinician = self
            .directory
            .by_number(&npi)
            .await
            .map_err(registry_error)?
            .ok_or(ApiError::NotFound)?;
        let tx = self.begin(c).await?;
        if let Some(row) = entity::member::Entity::find()
            .filter(entity::member::Column::UserId.eq(c.account_id))
            .filter(entity::member::Column::Npi.eq(npi.to_string()))
            .one(&tx)
            .await
            .map_err(database_error)?
        {
            return member_view(row);
        }
        let now = Utc::now();
        let row = entity::member::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(c.account_id),
            npi: Set(npi.to_string()),
            snapshot: Set(json(&clinician)?),
            role: Set(String::new()),
            nickname: Set(String::new()),
            note: Set(String::new()),
            added_at: Set(now),
            refreshed_at: Set(now),
        }
        .insert(&tx)
        .await
        .map_err(database_error)?;
        tx.commit().await.map_err(database_error)?;
        member_view(row)
    }
    /// Patch patient-owned labels; absent fields are preserved, empty strings clear.
    pub async fn patch(
        &self,
        c: &AccountContext,
        id: Uuid,
        patch: MemberPatch,
    ) -> Result<CareTeamMember, ApiError> {
        for value in [&patch.role, &patch.nickname, &patch.note]
            .into_iter()
            .flatten()
        {
            bounded(value, 4000)?;
        }
        let tx = self.begin(c).await?;
        let mut row = owned_member(&tx, c, id).await?.into_active_model();
        if let Some(value) = patch.role {
            row.role = Set(value);
        }
        if let Some(value) = patch.nickname {
            row.nickname = Set(value);
        }
        if let Some(value) = patch.note {
            row.note = Set(value);
        }
        let row = row.update(&tx).await.map_err(database_error)?;
        tx.commit().await.map_err(database_error)?;
        member_view(row)
    }
    /// Delete an owned member and its saved release templates atomically.
    pub async fn delete(&self, c: &AccountContext, id: Uuid) -> Result<(), ApiError> {
        let tx = self.begin(c).await?;
        let count = entity::member::Entity::delete_many()
            .filter(entity::member::Column::UserId.eq(c.account_id))
            .filter(entity::member::Column::Id.eq(id))
            .exec(&tx)
            .await
            .map_err(database_error)?
            .rows_affected;
        if count != 1 {
            return Err(ApiError::NotFound);
        }
        tx.commit().await.map_err(database_error)
    }
    /// Refresh directory data without changing patient labels or older release snapshots.
    pub async fn refresh(&self, c: &AccountContext, id: Uuid) -> Result<CareTeamMember, ApiError> {
        let tx = self.begin(c).await?;
        let initial = owned_member(&tx, c, id).await?;
        tx.commit().await.map_err(database_error)?;
        let npi = npi_registry::Npi::parse(&initial.npi)
            .map_err(|_| ApiError::Unavailable("Invalid stored NPI"))?;
        let clinician = self
            .directory
            .by_number(&npi)
            .await
            .map_err(registry_error)?
            .ok_or(ApiError::NotFound)?;
        let tx = self.begin(c).await?;
        let mut row = owned_member(&tx, c, id).await?.into_active_model();
        row.snapshot = Set(json(&clinician)?);
        row.refreshed_at = Set(Utc::now());
        let row = row.update(&tx).await.map_err(database_error)?;
        tx.commit().await.map_err(database_error)?;
        member_view(row)
    }
}
async fn owned_member(
    tx: &DatabaseTransaction,
    c: &AccountContext,
    id: Uuid,
) -> Result<entity::member::Model, ApiError> {
    entity::member::Entity::find_by_id(id)
        .filter(entity::member::Column::UserId.eq(c.account_id))
        .one(tx)
        .await
        .map_err(database_error)?
        .ok_or(ApiError::NotFound)
}
fn member_view(row: entity::member::Model) -> Result<CareTeamMember, ApiError> {
    Ok(CareTeamMember {
        id: row.id,
        clinician: serde_json::from_value(row.snapshot)
            .map_err(|_| ApiError::Unavailable("Stored directory data unavailable"))?,
        role: row.role,
        nickname: row.nickname,
        note: row.note,
        added_at: row.added_at,
        refreshed_at: row.refreshed_at,
    })
}
pub(crate) fn registry_error(error: npi_registry::RegistryError) -> ApiError {
    match error {
        npi_registry::RegistryError::InvalidQuery(_) | npi_registry::RegistryError::Rejected(_) => {
            ApiError::Unprocessable("Invalid directory query")
        }
        _ => ApiError::Unavailable("Clinician directory unavailable"),
    }
}
fn json(value: &impl Serialize) -> Result<serde_json::Value, ApiError> {
    serde_json::to_value(value).map_err(|_| ApiError::Unprocessable("Invalid care-team data"))
}
fn bounded(value: &str, max: usize) -> Result<(), ApiError> {
    if value.len() > max
        || value
            .chars()
            .any(|c| c.is_control() && c != '\n' && c != '\t')
    {
        Err(ApiError::Unprocessable(
            "Invalid text length or control character",
        ))
    } else {
        Ok(())
    }
}
/// Patient labels; null/absent leaves a field unchanged, empty text clears it.
#[derive(Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(missing_docs)]
pub struct MemberPatch {
    pub role: Option<String>,
    pub nickname: Option<String>,
    pub note: Option<String>,
}
/// Account-private member projection; the internal account ID is never returned.
#[derive(Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CareTeamMember {
    pub id: Uuid,
    pub clinician: npi_registry::Clinician,
    pub role: String,
    pub nickname: String,
    pub note: String,
    pub added_at: DateTime<Utc>,
    pub refreshed_at: DateTime<Utc>,
}
