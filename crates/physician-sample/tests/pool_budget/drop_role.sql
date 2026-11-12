-- Ends the role's sessions and drops it, run as the cluster admin.
-- psql -v role=... -f drop_role.sql
\set ON_ERROR_STOP on
SELECT
    pg_terminate_backend(pid)
FROM
    pg_stat_activity
WHERE
    usename = :'role';

DROP ROLE IF EXISTS :"role";
