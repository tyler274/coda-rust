//! One bounded execution loop. Hosts own durable truth and every effect.
//!
//! There is no retry or recovery loop here. In particular, dropping a tool
//! future does not establish that its effect was rolled back. After any failure
//! following `ToolStarted`, the host reconciles the original operation before
//! deciding whether to resume. Persisted events are execution evidence, not a
//! product lifecycle. A candidate must still pass the product's commit fence.

use std::{collections::HashSet, future::Future, time::Duration};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::{sync::watch, time::Instant};

/// A cooperative stop signal, independent of any product terminal state.
#[derive(Clone, Debug)]
pub struct Cancellation(watch::Sender<bool>);

impl Default for Cancellation {
    fn default() -> Self {
        Self(watch::channel(false).0)
    }
}

impl Cancellation {
    /// Requests cancellation. An already committed effect is not undone.
    pub fn cancel(&self) {
        self.0.send_replace(true);
    }

    async fn cancelled(&self) {
        let mut rx = self.0.subscribe();
        let _ = rx.wait_for(|cancelled| *cancelled).await;
    }
}

/// Provider-neutral conversation item. Opaque items retain provider continuation
/// data, never authority to execute a tool.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Item {
    /// Authored message; input roles are user, assistant or developer.
    Message {
        /// Wire role.
        role: String,
        /// Authored content.
        text: String,
    },
    /// Provider-requested tool call.
    Call(Invocation),
    /// Exactly one result for a call.
    Result {
        /// Matching call identity.
        call_id: String,
        /// Result bytes interpreted by the model, not the engine.
        output: String,
    },
    /// Provider-specific continuation item, for example reasoning metadata.
    Opaque(Value),
}

/// An invocation identity is unique within an execution attempt. Products supply
/// their own durable effect identity; a provider call ID is not an idempotency key.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Invocation {
    /// Provider call identity.
    pub id: String,
    /// Tool name.
    pub name: String,
    /// Raw JSON arguments. Invalid JSON produces feedback, not an invocation.
    pub arguments: String,
}

/// A tool offered by the product, not inferred from a skill or provider response.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tool {
    /// Unique callable name.
    pub name: String,
    /// Model-facing description.
    pub description: String,
    /// JSON schema. The host validates product-specific argument semantics.
    pub parameters: Value,
}

/// One product-prepared context. Selection and preservation policy stay outside
/// the engine; the engine checks complete call/result pairing before dispatch.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    /// Product-owned instructions.
    pub instructions: String,
    /// Ordered conversation and provider continuation items.
    pub items: Vec<Item>,
    /// Tools callable on this request.
    pub tools: Vec<Tool>,
}

/// Owner-selected bounds for one attempt; hosts retain cross-attempt usage.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Limits {
    /// Maximum dispatched provider requests.
    pub model_calls: u32,
    /// Maximum executed tools across all turns.
    pub tool_calls: u32,
    /// Maximum calls in a single provider response; oversized batches run none.
    pub calls_per_turn: usize,
    /// Maximum serialized provider request bytes.
    pub request_bytes: usize,
    /// Maximum buffered provider response bytes.
    pub response_bytes: usize,
    /// Maximum single tool result bytes.
    pub tool_result_bytes: usize,
    /// Maximum aggregate delivered tool result bytes.
    pub total_tool_result_bytes: usize,
    /// Provider output-token ceiling.
    pub output_tokens: u32,
    /// Whole attempt deadline, including host callbacks.
    pub elapsed: Duration,
}

/// Attempt-local accounting; durable owners aggregate across attempts.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Usage {
    /// Provider requests dispatched, including uncertain outcomes.
    pub model_calls: u32,
    /// Tools dispatched, including uncertain outcomes.
    pub tool_calls: u32,
    /// Tool-result bytes delivered to subsequent context.
    pub tool_result_bytes: usize,
}

/// Immutable request information passed to the provider.
pub struct Request<'a> {
    /// Owner-provided attempt identity, not a new engine-owned run identity.
    pub attempt_id: &'a str,
    /// One-based provider request ordinal.
    pub ordinal: u32,
    /// Product context after engine bound checks.
    pub context: &'a Context,
    /// Execution limits, including the response byte cap.
    pub limits: &'a Limits,
}

/// A completed provider exchange; neither text nor tool calls are committed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Turn {
    /// Ordered output, including opaque continuation items.
    pub items: Vec<Item>,
    /// Provider-owned usage/response evidence for the host's record.
    pub receipt: Value,
}

