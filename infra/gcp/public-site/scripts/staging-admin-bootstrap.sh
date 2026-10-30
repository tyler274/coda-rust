#!/usr/bin/env bash
# One-time administrator bootstrap for the public-site staging OpenTofu root.
#
# Run by an administrator of coda-health-staging, not by the deployer. It only
# prepares prerequisites that infra/gcp/public-site deliberately does not own:
# APIs, the dedicated staging state bucket, private Artifact Registry
# repositories (site images, a Docker Hub pull-through cache and a digest copy of
# the distroless runtime base), a CI deployer service account with
# resource-scoped roles, GitHub Workload Identity Federation, and
# (optionally) a time-bound grant for a human first apply. It refuses to continue
# if any of those repositories (or the legacy coda-agents repository, when
# readable) grants allUsers or allAuthenticatedUsers.
#
# It never applies OpenTofu, never prints or stores a secret value, and never
# touches coda-health-prod. Every create step checks first and is safe to rerun;
# rerun it after the first `tofu apply` to add the bindings that need the Cloud
# Run service and runtime service account to exist.
#
#   bash infra/gcp/public-site/scripts/staging-admin-bootstrap.sh --print
#   bash infra/gcp/public-site/scripts/staging-admin-bootstrap.sh
#   bash infra/gcp/public-site/scripts/staging-admin-bootstrap.sh --temporary-deployer-grant
#   bash infra/gcp/public-site/scripts/staging-admin-bootstrap.sh --check-group   # Workspace admin
#
# Flags:
#   --print                     Print the commands without calling gcloud.
#   --temporary-deployer-grant  OPTIONAL. Grant DEPLOYER_USER the first-apply roles,
#                               each expiring after GRANT_HOURS (IAM condition).
#   --physician-proxy           OPTIONAL approved staging /physician proxy: create a
#                               signJwt-only custom role and bind runtime to itself.
#                               Physician target IAP stays in rust-environments.
#   --check-group               Read-only Google Group lookup (needs a Workspace/Cloud
#                               Identity admin); runs only this step.
#   -h, --help                  Show this header.

set -euo pipefail

# --- Inputs (non-secret). Override through the environment if needed. ---------
PROJECT_ID="${PROJECT_ID:-coda-health-staging}"
PROJECT_NUMBER="${PROJECT_NUMBER:-134570912572}"
REGION="${REGION:-us-east1}"

STATE_BUCKET="${STATE_BUCKET:-coda-health-staging-tofu-state-us-east1}"

SERVICE_NAME="${SERVICE_NAME:-public-site-staging}"
RUNTIME_SA="${SERVICE_NAME}@${PROJECT_ID}.iam.gserviceaccount.com"
RUN_AGENT="service-${PROJECT_NUMBER}@serverless-robot-prod.iam.gserviceaccount.com"

# Private Artifact Registry repositories (all in PROJECT_ID / REGION).
AR_HOST="${REGION}-docker.pkg.dev"
SITE_REPOSITORY="${SITE_REPOSITORY:-public-site}"                # standard, immutable tags
DOCKERHUB_REPOSITORY="${DOCKERHUB_REPOSITORY:-public-site-dockerhub}" # remote, Docker Hub
BASE_REPOSITORY="${BASE_REPOSITORY:-public-site-base}"           # standard, immutable tags
LEGACY_REPOSITORY="${LEGACY_REPOSITORY:-coda-agents}"            # first push; checked, not changed
SITE_KEEP_COUNT="${SITE_KEEP_COUNT:-10}"
# Readers of the two base-image repositories (local image builds).
BUILDER_GROUP="${BUILDER_GROUP:-gcp-engineering@codahealth.ai}"

# Base images, by digest. Must match the FROM lines in crates/public-site/Dockerfile.
RUST_DIGEST="sha256:606fd313a0f49743ee2a7bd49a0914bab7deedb12791f3a846a34a4711db7ed2" # rust:1.95.0-alpine3.23
DISTROLESS_SOURCE="gcr.io/distroless/static-debian12"
DISTROLESS_DIGEST="sha256:afa5c872c891853ca7fcf1f12c3edb23f7eeef36189728842dd51042ff57f7ab" # :nonroot
DISTROLESS_TAG="nonroot-${DISTROLESS_DIGEST:7:12}"

