# Nonsecret staging inputs. The gateway address and caller identities are
# not repeated here: they come from infra/gcp/rust-environments outputs as a
# second -var-file (see README.md, "Inputs from rust-environments").
environment      = "staging"
project_id       = "coda-rust-staging"
cluster_project  = "coda-rust-staging"
cluster_location = "us-east1"
cluster_name     = "rust-observability"
network_name     = "rust-private"
region           = "us-east1"

# Datadog-only configuration. Enabling Langfuse requires its Cloud SQL IAM
# grants and Doppler secrets; the gateway drops generation copies while off.
langfuse_enabled = false

# Slack #engineering-observability through Datadog's Coda_Health Slack account.
# Monitor creation has a separate datadog_monitors_enabled authorization gate.
alert_notify = "@slack-Coda_Health-engineering-observability"
