use std::{collections::VecDeque, time::Duration};

use async_trait::async_trait;
use harness::engine::*;
use serde_json::{Value, json};

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
fn call(id: &str, name: &str, arguments: &str) -> Item {
    Item::Call(Invocation {
        id: id.into(),
        name: name.into(),
        arguments: arguments.into(),
    })
}
fn text(value: &str) -> Item {
    Item::Message {
        role: "assistant".into(),
        text: value.into(),
    }
}
fn turn(items: Vec<Item>) -> Turn {
    Turn {
        items,
        receipt: json!({"usage": 1}),
    }
}

struct Script {
    turns: VecDeque<Turn>,
    requests: Vec<Context>,
    fail: bool,
}
#[async_trait]
impl Provider for Script {
    fn request_bytes(&self, _: &Request<'_>) -> Result<usize, String> {
        Ok(10)
    }
    async fn complete(&mut self, request: Request<'_>) -> Result<Turn, String> {
        self.requests.push(request.context.clone());
        if self.fail {
            return Err("transport unknown".into());
        }
        Ok(self.turns.pop_front().expect("unexpected provider retry"))
    }
}
impl Script {
    fn new(turns: Vec<Turn>) -> Self {
        Self {
            turns: turns.into(),
            requests: vec![],
            fail: false,
        }
    }
}

#[derive(Default)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "each bool injects one independent fault into the fake"
)]
struct Owner {
    events: Vec<Event>,
    executed: usize,
    fail_event: Option<&'static str>,
    pending_event: bool,
    deny: bool,
    revoke_after_start: bool,
    revoked: bool,
    uncertain: bool,
    cancel_on_tool: Option<Cancellation>,
}
fn event_name(kind: &EventKind) -> &'static str {
    match kind {
        EventKind::ProviderStarted { .. } => "provider_start",
        EventKind::ProviderReturned(_) => "provider_return",
        EventKind::TextDelta { .. } => "delta",
        EventKind::ToolStarted(_) => "tool_start",
        EventKind::ToolReturned { .. } => "tool_return",
        EventKind::Candidate { .. } => "candidate",
    }
}
#[async_trait]
impl Host for Owner {
    async fn prepare(&mut self, transcript: &[Item], _: &Usage) -> Result<Context, String> {
        Ok(Context {
            instructions: "owner instructions".into(),
            items: transcript.to_vec(),
            tools: vec![Tool {
                name: "save".into(),
                description: "owner action".into(),
                parameters: json!({"type":"object"}),
            }],
        })
    }
    async fn assert_current(&mut self) -> Result<(), String> {
        if self.revoked {
            Err("revoked".into())
        } else {
            Ok(())
        }
    }
    async fn persist(&mut self, event: Event) -> Result<(), String> {
        if self.pending_event {
            return std::future::pending().await;
        }
        if self.fail_event == Some(event_name(&event.kind)) {
            return Err("not acknowledged".into());
        }
        if self.revoke_after_start && matches!(event.kind, EventKind::ToolStarted(_)) {
            self.revoked = true;
        }
        self.events.push(event);
        Ok(())
    }
    async fn admit(&mut self, _: &Invocation, _: &Value) -> Result<Admission, String> {
        Ok(if self.deny {
            Admission::Deny("not authorized".into())
        } else {
            Admission::Allow
        })
    }
    async fn execute(&mut self, _: &Invocation, _: Value) -> Result<String, String> {
        self.executed += 1;
        if let Some(cancel) = &self.cancel_on_tool {
            cancel.cancel();
            return std::future::pending().await;
        }
        if self.uncertain {
            return Err("effect may have committed".into());
        }
        Ok("saved".into())
    }
}

