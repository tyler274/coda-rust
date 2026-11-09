#!/usr/bin/env python3
"""Inspect dockerTools archives without credentials or a running Docker daemon."""
import json
import pathlib
import sys
import tarfile

SKILLS = pathlib.Path(__file__).resolve().parent.parent / "skills"


def packaged_skills():
    """Every file of the source catalog, relative to skills/."""
    return sorted(
        str(path.relative_to(SKILLS)) for path in SKILLS.rglob("*") if path.is_file()
    )

ROLES = {"runtime", "migration", "job"}
SECRET_KEYS = {
    "DOPPLER_TOKEN", "OPENAI_API_KEY", "DATABASE_URL", "PHYSICIAN_DATABASE_URL",
    "FRONT_DOOR_DATABASE_URL", "DUET_PRIVACY_DELIVERY_DATABASE_URL", "DUET_BEARER",
    "DUET_API_BEARER", "PHYSICIAN_API_BEARER", "CODA_ALLOW_MODEL_CALLS",
    "DUET_OPENAI_PHI_READY", "PHYSICIAN_REDIS_URL", "PHYSICIAN_REDIS_CA_PEM",
    "PUBSUB_EMULATOR_HOST", "PHYSICIAN_LOCAL_SCALE",
}
# Images that read no secrets and therefore start without Doppler.
SECRET_FREE = ("/availability-probe", "/observability-relay", "/langfuse-sso")
TOOLCHAIN = ("/bin/cargo", "/bin/rustc", "/bin/nix", "/bin/gcc", "/bin/sh", "/bin/bash", "/bin/psql")
# Store paths that must never be runtime references of an image.
BUILD_ONLY = ("rust-", "rustc-", "cargo-", "gcc-wrapper", "binutils", "bash-",
              "coreutils-", "wasm-bindgen", "coda-servers", "coda-web", "vendor-")


def check(path):
    with tarfile.open(path, "r:*") as archive:
        manifest = json.load(archive.extractfile("manifest.json"))
        assert len(manifest) == 1, "expected one image"
        image = manifest[0]
        config = json.load(archive.extractfile(image["Config"]))
        runtime = config["config"]
        labels = runtime["Labels"]
        assert runtime["User"] == "65532:65532", "image must be non-root"
        assert runtime["WorkingDir"] == "/tmp"
        assert labels["org.opencontainers.image.revision"]
        assert labels["com.coda.runtime-readiness"] == "packaging-only-not-staging-ready"
        role = labels["com.coda.role"]
        assert role in ROLES, role
        assert all(not tag.endswith(":latest") for tag in image["RepoTags"])
        env = dict(value.split("=", 1) for value in runtime["Env"])
        assert not SECRET_KEYS & env.keys(), SECRET_KEYS & env.keys()
        assert ("PORT" in env) == (role == "runtime"), "only servers declare PORT"
        assert not any(k in env for k in ("PHYSICIAN_LISTEN_ADDR", "DUET_LISTEN_ADDR")), \
            "listen address must come from Cloud Run's PORT"
        entry = runtime["Entrypoint"]
        if labels["com.coda.binary"].endswith(SECRET_FREE):
            # Its identity is its Cloud Run service account; no Doppler, no token.
            assert entry == [labels["com.coda.binary"]], entry
        else:
            assert entry[0].endswith("/bin/doppler") and entry[1] == "run", entry
            assert "--no-fallback" in entry, "Doppler must not persist a secret fallback file"
            assert entry[entry.index("--") + 1] == labels["com.coda.binary"], entry
        assert not runtime.get("Cmd"), "no implicit arguments"
        paths = set()
        chart_manifest = None
        chart_directory = env.get("PHYSICIAN_SYNTHETIC_CHARTS", "").lstrip("/")
        for layer in image["Layers"]:
            with tarfile.open(fileobj=archive.extractfile(layer), mode="r|*") as contents:
                for member in contents:
                    name = member.name.removeprefix("./").strip("/")
                    paths.add(name)
                    if chart_directory and name == f"{chart_directory}/manifest.json":
                        chart_manifest = json.load(contents.extractfile(member))
                    assert "submodules/" not in name, name
                    assert "/.git/" not in name and "/.cache/" not in name, name
                    assert not name.endswith(TOOLCHAIN), name
                    parts = name.split("/")
                    if len(parts) >= 3 and parts[:2] == ["nix", "store"]:
                        store_name = parts[2].split("-", 1)[-1]
                        assert not store_name.startswith(BUILD_ONLY), name
        assert entry[0].lstrip("/") in paths
        assert labels["com.coda.binary"].lstrip("/") in paths
        for required in ("etc/passwd", "etc/group", "etc/ssl/certs/ca-certificates.crt", "tmp"):
            assert required in paths, required
        assert env["SSL_CERT_FILE"].lstrip("/") in paths
        assert f"{env['TZDIR'].lstrip('/')}/America/Los_Angeles" in paths
        if labels["com.coda.binary"].endswith(("/physician-sample", "/physician-worker")):
            # Hosted physician startup refuses without it; Doppler must not be
            # the only place it is set.
            assert env.get("PHYSICIAN_AUTH") == "required", "physician servers require sessions"
        if labels["com.coda.binary"].endswith("/physician-worker"):
            # Hosted worker startup loads the catalog from here and refuses
            # without it; every packaged file must be in the image.
            assert "PHYSICIAN_SKILLS_DIR" in env, "the worker image must name its skill catalog"
            directory = env["PHYSICIAN_SKILLS_DIR"].lstrip("/")
            expected = packaged_skills()
            assert expected, "the source catalog is empty"
            missing = [name for name in expected if f"{directory}/{name}" not in paths]
            assert not missing, f"skill assets missing from the worker image: {missing}"
        elif "PHYSICIAN_SKILLS_DIR" in env:
            raise AssertionError("only the worker image carries the skill catalog")
        if labels["com.coda.binary"].endswith("/physician-migrate"):
            assert "PHYSICIAN_SYNTHETIC_CHARTS" in env, "the migration image must name its chart bank"
            directory = env["PHYSICIAN_SYNTHETIC_CHARTS"].lstrip("/")
            assert chart_manifest and chart_manifest["patients"], "nonempty chart manifest missing from migration image"
            expected = {
                f"{directory}/patients/{patient['mrn']}/{family}.ndjson"
                for patient in chart_manifest["patients"] for family in chart_manifest["families"]
            }
            missing = expected - paths
            assert not missing, f"patient chart files missing from migration image: {sorted(missing)}"
        elif "PHYSICIAN_SYNTHETIC_CHARTS" in env:
            raise AssertionError("only the migration image carries the chart bank")
        for key in ("PHYSICIAN_SITE_DIR", "DUET_SITE_DIR"):
            if key in env:
                directory = env[key].lstrip("/")
                product = "physician-app" if key.startswith("PHYSICIAN") else "duet-app"
                for asset in ("index.html", f"{product}.js", f"{product}_bg.wasm"):
                    assert f"{directory}/{asset}" in paths, asset
        print(f"{image['RepoTags'][0]} ({role}): non-root closure, Doppler entrypoint, "
              f"CA/tz/passwd, assets, labels and secret-free configuration OK")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        raise SystemExit("usage: check-images.py IMAGE.tar.gz [...]")
    for filename in sys.argv[1:]:
        check(filename)
