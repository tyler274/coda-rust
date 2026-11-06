variable "project_id" {
  description = "Existing project that owns the cache; this root creates no project or product grants."
  type        = string
  nullable    = false

  validation {
    condition     = can(regex("^[a-z][a-z0-9-]{4,28}[a-z0-9]$", var.project_id))
    error_message = "project_id must be a valid Google Cloud project ID."
  }
}

variable "region" {
  description = "Regional GCS location for the cache (for example us-central1)."
  type        = string
  nullable    = false

  validation {
    condition     = can(regex("^[a-z]+-[a-z]+[0-9]+$", var.region))
    error_message = "region must be a regional location, not a multi-region or zone."
  }
}

variable "bucket_name" {
  description = "Globally unique cache bucket name, distinct from the remote state bucket."
  type        = string
  nullable    = false

  validation {
    condition     = length(var.bucket_name) >= 3 && length(var.bucket_name) <= 63 && can(regex("^[a-z0-9][a-z0-9-]*[a-z0-9]$", var.bucket_name))
    error_message = "bucket_name must be 3–63 lowercase letters, digits or hyphens, beginning and ending with a letter or digit."
  }
}
