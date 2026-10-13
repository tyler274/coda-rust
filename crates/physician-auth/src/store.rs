//! The Postgres `AuthStore` through SeaORM.
//!
//! Same contract as the memory store; durable truth lives in the
//! `physician_*` tables the crate's migration chain owns.

use std::time::Duration;

use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectOptions, Database, DatabaseConnection, DbErr,
    EntityTrait, QueryFilter, Set, TransactionTrait,
};
use sea_orm_migration::MigratorTrait;

use crate::migration::{Accounts, Invitations, Memberships, Sessions, SignInTokens};
use crate::{
    Account, AuthStore, INVITATION_TTL_SECONDS, Membership, PanelError, PhysicianIdentity,
    SESSION_TTL_SECONDS, SIGN_IN_TOKEN_TTL_SECONDS, SessionError, SignInError, mint_token,
    token_hash,
};

/// The SQL-backed store.
#[derive(Debug, Clone)]
pub struct PostgresAuthStore {
    db: DatabaseConnection,
}

impl PostgresAuthStore {
    /// Connects to `database_url` without applying migrations.
    pub async fn connect(database_url: &str) -> Result<Self, DbErr> {
        let mut options = ConnectOptions::new(database_url.to_owned());
        options
            .max_connections(8)
            .connect_timeout(Duration::from_secs(10));
        let db = Database::connect(options).await?;
        Ok(Self { db })
    }

    /// Connects and applies the crate's migration chain.
    pub async fn connect_with_migrations(database_url: &str) -> Result<Self, DbErr> {
        let store = Self::connect(database_url).await?;
        crate::migration::Migrator::up(&store.db, None).await?;
        Ok(store)
    }

    /// Applies the migration chain on an existing connection.
    pub async fn migrate(&self) -> Result<(), DbErr> {
        crate::migration::Migrator::up(&self.db, None).await
    }

    fn account_row(row: crate::migration::AccountRow) -> Account {
        Account {
            identity: PhysicianIdentity {
                practice_id: row.practice_id,
                physician_id: row.physician_id,
            },
            email: row.email,
            display_name: row.display_name,
            active: row.active,
            admin: row.admin,
            sessions_ended_before: row.sessions_ended_before.max(0) as u64,
        }
    }
}

#[async_trait]
impl AuthStore for PostgresAuthStore {
    async fn begin_sign_in(&self, email: &str, now: u64) -> Result<String, SignInError> {
        let key = email.to_lowercase();
        let account = Accounts::find()
            .filter(crate::migration::account::Column::Email.eq(&key))
            .one(&self.db)
            .await
            .map_err(stale)?
            .ok_or(SignInError::Invalid)?;
        if !account.active {
            return Err(SignInError::Inactive);
        }
        // One pending token per account: re-requesting replaces the last one.
        let token = mint_token();
        let active = crate::migration::SignInTokenActiveModel {
            id: Default::default(),
            account_id: Set(account.id),
            token_hash: Set(token_hash(&token).to_vec()),
            created_at: Set(now as i64),
            expires_at: Set((now + SIGN_IN_TOKEN_TTL_SECONDS) as i64),
        };
        let txn = self.db.begin().await.map_err(stale)?;
        SignInTokens::delete_many()
            .filter(crate::migration::sign_in_token::Column::AccountId.eq(account.id))
            .exec(&txn)
            .await
            .map_err(stale)?;
        active.insert(&txn).await.map_err(stale)?;
        txn.commit().await.map_err(stale)?;
        Ok(token)
    }

