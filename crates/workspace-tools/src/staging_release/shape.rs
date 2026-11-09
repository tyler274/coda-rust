//! The Cloud Run template shapes a release may roll.
//!
//! [`rust-environments`](/infra/gcp/rust-environments) owns every service
//! field except the product container's image. With observability on, a
//! service runs two containers: the product, named after the service, and the
//! `otel-collector` sidecar. With it off, one unnamed product container. The
//! observability relay runs one container named after the service. Any other
//! shape is refused before anything is migrated or rolled out, and after the
//! rollout nothing but the product image may have moved.
use super::{Error, LocalCode, Result};
use serde_json::{Map, Value};

const SIDECAR: &str = "otel-collector";
/// Its single container is named, so the release targets it by name.
const NAMED_SINGLE: &str = "observability-relay";
/// Set on the product container by observability-cluster.tf (`DD_ENV` and
/// `observability_product_env`), with the values each may take. An empty list
/// accepts any nonempty value. `CODA_TELEMETRY_CONTENT` is either policy:
/// rust-environments' `telemetry_content` chooses it, and the gateway strips
/// content from every pipeline but Langfuse's.
const PRODUCT_ENV: [(&str, &[&str]); 5] = [
    ("DD_ENV", &[]),
    ("OTEL_EXPORTER_OTLP_ENDPOINT", &["http://localhost:4318"]),
    ("CODA_LOG_FORMAT", &["json"]),
    ("CODA_TELEMETRY_CONTENT", &["none", "full"]),
    ("CODA_TELEMETRY_BACKEND", &[]),
];
const SIDECAR_ENV: [&str; 3] = [
    "OTEL_SIDECAR_CONFIG",
    "OTEL_GATEWAY_ENDPOINT",
    "OTEL_GATEWAY_AUDIENCE",
];
/// Written by every gcloud update; not infrastructure configuration.
const CLIENT_METADATA: [(&str, &[&str]); 2] = [
    (
        "annotations",
        &[
            "run.googleapis.com/client-name",
            "run.googleapis.com/client-version",
        ],
    ),
    ("labels", &["client.knative.dev/nonce"]),
];

/// A well-formed service template as the release sees it. Never serialized:
/// the template carries environment values that do not belong in receipts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Shape {
    /// The `--container` to update; `None` for the single unnamed container.
    container: Option<String>,
    sidecar: bool,
    /// The template without the product image and gcloud client metadata.
    infrastructure: Value,
}

fn containers(spec: &Value) -> &[Value] {
    spec.get("containers")
        .and_then(Value::as_array)
        .map_or(&[], Vec::as_slice)
}
/// A container's name; an empty one is the same as none, as gcloud prints it.
fn name(container: &Value) -> Option<&str> {
    container
        .get("name")
        .and_then(Value::as_str)
        .filter(|name| !name.is_empty())
}
fn env<'a>(container: &'a Value, key: &str) -> Option<&'a str> {
    container
        .get("env")
        .and_then(Value::as_array)?
        .iter()
        .find(|variable| name(variable) == Some(key))?
        .get("value")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
}

/// The product container of a pod spec (a service template's or a revision's
/// `spec`): the one named after the service, else the single unnamed one.
pub(super) fn product<'a>(spec: &'a Value, service: &str) -> Result<&'a Value> {
    Ok(&containers(spec)[product_index(spec, service)?])
}
fn product_index(spec: &Value, service: &str) -> Result<usize> {
    let found = containers(spec);
    let mut named = (0..found.len()).filter(|&index| name(&found[index]) == Some(service));
    match (named.next(), named.next(), found) {
        (Some(index), None, _) => Ok(index),
        (None, _, [only]) if name(only).is_none() => Ok(0),
        _ => Err(Error::Cloud(LocalCode::ProductContainerNotFound)),
    }
}

