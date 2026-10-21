//! The Postgres `FrontDoorStore` through SeaORM.
//!
//! Same contract as the memory store; durable truth lives in the
//! `front_door_*` tables the crate's migration chain owns.

use std::time::Duration;

use crate::migration::{Accounts, Consents, Invitations, Links, Premiums, Sessions, SignInTokens};
use crate::{
    Account, Consent, FrontDoorStore, Invitation, InviteError, LinkError, Premium, RosterPatient,
    SESSION_TTL_SECONDS, SIGN_IN_TOKEN_TTL_SECONDS, SessionError, SignInError, cadence_for,
    mint_token, token_hash,
};
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectOptions, Database, DatabaseConnection, DbErr,
    EntityTrait, QueryFilter, Set, TransactionTrait,
};

/// The SQL-backed store.
#[derive(Debug, Clone)]
pub struct PostgresFrontDoorStore {
    db: DatabaseConnection,
}

impl PostgresFrontDoorStore {
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
        crate::migration::upgrade::migrate(&store.db).await?;
        Ok(store)
    }

    /// Applies the migration chain on an existing connection.
    pub async fn migrate(&self) -> Result<(), DbErr> {
        crate::migration::upgrade::migrate(&self.db).await
    }

    fn account_row(row: crate::migration::AccountRow) -> Account {
        Account {
            id: row.id,
            email: row.email,
            display_name: row.display_name,
            active: row.active,
            link: match (row.linked_practice_id, row.linked_patient_id) {
                (Some(practice_id), Some(patient_id)) => Some(RosterPatient {
                    practice_id,
                    patient_id,
                }),
                _ => None,
            },
            sessions_ended_before: row.sessions_ended_before.max(0) as u64,
        }
    }

    fn consent_row(row: crate::migration::ConsentRow) -> Consent {
        match row.withdrawn_at {
            Some(withdrawn_at) if withdrawn_at > 0 => Consent::Withdrawn {
                at: row.consented_at.max(0) as u64,
                withdrawn_at: withdrawn_at.max(0) as u64,
            },
            _ => Consent::Granted {
                at: row.consented_at.max(0) as u64,
            },
        }
    }
}