# The image already pushed to the legacy repository (built from b18760e).
MIGRATE_IMAGE="${MIGRATE_IMAGE:-public-site}"
MIGRATE_DIGEST="${MIGRATE_DIGEST:-sha256:a179ab017b9b47179b0f9e71e7237242b9d454d8e27d98b2eb671bbbfd263140}"
MIGRATE_TAG="${MIGRATE_TAG:-b18760ef98a82e68aef29ac7a20bc48796794bf9}"


CI_SA_ID="${CI_SA_ID:-public-site-ci-deployer}"
CI_SA="${CI_SA_ID}@${PROJECT_ID}.iam.gserviceaccount.com"

WIF_POOL="${WIF_POOL:-github-actions}"
WIF_PROVIDER="${WIF_PROVIDER:-github-oidc}"
WIF_REPOSITORY="${WIF_REPOSITORY:-Coda-Health/coda-rust}"
WIF_REPOSITORY_ID="${WIF_REPOSITORY_ID:-1412706552}"
WIF_OWNER_ID="${WIF_OWNER_ID:-306538119}"
WIF_REF="${WIF_REF:-refs/heads/main}"
WIF_ENVIRONMENT="${WIF_ENVIRONMENT:-staging}"

IAP_GROUP="${IAP_GROUP:-gcp-engineering@codahealth.ai}"
DEPLOYER_USER="${DEPLOYER_USER:-tyler@codahealth.ai}"
GRANT_HOURS="${GRANT_HOURS:-24}"

APIS=(
  serviceusage.googleapis.com
  cloudresourcemanager.googleapis.com
  iam.googleapis.com
  iamcredentials.googleapis.com
  sts.googleapis.com
  run.googleapis.com
  iap.googleapis.com
  artifactregistry.googleapis.com
  storage.googleapis.com
)

# --- Flags --------------------------------------------------------------------
PRINT=0
TEMP_GRANT=0
PHYSICIAN_PROXY=0
CHECK_GROUP=0
for arg in "$@"; do
  case "$arg" in
    --print | --dry-run) PRINT=1 ;;
    --temporary-deployer-grant) TEMP_GRANT=1 ;;
    --check-group) CHECK_GROUP=1 ;;
    --physician-proxy) PHYSICIAN_PROXY=1 ;;
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
repo_missing() {
  missing "${G[@]}" artifacts repositories describe "$1" --location="$REGION"
}
# Fails if the repository grants allUsers or allAuthenticatedUsers. An
# unreadable policy fails too, unless the second argument is "optional".
assert_repo_private() {
  local repo="$1" mode="${2:-required}" members
  printf '+ %s artifacts repositories get-iam-policy %q --location=%q --flatten=bindings[].members --format=value(bindings.members)\n' \
    "${G[*]}" "$repo" "$REGION"
  if [[ $PRINT -eq 1 ]]; then
    note "(refuses to continue if ${repo} lists allUsers or allAuthenticatedUsers)"
    return 0
  fi
  if ! members="$("${G[@]}" artifacts repositories get-iam-policy "$repo" --location="$REGION" \
    --flatten='bindings[].members' --format='value(bindings.members)' 2>/dev/null)"; then
    if [[ "$mode" == optional ]]; then
      note "WARNING: cannot read the IAM policy of ${repo}; public access is unverified"
      return 0
    fi
    echo "cannot read the IAM policy of ${repo}; refusing to continue" >&2
    exit 1
  fi
  if grep -qxE 'allUsers|allAuthenticatedUsers' <<<"$members"; then
    echo "REFUSING: ${repo} grants $(grep -xE 'allUsers|allAuthenticatedUsers' <<<"$members" | sort -u | paste -sd, -)." >&2
    echo "Remove it: gcloud --project=${PROJECT_ID} artifacts repositories remove-iam-policy-binding ${repo} --location=${REGION} --member=MEMBER --role=ROLE" >&2
    exit 1
  fi
  note "${repo}: no allUsers / allAuthenticatedUsers bindings"
}
legacy_ci_writer() {
  "${G[@]}" artifacts repositories get-iam-policy "$LEGACY_REPOSITORY" --location="$REGION" \
    --flatten='bindings[].members' \
    --filter="bindings.role=roles/artifactregistry.writer AND bindings.members=serviceAccount:${CI_SA}" \
    --format='value(bindings.role)' 2>/dev/null | grep -q .
}
docker_digest_exists() {
  exists "${G[@]}" artifacts docker images describe "$1"
}

