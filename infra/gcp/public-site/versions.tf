terraform {
  required_version = ">= 1.9.0, < 2.0.0"
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "= 7.43.0"
    }
  }
  # Partial GCS backend. Staging state is owned by this root:
  #   tofu init -backend-config=backend/staging.gcs.tfbackend
  # Mocked tests and validation use `tofu init -backend=false` and need no
  # credentials. Never apply with local state; never initialize production here.
  backend "gcs" {}
}

provider "google" {
  project = var.project_id
  region  = var.region
}
