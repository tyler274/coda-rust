"""Concrete CLI adapters for release.py; subprocesses are injectable in tests.

Input JSON: {environment: Environment fields, candidate: Candidate fields,
commands: {repository, workflow_path, required_job_ids, runtime_artifact,
compatibility_artifact, previous_artifact, deploy_configuration,
verifier_configuration, smoke_configuration, smoke_command, receipt_path}}.
`repository` is tyler274/coda-rust; workflow_path is .github/workflows/ci.yml.
Job IDs are numeric GitHub job database IDs for this attempt, NOT display names.
required_job_ids must contain every prerequisite job (including aggregate ci and
cargo-deny); Candidate.checks values map the six proof families onto these IDs.
Artifact names MUST end in -<run_id>-<attempt>; producers use upload-artifact v4
immutable artifacts with one evidence.json. No local approval boolean is proof.

Evidence produced by trusted main workflow jobs:
* build artifact: {sha, run_id, job_id, image, app, role}; the successful job must
  build, test and publish that exact digest, including OCI source revision label.
* runtime artifact: {sha, run_id, jobs: {<requirement>: <job database ID>}}.
  RUNTIME_REQUIREMENTS below names the required owner acceptance jobs. Missing or
  unavailable evidence stops release, never means success. Owners may use one
  comprehensive successful job for several requirements.
* compatibility artifact: {sha, run_id, job_id, images, previous_images,
  migration_kind: "additive", rollback_schema_images: {app: migration_digest}}.
  Successful producer must test previous runtime images against these migrations.
  On rollback, exact current migration job images must still match; no other
  migrators may write schemas under the single-writer coordination contract.
* previous artifact: {sha, run_id, job_id, services: {app: {image, revision}}}.
  Produced by successful prior release proof; this artifact is referenced via
  previous_run_id and previous_artifact from trusted workflow inputs. The prior
  workflow is checked independently; its SHA need not be today's main.

Coordination is GitHub job-level concurrency.group rust-staging-<project>,
cancel-in-progress:false, protected environment staging. ALL mutation paths
(including rollback) must use that exact same-repository group. There must be NO
other repo/operator deployer or migrator while this runs. No cross-repo exclusion
is claimed. Group configuration is trusted workflow code, not a fake cloud lock.
`acquire` verifies Actions context/run; `guard` rechecks run attempt + latest main.

IAM: deployer uses only infrastructure's services.get/update, revisions.get/list,
jobs.get/update/run, executions.get/list, operations.get, projects.get, scoped
actAs, and repository writer. Never deploy/create/replace/setIamPolicy commands.
Verifier separately needs run.services.getIamPolicy, run.jobs.getIamPolicy and
IAP policy-read authority. Smoke configuration separately authenticates the
operator-approved IAP test identity/audience. No impersonation grant is assumed.
Each gcloud invocation chooses its explicit configuration; no global config edits.

Future CI patch (do not enable before runtime/image producers exist):
- needs: every final producer job ID (image/database owners supply these IDs).
- if: github.event_name == 'push' && github.ref == 'refs/heads/main', all needs
  success; environment: staging; concurrency as above at JOB scope.
- permissions contents:read, actions:read, id-token:write (deployments:write only
  if a separate summary step uses GitHub deployments).
- authenticate three separate gcloud configurations through approved OIDC or
  operator-supported identities; do not give deployer smoke/IAM-read privilege.
- invoke python -B scripts/gcp/release.py --input <trusted-bundle> --execute;
  default invocation is offline and does not read credentials or spawn commands.
- always upload receipt_path via upload-artifact; publish a sanitized summary to
  GitHub and ENG-167. A killed runner requires inspecting service traffic before
  retry; a local receipt alone is NOT a durable GitHub artifact.
- image jobs publish before release; `publish` verifies Artifact Registry digest
  metadata, never invents tags/artifact names or pretends to upload an image.

Cloud Run describe JSON is the v1 Knative representation used by gcloud run.
All unknown configuration fields are retained in spec.resource; only enumerated
server metadata/status and release-owned fields are excluded. Operator baselines
must be exported using normalize(), never inferred from observations at release.
"""

from __future__ import annotations

import argparse
from copy import deepcopy
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile
from urllib.parse import urlsplit
from urllib.request import Request, build_opener, HTTPRedirectHandler
from urllib.error import URLError

from release import APPS, ROLES, Candidate, Environment, Orchestrator, Receipt, Rejected, require, validate

