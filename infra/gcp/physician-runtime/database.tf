resource "google_sql_database_instance" "physician" {
  project             = var.project_id
  name                = "coda-physician-runtime"
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

    ip_configuration {
      ipv4_enabled                                  = false
      private_network                               = google_compute_network.physician.id
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
      application         = "physician-runtime"
      environment         = var.project_id
      data_classification = "synthetic-non-phi"
    }
  }

  lifecycle {
    prevent_destroy = true
  }

  depends_on = [
    google_project_service.workload,
    google_service_networking_connection.private_services,
  ]
}

resource "google_sql_database" "physician" {
  project  = var.project_id
  name     = local.database_name
  instance = google_sql_database_instance.physician.name
}

# The site connects as its own IAM service account; no password exists.
resource "google_sql_user" "physician_site" {
  project  = var.project_id
  name     = trimsuffix(local.site_account, ".gserviceaccount.com")
  instance = google_sql_database_instance.physician.name
  type     = "CLOUD_IAM_SERVICE_ACCOUNT"
}

# The migration job owns schema changes and so holds the database admin role.
resource "google_sql_user" "database_migrator" {
  project        = var.project_id
  name           = trimsuffix(local.migration_account, ".gserviceaccount.com")
  instance       = google_sql_database_instance.physician.name
  type           = "CLOUD_IAM_SERVICE_ACCOUNT"
  database_roles = ["cloudsqlsuperuser"]
}
