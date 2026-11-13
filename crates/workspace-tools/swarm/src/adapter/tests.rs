use std::collections::BTreeMap;
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

/// A fake `claude` that prints a stream-json session, and a usage-limit
/// result when its prompt says so. Records its args to `$PWD/args`.
const FAKE_CLAUDE: &str = r#"#!/bin/sh
printf '%s\n' "$@" > args
prompt=$(cat)
echo '{"type":"system","subtype":"init","session_id":"sess-42"}'
case "$prompt" in
  *limit*)
    echo '{"type":"rate_limit_event","rate_limit_info":{"status":"rejected","resetsAt":4102444800,"rateLimitType":"seven_day"},"session_id":"sess-42"}'
    echo '{"type":"result","is_error":true,"result":"You have hit your weekly limit","session_id":"sess-42"}'
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
                    body: if raw.is_empty() {
                        Value::Null
                    } else {
                        serde_json::from_slice(&raw).expect("json body")
                    },
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

/// Hands out [`KEY`] for every item and counts the fetches.
#[derive(Debug, Default)]
struct FakeKeys(Mutex<Vec<KeyItem>>);

impl KeySource for FakeKeys {
    fn get(&self, item: KeyItem) -> Result<Secret, KeyError> {
        self.0.lock().expect("fetches").push(item);
        Ok(Secret::new(KEY))
    }
}

