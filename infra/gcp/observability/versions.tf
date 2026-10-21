terraform {
  required_version = ">= 1.11.0, < 2.0.0"

  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 7.43"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.38"
    }
    helm = {
      source  = "hashicorp/helm"
      version = "~> 3.1"
    }
    datadog = {
      source  = "DataDog/datadog"
      version = "~> 4.26"
    }
  }

  # Validation-only from this workspace: `tofu init -backend=false`. The
  # prefix is reserved so an operator who takes state ownership lands beside
  # the other coda-rust roots.
  backend "gcs" {
    bucket = "coda-health-prod-terraform-state-us-east1"
    prefix = "coda-rust/observability"
  }
}

provider "google" {
  project               = var.project_id
  region                = var.region
  billing_project       = var.project_id
  user_project_override = true
}

# The observability stack lands on the physician runtime's existing Autopilot
# cluster; this root reads it and never manages it.
data "google_container_cluster" "runtime" {
  project  = var.project_id
  name     = var.cluster_name
  location = var.region
}

data "google_client_config" "current" {}

provider "kubernetes" {
  host = "https://${data.google_container_cluster.runtime.endpoint}"

  token                  = data.google_client_config.current.access_token
  cluster_ca_certificate = base64decode(data.google_container_cluster.runtime.master_auth[0].cluster_ca_certificate)
}

provider "helm" {
  kubernetes = {
    host                   = "https://${data.google_container_cluster.runtime.endpoint}"
    token                  = data.google_client_config.current.access_token
    cluster_ca_certificate = base64decode(data.google_container_cluster.runtime.master_auth[0].cluster_ca_certificate)
  }
}

# Keys come from the environment (`doppler run --project coda-rust --config
# prd -- tofu plan` exports DD_API_KEY and DD_APP_KEY); none live here.
provider "datadog" {
  api_url = "https://api.${var.datadog_site}/"
}
