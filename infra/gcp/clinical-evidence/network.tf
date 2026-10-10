resource "google_compute_network" "clinical_evidence" {
  project                 = var.project_id
  name                    = "coda-clinical-evidence"
  auto_create_subnetworks = false
  routing_mode            = "REGIONAL"

  depends_on = [google_project_service.workload]
}

resource "google_compute_subnetwork" "operations" {
  project                  = var.project_id
  name                     = "coda-clinical-evidence-operations-us-east1"
  region                   = var.region
  network                  = google_compute_network.clinical_evidence.id
  ip_cidr_range            = var.network_cidr
  private_ip_google_access = true
}

resource "google_compute_subnetwork" "retrieval" {
  project                  = var.project_id
  name                     = "coda-clinical-evidence-retrieval-us-east4"
  region                   = var.gpu_region
  network                  = google_compute_network.clinical_evidence.id
  ip_cidr_range            = var.gpu_network_cidr
  private_ip_google_access = true
}

resource "google_compute_global_address" "private_services" {
  project       = var.project_id
  name          = "coda-clinical-evidence-private-services"
  purpose       = "VPC_PEERING"
  address_type  = "INTERNAL"
  prefix_length = 24
  network       = google_compute_network.clinical_evidence.id
}

resource "google_service_networking_connection" "private_services" {
  network                 = google_compute_network.clinical_evidence.id
  service                 = "servicenetworking.googleapis.com"
  reserved_peering_ranges = [google_compute_global_address.private_services.name]
}
