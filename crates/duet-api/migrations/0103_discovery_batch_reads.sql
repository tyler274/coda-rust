begin;

-- Planned maintenance: the serving release offers the single-record and
-- single-fact tools these references name.

-- Luna now reads whole records and measurement summaries several at a time.
-- Search results name the read that opens them, so the stored references move
-- to the batch tools; the index stays a disposable projection of its owners.
update public.health_discovery_documents
   set reference = jsonb_build_object(
         'tool', 'read_health_records',
         'arguments', jsonb_build_object(
           'revisions', jsonb_build_array(reference->'arguments'->'revision')))
 where reference->>'tool' = 'read_health_record';

update public.health_discovery_documents
   set reference = jsonb_build_object(
         'tool', 'read_health_facts',
         'arguments', jsonb_build_object(
           'fact_ids', jsonb_build_array(reference->'arguments'->'fact_id')))
 where reference->>'tool' = 'read_health_fact';

-- Phone summaries are indexed as the words Luna reads; that wording changed,
-- so their owner rebuilds them.
select public.invalidate_health_discovery(user_id, 'native_fact', user_id::text)
  from public.health_contexts;

insert into public.schema_migrations(version) values ('0103_discovery_batch_reads');
commit;
