#[path = "support/recover.rs"]
mod recover;
#[path = "support/requests.rs"]
mod request_records;
use async_trait::async_trait;
use harness::engine::{
    self, AttemptFailure, Event, EventKind, Failure, Invocation, RunLimits, recipe::*,
};
use serde_json::{Value, json};

fn request() -> Request {
    Request {
        cache_key: None,
        model: "controlled".into(),
        context_window_tokens: 100_000,
        max_input_tokens: 100_000,
        options: json!({}),
        instructions: "original".into(),
        tools: ["exact_read", "finish"]
            .into_iter()
            .map(|name| {
                engine::Tool {
                    name: name.into(),
                    description: name.into(),
                    parameters: json!({"type":"object"}),
                }
                .function()
            })
            .collect(),
        items: vec![],
    }
}
fn response(calls: Vec<Invocation<Value>>) -> Turn {
    Turn {
        items: {
            let mut items = vec![Item::assistant("answer")];
            items.extend(calls.into_iter().map(Item::from));
            items
        },
        usage: Usage {
            cached_input_tokens: None,
            input_tokens: Some(3),
            output_tokens: Some(2),
        },
        receipt: json!({"mode": "controlled"}),
    }
}
fn call(id: &str) -> Invocation<Value> {
    Invocation {
        id: id.into(),
        name: "exact_read".into(),
        arguments: json!({"revision": 7}),
    }
}
#[derive(Default)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "each bool injects one independent fault into the fake"
)]
struct TestProduct {
    records: Vec<Event>,
    effects: usize,
    completed: bool,
    stale: bool,
    withdraw_on_response: bool,
    fail_effect: bool,
    partial: bool,
}
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
        if self.stale || self.partial {
            Err(engine::NotCurrent::Withdrawn.into())
        } else {
            Ok(())
        }
    }
    async fn persist(&mut self, record: Event) -> Result<(), engine::ProductError> {
        if matches!(
            record,
            Event {
                kind: EventKind::ProviderReturned(_),
                ..
            }
        ) && self.withdraw_on_response
        {
            self.stale = true;
        }
        self.records.push(record);
        Ok(())
    }
    fn execute(&mut self, _: &Invocation, _: Value) -> harness::engine::recipe::ToolFuture {
        let outcome = (|| {
            if self.fail_effect {
                return Err("uncertain effect retained by owner".into());
            }
            self.effects += 1;
            Ok(ToolOutcome::Returned(json!({"exact": true}).to_string()))
        })();
        Box::pin(std::future::ready(outcome))
    }
    async fn finish(&mut self, _: &Turn) -> Result<Finish, engine::ProductError> {
        self.assert_current().await?;
        self.completed = true;
        Ok(Finish::Committed)
    }
    async fn finish_partial(
        &mut self,
        _: &Turn,
        _: &[engine::LimitBackstop],
    ) -> Result<(), engine::ProductError> {
        self.partial = true;
        Ok(())
    }
}
struct Controlled {
    replies: Vec<Turn>,
    cancel: bool,
    fail: bool,
}
impl Provider for Controlled {
    async fn exchange(
        &mut self,
        _: &Request,
        token: &Cancellation,
        _: &RunLimits,
        observer: &mut dyn Observer,
    ) -> Result<Turn, engine::Cause> {
        if self.fail {
            return Err("provider unavailable".into());
        }
        let response = self.replies.remove(0);
        observer
            .emit(StreamEvent::Text(response.text().into()))
            .await?;
        if self.cancel {
            token.cancel();
        }
        Ok(response)
    }
}
/// Restart after the provider return was saved. Capture the first successor
/// dispatch so assertions cover what the model actually received.
async fn restart_after_return(
    owner: &mut dyn engine::Product,
    request: Request,
    returned: &Turn,
) -> Result<Option<Request>, engine::Failure> {
    struct Successor(Option<Request>);
    impl Provider for Successor {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &Cancellation,
            _: &RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.0 = Some(request.clone());
            Ok(response(Vec::new()))
        }
    }
    let events = [
        Event {
            attempt_id: "first".into(),
            sequence: 1,
            ordinal: 1,
            kind: EventKind::ProviderStarted {
                dispatched_at_unix_ms: 1,
                request: request_records::saved("first", 1, &request),
                context: Default::default(),
            },
        },
        Event {
            attempt_id: "first".into(),
            sequence: 2,
            ordinal: 1,
            kind: EventKind::ProviderReturned(returned.clone()),
        },
    ];
    let mut provider = Successor(None);
    recover::work(request.freeze()?, "successor", &events, 0)?
        .run(
            &mut provider,
            owner,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await?;
    Ok(provider.0)
}

