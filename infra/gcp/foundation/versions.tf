terraform {
  required_version = ">= 1.9.0, < 2.0.0"

  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 7.43"
    }
  }

  backend "gcs" {
    bucket = "coda-health-prod-terraform-state-us-east1"
    prefix = "coda-agents/foundation"
  }
}

provider "google" {
  project               = var.production_project_id
  region                = var.region
  billing_project       = var.production_project_id
  user_project_override = true
}
