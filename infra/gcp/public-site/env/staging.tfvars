# Non-secret public-site staging inputs. No secret values belong here.
# image: the 36791a9 (main) build, pushed by digest to the private public-site
# repository.
project_id     = "coda-health-staging"
project_number = "134570912572"
region         = "us-east1"
environment    = "staging"
staging_domain = "staging.codahealth.ai"
iap_group      = "gcp-engineering@dabney.moe"
image          = "us-east1-docker.pkg.dev/coda-health-staging/public-site/public-site@sha256:7eb037bbe941cf4bea25b83c7a42c233844092bee924a042dc7a7032849e1b28"
release_sha    = "36791a921325e176061c38677475d4921fb7cb64"
# Physician preview lives in coda-rust-staging (836508863523), behind its own IAP.
# Fixed /physician upstream, not the marketing origin or the POST /auth/sign-in API.
# Before release: operator bootstraps self-signJwt and applies physician's
# authoritative IAP allowlist. CI reads this file; it does not provision that IAM.
physician_app_url = "https://physician-836508863523.us-east1.run.app"
# /grafana/ goes to the observability relay in coda-rust-staging, behind its
# own IAP (rust-environments observability-relay.tf).
observability_relay_url = "https://observability-relay-836508863523.us-east1.run.app"
