use local_product::physician_sources::{
    ChartRequest, ChartScope, READ_TOOL, ReadablePart, RecordDate, SEARCH_TOOL, SourceRevision,
    exact_read,
};
use serde_json::json;
#[test]
fn exact_reader_rejects_corruption_and_cross_workspace_and_never_truncates_content() {
    let content = "source-native α result 4.1 mmol/L";
    let source = SourceRevision {
        workspace: "one".into(),
        source_id: "ehr".into(),
        candidate_id: "original-id".into(),
        revision: "revision-7".into(),
        record_type: "DocumentReference".into(),
        dates: vec![RecordDate {
            name: "date".into(),
            value: "2026-08".into(),
        }],
        content: content.into(),
        sha256: local_product::sha256_hex(content.as_bytes()),
        readable_parts: vec![ReadablePart {
            label: "attachment".into(),
            text: "Potassium 4.1 mmol/L".into(),
        }],
        provenance: json!({"source_id":"native-1"}),
        native: None,
    };
    let read = exact_read(
        std::slice::from_ref(&source),
        "one",
        &["original-id".into()],
        4096,
    )
    .unwrap();
    let read: serde_json::Value = serde_json::from_str(read.text().unwrap()).unwrap();
    assert_eq!(read["results"][0]["content"], content);
    assert_eq!(read["results"][0]["revision"], "revision-7");
    assert_eq!(read["results"][0]["record_type"], "DocumentReference");
    assert_eq!(read["results"][0]["dates"][0]["value"], "2026-08");
    assert_eq!(
        read["results"][0]["readable_parts"][0]["text"],
        "Potassium 4.1 mmol/L"
    );
    // A whole-record read that reached the model in the current window is
    // citable; the same read before a fresh window, or a passage, is not.
    let citation_id = read["results"][0]["citation_id"]
        .as_str()
        .unwrap()
        .to_owned();
    let events = delivered_read("read", &read.to_string());
    let delivered =
        local_product::physician_sources::delivered_citations(events.iter(), &[]).unwrap();
    let citations = local_product::physician_chat::published(
        &format!("Claim [[patient_source:{citation_id}]]"),
        &delivered,
    )
    .unwrap()
    .citations;
    let saved = serde_json::to_value(&citations[0]).unwrap();
    assert_eq!(
        saved["readable_parts"],
        read["results"][0]["readable_parts"]
    );
    assert_eq!(saved["content"], content);
    assert_eq!(saved["provenance"], source.provenance);
    let mut legacy = saved;
    legacy.as_object_mut().unwrap().remove("readable_parts");
    let decoded: local_product::physician_chat::Citation = serde_json::from_value(legacy).unwrap();
    assert!(decoded.readable_parts.is_empty());
    assert_eq!(decoded.content, content);
    let denied = exact_read(
        std::slice::from_ref(&source),
        "two",
        &["original-id".into()],
        4096,
    )
    .unwrap();
    let denied: serde_json::Value = serde_json::from_str(denied.text().unwrap()).unwrap();
    assert_eq!(denied["results"][0]["outcome"], "unavailable");
    let mut corrupt = source.clone();
    corrupt.content.push('!');
    assert!(exact_read(&[corrupt], "one", &["original-id".into()], 4096).is_err());
    let mut large = source;
    large.content = "α🔥\n\"".repeat(2400);
    large.sha256 = local_product::sha256_hex(large.content.as_bytes());
    let other = SourceRevision {
        workspace: "two".into(),
        revision: "private-revision".into(),
        ..large.clone()
    };
    let sources = [other, large.clone()];
    let search = ChartRequest::parse(
        SEARCH_TOOL,
        &json!({"query":"4.1 potassium","record_type":"documentreference"}),
        ChartScope::Chat,
    )
    .unwrap();
    let page = search.run("one", 12, &sources).unwrap();
    let page: serde_json::Value = serde_json::from_str(page.text().unwrap()).unwrap();
    assert_eq!(page["results"].as_array().unwrap().len(), 1);
    assert!(!page.to_string().contains("private-revision"));
    assert!(page["results"][0].get("content").is_none());
    let mut catalog = Vec::new();
    for index in (0..41).rev() {
        catalog.push(SourceRevision {
            candidate_id: format!("record-{index:02}"),
            ..large.clone()
        });
    }
    let mut after = serde_json::Value::Null;
    let mut seen = std::collections::BTreeSet::new();
    loop {
        let search = ChartRequest::parse(
            SEARCH_TOOL,
            &json!({"query":"","after":after}),
            ChartScope::Chat,
        )
        .unwrap();
        if !after.is_null() {
            assert!(matches!(
                search.run("one", 13, &catalog).unwrap(),
                harness::engine::ToolOutcome::Refused(_)
            ));
        }
        let page = search.run("one", 12, &catalog).unwrap();
        let page: serde_json::Value = serde_json::from_str(page.text().unwrap()).unwrap();
        for item in page["results"].as_array().unwrap() {
            assert!(
                seen.insert(
                    item["reference"]["candidate_id"]
                        .as_str()
                        .unwrap()
                        .to_owned()
                )
            );
        }
        after = page["next"].clone();
        if after.is_null() {
            break;
        }
    }
    assert_eq!(seen.len(), catalog.len());
    let mut reference = page["results"][0]["reference"].clone();
    let mut parts = [String::new(), String::new()];
    loop {
        let read = ChartRequest::parse(
            READ_TOOL,
            &json!({"references":[reference],"max_result_bytes":4096}),
            ChartScope::Chat,
        )
        .unwrap();
        let page = read.run("one", 12, &sources).unwrap();
        let page: serde_json::Value = serde_json::from_str(page.text().unwrap()).unwrap();
        assert!(page.to_string().len() <= 4096);
        let result = &page["results"][0];
        assert_eq!(result["outcome"], "passage");
        let part = &mut parts[result["part"].as_u64().unwrap() as usize];
        assert_eq!(result["start_byte"], part.len());
        part.push_str(result["content"].as_str().unwrap());
        assert_eq!(result["end_byte"], part.len());
        reference = result["next"].clone();
        if reference.is_null() {
            break;
        }
        let changed_sources = [SourceRevision {
            revision: "new-revision".into(),
            ..large.clone()
        }];
        let changed = read.run("one", 20, &changed_sources).unwrap();
        let changed: serde_json::Value = serde_json::from_str(changed.text().unwrap()).unwrap();
        assert_eq!(
            changed["results"][0]["reason"],
            "revision_changed_repeat_discovery"
        );
    }
    assert_eq!(parts, [large.content, large.readable_parts[0].text.clone()]);
}

