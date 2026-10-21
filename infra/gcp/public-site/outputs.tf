output "service_name" {
  value = google_cloud_run_v2_service.site.name
}

output "load_balancer_ip" {
  description = "Reserved address only; no DNS or production traffic change is performed by this root."
  value       = google_compute_global_address.site.address
}

output "cdn_enabled" {
  value = google_compute_backend_service.site.enable_cdn
}
