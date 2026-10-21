-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Genome is removed from Duet with all of its data (Dev, October 2, 2026).
-- Each person's imports and findings go, along with the shared reference
-- tables and lookups they were matched against. Dropping genome_imports also
-- drops its row policy and its privacy-journal trigger; archived
-- genome_withdraw receipts remain history that recovery accepts and replays
-- nothing for. The raw files on the private mount are deleted by the release
-- runbook, not from SQL.
--
-- Maintenance: the serving release still reads and writes these tables.
drop view public.genome_imports_bookkeeping;
drop table public.genome_findings;
drop table public.genome_imports;
drop function public.lookup_variants(text[]);
drop function public.lookup_gwas(text[]);
drop table public.clinvar;
drop table public.gwas;
drop table public.frequencies;

insert into public.schema_migrations(version) values('0106_remove_genome');
commit;
