# Historical admission executes the same canonical maintainer on finite disk.
# Source identities, SQL transactions and publication lifecycle remain unchanged.
locals {
  clinical_admission_batch = local.clinical_refresh_enabled ? {
    taskGroups = [{
      taskCount   = "1"
      parallelism = "1"
      taskSpec = {
        maxRetryCount     = local.clinical_batch_task_retry.maxRetryCount
        lifecyclePolicies = local.clinical_batch_task_retry.lifecyclePolicies
        maxRunDuration    = "${var.clinical_bulk_admission_timeout_seconds}s"
        computeResource   = { cpuMilli = "4000", memoryMib = "16384" }
        volumes           = [{ deviceName = "admission", mountPath = "/mnt/disks/admission" }]
      }
    }]
    allocationPolicy = {
      serviceAccount = { email = google_service_account.clinical_corpus_maintainer[0].email }
      instances = [{ policy = {
        machineType       = "e2-standard-4"
        provisioningModel = "STANDARD"
        bootDisk          = { type = "pd-balanced", sizeGb = "30" }
        disks             = [{ deviceName = "admission", newDisk = { type = "pd-balanced", sizeGb = "100" } }]
      } }]
      network = { networkInterfaces = [{
        network    = google_compute_network.clinical_evidence.id
        subnetwork = google_compute_subnetwork.operations.id
      }] }
    }
    logsPolicy = { destination = "CLOUD_LOGGING" }
    labels     = { owner = "clinical-source-admission" }
  } : null
}

resource "google_service_account_iam_member" "clinical_refresh_admission_identity" {
  count              = local.clinical_refresh_enabled ? 1 : 0
  service_account_id = google_service_account.clinical_corpus_maintainer[0].name
  role               = "roles/iam.serviceAccountUser"
  member             = "serviceAccount:${google_service_account.clinical_refresh_workflow[0].email}"
}

resource "google_project_iam_member" "clinical_admission_batch_runtime" {
  for_each = local.clinical_refresh_enabled ? toset(["roles/batch.agentReporter", "roles/logging.logWriter"]) : toset([])
  project  = var.project_id
  role     = each.value
  member   = "serviceAccount:${google_service_account.clinical_corpus_maintainer[0].email}"
}

resource "google_artifact_registry_repository_iam_member" "clinical_admission_batch_image" {
  count      = local.clinical_refresh_enabled ? 1 : 0
  project    = var.project_id
  location   = google_artifact_registry_repository.clinical_evidence.location
  repository = google_artifact_registry_repository.clinical_evidence.name
  role       = "roles/artifactregistry.reader"
  member     = "serviceAccount:${google_service_account.clinical_corpus_maintainer[0].email}"
}
