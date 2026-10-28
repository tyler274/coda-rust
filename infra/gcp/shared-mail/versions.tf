terraform {
  required_version = ">= 1.11.0, < 2.0.0"
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "= 7.24.0"
    }
  }
  backend "gcs" {
    bucket = "coda-health-prod-terraform-state-us-east1"
    prefix = "coda-rust/shared-mail/production"
  }
}

provider "google" {
  project = "coda-health-prod"
  region  = "us-east1"
}
