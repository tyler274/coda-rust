-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- A citation is a unit identity plus version. This column recorded which parts
-- of a source the model had already seen; the per-run delivery set replaced it
-- and nothing reads or writes the column. Maintenance rather than online,
-- because the previous image still inserts the value on every citation.
alter table public.conversation_citations drop column read_receipts;

insert into public.schema_migrations(version)
values('0049_remove_citation_read_receipts');
commit;
