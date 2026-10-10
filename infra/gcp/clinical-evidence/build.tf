resource "google_artifact_registry_repository" "clinical_evidence" {
  project       = var.project_id
  location      = var.region
  repository_id = "coda-clinical-evidence"
  description   = "Immutable production Clinical Evidence service and operation images"
  format        = "DOCKER"

  cleanup_policy_dry_run = false

  cleanup_policies {
    id     = "delete-untagged-after-seven-days"
    action = "DELETE"

    condition {
      tag_state  = "UNTAGGED"
      older_than = "604800s"
    }
  }

  cleanup_policies {
    id     = "keep-two-most-recent-images-per-package"
    action = "KEEP"

    most_recent_versions {
      keep_count = 2
    }
  }

  lifecycle {
    prevent_destroy = true
  }

  depends_on = [google_project_service.workload]
}

# Builds run as their own identity with only these three grants, not the compute default account.
resource "google_service_account" "cloud_build" {
  project      = var.project_id
  account_id   = "coda-clinical-build"
  display_name = "Coda clinical image build"

  depends_on = [google_project_service.workload]
}

resource "google_artifact_registry_repository_iam_member" "cloud_build_write" {
  project    = var.project_id
  location   = google_artifact_registry_repository.clinical_evidence.location
  repository = google_artifact_registry_repository.clinical_evidence.name
  role       = "roles/artifactregistry.writer"
  member     = "serviceAccount:${google_service_account.cloud_build.email}"

  depends_on = [google_project_service.workload]
}

resource "google_storage_bucket" "cloud_build_source" {
  project                     = var.project_id
  name                        = "${var.project_id}-clinical-build-source-${var.region}"
  location                    = var.region
  storage_class               = "STANDARD"
  uniform_bucket_level_access = true
  public_access_prevention    = "enforced"
  force_destroy               = false

  soft_delete_policy {
    retention_duration_seconds = 0
  }

  lifecycle_rule {
    action {
      type = "Delete"
    }
    condition {
      age = 1
    }
  }

  lifecycle {
    prevent_destroy = true
  }

  depends_on = [google_project_service.workload]
}

resource "google_storage_bucket_iam_member" "cloud_build_source_read" {
  bucket = google_storage_bucket.cloud_build_source.name
  role   = "roles/storage.objectViewer"
  member = "serviceAccount:${google_service_account.cloud_build.email}"

  depends_on = [google_project_service.workload]
}

resource "google_project_iam_member" "cloud_build_logs" {
  project = var.project_id
  role    = "roles/logging.logWriter"
  member  = "serviceAccount:${google_service_account.cloud_build.email}"

  depends_on = [google_project_service.workload]
}
