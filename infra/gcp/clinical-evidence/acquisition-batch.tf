# Workflows creates bounded native Batch jobs. The existing source-acquisition
# account and content-addressed GCS graph own source progress across task loss.
locals {
  # Every clinical job reruns only when Batch itself lost the VM: 50003 rebooted,
  # 50006 recreated. Each job resumes from its saved progress and writes once.
  # Lost contact (50002) or an uncancellable task (50004) may leave the old VM
  # writing, so those fail, as does every other exit.
  clinical_batch_task_retry = {
    maxRetryCount     = 2
    lifecyclePolicies = [{ action = "RETRY_TASK", actionCondition = { exitCodes = [50003, 50006] } }]
  }
  clinical_source_acquisition_pmc = {
    base_url            = "https://pmc-oa-opendata.s3.amazonaws.com/"
    workers             = 8
    batch_size          = 32
    time_budget_seconds = var.clinical_source_acquisition_time_budget_seconds
    collection_policy   = jsondecode(file("${path.module}/pmc-collection-policy.json"))
  }
  # A backfill run pins the next fixed cohort from the raw archive (or resumes
  # the saved one) and stops after acquisition; publication stays explicit.
  clinical_source_acquisition_modes = merge({
    acquisition = { pmc = local.clinical_source_acquisition_pmc, max_run_seconds = 79200 }
    }, var.clinical_pmc_raw_archive_receipt == null ? {} : {
    backfill = { max_run_seconds = 43200, pmc = merge(local.clinical_source_acquisition_pmc, {
      batch_size              = 128
      time_budget_seconds     = var.clinical_backfill_time_budget_seconds
      include_backfill        = true
      raw_archive_receipt     = var.clinical_pmc_raw_archive_receipt
      pin_historical_articles = var.clinical_backfill_articles
    }) }
  })
  clinical_source_acquisition_batches = { for mode, settings in local.clinical_source_acquisition_modes : mode => {
    taskGroups = [{
      taskCount   = "1"
      parallelism = "1"
      taskSpec = {
        maxRetryCount     = local.clinical_batch_task_retry.maxRetryCount
        lifecyclePolicies = local.clinical_batch_task_retry.lifecyclePolicies
        maxRunDuration    = "${settings.max_run_seconds}s"
        computeResource   = { cpuMilli = "4000", memoryMib = "16384" }
        runnables = [{
          container = {
            imageUri   = var.source_maintenance_image
            entrypoint = "node"
            commands   = ["dist/src/server/clinicalCorpus/acquire/hostedClinicalSourceAcquisition.js"]
            volumes    = ["/mnt/disks/acquisition:/work"]
          }
          # With USPSTF enabled, Batch resolves its key from Secret Manager when the job
          # starts, so the key never appears in the job spec, workflow or logs.
          environment = merge({
            variables = {
              TMPDIR                      = "/work"
              CODA_CLINICAL_CORPUS_BUCKET = var.clinical_corpus_bucket_name
              CODA_CLINICAL_SOURCE_ACQUISITION_CONFIG = jsonencode(merge({
                schema_version         = 1
                state_object           = "operations/source-acquisition/state-v1.json"
                snapshot_prefix        = "operations/source-acquisition/snapshots"
                python_executable      = "python3"
                source_timeout_seconds = 900
                dailymed               = { catalog_url = "https://dailymed.nlm.nih.gov/dailymed/spl-resources-all-drug-labels.cfm" }
                pmc                    = settings.pmc
                mmwr                   = { index_urls = var.clinical_mmwr_index_urls, workers = 4, timeout_seconds = 30 }
              }, var.clinical_uspstf_enabled ? { uspstf = { timeout_seconds = 60 } } : {}))
              CODA_CLINICAL_SOURCE_ACQUISITION_BOOTSTRAP = jsonencode({
                schema_version = 1
                baseline = {
                  dailymed = { admitted_through = try(var.clinical_source_acquisition_bootstrap.dailymed_admitted_through, "") }
                  pmc      = { receipt_object = try(var.clinical_source_acquisition_bootstrap.pmc_receipt_object, "") }
                  mmwr     = { inventory_object = try(var.clinical_source_acquisition_bootstrap.mmwr_inventory_object, "") }
                }
                pending = null
              })
            }
            }, var.clinical_uspstf_enabled ? { secretVariables = {
              CODA_USPSTF_API_KEY = "${google_secret_manager_secret.clinical_uspstf_api_key[0].id}/versions/latest"
          } } : {})
        }]
        volumes = [{ deviceName = "acquisition", mountPath = "/mnt/disks/acquisition" }]
      }
    }]
    allocationPolicy = {
      serviceAccount = { email = google_service_account.clinical_source_acquisition[0].email }
      instances = [{ policy = {
        machineType       = "e2-standard-4"
        provisioningModel = "STANDARD"
        bootDisk          = { type = "pd-balanced", sizeGb = "30" }
        disks             = [{ deviceName = "acquisition", newDisk = { type = "pd-balanced", sizeGb = "100" } }]
      } }]
      network = { networkInterfaces = [{
        network    = google_compute_network.clinical_evidence.id
        subnetwork = google_compute_subnetwork.operations.id
      }] }
    }
    logsPolicy = { destination = "CLOUD_LOGGING" }
    labels     = { owner = "clinical-source-acquisition" }
  } if var.clinical_source_acquisition_enabled }
  clinical_source_acquisition_batch = lookup(local.clinical_source_acquisition_batches, "acquisition", null)
  clinical_source_backfill_batch    = lookup(local.clinical_source_acquisition_batches, "backfill", null)
}

