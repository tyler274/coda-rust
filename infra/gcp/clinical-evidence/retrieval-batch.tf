# Disk-backed native index execution uses the existing builder identity and
# object grants. These are the standard Batch execution permissions only.
locals {
  # Batch accepts one instance policy per job, so each attempt is one machine with its
  # own job ID: an ID stays taken while Batch deletes its job. The machines alternate,
  # and the workflow's one elapsed deadline ends the list early; these are only the
  # names an attempt may use, at most one per scheduling wait in the capacity wait.
  clinical_retrieval_attempt_slots = [for index in range(ceil(var.clinical_gpu_capacity_wait_seconds / var.clinical_batch_scheduling_wait_seconds)) : {
    machine = var.clinical_retrieval_build_machines[index % length(var.clinical_retrieval_build_machines)]
    round   = floor(index / length(var.clinical_retrieval_build_machines)) + 1
  }]
  clinical_retrieval_attempts = [for slot in local.clinical_retrieval_attempt_slots : {
    name       = slot.machine.name
    round      = slot.round
    job_prefix = "coda-retrieval-${slot.machine.name}-${slot.round}-"
    instance = {
      installGpuDrivers = true
      policy = {
        machineType       = slot.machine.machine_type
        provisioningModel = "STANDARD"
        reservation       = "NO_RESERVATION"
        # The CUDA 13 image requires R580. This exact Batch COS release lists
        # 580.173.02 for L4 and T4; do not inherit its older default driver.
        accelerators = [{ type = slot.machine.accelerator, count = "1", driverVersion = "580.173.02" }]
        bootDisk = {
          image  = "projects/batch-custom-image/global/images/batch-cos-stable-official-20260904-00-p00"
          type   = "pd-balanced"
          sizeGb = "50"
        }
        disks = [{ deviceName = "retrieval", newDisk = { type = "pd-balanced", sizeGb = "200" } }]
      }
    }
  }]

  # Image, command and environment come from the canonical builder definition
  # at dispatch. This spec owns only the finite native execution resources;
  # the workflow sets its instance from each attempt.
  clinical_retrieval_batch = var.clinical_retrieval_build_enabled ? {
    taskGroups = [{
      taskCount   = "1"
      parallelism = "1"
      taskSpec = {
        maxRetryCount     = local.clinical_batch_task_retry.maxRetryCount
        lifecyclePolicies = local.clinical_batch_task_retry.lifecyclePolicies
        maxRunDuration    = "${var.clinical_bulk_build_timeout_seconds}s"
        computeResource   = { cpuMilli = "12000", memoryMib = "49152" }
        volumes           = [{ deviceName = "retrieval", mountPath = "/mnt/disks/retrieval" }]
      }
    }]
    allocationPolicy = {
      serviceAccount = { email = google_service_account.clinical_retrieval_builder[0].email }
      instances      = [local.clinical_retrieval_attempts[0].instance]
      network = { networkInterfaces = [{
        network    = google_compute_network.clinical_evidence.id
        subnetwork = google_compute_subnetwork.operations.id
      }] }
    }
    logsPolicy = { destination = "CLOUD_LOGGING" }
    labels     = { owner = "clinical-retrieval-build" }
  } : null
}

resource "google_service_account_iam_member" "clinical_refresh_builder_identity" {
  count              = local.clinical_refresh_enabled ? 1 : 0
  service_account_id = google_service_account.clinical_retrieval_builder[0].name
  role               = "roles/iam.serviceAccountUser"
  member             = "serviceAccount:${google_service_account.clinical_refresh_workflow[0].email}"
}

resource "google_project_iam_member" "clinical_retrieval_batch_runtime" {
  for_each = var.clinical_retrieval_build_enabled ? toset([
    "roles/batch.agentReporter",
    "roles/logging.logWriter",
  ]) : toset([])

  project = var.project_id
  role    = each.value
  member  = "serviceAccount:${google_service_account.clinical_retrieval_builder[0].email}"
}

resource "google_artifact_registry_repository_iam_member" "clinical_retrieval_batch_image" {
  count      = var.clinical_retrieval_build_enabled ? 1 : 0
  project    = var.project_id
  location   = google_artifact_registry_repository.clinical_evidence.location
  repository = google_artifact_registry_repository.clinical_evidence.name
  role       = "roles/artifactregistry.reader"
  member     = "serviceAccount:${google_service_account.clinical_retrieval_builder[0].email}"
}
