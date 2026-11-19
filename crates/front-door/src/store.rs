//! The Postgres `FrontDoorStore` through SeaORM.
//!
//! Same contract as the memory store; durable truth lives in the
//! `front_door_*` tables the crate's migration chain owns.

use std::time::Duration;

use crate::migration::{Accounts, Consents, Invitations, Links, Premiums, Sessions, SignInTokens};
use crate::{
    Account, Consent, FrontDoorStore, Invitation, InviteError, LinkError, Premium, RosterPatient,
    SESSION_TTL_SECONDS, SIGN_IN_TOKEN_TTL_SECONDS, SessionError, SignInError, mint_token,
    token_hash,
};
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectOptions, Database, DatabaseConnection, DbErr,
    EntityTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

/// The SQL-backed store.
#[derive(Debug, Clone)]
pub struct PostgresFrontDoorStore {
    pub(crate) db: DatabaseConnection,
}

impl PostgresFrontDoorStore {
    /// Connects without applying migrations; retained for existing callers.
    pub async fn connect(database_url: &str) -> Result<Self, DbErr> {
        Self::connect_without_migrations(database_url).await
    }

    /// Opens a bounded runtime pool without creating or upgrading any schema.
    ///
    /// Run `front-door-migrate` with the migration identity first. This
    /// constructor only establishes connectivity: callers must separately
    /// check readiness before accepting traffic. It does not adopt legacy
    /// migration records or repair tables; runtime credentials need no
    /// schema-owner rights.
    pub async fn connect_without_migrations(database_url: &str) -> Result<Self, DbErr> {
        let mut options = ConnectOptions::new(database_url.to_owned());
        options
            .max_connections(4)
            .connect_timeout(Duration::from_secs(10));
        let db = Database::connect(options).await?;
        Ok(Self::from_connection(db))
    }

    /// Uses an existing pool without applying migrations or changing its limits.
    pub fn from_connection(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Checks every owned table without creating a migration ledger or writing data.
    pub async fn check_schema(&self) -> Result<(), DbErr> {
        Accounts::find().one(&self.db).await?;
        Consents::find().one(&self.db).await?;
        Invitations::find().one(&self.db).await?;
        Links::find().one(&self.db).await?;
        Premiums::find().one(&self.db).await?;
        Sessions::find().one(&self.db).await?;
        SignInTokens::find().one(&self.db).await?;
        crate::migration::identity::Entity::find()
            .one(&self.db)
            .await?;
        crate::migration::permission::Entity::find()
            .one(&self.db)
            .await?;
        crate::migration::restriction::Entity::find()
            .one(&self.db)
            .await?;
        crate::migration::ForMyDoctorShares::find()
            .one(&self.db)
            .await?;
        crate::migration::SharedRecords::find()
            .one(&self.db)
            .await?;
        Ok(())
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
        // Identity Platform accounts cannot fall back to the legacy email proof
        // endpoint (which is not their credential authority).
        if crate::migration::identity::Entity::find()
            .filter(crate::migration::identity::Column::AccountId.eq(account.id))
            .one(&self.db)
            .await
            .map_err(|_| SignInError::Invalid)?
            .is_some()
        {
            return Err(SignInError::Invalid);
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
        if crate::migration::identity::Entity::find()
            .filter(crate::migration::identity::Column::AccountId.eq(account.id))
            .one(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?
            .is_some()
        {
            return Err(SignInError::Invalid);
        }
        // Only the transaction that consumes the token may mint a session.
        // A concurrent verifier can have read the same pending row already.
        let consumed = SignInTokens::delete_by_id(pending.id)
            .exec(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?;
        if consumed.rows_affected != 1 {
            return Err(SignInError::Invalid);
        }
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
                update
            }
            None => crate::migration::PremiumActiveModel {
                patient_key: Set(patient_key(patient)),
                premium: Set(premium),
                created_at: Set(now as i64),
            },
        };
        Premiums::insert(active)
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(
                    crate::migration::premium::Column::PatientKey,
                )
                .update_columns([crate::migration::premium::Column::Premium])
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
            })
            .unwrap_or(Premium { premium: false })
    }

