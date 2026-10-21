-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Frozen daily reply-feedback counts. The first operator report that covers a
-- completed UTC day records the day and stores its reportable groups; every
-- later report reads them, so later feedback changes and account deletions
-- never change a reported day. A day is frozen as it is when first read,
-- including the days before the first report. A group is a number of people
-- (at least 5, the smallest reported group) with its rating, reason (null for
-- the rating's own group), model and instruction version: no account,
-- message, run, note or reply text, so nothing here belongs to one person.
--
-- Online: the serving image never reads these tables, and new empty tables
-- build their indexes and checks without scanning or rewriting anything.
create table public.conversation_reply_feedback_days (
    day date primary key,
    frozen_at timestamptz not null default now()
);
create table public.conversation_reply_feedback_groups (
    day date not null references public.conversation_reply_feedback_days(day),
    rating text not null check (rating in ('up','down')),
    reason text,
    model text,
    instruction_version text,
    people integer not null check (people >= 5)
);
create index conversation_reply_feedback_groups_day
    on public.conversation_reply_feedback_groups(day);

revoke all on public.conversation_reply_feedback_days from public;
revoke all on public.conversation_reply_feedback_groups from public;
insert into public.schema_migrations(version) values('0078_conversation_reply_feedback_days');
commit;