# --- Optional: Workspace admin group check (read-only) ------------------------
if [[ $CHECK_GROUP -eq 1 ]]; then
  step "Check the IAP group exists and whether ${DEPLOYER_USER} is a transitive member"
  note "Needs a Google Workspace / Cloud Identity admin (groups reader). Read-only."
  run "${G[@]}" identity groups describe "$IAP_GROUP"
  run "${G[@]}" identity groups memberships check-transitive-membership \
    --group-email="$IAP_GROUP" --member-email="$DEPLOYER_USER"
  exit 0
fi

# The proxy grant is separately approved staging-only bootstrap. Do not allow
# environment overrides to turn this convenience flag into production authority.
if [[ $PHYSICIAN_PROXY -eq 1 ]] && [[ "$PROJECT_ID" != coda-health-staging || "$PROJECT_NUMBER" != 134570912572 || "$SERVICE_NAME" != public-site-staging ]]; then
  echo "--physician-proxy is restricted to public-site-staging in coda-health-staging" >&2
  exit 1
fi

# --- 0. Preflight -------------------------------------------------------------
step "Preflight: ${PROJECT_ID} (${PROJECT_NUMBER}), region ${REGION}"
if [[ $PRINT -eq 0 ]]; then
  command -v gcloud >/dev/null || { echo "gcloud not on PATH; use nix develop" >&2; exit 1; }
  docker buildx version >/dev/null 2>&1 || { echo "docker with buildx is required for the image copy steps" >&2; exit 1; }
  note "gcloud account: $(gcloud config get-value account 2>/dev/null)"
  actual_number="$("${G[@]}" projects describe "$PROJECT_ID" --format='value(projectNumber)')"
  if [[ "$actual_number" != "$PROJECT_NUMBER" ]]; then
    echo "project number mismatch: expected ${PROJECT_NUMBER}, got ${actual_number}" >&2
    exit 1
  fi
else
  note "--print: no gcloud calls are made; every create step is shown."
fi

# --- 1. APIs ------------------------------------------------------------------
step "Enable required APIs (idempotent)"
run "${G[@]}" services enable "${APIS[@]}"
note "The IAP service agent service-${PROJECT_NUMBER}@gcp-sa-iap.iam.gserviceaccount.com already exists."

# --- 2. Dedicated staging state bucket ---------------------------------------
step "Create the staging OpenTofu state bucket gs://${STATE_BUCKET}"
if missing "${G[@]}" storage buckets describe "gs://${STATE_BUCKET}"; then
  run "${G[@]}" storage buckets create "gs://${STATE_BUCKET}" \
    --location="$REGION" --default-storage-class=STANDARD \
    --uniform-bucket-level-access --public-access-prevention
else
  note "exists"
fi
# Default soft-delete policy is kept. Versioning, UBLA and PAP are (re)asserted.
run "${G[@]}" storage buckets update "gs://${STATE_BUCKET}" \
  --versioning --uniform-bucket-level-access --public-access-prevention

# --- 3. Private Artifact Registry repositories --------------------------------
step "Create private Artifact Registry repositories in ${REGION} (Google-managed encryption)"
# Site images: CI writes, the runtime account reads. Tags can never move.
if repo_missing "$SITE_REPOSITORY"; then
  run "${G[@]}" artifacts repositories create "$SITE_REPOSITORY" --location="$REGION" \
    --repository-format=docker --mode=standard-repository --immutable-tags \
    --description="public-site images, digest-pinned and immutable" \
    --labels=app=public-site,env=staging
