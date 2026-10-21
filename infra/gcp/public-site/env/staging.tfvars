# Non-secret public-site staging inputs. No secret values belong here.
# image: the b18760e build, copied by digest from coda-agents into the private
# public-site repository by scripts/staging-admin-bootstrap.sh (which must run first).
project_id                   = "coda-health-staging"
project_number               = "134570912572"
region                       = "us-east1"
environment                  = "staging"
staging_domain               = "staging.codahealth.ai"
iap_group                    = "gcp-engineering@codahealth.ai"
image                        = "us-east1-docker.pkg.dev/coda-health-staging/public-site/public-site@sha256:a179ab017b9b47179b0f9e71e7237242b9d454d8e27d98b2eb671bbbfd263140"
release_sha                  = "b18760ef98a82e68aef29ac7a20bc48796794bf9"
physician_app_url            = "https://staging.codahealth.ai"
doppler_token_secret_id      = "public-site-stg-doppler-token"
doppler_token_secret_version = "2"