RUNTIME_REQUIREMENTS = frozenset({
    "private-sql-tls-iam-refresh", "bounded-pools-no-auto-migrate", "durable-auth-chat",
    "account-isolation-revocation", "live-provider-persistence", "model-usage-bounds",
    "packaged-real-backend-browser", "hosted-identity-no-dev-tokens",
})


class Unavailable(Rejected):
    """Unavailable command/evidence, distinct from a negative verification."""


class Commands:
    def __init__(self, run=subprocess.run):
        self.run = run

    def text(self, argv, *, input=None):
        # Never use shell=True or print args/stdout/stderr. In particular token
        # responses and smoke stdin must never be attached to exceptions/logs.
        try:
            result = self.run(argv, input=input, capture_output=True, text=True,
                              check=False, timeout=900)
        except (OSError, subprocess.SubprocessError):
            raise Unavailable("Command unavailable or timed out") from None
        if result.returncode:
            raise Unavailable("Command returned nonzero; output withheld")
        return result.stdout

    def json(self, argv, **kwargs):
        try:
            return json.loads(self.text(argv, **kwargs))
        except (ValueError, TypeError):
            raise Unavailable("Command did not return JSON; output withheld") from None


def canonical_policy(policy):
    """Ignore volatile etag, preserve bindings/conditions/audit configuration."""
    policy = deepcopy(policy)
    policy.pop("etag", None)
    for binding in policy.get("bindings", []):
        binding["members"] = sorted(binding.get("members", []))
    policy["bindings"] = sorted(policy.get("bindings", []), key=lambda x: json.dumps(x, sort_keys=True))
    return policy


def normalize(raw, *, job=False):
    raw = deepcopy(raw)
    status = raw.pop("status", {})
    metadata = raw["metadata"]
    for key in ("uid", "resourceVersion", "generation", "creationTimestamp", "selfLink"):
        metadata.pop(key, None)
    for owner in (metadata, raw["spec"]["template"].get("metadata", {})):
        annotations = owner.get("annotations", {})
        for key in ("run.googleapis.com/operation-id", "run.googleapis.com/creator",
                    "run.googleapis.com/lastModifier", "serving.knative.dev/creator",
                    "serving.knative.dev/lastModifier", "run.googleapis.com/client-name",
                    "run.googleapis.com/client-version", "run.googleapis.com/urls",
                    "run.googleapis.com/ingress-status"):
            annotations.pop(key, None)
        labels = owner.get("labels", {})
        labels.pop("cloud.googleapis.com/location", None)
        labels.pop("run.googleapis.com/startupProbeType", None)
    template = raw["spec"]["template"]
    revision = template.get("metadata", {}).pop("name", None)
    task = template["spec"]["template"]["spec"] if job else template["spec"]
    containers = task["containers"]
    require(len(containers) == 1 or (
        not job and len(containers) == 2
        and containers[1].get("name") == "otel-collector"
        and not containers[1].get("ports")
        and containers[0].get("ports")
    ), "Only the ingress product and optional infrastructure-owned otel-collector are supported")
    # Only the product image is release-owned. Preserve the complete collector
    # image/config in the normalized baseline so any sidecar drift is refused.
    container = containers[0]
    image = container.pop("image")
    env = {item["name"]: item for item in container.get("env", [])}
    secret = env["DOPPLER_TOKEN"]["valueFrom"]["secretKeyRef"]
    annotations = (template["spec"]["template"].get("metadata", {}) if job
                   else template.get("metadata", {})).get("annotations", {})
    top_annotations = metadata.get("annotations", {})
    spec = {
        "resource": raw,
        "serviceAccount": task["serviceAccountName"],
        "dopplerConfig": env["DOPPLER_CONFIG"]["value"],
        "dopplerTokenSecret": secret["name"], "dopplerTokenVersion": secret["key"],
        "vpcAccess": {"egress": annotations.get("run.googleapis.com/vpc-access-egress", "").upper().replace("-", "_")},
    }
    if not job:
        raw["spec"].pop("traffic", None)
        spec.update(iapEnabled=top_annotations.get("run.googleapis.com/iap-enabled") == "true",
                    invokerIamDisabled=top_annotations.get("run.googleapis.com/invoker-iam-disabled", "false") == "true",
                    port=container["ports"][0]["containerPort"],
                    startupProbe={"path": container["startupProbe"]["httpGet"]["path"]})
    traffic = {}
    for entry in status.get("traffic", []):
        if entry.get("percent", 0):
            require(bool(entry.get("revisionName")), "Traffic must resolve to concrete revision")
            traffic[entry["revisionName"]] = traffic.get(entry["revisionName"], 0) + entry["percent"]
    return {"spec": spec, "image": image, "revision": revision or status.get("latestCreatedRevisionName"),
            "traffic": traffic, "url": status.get("url"), "status": status}


