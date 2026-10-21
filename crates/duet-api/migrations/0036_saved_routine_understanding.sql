-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

alter table public.operating_indexes
    add column saved_routine_ids uuid[] not null default '{}';

create function public.invalidate_saved_routine_understanding() returns trigger
language plpgsql set search_path = public as $$
begin
    if tg_op = 'DELETE' then
        delete from public.operating_indexes
        where user_id = old.user_id and old.id = any(saved_routine_ids);
        return old;
    end if;
    update public.operating_indexes set superseded_at = clock_timestamp()
    where user_id = old.user_id and old.id = any(saved_routine_ids);
    return new;
end;
$$;

create trigger saved_routine_understanding
    after update or delete on public.saved_routines
    for each row execute function public.invalidate_saved_routine_understanding();
revoke all on function public.invalidate_saved_routine_understanding() from public;

insert into public.schema_migrations(version) values ('0036_saved_routine_understanding');
commit;
