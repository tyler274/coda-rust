# Grafana operations backend (telemetry_backend = grafana): self-hosted
# Tempo (traces), Loki (logs), Prometheus (span metrics Tempo derives) and
# Grafana (dashboards, the four Coda alerts, Slack). Data reaches them only
# through the gateway collector (collector.tf), after its caller filter and
# content-stripping processors, so nothing here changes what leaves a service.
#
# Each component is one replica of a StatefulSet with its own persistent disk
# and a ClusterIP Service; nothing is exposed outside the cluster. People use
# `kubectl port-forward svc/grafana 3000`, as with Langfuse. The configs are
# the shared files under dev/observability/grafana, read with file() (their
# ${...} references are expanded by Tempo and Loki with -config.expand-env and
# by Grafana's provisioning, never by tofu).
locals {
  grafana_config_dir = "${path.module}/../../../dev/observability/grafana"
  grafana_url        = "http://grafana.${local.namespace}.svc.cluster.local:3000"

  grafana_rules_config      = file("${local.grafana_config_dir}/provisioning/alerting/rules.yaml")
  grafana_rules_by_uid      = { for rule in flatten([for group in yamldecode(local.grafana_rules_config).groups : group.rules]) : rule.uid => rule }
  prometheus_retention      = "${var.grafana_metrics_retention_days}d"
  grafana_monitoring_enable = local.grafana_backend && var.grafana_cloud_monitoring_enabled
  grafana_ksa               = "grafana"

  # Optional Cloud Monitoring datasource for Cloud Run and GKE platform
  # metrics. GCE authentication reads the pod's Workload Identity, so no key
  # exists anywhere.
  grafana_cloud_monitoring_datasource = yamlencode({
    apiVersion = 1
    datasources = [{
      name     = "Google Cloud Monitoring"
      uid      = "cloud-monitoring"
      type     = "stackdriver"
      access   = "proxy"
      editable = false
      jsonData = {
        authenticationType = "gce"
        defaultProject     = var.cluster_project
      }
    }]
  })

  grafana_contact = yamldecode(file("${local.grafana_config_dir}/provisioning/alerting/contact.yaml"))
  grafana_test_contact = {
    orgId = 1
    name  = "coda-slack-test"
    receivers = [{
      uid      = "coda-slack-test", type = "slack", disableResolveMessage = false
      settings = { url = "$${GRAFANA_SLACK_WEBHOOK_URL}" }
    }]
  }
  grafana_contact_config = yamlencode({
    apiVersion          = 1
    contactPoints       = concat(local.grafana_contact.contactPoints, var.grafana_test_notifications_enabled ? [local.grafana_test_contact] : [])
    muteTimes           = local.grafana_contact.muteTimes
    deleteContactPoints = var.grafana_test_notifications_enabled ? [] : [{ orgId = 1, uid = "coda-slack-test" }]
    policies = [merge(local.grafana_contact.policies[0], {
      routes = concat(var.grafana_test_notifications_enabled ? [{
        receiver        = "coda-slack-test"
        object_matchers = [["qualification", "=", "synthetic-notification-test"], ["alertname", "=", "Coda synthetic notification test"]]
        continue        = false
      }] : [], local.grafana_contact.policies[0].routes)
    })]
  })
  grafana_compatibility_config = var.grafana_compatibility_metrics_url == "" ? {
    "compatibility-datasource.yaml" = yamlencode({
      apiVersion        = 1
      deleteDatasources = [{ orgId = 1, name = "Compatibility metrics" }]
    })
    "compatibility-rules.yaml" = yamlencode({
      apiVersion  = 1
      deleteRules = [for uid in ["coda-readiness-shadow", "coda-dependency-shadow", "coda-signal-freshness-shadow"] : { orgId = 1, uid = uid }]
    })
    } : {
    "compatibility-datasource.yaml" = yamlencode({
      apiVersion  = 1
      datasources = [{ name = "Compatibility metrics", uid = "compatibility", type = "prometheus", access = "proxy", editable = false, url = var.grafana_compatibility_metrics_url }]
    })
    "compatibility-rules.yaml" = file("${local.grafana_config_dir}/compatibility-rules.yaml")
  }

  # Uniform shape for the four StatefulSets, empty under datadog. config_items
  # maps each ConfigMap key to its path under config_mount. Images and args
  # match the local compose profile `grafana`, which proves them end to end.
  grafana_stack = { for name, component in local.grafana_components : name => component if local.grafana_backend }
  grafana_components = {
    tempo = {
      image        = "docker.io/grafana/tempo:3.1.1"
      args         = ["-target=all", "-config.file=/etc/tempo/tempo.yaml", "-config.expand-env=true"]
      config_data  = { "tempo.yaml" = file("${local.grafana_config_dir}/tempo.yaml") }
      config_items = { "tempo.yaml" = "tempo.yaml" }
      config_mount = "/etc/tempo"
      data_mount   = "/var/tempo"
      disk         = "50Gi"
      user         = 10001
      env = {
        PROMETHEUS_REMOTE_WRITE_URL = local.prometheus_remote_url
        TEMPO_BLOCK_RETENTION       = "${var.grafana_traces_retention_days * 24}h"
      }
      env_from_secret = null
      ports           = { otlp-grpc = 4317, http = 3200 }
      probe           = { path = "/ready", port = 3200 }
      resources       = { cpu = "500m", memory = "2Gi" }
    }
    loki = {
      image        = "docker.io/grafana/loki:3.7.9"
      args         = ["-config.file=/etc/loki/loki.yaml", "-config.expand-env=true"]
      config_data  = { "loki.yaml" = file("${local.grafana_config_dir}/loki.yaml") }
      config_items = { "loki.yaml" = "loki.yaml" }
      config_mount = "/etc/loki"
      data_mount   = "/loki"
      disk         = "50Gi"
      user         = 10001
      env = {
        LOKI_RETENTION_PERIOD = "${var.grafana_logs_retention_days * 24}h"
      }
      env_from_secret = null
      ports           = { http = 3100 }
      probe           = { path = "/ready", port = 3100 }
      resources       = { cpu = "500m", memory = "1Gi" }
    }
    prometheus = {
      image = "docker.io/prom/prometheus:v3.16.0"
      args = [
        "--config.file=/etc/prometheus/prometheus.yaml",
        "--storage.tsdb.path=/prometheus",
        "--storage.tsdb.retention.time=${local.prometheus_retention}",
        "--web.enable-remote-write-receiver",
        "--enable-feature=exemplar-storage",
      ]
      config_data     = { "prometheus.yaml" = file("${local.grafana_config_dir}/prometheus.yaml") }
      config_items    = { "prometheus.yaml" = "prometheus.yaml" }
      config_mount    = "/etc/prometheus"
      data_mount      = "/prometheus"
      disk            = "20Gi"
      user            = 65534
      env             = {}
      env_from_secret = null
      ports           = { http = 9090 }
      probe           = { path = "/-/ready", port = 9090 }
      resources       = { cpu = "250m", memory = "1Gi" }
    }
    grafana = {
      image = "docker.io/grafana/grafana:13.3.0"
      args  = []
      config_data = merge(
        {
          "datasources.yaml" = file("${local.grafana_config_dir}/provisioning/datasources.yaml")
          "rules.yaml"       = local.grafana_rules_config
          "contact.yaml"     = local.grafana_contact_config
        },
        local.grafana_monitoring_enable ? { "cloud-monitoring.yaml" = local.grafana_cloud_monitoring_datasource } : {},
        local.grafana_compatibility_config,
      )
      config_items = merge(
        {
          "datasources.yaml" = "datasources/datasources.yaml"
          "rules.yaml"       = "alerting/rules.yaml"
          "contact.yaml"     = "alerting/contact.yaml"
        },
        local.grafana_monitoring_enable ? { "cloud-monitoring.yaml" = "datasources/cloud-monitoring.yaml" } : {},
        {
          "compatibility-datasource.yaml" = "datasources/compatibility.yaml"
          "compatibility-rules.yaml"      = "alerting/compatibility.yaml"
        },
      )
      config_mount = "/etc/grafana/provisioning"
      data_mount   = "/var/lib/grafana"
      disk         = "5Gi"
      user         = 472
      env = {
        GF_SERVER_ROOT_URL                    = "http://localhost:3000"
        GF_USERS_ALLOW_SIGN_UP                = "false"
        GF_AUTH_ANONYMOUS_ENABLED             = "false"
        GF_ANALYTICS_REPORTING_ENABLED        = "false"
        GF_ANALYTICS_CHECK_FOR_UPDATES        = "false"
        GF_ANALYTICS_CHECK_FOR_PLUGIN_UPDATES = "false"
        GF_NEWS_NEWS_FEED_ENABLED             = "false"
        GF_SECURITY_ADMIN_USER                = "admin"
      }
      # GF_SECURITY_ADMIN_PASSWORD and GRAFANA_SLACK_WEBHOOK_URL (doppler.tf).
      env_from_secret = "grafana-secrets"
      ports           = { http = 3000 }
      probe           = { path = "/api/health", port = 3000 }
      resources       = { cpu = "250m", memory = "512Mi" }
    }
  }
}