class NoRedirect(HTTPRedirectHandler):
    def redirect_request(self, *args, **kwargs):
        return None


def ready_request(url, token):
    try:
        request = Request(url, headers={"Authorization": f"Bearer {token}"})
        with build_opener(NoRedirect).open(request, timeout=30) as response:
            return response.status == 200
    except (URLError, ValueError):
        raise Unavailable("Authenticated readiness unavailable; response withheld") from None


class CloudRunner:
    def __init__(self, env, candidate, config, *, commands=None, context=None, http=ready_request):
        self.env, self.candidate, self.config = env, candidate, config
        self.commands = commands or Commands()
        self.context = os.environ if context is None else context
        self.http = http
        self.repository = config["repository"]
        require(self.repository == "tyler274/coda-rust", "Unexpected repository")
        require(config["workflow_path"] == ".github/workflows/ci.yml", "Unexpected workflow")
        for key in ("deploy_configuration", "verifier_configuration", "smoke_configuration"):
            require(bool(re.fullmatch(r"[a-z][a-z0-9-]*", config[key])), "Explicit gcloud configurations required")
        require(len({config[k] for k in ("deploy_configuration", "verifier_configuration", "smoke_configuration")}) == 3,
                "Separate deployer/verifier/smoke configurations required")
        require(isinstance(config["smoke_command"], list) and bool(config["smoke_command"])
                and all(isinstance(x, str) and x for x in config["smoke_command"]), "Product smoke argv required")
        require(all(str(x).isdigit() for x in config["required_job_ids"])
                and bool(config["required_job_ids"]), "Exact required job database IDs required")
        require(set(candidate.checks.values()) <= {str(x) for x in config["required_job_ids"]},
                "Check families must map to required jobs")
        for key in ("runtime_artifact", "compatibility_artifact", "previous_artifact", "previous_run_id", "receipt_path"):
            require(bool(config[key]), "Missing evidence/receipt configuration")
        self.scope = ["--project", env.release_contract["project_id"], "--region", env.release_contract["region"]]
        self.active = False
        self.artifacts = {}

    def gh(self, *args):
        return self.commands.json(["gh", *args])

    def api(self, endpoint):
        return self.gh("api", f"repos/{self.repository}/{endpoint}")

    def gcloud(self, args, *, identity="deploy", regional=True):
        config = self.config[{"deploy": "deploy_configuration", "verifier": "verifier_configuration",
                              "smoke": "smoke_configuration"}[identity]]
        scope = self.scope if regional else self.scope[:2]
        return self.commands.json(["gcloud", "--configuration", config, *args, *scope, "--quiet", "--format=json"])

    def run_info(self, run_id=None, *, sha=None):
        run_id = run_id or self.candidate.run_id
        require(bool(re.fullmatch(r"[0-9]+-[0-9]+", run_id)), "Invalid workflow run-attempt")
        number, attempt = run_id.split("-")
        info = self.gh("run", "view", number, "--attempt", attempt, "--repo", self.repository,
                       "--json", "headSha,headBranch,event,status,databaseId")
        details = self.api(f"actions/runs/{number}/attempts/{attempt}")
        require(info["event"] == "push" and info["headBranch"] == "main"
                and details["path"] == self.config["workflow_path"]
                and details["run_attempt"] == int(attempt)
                and details["head_sha"] == info["headSha"], "Untrusted workflow run")
        if sha is not None:
            require(info["headSha"] == sha, "Evidence SHA mismatch")
        return info

    def jobs(self, run_id=None):
        number, attempt = (run_id or self.candidate.run_id).split("-")
        # Pagination explicitly prevents silently accepting only the first 100 jobs.
        pages = self.gh("api", f"repos/{self.repository}/actions/runs/{number}/attempts/{attempt}/jobs?per_page=100",
                        "--paginate", "--slurp")
        return {str(job["id"]): job for page in pages for job in page["jobs"]}

    def successful_job(self, job_id, *, run_id=None):
        job = self.jobs(run_id).get(str(job_id))
        require(job is not None, "Required job evidence unavailable")
        require(job["status"] == "completed" and job["conclusion"] == "success", "Required job did not succeed")
        return True

    def artifact(self, name, *, run_id=None, sha=None):
        run_id = run_id or self.candidate.run_id
        sha = sha or self.candidate.sha
        require(bool(re.fullmatch(r"[A-Za-z0-9_.-]+", name)) and name.endswith("-" + run_id),
                "Immutable attempt-specific artifact name required")
        key = (run_id, name)
        if key in self.artifacts:
            return deepcopy(self.artifacts[key])
        self.run_info(run_id, sha=sha)
        number = run_id.split("-")[0]
        pages = self.gh("api", f"repos/{self.repository}/actions/runs/{number}/artifacts?per_page=100",
                        "--paginate", "--slurp")
        matches = [a for page in pages for a in page["artifacts"] if a["name"] == name and not a["expired"]]
        require(len(matches) == 1 and matches[0].get("digest", "").startswith("sha256:"),
                "Unique immutable workflow artifact unavailable")
        with tempfile.TemporaryDirectory(prefix="release-evidence-") as directory:
            self.commands.text(["gh", "run", "download", number, "--repo", self.repository,
                                "--name", name, "--dir", directory])
            path = Path(directory) / "evidence.json"
            require(path.is_file() and not path.is_symlink() and path.stat().st_size <= 1048576,
                    "Evidence manifest missing or oversized")
            try:
                evidence = json.loads(path.read_text())
            except (ValueError, OSError):
                raise Unavailable("Evidence manifest unreadable") from None
        require(evidence["sha"] == sha and evidence["run_id"] == run_id, "Artifact identity mismatch")
        self.artifacts[key] = evidence
        return deepcopy(evidence)

    def registry_digest(self, image):
        result = self.gcloud(["artifacts", "docker", "images", "describe", image], regional=False)
        summary = result["image_summary"]
        require(summary["digest"] == image.split("@")[1]
                and summary["fully_qualified_digest"] == image, "Registry digest mismatch")
        return image

    def read(self, name):
        job = name.endswith("-migrate")
        raw = self.gcloud(["run", "jobs" if job else "services", "describe", name])
        return normalize(raw, job=job)

    def verify_revision(self, name, revision, image):
        raw = self.gcloud(["run", "revisions", "describe", revision])
        require(raw["metadata"]["labels"]["serving.knative.dev/service"] == name, "Revision belongs to other service")
        require(raw["status"]["imageDigest"] == image, "Revision digest mismatch")
        require(any(c["type"] == "Ready" and c["status"] == "True" for c in raw["status"]["conditions"]),
                "Revision not Ready")
        return True

    def call(self, operation, **a):
        if operation == "acquire":
            expected = {"GITHUB_ACTIONS": "true", "GITHUB_REPOSITORY": self.repository,
                        "GITHUB_REF": "refs/heads/main", "GITHUB_EVENT_NAME": "push",
                        "GITHUB_SHA": self.candidate.sha,
                        "GITHUB_RUN_ID": self.candidate.run_id.split("-")[0],
                        "GITHUB_RUN_ATTEMPT": self.candidate.run_id.split("-")[1]}
            require(all(self.context.get(k) == v for k, v in expected.items()), "Protected main Actions context required")
            require(a["group"] == f"rust-staging-{self.env.release_contract['project_id']}"
                    and a["cancel_in_progress"] is False, "Unexpected concurrency contract")
            self.active = True
            return self.candidate.run_id
        if operation == "release":
            self.active = False
            return None
        if operation == "guard":
            info = self.run_info(sha=a["sha"])
            current = self.api(f"actions/runs/{self.candidate.run_id.split('-')[0]}")
            valid = (self.active and a["lease"] == self.candidate.run_id and info["status"] == "in_progress"
                     and current["run_attempt"] == int(self.candidate.run_id.split("-")[1]))
            return {"lease_valid": valid, "main_sha": self.api("git/ref/heads/main")["object"]["sha"]}
        if operation == "verify_checks":
            self.run_info(sha=a["sha"])
            for job in self.config["required_job_ids"]:
                self.successful_job(job)
            runtime = self.artifact(self.config["runtime_artifact"])
            require(set(runtime["jobs"]) == RUNTIME_REQUIREMENTS, "Runtime acceptance evidence incomplete")
            for job in runtime["jobs"].values():
                self.successful_job(job)
            return True
        if operation == "verify_build":
            proof = self.artifact(a["evidence"])
            require(all(proof[k] == a[k] for k in ("sha", "image", "app", "role")), "Build manifest mismatch")
            self.successful_job(proof["job_id"])
            self.registry_digest(a["image"])
            return True
        if operation == "publish":
            # Publication belongs to the successful image producer; no fake upload.
            return self.registry_digest(a["image"])
        if operation == "read":
            return self.read(a["name"])
        if operation == "read_iam":
            name = a["name"]
            job = name.endswith("-migrate")
            policy = self.gcloud(["run", "jobs" if job else "services", "get-iam-policy", name], identity="verifier")
            result = {"run": canonical_policy(policy)}
            if not job:
                policy = self.gcloud(["iap", "web", "get-iam-policy", "--resource-type=cloud-run",
                                      "--service", name], identity="verifier")
                result["iap"] = canonical_policy(policy)
            return result
        if operation == "verify_previous":
            run_id = self.config["previous_run_id"]
            info = self.run_info(run_id)
            proof = self.artifact(self.config["previous_artifact"], run_id=run_id, sha=info["headSha"])
            self.successful_job(proof["job_id"], run_id=run_id)
            previous = proof["services"][a["app"]]
            require(all(previous[k] == a["previous"][k] for k in ("revision", "image")), "Previous healthy evidence mismatch")
            return self.verify_revision(a["app"], previous["revision"], previous["image"])
        if operation == "verify_compatibility":
            require(a["evidence"] == self.config["compatibility_artifact"], "Unexpected compatibility artifact")
            proof = self.artifact(a["evidence"])
            self.successful_job(proof["job_id"])
            require(proof["migration_kind"] == "additive" and proof["images"] == a["images"]
                    and proof["previous_images"] == {app: a["previous"][app]["image"] for app in APPS},
                    "Schema compatibility evidence mismatch")
            if a.get("actual_schema"):
                require(all(self.read(f"{app}-migrate")["image"] == proof["rollback_schema_images"][app]
                            == a["images"][app]["migration"] for app in APPS), "Actual schema images not covered")
            return True
        if operation == "read_revision":
            raw = self.gcloud(["run", "revisions", "describe", a["revision"]])
            require(raw["metadata"]["labels"]["serving.knative.dev/service"] == a["name"],
                    "Serving revision belongs to another service")
            return {"revision": a["revision"], "image": raw["status"]["imageDigest"]}
        if operation == "verify_revision":
            return self.verify_revision(a["name"], a["revision"], a["image"])
        if operation == "verify_receipt":
            # Same-workflow rollback consumes the receipt uploaded by the failed
            # release step, with success of the separate always-run uploader job.
            proof = self.artifact(self.config["rollback_artifact"])
            self.successful_job(proof["job_id"])
            require(proof["receipt"] == a["receipt"], "Rollback receipt differs from workflow artifact")
            return True
        if operation == "update_job_image":
            return self.gcloud(["run", "jobs", "update", a["name"], "--image", a["image"]])
        if operation == "execute_job":
            raw = self.gcloud(["run", "jobs", "execute", a["name"], "--wait"])
            status = raw["status"]
            success = (status.get("succeededCount") == 1 and not status.get("failedCount", 0)
                       and any(c["type"] == "Completed" and c["status"] == "True" for c in status["conditions"]))
            return {"status": "succeeded" if success else "failed", "id": raw["metadata"]["name"]}
        if operation == "update_service_image":
            require(a["no_traffic"] is True, "Candidates cannot receive traffic")
            current = self.read(a["name"])
            require(current["traffic"] == a["traffic"], "Previous traffic changed")
            # gcloud --no-traffic pins existing LATEST traffic to its resolved
            # revision. Verify resolved traffic again after the completed update.
            suffix = a["revision"].removeprefix(a["name"] + "-")
            self.gcloud(["run", "services", "update", a["name"], "--image", a["image"],
                         "--revision-suffix", suffix, "--no-traffic", "--tag", a["tag"]])
            current = self.read(a["name"])
            require(current["traffic"] == a["traffic"], "Candidate unexpectedly changed traffic")
            self.verify_revision(a["name"], a["revision"], a["image"])
            urls = [t["url"] for t in current["status"]["traffic"]
                    if t.get("tag") == a["tag"] and t.get("revisionName") == a["revision"]]
            require(len(urls) == 1, "Candidate tag URL unavailable")
            return {"ready": True, "revision": a["revision"], "candidate_url": urls[0]}
        if operation == "smoke":
            url = urlsplit(a["url"])
            base = urlsplit(self.env.release_contract["urls"][a["app"]])
            require(url.scheme == "https" and not url.username and not url.password
                    and not url.port and not url.query and not url.fragment
                    and url.hostname.endswith("---" + base.hostname) and base.hostname.endswith(".run.app"),
                    "Untrusted candidate origin")
            self.verify_revision(a["app"], a["revision"], a["image"])
            token = self.commands.text(["gcloud", "--configuration", self.config["smoke_configuration"],
                                       "auth", "print-identity-token", "--account", a["identity"],
                                       "--audiences", a["audience"], "--quiet"]).strip()
            require(bool(token) and "\n" not in token, "Smoke identity token unavailable")
            require(self.http(a["url"].rstrip("/") + a["path"], token), "IAP readiness failed")
            result = self.commands.json([*self.config["smoke_command"], "--url", a["url"],
                                         "--revision", a["revision"], "--image", a["image"]], input=token)
            require(result == {"passed": True, "revision": a["revision"], "image": a["image"]},
                    "Product smoke did not confirm candidate identity")
            return {"passed": True, "evidence": f"github:{self.candidate.run_id}:smoke:{a['revision']}"}
        if operation == "set_traffic":
            return self.gcloud(["run", "services", "update-traffic", a["name"],
                                "--to-revisions", f"{a['revision']}={a['percent']}"])
        if operation == "persist":
            path = Path(self.config["receipt_path"])
            # Workflow always uploads this path; file mode prevents casual reads.
            fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_TRUNC, 0o600)
            with os.fdopen(fd, "w") as stream:
                json.dump(a["receipt"], stream, sort_keys=True, indent=2)
                stream.flush()
                os.fsync(stream.fileno())
            return None
        raise Rejected("Unsupported release operation")


