-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- A person's own 👍/👎 on one of Duet's replies, kept in their account. One
-- row per (account, assistant message). It references the reply and the run
-- that wrote it and copies none of their content: the run's
-- generation_metadata already records the model, instruction version, skills
-- and receipts. The optional note is the person's words and never leaves the
-- account. Cleared feedback keeps its row with no rating, so a live update
-- can carry the clearing at its revision; the row has no content then.
--
-- Online: the serving image never reads this table, and a new empty table
-- builds its indexes and checks without scanning or rewriting anything. Its
-- foreign keys briefly lock conversation_messages and conversation_runs
-- against writes while the constraints are created, without a scan.
create table public.conversation_reply_feedback (
    user_id uuid not null,
    message_id uuid not null,
    run_id uuid not null references public.conversation_runs(id) on delete cascade,
    rating text check (rating in ('up','down')),
    reasons text[] not null default '{}' check (cardinality(reasons) <= 8),
    note text check (char_length(note) between 1 and 1000),
    revision bigint not null check (revision > 0),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    primary key (user_id, message_id),
    foreign key (user_id, message_id)
        references public.conversation_messages(user_id, id) on delete cascade,
    check (rating is not null or (cardinality(reasons) = 0 and note is null))
);
create index conversation_reply_feedback_changes
    on public.conversation_reply_feedback(user_id, revision);
create index conversation_reply_feedback_updated
    on public.conversation_reply_feedback(updated_at) where rating is not null;
create index conversation_reply_feedback_run
    on public.conversation_reply_feedback(run_id);

revoke all on public.conversation_reply_feedback from public;
insert into public.schema_migrations(version) values('0076_conversation_reply_feedback');
commit;
