//! The operator's tools the drill drives: gcloud for the physician origin,
//! the access token and the deployed revisions; kubectl, always pinned to the
//! staging observability cluster, for scaling and port-forwards.
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};

use serde_json::{Value, json};

use super::DrillError;
use crate::observability_drill::live::{STAGING_PROJECT, STAGING_REGION};

const NAMESPACE: &str = "observability";
/// The kubeconfig context `gcloud container clusters get-credentials
/// rust-observability --project=coda-rust-staging --region=us-east1` writes.
/// Every kubectl call names it, so an ambient context elsewhere is never used.
pub(super) const CONTEXT: &str = "gke_coda-rust-staging_us-east1_rust-observability";

fn gcloud(args: &[String], what: &'static str) -> Result<String, DrillError> {
    let output = Command::new("gcloud")
        .args(args)
        .output()
        .map_err(|source| DrillError::Io { what, source })?;
    if !output.status.success() {
        return Err(DrillError::Gcloud {
            what,
            code: output.status.code(),
            status: crate::failure::api_status(&output.stderr)
                .unwrap_or("no API status")
                .to_owned(),
        });
    }
    let text = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if text.is_empty() || text.contains('\n') {
        return Err(DrillError::Shape {
            what,
            field: "a single value",
        });
    }
    Ok(text)
}

fn describe(service: &str, format: &str, what: &'static str) -> Result<String, DrillError> {
    gcloud(
        &[
            "run".into(),
            "services".into(),
            "describe".into(),
            service.into(),
            format!("--project={STAGING_PROJECT}"),
            format!("--region={STAGING_REGION}"),
            format!("--format=value({format})"),
        ],
        what,
    )
}

/// The physician service's own `run.app` origin, which its IAP audience names.
pub(super) fn physician_origin() -> Result<String, DrillError> {
    describe("physician", "status.url", "the physician origin")
}

/// The operator's access token, used only to call IAM `signJwt`.
pub(super) fn access_token() -> Result<String, DrillError> {
    gcloud(
        &["auth".into(), "print-access-token".into()],
        "the operator access token",
    )
}

/// The deployed revisions and images the drill runs against.
pub(super) fn revisions() -> Result<Value, DrillError> {
    let mut found = serde_json::Map::new();
    for service in ["physician", "physician-worker"] {
        let revision = describe(
            service,
            "status.latestReadyRevisionName",
            "a Cloud Run revision",
        )?;
        let image = describe(
            service,
            "spec.template.spec.containers[0].image",
            "a Cloud Run image",
        )?;
        found.insert(
            service.to_owned(),
            json!({ "revision": revision, "image": image }),
        );
    }
    for deployment in ["langfuse-web", "otel-collector"] {
        let image = kubectl_value(
            &[
                "get",
                "deployment",
                deployment,
                "-o",
                "jsonpath={.spec.template.spec.containers[0].image}",
            ],
            "a cluster image",
        )?;
        found.insert(deployment.to_owned(), json!({ "image": image }));
    }
    found.insert("kube_context".to_owned(), json!(CONTEXT));
    Ok(Value::Object(found))
}

/// `kubectl --context CONTEXT -n observability`, the only way the drill
/// starts kubectl.
fn kubectl() -> Command {
    let mut command = Command::new("kubectl");
    command.args(["--context", CONTEXT, "-n", NAMESPACE]);
    command
}

