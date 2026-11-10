variable "project_id" {
  description = "Project hosting the observability cluster, the Langfuse stores, and the GCP integration identity. For staging this is the Rust staging project, the same one as cluster_project."
  type        = string
}

variable "region" {
  description = "Region of the Langfuse database and the blob bucket."
  type        = string
  default     = "us-east1"
}

variable "cluster_project" {
  description = "Project of the observability Autopilot cluster, from infra/gcp/rust-environments' observability_cluster output."
  type        = string
}

variable "cluster_location" {
  description = "Location (region) of the observability Autopilot cluster, from infra/gcp/rust-environments' observability_cluster output."
  type        = string
}

variable "cluster_name" {
  description = "Name of the observability Autopilot cluster, from infra/gcp/rust-environments' observability_cluster output. This root never manages the cluster."
  type        = string
}

variable "network_name" {
  description = "Existing VPC the cluster and Cloud Run share (rust-private in staging), owned by infra/gcp/rust-environments. Read only: Langfuse's Cloud SQL attaches to its Private Service Access range."
  type        = string
}

variable "environment" {
  description = "Deployment environment. Selects the Doppler config and the Datadog env tag. Explicit: there is no default, and only staging is accepted until production observability is separately designed and authorized."
  type        = string

  validation {
    condition     = var.environment == "staging"
    error_message = "environment must be staging; production observability is not designed in this root."
  }
}

variable "otel_gateway_ip" {
  description = "Reserved INTERNAL address for the collector's internal passthrough load balancer, from infra/gcp/rust-environments' otel_gateway output. Sidecars send OTLP/HTTP to http://<ip>:4318."
  type        = string

  validation {
    condition     = can(cidrhost("${var.otel_gateway_ip}/32", 0)) && can(regex("^(10\\.|172\\.(1[6-9]|2[0-9]|3[01])\\.|192\\.168\\.)", var.otel_gateway_ip))
    error_message = "otel_gateway_ip must be one private RFC1918 IPv4 address."
  }
}

variable "otel_gateway_source_ranges" {
  description = "Only these CIDRs may reach the collector gateway: the Cloud Run Direct VPC egress subnet (rust-run) from infra/gcp/rust-environments."
  type        = list(string)

  validation {
    condition = length(var.otel_gateway_source_ranges) > 0 && alltrue([
      for cidr in var.otel_gateway_source_ranges : try(
        can(cidrhost(cidr, 0)) && tonumber(split("/", cidr)[1]) >= 16 &&
        can(regex("^(10\\.|172\\.(1[6-9]|2[0-9]|3[01])\\.|192\\.168\\.)", cidr)), false
      )
    ])
    error_message = "otel_gateway_source_ranges must list private CIDRs no wider than /16, never 0.0.0.0/0."
  }
}

variable "otel_gateway_audience" {
  description = "Audience every sidecar's Google ID token carries and the gateway's oidc authenticator requires, from infra/gcp/rust-environments' otel_gateway output."
  type        = string

  validation {
    condition     = trimspace(var.otel_gateway_audience) != ""
    error_message = "otel_gateway_audience must be nonempty."
  }
}

variable "otel_gateway_allowed_callers" {
  description = "Cloud Run runtime service-account emails whose Google ID tokens the gateway accepts (physician, duet, physician-worker). Every other Google identity is refused even with the right audience."
  type        = list(string)

  validation {
    condition = length(var.otel_gateway_allowed_callers) > 0 && alltrue([
      for email in var.otel_gateway_allowed_callers : can(regex("^[a-z][a-z0-9-]{4,28}[a-z0-9]@[a-z][a-z0-9-]{4,28}[a-z0-9]\\.iam\\.gserviceaccount\\.com$", email))
    ])
    error_message = "otel_gateway_allowed_callers must list explicit service-account emails, never users, groups or wildcards."
  }
}

variable "telemetry_backend" {
  description = "Operations destinations behind the gateway: datadog (compatibility default), grafana, or both with independent exporter queues. Datadog remains the primary operational notifier; Grafana evaluates in shadow mode. Langfuse is independent."
  type        = string
  default     = "datadog"

  validation {
    condition     = contains(["datadog", "grafana", "both"], var.telemetry_backend)
    error_message = "telemetry_backend must be datadog, grafana or both."
  }
}

