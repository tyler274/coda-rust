"""Mocked subprocess coverage; never invokes gh, gcloud, providers or HTTP."""
from contextlib import redirect_stdout
from copy import deepcopy
from dataclasses import asdict
import io
import json
from pathlib import Path
import subprocess
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

from release import Rejected
from release_commands import (CloudRunner, Commands, RUNTIME_REQUIREMENTS, Unavailable,
                              canonical_policy, cli, dry_plan, normalize)
from test_release import fixture, SHA, DIGEST


def configuration():
    return {
        "repository": "tyler274/coda-rust", "workflow_path": ".github/workflows/ci.yml",
        "required_job_ids": ["10", "20"], "runtime_artifact": "runtime-123-1",
        "compatibility_artifact": "schema-123-1", "previous_artifact": "healthy-122-1",
        "previous_run_id": "122-1", "deploy_configuration": "release",
        "verifier_configuration": "verifier", "smoke_configuration": "smoke",
        "smoke_command": ["python", "trusted-smoke.py"], "receipt_path": "unused.json",
    }


def resource(*, job=False):
    task = {"serviceAccountName": "service@example.invalid", "containers": [{
        "image": "us-east1-docker.pkg.dev/synthetic-staging/apps/physician@sha256:" + DIGEST,
        "env": [{"name": "DOPPLER_CONFIG", "value": "stg_physician_runtime"},
                {"name": "DOPPLER_TOKEN", "valueFrom": {"secretKeyRef": {"name": "token", "key": "3"}}}],
        "ports": [{"containerPort": 8080}], "startupProbe": {"httpGet": {"path": "/readyz"}},
        "resources": {"limits": {"cpu": "1", "memory": "1Gi"}},
    }]}
    template = {"metadata": {"name": "physician-old", "annotations": {
        "run.googleapis.com/vpc-access-egress": "private-ranges-only",
        "autoscaling.knative.dev/maxScale": "2", "run.googleapis.com/network-interfaces": "private"}},
        "spec": task}
    if job:
        template = {"metadata": {}, "spec": {"taskCount": 1, "parallelism": 1, "template": template}}
    return {"apiVersion": "run.googleapis.com/v1", "kind": "Job" if job else "Service",
            "metadata": {"name": "physician-migrate" if job else "physician", "generation": 5,
                         "annotations": {"run.googleapis.com/iap-enabled": "true"}},
            "spec": {"template": template, **({} if job else {"traffic": [{"latestRevision": True, "percent": 100}]})},
            "status": {"traffic": [{"revisionName": "physician-old", "percent": 100}],
                       "url": "https://physician-abc.us-east1.run.app"}}


class FakeProcess:
    def __init__(self):
        self.calls = []
        self.responses = []
        self.artifact = None

    def __call__(self, argv, **kwargs):
        assert isinstance(argv, list)
        assert "shell" not in kwargs
        self.calls.append((argv, kwargs))
        if argv[:3] == ["gh", "run", "download"]:
            directory = Path(argv[argv.index("--dir") + 1])
            (directory / "evidence.json").write_text(json.dumps(self.artifact))
            return SimpleNamespace(returncode=0, stdout="", stderr="")
        if not self.responses:
            raise AssertionError(f"Unexpected command: {argv}")
        value = self.responses.pop(0)
        if isinstance(value, Exception):
            raise value
        if isinstance(value, SimpleNamespace):
            return value
        return SimpleNamespace(returncode=0, stdout=json.dumps(value), stderr="")


