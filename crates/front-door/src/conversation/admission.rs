//! Duet admission controls ported from the product's cost-control owner: the
//! append-only operator AI switch, the daily reply allowance with its rolling
//! backstop, and the operator attestation that the model runtime may receive
//! health data. Database time judges every window; the client timezone only
//! chooses the local calendar day. Nothing here dispatches a model.
use super::*;

/// Environment key for the per-person daily allowance.
pub const DAILY_REPLY_LIMIT_ENV: &str = "DUET_DAILY_REPLY_LIMIT";
/// Product default when the key is absent.
pub const DEFAULT_DAILY_REPLY_LIMIT: i64 = 50;
/// Rolling 24h backstop multiple of the daily limit.
pub const ROLLING_MULTIPLE: i64 = 2;
/// Operator attestation that the deployed provider project and key are
/// verified for health data. Setting it is an operator decision, never a default.
pub const PHI_READY_ENV: &str = "DUET_OPENAI_PHI_READY";
const SWITCH_LOCK: &str = "duet:model_execution_switch";
/// Failure marker distinguishing operator pauses from terms pauses.
pub(super) const SWITCHED_OFF: &str = "model_execution_switched_off";

/// Validated daily allowance. Malformed or non-positive values are errors,
/// never silently replaced by the default.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DailyLimit(i64);
impl DailyLimit {
    /// Reads [`DAILY_REPLY_LIMIT_ENV`]; absent or blank means the default.
    pub fn from_env(get: impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        let raw = get(DAILY_REPLY_LIMIT_ENV).unwrap_or_default();
        let raw = raw.trim();
        if raw.is_empty() {
            return Ok(Self(DEFAULT_DAILY_REPLY_LIMIT));
        }
        match raw.parse::<i64>() {
            Ok(n) if raw.bytes().all(|b| b.is_ascii_digit()) && (1..=100_000).contains(&n) => {
                Ok(Self(n))
            }
            _ => Err(format!(
                "{DAILY_REPLY_LIMIT_ENV} must be a positive integer"
            )),
        }
    }
    /// The configured per-local-day limit.
    pub fn get(self) -> i64 {
        self.0
    }
}

/// Operator attestation read from [`PHI_READY_ENV`]. It is evidence an operator
/// supplied, not a qualification this code can establish for itself.
pub struct OperatorAttestation {
    _private: (),
}
impl OperatorAttestation {
    /// Present only for `1`, `true` or `yes`; anything else fails closed.
    pub fn from_env(get: impl Fn(&str) -> Option<String>) -> Option<Self> {
        let value = get(PHI_READY_ENV)?.trim().to_ascii_lowercase();
        ["1", "true", "yes"]
            .contains(&value.as_str())
            .then_some(Self { _private: () })
    }
}

/// Capability to accept NEW conversation work. It carries the allowance policy
/// and proves an operator attested the runtime; the operator switch, consent and
/// allowance are still checked transactionally on every acceptance.
pub struct Admission {
    pub(super) limit: i64,
}
impl Admission {
    /// Composition-only constructor. Without an attestation there is no value.
    pub fn new(limit: DailyLimit, _attestation: OperatorAttestation) -> Self {
        Self { limit: limit.0 }
    }
}

/// Newest audited switch row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwitchChange {
    /// Monotonic audit identity.
    pub id: i64,
    /// Whether model execution is on.
    pub enabled: bool,
    /// Operator identity as supplied by the operator tool.
    pub changed_by: String,
    /// Operator-supplied reason.
    pub reason: String,
}

/// Newest switch row; no row is off. Takes the shared fence so a concurrent
/// change either commits first or waits for this transaction.
pub(super) async fn enabled(tx: &DatabaseTransaction) -> Result<bool, AuthorityError> {
    // PostgreSQL transaction advisory locks have no SeaORM row-lock equivalent.
    // Keep the shared/exclusive protocol and the bound, product-owned lock key.
    tx.execute_raw(sql(
        "SELECT pg_advisory_xact_lock_shared(hashtextextended($1,0))",
        vec![SWITCH_LOCK.into()],
    ))
    .await?;
    Ok(tx
        .query_one_raw(statement(
            Query::select()
                .column(W::Enabled)
                .from(W::Table)
                .order_by(W::Id, Order::Desc)
                .limit(1),
        ))
        .await?
        .map(|r| r.try_get::<bool>("", "enabled"))
        .transpose()?
        .unwrap_or(false))
}

