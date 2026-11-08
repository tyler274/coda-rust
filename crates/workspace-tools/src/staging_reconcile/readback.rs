//! Physician Cloud Run readback compared with the reviewed root's planned values.
use super::{Result, refuse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

pub(super) const SERVICE: &str = "physician";
const ADDRESS: &str = "google_cloud_run_v2_service.app[\"physician\"]";

/// The settings this root owns and a release must not change.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub(super) enum Setting {
    MinInstances,
    MaxInstances,
    CpuThrottling,
    VpcEgress,
}
impl Setting {
    pub const ALL: [Self; 4] = [
        Self::MinInstances,
        Self::MaxInstances,
        Self::CpuThrottling,
        Self::VpcEgress,
    ];
    fn annotation(self) -> &'static str {
        match self {
            Self::MinInstances => "autoscaling.knative.dev/minScale",
            Self::MaxInstances => "autoscaling.knative.dev/maxScale",
            Self::CpuThrottling => "run.googleapis.com/cpu-throttling",
            Self::VpcEgress => "run.googleapis.com/vpc-access-egress",
        }
    }
    /// Cloud Run omits an annotation that holds its default.
    fn default_value(self) -> Option<&'static str> {
        match self {
            Self::MinInstances => Some("0"),
            Self::MaxInstances => None,
            Self::CpuThrottling => Some("true"),
            Self::VpcEgress => Some("private-ranges-only"),
        }
    }
}

/// Expected values in the Cloud Run annotation vocabulary.
pub(super) type Expected = BTreeMap<Setting, String>;

/// Parse `tofu show -json PLAN` without retaining anything but the four settings.
pub(super) fn expected(show_json: &[u8]) -> Result<Expected> {
    let plan: Value = serde_json::from_slice(show_json)?;
    let resource = plan
        .pointer("/planned_values/root_module/resources")
        .and_then(Value::as_array)
        .and_then(|resources| {
            resources
                .iter()
                .find(|resource| resource.get("address").and_then(Value::as_str) == Some(ADDRESS))
        })
        .ok_or_else(|| refuse("reviewed plan has no physician Cloud Run service"))?;
    let template = resource
        .pointer("/values/template/0")
        .ok_or_else(|| refuse("physician service has no template"))?;
    let number = |pointer: &str| {
        template
            .pointer(pointer)
            .and_then(Value::as_u64)
            .map(|value| value.to_string())
    };
    let cpu_idle = product_container(template)?
        .pointer("/resources/0/cpu_idle")
        .and_then(Value::as_bool);
    let egress = template
        .pointer("/vpc_access/0/egress")
        .and_then(Value::as_str);
    let values = [
        (
            Setting::MinInstances,
            number("/scaling/0/min_instance_count"),
        ),
        (
            Setting::MaxInstances,
            number("/scaling/0/max_instance_count"),
        ),
        (
            Setting::CpuThrottling,
            cpu_idle.map(|idle| idle.to_string()),
        ),
        (
            Setting::VpcEgress,
            egress.and_then(|egress| match egress {
                "ALL_TRAFFIC" => Some("all-traffic".to_owned()),
                "PRIVATE_RANGES_ONLY" => Some("private-ranges-only".to_owned()),
                _ => None,
            }),
        ),
    ];
    values
        .into_iter()
        .map(|(setting, value)| {
            value
                .map(|value| (setting, value))
                .ok_or_else(|| refuse(format!("reviewed plan omits physician {setting:?}")))
        })
        .collect()
}

