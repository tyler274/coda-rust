use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use serde_json::{Value, json};

struct Client {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next_id: u64,
}

impl Client {
    fn start(dir: &Path, agent: &str, create: bool) -> Self {
        let mut command = Command::new(env!("CARGO_BIN_EXE_coda-swarm"));
        command
            .arg("mcp")
            .env("CODA_SWARM_DIR", dir)
            .env("CODA_SWARM_AGENT", agent)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped());
        if create {
            command.arg("--create");
        }
        let mut child = command.spawn().expect("spawn coda-swarm");
        let stdin = child.stdin.take().expect("stdin");
        let stdout = BufReader::new(child.stdout.take().expect("stdout"));
        let mut client = Self {
            child,
            stdin,
            stdout,
            next_id: 0,
        };
        let init = client.request(
            "initialize",
            json!({ "protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": { "name": "test", "version": "0" } }),
        );
        assert_eq!(init["result"]["serverInfo"]["name"], "coda-swarm");
        client.notify("notifications/initialized");
        client
    }

    fn send(&mut self, method: &str, params: Value) -> u64 {
        self.next_id += 1;
        let line =
            json!({ "jsonrpc": "2.0", "id": self.next_id, "method": method, "params": params });
        writeln!(self.stdin, "{line}").expect("write request");
        self.next_id
    }

    fn notify(&mut self, method: &str) {
        writeln!(
            self.stdin,
            "{}",
            json!({ "jsonrpc": "2.0", "method": method })
        )
        .expect("write");
    }

    fn receive(&mut self, id: u64) -> Value {
        let mut line = String::new();
        self.stdout.read_line(&mut line).expect("read reply");
        let reply: Value = serde_json::from_str(&line).expect("reply is JSON");
        assert_eq!(reply["id"], id, "replies arrive in request order");
        reply
    }

    fn request(&mut self, method: &str, params: Value) -> Value {
        let id = self.send(method, params);
        self.receive(id)
    }

    fn call(&mut self, tool: &str, arguments: Value) -> Value {
        let reply = self.request(
            "tools/call",
            json!({ "name": tool, "arguments": arguments }),
        );
        reply["result"].clone()
    }

    fn ok(&mut self, tool: &str, arguments: Value) -> Value {
        let result = self.call(tool, arguments);
        assert_eq!(result["isError"], false, "{tool} failed: {result}");
        result["structuredContent"].clone()
    }

    fn refused(&mut self, tool: &str, arguments: Value) -> String {
        let result = self.call(tool, arguments);
        assert_eq!(result["isError"], true, "{tool} should fail: {result}");
        result["content"][0]["text"]
            .as_str()
            .expect("error text")
            .to_owned()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn every_advertised_input_schema_is_a_valid_closed_object_schema() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut lead = Client::start(dir.path(), "lead", true);
    let tools = lead.request("tools/list", json!({}))["result"]["tools"].clone();
    let names: Vec<&str> = tools
        .as_array()
        .expect("tools")
        .iter()
        .map(|tool| tool["name"].as_str().expect("name"))
        .collect();
    assert_eq!(
        names,
        [
            "swarm",
            "post_message",
            "read_board",
            "swarm_wait",
            "board_view"
        ]
    );
    for tool in tools.as_array().expect("tools") {
        let schema = &tool["inputSchema"];
        assert_eq!(
            schema["$schema"],
            "https://json-schema.org/draft/2020-12/schema"
        );
        jsonschema::meta::validate(schema)
            .unwrap_or_else(|error| panic!("{} schema: {error}", tool["name"]));
        jsonschema::draft202012::new(schema).expect("schema compiles");
        assert_eq!(schema["type"], "object");
        let properties = schema["properties"].as_object().expect("properties");
        for required in schema["required"].as_array().expect("required") {
            let key = required.as_str().expect("required names are strings");
            assert!(
                properties.contains_key(key),
                "{} requires unknown {key}",
                tool["name"]
            );
        }
    }
}

