use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use serde_json::{Value, json};
use tempfile::TempDir;

use super::cli::golden_args;
use super::*;

const KEY: &str = "sk-test-secret-key";

fn name(text: &str) -> MemberName {
    MemberName::parse(text).expect("valid name")
}

fn request(dir: &Path, mcp_command: Vec<String>, env: Vec<(String, String)>) -> TurnRequest {
    TurnRequest {
        member: name("scout"),
        model: Some("m".into()),
        prompt: "do the thing".into(),
        cwd: dir.to_owned(),
        mcp_command,
        env,
    }
}

fn swarm_mcp() -> Vec<String> {
    [
        "coda-swarm",
        "mcp",
        "--dir",
        "/swarm/s1",
        "--agent",
        "scout",
    ]
    .map(String::from)
    .to_vec()
}

fn script(dir: &Path, file: &str, body: &str) -> PathBuf {
    let path = dir.join(file);
    fs::write(&path, body).expect("script");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("chmod");
    path
}

#[test]
fn claude_command_line() {
    let dir = TempDir::new().expect("tempdir");
    let adapter = CliAdapter::new(Cli::Claude, "opus");
    let req = request(dir.path(), swarm_mcp(), Vec::new());
    let config = r#"{"mcpServers":{"coda-swarm":{"args":["mcp","--dir","/swarm/s1","--agent","scout"],"command":"coda-swarm","type":"stdio"}}}"#;
    let base = [
        "-p",
        "--output-format",
        "stream-json",
        "--verbose",
        "--model",
        "opus",
        "--mcp-config",
        config,
        "--strict-mcp-config",
        "--permission-mode",
        "bypassPermissions",
    ];
    assert_eq!(golden_args(&adapter, &req, None), base);
    let mut resumed = base.to_vec();
    resumed.extend(["--resume", "s-1"]);
    assert_eq!(golden_args(&adapter, &req, Some(("s-1", false))), resumed);
    let mut forked = base.to_vec();
    forked.extend(["--resume", "s-1", "--fork-session"]);
    assert_eq!(golden_args(&adapter, &req, Some(("s-1", true))), forked);
}

#[test]
fn codex_command_line() {
    let dir = TempDir::new().expect("tempdir");
    let adapter = CliAdapter::new(Cli::Codex, "gpt-5-codex");
    let req = request(dir.path(), swarm_mcp(), Vec::new());
    let options = [
        "--json",
        "--skip-git-repo-check",
        "--ignore-user-config",
        "--dangerously-bypass-approvals-and-sandbox",
        "-m",
        "gpt-5-codex",
        "-c",
        r#"mcp_servers.coda-swarm.command="coda-swarm""#,
        "-c",
        r#"mcp_servers.coda-swarm.args=["mcp","--dir","/swarm/s1","--agent","scout"]"#,
    ];
    let line = |head: &[&str], tail: &[&str]| -> Vec<String> {
        head.iter()
            .chain(&options)
            .chain(tail)
            .map(|arg| (*arg).to_owned())
            .collect()
    };
    assert_eq!(golden_args(&adapter, &req, None), line(&["exec"], &["-"]));
    assert_eq!(
        golden_args(&adapter, &req, Some(("t-1", false))),
        line(&["exec", "resume"], &["t-1", "-"])
    );
    assert_eq!(
        golden_args(&adapter, &req, Some(("t-1", true))),
        line(&["exec", "fork"], &["t-1", "-"])
    );
}

#[test]
fn gemini_command_line_and_no_fork() {
    let dir = TempDir::new().expect("tempdir");
    let adapter = CliAdapter::new(Cli::Gemini, "gemini-2.5-pro");
    let req = request(dir.path(), swarm_mcp(), Vec::new());
    assert_eq!(
        golden_args(&adapter, &req, None),
        [
            "--model",
            "gemini-2.5-pro",
            "--yolo",
            "--output-format",
            "json",
            "--allowed-mcp-server-names",
            "coda-swarm",
        ]
    );
    assert_eq!(
        adapter.fork(&name("scout"), &name("twin"), None),
        Err(ForkError::Unsupported { adapter: "gemini" })
    );
}

/// A fake `claude` that prints a stream-json session, and a usage-limit
/// result when its prompt says so. Records its args to `$PWD/args`.
const FAKE_CLAUDE: &str = r#"#!/bin/sh
printf '%s\n' "$@" > args
prompt=$(cat)
echo '{"type":"system","subtype":"init","session_id":"sess-42"}'
case "$prompt" in
  *limit*)
    echo '{"type":"result","is_error":true,"result":"Claude AI usage limit reached|4102444800","session_id":"sess-42"}'
    exit 1 ;;
  *)
    echo '{"type":"result","is_error":false,"result":"done","session_id":"sess-42"}' ;;
