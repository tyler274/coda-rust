# Non-secret public-site staging inputs. No secret values belong here.
# image: the 3609430 (main) build, pushed by digest to the private public-site
# repository; the earlier b18760e copy came from scripts/staging-admin-bootstrap.sh.
project_id     = "coda-health-staging"
project_number = "134570912572"
region         = "us-east1"
environment    = "staging"
staging_domain = "staging.codahealth.ai"
iap_group      = "gcp-engineering@codahealth.ai"
image          = "us-east1-docker.pkg.dev/coda-health-staging/public-site/public-site@sha256:5a8693ef4801ca73a661cb7c018f743495c86ba4b681ac6e4a34135709133e29"
release_sha    = "3609430a18194fd489bf39d39337b53cdd5fbfbd"
# Physician preview lives in coda-rust-staging (836508863523), behind its own IAP.
# Fixed /physician upstream, not the marketing origin or the POST /auth/sign-in API.
# Before release: operator bootstraps self-signJwt and applies physician's
# authoritative IAP allowlist. CI reads this file; it does not provision that IAM.
physician_app_url            = "https://physician-836508863523.us-east1.run.app"
physician_proxy_enabled      = true
doppler_token_secret_id      = "public-site-stg-doppler-token"
doppler_token_secret_version = "2"
