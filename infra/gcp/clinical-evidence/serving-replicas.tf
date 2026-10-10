# Clinical search on always-on L4 servers, each serving the active build from its own disk
# (work/clinical-search-serving-spec.md). Two replicas in separate us-east1 zones, beside the bucket and the
# database. Google recreates a replica that stops answering, and the internal load balancer sends each search to
# the ready replica with fewer searches in flight. Before launch they run only in test windows.

locals {
  clinical_serving_replicas_enabled = var.clinical_serving_replica_count > 0
  clinical_serving_tag              = "coda-clinical-serving"
  clinical_serving_data_root        = "/var/lib/coda-clinical"
  # One name for the replica group and the close job that drains it, so a renamed group can't read as absent.
  clinical_serving_group_name = "coda-clinical-serving"
  clinical_serving_group      = "projects/${var.project_id}/regions/${var.region}/instanceGroupManagers/${local.clinical_serving_group_name}"
}

resource "google_compute_subnetwork" "serving" {
  project                  = var.project_id
  name                     = "coda-clinical-evidence-serving-us-east1"
  region                   = var.region
  network                  = google_compute_network.clinical_evidence.id
  ip_cidr_range            = var.serving_network_cidr
  private_ip_google_access = true
}

# Google runs the internal load balancer's proxies in their own subnet.
resource "google_compute_subnetwork" "serving_proxy" {
  project       = var.project_id
  name          = "coda-clinical-evidence-serving-proxy-us-east1"
  region        = var.region
  network       = google_compute_network.clinical_evidence.id
  ip_cidr_range = var.serving_proxy_network_cidr
  purpose       = "REGIONAL_MANAGED_PROXY"
  role          = "ACTIVE"
}

resource "google_compute_firewall" "clinical_serving_from_load_balancer" {
  project     = var.project_id
  name        = "coda-clinical-serving-from-load-balancer"
  network     = google_compute_network.clinical_evidence.id
  direction   = "INGRESS"
  target_tags = [local.clinical_serving_tag]
  # Google's health checkers and the load balancer's own proxies.
  source_ranges = ["35.191.0.0/16", "130.211.0.0/22", var.serving_proxy_network_cidr]

  allow {
    protocol = "tcp"
    ports    = ["8080"]
  }
}

resource "google_compute_firewall" "clinical_serving_operator_ssh" {
  project     = var.project_id
  name        = "coda-clinical-serving-operator-ssh"
  network     = google_compute_network.clinical_evidence.id
  direction   = "INGRESS"
  target_tags = [local.clinical_serving_tag]
  # Identity-Aware Proxy only; who may connect is decided by IAM and OS Login.
  source_ranges = ["35.235.240.0/20"]

  allow {
    protocol = "tcp"
    ports    = ["22"]
  }
}

resource "google_artifact_registry_repository_iam_member" "clinical_serving_image" {
  project    = var.project_id
  location   = google_artifact_registry_repository.clinical_evidence.location
  repository = google_artifact_registry_repository.clinical_evidence.name
  role       = "roles/artifactregistry.reader"
  member     = "serviceAccount:${google_service_account.clinical_retrieval.email}"
}

resource "google_project_iam_member" "clinical_serving_logs" {
  project = var.project_id
  role    = "roles/logging.logWriter"
  member  = "serviceAccount:${google_service_account.clinical_retrieval.email}"
}

resource "google_compute_instance_template" "clinical_serving" {
  count = local.clinical_serving_replicas_enabled ? 1 : 0

  project      = var.project_id
  name_prefix  = "coda-clinical-serving-"
  region       = var.region
  machine_type = "g2-standard-8"
  tags         = [local.clinical_serving_tag]
  labels       = { owner = "clinical-evidence" }

  # A G2 server carries its L4; a GPU server can't migrate live, so maintenance restarts it.
  scheduling {
    on_host_maintenance = "TERMINATE"
    automatic_restart   = true
  }

  disk {
    source_image = var.clinical_serving_boot_image
    disk_type    = "pd-balanced"
    disk_size_gb = 100
    boot         = true
    auto_delete  = true
  }

  # The table copy and runtime assets. Recovery copies them again; redundancy is the other replica.
  disk {
    type         = "SCRATCH"
    disk_type    = "local-ssd"
    interface    = "NVME"
    disk_size_gb = 375
  }

  network_interface {
    subnetwork = google_compute_subnetwork.serving.id
  }

  service_account {
    email  = google_service_account.clinical_retrieval.email
    scopes = ["cloud-platform"]
  }

  shielded_instance_config {
    enable_secure_boot          = true
    enable_vtpm                 = true
    enable_integrity_monitoring = true
  }

  metadata = {
    enable-oslogin = "TRUE"
    startup-script = templatefile("${path.module}/clinical-serving-startup.sh.tftpl", {
      image     = var.gpu_clinical_retrieval_image
      data_root = local.clinical_serving_data_root
      environment = merge({ for variable in local.clinical_evidence_server_environment : variable.name => variable.value }, {
        CODA_CLINICAL_RETRIEVAL_CACHE_ROOT = "${local.clinical_serving_data_root}/runtime"
        CODA_CLINICAL_TABLE_MIRROR_ROOT    = "${local.clinical_serving_data_root}/table"
        CODA_CLINICAL_RETRIEVAL_WORKERS    = tostring(var.clinical_evidence_worker_count)
        PORT                               = "8080"
      })
    })
  }

  lifecycle {
    create_before_destroy = true
  }

  depends_on = [
    google_artifact_registry_repository_iam_member.clinical_serving_image,
    google_project_iam_member.clinical_serving_logs,
    google_project_iam_member.clinical_retrieval_cloud_sql,
    google_storage_bucket_iam_member.clinical_retrieval_build_read,
  ]
}