fn http_turn(dir: &Path, base_url: String, key: bool) -> (HttpAdapter, TurnRequest) {
    let stub = script(dir, "mcp.sh", STUB_MCP);
    let key = key.then(|| HttpKey {
        item: KeyItem::ModelProxy,
        source: Arc::new(FakeKeys::default()),
    });
    let mut adapter = HttpAdapter::new(base_url, "test-model", key);
    adapter.limits.poll = Duration::from_millis(5);
    adapter.limits.kill_grace = Duration::from_millis(200);
    adapter.mcp_timeout = Duration::from_secs(10);
    adapter.request_timeout = Duration::from_secs(10);
    let env = vec![("CODA_SWARM_AGENT".to_owned(), "scout".to_owned())];
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
    let (adapter, req) = http_turn(dir.path(), format!("{}/v1", server.url), false);
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
fn model_proxy_key_is_a_bearer_header_the_mcp_server_never_sees() {
    let dir = TempDir::new().expect("tempdir");
    let server = FakeServer::start(vec![
        ok(json!({ "choices": [{ "message": {
            "role": "assistant",
            "content": null,
            "tool_calls": [{ "id": "call_1", "type": "function",
                "function": { "name": "post", "arguments": "{}" } }]
        }}]})),
        ok(json!({ "choices": [{ "message": { "role": "assistant", "content": "ok" } }] })),
    ]);
    let (mut adapter, req) = http_turn(dir.path(), format!("{}/v1", server.url), true);
    adapter.max_tokens = Some(256);
    assert_eq!(
        adapter.run_turn(req, &Cancel::default()),
        TurnOutcome::Completed
    );
    let seen = server.finish();

    for exchange in &seen {
        assert!(
            exchange
                .head
                .contains(&format!("authorization: Bearer {KEY}")),
            "{}",
            exchange.head
        );
    }
    assert_eq!(seen[0].body["max_tokens"], 256);
    let mcp_env = fs::read_to_string(dir.path().join("env")).expect("env");
    assert!(!mcp_env.contains(KEY), "the MCP server never sees the key");
}

fn calls(calls: &[(&str, Value)]) -> Canned {
    let calls: Vec<Value> = calls
        .iter()
        .enumerate()
        .map(|(index, (name, arguments))| {
            json!({ "id": format!("call_{index}"), "type": "function",
                "function": { "name": name, "arguments": arguments.to_string() } })
        })
        .collect();
    ok(json!({ "choices": [{ "message": {
        "role": "assistant", "content": null, "tool_calls": calls
    }}]}))
}

fn answer() -> Canned {
    ok(json!({ "choices": [{ "message": { "role": "assistant", "content": "done" } }] }))
}

/// The tool results the model saw in the request after its calls.
fn tool_results(exchange: &Exchange) -> Vec<String> {
    exchange.body["messages"]
        .as_array()
        .expect("messages")
        .iter()
        .filter(|message| message["role"] == "tool")
        .map(|message| message["content"].as_str().expect("content").to_owned())
        .collect()
}

#[test]
fn workspace_tools_read_edit_and_run_in_the_workspace() {
    let dir = TempDir::new().expect("tempdir");
    fs::write(dir.path().join("greeting.txt"), "hello world\n").expect("seed");
    let server = FakeServer::start(vec![
        calls(&[
            ("read_file", json!({ "path": "greeting.txt" })),
            (
                "edit_file",
                json!({ "path": "greeting.txt", "old": "world", "new": "swarm" }),
            ),
            ("shell", json!({ "command": "cat greeting.txt; exit 3" })),
            (
                "edit_file",
                json!({ "path": "greeting.txt", "old": "l", "new": "L" }),
            ),
            ("search", json!({ "pattern": "sw.rm", "glob": "*.txt" })),
            (
                "write_file",
                json!({ "path": "notes/abc", "content": "a\nb\nc\n" }),
            ),
            (
                "read_file",
                json!({ "path": "notes/abc", "offset": 2, "limit": 1 }),
            ),
        ]),
        answer(),
    ]);
    let (adapter, req) = http_turn(dir.path(), server.url.clone(), false);
    assert_eq!(
        adapter.run_turn(req, &Cancel::default()),
        TurnOutcome::Completed
    );
    let seen = server.finish();

    let offered: Vec<&str> = seen[0].body["tools"]
        .as_array()
        .expect("tools")
        .iter()
        .map(|tool| tool["function"]["name"].as_str().expect("name"))
        .collect();
    assert_eq!(
        offered,
        [
            "post",
            "read_file",
            "write_file",
            "edit_file",
            "list_dir",
            "search",
            "shell"
        ]
    );
    assert_eq!(seen[0].body["stream"], false);
    let results = tool_results(&seen[1]);
    assert_eq!(results[0], "hello world\n");
    assert_eq!(results[1], "replaced 1 occurrence(s) in greeting.txt");
    assert_eq!(results[2], "hello swarm\n[exit status: 3]");
    assert!(results[3].contains("occurs 2 times"), "{}", results[3]);
    assert_eq!(results[4], "./greeting.txt:1:hello swarm\n");
    assert_eq!(results[6], "b\n\n[truncated: continue with offset 3]");
    assert_eq!(
        fs::read_to_string(dir.path().join("greeting.txt")).expect("edited"),
        "hello swarm\n"
    );
}

#[test]
fn workspace_paths_cannot_escape() {
    let dir = TempDir::new().expect("tempdir");
    let outside = TempDir::new().expect("outside");
    fs::write(outside.path().join("secret"), "s3cret").expect("secret");
    std::os::unix::fs::symlink(outside.path(), dir.path().join("link")).expect("symlink");
    let absolute = outside.path().join("secret").display().to_string();
    let server = FakeServer::start(vec![
        calls(&[
            ("read_file", json!({ "path": "../secret" })),
            ("read_file", json!({ "path": absolute })),
            ("read_file", json!({ "path": "link/secret" })),
            ("write_file", json!({ "path": "link/new", "content": "x" })),
            (
                "write_file",
                json!({ "path": "gone/../../new", "content": "x" }),
            ),
            ("list_dir", json!({ "path": "link" })),
        ]),
        answer(),
    ]);
    let (adapter, req) = http_turn(dir.path(), server.url.clone(), false);
    assert_eq!(
        adapter.run_turn(req, &Cancel::default()),
        TurnOutcome::Completed
    );
    let seen = server.finish();

    let results = tool_results(&seen[1]);
    assert_eq!(results.len(), 6);
    for result in &results {
        assert!(result.ends_with("outside the workspace"), "{result}");
    }
    assert!(!outside.path().join("new").exists());
    assert!(!dir.path().parent().expect("parent").join("new").exists());
}

#[test]
fn shell_output_is_capped_with_a_marker() {
    let dir = TempDir::new().expect("tempdir");
    let server = FakeServer::start(vec![
        calls(&[("shell", json!({ "command": "yes line | head -c 100000" }))]),
        answer(),
    ]);
    let (adapter, req) = http_turn(dir.path(), server.url.clone(), false);
    assert_eq!(
        adapter.run_turn(req, &Cancel::default()),
        TurnOutcome::Completed
    );
    let seen = server.finish();

    let output = &tool_results(&seen[1])[0];
    assert!(
        output.ends_with("[output truncated at 32768 bytes]\n[exit status: 0]"),
        "{}",
        &output[output.len() - 80..]
    );
    assert!(output.len() < 33 * 1024, "{}", output.len());
}

#[test]
fn cancel_stops_a_running_shell_command() {
    let dir = TempDir::new().expect("tempdir");
    let server = FakeServer::start(vec![calls(&[(
        "shell",
        json!({ "command": "sleep 30 & echo $! > sleep.pid; wait" }),
    )])]);
    let (adapter, req) = http_turn(dir.path(), server.url.clone(), false);
    let cancel = Cancel::default();
    let pid_file = dir.path().join("sleep.pid");
    let canceller = {
        let cancel = cancel.clone();
        let pid_file = pid_file.clone();
        thread::spawn(move || {
            while !fs::read_to_string(&pid_file).is_ok_and(|text| text.ends_with('\n')) {
                thread::sleep(Duration::from_millis(5));
            }
            cancel.cancel();
            std::time::Instant::now()
        })
    };
    let outcome = adapter.run_turn(req, &cancel);
    let ended = std::time::Instant::now();
    let cancelled_at = canceller.join().expect("canceller");
    server.finish();

    assert_eq!(outcome, TurnOutcome::Cancelled);
    assert!(
        ended - cancelled_at < Duration::from_secs(2),
        "turn ends promptly after cancel"
    );
    let pid: i32 = fs::read_to_string(&pid_file)
        .expect("pid")
        .trim()
        .parse()
        .expect("pid number");
    let pid = nix::unistd::Pid::from_raw(pid);
    let deadline = std::time::Instant::now() + Duration::from_secs(2);
    while nix::sys::signal::kill(pid, None).is_ok() {
        assert!(
            std::time::Instant::now() < deadline,
            "the shell's sleep outlived the turn"
        );
        thread::sleep(Duration::from_millis(10));
    }
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
    let (adapter, req) = http_turn(dir.path(), server.url.clone(), true);
    assert_eq!(
        adapter.run_turn(req.clone(), &Cancel::default()),
        TurnOutcome::QuotaExhausted {
            retry_after: Duration::from_secs(7)
        }
    );
    let failed = adapter.run_turn(req, &Cancel::default());
    server.finish();
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
            (
                "gemini-proxy",
                AdapterKind::OpenaiHttp {
                    key: Some(KeyItem::ModelProxy),
                    ..
                }
            ),
            ("gpt-codex", AdapterKind::CodexCli),
            (
                "grok-proxy",
                AdapterKind::OpenaiHttp {
                    key: Some(KeyItem::ModelProxy),
                    ..
                }
            ),
            ("qwen-local", AdapterKind::OpenaiHttp { key: None, .. }),
        ]
    ));
    let adapter = RegistryAdapter::new(registry, Arc::new(FakeKeys::default()));
    assert!(adapter.models().admits("gpt-codex"));
    assert!(!adapter.models().admits("gpt-4"));
    assert_eq!(
        adapter.fork(&name("a"), &name("b"), Some("nope")),
        Err(ForkError::Model(ResolveError::Unknown("nope".into())))
    );
}

