//! The Postgres `FrontDoorStore` through SeaORM.
//!
//! Same contract as the memory store; durable truth lives in the
//! `front_door_*` tables the crate's migration chain owns.

use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

use crate::migration::{
    Accounts, Connections, ForMyDoctorShares, Invitations, Sessions, SharedRecords, SignInTokens,
};
use crate::{
    Account, AccountConnection, ConnectionEndReason, ConnectionStanding, ConsentedPatientLink,
    ForMyDoctorError, ForMyDoctorSendRequest, ForMyDoctorShare, ForMyDoctorShareStatus,
    FrontDoorStore, Invitation, InviteError, LinkError, RosterConnectionStatus, RosterPatient,
    SIGN_IN_TOKEN_TTL_SECONDS, SessionError, SharedPatientRecord, SharedPatientRecordInput,
    SignInError, mint_token, resolve_for_my_doctor_target, resolve_roster_status, token_hash,
};
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectOptions, ConnectionTrait, Database, DatabaseConnection,
    DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait,
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
        Connections::find().one(&self.db).await?;
        Invitations::find().one(&self.db).await?;
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
        ForMyDoctorShares::find().one(&self.db).await?;
        SharedRecords::find().one(&self.db).await?;
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

    fn connection_standing(row: &crate::migration::ConnectionRow) -> Option<ConnectionStanding> {
        let connected_at = row.connected_at.max(0) as u64;
        match row.status.as_str() {
            "connected" => Some(ConnectionStanding::Connected { connected_at }),
            "ended" => {
                let ended_at = row.ended_at?.max(0) as u64;
                let reason = ConnectionEndReason::parse(row.end_reason.as_deref()?)?;
                Some(ConnectionStanding::Ended {
                    connected_at,
                    ended_at,
                    reason,
                })
            }
            _ => None,
        }
    }

    fn account_connection(row: crate::migration::ConnectionRow) -> Option<AccountConnection> {
        let standing = Self::connection_standing(&row)?;
        Some(AccountConnection {
            patient: RosterPatient {
                practice_id: row.practice_id,
                patient_id: row.patient_id,
            },
            standing,
        })
    }

    async fn load_account_with_connections(
        db: &impl ConnectionTrait,
        row: crate::migration::AccountRow,
    ) -> Result<Account, DbErr> {
        use crate::migration::connection::Column;
        let conn_rows = Connections::find()
            .filter(Column::AccountId.eq(row.id))
            .order_by_asc(Column::PracticeId)
            .order_by_asc(Column::PatientId)
            .all(db)
            .await?;
        let connections = conn_rows
            .into_iter()
            .filter_map(Self::account_connection)
            .collect();
        Ok(Account {
            id: row.id,
            email: row.email,
            display_name: row.display_name,
            active: row.active,
            connections,
            sessions_ended_before: row.sessions_ended_before.max(0) as u64,
        })
    }

    fn invitation_row(row: crate::migration::InvitationRow) -> Invitation {
        Invitation {
            patient: RosterPatient {
                practice_id: row.practice_id,
                patient_id: row.patient_id,
            },
            email: row.email,
            display_name: row.display_name,
            invited_at: row.created_at.max(0) as u64,
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
        use crate::migration::{connection, invitation};
        let key = email.to_lowercase();
        let active_existing = Connections::find()
            .filter(connection::Column::PracticeId.eq(&patient.practice_id))
            .filter(connection::Column::PatientId.eq(&patient.patient_id))
            .filter(connection::Column::Status.eq("connected"))
            .one(&self.db)
            .await
            .map_err(|_| InviteError::Conflict)?;
        if active_existing.is_some() {
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
        };
        Invitations::insert(active)
            .on_conflict(
                sea_orm::sea_query::OnConflict::columns([
                    invitation::Column::PracticeId,
                    invitation::Column::PatientId,
                ])
                .update_columns([
                    invitation::Column::Email,
                    invitation::Column::DisplayName,
                    invitation::Column::TokenHash,
                    invitation::Column::CreatedAt,
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
            .order_by_desc(crate::migration::invitation::Column::CreatedAt)
            .order_by_desc(crate::migration::invitation::Column::Id)
            .one(&self.db)
            .await
            .ok()
            .flatten()?;
        Some(Self::invitation_row(row))
    }

    async fn invitation_for(&self, patient: &RosterPatient) -> Option<Invitation> {
        use crate::migration::invitation::Column;
        let row = Invitations::find()
            .filter(Column::PracticeId.eq(&patient.practice_id))
            .filter(Column::PatientId.eq(&patient.patient_id))
            .one(&self.db)
            .await
            .ok()
            .flatten()?;
        Some(Self::invitation_row(row))
    }

    async fn redeem_invitation(
        &self,
        email: &str,
        token: &str,
        now: u64,
    ) -> Result<(uuid::Uuid, String), SignInError> {
        use crate::migration::{account, connection, invitation};
        let key = email.to_lowercase();
        let hash = token_hash(token).to_vec();
        let txn = self.db.begin().await.map_err(|_| SignInError::Invalid)?;

        let invitation = Invitations::find()
            .filter(invitation::Column::Email.eq(&key))
            .filter(invitation::Column::TokenHash.eq(hash))
            .lock_exclusive()
            .one(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?
            .ok_or(SignInError::Invalid)?;

        let account_id = if let Some(existing) = Accounts::find()
            .filter(account::Column::Email.eq(&key))
            .lock_exclusive()
            .one(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?
        {
            if !existing.active {
                return Err(SignInError::Inactive);
            }
            existing.id
        } else {
            let new_account = crate::migration::AccountActiveModel {
                id: Set(uuid::Uuid::new_v4()),
                email: Set(key),
                display_name: Set(invitation.display_name.clone()),
                active: Set(true),
                sessions_ended_before: Set(0),
                created_at: Set(now as i64),
            };
            new_account
                .insert(&txn)
                .await
                .map_err(|_| SignInError::Invalid)?
                .id
        };

        // Delete the invitation row inside the transaction: a raced second
        // redeem finds nothing, so each invitation redeems once.
        let deleted = Invitations::delete_by_id(invitation.id)
            .exec(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?;
        if deleted.rows_affected != 1 {
            return Err(SignInError::Invalid);
        }

        let conflict_other = Connections::find()
            .filter(connection::Column::PracticeId.eq(&invitation.practice_id))
            .filter(connection::Column::PatientId.eq(&invitation.patient_id))
            .filter(connection::Column::Status.eq("connected"))
            .filter(connection::Column::AccountId.ne(account_id))
            .one(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?
            .is_some();
        if conflict_other {
            return Err(SignInError::Invalid);
        }

        let existing_conn = Connections::find_by_id((
            account_id,
            invitation.practice_id.clone(),
            invitation.patient_id.clone(),
        ))
        .one(&txn)
        .await
        .map_err(|_| SignInError::Invalid)?;
        match existing_conn {
            Some(row) => {
                let mut active: crate::migration::ConnectionActiveModel = row.into();
                active.status = Set("connected".to_owned());
                active.connected_at = Set(now as i64);
                active.ended_at = Set(None);
                active.end_reason = Set(None);
                active.updated_at = Set(now as i64);
                active
                    .update(&txn)
                    .await
                    .map_err(|_| SignInError::Invalid)?;
            }
            None => {
                crate::migration::ConnectionActiveModel {
                    account_id: Set(account_id),
                    practice_id: Set(invitation.practice_id),
                    patient_id: Set(invitation.patient_id),
                    status: Set("connected".to_owned()),
                    connected_at: Set(now as i64),
                    ended_at: Set(None),
                    end_reason: Set(None),
                    created_at: Set(now as i64),
                    updated_at: Set(now as i64),
                }
                .insert(&txn)
                .await
                .map_err(|_| SignInError::Invalid)?;
            }
        }

        let session_token = mint_token();
        crate::migration::SessionActiveModel {
            id: Default::default(),
            account_id: Set(account_id),
            token_hash: Set(token_hash(&session_token).to_vec()),
            created_at: Set(now as i64),
        }
        .insert(&txn)
        .await
        .map_err(|_| SignInError::Invalid)?;
        txn.commit().await.map_err(|_| SignInError::Invalid)?;
        Ok((account_id, session_token))
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
        if crate::migration::identity::Entity::find()
            .filter(crate::migration::identity::Column::AccountId.eq(account.id))
            .one(&self.db)
            .await
            .map_err(|_| SignInError::Invalid)?
            .is_some()
        {
            return Err(SignInError::Invalid);
        }
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
        let consumed = SignInTokens::delete_by_id(pending.id)
            .exec(&txn)
            .await
            .map_err(|_| SignInError::Invalid)?;
        if consumed.rows_affected != 1 {
            return Err(SignInError::Invalid);
        }
        let session_token = mint_token();
        crate::migration::SessionActiveModel {
            id: Default::default(),
            account_id: Set(account.id),
            token_hash: Set(token_hash(&session_token).to_vec()),
            created_at: Set(now as i64),
        }
        .insert(&txn)
        .await
        .map_err(|_| SignInError::Invalid)?;
        txn.commit().await.map_err(|_| SignInError::Invalid)?;
        Ok((account.id, session_token))
    }

    async fn session(&self, token: &str, _now: u64) -> Result<uuid::Uuid, SessionError> {
        let hash = token_hash(token);
        let session = Sessions::find()
            .filter(crate::migration::session::Column::TokenHash.eq(hash.to_vec()))
            .one(&self.db)
            .await
            .map_err(|_| SessionError::Invalid)?
            .ok_or(SessionError::Invalid)?;
        let account = Accounts::find_by_id(session.account_id)
            .one(&self.db)
            .await
            .map_err(|_| SessionError::Inactive)?
            .ok_or(SessionError::Inactive)?;
        if !account.active {
            return Err(SessionError::Inactive);
        }
        if session.created_at < account.sessions_ended_before {
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
        let row = Accounts::find()
            .filter(crate::migration::account::Column::Email.eq(email.to_lowercase()))
            .one(&self.db)
            .await
            .ok()
            .flatten()?;
        Self::load_account_with_connections(&self.db, row)
            .await
            .ok()
    }

    async fn account(&self, account_id: uuid::Uuid) -> Option<Account> {
        let row = Accounts::find_by_id(account_id)
            .one(&self.db)
            .await
            .ok()
            .flatten()?;
        Self::load_account_with_connections(&self.db, row)
            .await
            .ok()
    }

    async fn connections(
        &self,
        account_id: uuid::Uuid,
    ) -> Result<Vec<AccountConnection>, LinkError> {
        let account = Accounts::find_by_id(account_id)
            .one(&self.db)
            .await
            .map_err(|_| LinkError::Unlinked)?
            .ok_or(LinkError::Unlinked)?;
        let hydrated = Self::load_account_with_connections(&self.db, account)
            .await
            .map_err(|_| LinkError::Unlinked)?;
        Ok(hydrated.connections)
    }

    async fn connected_accounts(&self, practice_id: &str) -> Vec<(Account, RosterPatient)> {
        use crate::migration::connection::Column;
        let conn_rows = Connections::find()
            .filter(Column::PracticeId.eq(practice_id))
            .filter(Column::Status.eq("connected"))
            .order_by_asc(Column::AccountId)
            .order_by_asc(Column::PatientId)
            .all(&self.db)
            .await
            .unwrap_or_default();
        let mut out = Vec::with_capacity(conn_rows.len());
        for row in conn_rows {
            if let Some(account) = self.account(row.account_id).await {
                out.push((
                    account,
                    RosterPatient {
                        practice_id: row.practice_id,
                        patient_id: row.patient_id,
                    },
                ));
            }
        }
        out
    }

    async fn end_connection(
        &self,
        account_id: uuid::Uuid,
        patient: &RosterPatient,
        reason: ConnectionEndReason,
        now: u64,
    ) -> Result<(), LinkError> {
        let txn = self.db.begin().await.map_err(|_| LinkError::Unlinked)?;
        let existing = Connections::find_by_id((
            account_id,
            patient.practice_id.clone(),
            patient.patient_id.clone(),
        ))
        .lock_exclusive()
        .one(&txn)
        .await
        .map_err(|_| LinkError::Unlinked)?
        .ok_or(LinkError::Unlinked)?;
        if existing.status != "connected" {
            return Err(LinkError::Unlinked);
        }
        let mut active: crate::migration::ConnectionActiveModel = existing.into();
        active.status = Set("ended".to_owned());
        active.ended_at = Set(Some(now as i64));
        active.end_reason = Set(Some(reason.as_str().to_owned()));
        active.updated_at = Set(now as i64);
        active.update(&txn).await.map_err(|_| LinkError::Unlinked)?;
        txn.commit().await.map_err(|_| LinkError::Unlinked)?;
        Ok(())
    }

    async fn roster_connection(&self, patient: &RosterPatient) -> RosterConnectionStatus {
        use crate::migration::{connection, invitation};
        let conn_rows = Connections::find()
            .filter(connection::Column::PracticeId.eq(&patient.practice_id))
            .filter(connection::Column::PatientId.eq(&patient.patient_id))
            .all(&self.db)
            .await
            .unwrap_or_default();
        let standings: Vec<(uuid::Uuid, ConnectionStanding)> = conn_rows
            .iter()
            .filter_map(|row| Some((row.account_id, Self::connection_standing(row)?)))
            .collect();
        let inv = Invitations::find()
            .filter(invitation::Column::PracticeId.eq(&patient.practice_id))
            .filter(invitation::Column::PatientId.eq(&patient.patient_id))
            .one(&self.db)
            .await
            .ok()
            .flatten()
            .map(Self::invitation_row);
        resolve_roster_status(standings.iter().map(|(aid, st)| (*aid, st)), inv.as_ref())
    }

    async fn practice_connections(
        &self,
        practice_id: &str,
    ) -> Result<BTreeMap<String, RosterConnectionStatus>, ForMyDoctorError> {
        use crate::migration::{connection, invitation};
        let conn_rows = Connections::find()
            .filter(connection::Column::PracticeId.eq(practice_id))
            .all(&self.db)
            .await?;
        let inv_rows = Invitations::find()
            .filter(invitation::Column::PracticeId.eq(practice_id))
            .all(&self.db)
            .await?;

        let mut by_patient_conns: BTreeMap<String, Vec<(uuid::Uuid, ConnectionStanding)>> =
            BTreeMap::new();
        let mut patient_ids = BTreeSet::new();
        for row in conn_rows {
            if let Some(standing) = Self::connection_standing(&row) {
                patient_ids.insert(row.patient_id.clone());
                by_patient_conns
                    .entry(row.patient_id)
                    .or_default()
                    .push((row.account_id, standing));
            }
        }
        let mut by_patient_inv: BTreeMap<String, Invitation> = BTreeMap::new();
        for row in inv_rows {
            patient_ids.insert(row.patient_id.clone());
            by_patient_inv.insert(row.patient_id.clone(), Self::invitation_row(row));
        }

        let mut out = BTreeMap::new();
        for patient_id in patient_ids {
            let conns = by_patient_conns
                .get(&patient_id)
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            let inv = by_patient_inv.get(&patient_id);
            let status = resolve_roster_status(conns.iter().map(|(aid, st)| (*aid, st)), inv);
            if !matches!(status, RosterConnectionStatus::NotInvited) {
                out.insert(patient_id, status);
            }
        }
        Ok(out)
    }

    async fn send_for_my_doctor(
        &self,
        account_id: uuid::Uuid,
        request: ForMyDoctorSendRequest,
        now: u64,
    ) -> Result<ForMyDoctorShare, ForMyDoctorError> {
        use crate::migration::{connection, for_my_doctor_share};
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
        let active_rows = Connections::find()
            .filter(connection::Column::AccountId.eq(account_id))
            .filter(connection::Column::Status.eq("connected"))
            .order_by_asc(connection::Column::PracticeId)
            .order_by_asc(connection::Column::PatientId)
            .all(&txn)
            .await?;
        let active_pairs: Vec<(RosterPatient, u64)> = active_rows
            .into_iter()
            .map(|r| {
                (
                    RosterPatient {
                        practice_id: r.practice_id,
                        patient_id: r.patient_id,
                    },
                    r.connected_at.max(0) as u64,
                )
            })
            .collect();
        let active_refs: Vec<(&RosterPatient, u64)> =
            active_pairs.iter().map(|(p, ts)| (p, *ts)).collect();
        let (patient, consent_at) =
            resolve_for_my_doctor_target(&active_refs, request.target.as_ref())?;

        if let Some(existing) = ForMyDoctorShares::find()
            .filter(for_my_doctor_share::Column::AccountId.eq(account_id))
            .filter(for_my_doctor_share::Column::PracticeId.eq(&patient.practice_id))
            .filter(for_my_doctor_share::Column::PatientId.eq(&patient.patient_id))
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

        let share_id = format!("fmd-{}", uuid::Uuid::new_v4().simple());
        crate::migration::ForMyDoctorShareActiveModel {
            share_id: Set(share_id.clone()),
            account_id: Set(account_id),
            practice_id: Set(patient.practice_id.clone()),
            patient_id: Set(patient.patient_id.clone()),
            share_key: Set(share_key.to_owned()),
            title: Set(title.to_owned()),
            summary_text: Set(summary_text.to_owned()),
            origin_label: Set(origin_label.clone()),
            sent_at: Set(now as i64),
            consent_at: Set(consent_at as i64),
            received_at: Set(None),
        }
        .insert(&txn)
        .await?;
        txn.commit().await?;
        Ok(ForMyDoctorShare {
            share_id,
            account_id,
            patient,
            share_key: share_key.to_owned(),
            title: title.to_owned(),
            summary_text: summary_text.to_owned(),
            origin_label,
            sent_at: now,
            consent_at,
            standing: ForMyDoctorShareStatus::Sent,
        })
    }

    async fn for_my_doctor_shares_for_account(
        &self,
        account_id: uuid::Uuid,
    ) -> Result<Vec<ForMyDoctorShare>, ForMyDoctorError> {
        use crate::migration::{connection, for_my_doctor_share};
        let has_connection = Connections::find()
            .filter(connection::Column::AccountId.eq(account_id))
            .one(&self.db)
            .await?
            .is_some();
        if !has_connection {
            return Err(ForMyDoctorError::Unlinked);
        }
        let rows = ForMyDoctorShares::find()
            .filter(for_my_doctor_share::Column::AccountId.eq(account_id))
            .order_by_asc(for_my_doctor_share::Column::SentAt)
            .order_by_asc(for_my_doctor_share::Column::ShareId)
            .all(&self.db)
            .await?;
        Ok(rows.into_iter().map(share_row).collect())
    }

    async fn pending_for_my_doctor_shares_for_practice(
        &self,
        practice_id: &str,
    ) -> Result<Vec<ForMyDoctorShare>, ForMyDoctorError> {
        use crate::migration::for_my_doctor_share::Column;
        let rows = ForMyDoctorShares::find()
            .filter(Column::PracticeId.eq(practice_id))
            .filter(Column::ReceivedAt.is_null())
            .order_by_asc(Column::SentAt)
            .order_by_asc(Column::ShareId)
            .all(&self.db)
            .await?;
        let mut out = Vec::new();
        for row in rows {
            let connected = Connections::find_by_id((
                row.account_id,
                row.practice_id.clone(),
                row.patient_id.clone(),
            ))
            .one(&self.db)
            .await?
            .is_some_and(|c| c.status == "connected");
            if connected {
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
    ) -> Result<ForMyDoctorShare, ForMyDoctorError> {
        let txn = self.db.begin().await?;
        let existing = ForMyDoctorShares::find_by_id(share_id.to_owned())
            .one(&txn)
            .await?
            .filter(|row| row.practice_id == practice_id)
            .ok_or(ForMyDoctorError::NotFound)?;
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
        records: &[SharedPatientRecordInput],
        now: u64,
    ) -> Result<(), ForMyDoctorError> {
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
            SharedRecords::insert(crate::migration::SharedRecordActiveModel {
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
    ) -> Result<Vec<ConsentedPatientLink>, ForMyDoctorError> {
        use crate::migration::connection::Column;
        let rows = Connections::find()
            .filter(Column::PracticeId.eq(practice_id))
            .filter(Column::Status.eq("connected"))
            .order_by_asc(Column::AccountId)
            .order_by_asc(Column::PatientId)
            .all(&self.db)
            .await?;
        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            let Some(acct_row) = Accounts::find_by_id(row.account_id).one(&self.db).await? else {
                continue;
            };
            let account = Self::load_account_with_connections(&self.db, acct_row).await?;
            out.push(ConsentedPatientLink {
                account,
                patient: RosterPatient {
                    practice_id: row.practice_id,
                    patient_id: row.patient_id,
                },
                consented_at: row.connected_at.max(0) as u64,
            });
        }
        Ok(out)
    }

    async fn shared_records_for_practice_patient(
        &self,
        practice_id: &str,
        patient_id: &str,
    ) -> Result<Vec<SharedPatientRecord>, ForMyDoctorError> {
        use crate::migration::connection;
        use crate::migration::shared_record::Column;
        let conn_rows = Connections::find()
            .filter(connection::Column::PracticeId.eq(practice_id))
            .filter(connection::Column::PatientId.eq(patient_id))
            .filter(connection::Column::Status.eq("connected"))
            .order_by_asc(connection::Column::AccountId)
            .all(&self.db)
            .await?;
        let mut out = Vec::new();
        for conn in conn_rows {
            let recs = SharedRecords::find()
                .filter(Column::AccountId.eq(conn.account_id))
                .order_by_asc(Column::RecordId)
                .all(&self.db)
                .await?;
            for rec in recs {
                out.push(SharedPatientRecord {
                    account_id: conn.account_id,
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

    async fn connected_care_practices(&self) -> Result<Vec<String>, ForMyDoctorError> {
        use crate::migration::connection::Column;
        let rows = Connections::find()
            .filter(Column::Status.eq("connected"))
            .select_only()
            .column(Column::PracticeId)
            .distinct()
            .into_tuple::<String>()
            .all(&self.db)
            .await?;
        let mut practices = BTreeSet::new();
        practices.extend(rows);
        Ok(practices.into_iter().collect())
    }
}

fn share_row(row: crate::migration::ForMyDoctorShareRow) -> ForMyDoctorShare {
    ForMyDoctorShare {
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
            Some(ts) => ForMyDoctorShareStatus::Received {
                received_at: ts.max(0) as u64,
            },
            None => ForMyDoctorShareStatus::Sent,
        },
    }
}