/// The planned physician product container: named `physician` beside the
/// otel-collector sidecar, else the single unnamed container.
fn product_container(template: &Value) -> Result<&Value> {
    let containers = template
        .get("containers")
        .and_then(Value::as_array)
        .map_or(&[][..], Vec::as_slice);
    fn name(container: &Value) -> Option<&str> {
        container
            .get("name")
            .and_then(Value::as_str)
            .filter(|name| !name.is_empty())
    }
    let mut named = containers.iter().filter(|c| name(c) == Some(SERVICE));
    match (named.next(), named.next(), containers) {
        (Some(product), None, _) => Ok(product),
        (None, _, [only]) if name(only).is_none() => Ok(only),
        _ => Err(refuse(
            "reviewed plan has no single physician product container",
        )),
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Service {
    spec: ServiceSpec,
    #[serde(default)]
    status: Status,
}
#[derive(Debug, Deserialize)]
struct ServiceSpec {
    template: Template,
}
#[derive(Debug, Deserialize)]
struct Template {
    #[serde(default)]
    metadata: Metadata,
}
#[derive(Debug, Default, Deserialize)]
struct Metadata {
    #[serde(default)]
    annotations: BTreeMap<String, String>,
}
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Status {
    #[serde(default)]
    conditions: Vec<Condition>,
    latest_created_revision_name: Option<String>,
    latest_ready_revision_name: Option<String>,
    #[serde(default)]
    traffic: Vec<Traffic>,
}
#[derive(Debug, Deserialize)]
struct Condition {
    #[serde(rename = "type")]
    kind: String,
    status: String,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(super) struct Traffic {
    #[serde(default)]
    pub revision_name: Option<String>,
    #[serde(default)]
    pub percent: u64,
    #[serde(default)]
    pub latest_revision: bool,
    #[serde(default)]
    pub tag: Option<String>,
}

impl Service {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        Ok(serde_json::from_slice(bytes)?)
    }
    fn ready(&self) -> bool {
        self.status
            .conditions
            .iter()
            .any(|condition| condition.kind == "Ready" && condition.status == "True")
    }
    /// A rollout is still settling when the newest revision is not yet Ready.
    pub fn settled(&self) -> bool {
        self.ready()
            && self.status.latest_created_revision_name.is_some()
            && self.status.latest_created_revision_name == self.status.latest_ready_revision_name
    }
    pub fn latest_ready(&self) -> Option<&str> {
        self.status.latest_ready_revision_name.as_deref()
    }
    fn observed(&self, setting: Setting) -> Option<String> {
        self.spec
            .template
            .metadata
            .annotations
            .get(setting.annotation())
            .cloned()
            .or_else(|| setting.default_value().map(str::to_owned))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "finding")]
pub(super) enum Finding {
    /// The no-change plan proposed changes: live infrastructure or state differs from the root.
    Drift,
    Setting {
        setting: Setting,
        expected: String,
        observed: Option<String>,
    },
    NotReady {
        latest_created: Option<String>,
        latest_ready: Option<String>,
    },
    Traffic {
        latest_ready: Option<String>,
        observed: Vec<Traffic>,
    },
}

/// Every difference between the reviewed settings and the live service.
pub(super) fn compare(expected: &Expected, service: &Service) -> Vec<Finding> {
    let mut findings: Vec<Finding> = Setting::ALL
        .into_iter()
        .filter_map(|setting| {
            let expected = expected.get(&setting)?;
            let observed = service.observed(setting);
            (observed.as_ref() != Some(expected)).then(|| Finding::Setting {
                setting,
                expected: expected.clone(),
                observed,
            })
        })
        .collect();
    if !service.settled() {
        findings.push(Finding::NotReady {
            latest_created: service.status.latest_created_revision_name.clone(),
            latest_ready: service.status.latest_ready_revision_name.clone(),
        });
    }
    let traffic = &service.status.traffic;
    let serving = |entry: &Traffic| {
        entry.latest_revision || entry.revision_name.as_deref() == service.latest_ready()
    };
    let intended = service.latest_ready().is_some()
        && traffic.iter().filter(|entry| entry.percent > 0).count() == 1
        && traffic
            .iter()
            .any(|entry| entry.percent == 100 && entry.tag.is_none() && serving(entry));
    if !intended {
        findings.push(Finding::Traffic {
            latest_ready: service.latest_ready().map(str::to_owned),
            observed: traffic.clone(),
        });
    }
    findings
}
