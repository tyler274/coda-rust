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

# The same public-site principal reaches /grafana/ and /langfuse/ through the
# observability relay (observability-relay.tf); it gets no other access.
observability_proxy_invokers = ["serviceAccount:public-site-staging@coda-health-staging.iam.gserviceaccount.com"]

# Staging-preview workloads remain behind direct IAP. SQL grants were
# installed with bootstrap-grants.sql; DB credentials live only in the four
# per-role Doppler configs below. Physician sign-in is still the proof-token
# preview and Duet's Identity Platform API key is a placeholder (ENG-167).
enable_workloads                       = true
runtime_and_database_contract_approved = true
# Approved staging relay networking; preserve the Workspace-allowlisted IP.
enable_physician_smtp_relay = true

# Selected sender-split image for staging qualification only.
# Publish and verify this exact digest before applying; desired configuration
# does not establish deployment or delivery. Production retains its selected image.
mail_relay_image    = "us-east1-docker.pkg.dev/coda-rust-staging/apps/mail-relay@sha256:4179edf841befcfa33b5dcdb0e1efe0bf20d1dce184fe9359efcb9eef71c6ea4"
mail_relay_invokers = ["serviceAccount:workspace-web-staging@coda-gym.iam.gserviceaccount.com"]

# Physician scaling (book/src/gcp.md#physician-scaling-redis-fan-out-and-pubsub-chat-turns):
# Memorystore Basic 1 GB (about $35/month, always billed), the Pub/Sub chat-turn
# queue, the internal physician-worker, and one always-on physician site
# instance with allocated CPU (about $53/month) that runs the recovery sweep.
# images.physician.worker is the initial digest. Later releases own that field.
enable_physician_scale = true

# Private GKE Autopilot cluster for the OpenTelemetry gateway, Datadog Agent and
# Langfuse (observability-cluster.tf); infra/gcp/observability deploys into it.
observability_cluster_enabled = true

# The observability relay (observability-relay.tf). Set to the first published
# coda-observability-relay digest; later releases own the image field.
observability_relay_image = "us-east1-docker.pkg.dev/coda-rust-staging/apps/coda-observability-relay@sha256:0f08de9f8a389e3c7533c20f57286d780d186eeacfd00030bad2471a5820037b"

# The otel-collector sidecar on physician, duet and physician-worker, sending to
# the gateway above. Needs the observability root applied and its gateway ready.
observability_enabled = true
# Both destinations behind the gateway; must match infra/gcp/observability.
telemetry_backend = "both"
# Prompt and completion text on generation spans. The gateway strips it from
# the Datadog and Grafana pipelines and sends it only to the in-project
# Langfuse; must match infra/gcp/observability, which requires Langfuse on.
telemetry_content = "full"

# The authenticated availability probe Cloud Run Job (availability.tf), from
# the published image, runs every minute. The direct Cloud Monitoring
# watchdog notifies Slack #engineering-observability through the existing
# "GCP Monitoring" channel when its heartbeat stops.
availability_probe_image            = "us-east1-docker.pkg.dev/coda-rust-staging/apps/coda-availability-probe@sha256:f115c044a68556f31800488f8e5cbfebcf02ee73c20727733750067fb78cc458"
availability_probe_schedule_enabled = true
availability_watchdog_enabled       = true
availability_watchdog_channels      = ["projects/coda-rust-staging/notificationChannels/12022100174098334389"]

# Rehearsal chats (observability-drill langfuse) run in this synthetic practice;
# their generations are labelled simulation in Langfuse. Real staging accounts
# stay in roger-synthetic-practice and stay real.
physician_simulation_practices = ["coda-rehearsal-practice"]

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
    PHYSICIAN_STAGING_ACCOUNTS = "dev@codahealth.ai|Dev|admin,tyler@codahealth.ai|Tyler|admin,physician@codahealth.test|Staging Physician|admin,ops@codahealth.ai|Ops|admin,eli@codahealth.ai|Eli|admin"
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
