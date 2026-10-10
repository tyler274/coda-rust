mock_provider "google" {}
mock_provider "google-beta" {}

variables {
  clinical_query_model_archive = {
    object = "builds/sha256/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/MedCPT-Query-Encoder.tar.gz"
    sha256 = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
  }
  clinical_cross_encoder_archive = {
    object = "builds/sha256/bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb/MedCPT-Cross-Encoder.tar.gz"
    sha256 = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
  }
}

run "reject_enabled_acquisition_without_image_and_baseline" {
  command = plan

  variables {
    clinical_source_acquisition_enabled = true
  }

  expect_failures = [var.clinical_source_acquisition_enabled]
}

run "reject_enabled_retrieval_build_without_exact_inputs" {
  command = plan

  variables {
    clinical_retrieval_build_enabled = true
  }

  expect_failures = [var.clinical_retrieval_build_enabled]
}

run "accept_bounded_scale_to_zero_retrieval_build" {
  command = plan

  variables {
    clinical_retrieval_build_enabled = true
    clinical_retrieval_build_image   = "us-east1-docker.pkg.dev/coda-health-prod/coda-clinical-evidence/runtime-gpu@sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    clinical_retrieval_max_passages  = 15000000
    clinical_document_model_archive = {
      object         = "builds/sha256/cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc/MedCPT-Article-Encoder.tar.gz"
      sha256         = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
      weights_sha256 = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
    }
  }

  assert {
    condition     = google_cloud_run_v2_job.clinical_retrieval_builder[0].template[0].task_count == 1
    error_message = "Clinical retrieval build must remain one exact task."
  }

  assert {
    condition     = google_cloud_run_v2_job.clinical_retrieval_builder[0].template[0].template[0].max_retries == 0
    error_message = "Clinical retrieval build must not create a paid retry loop."
  }

  assert {
    condition     = google_cloud_run_v2_job.clinical_retrieval_builder[0].template[0].template[0].timeout == "1800s"
    error_message = "Clinical retrieval build must have one thirty-minute hard ceiling."
  }

  assert {
    condition     = google_cloud_run_v2_job.clinical_retrieval_builder[0].template[0].template[0].containers[0].resources[0].limits["cpu"] == "8"
    error_message = "Clinical retrieval build must stay within the reviewed CPU ceiling."
  }

  assert {
    condition     = google_cloud_run_v2_job.clinical_retrieval_builder[0].template[0].template[0].containers[0].resources[0].limits["memory"] == "32Gi"
    error_message = "Clinical retrieval build must stay within the measured current-scale memory envelope."
  }

  assert {
    condition     = google_cloud_run_v2_job.clinical_retrieval_builder[0].location == var.gpu_region
    error_message = "Clinical retrieval build must run in the configured GPU region."
  }

  assert {
    condition     = google_cloud_run_v2_job.clinical_retrieval_builder[0].template[0].template[0].containers[0].resources[0].limits["nvidia.com/gpu"] == "1"
    error_message = "Clinical retrieval build must require one GPU rather than silently falling back to CPU."
  }

  assert {
    condition = jsondecode(one([
      for environment in google_cloud_run_v2_job.clinical_retrieval_builder[0].template[0].template[0].containers[0].env : environment.value
      if environment.name == "CODA_CLINICAL_RETRIEVAL_BUILD_CONFIG"
    ])).max_passages == 15000000
    error_message = "Clinical retrieval build must carry the explicit pre-inference passage ceiling."
  }

  assert {
    condition     = google_storage_bucket_iam_member.clinical_retrieval_builder_create[0].role == "roles/storage.objectCreator"
    error_message = "The builder may create immutable build objects but may not overwrite or delete them."
  }
}

