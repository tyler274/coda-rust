# Nonsecret bootstrap inputs for the existing isolated Rust staging project.
# Database and workload activation require their separate readiness work.
environment = "stg"
projects = {
  stg = {
    project_id        = "coda-rust-staging"
    state_bucket_name = "coda-rust-staging-tofu-state-us-east1"
  }
}
organization_id      = "301140270943"
billing_account      = "01691B-49BADA-F3D54D"
region               = "us-east1"
iap_group            = "gcp-engineering@codahealth.ai"
state_admin_members  = ["group:gcp-engineering@codahealth.ai"]
github_repository_id = "1412706552"
github_owner_id      = "306538119"

# The billing administrator owns the project budget outside this state.
enable_databases = false
enable_workloads = false
