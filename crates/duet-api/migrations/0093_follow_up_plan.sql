-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- A follow-up belongs to at most one of the same account's Plans. The link is
-- presentation, never execution authority: deleting the Plan clears it and
-- leaves the follow-up running.
--
-- Online: a nullable column with no default is metadata-only, and the foreign
-- key is NOT VALID because every existing row is null. The serving runtime
-- never writes the column and reads follow-ups by explicit field, so it
-- ignores the link; its Plan deletions still clear links through the key.
alter table public.follow_ups
    add column routine_id uuid,
    add constraint follow_ups_routine_fkey
        foreign key (user_id, routine_id)
        references public.saved_routines(user_id, id)
        on delete set null (routine_id) not valid;

insert into public.schema_migrations(version) values('0093_follow_up_plan');
commit;
