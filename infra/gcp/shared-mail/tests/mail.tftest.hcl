mock_provider "google" {}

run "prepare_before_first_image" {
  command = plan
  assert {
    condition     = length(google_cloud_run_v2_service.mail) == 0 && length(google_cloud_run_v2_service_iam_binding.mail) == 0
    error_message = "Preparing the image registry and egress must not activate mail delivery."
  }
}

run "production_delivery_boundary" {
  command = plan
  variables {
    image = "us-east1-docker.pkg.dev/coda-health-prod/shared-mail/mail-relay@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
  }
  assert {
    condition     = !google_cloud_run_v2_service.mail[0].invoker_iam_disabled && google_cloud_run_v2_service_iam_binding.mail[0].members == toset(["serviceAccount:workspace-web@coda-gym.iam.gserviceaccount.com"])
    error_message = "Only Gym's production identity may invoke this deployment; no public or staging caller."
  }
  assert {
    condition     = google_cloud_run_v2_service.mail[0].template[0].vpc_access[0].egress == "ALL_TRAFFIC" && google_compute_router_nat.mail.nat_ip_allocate_option == "MANUAL_ONLY" && length(google_compute_router_nat.mail.nat_ips) == 1
    error_message = "SMTP must leave through the single Workspace-allowlisted static address."
  }
}

run "reject_staging_image" {
  command = plan
  variables {
    image = "us-east1-docker.pkg.dev/coda-rust-staging/apps/mail-relay@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
  }
  expect_failures = [var.image]
}

run "reject_public_invocation" {
  command = plan
  variables { invokers = ["allUsers"] }
  expect_failures = [var.invokers]
}
