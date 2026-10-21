-- Capture only account incarnations present during this upgrade. The original
-- receipt's account UUID is an incarnation, not an external login identity.
-- Runtime cannot enroll later accounts or rebind a receipt to a recreated row.
create table public.duet_legacy_privacy_accounts (
    account_id uuid primary key,
    incarnation uuid not null,
    admitted_at timestamptz not null default now()
);

insert into public.duet_legacy_privacy_accounts (account_id, incarnation)
select
    id,
    duet_incarnation
from
    public.users;

alter table public.duet_legacy_privacy_accounts enable row level security;

create policy account_scope on public.duet_legacy_privacy_accounts
    using (account_id = public.acting_account ());
