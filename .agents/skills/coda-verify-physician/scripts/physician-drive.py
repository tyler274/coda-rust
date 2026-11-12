#!/usr/bin/env python3
"""Drive a running physician scale stack over its real HTTP entry points.

Reads the newest target/verification/runs/physician-scale-stack-*/stack.json
(or --stack PATH) for ports. Every command prints JSON and, with --out DIR,
writes it there too. No credentials: proof sign-in for the synthetic account.

  sign-in   [--site a|b]                          print a bearer session
  page-url  [--chat C] [--site b]                 the stack's patient, opened on that chat (?patient=[&chat=])
  send      --session S --chat C --text T [--site a] [--key K]
  snapshot  --session S --chat C [--site b] [--wait-text T | --wait-failed] [--timeout 90]
  resume    --session S --chat C [--site a]       Try again on the failed run
  watch     --session S [--site b] [--until EVENT] [--timeout 60]
  announce  --chat C --key K                      tell listening workers of that input again
  restart-valkey                                  SIGUSR1 to the stack (stop, wait, start)
  subscribers                                     Valkey bus subscriber count
"""
import argparse
import http.client
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import time
import urllib.error
import urllib.request
import uuid

REPO = Path(__file__).resolve().parents[4]
RUNS = REPO / "target" / "verification" / "runs"
WORK_CHANNEL = "physician_work"
CHANNEL = "physician:workspace-events:v1"
COULDNT_FINISH = "Coda couldn’t finish this response. Your message is saved."


def load_stack(path):
    if path is None:
        candidates = sorted(RUNS.glob("physician-scale-stack-*/stack.json"),
                            key=lambda item: item.stat().st_mtime)
        if not candidates:
            sys.exit("no stack.json under target/verification/runs; launch the stack first")
        path = candidates[-1]
    stack = json.loads(Path(path).read_text())
    try:
        os.kill(stack["pid"], 0)
    except OSError:
        sys.exit(f"stack pid {stack['pid']} from {path} is not running")
    stack["path"] = str(path)
    return stack


def port(stack, site):
    return stack["ports"][f"site-{site}"]


def request(method, url, body=None, bearer=None, timeout=15):
    headers = {"Content-Type": "application/json"}
    if bearer:
        headers["Authorization"] = f"Bearer {bearer}"
    data = None if body is None else json.dumps(body).encode()
    req = urllib.request.Request(url, data=data, headers=headers, method=method)
    try:
        with urllib.request.urlopen(req, timeout=timeout) as response:
            raw = response.read()
            return response.status, (json.loads(raw) if raw else None)
    except urllib.error.HTTPError as error:
        raw = error.read()
        try:
            return error.code, json.loads(raw)
        except ValueError:
            return error.code, raw.decode(errors="replace")


def emit(args, name, value):
    text = json.dumps(value, indent=2, ensure_ascii=False)
    print(text)
    if args.out:
        out = Path(args.out)
        out.mkdir(parents=True, exist_ok=True)
        (out / f"{name}.json").write_text(text + "\n")


def sign_in(stack, site):
    base = f"http://127.0.0.1:{port(stack, site)}"
    status, issued = request("POST", f"{base}/auth/sign-in", {"email": stack["sign_in_email"]})
    if status != 200 or not isinstance(issued, dict) or "token" not in issued:
        sys.exit(f"sign-in {status}: {issued}")
    status, verified = request("POST", f"{base}/auth/verify",
                               {"email": stack["sign_in_email"], "token": issued["token"]})
    if status != 200:
        sys.exit(f"verify {status}: {verified}")
    return verified["session"]


def snapshot(stack, site, session, chat):
    url = (f"http://127.0.0.1:{port(stack, site)}/patient-workspaces/"
           f"{stack['workspace']}/chats/{chat}/snapshot")
    return request("GET", url, bearer=session)


def resume_line(body):
    """The line of a turn that ended and can be resumed. A turn still running
    also shows a mechanical line, with a stop target and nothing to resume."""
    for item in (body or {}).get("snapshot", {}).get("conversation", []):
        if item.get("kind") == "mechanical" and "resume_target" in item:
            return item
    return None


def texts(body):
    return [item.get("text") for item in (body or {}).get("snapshot", {}).get("conversation", [])
            if item.get("kind") != "mechanical"]