/// Reads a service's `spec.template`, refusing a shape infrastructure would
/// not produce.
pub(super) fn read(template: &Value, service: &str) -> Result<Shape> {
    let spec = template
        .get("spec")
        .ok_or(Error::Cloud(LocalCode::MissingCloudField))?;
    let found = containers(spec);
    let sidecar = found.iter().any(|c| name(c) == Some(SIDECAR));
    if sidecar {
        let mut names: Vec<_> = found.iter().map(name).collect();
        names.sort_unstable();
        let mut wanted = [Some(service), Some(SIDECAR)];
        wanted.sort_unstable();
        if names != wanted {
            return Err(Error::Cloud(LocalCode::UnexpectedContainerSet));
        }
        let product = product(spec, service)?;
        let telemetry = PRODUCT_ENV.iter().all(|(key, wanted)| {
            env(product, key).is_some_and(|value| wanted.is_empty() || wanted.contains(&value))
        });
        let collector = found
            .iter()
            .find(|c| name(c) == Some(SIDECAR))
            .ok_or(Error::Cloud(LocalCode::UnexpectedContainerSet))?;
        let gateway = SIDECAR_ENV.iter().all(|key| env(collector, key).is_some());
        let ingress = collector
            .get("ports")
            .and_then(Value::as_array)
            .is_some_and(|ports| !ports.is_empty());
        if !telemetry || !gateway || ingress {
            return Err(Error::Cloud(LocalCode::TelemetryContractIncomplete));
        }
    } else {
        let wanted = (service == NAMED_SINGLE).then_some(service);
        if !matches!(found, [only] if name(only) == wanted) {
            return Err(Error::Cloud(LocalCode::UnexpectedContainerSet));
        }
    }
    let named = sidecar || service == NAMED_SINGLE;
    Ok(Shape {
        container: named.then(|| service.to_owned()),
        sidecar,
        infrastructure: infrastructure_view(template, service)?,
    })
}

fn infrastructure_view(template: &Value, service: &str) -> Result<Value> {
    let mut view = template.clone();
    if let Some(metadata) = view.get_mut("metadata").and_then(Value::as_object_mut) {
        for (field, keys) in CLIENT_METADATA {
            if let Some(map) = metadata.get_mut(field).and_then(Value::as_object_mut) {
                for key in keys {
                    map.remove(*key);
                }
            }
            // gcloud may add the first label to a template that had none, so
            // an absent map and an empty one are the same configuration.
            if metadata
                .get(field)
                .and_then(Value::as_object)
                .is_some_and(Map::is_empty)
            {
                metadata.remove(field);
            }
        }
        metadata.remove("name");
    }
    let spec = view
        .get_mut("spec")
        .ok_or(Error::Cloud(LocalCode::MissingCloudField))?;
    let index = product_index(spec, service)?;
    if let Some(product) = spec
        .get_mut("containers")
        .and_then(|containers| containers.get_mut(index))
        .and_then(Value::as_object_mut)
    {
        product.remove("image");
    }
    Ok(view)
}

impl Shape {
    pub fn container(&self) -> Option<&str> {
        self.container.as_deref()
    }
    /// After a rollout: the container set and every infrastructure-owned
    /// field are as they were before it.
    pub fn preserves(&self, before: &Self) -> Result<()> {
        if self.sidecar != before.sidecar || self.container != before.container {
            return Err(Error::Cloud(LocalCode::ContainerSetChanged));
        }
        if self.infrastructure != before.infrastructure {
            return Err(Error::Cloud(LocalCode::InfrastructureFieldChanged));
        }
        Ok(())
    }
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;
    use serde_json::json;

    pub const OLD: &str = "old-image";
    pub const NEW: &str = "new-image";

