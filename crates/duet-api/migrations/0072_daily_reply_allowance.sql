-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- The daily reply allowance counts a person's chat-message runs since the
-- start of their local day and over the last 24 hours, and reads the zone of
-- their latest message, on every admission and availability answer
-- (including each conversation update a stream sends). Without this index
-- both reads visit every run the person ever had, through
-- conversation_run_changes(user_id, revision). EXPLAIN ANALYZE on a local copy
-- with 1 million runs over 10,000 accounts, warm cache: for an account with
-- 50,000 runs the count took 3.0 ms and the zone read 4.9 ms, 1,262 buffers
-- each, growing with its history; with the index, 0.20 ms (432 buffers) and
-- 0.003 ms (4 buffers). A typical account with 95 runs took 0.013 ms without
-- it and 0.006 ms with it.
--
-- Planned maintenance: building an index on this existing table blocks its
-- writes for the whole build, which grows with the table.
create index conversation_runs_daily_messages on public.conversation_runs(user_id,created_at)
    where message_id is not null;

insert into public.schema_migrations(version) values('0072_daily_reply_allowance');
commit;