else
  note "${SITE_REPOSITORY} exists"
fi
run "${G[@]}" artifacts repositories update "$SITE_REPOSITORY" --location="$REGION" --immutable-tags
# Delete untagged pushes after 7 days, but always keep the newest N versions.
# Tagged images cannot be deleted at all in an immutable-tag repository.
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
note "cleanup policy file:"
sed 's/^/      /' "$CLEANUP_POLICY"
run "${G[@]}" artifacts repositories set-cleanup-policies "$SITE_REPOSITORY" \
  --location="$REGION" --policy="$CLEANUP_POLICY"

# Docker Hub pull-through cache for the Rust builder image (library/rust).
if repo_missing "$DOCKERHUB_REPOSITORY"; then
  run "${G[@]}" artifacts repositories create "$DOCKERHUB_REPOSITORY" --location="$REGION" \
    --repository-format=docker --mode=remote-repository --remote-docker-repo=docker-hub \
    --remote-repo-config-desc="Docker Hub (unauthenticated)" \
    --description="Pull-through cache of Docker Hub for public-site builds" \
    --labels=app=public-site,env=staging
else
  note "${DOCKERHUB_REPOSITORY} exists"
fi

# Distroless runtime base: a standard repository holding a digest-identical copy
# (step 7), so builds never contact gcr.io and the content cannot change.
if repo_missing "$BASE_REPOSITORY"; then
  run "${G[@]}" artifacts repositories create "$BASE_REPOSITORY" --location="$REGION" \
    --repository-format=docker --mode=standard-repository --immutable-tags \
    --description="Digest-pinned copies of public-site base images" \
    --labels=app=public-site,env=staging
else
  note "${BASE_REPOSITORY} exists"
fi
run "${G[@]}" artifacts repositories update "$BASE_REPOSITORY" --location="$REGION" --immutable-tags

step "Refuse public repositories (allUsers / allAuthenticatedUsers)"
for repo in "$SITE_REPOSITORY" "$DOCKERHUB_REPOSITORY" "$BASE_REPOSITORY"; do
  assert_repo_private "$repo"
done
assert_repo_private "$LEGACY_REPOSITORY" optional

step "Repository-scoped readers"
# Developers and CI pull the two base images; nobody else reads them.
for repo in "$DOCKERHUB_REPOSITORY" "$BASE_REPOSITORY"; do
  run "${G[@]}" artifacts repositories add-iam-policy-binding "$repo" --location="$REGION" \
    --member="group:${BUILDER_GROUP}" --role=roles/artifactregistry.reader --condition=None
done
# Same-project Cloud Run pulls use the Cloud Run service agent, whose
# project-level roles/run.serviceAgent includes downloadArtifacts. Grant it the
# repository reader only if that default grant was removed.
if [[ $PRINT -eq 1 ]]; then
  note "If ${RUN_AGENT} lacks roles/run.serviceAgent on the project, it gets"
  note "roles/artifactregistry.reader on ${SITE_REPOSITORY} only:"
  run "${G[@]}" artifacts repositories add-iam-policy-binding "$SITE_REPOSITORY" --location="$REGION" \
    --member="serviceAccount:${RUN_AGENT}" --role=roles/artifactregistry.reader --condition=None
elif "${G[@]}" projects get-iam-policy "$PROJECT_ID" --flatten='bindings[].members' \
  --filter="bindings.members=serviceAccount:${RUN_AGENT} AND bindings.role=roles/run.serviceAgent" \
  --format='value(bindings.role)' | grep -q .; then
  note "${RUN_AGENT} holds roles/run.serviceAgent; no repository grant needed"
else
  note "${RUN_AGENT} lacks roles/run.serviceAgent; granting repository reader instead"
  run "${G[@]}" artifacts repositories add-iam-policy-binding "$SITE_REPOSITORY" --location="$REGION" \
    --member="serviceAccount:${RUN_AGENT}" --role=roles/artifactregistry.reader --condition=None
fi

