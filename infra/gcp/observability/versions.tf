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
  # operator initializes with backend/staging.gcs.tfbackend, which keeps this
  # state in the Rust staging project's state bucket beside rust-environments.
  backend "gcs" {}
}

provider "google" {
  project               = var.project_id
  region                = var.region
  billing_project       = var.project_id
  user_project_override = true
}

# The observability stack lands on the staging observability Autopilot
# cluster that infra/gcp/rust-environments owns (observability_cluster_enabled).
# This root reads it through variables and never manages the cluster or its
# network.
data "google_container_cluster" "runtime" {
  project  = var.cluster_project
  name     = var.cluster_name
  location = var.cluster_location
}

data "google_client_config" "current" {}

# The cluster's IP endpoint is private. Its DNS endpoint is reachable from an
# operator workstation and authorizes by IAM alone, and it serves a publicly
# trusted certificate, so no cluster CA is passed when it is used.
locals {
  cluster_dns_endpoint = try(data.google_container_cluster.runtime.control_plane_endpoints_config[0].dns_endpoint_config[0].endpoint, "")
  cluster_host         = local.cluster_dns_endpoint != "" ? "https://${local.cluster_dns_endpoint}" : "https://${data.google_container_cluster.runtime.endpoint}"
  cluster_ca           = local.cluster_dns_endpoint != "" ? null : base64decode(data.google_container_cluster.runtime.master_auth[0].cluster_ca_certificate)
}

provider "kubernetes" {
  host = local.cluster_host

  token                  = data.google_client_config.current.access_token
  cluster_ca_certificate = local.cluster_ca
}

provider "helm" {
  kubernetes = {
    host                   = local.cluster_host
    token                  = data.google_client_config.current.access_token
    cluster_ca_certificate = local.cluster_ca
  }
}

# Operator management keys come from the Doppler environment, never variables
# or state. Runtime ingestion receives only DD_API_KEY, not DD_APP_KEY.
# Under telemetry_backend = grafana no Datadog resource exists and the
# provider skips credential validation, so DD_API_KEY / DD_APP_KEY need not
# be set (provider 4.26: api_key and app_key are required only while validate
# is true).
provider "datadog" {
  api_url  = "https://api.${var.datadog_site}/"
  validate = local.datadog_backend ? "true" : "false"
}
