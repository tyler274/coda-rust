"""Archive transport regressions; no cloud, registry credentials or Nix build."""

import hashlib
import io
import json
from pathlib import Path
import shutil
import tarfile
import tempfile
import unittest

from staging_images import REPOSITORIES, package, publish, verify


REVISION = "a" * 40


def docker_archive(path, repository, revision=REVISION):
    layer = io.BytesIO()
    with tarfile.open(fileobj=layer, mode="w") as archive:
        payload = repository.encode()
        entry = tarfile.TarInfo("tested-payload")
        entry.size = len(payload)
        archive.addfile(entry, io.BytesIO(payload))
    layer_bytes = layer.getvalue()
    config = json.dumps({
        "architecture": "amd64", "os": "linux",
        "config": {"Labels": {"org.opencontainers.image.revision": revision}},
        "rootfs": {"type": "layers", "diff_ids": ["sha256:" + hashlib.sha256(layer_bytes).hexdigest()]},
    }).encode()
    config_name = hashlib.sha256(config).hexdigest() + ".json"
    manifest = [{"Config": config_name, "RepoTags": [f"{repository}:{revision}"],
                 "Layers": ["layer.tar"]}]
    with tarfile.open(path, "w:gz") as archive:
        for name, data in [("manifest.json", json.dumps(manifest).encode()),
                           (config_name, config), ("layer.tar", layer_bytes)]:
            entry = tarfile.TarInfo(name)
            entry.size = len(data)
            archive.addfile(entry, io.BytesIO(data))


class StagingImages(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.store = self.root / "store"
        self.store.mkdir()
        self.paths = self.root / "image-paths"
        self.bundle = self.root / "downloaded"
        self.originals = {}
        for key, repository in REPOSITORIES.items():
            archive = self.store / f"store-hash-{key}.tar.gz"
            docker_archive(archive, repository)
            self.originals[key] = archive.read_bytes()
        # The proof job also checks a job image that staging does not deploy.
        docker_archive(self.store / "privacy-job.tar.gz", "coda-duet-privacy-delivery")
        self.paths.write_text("".join(f"{path}\n" for path in self.store.iterdir()))

    def test_transferred_bundle_preserves_tested_bytes_without_source_store(self):
        package(self.paths, self.bundle, REVISION)
        shutil.rmtree(self.store)
        manifest = verify(self.bundle, REVISION)
        for key, original in self.originals.items():
            self.assertEqual((self.bundle / manifest["images"][key]["archive"]).read_bytes(), original)
        self.assertEqual(len(list(self.bundle.glob("*.tar.gz"))), 5)

    def test_rejects_changed_missing_wrong_revision_and_swapped_images(self):
        mutations = {
            "corrupted archive": lambda: (self.bundle / "duet-runtime.tar.gz").write_bytes(b"changed"),
            "missing archive": lambda: (self.bundle / "physician-worker.tar.gz").unlink(),
            "wrong commit": lambda: self.change_manifest(lambda m: m.update(source_revision="b" * 40)),
            "missing migration": lambda: self.change_manifest(lambda m: m["images"].pop("duet-migration")),
            "swapped image": self.swap_image,
        }
        for name, mutate in mutations.items():
            with self.subTest(name=name):
                if self.bundle.exists():
                    shutil.rmtree(self.bundle)
                package(self.paths, self.bundle, REVISION)
                mutate()
                with self.assertRaises((ValueError, FileNotFoundError)):
                    verify(self.bundle, REVISION)

    def change_manifest(self, mutate):
        path = self.bundle / "manifest.json"
        manifest = json.loads(path.read_text())
        mutate(manifest)
        path.write_text(json.dumps(manifest))

    def swap_image(self):
        source = self.bundle / "physician-runtime.tar.gz"
        target = self.bundle / "duet-runtime.tar.gz"
        shutil.copyfile(source, target)
        self.change_manifest(lambda m: m["images"]["duet-runtime"].update(
            sha256=hashlib.sha256(target.read_bytes()).hexdigest()))

    def test_packaging_rejects_duplicate_missing_or_wrong_revision_builds(self):
        original = self.paths.read_text()
        examples = [original + original.splitlines()[0] + "\n",
                    "\n".join(line for line in original.splitlines() if "duet-runtime" not in line),
                    original]
        for index, paths in enumerate(examples):
            with self.subTest(index=index):
                self.paths.write_text(paths)
                with self.assertRaises(ValueError):
                    package(self.paths, self.bundle, "b" * 40 if index == 2 else REVISION)
                self.assertFalse(self.bundle.exists())

    def test_publication_validates_every_archive_before_any_registry_operation(self):
        package(self.paths, self.bundle, REVISION)
        # Corrupt the last image: no earlier image may be published first.
        (self.bundle / "duet-migration.tar.gz").write_bytes(b"changed")
        output = self.root / "release"
        with self.assertRaisesRegex(ValueError, "checksum differs: duet-migration"):
            publish(self.bundle, output, REVISION, "unused.invalid/apps")
        self.assertFalse(output.exists())


if __name__ == "__main__":
    unittest.main()
