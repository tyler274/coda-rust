-- A login role for one pool_budget test, run as the cluster admin.
-- psql -v role=... -v password=... -f create_role.sql
\set ON_ERROR_STOP on
CREATE ROLE :"role" LOGIN PASSWORD :'password';
