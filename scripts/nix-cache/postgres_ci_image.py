#!/usr/bin/env python3
"""Address the full Postgres image derivation and resolve private GHCR digests."""

import argparse
import base64
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import urllib.error
import urllib.parse
import urllib.request


REPOSITORY = "coda-health/coda-rust/postgres-ci"
IMAGE = f"ghcr.io/{REPOSITORY}"
INSTALLABLE = ".#packages.x86_64-linux.postgres-ci-image"
MANIFEST_TYPES = ", ".join((
    "application/vnd.oci.image.index.v1+json",
    "application/vnd.oci.image.manifest.v1+json",
    "application/vnd.docker.distribution.manifest.list.v2+json",
    "application/vnd.docker.distribution.manifest.v2+json",
))


class MissingImage(ValueError):
    """Authenticated manifest lookup returned 404, not an authentication failure."""


def derivation_tag(drv_path):
    match = re.fullmatch(r"/nix/store/([0-9abcdfghijklmnpqrsvwxyz]{32})-[^/\s]+\.drv", drv_path)
    if not match:
        raise ValueError("Expected a complete Nix image derivation store path")
    return f"drv-{match[1]}"


def image_tag():
    drv_path = subprocess.check_output(
        ["nix", "eval", "--raw", f"{INSTALLABLE}.drvPath"], text=True,
    ).strip()
    return derivation_tag(drv_path)


def registry_digest(tag, username, password, open_url=urllib.request.urlopen):
    if not re.fullmatch(r"drv-[0-9abcdfghijklmnpqrsvwxyz]{32}", tag):
        raise ValueError("Invalid image derivation tag")
    if not username or not password:
        raise ValueError("Private GHCR lookup requires a scoped GitHub token; forks may lack package read access")
    query = urllib.parse.urlencode({"service": "ghcr.io", "scope": f"repository:{REPOSITORY}:pull"})
    credentials = base64.b64encode(f"{username}:{password}".encode()).decode()
    manifest_lookup = False
    try:
        request = urllib.request.Request(f"https://ghcr.io/token?{query}", headers={"Authorization": f"Basic {credentials}"})
        with open_url(request, timeout=60) as response:
            token = json.load(response)["token"]
        request = urllib.request.Request(
            f"https://ghcr.io/v2/{REPOSITORY}/manifests/{tag}",
            headers={"Authorization": f"Bearer {token}", "Accept": MANIFEST_TYPES},
            method="HEAD",
        )
        manifest_lookup = True
        with open_url(request, timeout=60) as response:
            digest = response.headers.get("Docker-Content-Digest", "")
    except urllib.error.HTTPError as error:
        # Never include response bodies, request headers or exception text.
        if manifest_lookup and error.code == 404:
            raise MissingImage(
                "Exact Postgres CI image is missing. Same-repository PRs and trusted runs "
                "can publish it; forks cannot publish and require an authorized run first. "
                "CI fails rather than accepting unrun tests or a stale image."
            ) from None
        raise ValueError(
            f"GHCR image lookup failed (HTTP {error.code}); verify private package read access. "
            "Fork tokens may not have access; run an authorized same-repository revision. "
            "This error is not treated as a missing image."
        ) from None
    except (urllib.error.URLError, TimeoutError, KeyError, TypeError, json.JSONDecodeError):
        raise ValueError("GHCR image lookup failed; no image was selected or marked missing") from None
    if not re.fullmatch(r"sha256:[0-9a-f]{64}", digest):
        raise ValueError("GHCR did not return a valid immutable manifest digest")
    return digest


def select(command, existing_tag=None):
    if existing_tag is not None:
        prefix = f"{IMAGE}:"
        if not existing_tag.startswith(prefix):
            raise ValueError("Image tag must address the configured GHCR repository")
        tag = existing_tag[len(prefix):]
        if not re.fullmatch(r"drv-[0-9abcdfghijklmnpqrsvwxyz]{32}", tag):
            raise ValueError("Invalid image derivation tag")
    else:
        tag = image_tag()
    outputs = {"tag": f"{IMAGE}:{tag}"}
    if command == "tag":
        return outputs
    try:
        digest = registry_digest(tag, os.environ.get("GITHUB_ACTOR", ""), os.environ.get("GHCR_READ_TOKEN", ""))
    except MissingImage:
        if command != "inspect":
            raise
        outputs["missing"] = "true"
        return outputs
    outputs.update(missing="false", image=f"{IMAGE}@{digest}")
    return outputs


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("tag", "inspect", "resolve"))
    parser.add_argument("--image-tag", help="Reuse the inspected derivation tag without reevaluating Nix")
    args = parser.parse_args()
    try:
        outputs = select(args.command, args.image_tag)
        lines = "".join(f"{key}={value}\n" for key, value in outputs.items())
        if output := os.environ.get("GITHUB_OUTPUT"):
            with Path(output).open("a") as stream:
                stream.write(lines)
        print(lines, end="")
    except (ValueError, subprocess.CalledProcessError) as error:
        sys.exit(f"Postgres CI image selection failed: {error}")


if __name__ == "__main__":
    main()
