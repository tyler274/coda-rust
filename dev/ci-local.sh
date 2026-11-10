#!/usr/bin/env bash
# Run one CI job of this worktree's HEAD on Cyrene's Firecracker CI, from the
# coda-agent sandbox, and print its log.
#
#   dev/ci-local.sh [job]      # job defaults to ci
#
# It writes a git bundle of HEAD and the job name into the CI request spool,
# waits for the result, prints the log and exits with the job's status. The
# job runs `act -j <job>` offline in a disposable microVM with no secrets;
# uncommitted changes are not included. The host side is described in the
# NixOS configuration's README, "Coda CI in Firecracker".
set -euo pipefail

job=${1:-ci}
ci=${CODA_CI_DIR:-/agents/coda/ci}
requests=$ci/requests
results=$ci/results
timeout_seconds=${CODA_CI_TIMEOUT:-14400}

if [[ ! $job =~ ^[A-Za-z0-9_-]{1,64}$ ]]; then
  echo "ci-local: job must match [A-Za-z0-9_-]{1,64}: $job" >&2
  exit 2
fi
if [[ ! -d $requests || ! -d $results ]]; then
  echo "ci-local: no CI spool at $ci; run this inside the coda-agent sandbox" >&2
  exit 2
fi

commit=$(git rev-parse --verify 'HEAD^{commit}')
if [[ -n $(git status --porcelain --untracked-files=no) ]]; then
  echo "ci-local: uncommitted changes are not sent; testing $commit" >&2
fi

# Bundle from a scratch repository that borrows this one's objects, so this
# repository gains no refs.
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
git init --quiet --bare "$scratch/repo"
git rev-parse --path-format=absolute --git-common-dir | sed 's|$|/objects|' \
  > "$scratch/repo/objects/info/alternates"
git -C "$scratch/repo" update-ref refs/coda-ci/head "$commit"
git -C "$scratch/repo" bundle create --quiet "$scratch/repo.bundle" refs/coda-ci/head

id=$(od -An -N6 -tx1 /dev/urandom | tr -d ' \n')
cp "$scratch/repo.bundle" "$requests/$id.bundle"
# The job file goes last and appears whole, by rename: it marks the request
# complete.
printf '%s\n' "$job" > "$requests/.$id.job.part"
mv "$requests/.$id.job.part" "$requests/$id.job"
echo "ci-local: queued $id: act -j $job at $commit" >&2

deadline=$((SECONDS + timeout_seconds))
until [[ -e $results/$id/status ]]; do
  if ((SECONDS >= deadline)); then
    echo "ci-local: no result for $id after ${timeout_seconds}s" >&2
    exit 124
  fi
  sleep 5
done
cat "$results/$id/log" 2>/dev/null || echo "ci-local: $id has no log" >&2
status=$(<"$results/$id/status")
echo "ci-local: $id exited $status" >&2
exit "$status"
