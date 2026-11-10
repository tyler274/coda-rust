variable "environment" {
  type        = string
  description = "Apply this root separately for dev, stg, and prd; never a shared state."
  validation {
    condition     = contains(["dev", "stg", "prd"], var.environment)
    error_message = "environment must be dev, stg, or prd."
  }
}

variable "projects" {
  type = map(object({
    project_id        = string
    state_bucket_name = string
  }))
  description = "Operator-verified project IDs and state buckets, including the selected environment. Import an existing bootstrap project before applying."
  validation {
    condition = (
      contains(keys(var.projects), var.environment) &&
      alltrue([for environment in keys(var.projects) : contains(["dev", "stg", "prd"], environment)]) &&
      length(distinct([for p in values(var.projects) : p.project_id])) == length(var.projects) &&
      length(distinct([for p in values(var.projects) : p.state_bucket_name])) == length(var.projects) &&
      alltrue([for p in values(var.projects) : can(regex("^[a-z][a-z0-9-]{4,28}[a-z0-9]$", p.project_id))])
    )
    error_message = "Include the selected environment, use only dev/stg/prd keys, and supply distinct valid project IDs and state bucket names."
  }
}

variable "organization_id" {
  type        = string
  description = "Operator-approved organization; required even when a folder is supplied (IAP internal users belong to this organization)."
  validation {
    condition     = can(regex("^[0-9]+$", var.organization_id))
    error_message = "Supply a numeric organization ID."
  }
}

variable "folder_id" {
  type        = string
  default     = null
  description = "Optional operator-approved folder in organization_id. Operator must verify the parent relationship."
}

variable "billing_account" {
  type        = string
  description = "Operator-approved billing account; initial project creation needs billing association authority. Budgets remain outside this state with billing-account administrators."
}

variable "region" {
  type        = string
  default     = "us-east1"
  description = "Proposed region; operator must approve availability and residency."
}

variable "iap_group" {
  type        = string
  description = "Approved in-organization Google group email, without group: prefix. External identities require separate OAuth setup."
  validation {
    condition     = can(regex("^[^@: ]+@[^@ ]+\\.[^@ ]+$", var.iap_group))
    error_message = "iap_group must be a group email, not a public principal."
  }
}

variable "physician_proxy_invokers" {
  description = "Explicit operator-approved service-account IAP members for physician only (including cross-project public-site proxies). Empty by default, including production; never grants Duet or direct run.invoker access."
  type        = set(string)
  default     = []
  nullable    = false
  validation {
    condition     = alltrue([for member in var.physician_proxy_invokers : can(regex("^serviceAccount:[a-z][a-z0-9-]{4,28}[a-z0-9]@[a-z][a-z0-9-]{4,28}[a-z0-9]\\.iam\\.gserviceaccount\\.com$", member))])
    error_message = "physician_proxy_invokers must contain explicit serviceAccount:NAME@PROJECT.iam.gserviceaccount.com members, never groups, users, domains, or wildcards."
  }
}

variable "physician_simulation_practices" {
  description = "Physician practice ids whose chats are rehearsal traffic: telemetry labels their generations simulation and their sessions may tag a message with a scenario. Set on the physician site, worker and migration job (which seeds the rehearsal practice). Staging only; the binaries refuse it outside the staging-preview profile."
  type        = set(string)
  default     = []
  nullable    = false
  validation {
    condition     = length(var.physician_simulation_practices) == 0 || var.environment == "stg"
    error_message = "physician_simulation_practices is staging-only; production never relabels physician traffic."
  }
  validation {
    condition     = alltrue([for practice in var.physician_simulation_practices : can(regex("^[a-z0-9][a-z0-9-]{2,62}$", practice))])
    error_message = "physician_simulation_practices entries are practice ids: lower-case letters, digits and dashes."
  }
}

variable "observability_proxy_invokers" {
  description = "Explicit operator-approved service-account IAP members for the observability relay only: in staging, the public-site service account that serves /grafana/. Never grants physician, Duet or direct run.invoker access."
  type        = set(string)
  default     = []
  nullable    = false
  validation {
    condition     = alltrue([for member in var.observability_proxy_invokers : can(regex("^serviceAccount:[a-z][a-z0-9-]{4,28}[a-z0-9]@[a-z][a-z0-9-]{4,28}[a-z0-9]\\.iam\\.gserviceaccount\\.com$", member))])
    error_message = "observability_proxy_invokers must contain explicit serviceAccount:NAME@PROJECT.iam.gserviceaccount.com members, never groups, users, domains, or wildcards."
  }
}