#[test]
fn registry_rejects_unknown_adapters_keys_and_fields() {
    let rejected = [
        r#"{"models":{"x":{"adapter":"telepathy","model":"m"}}}"#,
        r#"{"models":{"x":{"adapter":"anthropic-http","model":"m"}}}"#,
        r#"{"models":{"x":{"adapter":"gemini-cli","model":"m"}}}"#,
        r#"{"models":{"x":{"adapter":"openai-http","model":"m"}}}"#,
        r#"{"models":{"x":{"adapter":"openai-http","base_url":"u","model":"m","key":"openrouter"}}}"#,
        r#"{"models":{"x":{"adapter":"openai-http","base_url":"u","model":"m","key":{"item":"model-proxy","env":"K"}}}}"#,
        r#"{"model":{}}"#,
    ];
    for text in rejected {
        assert!(Registry::parse(text).is_err(), "{text}");
    }
    let proxy = r#"{"models":{"x":{"adapter":"openai-http","base_url":"u","model":"m","key":"model-proxy"}}}"#;
    let entry = &Registry::parse(proxy).expect("model-proxy key").models[&ModelId::new("x")];
    assert!(matches!(
        entry.kind,
        AdapterKind::OpenaiHttp {
            key: Some(KeyItem::ModelProxy),
            ..
        }
    ));
}

