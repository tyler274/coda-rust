#[path = "support/recover.rs"]
mod recover;
use async_trait::async_trait;
use harness::engine::{self, Event, EventKind, Invocation, RunLimits, recipe::*};
use harness::provider::ResponsesProvider;
use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn request() -> Request {
    Request {
        cache_key: None,
        model: "controlled".into(),
        context_window_tokens: 100_000,
        max_input_tokens: 100_000,
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
        // A content-negotiating endpoint can return JSON for a wildcard Accept.
        if !String::from_utf8_lossy(&request[..n])
            .lines()
            .any(|h| h.eq_ignore_ascii_case("accept: text/event-stream"))
        {
            socket.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n{}").await.unwrap();
            return;
        }
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
    let message = json!({"id":"m","type":"message","role":"assistant","status":"completed",
        "content":[{"type":"output_text","text":"café"}]});
    let before_terminal = [
        json!({"type":"response.created","response":{"id":"r1"}}),
        json!({"type":"response.output_item.added","output_index":0,"item":{"id":"m","type":"message","role":"assistant"}}),
        json!({"type":"response.content_part.added","output_index":0,"content_index":0,"item_id":"m","part":{"type":"output_text"}}),
        json!({"type":"response.output_text.delta","output_index":0,"content_index":0,"item_id":"m","delta":"café"}),
    ];
    let after_delta = [
        json!({"type":"response.output_text.done","output_index":0,"content_index":0,"item_id":"m","text":"café"}),
        json!({"type":"response.content_part.done","output_index":0,"content_index":0,"item_id":"m","part":message["content"][0]}),
        json!({"type":"response.output_item.done","output_index":0,"item":message}),
        json!({"type":"response.completed","response":{"id":"r1","status":"completed","output":[message],"usage":{"input_tokens":4,"output_tokens":2}}}),
    ];
    let chunk = |start, events: &[Value]| {
        events
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, mut value)| {
                value["sequence_number"] = json!(start + index);
                event(value)
            })
            .collect::<String>()
    };
    let endpoint = server(
        vec![
            chunk(0, &before_terminal),
            chunk(before_terminal.len(), &after_delta) + "data: [DONE]\r\n\r\n",
        ],
        Some(wait),
    )
    .await;
    let mut provider = ResponsesProvider::new(
        endpoint,
        "synthetic".into(),
        std::sync::Arc::new(quota::Gate::default()),
    )
    .unwrap();
    let mut sent = Some(sent);
    let mut pieces = vec![];
    let response = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        provider.exchange(
            &request(),
            &Cancellation::default(),
            &RunLimits::default(),
            &mut |event| {
                if let StreamEvent::Text(delta) = event {
                    pieces.push(delta.text);
                    sent.take().unwrap().send(()).unwrap();
                }
                Ok(())
            },
        ),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(pieces, ["café"]);
    assert_eq!(response.usage.output_tokens, Some(2));
}
#[derive(Default)]
struct TestProduct(Vec<Event>);
#[async_trait]
impl engine::Product for TestProduct {
    fn read_result(
        &mut self,
        _call_id: &str,
    ) -> harness::engine::recipe::ToolFuture<Option<String>> {
        Box::pin(async { Err("result paging is unavailable for this fixture".into()) })
    }
    async fn window_seed(
        &mut self,
    ) -> Result<harness::engine::WindowSeed, harness::engine::ProductError> {
        Err("working windows are unavailable for this fixture".into())
    }

    async fn message_sources(
        &mut self,
    ) -> Result<Option<Vec<harness::engine::ProductMessage>>, harness::engine::ProductError> {
        Ok(None)
    }

    async fn admit_binding(
        &mut self,
        _: &harness::engine::recipe::RequestBinding,
    ) -> Result<(), harness::engine::ProductError> {
        Ok(())
    }

