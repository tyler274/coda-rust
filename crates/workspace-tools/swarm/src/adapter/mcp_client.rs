//! A minimal stdio MCP client for HTTP adapters: start the swarm MCP
//! command, list its tools, and call them. One request in flight at a time.

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use super::Cancel;
use super::process::{self, ProcessLimits};

const PROTOCOL_VERSION: &str = "2025-06-18";

#[derive(Debug, Clone, PartialEq)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

/// The text of a tool call result, and whether the tool reported an error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallResult {
    pub text: String,
    pub is_error: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum McpError {
    #[error("the MCP command is empty")]
    NoCommand,
    #[error("start MCP command {program}")]
    Start {
        program: String,
        #[source]
        source: std::io::Error,
    },
    #[error("write to the MCP server")]
    Write(#[source] std::io::Error),
    #[error("the MCP server closed its output")]
    Closed,
    #[error("the MCP server sent {0}")]
    Malformed(String),
    #[error("MCP {method} failed: {message}")]
    Rpc { method: String, message: String },
    #[error("cancelled")]
    Cancelled,
    #[error("the MCP server did not answer {method} within {after:?}")]
    TimedOut { method: String, after: Duration },
}

pub struct McpClient {
    child: Child,
    stdin: Option<ChildStdin>,
    lines: Receiver<String>,
    next_id: u64,
    limits: ProcessLimits,
    call_timeout: Duration,
}

impl McpClient {
    /// Starts `command` in its own process group and runs the MCP
    /// handshake.
    pub fn start(
        command: &[String],
        env: &[(String, String)],
        cwd: &std::path::Path,
        limits: ProcessLimits,
        call_timeout: Duration,
        cancel: &Cancel,
    ) -> Result<Self, McpError> {
        use std::os::unix::process::CommandExt;
        let (program, args) = command.split_first().ok_or(McpError::NoCommand)?;
        let mut child = Command::new(program)
            .args(args)
            .envs(env.iter().map(|(name, value)| (name, value)))
            .current_dir(cwd)
            .process_group(0)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|source| McpError::Start {
                program: program.clone(),
                source,
            })?;
        let (Some(stdin), Some(stdout)) = (child.stdin.take(), child.stdout.take()) else {
            return Err(McpError::Closed);
        };
        let mut client = Self {
            child,
            stdin: Some(stdin),
            lines: read_lines(stdout),
            next_id: 0,
            limits,
            call_timeout,
        };
        client.request(
            "initialize",
            json!({
                "protocolVersion": PROTOCOL_VERSION,
                "capabilities": {},
                "clientInfo": { "name": "coda-swarm", "version": env!("CARGO_PKG_VERSION") }
            }),
            cancel,
        )?;
        client.notify("notifications/initialized")?;
        Ok(client)
    }

    pub fn list_tools(&mut self, cancel: &Cancel) -> Result<Vec<Tool>, McpError> {
        let mut tools = Vec::new();
        let mut cursor: Option<String> = None;
        loop {
            let params = cursor.map_or_else(|| json!({}), |cursor| json!({ "cursor": cursor }));
            let page = self.request("tools/list", params, cancel)?;
            let Some(listed) = page["tools"].as_array() else {
                return Err(McpError::Malformed(format!(
                    "a tools/list result without tools: {page}"
                )));
            };
            for tool in listed {
                let Some(name) = tool["name"].as_str() else {
                    return Err(McpError::Malformed(format!(
                        "a tool without a name: {tool}"
                    )));
                };
                tools.push(Tool {
                    name: name.to_owned(),
                    description: tool["description"].as_str().unwrap_or_default().to_owned(),
                    input_schema: tool
                        .get("inputSchema")
                        .cloned()
                        .unwrap_or_else(|| json!({ "type": "object" })),
                });
            }
            cursor = page["nextCursor"].as_str().map(str::to_owned);
            if cursor.is_none() {
                return Ok(tools);
            }
        }
    }

    pub fn call_tool(
        &mut self,
        name: &str,
        arguments: Value,
        cancel: &Cancel,
    ) -> Result<CallResult, McpError> {
        let result = self.request(
            "tools/call",
            json!({ "name": name, "arguments": arguments }),
            cancel,
        )?;
        let text = result["content"]
            .as_array()
            .map(|blocks| {
                blocks
                    .iter()
                    .map(|block| match block["text"].as_str() {
                        Some(text) => text.to_owned(),
                        None => block.to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .unwrap_or_default();
        Ok(CallResult {
            text,
            is_error: result["isError"] == true,
        })
    }

    fn notify(&mut self, method: &str) -> Result<(), McpError> {
        self.send(&json!({ "jsonrpc": "2.0", "method": method }))
    }

    fn request(&mut self, method: &str, params: Value, cancel: &Cancel) -> Result<Value, McpError> {
        self.next_id += 1;
        let id = self.next_id;
        self.send(&json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params }))?;
        let deadline = Instant::now() + self.call_timeout;
        loop {
            if cancel.is_cancelled() {
                return Err(McpError::Cancelled);
            }
            let now = Instant::now();
            if now >= deadline {
                return Err(McpError::TimedOut {
                    method: method.to_owned(),
                    after: self.call_timeout,
                });
            }
            let wait = self.limits.poll.min(deadline - now);
            let line = match self.lines.recv_timeout(wait) {
                Ok(line) => line,
                Err(RecvTimeoutError::Timeout) => continue,
                Err(RecvTimeoutError::Disconnected) => return Err(McpError::Closed),
            };
            let Ok(message) = serde_json::from_str::<Value>(&line) else {
                return Err(McpError::Malformed(format!("a non-JSON line: {line}")));
            };
            if message["id"] != id {
                continue;
            }
            if let Some(error) = message.get("error") {
                return Err(McpError::Rpc {
                    method: method.to_owned(),
                    message: error["message"]
                        .as_str()
                        .map_or_else(|| error.to_string(), str::to_owned),
                });
            }
            return Ok(message["result"].clone());
        }
    }

    fn send(&mut self, message: &Value) -> Result<(), McpError> {
        let mut line = message.to_string();
        line.push('\n');
        let Some(stdin) = self.stdin.as_mut() else {
            return Err(McpError::Closed);
        };
        stdin
            .write_all(line.as_bytes())
            .and_then(|()| stdin.flush())
            .map_err(McpError::Write)
    }
}

impl Drop for McpClient {
    /// Closing stdin asks the server to exit; the group is killed after
    /// the grace period either way.
    fn drop(&mut self) {
        drop(self.stdin.take());
        if let Ok(pid) = process::child_pid(&self.child) {
            // Nothing can report a failure from drop.
            let _ = process::wait_then_reap(&self.limits, &mut self.child, pid);
        }
    }
}

fn read_lines(stdout: ChildStdout) -> Receiver<String> {
    let (send, receive) = mpsc::channel();
    thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            let Ok(line) = line else { return };
            if line.trim().is_empty() {
                continue;
            }
            if send.send(line).is_err() {
                return;
            }
        }
    });
    receive
}
