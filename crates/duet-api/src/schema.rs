//! The vendored Duet schema chain.
//!
//! The SQL files in `crates/duet-api/migrations` are copied verbatim from
//! coda-app (see that directory's README for the source commit). Each file is
//! one transaction that records its own version in `public.schema_migrations`.
//! This applies the versions not yet recorded, in filename order, and checks
//! that the file wrote its receipt. A rerun applies nothing.
//!
//! `MIGRATIONS` carries the SHA-256 of every vendored file, computed when the
//! crate was built. The SQL compiled into the binary is hashed again before
//! anything runs, so a migration that does not match its vendored bytes is
//! refused.

use sha2::{Digest, Sha256};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};

include!(concat!(env!("OUT_DIR"), "/duet_migrations.rs"));

/// Applies every unrecorded migration. Returns how many were applied.
pub async fn apply(database_url: &str) -> Result<usize, String> {
    verify_checksums()?;
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(database_url)
        .await
        .map_err(|error| error.to_string())?;
    let mut applied = 0;
    for (file, _) in MIGRATIONS {
        let version = file.trim_end_matches(".sql");
        if recorded(&pool, version).await? {
            continue;
        }
        let mut tx = pool.begin().await.map_err(|error| error.to_string())?;
        // The SQL is a vendored file whose checksum was just verified above.
        sqlx::raw_sql(sqlx::AssertSqlSafe(migration_sql(file)))
            .execute(&mut *tx)
            .await
            .map_err(|error| format!("{version}: {error}"))?;
        tx.commit()
            .await
            .map_err(|error| format!("{version}: {error}"))?;
        if !recorded(&pool, version).await? {
            return Err(format!(
                "{version} committed without recording its schema_migrations receipt"
            ));
        }
        applied += 1;
    }
    Ok(applied)
}

/// Confirms every compiled migration matches the checksum pinned at build time.
pub fn verify_checksums() -> Result<(), String> {
    for (file, expected) in MIGRATIONS {
        let digest = Sha256::digest(migration_sql(file).as_bytes());
        let digest = digest
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        if digest != *expected {
            return Err(format!(
                "{file} checksum {digest} does not match the vendored {expected}"
            ));
        }
    }
    Ok(())
}

async fn recorded(pool: &PgPool, version: &str) -> Result<bool, String> {
    let ledger_exists: bool = sqlx::query_scalar(
        "select exists (\
            select 1 from information_schema.tables \
            where table_schema = 'public' and table_name = 'schema_migrations')",
    )
    .fetch_one(pool)
    .await
    .map_err(|error| error.to_string())?;
    if !ledger_exists {
        return Ok(false);
    }
    let row =
        sqlx::query("select exists (select 1 from public.schema_migrations where version = $1)")
            .bind(version)
            .fetch_one(pool)
            .await
            .map_err(|error| error.to_string())?;
    row.try_get(0).map_err(|error| error.to_string())
}