    async fn assert_current(&mut self) -> Result<(), engine::ProductError> {
        Ok(())
    }
    async fn persist(&mut self, record: Event) -> Result<(), engine::ProductError> {
        self.0.push(record);
        Ok(())
    }
    async fn live_text(
        &mut self,
        _: u64,
        _: engine::TextDelta,
    ) -> Result<TextDelivery, engine::ProductError> {
        Err("product presentation fence closed".into())
    }
    fn execute(&mut self, _: &Invocation, _: Value) -> harness::engine::recipe::ToolFuture {
        panic!("no effects")
    }
    async fn finish_partial(
        &mut self,
        turn: &Turn,
        _: &[engine::LimitBackstop],
    ) -> Result<(), engine::ProductError> {
        assert!(turn.calls().next().is_none());
        Ok(())
    }
    async fn finish(&mut self, _: &Turn) -> Result<Finish, engine::ProductError> {
        panic!("incomplete must never publish")
    }
}
#[tokio::test]
async fn failed_terminal_retains_reported_usage_and_provider_identity() {
    for reported in [true, false] {
        let mut frames = vec![event(
            json!({"type":"response.created","sequence_number":0,"response":{"id":"r-interrupted"}}),
        )];
        if reported {
            frames.push(event(json!({"type":"response.incomplete","sequence_number":1,"response":{"id":"r-interrupted","status":"incomplete","usage":{"input_tokens":13,"output_tokens":7}}})));
        }
        let endpoint = server(frames, None).await;
        let mut provider = ResponsesProvider::new(
            endpoint,
            "synthetic".into(),
            std::sync::Arc::new(quota::Gate::default()),
        )
        .unwrap();
        let mut owner = TestProduct::default();
        assert!(
            recover::work(request().freeze().unwrap(), "test", &[], 0)
                .unwrap()
                .run(
                    &mut provider,
                    &mut owner,
                    RunLimits::default(),
                    &Cancellation::default(),
                )
                .await
                .is_err()
        );
        // The server answers only the first request; later requests fail and
        // the attempt ends. The first request keeps its own usage and identity.
        let progress = Progress::from_events(owner.0.iter().filter(|event| event.ordinal == 1));
        assert_eq!(progress.known_tokens, if reported { 20 } else { 0 });
        assert_eq!(progress.unknown_usage, u64::from(!reported));
        assert!(owner.0.iter().any(|r|matches!(r,Event{kind:EventKind::ProviderObservation{usage,receipt:provenance,..},..} if usage.input_tokens==reported.then_some(13)&&provenance["id"]=="r-interrupted")));
    }
}

