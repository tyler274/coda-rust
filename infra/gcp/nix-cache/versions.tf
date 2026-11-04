terraform {
  required_version = ">= 1.9.0, < 2.0.0"
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "= 7.43.0"
    }
  }

  # Supply a separate state bucket and prefix with -backend-config at init.
  # Validation and mocked tests use init -backend=false; never apply local state.
  backend "gcs" {}
}

provider "google" {
  project = var.project_id
  region  = var.region
}
