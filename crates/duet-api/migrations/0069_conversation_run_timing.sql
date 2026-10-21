-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Durable execution timing for each conversation run, in the database clock
-- that already stamps created_at. started_at is written once, by the claim's
-- queued-to-running update. first_output_at is written once, when the first
-- visible assistant text is persisted (a streamed delta, or the published text
-- of a run that does not stream). Queue wait and time to first text are
-- derived from these and created_at; no derived value is stored.
--
-- Online contract: both columns are nullable with no default, so adding them
-- rewrites no rows. The serving image never names them: runs it claims or
-- streams keep NULL, which reads as "not recorded", never as zero. Existing
-- rows are not backfilled because their start and first-text times were never
-- observed.
alter table public.conversation_runs
    add column started_at timestamptz,
    add column first_output_at timestamptz;

insert into public.schema_migrations(version) values('0069_conversation_run_timing');
commit;
