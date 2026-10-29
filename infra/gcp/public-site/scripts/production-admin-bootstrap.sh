#!/usr/bin/env bash
# One-time administrator bootstrap for public-site production.
#
# Run by an administrator of coda-health-prod (and, for one cross-project grant,
# of coda-health-staging), with Dev's authorization. It prepares only what
# infra/gcp/public-site deliberately does not own: APIs, the private site image
# repository, the Doppler prd token secret, a CI promotion service account with
# resource-scoped roles, and GitHub Workload Identity Federation for the
# `production` environment. Production builds nothing: promotion copies the
# digest staging tested, so there are no base-image repositories here.
#
# It never applies OpenTofu, never prints or stores a secret value, never
# creates a bucket, and never reads or writes state outside
# coda-rust/public-site/production. Every create step checks first and is safe
# to rerun; rerun it after the first apply to add the bindings that need the
# Cloud Run service and runtime service account to exist.
#
#   bash infra/gcp/public-site/scripts/production-admin-bootstrap.sh --print
#   bash infra/gcp/public-site/scripts/production-admin-bootstrap.sh
#
# Flags:
#   --print                 Print the commands without calling gcloud or Doppler.
#   --skip-doppler-token    Create the secret but do not add a version (no Doppler login).
#   --rotate-doppler-token  Add a new token version even if one is enabled; bump
#                           doppler_token_secret_version in env/production.tfvars after.
#   -h, --help              Show this header.

set -euo pipefail

# --- Inputs (non-secret). Fixed: this script prepares production only. -----------
readonly PROJECT_ID="coda-health-prod"
readonly PROJECT_NUMBER="720954567292"
readonly REGION="us-east1"

# Existing shared state bucket; this root writes only under its own prefix.
readonly STATE_BUCKET="coda-health-prod-terraform-state-us-east1"
readonly STATE_PREFIX="coda-rust/public-site/production"

readonly SERVICE_NAME="public-site-production"
readonly RUNTIME_SA="${SERVICE_NAME}@${PROJECT_ID}.iam.gserviceaccount.com"

readonly SITE_REPOSITORY="public-site" # standard, immutable tags
SITE_KEEP_COUNT="${SITE_KEEP_COUNT:-10}"
# Promotion copies from the repository the staging deploy publishes to.
readonly STAGING_PROJECT_ID="coda-health-staging"
readonly STAGING_REPOSITORY="public-site"

readonly SECRET_ID="public-site-prd-doppler-token"
# Fixed on purpose: this secret must only ever hold a read-only prd token.
readonly TOKEN_DOPPLER_PROJECT="coda-rust"
readonly TOKEN_DOPPLER_CONFIG="prd"
readonly TOKEN_NAME="public-site-production-runtime"

readonly CI_SA_ID="public-site-prod-deployer"
readonly CI_SA="${CI_SA_ID}@${PROJECT_ID}.iam.gserviceaccount.com"

readonly WIF_POOL="github-actions"
readonly WIF_PROVIDER="github-oidc"
readonly WIF_REPOSITORY="Coda-Health/coda-rust"
readonly WIF_REPOSITORY_ID="1412706552"
readonly WIF_OWNER_ID="306538119"
readonly WIF_REF="refs/heads/main"
readonly WIF_ENVIRONMENT="production"

APIS=(
  serviceusage.googleapis.com
  cloudresourcemanager.googleapis.com
  iam.googleapis.com
  iamcredentials.googleapis.com
  sts.googleapis.com
  run.googleapis.com
  compute.googleapis.com
  certificatemanager.googleapis.com
  artifactregistry.googleapis.com
  secretmanager.googleapis.com
)

# --- Flags --------------------------------------------------------------------
PRINT=0
SKIP_TOKEN=0
ROTATE_TOKEN=0
for arg in "$@"; do
  case "$arg" in
    --print | --dry-run) PRINT=1 ;;
    --skip-doppler-token) SKIP_TOKEN=1 ;;
    --rotate-doppler-token) ROTATE_TOKEN=1 ;;
    -h | --help)
      sed -n '2,/^set -euo/p' "$0" | sed '$d' | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *)
      echo "unknown flag: $arg (see --help)" >&2
      exit 2
      ;;
  esac
done

export CLOUDSDK_CORE_DISABLE_PROMPTS=1
G=(gcloud --project="$PROJECT_ID")

