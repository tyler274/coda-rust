//! API-backed OpenAI Responses adapter. Buffered, bounded output, `store: false`,
//! no retries and no secret logging. Streaming publication is intentionally not
//! implemented: products must define their visibility and durable delta contract.

use async_trait::async_trait;
use serde_json::{Value, json};

use crate::engine::{Invocation, Item, Provider, Request, Turn};

/// OpenAI API adapter. Credentials stay in this instance and are never in events.
/// Construction does not authorize paid calls; the composition supplies that gate.
pub struct Responses {
    client: reqwest::Client,
    endpoint: reqwest::Url,
    key: String,
    model: String,
}

impl Responses {
    /// Uses the fixed production endpoint. No request is made by construction.
    pub fn new(key: String, model: String) -> Result<Self, String> {
        Self::with_endpoint(key, model, "https://api.openai.com/v1/responses")
    }

    /// Selects an explicit HTTPS endpoint, or loopback HTTP for protocol tests.
    /// Callers must trust the endpoint: it receives the supplied API credential.
    pub fn with_endpoint(key: String, model: String, endpoint: &str) -> Result<Self, String> {
        let endpoint = reqwest::Url::parse(endpoint).map_err(|_| "invalid provider endpoint")?;
        let loopback = matches!(
            endpoint.host_str(),
            Some("127.0.0.1" | "localhost" | "[::1]")
        );
        if (endpoint.scheme() != "https" && !(endpoint.scheme() == "http" && loopback))
            || !endpoint.username().is_empty()
            || endpoint.password().is_some()
            || endpoint.fragment().is_some()
            || endpoint.query().is_some()
            || key.trim().is_empty()
            || model.trim().is_empty()
        {
            return Err("provider requires credentials, model and trusted HTTPS endpoint".into());
        }
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .build()
            .map_err(|_| "cannot construct provider client")?;
        Ok(Self {
            client,
            endpoint,
            key,
            model,
        })
    }

    fn body(&self, request: &Request<'_>) -> Result<Value, String> {
        let input: Result<Vec<_>, _> = request.context.items.iter().map(|item| Ok(match item {
            Item::Message { role, text } => json!({"role": role, "content": text}),
            Item::Call(call) => json!({"type": "function_call", "call_id": call.id,
                "name": call.name, "arguments": call.arguments}),
            Item::Result { call_id, output } => json!({"type": "function_call_output", "call_id": call_id, "output": output}),
            Item::Opaque(value) => {
                // Opaque items cannot smuggle calls, results or instruction roles.
                if value.get("type").and_then(Value::as_str) != Some("reasoning") {
                    return Err("unsupported opaque Responses continuation item".to_owned());
                }
                value.clone()
            }
        })).collect();
        Ok(json!({
            "model": self.model, "instructions": request.context.instructions,
            "input": input?, "store": false, "stream": false,
            "max_output_tokens": request.limits.output_tokens,
            "tools": request.context.tools.iter().map(|tool| json!({
                "type": "function", "name": tool.name, "description": tool.description,
                "parameters": tool.parameters,
            })).collect::<Vec<_>>(),
        }))
    }
}

#[async_trait]
impl Provider for Responses {
    fn request_bytes(&self, request: &Request<'_>) -> Result<usize, String> {
        serde_json::to_vec(&self.body(request)?)
            .map(|bytes| bytes.len())
            .map_err(|_| "invalid request JSON".into())
    }

    async fn complete(&mut self, request: Request<'_>) -> Result<Turn, String> {
        let body = self.body(&request)?;
        let mut response = self
            .client
            .post(self.endpoint.clone())
            .bearer_auth(&self.key)
            .json(&body)
            .send()
            .await
            .map_err(|_| "provider transport failed; outcome may be unknown")?;
        if !response.status().is_success() {
            return Err(format!(
                "provider HTTP {}; no automatic retry",
                response.status().as_u16()
            ));
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| "provider body interrupted; outcome unknown")?
        {
            if chunk.len() > request.limits.response_bytes.saturating_sub(bytes.len()) {
                return Err("provider response byte limit".into());
            }
            bytes.extend_from_slice(&chunk);
        }
        decode(&bytes)
    }
}

fn decode(bytes: &[u8]) -> Result<Turn, String> {
    let response: Value = serde_json::from_slice(bytes).map_err(|_| "invalid provider JSON")?;
    if response.get("status").and_then(Value::as_str) != Some("completed") {
        return Err("provider response did not complete; output is not executable".into());
    }
    let output = response
        .get("output")
        .and_then(Value::as_array)
        .ok_or("missing provider output")?;
    let mut items = Vec::new();
    for item in output {
        match item.get("type").and_then(Value::as_str) {
            Some("function_call") => {
                if item
                    .get("status")
                    .and_then(Value::as_str)
                    .is_some_and(|s| s != "completed")
                {
                    return Err("incomplete provider tool call".into());
                }
                items.push(Item::Call(Invocation {
                    id: string(item, "call_id")?,
                    name: string(item, "name")?,
                    arguments: string(item, "arguments")?,
                }));
            }
            Some("message") => {
                if item.get("role").and_then(Value::as_str) != Some("assistant")
                    || item.get("status").and_then(Value::as_str) != Some("completed")
                {
                    return Err("invalid provider message".into());
                }
                let parts = item
                    .get("content")
                    .and_then(Value::as_array)
                    .ok_or("missing message content")?;
                let mut text = String::new();
                for part in parts {
                    match part.get("type").and_then(Value::as_str) {
                        Some("output_text") => text.push_str(&string(part, "text")?),
                        Some("refusal") => return Err("provider refused request".into()),
                        _ => return Err("unsupported provider content".into()),
                    }
                }
                items.push(Item::Message {
                    role: "assistant".into(),
                    text,
                });
            }
            Some("reasoning") => items.push(Item::Opaque(item.clone())),
            _ => return Err("unsupported provider output item".into()),
        }
    }
    Ok(Turn {
        items,
        receipt: json!({"id": response.get("id"), "usage": response.get("usage"), "status": "completed"}),
    })
}

fn string(value: &Value, key: &str) -> Result<String, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| format!("missing provider {key}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incomplete_output_never_executes() {
        assert!(decode(br#"{"status":"incomplete","output":[{"type":"function_call","call_id":"c","name":"save","arguments":"{}"}]}"#).is_err());
        assert!(
            decode(br#"{"status":"completed","output":[{"type":"function_call","call_id":"c"}]}"#)
                .is_err()
        );
    }

    #[test]
    fn endpoint_requires_tls_except_local_tests() {
        assert!(
            Responses::with_endpoint(
                "secret".into(),
                "model".into(),
                "http://example.com/v1/responses"
            )
            .is_err()
        );
        assert!(Responses::new("".into(), "model".into()).is_err());
    }
}