run "keep_recipe_replacement_out_of_the_recurring_job" {
  command = plan

  variables {
    clinical_retrieval_build_enabled = true
    clinical_retrieval_build_image   = "us-east1-docker.pkg.dev/coda-health-prod/coda-clinical-evidence/runtime-gpu@sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    clinical_document_model_archive = {
      object         = "builds/sha256/cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc/MedCPT-Article-Encoder.tar.gz"
      sha256         = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
      weights_sha256 = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
    }
  }

  assert {
    condition = !contains(keys(jsondecode(one([
      for environment in google_cloud_run_v2_job.clinical_retrieval_builder[0].template[0].template[0].containers[0].env : environment.value
      if environment.name == "CODA_CLINICAL_RETRIEVAL_BUILD_CONFIG"
    ]))), "replace_build_id")
    error_message = "Recipe replacement belongs only to the named workflow execution."
  }
}

run "accept_bounded_daily_source_acquisition" {
  command = plan

  variables {
    clinical_source_acquisition_enabled = true
    source_maintenance_image            = "us-east1-docker.pkg.dev/coda-health-prod/coda-clinical-evidence/maintainer@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    clinical_source_terms_retrieved_at  = "2026-09-02T00:00:00Z"
    clinical_source_acquisition_bootstrap = {
      dailymed_admitted_through = "2026-08-28"
      pmc_receipt_object        = "operations/source-acquisition/objects/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/data"
      mmwr_inventory_object     = "operations/source-acquisition/bootstrap/mmwr/inventory.json"
    }
  }

  assert {
    condition     = local.clinical_source_acquisition_batch.taskGroups[0].taskCount == "1"
    error_message = "Clinical source acquisition must remain single-task."
  }

  assert {
    condition = (
      local.clinical_source_acquisition_batch.taskGroups[0].taskSpec.maxRetryCount == 2 &&
      jsonencode(local.clinical_source_acquisition_batch.taskGroups[0].taskSpec.lifecyclePolicies) ==
      jsonencode([{ action = "RETRY_TASK", actionCondition = { exitCodes = [50003, 50006] } }])
    )
    error_message = "Clinical source acquisition may rerun only twice, and only when Batch lost its VM, never as a paid retry loop."
  }

  assert {
    condition     = local.clinical_source_acquisition_batch.taskGroups[0].taskSpec.computeResource.cpuMilli == "4000"
    error_message = "Clinical source acquisition must stay on bounded CPU-only compute."
  }

  assert {
    condition     = local.clinical_source_acquisition_batch.taskGroups[0].taskSpec.computeResource.memoryMib == "16384"
    error_message = "Clinical source acquisition must fit the measured catch-up snapshot without duplicate staging."
  }

  assert {
    condition     = local.clinical_source_acquisition_batch.taskGroups[0].taskSpec.maxRunDuration == "79200s"
    error_message = "Clinical source acquisition must have one bounded catch-up window."
  }

  assert {
    condition     = google_cloud_run_v2_job.clinical_source_admission[0].template[0].template[0].containers[0].resources[0].limits["memory"] == "8Gi"
    error_message = "Clinical source admission must fit the measured current-scale maintenance envelope."
  }

  assert {
    condition     = google_cloud_run_v2_job.clinical_source_admission[0].template[0].template[0].timeout == "3600s"
    error_message = "Clinical source admission must have one bounded current-scale maintenance window."
  }

  assert {
    condition     = google_cloud_run_v2_job.clinical_payload_reconciliation[0].template[0].task_count == 1
    error_message = "Clinical payload reconciliation must remain one bounded scale-to-zero task."
  }

  assert {
    condition     = google_cloud_run_v2_job.clinical_payload_reconciliation[0].template[0].template[0].max_retries == 0
    error_message = "Clinical payload reconciliation must fail closed instead of retrying mutations automatically."
  }

  assert {
    condition     = google_cloud_run_v2_job.clinical_payload_reconciliation[0].template[0].template[0].containers[0].resources[0].limits["cpu"] == "1"
    error_message = "Clinical payload reconciliation must stay on bounded CPU-only compute."
  }

  assert {
    condition     = google_cloud_run_v2_job.clinical_payload_reconciliation[0].template[0].template[0].containers[0].resources[0].limits["memory"] == "1Gi"
    error_message = "Clinical payload reconciliation must stay within its bounded memory envelope."
  }

  assert {
    condition     = length(google_cloud_scheduler_job.clinical_source_acquisition) == 0
    error_message = "Acquisition alone must not provision an incomplete daily refresh scheduler."
  }

  assert {
    condition     = length(google_monitoring_alert_policy.clinical_source_acquisition_stale) == 0
    error_message = "An unprovisioned refresh scheduler must not emit a false stale-acquisition alert."
  }

  assert {
    condition     = local.clinical_source_backfill_batch == null
    error_message = "Backfill needs an explicit raw PMC archive."
  }

  assert {
    condition = (
      !contains(keys(jsondecode(local.clinical_source_acquisition_batch.taskGroups[0].taskSpec.runnables[0].environment.variables.CODA_CLINICAL_SOURCE_ACQUISITION_CONFIG)), "uspstf") &&
      !contains(keys(local.clinical_source_acquisition_batch.taskGroups[0].taskSpec.runnables[0].environment), "secretVariables")
    )
    error_message = "USPSTF must stay out of acquisition until it is explicitly enabled."
  }

  assert {
    condition = (
      google_secret_manager_secret.clinical_uspstf_api_key[0].secret_id == "coda-clinical-uspstf-api-key" &&
      google_secret_manager_secret.clinical_uspstf_api_key[0].deletion_protection &&
      google_secret_manager_secret.clinical_uspstf_api_key[0].replication[0].user_managed[0].replicas[0].location == var.region &&
      google_secret_manager_secret_iam_member.clinical_uspstf_api_key[0].role == "roles/secretmanager.secretAccessor"
    )
    error_message = "The USPSTF key needs its owned secret before enablement, readable only by acquisition."
  }
}

