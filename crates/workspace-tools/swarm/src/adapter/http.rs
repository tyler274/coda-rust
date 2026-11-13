//! Chat APIs driven directly: the Anthropic Messages API and OpenAI-style
//! chat completions (OpenAI, OpenRouter, Ollama). Each turn starts the swarm
//! MCP command, offers its tools to the model, and runs tool calls through
//! it until the model answers without one or the call budget runs out.

use std::collections::BTreeMap;
use std::sync::mpsc;
use std::sync::{Arc, Mutex, PoisonError};
use std::thread;
use std::time::Duration;

use serde_json::{Value, json};

use super::mcp_client::{McpClient, McpError, Tool};
use super::process::ProcessLimits;
use super::{
    Cancel, DEFAULT_RETRY_AFTER, ForkError, TurnOutcome, TurnRequest, WorkerAdapter, excerpt,
};
use crate::names::MemberName;

const ANTHROPIC_VERSION: &str = "2023-06-01";
const MAX_RESPONSE: u64 = 16 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Api {
    Anthropic {
        base_url: String,
        max_tokens: u32,
    },
    /// `max_tokens` is sent only when set: OpenAI's newer models reject it.
    OpenAi {
        base_url: String,
        max_tokens: Option<u32>,
    },
}

impl Api {
    fn name(&self) -> &'static str {
        match self {
            Self::Anthropic { .. } => "anthropic",
            Self::OpenAi { .. } => "openai-compatible",
        }
    }
}

/// Member conversations in one API's message format, shared by every model
/// entry on that API so a member can fork across them.
#[derive(Debug, Default)]
pub struct Histories(Mutex<BTreeMap<MemberName, Vec<Value>>>);

impl Histories {
    fn get(&self, member: &MemberName) -> Vec<Value> {
        self.lock().get(member).cloned().unwrap_or_default()
    }

