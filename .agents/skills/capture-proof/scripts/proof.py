#!/usr/bin/env python3
"""Record, check, publish and render a change's proof: each acceptance criterion's evidence.

The bundle lives in target/verification/proof/<branch>/ with manifest.json beside the
artifacts. Every entry records the revision it ran against, so stale proof shows.

  run        --claim C [--label L] [--expect RE] [--expect-fail] [--at REV [--with PATH...]] -- CMD...
  add        --claim C [--label L] [--revision REV] PATH   a log, response, screenshot or run folder
  screenshot --claim C --url URL [--label L] [--window 1440,900]
  gap        --claim C --reason R                      a criterion with no proof, and why
  supersede  ID... --reason R                          retire entries a later one replaces; history stays
  keep       ID... --reason R                          keep stale entries at this head, with the reason
  status                                               every entry; exit 1 if any is failing or stale
  check                                                secrets, emails and sizes; exit 1 if unpublishable
  publish    [--remote origin] [--web-base URL]        push the bundle to refs/proof/<branch>
  render     [--tail 15]                               the PR's Proof section, in Markdown
"""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
import time

FILE_LIMIT = 2 * 1024 * 1024
BUNDLE_LIMIT = 25 * 1024 * 1024
IMAGE_SUFFIXES = {".png", ".jpg", ".jpeg", ".gif", ".webp"}
SECRETS = [
    ("bearer token", re.compile(r"Bearer\s+[A-Za-z0-9._~+/=-]{16,}")),
    ("authorization header", re.compile(r"(?i)\bauthorization\s*[:=]\s*\S{8,}")),
    ("JWT", re.compile(r"\beyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]+")),
    ("private key", re.compile(r"-----BEGIN [A-Z ]*PRIVATE KEY-----")),
    ("Google API key", re.compile(r"\bAIza[0-9A-Za-z_-]{35}\b")),
    ("Doppler token", re.compile(r"\bdp\.(?:st|ct|sa|pt|scim|audit)\.[A-Za-z0-9_.-]{20,}")),
    ("GitHub token", re.compile(r"\b(?:gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{30,})")),
    ("AWS key", re.compile(r"\bAKIA[0-9A-Z]{16}\b")),
    ("Coda token", re.compile(r"\bcoda[a-z]-[0-9a-f]{32}\b")),
    ("credential argument", re.compile(
        r"(?i)--(?:session|token|password|secret|api-key|bearer)(?:=|\s+|[\"'],\s*[\"'])(?!<redacted>)[^\s\"']{8,}")),
    ("URL credentials", re.compile(r"://[^/\s:@\"']+:[^/\s@\"']+@")),
    ("URL credential parameter", re.compile(
        r"(?i)[?&](?:token|session|key|access_token|sig|signature)=[^&\s\"']{8,}")),
    ("credential field", re.compile(
        r"(?i)[\"']?\b(?:token|session|access_token|refresh_token|secret|password|api_key)\b[\"']?"
        r"\s*[:=]\s*[\"'][^\"'\s]{12,}[\"']")),
]
EMAIL = re.compile(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}")
SYNTHETIC_EMAIL = re.compile(
    r"(?i)@(?:[a-z0-9.-]+\.)?(?:codahealth\.test|example\.(?:com|org|net)|anthropic\.com|"
    r"users\.noreply\.github\.com)$")


def redact(text):
    for _, pattern in SECRETS:
        text = pattern.sub("<redacted>", text)
    return text


def git(*args, cwd=None, check=True):
    result = subprocess.run(["git", *args], cwd=cwd, capture_output=True, text=True)
    if check and result.returncode:
        sys.exit(f"git {' '.join(args)}: {result.stderr.strip()}")
    return result.stdout.strip()


def repo_root():
    return Path(git("rev-parse", "--show-toplevel"))


def revision(cwd=None):
    head = git("rev-parse", "HEAD", cwd=cwd)
    dirty = git("status", "--porcelain", "--untracked-files=no", cwd=cwd) != ""
    return head, dirty


def bundle_dir():
    branch = git("rev-parse", "--abbrev-ref", "HEAD")
    slug = re.sub(r"[^A-Za-z0-9._-]+", "-", branch).strip("-") or "detached"
    path = repo_root() / "target" / "verification" / "proof" / slug
    path.mkdir(parents=True, exist_ok=True)
    return path, slug


