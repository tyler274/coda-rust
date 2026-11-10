//! The operator's tools the drill drives: gcloud for the physician origin,
//! the access token and the deployed revisions; kubectl for scaling and
//! port-forwards into the observability namespace.
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};

use serde_json::{Value, json};

use super::DrillError;
use crate::observability_drill::live::{ProcessShell, STAGING_PROJECT, STAGING_REGION, Shell};
use crate::observability_drill::model::SignalError;

const NAMESPACE: &str = "observability";

fn gcloud(args: &[String], what: &'static str) -> Result<String, DrillError> {
    let captured = ProcessShell
        .run(args)
        .map_err(|source| DrillError::Tool { what, source })?;
    if !captured.success {
        return Err(DrillError::Tool {
            what,
            source: SignalError::new(format!(
                "gcloud exited {:?} ({})",
                captured.code,
                crate::failure::api_status(&captured.stderr).unwrap_or("no API status")
            )),
        });
    }
    let text = String::from_utf8_lossy(&captured.stdout).trim().to_owned();
    if text.is_empty() || text.contains('\n') {
        return Err(DrillError::Tool {
            what,
            source: SignalError::new("gcloud printed no single value"),
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
    Ok(Value::Object(found))
}

fn kubectl_value(args: &[&str], what: &'static str) -> Result<String, DrillError> {
    let output = Command::new("kubectl")
        .arg("-n")
        .arg(NAMESPACE)
        .args(args)
        .output()
        .map_err(|error| DrillError::Tool {
            what,
            source: SignalError::new(format!("kubectl could not be started ({:?})", error.kind())),
        })?;
    if !output.status.success() {
        return Err(DrillError::Tool {
            what,
            source: SignalError::new(format!("kubectl exited {:?}", output.status.code())),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// Scales observability deployments.
pub(super) struct Kubectl;

impl Kubectl {
    pub(super) fn replicas(&mut self, deployment: &str) -> Result<u32, DrillError> {
        let text = kubectl_value(
            &[
                "get",
                "deployment",
                deployment,
                "-o",
                "jsonpath={.spec.replicas}",
            ],
            "a deployment's replicas",
        )?;
        text.parse().map_err(|_parse| DrillError::Tool {
            what: "a deployment's replicas",
            source: SignalError::new(format!("kubectl printed {text:?}, not a count")),
        })
    }

    pub(super) fn scale(&mut self, deployment: &str, replicas: u32) -> Result<(), DrillError> {
        kubectl_value(
            &[
                "scale",
                "deployment",
                deployment,
                &format!("--replicas={replicas}"),
            ],
            "kubectl scale",
        )?;
        if replicas > 0 {
            self.rollout(deployment)?;
        }
        Ok(())
    }

    pub(super) fn rollout(&mut self, deployment: &str) -> Result<(), DrillError> {
        kubectl_value(
            &[
                "rollout",
                "status",
                &format!("deployment/{deployment}"),
                "--timeout=300s",
            ],
            "kubectl rollout status",
        )
        .map(|_| ())
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
        let mut child = Command::new("kubectl")
            .args([
                "-n",
                NAMESPACE,
                "port-forward",
                &format!("svc/{service}"),
                &format!(":{remote}"),
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| DrillError::Tool {
                what,
                source: SignalError::new(format!(
                    "kubectl could not be started ({:?})",
                    error.kind()
                )),
            })?;
        let stdout = child.stdout.take().ok_or(DrillError::Shape {
            what,
            field: "stdout",
        })?;
        let mut line = String::new();
        let read = BufReader::new(stdout).read_line(&mut line);
        let port = read.ok().and_then(|_| forwarded_port(&line));
        match port {
            Some(port) => Ok(Self { child, port }),
            None => {
                let _ = child.kill();
                Err(DrillError::Tool {
                    what,
                    source: SignalError::new(format!("svc/{service} was not forwarded")),
                })
            }
        }
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
}