esac
"#;

#[test]
fn claude_session_forks_and_usage_limit_is_quota() {
    let dir = TempDir::new().expect("tempdir");
    let mut adapter = CliAdapter::new(Cli::Claude, "opus");
    adapter.program = script(dir.path(), "claude", FAKE_CLAUDE);
    let req = request(dir.path(), swarm_mcp(), Vec::new());

    assert_eq!(
        adapter.fork(&name("scout"), &name("twin"), None),
        Err(ForkError::NoHistory(name("scout")))
    );
    assert_eq!(
        adapter.run_turn(req.clone(), &Cancel::default()),
        TurnOutcome::Completed
    );
    adapter
        .fork(&name("scout"), &name("twin"), None)
        .expect("fork");
    let twin = TurnRequest {
        member: name("twin"),
        prompt: "hit the limit".into(),
        ..req
    };
    let outcome = adapter.run_turn(twin, &Cancel::default());
    let args = fs::read_to_string(dir.path().join("args")).expect("args");
    assert!(
        args.ends_with("--resume\nsess-42\n--fork-session\n"),
        "twin's first turn forks scout's session: {args}"
    );
    let TurnOutcome::QuotaExhausted { retry_after } = outcome else {
        panic!("usage limit maps to quota: {outcome:?}");
    };
    assert!(
        retry_after > Duration::from_secs(3600),
        "retry waits for the reported reset: {retry_after:?}"
    );
}

/// A stdio MCP server with one `post` tool. Each call appends its request
/// line to `$1/calls` and answers `posted`.
const STUB_MCP: &str = r#"#!/bin/sh
dir="$1"
while IFS= read -r line; do
  id=$(printf '%s' "$line" | sed -n 's/^{"id":\([0-9]*\),.*/\1/p')
  [ -z "$id" ] && continue
  case "$line" in
    *'"method":"initialize"'*)
      echo "{\"jsonrpc\":\"2.0\",\"id\":$id,\"result\":{\"protocolVersion\":\"2025-06-18\",\"capabilities\":{\"tools\":{}},\"serverInfo\":{\"name\":\"stub\",\"version\":\"0\"}}}" ;;
    *'"method":"tools/list"'*)
      echo "{\"jsonrpc\":\"2.0\",\"id\":$id,\"result\":{\"tools\":[{\"name\":\"post\",\"description\":\"Post to the board\",\"inputSchema\":{\"type\":\"object\",\"properties\":{\"text\":{\"type\":\"string\"}}}}]}}" ;;
    *'"method":"tools/call"'*)
      printf '%s\n' "$line" >> "$dir/calls"
      env > "$dir/env"
      echo "{\"jsonrpc\":\"2.0\",\"id\":$id,\"result\":{\"content\":[{\"type\":\"text\",\"text\":\"posted\"}]}}" ;;
  esac
done
"#;

struct Canned {
    status: u16,
    headers: Vec<(&'static str, String)>,
    body: Value,
}

fn ok(body: Value) -> Canned {
    Canned {
        status: 200,
        headers: Vec::new(),
        body,
    }
}

struct Exchange {
    head: String,
    body: Value,
}

/// Answers each connection with the next canned response and records the
/// request.
struct FakeServer {
    url: String,
    seen: Arc<Mutex<Vec<Exchange>>>,
    thread: JoinHandle<()>,
}

impl FakeServer {
    fn start(responses: Vec<Canned>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let url = format!("http://{}", listener.local_addr().expect("addr"));
        let seen = Arc::new(Mutex::new(Vec::new()));
        let record = Arc::clone(&seen);
        let thread = thread::spawn(move || {
            for Canned {
                status,
                headers,
                body,
            } in responses
            {
                let (stream, _) = listener.accept().expect("accept");
                let mut reader = BufReader::new(stream);
                let mut head = String::new();
                let mut length = 0;
                loop {
                    let mut line = String::new();
                    reader.read_line(&mut line).expect("header");
                    if let Some(value) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                        length = value.trim().parse().expect("length");
                    }
                    if line == "\r\n" {
                        break;
                    }
                    head.push_str(&line);
                }
                let mut raw = vec![0; length];
                reader.read_exact(&mut raw).expect("body");
                record.lock().expect("seen").push(Exchange {
                    head,
                    body: serde_json::from_slice(&raw).expect("json body"),
                });
                let text = body.to_string();
                let extra: String = headers
                    .iter()
                    .map(|(name, value)| format!("{name}: {value}\r\n"))
                    .collect();
                let reply = format!(
                    "HTTP/1.1 {status} X\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n{extra}\r\n{text}",
                    text.len()
                );
                reader.get_mut().write_all(reply.as_bytes()).expect("reply");
            }
        });
        Self { url, seen, thread }
    }

