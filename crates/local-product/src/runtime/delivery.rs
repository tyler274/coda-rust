//! Hosted work is run from the rows that own it. The conversation row (and
//! the reply-draft row) is the acceptance, completion, lease, retry and
//! recovery owner. A `NOTIFY` on [`WORK_CHANNEL`], sent in the accepting
//! transaction, only tells a listening worker sooner than its next poll; a
//! lost notification delays work and never loses it. Provider calls
//! themselves are not exactly-once.
use std::time::Duration;

use sea_orm::sea_query::{Alias, Expr, Func, IntoColumnRef, IntoTableRef, Query, SelectStatement};
use sea_orm::sqlx::postgres::PgListener;
use sea_orm::{ConnectionTrait, DatabaseTransaction, DbErr, EntityTrait, RuntimeErr};
use serde::{Deserialize, Serialize};

use super::entities::{conversation, reply_draft};
use super::{Claim, Conversation, MAX_FAILED_ATTEMPTS, Scope, Store, database_clock, error, keys};

/// The Postgres notification channel that wakes hosted workers.
pub const WORK_CHANNEL: &str = "physician_work";

/// Postgres refuses a notification payload of 8000 bytes or more.
const MAX_NOTIFY_BYTES: usize = 7999;

/// W3C trace context of the request that made an input runnable, saved with
/// it so the attempt that runs it can relate to that request.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceContext {
    /// The `traceparent` header value.
    pub traceparent: String,
    /// The `tracestate` header value, when the request carried one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracestate: Option<String>,
}

/// How long failed work waits before it is due again: `first` after the
/// first failure, doubling with each further one, never above `ceiling`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Backoff {
    first: i64,
    ceiling: i64,
}

impl Backoff {
    /// Hosted retries: 10 s, then doubling to at most 600 s.
    pub const HOSTED: Self = Self {
        first: 10,
        ceiling: 600,
    };

    /// No delay: failed work is due again at once. For proofs that drive
    /// retries without waiting; hosted work uses [`Self::HOSTED`].
    pub const IMMEDIATE: Self = Self {
        first: 0,
        ceiling: 0,
    };

    /// A schedule starting at `first` seconds and capped at `ceiling`.
    /// `None` unless `0 <= first <= ceiling <= 600`.
    pub fn new(first: i64, ceiling: i64) -> Option<Self> {
        (0 <= first && first <= ceiling && ceiling <= 600).then_some(Self { first, ceiling })
    }

    /// Seconds to wait after the `failures`th unsuccessful attempt.
    fn seconds(self, failures: u32) -> i64 {
        let doublings = failures.saturating_sub(1).min(16);
        self.first
            .saturating_mul(1_i64 << doublings)
            .min(self.ceiling)
    }
}

/// What a recorded failed attempt left behind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Retry {
    /// The work is pending again and due after this long.
    After(Duration),
    /// This was the [`MAX_FAILED_ATTEMPTS`]th failure: the work ended as
    /// failed and only Try again reopens it.
    GaveUp,
    /// The claim no longer owned the work (Stop, a newer input, or a
    /// successor attempt), so nothing changed.
    NotCurrent,
}

/// Records one more failure on work whose attempt just failed and says what
/// became of it. `failures` and `dispatch_after` are the row's own fields.
pub(super) fn failed_once(
    failures: &mut u32,
    dispatch_after: &mut i64,
    now: i64,
    backoff: Backoff,
) -> (&'static str, Retry) {
    *failures += 1;
    if *failures >= MAX_FAILED_ATTEMPTS {
        return ("failed", Retry::GaveUp);
    }
    let seconds = backoff.seconds(*failures);
    *dispatch_after = now + seconds;
    (
        "pending",
        Retry::After(Duration::from_secs(seconds.unsigned_abs())),
    )
}

/// Tells listening workers, when `tx` commits, that `delivery` is runnable.
/// The payload is the delivery identity only. One too large for Postgres to
/// carry is left to the workers' poll.
pub(super) async fn notify(tx: &DatabaseTransaction, delivery: &Delivery) -> Result<(), DbErr> {
    let payload = serde_json::to_string(delivery).map_err(|_| error("invalid delivery"))?;
    if payload.len() > MAX_NOTIFY_BYTES {
        return Ok(());
    }
    let query = Query::select()
        .expr(
            Func::cust(Alias::new("pg_notify"))
                .arg(WORK_CHANNEL)
                .arg(payload),
        )
        .to_owned();
    tx.execute(&query).await?;
    Ok(())
}

