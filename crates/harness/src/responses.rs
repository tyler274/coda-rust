//! Structured OpenAI Responses wire adapter. Construction is explicit and
//! never reads credentials or authorizes a paid call. Product startup owns that
//! authorization. It preserves call IDs, JSON arguments, usage and provenance.

use crate::execution::{Call, Cancellation, Item, Provider, Request, Response, StreamEvent, Usage};
use serde_json::{Value, json};

/// Explicitly configured provider. Keys are never serialized or debug printed.
pub struct ResponsesProvider {
    client: reqwest::Client,
    endpoint: String,
    key: String,
}
impl ResponsesProvider {
    /// Creates the transport after the product has authorized calls. The client
    /// has a bounded total deadline; no automatic provider retry is performed.
    pub fn new(endpoint: String, key: String) -> Result<Self, String> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .map_err(|_| "could not build provider client")?;
        Ok(Self {
            client,
            endpoint,
            key,
        })
    }
}

/// Converts neutral context to Responses wire objects without dropping calls.
pub fn encode(request: &Request) -> Value {
    let input: Vec<Value> = request.items.iter().map(|item| match item {
        Item::Message { role, text } => json!({"role": role, "content": text}),
        Item::Call(call) => json!({"type": "function_call", "call_id": call.id, "name": call.name, "arguments": call.arguments.to_string()}),
        Item::Result { call_id, value } => json!({"type": "function_call_output", "call_id": call_id, "output": value.to_string()}),
    }).collect();
    let mut reasoning = request.options.clone();
    let max_output = reasoning
        .as_object_mut()
        .and_then(|o| o.remove("max_output_tokens"));
    let mut body = json!({"model": request.model, "reasoning": reasoning, "instructions": request.instructions,
        "tools": request.tools, "input": input, "store": false});
    if let Some(max_output) = max_output {
        body["max_output_tokens"] = max_output;
    }
    body
}