/// A call the provider announces is saved as begun, with its name, while its
/// request is still under way: after the request starts and before it ends,
/// here cut off before the call completed. An item announced that is not a
/// call begins nothing.
#[tokio::test]
async fn an_announced_call_is_saved_as_begun_before_its_request_ends() {
    let frames = [
        json!({"type":"response.created","response":{"id":"r-begun"}}),
        json!({"type":"response.output_item.added","output_index":0,"item":{"type":"reasoning","id":"rs","summary":[]}}),
        json!({"type":"response.output_item.done","output_index":0,"item":{"type":"reasoning","id":"rs","summary":[]}}),
        json!({"type":"response.output_item.added","output_index":1,"item":{"type":"function_call","id":"fc","call_id":"c","name":"finish_answer","arguments":"","status":"in_progress"}}),
        json!({"type":"response.function_call_arguments.delta","output_index":1,"item_id":"fc","delta":"{\"answer\":"}),
        json!({"type":"response.incomplete","response":{"id":"r-begun","status":"incomplete"}}),
    ];
    let frames = frames
        .into_iter()
        .enumerate()
        .map(|(index, mut value)| {
            value["sequence_number"] = json!(index);
            event(value)
        })
        .collect();
    let mut provider = ResponsesProvider::new(
        server(frames, None).await,
        "synthetic".into(),
        std::sync::Arc::new(quota::Gate::default()),
    )
    .unwrap();
    let mut owner = TestProduct::default();
    let ended = recover::work(request().freeze().unwrap(), "test", &[], 0)
        .unwrap()
        .run(
            &mut provider,
            &mut owner,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await;
    assert!(ended.is_err());
    // The server answers only the first request; later requests fail.
    let kinds: Vec<&EventKind> = owner
        .0
        .iter()
        .filter(|event| event.ordinal == 1)
        .map(|event| &event.kind)
        .collect();
    let began: Vec<usize> = kinds
        .iter()
        .enumerate()
        .filter(|(_, kind)| matches!(kind, EventKind::CallBegan { .. }))
        .map(|(index, _)| index)
        .collect();
    assert!(
        matches!(&began[..], [at] if matches!(kinds[*at], EventKind::CallBegan { name } if name == "finish_answer")),
        "{kinds:?}"
    );
    let started = kinds
        .iter()
        .position(|kind| matches!(kind, EventKind::ProviderStarted { .. }));
    let ended = kinds.iter().position(|kind| {
        matches!(
            kind,
            EventKind::ProviderFailed { .. } | EventKind::TransportRetry { .. }
        )
    });
    assert!(
        started.is_some_and(|started| started < began[0])
            && ended.is_some_and(|ended| began[0] < ended),
        "{kinds:?}"
    );
}

#[tokio::test]
async fn presentation_rejection_cannot_discard_late_usage_or_publish() {
    struct Late;
    impl Provider for Late {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &Cancellation,
            _: &RunLimits,
            observer: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            let _rejected = observer
                .emit(StreamEvent::Text("not publishable".into()))
                .await;
            Ok(Turn {
                items: vec![harness::engine::Item::assistant("late completion")],
                usage: Usage {
                    cached_input_tokens: None,
                    input_tokens: Some(11),
                    output_tokens: Some(5),
                },
                receipt: json!({"mode":"local"}),
            })
        }
    }
    let mut owner = TestProduct::default();
    assert!(
        recover::work(request().freeze().unwrap(), "test", &[], 0)
            .unwrap()
            .run(
                &mut Late,
                &mut owner,
                RunLimits::default(),
                &Cancellation::default(),
            )
            .await
            .is_err()
    );
    assert_eq!(Progress::from_events(&owner.0).known_tokens, 16);
    assert!(owner.0.iter().any(|r| matches!(
        r,
        Event {
            kind: EventKind::ProviderReturned(_),
            ..
        }
    )));
}
#[tokio::test]
async fn cancelled_exchange_retains_cumulative_evidence_without_double_counting() {
    struct CancelAfterEvidence;
    impl Provider for CancelAfterEvidence {
        async fn exchange(
            &mut self,
            _: &Request,
            cancel: &Cancellation,
            _: &RunLimits,
            observer: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            observer
                .emit(StreamEvent::Receipt {
                    identity: json!({"response_id":"cancelled-response"}),
                    usage: Usage::default(),
                })
                .await?;
            observer
                .emit(StreamEvent::Usage(Usage {
                    cached_input_tokens: None,
                    input_tokens: Some(11),
                    output_tokens: None,
                }))
                .await?;
            observer
                .emit(StreamEvent::Usage(Usage {
                    cached_input_tokens: None,
                    input_tokens: Some(11),
                    output_tokens: Some(5),
                }))
                .await?;
            cancel.cancel();
            // Even a successful terminal arriving after cancellation is evidence,
            // never permission to execute its tools or publish its text.
            Ok(Turn {
                items: vec![
                    harness::engine::Item::assistant("must not publish"),
                    Invocation {
                        id: "late-effect".into(),
                        name: "save".into(),
                        arguments: json!({}),
                    }
                    .into(),
                ],
                usage: Usage {
                    cached_input_tokens: None,
                    input_tokens: Some(11),
                    output_tokens: Some(5),
                },
                receipt: json!({"response_id":"cancelled-response"}),
            })
        }
    }
    let mut owner = TestProduct(vec![
        Event {
            attempt_id: "prior".into(),
            ordinal: 4,
            sequence: 1,
            kind: EventKind::ProviderStarted {
                dispatched_at_unix_ms: 1,
                request: engine::SavedRequest {
                    added: vec![],
                    items: vec![],
                },
                context: Default::default(),
            },
        },
        Event {
            attempt_id: "prior".into(),
            ordinal: 4,
            sequence: 2,
            kind: EventKind::ProviderObservation {
                usage: Usage {
                    cached_input_tokens: None,
                    input_tokens: Some(3),
                    output_tokens: None,
                },
                receipt: json!({"response_id":"earlier-interrupted"}),
            },
        },
    ]);
    let prior = owner.0.clone();
    let error = recover::work(request().freeze().unwrap(), "test", &prior, 0)
        .unwrap()
        .run(
            &mut CancelAfterEvidence,
            &mut owner,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap_err();
    assert_eq!(error.to_string(), "execution cancelled");
    let progress = Progress::from_events(&owner.0);
    assert_eq!(
        (
            progress.calls,
            progress.known_tokens,
            progress.unknown_usage
        ),
        (5, 19, 1)
    );
    assert_eq!(
        owner
            .0
            .iter()
            .filter(|e| e.ordinal == 5 && matches!(e.kind, EventKind::ProviderObservation { .. }))
            .count(),
        3
    );
    assert!(owner.0.iter().any(|record| matches!(record,
        Event{ordinal:5,kind:EventKind::ProviderObservation{receipt:provenance,..},..}
        if provenance["response_id"] == "cancelled-response")));
    assert!(matches!(
        owner.0.last(),
        Some(Event {
            ordinal: 5,
            kind: EventKind::ProviderReturned(_),
            ..
        })
    ));
    // TestProduct::dispatch/complete panic, so this also proves no late effects.
}

#[tokio::test]
async fn expired_transport_settlement_allows_bounded_async_final_retention() {
    struct NeverReturns(usize);
    impl Provider for NeverReturns {
        async fn exchange(
            &mut self,
            _: &Request,
            cancel: &Cancellation,
            _: &RunLimits,
            observer: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.0 += 1;
            observer
                .emit(StreamEvent::Receipt {
                    identity: json!({"response_id":"async-retention"}),
                    usage: Usage::default(),
                })
                .await?;
            observer
                .emit(StreamEvent::Usage(Usage {
                    cached_input_tokens: None,
                    input_tokens: Some(10),
                    output_tokens: Some(5),
                }))
                .await?;
            cancel.cancel();
            std::future::pending().await
        }
    }
    struct AsyncEvidence {
        records: Vec<Event>,
        failure_attempts: usize,
        stall_failure: bool,
    }
    #[async_trait]
    impl engine::Product for AsyncEvidence {
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
        ) -> Result<(), harness::engine::ProductError> {
            Err("this fixture has no partial-result disposition".into())
        }
        async fn window_seed(
            &mut self,
        ) -> Result<harness::engine::WindowSeed, harness::engine::ProductError> {
            Err("working windows are unavailable for this fixture".into())
        }

        async fn message_sources(
            &mut self,
        ) -> Result<Option<Vec<harness::engine::ProductMessage>>, harness::engine::ProductError>
        {
            Ok(None)
        }

        async fn admit_binding(
            &mut self,
            _: &harness::engine::recipe::RequestBinding,
        ) -> Result<(), harness::engine::ProductError> {
            Ok(())
        }

        async fn assert_current(&mut self) -> Result<(), engine::ProductError> {
            Ok(())
        }
        async fn persist(&mut self, record: Event) -> Result<(), engine::ProductError> {
            if matches!(
                record,
                Event {
                    kind: EventKind::ProviderFailed { .. },
                    ..
                }
            ) {
                self.failure_attempts += 1;
                if self.stall_failure {
                    // A never-acknowledged final write remains uncertain; the
                    // loop must not retry it or wait forever in the new grace.
                    return std::future::pending().await;
                }
            }
            // A real asynchronous DB acknowledgment yields. Merely accepting
            // immediately-ready futures after the transport deadline is not enough.
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            self.records.push(record);
            Ok(())
        }
        fn execute(&mut self, _: &Invocation, _: Value) -> harness::engine::recipe::ToolFuture {
            panic!("cancelled transport cannot authorize effects")
        }
        async fn finish(&mut self, _: &Turn) -> Result<Finish, engine::ProductError> {
            panic!("cancelled transport cannot authorize publication")
        }
    }
    for stall_failure in [false, true] {
        let mut provider = NeverReturns(0);
        let mut owner = AsyncEvidence {
            records: vec![],
            failure_attempts: 0,
            stall_failure,
        };
        let cancel = Cancellation::default();
        let error = tokio::time::timeout(
            std::time::Duration::from_secs(3),
            recover::work(request().freeze().unwrap(), "test", &[], 0)
                .unwrap()
                .run(&mut provider, &mut owner, RunLimits::default(), &cancel),
        )
        .await
        .expect("transport plus final retention must finish within three seconds")
        .expect_err("cancelled transport cannot complete");
        assert!(cancel.is_cancelled());
        assert_eq!(provider.0, 1);
        assert_eq!(
            owner.failure_attempts, 1,
            "final failure retention is attempted exactly once"
        );
        let progress = Progress::from_events(&owner.records);
        assert_eq!(
            (
                progress.calls,
                progress.known_tokens,
                progress.unknown_usage
            ),
            (1, 15, 0)
        );
        let observations: Vec<_> = owner
            .records
            .iter()
            .filter_map(|record| match record {
                Event {
                    ordinal: 1,
                    kind:
                        EventKind::ProviderObservation {
                            usage,
                            receipt: provenance,
                            ..
                        },
                    ..
                } => Some((usage, provenance)),
                _ => None,
            })
            .collect();
        assert_eq!(
            observations.len(),
            3,
            "both streamed snapshots and final post-timeout observation must acknowledge asynchronously"
        );
        let (usage, identity) = observations.last().unwrap();
        assert_eq!(
            **usage,
            Usage {
                cached_input_tokens: None,
                input_tokens: Some(10),
                output_tokens: Some(5)
            }
        );
        assert_eq!(identity["response_id"], "async-retention");
        assert!(!owner.records.iter().any(|record| matches!(
            record,
            Event {
                kind: EventKind::ProviderReturned(_),
                ..
            }
        )));
        if !stall_failure {
            assert!(
                matches!(owner.records.last(), Some(Event{ordinal:1,kind:EventKind::ProviderFailed{reason},..})
                if reason == &error.to_string()),
                "the asynchronous final failure acknowledgment must finish"
            );
        }
        // No assertion that a pending failure write rolled back: its owner
        // reconciles that uncertainty. We assert only boundedness and no retry.
    }
}

