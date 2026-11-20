//! The Postgres `FrontDoorStore` through SeaORM.
//!
//! Same contract as the memory store; durable truth lives in the
//! `front_door_*` tables the crate's migration chain owns.

use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

use crate::migration::{
    Accounts, Connections, ForMyDoctorShares, InvitationCredentials, Invitations, Sessions,
    SharedRecords, SignInTokens,
};
use crate::{
    Account, AccountConnection, ConnectionEndReason, ConnectionStanding, ConsentedPatientLink,
    ForMyDoctorError, ForMyDoctorSendRequest, ForMyDoctorShare, ForMyDoctorShareStatus,
    FrontDoorStore, Invitation, InvitationAcceptError, InvitationCheckError, InvitationCredential,
    InvitationGateState, InvitationOpenStanding, InvitationSharingPreview, InviteError,
    InvitePatientParams, IssuedInvitation, LinkError, MAX_DOB_ATTEMPTS, RosterConnectionStatus,
    RosterPatient, SHARES_SUMMARY, SIGN_IN_TOKEN_TTL_SECONDS, SessionError, SharedPatientRecord,
    SharedPatientRecordInput, SignInError, format_birth_date, is_adult_on, mint_short_code,
    mint_token, normalize_short_code, parse_birth_date, resolve_for_my_doctor_target,
    resolve_roster_status, short_code_hash, token_hash,
};
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectOptions, ConnectionTrait, Database, DatabaseConnection,
    DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait,
};

