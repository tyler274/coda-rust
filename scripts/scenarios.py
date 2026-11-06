#!/usr/bin/env python3
"""Run the available P1 and D1 scenarios through real Rust services and write a receipt.

    scripts/scenarios.py [--only P1,D1] [--require-complete] [--evidence DIR]

P1 runs over HTTP against scripts/physician-scale-stack.py: two physician-sample
sites, physician-worker, Postgres, Valkey and the Pub/Sub emulator, with the
worker's fixed reply. D1 runs the duet-api Postgres suites against that same
disposable Postgres. Nothing reads Doppler, Google credentials or a model key.

receipt.json records the git revision, binary and chart digests, each step's
command identity and observation, and each capability the journey needs that
this run did not exercise. Every step is labeled controlled, real-model or
clinical-judgment; this runner produces only controlled evidence. A failed step,
a failed build or an unclean shutdown exits 1 and still writes the receipt.
Missing capabilities are printed and exit 0 unless --require-complete.
"""
import argparse
import datetime
import hashlib
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
        "model_execution": "physician-worker fixed chart-reading provider (text in services.worker.fixed_text); "
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


def restart_applications(state, count):
    os.kill(state["pid"], signal.SIGUSR2)
    deadline = time.monotonic() + 60
    while time.monotonic() < deadline:
        try:
            current = json.loads((Path(state["evidence"]) / "stack.json").read_text())
        except json.JSONDecodeError:  # restart state can be mid-write
            current = {}
        if current.get("application_restarts") == count:
            return
        time.sleep(0.2)
    raise RuntimeError("owned application restart did not finish")


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

    subprocess.run(["cargo", "test", "--locked", "-p", "physician-sample", "--test", "site",
                    "seed_p1_attachment_source", "--", "--ignored", "--exact"], cwd=REPO,
                   env=dict(os.environ, PHYSICIAN_TEST_DATABASE_URL=state["postgres"]["url"]), check=True)
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

    # Interrupt the owned worker during its delayed first provider call, and the
    # accepting site. Site B and the durable delivery owner recover the same command.
    deadline = time.monotonic() + 15
    active = ""
    while time.monotonic() < deadline:
        active = run(["psql", state["postgres"]["url"], "-At", "-v", "ON_ERROR_STOP=1",
                      "-v", f"chat={chat}"], input="select state->>'status' from physician_runtime_conversations where chat = :'chat';\n")
        if active == "active":
            break
        time.sleep(0.1)
    receipt.step("P1", "worker active before interruption", actor, active == "active",
                 f"durable state before process loss: {active}", command_id=key)
    restart_applications(state, 1)
    replied = lambda items: any(kind == "model" and "[[patient_source:" in (text or "")
                                for kind, text in items)
    body, items = wait_conversation(b, session, chat, replied, timeout=660)
    receipt.step("P1", "reply read back on site B", actor, replied(items),
                 f"site B snapshot holds {items}", command_id=key,
                 artifact=save(out, "snapshot", body))

    restarted = json.loads((Path(state["evidence"]) / "stack.json").read_text())
    receipt.step("P1", "site and worker restarted", actor, restarted.get("application_restarts") == 1,
                 f"owned application restarts: {restarted.get('application_restarts', 0)}", command_id=key)
    original_items = items
    original_body = body
    restart_applications(state, 2)
    body, items = conversation(b, session, chat)
    receipt.step("P1", "published answer survives restart of both sites and worker", actor,
                 body == original_body, "Post-publication restart includes serving site B; exact saved answer unchanged",
                 command_id=key, artifact=save(out, "published-after-restart", body))
    status, again = http(b, "POST", "/commands", command, bearer=session)
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
                 status == 200 and outcome == "accepted" and durable.startswith("completed|")
                 and durable.endswith("|2") and len(items) == 2 and items == original_items,
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

    citations = [segment for item in body.get("snapshot", {}).get("conversation", [])
                 for segment in item.get("segments", []) if segment.get("kind") == "citation"]
    exact = all(c.get("state") == "resolved" and c.get("source_presentation", {}).get("content", {}).get("text")
                for c in citations)
    receipt.step("P1", "exact cited source survives interrupted worker and readback", actor,
                 bool(citations) and exact and body == original_body,
                 f"{len(citations)} persisted source citations; duplicate readback unchanged",
                 command_id=key, artifact=save(out, "citations-after-restart", body))
    receipt.gap("P1", "clinical grounding, Sources and Basis", "controlled provider proves source transport, not clinical judgment or the full answer controls",
                "crates/local-product/src/physician_chat.rs")
    if citations:
        proof_input = out / "citation-browser-input.json"
        proof_input.write_text(json.dumps({
            "url": f"http://127.0.0.1:{b}/?patient={stack.WORKSPACE}&chat={chat}",
            "citation": citations[0],
        }, indent=2))
        browser_env = dict(os.environ, PHYSICIAN_TEST_DATABASE_URL=state["postgres"]["url"],
                           PHYSICIAN_CITATION_PROOF=str(proof_input.resolve()),
                           PHYSICIAN_PROOF_SHOTS=str((out / "browser").resolve()))
        argv = ["cargo", "test", "--locked", "-p", "physician-sample", "--test", "site",
                "recovered_chat_citation_opens_exact_source_after_reload", "--", "--ignored", "--exact", "--nocapture"]
        result = subprocess.run(argv, cwd=REPO, env=browser_env, capture_output=True, text=True)
        log = out / "citation-browser.log"
        log.write_text(f"$ {' '.join(argv)}\nexit {result.returncode}\n{result.stdout}{result.stderr}")
        receipt.step("P1", "reopen exact citation after recovery and page reload", actor,
                     result.returncode == 0,
                     "Chromium checks patient and exact source identity, opens the saved citation, corrects the source, reloads and reopens the retained original text and revision/digest",
                     command_id=key, artifact=str(log.relative_to(REPO)))
        if result.returncode == 0:
            restart_applications(state, 3)
            after_correction, _ = conversation(b, session, chat)
            receipt.step("P1", "corrected chart cannot replace published evidence after restart", actor,
                         after_correction == original_body,
                         "Both sites and worker restarted after source correction; saved answer and citation unchanged",
                         command_id=key, artifact=save(out, "corrected-chart-after-restart", after_correction))
            proof_input.write_text(json.dumps({
                "url": f"http://127.0.0.1:{b}/?patient={stack.WORKSPACE}&chat={chat}",
                "citation": citations[0], "correct_source": False,
            }, indent=2))
            browser_env["PHYSICIAN_PROOF_SHOTS"] = str((out / "browser-after-correction-restart").resolve())
            result = subprocess.run(argv, cwd=REPO, env=browser_env, capture_output=True, text=True)
            log = out / "citation-browser-after-correction-restart.log"
            log.write_text(f"$ {' '.join(argv)}\nexit {result.returncode}\n{result.stdout}{result.stderr}")
            receipt.step("P1", "reopen published attachment after correction and service restart", actor,
                         result.returncode == 0, "Chromium reopens original readable attachment and native bytes on desktop and phone",
                         command_id=key, artifact=str(log.relative_to(REPO)))
    else:
        receipt.step("P1", "reopen exact citation after recovery and page reload", actor, False,
                     "No saved citation available to open", command_id=key)
    receipt.gap("P1", "real-model answer",
                "this runner answers with the worker's fixed text; real-model proof needs staging "
                "or CODA_LOCAL_MODEL_EXECUTION=api with an authorized key", "crates/physician-sample/src/bin/physician-worker.rs")
    receipt.gap("P1", "hosted restart and redeploy", "local process interruption does not qualify deployed recovery",
                "scripts/physician-scale-stack.py")
    receipt.gap("P1", "dictation", "POST /dictation answers 503 on every site", "crates/physician-sample/src/lib.rs")


