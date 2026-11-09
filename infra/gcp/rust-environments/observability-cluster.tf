# --- Observability cluster (observability_cluster_enabled) ------------------
#
# A private GKE Autopilot cluster on rust-private that hosts the shared
# OpenTelemetry gateway. This root owns the cluster, its subnet and the
# gateway's reserved internal address; infra/gcp/observability deploys the
# gateway (and Langfuse) into it through the outputs below and never manages
# the cluster or the network.
#
# Address plan on rust-private (no range may overlap another):
#   10.80.0.0/24    rust-run, Cloud Run Direct VPC egress (database.tf)
#   10.80.1.0/26    rust-relay, the observability relay's Direct VPC egress
#                   (observability-relay.tf)
#   10.81.0.0/16    rust-sql-private, Private Service Access for SQL and Redis
#   10.82.0.0/22    rust-observability nodes and the gateway's internal LB IP
#   10.84.0.0/14    observability-pods (10.84.0.0 - 10.87.255.255)
#   10.88.0.0/20    observability-services
#   172.16.80.0/28  GKE control plane
locals {
  observability_cluster = var.observability_cluster_enabled
  # The google_project precondition refuses observability_enabled without the
  # cluster; the conjunction only keeps evaluation safe until it reports.
  observability         = var.observability_enabled && var.observability_cluster_enabled
  otel_gateway_audience = coalesce(var.otel_gateway_audience, "coda-otel-gateway-${var.environment}")
  otel_gateway_endpoint = local.observability_cluster ? "http://${google_compute_address.otel_gateway[0].address}:4318" : null
  # The gateway admits traffic only from Cloud Run's Direct VPC egress subnet.
  otel_gateway_source_ranges = [google_compute_subnetwork.run.ip_cidr_range]
  # Datadog's env tag, matching infra/gcp/observability's monitors and
  # dashboards (env:staging), not the short root key (stg).
  dd_env = { dev = "dev", stg = "staging", prd = "production" }[var.environment]
  # Product-container settings that only infrastructure sets.
  observability_owned_env = ["DD_ENV", "DD_VERSION", "OTEL_EXPORTER_OTLP_ENDPOINT", "CODA_LOG_FORMAT", "CODA_TELEMETRY_CONTENT", "CODA_TELEMETRY_BACKEND"]
  # Added to the product container only while the sidecar runs. The same
  # values as infra/gcp/observability's service_telemetry_env output;
  # telemetry_backend and telemetry_content must match that root's choices.
  observability_product_env = local.observability ? {
    OTEL_EXPORTER_OTLP_ENDPOINT = "http://localhost:4318"
    CODA_LOG_FORMAT             = "json"
    CODA_TELEMETRY_CONTENT      = var.telemetry_content
    CODA_TELEMETRY_BACKEND      = var.telemetry_backend
  } : {}
  # Index 1 of every observed service. Never a dependency of the product
  # container (index 0, release-owned image), so the product starts on its own.
  otel_sidecar = local.observability ? [{
    name   = "otel-collector"
    image  = var.otel_sidecar_image
    args   = ["--config=env:OTEL_SIDECAR_CONFIG"]
    config = file("${path.module}/../../../dev/observability/sidecar.yaml")
  }] : []
}

resource "google_compute_subnetwork" "observability" {
  count                    = local.observability_cluster ? 1 : 0
  project                  = google_project.environment.project_id
  name                     = "rust-observability"
  region                   = var.region
  network                  = google_compute_network.private.id
  ip_cidr_range            = "10.82.0.0/22"
  private_ip_google_access = true
  secondary_ip_range {
    range_name    = "observability-pods"
    ip_cidr_range = "10.84.0.0/14"
  }
  secondary_ip_range {
    range_name    = "observability-services"
    ip_cidr_range = "10.88.0.0/20"
  }
}

