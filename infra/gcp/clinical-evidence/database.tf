resource "google_service_account" "clinical_retrieval" {
  project      = var.project_id
  account_id   = "coda-clinical-retrieval"
  display_name = "Coda production clinical evidence retrieval"

  depends_on = [google_project_service.workload]
}

resource "google_service_account" "clinical_activator" {
  project      = var.project_id
  account_id   = "coda-clinical-activator"
  display_name = "Coda production clinical retrieval activator"

  depends_on = [google_project_service.workload]
}

resource "google_service_account" "clinical_retrieval_builder" {
  count = var.clinical_retrieval_build_enabled ? 1 : 0

  project      = var.project_id
  account_id   = "coda-clinical-builder"
  display_name = "Coda production clinical retrieval builder"

  depends_on = [google_project_service.workload]
}

resource "google_service_account" "clinical_corpus_maintainer" {
  count = var.clinical_source_acquisition_enabled ? 1 : 0

  project      = var.project_id
  account_id   = "coda-clinical-maintainer"
  display_name = "Coda production clinical corpus maintainer"

  depends_on = [google_project_service.workload]
}

resource "google_service_account" "database_migrator" {
  project      = var.project_id
  account_id   = "coda-clinical-migrator"
  display_name = "Coda production clinical database migrator"

  depends_on = [google_project_service.workload]
}

resource "google_project_iam_member" "clinical_retrieval_cloud_sql" {
  for_each = toset([
    "roles/cloudsql.client",
    "roles/cloudsql.instanceUser",
  ])

  project = var.project_id
  role    = each.value
  member  = "serviceAccount:${google_service_account.clinical_retrieval.email}"
}

resource "google_project_iam_member" "clinical_activator_cloud_sql" {
  for_each = toset([
    "roles/cloudsql.client",
    "roles/cloudsql.instanceUser",
  ])

  project = var.project_id
  role    = each.value
  member  = "serviceAccount:${google_service_account.clinical_activator.email}"
}

resource "google_project_iam_member" "clinical_retrieval_builder_cloud_sql" {
  for_each = var.clinical_retrieval_build_enabled ? toset([
    "roles/cloudsql.client",
    "roles/cloudsql.instanceUser",
  ]) : toset([])

  project = var.project_id
  role    = each.value
  member  = "serviceAccount:${google_service_account.clinical_retrieval_builder[0].email}"
}

resource "google_project_iam_member" "clinical_corpus_maintainer_cloud_sql" {
  for_each = var.clinical_source_acquisition_enabled ? toset([
    "roles/cloudsql.client",
    "roles/cloudsql.instanceUser",
  ]) : toset([])

  project = var.project_id
  role    = each.value
  member  = "serviceAccount:${google_service_account.clinical_corpus_maintainer[0].email}"
}

resource "google_project_iam_member" "migrator_cloud_sql" {
  for_each = toset([
    "roles/cloudsql.client",
    "roles/cloudsql.instanceUser",
  ])

  project = var.project_id
  role    = each.value
  member  = "serviceAccount:${google_service_account.database_migrator.email}"
}

resource "google_storage_bucket_iam_member" "clinical_retrieval_build_read" {
  bucket = var.clinical_corpus_bucket_name
  role   = "roles/storage.objectViewer"
  member = "serviceAccount:${google_service_account.clinical_retrieval.email}"

  condition {
    title       = "production-clinical-runtime-build-read"
    description = "Allow serving to materialize only immutable retrieval runtime builds."
    expression  = "resource.name == 'projects/_/buckets/${var.clinical_corpus_bucket_name}' || resource.name.startsWith('projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/builds/')"
  }
}

resource "google_storage_bucket_iam_member" "clinical_activator_refresh_read" {
  bucket = var.clinical_corpus_bucket_name
  role   = "roles/storage.objectViewer"
  member = "serviceAccount:${google_service_account.clinical_activator.email}"

  condition {
    title       = "production-clinical-activator-refresh-read"
    description = "Read run-scoped receipts and immutable publication descriptors for activation; no index, payload or write access."
    expression  = "resource.name.startsWith('projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/operations/retrieval-refresh/') || (resource.name.startsWith('projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/builds/sha256/') && resource.name.endsWith('/clinical-retrieval-runtime-publication.json'))"
  }
}

