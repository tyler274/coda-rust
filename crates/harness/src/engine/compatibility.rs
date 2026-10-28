//! Representation and owner callbacks for the structured compatibility facade.
//! No model/tool orchestration lives here; `super::drive` owns that ordering.

use super::{Batch, Decision, Driver, drive, exchange, execute_batch, validate_identities};
use crate::execution::*;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashSet;

struct Adapter<'a, P, O> {
    provider: &'a mut P,
    owner: &'a mut O,
    request: Request,
    progress: &'a mut Progress,
    limits: Limits,
    cancel: &'a Cancellation,
    emit: &'a mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
    seen: HashSet<String>,
}

impl<P: Provider, O: Owner> Driver for Adapter<'_, P, O> {
    type Prepared = ();
    type Turn = Response;
    type Output = Response;

    async fn prepare(&mut self) -> Result<(), String> {
        self.cancel.check()?;
        self.owner.check().await?;
        if self.progress.calls >= self.limits.max_calls
            || self.progress.known_tokens > self.limits.max_known_tokens
        {
            return Err("execution usage bound reached".into());
        }
        bounded(&self.request, self.limits.max_context_bytes)?;
        self.progress.calls = self
            .progress
            .calls
            .checked_add(1)
            .ok_or("call counter overflow")?;
        self.owner
            .record(Record::Dispatch {
                sequence: self.progress.calls,
                request: self.request.clone(),
            })
            .await?;
        self.cancel.check()?;
        self.owner.check().await
    }

    async fn exchange(&mut self, _: &()) -> Result<Response, String> {
        let response = exchange::observed(
            self.provider,
            self.owner,
            &self.request,
            self.progress,
            self.cancel,
            self.emit,
        )
        .await?;
        self.cancel.check()?;
        self.owner.check().await?;
        Ok(response)
    }

    fn validate(&mut self, _: &(), response: &Response) -> Result<Vec<Call>, String> {
        bounded(response, self.limits.max_context_bytes)?;
        if self.progress.known_tokens > self.limits.max_known_tokens
            || response.calls.len() > self.limits.max_tools_per_response
        {
            return Err("execution usage bound reached".into());
        }
        validate_calls(&mut self.seen, response)?;
        Ok(response.calls.clone())
    }

    async fn settle(
        &mut self,
        _: (),
        response: Response,
        empty: bool,
    ) -> Result<Decision<Response>, String> {
        if empty {
            self.owner.complete(&response).await?;
            return Ok(Decision::Complete(response));
        }
        if self.owner.terminal(&response) {
            return match self.owner.finish(&response).await? {
                Finish::Committed => Ok(Decision::Complete(response)),
                Finish::Continue(value) => {
                    append_exchange(&mut self.request, &response);
                    append_continuation(&mut self.request, &response, &value);
                    Ok(Decision::Continue)
                }
            };
        }
        append_exchange(&mut self.request, &response);
        Ok(Decision::Tools)
    }
}

impl<P: Provider, O: Owner> Batch for Adapter<'_, P, O> {
    type Call = Call;
    type Error = String;

    async fn tool(&mut self, call: Call) -> Result<(), String> {
        dispatch(self.owner, &mut self.request, call, self.cancel).await
    }
}

pub(crate) async fn run<P: Provider, O: Owner>(
    provider: &mut P,
    owner: &mut O,
    request: Request,
    progress: &mut Progress,
    limits: Limits,
    cancel: &Cancellation,
    emit: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
) -> Result<Response, String> {
    let seen = request
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Call(call) => Some(call.id.clone()),
            _ => None,
        })
        .collect();
    drive(&mut Adapter {
        provider,
        owner,
        request,
        progress,
        limits,
        cancel,
        emit,
        seen,
    })
    .await
}

fn validate_calls(seen: &mut HashSet<String>, response: &Response) -> Result<(), String> {
    if !validate_identities(
        seen,
        response
            .calls
            .iter()
            .map(|call| (call.id.as_str(), call.name.as_str())),
    ) {
        return Err("invalid or repeated tool invocation identity".into());
    }
    Ok(())
}

async fn dispatch<O: Owner>(
    owner: &mut O,
    request: &mut Request,
    call: Call,
    cancel: &Cancellation,
) -> Result<(), String> {
    cancel.check()?;
    owner.check().await?;
    let value = owner.dispatch(&call, cancel).await?;
    request.items.push(Item::Result {
        call_id: call.id,
        value,
    });
    Ok(())
}

// Recovery has no provider, budgets or fresh run. It uses exactly the batch
// executor and dispatch callback used by the live adapter.
struct Recovery<'a, O> {
    owner: &'a mut O,
    request: Request,
    cancel: &'a Cancellation,
}

impl<O: Owner> Batch for Recovery<'_, O> {
    type Call = Call;
    type Error = String;

    async fn tool(&mut self, call: Call) -> Result<(), String> {
        dispatch(self.owner, &mut self.request, call, self.cancel).await
    }
}

pub(crate) async fn resume_tools<O: Owner>(
    owner: &mut O,
    mut request: Request,
    response: &Response,
    cancel: &Cancellation,
) -> Result<Request, String> {
    if response.calls.is_empty() || owner.terminal(response) {
        return Err("terminal exchange must use product completion recovery".into());
    }
    // A retained Exchange precedes live validation. Recovery must reject IDs
    // from the original Dispatch context too, before any sibling effect runs.
    let mut seen = request
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Call(call) => Some(call.id.clone()),
            _ => None,
        })
        .collect();
    validate_calls(&mut seen, response)?;
    append_exchange(&mut request, response);
    let mut recovery = Recovery {
        owner,
        request,
        cancel,
    };
    execute_batch(&mut recovery, response.calls.clone()).await?;
    Ok(recovery.request)
}

pub(crate) fn continued(mut request: Request, response: &Response, value: &Value) -> Request {
    append_exchange(&mut request, response);
    append_continuation(&mut request, response, value);
    request
}

fn append_continuation(request: &mut Request, response: &Response, value: &Value) {
    request
        .items
        .extend(response.calls.iter().map(|call| Item::Result {
            call_id: call.id.clone(),
            value: value.clone(),
        }));
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
