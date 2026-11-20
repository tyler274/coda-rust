#!/usr/bin/env python3
"""Run the available P1 and D1 scenarios through real Rust services and write a receipt.

    scripts/scenarios.py [--only P1,D1] [--require-complete] [--evidence DIR]

P1 runs over HTTP against scripts/physician-scale-stack.py: two physician-sample
sites, physician-worker, Postgres and Valkey, with the
worker answering through the local Responses stub. D1 migrates a fresh database
on that same disposable Postgres through duet-migrate, drives the duet-api
binary over HTTP with its local identity stand-in (killing and restarting it
after a write), then runs the duet-api Postgres suites. Nothing reads Doppler,
Google credentials or a model key.

receipt.json records the git revision, binary and chart digests, each step's
command identity and observation, and each capability the journey needs that
this run did not exercise. Every step is labeled controlled, real-model or
clinical-judgment; this runner produces only controlled evidence. A failed step,
a failed build or an unclean shutdown exits 1 and still writes the receipt.
Missing capabilities are printed and exit 0 unless --require-complete.
"""
import argparse
import base64
import datetime
import hashlib
import hmac
import importlib.util
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import time
import uuid

REPO = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("stack", REPO / "scripts/physician-scale-stack.py")
stack = importlib.util.module_from_spec(spec)
spec.loader.exec_module(stack)

BINARIES = ("physician-sample", "physician-worker", "physician-migrate")
AVAILABLE_JOURNEYS = {"P1", "D1", "L1"}
CHARTS = REPO / "target/patient-bank/bank"
D1_SUITES = ("database_foundation", "public_budget")
# What each D1 suite holds. The suites own the assertions; the receipt names them.
D1_TESTS = {
    "fresh_and_upgrade_keep_identity_and_enforce_account_scope":
        "fresh and upgraded schema keep identity; one account cannot read another's rows",
    "public_retrieval_budget_spans_pools":
        "the public retrieval budget holds across pools and counts misses",
}
STRIPPED = ("OPENAI_API_KEY", "CODA_ALLOW_MODEL_CALLS", "GOOGLE_APPLICATION_CREDENTIALS")


class Receipt:
    def __init__(self, path):
        self.path = path
        self.steps = []
        self.missing = []
        self.data = {"schema": "coda-scenario-receipt/1", "started_at": now(),
                     "evidence_classes": ["controlled", "real-model", "clinical-judgment"],
                     "steps": self.steps, "missing": self.missing}

    def step(self, journey, name, actor, passed, observation, command_id=None, artifact=None):
        self.steps.append({"journey": journey, "step": name, "actor": actor,
                           "command_id": command_id, "evidence": "controlled",
                           "passed": bool(passed), "observation": observation,
                           "artifact": artifact})
        print(f"{'PASS' if passed else 'FAIL'} {journey} {name}: {observation}", flush=True)

    def gap(self, journey, capability, why, owner):
        self.missing.append({"journey": journey, "capability": capability, "why": why, "owner": owner})

    def write(self):
        failed = [s for s in self.steps if not s["passed"]]
        evidence = [s["evidence"] for s in self.steps]
        self.data.update(finished_at=now(), outcome=(
            "failed" if failed or not self.steps else "incomplete" if self.missing else "complete"),
            counts={"steps": len(self.steps), "failed": len(failed), "missing": len(self.missing),
                    "real_model_steps": evidence.count("real-model"),
                    "clinical_judgment_steps": evidence.count("clinical-judgment")})
        self.path.write_text(json.dumps(self.data, indent=2, ensure_ascii=False) + "\n")


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest() if path.is_file() else None


def run(argv, **kwargs):
    return subprocess.run(argv, cwd=REPO, check=True, capture_output=True, text=True, **kwargs).stdout.strip()


def sources(bin_dir):
    status = run(["git", "status", "--porcelain", "--untracked-files=no"])
    manifest = CHARTS / "manifest.json"
    page = REPO / "target/physician-web/physician-app_bg.wasm"
    return {
        "git_sha": run(["git", "rev-parse", "HEAD"]),
        "git_dirty": bool(status),
        "rustc": run(["rustc", "-V"]),
        "binaries": {name: digest(bin_dir / name) for name in BINARIES},
        "charts": {"path": str(CHARTS.relative_to(REPO)), "manifest_sha256": digest(manifest),
                   "as_of": json.loads(manifest.read_text()).get("as_of")},
        "physician_page_wasm_sha256": digest(page),
        "image_digests": None,
        "image_digests_note": "local binaries built from git_sha; no container image was built or deployed",
        "model_execution": f"physician-worker PHYSICIAN_RESPONSES_STUB_PORT, scripts/physician-responses-stub.py answering {stack.FIXED_TEXT!r}",
    }


def http(port, method, path, body=None, bearer=None):
    try:
        return stack.http_json(port, method, path, body, bearer)
    except stack.urllib.error.HTTPError as error:
        raw = error.read()
        return error.code, (json.loads(raw) if raw.startswith(b"{") else raw.decode(errors="replace"))


def save(directory, name, value):
    directory.mkdir(parents=True, exist_ok=True)
    path = directory / f"{name}.json"
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n")
    return str(path.relative_to(REPO)) if path.is_relative_to(REPO) else str(path)


def conversation(port, session, chat):
    _, body = http(port, "GET", f"/patient-workspaces/{stack.WORKSPACE}/chats/{chat}/snapshot",
                   bearer=session)
    return body, stack.transcript_items(body)


def wait_conversation(port, session, chat, want, timeout=60):
    deadline = time.monotonic() + timeout
    while True:
        body, items = conversation(port, session, chat)
        if want(items) or time.monotonic() > deadline:
            return body, items
        time.sleep(0.2)


