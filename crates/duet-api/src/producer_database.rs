//! Explicit migration-time provisioning of non-owner producer capabilities.
//!
//! Ordinary runtime credentials retain SELECT authority on source owners. A
//! producer login is separately provisioned by the operator; no role is created,
//! no password is stored, and all producer transactions remain account RLS-bound.
use sqlx::{Connection, PgConnection};

/// The write owner for which a separately provisioned login is authorized.
#[derive(Clone, Copy)]
pub enum ProducerKind {
    /// Canonical context arithmetic and snapshot publication only.
    HealthContext,
    /// Accepted conversation effects: follow-through and Today publication.
    AcceptedWork,
}

/// Grant narrowly scoped producer writes after workspace migrations have run.
/// This is a migration credential operation, never an API startup operation.
pub async fn grant(
    migration_url: &str,
    producer_role: &str,
    kind: ProducerKind,
) -> Result<(), sqlx::Error> {
    let mut connection = PgConnection::connect(migration_url).await?;
    let mut tx = connection.begin().await?;
    let safe: bool = sqlx::query_scalar(
        "select exists(select 1 from pg_roles where rolname=$1 and rolcanlogin and not rolsuper and not rolbypassrls and not rolcreatedb and not rolcreaterole and rolname<>current_user and not pg_has_role(rolname,current_user,'MEMBER'))",
    ).bind(producer_role).fetch_one(&mut *tx).await?;
    if !safe {
        return Err(sqlx::Error::Protocol(
            "producer must be a separately provisioned unprivileged login".into(),
        ));
    }
    let role = format!("\"{}\"", producer_role.replace('"', "\"\""));
    // The role receives base account/ledger privileges via migrate(role). This
    // explicit second step grants only the selected producer family's mutations.
    let statements = match kind {
        ProducerKind::HealthContext => format!(
            "grant select,insert,update on public.health_contexts to {role}; grant select,insert on public.duet_health_context_operations to {role};"
        ),
        ProducerKind::AcceptedWork => format!(
            "grant select,insert,update on public.follow_ups,public.duet_follow_up_occurrences,public.duet_follow_up_settings to {role}; grant select,insert on public.insight_cycles,public.duet_follow_up_origins to {role}; grant update(corrected_insight_ids,updated_at) on public.insight_cycles to {role}; grant select on public.duet_due_follow_up_accounts to {role};"
        ),
    };
    sqlx::raw_sql(&statements).execute(&mut *tx).await?;
    tx.commit().await
}
