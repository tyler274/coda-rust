//! Durable Duet authority over the retained front-door account rows.
//!
//! Callers must verify the Identity Platform token and current provider revocation
//! state before passing a principal here. Email is metadata, NEVER a lookup key
//! for authentication or merging. Provider linking must preserve the verified
//! project/subject; this module does not offer an email-based linking escape hatch.
use crate::{
    PostgresFrontDoorStore,
    migration::{account, identity, permission},
};
use sea_orm::{
    ActiveModelTrait, DatabaseTransaction, DbErr, EntityTrait, IntoActiveModel, QuerySelect, Set,
    TransactionTrait,
};
use uuid::Uuid;

/// Current agreement versions inherited from Duet's original owner.
pub const TERMS_VERSION: &str = "2026-09-23";
/// Current Luna permission version.
pub const LUNA_VERSION: &str = "2026-09-24";

/// Verified provider principal, supplied only by a trusted token boundary.
#[derive(Clone, Debug)]
pub struct Principal {
    /// Explicit Identity Platform project, not an inferred Cloud Run project.
    pub project: String,
    /// Stable Identity Platform UID across securely linked providers.
    pub subject: String,
    /// Original authentication time; refreshing a token cannot advance it.
    pub authenticated_at: i64,
    /// Verified email, retained as metadata only.
    pub email: String,
}

/// Refusals distinguish denied authority from storage unavailability.
#[derive(Debug)]
pub enum AuthorityError {
    /// No account is provisioned for this verified subject.
    Absent,
    /// Invalid, inactive, or revoked authority.
    Refused,
    /// The submitted agreement version changed.
    AgreementChanged,
    /// A restriction must be confirmed by the privacy-journal owner first.
    RestrictionPending,
    /// Durable state could not be established. Never grant on this error.
    Storage(DbErr),
}
impl From<DbErr> for AuthorityError {
    fn from(error: DbErr) -> Self {
        Self::Storage(error)
    }
}

/// Account-scoped authorization snapshot, not a permanent permission grant.
#[derive(Clone, Debug)]
pub struct Access {
    /// Canonical existing account id for every Duet data owner.
    pub account_id: Uuid,
    /// Durable terms/Luna state. Consumers recheck under lock before effects.
    pub permissions: permission::Model,
}
impl Access {
    /// New model work requires current terms, permission, and no pending restriction.
    /// Operator readiness remains a separate required gate.
    pub fn allows_luna(&self) -> bool {
        self.permissions.terms_version.as_deref() == Some(TERMS_VERSION)
            && self.permissions.luna_version.as_deref() == Some(LUNA_VERSION)
            && self.permissions.luna_allowed
            && !self.permissions.restriction_pending
    }
}

async fn locked(
    tx: &DatabaseTransaction,
    principal: &Principal,
) -> Result<(account::Model, permission::Model), AuthorityError> {
    let mapping =
        identity::Entity::find_by_id((principal.project.clone(), principal.subject.clone()))
            .one(tx)
            .await?
            .ok_or(AuthorityError::Absent)?;
    let account = account::Entity::find_by_id(mapping.account_id)
        .lock_exclusive()
        .one(tx)
        .await?
        .ok_or(AuthorityError::Absent)?;
    if !account.active
        || principal.authenticated_at <= 0
        || principal.authenticated_at < account.sessions_ended_before
    {
        return Err(AuthorityError::Refused);
    }
    let permission = permission::Entity::find_by_id(account.id)
        .one(tx)
        .await?
        .ok_or(AuthorityError::Refused)?;
    Ok((account, permission))
}