/// An admitted record, as the runtime store hands records to delivery checks.
fn event(
    sequence: u64,
    ordinal: u64,
    kind: harness::engine::EventKind,
) -> (harness::engine::Event, bool) {
    let event = harness::engine::Event {
        attempt_id: "attempt".into(),
        sequence,
        ordinal,
        kind,
    };
    (event, true)
}

/// One chart read returned and then delivered whole to the model.
fn delivered_read(call_id: &str, output: &str) -> Vec<(harness::engine::Event, bool)> {
    use harness::engine::{EventKind, Item, RequestItem, SavedRequest, results};
    vec![
        event(
            1,
            0,
            EventKind::ToolReturned {
                call_id: call_id.into(),
                result: results::ToolResult::Original(output.into()),
            },
        ),
        event(
            2,
            1,
            EventKind::ProviderStarted {
                dispatched_at_unix_ms: 1,
                request: SavedRequest {
                    added: vec![Item::Result {
                        call_id: call_id.into(),
                        output: results::ResultView::whole(call_id, output),
                    }],
                    items: vec![RequestItem::Event {
                        attempt: "attempt".into(),
                        sequence: 2,
                        index: 0,
                    }],
                },
                context: Default::default(),
            },
        ),
        event(
            3,
            1,
            EventKind::ProviderReturned(harness::engine::Turn {
                items: vec![],
                receipt: serde_json::Value::Null,
                usage: Default::default(),
            }),
        ),
    ]
}

