locals {
  production_control_plane_services = toset([
    "billingbudgets.googleapis.com",
    "cloudbilling.googleapis.com",
    "cloudresourcemanager.googleapis.com",
    "serviceusage.googleapis.com",
    "storage.googleapis.com",
  ])
}

resource "google_project_service" "production_control_plane" {
  for_each = local.production_control_plane_services

  project = var.production_project_id
  service = each.value

  disable_on_destroy = false
}

resource "google_project" "staging" {
  project_id      = var.staging_project_id
  name            = "Coda Health Staging"
  org_id          = var.organization_id
  billing_account = var.billing_account_id
  deletion_policy = "PREVENT"

  labels = {
    application         = "coda-agents"
    environment         = "staging"
    data_classification = "non-phi"
  }

  depends_on = [google_project_service.production_control_plane]
}

resource "google_billing_budget" "staging" {
  billing_account = var.billing_account_id
  display_name    = "Coda Agents Staging Guardrail"

  budget_filter {
    projects = ["projects/${google_project.staging.number}"]
  }

  amount {
    specified_amount {
      currency_code = "USD"
      units         = tostring(var.staging_monthly_budget_usd)
    }
  }

  dynamic "threshold_rules" {
    for_each = toset([0.25, 0.50, 0.90, 1.00])
    content {
      threshold_percent = threshold_rules.value
      spend_basis       = "CURRENT_SPEND"
    }
  }

  depends_on = [google_project_service.production_control_plane]
}

resource "google_storage_bucket" "terraform_state" {
  project                     = var.production_project_id
  name                        = var.state_bucket_name
  location                    = var.region
  storage_class               = "STANDARD"
  uniform_bucket_level_access = true
  public_access_prevention    = "enforced"
  force_destroy               = false

  versioning {
    enabled = true
  }

  soft_delete_policy {
    retention_duration_seconds = 604800
  }

  lifecycle {
    prevent_destroy = true
  }

  depends_on = [google_project_service.production_control_plane]
}

resource "google_storage_bucket" "clinical_corpus" {
  project                     = var.production_project_id
  name                        = var.clinical_corpus_bucket_name
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

  depends_on = [google_project_service.production_control_plane]
}