def dry_plan(env, candidate, config):
    """No external commands, evidence claims or credential access."""
    runner = CloudRunner(env, candidate, config, context={})
    return {"mode": "offline-dry-run", "evidence": "NOT CHECKED", "sha": candidate.sha,
            "concurrency_group": f"rust-staging-{env.release_contract['project_id']}",
            "steps": ["verify current main and all successful jobs", "verify runtime/build/schema artifacts",
                      "verify published registry digests", "compare infrastructure and IAM baselines",
                      "update and execute both migration jobs", "create tagged no-traffic revisions",
                      "separate-identity IAP readiness and product smoke", "promote services sequentially",
                      "reconcile traffic and upload receipt even on failure"],
            "images": candidate.images,
            "read_only_commands": [["gh", "run", "view", candidate.run_id.split("-")[0], "--repo", runner.repository],
                                   ["gh", "api", f"repos/{runner.repository}/git/ref/heads/main"],
                                   *[["gcloud", "--configuration", config["deploy_configuration"], "artifacts", "docker",
                                      "images", "describe", candidate.images[app][role], "--format=json"]
                                     for app in APPS for role in ROLES]]}


def cli(argv=None):
    parser = argparse.ArgumentParser(description="Validate/plan release offline by default; no credentials are inspected")
    parser.add_argument("--input", required=True, help="Trusted workflow-produced JSON bundle, never secret values")
    modes = parser.add_mutually_exclusive_group()
    modes.add_argument("--execute", action="store_true", help="Execute only in protected main Actions job")
    modes.add_argument("--validate", action="store_true", help="Validate inputs offline without planning")
    parser.add_argument("--rollback-receipt", help="Receipt JSON; execution also verifies workflow artifact")
    args = parser.parse_args(argv)
    try:
        bundle = json.loads(Path(args.input).read_text())
        env, candidate = Environment(**bundle["environment"]), Candidate(**bundle["candidate"])
        validate(env, candidate)
        plan = dry_plan(env, candidate, bundle["commands"])
        if not args.execute:
            print(json.dumps({"mode": "offline-validation", "evidence": "NOT CHECKED"} if args.validate else plan, indent=2))
            return 0
        runner = CloudRunner(env, candidate, bundle["commands"])
        engine = Orchestrator(runner, env)
        if args.rollback_receipt:
            prior = Receipt(**json.loads(Path(args.rollback_receipt).read_text()))
            receipt = engine.rollback(candidate, prior)
        else:
            receipt = engine.release(candidate)
        print(json.dumps({"status": receipt.status, "phase": receipt.phase}))
        return 0 if receipt.status in ("succeeded", "rolled-back") else 1
    except (Rejected, KeyError, TypeError, ValueError, OSError):
        # Never echo user/command data, credentials, raw errors or evidence bodies.
        print("Release rejected: configuration or evidence invalid/unavailable; no success asserted.")
        return 2


if __name__ == "__main__":
    raise SystemExit(cli())
