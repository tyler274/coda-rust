//! The board as a stdio MCP server: newline-delimited JSON-RPC 2.0 with
//! `initialize`, `tools/list` and `tools/call`. One server process is one
//! member's connection.

mod server;
mod spawner;
mod tools;

use std::error::Error;
use std::io::{self, BufRead, Write};

use serde::Deserialize;
use serde_json::{Value, json};

pub use server::{Server, ToolError};
pub use spawner::{
    Catalog, ModelInfo, RosterOnlySpawner, SpawnError, SpawnSpec, Spawner, Workspace,
};
pub use tools::Tool;

use crate::store::Store;

pub const PROTOCOL_VERSION: &str = "2025-06-18";

const PARSE_ERROR: i64 = -32700;
const INVALID_REQUEST: i64 = -32600;
const METHOD_NOT_FOUND: i64 = -32601;
const INVALID_PARAMS: i64 = -32602;

#[derive(Deserialize)]
struct Request {
    jsonrpc: String,
    /// Absent for a notification, which gets no reply.
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Deserialize)]
struct CallParams {
    name: String,
    #[serde(default)]
    arguments: Option<Value>,
}

fn error(id: Value, code: i64, message: impl Into<String>) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message.into() } })
}

fn result(id: Value, result: Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "result": result })
}

/// The error and each of its causes, so the agent sees why.
fn chain(error: &dyn Error) -> String {
    let mut text = error.to_string();
    let mut source = error.source();
    while let Some(cause) = source {
        text.push_str(": ");
        text.push_str(&cause.to_string());
        source = cause.source();
    }
    text
}

impl<S: Store, P: Spawner> Server<S, P> {
    /// Serves requests from `input` until it closes.
    pub fn serve(&self, input: impl BufRead, mut output: impl Write) -> io::Result<()> {
        for line in input.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            if let Some(reply) = self.handle(&line) {
                serde_json::to_writer(&mut output, &reply)?;
                output.write_all(b"\n")?;
                output.flush()?;
            }
        }
        Ok(())
    }

    fn handle(&self, line: &str) -> Option<Value> {
        let request: Request = match serde_json::from_str(line) {
            Ok(request) => request,
            Err(source) => {
                let code = if serde_json::from_str::<Value>(line).is_ok() {
                    INVALID_REQUEST
                } else {
                    PARSE_ERROR
                };
                return Some(error(Value::Null, code, source.to_string()));
            }
        };
        let id = request.id?;
        if request.jsonrpc != "2.0" {
            return Some(error(id, INVALID_REQUEST, "jsonrpc must be \"2.0\""));
        }
        Some(match request.method.as_str() {
            "initialize" => result(
                id,
                json!({
                    "protocolVersion": PROTOCOL_VERSION,
                    "capabilities": { "tools": {} },
                    "serverInfo": { "name": "coda-swarm", "version": env!("CARGO_PKG_VERSION") },
                }),
            ),
            "ping" => result(id, json!({})),
            "tools/list" => result(id, json!({ "tools": Tool::ALL.map(Tool::descriptor) })),
            "tools/call" => match serde_json::from_value::<CallParams>(request.params) {
                Err(source) => error(id, INVALID_PARAMS, source.to_string()),
                Ok(call) => match Tool::parse(&call.name) {
                    None => error(id, INVALID_PARAMS, format!("unknown tool {}", call.name)),
                    Some(tool) => result(id, self.tool_result(tool, call.arguments)),
                },
            },
            other => error(id, METHOD_NOT_FOUND, format!("unknown method {other}")),
        })
    }

    fn tool_result(&self, tool: Tool, arguments: Option<Value>) -> Value {
        let arguments = arguments.unwrap_or_else(|| json!({}));
        match self.call(tool, arguments) {
            Ok(value) => json!({
                "content": [{ "type": "text", "text": value.to_string() }],
                "structuredContent": value,
                "isError": false,
            }),
            Err(failure) => json!({
                "content": [{ "type": "text", "text": chain(&failure) }],
                "isError": true,
            }),
        }
    }
}
