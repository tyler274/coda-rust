//! No-tools Duet adapter over the approved neutral engine. Not composed into HTTP
//! or hosted startup. Qualification, instructions/context and live provider
//! admission remain unavailable; no production execution permit constructor.
use front_door::{
    PostgresFrontDoorStore,
    conversation::execution::{Claim, StoredEvent},
};
use harness::engine::{
    self, Admission, Cancellation, Context, Event, EventKind, Host, Invocation, Item, Limits,
    Provider, Usage,
};
use serde_json::Value;
use std::{sync::Arc, time::Duration};

/// Intentionally unconstructible outside this module's controlled tests. Not a
/// substitute for the future transactionally qualified admission owner.
pub struct ExecutionPermit {
    _private: (),
}
struct Owner {
    store: Arc<PostgresFrontDoorStore>,
    claim: Claim,
    context: Context,
}
#[async_trait::async_trait]
impl Host for Owner {
    async fn prepare(&mut self, transcript: &[Item], _: &Usage) -> Result<Context, String> {
        let mut context = self.context.clone();
        context.items.extend_from_slice(transcript);
        Ok(context)
    }
    async fn assert_current(&mut self) -> Result<(), String> {
        self.store
            .assert_conversation_claim(&self.claim)
            .await
            .map_err(|_| "Duet claim unavailable".into())
    }
    async fn persist(&mut self, event: Event) -> Result<(), String> {
        if event.attempt_id != self.claim.attempt_id().to_string() {
            return Err("Attempt mismatch".into());
        }
        let delta = match &event.kind {
            EventKind::TextDelta { delta, .. } => Some(delta.text.clone()),
            _ => None,
        };
        let candidate = match &event.kind {
            EventKind::Candidate { text } => Some(text.clone()),
            _ => None,
        };
        let body = serde_json::to_string(&event).map_err(|_| "Invalid event")?;
        self.store
            .acknowledge_conversation_event(
                &self.claim,
                &StoredEvent {
                    sequence: i64::try_from(event.sequence).map_err(|_| "Invalid sequence")?,
                    body,
                    delta,
                    candidate,
                },
            )
            .await
            .map_err(|_| "Duet event acknowledgment unavailable".into())
    }
    async fn admit(&mut self, _: &Invocation, _: &Value) -> Result<Admission, String> {
        Ok(Admission::Deny(
            "No Duet effects admitted in this slice".into(),
        ))
    }
    async fn execute(&mut self, _: &Invocation, _: Value) -> Result<String, String> {
        Err("No effect executor".into())
    }
}
/// Execute one claimed no-tools attempt, then commit the acknowledged candidate
/// through Duet's fence. Failure leaves the lease for bounded durable recovery;
/// cancellation never implies rollback or intentional patient Stop.
/// Context/provider must eventually come from the qualified Duet owner, not HTTP.
pub async fn execute(
    store: Arc<PostgresFrontDoorStore>,
    claim: Claim,
    context: Context,
    provider: &mut dyn Provider,
    cancel: &Cancellation,
    _permit: &ExecutionPermit,
) -> Result<(), String> {
    if !context.tools.is_empty() {
        return Err("Tools not admitted".into());
    }
    store
        .begin_conversation_execution(&claim)
        .await
        .map_err(|_| "Duet attempt already started or unavailable")?;
    let mut owner = Owner {
        store: store.clone(),
        claim: claim.clone(),
        context,
    };
    let limits = Limits {
        model_calls: 1,
        tool_calls: 0,
        calls_per_turn: 1,
        request_bytes: 65536,
        response_bytes: 131072,
        tool_result_bytes: 4096,
        total_tool_result_bytes: 4096,
        output_tokens: 8192,
        elapsed: Duration::from_secs(660),
    };
    let candidate = engine::run(
        provider,
        &mut owner,
        &claim.attempt_id().to_string(),
        Vec::new(),
        &limits,
        cancel,
    )
    .await
    .map_err(|_| "Duet execution incomplete")?;
    store
        .complete_conversation(&claim, &candidate.text)
        .await
        .map_err(|_| "Duet completion unavailable".into())
}
#[cfg(test)]
mod tests;
