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
async fn cancelled_exchange_retains_cumulative_evidence_without_double_counting() {
    struct CancelAfterEvidence;
    impl Provider for CancelAfterEvidence {
        async fn exchange(
            &mut self,
            _: &Request,
            cancel: &Cancellation,
            emit: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
        ) -> Result<Response, String> {
            emit(StreamEvent::ProviderIdentity(
                json!({"response_id":"cancelled-response"}),
            ))?;
            emit(StreamEvent::Usage(Usage {
                input_tokens: Some(11),
                output_tokens: None,
            }))?;
            emit(StreamEvent::Usage(Usage {
                input_tokens: Some(11),
                output_tokens: Some(5),
            }))?;
            cancel.cancel();
            // Even a successful terminal arriving after cancellation is evidence,
            // never permission to execute its tools or publish its text.
            Ok(Response {
                text: "must not publish".into(),
                calls: vec![Call {
                    id: "late-effect".into(),
                    name: "save".into(),
                    arguments: json!({}),
                }],
                usage: Usage {
                    input_tokens: Some(11),
                    output_tokens: Some(5),
                },
                provenance: json!({"response_id":"cancelled-response"}),
            })
        }
    }
    let mut owner = Product(vec![
        Record::Dispatch {
            sequence: 4,
            request: request(),
        },
        Record::Observation {
            sequence: 4,
            usage: Usage {
                input_tokens: Some(3),
                output_tokens: None,
            },
            provenance: json!({"response_id":"earlier-interrupted"}),
        },
    ]);
    let mut progress = Progress::from_records(&owner.0);
    let mut events = Vec::new();
    let error = run(
        &mut CancelAfterEvidence,
        &mut owner,
        request(),
        &mut progress,
        Limits::default(),
        &Cancellation::default(),
        &mut |event| {
            events.push(event);
            Ok(())
        },
    )
    .await
    .unwrap_err();
    assert_eq!(error, "execution cancelled");
    assert_eq!(
        progress,
        Progress {
            calls: 5,
            known_tokens: 19,
            unknown_usage: 1
        }
    );
    assert_eq!(Progress::from_records(&owner.0), progress);
    assert_eq!(events.len(), 3);
    assert!(owner.0.iter().any(|record| matches!(record,
        Record::Observation { sequence: 5, provenance, .. }
        if provenance["response_id"] == "cancelled-response")));
    assert!(matches!(
        owner.0.last(),
        Some(Record::Exchange { sequence: 5, .. })
    ));
    // Product::dispatch/complete panic, so this also proves no late effects.
}

