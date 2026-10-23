//! Queue wakeups are hints. The conversation row remains the acceptance,
//! completion, lease and recovery owner. No broker acknowledgement substitutes
//! for a committed reply; provider calls themselves are not exactly-once.
use super::*;

/// Non-clinical broker payload, fenced to the accepted input identity.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Delivery {
    /// Complete product-owned authorization scope, not user-supplied authority.
    pub scope: Scope,
    /// Idempotency identity of the latest accepted input.
    pub input_id: String,
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
                deliveries.push(Delivery { scope, input_id });
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
    /// releases Stop, changes a successor, or reopens completed work.
    pub async fn retry_delivery(&self, claim: &Claim) -> Result<(), DbErr> {
        let (tx, mut state, _) = self.locked(&claim.scope).await?;
        if state.revision == claim.revision
            && matches!(state.status.as_str(), "active" | "failed")
            && let Some(attempt) = state.attempts.last_mut().filter(|a| a.id == claim.attempt)
        {
            if attempt.status == "active" {
                attempt.status = "interrupted".into();
            }
            state.status = "pending".into();
        }
        Self::save(tx, &claim.scope, &state).await
    }

    /// True only when this input is settled or superseded. In-flight and failed
    /// work must not be acknowledged: that could lose the sole crash redelivery.
    pub async fn delivery_settled(&self, delivery: &Delivery) -> Result<bool, DbErr> {
        let state = self.conversation(&delivery.scope).await?;
        Ok(state.input_id.as_deref() != Some(&delivery.input_id)
            || matches!(state.status.as_str(), "completed" | "held"))
    }
}
