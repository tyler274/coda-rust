#[path = "support/recover.rs"]
mod recover;
use recover::{admitted, fresh, recovered};
#[path = "support/requests.rs"]
mod request_records;
use std::{
    collections::{BTreeMap, VecDeque},
    time::Duration,
};

use async_trait::async_trait;
use harness::engine::{
    recipe::{Finish, Observer, Provider, Request, Usage},
    *,
};
use serde_json::{Value, json};

fn limits() -> RunLimits {
    RunLimits {
        max_calls: 3,
        max_response_bytes: 100_000,
        ..RunLimits::default()
    }
}
fn request(items: Vec<Item>) -> Request {
    Request {
        cache_key: None,
        model: "test-model".into(),
        context_window_tokens: 100_000,
        max_input_tokens: 100_000,
        options: json!({}),
        instructions: "owner instructions".into(),
        items,
        tools: vec![
            Tool {
                name: "save".into(),
                description: "owner action".into(),
                parameters: json!({"type":"object"}),
            }
            .function(),
        ],
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
        usage: Default::default(),
        items,
        receipt: json!({"usage": 1}),
    }
}

struct Script {
    turns: VecDeque<Turn>,
    requests: Vec<Request>,
    fail: bool,
    fail_at: Option<usize>,
    ready_calls: usize,
    fail_ready_at: Option<usize>,
}
impl Provider for Script {
    fn ready(&mut self) -> impl Future<Output = Result<(), Cause>> + Send {
        self.ready_calls += 1;
        let fail = self.fail_ready_at == Some(self.ready_calls);
        async move {
            if fail {
                Err("readiness gate unavailable".into())
            } else {
                Ok(())
            }
        }
    }
    async fn exchange(
        &mut self,
        request: &Request,
        _: &Cancellation,
        _: &RunLimits,
        _: &mut dyn Observer,
    ) -> Result<Turn, Cause> {
        self.requests.push(request.clone());
        if self.fail || self.fail_at == Some(self.requests.len()) {
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
            fail_at: None,
            ready_calls: 0,
            fail_ready_at: None,
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
    tool_outcomes: VecDeque<ToolOutcome>,
    finish_outcomes: VecDeque<Finish>,
    revoke_after_start: bool,
    currency: Currency,
    /// The next call that runs steers the attempt with new input, then returns.
    steer_on_tool: bool,
    finished: usize,
    partial: Vec<LimitBackstop>,
    /// The next tool call fails with this error.
    tool_error: Option<ToolError>,
    /// Saving a tool result fails retryably this many times.
    save_failures: usize,
    /// Each tool result event whose save failed, as JSON.
    unsaved: Vec<Value>,
}
/// Whether the attempt is still current, as the owner's check reports it.
#[derive(Default)]
enum Currency {
    #[default]
    Current,
    /// Authority was lost.
    Revoked,
    /// Newer input superseded the attempt.
    Superseded,
}
/// A tool error, and whether the attempt was superseded at the same time.
enum ToolError {
    Own(&'static str),
    Superseding(&'static str),
}
fn event_name(kind: &EventKind) -> &'static str {
    match kind {
        EventKind::ProviderFailed { .. } => "provider_failed",
        EventKind::ContextRejected => "context_rejected",
        EventKind::TransportRetry { .. } => "transport_retry",
        EventKind::ProviderBlocked { .. } => "provider_blocked",
        EventKind::ProviderStarted { .. } => "provider_start",
        EventKind::ProviderReturned(_) => "provider_return",
        EventKind::ToolStarted(_) => "tool_start",
        EventKind::ToolReturned { .. } => "tool_return",
        EventKind::ProviderObservation { .. } => "observation",
        EventKind::Candidate { .. } => "candidate",
        EventKind::WindowOpened { .. } => "window_opened",
    }
}
#[async_trait]
impl Product for Owner {
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

    async fn assert_current(&mut self) -> Result<(), ProductError> {
        match self.currency {
            Currency::Current => Ok(()),
            Currency::Revoked => Err("revoked".into()),
            Currency::Superseded => Err(NotCurrent::Superseded.into()),
        }
    }
    async fn persist(&mut self, event: Event) -> Result<(), ProductError> {
        if self.pending_event {
            return std::future::pending().await;
        }
        if self.fail_event == Some(event_name(&event.kind)) {
            return Err("not acknowledged".into());
        }
        if self.save_failures > 0 && matches!(event.kind, EventKind::ToolReturned { .. }) {
            self.save_failures -= 1;
            self.unsaved.push(json!(event));
            return Err(RetryablePersistence::new("storage unavailable").into());
        }
        if self.revoke_after_start && matches!(event.kind, EventKind::ToolStarted(_)) {
            self.currency = Currency::Revoked;
        }
        self.events.push(event);
        Ok(())
    }
    fn execute(&mut self, _: &Invocation, _: Value) -> harness::engine::recipe::ToolFuture {
        if let Some(error) = self.tool_error.take() {
            let error = match error {
                ToolError::Own(error) => error,
                ToolError::Superseding(error) => {
                    self.currency = Currency::Superseded;
                    error
                }
            };
            return Box::pin(std::future::ready(Err(error.into())));
        }
        if let Some(outcome) = self.tool_outcomes.pop_front() {
            self.executed += usize::from(matches!(&outcome, ToolOutcome::Returned(_)));
            return Box::pin(std::future::ready(Ok(outcome)));
        }
        if std::mem::take(&mut self.steer_on_tool) {
            self.executed += 1;
            self.currency = Currency::Superseded;
            return Box::pin(async move {
                tokio::task::yield_now().await;
                Ok(ToolOutcome::Returned("saved after steering".into()))
            });
        }
        self.executed += 1;
        Box::pin(std::future::ready(Ok(ToolOutcome::Returned(
            "saved".into(),
        ))))
    }
    async fn finish(&mut self, _: &Turn) -> Result<Finish, ProductError> {
        self.finished += 1;
        Ok(self
            .finish_outcomes
            .pop_front()
            .unwrap_or(Finish::Committed))
    }
    async fn finish_partial(
        &mut self,
        _: &Turn,
        backstops: &[LimitBackstop],
    ) -> Result<(), ProductError> {
        self.partial.extend_from_slice(backstops);
        Ok(())
    }
}
impl Owner {
    /// How many times the harness recorded this call as started.
    fn started(&self, id: &str) -> usize {
        self.events
            .iter()
            .filter(|event| matches!(&event.kind, EventKind::ToolStarted(call) if call.id == id))
            .count()
    }
}

#[tokio::test]
async fn a_crash_replays_the_exact_request_only_while_another_work_dispatch_is_allowed() {
    for terminal in [None, Some("transport"), Some("provider")] {
        let mut frozen = request(vec![Item::Message {
            role: "user".into(),
            text: "Keep the completed work and finish this request.".into(),
        }]);
        frozen.cache_key = Some(harness::PromptCacheKey::scoped(
            "recovery-work",
            &["accepted"],
        ));
        let mut first = Script::new(vec![turn(vec![call("completed", "save", "{}")])]);
        first.fail_at = Some(2);
        let mut owner = Owner::default();
        let bounds = RunLimits {
            max_calls: 3,
            ..RunLimits::default()
        };
        let failure = fresh(frozen.clone(), "before-crash")
            .run(&mut first, &mut owner, bounds, &Cancellation::default())
            .await
            .unwrap_err();
        assert!(matches!(failure, Failure::Provider(_)));
        let sent = serde_json::to_vec(&first.requests[1]).unwrap();
        assert!(String::from_utf8_lossy(&sent).contains(LIMIT_WARNING));
        // A crash can follow the dispatch, its retry acknowledgment, or its failure.
        // The product decides eligibility; all eligible retries use the saved bytes.
        let dispatch = owner
            .events
            .iter()
            .rposition(|event| matches!(event.kind, EventKind::ProviderStarted { .. }))
            .unwrap();
        owner.events.truncate(dispatch + 1);
        if let Some(terminal) = terminal {
            let started = owner.events.last().unwrap();
            owner.events.push(Event {
                attempt_id: started.attempt_id.clone(),
                ordinal: started.ordinal,
                sequence: started.sequence + 1,
                kind: if terminal == "transport" {
                    EventKind::TransportRetry {
                        reason: "connection reset".into(),
                    }
                } else {
                    EventKind::ProviderFailed {
                        reason: "provider unavailable".into(),
                    }
                },
            });
        }
        owner.events = serde_json::from_slice(&serde_json::to_vec(&owner.events).unwrap()).unwrap();
        let work = recovered(
            frozen.clone(),
            "after-crash",
            admitted(&owner.events),
            0,
            &[],
            None,
            recovery::Inputs::default(),
        );
        let mut successor =
            Script::new(vec![turn(vec![text("Finished from the completed work.")])]);
        work.run(&mut successor, &mut owner, bounds, &Cancellation::default())
            .await
            .unwrap();
        assert_eq!(
            successor.requests[0].cache_key, frozen.cache_key,
            "recovery retains the accepted run cache scope"
        );
        if terminal == Some("transport") {
            assert_eq!(
                serde_json::to_vec(&successor.requests[0]).unwrap(),
                sent,
                "an allowed retry replays its exact accepted request"
            );
        } else {
            assert!(successor.requests[0].items.iter().any(|item|
                matches!(item, Item::Message { role, text } if role == "developer" && text == FINAL_REQUEST)),
                "a newly reached request backstop needs the final request");
        }
        assert_eq!(owner.executed, 1, "completed effects survive the crash");
    }
}
#[tokio::test]
async fn requests_reference_the_saved_recipe_and_delivered_tool_result() {
    let mut frozen = request(vec![]);
    frozen.instructions = "unique-frozen-instructions".into();
    frozen.cache_key = Some(harness::PromptCacheKey::scoped("test-work", &["first"]));
    let mut other_scope = frozen.clone();
    other_scope.cache_key = Some(harness::PromptCacheKey::scoped("test-work", &["second"]));
    assert_eq!(
        frozen.recipe(),
        other_scope.recipe(),
        "cache scope is not part of the shared recipe"
    );
    let result = "unique-retained-tool-result";
    let mut owner = Owner {
        tool_outcomes: vec![ToolOutcome::Returned(result.into())].into(),
        ..Default::default()
    };
    let mut provider = Script::new(vec![
        turn(vec![call("stored", "save", "{}")]),
        turn(vec![text("Finished using the saved result.")]),
    ]);
    fresh(frozen.clone(), "parts")
        .run(
            &mut provider,
            &mut owner,
            RunLimits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert!(
        provider
            .requests
            .iter()
            .all(|request| request.cache_key == frozen.cache_key)
    );
    let saved = serde_json::to_string(&(&frozen, &owner.events)).unwrap();
    assert_eq!(
        saved.matches(&frozen.instructions).count(),
        1,
        "the frozen recipe is saved once"
    );
    assert_eq!(
        saved.matches(result).count(),
        1,
        "the delivered tool result is saved once"
    );
    assert!(
        provider.requests[1].items.iter().any(
            |item| matches!(item, Item::MaterializedResult { output, .. } if output == result)
        )
    );
}

#[tokio::test]
async fn acknowledged_loop_pairs_results_and_returns_only_candidate() {
    let mut provider = Script::new(vec![
        turn(vec![text("First. "), call("c", "save", "{}")]),
        turn(vec![text("Done.")]),
    ]);
    let mut owner = Owner::default();
    let result = fresh(request(vec![]), "attempt")
        .run(
            &mut provider,
            &mut owner,
            limits(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(result.text(), "Done.");
    assert_eq!(provider.requests[1].items.first(), Some(&text("First. ")));
    assert_eq!(owner.finished, 1);
    assert_eq!(owner.executed, 1);
    assert_eq!(provider.requests.len(), 2);
    assert!(provider.requests[1].items.iter().any(|item| matches!(item, Item::MaterializedResult { call_id, output } if call_id == "c" && output.starts_with("saved") && output.ends_with(LIMIT_WARNING))));
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

#[derive(Default)]
struct WindowOwner {
    events: Vec<Event>,
    notes: BTreeMap<String, String>,
    effect_counts: BTreeMap<String, usize>,
    completed: Option<String>,
    completion_count: usize,
    seed: Vec<Item>,
    stable_output: Option<String>,
    stable_outputs: BTreeMap<String, String>,
    lose_save_receipt_once: bool,
    history_messages: Vec<history::Message<Value>>,
    source_updates: VecDeque<Vec<ProductMessage>>,
    note_references: Vec<String>,
}

#[async_trait]
impl Product for WindowOwner {
    async fn message_sources(&mut self) -> Result<Option<Vec<ProductMessage>>, ProductError> {
        Ok(self.source_updates.pop_front())
    }

    async fn admit_binding(
        &mut self,
        _: &harness::engine::recipe::RequestBinding,
    ) -> Result<(), harness::engine::ProductError> {
        Ok(())
    }

    async fn assert_current(&mut self) -> Result<(), ProductError> {
        Ok(())
    }

    async fn persist(&mut self, event: Event) -> Result<(), ProductError> {
        if self.lose_save_receipt_once
            && matches!(&event.kind, EventKind::ToolReturned { call_id, .. } if call_id == "save")
        {
            self.lose_save_receipt_once = false;
            return Err("process lost after the note save".into());
        }
        self.events.push(event);
        Ok(())
    }

    fn execute(
        &mut self,
        call: &Invocation,
        arguments: Value,
    ) -> harness::engine::recipe::ToolFuture {
        let outcome = (|| {
            *self.effect_counts.entry(call.id.clone()).or_default() += 1;
            match call.name.as_str() {
                "save_working_note" => {
                    let text = arguments["text"]
                        .as_str()
                        .filter(|text| !text.trim().is_empty())
                        .ok_or("empty note")?;
                    if let Some(saved) = self.notes.get(&call.id) {
                        if saved != text {
                            return Err("note identity collision".into());
                        }
                    } else {
                        self.notes.insert(call.id.clone(), text.into());
                    }
                    Ok(ToolOutcome::Returned(
                        json!({"reference":call.id}).to_string(),
                    ))
                }
                history::LIST | history::SEARCH => {
                    let query = history::Query::parse(&call.name, &arguments)?;
                    let page = query.answer(&self.history_messages, admitted(&self.events))?;
                    Ok(ToolOutcome::History(page))
                }
                history::READ => {
                    if let Some(output) = self.stable_outputs.get(&call.id) {
                        Ok(ToolOutcome::History(serde_json::from_str(output)?))
                    } else {
                        Ok(ToolOutcome::History(
                            history::Query::parse(&call.name, &arguments)?
                                .answer(&self.history_messages, admitted(&self.events))?,
                        ))
                    }
                }
                "read_working_note" => {
                    let reference = arguments["reference"].as_str().ok_or("missing reference")?;
                    let text = self.notes.get(reference).ok_or("unknown note")?;
                    Ok(ToolOutcome::Returned(
                        json!({"reference":reference,"text":text}).to_string(),
                    ))
                }
                _ => Ok(ToolOutcome::Returned(
                    self.stable_outputs
                        .get(&call.id)
                        .cloned()
                        .or_else(|| self.stable_output.clone())
                        .unwrap_or_else(|| json!({"stable":"evidence-7"}).to_string()),
                )),
            }
        })();
        Box::pin(std::future::ready(outcome))
    }

    fn saved_outcome(
        &mut self,
        call: &Invocation,
    ) -> harness::engine::recipe::ToolFuture<Option<ToolOutcome>> {
        let outcome = self
            .notes
            .contains_key(&call.id)
            .then(|| ToolOutcome::Returned(json!({"reference":call.id}).to_string()));
        Box::pin(std::future::ready(Ok(outcome)))
    }

    fn read_result(&mut self, id: &str) -> harness::engine::recipe::ToolFuture<Option<String>> {
        let messages: Vec<_> = self
            .history_messages
            .iter()
            .map(history::Message::source)
            .collect();
        let outcome = results::resolve(admitted(&self.events), id, &messages).map_err(Into::into);
        Box::pin(std::future::ready(outcome))
    }

    async fn finish(&mut self, turn: &Turn) -> Result<Finish, ProductError> {
        self.completion_count += 1;
        self.completed = Some(turn.text());
        Ok(Finish::Committed)
    }

    async fn finish_partial(
        &mut self,
        turn: &Turn,
        _: &[LimitBackstop],
    ) -> Result<(), ProductError> {
        self.completed = Some(turn.text());
        Ok(())
    }

    async fn window_seed(&mut self) -> Result<harness::engine::WindowSeed, ProductError> {
        Ok(harness::engine::WindowSeed {
            items: self.seed.clone(),
            note_references: self.note_references.clone(),
        })
    }
}

fn window_request() -> Request {
    let mut request = request(vec![Item::Message {
        role: "user".into(),
        text: format!("original accepted assignment {}", "x".repeat(500)),
    }]);
    request.tools.push(
        Tool {
            name: "stable_read".into(),
            description: "read stable evidence".into(),
            parameters: json!({"type":"object"}),
        }
        .function(),
    );
    offer_working_notes(&mut request.instructions, &mut request.tools);
    request
}

/// The first or the ending request near the end of the window.
/// The result text before the appended limit warning, when the warning is there.
/// The output the model got for a call in a request, without the limit notice.
fn sent_result<'a>(request: &'a Request, id: &str) -> &'a str {
    let output = request
        .items
        .iter()
        .find_map(|item| match item {
            Item::MaterializedResult { call_id, output } if call_id == id => Some(output),
            _ => None,
        })
        .unwrap_or_else(|| panic!("no result for {id}"));
    before_warning(output).unwrap_or(output)
}

fn before_warning(output: &str) -> Option<&str> {
    output.strip_suffix(LIMIT_WARNING)?.strip_suffix("\n\n")
}

fn is_handoff_ask(text: &str) -> bool {
    text == handoff_ending() || text.starts_with("This context window is nearly full")
}

#[tokio::test]
async fn fresh_window_requires_a_durable_handoff_and_keeps_sibling_evidence() {
    let note = "arbitrary model note: pending synthesis; evidence reference evidence-7";
    let mut provider = Script::new(vec![
        turn(vec![call("prior", "stable_read", "{}")]),
        turn(vec![
            call(
                "save",
                "save_working_note",
                &json!({"text":note,"fresh_window":true}).to_string(),
            ),
            call("read", "stable_read", "{}"),
        ]),
        turn(vec![call(
            "open",
            "read_working_note",
            &json!({"reference":"save"}).to_string(),
        )]),
        turn(vec![text("Completed from the exact durable handoff.")]),
    ]);
    let mut owner = WindowOwner {
        seed: vec![Item::Message {
            role: "user".into(),
            text: "original accepted assignment".into(),
        }],
        stable_output: Some(json!({"stable":"evidence-7","exact":"x".repeat(800)}).to_string()),
        ..Default::default()
    };
    let mut bounds = limits();
    bounds.max_calls = 6;
    let mut frozen = window_request();
    frozen.context_window_tokens = 200_000;
    frozen.max_input_tokens = 200_000;
    provider.turns[0].usage = Usage {
        cached_input_tokens: None,
        input_tokens: Some(179_500),
        output_tokens: Some(500),
    };
    let result = fresh(frozen, "attempt")
        .run(&mut provider, &mut owner, bounds, &Cancellation::default())
        .await
        .unwrap();
    assert_eq!(result.text(), "Completed from the exact durable handoff.");
    assert_eq!(owner.notes.get("save").map(String::as_str), Some(note));
    assert!(
        !provider.requests[0]
            .items
            .iter()
            .any(|item| matches!(item, Item::Message { text, .. } if is_handoff_ask(text)))
    );
    assert!(
        provider.requests[1]
            .items
            .iter()
            .any(|item| matches!(item, Item::Message { text, .. } if is_handoff_ask(text)))
    );
    let fresh = &provider.requests[2];
    assert!(fresh.items.contains(&Item::assistant(note)));
    assert!(
        provider
            .requests
            .iter()
            .all(|request| request.tools == provider.requests[0].tools)
    );
    assert!(
        !fresh.items.iter().any(
            |item| matches!(item, Item::Message { text, .. } if text.contains(&"x".repeat(100)))
        )
    );
    let [opening] = developer_text(fresh)[..] else {
        panic!("one developer message opens the fresh window");
    };
    assert!(opening.contains("starts fresh for the same accepted work"));
    assert!(opening.contains("reference save"));
    assert!(
        !opening.contains("Work done after this note"),
        "a note saved for this window covers the work so far"
    );
    assert!(fresh.items.iter().any(
        |item| matches!(item, Item::MaterializedResult { call_id, output } if call_id == "read" && output.contains("evidence-7"))
    ));
    assert!(provider.requests[3].items.iter().any(
        |item| matches!(item, Item::MaterializedResult { call_id, output } if call_id == "open" && output.contains(note))
    ));
    assert_eq!(
        provider
            .requests
            .iter()
            .flat_map(|request| request.items.iter())
            .filter(|item| matches!(item, Item::Message { text, .. } if is_handoff_ask(text)))
            .count(),
        1,
        "resident seed bytes cannot force another handoff before the answer"
    );
    let recorded = request_records::dispatched(&owner.events, &provider.requests[0]);
    assert_eq!(recorded.len(), provider.requests.len());
    for (saved, sent) in recorded.iter().zip(&provider.requests) {
        assert_eq!(saved.tools, sent.tools);
        assert_eq!(saved.instructions, sent.instructions);
        let saved_nonresults: Vec<_> = saved
            .items
            .iter()
            .filter(|item| !matches!(item, Item::Result { .. }))
            .collect();
        let sent_nonresults: Vec<_> = sent
            .items
            .iter()
            .filter(|item| !matches!(item, Item::MaterializedResult { .. }))
            .collect();
        assert_eq!(saved_nonresults, sent_nonresults);
        let saved_results: Vec<_> = saved
            .items
            .iter()
            .filter_map(|item| match item {
                Item::Result { call_id, .. } => Some(call_id),
                _ => None,
            })
            .collect();
        let sent_results: Vec<_> = sent
            .items
            .iter()
            .filter_map(|item| match item {
                Item::MaterializedResult { call_id, .. } => Some(call_id),
                _ => None,
            })
            .collect();
        assert_eq!(saved_results, sent_results);
    }
    let opened = owner
        .events
        .iter()
        .find(|event| matches!(event.kind, EventKind::WindowOpened { .. }))
        .unwrap();
    let EventKind::WindowOpened {
        opened_at_unix_ms: timestamp,
        ..
    } = opened.kind
    else {
        unreachable!()
    };
    let dispatch_times: Vec<_> = owner
        .events
        .iter()
        .filter_map(|event| match event.kind {
            EventKind::ProviderStarted {
                dispatched_at_unix_ms,
                ..
            } => Some(dispatched_at_unix_ms),
            _ => None,
        })
        .collect();
    assert!(
        timestamp >= dispatch_times[1] && timestamp <= dispatch_times[2],
        "WindowOpened retains its own opening time"
    );
}

#[tokio::test]
async fn a_window_asked_for_in_a_batch_carries_the_newest_note_of_that_batch() {
    let save = |id: &str, text: &str, fresh_window: bool| {
        call(
            id,
            "save_working_note",
            &json!({"text":text,"fresh_window":fresh_window}).to_string(),
        )
    };
    let older = "Older checkpoint that asked for the window.";
    let newest = "Newest checkpoint, saved after the ask.";
    let mut provider = Script::new(vec![
        turn(vec![
            save("asked", older, true),
            save("newest", newest, false),
        ]),
        turn(vec![text("Done.")]),
    ]);
    let mut owner = WindowOwner {
        seed: vec![Item::Message {
            role: "user".into(),
            text: "original accepted assignment".into(),
        }],
        ..Default::default()
    };
    fresh(window_request(), "newest-note")
        .run(
            &mut provider,
            &mut owner,
            limits(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(window_reasons(&owner.events), [WindowReason::ModelAsked]);
    assert!(owner.events.iter().any(|event| matches!(&event.kind,
        EventKind::WindowOpened { handoff: Some(note), .. } if note.reference == "newest")));
    let opened = &provider.requests[1];
    assert!(opened.items.contains(&Item::assistant(newest)));
    assert!(!opened.items.contains(&Item::assistant(older)));
    let [message] = developer_text(opened)[..] else {
        panic!("one developer message opens the fresh window");
    };
    assert!(message.contains("reference newest"), "{message}");
    assert!(message.contains(r#"["asked"]"#), "{message}");
}

#[tokio::test]
async fn opening_history_uses_the_same_pressure_point_as_later_requests() {
    for (window, input_limit, threshold) in [
        (100_000, 100_000, 75_254),
        (272_000, 272_000, 230_054),
        (100_000, 50_000, 45_000),
    ] {
        for (target_tokens, seeded) in [
            (threshold - 500, false),
            (threshold - 10, true),
            (threshold + 200, true),
        ] {
            let current = Item::Message {
                role: "user".into(),
                text: "Continue the accepted question.".into(),
            };
            let mut frozen = window_request();
            frozen.context_window_tokens = window;
            frozen.max_input_tokens = input_limit;
            frozen.items = vec![Item::assistant(""), current.clone()];
            let overhead = context::estimate_request(&frozen).unwrap();
            let earlier = Item::assistant("x".repeat(((target_tokens - overhead) * 4) as usize));
            frozen.items[0] = earlier.clone();
            assert_eq!(context::estimate_request(&frozen).unwrap(), target_tokens);
            // The middle case crosses 90% of usable input with the handoff prompt.
            // Opening and subsequent dispatches must reserve that same prompt.
            let mut owner = WindowOwner {
                seed: vec![current.clone()],
                ..Default::default()
            };
            let mut provider = Script::new(vec![turn(vec![text("Answer.")])]);
            fresh(frozen, "opening")
                .run(
                    &mut provider,
                    &mut owner,
                    limits(),
                    &Cancellation::default(),
                )
                .await
                .unwrap();
            let [sent] = provider.requests.as_slice() else {
                panic!("one opening request");
            };
            assert_eq!(
                sent.items.contains(&earlier),
                !seeded,
                "past the handoff point, open from the product seed"
            );
            assert!(sent.items.contains(&current));
            assert!(
                !sent.items.iter().any(|item| matches!(item,
                    Item::Message { text, .. } if is_handoff_ask(text)
                )),
                "a seeded opening does not need an immediate handoff"
            );
            assert!(sent.options.get("max_output_tokens").is_none());
        }
    }
}

#[tokio::test]
async fn full_context_without_a_note_opens_from_exact_inputs_and_pending_results() {
    let seed = vec![
        Item::Message {
            role: "user".into(),
            text: "Original assignment, unchanged.".into(),
        },
        Item::Message {
            role: "user".into(),
            text: "Accepted steering, unchanged.".into(),
        },
    ];
    let outputs = BTreeMap::from([
        ("earlier".into(), "o".repeat(7_000)),
        ("large".into(), format!("START {} END", "n".repeat(6_500))),
        ("small".into(), "s".repeat(2_000)),
    ]);
    let mut owner = WindowOwner {
        stable_outputs: outputs.clone(),
        seed: seed.clone(),
        ..Default::default()
    };
    let mut provider = Script::new(vec![
        turn(vec![call("earlier", "stable_read", "{}")]),
        turn(vec![
            call("large", "stable_read", "{}"),
            call("small", "stable_read", "{}"),
        ]),
        turn(vec![text("Done from retained results.")]),
    ]);
    let mut frozen = window_request();
    frozen.tools.push(results::tool().function());
    // Too small for the earlier result plus previews of this turn, large
    // enough for the fresh window to show the pending results whole.
    frozen.max_input_tokens = 3_800;
    frozen.items.push(Item::assistant(
        "Earlier investigation stays in the run record.",
    ));
    fresh(frozen, "attempt")
        .run(
            &mut provider,
            &mut owner,
            RunLimits {
                max_calls: 8,
                ..limits()
            },
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(provider.requests.len(), 3);
    assert!(provider.requests[2].items.starts_with(&seed));
    assert!(!provider.requests[2].items.contains(&Item::assistant(
        "Earlier investigation stays in the run record."
    )));
    assert_eq!(owner.completion_count, 1);
    assert!(owner.notes.is_empty());
    assert!(provider.requests[1].items.iter().any(|item| matches!(item,
        Item::MaterializedResult { call_id, output } if call_id == "earlier" && output == &outputs["earlier"])));
    assert!(
        !provider.requests[2].items.iter().any(|item| matches!(item,
        Item::MaterializedResult { call_id, .. } if call_id == "earlier")),
        "context pressure opens a fresh window instead of trimming pending results"
    );
    assert!(owner.events.iter().any(|event| matches!(
        event.kind,
        EventKind::WindowOpened {
            reason: WindowReason::Full,
            ..
        }
    )));
    let shown = |id: &str| {
        provider.requests[2]
            .items
            .iter()
            .find_map(|item| match item {
                Item::MaterializedResult { call_id, output } if call_id == id => Some(output),
                _ => None,
            })
            .unwrap()
    };
    assert_eq!(shown("small"), &outputs["small"]);
    assert_eq!(
        shown("large"),
        &outputs["large"],
        "pending results within their bounds remain whole"
    );
    for (id, original) in outputs {
        assert_eq!(
            results::resolve(admitted(&owner.events), &id, &[])
                .unwrap()
                .as_deref(),
            Some(original.as_str())
        );
        assert_eq!(owner.effect_counts.get(&id), Some(&1));
    }
}

#[tokio::test]
async fn a_parallel_batch_uses_short_previews_and_can_continue_after_restart() {
    let outputs = (1..=3)
        .map(|n| {
            (
                format!("source-{n}"),
                format!("Evidence {n}: {}", "x\"🪕".repeat(44_000)),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut owner = WindowOwner {
        stable_outputs: outputs.clone(),
        ..Default::default()
    };
    let mut frozen = window_request();
    frozen.tools.push(results::tool().function());
    frozen
        .tools
        .extend(history::tools().into_iter().map(|tool| tool.function()));
    frozen.context_window_tokens = 24_000;
    let bounds = RunLimits {
        max_calls: 8,
        ..limits()
    };
    let mut first = Script::new(vec![turn(
        outputs
            .keys()
            .map(|id| call(id, "stable_read", "{}"))
            .collect(),
    )]);
    // Interrupt before the first new-window dispatch. Its source reads must
    // remain addressable on a replacement worker without executing them twice.
    first.turns.push_back(turn(vec![call("note","save_working_note",
        r#"{"text":"Three large sources were retained. Continue with the outstanding page.","fresh_window":true}"#)]));
    first.fail_ready_at = Some(3);
    let error = fresh(frozen.clone(), "first")
        .run(&mut first, &mut owner, bounds, &Cancellation::default())
        .await
        .unwrap_err();
    assert!(matches!(error, Failure::Provider(_)));
    let handoff = &first.requests[1];
    assert!(
        harness::engine::context::estimate_request(handoff).unwrap() < frozen.context_window_tokens
    );
    assert_eq!(handoff.tools, frozen.tools);
    assert!(
        Request {
            items: handoff.items.clone(),
            ..frozen.clone()
        }
        .freeze()
        .is_err(),
        "provider-wire results never reenter retained context"
    );
    let mut offsets = BTreeMap::new();
    for (id, original) in &outputs {
        let encoded = handoff
            .items
            .iter()
            .find_map(|item| match item {
                Item::MaterializedResult { call_id, output } if call_id == id => Some(output),
                _ => None,
            })
            .unwrap();
        assert!(
            encoded.chars().count() <= 1_500,
            "preview must remain bounded"
        );
        let page: Value = serde_json::from_str(
            handoff
                .items
                .iter()
                .find_map(|i| match i {
                    Item::MaterializedResult { call_id, output } if call_id == id => Some(output),
                    _ => None,
                })
                .unwrap(),
        )
        .unwrap();
        assert_eq!(page["read_with"], results::READ);
        let offset = page["next_offset"].as_u64().unwrap() as usize;
        assert!(offset > 0 && offset < original.len());
        assert_eq!(page["text"], original[..offset]);
        offsets.insert(id.clone(), offset);
        assert_eq!(
            results::resolve(admitted(&owner.events), id, &[])
                .unwrap()
                .as_deref(),
            Some(original.as_str())
        );
    }
    let offset = offsets["source-2"];
    let mut continuation = Script::new(vec![
        turn(vec![
            call(
                "directory",
                history::LIST,
                r#"{"tool_name":"stable_read","window":0}"#,
            ),
            call(
                "next",
                results::READ,
                &json!({"call_id":"source-2","offset":offset}).to_string(),
            ),
        ]),
        turn(vec![text("Completed from the retained evidence.")]),
    ]);
    let work = recovered(
        frozen,
        "successor",
        admitted(&owner.events),
        0,
        &[],
        None,
        recovery::Inputs::default(),
    );
    work.run(
        &mut continuation,
        &mut owner,
        bounds,
        &Cancellation::default(),
    )
    .await
    .unwrap();
    let directory: Value = serde_json::from_str(
        continuation.requests[1]
            .items
            .iter()
            .find_map(|item| match item {
                Item::MaterializedResult { call_id, output } if call_id == "directory" => {
                    Some(output)
                }
                _ => None,
            })
            .unwrap(),
    )
    .unwrap();
    assert_eq!(directory["items"].as_array().unwrap().len(), outputs.len());
    for item in directory["items"].as_array().unwrap() {
        assert!(outputs.contains_key(item["call"]["id"].as_str().unwrap()));
        assert_eq!(item["window"], 0);
    }
    let page: Value = serde_json::from_str(
        continuation.requests[1]
            .items
            .iter()
            .find_map(|i| match i {
                Item::MaterializedResult { call_id, output } if call_id == "next" => Some(output),
                _ => None,
            })
            .unwrap(),
    )
    .unwrap();
    assert_eq!(page["offset"], offset);
    assert!(outputs["source-2"][offset..].starts_with(page["text"].as_str().unwrap()));
    assert!(
        first
            .requests
            .iter()
            .chain(&continuation.requests)
            .all(
                |r| harness::engine::context::estimate_request(r).unwrap() + 100
                    <= r.context_window_tokens
            )
    );
    for id in outputs.keys() {
        assert_eq!(owner.effect_counts.get(id), Some(&1));
    }
    assert_eq!(owner.completion_count, 1);
}

#[tokio::test]
async fn interrupted_note_save_reconciles_by_call_identity_before_window_advance() {
    let response = turn(vec![call(
        "save",
        "save_working_note",
        r#"{"text":"durable note","fresh_window":true}"#,
    )]);
    let mut first = Script::new(vec![response.clone()]);
    let mut owner = WindowOwner {
        lose_save_receipt_once: true,
        seed: vec![Item::Message {
            role: "user".into(),
            text: "same assignment".into(),
        }],
        ..Default::default()
    };
    let request = window_request();
    let error = fresh(request.clone(), "attempt-1")
        .run(&mut first, &mut owner, limits(), &Cancellation::default())
        .await
        .unwrap_err();
    assert!(matches!(error, Failure::Host(_)));
    assert!(
        !owner
            .events
            .iter()
            .any(|event| matches!(event.kind, EventKind::WindowOpened { .. }))
    );
    // Drop and reopen the durable journal before the successor runs.
    owner.events = serde_json::from_str(&serde_json::to_string(&owner.events).unwrap()).unwrap();

    let work = recovered(
        request,
        "attempt-2",
        admitted(&owner.events),
        0,
        &[],
        None,
        recovery::Inputs::default(),
    );
    let mut continuation = Script::new(vec![turn(vec![text("continued")])]);
    work.run(
        &mut continuation,
        &mut owner,
        limits(),
        &Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(
        owner.effect_counts["save"], 1,
        "recovery reads the saved outcome without executing the call again"
    );
    assert!(
        continuation.requests[0].items.iter().any(
            |item| matches!(item, Item::Message { text, .. } if text.contains("reference save"))
        )
    );
}

#[tokio::test]
async fn rearmed_partial_uses_current_message_source_without_copying_it() {
    for deleted in [false, true] {
        let private = "Private partial answer to remove before retry.";
        let frozen = window_request();
        let mut owner = WindowOwner::default();
        fresh(frozen.clone(), "first")
            .run(
                &mut Script::new(vec![turn(vec![text(private)])]),
                &mut owner,
                RunLimits {
                    max_calls: 1,
                    ..limits()
                },
                &Cancellation::default(),
            )
            .await
            .unwrap();
        let partial = ProductMessage {
            id: "partial".into(),
            role: harness::engine::history::Role::Assistant,
            text: owner.completed.clone().unwrap(),
        };
        let work = recovered(
            frozen,
            "retry",
            admitted(&owner.events),
            1,
            std::slice::from_ref(&partial),
            Some(partial.id.clone()),
            recovery::Inputs::default(),
        );
        // The product deletes the source after assembling the continuation, before dispatch.
        if deleted {
            owner.source_updates.push_back(vec![]);
        }
        let mut provider = Script::new(vec![turn(vec![text("Remaining work completed.")])]);
        work.run(
            &mut provider,
            &mut owner,
            limits(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
        assert_eq!(
            serde_json::to_string(&provider.requests[0])
                .unwrap()
                .contains(private),
            !deleted,
            "partial text must follow its current product source"
        );
        for event in &owner.events {
            if let EventKind::ProviderStarted { request, .. } = &event.kind {
                assert!(
                    !serde_json::to_string(request).unwrap().contains(private),
                    "dispatch records retain a message identity, never a copied partial"
                );
            }
        }
    }
}

#[tokio::test]
async fn an_old_note_before_new_input_cannot_open_a_window() {
    let mut initial = window_request();
    initial.items = vec![
        call(
            "old-save",
            "save_working_note",
            r#"{"text":"old promise","fresh_window":true}"#,
        ),
        Item::Result {
            call_id: "old-save".into(),
            output: results::ResultView::inline(json!({"reference":"old-save"}).to_string()),
        },
        Item::Message {
            role: "user".into(),
            text: "new direction after that note".into(),
        },
    ];
    let mut provider = Script::new(vec![turn(vec![text("Answered the new direction.")])]);
    let mut owner = WindowOwner {
        seed: vec![Item::Message {
            role: "user".into(),
            text: "wrong stale seed".into(),
        }],
        ..Default::default()
    };
    fresh(initial, "attempt")
        .run(
            &mut provider,
            &mut owner,
            limits(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert!(provider.requests[0].items.iter().any(
        |item| matches!(item, Item::Message { text, .. } if text == "new direction after that note")
    ));
    assert!(
        !owner
            .events
            .iter()
            .any(|event| matches!(event.kind, EventKind::WindowOpened { .. }))
    );
}

#[tokio::test]
async fn one_warning_precedes_a_reserved_final_with_the_admitted_tools() {
    let mut provider = Script::new(vec![
        turn(vec![call("read-1", "save", "{}")]),
        turn(vec![call("read-2", "save", "{}")]),
        turn(vec![text(
            "Honest partial answer; one check remains unavailable.",
        )]),
    ]);
    let mut owner = Owner::default();
    let mut bounds = limits();
    bounds.max_calls = 3;
    fresh(request(vec![]), "attempt")
        .run(&mut provider, &mut owner, bounds, &Cancellation::default())
        .await
        .unwrap();
    assert_eq!(provider.requests.len(), 3);
    assert!(
        provider.requests[1]
            .items
            .iter()
            .any(|item| matches!(item, Item::MaterializedResult { call_id, output } if call_id == "read-1" && output.ends_with(LIMIT_WARNING)))
    );
    assert_eq!(
        provider
            .requests
            .iter()
            .flat_map(|request| &request.items)
            .filter(|item| matches!(item, Item::MaterializedResult { output, .. } if output.ends_with(LIMIT_WARNING)))
            .count(),
        2
    );
    assert!(owner.events.iter().any(|event| matches!(
        &event.kind,
        EventKind::ToolReturned { call_id, result: results::ToolResult::Original(output), .. }
            if call_id == "read-1" && output == "saved"
    )));
    assert!(
        results::Delivery::from_events(admitted(&owner.events), &[])
            .unwrap()
            .covers("read-1", None),
        "a persistent warning must not hide delivered source evidence"
    );
    assert_eq!(provider.requests[2].tools, provider.requests[0].tools);
    assert!(
        provider.requests[2]
            .items
            .iter()
            .any(|item| matches!(item, Item::Message { text, .. } if text == FINAL_REQUEST))
    );
    assert_eq!(owner.finished, 0);
    assert_eq!(owner.partial, [LimitBackstop::Requests]);
}

#[tokio::test]
async fn unknown_usage_uses_content_estimates_without_inventing_reported_counts() {
    let mut provider = Script::new(vec![
        turn(vec![text(&"x".repeat(4000)), call("read", "save", "{}")]),
        turn(vec![text("The retained work is partial.")]),
    ]);
    let mut owner = Owner::default();
    let mut frozen = request(vec![]);
    frozen.instructions = "x".repeat(4000);
    fresh(frozen, "estimated")
        .run(
            &mut provider,
            &mut owner,
            RunLimits {
                max_calls: 100,
                max_tokens: 1800,
                ..limits()
            },
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(provider.requests.len(), 2);
    assert_eq!(owner.partial, [LimitBackstop::Tokens]);
    let progress = harness::engine::recipe::Progress::from_events(&owner.events);
    assert_eq!(progress.known_tokens, 0);
    assert_eq!(progress.unknown_usage, 2);
    assert!(progress.budget_tokens > 1800);
    assert!(
        owner
            .events
            .iter()
            .filter_map(|event| match &event.kind {
                EventKind::ProviderReturned(turn) => Some(&turn.usage),
                _ => None,
            })
            .all(|usage| usage.input_tokens.is_none() && usage.output_tokens.is_none())
    );
}

#[tokio::test]
async fn failed_acknowledgments_prevent_dependent_dispatch() {
    for (boundary, providers, tools) in [
        ("provider_start", 0, 0),
        ("provider_return", 1, 0),
        ("tool_start", 1, 0),
        ("tool_return", 1, 1),
        ("candidate", 1, 0),
    ] {
        let mut provider = Script::new(vec![turn(if boundary == "candidate" {
            vec![text("done")]
        } else {
            vec![call("c", "save", "{}")]
        })]);
        let mut owner = Owner {
            fail_event: Some(boundary),
            ..Owner::default()
        };
        let error = fresh(request(vec![]), "attempt")
            .run(
                &mut provider,
                &mut owner,
                limits(),
                &Cancellation::default(),
            )
            .await
            .unwrap_err();
        assert!(matches!(error, Failure::Host(_)));
        assert_eq!(provider.requests.len(), providers);
        assert_eq!(owner.executed, tools);
        assert_eq!(owner.finished, 0);
    }
}

#[tokio::test]
async fn empty_and_repeated_call_ids_get_new_ids_that_the_record_results_and_requests_keep() {
    // Some providers repeat one ID in a batch or number calls the same way in
    // every response. Each call still runs once and pairs with its own result.
    let mut provider = Script::new(vec![
        turn(vec![call("same", "save", "{}"), call("same", "save", "{}")]),
        turn(vec![call("same", "save", "{}"), call(" ", "save", "{}")]),
        turn(vec![text("Done.")]),
    ]);
    let mut owner = Owner::default();
    let mut allowance = limits();
    allowance.max_calls = 5;
    fresh(request(vec![]), "attempt")
        .run(
            &mut provider,
            &mut owner,
            allowance,
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(owner.executed, 4, "every call runs once");
    let saved: Vec<_> = owner
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::ProviderReturned(turn) => Some(turn.calls().map(|call| call.id.clone())),
            _ => None,
        })
        .flatten()
        .collect();
    let returned: Vec<_> = owner
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::ToolReturned { call_id, .. } => Some(call_id.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(returned, saved, "each result is saved under its call's ID");
    let unique: std::collections::HashSet<_> = saved.iter().collect();
    assert_eq!(unique.len(), 4, "no two calls in the run share an ID");
    assert!(saved.iter().all(|id| !id.trim().is_empty()));
    let last = provider.requests.last().unwrap();
    let sent_calls: Vec<_> = last
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Call(call) => Some(call.id.clone()),
            _ => None,
        })
        .collect();
    let sent_results: Vec<_> = last
        .items
        .iter()
        .filter_map(|item| match item {
            Item::MaterializedResult { call_id, .. } => Some(call_id.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(sent_calls, saved, "later requests carry the saved IDs");
    assert_eq!(sent_results, saved);
}

#[tokio::test]
async fn a_failed_write_tells_the_model_its_outcome_is_unknown_and_never_runs_again() {
    // The write fails and its result is saved. The attempt then stops before
    // its next request, and a later attempt continues from the saved events.
    let mut provider = Script::new(vec![turn(vec![call("write", "save", "{}")])]);
    provider.fail_ready_at = Some(2);
    let mut owner = Owner {
        tool_error: Some(ToolError::Own("connection reset while saving")),
        ..Owner::default()
    };
    let error = fresh(request(vec![]), "failed-write")
        .run(
            &mut provider,
            &mut owner,
            limits(),
            &Cancellation::default(),
        )
        .await
        .unwrap_err();
    assert!(
        matches!(&error, Failure::Provider(cause) if cause.to_string() == "readiness gate unavailable"),
        "a tool's own failure does not end the attempt: {error}"
    );
    let saved = owner.events.clone();
    let mut successor = Script::new(vec![turn(vec![text("I checked the current state first.")])]);
    let answer = recovered(
        request(vec![]),
        "successor",
        admitted(&saved),
        0,
        &[],
        None,
        recovery::Inputs::default(),
    )
    .run(
        &mut successor,
        &mut owner,
        limits(),
        &Cancellation::default(),
    )
    .await
    .expect("the later attempt continues from the saved result");
    assert_eq!(answer.text(), "I checked the current state first.");
    let [continued] = successor.requests.as_slice() else {
        panic!("one request answers the work");
    };
    let sent: Value = serde_json::from_str(sent_result(continued, "write")).unwrap();
    assert_eq!(
        sent,
        json!({"outcome":"unknown","error":"This call failed and its outcome is unknown. Check the current state before you repeat it."}),
        "the model is told the outcome is unknown, never that the call did not run"
    );
    assert_eq!(
        owner.started("write"),
        1,
        "the harness never runs it again, before or after the restart"
    );
}

#[tokio::test(start_paused = true)]
async fn a_save_that_fails_retryably_and_then_succeeds_keeps_the_event() {
    let mut provider = Script::new(vec![
        turn(vec![call("write", "save", "{}")]),
        turn(vec![text("Done.")]),
    ]);
    let mut owner = Owner {
        save_failures: 12,
        ..Owner::default()
    };
    let answer = fresh(request(vec![]), "save-retry")
        .run(
            &mut provider,
            &mut owner,
            limits(),
            &Cancellation::default(),
        )
        .await
        .expect("the save retries until it is acknowledged");
    assert_eq!(answer.text(), "Done.");
    let saved: Vec<_> = owner
        .events
        .iter()
        .filter(|event| matches!(event.kind, EventKind::ToolReturned { .. }))
        .map(|event| json!(event))
        .collect();
    let [saved] = saved.as_slice() else {
        panic!("the result is saved once");
    };
    assert_eq!(owner.unsaved.len(), 12);
    assert!(
        owner.unsaved.iter().all(|tried| tried == saved),
        "every try saves the exact event"
    );
    assert_eq!(owner.executed, 1, "the tool runs once");
    assert_eq!(provider.requests.len(), 2);
}

#[tokio::test(start_paused = true)]
async fn stop_during_a_save_retry_ends_it() {
    let mut provider = Script::new(vec![
        turn(vec![call("write", "save", "{}")]),
        turn(vec![text("unused")]),
    ]);
    let mut owner = Owner {
        save_failures: usize::MAX,
        ..Owner::default()
    };
    let cancel = Cancellation::default();
    let stop = cancel.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(500)).await;
        stop.cancel();
    });
    let error = fresh(request(vec![]), "save-stop")
        .run(&mut provider, &mut owner, limits(), &cancel)
        .await
        .unwrap_err();
    assert!(matches!(error, Failure::Cancelled), "{error}");
    assert!(!owner.unsaved.is_empty());
    assert_eq!(provider.requests.len(), 1);
}

#[tokio::test]
async fn a_tool_error_after_the_attempt_was_superseded_still_interrupts() {
    let mut provider = Script::new(vec![
        turn(vec![call("write", "save", "{}")]),
        turn(vec![text("unused")]),
    ]);
    let mut owner = Owner {
        tool_error: Some(ToolError::Superseding("lease lost")),
        ..Owner::default()
    };
    let error = fresh(request(vec![]), "superseded")
        .run(
            &mut provider,
            &mut owner,
            limits(),
            &Cancellation::default(),
        )
        .await
        .unwrap_err();
    assert!(
        matches!(error, Failure::NotCurrent(NotCurrent::Superseded)),
        "the newer input ends the attempt, not the tool's error: {error}"
    );
    assert_eq!(
        AttemptFailure::of(&error, &Cancellation::default()),
        AttemptFailure::Interrupted,
        "superseded work is interrupted, not a failed attempt"
    );
    assert_eq!(provider.requests.len(), 1);
    assert!(
        !owner
            .events
            .iter()
            .any(|event| matches!(event.kind, EventKind::ToolReturned { .. })),
        "no result is recorded for a call whose attempt is no longer current"
    );
}

#[tokio::test]
async fn an_unknown_tool_is_answered_with_the_tools_the_job_has() {
    let mut provider = Script::new(vec![
        turn(vec![call("guess", "send_message", "{}")]),
        turn(vec![text("Done.")]),
    ]);
    let mut owner = Owner::default();
    fresh(request(vec![]), "unknown-tool")
        .run(
            &mut provider,
            &mut owner,
            limits(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert!(
        owner.events.iter().any(|event| matches!(&event.kind,
            EventKind::ToolReturned { call_id, result: results::ToolResult::Refused(refusal) }
                if call_id == "guess"
                    && refusal.reason() == "Tool 'send_message' does not exist. Available tools: save.")),
        "the model learns which tools it can call"
    );
    assert_eq!(owner.executed, 0);
}

#[tokio::test]
async fn an_empty_tool_name_is_a_correctable_error_and_its_siblings_run() {
    let mut provider = Script::new(vec![
        turn(vec![call("blank", "", "{}"), call("good", "save", "{}")]),
        turn(vec![text("Done.")]),
    ]);
    let mut owner = Owner::default();
    let result = fresh(request(vec![]), "blank-name")
        .run(
            &mut provider,
            &mut owner,
            limits(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(result.text(), "Done.");
    assert_eq!(owner.executed, 1, "the valid sibling runs");
    assert!(
        owner.events.iter().any(|event| matches!(&event.kind,
            EventKind::ToolReturned { call_id, result: results::ToolResult::Refused(refusal) }
                if call_id == "blank" && refusal.reason().contains("empty"))),
        "the empty name is answered under the call's ID and says the name was empty"
    );
    assert!(
        !owner
            .events
            .iter()
            .any(|event| matches!(&event.kind, EventKind::ToolStarted(call) if call.id == "blank")),
        "a call with no name never starts"
    );
}

#[tokio::test]
async fn malformed_and_unknown_siblings_preserve_successful_result_and_order() {
    let calls = vec![
        call("malformed", "save", "{broken"),
        call("good", "save", " { \"revision\" : 7 } "),
        call("unknown", "not_registered", "{}"),
        call("refused", "save", "{}"),
    ];
    let mut provider = Script::new(vec![turn(calls.clone()), turn(vec![text("Done.")])]);
    let mut owner = Owner {
        tool_outcomes: VecDeque::from([
            ToolOutcome::Returned("saved".into()),
            ToolOutcome::refused("not authorized"),
        ]),
        ..Default::default()
    };
    let result = fresh(request(vec![]), "siblings")
        .run(
            &mut provider,
            &mut owner,
            limits(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(result.text(), "Done.");
    assert_eq!(owner.executed, 1);
    assert_eq!(&provider.requests[1].items[..4], calls.as_slice());
    let results: Vec<_> = provider.requests[1]
        .items
        .iter()
        .filter_map(|item| {
            if let Item::MaterializedResult { call_id, output } = item {
                Some((call_id.as_str(), output.as_str()))
            } else {
                None
            }
        })
        .collect();
    assert_eq!(results.len(), 4);
    assert_eq!(results[0].0, "malformed");
    assert_eq!(results[1], ("good", "saved"));
    assert_eq!(results[2].0, "unknown");
    assert!(results[0].1.contains("error"));
    assert!(results[2].1.contains("error"));
    assert_eq!(results[3].0, "refused");
    assert!(results[3].1.contains("not authorized"));
    assert!(owner.events.iter().any(|event| matches!(&event.kind, EventKind::ToolReturned { call_id, result: results::ToolResult::Refused(_) } if call_id == "refused")));
    let returned: Vec<_> = owner
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::ToolReturned { call_id, result } => Some((call_id.as_str(), result)),
            _ => None,
        })
        .collect();
    assert_eq!(
        returned.iter().map(|(id, _)| *id).collect::<Vec<_>>(),
        results.iter().map(|(id, _)| *id).collect::<Vec<_>>()
    );
    assert_eq!(
        returned[1].1,
        &results::ToolResult::Original("saved".into())
    );
    let warned = before_warning(results[3].1).expect("the warning follows the newest result");
    for (index, output) in [(0, results[0].1), (2, results[2].1), (3, warned)] {
        let results::ToolResult::Refused(refusal) = returned[index].1 else {
            panic!("a refused call keeps its typed refusal")
        };
        assert_eq!(
            serde_json::from_str::<Value>(output).unwrap()["error"],
            refusal.reason()
        );
    }
}

#[tokio::test]
async fn repeated_refusals_nudge_without_stopping_and_restore_without_double_counting() {
    let mut owner = Owner::default();
    let mut first = Script::new(vec![
        turn(vec![call("first", "unknown", r#"{"a":1,"b":2}"#)]),
        turn(vec![call("second", "unknown", r#"{ "b": 2, "a": 1 }"#)]),
    ]);
    first.fail_ready_at = Some(3);
    let mut allowance = limits();
    allowance.max_calls = 20;
    fresh(request(vec![]), "first-attempt")
        .run(&mut first, &mut owner, allowance, &Cancellation::default())
        .await
        .unwrap_err();
    let saved = owner.events.clone();
    let mut next = Script::new(vec![
        turn(vec![call("third", "unknown", r#"{"a":1,"b":2}"#)]),
        turn(vec![call("different", "unknown", r#"{"a":2}"#)]),
        turn(vec![call("success", "save", "{}")]),
        turn(vec![call("again-one", "unknown", r#"{"a":2}"#)]),
        turn(vec![call("again-two", "unknown", r#"{"a":2}"#)]),
        turn(vec![call("again-three", "unknown", r#"{"a":2}"#)]),
        turn(vec![text("I reassessed and can answer without that tool.")]),
    ]);
    let result = recovered(
        request(vec![]),
        "successor",
        admitted(&saved),
        0,
        &[],
        None,
        recovery::Inputs::default(),
    )
    .run(&mut next, &mut owner, allowance, &Cancellation::default())
    .await
    .unwrap();
    assert_eq!(
        result.text(),
        "I reassessed and can answer without that tool."
    );
    let final_request = next.requests.last().unwrap();
    for id in [
        "first",
        "second",
        "third",
        "different",
        "success",
        "again-one",
        "again-two",
        "again-three",
    ] {
        let output = final_request
            .items
            .iter()
            .find_map(|item| match item {
                Item::MaterializedResult { call_id, output } if call_id == id => Some(output),
                _ => None,
            })
            .unwrap();
        assert_eq!(
            output.contains("Do not repeat this call unchanged"),
            matches!(id, "third" | "again-three"),
            "guidance belongs only to the third consecutive identical refusal: {id}: {output}"
        );
    }
    assert_eq!(
        owner.executed, 1,
        "successful tool still executes after the nudge"
    );
    assert!(owner.events.iter().all(|event| !matches!(&event.kind,EventKind::ToolReturned{result: results::ToolResult::Original(output),..} if output.contains("Do not repeat this call unchanged"))),"durable receipts keep the original error");
}

#[tokio::test]
async fn currentness_rechecked_after_durable_intent() {
    let mut provider = Script::new(vec![turn(vec![call("c", "save", "{}")])]);
    let mut owner = Owner {
        revoke_after_start: true,
        ..Owner::default()
    };
    assert!(
        fresh(request(vec![]), "attempt")
            .run(
                &mut provider,
                &mut owner,
                limits(),
                &Cancellation::default(),
            )
            .await
            .is_err()
    );
    assert_eq!(owner.executed, 0);
}

#[tokio::test]
async fn uncertain_effect_is_never_replayed() {
    let cancel = Cancellation::default();
    let mut provider = Script::new(vec![turn(vec![
        text("Unpublished draft before steering"),
        call("c", "save", "{}"),
        call("unstarted-write", "save", "{}"),
    ])]);
    let mut owner = Owner {
        steer_on_tool: true,
        ..Owner::default()
    };
    let error = fresh(request(vec![]), "attempt")
        .run(&mut provider, &mut owner, limits(), &cancel)
        .await
        .unwrap_err();
    assert!(
        matches!(error, Failure::NotCurrent(NotCurrent::Superseded)),
        "{error}"
    );
    assert_eq!(owner.executed, 1);
    assert_eq!(
        owner
            .events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::ToolStarted(_)))
            .count(),
        1
    );
    assert_eq!(provider.requests.len(), 1);
    assert!(
        owner.events.iter().any(|event| matches!(&event.kind,
        EventKind::ToolReturned {call_id, result}
        if call_id == "c" && result.original() == Some("saved after steering"))),
        "a running call must finish and its result must be saved after steering"
    );
    let records = owner
        .events
        .iter()
        .map(|event| {
            if matches!(event.kind, EventKind::ToolReturned { .. }) {
                recipe::ReplayEvent::EvidenceOnly(event)
            } else {
                recipe::ReplayEvent::Admitted(event)
            }
        })
        .collect();
    let accepted = vec![AcceptedInput {
        id: "steer".into(),
        items: vec![Item::Message {
            role: "user".into(),
            text: "New direction".into(),
        }],
    }];
    let recovered = request(vec![])
        .freeze()
        .unwrap()
        .recover(
            "successor",
            recovery::Records {
                events: records,
                allowance_after: 0,
            },
            recovery::Messages {
                sources: &[],
                partial: None,
            },
            recovery::Inputs {
                accepted,
                source_feedback: None,
            },
        )
        .unwrap();
    let mut successor = Script::new(vec![turn(vec![text("Answer after recovery")])]);
    owner.currency = Currency::Current;
    recovered
        .run(
            &mut successor,
            &mut owner,
            limits(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    let sent = &successor.requests[0].items;
    let output = |id: &str| {
        let output = sent
            .iter()
            .find_map(|item| match item {
                Item::MaterializedResult { call_id, output } if call_id == id => Some(output),
                _ => None,
            })
            .unwrap();
        before_warning(output).unwrap_or(output)
    };
    assert_eq!(output("c"), "saved after steering");
    assert_eq!(
        serde_json::from_str::<Value>(output("unstarted-write")).unwrap()["outcome"],
        "not_run"
    );
    assert!(!sent.iter().any(|item| matches!(item, Item::Message {text, ..} if text == "Unpublished draft before steering")));
    let starts = |id: &str| {
        owner
            .events
            .iter()
            .filter(|event| matches!(&event.kind, EventKind::ToolStarted(call) if call.id == id))
            .count()
    };
    assert_eq!(starts("c"), 1, "a started call is never admitted again");
    assert_eq!(starts("unstarted-write"), 0);
    assert_eq!(owner.executed, 1);
    assert!(
        !owner
            .events
            .iter()
            .any(|e| e.attempt_id == "attempt" && matches!(e.kind, EventKind::Candidate { .. }))
    );
}

#[tokio::test]
async fn cancelled_before_start_and_provider_failure_never_retry() {
    let cancel = Cancellation::default();
    cancel.cancel();
    let mut provider = Script::new(vec![]);
    let mut owner = Owner::default();
    assert!(matches!(
        fresh(request(vec![]), "attempt")
            .run(&mut provider, &mut owner, limits(), &cancel)
            .await
            .unwrap_err(),
        Failure::Cancelled
    ));
    assert!(provider.requests.is_empty());
    provider.fail = true;
    let error = fresh(request(vec![]), "attempt")
        .run(
            &mut provider,
            &mut owner,
            limits(),
            &Cancellation::default(),
        )
        .await
        .unwrap_err();
    assert!(matches!(error, Failure::Provider(_)), "{error}");
    assert_eq!(provider.requests.len(), 1);
    assert!(
        matches!(&owner.events[owner.events.len()-2].kind, EventKind::ProviderObservation {usage,..} if usage.input_tokens.is_none() && usage.output_tokens.is_none())
    );
    assert!(
        matches!(owner.events.last().map(|event| &event.kind), Some(EventKind::ProviderFailed {reason}) if reason == "transport unknown")
    );
}

#[tokio::test]
async fn model_request_and_response_budgets_are_enforced() {
    let bounds = limits();
    // A large working window cannot authorize dispatch above the provider's input limit.
    let mut input_limited = request(vec![Item::Message {
        role: "user".into(),
        text: "x".repeat(400),
    }]);
    input_limited.max_input_tokens = 100;
    let mut provider = Script::new(vec![]);
    assert!(
        fresh(input_limited, "input-limit")
            .run(
                &mut provider,
                &mut Owner::default(),
                bounds,
                &Cancellation::default(),
            )
            .await
            .is_err()
    );
    assert!(provider.requests.is_empty());

    // Provider transport metadata does not consume the model's input window.
    // The shared 4 MiB transport/turn bound still refuses an oversized response.
    for (bytes, accepted) in [(1_100_000, true), (4_200_000, false)] {
        let mut response = turn(vec![text("Small model-visible answer.")]);
        response.receipt = json!({"provider_metadata":"x".repeat(bytes)});
        let mut provider = Script::new(vec![response]);
        let mut owner = Owner::default();
        let mut frozen = request(vec![]);
        frozen.context_window_tokens = 20_000;
        let result = fresh(frozen, "transport")
            .run(
                &mut provider,
                &mut owner,
                RunLimits::default(),
                &Cancellation::default(),
            )
            .await;
        assert_eq!(result.is_ok(), accepted);
        if let Err(error) = result {
            assert_eq!(
                harness::engine::AttemptFailure::of(
                    &error,
                    &harness::engine::Cancellation::default()
                ),
                harness::engine::AttemptFailure::Failed,
                "an operational byte bound must not become a run-limit hold"
            );
        }
        assert_eq!(owner.finished, usize::from(accepted));
    }
}

#[tokio::test]
async fn each_request_matches_calls_and_results_and_its_record_says_what_was_sent() {
    let user = |text: &str| Item::Message {
        role: "user".into(),
        text: text.into(),
    };
    let mut provider = Script::new(vec![
        turn(vec![call("next", "save", "{}")]),
        turn(vec![text("Done.")]),
    ]);
    let mut owner = Owner::default();
    fresh(
        request(vec![
            call("lost", "save", "{}"),
            user("Continue."),
            Item::Result {
                call_id: "orphan".into(),
                output: results::ResultView::inline("stray".into()),
            },
            user("And finish."),
        ]),
        "pairing",
    )
    .run(
        &mut provider,
        &mut owner,
        limits(),
        &Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(owner.executed, 1, "the interrupted call is never run again");
    let mut stand_ins = Vec::new();
    for sent in &provider.requests {
        let position = sent
            .items
            .iter()
            .position(|item| matches!(item, Item::Call(call) if call.id == "lost"))
            .unwrap();
        let Some(Item::MaterializedResult { call_id, output }) = sent.items.get(position + 1)
        else {
            panic!("the call without a result gets one right after it");
        };
        assert_eq!(call_id, "lost");
        stand_ins.push(output.clone());
        assert!(
            !sent.items.iter().any(|item| matches!(item,
                Item::MaterializedResult { call_id, .. } if call_id == "orphan")),
            "a result without a call is left out"
        );
    }
    assert_eq!(
        stand_ins[0], stand_ins[1],
        "later requests keep the same prefix"
    );
    let recorded = request_records::dispatched(&owner.events, &provider.requests[0]);
    for (saved, sent) in recorded.iter().zip(&provider.requests) {
        let calls_and_results = |items: &[Item]| {
            items
                .iter()
                .filter_map(|item| match item {
                    Item::Call(call) => Some(call.id.clone()),
                    Item::Result { call_id, .. } | Item::MaterializedResult { call_id, .. } => {
                        Some(format!("result {call_id}"))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(
            calls_and_results(&saved.items),
            calls_and_results(&sent.items),
            "the saved request says which calls and results were sent"
        );
    }
}

/// The window recipe with paging and the note tools both offered.
fn paging_window_request(items: Vec<Item>) -> Request {
    let mut frozen = window_request();
    frozen.tools.push(results::tool().function());
    frozen.items = items;
    frozen
}

#[tokio::test]
async fn a_result_without_a_call_is_left_out_without_failing_the_run() {
    let mut provider = Script::new(vec![turn(vec![text("Done.")])]);
    let mut owner = WindowOwner::default();
    fresh(
        paging_window_request(vec![
            Item::Message {
                role: "user".into(),
                text: "Continue.".into(),
            },
            Item::Result {
                call_id: "orphan".into(),
                output: results::ResultView::inline("stray".into()),
            },
        ]),
        "orphan",
    )
    .run(
        &mut provider,
        &mut owner,
        limits(),
        &Cancellation::default(),
    )
    .await
    .unwrap();
    let [sent] = provider.requests.as_slice() else {
        panic!("one request answers the work");
    };
    assert!(
        !sent.items.iter().any(|item| matches!(item,
            Item::MaterializedResult { call_id, .. } if call_id == "orphan")),
        "a result without a call is not sent"
    );
}

#[tokio::test]
async fn calls_without_results_reach_a_fresh_window_with_their_interrupted_results() {
    let real = "The real result of a.";
    let answered = Item::Result {
        call_id: "a".into(),
        output: results::ResultView::inline(real.into()),
    };
    // One call without a result, two, and a turn where only the first call
    // has its result. Each call keeps a real result or gets the interrupted one.
    for (calls, given, known) in [
        (vec!["a"], None, None),
        (vec!["a", "b"], None, None),
        (vec!["a", "b"], Some(answered), Some(("a", real))),
    ] {
        let current = Item::Message {
            role: "user".into(),
            text: "Continue the accepted question.".into(),
        };
        // Past the handoff point, so the run opens in a fresh window.
        let earlier = Item::assistant("x".repeat(320_000));
        let mut provider = Script::new(vec![turn(vec![text("Done.")])]);
        let mut owner = WindowOwner {
            seed: vec![current.clone()],
            ..Default::default()
        };
        let mut items = vec![current, earlier.clone()];
        items.extend(calls.iter().map(|id| call(id, "stable_read", "{}")));
        items.extend(given);
        fresh(paging_window_request(items), "interrupted")
            .run(
                &mut provider,
                &mut owner,
                limits(),
                &Cancellation::default(),
            )
            .await
            .unwrap();
        assert_eq!(window_reasons(&owner.events), [WindowReason::NearEnd]);
        let [sent] = provider.requests.as_slice() else {
            panic!("one request answers the work");
        };
        assert!(!sent.items.contains(&earlier), "the window is fresh");
        let call_at = |id: &str| {
            sent.items
                .iter()
                .position(|item| matches!(item, Item::Call(call) if call.id == id))
                .unwrap_or_else(|| panic!("{id} is pending work in the fresh window: {calls:?}"))
        };
        let result_at = |id: &str| {
            sent.items
                .iter()
                .position(|item| matches!(item, Item::MaterializedResult { call_id, .. } if call_id == id))
                .unwrap_or_else(|| panic!("{id} has a result in the fresh window: {calls:?}"))
        };
        // The shape of a normal turn: its calls, then one result for each in call order.
        let order: Vec<_> = calls
            .iter()
            .map(|id| call_at(id))
            .chain(calls.iter().map(|id| result_at(id)))
            .collect();
        assert!(order.is_sorted(), "{calls:?}: {order:?}");
        for id in &calls {
            let Item::MaterializedResult { output, .. } = &sent.items[result_at(id)] else {
                unreachable!()
            };
            match known {
                Some((answered, text)) if answered == *id => assert_eq!(output, text),
                _ => assert!(output.contains("outcome is unknown"), "{id}: {output}"),
            }
            assert_eq!(owner.effect_counts.get(*id), None, "the call never runs");
        }
    }
}

#[tokio::test]
async fn delivery_never_credits_a_result_left_out_of_the_request() {
    let original = "retained evidence the model never received";
    let earlier = [
        Event {
            attempt_id: "earlier".into(),
            sequence: 1,
            ordinal: 0,
            kind: EventKind::ToolStarted(Invocation {
                id: "read".into(),
                name: "stable_read".into(),
                arguments: "{}".into(),
            }),
        },
        Event {
            attempt_id: "earlier".into(),
            sequence: 2,
            ordinal: 0,
            kind: EventKind::ToolReturned {
                call_id: "read".into(),
                result: results::ToolResult::Original(original.into()),
            },
        },
    ];
    // The first result answers the call; the second one for the same call is
    // left out of the request.
    let frozen = paging_window_request(vec![
        call("read", "stable_read", "{}"),
        Item::Result {
            call_id: "read".into(),
            output: results::ResultView::inline("interrupted".into()),
        },
        Item::Result {
            call_id: "read".into(),
            output: results::ResultView::whole("read", original),
        },
        Item::Message {
            role: "user".into(),
            text: "Continue.".into(),
        },
    ]);
    let mut provider = Script::new(vec![turn(vec![text("Done.")])]);
    let mut owner = WindowOwner {
        events: earlier.to_vec(),
        ..Default::default()
    };
    recovered(
        frozen,
        "successor",
        admitted(&earlier),
        0,
        &[],
        None,
        recovery::Inputs::default(),
    )
    .run(
        &mut provider,
        &mut owner,
        limits(),
        &Cancellation::default(),
    )
    .await
    .unwrap();
    assert!(
        !provider.requests[0].items.iter().any(
            |item| matches!(item, Item::MaterializedResult { output, .. } if output == original)
        ),
        "the second result is left out of the request"
    );
    let delivery = results::Delivery::from_events(admitted(&owner.events), &[]).unwrap();
    assert!(
        !delivery.covers("read", None),
        "delivery credits only what the request carried"
    );
}

#[tokio::test]
async fn recovered_completion_delivers_the_products_decision() {
    let mut owner = Owner {
        finish_outcomes: vec![Finish::Continue(json!("saved draft changed"))].into(),
        ..Default::default()
    };
    let response = turn(vec![text("Ready for the product to publish.")]);
    let request = request(vec![]);
    let events = [
        Event {
            attempt_id: "old".into(),
            sequence: 1,
            ordinal: 1,
            kind: EventKind::ProviderStarted {
                dispatched_at_unix_ms: 1,
                request: request_records::saved("old", 1, &request),
                context: Default::default(),
            },
        },
        Event {
            attempt_id: "old".into(),
            sequence: 2,
            ordinal: 1,
            kind: EventKind::ProviderReturned(response),
        },
    ];
    let mut provider = Script::new(vec![]);
    provider.fail = true;
    let error = recovered(
        request,
        "successor",
        admitted(&events),
        0,
        &[],
        None,
        recovery::Inputs::default(),
    )
    .run(
        &mut provider,
        &mut owner,
        RunLimits::default(),
        &Cancellation::default(),
    )
    .await
    .unwrap_err();
    assert!(matches!(error, Failure::Provider(_)));
    assert!(provider.requests[0].items.iter().any(
        |item| matches!(item, Item::Message { text, .. } if text.contains("saved draft changed"))
    ));
    assert_eq!(owner.executed, 0);
    assert!(matches!(
        &owner.events[0],
        Event {
            ordinal: 1,
            sequence: 1,
            kind: EventKind::Candidate { .. },
            ..
        }
    ));
}

#[tokio::test]
async fn recovered_reserved_final_is_retained_partial_not_published_complete() {
    let mut final_request = request(vec![]);
    final_request.items.push(Item::Message {
        role: "developer".into(),
        text: FINAL_REQUEST.into(),
    });
    let response = turn(vec![
        text("Honest retained partial."),
        call("ignored", "save", "bad arguments"),
    ]);
    let events = vec![
        Event {
            attempt_id: "attempt".into(),
            sequence: 2,
            ordinal: 1,
            kind: EventKind::ProviderStarted {
                dispatched_at_unix_ms: 1,
                request: request_records::saved("attempt", 2, &final_request),
                context: DispatchContext {
                    window_stage: WindowStage::Accumulated,
                    usage: Default::default(),
                    carried_inputs: vec![],
                    purpose: RequestPurpose::Final {
                        backstops: harness::engine::ReachedBackstops::new(vec![
                            LimitBackstop::Requests,
                        ])
                        .unwrap(),
                    },
                    warning_added: false,
                },
            },
        },
        Event {
            attempt_id: "attempt".into(),
            sequence: 3,
            ordinal: 1,
            kind: EventKind::ProviderReturned(response.clone()),
        },
    ];
    let pending =
        harness::engine::pending_final(&final_request.binding(), admitted(&events), 0, &[])
            .unwrap()
            .expect("pending final");
    assert_eq!(pending.request, final_request);
    assert_eq!(pending.turn.items, [text("Honest retained partial.")]);
    assert_eq!(*pending.backstops, [LimitBackstop::Requests]);
    let mut empty = events.clone();
    empty[1].kind = EventKind::ProviderReturned(turn(Vec::new()));
    assert!(
        harness::engine::pending_final(&final_request.binding(), admitted(&empty), 0, &[])
            .unwrap()
            .unwrap()
            .turn
            .items
            .is_empty()
    );
    let mut tool_call = events.clone();
    tool_call[1].kind =
        EventKind::ProviderReturned(turn(vec![call("invalid-final", "save", "{}")]));
    assert!(
        harness::engine::pending_final(&final_request.binding(), admitted(&tool_call), 0, &[])
            .unwrap()
            .unwrap()
            .turn
            .items
            .is_empty()
    );
    let mut owner = Owner::default();
    let result = recovered(
        final_request,
        "successor",
        admitted(&events),
        0,
        &[],
        None,
        recovery::Inputs::default(),
    )
    .run(
        &mut Script::new(vec![]),
        &mut owner,
        RunLimits::default(),
        &Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(result, pending.turn);
    assert_eq!(owner.executed, 0);
    assert_eq!(owner.finished, 0);
    assert_eq!(owner.partial, [LimitBackstop::Requests]);
}

#[test]
fn saved_work_distinguishes_absent_state_from_unknown_provider_usage() {
    let start = EventKind::ProviderStarted {
        dispatched_at_unix_ms: 1,
        request: request_records::saved("attempt", 1, &request(Vec::new())),
        context: Default::default(),
    };
    let mut value = serde_json::to_value(&start).unwrap();
    assert!(value["ProviderStarted"].get("context").is_some());
    value["ProviderStarted"]
        .as_object_mut()
        .unwrap()
        .remove("context");
    assert!(serde_json::from_value::<EventKind>(value).is_err());

    let mut progress = serde_json::to_value(harness::engine::recipe::Progress::default()).unwrap();
    progress.as_object_mut().unwrap().remove("budget_tokens");
    assert!(serde_json::from_value::<harness::engine::recipe::Progress>(progress).is_err());
    let unknown: Turn = serde_json::from_value(json!({"items":[],"receipt":null})).unwrap();
    assert_eq!(unknown.usage, harness::engine::recipe::Usage::default());
}

#[tokio::test]
async fn reported_context_adds_new_results_and_resets_its_estimate_after_handoff() {
    let mut first = turn(vec![call("first", "stable_read", "{}")]);
    first.usage = Usage {
        cached_input_tokens: None,
        input_tokens: Some(164_150),
        output_tokens: Some(100),
    };
    let mut second = turn(vec![call("second", "stable_read", "{}")]);
    second.usage = Usage {
        cached_input_tokens: None,
        input_tokens: Some(164_650),
        output_tokens: Some(100),
    };
    let note = "Both exact results are retained; finish the accepted assignment.";
    let mut provider = Script::new(vec![
        first,
        second,
        turn(vec![call(
            "checkpoint",
            "save_working_note",
            &json!({"text":note,"fresh_window":true}).to_string(),
        )]),
        turn(vec![text("Completed after the model-sized crossing.")]),
    ]);
    let mut owner = WindowOwner {
        seed: vec![Item::Message {
            role: "user".into(),
            text: "Exact accepted assignment".into(),
        }],
        stable_output: Some("x".repeat(2_000)),
        ..Default::default()
    };
    let mut frozen = window_request();
    frozen.context_window_tokens = 200_000;
    frozen.max_input_tokens = 200_000;
    fresh(frozen, "context")
        .run(
            &mut provider,
            &mut owner,
            RunLimits {
                max_calls: 8,
                ..limits()
            },
            &Cancellation::default(),
        )
        .await
        .unwrap();
    let contexts: Vec<_> = owner
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::ProviderStarted { context, .. } => Some(context),
            _ => None,
        })
        .collect();
    assert_eq!(contexts.len(), 4);
    assert_eq!(contexts[0].usage.reported_tokens, None);
    assert!(contexts[0].usage.input_tokens > 0);
    assert_eq!(contexts[1].usage.reported_tokens, Some(164_250));
    assert!((164_750..165_250).contains(&contexts[1].usage.input_tokens));
    assert_eq!(contexts[1].purpose, RequestPurpose::Work);
    assert_eq!(contexts[2].usage.reported_tokens, Some(164_750));
    assert!((165_250..165_750).contains(&contexts[2].usage.input_tokens));
    assert_eq!(contexts[2].purpose, RequestPurpose::Handoff);
    assert_eq!(contexts[3].usage.reported_tokens, None);
    assert!(contexts[3].usage.input_tokens > 0 && contexts[3].usage.input_tokens < 9_000);
    assert!(
        contexts
            .iter()
            .all(|context| context.usage.reply_reserve == 16_384)
    );
    assert!(provider.requests[3].items.contains(&Item::assistant(note)));
    assert!(provider.requests[3].items.contains(&Item::Message {
        role: "user".into(),
        text: "Exact accepted assignment".into()
    }));
}

#[tokio::test]
async fn requested_pages_share_the_turn_bound_without_becoming_previews() {
    for max_calls in [8, 4] {
        let original = "é中".repeat(90_000);
        let mut owner = WindowOwner {
            stable_outputs: BTreeMap::from([
                ("source".into(), original.clone()),
                ("nonread".into(), "ordinary".repeat(7_000)),
            ]),
            history_messages: vec![history::Message {
                id: "message".into(),
                role: history::Role::User,
                text: original.clone(),
                metadata: json!({}),
            }],
            ..Default::default()
        };
        let mut frozen = window_request();
        frozen.context_window_tokens = 100_000;
        frozen.tools.push(results::tool().function());
        frozen
            .tools
            .extend(history::tools().into_iter().map(|tool| tool.function()));
        let mut provider = Script::new(vec![
            turn(vec![call("source", "stable_read", "{}")]),
            turn(vec![
                call(
                    "page-0",
                    results::READ,
                    &json!({"call_id":"source", "offset":0, "limit":60_000}).to_string(),
                ),
                call(
                    "page-2",
                    history::READ,
                    &json!({"id":"message", "offset_chars":120_000, "limit_chars":25_000})
                        .to_string(),
                ),
                call("nonread", "stable_read", "{}"),
                call(
                    "page-1",
                    results::READ,
                    &json!({"call_id":"source", "offset":150_000, "limit":55_000}).to_string(),
                ),
            ]),
            turn(vec![text("Read the requested pages.")]),
        ]);
        let sources = [ProductMessage {
            id: "message".into(),
            role: harness::engine::history::Role::User,
            text: original.clone(),
        }];
        let work = recovered(
            frozen,
            "attempt",
            admitted(&owner.events),
            0,
            &sources,
            None,
            recovery::Inputs::default(),
        );
        work.run(
            &mut provider,
            &mut owner,
            RunLimits {
                max_calls,
                ..limits()
            },
            &Cancellation::default(),
        )
        .await
        .unwrap();
        let outputs = provider.requests[2]
            .items
            .iter()
            .filter_map(|item| match item {
                Item::MaterializedResult { call_id, output }
                    if call_id.starts_with("page-") || call_id == "nonread" =>
                {
                    Some((call_id.as_str(), output.as_str()))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            outputs.iter().map(|(id, _)| *id).collect::<Vec<_>>(),
            ["page-0", "page-2", "nonread", "page-1"]
        );
        let outputs = outputs.into_iter().collect::<BTreeMap<_, _>>();
        assert!(
            outputs
                .values()
                .map(|output| output.chars().count())
                .sum::<usize>()
                <= 120_000
        );
        let nonread: Value = serde_json::from_str(outputs["nonread"]).unwrap();
        assert!(
            nonread.get("tail").is_some(),
            "the non-read must become a preview first"
        );
        assert!(outputs["nonread"].chars().count() <= 1500);
        let small: Value = serde_json::from_str(outputs["page-2"]).unwrap();
        let small = &small["items"][0];
        assert_eq!(
            small["text"].as_str().unwrap().chars().count(),
            25_000,
            "a page below the shared final length stays whole"
        );
        assert_eq!(
            small["text"],
            original
                .chars()
                .skip(120_000)
                .take(25_000)
                .collect::<String>()
        );
        assert_eq!(small["offset_chars"], 120_000);
        assert_eq!(small["next_offset_chars"], 145_000);
        assert_eq!(
            outputs["page-0"].chars().count(),
            outputs["page-1"].chars().count(),
            "large pages shorten to the same final length"
        );
        for (id, offset) in [("page-0", 0), ("page-1", 150_000)] {
            let warned = before_warning(outputs[id]);
            assert_eq!(warned.is_some(), max_calls == 4 && id == "page-1");
            let page: Value = serde_json::from_str(warned.unwrap_or(outputs[id])).unwrap();
            let text = page["text"].as_str().unwrap();
            assert!((40_000..50_000).contains(&text.chars().count()));
            assert!(
                page.get("tail").is_none(),
                "a requested read is never a preview"
            );
            assert_eq!(page["offset"], offset);
            assert_eq!(page["next_offset"], offset + text.len());
            assert_eq!(text, &original[offset..offset + text.len()]);
        }
        assert_eq!(owner.effect_counts.get("source"), Some(&1));
    }
}

#[tokio::test]
async fn deleted_history_messages_leave_no_copied_text_and_resume_with_changed_sources() {
    for (name, arguments) in [
        (history::READ, json!({"id":"private-message"})),
        (history::LIST, json!({})),
        (history::SEARCH, json!({"text":"private"})),
    ] {
        let bounds = RunLimits {
            max_calls: 8,
            ..limits()
        };
        let private = "Unique private patient words 🪕 that must disappear";
        let excerpt = "patient words 🪕 that must disappear";
        let message = ProductMessage {
            id: "private-message".into(),
            role: harness::engine::history::Role::User,
            text: private.into(),
        };
        let mut owner = WindowOwner {
            history_messages: vec![history::Message {
                id: message.id.clone(),
                role: history::Role::User,
                text: message.text.clone(),
                metadata: Value::Null,
            }],
            ..Default::default()
        };
        let mut frozen = window_request();
        frozen
            .tools
            .extend(history::tools().into_iter().map(|tool| tool.function()));
        frozen.tools.push(results::tool().function());
        let work = recovered(
            frozen.clone(),
            "before-deletion",
            admitted(&owner.events),
            0,
            std::slice::from_ref(&message),
            None,
            recovery::Inputs::default(),
        );
        let mut first = Script::new(vec![
            turn(vec![call("history-read", name, &arguments.to_string())]),
            turn(vec![
                call(
                    "nested-page",
                    results::READ,
                    &json!({"call_id":"history-read"}).to_string(),
                ),
                call(
                    "nested-directory",
                    history::LIST,
                    &json!({"tool_name":name,"max_chars":2000}).to_string(),
                ),
            ]),
        ]);
        first.fail_at = Some(3);
        let failed = work
            .run(&mut first, &mut owner, bounds, &Cancellation::default())
            .await;
        assert!(failed.is_err());
        assert!(first.requests[2].items.iter().any(|item|
            matches!(item, Item::MaterializedResult { output, .. } if output.contains(excerpt))), "{name}: {:?}", first.requests[2].items);
        owner.history_messages.clear();
        let recorded = serde_json::to_string(&owner.events).unwrap();
        assert!(
            !recorded.contains(excerpt),
            "history receipts must retain references, not private message text"
        );
        let resumed = recovered(
            frozen,
            "after-deletion",
            admitted(&owner.events),
            0,
            &[],
            None,
            recovery::Inputs::default(),
        );
        let mut second = Script::new(vec![turn(vec![text(
            "Continued with the remaining sources.",
        )])]);
        resumed
            .run(&mut second, &mut owner, bounds, &Cancellation::default())
            .await
            .unwrap();
        assert_ne!(first.requests[2].items, second.requests[0].items);
        assert!(second.requests[0].items.iter().any(
            |item| matches!(item, Item::Message { role, text } if role == "developer" && text.contains("changed_sources"))
        ));
        assert!(second.requests[0].items.iter().any(|item|
            matches!(item, Item::MaterializedResult { output, .. } if output.contains("source_unavailable"))));
        assert_eq!(
            second.requests[0].instructions,
            first.requests[2].instructions
        );
        assert_eq!(second.requests[0].tools, first.requests[2].tools);
        assert!(
            !serde_json::to_string(&second.requests[0])
                .unwrap()
                .contains(excerpt)
        );
        assert_eq!(
            owner.effect_counts["history-read"], 1,
            "a deleted read is not executed again"
        );
        assert_eq!(owner.completion_count, 1);
    }
}

#[tokio::test]
async fn a_saved_cut_off_turn_recovers_after_its_sources_changed() {
    let message = ProductMessage {
        id: "question".into(),
        role: harness::engine::history::Role::User,
        text: "Original question".into(),
    };
    let frozen = request(vec![Item::ProductMessage(message.clone())]);
    let mut owner = Owner::default();
    let mut first = Script::new(vec![turn(vec![Item::OutputCutoff])]);
    first.fail_at = Some(2);
    let failed = recovered(
        frozen.clone(),
        "before-crash",
        admitted(&owner.events),
        0,
        std::slice::from_ref(&message),
        None,
        recovery::Inputs::default(),
    )
    .run(&mut first, &mut owner, limits(), &Cancellation::default())
    .await;
    assert!(failed.is_err());
    // The process stopped right after the cut-off turn was saved.
    let returned = owner
        .events
        .iter()
        .position(|event| matches!(&event.kind, EventKind::ProviderReturned(turn) if turn.items == [Item::OutputCutoff]))
        .unwrap();
    owner.events.truncate(returned + 1);
    // The question was deleted before recovery.
    let mut successor = Script::new(vec![turn(vec![text("Answer from the current sources.")])]);
    let result = recovered(
        frozen,
        "after-crash",
        admitted(&owner.events),
        0,
        &[],
        None,
        recovery::Inputs::default(),
    )
    .run(
        &mut successor,
        &mut owner,
        limits(),
        &Cancellation::default(),
    )
    .await;
    assert!(
        result.is_ok(),
        "a cut-off turn must not stop recovery after its sources changed: {result:?}"
    );
    assert_eq!(successor.requests.len(), 1);
    assert!(successor.requests[0].items.iter().any(
        |item| matches!(item, Item::Message { role, text } if role == "developer" && text.contains("changed_sources"))
    ));
}

#[tokio::test]
async fn context_meter_counts_the_encoded_result_view() {
    let mut owner = WindowOwner {
        stable_output: Some("🪕\"\n".repeat(30_000)),
        ..Default::default()
    };
    let mut provider = Script::new(vec![
        turn(vec![call("read", "stable_read", "{}")]),
        turn(vec![text("Done")]),
    ]);
    let mut frozen = window_request();
    frozen.tools.push(results::tool().function());
    fresh(frozen, "encoded")
        .run(
            &mut provider,
            &mut owner,
            limits(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    let context = owner
        .events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::ProviderStarted { context, .. } => Some(context),
            _ => None,
        })
        .nth(1)
        .unwrap();
    assert_eq!(
        context.usage.input_tokens,
        context::estimate_request(&provider.requests[1]).unwrap(),
        "meter must count the encoded view delivered to the model"
    );
}

#[tokio::test]
async fn budget_warning_fits_inside_the_preview_limit() {
    let mut owner = WindowOwner {
        stable_output: Some("x".repeat(70_000)),
        ..Default::default()
    };
    let mut provider = Script::new(vec![
        turn(vec![call("read", "stable_read", "{}")]),
        turn(vec![text("Done")]),
    ]);
    let mut frozen = window_request();
    frozen.tools.push(results::tool().function());
    fresh(frozen, "preview")
        .run(
            &mut provider,
            &mut owner,
            limits(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    let output = provider.requests[1]
        .items
        .iter()
        .find_map(|item| match item {
            Item::MaterializedResult { output, .. } => Some(output),
            _ => None,
        })
        .unwrap();
    assert!(
        before_warning(output).is_some(),
        "fixture must carry the pending warning"
    );
    assert!(
        output.chars().count() <= 1_500,
        "preview including warning exceeds 1500 characters: {}",
        output.chars().count()
    );
}

#[tokio::test]
async fn source_refresh_preserves_delivered_results_and_the_new_batch_boundary() {
    let message = ProductMessage {
        id: "deleted-message".into(),
        role: harness::engine::history::Role::User,
        text: "Private text to remove".into(),
    };
    let mut owner = WindowOwner {
        stable_output: Some("e".repeat(70_000)),
        source_updates: VecDeque::from([
            vec![message.clone()],
            vec![message.clone()],
            vec![message.clone()],
            vec![],
        ]),
        ..Default::default()
    };
    let mut frozen = window_request();
    frozen.context_window_tokens = 200_000;
    frozen.max_input_tokens = 200_000;
    frozen
        .items
        .insert(0, Item::ProductMessage(message.clone()));
    frozen.tools.push(results::tool().function());
    let work = recovered(
        frozen,
        "live-source-change",
        std::iter::empty::<recipe::ReplayEvent>(),
        0,
        std::slice::from_ref(&message),
        None,
        recovery::Inputs::default(),
    );
    let mut provider = Script::new(vec![
        turn(vec![call("first", "stable_read", "{}")]),
        turn(vec![call("second", "stable_read", "{}")]),
        turn(vec![call("third", "stable_read", "{}")]),
        turn(vec![text("Complete")]),
    ]);
    work.run(
        &mut provider,
        &mut owner,
        RunLimits {
            max_calls: 8,
            ..limits()
        },
        &Cancellation::default(),
    )
    .await
    .unwrap();
    let result = |request: &Request, id: &str| {
        request
            .items
            .iter()
            .find_map(|item| match item {
                Item::MaterializedResult { call_id, output } if call_id == id => {
                    Some(output.clone())
                }
                _ => None,
            })
            .unwrap()
    };
    for id in ["first", "second"] {
        assert!(
            result(&provider.requests[3], id) == result(&provider.requests[2], id),
            "source refresh must preserve already delivered result bytes"
        );
    }
    assert_eq!(result(&provider.requests[3], "third"), "e".repeat(70_000));
    let last = serde_json::to_string(&provider.requests[3]).unwrap();
    assert!(!last.contains("Private text to remove"));
    assert!(last.contains("changed_sources"));
    assert!(
        last.contains("source unavailable"),
        "deleted source keeps its place with an unavailable marker"
    );
}

#[tokio::test]
async fn handoff_measures_the_bounded_batch_sent_to_the_provider() {
    let mut owner = WindowOwner {
        stable_output: Some("e".repeat(50_000)),
        ..Default::default()
    };
    let mut provider = Script::new(vec![
        turn(
            (0..8)
                .map(|n| call(&format!("read-{n}"), "stable_read", "{}"))
                .collect(),
        ),
        turn(vec![text("Done")]),
    ]);
    let mut frozen = window_request();
    frozen.context_window_tokens = 100_000;
    frozen.max_input_tokens = 100_000;
    frozen.tools.push(results::tool().function());
    fresh(frozen, "bounded-handoff")
        .run(
            &mut provider,
            &mut owner,
            RunLimits {
                max_calls: 8,
                ..limits()
            },
            &Cancellation::default(),
        )
        .await
        .unwrap();
    let sent = &provider.requests[1];
    let threshold =
        context::input_capacity(sent.context_window_tokens, sent.max_input_tokens).unwrap() * 90
            / 100;
    assert!(context::estimate_request(sent).unwrap() < threshold);
    assert!(
        !sent.items.iter().any(|item| matches!(item,
        Item::Message { text, .. } if is_handoff_ask(text))),
        "a bounded batch below 90% must not request handoff"
    );
}

#[test]
fn requested_cancellation_precedes_other_attempt_end_causes() {
    use harness::engine::AttemptFailure;
    let cancel = Cancellation::default();
    for (error, expected) in [
        (
            Failure::Host("database refused settlement".into()),
            AttemptFailure::Failed,
        ),
        (
            Failure::Provider("provider failed".into()),
            AttemptFailure::Failed,
        ),
        (Failure::Cancelled, AttemptFailure::Interrupted),
    ] {
        assert_eq!(AttemptFailure::of(&error, &cancel), expected);
        let requested = Cancellation::default();
        requested.cancel();
        assert_eq!(
            AttemptFailure::of(&error, &requested),
            AttemptFailure::Interrupted
        );
    }
}

#[tokio::test]
async fn later_request_handoff_uses_the_small_window_boundary() {
    for (window, input_limit) in [(100_000, 100_000), (100_000, 50_000)] {
        let capacity = context::input_capacity(window, input_limit).unwrap();
        let threshold = capacity * 90 / 100;
        for offset in [-1_i64, 0, 1] {
            let mut frozen = window_request();
            frozen.context_window_tokens = window;
            frozen.max_input_tokens = input_limit;
            let mut after_read = frozen.clone();
            let read = call("read", "stable_read", "{}");
            after_read.items.extend([
                read.clone(),
                Item::MaterializedResult {
                    call_id: "read".into(),
                    output: "evidence".into(),
                },
                Item::Message {
                    role: "developer".into(),
                    text: handoff_ask(capacity - threshold),
                },
            ]);
            let added = context::estimate_request(&after_read).unwrap()
                - context::estimate_request(&frozen).unwrap();
            let mut response = turn(vec![read]);
            response.usage.input_tokens = Some(threshold.saturating_add_signed(offset) - added);
            response.usage.output_tokens = None;
            let mut provider = Script::new(vec![response, turn(vec![text("Done")])]);
            let mut owner = WindowOwner {
                stable_output: Some("evidence".into()),
                ..Default::default()
            };
            fresh(frozen, "small-window")
                .run(
                    &mut provider,
                    &mut owner,
                    RunLimits {
                        max_calls: 8,
                        ..limits()
                    },
                    &Cancellation::default(),
                )
                .await
                .unwrap();
            let handoff = provider.requests[1]
                .items
                .iter()
                .any(|item| matches!(item, Item::Message { text, .. } if is_handoff_ask(text)));
            assert_eq!(
                handoff,
                offset >= 0,
                "handoff boundary {threshold} at offset {offset}"
            );
        }
    }
}

#[tokio::test]
async fn a_refusal_delivered_with_the_limit_warning_acknowledges_the_correction() {
    let refusal = harness::engine::recipe::Refusal::new("Read the current source.");
    let mut owner = Owner {
        tool_outcomes: vec![ToolOutcome::Refused(refusal.clone())].into(),
        ..Default::default()
    };
    let mut script = Script::new(vec![
        turn(vec![call("refused-write", "save", "{}")]),
        turn(vec![text("Done")]),
    ]);
    fresh(request(vec![]), "refusal-warning")
        .run(&mut script, &mut owner, limits(), &Cancellation::default())
        .await
        .unwrap();
    let delivered = script.requests[1]
        .items
        .iter()
        .find_map(|item| match item {
            Item::MaterializedResult { call_id, output } if call_id == "refused-write" => {
                Some(output)
            }
            _ => None,
        })
        .unwrap();
    assert!(
        before_warning(delivered).is_some(),
        "the near-limit warning rides on the newest result, here the refusal"
    );
    assert!(
        results::Delivery::from_events(admitted(&owner.events), &[])
            .unwrap()
            .received_refusal(&refusal),
        "a refusal that reached a completed exchange acknowledges the correction"
    );
    let returned = owner
        .events
        .iter()
        .rposition(|event| matches!(event.kind, EventKind::ProviderReturned(_)))
        .unwrap();
    let mut failed = owner.events[..returned].to_vec();
    failed.push(Event {
        kind: EventKind::ProviderFailed {
            reason: "provider failed".into(),
        },
        ..owner.events[returned].clone()
    });
    assert!(
        !results::Delivery::from_events(admitted(&failed), &[])
            .unwrap()
            .received_refusal(&refusal),
        "a failed dispatch cannot acknowledge the correction"
    );
    let mut later = owner.events.clone();
    later.push(Event {
        attempt_id: "refusal-warning".into(),
        sequence: later.len() as u64 + 1,
        ordinal: later.last().unwrap().ordinal,
        kind: EventKind::WindowOpened {
            opened_at_unix_ms: 0,
            reason: WindowReason::ModelAsked,
            handoff: None,
        },
    });
    assert!(
        !results::Delivery::from_events(admitted(&later), &[])
            .unwrap()
            .received_refusal(&refusal),
        "only the latest window counts"
    );
    assert!(
        !results::Delivery::from_events(admitted(&owner.events), &[])
            .unwrap()
            .received_refusal(&harness::engine::recipe::Refusal::new(
                "A different correction."
            )),
        "another refusal was not delivered"
    );
}

#[tokio::test]
async fn tool_and_finish_refusals_use_one_feedback_shape() {
    let refusal = harness::engine::recipe::Refusal::new("Read the current source.");
    let mut owner = Owner {
        tool_outcomes: vec![ToolOutcome::Refused(refusal.clone())].into(),
        finish_outcomes: vec![Finish::Refused(refusal.clone())].into(),
        ..Default::default()
    };
    let mut script = Script::new(vec![
        turn(vec![call("refused-write", "save", "{}")]),
        turn(vec![text("Before correction")]),
        turn(vec![text("Reconsidered")]),
    ]);
    let mut allowance = limits();
    allowance.max_calls = 20;
    fresh(request(vec![]), "refusal-shape")
        .run(&mut script, &mut owner, allowance, &Cancellation::default())
        .await
        .unwrap();
    let output = script
        .requests
        .last()
        .unwrap()
        .items
        .iter()
        .find_map(|item| match item {
            Item::MaterializedResult { call_id, output } if call_id == "refused-write" => {
                Some(serde_json::from_str::<Value>(output).unwrap())
            }
            _ => None,
        })
        .unwrap();
    assert_eq!(
        output,
        json!({"executed":false,"error":"Read the current source."}),
        "the canonical refusal shape"
    );
    assert!(owner.events.iter().any(|event| matches!(&event.kind, EventKind::ToolReturned {call_id,result: results::ToolResult::Refused(saved)} if call_id=="refused-write" && saved == &refusal)));
    assert!(script.requests.last().unwrap().items.iter().any(|item| matches!(item, Item::Message {role,text} if role=="developer" && serde_json::from_str::<Value>(text).is_ok_and(|feedback| feedback==output))), "finish and tool refusals have the same shape");
}

const GUIDANCE: &str = "This work can outlast one context window.";

/// Model-facing text of every developer message in one request.
fn developer_text(request: &Request) -> Vec<&str> {
    request
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Message { role, text } if role == "developer" => Some(text.as_str()),
            _ => None,
        })
        .collect()
}

/// Dispatch contexts in provider order, as the run record saved them.
fn dispatches(events: &[Event]) -> Vec<&DispatchContext> {
    events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::ProviderStarted { context, .. } => Some(context),
            _ => None,
        })
        .collect()
}

fn window_reasons(events: &[Event]) -> Vec<WindowReason> {
    events
        .iter()
        .filter_map(|event| match &event.kind {
            EventKind::WindowOpened { reason, .. } => Some(*reason),
            _ => None,
        })
        .collect()
}

#[test]
fn offering_working_notes_adds_the_tools_with_their_guidance_once() {
    let mut plain = request(vec![]);
    let owner = plain.instructions.clone();
    plain.tools.push(results::tool().function());
    plain
        .tools
        .extend(history::tools().iter().map(Tool::function));
    let mut offered = plain.clone();
    offer_working_notes(&mut offered.instructions, &mut offered.tools);
    assert!(offered.instructions.starts_with(&owner));
    assert_eq!(offered.instructions.matches(GUIDANCE).count(), 1);
    for name in ["save_working_note", "read_working_note"] {
        assert!(offered.tools.iter().any(|tool| tool["name"] == name));
        assert!(!plain.tools.iter().any(|tool| tool["name"] == name));
    }
    assert!(
        offered.instructions.contains(history::LIST)
            && offered.instructions.contains(results::READ),
        "the guidance names the recovery tools the recipe offers"
    );
    assert!(!plain.instructions.contains(GUIDANCE));
    let mut bare = request(vec![]);
    offer_working_notes(&mut bare.instructions, &mut bare.tools);
    assert!(
        !bare.instructions.contains(history::LIST) && !bare.instructions.contains(results::READ),
        "the guidance never names a tool the recipe does not offer"
    );
}

#[tokio::test]
async fn a_turn_larger_than_the_room_left_is_previewed_and_the_window_is_asked_to_close() {
    let big = |n: usize| (format!("big-{n}"), format!("{n}{}", "r".repeat(20_000)));
    let outputs: BTreeMap<_, _> = (0..4).map(big).collect();
    let mut owner = WindowOwner {
        stable_outputs: outputs.clone(),
        ..Default::default()
    };
    let mut first = turn(vec![call("first", "stable_read", "{}")]);
    first.usage = Usage {
        cached_input_tokens: None,
        input_tokens: Some(70_000),
        output_tokens: Some(100),
    };
    let mut provider = Script::new(vec![
        first,
        turn(
            outputs
                .keys()
                .map(|id| call(id, "stable_read", "{}"))
                .collect(),
        ),
        turn(vec![text("Done.")]),
    ]);
    let mut frozen = window_request();
    frozen.tools.push(results::tool().function());
    let limit =
        context::input_capacity(frozen.context_window_tokens, frozen.max_input_tokens).unwrap();
    fresh(frozen, "jump")
        .run(
            &mut provider,
            &mut owner,
            RunLimits {
                max_calls: 8,
                ..limits()
            },
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert!(
        window_reasons(&owner.events).is_empty(),
        "previews keep the request in this window, so no window opens"
    );
    let contexts = dispatches(&owner.events);
    assert_eq!(contexts[2].purpose, RequestPurpose::Handoff);
    assert!(contexts[2].usage.input_tokens <= limit);
    let sent = &provider.requests[2];
    let ask = developer_text(sent)
        .into_iter()
        .find(|text| text.starts_with("This context window is nearly full"))
        .expect("the request near the end asks to answer or save");
    let left: u64 = ask
        .split("about ")
        .nth(1)
        .and_then(|rest| rest.split(' ').next())
        .and_then(|count| count.parse().ok())
        .expect("the ask gives the tokens left");
    let measured_left = limit - contexts[2].usage.input_tokens;
    assert!(
        left <= measured_left && measured_left - left <= 1,
        "the ask gives the input left: {left} of {measured_left}"
    );
    let shown: Vec<_> = sent
        .items
        .iter()
        .filter_map(|item| match item {
            Item::MaterializedResult { call_id, output } if outputs.contains_key(call_id) => {
                Some(output)
            }
            _ => None,
        })
        .collect();
    assert_eq!(shown.len(), outputs.len(), "every call keeps its result");
    assert!(
        shown
            .iter()
            .any(|output| output.chars().count() <= 1_500 && output.contains(results::READ)),
        "what does not fit is shown as a preview"
    );
    for (id, original) in &outputs {
        assert_eq!(
            results::resolve(admitted(&owner.events), id, &[])
                .unwrap()
                .as_deref(),
            Some(original.as_str()),
            "the saved original stays readable"
        );
    }
}

#[tokio::test]
async fn calls_made_instead_of_saving_run_and_the_next_request_says_stop_now() {
    let mut first = turn(vec![call("first", "stable_read", "{}")]);
    first.usage = Usage {
        cached_input_tokens: None,
        input_tokens: Some(76_000),
        output_tokens: Some(100),
    };
    let mut provider = Script::new(vec![
        first,
        turn(vec![call("ignored", "stable_read", "{}")]),
        turn(vec![text("Done.")]),
    ]);
    let mut owner = WindowOwner::default();
    fresh(window_request(), "ignored-ask")
        .run(
            &mut provider,
            &mut owner,
            RunLimits {
                max_calls: 8,
                ..limits()
            },
            &Cancellation::default(),
        )
        .await
        .unwrap();
    let asks: Vec<_> = provider
        .requests
        .iter()
        .map(|sent| {
            developer_text(sent)
                .into_iter()
                .filter(|text| is_handoff_ask(text))
                .collect::<Vec<_>>()
        })
        .collect();
    assert!(asks[0].is_empty());
    assert_eq!(asks[1].len(), 1);
    assert!(asks[1][0].starts_with("This context window is nearly full"));
    assert_eq!(asks[2], [handoff_ending()]);
    assert_eq!(owner.effect_counts.get("ignored"), Some(&1));
    assert!(provider.requests[2].items.iter().any(
        |item| matches!(item, Item::MaterializedResult { call_id, .. } if call_id == "ignored")
    ));
}

#[tokio::test]
async fn a_run_names_the_notes_earlier_work_saved_once_and_keeps_it_in_the_window() {
    // Earlier work in this conversation saved two notes. This run's own window
    // fits, so it opens without a fresh window.
    let mut owner = WindowOwner {
        note_references: vec!["z-earlier".into(), "a-later".into()],
        ..Default::default()
    };
    owner
        .notes
        .insert("z-earlier".into(), "Private checkpoint text.".into());
    let mut provider = Script::new(vec![
        turn(vec![call("first", "stable_read", "{}")]),
        turn(vec![text("Done.")]),
    ]);
    fresh(window_request(), "notes-hint")
        .run(
            &mut provider,
            &mut owner,
            limits(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    let hints: Vec<Vec<&str>> = provider
        .requests
        .iter()
        .map(|sent| {
            developer_text(sent)
                .into_iter()
                .filter(|text| text.contains("z-earlier"))
                .collect()
        })
        .collect();
    let [first] = hints[0][..] else {
        panic!("the first request names the earlier notes once: {hints:?}");
    };
    assert!(
        first.find("z-earlier") < first.find("a-later"),
        "the references keep the order saved"
    );
    assert!(!first.contains("Private checkpoint text."), "no note text");
    assert_eq!(
        hints[1],
        [first],
        "the later request keeps the same message"
    );
    assert_eq!(
        provider.requests[0].items[..],
        provider.requests[1].items[..provider.requests[0].items.len()],
        "later requests keep the first request's prefix"
    );
}

/// A run that saves `notes` without asking for a window, then overflows.
async fn forced_close(notes: &[(&str, &str)]) -> (Script, WindowOwner) {
    let saves = notes
        .iter()
        .map(|(id, text)| {
            call(
                id,
                "save_working_note",
                &json!({"text":text,"fresh_window":false}).to_string(),
            )
        })
        .collect::<Vec<_>>();
    let mut overflow = turn(vec![call("after", "stable_read", "{}")]);
    overflow.usage = Usage {
        cached_input_tokens: None,
        input_tokens: Some(84_000),
        output_tokens: Some(100),
    };
    let mut turns = Vec::new();
    if !saves.is_empty() {
        turns.push(turn(saves));
    }
    turns.extend([overflow, turn(vec![text("Done.")])]);
    let mut provider = Script::new(turns);
    let mut owner = WindowOwner {
        seed: vec![Item::Message {
            role: "user".into(),
            text: "original accepted assignment".into(),
        }],
        ..Default::default()
    };
    let mut frozen = window_request();
    frozen.tools.push(results::tool().function());
    frozen
        .tools
        .extend(history::tools().iter().map(Tool::function));
    fresh(frozen, "forced")
        .run(
            &mut provider,
            &mut owner,
            RunLimits {
                max_calls: 8,
                ..limits()
            },
            &Cancellation::default(),
        )
        .await
        .unwrap();
    assert_eq!(window_reasons(&owner.events), [WindowReason::Full]);
    (provider, owner)
}

#[tokio::test]
async fn a_forced_close_carries_the_newest_note_and_earlier_references_in_save_order() {
    let notes = [
        ("z-first", "First checkpoint."),
        ("a-second", "Second checkpoint."),
        (
            "m-newest",
            "Newest checkpoint: sources A and B read; C remains.",
        ),
    ];
    let (provider, _) = forced_close(&notes).await;
    let opened = provider.requests.last().unwrap();
    let [message] = developer_text(opened)[..] else {
        panic!("one developer message opens the fresh window");
    };
    assert!(message.contains("starts fresh for the same accepted work"));
    assert!(message.contains(history::LIST) && message.contains(results::READ));
    assert!(message.contains(r#"["z-first","a-second"]"#), "{message}");
    assert!(message.contains("reference m-newest"));
    assert!(message.contains("Work done after this note was saved is not in it."));
    assert!(opened.items.contains(&Item::assistant(notes[2].1)));
    for (_, earlier) in &notes[..2] {
        assert!(!opened.items.contains(&Item::assistant(*earlier)));
    }
}

#[tokio::test]
async fn a_forced_close_without_a_note_says_only_that_the_window_is_fresh() {
    let (provider, _) = forced_close(&[]).await;
    let opened = provider.requests.last().unwrap();
    let [message] = developer_text(opened)[..] else {
        panic!("one developer message opens the fresh window");
    };
    assert!(message.contains("starts fresh for the same accepted work"));
    assert!(!message.contains("working note"));
    assert!(
        !opened
            .items
            .iter()
            .any(|item| matches!(item, Item::Message { role, .. } if role == "assistant")),
        "nothing is summarized"
    );
}

#[tokio::test]
async fn a_saved_handoff_dispatch_replays_unchanged_after_a_restart() {
    let mut first = turn(vec![call("first", "stable_read", "{}")]);
    first.usage = Usage {
        cached_input_tokens: None,
        input_tokens: Some(76_000),
        output_tokens: Some(100),
    };
    let mut provider = Script::new(vec![first]);
    provider.fail_at = Some(2);
    let mut owner = WindowOwner::default();
    let frozen = window_request();
    let bounds = RunLimits {
        max_calls: 8,
        ..limits()
    };
    fresh(frozen.clone(), "before-crash")
        .run(&mut provider, &mut owner, bounds, &Cancellation::default())
        .await
        .unwrap_err();
    let sent = serde_json::to_vec(&provider.requests[1]).unwrap();
    assert_eq!(
        dispatches(&owner.events)[1].purpose,
        RequestPurpose::Handoff
    );
    // A crash right after the dispatch: its retry must use the saved bytes.
    let dispatch = owner
        .events
        .iter()
        .rposition(|event| matches!(event.kind, EventKind::ProviderStarted { .. }))
        .unwrap();
    owner.events.truncate(dispatch + 1);
    let started = owner.events[dispatch].clone();
    owner.events.push(Event {
        attempt_id: started.attempt_id.clone(),
        ordinal: started.ordinal,
        sequence: started.sequence + 1,
        kind: EventKind::TransportRetry {
            reason: "connection reset".into(),
        },
    });
    owner.events = serde_json::from_slice(&serde_json::to_vec(&owner.events).unwrap()).unwrap();
    let mut successor = Script::new(vec![turn(vec![text("Done.")])]);
    recovered(
        frozen,
        "after-crash",
        admitted(&owner.events),
        0,
        &[],
        None,
        recovery::Inputs::default(),
    )
    .run(&mut successor, &mut owner, bounds, &Cancellation::default())
    .await
    .unwrap();
    assert_eq!(
        serde_json::to_vec(&successor.requests[0]).unwrap(),
        sent,
        "the handoff ask comes from the saved dispatch, not a new measurement"
    );
}
