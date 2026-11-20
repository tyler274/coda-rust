-- Standalone email authentication challenges, non-expiring device sessions, and UI appearance.
alter table public.users
    add column if not exists email text,
    add column if not exists appearance text not null default 'system'
        check (appearance in ('system', 'light', 'dark'));

create unique index if not exists duet_users_email_unique
    on public.users (lower(email))
    where email is not null;

-- Narrow directory view for email sign-in / email-change uniqueness checks before RLS account scope.
create or replace view public.duet_email_directory as
select
    id,
    auth_user_id,
    email,
    appearance
from
    public.users;

revoke all on public.duet_email_directory from public;

create table if not exists public.duet_email_challenges (
    id uuid primary key default gen_random_uuid (),
    purpose text not null check (purpose in ('sign_in', 'email_change')),
    email text not null check (char_length(btrim(email)) between 3 and 320),
    user_id uuid references public.users (id) on delete cascade,
    link_token_hash bytea not null unique,
    short_code_hash bytea not null,
    expires_at timestamptz not null,
    consumed_at timestamptz,
    attempts integer not null default 0,
    created_at timestamptz not null default now()
);

create index if not exists duet_email_challenges_lookup
    on public.duet_email_challenges (lower(email), purpose, created_at desc);

revoke all on public.duet_email_challenges from public;

create table if not exists public.duet_sessions (
    id uuid primary key default gen_random_uuid (),
    user_id uuid not null references public.users (id) on delete cascade,
    token_hash bytea not null unique,
    created_at timestamptz not null default now(),
    revoked_at timestamptz
);

create index if not exists duet_sessions_user
    on public.duet_sessions (user_id, created_at desc);

revoke all on public.duet_sessions from public;
