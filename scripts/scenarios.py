#!/usr/bin/env python3
"""Run the available P1 and D1 scenarios through real Rust services and write a receipt.

    scripts/scenarios.py [--only P1,D1] [--require-complete] [--evidence DIR]

P1 runs over HTTP against scripts/physician-scale-stack.py: two physician-sample
sites, physician-worker, Postgres and Valkey, with the worker's fixed reply. D1 migrates a fresh database on that same disposable
Postgres through duet-migrate, drives the duet-api binary over HTTP with its
local identity and fixed-reply stand-ins (killing and restarting it mid-reply),
then runs the duet-api Postgres suites. Nothing reads Doppler, Google
credentials or a model key.

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
CHARTS = REPO / "target/patient-bank/bank"
D1_SUITES = ("database_foundation", "steering", "conversation_proof")
# What each D1 suite holds. The suites own the assertions; the receipt names them.
D1_TESTS = {
    "fresh_and_upgrade_keep_identity_and_enforce_account_scope":
        "fresh and upgraded schema keep identity; one account cannot read another's rows",
    "steer_reaches_the_reply_in_progress_once":
        "a second message steers the running reply; one paid run; a restarted store cannot replay it",
    "http_steer_relaunches_the_same_command_without_the_recovery_poll":
        "account, Terms and AI consent over HTTP, then message and steered reply from a loopback Responses stand-in",
    "durable_tool_recovery_memory_and_permission_fences":
        "interrupted reply resumes its committed tool batch once; withdrawn permission fences effects",
    "accepted_provider_failure_is_safe_and_correlated":
        "a provider failure after 202 is traced to the accepted request without private content",
    "failed_steer_lookup_still_attempts_privacy_confirmation":
        "a database failure after 202 still attempts privacy confirmation and is traced without private content",
    "a_landed_message_is_confirmed_by_reading_it":
        "an uncertain send is confirmed by reading it back; another account reads it as missing",
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
        "model_execution": f"physician-worker PHYSICIAN_WORKER_FIXED_TEXT={stack.FIXED_TEXT!r}; "
                           "duet-api controlled providers inside its test suites",
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
    return body, [(item.get("kind"), item.get("text"))
                  for item in (body or {}).get("snapshot", {}).get("conversation", [])]


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
                f"the reply carries {'' if cited else 'no '}citation fields; the hosted worker's only "
                "tool is skill_view, so it cannot read the chart",
                "crates/local-product/src/physician_chat.rs, crates/physician-sample/src/bin/physician-worker.rs")
    receipt.gap("P1", "reopen a cited source in the side panel",
                "no citation exists to open", "crates/physician-sample/src/live.rs, crates/physician-app/src/lib.rs")
    receipt.gap("P1", "real-model answer",
                "this runner answers with the worker's fixed text; real-model proof needs staging "
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
    receipt.gap("D1", "real-model reply",
                f"the served binary answers with DUET_LOCAL_MODEL_REPLY={D1_REPLY!r}; real-model proof "
                "needs CODA_ALLOW_MODEL_CALLS=1 with an authorized key", "crates/duet-api/src/conversation_routes.rs")


D1_REPLY = "Thanks, noted."
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
    receipt.data["sources"]["duet_model_execution"] = f"duet-api DUET_LOCAL_MODEL_REPLY={D1_REPLY!r}"
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
           "DUET_LOCAL_MODEL_REPLY": D1_REPLY, "DUET_PRIVACY_JOURNAL_DIRECTORY": str(out / "journal"),
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
        duet.start(DUET_LOCAL_MODEL_DELAY_MS="20000")
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

        status, page = http(duet.port, "GET", "/api/conversation/messages", bearer=bearer)
        receipt.step("D1", "start a conversation", actor,
                     status == 200 and (page or {}).get("messages") == []
                     and (page or {}).get("availability", {}).get("can_send"),
                     f"GET /api/conversation/messages answered {status} with "
                     f"{len((page or {}).get('messages', []))} messages, availability {(page or {}).get('availability')}")

        command = str(uuid.uuid4())
        ids["command_id"] = command
        message = {"id": command, "text": "I walked for twenty minutes after lunch.",
                   "timezone": "America/New_York"}
        status, accepted = http(duet.port, "POST", "/api/conversation/messages", message, bearer)
        deadline = time.monotonic() + 15
        durable = ""
        while time.monotonic() < deadline:
            durable = sql(d1_command_query(command))
            if durable.startswith("running|") and durable.split("|")[2] != "0":
                break
            time.sleep(0.1)
        receipt.step("D1", "message accepted and dispatched", actor,
                     status == 202 and durable.startswith("running|1|1|"),
                     f"POST answered {status} run {(accepted or {}).get('run', {}).get('state')}; durable "
                     f"state|claim|dispatches|reply is {durable!r}", command_id=command,
                     artifact=save(out, "send", {"request": message, "status": status, "body": accepted}))

        killed = duet.kill()
        durable = sql(d1_command_query(command))
        receipt.step("D1", "duet-api killed mid-reply", "runner",
                     durable.startswith("running|1|1|") and durable.endswith("|f"),
                     f"SIGKILL (exit {killed}) while the fixed provider held the exchange open; "
                     f"state|claim|dispatches|reply is {durable!r}", command_id=command)

        duet.start()
        restarted = time.monotonic()
        replied = lambda page: any(m.get("role") == "assistant" for m in (page or {}).get("messages", []))
        page = None
        while time.monotonic() - restarted < 90:
            status, page = http(duet.port, "GET", "/api/conversation/messages", bearer=bearer)
            if replied(page):
                break
            time.sleep(0.5)
        durable = sql(d1_command_query(command))
        roles = [(m.get("role"), m.get("text")) for m in (page or {}).get("messages", [])]
        receipt.step("D1", "reply persists after restart", actor,
                     roles == [("user", message["text"]), ("assistant", D1_REPLY)]
                     and durable.startswith("complete|2|") and durable.endswith("|t"),
                     f"after {time.monotonic() - restarted:.1f}s the recovered transcript is {roles}; "
                     f"state|claim|dispatches|reply is {durable!r}", command_id=command,
                     artifact=save(out, "recovered", page))

        status, again = http(duet.port, "POST", "/api/conversation/messages", message, bearer)
        time.sleep(2)
        _, page = http(duet.port, "GET", "/api/conversation/messages", bearer=bearer)
        after = [(m.get("role"), m.get("text")) for m in (page or {}).get("messages", [])]
        commands = sql(f"select count(*) from duet_commands where user_id = '{ids['account_id']}';\n")
        receipt.step("D1", "retried send has one reply", actor,
                     status == 202 and after == roles and commands == "1"
                     and sql(d1_command_query(command)) == durable,
                     f"the same message id answered {status}; the transcript holds {after}; "
                     f"{commands} durable commands", command_id=command,
                     artifact=save(out, "retry", {"status": status, "body": again, "page": page}))

        d1_account_isolation(receipt, duet.port, out, other_bearer, ids, "after worker loss")

        event = str(uuid.uuid4())
        ids["sign_out_event_id"] = event
        status, signed_out = http(duet.port, "POST", "/api/auth/privacy-restrictions",
                                  {"event_id": event, "operation": "sessions_revoke", "scope_version": 0},
                                  bearer)
        refused_status, refused_body = http(duet.port, "GET", "/api/conversation/messages", bearer=bearer)
        receipt.step("D1", "sign out everywhere refuses the old session", actor,
                     status == 200 and refused_status == 401
                     and "session_revoked" in json.dumps(refused_body),
                     f"sessions_revoke answered {status}; the earlier bearer then answered "
                     f"{refused_status} {refused_body}", command_id=event,
                     artifact=save(out, "sign-out", {"status": status, "receipt": signed_out,
                                                     "refused": [refused_status, refused_body]}))

        stopped = duet.stop()
        duet.start()
        refused_status, _ = http(duet.port, "GET", "/api/conversation/messages", bearer=bearer)
        time.sleep(1.1)  # auth_time has whole-second precision; the fence holds the sign-out instant
        fresh = local_token(secret, subject, int(time.time()))
        status, page = http(duet.port, "GET", "/api/conversation/messages", bearer=fresh)
        latest = [(m.get("role"), m.get("text")) for m in (page or {}).get("messages", [])]
        receipt.step("D1", "after another restart the old session stays refused and a new sign-in reads the reply",
                     actor, stopped == 0 and refused_status == 401 and status == 200 and latest == roles,
                     f"SIGTERM exit {stopped}; the earlier bearer answered {refused_status}; a new "
                     f"session answered {status} with {latest}", command_id=command)
        d1_account_isolation(receipt, duet.port, out, other_bearer, ids, "after revocation and restart")
    finally:
        stopped = duet.stop()
        receipt.step("D1", "duet-api shutdown", "runner", stopped in (0, None), f"SIGTERM exit {stopped}")


def d1_account_isolation(receipt, port, out, bearer, ids, phase):
    context_status, context = http(port, "GET", "/api/auth/account/context", bearer=bearer)
    page_status, page = http(port, "GET", "/api/conversation/messages", bearer=bearer)
    command = ids["command_id"]
    refusals = [(method, suffix, *http(port, method, f"/api/conversation/commands/{command}/{suffix}",
                                      bearer=bearer))
                for method, suffix in (("GET", "events"), ("GET", "usage"), ("POST", "resume"))]
    passed = (context_status == 200 and (context or {}).get("account_id") == ids["other_account_id"]
              and page_status == 200 and (page or {}).get("messages") == []
              and all(status == 404 for _, _, status, _ in refusals))
    receipt.step("D1", f"two-account refusal {phase}", ids["other_subject"], passed,
                 f"second account context {context_status}, transcript {page_status}; "
                 f"foreign command endpoints {[(method, path, status) for method, path, status, _ in refusals]}",
                 command_id=command,
                 artifact=save(out, f"isolation-{phase.replace(' ', '-')}",
                               {"context": context, "page": page, "refusals": refusals}))


def d1_command_query(command):
    return ("select c.state, c.claim, (select count(*) from duet_execution_records r "
            "where r.user_id = c.user_id and r.command_id = c.id and r.record ? 'Dispatch'), "
            "c.reply is not null from duet_commands c "
            f"where c.id = '{uuid.UUID(command)}';\n")


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


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--only", default="P1,D1", help="comma-separated journeys (default P1,D1)")
    parser.add_argument("--require-complete", action="store_true",
                        help="exit 3 when any capability is missing")
    parser.add_argument("--evidence", type=Path, help="new evidence directory "
                        "(default target/verification/runs/scenarios-<time>-<pid>)")
    args = parser.parse_args()
    journeys = {j.strip().upper() for j in args.only.split(",") if j.strip()}
    if not journeys:
        parser.error("--only selected no journeys; available: P1, D1")
    if not journeys <= {"P1", "D1"}:
        parser.error(f"unknown journeys {sorted(journeys - {'P1', 'D1'})}; available: P1, D1")
    stack.environment(os.environ, Path("/unused"))  # refuses to run in a hosted environment
    stamp = datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
    evidence = (args.evidence or REPO / f"target/verification/runs/scenarios-{stamp}-{os.getpid()}").resolve()
    evidence.mkdir(parents=True, exist_ok=False)
    bin_dir = REPO / "target/debug"
    print(f"Evidence: {evidence}", flush=True)
    receipt = Receipt(evidence / "receipt.json")
    receipt.data.update(journeys=sorted(journeys), evidence_dir=str(evidence))
    process = None
    try:
        subprocess.run(["cargo", "build", "--locked", "-p", "physician-sample", "--bins"],
                       cwd=REPO, check=True)
        if not (CHARTS / "manifest.json").is_file():
            subprocess.run(["cargo", "run", "--locked", "-q", "-p", "patient-bank", "--", "build"],
                           cwd=REPO, check=True, stdout=subprocess.DEVNULL)
        receipt.data["sources"] = sources(bin_dir)
        process = start_stack(evidence, bin_dir)
        state = wait_ready(process, evidence)
        receipt.data["services"] = {k: state[k] for k in ("sites", "worker", "postgres", "valkey")}
        if "P1" in journeys:
            p1(receipt, state, evidence / "p1")
        if "D1" in journeys:
            d1(receipt, state, evidence / "d1")
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