#[tokio::test]
async fn cancellation_bounds_pending_owner_callbacks_without_assuming_rollback() {
    #[derive(Clone, Copy, Debug)]
    enum Boundary {
        Observation,
        Provisional,
        Check,
        Retention,
        SteeredRetention,
    }
    struct PendingOwner {
        boundary: Boundary,
        entered: Option<tokio::sync::oneshot::Sender<()>>,
        records: Vec<Event>,
        observed: bool,
    }
    impl PendingOwner {
        async fn hang(&mut self) -> Result<(), engine::ProductError> {
            self.entered
                .take()
                .expect("callback must not be retried")
                .send(())
                .unwrap();
            std::future::pending().await
        }
    }
    #[async_trait]
    impl engine::Product for PendingOwner {
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
        ) -> Result<(), harness::engine::ProductError> {
            Err("this fixture has no partial-result disposition".into())
        }
        async fn window_seed(
            &mut self,
        ) -> Result<harness::engine::WindowSeed, harness::engine::ProductError> {
            Err("working windows are unavailable for this fixture".into())
        }

        async fn message_sources(
            &mut self,
        ) -> Result<Option<Vec<harness::engine::ProductMessage>>, harness::engine::ProductError>
        {
            Ok(None)
        }

        async fn admit_binding(
            &mut self,
            _: &harness::engine::recipe::RequestBinding,
        ) -> Result<(), harness::engine::ProductError> {
            Ok(())
        }

        async fn assert_current(&mut self) -> Result<(), engine::ProductError> {
            if matches!(self.boundary, Boundary::SteeredRetention) && self.observed {
                return Err(engine::NotCurrent::Superseded.into());
            }
            if matches!(self.boundary, Boundary::Check) && self.observed {
                return self.hang().await;
            }
            Ok(())
        }
        async fn persist(&mut self, record: Event) -> Result<(), engine::ProductError> {
            let observation = matches!(
                record,
                Event {
                    kind: EventKind::ProviderObservation { .. },
                    ..
                }
            );
            let retention = matches!(
                record,
                Event {
                    kind: EventKind::ProviderReturned(_),
                    ..
                }
            );
            self.observed |= observation;
            // Model a write whose acknowledgment hangs after it may commit.
            // Dropping this future does not imply that write was rolled back.
            self.records.push(record);
            if (matches!(self.boundary, Boundary::Observation) && observation)
                || (matches!(
                    self.boundary,
                    Boundary::Retention | Boundary::SteeredRetention
                ) && retention)
            {
                return self.hang().await;
            }
            Ok(())
        }
        async fn live_text(
            &mut self,
            _: u64,
            _: engine::TextDelta,
        ) -> Result<TextDelivery, engine::ProductError> {
            if matches!(self.boundary, Boundary::Provisional) {
                self.hang().await?;
            }
            Ok(TextDelivery::Visible)
        }
        fn execute(&mut self, _: &Invocation, _: Value) -> harness::engine::recipe::ToolFuture {
            panic!("cancelled acknowledgment cannot authorize effects")
        }
        async fn finish(&mut self, _: &Turn) -> Result<Finish, engine::ProductError> {
            panic!("cancelled acknowledgment cannot authorize publication")
        }
    }
    struct ProviderAtBoundary {
        boundary: Boundary,
        calls: usize,
    }
    impl Provider for ProviderAtBoundary {
        async fn exchange(
            &mut self,
            _: &Request,
            cancel: &Cancellation,
            _: &RunLimits,
            observer: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.calls += 1;
            observer
                .emit(StreamEvent::Receipt {
                    identity: json!({"response_id":"pending-owner"}),
                    usage: Usage::default(),
                })
                .await?;
            observer
                .emit(StreamEvent::Usage(Usage {
                    cached_input_tokens: None,
                    input_tokens: Some(10),
                    output_tokens: None,
                }))
                .await?;
            if matches!(self.boundary, Boundary::Provisional) {
                observer
                    .emit(StreamEvent::Text("provisional only".into()))
                    .await?;
            }
            if matches!(self.boundary, Boundary::SteeredRetention) {
                while !cancel.is_cancelled() {
                    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                }
            }
            if matches!(
                self.boundary,
                Boundary::Retention | Boundary::SteeredRetention
            ) {
                return Ok(Turn {
                    items: vec![harness::engine::Item::assistant(
                        "not authorized for publication",
                    )],
                    usage: Usage::default(),
                    receipt: json!({"response_id":"pending-owner"}),
                });
            }
            std::future::pending().await
        }
    }
    for boundary in [
        Boundary::Observation,
        Boundary::Provisional,
        Boundary::Check,
        Boundary::Retention,
        Boundary::SteeredRetention,
    ] {
        let (entered, wait) = tokio::sync::oneshot::channel();
        let cancel = Cancellation::default();
        let cancellation = cancel.clone();
        // Cancel only after the selected callback is actually pending. No
        // scheduler-dependent sleep is needed to reach the intended boundary.
        let canceller = tokio::spawn(async move {
            wait.await.expect("selected callback must be reached");
            if !matches!(boundary, Boundary::SteeredRetention) {
                cancellation.cancel();
            }
        });
        let mut owner = PendingOwner {
            boundary,
            entered: Some(entered),
            records: vec![],
            observed: false,
        };
        let mut provider = ProviderAtBoundary { boundary, calls: 0 };
        let outcome = tokio::time::timeout(
            std::time::Duration::from_secs(3),
            recover::work(request().freeze().unwrap(), "test", &[], 0)
                .unwrap()
                .run(&mut provider, &mut owner, RunLimits::default(), &cancel),
        )
        .await;
        canceller.abort();
        assert!(
            outcome.is_ok(),
            "{boundary:?} acknowledgment must not defeat cancellation settlement"
        );
        let error = outcome
            .unwrap()
            .expect_err("interruption cannot return completion");
        assert_eq!(
            engine::AttemptFailure::of(&error, &cancel),
            engine::AttemptFailure::Interrupted,
            "a detected steer must survive a later retention drain timeout"
        );
        assert_eq!(
            cancel.is_cancelled(),
            !matches!(boundary, Boundary::SteeredRetention)
        );
        assert!(owner.entered.is_none(), "test must reach {boundary:?}");
        assert_eq!(provider.calls, 1);
        assert_eq!(Progress::from_events(&owner.records).calls, 1);
        // Intentionally do not assert pending writes disappeared: the owner
        // must reconcile their uncertain outcomes using its durable record.
    }
}

