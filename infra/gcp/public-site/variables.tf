variable "project_id" {
  description = "Existing GCP project. API enablement and state ownership are operator prerequisites."
  type        = string
}

variable "project_number" {
  description = "Numeric number of the existing project (coda-health-staging: 134570912572), not its project ID; used for the IAP service agent and signed assertion audience."
  type        = string
  nullable    = false
  validation {
    condition     = can(regex("^[1-9][0-9]*$", var.project_number))
    error_message = "project_number must be a positive numeric string for the existing project."
  }
}

variable "iap_group" {
  description = "Engineering Google Group email for service-scoped staging IAP access. Membership is operator-managed; no CI or smoke identity is implicitly granted access."
  type        = string
  default     = "gcp-engineering@codahealth.ai"
  nullable    = false
  validation {
    condition     = can(regex("^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}$", var.iap_group))
    error_message = "iap_group must be a Google Group email, not a domain, wildcard, or IAM principal prefix."
  }
}

variable "region" {
  type    = string
  default = "us-east1"
}

variable "staging_domain" {
  description = "Verified custom hostname for the IAP-protected staging service. DNS remains operator-managed; production does not use this mapping."
  type        = string
  default     = null
}

variable "environment" {
  type    = string
  default = "staging"
  validation {
    condition     = contains(["staging", "production"], var.environment)
    error_message = "Only staging and production are supported; staging must enable native IAP and origin assertion verification."
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
  description = "Published linux/amd64 image pinned by digest, from the private public-site Artifact Registry repository in this project and region (staging: us-east1-docker.pkg.dev/coda-health-staging/public-site/...)."
  type        = string
  validation {
    condition     = can(regex("^[^@]+@sha256:[0-9a-f]{64}$", var.image))
    error_message = "image must be an immutable sha256 image reference."
  }
  # The repository is created private by scripts/staging-admin-bootstrap.sh.
  # Same-project images let the Cloud Run service agent pull without any
  # cross-project grant, and no public registry or shared repository is used.
  validation {
    condition     = startswith(var.image, "${var.region}-docker.pkg.dev/${var.project_id}/public-site/") && can(regex("^[a-z0-9-]+-docker\\.pkg\\.dev/[a-z0-9-]+/public-site/[a-z0-9]+([._/-][a-z0-9]+)*(:[A-Za-z0-9_][A-Za-z0-9_.-]{0,127})?@sha256:[0-9a-f]{64}$", var.image))
    error_message = "image must come from the private Artifact Registry repository REGION-docker.pkg.dev/PROJECT_ID/public-site/ (staging: us-east1-docker.pkg.dev/coda-health-staging/public-site/), never a public registry or another repository."
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
  description = "Fixed physician HTTPS upstream origin for /physician. Set it only where the site offers physician sign-in: it mounts /physician, renders the Sign in link and gives the runtime its proxy signing identity, so operators provision runtime self-signJwt and physician-only IAP access first. Null means no Sign in link and no /physician mount. No physician backend or target IAM is provisioned here."
  type        = string
  default     = null
  validation {
    condition     = var.physician_app_url == null ? true : can(regex("^https://[a-z0-9]([a-z0-9-]*[a-z0-9])?(\\.[a-z0-9]([a-z0-9-]*[a-z0-9])?)+(:[0-9]{1,5})?/?$", var.physician_app_url))
    error_message = "Deployed physician_app_url must be a fixed HTTPS origin without credentials, path, query, or fragment."
  }
}

variable "observability_relay_url" {
  description = "Staging only: the observability relay's HTTPS origin (infra/gcp/rust-environments output observability_relay.url). It mounts /grafana/, which public-site proxies to the relay with the same signed service-account JWT as /physician, plus the caller's verified IAP assertion as X-Coda-IAP-Assertion. Operators add this site's service account to rust-environments' observability_proxy_invokers first. Null means the mount does not exist."
  type        = string
  default     = null
  validation {
    condition     = var.observability_relay_url == null ? true : can(regex("^https://[a-z0-9]([a-z0-9-]*[a-z0-9])?(\\.[a-z0-9]([a-z0-9-]*[a-z0-9])?)+(:[0-9]{1,5})?/?$", var.observability_relay_url))
    error_message = "observability_relay_url must be a fixed HTTPS origin without credentials, path, query, or fragment."
  }
  validation {
    condition     = var.observability_relay_url == null || var.environment == "staging"
    error_message = "observability_relay_url is staging-only."
  }
}

variable "domains" {
  description = "Production hostnames the load balancer serves (apex first). Certificate Manager issues one Google-managed certificate for them through DNS authorization, so it can be active before traffic moves; the operator adds the certificate_dns_records CNAMEs. Staging uses its domain mapping instead and sets none."
  type        = list(string)
  default     = []
  nullable    = false
  validation {
    condition     = alltrue([for domain in var.domains : can(regex("^([a-z0-9]([a-z0-9-]*[a-z0-9])?\\.)+[a-z]{2,}$", domain))])
    error_message = "domains must be lowercase hostnames without scheme, port, path or wildcard."
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
