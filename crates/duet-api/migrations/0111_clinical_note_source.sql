-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- An Apple clinical note keeps the date fields its source declared, as every
-- other clinical record keeps them under `source`: verbatim (`declared_dates`)
-- with the one its date was projected from (`projected_date_role`). A
-- note whose source declared only a day then shows on that day everywhere,
-- not on the day its stored midnight-UTC instant falls on. Notes saved before
-- this keep a null source until the phone imports them again.
alter table public.clinical_notes add column source jsonb;
alter table public.clinical_notes
    add constraint clinical_notes_source_object
    check (source is null or jsonb_typeof(source) = 'object') not valid;

-- Every list built before this release lacks each row's `day`, yet reads as
-- current, so a date-only record keeps showing on the viewer's day. Rebuild
-- every account's list through the existing browse generation; reads say
-- refreshing until the recovery sweep has. Maintenance: a serving release
-- would rebuild the lists again without the day.
update public.evidence_browse_generations
set generation = generation + 1,
    minimum_readable_generation = generation + 1;

insert into public.schema_migrations(version) values ('0111_clinical_note_source');
commit;