    async fn complete_sign_in(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(PhysicianIdentity, String), SignInError> {
        let key = email.to_lowercase();
        let txn = self.db.begin().await.map_err(stale)?;
        let (pending, account) = pending_account(&txn, &key, token).await?;
        if pending.expires_at <= now as i64 {
            return Err(SignInError::Invalid);
        }
        if !account.active {
            return Err(SignInError::Inactive);
        }
        // The pending row is deleted inside the transaction: a raced second
        // verify finds nothing, so a token mints one session.
        SignInTokens::delete_by_id(pending.id)
            .exec(&txn)
            .await
            .map_err(stale)?;
        let session_token = mint_token();
        let session = crate::migration::SessionActiveModel {
            id: Default::default(),
            account_id: Set(account.id),
            token_hash: Set(token_hash(&session_token).to_vec()),
            created_at: Set(now as i64),
            expires_at: Set((now + SESSION_TTL_SECONDS) as i64),
        };
        session.insert(&txn).await.map_err(stale)?;
        txn.commit().await.map_err(stale)?;
        Ok((
            PhysicianIdentity {
                practice_id: account.practice_id,
                physician_id: account.physician_id,
            },
            session_token,
        ))
    }

    async fn session(&self, token: &str, now: u64) -> Result<PhysicianIdentity, SessionError> {
        let hash = token_hash(token);
        let session = Sessions::find()
            .filter(crate::migration::session::Column::TokenHash.eq(hash.to_vec()))
            .one(&self.db)
            .await
            .map_err(invalid)?
            .ok_or(SessionError::Invalid)?;
        if session.expires_at <= now as i64 {
            return Err(SessionError::Invalid);
        }
        let account = Accounts::find_by_id(session.account_id)
            .one(&self.db)
            .await
            .map_err(invalid)?
            .ok_or(SessionError::Inactive)?;
        if !account.active {
            return Err(SessionError::Inactive);
        }
        if session.created_at < account.sessions_ended_before {
            // Ended by deactivation: the account is active again, this
            // session still is not.
            return Err(SessionError::Inactive);
        }
        Ok(PhysicianIdentity {
            practice_id: account.practice_id,
            physician_id: account.physician_id,
        })
    }

    async fn end_session(&self, token: &str) {
        let _ = Sessions::delete_many()
            .filter(crate::migration::session::Column::TokenHash.eq(token_hash(token).to_vec()))
            .exec(&self.db)
            .await;
    }

    async fn account_by_email(&self, email: &str) -> Option<Account> {
        Accounts::find()
            .filter(crate::migration::account::Column::Email.eq(email.to_lowercase()))
            .one(&self.db)
            .await
            .ok()
            .flatten()
            .map(Self::account_row)
    }

    async fn account_by_identity(&self, identity: &PhysicianIdentity) -> Option<Account> {
        Accounts::find()
            .filter(
                crate::migration::account::Column::PracticeId
                    .eq(&identity.practice_id)
                    .and(crate::migration::account::Column::PhysicianId.eq(&identity.physician_id)),
            )
            .one(&self.db)
            .await
            .ok()
            .flatten()
            .map(Self::account_row)
    }

    async fn membership(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
    ) -> Membership {
        Memberships::find()
            .filter(
                crate::migration::membership::Column::PracticeId
                    .eq(&identity.practice_id)
                    .and(
                        crate::migration::membership::Column::PhysicianId
                            .eq(&identity.physician_id),
                    )
                    .and(
                        crate::migration::membership::Column::PatientWorkspaceId
                            .eq(patient_workspace_id),
                    ),
            )
            .one(&self.db)
            .await
            .ok()
            .flatten()
            .filter(|row| row.allowed)
            .map(|_| Membership::Allowed)
            .unwrap_or(Membership::Denied)
    }

    async fn set_membership(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
        membership: Membership,
    ) {
        let allowed = membership == Membership::Allowed;
        let existing = Memberships::find()
            .filter(
                crate::migration::membership::Column::PracticeId
                    .eq(&identity.practice_id)
                    .and(
                        crate::migration::membership::Column::PhysicianId
                            .eq(&identity.physician_id),
                    )
                    .and(
                        crate::migration::membership::Column::PatientWorkspaceId
                            .eq(patient_workspace_id),
                    ),
            )
            .one(&self.db)
            .await;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|since| since.as_secs())
            .unwrap_or(0) as i64;
        let _ = match existing.ok().flatten() {
            Some(row) => {
                let mut update: crate::migration::MembershipActiveModel = row.into();
                update.allowed = Set(allowed);
                update.update(&self.db).await
            }
            None => {
                crate::migration::MembershipActiveModel {
                    id: Default::default(),
                    practice_id: Set(identity.practice_id.clone()),
                    physician_id: Set(identity.physician_id.clone()),
                    patient_workspace_id: Set(patient_workspace_id.to_owned()),
                    allowed: Set(allowed),
                    created_at: Set(now),
                }
                .insert(&self.db)
                .await
            }
        };
    }