variable "observability_relay_image" {
  type        = string
  default     = null
  nullable    = true
  description = "Initial digest-pinned observability-relay image (observability-relay.tf); null creates no relay resources. Later releases own the image field."
  validation {
    condition     = var.observability_relay_image == null ? true : var.environment == "stg" && var.observability_cluster_enabled && can(regex("^${var.region}-docker\\.pkg\\.dev/[a-z0-9-]+/apps/coda-observability-relay@sha256:[a-f0-9]{64}$", var.observability_relay_image))
    error_message = "The observability relay requires staging, the observability cluster and a digest-pinned coda-observability-relay image."
  }
}

variable "state_admin_members" {
  type        = set(string)
  description = "Environment-specific operator principals allowed to manage state objects. Never deployers or federated CI principals."
  validation {
    condition     = length(var.state_admin_members) > 0 && alltrue([for m in var.state_admin_members : startswith(m, "group:") || startswith(m, "serviceAccount:")])
    error_message = "Specify explicit operator groups or service accounts, not public principals."
  }
}

variable "github_repository_id" {
  type        = string
  description = "Immutable numeric GitHub ID for Coda-Health/coda-rust, verified by the operator."
  validation {
    condition     = can(regex("^[0-9]+$", var.github_repository_id))
    error_message = "Supply the immutable numeric coda-rust repository ID."
  }
}

variable "github_owner_id" {
  type        = string
  description = "Immutable numeric GitHub owner ID for Coda-Health."
  validation {
    condition     = can(regex("^[0-9]+$", var.github_owner_id))
    error_message = "Supply the immutable numeric repository owner ID."
  }
}

variable "enable_databases" {
  type        = bool
  default     = false
  description = "Explicit cost gate: creates two private PostgreSQL instances, even if services remain off."
}

variable "enable_workloads" {
  type        = bool
  default     = false
  description = "Explicit activation gate; foundations alone create no application service or migration job."
}

variable "production_activation_approved" {
  type        = bool
  default     = false
  description = "Separate operator authorization for ANY prod database or workload activation; never controlled by staging CI."
}

variable "database_passwords" {
  type        = map(string)
  default     = {}
  description = "Write-only passwords from Doppler for all four per-app, per-role database users. Required when databases are enabled; foundations alone need no credentials. Write-only values never enter state."
  sensitive   = true
  nullable    = false

  validation {
    condition = !var.enable_databases || alltrue([
      for key in ["physician-runtime", "physician-migration", "duet-runtime", "duet-migration"] :
      try(trimspace(var.database_passwords[key]) != "" && trimspace(var.database_passwords[key]) != "bootstrap-replaced-out-of-band", false)
    ])
    error_message = "Enabling databases requires a nonempty Doppler password for each of physician-runtime, physician-migration, duet-runtime, and duet-migration. The former bootstrap placeholder is not a credential."
  }
}

variable "database_password_versions" {
  type        = map(number)
  default     = {}
  description = "Bump the number for a key to apply a rotated database_passwords value. Only the version is stored, never the password."
}

variable "runtime_and_database_contract_approved" {
  type        = bool
  default     = false
  description = "Operator attests SQL grants were installed, database passwords live only in Doppler and the runtime connects with sslmode=require over the private IP, durable per-account auth/chat, no auto-migrate, enforced pool/model bounds, and verified readiness. This is a prerequisite, not implemented by infrastructure."
}

variable "enable_physician_scale" {
  type        = bool
  default     = false
  description = "Explicit cost gate for physician scaling: Memorystore Redis (workspace_changed fan-out) and the internal physician-worker service, which pulls accepted turns from Postgres. Effective only when enable_workloads is true. Duet is never affected."
}

variable "images" {
  type = map(object({
    runtime   = string
    migration = string
    # Physician only, required when enable_physician_scale is effective: the
    # initial physician-worker digest. Subsequent images are release-owned.
    worker = optional(string)
  }))
  default     = {}
  description = "Initial physician/duet runtime and migration image digests (plus physician.worker when physician scaling is on) from this environment's Artifact Registry. Subsequent images are release-owned."
}

variable "runtime_environment" {
  type        = map(map(string))
  default     = {}
  description = "Approved NONSECRET per-app runtime settings, including product auth/API mode and enforced model/pool limits agreed with runtime owners. All secrets remain in Doppler. No invented runtime flags are supplied."
}

