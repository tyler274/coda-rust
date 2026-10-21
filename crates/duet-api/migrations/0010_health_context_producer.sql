-- Canonical producer credentials are distinct from the retained-source reader.
-- The migrator grants only that role SELECT/INSERT/UPDATE on health_contexts and
-- SELECT/INSERT on this ledger. No SECURITY DEFINER or PUBLIC mutation grant.
alter table public.health_contexts
 add column producer_input jsonb,
 add column producer_dependencies jsonb not null default '[]',
 add column producer_limits jsonb not null default '[]',
 add column producer_fingerprint text;

-- Metadata only: retries cannot resurrect a superseded/withdrawn body.
create table public.duet_health_context_operations (
 user_id uuid not null references public.users(id) on delete cascade,
 operation_id uuid not null,
 request_sha256 text not null,
 snapshot_id uuid not null,
 revision bigint not null check(revision>0),
 primary key(user_id,operation_id)
);
alter table public.duet_health_context_operations enable row level security;
alter table public.duet_health_context_operations force row level security;
create policy health_context_operation_guard on public.duet_health_context_operations
 as restrictive using(user_id=public.acting_account()) with check(user_id=public.acting_account());
create policy health_context_operation_access on public.duet_health_context_operations
 using(user_id=public.acting_account()) with check(user_id=public.acting_account());
revoke all on public.duet_health_context_operations from public;
