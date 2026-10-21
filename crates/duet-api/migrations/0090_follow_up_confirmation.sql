-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- A follow-up saved or revised by a reply that read text the person did not
-- write stays paused until the person resumes it. The flag only says why a
-- paused follow-up is paused; state remains the one execution authority.
--
-- Online: a constant default is metadata-only, with no scan or rewrite. The
-- serving runtime never writes the column, so its rows read as not waiting;
-- a resume or pause it performs can leave the flag on a non-paused row, which
-- the new runtime ignores and clears on its next state change.
alter table public.follow_ups
    add column confirmation_required boolean not null default false;

insert into public.schema_migrations(version) values('0090_follow_up_confirmation');
commit;