variable "grafana_test_notifications_enabled" {
  description = "Explicitly enable only the labeled synthetic Grafana Slack route. Operational rules remain shadow-only; requires operator approval and the webhook in Doppler."
  type        = bool
  default     = false
  validation {
    condition     = !var.grafana_test_notifications_enabled || contains(["grafana", "both"], var.telemetry_backend)
    error_message = "Grafana test notifications require grafana or both."
  }
}

variable "grafana_compatibility_metrics_url" {
  description = "Optional private Prometheus-compatible source for normalized readiness/dependency signals. Empty disables its datasource and rules. The signal producer must implement the documented compatibility contract; Tempo span metrics are not this source. No credentials in URLs."
  type        = string
  default     = ""
  validation {
    condition     = var.grafana_compatibility_metrics_url == "" || can(regex("^https?://[a-zA-Z0-9.-]+(:[0-9]+)?/?$", var.grafana_compatibility_metrics_url))
    error_message = "Use a credential-free internal HTTP(S) origin, or empty to disable."
  }
  validation {
    condition     = var.grafana_compatibility_metrics_url == "" || contains(["grafana", "both"], var.telemetry_backend)
    error_message = "Compatibility metrics require grafana or both."
  }
}

variable "grafana_traces_retention_days" {
  description = "Days Tempo keeps traces (telemetry_backend = grafana)."
  type        = number
  default     = 14

  validation {
    condition     = var.grafana_traces_retention_days >= 1 && var.grafana_traces_retention_days <= 90 && floor(var.grafana_traces_retention_days) == var.grafana_traces_retention_days
    error_message = "grafana_traces_retention_days must be a whole number of days between 1 and 90."
  }
}

variable "grafana_logs_retention_days" {
  description = "Days Loki keeps log records (telemetry_backend = grafana)."
  type        = number
  default     = 30

  validation {
    condition     = var.grafana_logs_retention_days >= 1 && var.grafana_logs_retention_days <= 365 && floor(var.grafana_logs_retention_days) == var.grafana_logs_retention_days
    error_message = "grafana_logs_retention_days must be a whole number of days between 1 and 365."
  }
}

variable "grafana_metrics_retention_days" {
  description = "Days Prometheus keeps the span metrics Tempo derives (telemetry_backend = grafana)."
  type        = number
  default     = 30

  validation {
    condition     = var.grafana_metrics_retention_days >= 1 && var.grafana_metrics_retention_days <= 365 && floor(var.grafana_metrics_retention_days) == var.grafana_metrics_retention_days
    error_message = "grafana_metrics_retention_days must be a whole number of days between 1 and 365."
  }
}

variable "grafana_cloud_monitoring_enabled" {
  description = "Adds a Google Cloud Monitoring datasource to Grafana for Cloud Run and GKE platform metrics in cluster_project, read through Workload Identity by a service account holding only roles/monitoring.viewer. The Grafana counterpart of datadog_gcp_integration_enabled; off by default."
  type        = bool
  default     = false

  validation {
    condition     = !var.grafana_cloud_monitoring_enabled || contains(["grafana", "both"], var.telemetry_backend)
    error_message = "grafana_cloud_monitoring_enabled requires telemetry_backend = grafana."
  }
}

variable "datadog_gcp_integration_enabled" {
  description = "Creates the Datadog GCP integration for cluster_project only (Cloud Run and GKE metrics through service-account impersonation; no key). Billable and external: off until the operator and the Datadog account owner approve it."
  type        = bool
  default     = false

  validation {
    condition     = !var.datadog_gcp_integration_enabled || contains(["datadog", "both"], var.telemetry_backend)
    error_message = "The Datadog GCP integration requires telemetry_backend = datadog; grafana uses grafana_cloud_monitoring_enabled."
  }
}

variable "datadog_gcp_cloud_run_services" {
  description = "Cloud Run services whose revision metrics the GCP integration imports. Other services in the project (the mail relay, for one) stay out."
  type        = set(string)
  default     = ["physician", "duet", "physician-worker"]

  validation {
    condition     = length(var.datadog_gcp_cloud_run_services) > 0 && alltrue([for s in var.datadog_gcp_cloud_run_services : can(regex("^[a-z][a-z0-9-]{0,62}$", s))])
    error_message = "datadog_gcp_cloud_run_services must list Cloud Run service names."
  }
}

