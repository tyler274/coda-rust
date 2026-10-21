-- duet-release: online
begin;

-- PostgreSQL grants EXECUTE to PUBLIC on every new function unless the
-- creating role's global default privileges say otherwise. A per-schema
-- default can only add to the global one, never remove from it, so this is
-- the form that covers every function the migrating role creates from here
-- on. It applies to the role running migrations: duet_owner in production.
alter default privileges revoke execute on functions from public;

-- The two functions 0051 created before that default existed. Both are
-- trigger functions, and a trigger fires without EXECUTE on its function.
revoke execute on function public.invalidate_doctor_preparation_work(),
  public.notify_doctor_preparation_work() from public;

insert into public.schema_migrations(version) values ('0079_function_execute_privileges');
commit;
