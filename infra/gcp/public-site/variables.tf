variable "project_id" {
  description = "Existing GCP project. API enablement and state ownership are operator prerequisites."
  type        = string
}

variable "region" {
  type    = string
  default = "us-east1"
}

variable "environment" {
  type    = string
  default = "staging"
  validation {
    condition     = contains(["staging", "production"], var.environment)
    error_message = "Only staging and production are supported; staging must enable the origin Access gate."
  }
}

variable "name" {
  type    = string
  default = "public-site"
  validation {
    condition     = can(regex("^[a-z][a-z0-9-]{0,18}[a-z0-9]$", var.name))
    error_message = "Use a 2–20 character lowercase resource name."
  }
}

variable "image" {
  description = "Published linux/amd64 image pinned by digest; never a mutable tag."
  type        = string
  validation {
    condition     = can(regex("^[^@]+@sha256:[0-9a-f]{64}$", var.image))
    error_message = "image must be an immutable sha256 image reference."
  }
}

variable "release_sha" {
  type = string
  validation {
    condition     = can(regex("^[0-9a-f]{40}$", var.release_sha))
    error_message = "release_sha must be a full Git commit SHA."
  }
}

variable "physician_app_url" {
  description = "Separate physician sign-in destination; no physician backend is provisioned here."
  type        = string
  validation {
    condition     = can(regex("^https://[^/]+", var.physician_app_url))
    error_message = "Deployed physician_app_url must use HTTPS."
  }
}

variable "certificate_ids" {
  description = "Existing global Compute SSL certificate self-links. Certificate issuance/DNS are outside this root."
  type        = list(string)
  validation {
    condition     = length(var.certificate_ids) > 0
    error_message = "An operator-provisioned TLS certificate is required."
  }
}

variable "doppler_token_secret_id" {
  description = "Existing Secret Manager secret containing only a read-only, environment-scoped DOPPLER_TOKEN; never its value."
  type        = string
}

variable "doppler_token_secret_version" {
  description = "Existing immutable numeric secret version. Rotation is an explicit revision change."
  type        = string
  validation {
    condition     = can(regex("^[1-9][0-9]*$", var.doppler_token_secret_version))
    error_message = "Pin a numeric DOPPLER_TOKEN secret version, not latest."
  }
}

variable "min_instances" {
  type    = number
  default = 0
  validation {
    condition     = var.min_instances >= 0 && floor(var.min_instances) == var.min_instances
    error_message = "min_instances must be a nonnegative integer."
  }
}

variable "max_instances" {
  type    = number
  default = 10
  validation {
    condition     = var.max_instances >= 1 && var.max_instances <= 100 && floor(var.max_instances) == var.max_instances
    error_message = "max_instances must be an integer between 1 and 100."
  }
}

variable "concurrency" {
  type    = number
  default = 1000
  validation {
    condition     = var.concurrency >= 1 && var.concurrency <= 1000 && floor(var.concurrency) == var.concurrency
    error_message = "Cloud Run concurrency must be an integer between 1 and 1000."
  }
}
