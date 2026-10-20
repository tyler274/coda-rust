"""Offline source-level ownership checks not represented by provider mocks."""
from pathlib import Path
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

    def test_single_isolated_root(self):
        versions = (ROOT / "versions.tf").read_text()
        self.assertIn('backend "gcs" {}', versions)
        self.assertNotIn('bucket =', versions)
        self.assertIn('version = "= 7.24.0"', versions)


if __name__ == "__main__":
    unittest.main()
