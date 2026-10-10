locals {
  workload_services = toset(concat([
    "artifactregistry.googleapis.com",
    "cloudbuild.googleapis.com",
    "cloudresourcemanager.googleapis.com",
    "compute.googleapis.com",
    "iam.googleapis.com",
    "iamcredentials.googleapis.com",
    "run.googleapis.com",
    "serviceusage.googleapis.com",
    "servicenetworking.googleapis.com",
    "sqladmin.googleapis.com",
    "storage.googleapis.com",
    ], var.clinical_source_acquisition_enabled ? [
    "batch.googleapis.com",
    "cloudscheduler.googleapis.com",
    "logging.googleapis.com",
    "monitoring.googleapis.com",
    "secretmanager.googleapis.com",
    "workflowexecutions.googleapis.com",
    "workflows.googleapis.com",
  ] : []))
  clinical_database_name = "coda_clinical_corpus_production"
  clinical_operations_notification_channels = setunion(
    var.clinical_operations_notification_channels,
    toset(google_monitoring_notification_channel.clinical_operations_email[*].name),
  )
  clinical_refresh_enabled = (
    var.clinical_source_acquisition_enabled &&
    var.clinical_retrieval_build_enabled
  )
}
