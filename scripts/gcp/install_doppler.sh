#!/usr/bin/env bash
# Install the pinned Doppler CLI release the release scripts and CI rely on.
# Follows the install_cloud_sql_proxy.sh pattern: one exact version, verified
# against its published SHA256, extracted to a single caller-chosen path.

set -euo pipefail

readonly DOPPLER_VERSION="v3.77.0"
readonly DOPPLER_SHA256="92587cb89c2ff82fc2ddec2b2a9f275ff320994a44e16cd9310b55a429a82084"
readonly DOPPLER_URL="https://cli.doppler.com/download?os=linux&arch=amd64&format=tar&version=${DOPPLER_VERSION}"

if [[ $# -ne 1 ]]; then
  echo "usage: $0 OUTPUT_PATH" >&2
  exit 2
fi

output_path="$1"
tarball="$(mktemp -d)/doppler-${DOPPLER_VERSION}.tar"
curl --fail --silent --show-error --location "$DOPPLER_URL" --output "$tarball"
echo "${DOPPLER_SHA256}  ${tarball}" | sha256sum --check --status
# The tarball's binary entry is `doppler` (no `./` prefix); extract only it to
# the exact path. nix/packages.nix reads the version and SHA-256 above, so the
# container images carry this same verified release.
tar --extract --file="$tarball" --directory="$(dirname -- "$output_path")" \
  --transform="s|^doppler$|$(basename -- "$output_path")|" doppler
chmod 0755 "$output_path"
echo "Installed Doppler CLI ${DOPPLER_VERSION}."
