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
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.38"
    }
  }

  backend "gcs" {
    bucket = "coda-health-prod-terraform-state-us-east1"
    prefix = "coda-rust/physician-runtime"
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

provider "kubernetes" {
  host = "https://${google_container_cluster.physician.endpoint}"

  token                  = data.google_client_config.current.access_token
  cluster_ca_certificate = google_container_cluster.physician.master_auth[0].cluster_ca_certificate
}

data "google_client_config" "current" {}
