mock_provider "google" {}
mock_provider "google-beta" {}

variables {
  gpu_clinical_retrieval_image = "us-east1-docker.pkg.dev/coda-health-prod/coda-clinical-evidence/runtime-gpu@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
}

run "replicas_are_absent_outside_a_test_window" {
  command = plan

  assert {
    condition     = length(google_compute_region_instance_group_manager.clinical_serving) == 0 && length(google_compute_forwarding_rule.clinical_serving) == 0
    error_message = "No serving replica or load balancer may exist at zero replicas."
  }
}

run "two_replicas_serve_from_their_own_disks_behind_the_internal_load_balancer" {
  command = plan

  variables {
    clinical_serving_replica_count = 2
  }

  override_resource {
    target          = google_sql_database_instance.clinical_evidence
    override_during = plan
    values          = { connection_name = "coda-health-prod:us-east1:coda-clinical-evidence" }
  }

  override_resource {
    target          = google_service_account.clinical_retrieval
    override_during = plan
    values          = { email = "coda-clinical-retrieval@coda-health-prod.iam.gserviceaccount.com" }
  }

  override_resource {
    target          = google_compute_region_health_check.clinical_serving
    override_during = plan
    values          = { id = "projects/coda-health-prod/regions/us-east1/healthChecks/coda-clinical-serving-ready" }
  }

  override_resource {
    target          = google_compute_region_health_check.clinical_serving_live
    override_during = plan
    values          = { id = "projects/coda-health-prod/regions/us-east1/healthChecks/coda-clinical-serving-live" }
  }

  assert {
    condition     = google_compute_region_instance_group_manager.clinical_serving[0].target_size == 2 && length(google_compute_region_instance_group_manager.clinical_serving[0].distribution_policy_zones) == 2
    error_message = "Two replicas must spread across two zones."
  }

  assert {
    condition     = strcontains(google_compute_instance_template.clinical_serving[0].metadata["startup-script"], "CODA_CLINICAL_TABLE_MIRROR_ROOT=/var/lib/coda-clinical/table") && strcontains(google_compute_instance_template.clinical_serving[0].metadata["startup-script"], var.gpu_clinical_retrieval_image)
    error_message = "Each replica must run the pinned image and serve from its own disk."
  }

  assert {
    condition     = google_compute_region_instance_group_manager.clinical_serving[0].update_policy[0].max_unavailable_fixed == 0 && google_compute_region_health_check.clinical_serving_live.http_health_check[0].request_path == "/live"
    error_message = "Replacing replicas must never take serving ones away first, and a replacement counts only once it serves."
  }

  assert {
    condition     = google_compute_region_instance_group_manager.clinical_serving[0].auto_healing_policies[0].health_check == google_compute_region_health_check.clinical_serving_live.id
    error_message = "Auto-healing must use the liveness check: a replacement counts only once it serves."
  }

  assert {
    condition     = tolist(google_compute_region_backend_service.clinical_serving[0].health_checks) == tolist([google_compute_region_health_check.clinical_serving.id])
    error_message = "Traffic must follow the readiness check."
  }

  assert {
    condition     = google_compute_region_backend_service.clinical_serving[0].locality_lb_policy == "LEAST_REQUEST"
    error_message = "Searches must go to the replica with fewer in flight."
  }
}

run "reject_replicas_without_an_exact_image" {
  command = plan

  variables {
    gpu_clinical_retrieval_image   = null
    clinical_serving_replica_count = 2
  }

  expect_failures = [var.clinical_serving_replica_count]
}

run "reject_an_unmeasured_replica_count" {
  command = plan

  variables {
    clinical_serving_replica_count = 1
  }

  expect_failures = [var.clinical_serving_replica_count]
}

run "reject_zones_outside_the_bucket_region" {
  command = plan

  variables {
    clinical_serving_zones = ["us-east1-b", "us-east4-a"]
  }

  expect_failures = [var.clinical_serving_zones]
}

run "reject_a_moving_boot_image_family" {
  command = plan

  variables {
    clinical_serving_boot_image = "projects/deeplearning-platform-release/global/images/family/common-cu129-ubuntu-2204-nvidia-580"
  }

  expect_failures = [var.clinical_serving_boot_image]
}
