terraform {
  required_version = ">= 1.9.0, < 2.0.0"
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "= 7.43.0"
    }
  }
  # Partial GCS backend, one state per environment:
  #   tofu init -backend-config=backend/staging.gcs.tfbackend
  #   tofu init -backend-config=backend/production.gcs.tfbackend
  # Mocked tests and validation use `tofu init -backend=false` and need no
  # credentials. Never apply with local state. Production applies only from a
  # reviewed saved plan with Dev's authorization.
  backend "gcs" {}
}

provider "google" {
  project = var.project_id
  region  = var.region
}
