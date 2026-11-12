-- What the physician runtime role may do, run as the test database's owner.
-- psql -v role=... -f grant_role.sql
\set ON_ERROR_STOP on
GRANT USAGE ON SCHEMA public TO :"role";

GRANT ALL ON ALL TABLES IN SCHEMA public TO :"role";

GRANT ALL ON ALL SEQUENCES IN SCHEMA public TO :"role";
