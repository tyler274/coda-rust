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
