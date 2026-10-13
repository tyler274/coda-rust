resource "google_compute_network" "physician" {
  project                 = var.project_id
  name                    = "coda-physician-runtime"
  auto_create_subnetworks = false
  routing_mode            = "REGIONAL"

  depends_on = [google_project_service.workload]
}

resource "google_compute_subnetwork" "physician" {
  project                  = var.project_id
  name                     = "coda-physician-runtime-${var.region}"
  region                   = var.region
  network                  = google_compute_network.physician.id
  ip_cidr_range            = var.network_cidr
  private_ip_google_access = true
}

# Cloud SQL reaches the network only through Private Service Access.
resource "google_compute_global_address" "private_services" {
  project       = var.project_id
  name          = "coda-physician-private-services"
  purpose       = "VPC_PEERING"
  address_type  = "INTERNAL"
  prefix_length = var.private_services_prefix_length
  network       = google_compute_network.physician.id
}

resource "google_service_networking_connection" "private_services" {
  network                 = google_compute_network.physician.id
  service                 = "servicenetworking.googleapis.com"
  reserved_peering_ranges = [google_compute_global_address.private_services.name]

  depends_on = [google_project_service.workload]
}