resource "google_container_cluster" "observability" {
  count               = local.observability_cluster ? 1 : 0
  project             = google_project.environment.project_id
  name                = "rust-observability"
  location            = var.region
  enable_autopilot    = true
  deletion_protection = true

  network    = google_compute_network.private.id
  subnetwork = google_compute_subnetwork.observability[0].id

  ip_allocation_policy {
    stack_type                    = "IPV4"
    cluster_secondary_range_name  = google_compute_subnetwork.observability[0].secondary_ip_range[0].range_name
    services_secondary_range_name = google_compute_subnetwork.observability[0].secondary_ip_range[1].range_name
  }

  # Nodes have no public IPs and the IP endpoint is private only. Operators
  # (and the observability root) reach the control plane through the
  # DNS-based endpoint, which IAM governs (container.clusters.connect), so no
  # bastion or authorized public network is needed.
  private_cluster_config {
    enable_private_nodes    = true
    enable_private_endpoint = true
    master_ipv4_cidr_block  = "172.16.80.0/28"
  }

  # GKE refuses a private IP endpoint unless authorized networks are on. None
  # is listed, so nothing outside the VPC reaches the IP endpoint; the DNS
  # endpoint is governed by IAM, not by this list.
  master_authorized_networks_config {}

  control_plane_endpoints_config {
    dns_endpoint_config {
      allow_external_traffic = true
    }
    ip_endpoints_config {
      enabled = true
    }
  }

  workload_identity_config {
    workload_pool = "${local.project_id}.svc.id.goog"
  }

  release_channel {
    channel = "REGULAR"
  }

  depends_on = [google_project_service.api]
}

# Private nodes have no external IPs. The gateway exports to Datadog's public
# intake and the stack pulls public images (Docker Hub), so the observability
# subnet (nodes and pods) gets its own Cloud NAT with Google-allocated IPs.
# rust-run keeps its separate, static-IP NAT (smtp-relay.tf); a subnet has at
# most one NAT gateway.
resource "google_compute_router" "observability" {
  count   = local.observability_cluster ? 1 : 0
  project = google_project.environment.project_id
  name    = "rust-observability-egress"
  region  = var.region
  network = google_compute_network.private.id
}

resource "google_compute_router_nat" "observability" {
  count                              = local.observability_cluster ? 1 : 0
  project                            = google_project.environment.project_id
  name                               = "rust-observability-egress"
  region                             = var.region
  router                             = google_compute_router.observability[0].name
  nat_ip_allocate_option             = "AUTO_ONLY"
  source_subnetwork_ip_ranges_to_nat = "LIST_OF_SUBNETWORKS"
  subnetwork {
    name                    = google_compute_subnetwork.observability[0].id
    source_ip_ranges_to_nat = ["ALL_IP_RANGES"]
  }
}

# The collector's internal passthrough load balancer takes this address
# (loadBalancerIP on the gateway Service). It lives in the node subnet, which
# GKE uses for internal load balancers by default.
resource "google_compute_address" "otel_gateway" {
  count        = local.observability_cluster ? 1 : 0
  project      = google_project.environment.project_id
  name         = "otel-gateway"
  region       = var.region
  address_type = "INTERNAL"
  subnetwork   = google_compute_subnetwork.observability[0].id
}

# Inputs for infra/gcp/observability (cluster_*, otel_gateway_*). Null while
# the cluster gate is off.
output "observability_cluster" {
  description = "The private observability Autopilot cluster: reach its control plane through dns_endpoint (IAM-governed, no bastion)."
  value = local.observability_cluster ? {
    name         = google_container_cluster.observability[0].name
    location     = google_container_cluster.observability[0].location
    project      = google_container_cluster.observability[0].project
    network      = google_compute_network.private.name
    subnetwork   = google_compute_subnetwork.observability[0].name
    dns_endpoint = try(google_container_cluster.observability[0].control_plane_endpoints_config[0].dns_endpoint_config[0].endpoint, null)
  } : null
}

output "otel_gateway" {
  description = "The OTLP/HTTP gateway the Cloud Run sidecars export to: the reserved internal LB address, its endpoint, the ID-token audience, the source ranges for loadBalancerSourceRanges and the runtime identities the gateway admits."
  value = local.observability_cluster ? {
    ip              = google_compute_address.otel_gateway[0].address
    endpoint        = local.otel_gateway_endpoint
    audience        = local.otel_gateway_audience
    source_ranges   = local.otel_gateway_source_ranges
    allowed_callers = sort(distinct(concat([for app in local.apps : google_service_account.app["${app}-runtime"].email], [for worker in google_service_account.physician_worker : worker.email])))
  } : null
}

output "otel_gateway_endpoint" {
  description = "OTLP/HTTP base the sidecars export to (http://<gateway ip>:4318). Null while the cluster gate is off."
  value       = local.otel_gateway_endpoint
}
