//! Bounded CLI transport. Raw diagnostics stay in typed errors, never in receipts/log output.
use super::*;
use std::io::Read;
use std::process::{Output, Stdio};

pub(super) const BUDGET_EXHAUSTED: LocalCode = LocalCode::ReleaseCommandBudgetExhausted;

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
    let mut stdout = child
        .stdout
        .take()
        .ok_or(Error::Cloud(LocalCode::MissingStdout))?;
    let mut stderr = child
        .stderr
        .take()
        .ok_or(Error::Cloud(LocalCode::MissingStderr))?;
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
            return Err(Error::Cloud(LocalCode::CommandPipeDrainTimeout));
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    let stdout = out
        .join()
        .map_err(|_panic| Error::Cloud(LocalCode::StdoutReaderPanicked))??;
    let stderr = err
        .join()
        .map_err(|_panic| Error::Cloud(LocalCode::StderrReaderPanicked))??;
    if timed_out {
        return Err(Error::Command {
            code: LocalCode::GcloudTimeout,
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
            return Err(Error::Cloud(BUDGET_EXHAUSTED));
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
        code: LocalCode::GcloudCommandFailed,
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
            let result = cloud.service(ReleaseService::PhysicianWorker);
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
        let error = cloud.execute(Product::Physician).expect_err("failed");
        assert_eq!(error.safe_code().as_str(), "gcloud-command-failed");
        let Error::Command { source, .. } = error else {
            panic!("typed command cause");
        };
        assert_eq!(source.status, Some(7));
        assert_eq!(source.stderr, b"secret-token");
        assert!(cloud.runner.args[0].contains(&"--async".into()));
    }
    #[test]
    fn service_and_execution_json_contracts_and_mutation_arguments() {
        let receipt = super::super::tests::receipt();
        let image = receipt
            .images
            .get(ImageRole::PhysicianRuntime)
            .expect("image");
        let service = serde_json::json!({"spec":{"template":{"spec":{"containers":[{"image":image.as_str()}]}}},"status":{"latestCreatedRevisionName":"rev", "latestReadyRevisionName":"rev", "traffic":[{"revisionName":"rev","percent":100,"latestRevision":true}]}});
        let revision = serde_json::json!({"spec":{"containers":[{"image":image.as_str()}]}});
        let migration_image = receipt
            .images
            .get(ImageRole::PhysicianMigration)
            .expect("migration image");
        let execution = serde_json::json!({"spec":{"template":{"spec":{"containers":[{"image":migration_image.as_str()}]}}},"status":{"conditions":[{"type":"Completed","status":"False"}]}});
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
            &cloud.service(ReleaseService::Physician).expect("service"),
            image
        ));
        assert_eq!(
            cloud
                .execution(
                    &ExecutionId::try_from("job-execution".to_owned()).expect("id"),
                    migration_image
                )
                .expect("execution"),
            Execution::Failed
        );
        assert!(receipt.images.complete());
        cloud
            .update_job(Product::Physician, &receipt.images)
            .expect("job");
        cloud
            .update_service(ReleaseService::PhysicianWorker, None, &receipt.images)
            .expect("worker");
        assert_eq!(
            &cloud.runner.args[3][3..8],
            [
                "jobs",
                "update",
                "physician-migrate",
                "--image",
                &format!(
                    "us-east1-docker.pkg.dev/coda-rust-staging/apps/coda-physician-migrate@sha256:{}",
                    "b".repeat(64)
                )
            ]
        );
        assert_eq!(
            &cloud.runner.args[4][3..9],
            [
                "services",
                "update",
                "physician-worker",
                "--image",
                &format!(
                    "us-east1-docker.pkg.dev/coda-rust-staging/apps/coda-physician-worker@sha256:{}",
                    "b".repeat(64)
                ),
                "--async"
            ]
        );
    }
    #[test]
    fn two_container_service_reads_and_updates_the_named_product() {
        let receipt = super::super::tests::receipt();
        let image = receipt
            .images
            .get(ImageRole::PhysicianRuntime)
            .expect("image");
        // Cloud Run may list the sidecar first: index 0 is not the product.
        let template = shape::tests::observed("physician", image.as_str());
        let sidecar = template["spec"]["containers"][0].clone();
        let service = serde_json::json!({"spec":{"template":template},"status":{"latestCreatedRevisionName":"rev", "latestReadyRevisionName":"rev", "traffic":[{"revisionName":"rev","percent":100,"latestRevision":true}]}});
        let revision = serde_json::json!({"spec":{"containers":[sidecar, {"name":"physician","image":image.as_str()}]}});
        let mut cloud = Gcloud::new(
            Fake {
                outputs: VecDeque::from([
                    output(0, &service.to_string(), ""),
                    output(0, &revision.to_string(), ""),
                    output(0, &service.to_string(), ""),
                    output(0, "{}", ""),
                ]),
                args: vec![],
            },
            Duration::from_secs(1),
        );
        assert!(serving(
            &cloud.service(ReleaseService::Physician).expect("service"),
            image
        ));
        let shape = cloud.shape(ReleaseService::Physician).expect("shape");
        assert_eq!(shape.container(), Some("physician"));
        cloud
            .update_service(
                ReleaseService::Physician,
                shape.container(),
                &receipt.images,
            )
            .expect("update");
        assert_eq!(
            &cloud.runner.args[3][3..10],
            [
                "services",
                "update",
                "physician",
                "--container",
                "physician",
                "--image",
                image.as_str(),
            ]
        );
    }
    #[test]
    fn mutations_reject_missing_target_images_before_invoking_runner() {
        let receipt = super::super::tests::receipt();
        for missing in ImageRole::ALL {
            let mut images = Images::default();
            for role in ImageRole::ALL {
                if role != missing {
                    images.insert(receipt.images.get(role).expect("image").clone());
                }
            }
            let mut cloud = Gcloud::new(
                Fake {
                    outputs: VecDeque::new(),
                    args: vec![],
                },
                Duration::from_secs(1),
            );
            let (result, message) = match missing {
                ImageRole::PhysicianMigration => (
                    cloud.update_job(Product::Physician, &images),
                    "missing migration digest",
                ),
                ImageRole::DuetMigration => (
                    cloud.update_job(Product::Duet, &images),
                    "missing migration digest",
                ),
                ImageRole::PhysicianRuntime => (
                    cloud.update_service(ReleaseService::Physician, None, &images),
                    "missing runtime digest",
                ),
                ImageRole::DuetRuntime => (
                    cloud.update_service(ReleaseService::Duet, None, &images),
                    "missing runtime digest",
                ),
                ImageRole::PhysicianWorker => (
                    cloud.update_service(ReleaseService::PhysicianWorker, None, &images),
                    "missing runtime digest",
                ),
            };
            assert!(matches!(result, Err(Error::Input(actual)) if actual == message));
            assert!(
                cloud.runner.args.is_empty(),
                "runner invoked for {missing:?}"
            );
        }
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
    fn failure_output_names_only_the_allowlisted_api_status() {
        let mut receipt = super::super::tests::receipt();
        let failure = || {
            command_error(output(
                1,
                "secret-stdout",
                "ERROR: (gcloud.run.services.update) PERMISSION_DENIED: secret-stderr",
            ))
        };
        retain_cause(&mut receipt, failure());
        let error = Error::Related {
            primary: Box::new(Error::Cloud(LocalCode::RolloutNotConfirmed)),
            causes: receipt.causes.clone(),
        };
        let message = failure_message(&error);
        assert_eq!(
            message,
            "staging release failed: rollout-not-confirmed (API status PERMISSION_DENIED)"
        );
        let encoded = serde_json::to_string(&receipt).expect("receipt");
        assert!(encoded.contains("\"PERMISSION_DENIED\""));
        assert!(!encoded.contains("secret") && !message.contains("secret"));
        assert_eq!(
            failure_message(&command_error(output(1, "", "secret-stderr"))),
            "staging release failed: gcloud-command-failed"
        );
        assert_eq!(failure_message(&failure()).matches("PERMISSION").count(), 1);
    }
    #[test]
    fn process_timeout_is_bounded_and_retains_cause() {
        let start = Instant::now();
        let error = bounded_output(
            Command::new("sh").args(["-c", "exec sleep 5"]),
            Duration::from_millis(30),
        )
        .expect_err("timeout");
        assert_eq!(error.safe_code().as_str(), "gcloud-timeout");
        assert!(start.elapsed() < Duration::from_secs(2));
    }
}