resource "kubernetes_config_map_v1" "grafana_stack" {
  for_each = local.grafana_stack

  metadata {
    name      = each.key
    namespace = kubernetes_namespace.observability.metadata[0].name
  }

  data = each.value.config_data

  lifecycle {
    # rules.yaml fixes its thresholds (Grafana provisioning has no defaults
    # for unset variables); this root's threshold variables must match them
    # so both backends alert on the same numbers.
    precondition {
      condition = each.key != "grafana" || (
        local.grafana_rules_by_uid["coda-site-5xx-rate"].data[1].model.conditions[0].evaluator.params[0] == var.site_5xx_rate_threshold &&
        local.grafana_rules_by_uid["coda-chat-generation-errors"].data[1].model.conditions[0].evaluator.params[0] == var.chat_generation_error_threshold &&
        local.grafana_rules_by_uid["coda-async-failures"].data[1].model.conditions[0].evaluator.params[0] == var.async_failure_threshold &&
        alltrue([
          for service, threshold in var.service_p95_latency_threshold_seconds : anytrue([
            for segment in split(" or ", replace(local.grafana_rules_by_uid["coda-site-p95-latency"].data[0].model.expr, "\n", " ")) :
            strcontains(segment, "{service=\"${service}\"") && endswith(trimspace(segment), "/ ${threshold}")
          ])
        ])
      )
      error_message = "Under telemetry_backend = grafana the alert thresholds come from dev/observability/grafana/provisioning/alerting/rules.yaml; change that file together with the threshold variables."
    }
  }
}