/// Adapter mechanics only. No automatic retry or product authority.
#[async_trait]
pub trait Provider: Send {
    /// Exact serialized request size, including adapter-specific framing.
    fn request_bytes(&self, request: &Request<'_>) -> Result<usize, String>;
    /// Executes one request, honoring response bounds. No transparent retries.
    async fn complete(&mut self, request: Request<'_>) -> Result<Turn, String>;
}

/// Execution evidence awaited before dependent operations. Sequence numbers are
/// scoped to the owner-provided attempt, not a new durable state machine.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    /// Owner-provided attempt identity.
    pub attempt_id: String,
    /// Monotonic, one-based event ordinal within this invocation of `run`.
    pub sequence: u64,
    /// Evidence to persist through the product owner.
    pub kind: EventKind,
}

/// Neutral event payloads. The product decides retention and redaction.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventKind {
    /// Must be acknowledged before dispatch; owner may reserve durable budget.
    ProviderStarted {
        /// Request ordinal.
        ordinal: u32,
        /// Exact request context.
        context: Context,
    },
    /// Must be acknowledged before interpreting calls or returning a candidate.
    ProviderReturned(Turn),
    /// Must be acknowledged before an effect; not proof that it happened.
    ToolStarted(Invocation),
    /// Includes rejected calls and results whose delivery was refused by bounds.
    ToolReturned {
        /// Original call identity.
        call_id: String,
        /// Exact result, before any bounded model-visible replacement.
        output: String,
        /// Whether the callback executed (not whether it mutated product state).
        executed: bool,
    },
    /// Candidate evidence only; never a product completion event.
    Candidate {
        /// Candidate authored text across this attempt.
        text: String,
    },
}

/// Explicit product tool admission after mechanical protocol checks.
pub enum Admission {
    /// The host permits this operation now; mutation still needs its own fence.
    Allow,
    /// Return this reason to the model without executing the operation.
    Deny(String),
}

/// Product-owned connectors. Implementations must not rely on callback cancellation
/// to undo writes. `persist` succeeds only after durable acknowledgment. The host
/// owns operation reconciliation, transactions, leases and cross-attempt budgets.
#[async_trait]
pub trait Host: Send {
    /// Supplies context/preservation policy, using retained execution items.
    async fn prepare(&mut self, transcript: &[Item], usage: &Usage) -> Result<Context, String>;
    /// Checks the current owner authority before provider/tool dispatch.
    async fn assert_current(&mut self) -> Result<(), String>;
    /// Durably records evidence/reservations; rejection stops dependent execution.
    async fn persist(&mut self, event: Event) -> Result<(), String>;
    /// Validates product schema, permission and allowed effects for this call.
    async fn admit(&mut self, call: &Invocation, arguments: &Value) -> Result<Admission, String>;
    /// Runs one admitted tool. Enforce currentness transactionally for mutations.
    /// An error is an uncertain outcome, not a safe-to-retry tool error.
    /// Known refusals/errors should be returned as model-visible `Ok` feedback.
    async fn execute(&mut self, call: &Invocation, arguments: Value) -> Result<String, String>;
}

/// A candidate that must still pass the product's publication/commit fence.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Candidate {
    /// All authored assistant text in this invocation, including pre-tool prose.
    pub text: String,
    /// Retained complete exchange items; host decides durable context policy.
    pub transcript: Vec<Item>,
    /// Attempt-local usage (not an account allowance record).
    pub usage: Usage,
}

/// Why execution stopped, without deciding a product terminal state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Failure {
    /// Cooperative cancellation.
    Cancelled,
    /// Whole-attempt deadline expired.
    Deadline,
    /// An owner-selected bound was reached.
    Limit(String),
    /// Provider or context protocol integrity failed.
    Protocol(String),
    /// A provider failure; may have dispatched and consumed provider usage.
    Provider(String),
    /// Product callback failure; durable writes may need reconciliation.
    Host(String),
}

/// Failure plus diagnostic boundary/usage, so the owner can reconcile before retry.
#[derive(Clone, Debug)]
pub struct ExecutionError {
    /// Failure category.
    pub failure: Failure,
    /// Last boundary entered. This is diagnostic, not a durable lifecycle state.
    pub boundary: &'static str,
    /// Dispatched work counts retained even after cancellation or failure.
    pub usage: Usage,
}

struct Gate<'a> {
    cancel: &'a Cancellation,
    deadline: Instant,
    boundary: &'static str,
    usage: Usage,
}

impl Gate<'_> {
    async fn wait<T>(
        &mut self,
        boundary: &'static str,
        future: impl Future<Output = Result<T, Failure>>,
    ) -> Result<T, Failure> {
        self.boundary = boundary;
        tokio::select! {
            biased;
            () = self.cancel.cancelled() => Err(Failure::Cancelled),
            () = tokio::time::sleep_until(self.deadline) => Err(Failure::Deadline),
            value = future => value,
        }
    }
}