    /// The shape Cloud Run serves with observability off: one unnamed container.
    pub fn single(image: &str) -> Value {
        json!({
            "metadata": {
                "annotations": {
                    "run.googleapis.com/cpu-throttling": "false",
                    "autoscaling.knative.dev/minScale": "1",
                    "run.googleapis.com/client-name": "gcloud"
                },
                "labels": {"client.knative.dev/nonce": "one"}
            },
            "spec": {"containers": [{
                "image": image,
                "ports": [{"containerPort": 8080}],
                "env": [{"name": "DD_ENV", "value": "staging"}]
            }]}
        })
    }
    /// The relay's shape: one container, named after its service.
    pub fn named(service: &str, image: &str) -> Value {
        let mut template = single(image);
        template["spec"]["containers"][0]["name"] = service.into();
        template
    }
    /// The shape rust-environments produces with observability on. Cloud Run
    /// may list the sidecar first; order must not matter.
    pub fn observed(service: &str, image: &str) -> Value {
        let mut template = single(image);
        let product = &mut template["spec"]["containers"][0];
        product["name"] = service.into();
        for (key, value) in [
            ("OTEL_EXPORTER_OTLP_ENDPOINT", "http://localhost:4318"),
            ("CODA_LOG_FORMAT", "json"),
            ("CODA_TELEMETRY_CONTENT", "none"),
            ("CODA_TELEMETRY_BACKEND", "both"),
        ] {
            product["env"]
                .as_array_mut()
                .expect("env")
                .push(json!({"name": key, "value": value}));
        }
        template["spec"]["containers"]
            .as_array_mut()
            .expect("containers")
            .insert(
                0,
                json!({
                    "name": SIDECAR,
                    "image": "otel/opentelemetry-collector-contrib:0.155.0",
                    "env": [
                        {"name": "OTEL_SIDECAR_CONFIG", "value": "receivers: {}"},
                        {"name": "OTEL_GATEWAY_ENDPOINT", "value": "http://10.82.0.2:4318"},
                        {"name": "OTEL_GATEWAY_AUDIENCE", "value": "coda-otel-gateway-stg"}
                    ]
                }),
            );
        template
    }
    /// What `gcloud run services update [--container NAME] --image` leaves behind.
    pub fn released(template: &Value, service: &str, image: &str) -> Value {
        let mut after = template.clone();
        let metadata = after["metadata"].as_object_mut().expect("metadata");
        metadata
            .entry("labels")
            .or_insert_with(|| json!({}))
            .as_object_mut()
            .expect("labels")
            .insert("client.knative.dev/nonce".into(), "two".into());
        metadata["annotations"]["run.googleapis.com/client-version"] = "591.0.0".into();
        for container in after["spec"]["containers"]
            .as_array_mut()
            .expect("containers")
        {
            if name(container).is_none_or(|name| name == service) {
                container["image"] = image.into();
            }
        }
        after
    }
    fn product_of<'a>(template: &'a mut Value, service: &str) -> &'a mut Value {
        template["spec"]["containers"]
            .as_array_mut()
            .expect("containers")
            .iter_mut()
            .find(|c| name(c) == Some(service))
            .expect("product")
    }
    fn code(result: Result<impl std::fmt::Debug>) -> &'static str {
        result.expect_err("refused").safe_code().as_str()
    }
    fn verify(before: &Value, after: &Value) -> Result<()> {
        read(after, "physician")?.preserves(&read(before, "physician")?)
    }

    #[test]
    fn two_container_service_targets_the_named_product() {
        let template = observed("physician", OLD);
        let shape = read(&template, "physician").expect("shape");
        assert_eq!(shape.container(), Some("physician"));
        assert_eq!(
            product(&template["spec"], "physician").expect("product")["image"],
            OLD
        );
    }
    #[test]
    fn single_container_targets_the_default_container() {
        let template = single(OLD);
        assert_eq!(
            read(&template, "physician").expect("shape").container(),
            None
        );
        assert_eq!(
            product(&template["spec"], "physician").expect("product")["image"],
            OLD
        );
    }
    #[test]
    fn single_named_container_is_targeted_by_name() {
        let template = named("observability-relay", OLD);
        assert_eq!(
            read(&template, "observability-relay")
                .expect("shape")
                .container(),
            Some("observability-relay")
        );
        let after = released(&template, "observability-relay", NEW);
        read(&after, "observability-relay")
            .expect("after")
            .preserves(&read(&template, "observability-relay").expect("before"))
            .expect("preserved");
        for template in [single(OLD), named("other", OLD)] {
            assert_eq!(
                code(read(&template, "observability-relay")),
                "unexpected-container-set"
            );
        }
    }
    #[test]
    fn sidecar_without_telemetry_env_is_refused() {
        for key in [
            "DD_ENV",
            "OTEL_EXPORTER_OTLP_ENDPOINT",
            "CODA_TELEMETRY_CONTENT",
            "CODA_TELEMETRY_BACKEND",
        ] {
            let mut template = observed("physician", OLD);
            product_of(&mut template, "physician")["env"]
                .as_array_mut()
                .expect("env")
                .retain(|variable| name(variable) != Some(key));
            assert_eq!(
                code(read(&template, "physician")),
                "telemetry-contract-incomplete",
                "{key}"
            );
        }
    }
    #[test]
    fn content_is_a_known_policy() {
        let with_content = |policy: &str| {
            let mut template = observed("physician", OLD);
            for variable in product_of(&mut template, "physician")["env"]
                .as_array_mut()
                .expect("env")
            {
                if name(variable) == Some("CODA_TELEMETRY_CONTENT") {
                    variable["value"] = policy.into();
                }
            }
            template
        };
        for policy in ["none", "full"] {
            assert!(read(&with_content(policy), "physician").is_ok(), "{policy}");
        }
        assert_eq!(
            code(read(&with_content("prompts"), "physician")),
            "telemetry-contract-incomplete"
        );
    }
    #[test]
    fn sidecar_without_gateway_or_with_ingress_is_refused() {
        let mut template = observed("physician", OLD);
        template["spec"]["containers"][0]["env"]
            .as_array_mut()
            .expect("env")
            .pop();
        assert_eq!(
            code(read(&template, "physician")),
            "telemetry-contract-incomplete"
        );
        let mut template = observed("physician", OLD);
        template["spec"]["containers"][0]["ports"] = json!([{"containerPort": 8080}]);
        assert_eq!(
            code(read(&template, "physician")),
            "telemetry-contract-incomplete"
        );
    }
    #[test]
    fn wrong_or_unexpected_containers_are_refused() {
        // The product is not named after the service: the image would land
        // on a container the release cannot identify.
        let mut template = observed("physician", OLD);
        template["spec"]["containers"][1]["name"] = "app".into();
        assert_eq!(
            code(read(&template, "physician")),
            "unexpected-container-set"
        );
        assert_eq!(
            code(product(&template["spec"], "physician")),
            "product-container-not-found"
        );
        let mut template = single(OLD);
        template["spec"]["containers"]
            .as_array_mut()
            .expect("containers")
            .push(json!({"name": "extra", "image": "x"}));
        assert_eq!(
            code(read(&template, "physician")),
            "unexpected-container-set"
        );
        let mut template = single(OLD);
        template["spec"]["containers"][0]["name"] = "physician".into();
        assert_eq!(
            code(read(&template, "physician")),
            "unexpected-container-set"
        );
    }
    #[test]
    fn image_only_release_passes() {
        for before in [observed("physician", OLD), single(OLD)] {
            verify(&before, &released(&before, "physician", NEW)).expect("image only");
        }
    }
    #[test]
    fn first_client_label_on_a_template_without_labels_passes() {
        for mut before in [observed("physician", OLD), single(OLD)] {
            before["metadata"]
                .as_object_mut()
                .expect("metadata")
                .remove("labels");
            verify(&before, &released(&before, "physician", NEW)).expect("client label");
        }
    }
    #[test]
    fn dropped_sidecar_fails() {
        let before = observed("physician", OLD);
        let mut after = released(&before, "physician", NEW);
        let containers = after["spec"]["containers"]
            .as_array_mut()
            .expect("containers");
        containers.remove(0);
        containers[0]
            .as_object_mut()
            .expect("product")
            .remove("name");
        assert_eq!(code(verify(&before, &after)), "container-set-changed");
    }
    #[test]
    fn sidecar_image_change_fails() {
        let before = observed("physician", OLD);
        let mut after = released(&before, "physician", NEW);
        after["spec"]["containers"][0]["image"] = NEW.into();
        assert_eq!(
            code(verify(&before, &after)),
            "infrastructure-field-changed"
        );
    }
    #[test]
    fn cpu_or_scaling_drift_fails() {
        for (key, value) in [
            ("run.googleapis.com/cpu-throttling", "true"),
            ("autoscaling.knative.dev/minScale", "0"),
        ] {
            let before = observed("physician", OLD);
            let mut after = released(&before, "physician", NEW);
            after["metadata"]["annotations"][key] = value.into();
            assert_eq!(
                code(verify(&before, &after)),
                "infrastructure-field-changed",
                "{key}"
            );
        }
    }
    #[test]
    fn new_non_client_label_fails() {
        let mut before = observed("physician", OLD);
        before["metadata"]
            .as_object_mut()
            .expect("metadata")
            .remove("labels");
        let mut after = released(&before, "physician", NEW);
        after["metadata"]["labels"]["team"] = "other".into();
        assert_eq!(
            code(verify(&before, &after)),
            "infrastructure-field-changed"
        );
    }
}
