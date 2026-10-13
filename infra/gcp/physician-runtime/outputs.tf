output "cluster_name" {
  value = google_container_cluster.physician.name
}

output "cluster_endpoint" {
  value = google_container_cluster.physician.private_cluster_config[0].private_endpoint
}

output "cloud_sql_connection_name" {
  value = google_sql_database_instance.physician.connection_name
}

output "physician_database" {
  value = google_sql_database.physician.name
}

output "physician_site_service_account" {
  value = google_service_account.physician_site.email
}

output "database_migrator_service_account" {
  value = google_service_account.database_migrator.email
}

output "artifact_registry_repository" {
  value = google_artifact_registry_repository.physician.name
}