/// Count under the caller's account lock and return the admitting instant.
/// Errors with [`AuthorityError::DailyLimit`] when either bound is reached.
pub(super) async fn allowance(
    tx: &DatabaseTransaction,
    account: Uuid,
    timezone: &str,
    limit: i64,
) -> Result<String, AuthorityError> {
    // PostgreSQL-specific materialized wall-clock/calendar calculation: one
    // volatile clock observation defines both DST-aware local-day bounds and
    // the rolling window. Keep this bound statement rather than splitting it
    // into ORM reads that could observe different instants at a day boundary.
    let r = tx.query_one_raw(sql(
        "WITH b AS (SELECT now_at, (date_trunc('day', now_at AT TIME ZONE $2) AT TIME ZONE $2) AS day_start,
                ((date_trunc('day', now_at AT TIME ZONE $2) + interval '1 day') AT TIME ZONE $2) AS day_end
              FROM (SELECT clock_timestamp() AS now_at) t),
         counted AS (SELECT i.admitted_at FROM duet_conversation_inputs i, b
              WHERE i.account_id=$1 AND i.state<>'failed'
                AND i.admitted_at>=least(b.day_start, b.now_at-interval '24 hours'))
         SELECT b.now_at::text AS now_at,
           extract(epoch FROM b.day_end)::bigint AS day_end,
           (SELECT count(*) FROM counted WHERE admitted_at>=b.day_start AND admitted_at<b.day_end) AS used,
           (SELECT count(*) FROM counted WHERE admitted_at>b.now_at-interval '24 hours') AS rolling_used,
           extract(epoch FROM (SELECT admitted_at FROM counted WHERE admitted_at>b.now_at-interval '24 hours'
              ORDER BY admitted_at DESC OFFSET $3 LIMIT 1) + interval '24 hours')::bigint AS rolling_release
         FROM b",
        vec![account.into(), timezone.into(), (ROLLING_MULTIPLE * limit - 1).into()],
    )).await?.ok_or(AuthorityError::Refused)?;
    let used: i64 = r.try_get("", "used")?;
    let rolling: i64 = r.try_get("", "rolling_used")?;
    let day_end: i64 = r.try_get("", "day_end")?;
    let release: Option<i64> = r.try_get("", "rolling_release")?;
    let day_reached = used >= limit;
    let rolling_reached = rolling >= ROLLING_MULTIPLE * limit;
    if day_reached || rolling_reached {
        let mut resets_at = if day_reached { day_end } else { 0 };
        if rolling_reached {
            resets_at = std::cmp::max(resets_at, release.unwrap_or(day_end));
        }
        return Err(AuthorityError::DailyLimit { resets_at });
    }
    Ok(r.try_get("", "now_at")?)
}

