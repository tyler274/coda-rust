resource "google_container_cluster" "physician" {
  project             = var.project_id
  name                = "coda-physician-runtime"
  location            = var.region
  enable_autopilot    = true
  deletion_protection = true

  network    = google_compute_network.physician.id
  subnetwork = google_compute_subnetwork.physician.id

  private_cluster_config {
    enable_private_nodes    = true
    enable_private_endpoint = false
    master_ipv4_cidr_block  = "172.16.0.0/28"
  }

  ip_allocation_policy {
    # Autopilot carves pod and service ranges out of the named subnet.
    stack_type = "IPV4"
  }

  workload_identity_config {
    workload_pool = "${data.google_project.physician_runtime.project_id}.svc.id.goog"
  }

  release_channel {
    channel = "REGULAR"
  }

  authenticator_groups_config {
    security_group = "gke-security-groups@${data.google_project.physician_runtime.project_id}.iam.gserviceaccount.com"
  }

  depends_on = [google_project_service.workload]
}

resource "google_project_iam_member" "physician_site_workload_identity" {
  project = var.project_id
  role    = "roles/iam.workloadIdentityUser"
  member  = "principalSet://iam.googleapis.com/projects/${data.google_project.physician_runtime.number}/locations/${var.region}/clusters/${google_container_cluster.physician.name}/namespaces/physician/serviceAccounts/physician-site"

  depends_on = [google_container_cluster.physician]
}

resource "google_project_iam_member" "database_migrator_workload_identity" {
  project = var.project_id
  role    = "roles/iam.workloadIdentityUser"
  member  = "principalSet://iam.googleapis.com/projects/${data.google_project.physician_runtime.number}/locations/${var.region}/clusters/${google_container_cluster.physician.name}/namespaces/physician/serviceAccounts/physician-migrator"

  depends_on = [google_container_cluster.physician]
}
