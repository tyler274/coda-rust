-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- The admitting owner records which Cloud Tasks lane delivers a run: a
-- person's message or requested Today assessment is interactive; accepted
-- follow-ups and sweep-initiated reviews are scheduled. Enqueue and recovery
-- read this value and never infer it later. Execution ignores it.
--
-- Planned maintenance: the checked NOT NULL column and the backfill below hold
-- ACCESS EXCLUSIVE on conversation_runs through a full scan and update, which
-- is not lock-safe beside live traffic (see README.md). The application
-- contract would allow coexistence: the prior image inserts runs without
-- naming a lane, and its only deployed run producers are person messages and
-- the person-requested Today assessment, so the interactive default is correct
-- for every row it can write. The new image names the lane at every admission.
alter table public.conversation_runs
    add column dispatch_lane text not null default 'interactive'
        check (dispatch_lane in ('interactive','scheduled'));

-- Existing accepted follow-up runs are scheduled work. Existing assessments
-- keep the interactive default: no deployed runtime has admitted a
-- sweep-initiated review, so each retained assessment was person-requested.
update public.conversation_runs set dispatch_lane = 'scheduled'
where follow_up_occurrence_id is not null;

insert into public.schema_migrations(version) values('0068_conversation_dispatch_lane');
commit;