enum DobCheckOutcome {
    Matched(InvitationSharingPreview),
    Rejected(InvitationCheckError),
}

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
        InvitationCredentials::find().one(&self.db).await?;
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

    async fn end_connection_row(
        txn: &impl ConnectionTrait,
        row: crate::migration::ConnectionRow,
        reason: ConnectionEndReason,
        now: u64,
    ) -> Result<AccountConnection, DbErr> {
        let patient = RosterPatient {
            practice_id: row.practice_id.clone(),
            patient_id: row.patient_id.clone(),
        };
        let connected_at = u64::try_from(row.connected_at).unwrap_or_default();
        let mut active: crate::migration::ConnectionActiveModel = row.into();
        active.status = Set("ended".to_owned());
        active.ended_at = Set(Some(now as i64));
        active.end_reason = Set(Some(reason.as_str().to_owned()));
        active.updated_at = Set(now as i64);
        active.update(txn).await?;
        Ok(AccountConnection {
            patient,
            standing: ConnectionStanding::Ended {
                connected_at,
                ended_at: now,
                reason,
            },
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
            practice_name: row.practice_name,
            physician_name: row.physician_name,
            expected_birth_date: row.expected_birth_date,
            wrong_dob_attempts: u8::try_from(row.wrong_dob_attempts.max(0)).unwrap_or(u8::MAX),
            locked_at: row.locked_at.map(|ts| ts.max(0) as u64),
            last_delivery_failed_at: row.last_delivery_failed_at.map(|ts| ts.max(0) as u64),
            last_delivery_error: row.last_delivery_error,
            updated_at: row.updated_at.max(0) as u64,
        }
    }

    async fn insert_invitation_credentials(
        txn: &sea_orm::DatabaseTransaction,
        invitation_id: i64,
        link_token: &str,
        short_code: &str,
        now: u64,
    ) -> Result<(), DbErr> {
        let now_i64 = now as i64;
        crate::migration::InvitationCredentialActiveModel {
            id: Default::default(),
            invitation_id: Set(invitation_id),
            kind: Set("link".to_owned()),
            secret_hash: Set(token_hash(link_token).to_vec()),
            created_at: Set(now_i64),
        }
        .insert(txn)
        .await?;
        crate::migration::InvitationCredentialActiveModel {
            id: Default::default(),
            invitation_id: Set(invitation_id),
            kind: Set("code".to_owned()),
            secret_hash: Set(short_code_hash(&normalize_short_code(short_code)).to_vec()),
            created_at: Set(now_i64),
        }
        .insert(txn)
        .await?;
        Ok(())
    }

    async fn find_invitation_by_credential_locked(
        txn: &sea_orm::DatabaseTransaction,
        credential: &InvitationCredential,
    ) -> Result<Option<crate::migration::InvitationRow>, DbErr> {
        use crate::migration::invitation_credential;
        let Some(secret_hash) = credential.secret_hash() else {
            return Ok(None);
        };
        let Some(cred) = InvitationCredentials::find()
            .filter(invitation_credential::Column::SecretHash.eq(secret_hash.to_vec()))
            .one(txn)
            .await?
        else {
            return Ok(None);
        };
        Invitations::find_by_id(cred.invitation_id)
            .lock_exclusive()
            .one(txn)
            .await
    }

    async fn verify_dob_on_locked_row(
        txn: &sea_orm::DatabaseTransaction,
        row: &crate::migration::InvitationRow,
        birth_date: &str,
        now: u64,
    ) -> Result<DobCheckOutcome, InvitationCheckError> {
        let parsed = parse_birth_date(birth_date).ok_or(InvitationCheckError::InvalidBirthDate)?;
        let normalized_input = format_birth_date(parsed);
        let attempts = u8::try_from(row.wrong_dob_attempts.max(0)).unwrap_or(u8::MAX);
        if row.locked_at.is_some() || attempts >= MAX_DOB_ATTEMPTS {
            return Err(InvitationCheckError::Locked);
        }
        let Some(expected) = row.expected_birth_date.as_deref() else {
            return Err(InvitationCheckError::NotOpen);
        };
        if expected != normalized_input {
            let next_attempts = attempts.saturating_add(1);
            let mut active: crate::migration::InvitationActiveModel = row.clone().into();
            active.wrong_dob_attempts = Set(i32::from(next_attempts));
            let locked = next_attempts >= MAX_DOB_ATTEMPTS;
            if locked {
                active.locked_at = Set(Some(now as i64));
            }
            active.update(txn).await?;
            return Ok(DobCheckOutcome::Rejected(if locked {
                InvitationCheckError::Locked
            } else {
                InvitationCheckError::WrongBirthDate {
                    attempts_remaining: MAX_DOB_ATTEMPTS.saturating_sub(next_attempts),
                }
            }));
        }
        if !is_adult_on(parsed, now) {
            return Err(InvitationCheckError::Underage);
        }
        Ok(DobCheckOutcome::Matched(InvitationSharingPreview {
            practice_id: row.practice_id.clone(),
            practice_name: if row.practice_name.is_empty() {
                row.practice_id.clone()
            } else {
                row.practice_name.clone()
            },
            physician_name: row.physician_name.clone(),
            display_name: row.display_name.clone(),
            shares_summary: SHARES_SUMMARY.to_owned(),
        }))
    }

    async fn upsert_connected_row(
        txn: &sea_orm::DatabaseTransaction,
        account_id: uuid::Uuid,
        practice_id: &str,
        patient_id: &str,
        now: u64,
    ) -> Result<(), DbErr> {
        let existing_conn =
            Connections::find_by_id((account_id, practice_id.to_owned(), patient_id.to_owned()))
                .one(txn)
                .await?;
        match existing_conn {
            Some(row) => {
                let mut active: crate::migration::ConnectionActiveModel = row.into();
                active.status = Set("connected".to_owned());
                active.connected_at = Set(now as i64);
                active.ended_at = Set(None);
                active.end_reason = Set(None);
                active.updated_at = Set(now as i64);
                active.update(txn).await?;
            }
            None => {
                crate::migration::ConnectionActiveModel {
                    account_id: Set(account_id),
                    practice_id: Set(practice_id.to_owned()),
                    patient_id: Set(patient_id.to_owned()),
                    status: Set("connected".to_owned()),
                    connected_at: Set(now as i64),
                    ended_at: Set(None),
                    end_reason: Set(None),
                    created_at: Set(now as i64),
                    updated_at: Set(now as i64),
                }
                .insert(txn)
                .await?;
            }
        }
        Ok(())
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
        let issued = self
            .invite_patient(InvitePatientParams {
                patient: patient.clone(),
                email: email.to_owned(),
                display_name: display_name.to_owned(),
                practice_name: patient.practice_id.clone(),
                physician_name: String::new(),
                expected_birth_date: None,
                now,
            })
            .await?;
        Ok(issued.link_token)
    }

    async fn invite_patient(
        &self,
        params: InvitePatientParams,
    ) -> Result<IssuedInvitation, InviteError> {
        use crate::migration::{connection, invitation, invitation_credential};
        let key = params.email.trim().to_lowercase();
        if !key.contains('@') {
            return Err(InviteError::InvalidInput("email must be valid"));
        }
        let normalized_dob = match params.expected_birth_date.as_deref().map(str::trim) {
            Some("") | None => None,
            Some(raw) => {
                let parsed = parse_birth_date(raw).ok_or(InviteError::InvalidInput(
                    "expected_birth_date must be YYYY-MM-DD",
                ))?;
                Some(format_birth_date(parsed))
            }
        };

        let txn = self.db.begin().await?;
        let active_existing = Connections::find()
            .filter(connection::Column::PracticeId.eq(&params.patient.practice_id))
            .filter(connection::Column::PatientId.eq(&params.patient.patient_id))
            .filter(connection::Column::Status.eq("connected"))
            .one(&txn)
            .await?;
        if active_existing.is_some() {
            return Err(InviteError::Conflict);
        }

        let existing_inv = Invitations::find()
            .filter(invitation::Column::PracticeId.eq(&params.patient.practice_id))
            .filter(invitation::Column::PatientId.eq(&params.patient.patient_id))
            .lock_exclusive()
            .one(&txn)
            .await?;

        let link_token = mint_token();
        let short_code = mint_short_code();
        let row = match existing_inv {
            Some(existing) => {
                if existing.locked_at.is_some()
                    || existing.wrong_dob_attempts >= i32::from(MAX_DOB_ATTEMPTS)
                {
                    return Err(InviteError::Locked);
                }
                InvitationCredentials::delete_many()
                    .filter(invitation_credential::Column::InvitationId.eq(existing.id))
                    .exec(&txn)
                    .await?;
                let mut active: crate::migration::InvitationActiveModel = existing.into();
                active.email = Set(key);
                active.display_name = Set(params.display_name.trim().to_owned());
                active.token_hash = Set(token_hash(&link_token).to_vec());
                active.created_at = Set(params.now as i64);
                active.practice_name = Set(params.practice_name.trim().to_owned());
                active.physician_name = Set(params.physician_name.trim().to_owned());
                active.expected_birth_date = Set(normalized_dob);
                active.wrong_dob_attempts = Set(0);
                active.locked_at = Set(None);
                active.last_delivery_failed_at = Set(None);
                active.last_delivery_error = Set(None);
                active.updated_at = Set(params.now as i64);
                active.update(&txn).await?
            }
            None => {
                crate::migration::InvitationActiveModel {
                    id: Default::default(),
                    practice_id: Set(params.patient.practice_id.clone()),
                    patient_id: Set(params.patient.patient_id.clone()),
                    email: Set(key),
                    display_name: Set(params.display_name.trim().to_owned()),
                    token_hash: Set(token_hash(&link_token).to_vec()),
                    created_at: Set(params.now as i64),
                    practice_name: Set(params.practice_name.trim().to_owned()),
                    physician_name: Set(params.physician_name.trim().to_owned()),
                    expected_birth_date: Set(normalized_dob),
                    wrong_dob_attempts: Set(0),
                    locked_at: Set(None),
                    last_delivery_failed_at: Set(None),
                    last_delivery_error: Set(None),
                    updated_at: Set(params.now as i64),
                }
                .insert(&txn)
                .await?
            }
        };
        Self::insert_invitation_credentials(&txn, row.id, &link_token, &short_code, params.now)
            .await?;
        txn.commit().await?;
        Ok(IssuedInvitation {
            link_token,
            short_code,
            invitation: Self::invitation_row(row),
        })
    }

    async fn resend_invitation(
        &self,
        patient: &RosterPatient,
        now: u64,
    ) -> Result<IssuedInvitation, InviteError> {
        use crate::migration::{connection, invitation};
        let txn = self.db.begin().await?;
        let active_existing = Connections::find()
            .filter(connection::Column::PracticeId.eq(&patient.practice_id))
            .filter(connection::Column::PatientId.eq(&patient.patient_id))
            .filter(connection::Column::Status.eq("connected"))
            .one(&txn)
            .await?;
        if active_existing.is_some() {
            return Err(InviteError::Conflict);
        }
        let existing = Invitations::find()
            .filter(invitation::Column::PracticeId.eq(&patient.practice_id))
            .filter(invitation::Column::PatientId.eq(&patient.patient_id))
            .lock_exclusive()
            .one(&txn)
            .await?
            .ok_or(InviteError::NotFound)?;
        if existing.locked_at.is_some()
            || existing.wrong_dob_attempts >= i32::from(MAX_DOB_ATTEMPTS)
        {
            return Err(InviteError::Locked);
        }
        let link_token = mint_token();
        let short_code = mint_short_code();
        let mut active: crate::migration::InvitationActiveModel = existing.into();
        active.token_hash = Set(token_hash(&link_token).to_vec());
        active.last_delivery_failed_at = Set(None);
        active.last_delivery_error = Set(None);
        active.updated_at = Set(now as i64);
        let updated = active.update(&txn).await?;
        Self::insert_invitation_credentials(&txn, updated.id, &link_token, &short_code, now)
            .await?;
        txn.commit().await?;
        Ok(IssuedInvitation {
            link_token,
            short_code,
            invitation: Self::invitation_row(updated),
        })
    }

    async fn cancel_invitation(&self, patient: &RosterPatient) -> Result<(), InviteError> {
        use crate::migration::invitation;
        let res = Invitations::delete_many()
            .filter(invitation::Column::PracticeId.eq(&patient.practice_id))
            .filter(invitation::Column::PatientId.eq(&patient.patient_id))
            .exec(&self.db)
            .await?;
        if res.rows_affected == 0 {
            return Err(InviteError::NotFound);
        }
        Ok(())
    }

    async fn mark_invitation_delivery_failed(
        &self,
        patient: &RosterPatient,
        detail: &str,
        now: u64,
    ) -> Result<(), InviteError> {
        use crate::migration::invitation;
        let txn = self.db.begin().await?;
        let existing = Invitations::find()
            .filter(invitation::Column::PracticeId.eq(&patient.practice_id))
            .filter(invitation::Column::PatientId.eq(&patient.patient_id))
            .lock_exclusive()
            .one(&txn)
            .await?
            .ok_or(InviteError::NotFound)?;
        let mut active: crate::migration::InvitationActiveModel = existing.into();
        active.last_delivery_failed_at = Set(Some(now as i64));
        active.last_delivery_error = Set(Some(detail.trim().chars().take(200).collect()));
        active.update(&txn).await?;
        txn.commit().await?;
        Ok(())
    }

    async fn inspect_invitation(
        &self,
        credential: &InvitationCredential,
    ) -> Result<InvitationGateState, InvitationCheckError> {
        let txn = self.db.begin().await?;
        let row = Self::find_invitation_by_credential_locked(&txn, credential)
            .await?
            .ok_or(InvitationCheckError::NotOpen)?;
        let attempts = u8::try_from(row.wrong_dob_attempts.max(0)).unwrap_or(u8::MAX);
        let standing = if let Some(locked_at) = row.locked_at {
            InvitationOpenStanding::Locked {
                locked_at: locked_at.max(0) as u64,
            }
        } else if attempts >= MAX_DOB_ATTEMPTS {
            InvitationOpenStanding::Locked {
                locked_at: row.updated_at.max(0) as u64,
            }
        } else {
            InvitationOpenStanding::Active {
                wrong_dob_attempts: attempts,
                attempts_remaining: MAX_DOB_ATTEMPTS.saturating_sub(attempts),
            }
        };
        txn.commit().await?;
        Ok(InvitationGateState { standing })
    }

    async fn verify_invitation_dob(
        &self,
        credential: &InvitationCredential,
        birth_date: &str,
        now: u64,
    ) -> Result<InvitationSharingPreview, InvitationCheckError> {
        let txn = self.db.begin().await?;
        let row = Self::find_invitation_by_credential_locked(&txn, credential)
            .await?
            .ok_or(InvitationCheckError::NotOpen)?;
        let outcome = Self::verify_dob_on_locked_row(&txn, &row, birth_date, now).await?;
        txn.commit().await?;
        match outcome {
            DobCheckOutcome::Matched(preview) => Ok(preview),
            DobCheckOutcome::Rejected(err) => Err(err),
        }
    }

    async fn accept_invitation_share(
        &self,
        account_id: Option<uuid::Uuid>,
        account_email: Option<&str>,
        credential: &InvitationCredential,
        birth_date: &str,
        now: u64,
    ) -> Result<(uuid::Uuid, AccountConnection), InvitationAcceptError> {
        use crate::migration::account;
        let txn = self.db.begin().await?;
        let row = Self::find_invitation_by_credential_locked(&txn, credential)
            .await
            .map_err(InvitationAcceptError::Storage)?
            .ok_or(InvitationCheckError::NotOpen)?;
        match Self::verify_dob_on_locked_row(&txn, &row, birth_date, now).await? {
            DobCheckOutcome::Matched(_) => {}
            DobCheckOutcome::Rejected(err) => {
                txn.commit().await.map_err(InvitationAcceptError::Storage)?;
                return Err(err.into());
            }
        }

        let target_account_id = match account_id {
            Some(existing_id) => {
                if let Some(acct) = Accounts::find_by_id(existing_id)
                    .lock_exclusive()
                    .one(&txn)
                    .await
                    .map_err(InvitationAcceptError::Storage)?
                {
                    if !acct.active {
                        return Err(InvitationAcceptError::InactiveAccount);
                    }
                    acct.id
                } else {
                    let email_key = account_email
                        .map(|e| e.trim().to_lowercase())
                        .filter(|e| !e.is_empty())
                        .unwrap_or_else(|| row.email.clone());
                    crate::migration::AccountActiveModel {
                        id: Set(existing_id),
                        email: Set(email_key),
                        display_name: Set(row.display_name.clone()),
                        active: Set(true),
                        sessions_ended_before: Set(0),
                        created_at: Set(now as i64),
                    }
                    .insert(&txn)
                    .await
                    .map_err(InvitationAcceptError::Storage)?
                    .id
                }
            }
            None => {
                let email_key = account_email
                    .map(|e| e.trim().to_lowercase())
                    .filter(|e| !e.is_empty())
                    .ok_or(InvitationAcceptError::InactiveAccount)?;
                if let Some(existing) = Accounts::find()
                    .filter(account::Column::Email.eq(&email_key))
                    .lock_exclusive()
                    .one(&txn)
                    .await
                    .map_err(InvitationAcceptError::Storage)?
                {
                    if !existing.active {
                        return Err(InvitationAcceptError::InactiveAccount);
                    }
                    existing.id
                } else {
                    crate::migration::AccountActiveModel {
                        id: Set(uuid::Uuid::new_v4()),
                        email: Set(email_key),
                        display_name: Set(row.display_name.clone()),
                        active: Set(true),
                        sessions_ended_before: Set(0),
                        created_at: Set(now as i64),
                    }
                    .insert(&txn)
                    .await
                    .map_err(InvitationAcceptError::Storage)?
                    .id
                }
            }
        };

        let conflict_other = Connections::find()
            .filter(crate::migration::connection::Column::PracticeId.eq(&row.practice_id))
            .filter(crate::migration::connection::Column::PatientId.eq(&row.patient_id))
            .filter(crate::migration::connection::Column::Status.eq("connected"))
            .filter(crate::migration::connection::Column::AccountId.ne(target_account_id))
            .one(&txn)
            .await
            .map_err(InvitationAcceptError::Storage)?
            .is_some();
        if conflict_other {
            return Err(InvitationAcceptError::Conflict);
        }

        let deleted = Invitations::delete_by_id(row.id)
            .exec(&txn)
            .await
            .map_err(InvitationAcceptError::Storage)?;
        if deleted.rows_affected != 1 {
            return Err(InvitationCheckError::NotOpen.into());
        }

        Self::upsert_connected_row(
            &txn,
            target_account_id,
            &row.practice_id,
            &row.patient_id,
            now,
        )
        .await
        .map_err(InvitationAcceptError::Storage)?;
        txn.commit().await.map_err(InvitationAcceptError::Storage)?;

        Ok((
            target_account_id,
            AccountConnection {
                patient: RosterPatient {
                    practice_id: row.practice_id,
                    patient_id: row.patient_id,
                },
                standing: ConnectionStanding::Connected { connected_at: now },
            },
        ))
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
        use crate::migration::{account, connection};
        let key = email.to_lowercase();
        let txn = self.db.begin().await.map_err(|_| SignInError::Invalid)?;

        let invitation = Self::find_invitation_by_credential_locked(
            &txn,
            &InvitationCredential::LinkToken(token.to_owned()),
        )
        .await
        .map_err(|_| SignInError::Invalid)?
        .ok_or(SignInError::Invalid)?;
        if invitation.email != key
            || invitation.locked_at.is_some()
            || invitation.expected_birth_date.is_some()
        {
            return Err(SignInError::Invalid);
        }

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

        Self::upsert_connected_row(
            &txn,
            account_id,
            &invitation.practice_id,
            &invitation.patient_id,
            now,
        )
        .await
        .map_err(|_| SignInError::Invalid)?;

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
            .await?
            .ok_or(LinkError::Unlinked)?;
        let hydrated = Self::load_account_with_connections(&self.db, account).await?;
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
        let txn = self.db.begin().await?;
        let existing = Connections::find_by_id((
            account_id,
            patient.practice_id.clone(),
            patient.patient_id.clone(),
        ))
        .lock_exclusive()
        .one(&txn)
        .await?
        .ok_or(LinkError::Unlinked)?;
        if existing.status != "connected" {
            return Err(LinkError::Unlinked);
        }
        Self::end_connection_row(&txn, existing, reason, now).await?;
        txn.commit().await?;
        Ok(())
    }

    async fn leave_connection(
        &self,
        account_id: uuid::Uuid,
        practice_id: &str,
        now: u64,
    ) -> Result<Vec<AccountConnection>, LinkError> {
        use crate::migration::connection::Column;
        let txn = self.db.begin().await?;
        let rows = Connections::find()
            .filter(Column::AccountId.eq(account_id))
            .filter(Column::PracticeId.eq(practice_id))
            .filter(Column::Status.eq("connected"))
            .order_by_asc(Column::PatientId)
            .lock_exclusive()
            .all(&txn)
            .await?;
        if rows.is_empty() {
            return Err(LinkError::Unlinked);
        }
        let mut ended = Vec::with_capacity(rows.len());
        for row in rows {
            ended.push(
                Self::end_connection_row(&txn, row, ConnectionEndReason::PatientLeft, now).await?,
            );
        }
        txn.commit().await?;
        Ok(ended)
    }

    async fn discharge_connection(
        &self,
        patient: &RosterPatient,
        now: u64,
    ) -> Result<AccountConnection, LinkError> {
        use crate::migration::connection::Column;
        let txn = self.db.begin().await?;
        let existing = Connections::find()
            .filter(Column::PracticeId.eq(&patient.practice_id))
            .filter(Column::PatientId.eq(&patient.patient_id))
            .filter(Column::Status.eq("connected"))
            .lock_exclusive()
            .one(&txn)
            .await?
            .ok_or(LinkError::Unlinked)?;
        let ended =
            Self::end_connection_row(&txn, existing, ConnectionEndReason::PracticeDischarged, now)
                .await?;
        txn.commit().await?;
        Ok(ended)
    }

    async fn end_all_connections_for_account(
        &self,
        account_id: uuid::Uuid,
        now: u64,
    ) -> Result<usize, ForMyDoctorError> {
        use crate::migration::{account, connection, session, sign_in_token};
        let txn = self.db.begin().await?;
        let rows = Connections::find()
            .filter(connection::Column::AccountId.eq(account_id))
            .filter(connection::Column::Status.eq("connected"))
            .lock_exclusive()
            .all(&txn)
            .await?;
        let count = rows.len();
        for row in rows {
            Self::end_connection_row(&txn, row, ConnectionEndReason::AccountDeleted, now).await?;
        }
        if let Some(acct) = Accounts::find_by_id(account_id)
            .lock_exclusive()
            .one(&txn)
            .await?
        {
            let cutoff = acct.sessions_ended_before.max(now as i64);
            let mut active_acct: crate::migration::AccountActiveModel = acct.into();
            active_acct.active = Set(false);
            active_acct.sessions_ended_before = Set(cutoff);
            active_acct.update(&txn).await?;
        } else {
            let _ = account::Column::Id;
        }
        Sessions::delete_many()
            .filter(session::Column::AccountId.eq(account_id))
            .exec(&txn)
            .await?;
        SignInTokens::delete_many()
            .filter(sign_in_token::Column::AccountId.eq(account_id))
            .exec(&txn)
            .await?;
        txn.commit().await?;
        Ok(count)
    }

    async fn roster_connection(
        &self,
        patient: &RosterPatient,
    ) -> Result<RosterConnectionStatus, ForMyDoctorError> {
        use crate::migration::{connection, invitation};
        let conn_rows = Connections::find()
            .filter(connection::Column::PracticeId.eq(&patient.practice_id))
            .filter(connection::Column::PatientId.eq(&patient.patient_id))
            .all(&self.db)
            .await?;
        let standings: Vec<(uuid::Uuid, ConnectionStanding)> = conn_rows
            .iter()
            .filter_map(|row| Some((row.account_id, Self::connection_standing(row)?)))
            .collect();
        let inv = Invitations::find()
            .filter(invitation::Column::PracticeId.eq(&patient.practice_id))
            .filter(invitation::Column::PatientId.eq(&patient.patient_id))
            .one(&self.db)
            .await?
            .map(Self::invitation_row);
        Ok(resolve_roster_status(
            standings.iter().map(|(aid, st)| (*aid, st)),
            inv.as_ref(),
        ))
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
