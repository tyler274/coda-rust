//! The Postgres `AuthStore` through SeaORM.
//!
//! Same contract as the memory store; durable truth lives in the
//! `physician_*` tables the crate's migration chain owns.

use std::time::Duration;

use crate::migration::{
    Accounts, Invitations, Memberships, PatientLinks, Patients, Practices, Sessions, SignInTokens,
};
use crate::{
    Account, AuthStore, INVITATION_TTL_SECONDS, Membership, PanelError, PatientLink,
    PhysicianIdentity, PracticeClock, SIGN_IN_TOKEN_TTL_SECONDS, Session, SessionActivity,
    SessionError, SessionLifetime, SessionRenewal, SignInError, StoreUnavailable,
    mint_patient_token, mint_token, token_hash,
};
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectOptions, ConnectionTrait, Database, DatabaseConnection,
    DbErr, EntityTrait, QueryFilter, QuerySelect, QueryTrait, Set, TransactionTrait,
    sea_query::{Expr, JoinType, LockType, Query},
};

fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

async fn register_patient_on(
    db: &impl ConnectionTrait,
    practice_id: &str,
    workspace: &str,
) -> Result<(), DbErr> {
    use crate::migration::patient::Column;
    Patients::insert(crate::migration::PatientActiveModel {
        patient_workspace_id: Set(workspace.to_owned()),
        practice_id: Set(practice_id.to_owned()),
        created_at: Set(unix_now()),
    })
    .on_conflict(
        sea_orm::sea_query::OnConflict::column(Column::PatientWorkspaceId)
            .do_nothing()
            .to_owned(),
    )
    .try_insert()
    .exec(db)
    .await?;
    Ok(())
}

/// The SQL-backed store.
#[derive(Debug, Clone)]
pub struct PostgresAuthStore {
    db: DatabaseConnection,
}

impl PostgresAuthStore {
    /// Connects without applying migrations; retained for existing callers.
    pub async fn connect(database_url: &str) -> Result<Self, DbErr> {
        Self::connect_without_migrations(database_url).await
    }

    /// Opens a bounded runtime pool without creating or upgrading any schema.
    ///
    /// Run `physician-auth-migrate` with the migration identity first. This
    /// constructor only establishes connectivity: callers must separately
    /// check readiness before accepting traffic. Missing or outdated tables
    /// are not repaired, and runtime credentials need no schema-owner rights.
    pub async fn connect_without_migrations(database_url: &str) -> Result<Self, DbErr> {
        let mut options = ConnectOptions::new(database_url.to_owned());
        options
            .max_connections(8)
            .connect_timeout(Duration::from_secs(10));
        let db = Database::connect(options).await?;
        Ok(Self::from_connection(db))
    }

