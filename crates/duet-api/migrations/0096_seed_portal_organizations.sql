-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Seed the vendor-keyed directory from the Epic mirror. Connection instance
-- keys are directory IDs, so existing Epic connections resolve to the same
-- organization rows. Online: an insert into a table 0095 created, read from a
-- table nothing here alters.
insert into public.portal_organizations
    (provider, id, name, fhir_base_url, search_terms, active, source_updated_at, updated_at)
select 'epic', id, name, fhir_base_url, search_terms, active, source_updated_at, updated_at
from public.epic_organizations;

insert into public.schema_migrations(version) values('0096_seed_portal_organizations');
commit;
