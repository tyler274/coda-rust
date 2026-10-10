locals {
  # Keep one bounded ceiling for the atomic activation job and both workflow
  # paths, including activation recovery; activation does not rescan the corpus.
  clinical_activation_timeout_seconds = 2700
  clinical_refresh_jobs = local.clinical_refresh_enabled ? merge({
    admission = { location = var.region, name = google_cloud_run_v2_job.clinical_source_admission[0].name }
    builder   = { location = var.gpu_region, name = google_cloud_run_v2_job.clinical_retrieval_builder[0].name }
    activator = { location = var.region, name = google_cloud_run_v2_job.clinical_retrieval_activator[0].name }
    finalizer = { location = var.region, name = google_cloud_run_v2_job.clinical_source_finalization[0].name }
    }, var.migration_image == null ? {} : {
    migration = { location = var.region, name = google_cloud_run_v2_job.database_migrator[0].name }
  }) : {}

  # Payload reconciliation also uses SQL, although this workflow never dispatches
  # it. Include it in cleanup's foreign-owner checks without granting run access.
  clinical_refresh_guard_jobs = merge(local.clinical_refresh_jobs, local.clinical_refresh_enabled ? {
    reconciliation = { location = var.region, name = google_cloud_run_v2_job.clinical_payload_reconciliation[0].name }
  } : {})

  clinical_refresh_template_values = local.clinical_refresh_enabled ? {
    project_id                    = var.project_id
    region                        = var.region
    gpu_region                    = var.gpu_region
    bucket_name                   = var.clinical_corpus_bucket_name
    sql_instance                  = google_sql_database_instance.clinical_evidence.name
    acquisition_batch             = jsonencode(local.clinical_source_acquisition_batch)
    backfill_batch                = jsonencode(local.clinical_source_backfill_batch)
    backfill_enabled              = local.clinical_source_backfill_batch != null
    uspstf_enabled                = var.clinical_uspstf_enabled
    admission_batch               = jsonencode(local.clinical_admission_batch)
    bulk_admission_wait_seconds   = var.clinical_bulk_admission_timeout_seconds + 900
    retrieval_batch               = jsonencode(local.clinical_retrieval_batch)
    retrieval_attempts            = jsonencode(local.clinical_retrieval_attempts)
    batch_scheduling_wait_seconds = var.clinical_batch_scheduling_wait_seconds
    gpu_capacity_wait_seconds     = var.clinical_gpu_capacity_wait_seconds
    # A publication waits for admission, the GPU capacity wait plus its last job's
    # deletion, then one build and activation, each within its own window.
    publication_wait_seconds = (var.clinical_batch_scheduling_wait_seconds + var.clinical_bulk_admission_timeout_seconds + 900 +
      var.clinical_gpu_capacity_wait_seconds + 600 + var.clinical_bulk_build_timeout_seconds + 900 +
    local.clinical_activation_timeout_seconds + 900)
    admission_job                  = google_cloud_run_v2_job.clinical_source_admission[0].name
    builder_job                    = google_cloud_run_v2_job.clinical_retrieval_builder[0].name
    activator_job                  = google_cloud_run_v2_job.clinical_retrieval_activator[0].name
    activation_timeout_seconds     = local.clinical_activation_timeout_seconds
    bulk_admission_timeout_seconds = var.clinical_bulk_admission_timeout_seconds
    bulk_build_wait_seconds        = var.clinical_bulk_build_timeout_seconds + 900
    finalization_job               = google_cloud_run_v2_job.clinical_source_finalization[0].name
    manage_sql_activation          = var.sql_activation_policy == "NEVER"
    bibliographic_status_enabled   = var.clinical_bibliographic_contact_email != null
    migration_job                  = try(google_cloud_run_v2_job.database_migrator[0].name, "")
    maintenance_enabled            = !var.clinical_source_scheduler_enabled && var.sql_activation_policy == "NEVER" && var.migration_image != null && var.clinical_evidence_service_enabled
    serving_close_job              = try("projects/${var.project_id}/locations/${var.region}/jobs/${google_cloud_run_v2_job.clinical_serving_close[0].name}", "")
    job_paths                      = jsonencode([for job in local.clinical_refresh_guard_jobs : "projects/${var.project_id}/locations/${job.location}/jobs/${job.name}"])
  } : {}
}

resource "google_service_account" "clinical_refresh_workflow" {
  count = local.clinical_refresh_enabled ? 1 : 0

  project      = var.project_id
  account_id   = "coda-clinical-refresh"
  display_name = "Coda clinical corpus refresh workflow"

  depends_on = [google_project_service.workload]
}

resource "google_project_iam_custom_role" "clinical_refresh_sql_control" {
  count = local.clinical_refresh_enabled && var.sql_activation_policy == "NEVER" ? 1 : 0

  project     = var.project_id
  role_id     = "CodaClinicalRefreshSqlControl"
  title       = "Coda Clinical Refresh SQL Control"
  description = "Start, inspect, and park only the production clinical SQL instance during refresh."
  permissions = [
    "cloudsql.instances.get",
    "cloudsql.instances.update",
  ]
}

resource "google_project_iam_member" "clinical_refresh_sql_control" {
  count = local.clinical_refresh_enabled && var.sql_activation_policy == "NEVER" ? 1 : 0

  project = var.project_id
  role    = google_project_iam_custom_role.clinical_refresh_sql_control[0].id
  member  = "serviceAccount:${google_service_account.clinical_refresh_workflow[0].email}"

  condition {
    title       = "production-clinical-sql-instance-only"
    description = "Start, inspect, and park only the production clinical SQL instance."
    expression  = "resource.type == 'sqladmin.googleapis.com/Instance' && resource.name == 'projects/${var.project_id}/instances/${google_sql_database_instance.clinical_evidence.name}'"
  }
}

