//! Queue wakeups are hints. The conversation row remains the acceptance,
//! completion, lease and recovery owner. No broker acknowledgement substitutes
//! for a committed reply; provider calls themselves are not exactly-once.
use sea_orm::{DbErr, EntityTrait};
use serde::{Deserialize, Serialize};

use super::entities::conversation;
use super::{Claim, Conversation, MAX_FAILED_ATTEMPTS, Scope, Store, error, keys};

/// Which durable owner a wakeup names.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
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
    fn is_chat_turn(&self) -> bool {
        *self == Self::ChatTurn
    }
}

/// Non-clinical broker payload, fenced to the accepted input identity.
///
/// A chat turn omits `kind`, so payloads stay byte-identical to the ones
/// published before revisions existed, and both old and new workers read them.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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
                && state.session_hash.is_some()
                && (state.status == "pending"
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

    /// Records successful broker publication only after its acknowledgement.
    /// If the process dies before this write, the next sweep safely republishes.
    /// A wakeup is periodically recoverable until the authoritative work settles.
    pub async fn dispatched(&self, delivery: &Delivery, retry_after: i64) -> Result<(), DbErr> {
        if !(1..=600).contains(&retry_after) {
            return Err(error("invalid dispatch retry"));
        }
        if delivery.kind == DeliveryKind::ReplyRevision {
            return self.revision_dispatched(delivery, retry_after).await;
        }
        let (tx, mut state, now) = self.locked(&delivery.scope).await?;
        if state.input_id.as_deref() == Some(&delivery.input_id) {
            state.dispatch_after = now + retry_after;
        }
        Self::save(tx, &delivery.scope, &state).await
    }

    /// Atomically claims only the input named by the wakeup, never a newer turn.
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

    /// Release this delivery's unsuccessful attempt for broker retry. Never
    /// releases Stop, changes a successor, or reopens completed or failed
    /// work: a reached bound has already ended the turn, and only Try again
    /// reopens it. The attempt is recorded as failed; an expired lease is
    /// recorded as interrupted by the next claim instead. The
    /// [`MAX_FAILED_ATTEMPTS`]th failure ends the work as failed, so neither
    /// redelivery nor the recovery sweep starts it again.
    pub async fn retry_delivery(&self, claim: &Claim) -> Result<(), DbErr> {
        let (tx, mut state, _) = self.locked(&claim.scope).await?;
        if state.revision == claim.revision
            && state.status == "active"
            && let Some(attempt) = state.attempts.last_mut().filter(|a| a.id == claim.attempt)
        {
            if attempt.status == "active" {
                attempt.status = "failed".into();
            }
            state.failures += 1;
            state.status = if state.failures >= MAX_FAILED_ATTEMPTS {
                "failed"
            } else {
                "pending"
            }
            .into();
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

    /// Where the input named by a wakeup stands, read under the conversation
    /// lock. A worker uses it when its claim found nothing to run.
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
    /// Recovery gave up after [`MAX_FAILED_ATTEMPTS`]. Do not acknowledge, so
    /// the broker forwards the message to its dead-letter topic. Application
    /// attempts are not broker deliveries: a message republished under a new
    /// ID still has its own delivery count to exhaust. Try again resumes it.
    Failed,
}