variable "datadog_gcp_disabled_metric_namespaces" {
  description = "Datadog GCP metric namespace ids switched off. The integration has no allowlist, so the other services this project hosts are disabled by id; Cloud Run (run) and GKE stay on. Confirm the ids against the integration's namespace list before enabling."
  type        = set(string)
  default     = ["cloudsql", "pubsub", "redis", "storage", "prometheus"]

  validation {
    condition     = length(setintersection(var.datadog_gcp_disabled_metric_namespaces, toset(["run", "gke", "container", "kubernetes"]))) == 0
    error_message = "Cloud Run and GKE namespaces are what the integration is for; do not disable them here."
  }
}

variable "doppler_token_secret_id" {
  description = "Existing Secret Manager secret holding only a read-only, config-scoped DOPPLER_TOKEN; never its value."
  type        = string
  default     = "observability-doppler-token"
}

variable "doppler_token_secret_version" {
  description = "Pinned numeric version of the DOPPLER_TOKEN secret. Bumping it rewrites the in-cluster token."
  type        = string
  default     = "1"

  validation {
    condition     = can(regex("^[1-9][0-9]*$", var.doppler_token_secret_version))
    error_message = "Pin a numeric DOPPLER_TOKEN secret version, not latest."
  }
}

variable "telemetry_content" {
  description = "CODA_TELEMETRY_CONTENT handed to services: none sends no prompt or completion text anywhere; full sends it to the in-project Langfuse only."
  type        = string
  default     = "none"

  validation {
    condition     = contains(["none", "full"], var.telemetry_content)
    error_message = "telemetry_content must be none or full."
  }

  # Without Langfuse the generation pipeline goes to nop, so full would only
  # add content nobody may receive.
  validation {
    condition     = var.telemetry_content == "none" || var.langfuse_enabled
    error_message = "telemetry_content = full requires langfuse_enabled; content goes only to the in-project Langfuse."
  }
}

variable "datadog_site" {
  description = "Datadog site the Agent and the monitors talk to."
  type        = string
  default     = "datadoghq.com"
}

variable "datadog_monitors_enabled" {
  description = "Separate operator gate after ingestion, queries and notification routing are verified. Does not enable any Datadog account products."
  type        = bool
  default     = false
  validation {
    condition     = !var.datadog_monitors_enabled || var.alert_notify != ""
    error_message = "Enabling monitors requires an explicitly confirmed notification handle."
  }
  validation {
    condition     = !var.datadog_monitors_enabled || contains(["datadog", "both"], var.telemetry_backend)
    error_message = "Datadog monitors require telemetry_backend = datadog; grafana provisions its own alert rules."
  }
}

variable "datadog_qualification_monitor_enabled" {
  description = "Separate gate for the synthetic readiness monitor on coda.qualification.synthetic_readiness. Off by default. Does not enable service monitors."
  type        = bool
  default     = false

  validation {
    condition     = !var.datadog_qualification_monitor_enabled || (var.alert_notify != "" && contains(["datadog", "both"], var.telemetry_backend))
    error_message = "The qualification monitor requires telemetry_backend datadog or both, and an explicitly confirmed alert handle."
  }
}

variable "chat_failure_monitor_enabled" {
  description = "Separate gate for the physician chat generation failure monitor alone. datadog_monitors_enabled also creates it; this gate does not enable the 5xx, latency, async-failure or memory monitors."
  type        = bool
  default     = false

  validation {
    condition     = !var.chat_failure_monitor_enabled || (var.alert_notify != "" && contains(["datadog", "both"], var.telemetry_backend))
    error_message = "The chat failure monitor requires telemetry_backend datadog or both, and an explicitly confirmed alert handle."
  }
}

variable "alert_notify" {
  description = "Operator-confirmed Datadog notification handle. Empty until routing is approved; there is no guessed Slack channel."
  type        = string
  default     = ""

  validation {
    condition     = var.alert_notify == "" || can(regex("^@[A-Za-z0-9._-]+$", var.alert_notify))
    error_message = "alert_notify must be one Datadog @-handle, or empty while monitors are disabled."
  }
}

variable "async_failure_threshold" {
  description = "Safe asynchronous failure events per service in ten minutes before alerting."
  type        = number
  default     = 0
  validation {
    condition     = var.async_failure_threshold >= 0
    error_message = "async_failure_threshold cannot be negative."
  }
}