class AdapterTests(unittest.TestCase):
    def setUp(self):
        self.env, self.candidate = fixture()
        self.candidate.checks.update({k: "10" for k in self.candidate.checks})
        self.config = configuration()
        self.process = FakeProcess()
        self.runner = CloudRunner(self.env, self.candidate, self.config, commands=Commands(self.process), context={})

    def run_responses(self, *, sha=SHA, attempt=1, status="in_progress", path=".github/workflows/ci.yml"):
        return [{"headSha": sha, "event": "push", "headBranch": "main", "status": status, "databaseId": 123},
                {"path": path, "run_attempt": attempt, "head_sha": sha}]

    def test_subprocess_nonzero_redacts_everything(self):
        self.process.responses = [SimpleNamespace(returncode=1, stdout="secret-token", stderr="Bearer secret")]
        with self.assertRaises(Unavailable) as error:
            self.runner.commands.json(["gcloud", "ignored"])
        self.assertNotIn("secret", str(error.exception))
        self.assertNotIn("Bearer", str(error.exception))

    def test_timeout_and_invalid_json_are_unavailable_not_success(self):
        for response in (subprocess.TimeoutExpired("secret-command", 1),
                         SimpleNamespace(returncode=0, stdout="secret-not-json", stderr="")):
            self.process.responses = [response]
            with self.assertRaises(Unavailable):
                self.runner.commands.json(["gh", "ignored"])

    def test_guard_uses_remote_main_and_attempt(self):
        self.runner.active = True
        self.process.responses = self.run_responses() + [{"run_attempt": 1}, {"object": {"sha": "b" * 40}}]
        result = self.runner.call("guard", lease="123-1", sha=SHA)
        self.assertEqual(result, {"lease_valid": True, "main_sha": "b" * 40})
        self.assertIn("repos/tyler274/coda-rust/git/ref/heads/main", self.process.calls[-1][0])

    def test_guard_rejects_superseded_attempt(self):
        self.runner.active = True
        self.process.responses = self.run_responses() + [{"run_attempt": 2}, {"object": {"sha": SHA}}]
        self.assertFalse(self.runner.call("guard", lease="123-1", sha=SHA)["lease_valid"])

    def test_wrong_workflow_or_sha_cannot_prove_run(self):
        for responses in (self.run_responses(path="other.yml"), self.run_responses(sha="c" * 40)):
            self.process.responses = responses
            with self.assertRaises(Rejected):
                self.runner.run_info(sha=SHA)

    def test_successful_jobs_require_exact_id_completed_and_success(self):
        for job in ({"id": 10, "status": "completed", "conclusion": "failure"},
                    {"id": 10, "status": "completed", "conclusion": "skipped"},
                    {"id": 10, "status": "in_progress", "conclusion": None},
                    {"id": 20, "status": "completed", "conclusion": "success"}):
            self.process.responses = [[{"jobs": [job]}]]
            with self.assertRaises(Rejected):
                self.runner.successful_job("10")
        self.process.responses = [[{"jobs": []}, {"jobs": [{"id": 10, "status": "completed", "conclusion": "success"}]}]]
        self.assertTrue(self.runner.successful_job("10"))
        self.assertIn("--paginate", self.process.calls[-1][0])

    def test_missing_runtime_artifact_cannot_be_approval_flag(self):
        self.process.responses = self.run_responses() + [
            [{"jobs": [{"id": 10, "status": "completed", "conclusion": "success"}]}],
            [{"jobs": [{"id": 20, "status": "completed", "conclusion": "success"}]}],
            *self.run_responses(), [{"artifacts": []}],
        ]
        with self.assertRaises(Rejected):
            self.runner.call("verify_checks", sha=SHA)

    def test_immutable_artifact_download_and_identity(self):
        self.process.responses = self.run_responses() + [[{"artifacts": [
            {"name": "build-123-1", "expired": False, "digest": "sha256:" + DIGEST}]}]]
        self.process.artifact = {"sha": SHA, "run_id": "123-1", "job_id": "10"}
        self.assertEqual(self.runner.artifact("build-123-1"), self.process.artifact)
        self.assertEqual(self.process.calls[-1][0][:3], ["gh", "run", "download"])
        # No arbitrary filesystem input or differently-attempted evidence accepted.
        with self.assertRaises(Rejected):
            self.runner.artifact("../build-123-1")
        with self.assertRaises(Rejected):
            self.runner.artifact("build-123-2")

    def test_registry_metadata_matches_full_digest_not_tag(self):
        image = self.candidate.images["duet"]["runtime"]
        self.process.responses = [{"image_summary": {"digest": "sha256:" + DIGEST, "fully_qualified_digest": image}}]
        self.assertEqual(self.runner.registry_digest(image), image)
        argv = self.process.calls[-1][0]
        self.assertIn(image, argv)
        self.assertNotIn("--region", argv)
        self.process.responses = [{"image_summary": {"digest": "sha256:wrong", "fully_qualified_digest": image}}]
        with self.assertRaises(Rejected):
            self.runner.registry_digest(image)

    def test_normalization_preserves_unknown_fields_and_all_caps(self):
        raw = resource()
        original = deepcopy(raw)
        first = normalize(raw)
        self.assertEqual(raw, original)
        self.assertTrue(first["spec"]["iapEnabled"])
        self.assertEqual(first["spec"]["vpcAccess"]["egress"], "PRIVATE_RANGES_ONLY")
        self.assertEqual(first["traffic"], {"physician-old": 100})
        raw["spec"]["template"]["spec"]["newSecurityField"] = "changed"
        self.assertNotEqual(first["spec"], normalize(raw)["spec"])
        raw = resource()
        raw["metadata"]["generation"] = 8
        raw["spec"]["template"]["metadata"]["name"] = "new-revision"
        raw["spec"]["template"]["spec"]["containers"][0]["image"] = "new-image"
        raw["spec"]["traffic"] = [{"revisionName": "new", "percent": 100}]
        self.assertEqual(first["spec"], normalize(raw)["spec"])
        self.assertEqual(normalize(resource(job=True), job=True)["spec"]["dopplerTokenVersion"], "3")

    def test_collector_stays_infrastructure_owned_across_product_releases(self):
        raw = resource()
        containers = raw["spec"]["template"]["spec"]["containers"]
        containers[0]["name"] = "physician"
        containers.append({"name": "otel-collector", "image": "otel:0.155.0",
                           "env": [{"name": "OTEL_GATEWAY_ENDPOINT", "value": "http://10.82.0.10:4318"}]})
        baseline = normalize(raw)["spec"]
        containers[0]["image"] = "new-product-image"
        self.assertEqual(baseline, normalize(raw)["spec"])
        containers[1]["image"] = "unapproved-collector"
        self.assertNotEqual(baseline, normalize(raw)["spec"])
        containers[1]["image"] = "otel:0.155.0"
        containers[1]["env"][0]["value"] = "http://public.invalid"
        self.assertNotEqual(baseline, normalize(raw)["spec"])
        containers[1]["ports"] = [{"containerPort": 4318}]
        with self.assertRaises(Rejected):
            normalize(raw)

    def test_immutable_image_version_survives_secret_injection(self):
        source = (Path(__file__).resolve().parents[2] / "nix/packages.nix").read_text()
        self.assertIn('"DD_VERSION=${revision}"', source)
        self.assertIn('"org.opencontainers.image.revision" = revision;', source)
        preserve = next(line for line in source.splitlines() if "--preserve-env=" in line)
        for name in ("DD_VERSION", "DD_ENV", "OTEL_EXPORTER_OTLP_ENDPOINT", "CODA_LOG_FORMAT", "CODA_TELEMETRY_CONTENT"):
            self.assertIn(name, preserve)

    def test_policy_normalization_keeps_conditions_and_audit(self):
        p = {"etag": "volatile", "bindings": [{"role": "test", "members": ["b", "a"],
                                                "condition": {"expression": "false"}}], "auditConfigs": ["x"]}
        normalized = canonical_policy(p)
        self.assertNotIn("etag", normalized)
        self.assertEqual(normalized["bindings"][0]["members"], ["a", "b"])
        self.assertEqual(normalized["bindings"][0]["condition"], {"expression": "false"})
        self.assertEqual(normalized["auditConfigs"], ["x"])

    def test_iam_reads_use_separate_configuration_and_never_set_policy(self):
        self.process.responses = [{"bindings": []}, {"bindings": []}]
        self.runner.call("read_iam", name="physician")
        for argv, _ in self.process.calls:
            self.assertEqual(argv[1:3], ["--configuration", "verifier"])
            self.assertIn("get-iam-policy", argv)
            self.assertNotIn("set-iam-policy", argv)

    def test_job_commands_have_no_overrides_or_create_permissions(self):
        self.process.responses = [{}, {"metadata": {"name": "execution-1"}, "status": {
            "succeededCount": 1, "conditions": [{"type": "Completed", "status": "True"}]}}]
        self.runner.call("update_job_image", name="duet-migrate", image=self.candidate.images["duet"]["migration"])
        result = self.runner.call("execute_job", name="duet-migrate")
        self.assertEqual(result["status"], "succeeded")
        argv = self.process.calls[-1][0]
        self.assertIn("--wait", argv)
        self.assertNotIn("--args", argv)
        self.assertNotIn("--set-env-vars", argv)
        self.assertNotIn("--service-account", argv)

    def test_failed_job_condition_is_not_success(self):
        self.process.responses = [{"metadata": {"name": "failed"}, "status": {
            "succeededCount": 0, "failedCount": 1, "conditions": [{"type": "Completed", "status": "False"}]}}]
        self.assertEqual(self.runner.call("execute_job", name="duet-migrate")["status"], "failed")

    def test_smoke_origin_rejected_before_token_command(self):
        for url in ("https://evil.invalid", "https://physician.example.invalid", "http://candidate.run.app"):
            with self.assertRaises(Rejected):
                self.runner.call("smoke", url=url, app="physician")
        self.assertFalse(self.process.calls)

    def test_smoke_token_only_in_stdin_not_argv_or_receipt(self):
        self.env.release_contract["urls"]["physician"] = "https://physician-abc.us-east1.run.app"
        image = self.candidate.images["physician"]["runtime"]
        self.process.responses = [{"metadata": {"labels": {"serving.knative.dev/service": "physician"}},
                                   "status": {"imageDigest": image, "conditions": [{"type": "Ready", "status": "True"}]}},
                                  SimpleNamespace(returncode=0, stdout="secret-token", stderr=""),
                                  {"passed": True, "revision": "physician-new", "image": image}]
        self.runner.http = lambda url, token: token == "secret-token" and url.endswith("/readyz")
        result = self.runner.call("smoke", url="https://candidate---physician-abc.us-east1.run.app",
                                  app="physician", revision="physician-new", image=image,
                                  path="/readyz", identity=self.env.smoke_identity, audience=self.env.smoke_audience)
        self.assertTrue(result["passed"])
        self.assertNotIn("secret-token", repr(result))
        self.assertNotIn("secret-token", repr([a for a, _ in self.process.calls]))
        self.assertEqual(self.process.calls[-1][1]["input"], "secret-token")

    def test_dry_run_and_validation_never_spawn_subprocess(self):
        bundle = {"environment": asdict(self.env), "candidate": asdict(self.candidate), "commands": self.config}
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "input.json"
            path.write_text(json.dumps(bundle))
            with patch("subprocess.run", side_effect=AssertionError("must not execute")):
                for extra in ([], ["--validate"]):
                    output = io.StringIO()
                    with redirect_stdout(output):
                        self.assertEqual(cli(["--input", str(path), *extra]), 0)
                    self.assertEqual(json.loads(output.getvalue())["evidence"], "NOT CHECKED")

    def test_missing_configs_fail_offline(self):
        for key in ("smoke_configuration", "required_job_ids", "runtime_artifact"):
            config = deepcopy(self.config)
            config[key] = [] if key == "required_job_ids" else ""
            with self.assertRaises(Rejected):
                dry_plan(self.env, self.candidate, config)

    def test_candidate_update_uses_only_image_revision_tag_and_no_traffic(self):
        old = resource()
        candidate = resource()
        image = self.candidate.images["physician"]["runtime"]
        candidate["spec"]["template"]["spec"]["containers"][0]["image"] = image
        candidate["spec"]["template"]["metadata"]["name"] = "physician-candidate"
        candidate["status"]["traffic"].append({"tag": "candidate-123-1", "revisionName": "physician-candidate",
                                                "url": "https://candidate---physician.run.app"})
        self.process.responses = [old, {}, candidate, {
            "metadata": {"labels": {"serving.knative.dev/service": "physician"}},
            "status": {"imageDigest": image, "conditions": [{"type": "Ready", "status": "True"}]}}]
        result = self.runner.call("update_service_image", name="physician", image=image,
                                  revision="physician-candidate", tag="candidate-123-1", no_traffic=True,
                                  traffic={"physician-old": 100})
        self.assertTrue(result["ready"])
        argv = self.process.calls[1][0]
        self.assertIn("--no-traffic", argv)
        self.assertIn("--revision-suffix", argv)
        self.assertNotIn("--service-account", argv)
        self.assertNotIn("--set-env-vars", argv)
        self.assertNotIn("deploy", argv)
        self.assertNotIn("replace", argv)

    def test_runtime_requirements_verify_successful_owner_jobs(self):
        self.process.responses = self.run_responses() + [
            [{"jobs": [{"id": 10, "status": "completed", "conclusion": "success"}]}],
            [{"jobs": [{"id": 20, "status": "completed", "conclusion": "success"}]}],
            *self.run_responses(), [{"artifacts": [{"name": "runtime-123-1", "expired": False,
                                                    "digest": "sha256:" + DIGEST}]}],
            *[[{"jobs": [{"id": 30, "status": "completed", "conclusion": "success"}]}]
              for _ in RUNTIME_REQUIREMENTS],
        ]
        self.process.artifact = {"sha": SHA, "run_id": "123-1", "jobs": {key: "30" for key in RUNTIME_REQUIREMENTS}}
        self.assertTrue(self.runner.call("verify_checks", sha=SHA))
        self.assertFalse(self.process.responses)

    def test_no_implicit_local_execution_context(self):
        with self.assertRaises(Rejected):
            self.runner.call("acquire", group="rust-staging-synthetic-staging", cancel_in_progress=False)
        self.assertFalse(self.process.calls)


if __name__ == "__main__":
    unittest.main()