#[async_trait]
impl FrontDoorStore for PostgresFrontDoorStore {
    async fn invite(
        &self,
        patient: &RosterPatient,
        email: &str,
        display_name: &str,
        now: u64,
    ) -> Result<String, InviteError> {
        let key = email.to_lowercase();
        if Accounts::find()
            .filter(crate::migration::account::Column::Email.eq(&key))
            .one(&self.db)
            .await
            .map_err(|_| InviteError::Conflict)?
            .is_some()
        {
            // The email already holds a front-door account.
            return Err(InviteError::Conflict);
        }
        let token = mint_token();
        let active = crate::migration::InvitationActiveModel {
            id: Default::default(),
            practice_id: Set(patient.practice_id.clone()),
            patient_id: Set(patient.patient_id.clone()),
            email: Set(key),
            display_name: Set(display_name.to_owned()),
            token_hash: Set(token_hash(&token).to_vec()),
            created_at: Set(now as i64),
            expires_at: Set((now + crate::INVITATION_TTL_SECONDS) as i64),
        };
        Invitations::insert(active)
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(crate::migration::invitation::Column::Email)
                    .update_columns([
                        crate::migration::invitation::Column::PracticeId,
                        crate::migration::invitation::Column::PatientId,
                        crate::migration::invitation::Column::DisplayName,
                        crate::migration::invitation::Column::TokenHash,
                        crate::migration::invitation::Column::CreatedAt,
                        crate::migration::invitation::Column::ExpiresAt,
                    ])
                    .to_owned(),
            )
            .exec(&self.db)
            .await
            .map_err(|_| InviteError::Conflict)?;
        Ok(token)
    }

    async fn invitation(&self, email: &str) -> Option<Invitation> {
        let row = Invitations::find()
            .filter(crate::migration::invitation::Column::Email.eq(email.to_lowercase()))
            .one(&self.db)
            .await
            .ok()
            .flatten()?;
        Some(Invitation {
            patient: RosterPatient {
                practice_id: row.practice_id,
                patient_id: row.patient_id,
            },
            email: row.email,
            display_name: row.display_name,
            expires_at: row.expires_at.max(0) as u64,
        })
    }

    async fn redeem_invitation(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(uuid::Uuid, String), SignInError> {
        let key = email.to_lowercase();
        let txn = self.db.begin().await.map_err(|_| SignInError::Invalid)?;
        if Accounts::find()
            .filter(crate::migration::account::Column::Email.eq(&key))
            .one(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?
            .is_some()
        {
            // The account exists; it signs in rather than redeeming again.
            return Err(SignInError::Invalid);
        }
        let invitation = Invitations::find()
            .filter(crate::migration::invitation::Column::Email.eq(&key))
            .one(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?
            .ok_or(SignInError::Invalid)?;
        if invitation.expires_at <= now as i64
            || invitation.token_hash != token_hash(token).to_vec()
        {
            return Err(SignInError::Invalid);
        }
        // The invitation row is deleted inside the transaction: a raced
        // second redeem finds nothing, so an invitation redeems once.
        Invitations::delete_by_id(invitation.id)
            .exec(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?;
        let account = crate::migration::AccountActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            email: Set(key),
            display_name: Set(invitation.display_name.clone()),
            active: Set(true),
            linked_practice_id: Set(Some(invitation.practice_id.clone())),
            linked_patient_id: Set(Some(invitation.patient_id.clone())),
            sessions_ended_before: Set(0),
            created_at: Set(now as i64),
        };
        let account = account
            .insert(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?;
        let link = crate::migration::LinkActiveModel {
            account_id: Set(account.id),
            practice_id: Set(invitation.practice_id.clone()),
            patient_id: Set(invitation.patient_id.clone()),
            created_at: Set(now as i64),
        };
        link.insert(&txn).await.map_err(|_| SignInError::Invalid)?;
        let session_token = mint_token();
        let session = crate::migration::SessionActiveModel {
            id: Default::default(),
            account_id: Set(account.id),
            token_hash: Set(token_hash(&session_token).to_vec()),
            created_at: Set(now as i64),
            expires_at: Set((now + SESSION_TTL_SECONDS) as i64),
        };
        session
            .insert(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?;
        txn.commit().await.map_err(|_| SignInError::Invalid)?;
        Ok((account.id, session_token))
    }

    async fn begin_sign_in(&self, email: &str, now: u64) -> Result<String, SignInError> {
        let key = email.to_lowercase();
        let account = Accounts::find()
            .filter(crate::migration::account::Column::Email.eq(&key))
            .one(&self.db)
            .await
            .map_err(|_| SignInError::Invalid)?
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
        let txn = self.db.begin().await.map_err(|_| SignInError::Invalid)?;
        SignInTokens::delete_many()
            .filter(crate::migration::sign_in_token::Column::AccountId.eq(account.id))
            .exec(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?;
        active
            .insert(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?;
        txn.commit().await.map_err(|_| SignInError::Invalid)?;
        Ok(token)
    }

    async fn complete_sign_in(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(uuid::Uuid, String), SignInError> {
        let key = email.to_lowercase();
        let txn = self.db.begin().await.map_err(|_| SignInError::Invalid)?;
        let pending = SignInTokens::find()
            .filter(
                crate::migration::sign_in_token::Column::TokenHash.eq(token_hash(token).to_vec()),
            )
            .one(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?
            .ok_or(SignInError::Invalid)?;
        if pending.expires_at <= now as i64 {
            return Err(SignInError::Invalid);
        }
        let account = Accounts::find_by_id(pending.account_id)
            .one(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?
            .ok_or(SignInError::Invalid)?;
        if account.email != key || !account.active {
            return Err(SignInError::Invalid);
        }
        // The pending row is deleted inside the transaction: a raced second
        // verify finds nothing, so a token mints one session.
        SignInTokens::delete_by_id(pending.id)
            .exec(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?;
        let session_token = mint_token();
        let session = crate::migration::SessionActiveModel {
            id: Default::default(),
            account_id: Set(account.id),
            token_hash: Set(token_hash(&session_token).to_vec()),
            created_at: Set(now as i64),
            expires_at: Set((now + SESSION_TTL_SECONDS) as i64),
        };
        session
            .insert(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?;
        txn.commit().await.map_err(|_| SignInError::Invalid)?;
        Ok((account.id, session_token))
    }

    async fn session(&self, token: &str, now: u64) -> Result<uuid::Uuid, SessionError> {
        let hash = token_hash(token);
        let session = Sessions::find()
            .filter(crate::migration::session::Column::TokenHash.eq(hash.to_vec()))
            .one(&self.db)
            .await
            .map_err(|_| SessionError::Invalid)?
            .ok_or(SessionError::Invalid)?;
        if session.expires_at <= now as i64 {
            return Err(SessionError::Invalid);
        }
        let account = Accounts::find_by_id(session.account_id)
            .one(&self.db)
            .await
            .map_err(|_| SessionError::Inactive)?
            .ok_or(SessionError::Inactive)?;
        if !account.active {
            return Err(SessionError::Inactive);
        }
        if session.created_at < account.sessions_ended_before {
            // Ended by deactivation: the account is active again, this
            // session still is not.
            return Err(SessionError::Inactive);
        }
        Ok(account.id)
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

    async fn account(&self, account_id: uuid::Uuid) -> Option<Account> {
        Accounts::find_by_id(account_id)
            .one(&self.db)
            .await
            .ok()
            .flatten()
            .map(Self::account_row)
    }

    async fn link(&self, account_id: uuid::Uuid) -> Result<RosterPatient, LinkError> {
        let account = Accounts::find_by_id(account_id)
            .one(&self.db)
            .await
            .map_err(|_| LinkError::Unlinked)?
            .ok_or(LinkError::Unlinked)?;
        Self::account_row(account).link.ok_or(LinkError::Unlinked)
    }

    async fn linked_accounts(&self, practice_id: &str) -> Vec<(Account, RosterPatient)> {
        let mut rows: Vec<(Account, RosterPatient)> = Links::find()
            .filter(crate::migration::link::Column::PracticeId.eq(practice_id))
            .find_with_related(Accounts)
            .all(&self.db)
            .await
            .unwrap_or_default()
            .into_iter()
            .filter_map(|(link, accounts)| {
                let account = accounts.into_iter().next()?;
                Some((
                    Self::account_row(account),
                    RosterPatient {
                        practice_id: link.practice_id,
                        patient_id: link.patient_id,
                    },
                ))
            })
            .collect();
        rows.sort_by_key(|(left, _)| left.id);
        rows
    }

    async fn grant_consent(&self, account_id: uuid::Uuid, now: u64) -> Result<(), LinkError> {
        let account = Accounts::find_by_id(account_id)
            .one(&self.db)
            .await
            .map_err(|_| LinkError::Unlinked)?
            .ok_or(LinkError::Unlinked)?;
        if account.linked_practice_id.is_none() {
            return Err(LinkError::Unlinked);
        }
        // Re-consenting after a withdrawal restores sharing: the row's
        // `withdrawn_at` clears and the consent second moves forward.
        let existing = Consents::find_by_id(account_id)
            .one(&self.db)
            .await
            .map_err(|_| LinkError::Unlinked)?;
        let active = match existing {
            Some(row) => {
                let mut update: crate::migration::ConsentActiveModel = row.into();
                update.consented_at = Set(now as i64);
                update.withdrawn_at = Set(None);
                update
            }
            None => crate::migration::ConsentActiveModel {
                account_id: Set(account_id),
                consented_at: Set(now as i64),
                withdrawn_at: Set(None),
            },
        };
        // Explicit upsert: a natural primary key must not make a new row an UPDATE.
        Consents::insert(active)
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(
                    crate::migration::consent::Column::AccountId,
                )
                .update_columns([
                    crate::migration::consent::Column::ConsentedAt,
                    crate::migration::consent::Column::WithdrawnAt,
                ])
                .to_owned(),
            )
            .exec(&self.db)
            .await
            .map_err(|_| LinkError::Unlinked)?;
        Ok(())
    }

    async fn withdraw_consent(&self, account_id: uuid::Uuid, now: u64) -> Result<(), LinkError> {
        let account = Accounts::find_by_id(account_id)
            .one(&self.db)
            .await
            .map_err(|_| LinkError::Unlinked)?
            .ok_or(LinkError::Unlinked)?;
        if account.linked_practice_id.is_none() {
            return Err(LinkError::Unlinked);
        }
        if let Some(row) = Consents::find_by_id(account_id)
            .one(&self.db)
            .await
            .map_err(|_| LinkError::Unlinked)?
        {
            let mut update: crate::migration::ConsentActiveModel = row.into();
            update.withdrawn_at = Set(Some(now as i64));
            update
                .update(&self.db)
                .await
                .map_err(|_| LinkError::Unlinked)?;
        }
        Ok(())
    }

    async fn consent(&self, account_id: uuid::Uuid) -> Result<Consent, LinkError> {
        let account = Accounts::find_by_id(account_id)
            .one(&self.db)
            .await
            .map_err(|_| LinkError::Unlinked)?
            .ok_or(LinkError::Unlinked)?;
        if account.linked_practice_id.is_none() {
            return Err(LinkError::Unlinked);
        }
        Ok(Consents::find_by_id(account_id)
            .one(&self.db)
            .await
            .map_err(|_| LinkError::Unlinked)?
            .map(Self::consent_row)
            .unwrap_or(Consent::None))
    }

    async fn set_premium(
        &self,
        patient: &RosterPatient,
        premium: bool,
        now: u64,
    ) -> Result<(), LinkError> {
        let linked = Links::find()
            .filter(crate::migration::link::Column::PracticeId.eq(&patient.practice_id))
            .filter(crate::migration::link::Column::PatientId.eq(&patient.patient_id))
            .one(&self.db)
            .await
            .map_err(|_| LinkError::Unlinked)?
            .is_some();
        if !linked {
            return Err(LinkError::Unlinked);
        }
        let existing = Premiums::find_by_id(patient_key(patient))
            .one(&self.db)
            .await
            .map_err(|_| LinkError::Unlinked)?;
        let active = match existing {
            Some(row) => {
                let mut update: crate::migration::PremiumActiveModel = row.into();
                update.premium = Set(premium);
                update.cadence = Set(cadence_for(premium).to_owned());
                update
            }
            None => crate::migration::PremiumActiveModel {
                patient_key: Set(patient_key(patient)),
                premium: Set(premium),
                cadence: Set(cadence_for(premium).to_owned()),
                created_at: Set(now as i64),
            },
        };
        Premiums::insert(active)
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(
                    crate::migration::premium::Column::PatientKey,
                )
                .update_columns([
                    crate::migration::premium::Column::Premium,
                    crate::migration::premium::Column::Cadence,
                ])
                .to_owned(),
            )
            .exec(&self.db)
            .await
            .map_err(|_| LinkError::Unlinked)?;
        Ok(())
    }

    async fn premium(&self, patient: &RosterPatient) -> Premium {
        Premiums::find_by_id(patient_key(patient))
            .one(&self.db)
            .await
            .ok()
            .flatten()
            .map(|row| Premium {
                premium: row.premium,
                cadence: row.cadence,
            })
            .unwrap_or(Premium {
                premium: false,
                cadence: cadence_for(false).to_owned(),
            })
    }
}

/// The premium row's key: one practice-scoped roster patient.
fn patient_key(patient: &RosterPatient) -> String {
    format!("{}:{}", patient.practice_id, patient.patient_id)
}