step() { printf '\n==> %s\n' "$*"; }
note() { printf '    %s\n' "$*"; }
# Prints a shell-quoted command, then runs it unless --print.
run() {
  printf '+'
  printf ' %q' "$@"
  printf '\n'
  if [[ $PRINT -eq 0 ]]; then "$@"; fi
}
# True in --print mode (so creation commands are shown) or when the probe fails.
missing() {
  if [[ $PRINT -eq 1 ]]; then return 0; fi
  ! "$@" >/dev/null 2>&1
}
exists() {
  [[ $PRINT -eq 0 ]] && "$@" >/dev/null 2>&1
}
# Fails if the repository grants allUsers or allAuthenticatedUsers.
assert_repo_private() {
  local repo="$1" members
  printf '+ %s artifacts repositories get-iam-policy %q --location=%q --flatten=bindings[].members --format=value(bindings.members)\n' \
    "${G[*]}" "$repo" "$REGION"
  if [[ $PRINT -eq 1 ]]; then
    note "(refuses to continue if ${repo} lists allUsers or allAuthenticatedUsers)"
    return 0
  fi
  members="$("${G[@]}" artifacts repositories get-iam-policy "$repo" --location="$REGION" \
    --flatten='bindings[].members' --format='value(bindings.members)')"
  if grep -qxE 'allUsers|allAuthenticatedUsers' <<<"$members"; then
    echo "REFUSING: ${repo} is public; remove the allUsers/allAuthenticatedUsers binding first." >&2
    exit 1
  fi
  note "${repo}: no allUsers / allAuthenticatedUsers bindings"
}

# --- 0. Preflight -------------------------------------------------------------
step "Preflight: ${PROJECT_ID} (${PROJECT_NUMBER}), region ${REGION}"
if [[ $PRINT -eq 0 ]]; then
  command -v gcloud >/dev/null || { echo "gcloud not on PATH; use nix develop" >&2; exit 1; }
  note "gcloud account: $(gcloud config get-value account 2>/dev/null)"
  actual_number="$("${G[@]}" projects describe "$PROJECT_ID" --format='value(projectNumber)')"
  if [[ "$actual_number" != "$PROJECT_NUMBER" ]]; then
    echo "project number mismatch: expected ${PROJECT_NUMBER}, got ${actual_number}" >&2
    exit 1
  fi
  ubla="$("${G[@]}" storage buckets describe "gs://${STATE_BUCKET}" --format='value(uniform_bucket_level_access)')"
  if [[ ! "$ubla" =~ ^[Tt]rue$ ]]; then
    echo "gs://${STATE_BUCKET} must use uniform bucket-level access for prefix-scoped grants" >&2
    exit 1
  fi
else
  note "--print: no gcloud or Doppler calls are made; every create step is shown."
fi

# --- 1. APIs ------------------------------------------------------------------
step "Enable required APIs (idempotent)"
run "${G[@]}" services enable "${APIS[@]}"

# --- 2. Private site image repository -----------------------------------------
step "Create the private ${SITE_REPOSITORY} repository in ${REGION}"
if missing "${G[@]}" artifacts repositories describe "$SITE_REPOSITORY" --location="$REGION"; then
  run "${G[@]}" artifacts repositories create "$SITE_REPOSITORY" --location="$REGION" \
    --repository-format=docker --mode=standard-repository --immutable-tags \
    --description="public-site images promoted from staging, digest-pinned and immutable" \
    --labels=app=public-site,env=production
else
  note "${SITE_REPOSITORY} exists"
fi
run "${G[@]}" artifacts repositories update "$SITE_REPOSITORY" --location="$REGION" --immutable-tags
CLEANUP_POLICY="$(mktemp)"
trap 'rm -f "$CLEANUP_POLICY"' EXIT
cat >"$CLEANUP_POLICY" <<JSON
[
  {"name": "delete-untagged-after-seven-days", "action": {"type": "Delete"},
   "condition": {"tagState": "untagged", "olderThan": "7d"}},
  {"name": "keep-recent-${SITE_KEEP_COUNT}", "action": {"type": "Keep"},
   "mostRecentVersions": {"keepCount": ${SITE_KEEP_COUNT}}}
]
JSON
run "${G[@]}" artifacts repositories set-cleanup-policies "$SITE_REPOSITORY" \
  --location="$REGION" --policy="$CLEANUP_POLICY"
assert_repo_private "$SITE_REPOSITORY"
note "Same-project Cloud Run pulls use the Cloud Run service agent (roles/run.serviceAgent)."

# --- 3. Doppler token secret --------------------------------------------------
step "Create Secret Manager secret ${SECRET_ID} (user-managed, ${REGION})"
if missing "${G[@]}" secrets describe "$SECRET_ID"; then
  run "${G[@]}" secrets create "$SECRET_ID" \
    --replication-policy=user-managed --locations="$REGION" \
    --labels=app=public-site,env=production
