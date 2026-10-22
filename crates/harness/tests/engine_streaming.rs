//! Loopback SSE protocol and owner acknowledgment proof. No live provider calls.
use async_trait::async_trait;
use harness::{engine::*, responses::Responses};
use serde_json::{Value, json};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

fn limits() -> Limits {
    Limits {
        model_calls: 3,
        tool_calls: 3,
        calls_per_turn: 3,
        request_bytes: 100_000,
        response_bytes: 100_000,
        tool_result_bytes: 1024,
        total_tool_result_bytes: 4096,
        output_tokens: 100,
        elapsed: Duration::from_secs(2),
    }
}
fn message() -> Value {
    json!({"type":"message","id":"m","role":"assistant","status":"completed","content":[{"type":"output_text","text":"Hi é","annotations":[]}]})
}
fn events(tool: bool) -> Vec<Value> {
    let mut events = vec![
        json!({"type":"response.created","response":{"id":"r"}}),
        json!({"type":"response.output_item.added","output_index":0,"item":{"type":"message","id":"m","role":"assistant","content":[]}}),
        json!({"type":"response.content_part.added","output_index":0,"item_id":"m","content_index":0,"part":{"type":"output_text","text":""}}),
        json!({"type":"response.output_text.delta","output_index":0,"item_id":"m","content_index":0,"delta":"Hi "}),
        json!({"type":"response.output_text.delta","output_index":0,"item_id":"m","content_index":0,"delta":"é"}),
        json!({"type":"response.output_text.done","output_index":0,"item_id":"m","content_index":0,"text":"Hi é"}),
        json!({"type":"response.content_part.done","output_index":0,"item_id":"m","content_index":0,"part":message()["content"][0]}),
        json!({"type":"response.output_item.done","output_index":0,"item":message()}),
    ];
    let mut output = vec![message()];
    if tool {
        let call = json!({"type":"function_call","id":"f","call_id":"c","name":"save","arguments":"{}","status":"completed"});
        // Arguments arrive between text fragments, without running the tool early.
        events.insert(4, json!({"type":"response.output_item.added","output_index":1,"item":{"type":"function_call","id":"f","call_id":"c","name":"save","arguments":""}}));
        events.insert(5, json!({"type":"response.function_call_arguments.delta","output_index":1,"item_id":"f","delta":"{"}));
        events.push(json!({"type":"response.function_call_arguments.delta","output_index":1,"item_id":"f","delta":"}"}));
        events.push(json!({"type":"response.function_call_arguments.done","output_index":1,"item_id":"f","arguments":"{}"}));
        events.push(json!({"type":"response.output_item.done","output_index":1,"item":call}));
        output.push(call);
    }
    events.push(json!({"type":"response.completed","response":{"id":"r","status":"completed","output":output,"usage":{}}}));
    for (i, event) in events.iter_mut().enumerate() {
        event["sequence_number"] = json!(i);
    }
    events
}
fn bytes(events: &[Value]) -> Vec<u8> {
    let mut result = b": heartbeat\r\n\r\n".to_vec();
    for event in events {
        result.extend_from_slice(
            format!(
                "event: {}\r\ndata: {}\r\n\r\n",
                event["type"].as_str().unwrap(),
                event
            )
            .as_bytes(),
        );
    }
    result
}

