begin;

alter table public.source_connections add column authorization_revision bigint not null default 0;
alter table public.source_connections add column acquisition_claim_unit uuid;
alter table public.source_connections add column acquisition_claim_version bigint;
alter table public.source_connections add column acquisition_claim_until timestamptz;
alter table public.source_connections add column last_refresh_day date;
alter table public.source_connections drop constraint source_connections_failure_kind_check;
alter table public.source_connections add constraint source_connections_failure_kind_check
    check (last_failure_kind is null or last_failure_kind in (
        'temporary','access_denied','reauthorization_required','account_not_linked','provider_contract_failure'
    ));

-- Query membership describes acquisition coverage, never clinical identity.
create table public.source_acquisition_units (
    id uuid primary key default gen_random_uuid(),
    connection_id uuid not null references public.source_connections(id) on delete cascade,
    query_key text not null,
    resource_type text not null,
    query_parameters jsonb not null,
    policy_version text not null,
    authorization_revision bigint not null,
    state text not null check (state in ('pending','running','completed','failed','unavailable')),
    attempt_id uuid not null unique,
    claim_version bigint not null default 0,
    claim_until timestamptz,
    continuation_ciphertext bytea,
    pages integer not null default 0 check (pages >= 0),
    terminal boolean not null default false,
    last_completed_at timestamptz,
    last_failure_code text,
    retry_count integer not null default 0 check (retry_count>=0),
    continuation_restarts integer not null default 0 check (continuation_restarts between 0 and 1),
    not_before timestamptz not null default now(),
    expires_at timestamptz not null default now()+interval '24 hours',
    published_count integer not null default 0 check (published_count>=0),
    updated_at timestamptz not null default now(),
    unique (connection_id, query_key)
);

create table public.source_acquisition_staging (
    unit_id uuid not null references public.source_acquisition_units(id) on delete cascade,
    evidence_id uuid not null,
    payload jsonb not null,
    primary key (unit_id, evidence_id)
);

create table public.source_query_memberships (
    unit_id uuid not null references public.source_acquisition_units(id) on delete cascade,
    evidence_id uuid not null references public.health_evidence(id) on delete cascade,
    observed_at timestamptz not null,
    primary key (unit_id, evidence_id)
);
create index source_query_memberships_evidence_idx on public.source_query_memberships(evidence_id);

-- Keyed request digests detect cycles across worker restarts. No patient URL
-- or bearer token is stored in the operational receipt; rows expire with staging.
create table public.source_acquisition_pages (
    unit_id uuid not null references public.source_acquisition_units(id) on delete cascade,
    request_digest text not null,
    primary key(unit_id,request_digest)
);

create function public.invalidate_acquisition_authorization() returns trigger
language plpgsql set search_path=pg_catalog,public as $$
begin
    update public.source_connections set acquisition_claim_unit=null,
        acquisition_claim_version=null,acquisition_claim_until=null where id=new.id;
    delete from public.source_acquisition_staging where unit_id in (
        select id from public.source_acquisition_units where connection_id=new.id
    );
    delete from public.source_acquisition_pages where unit_id in (
        select id from public.source_acquisition_units where connection_id=new.id
    );
    update public.source_acquisition_units set state='unavailable',claim_until=null,
        continuation_ciphertext=null,last_failure_code='authorization_changed',updated_at=now()
    where connection_id=new.id;
    return null;
end;
$$;
revoke all on function public.invalidate_acquisition_authorization() from public;
create trigger source_authorization_acquisition_invalidation
after update of authorization_revision on public.source_connections
for each row when (new.authorization_revision is distinct from old.authorization_revision)
execute function public.invalidate_acquisition_authorization();

insert into public.schema_migrations(version) values ('0013_source_acquisition_units');
commit;
