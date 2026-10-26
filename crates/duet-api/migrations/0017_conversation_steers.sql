-- ENG-200: a second message in the same conversation steers the reply in
-- progress. The steer is durable input to the existing accepted command, not a
-- successor run. steer_revision fences out any attempt that started before it.
alter table public.duet_commands
    add column steer_revision bigint not null default 0;

create table public.duet_command_steers (
    user_id uuid not null references public.users (id) on delete cascade,
    id uuid not null,
    command_id uuid not null,
    input text not null,
    submission jsonb not null,
    admission jsonb not null default '{}',
    -- Dispatch ordinal that first carried this steer; null until dispatched.
    applied_sequence bigint check (applied_sequence > 0),
    -- Statement time, after the account lock, so admission order is preserved.
    created_at timestamptz not null default clock_timestamp(),
    primary key (user_id, id),
    foreign key (user_id, command_id) references public.duet_commands (user_id, id) on delete cascade
);

create index duet_command_steers_command on public.duet_command_steers (user_id, command_id, created_at, id);

alter table public.duet_command_steers enable row level security;

alter table public.duet_command_steers force row level security;

create policy account_isolation on public.duet_command_steers
    using (user_id = public.acting_account())
    with check (user_id = public.acting_account());
