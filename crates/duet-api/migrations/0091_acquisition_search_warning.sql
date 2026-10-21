-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- A provider may attach a warning to a search. The query keeps the returned
-- records and completes as limited, never complete. Only the fixed severity
-- is stored; provider diagnostics and codes never are.
--
-- Online: a nullable column with no default is metadata-only, and the check
-- is NOT VALID because every existing row is null. The serving
-- runtime never writes it and fails warned searches, so it cannot complete a
-- unit that later reads as warned.
alter table public.source_acquisition_units
    add column search_outcome text,
    add constraint source_acquisition_units_search_outcome_check
        check (search_outcome is null or search_outcome = 'warning') not valid;

insert into public.schema_migrations(version) values('0091_acquisition_search_warning');
commit;
