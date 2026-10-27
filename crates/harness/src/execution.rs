//! Product-neutral execution. The adapter is the only durable owner.
//!
//! Admission, claims, authorization, exact reads, effect reconciliation, recovery
//! and final publication belong to `Owner`, not to this loop. In particular a
//! successful provider response is not a completed product operation.

use std::collections::HashSet;
use std::future::Future;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Cooperative cancellation shared with transports and tools.
#[derive(Clone, Default)]
pub struct Cancellation(Arc<AtomicBool>);
impl Cancellation {
    /// Requests cancellation; it never commits product state.
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Release);
    }
    /// Whether cancellation was requested.
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
    /// Stops dispatch after cancellation.
    pub fn check(&self) -> Result<(), String> {
        if self.is_cancelled() {
            Err("execution cancelled".into())
        } else {
            Ok(())
        }
    }
}

/// Structured tool invocation, with provider identity retained verbatim.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Call {
    /// Stable invocation identity; the product must reconcile retries by it.
    pub id: String,
    /// Product-authorized tool name.
    pub name: String,
    /// Uninterpreted structured arguments.
    pub arguments: Value,
}

/// A context item. No product state or clinical types enter the engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Item {
    /// Authored user/system/assistant message.
    Message {
        /// Provider role.
        role: String,
        /// Authored content.
        text: String,
    },
    /// Assistant tool invocation.
    Call(Call),
    /// Product-admitted result; errors can be returned as structured feedback.
    Result {
        /// Invocation identity.
        call_id: String,
        /// Product-owned result bytes.
        value: Value,
    },
}

/// Complete provider context, assembled and restored by the product adapter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    /// Model binding, never selected by the engine.
    pub model: String,
    /// Provider-specific inference options.
    pub options: Value,
    /// Product instructions, preserved across exchanges.
    pub instructions: String,
    /// Tool schemas authorized by the product for this work.
    pub tools: Vec<Value>,
    /// Exact context; the engine never truncates or summarizes it.
    pub items: Vec<Item>,
}

/// Measured usage. Unknown token counts stay unknown, never become zero.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Usage {
    /// Measured input tokens, when supplied by the provider.
    pub input_tokens: Option<u64>,
    /// Measured output tokens, when supplied by the provider.
    pub output_tokens: Option<u64>,
}

/// A completed exchange, not a product completion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// Assistant text (possibly empty while tools are requested).
    pub text: String,
    /// Tool calls in provider order.
    pub calls: Vec<Call>,
    /// Actual reported usage.
    pub usage: Usage,
    /// Provider/record/replay provenance retained by the owner.
    pub provenance: Value,
}

/// Events are provisional; they confer no publication authority.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StreamEvent {
    /// Incremental authored text.
    Text(String),
    /// Provider usage observation.
    Usage(Usage),
    /// Provider response identity observed before termination. Persist it for
    /// interrupted-call reconciliation; it is not a completion receipt.
    ProviderIdentity(Value),
}

/// Provider transport. Poll `cancel` during long operations and call `emit` for
/// provisional streaming. Returning an error from `emit` must stop streaming.
pub trait Provider {
    /// Executes exactly one exchange. No automatic retry or tool execution.
    fn exchange(
        &mut self,
        request: &Request,
        cancel: &Cancellation,
        emit: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
    ) -> impl Future<Output = Result<Response, String>> + Send;
}

/// Execution evidence written into the product's existing accepted-work owner.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Record {
    /// Must commit before contacting a provider.
    Dispatch {
        /// Monotonic attempt ordinal supplied by the owner.
        sequence: u64,
        /// Exact dispatched request.
        request: Request,
    },
    /// Always retained before a currentness check, including late output usage.
    Exchange {
        /// Dispatch ordinal.
        sequence: u64,
        /// Raw structured response.
        response: Response,
    },
    /// Partial provider observations retained even when transport is interrupted.
    Observation {
        /// Dispatch ordinal.
        sequence: u64,
        /// Most recently reported cumulative usage, not a delta.
        usage: Usage,
        /// Observed provider identity/provenance, if any.
        provenance: Value,
    },
    /// Failed dispatch. The adapter decides recovery, never the engine.
    Failure {
        /// Dispatch ordinal.
        sequence: u64,
        /// Transport failure, without credentials.
        reason: String,
    },
}

