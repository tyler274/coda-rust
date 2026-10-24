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
# Staging-preview: two shared-core instances (~$10/month each, always billed).
enable_databases = true
sql_tier         = "db-f1-micro"

# The public-site origin verifies its own staging IAP assertion before proxying.
# This principal gets physician IAP access only; Duet remains group-only.
# Operator approval/application is required before enabling the public proxy.
physician_proxy_invokers = ["serviceAccount:public-site-staging@coda-health-staging.iam.gserviceaccount.com"]

# Staging-preview workloads remain behind direct IAP. SQL grants were
# installed with bootstrap-grants.sql; DB credentials live only in the four
# per-role Doppler configs below. Physician sign-in is still the proof-token
# preview and Duet's Identity Platform API key is a placeholder (ENG-167).
enable_workloads                       = true
runtime_and_database_contract_approved = true
# Approved staging relay networking; preserve the Workspace-allowlisted IP.
enable_physician_smtp_relay = true

# Physician scaling (book/src/gcp.md#physician-scaling-redis-fan-out-and-pubsub-chat-turns):
# Memorystore Basic 1 GB (about $35/month, always billed), the Pub/Sub chat-turn
# queue, the internal physician-worker and always-allocated physician CPU.
# images.physician.worker is the initial digest. Later releases own that field.
enable_physician_scale = true

images = {
  # Initial digests only; later releases own the image field.
  physician = {
    runtime   = "us-east1-docker.pkg.dev/coda-rust-staging/apps/coda-physician-sample@sha256:042628a23dbb554af0368b7fad78166593384bcc695eeb410b69be7f7050f020"
    migration = "us-east1-docker.pkg.dev/coda-rust-staging/apps/coda-physician-migrate@sha256:af679d7edb9b20f04fc42397d3ac00d78c12c4d79a799d7e189094e094effac4"
    worker    = "us-east1-docker.pkg.dev/coda-rust-staging/apps/coda-physician-worker@sha256:b9e8fef897b13c1bd8b8440c4cfce8ed2cd333e21af8705d248b044dec20ab85"
  }
  duet = {
    runtime   = "us-east1-docker.pkg.dev/coda-rust-staging/apps/coda-duet-api@sha256:7d5aa30d062a10ec02dad6a06c6730d058a123437f6bf90529884e7fbfced6ed"
    migration = "us-east1-docker.pkg.dev/coda-rust-staging/apps/coda-duet-migrate@sha256:8624b69525ae479d2d75058f1df7fb4404bbcd60bb001f35a5ef443e38978823"
  }
}
doppler_configs = {
  physician-runtime   = "stg_physician_runtime"
  physician-migration = "stg_physician_migration"
  duet-runtime        = "stg_duet_runtime"
  duet-migration      = "stg_duet_migration"
}
doppler_token_versions = {
  physician = { runtime = "1", migration = "1" }
  duet      = { runtime = "1", migration = "1" }
}
# physician-migrate ensures these preview sign-in accounts exist in the
# synthetic practice on the staging physician's panel; it never reactivates or
# edits an existing account. Patient invitations do not create physician users.
migration_environment = {
  physician = {
    CODA_HOSTED_PROFILE        = "staging-preview"
    PHYSICIAN_STAGING_ACCOUNTS = "dev@codahealth.ai|Dev|admin,tyler@codahealth.ai|Tyler|admin,physician@codahealth.test|Staging Physician|admin"
  }
}
runtime_environment = {
  physician = {
    CODA_ENVIRONMENT    = "stg"
    CODA_HOSTED_PROFILE = "staging-preview"
    # The image runs physician chat in API mode, which refuses to start without
    # this allowance; the key comes from stg_physician_runtime. IAP limits
    # callers to iap_group and the explicitly allowlisted, IAP-gated site proxy.
    CODA_ALLOW_MODEL_CALLS = "1"
  }
  duet = {
    CODA_ENVIRONMENT             = "stg"
    CODA_HOSTED_PROFILE          = "staging-preview"
    IDENTITY_PLATFORM_PROJECT_ID = "coda-rust-staging"
    DUET_PUBLIC_ORIGIN           = "https://duet.staging.invalid"
    DUET_SHL_VIEWER_URL          = "https://duet.staging.invalid/shl"
  }
}
