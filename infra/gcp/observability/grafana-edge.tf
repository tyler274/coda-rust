# Optional browser edge for Grafana (grafana_edge): one external Gateway on
# a reserved global address, a Google-managed certificate authorized by DNS,
# and IAP on the Grafana backend. IAP decides who reaches Grafana; Grafana
# then signs the person in from the signed IAP assertion (grafana.tf), so a
# request that bypasses IAP carries no identity Grafana accepts. Only the
# Grafana Service is routed; Tempo, Loki, Prometheus, Langfuse and the
# collector stay ClusterIP or internal.
locals {
  grafana_edge         = local.grafana_backend ? var.grafana_edge : null
  grafana_edge_enabled = local.grafana_edge != null
  grafana_edge_name    = "grafana-edge"
  grafana_edge_class   = "gke-l7-global-external-managed"
  grafana_edge_host    = try(local.grafana_edge.hostname, null)
  grafana_edge_url     = local.grafana_edge_enabled ? "https://${local.grafana_edge_host}" : null

  # The assertion audience IAP signs for a global backend service. Known only
  # after the operator verifies the GKE-created backend; until then Grafana
  # keeps JWT sign-in off and nobody is granted through IAP.
  grafana_edge_backend_id = try(local.grafana_edge.backend_service_id, null)
  grafana_iap_audience    = local.grafana_edge_backend_id == null ? null : "/projects/${data.google_project.runtime.number}/global/backendServices/${local.grafana_edge_backend_id}"

  # Rendered only by helm_release.grafana_edge, which exists only with the edge.
  grafana_edge_manifests = [for manifest in [
    {
      apiVersion = "gateway.networking.k8s.io/v1"
      kind       = "Gateway"
      metadata = {
        name        = local.grafana_edge_name
        namespace   = local.namespace
        annotations = { "networking.gke.io/certmap" = one(google_certificate_manager_certificate_map.grafana_edge[*].name) }
      }
      spec = {
        gatewayClassName = local.grafana_edge_class
        addresses        = [{ type = "NamedAddress", value = one(google_compute_global_address.grafana_edge[*].name) }]
        listeners = [{
          name     = "https"
          protocol = "HTTPS"
          port     = 443
          hostname = local.grafana_edge_host
        }]
      }
    },
    {
      apiVersion = "gateway.networking.k8s.io/v1"
      kind       = "HTTPRoute"
      metadata   = { name = "grafana", namespace = local.namespace }
      spec = {
        parentRefs = [{ name = local.grafana_edge_name }]
        hostnames  = [local.grafana_edge_host]
        rules      = [{ backendRefs = [{ name = "grafana", port = 3000 }] }]
      }
    },
    {
      apiVersion = "networking.gke.io/v1"
      kind       = "GCPBackendPolicy"
      metadata   = { name = "grafana-iap", namespace = local.namespace }
      spec = {
        # No clientID or secret: IAP uses Google's managed OAuth client,
        # which admits only people in the organization. GKE cannot later
        # switch a backend from a custom client back to the managed one.
        default   = { iap = { enabled = true } }
        targetRef = { group = "", kind = "Service", name = "grafana" }
      }
    },
    {
      # Grafana answers / with a redirect to /login, which the default
      # health check would count as unhealthy.
      apiVersion = "networking.gke.io/v1"
      kind       = "HealthCheckPolicy"
      metadata   = { name = "grafana-health", namespace = local.namespace }
      spec = {
        default = {
          config = {
            type            = "HTTP"
            httpHealthCheck = { port = 3000, requestPath = "/api/health" }
          }
        }
        targetRef = { group = "", kind = "Service", name = "grafana" }
      }
    },
  ] : manifest if local.grafana_edge_enabled]
}

resource "google_compute_global_address" "grafana_edge" {
  count = local.grafana_edge_enabled ? 1 : 0

  project = var.project_id
  name    = local.grafana_edge_name

  depends_on = [google_project_service.workload]
}

# Authorized by a DNS record rather than by serving traffic, so the
# certificate is active before the hostname points at the edge.
resource "google_certificate_manager_dns_authorization" "grafana_edge" {
  count = local.grafana_edge_enabled ? 1 : 0

  project = var.project_id
  name    = "${local.grafana_edge_name}-${replace(local.grafana_edge.hostname, ".", "-")}"
  domain  = local.grafana_edge.hostname

  depends_on = [google_project_service.workload]
}

resource "google_certificate_manager_certificate" "grafana_edge" {
  count = local.grafana_edge_enabled ? 1 : 0

  project = var.project_id
  name    = local.grafana_edge_name
  managed {
    domains            = [local.grafana_edge.hostname]
    dns_authorizations = [google_certificate_manager_dns_authorization.grafana_edge[0].id]
  }
}

resource "google_certificate_manager_certificate_map" "grafana_edge" {
  count = local.grafana_edge_enabled ? 1 : 0

  project = var.project_id
  name    = local.grafana_edge_name

  depends_on = [google_project_service.workload]
}

resource "google_certificate_manager_certificate_map_entry" "grafana_edge" {
  count = local.grafana_edge_enabled ? 1 : 0

  project      = var.project_id
  name         = "${local.grafana_edge_name}-${replace(local.grafana_edge.hostname, ".", "-")}"
  map          = google_certificate_manager_certificate_map.grafana_edge[0].name
  certificates = [google_certificate_manager_certificate.grafana_edge[0].id]
  hostname     = local.grafana_edge.hostname
}

resource "helm_release" "grafana_edge" {
  count = local.grafana_edge_enabled ? 1 : 0

  name      = "grafana-edge"
  chart     = local.manifests_chart
  namespace = kubernetes_namespace.observability.metadata[0].name
  atomic    = true

  values = [yamlencode({ manifests = local.grafana_edge_manifests })]

  depends_on = [
    kubernetes_service_v1.grafana_stack,
    google_certificate_manager_certificate_map_entry.grafana_edge,
  ]
}

# GKE owns the backend identity. Bootstrap the edge without a grant, then
# bind only the backend the operator verified for the Grafana route.
resource "google_iap_web_backend_service_iam_member" "grafana_edge" {
  for_each = local.grafana_edge_enabled && local.grafana_edge_backend_id != null ? toset(local.grafana_edge.members) : toset([])

  project             = var.project_id
  web_backend_service = local.grafana_edge.backend_service
  role                = "roles/iap.httpsResourceAccessor"
  member              = each.value
}