async fn server(
    replies: Vec<Vec<u8>>,
    hold: bool,
) -> (String, Arc<AtomicUsize>, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/v1/responses", listener.local_addr().unwrap());
    let count = Arc::new(AtomicUsize::new(0));
    let observed = count.clone();
    let task = tokio::spawn(async move {
        for reply in replies {
            let (mut stream, _) = listener.accept().await.unwrap();
            observed.fetch_add(1, Ordering::SeqCst);
            let mut request = Vec::new();
            loop {
                let mut chunk = [0; 4096];
                let n = stream.read(&mut chunk).await.unwrap();
                if n == 0 {
                    return;
                }
                request.extend_from_slice(&chunk[..n]);
                if let Some(offset) = request.windows(4).position(|v| v == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&request[..offset]);
                    let len = headers
                        .lines()
                        .find_map(|line| {
                            let (name, value) = line.split_once(':')?;
                            name.eq_ignore_ascii_case("content-length")
                                .then(|| value.trim().parse::<usize>().unwrap())
                        })
                        .unwrap();
                    if request.len() >= offset + 4 + len {
                        let body: Value =
                            serde_json::from_slice(&request[offset + 4..offset + 4 + len]).unwrap();
                        assert_eq!(body["stream"], true);
                        break;
                    }
                }
            }
            stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\n").await.unwrap();
            // Every byte is a transport write; the decoder is also unit-tested
            // bytewise because TCP may coalesce these writes.
            for byte in reply {
                if stream.write_all(&[byte]).await.is_err() {
                    return;
                }
            }
            if hold {
                std::future::pending::<()>().await;
            }
        }
    });
    (endpoint, count, task)
}
#[derive(Default)]
struct Owner {
    events: Vec<Event>,
    visible: String,
    tools: usize,
    reject: bool,
    cancel: Option<Cancellation>,
    pending: bool,
}
#[async_trait]
impl Host for Owner {
    async fn prepare(&mut self, transcript: &[Item], _: &Usage) -> Result<Context, String> {
        Ok(Context {
            instructions: "owner".into(),
            items: transcript.to_vec(),
            tools: vec![Tool {
                name: "save".into(),
                description: "save".into(),
                parameters: json!({"type":"object"}),
            }],
        })
    }
    async fn assert_current(&mut self) -> Result<(), String> {
        Ok(())
    }
    async fn persist(&mut self, event: Event) -> Result<(), String> {
        if let EventKind::TextDelta { delta, .. } = &event.kind {
            if self.reject {
                return Err("publication refused".into());
            }
            if self.pending {
                return std::future::pending().await;
            }
            self.visible.push_str(&delta.text);
            if let Some(cancel) = &self.cancel {
                cancel.cancel();
            }
        }
        self.events.push(event);
        Ok(())
    }
    async fn admit(&mut self, _: &Invocation, _: &Value) -> Result<Admission, String> {
        Ok(Admission::Allow)
    }
    async fn execute(&mut self, _: &Invocation, _: Value) -> Result<String, String> {
        self.tools += 1;
        Ok("saved".into())
    }
}
async fn attempt(
    replies: Vec<Vec<u8>>,
    owner: &mut Owner,
    cap: usize,
    hold: bool,
    cancel: &Cancellation,
) -> (Result<Candidate, ExecutionError>, usize) {
    let (endpoint, count, task) = server(replies, hold).await;
    let mut provider =
        Responses::with_endpoint("local-test".into(), "test-model".into(), &endpoint)
            .unwrap()
            .streaming();
    let mut bounds = limits();
    bounds.response_bytes = cap;
    bounds.elapsed = Duration::from_millis(250);
    let result = run(&mut provider, owner, "a", vec![], &bounds, cancel).await;
    let count = count.load(Ordering::SeqCst);
    task.abort();
    (result, count)
}