# Routing follows readiness: a replica receives searches only while it serves the active build.
resource "google_compute_region_health_check" "clinical_serving" {
  project             = var.project_id
  name                = "coda-clinical-serving-ready"
  region              = var.region
  check_interval_sec  = 10
  timeout_sec         = 5
  healthy_threshold   = 1
  unhealthy_threshold = 3

  http_health_check {
    port         = 8080
    request_path = "/ready"
  }
}

# The group follows liveness: passing only once this replica has served the active build, then while it answers.
# Google counts a new replica as available once this has passed, so an update retires an old replica only after
# its replacement serves. A database blip makes every replica unready at once but never looks like a dead one;
# recreating them all would turn seconds without search into the time it takes to copy the table again.
resource "google_compute_region_health_check" "clinical_serving_live" {
  project             = var.project_id
  name                = "coda-clinical-serving-live"
  region              = var.region
  check_interval_sec  = 10
  timeout_sec         = 5
  healthy_threshold   = 1
  unhealthy_threshold = 6

  http_health_check {
    port         = 8080
    request_path = "/live"
  }
}

# The entry point's one fixed destination, kept while test windows remove the replicas.
resource "google_compute_address" "clinical_serving" {
  project      = var.project_id
  name         = "coda-clinical-serving"
  region       = var.region
  subnetwork   = google_compute_subnetwork.serving.id
  address_type = "INTERNAL"
  purpose      = "SHARED_LOADBALANCER_VIP"
}

resource "google_compute_region_instance_group_manager" "clinical_serving" {
  count = local.clinical_serving_replicas_enabled ? 1 : 0

  project                          = var.project_id
  name                             = local.clinical_serving_group_name
  region                           = var.region
  base_instance_name               = "coda-clinical-serving"
  distribution_policy_zones        = var.clinical_serving_zones
  distribution_policy_target_shape = "EVEN"
  target_size                      = var.clinical_serving_replica_count

  version {
    instance_template = google_compute_instance_template.clinical_serving[0].self_link_unique
  }

  named_port {
    name = "http"
    port = 8080
  }

  # A new replica copies the table and loads its workers before it is first live.
  auto_healing_policies {
    health_check      = google_compute_region_health_check.clinical_serving_live.id
    initial_delay_sec = 3600
  }

  # Replacements come up beside the serving replicas, never instead of them, and count only once they serve.
  update_policy {
    type                         = "PROACTIVE"
    minimal_action               = "REPLACE"
    instance_redistribution_type = "NONE"
    max_surge_fixed              = length(var.clinical_serving_zones)
    max_unavailable_fixed        = 0
  }
}

resource "google_compute_region_backend_service" "clinical_serving" {
  count = local.clinical_serving_replicas_enabled ? 1 : 0

  project               = var.project_id
  name                  = "coda-clinical-serving"
  region                = var.region
  load_balancing_scheme = "INTERNAL_MANAGED"
  protocol              = "HTTP"
  port_name             = "http"
  timeout_sec           = 60
  health_checks         = [google_compute_region_health_check.clinical_serving.id]
  # A search holds a replica for seconds; the next one goes where fewer are in flight.
  locality_lb_policy = "LEAST_REQUEST"

  backend {
    group           = google_compute_region_instance_group_manager.clinical_serving[0].instance_group
    balancing_mode  = "UTILIZATION"
    capacity_scaler = 1
  }
}

resource "google_compute_region_url_map" "clinical_serving" {
  count = local.clinical_serving_replicas_enabled ? 1 : 0

  project         = var.project_id
  name            = "coda-clinical-serving"
  region          = var.region
  default_service = google_compute_region_backend_service.clinical_serving[0].id
}

resource "google_compute_region_target_http_proxy" "clinical_serving" {
  count = local.clinical_serving_replicas_enabled ? 1 : 0

  project = var.project_id
  name    = "coda-clinical-serving"
  region  = var.region
  url_map = google_compute_region_url_map.clinical_serving[0].id
}

resource "google_compute_forwarding_rule" "clinical_serving" {
  count = local.clinical_serving_replicas_enabled ? 1 : 0

  project               = var.project_id
  name                  = "coda-clinical-serving"
  region                = var.region
  load_balancing_scheme = "INTERNAL_MANAGED"
  ip_protocol           = "TCP"
  port_range            = "8080"
  network               = google_compute_network.clinical_evidence.id
  subnetwork            = google_compute_subnetwork.serving.id
  ip_address            = google_compute_address.clinical_serving.address
  target                = google_compute_region_target_http_proxy.clinical_serving[0].id
  # The authenticated entry point may sit in another region of the same network.
  allow_global_access = true

  depends_on = [google_compute_subnetwork.serving_proxy]
}