    async fn send_for_my_doctor(
        &self,
        account_id: uuid::Uuid,
        request: crate::ForMyDoctorSendRequest,
        now: u64,
    ) -> Result<crate::ForMyDoctorShare, crate::ForMyDoctorError> {
        use crate::ForMyDoctorError;
        use crate::migration::for_my_doctor_share;
        let share_key = request.share_key.trim();
        let title = request.title.trim();
        let summary_text = request.summary_text.trim();
        if share_key.is_empty() {
            return Err(ForMyDoctorError::InvalidInput(
                "share_key must be non-empty",
            ));
        }
        if title.is_empty() {
            return Err(ForMyDoctorError::InvalidInput("title must be non-empty"));
        }
        if summary_text.is_empty() {
            return Err(ForMyDoctorError::InvalidInput(
                "summary_text must be non-empty",
            ));
        }
        let origin_label = request
            .origin_label
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("Duet — For my doctor")
            .to_owned();

        let txn = self.db.begin().await?;
        let account = Accounts::find_by_id(account_id)
            .one(&txn)
            .await?
            .ok_or(ForMyDoctorError::Unlinked)?;
        let (Some(practice_id), Some(patient_id)) =
            (account.linked_practice_id, account.linked_patient_id)
        else {
            return Err(ForMyDoctorError::Unlinked);
        };
        if let Some(existing) = crate::migration::ForMyDoctorShares::find()
            .filter(for_my_doctor_share::Column::AccountId.eq(account_id))
            .filter(for_my_doctor_share::Column::ShareKey.eq(share_key))
            .one(&txn)
            .await?
        {
            if existing.title == title
                && existing.summary_text == summary_text
                && existing.origin_label == origin_label
            {
                txn.commit().await?;
                return Ok(share_row(existing));
            }
            return Err(ForMyDoctorError::Conflict);
        }
        let consent = Consents::find_by_id(account_id)
            .one(&txn)
            .await?
            .map(Self::consent_row)
            .unwrap_or(Consent::None);
        let Consent::Granted { at: consent_at } = consent else {
            return Err(ForMyDoctorError::ConsentRequired);
        };
        let share_id = format!("fmd-{}", uuid::Uuid::new_v4().simple());
        let active = crate::migration::ForMyDoctorShareActiveModel {
            share_id: Set(share_id.clone()),
            account_id: Set(account_id),
            practice_id: Set(practice_id.clone()),
            patient_id: Set(patient_id.clone()),
            share_key: Set(share_key.to_owned()),
            title: Set(title.to_owned()),
            summary_text: Set(summary_text.to_owned()),
            origin_label: Set(origin_label.clone()),
            sent_at: Set(now as i64),
            consent_at: Set(consent_at as i64),
            received_at: Set(None),
        };
        active.insert(&txn).await?;
        txn.commit().await?;
        Ok(crate::ForMyDoctorShare {
            share_id,
            account_id,
            patient: RosterPatient {
                practice_id,
                patient_id,
            },
            share_key: share_key.to_owned(),
            title: title.to_owned(),
            summary_text: summary_text.to_owned(),
            origin_label,
            sent_at: now,
            consent_at,
            standing: crate::ForMyDoctorShareStatus::Sent,
        })
    }

    async fn for_my_doctor_shares_for_account(
        &self,
        account_id: uuid::Uuid,
    ) -> Result<Vec<crate::ForMyDoctorShare>, crate::ForMyDoctorError> {
        use crate::migration::for_my_doctor_share::Column;
        let account = Accounts::find_by_id(account_id)
            .one(&self.db)
            .await?
            .ok_or(crate::ForMyDoctorError::Unlinked)?;
        if account.linked_practice_id.is_none() || account.linked_patient_id.is_none() {
            return Err(crate::ForMyDoctorError::Unlinked);
        }
        let rows = crate::migration::ForMyDoctorShares::find()
            .filter(Column::AccountId.eq(account_id))
            .order_by_asc(Column::SentAt)
            .order_by_asc(Column::ShareId)
            .all(&self.db)
            .await?;
        Ok(rows.into_iter().map(share_row).collect())
    }