# --- 4. CI deployer service account (no keys) ----------------------------------
step "Create CI deployer service account ${CI_SA}"
if missing "${G[@]}" iam service-accounts describe "$CI_SA"; then
  run "${G[@]}" iam service-accounts create "$CI_SA_ID" \
    --display-name="public-site staging CI deployer" \
    --description="GitHub Actions (WIF) deployer for infra/gcp/public-site staging. No keys."
else
  note "exists"
fi

step "Resource-scoped roles for the CI deployer (before first apply)"
# State: read/write/lock objects in the staging state bucket only.
run "${G[@]}" storage buckets add-iam-policy-binding "gs://${STATE_BUCKET}" \
  --member="serviceAccount:${CI_SA}" --role=roles/storage.objectAdmin --condition=None
# Image publishing to the site repository only (writer includes reader).
run "${G[@]}" artifacts repositories add-iam-policy-binding "$SITE_REPOSITORY" \
  --location="$REGION" --member="serviceAccount:${CI_SA}" \
  --role=roles/artifactregistry.writer --condition=None
# Base images for the build: read only.
for repo in "$DOCKERHUB_REPOSITORY" "$BASE_REPOSITORY"; do
  run "${G[@]}" artifacts repositories add-iam-policy-binding "$repo" --location="$REGION" \
    --member="serviceAccount:${CI_SA}" --role=roles/artifactregistry.reader --condition=None
done
# CI never writes to the shared coda-agents repository. Remove a writer grant
# an earlier version of this script may have added there.
if [[ $PRINT -eq 1 ]] || legacy_ci_writer; then
  run "${G[@]}" artifacts repositories remove-iam-policy-binding "$LEGACY_REPOSITORY" \
    --location="$REGION" --member="serviceAccount:${CI_SA}" \
    --role=roles/artifactregistry.writer --all
else
  note "no CI writer grant found on ${LEGACY_REPOSITORY} (or its policy is unreadable)"
fi
# Read-only Cloud Run access at project level: the provider polls location-
# scoped Cloud Run operations, which a service-level grant does not cover.
run "${G[@]}" projects add-iam-policy-binding "$PROJECT_ID" \
  --member="serviceAccount:${CI_SA}" --role=roles/run.viewer --condition=None

step "Service-scoped roles for the CI deployer (need the first apply)"
# The CI deployer never creates the runtime SA or the service: the first apply
# is a human apply (admin, or the optional grant below). Afterwards CI needs only
# these resource-scoped roles. Tofu's authoritative bindings cover only
# run.invoker and iap.httpsResourceAccessor, so these grants are not removed.
if [[ $PRINT -eq 1 ]] || exists "${G[@]}" iam service-accounts describe "$RUNTIME_SA"; then
  run "${G[@]}" iam service-accounts add-iam-policy-binding "$RUNTIME_SA" \
    --member="serviceAccount:${CI_SA}" --role=roles/iam.serviceAccountUser --condition=None
  # Site images only. Cloud Run pulls through its service agent; this keeps the
  # runtime identity's read scope explicit and limited to the one repository.
  run "${G[@]}" artifacts repositories add-iam-policy-binding "$SITE_REPOSITORY" \
    --location="$REGION" --member="serviceAccount:${RUNTIME_SA}" \
    --role=roles/artifactregistry.reader --condition=None
else
  note "runtime SA ${RUNTIME_SA} does not exist yet; rerun this script after the first apply"
fi
if [[ $PRINT -eq 1 ]] || exists "${G[@]}" run services describe "$SERVICE_NAME" --region="$REGION"; then
  run "${G[@]}" run services add-iam-policy-binding "$SERVICE_NAME" --region="$REGION" \
    --member="serviceAccount:${CI_SA}" --role=roles/run.admin --condition=None
  # The service's IAP accessor policy (get/setIamPolicy); no narrower role exists.
  run "${G[@]}" iap web add-iam-policy-binding --resource-type=cloud-run \
    --service="$SERVICE_NAME" --region="$REGION" \
    --member="serviceAccount:${CI_SA}" --role=roles/iap.admin --condition=None
else
  note "Cloud Run service ${SERVICE_NAME} does not exist yet; rerun this script after the first apply"
