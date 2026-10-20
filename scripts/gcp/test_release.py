"""No credentials, network, subprocesses or cloud SDK required.

Run from the Nix dev shell:
    python -m unittest discover -s scripts/gcp -p 'test_release.py' -v
Fixtures model normalized resource observations, not provisioned infrastructure.
"""

from copy import deepcopy
from dataclasses import replace
import unittest

from release import Candidate, Environment, Orchestrator, CHECKS, main

SHA = "a" * 40
DIGEST = "b" * 64
REPO = "us-east1-docker.pkg.dev/synthetic-staging/apps"
MUTATIONS = {"publish", "update_job_image", "execute_job", "update_service_image", "set_traffic"}


def fixture():
    contract = {
        "project_id": "synthetic-staging", "region": "us-east1", "github_environment": "staging",
        "deployer_service_account": "release@example.invalid", "artifact_repository": REPO,
        "workloads_enabled": True, "services": {x: x for x in ("physician", "duet")},
        "migration_jobs": {x: f"{x}-migrate" for x in ("physician", "duet")},
        "urls": {x: f"https://{x}.example.invalid" for x in ("physician", "duet")},
        "readiness_path": "/readyz",
    }
    resources, iam, images, builds, db = {}, {}, {}, {}, {}
    for app in ("physician", "duet"):
        db[app] = {"required_pool_max": 4, "runtime_principal": f"{app}-runtime",
                   "migration_principal": f"{app}-migration"}
        images[app], builds[app] = {}, {}
        for role in ("runtime", "migration"):
            name = app if role == "runtime" else f"{app}-migrate"
            resources[name] = {
                "serviceAccount": f"{app}-{role}@example.invalid",
                "dopplerConfig": f"stg_{app}_{role}", "dopplerTokenSecret": f"{app}-{role}-token",
                "dopplerTokenVersion": "3", "vpcAccess": {"egress": "PRIVATE_RANGES_ONLY", "network": "private"},
                "caps": {"maxInstances": 2, "cpu": 1, "memory": "1Gi"},
                "env": {"model_limit": "10"}, "iapEnabled": True, "invokerIamDisabled": False,
                "port": 8080, "startupProbe": {"path": "/readyz", "periodSeconds": 5},
            }
            iam[name] = {"invoker": ["iap-agent@example.invalid"], "iapAccessor": ["group:testers@example.invalid"]}
            images[app][role] = f"{REPO}/{app}-{role}@sha256:{DIGEST}"
            builds[app][role] = f"build:{app}:{role}:{SHA}"
    env = Environment(contract, db, resources, iam, "smoke@example.invalid", "operator-oauth-audience", "verifier@example.invalid")
    candidate = Candidate(SHA, "123-1", images, builds, {x: f"check:{x}:{SHA}" for x in CHECKS}, "schema-proof:123")
    return env, candidate


class MockRunner:
    def __init__(self, env):
        self.calls = []
        self.resources = {name: {"spec": deepcopy(spec), "image": f"{REPO}/{name}@sha256:{'c' * 64}",
                                 "revision": f"{name}-old", "traffic": {f"{name}-old": 100}}
                          for name, spec in env.resources.items()}
        self.iam = deepcopy(env.iam)
        self.main_sha = SHA
        self.lease_valid = True
        self.fail = None
        self.hook = None
        self.proofs = {}
        self.receipts = []

    def call(self, operation, **args):
        self.calls.append((operation, deepcopy(args)))
        if self.hook:
            self.hook(operation, args)
        if self.fail == (operation, args.get("name", args.get("app"))):
            raise RuntimeError("sensitive error must never appear in receipt")
        if operation == "acquire":
            return "exclusive-test-lease"
        if operation == "guard":
            return {"lease_valid": self.lease_valid, "main_sha": self.main_sha}
        if operation.startswith("verify_"):
            return self.proofs.get(operation, True)
        if operation == "read":
            return deepcopy(self.resources[args["name"]])
        if operation == "read_iam":
            return deepcopy(self.iam[args["name"]])
        if operation == "publish":
            return args["image"]
        if operation == "update_job_image":
            self.resources[args["name"]]["image"] = args["image"]
        elif operation == "execute_job":
            return {"status": "succeeded", "id": f"executions/{args['name']}-123"}
        elif operation == "update_service_image":
            assert args["no_traffic"] is True
            resource = self.resources[args["name"]]
            resource["image"] = args["image"]
            resource["revision"] = args["revision"]
            # Current traffic remains explicitly pinned to the old revision.
            return {"ready": True, "revision": args["revision"],
                    "candidate_url": f"https://candidate-{args['name']}.example.invalid"}
        elif operation == "smoke":
            return {"passed": True, "evidence": f"smoke:{args['app']}:{args['revision']}"}
        elif operation == "set_traffic":
            self.resources[args["name"]]["traffic"] = {args["revision"]: args["percent"]}
        elif operation == "persist":
            self.receipts.append(args["receipt"])
        elif operation != "release":
            raise AssertionError(f"Unexpected operation {operation}")


