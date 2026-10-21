variable "project_id" {
  description = "Project hosting the physician runtime cluster this stack lands on."
  type        = string
  default     = "coda-health-staging"
}

variable "region" {
  description = "Region of the runtime cluster, the Langfuse database, and the blob bucket."
  type        = string
  default     = "us-east1"
}

variable "cluster_name" {
  description = "Existing Autopilot cluster owned by infra/gcp/physician-runtime."
  type        = string
  default     = "coda-physician-runtime"
}

variable "network_name" {
  description = "Existing VPC (with Private Service Access) owned by infra/gcp/physician-runtime."
  type        = string
  default     = "coda-physician-runtime"
}

variable "environment" {
  description = "Deployment environment. Selects the Doppler config and the Datadog env tag."
  type        = string
  default     = "staging"

  validation {
    condition     = contains(["staging", "production"], var.environment)
    error_message = "environment must be staging or production."
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

variable "alert_notify" {
  description = "Datadog notification handle for every monitor, e.g. @slack-coda-alerts or @pagerduty-coda."
  type        = string
  default     = "@slack-coda-alerts"

  validation {
    condition     = can(regex("^@[A-Za-z0-9._@-]+$", var.alert_notify))
    error_message = "alert_notify must be one Datadog @-handle."
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

variable "site_p95_latency_threshold_seconds" {
  description = "physician-site p95 server latency over 10 minutes that pages."
  type        = number
  default     = 2
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
      var.langfuse_hostname != null && var.langfuse_iap_client_id != null && var.langfuse_iap_certificate != null && length(var.langfuse_iap_members) > 0
    )
    error_message = "langfuse_iap_enabled requires langfuse_hostname, langfuse_iap_client_id, langfuse_iap_certificate, and at least one langfuse_iap_members entry."
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
}

variable "proxy_only_subnet_cidr" {
  description = "Proxy-only subnet the regional internal load balancer needs; created only with the IAP gateway."
  type        = string
  default     = "10.36.8.0/23"
}
