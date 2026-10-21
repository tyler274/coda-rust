-- Raw independent legacy receipt bytes retained, never rewritten under the same journal key.
create table public.duet_legacy_privacy_replays (
    event_id uuid primary key,
    account_id uuid not null,
    receipt_bytes bytea not null,
    replayed_at timestamptz not null default now()
);

alter table public.duet_legacy_privacy_replays enable row level security;

create policy account_scope on public.duet_legacy_privacy_replays
    using (account_id = public.acting_account ())
    with check (account_id = public.acting_account ());

create or replace view public.duet_unsupported_receipts as
select
    exists (
        select
            1
        from
            public.privacy_recovery_events p
        where
            p.receipt_bytes is null
            and not exists (
                select
                    1
                from
                    public.duet_legacy_privacy_replays l
                where
                    l.event_id = p.id
                    and l.account_id = p.account_id)) as present;