run "accept_uspstf_with_its_secret" {
  command = plan

  variables {
    clinical_source_acquisition_enabled = true
    source_maintenance_image            = "us-east1-docker.pkg.dev/coda-health-prod/coda-clinical-evidence/maintainer@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    clinical_source_terms_retrieved_at  = "2026-09-02T00:00:00Z"
    clinical_source_acquisition_bootstrap = {
      dailymed_admitted_through = "2026-08-28"
      pmc_receipt_object        = "operations/source-acquisition/objects/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/data"
      mmwr_inventory_object     = "operations/source-acquisition/bootstrap/mmwr/inventory.json"
    }
    clinical_uspstf_enabled = true
  }

  assert {
    condition     = jsondecode(local.clinical_source_acquisition_batch.taskGroups[0].taskSpec.runnables[0].environment.variables.CODA_CLINICAL_SOURCE_ACQUISITION_CONFIG).uspstf.timeout_seconds == 60
    error_message = "Enabled USPSTF acquisition needs its bounded request timeout."
  }

  assert {
    condition = (
      keys(local.clinical_source_acquisition_batch.taskGroups[0].taskSpec.runnables[0].environment.secretVariables) == ["CODA_USPSTF_API_KEY"] &&
      !contains(keys(local.clinical_source_acquisition_batch.taskGroups[0].taskSpec.runnables[0].environment.variables), "CODA_USPSTF_API_KEY")
    )
    error_message = "The USPSTF key must reach acquisition only as a Batch secret variable."
  }
}

run "reject_uspstf_without_acquisition" {
  command = plan

  variables {
    clinical_uspstf_enabled = true
  }

  expect_failures = [var.clinical_uspstf_enabled]
}

