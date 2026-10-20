#!/usr/bin/env python3
"""Inspect dockerTools archives without credentials or a running Docker daemon."""
import json
import sys
import tarfile


def check(path):
    with tarfile.open(path, "r:*") as archive:
        manifest = json.load(archive.extractfile("manifest.json"))
        assert len(manifest) == 1, "expected one image"
        image = manifest[0]
        config = json.load(archive.extractfile(image["Config"]))
        runtime = config["config"]
        assert runtime["User"] == "65532:65532", "image must be non-root"
        assert runtime["WorkingDir"] == "/tmp"
        assert runtime["Labels"]["org.opencontainers.image.revision"]
        assert runtime["Labels"]["com.coda.runtime-readiness"] == "packaging-only-not-staging-ready"
        assert all(not tag.endswith(":latest") for tag in image["RepoTags"])
        env = dict(value.split("=", 1) for value in runtime["Env"])
        assert not {"DOPPLER_TOKEN", "OPENAI_API_KEY", "DATABASE_URL", "DUET_BEARER"} & env.keys()
        paths = set()
        for layer in image["Layers"]:
            with tarfile.open(fileobj=archive.extractfile(layer), mode="r|*") as contents:
                for member in contents:
                    name = member.name.removeprefix("./").strip("/")
                    paths.add(name)
                    assert "submodules/" not in name, name
                    assert "/.git/" not in name and "/.cache/" not in name, name
                    assert not name.endswith(("/bin/cargo", "/bin/rustc", "/bin/nix", "/bin/gcc")), name
        assert runtime["Entrypoint"][0].lstrip("/") in paths
        assert any(name.endswith("/bin/doppler") for name in paths)
        for key in ("PHYSICIAN_SITE_DIR", "DUET_SITE_DIR"):
            if key in env:
                directory = env[key].lstrip("/")
                product = "physician-app" if key.startswith("PHYSICIAN") else "duet-app"
                for asset in ("index.html", f"{product}.js", f"{product}_bg.wasm"):
                    assert f"{directory}/{asset}" in paths, asset
        print(f"{path}: non-root closure, entrypoint, assets, labels, and secret-free configuration OK")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        raise SystemExit("usage: check-images.py IMAGE.tar.gz [...]")
    for filename in sys.argv[1:]:
        check(filename)
