-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- The AI switch: operator state that the API, worker and jobs all read, with
-- its own audit trail. Each change appends one row saying who changed it,
-- when and why; the newest row is the current state. Rows are never updated
-- or deleted, so the history is the state. Model execution starts on.
--
-- Online: a new table the serving image never reads. Its seed row is
-- inserted into that new table, not a backfill of an existing one.
create table public.model_execution_switch (
    id bigint generated always as identity primary key,
    enabled boolean not null,
    changed_by text not null check (char_length(btrim(changed_by)) between 1 and 200),
    reason text not null check (char_length(btrim(reason)) between 1 and 1000),
    changed_at timestamptz not null default now()
);

create function public.model_execution_switch_append_only() returns trigger
language plpgsql as $$
begin
    raise exception 'model_execution_switch is append-only; insert a new row to change it';
end $$;

create trigger model_execution_switch_rows_append_only
    before update or delete on public.model_execution_switch
    for each row execute function public.model_execution_switch_append_only();
create trigger model_execution_switch_append_only
    before truncate on public.model_execution_switch
    for each statement execute function public.model_execution_switch_append_only();

insert into public.model_execution_switch(enabled,changed_by,reason)
values (true, 'migration 0073', 'Model execution starts on.');

revoke all on public.model_execution_switch from public;
revoke all on function public.model_execution_switch_append_only() from public;
-- Operator-only: the runtime role reads the switch and cannot change it.
-- Default privileges give that role DML on every new table, so take it back
-- here; scripts/gcp/split_database_roles.py keeps the same rule on a rerun.
-- The operator writes through the owner role.
do $$
begin
    if exists (select 1 from pg_roles where rolname = 'duet_runtime') then
        revoke all on public.model_execution_switch from duet_runtime;
        grant select on public.model_execution_switch to duet_runtime;
    end if;
end $$;

insert into public.schema_migrations(version) values('0073_model_execution_switch');
commit;
