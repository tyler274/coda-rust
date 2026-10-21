# One collector deployment fans every service's OTLP out to the Datadog Agent
# and to Langfuse. It runs dev/observability/collector.yaml unchanged: the
# file's ${env:...} references are resolved by the collector at start, so it
# is read with file(), not templatefile(), which would try to interpolate them.
locals {
  collector_config_path = "${path.module}/../../../dev/observability/collector.yaml"
  collector_config      = file(local.collector_config_path)
  collector_image       = "otel/opentelemetry-collector-contrib:0.155.0"
  collector_replicas    = 2
  collector_labels = {
    application = "observability"
    component   = "otel-collector"
  }

  # Parsed for tests/observability.tftest.hcl, which proves the Datadog
  # pipeline deletes content and Langfuse receives only generation spans.
  # The OTTL string literal escapes its backslashes; unescape them the way
  # OTTL does so the test applies the exact regex the collector runs.
  collector_parsed = yamldecode(local.collector_config)
  collector_strip_patterns = {
    for group in local.collector_parsed.processors["transform/strip_content"].trace_statements :
    group.context => [
      for s in group.statements : replace(regex("^delete_matching_keys\\(attributes, \"(.*)\"\\)$", s)[0], "\\\\", "\\")
    ]
  }
}

resource "kubernetes_config_map_v1" "collector" {
  metadata {
    name      = local.collector_service
    namespace = kubernetes_namespace.observability.metadata[0].name
  }

  data = {
    "collector.yaml" = local.collector_config
  }
}

resource "kubernetes_service_account_v1" "collector" {
  metadata {
    name      = local.collector_service
    namespace = kubernetes_namespace.observability.metadata[0].name
  }

  # The collector calls no Google API.
  automount_service_account_token = false
}

resource "kubernetes_deployment_v1" "collector" {
  metadata {
    name      = local.collector_service
    namespace = kubernetes_namespace.observability.metadata[0].name
    labels    = local.collector_labels
  }

  spec {
    replicas = local.collector_replicas

    selector {
      match_labels = local.collector_labels
    }

    template {
      metadata {
        labels = local.collector_labels
        annotations = {
          # Roll the pods when the shared config changes.
          "checksum/config" = sha256(local.collector_config)
        }
      }

      spec {
        service_account_name            = kubernetes_service_account_v1.collector.metadata[0].name
        automount_service_account_token = false

        container {
          name  = "otel-collector"
          image = local.collector_image
          args  = ["--config=/etc/otelcol/collector.yaml"]

          env {
            name  = "DATADOG_OTLP_ENDPOINT"
            value = local.datadog_agent_otlp_grpc
          }

          env {
            name  = "LANGFUSE_OTLP_ENDPOINT"
            value = local.langfuse_otlp_endpoint
          }

          # LANGFUSE_PUBLIC_KEY / LANGFUSE_SECRET_KEY, synced from Doppler.
          env_from {
            secret_ref {
              name = "otel-collector-langfuse"
            }
          }

          port {
            name           = "otlp-grpc"
            container_port = 4317
          }

          port {
            name           = "otlp-http"
            container_port = 4318
          }

          port {
            name           = "health"
            container_port = 13133
          }

          readiness_probe {
            http_get {
              path = "/"
              port = 13133
            }
            period_seconds = 10
          }

          liveness_probe {
            http_get {
              path = "/"
              port = 13133
            }
            initial_delay_seconds = 10
            period_seconds        = 20
          }

          resources {
            requests = {
              cpu    = "250m"
              memory = "512Mi"
            }
            limits = {
              cpu    = "1"
              memory = "1Gi"
            }
          }

          volume_mount {
            name       = "config"
            mount_path = "/etc/otelcol"
            read_only  = true
          }
        }

        volume {
          name = "config"
          config_map {
            name = kubernetes_config_map_v1.collector.metadata[0].name
          }
        }
      }
    }
  }

  depends_on = [helm_release.doppler_secrets]
}

resource "kubernetes_service_v1" "collector" {
  metadata {
    name      = local.collector_service
    namespace = kubernetes_namespace.observability.metadata[0].name
  }

  spec {
    selector = local.collector_labels

    port {
      name        = "otlp-grpc"
      port        = 4317
      target_port = 4317
    }

    port {
      name        = "otlp-http"
      port        = 4318
      target_port = 4318
    }

    type = "ClusterIP"
  }
}
