#!/usr/bin/env python3
"""Validate repository-owned engineering skills without launching their tools."""

import ast
import json
from pathlib import Path
import re
import subprocess
import sys

import yaml


MODULES = {".ts", ".mts", ".js", ".mjs"}
RELATIVE_IMPORT = re.compile(r"""\b(?:from|import)\s*\(?\s*(["'])(\.{1,2}/[^"']+)\1""")


def check_relative_imports(path):
    """Node resolves a module's relative imports as written, so each must name a file."""
    for _quote, target in RELATIVE_IMPORT.findall(path.read_text()):
        if not (path.parent / target).is_file():
            raise ValueError(f"{path}: imports {target}, which does not exist")


def check(root):
    skills = root / ".agents/skills"
    discovery = root / ".claude/skills"
    folders = sorted(path for path in skills.iterdir() if path.is_dir())
    if not folders:
        raise ValueError("No engineering skills found")
    for folder in folders:
        path = folder / "SKILL.md"
        text = path.read_text()
        match = re.match(r"\A---\r?\n(.*?)\r?\n---(?:\r?\n|$)", text, re.DOTALL)
        if not match:
            raise ValueError(f"{path}: missing YAML frontmatter")
        metadata = yaml.safe_load(match.group(1))
        if not isinstance(metadata, dict) or metadata.get("name") != folder.name:
            raise ValueError(f"{path}: name must match the skill directory")
        if not isinstance(metadata.get("description"), str) or not metadata["description"].strip():
            raise ValueError(f"{path}: description must be a nonempty string")
        link = discovery / folder.name
        if not link.is_symlink() or link.resolve() != folder.resolve():
            raise ValueError(f"{link}: must link to {folder}")
    for link in discovery.iterdir():
        if not link.is_symlink() or link.resolve() != (skills / link.name).resolve() or not link.exists():
            raise ValueError(f"{link}: missing or noncanonical skill link")
    for path in sorted(skills.rglob("*")):
        if not path.is_file():
            continue
        if path.suffix in MODULES:
            check_relative_imports(path)
        if path.suffix == ".py":
            ast.parse(path.read_text(), filename=str(path))
        elif path.suffix in {".yaml", ".yml"}:
            yaml.safe_load(path.read_text())
        elif path.suffix == ".json":
            json.loads(path.read_text())
        elif path.suffix in {".js", ".mjs", ".cjs"}:
            subprocess.run(["node", "--check", str(path)], check=True)
        elif path.suffix == ".sh":
            subprocess.run(["bash", "-n", str(path)], check=True)
    print(f"Validated {len(folders)} engineering skills, discovery links, metadata and script syntax.")


if __name__ == "__main__":
    try:
        check(Path.cwd())
    except (OSError, ValueError, SyntaxError, yaml.YAMLError, subprocess.CalledProcessError) as error:
        sys.exit(str(error))
