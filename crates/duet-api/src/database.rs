//! Workspace-owned migration and account transaction boundary.
//!
//! Migration credentials are supplied only to `duet-migrate`, never to the
//! runtime store. This foundation bootstraps `public.users` or adopts it in
//! place; evidence, privacy, conversation and workspace owners append their
//! own migrations here, without replaying the reference beta reset. The
//! reference `schema_migrations` ledger is not claimed or modified.

#[warn(
    clippy::disallowed_methods,
    reason = "the typed adapter states its own exception"
)]
pub(crate) mod orm;

use sha2::{Digest, Sha256};
use sqlx::{AssertSqlSafe, Connection, PgConnection, PgPool, Postgres, Transaction};
use uuid::Uuid;

const MIGRATIONS: &[(&str, &str)] = &[
    (
        "0001_account_boundary",
        include_str!("../migrations/0001_account_boundary.sql"),
    ),
    (
        "0002_identity_privacy",
        include_str!("../migrations/0002_identity_privacy.sql"),
    ),
    (
        "0003_canonical_evidence",
        include_str!("../migrations/0003_canonical_evidence.sql"),
    ),
    (
        "0004_doctor_publication",
        include_str!("../migrations/0004_doctor_publication.sql"),
    ),
    (
        "0005_conversations",
        include_str!("../migrations/0005_conversations.sql"),
    ),
    (
        "0006_public_budget",
        include_str!("../migrations/0006_public_budget.sql"),
    ),
    (
        "0007_legacy_receipts",
        include_str!("../migrations/0007_legacy_receipts.sql"),
    ),
    (
        "0008_conversation_admission",
        include_str!("../migrations/0008_conversation_admission.sql"),
    ),
    (
        "0009_retained_context_owners",
        include_str!("../migrations/0009_retained_context_owners.sql"),
    ),
    (
        "0010_health_context_producer",
        include_str!("../migrations/0010_health_context_producer.sql"),
    ),
    (
        "0011_follow_up_producer",
        include_str!("../migrations/0011_follow_up_producer.sql"),
    ),
    (
        "0012_today_producer",
        include_str!("../migrations/0012_today_producer.sql"),
    ),
    (
        "0013_scheduled_follow_up_work",
        include_str!("../migrations/0013_scheduled_follow_up_work.sql"),
    ),
    (
        "0014_execution_policy",
        include_str!("../migrations/0014_execution_policy.sql"),
    ),
    (
        "0015_legacy_privacy_cohort",
        include_str!("../migrations/0015_legacy_privacy_cohort.sql"),
    ),
    (
        "0016_public_budget_misses",
        include_str!("../migrations/0016_public_budget_misses.sql"),
    ),
    (
        "0017_conversation_steers",
        include_str!("../migrations/0017_conversation_steers.sql"),
    ),
    (
        "0018_source_authority",
        include_str!("../migrations/0018_source_authority.sql"),
    ),
];

fn refused(message: &str) -> sqlx::Error {
    sqlx::Error::Protocol(message.to_owned())
}

