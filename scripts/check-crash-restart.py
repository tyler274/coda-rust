#!/usr/bin/env python3
"""Run a compiled crash-restart test as an unprivileged user on disposable Postgres."""
import os
from pathlib import Path
import subprocess
import sys
import tempfile

if len(sys.argv) != 2:
    sys.exit("Usage: check-crash-restart.py <runtime_postgres test executable>")
binary = sys.argv[1]

# The restart test only accepts this disposable path and restarts on port 55489.
with tempfile.TemporaryDirectory(prefix="physician-restart-qualification-", dir="/tmp") as directory:
    cluster = Path(directory)
    env = {**os.environ,
           "PHYSICIAN_RUNTIME_TEST_DATABASE_URL": "postgresql://postgres@127.0.0.1:55489/postgres",
           "PHYSICIAN_RUNTIME_TEST_PGDATA": str(cluster)}
    subprocess.run(["initdb", "-U", "postgres", "-A", "trust", "-D", directory],
                   check=True, stdout=subprocess.DEVNULL)
    try:
        subprocess.run(["pg_ctl", "-D", directory, "-l", str(cluster / "startup.log"),
                        "-o", "-p 55489 -h 127.0.0.1 -k /tmp", "-w", "start"], check=True)
        print(f"     Running isolated restart ({binary})", flush=True)
        subprocess.run([binary, "--exact", "database_crash_restart_retains_acknowledged_command_and_event",
                        "--ignored", "--show-output"], env=env, check=True)
    finally:
        if (cluster / "postmaster.pid").exists():
            subprocess.run(["pg_ctl", "-D", directory, "-m", "fast", "-w", "stop"], check=True)