variable "container_memory_limit_ratio" {
  description = "Share of its own memory limit an observability-namespace container may hold for 10 minutes before alerting."
  type        = number
  default     = 0.9
  validation {
    condition     = var.container_memory_limit_ratio > 0 && var.container_memory_limit_ratio < 1
    error_message = "container_memory_limit_ratio must be between 0 and 1."
  }
}

variable "site_5xx_rate_threshold" {
  description = "physician-site 5xx share of requests over 5 minutes that pages."
  type        = number
  default     = 0.05
}

variable "chat_generation_error_threshold" {
  description = "Failed chat generation spans in 10 minutes that pages."
  type        = number
  default     = 5
}

variable "service_p95_latency_threshold_seconds" {
  description = "Per-service p95 HTTP latency thresholds. Worker pushes include model execution and must not inherit the interactive API's two-second budget. Calibrate against staging ingestion before enabling monitors."
  type        = map(number)
  default = {
    physician-sample = 2
    physician-worker = 120
    duet-api         = 2
  }
  validation {
    condition     = toset(keys(var.service_p95_latency_threshold_seconds)) == toset(["physician-sample", "physician-worker", "duet-api"]) && alltrue([for threshold in values(var.service_p95_latency_threshold_seconds) : threshold > 0])
    error_message = "Supply a positive latency threshold for each of the three staging services."
  }
}

variable "langfuse_enabled" {
  description = "Deploys Langfuse and its stores. Off: the gateway still sends traces and logs to Datadog, and its generation pipeline goes to the nop exporter (collector.gke-no-langfuse.yaml). Turning it on needs the project IAM grants and the HMAC key in Doppler first."
  type        = bool
  default     = true
}

variable "langfuse_ee_trial" {
  description = "Sets LANGFUSE_EE_LICENSE_KEY on Langfuse web and worker from Doppler's LANGFUSE_EE_LICENSE_KEY, which unlocks the Enterprise features (project roles, audit logs, data retention, protected prompt labels, organization creators, ingestion masking). For a testing trial Doppler holds a langfuse_ee_ placeholder, which ee/LICENSE permits for development and testing only; a real key replaces it in Doppler alone."
  type        = bool
  default     = false

  validation {
    condition     = !var.langfuse_ee_trial || var.langfuse_enabled
    error_message = "langfuse_ee_trial requires langfuse_enabled."
  }
}

variable "langfuse_sql_tier" {
  description = "Cloud SQL machine tier for the Langfuse Postgres."
  type        = string
  default     = "db-custom-1-3840"
}

variable "langfuse_sql_disk_autoresize_limit_gb" {
  description = "Hard Cloud SQL storage ceiling for Langfuse."
  type        = number
  default     = 50

  validation {
    condition     = var.langfuse_sql_disk_autoresize_limit_gb >= 20
    error_message = "The storage ceiling must stay at or above the 20 GB initial disk."
  }
}

variable "langfuse_retention_days" {
  description = "Days Langfuse keeps traces and blobs. Langfuse is a copy of Postgres and git, so expiry loses nothing."
  type        = number
  default     = 90

  validation {
    condition     = var.langfuse_retention_days >= 7 && var.langfuse_retention_days <= 730
    error_message = "langfuse_retention_days must be between 7 and 730."
  }
}

variable "clickhouse_disk_gb" {
  description = "PVC size for the in-cluster ClickHouse."
  type        = number
  default     = 50
}

variable "staging_proxy" {
  description = "People reach Grafana and Langfuse at <origin>/grafana/ and <origin>/langfuse/ through public-site's mounts and the observability relay (README.md, \"Boundary\"). iap_audience is public-site's IAP audience (infra/gcp/public-site output iap_audience): Grafana signs people in from that verified assertion, which public-site forwards as X-Coda-IAP-Assertion. Null keeps both apps at their root paths for kubectl port-forward."
  type = object({
    origin       = string
    iap_audience = string
  })
  default = null

  validation {
    condition     = var.staging_proxy == null || can(regex("^https://([a-z0-9]([a-z0-9-]*[a-z0-9])?\\.)+[a-z]{2,}$", var.staging_proxy.origin))
    error_message = "staging_proxy.origin must be a bare https origin without a path, port or trailing slash."
  }

  validation {
    condition     = var.staging_proxy == null || can(regex("^/projects/[0-9]+/locations/[a-z0-9-]+/services/[a-z0-9-]+$", var.staging_proxy.iap_audience))
    error_message = "staging_proxy.iap_audience must be the Cloud Run IAP audience /projects/NUMBER/locations/REGION/services/NAME."
  }
}