run "accept_pinned_backfill_acquisition" {
  command = plan

  variables {
    clinical_source_acquisition_enabled = true
    source_maintenance_image            = "us-east1-docker.pkg.dev/coda-health-prod/coda-clinical-evidence/maintainer@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    clinical_source_terms_retrieved_at  = "2026-09-02T00:00:00Z"
    clinical_source_acquisition_bootstrap = {
      dailymed_admitted_through = "2026-08-28"
      pmc_receipt_object        = "operations/source-acquisition/objects/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/data"
      mmwr_inventory_object     = "operations/source-acquisition/bootstrap/mmwr/inventory.json"
    }
    clinical_pmc_raw_archive_receipt = {
      bucket     = "coda-health-prod-clinical-corpus-us-east1"
      name       = "operations/source-acquisition/baselines/pmc/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb/completion-receipts/cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc.json"
      generation = "1"
      byte_count = 34133
      sha256     = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
    }
  }

  assert {
    condition = (
      local.clinical_source_backfill_batch.taskGroups[0].taskSpec.maxRunDuration == "43200s" &&
      jsondecode(local.clinical_source_backfill_batch.taskGroups[0].taskSpec.runnables[0].environment.variables.CODA_CLINICAL_SOURCE_ACQUISITION_CONFIG).pmc.pin_historical_articles == 100000 &&
      jsondecode(local.clinical_source_backfill_batch.taskGroups[0].taskSpec.runnables[0].environment.variables.CODA_CLINICAL_SOURCE_ACQUISITION_CONFIG).pmc.time_budget_seconds == 28800 &&
      jsondecode(local.clinical_source_backfill_batch.taskGroups[0].taskSpec.runnables[0].environment.variables.CODA_CLINICAL_SOURCE_ACQUISITION_CONFIG).pmc.include_backfill &&
      jsondecode(local.clinical_source_backfill_batch.taskGroups[0].taskSpec.runnables[0].environment.variables.CODA_CLINICAL_SOURCE_ACQUISITION_CONFIG).pmc.raw_archive_receipt.sha256 == var.clinical_pmc_raw_archive_receipt.sha256
    )
    error_message = "Backfill must pin a bounded cohort from the configured archive within one bounded task."
  }

  assert {
    condition = (
      local.clinical_source_acquisition_batch.taskGroups[0].taskSpec.maxRunDuration == "79200s" &&
      jsondecode(local.clinical_source_acquisition_batch.taskGroups[0].taskSpec.runnables[0].environment.variables.CODA_CLINICAL_SOURCE_ACQUISITION_CONFIG).pmc.time_budget_seconds == 7200 &&
      !can(jsondecode(local.clinical_source_acquisition_batch.taskGroups[0].taskSpec.runnables[0].environment.variables.CODA_CLINICAL_SOURCE_ACQUISITION_CONFIG).pmc.pin_historical_articles)
    )
    error_message = "Configuring backfill must not change daily acquisition."
  }
}

