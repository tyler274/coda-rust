-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- The existing owner now distinguishes contextual Check-ins, which need the
-- agent at occurrence time, from fixed-text reminders, whose accepted text is
-- itself the complete result. Existing accepted work remains a Check-in.
alter table public.follow_ups
    add column operation text not null default 'check_in'
        check(operation in ('check_in','reminder'));

insert into public.schema_migrations(version)
values('0062_requested_follow_up_operations');
commit;
