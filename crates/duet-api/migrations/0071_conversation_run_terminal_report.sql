-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Every started conversation run emits at most one content-free
-- conversation_run_terminal line. terminal_reported_at records that its
-- report was claimed: whoever reports a run sets it with a guarded update
-- ("where terminal_reported_at is null") in the transaction that decides to
-- log, and writes the line after that transaction commits. So a line is never
-- duplicated, and it is lost only if the process dies between that commit and
-- the write; nothing repairs a lost line. The line is an operational signal:
-- the run row's own times and receipts are the durable record. A run is
-- reported by its claim holder, by the claim that settles its expired lease,
-- or by the conversation recovery backstop once a run something else ended
-- has outlived its holder's lease without being reported.
--
-- Nullable with no default and no backfill: existing runs keep NULL. The
-- backstop reports only runs started after this migration's own recorded
-- applied_at, so no historical run is reported and no time is invented.
alter table public.conversation_runs add column terminal_reported_at timestamptz;

-- The backstop's working set: started runs not yet reported. A run leaves
-- the index when it is reported, so it stays as small as the runs in flight.
--
-- Planned maintenance: building an index on this existing table blocks its
-- writes for the whole build, which grows with the table.
create index conversation_runs_terminal_unreported on public.conversation_runs(started_at)
    where terminal_reported_at is null and started_at is not null;

insert into public.schema_migrations(version) values('0071_conversation_run_terminal_report');
commit;
