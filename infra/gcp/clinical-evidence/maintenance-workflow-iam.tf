resource "google_project_iam_custom_role" "clinical_refresh_job_read" {
  count       = local.clinical_refresh_enabled ? 1 : 0
  project     = var.project_id
  role_id     = "CodaClinicalRefreshJobRead"
  title       = "Coda Clinical Refresh Job Read"
  description = "Inspect the exact clinical job definitions and reconcile their native executions."
  permissions = ["run.jobs.get", "run.executions.get", "run.executions.list"]
}

resource "google_cloud_run_v2_job_iam_member" "clinical_refresh_job_read" {
  for_each = local.clinical_refresh_guard_jobs
  project  = var.project_id
  location = each.value.location
  name     = each.value.name
  role     = google_project_iam_custom_role.clinical_refresh_job_read[0].name
  member   = "serviceAccount:${google_service_account.clinical_refresh_workflow[0].email}"
}