def d1(receipt, state, out):
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
    receipt.gap("D1", "duet-api binary on a local stack",
                "main.rs verifies identity only through identitytoolkit.googleapis.com and sends "
                "model calls only to api.openai.com, so the served binary cannot run without cloud "
                "credentials; D1 runs through the in-process router instead", "crates/duet-api/src/main.rs")
    receipt.gap("D1", "installed iPhone and Android clients",
                "sign-up, Terms, AI choice, restart and lock-screen alerts need installed apps; "
                "nothing here drives a phone", "crates/duet-app, crates/duet-client, crates/mobile")
    receipt.gap("D1", "Apple, Google and email sign-up",
                "the suites use a synthetic IdentityVerifier", "crates/duet-api/src/identity.rs")
    receipt.gap("D1", "real-model reply", "providers are controlled stand-ins",
                "crates/duet-api/src/conversation_routes.rs")
    receipt.gap("D1", "account and command identities in the receipt",
                "the suites generate their account and command ids and drop their databases; "
                "they do not export those ids, so D1 steps name the test, not the command",
                "crates/duet-api/tests")


def start_stack(evidence, bin_dir):
    log = (evidence / "stack.out").open("w")
    return subprocess.Popen(
        [sys.executable, str(REPO / "scripts/physician-scale-stack.py"), "--bin-dir", str(bin_dir),
         "--worker-text", "Controlled recorded source {{patient_source}}",
         "--worker-delay-ms", "2000",
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
        if "P1" in journeys:
            # Build the served WASM from this revision before fingerprinting it.
            subprocess.run(["cargo", "test", "--locked", "-p", "physician-sample", "--test", "site",
                            "the_practice_and_its_patients_are_visible", "--", "--ignored", "--exact"],
                           cwd=REPO, check=True)
        receipt.data["sources"] = sources(bin_dir)
        process = start_stack(evidence, bin_dir)
        state = wait_ready(process, evidence)
        receipt.data["services"] = {k: state[k] for k in ("sites", "worker", "postgres", "valkey", "pubsub")}
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
