-- Removes the role's grants in the test database, run as its owner.
-- psql -v role=... -f drop_owned.sql
\set ON_ERROR_STOP on
DROP OWNED BY :"role";
