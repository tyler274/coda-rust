use std::{
    fs,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);

fn run(records: &str, command: &str, filters: &[&str]) -> std::process::Output {
    let path = std::env::temp_dir().join(format!(
        "discord-fixture-{}-{}.jsonl",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::write(&path, records).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_workspace-tools"))
        .args(["discord", command, "--backup"])
        .arg(&path)
        .args(filters)
        .output()
        .unwrap();
    fs::remove_file(path).unwrap();
    output
}
fn record(id: &str, author: &str, content: &str) -> String {
    format!(
        "{}\n",
        serde_json::json!({"id":id,"channel_id":"123","author":{"id":author,"username":"synthetic"},"timestamp":"2026-01-01T00:00:00Z","content":content,"attachments":[]})
    )
}
#[test]
fn inspect_counts_without_content() {
    let output = run(&record("456", "789", "synthetic secret"), "inspect", &[]);
    assert!(output.status.success());
    assert_eq!(output.stdout, b"records=1\n");
    assert!(output.stderr.is_empty());
}
#[test]
fn search_combines_filters_and_escapes_content() {
    let records = record("456", "789", "hello\nworld") + &record("457", "790", "hello");
    let output = run(&records, "search", &["--author", "789", "--text", "hello"]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<_> = stdout.lines().collect();
    assert_eq!(lines.len(), 2);
    let result: serde_json::Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(result["content"], "hello\nworld");
    assert_eq!(result["author_id"], "789");
    assert_eq!(lines[1], "records=1");
    assert_eq!(
        run(&records, "search", &["--text", "absent"]).stdout,
        b"records=0\n"
    );
}
#[test]
fn malformed_ids_and_filters_fail_without_echo() {
    for invalid in ["0", "-1", "01", "secret", "18446744073709551616"] {
        let output = run(&record(invalid, "789", "secret"), "inspect", &[]);
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(!String::from_utf8(output.stderr).unwrap().contains("secret"));
        assert!(
            !run(
                &record("456", "789", "secret"),
                "search",
                &["--author", invalid]
            )
            .status
            .success()
        );
    }
}
#[test]
fn malformed_json_and_missing_filters_fail() {
    assert!(!run("not json\n", "inspect", &[]).status.success());
    assert!(!run("", "search", &[]).status.success());
    assert_eq!(run("", "inspect", &[]).stdout, b"records=0\n");
}