impl PostgresFrontDoorStore {
    /// Explicit standalone provisioning. A matching email never adopts an account.
    /// Unique constraints refuse racing/colliding provisioning; retry resolves the
    /// same subject rather than creating a second account. No old-account migration.
    pub async fn provision_identity(
        &self,
        principal: &Principal,
        now: i64,
    ) -> Result<Access, AuthorityError> {
        if principal.project.is_empty()
            || principal.subject.is_empty()
            || principal.subject.len() > 128
            || principal.email.trim().is_empty()
            || principal.authenticated_at <= 0
            || principal.authenticated_at > now
        {
            return Err(AuthorityError::Refused);
        }
        let tx = self.db.begin().await?;
        if identity::Entity::find_by_id((principal.project.clone(), principal.subject.clone()))
            .one(&tx)
            .await?
            .is_some()
        {
            let (account, permissions) = locked(&tx, principal).await?;
            tx.commit().await?;
            return Ok(Access {
                account_id: account.id,
                permissions,
            });
        }
        // Never look up by email to reuse an account. A collision fails closed;
        // linking existing separate identities requires a distinct approved owner.
        let id = Uuid::new_v4();
        account::ActiveModel {
            id: Set(id),
            email: Set(principal.email.to_lowercase()),
            display_name: Set(String::new()),
            active: Set(true),
            linked_practice_id: Set(None),
            linked_patient_id: Set(None),
            sessions_ended_before: Set(0),
            created_at: Set(now),
        }
        .insert(&tx)
        .await?;
        identity::ActiveModel {
            project: Set(principal.project.clone()),
            subject: Set(principal.subject.clone()),
            account_id: Set(id),
        }
        .insert(&tx)
        .await?;
        let permissions = permission::ActiveModel {
            account_id: Set(id),
            terms_version: Set(None),
            terms_at: Set(None),
            luna_version: Set(None),
            luna_allowed: Set(false),
            luna_at: Set(None),
            revision: Set(0),
            restriction_pending: Set(false),
        }
        .insert(&tx)
        .await?;
        tx.commit().await?;
        Ok(Access {
            account_id: id,
            permissions,
        })
    }

    /// Ordinary requests only resolve existing owners; never provision on reads.
    pub async fn identity_access(&self, principal: &Principal) -> Result<Access, AuthorityError> {
        let tx = self.db.begin().await?;
        let (account, permissions) = locked(&tx, principal).await?;
        tx.commit().await?;
        Ok(Access {
            account_id: account.id,
            permissions,
        })
    }

    /// Revoke all sessions through the next whole second. The stored cutoff never
    /// moves backwards. This is LOCAL enforcement, not a provider-confirmation receipt.
    pub async fn revoke_identity_sessions(
        &self,
        principal: &Principal,
        now: i64,
    ) -> Result<(), AuthorityError> {
        let tx = self.db.begin().await?;
        let (account, _) = locked(&tx, principal).await?;
        let fence = now.checked_add(1).ok_or(AuthorityError::Refused)?;
        let cutoff = account.sessions_ended_before.max(fence);
        let mut update = account.into_active_model();
        update.sessions_ended_before = Set(cutoff);
        update.update(&tx).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Versioned terms acceptance is idempotent and checked under the session lock.
    pub async fn accept_identity_terms(
        &self,
        principal: &Principal,
        version: &str,
        now: i64,
    ) -> Result<Access, AuthorityError> {
        if version != TERMS_VERSION {
            return Err(AuthorityError::AgreementChanged);
        }
        let tx = self.db.begin().await?;
        let (account, permissions) = locked(&tx, principal).await?;
        let permissions = if permissions.terms_version.as_deref() != Some(version) {
            let revision = permissions.revision + 1;
            let mut update = permissions.into_active_model();
            update.terms_version = Set(Some(version.to_owned()));
            update.terms_at = Set(Some(now));
            update.revision = Set(revision);
            update.update(&tx).await?
        } else {
            permissions
        };
        tx.commit().await?;
        Ok(Access {
            account_id: account.id,
            permissions,
        })
    }

    /// Withdrawals persist immediately and cannot be undone before journal
    /// confirmation. There is deliberately no public 'clear pending' shortcut.
    /// Full journal confirmation/replay is ENG-113, not implemented by this store.
    pub async fn set_identity_luna(
        &self,
        principal: &Principal,
        version: &str,
        allowed: bool,
        now: i64,
    ) -> Result<Access, AuthorityError> {
        if version != LUNA_VERSION {
            return Err(AuthorityError::AgreementChanged);
        }
        let tx = self.db.begin().await?;
        let (account, permissions) = locked(&tx, principal).await?;
        if permissions.terms_version.as_deref() != Some(TERMS_VERSION) {
            return Err(AuthorityError::Refused);
        }
        if allowed && permissions.restriction_pending {
            return Err(AuthorityError::RestrictionPending);
        }
        let revision = permissions.revision + 1;
        let pending = permissions.restriction_pending || !allowed;
        let mut update = permissions.into_active_model();
        update.luna_version = Set(Some(version.to_owned()));
        update.luna_allowed = Set(allowed);
        update.luna_at = Set(Some(now));
        update.revision = Set(revision);
        update.restriction_pending = Set(pending);
        let permissions = update.update(&tx).await?;
        tx.commit().await?;
        Ok(Access {
            account_id: account.id,
            permissions,
        })
    }
}