def p1(receipt, state, out):
    a, b = state["ports"]["site-a"], state["ports"]["site-b"]
    actor = f"{stack.EMAIL} ({stack.PRACTICE}/{stack.PHYSICIAN})"
    status, body = http(a, "GET", "/patient-roster?query=Abigail")
    receipt.step("P1", "unauthenticated roster refused", "anonymous", status == 401,
                 f"GET /patient-roster without a session answered {status}",
                 artifact=save(out, "roster-anonymous", {"status": status, "body": body}))

    session = stack.sign_in(a)
    receipt.step("P1", "proof sign-in", actor, bool(session), "site A issued a bearer session")

    status, body = http(b, "GET", "/patient-roster?query=Abigail", bearer=session)
    ids = [row.get("patient_workspace_id") for row in (body or {}).get("patients", [])]
    receipt.step("P1", "Find patient on site B", actor, status == 200 and stack.WORKSPACE in ids,
                 f"roster search 'Abigail' answered {status} with {len(ids)} patients, "
                 f"{'including' if stack.WORKSPACE in ids else 'missing'} {stack.WORKSPACE}",
                 artifact=save(out, "roster", {"status": status, "body": body}))

    status, page = http(b, "GET", f"/patient-workspaces/{stack.WORKSPACE}", bearer=session)
    identity = (page or {}).get("identity", {})
    synced = [s.get("state", {}).get("synchronized_at") for s in (page or {}).get("connected_sources", [])]
    receipt.step("P1", "patient page identity and records age", actor,
                 status == 200 and identity.get("birth_date") and identity.get("medical_record_numbers")
                 and any(synced),
                 f"DOB {identity.get('birth_date')}, MRNs {len(identity.get('medical_record_numbers') or [])}, "
                 f"records synchronized_at {synced}",
                 artifact=save(out, "workspace", {"status": status, "body": page}))

    chat, key = f"chat-scenario-{uuid.uuid4().hex}", f"scenario-{uuid.uuid4().hex}"
    command = {"type": "message.submit", "idempotency_key": key,
               "payload": {"patient_workspace_id": stack.WORKSPACE, "chat_id": chat,
                           "text": "What is her most recent A1c, and when was it drawn?",
                           "physician_time_zone": "America/New_York"}}
    status, accepted = http(a, "POST", "/commands", command, bearer=session)
    receipt.step("P1", "question accepted on site A", actor,
                 status == 200 and (accepted or {}).get("outcome") == "accepted",
                 f"message.submit answered {status} {(accepted or {}).get('outcome')}", command_id=key,
                 artifact=save(out, "submit", {"request": command, "status": status, "body": accepted}))

    replied = lambda items: ("model", stack.FIXED_TEXT) in items
    body, items = wait_conversation(b, session, chat, replied)
    receipt.step("P1", "reply read back on site B", actor, replied(items),
                 f"site B snapshot holds {items}", command_id=key,
                 artifact=save(out, "snapshot", body))

    status, again = http(a, "POST", "/commands", command, bearer=session)
    outcome = (again or {}).get("outcome")
    time.sleep(3)
    body, items = conversation(b, session, chat)
    # psql interpolates :'chat' (quoted, bound) only in stdin scripts, not in -c.
    durable = run(["psql", state["postgres"]["url"], "-At", "-v", "ON_ERROR_STOP=1",
                   "-v", f"chat={chat}"],
                  input="select state->>'status', jsonb_array_length(state->'attempts'), "
                        "jsonb_array_length(state->'messages') from physician_runtime_conversations "
                        "where chat = :'chat';\n")
    receipt.step("P1", "duplicate submit has one effect", actor,
                 status == 200 and outcome == "accepted" and durable == "completed|1|2"
                 and items == [("physician", command["payload"]["text"]), ("model", stack.FIXED_TEXT)],
                 f"the same idempotency key answered {status} {outcome}; the chat holds {len(items)} "
                 f"items; durable status|attempts|messages is {durable!r}", command_id=key,
                 artifact=save(out, "duplicate", {"status": status, "body": again,
                                                  "durable": durable, "snapshot": body}))

    status, page = http(b, "GET", f"/patient-workspaces/{stack.WORKSPACE}", bearer=session)
    chats = [c.get("chat_id") for c in (page or {}).get("chats", [])]
    receipt.step("P1", "conversation listed in history on site B", actor, chat in chats,
                 f"workspace lists {len(chats)} chats, {'including' if chat in chats else 'missing'} {chat}",
                 artifact=save(out, "workspace-after", {"status": status, "body": page}))

    revoked = stack.sign_in(a)
    signed_out, _ = http(a, "POST", "/auth/sign-out", bearer=revoked)
    status, _ = http(b, "GET", f"/patient-workspaces/{stack.WORKSPACE}", bearer=revoked)
    receipt.step("P1", "signed-out session refused on the other site", actor,
                 signed_out in (200, 204) and status == 401,
                 f"sign-out on A answered {signed_out}; the same bearer on B answered {status}")

    cited = any(key in json.dumps(body) for key in ("citation", "sources", "basis"))
    receipt.gap("P1", "chart-grounded answer with citations, Sources and Basis",
                f"the reply carries {'' if cited else 'no '}citation fields; chat offers the chart tools, but "
                "the local Responses stub answers with fixed text and calls none of them",
                "scripts/physician-scale-stack.py, scripts/physician-responses-stub.py")
    receipt.gap("P1", "reopen a cited source in the side panel",
                "no citation exists to open", "crates/physician-sample/src/live.rs, crates/physician-app/src/lib.rs")
    receipt.gap("P1", "real-model answer",
                "this runner answers with the local Responses stub's fixed text; real-model proof needs staging "
                "or CODA_LOCAL_MODEL_EXECUTION=api with an authorized key", "crates/physician-sample/src/bin/physician-worker.rs")
    receipt.gap("P1", "site and worker restart inside the scenario",
                "the stack restarts only Valkey; process restart is held by the instance_loss, "
                "recovery and stop suites in scripts/check-postgres.sh, not run here",
                "scripts/physician-scale-stack.py")
    receipt.gap("P1", "dictation", "POST /dictation answers 503 on every site", "crates/physician-sample/src/lib.rs")


def d1(receipt, state, out):
    d1_binary(receipt, state, out)
    d1_suites(receipt, state, out)
    receipt.gap("D1", "installed iPhone and Android clients",
                "sign-up, Terms, AI choice, restart and lock-screen alerts need installed apps; "
                "this runner drives duet-api over HTTP, not a phone", "crates/duet-app, crates/duet-client, crates/mobile")
    receipt.gap("D1", "Apple, Google and email sign-up",
                "the served binary verifies locally signed tokens (DUET_LOCAL_IDENTITY_SECRET), "
                "not Identity Platform", "crates/duet-api/src/identity.rs")


D1_BINARIES = ("duet-api", "duet-migrate")
D1_TERMS, D1_CONSENT = "2026-09-23", "2026-09-24"


def local_token(secret, subject, auth_time):
    encode = lambda value: base64.urlsafe_b64encode(value).rstrip(b"=")
    claims = {"iss": "duet-local", "aud": "duet-local", "sub": subject,
              "auth_time": auth_time, "exp": auth_time + 3600}
    signing = b".".join(encode(json.dumps(part, separators=(",", ":")).encode())
                        for part in ({"alg": "HS256", "typ": "JWT"}, claims))
    signature = hmac.new(secret.encode(), signing, hashlib.sha256).digest()
    return (signing + b"." + encode(signature)).decode()


