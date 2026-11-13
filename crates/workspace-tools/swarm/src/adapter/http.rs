//! OpenAI-style chat completions driven directly: the host model proxy or a
//! local Ollama. Each turn starts the swarm MCP command, offers its tools and
//! the workspace tools to the model, and runs tool calls until the model
//! answers without one or the call budget runs out. Replies are requested
//! whole, not streamed.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::mpsc;
use std::sync::{Arc, Mutex, PoisonError};
use std::thread;
use std::time::Duration;

use serde_json::{Value, json};

use super::keys::{KeyItem, KeySource};
use super::mcp_client::{CallResult, McpClient, McpError, Tool};
use super::process::ProcessLimits;
use super::workspace_tools::{Cancelled, Workspace, WorkspaceCall, WorkspaceTool};
use super::{
    Cancel, DEFAULT_RETRY_AFTER, ForkError, TurnOutcome, TurnRequest, WorkerAdapter, excerpt,
};
use crate::names::MemberName;

const API: &str = "openai-compatible";
const MAX_RESPONSE: u64 = 16 * 1024 * 1024;

/// Member conversations, shared by every HTTP model entry so a member can
/// fork across them.
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

/// The broker item an endpoint's key comes from, and where to fetch it.
#[derive(Debug, Clone)]
pub struct HttpKey {
    pub item: KeyItem,
    pub source: Arc<dyn KeySource>,
}

#[derive(Debug, Clone)]
pub struct HttpAdapter {
    /// The API base, e.g. `http://10.233.1.1:47821/v1`.
    pub base_url: String,
    /// The provider model name.
    pub model: String,
    /// Sent only when set: OpenAI's newer models reject it.
    pub max_tokens: Option<u32>,
    /// Sent as a bearer token; `None` sends no key.
    pub key: Option<HttpKey>,
    /// Model calls per turn before the turn fails without a final answer.
    pub max_calls: u32,
    pub request_timeout: Duration,
    pub mcp_timeout: Duration,
    pub limits: ProcessLimits,
    histories: Arc<Histories>,
}

impl HttpAdapter {
    pub fn new(
        base_url: impl Into<String>,
        model: impl Into<String>,
        key: Option<HttpKey>,
    ) -> Self {
        Self::with_histories(base_url, model, key, Arc::default())
    }

