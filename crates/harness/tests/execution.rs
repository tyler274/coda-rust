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

#[test]
fn structured_wire_roundtrip_preserves_tool_ids_usage_and_incomplete_failure() {
    let body = json!({"status": "completed", "id": "provider-1", "output": [
        {"type": "message", "content": [{"type": "output_text", "text": "reading"}]},
        {"type": "function_call", "call_id": "read-1", "name": "exact_read", "arguments": "{\"revision\":7}"}
    ], "usage": {"input_tokens": 11, "output_tokens": 5}});
    let result = harness::responses::decode(&body).unwrap();
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
    let wire = harness::responses::encode(&req);
    assert_eq!(wire["input"][0]["call_id"], "read-1");
    assert_eq!(wire["input"][1]["type"], "function_call_output");
    assert_eq!(wire["instructions"], "original");
    let mut incomplete = body;
    incomplete["status"] = json!("incomplete");
    assert!(harness::responses::decode(&incomplete).is_err());
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
