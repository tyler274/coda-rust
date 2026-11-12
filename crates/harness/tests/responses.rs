//! Local HTTP wire proof only. These tests never contact a model service.
#[path = "support/recover.rs"]
mod recover;
use std::time::Duration;

use harness::{
    engine::{
        self, Cause, Item, RunLimits, Tool, ToolOutcome,
        recipe::{Finish, Provider, Request},
    },
    provider::ResponsesProvider,
};
use serde_json::{Value, json};
#[path = "support/responses.rs"]
mod wire;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

fn limits() -> RunLimits {
    RunLimits {
        max_calls: 4,
        max_response_bytes: 4096,
        ..RunLimits::default()
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
            let content_type = if status == "200 OK" {
                "text/event-stream"
            } else {
                "application/json"
            };
            let body = if status == "200 OK" {
                serde_json::from_str(&body)
                    .map(wire::stream)
                    .unwrap_or(body)
            } else {
                body
            };
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
                "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(reply.as_bytes()).await.unwrap();
            requests.push(request);
        }
        requests
    });
    (endpoint, task)
}

fn request() -> Request {
    Request {
        model: "test-model".into(),
        context_window_tokens: 100_000,
        max_input_tokens: 100_000,
        options: json!({"effort":"high"}),
        cache_key: None,
        instructions: "explicit owner instructions".into(),
        items: vec![Item::Message {
            role: "user".into(),
            text: "hello".into(),
        }],
        preloaded_skills: vec![],
        tools: vec![
            Tool {
                name: "read".into(),
                description: "Read permitted data".into(),
                parameters: json!({"type":"object"}),
            }
            .function(),
        ],
    }
}

#[tokio::test]
async fn real_http_two_turn_skill_loop_preserves_opaque_reasoning_and_exact_result() {
    use async_trait::async_trait;
    use harness::engine::{Event, EventKind, Invocation, Product, Turn};

    struct SkillProduct {
        harness: harness::skills::Package,
        events: Vec<Event>,
        outputs: Vec<String>,
    }
    #[async_trait]
    impl Product for SkillProduct {
        fn read_result(
            &mut self,
            _call_id: &str,
        ) -> harness::engine::recipe::ToolFuture<Option<String>> {
            Box::pin(async { Err("result paging is unavailable for this fixture".into()) })
        }
        async fn finish_partial(
            &mut self,
            _turn: &harness::engine::Turn,
            _backstops: &[harness::engine::LimitBackstop],
        ) -> Result<(), harness::engine::Cause> {
            Err("this fixture has no partial-result disposition".into())
        }
        async fn window_seed(
            &mut self,
        ) -> Result<harness::engine::WindowSeed, harness::engine::Cause> {
            Err("working windows are unavailable for this fixture".into())
        }

        async fn message_sources(
            &mut self,
        ) -> Result<Option<Vec<harness::engine::ProductMessage>>, harness::engine::Cause> {
            Ok(None)
        }

        async fn admit_binding(
            &mut self,
            _: &harness::engine::recipe::RequestBinding,
        ) -> Result<(), harness::engine::Cause> {
            Ok(())
        }

        async fn assert_current(&mut self) -> Result<(), Cause> {
            Ok(())
        }
        async fn persist(&mut self, event: Event) -> Result<(), Cause> {
            self.events.push(event);
            Ok(())
        }
        fn execute(
            &mut self,
            call: &Invocation,
            arguments: Value,
        ) -> harness::engine::recipe::ToolFuture {
            let outcome = (|| {
                assert_eq!(call.name, "skill_view");
                assert!(matches!(self.events.last().map(|event| &event.kind),
                Some(EventKind::ToolStarted(started)) if started == call));
                let view = self.harness.execute_skill_view(&arguments)?;
                let output = view.to_json();
                self.outputs.push(output.clone());
                Ok(ToolOutcome::Returned(output))
            })();
            Box::pin(std::future::ready(outcome))
        }
        async fn finish(&mut self, _: &Turn) -> Result<Finish, Cause> {
            Ok(Finish::Committed)
        }
    }
    let reasoning = json!({"type":"reasoning","id":"reasoning-skill-1","summary":[],
        "encrypted_content":"opaque-provider-continuation+/="});
    let arguments = " { \"name\" : \"patient-record-retrieval\" } ";
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
    let mut provider = ResponsesProvider::new(
        endpoint,
        "local-test-key".into(),
        std::sync::Arc::new(quota::Gate::default()),
    )
    .unwrap();
    let mut owner = SkillProduct {
        harness: harness::skills::Package::load(
            "chords",
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"),
        )
        .unwrap(),
        events: vec![],
        outputs: vec![],
    };
    let expected_output = owner
        .harness
        .execute_skill_view(&serde_json::json!({"name":"patient-record-retrieval"}))
        .unwrap()
        .to_json();
    let decoded_output: Value = serde_json::from_str(&expected_output).unwrap();
    assert_eq!(decoded_output["status"], "available");
    assert!(!decoded_output["content"].as_str().unwrap().is_empty());
    let bounds = limits();
    let mut request = request();
    request.cache_key = Some(harness::PromptCacheKey::scoped("test", &["scope-a"]));
    request.instructions = "Use only the authorized skill catalog.".into();
    request.tools = vec![harness::skills::tool().function()];
    request.items = vec![Item::Message {
        role: "user".into(),
        text: "Read the pre-visit preparation skill.".into(),
    }];
    let result = recover::work(request.freeze().unwrap(), "http-skill", &[], 0)
        .unwrap()
        .run(
            &mut provider,
            &mut owner,
            bounds,
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();
    let requests = tokio::time::timeout(Duration::from_secs(2), server)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0]["store"], false);
    assert_eq!(
        requests[0]["include"],
        json!(["reasoning.encrypted_content"])
    );
    assert_eq!(requests[0]["stream"], true);
    assert_eq!(requests[0]["model"], "test-model");
    assert_eq!(requests[0]["reasoning"]["effort"], "high");
    assert_eq!(requests[0]["tools"][0]["type"], "function");
    assert_eq!(
        requests[0]["input"],
        json!([{"role":"user","content":"Read the pre-visit preparation skill."}])
    );
    let key = requests[0]["prompt_cache_key"]
        .as_str()
        .expect("opaque cache partition on first request");
    assert_eq!(key.len(), 64);
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
    assert_eq!(result.text(), "The skill is available.");
    let progress = engine::recipe::Progress::from_events(&owner.events);
    assert_eq!(progress.calls, 2);
    assert_eq!(progress.known_tokens, 17);
    assert!(matches!(owner.events.last().map(|event| &event.kind),
        Some(EventKind::Candidate { text }) if text == "The skill is available."));
}

#[tokio::test]
async fn http_failure_omits_the_provider_error_body() {
    let (endpoint, server) = server(
        "do not leak provider body secret".into(),
        "429 Too Many Requests",
    )
    .await;
    let mut provider = ResponsesProvider::new(
        endpoint,
        "local-test-key".into(),
        std::sync::Arc::new(quota::Gate::default()),
    )
    .unwrap();
    let error = provider
        .exchange(
            &request(),
            &engine::Cancellation::default(),
            &limits(),
            &mut |_| Ok(()),
        )
        .await
        .unwrap_err();
    assert!(!error.to_string().contains("secret"));
    server.await.unwrap();
}

#[path = "support/quota.rs"]
mod quota;
