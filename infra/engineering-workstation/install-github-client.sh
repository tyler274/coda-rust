#!/usr/bin/env bash
# Prepare company clients. Git cutover and personal logout follow live proof.
set -euo pipefail
if [[ $EUID == 0 || $# -ne 1 ]]; then
  echo "Usage: bash $0 OWNER/REPOSITORY (as the desktop owner)" >&2
  exit 1
fi
recipe_dir=$(cd -- "$(dirname -- "$0")" && pwd)
bridge_dir="$HOME/.local/share/coda-engineering/slack-bridge"
test -x "$bridge_dir/.venv/bin/python"
install -m 600 "$recipe_dir/bridge/github_access.py" "$bridge_dir/github_access.py"
python3 - "$bridge_dir" "$1" <<'PY'
from pathlib import Path
import re, shlex, sys
directory, repository = sys.argv[1:]
if not re.fullmatch(r'[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+', repository):
    raise SystemExit('Use owner/repository')
binary_dir = Path.home() / '.local/bin'
binary_dir.mkdir(parents=True, exist_ok=True)
for name, mode in (('git-credential-coda-company', 'git-credential'), ('coda-gh', 'gh')):
    arguments = [str(Path(directory) / '.venv/bin/python'), str(Path(directory) / 'github_access.py'),
                 '--repository', repository, mode]
    launcher = binary_dir / name
    launcher.write_text('#!/bin/sh\nexec ' + shlex.join(arguments) + ' "$@"\n')
    launcher.chmod(0o700)
PY
"$bridge_dir/.venv/bin/python" "$recipe_dir/bridge/client_setup.py" git \
  --repository "$1" \
  --helper "$HOME/.local/bin/git-credential-coda-company" \
  --gitconfig "$HOME/.gitconfig"
echo "Company Git helper registered for https://github.com/$1 only. Prove fetch and CI logs before removing personal authentication."
