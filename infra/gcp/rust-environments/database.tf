resource "google_compute_network" "private" {
  project                 = google_project.environment.project_id
  name                    = "rust-private"
  auto_create_subnetworks = false
  depends_on              = [google_project_service.api]
}

resource "google_compute_subnetwork" "run" {
  project                  = google_project.environment.project_id
  name                     = "rust-run"
  region                   = var.region
  network                  = google_compute_network.private.id
  ip_cidr_range            = "10.80.0.0/24"
  private_ip_google_access = true
}

resource "google_compute_global_address" "sql" {
  project       = google_project.environment.project_id
  name          = "rust-sql-private"
  purpose       = "VPC_PEERING"
  address_type  = "INTERNAL"
  address       = "10.81.0.0"
  prefix_length = 16
  network       = google_compute_network.private.id
}

resource "google_service_networking_connection" "sql" {
  network                 = google_compute_network.private.id
  service                 = "servicenetworking.googleapis.com"
  reserved_peering_ranges = [google_compute_global_address.sql.name]
  depends_on              = [google_project_service.api]
}

# Two instances intentionally cost more than two databases in one instance:
# instance-scoped IAM and PostgreSQL catalog/role authority never cross products.
# Scale-to-zero Cloud Run does NOT stop SQL billing. No public IP or authorized
# external networks. Direct VPC egress reaches the private endpoint.
resource "google_sql_database_instance" "app" {
  for_each            = local.databases
  project             = google_project.environment.project_id
  name                = local.sql_instance_names[each.key]
  region              = var.region
  database_version    = "POSTGRES_17"
  deletion_protection = true
  settings {
    tier                        = var.sql_tier
    edition                     = "ENTERPRISE"
    availability_type           = var.environment == "prd" ? "REGIONAL" : "ZONAL"
    deletion_protection_enabled = true
    disk_size                   = 20
    disk_type                   = "PD_SSD"
    disk_autoresize             = true
    disk_autoresize_limit       = var.sql_disk_limit_gb
    ip_configuration {
      ipv4_enabled    = false
      private_network = google_compute_network.private.id
      ssl_mode        = "ENCRYPTED_ONLY"
    }
    database_flags {
      name  = "cloudsql.iam_authentication"
      value = "on"
    }
    database_flags {
      name  = "max_connections"
      value = "100"
    }
    backup_configuration {
      enabled                        = true
      point_in_time_recovery_enabled = true
      transaction_log_retention_days = 7
      start_time                     = "03:00"
      backup_retention_settings { retained_backups = 7 }
    }
    maintenance_window {
      day          = 7
      hour         = 4
      update_track = "stable"
    }
  }
  depends_on = [google_service_networking_connection.sql]
  lifecycle { prevent_destroy = true }
}

resource "google_sql_database" "app" {
  for_each = local.databases
  project  = google_project.environment.project_id
  instance = google_sql_database_instance.app[each.key].name
  name     = each.key
  lifecycle { prevent_destroy = true }
}

resource "google_sql_user" "app" {
  for_each            = var.enable_databases ? local.identities : {}
  project             = google_project.environment.project_id
  instance            = google_sql_database_instance.app[each.value.app].name
  name                = "${each.value.app}-${each.value.role}"
  type                = "BUILT_IN"
  password_wo         = lookup(var.database_passwords, each.key, null)
  password_wo_version = try(var.database_password_versions[each.key], 1)
  deletion_policy     = "ABANDON"
  # Passwords are required when databases are enabled and supplied from
  # Doppler through this write-only field; no password value enters state.
  # These users get no SQL
  # grants here and MUST NOT receive cloudsqlsuperuser. An authorized DB
  # operator runs bootstrap-grants.sql before acknowledging
  # runtime_and_database_contract_approved.
}

resource "google_project_iam_member" "sql_login" {
  # Bootstrap project IAM before engineers activate the separately gated databases.
  for_each = local.identities
  project  = google_project.environment.project_id
  role     = "roles/cloudsql.instanceUser"
  member   = "serviceAccount:${google_service_account.app[each.key].email}"
  condition {
    title      = "only-${each.value.app}-instance"
    expression = "resource.name == 'projects/${local.project_id}/instances/${local.sql_instance_names[each.value.app]}'"
  }
}

resource "google_project_iam_member" "sql_connect" {
  for_each = local.identities
  project  = google_project.environment.project_id
  role     = "roles/cloudsql.client"
  member   = "serviceAccount:${google_service_account.app[each.key].email}"
  condition {
    title      = "only-${each.value.app}-instance"
    expression = "resource.name == 'projects/${local.project_id}/instances/${local.sql_instance_names[each.value.app]}'"
  }
}
