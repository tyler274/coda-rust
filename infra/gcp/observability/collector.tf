# One collector deployment fans every service's OTLP out to the selected
# operations backend (the Datadog Agent, or Tempo and Loki) and to Langfuse.
# It runs dev/observability/collector.yaml merged with the gateway overlay
# collector.gke.yaml (oidc receiver auth and the caller allowlist), then the
# backend overlay collector.<telemetry_backend>.yaml last, all unchanged:
# their ${env:...} references are resolved by the collector at start, so they
# are read with file(), not templatefile(), which would try to interpolate
# them. The backend overlay sets only the traces/ops and logs/ops exporters;
# without it those pipelines export to nop.
#
# Cloud Run reaches it through the otel-gateway Service below: an internal
# passthrough load balancer on the reserved gateway IP, open only to the
# Cloud Run egress range, OTLP/HTTP 4318 only. Every request carries a Google
# ID token minted by the calling service's own runtime identity for
# otel_gateway_audience; the overlay refuses anything else.
#
# With langfuse_host set, the otlphttp/langfuse exporter posts generation
# spans to that Langfuse project over HTTPS with Basic auth from the project
# key pair. Without it, a third file, collector.gke-no-langfuse.yaml, sends
# the generation pipeline to nop, so the collector needs no Langfuse endpoint
# or keys.
locals {
  collector_config_path         = "${path.module}/../../../dev/observability/collector.yaml"
  collector_config              = file(local.collector_config_path)
  collector_gateway_config_path = "${path.module}/../../../dev/observability/collector.gke.yaml"
  collector_gateway_config      = file(local.collector_gateway_config_path)
  collector_no_langfuse_config  = file("${path.module}/../../../dev/observability/collector.gke-no-langfuse.yaml")
  collector_backend_file        = "collector.${var.telemetry_backend}.yaml"
  collector_backend_config      = file("${path.module}/../../../dev/observability/${local.collector_backend_file}")
  collector_config_data = merge(
    {
      "collector.yaml"               = local.collector_config
      "collector.gke.yaml"           = local.collector_gateway_config
      (local.collector_backend_file) = local.collector_backend_config
    },
    local.langfuse_enabled ? {} : { "collector.gke-no-langfuse.yaml" = local.collector_no_langfuse_config },
    var.collector_self_metrics_enabled ? { "collector.self-metrics.yaml" = file("${path.module}/../../../dev/observability/collector.self-metrics.yaml") } : {},
  )
  # Later files merge over earlier ones, so the order is fixed here: the
  # backend overlay is always last.
  collector_config_files = concat(
    ["collector.yaml", "collector.gke.yaml"],
    local.langfuse_enabled ? [] : ["collector.gke-no-langfuse.yaml"],
    var.collector_self_metrics_enabled ? ["collector.self-metrics.yaml"] : [],
    [local.collector_backend_file],
  )
  # Backend endpoints the overlay reads; only the selected backend's are set.
  collector_backend_env = merge(
    local.datadog_backend ? { DATADOG_OTLP_ENDPOINT = local.datadog_agent_otlp_grpc } : {},
    local.grafana_backend ? {
      TEMPO_OTLP_ENDPOINT = local.tempo_otlp_grpc
      LOKI_OTLP_ENDPOINT  = local.loki_otlp_http
    } : {},
  )
  collector_image    = "otel/opentelemetry-collector-contrib:0.155.0"
  collector_replicas = 2
  collector_labels = {
    application = "observability"
    component   = "otel-collector"
  }
  # The gateway admits these callers by exact email claim. The overlay splits
  # on commas without trimming, so the list is joined with no spaces.
  collector_allowed_callers = join(",", sort(distinct(var.otel_gateway_allowed_callers)))
  otel_gateway_service      = "otel-gateway"

  # Parse the shared policy's explicit attribute allowlists for focused
  # infrastructure contracts. Runtime privacy behavior is tested by telemetry.
  collector_parsed = yamldecode(local.collector_config)
  collector_trace_allowed_keys = {
    for group in local.collector_parsed.processors["transform/strip_content"].trace_statements :
    group.context => jsondecode(one([
      for statement in group.statements : regex("^keep_keys\\(attributes, (.*)\\)$", statement)[0]
      if startswith(statement, "keep_keys(attributes,")
    ]))
  }
  collector_log_allowed_keys = {
    for group in local.collector_parsed.processors["transform/strip_log_content"].log_statements :
    group.context => jsondecode(one([
      for statement in group.statements : regex("^keep_keys\\(attributes, (.*)\\)$", statement)[0]
      if startswith(statement, "keep_keys(attributes,")
    ]))
  }
  # The gateway overlay restates every pipeline's processors; tests prove it
  # keeps the content-stripping and generation-filter steps.
  collector_gateway_parsed = yamldecode(local.collector_gateway_config)
}

