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
