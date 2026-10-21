-- duet-release: online
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Constant default, no rewrite; old workers ignore the new counters. A run
-- keeps its allowance across input restarts and queue deliveries. These are
-- operational counts, never prompts, tool arguments or health content.
alter table public.conversation_runs
    add column execution_usage jsonb not null default '{}'::jsonb;

-- Deletion revokes processing authority, not just freshness. Reuse the
-- canonical evidence mutation owner and existing permission fence so a live
-- snapshot cannot outlive deletion, even if that record is reimported later.
-- Function replacement only; ordinary inserts keep their existing behavior.
create or replace function public.advance_evidence_revision() returns trigger
language plpgsql set search_path = pg_catalog, public as $$
declare entity_name text := case TG_TABLE_NAME
    when 'clinical_notes' then 'clinical_note' else 'health_evidence' end;
begin
    perform id from public.users
    where id in (select user_id from changed_evidence) order by id for update;
    update public.users set evidence_revision = evidence_revision + 1,
        intelligence_control_revision = intelligence_control_revision + (TG_OP='DELETE')::int
    where id in (select user_id from changed_evidence);
    insert into public.evidence_sync_changes(user_id, entity, record_id, client_record_id, revision, deleted)
    select e.user_id, entity_name, e.id, coalesce(to_jsonb(e)#>>'{payload,id}', e.id::text), u.evidence_revision, TG_OP='DELETE'
    from changed_evidence e join public.users u on u.id=e.user_id
    on conflict(user_id, entity, record_id) do update
    set revision=excluded.revision, deleted=excluded.deleted, client_record_id=excluded.client_record_id;
    return null;
end;
$$;

insert into public.schema_migrations(version) values('0092_conversation_execution_budget');
commit;
