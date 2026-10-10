terraform {
  required_version = ">= 1.9.0, < 2.0.0"

  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 7.43"
    }
    google-beta = {
      source  = "hashicorp/google-beta"
      version = "~> 7.43"
    }
  }

  backend "gcs" {
    bucket = "coda-health-prod-terraform-state-us-east1"
    prefix = "coda-agents/staging"
  }
}

provider "google" {
  project               = var.project_id
  region                = var.region
  billing_project       = var.project_id
  user_project_override = true
}

provider "google-beta" {
  project               = var.project_id
  region                = var.region
  billing_project       = var.project_id
  user_project_override = true
}
