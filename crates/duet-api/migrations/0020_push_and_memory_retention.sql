-- Generic multi-device push tokens/deliveries, default-off AI intelligence consent,
-- separate opt-in research consent, 30-day rolling private memory version retention,
-- and Duet-owned harness run records (`intent/duet.md`, `intent/harness.md`).

-- Default AI intelligence off until the person explicitly grants AI consent.
alter table public.users
    alter column intelligence_enabled set default false;

update public.users
set intelligence_enabled = false
where intelligence_enabled = true
  and coalesce(luna_consent_decision, '') <> 'accepted';

create table public.duet_push_devices (
    user_id uuid not null references public.users (id) on delete cascade,
    device_id text not null,
    session_authenticated_at timestamptz not null,
    platform text not null check (platform in ('apns', 'fcm')),
    push_token text not null,
    registered_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    revoked_at timestamptz,
    primary key (user_id, device_id)
);

create index duet_push_devices_active on public.duet_push_devices (user_id)
where
    revoked_at is null;

-- Lock-screen notifications are strictly generic categories; no clinical text is stored here.
create table public.duet_push_deliveries (
    id uuid not null,
    user_id uuid not null references public.users (id) on delete cascade,
    device_id text not null,
    alert_kind text not null check (
        alert_kind in (
            'doctor_message',
            'new_record',
            'appointment_reminder',
            'account_notice'
        )
    ),
    status text not null check (status in ('pending', 'sent', 'cancelled')),
    created_at timestamptz not null default now(),
    dispatched_at timestamptz,
    primary key (user_id, id),
    foreign key (user_id, device_id) references public.duet_push_devices (user_id, device_id) on delete cascade
);

create index duet_push_deliveries_pending on public.duet_push_deliveries (user_id, device_id)
where
    status = 'pending';

-- Past versions of Duet's private memory are retained for 30 days, then pruned,
-- and deleted immediately with the account.
create table public.duet_private_memory_versions (
    user_id uuid not null references public.users (id) on delete cascade,
    revision bigint not null,
    content jsonb not null,
    recorded_at timestamptz not null default now(),
    expires_at timestamptz not null,
    primary key (user_id, revision)
);

create index duet_private_memory_versions_expires on public.duet_private_memory_versions (expires_at);

-- Separate opt-in consent for product improvement/research and standing AI disclosure receipt.
create table public.duet_ai_preferences (
    user_id uuid primary key references public.users (id) on delete cascade,
    research_consent_state text not null default 'off' check (research_consent_state in ('off', 'granted', 'withdrawn')),
    research_consent_at timestamptz,
    research_consent_revision bigint not null default 0,
    standing_disclosure_acknowledged_at timestamptz,
    updated_at timestamptz not null default now()
);

-- Duet-owned durable run record for harness-executed assistant work.
create table public.duet_harness_runs (
    user_id uuid not null references public.users (id) on delete cascade,
    run_id text not null,
    continues_run_id text,
    recipe_name text not null,
    model text not null,
    prompt_cache_key text not null,
    outcome text not null check (outcome in ('running', 'completed', 'held', 'failed', 'cancelled')),
    hold_reason text check (hold_reason in ('stopped', 'limit', 'provider_blocked')),
    events jsonb not null default '[]'::jsonb,
    input_tokens bigint,
    output_tokens bigint,
    started_at timestamptz not null default now(),
    ended_at timestamptz,
    primary key (user_id, run_id)
);

do $$
declare
    t text;
begin
    foreach t in array array[
        'duet_push_devices',
        'duet_push_deliveries',
        'duet_private_memory_versions',
        'duet_ai_preferences',
        'duet_harness_runs'
    ] loop
        execute format('alter table public.%I enable row level security', t);
        execute format('alter table public.%I force row level security', t);
        execute format(
            'create policy account_guard on public.%I as restrictive using (user_id = public.acting_account()) with check (user_id = public.acting_account())',
            t
        );
        execute format(
            'create policy account_isolation on public.%I using (user_id = public.acting_account()) with check (user_id = public.acting_account())',
            t
        );
    end loop;
end
$$;

-- Content-free operational directory for 30-day memory version pruning: carries account IDs only.
create view public.duet_memory_retention_due as
select distinct
    user_id
from
    public.duet_private_memory_versions
where
    expires_at <= now();

revoke all on public.duet_memory_retention_due from public;
