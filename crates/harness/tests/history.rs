use harness::engine::{
    Event, EventKind, Invocation, WindowReason,
    history::{HistoryItem, LIST, Message, Page, Query, READ, Role, SEARCH},
    recipe::ReplayEvent,
    results::ToolResult,
};
use serde_json::{Value, json};

/// Two tool results in window 0, one in window 1.
fn two_windows() -> Vec<Event> {
    let mut events = Vec::new();
    let mut push = |kind| {
        events.push(Event {
            attempt_id: "attempt".into(),
            sequence: events.len() as u64 + 1,
            ordinal: 1,
            kind,
        })
    };
    for (id, tool, text, opens) in [
        ("old", "read_chart", "old window", false),
        ("unrelated", "search_chart", "other tool", false),
        ("current", "read_chart", "current window", true),
    ] {
        if opens {
            push(EventKind::WindowOpened {
                opened_at_unix_ms: 0,
                reason: WindowReason::ModelAsked,
                handoff: None,
            });
        }
        push(EventKind::ToolStarted(Invocation {
            id: id.into(),
            name: tool.into(),
            arguments: "{}".into(),
        }));
        push(EventKind::ToolReturned {
            call_id: id.into(),
            result: ToolResult::Original(text.into()),
        });
    }
    events
}

fn answer(name: &str, arguments: Value, events: &[Event]) -> Page<Value> {
    let messages = [Message {
        id: "asked".into(),
        role: Role::User,
        text: "Original question with window text.".into(),
        metadata: Value::Null,
    }];
    Query::parse(name, &arguments)
        .unwrap()
        .answer(&messages, events.iter().map(ReplayEvent::Admitted))
        .unwrap()
}

#[test]
fn a_tool_name_with_no_results_lists_the_tool_names_that_have_results() {
    let events = two_windows();
    for name in [LIST, SEARCH] {
        let mut arguments = json!({"tool_name":":"});
        if name == SEARCH {
            arguments["text"] = json!("window");
        }
        let page = answer(name, arguments.clone(), &events);
        assert!(page.items.is_empty());
        assert_eq!(
            page.tools_with_results.as_deref(),
            Some(&["read_chart".to_owned(), "search_chart".to_owned()][..])
        );
        arguments["tool_name"] = json!("read_chart");
        let page = answer(name, arguments, &events);
        assert_eq!(page.items.len(), 2);
        assert!(
            page.tools_with_results.is_none(),
            "a matching filter is unchanged"
        );
        assert!(
            !serde_json::to_string(&page)
                .unwrap()
                .contains("tools_with_results")
        );
    }
}

#[test]
fn null_optional_arguments_mean_the_same_as_leaving_them_out() {
    let events = two_windows();
    let nulls = json!({"cursor":null,"order":null,"limit":null,"max_chars":null,"tool_name":null,"window":null});
    let listed = answer(LIST, nulls.clone(), &events);
    assert_eq!(listed, answer(LIST, json!({}), &events));
    assert_eq!(
        listed.items.len(),
        4,
        "null filters list the message and the results from every window"
    );
    let windows: Vec<_> = listed
        .items
        .iter()
        .filter_map(|item| match item {
            HistoryItem::Tool(result) => Some(result.window),
            HistoryItem::Message(_) => None,
        })
        .collect();
    assert!(windows.contains(&0) && windows.contains(&1));
    let mut search = nulls;
    search["text"] = json!("window");
    let mut plain = json!({"text":"window"});
    assert_eq!(
        answer(SEARCH, search, &events),
        answer(SEARCH, plain.take(), &events)
    );
    assert_eq!(
        answer(
            READ,
            json!({"id":"asked","offset_chars":null,"limit_chars":null}),
            &events
        ),
        answer(READ, json!({"id":"asked"}), &events)
    );
}

#[test]
fn history_tool_and_window_filters_exclude_other_windows() {
    let events = two_windows();
    let page = Query::parse(LIST, &json!({"tool_name":"read_chart"}))
        .unwrap()
        .answer::<serde_json::Value>(
            &[],
            events
                .iter()
                .map(harness::engine::recipe::ReplayEvent::Admitted),
        )
        .unwrap();
    assert_eq!(
        page.items.len(),
        2,
        "both windows are present before filtering"
    );
    for (window, id, text) in [(0, "old", "old window"), (1, "current", "current window")] {
        let page = Query::parse(LIST, &json!({"tool_name":"read_chart", "window":window}))
            .unwrap()
            .answer::<serde_json::Value>(
                &[],
                events
                    .iter()
                    .map(harness::engine::recipe::ReplayEvent::Admitted),
            )
            .unwrap();
        assert_eq!(page.items.len(), 1);
        let HistoryItem::Tool(result) = &page.items[0] else {
            panic!("expected the selected tool result")
        };
        assert_eq!(result.call.id, id);
        assert_eq!(result.window, window);
        assert_eq!(result.text, text);
        assert!(page.next_cursor.is_none());
    }
}