/// Apply checksum-verified workspace migrations with separate owner credentials.
///
/// The operator provisions the runtime login; this function never creates roles
/// or changes credentials. Existing account/evidence/workspace rows and the
/// reference migration ledger are preserved. Repeated application is a no-op.
pub async fn migrate(migration_url: &str, runtime_role: &str) -> Result<(), sqlx::Error> {
    let mut connection = PgConnection::connect(migration_url).await?;
    let mut tx = connection.begin().await?;
    sqlx::query("select pg_advisory_xact_lock(460001)")
        .execute(&mut *tx)
        .await?;
    let safe: bool = sqlx::query_scalar(
        "select exists(select 1 from pg_roles where rolname = $1 and rolcanlogin \
         and not rolsuper and not rolbypassrls and not rolcreatedb and not rolcreaterole \
         and rolname <> current_user and not pg_has_role(rolname, current_user, 'MEMBER'))",
    )
    .bind(runtime_role)
    .fetch_one(&mut *tx)
    .await?;
    if !safe {
        return Err(refused(
            "runtime role must be a separate unprivileged non-owner login",
        ));
    }
    sqlx::raw_sql(
        "create table if not exists public.duet_schema_migrations (\
         version text primary key, checksum text not null, applied_at timestamptz not null default now());\
         revoke all on public.duet_schema_migrations from public;",
    )
    .execute(&mut *tx)
    .await?;
    for (version, sql) in MIGRATIONS {
        let checksum: String = Sha256::digest(sql.as_bytes())
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        let existing: Option<String> = sqlx::query_scalar(
            "select checksum from public.duet_schema_migrations where version = $1",
        )
        .bind(version)
        .fetch_optional(&mut *tx)
        .await?;
        if let Some(existing) = existing {
            if existing != checksum {
                return Err(refused("applied Duet migration checksum differs"));
            }
            continue;
        }
        sqlx::raw_sql(*sql).execute(&mut *tx).await?;
        sqlx::query("insert into public.duet_schema_migrations(version, checksum) values ($1, $2)")
            .bind(version)
            .bind(checksum)
            .execute(&mut *tx)
            .await?;
    }
    // Quote an identifier, not a connection string or a value supplied to SQL.
    let role = format!("\"{}\"", runtime_role.replace('"', "\"\""));
    // `role` is the quoted identifier above; nothing else is interpolated.
    sqlx::raw_sql(AssertSqlSafe(format!(
        "grant usage on schema public to {role}; \
         grant select, insert, update, delete on public.users, public.duet_front_door_mapping, public.health_evidence, public.clinical_notes, public.evidence_sync_changes to {role}; \
         grant select, insert on public.canonical_evidence_revisions, public.duet_evidence_ingestions to {role}; \
         revoke update, delete on public.canonical_evidence_revisions, public.duet_evidence_ingestions from {role}; \
         grant select, insert, update on public.privacy_recovery_events to {role}; \
         grant select on public.duet_identity_directory, public.duet_privacy_pending_accounts, public.duet_publication_directory, public.duet_restore_gate, public.duet_unsupported_receipts to {role}; \
         grant select,insert on public.duet_deleted_identities to {role}; \
         grant select,insert,update,delete on public.doctor_collection_items,public.duet_doctor_proposals,public.duet_doctor_saves,public.duet_device_snapshots,public.duet_publications to {role}; \
         grant usage, select on sequence public.canonical_evidence_revisions_revision_seq to {role}; \
         grant select,insert,update on public.duet_model_calls to {role}; \
         grant select,insert,update,delete on public.duet_conversations,public.duet_commands,public.duet_execution_records,public.duet_tool_effects,public.duet_private_memory,public.duet_conversation_events,public.duet_command_steers to {role}; \
         grant usage,select on sequence public.duet_execution_records_ordinal_seq, public.duet_conversation_events_sequence_seq to {role}; \
         grant select on public.duet_pending_commands,public.health_contexts,public.follow_ups,public.insight_cycles,public.duet_follow_up_origins,public.duet_follow_up_occurrences,public.duet_follow_up_settings to {role}; \
         grant select on public.duet_legacy_privacy_accounts to {role}; \
         grant select,insert on public.duet_legacy_privacy_replays to {role}; \
         grant execute on function public.duet_admit_public_retrieval() to {role}; \
         grant execute on function public.duet_admit_public_retrieval_miss() to {role}; \
         grant execute on function public.acting_account() to {role}; \
         revoke all on public.duet_schema_migrations from {role};"
    )))
    .execute(&mut *tx)
    .await?;
    // Check the effective grant graph, including inherited owner membership,
    // before committing an installation. Provisioning must revoke unsafe legacy
    // grants explicitly; the migrator does not silently rewrite other owners.
    sqlx::raw_sql(AssertSqlSafe(format!("set local role {role}")))
        .execute(&mut *tx)
        .await?;
    validate_runtime(&mut tx).await?;
    sqlx::query("reset role").execute(&mut *tx).await?;
    tx.commit().await
}

/// Pool used by durable Duet owners, never by migrations or physician state.
#[derive(Clone, Debug)]
pub struct AccountDatabase {
    pool: PgPool,
}