/// A worker's subscription to [`WORK_CHANNEL`] on its own connection.
pub struct Wakeups {
    listener: PgListener,
}

impl Wakeups {
    /// The next delivery a committed transaction announced. `Ok(None)` when
    /// the connection closed: notifications sent until a new subscription
    /// listens are lost, so the caller must poll after resubscribing. A
    /// payload that is not a delivery is skipped.
    pub async fn next(&mut self) -> Result<Option<Delivery>, DbErr> {
        loop {
            let Some(notification) = self.listener.try_recv().await.map_err(connection)? else {
                return Ok(None);
            };
            match serde_json::from_str(notification.payload()) {
                Ok(delivery) => return Ok(Some(delivery)),
                Err(_) => tracing::warn!("physician work notification is not a delivery"),
            }
        }
    }
}

/// Seconds until the soonest live lease in `table` runs out, by the database
/// clock; null when no lease is live.
fn soonest_lease_end(table: impl IntoTableRef, state: impl IntoColumnRef) -> SelectStatement {
    // Scoped here: the blanket `ExprTrait::min` would shadow `Ord::min`.
    use sea_orm::sea_query::ExprTrait;
    use sea_orm::sea_query::extension::postgres::PgExpr;
    let state = Expr::col(state);
    let expires = state
        .clone()
        .get_json_field("attempts")
        .get_json_field(-1_i32)
        .cast_json_field("expires")
        .cast_as(Alias::new("bigint"));
    Query::select()
        .expr_as(
            Expr::from(Func::min(expires.clone())).sub(database_clock()),
            Alias::new("left"),
        )
        .from(table)
        .and_where(state.cast_json_field("status").eq("active"))
        .and_where(expires.gt(database_clock()))
        .to_owned()
}

fn connection(source: sea_orm::sqlx::Error) -> DbErr {
    DbErr::Conn(RuntimeErr::SqlxError(source.into()))
}

/// Which durable owner a wakeup names.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryKind {
    /// A chat turn in `physician_runtime_conversations`.
    #[default]
    ChatTurn,
    /// A reply revision in `physician_runtime_reply_drafts`; `scope.chat`
    /// names the reply work.
    ReplyRevision,
}

impl DeliveryKind {
    /// The kind's name in telemetry: `chat_turn` or `reply_revision`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChatTurn => "chat_turn",
            Self::ReplyRevision => "reply_revision",
        }
    }

    fn is_chat_turn(&self) -> bool {
        *self == Self::ChatTurn
    }
}

/// Non-clinical identity of runnable work, fenced to the accepted input.
/// It is the whole notification payload. A chat turn omits `kind`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Delivery {
    /// The owner this wakeup names; absent means a chat turn.
    #[serde(default, skip_serializing_if = "DeliveryKind::is_chat_turn")]
    pub kind: DeliveryKind,
    /// Complete product-owned authorization scope, not user-supplied authority.
    pub scope: Scope,
    /// Idempotency identity of the latest accepted input.
    pub input_id: String,
}

impl Delivery {
    /// A chat-turn wakeup for `input_id`.
    pub fn chat_turn(scope: Scope, input_id: String) -> Self {
        Self {
            kind: DeliveryKind::ChatTurn,
            scope,
            input_id,
        }
    }

    /// A reply-revision wakeup for the revision command `input_id`.
    pub fn reply_revision(scope: Scope, input_id: String) -> Self {
        Self {
            kind: DeliveryKind::ReplyRevision,
            scope,
            input_id,
        }
    }
}

/// One keyset page of due work.
#[derive(Debug)]
pub struct DispatchPage {
    /// Due deliveries. A worker may see one more than once; the claim fences it.
    pub deliveries: Vec<Delivery>,
    /// Continue here; None ends this sweep.
    pub next: Option<Scope>,
}

impl Store {
    /// Subscribes to [`WORK_CHANNEL`] on a connection taken from this
    /// store's pool and held until the subscription is dropped.
    pub async fn wakeups(&self) -> Result<Wakeups, DbErr> {
        let mut listener = PgListener::connect_with(self.db.get_postgres_connection_pool())
            .await
            .map_err(connection)?;
        // A reconnect inside the listener would hide the gap in which
        // notifications were lost; the caller resubscribes and polls instead.
        listener.eager_reconnect(false);
        listener.listen(WORK_CHANNEL).await.map_err(connection)?;
        Ok(Wakeups { listener })
    }