/// Executes one bounded attempt. No automatic retries, durable claims or product
/// commits. The caller must provide a fresh attempt identity for each invocation
/// and reconcile prior uncertain operations before invoking again.
pub async fn run(
    provider: &mut dyn Provider,
    host: &mut dyn Host,
    attempt_id: &str,
    initial: Vec<Item>,
    limits: &Limits,
    cancel: &Cancellation,
) -> Result<Candidate, ExecutionError> {
    let deadline = Instant::now()
        .checked_add(limits.elapsed)
        .ok_or_else(|| ExecutionError {
            failure: Failure::Limit("deadline cannot be represented".into()),
            boundary: "configuration",
            usage: Usage::default(),
        })?;
    let mut gate = Gate {
        cancel,
        deadline,
        boundary: "configuration",
        usage: Usage::default(),
    };
    let result = execute(provider, host, attempt_id, initial, limits, &mut gate).await;
    result.map_err(|failure| ExecutionError {
        failure,
        boundary: gate.boundary,
        usage: gate.usage,
    })
}

async fn record(
    gate: &mut Gate<'_>,
    host: &mut dyn Host,
    attempt: &str,
    sequence: &mut u64,
    kind: EventKind,
) -> Result<(), Failure> {
    *sequence += 1;
    gate.wait("event acknowledgment", async {
        host.persist(Event {
            attempt_id: attempt.to_owned(),
            sequence: *sequence,
            kind,
        })
        .await
        .map_err(Failure::Host)
    })
    .await
}

async fn current(gate: &mut Gate<'_>, host: &mut dyn Host) -> Result<(), Failure> {
    gate.wait("currentness", async {
        host.assert_current().await.map_err(Failure::Host)
    })
    .await
}

