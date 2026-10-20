terraform {
  required_version = ">= 1.9.0, < 2.0.0"
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "= 7.43.0"
    }
  }
  # Deliberately no backend. This new root is validation-only until an operator
  # approves state ownership and deployment; never apply with local state.
}

provider "google" {
  project = var.project_id
  region  = var.region
}