    async fn pending_for_my_doctor_shares_for_practice(
        &self,
        practice_id: &str,
    ) -> Result<Vec<crate::ForMyDoctorShare>, crate::ForMyDoctorError> {
        use crate::migration::for_my_doctor_share::Column;
        let rows = crate::migration::ForMyDoctorShares::find()
            .filter(Column::PracticeId.eq(practice_id))
            .filter(Column::ReceivedAt.is_null())
            .order_by_asc(Column::SentAt)
            .order_by_asc(Column::ShareId)
            .all(&self.db)
            .await?;
        let mut out = Vec::new();
        for row in rows {
            let consented = Consents::find_by_id(row.account_id)
                .one(&self.db)
                .await?
                .is_some_and(|c| c.withdrawn_at.is_none());
            if consented {
                out.push(share_row(row));
            }
        }
        Ok(out)
    }

    async fn mark_for_my_doctor_received(
        &self,
        practice_id: &str,
        share_id: &str,
        received_at: u64,
    ) -> Result<crate::ForMyDoctorShare, crate::ForMyDoctorError> {
        let txn = self.db.begin().await?;
        let existing = crate::migration::ForMyDoctorShares::find_by_id(share_id.to_owned())
            .one(&txn)
            .await?
            .filter(|row| row.practice_id == practice_id)
            .ok_or(crate::ForMyDoctorError::NotFound)?;
        let model = if existing.received_at.is_none() {
            let mut active: crate::migration::ForMyDoctorShareActiveModel = existing.into();
            active.received_at = Set(Some(received_at as i64));
            active.update(&txn).await?
        } else {
            existing
        };
        txn.commit().await?;
        Ok(share_row(model))
    }

    async fn upsert_shared_records(
        &self,
        account_id: uuid::Uuid,
        records: &[crate::SharedPatientRecordInput],
        now: u64,
    ) -> Result<(), crate::ForMyDoctorError> {
        use crate::ForMyDoctorError;
        use crate::migration::shared_record::Column;
        for rec in records {
            if rec.record_id.trim().is_empty()
                || rec.revision.trim().is_empty()
                || rec.record_type.trim().is_empty()
                || rec.title.trim().is_empty()
                || rec.content_text.trim().is_empty()
            {
                return Err(ForMyDoctorError::InvalidInput(
                    "shared record fields must be non-empty",
                ));
            }
        }
        let txn = self.db.begin().await?;
        let exists = Accounts::find_by_id(account_id).one(&txn).await?.is_some();
        if !exists {
            return Err(ForMyDoctorError::Unlinked);
        }
        for rec in records {
            crate::migration::SharedRecords::insert(crate::migration::SharedRecordActiveModel {
                account_id: Set(account_id),
                record_id: Set(rec.record_id.trim().to_owned()),
                revision: Set(rec.revision.trim().to_owned()),
                record_type: Set(rec.record_type.trim().to_owned()),
                title: Set(rec.title.trim().to_owned()),
                content_text: Set(rec.content_text.trim().to_owned()),
                recorded_at: Set(rec.recorded_at as i64),
                updated_at: Set(now as i64),
            })
            .on_conflict(
                sea_orm::sea_query::OnConflict::columns([Column::AccountId, Column::RecordId])
                    .update_columns([
                        Column::Revision,
                        Column::RecordType,
                        Column::Title,
                        Column::ContentText,
                        Column::RecordedAt,
                        Column::UpdatedAt,
                    ])
                    .to_owned(),
            )
            .exec(&txn)
            .await?;
        }
        txn.commit().await?;
        Ok(())
    }

    async fn consented_links_for_practice(
        &self,
        practice_id: &str,
    ) -> Result<Vec<crate::ConsentedPatientLink>, crate::ForMyDoctorError> {
        let rows = Accounts::find()
            .filter(crate::migration::account::Column::LinkedPracticeId.eq(practice_id))
            .filter(crate::migration::account::Column::LinkedPatientId.is_not_null())
            .order_by_asc(crate::migration::account::Column::Id)
            .all(&self.db)
            .await?;
        let mut out = Vec::new();
        for row in rows {
            let consent = Consents::find_by_id(row.id)
                .one(&self.db)
                .await?
                .map(Self::consent_row)
                .unwrap_or(Consent::None);
            let Consent::Granted { at } = consent else {
                continue;
            };
            let account = Self::account_row(row);
            if let Some(patient) = account.link.clone() {
                out.push(crate::ConsentedPatientLink {
                    account,
                    patient,
                    consented_at: at,
                });
            }
        }
        Ok(out)
    }

