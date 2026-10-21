-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Execution metadata only. Canonical mutations already commit their durable
-- health_discovery_pending marker. One account shares one bounded delivery;
-- queue payloads never contain records, prompts, or search text.
create table public.health_discovery_work (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null unique references public.users(id) on delete cascade,
    claim_token uuid,
    claim_until timestamptz,
    delivery_generation bigint not null default 0,
    delivery_until timestamptz,
    delivery_confirmed boolean not null default false
);

-- Lock the pending generation before invalidating its projection. Publication
-- takes the same marker lock. This also protects canonical writers that do not
-- hold an account mutation lock: a delete that ran before a concurrent insert
-- must not leave that inserted old generation searchable after the edit commits.
create or replace function public.invalidate_health_discovery(account uuid,kind text,source text)
returns void language plpgsql set search_path=pg_catalog,public as $$
begin
    if not exists(select 1 from public.users where id=account) then return; end if;
    insert into public.health_discovery_pending(user_id,source_kind,source_id)
        values(account,kind,source)
    on conflict(user_id,source_kind,source_id) do update
        set generation=gen_random_uuid(),changed_at=clock_timestamp();
    delete from public.health_discovery_documents
        where user_id=account and source_kind=kind and source_id=source;
end $$;

insert into public.schema_migrations(version) values ('0053_discovery_queued_delivery');

commit;