    async fn panel(&self, identity: &PhysicianIdentity) -> Vec<String> {
        let mut workspaces: Vec<String> = Memberships::find()
            .filter(
                crate::migration::membership::Column::PracticeId
                    .eq(&identity.practice_id)
                    .and(
                        crate::migration::membership::Column::PhysicianId
                            .eq(&identity.physician_id),
                    )
                    .and(crate::migration::membership::Column::Allowed.eq(true)),
            )
            .all(&self.db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|row| row.patient_workspace_id)
            .collect();
        workspaces.sort();
        workspaces
    }

    async fn assign_panel(
        &self,
        actor: &PhysicianIdentity,
        physician_id: &str,
        patient_workspace_id: &str,
        membership: Membership,
    ) -> Result<(), PanelError> {
        let caller = self
            .account_by_identity(actor)
            .await
            .ok_or(PanelError::Forbidden)?;
        if !caller.active || !caller.admin {
            return Err(PanelError::Forbidden);
        }
        let target = PhysicianIdentity {
            practice_id: actor.practice_id.clone(),
            physician_id: physician_id.to_owned(),
        };
        if self.account_by_identity(&target).await.is_none() {
            return Err(PanelError::UnknownTarget);
        }
        self.set_membership(&target, patient_workspace_id, membership)
            .await;
        Ok(())
    }

    async fn invite(
        &self,
        practice_id: &str,
        email: &str,
        display_name: &str,
        admin: bool,
        now: u64,
    ) -> Result<String, SignInError> {
        let key = email.to_lowercase();
        let token = mint_token();
        let active = crate::migration::InvitationActiveModel {
            id: Default::default(),
            practice_id: Set(practice_id.to_owned()),
            email: Set(key),
            display_name: Set(display_name.to_owned()),
            admin: Set(admin),
            token_hash: Set(token_hash(&token).to_vec()),
            created_at: Set(now as i64),
            expires_at: Set((now + INVITATION_TTL_SECONDS) as i64),
        };
        active.insert(&self.db).await.map_err(stale)?;
        Ok(token)
    }

    async fn redeem_invitation(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(PhysicianIdentity, String), SignInError> {
        let key = email.to_lowercase();
        let txn = self.db.begin().await.map_err(stale)?;
        if Accounts::find()
            .filter(crate::migration::account::Column::Email.eq(&key))
            .one(&txn)
            .await
            .map_err(stale)?
            .is_some()
        {
            // The account exists; it signs in rather than redeeming again.
            return Err(SignInError::Invalid);
        }
        let invitation = Invitations::find()
            .filter(crate::migration::invitation::Column::Email.eq(&key))
            .one(&txn)
            .await
            .map_err(stale)?
            .ok_or(SignInError::Invalid)?;
        if invitation.expires_at <= now as i64
            || invitation.token_hash != token_hash(token).to_vec()
        {
            return Err(SignInError::Invalid);
        }
        // The invitation row is deleted inside the transaction: a raced
        // second redeem finds nothing.
        Invitations::delete_by_id(invitation.id)
            .exec(&txn)
            .await
            .map_err(stale)?;
        let account = crate::migration::AccountActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            practice_id: Set(invitation.practice_id.clone()),
            physician_id: Set(uuid::Uuid::new_v4().simple().to_string()),
            email: Set(key),
            display_name: Set(invitation.display_name.clone()),
            active: Set(true),
            admin: Set(invitation.admin),
            sessions_ended_before: Set(0),
            created_at: Set(now as i64),
        };
        let account = account.insert(&txn).await.map_err(stale)?;
        let session_token = mint_token();
        let session = crate::migration::SessionActiveModel {
            id: Default::default(),
            account_id: Set(account.id),
            token_hash: Set(token_hash(&session_token).to_vec()),
            created_at: Set(now as i64),
            expires_at: Set((now + SESSION_TTL_SECONDS) as i64),
        };
        session.insert(&txn).await.map_err(stale)?;
        txn.commit().await.map_err(stale)?;
        Ok((
            PhysicianIdentity {
                practice_id: account.practice_id,
                physician_id: account.physician_id,
            },
            session_token,
        ))
    }

