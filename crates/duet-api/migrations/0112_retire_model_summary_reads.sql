-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Measurements are calculated from canonical evidence when read. Remove only
-- the disposable search path to the retired summary tools. Published citations,
-- historical understanding provenance and the phone's projections stay intact.
drop trigger health_discovery_native on public.health_contexts;
delete from public.health_discovery_documents where source_kind='native_fact';
delete from public.health_discovery_pending where source_kind='native_fact';

insert into public.schema_migrations(version) values ('0112_retire_model_summary_reads');
commit;