def command(stack, site, session, kind, key, payload):
    url = f"http://127.0.0.1:{port(stack, site)}/commands"
    return request("POST", url, {"type": kind, "idempotency_key": key, "payload": payload},
                   bearer=session)


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--stack", type=Path, help="stack.json (default: newest run)")
    parser.add_argument("--out", help="also write the JSON result into this evidence folder")
    sub = parser.add_subparsers(dest="action", required=True)
    s = sub.add_parser("sign-in"); s.add_argument("--site", default="a")
    s = sub.add_parser("page-url"); s.add_argument("--site", default="b")
    s.add_argument("--chat", help="omit to open the workspace without a chat")
    s = sub.add_parser("send")
    s.add_argument("--site", default="a"); s.add_argument("--session", required=True)
    s.add_argument("--chat", required=True); s.add_argument("--text", required=True)
    s.add_argument("--key", help="idempotency key (reuse it to prove a duplicate)")
    s = sub.add_parser("snapshot")
    s.add_argument("--site", default="b"); s.add_argument("--session", required=True)
    s.add_argument("--chat", required=True); s.add_argument("--wait-text")
    s.add_argument("--wait-failed", action="store_true"); s.add_argument("--timeout", type=float, default=90)
    s = sub.add_parser("resume")
    s.add_argument("--site", default="a"); s.add_argument("--session", required=True)
    s.add_argument("--chat", required=True)
    s.add_argument("--run", help="override the run id (a wrong one must be rejected)")
    s = sub.add_parser("watch")
    s.add_argument("--site", default="b"); s.add_argument("--session", required=True)
    s.add_argument("--until", default="workspace_changed"); s.add_argument("--timeout", type=float, default=60)
    s = sub.add_parser("announce")
    s.add_argument("--chat", required=True)
    s.add_argument("--key", required=True, help="the accepted command's idempotency key")
    sub.add_parser("restart-valkey")
    sub.add_parser("subscribers")
    args = parser.parse_args()
    stack = load_stack(args.stack)

    if args.action == "sign-in":
        session = sign_in(stack, args.site)
        print(session)
        return 0
    if args.action == "page-url":
        chat = f"&chat={args.chat}" if args.chat else ""
        print(f"http://127.0.0.1:{port(stack, args.site)}/?patient={stack['workspace']}{chat}")
        return 0
    if args.action == "send":
        key = args.key or f"key-{uuid.uuid4().hex}"
        status, body = command(stack, args.site, args.session, "message.submit", key, {
            "patient_workspace_id": stack["workspace"], "chat_id": args.chat, "text": args.text,
            "physician_time_zone": "America/New_York"})
        emit(args, f"send-{args.site}", {"site": args.site, "key": key, "status": status, "body": body})
        return 0 if status == 200 and isinstance(body, dict) and body.get("outcome") == "accepted" else 1
    if args.action == "snapshot":
        deadline = time.monotonic() + args.timeout
        while True:
            status, body = snapshot(stack, args.site, args.session, args.chat)
            done = status == 200 and (
                (args.wait_text and args.wait_text in texts(body))
                or (args.wait_failed and resume_line(body) is not None)
                or (not args.wait_text and not args.wait_failed))
            if done or time.monotonic() >= deadline:
                break
            time.sleep(0.25)
        emit(args, f"snapshot-{args.site}", {"site": args.site, "status": status, "matched": bool(done),
                                              "body": body})
        return 0 if done else 1
    if args.action == "resume":
        status, body = snapshot(stack, args.site, args.session, args.chat)
        line = resume_line(body)
        if line is None:
            sys.exit(f"no failed turn to resume in {args.chat}: {body}")
        target = line["resume_target"]
        run = args.run or target["expected_run_id"]
        status, answer = command(stack, args.site, args.session, "work.resume",
                                 f"resume-{target['work_item_id']}-{run}", {
                                     "patient_workspace_id": stack["workspace"], "chat_id": args.chat,
                                     "work_item_id": target["work_item_id"], "expected_run_id": run})
        emit(args, f"resume-{args.site}", {"site": args.site, "mechanical": line, "run": run,
                                           "status": status, "body": answer})
        return 0 if isinstance(answer, dict) and answer.get("outcome") == "accepted" else 1
    if args.action == "watch":
        connection = http.client.HTTPConnection("127.0.0.1", port(stack, args.site), timeout=args.timeout)
        connection.request("GET", f"/patient-workspaces/events?patient_workspace_id={stack['workspace']}",
                           headers={"Authorization": f"Bearer {args.session}"})
        response = connection.getresponse()
        buffer, started = "", time.monotonic()
        print(f"watching site {args.site} (HTTP {response.status}) until event: {args.until}",
              file=sys.stderr, flush=True)
        try:
            while f"event: {args.until}" not in buffer:
                chunk = response.read1(4096)
                if not chunk:
                    break
                buffer += chunk.decode(errors="replace")
        except (TimeoutError, OSError):
            pass
        seen = f"event: {args.until}" in buffer
        emit(args, f"watch-{args.site}-{args.until}", {
            "site": args.site, "status": response.status, "until": args.until, "seen": seen,
            "seconds": round(time.monotonic() - started, 3), "frames": buffer})
        return 0 if seen else 1
    if args.action == "announce":
        # The notification the accepting transaction sent, sent once more: a
        # worker that hears of settled work must do nothing.
        account = stack["account"]
        delivery = {"scope": {"practice": account["practice"], "physician": account["physician"],
                              "workspace": stack["workspace"], "chat": args.chat},
                    "input_id": args.key}
        sent = subprocess.run(
            ["psql", stack["postgres"]["url"], "-v", "ON_ERROR_STOP=1", "-At",
             "-v", f"channel={WORK_CHANNEL}", "-v", f"payload={json.dumps(delivery)}"],
            input="select pg_notify(:'channel', :'payload');\n", capture_output=True, text=True)
        emit(args, "announce", {"channel": WORK_CHANNEL, "delivery": delivery,
                                "exit": sent.returncode, "stderr": sent.stderr.strip()})
        return sent.returncode
    if args.action == "restart-valkey":
        os.kill(stack["pid"], signal.SIGUSR1)
        print(f"sent SIGUSR1 to stack pid {stack['pid']}; see the stack's output for the restart")
        return 0
    if args.action == "subscribers":
        output = subprocess.run(["valkey-cli", "-h", "127.0.0.1", "-p", str(stack["ports"]["valkey"]),
                                 "--raw", "PUBSUB", "NUMSUB", CHANNEL], capture_output=True).stdout.decode()
        print(output.split()[1] if len(output.split()) > 1 else "0")
        return 0
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