resource "kubernetes_service_account_v1" "grafana" {
  count = local.grafana_count

  metadata {
    name      = local.grafana_ksa
    namespace = kubernetes_namespace.observability.metadata[0].name
    annotations = local.grafana_monitoring_enable ? {
      "iam.gke.io/gcp-service-account" = google_service_account.grafana_monitoring[0].email
    } : {}
  }

  # Only the Cloud Monitoring datasource calls a Google API, through the GKE
  # metadata server; nothing reads the Kubernetes API.
  automount_service_account_token = false
}

resource "kubernetes_stateful_set_v1" "grafana_stack" {
  for_each = local.grafana_stack

  metadata {
    name      = each.key
    namespace = kubernetes_namespace.observability.metadata[0].name
    labels    = { application = "observability", component = each.key }
  }

  spec {
    replicas     = 1
    service_name = kubernetes_service_v1.grafana_stack[each.key].metadata[0].name

    selector {
      match_labels = { application = "observability", component = each.key }
    }

    template {
      metadata {
        labels = { application = "observability", component = each.key }
        annotations = {
          # Roll the pod when its config or a retention setting changes.
          "checksum/config" = sha256(jsonencode([each.value.config_data, each.value.args, each.value.env]))
        }
      }

      spec {
        service_account_name            = each.key == "grafana" ? kubernetes_service_account_v1.grafana[0].metadata[0].name : null
        automount_service_account_token = false

        security_context {
          run_as_non_root = true
          run_as_user     = each.value.user
          run_as_group    = each.value.user
          fs_group        = each.value.user
        }

        container {
          name  = each.key
          image = each.value.image
          args  = each.value.args

          dynamic "env" {
            for_each = each.value.env
            content {
              name  = env.key
              value = env.value
            }
          }

          dynamic "env_from" {
            for_each = each.value.env_from_secret == null ? [] : [each.value.env_from_secret]
            content {
              secret_ref {
                name = env_from.value
              }
            }
          }

          dynamic "port" {
            for_each = each.value.ports
            content {
              name           = port.key
              container_port = port.value
            }
          }

          readiness_probe {
            http_get {
              path = each.value.probe.path
              port = each.value.probe.port
            }
            initial_delay_seconds = 10
            period_seconds        = 10
          }

          # Autopilot bills requests and sets limits to match.
          resources {
            requests = each.value.resources
            limits   = each.value.resources
          }

          volume_mount {
            name       = "config"
            mount_path = each.value.config_mount
            read_only  = true
          }

          volume_mount {
            name       = "data"
            mount_path = each.value.data_mount
          }
        }

        volume {
          name = "config"
          config_map {
            name = kubernetes_config_map_v1.grafana_stack[each.key].metadata[0].name
            dynamic "items" {
              for_each = each.value.config_items
              content {
                key  = items.key
                path = items.value
              }
            }
          }
        }
      }
    }

    volume_claim_template {
      metadata {
        name = "data"
      }
      spec {
        access_modes = ["ReadWriteOnce"]
        resources {
          requests = { storage = each.value.disk }
        }
      }
    }
  }

  depends_on = [helm_release.doppler_secrets]
}

