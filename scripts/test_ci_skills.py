"""Protect portable skill discovery and reject malformed instructions/scripts."""

import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("check_engineering_skills.py")


class SkillChecks(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.skill = self.root / ".agents/skills/example"
        self.skill.mkdir(parents=True)
        self.instructions = self.skill / "SKILL.md"
        self.instructions.write_text("---\nname: example\ndescription: A usable skill\n---\n# Example\n")
        self.discovery = self.root / ".claude/skills"
        self.discovery.mkdir(parents=True)
        (self.discovery / "example").symlink_to("../../.agents/skills/example")

    def check(self, success, detail=""):
        result = subprocess.run([sys.executable, str(SCRIPT)], cwd=self.root,
                                env=os.environ, capture_output=True, text=True)
        self.assertEqual(result.returncode == 0, success, result.stdout + result.stderr)
        if detail:
            self.assertIn(detail, result.stderr)

    def test_portable_skill_and_script_are_validated_without_execution(self):
        (self.skill / "danger.py").write_text("raise RuntimeError('must not execute')\n")
        self.check(True)

    def test_invalid_metadata_fails(self):
        for text, reason in [("# No metadata", "frontmatter"),
                             ("---\nname: wrong\ndescription: Fine\n---\n", "name"),
                             ("---\nname: example\ndescription: ''\n---\n", "description"),
                             ("---\nname: [unterminated\n---\n", "parsing")]:
            with self.subTest(reason=reason):
                self.instructions.write_text(text)
                self.check(False, reason)

    def test_discovery_must_resolve_to_canonical_skill(self):
        link = self.discovery / "example"
        link.unlink()
        self.check(False, "must link")
        link.symlink_to("../../.agents/skills/missing")
        self.check(False, "must link")
        link.unlink()
        link.symlink_to("../../.agents/skills/example")
        (self.discovery / "obsolete").symlink_to("../../.agents/skills/missing")
        self.check(False, "noncanonical")

    def test_relative_imports_must_name_existing_files(self):
        script = self.skill / "scripts/run.ts"
        script.parent.mkdir()
        script.write_text('import fs from "node:fs";\nimport { stop } from "./stop.ts";\n'
                          'export { draw } from "../draw.mjs";\n')
        self.check(False, "imports ./stop.ts")
        (script.parent / "stop.ts").write_text("export const stop = () => {};\n")
        self.check(False, "imports ../draw.mjs")
        (self.skill / "draw.mjs").write_text("export const draw = () => {};\n")
        self.check(True)
        (self.skill / "draw.mjs").write_text('export const draw = () => import("./late.mjs");\n')
        self.check(False, "imports ./late.mjs")

    def test_invalid_script_and_configuration_fail(self):
        for name, text in [("script.py", "def broken(\n"),
                           ("reviewer-app.json", "{broken"),
                           ("agents.yaml", "interface: [broken")]:
            with self.subTest(name=name):
                path = self.skill / name
                path.write_text(text)
                self.check(False)
                path.unlink()


if __name__ == "__main__":
    unittest.main()
