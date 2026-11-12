//! Hosted work is run from the rows that own it. The conversation row (and
//! the reply-draft row) is the acceptance, completion, lease, retry and
//! recovery owner. A `NOTIFY` on [`WORK_CHANNEL`], sent in the transaction
//! that makes work runnable, only tells a listening worker sooner than its
//! next poll; a lost notification delays work and never loses it. Provider
//! calls themselves are not exactly-once.
use std::time::Duration;

use harness::engine::MAX_FAILED_ATTEMPTS;
use sea_orm::sea_query::{Alias, Expr, ExprTrait, Func, IntoTableRef, Query, SelectStatement};
use sea_orm::sqlx::postgres::PgListener;
use sea_orm::{ColumnTrait, ConnectionTrait, DatabaseTransaction, DbErr, EntityTrait, RuntimeErr};
use serde::{Deserialize, Serialize};

use super::entities::{chat_attempt, conversation, revision_attempt};
use super::{
    AttemptStatus, Claim, Conversation, RuntimeError, Scope, Store, WorkStatus, database_clock,
    error, keys,
};

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
        let doublings = Ord::min(failures.saturating_sub(1), 16);
        Ord::min(self.first.saturating_mul(1_i64 << doublings), self.ceiling)
    }
}

/// What a counted failure left behind. The work's stored status follows
/// from it through [`Self::work_status`] and [`Self::draft_status`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AfterFailure {
    /// The work is pending again and due after this long.
    RetryAt(Duration),
    /// This was the [`MAX_FAILED_ATTEMPTS`]th failure: the work ended as
    /// failed and only Try again reopens it.
    GaveUp,
}

impl AfterFailure {
    /// The conversation status this outcome leaves.
    pub fn work_status(self) -> WorkStatus {
        match self {
            Self::RetryAt(_) => WorkStatus::Pending,
            Self::GaveUp => WorkStatus::Failed,
        }
    }

    /// The reply draft status this outcome leaves.
    pub fn draft_status(self) -> super::reply::DraftStatus {
        match self {
            Self::RetryAt(_) => super::reply::DraftStatus::Pending,
            Self::GaveUp => super::reply::DraftStatus::Failed,
        }
    }
}

/// The work row's fields a counted failure moves: how many attempts failed
/// in the current run, already including this one, and when it is due.
pub(super) struct RetryClock<'a> {
    pub failures: u32,
    pub dispatch_after: &'a mut i64,
}

/// Schedules the next attempt after a counted failure, or gives up at the
/// [`MAX_FAILED_ATTEMPTS`]th, the same bound the shared run rule ends on.
pub(super) fn failed_once(clock: RetryClock<'_>, now: i64, backoff: Backoff) -> AfterFailure {
    if clock.failures >= MAX_FAILED_ATTEMPTS {
        return AfterFailure::GaveUp;
    }
    let seconds = backoff.seconds(clock.failures);
    *clock.dispatch_after = now + seconds;
    AfterFailure::RetryAt(Duration::from_secs(seconds.unsigned_abs()))
}

