output "artifact_registry_repository" {
  value = google_artifact_registry_repository.clinical_evidence.name
}

output "cloud_build_service_account" {
  value = google_service_account.cloud_build.id
}

output "cloud_build_source_bucket" {
  value = google_storage_bucket.cloud_build_source.name
}

output "cloud_sql_connection_name" {
  value = google_sql_database_instance.clinical_evidence.connection_name
}

output "cloud_sql_service_identity" {
  value = google_project_service_identity.cloud_sql.email
}

output "clinical_database" {
  value = google_sql_database.clinical.name
}

output "clinical_retrieval_service_account" {
  value = google_service_account.clinical_retrieval.email
}

output "clinical_activator_service_account" {
  value = google_service_account.clinical_activator.email
}

output "database_migrator_service_account" {
  value = google_service_account.database_migrator.email
}

output "cloud_run_clinical_evidence_service" {
  value = one(google_cloud_run_v2_service.clinical_evidence[*].name)
}

output "cloud_run_clinical_evidence_url" {
  value = one(google_cloud_run_v2_service.clinical_evidence[*].uri)
}

output "cloud_run_migration_job" {
  value = one(google_cloud_run_v2_job.database_migrator[*].name)
}

output "cloud_run_clinical_retrieval_activation_job" {
  value = one(google_cloud_run_v2_job.clinical_retrieval_activator[*].name)
}

output "cloud_run_clinical_retrieval_build_job" {
  value = one(google_cloud_run_v2_job.clinical_retrieval_builder[*].name)
}

output "clinical_source_acquisition_batch_prefix" {
  value = var.clinical_source_acquisition_enabled ? "projects/${var.project_id}/locations/${var.region}/jobs/coda-acquisition-" : null
}

output "cloud_run_clinical_source_admission_job" {
  value = one(google_cloud_run_v2_job.clinical_source_admission[*].name)
}

output "cloud_run_clinical_source_finalization_job" {
  value = one(google_cloud_run_v2_job.clinical_source_finalization[*].name)
}

output "cloud_run_clinical_payload_reconciliation_job" {
  value = try(google_cloud_run_v2_job.clinical_payload_reconciliation[0].name, null)
}

output "cloud_scheduler_clinical_source_acquisition_job" {
  value = one(google_cloud_scheduler_job.clinical_source_acquisition[*].name)
}

output "clinical_corpus_refresh_workflow" {
  value = one(google_workflows_workflow.clinical_refresh[*].name)
}