class Duet:
    """One duet-api process over the scenario database, restartable on the same port."""

    def __init__(self, out, env):
        self.out, self.env, self.port, self.process, self.starts = out, env, stack.free_port(), None, 0

    def start(self, **extra):
        self.starts += 1
        log = (self.out / f"duet-api-{self.starts}.log").open("w")
        env = {**self.env, **extra, "DUET_LISTEN_ADDR": f"127.0.0.1:{self.port}"}
        self.process = subprocess.Popen([str(REPO / "target/debug/duet-api")], cwd=REPO, env=env,
                                        stdout=log, stderr=subprocess.STDOUT)
        deadline = time.monotonic() + 60
        while time.monotonic() < deadline:
            if self.process.poll() is not None:
                raise RuntimeError(f"duet-api exited {self.process.returncode}; see {log.name}")
            try:
                if http(self.port, "GET", "/health")[0] == 200:
                    return
            except OSError:
                pass
            time.sleep(0.2)
        raise RuntimeError(f"duet-api not live after 60s; see {log.name}")

    def kill(self):
        self.process.send_signal(signal.SIGKILL)
        return self.process.wait(timeout=10)

    def stop(self):
        if self.process is None or self.process.poll() is not None:
            return self.process.returncode if self.process else None
        self.process.send_signal(signal.SIGTERM)
        try:
            return self.process.wait(timeout=30)
        except subprocess.TimeoutExpired:
            self.process.kill()
            self.process.wait()
            return "killed after ignoring SIGTERM for 30s"


def d1_binary(receipt, state, out):
    out.mkdir(parents=True, exist_ok=True)
    subprocess.run(["cargo", "build", "--locked", "-p", "duet-api", "--bins"], cwd=REPO, check=True)
    bin_dir = REPO / "target/debug"
    receipt.data["sources"]["duet_binaries"] = {name: digest(bin_dir / name) for name in D1_BINARIES}
    suffix = uuid.uuid4().hex[:12]
    database, role = f"duet_scenario_{suffix}", f"duet_runtime_{suffix}"
    admin = state["postgres"]["url"]
    owner_url = admin.rsplit("/", 1)[0] + f"/{database}"
    runtime_url = f"postgresql://{role}@127.0.0.1:{state['postgres']['port']}/{database}"
    run(["psql", admin, "-v", "ON_ERROR_STOP=1", "-q", "-c", f"create role {role} login",
         "-c", f"create database {database}"])
    sql = lambda query: run(["psql", owner_url, "-At", "-v", "ON_ERROR_STOP=1"], input=query)
    base = stack.environment(os.environ, out)

    def migrate(runtime_role):
        return subprocess.run([str(bin_dir / "duet-migrate")], cwd=REPO, capture_output=True, text=True,
                              env={**base, "DUET_MIGRATION_DATABASE_URL": owner_url,
                                   "DUET_RUNTIME_ROLE": runtime_role})
    refused = migrate("postgres")
    migrated = migrate(role)
    privileges = sql(f"select rolsuper, rolcreaterole, rolbypassrls from pg_roles where rolname = '{role}';\n"
                     "select count(*) from public.duet_schema_migrations;\n"
                     f"select pg_get_userbyid(datdba) from pg_database where datname = '{database}';\n").split("\n")
    receipt.step("D1", "migrations through duet-migrate with a separate runtime role", "operator",
                 refused.returncode != 0 and migrated.returncode == 0 and privileges[0] == "f|f|f",
                 f"duet-migrate refused the owner as runtime role (exit {refused.returncode}: "
                 f"{refused.stderr.strip()!r}) and applied {privileges[1]} migrations owned by "
                 f"{privileges[2]} for runtime login {role} (super|createrole|bypassrls {privileges[0]})",
                 artifact=save(out, "migrate", {"refused": [refused.returncode, refused.stderr],
                                                "applied": [migrated.returncode, migrated.stdout, migrated.stderr],
                                                "runtime_role": privileges[0]}))

    secret = base64.urlsafe_b64encode(os.urandom(32)).decode()
    env = {**base, "DATABASE_URL": runtime_url, "DUET_LOCAL_IDENTITY_SECRET": secret,
           "DUET_PRIVACY_JOURNAL_DIRECTORY": str(out / "journal"),
           "DUET_PUBLICATION_MASTER_KEY": base64.urlsafe_b64encode(os.urandom(32)).rstrip(b"=").decode()}
    duet = Duet(out, env)
    env.update(DUET_PUBLIC_ORIGIN=f"http://127.0.0.1:{duet.port}",
               DUET_SHL_VIEWER_URL=f"http://127.0.0.1:{duet.port}/viewer")
    hosted = subprocess.run([str(bin_dir / "duet-api")], cwd=REPO, capture_output=True, text=True, timeout=30,
                            env={**env, "K_SERVICE": "duet", "CODA_ENVIRONMENT": "stg",
                                 "CODA_HOSTED_PROFILE": "staging-preview",
                                 "IDENTITY_PLATFORM_PROJECT_ID": "coda-rust-stg"})
    receipt.step("D1", "hosted startup refuses the local stand-ins", "operator",
                 hosted.returncode != 0 and "DUET_LOCAL_IDENTITY_SECRET" in hosted.stderr,
                 f"duet-api with K_SERVICE and the complete staging-preview profile plus the local "
                 f"stand-ins exited {hosted.returncode}: {hosted.stderr.strip()!r}")

    subject = f"scenario-{suffix}"
    bearer = local_token(secret, subject, int(time.time()))
    actor = f"synthetic Duet subject {subject}"
    ids = {"database": database, "runtime_role": role, "subject": subject}
    receipt.data["d1"] = ids
    try:
        duet.start()
        enrolled = [(path, http(duet.port, "PUT", path, body, bearer)[0]) for path, body in (
            ("/api/auth/account", {}),
            ("/api/auth/account/terms", {"version": D1_TERMS}),
            ("/api/auth/account/luna-consent",
             {"decision": "accepted", "version": D1_CONSENT, "expected_revision": 0}))]
        status, context = http(duet.port, "GET", "/api/auth/account/context", bearer=bearer)
        ids["account_id"] = (context or {}).get("account_id")
        receipt.step("D1", "sign up: account, Terms and AI consent", actor,
                     all(code == 200 for _, code in enrolled) and status == 200 and ids["account_id"],
                     f"{enrolled}; context answered {status} with account {ids['account_id']}",
                     artifact=save(out, "account", {"enrolled": enrolled, "context": context}))

        other_subject = f"scenario-other-{suffix}"
        other_bearer = local_token(secret, other_subject, int(time.time()))
        other_enrolled = [(path, http(duet.port, "PUT", path, body, other_bearer)[0]) for path, body in (
            ("/api/auth/account", {}),
            ("/api/auth/account/terms", {"version": D1_TERMS}),
            ("/api/auth/account/luna-consent",
             {"decision": "accepted", "version": D1_CONSENT, "expected_revision": 0}))]
        status, other_context = http(duet.port, "GET", "/api/auth/account/context", bearer=other_bearer)
        ids["other_subject"] = other_subject
        ids["other_account_id"] = (other_context or {}).get("account_id")
        receipt.step("D1", "second account has a distinct durable identity", other_subject,
                     all(code == 200 for _, code in other_enrolled) and status == 200
                     and ids["other_account_id"] and ids["other_account_id"] != ids["account_id"],
                     f"{other_enrolled}; context answered {status} with account {ids['other_account_id']}",
                     artifact=save(out, "other-account", {"enrolled": other_enrolled, "context": other_context}))

        record_id = str(uuid.uuid4())
        ids["record_id"] = record_id
        record = {"operation_id": str(uuid.uuid4()), "canonical_record_id": record_id,
                  "record_id": "scenario/walk", "entity": "health_evidence",
                  "payload": {"concept_id": "steps", "title": "Afternoon walk",
                              "measured_at": "2026-10-28T13:00:00-04:00",
                              "source": {"kind": "appleHealth"},
                              "value": {"literal": "2400", "unit": "count"}}}
        status, stored = http(duet.port, "POST", "/api/canonical-evidence", record, bearer)
        address = {key: (stored or {}).get(key)
                   for key in ("canonical_record_id", "record_id", "entity", "revision")}
        ids["address"] = address
        receipt.step("D1", "store a health record", actor,
                     status == 200 and address["canonical_record_id"] == record_id,
                     f"POST /api/canonical-evidence answered {status} with revision {address['revision']}",
                     command_id=record_id, artifact=save(out, "record", {"request": record, "status": status,
                                                                         "body": stored}))

        killed = duet.kill()
        receipt.step("D1", "duet-api killed after the write", "runner", killed is not None,
                     f"SIGKILL (exit {killed})", command_id=record_id)

        duet.start()
        status, view = http(duet.port, "POST", "/api/evidence/revision", {"address": address}, bearer)
        kept = (view or {}).get("status") == "current" and \
            ((view or {}).get("payload") or {}).get("value", {}).get("literal") == "2400"
        receipt.step("D1", "the record persists after restart", actor, status == 200 and kept,
                     f"POST /api/evidence/revision answered {status} with status {(view or {}).get('status')}",
                     command_id=record_id, artifact=save(out, "recovered", view))

        d1_account_isolation(receipt, duet.port, out, other_bearer, ids, "after restart")

        event = str(uuid.uuid4())
        ids["sign_out_event_id"] = event
        status, signed_out = http(duet.port, "POST", "/api/auth/privacy-restrictions",
                                  {"event_id": event, "operation": "sessions_revoke", "scope_version": 0},
                                  bearer)
        refused_status, refused_body = http(duet.port, "POST", "/api/evidence/revision",
                                            {"address": address}, bearer)
        receipt.step("D1", "sign out everywhere refuses the old session", actor,
                     status == 200 and refused_status == 401
                     and "session_revoked" in json.dumps(refused_body),
                     f"sessions_revoke answered {status}; the earlier bearer then answered "
                     f"{refused_status} {refused_body}", command_id=event,
                     artifact=save(out, "sign-out", {"status": status, "receipt": signed_out,
                                                     "refused": [refused_status, refused_body]}))

        stopped = duet.stop()
        duet.start()
        refused_status, _ = http(duet.port, "POST", "/api/evidence/revision", {"address": address}, bearer)
        time.sleep(1.1)  # auth_time has whole-second precision; the fence holds the sign-out instant
        fresh = local_token(secret, subject, int(time.time()))
        status, view = http(duet.port, "POST", "/api/evidence/revision", {"address": address}, fresh)
        receipt.step("D1", "after another restart the old session stays refused and a new sign-in reads the record",
                     actor, stopped == 0 and refused_status == 401 and status == 200
                     and (view or {}).get("status") == "current",
                     f"SIGTERM exit {stopped}; the earlier bearer answered {refused_status}; a new "
                     f"session answered {status} with status {(view or {}).get('status')}", command_id=record_id)
        d1_account_isolation(receipt, duet.port, out, other_bearer, ids, "after revocation and restart")
    finally:
        stopped = duet.stop()
        receipt.step("D1", "duet-api shutdown", "runner", stopped in (0, None), f"SIGTERM exit {stopped}")