    /// One bounded page of due chat turns: pending work whose
    /// `dispatch_after` has passed, and attempts whose lease ran out. Uses
    /// the existing pending projection, including its Postgres JSON/clock
    /// predicate; it does not introduce another persistence family.
    pub async fn dispatch_due(
        &self,
        maximum: usize,
        after: Option<&Scope>,
    ) -> Result<DispatchPage, DbErr> {
        if !(1..=100).contains(&maximum) {
            return Err(error("invalid dispatch bounds"));
        }
        let scopes = self.pending_batch(maximum, after).await?;
        let next = (scopes.len() == maximum).then(|| scopes.last().unwrap().clone());
        let mut deliveries = Vec::new();
        for scope in scopes {
            let (tx, state, now) = self.locked(&scope).await?;
            if state.session_hash.is_some()
                && ((state.status == "pending" && state.dispatch_after <= now)
                    || (state.status == "active"
                        && state.attempts.last().is_some_and(|a| a.expires <= now)))
                && let Some(input_id) = state.input_id
            {
                deliveries.push(Delivery::chat_turn(scope, input_id));
            }
            tx.commit().await?;
        }
        Ok(DispatchPage { deliveries, next })
    }

    /// How long until the soonest live claim on a chat turn or reply
    /// revision runs out, when one is held. A worker that died leaves no
    /// notification, so its turn becomes due only then. Leases already run
    /// out are left to the poll.
    pub async fn soonest_lease_end(&self) -> Result<Option<Duration>, DbErr> {
        let tables = [
            soonest_lease_end(conversation::Entity, conversation::Column::State),
            soonest_lease_end(reply_draft::Entity, reply_draft::Column::State),
        ];
        let mut soonest: Option<i64> = None;
        for query in tables {
            let left: Option<i64> = match self.db.query_one(&query).await? {
                Some(row) => row.try_get("", "left")?,
                None => None,
            };
            soonest = soonest.into_iter().chain(left).min();
        }
        Ok(soonest.map(|seconds| Duration::from_secs(seconds.unsigned_abs())))
    }

    /// Atomically claims only the input named by the delivery, never a newer
    /// turn, and only once it is due.
    pub async fn claim_delivery(
        &self,
        delivery: &Delivery,
        lease_seconds: i64,
    ) -> Result<Option<Claim>, DbErr> {
        if delivery.kind != DeliveryKind::ChatTurn {
            return Err(error("not a chat turn delivery"));
        }
        self.claim_inner(&delivery.scope, lease_seconds, Some(&delivery.input_id))
            .await
    }

    /// Extends only a live claim using the database clock; expiry cannot revive.
    pub async fn heartbeat(&self, claim: &Claim, lease_seconds: i64) -> Result<(), DbErr> {
        if !(1..=600).contains(&lease_seconds) {
            return Err(error("invalid lease duration"));
        }
        let (tx, mut state, now) = self.locked(&claim.scope).await?;
        if !Self::current(&state, claim, now) {
            return Err(error("stale heartbeat"));
        }
        state.attempts.last_mut().unwrap().expires = now + lease_seconds;
        Self::save(tx, &claim.scope, &state).await
    }

    /// Records this claim's unsuccessful attempt and makes the work due
    /// again after `backoff`. Never releases Stop, changes a successor, or
    /// reopens completed or failed work: a reached bound has already ended
    /// the turn, and only Try again reopens it. The attempt is recorded as
    /// failed; an expired lease is recorded as interrupted by the next claim
    /// instead. The [`MAX_FAILED_ATTEMPTS`]th failure ends the work as
    /// failed, so nothing starts it again.
    pub async fn retry_delivery(&self, claim: &Claim, backoff: Backoff) -> Result<Retry, DbErr> {
        let (tx, mut state, now) = self.locked(&claim.scope).await?;
        let owned = state.revision == claim.revision && state.status == "active";
        let Some(attempt) = state
            .attempts
            .last_mut()
            .filter(|a| owned && a.id == claim.attempt)
        else {
            tx.commit().await?;
            return Ok(Retry::NotCurrent);
        };
        if attempt.status == "active" {
            attempt.status = "failed".into();
        }
        let (status, retry) =
            failed_once(&mut state.failures, &mut state.dispatch_after, now, backoff);
        state.status = status.into();
        Self::save(tx, &claim.scope, &state).await?;
        Ok(retry)
    }