impl AccountDatabase {
    /// Connect with runtime credentials, refusing roles that can bypass RLS or
    /// change schema. No migration runs on this path.
    pub async fn connect(runtime_url: &str) -> Result<Self, sqlx::Error> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(8)
            .connect(runtime_url)
            .await?;
        let database = Self { pool };
        let mut tx = database.pool.begin().await?;
        validate_runtime(&mut tx).await?;
        tx.rollback().await?;
        Ok(database)
    }

    /// Shared atomic budget for a valid public retrieval.
    pub async fn admit_public_retrieval(&self) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar("select public.duet_admit_public_retrieval()")
            .fetch_one(&self.pool)
            .await
    }
    /// Shared atomic budget for a failed public lookup.
    ///
    /// Raw SQL is allowed because this is the existing plpgsql bucket, not
    /// ordinary CRUD. The runtime role can only execute the security-definer
    /// function.
    pub async fn admit_public_retrieval_miss(&self) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar("select public.duet_admit_public_retrieval_miss()")
            .fetch_one(&self.pool)
            .await
    }
    /// Resolve only capability bookkeeping before entering the owning account transaction.
    pub async fn publication_owner(
        &self,
        hash: &str,
        shl: bool,
    ) -> Result<Option<(Uuid, Uuid)>, sqlx::Error> {
        sqlx::query_as("select id,user_id from duet_publication_directory where (case when $2 then shl_hash else token_hash end)=$1")
            .bind(hash).bind(shl).fetch_optional(&self.pool).await
    }
    /// Resolve deletion bookkeeping solely for exact privacy receipt reconciliation.
    pub async fn deleted_subject(
        &self,
        subject: &str,
    ) -> Result<Option<(Uuid, Uuid)>, sqlx::Error> {
        sqlx::query_as(
            "select account_id,incarnation from duet_deleted_identities where auth_user_id=$1",
        )
        .bind(subject)
        .fetch_optional(&self.pool)
        .await
    }
    /// Product deletion tombstones block stale provider sessions from recreating an account.
    pub async fn subject_deleted(&self, subject: &str) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar(
            "select exists(select 1 from duet_deleted_identities where auth_user_id=$1)",
        )
        .bind(subject)
        .fetch_one(&self.pool)
        .await
    }
    /// Legacy receipts need an explicit converter before this runtime can serve a restored database.
    pub async fn has_unsupported_receipts(&self) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar("select present from duet_unsupported_receipts")
            .fetch_one(&self.pool)
            .await
    }
    /// Bounded recovery directory contains account identifiers only, never private receipt content.
    pub async fn pending_privacy_accounts(&self) -> Result<Vec<Uuid>, sqlx::Error> {
        sqlx::query_scalar("select account_id from public.duet_privacy_pending_accounts order by account_id limit 64")
            .fetch_all(&self.pool).await
    }

    /// Resolve only the immutable provider subject, before entering account scope.
    pub async fn resolve_subject(&self, subject: &str) -> Result<Option<Uuid>, sqlx::Error> {
        sqlx::query_scalar("select id from public.duet_identity_directory where auth_user_id=$1")
            .bind(subject)
            .fetch_optional(&self.pool)
            .await
    }

    /// Begin a transaction scoped to the verified internal `users.id` (not an
    /// auth subject or front-door UUID). Authentication/mapping stays with the
    /// identity owner. Local scope is cleared on commit and rollback, including
    /// error/drop paths. Every owner uses this entrypoint, including workers.
    pub async fn begin(&self, account_id: Uuid) -> Result<Transaction<'_, Postgres>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        validate_runtime(&mut tx).await?;
        sqlx::query("select set_config('app.account_id', $1, true)")
            .bind(account_id.to_string())
            .execute(&mut *tx)
            .await?;
        Ok(tx)
    }
}

async fn validate_runtime(tx: &mut Transaction<'_, Postgres>) -> Result<(), sqlx::Error> {
    let safe: bool = sqlx::query_scalar(
        "select not (r.rolsuper or r.rolbypassrls or r.rolcreatedb or r.rolcreaterole) \
         and not has_schema_privilege(current_user, 'public', 'CREATE') \
         and not exists(select 1 from pg_roles elevated where \
             (elevated.rolsuper or elevated.rolbypassrls) \
             and pg_has_role(current_user, elevated.oid, 'MEMBER')) \
         and not exists(select 1 from pg_class c join pg_namespace n on n.oid = c.relnamespace \
             where n.nspname = 'public' and c.relkind in ('r', 'p') \
             and (pg_has_role(current_user, c.relowner, 'MEMBER') \
                  or has_table_privilege(current_user, c.oid, 'TRUNCATE'))) \
         and exists(select 1 from pg_class where oid = to_regclass('public.users') and relrowsecurity) \
         from pg_roles r where r.rolname = current_user",
    )
    .fetch_one(&mut **tx)
    .await?;
    if !safe {
        return Err(refused(
            "Duet runtime role can bypass the account boundary or schema is absent",
        ));
    }
    Ok(())
}