else
  note "exists"
fi

step "Add a read-only Doppler ${TOKEN_DOPPLER_PROJECT}/${TOKEN_DOPPLER_CONFIG} service token as a version"
enabled_version=""
if [[ $PRINT -eq 0 && $SKIP_TOKEN -eq 0 && $ROTATE_TOKEN -eq 0 ]]; then
  enabled_version="$("${G[@]}" secrets versions list "$SECRET_ID" \
    --filter='state=ENABLED' --format='value(name)' --limit=1 2>/dev/null || true)"
fi
if [[ $SKIP_TOKEN -eq 1 ]]; then
  note "skipped (--skip-doppler-token)"
elif [[ -n "$enabled_version" ]]; then
  note "enabled version ${enabled_version} already exists; pass --rotate-doppler-token to add another"
else
  # Strip the trailing newline Doppler prints; the token is never echoed,
  # assigned to a variable, or written to disk.
  printf '+ doppler configs tokens create %q --project %q --config %q --access read --plain \\\n' \
    "$TOKEN_NAME" "$TOKEN_DOPPLER_PROJECT" "$TOKEN_DOPPLER_CONFIG"
  printf '    | tr -d "\\r\\n" | gcloud --project=%q secrets versions add %q --data-file=-\n' "$PROJECT_ID" "$SECRET_ID"
  if [[ $PRINT -eq 0 ]]; then
    command -v doppler >/dev/null || { echo "doppler not on PATH; use nix develop" >&2; exit 1; }
    doppler configs tokens create "$TOKEN_NAME" \
      --project "$TOKEN_DOPPLER_PROJECT" --config "$TOKEN_DOPPLER_CONFIG" --access read --plain \
      | tr -d '\r\n' \
      | "${G[@]}" secrets versions add "$SECRET_ID" --data-file=- --format='value(name)'
  fi
  note "Pin doppler_token_secret_version in env/production.tfvars to the version printed above."
fi

# --- 4. CI promotion service account (no keys) --------------------------------
step "Create CI promotion service account ${CI_SA}"
if missing "${G[@]}" iam service-accounts describe "$CI_SA"; then
  run "${G[@]}" iam service-accounts create "$CI_SA_ID" \
    --display-name="public-site production promotion" \
    --description="GitHub Actions (WIF) promotion for infra/gcp/public-site production. No keys."
else
  note "exists"
fi

step "Resource-scoped roles for the CI promotion account (before first apply)"
# State: objects under this root's prefix only. Listing is a bucket-level call,
# so it is allowed only for list requests restricted to the same prefix.
STATE_CONDITION="^;^title=public-site-production-state;description=Only ${STATE_PREFIX}/ in ${STATE_BUCKET};expression=resource.name.startsWith('projects/_/buckets/${STATE_BUCKET}/objects/${STATE_PREFIX}/') || api.getAttribute('storage.googleapis.com/objectListPrefix', '').startsWith('${STATE_PREFIX}/')"
run "${G[@]}" storage buckets add-iam-policy-binding "gs://${STATE_BUCKET}" \
  --member="serviceAccount:${CI_SA}" --role=roles/storage.objectAdmin --condition="$STATE_CONDITION"
# Publish promoted images to the production site repository only.
run "${G[@]}" artifacts repositories add-iam-policy-binding "$SITE_REPOSITORY" \
  --location="$REGION" --member="serviceAccount:${CI_SA}" \
  --role=roles/artifactregistry.writer --condition=None
# Read the digest staging tested. Needs an administrator of coda-health-staging.
run gcloud --project="$STAGING_PROJECT_ID" artifacts repositories add-iam-policy-binding "$STAGING_REPOSITORY" \
  --location="$REGION" --member="serviceAccount:${CI_SA}" \
  --role=roles/artifactregistry.reader --condition=None
# The root manages the runtime SA's accessor binding on this secret.
run "${G[@]}" secrets add-iam-policy-binding "$SECRET_ID" \
  --member="serviceAccount:${CI_SA}" --role=roles/secretmanager.admin --condition=None
# Read-only project access so a promotion plan can refresh the Cloud Run
# service, load balancer and certificate it may not change.
for role in roles/run.viewer roles/compute.viewer roles/certificatemanager.viewer; do
  run "${G[@]}" projects add-iam-policy-binding "$PROJECT_ID" \
    --member="serviceAccount:${CI_SA}" --role="$role" --condition=None
done