/// Product authority and execution-record interface. Each callback must be
/// fenced by the durable claim/accepted work captured inside the adapter.
/// `dispatch` reconciles uncertain/duplicate effects in that same owner.
pub trait Owner: Send {
    /// Product-defined terminal tool exchange. Returning true routes the entire
    /// response to atomic completion, without executing its calls as effects.
    fn terminal(&self, _response: &Response) -> bool {
        false
    }
    /// Checks permission, cancellation, claim and source currentness.
    fn check(&mut self) -> impl Future<Output = Result<(), String>> + Send;
    /// Persists evidence, including usage for outputs that are now stale.
    fn record(&mut self, record: Record) -> impl Future<Output = Result<(), String>> + Send;
    /// Delivers provisional text under the product fence. This is never an
    /// approved reply or an execution record; products may discard it entirely.
    fn provisional(
        &mut self,
        _sequence: u64,
        _text: &str,
    ) -> impl Future<Output = Result<(), String>> + Send {
        async { Ok(()) }
    }
    /// Admits and executes one tool. Must recheck authority transactionally
    /// with effects, and reuse confirmed effects on a recovered invocation.
    fn dispatch(
        &mut self,
        call: &Call,
        cancel: &Cancellation,
    ) -> impl Future<Output = Result<Value, String>> + Send;
    /// Commits completion and staged effects atomically under the product fence.
    /// Must reject stale completion even if the preceding `check` succeeded.
    fn complete(&mut self, response: &Response) -> impl Future<Output = Result<(), String>> + Send;
    /// Settles a terminal exchange. The default commits through `complete`.
    /// A product may instead decline to publish because its inputs moved
    /// after the model last read them, and return `Finish::Continue` with a
    /// structured result for each terminal call. The run then makes another
    /// provider call under the same call and token bounds; nothing is
    /// published for the declined exchange.
    fn finish(
        &mut self,
        response: &Response,
    ) -> impl Future<Output = Result<Finish, String>> + Send {
        async move {
            self.complete(response).await?;
            Ok(Finish::Committed)
        }
    }
}

/// How a terminal exchange settled.
#[derive(Debug, Clone, PartialEq)]
pub enum Finish {
    /// The product committed the completion.
    Committed,
    /// The product declined to commit. The value is returned to the model as
    /// the result of each terminal call, and the run continues.
    Continue(Value),
}

/// Per-work resource bounds. Exhaustion leaves recovery to the product owner.
#[derive(Debug, Clone, Copy)]
pub struct Limits {
    /// Total provider calls, including retained earlier attempts.
    pub max_calls: u64,
    /// Maximum encoded request/response bytes (never silently truncate).
    pub max_context_bytes: usize,
    /// Maximum tools in one provider response.
    pub max_tools_per_response: usize,
    /// Maximum known cumulative tokens; unknown usage is independently bounded
    /// by calls and bytes and is explicitly counted in `Progress`.
    pub max_known_tokens: u64,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_calls: 16,
            max_context_bytes: 1_048_576,
            max_tools_per_response: 32,
            max_known_tokens: 200_000,
        }
    }
}

/// Restored counters from the product execution record, not a new run ledger.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Progress {
    /// Calls already dispatched (including failures).
    pub calls: u64,
    /// Sum of known token counts, including late responses.
    pub known_tokens: u64,
    /// Exchanges with at least one unknown token count.
    pub unknown_usage: u64,
}

impl Progress {
    /// Reconstructs cumulative usage from this accepted work's retained records.
    /// Observation snapshots and final exchange usage are never added twice.
    /// A dispatched call without final usage remains explicitly unknown.
    pub fn from_records(records: &[Record]) -> Self {
        let mut calls = std::collections::BTreeMap::<u64, Usage>::new();
        for record in records {
            match record {
                Record::Dispatch { sequence, .. } => {
                    calls.entry(*sequence).or_default();
                }
                Record::Observation {
                    sequence, usage, ..
                } => {
                    let previous = calls.entry(*sequence).or_default();
                    previous.input_tokens = usage.input_tokens.or(previous.input_tokens);
                    previous.output_tokens = usage.output_tokens.or(previous.output_tokens);
                }
                Record::Exchange { sequence, response } => {
                    let previous = calls.entry(*sequence).or_default();
                    previous.input_tokens = response.usage.input_tokens.or(previous.input_tokens);
                    previous.output_tokens =
                        response.usage.output_tokens.or(previous.output_tokens);
                }
                Record::Failure { .. } => {}
            }
        }
        let mut progress = Self {
            calls: calls.keys().next_back().copied().unwrap_or(0),
            ..Self::default()
        };
        for usage in calls.values() {
            progress.known_tokens = progress
                .known_tokens
                .saturating_add(usage.input_tokens.unwrap_or(0))
                .saturating_add(usage.output_tokens.unwrap_or(0));
            if usage.input_tokens.is_none() || usage.output_tokens.is_none() {
                progress.unknown_usage += 1;
            }
        }
        progress
    }
}

