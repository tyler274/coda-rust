use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

use serde_json::Value;

/// Same short text, overflowing text, and query on the Rust binary and the Python workers.
/// Skips, and says so, when no provisioned BGE directory is configured.
#[test]
fn matches_pytorch_when_the_model_directory_is_set() {
    let model = std::env::var("DUET_DISCOVERY_ENCODER_MODEL")
        .or_else(|_| std::env::var("CODA_PATIENT_ENCODER_MODEL"));
    let Ok(model) = model else {
        eprintln!(
            "encoder comparison not run: DUET_DISCOVERY_ENCODER_MODEL and CODA_PATIENT_ENCODER_MODEL are unset"
        );
        return;
    };
    let model = PathBuf::from(model);
    let python = std::env::var("DUET_DISCOVERY_ENCODER_PYTHON")
        .or_else(|_| std::env::var("CODA_PATIENT_ENCODER_PYTHON"))
        .unwrap_or_else(|_| "python3".into());
    let bin = PathBuf::from(env!("CARGO_BIN_EXE_encoder"));
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let overflowing = "quiet walk ".repeat(500);
    let cases = [
        (
            "duet",
            root.join("submodules/coda-app/backend/app/services/health_text_encoder_worker.py"),
        ),
        (
            "patient",
            root.join(
                "submodules/coda-agents/src/adapters/patientDiscovery/patient_text_encoder.py",
            ),
        ),
    ];
    for (protocol, script) in cases {
        let rust = encode(&bin, &model, protocol, &overflowing);
        let pytorch = encode_python(&python, &script, &model, protocol, &overflowing);
        assert_eq!(
            rust.len(),
            pytorch.len(),
            "{protocol} window count differs from PyTorch"
        );
        for (actual, expected) in rust.iter().zip(&pytorch) {
            assert_eq!(actual.len(), expected.len(), "{protocol} vector width");
            let score = cosine(actual, expected);
            assert!(score >= 0.999, "{protocol} cosine {score} is below 0.999");
        }
    }
}

fn encode(
    bin: &std::path::Path,
    model: &std::path::Path,
    protocol: &str,
    overflowing: &str,
) -> Vec<Vec<f64>> {
    let mut child = Command::new(bin)
        .args(["--protocol", protocol, "--model"])
        .arg(model)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("encoder binary");
    exchange(&mut child, protocol, overflowing)
}

fn encode_python(
    python: &str,
    script: &std::path::Path,
    model: &std::path::Path,
    protocol: &str,
    overflowing: &str,
) -> Vec<Vec<f64>> {
    let mut command = Command::new(python);
    command
        .arg(script)
        .arg("--model")
        .arg(model)
        .arg("--device")
        .arg("cpu");
    if protocol == "patient" {
        command.arg("--threads").arg("4");
    }
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap_or_else(|error| panic!("python encoder: {error}"));
    exchange(&mut child, protocol, overflowing)
}

fn exchange(child: &mut std::process::Child, protocol: &str, overflowing: &str) -> Vec<Vec<f64>> {
    let mut input = child.stdin.take().expect("stdin");
    let output = child.stdout.take().expect("stdout");
    let mut lines = std::io::BufReader::new(output).lines();
    let ready = lines.next().expect("startup").expect("startup line");
    let ready: Value = serde_json::from_str(&ready).expect("startup json");
    let recipe = ready["recipe"].as_str().expect("recipe");
    let prefix = if protocol == "duet" {
        "duet-bge-cls-v1:"
    } else {
        "patient-cls-v1:"
    };
    assert!(recipe.starts_with(prefix), "{ready}");
    if protocol == "patient" {
        assert_eq!(ready["type"], "ready");
    }

    let requests = [
        request(
            protocol,
            1,
            "query",
            serde_json::json!("What environment feels comfortable?"),
        ),
        request(
            protocol,
            2,
            "documents",
            serde_json::json!(["The crowded gym overwhelms me."]),
        ),
        request(protocol, 3, "documents", serde_json::json!([overflowing])),
    ];
    for request in &requests {
        writeln!(input, "{request}").expect("write");
    }
    input.flush().expect("flush");
    drop(input);

    let mut vectors = Vec::new();
    for _ in 0..requests.len() {
        let line = lines.next().expect("response").expect("response line");
        let value: Value =
            serde_json::from_str(&line).unwrap_or_else(|error| panic!("{error}: {line}"));
        if let Some(error) = value.get("error") {
            panic!("encoder error: {error}");
        }
        collect(&value["result"], &mut vectors);
    }
    let status = child.wait_timeout_or_kill();
    assert!(status.success(), "encoder exited {status:?}");
    vectors
}

fn request(protocol: &str, id: u64, operation: &str, payload: Value) -> String {
    let mut value = serde_json::Map::new();
    if protocol == "patient" {
        value.insert("id".into(), serde_json::json!(id));
    }
    value.insert("operation".into(), serde_json::json!(operation));
    let field = if operation == "query" {
        "text"
    } else {
        "texts"
    };
    value.insert(field.into(), payload);
    serde_json::Value::Object(value).to_string()
}

fn collect(value: &Value, vectors: &mut Vec<Vec<f64>>) {
    if let Some(items) = value.as_array() {
        if items.first().is_some_and(|item| item.is_array()) {
            for item in items {
                collect(item, vectors);
            }
            return;
        }
        vectors.push(
            items
                .iter()
                .map(|item| item.as_f64().expect("dimension"))
                .collect(),
        );
    }
}

fn cosine(left: &[f64], right: &[f64]) -> f64 {
    let dot: f64 = left.iter().zip(right).map(|(a, b)| a * b).sum();
    let left = left.iter().map(|value| value * value).sum::<f64>().sqrt();
    let right = right.iter().map(|value| value * value).sum::<f64>().sqrt();
    dot / (left * right)
}

trait Wait {
    fn wait_timeout_or_kill(&mut self) -> std::process::ExitStatus;
}

impl Wait for std::process::Child {
    fn wait_timeout_or_kill(&mut self) -> std::process::ExitStatus {
        let start = std::time::Instant::now();
        loop {
            if let Some(status) = self.try_wait().expect("wait") {
                return status;
            }
            if start.elapsed() > Duration::from_secs(30) {
                let _ = self.kill();
                return self.wait().expect("killed");
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }
}
