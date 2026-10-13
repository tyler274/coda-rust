variable "project_id" {
  description = "Isolated non-PHI project hosting the physician runtime."
  type        = string
  default     = "coda-health-staging"
}

variable "region" {
  description = "Pinned region for the physician runtime cluster and database."
  type        = string
  default     = "us-east1"
}

variable "network_cidr" {
  description = "Subnet the Autopilot pods use, with Private Service Connect carved out of it."
  type        = string
  default     = "10.36.0.0/22"
}

variable "private_services_prefix_length" {
  description = "Prefix length for the Cloud SQL Private Service Access range."
  type        = number
  default     = 24
}

variable "sql_tier" {
  description = "Cloud SQL machine tier for the physician database."
  type        = string
  default     = "db-custom-1-3840"
}

variable "sql_activation_policy" {
  description = "ALWAYS while the physician runtime serves, NEVER when it is parked."
  type        = string
  default     = "NEVER"

  validation {
    condition     = contains(["ALWAYS", "NEVER"], var.sql_activation_policy)
    error_message = "sql_activation_policy must be ALWAYS or NEVER."
  }
}

variable "sql_disk_autoresize_limit_gb" {
  description = "Hard Cloud SQL storage ceiling. Raise only after a measured growth review."
  type        = number
  default     = 50

  validation {
    condition     = var.sql_disk_autoresize_limit_gb >= 20
    error_message = "The storage ceiling must stay at or above the 20 GB initial disk."
  }
}

variable "physician_site_image" {
  description = "Immutable physician site image digest. Null leaves the workload undeployed."
  type        = string
  default     = null
  nullable    = true

  validation {
    condition = var.physician_site_image == null || can(regex(
      "^[a-z0-9-]+-docker\\.pkg\\.dev/[a-z0-9-]+/[a-z0-9-]+/physician-site@sha256:[0-9a-f]{64}$",
      var.physician_site_image,
    ))
    error_message = "physician_site_image must be an Artifact Registry sha256 digest for the physician-site repository."
  }
}

variable "physician_migration_image" {
  description = "Digest-pinned image for the run-to-completion database migration job."
  type        = string
  default     = null
  nullable    = true

  validation {
    condition = var.physician_migration_image == null || can(regex(
      "^[a-z0-9-]+-docker\\.pkg\\.dev/[a-z0-9-]+/[a-z0-9-]+/physician-site@sha256:[0-9a-f]{64}$",
      var.physician_migration_image,
    ))
    error_message = "physician_migration_image must be an Artifact Registry sha256 digest for the physician-site repository."
  }
}

variable "physician_site_enabled" {
  description = "Creates the private physician site workload only after the image, origin, and database are named."
  type        = bool
  default     = false

  validation {
    condition = !var.physician_site_enabled || (
      var.physician_site_image != null && var.physician_browser_origin != null
    )
    error_message = "physician_site_enabled requires an immutable physician_site_image and an exact physician_browser_origin."
  }
}

variable "physician_browser_origin" {
  description = "Exact HTTPS browser origin allowed to reach the physician site."
  type        = string
  default     = null
  nullable    = true

  validation {
    condition = var.physician_browser_origin == null || can(regex(
      "^https://[^/@?#]+$",
      var.physician_browser_origin,
    ))
    error_message = "physician_browser_origin must be an exact HTTPS origin without credentials, path, query, or fragment."
  }
}

variable "migration_enabled" {
  description = "Creates the database migration job only when a digest-pinned migration image is bound."
  type        = bool
  default     = false

  validation {
    condition     = !var.migration_enabled || var.physician_migration_image != null
    error_message = "migration_enabled requires an immutable physician_migration_image digest."
  }
}

variable "sign_in_delivery" {
  description = "How sign-in tokens reach physicians in this deployment: proof stores them in the response."
  type        = string
  default     = "proof"
}
