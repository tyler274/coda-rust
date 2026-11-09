#!/usr/bin/env python3
"""Carry the image proof job's archives unchanged into the staging registry."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tarfile


REPOSITORIES = {
    "physician-runtime": "coda-physician-sample",
    "physician-worker": "coda-physician-worker",
    "physician-migration": "coda-physician-migrate",
    "duet-runtime": "coda-duet-api",
    "duet-migration": "coda-duet-migrate",
    "observability-relay": "coda-observability-relay",
    "langfuse-sso": "coda-langfuse-sso",
}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def sha256(path):
    with path.open("rb") as source:
        return hashlib.file_digest(source, "sha256").hexdigest()


def image_identity(path, revision):
    with tarfile.open(path, "r:*") as archive:
        manifest = json.load(archive.extractfile("manifest.json"))
        require(len(manifest) == 1, f"{path}: expected one image")
        image = manifest[0]
        require(len(image["RepoTags"]) == 1, f"{path}: expected one image tag")
        repository, tag = image["RepoTags"][0].rsplit(":", 1)
        config = archive.extractfile(image["Config"]).read()
        labels = json.loads(config)["config"]["Labels"]
        require(tag == revision and labels["org.opencontainers.image.revision"] == revision,
                f"{path}: image revision differs from {revision}")
        return repository, "sha256:" + hashlib.sha256(config).hexdigest()


def package(paths, output, revision):
    require(re.fullmatch(r"[0-9a-f]{40}", revision), "Expected a full source commit SHA")
    selected = {}
    for line in paths.read_text().splitlines():
        path = Path(line)
        repository, image_id = image_identity(path, revision)
        for key, expected in REPOSITORIES.items():
            if repository == expected:
                require(key not in selected, f"Duplicate tested image: {key}")
                selected[key] = (path, image_id)
    require(set(selected) == set(REPOSITORIES), "Missing tested staging images")
    output.mkdir()
    images = {}
    for key, (source, image_id) in selected.items():
        filename = f"{key}.tar.gz"
        target = output / filename
        shutil.copyfile(source, target)
        images[key] = {"archive": filename, "sha256": sha256(target), "image_id": image_id}
    manifest = {"source_revision": revision, "images": images}
    (output / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    return manifest


def verify(bundle, revision):
    manifest = json.loads((bundle / "manifest.json").read_text())
    require(manifest["source_revision"] == revision, "Tested bundle is from another commit")
    require(set(manifest["images"]) == set(REPOSITORIES), "Expected every tested staging image")
    for key, repository in REPOSITORIES.items():
        entry = manifest["images"][key]
        require(entry["archive"] == f"{key}.tar.gz", f"Unexpected archive for {key}")
        path = bundle / entry["archive"]
        require(sha256(path) == entry["sha256"], f"Tested archive checksum differs: {key}")
        actual_repository, image_id = image_identity(path, revision)
        require(actual_repository == repository, f"Wrong tested image for {key}")
        require(image_id == entry["image_id"], f"Tested image identity differs: {key}")
    return manifest


def publish(bundle, output, revision, registry):
    # Verify the complete handoff before the first registry write. There is no
    # rebuild fallback: absent, expired or changed proof output stops the release.
    manifest = verify(bundle, revision)
    credentials = "oauth2accesstoken:" + os.environ["ACCESS_TOKEN"]
    output.mkdir(exist_ok=True)
    shutil.copyfile(bundle / "manifest.json", output / "tested-images.json")
    for key, repository in REPOSITORIES.items():
        entry = manifest["images"][key]
        target = f"{registry}/{repository}"
        subprocess.run([
            "skopeo", "copy", "--insecure-policy", "--quiet",
            "--dest-creds", credentials, f"docker-archive:{bundle / entry['archive']}",
            f"docker://{target}:{revision}",
        ], check=True)
        digest = subprocess.check_output([
            "skopeo", "inspect", "--creds", credentials, "--format", "{{.Digest}}",
            f"docker://{target}:{revision}",
        ], text=True).strip()
        require(re.fullmatch(r"sha256:[0-9a-f]{64}", digest), f"No registry digest for {key}")
        pinned = f"{target}@{digest}"
        # Registry compression can change the manifest digest. The config digest
        # (Docker image ID) includes the uncompressed layer digests and must stay
        # identical to the image Docker ran in the proof job.
        config = subprocess.check_output([
            "skopeo", "inspect", "--creds", credentials, "--raw", "--config",
            f"docker://{pinned}",
        ])
        require("sha256:" + hashlib.sha256(config).hexdigest() == entry["image_id"],
                f"Published image differs from tested image: {key}")
        (output / key).write_text(pinned + "\n")
        print(f"Published tested {repository}@{digest}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    pack = commands.add_parser("package")
    pack.add_argument("paths", type=Path)
    pack.add_argument("output", type=Path)
    check = commands.add_parser("verify")
    check.add_argument("bundle", type=Path)
    push = commands.add_parser("publish")
    push.add_argument("bundle", type=Path)
    push.add_argument("output", type=Path)
    push.add_argument("--registry", required=True)
    for command in (pack, check, push):
        command.add_argument("--revision", required=True)
    args = parser.parse_args()
    try:
        if args.command == "package":
            package(args.paths, args.output, args.revision)
        elif args.command == "verify":
            verify(args.bundle, args.revision)
        else:
            publish(args.bundle, args.output, args.revision, args.registry)
    except subprocess.CalledProcessError as error:
        # Command arguments include the short-lived registry credential.
        raise SystemExit(f"Image publication command failed (exit {error.returncode})") from None
    except (ValueError, KeyError, OSError, tarfile.TarError) as error:
        raise SystemExit(str(error)) from None


if __name__ == "__main__":
    main()
