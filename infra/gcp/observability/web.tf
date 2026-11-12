# People reach Grafana only through public-site's /grafana/ mount
# (staging_proxy). public-site calls the IAP-protected observability relay in
# infra/gcp/rust-environments, and the relay's Direct VPC egress reaches this
# internal passthrough load balancer. It admits only the relay's own subnet,
# so neither the Cloud Run products on rust-run nor anything outside the VPC
# can reach Grafana directly.
locals {
  observability_web = var.observability_web == null || !local.grafana_backend ? {} : {
    grafana = {
      ip       = var.observability_web.grafana_ip
      selector = { application = "observability", component = "grafana" }
    }
  }
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
}