resource "google_storage_bucket_iam_member" "clinical_retrieval_payload_read" {
  bucket = var.clinical_corpus_bucket_name
  role   = "roles/storage.objectViewer"
  member = "serviceAccount:${google_service_account.clinical_retrieval.email}"

  condition {
    title       = "production-clinical-runtime-payload-read"
    description = "Read exact immutable source visuals after SQL rights authorization; no write or deletion."
    expression  = "resource.name.startsWith('projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/sha256/')"
  }
}

resource "google_sql_database_instance" "clinical_evidence" {
  project             = var.project_id
  name                = "coda-clinical-evidence-prod"
  region              = var.region
  database_version    = "POSTGRES_16"
  deletion_protection = true

  settings {
    tier                        = var.sql_tier
    edition                     = "ENTERPRISE"
    availability_type           = "ZONAL"
    activation_policy           = var.sql_activation_policy
    deletion_protection_enabled = true
    disk_type                   = "PD_SSD"
    disk_size                   = 20
    disk_autoresize             = true
    disk_autoresize_limit       = var.sql_disk_autoresize_limit_gb

    backup_configuration {
      enabled = false
    }

    ip_configuration {
      ipv4_enabled                                  = false
      private_network                               = google_compute_network.clinical_evidence.id
      enable_private_path_for_google_cloud_services = true
    }

    database_flags {
      name  = "cloudsql.iam_authentication"
      value = "on"
    }

    maintenance_window {
      day          = 7
      hour         = 7
      update_track = "stable"
    }

    user_labels = {
      application         = "coda-clinical-evidence"
      environment         = "production"
      data_classification = "public-clinical-evidence"
    }
  }

  lifecycle {
    prevent_destroy = true
  }

  depends_on = [google_service_networking_connection.private_services]
}

resource "google_storage_bucket_iam_member" "cloud_sql_build_read" {
  bucket = var.clinical_corpus_bucket_name
  role   = "roles/storage.objectViewer"
  member = "serviceAccount:${google_sql_database_instance.clinical_evidence.service_account_email_address}"

  condition {
    title       = "production-clinical-builds-only"
    description = "Allow the production clinical database to import immutable build artifacts only."
    expression  = "resource.name.startsWith('projects/_/buckets/${var.clinical_corpus_bucket_name}/objects/builds/')"
  }
}

resource "google_storage_bucket_iam_member" "cloud_sql_bucket_read" {
  bucket = var.clinical_corpus_bucket_name
  role   = "roles/storage.legacyBucketReader"
  member = "serviceAccount:${google_sql_database_instance.clinical_evidence.service_account_email_address}"
}

resource "google_sql_database" "clinical" {
  project  = var.project_id
  name     = local.clinical_database_name
  instance = google_sql_database_instance.clinical_evidence.name
}

resource "google_sql_user" "clinical_retrieval" {
  project  = var.project_id
  name     = trimsuffix(google_service_account.clinical_retrieval.email, ".gserviceaccount.com")
  instance = google_sql_database_instance.clinical_evidence.name
  type     = "CLOUD_IAM_SERVICE_ACCOUNT"
}

resource "google_sql_user" "clinical_activator" {
  project  = var.project_id
  name     = trimsuffix(google_service_account.clinical_activator.email, ".gserviceaccount.com")
  instance = google_sql_database_instance.clinical_evidence.name
  type     = "CLOUD_IAM_SERVICE_ACCOUNT"
}

resource "google_sql_user" "clinical_retrieval_builder" {
  count = var.clinical_retrieval_build_enabled ? 1 : 0

  project  = var.project_id
  name     = trimsuffix(google_service_account.clinical_retrieval_builder[0].email, ".gserviceaccount.com")
  instance = google_sql_database_instance.clinical_evidence.name
  type     = "CLOUD_IAM_SERVICE_ACCOUNT"
}

resource "google_sql_user" "clinical_corpus_maintainer" {
  count = var.clinical_source_acquisition_enabled ? 1 : 0

  project  = var.project_id
  name     = trimsuffix(google_service_account.clinical_corpus_maintainer[0].email, ".gserviceaccount.com")
  instance = google_sql_database_instance.clinical_evidence.name
  type     = "CLOUD_IAM_SERVICE_ACCOUNT"
}

resource "google_sql_user" "database_migrator" {
  project        = var.project_id
  name           = trimsuffix(google_service_account.database_migrator.email, ".gserviceaccount.com")
  instance       = google_sql_database_instance.clinical_evidence.name
  type           = "CLOUD_IAM_SERVICE_ACCOUNT"
  database_roles = ["cloudsqlsuperuser"]
}
