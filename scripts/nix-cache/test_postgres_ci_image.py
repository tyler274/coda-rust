"""Exact derivation addressing, private registry reads and workflow authority."""

import base64
import io
import json
from pathlib import Path
import unittest
from unittest.mock import Mock, patch
import urllib.error

import yaml

import postgres_ci_image as image


ROOT = Path(__file__).resolve().parents[2]
HASH = "0123456789abcdfghijklmnpqrsvwxyz"
DIGEST = "sha256:" + "a" * 64


def response(body=b"", headers=None):
    result = io.BytesIO(body)
    result.headers = headers or {}
    return result


class ImageTests(unittest.TestCase):
    def test_tag_uses_complete_image_derivation_not_shell(self):
        with patch.object(image.subprocess, "check_output", return_value=f"/nix/store/{HASH}-stream-postgres-ci.drv\n") as evaluate:
            self.assertEqual(image.image_tag(), f"drv-{HASH}")
        self.assertEqual(evaluate.call_args.args[0], [
            "nix", "eval", "--raw", ".#packages.x86_64-linux.postgres-ci-image.drvPath",
        ])
        changed = "1" + HASH[1:]
        self.assertNotEqual(image.derivation_tag(f"/nix/store/{HASH}-image.drv"),
                            image.derivation_tag(f"/nix/store/{changed}-image.drv"))

    def test_invalid_derivations_cannot_inject_output_or_tags(self):
        for path in ("", f"/nix/store/{HASH}-image", f"/nix/store/{HASH}-image.drv\nimage=evil",
                     f"/tmp/{HASH}-image.drv", f"/nix/store/{'e' * 32}-image.drv"):
            with self.subTest(path=path), self.assertRaises(ValueError):
                image.derivation_tag(path)

    def test_resolves_private_tag_with_authenticated_pull_scope(self):
        open_url = Mock(side_effect=[response(json.dumps({"token": "registry-pull-token"}).encode()),
                                     response(headers={"Docker-Content-Digest": DIGEST})])
        self.assertEqual(image.registry_digest(f"drv-{HASH}", "actor", "github-secret", open_url=open_url), DIGEST)
        token_request, manifest_request = [call.args[0] for call in open_url.call_args_list]
        self.assertEqual(token_request.headers["Authorization"],
                         "Basic " + base64.b64encode(b"actor:github-secret").decode())
        self.assertNotIn("github-secret", token_request.full_url)
        self.assertIn("%3Apull", token_request.full_url)
        self.assertEqual(manifest_request.method, "HEAD")
        self.assertEqual(manifest_request.full_url, f"https://ghcr.io/v2/{image.REPOSITORY}/manifests/drv-{HASH}")
        self.assertEqual(manifest_request.headers["Authorization"], "Bearer registry-pull-token")

    def test_registry_errors_are_not_missing_or_secret_leaks(self):
        for phase in ("token", "manifest"):
            for code in (401, 403, 404, 429, 500):
                if phase == "manifest" and code == 404:
                    continue
                error = urllib.error.HTTPError("https://ghcr.io", code, "github-secret", {}, None)
                replies = [error] if phase == "token" else [response(b'{"token":"secret-pull-token"}'), error]
                with self.subTest(phase=phase, code=code), self.assertRaises(ValueError) as caught:
                    image.registry_digest(f"drv-{HASH}", "actor", "github-secret", open_url=Mock(side_effect=replies))
                self.assertNotIsInstance(caught.exception, image.MissingImage)
                self.assertNotIn("github-secret", str(caught.exception))
                self.assertNotIn("secret-pull-token", str(caught.exception))
                self.assertIn("not treated as a missing image", str(caught.exception))

    def test_only_manifest_404_is_missing(self):
        error = urllib.error.HTTPError("https://ghcr.io", 404, "secret-body", {}, None)
        replies = [response(b'{"token":"pull-token"}'), error]
        with self.assertRaisesRegex(image.MissingImage, "forks cannot publish"):
            image.registry_digest(f"drv-{HASH}", "actor", "github-secret", open_url=Mock(side_effect=replies))

    def test_inspect_bootstraps_missing_only_and_resolve_fails_closed(self):
        with patch.object(image, "image_tag", return_value=f"drv-{HASH}"), patch.object(image, "registry_digest") as lookup:
            lookup.side_effect = image.MissingImage("missing")
            self.assertEqual(image.select("inspect"), {"tag": f"{image.IMAGE}:drv-{HASH}", "missing": "true"})
            with self.assertRaises(image.MissingImage):
                image.select("resolve")
            lookup.side_effect = ValueError("inaccessible")
            with self.assertRaisesRegex(ValueError, "inaccessible"):
                image.select("inspect")
            lookup.side_effect = None
            lookup.return_value = DIGEST
            self.assertEqual(image.select("resolve")["image"], f"{image.IMAGE}@{DIGEST}")

    def test_published_digest_reuses_tag_without_nix_evaluation(self):
        with patch.object(image, "image_tag") as evaluate, patch.object(image, "registry_digest", return_value=DIGEST):
            self.assertEqual(image.select("resolve", f"{image.IMAGE}:drv-{HASH}")["image"], f"{image.IMAGE}@{DIGEST}")
            evaluate.assert_not_called()
            for invalid in ("other/repo:drv-" + HASH, image.IMAGE + ":latest", image.IMAGE + ":drv-" + HASH + "\ninjected=yes"):
                with self.subTest(invalid=invalid), self.assertRaises(ValueError):
                    image.select("resolve", invalid)

    def test_missing_credentials_do_not_attempt_anonymous_reads(self):
        open_url = Mock()
        with self.assertRaisesRegex(ValueError, "forks may lack package read access"):
            image.registry_digest(f"drv-{HASH}", "actor", "", open_url=open_url)
        open_url.assert_not_called()

    def test_absent_or_malformed_manifest_digest_fails(self):
        for digest in ("", "latest", "sha256:bad", DIGEST + "\nimage=evil"):
            open_url = Mock(side_effect=[response(b'{"token":"token"}'),
                                         response(headers={"Docker-Content-Digest": digest})])
            with self.subTest(digest=digest), self.assertRaisesRegex(ValueError, "valid immutable"):
                image.registry_digest(f"drv-{HASH}", "actor", "github-secret", open_url=open_url)


class WorkflowTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.workflow = yaml.safe_load((ROOT / ".github/workflows/ci.yml").read_text())
        cls.jobs = cls.workflow["jobs"]

    def test_authorized_publication_has_isolated_credentials(self):
        publisher = self.jobs["postgres-image"]
        self.assertIn("github.ref == 'refs/heads/main'", publisher["env"]["CAN_PUBLISH"])
        self.assertIn("github.event_name == 'pull_request' && github.event.pull_request.head.repo.full_name == github.repository", publisher["env"]["CAN_PUBLISH"])
        self.assertNotIn("pull_request_target", self.workflow[True])
        self.assertEqual(publisher["permissions"], {"contents": "read", "packages": "write"})
        credential_steps = [s for s in publisher["steps"] if "GHCR_WRITE_TOKEN" in s.get("env", {})]
        self.assertEqual(len(credential_steps), 1)
        publish = credential_steps[0]
        self.assertEqual(publish["if"], "steps.address.outputs.missing == 'true' && env.CAN_PUBLISH == 'true'")
        for command in ("mktemp -d", "trap 'rm -rf", "--password-stdin", "unset GHCR_WRITE_TOKEN"):
            self.assertIn(command, publish["run"])
        self.assertNotIn("nix build", publish["run"])
        build = next(s for s in publisher["steps"] if "nix build" in s.get("run", ""))
        self.assertIn('"$stream" | docker load', build["run"])
        self.assertNotIn("github.token", str(build))
        self.assertEqual(build["if"], "steps.address.outputs.missing == 'true' && env.CAN_PUBLISH == 'true'")
        inspect = next(s for s in publisher["steps"] if s.get("id") == "address")
        self.assertEqual(inspect["run"], "python3 scripts/nix-cache/postgres_ci_image.py inspect")
        self.assertEqual(inspect["env"], {"GHCR_READ_TOKEN": "${{ github.token }}"})
        self.assertNotIn("docker manifest inspect", str(publisher))

    def test_publication_condition_allows_same_repo_but_denies_forks(self):
        job = self.jobs["postgres-image"]
        expression = job["if"] + " && (" + job["env"]["CAN_PUBLISH"][3:-2].strip() + ")"
        for event, ref, head_repo, full, allowed in (
            ("pull_request", "refs/pull/1/merge", "owner/repo", "true", True),
            ("pull_request", "refs/pull/2/merge", "fork/repo", "true", False),
            ("pull_request_target", "refs/heads/main", "fork/repo", "true", False),
            ("push", "refs/heads/main", "", "true", True),
            ("workflow_dispatch", "refs/heads/main", "", "true", True),
            ("workflow_dispatch", "refs/heads/eng-304-ci-image", "", "true", True),
            ("schedule", "refs/heads/main", "", "true", True),
            ("push", "refs/heads/main", "", "false", False),
        ):
            substituted = expression
            for key, value in {
                "needs.scope.outputs.full": full,
                "github.event.pull_request.head.repo.full_name": head_repo,
                "github.repository": "owner/repo", "github.event_name": event, "github.ref": ref,
            }.items():
                substituted = substituted.replace(key, repr(value))
            substituted = substituted.replace("&&", "and").replace("||", "or")
            with self.subTest(event=event, ref=ref, head_repo=head_repo):
                self.assertEqual(eval(substituted, {"__builtins__": {}}), allowed)

    def test_readers_work_with_fork_permissions_and_do_not_publish(self):
        reader = self.jobs["postgres-image"]
        self.assertEqual(reader["if"], "needs.scope.outputs.full == 'true'")
        self.assertNotIn("continue-on-error", reader)
        refusal = next(s for s in reader["steps"] if "Refuse unauthorized" in s.get("name", ""))
        self.assertEqual(refusal["if"], "steps.address.outputs.missing == 'true' && env.CAN_PUBLISH != 'true'")
        self.assertIn("exit 1", refusal["run"])
        resolve = next(s for s in reader["steps"] if s.get("id") == "resolve")
        self.assertEqual(resolve["env"]["GHCR_READ_TOKEN"], "${{ github.token }}")
        self.assertIn('--image-tag "$IMAGE_TAG"', resolve["run"])
        self.assertEqual(resolve["if"], "steps.address.outputs.missing == 'true' && env.CAN_PUBLISH == 'true'")
        postgres = self.jobs["postgres"]
        self.assertEqual(postgres["permissions"]["packages"], "read")
        self.assertEqual(postgres["container"], {
            "image": "${{ needs.postgres-image.outputs.image }}",
            "credentials": {"username": "${{ github.actor }}", "password": "${{ github.token }}"},
        })
        for job in (reader, postgres):
            checkout = next(s for s in job["steps"] if s.get("uses", "").startswith("actions/checkout@"))
            self.assertIs(checkout["with"]["persist-credentials"], False)

    def test_gate_requires_resolution_and_shards(self):
        self.assertIn("postgres-image", self.jobs["result"]["needs"])
        self.assertIn("postgres", self.jobs["result"]["needs"])
        self.assertNotIn("postgres-image-publish", self.jobs)
        job = self.jobs["postgres-image"]
        self.assertEqual(job["needs"], ["scope"])
        self.assertEqual(sum(s.get("uses", "").startswith("cachix/install-nix-action@") for s in job["steps"]), 1)
        self.assertEqual(job["outputs"]["image"], "${{ steps.address.outputs.image || steps.resolve.outputs.image }}")
        self.assertIn("postgres-image", self.jobs["postgres"]["needs"])

    def test_container_uses_baked_environment_service_dns_and_same_tests(self):
        postgres = self.jobs["postgres"]
        self.assertEqual(postgres["defaults"]["run"]["shell"], "bash --noprofile --norc -eo pipefail {0}")
        self.assertEqual(postgres["env"], {
            "BASH_ENV": "/etc/coda/postgres-ci.env", "CODA_REQUIRE_POSTGRES_TESTS": "1",
            "CARGO_HOME": "${{ github.workspace }}/.ci-cargo",
            "CARGO_INCREMENTAL": "0", "CARGO_PROFILE_DEV_DEBUG": "0", "CARGO_PROFILE_TEST_DEBUG": "0",
        })
        self.assertEqual(postgres["strategy"]["matrix"]["shard"], [1, 2])
        self.assertNotIn("nix develop", str(postgres))
        self.assertNotIn("install-nix", str(postgres))
        self.assertNotIn("localhost", str(postgres))
        self.assertNotIn("job.services", str(postgres))
        proof = next(s for s in postgres["steps"] if "bash scripts/check-postgres.sh" in s.get("run", ""))
        self.assertNotIn("if", proof)
        for key, value in proof["env"].items():
            if key.endswith("DATABASE_URL"):
                self.assertIn("@postgres:5432/", value)
        cache = next(s for s in postgres["steps"] if s.get("id") == "postgres-cache")
        self.assertIn("nix/postgres-ci-image.nix", cache["with"]["key"])


if __name__ == "__main__":
    unittest.main()