    /// Hands a live claim back when its worker is stopping, so another
    /// worker runs the turn at once instead of waiting out the lease. The
    /// attempt is recorded as interrupted and is not counted as a failure.
    /// A claim that no longer owns the work changes nothing.
    pub async fn release(&self, claim: &Claim) -> Result<(), DbErr> {
        let (tx, mut state, _) = self.locked(&claim.scope).await?;
        let owned = state.revision == claim.revision && state.status == "active";
        let Some(attempt) = state
            .attempts
            .last_mut()
            .filter(|a| owned && a.id == claim.attempt && a.status == "active")
        else {
            return tx.commit().await;
        };
        attempt.status = "interrupted".into();
        state.status = "pending".into();
        state.dispatch_after = 0;
        if let Some(input_id) = state.input_id.clone() {
            notify(&tx, &Delivery::chat_turn(claim.scope.clone(), input_id)).await?;
        }
        Self::save(tx, &claim.scope, &state).await
    }

    /// Unlocked display read: never contends with a claim or publication lock.
    /// `None` before the first accepted input. Not a fence for any decision.
    pub async fn read(&self, scope: &Scope) -> Result<Option<Conversation>, DbErr> {
        let Some(row) = conversation::Entity::find_by_id(keys(scope))
            .one(&self.db)
            .await?
        else {
            return Ok(None);
        };
        serde_json::from_value(row.state)
            .map(Some)
            .map_err(|_| error("corrupt conversation"))
    }

    /// Where the input named by a delivery stands, read under the
    /// conversation lock. A worker uses it when its claim found nothing to run.
    pub async fn delivery_state(&self, delivery: &Delivery) -> Result<DeliveryState, DbErr> {
        if delivery.kind == DeliveryKind::ReplyRevision {
            return self.revision_state(delivery).await;
        }
        let (tx, state, now) = self.locked(&delivery.scope).await?;
        tx.commit().await?;
        if state.input_id.as_deref() != Some(&delivery.input_id) {
            return Ok(DeliveryState::Settled);
        }
        Ok(match state.status.as_str() {
            "completed" | "held" => DeliveryState::Settled,
            "failed" => DeliveryState::Failed,
            "active" if state.attempts.last().is_some_and(|a| a.expires > now) => {
                DeliveryState::Claimed
            }
            _ => DeliveryState::Due,
        })
    }
}

/// Where the work a delivery names stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeliveryState {
    /// Committed, held by Stop, or superseded by a newer accepted input:
    /// nothing is left for this delivery to do.
    Settled,
    /// A live claim is running this input. If that worker dies, its lease
    /// expires and the work is due again.
    Claimed,
    /// Runnable now or once its retry delay passes: pending, or a lease that
    /// has run out.
    Due,
    /// Ended as failed after [`MAX_FAILED_ATTEMPTS`]. Try again resumes it.
    Failed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hosted_backoff_grows_from_ten_seconds_to_its_ceiling() {
        let delays: Vec<i64> = (1..=8).map(|n| Backoff::HOSTED.seconds(n)).collect();
        assert_eq!(delays, [10, 20, 40, 80, 160, 320, 600, 600]);
        assert_eq!(Backoff::HOSTED.seconds(u32::MAX), 600);
        assert_eq!(Backoff::new(0, 0), Some(Backoff::IMMEDIATE));
        assert_eq!(Backoff::IMMEDIATE.seconds(3), 0);
        for (first, ceiling) in [(-1, 10), (11, 10), (10, 601)] {
            assert_eq!(Backoff::new(first, ceiling), None);
        }
    }

    #[test]
    fn a_failure_waits_its_backoff_and_the_last_one_gives_up() {
        let (mut failures, mut after) = (0, 0);
        let first = failed_once(&mut failures, &mut after, 1000, Backoff::HOSTED);
        assert_eq!(first, ("pending", Retry::After(Duration::from_secs(10))));
        assert_eq!((failures, after), (1, 1010));
        failures = MAX_FAILED_ATTEMPTS - 1;
        let last = failed_once(&mut failures, &mut after, 2000, Backoff::HOSTED);
        assert_eq!(last, ("failed", Retry::GaveUp));
        // Failed work keeps its earlier hint; nothing reads it until Try again resets it.
        assert_eq!((failures, after), (MAX_FAILED_ATTEMPTS, 1010));
    }

    #[test]
    fn a_chat_turn_payload_names_only_the_scope_and_input() {
        let delivery = Delivery::chat_turn(
            Scope {
                practice: "p".into(),
                physician: "d".into(),
                workspace: "w".into(),
                chat: "c".into(),
            },
            "k".into(),
        );
        let payload = serde_json::to_string(&delivery).unwrap();
        assert_eq!(
            payload,
            r#"{"scope":{"practice":"p","physician":"d","workspace":"w","chat":"c"},"input_id":"k"}"#
        );
        assert_eq!(
            serde_json::from_str::<Delivery>(&payload).unwrap(),
            delivery
        );
    }
}