#[tokio::test]
async fn acknowledged_loop_pairs_results_and_returns_only_candidate() {
    let mut provider = Script::new(vec![
        turn(vec![text("First. "), call("c", "save", "{}")]),
        turn(vec![text("Done.")]),
    ]);
    let mut owner = Owner::default();
    let result = run(
        &mut provider,
        &mut owner,
        "attempt",
        vec![],
        &limits(),
        &Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(result.text, "First. Done.");
    assert_eq!(owner.executed, 1);
    assert_eq!(provider.requests.len(), 2);
    assert!(provider.requests[1].items.iter().any(|item| matches!(item, Item::Result { call_id, output } if call_id == "c" && output == "saved")));
    assert_eq!(
        owner
            .events
            .iter()
            .map(|event| event_name(&event.kind))
            .collect::<Vec<_>>(),
        [
            "provider_start",
            "provider_return",
            "tool_start",
            "tool_return",
            "provider_start",
            "provider_return",
            "candidate"
        ]
    );
    assert!(
        owner
            .events
            .iter()
            .enumerate()
            .all(|(i, event)| event.sequence == i as u64 + 1)
    );
}

#[tokio::test]
async fn failed_acknowledgments_prevent_dependent_dispatch() {
    for (boundary, providers, tools) in [
        ("provider_start", 0, 0),
        ("provider_return", 1, 0),
        ("tool_start", 1, 0),
        ("tool_return", 1, 1),
    ] {
        let mut provider = Script::new(vec![turn(vec![call("c", "save", "{}")])]);
        let mut owner = Owner {
            fail_event: Some(boundary),
            ..Owner::default()
        };
        let error = run(
            &mut provider,
            &mut owner,
            "attempt",
            vec![],
            &limits(),
            &Cancellation::default(),
        )
        .await
        .unwrap_err();
        assert!(matches!(error.failure, Failure::Host(_)));
        assert_eq!(provider.requests.len(), providers);
        assert_eq!(owner.executed, tools);
    }
}

#[tokio::test]
async fn pending_acknowledgment_obeys_deadline_without_dispatch() {
    let mut provider = Script::new(vec![]);
    let mut owner = Owner {
        pending_event: true,
        ..Owner::default()
    };
    let mut bounds = limits();
    bounds.elapsed = Duration::from_millis(10);
    let error = run(
        &mut provider,
        &mut owner,
        "attempt",
        vec![],
        &bounds,
        &Cancellation::default(),
    )
    .await
    .unwrap_err();
    assert_eq!(error.failure, Failure::Deadline);
    assert!(provider.requests.is_empty());
}

#[tokio::test]
async fn duplicate_calls_and_oversized_batches_execute_nothing() {
    for items in [
        vec![call("same", "save", "{}"), call("same", "save", "{}")],
        (0..4).map(|n| call(&n.to_string(), "save", "{}")).collect(),
    ] {
        let mut provider = Script::new(vec![turn(items)]);
        let mut owner = Owner::default();
        assert!(
            run(
                &mut provider,
                &mut owner,
                "attempt",
                vec![],
                &limits(),
                &Cancellation::default()
            )
            .await
            .is_err()
        );
        assert_eq!(owner.executed, 0);
    }
}

#[tokio::test]
async fn malformed_and_unknown_siblings_preserve_successful_result_and_order() {
    let calls = vec![
        call("malformed", "save", "{broken"),
        call("good", "save", " { \"revision\" : 7 } "),
        call("unknown", "not_registered", "{}"),
    ];
    let mut provider = Script::new(vec![turn(calls.clone()), turn(vec![text("Done.")])]);
    let mut owner = Owner::default();
    let result = run(
        &mut provider,
        &mut owner,
        "siblings",
        vec![],
        &limits(),
        &Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(result.text, "Done.");
    assert_eq!(owner.executed, 1);
    assert_eq!(&provider.requests[1].items[..3], calls.as_slice());
    let results: Vec<_> = provider.requests[1]
        .items
        .iter()
        .filter_map(|item| {
            if let Item::Result { call_id, output } = item {
                Some((call_id.as_str(), output.as_str()))
            } else {
                None
            }
        })
        .collect();
    assert_eq!(results.len(), 3);
    assert_eq!(results[0].0, "malformed");
    assert_eq!(results[1], ("good", "saved"));
    assert_eq!(results[2].0, "unknown");
    assert!(results[0].1.contains("error"));
    assert!(results[2].1.contains("error"));
    let returned: Vec<_> = owner
        .events
        .iter()
        .filter_map(|event| {
            if let EventKind::ToolReturned {
                call_id, output, ..
            } = &event.kind
            {
                Some((call_id.as_str(), output.as_str()))
            } else {
                None
            }
        })
        .collect();
    assert_eq!(
        returned, results,
        "all feedback is acknowledged before redispatch"
    );
}

#[tokio::test]
async fn malformed_unknown_and_denied_tools_return_feedback() {
    for (invocation, deny) in [
        (call("c", "save", "bad json"), false),
        (call("c", "unknown", "{}"), false),
        (call("c", "save", "{}"), true),
    ] {
        let mut provider = Script::new(vec![
            turn(vec![invocation]),
            turn(vec![text("Cannot do that.")]),
        ]);
        let mut owner = Owner {
            deny,
            ..Owner::default()
        };
        let result = run(
            &mut provider,
            &mut owner,
            "attempt",
            vec![],
            &limits(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
        assert_eq!(owner.executed, 0);
        assert!(
            result.transcript.iter().any(
                |item| matches!(item, Item::Result { output, .. } if output.contains("false"))
            )
        );
    }
}

#[tokio::test]
async fn currentness_rechecked_after_durable_intent() {
    let mut provider = Script::new(vec![turn(vec![call("c", "save", "{}")])]);
    let mut owner = Owner {
        revoke_after_start: true,
        ..Owner::default()
    };
    assert!(
        run(
            &mut provider,
            &mut owner,
            "attempt",
            vec![],
            &limits(),
            &Cancellation::default()
        )
        .await
        .is_err()
    );
    assert_eq!(owner.executed, 0);
}

#[tokio::test]
async fn uncertain_effect_is_never_replayed() {
    for cancelled in [false, true] {
        let cancel = Cancellation::default();
        let mut provider = Script::new(vec![turn(vec![call("c", "save", "{}")])]);
        let mut owner = Owner {
            uncertain: !cancelled,
            cancel_on_tool: cancelled.then(|| cancel.clone()),
            ..Owner::default()
        };
        let error = run(
            &mut provider,
            &mut owner,
            "attempt",
            vec![],
            &limits(),
            &cancel,
        )
        .await
        .unwrap_err();
        assert_eq!(owner.executed, 1);
        assert_eq!(error.usage.tool_calls, 1);
        assert_eq!(provider.requests.len(), 1);
        assert_eq!(error.boundary, "tool dispatch; reconcile uncertain effects");
        assert!(
            !owner
                .events
                .iter()
                .any(|e| matches!(e.kind, EventKind::Candidate { .. }))
        );
    }
}

#[tokio::test]
async fn cancelled_before_start_and_provider_failure_never_retry() {
    let cancel = Cancellation::default();
    cancel.cancel();
    let mut provider = Script::new(vec![]);
    let mut owner = Owner::default();
    assert_eq!(
        run(
            &mut provider,
            &mut owner,
            "attempt",
            vec![],
            &limits(),
            &cancel
        )
        .await
        .unwrap_err()
        .failure,
        Failure::Cancelled
    );
    assert!(provider.requests.is_empty());
    provider.fail = true;
    let error = run(
        &mut provider,
        &mut owner,
        "attempt",
        vec![],
        &limits(),
        &Cancellation::default(),
    )
    .await
    .unwrap_err();
    assert!(matches!(error.failure, Failure::Provider(_)));
    assert_eq!(error.usage.model_calls, 1);
}

#[tokio::test]
async fn limits_preserve_acknowledged_effect_without_reexecution() {
    let mut bounds = limits();
    bounds.tool_result_bytes = 1;
    let mut provider = Script::new(vec![turn(vec![call("c", "save", "{}")])]);
    let mut owner = Owner::default();
    let error = run(
        &mut provider,
        &mut owner,
        "attempt",
        vec![],
        &bounds,
        &Cancellation::default(),
    )
    .await
    .unwrap_err();
    assert!(matches!(error.failure, Failure::Limit(_)));
    assert_eq!(owner.executed, 1);
    assert!(owner.events.iter().any(
        |event| matches!(&event.kind, EventKind::ToolReturned { output, .. } if output == "saved")
    ));
    assert_eq!(provider.requests.len(), 1);
}

#[tokio::test]
async fn model_request_and_tool_budgets_are_enforced() {
    let mut bounds = limits();
    bounds.request_bytes = 1;
    let mut provider = Script::new(vec![]);
    let mut owner = Owner::default();
    assert!(
        run(
            &mut provider,
            &mut owner,
            "a",
            vec![],
            &bounds,
            &Cancellation::default()
        )
        .await
        .is_err()
    );
    assert!(provider.requests.is_empty());

    bounds = limits();
    bounds.tool_calls = 0;
    provider = Script::new(vec![
        turn(vec![call("c", "save", "{}")]),
        turn(vec![text("No action.")]),
    ]);
    assert!(
        run(
            &mut provider,
            &mut owner,
            "b",
            vec![],
            &bounds,
            &Cancellation::default()
        )
        .await
        .is_ok()
    );
    assert_eq!(owner.executed, 0);

    bounds = limits();
    bounds.model_calls = 1;
    provider = Script::new(vec![turn(vec![call("d", "save", "{}")])]);
    let error = run(
        &mut provider,
        &mut owner,
        "c",
        vec![],
        &bounds,
        &Cancellation::default(),
    )
    .await
    .unwrap_err();
    assert!(matches!(error.failure, Failure::Limit(_)));
    assert_eq!(provider.requests.len(), 1);
}

#[tokio::test]
async fn rejected_candidate_ack_never_returns_completion() {
    let mut provider = Script::new(vec![turn(vec![text("done")])]);
    let mut owner = Owner {
        fail_event: Some("candidate"),
        ..Owner::default()
    };
    assert!(
        run(
            &mut provider,
            &mut owner,
            "a",
            vec![],
            &limits(),
            &Cancellation::default()
        )
        .await
        .is_err()
    );
}

#[tokio::test]
async fn repeated_identity_across_turns_never_reexecutes() {
    let mut provider = Script::new(vec![
        turn(vec![call("c", "save", "{}")]),
        turn(vec![call("c", "save", "{}")]),
    ]);
    let mut owner = Owner::default();
    assert!(
        run(
            &mut provider,
            &mut owner,
            "a",
            vec![],
            &limits(),
            &Cancellation::default()
        )
        .await
        .is_err()
    );
    assert_eq!(owner.executed, 1);
}

#[test]
fn context_requires_complete_unique_pairs() {
    assert!(validate_context(&[call("c", "save", "{}")]).is_err());
    assert!(
        validate_context(&[Item::Result {
            call_id: "orphan".into(),
            output: "x".into()
        }])
        .is_err()
    );
    assert!(
        validate_context(&[
            call("c", "save", "{}"),
            Item::Result {
                call_id: "c".into(),
                output: "ok".into()
            }
        ])
        .is_ok()
    );
}
