//! Bounded CLI transport. Raw diagnostics stay in typed errors, never in receipts/log output.
use super::*;
use std::io::Read;
use std::process::{Output, Stdio};

pub(super) trait Runner {
    fn output(&mut self, args: &[String]) -> Result<Output>;
}
pub(super) struct ProcessRunner;
impl Runner for ProcessRunner {
    fn output(&mut self, args: &[String]) -> Result<Output> {
        bounded_output(
            Command::new("gcloud")
                .args(args)
                .env("CLOUDSDK_CORE_DISABLE_FILE_LOGGING", "true"),
            Duration::from_secs(120),
        )
    }
}
fn bounded_output(command: &mut Command, timeout: Duration) -> Result<Output> {
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stdout = child.stdout.take().ok_or(Error::Cloud("missing-stdout"))?;
    let mut stderr = child.stderr.take().ok_or(Error::Cloud("missing-stderr"))?;
    let out = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        stdout.read_to_end(&mut bytes).map(|_| bytes)
    });
    let err = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        stderr.read_to_end(&mut bytes).map(|_| bytes)
    });
    let deadline = Instant::now() + timeout;
    let timed_out;
    let status = loop {
        if let Some(status) = child.try_wait()? {
            timed_out = false;
            break status;
        }
        if Instant::now() >= deadline {
            child.kill()?;
            timed_out = true;
            break child.wait()?;
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    let drain_deadline = Instant::now() + Duration::from_secs(1);
    while !out.is_finished() || !err.is_finished() {
        if Instant::now() >= drain_deadline {
            return Err(Error::Cloud("command-pipe-drain-timeout"));
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    let stdout = out
        .join()
        .map_err(|_panic| Error::Cloud("stdout-reader-panicked"))??;
    let stderr = err
        .join()
        .map_err(|_panic| Error::Cloud("stderr-reader-panicked"))??;
    if timed_out {
        return Err(Error::Command {
            code: "gcloud-timeout",
            source: Arc::new(CommandFailure {
                status: status.code(),
                stdout,
                stderr,
            }),
        });
    }
    Ok(Output {
        status,
        stdout,
        stderr,
    })
}
pub(super) struct Gcloud<R = ProcessRunner> {
    pub runner: R,
    // Shared wall-clock budget bounds polling as well as individual calls.
    deadline: Instant,
}
impl Default for Gcloud {
    fn default() -> Self {
        Self::new(ProcessRunner, Duration::from_secs(1800))
    }
}
impl<R: Runner> Gcloud<R> {
    pub fn new(runner: R, budget: Duration) -> Self {
        Self {
            runner,
            deadline: Instant::now() + budget,
        }
    }
    pub fn call(&mut self, args: &[&str]) -> Result<Output> {
        if Instant::now() >= self.deadline {
            return Err(Error::Cloud("release-command-budget-exhausted"));
        }
        let mut all: Vec<String> = ["--project", PROJECT, "run"]
            .into_iter()
            .chain(args.iter().copied())
            .chain(["--region", REGION, "--quiet", "--format=json"])
            .map(str::to_owned)
            .collect();
        // The SDK's formatted CLI error is not an absence contract. Capture the
        // API's structured error body instead; never print or persist HTTP logs.
        if args.starts_with(&["services", "describe"]) {
            all.push("--log-http".into());
        }
        self.runner.output(&all)
    }
    pub fn json(&mut self, args: &[&str]) -> Result<Value> {
        let output = self.call(args)?;
        if !output.status.success() {
            return Err(command_error(output));
        }
        Ok(serde_json::from_slice(&output.stdout)?)
    }
}
pub(super) fn command_error(output: Output) -> Error {
    Error::Command {
        code: "gcloud-command-failed",
        source: Arc::new(CommandFailure {
            status: output.status.code(),
            stdout: output.stdout,
            stderr: output.stderr,
        }),
    }
}
pub(super) fn structured_not_found(stderr: &[u8]) -> bool {
    // Parse only an HTTP response body section, not messages that happen to
    // mention NOT_FOUND (IAM and transport failures must remain failures).
    let text = String::from_utf8_lossy(stderr);
    text.rsplit_once("---- response start ----")
        .is_some_and(|(_, section)| {
            if !section.lines().any(|line| line.trim() == "status: 404") {
                return false;
            }
            let Some((_, body)) = section.split_once("-- body start --") else {
                return false;
            };
            let mut values =
                serde_json::Deserializer::from_str(body.trim_start()).into_iter::<Value>();
            let Some(Ok(value)) = values.next() else {
                return false;
            };
            value.pointer("/error/code").and_then(Value::as_u64) == Some(404)
                && value.pointer("/error/status").and_then(Value::as_str) == Some("NOT_FOUND")
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use std::os::unix::process::ExitStatusExt;
    struct Fake {
        outputs: VecDeque<Output>,
        args: Vec<Vec<String>>,
    }
    impl Runner for Fake {
        fn output(&mut self, args: &[String]) -> Result<Output> {
            self.args.push(args.to_vec());
            Ok(self.outputs.pop_front().expect("response"))
        }
    }
    fn output(code: i32, stdout: &str, stderr: &str) -> Output {
        Output {
            status: std::process::ExitStatus::from_raw(code << 8),
            stdout: stdout.as_bytes().to_vec(),
            stderr: stderr.as_bytes().to_vec(),
        }
    }
    #[test]
    fn direct_describe_absence_requires_structured_api_not_found() {
        for (stderr, absent) in [
            (
                "---- response start ----\nstatus: 404\n-- headers start --\n-- headers end --\n-- body start --\n{\"error\":{\"code\":404,\"status\":\"NOT_FOUND\"}}\n---- response end ----",
                true,
            ),
            ("ERROR: NOT_FOUND Cannot find service", false),
            (
                "---- response start ----\nstatus: 403\n-- headers start --\n-- headers end --\n-- body start --\n{\"error\":{\"code\":403,\"status\":\"PERMISSION_DENIED\",\"message\":\"NOT_FOUND\"}}",
                false,
            ),
        ] {
            let mut cloud = Gcloud::new(
                Fake {
                    outputs: VecDeque::from([output(1, "", stderr)]),
                    args: vec![],
                },
                Duration::from_secs(1),
            );
            let result = cloud.service("physician-worker");
            if absent {
                assert_eq!(result.expect("absent"), Observation::Absent);
            } else {
                assert!(result.is_err());
            }
            let args = &cloud.runner.args[0];
            assert_eq!(
                &args[..6],
                [
                    "--project",
                    PROJECT,
                    "run",
                    "services",
                    "describe",
                    "physician-worker"
                ]
            );
            assert!(args.contains(&"--log-http".into()));
            assert!(!args.contains(&"list".into()));
        }
    }
    #[test]
    fn failed_execute_preserves_command_cause_and_is_async() {
        let mut cloud = Gcloud::new(
            Fake {
                outputs: VecDeque::from([output(7, "", "secret-token")]),
                args: vec![],
            },
            Duration::from_secs(1),
        );
        let error = cloud.execute("physician-migrate").expect_err("failed");
        assert_eq!(error.safe_code(), "gcloud-command-failed");
        let Error::Command { source, .. } = error else {
            panic!("typed command cause");
        };
        assert_eq!(source.status, Some(7));
        assert_eq!(source.stderr, b"secret-token");
        assert!(cloud.runner.args[0].contains(&"--async".into()));
    }
    #[test]
    fn service_and_execution_json_contracts_and_mutation_arguments() {
        let service = serde_json::json!({"spec":{"template":{"spec":{"containers":[{"image":"image@sha256:abc"}]}}},"status":{"latestCreatedRevisionName":"rev", "latestReadyRevisionName":"rev", "traffic":[{"revisionName":"rev","percent":100,"latestRevision":true}]}});
        let revision = serde_json::json!({"spec":{"containers":[{"image":"image@sha256:abc"}]}});
        let execution = serde_json::json!({"spec":{"template":{"spec":{"containers":[{"image":"image@sha256:abc"}]}}},"status":{"conditions":[{"type":"Completed","status":"False"}]}});
        let mut cloud = Gcloud::new(
            Fake {
                outputs: VecDeque::from([
                    output(0, &service.to_string(), ""),
                    output(0, &revision.to_string(), ""),
                    output(0, &execution.to_string(), ""),
                    output(0, "{}", ""),
                    output(0, "{}", ""),
                ]),
                args: vec![],
            },
            Duration::from_secs(1),
        );
        assert!(serving(
            &cloud.service("physician").expect("service"),
            "image@sha256:abc"
        ));
        assert_eq!(
            cloud
                .execution("job-execution", "image@sha256:abc")
                .expect("execution"),
            Execution::Failed
        );
        cloud
            .update_job("physician-migrate", "image@sha256:abc")
            .expect("job");
        cloud
            .update_service("physician-worker", "image@sha256:abc")
            .expect("worker");
        assert_eq!(
            &cloud.runner.args[3][3..8],
            [
                "jobs",
                "update",
                "physician-migrate",
                "--image",
                "image@sha256:abc"
            ]
        );
        assert_eq!(
            &cloud.runner.args[4][3..8],
            [
                "services",
                "update",
                "physician-worker",
                "--image",
                "image@sha256:abc"
            ]
        );
    }
    #[test]
    fn raw_command_diagnostics_never_enter_receipt_or_summary() {
        let mut receipt = super::super::tests::receipt();
        retain_cause(
            &mut receipt,
            command_error(output(1, "secret-stdout", "secret-stderr")),
        );
        let encoded = serde_json::to_string(&receipt).expect("receipt");
        let mut summary = Vec::new();
        write_summary(&receipt, &mut summary).expect("summary");
        assert!(!encoded.contains("secret"));
        assert!(!String::from_utf8(summary).expect("utf8").contains("secret"));
        assert_eq!(receipt.causes.len(), 1);
    }
    #[test]
    fn process_timeout_is_bounded_and_retains_cause() {
        let start = Instant::now();
        let error = bounded_output(
            Command::new("sh").args(["-c", "exec sleep 5"]),
            Duration::from_millis(30),
        )
        .expect_err("timeout");
        assert_eq!(error.safe_code(), "gcloud-timeout");
        assert!(start.elapsed() < Duration::from_secs(2));
    }
}
