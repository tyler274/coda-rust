output "artifact_registry_repository" {
  value = google_artifact_registry_repository.runtime.name
}

output "cloud_sql_connection_name" {
  value = google_sql_database_instance.staging.connection_name
}

output "cloud_sql_service_identity" {
  value = google_project_service_identity.cloud_sql.email
}

output "runtime_database" {
  value = google_sql_database.runtime.name
}

output "runtime_evidence_bucket" {
  value = google_storage_bucket.runtime_evidence.name
}

output "application_service_account" {
  value = google_service_account.application.email
}

output "database_migrator_service_account" {
  value = google_service_account.database_migrator.email
}

output "cloud_run_application_service" {
  value = one(google_cloud_run_v2_service.application[*].name)
}

output "cloud_run_migration_job" {
  value = one(google_cloud_run_v2_job.database_migrator[*].name)
}
