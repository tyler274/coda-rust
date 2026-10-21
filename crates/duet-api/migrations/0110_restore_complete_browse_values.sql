-- Rebuild affected disposable projections from their intact canonical values.
-- The prior builder retained only the first 400 characters of text results.
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

update public.evidence_browse_generations g
set generation = g.generation + 1,
    minimum_readable_generation = g.generation + 1
where exists (
    select 1 from public.health_evidence e
    where e.user_id = g.user_id
      and public.is_evidence_browse_input(e.concept_id, e.payload)
      and e.payload#>>'{value,type}' = 'text'
      and length(e.payload#>>'{value,text}') > 400
);

-- Recalculate cached server facts through their existing owner even when no
-- source has changed. The former arithmetic gates discarded defined values.
update public.health_contexts
set source_generations = source_generations - 'canonical_evidence_revision'
where source_generations ? 'canonical_evidence_revision';

insert into public.schema_migrations (version)
values ('0110_restore_complete_browse_values')
on conflict do nothing;

commit;
