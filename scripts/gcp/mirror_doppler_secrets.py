#!/usr/bin/env python3
"""Mirror Duet's Doppler secrets into the GCP Secret Manager the runtime reads.

ENG-28 keeps Doppler as the single place humans manage application
credentials. Cloud Run services and jobs keep consuming secrets through
Secret Manager via `--set-secrets` (revision metadata never holds secret
values, and only runtime identities can read them). This script is the one
narrow bridge between the two owners.

Behavior:
- The consumed secret names and versions come from the production runtime
  manifest (`infra/gcp/production-runtime.json`), never from a second list,
  so the mirror cannot drift from what services actually mount.
- Each mapped Doppler key is compared against the current Secret Manager
  value. Only differing secrets receive a new version; unchanged secrets
  are left alone, so repeated runs are idempotent.
- Missing in Doppler or unreadable in Secret Manager is a hard failure
  (exit 1) with a per-secret report. Values are never printed.
- The service mirror identity is granted `roles/secretmanager.secretAccessor`
  on every mirrored secret so deploys never outpace IAM.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path

# The runtime manifest stays in coda-app (it is a release input there and
# defines what the Duet services mount). This canonical copy resolves it
# through the superproject's submodule checkout.
DEFAULT_MANIFEST = (
    Path(__file__).resolve().parents[2]
    / "submodules"
    / "coda-app"
    / "infra"
    / "gcp"
    / "production-runtime.json"
)

# Doppler config per mode. Doppler's stg/prd configs mirror the two GCP
# projects named in the manifest.
DOPPLER_PROJECT = "coda-rust"
DOPPLER_CONFIG = {"production": "prd", "staging": "stg"}

# Secret Manager name -> the Doppler key holding its value. Most Doppler
# keys equal the manifest's env-var name (e.g. DUET_APNS_KEY_ID); the
# database credentials are stored under explicit aliases so the local
# development DATABASE_URL can never be mistaken for a hosted credential.
SM_ALIASES = {
    "DATABASE_RUNTIME_URL": "DUET_DATABASE_RUNTIME_URL",
    "DATABASE_DIRECTORY_URL": "DUET_DATABASE_DIRECTORY_URL",
    "DATABASE_MIGRATOR_URL": "DUET_DATABASE_MIGRATOR_URL",
}

# Release-path secrets read directly from Secret Manager by the migration
# runner (never mounted on a service, so the manifest does not declare
# them). They mirror in every mode; the migrator credential is required
# before staging and production SQL alike.
RELEASE_PATH_SECRETS = ("DATABASE_MIGRATOR_URL",)

# Doppler env names that only back local/development use and must never be
# pushed into a hosted Secret Manager.
LOCAL_ONLY_EXACT = {
    "DATABASE_URL",
    "DUET_API_AUTH",
    "DUET_API_URL",
    "DUET_BEARER",
    "PHYSICIAN_API_URL",
    "DOPPLER_CONFIG",
    "DOPPLER_ENVIRONMENT",
    "DOPPLER_PROJECT",
}

LOCAL_ONLY_PREFIXES = (
    "EPIC_SANDBOX_",
    "MODMED_SANDBOX_",
    "NEXTGEN_SANDBOX_",
    "ATHENA_PREVIEW_",
    "DUET_ATHENA_NONPROD_",
    "DUET_DEXCOM_NONPROD_",
    "DUET_EPIC_NONPROD_",  # dev sandbox credential, not the prod client id
)


def gcloud(*args: str, check: bool = True, stdin: str | None = None) -> str:
    result = subprocess.run(
        ["gcloud", *args],
        input=stdin,
        capture_output=True,
        text=True,
        check=False,
    )
    if check and result.returncode != 0:
        raise RuntimeError(
            f"gcloud {' '.join(args[:3])}... failed: {result.stderr.strip()}"
        )
    return result.stdout


def doppler_download(config: str) -> dict[str, str]:
    result = subprocess.run(
        [
            "doppler", "secrets", "download",
            "--project", DOPPLER_PROJECT,
            "--config", config,
            "--no-file", "--format", "json",
        ],
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(
            f"Doppler download ({config}) failed: {result.stderr.strip()}"
        )
    secrets = json.loads(result.stdout)
    return {key: str(value) for key, value in secrets.items()}


def manifest_secret_pairs(manifest: dict, mode: str) -> list[tuple[str, str]]:
    """(env name, Secret Manager name) pairs, exactly as consumers mount them.

    In staging mode only the staging contracts are read: the staged services
    mount only DATABASE_URL, and staging jobs do not exist.
    """
    pairs: list[tuple[str, str]] = []
    if mode == "staging":
        sections = [manifest["staging_service"], manifest["staging_worker_service"]]
    else:
        sections = [manifest["service"], manifest["worker_service"]]
        sections.extend(manifest["job_profiles"].values())
    for section in sections:
        for env_name, (secret_name, _version) in section["secrets"].items():
            pairs.append((env_name, secret_name))
    return sorted(set(pairs))


def is_local_only(env_name: str) -> bool:
    return (
        env_name in LOCAL_ONLY_EXACT
        or any(env_name.startswith(prefix) for prefix in LOCAL_ONLY_PREFIXES)
    )


def current_secret_value(project: str, secret: str) -> str | None:
    """Latest Secret Manager value, or None when the secret does not exist."""
    result = subprocess.run(
        [
            "gcloud", "secrets", "versions", "access", "latest",
            f"--project={project}", f"--secret={secret}",
        ],
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode == 0:
        return result.stdout
    # Distinguish NOT_FOUND (a secret not yet created; mirrorable) from any
    # other failure (permissions, network), which must fail closed.
    stderr = result.stderr.strip()
    if "NOT_FOUND" in stderr or "not found" in stderr.lower():
        return None
    raise RuntimeError(f"Reading secret {secret} failed: {stderr}")


def mirror(
    mode: str,
    project: str,
    manifest: dict,
    mirror_identity: str | None,
    dry_run: bool,
) -> int:
    config = DOPPLER_CONFIG[mode]
    secrets = doppler_download(config)

    # Phase 1 - validate everything before touching Secret Manager. A single
    # missing Doppler value must block all writes so a partial mirror can
    # never leave services pointing at stale credentials.
    wanted: list[tuple[str, str, str]] = []  # (env, sm_name, desired value)
    report: list[tuple[str, str, str]] = []
    pairs = manifest_secret_pairs(manifest, mode)
    for release_secret in RELEASE_PATH_SECRETS:
        pairs.append((release_secret, release_secret))
    for env_name, secret_name in sorted(set(pairs)):
        if is_local_only(env_name):
            continue
        doppler_name = SM_ALIASES.get(secret_name, env_name)
        if doppler_name not in secrets or not secrets[doppler_name].strip():
            report.append((env_name, "MISSING_IN_DOPPLER", doppler_name))
            continue
        wanted.append((env_name, secret_name, secrets[doppler_name]))

    if any(action == "MISSING_IN_DOPPLER" for _e, action, _t in report):
        width = max((len(row[0]) for row in report), default=10)
        for env_name, action, target in report:
            print(f"{env_name.ljust(width)}  {action.ljust(18)}  {target}")
        print("Mirror incomplete: missing Doppler values above.", file=sys.stderr)
        return 1

    # Phase 2 - compare and write only once every value is present.
    for env_name, secret_name, desired in wanted:
        current = current_secret_value(project, secret_name)
        if current is not None and current == desired:
            report.append((env_name, "unchanged", secret_name))
            continue
        report.append(
            (env_name, "would-add-version" if dry_run else "added-version", secret_name)
        )
        if not dry_run:
            gcloud(
                "secrets", "versions", "add", secret_name,
                f"--project={project}",
                "--data-file=-",
                "--quiet",
                stdin=desired,
            )
        if mirror_identity:
            gcloud(
                "secrets", "add-iam-policy-binding", secret_name,
                f"--project={project}",
                f"--member=serviceAccount:{mirror_identity}",
                "--role=roles/secretmanager.secretAccessor",
                "--quiet",
            )
    width = max((len(row[0]) for row in report), default=10)
    for env_name, action, target in report:
        print(f"{env_name.ljust(width)}  {action.ljust(18)}  {target}")
    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--mode", choices=sorted(DOPPLER_CONFIG), required=True)
    parser.add_argument("--project", required=True)
    parser.add_argument(
        "--mirror-identity", default=None,
        help="Service account to grant secretAccessor on mirrored secrets",
    )
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    manifest = json.loads(args.manifest.read_text())
    expected_project = (
        manifest["staging_project"] if args.mode == "staging" else manifest["project"]
    )
    if args.project != expected_project:
        raise RuntimeError(
            f"Mirror mode {args.mode} targets only {expected_project}, not {args.project}."
        )
    return mirror(
        args.mode,
        args.project,
        manifest,
        args.mirror_identity,
        args.dry_run,
    )


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except RuntimeError as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1)
    except OSError as error:
        # A missing doppler/gcloud binary must fail closed with an operator
        # legible message, not a traceback.
        print(f"error: release mirror could not run a required CLI: {error}", file=sys.stderr)
        raise SystemExit(1)
