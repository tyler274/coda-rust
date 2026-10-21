begin;

-- The slot hash owns cycle idempotency. Evidence, context, and control revisions
-- own the generation fence. These legacy fields have no remaining reader or
-- decision role after migration 0018 established those canonical owners.
alter table public.insight_cycles
    drop column candidate_fingerprint,
    drop column included_concept_ids;

-- Authorization revision already owns atomic source withdrawal. Preserve that
-- owner while also deleting a quiet cycle whose source dependency exists only
-- on its operating index, before deleting the index or canonical receipts.
create or replace function public.invalidate_acquisition_authorization()
returns trigger
language plpgsql set search_path=pg_catalog,public as $$
begin
    update public.source_connections
    set acquisition_claim_unit=null,
        acquisition_claim_version=null,
        acquisition_claim_until=null
    where id=new.id;

    delete from public.source_acquisition_staging where unit_id in (
        select id from public.source_acquisition_units where connection_id=new.id
    );
    delete from public.source_acquisition_pages where unit_id in (
        select id from public.source_acquisition_units where connection_id=new.id
    );
    update public.source_acquisition_units
    set state='unavailable',
        claim_until=null,
        continuation_ciphertext=null,
        pages=0,
        terminal=false,
        last_completed_at=null,
        published_count=0,
        retry_count=0,
        continuation_restarts=0,
        not_before=now(),
        last_failure_code='authorization_changed',
        updated_at=now()
    where connection_id=new.id;

    delete from public.health_contexts where user_id=new.user_id;
    delete from public.insight_cycles
    where user_id=new.user_id
      and id in (
          select cycle_id from public.operating_indexes
          where user_id=new.user_id
            and (
                source_connection_ids @> array[new.id::text]
                or (new.provider='googleHealth'
                    and source_kinds && array['googleHealth', 'multiple']::text[])
            )
      );
    delete from public.doctor_collection_items
    where user_id=new.user_id
      and (
          source_connection_ids @> array[new.id::text]
          or (new.provider='googleHealth'
              and source_kinds && array['googleHealth', 'multiple']::text[])
      );
    delete from public.insight_cycles
    where user_id=new.user_id
      and (
          source_connection_ids @> array[new.id::text]
          or (new.provider='googleHealth'
              and source_kinds && array['googleHealth', 'multiple']::text[])
      );
    delete from public.operating_indexes
    where user_id=new.user_id
      and (
          source_connection_ids @> array[new.id::text]
          or (new.provider='googleHealth'
              and source_kinds && array['googleHealth', 'multiple']::text[])
      );

    delete from public.health_evidence
    where user_id=new.user_id
      and source_kind=new.provider
      and source_connection_id=new.id::text;
    return null;
end;
$$;

revoke all on function public.invalidate_acquisition_authorization() from public;

insert into public.schema_migrations(version)
values ('0021_subtractive_lifecycle_cleanup');

commit;