def d1_account_isolation(receipt, port, out, bearer, ids, phase):
    context_status, context = http(port, "GET", "/api/auth/account/context", bearer=bearer)
    browse_status, browse = http(port, "POST", "/api/evidence/browse", {"include_removed": True}, bearer)
    read_status, read = http(port, "POST", "/api/evidence/revision", {"address": ids["address"]}, bearer)
    passed = (context_status == 200 and (context or {}).get("account_id") == ids["other_account_id"]
              and browse_status == 200 and (browse or {}).get("total_count") == 0
              and read_status == 404)
    receipt.step("D1", f"two-account refusal {phase}", ids["other_subject"], passed,
                 f"second account context {context_status}, browse {browse_status} with "
                 f"{(browse or {}).get('total_count')} records; the first account's exact record {read_status}",
                 command_id=ids["record_id"],
                 artifact=save(out, f"isolation-{phase.replace(' ', '-')}",
                               {"context": context, "browse": browse, "read": [read_status, read]}))


def d1_suites(receipt, state, out):
    env = {k: v for k, v in os.environ.items() if k not in STRIPPED and not k.startswith("DOPPLER_")}
    env["DUET_TEST_DATABASE_URL"] = state["postgres"]["url"]
    argv = ["cargo", "test", "--locked", "-p", "duet-api",
            *[arg for suite in D1_SUITES for arg in ("--test", suite)], "--", "--test-threads=1"]
    out.mkdir(parents=True, exist_ok=True)
    log = out / "duet-api-tests.log"
    result = subprocess.run(argv, cwd=REPO, env=env, capture_output=True, text=True)
    log.write_text(f"$ DUET_TEST_DATABASE_URL={state['postgres']['url']} {' '.join(argv)}\n"
                   f"exit {result.returncode}\n{result.stdout}{result.stderr}")
    seen = dict(re.findall(r"^test (\S+) \.\.\. (\w+)", result.stdout, re.M))
    artifact = str(log.relative_to(REPO)) if log.is_relative_to(REPO) else str(log)
    receipt.step("D1", "cargo test exit", "runner", result.returncode == 0,
                 f"{' '.join(argv)} exited {result.returncode}", artifact=artifact)
    for name, holds in D1_TESTS.items():
        receipt.step("D1", name, "synthetic Duet accounts", seen.get(name) == "ok",
                     f"{seen.get(name, 'not run')}: {holds}", artifact=artifact)
    unexpected = sorted(set(seen) - set(D1_TESTS))
    if unexpected:
        receipt.step("D1", "suite inventory", "runner", False,
                     f"tests not described in this runner: {unexpected}", artifact=artifact)


