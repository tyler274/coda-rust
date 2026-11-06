"""Offline source-level ownership checks not represented by provider mocks."""
from pathlib import Path
import os
import subprocess
import re
import unittest

ROOT = Path(__file__).resolve().parents[1]
SERVICE_IGNORE = "ignore_changes = [template[0].containers[0].image, template[0].revision, traffic, client, client_version]"


def config_text(*names):
    """Executable HCL only: explanatory comments are dropped."""
    paths = [ROOT / name for name in names] if names else sorted(ROOT.glob("*.tf"))
    return "\n".join(line for path in paths for line in path.read_text().splitlines() if not line.lstrip().startswith("#"))


def resource_blocks(text, kind):
    """Map resource name -> body for every `resource "kind" "name" {` block."""
    blocks = {}
    for match in re.finditer(r'^resource "%s" "([^"]+)" \{\n(.*?)^\}' % re.escape(kind), text, re.M | re.S):
        blocks[match.group(1)] = match.group(2)
    return blocks


class Contract(unittest.TestCase):
    def test_release_drift_scope(self):
        text = (ROOT / "workloads.tf").read_text()
        # Both the app services and the physician-worker: release owns the image only.
        self.assertEqual(text.count(SERVICE_IGNORE), 2)
        self.assertIn("ignore_changes = [template[0].template[0].containers[0].image, client, client_version]", text)
        self.assertNotIn("ignore_changes = [template]", text)
        self.assertNotIn("ignore_changes = all", text)

    def test_release_targets_the_product_container_by_name(self):
        # staging-release (crates/workspace-tools/src/staging_release/shape.rs)
        # finds the product container by the
        # service name and the sidecar by "otel-collector"; index 0 stays the
        # product so ignore_changes covers exactly the release-owned image.
        workloads = config_text("workloads.tf")
        services = resource_blocks(workloads, "google_cloud_run_v2_service")
        self.assertIn("name  = local.observability ? each.key : null", services["app"])
        self.assertIn('name  = local.observability ? "physician-worker" : null', services["physician_worker"])
        self.assertIn('name                 = "physician-worker"', services["physician_worker"])
        self.assertIn('name                 = each.key', services["app"])
        sidecar = config_text("observability-cluster.tf")
        self.assertIn('name   = "otel-collector"', sidecar)
        shape = (ROOT.parents[2] / "crates/workspace-tools/src/staging_release/shape.rs").read_text()
        self.assertIn('const SIDECAR: &str = "otel-collector";', shape)
        for key in ["DD_ENV", "OTEL_EXPORTER_OTLP_ENDPOINT", "CODA_LOG_FORMAT", "CODA_TELEMETRY_CONTENT", "CODA_TELEMETRY_BACKEND"]:
            self.assertIn(f'"{key}"', shape)
            self.assertIn(key, sidecar + workloads)
        for key in ["OTEL_SIDECAR_CONFIG", "OTEL_GATEWAY_ENDPOINT", "OTEL_GATEWAY_AUDIENCE"]:
            self.assertIn(f'"{key}"', shape)
            self.assertIn(f'name  = "{key}"', workloads)

    def test_no_secret_values_or_public_access(self):
        text = "\n".join(path.read_text() for path in ROOT.glob("*.tf"))
        for forbidden in ["allUsers", "allAuthenticatedUsers", 'resource "google_secret_manager_secret_version"', 'data "google_secret_manager_secret_version"', 'resource "google_service_account_key"', 'role    = "roles/run.admin"']:
            # Ignore explanatory comments; only inspect executable HCL lines.
            config = "\n".join(line for line in text.splitlines() if not line.lstrip().startswith("#"))
            self.assertNotIn(forbidden, config)

    def test_billing_remains_outside_engineering_state(self):
        # A billing-account resource would require billing access even for a
        # routine refresh/plan by the project infrastructure administrator.
        config = "\n".join(path.read_text() for path in ROOT.glob("*.tf"))
        self.assertNotIn('resource "google_billing_budget"', config)

    def test_proxy_access_has_one_physician_only_owner(self):
        workloads = (ROOT / "workloads.tf").read_text()
        binding = workloads.split('resource "google_iap_web_cloud_run_service_iam_binding" "team" {', 1)[1].split('\n}', 1)[0]
        self.assertIn('for_each               = local.workloads', binding)
        self.assertIn('cloud_run_service_name = google_cloud_run_v2_service.app[each.key].name', binding)
        self.assertIn('each.key == "physician" ? var.physician_proxy_invokers : toset([])', binding)
        self.assertIn('toset(["group:${var.iap_group}"])', binding)
        variables = (ROOT / "variables.tf").read_text().split('variable "physician_proxy_invokers" {', 1)[1].split('\n}', 1)[0]
        self.assertIn('default     = []', variables)
        self.assertIn('^serviceAccount:', variables)
        self.assertIn('iamcredentials.googleapis.com', (ROOT / "foundation.tf").read_text())
        staging = (ROOT / "env/staging.tfvars").read_text()
        self.assertIn('physician_proxy_invokers = ["serviceAccount:public-site-staging@coda-health-staging.iam.gserviceaccount.com"]', staging)
        self.assertIn('CODA_HOSTED_PROFILE = "staging-preview"', staging)

    def test_public_proxy_preserves_service_only_release_authority(self):
        site = ROOT.parent / "public-site"
        main = (site / "main.tf").read_text()
        self.assertIn('timeout                          = "300s"', main)
        self.assertIn('PHYSICIAN_PROXY_SERVICE_ACCOUNT = google_service_account.site.email', main)
        self.assertIn('iap_enabled          = local.staging', main)
        self.assertRegex((site / "env/staging.tfvars").read_text(), r'(?m)^physician_app_url\s*= "https://physician-836508863523\.us-east1\.run\.app"$')
        production = (site / "env/production.tfvars").read_text()
        self.assertRegex(production, r'(?m)^environment\s*= "production"$')
        self.assertRegex(production, r'(?m)^project_id\s*= "coda-health-prod"$')
        self.assertNotRegex(production, r'(?m)^physician_app_url\s*=')
        backend = (site / "backend/production.gcs.tfbackend").read_text()
        self.assertIn('bucket = "coda-health-prod-public-site-tofu-state-us-east1"', backend)
        self.assertIn('prefix = "public-site/production"', backend)
        for forbidden in ['google_project_iam_member', 'google_project_service', 'google_secret_manager_secret_iam_member', 'google_iap_web_cloud_run_service_iam_member', 'google_service_account_iam_member']:
            self.assertNotIn(f'resource "{forbidden}"', main)
        bootstrap = (site / "scripts/staging-admin-bootstrap.sh").read_text()
        self.assertIn('iamcredentials.googleapis.com', bootstrap)
        self.assertEqual(bootstrap.count('--permissions=iam.serviceAccounts.signJwt --stage=GA'), 2)
        self.assertIn('if [[ $PHYSICIAN_PROXY -eq 1 ]]; then', bootstrap)
        self.assertIn('--physician-proxy is restricted to public-site-staging in coda-health-staging', bootstrap)
        self.assertIn('--member="serviceAccount:${RUNTIME_SA}" \\\n    --role="projects/${PROJECT_ID}/roles/${PROXY_ROLE}"', bootstrap)
        self.assertNotIn('roles/iam.serviceAccountTokenCreator', bootstrap)
        self.assertNotIn('iam.serviceAccounts.signBlob', bootstrap)
        workflow = (ROOT.parents[2] / ".github/workflows/ci.yml").read_text()
        self.assertIn('-var-file=env/staging.tfvars', workflow)
        self.assertIn('.address != "google_cloud_run_v2_service.site"', workflow)

    def test_proxy_bootstrap_rejects_production_before_cloud_calls(self):
        script = ROOT.parent / "public-site/scripts/staging-admin-bootstrap.sh"
        result = subprocess.run(
            ["bash", str(script), "--print", "--physician-proxy"],
            env={**os.environ, "PROJECT_ID": "coda-health-prod"},
            capture_output=True, text=True, check=False,
        )
        self.assertEqual(result.returncode, 1)
        self.assertIn("restricted to public-site-staging", result.stderr)
        self.assertEqual(result.stdout, "")

    def test_physician_scale_is_physician_only(self):
        # Every Redis, Pub/Sub and worker resource iterates physician_scale_apps
        # (or the physician_scale flag), so Duet can never receive one.
        text = config_text()
        scale_kinds = ["google_redis_instance", "google_pubsub_topic", "google_pubsub_subscription",
                       "google_pubsub_topic_iam_binding", "google_pubsub_subscription_iam_binding"]
        for kind in scale_kinds:
            blocks = resource_blocks(text, kind)
            self.assertTrue(blocks, kind)
            for name, body in blocks.items():
                with self.subTest(kind=kind, name=name):
                    self.assertIn("for_each = local.physician_scale_apps", re.sub(r"\s+", " ", body))
        # No scaling resource body names Duet, its identities or its slots.
        scale_kinds += ["google_service_account", "google_cloud_run_v2_service", "google_cloud_run_v2_service_iam_binding",
                        "google_service_account_iam_member", "google_project_iam_member", "google_project_service_identity"]
        for kind in scale_kinds:
            for name, body in resource_blocks(text, kind).items():
                if "physician_scale" not in body:
                    continue
                with self.subTest(kind=kind, scale_resource=name):
                    # Descriptions are prose; only references are checked.
                    references = "\n".join(line for line in body.splitlines() if "description" not in line)
                    self.assertNotIn("duet", references.lower())
        # Only physician is ever handed the chat topic; Duet has no PHYSICIAN_* env.
        workloads = config_text("workloads.tf")
        self.assertIn('for_each = each.key == "physician" ? google_pubsub_topic.chat_turns : {}', workloads)
        # Only physician keeps CPU between requests, with or without the
        # otel-collector sidecar: telemetry never makes Duet or the worker
        # bill for idle instance time.
        self.assertEqual(workloads.count('cpu_idle          = each.key != "physician"'), 2)
        self.assertNotRegex(workloads, r"cpu_idle\s+=.*observability")
        # Only physician keeps a minimum instance, and only while its queue exists.
        self.assertIn('min_instance_count = each.key == "physician" && length(google_pubsub_topic.chat_turns) > 0 ? 1 : 0', workloads)

    def test_worker_is_internal_and_invoked_only_by_push_and_probe(self):
        workloads = config_text("workloads.tf")
        worker = resource_blocks(workloads, "google_cloud_run_v2_service")["physician_worker"]
        self.assertRegex(worker, r'ingress\s+= "INGRESS_TRAFFIC_INTERNAL_ONLY"')
        self.assertRegex(worker, r"invoker_iam_disabled\s+= false")
        self.assertRegex(worker, r"iap_enabled\s+= false")
        self.assertEqual(len(re.findall(r"cpu_idle\s+= true\n", worker)), 2)
        invoker = resource_blocks(workloads, "google_cloud_run_v2_service_iam_binding")["physician_worker_invoker"]
        members = invoker.split("members", 1)[1]
        self.assertEqual(
            re.sub(r"\s+", " ", members).strip(),
            '= concat( ["serviceAccount:${google_service_account.physician_pubsub_push[each.key].email}"], '
            '[for identity in google_service_account.availability : "serviceAccount:${identity.email}"], )',
        )
        # No additive run.invoker grant may widen the authoritative binding.
        for kind in ["google_cloud_run_v2_service_iam_member", "google_cloud_run_service_iam_member", "google_project_iam_member"]:
            for name, body in resource_blocks(config_text(), kind).items():
                with self.subTest(kind=kind, name=name):
                    self.assertNotIn("roles/run.invoker", body)

    def test_redis_auth_never_leaves_sensitive_output(self):
        text = config_text()
        uses = [line for line in text.splitlines() if ".auth_string" in line]
        self.assertEqual(len(uses), 1, uses)
        output = re.search(r'output "physician_redis_auth_string" \{(.*?)^\}', text, re.M | re.S)
        self.assertIsNotNone(output)
        self.assertIn("sensitive   = true", output.group(1))
        # The Redis URL (which embeds AUTH) is Doppler-owned, never an infra env value.
        self.assertNotRegex(text, r'name\s*=\s*"PHYSICIAN_REDIS_(URL|CA_PEM)"')

    def test_single_isolated_root(self):
        versions = (ROOT / "versions.tf").read_text()
        self.assertIn('backend "gcs" {}', versions)
        self.assertNotIn('bucket =', versions)
        self.assertIn('version = "= 7.24.0"', versions)


if __name__ == "__main__":
    unittest.main()