fi
note "No run.invoker or iap.httpsResourceAccessor is granted: CI gets no website access."
note "No iam.serviceAccountAdmin: CI never creates or edits the runtime SA."

# Operator-owned IAM, not release-managed resources: the release root may only
# update the existing Cloud Run service. This role definition grants nothing at
# project scope. Its only binding is runtime -> itself on that service account.
# signJwt does not constrain audience: least privilege relies on the fixed
# application upstream and the separately scoped physician IAP binding.
if [[ $PHYSICIAN_PROXY -eq 1 ]]; then
  step "Approved staging proxy: runtime self-signJwt only"
  if [[ $PRINT -eq 0 ]] && ! exists "${G[@]}" iam service-accounts describe "$RUNTIME_SA"; then
    echo "runtime SA must exist before --physician-proxy; perform first apply separately" >&2
    exit 1
  fi
  PROXY_ROLE=publicSitePhysicianProxySigner
  if missing "${G[@]}" iam roles describe "$PROXY_ROLE"; then
    run "${G[@]}" iam roles create "$PROXY_ROLE" \
      --title="Public site physician proxy JWT signer" \
      --description="Runtime self-signJwt only; physician access is separately service-scoped" \
      --permissions=iam.serviceAccounts.signJwt --stage=GA
  else
    run "${G[@]}" iam roles update "$PROXY_ROLE" \
      --permissions=iam.serviceAccounts.signJwt --stage=GA
  fi
  run "${G[@]}" iam service-accounts add-iam-policy-binding "$RUNTIME_SA" \
    --member="serviceAccount:${RUNTIME_SA}" \
    --role="projects/${PROJECT_ID}/roles/${PROXY_ROLE}" --condition=None
  note "No tokenCreator, signBlob/getAccessToken permission, service-account keys or CI signing grant."
  note "signJwt is still identity-signing authority, not an audience-restricted permission; approve this boundary explicitly."
  note "The coda-rust-staging operator must apply physician_proxy_invokers through its authoritative physician IAP binding."
fi

# --- 5. GitHub Actions Workload Identity Federation ---------------------------
WIF_CONDITION="assertion.repository == '${WIF_REPOSITORY}' && assertion.repository_id == '${WIF_REPOSITORY_ID}' && assertion.repository_owner_id == '${WIF_OWNER_ID}' && assertion.ref == '${WIF_REF}' && assertion.environment == '${WIF_ENVIRONMENT}'"
WIF_MAPPING="google.subject=assertion.sub,attribute.repository=assertion.repository,attribute.repository_id=assertion.repository_id,attribute.repository_owner_id=assertion.repository_owner_id,attribute.ref=assertion.ref,attribute.environment=assertion.environment"
POOL_NAME="projects/${PROJECT_NUMBER}/locations/global/workloadIdentityPools/${WIF_POOL}"
# The repository opts into GitHub's immutable subject claim, which carries the
# owner and repository IDs: repo:Coda-Health@306538119/coda-rust@1412706552:...
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

# --- 6. Base images into the private repositories, by digest -------------------
# A carbon copy (imagetools create with one source and --prefer-index=false)
# keeps the manifest bytes, so the destination digest equals the source digest;
# the describe-by-digest check below proves it. `crane copy SRC DST` is an
# equivalent tool. Needs docker with buildx, authenticated for ${AR_HOST}.
DISTROLESS_DEST="${AR_HOST}/${PROJECT_ID}/${BASE_REPOSITORY}/distroless/static-debian12"
RUST_REMOTE="${AR_HOST}/${PROJECT_ID}/${DOCKERHUB_REPOSITORY}/library/rust"
SITE_DEST="${AR_HOST}/${PROJECT_ID}/${SITE_REPOSITORY}/${MIGRATE_IMAGE}"
LEGACY_SOURCE="${AR_HOST}/${PROJECT_ID}/${LEGACY_REPOSITORY}/${MIGRATE_IMAGE}@${MIGRATE_DIGEST}"
step "Authenticate docker to ${AR_HOST} (writes a credential helper entry, no secret)"
run gcloud auth configure-docker "$AR_HOST" --quiet

