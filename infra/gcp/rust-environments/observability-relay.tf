# --- Observability relay (observability_relay_image) ------------------------
#
# public-site serves Grafana under /grafana/ and Langfuse under /langfuse/ by
# proxying to this service (crates/observability-relay). Cloud Run IAP admits
# the engineering group and observability_proxy_invokers, the public-site
# service account. The relay leaves through Direct VPC egress on its own
# subnet, and the two internal passthrough load balancers that
# infra/gcp/observability creates admit only that subnet. That keeps the
# physician and Duet services, which share rust-run, off Grafana and Langfuse.
#
# It reads no secrets and has no database, Doppler token or sidecar: one
# unnamed container, which staging-release updates like physician-worker
# (crates/workspace-tools/src/staging_release).
locals {
  observability_relay = var.observability_relay_image != null && local.observability_cluster ? toset(["relay"]) : toset([])
}

# Cloud Run Direct VPC egress needs at least a /26.
resource "google_compute_subnetwork" "relay" {
  for_each                 = local.observability_relay
  project                  = google_project.environment.project_id
  name                     = "rust-relay"
  region                   = var.region
  network                  = google_compute_network.private.id
  ip_cidr_range            = "10.80.1.0/26"
  private_ip_google_access = true
}

# The two load balancers take these addresses (loadBalancerIP), in the node
# subnet next to the otel gateway's.
resource "google_compute_address" "observability_web" {
  for_each     = { for app in ["grafana", "langfuse"] : app => app if length(local.observability_relay) > 0 }
  project      = google_project.environment.project_id
  name         = "${each.key}-internal"
  region       = var.region
  address_type = "INTERNAL"
  subnetwork   = google_compute_subnetwork.observability[0].id
}

resource "google_service_account" "observability_relay" {
  for_each     = local.observability_relay
  project      = google_project.environment.project_id
  account_id   = "observability-relay"
  display_name = "observability relay runtime (${var.environment})"
  description  = "observability-relay Cloud Run identity. No project roles, secrets, SQL or product access."
  depends_on   = [google_project_service.api]
}

resource "google_cloud_run_v2_service" "observability_relay" {
  for_each             = local.observability_relay
  project              = google_project.environment.project_id
  name                 = "observability-relay"
  location             = var.region
  description          = "Staging hop from public-site to the private Grafana and Langfuse load balancers."
  ingress              = "INGRESS_TRAFFIC_ALL"
  iap_enabled          = true
  invoker_iam_disabled = false
  # Stateless; turning the gate off must remove it in one reviewed plan.
  deletion_protection = false
  template {
    service_account                  = google_service_account.observability_relay[each.key].email
    execution_environment            = "EXECUTION_ENVIRONMENT_GEN2"
    timeout                          = "300s"
    max_instance_request_concurrency = 80
    scaling {
      min_instance_count = 0
      max_instance_count = var.max_instances
    }
    vpc_access {
      egress = "PRIVATE_RANGES_ONLY"
      network_interfaces {
        network    = google_compute_network.private.name
        subnetwork = google_compute_subnetwork.relay[each.key].name
      }
    }
    containers {
      image = var.observability_relay_image
      ports { container_port = 8080 }
      resources {
        limits            = { cpu = "1", memory = "512Mi" }
        cpu_idle          = true
        startup_cpu_boost = false
      }
      env {
        name  = "RELAY_IAP_AUDIENCE"
        value = "/projects/${google_project.environment.number}/locations/${var.region}/services/observability-relay"
      }
      env {
        name  = "GRAFANA_UPSTREAM"
        value = "http://${google_compute_address.observability_web["grafana"].address}:3000"
      }
      env {
        name  = "LANGFUSE_UPSTREAM"
        value = "http://${google_compute_address.observability_web["langfuse"].address}:3000"
      }
      # The relay has no health route; a TCP probe proves it is listening.
      startup_probe {
        initial_delay_seconds = 0
        period_seconds        = 2
        timeout_seconds       = 1
        failure_threshold     = 15
        tcp_socket { port = 8080 }
      }
    }
  }
  lifecycle {
    # Same release-owned fields as the app services (workloads.tf).
    ignore_changes = [template[0].containers[0].image, template[0].revision, traffic, client, client_version]
  }
  depends_on = [google_project_service_identity.iap]
}

resource "google_cloud_run_v2_service_iam_binding" "observability_relay_invoker" {
  for_each = local.observability_relay
  project  = google_project.environment.project_id
  location = var.region
  name     = google_cloud_run_v2_service.observability_relay[each.key].name
  role     = "roles/run.invoker"
  members  = ["serviceAccount:${google_project_service_identity.iap.email}"]
}

# Authoritative, like the app services' binding in workloads.tf.
resource "google_iap_web_cloud_run_service_iam_binding" "observability_relay" {
  for_each               = local.observability_relay
  project                = google_project.environment.project_id
  location               = var.region
  cloud_run_service_name = google_cloud_run_v2_service.observability_relay[each.key].name
  role                   = "roles/iap.httpsResourceAccessor"
  members                = setunion(toset(["group:${var.iap_group}"]), var.observability_proxy_invokers)
}

# Release updates the relay image like the app services (deploy-staging).
resource "google_cloud_run_v2_service_iam_member" "observability_relay_release" {
  for_each = local.observability_relay
  project  = google_project.environment.project_id
  location = var.region
  name     = google_cloud_run_v2_service.observability_relay[each.key].name
  role     = google_project_iam_custom_role.release_service.name
  member   = "serviceAccount:${google_service_account.deployer.email}"
}

resource "google_service_account_iam_member" "observability_relay_release_act_as" {
  for_each           = local.observability_relay
  service_account_id = google_service_account.observability_relay[each.key].name
  role               = "roles/iam.serviceAccountUser"
  member             = "serviceAccount:${google_service_account.deployer.email}"
}

# Langfuse web runs a base-path build pushed to the apps repository
# (.github/workflows/langfuse-web-image.yml). Autopilot nodes pull as the
# project's default compute account, so it may read this one repository.
resource "google_artifact_registry_repository_iam_member" "observability_nodes" {
  for_each   = local.observability_relay
  project    = google_project.environment.project_id
  location   = var.region
  repository = google_artifact_registry_repository.apps.name
  role       = "roles/artifactregistry.reader"
  member     = "serviceAccount:${google_project.environment.number}-compute@developer.gserviceaccount.com"
}

# Inputs for infra/gcp/observability (observability_web) and public-site
# (observability_relay_url).
output "observability_relay" {
  description = "The relay's URL for public-site, the reserved internal addresses of the Grafana and Langfuse load balancers, and the only source range they admit. Null while the relay is off."
  value = length(local.observability_relay) > 0 ? {
    url           = google_cloud_run_v2_service.observability_relay["relay"].uri
    grafana_ip    = google_compute_address.observability_web["grafana"].address
    langfuse_ip   = google_compute_address.observability_web["langfuse"].address
    source_ranges = [google_compute_subnetwork.relay["relay"].ip_cidr_range]
  } : null
}