    async fn deactivate(&self, email: &str) -> bool {
        let key = email.to_lowercase();
        let Ok(txn) = self.db.begin().await else {
            return false;
        };
        let Some(account) = Accounts::find()
            .filter(crate::migration::account::Column::Email.eq(&key))
            .one(&txn)
            .await
            .ok()
            .flatten()
        else {
            return false;
        };
        let active = crate::migration::AccountActiveModel {
            id: Set(account.id),
            practice_id: Set(account.practice_id),
            physician_id: Set(account.physician_id),
            email: Set(account.email),
            display_name: Set(account.display_name),
            active: Set(false),
            admin: Set(account.admin),
            sessions_ended_before: Set(account.sessions_ended_before),
            created_at: Set(account.created_at),
        };
        // The fence, not row deletion: reactivation never resurrects the
        // sessions deactivation ended, and resolution keeps naming why.
        let updated = crate::migration::AccountActiveModel {
            sessions_ended_before: Set(account.sessions_ended_before.max(
                Sessions::find()
                    .filter(crate::migration::session::Column::AccountId.eq(account.id))
                    .all(&txn)
                    .await
                    .map(|rows| rows.iter().map(|row| row.created_at).max().unwrap_or(0) + 1)
                    .unwrap_or(account.sessions_ended_before),
            )),
            ..active
        };
        updated.update(&txn).await.is_ok() && txn.commit().await.is_ok()
    }

    async fn reactivate(&self, email: &str) -> bool {
        let key = email.to_lowercase();
        let Some(account) = Accounts::find()
            .filter(crate::migration::account::Column::Email.eq(&key))
            .one(&self.db)
            .await
            .ok()
            .flatten()
        else {
            return false;
        };
        let update = crate::migration::AccountActiveModel {
            id: Set(account.id),
            practice_id: Set(account.practice_id),
            physician_id: Set(account.physician_id),
            email: Set(account.email),
            display_name: Set(account.display_name),
            active: Set(true),
            admin: Set(account.admin),
            sessions_ended_before: Set(account.sessions_ended_before),
            created_at: Set(account.created_at),
        };
        update.update(&self.db).await.is_ok()
    }

    async fn remove_account(&self, email: &str) -> bool {
        Accounts::delete_many()
            .filter(crate::migration::account::Column::Email.eq(email.to_lowercase()))
            .exec(&self.db)
            .await
            .map(|result| result.rows_affected > 0)
            .unwrap_or(false)
    }
}

/// The raced second verify finds no pending row.
async fn pending_account(
    txn: &impl sea_orm::ConnectionTrait,
    email: &str,
    token: &str,
) -> Result<
    (
        crate::migration::SignInTokenRow,
        crate::migration::AccountRow,
    ),
    SignInError,
> {
    let pending = SignInTokens::find()
        .filter(crate::migration::sign_in_token::Column::TokenHash.eq(token_hash(token).to_vec()))
        .one(txn)
        .await
        .map_err(|_| SignInError::Invalid)?
        .ok_or(SignInError::Invalid)?;
    let account = Accounts::find_by_id(pending.account_id)
        .one(txn)
        .await
        .map_err(|_| SignInError::Invalid)?
        .ok_or(SignInError::Invalid)?;
    if account.email != email {
        return Err(SignInError::Invalid);
    }
    Ok((pending, account))
}

fn stale(_: DbErr) -> SignInError {
    SignInError::Invalid
}

fn invalid(_: DbErr) -> SessionError {
    SessionError::Invalid
}