variable "migration_environment" {
  type        = map(map(string))
  default     = {}
  description = "Approved NONSECRET per-app migration-job settings, such as the staging-preview profile and its seeded sign-in account list. Secrets remain in the migration Doppler config."
}

variable "doppler_token_versions" {
  type = map(object({
    runtime   = string
    migration = string
  }))
  default     = {}
  description = "Pinned numeric Secret Manager versions for per-app, per-role DOPPLER_TOKEN bootstrap secrets. Operator populates out of band; no values enter state. Migration tokens must expose DB bootstrap only, not model/application secrets."
}

variable "doppler_configs" {
  type        = map(string)
  default     = {}
  description = "Four per-product/per-role least-scope Doppler child configs derived from dev/stg/prd (keys physician-runtime, physician-migration, duet-runtime, duet-migration). A token scoped only to the shared parent would leak cross-product authority. Migration configs contain no model keys."
}

variable "max_instances" {
  type    = number
  default = 2
  validation {
    condition     = var.max_instances >= 1 && var.max_instances <= 5 && floor(var.max_instances) == var.max_instances
    error_message = "Use 1–5 instances per app. Account for overlapping revisions when sizing pools."
  }
}

variable "observability_cluster_enabled" {
  type        = bool
  default     = false
  description = "Explicit cost gate for the private GKE Autopilot cluster on rust-private that hosts the OpenTelemetry gateway (observability-cluster.tf). The infra/gcp/observability root deploys the gateway into it."
  validation {
    condition     = !var.observability_cluster_enabled || var.environment == "stg"
    error_message = "Observability is scoped to staging; this gate cannot provision a development or production cluster."
  }
}

variable "observability_enabled" {
  type        = bool
  default     = false
  description = "Adds the otel-collector sidecar to physician, duet and physician-worker and points the product container at it. Requires observability_cluster_enabled and a running gateway; product health never depends on the sidecar."
}

variable "telemetry_backend" {
  type        = string
  default     = "datadog"
  description = "CODA_TELEMETRY_BACKEND set on the product containers while observability_enabled is on: datadog and both keep the dd.* log correlation keys alongside W3C ids; grafana drops the aliases. Must match infra/gcp/observability's telemetry_backend."
  validation {
    condition     = contains(["datadog", "grafana", "both"], var.telemetry_backend)
    error_message = "telemetry_backend must be datadog, grafana or both."
  }
}

variable "telemetry_content" {
  type        = string
  default     = "none"
  description = "CODA_TELEMETRY_CONTENT set on the product containers while observability_enabled is on: none records no prompt or completion text; full attaches it to generation spans, which the gateway sends only to Langfuse HIPAA cloud. Must match infra/gcp/observability's telemetry_content, which refuses full without langfuse_host."
  validation {
    condition     = contains(["none", "full"], var.telemetry_content)
    error_message = "telemetry_content must be none or full."
  }
}

variable "otel_sidecar_image" {
  type        = string
  default     = "otel/opentelemetry-collector-contrib:0.155.0"
  description = "Pinned OpenTelemetry Collector Contrib image for the Cloud Run sidecar, the same release infra/gcp/observability runs as the gateway. No Docker Hub mirror exists in this project, so Cloud Run pulls the exact public release tag; never latest."
  validation {
    condition     = can(regex("^[^:@ ]+(:[0-9]+\\.[0-9]+\\.[0-9]+(@sha256:[a-f0-9]{64})?|@sha256:[a-f0-9]{64})$", var.otel_sidecar_image))
    error_message = "otel_sidecar_image must be pinned to an exact X.Y.Z tag and/or a sha256 digest."
  }
}

variable "otel_gateway_audience" {
  type        = string
  default     = null
  description = "OIDC audience the sidecar presents to the gateway and the gateway verifies. Null derives coda-otel-gateway-<environment>."
}

variable "sql_tier" {
  type        = string
  default     = "db-custom-1-3840"
  description = "Per-product always-on cost once databases are enabled; no shared product DB authority."
}

variable "sql_disk_limit_gb" {
  type    = number
  default = 50
  validation {
    condition     = var.sql_disk_limit_gb >= 20 && var.sql_disk_limit_gb <= 200
    error_message = "SQL disk limit must be 20–200 GiB."
  }
}