/// Decodes one completed Responses exchange. Incomplete/refused provider
/// results never become an empty successful product completion.
pub fn decode(body: &Value) -> Result<Response, String> {
    if body.get("status").and_then(Value::as_str) != Some("completed") {
        return Err("provider response was not completed".into());
    }
    let output = body
        .get("output")
        .and_then(Value::as_array)
        .ok_or("provider response has no output")?;
    let mut text = String::new();
    let mut calls = vec![];
    for item in output {
        match item.get("type").and_then(Value::as_str) {
            Some("function_call") => {
                calls.push(Call {
                    id: item
                        .get("call_id")
                        .and_then(Value::as_str)
                        .ok_or("missing call identity")?
                        .into(),
                    name: item
                        .get("name")
                        .and_then(Value::as_str)
                        .ok_or("missing tool name")?
                        .into(),
                    arguments: serde_json::from_str(
                        item.get("arguments")
                            .and_then(Value::as_str)
                            .ok_or("missing tool arguments")?,
                    )
                    .map_err(|_| "invalid tool argument JSON")?,
                });
            }
            Some("message") => {
                for part in item
                    .get("content")
                    .and_then(Value::as_array)
                    .ok_or("missing message content")?
                {
                    match part.get("type").and_then(Value::as_str) {
                        Some("output_text") => text.push_str(
                            part.get("text")
                                .and_then(Value::as_str)
                                .ok_or("missing output text")?,
                        ),
                        Some("refusal") => return Err("provider refused response".into()),
                        _ => return Err("unsupported provider message content".into()),
                    }
                }
            }
            // Reasoning stays provider-private and is not a clinical source.
            Some("reasoning") => {}
            _ => return Err("unsupported provider output item".into()),
        }
    }
    if text.is_empty() && calls.is_empty() {
        return Err("provider returned no authored output".into());
    }
    Ok(Response {
        text,
        calls,
        usage: Usage {
            input_tokens: body.pointer("/usage/input_tokens").and_then(Value::as_u64),
            output_tokens: body.pointer("/usage/output_tokens").and_then(Value::as_u64),
        },
        provenance: json!({"adapter": "openai-responses", "mode": "api", "response_id": body.get("id"), "cached_input_tokens": body.pointer("/usage/input_tokens_details/cached_tokens")}),
    })
}
impl Provider for ResponsesProvider {
    async fn exchange(
        &mut self,
        request: &Request,
        cancel: &Cancellation,
        emit: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
    ) -> Result<Response, String> {
        cancel.check()?;
        let pending = async {
            let response = self
                .client
                .post(&self.endpoint)
                .bearer_auth(&self.key)
                .json(&{
                    let mut body = encode(request);
                    body["stream"] = json!(true);
                    body
                })
                .send()
                .await
                .map_err(|_| "provider request failed")?;
            if !response.status().is_success() {
                return Err(format!("provider returned HTTP {}", response.status()));
            }
            if !response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .is_some_and(|v| v.starts_with("text/event-stream"))
            {
                return Err("provider did not return an event stream".into());
            }
            let mut response = response;
            let mut decoder = EventDecoder::default();
            let mut total = 0usize;
            while let Some(chunk) = response.chunk().await.map_err(|_| "provider body failed")? {
                total = total.saturating_add(chunk.len());
                if total > 4 * 1024 * 1024 {
                    return Err("provider body bound reached".into());
                }
                for event in decoder.push(&chunk)? {
                    match event.get("type").and_then(Value::as_str) {
                        Some("response.output_text.delta") => emit(StreamEvent::Text(
                            event
                                .get("delta")
                                .and_then(Value::as_str)
                                .ok_or("missing text delta")?
                                .into(),
                        ))?,
                        Some("response.created" | "response.in_progress") => {
                            emit(StreamEvent::ProviderIdentity(
                                json!({"adapter": "openai-responses", "response_id": event.pointer("/response/id")}),
                            ))?;
                        }
                        Some("response.completed" | "response.incomplete" | "response.failed") => {
                            let body = event.get("response").ok_or("missing terminal response")?;
                            emit(StreamEvent::ProviderIdentity(
                                json!({"adapter":"openai-responses","response_id":body.get("id"),"cached_input_tokens":body.pointer("/usage/input_tokens_details/cached_tokens")}),
                            ))?;
                            emit(StreamEvent::Usage(Usage {
                                input_tokens: body
                                    .pointer("/usage/input_tokens")
                                    .and_then(Value::as_u64),
                                output_tokens: body
                                    .pointer("/usage/output_tokens")
                                    .and_then(Value::as_u64),
                            }))?;
                            return decode(body);
                        }
                        Some("error") => return Err("provider stream error".into()),
                        _ => {} // Item/argument deltas are finalized by the terminal response.
                    }
                }
            }
            Err("provider stream interrupted before terminal response; usage may be unknown".into())
        };
        let response = tokio::select! {
            result = pending => result?,
            _ = async { loop { if cancel.is_cancelled() { break; } tokio::time::sleep(std::time::Duration::from_millis(20)).await; } } => {
                return Err("provider request cancelled; usage unknown".into());
            }
        };
        Ok(response)
    }
}

/// Incremental SSE framing. UTF-8 is decoded only after whole lines arrive;
/// chunks may split multibyte characters, CRLF, or a JSON tool argument.
#[derive(Default)]
struct EventDecoder {
    pending: Vec<u8>,
    data: Vec<String>,
}
impl EventDecoder {
    fn push(&mut self, bytes: &[u8]) -> Result<Vec<Value>, String> {
        self.pending.extend_from_slice(bytes);
        let mut events = Vec::new();
        while let Some(end) = self.pending.iter().position(|b| *b == b'\n') {
            let line: Vec<u8> = self.pending.drain(..=end).collect();
            let line = std::str::from_utf8(&line)
                .map_err(|_| "invalid stream UTF-8")?
                .trim_end_matches(['\n', '\r']);
            if line.is_empty() {
                if !self.data.is_empty() {
                    let data = self.data.join("\n");
                    self.data.clear();
                    if data != "[DONE]" {
                        events
                            .push(serde_json::from_str(&data).map_err(|_| "invalid stream JSON")?);
                    }
                }
            } else if let Some(data) = line.strip_prefix("data:") {
                self.data
                    .push(data.strip_prefix(' ').unwrap_or(data).into());
            }
        }
        Ok(events)
    }
}
