# Non-secret public-site production inputs. No secret values belong here.
# Production serves staging's tested image without sign-in: no
# physician_app_url, so there is no Sign in link and no /physician mount.
# image and release_sha are not pinned here. The promotion job, and the
# operator's first apply, pass the digest staging tested and its commit.
project_id                   = "coda-health-prod"
project_number               = "720954567292"
region                       = "us-east1"
environment                  = "production"
domains                      = ["codahealth.ai", "www.codahealth.ai"]
doppler_token_secret_id      = "public-site-prd-doppler-token"
doppler_token_secret_version = "1"