    /// Uses an existing pool without applying migrations or changing its limits.
    pub fn from_connection(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// The runtime pool, for the authority change listener.
    pub fn pool(&self) -> &sea_orm::sqlx::PgPool {
        self.db.get_postgres_connection_pool()
    }

    /// Checks all owned tables without installing or updating a migration ledger.
    pub async fn check_schema(&self) -> Result<(), DbErr> {
        Accounts::find().one(&self.db).await?;
        Invitations::find().one(&self.db).await?;
        Memberships::find().one(&self.db).await?;
        PatientLinks::find().one(&self.db).await?;
        Patients::find().one(&self.db).await?;
        Sessions::find().one(&self.db).await?;
        SignInTokens::find().one(&self.db).await?;
        Ok(())
    }

    /// Connects and applies the crate's migration chain.
    pub async fn connect_with_migrations(database_url: &str) -> Result<Self, DbErr> {
        let store = Self::connect(database_url).await?;
        crate::migration::migrate(&store.db).await?;
        Ok(store)
    }

    /// Applies the migration chain on an existing connection.
    pub async fn migrate(&self) -> Result<(), DbErr> {
        crate::migration::migrate(&self.db).await
    }

    /// Creates one account when its email is absent; returns whether it did.
    ///
    /// An existing account is left exactly as it is, so a deactivation or
    /// removal-and-reinvite an administrator made is never undone by a rerun.
    /// The staging migration job uses this to seed its preview accounts.
    pub async fn ensure_account(
        &self,
        practice_id: &str,
        physician_id: &str,
        email: &str,
        display_name: &str,
        admin: bool,
        now: u64,
    ) -> Result<bool, DbErr> {
        let key = email.trim().to_lowercase();
        let account = crate::migration::AccountActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            practice_id: Set(practice_id.to_owned()),
            physician_id: Set(physician_id.to_owned()),
            email: Set(key),
            display_name: Set(display_name.to_owned()),
            active: Set(true),
            admin: Set(admin),
            sessions_ended_before: Set(0),
            created_at: Set(now as i64),
        };
        let inserted = Accounts::insert(account)
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(crate::migration::account::Column::Email)
                    .do_nothing()
                    .to_owned(),
            )
            .try_insert()
            .exec(&self.db)
            .await?;
        Ok(matches!(inserted, sea_orm::TryInsertResult::Inserted(_)))
    }

    /// Non-secret reference to the existing session, usable for durable accepted work.
    pub fn session_reference(token: &str) -> Vec<u8> {
        token_hash(token).to_vec()
    }

    /// Checks clinical workspace authority on the caller's publication transaction.
    /// Row SHARE locks conflict with session deletion, account deactivation/removal,
    /// panel removal and patient reassignment. Keep this transaction open through
    /// publication. The hash is a reference to an existing session, not a new grant.
    pub async fn lock_runtime_authority(
        tx: &sea_orm::DatabaseTransaction,
        session_hash: &[u8],
        identity: &PhysicianIdentity,
        workspace: &str,
    ) -> Result<(), DbErr> {
        Self::runtime_authority(tx, session_hash, identity, workspace, Some(LockType::Share)).await
    }

    /// Read current runtime authority without taking locks or changing session state.
    pub async fn check_runtime_authority(
        tx: &sea_orm::DatabaseTransaction,
        session_hash: &[u8],
        identity: &PhysicianIdentity,
        workspace: &str,
    ) -> Result<(), DbErr> {
        Self::runtime_authority(tx, session_hash, identity, workspace, None).await
    }

    async fn runtime_authority(
        tx: &sea_orm::DatabaseTransaction,
        session_hash: &[u8],
        identity: &PhysicianIdentity,
        workspace: &str,
        lock: Option<LockType>,
    ) -> Result<(), DbErr> {
        let expires = authority_query(session_hash, identity, workspace, lock)
            .into_tuple::<i64>()
            .one(tx)
            .await?
            .ok_or_else(|| DbErr::Custom("runtime authority revoked".into()))?;
        // PostgreSQL-specific wall-clock expression: CURRENT_TIMESTAMP is fixed at
        // transaction start and would admit a session that expired while waiting
        // for locks. Read it in a separate statement AFTER authority locks arrive.
        let now = tx
            .query_one(
                &Query::select()
                    .expr_as(
                        Expr::cust("floor(extract(epoch FROM clock_timestamp()))::bigint"),
                        "now",
                    )
                    .to_owned(),
            )
            .await?
            .ok_or_else(|| DbErr::Custom("missing database clock".into()))?;
        if expires <= now.try_get::<i64>("", "now")? {
            return Err(DbErr::Custom("runtime session expired".into()));
        }
        Ok(())
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

/// One authority query, optionally locking participating rows for publication.
/// Do not split these joins into independent lookups: revocation/reassignment
/// must conflict with the same four-row fence held through publication.
fn authority_query(
    session_hash: &[u8],
    identity: &PhysicianIdentity,
    workspace: &str,
    lock: Option<LockType>,
) -> sea_orm::Select<Sessions> {
    use crate::migration::{account, membership, patient, session};
    use sea_orm::sea_query::ExprTrait;

    let mut query = Sessions::find()
        .select_only()
        .column(session::Column::ExpiresAt)
        .filter(session::Column::TokenHash.eq(session_hash.to_vec()))
        .filter(account::Column::PracticeId.eq(&identity.practice_id))
        .filter(account::Column::PhysicianId.eq(&identity.physician_id))
        .filter(account::Column::Active.eq(true))
        .filter(membership::Column::PatientWorkspaceId.eq(workspace))
        .filter(membership::Column::Allowed.eq(true))
        .filter(
            Expr::col((Sessions, session::Column::CreatedAt))
                .gte(Expr::col((Accounts, account::Column::SessionsEndedBefore))),
        );
    QueryTrait::query(&mut query)
        .join(
            JoinType::InnerJoin,
            Accounts,
            Expr::col((Accounts, account::Column::Id))
                .eq(Expr::col((Sessions, session::Column::AccountId))),
        )
        .join(
            JoinType::InnerJoin,
            Memberships,
            Expr::col((Memberships, membership::Column::PracticeId))
                .eq(Expr::col((Accounts, account::Column::PracticeId)))
                .and(
                    Expr::col((Memberships, membership::Column::PhysicianId))
                        .eq(Expr::col((Accounts, account::Column::PhysicianId))),
                ),
        )
        .join(
            JoinType::InnerJoin,
            Patients,
            Expr::col((Patients, patient::Column::PatientWorkspaceId))
                .eq(Expr::col((
                    Memberships,
                    membership::Column::PatientWorkspaceId,
                )))
                .and(
                    Expr::col((Patients, patient::Column::PracticeId))
                        .eq(Expr::col((Accounts, account::Column::PracticeId))),
                ),
        );
    // Publication locks exactly the session, account, membership and patient rows.
    // A currency preflight uses the same authority query without a lock.
    if let Some(lock) = lock {
        QueryTrait::query(&mut query).lock(lock);
    }
    query
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
        // Only the transaction that consumes the token may mint a session.
        // A concurrent verifier can have read the same pending row already.
        let consumed = SignInTokens::delete_by_id(pending.id)
            .exec(&txn)
            .await
            .map_err(stale)?;
        if consumed.rows_affected != 1 {
            return Err(SignInError::Invalid);
        }
        let session_token = mint_token();
        let session = crate::migration::SessionActiveModel {
            id: Default::default(),
            account_id: Set(account.id),
            token_hash: Set(token_hash(&session_token).to_vec()),
            created_at: Set(now as i64),
            expires_at: Set(SessionLifetime::mint(now).expires_at() as i64),
            ends_at: Set(None),
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

    async fn mint_fixed_session(
        &self,
        identity: &PhysicianIdentity,
        now: u64,
        ends_at: u64,
    ) -> Result<String, SignInError> {
        let txn = self.db.begin().await.map_err(stale)?;
        let account = Accounts::find()
            .filter(crate::migration::account::Column::PracticeId.eq(&identity.practice_id))
            .filter(crate::migration::account::Column::PhysicianId.eq(&identity.physician_id))
            .lock(LockType::Share)
            .one(&txn)
            .await
            .map_err(stale)?
            .ok_or(SignInError::Invalid)?;
        if !account.active {
            return Err(SignInError::Inactive);
        }
        let lifetime = SessionLifetime::fixed(now, ends_at);
        let session_token = mint_token();
        let session = crate::migration::SessionActiveModel {
            id: Default::default(),
            account_id: Set(account.id),
            token_hash: Set(token_hash(&session_token).to_vec()),
            created_at: Set(now as i64),
            expires_at: Set(lifetime.expires_at() as i64),
            ends_at: Set(Some(lifetime.expires_at() as i64)),
        };
        session.insert(&txn).await.map_err(stale)?;
        txn.commit().await.map_err(stale)?;
        Ok(session_token)
    }

    async fn verified_session(&self, token: &str, now: u64) -> Result<Session, SessionError> {
        let hash = token_hash(token);
        let session = Sessions::find()
            .filter(crate::migration::session::Column::TokenHash.eq(hash.to_vec()))
            .one(&self.db)
            .await
            .map_err(invalid)?
            .ok_or(SessionError::Invalid)?;
        let created_at = session.created_at as u64;
        let lifetime = SessionLifetime::from_persisted(
            created_at,
            session.expires_at as u64,
            session.ends_at.map(|end| end as u64),
        );
        if !lifetime.allows(now) {
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
        Ok(Session {
            identity: PhysicianIdentity {
                practice_id: account.practice_id,
                physician_id: account.physician_id,
            },
            token_hash: hash,
            lifetime,
            created_at,
        })
    }

    async fn note_physician_activity(
        &self,
        token: &str,
        now: u64,
    ) -> Result<SessionActivity, StoreUnavailable> {
        let hash = token_hash(token);
        let row = Sessions::find()
            .filter(crate::migration::session::Column::TokenHash.eq(hash.to_vec()))
            .one(&self.db)
            .await
            .map_err(|_| StoreUnavailable)?;
        let Some(row) = row else {
            return Ok(SessionActivity::Ended);
        };
        let lifetime = SessionLifetime::from_persisted(
            row.created_at as u64,
            row.expires_at as u64,
            row.ends_at.map(|end| end as u64),
        );
        match lifetime.renew(now) {
            SessionRenewal::Ended => Ok(SessionActivity::Ended),
            SessionRenewal::Covered => Ok(SessionActivity::Covered),
            SessionRenewal::Extend { idle_deadline } => {
                let updated = Sessions::update_many()
                    .col_expr(
                        crate::migration::session::Column::ExpiresAt,
                        Expr::value(idle_deadline as i64),
                    )
                    .filter(crate::migration::session::Column::TokenHash.eq(hash.to_vec()))
                    .filter(crate::migration::session::Column::ExpiresAt.gt(now as i64))
                    .filter(crate::migration::session::Column::ExpiresAt.lt(idle_deadline as i64))
                    .exec(&self.db)
                    .await
                    .map_err(|_| StoreUnavailable)?;
                Ok(if updated.rows_affected == 0 {
                    SessionActivity::Covered
                } else {
                    SessionActivity::Extended
                })
            }
        }
    }

    async fn publish_authorized(
        &self,
        token: &str,
        identity: &PhysicianIdentity,
        workspace: &str,
        now: u64,
        publish: &mut (dyn FnMut() -> Result<(), String> + Send),
    ) -> Result<(), String> {
        let txn = self.db.begin().await.map_err(|e| e.to_string())?;
        let row = authority_query(
            &token_hash(token),
            identity,
            workspace,
            Some(LockType::Update),
        )
        .filter(crate::migration::session::Column::ExpiresAt.gt(now as i64))
        .into_tuple::<i64>()
        .one(&txn)
        .await
        .map_err(|e| e.to_string())?;
        if row.is_none() {
            return Err("physician authority changed".into());
        }
        // All direct SQL deletes/updates conflict with these authority row locks.
        publish()?;
        txn.commit().await.map_err(|e| e.to_string())
    }
    async fn end_session(&self, token: &str) -> Result<(), StoreUnavailable> {
        Sessions::delete_many()
            .filter(crate::migration::session::Column::TokenHash.eq(token_hash(token).to_vec()))
            .exec(&self.db)
            .await
            .map_err(|_| StoreUnavailable)?;
        Ok(())
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
            .filter(crate::migration::account::Column::PracticeId.eq(&identity.practice_id))
            .filter(crate::migration::account::Column::PhysicianId.eq(&identity.physician_id))
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
        let Ok(owner) = self.patient_practice(patient_workspace_id).await else {
            return Membership::Denied;
        };
        if owner
            .as_deref()
            .is_some_and(|owner| owner != identity.practice_id)
        {
            return Membership::Denied;
        }
        let panel_allowed = Memberships::find()
            .filter(crate::migration::membership::Column::PracticeId.eq(&identity.practice_id))
            .filter(crate::migration::membership::Column::PhysicianId.eq(&identity.physician_id))
            .filter(
                crate::migration::membership::Column::PatientWorkspaceId.eq(patient_workspace_id),
            )
            .one(&self.db)
            .await
            .ok()
            .flatten()
            .is_some_and(|row| row.allowed);
        if panel_allowed {
            return Membership::Allowed;
        }
        if owner.as_deref() == Some(identity.practice_id.as_str()) {
            return Membership::ClinicDemographic;
        }
        Membership::Denied
    }

    async fn set_membership(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
        membership: Membership,
    ) -> Result<(), StoreUnavailable> {
        use crate::migration::membership::Column;
        let txn = self.db.begin().await.map_err(|_| StoreUnavailable)?;
        if membership == Membership::ClinicDemographic {
            register_patient_on(&txn, &identity.practice_id, patient_workspace_id)
                .await
                .map_err(|_| StoreUnavailable)?;
        }
        Memberships::insert(crate::migration::MembershipActiveModel {
            id: Default::default(),
            practice_id: Set(identity.practice_id.clone()),
            physician_id: Set(identity.physician_id.clone()),
            patient_workspace_id: Set(patient_workspace_id.to_owned()),
            allowed: Set(membership == Membership::Allowed),
            created_at: Set(unix_now()),
        })
        .on_conflict(
            sea_orm::sea_query::OnConflict::columns([
                Column::PracticeId,
                Column::PhysicianId,
                Column::PatientWorkspaceId,
            ])
            .update_column(Column::Allowed)
            .to_owned(),
        )
        .exec_without_returning(&txn)
        .await
        .map_err(|_| StoreUnavailable)?;
        txn.commit().await.map_err(|_| StoreUnavailable)
    }

    async fn place_on_panel(
        &self,
        identity: &PhysicianIdentity,
        patient_workspace_id: &str,
    ) -> Result<(), StoreUnavailable> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|since| since.as_secs())
            .unwrap_or(0) as i64;
        let place = crate::migration::MembershipActiveModel {
            id: Default::default(),
            practice_id: Set(identity.practice_id.clone()),
            physician_id: Set(identity.physician_id.clone()),
            patient_workspace_id: Set(patient_workspace_id.to_owned()),
            allowed: Set(true),
            created_at: Set(now),
        };
        Memberships::insert(place)
            .on_conflict(
                sea_orm::sea_query::OnConflict::columns([
                    crate::migration::membership::Column::PracticeId,
                    crate::migration::membership::Column::PhysicianId,
                    crate::migration::membership::Column::PatientWorkspaceId,
                ])
                .do_nothing()
                .to_owned(),
            )
            .try_insert()
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(|_| StoreUnavailable)
    }

    async fn panel(&self, identity: &PhysicianIdentity) -> Vec<String> {
        use crate::migration::{membership, patient};
        use sea_orm::sea_query::ExprTrait;

        // A workspace registered to another practice never joins this panel.
        let foreign = Query::select()
            .column(patient::Column::PatientWorkspaceId)
            .from(Patients)
            .and_where(Expr::col((Patients, patient::Column::PracticeId)).ne(&identity.practice_id))
            .to_owned();
        let mut workspaces: Vec<String> = Memberships::find()
            .filter(membership::Column::PracticeId.eq(&identity.practice_id))
            .filter(membership::Column::PhysicianId.eq(&identity.physician_id))
            .filter(membership::Column::Allowed.eq(true))
            .filter(membership::Column::PatientWorkspaceId.not_in_subquery(foreign))
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
        let owner = self
            .patient_practice(patient_workspace_id)
            .await
            .map_err(|_| PanelError::Unavailable)?;
        if owner.is_some_and(|owner| owner != actor.practice_id) {
            return Err(PanelError::UnknownTarget);
        }
        self.set_membership(&target, patient_workspace_id, membership)
            .await
            .map_err(|_| PanelError::Unavailable)
    }

    async fn register_patient(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
    ) -> Result<(), StoreUnavailable> {
        register_patient_on(&self.db, practice_id, patient_workspace_id)
            .await
            .map_err(|_| StoreUnavailable)
    }

    async fn ensure_practice(
        &self,
        practice_id: &str,
        clock: PracticeClock,
    ) -> Result<(), StoreUnavailable> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|since| since.as_secs())
            .unwrap_or(0) as i64;
        let fixed_time = match clock {
            PracticeClock::RealTime => None,
            PracticeClock::Fixed(instant) => Some(instant as i64),
        };
        let practice = crate::migration::PracticeActiveModel {
            practice_id: Set(practice_id.to_owned()),
            fixed_time: Set(fixed_time),
            created_at: Set(now),
        };
        Practices::insert(practice)
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(
                    crate::migration::practice::Column::PracticeId,
                )
                .do_nothing()
                .to_owned(),
            )
            .try_insert()
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(|_| StoreUnavailable)
    }

    async fn practice_clock(&self, practice_id: &str) -> Result<PracticeClock, StoreUnavailable> {
        let practice = Practices::find_by_id(practice_id.to_owned())
            .one(&self.db)
            .await
            .map_err(|_| StoreUnavailable)?;
        Ok(match practice.and_then(|row| row.fixed_time) {
            Some(instant) => PracticeClock::Fixed(instant as u64),
            None => PracticeClock::RealTime,
        })
    }

    async fn patient_practice(
        &self,
        patient_workspace_id: &str,
    ) -> Result<Option<String>, StoreUnavailable> {
        Patients::find_by_id(patient_workspace_id.to_owned())
            .one(&self.db)
            .await
            .map(|row| row.map(|row| row.practice_id))
            .map_err(|_| StoreUnavailable)
    }

    async fn practice_patients(&self, practice_id: &str) -> Vec<String> {
        let mut workspaces: Vec<String> = Patients::find()
            .filter(crate::migration::patient::Column::PracticeId.eq(practice_id))
            .all(&self.db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|row| row.patient_workspace_id)
            .collect();
        workspaces.sort();
        workspaces
    }

    async fn practice_accounts(&self, practice_id: &str) -> Result<Vec<Account>, StoreUnavailable> {
        let mut accounts: Vec<Account> = Accounts::find()
            .filter(crate::migration::account::Column::PracticeId.eq(practice_id))
            .all(&self.db)
            .await
            .map_err(|_| StoreUnavailable)?
            .into_iter()
            .map(Self::account_row)
            .collect();
        accounts.sort();
        Ok(accounts)
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
            expires_at: Set(SessionLifetime::mint(now).expires_at() as i64),
            ends_at: Set(None),
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

    async fn invite_patient(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        now: u64,
    ) -> Result<String, SignInError> {
        let practice_id = practice_id.trim();
        let patient_workspace_id = patient_workspace_id.trim();
        if practice_id.is_empty() || patient_workspace_id.is_empty() {
            return Err(SignInError::Invalid);
        }
        let token = mint_patient_token();
        let hash = token_hash(&token).to_vec();
        let expires_at = (now + INVITATION_TTL_SECONDS) as i64;
        let existing = PatientLinks::find()
            .filter(crate::migration::patient_link::Column::PracticeId.eq(practice_id))
            .filter(
                crate::migration::patient_link::Column::PatientWorkspaceId.eq(patient_workspace_id),
            )
            .one(&self.db)
            .await
            .map_err(stale)?;
        match existing {
            Some(row) => {
                let mut update: crate::migration::PatientLinkActiveModel = row.into();
                update.pending_token_hash = Set(Some(hash));
                update.pending_expires_at = Set(Some(expires_at));
                update.update(&self.db).await.map_err(stale)?;
            }
            None => {
                crate::migration::PatientLinkActiveModel {
                    id: Default::default(),
                    practice_id: Set(practice_id.to_owned()),
                    patient_workspace_id: Set(patient_workspace_id.to_owned()),
                    pending_token_hash: Set(Some(hash)),
                    pending_expires_at: Set(Some(expires_at)),
                    duet_account_id: Set(None),
                    linked: Set(false),
                    sharing_consented: Set(false),
                    consent_version: Set(None),
                    consented_at: Set(None),
                    revoked_at: Set(None),
                    created_at: Set(now as i64),
                }
                .insert(&self.db)
                .await
                .map_err(stale)?;
            }
        }
        Ok(token)
    }

    async fn accept_patient_invitation(
        &self,
        token: &str,
        duet_account_id: &str,
        now: u64,
    ) -> Result<PatientLink, SignInError> {
        let duet_account_id = duet_account_id.trim();
        if duet_account_id.is_empty() {
            return Err(SignInError::Invalid);
        }
        let hash = token_hash(token).to_vec();
        let txn = self.db.begin().await.map_err(stale)?;
        let row = PatientLinks::find()
            .filter(crate::migration::patient_link::Column::PendingTokenHash.eq(hash))
            .lock_exclusive()
            .one(&txn)
            .await
            .map_err(stale)?
            .ok_or(SignInError::Invalid)?;
        let expires_at = row.pending_expires_at.unwrap_or(0);
        let changed_account = row.duet_account_id.as_deref() != Some(duet_account_id);
        let mut update: crate::migration::PatientLinkActiveModel = row.into();
        update.pending_token_hash = Set(None);
        update.pending_expires_at = Set(None);
        if expires_at <= now as i64 {
            update.update(&txn).await.map_err(stale)?;
            txn.commit().await.map_err(stale)?;
            return Err(SignInError::Invalid);
        }
        if changed_account {
            update.sharing_consented = Set(false);
            update.consent_version = Set(None);
            update.consented_at = Set(None);
            update.revoked_at = Set(None);
        }
        update.duet_account_id = Set(Some(duet_account_id.to_owned()));
        update.linked = Set(true);
        let updated = update.update(&txn).await.map_err(stale)?;
        txn.commit().await.map_err(stale)?;
        Ok(patient_link_from_row(updated))
    }

    async fn grant_patient_consent(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        duet_account_id: &str,
        consent_version: &str,
        now: u64,
    ) -> Result<PatientLink, SignInError> {
        let consent_version = consent_version.trim();
        let duet_account_id = duet_account_id.trim();
        if consent_version.is_empty() || duet_account_id.is_empty() {
            return Err(SignInError::Invalid);
        }
        let txn = self.db.begin().await.map_err(stale)?;
        let row = PatientLinks::find()
            .filter(crate::migration::patient_link::Column::PracticeId.eq(practice_id))
            .filter(
                crate::migration::patient_link::Column::PatientWorkspaceId.eq(patient_workspace_id),
            )
            .lock_exclusive()
            .one(&txn)
            .await
            .map_err(stale)?
            .ok_or(SignInError::Invalid)?;
        if !row.linked || row.duet_account_id.as_deref() != Some(duet_account_id) {
            return Err(SignInError::Invalid);
        }
        if row.revoked_at.is_some() {
            return Err(SignInError::Invalid);
        }
        let mut update: crate::migration::PatientLinkActiveModel = row.into();
        update.sharing_consented = Set(true);
        update.consent_version = Set(Some(consent_version.to_owned()));
        update.consented_at = Set(Some(now as i64));
        let updated = update.update(&txn).await.map_err(stale)?;
        txn.commit().await.map_err(stale)?;
        Ok(patient_link_from_row(updated))
    }

    async fn revoke_patient_consent(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
        now: u64,
    ) -> Result<PatientLink, SignInError> {
        let row = PatientLinks::find()
            .filter(crate::migration::patient_link::Column::PracticeId.eq(practice_id))
            .filter(
                crate::migration::patient_link::Column::PatientWorkspaceId.eq(patient_workspace_id),
            )
            .one(&self.db)
            .await
            .map_err(stale)?
            .ok_or(SignInError::Invalid)?;
        if !row.linked {
            return Err(SignInError::Invalid);
        }
        let mut update: crate::migration::PatientLinkActiveModel = row.into();
        update.sharing_consented = Set(false);
        update.revoked_at = Set(Some(now as i64));
        let updated = update.update(&self.db).await.map_err(stale)?;
        Ok(patient_link_from_row(updated))
    }

    async fn patient_link(
        &self,
        practice_id: &str,
        patient_workspace_id: &str,
    ) -> Option<PatientLink> {
        PatientLinks::find()
            .filter(crate::migration::patient_link::Column::PracticeId.eq(practice_id))
            .filter(
                crate::migration::patient_link::Column::PatientWorkspaceId.eq(patient_workspace_id),
            )
            .one(&self.db)
            .await
            .ok()
            .flatten()
            .map(patient_link_from_row)
    }
}

