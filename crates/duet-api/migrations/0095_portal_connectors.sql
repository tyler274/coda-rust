-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Oracle Health and healow patient portals join Epic MyChart on the one
-- connector owner. Each vendor keeps its own provider and evidence source kind,
-- so a record always names where it came from; Epic's values are unchanged.
--
-- Online: the widened checks are added NOT VALID (validated by a later
-- migration) and admit every value the serving release writes. The directory
-- moves to a new table keyed by vendor and organization (0096 seeds it from the
-- Epic mirror); the serving release keeps reading epic_organizations, which a
-- later release drops.
alter table public.source_connections
    drop constraint source_connections_provider_check,
    add constraint source_connections_provider_check
    check (provider = any (array[
        'googleHealth'::text,
        'epic'::text,
        'oracleHealth'::text,
        'healow'::text
    ])) not valid;

alter table public.health_evidence
    drop constraint health_evidence_source_kind_check,
    add constraint health_evidence_source_kind_check
    check (source_kind = any (array[
        'appleHealth'::text,
        'googleHealth'::text,
        'epic'::text,
        'oracleHealth'::text,
        'healow'::text,
        'healthRecords'::text,
        'manual'::text
    ])) not valid;

create table public.portal_organizations (
    provider text not null,
    id text not null,
    name text not null,
    location text,
    fhir_base_url text not null,
    search_terms text[] default '{}'::text[] not null,
    active boolean default true not null,
    source_updated_at timestamp with time zone not null,
    updated_at timestamp with time zone default now() not null,
    primary key (provider, id),
    constraint portal_organizations_provider_check
        check (provider = any (array['epic'::text, 'oracleHealth'::text, 'healow'::text])),
    constraint portal_organizations_https_check
        check (fhir_base_url ~ '^https://')
);

create index portal_organizations_name_idx
    on public.portal_organizations (lower(name));

-- The directory refresh role writes only the public directory.
-- scripts/gcp/split_database_roles.py keeps the same rule on a rerun.
do $$
begin
    if exists (select 1 from pg_roles where rolname = 'duet_directory') then
        grant select, insert, update on public.portal_organizations to duet_directory;
    end if;
end $$;

insert into public.schema_migrations(version) values('0095_portal_connectors');
commit;
