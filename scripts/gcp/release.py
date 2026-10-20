#!/usr/bin/env python3
"""Release state machine; executable adapters and offline CLI in release_commands.

Future CI integration (after producer jobs and operator provisioning):
* Main push only, protected `staging` environment, OIDC repository/owner IDs,
  ref + environment subject restricted as in infra commit 4a64ba24dc4156fe0d409ba0b6e55f48d8b21639.
* needs: ALL app, audit, format/lint, database fresh/upgrade/rerun, browser and
  four image proof jobs. Current job IDs include `ci` and `cargo-deny`; database
  and image IDs must come from their owners, never guessed here. Pass exact
  SHA-bound check run identities, not caller booleans. PR jobs have no deployment
  credentials. Required job permissions: contents:read, actions:read,
  checks:read, id-token:write, deployments:write; no infrastructure credentials.
  Add the offline unittest command from test_release.py to PR/main CI now;
  do not add an enabled deployment job until the live adapter is accepted.
* One same-repository environment concurrency group, cancel-in-progress: false,
  spanning migration, smoke, promotion and receipt persistence. The lease token
  is the GitHub run-attempt, NOT a distributed lock. All deployers/migrators must
  use that group; no other repo or operator writer may run concurrently.
  GitHub concurrency is not FIFO: reject stale queued SHAs.
* Supply operator-exported release_contract/database_contract and approved v2
  resource specs + IAM policies independently of the deployer's observed state.
  Never infer expected infrastructure from the currently running service.
* Supply exact four repository@sha256 inputs with verified source/build/proof
  provenance; no guessed Nix output names or tags. Publication preserves digest.
* Authenticate smoke with a separately authorized in-org test identity + OAuth
  audience; exercise /readyz and product auth on each tagged candidate URL.
  No token minting privilege or IAP access is implied for the deployer.
* IAM attestation requires a separate read-authorized verifier: deployer lacks
  getIamPolicy. No adapter may grant IAM or widen the deployer's permissions.
* Always persist Receipt (including failure/unknown traffic) as a restricted CI
  artifact, then summarize commit, build identities, digests, migrations,
  revisions and both URLs in GitHub/ENG-167. Never log credentials or raw errors.

Runner operations below are semantic interfaces, NOT shell command strings.
A live adapter must map them to narrow field-mask image/traffic updates, await
operations, reconcile ambiguous writes, and normalize Cloud Run v2 output into
`spec` (all mutable fields) + `image`, `revision`, `url`, `traffic`. It must not
silently discard unknown mutable fields. IAM comes from the separate verifier.
Migration execution must be awaited with no overrides and no runtime identity.

release_commands implements concrete gh/gcloud adapters and trusted workflow
artifact checks. Offline validation/dry-run is the default. --execute requires
protected Actions context and actual successful producer evidence; no force or
runtime-accepted boolean bypass exists. Runtime owners must supply real proofs.
"""

from __future__ import annotations

from copy import deepcopy
from dataclasses import dataclass, field
import re
from typing import Any, Protocol

APPS = ("physician", "duet")
ROLES = ("runtime", "migration")
CHECKS = frozenset({"apps", "audit", "format-lint", "database", "browser", "images"})

