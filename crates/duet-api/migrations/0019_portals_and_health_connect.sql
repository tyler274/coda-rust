-- Epic/MyChart & Oracle Health portal connections, setup "Not now" persistence,
-- and Android Health Connect generation/disconnect tracking. Additive in place.

alter table public.users
    add column if not exists health_connect_generation integer not null default 0,
    add column if not exists health_connect_disconnected_at timestamptz,
    add column if not exists setup_connectors_status text not null default 'pending',
    add column if not exists setup_connectors_updated_at timestamptz;

alter table public.users
    drop constraint if exists duet_setup_connectors_status_check;

alter table public.users
    add constraint duet_setup_connectors_status_check
    check (setup_connectors_status in ('pending', 'skipped_not_now', 'completed'));

create table if not exists public.duet_portal_connections (
    id uuid primary key default gen_random_uuid (),
    user_id uuid not null references public.users (id) on delete cascade,
    organization_id text not null,
    organization_name text not null,
    vendor text not null check (vendor in ('epic', 'oracle')),
    brand text not null,
    fhir_base_url text not null,
    state text not null check (state in ('connected', 'partial', 'broken', 'disconnected')),
    granted_scopes text[] not null default array[]::text[],
    access_token text,
    refresh_token text,
    token_expires_at timestamptz,
    last_synced_at timestamptz,
    failing_since timestamptz,
    failure_reason text,
    requires_reconnect boolean not null default false,
    partial_failed_count integer not null default 0,
    next_refresh_at timestamptz,
    last_attempt_at timestamptz,
    disconnected_at timestamptz,
    evidence_revision bigint not null default 0,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    unique (user_id, organization_id)
);

create index if not exists duet_portal_connections_due
    on public.duet_portal_connections (next_refresh_at)
    where state <> 'disconnected' and next_refresh_at is not null;

alter table public.duet_portal_connections enable row level security;

drop policy if exists duet_portal_connections_guard on public.duet_portal_connections;
create policy duet_portal_connections_guard on public.duet_portal_connections as restrictive
    using (user_id = public.acting_account ())
    with check (user_id = public.acting_account ());

drop policy if exists duet_portal_connections_access on public.duet_portal_connections;
create policy duet_portal_connections_access on public.duet_portal_connections
    using (user_id = public.acting_account ())
    with check (user_id = public.acting_account ());

create table if not exists public.duet_portal_oauth_states (
    state text primary key,
    user_id uuid not null references public.users (id) on delete cascade,
    organization_id text not null,
    vendor text not null check (vendor in ('epic', 'oracle')),
    code_verifier text not null,
    redirect_uri text not null,
    created_at timestamptz not null default now(),
    expires_at timestamptz not null,
    consumed_at timestamptz
);

create index if not exists duet_portal_oauth_states_user
    on public.duet_portal_oauth_states (user_id, expires_at);

alter table public.duet_portal_oauth_states enable row level security;

drop policy if exists duet_portal_oauth_states_guard on public.duet_portal_oauth_states;
create policy duet_portal_oauth_states_guard on public.duet_portal_oauth_states as restrictive
    using (user_id = public.acting_account ())
    with check (user_id = public.acting_account ());

drop policy if exists duet_portal_oauth_states_access on public.duet_portal_oauth_states;
create policy duet_portal_oauth_states_access on public.duet_portal_oauth_states
    using (user_id = public.acting_account ())
    with check (user_id = public.acting_account ());

-- Content-free operational directory for daily portal refresh and overdue
-- engineering diagnostics: exposes identifiers, vendor, state, and timestamps
-- only — never OAuth tokens, patient names, or clinical records.
create or replace view public.duet_portal_refresh_queue as
select
    id,
    user_id,
    vendor,
    organization_id,
    state,
    next_refresh_at,
    last_synced_at,
    failing_since,
    failure_reason,
    partial_failed_count
from
    public.duet_portal_connections
where
    state <> 'disconnected';

revoke all on public.duet_portal_refresh_queue from public;
