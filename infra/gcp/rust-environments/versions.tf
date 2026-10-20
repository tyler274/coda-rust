terraform {
  required_version = ">= 1.11.0, < 2.0.0"
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "= 7.24.0"
    }
    google-beta = {
      source  = "hashicorp/google-beta"
      version = "= 7.24.0"
    }
  }
  # One separately authorized bucket per environment. Bootstrap with local state
  # using -backend=false, then the OPERATOR migrates it to the emitted backend.
  # Never use workspaces to multiplex these environments or reuse reference state.
  backend "gcs" {}
}

provider "google" {
  region = var.region
}

provider "google-beta" {
  region = var.region
}
