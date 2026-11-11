//! The physician product's shared provider cooldown. It contains no patient
//! identity or request data; the database clock governs every replica.
use std::time::Duration;

use sea_orm::{
    ConnectionTrait,
    sea_query::{Alias, Expr, ExprTrait, Func, Query},
};

use super::{Store, async_trait, entities::provider_quota, error};

fn clock_milliseconds() -> Expr {
    // SeaQuery has no expression for PostgreSQL's wall clock and epoch
    // extraction. The expression is fixed; delay values remain parameters.
    Expr::cust("floor(extract(epoch FROM clock_timestamp()) * 1000)::bigint")
}

#[async_trait]
impl harness::provider::QuotaGate for Store {
    async fn remaining(&self) -> Result<Duration, harness::engine::Cause> {
        let query = Query::select()
            .expr_as(
                Expr::col(provider_quota::Column::NotBeforeMs).sub(clock_milliseconds()),
                Alias::new("remaining_ms"),
            )
            .from(provider_quota::Entity)
            .and_where(Expr::col(provider_quota::Column::Singleton).eq(true))
            .to_owned();
        let row = self
            .db
            .query_one(&query)
            .await?
            .ok_or_else(|| error("provider quota row is missing"))?;
        let milliseconds: i64 = row.try_get("", "remaining_ms")?;
        Ok(Duration::from_millis(
            std::cmp::max(milliseconds, 0).unsigned_abs(),
        ))
    }

    async fn defer(&self, delay: Duration) -> Result<(), harness::engine::Cause> {
        // Round up fractional milliseconds, so another replica never starts
        // before the provider's requested delay has elapsed. The write also
        // rounds the database clock up; reads use its floor.
        let milliseconds =
            delay.as_millis() + u128::from(!delay.subsec_nanos().is_multiple_of(1_000_000));
        let milliseconds = i64::try_from(milliseconds)?;
        let query = Query::update()
            .table(provider_quota::Entity)
            .value(
                provider_quota::Column::NotBeforeMs,
                Func::greatest([
                    Expr::col(provider_quota::Column::NotBeforeMs),
                    clock_milliseconds().add(1_i64).add(milliseconds),
                ]),
            )
            .and_where(Expr::col(provider_quota::Column::Singleton).eq(true))
            .to_owned();
        let changed = self.db.execute(&query).await?;
        if changed.rows_affected() != 1 {
            return Err(error("provider quota row is missing").into());
        }
        Ok(())
    }
}
