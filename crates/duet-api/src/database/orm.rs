//! Run Duet-owned ORM statements inside the caller's account-scoped transaction.

use sea_orm::sea_query::{InsertStatement, PostgresQueryBuilder, SelectStatement, UpdateStatement};
use sea_query_sqlx::{SqlxBinder, SqlxValues};
use sqlx::{AssertSqlSafe, Postgres, Transaction, postgres::PgRow};

/// A typed SeaQuery statement executable on the account's sqlx transaction.
pub(crate) trait Statement {
    /// Render PostgreSQL text with every runtime value bound separately.
    fn render(&self) -> (String, SqlxValues);
}
macro_rules! statement {
    ($($t:ty),*) => {$(
        impl Statement for $t {
            fn render(&self) -> (String, SqlxValues) {
                self.build_sqlx(PostgresQueryBuilder)
            }
        }
    )*};
}
statement!(SelectStatement, InsertStatement, UpdateStatement);

/// Execute a typed statement; returns affected rows.
pub(crate) async fn execute(
    tx: &mut Transaction<'_, Postgres>,
    statement: &impl Statement,
) -> Result<u64, sqlx::Error> {
    let (sql, values) = statement.render();
    // SeaQuery renders code-owned identifiers; every runtime value is bound.
    Ok(sqlx::query_with(AssertSqlSafe(sql), values)
        .execute(&mut **tx)
        .await?
        .rows_affected())
}

/// Fetch at most one row of a typed statement.
pub(crate) async fn row(
    tx: &mut Transaction<'_, Postgres>,
    statement: &impl Statement,
) -> Result<Option<PgRow>, sqlx::Error> {
    let (sql, values) = statement.render();
    // SeaQuery renders code-owned identifiers; every runtime value is bound.
    sqlx::query_with(AssertSqlSafe(sql), values)
        .fetch_optional(&mut **tx)
        .await
}

/// Fetch every row of a typed statement.
pub(crate) async fn rows(
    tx: &mut Transaction<'_, Postgres>,
    statement: &impl Statement,
) -> Result<Vec<PgRow>, sqlx::Error> {
    let (sql, values) = statement.render();
    // SeaQuery renders code-owned identifiers; every runtime value is bound.
    sqlx::query_with(AssertSqlSafe(sql), values)
        .fetch_all(&mut **tx)
        .await
}

/// Fetch at most one typed entity model.
pub(crate) async fn model<M: sea_orm::FromQueryResult>(
    tx: &mut Transaction<'_, Postgres>,
    statement: &impl Statement,
) -> Result<Option<M>, sqlx::Error> {
    row(tx, statement).await?.map(decode).transpose()
}

/// Fetch every typed entity model.
pub(crate) async fn models<M: sea_orm::FromQueryResult>(
    tx: &mut Transaction<'_, Postgres>,
    statement: &impl Statement,
) -> Result<Vec<M>, sqlx::Error> {
    rows(tx, statement).await?.into_iter().map(decode).collect()
}

fn decode<M: sea_orm::FromQueryResult>(row: PgRow) -> Result<M, sqlx::Error> {
    M::from_query_result(&sea_orm::QueryResult::from(row), "")
        .map_err(|error| sqlx::Error::Decode(error.to_string().into()))
}