    async fn shared_records_for_practice_patient(
        &self,
        practice_id: &str,
        patient_id: &str,
    ) -> Result<Vec<crate::SharedPatientRecord>, crate::ForMyDoctorError> {
        use crate::migration::account;
        use crate::migration::shared_record::Column;
        let accounts = Accounts::find()
            .filter(account::Column::LinkedPracticeId.eq(practice_id))
            .filter(account::Column::LinkedPatientId.eq(patient_id))
            .order_by_asc(account::Column::Id)
            .all(&self.db)
            .await?;
        let mut out = Vec::new();
        for acct in accounts {
            let consented = Consents::find_by_id(acct.id)
                .one(&self.db)
                .await?
                .is_some_and(|c| c.withdrawn_at.is_none());
            if !consented {
                continue;
            }
            let recs = crate::migration::SharedRecords::find()
                .filter(Column::AccountId.eq(acct.id))
                .order_by_asc(Column::RecordId)
                .all(&self.db)
                .await?;
            for rec in recs {
                out.push(crate::SharedPatientRecord {
                    account_id: acct.id,
                    patient: RosterPatient {
                        practice_id: practice_id.to_owned(),
                        patient_id: patient_id.to_owned(),
                    },
                    record_id: rec.record_id,
                    revision: rec.revision,
                    record_type: rec.record_type,
                    title: rec.title,
                    content_text: rec.content_text,
                    recorded_at: rec.recorded_at.max(0) as u64,
                    updated_at: rec.updated_at.max(0) as u64,
                });
            }
        }
        Ok(out)
    }

    async fn connected_care_practices(&self) -> Result<Vec<String>, crate::ForMyDoctorError> {
        use crate::migration::{account, consent, for_my_doctor_share};
        use std::collections::BTreeSet;
        let consented_ids: Vec<uuid::Uuid> = Consents::find()
            .filter(consent::Column::WithdrawnAt.is_null())
            .all(&self.db)
            .await?
            .into_iter()
            .map(|c| c.account_id)
            .collect();
        if consented_ids.is_empty() {
            return Ok(Vec::new());
        }
        let mut practices = BTreeSet::new();
        let linked_accounts = Accounts::find()
            .filter(account::Column::Id.is_in(consented_ids.clone()))
            .filter(account::Column::LinkedPracticeId.is_not_null())
            .filter(account::Column::LinkedPatientId.is_not_null())
            .all(&self.db)
            .await?;
        for acct in linked_accounts {
            if let Some(practice_id) = acct.linked_practice_id {
                practices.insert(practice_id);
            }
        }
        let pending_shares = crate::migration::ForMyDoctorShares::find()
            .filter(for_my_doctor_share::Column::AccountId.is_in(consented_ids))
            .filter(for_my_doctor_share::Column::ReceivedAt.is_null())
            .all(&self.db)
            .await?;
        for share in pending_shares {
            practices.insert(share.practice_id);
        }
        Ok(practices.into_iter().collect())
    }
}

fn share_row(row: crate::migration::ForMyDoctorShareRow) -> crate::ForMyDoctorShare {
    crate::ForMyDoctorShare {
        share_id: row.share_id,
        account_id: row.account_id,
        patient: RosterPatient {
            practice_id: row.practice_id,
            patient_id: row.patient_id,
        },
        share_key: row.share_key,
        title: row.title,
        summary_text: row.summary_text,
        origin_label: row.origin_label,
        sent_at: row.sent_at.max(0) as u64,
        consent_at: row.consent_at.max(0) as u64,
        standing: match row.received_at {
            Some(ts) => crate::ForMyDoctorShareStatus::Received {
                received_at: ts.max(0) as u64,
            },
            None => crate::ForMyDoctorShareStatus::Sent,
        },
    }
}

/// The premium row's key: one practice-scoped roster patient.
fn patient_key(patient: &RosterPatient) -> String {
    format!("{}:{}", patient.practice_id, patient.patient_id)
}