def start_stack(evidence, bin_dir):
    log = (evidence / "stack.out").open("w")
    return subprocess.Popen(
        [sys.executable, str(REPO / "scripts/physician-scale-stack.py"), "--bin-dir", str(bin_dir),
         "--evidence", str(evidence / "stack")], cwd=REPO, stdout=log, stderr=subprocess.STDOUT)


def wait_ready(process, evidence, timeout=600):
    path = evidence / "stack" / "stack.json"
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if process.poll() is not None:
            raise RuntimeError(f"scale stack exited {process.returncode}; see {evidence / 'stack.out'}")
        try:
            state = json.loads(path.read_text())
        except (FileNotFoundError, json.JSONDecodeError):  # absent or mid-write
            state = {}
        if "ready_at" in state:
            return state
        time.sleep(0.5)
    raise RuntimeError(f"scale stack not ready after {timeout}s; see {evidence / 'stack.out'}")


def stop_stack(process):
    if process.poll() is not None:
        return None
    process.send_signal(signal.SIGTERM)
    try:
        process.wait(timeout=60)
        return None
    except subprocess.TimeoutExpired:
        process.kill()
        process.wait()
        return "scale stack ignored SIGTERM for 60s and was killed"


def build_l1_synthetic_bank(bank_dir):
    """Build a self-contained deterministic multi-patient bank for the L1 simulation."""
    (bank_dir / "calendar").mkdir(parents=True, exist_ok=True)
    note_text = (
        "CLINIC NOTE (2026-08-12): Well-controlled essential hypertension on "
        "lisinopril 10 mg daily; baseline serum potassium 4.2 mmol/L."
    )
    encoded_note = base64.b64encode(note_text.encode()).decode()
    for source_id, mrn, given, family, dob in (
        ("MRN-ALICE", "00090001", "Alice", "Carter", "1972-03-18"),
        ("MRN-BOB", "00090002", "Bob", "Diaz", "1965-11-04"),
    ):
        chart_dir = bank_dir / "patients" / source_id
        chart_dir.mkdir(parents=True, exist_ok=True)
        patient = {
            "resourceType": "Patient",
            "id": source_id,
            "identifier": [{"system": "urn:oid:2.16.840.1.113883.19.5", "value": mrn}],
            "name": [{"family": family, "given": [given]}],
            "gender": "female" if given == "Alice" else "male",
            "birthDate": dob,
        }
        note = {
            "resourceType": "DocumentReference",
            "id": "note-baseline-1",
            "status": "current",
            "subject": {"reference": f"Patient/{source_id}"},
            "description": "Routine Primary Care Note",
            "date": "2026-08-12T12:05:00Z",
            "content": [{"attachment": {"contentType": "text/plain; charset=utf-8", "data": encoded_note}}],
        }
        (chart_dir / "Patient.ndjson").write_text(json.dumps(patient) + "\n")
        (chart_dir / "DocumentReference.ndjson").write_text(json.dumps(note) + "\n")
    baseline_appt = {
        "resourceType": "Appointment",
        "id": "appt-alice-baseline",
        "status": "booked",
        "description": "Routine Hypertension Follow-up",
        "start": "2026-08-27T19:30:00Z",
        "end": "2026-08-27T20:00:00Z",
        "participant": [{"actor": {"reference": "Patient/MRN-ALICE"}, "status": "accepted"}],
    }
    (bank_dir / "calendar" / "Appointment.ndjson").write_text(json.dumps(baseline_appt) + "\n")
    return bank_dir