#[tokio::test]
async fn partial_usage_snapshots_merge_per_field_for_success_and_interruption() {
    struct PartialUsage {
        interrupted: bool,
        calls: usize,
    }
    impl Provider for PartialUsage {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &Cancellation,
            _: &RunLimits,
            observer: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.calls += 1;
            observer
                .emit(StreamEvent::Receipt {
                    identity: json!({"response_id":"partial-usage"}),
                    usage: Usage::default(),
                })
                .await?;
            observer
                .emit(StreamEvent::Usage(Usage {
                    cached_input_tokens: None,
                    input_tokens: Some(10),
                    output_tokens: None,
                }))
                .await?;
            observer
                .emit(StreamEvent::Usage(Usage {
                    cached_input_tokens: None,
                    input_tokens: None,
                    output_tokens: Some(5),
                }))
                .await?;
            if self.interrupted {
                return Err("transport interrupted after usage".into());
            }
            Ok(Turn {
                items: vec![
                    harness::engine::Item::assistant("Retained handoff text"),
                    Invocation {
                        id: "over-budget-effect".into(),
                        name: "save".into(),
                        arguments: json!({}),
                    }
                    .into(),
                ],
                usage: Usage::default(),
                receipt: json!({"response_id":"partial-usage"}),
            })
        }
    }
    for interrupted in [false, true] {
        let mut provider = PartialUsage {
            interrupted,
            calls: 0,
        };
        let mut owner = TestProduct::default();
        let bounds = RunLimits {
            max_calls: 1,
            ..RunLimits::default()
        };
        let result = recover::work(request().freeze().unwrap(), "test", &[], 0)
            .unwrap()
            .run(&mut provider, &mut owner, bounds, &Cancellation::default())
            .await;
        if interrupted {
            assert_eq!(
                result.unwrap_err().to_string(),
                "transport interrupted after usage"
            );
        } else {
            assert_eq!(result.unwrap().text(), "Retained handoff text");
        }
        let progress = Progress::from_events(&owner.0);
        assert_eq!(
            (
                progress.calls,
                progress.known_tokens,
                progress.unknown_usage
            ),
            (1, 15, 0)
        );
        if interrupted {
            assert!(matches!(
                owner.0.last(),
                Some(Event {
                    ordinal: 1,
                    kind: EventKind::ProviderFailed { .. },
                    ..
                })
            ));
        } else {
            assert!(
                matches!(owner.0.iter().find(|event| matches!(event.kind, EventKind::ProviderReturned(_))), Some(Event{ordinal:1,kind:EventKind::ProviderReturned(response),..})
                if response.usage == Usage {
            cached_input_tokens: None, input_tokens: Some(10), output_tokens: Some(5) })
            );
        }
    }
}

