mock_provider "google" {}
mock_provider "google-beta" {}

run "reject_non_cloud_run_clinical_evidence_url" {
  command = plan

  variables {
    clinical_evidence_service_url = "https://example.com"
  }

  expect_failures = [var.clinical_evidence_service_url]
}

run "require_clinical_evidence_url_for_application" {
  command = plan

  variables {
    application_image           = "us-east1-docker.pkg.dev/coda-health-staging/coda-agents/application@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    application_service_enabled = true
  }

  expect_failures = [var.application_service_enabled]
}
