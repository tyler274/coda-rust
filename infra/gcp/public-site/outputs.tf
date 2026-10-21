output "service_name" {
  value = google_cloud_run_v2_service.site.name
}

output "staging_url" {
  description = "Native-IAP-protected staging run.app URL; null for production, whose default URI is disabled."
  value       = local.staging ? google_cloud_run_v2_service.site.uri : null
}

output "iap_audience" {
  description = "Expected signed IAP assertion audience for staging, not an OAuth client ID or smoke-token target audience."
  value       = local.staging ? local.iap_audience : null
}

output "load_balancer_ip" {
  description = "Production reserved address only; null for staging. No DNS or traffic change is performed by this root."
  value       = local.staging ? null : google_compute_global_address.site[0].address
}

output "cdn_enabled" {
  value = local.staging ? false : google_compute_backend_service.site[0].enable_cdn
}
