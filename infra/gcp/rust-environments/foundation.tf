locals {
  project_id         = var.projects[var.environment].project_id
  apps               = toset(["physician", "duet"])
  sql_instance_names = { for app in local.apps : app => "rust-${app}" }
  databases          = var.enable_databases ? local.apps : toset([])
  workloads          = var.enable_workloads ? local.apps : toset([])
  identities         = { for pair in setproduct(local.apps, ["runtime", "migration"]) : "${pair[0]}-${pair[1]}" => { app = pair[0], role = pair[1] } }
  services = toset([
    "artifactregistry.googleapis.com", "cloudresourcemanager.googleapis.com",
    "compute.googleapis.com", "iam.googleapis.com", "iamcredentials.googleapis.com",
    "iap.googleapis.com", "logging.googleapis.com", "monitoring.googleapis.com",
    "run.googleapis.com", "secretmanager.googleapis.com", "serviceusage.googleapis.com",
    "servicenetworking.googleapis.com", "sqladmin.googleapis.com", "storage.googleapis.com",
    "sts.googleapis.com",
  ])
}

resource "google_project" "environment" {
  project_id          = local.project_id
  name                = "Coda Rust ${var.environment}"
  org_id              = var.folder_id == null ? var.organization_id : null
  folder_id           = var.folder_id
  billing_account     = var.billing_account
  auto_create_network = false
  deletion_policy     = "PREVENT"
  labels              = { product = "coda-rust", environment = var.environment }
  lifecycle {
    prevent_destroy = true
    precondition {
      condition     = var.environment != "prd" || (!var.enable_workloads && !var.enable_databases) || var.production_activation_approved
      error_message = "Prod databases/workloads require separately recorded operator authorization."
    }
    precondition {
      condition     = !var.enable_workloads || (var.enable_databases && var.runtime_and_database_contract_approved)
      error_message = "Workloads require private databases and explicit runtime/SQL-grant acceptance."
    }
    precondition {
      condition = !var.enable_workloads || (
        length(distinct(values(var.doppler_configs))) == 4 &&
        alltrue([for key in keys(local.identities) : startswith(try(var.doppler_configs[key], ""), "${var.environment}_")]) &&
        alltrue(flatten([for config in concat(values(var.runtime_environment), values(var.migration_environment)) : [for key in keys(config) : !contains(["DOPPLER_TOKEN", "DOPPLER_PROJECT", "DOPPLER_CONFIG", "PORT"], key)]]))
      )
      error_message = "Active workloads need four distinct environment-derived least-scope Doppler configs and may not override bootstrap/port settings."
    }
    precondition {
      condition = !var.enable_workloads || alltrue([for app in local.apps :
        can(regex("^${var.region}-docker\\.pkg\\.dev/${local.project_id}/apps/[^@]+@sha256:[a-f0-9]{64}$", try(var.images[app].runtime, ""))) &&
        can(regex("^${var.region}-docker\\.pkg\\.dev/${local.project_id}/apps/[^@]+@sha256:[a-f0-9]{64}$", try(var.images[app].migration, ""))) &&
        can(regex("^[1-9][0-9]*$", try(var.doppler_token_versions[app].runtime, ""))) &&
        can(regex("^[1-9][0-9]*$", try(var.doppler_token_versions[app].migration, ""))) &&
        length(try(var.runtime_environment[app], {})) > 0
      ])
      error_message = "Each active app needs local registry SHA256 digests, pinned token versions, and approved nonsecret runtime config."
    }
  }
}

resource "google_project_service" "api" {
  for_each           = local.services
  project            = google_project.environment.project_id
  service            = each.value
  disable_on_destroy = false
}

resource "google_storage_bucket" "state" {
  project                     = google_project.environment.project_id
  name                        = var.projects[var.environment].state_bucket_name
  location                    = var.region
  uniform_bucket_level_access = true
  public_access_prevention    = "enforced"
  force_destroy               = false
  versioning { enabled = true }
  lifecycle_rule {
    condition {
      days_since_noncurrent_time = 90
      num_newer_versions         = 10
    }
    action { type = "Delete" }
  }
  lifecycle { prevent_destroy = true }
}

resource "google_storage_bucket_iam_binding" "state_operators" {
  bucket  = google_storage_bucket.state.name
  role    = "roles/storage.objectAdmin"
  members = var.state_admin_members
  # No CI/deployer binding to this bucket or its project-wide storage roles.
}

resource "google_artifact_registry_repository" "apps" {
  project       = google_project.environment.project_id
  location      = var.region
  repository_id = "apps"
  format        = "DOCKER"
  docker_config { immutable_tags = true }
  # No automatic image deletion: preserve rollback digests until release policy
  # supplies an evidence-backed keep set. Budget covers retained registry storage.
  depends_on = [google_project_service.api]
}

# Billing-account administrators manage budgets outside this engineering state.
# Refreshing or planning project infrastructure must not require billing access.

resource "google_logging_project_bucket_config" "default" {
  project        = google_project.environment.project_id
  location       = "global"
  bucket_id      = "_Default"
  retention_days = 30
  depends_on     = [google_project_service.api]
}

resource "google_service_account" "app" {
  for_each     = local.identities
  project      = google_project.environment.project_id
  account_id   = each.key
  display_name = "${each.value.app} ${each.value.role} (${var.environment})"
  depends_on   = [google_project_service.api]
}

resource "google_secret_manager_secret" "doppler" {
  for_each  = local.identities
  project   = google_project.environment.project_id
  secret_id = "${each.key}-doppler-token"
  labels    = { environment = var.environment, app = each.value.app, purpose = "doppler-bootstrap" }
  replication {
    auto {}
  }
  depends_on = [google_project_service.api]
  # Deliberately NO secret_version resource/data source. Operator writes only a
  # read-only, least-scope Doppler token for this product/role/config out of band.
}

resource "google_secret_manager_secret_iam_binding" "bootstrap_reader" {
  for_each  = local.identities
  project   = google_project.environment.project_id
  secret_id = google_secret_manager_secret.doppler[each.key].secret_id
  role      = "roles/secretmanager.secretAccessor"
  members   = ["serviceAccount:${google_service_account.app[each.key].email}"]
}
