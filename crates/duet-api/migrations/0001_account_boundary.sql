-- Workspace baseline, deliberately NOT a replay of the reference migration chain.
-- Existing public.users identities/columns, evidence and doctor workspace rows
-- remain untouched. Later owner migrations extend these same tables in place.
-- In particular, never replay 0119_fixed_doctor_page's destructive beta reset.
create table if not exists public.users (
    id uuid primary key default gen_random_uuid (),
    auth_user_id text not null unique,
    apple_health_generation integer not null default 0,
    apple_health_disconnected_at timestamptz,
    health_records_generation integer not null default 0,
    health_records_disconnected_at timestamptz,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    setup_complete boolean not null default false,
    sharing_label text not null check (char_length(btrim(sharing_label)) between 1 and 40 and sharing_label = btrim(sharing_label)),
    display_name text check (display_name is null or (char_length(btrim(display_name)) between 1 and 80 and display_name = btrim(display_name))),
    doctor_note text check (doctor_note is null or char_length(doctor_note) <= 1000)
);

-- Refuse incompatible pre-existing identity owners rather than inventing IDs,
-- merging by email, or treating a partial import as a fresh installation.
do $$
begin
    if not exists (
        select
            1
        from
            pg_constraint
        where
            conrelid = 'public.users'::regclass
            and contype = 'p'
            and pg_get_constraintdef(oid) = 'PRIMARY KEY (id)')
        or not exists (
            select
                1
            from
                pg_constraint
            where
                conrelid = 'public.users'::regclass
                and contype = 'u'
                and pg_get_constraintdef(oid) = 'UNIQUE (auth_user_id)') then
        raise exception 'existing Duet identity schema is incompatible';
end if;
end
$$;

create or replace function public.acting_account ()
    returns uuid
    language sql
    stable parallel safe
    as $$
    select
        nullif (current_setting('app.account_id', true), '')::uuid
$$;

revoke all on function public.acting_account () from public;

alter table public.users enable row level security;

-- Keep the reference patient_rows policy, if present. This restrictive guard
-- also intersects any older permissive policy and fails closed with no scope.
drop policy if exists duet_account_guard on public.users;

create policy duet_account_guard on public.users as restrictive
    using (id = public.acting_account ())
    with check (id = public.acting_account ());

drop policy if exists duet_account_access on public.users;

create policy duet_account_access on public.users
    using (id = public.acting_account ())
    with check (id = public.acting_account ());
