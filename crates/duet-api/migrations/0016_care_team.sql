-- Duet-owned directory snapshots and review templates, not physician accounts.
create table public.duet_care_team (
    id uuid primary key,
    user_id uuid not null references public.users(id) on delete cascade,
    npi text not null check (npi ~ '^[0-9]{10}$'),
    snapshot jsonb not null,
    role text not null default '',
    nickname text not null default '',
    note text not null default '',
    added_at timestamptz not null default now(),
    refreshed_at timestamptz not null default now(),
    unique (user_id, npi),
    unique (user_id, id)
);
create table public.duet_release_authorizations (
    id uuid primary key,
    user_id uuid not null references public.users(id) on delete cascade,
    discloser_member_id uuid not null,
    patient jsonb not null,
    discloser jsonb not null,
    recipient jsonb not null,
    scope jsonb not null,
    purpose text not null default 'At the request of the individual',
    expires_on date,
    expiration_event text,
    signed_name text,
    signed_at timestamptz,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    check (expires_on is null or expiration_event is null),
    check ((signed_name is null) = (signed_at is null)),
    foreign key (user_id, discloser_member_id) references public.duet_care_team(user_id, id) on delete cascade
);
create index duet_release_authorizations_account on public.duet_release_authorizations(user_id);
alter table public.duet_care_team enable row level security;
alter table public.duet_care_team force row level security;
create policy duet_care_team_account on public.duet_care_team
    using (user_id = public.acting_account()) with check (user_id = public.acting_account());
alter table public.duet_release_authorizations enable row level security;
alter table public.duet_release_authorizations force row level security;
create policy duet_release_authorizations_account on public.duet_release_authorizations
    using (user_id = public.acting_account()) with check (user_id = public.acting_account());
revoke all on public.duet_care_team, public.duet_release_authorizations from public;
