resource "google_storage_bucket_iam_member" "clinical_retrieval_builder_read" {
  count = var.clinical_retrieval_build_enabled ? 1 : 0

  bucket = var.clinical_corpus_bucket_name
  role   = "roles/storage.objectViewer"
  member = "serviceAccount:${google_service_account.clinical_retrieval_builder[0].email}"

  condition {
    title       = "production-clinical-builder-read"
    description = "Read only immutable retrieval build inputs and collision targets."
    expression  = "resource.name == 'projects/_/buckets/${var.clinical_corpus_bucket_name}' || resource.name.startsWith('projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/builds/')"
  }
}

resource "google_storage_bucket_iam_member" "clinical_retrieval_builder_create" {
  count = var.clinical_retrieval_build_enabled ? 1 : 0

  bucket = var.clinical_corpus_bucket_name
  role   = "roles/storage.objectCreator"
  member = "serviceAccount:${google_service_account.clinical_retrieval_builder[0].email}"

  condition {
    title       = "production-clinical-builder-create"
    description = "Create immutable retrieval builds without overwrite or deletion authority."
    expression  = "resource.name.startsWith('projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/builds/')"
  }
}

# Native Lance owns transactional manifests and the mutable latest-version hint.
# Immutable runtime assets and publication receipts retain create-only authority.
resource "google_storage_bucket_iam_member" "clinical_retrieval_builder_native_write" {
  count  = var.clinical_retrieval_build_enabled ? 1 : 0
  bucket = var.clinical_corpus_bucket_name
  role   = "roles/storage.objectUser"
  member = "serviceAccount:${google_service_account.clinical_retrieval_builder[0].email}"

  condition {
    title       = "production-clinical-builder-native-lance"
    description = "Write native Lance manifests and fragments only within the retrieval projection."
    expression  = "resource.name.startsWith('projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/builds/lance/')"
  }
}

resource "google_storage_bucket_iam_member" "clinical_retrieval_builder_refresh_read" {
  count = var.clinical_retrieval_build_enabled ? 1 : 0

  bucket = var.clinical_corpus_bucket_name
  role   = "roles/storage.objectViewer"
  member = "serviceAccount:${google_service_account.clinical_retrieval_builder[0].email}"

  condition {
    title       = "production-clinical-builder-refresh-read"
    description = "Read only exact immutable refresh receipts for collision verification."
    expression  = "resource.name.startsWith('projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/operations/retrieval-refresh/')"
  }
}

resource "google_storage_bucket_iam_member" "clinical_retrieval_builder_refresh_create" {
  count = var.clinical_retrieval_build_enabled ? 1 : 0

  bucket = var.clinical_corpus_bucket_name
  role   = "roles/storage.objectCreator"
  member = "serviceAccount:${google_service_account.clinical_retrieval_builder[0].email}"

  condition {
    title       = "production-clinical-builder-refresh-create"
    description = "Create immutable run-scoped retrieval refresh receipts without overwrite authority."
    expression  = "resource.name.startsWith('projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/operations/retrieval-refresh/')"
  }
}

resource "google_cloud_run_v2_job" "clinical_retrieval_builder" {
  count = var.clinical_retrieval_build_enabled ? 1 : 0

  project             = var.project_id
  name                = "coda-clinical-retrieval-builder"
  location            = var.gpu_region
  deletion_protection = true

  template {
    parallelism = 1
    task_count  = 1

    template {
      service_account               = google_service_account.clinical_retrieval_builder[0].email
      timeout                       = "1800s"
      max_retries                   = 0
      gpu_zonal_redundancy_disabled = true

      containers {
        name    = "retrieval-builder"
        image   = var.clinical_retrieval_build_image
        command = ["node"]
        args    = ["dist/src/server/clinicalCorpus/build/hostedClinicalRetrievalBuild.js"]

        resources {
          limits = {
            cpu              = "8"
            memory           = "32Gi"
            "nvidia.com/gpu" = "1"
          }
        }

        env {
          name  = "CODA_CLINICAL_CORPUS_BUCKET"
          value = var.clinical_corpus_bucket_name
        }
        env {
          name  = "CODA_CLOUD_SQL_INSTANCE"
          value = google_sql_database_instance.clinical_evidence.connection_name
        }
        env {
          name  = "CODA_CLOUD_SQL_IAM_USER"
          value = trimsuffix(google_service_account.clinical_retrieval_builder[0].email, ".gserviceaccount.com")
        }
        env {
          name = "CODA_CLINICAL_RETRIEVAL_BUILD_CONFIG"
          value = jsonencode({
            schema_version         = 2
            expected_database_name = local.clinical_database_name
            python_executable      = "/opt/coda-python/bin/python3"
            document_model = {
              archive_object = var.clinical_document_model_archive.object
              archive_sha256 = var.clinical_document_model_archive.sha256
              weights_sha256 = var.clinical_document_model_archive.weights_sha256
            }
            query_model = {
              archive_object = var.clinical_query_model_archive.object
              archive_sha256 = var.clinical_query_model_archive.sha256
            }
            cross_encoder = {
              archive_object = var.clinical_cross_encoder_archive.object
              archive_sha256 = var.clinical_cross_encoder_archive.sha256
            }
            batch_size      = var.clinical_retrieval_build_batch_size
            max_passages    = var.clinical_retrieval_max_passages
            required_device = "cuda"
          })
        }
      }

      node_selector {
        accelerator = "nvidia-l4"
      }

      vpc_access {
        egress = "PRIVATE_RANGES_ONLY"

        network_interfaces {
          network    = google_compute_network.clinical_evidence.name
          subnetwork = google_compute_subnetwork.retrieval.name
        }
      }
    }
  }

  depends_on = [
    google_project_service.workload,
    google_project_iam_member.clinical_retrieval_builder_cloud_sql,
    google_sql_user.clinical_retrieval_builder,
    google_storage_bucket_iam_member.clinical_retrieval_builder_read,
    google_storage_bucket_iam_member.clinical_retrieval_builder_create,
    google_storage_bucket_iam_member.clinical_retrieval_builder_refresh_read,
    google_storage_bucket_iam_member.clinical_retrieval_builder_refresh_create,
  ]
}
