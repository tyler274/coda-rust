"""The Doppler→Secret Manager mirror keeps Doppler the single source of truth.

These tests exercise `scripts/gcp/mirror_doppler_secrets.py` through fake
`doppler` and `gcloud` executables. The manifest is a synthetic fixture in the
runtime manifest's shape, because CI does not check out the private coda-app
submodule; the fixture stores are built through the script's own mapping.
Values are synthetic; no real secret is read or written.
"""

from __future__ import annotations

import importlib.util
import json
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SCRIPT = ROOT / "scripts/gcp/mirror_doppler_secrets.py"
MANIFEST = Path(__file__).resolve().parent / "fixtures/production-runtime.json"


def load_script():
    spec = importlib.util.spec_from_file_location("mirror_doppler_secrets", SCRIPT)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


mirror = load_script()


def doppler_env(tmp_path: Path, secrets: dict[str, str]) -> dict[str, str]:
    """A fake doppler CLI that serves the given config values."""
    payload = json.dumps(secrets)
    doppler = tmp_path / "doppler"
    doppler.write_text(f"""#!{sys.executable}
import json, sys
args = sys.argv[1:]
with open({str(tmp_path / 'doppler-calls.log')!r}, "a") as log:
    log.write(" ".join(args) + "\\n")
if args[:2] == ["secrets", "download"] and "--config" in args:
    print({payload!r})
else:
    print("unexpected " + " ".join(args), file=sys.stderr)
    raise SystemExit(2)
""")
    doppler.chmod(0o755)
    return {**os.environ, "PATH": f"{tmp_path}{os.pathsep}{os.environ['PATH']}"}


def gcloud_env(tmp_path: Path, existing: dict[str, str] | None = None) -> dict[str, str]:
    """A fake gcloud that records calls and serves stored secret values."""
    stored = json.dumps(existing or {})
    gcloud = tmp_path / "gcloud"
    gcloud.write_text(f"""#!{sys.executable}
import json, sys
args = sys.argv[1:]
with open({str(tmp_path / 'gcloud-calls.log')!r}, "a") as log:
    log.write(" ".join(args) + "\\n")
stored = json.loads({stored!r})
if args[:2] == ["secrets", "versions"] and args[2] == "access":
    secret = next(a for a in args if a.startswith("--secret=")).split("=", 1)[1]
    if secret in stored:
        sys.stdout.write(stored[secret])
    else:
        print(f"ERROR: (gcloud.secretmanager.versions.access) NOT_FOUND: {{secret}}",
              file=sys.stderr)
        raise SystemExit(1)
elif args[:3] == ["secrets", "versions", "add"]:
    secret = args[3]
    stored[secret] = sys.stdin.read()
    print(f"Created version for {{secret}}")
elif args[:3] == ["secrets", "add-iam-policy-binding"]:
    print("Updated policy")
else:
    print("unexpected " + " ".join(args), file=sys.stderr)
    raise SystemExit(2)
""")
    gcloud.chmod(0o755)
    return {**os.environ, "PATH": f"{tmp_path}{os.pathsep}{os.environ['PATH']}"}


def run(script_env: dict[str, str], *args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, SCRIPT, f"--manifest={MANIFEST}", *args],
        env=script_env,
        capture_output=True,
        text=True,
        check=False,
    )


def gcloud_calls(tmp_path: Path) -> str:
    log = tmp_path / "gcloud-calls.log"
    return log.read_text() if log.exists() else ""


def consistent_inputs(
    mode: str,
    rotations: dict[str, str] | None = None,
    omissions: set[str] | None = None,
) -> tuple[dict[str, str], dict[str, str]]:
    """Doppler payload and Secret Manager store that agree through the
    script's own mapping for one mode. Rotations apply to Doppler only (the
    SM store keeps the stale value); omissions remove a Doppler key."""
    rotations = rotations or {}
    omissions = omissions or set()
    doppler: dict[str, str] = {}
    sm: dict[str, str] = {}
    pairs = mirror.manifest_secret_pairs(json.loads(MANIFEST.read_text()), mode)
    for release_secret in mirror.RELEASE_PATH_SECRETS:
        pairs.append((release_secret, release_secret))
    for env_name, secret_name in sorted(set(pairs)):
        if mirror.is_local_only(env_name):
            continue
        sm[secret_name] = f"value-of-{secret_name}"
        if env_name not in omissions:
            doppler[mirror.SM_ALIASES.get(secret_name, env_name)] = rotations.get(
                env_name, f"value-of-{secret_name}"
            )
    return doppler, sm


PROJECT = {"production": "coda-duet-prod", "staging": "coda-duet-staging"}


def test_missing_doppler_value_fails_closed_and_blocks_writes(tmp_path):
    doppler_payload, sm_store = consistent_inputs(
        "production", omissions={"OPENAI_API_KEY"}
    )
    env = doppler_env(tmp_path, doppler_payload)
    env = gcloud_env(tmp_path, sm_store)
    result = run(env, "--mode", "production", f"--project={PROJECT['production']}")
    assert result.returncode == 1
    assert "MISSING_IN_DOPPLER" in result.stdout
    assert "OPENAI_API_KEY" in result.stdout
    # The abort happens before any Secret Manager read or write.
    assert "versions add" not in gcloud_calls(tmp_path)