# The AHRQ organization key for the Prevention TaskForce API. Terraform owns the secret and
# who may read it; its one value is added by hand and never enters tfvars or state.
resource "google_secret_manager_secret" "clinical_uspstf_api_key" {
  count               = var.clinical_source_acquisition_enabled ? 1 : 0
  project             = var.project_id
  secret_id           = "coda-clinical-uspstf-api-key"
  deletion_protection = true

  replication {
    user_managed {
      replicas {
        location = var.region
      }
    }
  }

  depends_on = [google_project_service.workload]
}

resource "google_secret_manager_secret_iam_member" "clinical_uspstf_api_key" {
  count     = var.clinical_source_acquisition_enabled ? 1 : 0
  project   = var.project_id
  secret_id = google_secret_manager_secret.clinical_uspstf_api_key[0].secret_id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${google_service_account.clinical_source_acquisition[0].email}"
}

resource "google_project_iam_member" "clinical_source_batch_agent" {
  count   = var.clinical_source_acquisition_enabled ? 1 : 0
  project = var.project_id
  role    = "roles/batch.agentReporter"
  member  = "serviceAccount:${google_service_account.clinical_source_acquisition[0].email}"
}

resource "google_project_iam_member" "clinical_source_batch_logging" {
  count   = var.clinical_source_acquisition_enabled ? 1 : 0
  project = var.project_id
  role    = "roles/logging.logWriter"
  member  = "serviceAccount:${google_service_account.clinical_source_acquisition[0].email}"
}

resource "google_artifact_registry_repository_iam_member" "clinical_source_batch_image" {
  count      = var.clinical_source_acquisition_enabled ? 1 : 0
  project    = var.project_id
  location   = google_artifact_registry_repository.clinical_evidence.location
  repository = google_artifact_registry_repository.clinical_evidence.name
  role       = "roles/artifactregistry.reader"
  member     = "serviceAccount:${google_service_account.clinical_source_acquisition[0].email}"
}

resource "google_project_iam_custom_role" "clinical_refresh_batch" {
  count       = local.clinical_refresh_enabled ? 1 : 0
  project     = var.project_id
  role_id     = "CodaClinicalAcquisitionBatch"
  title       = "Coda Clinical Acquisition Batch"
  description = "Submit, inspect and delete acquisition jobs by exact workflow-owned identity."
  permissions = ["batch.jobs.create", "batch.jobs.get", "batch.jobs.delete", "batch.operations.get"]
}

resource "google_project_iam_member" "clinical_refresh_batch" {
  count   = local.clinical_refresh_enabled ? 1 : 0
  project = var.project_id
  role    = google_project_iam_custom_role.clinical_refresh_batch[0].id
  member  = "serviceAccount:${google_service_account.clinical_refresh_workflow[0].email}"
}

resource "google_service_account_iam_member" "clinical_refresh_acquisition_identity" {
  count              = local.clinical_refresh_enabled ? 1 : 0
  service_account_id = google_service_account.clinical_source_acquisition[0].name
  role               = "roles/iam.serviceAccountUser"
  member             = "serviceAccount:${google_service_account.clinical_refresh_workflow[0].email}"
}
