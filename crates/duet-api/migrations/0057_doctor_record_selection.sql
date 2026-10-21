-- duet-release: online
begin;

-- Saved health information keeps its existing collection kind and owner.
-- Reuse the exact addressed-read receipt; canonical history owns the body.
alter table public.doctor_collection_items
    add column record_selection jsonb,
    add constraint doctor_collection_record_selection_check check (
        record_selection is null or coalesce((
            kind = 'fact'
            and jsonb_typeof(record_selection) = 'object'
            and record_selection ?& array['entity','record_id','canonical_record_id','revision']
            and record_selection->>'entity' in ('health_evidence','clinical_note')
            and record_selection->>'canonical_record_id' = source_id
            and (record_selection->>'record_id')::uuid is not null
            and (record_selection->>'revision')::bigint > 0
        ), false)
    );

insert into public.schema_migrations(version) values ('0057_doctor_record_selection');
commit;