step "Service-scoped roles for the CI promotion account (need the first apply)"
# The first apply is an administrator's, from a reviewed saved plan. Afterwards
# promotion only updates the existing Cloud Run service in place.
if [[ $PRINT -eq 1 ]] || exists "${G[@]}" iam service-accounts describe "$RUNTIME_SA"; then
  run "${G[@]}" iam service-accounts add-iam-policy-binding "$RUNTIME_SA" \
    --member="serviceAccount:${CI_SA}" --role=roles/iam.serviceAccountUser --condition=None
  run "${G[@]}" artifacts repositories add-iam-policy-binding "$SITE_REPOSITORY" \
    --location="$REGION" --member="serviceAccount:${RUNTIME_SA}" \
    --role=roles/artifactregistry.reader --condition=None
else
  note "runtime SA ${RUNTIME_SA} does not exist yet; rerun this script after the first apply"
fi
if [[ $PRINT -eq 1 ]] || exists "${G[@]}" run services describe "$SERVICE_NAME" --region="$REGION"; then
  run "${G[@]}" run services add-iam-policy-binding "$SERVICE_NAME" --region="$REGION" \
    --member="serviceAccount:${CI_SA}" --role=roles/run.admin --condition=None
else
  note "Cloud Run service ${SERVICE_NAME} does not exist yet; rerun this script after the first apply"
fi
note "No compute, certificate or IAM admin roles: promotion cannot change the load balancer, TLS or identities."

# --- 5. GitHub Actions Workload Identity Federation ---------------------------
WIF_CONDITION="assertion.repository == '${WIF_REPOSITORY}' && assertion.repository_id == '${WIF_REPOSITORY_ID}' && assertion.repository_owner_id == '${WIF_OWNER_ID}' && assertion.ref == '${WIF_REF}' && assertion.environment == '${WIF_ENVIRONMENT}'"
WIF_MAPPING="google.subject=assertion.sub,attribute.repository=assertion.repository,attribute.repository_id=assertion.repository_id,attribute.repository_owner_id=assertion.repository_owner_id,attribute.ref=assertion.ref,attribute.environment=assertion.environment"
POOL_NAME="projects/${PROJECT_NUMBER}/locations/global/workloadIdentityPools/${WIF_POOL}"
# The repository uses GitHub's immutable subject claim with owner and repository IDs.
WIF_SUBJECT="repo:${WIF_REPOSITORY%%/*}@${WIF_OWNER_ID}/${WIF_REPOSITORY#*/}@${WIF_REPOSITORY_ID}:environment:${WIF_ENVIRONMENT}"

step "Create WIF pool ${WIF_POOL} and GitHub OIDC provider ${WIF_PROVIDER}"
if missing "${G[@]}" iam workload-identity-pools describe "$WIF_POOL" --location=global; then
  run "${G[@]}" iam workload-identity-pools create "$WIF_POOL" --location=global \
    --display-name="GitHub Actions" --description="GitHub Actions OIDC for ${WIF_REPOSITORY}"
else
  note "pool exists"
fi
if missing "${G[@]}" iam workload-identity-pools providers describe "$WIF_PROVIDER" \
  --location=global --workload-identity-pool="$WIF_POOL"; then
  run "${G[@]}" iam workload-identity-pools providers create-oidc "$WIF_PROVIDER" \
    --location=global --workload-identity-pool="$WIF_POOL" \
    --display-name="GitHub ${WIF_REPOSITORY}" \
    --issuer-uri="https://token.actions.githubusercontent.com" \
    --attribute-mapping="$WIF_MAPPING" --attribute-condition="$WIF_CONDITION"
else
  # Reassert the restriction so a hand-edited provider cannot stay wider.
  run "${G[@]}" iam workload-identity-pools providers update-oidc "$WIF_PROVIDER" \
    --location=global --workload-identity-pool="$WIF_POOL" \
    --attribute-mapping="$WIF_MAPPING" --attribute-condition="$WIF_CONDITION"
fi

step "Allow only ${WIF_SUBJECT} to impersonate ${CI_SA}"
run "${G[@]}" iam service-accounts add-iam-policy-binding "$CI_SA" \
  --role=roles/iam.workloadIdentityUser --condition=None \
  --member="principal://iam.googleapis.com/${POOL_NAME}/subject/${WIF_SUBJECT}"
note "Workflow auth: workload_identity_provider=${POOL_NAME}/providers/${WIF_PROVIDER}"
note "             service_account=${CI_SA} (job must use environment: ${WIF_ENVIRONMENT} on ${WIF_REF})"
note "In GitHub, give the ${WIF_ENVIRONMENT} environment Dev as a required reviewer."

step "Done. Next: the first apply from a reviewed saved plan (book: Production state and deployment)."
