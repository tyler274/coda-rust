#!/usr/bin/env python3
"""Check tracked authored SQL (not generated SQL or arbitrary Rust strings).

Run: nix develop --command python scripts/check-sql.py [--write]
The required Duet schema-readiness query and the physician authority NOTIFY
trigger migration are included by Rust at compile time. All other
tracked .sql files outside read-only submodules are included automatically.
Duet migrations under crates/duet-api/migrations are excluded: each file is
checksummed into duet_schema_migrations, so formatting must not change it.
Migration builders, dynamic Rust templates and test-fixture strings are not
parsed or reformatted. No migration/checksum inputs are changed by this check.
Formatting is not an endorsement of raw SQL: AGENTS.md requires ORM-first CRUD.
These required files describe current coverage, not a permanent SQL quota; update
REQUIRED and the query tests when a bounded ORM conversion removes a query.
"""

import argparse
import difflib
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parent.parent
REQUIRED = {
    "crates/duet-api/src/sql/has_migration.sql",
    # PostgreSQL-only exception: the physician authority NOTIFY trigger has no
    # SeaQuery trigger/PL/pgSQL builder. Replicas LISTEN on physician_authority.
    "crates/physician-auth/src/authority_notify.sql",
}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--write", action="store_true", help="format files in place")
    args = parser.parse_args()
    version = subprocess.check_output(["pg_format", "--version"], text=True).strip()
    if version != "pg_format version 5.11":
        sys.exit(f"Expected pg_format 5.11 from the Nix dev shell; got {version}")
    tracked = subprocess.check_output(
        ["git", "ls-files", "-z", "--", "*.sql", ":!:submodules/**"], cwd=ROOT
    ).decode().split("\0")
    paths = sorted(
        name
        for name in set(filter(None, tracked))
        if not name.startswith("crates/duet-api/migrations/")
        and (ROOT / name).is_file()
    )
    missing = REQUIRED - set(paths)
    if missing:
        sys.exit(f"Required authored SQL is not tracked: {sorted(missing)}")
    if not paths:
        sys.exit("No authored SQL files remain; retire this check explicitly instead of claiming coverage")
    failed = False
    for name in paths:
        path = ROOT / name
        original = path.read_text()
        if not original.strip():
            sys.exit(f"SQL file is empty: {name}")
        formatted = subprocess.check_output(
            ["pg_format", "--no-rcfile", "--config", str(ROOT / ".pg_format"), str(path)],
            text=True,
            cwd=ROOT,
        )
        if original != formatted:
            if args.write:
                path.write_text(formatted)
            else:
                failed = True
                sys.stdout.writelines(difflib.unified_diff(
                    original.splitlines(keepends=True),
                    formatted.splitlines(keepends=True),
                    fromfile=name,
                    tofile=f"{name} (formatted)",
                ))
    print(f"{'Formatted' if args.write else 'Checked'} {len(paths)} authored SQL files with {version}")
    if failed:
        sys.exit("SQL layout differs; run nix develop --command python scripts/check-sql.py --write")


if __name__ == "__main__":
    main()