/// Executes accepted work. On error the adapter must settle/reclaim its work;
/// the engine neither discards it nor decides a retry policy. Restore `request`
/// and `progress` from that same owner when resuming.
pub async fn run<P: Provider, O: Owner>(
    provider: &mut P,
    owner: &mut O,
    mut request: Request,
    progress: &mut Progress,
    limits: Limits,
    cancel: &Cancellation,
    emit: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
) -> Result<Response, String> {
    let mut seen: HashSet<String> = request
        .items
        .iter()
        .filter_map(|item| {
            if let Item::Call(call) = item {
                Some(call.id.clone())
            } else {
                None
            }
        })
        .collect();
    loop {
        cancel.check()?;
        owner.check().await?;
        if progress.calls >= limits.max_calls || progress.known_tokens > limits.max_known_tokens {
            return Err("execution usage bound reached".into());
        }
        bounded(&request, limits.max_context_bytes)?;
        progress.calls = progress
            .calls
            .checked_add(1)
            .ok_or("call counter overflow")?;
        let sequence = progress.calls;
        owner
            .record(Record::Dispatch {
                sequence,
                request: request.clone(),
            })
            .await?;
        cancel.check()?;
        owner.check().await?;
        let mut observed_usage = Usage::default();
        let mut observed_identity = Value::Null;
        // Drain provider observations into the existing owner while the HTTP
        // stream is still open. Bounded backpressure fails the call rather than
        // accumulating unbounded unjournaled provider evidence.
        let (observations, mut pending_observations) = tokio::sync::mpsc::channel(64);
        let mut deliver = |event: StreamEvent| {
            match &event {
                StreamEvent::Usage(usage) => observed_usage = usage.clone(),
                StreamEvent::ProviderIdentity(identity) => observed_identity = identity.clone(),
                StreamEvent::Text(_) => {}
            }
            let pending = match &event {
                StreamEvent::Text(text) => Err(text.clone()),
                _ => Ok(Record::Observation {
                    sequence,
                    usage: observed_usage.clone(),
                    provenance: observed_identity.clone(),
                }),
            };
            observations
                .try_send(pending)
                .map_err(|_| "provider observation backpressure".to_owned())?;
            cancel.check()?;
            emit(event)
        };
        let result = {
            let exchange = provider.exchange(&request, cancel, &mut deliver);
            tokio::pin!(exchange);
            let mut delivery_error = None;
            let result = loop {
                tokio::select! {
                    biased;
                    Some(observation) = pending_observations.recv() => match observation {
                        Ok(record) => owner.record(record).await?,
                        Err(text) if delivery_error.is_none() => {
                            if let Err(error) = owner.provisional(sequence, &text).await {
                                delivery_error = Some(error);
                                cancel.cancel();
                            }
                        },
                        Err(_) => {},
                    },
                    result = &mut exchange => break result,
                    _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {
                        if owner.check().await.is_err() { cancel.cancel(); }
                    }
                }
            };
            while let Ok(observation) = pending_observations.try_recv() {
                match observation {
                    Ok(record) => owner.record(record).await?,
                    Err(text) if delivery_error.is_none() => {
                        if let Err(error) = owner.provisional(sequence, &text).await {
                            delivery_error = Some(error);
                            cancel.cancel();
                        }
                    }
                    Err(_) => {}
                }
            }
            // Retain successful late response usage even after presentation
            // was fenced off. Cancellation below prevents publication.
            match result {
                Ok(response) => Ok(response),
                Err(error) => Err(delivery_error.unwrap_or(error)),
            }
        };
        let response = match result {
            Ok(mut response) => {
                response.usage.input_tokens =
                    response.usage.input_tokens.or(observed_usage.input_tokens);
                response.usage.output_tokens = response
                    .usage
                    .output_tokens
                    .or(observed_usage.output_tokens);
                response
            }
            Err(reason) => {
                progress.known_tokens = progress
                    .known_tokens
                    .saturating_add(observed_usage.input_tokens.unwrap_or(0))
                    .saturating_add(observed_usage.output_tokens.unwrap_or(0));
                if observed_usage.input_tokens.is_none() || observed_usage.output_tokens.is_none() {
                    progress.unknown_usage = progress.unknown_usage.saturating_add(1);
                }
                owner
                    .record(Record::Observation {
                        sequence,
                        usage: observed_usage,
                        provenance: observed_identity,
                    })
                    .await?;
                owner
                    .record(Record::Failure {
                        sequence,
                        reason: reason.clone(),
                    })
                    .await?;
                return Err(reason);
            }
        };
        owner
            .record(Record::Exchange {
                sequence,
                response: response.clone(),
            })
            .await?;
        progress.known_tokens = progress
            .known_tokens
            .saturating_add(response.usage.input_tokens.unwrap_or(0))
            .saturating_add(response.usage.output_tokens.unwrap_or(0));
        if response.usage.input_tokens.is_none() || response.usage.output_tokens.is_none() {
            progress.unknown_usage = progress.unknown_usage.saturating_add(1);
        }
        cancel.check()?;
        owner.check().await?;
        bounded(&response, limits.max_context_bytes)?;
        if progress.known_tokens > limits.max_known_tokens
            || response.calls.len() > limits.max_tools_per_response
        {
            return Err("execution usage bound reached".into());
        }
        // Validate the whole batch before any effect, not halfway through it.
        for call in &response.calls {
            if call.id.trim().is_empty()
                || call.name.trim().is_empty()
                || !seen.insert(call.id.clone())
            {
                return Err("invalid or repeated tool invocation identity".into());
            }
        }
        if response.calls.is_empty() {
            owner.complete(&response).await?;
            return Ok(response);
        }
        if owner.terminal(&response) {
            match owner.finish(&response).await? {
                Finish::Committed => return Ok(response),
                Finish::Continue(value) => {
                    request = continued(request, &response, &value);
                    continue;
                }
            }
        }
        request = resume_tools(owner, request, &response, cancel).await?;
    }
}

