# Datadog: Operator, one DatadogAgent for the Autopilot cluster, and the
# three monitors that page. The Agent receives OTLP only from the collector,
# whose Datadog pipeline has already deleted prompt and completion content.
locals {
  datadog_service_filter = "service:physician-sample,env:${local.dd_env}"

  datadog_agent = {
    apiVersion = "datadoghq.com/v2alpha1"
    kind       = "DatadogAgent"
    metadata = {
      name      = local.datadog_agent_name
      namespace = local.namespace
      annotations = {
        # Autopilot is never auto-detected; this provider swaps host paths,
        # images, and the PriorityClass for the Autopilot allowlist.
        "agent.datadoghq.com/cluster-provider" = "gke-autopilot"
      }
    }
    spec = {
      global = {
        clusterName = var.cluster_name
        site        = var.datadog_site
        tags        = ["env:${local.dd_env}"]
        credentials = {
          apiSecret = { secretName = "datadog-secret", keyName = "api-key" }
          appSecret = { secretName = "datadog-secret", keyName = "app-key" }
        }
      }
      features = {
        apm = { enabled = true }
        logCollection = {
          enabled             = true
          containerCollectAll = true
        }
        otlp = {
          receiver = {
            protocols = {
              grpc = { enabled = true, endpoint = "0.0.0.0:4317" }
              http = { enabled = true, endpoint = "0.0.0.0:4318" }
            }
          }
        }
      }
      override = {
        nodeAgent = {
          env = [
            # Langfuse and its stores run in this namespace and can log agent
            # content on errors; keep their logs out of Datadog. Metrics and
            # the collector's own spans still flow.
            { name = "DD_CONTAINER_EXCLUDE_LOGS", value = "kube_namespace:${local.namespace}" },
          ]
        }
      }
    }
  }
}

resource "helm_release" "datadog_operator" {
  name       = "datadog-operator"
  repository = "https://helm.datadoghq.com"
  chart      = "datadog-operator"
  version    = "2.30.0"
  namespace  = kubernetes_namespace.observability.metadata[0].name
  atomic     = true
}

resource "helm_release" "datadog_agent" {
  name      = "datadog-agent"
  chart     = local.manifests_chart
  namespace = kubernetes_namespace.observability.metadata[0].name
  atomic    = true

  values = [yamlencode({ manifests = [local.datadog_agent] })]

  depends_on = [
    helm_release.datadog_operator,
    helm_release.doppler_secrets,
  ]
}

resource "datadog_monitor" "site_5xx_rate" {
  name    = "[${local.dd_env}] physician-site 5xx rate"
  type    = "query alert"
  query   = "sum(last_5m):sum:trace.http.server.request.errors{${local.datadog_service_filter}}.as_count() / sum:trace.http.server.request.hits{${local.datadog_service_filter}}.as_count() > ${var.site_5xx_rate_threshold}"
  message = <<-EOT
    physician-site is answering more than ${var.site_5xx_rate_threshold * 100}% of requests with 5xx.
    Open the failing traces in APM, then follow coda.run_id to the Langfuse trace (book: Observability).
    ${var.alert_notify}
  EOT

  monitor_thresholds {
    critical = var.site_5xx_rate_threshold
  }

  require_full_window = false
  notify_no_data      = false
  tags                = ["service:physician-sample", "env:${local.dd_env}", "team:coda", "issue:eng-65"]
}

# Chat generation failures are tracing::error! events the site emits inside
# its gen_ai.generation span; crates/telemetry writes that span name as the
# JSON log's `span` field, plus the dd.trace_id that links to APM and from
# there to coda.run_id. In physician-sample only physician chat opens it.
resource "datadog_monitor" "chat_generation_errors" {
  name    = "[${local.dd_env}] physician chat generation errors"
  type    = "log alert"
  query   = "logs(\"service:physician-sample env:${local.dd_env} status:error @span:gen_ai.generation\").index(\"*\").rollup(\"count\").last(\"10m\") > ${var.chat_generation_error_threshold}"
  message = <<-EOT
    More than ${var.chat_generation_error_threshold} physician chat generations failed in 10 minutes.
    Each log line links to its trace; the generation span's coda.run_id finds the Langfuse trace.
    ${var.alert_notify}
  EOT

  monitor_thresholds {
    critical = var.chat_generation_error_threshold
  }

  notify_no_data = false
  tags           = ["service:physician-sample", "env:${local.dd_env}", "team:coda", "issue:eng-65"]
}

resource "datadog_monitor" "site_p95_latency" {
  name    = "[${local.dd_env}] physician-site p95 latency"
  type    = "query alert"
  query   = "percentile(last_10m):p95:trace.http.server.request{${local.datadog_service_filter}} > ${var.site_p95_latency_threshold_seconds}"
  message = <<-EOT
    physician-site p95 server latency is above ${var.site_p95_latency_threshold_seconds}s.
    Slow chat turns show their generation span (model, tokens, duration) in the trace.
    ${var.alert_notify}
  EOT

  monitor_thresholds {
    critical = var.site_p95_latency_threshold_seconds
  }

  require_full_window = false
  notify_no_data      = false
  tags                = ["service:physician-sample", "env:${local.dd_env}", "team:coda", "issue:eng-65"]
}