resource "kubernetes_service_v1" "grafana_stack" {
  for_each = local.grafana_stack

  metadata {
    name      = each.key
    namespace = kubernetes_namespace.observability.metadata[0].name
  }

  spec {
    type     = "ClusterIP"
    selector = { application = "observability", component = each.key }

    dynamic "port" {
      for_each = each.value.ports
      content {
        name        = port.key
        port        = port.value
        target_port = port.value
      }
    }
  }

  lifecycle {
    ignore_changes = [metadata[0].annotations["cloud.google.com/neg"]]
  }
}

# Cloud Monitoring datasource identity (grafana_cloud_monitoring_enabled):
# a read-only Google service account the Grafana pod assumes through Workload
# Identity. It replaces the Datadog GCP integration under this backend.
resource "google_service_account" "grafana_monitoring" {
  count = local.grafana_monitoring_enable ? 1 : 0

  project      = var.cluster_project
  account_id   = "grafana-cloud-monitoring"
  display_name = "Grafana Cloud Monitoring datasource (${var.environment}, read-only)"
  description  = "Assumed by the in-cluster Grafana through Workload Identity to read Cloud Run and GKE metrics in this project. No key."

  depends_on = [google_project_service.workload]
}

resource "google_project_iam_member" "grafana_monitoring" {
  count = local.grafana_monitoring_enable ? 1 : 0

  project = var.cluster_project
  role    = "roles/monitoring.viewer"
  member  = "serviceAccount:${google_service_account.grafana_monitoring[0].email}"
}

resource "google_service_account_iam_member" "grafana_workload_identity" {
  count = local.grafana_monitoring_enable ? 1 : 0

  service_account_id = google_service_account.grafana_monitoring[0].name
  role               = "roles/iam.workloadIdentityUser"
  member             = "serviceAccount:${var.cluster_project}.svc.id.goog[${local.namespace}/${local.grafana_ksa}]"
}
