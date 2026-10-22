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
-- Cloud SQL administrators are not superusers, and PostgreSQL refuses even a
-- no-op SUPERUSER/REPLICATION change from them. Clear what the administrator
-- may clear, then refuse to continue if either elevated attribute is present.
ALTER ROLE :"runtime_user" NOCREATEDB NOCREATEROLE;
ALTER ROLE :"migration_user" NOCREATEDB NOCREATEROLE;
-- psql variables do not expand inside DO bodies; pass the names as settings.
SELECT
    set_config('coda.runtime_user', :'runtime_user', TRUE),
    set_config('coda.migration_user', :'migration_user', TRUE);
DO $$
BEGIN
    IF EXISTS (
        SELECT
            1
        FROM
            pg_roles
        WHERE
            rolname IN (current_setting('coda.runtime_user'), current_setting('coda.migration_user'))
            AND (rolsuper
                OR rolreplication
                OR rolbypassrls)) THEN
    RAISE EXCEPTION 'refusing: an application principal has SUPERUSER, REPLICATION or BYPASSRLS';
END IF;
END
$$;
-- Runtime gets no schema ownership/CREATE or migration role membership.
ALTER SCHEMA public OWNER TO :"migration_user";
-- The migrator may assume (never inherit) the runtime role, so duet-migrate can
-- prove the runtime's effective grants with SET LOCAL ROLE before committing.
-- Grants flow migrator -> runtime only; the runtime gains nothing.
GRANT :"runtime_user" TO :"migration_user" WITH INHERIT FALSE,
SET TRUE;
GRANT USAGE ON SCHEMA public TO :"runtime_user";
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO :"runtime_user";
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO :"runtime_user";
REVOKE EXECUTE ON ALL FUNCTIONS IN SCHEMA public FROM PUBLIC;
ALTER DEFAULT PRIVILEGES FOR ROLE :"migration_user" IN SCHEMA public GRANT
SELECT
, INSERT, UPDATE, DELETE ON TABLES TO :"runtime_user";
ALTER DEFAULT PRIVILEGES FOR ROLE :"migration_user" IN SCHEMA public GRANT USAGE,
SELECT
    ON SEQUENCES TO :"runtime_user";
-- PostgreSQL grants PUBLIC EXECUTE globally by default; a schema-local revoke
-- cannot remove that global default. Revoke globally for this creating role.
ALTER DEFAULT PRIVILEGES FOR ROLE :"migration_user" REVOKE EXECUTE ON FUNCTIONS FROM PUBLIC;
-- At most 5 instances x 2 overlapping revisions x pool 4 = 40, reserving room
-- below max_connections=100 for migrations/operators. Role limits also bound
-- connections if a runtime pool is misconfigured. Model limits require runtime
-- enforcement/provider quotas; neither budgets nor these SQL grants enforce it.
ALTER ROLE :"runtime_user" CONNECTION
LIMIT 40;
ALTER ROLE :"migration_user" CONNECTION
LIMIT 4;
COMMIT;
