mock_provider "google" {}
mock_provider "google-beta" {}

run "reject_anonymous_invoker" {
  command = plan

  variables {
    clinical_evidence_invokers = ["allUsers"]
  }

  expect_failures = [var.clinical_evidence_invokers]
}

run "accept_explicit_invoker_classes" {
  command = plan

  variables {
    clinical_evidence_invokers = [
      "serviceAccount:coda-application@coda-health-prod.iam.gserviceaccount.com",
      "user:dev@codahealth.ai",
      "group:engineering@codahealth.ai",
    ]
  }
}