    pub fn with_histories(
        base_url: impl Into<String>,
        model: impl Into<String>,
        key: Option<HttpKey>,
        histories: Arc<Histories>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            model: model.into(),
            max_tokens: None,
            key,
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
        let key = match self.key.as_ref().map(|key| key.source.get(key.item)) {
            None => None,
            Some(Ok(key)) => Some(key),
            Some(Err(error)) => {
                return TurnOutcome::Failed {
                    reason: format!("{API} key: {error}"),
                };
            }
        };
        let key = key.as_ref().map(|key| key.expose());
        let mut messages = self.histories.get(&request.member);
        messages.push(json!({ "role": "user", "content": request.prompt }));
        match self.converse(&request, key, &mut messages, cancel) {
            Ok(()) => {
                self.histories.set(request.member.clone(), messages);
                TurnOutcome::Completed
            }
            Err(Stop::Cancelled) => TurnOutcome::Cancelled,
            Err(Stop::Quota(retry_after)) => TurnOutcome::QuotaExhausted { retry_after },
            Err(Stop::Failed(reason)) => TurnOutcome::Failed {
                reason: match key {
                    Some(key) => reason.replace(key, "<redacted>"),
                    None => reason,
                },
            },
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
        key: Option<&str>,
        messages: &mut Vec<Value>,
        cancel: &Cancel,
    ) -> Result<(), Stop> {
        let mut mcp = McpClient::start(
            &request.mcp_command,
            &request.env,
            &request.cwd,
            self.limits.clone(),
            self.mcp_timeout,
            cancel,
        )?;
        let mut tools = mcp.list_tools(cancel)?;
        if let Some(clash) = tools
            .iter()
            .find(|tool| WorkspaceTool::parse(&tool.name).is_some())
        {
            return Err(Stop::Failed(format!(
                "swarm MCP tool `{}` collides with a workspace tool",
                clash.name
            )));
        }
        tools.extend(WorkspaceTool::ALL.map(WorkspaceTool::declaration));
        let workspace = Workspace::open(&request.cwd, request.env.clone(), self.limits.clone())
            .map_err(Stop::Failed)?;
        for _ in 0..self.max_calls {
            let body = self.body(messages, &tools);
            let response = self.post(key, body, cancel)?;
            let reply = Self::parse(&response)?;
            messages.push(reply.message);
            if reply.calls.is_empty() {
                return Ok(());
            }
            for call in reply.calls {
                let called = match (WorkspaceTool::parse(&call.name), call.arguments) {
                    (_, Err(message)) => Err(McpError::Rpc {
                        method: call.name.clone(),
                        message,
                    }),
                    (Some(tool), Ok(arguments)) => match WorkspaceCall::parse(tool, arguments) {
                        Ok(parsed) => workspace
                            .run(parsed, cancel)
                            .map_err(|Cancelled| McpError::Cancelled),
                        Err(message) => Err(McpError::Rpc {
                            method: call.name.clone(),
                            message,
                        }),
                    },
                    (None, Ok(arguments)) => mcp.call_tool(&call.name, arguments, cancel),
                };
                let result = match called {
                    Ok(result) => result,
                    Err(McpError::Cancelled) => return Err(Stop::Cancelled),
                    Err(McpError::Rpc { message, .. }) => CallResult {
                        text: message,
                        is_error: true,
                    },
                    Err(other) => return Err(other.into()),
                };
                messages.push(
                    json!({ "role": "tool", "tool_call_id": call.id, "content": result.text }),
                );
            }
        }
        Err(Stop::Failed(format!(
            "no final answer after {} model calls",
            self.max_calls
        )))
    }

    fn body(&self, messages: &[Value], tools: &[Tool]) -> Value {
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
            "stream": false,
        });
        if let Some(max_tokens) = self.max_tokens {
            body["max_tokens"] = json!(max_tokens);
        }
        body
    }

    /// Posts on a worker thread so cancellation returns at once; an
    /// abandoned request ends on its own timeout.
    fn post(&self, key: Option<&str>, body: Value, cancel: &Cancel) -> Result<Value, Stop> {
        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        let authorization = key.map(|key| format!("Bearer {key}"));
        let agent = agent(self.request_timeout);
        let (send, receive) = mpsc::channel();
        thread::spawn(move || {
            let mut call = agent.post(&url).header("content-type", "application/json");
            if let Some(authorization) = authorization {
                call = call.header("authorization", authorization);
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
        let (status, retry_after, text) =
            received.map_err(|error| Stop::Failed(format!("{API} request: {error}")))?;
        let text = text.map_err(|error| Stop::Failed(format!("{API} response body: {error}")))?;
        if status == 429 {
            return Err(Stop::Quota(
                retry_after.map_or(DEFAULT_RETRY_AFTER, Duration::from_secs),
            ));
        }
        if !(200..300).contains(&status) {
            return Err(Stop::Failed(format!(
                "{API} returned HTTP {status}: {}",
                excerpt(&text)
            )));
        }
        serde_json::from_str(&text).map_err(|error| {
            Stop::Failed(format!(
                "{API} returned invalid JSON ({error}): {}",
                excerpt(&text)
            ))
        })
    }

    fn parse(response: &Value) -> Result<Reply, Stop> {
        let malformed = || {
            Stop::Failed(format!(
                "{API} returned an unexpected reply: {}",
                excerpt(&response.to_string())
            ))
        };
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
                        arguments: serde_json::from_str(arguments)
                            .map_err(|error| format!("arguments are not valid JSON: {error}")),
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

fn agent(timeout: Duration) -> ureq::Agent {
    ureq::Agent::config_builder()
        .http_status_as_error(false)
        .timeout_global(Some(timeout))
        .build()
        .into()
}

/// The model ids `GET {base_url}/models` lists. Errors never quote the
/// response body, which could echo the key.
pub fn served_models(
    base_url: &str,
    key: Option<&str>,
    timeout: Duration,
) -> Result<BTreeSet<String>, String> {
    let url = format!("{}/models", base_url.trim_end_matches('/'));
    let mut call = agent(timeout).get(&url);
    if let Some(key) = key {
        call = call.header("authorization", format!("Bearer {key}"));
    }
    let mut response = call.call().map_err(|error| error.to_string())?;
    let status = response.status().as_u16();
    if !(200..300).contains(&status) {
        return Err(format!("GET {url} returned HTTP {status}"));
    }
    let text = response
        .body_mut()
        .with_config()
        .limit(MAX_RESPONSE)
        .read_to_string()
        .map_err(|error| format!("GET {url}: {error}"))?;
    let list: Value = serde_json::from_str(&text).map_err(|error| format!("GET {url}: {error}"))?;
    let data = list["data"]
        .as_array()
        .ok_or_else(|| format!("GET {url} returned no `data` list"))?;
    Ok(data
        .iter()
        .filter_map(|model| model["id"].as_str().map(str::to_owned))
        .collect())
}