variable "observability_web" {
  description = "Internal passthrough load balancers for grafana:3000, langfuse-web:3000 and langfuse-sso:3000, from infra/gcp/rust-environments' observability_relay output: the reserved INTERNAL addresses and the only source range they admit (the relay's rust-relay subnet). langfuse_sso_ip is needed only with langfuse_sso_image. Null keeps them all ClusterIP-only."
  type = object({
    grafana_ip      = string
    langfuse_ip     = string
    langfuse_sso_ip = optional(string)
    source_ranges   = list(string)
  })
  default = null

  validation {
    condition = var.observability_web == null || alltrue([
      for ip in concat(
        [try(var.observability_web.grafana_ip, ""), try(var.observability_web.langfuse_ip, "")],
        try(var.observability_web.langfuse_sso_ip, null) == null ? [] : [var.observability_web.langfuse_sso_ip],
      ) :
      can(cidrhost("${ip}/32", 0)) && can(regex("^(10\\.|172\\.(1[6-9]|2[0-9]|3[01])\\.|192\\.168\\.)", ip))
    ])
    error_message = "observability_web addresses must be private RFC1918 IPv4 addresses."
  }

  validation {
    condition = var.observability_web == null || try(length(var.observability_web.source_ranges) > 0 && alltrue([
      for cidr in var.observability_web.source_ranges : try(
        can(cidrhost(cidr, 0)) && tonumber(split("/", cidr)[1]) >= 24 &&
        can(regex("^(10\\.|172\\.(1[6-9]|2[0-9]|3[01])\\.|192\\.168\\.)", cidr)), false
      )
    ]), false)
    error_message = "observability_web.source_ranges must list private CIDRs no wider than /24: only the relay subnet, never the Cloud Run or node ranges."
  }

  validation {
    condition     = var.observability_web == null || var.staging_proxy != null
    error_message = "observability_web requires staging_proxy: the apps it exposes must be configured for their /grafana and /langfuse sub-paths."
  }
}

variable "langfuse_web_image" {
  description = "Digest-pinned Langfuse web image built from the pinned upstream tag with NEXT_PUBLIC_BASE_PATH=/langfuse (.github/workflows/langfuse-web-image.yml). Next.js bakes the base path in at build time, so the stock image cannot serve /langfuse; required with staging_proxy and langfuse_enabled. The worker stays stock."
  type        = string
  default     = null

  validation {
    condition     = var.langfuse_web_image == null || can(regex("^[a-z0-9.-]+-docker\\.pkg\\.dev/[a-z0-9-]+/[a-z0-9-]+/langfuse-web@sha256:[0-9a-f]{64}$", var.langfuse_web_image))
    error_message = "langfuse_web_image must be an Artifact Registry langfuse-web image pinned by sha256 digest."
  }

  validation {
    condition     = var.staging_proxy == null || !var.langfuse_enabled || var.langfuse_web_image != null
    error_message = "staging_proxy with langfuse_enabled requires langfuse_web_image, the /langfuse base-path build."
  }
}

variable "langfuse_sso_image" {
  description = "Digest-pinned coda-langfuse-sso image (crates/langfuse-sso), published by the staging release. Set, people who passed staging's IAP sign in to Langfuse as their Google account (langfuse-sso.tf). Null leaves Langfuse on its password form."
  type        = string
  default     = null

  validation {
    condition     = var.langfuse_sso_image == null || can(regex("^[a-z0-9.-]+-docker\\.pkg\\.dev/[a-z0-9-]+/[a-z0-9-]+/coda-langfuse-sso@sha256:[0-9a-f]{64}$", var.langfuse_sso_image))
    error_message = "langfuse_sso_image must be an Artifact Registry coda-langfuse-sso image pinned by sha256 digest."
  }

  validation {
    condition     = var.langfuse_sso_image == null || (var.langfuse_enabled && var.staging_proxy != null && try(var.observability_web.langfuse_sso_ip, null) != null)
    error_message = "langfuse_sso_image requires langfuse_enabled, staging_proxy and observability_web.langfuse_sso_ip: browsers reach its authorize step only through public-site and the relay."
  }
}