/// What the observer received while a fixture replayed: each text piece, and
/// the name of each call the provider announced as begun.
#[derive(Default)]
struct Observed {
    pieces: Vec<String>,
    began: Vec<String>,
}

/// Replays a scrubbed real provider stream from `fixtures/provider-streams`
/// over local HTTP. Returns the finished turn, what the observer received, and
/// the fixture's events.
async fn replay(fixture: &str) -> (Turn, Observed, Vec<Value>) {
    // One frame's data payload per line; `#` lines are the fixture's header.
    let payloads: Vec<&str> = fixture
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect();
    let events = payloads
        .iter()
        .map(|payload| serde_json::from_str(payload).unwrap())
        .collect();
    // The capture kept only each frame's data, so each frame is `data:` alone.
    let frames = payloads
        .iter()
        .map(|payload| format!("data: {payload}\n\n"))
        .collect();
    let mut provider = ResponsesProvider::new(
        server(frames, None).await,
        "synthetic".into(),
        std::sync::Arc::new(quota::Gate::default()),
    )
    .unwrap();
    let mut observed = Observed::default();
    let turn = provider
        .exchange(
            &request(),
            &Cancellation::default(),
            &RunLimits::default(),
            &mut |event| {
                match event {
                    StreamEvent::Text(delta) => observed.pieces.push(delta.text),
                    StreamEvent::CallBegan { name } => observed.began.push(name),
                    StreamEvent::Usage(_) | StreamEvent::Receipt { .. } => {}
                }
                Ok(())
            },
        )
        .await
        .unwrap();
    (turn, observed, events)
}
fn of_type<'a>(events: &'a [Value], kind: &'a str) -> impl Iterator<Item = &'a Value> {
    events.iter().filter(move |event| event["type"] == kind)
}
/// The turn the provider contract promises: each completed output item, in
/// output order.
fn completed_items(events: &[Value]) -> Vec<engine::Item> {
    let mut done: Vec<&Value> = of_type(events, "response.output_item.done").collect();
    done.sort_by_key(|event| event["output_index"].as_u64());
    done.into_iter()
        .map(|event| {
            let item = &event["item"];
            let text = |key: &str| item[key].as_str().unwrap().to_owned();
            match item["type"].as_str().unwrap() {
                "function_call" => engine::Item::Call(Invocation {
                    id: text("call_id"),
                    name: text("name"),
                    arguments: text("arguments"),
                }),
                "message" => engine::Item::Message {
                    role: text("role"),
                    text: item["content"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|part| part["text"].as_str().unwrap())
                        .collect(),
                },
                "reasoning" => engine::Item::Opaque(item.clone()),
                other => panic!("fixture holds an unexpected item type {other}"),
            }
        })
        .collect()
}
fn completed_usage(events: &[Value]) -> Usage {
    let completed = of_type(events, "response.completed").next().unwrap();
    let usage = &completed["response"]["usage"];
    Usage {
        input_tokens: usage["input_tokens"].as_u64(),
        cached_input_tokens: usage["input_tokens_details"]["cached_tokens"].as_u64(),
        output_tokens: usage["output_tokens"].as_u64(),
    }
}
/// The fixture holds these event types and top-level keys the harness does not
/// use, so a successful replay shows they cause no failure.
fn assert_holds(events: &[Value], kinds: &[&str], keys: &[&str]) {
    for kind in kinds {
        assert!(of_type(events, kind).next().is_some(), "no {kind} event");
    }
    for key in keys {
        assert!(
            events.iter().any(|event| event.get(key).is_some()),
            "no {key} key"
        );
    }
}

