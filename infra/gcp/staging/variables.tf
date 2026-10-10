variable "project_id" {
  description = "Isolated non-PHI Coda Agents staging project."
  type        = string
  default     = "coda-health-staging"
}

variable "region" {
  description = "Pinned region for staging runtime and data services."
  type        = string
  default     = "us-east1"
}

variable "network_cidr" {
  description = "Direct VPC egress subnet reserved for Cloud Run revisions."
  type        = string
  default     = "10.24.0.0/26"
}

variable "sql_tier" {
  description = "Cloud SQL machine tier used only during named proof windows."
  type        = string
  default     = "db-custom-1-3840"
}

variable "sql_activation_policy" {
  description = "ALWAYS during initial instance creation or a named proof window; NEVER when staging is parked."
  type        = string
  default     = "NEVER"

  validation {
    condition     = contains(["ALWAYS", "NEVER"], var.sql_activation_policy)
    error_message = "sql_activation_policy must be ALWAYS or NEVER."
  }
}

variable "application_image" {
  description = "Immutable Coda application image digest. Null leaves the hosted application undeployed."
  type        = string
  default     = null
  nullable    = true

  validation {
    condition = var.application_image == null || can(regex(
      "^us-east1-docker\\.pkg\\.dev/coda-health-staging/coda-agents/application@sha256:[0-9a-f]{64}$",
      var.application_image,
    ))
    error_message = "application_image must be the exact staging Artifact Registry sha256 digest."
  }
}

variable "application_browser_origin" {
  description = "Exact HTTPS browser origin allowed to mutate the hosted Coda application."
  type        = string
  default     = null
  nullable    = true

  validation {
    condition = var.application_browser_origin == null || can(regex(
      "^https://[^/@?#]+$",
      var.application_browser_origin,
    ))
    error_message = "application_browser_origin must be an exact HTTPS origin without credentials, path, query, or fragment."
  }
}

variable "clinical_evidence_service_url" {
  description = "Authenticated production Clinical Evidence endpoint used by the synthetic hosted application."
  type        = string
  default     = null
  nullable    = true

  validation {
    condition = var.clinical_evidence_service_url == null || can(regex(
      "^https://[a-z0-9-]+\\.a\\.run\\.app$",
      var.clinical_evidence_service_url,
    ))
    error_message = "clinical_evidence_service_url must be an HTTPS Cloud Run service URL."
  }
}

variable "application_service_enabled" {
  description = "Creates the private hosted Coda application only after provider-secret and ingress approval."
  type        = bool
  default     = false

  validation {
    condition = !var.application_service_enabled || (
      var.application_image != null && var.application_browser_origin != null && var.clinical_evidence_service_url != null
    )
    error_message = "application_service_enabled requires an immutable application_image, exact browser origin, and the production Clinical Evidence service URL."
  }
}

variable "migration_image" {
  description = "Immutable Artifact Registry application image used only by the run-to-completion database migration job."
  type        = string
  default     = null
  nullable    = true

  validation {
    condition = var.migration_image == null || can(regex(
      "^us-east1-docker\\.pkg\\.dev/coda-health-staging/coda-agents/migration@sha256:[0-9a-f]{64}$",
      var.migration_image,
    ))
    error_message = "migration_image must be the exact staging Artifact Registry sha256 digest."
  }
}

variable "proof_window_active" {
  description = "Keeps the hosted application resident only during an approved proof window."
  type        = bool
  default     = false
}