/// Reconciles the tool batch of an already retained exchange without making
/// another provider call. Recovery passes the corresponding Dispatch request
/// (before this response was appended). `Owner::dispatch` must return retained
/// results for confirmed effects and reconcile uncertain ones by original call
/// identity. This function does not decide whether a product may resume.
pub async fn resume_tools<O: Owner>(
    owner: &mut O,
    mut request: Request,
    response: &Response,
    cancel: &Cancellation,
) -> Result<Request, String> {
    if response.calls.is_empty() || owner.terminal(response) {
        return Err("terminal exchange must use product completion recovery".into());
    }
    let mut ids = HashSet::new();
    for call in &response.calls {
        if call.id.trim().is_empty() || call.name.trim().is_empty() || !ids.insert(&call.id) {
            return Err("invalid or repeated tool invocation identity".into());
        }
    }
    append_exchange(&mut request, response);
    for call in &response.calls {
        cancel.check()?;
        owner.check().await?;
        let value = owner.dispatch(call, cancel).await?;
        request.items.push(Item::Result {
            call_id: call.id.clone(),
            value,
        });
    }
    Ok(request)
}

/// The request after a declined terminal exchange: the exchange itself, then
/// the product's result for each terminal call. Exposed so recovery can rebuild
/// the same context from retained records.
pub fn continued(mut request: Request, response: &Response, value: &Value) -> Request {
    append_exchange(&mut request, response);
    request
        .items
        .extend(response.calls.iter().map(|call| Item::Result {
            call_id: call.id.clone(),
            value: value.clone(),
        }));
    request
}

fn append_exchange(request: &mut Request, response: &Response) {
    if !response.text.is_empty() {
        request.items.push(Item::Message {
            role: "assistant".into(),
            text: response.text.clone(),
        });
    }
    request
        .items
        .extend(response.calls.iter().cloned().map(Item::Call));
}

fn bounded(value: &impl Serialize, max: usize) -> Result<(), String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    if bytes.len() > max {
        Err("execution context byte bound reached".into())
    } else {
        Ok(())
    }
}
