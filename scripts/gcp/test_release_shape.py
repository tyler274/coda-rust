"""Offline checks for release_shape.py; no gcloud, credentials or network.

    python -m unittest discover -s scripts/gcp -p 'test_release*.py' -v
"""
from copy import deepcopy
import io
import json
from contextlib import redirect_stderr, redirect_stdout
from pathlib import Path
import subprocess
import tempfile
import unittest

from release_shape import ShapeError, expected, main, target, verify

ROOT = Path(__file__).resolve().parents[2]
OLD = "us-east1-docker.pkg.dev/p/apps/physician@sha256:" + "a" * 64
NEW = "us-east1-docker.pkg.dev/p/apps/physician@sha256:" + "b" * 64


def single(image=OLD):
    """The shape Cloud Run serves today: one unnamed container."""
    return {"spec": {"template": {
        "metadata": {"annotations": {"run.googleapis.com/cpu-throttling": "false", "autoscaling.knative.dev/minScale": "1",
                                     "run.googleapis.com/client-name": "gcloud"},
                     "labels": {"client.knative.dev/nonce": "one"}},
        "spec": {"containers": [{"image": image, "ports": [{"containerPort": 8080}],
                                 "env": [{"name": "DD_ENV", "value": "staging"}]}]},
    }}}


def observed(image=OLD):
    """The shape rust-environments produces with observability on."""
    service = single(image)
    product = service["spec"]["template"]["spec"]["containers"][0]
    product["name"] = "physician"
    product["env"] += [
        {"name": "OTEL_EXPORTER_OTLP_ENDPOINT", "value": "http://localhost:4318"},
        {"name": "CODA_LOG_FORMAT", "value": "json"},
        {"name": "CODA_TELEMETRY_CONTENT", "value": "none"},
        {"name": "CODA_TELEMETRY_BACKEND", "value": "datadog"},
    ]
    # Cloud Run may list the sidecar first; order must not matter.
    service["spec"]["template"]["spec"]["containers"].insert(0, {
        "name": "otel-collector", "image": "otel/opentelemetry-collector-contrib:0.155.0",
        "env": [{"name": "OTEL_SIDECAR_CONFIG", "value": "receivers: {}"},
                {"name": "OTEL_GATEWAY_ENDPOINT", "value": "http://10.82.0.2:4318"},
                {"name": "OTEL_GATEWAY_AUDIENCE", "value": "coda-otel-gateway-stg"}],
    })
    return service


def released(service, image=NEW, name="physician"):
    """What `gcloud run services update --container NAME --image` leaves behind."""
    after = deepcopy(service)
    template = after["spec"]["template"]
    template["metadata"].setdefault("labels", {})["client.knative.dev/nonce"] = "two"
    template["metadata"]["annotations"]["run.googleapis.com/client-version"] = "591.0.0"
    for container in template["spec"]["containers"]:
        if container.get("name") in (name, None):
            container["image"] = image
    return after


def containers(service):
    return service["spec"]["template"]["spec"]["containers"]


class Expected(unittest.TestCase):
    def test_needs_both_gates(self):
        self.assertEqual(expected("observability_cluster_enabled = true\nobservability_enabled = true\n"), "on")
        self.assertEqual(expected("observability_cluster_enabled = true\nobservability_enabled = false\n"), "off")
        self.assertEqual(expected("observability_enabled = true\n"), "off")
        self.assertEqual(expected("# observability_enabled = true\nobservability_cluster_enabled = true\n"), "off")

    def test_committed_staging_turns_observability_on(self):
        text = (ROOT / "infra/gcp/rust-environments/env/staging.tfvars").read_text()
        self.assertEqual(expected(text), "on")


class Target(unittest.TestCase):
    def test_observed_service_targets_the_named_product(self):
        self.assertEqual(target(observed(), "physician", "on"), ("physician", None))

    def test_single_container_targets_the_default_container(self):
        self.assertEqual(target(single(), "physician", "off"), ("", None))

    def test_unapplied_plan_releases_image_only_with_a_warning(self):
        # Today's staging: tfvars on, services still single-container.
        container, warning = target(single(), "physician", "on")
        self.assertEqual(container, "")
        self.assertIn("apply the reviewed infra/gcp/rust-environments plan", warning)

    def test_sidecar_without_telemetry_env_is_refused(self):
        for key in ("DD_ENV", "OTEL_EXPORTER_OTLP_ENDPOINT", "CODA_TELEMETRY_CONTENT", "CODA_TELEMETRY_BACKEND"):
            service = observed()
            product = next(c for c in containers(service) if c.get("name") == "physician")
            product["env"] = [e for e in product["env"] if e["name"] != key]
            with self.subTest(key=key), self.assertRaisesRegex(ShapeError, key):
                target(service, "physician", "on")

    def test_content_must_stay_off(self):
        service = observed()
        product = next(c for c in containers(service) if c.get("name") == "physician")
        next(e for e in product["env"] if e["name"] == "CODA_TELEMETRY_CONTENT")["value"] = "full"
        with self.assertRaisesRegex(ShapeError, "CODA_TELEMETRY_CONTENT=none"):
            target(service, "physician", "on")

    def test_sidecar_without_gateway_is_refused(self):
        service = observed()
        containers(service)[0]["env"].pop()
        with self.assertRaisesRegex(ShapeError, "OTEL_GATEWAY_AUDIENCE"):
            target(service, "physician", "on")

    def test_unexpected_container_set_is_refused(self):
        service = observed()
        containers(service)[1]["name"] = "app"
        with self.assertRaisesRegex(ShapeError, "expected containers"):
            target(service, "physician", "on")
        service = single()
        containers(service).append({"name": "extra", "image": "x"})
        with self.assertRaises(ShapeError):
            target(service, "physician", "off")


