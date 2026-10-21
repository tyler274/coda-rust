output "namespace" {
  value = kubernetes_namespace.observability.metadata[0].name
}

output "collector_otlp_http_endpoint" {
  description = "OTEL_EXPORTER_OTLP_ENDPOINT for in-cluster services."
  value       = local.collector_otlp_http
}

output "collector_otlp_grpc_endpoint" {
  value = local.collector_otlp_grpc
}

output "service_telemetry_env" {
  description = "Environment every instrumented service sets (physician-runtime binds the same values)."
  value       = local.service_telemetry_env
}

output "langfuse_web_url" {
  description = "In-cluster Langfuse URL. Humans use kubectl port-forward svc/langfuse-web 3000 until the IAP gateway is on."
  value       = local.langfuse_web_url
}

output "langfuse_cloud_sql_connection_name" {
  value = google_sql_database_instance.langfuse.connection_name
}

output "langfuse_blob_bucket" {
  value = google_storage_bucket.langfuse.name
}

output "langfuse_hmac_service_account" {
  description = "Mint the S3-interop HMAC key for this account and store it in Doppler as LANGFUSE_S3_ACCESS_KEY_ID / LANGFUSE_S3_SECRET_ACCESS_KEY."
  value       = google_service_account.langfuse_blobs.email
}

output "datadog_monitor_ids" {
  value = {
    site_5xx_rate          = datadog_monitor.site_5xx_rate.id
    chat_generation_errors = datadog_monitor.chat_generation_errors.id
    site_p95_latency       = datadog_monitor.site_p95_latency.id
  }
}