resource "google_project_iam_custom_role" "clinical_refresh_run_operation_read" {
  count = local.clinical_refresh_enabled ? 1 : 0

  project     = var.project_id
  role_id     = "CodaClinicalRefreshRunOperationRead"
  title       = "Coda Clinical Refresh Run Operation Read"
  description = "Read Cloud Run operations and inspect concurrent executions of the clinical workflow."
  permissions = ["run.operations.get", "workflows.executions.list", "workflows.executions.get", "workflows.executions.create"]
}

resource "google_project_iam_member" "clinical_refresh_run_operation_read" {
  count = local.clinical_refresh_enabled ? 1 : 0

  project = var.project_id
  role    = google_project_iam_custom_role.clinical_refresh_run_operation_read[0].id
  member  = "serviceAccount:${google_service_account.clinical_refresh_workflow[0].email}"
}

resource "google_project_iam_member" "clinical_refresh_logging" {
  count = local.clinical_refresh_enabled ? 1 : 0

  project = var.project_id
  role    = "roles/logging.logWriter"
  member  = "serviceAccount:${google_service_account.clinical_refresh_workflow[0].email}"
}

resource "google_storage_bucket_iam_member" "clinical_refresh_state_read" {
  count = local.clinical_refresh_enabled ? 1 : 0

  bucket = var.clinical_corpus_bucket_name
  role   = "roles/storage.objectUser"
  member = "serviceAccount:${google_service_account.clinical_refresh_workflow[0].email}"

  condition {
    title       = "production-clinical-refresh-state-read"
    description = "Read and conditionally update only the existing acquisition state and workflow windows."
    expression  = "resource.name == 'projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/operations/source-acquisition/state-v1.json'"
  }
}

resource "google_storage_bucket_iam_member" "clinical_refresh_build_receipt_read" {
  count = local.clinical_refresh_enabled ? 1 : 0

  bucket = var.clinical_corpus_bucket_name
  role   = "roles/storage.objectViewer"
  member = "serviceAccount:${google_service_account.clinical_refresh_workflow[0].email}"

  condition {
    title       = "production-clinical-refresh-build-receipt-read"
    description = "Read immutable execution build receipts before starting recovery compute."
    expression  = "resource.name.startsWith('projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/operations/retrieval-refresh/') && resource.name.endsWith('/build.json')"
  }
}

resource "google_cloud_run_v2_job_iam_member" "clinical_refresh_jobs" {
  for_each = local.clinical_refresh_jobs

  project  = var.project_id
  location = each.value.location
  name     = each.value.name
  role     = "roles/run.jobsExecutorWithOverrides"
  member   = "serviceAccount:${google_service_account.clinical_refresh_workflow[0].email}"
}

resource "google_workflows_workflow" "clinical_refresh" {
  count = local.clinical_refresh_enabled ? 1 : 0

  project             = var.project_id
  name                = "coda-clinical-corpus-refresh"
  region              = var.region
  description         = "Change-driven clinical corpus acquisition, admission, build, activation, and finalization"
  service_account     = google_service_account.clinical_refresh_workflow[0].id
  deletion_protection = true
  source_contents = join("\n", [
    for template in [
      "clinical-refresh-workflow.yaml.tftpl",
      "clinical-acquisition-workflow.yaml.tftpl",
      "clinical-batch-workflow.yaml.tftpl",
      "clinical-retrieval-batch-workflow.yaml.tftpl",
      "clinical-admission-batch-workflow.yaml.tftpl",
      "clinical-workflow-windows.yaml.tftpl",
      "clinical-maintenance-workflow.yaml.tftpl",
      "clinical-job-operations.yaml.tftpl",
      "clinical-workflow-cleanup.yaml.tftpl",
    ] : templatefile("${path.module}/${template}", local.clinical_refresh_template_values)
  ])

  depends_on = [
    google_project_iam_member.clinical_refresh_sql_control,
    google_project_iam_member.clinical_refresh_batch,
    google_service_account_iam_member.clinical_refresh_acquisition_identity,
    google_service_account_iam_member.clinical_refresh_builder_identity,
    google_service_account_iam_member.clinical_refresh_admission_identity,
    google_project_iam_member.clinical_admission_batch_runtime,
    google_artifact_registry_repository_iam_member.clinical_admission_batch_image,
    google_project_iam_member.clinical_refresh_run_operation_read,
    google_project_iam_member.clinical_refresh_logging,
    google_storage_bucket_iam_member.clinical_refresh_state_read,
    google_storage_bucket_iam_member.clinical_refresh_build_receipt_read,
    google_cloud_run_v2_job_iam_member.clinical_refresh_jobs,
    google_cloud_run_v2_job_iam_member.clinical_refresh_job_read,
    google_cloud_run_v2_job_iam_member.clinical_refresh_serving_close,
  ]
}

resource "google_project_iam_member" "clinical_refresh_scheduler" {
  count = local.clinical_refresh_enabled ? 1 : 0

  project = var.project_id
  role    = "roles/workflows.invoker"
  member  = "serviceAccount:${google_service_account.clinical_source_scheduler[0].email}"

  depends_on = [google_workflows_workflow.clinical_refresh]
}