def load():
    path, slug = bundle_dir()
    manifest = path / "manifest.json"
    data = json.loads(manifest.read_text()) if manifest.exists() else {"branch": slug, "entries": []}
    return path, data


def save(path, data):
    (path / "manifest.json").write_text(json.dumps(data, indent=2) + "\n")


def new_entry(data, args, kind):
    head, dirty = revision()
    data.pop("published", None)
    entry = {"id": len(data["entries"]) + 1, "claim": args.claim, "label": args.label or "",
             "kind": kind, "revision": head, "dirty": dirty,
             "recorded": datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds")}
    data["entries"].append(entry)
    return entry


def slugify(text):
    return re.sub(r"[^A-Za-z0-9._-]+", "-", text).strip("-")[:48] or "item"


def cmd_run(args):
    if not args.command:
        sys.exit("run: give the command after --")
    path, data = load()
    entry = new_entry(data, args, "run")
    redacted = [redact(arg) for arg in args.command]
    previous = None
    for index, arg in enumerate(args.command):
        if previous and re.fullmatch(r"(?i)--(?:session|token|password|secret|api-key|bearer)", previous):
            redacted[index] = "<redacted>"
        previous = arg
    entry.update(command=redacted, expect=args.expect, expect_fail=args.expect_fail)
    log_name = f"{entry['id']:02d}-{slugify(args.label or args.command[0])}.log"
    entry["files"] = [log_name]
    cwd, worktree = Path.cwd(), None
    if args.at:
        base = git("rev-parse", args.at)
        entry["at"] = base
        worktree = Path(tempfile.mkdtemp(prefix="proof-at-"))
        git("worktree", "add", "--detach", str(worktree), base)
        root = repo_root()
        for item in args.with_paths:
            source = root / item
            target = worktree / item
            target.parent.mkdir(parents=True, exist_ok=True)
            (shutil.copytree if source.is_dir() else shutil.copy2)(source, target)
        entry["with"] = args.with_paths
        cwd = worktree / Path.cwd().relative_to(root)
    started = time.monotonic()
    output = []
    try:
        with (path / log_name).open("w") as log:
            try:
                process = subprocess.Popen(args.command, cwd=cwd, stdout=subprocess.PIPE,
                                           stderr=subprocess.STDOUT, text=True, errors="replace")
            except OSError as error:
                log.write(f"{error}\n")
                output.append(f"{error}\n")
                exit_code = 127
            else:
                for line in process.stdout:
                    sys.stdout.write(line)
                    log.write(line)
                    output.append(line)
                exit_code = process.wait()
    finally:
        if worktree:
            git("worktree", "remove", "--force", str(worktree), check=False)
    entry["seconds"] = round(time.monotonic() - started, 1)
    entry["exit"] = exit_code
    failed_as_asked = (exit_code != 0) == bool(args.expect_fail)
    matched = True
    if args.expect:
        matched = re.search(args.expect, "".join(output), re.MULTILINE) is not None
        entry["expect_matched"] = matched
    entry["ok"] = failed_as_asked and matched
    save(path, data)
    print(f"proof: entry {entry['id']} {args.claim} {'ok' if entry['ok'] else 'NOT OK'} "
          f"(exit {exit_code}) -> {path / log_name}", file=sys.stderr)
    return 0 if entry["ok"] else (exit_code or 1)


def cmd_add(args):
    source = Path(args.path)
    if not source.exists():
        sys.exit(f"add: {source} does not exist")
    path, data = load()
    kind = "run-dir" if source.is_dir() else (
        "screenshot" if source.suffix.lower() in IMAGE_SUFFIXES else "file")
    entry = new_entry(data, args, kind)
    name = f"{entry['id']:02d}-{source.name}"
    if source.is_dir():
        shutil.copytree(source, path / name)
        entry["files"] = sorted(str(p.relative_to(path)) for p in (path / name).rglob("*") if p.is_file())
    else:
        shutil.copy2(source, path / name)
        entry["files"] = [name]
    entry["source"] = str(source)
    entry["revision"] = git("rev-parse", args.revision) if args.revision else None
    entry["dirty"] = False
    entry["ok"] = True
    save(path, data)
    print(f"proof: entry {entry['id']} {args.claim} added {kind} -> {path / name}", file=sys.stderr)
    return 0


def cmd_screenshot(args):
    browser = args.chromium or os.environ.get("CHROMIUM")
    if not browser:
        sys.exit("screenshot: set CHROMIUM or pass --chromium")
    path, data = load()
    entry = new_entry(data, args, "screenshot")
    name = f"{entry['id']:02d}-{slugify(args.label or args.url)}.png"
    result = subprocess.run([browser, "--headless=new", "--disable-gpu", "--hide-scrollbars",
                             f"--window-size={args.window}", f"--screenshot={path / name}", args.url],
                            capture_output=True, text=True)
    entry.update(url=args.url, files=[name], ok=result.returncode == 0 and (path / name).exists())
    save(path, data)
    if not entry["ok"]:
        sys.exit(f"screenshot failed: {result.stderr.strip()[-400:]}")
    print(f"proof: entry {entry['id']} {args.claim} -> {path / name}; look at it before relying on it",
          file=sys.stderr)
    return 0


def cmd_gap(args):
    path, data = load()
    entry = new_entry(data, args, "gap")
    entry.update(reason=args.reason, ok=False, files=[])
    save(path, data)
    return 0


def cmd_supersede(args):
    path, data = load()
    for entry in pick(data, args.ids):
        entry["superseded"] = args.reason
    data.pop("published", None)
    save(path, data)
    return 0


def cmd_keep(args):
    path, data = load()
    head, dirty = revision()
    if dirty:
        sys.exit("keep: commit first; a reason cannot cover uncommitted changes")
    for entry in pick(data, args.ids):
        entry["kept"] = {"revision": head, "reason": args.reason}
    data.pop("published", None)
    save(path, data)
    return 0


def pick(data, ids):
    entries = {entry["id"]: entry for entry in data["entries"]}
    missing = [i for i in ids if i not in entries]
    if missing:
        sys.exit(f"no entries {missing}")
    return [entries[i] for i in ids]


def current(data):
    return [entry for entry in data["entries"] if not entry.get("superseded")]


def stale(entry, head):
    if entry["kind"] == "gap":
        return False
    if entry.get("kept", {}).get("revision") == head:
        return False
    return entry["revision"] != head or entry["dirty"]


def cmd_status(args):
    path, data = load()
    head, dirty = revision()
    bad = 0
    print(f"bundle {path}\nhead {head[:10]}{' (uncommitted changes)' if dirty else ''}")
    for entry in sorted(current(data), key=lambda e: (e["ok"] and not stale(e, head), e["id"])):
        if entry["kind"] == "gap":
            state = f"GAP: {entry['reason']}"
        else:
            state = "ok" if entry["ok"] else f"FAILING (exit {entry.get('exit')})"
            if entry["revision"] is None and stale(entry, head):
                state += ", REVISION UNKNOWN (add --revision)"
            elif stale(entry, head):
                state += f", STALE at {entry['revision'][:10]}{' + uncommitted' if entry['dirty'] else ''}"
            elif entry.get("kept", {}).get("revision") == head:
                state += f", kept from {(entry['revision'] or 'unknown')[:10]}: {entry['kept']['reason']}"
        bad += entry["kind"] != "gap" and (not entry["ok"] or stale(entry, head))
        print(f"{entry['id']:>3} {entry['claim']:<8} {entry['kind']:<10} {entry['label'] or ''}  {state}")
    return 1 if bad else 0


def scan(path, data):
    problems, warnings, total = [], [], 0
    for entry in data["entries"]:
        for name in entry.get("files", []):
            file = path / name
            size = file.stat().st_size
            total += size
            if size > FILE_LIMIT:
                problems.append(f"{name}: {size} bytes is over the {FILE_LIMIT} byte file limit")
            if file.suffix.lower() in IMAGE_SUFFIXES:
                continue
            text = file.read_text(errors="replace")
            for kind, pattern in SECRETS:
                for match in pattern.finditer(text):
                    line = text.count("\n", 0, match.start()) + 1
                    problems.append(f"{name}:{line}: possible {kind}")
            for address in sorted(set(EMAIL.findall(text))):
                if not SYNTHETIC_EMAIL.search(address):
                    warnings.append(f"{name}: email {address} is not a synthetic account")
    manifest = json.dumps(data)
    for kind, pattern in SECRETS:
        if pattern.search(manifest):
            problems.append(f"manifest.json: possible {kind}")
    if total > BUNDLE_LIMIT:
        problems.append(f"bundle is {total} bytes, over the {BUNDLE_LIMIT} byte limit")
    return problems, warnings


def cmd_check(args):
    path, data = load()
    problems, warnings = scan(path, data)
    for line in warnings:
        print(f"warning: {line}", file=sys.stderr)
    for line in problems:
        print(f"refused: {line}", file=sys.stderr)
    if not problems:
        print(f"proof: {len(data['entries'])} entries are publishable", file=sys.stderr)
    return 1 if problems else 0


def web_base(remote_url):
    match = re.search(r"github\.com[:/]+([^/]+)/([^/]+?)(?:\.git)?/?$", remote_url)
    return f"https://github.com/{match.group(1)}/{match.group(2)}" if match else None


def cmd_publish(args):
    path, data = load()
    problems, _ = scan(path, data)
    if problems:
        cmd_check(args)
        sys.exit("publish: fix or remove the refused artifacts first")
    remote_url = git("remote", "get-url", args.remote, check=False) or args.remote
    base = args.web_base or web_base(remote_url)
    if not base:
        sys.exit(f"publish: cannot derive a GitHub URL from {remote_url}; pass --web-base")
    ref = f"refs/proof/{data['branch']}"
    head, _ = revision()
    prefix = head[:10]
    parent = None
    if git("ls-remote", args.remote, ref, check=False):
        git("fetch", "--quiet", args.remote, f"+{ref}:{ref}")
        parent = git("rev-parse", ref)
    index = Path(tempfile.mkdtemp(prefix="proof-index-")) / "index"
    env = dict(os.environ, GIT_INDEX_FILE=str(index))
    names = ["manifest.json"] + [n for e in data["entries"] for n in e.get("files", [])]
    for name in names:
        blob = git("hash-object", "-w", str(path / name))
        subprocess.run(["git", "update-index", "--add", "--cacheinfo", f"100644,{blob},{prefix}/{name}"],
                       env=env, check=True)
    tree = subprocess.run(["git", "write-tree"], env=env, check=True, capture_output=True,
                          text=True).stdout.strip()
    message = f"Proof for {data['branch']} at {head}"
    commit = git("commit-tree", tree, *(["-p", parent] if parent else []), "-m", message)
    push = subprocess.run(["git", "push", "--quiet", args.remote, f"{commit}:{ref}"],
                          capture_output=True, text=True)
    if push.returncode:
        sys.exit(f"publish: push failed: {push.stderr.strip()}")
    data["published"] = {"ref": ref, "commit": commit, "prefix": prefix, "web_base": base,
                         "revision": head, "digest": digest(path, data)}
    save(path, data)
    print(f"proof: published {len(names)} files to {ref} at {commit[:10]}; "
          f"{base}/tree/{commit}/{prefix}", file=sys.stderr)
    return 0


def digest(path, data):
    hasher = hashlib.sha256(json.dumps(data["entries"], sort_keys=True).encode())
    for entry in data["entries"]:
        for name in entry.get("files", []):
            hasher.update(name.encode() + b"\0" + (path / name).read_bytes())
    return hasher.hexdigest()


def published(path, data):
    record = data.get("published")
    if record and record["revision"] == revision()[0] and record.get("digest") == digest(path, data):
        return record
    return None


def link(data, path, name, raw=False, tree=False):
    record = published(path, data)
    if record:
        kind = "tree" if tree else "blob"
        url = f"{record['web_base']}/{kind}/{record['commit']}/{record['prefix']}/{name}"
        return url + ("?raw=true" if raw else "")
    return str(path / name)


def result(entry, head):
    if entry["kind"] == "gap":
        return f"Gap: {entry['reason']}"
    if entry["kind"] == "run":
        if entry.get("expect_fail"):
            text = (f"Fails as expected at `{entry['at'][:10]}`" if entry.get("at") else
                    "Fails as expected") if entry["ok"] else "**Did not fail as expected**"
        else:
            text = "Pass" if entry["ok"] else "**Fail**"
        text += f" (exit {entry['exit']})"
        if entry.get("expect") is not None:
            text += "" if entry.get("expect_matched") else ", expected output missing"
    else:
        text = "Recorded"
    if entry["revision"] is None:
        text += ", revision unknown"
    elif stale(entry, head):
        text += f", at `{entry['revision'][:10]}`{' with uncommitted changes' if entry['dirty'] else ''}"
    if entry.get("kept", {}).get("revision") == head:
        text += f", from `{(entry['revision'] or 'unknown')[:10]}`, kept: {entry['kept']['reason']}"
    return text


def cmd_render(args):
    path, data = load()
    head, dirty = revision()
    record = published(path, data)
    bundle = (f"[all evidence]({record['web_base']}/tree/{record['commit']}/{record['prefix']})"
              if record else f"all evidence in `{path}` (not published)")
    lines = [f"## Proof (at `{head[:10]}`{', with uncommitted changes' if dirty else ''})", "",
             bundle, "", "| Criterion | Evidence | Result |", "| --- | --- | --- |"]
    entries = current(data)
    for entry in sorted(entries, key=lambda e: (e["claim"], e["id"])):
        files = entry.get("files", [])
        label = entry["label"]
        if entry["kind"] == "run":
            evidence = f"`{label or shlex.join(entry['command'])}` ([log]({link(data, path, files[0])}))"
        elif entry["kind"] == "run-dir":
            folder = files[0].split("/")[0] if files else ""
            evidence = (f"[{label or entry['source']}]({link(data, path, folder, tree=True)}) "
                        f"({len(files)} files)")
        elif files:
            evidence = f"[{label or files[0]}]({link(data, path, files[0])})"
        else:
            evidence = "None"
        lines.append(f"| {entry['claim']} | {evidence} | {result(entry, head)} |")
    for entry in entries:
        files = entry.get("files", [])
        if entry["kind"] == "run":
            tail = redact((path / files[0]).read_text(errors="replace")).splitlines()[-args.tail:]
            where = f" at `{entry['at'][:10]}`" if entry.get("at") else ""
            lines += ["", f"<details><summary>{entry['claim']}: "
                          f"{entry['label'] or entry['command'][0]}{where}, last {len(tail)} lines</summary>",
                      "", "```console", f"$ {shlex.join(entry['command'])}", *tail,
                      f"# exit {entry['exit']}", "```", "</details>"]
        for name in files:
            if Path(name).suffix.lower() in IMAGE_SUFFIXES:
                lines += ["", f"![{entry['claim']}: {entry['label'] or name}]({link(data, path, name, raw=True)})"]
    print("\n".join(lines))
    return 0


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    commands = parser.add_subparsers(dest="action", required=True)

    def claimed(name):
        sub = commands.add_parser(name)
        sub.add_argument("--claim", required=True, help="the acceptance criterion, such as AC2")
        sub.add_argument("--label", help="what this evidence is, in a few words")
        return sub

    run = claimed("run")
    run.add_argument("--expect", help="a regular expression the output must contain")
    run.add_argument("--expect-fail", action="store_true", help="the command must exit nonzero")
    run.add_argument("--at", help="run in a temporary worktree at this revision")
    run.add_argument("--with", dest="with_paths", nargs="+", default=[],
                     help="paths copied from this checkout into the --at worktree, such as a new test")
    run.add_argument("command", nargs=argparse.REMAINDER)
    run.set_defaults(func=cmd_run)
    add = claimed("add")
    add.add_argument("--revision", help="the revision the artifact was produced at, when known")
    add.add_argument("path")
    add.set_defaults(func=cmd_add)
    shot = claimed("screenshot")
    shot.add_argument("--url", required=True)
    shot.add_argument("--window", default="1440,900")
    shot.add_argument("--chromium")
    shot.set_defaults(func=cmd_screenshot)
    gap = claimed("gap")
    gap.add_argument("--reason", required=True)
    gap.set_defaults(func=cmd_gap)
    for name, func, text in [("supersede", cmd_supersede, "why a later entry replaces these"),
                             ("keep", cmd_keep, "why later changes cannot affect these")]:
        sub = commands.add_parser(name)
        sub.add_argument("ids", nargs="+", type=int)
        sub.add_argument("--reason", required=True, help=text)
        sub.set_defaults(func=func)
    commands.add_parser("status").set_defaults(func=cmd_status)
    commands.add_parser("check").set_defaults(func=cmd_check)
    publish = commands.add_parser("publish")
    publish.add_argument("--remote", default="origin", help="a remote name or URL")
    publish.add_argument("--web-base", help="the repository's web URL, when the remote does not name GitHub")
    publish.set_defaults(func=cmd_publish)
    render = commands.add_parser("render")
    render.add_argument("--tail", type=int, default=15, help="output lines shown for each run")
    render.set_defaults(func=cmd_render)

    args = parser.parse_args()
    if getattr(args, "command", None) and args.command[0] == "--":
        args.command = args.command[1:]
    sys.exit(args.func(args))


if __name__ == "__main__":
    main()