    fn finish(self) -> Vec<Exchange> {
        self.thread.join().expect("server");
        Arc::try_unwrap(self.seen)
            .ok()
            .expect("sole owner")
            .into_inner()
            .expect("seen")
    }
}

fn http_turn(dir: &Path, api: Api, key: Option<&str>) -> (HttpAdapter, TurnRequest) {
    let stub = script(dir, "mcp.sh", STUB_MCP);
    let mut adapter = HttpAdapter::new(api, "test-model", key.map(|_| "PROVIDER_KEY".into()));
    adapter.limits.poll = Duration::from_millis(5);
    adapter.limits.kill_grace = Duration::from_millis(200);
    adapter.mcp_timeout = Duration::from_secs(10);
    adapter.request_timeout = Duration::from_secs(10);
    let mut env = vec![("CODA_SWARM_AGENT".to_owned(), "scout".to_owned())];
    if let Some(key) = key {
        env.push(("PROVIDER_KEY".into(), key.into()));
    }
    // Through sh: exec of a script another test thread's fork may still hold
    // open for writing fails with ETXTBSY.
    let mcp = vec![
        "/bin/sh".to_owned(),
        stub.display().to_string(),
        dir.display().to_string(),
    ];
    (adapter, request(dir, mcp, env))
}

#[test]
fn anthropic_runs_a_tool_call_through_mcp() {
    let dir = TempDir::new().expect("tempdir");
    let server = FakeServer::start(vec![
        ok(json!({ "content": [
            { "type": "text", "text": "posting" },
            { "type": "tool_use", "id": "tu_1", "name": "post", "input": { "text": "hello board" } }
        ]})),
        ok(json!({ "content": [{ "type": "text", "text": "all done" }] })),
    ]);
    let api = Api::Anthropic {
        base_url: server.url.clone(),
        max_tokens: 1024,
    };
    let (adapter, req) = http_turn(dir.path(), api, Some(KEY));
    assert_eq!(
        adapter.run_turn(req, &Cancel::default()),
        TurnOutcome::Completed
    );
    let seen = server.finish();

    let calls = fs::read_to_string(dir.path().join("calls")).expect("calls");
    assert!(
        calls.contains(r#""arguments":{"text":"hello board"}"#),
        "{calls}"
    );
    let mcp_env = fs::read_to_string(dir.path().join("env")).expect("env");
    assert!(!mcp_env.contains(KEY), "the MCP server never sees the key");
    let first = &seen[0];
    assert!(
        first.head.starts_with("POST /v1/messages "),
        "{}",
        first.head
    );
    assert!(first.head.contains(&format!("x-api-key: {KEY}")));
    assert!(first.head.contains("anthropic-version: 2023-06-01"));
    assert_eq!(first.body["tools"][0]["name"], "post");
    assert_eq!(first.body["tools"][0]["input_schema"]["type"], "object");
    assert_eq!(first.body["messages"][0]["content"], "do the thing");
    let result = &seen[1].body["messages"][2]["content"][0];
    assert_eq!(result["type"], "tool_result");
    assert_eq!(result["tool_use_id"], "tu_1");
    assert_eq!(result["content"], "posted");
}

#[test]
fn openai_runs_a_tool_call_and_keeps_history() {
    let dir = TempDir::new().expect("tempdir");
    let server = FakeServer::start(vec![
        ok(json!({ "choices": [{ "message": {
            "role": "assistant",
            "content": null,
            "tool_calls": [{ "id": "call_1", "type": "function",
                "function": { "name": "post", "arguments": "{\"text\":\"hi\"}" } }]
        }}]})),
        ok(json!({ "choices": [{ "message": { "role": "assistant", "content": "ok" } }] })),
        ok(json!({ "choices": [{ "message": { "role": "assistant", "content": "again" } }] })),
    ]);
    let api = Api::OpenAi {
        base_url: format!("{}/v1", server.url),
        max_tokens: None,
    };
    let (adapter, req) = http_turn(dir.path(), api, None);
    assert_eq!(
        adapter.run_turn(req.clone(), &Cancel::default()),
        TurnOutcome::Completed
    );
    adapter
        .fork(&name("scout"), &name("twin"), None)
        .expect("fork");
    let twin = TurnRequest {
        member: name("twin"),
        prompt: "next".into(),
        ..req
    };
    assert_eq!(
        adapter.run_turn(twin, &Cancel::default()),
        TurnOutcome::Completed
    );
    let seen = server.finish();

    assert!(seen[0].head.starts_with("POST /v1/chat/completions "));
    assert!(!seen[0].head.to_ascii_lowercase().contains("authorization"));
    assert_eq!(seen[0].body["tools"][0]["function"]["name"], "post");
    assert!(seen[0].body.get("max_tokens").is_none());
    let tool = &seen[1].body["messages"][2];
    assert_eq!(tool["role"], "tool");
    assert_eq!(tool["tool_call_id"], "call_1");
    assert_eq!(tool["content"], "posted");
    let forked = seen[2].body["messages"].as_array().expect("messages");
    assert_eq!(forked.len(), 5, "twin continues scout's four messages");
    assert_eq!(forked[4]["content"], "next");
}

#[test]
fn http_429_is_quota_and_errors_never_show_the_key() {
    let dir = TempDir::new().expect("tempdir");
    let server = FakeServer::start(vec![
        Canned {
            status: 429,
            headers: vec![("retry-after", "7".into())],
            body: json!({ "error": { "message": "rate limited" } }),
        },
        Canned {
            status: 500,
            headers: Vec::new(),
            body: json!({ "error": { "message": format!("bad key {KEY}") } }),
        },
    ]);
    let api = Api::OpenAi {
        base_url: server.url.clone(),
        max_tokens: Some(256),
    };
    let (adapter, req) = http_turn(dir.path(), api, Some(KEY));
    assert_eq!(
        adapter.run_turn(req.clone(), &Cancel::default()),
        TurnOutcome::QuotaExhausted {
            retry_after: Duration::from_secs(7)
        }
    );
    let failed = adapter.run_turn(req, &Cancel::default());
    let seen = server.finish();
    assert!(
        seen[0]
            .head
            .contains(&format!("authorization: Bearer {KEY}"))
    );
    assert_eq!(seen[0].body["max_tokens"], 256);
    let TurnOutcome::Failed { reason } = failed else {
        panic!("HTTP 500 fails the turn: {failed:?}");
    };
    assert!(reason.contains("HTTP 500"), "{reason}");
    assert!(!reason.contains(KEY), "{reason}");
}

#[test]
fn example_registry_parses_and_lists_its_models() {
    let registry = Registry::parse(include_str!("../../models.example.json")).expect("example");
    let kinds: Vec<(&str, &AdapterKind)> = registry
        .models
        .iter()
        .map(|(id, entry)| (id.as_str(), &entry.kind))
        .collect();
    assert!(matches!(
        kinds[..],
        [
            ("claude-opus", AdapterKind::ClaudeCli),
            ("claude-sonnet-api", AdapterKind::AnthropicHttp { .. }),
            ("gemini-pro", AdapterKind::GeminiCli),
            ("gpt-codex", AdapterKind::CodexCli),
            ("qwen-coder-local", AdapterKind::OpenaiHttp { .. }),
            ("qwen-coder-openrouter", AdapterKind::OpenaiHttp { .. }),
        ]
    ));
    let local = &registry.models[&ModelId::new("qwen-coder-local")];
    assert_eq!(local.key, None);
    assert_eq!(
        local.kind,
        AdapterKind::OpenaiHttp {
            base_url: "http://10.233.1.1:11434/v1".into(),
            max_tokens: None,
            max_calls: 32,
        }
    );
    let adapter = RegistryAdapter::new(registry);
    assert!(adapter.models().admits("gpt-codex"));
    assert!(!adapter.models().admits("gpt-4"));
    assert_eq!(adapter.keys(Some("claude-opus")), []);
    assert_eq!(
        adapter.keys(Some("claude-sonnet-api")),
        [KeyEnv {
            var: "ANTHROPIC_API_KEY".into(),
            item: "anthropic".into()
        }]
    );
    assert_eq!(
        adapter.fork(&name("a"), &name("b"), Some("nope")),
        Err(ForkError::Model(ResolveError::Unknown("nope".into())))
    );
}

#[test]
fn registry_rejects_unknown_adapters_and_fields() {
    let unknown = r#"{"models":{"x":{"adapter":"telepathy","model":"m"}}}"#;
    assert!(Registry::parse(unknown).is_err());
    let missing_url = r#"{"models":{"x":{"adapter":"openai-http","model":"m"}}}"#;
    assert!(Registry::parse(missing_url).is_err());
    let typo = r#"{"model":{}}"#;
    assert!(Registry::parse(typo).is_err());
}