class ReleaseTests(unittest.TestCase):
    def setUp(self):
        self.env, self.candidate = fixture()
        self.runner = MockRunner(self.env)
        self.engine = Orchestrator(self.runner, self.env)

    def run_release(self, **changes):
        return self.engine.release(replace(self.candidate, **changes))

    def operations(self):
        return [op for op, _ in self.runner.calls]

    def assert_no_mutations(self):
        self.assertFalse(set(self.operations()) & MUTATIONS)

    def test_success_orders_all_gates_and_records_identity(self):
        receipt = self.run_release()
        self.assertEqual(receipt.status, "succeeded")
        self.assertEqual(receipt.integrity, "verified")
        ops = self.operations()
        self.assertLess(max(i for i, x in enumerate(ops) if x == "verify_build"), ops.index("publish"))
        self.assertLess(max(i for i, x in enumerate(ops) if x == "execute_job"), ops.index("update_service_image"))
        self.assertLess(max(i for i, x in enumerate(ops) if x == "smoke"), ops.index("set_traffic"))
        self.assertLess(ops.index("persist"), ops.index("release"))
        self.assertEqual(receipt.images, self.candidate.images)
        self.assertEqual(receipt.builds, self.candidate.builds)
        self.assertEqual(receipt.urls, self.env.release_contract["urls"])
        self.assertEqual(set(receipt.migrations), {"physician", "duet"})
        self.assertEqual(receipt.promoted, ["physician", "duet"])
        for op, args in self.runner.calls:
            if op == "smoke":
                self.assertEqual(args["identity"], self.env.smoke_identity)
                self.assertEqual(args["audience"], self.env.smoke_audience)
            if op == "execute_job":
                self.assertEqual(set(args), {"project", "region", "name", "wait"})
            self.assertNotIn("set_iam", op)

    def test_digest_only_and_repository_scoped(self):
        for image in (f"{REPO}/app:latest", f"{REPO}/app:v1@sha256:{DIGEST}",
                      f"elsewhere.invalid/app@sha256:{DIGEST}", f"{REPO}/app@sha256:abc"):
            with self.subTest(image=image):
                self.setUp()
                images = deepcopy(self.candidate.images)
                images["physician"]["runtime"] = image
                self.assertEqual(self.run_release(images=images).status, "failed")
                self.assert_no_mutations()

    def test_missing_smoke_identity_or_audience_fails_closed(self):
        for field in ("smoke_identity", "smoke_audience", "iam_verifier"):
            with self.subTest(field=field):
                self.setUp()
                self.engine.env = replace(self.env, **{field: ""})
                self.assertEqual(self.run_release().status, "failed")
                self.assert_no_mutations()

    def test_deployer_cannot_smoke_or_verify_iam(self):
        for field in ("smoke_identity", "iam_verifier"):
            self.setUp()
            self.engine.env = replace(self.env, **{field: self.env.release_contract["deployer_service_account"]})
            self.assertEqual(self.run_release().status, "failed")
            self.assert_no_mutations()

    def test_four_scoped_secrets_and_configs(self):
        for field in ("serviceAccount", "dopplerConfig", "dopplerTokenSecret"):
            self.setUp()
            self.env.resources["duet"][field] = self.env.resources["physician"][field]
            self.assertEqual(self.run_release().status, "failed")
            self.assert_no_mutations()
        self.setUp()
        self.env.resources["duet"]["dopplerTokenVersion"] = "latest"
        self.assertEqual(self.run_release().status, "failed")
        self.assert_no_mutations()

    def test_stale_sha_and_lost_lease_fail_before_publication(self):
        for stale in (True, False):
            self.setUp()
            if stale:
                self.runner.main_sha = "d" * 40
            else:
                self.runner.lease_valid = False
            self.assertEqual(self.run_release().status, "failed")
            self.assert_no_mutations()

    def test_stale_after_smoke_never_promotes(self):
        def hook(op, args):
            if op == "smoke" and args["app"] == "duet":
                self.runner.main_sha = "d" * 40
        self.runner.hook = hook
        self.assertEqual(self.run_release().status, "failed")
        self.assertNotIn("set_traffic", self.operations())

    def test_rejected_checks_build_or_compatibility_cannot_publish(self):
        for proof in ("verify_checks", "verify_build", "verify_compatibility", "verify_previous"):
            self.setUp()
            self.runner.proofs[proof] = False
            self.assertEqual(self.run_release().status, "failed")
            self.assert_no_mutations()

    def test_missing_check_cannot_publish(self):
        checks = dict(self.candidate.checks)
        checks.pop("audit")
        self.assertEqual(self.run_release(checks=checks).status, "failed")
        self.assert_no_mutations()

    def test_each_infrastructure_field_and_iam_checked_before_update(self):
        for field in self.env.resources["physician"]:
            with self.subTest(field=field):
                self.setUp()
                self.runner.resources["physician"]["spec"][field] = "drift"
                self.assertEqual(self.run_release().status, "failed")
                self.assert_no_mutations()
        self.setUp()
        self.runner.iam["duet"]["invoker"] = ["allUsers"]
        self.assertEqual(self.run_release().status, "failed")
        self.assert_no_mutations()

    def test_infrastructure_drift_during_image_update_is_caught(self):
        def hook(op, args):
            if op == "update_service_image":
                self.runner.resources[args["name"]]["spec"]["iapEnabled"] = False
        self.runner.hook = hook
        receipt = self.run_release()
        self.assertEqual(receipt.status, "failed")
        self.assertEqual(receipt.integrity, "unknown-or-drifted")
        self.assertNotIn("set_traffic", self.operations())

    def test_migration_failure_never_updates_service_or_traffic(self):
        self.runner.fail = ("execute_job", "duet-migrate")
        receipt = self.run_release()
        self.assertEqual(receipt.status, "failed")
        self.assertEqual(receipt.migrations["duet"], "unknown")
        self.assertIn("physician", receipt.migrations)
        self.assertNotIn("update_service_image", self.operations())
        self.assertNotIn("set_traffic", self.operations())
        self.assertNotIn("sensitive", repr(receipt))

    def test_failed_smoke_preserves_last_healthy_traffic(self):
        self.runner.fail = ("smoke", "duet")
        receipt = self.run_release()
        self.assertEqual(receipt.status, "failed")
        for app in ("physician", "duet"):
            self.assertEqual(receipt.traffic[app], {f"{app}-old": 100})
        self.assertNotIn("set_traffic", self.operations())

    def test_partial_promotion_is_not_atomic_or_success(self):
        self.runner.fail = ("set_traffic", "duet")
        receipt = self.run_release()
        self.assertEqual(receipt.status, "partial-or-unverified")
        self.assertEqual(receipt.promoted, ["physician"])
        self.assertEqual(receipt.traffic["duet"], {"duet-old": 100})
        self.assertEqual(self.runner.receipts[-1]["status"], "partial-or-unverified")
        # No automatic compensating migration/rollback on an uncertain result.
        self.assertEqual(self.operations().count("set_traffic"), 2)

    def test_ambiguous_write_is_reconciled_in_receipt(self):
        def hook(op, args):
            if op == "set_traffic" and args["name"] == "physician":
                self.runner.resources["physician"]["traffic"] = {args["revision"]: 100}
        self.runner.hook = hook
        self.runner.fail = ("set_traffic", "physician")
        receipt = self.run_release()
        self.assertEqual(receipt.status, "partial-or-unverified")
        self.assertEqual(receipt.promoted, [])
        self.assertEqual(receipt.traffic["physician"], {receipt.revisions["physician"]: 100})

    def test_receipt_failure_does_not_report_success_and_releases_lease(self):
        self.runner.fail = ("persist", None)
        with self.assertRaises(RuntimeError):
            self.run_release()
        self.assertEqual(self.operations()[-1], "release")

    def test_rollback_requires_actual_schema_compatibility(self):
        prior = self.run_release()
        self.runner.calls.clear()
        self.runner.proofs["verify_compatibility"] = False
        receipt = self.engine.rollback(self.candidate, prior)
        self.assertNotEqual(receipt.status, "rolled-back")
        self.assert_no_mutations()
        check = next(args for op, args in self.runner.calls if op == "verify_compatibility")
        self.assertIs(check["actual_schema"], True)

    def test_rollback_only_restores_known_revision_traffic(self):
        prior = self.run_release()
        self.runner.calls.clear()
        receipt = self.engine.rollback(self.candidate, prior)
        self.assertEqual(receipt.status, "rolled-back")
        self.assertEqual(set(self.operations()) & MUTATIONS, {"set_traffic"})
        for app in ("physician", "duet"):
            self.assertEqual(receipt.traffic[app], {f"{app}-old": 100})
            self.assertEqual(receipt.rollback[app], f"{app}-old")

    def test_untrusted_receipt_and_missing_revision_block_rollback(self):
        for proof in ("verify_receipt", "verify_revision"):
            self.setUp()
            prior = self.run_release()
            self.runner.calls.clear()
            self.runner.proofs[proof] = False
            self.assertNotEqual(self.engine.rollback(self.candidate, prior).status, "rolled-back")
            self.assert_no_mutations()

    def test_partial_rollback_remains_explicit(self):
        prior = self.run_release()
        self.runner.calls.clear()
        self.runner.fail = ("set_traffic", "duet")
        receipt = self.engine.rollback(self.candidate, prior)
        self.assertEqual(receipt.status, "rollback-partial-or-unverified")
        self.assertEqual(receipt.rollback, {"physician": "physician-old"})
        self.assertEqual(receipt.traffic["duet"], {prior.revisions["duet"]: 100})

    def test_unknown_final_traffic_never_reports_success(self):
        def hook(op, args):
            if op == "set_traffic" and args["name"] == "duet":
                self.runner.fail = ("read", "duet")
        self.runner.hook = hook
        receipt = self.run_release()
        self.assertEqual(receipt.status, "partial-or-unverified")
        self.assertEqual(receipt.traffic["duet"], "unknown")

    def test_lease_unavailable_cannot_mutate(self):
        original = self.runner.call
        self.runner.call = lambda op, **args: None if op == "acquire" else original(op, **args)
        self.assertEqual(self.run_release().status, "failed")
        self.assert_no_mutations()

    def test_published_digest_mismatch_stops_before_migrations(self):
        original = self.runner.call
        self.runner.call = lambda op, **args: "wrong-digest" if op == "publish" else original(op, **args)
        self.assertEqual(self.run_release().status, "failed")
        self.assertNotIn("execute_job", self.operations())

    def test_candidate_cannot_shift_traffic_before_smoke(self):
        def hook(op, args):
            if op == "update_service_image":
                self.runner.resources[args["name"]]["traffic"] = {args["revision"]: 100}
        self.runner.hook = hook
        receipt = self.run_release()
        self.assertEqual(receipt.status, "partial-or-unverified")
        self.assertNotIn("smoke", self.operations())
        self.assertNotIn("set_traffic", self.operations())

    def test_real_entrypoint_has_no_override(self):
        self.assertEqual(main(), 2)


if __name__ == "__main__":
    unittest.main()