def l1(receipt, bin_dir, out):
    """Run a multi-week (3-week) passive-patient L1 simulation against real Rust services."""
    out.mkdir(parents=True, exist_ok=True)
    actor = f"{stack.EMAIL} ({stack.PRACTICE}/{stack.PHYSICIAN})"
    l1_stack = stack.Stack(out / "runtime")
    (out / "runtime").mkdir(parents=True, exist_ok=True)
    base_bank = build_l1_synthetic_bank(out / "base-bank")
    work_bank = out / "work-bank"
    pg, redis, site_port, worker_port, provider_port = [stack.free_port() for _ in range(5)]
    database = f"postgresql://postgres@127.0.0.1:{pg}/postgres"
    model_settings = {}
    for role, model, effort in (("CHAT", "gpt-6.1-sol", "high"),
                                ("BACKGROUND", "gpt-6-luna", "max")):
        model_settings.update({
            f"CODA_{role}_MODEL": model,
            f"CODA_{role}_EFFORT": effort,
            f"CODA_{role}_CONTEXT_WINDOW_TOKENS": "272000",
            f"CODA_{role}_INPUT_LIMIT_TOKENS": "922000",
        })
    shared_env = model_settings | {
        "CODA_HOSTED_PROFILE": "staging-preview",
        "PHYSICIAN_LOCAL_SCALE": "1",
        "PHYSICIAN_AUTH": "required",
        "PHYSICIAN_DATABASE_URL": database,
        "PHYSICIAN_REDIS_URL": f"redis://127.0.0.1:{redis}",
        "PHYSICIAN_DURABLE_CHAT": "1",
        "PHYSICIAN_RESPONSES_STUB_PORT": str(provider_port),
        "PHYSICIAN_SYNTHETIC_CHARTS": str(base_bank),
    }
    site_env = shared_env | {
        "CODA_STAGING_SYNTHETIC_CONTROL": "enabled",
        "CODA_STAGING_SYNTHETIC_WORKDIR": str(work_bank),
        "PHYSICIAN_LISTEN_ADDR": f"127.0.0.1:{site_port}",
    }
    worker_env = shared_env | {
        "PHYSICIAN_WORKER_LISTEN_ADDR": f"127.0.0.1:{worker_port}",
        "PHYSICIAN_LISTEN_ADDR": f"127.0.0.1:{worker_port}",
        "PHYSICIAN_WORKER_RETRY_SECONDS": "1",
    }
    try:
        pg_data = out / "runtime" / "pg"
        subprocess.run(["initdb", "-D", str(pg_data), "-U", "postgres", "--auth=trust"],
                       env=l1_stack.env, check=True, stdout=subprocess.DEVNULL)
        l1_stack.start("postgres", ["postgres", "-D", str(pg_data), "-h", "127.0.0.1",
                                    "-p", str(pg), "-k", str(out / "runtime")])
        l1_stack.start("valkey", stack.valkey_argv(redis))
        l1_stack.wait(lambda: subprocess.run(["pg_isready", "-d", database], env=l1_stack.env,
                                             stdout=subprocess.DEVNULL).returncode == 0, "L1 Postgres")
        l1_stack.wait(lambda: stack.valkey_ping(l1_stack, redis), "L1 Valkey")
        subprocess.run([str(bin_dir / "physician-migrate")], env=l1_stack.env | shared_env, check=True)
        stack.seed_account(l1_stack, database)
        failures_file = out / "runtime" / "provider-failures"
        failures_file.write_text("0")
        l1_stack.start("responses", [sys.executable, str(REPO / "scripts/physician-responses-stub.py"),
                                     "--port", str(provider_port), "--failures", str(failures_file)])
        l1_stack.wait(lambda: stack.healthy(provider_port), "L1 Responses stub")
        l1_stack.start("worker", [str(bin_dir / "physician-worker")], worker_env)
        l1_stack.wait(lambda: stack.healthy(worker_port), "L1 physician-worker")
        l1_stack.start("site", [str(bin_dir / "physician-sample")], site_env)
        l1_stack.wait(lambda: stack.healthy(site_port), "L1 physician-sample")

        session = stack.sign_in(site_port)
        receipt.step("L1", "staging sign-in", actor, bool(session), "issued staging physician session")

        st, clock0 = http(site_port, "GET", "/staging-synthetic/clock", bearer=session)
        st_p, patients_resp = http(site_port, "GET", "/staging-synthetic/patients", bearer=session)
        patients = (patients_resp or {}).get("patients", [])
        alice = next((p for p in patients if p.get("source_patient_id") == "MRN-ALICE"), None)
        bob = next((p for p in patients if p.get("source_patient_id") == "MRN-BOB"), None)
        receipt.step("L1", "day-0 baseline clock and panel", actor,
                     st == 200 and st_p == 200 and (clock0 or {}).get("elapsed_seconds") == 0
                     and alice is not None and bob is not None,
                     f"baseline clock {(clock0 or {}).get('rfc3339')} with {len(patients)} panel patients",
                     artifact=save(out, "day0-baseline", {"clock": clock0, "patients": patients_resp}))
        if not alice:
            return
        alice_ws = alice["patient_workspace_id"]

        # Week 1 (+7 days): advance persisted clock, deliver chart, Spruce, Duet, visit, and undecided recommendation.
        st, clock_w1 = http(site_port, "POST", "/staging-synthetic/clock",
                            {"mode": "advance_by_seconds", "seconds": 7 * 86400}, bearer=session)
        w1_unix = (clock_w1 or {}).get("unix_seconds", 0)
        w1_date = (clock_w1 or {}).get("date_iso", "2026-09-03")
        receipt.step("L1", "week-1 clock advance", actor,
                     st == 200 and (clock_w1 or {}).get("elapsed_seconds") == 7 * 86400,
                     f"persisted clock advanced to {(clock_w1 or {}).get('rfc3339')}",
                     artifact=save(out, "week1-clock", clock_w1))

        chart_event = {
            "event_type": "free_form_chart_record",
            "patient": {"by": "source_patient_id", "value": "MRN-ALICE"},
            "record_id": "bmp-k-l1-week1",
            "kind": "lab_result",
            "title": "Basic Metabolic Panel - Potassium",
            "text": "Serum potassium 6.1 mmol/L (critical high; repeat BMP and ECG recommended).",
            "effective_at": f"{w1_date}T13:00:00Z",
        }
        st_c, saved_chart = http(site_port, "POST", "/staging-synthetic/events", chart_event, bearer=session)
        spruce_event = {
            "event_type": "spruce_patient_message",
            "patient": {"by": "source_patient_id", "value": "MRN-ALICE"},
            "conversation_id": "conv-l1-alice-1",
            "message_id": "msg-l1-alice-1",
            "sender_name": "Alice Carter",
            "sender_contact": "+15550190001",
            "text": "Hi Dr. Roger, I saw my lab result on the portal—should I hold my lisinopril?",
            "sent_at": f"{w1_date}T14:00:00Z",
        }
        st_s, saved_spruce = http(site_port, "POST", "/staging-synthetic/events", spruce_event, bearer=session)
        duet_event = {
            "event_type": "duet_patient_update",
            "patient": {"by": "source_patient_id", "value": "MRN-ALICE"},
            "record_id": "duet-bp-w1",
            "category": "home_bp",
            "title": "Home Blood Pressure Log",
            "text": "Morning BP 146/90 mmHg, heart rate 72 bpm, no chest pain or dizziness.",
            "observed_at": f"{w1_date}T12:00:00Z",
        }
        st_d, saved_duet = http(site_port, "POST", "/staging-synthetic/events", duet_event, bearer=session)
        visit_start = datetime.datetime.fromtimestamp(w1_unix + 3600, datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
        visit_end = datetime.datetime.fromtimestamp(w1_unix + 5400, datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
        visit_event = {
            "event_type": "visit_change",
            "patient": {"by": "source_patient_id", "value": "MRN-ALICE"},
            "appointment_id": "appt-l1-alice-w1",
            "status": "booked",
            "starts_at": visit_start,
            "ends_at": visit_end,
            "label": "Urgent Potassium Follow-up",
        }
        st_v, saved_visit = http(site_port, "POST", "/staging-synthetic/events", visit_event, bearer=session)
        rec_event = {
            "event_type": "medication_recommendation",
            "patient": {"by": "source_patient_id", "value": "MRN-ALICE"},
            "recommendation_id": "rec-l1-lisinopril",
            "medication": "Lisinopril 20 mg PO daily (hold pending repeat K+)",
            "rationale": "Elevated serum potassium 6.1 mmol/L with home BP 146/90",
            "detail": "Hold lisinopril until repeat potassium normalizes; recheck BMP within 48h.",
            "source_label": "Week-1 chart review",
        }
        st_r, saved_rec = http(site_port, "POST", "/staging-synthetic/events", rec_event, bearer=session)
        receipt.step("L1", "week-1 multi-source event intake", actor,
                     all(code == 201 for code in (st_c, st_s, st_d, st_v, st_r)),
                     f"statuses chart={st_c} spruce={st_s} duet={st_d} visit={st_v} rec={st_r}",
                     artifact=save(out, "week1-events", {
                         "chart": saved_chart, "spruce": saved_spruce, "duet": saved_duet,
                         "visit": saved_visit, "recommendation": saved_rec}))

        def wait_l1_measure(predicate, timeout=20.0):
            deadline = time.monotonic() + timeout
            last_status, last_body = 0, None
            while time.monotonic() < deadline:
                last_status, last_body = http(site_port, "GET", "/staging-synthetic/measure", bearer=session)
                if last_status == 200 and predicate(last_body or {}):
                    return last_status, last_body
                time.sleep(0.25)
            return last_status, last_body

        st_rnd1, rounds1 = http(site_port, "POST", "/staging-synthetic/rounds", {}, bearer=session)
        # Wait for worker/site to settle week-1 reviews, then run visit preparation
        # (re-invoking once if the async worker claimed the selection job in parallel).
        wait_l1_measure(
            lambda m: (m.get("necessary_care") or {}).get("total_reviews_completed", 0) >= 2
        )
        st_prep, prep1 = http(site_port, "POST", "/staging-synthetic/visits/prepare", {}, bearer=session)
        for _ in range(10):
            _, cur_m = http(site_port, "GET", "/staging-synthetic/measure", bearer=session)
            if ((cur_m or {}).get("necessary_care") or {}).get("visit_briefings_ready", 0) >= 1:
                break
            time.sleep(0.5)
            st_prep, prep1 = http(site_port, "POST", "/staging-synthetic/visits/prepare", {}, bearer=session)
        _, measure_w1 = wait_l1_measure(
            lambda m: (m.get("necessary_care") or {}).get("total_reviews_completed", 0) >= 2
            and (m.get("necessary_care") or {}).get("visit_briefings_ready", 0) >= 1
        )
        care_w1 = (measure_w1 or {}).get("necessary_care", {})
        receipt.step("L1", "week-1 rounds and visit preparation", actor,
                     st_rnd1 == 200 and st_prep == 200
                     and care_w1.get("total_reviews_completed", 0) >= 2
                     and care_w1.get("quiet_reviews_completed", 0) >= 1
                     and care_w1.get("assessed_reviews_completed", 0) >= 1
                     and care_w1.get("visit_briefings_ready", 0) >= 1,
                     f"rounds={st_rnd1} completed_reviews={care_w1.get('total_reviews_completed')} "
                     f"(quiet={care_w1.get('quiet_reviews_completed')}, assessed={care_w1.get('assessed_reviews_completed')}) "
                     f"briefings={care_w1.get('visit_briefings_ready')}",
                     artifact=save(out, "week1-rounds-prep", {"rounds": rounds1, "visits": prep1, "measure": measure_w1}))

        # Mid-run physician-sample restart: verify persisted simulation clock and undecided state survive restart.
        l1_stack.stop("site")
        l1_stack.start("site", [str(bin_dir / "physician-sample")], site_env)
        l1_stack.wait(lambda: stack.healthy(site_port), "L1 physician-sample after restart")
        st_clk_after, clock_after_restart = http(site_port, "GET", "/staging-synthetic/clock", bearer=session)
        st_ws_after, ws_after_restart = http(site_port, "GET", f"/patient-workspaces/{alice_ws}", bearer=session)
        undecided_after = (ws_after_restart or {}).get("undecided_recommendations", [])
        active_after = (ws_after_restart or {}).get("active_plan_items", [])
        receipt.step("L1", "persisted clock and undecided recommendation survive restart", actor,
                     st_clk_after == 200 and (clock_after_restart or {}).get("unix_seconds") == w1_unix
                     and st_ws_after == 200 and len(undecided_after) == 1 and len(active_after) == 0,
                     f"clock after restart={(clock_after_restart or {}).get('rfc3339')} "
                     f"undecided={len(undecided_after)} active_plan={len(active_after)}",
                     artifact=save(out, "after-restart", {"clock": clock_after_restart, "workspace": ws_after_restart}))

        # Week 2 (+7 days -> day 14): silence never approves undecided recommendation.
        st_w2, clock_w2 = http(site_port, "POST", "/staging-synthetic/clock",
                               {"mode": "advance_by_seconds", "seconds": 7 * 86400}, bearer=session)
        st_rnd2, rounds2 = http(site_port, "POST", "/staging-synthetic/rounds", {}, bearer=session)
        st_ws_w2, ws_w2 = http(site_port, "GET", f"/patient-workspaces/{alice_ws}", bearer=session)
        receipt.step("L1", "week-2 silence holds undecided recommendation out of active plan", actor,
                     st_w2 == 200 and (clock_w2 or {}).get("elapsed_seconds") == 14 * 86400
                     and st_rnd2 == 200 and st_ws_w2 == 200
                     and len((ws_w2 or {}).get("undecided_recommendations", [])) == 1
                     and len((ws_w2 or {}).get("active_plan_items", [])) == 0,
                     f"day-14 clock={(clock_w2 or {}).get('rfc3339')} "
                     f"undecided={len((ws_w2 or {}).get('undecided_recommendations', []))} "
                     f"active_plan={len((ws_w2 or {}).get('active_plan_items', []))}",
                     artifact=save(out, "week2-silence", {"clock": clock_w2, "rounds": rounds2, "workspace": ws_w2}))

        # Week 3 (+7 days -> day 21): explicit physician approval moves recommendation into active_plan_items,
        # then read L1 measure into receipt.json["l1_measure"].
        st_w3, clock_w3 = http(site_port, "POST", "/staging-synthetic/clock",
                               {"mode": "advance_by_seconds", "seconds": 7 * 86400}, bearer=session)
        decide_cmd = {
            "type": "plan.decide_recommendation",
            "idempotency_key": f"l1-decide-{uuid.uuid4().hex}",
            "payload": {
                "patient_workspace_id": alice_ws,
                "recommendation_id": "rec-l1-lisinopril",
            },
        }
        st_dec, decided = http(site_port, "POST", "/commands", decide_cmd, bearer=session)
        st_ws_w3, ws_w3 = http(site_port, "GET", f"/patient-workspaces/{alice_ws}", bearer=session)
        st_rnd3, rounds3 = http(site_port, "POST", "/staging-synthetic/rounds", {}, bearer=session)
        wait_l1_measure(
            lambda m: (m.get("necessary_care") or {}).get("total_reviews_completed", 0) >= 4
        )
        receipt.step("L1", "week-3 explicit physician decision promotes into active care plan", actor,
                     st_w3 == 200 and st_dec == 200 and (decided or {}).get("outcome") == "accepted"
                     and st_ws_w3 == 200 and st_rnd3 == 200
                     and len((ws_w3 or {}).get("undecided_recommendations", [])) == 0
                     and len((ws_w3 or {}).get("active_plan_items", [])) == 1,
                     f"day-21 approve outcome={(decided or {}).get('outcome')} "
                     f"active_plan={len((ws_w3 or {}).get('active_plan_items', []))}",
                     command_id=decide_cmd["idempotency_key"],
                     artifact=save(out, "week3-approval", {"clock": clock_w3, "decision": decided, "workspace": ws_w3, "rounds": rounds3}))

        st_m, measure = http(site_port, "GET", "/staging-synthetic/measure", bearer=session)
        receipt.data["l1_measure"] = measure
        rate = (measure or {}).get("physician_touched_per_100_patients_per_week")
        care = (measure or {}).get("necessary_care", {})
        measure_ok = (
            st_m == 200
            and (measure or {}).get("panel_size", 0) >= 2
            and abs((measure or {}).get("elapsed_weeks", 0.0) - 3.0) < 1e-6
            and isinstance(rate, (int, float)) and rate >= 0.0
            and care.get("total_reviews_completed", 0) >= 1
            and care.get("visit_briefings_ready", 0) >= 1
            and care.get("approved_plan_items", 0) == 1
            and care.get("spruce_messages_ingested", 0) >= 1
            and care.get("duet_updates_recorded", 0) >= 1
        )
        receipt.step("L1", "l1 measure and necessary-care accounting", actor, measure_ok,
                     f"rate={rate} touched/100pts/wk over {(measure or {}).get('elapsed_weeks')}w "
                     f"panel={(measure or {}).get('panel_size')} reviews={care.get('total_reviews_completed')} "
                     f"briefings={care.get('visit_briefings_ready')} spruce={care.get('spruce_messages_ingested')} "
                     f"duet={care.get('duet_updates_recorded')} approved_plan={care.get('approved_plan_items')}",
                     artifact=save(out, "l1-measure", measure))

        # Reset & replay check
        st_rst, reset_status = http(site_port, "POST", "/staging-synthetic/reset", {}, bearer=session)
        st_clk_rst, clock_rst = http(site_port, "GET", "/staging-synthetic/clock", bearer=session)
        st_rep, replayed = http(site_port, "POST", "/staging-synthetic/events", saved_chart["event"], bearer=session)
        counts = (reset_status or {}).get("counts", {})
        receipt.step("L1", "reset restores baseline clock/state and replays saved event", actor,
                     st_rst == 200 and st_clk_rst == 200 and (clock_rst or {}).get("elapsed_seconds") == 0
                     and counts.get("reviews") == 0 and counts.get("briefings") == 0
                     and counts.get("open_concerns") == 0 and st_rep == 201,
                     f"reset elapsed={(clock_rst or {}).get('elapsed_seconds')} reviews={counts.get('reviews')} replay={st_rep}",
                     artifact=save(out, "reset-replay", {"reset": reset_status, "clock": clock_rst, "replay": replayed}))
    finally:
        l1_stack.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--only", default="P1,D1",
                        help="comma-separated journeys (default P1,D1; available: P1, D1, L1)")
    parser.add_argument("--require-complete", action="store_true",
                        help="exit 3 when any capability is missing")
    parser.add_argument("--evidence", type=Path, help="new evidence directory "
                        "(default target/verification/runs/scenarios-<time>-<pid>)")
    args = parser.parse_args()
    journeys = {j.strip().upper() for j in args.only.split(",") if j.strip()}
    if not journeys:
        parser.error("--only selected no journeys; available: P1, D1, L1")
    if not journeys <= AVAILABLE_JOURNEYS:
        parser.error(f"unknown journeys {sorted(journeys - AVAILABLE_JOURNEYS)}; available: P1, D1, L1")
    stack.environment(os.environ, Path("/unused"))  # refuses to run in a hosted environment
    stamp = datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
    evidence = (args.evidence or REPO / f"target/verification/runs/scenarios-{stamp}-{os.getpid()}").resolve()
    evidence.mkdir(parents=True, exist_ok=False)
    target_root = Path(os.environ["CARGO_TARGET_DIR"]) if os.environ.get("CARGO_TARGET_DIR") else REPO / "target"
    bin_dir = target_root / "debug"
    print(f"Evidence: {evidence}", flush=True)
    receipt = Receipt(evidence / "receipt.json")
    receipt.data.update(journeys=sorted(journeys), evidence_dir=str(evidence))
    process = None
    try:
        subprocess.run(["cargo", "build", "--locked", "-p", "physician-sample", "--bins"],
                       cwd=REPO, check=True)
        need_scale_stack = bool(journeys & {"P1", "D1"})
        if need_scale_stack and not (CHARTS / "manifest.json").is_file():
            subprocess.run(["cargo", "run", "--locked", "-q", "-p", "patient-bank", "--", "build"],
                           cwd=REPO, check=True, stdout=subprocess.DEVNULL)
        if need_scale_stack:
            receipt.data["sources"] = sources(bin_dir)
            process = start_stack(evidence, bin_dir)
            state = wait_ready(process, evidence)
            receipt.data["services"] = {k: state[k] for k in ("sites", "worker", "postgres", "valkey")}
            if "P1" in journeys:
                p1(receipt, state, evidence / "p1")
            if "D1" in journeys:
                d1(receipt, state, evidence / "d1")
        else:
            receipt.data["sources"] = {
                "git_sha": run(["git", "rev-parse", "HEAD"]),
                "git_dirty": bool(run(["git", "status", "--porcelain", "--untracked-files=no"])),
                "rustc": run(["rustc", "-V"]),
                "binaries": {name: digest(bin_dir / name) for name in BINARIES},
            }
        if "L1" in journeys:
            l1(receipt, bin_dir, evidence / "l1")
    except BaseException as error:
        receipt.step("runner", "unexpected error", "runner", False, repr(error))
    finally:
        try:
            stopped = stop_stack(process) if process else None
        except BaseException as error:
            stopped = f"stopping the scale stack failed: {error!r}"
        if stopped:
            receipt.step("runner", "stack shutdown", "runner", False, stopped)
        receipt.write()

    counts = receipt.data["counts"]
    print(f"\n{receipt.data['outcome'].upper()}: {counts['steps'] - counts['failed']}/{counts['steps']} "
          f"steps passed; {counts['real_model_steps']} real-model, "
          f"{counts['clinical_judgment_steps']} clinical-judgment; "
          f"{counts['missing']} missing capabilities", flush=True)
    for gap in receipt.missing:
        print(f"MISSING {gap['journey']} {gap['capability']}: {gap['why']}", flush=True)
    print(f"Receipt: {receipt.path}", flush=True)
    if counts["failed"]:
        return 1
    return 3 if args.require_complete and counts["missing"] else 0


if __name__ == "__main__":
    raise SystemExit(main())
