"""Evaluate real release derivations to prove dependency reuse (requires Nix).

Run inside `nix develop .#agent --command python scripts/test_nix_build_reuse.py`.
The mutations happen only in a disposable copy, never in the working tree.
"""

import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[1]
DEPS = ("coda-servers-deps", "coda-web-deps", "patient-bank-deps")
BUILDS = ("coda-servers", "coda-web", "patient-bank")


class BuildReuse(unittest.TestCase):
    def test_dependency_identity_and_packaged_inputs(self):
        with tempfile.TemporaryDirectory(prefix="coda-nix-reuse-") as directory:
            copy = Path(directory)
            for name in ("crates", "nix", "skills", "patients"):
                shutil.copytree(ROOT / name, copy / name)
            for name in ("Cargo.toml", "Cargo.lock", "platform/ios/build_bridge.rs"):
                destination = copy / name
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(ROOT / name, destination)

            def evaluate():
                expression = f'''
                  let
                    flake = builtins.getFlake {json.dumps(str(ROOT))};
                    pkgs = import flake.inputs.nixpkgs {{
                      system = "x86_64-linux";
                      overlays = [ flake.inputs.rust-overlay.overlays.default ];
                      config.allowUnfree = true;
                    }};
                    packages = import {copy}/nix/packages.nix {{
                      inherit pkgs;
                      self = flake;
                      crane = flake.inputs.crane;
                    }};
                  in builtins.mapAttrs (_: p: p.drvPath) packages
                '''
                return json.loads(subprocess.check_output(
                    ["nix", "eval", "--impure", "--json", "--expr", expression],
                    cwd=ROOT, text=True,
                ))

            def append(name, text):
                with (copy / name).open("a") as file:
                    file.write(text)

            baseline = evaluate()
            append("crates/patient-bank/src/main.rs", "\n// Reuse proof: source-only edit.\n")
            source_edit = evaluate()
            for name in DEPS:
                self.assertEqual(baseline[name], source_edit[name], name)
            for name in BUILDS:
                self.assertNotEqual(baseline[name], source_edit[name], name)

            # Non-Rust inputs must invalidate the real builds, not dependency artifacts.
            append("crates/duet-api/agent/instructions/identity.md", "\nReuse proof.\n")
            append("crates/physician-app/index.html", "\n<!-- Reuse proof. -->\n")
            assets_edit = evaluate()
            for name in DEPS:
                self.assertEqual(source_edit[name], assets_edit[name], name)
            for name in BUILDS:
                self.assertNotEqual(source_edit[name], assets_edit[name], name)

            skill = next((copy / "skills").glob("*/SKILL.md"))
            append(str(skill.relative_to(copy)), "\nReuse proof.\n")
            skills_edit = evaluate()
            for name in DEPS + BUILDS:
                self.assertEqual(assets_edit[name], skills_edit[name], name)
            self.assertNotEqual(assets_edit["physician-skills"], skills_edit["physician-skills"])

            # Lock changes must invalidate all dependency outputs.
            append("Cargo.lock", "\n# Reuse proof: changed lock input.\n")
            lock_edit = evaluate()
            for name in DEPS:
                self.assertNotEqual(skills_edit[name], lock_edit[name], name)

            # Build-profile changes must invalidate dependencies without Rust edits.
            append("Cargo.toml", '\n[profile.release]\nopt-level = 2\n')
            profile_edit = evaluate()
            for name in DEPS:
                self.assertNotEqual(lock_edit[name], profile_edit[name], name)
            print("Verified source/assets/skills reuse and lock/profile invalidation:")
            for name in DEPS:
                print(f"  {name}: {baseline[name]}")


if __name__ == "__main__":
    unittest.main()