#[tokio::test]
async fn streamed_prose_survives_tool_interleaving_and_finalization() {
    let mut owner = Owner::default();
    let (result, count) = attempt(
        vec![bytes(&events(true)), bytes(&events(false))],
        &mut owner,
        100_000,
        false,
        &Cancellation::default(),
    )
    .await;
    let result = result.unwrap();
    assert_eq!(count, 2);
    assert_eq!(owner.tools, 1);
    assert_eq!(result.text, "Hi éHi é");
    assert_eq!(result.text, owner.visible);
    let start = owner
        .events
        .iter()
        .position(|e| matches!(e.kind, EventKind::ToolStarted(_)))
        .unwrap();
    let returned = owner
        .events
        .iter()
        .position(|e| matches!(e.kind, EventKind::ProviderReturned(_)))
        .unwrap();
    assert!(returned < start);
}
#[tokio::test]
async fn disconnect_after_visible_text_is_not_retried_or_completed() {
    let mut owner = Owner::default();
    let prefix = events(true)[..6].to_vec();
    let (result, count) = attempt(
        vec![bytes(&prefix)],
        &mut owner,
        100_000,
        false,
        &Cancellation::default(),
    )
    .await;
    assert!(result.is_err());
    assert_eq!(count, 1);
    assert_eq!(owner.visible, "Hi ");
    assert_eq!(owner.tools, 0);
    assert!(
        !owner
            .events
            .iter()
            .any(|e| matches!(e.kind, EventKind::Candidate { .. }))
    );
}
#[tokio::test]
async fn failed_ack_and_cancellation_stop_stream_without_effects() {
    for mode in 0..3 {
        let cancel = Cancellation::default();
        let mut owner = Owner {
            reject: mode == 0,
            cancel: (mode == 1).then(|| cancel.clone()),
            pending: mode == 2,
            ..Owner::default()
        };
        let (result, count) = attempt(
            vec![bytes(&events(true))],
            &mut owner,
            100_000,
            true,
            &cancel,
        )
        .await;
        let error = result.unwrap_err();
        assert_eq!(count, 1);
        assert_eq!(owner.tools, 0);
        assert!(matches!(
            error.failure,
            Failure::Host(_) | Failure::Cancelled | Failure::Deadline
        ));
        if mode != 1 {
            assert!(owner.visible.is_empty());
        } else {
            assert_eq!(
                owner.visible, "Hi ",
                "cancellation fences the next ready delta"
            );
        }
    }
}
#[tokio::test]
async fn terminal_and_identity_corruption_fail_closed() {
    let mut cases = Vec::new();
    let mut duplicate = events(false);
    duplicate[4]["sequence_number"] = duplicate[3]["sequence_number"].clone();
    cases.push(bytes(&duplicate));
    let mut wrong = events(false);
    wrong[4]["item_id"] = json!("other");
    cases.push(bytes(&wrong));
    let mut incomplete = events(false);
    incomplete.last_mut().unwrap()["type"] = json!("response.incomplete");
    cases.push(bytes(&incomplete));
    let mut failed = events(false);
    failed.last_mut().unwrap()["type"] = json!("response.failed");
    cases.push(bytes(&failed));
    let mut error = events(false);
    error.last_mut().unwrap()["type"] = json!("error");
    cases.push(bytes(&error));
    let mut missing = events(false);
    missing.pop();
    cases.push(bytes(&missing));
    let mut mismatched = events(false);
    mismatched.last_mut().unwrap()["response"]["output"][0]["content"][0]["text"] =
        json!("changed");
    cases.push(bytes(&mismatched));
    let mut call = events(true);
    let done = call
        .iter_mut()
        .find(|e| e["type"] == "response.output_item.done" && e["output_index"] == 1)
        .unwrap();
    done["item"]["call_id"] = json!("changed");
    cases.push(bytes(&call));
    let mut trailing = bytes(&events(false));
    trailing.extend_from_slice(b"data: {\"type\":");
    cases.push(trailing);
    for reply in cases {
        let mut owner = Owner::default();
        let (result, count) = attempt(
            vec![reply],
            &mut owner,
            100_000,
            false,
            &Cancellation::default(),
        )
        .await;
        assert!(result.is_err());
        assert_eq!(count, 1);
        assert_eq!(owner.tools, 0);
    }
}
#[tokio::test]
async fn oversized_stream_fails_without_retry() {
    let mut owner = Owner::default();
    let (result, count) = attempt(
        vec![bytes(&events(false))],
        &mut owner,
        64,
        false,
        &Cancellation::default(),
    )
    .await;
    assert!(result.is_err());
    assert_eq!(count, 1);
}