impl PostgresFrontDoorStore {
    /// Append one audited operator change. Turning off first drains account
    /// transactions, then pauses queued/running work as the same accepted runs
    /// (their allowance reservation is kept). Turning on does not resume them.
    pub async fn set_model_execution(
        &self,
        enabled: bool,
        changed_by: &str,
        reason: &str,
    ) -> Result<SwitchChange, AuthorityError> {
        let (changed_by, reason) = (changed_by.trim(), reason.trim());
        if changed_by.is_empty()
            || changed_by.chars().count() > 200
            || reason.is_empty()
            || reason.chars().count() > 1000
        {
            return Err(AuthorityError::Refused);
        }
        let tx = self.db.begin().await?;
        if !enabled {
            // A table-level lock drains existing account lockers and prevents
            // new ones; FOR UPDATE on existing rows cannot fence new accounts.
            // SeaQuery has row locks but no PostgreSQL LOCK TABLE builder.
            tx.execute_raw(sql(
                "LOCK TABLE front_door_accounts IN EXCLUSIVE MODE",
                vec![],
            ))
            .await?;
        }
        // Exclusive counterpart of enabled()'s PostgreSQL advisory fence.
        tx.execute_raw(sql(
            "SELECT pg_advisory_xact_lock(hashtextextended($1,0))",
            vec![SWITCH_LOCK.into()],
        ))
        .await?;
        let r = tx
            .query_one_raw(statement(
                Query::insert()
                    .into_table(W::Table)
                    .columns([W::Enabled, W::ChangedBy, W::Reason])
                    .values_panic([enabled.into(), changed_by.into(), reason.into()])
                    .returning(Query::returning().columns([
                        W::Id,
                        W::Enabled,
                        W::ChangedBy,
                        W::Reason,
                    ])),
            ))
            .await?
            .ok_or(AuthorityError::Refused)?;
        if !enabled {
            let accounts = tx
                .query_all_raw(statement(
                    Query::select()
                        .distinct()
                        .column(I::AccountId)
                        .from(I::Table)
                        .and_where(Expr::col(I::State).is_in(["queued", "running"])),
                ))
                .await?;
            for a in accounts {
                let account: Uuid = a.try_get("", "account_id")?;
                let rev = execution::revision(&tx, account).await?;
                tx.execute_raw(statement(
                    Query::update()
                        .table(T::Table)
                        .value(T::State, "interrupted")
                        .and_where(Expr::col(T::State).eq("running"))
                        .and_where(
                            Expr::col(T::RunId).in_subquery(
                                Query::select()
                                    .column(I::RunId)
                                    .from(I::Table)
                                    .and_where(Expr::col(I::AccountId).eq(account))
                                    .and_where(Expr::col(I::State).is_in(["queued", "running"]))
                                    .to_owned(),
                            ),
                        ),
                ))
                .await?;
                tx.execute_raw(statement(
                    Query::update()
                        .table(M::Table)
                        .value(M::State, "interrupted")
                        .value(M::Revision, rev)
                        .and_where(Expr::col(M::AccountId).eq(account))
                        .and_where(Expr::col(M::Role).eq("assistant"))
                        .and_where(Expr::col(M::State).is_in(["pending", "streaming"])),
                ))
                .await?;
                tx.execute_raw(statement(
                    Query::update()
                        .table(I::Table)
                        .value(I::State, "paused")
                        .value(I::Failure, SWITCHED_OFF)
                        .value(
                            I::LeaseUntil,
                            Expr::Keyword(sea_orm::sea_query::Keyword::Null),
                        )
                        .value(I::Revision, rev)
                        .and_where(Expr::col(I::AccountId).eq(account))
                        .and_where(Expr::col(I::State).is_in(["queued", "running"])),
                ))
                .await?;
            }
        }
        tx.commit().await?;
        Ok(SwitchChange {
            id: r.try_get("", "id")?,
            enabled: r.try_get("", "enabled")?,
            changed_by: r.try_get("", "changed_by")?,
            reason: r.try_get("", "reason")?,
        })
    }
    /// Current switch state for availability answers and the provider choke
    /// point. Absent history is off.
    pub async fn model_execution_enabled(&self) -> Result<bool, AuthorityError> {
        let tx = self.db.begin().await?;
        let on = enabled(&tx).await?;
        tx.commit().await?;
        Ok(on)
    }
}

#[cfg(test)]
mod unit {
    use super::*;
    #[test]
    fn limit_and_attestation_fail_closed() {
        assert_eq!(DailyLimit::from_env(|_| None).unwrap().get(), 50);
        assert_eq!(
            DailyLimit::from_env(|_| Some(" 7 ".into())).unwrap().get(),
            7
        );
        for bad in ["0", "-1", "+3", "x", "1.5", "999999999"] {
            assert!(DailyLimit::from_env(|_| Some(bad.into())).is_err(), "{bad}");
        }
        assert!(OperatorAttestation::from_env(|_| None).is_none());
        for no in ["", "0", "false", "no", "maybe"] {
            assert!(OperatorAttestation::from_env(|_| Some(no.into())).is_none());
        }
        for yes in ["1", "TRUE", " yes "] {
            assert!(OperatorAttestation::from_env(|_| Some(yes.into())).is_some());
        }
    }
}
