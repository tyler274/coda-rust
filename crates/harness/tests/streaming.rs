use harness::execution::*;
use harness::provider::ResponsesProvider;
use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn request() -> Request {
    Request {
        model: "controlled".into(),
        options: json!({}),
        instructions: "original".into(),
        tools: vec![],
        items: vec![],
    }
}
async fn server(events: Vec<String>, wait: Option<tokio::sync::oneshot::Receiver<()>>) -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/responses", listener.local_addr().unwrap());
    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut request = vec![0; 8192];
        let n = socket.read(&mut request).await.unwrap();
        assert!(String::from_utf8_lossy(&request[..n]).contains("POST /responses"));
        socket
            .write_all(
                b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\n",
            )
            .await
            .unwrap();
        let mut wait = wait;
        for (index, event) in events.into_iter().enumerate() {
            // Deliberately split every frame, including multibyte text.
            for byte in event.as_bytes() {
                socket.write_all(&[*byte]).await.unwrap();
            }
            socket.flush().await.unwrap();
            if index == 0
                && let Some(wait) = wait.take()
            {
                wait.await.unwrap();
            }
        }
    });
    endpoint
}
fn event(value: Value) -> String {
    format!("data: {value}\r\n\r\n")
}
#[tokio::test]
async fn native_delta_arrives_before_provider_terminal_over_http() {
    let (sent, wait) = tokio::sync::oneshot::channel();
    let endpoint = server(vec![event(json!({"type":"response.output_text.delta","delta":"café"})),event(json!({"type":"response.completed","response":{"id":"r1","status":"completed","output":[{"type":"message","content":[{"type":"output_text","text":"café"}]}],"usage":{"input_tokens":4,"output_tokens":2}}}))],Some(wait)).await;
    let mut provider = ResponsesProvider::new(endpoint, "synthetic".into()).unwrap();
    let mut sent = Some(sent);
    let mut pieces = vec![];
    let response = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        provider.exchange(&request(), &Cancellation::default(), &mut |event| {
            if let StreamEvent::Text(text) = event {
                pieces.push(text);
                sent.take().unwrap().send(()).unwrap();
            }
            Ok(())
        }),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(pieces, ["café"]);
    assert_eq!(response.usage.output_tokens, Some(2));
}
#[derive(Default)]
struct Product(Vec<Record>);
impl Owner for Product {
    async fn check(&mut self) -> Result<(), String> {
        Ok(())
    }
    async fn record(&mut self, record: Record) -> Result<(), String> {
        self.0.push(record);
        Ok(())
    }
    async fn provisional(&mut self, _: u64, _: &str) -> Result<(), String> {
        Err("product presentation fence closed".into())
    }
    async fn dispatch(&mut self, _: &Call, _: &Cancellation) -> Result<Value, String> {
        panic!("no effects")
    }
    async fn complete(&mut self, _: &Response) -> Result<(), String> {
        panic!("incomplete must never publish")
    }
}
#[tokio::test]
async fn failed_terminal_retains_reported_usage_and_provider_identity() {
    let endpoint=server(vec![event(json!({"type":"response.created","response":{"id":"r-interrupted"}})),event(json!({"type":"response.incomplete","response":{"id":"r-interrupted","status":"incomplete","usage":{"input_tokens":13,"output_tokens":7}}}))],None).await;
    let mut provider = ResponsesProvider::new(endpoint, "synthetic".into()).unwrap();
    let mut owner = Product::default();
    let mut progress = Progress::default();
    assert!(
        run(
            &mut provider,
            &mut owner,
            request(),
            &mut progress,
            Limits::default(),
            &Cancellation::default(),
            &mut |_| Ok(())
        )
        .await
        .is_err()
    );
    assert_eq!(progress.known_tokens, 20);
    assert_eq!(Progress::from_records(&owner.0), progress);
    assert!(owner.0.iter().any(|r|matches!(r,Record::Observation{usage,provenance,..} if usage.input_tokens==Some(13)&&provenance["response_id"]=="r-interrupted")));
}
#[tokio::test]
async fn presentation_rejection_cannot_discard_late_usage_or_publish() {
    struct Late;
    impl Provider for Late {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &Cancellation,
            emit: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
        ) -> Result<Response, String> {
            emit(StreamEvent::Text("not publishable".into()))?;
            Ok(Response {
                text: "late completion".into(),
                calls: vec![],
                usage: Usage {
                    input_tokens: Some(11),
                    output_tokens: Some(5),
                },
                provenance: json!({"mode":"local"}),
            })
        }
    }
    let mut owner = Product::default();
    let mut progress = Progress::default();
    assert!(
        run(
            &mut Late,
            &mut owner,
            request(),
            &mut progress,
            Limits::default(),
            &Cancellation::default(),
            &mut |_| Ok(())
        )
        .await
        .is_err()
    );
    assert_eq!(progress.known_tokens, 16);
    assert_eq!(Progress::from_records(&owner.0), progress);
    assert!(owner.0.iter().any(|r| matches!(r, Record::Exchange { .. })));
}
#[tokio::test]
async fn abrupt_disconnect_records_unknown_usage_not_zero() {
    let endpoint = server(
        vec![event(
            json!({"type":"response.created","response":{"id":"r-lost"}}),
        )],
        None,
    )
    .await;
    let mut provider = ResponsesProvider::new(endpoint, "synthetic".into()).unwrap();
    let mut owner = Product::default();
    let mut progress = Progress::default();
    assert!(
        run(
            &mut provider,
            &mut owner,
            request(),
            &mut progress,
            Limits::default(),
            &Cancellation::default(),
            &mut |_| Ok(())
        )
        .await
        .is_err()
    );
    assert_eq!(progress.unknown_usage, 1);
    assert!(owner.0.iter().any(|r|matches!(r,Record::Observation{usage,provenance,..} if usage.input_tokens.is_none()&&provenance["response_id"]=="r-lost")));
}
