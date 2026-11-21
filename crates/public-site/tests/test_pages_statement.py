"""Verify the Pages-only statement without building the product applications."""
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest
from html.parser import HTMLParser

ROOT = Path(__file__).resolve().parents[3]
STATEMENT = Path("crates/public-site/pages-only/personal-clarification/index.html")


class Text(HTMLParser):
    def __init__(self):
        super().__init__()
        self.in_main = False
        self.parts = []

    def handle_starttag(self, tag, attrs):
        if tag == "main":
            self.in_main = True

    def handle_endtag(self, tag):
        if tag == "main":
            self.in_main = False

    def handle_data(self, data):
        if self.in_main and data.strip():
            self.parts.append(data.strip())


class PagesStatement(unittest.TestCase):
    def test_approved_text(self):
        parser = Text()
        parser.feed((ROOT / STATEMENT).read_text())
        self.assertEqual(parser.parts, [
            "A personal clarification",
            "I created a Discord server to talk with people. Creating that space does not, by itself, settle whether a particular interaction was respectful or hurtful.",
            "If you disagree with something I said, please identify the specific statement so I can respond clearly. I will distinguish what I directly observed from what I suspect, and I ask others not to speculate about my mental health.",
            "This is a personal statement, not a statement on behalf of Coda Health.",
        ])

    def test_workflow_domain_gate(self):
        workflow = (ROOT / ".github/workflows/pages.yml").read_text()
        step = workflow.split("      - name: Include the approved personal statement", 1)[1]
        script = step.split("        run: |\n", 1)[1].split("      - name:", 1)[0]
        script = "\n".join(line[10:] for line in script.splitlines())
        for origin, included in [
            ("https://coda.dabney.moe", True),
            ("http://coda.dabney.moe", True),
            ("https://codahealth.ai", False),
            ("https://www.codahealth.ai", False),
            ("https://tyler274.github.io/coda-rust", False),
            ("https://coda.dabney.moe.example.com", False),
            ("", False),
        ]:
            with self.subTest(origin=origin), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                source = root / STATEMENT
                source.parent.mkdir(parents=True)
                shutil.copyfile(ROOT / STATEMENT, source)
                subprocess.run(
                    ["bash", "--noprofile", "--norc", "-eu", "-c", script],
                    cwd=root, env={**os.environ, "BASE_URL": origin}, check=True,
                )
                target = root / "_site/personal-clarification/index.html"
                self.assertEqual(target.exists(), included)
                if included:
                    self.assertEqual(target.read_bytes(), source.read_bytes())

    def test_not_in_runtime_assets_or_routes(self):
        build = (ROOT / "crates/public-site/build.rs").read_text()
        self.assertIn('Path::new("assets")', build)
        self.assertNotIn("pages-only", build)
        route = (ROOT / "crates/public-site/src/route.rs").read_text()
        self.assertNotIn("personal-clarification", route)


if __name__ == "__main__":
    unittest.main()