step "Copy ${DISTROLESS_SOURCE}:nonroot into ${BASE_REPOSITORY} by digest"
if [[ $PRINT -eq 1 ]] || ! docker_digest_exists "${DISTROLESS_DEST}@${DISTROLESS_DIGEST}"; then
  run docker buildx imagetools create --prefer-index=false \
    --tag "${DISTROLESS_DEST}:${DISTROLESS_TAG}" "${DISTROLESS_SOURCE}@${DISTROLESS_DIGEST}"
else
  note "already present"
fi
run "${G[@]}" artifacts docker images describe "${DISTROLESS_DEST}@${DISTROLESS_DIGEST}" \
  --format='value(image_summary.digest)'

step "Check the Docker Hub cache serves library/rust@${RUST_DIGEST:0:19}..."
# A digest reference either resolves to exactly this content or fails. Layers
# are cached on the first build that pulls them.
run docker buildx imagetools inspect "${RUST_REMOTE}@${RUST_DIGEST}"

# --- 7. Move the already-pushed site image into the site repository ------------
step "Copy ${LEGACY_SOURCE} into ${SITE_REPOSITORY}"
if [[ $PRINT -eq 1 ]] || ! docker_digest_exists "${SITE_DEST}@${MIGRATE_DIGEST}"; then
  run docker buildx imagetools create --prefer-index=false \
    --tag "${SITE_DEST}:${MIGRATE_TAG}" "$LEGACY_SOURCE"
else
  note "already present"
fi
run "${G[@]}" artifacts docker images describe "${SITE_DEST}@${MIGRATE_DIGEST}" \
  --format='value(image_summary.digest)'
note "env/staging.tfvars already references that digest in ${SITE_REPOSITORY}."

step "Re-check that no repository is public"
for repo in "$SITE_REPOSITORY" "$DOCKERHUB_REPOSITORY" "$BASE_REPOSITORY"; do
  assert_repo_private "$repo"
done
assert_repo_private "$LEGACY_REPOSITORY" optional

# --- 8. OPTIONAL: time-bound human first-apply grant ---------------------------
if [[ $TEMP_GRANT -eq 1 ]]; then
  if [[ $PRINT -eq 1 ]]; then
    EXPIRY="<now+${GRANT_HOURS}h, RFC 3339 UTC>"
  else
    EXPIRY="$(date -u -d "+${GRANT_HOURS} hours" +%Y-%m-%dT%H:%M:%SZ)"
  fi
  # ';' delimits the condition fields because the timestamp contains ':'.
  COND="^;^title=public-site-first-apply-temporary;description=Expires ${EXPIRY};expression=request.time < timestamp('${EXPIRY}')"
  M="user:${DEPLOYER_USER}"

  step "OPTIONAL: grant ${DEPLOYER_USER} first-apply roles until ${EXPIRY}"
  note "Each binding carries an IAM condition and stops working at the expiry."
  # Create the runtime SA, act as it, create/update the service and set its
  # invoker policy, and set the service's IAP accessor policy.
  for role in roles/iam.serviceAccountCreator roles/iam.serviceAccountUser \
    roles/run.admin roles/iap.admin; do
    run "${G[@]}" projects add-iam-policy-binding "$PROJECT_ID" \
      --member="$M" --role="$role" --condition="$COND"
  done
  # Read/write/lock the staging state objects.
  run "${G[@]}" storage buckets add-iam-policy-binding "gs://${STATE_BUCKET}" \
    --member="$M" --role=roles/storage.objectAdmin --condition="$COND"
  # Deploying a revision needs read access to its image repository.
  run "${G[@]}" artifacts repositories add-iam-policy-binding "$SITE_REPOSITORY" \
    --location="$REGION" --member="$M" --role=roles/artifactregistry.reader --condition="$COND"
  note "Expired conditional bindings grant nothing; remove them later with"
  note "remove-iam-policy-binding --member=${M} --role=ROLE --all"
fi

step "Done. Next: the deployer runs tofu init/plan/apply (book: Staging state and deployment)."
