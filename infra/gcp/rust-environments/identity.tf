locals {
  github_environment = { dev = "dev", stg = "staging", prd = "production" }[var.environment]
  github_subject     = "repo:Coda-Health/coda-rust:environment:${local.github_environment}"
  # Require the environment claim AND subject: a branch-only or PR token cannot
  # impersonate this identity. Immutable IDs defeat repository-name recycling.
  oidc_condition = join(" && ", [
    "assertion.repository == 'Coda-Health/coda-rust'",
    "assertion.repository_id == '${var.github_repository_id}'",
    "assertion.repository_owner_id == '${var.github_owner_id}'",
    "assertion.ref == 'refs/heads/main'",
    "assertion.environment == '${local.github_environment}'",
    "assertion.sub == '${local.github_subject}'",
    var.environment == "stg" ? "assertion.event_name == 'push'" : "assertion.event_name == 'workflow_dispatch'",
  ])
}

resource "google_service_account" "deployer" {
  project      = google_project.environment.project_id
  account_id   = "rust-${var.environment}-deployer"
  display_name = "coda-rust ${var.environment} release only"
  depends_on   = [google_project_service.api]
}

resource "google_iam_workload_identity_pool" "github" {
  project                   = google_project.environment.project_id
  workload_identity_pool_id = "coda-rust-${var.environment}"
  display_name              = "coda-rust ${var.environment} GitHub"
  depends_on                = [google_project_service.api]
}

resource "google_iam_workload_identity_pool_provider" "github" {
  project                            = google_project.environment.project_id
  workload_identity_pool_id          = google_iam_workload_identity_pool.github.workload_identity_pool_id
  workload_identity_pool_provider_id = "github"
  attribute_condition                = local.oidc_condition
  attribute_mapping = {
    "google.subject"                = "assertion.sub"
    "attribute.repository_id"       = "assertion.repository_id"
    "attribute.repository_owner_id" = "assertion.repository_owner_id"
    "attribute.environment"         = "assertion.environment"
    "attribute.ref"                 = "assertion.ref"
  }
  oidc { issuer_uri = "https://token.actions.githubusercontent.com" }
}

resource "google_service_account_iam_binding" "github" {
  service_account_id = google_service_account.deployer.name
  role               = "roles/iam.workloadIdentityUser"
  members = [
    "principal://iam.googleapis.com/${google_iam_workload_identity_pool.github.name}/subject/${local.github_subject}"
  ]
}

resource "google_artifact_registry_repository_iam_member" "publisher" {
  project    = google_project.environment.project_id
  location   = var.region
  repository = google_artifact_registry_repository.apps.name
  role       = "roles/artifactregistry.writer"
  member     = "serviceAccount:${google_service_account.deployer.email}"
}

# No run.admin/developer project-wide role, no create/delete or setIamPolicy,
# no SQL admin, no secret access, no state/billing/infrastructure authority.
# Cloud Run update is not field-scoped by IAM: the trusted release workflow must
# verify IAP, service identity, network and caps before and after each update.
resource "google_project_iam_custom_role" "release_service" {
  project     = google_project.environment.project_id
  role_id     = "rustReleaseService"
  title       = "Update existing Rust service revisions and traffic"
  permissions = ["run.services.get", "run.services.update", "run.revisions.get", "run.revisions.list"]
}

resource "google_project_iam_custom_role" "release_job" {
  project     = google_project.environment.project_id
  role_id     = "rustReleaseJob"
  title       = "Update and execute existing Rust migrations"
  permissions = ["run.jobs.get", "run.jobs.update", "run.jobs.run", "run.executions.get", "run.executions.list"]
}

resource "google_project_iam_custom_role" "release_operations" {
  project     = google_project.environment.project_id
  role_id     = "rustReleaseOperations"
  title       = "Poll asynchronous Cloud Run release operations"
  permissions = ["run.operations.get", "resourcemanager.projects.get"]
}

resource "google_project_iam_member" "release_operations" {
  project = google_project.environment.project_id
  role    = google_project_iam_custom_role.release_operations.name
  member  = "serviceAccount:${google_service_account.deployer.email}"
}

resource "google_service_account_iam_member" "release_act_as" {
  for_each           = var.enable_workloads ? local.identities : {}
  service_account_id = google_service_account.app[each.key].name
  role               = "roles/iam.serviceAccountUser"
  member             = "serviceAccount:${google_service_account.deployer.email}"
}

resource "google_cloud_run_v2_service_iam_member" "release" {
  for_each = local.workloads
  project  = google_project.environment.project_id
  location = var.region
  name     = google_cloud_run_v2_service.app[each.key].name
  role     = google_project_iam_custom_role.release_service.name
  member   = "serviceAccount:${google_service_account.deployer.email}"
}

resource "google_cloud_run_v2_job_iam_member" "release" {
  for_each = local.workloads
  project  = google_project.environment.project_id
  location = var.region
  name     = google_cloud_run_v2_job.migration[each.key].name
  role     = google_project_iam_custom_role.release_job.name
  member   = "serviceAccount:${google_service_account.deployer.email}"
}