/// Citation authority comes only from a whole record the model received in its
/// current window: a passage carries no identity, and a fresh window drops it.
#[test]
fn only_whole_records_delivered_in_the_current_window_are_citable() {
    use local_product::physician_chat::{PublicationError, published, published_partial};
    use local_product::physician_sources::delivered_citations;
    let content = "Potassium 4.1 mmol/L";
    let source = SourceRevision {
        workspace: "one".into(),
        source_id: "ehr".into(),
        candidate_id: "lab".into(),
        revision: "r1".into(),
        record_type: "Observation".into(),
        dates: vec![],
        content: content.into(),
        sha256: local_product::sha256_hex(content.as_bytes()),
        readable_parts: vec![],
        provenance: json!({"native_record":"lab"}),
        native: None,
    };
    let whole = exact_read(std::slice::from_ref(&source), "one", &["lab".into()], 4096).unwrap();
    let whole = whole.text().unwrap().to_owned();
    let id = local_product::physician_sources::citation_id(&source);
    assert!(whole.contains(&id));
    let answer = format!("Potassium is 4.1 [[patient_source:{id}]].");

    let current = delivered_read("read", &whole);
    let delivered = delivered_citations(current.iter(), &[]).unwrap();
    let answered = published(&answer, &delivered).unwrap();
    assert_eq!(answered.text, answer);
    assert_eq!(answered.citations[0].id, id);
    assert_eq!(answered.citations[0].content, content);

    // Returned but never delivered: not citable.
    let undelivered = &current[..1];
    let delivered = delivered_citations(undelivered.iter(), &[]).unwrap();
    assert_eq!(
        published(&answer, &delivered).err(),
        Some(PublicationError::NotRead(id.clone()))
    );

    // Delivered in a window the model has since left: not citable.
    let mut earlier = current.clone();
    earlier.push(event(
        4,
        1,
        harness::engine::EventKind::WindowOpened {
            opened_at_unix_ms: 0,
            handoff: None,
            reason: harness::engine::WindowReason::ModelAsked,
        },
    ));
    let delivered = delivered_citations(earlier.iter(), &[]).unwrap();
    assert!(delivered.is_empty());
    assert_eq!(
        published(&answer, &delivered).err(),
        Some(PublicationError::NotRead(id.clone()))
    );
    // A partial answer cannot be corrected, so the unsupported marker leaves it.
    let partial = published_partial(&answer, &delivered);
    assert_eq!(partial.text, "Potassium is 4.1 .");
    assert!(partial.citations.is_empty());

    // A passage is delivered but carries no citation identity.
    let mut large = source;
    large.content = "x".repeat(10_000);
    large.sha256 = local_product::sha256_hex(large.content.as_bytes());
    let passage = ChartRequest::parse(
        READ_TOOL,
        &json!({"references":[{"candidate_id":"lab","revision":"r1","cursor":null}],"max_result_bytes":4096}),
        ChartScope::Chat,
    )
    .unwrap()
    .run("one", 1, std::slice::from_ref(&large))
    .unwrap();
    let passage = passage.text().unwrap().to_owned();
    assert!(passage.contains("\"outcome\":\"passage\""));
    assert!(!passage.contains("citation_id"));
    let events = delivered_read("passage", &passage);
    assert!(delivered_citations(events.iter(), &[]).unwrap().is_empty());

    // An unclosed marker refuses a final answer and loses only its opening in a partial one.
    assert_eq!(
        published("Claim [[patient_source:abc", &[]).err(),
        Some(PublicationError::Unterminated)
    );
    assert_eq!(
        published_partial("Claim [[patient_source:abc", &[]).text,
        "Claim abc"
    );
}
