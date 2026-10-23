"""Offline source-level ownership checks not represented by provider mocks."""
from pathlib import Path
import os
import subprocess
import unittest

ROOT = Path(__file__).resolve().parents[1]


class Contract(unittest.TestCase):
    def test_release_drift_scope(self):
        text = (ROOT / "workloads.tf").read_text()
        self.assertIn("ignore_changes = [template[0].containers[0].image, template[0].revision, traffic, client, client_version]", text)
        self.assertIn("ignore_changes = [template[0].template[0].containers[0].image, client, client_version]", text)
        self.assertNotIn("ignore_changes = [template]", text)
        self.assertNotIn("ignore_changes = all", text)

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
        self.assertIn('${var.physician_proxy_enabled ? ",PHYSICIAN_PROXY_SERVICE_ACCOUNT" : ""}', main)
        self.assertIn('iap_enabled          = local.staging', main)
        self.assertIn('physician_proxy_enabled      = true', (site / "env/staging.tfvars").read_text())
        for forbidden in ['google_project_iam_member', 'google_project_service', 'google_iap_web_cloud_run_service_iam_member', 'google_service_account_iam_member']:
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

    def test_single_isolated_root(self):
        versions = (ROOT / "versions.tf").read_text()
        self.assertIn('backend "gcs" {}', versions)
        self.assertNotIn('bucket =', versions)
        self.assertIn('version = "= 7.24.0"', versions)


if __name__ == "__main__":
    unittest.main()
