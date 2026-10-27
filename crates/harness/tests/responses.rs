//! Local HTTP wire proof only. These tests never contact a model service.
use std::time::Duration;

use harness::{
    engine::{Context, Item, Limits, Provider, Request, Tool},
    responses::Responses,
};
use serde_json::{Value, json};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

fn limits() -> Limits {
    Limits {
        model_calls: 2,
        tool_calls: 2,
        calls_per_turn: 2,
        request_bytes: 4096,
        response_bytes: 4096,
        tool_result_bytes: 1024,
        total_tool_result_bytes: 2048,
        output_tokens: 100,
        elapsed: Duration::from_secs(2),
    }
}

async fn server(body: String, status: &str) -> (String, tokio::task::JoinHandle<Value>) {
    let (endpoint, task) = sequence_server(vec![body], status).await;
    (
        endpoint,
        tokio::spawn(async move { task.await.unwrap().remove(0) }),
    )
}

async fn sequence_server(
    bodies: Vec<String>,
    status: &str,
) -> (String, tokio::task::JoinHandle<Vec<Value>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/v1/responses", listener.local_addr().unwrap());
    let status = status.to_owned();
    let task = tokio::spawn(async move {
        let mut requests = Vec::new();
        for body in bodies {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut bytes = Vec::new();
            let (offset, len) = loop {
                let mut chunk = [0; 1024];
                let count = stream.read(&mut chunk).await.unwrap();
                assert!(count > 0);
                bytes.extend_from_slice(&chunk[..count]);
                if let Some(end) = bytes.windows(4).position(|part| part == b"\r\n\r\n") {
                    let header = String::from_utf8_lossy(&bytes[..end]);
                    let len = header
                        .lines()
                        .find_map(|line| {
                            let (name, value) = line.split_once(':')?;
                            name.eq_ignore_ascii_case("content-length")
                                .then(|| value.trim().parse::<usize>().unwrap())
                        })
                        .unwrap();
                    assert!(
                        header
                            .to_ascii_lowercase()
                            .contains("authorization: bearer local-test-key")
                    );
                    break (end + 4, len);
                }
            };
            while bytes.len() < offset + len {
                let mut chunk = [0; 1024];
                let count = stream.read(&mut chunk).await.unwrap();
                assert!(count > 0);
                bytes.extend_from_slice(&chunk[..count]);
            }
            let request = serde_json::from_slice(&bytes[offset..offset + len]).unwrap();
            let reply = format!(
                "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(reply.as_bytes()).await.unwrap();
            requests.push(request);
        }
        requests
    });
    (endpoint, task)
}

fn context() -> Context {
    Context {
        instructions: "explicit owner instructions".into(),
        items: vec![Item::Message {
            role: "user".into(),
            text: "hello".into(),
        }],
        tools: vec![Tool {
            name: "read".into(),
            description: "Read permitted data".into(),
            parameters: json!({"type":"object"}),
        }],
    }
}

#[tokio::test]
async fn real_http_adapter_frames_request_and_decodes_calls() {
    let response = json!({"id":"response-1","status":"completed","usage":{"input_tokens":1},"output":[
        {"type":"reasoning","id":"reasoning-1","summary":[]},
        {"type":"function_call","status":"completed","call_id":"call-1","name":"read","arguments":"{}"}
    ]}).to_string();
    let (endpoint, server) = server(response, "200 OK").await;
    let mut provider =
        Responses::with_endpoint("local-test-key".into(), "test-model".into(), &endpoint)
            .unwrap()
            .reasoning_effort("high".into());
    let context = context();
    let limits = limits();
    let request = || Request {
        attempt_id: "a",
        ordinal: 1,
        context: &context,
        limits: &limits,
    };
    let size = provider.request_bytes(&request()).unwrap();
    let result = provider.complete(request()).await.unwrap();
    let body = server.await.unwrap();
    assert_eq!(size, serde_json::to_vec(&body).unwrap().len());
    assert_eq!(body["store"], false);
    assert_eq!(body["stream"], false);
    assert_eq!(body["model"], "test-model");
    assert_eq!(body["reasoning"]["effort"], "high");
    assert_eq!(body["max_output_tokens"], 100);
    assert_eq!(body["tools"][0]["type"], "function");
    assert!(matches!(&result.items[1], Item::Call(call) if call.id == "call-1"));
    assert_eq!(result.receipt["id"], "response-1");
}

#[tokio::test]
async fn real_http_two_turn_skill_loop_preserves_opaque_reasoning_and_exact_result() {
    use async_trait::async_trait;
    use harness::engine::{self, Admission, Event, EventKind, Host, Invocation, Usage};

    struct SkillHost {
        harness: harness::Harness,
        events: Vec<Event>,
        outputs: Vec<String>,
    }
    #[async_trait]
    impl Host for SkillHost {
        async fn prepare(&mut self, transcript: &[Item], _: &Usage) -> Result<Context, String> {
            Ok(Context {
                instructions: "Use only the authorized skill catalog.".into(),
                items: transcript.to_vec(),
                tools: vec![Tool {
                    name: "skill_view".into(),
                    description: "Read one authorized skill".into(),
                    parameters: json!({"type":"object","properties":{
                        "name":{"type":"string"},"max_result_bytes":{"type":"integer"}
                    },"required":["name","max_result_bytes"],"additionalProperties":false}),
                }],
            })
        }
        async fn assert_current(&mut self) -> Result<(), String> {
            Ok(())
        }
        async fn persist(&mut self, event: Event) -> Result<(), String> {
            self.events.push(event);
            Ok(())
        }
        async fn admit(&mut self, call: &Invocation, _: &Value) -> Result<Admission, String> {
            assert_eq!(call.name, "skill_view");
            Ok(Admission::Allow)
        }
        async fn execute(&mut self, call: &Invocation, arguments: Value) -> Result<String, String> {
            assert!(matches!(self.events.last().map(|event| &event.kind),
                Some(EventKind::ToolStarted(started)) if started == call));
            let view = self.harness.skill_view(
                arguments["name"].as_str().unwrap(),
                None,
                arguments["max_result_bytes"].as_u64().unwrap() as u32,
            )?;
            let output = view.to_json();
            self.outputs.push(output.clone());
            Ok(output)
        }
    }
    let reasoning = json!({"type":"reasoning","id":"reasoning-skill-1","summary":[],
        "encrypted_content":"opaque-provider-continuation+/="});
    let arguments = " { \"name\" : \"pre-visit-preparation\", \"max_result_bytes\" : 65536 } ";
    let first = json!({"id":"response-skill-1","status":"completed","output":[
        reasoning.clone(),
        {"type":"function_call","status":"completed","call_id":"skill-1",
         "name":"skill_view","arguments":arguments}
    ],"usage":{"input_tokens":4,"output_tokens":2}});
    let second = json!({"id":"response-skill-2","status":"completed","output":[
        {"type":"message","role":"assistant","status":"completed",
         "content":[{"type":"output_text","text":"The skill is available."}]}
    ],"usage":{"input_tokens":8,"output_tokens":3}});
    let (endpoint, server) =
        sequence_server(vec![first.to_string(), second.to_string()], "200 OK").await;
    let mut provider =
        Responses::with_endpoint("local-test-key".into(), "test-model".into(), &endpoint).unwrap();
    let mut owner = SkillHost {
        harness: harness::Harness::load(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"),
        )
        .unwrap(),
        events: vec![],
        outputs: vec![],
    };
    let expected_output = owner
        .harness
        .skill_view("pre-visit-preparation", None, 65536)
        .unwrap()
        .to_json();
    let decoded_output: Value = serde_json::from_str(&expected_output).unwrap();
    assert_eq!(decoded_output["status"], "available");
    assert!(!decoded_output["content"].as_str().unwrap().is_empty());
    let mut bounds = limits();
    bounds.request_bytes = 100_000;
    bounds.tool_result_bytes = 65_536;
    bounds.total_tool_result_bytes = 65_536;
    let result = engine::run(
        &mut provider,
        &mut owner,
        "http-skill",
        vec![Item::Message {
            role: "user".into(),
            text: "Read the pre-visit preparation skill.".into(),
        }],
        &bounds,
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    let requests = tokio::time::timeout(Duration::from_secs(2), server)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(requests.len(), 2);
    assert_eq!(
        requests[0]["input"],
        json!([{"role":"user","content":"Read the pre-visit preparation skill."}])
    );
    let mut expected_next = requests[0].clone();
    expected_next["input"].as_array_mut().unwrap().extend([
        reasoning.clone(),
        json!({"type":"function_call","call_id":"skill-1","name":"skill_view","arguments":arguments}),
        json!({"type":"function_call_output","call_id":"skill-1","output":expected_output}),
    ]);
    assert_eq!(
        requests[1], expected_next,
        "the complete next wire request must preserve context and exact result bytes"
    );
    assert_eq!(owner.outputs, vec![expected_output.clone()]);
    assert_eq!(result.text, "The skill is available.");
    assert_eq!(result.usage.model_calls, 2);
    assert_eq!(result.usage.tool_calls, 1);
    assert!(matches!(&result.transcript[1], Item::Opaque(value) if value == &reasoning));
    assert!(matches!(&owner.events[2].kind, EventKind::ToolStarted(call) if call.id == "skill-1"));
    assert!(
        matches!(&owner.events[3].kind, EventKind::ToolReturned { call_id, output, executed: true }
        if call_id == "skill-1" && output == &expected_output)
    );
    assert!(matches!(
        &owner.events[4].kind,
        EventKind::ProviderStarted { ordinal: 2, .. }
    ));
    assert!(matches!(owner.events.last().map(|event| &event.kind),
        Some(EventKind::Candidate { text }) if text == "The skill is available."));
}

#[tokio::test]
async fn http_failure_and_response_bounds_fail_without_retries() {
    for (body, status, cap) in [
        (
            "do not leak provider body secret",
            "429 Too Many Requests",
            4096,
        ),
        ("xxxxxxxxxxxxxxxx", "200 OK", 8),
    ] {
        let (endpoint, server) = server(body.into(), status).await;
        let mut provider =
            Responses::with_endpoint("local-test-key".into(), "model".into(), &endpoint).unwrap();
        let context = context();
        let mut limits = limits();
        limits.response_bytes = cap;
        let error = provider
            .complete(Request {
                attempt_id: "a",
                ordinal: 1,
                context: &context,
                limits: &limits,
            })
            .await
            .unwrap_err();
        assert!(!error.contains("secret"));
        server.await.unwrap();
    }
}

#[tokio::test]
async fn opaque_items_cannot_hide_tool_execution_or_system_instructions() {
    let provider = Responses::new("local-test-key".into(), "model".into()).unwrap();
    let mut context = context();
    let limits = limits();
    context.items.push(Item::Opaque(
        json!({"type":"function_call","call_id":"hidden"}),
    ));
    assert!(
        provider
            .request_bytes(&Request {
                attempt_id: "a",
                ordinal: 1,
                context: &context,
                limits: &limits
            })
            .is_err()
    );
}