fn http_entry(base_url: &str, model: &str, key: Option<&str>) -> Value {
    json!({ "adapter": "openai-http", "base_url": base_url, "model": model, "key": key })
}

#[test]
fn models_report_whether_each_endpoint_serves_them() {
    let server = FakeServer::start(vec![ok(json!({
        "object": "list",
        "data": [{ "id": "ungate-grok-4.7-high", "object": "model", "owned_by": "cursor" }]
    }))]);
    let proxy = format!("{}/v1", server.url);
    let closed = TcpListener::bind("127.0.0.1:0").expect("bind");
    let gone = format!("http://{}/v1", closed.local_addr().expect("addr"));
    drop(closed);
    let registry = json!({ "models": {
        "grok": http_entry(&proxy, "ungate-grok-4.7-high", Some("model-proxy")),
        "gemini": http_entry(&proxy, "ungate-gemini-unmapped", Some("model-proxy")),
        "local": http_entry(&gone, "qwen2.5-coder:32b", None),
        "claude": { "adapter": "claude-cli", "model": "opus" },
    }});
    let registry = Registry::parse(&registry.to_string()).expect("registry");
    let keys = Arc::new(FakeKeys::default());
    let source: Arc<dyn KeySource> = keys.clone();
    let mut adapter = RegistryAdapter::new(registry, source);
    adapter.probe_timeout = Duration::from_secs(2);

    let Catalog::Listed(models) = adapter.probe_models() else {
        panic!("a registry lists its models");
    };
    let seen = server.finish();
    let availability: BTreeMap<&str, Option<&Availability>> = models
        .iter()
        .map(|model| (model.name.as_str(), model.availability.as_ref()))
        .collect();
    assert_eq!(availability["grok"], Some(&Availability::Available));
    assert_eq!(availability["gemini"], Some(&Availability::NotServed));
    assert!(
        matches!(
            availability["local"],
            Some(Availability::Unreachable { .. })
        ),
        "{availability:?}"
    );
    assert_eq!(availability["claude"], None, "CLI models are not probed");
    assert_eq!(seen.len(), 1, "one GET per endpoint");
    assert!(
        seen[0].head.starts_with("GET /v1/models "),
        "{}",
        seen[0].head
    );
    assert!(
        seen[0]
            .head
            .contains(&format!("authorization: Bearer {KEY}"))
    );
    assert_eq!(*keys.0.lock().expect("fetches"), [KeyItem::ModelProxy]);
    assert!(
        adapter.models().admits("gemini"),
        "the registry, not the endpoint, decides what members can use"
    );
}
