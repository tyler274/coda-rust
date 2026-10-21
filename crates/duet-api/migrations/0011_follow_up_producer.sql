-- Fresh follow-through uses the existing retained owner, not an import endpoint.
-- Migration owner executes this file; runtime grants are deliberately explicit in
-- database::migrate. Do not broaden health_contexts/insight_cycles privileges.
create table public.duet_follow_up_origins (
    user_id uuid not null,
    follow_up_id uuid not null,
    command_id uuid not null,
    creation_identity jsonb not null,
    primary key (user_id, follow_up_id),
    foreign key (user_id, follow_up_id) references public.follow_ups (user_id, id) on delete cascade,
    foreign key (user_id, command_id) references public.duet_commands (user_id, id)
);

create table public.duet_follow_up_occurrences (
    user_id uuid not null,
    id uuid not null,
    follow_up_id uuid not null,
    follow_up_revision bigint not null check (follow_up_revision > 0),
    scheduled_for timestamptz not null,
    command_id uuid not null,
    operation text not null check (operation in ('reminder', 'check_in')),
    accepted_instruction text not null,
    status text not null default 'queued' check (status in ('queued', 'running', 'completed', 'unavailable', 'failed', 'interrupted')),
    conversation_message_id uuid,
    completed_at timestamptz,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    primary key (user_id, id),
    unique (user_id, command_id),
    unique (user_id, follow_up_id, follow_up_revision, scheduled_for),
    foreign key (user_id, follow_up_id) references public.follow_ups (user_id, id) on delete cascade,
    -- The existing conversation owner inserts its assistant-only scheduled command
    -- in the SAME transaction after reserve_due returns its frozen assignment.
    foreign key (user_id, command_id) references public.duet_commands (user_id, id) deferrable initially deferred,
    check ((status in ('queued', 'running')) = (completed_at is null)),
    check (status = 'completed' or conversation_message_id is null)
);

create unique index duet_follow_up_one_unfinished on public.duet_follow_up_occurrences (user_id, follow_up_id)
where
    status in ('queued', 'running');

create index duet_follow_up_due on public.follow_ups (user_id, next_occurrence_at, id)
where
    state = 'active' and not confirmation_required and next_occurrence_at is not null;

-- Holds are account controls, not rewritten accepted definitions.
create table public.duet_follow_up_settings (
    user_id uuid primary key references public.users (id) on delete cascade,
    revision bigint not null default 1 check (revision > 0),
    enabled boolean not null default true
);

do $$
declare
    t text;
begin
    foreach t in array array['duet_follow_up_origins', 'duet_follow_up_occurrences', 'duet_follow_up_settings'] loop
        execute format('alter table public.%I enable row level security', t);
        execute format('alter table public.%I force row level security', t);
        execute format('create policy duet_follow_up_account on public.%I using(user_id=public.acting_account()) with check(user_id=public.acting_account())', t);
    end loop;
end
$$;

revoke all on public.duet_follow_up_origins, public.duet_follow_up_occurrences, public.duet_follow_up_settings from public;

-- Non-health discovery only; it grants neither admission nor execution authority.
create view public.duet_due_follow_up_accounts as select distinct
    user_id
from
    public.follow_ups
where
    state = 'active'
    and not confirmation_required
    and next_occurrence_at <= now();

revoke all on public.duet_due_follow_up_accounts from public;