fn patient_link_from_row(row: crate::migration::PatientLinkRow) -> PatientLink {
    PatientLink {
        practice_id: row.practice_id,
        patient_workspace_id: row.patient_workspace_id,
        duet_account_id: row.duet_account_id,
        linked: row.linked,
        sharing_consented: row.sharing_consented,
        consent_version: row.consent_version,
        consented_at: row.consented_at.map(|ts| ts.max(0) as u64),
        revoked_at: row.revoked_at.map(|ts| ts.max(0) as u64),
    }
}

/// Reads the pending token and its account; the caller must atomically consume it.
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

#[cfg(test)]
mod query_tests {
    use super::*;
    use sea_orm::DbBackend;

    #[test]
    fn authority_query_binds_values_and_locks_the_joined_owners() {
        let identity = PhysicianIdentity {
            practice_id: "practice' OR true --".into(),
            physician_id: "physician".into(),
        };
        for (lock, suffix) in [
            (LockType::Share, "FOR SHARE"),
            (LockType::Update, "FOR UPDATE"),
        ] {
            let statement = authority_query(b"session-hash", &identity, "workspace", Some(lock))
                .build(DbBackend::Postgres);
            assert_eq!(
                statement.sql,
                format!(
                    "SELECT \"physician_sessions\".\"expires_at\" FROM \"physician_sessions\" \
                     INNER JOIN \"physician_accounts\" ON \"physician_accounts\".\"id\" = \"physician_sessions\".\"account_id\" \
                     INNER JOIN \"physician_memberships\" ON \"physician_memberships\".\"practice_id\" = \"physician_accounts\".\"practice_id\" AND \"physician_memberships\".\"physician_id\" = \"physician_accounts\".\"physician_id\" \
                     INNER JOIN \"physician_patients\" ON \"physician_patients\".\"patient_workspace_id\" = \"physician_memberships\".\"patient_workspace_id\" AND \"physician_patients\".\"practice_id\" = \"physician_accounts\".\"practice_id\" \
                     WHERE \"physician_sessions\".\"token_hash\" = $1 AND \"physician_accounts\".\"practice_id\" = $2 AND \"physician_accounts\".\"physician_id\" = $3 AND \"physician_accounts\".\"active\" = $4 AND \"physician_memberships\".\"patient_workspace_id\" = $5 AND \"physician_memberships\".\"allowed\" = $6 AND \"physician_sessions\".\"created_at\" >= \"physician_accounts\".\"sessions_ended_before\" {suffix}"
                )
            );
            assert_eq!(
                statement.values.unwrap().0,
                vec![
                    b"session-hash".to_vec().into(),
                    identity.practice_id.clone().into(),
                    identity.physician_id.clone().into(),
                    true.into(),
                    "workspace".into(),
                    true.into(),
                ]
            );
        }
    }
}
