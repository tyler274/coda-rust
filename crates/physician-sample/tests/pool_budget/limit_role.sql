-- How many sessions the role may hold (-1 lifts the limit), run as the
-- cluster admin. psql -v role=... -v connections=... -f limit_role.sql
\set ON_ERROR_STOP on
SELECT
    :'connections'::integer AS connections \gset

ALTER ROLE :"role" CONNECTION
LIMIT :connections;