run "enable_scheduler_only_after_explicit_gate" {
  command = plan

  variables {
    clinical_source_acquisition_enabled     = true
    clinical_source_scheduler_enabled       = true
    clinical_bulk_admission_timeout_seconds = 43200
    clinical_bulk_build_timeout_seconds     = 64800
    clinical_retrieval_build_enabled        = true
    clinical_retrieval_build_image          = "us-east1-docker.pkg.dev/coda-health-prod/coda-clinical-evidence/runtime-gpu@sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    clinical_document_model_archive = {
      object         = "builds/sha256/cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc/MedCPT-Article-Encoder.tar.gz"
      sha256         = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
      weights_sha256 = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
    }
    source_maintenance_image           = "us-east1-docker.pkg.dev/coda-health-prod/coda-clinical-evidence/maintainer@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    clinical_source_terms_retrieved_at = "2026-09-02T00:00:00Z"
    clinical_source_acquisition_bootstrap = {
      dailymed_admitted_through = "2026-08-28"
      pmc_receipt_object        = "operations/source-acquisition/objects/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/data"
      mmwr_inventory_object     = "operations/source-acquisition/snapshots/bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb/mmwr/inventory.json"
    }
  }

  assert {
    condition     = !google_cloud_scheduler_job.clinical_source_acquisition[0].paused
    error_message = "The daily scheduler should run only after the explicit gate is enabled."
  }

  assert {
    condition     = !local.clinical_refresh_template_values.uspstf_enabled
    error_message = "A disabled USPSTF must not be required for a complete refresh."
  }

  assert {
    condition = (
      strcontains(google_monitoring_alert_policy.clinical_source_incomplete[0].conditions[0].condition_matched_log[0].filter, "jsonPayload.owner=\"clinical-source-night\"") &&
      strcontains(google_monitoring_alert_policy.clinical_source_incomplete[0].conditions[0].condition_matched_log[0].filter, "jsonPayload.needs_person>0") &&
      !strcontains(google_monitoring_alert_policy.clinical_source_incomplete[0].conditions[0].condition_matched_log[0].filter, "resource.type") &&
      !strcontains(google_monitoring_alert_policy.clinical_source_incomplete[0].conditions[0].condition_matched_log[0].filter, "source_observations")
    )
    error_message = "The alert must read the line the job that saved a source's night logs, on every route, so a backfill's night emails too and a disabled USPSTF's retained failure never counts (the line is exercised in test/clinicalSourceFailureBoundary.test.ts)."
  }

  assert {
    condition = (
      google_storage_bucket_iam_member.clinical_refresh_build_receipt_read[0].role == "roles/storage.objectViewer" &&
      google_storage_bucket_iam_member.clinical_refresh_build_receipt_read[0].bucket == var.clinical_corpus_bucket_name &&
      google_storage_bucket_iam_member.clinical_refresh_build_receipt_read[0].condition[0].expression == "resource.name.startsWith('projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/operations/retrieval-refresh/') && resource.name.endsWith('/build.json')" &&
      google_storage_bucket_iam_member.clinical_refresh_state_read[0].condition[0].expression == "resource.name == 'projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/operations/source-acquisition/state-v1.json'"
    )
    error_message = "Saved-build recovery must read execution receipts without write access or broadening acquisition-state permissions."
  }

  assert {
    condition = (
      local.clinical_admission_batch.taskGroups[0].taskSpec.maxRunDuration == "43200s" &&
      local.clinical_admission_batch.taskGroups[0].taskSpec.maxRetryCount == 2 &&
      jsonencode(local.clinical_admission_batch.taskGroups[0].taskSpec.lifecyclePolicies) == jsonencode([{ action = "RETRY_TASK", actionCondition = { exitCodes = [50003, 50006] } }]) &&
      local.clinical_admission_batch.allocationPolicy.instances[0].policy.disks[0].newDisk.sizeGb == "100" &&
      tonumber(local.clinical_refresh_template_values.bulk_admission_wait_seconds) == 44100 &&
      local.clinical_retrieval_batch.taskGroups[0].taskSpec.maxRunDuration == "64800s" &&
      local.clinical_retrieval_batch.taskGroups[0].taskSpec.maxRetryCount == 2 &&
      jsonencode(local.clinical_retrieval_batch.taskGroups[0].taskSpec.lifecyclePolicies) == jsonencode([{ action = "RETRY_TASK", actionCondition = { exitCodes = [50003, 50006] } }]) &&
      local.clinical_retrieval_batch.taskGroups[0].taskSpec.computeResource.memoryMib == "49152" &&
      local.clinical_retrieval_batch.allocationPolicy.instances[0].policy.machineType == "g2-standard-12" &&
      tonumber(local.clinical_refresh_template_values.bulk_build_wait_seconds) == 65700 &&
      tonumber(local.clinical_refresh_template_values.bulk_admission_timeout_seconds) == 43200 &&
      google_cloud_run_v2_job.clinical_source_admission[0].template[0].template[0].timeout == "3600s" &&
      google_cloud_run_v2_job.clinical_retrieval_builder[0].template[0].template[0].timeout == "1800s"
    )
    error_message = "Bulk publication limits must reach native jobs and workflow waits without increasing daily deadlines or retrying beyond a lost VM."
  }

  assert {
    condition = (
      join(",", [for attempt in local.clinical_retrieval_attempts : attempt.job_prefix]) == join(",", [
        "coda-retrieval-l4-1-", "coda-retrieval-t4-1-", "coda-retrieval-l4-2-", "coda-retrieval-t4-2-",
        "coda-retrieval-l4-3-", "coda-retrieval-t4-3-", "coda-retrieval-l4-4-", "coda-retrieval-t4-4-",
        "coda-retrieval-l4-5-", "coda-retrieval-t4-5-", "coda-retrieval-l4-6-", "coda-retrieval-t4-6-",
      ]) &&
      tonumber(local.clinical_refresh_template_values.publication_wait_seconds) == 207600
    )
    error_message = "A publication must name L4 then T4 attempts of two hours each, six rounds in its 24-hour capacity wait, and wait for that one wait, not per attempt."
  }

  assert {
    condition = (
      jsondecode(local.clinical_source_acquisition_batch.taskGroups[0].taskSpec.runnables[0].environment.variables.CODA_CLINICAL_SOURCE_ACQUISITION_CONFIG).pmc.time_budget_seconds == 7200 &&
      !try(jsondecode(local.clinical_source_acquisition_batch.taskGroups[0].taskSpec.runnables[0].environment.variables.CODA_CLINICAL_SOURCE_ACQUISITION_CONFIG).pmc.include_backfill, false)
    )
    error_message = "Larger historical publications must not turn daily acquisition into backfill."
  }

  assert {
    condition = (
      contains(keys(google_cloud_run_v2_job_iam_member.clinical_refresh_job_read), "reconciliation") &&
      !contains(keys(google_cloud_run_v2_job_iam_member.clinical_refresh_jobs), "reconciliation")
    )
    error_message = "Cleanup must observe the SQL reconciliation job without gaining permission to dispatch it."
  }

  assert {
    condition     = google_monitoring_alert_policy.clinical_source_acquisition_stale[0].enabled
    error_message = "An enabled scheduler must retain its missing-success alert."
  }

  assert {
    condition = (
      strcontains(google_logging_metric.clinical_source_acquisition_success[0].filter, "jsonPayload.execution_succeeded=true") &&
      !strcontains(google_logging_metric.clinical_source_acquisition_success[0].filter, "source_checks_complete") &&
      strcontains(google_logging_metric.clinical_source_acquisition_failure[0].filter, "jsonPayload.execution_succeeded=false") &&
      !strcontains(google_logging_metric.clinical_source_acquisition_failure[0].filter, "source_checks_complete") &&
      strcontains(google_logging_metric.clinical_source_incomplete[0].filter, "jsonPayload.execution_succeeded=true") &&
      strcontains(google_logging_metric.clinical_source_incomplete[0].filter, "jsonPayload.source_checks_complete=false") &&
      strcontains(google_logging_metric.clinical_optional_visual_unavailable[0].filter, "changed_article_media.unavailable_static_images>0")
    )
    error_message = "Execution, source completeness, and optional visual availability must remain separate health projections."
  }

  assert {
    condition = (
      google_monitoring_alert_policy.clinical_source_acquisition_failure[0].conditions[0].condition_matched_log[0].filter == google_logging_metric.clinical_source_acquisition_failure[0].filter &&
      google_monitoring_alert_policy.clinical_source_acquisition_failure[0].alert_strategy[0].notification_rate_limit[0].period == "3600s" &&
      google_monitoring_alert_policy.clinical_source_incomplete[0].alert_strategy[0].notification_rate_limit[0].period == "86400s" &&
      !strcontains(google_monitoring_alert_policy.clinical_source_incomplete[0].conditions[0].condition_matched_log[0].filter, "source_checks_complete") &&
      !strcontains(google_monitoring_alert_policy.clinical_source_incomplete[0].conditions[0].condition_matched_log[0].filter, "backfill") &&
      !strcontains(google_monitoring_alert_policy.clinical_source_incomplete[0].conditions[0].condition_matched_log[0].filter, "unresolved") &&
      !strcontains(google_monitoring_alert_policy.clinical_source_incomplete[0].conditions[0].condition_matched_log[0].filter, "refresh>0") &&
      !strcontains(google_monitoring_alert_policy.clinical_source_incomplete[0].conditions[0].condition_matched_log[0].filter, "discovery>0") &&
      strcontains(google_monitoring_alert_policy.clinical_source_incomplete[0].conditions[0].condition_matched_log[0].filter, "jsonPayload.needs_person>0")
    )
    error_message = "Backlog and records waiting on their own retry must not email; whatever a source reported to a person tonight must, with bounded repeat notifications."
  }

  assert {
    condition = alltrue([for policy in [
      google_monitoring_alert_policy.clinical_source_acquisition_failure[0],
      google_monitoring_alert_policy.clinical_source_incomplete[0],
      google_monitoring_alert_policy.clinical_source_acquisition_stale[0]
    ] : toset(policy.alert_strategy[0].notification_prompts) == toset(["OPENED"])])
    error_message = "An alert window clearing must not email an unsupported source-recovery claim."
  }
}

