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

output "staging_domain_records" {
  description = "DNS records required by the staging domain mapping; no DNS records are changed by this root."
  value       = try(google_cloud_run_domain_mapping.staging[0].status[0].resource_records, [])
}

output "load_balancer_ip" {
  description = "Production reserved address only; null for staging. No DNS or traffic change is performed by this root."
  value       = local.staging ? null : google_compute_global_address.site[0].address
}

output "certificate_dns_records" {
  description = "Production only: the CNAME records that authorize the managed certificate. The operator adds them in DNS before cutover; this root changes no DNS."
  value = [
    for record in flatten([for authorization in values(google_certificate_manager_dns_authorization.site) : authorization.dns_resource_record]) :
    { name = record.name, type = record.type, data = record.data }
  ]
}

output "cdn_enabled" {
  value = local.staging ? false : google_compute_backend_service.site[0].enable_cdn
}
