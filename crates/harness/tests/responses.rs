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
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/v1/responses", listener.local_addr().unwrap());
    let status = status.to_owned();
    let task = tokio::spawn(async move {
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
        request
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