def test_staging_mirror_requires_only_the_staging_contract(tmp_path):
    # Staging mounts only DATABASE_URL; the stg config must not be demanded
    # the production-only credentials (APNs, connectors, pepper).
    doppler_payload, sm_store = consistent_inputs("staging")
    env = doppler_env(tmp_path, doppler_payload)
    env = gcloud_env(tmp_path, sm_store)
    result = run(env, "--mode", "staging", f"--project={PROJECT['staging']}")
    assert result.returncode == 0, result.stdout + result.stderr
    calls = gcloud_calls(tmp_path)
    assert "versions add" not in calls
    for secret in ("DUET_APNS_KEY_ID", "duet-connector-token-key", "OPENAI_API_KEY"):
        assert f"--secret={secret}" not in calls
    assert "unchanged" in result.stdout


def test_staging_mirror_demands_the_migrator_credential(tmp_path):
    doppler_payload, sm_store = consistent_inputs(
        "staging", omissions={"DATABASE_MIGRATOR_URL"}
    )
    env = doppler_env(tmp_path, doppler_payload)
    env = gcloud_env(tmp_path, sm_store)
    result = run(env, "--mode", "staging", f"--project={PROJECT['staging']}")
    assert result.returncode == 1
    assert "MISSING_IN_DOPPLER" in result.stdout
    assert "DATABASE_MIGRATOR_URL" in result.stdout
    assert "versions add" not in gcloud_calls(tmp_path)


def test_unchanged_secret_gets_no_new_version_and_report_hides_values(tmp_path):
    doppler_payload, sm_store = consistent_inputs("production")
    env = doppler_env(tmp_path, doppler_payload)
    env = gcloud_env(tmp_path, sm_store)
    result = run(env, "--mode", "production", f"--project={PROJECT['production']}")
    assert result.returncode == 0, result.stderr
    assert "versions add" not in gcloud_calls(tmp_path)
    for value in set(doppler_payload.values()) | set(sm_store.values()):
        assert value not in result.stdout
    assert "unchanged" in result.stdout


def test_changed_secret_gets_exactly_one_new_version(tmp_path):
    rotated = "value-of-OPENAI_API_KEY-rotated"
    doppler_payload, sm_store = consistent_inputs(
        "production", rotations={"OPENAI_API_KEY": rotated}
    )
    env = doppler_env(tmp_path, doppler_payload)
    env = gcloud_env(tmp_path, sm_store)
    result = run(env, "--mode", "production", f"--project={PROJECT['production']}")
    assert result.returncode == 0, result.stderr
    calls = gcloud_calls(tmp_path)
    assert calls.count("versions add") == 1
    assert "OPENAI_API_KEY" in calls
    assert rotated not in result.stdout
    assert "added-version" in result.stdout


def test_dry_run_makes_no_writes(tmp_path):
    doppler_payload, _sm_store = consistent_inputs("production")
    env = doppler_env(tmp_path, doppler_payload)
    env = gcloud_env(tmp_path, {})
    result = run(
        env, "--mode", "production", f"--project={PROJECT['production']}", "--dry-run"
    )
    assert result.returncode == 0, result.stderr
    assert "versions add" not in gcloud_calls(tmp_path)
    assert "would-add-version" in result.stdout


def test_project_mode_mismatch_is_refused(tmp_path):
    doppler_payload, _sm_store = consistent_inputs("staging")
    env = doppler_env(tmp_path, doppler_payload)
    result = run(env, "--mode", "staging", "--project=coda-duet-prod")
    assert result.returncode == 1
    assert "targets only" in result.stderr
    assert "gcloud" not in gcloud_calls(tmp_path) if (tmp_path / "gcloud-calls.log").exists() else True


def test_local_only_dev_secrets_are_never_mirrored(tmp_path):
    doppler_payload, _sm_store = consistent_inputs("production")
    dev_extra = {
        "DATABASE_URL": "postgresql://local-only/duet",
        "EPIC_SANDBOX_CLIENT_ID": "local-sandbox",
        "DUET_DEXCOM_NONPROD_CLIENT_ID": "local-dexcom",
    }
    env = doppler_env(tmp_path, {**doppler_payload, **dev_extra})
    env = gcloud_env(tmp_path, {})
    result = run(
        env, "--mode", "production", f"--project={PROJECT['production']}", "--dry-run"
    )
    assert result.returncode == 0, result.stderr
    calls = gcloud_calls(tmp_path)
    for value in dev_extra.values():
        assert value not in calls
    assert "EPIC_SANDBOX" not in result.stdout


def test_unknown_gcloud_failure_fails_closed(tmp_path):
    doppler_payload, _sm_store = consistent_inputs("production")
    env = doppler_env(tmp_path, doppler_payload)
    gcloud = tmp_path / "gcloud"
    gcloud.write_text("#!/usr/bin/env bash\nexit 3\n")
    gcloud.chmod(0o755)
    env = {**os.environ, "PATH": f"{tmp_path}{os.pathsep}{os.environ['PATH']}"}
    result = run(env, "--mode", "production", f"--project={PROJECT['production']}")
    assert result.returncode == 1
    assert "error:" in result.stderr
    assert "versions add" not in gcloud_calls(tmp_path)