    fn set(&self, member: MemberName, messages: Vec<Value>) {
        self.lock().insert(member, messages);
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, BTreeMap<MemberName, Vec<Value>>> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

#[derive(Debug, Clone)]
pub struct HttpAdapter {
    pub api: Api,
    /// The provider model name.
    pub model: String,
    /// The request env variable holding the API key; `None` sends no key.
    pub key_env: Option<String>,
    /// Model calls per turn before the turn fails without a final answer.
    pub max_calls: u32,
    pub request_timeout: Duration,
    pub mcp_timeout: Duration,
    pub limits: ProcessLimits,
    histories: Arc<Histories>,
}

impl HttpAdapter {
    pub fn new(api: Api, model: impl Into<String>, key_env: Option<String>) -> Self {
        Self::with_histories(api, model, key_env, Arc::default())
    }

    pub fn with_histories(
        api: Api,
        model: impl Into<String>,
        key_env: Option<String>,
        histories: Arc<Histories>,
    ) -> Self {
        Self {
            api,
            model: model.into(),
            key_env,
            max_calls: 32,
            request_timeout: Duration::from_secs(600),
            mcp_timeout: Duration::from_secs(300),
            limits: ProcessLimits::default(),
            histories,
        }
    }
}

/// Why a turn stopped before a final answer.
enum Stop {
    Cancelled,
    Quota(Duration),
    Failed(String),
}

impl From<McpError> for Stop {
    fn from(error: McpError) -> Self {
        match error {
            McpError::Cancelled => Self::Cancelled,
            other => Self::Failed(format!("swarm MCP: {}", error_chain(&other))),
        }
    }
}

fn error_chain(error: &dyn std::error::Error) -> String {
    let mut text = error.to_string();
    let mut source = error.source();
    while let Some(cause) = source {
        text.push_str(": ");
        text.push_str(&cause.to_string());
        source = cause.source();
    }
    text
}

/// A model's reply: the assistant message to keep, and its tool calls.
struct Reply {
    message: Value,
    calls: Vec<Call>,
}

struct Call {
    id: String,
    name: String,
    /// The model's arguments, or why they could not be parsed.
    arguments: Result<Value, String>,
}

impl WorkerAdapter for HttpAdapter {
    fn run_turn(&self, request: TurnRequest, cancel: &Cancel) -> TurnOutcome {
        let key = self.key_env.as_ref().map(|var| {
            request
                .env
                .iter()
                .find(|(name, _)| name == var)
                .map(|(_, value)| value.clone())
        });
        let key = match key {
            Some(None) => {
                return TurnOutcome::Failed {
                    reason: format!(
                        "{} key {} is not in the turn env",
                        self.api.name(),
                        self.key_env.as_deref().unwrap_or_default()
                    ),
                };
            }
            Some(Some(key)) => Some(key),
            None => None,
        };
        // The provider key stays in this process; the MCP server never sees it.
        let mcp_env: Vec<(String, String)> = request
            .env
            .iter()
            .filter(|(name, _)| Some(name) != self.key_env.as_ref())
            .cloned()
            .collect();
        let mut messages = self.histories.get(&request.member);
        messages.push(json!({ "role": "user", "content": request.prompt }));
        let ended = self.converse(&request, &mcp_env, key.as_deref(), &mut messages, cancel);
        match ended {
            Ok(()) => {
                self.histories.set(request.member.clone(), messages);
                TurnOutcome::Completed
            }
            Err(Stop::Cancelled) => TurnOutcome::Cancelled,
            Err(Stop::Quota(retry_after)) => TurnOutcome::QuotaExhausted { retry_after },
            Err(Stop::Failed(reason)) => {
                let reason = match &key {
                    Some(key) => reason.replace(key.as_str(), "<redacted>"),
                    None => reason,
                };
                TurnOutcome::Failed {
                    reason: request.redact(reason),
                }
            }
        }
    }

    fn fork(
        &self,
        from: &MemberName,
        to: &MemberName,
        _model: Option<&str>,
    ) -> Result<(), ForkError> {
        let history = self.histories.get(from);
        if history.is_empty() {
            return Err(ForkError::NoHistory(from.clone()));
        }
        self.histories.set(to.clone(), history);
        Ok(())
    }
}

impl HttpAdapter {
    fn converse(
        &self,
        request: &TurnRequest,
        mcp_env: &[(String, String)],
        key: Option<&str>,
        messages: &mut Vec<Value>,
        cancel: &Cancel,
    ) -> Result<(), Stop> {
        let mut mcp = McpClient::start(
            &request.mcp_command,
            mcp_env,
            &request.cwd,
            self.limits.clone(),
            self.mcp_timeout,
            cancel,
        )?;
        let tools = mcp.list_tools(cancel)?;
        for _ in 0..self.max_calls {
            let body = self.body(messages, &tools);
            let response = self.post(key, body, cancel)?;
            let reply = self.parse(&response)?;
            messages.push(reply.message);
            if reply.calls.is_empty() {
                return Ok(());
            }
            let mut results = Vec::with_capacity(reply.calls.len());
            for call in reply.calls {
                let called = match call.arguments {
                    Ok(arguments) => mcp.call_tool(&call.name, arguments, cancel),
                    Err(message) => Err(McpError::Rpc {
                        method: call.name.clone(),
                        message,
                    }),
                };
                let result = match called {
                    Ok(result) => result,
                    Err(McpError::Cancelled) => return Err(Stop::Cancelled),
                    Err(McpError::Rpc { message, .. }) => super::mcp_client::CallResult {
                        text: message,
                        is_error: true,
                    },
                    Err(other) => return Err(other.into()),
                };
                results.push((call.id, result));
            }
            match self.api {
                Api::Anthropic { .. } => {
                    let blocks: Vec<Value> = results
                        .into_iter()
                        .map(|(id, result)| {
                            json!({
                                "type": "tool_result",
                                "tool_use_id": id,
                                "content": result.text,
                                "is_error": result.is_error,
                            })
                        })
                        .collect();
                    messages.push(json!({ "role": "user", "content": blocks }));
                }
                Api::OpenAi { .. } => {
                    messages.extend(results.into_iter().map(|(id, result)| {
                        json!({ "role": "tool", "tool_call_id": id, "content": result.text })
                    }));
                }
            }
        }
        Err(Stop::Failed(format!(
            "no final answer after {} model calls",
            self.max_calls
        )))
    }

    fn body(&self, messages: &[Value], tools: &[Tool]) -> Value {
        match &self.api {
            Api::Anthropic { max_tokens, .. } => {
                let tools: Vec<Value> = tools
                    .iter()
                    .map(|tool| {
                        json!({
                            "name": tool.name,
                            "description": tool.description,
                            "input_schema": tool.input_schema,
                        })
                    })
                    .collect();
                json!({
                    "model": self.model,
                    "max_tokens": max_tokens,
                    "messages": messages,
                    "tools": tools,
                })
            }
            Api::OpenAi { max_tokens, .. } => {
                let tools: Vec<Value> = tools
                    .iter()
                    .map(|tool| {
                        json!({
                            "type": "function",
                            "function": {
                                "name": tool.name,
                                "description": tool.description,
                                "parameters": tool.input_schema,
                            }
                        })
                    })
                    .collect();
                let mut body = json!({
                    "model": self.model,
                    "messages": messages,
                    "tools": tools,
                });
                if let Some(max_tokens) = max_tokens {
                    body["max_tokens"] = json!(max_tokens);
                }
                body
            }
        }
    }

    /// Posts on a worker thread so cancellation returns at once; an
    /// abandoned request ends on its own timeout.
    fn post(&self, key: Option<&str>, body: Value, cancel: &Cancel) -> Result<Value, Stop> {
        let (url, headers) = match &self.api {
            Api::Anthropic { base_url, .. } => {
                let mut headers = vec![("anthropic-version", ANTHROPIC_VERSION.to_owned())];
                if let Some(key) = key {
                    headers.push(("x-api-key", key.to_owned()));
                }
                (
                    format!("{}/v1/messages", base_url.trim_end_matches('/')),
                    headers,
                )
            }
            Api::OpenAi { base_url, .. } => {
                let headers = key
                    .map(|key| vec![("authorization", format!("Bearer {key}"))])
                    .unwrap_or_default();
                (
                    format!("{}/chat/completions", base_url.trim_end_matches('/')),
                    headers,
                )
            }
        };
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .timeout_global(Some(self.request_timeout))
            .build()
            .into();
        let (send, receive) = mpsc::channel();
        thread::spawn(move || {
            let mut call = agent.post(&url).header("content-type", "application/json");
            for (name, value) in headers {
                call = call.header(name, value);
            }
            let result = call.send(body.to_string()).map(|mut response| {
                let status = response.status().as_u16();
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|value| value.to_str().ok())
                    .and_then(|value| value.trim().parse::<u64>().ok());
                let text = response
                    .body_mut()
                    .with_config()
                    .limit(MAX_RESPONSE)
                    .read_to_string();
                (status, retry_after, text)
            });
            // The turn may have been cancelled and stopped listening.
            let _ = send.send(result);
        });
        let received = loop {
            if cancel.is_cancelled() {
                return Err(Stop::Cancelled);
            }
            match receive.recv_timeout(self.limits.poll) {
                Ok(received) => break received,
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    return Err(Stop::Failed("HTTP request thread panicked".into()));
                }
            }
        };
        let (status, retry_after, text) = received
            .map_err(|error| Stop::Failed(format!("{} request: {error}", self.api.name())))?;
        let text = text
            .map_err(|error| Stop::Failed(format!("{} response body: {error}", self.api.name())))?;
        if status == 429 {
            return Err(Stop::Quota(
                retry_after.map_or(DEFAULT_RETRY_AFTER, Duration::from_secs),
            ));
        }
        if !(200..300).contains(&status) {
            return Err(Stop::Failed(format!(
                "{} returned HTTP {status}: {}",
                self.api.name(),
                excerpt(&text)
            )));
        }
        serde_json::from_str(&text).map_err(|error| {
            Stop::Failed(format!(
                "{} returned invalid JSON ({error}): {}",
                self.api.name(),
                excerpt(&text)
            ))
        })
    }