run "reject_unbounded_bulk_admission" {
  command = plan
  variables { clinical_bulk_admission_timeout_seconds = 43201 }
  expect_failures = [var.clinical_bulk_admission_timeout_seconds]
}

run "reject_a100_build_machines" {
  command = plan
  variables {
    clinical_retrieval_build_machines = [{ name = "a100", machine_type = "a2-highgpu-1g", accelerator = "nvidia-tesla-a100" }]
  }
  expect_failures = [var.clinical_retrieval_build_machines]
}

run "reject_short_batch_scheduling_wait" {
  command = plan
  variables { clinical_batch_scheduling_wait_seconds = 600 }
  expect_failures = [var.clinical_batch_scheduling_wait_seconds]
}

run "bound_gpu_capacity_wait_by_elapsed_time" {
  command = plan
  variables {
    clinical_gpu_capacity_wait_seconds     = 14400
    clinical_batch_scheduling_wait_seconds = 21600
  }

  assert {
    condition     = join(",", [for attempt in local.clinical_retrieval_attempts : attempt.job_prefix]) == "coda-retrieval-l4-1-"
    error_message = "A scheduling wait longer than the capacity wait must leave one attempt, which the workflow cuts to the capacity wait."
  }
}

run "reject_unbounded_gpu_capacity_wait" {
  command = plan
  variables { clinical_gpu_capacity_wait_seconds = 259200 }
  expect_failures = [var.clinical_gpu_capacity_wait_seconds]
}

