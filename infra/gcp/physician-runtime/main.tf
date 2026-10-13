locals {
  workload_services = toset([
    "artifactregistry.googleapis.com",
    "cloudresourcemanager.googleapis.com",
    "compute.googleapis.com",
    "container.googleapis.com",
    "iam.googleapis.com",
    "serviceusage.googleapis.com",
    "servicenetworking.googleapis.com",
    "sqladmin.googleapis.com",
  ])
  database_name = "physician_runtime"
  # The socket form keeps the database URL free of passwords and of any
  # public address: IAM login plus the Cloud SQL Unix socket mount.
  site_connection        = "postgres://${trimsuffix(google_service_account.physician_site.email, ".gserviceaccount.com")}@/${local.database_name}?host=/cloudsql/${google_sql_database_instance.physician.connection_name}"
  site_replicas          = 2
  site_port              = 8790
  site_account           = google_service_account.physician_site.email
  migration_account      = google_service_account.database_migrator.email
  physician_site_gsa_urn = "serviceAccount:${local.site_account}"
  migration_gsa_urn      = "serviceAccount:${local.migration_account}"
}

data "google_project" "physician_runtime" {
  project_id = var.project_id
}

resource "google_project_service" "workload" {
  for_each = local.workload_services

  project = var.project_id
  service = each.value

  disable_on_destroy = false
}

resource "google_project_service_identity" "cloud_sql" {
  provider = google-beta

  project = var.project_id
  service = "sqladmin.googleapis.com"

  depends_on = [google_project_service.workload]
}
