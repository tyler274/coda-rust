#!/usr/bin/env bash
# Build the book and check every link in the HTML it produces, offline.
#
# Absolute links (/crates/..., /infra/..., /submodules/...) resolve against the
# repository root, the way `python3 -m http.server` serves the book from this
# checkout. `#fragment` links are checked against the target page's headings,
# so a renamed heading in a product document fails here. External URLs are not
# fetched: the product repositories are private and Linear needs a login.
#
# A submodule that is not checked out (CI checks out none) cannot be read, so
# links into it are skipped with a notice rather than failed. Run this from a
# checkout with submodules initialised to check them too.
#
# lychee comes from flake.nix locally and from the pinned release in CI.
set -euo pipefail
root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
cd "$root"

mdbook build book

args=(--offline --no-progress --include-fragments --format detailed --root-dir "$root")
while read -r _ path; do
  if [ ! -e "$path/.git" ]; then
    echo "notice: $path is not checked out; links into it are not checked" >&2
    args+=(--exclude "^file://.*/${path}/")
  fi
done < <(git config --file .gitmodules --get-regexp '^submodule\..*\.path$')

lychee "${args[@]}" target/book