run "reject_fractional_bulk_build" {
  command = plan
  variables { clinical_bulk_build_timeout_seconds = 43200.5 }
  expect_failures = [var.clinical_bulk_build_timeout_seconds]
}

run "reject_unbounded_sql_disk" {
  command = plan

  variables {
    sql_disk_autoresize_limit_gb = 501
  }

  expect_failures = [var.sql_disk_autoresize_limit_gb]
}

run "reject_arbitrary_mmwr_surface" {
  command = plan

  variables {
    clinical_mmwr_index_urls = ["https://example.test/releases"]
  }

  expect_failures = [var.clinical_mmwr_index_urls]
}

run "reject_bootstrap_outside_owned_operations_prefix" {
  command = plan

  variables {
    clinical_source_acquisition_bootstrap = {
      dailymed_admitted_through = "2026-08-28"
      pmc_receipt_object        = "sha256/aa/source.json"
      mmwr_inventory_object     = "operations/source-acquisition/bootstrap/mmwr/inventory.json"
    }
  }

  expect_failures = [var.clinical_source_acquisition_bootstrap]
}

run "reject_unqualified_passage_capacity" {
  command = plan
  variables { clinical_retrieval_max_passages = 46000001 }
  expect_failures = [var.clinical_retrieval_max_passages]
}