async fn execute(
    provider: &mut dyn Provider,
    host: &mut dyn Host,
    attempt: &str,
    mut transcript: Vec<Item>,
    limits: &Limits,
    gate: &mut Gate<'_>,
) -> Result<Candidate, Failure> {
    if attempt.trim().is_empty()
        || limits.model_calls == 0
        || limits.calls_per_turn == 0
        || limits.request_bytes == 0
        || limits.response_bytes == 0
        || limits.output_tokens == 0
        || limits.tool_result_bytes == 0
        || limits.total_tool_result_bytes == 0
        || limits.elapsed.is_zero()
    {
        return Err(Failure::Limit(
            "invalid execution bounds or attempt identity".into(),
        ));
    }
    validate_context(&transcript)?;
    let mut seen: HashSet<String> = transcript
        .iter()
        .filter_map(|item| match item {
            Item::Call(call) => Some(call.id.clone()),
            _ => None,
        })
        .collect();
    let mut sequence = 0;
    let mut text = String::new();
    loop {
        if gate.usage.model_calls >= limits.model_calls {
            return Err(Failure::Limit("model calls".into()));
        }
        current(gate, host).await?;
        let usage = gate.usage.clone();
        let mut context = gate
            .wait("context preparation", async {
                host.prepare(&transcript, &usage)
                    .await
                    .map_err(Failure::Host)
            })
            .await?;
        if gate.usage.tool_calls >= limits.tool_calls {
            context.tools.clear();
        }
        validate_context(&context.items)?;
        // Context selected by the owner may restore earlier exchanges. Their IDs
        // must also be protected against a new provider invocation.
        for item in &context.items {
            if let Item::Call(call) = item {
                seen.insert(call.id.clone());
            }
        }
        let mut names = HashSet::new();
        if context
            .tools
            .iter()
            .any(|tool| tool.name.trim().is_empty() || !names.insert(tool.name.clone()))
        {
            return Err(Failure::Protocol(
                "empty or repeated offered tool name".into(),
            ));
        }
        let ordinal = gate.usage.model_calls + 1;
        let request = || Request {
            attempt_id: attempt,
            ordinal,
            context: &context,
            limits,
        };
        if provider
            .request_bytes(&request())
            .map_err(Failure::Provider)?
            > limits.request_bytes
        {
            return Err(Failure::Limit(
                "request bytes; owner must preserve context".into(),
            ));
        }
        record(
            gate,
            host,
            attempt,
            &mut sequence,
            EventKind::ProviderStarted {
                ordinal,
                context: context.clone(),
            },
        )
        .await?;
        current(gate, host).await?;
        gate.usage.model_calls += 1;
        let turn = gate
            .wait("provider dispatch", async {
                provider
                    .complete(request())
                    .await
                    .map_err(Failure::Provider)
            })
            .await?;
        record(
            gate,
            host,
            attempt,
            &mut sequence,
            EventKind::ProviderReturned(turn.clone()),
        )
        .await?;
        let calls: Vec<_> = turn
            .items
            .iter()
            .filter_map(|item| match item {
                Item::Call(call) => Some(call.clone()),
                _ => None,
            })
            .collect();
        // Validate the entire batch before invoking any callback with effects.
        for item in &turn.items {
            match item {
                Item::Result { .. } => {
                    return Err(Failure::Protocol("provider authored a tool result".into()));
                }
                Item::Message { role, .. } if role != "assistant" => {
                    return Err(Failure::Protocol(
                        "provider authored a non-assistant message".into(),
                    ));
                }
                _ => (),
            }
        }
        for call in &calls {
            if call.id.trim().is_empty()
                || call.name.trim().is_empty()
                || !seen.insert(call.id.clone())
            {
                return Err(Failure::Protocol(
                    "empty or repeated tool call identity/name".into(),
                ));
            }
        }
        if calls.len() > limits.calls_per_turn {
            return Err(Failure::Limit("tool batch; no calls executed".into()));
        }
        for item in &turn.items {
            if let Item::Message { text: part, .. } = item {
                text.push_str(part);
            }
        }
        transcript = context.items;
        transcript.extend(turn.items);
        if calls.is_empty() {
            if text.trim().is_empty() {
                return Err(Failure::Protocol("no authored completion".into()));
            }
            current(gate, host).await?;
            record(
                gate,
                host,
                attempt,
                &mut sequence,
                EventKind::Candidate { text: text.clone() },
            )
            .await?;
            return Ok(Candidate {
                text,
                transcript,
                usage: gate.usage.clone(),
            });
        }
        for call in calls {
            let arguments = serde_json::from_str::<Value>(&call.arguments);
            let denial = if !names.contains(&call.name) {
                Some("unknown or unoffered tool".to_owned())
            } else if arguments.as_ref().is_err() || !arguments.as_ref().is_ok_and(Value::is_object)
            {
                Some("arguments must be a JSON object".to_owned())
            } else if gate.usage.tool_calls >= limits.tool_calls {
                Some("tool execution allowance exhausted".to_owned())
            } else if gate.usage.tool_result_bytes >= limits.total_tool_result_bytes {
                return Err(Failure::Limit(
                    "tool result allowance exhausted before dispatch".into(),
                ));
            } else {
                current(gate, host).await?;
                match gate
                    .wait("tool admission", async {
                        host.admit(&call, arguments.as_ref().expect("validated arguments"))
                            .await
                            .map_err(Failure::Host)
                    })
                    .await?
                {
                    Admission::Allow => None,
                    Admission::Deny(reason) => Some(reason),
                }
            };
            let executed = denial.is_none();
            let output = if let Some(reason) = denial {
                serde_json::json!({"executed": false, "error": reason}).to_string()
            } else {
                record(
                    gate,
                    host,
                    attempt,
                    &mut sequence,
                    EventKind::ToolStarted(call.clone()),
                )
                .await?;
                current(gate, host).await?;
                gate.usage.tool_calls += 1;
                gate.wait("tool dispatch; reconcile uncertain effects", async {
                    host.execute(&call, arguments.expect("validated arguments"))
                        .await
                        .map_err(Failure::Host)
                })
                .await?
            };
            record(
                gate,
                host,
                attempt,
                &mut sequence,
                EventKind::ToolReturned {
                    call_id: call.id.clone(),
                    output: output.clone(),
                    executed,
                },
            )
            .await?;
            // Never replay an effect to obtain a smaller result. Its exact output
            // is already acknowledged by the owner before refusing delivery.
            if output.len() > limits.tool_result_bytes
                || output.len()
                    > limits
                        .total_tool_result_bytes
                        .saturating_sub(gate.usage.tool_result_bytes)
            {
                return Err(Failure::Limit(
                    "tool result bytes; result retained by owner".into(),
                ));
            }
            gate.usage.tool_result_bytes += output.len();
            transcript.push(Item::Result {
                call_id: call.id,
                output,
            });
        }
    }
}

/// Checks complete, unique call/result pairing; context preservation must never
/// create orphan results, duplicate identities or split an unfinished exchange.
pub fn validate_context(items: &[Item]) -> Result<(), Failure> {
    let mut seen = HashSet::new();
    let mut pending = HashSet::new();
    for item in items {
        match item {
            Item::Call(call) => {
                if call.id.trim().is_empty() || !seen.insert(call.id.as_str()) {
                    return Err(Failure::Protocol(
                        "duplicate/empty context call identity".into(),
                    ));
                }
                pending.insert(call.id.as_str());
            }
            Item::Result { call_id, .. } => {
                if !pending.remove(call_id.as_str()) {
                    return Err(Failure::Protocol("orphan/duplicate context result".into()));
                }
            }
            Item::Message { role, .. } => {
                if (!pending.is_empty() && role != "assistant")
                    || !matches!(role.as_str(), "user" | "assistant" | "developer")
                {
                    return Err(Failure::Protocol(
                        "invalid message role or unpaired exchange".into(),
                    ));
                }
            }
            Item::Opaque(_) => (),
        }
    }
    if !pending.is_empty() {
        return Err(Failure::Protocol("missing context tool result".into()));
    }
    Ok(())
}