/// Tells listening workers, when `tx` commits, that `delivery` is runnable.
/// The payload is the delivery identity only. One too large for Postgres to
/// carry is left to the workers' poll.
pub(super) async fn notify(tx: &DatabaseTransaction, delivery: &Delivery) -> Result<(), DbErr> {
    let payload = serde_json::to_string(delivery)
        .map_err(super::conversion_error("Delivery", "notification payload"))?;
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

/// Seconds until the soonest live lease among one kind's attempts runs out,
/// by the database clock; null when no lease is live.
fn soonest_lease_end<C: ColumnTrait + Copy>(
    table: impl IntoTableRef,
    status: C,
    expires: C,
) -> SelectStatement {
    Query::select()
        .expr_as(
            Expr::from(Func::min(Expr::col(expires))).sub(database_clock()),
            Alias::new("left"),
        )
        .from(table)
        .and_where(status.eq(AttemptStatus::Active))
        .and_where(Expr::col(expires).gt(database_clock()))
        .to_owned()
}

fn connection(source: sea_orm::sqlx::Error) -> DbErr {
    DbErr::Conn(RuntimeErr::SqlxError(source.into()))
}

/// Whether a failed attempt still owned the work when settlement was applied.
/// A newer input waiting on the attempt is row state, not part of this result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttemptSettlement {
    /// The attempt was ended with its supplied reason, or as superseded when
    /// it ran an older input than the work's current one, and that reason
    /// does not count as a failure: an interruption or a hold.
    Applied,
    /// The attempt was ended as a counted failure, with what that left.
    Failed(AfterFailure),
    /// Completion, a hold, new input or another attempt already owns the outcome.
    Superseded,
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

    /// One bounded page of due chat turns: pending work whose retry delay
    /// has passed, and attempts whose lease ran out.
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
            // A steer waits for its draining predecessor; settling that
            // attempt tells listening workers.
            let lease_held = state.attempts.last().is_some_and(|a| a.owns_lease(now));
            let due = match state.status {
                WorkStatus::Pending => state.dispatch_after <= now && !lease_held,
                WorkStatus::Active => !lease_held,
                WorkStatus::Completed
                | WorkStatus::Failed
                | WorkStatus::Held
                | WorkStatus::Cancelled => false,
            };
            if due && let Some(input_id) = state.input_id() {
                deliveries.push(Delivery::chat_turn(scope, input_id.to_owned()));
            }
            tx.commit().await?;
        }
        Ok(DispatchPage { deliveries, next })
    }

    /// How long until the soonest live claim on a chat turn or reply
    /// revision runs out, when one is held. A worker that died leaves no
    /// notification, so its work becomes due only then. Leases already run
    /// out are left to the poll.
    pub async fn soonest_lease_end(&self) -> Result<Option<Duration>, DbErr> {
        let queries = [
            soonest_lease_end(
                chat_attempt::Entity,
                chat_attempt::Column::Status,
                chat_attempt::Column::Expires,
            ),
            soonest_lease_end(
                revision_attempt::Entity,
                revision_attempt::Column::Status,
                revision_attempt::Column::Expires,
            ),
        ];
        let mut soonest: Option<i64> = None;
        for query in queries {
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
    ) -> Result<Option<Claim>, RuntimeError> {
        if delivery.kind != DeliveryKind::ChatTurn {
            return Err(error("not a chat turn delivery").into());
        }
        self.claim_inner(&delivery.scope, lease_seconds, Some(&delivery.input_id))
            .await
    }

    /// End this attempt once using the harness classification. A stale
    /// claim, Stop, or settled turn cannot consume another attempt or reopen
    /// the work. A counted failure makes the work due again after `backoff`,
    /// or ends it as failed at the [`MAX_FAILED_ATTEMPTS`]th. An interrupted
    /// attempt, such as one whose worker is stopping, hands the work back at
    /// once. Whenever settlement leaves the work due now, including a newer
    /// input that waited on this attempt, listening workers are told.
    pub async fn settle_chat_attempt(
        &self,
        claim: &Claim,
        reason: super::AttemptFailure,
        backoff: Backoff,
    ) -> Result<AttemptSettlement, RuntimeError> {
        let (tx, mut state, now) = self.locked(&claim.scope).await?;
        let Some(attempt) = state.attempts.last_mut().filter(|a| a.id == claim.attempt) else {
            tx.rollback().await?;
            return Ok(AttemptSettlement::Superseded);
        };
        // Work may have been steered or stopped while its tool calls drained.
        // Settlement releases their claim without changing the newer direction.
        attempt.expires = now;
        let applied = state.status.running()
            && state
                .latest_work_attempt()
                .is_some_and(|attempt| attempt.status == AttemptStatus::Active);
        let mut settlement = AttemptSettlement::Superseded;
        if applied {
            let before = state.failures;
            state.end_active_attempt(reason)?;
            settlement = if state.failures > before {
                let clock = RetryClock {
                    failures: state.failures,
                    dispatch_after: &mut state.dispatch_after,
                };
                let after = failed_once(clock, now, backoff);
                // The shared run rule ended the run at the same bound; the
                // stored status is the one this outcome names.
                state.status = after.work_status();
                AttemptSettlement::Failed(after)
            } else {
                AttemptSettlement::Applied
            };
        }
        if state.status == WorkStatus::Pending
            && state.dispatch_after <= now
            && let Some(input_id) = state.input_id()
        {
            notify(
                &tx,
                &Delivery::chat_turn(claim.scope.clone(), input_id.to_owned()),
            )
            .await?;
        }
        Self::save(tx, &claim.scope, &state).await?;
        Ok(settlement)
    }

    /// Unlocked display read: never contends with a claim or publication lock.
    /// `None` before the first accepted input. Not a fence for any decision.
    pub async fn read(&self, scope: &Scope) -> Result<Option<Conversation>, DbErr> {
        let tx = self.snapshot().await?;
        let Some(row) = conversation::Entity::find_by_id(keys(scope))
            .one(&tx)
            .await?
        else {
            tx.commit().await?;
            return Ok(None);
        };
        let mut state = Self::conversation_control(&tx, scope, row).await?;
        Self::conversation_events(&tx, scope, &mut state).await?;
        tx.commit().await?;
        Ok(Some(state))
    }

    /// Where the input named by a delivery stands, read under the
    /// conversation lock. A worker uses it when its claim found nothing to run.
    pub async fn delivery_state(&self, delivery: &Delivery) -> Result<DeliveryState, DbErr> {
        if delivery.kind == DeliveryKind::ReplyRevision {
            return self.revision_state(delivery).await;
        }
        let (tx, state, now) = self.locked(&delivery.scope).await?;
        tx.commit().await?;
        if state.input_id() != Some(&delivery.input_id) {
            return Ok(DeliveryState::Settled);
        }
        Ok(match state.status {
            WorkStatus::Completed | WorkStatus::Held => DeliveryState::Settled,
            WorkStatus::Failed => DeliveryState::Failed,
            WorkStatus::Active if state.attempts.last().is_some_and(|a| a.owns_lease(now)) => {
                DeliveryState::Claimed
            }
            WorkStatus::Active | WorkStatus::Pending | WorkStatus::Cancelled => DeliveryState::Due,
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
        let mut after = 0;
        let first = failed_once(
            RetryClock {
                failures: 1,
                dispatch_after: &mut after,
            },
            1000,
            Backoff::HOSTED,
        );
        assert_eq!(first, AfterFailure::RetryAt(Duration::from_secs(10)));
        assert_eq!(first.work_status(), WorkStatus::Pending);
        assert_eq!(after, 1010);
        let last = failed_once(
            RetryClock {
                failures: MAX_FAILED_ATTEMPTS,
                dispatch_after: &mut after,
            },
            2000,
            Backoff::HOSTED,
        );
        assert_eq!(last, AfterFailure::GaveUp);
        assert_eq!(last.work_status(), WorkStatus::Failed);
        assert_eq!(
            last.draft_status(),
            super::super::reply::DraftStatus::Failed
        );
        // Failed work keeps its earlier hint; nothing reads it until Try again resets it.
        assert_eq!(after, 1010);
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