run "reject_unbounded_bulk_build" {
  command = plan
  variables { clinical_bulk_build_timeout_seconds = 64801 }
  expect_failures = [var.clinical_bulk_build_timeout_seconds]
}

run "require_enabled_uspstf_in_a_complete_refresh" {
  command = plan

  variables {
    clinical_source_acquisition_enabled     = true
    clinical_source_scheduler_enabled       = true
    clinical_bulk_admission_timeout_seconds = 43200
    clinical_bulk_build_timeout_seconds     = 64800
    clinical_retrieval_build_enabled        = true
    clinical_retrieval_build_image          = "us-east1-docker.pkg.dev/coda-health-prod/coda-clinical-evidence/runtime-gpu@sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    clinical_document_model_archive = {
      object         = "builds/sha256/cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc/MedCPT-Article-Encoder.tar.gz"
      sha256         = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
      weights_sha256 = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
    }
    source_maintenance_image           = "us-east1-docker.pkg.dev/coda-health-prod/coda-clinical-evidence/maintainer@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    clinical_source_terms_retrieved_at = "2026-09-02T00:00:00Z"
    clinical_source_acquisition_bootstrap = {
      dailymed_admitted_through = "2026-08-28"
      pmc_receipt_object        = "operations/source-acquisition/objects/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/data"
      mmwr_inventory_object     = "operations/source-acquisition/snapshots/bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb/mmwr/inventory.json"
    }
    clinical_uspstf_enabled = true
  }

  assert {
    condition     = local.clinical_refresh_template_values.uspstf_enabled
    error_message = "An enabled USPSTF must be healthy for a refresh to count as complete (the rendered check is exercised in test/clinicalRefreshWorkflowBoundary.test.ts)."
  }
}

run "serving_close_drains_exactly_the_replica_group" {
  command = plan

  variables {
    clinical_source_acquisition_enabled = true
    clinical_retrieval_build_enabled    = true
    clinical_retrieval_build_image      = "us-east1-docker.pkg.dev/coda-health-prod/coda-clinical-evidence/runtime-gpu@sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    clinical_document_model_archive = {
      object         = "builds/sha256/cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc/MedCPT-Article-Encoder.tar.gz"
      sha256         = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
      weights_sha256 = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
    }
    source_maintenance_image           = "us-east1-docker.pkg.dev/coda-health-prod/coda-clinical-evidence/maintainer@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    clinical_source_terms_retrieved_at = "2026-09-02T00:00:00Z"
    clinical_source_acquisition_bootstrap = {
      dailymed_admitted_through = "2026-08-28"
      pmc_receipt_object        = "operations/source-acquisition/objects/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/data"
      mmwr_inventory_object     = "operations/source-acquisition/snapshots/bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb/mmwr/inventory.json"
    }
    clinical_evidence_service_enabled = true
    clinical_evidence_invokers        = ["user:dev@codahealth.ai"]
    gpu_clinical_retrieval_image      = "us-east1-docker.pkg.dev/coda-health-prod/coda-clinical-evidence/runtime-gpu@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    clinical_serving_replica_count    = 2
  }

  assert {
    condition = (
      one([for variable in google_cloud_run_v2_job.clinical_serving_close[0].template[0].template[0].containers[0].env : variable.value if variable.name == "CODA_CLINICAL_SERVING_REPLICAS"]) ==
      "projects/${var.project_id}/regions/${var.region}/instanceGroupManagers/${google_compute_region_instance_group_manager.clinical_serving[0].name}"
    )
    error_message = "Closing serving must drain the replica group that serves, by its exact name."
  }

  assert {
    condition = (
      length(google_project_iam_member.clinical_serving_closer_replicas[0].condition) == 0 &&
      toset(google_project_iam_custom_role.clinical_serving_replica_close[0].permissions) == toset(["compute.instanceGroupManagers.get", "compute.instanceGroupManagers.update"])
    )
    error_message = "The closer may inspect and resize instance groups and nothing more, with no condition IAM can't evaluate for a group."
  }
}