resource "kubernetes_config_map_v1" "collector" {
  metadata {
    name      = local.collector_service
    namespace = kubernetes_namespace.observability.metadata[0].name
  }

  data = local.collector_config_data
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
        annotations = merge(local.collector_metrics_annotations, {
          # Roll the pods when the shared config, an overlay (including the
          # backend overlay) or the caller allowlist changes.
          "checksum/config" = sha256(join("\n", concat(
            [for file in local.collector_config_files : local.collector_config_data[file]],
            [var.otel_gateway_audience, local.collector_allowed_callers],
          )))
        })
      }

      spec {
        service_account_name            = kubernetes_service_account_v1.collector.metadata[0].name
        automount_service_account_token = false

        # Autopilot admits the pod with this profile. Declaring it matches
        # the cluster; leaving it out makes every plan remove it.
        security_context {
          run_as_non_root     = false
          supplemental_groups = []

          seccomp_profile {
            type = "RuntimeDefault"
          }
        }

        toleration {
          key      = "kubernetes.io/arch"
          operator = "Equal"
          value    = "amd64"
          effect   = "NoSchedule"
        }

        container {
          name  = "otel-collector"
          image = local.collector_image
          # Later files merge over earlier ones: the gateway overlay adds auth
          # and the backend overlay, last, picks the operations exporters.
          args = [for file in local.collector_config_files : "--config=/etc/otelcol/${file}"]

          dynamic "env" {
            for_each = local.collector_backend_env
            content {
              name  = env.key
              value = env.value
            }
          }

          env {
            name  = "OTEL_GATEWAY_AUDIENCE"
            value = var.otel_gateway_audience
          }

          env {
            name  = "OTEL_GATEWAY_ALLOWED_CALLERS"
            value = local.collector_allowed_callers
          }

          dynamic "env" {
            for_each = local.langfuse_enabled ? [local.langfuse_otlp_endpoint] : []
            content {
              name  = "LANGFUSE_OTLP_ENDPOINT"
              value = env.value
            }
          }

          # LANGFUSE_PUBLIC_KEY / LANGFUSE_SECRET_KEY, synced from Doppler.
          dynamic "env_from" {
            for_each = local.langfuse_enabled ? ["otel-collector-langfuse"] : []
            content {
              secret_ref {
                name = env_from.value
              }
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

          # Autopilot writes these container defaults at admission.
          security_context {
            allow_privilege_escalation = false
            privileged                 = false
            read_only_root_filesystem  = false
            run_as_non_root            = false

            capabilities {
              add  = []
              drop = ["NET_RAW"]
            }
          }

          resources {
            requests = {
              cpu               = "250m"
              memory            = "512Mi"
              ephemeral-storage = "1Gi"
            }
            limits = {
              cpu               = "1"
              memory            = "1Gi"
              ephemeral-storage = "1Gi"
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

  # Autopilot's warden writes these after admission. The version and the
  # adjustment record change outside this root.
  lifecycle {
    ignore_changes = [
      metadata[0].annotations["autopilot.gke.io/resource-adjustment"],
      metadata[0].annotations["autopilot.gke.io/warden-version"],
    ]
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

  lifecycle {
    ignore_changes = [metadata[0].annotations["cloud.google.com/neg"]]
  }
}

# The only path into the collector from outside the cluster: a GKE internal
# passthrough Network Load Balancer on the reserved gateway address that
# infra/gcp/rust-environments owns. GKE turns loadBalancerSourceRanges into
# the VPC firewall rule, so only the Cloud Run egress subnet reaches it, and
# only on OTLP/HTTP 4318. Traffic stays inside the VPC, which Google
# encrypts in transit; the receiver itself is plain HTTP with oidc bearer
# authentication (collector.gke.yaml), no TLS of its own.
resource "kubernetes_service_v1" "otel_gateway" {
  metadata {
    name      = local.otel_gateway_service
    namespace = kubernetes_namespace.observability.metadata[0].name
    annotations = {
      "networking.gke.io/load-balancer-type" = "Internal"
    }
  }

  spec {
    type                        = "LoadBalancer"
    selector                    = local.collector_labels
    load_balancer_ip            = var.otel_gateway_ip
    load_balancer_source_ranges = var.otel_gateway_source_ranges
    # Keep the client IP so the source-range firewall sees Cloud Run's
    # address, and route only to nodes running a collector pod.
    external_traffic_policy = "Local"

    port {
      name        = "otlp-http"
      protocol    = "TCP"
      port        = 4318
      target_port = 4318
    }
  }

  # GKE's NEG controller writes these after creation; removing them would
  # fight the controller on every apply.
  lifecycle {
    ignore_changes = [
      metadata[0].annotations["cloud.google.com/neg"],
      metadata[0].annotations["cloud.google.com/neg-status"],
    ]
  }
}
