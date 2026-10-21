alter table public.users
 add column if not exists doctor_note_generation bigint not null default 0,
 add column if not exists doctor_page jsonb not null default '{}',
 add column if not exists doctor_proposal jsonb,
 add column if not exists doctor_page_updated_at timestamptz not null default now();
create table if not exists public.doctor_collection_items (
 id uuid primary key,user_id uuid not null references public.users(id) on delete cascade,
 kind text not null,source_id text not null,title text not null,detail text not null,
 evidence jsonb not null default '[]',range_start timestamptz,range_end timestamptz,
 saved_at timestamptz not null default now(),position integer not null,
 unique(user_id,kind,source_id)
);
alter table public.doctor_collection_items
 add column if not exists record_selection jsonb,
 add column if not exists fact_snapshot jsonb,
 add column if not exists fact_snapshot_id uuid,
 add column if not exists fact_timezone text not null default 'UTC',
 add column if not exists page_entry jsonb,
 add column if not exists medication_confirmed boolean not null default false;
create table public.duet_doctor_proposals (
 user_id uuid not null references public.users(id) on delete cascade,
 id uuid not null,run_id uuid not null,proposal jsonb not null,
 state text not null default 'pending' check(state in ('pending','approved','rejected','superseded')),
 primary key(user_id,id),unique(user_id,run_id)
);
create table public.duet_doctor_saves (
 user_id uuid not null references public.users(id) on delete cascade,
 expected_generation bigint not null,request jsonb not null,result jsonb not null,
 primary key(user_id,expected_generation)
);
create table public.duet_device_snapshots (
 user_id uuid not null references public.users(id) on delete cascade,
 id uuid not null,generated_at timestamptz not null,payload jsonb not null,received_at timestamptz not null default now(),
 primary key(user_id,id)
);
create table public.duet_publications (
 id uuid primary key,user_id uuid not null references public.users(id) on delete cascade,
 operation_id uuid not null,request jsonb not null,generation bigint not null,
 token_hash text not null unique,shl_hash text not null unique,
 protected_capabilities bytea not null,
 snapshot jsonb not null,pdf bytea not null,digest text not null,
 created_at timestamptz not null default now(),expires_at timestamptz not null,
 revoked_at timestamptz,unique(user_id,operation_id)
);
create unique index duet_one_active_publication on public.duet_publications(user_id) where revoked_at is null;
-- A narrow capability directory resolves only an account/id. Every payload fetch
-- then enters account scope and rechecks expiry/revocation/source currentness.
create view public.duet_publication_directory as select id,user_id,token_hash,shl_hash from public.duet_publications;
revoke all on public.duet_publication_directory from public;
-- Preserve original subjects after product deletion, preventing auto-reprovision by stale provider sessions.
create table public.duet_deleted_identities (
 auth_user_id text primary key,account_id uuid not null,incarnation uuid not null,deleted_at timestamptz not null
);
revoke all on public.duet_deleted_identities from public;
-- Restores MUST start quarantined. The application additionally requires a journal
-- inventory reconciliation before each new process admits private/public reads.
create table public.duet_restore_gate (singleton boolean primary key default true check(singleton),epoch uuid not null default gen_random_uuid());
insert into public.duet_restore_gate(singleton) values(true);
revoke all on public.duet_restore_gate from public;
create view public.duet_unsupported_receipts as select exists(select 1 from public.privacy_recovery_events where receipt_bytes is null) as present;
revoke all on public.duet_unsupported_receipts from public;
-- Replace only triggers owned by the prior workspace generation implementation.
do $$ declare r record; begin
 for r in select c.relname,t.tgname from pg_trigger t join pg_class c on c.oid=t.tgrelid join pg_proc p on p.oid=t.tgfoid
 where c.oid in ('public.users'::regclass,'public.doctor_collection_items'::regclass)
 and p.proname in ('privacy_note_generation','doctor_items_changed','record_privacy_restriction') loop
 execute format('drop trigger %I on public.%I',r.tgname,r.relname); end loop;
end $$;
do $$ declare t text; begin
 foreach t in array array['doctor_collection_items','duet_doctor_proposals','duet_doctor_saves','duet_device_snapshots','duet_publications'] loop
 execute format('alter table public.%I enable row level security',t);
 execute format('create policy duet_scope_guard on public.%I as restrictive using(user_id=public.acting_account()) with check(user_id=public.acting_account())',t);
 execute format('create policy duet_scope_access on public.%I using(user_id=public.acting_account()) with check(user_id=public.acting_account())',t);
 end loop;
end $$;
