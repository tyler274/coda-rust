-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Optional presentation metadata. The old runtime may continue writing NULL;
-- both old and new runtimes keep accepted_instruction as execution authority.
alter table public.follow_ups
    add column display_title text
        check(display_title is null or
              (length(display_title) <= 96 and
               length(btrim(display_title)) > 0));

insert into public.schema_migrations(version) values('0067_follow_up_display_title');
commit;
