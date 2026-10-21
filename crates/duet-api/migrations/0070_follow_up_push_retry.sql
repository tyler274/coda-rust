-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- A follow-up notification is operational delivery state, never health
-- content: it expires a day after its reply was saved, a person's older
-- pending notices are superseded by their newest, and a known transient APNs
-- failure (429 or 5xx) returns the row to pending until next_attempt_at.
-- next_attempt_at is not exported and is removed with the row by the
-- unchanged account cascade.
--
-- Planned maintenance: replacing the state CHECK validates every row under
-- ACCESS EXCLUSIVE, which is not lock-safe beside live traffic (see
-- README.md). The application contract would allow coexistence: the column is
-- nullable with no default, so adding it rewrites no rows, and NULL means "due
-- now", which is how the prior image already treats every pending row. The
-- two new states are only written by the new image; the serving image selects
-- only 'pending' rows and settles only the states it already knows, so it
-- never reads or writes them. Its one deployed sender is the worker's attempt for the run it just finished, a
-- fresh row that is neither expired nor backed off.
alter table public.follow_up_push_deliveries add column next_attempt_at timestamptz;

alter table public.follow_up_push_deliveries
    drop constraint follow_up_push_deliveries_state_check,
    add constraint follow_up_push_deliveries_state_check check (state in (
        'pending','dispatching','accepted','unavailable','suppressed','rejected','uncertain',
        'expired','superseded'));

insert into public.schema_migrations(version) values('0070_follow_up_push_retry');
commit;
