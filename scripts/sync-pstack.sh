#!/usr/bin/env bash
# Run in a clean Coda checkout with a full-history cursor/plugins clone as the argument.
set -euo pipefail

upstream=${1:?usage: sync-pstack.sh /path/to/cursor-plugins}
revision=$(git -C "$upstream" log -1 --format=%H HEAD -- pstack/skills ':!pstack/skills/statechart/scripts/test')
# poteto-help links to the plugin's own docs and README, which live outside pstack/skills.
mapfile -t skills < <(git -C "$upstream" ls-tree --name-only "$revision" pstack/skills/ | xargs -n1 basename | grep -vx poteto-help | sort)
test "${#skills[@]}" -gt 0
paths=("${skills[@]/#/pstack/skills/}")
snapshot=$(mktemp -d)
trap 'rm -rf "$snapshot"' EXIT
git -C "$upstream" archive "$revision" pstack/LICENSE "${paths[@]}" | tar -x -C "$snapshot"

# Validate the whole import before replacing any files.
for skill in "${skills[@]}"; do
  test -f "$snapshot/pstack/skills/$skill/SKILL.md"
done
rm -rf "$snapshot/pstack/skills/statechart/scripts/test"
for skill in "${skills[@]}"; do
  rsync -a --delete "$snapshot/pstack/skills/$skill/" ".agents/skills/$skill/"
  test -e ".claude/skills/$skill" || ln -s "../../.agents/skills/$skill" ".claude/skills/$skill"
done

if [[ -z $(git status --porcelain -- "${skills[@]/#/.agents/skills/}" "${skills[@]/#/.claude/skills/}") ]]; then
  echo "PStack skills are current."
  exit 0
fi

cp "$snapshot/pstack/LICENSE" verification/PSTACK-LICENSE
python3 - "$revision" <<'PY'
from pathlib import Path
import re
import sys

revision = sys.argv[1]
readme = Path("verification/README.md")
link = f"[PStack at {revision[:7]}](https://github.com/cursor/plugins/tree/{revision}/pstack/skills)"
text, count = re.subn(r"\[PStack at [0-9a-f]+\]\(https://github\.com/cursor/plugins/tree/[0-9a-f]{40}/pstack/skills\)", link, readme.read_text())
if count != 1:
    raise SystemExit("Expected one upstream PStack revision link in verification/README.md")
readme.write_text(text)
PY
