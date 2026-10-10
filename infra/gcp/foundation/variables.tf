variable "organization_id" {
  description = "Google Cloud organization that owns the Coda product projects."
  type        = string
}

variable "billing_account_id" {
  description = "Billing account attached to the isolated staging project."
  type        = string
  sensitive   = true
}

variable "production_project_id" {
  description = "Existing project that owns the shared public clinical-corpus objects."
  type        = string
  default     = "coda-health-prod"
}

variable "staging_project_id" {
  description = "Isolated non-PHI hosted-proof project."
  type        = string
  default     = "coda-health-staging"
}

variable "region" {
  description = "Pinned region for the first hosted proof."
  type        = string
  default     = "us-east1"
}

variable "state_bucket_name" {
  description = "Private GCS bucket containing Terraform state only."
  type        = string
  default     = "coda-health-prod-terraform-state-us-east1"
}

variable "clinical_corpus_bucket_name" {
  description = "Shared immutable public-literature object boundary."
  type        = string
  default     = "coda-health-prod-clinical-corpus-us-east1"
}

variable "staging_monthly_budget_usd" {
  description = "Alerting guardrail for the non-PHI staging project; it is not a spend cap."
  type        = number
  default     = 10
}