#[test]
fn members_post_read_add_and_wake_each_other_across_processes() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut lead = Client::start(dir.path(), "lead", true);

    let first = lead.ok("post_message", json!({ "text": "kickoff" }));
    assert_eq!(first["seq"], 1);
    let read = lead.ok("read_board", json!({}));
    assert_eq!(read["messages"][0]["text"], "kickoff");
    assert_eq!(read["last_read_seq"], 1);

    let added = lead.ok(
        "swarm",
        json!({ "action": "add", "name": "scout", "brief": "map the parser", "model": "m1" }),
    );
    assert_eq!(added["added"][0]["name"], "scout");
    assert_eq!(added["added"][0]["brief_seq"], 2);
    let status = lead.ok("swarm", json!({ "action": "status" }));
    let scout = status["members"]
        .as_array()
        .expect("members")
        .iter()
        .find(|m| m["name"] == "scout")
        .expect("scout on roster")
        .clone();
    assert_eq!(scout["state"], "waiting");
    assert_eq!(scout["model"], "m1");
    assert_eq!(scout["added_by"], "lead");

    let wait = lead.send(
        "tools/call",
        json!({ "name": "swarm_wait", "arguments": { "since": 2, "timeout": 30 } }),
    );

    let mut scout = Client::start(dir.path(), "scout", false);
    let brief = scout.ok("read_board", json!({}));
    assert_eq!(brief["messages"][1]["text"], "@scout map the parser");
    scout.ok(
        "post_message",
        json!({ "text": "chatter nobody is named in" }),
    );
    scout.ok("post_message", json!({ "text": "@lead parser mapped" }));

    let woke = lead.receive(wait)["result"].clone();
    assert_eq!(woke["isError"], false, "{woke}");
    let woke = &woke["structuredContent"];
    assert_eq!(woke["woken"], true);
    let texts: Vec<&str> = woke["messages"]
        .as_array()
        .expect("messages")
        .iter()
        .map(|m| m["text"].as_str().expect("text"))
        .collect();
    assert_eq!(texts, ["chatter nobody is named in", "@lead parser mapped"]);
    assert_eq!(woke["last_read_seq"], 4);

    let view = lead.ok("board_view", json!({}));
    assert_eq!(view["head"], 4);
    assert_eq!(view["channels"][0]["name"], "main");
}

#[test]
fn board_view_advertises_a_self_contained_mcp_app() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut lead = Client::start(dir.path(), "lead", true);

    let tools = lead.request("tools/list", json!({}))["result"]["tools"].clone();
    let with_ui: Vec<(&str, &str)> = tools
        .as_array()
        .expect("tools")
        .iter()
        .filter_map(|tool| {
            let uri = tool["_meta"]["ui"]["resourceUri"].as_str()?;
            Some((tool["name"].as_str().expect("name"), uri))
        })
        .collect();
    assert_eq!(with_ui, [("board_view", "ui://coda-swarm/board")]);

    let listed = lead.request("resources/list", json!({}))["result"]["resources"].clone();
    assert_eq!(listed.as_array().expect("resources").len(), 1);
    assert_eq!(listed[0]["uri"], "ui://coda-swarm/board");
    assert_eq!(listed[0]["mimeType"], "text/html;profile=mcp-app");

    let read = lead.request("resources/read", json!({ "uri": "ui://coda-swarm/board" }));
    let content = &read["result"]["contents"][0];
    assert_eq!(content["uri"], "ui://coda-swarm/board");
    assert_eq!(content["mimeType"], "text/html;profile=mcp-app");
    assert!(
        content["_meta"]["ui"].get("csp").is_none(),
        "declares no network domains"
    );
    let html = content["text"].as_str().expect("html text");
    assert!(html.starts_with("<!DOCTYPE html>"), "an HTML5 document");
    assert!(
        html.contains("\"board_view\""),
        "refreshes through board_view"
    );
    assert!(html.contains("ui/initialize"), "runs the App handshake");
    let lowered = html.to_ascii_lowercase();
    for external in [
        "http://",
        "https://",
        "src=",
        "href=",
        "<link",
        "@import",
        "url(",
        "fetch(",
        "xmlhttprequest",
        "websocket",
        "eventsource",
        "import(",
        "sendbeacon",
    ] {
        assert!(!lowered.contains(external), "view loads {external}");
    }

    let missing = lead.request("resources/read", json!({ "uri": "ui://coda-swarm/nope" }));
    assert_eq!(missing["error"]["code"], -32002);

    lead.ok(
        "swarm",
        json!({ "action": "title", "title": "Parser swarm" }),
    );
    lead.ok(
        "swarm",
        json!({ "action": "add", "name": "scout", "brief": "map the parser", "model": "m1" }),
    );
    let view = lead.call("board_view", json!({}));
    assert_eq!(view["isError"], false, "{view}");
    assert_eq!(view["structuredContent"]["members"][1]["name"], "scout");
    let summary = view["content"][0]["text"].as_str().expect("summary");
    assert!(
        summary.starts_with("Parser swarm ("),
        "a plain summary, not JSON: {summary}"
    );
    assert!(
        summary.contains("- scout (worker): waiting, m1"),
        "{summary}"
    );
    assert!(
        summary.contains("[1] #main lead: @scout map the parser"),
        "{summary}"
    );
}

