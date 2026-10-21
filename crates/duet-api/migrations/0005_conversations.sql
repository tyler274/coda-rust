-- Accepted work is the sole execution owner; no independent harness run ledger.
create table public.duet_conversations (
    user_id uuid not null references public.users (id) on delete cascade,
    id uuid not null,
    revision bigint not null default 0,
    created_at timestamptz not null default now(),
    primary key (user_id, id)
);

create table public.duet_commands (
    user_id uuid not null references public.users (id) on delete cascade,
    id uuid not null,
    conversation_id uuid not null,
    context jsonb not null,
    input text not null,
    base_revision bigint not null,
    request jsonb not null,
    state text not null default 'accepted' check (state in ('accepted', 'running', 'complete', 'cancelled')),
    lease_id uuid,
    lease_until timestamptz,
    claim bigint not null default 0,
    memory_revision bigint not null default -1,
    staged_memory jsonb,
    reply jsonb,
    created_at timestamptz not null default now(),
    primary key (user_id, id),
    foreign key (user_id, conversation_id) references public.duet_conversations (user_id, id)
);

create unique index duet_one_active_conversation on public.duet_commands (user_id, conversation_id)
where
    state in ('accepted', 'running');

create table public.duet_execution_records (
    user_id uuid not null,
    command_id uuid not null,
    ordinal bigint generated always as identity,
    claim bigint not null,
    admissible boolean not null,
    record jsonb not null,
    primary key (user_id, command_id, ordinal),
    foreign key (user_id, command_id) references public.duet_commands (user_id, id) on delete cascade
);

create table public.duet_tool_effects (
    user_id uuid not null,
    command_id uuid not null,
    invocation_id text not null,
    call jsonb not null,
    result jsonb not null,
    primary key (user_id, command_id, invocation_id),
    foreign key (user_id, command_id) references public.duet_commands (user_id, id) on delete cascade
);

create table public.duet_private_memory (
    user_id uuid primary key references public.users (id) on delete cascade,
    revision bigint not null default 0,
    content jsonb not null
);

create table public.duet_conversation_events (
    user_id uuid not null,
    command_id uuid not null,
    sequence bigint
        generated always as identity,
        event jsonb not null,
        primary key (user_id, command_id, sequence),
        foreign key (user_id, command_id) references public.duet_commands (user_id, id) on delete cascade
);

do $$
declare
    t text;
begin
    foreach t in array array['duet_conversations', 'duet_commands', 'duet_execution_records', 'duet_tool_effects', 'duet_private_memory', 'duet_conversation_events'] loop
        execute format('alter table public.%I enable row level security', t);
        execute format('alter table public.%I force row level security', t);
        execute format('create policy account_isolation on public.%I using(user_id=public.acting_account()) with check(user_id=public.acting_account())', t);
    end loop;
end
$$;

-- Recovery directory carries only routing identities, never input or provider context.
create view public.duet_pending_commands as
select
    user_id,
    id
from
    public.duet_commands
where
    state in ('accepted', 'running')
    and (lease_until is null
        or lease_until < now());

revoke all on public.duet_pending_commands from public;
