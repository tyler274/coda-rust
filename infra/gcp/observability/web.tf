# People reach Grafana and Langfuse only through public-site's /grafana/,
# /langfuse/ and /langfuse-sso/ mounts (staging_proxy). public-site calls the
# IAP-protected observability relay in infra/gcp/rust-environments, and the
# relay's Direct VPC egress reaches these internal passthrough load
# balancers. They admit only the relay's own subnet, so neither the Cloud Run
# products on rust-run nor anything outside the VPC can reach an app directly.
locals {
  observability_web = var.observability_web == null ? {} : merge(
    local.grafana_backend ? {
      grafana = {
        ip       = var.observability_web.grafana_ip
        selector = { application = "observability", component = "grafana" }
      }
    } : {},
    var.langfuse_enabled ? {
      langfuse = {
        ip = var.observability_web.langfuse_ip
        # The chart's web pod labels (its own langfuse-web Service selects
        # the same three).
        selector = {
          "app.kubernetes.io/name"     = "langfuse"
          "app.kubernetes.io/instance" = local.langfuse_release
          app                          = "web"
        }
      }
    } : {},
    # Only the issuer's authorize step is meant for browsers; public-site
    # passes nothing else under /langfuse-sso (langfuse-sso.tf).
    local.langfuse_sso ? {
      langfuse-sso = {
        ip       = var.observability_web.langfuse_sso_ip
        selector = local.langfuse_sso_labels
      }
    } : {},
  )
}

resource "kubernetes_service_v1" "observability_web" {
  for_each = local.observability_web

  metadata {
    name      = "${each.key}-internal"
    namespace = kubernetes_namespace.observability.metadata[0].name
    annotations = {
      "networking.gke.io/load-balancer-type" = "Internal"
    }
  }

  spec {
    type                        = "LoadBalancer"
    selector                    = each.value.selector
    load_balancer_ip            = each.value.ip
    load_balancer_source_ranges = var.observability_web.source_ranges
    external_traffic_policy     = "Local"

    port {
      name        = "http"
      protocol    = "TCP"
      port        = 3000
      target_port = 3000
    }
  }

  depends_on = [helm_release.langfuse, kubernetes_deployment_v1.langfuse_sso]
}
