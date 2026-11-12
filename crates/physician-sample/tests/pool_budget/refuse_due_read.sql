-- Takes away the worker's due read, run as the test database's owner.
-- psql -v role=... -f refuse_due_read.sql
\set ON_ERROR_STOP on
REVOKE SELECT ON physician_runtime_conversations FROM :"role";
