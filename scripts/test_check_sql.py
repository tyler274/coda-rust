"""Regression tests for authored-SQL inventory after bounded ORM conversions."""

import contextlib
import importlib.util
import io
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location(
    "check_sql", Path(__file__).with_name("check-sql.py")
)
check_sql = importlib.util.module_from_spec(spec)
spec.loader.exec_module(check_sql)


class SqlInventoryTests(unittest.TestCase):
    def run_check(self, tracked, files, required):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for name, content in files.items():
                path = root / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(content)

            def output(command, **kwargs):
                if command == ["pg_format", "--version"]:
                    return "pg_format version 5.11\n"
                if command[0] == "git":
                    return "\0".join(tracked).encode()
                return Path(command[-1]).read_text()

            capture = io.StringIO()
            with (
                patch.object(check_sql, "ROOT", root),
                patch.object(check_sql, "REQUIRED", required),
                patch.object(check_sql.subprocess, "check_output", side_effect=output),
                patch("sys.argv", ["check-sql.py"]),
                contextlib.redirect_stdout(capture),
            ):
                check_sql.main()
            return capture.getvalue()

    def test_removed_crud_is_not_a_permanent_sql_quota(self):
        surviving = "crates/duet-api/src/sql/has_migration.sql"
        removed = "crates/duet-api/src/sql/list_moments.sql"
        migration = "crates/duet-api/migrations/0001_account_boundary.sql"
        result = self.run_check(
            [surviving, removed, migration],
            {surviving: "select 1;\n", migration: "immutable migration bytes"},
            {surviving},
        )
        self.assertIn("Checked 1 authored SQL files", result)

    def test_missing_required_query_fails_even_if_still_in_git_index(self):
        with self.assertRaisesRegex(SystemExit, "Required authored SQL"):
            self.run_check(["required.sql"], {}, {"required.sql"})

    def test_empty_inventory_cannot_claim_coverage(self):
        with self.assertRaisesRegex(SystemExit, "No authored SQL files remain"):
            self.run_check([], {}, set())

    def test_empty_sql_cannot_replace_a_removed_query(self):
        with self.assertRaisesRegex(SystemExit, "SQL file is empty"):
            self.run_check(["required.sql"], {"required.sql": " \n"}, {"required.sql"})

    def test_actual_required_inventory_keeps_only_postgres_exceptions(self):
        self.assertEqual(
            check_sql.REQUIRED,
            {
                "crates/duet-api/src/sql/has_migration.sql",
                "crates/physician-auth/src/authority_notify.sql",
            },
        )

    def test_required_inventory_files_are_included_by_rust(self):
        root = Path(__file__).resolve().parent.parent
        sources = {
            "crates/duet-api/src/sql/has_migration.sql": "crates/duet-api/src",
            "crates/physician-auth/src/authority_notify.sql": "crates/physician-auth/src",
        }
        for required, crate in sources.items():
            name = Path(required).name
            included = any(
                name in path.read_text()
                for path in (root / crate).rglob("*.rs")
            )
            self.assertTrue(included, f"{required} is not include_str!'d by Rust")


if __name__ == "__main__":
    unittest.main()