#[tokio::test]
async fn a_call_reusing_an_id_from_the_context_is_saved_under_a_new_id_and_runs() {
    let mut req = request();
    req.items = vec![
        Item::from(call("already-read")),
        Item::Result {
            call_id: "already-read".into(),
            output: engine::results::ResultView::inline(json!({"retained": true}).to_string()),
        },
    ];
    let reused = response(vec![call("new-sibling"), call("already-read")]);
    let mut provider = Controlled {
        replies: vec![reused.clone(), response(vec![])],
        cancel: false,
        fail: false,
    };
    let mut owner = TestProduct::default();
    recover::work(req.freeze().unwrap(), "test", &[], 0)
        .unwrap()
        .run(
            &mut provider,
            &mut owner,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    let retained = owner
        .records
        .iter()
        .find_map(|record| match &record.kind {
            EventKind::ProviderReturned(turn) => Some(turn.clone()),
            _ => None,
        })
        .unwrap();
    let ids: Vec<_> = retained.calls().map(|call| call.id.as_str()).collect();
    assert_eq!(ids[0], "new-sibling");
    assert_ne!(ids[1], "already-read", "the saved response has the new ID");
    let returned: Vec<_> = owner
        .records
        .iter()
        .filter_map(|record| match &record.kind {
            EventKind::ToolReturned { call_id, .. } => Some(call_id.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(
        returned, ids,
        "each result is saved under the saved call ID"
    );
    assert_eq!(owner.effects, 2, "both calls run once");
    assert!(owner.completed);
}

#[tokio::test]
async fn late_stale_output_retains_usage_without_effects() {
    let mut provider = Controlled {
        replies: vec![response(vec![call("1")])],
        cancel: false,
        fail: false,
    };
    let mut owner = TestProduct {
        withdraw_on_response: true,
        ..Default::default()
    };
    let error = recover::work(request().freeze().unwrap(), "test", &[], 0)
        .unwrap()
        .run(
            &mut provider,
            &mut owner,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap_err();
    assert!(
        matches!(error, Failure::NotCurrent(engine::NotCurrent::Withdrawn)),
        "{error}"
    );
    assert_eq!(
        AttemptFailure::of(&error, &Cancellation::default()),
        AttemptFailure::Interrupted
    );
    assert_eq!(Progress::from_events(&owner.records).known_tokens, 5);
    assert_eq!(owner.effects, 0);
    assert!(!owner.completed);
    assert!(matches!(
        owner.records.last(),
        Some(Event {
            kind: EventKind::ProviderReturned(_),
            ..
        })
    ));
}

#[test]
fn restored_usage_merges_within_an_attempt_never_across_attempts() {
    let observation = |attempt: &str, sequence, ordinal, input_tokens, output_tokens| Event {
        attempt_id: attempt.into(),
        sequence,
        ordinal,
        kind: EventKind::ProviderObservation {
            usage: Usage {
                cached_input_tokens: None,
                input_tokens,
                output_tokens,
            },
            receipt: json!({"synthetic": "retained-usage"}),
        },
    };
    let historical = [
        observation("first", 1, 1, Some(10), None),
        observation("second", 1, 1, Some(20), None),
        observation("first", 2, 1, None, Some(5)),
        observation("first", 3, 1, None, None),
    ];
    assert_eq!(
        Progress::from_events(&historical),
        Progress {
            calls: 2,
            known_tokens: 35,
            unknown_usage: 1,
            budget_tokens: 35,
            ..Progress::default()
        },
        "late field updates merge within their attempt, never across attempts"
    );
}

#[tokio::test]
async fn fresh_allowance_counts_saved_calls_not_monotonic_provider_ordinals() {
    let started = |sequence, ordinal| Event {
        attempt_id: "attempt".into(),
        sequence,
        ordinal,
        kind: EventKind::ProviderStarted {
            dispatched_at_unix_ms: 1,
            request: request_records::saved("attempt", sequence, &request()),
            context: Default::default(),
        },
    };
    let events = [started(1, 99), started(2, 100), started(3, 101)];
    let rows: Vec<_> = events.iter().cloned().map(|event| (event, false)).collect();
    let mut provider = Controlled {
        replies: vec![response(vec![])],
        cancel: false,
        fail: false,
    };
    let mut owner = TestProduct::default();
    let turn = recover::from_rows(request().freeze().unwrap(), "attempt", &rows, 100)
        .unwrap()
        .run(
            &mut provider,
            &mut owner,
            RunLimits {
                max_calls: 2,
                ..RunLimits::default()
            },
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(turn.text(), "answer");
    assert!(provider.replies.is_empty());
    assert!(matches!(
        owner.records.first(),
        Some(Event {
            ordinal: 102,
            kind: EventKind::ProviderStarted { .. },
            ..
        })
    ));

    let retained_suffix = [started(1, 100)];
    let rows: Vec<_> = retained_suffix
        .iter()
        .cloned()
        .map(|event| (event, false))
        .collect();
    let mut provider = Controlled {
        replies: vec![response(vec![])],
        cancel: false,
        fail: false,
    };
    let mut owner = TestProduct::default();
    let error = recover::from_rows(request().freeze().unwrap(), "attempt", &rows, 90)
        .unwrap()
        .run(
            &mut provider,
            &mut owner,
            RunLimits {
                max_calls: 10,
                ..RunLimits::default()
            },
            &Cancellation::default(),
        )
        .await
        .unwrap_err();
    assert_eq!(error.to_string(), "model calls");
    assert_eq!(provider.replies.len(), 1);
    assert!(owner.records.is_empty());
}

/// Declines the first `moves` terminal exchanges because its inputs moved,
/// then commits.
struct Moving {
    records: Vec<Event>,
    moves: usize,
    committed: usize,
    partials: usize,
}
#[async_trait]
impl engine::Product for Moving {
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

    fn terminal(&self, response: &Turn) -> bool {
        response.calls().any(|call| call.name == "finish")
    }
    async fn assert_current(&mut self) -> Result<(), engine::ProductError> {
        Ok(())
    }
    async fn persist(&mut self, record: Event) -> Result<(), engine::ProductError> {
        self.records.push(record);
        Ok(())
    }
    fn execute(&mut self, _: &Invocation, _: Value) -> harness::engine::recipe::ToolFuture {
        let outcome = Ok(ToolOutcome::Returned(json!({"read": true}).to_string()));
        Box::pin(std::future::ready(outcome))
    }
    async fn finish(&mut self, _: &Turn) -> Result<Finish, engine::ProductError> {
        if self.moves > 0 {
            self.moves -= 1;
            return Ok(Finish::Continue(json!({"outcome": "inputs_changed"})));
        }
        self.committed += 1;
        Ok(Finish::Committed)
    }
    async fn finish_partial(
        &mut self,
        _: &Turn,
        _: &[engine::LimitBackstop],
    ) -> Result<(), engine::ProductError> {
        self.partials += 1;
        Ok(())
    }
}
fn finish(id: &str) -> Invocation<Value> {
    Invocation {
        id: id.into(),
        name: "finish".into(),
        arguments: json!({}),
    }
}

#[tokio::test]
async fn declined_finish_continues_the_same_run_and_then_commits() {
    let mut provider = Controlled {
        replies: vec![
            response(vec![finish("finish-1")]),
            response(vec![call("read-1")]),
            response(vec![finish("finish-2")]),
        ],
        cancel: false,
        fail: false,
    };
    let mut owner = Moving {
        records: vec![],
        moves: 1,
        committed: 0,
        partials: 0,
    };
    recover::work(request().freeze().unwrap(), "test", &[], 0)
        .unwrap()
        .run(
            &mut provider,
            &mut owner,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(owner.committed, 1);
    assert_eq!(Progress::from_events(&owner.records).calls, 3);
    let dispatches = request_records::dispatched(&owner.records, &request());
    assert_eq!(dispatches.len(), 3);
    // The second dispatch carries the declined finish and its product result.
    assert_eq!(
        dispatches[1].items,
        vec![
            Item::Message {
                role: "assistant".into(),
                text: "answer".into(),
            },
            Item::from(finish("finish-1")),
            Item::Result {
                call_id: "finish-1".into(),
                output: engine::results::ResultView::inline(
                    json!({"outcome": "inputs_changed"}).to_string(),
                ),
            },
        ]
    );
}

#[tokio::test]
async fn mixed_terminal_batch_reaches_product_validation_before_any_effect() {
    struct AtomicTerminal {
        seen: Vec<Turn>,
        effects: usize,
    }
    #[async_trait]
    impl engine::Product for AtomicTerminal {
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

        fn terminal(&self, response: &Turn) -> bool {
            response.calls().any(|call| call.name == "finish")
        }
        async fn assert_current(&mut self) -> Result<(), engine::ProductError> {
            Ok(())
        }
        async fn persist(&mut self, _: Event) -> Result<(), engine::ProductError> {
            Ok(())
        }
        fn execute(&mut self, _: &Invocation, _: Value) -> harness::engine::recipe::ToolFuture {
            let outcome = {
                self.effects += 1;
                Ok(ToolOutcome::Returned(Value::Null.to_string()))
            };
            Box::pin(std::future::ready(outcome))
        }
        async fn finish(&mut self, response: &Turn) -> Result<Finish, engine::ProductError> {
            self.seen.push(response.clone());
            Err("product rejects mixed terminal effects".into())
        }
    }
    for calls in [
        vec![call("effect"), finish("terminal")],
        vec![finish("terminal"), call("effect")],
        vec![call("duplicate"), finish("duplicate")],
    ] {
        let exchange = response(calls);
        let duplicate = exchange.calls().next().unwrap().id == exchange.calls().nth(1).unwrap().id;
        let mut provider = Controlled {
            replies: vec![exchange.clone()],
            cancel: false,
            fail: false,
        };
        let mut owner = AtomicTerminal {
            seen: vec![],
            effects: 0,
        };
        let error = recover::work(request().freeze().unwrap(), "test", &[], 0)
            .unwrap()
            .run(
                &mut provider,
                &mut owner,
                RunLimits::default(),
                &Cancellation::default(),
            )
            .await
            .unwrap_err();
        assert_eq!(owner.effects, 0);
        assert_eq!(error.to_string(), "product rejects mixed terminal effects");
        assert_eq!(owner.seen.len(), 1);
        let retained = &owner.seen[0];
        assert_eq!(
            (&retained.receipt, &retained.usage),
            (&exchange.receipt, &exchange.usage)
        );
        if duplicate {
            let ids: Vec<_> = retained.calls().map(|call| &call.id).collect();
            assert_ne!(
                ids[0], ids[1],
                "the repeated ID is replaced before validation"
            );
        } else {
            assert_eq!(retained.items, exchange.items);
        }
        let saved = retained.clone();
        assert_eq!(
            restart_after_return(&mut owner, request(), &saved)
                .await
                .unwrap_err()
                .to_string(),
            "product rejects mixed terminal effects"
        );
        assert_eq!(
            owner.effects, 0,
            "recovery cannot execute terminal siblings either"
        );
    }
}

#[tokio::test]
async fn source_refresh_keeps_restored_ordinals_usage_and_request_ceiling() {
    {
        let mut provider = Controlled {
            replies: vec![response(vec![finish("refresh-1")]), response(vec![])],
            cancel: false,
            fail: false,
        };
        let mut saved = Vec::new();
        for ordinal in 1..=7 {
            saved.push(Event {
                attempt_id: "prior".into(),
                sequence: ordinal * 2 - 1,
                ordinal,
                kind: EventKind::ProviderStarted {
                    dispatched_at_unix_ms: 1,
                    request: request_records::saved("prior", ordinal * 2 - 1, &request()),
                    context: Default::default(),
                },
            });
            let usage = match ordinal {
                1 => Usage {
                    input_tokens: Some(5),
                    output_tokens: None,
                    cached_input_tokens: Some(0),
                },
                2 => Usage {
                    input_tokens: None,
                    output_tokens: Some(6),
                    cached_input_tokens: Some(0),
                },
                _ => Usage {
                    input_tokens: Some(0),
                    output_tokens: Some(0),
                    cached_input_tokens: Some(0),
                },
            };
            saved.push(Event {
                attempt_id: "prior".into(),
                sequence: ordinal * 2,
                ordinal,
                kind: EventKind::ProviderReturned(Turn {
                    items: vec![],
                    receipt: json!({"seed":"restored usage"}),
                    usage,
                }),
            });
        }
        let mut owner = Moving {
            records: saved.clone(),
            moves: usize::MAX,
            committed: 0,
            partials: 0,
        };
        let rows: Vec<_> = saved.iter().cloned().map(|event| (event, false)).collect();
        let error = recover::from_rows(request().freeze().unwrap(), "test", &rows, 0)
            .unwrap()
            .run(
                &mut provider,
                &mut owner,
                RunLimits {
                    max_calls: 9,
                    ..RunLimits::default()
                },
                &Cancellation::default(),
            )
            .await
            .unwrap();
        assert_eq!(error.text(), "answer");
        let progress = Progress::from_events(&owner.records);
        assert_eq!(
            (
                progress.calls,
                progress.known_tokens,
                progress.unknown_usage
            ),
            (9, 21, 2)
        );
        assert!(provider.replies.is_empty());
        assert_eq!(owner.committed, 0);
        assert_eq!(owner.partials, 1);
        assert_eq!(owner.moves, usize::MAX - 1);
        assert_eq!(
            owner
                .records
                .iter()
                .filter_map(|event| (event.attempt_id == "test"
                    && matches!(event.kind, EventKind::ProviderStarted { .. }))
                .then_some(event.ordinal))
                .collect::<Vec<_>>(),
            vec![8, 9]
        );
        assert!(
            owner
                .records
                .iter()
                .any(|e| matches!(e.kind, EventKind::ProviderReturned(_)))
        );
    }
}

#[tokio::test]
async fn responses_recovery_preserves_order_raw_arguments_and_per_call_feedback() {
    let reasoning =
        json!({"type":"reasoning","id":"r","summary":[], "encrypted_content":"fixture"});
    let raw = " { \"revision\" : 7 } ";
    let response = Turn {
        items: vec![
            Item::Call(Invocation {
                id: "bad".into(),
                name: "exact_read".into(),
                arguments: "{".into(),
            }),
            Item::Opaque(reasoning.clone()),
            Item::assistant("reading"),
            Item::Call(Invocation {
                id: "good".into(),
                name: "exact_read".into(),
                arguments: raw.into(),
            }),
        ],
        usage: Usage {
            input_tokens: Some(5),
            cached_input_tokens: Some(0),
            output_tokens: None,
        },
        receipt: json!({"mode":"synthetic"}),
    };
    let retained = serde_json::to_vec(&response).unwrap();
    let mut owner = TestProduct::default();
    let restored = restart_after_return(
        &mut owner,
        request(),
        &serde_json::from_slice(&retained).unwrap(),
    )
    .await
    .unwrap()
    .expect("successor request");
    assert_eq!(owner.effects, 1);
    assert!(
        matches!(&restored.items[0], Item::Call(call) if call.id == "bad" && call.arguments == "{")
    );
    assert_eq!(restored.items[1], Item::Opaque(reasoning));
    assert!(
        matches!(&restored.items[2], Item::Call(call) if call.id == "good" && call.arguments == raw)
    );
    let Item::MaterializedResult { call_id, output } = &restored.items[3] else {
        panic!("malformed sibling refusal")
    };
    assert_eq!(call_id, "bad");
    let denied: Value = serde_json::from_str(output).unwrap();
    assert_eq!(denied["executed"], false);
    assert!(
        matches!(&restored.items[4], Item::MaterializedResult { call_id, output } if call_id == "good" && output == "{\"exact\":true}")
    );
    assert!(
        !restored
            .items
            .iter()
            .any(|item| matches!(item, Item::Message { role, .. } if role == "assistant"))
    );
    assert!(owner.completed);
}

/// Reported usage reaches the real warning and reserved-final dispatches.
#[tokio::test]
async fn token_backstop_excludes_cached_input_and_survives_trace_restoration() {
    struct Metered {
        calls: usize,
    }
    impl Provider for Metered {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &Cancellation,
            _: &RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, engine::Cause> {
            self.calls += 1;
            let (input, cached, output) = match self.calls {
                1 => (10_000_000, 8_400_000, 0),
                2 => {
                    assert!(request.items.iter().any(|item| matches!(item,
                        Item::MaterializedResult { output, .. } if output.contains(engine::LIMIT_WARNING)
                    )), "80 percent of the token budget must produce its one warning");
                    assert!(
                        !request.items.iter().any(|item| matches!(item,
                            Item::Message { text, .. } if text == engine::FINAL_REQUEST
                        )),
                        "cached input must not exhaust the budget"
                    );
                    (500_000, 100_000, 0)
                }
                3 => {
                    assert_eq!(request.tools, self::request().tools);
                    assert!(
                        request.items.iter().any(|item| matches!(item,
                            Item::Message { text, .. } if text == engine::FINAL_REQUEST
                        )),
                        "the token backstop reserves a final request"
                    );
                    (0, 0, 0)
                }
                _ => panic!("the partial work cannot redispatch"),
            };
            let mut response = Turn {
                items: Vec::new(),
                usage: Usage {
                    input_tokens: Some(input),
                    output_tokens: Some(output),
                    cached_input_tokens: Some(cached),
                },
                receipt: json!({"mode":"synthetic"}),
            };
            response.items = if self.calls == 3 {
                vec![
                    Item::assistant("The retained work is partial."),
                    call("ignored-final-tool").into(),
                ]
            } else {
                vec![call(&format!("read-{}", self.calls)).into()]
            };
            Ok(response)
        }
    }
    let mut req = request();
    // Isolate cumulative budget from context pressure with synthetic reported usage.
    req.context_window_tokens = 20_000_000;
    req.max_input_tokens = 20_000_000;
    let mut product = TestProduct::default();
    let mut provider = Metered { calls: 0 };
    let limits = RunLimits::default();
    let partial = recover::work(req.clone().freeze().unwrap(), "first", &[], 0)
        .unwrap()
        .run(
            &mut provider,
            &mut product,
            limits,
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(partial.text(), "The retained work is partial.");
    assert_eq!(provider.calls, 3);
    assert_eq!(
        product.effects, 2,
        "the final request cannot execute its proposed tool"
    );
    assert!(!product.completed && product.partial);
    let progress = Progress::from_events(&product.records);
    assert_eq!(
        serde_json::to_value(&progress).unwrap()["budget_tokens"],
        2_000_000
    );
    assert_eq!(progress.known_tokens, 10_500_000);
    let warnings = product
        .records
        .iter()
        .filter(|event| {
            matches!(&event.kind,
                EventKind::ProviderStarted {context,..} if context.warning_added
            )
        })
        .count();
    assert_eq!(warnings, 1);
    let encoded = serde_json::to_vec(&product.records).unwrap();
    let saved: Vec<Event> = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(Progress::from_events(&saved), progress);
    let count = product.records.len();
    let error = recover::work(req.freeze().unwrap(), "reclaimed", &saved, 0)
        .unwrap()
        .run(
            &mut provider,
            &mut product,
            limits,
            &Cancellation::default(),
        )
        .await
        .unwrap_err();
    assert!(
        matches!(error, Failure::NotCurrent(engine::NotCurrent::Withdrawn)),
        "a held attempt is ended, so a reclaimed run stops: {error}"
    );
    assert_eq!(provider.calls, 3);
    assert_eq!(product.records.len(), count);
}