#[tokio::test]
async fn captured_call_with_argument_deltas_yields_the_completed_call() {
    let (turn, observed, events) = replay(include_str!(
        "fixtures/provider-streams/call-with-argument-deltas.txt"
    ))
    .await;
    assert_holds(
        &events,
        &[
            "response.in_progress",
            "response.function_call_arguments.delta",
            "response.function_call_arguments.done",
            "response.metadata",
        ],
        &["safety_buffering", "obfuscation", "sequence_number"],
    );
    let expected = completed_items(&events);
    assert!(matches!(&expected[..], [engine::Item::Call(call)] if call.name == "skill_view"));
    assert_eq!(turn.items, expected);
    assert!(observed.pieces.is_empty());
    // The call is announced before its arguments arrive, and begins once.
    assert_eq!(observed.began, ["skill_view"]);
    assert_eq!(turn.usage, completed_usage(&events));
}

#[tokio::test]
async fn captured_calls_without_argument_deltas_yield_the_completed_calls() {
    let (turn, observed, events) = replay(include_str!(
        "fixtures/provider-streams/reasoning-then-calls-without-argument-deltas.txt"
    ))
    .await;
    assert_holds(
        &events,
        &[
            "response.in_progress",
            "response.function_call_arguments.done",
            "response.metadata",
        ],
        &["safety_buffering", "sequence_number"],
    );
    assert!(
        of_type(&events, "response.function_call_arguments.delta")
            .next()
            .is_none()
    );
    // Each call's arguments are its completed item's arguments.
    let expected = completed_items(&events);
    assert!(matches!(
        &expected[..],
        [engine::Item::Opaque(_), calls @ ..] if calls.len() == 4
            && calls.iter().all(|item| matches!(item, engine::Item::Call(_)))
    ));
    assert_eq!(turn.items, expected);
    // Each announced call begins once, in the order announced; the reasoning
    // item begins no call.
    assert_eq!(
        observed.began,
        [
            "search_patient_information",
            "search_patient_information",
            "search_patient_information",
            "read_patient_sources"
        ]
    );
    assert_eq!(turn.usage, completed_usage(&events));
}

#[tokio::test]
async fn captured_text_answer_is_the_completed_message_not_the_joined_deltas() {
    let (turn, observed, events) = replay(include_str!(
        "fixtures/provider-streams/text-answer-trimmed.txt"
    ))
    .await;
    assert_holds(
        &events,
        &[
            "response.in_progress",
            "response.content_part.added",
            "response.metadata",
            "response.output_text.done",
            "response.content_part.done",
        ],
        &[
            "safety_buffering",
            "obfuscation",
            "logprobs",
            "sequence_number",
        ],
    );
    let deltas: Vec<&str> = of_type(&events, "response.output_text.delta")
        .map(|event| event["delta"].as_str().unwrap())
        .collect();
    assert_eq!(observed.pieces, deltas);
    assert!(observed.began.is_empty());
    // The fixture keeps only the first and last deltas, so they no longer
    // join to the completed text.
    let expected = completed_items(&events);
    assert!(matches!(
        &expected[..],
        [engine::Item::Message { text, .. }] if *text != deltas.concat()
    ));
    assert_eq!(turn.items, expected);
    assert_eq!(turn.usage, completed_usage(&events));
}

#[path = "support/quota.rs"]
mod quota;
