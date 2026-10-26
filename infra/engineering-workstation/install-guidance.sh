#!/usr/bin/env bash
set -euo pipefail

recipe_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
shared_dir="$HOME/.agents"
mkdir -p "$shared_dir/skills/workstation-communication" "$HOME/.codex/skills" "$HOME/.claude"

# These are one shared owner, not two copies that can drift. Refuse unrelated
# existing instructions so an operator can reconcile them without losing content.
link_shared() {
  local target=$1 link=$2
  if [[ -L "$link" && "$(readlink -- "$link")" == "$target" ]]; then
    return
  fi
  if [[ -e "$link" || -L "$link" ]]; then
    printf 'Refusing to replace existing path: %s\n' "$link" >&2
    exit 1
  fi
  ln -s -- "$target" "$link"
}

for destination in "$HOME/.codex/AGENTS.md" "$HOME/.claude/CLAUDE.md" "$HOME/.claude/skills"; do
  if [[ -e "$destination" && ! -L "$destination" ]]; then
    printf 'Reconcile existing instructions before installing: %s\n' "$destination" >&2
    exit 1
  fi
done
install -m 600 "$recipe_dir/guidance/AGENTS.md" "$shared_dir/AGENTS.md"
install -m 600 "$recipe_dir/guidance/skills/workstation-communication/SKILL.md" \
  "$shared_dir/skills/workstation-communication/SKILL.md"
link_shared "$shared_dir/AGENTS.md" "$HOME/.codex/AGENTS.md"
link_shared "$shared_dir/AGENTS.md" "$HOME/.claude/CLAUDE.md"
link_shared "$shared_dir/skills" "$HOME/.claude/skills"

# Preserve existing personal skills and their old paths while sharing their
# actual source with Claude. Bundled Codex system skills stay with Codex.
for name in agent-coordination repo-graph-audit repo-orientation review-pr test-audit chrome-devtools; do
  source="$HOME/.codex/skills/$name"
  target="$shared_dir/skills/$name"
  if [[ -e "$source" || -L "$source" ]]; then
    if [[ -L "$source" && "$(readlink -- "$source")" == "$target" ]]; then
      continue
    fi
    if [[ -e "$target" || -L "$target" ]]; then
      printf 'Reconcile existing skill before sharing: %s\n' "$name" >&2
      exit 1
    fi
    mv -- "$source" "$target"
    ln -s -- "$target" "$source"
  fi
done
printf 'Shared workstation guidance and skills installed for Codex and Claude Code.\n'
