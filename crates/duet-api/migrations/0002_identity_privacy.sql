-- Extend the existing identity; never derive identity from email or a practice link.
alter table public.users
 add column if not exists display_name text,
 add column if not exists setup_complete boolean not null default false,
 add column if not exists terms_version text,
 add column if not exists terms_accepted_at timestamptz,
 add column if not exists luna_consent_version text,
 add column if not exists luna_consent_decision text,
 add column if not exists luna_consent_at timestamptz,
 add column if not exists intelligence_enabled boolean not null default true,
 add column if not exists intelligence_control_revision bigint not null default 0,
 add column if not exists sessions_revoked_before timestamptz,
 add column if not exists today_stat_ids text[],
 add column if not exists evidence_revision bigint not null default 0,
 add column if not exists duet_incarnation uuid not null default gen_random_uuid();
-- Narrow pre-account resolver: no patient prose and no email lookup.
create or replace view public.duet_identity_directory as
 select id,auth_user_id from public.users;
revoke all on public.duet_identity_directory from public;
create table public.duet_front_door_mapping (
 front_door_account_id uuid primary key,
 user_id uuid not null unique references public.users(id) on delete cascade
);
alter table public.duet_front_door_mapping enable row level security;
create policy account_rows on public.duet_front_door_mapping
 using(user_id=public.acting_account()) with check(user_id=public.acting_account());
create table if not exists public.privacy_recovery_events (
 id uuid primary key default gen_random_uuid(), account_id uuid not null,
 operation text not null, scope_id uuid not null, scope_version text not null default '',
 occurred_at timestamptz not null default clock_timestamp(), journaled_at timestamptz
);
alter table public.privacy_recovery_events
 add column if not exists account_incarnation uuid,
 add column if not exists receipt_bytes bytea,
 add column if not exists attempts integer not null default 0,
 add column if not exists next_attempt_at timestamptz not null default now();
create index if not exists duet_privacy_pending on public.privacy_recovery_events(account_id,next_attempt_at) where journaled_at is null;
alter table public.privacy_recovery_events enable row level security;
create policy duet_privacy_guard on public.privacy_recovery_events as restrictive
 using(account_id=public.acting_account()) with check(account_id=public.acting_account());
create policy duet_privacy_access on public.privacy_recovery_events
 using(account_id=public.acting_account()) with check(account_id=public.acting_account());
-- During migration, historical receipts must be delivered by the original journal
-- owner before cutover. Never silently discard pending restrictions lacking the
-- new typed incarnation/byte envelope.
do $$ begin
 if exists(select 1 from public.privacy_recovery_events where journaled_at is null and receipt_bytes is null)
 then raise exception 'drain legacy privacy journal before durable owner cutover'; end if;
end $$;
-- Recovery sees only account identifiers, never receipt bytes or private state before scoping.
create view public.duet_privacy_pending_accounts as
 select distinct account_id from public.privacy_recovery_events
 where journaled_at is null and receipt_bytes is not null and next_attempt_at<=now();
revoke all on public.duet_privacy_pending_accounts from public;
create function public.duet_immutable_receipt() returns trigger language plpgsql as $$
begin
 if (new.id,new.account_id,new.operation,new.scope_id,new.scope_version,new.occurred_at,new.account_incarnation,new.receipt_bytes)
 is distinct from (old.id,old.account_id,old.operation,old.scope_id,old.scope_version,old.occurred_at,old.account_incarnation,old.receipt_bytes)
 then raise exception 'privacy receipt is immutable'; end if;
 return new;
end $$;
create trigger duet_immutable_receipt before update on public.privacy_recovery_events
 for each row execute function public.duet_immutable_receipt();
-- Replace legacy users-level privacy trigger only: this release commits typed
-- events explicitly in the action transaction. Other resource owners retain
-- their triggers until their route family migrates.
do $$ declare r record; begin
 for r in select t.tgname from pg_trigger t join pg_proc p on p.oid=t.tgfoid
 where t.tgrelid='public.users'::regclass and p.proname='record_privacy_restriction' loop
  execute format('drop trigger %I on public.users',r.tgname);
 end loop;
end $$;
