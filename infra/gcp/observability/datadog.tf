# Datadog: Operator, one DatadogAgent for the Autopilot cluster, and gated
# per-service monitors. The Agent receives OTLP only from the collector,
# whose operations pipelines have already deleted prompt and completion
# content. Everything here exists only with telemetry_backend = datadog.
locals {
  monitored_services = local.datadog_backend && var.datadog_monitors_enabled ? toset(["physician-sample", "physician-worker", "duet-api"]) : toset([])
  async_services     = local.monitored_services

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
        }
      }
      features = {
        apm = { enabled = true }
        # Log forwarding stays on for the OTLP log records the gateway sends
        # (already stripped of content). No container in this cluster is
        # tailed by default: the cluster runs only this stack, and Langfuse
        # and its stores can log agent content on errors.
        logCollection = {
          enabled             = true
          containerCollectAll = false
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
            # Belt and braces with containerCollectAll = false: even an
            # autodiscovery annotation never tails this namespace's container
            # logs. OTLP log records are not container logs and still flow.
            { name = "DD_CONTAINER_EXCLUDE_LOGS", value = "kube_namespace:${local.namespace}" },
            # The CRD has no OTLP logs switch; the Agent ingests OTLP logs only
            # with this set. The gateway's logs/ops pipeline needs it.
            { name = "DD_OTLP_CONFIG_LOGS_ENABLED", value = "true" },
            { name = "DD_ENV", value = local.dd_env },
          ]
        }
      }
    }
  }
}

resource "helm_release" "datadog_operator" {
  count      = local.datadog_count
  name       = "datadog-operator"
  repository = "https://helm.datadoghq.com"
  chart      = "datadog-operator"
  version    = "2.30.0"
  namespace  = kubernetes_namespace.observability.metadata[0].name
  atomic     = true
}

# The releases gained a backend count; keep the applied staging instances.
moved {
  from = helm_release.datadog_operator
  to   = helm_release.datadog_operator[0]
}

moved {
  from = helm_release.datadog_agent
  to   = helm_release.datadog_agent[0]
}

resource "helm_release" "datadog_agent" {
  count     = local.datadog_count
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
  for_each = local.monitored_services
  name     = "[${local.dd_env}] ${each.key} 5xx rate"
  type     = "query alert"
  query    = "sum(last_5m):sum:trace.http.server.request.errors{service:${each.key},env:${local.dd_env}}.as_count() / sum:trace.http.server.request.hits{service:${each.key},env:${local.dd_env}}.as_count() > ${var.site_5xx_rate_threshold}"
  message  = <<-EOT
    ${each.key} is answering more than ${var.site_5xx_rate_threshold * 100}% of requests with 5xx.
    Open the failing request in APM. If it includes a physician attempt, read its canonical record using coda.run_id before following the matching generation to Langfuse. Duet operations do not require a Langfuse generation.
    ${var.alert_notify}
  EOT

  monitor_thresholds {
    critical = var.site_5xx_rate_threshold
  }

  require_full_window = false
  notify_no_data      = false
  tags                = ["service:${each.key}", "env:${local.dd_env}", "team:coda"]
}

# Select generation attributes emitted by the worker and retained by the gateway,
# not Datadog's derived operation name. Validate ingestion and trace retention
# before enabling; the query includes chat and reply-revision generations.
resource "datadog_monitor" "chat_generation_errors" {
  count   = local.datadog_backend && var.datadog_monitors_enabled ? 1 : 0
  name    = "[${local.dd_env}] physician chat generation errors"
  type    = "trace-analytics alert"
  query   = "trace-analytics(\"service:physician-worker env:${local.dd_env} status:error @coda.job:physician_* @gen_ai.system:*\").rollup(\"count\").last(\"10m\") > ${var.chat_generation_error_threshold}"
  message = <<-EOT
    More than ${var.chat_generation_error_threshold} physician chat generations failed in 10 minutes.
    Open the generation trace and use coda.run_id to read the canonical attempt, then find the matching Langfuse generation.
    ${var.alert_notify}
  EOT

  monitor_thresholds {
    critical = var.chat_generation_error_threshold
  }

  notify_no_data = false
  tags           = ["service:physician-worker", "env:${local.dd_env}", "team:coda"]
}

# Fixed operational events emitted by the product; not stdout/error text.
# These reach Datadog through the safe OTLP log pipeline. No run IDs become
# metric dimensions, and worker failures alert independently of HTTP status.
resource "datadog_monitor" "async_failures" {
  for_each = local.async_services
  name     = "[${local.dd_env}] ${each.key} asynchronous failures"
  type     = "log alert"
  query    = "logs(\"service:${each.key} env:${local.dd_env} status:error\").index(\"*\").rollup(\"count\").last(\"10m\") > ${var.async_failure_threshold}"
  message  = <<-EOT
    ${each.key} reported asynchronous failures, including work whose original HTTP request succeeded.
    Open the correlated trace from the log. For physician-worker, read the persisted canonical attempt using coda.run_id, then find that attempt's generation in Langfuse. A pre-claim failure has no canonical run ID; investigate its trace rather than inventing one.
    ${var.alert_notify}
  EOT
  monitor_thresholds {
    critical = var.async_failure_threshold
  }
  notify_no_data = false
  tags           = ["service:${each.key}", "env:${local.dd_env}", "team:coda"]
}

resource "datadog_monitor" "site_p95_latency" {
  for_each = local.monitored_services
  name     = "[${local.dd_env}] ${each.key} p95 latency"
  type     = "query alert"
  query    = "percentile(last_10m):p95:trace.http.server.request{service:${each.key},env:${local.dd_env}} > ${var.service_p95_latency_threshold_seconds[each.key]}"
  message  = <<-EOT
    ${each.key} p95 server latency is above ${var.service_p95_latency_threshold_seconds[each.key]}s.
    Slow chat turns show their generation span (model, tokens, duration) in the trace.
    ${var.alert_notify}
  EOT

  monitor_thresholds {
    critical = var.service_p95_latency_threshold_seconds[each.key]
  }

  require_full_window = false
  notify_no_data      = false
  tags                = ["service:${each.key}", "env:${local.dd_env}", "team:coda"]
}
