//! Queue wakeups are hints. The conversation row remains the acceptance,
//! completion, lease and recovery owner. No broker acknowledgement substitutes
//! for a committed reply; provider calls themselves are not exactly-once.
use sea_orm::{DbErr, EntityTrait};
use serde::{Deserialize, Serialize};

use super::entities::conversation;
use super::{
    AttemptStatus, Claim, Conversation, RuntimeError, Scope, Store, WorkStatus, error, keys,
};

// Bound wakeup recovery by the 600-second push acknowledgement deadline in
// infra/gcp/rust-environments/queue.tf; zero would make it immediately due again.
const DISPATCH_RETRY_SECONDS: std::ops::RangeInclusive<i64> = 1..=600;

/// Whether a failed attempt still owned the work when settlement was applied.
/// A newer input waiting on the attempt is row state, not part of this result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttemptSettlement {
    /// The attempt was ended with its supplied reason, or as superseded when
    /// it ran an older input than the conversation's current one.
    Applied,
    /// Completion, a hold, new input or another attempt already owns the outcome.
    Superseded,
}

/// Which durable owner a wakeup names.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryKind {
    /// A chat turn in `physician_runtime_conversations`.
    ChatTurn,
    /// A reply revision in `physician_runtime_reply_drafts`; `scope.chat`
    /// names the reply work.
    ReplyRevision,
}

/// Non-clinical broker payload, fenced to the accepted input identity.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Delivery {
    /// The owner this wakeup names.
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

/// One keyset page of recoverable queue wakeups.
#[derive(Debug)]
pub struct DispatchPage {
    /// Due wakeups; publication is intentionally at-least-once.
    pub deliveries: Vec<Delivery>,
    /// Continue here; None ends this sweep.
    pub next: Option<Scope>,
}

impl Store {
    /// Bounded recovery of acceptance/publish crashes and expired worker leases.
    /// Uses the existing pending projection, including its Postgres JSON/clock
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
            if state.dispatch_after <= now
                && (state.status == WorkStatus::Pending
                    || (state.status == WorkStatus::Active
                        && state.attempts.last().is_some_and(|a| a.expires <= now)))
                && let Some(input_id) = state.input_id()
            {
                deliveries.push(Delivery::chat_turn(scope, input_id.to_owned()));
            }
            tx.commit().await?;
        }
        Ok(DispatchPage { deliveries, next })
    }

    /// Records successful broker publication only after its acknowledgement.
    /// If the process dies before this write, the next sweep safely republishes.
    /// A wakeup is periodically recoverable until the authoritative work settles.
    pub async fn dispatched(&self, delivery: &Delivery, retry_after: i64) -> Result<(), DbErr> {
        if !DISPATCH_RETRY_SECONDS.contains(&retry_after) {
            return Err(error("invalid dispatch retry"));
        }
        if delivery.kind == DeliveryKind::ReplyRevision {
            return self.revision_dispatched(delivery, retry_after).await;
        }
        let (tx, mut state, now) = self.locked(&delivery.scope).await?;
        if state.input_id() == Some(&delivery.input_id) {
            state.dispatch_after = now + retry_after;
        }
        Self::save(tx, &delivery.scope, &state).await
    }

    /// Atomically claims only the input named by the wakeup, never a newer turn.
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

    /// End this delivery once using the harness classification. A stale claim,
    /// Stop, or settled turn cannot consume another attempt or reopen the work.
    /// Settlement releases the old claim; a newer input waiting on it becomes
    /// due at once, so the recovery sweep publishes its wakeup.
    pub async fn settle_chat_attempt(
        &self,
        claim: &Claim,
        reason: super::AttemptFailure,
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
        if applied {
            state.end_active_attempt(reason)?;
        }
        if state.revision > claim.revision && state.status == WorkStatus::Pending {
            state.dispatch_after = now;
        }
        Self::save(tx, &claim.scope, &state).await?;
        Ok(if applied {
            AttemptSettlement::Applied
        } else {
            AttemptSettlement::Superseded
        })
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

    /// Where the input named by a wakeup stands, read under the conversation
    /// lock. A worker uses it when its claim found nothing to run.
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

/// The authoritative answer to "what should the broker do with this wakeup?".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeliveryState {
    /// Committed, held by Stop, or superseded by a newer accepted input.
    /// Acknowledge: nothing is left for this message to do.
    Settled,
    /// A live claim is running this input. Acknowledge: if that worker dies,
    /// its lease expires and the recovery sweep republishes the input.
    Claimed,
    /// Still runnable (pending, or a lease that has run out). Do not
    /// acknowledge, so the broker redelivers it.
    Due,
    /// Recovery gave up after [`harness::engine::MAX_FAILED_ATTEMPTS`]. Do not acknowledge, so
    /// the broker forwards the message to its dead-letter topic. Application
    /// attempts are not broker deliveries: a message republished under a new
    /// ID still has its own delivery count to exhaust. Try again resumes it.
    Failed,
}
