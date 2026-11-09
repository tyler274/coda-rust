use local_product::physician_sources::{ReadablePart, RecordDate, SourceRevision, exact_read};
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
    assert_eq!(read["results"][0]["content"], content);
    assert_eq!(read["results"][0]["revision"], "revision-7");
    assert_eq!(read["results"][0]["record_type"], "DocumentReference");
    assert_eq!(read["results"][0]["dates"][0]["value"], "2026-08");
    assert_eq!(
        read["results"][0]["readable_parts"][0]["text"],
        "Potassium 4.1 mmol/L"
    );
    let events = [
        harness::engine::Event {
            attempt_id: "attempt".into(),
            sequence: 1,
            kind: harness::engine::EventKind::ToolStarted(harness::engine::Invocation {
                id: "read".into(),
                name: "read_patient_sources".into(),
                arguments: "{}".into(),
            }),
        },
        harness::engine::Event {
            attempt_id: "attempt".into(),
            sequence: 2,
            kind: harness::engine::EventKind::ToolReturned {
                call_id: "read".into(),
                output: read.to_string(),
                executed: true,
            },
        },
    ];
    let citations = local_product::physician_chat::published(
        &format!(
            "Claim [[patient_source:{}]]",
            read["results"][0]["citation_id"].as_str().unwrap()
        ),
        &events,
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
    assert_eq!(denied["results"][0]["outcome"], "unavailable");
    let mut corrupt = source.clone();
    corrupt.content.push('!');
    assert!(exact_read(&[corrupt], "one", &["original-id".into()], 4096).is_err());
    let mut large = source;
    large.content = "X".repeat(10000);
    large.sha256 = local_product::sha256_hex(large.content.as_bytes());
    let read = exact_read(&[large], "one", &["original-id".into()], 4096).unwrap();
    assert_eq!(read["results"][0]["outcome"], "over_budget");
    assert!(read["results"][0].get("content").is_none());
}
