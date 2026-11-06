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

variable "datadog_gcp_integration_enabled" {
  description = "Creates the Datadog GCP integration for cluster_project only (Cloud Run and GKE metrics through service-account impersonation; no key). Billable and external: off until the operator and the Datadog account owner approve it."
  type        = bool
  default     = false
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

variable "langfuse_iap_enabled" {
  description = "Adds an IAP-gated internal Gateway for the Langfuse UI. Off: reach it with kubectl port-forward."
  type        = bool
  default     = false

  validation {
    condition = !var.langfuse_iap_enabled || (
      var.langfuse_hostname != null && var.langfuse_iap_client_id != null && var.langfuse_iap_certificate != null
    )
    error_message = "langfuse_iap_enabled requires langfuse_hostname, langfuse_iap_client_id, and langfuse_iap_certificate."
  }

  validation {
    condition     = !var.langfuse_iap_enabled || var.langfuse_enabled
    error_message = "The Langfuse IAP gateway requires langfuse_enabled."
  }
}

variable "langfuse_hostname" {
  description = "Internal hostname the IAP gateway serves Langfuse on."
  type        = string
  default     = null
  nullable    = true
}

variable "langfuse_iap_client_id" {
  description = "OAuth client id for IAP. Its secret syncs from Doppler as LANGFUSE_IAP_CLIENT_SECRET."
  type        = string
  default     = null
  nullable    = true
}

variable "langfuse_iap_certificate" {
  description = "Existing regional SSL certificate name the internal gateway terminates TLS with."
  type        = string
  default     = null
  nullable    = true
}

variable "langfuse_iap_members" {
  description = "IAM principals (user:, group:) allowed through IAP to the Langfuse UI."
  type        = list(string)
  default     = []

  validation {
    condition     = !var.langfuse_iap_enabled || length(var.langfuse_iap_members) == 0 || var.langfuse_iap_backend_service != null
    error_message = "Granting Langfuse access requires its operator-verified regional backend service name. Bootstrap the gateway with no members until that name is known."
  }
}

variable "langfuse_iap_backend_service" {
  description = "GKE-created regional backend service name, verified by the operator for the Langfuse HTTPRoute. Null bootstraps the gateway without granting access; never inferred from the Kubernetes Service name."
  type        = string
  default     = null

  validation {
    condition     = var.langfuse_iap_backend_service == null || try(trimspace(var.langfuse_iap_backend_service) != "", false)
    error_message = "The Langfuse regional backend service name must be nonempty when supplied."
  }
}

variable "proxy_only_subnet_cidr" {
  description = "Proxy-only subnet the regional internal load balancer needs; created only with the IAP gateway."
  type        = string
  default     = "10.36.8.0/23"
}
