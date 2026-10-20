-- Operator-only bootstrap, connected to ONE app database as its authorized
-- administrator. NOT executed by Terraform or either application identity.
-- psql -v database=physician -v runtime_user=... -v migration_user=... -f ...
-- User names come from database_contract output. Repeat separately for Duet.
-- IAM token refresh and private TLS/connector support belong to the runtime.
-- Keep migrations as the migration principal (no SET ROLE to a shared owner).
\set ON_ERROR_STOP on
BEGIN;
REVOKE ALL ON DATABASE :"database" FROM PUBLIC;
REVOKE ALL ON SCHEMA public FROM PUBLIC;
REVOKE ALL ON DATABASE :"database" FROM :"runtime_user";
GRANT CONNECT ON DATABASE :"database" TO :"runtime_user", :"migration_user";
ALTER ROLE :"runtime_user" NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION;
ALTER ROLE :"migration_user" NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION;
-- Runtime gets no schema ownership/CREATE or migration role membership.
ALTER SCHEMA public OWNER TO :"migration_user";
GRANT USAGE ON SCHEMA public TO :"runtime_user";
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO :"runtime_user";
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO :"runtime_user";
REVOKE EXECUTE ON ALL FUNCTIONS IN SCHEMA public FROM PUBLIC;
ALTER DEFAULT PRIVILEGES FOR ROLE :"migration_user" IN SCHEMA public
  GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO :"runtime_user";
ALTER DEFAULT PRIVILEGES FOR ROLE :"migration_user" IN SCHEMA public
  GRANT USAGE, SELECT ON SEQUENCES TO :"runtime_user";
-- PostgreSQL grants PUBLIC EXECUTE globally by default; a schema-local revoke
-- cannot remove that global default. Revoke globally for this creating role.
ALTER DEFAULT PRIVILEGES FOR ROLE :"migration_user"
  REVOKE EXECUTE ON FUNCTIONS FROM PUBLIC;
-- At most 5 instances x 2 overlapping revisions x pool 4 = 40, reserving room
-- below max_connections=100 for migrations/operators. Role limits also bound
-- connections if a runtime pool is misconfigured. Model limits require runtime
-- enforcement/provider quotas; neither budgets nor these SQL grants enforce it.
ALTER ROLE :"runtime_user" CONNECTION LIMIT 40;
ALTER ROLE :"migration_user" CONNECTION LIMIT 4;
COMMIT;
