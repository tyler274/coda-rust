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
    let evidence =
        local_product::citations::WindowEvidence::from_records(events.iter(), &[], "one").unwrap();
    let mut staged = local_product::citations::StagedPublications::default();
    staged.apply(local_product::citations::PublishCitationsCall {
        replace: false,
        previews: vec![local_product::citations::PreviewSubmission {
            reference: "original-id".into(),
            revision: "revision-7".into(),
            preview: "Records potassium 4.1 mmol/L.".into(),
        }],
        citations: vec![local_product::citations::CitationSubmission {
            id: citation_id.clone(),
            reference: "original-id".into(),
            revision: "revision-7".into(),
            locator: None,
        }],
    });
    let citations = local_product::citations::publish_job_citations(
        &format!("Claim [[patient_source:{citation_id}]]"),
        local_product::citations::JobCitationMode::InlineMarkers,
        &evidence,
        &staged,
        std::slice::from_ref(&source),
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

/// Citation authority comes strictly from evidence delivered to the model in its
/// current window and previews/citations submitted via `publish_citations`.
#[test]
fn only_evidence_delivered_in_the_current_window_publishes_and_answer_text_is_never_parsed() {
    use local_product::citations::{
        JobCitationMode, PublicationError, StagedPublications, WindowEvidence,
        execute_publish_citations, publish_job_citations, publish_partial_chat,
    };

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

    let mut current = delivered_read("read", &whole);
    let evidence = WindowEvidence::from_records(current.iter(), &[], "one").unwrap();

    // Answer text is never parsed for previews: embedding a bracketed summary block without
    // calling publish_citations refuses at finish, and once publish_citations supplies the preview,
    // any bracketed text block in the answer remains untouched rather than parsed into summary.
    let open_tag = concat!("[[source_", "summaries]]");
    let close_tag = concat!("[[/source_", "summaries]]");
    let text_with_legacy_block =
        format!("{answer}\n{open_tag}\n{id}: Parsed from text.\n{close_tag}");
    assert_eq!(
        publish_job_citations(
            &text_with_legacy_block,
            JobCitationMode::InlineMarkers,
            &evidence,
            &StagedPublications::default(),
            std::slice::from_ref(&source),
        )
        .err(),
        Some(PublicationError::CitationMissingPreview {
            id: id.clone(),
            reference: "lab".into(),
            revision: "r1".into(),
        })
    );

    let tool_out = execute_publish_citations(
        &json!({
            "previews": [{"reference": "lab", "revision": "r1", "preview": "Potassium 4.1 mmol/L recorded by lab."}],
            "citations": [{"id": id, "reference": "lab", "revision": "r1", "locator": null}]
        }),
        &evidence,
        std::slice::from_ref(&source),
    );
    let receipt = tool_out.text().unwrap().to_owned();
    current.push(event(
        4,
        1,
        harness::engine::EventKind::ToolReturned {
            call_id: "pub-1".into(),
            result: harness::engine::results::ToolResult::Original(receipt),
        },
    ));
    let staged = StagedPublications::from_records(current.iter());

    let untouched = publish_job_citations(
        &text_with_legacy_block,
        JobCitationMode::InlineMarkers,
        &evidence,
        &staged,
        std::slice::from_ref(&source),
    )
    .unwrap();
    assert_eq!(
        untouched.text, text_with_legacy_block,
        "nothing is parsed or stripped out of answer text for previews"
    );
    assert_eq!(
        untouched.citations[0].summary.as_deref(),
        Some("Potassium 4.1 mmol/L recorded by lab.")
    );
    assert_eq!(untouched.citations[0].id, id);
    assert_eq!(untouched.citations[0].content, content);

    // Returned but never delivered in a model request: not citable.
    let undelivered = &current[..1];
    let undelivered_evidence =
        WindowEvidence::from_records(undelivered.iter(), &[], "one").unwrap();
    assert_eq!(
        publish_job_citations(
            &answer,
            JobCitationMode::InlineMarkers,
            &undelivered_evidence,
            &staged,
            std::slice::from_ref(&source),
        )
        .err(),
        Some(PublicationError::CitationNotInWindow {
            id: id.clone(),
            reference: "lab".into(),
            revision: "r1".into(),
        })
    );

    // Delivered in a window the model has since left: refused at finish.
    let mut earlier = current.clone();
    earlier.push(event(
        5,
        2,
        harness::engine::EventKind::WindowOpened {
            opened_at_unix_ms: 0,
            handoff: None,
            reason: harness::engine::WindowReason::ModelAsked,
        },
    ));
    let fresh_evidence = WindowEvidence::from_records(earlier.iter(), &[], "one").unwrap();
    let fresh_staged = StagedPublications::from_records(earlier.iter());
    assert!(fresh_evidence.whole.is_empty());
    assert!(fresh_staged.previews.is_empty());
    assert!(fresh_staged.citations.is_empty());
    assert_eq!(
        publish_job_citations(
            &answer,
            JobCitationMode::InlineMarkers,
            &fresh_evidence,
            &fresh_staged,
            std::slice::from_ref(&source),
        )
        .err(),
        Some(PublicationError::CitationNotInWindow {
            id: id.clone(),
            reference: "lab".into(),
            revision: "r1".into(),
        })
    );
    // A partial answer cannot be corrected, so the unsupported marker leaves it.
    let partial = publish_partial_chat(
        &answer,
        &fresh_evidence,
        &fresh_staged,
        std::slice::from_ref(&source),
    );
    assert_eq!(partial.text, "Potassium is 4.1 .");
    assert!(partial.citations.is_empty());

    // Moving on to a new record in the fresh window (with default `replace: false`) publishes
    // cleanly in both InlineMarkers and DeclaredSet modes without stale Window 0 leakage or refusal.
    let note_content = "Follow-up visit note.";
    let note = SourceRevision {
        workspace: "one".into(),
        source_id: "ehr".into(),
        candidate_id: "note-2".into(),
        revision: "r2".into(),
        record_type: "DocumentReference".into(),
        dates: vec![],
        content: note_content.into(),
        sha256: local_product::sha256_hex(note_content.as_bytes()),
        readable_parts: vec![],
        provenance: json!({"native_record":"note-2"}),
        native: None,
    };
    let note_whole = exact_read(std::slice::from_ref(&note), "one", &["note-2".into()], 4096)
        .unwrap()
        .text()
        .unwrap()
        .to_owned();
    let mut moved_on = earlier.clone();
    moved_on.extend([
        event(
            6,
            2,
            harness::engine::EventKind::ToolReturned {
                call_id: "read-note".into(),
                result: harness::engine::results::ToolResult::Original(note_whole.clone()),
            },
        ),
        event(
            7,
            3,
            harness::engine::EventKind::ProviderStarted {
                dispatched_at_unix_ms: 2,
                request: harness::engine::SavedRequest {
                    added: vec![harness::engine::Item::Result {
                        call_id: "read-note".into(),
                        output: harness::engine::results::ResultView::whole(
                            "read-note",
                            &note_whole,
                        ),
                    }],
                    items: vec![harness::engine::RequestItem::Event {
                        attempt: "attempt".into(),
                        sequence: 7,
                        index: 0,
                    }],
                },
                context: Default::default(),
            },
        ),
        event(
            8,
            3,
            harness::engine::EventKind::ProviderReturned(harness::engine::Turn {
                items: vec![],
                receipt: serde_json::Value::Null,
                usage: Default::default(),
            }),
        ),
    ]);
    let moved_evidence = WindowEvidence::from_records(moved_on.iter(), &[], "one").unwrap();
    let note_pub = execute_publish_citations(
        &json!({
            "previews": [{"reference": "note-2", "revision": "r2", "preview": "Follow-up visit note recorded."}],
            "citations": [{"id": "n2", "reference": "note-2", "revision": "r2", "locator": null}]
        }),
        &moved_evidence,
        &[source.clone(), note.clone()],
    )
    .text()
    .unwrap()
    .to_owned();
    moved_on.push(event(
        9,
        3,
        harness::engine::EventKind::ToolReturned {
            call_id: "pub-2".into(),
            result: harness::engine::results::ToolResult::Original(note_pub),
        },
    ));
    let moved_staged = StagedPublications::from_records(moved_on.iter());
    let inline_moved = publish_job_citations(
        "Follow-up [[patient_source:n2]].",
        JobCitationMode::InlineMarkers,
        &moved_evidence,
        &moved_staged,
        &[source.clone(), note.clone()],
    )
    .unwrap();
    assert_eq!(inline_moved.citations.len(), 1);
    assert_eq!(inline_moved.citations[0].reference, "note-2");
    assert_eq!(inline_moved.source_previews.len(), 1);

    let declared_moved = publish_job_citations(
        "Revised reply grounded in the follow-up note.",
        JobCitationMode::DeclaredSet,
        &moved_evidence,
        &moved_staged,
        &[source.clone(), note],
    )
    .unwrap();
    assert_eq!(declared_moved.citations.len(), 1);
    assert_eq!(declared_moved.citations[0].reference, "note-2");
    assert_eq!(declared_moved.source_previews.len(), 1);

    // Re-reading Window 0's record in Window 1 without calling `publish_citations` in Window 1
    // does not silently reuse Window 0's preview or citation submission.
    let mut reread_without_republish = earlier;
    reread_without_republish.extend([
        event(
            6,
            2,
            harness::engine::EventKind::ToolReturned {
                call_id: "reread-lab".into(),
                result: harness::engine::results::ToolResult::Original(whole.clone()),
            },
        ),
        event(
            7,
            3,
            harness::engine::EventKind::ProviderStarted {
                dispatched_at_unix_ms: 2,
                request: harness::engine::SavedRequest {
                    added: vec![harness::engine::Item::Result {
                        call_id: "reread-lab".into(),
                        output: harness::engine::results::ResultView::whole("reread-lab", &whole),
                    }],
                    items: vec![harness::engine::RequestItem::Event {
                        attempt: "attempt".into(),
                        sequence: 7,
                        index: 0,
                    }],
                },
                context: Default::default(),
            },
        ),
        event(
            8,
            3,
            harness::engine::EventKind::ProviderReturned(harness::engine::Turn {
                items: vec![],
                receipt: serde_json::Value::Null,
                usage: Default::default(),
            }),
        ),
    ]);
    let reread_evidence =
        WindowEvidence::from_records(reread_without_republish.iter(), &[], "one").unwrap();
    let reread_staged = StagedPublications::from_records(reread_without_republish.iter());
    assert_eq!(
        publish_job_citations(
            &answer,
            JobCitationMode::InlineMarkers,
            &reread_evidence,
            &reread_staged,
            std::slice::from_ref(&source),
        )
        .err(),
        Some(PublicationError::CitationMissingPreview {
            id: id.clone(),
            reference: "lab".into(),
            revision: "r1".into(),
        })
    );
    assert_eq!(
        publish_job_citations(
            "Plain prose after re-read without new preview.",
            JobCitationMode::DeclaredSet,
            &reread_evidence,
            &reread_staged,
            std::slice::from_ref(&source),
        )
        .err(),
        Some(PublicationError::MissingPreview {
            reference: "lab".into(),
            revision: "r1".into(),
        })
    );

    // A partial passage read grants passage authority only for the delivered byte span,
    // never whole-record authority or unread spans.
    let mut large = source;
    large.content = format!("Delivered lead sentence. {}", "x".repeat(6_000)) + " Unread tail.";
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
    let passage_events = delivered_read("passage", &passage);
    let passage_evidence = WindowEvidence::from_records(passage_events.iter(), &[], "one").unwrap();
    assert!(passage_evidence.whole.is_empty());
    assert!(passage_evidence.has_record("lab", "r1"));

    let mut staged_whole_on_passage = StagedPublications::default();
    staged_whole_on_passage.apply(local_product::citations::PublishCitationsCall {
        previews: vec![local_product::citations::PreviewSubmission {
            reference: "lab".into(),
            revision: "r1".into(),
            preview: "Large lab report partial read.".into(),
        }],
        citations: vec![local_product::citations::CitationSubmission {
            id: "whole".into(),
            reference: "lab".into(),
            revision: "r1".into(),
            locator: None,
        }],
        replace: false,
    });
    assert_eq!(
        publish_job_citations(
            "Claim [[patient_source:whole]].",
            JobCitationMode::InlineMarkers,
            &passage_evidence,
            &staged_whole_on_passage,
            std::slice::from_ref(&large),
        )
        .err(),
        Some(PublicationError::CitationNotInWindow {
            id: "whole".into(),
            reference: "lab".into(),
            revision: "r1".into(),
        })
    );

    let mut staged_delivered_passage = StagedPublications::default();
    staged_delivered_passage.apply(local_product::citations::PublishCitationsCall {
        previews: vec![local_product::citations::PreviewSubmission {
            reference: "lab".into(),
            revision: "r1".into(),
            preview: "Large lab report partial read.".into(),
        }],
        citations: vec![local_product::citations::CitationSubmission {
            id: "lead".into(),
            reference: "lab".into(),
            revision: "r1".into(),
            locator: Some(local_product::citations::RequestedLocator::Passage {
                part: 0,
                quote: "Delivered lead sentence.".into(),
                start_byte: None,
            }),
        }],
        replace: false,
    });
    let published_passage = publish_job_citations(
        "Lead [[patient_source:lead]].",
        JobCitationMode::InlineMarkers,
        &passage_evidence,
        &staged_delivered_passage,
        std::slice::from_ref(&large),
    )
    .unwrap();
    assert_eq!(
        published_passage.citations[0].cited_passage(),
        Some("Delivered lead sentence.")
    );

    // An unclosed marker refuses a final answer and loses only its opening in a partial one.
    let empty_evidence = WindowEvidence::default();
    let empty_staged = StagedPublications::default();
    assert_eq!(
        publish_job_citations(
            "Claim [[patient_source:abc",
            JobCitationMode::InlineMarkers,
            &empty_evidence,
            &empty_staged,
            &[],
        )
        .err(),
        Some(PublicationError::Unterminated)
    );
    assert_eq!(
        publish_partial_chat(
            "Claim [[patient_source:abc",
            &empty_evidence,
            &empty_staged,
            &[]
        )
        .text,
        "Claim abc"
    );
}
