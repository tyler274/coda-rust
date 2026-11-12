//! Scheduler contracts through recovered work, with delayed tools and retained events.
#[path = "support/recover.rs"]
mod recover;
use async_trait::async_trait;
use harness::engine::{recipe::*, *};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::Notify;

fn call(id: &str, name: &str, delay: u64) -> Item {
    Item::Call(Invocation {
        id: id.into(),
        name: name.into(),
        arguments: json!({"delay":delay}).to_string(),
    })
}
fn request() -> Request {
    Request {
        cache_key: None,
        model: "stub".into(),
        context_window_tokens: 100_000,
        max_input_tokens: 100_000,
        options: json!({}),
        instructions: "Test tool scheduling".into(),
        items: vec![],
        tools: ["read", "write", "fail"]
            .into_iter()
            .map(|name| {
                Tool {
                    name: name.into(),
                    description: name.into(),
                    parameters: json!({"type":"object"}),
                }
                .function()
            })
            .collect(),
    }
}
fn turn(items: Vec<Item>) -> Turn {
    Turn {
        items,
        receipt: json!({}),
        usage: Usage::default(),
    }
}
#[derive(Default)]
struct Script {
    next: Option<Vec<Item>>,
    requests: Vec<Request>,
}
impl Provider for Script {
    async fn exchange(
        &mut self,
        request: &Request,
        _: &Cancellation,
        _: &RunLimits,
        _: &mut dyn Observer,
    ) -> Result<Turn, Cause> {
        self.requests.push(request.clone());
        Ok(turn(
            self.next
                .take()
                .unwrap_or_else(|| vec![Item::assistant("Done")]),
        ))
    }
}
#[derive(Default)]
struct Saved {
    events: Vec<Event>,
    calls: BTreeMap<String, usize>,
    active: usize,
    high_water: usize,
    writes: usize,
    order: Vec<String>,
    /// Set when the attempt stops being current.
    currency: Option<Loss>,
    /// Saving this call's result fails, not retryably.
    unsaved_result: Option<&'static str>,
}
/// Why the attempt stopped being current, as the owner's check reports it.
#[derive(Clone, Copy)]
enum Loss {
    /// New input replaced the direction.
    Superseded,
    /// The work no longer wants this attempt.
    Withdrawn,
}
#[derive(Default)]
struct Admission {
    committed: Notify,
    acknowledge: Notify,
}
#[derive(Default)]
struct Owner {
    saved: Arc<Mutex<Saved>>,
    returned: Arc<Notify>,
    transaction: Option<Arc<tokio::sync::Mutex<()>>>,
    admission: Option<Arc<Admission>>,
    search: Option<SearchOrder>,
    searched: Arc<Notify>,
}
/// Which finishes first: a history search or the sibling read in its batch.
#[derive(Clone, Copy)]
enum SearchOrder {
    /// The search reads the record after the sibling's result is saved.
    Late,
    /// The search answers before the sibling's result is saved.
    Early,
}
#[async_trait]
impl Product for Owner {
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
    async fn window_seed(&mut self) -> Result<harness::engine::WindowSeed, harness::engine::Cause> {
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

    fn read_only(&self, name: &str) -> bool {
        matches!(name, "read" | "fail")
    }
    async fn assert_current(&mut self) -> Result<(), Cause> {
        let mut saved = self.saved.lock().unwrap();
        saved.order.push("authority".into());
        match saved.currency {
            None => Ok(()),
            Some(Loss::Superseded) => Err(Box::new(Superseded)),
            Some(Loss::Withdrawn) => Err(Box::new(Withdrawn)),
        }
    }
    async fn persist(&mut self, event: Event) -> Result<(), Cause> {
        let _write = if matches!(event.kind, EventKind::ToolReturned { .. }) {
            match &self.transaction {
                Some(lock) => Some(lock.lock().await),
                None => None,
            }
        } else {
            None
        };
        let block = matches!(&event.kind, EventKind::ToolStarted(call) if call.id == "pending");
        {
            let mut saved = self.saved.lock().unwrap();
            if saved.events.iter().any(|saved| {
                saved.attempt_id == event.attempt_id && saved.sequence == event.sequence
            }) {
                return Err("execution event collision".into());
            }
            if let EventKind::ToolStarted(call) = &event.kind {
                saved.order.push(format!("admit:{}", call.id));
            }
            if let EventKind::ToolReturned { call_id, .. } = &event.kind
                && saved.unsaved_result == Some(call_id.as_str())
            {
                return Err("result not acknowledged".into());
            }
            if let EventKind::ToolReturned { call_id, .. } = &event.kind {
                saved.order.push(format!("returned:{call_id}"));
                self.returned.notify_one();
            }
            saved.events.push(event);
        }
        if block && let Some(admission) = &self.admission {
            admission.committed.notify_one();
            admission.acknowledge.notified().await;
        }
        Ok(())
    }
    fn execute(&mut self, call: &Invocation, args: Value) -> ToolFuture {
        let saved = self.saved.clone();
        let call = call.clone();
        let transaction = self.transaction.clone();
        let search = self.search;
        let searched = self.searched.clone();
        if call.name == history::SEARCH {
            return Box::pin(async move {
                if let Some(SearchOrder::Late) = search {
                    while !saved.lock().unwrap().events.iter().any(|event| {
                        matches!(&event.kind, EventKind::ToolReturned { call_id, .. } if call_id == "needle-sibling")
                    }) {
                        tokio::time::sleep(Duration::from_millis(1)).await;
                    }
                }
                let events = saved.lock().unwrap().events.clone();
                let page = history::execute::<Value>(
                    &call.name,
                    &args,
                    &[],
                    events.iter().map(ReplayEvent::Admitted),
                )?;
                searched.notify_one();
                Ok(ToolOutcome::History(page))
            });
        }
        if let (Some(SearchOrder::Early), "needle-sibling") = (search, call.id.as_str()) {
            return Box::pin(async move {
                searched.notified().await;
                Ok(ToolOutcome::Returned(json!({"id":call.id}).to_string()))
            });
        }
        if args["hang"] == true {
            return Box::pin(async move {
                saved
                    .lock()
                    .unwrap()
                    .order
                    .push(format!("start:{}", call.id));
                std::future::pending().await
            });
        }
        Box::pin(async move {
            let _read = match &transaction {
                Some(lock) => Some(lock.lock().await),
                None => None,
            };
            {
                let mut state = saved.lock().unwrap();
                *state.calls.entry(call.id.clone()).or_default() += 1;
                state.active += 1;
                state.high_water = state.high_water.max(state.active);
                state.order.push(format!("start:{}", call.id));
                if call.name == "write" {
                    assert_eq!(state.active, 1, "write crossed a barrier");
                    state.writes += 1;
                }
            }
            tokio::time::sleep(Duration::from_millis(args["delay"].as_u64().unwrap())).await;
            let mut state = saved.lock().unwrap();
            state.active -= 1;
            state.order.push(format!("end:{}", call.id));
            if call.name == "fail" {
                return Err("stub read failure".into());
            }
            Ok(ToolOutcome::Returned(
                json!({"id":call.id,"writes":state.writes}).to_string(),
            ))
        })
    }
    async fn finish(&mut self, _: &Turn) -> Result<Finish, Cause> {
        Ok(Finish::Committed)
    }
}
fn results(request: &Request) -> Vec<&str> {
    request
        .items
        .iter()
        .filter_map(|item| match item {
            Item::MaterializedResult { call_id, .. } => Some(call_id.as_str()),
            _ => None,
        })
        .collect()
}

#[tokio::test]
async fn eight_reads_overlap_and_a_write_splits_the_batch_without_reordering_results() {
    let mut owner = Owner::default();
    let mut calls = vec![
        call("slow", "read", 150),
        call("fast", "read", 60),
        call("middle", "read", 100),
    ];
    calls.extend((0..18).map(|n| call(&format!("queued-{n}"), "read", 20)));
    calls.extend([call("write", "write", 5), call("after", "read", 5)]);
    let expected: Vec<String> = calls
        .iter()
        .map(|item| match item {
            Item::Call(call) => call.id.clone(),
            _ => unreachable!(),
        })
        .collect();
    let mut provider = Script {
        next: Some(calls),
        ..Script::default()
    };
    recover::work(request().freeze().unwrap(), "first", &[], 0)
        .unwrap()
        .run(
            &mut provider,
            &mut owner,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    let state = owner.saved.lock().unwrap();
    assert_eq!(
        state.high_water, 8,
        "at most eight reads may be active in one batch"
    );
    assert_eq!(
        results(&provider.requests[1]),
        expected.iter().map(String::as_str).collect::<Vec<_>>()
    );
    let index = |needle: &str| {
        state
            .order
            .iter()
            .position(|entry| entry == needle)
            .unwrap()
    };
    for id in expected.iter().take(21) {
        assert!(index(&format!("end:{id}")) < index("admit:write"));
    }
    assert!(
        index("start:queued-5") < index("end:slow"),
        "a free read slot is filled without waiting for the slowest sibling"
    );
    assert!(index("end:write") < index("admit:after"));
    assert!(index("returned:fast") < index("end:slow"));
    for id in &expected {
        assert!(index(&format!("admit:{id}")) < index(&format!("start:{id}")));
        assert_eq!(state.order[index(&format!("admit:{id}")) + 1], "authority");
    }
}

#[tokio::test]
async fn a_failed_read_gives_the_model_an_error_result_and_its_siblings_keep_theirs() {
    let mut owner = Owner::default();
    let mut calls = vec![
        call("slow", "read", 60),
        call("failed", "fail", 10),
        call("fast", "read", 20),
    ];
    calls.extend((0..10).map(|n| call(&format!("later-{n}"), "read", 5)));
    let expected: Vec<String> = calls
        .iter()
        .map(|item| match item {
            Item::Call(call) => call.id.clone(),
            _ => unreachable!(),
        })
        .collect();
    let mut provider = Script {
        next: Some(calls),
        ..Script::default()
    };
    let answer = recover::work(request().freeze().unwrap(), "first", &[], 0)
        .unwrap()
        .run(
            &mut provider,
            &mut owner,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .expect("a tool's own failure does not end the run");
    assert_eq!(answer.text(), "Done");
    assert_eq!(provider.requests.len(), 2);
    assert_eq!(
        results(&provider.requests[1]),
        expected.iter().map(String::as_str).collect::<Vec<_>>(),
        "every call keeps its result, in call order"
    );
    let failed = provider.requests[1]
        .items
        .iter()
        .find_map(|item| match item {
            Item::MaterializedResult { call_id, output } if call_id == "failed" => Some(output),
            _ => None,
        })
        .unwrap();
    let failed: Value = serde_json::from_str(failed).unwrap();
    assert_eq!(failed["error"], "This call failed. You can try it again.");
    assert!(
        !failed.to_string().contains("stub read failure"),
        "the error itself never reaches the model"
    );
    let state = owner.saved.lock().unwrap();
    assert_eq!(state.calls["failed"], 1);
}

#[tokio::test]
async fn restart_keeps_a_later_read_that_finished_before_the_first() {
    for allowance_after in [0, 1] {
        let saved = Arc::new(Mutex::new(Saved::default()));
        let returned = Arc::new(Notify::new());
        let mut owner = Owner {
            saved: saved.clone(),
            returned: returned.clone(),
            transaction: None,
            admission: None,
            ..Owner::default()
        };
        let mut provider = Script {
            next: Some(vec![
                call("first", "read", 200),
                call("later", "read", 5),
                call("pending-write", "write", 0),
            ]),
            ..Script::default()
        };
        let cancel = Cancellation::default();
        {
            let run = recover::work(request().freeze().unwrap(), "before-crash", &[], 0)
                .unwrap()
                .run(&mut provider, &mut owner, RunLimits::default(), &cancel);
            tokio::pin!(run);
            tokio::select! { biased; _=returned.notified()=>(), result=&mut run=>panic!("run ended before crash: {result:?}") }
            // Drop the process-local run while the first read remains unfinished.
        }
        drop(owner);
        let events = {
            let mut state = saved.lock().unwrap();
            state.active = 0;
            state.events.clone()
        };
        assert!(events.iter().any(
            |event| matches!(&event.kind,EventKind::ToolReturned {call_id,..} if call_id=="later")
        ));
        assert!(!events.iter().any(
            |event| matches!(&event.kind,EventKind::ToolReturned {call_id,..} if call_id=="first")
        ));
        let mut successor = Owner {
            saved: saved.clone(),
            returned,
            transaction: None,
            admission: None,
            ..Owner::default()
        };
        let mut next = Script::default();
        request()
            .freeze()
            .unwrap()
            .recover(
                "successor",
                recovery::Records {
                    events: recover::admitted(&events).collect(),
                    allowance_after,
                },
                recovery::Messages {
                    sources: &[],
                    partial: None,
                },
                recovery::Inputs::default(),
            )
            .unwrap()
            .run(
                &mut next,
                &mut successor,
                RunLimits::default(),
                &Cancellation::default(),
            )
            .await
            .unwrap();
        let state = saved.lock().unwrap();
        assert_eq!(
            state.calls["later"], 1,
            "the acknowledged read must never execute again"
        );
        assert_eq!(
            state.calls["first"], 1,
            "a started call with no saved outcome must never execute again"
        );
        assert_eq!(
            results(&next.requests[0]),
            ["first", "later", "pending-write"]
        );
        assert_eq!(
            state.calls["pending-write"], 1,
            "unstarted calls continue on both recovery and explicit retry"
        );
        assert!(next.requests[0].items.iter().any(|item| {
        matches!(item, Item::MaterializedResult { call_id, output }
            if call_id == "first" && serde_json::from_str::<Value>(output).unwrap()["outcome"] == "unknown")
    }), "the model must see the started call's unknown outcome");
        for id in ["first", "later"] {
            assert_eq!(
                state
                    .events
                    .iter()
                    .filter(|event| {
                        matches!(&event.kind, EventKind::ToolStarted(call) if call.id == id)
                    })
                    .count(),
                1,
                "recovery must not admit a saved call again"
            );
        }
    }
}

#[tokio::test]
async fn reads_release_product_locks_while_receipts_are_being_saved() {
    let mut owner = Owner {
        transaction: Some(Arc::default()),
        ..Owner::default()
    };
    let mut provider = Script {
        next: Some(vec![call("first", "read", 5), call("second", "read", 10)]),
        ..Script::default()
    };
    tokio::time::timeout(
        Duration::from_millis(500),
        recover::work(request().freeze().unwrap(), "locks", &[], 0)
            .unwrap()
            .run(
                &mut provider,
                &mut owner,
                RunLimits::default(),
                &Cancellation::default(),
            ),
    )
    .await
    .expect("independent read must release its transaction while the event writer waits")
    .unwrap();
    assert_eq!(results(&provider.requests[1]), ["first", "second"]);
}

#[tokio::test]
async fn cancelling_committed_admission_keeps_its_record_and_runs_nothing_more() {
    let admission = Arc::new(Admission::default());
    let mut owner = Owner {
        admission: Some(admission.clone()),
        ..Owner::default()
    };
    let cancel = Cancellation::default();
    let mut provider = Script {
        next: Some(vec![call("running", "read", 5), call("pending", "read", 0)]),
        ..Script::default()
    };
    let run = recover::work(request().freeze().unwrap(), "cancel-admission", &[], 0)
        .unwrap()
        .run(&mut provider, &mut owner, RunLimits::default(), &cancel);
    let interrupt = async {
        admission.committed.notified().await;
        cancel.cancel();
        admission.acknowledge.notify_one();
    };
    let (result, ()) = tokio::join!(run, interrupt);
    let saved = owner.saved.lock().unwrap();
    assert!(
        saved.events.iter().any(|event| {
            matches!(&event.kind, EventKind::ToolStarted(call) if call.id == "pending")
        }),
        "cancellation cut the committed admission"
    );
    assert!(matches!(result, Err(Failure::Cancelled)));
    assert!(!saved.calls.contains_key("pending"));
    assert!(
        !saved.events.iter().any(|event| {
            matches!(&event.kind, EventKind::ToolReturned { call_id, .. } if call_id == "running")
        }),
        "Stop does not wait for a running call"
    );
}

#[tokio::test]
async fn a_search_in_a_batch_never_includes_a_siblings_result_whatever_order_they_finish() {
    struct Turns(std::collections::VecDeque<Vec<Item>>);
    impl Provider for Turns {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &Cancellation,
            _: &RunLimits,
            _: &mut dyn Observer,
        ) -> Result<Turn, Cause> {
            Ok(turn(
                self.0
                    .pop_front()
                    .unwrap_or_else(|| vec![Item::assistant("Done")]),
            ))
        }
    }
    for order in [SearchOrder::Late, SearchOrder::Early] {
        let mut owner = Owner {
            search: Some(order),
            ..Owner::default()
        };
        let mut provider = Turns(
            [
                vec![call("needle-earlier", "read", 0)],
                vec![
                    call("needle-sibling", "read", 0),
                    Item::Call(Invocation {
                        id: "search".into(),
                        name: history::SEARCH.into(),
                        arguments: json!({"text":"needle"}).to_string(),
                    }),
                ],
            ]
            .into(),
        );
        let mut frozen = request();
        frozen.tools.push(history::tools()[2].function());
        recover::work(frozen.freeze().unwrap(), "search-order", &[], 0)
            .unwrap()
            .run(
                &mut provider,
                &mut owner,
                RunLimits::default(),
                &Cancellation::default(),
            )
            .await
            .unwrap();
        let state = owner.saved.lock().unwrap();
        let found = state
            .events
            .iter()
            .find_map(|event| match &event.kind {
                EventKind::ToolReturned {
                    call_id,
                    result: results::ToolResult::History(page),
                } if call_id == "search" => Some(serde_json::to_string(page).unwrap()),
                _ => None,
            })
            .expect("the search result is saved");
        assert!(found.contains("needle-earlier"), "{found}");
        assert!(
            !found.contains("needle-sibling"),
            "only results saved before the search's turn started: {found}"
        );
    }
}

fn hang(id: &str, name: &str) -> Item {
    Item::Call(Invocation {
        id: id.into(),
        name: name.into(),
        arguments: json!({"hang":true}).to_string(),
    })
}

/// The model-facing result of one call in a request.
fn result_of(request: &Request, id: &str) -> Value {
    request
        .items
        .iter()
        .find_map(|item| match item {
            Item::MaterializedResult { call_id, output } if call_id == id => {
                Some(serde_json::from_str(output).unwrap())
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("no result for {id}"))
}

/// Runs one turn of calls to completion, or fails after an hour of virtual time.
async fn run_calls(owner: &mut Owner, calls: Vec<Item>) -> (Turn, Script) {
    let mut provider = Script {
        next: Some(calls),
        ..Script::default()
    };
    let answer = tokio::time::timeout(
        Duration::from_secs(3600),
        recover::work(request().freeze().unwrap(), "deadline", &[], 0)
            .unwrap()
            .run(
                &mut provider,
                owner,
                RunLimits::default(),
                &Cancellation::default(),
            ),
    )
    .await
    .expect("the run ends although a call never returns")
    .expect("a call that times out does not end the run");
    (answer, provider)
}

#[tokio::test(start_paused = true)]
async fn a_read_that_never_returns_times_out_at_its_deadline_and_the_run_continues() {
    let mut owner = Owner::default();
    let started = tokio::time::Instant::now();
    let (answer, provider) = run_calls(&mut owner, vec![hang("stuck", "read")]).await;
    assert_eq!(answer.text(), "Done");
    assert_eq!(provider.requests.len(), 2);
    assert_eq!(
        started.elapsed().as_secs(),
        300,
        "the deadline is 300 seconds"
    );
    assert_eq!(
        result_of(&provider.requests[1], "stuck"),
        json!({
            "executed": false,
            "error": "This call timed out before it finished. You can try it again."
        })
    );
    let saved = owner.saved.lock().unwrap();
    assert!(saved.events.iter().any(|event| matches!(
        &event.kind,
        EventKind::ToolReturned { call_id, .. } if call_id == "stuck"
    )));
}

#[tokio::test(start_paused = true)]
async fn an_effect_that_never_returns_times_out_with_an_unknown_outcome() {
    let mut owner = Owner::default();
    let (answer, provider) = run_calls(&mut owner, vec![hang("stuck", "write")]).await;
    assert_eq!(answer.text(), "Done");
    assert_eq!(
        result_of(&provider.requests[1], "stuck"),
        json!({
            "outcome": "unknown",
            "error": "This call timed out before it finished and its outcome is unknown. Check the current state before you repeat it."
        })
    );
}

/// Starts one hanging read, applies `interrupt` once it has started, and
/// returns the run's result with the virtual time from the interrupt to the end.
async fn interrupt_a_running_call(
    owner: &mut Owner,
    calls: Vec<Item>,
    cancel: &Cancellation,
    interrupt: impl FnOnce(),
) -> (Result<Turn, Failure>, Duration) {
    let saved = owner.saved.clone();
    let mut provider = Script {
        next: Some(calls),
        ..Script::default()
    };
    let run = recover::work(request().freeze().unwrap(), "interrupted", &[], 0)
        .unwrap()
        .run(&mut provider, owner, RunLimits::default(), cancel);
    let interrupted = async {
        while !saved
            .lock()
            .unwrap()
            .order
            .iter()
            .any(|entry| entry.starts_with("start:"))
        {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        interrupt();
        tokio::time::Instant::now()
    };
    let (result, at) = tokio::time::timeout(Duration::from_secs(3600), async {
        tokio::join!(run, interrupted)
    })
    .await
    .expect("the run ends");
    (result, at.elapsed())
}

/// Whether the saved record has this event for the call.
fn saved(owner: &Owner, kind: &str, id: &str) -> bool {
    owner
        .saved
        .lock()
        .unwrap()
        .events
        .iter()
        .any(|event| match &event.kind {
            EventKind::ToolStarted(call) => kind == "started" && call.id == id,
            EventKind::ToolReturned { call_id, .. } => kind == "returned" && call_id == id,
            _ => false,
        })
}

#[tokio::test(start_paused = true)]
async fn cancellation_stops_waiting_for_a_running_call() {
    let mut owner = Owner::default();
    let cancel = Cancellation::default();
    let (result, waited) =
        interrupt_a_running_call(&mut owner, vec![hang("stuck", "read")], &cancel, || {
            cancel.cancel()
        })
        .await;
    let error = result.unwrap_err();
    assert!(matches!(error, Failure::Cancelled), "{error}");
    assert_eq!(
        AttemptFailure::from_error(&error, &cancel),
        AttemptFailure::Interrupted
    );
    assert!(waited < Duration::from_secs(1), "waited {waited:?}");
    assert!(saved(&owner, "started", "stuck"));
    assert!(!saved(&owner, "returned", "stuck"));
}

#[tokio::test(start_paused = true)]
async fn withdrawn_work_stops_waiting_for_a_running_call() {
    let mut owner = Owner::default();
    let cancel = Cancellation::default();
    let currency = owner.saved.clone();
    let (result, waited) =
        interrupt_a_running_call(&mut owner, vec![hang("stuck", "read")], &cancel, || {
            currency.lock().unwrap().currency = Some(Loss::Withdrawn);
        })
        .await;
    let error = result.unwrap_err();
    assert!(
        matches!(&error, Failure::Host(cause) if cause.is::<Withdrawn>()),
        "{error}"
    );
    assert_eq!(
        AttemptFailure::from_error(&error, &cancel),
        AttemptFailure::Interrupted
    );
    assert!(waited <= Duration::from_secs(1), "waited {waited:?}");
    assert!(saved(&owner, "started", "stuck"));
    assert!(!saved(&owner, "returned", "stuck"));
}

#[tokio::test(start_paused = true)]
async fn new_input_lets_a_running_call_finish_and_keeps_its_result() {
    let mut owner = Owner::default();
    let cancel = Cancellation::default();
    let currency = owner.saved.clone();
    let (result, waited) = interrupt_a_running_call(
        &mut owner,
        vec![call("slow", "read", 30_000), call("later", "write", 0)],
        &cancel,
        || currency.lock().unwrap().currency = Some(Loss::Superseded),
    )
    .await;
    let error = result.unwrap_err();
    assert!(
        matches!(&error, Failure::Host(cause) if cause.is::<Superseded>()),
        "{error}"
    );
    assert!(waited >= Duration::from_secs(29), "waited {waited:?}");
    assert!(saved(&owner, "returned", "slow"));
    assert!(!saved(&owner, "started", "later"), "admission closed");
}

#[tokio::test(start_paused = true)]
async fn a_failed_result_save_ends_the_run_without_waiting_for_a_running_read() {
    let mut owner = Owner::default();
    owner.saved.lock().unwrap().unsaved_result = Some("fast");
    let mut provider = Script {
        next: Some(vec![call("fast", "read", 10), hang("stuck", "read")]),
        ..Script::default()
    };
    let started = tokio::time::Instant::now();
    let error = tokio::time::timeout(
        Duration::from_secs(3600),
        recover::work(request().freeze().unwrap(), "unsaved", &[], 0)
            .unwrap()
            .run(
                &mut provider,
                &mut owner,
                RunLimits::default(),
                &Cancellation::default(),
            ),
    )
    .await
    .expect("the run ends")
    .unwrap_err();
    assert!(matches!(error, Failure::Host(_)), "{error}");
    let waited = started.elapsed();
    assert!(waited < Duration::from_secs(1), "waited {waited:?}");
    assert!(saved(&owner, "started", "stuck"));
    assert!(!saved(&owner, "returned", "stuck"));
}