class Verify(unittest.TestCase):
    def test_image_only_release_passes(self):
        for before in (observed(), single()):
            with self.subTest(sidecar=len(containers(before)) == 2):
                verify(before, released(before), "physician", NEW)

    def test_first_client_label_on_a_template_without_labels_passes(self):
        # gcloud adds the nonce label even when the template had no labels.
        for before in (observed(), single()):
            del before["spec"]["template"]["metadata"]["labels"]
            with self.subTest(sidecar=len(containers(before)) == 2):
                verify(before, released(before), "physician", NEW)

    def test_dropped_sidecar_fails(self):
        before = observed()
        after = released(before)
        containers(after).pop(0)
        product = containers(after)[0]
        product.pop("name")
        with self.assertRaisesRegex(ShapeError, "container set"):
            verify(before, after, "physician", NEW)

    def test_sidecar_image_change_fails(self):
        before = observed()
        after = released(before)
        containers(after)[0]["image"] = NEW
        with self.assertRaisesRegex(ShapeError, "infrastructure-owned"):
            verify(before, after, "physician", NEW)

    def test_cpu_or_scaling_drift_fails(self):
        for key, value in (("run.googleapis.com/cpu-throttling", "true"), ("autoscaling.knative.dev/minScale", "0")):
            before = observed()
            after = released(before)
            after["spec"]["template"]["metadata"]["annotations"][key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ShapeError, "infrastructure-owned"):
                verify(before, after, "physician", NEW)

    def test_new_non_client_label_fails(self):
        before = observed()
        del before["spec"]["template"]["metadata"]["labels"]
        after = released(before)
        after["spec"]["template"]["metadata"]["labels"]["team"] = "other"
        with self.assertRaisesRegex(ShapeError, "infrastructure-owned"):
            verify(before, after, "physician", NEW)

    def test_wrong_image_fails(self):
        before = observed()
        with self.assertRaisesRegex(ShapeError, "expected"):
            verify(before, released(before, image=OLD), "physician", NEW)


class Cli(unittest.TestCase):
    def run_main(self, *args):
        out, err = io.StringIO(), io.StringIO()
        with redirect_stdout(out), redirect_stderr(err):
            code = main(list(args))
        return code, out.getvalue(), err.getvalue()

    def test_cli_round_trip(self):
        with tempfile.TemporaryDirectory() as directory:
            before, after = Path(directory, "before.json"), Path(directory, "after.json")
            before.write_text(json.dumps(single()))
            after.write_text(json.dumps(released(single())))
            code, out, err = self.run_main("target", str(before), "physician", "on")
            self.assertEqual((code, out), (0, "\n"))
            self.assertIn("::warning::", err)
            self.assertEqual(self.run_main("verify", str(before), str(after), "physician", NEW)[0], 0)
            code, _, err = self.run_main("verify", str(before), str(before), "physician", NEW)
            self.assertEqual(code, 1)
            self.assertIn("::error::", err)

    def test_rollout_script_updates_the_product_container_by_name(self):
        script = (ROOT / "scripts/gcp/rollout-service.sh").read_text()
        self.assertIn('services update "$service" --container "$container" --image "$image"', script)
        self.assertEqual(script.count("services update"), 2)
        workflow = (ROOT / ".github/workflows/ci.yml").read_text()
        deploy = workflow.split("  deploy-staging:", 1)[1].split("\n  observability:", 1)[0]
        self.assertNotIn("services update", deploy)
        self.assertIn('scripts/gcp/rollout-service.sh check "$app" release', deploy)
        self.assertIn('scripts/gcp/rollout-service.sh roll "$app" "$runtime" release', deploy)
        self.assertIn('scripts/gcp/rollout-service.sh roll physician-worker "$worker" release', deploy)
        self.assertNotIn("containers[0].image", deploy)

    def test_rollout_script_parses(self):
        subprocess.run(["bash", "-n", str(ROOT / "scripts/gcp/rollout-service.sh")], check=True)


if __name__ == "__main__":
    unittest.main()