#[tokio::test]
async fn noncooperative_cancelled_provider_settles_with_observed_evidence() {
    struct NeverSettles {
        calls: usize,
        output_tokens: Option<u64>,
    }
    impl Provider for NeverSettles {
        async fn exchange(
            &mut self,
            _: &Request,
            cancel: &Cancellation,
            emit: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
        ) -> Result<Response, String> {
            self.calls += 1;
            emit(StreamEvent::ProviderIdentity(
                json!({"response_id":"noncooperative-response"}),
            ))?;
            emit(StreamEvent::Usage(Usage {
                input_tokens: Some(7),
                output_tokens: None,
            }))?;
            emit(StreamEvent::Usage(Usage {
                input_tokens: Some(11),
                output_tokens: self.output_tokens,
            }))?;
            cancel.cancel();
            // Ignore the token forever: the loop, not provider cooperation,
            // must bound settlement while retaining already observed evidence.
            std::future::pending().await
        }
    }
    for output_tokens in [None, Some(5)] {
        let mut provider = NeverSettles {
            calls: 0,
            output_tokens,
        };
        // These owner callbacks are responsive; hanging owner callbacks are a
        // different contract. Product panics on effects or final publication.
        let mut owner = Product::default();
        let mut progress = Progress::default();
        let cancel = Cancellation::default();
        let error = tokio::time::timeout(
            std::time::Duration::from_secs(3),
            run(
                &mut provider,
                &mut owner,
                request(),
                &mut progress,
                Limits::default(),
                &cancel,
                &mut |_| Ok(()),
            ),
        )
        .await
        .expect("cancelled provider must settle within three seconds, even if it never returns")
        .expect_err("a cancelled, unfinished exchange cannot complete");
        assert!(error.contains("cancel"), "unexpected failure: {error}");
        assert!(cancel.is_cancelled());
        assert_eq!(provider.calls, 1, "settlement never retries the provider");
        assert_eq!(
            progress,
            Progress {
                calls: 1,
                known_tokens: 11 + output_tokens.unwrap_or(0),
                unknown_usage: u64::from(output_tokens.is_none()),
            }
        );
        assert_eq!(
            Progress::from_records(&owner.0),
            progress,
            "cumulative snapshots must not be added twice or unknown counts zero-filled"
        );
        assert!(matches!(
            owner.0.first(),
            Some(Record::Dispatch { sequence: 1, .. })
        ));
        assert!(owner.0.iter().any(|record| matches!(record,
            Record::Observation { sequence: 1, usage, provenance }
            if usage.input_tokens == Some(11)
                && usage.output_tokens == output_tokens
                && provenance["response_id"] == "noncooperative-response")));
        assert!(
            matches!(owner.0.last(), Some(Record::Failure { sequence: 1, reason })
            if reason == &error)
        );
        assert!(
            !owner
                .0
                .iter()
                .any(|record| matches!(record, Record::Exchange { .. })),
            "interrupted evidence must not become a successful exchange"
        );
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
    }
    struct PendingOwner {
        boundary: Boundary,
        entered: Option<tokio::sync::oneshot::Sender<()>>,
        records: Vec<Record>,
        observed: bool,
    }
    impl PendingOwner {
        async fn hang(&mut self) -> Result<(), String> {
            self.entered
                .take()
                .expect("callback must not be retried")
                .send(())
                .unwrap();
            std::future::pending().await
        }
    }
    impl Owner for PendingOwner {
        async fn check(&mut self) -> Result<(), String> {
            if matches!(self.boundary, Boundary::Check) && self.observed {
                return self.hang().await;
            }
            Ok(())
        }
        async fn record(&mut self, record: Record) -> Result<(), String> {
            let observation = matches!(record, Record::Observation { .. });
            let retention = matches!(record, Record::Exchange { .. });
            self.observed |= observation;
            // Model a write whose acknowledgment hangs after it may commit.
            // Dropping this future does not imply that write was rolled back.
            self.records.push(record);
            if (matches!(self.boundary, Boundary::Observation) && observation)
                || (matches!(self.boundary, Boundary::Retention) && retention)
            {
                return self.hang().await;
            }
            Ok(())
        }
        async fn provisional(&mut self, _: u64, _: &str) -> Result<(), String> {
            if matches!(self.boundary, Boundary::Provisional) {
                return self.hang().await;
            }
            Ok(())
        }
        async fn dispatch(&mut self, _: &Call, _: &Cancellation) -> Result<Value, String> {
            panic!("cancelled acknowledgment cannot authorize effects")
        }
        async fn complete(&mut self, _: &Response) -> Result<(), String> {
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
            _: &Cancellation,
            emit: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
        ) -> Result<Response, String> {
            self.calls += 1;
            emit(StreamEvent::ProviderIdentity(
                json!({"response_id":"pending-owner"}),
            ))?;
            emit(StreamEvent::Usage(Usage {
                input_tokens: Some(10),
                output_tokens: None,
            }))?;
            if matches!(self.boundary, Boundary::Provisional) {
                emit(StreamEvent::Text("provisional only".into()))?;
            }
            if matches!(self.boundary, Boundary::Retention) {
                return Ok(Response {
                    text: "not authorized for publication".into(),
                    calls: vec![],
                    usage: Usage::default(),
                    provenance: json!({"response_id":"pending-owner"}),
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
    ] {
        let (entered, wait) = tokio::sync::oneshot::channel();
        let cancel = Cancellation::default();
        let cancellation = cancel.clone();
        // Cancel only after the selected callback is actually pending. No
        // scheduler-dependent sleep is needed to reach the intended boundary.
        let canceller = tokio::spawn(async move {
            wait.await.expect("selected callback must be reached");
            cancellation.cancel();
        });
        let mut owner = PendingOwner {
            boundary,
            entered: Some(entered),
            records: vec![],
            observed: false,
        };
        let mut provider = ProviderAtBoundary { boundary, calls: 0 };
        let mut progress = Progress::default();
        let outcome = tokio::time::timeout(
            std::time::Duration::from_secs(3),
            run(
                &mut provider,
                &mut owner,
                request(),
                &mut progress,
                Limits::default(),
                &cancel,
                &mut |_| Ok(()),
            ),
        )
        .await;
        canceller.abort();
        assert!(
            outcome.is_ok(),
            "{boundary:?} acknowledgment must not defeat cancellation settlement"
        );
        assert!(
            outcome.unwrap().is_err(),
            "{boundary:?} cannot return completion"
        );
        assert!(cancel.is_cancelled());
        assert!(owner.entered.is_none(), "test must reach {boundary:?}");
        assert_eq!(provider.calls, 1);
        assert_eq!(progress.calls, 1);
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
            emit: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
        ) -> Result<Response, String> {
            self.calls += 1;
            emit(StreamEvent::ProviderIdentity(
                json!({"response_id":"partial-usage"}),
            ))?;
            emit(StreamEvent::Usage(Usage {
                input_tokens: Some(10),
                output_tokens: None,
            }))?;
            emit(StreamEvent::Usage(Usage {
                input_tokens: None,
                output_tokens: Some(5),
            }))?;
            if self.interrupted {
                return Err("transport interrupted after usage".into());
            }
            Ok(Response {
                text: "not publishable above budget".into(),
                calls: vec![Call {
                    id: "over-budget-effect".into(),
                    name: "save".into(),
                    arguments: json!({}),
                }],
                usage: Usage::default(),
                provenance: json!({"response_id":"partial-usage"}),
            })
        }
    }
    for interrupted in [false, true] {
        let mut provider = PartialUsage {
            interrupted,
            calls: 0,
        };
        let mut owner = Product::default();
        let mut progress = Progress::default();
        let bounds = Limits {
            max_known_tokens: 14,
            ..Limits::default()
        };
        let error = run(
            &mut provider,
            &mut owner,
            request(),
            &mut progress,
            bounds,
            &Cancellation::default(),
            &mut |_| Ok(()),
        )
        .await
        .unwrap_err();
        assert_eq!(
            error,
            if interrupted {
                "transport interrupted after usage"
            } else {
                "execution usage bound reached"
            }
        );
        assert_eq!(
            progress,
            Progress {
                calls: 1,
                known_tokens: 15,
                unknown_usage: 0
            }
        );
        assert_eq!(Progress::from_records(&owner.0), progress);
        if interrupted {
            assert!(matches!(
                owner.0.last(),
                Some(Record::Failure { sequence: 1, .. })
            ));
        } else {
            assert!(
                matches!(owner.0.last(), Some(Record::Exchange { sequence: 1, response })
                if response.usage == Usage { input_tokens: Some(10), output_tokens: Some(5) })
            );
        }
        let record_count = owner.0.len();
        // Re-enter using the same live counters, just as owner-controlled
        // recovery would. Lost input usage would incorrectly permit dispatch.
        let error = run(
            &mut provider,
            &mut owner,
            request(),
            &mut progress,
            bounds,
            &Cancellation::default(),
            &mut |_| Ok(()),
        )
        .await
        .unwrap_err();
        assert_eq!(error, "execution usage bound reached");
        assert_eq!(
            provider.calls, 1,
            "merged usage must prevent another provider call"
        );
        assert_eq!(owner.0.len(), record_count);
    }
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
