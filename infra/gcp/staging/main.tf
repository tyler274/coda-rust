locals {
  workload_services = toset([
    "artifactregistry.googleapis.com",
    "cloudbuild.googleapis.com",
    "cloudresourcemanager.googleapis.com",
    "compute.googleapis.com",
    "iam.googleapis.com",
    "serviceusage.googleapis.com",
    "servicenetworking.googleapis.com",
    "sqladmin.googleapis.com",
    "storage.googleapis.com",
  ])
  runtime_database_name       = "coda_runtime_staging"
  cloud_build_service_account = "${data.google_project.staging.number}-compute@developer.gserviceaccount.com"
}

check "interactive_proof_window" {
  assert {
    condition     = !var.proof_window_active || var.application_service_enabled
    error_message = "proof_window_active requires the application service."
  }
}

data "google_project" "staging" {
  project_id = var.project_id
}

resource "google_project_service" "runtime" {
  count = (
    var.application_image == null &&
    var.migration_image == null
  ) ? 0 : 1

  project = var.project_id
  service = "run.googleapis.com"

  disable_on_destroy = false
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

resource "google_compute_network" "staging" {
  project                 = var.project_id
  name                    = "coda-agents-staging"
  auto_create_subnetworks = false
  routing_mode            = "REGIONAL"

  depends_on = [google_project_service.workload]
}

resource "google_compute_subnetwork" "runtime" {
  project                  = var.project_id
  name                     = "coda-agents-runtime-us-east1"
  region                   = var.region
  network                  = google_compute_network.staging.id
  ip_cidr_range            = var.network_cidr
  private_ip_google_access = true
}

resource "google_compute_global_address" "private_services" {
  project       = var.project_id
  name          = "coda-agents-private-services"
  purpose       = "VPC_PEERING"
  address_type  = "INTERNAL"
  prefix_length = 24
  network       = google_compute_network.staging.id
}

resource "google_service_networking_connection" "private_services" {
  network                 = google_compute_network.staging.id
  service                 = "servicenetworking.googleapis.com"
  reserved_peering_ranges = [google_compute_global_address.private_services.name]
}

resource "google_service_account" "application" {
  project      = var.project_id
  account_id   = "coda-application"
  display_name = "Coda hosted application"

  depends_on = [google_project_service.workload]
}

resource "google_service_account" "database_migrator" {
  project      = var.project_id
  account_id   = "coda-database-migrator"
  display_name = "Coda database migrator"

  depends_on = [google_project_service.workload]
}

resource "google_project_iam_member" "application_cloud_sql" {
  for_each = toset([
    "roles/cloudsql.client",
    "roles/cloudsql.instanceUser",
  ])

  project = var.project_id
  role    = each.value
  member  = "serviceAccount:${google_service_account.application.email}"
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

resource "google_sql_database_instance" "staging" {
  project             = var.project_id
  name                = "coda-agents-staging"
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
    disk_autoresize_limit       = 100

    backup_configuration {
      enabled = false
    }

    ip_configuration {
      ipv4_enabled                                  = false
      private_network                               = google_compute_network.staging.id
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
      application         = "coda-agents"
      environment         = "staging"
      data_classification = "synthetic-non-phi"
    }
  }

  lifecycle {
    prevent_destroy = true
  }

  depends_on = [google_service_networking_connection.private_services]
}

resource "google_sql_database" "runtime" {
  project  = var.project_id
  name     = local.runtime_database_name
  instance = google_sql_database_instance.staging.name
}

resource "google_sql_user" "application" {
  project  = var.project_id
  name     = trimsuffix(google_service_account.application.email, ".gserviceaccount.com")
  instance = google_sql_database_instance.staging.name
  type     = "CLOUD_IAM_SERVICE_ACCOUNT"
}

resource "google_sql_user" "database_migrator" {
  project        = var.project_id
  name           = trimsuffix(google_service_account.database_migrator.email, ".gserviceaccount.com")
  instance       = google_sql_database_instance.staging.name
  type           = "CLOUD_IAM_SERVICE_ACCOUNT"
  database_roles = ["cloudsqlsuperuser"]
}

resource "google_storage_bucket" "runtime_evidence" {
  project                     = var.project_id
  name                        = "${var.project_id}-runtime-evidence-${var.region}"
  location                    = var.region
  storage_class               = "STANDARD"
  uniform_bucket_level_access = true
  public_access_prevention    = "enforced"
  force_destroy               = false

  soft_delete_policy {
    retention_duration_seconds = 604800
  }

  lifecycle {
    prevent_destroy = true
  }

  depends_on = [google_project_service.workload]
}

resource "google_storage_bucket_iam_member" "runtime_evidence_create" {
  bucket = google_storage_bucket.runtime_evidence.name
  role   = "roles/storage.objectCreator"
  member = "serviceAccount:${google_service_account.application.email}"
}

resource "google_storage_bucket_iam_member" "runtime_evidence_read" {
  bucket = google_storage_bucket.runtime_evidence.name
  role   = "roles/storage.objectViewer"
  member = "serviceAccount:${google_service_account.application.email}"
}

resource "google_artifact_registry_repository" "runtime" {
  project       = var.project_id
  location      = var.region
  repository_id = "coda-agents"
  description   = "Immutable Coda Agents staging runtime images"
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

  lifecycle {
    prevent_destroy = true
  }

  depends_on = [google_project_service.workload]
}

resource "google_artifact_registry_repository_iam_member" "cloud_build_write" {
  project    = var.project_id
  location   = google_artifact_registry_repository.runtime.location
  repository = google_artifact_registry_repository.runtime.name
  role       = "roles/artifactregistry.writer"
  member     = "serviceAccount:${local.cloud_build_service_account}"
}

resource "google_storage_bucket_iam_member" "cloud_build_source_read" {
  bucket = "${var.project_id}_cloudbuild"
  role   = "roles/storage.objectViewer"
  member = "serviceAccount:${local.cloud_build_service_account}"

  depends_on = [google_project_service.workload]
}

resource "google_project_iam_member" "cloud_build_logs" {
  project = var.project_id
  role    = "roles/logging.logWriter"
  member  = "serviceAccount:${local.cloud_build_service_account}"
}

resource "google_cloud_run_v2_service" "application" {
  count = var.application_image == null || !var.application_service_enabled ? 0 : 1

  project             = var.project_id
  name                = "coda-agents"
  location            = var.region
  deletion_protection = true
  ingress             = "INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER"

  template {
    service_account                  = google_service_account.application.email
    timeout                          = "3600s"
    max_instance_request_concurrency = 1

    scaling {
      min_instance_count = var.proof_window_active ? 1 : 0
      max_instance_count = 1
    }

    containers {
      image = var.application_image

      resources {
        limits = {
          cpu    = "2"
          memory = "8Gi"
        }
        cpu_idle          = false
        startup_cpu_boost = true
      }

      env {
        name  = "CODA_CLOUD_SQL_INSTANCE"
        value = google_sql_database_instance.staging.connection_name
      }
      env {
        name  = "CODA_CLOUD_SQL_IAM_USER"
        value = trimsuffix(google_service_account.application.email, ".gserviceaccount.com")
      }
      env {
        name  = "CODA_RUNTIME_DATABASE"
        value = google_sql_database.runtime.name
      }
      env {
        name  = "CODA_RUNTIME_EVIDENCE_BUCKET"
        value = google_storage_bucket.runtime_evidence.name
      }
      env {
        name  = "CODA_BROWSER_ORIGIN"
        value = var.application_browser_origin
      }
      env {
        name  = "CODA_CLINICAL_EVIDENCE_URL"
        value = var.clinical_evidence_service_url
      }
      env {
        name  = "CODA_CLINICAL_EVIDENCE_AUDIENCE"
        value = var.clinical_evidence_service_url
      }
      env {
        name  = "CODA_CLINICAL_EVIDENCE_AUTH_MODE"
        value = "google-id-token"
      }
      env {
        name  = "CODA_ALLOW_MODEL_CALLS"
        value = "1"
      }

      startup_probe {
        initial_delay_seconds = 20
        timeout_seconds       = 5
        period_seconds        = 10
        failure_threshold     = 30

        http_get {
          path = "/health"
          port = 8080
        }
      }
    }

    vpc_access {
      egress = "PRIVATE_RANGES_ONLY"

      network_interfaces {
        network    = google_compute_network.staging.name
        subnetwork = google_compute_subnetwork.runtime.name
      }
    }
  }

  lifecycle {
    precondition {
      condition     = !var.proof_window_active || var.application_service_enabled
      error_message = "An active interactive proof window requires the application service."
    }
  }

  depends_on = [
    google_project_service.runtime,
    google_project_iam_member.application_cloud_sql,
    google_sql_user.application,
    google_storage_bucket_iam_member.runtime_evidence_create,
    google_storage_bucket_iam_member.runtime_evidence_read,
  ]
}

resource "google_cloud_run_v2_job" "database_migrator" {
  count = var.migration_image == null ? 0 : 1

  project             = var.project_id
  name                = "coda-database-migrator"
  location            = var.region
  deletion_protection = true

  template {
    template {
      service_account = google_service_account.database_migrator.email
      timeout         = "900s"
      max_retries     = 0

      containers {
        image   = var.migration_image
        command = ["node"]
        args    = ["dist/src/server/migrateHostedRuntimeDatabase.js"]

        resources {
          limits = {
            cpu    = "1"
            memory = "1Gi"
          }
        }

        env {
          name  = "CODA_CLOUD_SQL_INSTANCE"
          value = google_sql_database_instance.staging.connection_name
        }
        env {
          name  = "CODA_CLOUD_SQL_MIGRATION_USER"
          value = trimsuffix(google_service_account.database_migrator.email, ".gserviceaccount.com")
        }
        env {
          name  = "CODA_CLOUD_SQL_APPLICATION_USER"
          value = trimsuffix(google_service_account.application.email, ".gserviceaccount.com")
        }
        env {
          name  = "CODA_RUNTIME_DATABASE"
          value = google_sql_database.runtime.name
        }
      }

      vpc_access {
        egress = "PRIVATE_RANGES_ONLY"

        network_interfaces {
          network    = google_compute_network.staging.name
          subnetwork = google_compute_subnetwork.runtime.name
        }
      }
    }
  }

  depends_on = [
    google_project_service.runtime,
    google_project_iam_member.migrator_cloud_sql,
    google_sql_user.database_migrator,
    google_sql_user.application,
  ]
}