fn kubectl_value(args: &[&str], what: &'static str) -> Result<String, DrillError> {
    let output = kubectl()
        .args(args)
        .output()
        .map_err(|source| DrillError::Io { what, source })?;
    if !output.status.success() {
        return Err(DrillError::Exited {
            what,
            code: output.status.code(),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// The deployments a drill may scale to zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Workload {
    LangfuseWeb,
    LangfuseSso,
}

impl Workload {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::LangfuseWeb => "langfuse-web",
            Self::LangfuseSso => "langfuse-sso",
        }
    }
}

/// Scales the drill's workloads in the staging observability namespace.
pub(super) struct Kubectl;

impl Kubectl {
    pub(super) fn replicas(&mut self, workload: Workload) -> Result<u32, DrillError> {
        let what = "a deployment's replicas";
        let text = kubectl_value(
            &[
                "get",
                "deployment",
                workload.name(),
                "-o",
                "jsonpath={.spec.replicas}",
            ],
            what,
        )?;
        text.parse()
            .map_err(|source| DrillError::Count { what, text, source })
    }

    /// Scales `workload` to zero and returns the guard that owns putting it
    /// back. `replicas` was read first, so the restore uses the real count.
    pub(super) fn scale_down(&mut self, workload: Workload) -> Result<ScaledDown, DrillError> {
        let replicas = self.replicas(workload)?;
        if replicas == 0 {
            return Err(DrillError::AlreadyDown(workload.name()));
        }
        self.scale(workload, 0)?;
        Ok(ScaledDown {
            workload,
            replicas,
            settled: false,
        })
    }

    /// Scales `workload` to `replicas` and waits for the rollout.
    pub(super) fn scale(&mut self, workload: Workload, replicas: u32) -> Result<(), DrillError> {
        kubectl_value(
            &[
                "scale",
                "deployment",
                workload.name(),
                &format!("--replicas={replicas}"),
            ],
            "kubectl scale",
        )?;
        if replicas == 0 {
            return Ok(());
        }
        kubectl_value(
            &[
                "rollout",
                "status",
                &format!("deployment/{}", workload.name()),
                "--timeout=300s",
            ],
            "kubectl rollout status",
        )
        .map(|_| ())
    }
}

/// A workload the drill scaled to zero. It must be settled: restored, or
/// deliberately left down for a later `recover`. Dropping it unsettled, on a
/// panic, restores it as a last resort.
#[must_use = "a scaled-down workload must be restored or left down on purpose"]
pub(super) struct ScaledDown {
    workload: Workload,
    replicas: u32,
    settled: bool,
}

impl ScaledDown {
    pub(super) fn replicas(&self) -> u32 {
        self.replicas
    }

    pub(super) fn restore(mut self, kubectl: &mut Kubectl) -> Result<(), DrillError> {
        self.settled = true;
        kubectl.scale(self.workload, self.replicas)
    }

    /// Restores after `primary` ended the run, keeping both errors. The
    /// second value says whether the workload is back.
    pub(super) fn restore_after(
        self,
        primary: DrillError,
        kubectl: &mut Kubectl,
    ) -> (DrillError, bool) {
        let workload = self.workload.name();
        match self.restore(kubectl) {
            Ok(()) => (primary, true),
            Err(restore) => (
                DrillError::Restore {
                    primary: Box::new(primary),
                    workload,
                    restore: Box::new(restore),
                },
                false,
            ),
        }
    }

    /// Leaves the workload down for `recover`, which reads the count back
    /// from this run's report.
    pub(super) fn leave_down(mut self) {
        self.settled = true;
    }
}

impl Drop for ScaledDown {
    fn drop(&mut self) {
        if self.settled {
            return;
        }
        let name = self.workload.name();
        if let Err(error) = Kubectl.scale(self.workload, self.replicas) {
            eprintln!(
                "{name} is still at 0 replicas and could not be restored ({error}); run \
                 kubectl --context {CONTEXT} -n {NAMESPACE} scale deployment {name} --replicas={}",
                self.replicas
            );
        }
    }
}

/// A `kubectl port-forward` on a free local port, stopped when dropped.
pub(super) struct Forward {
    child: Child,
    port: u16,
}

impl Forward {
    pub(super) fn open(service: &'static str, remote: u16) -> Result<Self, DrillError> {
        let what = "kubectl port-forward";
        let mut child = kubectl()
            .args([
                "port-forward",
                &format!("svc/{service}"),
                &format!(":{remote}"),
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|source| DrillError::Io { what, source })?;
        let stdout = child.stdout.take().ok_or(DrillError::Shape {
            what,
            field: "stdout",
        })?;
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        let port = match reader.read_line(&mut line) {
            Ok(_) => forwarded_port(&line).ok_or(DrillError::NotForwarded(service)),
            Err(source) => Err(DrillError::Io { what, source }),
        };
        let port = match port {
            Ok(port) => port,
            Err(error) => {
                // The helper never forwarded; nothing in the cluster changed.
                let _ = child.kill();
                let _ = child.wait();
                return Err(error);
            }
        };
        // kubectl logs every connection on stdout; keep reading so a closed
        // pipe never kills it mid-drill. The thread ends with the process.
        std::thread::spawn(move || std::io::copy(&mut reader, &mut std::io::sink()));
        Ok(Self { child, port })
    }

    pub(super) fn url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }
}

impl Drop for Forward {
    fn drop(&mut self) {
        // The forward is only a local helper process; a failed kill leaves
        // nothing in the cluster.
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// The local port in `Forwarding from 127.0.0.1:PORT -> REMOTE`.
pub(super) fn forwarded_port(line: &str) -> Option<u16> {
    let rest = line.strip_prefix("Forwarding from 127.0.0.1:")?;
    rest.split_whitespace().next()?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_forwarded_port_is_read_from_kubectl() {
        assert_eq!(
            forwarded_port("Forwarding from 127.0.0.1:41234 -> 3000\n"),
            Some(41234)
        );
        assert_eq!(forwarded_port("Forwarding from [::1]:41234 -> 3000"), None);
        assert_eq!(forwarded_port(""), None);
    }

    #[test]
    fn every_kubectl_call_names_the_staging_context() {
        let command = kubectl();
        let args: Vec<_> = command.get_args().collect();
        assert_eq!(args, ["--context", CONTEXT, "-n", NAMESPACE]);
        assert!(CONTEXT.contains(STAGING_PROJECT) && CONTEXT.contains(STAGING_REGION));
    }
}
