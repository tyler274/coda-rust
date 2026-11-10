use harness::execution::*;
use harness::transport::{Cassette, Replay, ReplayQueue};
use serde_json::{Value, json};

fn request() -> Request {
    Request {
        model: "controlled".into(),
        options: json!({}),
        instructions: "original".into(),
        tools: vec![],
        items: vec![],
    }
}
fn response(calls: Vec<Call>) -> Response {
    Response {
        text: "answer".into(),
        calls,
        usage: Usage {
            input_tokens: Some(3),
            output_tokens: Some(2),
        },
        provenance: json!({"mode": "controlled"}),
    }
}
fn call(id: &str) -> Call {
    Call {
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
struct Product {
    records: Vec<Record>,
    effects: usize,
    completed: bool,
    stale: bool,
    withdraw_on_response: bool,
    fail_effect: bool,
}
impl Owner for Product {
    async fn check(&mut self) -> Result<(), String> {
        if self.stale {
            Err("stale".into())
        } else {
            Ok(())
        }
    }
    async fn record(&mut self, record: Record) -> Result<(), String> {
        if matches!(record, Record::Exchange { .. }) && self.withdraw_on_response {
            self.stale = true;
        }
        self.records.push(record);
        Ok(())
    }
    async fn dispatch(&mut self, _: &Call, _: &Cancellation) -> Result<Value, String> {
        if self.fail_effect {
            return Err("uncertain effect retained by owner".into());
        }
        self.effects += 1;
        Ok(json!({"exact": true}))
    }
    async fn complete(&mut self, _: &Response) -> Result<(), String> {
        self.check().await?;
        self.completed = true;
        Ok(())
    }
}
struct Controlled {
    replies: Vec<Response>,
    cancel: bool,
    fail: bool,
}
impl Provider for Controlled {
    async fn exchange(
        &mut self,
        _: &Request,
        token: &Cancellation,
        emit: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
    ) -> Result<Response, String> {
        if self.fail {
            return Err("provider unavailable".into());
        }
        let response = self.replies.remove(0);
        emit(StreamEvent::Text(response.text.clone()))?;
        if self.cancel {
            token.cancel();
        }
        Ok(response)
    }
}
#[tokio::test]
async fn recovery_rejects_retained_reused_context_identity_before_any_sibling_effect() {
    let mut req = request();
    req.items = vec![
        Item::Call(call("already-read")),
        Item::Result {
            call_id: "already-read".into(),
            value: json!({"retained": true}),
        },
    ];
    let invalid = response(vec![call("new-sibling"), call("already-read")]);
    let mut provider = Controlled {
        replies: vec![invalid.clone()],
        cancel: false,
        fail: false,
    };
    let mut owner = Product::default();
    assert!(
        run(
            &mut provider,
            &mut owner,
            req.clone(),
            &mut Progress::default(),
            Limits::default(),
            &Cancellation::default(),
            &mut |_| Ok(()),
        )
        .await
        .is_err()
    );
    assert!(owner.records.iter().any(|record| matches!(record,
        Record::Exchange { response, .. } if response == &invalid)));
    assert_eq!(owner.effects, 0);
    assert!(
        resume_tools(&mut owner, req, &invalid, &Cancellation::default())
            .await
            .is_err()
    );
    assert_eq!(
        owner.effects, 0,
        "recovery must preserve whole-batch live rejection"
    );
    assert!(!owner.completed);
}

#[tokio::test]
async fn structured_replay_preserves_original_instructions_and_tool_exchange() {
    let req = request();
    let first = response(vec![call("read-1")]);
    let mut next = req.clone();
    next.items = vec![
        Item::Message {
            role: "assistant".into(),
            text: "answer".into(),
        },
        Item::Call(call("read-1")),
        Item::Result {
            call_id: "read-1".into(),
            value: json!({"exact": true}),
        },
    ];
    let mut provider = Replay::new(Cassette {
        schema_version: 2,
        adapter: "controlled".into(),
        authorization: "test".into(),
        synthetic_non_phi: true,
        exchanges: vec![(req.clone(), first), (next, response(vec![]))],
    })
    .unwrap();
    let mut owner = Product::default();
    let mut progress = Progress::default();
    let mut events = vec![];
    run(
        &mut provider,
        &mut owner,
        req,
        &mut progress,
        Limits::default(),
        &Cancellation::default(),
        &mut |event| {
            events.push(event);
            Ok(())
        },
    )
    .await
    .unwrap();
    assert!(owner.completed);
    assert_eq!(owner.effects, 1);
    assert_eq!(progress.known_tokens, 10);
    assert_eq!(provider.remaining(), 0);
    assert_eq!(events.len(), 4);
}
#[tokio::test]
async fn late_cancelled_or_stale_output_retains_usage_without_effects() {
    for cancelled in [true, false] {
        let mut provider = Controlled {
            replies: vec![response(vec![call("1")])],
            cancel: cancelled,
            fail: false,
        };
        let mut owner = Product {
            withdraw_on_response: !cancelled,
            ..Default::default()
        };
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
        assert_eq!(progress.known_tokens, 5);
        assert_eq!(owner.effects, 0);
        assert!(!owner.completed);
        assert!(matches!(
            owner.records.last(),
            Some(Record::Exchange { .. })
        ));
    }
}
#[tokio::test]
async fn repeated_calls_are_rejected_before_any_effect_and_failures_retained() {
    let mut provider = Controlled {
        replies: vec![response(vec![call("1"), call("1")])],
        cancel: false,
        fail: false,
    };
    let mut owner = Product::default();
    assert!(
        run(
            &mut provider,
            &mut owner,
            request(),
            &mut Progress::default(),
            Limits::default(),
            &Cancellation::default(),
            &mut |_| Ok(())
        )
        .await
        .is_err()
    );
    assert_eq!(owner.effects, 0);
    provider.fail = true;
    assert!(
        run(
            &mut provider,
            &mut owner,
            request(),
            &mut Progress::default(),
            Limits::default(),
            &Cancellation::default(),
            &mut |_| Ok(())
        )
        .await
        .is_err()
    );
    assert!(matches!(owner.records.last(), Some(Record::Failure { .. })));
}
#[tokio::test]
async fn restored_call_budget_and_uncertain_effect_do_not_succeed() {
    let mut provider = Controlled {
        replies: vec![response(vec![call("1")])],
        cancel: false,
        fail: false,
    };
    let mut owner = Product {
        fail_effect: true,
        ..Default::default()
    };
    let mut progress = Progress {
        calls: 16,
        ..Default::default()
    };
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
    assert!(owner.records.is_empty());
    progress.calls = 0;
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
    assert!(!owner.completed);
    assert_eq!(progress.calls, 1);
}
#[tokio::test]
async fn recovery_reconciles_recorded_tool_batch_without_provider_redispatch() {
    struct Recovering {
        confirmed: std::collections::HashMap<String, Value>,
        effects: usize,
    }
    impl Owner for Recovering {
        async fn check(&mut self) -> Result<(), String> {
            Ok(())
        }
        async fn record(&mut self, _: Record) -> Result<(), String> {
            Ok(())
        }
        async fn dispatch(&mut self, call: &Call, _: &Cancellation) -> Result<Value, String> {
            if let Some(value) = self.confirmed.get(&call.id) {
                return Ok(value.clone());
            }
            self.effects += 1;
            let value = json!({"retained": call.id});
            self.confirmed.insert(call.id.clone(), value.clone());
            Ok(value)
        }
        async fn complete(&mut self, _: &Response) -> Result<(), String> {
            Ok(())
        }
    }
    let exchange = response(vec![call("first"), call("second")]);
    let mut owner = Recovering {
        confirmed: std::collections::HashMap::new(),
        effects: 0,
    };
    // Process stopped after the first confirmed effect, before its next call.
    owner
        .dispatch(&exchange.calls[0], &Cancellation::default())
        .await
        .unwrap();
    let resumed = resume_tools(&mut owner, request(), &exchange, &Cancellation::default())
        .await
        .unwrap();
    assert_eq!(owner.effects, 2);
    assert_eq!(
        resumed
            .items
            .iter()
            .filter(|i| matches!(i, Item::Result { .. }))
            .count(),
        2
    );
    resume_tools(&mut owner, request(), &exchange, &Cancellation::default())
        .await
        .unwrap();
    assert_eq!(
        owner.effects, 2,
        "recovery never duplicates confirmed effects"
    );
}

/// Declines the first `moves` terminal exchanges because its inputs moved,
/// then commits.
struct Moving {
    records: Vec<Record>,
    moves: usize,
    committed: usize,
}
impl Owner for Moving {
    fn terminal(&self, response: &Response) -> bool {
        response.calls.iter().any(|call| call.name == "finish")
    }
    async fn check(&mut self) -> Result<(), String> {
        Ok(())
    }
    async fn record(&mut self, record: Record) -> Result<(), String> {
        self.records.push(record);
        Ok(())
    }
    async fn dispatch(&mut self, _: &Call, _: &Cancellation) -> Result<Value, String> {
        Ok(json!({"read": true}))
    }
    async fn complete(&mut self, _: &Response) -> Result<(), String> {
        self.committed += 1;
        Ok(())
    }
    async fn finish(&mut self, response: &Response) -> Result<Finish, String> {
        if self.moves > 0 {
            self.moves -= 1;
            return Ok(Finish::Continue(json!({"outcome": "inputs_changed"})));
        }
        self.complete(response).await?;
        Ok(Finish::Committed)
    }
}
fn finish(id: &str) -> Call {
    Call {
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
    };
    let mut progress = Progress::default();
    run(
        &mut provider,
        &mut owner,
        request(),
        &mut progress,
        Limits::default(),
        &Cancellation::default(),
        &mut |_| Ok(()),
    )
    .await
    .unwrap();
    assert_eq!(owner.committed, 1);
    assert_eq!(progress.calls, 3);
    let dispatches: Vec<&Request> = owner
        .records
        .iter()
        .filter_map(|record| match record {
            Record::Dispatch { request, .. } => Some(request),
            _ => None,
        })
        .collect();
    assert_eq!(dispatches.len(), 3);
    // The second dispatch carries the declined finish and its product result.
    assert_eq!(
        dispatches[1].items,
        vec![
            Item::Message {
                role: "assistant".into(),
                text: "answer".into(),
            },
            Item::Call(finish("finish-1")),
            Item::Result {
                call_id: "finish-1".into(),
                value: json!({"outcome": "inputs_changed"}),
            },
        ]
    );
    assert_eq!(
        dispatches[1].items,
        continued(
            request(),
            &response(vec![finish("finish-1")]),
            &json!({"outcome": "inputs_changed"})
        )
        .items
    );
}

#[tokio::test]
async fn declined_finishes_stay_within_the_call_bound() {
    let mut provider = Controlled {
        replies: (0..4)
            .map(|n| response(vec![finish(&format!("finish-{n}"))]))
            .collect(),
        cancel: false,
        fail: false,
    };
    let mut owner = Moving {
        records: vec![],
        moves: usize::MAX,
        committed: 0,
    };
    let mut progress = Progress::default();
    let error = run(
        &mut provider,
        &mut owner,
        request(),
        &mut progress,
        Limits {
            max_calls: 3,
            ..Limits::default()
        },
        &Cancellation::default(),
        &mut |_| Ok(()),
    )
    .await
    .unwrap_err();
    assert_eq!(error, "execution usage bound reached");
    assert_eq!(owner.committed, 0);
    assert_eq!(progress.calls, 3);
}

// Frozen pre-unification JSON, deliberately not generated with the current Rust
// types: serde roundtrips alone would accept a mutually changed writer/reader.
const OLD_REQUEST: &str = r#"{"model":"controlled","options":{"reasoning":{"effort":"high"}},"instructions":"original","tools":[{"name":"exact_read","type":"function"}],"items":[{"kind":"message","role":"user","text":"hello"},{"kind":"call","id":"read-1","name":"exact_read","arguments":{"revision":7}},{"kind":"result","call_id":"read-1","value":{"exact":true}}]}"#;
const OLD_RESPONSE: &str = r#"{"text":"answer","calls":[],"usage":{"input_tokens":3,"output_tokens":null},"provenance":{"id":"provider-old","mode":"controlled"}}"#;

#[tokio::test]
async fn frozen_request_records_and_v2_cassette_remain_readable_and_replayable() {
    let request: Request = serde_json::from_str(OLD_REQUEST).unwrap();
    assert_eq!(serde_json::to_string(&request).unwrap(), OLD_REQUEST);
    assert_eq!(request.items[1], Item::Call(call("read-1")));
    let response: Response = serde_json::from_str(OLD_RESPONSE).unwrap();
    assert_eq!(serde_json::to_string(&response).unwrap(), OLD_RESPONSE);
    let records_json = format!(
        r#"[{{"Dispatch":{{"sequence":7,"request":{OLD_REQUEST}}}}},{{"Observation":{{"sequence":7,"usage":{{"input_tokens":2,"output_tokens":null}},"provenance":{{"id":"provider-old"}}}}}},{{"Exchange":{{"sequence":7,"response":{OLD_RESPONSE}}}}},{{"Dispatch":{{"sequence":8,"request":{OLD_REQUEST}}}}},{{"Failure":{{"sequence":8,"reason":"interrupted"}}}}]"#
    );
    let records: Vec<Record> = serde_json::from_str(&records_json).unwrap();
    assert_eq!(serde_json::to_string(&records).unwrap(), records_json);
    assert_eq!(
        Progress::from_records(&records),
        Progress {
            calls: 8,
            known_tokens: 3,
            unknown_usage: 2
        }
    );
    let cassette_json = format!(
        r#"{{"schema_version":2,"adapter":"controlled","authorization":"ENG-159 synthetic fixture","synthetic_non_phi":true,"exchanges":[[{OLD_REQUEST},{OLD_RESPONSE}]]}}"#
    );
    let cassette: Cassette = serde_json::from_str(&cassette_json).unwrap();
    assert_eq!(serde_json::to_string(&cassette).unwrap(), cassette_json);
    let mut replay = Replay::new(cassette).unwrap();
    let mut owner = Product::default();
    let mut progress = Progress::default();
    let returned = run(
        &mut replay,
        &mut owner,
        request,
        &mut progress,
        Limits::default(),
        &Cancellation::default(),
        &mut |_| Ok(()),
    )
    .await
    .unwrap();
    assert_eq!(returned.text, "answer");
    assert_eq!(returned.usage.output_tokens, None);
    assert_eq!(returned.provenance["recorded"], response.provenance);
    assert_eq!(returned.provenance["mode"], "replay");
    assert!(owner.completed);
    assert_eq!(owner.effects, 0);
    assert_eq!(
        progress,
        Progress {
            calls: 1,
            known_tokens: 3,
            unknown_usage: 1
        }
    );
    assert_eq!(replay.remaining(), 0);
}

#[tokio::test]
async fn mixed_terminal_batch_reaches_product_validation_before_any_effect() {
    struct AtomicTerminal {
        seen: Vec<Response>,
        effects: usize,
    }
    impl Owner for AtomicTerminal {
        fn terminal(&self, response: &Response) -> bool {
            response.calls.iter().any(|call| call.name == "finish")
        }
        async fn check(&mut self) -> Result<(), String> {
            Ok(())
        }
        async fn record(&mut self, _: Record) -> Result<(), String> {
            Ok(())
        }
        async fn dispatch(&mut self, _: &Call, _: &Cancellation) -> Result<Value, String> {
            self.effects += 1;
            Ok(Value::Null)
        }
        async fn complete(&mut self, response: &Response) -> Result<(), String> {
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
        let duplicate = exchange.calls[0].id == exchange.calls[1].id;
        let mut provider = Controlled {
            replies: vec![exchange.clone()],
            cancel: false,
            fail: false,
        };
        let mut owner = AtomicTerminal {
            seen: vec![],
            effects: 0,
        };
        let error = run(
            &mut provider,
            &mut owner,
            request(),
            &mut Progress::default(),
            Limits::default(),
            &Cancellation::default(),
            &mut |_| Ok(()),
        )
        .await
        .unwrap_err();
        assert_eq!(owner.effects, 0);
        if duplicate {
            assert_eq!(error, "invalid or repeated tool invocation identity");
            assert!(
                owner.seen.is_empty(),
                "structural validation precedes completion"
            );
        } else {
            assert_eq!(error, "product rejects mixed terminal effects");
            assert_eq!(owner.seen, vec![exchange.clone()]);
        }
        assert_eq!(
            resume_tools(&mut owner, request(), &exchange, &Cancellation::default())
                .await
                .unwrap_err(),
            "terminal exchange must use product completion recovery"
        );
        assert_eq!(
            owner.effects, 0,
            "recovery cannot execute terminal siblings either"
        );
    }
}

#[tokio::test]
async fn source_refresh_keeps_restored_ordinals_unknown_usage_and_both_budgets() {
    for token_bound in [false, true] {
        let mut provider = Controlled {
            replies: vec![
                response(vec![finish("refresh-1")]),
                response(vec![finish("refresh-2")]),
            ],
            cancel: false,
            fail: false,
        };
        let mut owner = Moving {
            records: vec![],
            moves: usize::MAX,
            committed: 0,
        };
        let mut progress = Progress {
            calls: 7,
            known_tokens: 11,
            unknown_usage: 2,
        };
        let error = run(
            &mut provider,
            &mut owner,
            request(),
            &mut progress,
            Limits {
                max_calls: if token_bound { 20 } else { 8 },
                max_known_tokens: if token_bound { 15 } else { 100 },
                ..Limits::default()
            },
            &Cancellation::default(),
            &mut |_| Ok(()),
        )
        .await
        .unwrap_err();
        assert_eq!(error, "execution usage bound reached");
        assert_eq!(
            progress,
            Progress {
                calls: 8,
                known_tokens: 16,
                unknown_usage: 2
            }
        );
        assert_eq!(
            provider.replies.len(),
            1,
            "continuation cannot reset either budget"
        );
        assert_eq!(owner.committed, 0);
        assert_eq!(owner.moves, usize::MAX - usize::from(!token_bound));
        assert!(matches!(
            owner.records.as_slice(),
            [
                Record::Dispatch { sequence: 8, .. },
                Record::Exchange { sequence: 8, .. }
            ]
        ));
    }
}

#[tokio::test]
async fn restored_source_refresh_replays_exact_continuation_before_committing() {
    let first = response(vec![finish("source-moved")]);
    let next = continued(request(), &first, &json!({"outcome":"inputs_changed"}));
    let mut final_response = response(vec![finish("fresh-finish")]);
    final_response.text = "Only the fresh answer may publish.".into();
    final_response.usage.output_tokens = None;
    let mut provider = Replay::new(Cassette {
        schema_version: 2,
        adapter: "controlled".into(),
        authorization: "ENG-159 synthetic source refresh".into(),
        synthetic_non_phi: true,
        exchanges: vec![(request(), first), (next, final_response)],
    })
    .unwrap();
    let mut owner = Moving {
        records: vec![],
        moves: 1,
        committed: 0,
    };
    let mut progress = Progress {
        calls: 7,
        known_tokens: 11,
        unknown_usage: 2,
    };
    let result = run(
        &mut provider,
        &mut owner,
        request(),
        &mut progress,
        Limits {
            max_calls: 9,
            max_known_tokens: 19,
            ..Limits::default()
        },
        &Cancellation::default(),
        &mut |_| Ok(()),
    )
    .await
    .unwrap();
    assert_eq!(result.text, "Only the fresh answer may publish.");
    assert_eq!(owner.committed, 1);
    assert_eq!(provider.remaining(), 0);
    assert_eq!(
        progress,
        Progress {
            calls: 9,
            known_tokens: 19,
            unknown_usage: 3
        }
    );
    let ordinals: Vec<_> = owner
        .records
        .iter()
        .filter_map(|record| match record {
            Record::Dispatch { sequence, .. } => Some(*sequence),
            _ => None,
        })
        .collect();
    assert_eq!(ordinals, [8, 9]);
}

#[test]
fn structured_wire_roundtrip_preserves_tool_ids_usage_and_incomplete_failure() {
    let body = json!({"status": "completed", "id": "provider-1", "output": [
        {"type": "message", "content": [{"type": "output_text", "text": "reading"}]},
        {"type": "function_call", "call_id": "read-1", "name": "exact_read", "arguments": "{\"revision\":7}"}
    ], "usage": {"input_tokens": 11, "output_tokens": 5}});
    let result = harness::provider::decode(&body).unwrap();
    assert_eq!(result.calls, vec![call("read-1")]);
    assert_eq!(result.usage.input_tokens, Some(11));
    let mut req = request();
    req.items = vec![
        Item::Call(result.calls[0].clone()),
        Item::Result {
            call_id: "read-1".into(),
            value: json!({"value": 4}),
        },
    ];
    let wire = harness::provider::encode(&req);
    assert_eq!(wire["input"][0]["call_id"], "read-1");
    assert_eq!(wire["input"][1]["type"], "function_call_output");
    assert_eq!(wire["instructions"], "original");
    let mut incomplete = body;
    incomplete["status"] = json!("incomplete");
    assert!(harness::provider::decode(&incomplete).is_err());
}

#[test]
fn replay_identity_reconciles_same_bytes_and_provenance_is_required() {
    let mut queue = ReplayQueue::new(vec![("key".into(), 1), ("key".into(), 2)]);
    assert_eq!(queue.take("call", "key").unwrap(), 1);
    assert_eq!(queue.take("call", "key").unwrap(), 1);
    assert!(queue.take("call", "different").is_err());
    assert_eq!(queue.take("next", "key").unwrap(), 2);
    assert!(
        Replay::new(Cassette {
            schema_version: 2,
            adapter: "a".into(),
            authorization: "".into(),
            synthetic_non_phi: true,
            exchanges: vec![]
        })
        .is_err()
    );
}
