#!/usr/bin/env python3
"""Install the pinned upstream release into this checkout's ignored target/."""
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import tempfile
import urllib.request
import zipfile

root = Path(__file__).resolve().parent.parent
lock = json.loads((root / "verification/verus.json").read_text())
key = f"{platform.system()}-{platform.machine()}"
if key not in lock["assets"]:
    raise SystemExit(f"No pinned binary for {key}; use the upstream source-build instructions.")
asset = lock["assets"][key]
version = lock["version"]
dest = root / "target/verification/tools" / version
binary = dest / f"verus-{asset['name']}" / "verus"
if not binary.is_file():
    dest.mkdir(parents=True, exist_ok=True)
    name = f"verus-{version}-{asset['name']}.zip"
    url = f"https://github.com/verus-lang/verus/releases/download/release/{version}/{name}"
    with tempfile.TemporaryDirectory(prefix="coda-verus-") as scratch:
        archive = Path(scratch) / name
        print(f"Downloading pinned Verus {version} for {key}", flush=True)
        urllib.request.urlretrieve(url, archive)
        with archive.open("rb") as src:
            digest = hashlib.file_digest(src, "sha256").hexdigest()
        if digest != asset["sha256"]:
            raise SystemExit("Verus archive checksum mismatch; nothing was installed.")
        with zipfile.ZipFile(archive) as package:
            for entry in package.infolist():
                target = (dest / entry.filename).resolve()
                if not target.is_relative_to(dest.resolve()):
                    raise SystemExit("Unsafe archive member")
            package.extractall(dest)
            for entry in package.infolist():
                mode = entry.external_attr >> 16
                if mode and not entry.is_dir():
                    os.chmod(dest / entry.filename, mode & 0o777)
    if not binary.is_file():
        raise SystemExit("Release did not contain the expected verifier")
print(binary, flush=True)
# A missing Rust toolchain is reported by the upstream launcher. We do not
# change the workspace's default toolchain or silently bypass that check.
raise SystemExit(subprocess.call([str(binary), "--version"]))