class Rejected(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise Rejected(message)


class Runner(Protocol):
    """Injected command runner; concrete implementation lives in release_commands.

    call returns JSON-like observations. `guard` validates lease ownership and
    latest remote main SHA; `verify_build`/`verify_checks` verify trusted producer
    evidence, not user-provided claims. `read` returns normalized resources;
    `read_iam` is performed by an independent IAM verifier. `smoke` must verify
    the requested revision/digest, TLS, IAP and product identity (no redirects to
    sign-in accepted as success). `persist` durably stores the receipt.
    """

    def call(self, operation: str, **arguments: Any) -> Any: ...


@dataclass(frozen=True)
class Candidate:
    sha: str
    run_id: str
    images: dict[str, dict[str, str]]
    builds: dict[str, dict[str, str]]
    checks: dict[str, str]
    # Compatibility evidence must cover BOTH old app digests against the newly
    # migrated schema, not merely declare that SQL happens to be additive.
    compatibility_evidence: str


@dataclass(frozen=True)
class Environment:
    release_contract: dict[str, Any]
    database_contract: dict[str, Any]
    # These are operator-approved normalized specs, never a discovered baseline.
    resources: dict[str, dict[str, Any]]
    iam: dict[str, Any]
    smoke_identity: str
    smoke_audience: str
    iam_verifier: str


@dataclass
class Receipt:
    sha: str
    run_id: str
    images: dict[str, dict[str, str]]
    builds: dict[str, dict[str, str]]
    urls: dict[str, str]
    status: str = "pending"
    phase: str = "validate"
    migrations: dict[str, str] = field(default_factory=dict)
    revisions: dict[str, str] = field(default_factory=dict)
    previous: dict[str, dict[str, Any]] = field(default_factory=dict)
    traffic: dict[str, Any] = field(default_factory=dict)
    promoted: list[str] = field(default_factory=list)
    smoke: dict[str, str] = field(default_factory=dict)
    checks: dict[str, str] = field(default_factory=dict)
    compatibility_evidence: str = ""
    rollback: dict[str, str] = field(default_factory=dict)
    integrity: str = "unverified"
    failure: str | None = None


def validate(env: Environment, candidate: Candidate) -> None:
    c = env.release_contract
    require(c["github_environment"] == "staging" and c["workloads_enabled"] is True,
            "Only enabled staging workloads are supported")
    require(c["services"] == {app: app for app in APPS}, "Unexpected services")
    require(c["migration_jobs"] == {app: f"{app}-migrate" for app in APPS},
            "Unexpected migration jobs")
    require(c["readiness_path"] == "/readyz", "Unexpected readiness contract")
    require(set(c["urls"]) == set(APPS), "Both service URLs required")
    require(bool(env.smoke_identity and env.smoke_audience and env.iam_verifier),
            "Separate smoke identity, audience and IAM verifier required")
    require(env.smoke_identity != c["deployer_service_account"],
            "Deployer has no IAP access")
    require(env.iam_verifier != c["deployer_service_account"],
            "Deployer has no IAM-policy read authority")
    require(bool(re.fullmatch(r"[0-9a-f]{40}", candidate.sha)), "Exact source SHA required")
    require(bool(re.fullmatch(r"[0-9]+-[0-9]+", candidate.run_id)), "Run ID-attempt required")
    require(set(candidate.checks) == CHECKS and all(candidate.checks.values()),
            "All required check identities required")
    require(bool(candidate.compatibility_evidence), "Additive schema compatibility proof required")
    require(set(candidate.images) == set(APPS) == set(candidate.builds), "Four images required")
    expected_names = {app for app in APPS} | {f"{app}-migrate" for app in APPS}
    require(set(env.resources) == expected_names == set(env.iam), "Four infrastructure baselines required")
    identities, configs, tokens = set(), set(), set()
    for app in APPS:
        db = env.database_contract[app]
        require(db["required_pool_max"] == 4 and db["runtime_principal"] != db["migration_principal"],
                "Invalid database identity/pool contract")
        require(set(candidate.images[app]) == set(ROLES) == set(candidate.builds[app]),
                "Runtime and migration image/provenance required")
        for role in ROLES:
            image = candidate.images[app][role]
            prefix = re.escape(c["artifact_repository"])
            require(bool(re.fullmatch(prefix + r"/[a-z0-9][a-z0-9/_.-]*@sha256:[0-9a-f]{64}", image)),
                    "Only exact repository-scoped digests accepted")
            require(bool(candidate.builds[app][role]), "Build provenance identity required")
            name = app if role == "runtime" else f"{app}-migrate"
            spec = env.resources[name]
            identities.add(spec["serviceAccount"])
            configs.add(spec["dopplerConfig"])
            tokens.add(spec["dopplerTokenSecret"])
            require(str(spec["dopplerTokenVersion"]).isdigit() and int(spec["dopplerTokenVersion"]) > 0,
                    "Numeric pinned Doppler token version required")
            require(spec["vpcAccess"]["egress"] == "PRIVATE_RANGES_ONLY", "Private SQL routing required")
            if role == "runtime":
                require(spec["iapEnabled"] is True and spec["invokerIamDisabled"] is False,
                        "IAP and invoker IAM must remain enabled")
                require(spec["port"] == 8080 and spec["startupProbe"]["path"] == "/readyz",
                        "Port/probe mismatch")
    require(len(identities) == len(configs) == len(tokens) == 4,
            "Four distinct runtime/migrator identities, Doppler configs and token secrets required")


class Orchestrator:
    """Offline state machine. All external effects go through the injected runner."""

    def __init__(self, runner: Runner, environment: Environment):
        self.runner, self.env = runner, environment
        self.lease: str | None = None

    def call(self, operation: str, **arguments: Any) -> Any:
        return self.runner.call(operation, project=self.env.release_contract["project_id"],
                                region=self.env.release_contract["region"], **arguments)

    def guard(self, sha: str) -> None:
        result = self.call("guard", lease=self.lease, sha=sha)
        require(result == {"lease_valid": True, "main_sha": sha}, "Stale SHA or lost release lease")

    def verify(self) -> dict[str, Any]:
        observed = {}
        for name, expected in self.env.resources.items():
            observed[name] = self.call("read", name=name)
            require(observed[name]["spec"] == expected, f"Infrastructure drift: {name}")
            require(self.call("read_iam", name=name, identity=self.env.iam_verifier) == self.env.iam[name],
                    f"IAM drift: {name}")
        return observed

    def mutate(self, sha: str, operation: str, **arguments: Any) -> Any:
        self.guard(sha)
        self.verify()
        try:
            return self.call(operation, **arguments)
        finally:
            # Check even when an asynchronous operation returned an ambiguous error.
            self.verify()

    def release(self, candidate: Candidate) -> Receipt:
        r = Receipt(candidate.sha, candidate.run_id, deepcopy(candidate.images),
                    deepcopy(candidate.builds), deepcopy(self.env.release_contract["urls"]),
                    checks=deepcopy(candidate.checks), compatibility_evidence=candidate.compatibility_evidence)
        try:
            validate(self.env, candidate)
            self.lease = self.call("acquire", group=f"rust-staging-{self.env.release_contract['project_id']}",
                                   run_id=candidate.run_id, cancel_in_progress=False)
            require(bool(self.lease), "Exclusive release lease unavailable")
            self.guard(candidate.sha)
            require(self.call("verify_checks", sha=candidate.sha, checks=candidate.checks,
                              event="push", ref="refs/heads/main") is True, "CI proofs rejected")
            observed = self.verify()
            for app in APPS:
                previous = observed[app]
                if len(previous["traffic"]) == 1:
                    serving_revision = next(iter(previous["traffic"]))
                    if serving_revision != previous["revision"]:
                        # A failed no-traffic candidate may be the latest template;
                        # rollback must target the actual serving revision/image.
                        serving = self.call("read_revision", name=app, revision=serving_revision)
                        previous = {**previous, **serving}
                require(previous["traffic"] == {previous["revision"]: 100},
                        "An explicit single healthy previous revision is required")
                require(self.call("verify_previous", app=app, previous=previous) is True,
                        "Previous revision lacks healthy release evidence")
                r.previous[app] = {key: deepcopy(previous[key]) for key in ("image", "revision", "traffic")}
            require(self.call("verify_compatibility", evidence=candidate.compatibility_evidence,
                              sha=candidate.sha, previous=r.previous, images=candidate.images) is True,
                    "Additive schema/previous-image compatibility rejected")
            # Verify all four builds before publishing any artifact.
            for app in APPS:
                for role in ROLES:
                    require(self.call("verify_build", sha=candidate.sha, app=app, role=role,
                                      image=candidate.images[app][role], evidence=candidate.builds[app][role]) is True,
                            "Build identity rejected")
            r.phase = "publish"
            for app in APPS:
                for role in ROLES:
                    image = candidate.images[app][role]
                    require(self.mutate(candidate.sha, "publish", image=image,
                                        evidence=candidate.builds[app][role]) == image, "Published digest mismatch")
            r.phase = "migrate"
            for app in APPS:
                name = f"{app}-migrate"
                image = candidate.images[app]["migration"]
                self.mutate(candidate.sha, "update_job_image", name=name, image=image)
                require(self.call("read", name=name)["image"] == image, "Job digest mismatch")
                r.migrations[app] = "unknown"
                execution = self.mutate(candidate.sha, "execute_job", name=name, wait=True)
                require(execution.get("status") == "succeeded" and execution.get("id"), "Migration failed")
                r.migrations[app] = execution["id"]
            r.phase = "candidate"
            for app in APPS:
                revision = f"{app}-r{candidate.sha[:12]}-{candidate.run_id}"
                r.revisions[app] = revision
                result = self.mutate(candidate.sha, "update_service_image", name=app,
                                     image=candidate.images[app]["runtime"], revision=revision,
                                     no_traffic=True, tag=f"candidate-{candidate.run_id}",
                                     traffic=r.previous[app]["traffic"])
                require(result.get("ready") is True and result.get("revision") == revision,
                        "Candidate revision not ready")
                current = self.call("read", name=app)
                require(current["image"] == candidate.images[app]["runtime"] and
                        current["traffic"] == r.previous[app]["traffic"], "Candidate changed image/traffic unexpectedly")
                url = result.get("candidate_url", "")
                require(url.startswith("https://") and url != r.urls[app], "Dedicated HTTPS candidate URL required")
                proof = self.call("smoke", app=app, url=url, revision=revision,
                                  image=candidate.images[app]["runtime"], path="/readyz",
                                  identity=self.env.smoke_identity, audience=self.env.smoke_audience)
                require(proof.get("passed") is True and proof.get("evidence"), "Authenticated candidate smoke failed")
                r.smoke[app] = proof["evidence"]
            # Both migrations and both candidate smokes precede ANY traffic change.
            r.phase = "promote"
            for app in APPS:
                self.mutate(candidate.sha, "set_traffic", name=app, revision=r.revisions[app], percent=100)
                traffic = self.call("read", name=app)["traffic"]
                require(traffic == {r.revisions[app]: 100}, "Promotion not observed")
                r.promoted.append(app)
            r.status = "succeeded"
        except Exception:
            # Do not include exception text: runners may contain credentials/URLs.
            r.failure = f"Release stopped during {r.phase}; inspect restricted runner evidence"
            r.status = "failed"
        finally:
            self.finish(r)
        return r

    def finish(self, r: Receipt) -> None:
        if not self.lease:
            # Rejected local/configuration invocations must not probe cloud state
            # or read credentials while attempting failure reconciliation.
            r.integrity = "not-inspected"
            self.call("persist", receipt=deepcopy(r.__dict__))
            return
        try:
            self.verify()
            r.integrity = "verified"
        except Exception:
            r.integrity = "unknown-or-drifted"
            r.status = "failed"
        for app in APPS:
            try:
                r.traffic[app] = self.call("read", name=app)["traffic"]
            except Exception:
                r.traffic[app] = "unknown"
        active = [app for app in APPS if r.traffic.get(app) == {r.revisions.get(app): 100}]
        unknown = any(r.traffic.get(app) == "unknown" for app in APPS)
        if r.phase == "rollback":
            restored = [app for app in APPS
                        if r.traffic.get(app) == r.previous.get(app, {}).get("traffic")]
            if len(restored) != 2 or r.integrity != "verified":
                r.status = "rollback-partial-or-unverified"
        elif unknown or (active and (len(active) != 2 or r.status != "succeeded")):
            r.status = "partial-or-unverified"
        elif r.status == "succeeded" and len(active) != 2:
            r.status = "failed"
        try:
            # Persistence failure is fatal, never a successful return with lost evidence.
            self.call("persist", receipt=deepcopy(r.__dict__))
        finally:
            if self.lease:
                self.call("release", lease=self.lease)
                self.lease = None

    def rollback(self, candidate: Candidate, prior: Receipt) -> Receipt:
        """Traffic-only recovery; never reverse a migration or promise atomicity.

        verify_receipt must load the durable receipt by identity and reject caller
        edits. Compatibility must include the schema actually reached, including
        partially completed migrations. Unknown schema compatibility fails closed.
        """
        r = deepcopy(prior)
        r.status, r.phase, r.failure = "pending", "rollback", None
        try:
            validate(self.env, candidate)
            require(candidate.sha == prior.sha and candidate.images == prior.images, "Receipt mismatch")
            self.lease = self.call("acquire", group=f"rust-staging-{self.env.release_contract['project_id']}",
                                   run_id=candidate.run_id, cancel_in_progress=False)
            require(bool(self.lease), "Exclusive release lease unavailable")
            self.guard(candidate.sha)
            require(self.call("verify_receipt", receipt=prior.__dict__) is True, "Untrusted rollback receipt")
            require(self.call("verify_compatibility", evidence=candidate.compatibility_evidence,
                              sha=candidate.sha, previous=prior.previous, images=candidate.images,
                              actual_schema=True) is True, "Rollback not proven additive-schema compatible")
            for app in APPS:
                target = prior.previous[app]
                require(self.call("verify_revision", name=app, revision=target["revision"],
                                  image=target["image"]) is True, "Rollback revision/digest unavailable")
            for app in APPS:
                target = prior.previous[app]
                self.mutate(candidate.sha, "set_traffic", name=app, revision=target["revision"], percent=100)
                require(self.call("read", name=app)["traffic"] == target["traffic"], "Rollback unconfirmed")
                r.rollback[app] = target["revision"]
            r.status = "rolled-back"
        except Exception:
            r.status = "rollback-failed"
            r.failure = "Rollback stopped; inspect per-service traffic and restricted runner evidence"
        finally:
            self.finish(r)
        return r


def main(argv=None) -> int:
    from release_commands import cli
    return cli(argv)


if __name__ == "__main__":
    raise SystemExit(main())