    fn parse(&self, response: &Value) -> Result<Reply, Stop> {
        let malformed = || {
            Stop::Failed(format!(
                "{} returned an unexpected reply: {}",
                self.api.name(),
                excerpt(&response.to_string())
            ))
        };
        match self.api {
            Api::Anthropic { .. } => {
                let content = response["content"].as_array().ok_or_else(malformed)?;
                let calls = content
                    .iter()
                    .filter(|block| block["type"] == "tool_use")
                    .map(|block| {
                        Some(Call {
                            id: block["id"].as_str()?.to_owned(),
                            name: block["name"].as_str()?.to_owned(),
                            arguments: Ok(block["input"].clone()),
                        })
                    })
                    .collect::<Option<Vec<_>>>()
                    .ok_or_else(malformed)?;
                Ok(Reply {
                    message: json!({ "role": "assistant", "content": content }),
                    calls,
                })
            }
            Api::OpenAi { .. } => {
                let message = &response["choices"][0]["message"];
                if !message.is_object() {
                    return Err(malformed());
                }
                let calls = match message["tool_calls"].as_array() {
                    None => Vec::new(),
                    Some(calls) => calls
                        .iter()
                        .map(|call| {
                            let arguments = call["function"]["arguments"].as_str()?;
                            Some(Call {
                                id: call["id"].as_str()?.to_owned(),
                                name: call["function"]["name"].as_str()?.to_owned(),
                                arguments: serde_json::from_str(arguments).map_err(|error| {
                                    format!("arguments are not valid JSON: {error}")
                                }),
                            })
                        })
                        .collect::<Option<Vec<_>>>()
                        .ok_or_else(malformed)?,
                };
                Ok(Reply {
                    message: message.clone(),
                    calls,
                })
            }
        }
    }
}
