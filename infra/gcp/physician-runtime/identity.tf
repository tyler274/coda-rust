resource "google_service_account" "physician_site" {
  project      = var.project_id
  account_id   = "coda-physician-site"
  display_name = "Coda physician runtime site"

  depends_on = [google_project_service.workload]
}

resource "google_service_account" "database_migrator" {
  project      = var.project_id
  account_id   = "coda-physician-migrator"
  display_name = "Coda physician runtime database migrator"

  depends_on = [google_project_service.workload]
}

resource "google_artifact_registry_repository" "physician" {
  project       = var.project_id
  location      = var.region
  repository_id = "physician-site"
  description   = "Immutable physician runtime images"
  format        = "DOCKER"

  depends_on = [google_project_service.workload]
}

resource "google_project_iam_member" "physician_site_artifact_read" {
  project = var.project_id
  role    = "roles/artifactregistry.reader"
  member  = local.physician_site_gsa_urn
}

resource "google_project_iam_member" "physician_site_cloud_sql" {
  for_each = toset([
    "roles/cloudsql.client",
    "roles/cloudsql.instanceUser",
  ])

  project = var.project_id
  role    = each.value
  member  = local.physician_site_gsa_urn
}

resource "google_project_iam_member" "database_migrator_cloud_sql" {
  for_each = toset([
    "roles/cloudsql.client",
    "roles/cloudsql.instanceUser",
  ])

  project = var.project_id
  role    = each.value
  member  = local.migration_gsa_urn
}

# Keyless database authentication: each identity logs in as itself.
resource "google_project_iam_member" "physician_site_sql_login" {
  project = var.project_id
  role    = "roles/cloudsql.instanceUser"
  member  = local.physician_site_gsa_urn
}

resource "google_project_iam_member" "database_migrator_sql_login" {
  project = var.project_id
  role    = "roles/cloudsql.instanceUser"
  member  = local.migration_gsa_urn
}

# The site may pull only from the runtime repository it runs.
resource "google_artifact_registry_repository_iam_member" "physician_site_read" {
  project    = var.project_id
  location   = var.region
  repository = google_artifact_registry_repository.physician.name
  role       = "roles/artifactregistry.reader"
  member     = local.physician_site_gsa_urn
}
