"""Offline PostgreSQL proof of bootstrap grants; run with python in the Nix shell.
No cloud, Docker, passwords, or external connections. Own ephemeral Unix socket.
"""
import pathlib
import subprocess
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]


class SqlGrants(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tmp = tempfile.TemporaryDirectory(prefix="rust-env-sql-")
        cls.base = pathlib.Path(cls.tmp.name)
        cls.data = cls.base / "data"
        subprocess.run(["initdb", "-D", str(cls.data), "-A", "trust", "-U", "postgres"], check=True, capture_output=True)
        subprocess.run(["pg_ctl", "-D", str(cls.data), "-l", str(cls.base / "postgres.log"), "-o", f"-k {cls.base} -h '' -p 55489", "-w", "start"], check=True, capture_output=True)
        cls.psql = ["psql", "-h", str(cls.base), "-p", "55489", "-U", "postgres", "-v", "ON_ERROR_STOP=1"]
        cls.sql("CREATE ROLE runtime LOGIN; CREATE ROLE migrator LOGIN;", database="postgres")
        cls.sql("CREATE DATABASE app;", database="postgres")
        subprocess.run(cls.psql + ["-d", "app", "-v", "database=app", "-v", "runtime_user=runtime", "-v", "migration_user=migrator", "-f", str(ROOT / "bootstrap-grants.sql")], check=True, capture_output=True)
        cls.sql("SET SESSION AUTHORIZATION migrator; CREATE TABLE example(id serial PRIMARY KEY, body text); INSERT INTO example(body) VALUES ('synthetic');")

    @classmethod
    def tearDownClass(cls):
        subprocess.run(["pg_ctl", "-D", str(cls.data), "-m", "immediate", "-w", "stop"], check=True, capture_output=True)
        cls.tmp.cleanup()

    @classmethod
    def sql(cls, statement, database="app", check=True, user="postgres"):
        return subprocess.run(cls.psql + ["-U", user, "-d", database, "-Atc", statement], check=check, capture_output=True, text=True)

    def test_runtime_dml_without_ddl(self):
        self.sql("INSERT INTO example(body) VALUES ('test'); UPDATE example SET body='updated'; SELECT * FROM example; DELETE FROM example;", user="runtime")
        for statement in ["CREATE TABLE forbidden(id int)", "ALTER TABLE example ADD COLUMN forbidden int", "SET ROLE migrator", "CREATE SCHEMA forbidden"]:
            with self.subTest(statement=statement):
                self.assertNotEqual(self.sql(statement, check=False, user="runtime").returncode, 0)

    def test_no_database_or_role_administration(self):
        for role in ["runtime", "migrator"]:
            for statement in ["CREATE DATABASE forbidden", "CREATE ROLE forbidden"]:
                with self.subTest(role=role, statement=statement):
                    self.assertNotEqual(self.sql(statement, check=False, user=role).returncode, 0)

    def test_connection_caps(self):
        self.assertEqual(self.sql("SELECT rolconnlimit FROM pg_roles WHERE rolname='runtime'").stdout.strip(), "40")
        self.assertEqual(self.sql("SELECT rolconnlimit FROM pg_roles WHERE rolname='migrator'").stdout.strip(), "4")

    def test_function_execution_is_not_public_by_default(self):
        self.sql("SET SESSION AUTHORIZATION migrator; CREATE FUNCTION private_probe() RETURNS integer LANGUAGE sql AS 'SELECT 1';")
        self.assertEqual(self.sql("SELECT private_probe()", user="migrator").stdout.strip(), "1")
        self.assertNotEqual(self.sql("SELECT private_probe()", check=False, user="runtime").returncode, 0)

    def test_rerun(self):
        subprocess.run(self.psql + ["-d", "app", "-v", "database=app", "-v", "runtime_user=runtime", "-v", "migration_user=migrator", "-f", str(ROOT / "bootstrap-grants.sql")], check=True, capture_output=True)


if __name__ == "__main__":
    unittest.main()