#[test]
fn workers_manage_only_the_helpers_they_added() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut lead = Client::start(dir.path(), "lead", true);
    lead.ok(
        "swarm",
        json!({ "action": "add", "name": "scout", "brief": "go" }),
    );
    lead.ok(
        "swarm",
        json!({ "action": "add", "name": "peer", "brief": "go" }),
    );

    let mut scout = Client::start(dir.path(), "scout", false);
    scout.ok(
        "swarm",
        json!({ "action": "add", "name": "helper", "brief": "assist", "n": 2 }),
    );
    let refused = scout.refused("swarm", json!({ "action": "kill", "name": "peer" }));
    assert!(refused.contains("scout may not kill peer"), "{refused}");
    let refused = scout.refused("swarm", json!({ "action": "kill", "name": "lead" }));
    assert!(refused.contains("cannot be killed"), "{refused}");
    let refused = scout.refused("swarm", json!({ "action": "title", "title": "x" }));
    assert!(refused.contains("only the coordinator"), "{refused}");
    let refused = scout.refused("swarm", json!({ "action": "join", "swarm": "x" }));
    assert!(refused.contains("not support"), "{refused}");

    let killed = scout.ok("swarm", json!({ "action": "kill", "name": "helper" }));
    assert_eq!(killed["killed"], json!(["helper-1", "helper-2"]));
    let refused = scout.refused(
        "swarm",
        json!({ "action": "add", "name": "helper-1", "brief": "again" }),
    );
    assert!(refused.contains("already exists"), "{refused}");

    let killed = lead.ok("swarm", json!({ "action": "kill", "name": "scout" }));
    assert_eq!(killed["killed"], json!(["scout"]));
    let refused = scout.refused("post_message", json!({ "text": "still here?" }));
    assert!(refused.contains("killed"), "{refused}");
}

#[test]
fn channel_changes_are_board_rows_every_process_replays() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut lead = Client::start(dir.path(), "lead", true);
    let created = lead.ok(
        "swarm",
        json!({ "action": "channel", "op": "create", "channel": "ops", "topic": "deploys" }),
    );
    assert_eq!(created["seq"], 1);
    lead.ok(
        "swarm",
        json!({ "action": "add", "name": "scout", "brief": "deploy", "model": "m1", "channels": ["ops"] }),
    );

    let mut scout = Client::start(dir.path(), "scout", false);
    let posted = scout.ok("post_message", json!({ "text": "deploying" }));
    assert_eq!(
        posted["channel"], "ops",
        "home channel comes from the replayed subscription"
    );
    let rows: Vec<Value> = std::fs::read_to_string(dir.path().join("board.jsonl"))
        .expect("board")
        .lines()
        .map(|line| serde_json::from_str(line).expect("row"))
        .collect();
    assert_eq!(rows[0]["control"]["op"], "create");
    assert_eq!(rows[1]["control"]["op"], "subscribe");
    assert!(rows[0].get("kind").is_none());

    lead.ok(
        "swarm",
        json!({ "action": "channel", "op": "archive", "channel": "ops" }),
    );
    let refused = scout.refused("post_message", json!({ "text": "late", "channel": "ops" }));
    assert!(refused.contains("archived"), "{refused}");
    let fallback = scout.ok("post_message", json!({ "text": "back on main" }));
    assert_eq!(fallback["channel"], "main");
    let refused = lead.refused(
        "swarm",
        json!({ "action": "channel", "op": "create", "channel": "Ops" }),
    );
    assert!(refused.contains("invalid arguments"), "{refused}");
}
