-- duet-release: online
begin;

-- Fence old writers before taking the baseline and replacing their revision
-- triggers. Without this, a write between the backfill and trigger replacement
-- can be absent from every later delta. Reads remain available. NOWAIT avoids
-- holding one table lock while waiting on an active cross-table writer; a busy
-- installation fails the migration transaction without partially installing it.
lock table public.health_evidence, public.clinical_notes
    in share row exclusive mode nowait;

-- Compact change membership, never another copy of clinical content. A user's
-- locked revision row orders commits; sequence allocation alone cannot do so.
create table public.evidence_sync_changes (
    user_id uuid not null references public.users(id) on delete cascade,
    entity text not null check(entity in ('clinical_note', 'health_evidence')),
    record_id uuid not null,
    client_record_id text not null,
    revision bigint not null,
    deleted boolean not null,
    primary key(user_id, entity, record_id)
);
create index evidence_sync_changes_revision_idx
    on public.evidence_sync_changes(user_id, revision, entity, record_id);

insert into public.evidence_sync_changes
select e.user_id, 'health_evidence', e.id, coalesce(e.payload->>'id', e.id::text), u.evidence_revision, false
from public.health_evidence e join public.users u on u.id=e.user_id
union all
select n.user_id, 'clinical_note', n.id, n.id::text, u.evidence_revision, false
from public.clinical_notes n join public.users u on u.id=n.user_id;

create or replace function public.advance_evidence_revision() returns trigger
language plpgsql set search_path = pg_catalog, public as $$
declare entity_name text := case TG_TABLE_NAME
    when 'clinical_notes' then 'clinical_note' else 'health_evidence' end;
begin
    -- Stable lock order also permits statements spanning several accounts.
    perform id from public.users
    where id in (select user_id from changed_evidence) order by id for update;
    update public.users set evidence_revision = evidence_revision + 1
    where id in (select user_id from changed_evidence);
    insert into public.evidence_sync_changes(user_id, entity, record_id, client_record_id, revision, deleted)
    select e.user_id, entity_name, e.id, coalesce(to_jsonb(e)#>>'{payload,id}', e.id::text), u.evidence_revision, TG_OP='DELETE'
    from changed_evidence e join public.users u on u.id=e.user_id
    on conflict(user_id, entity, record_id) do update
    set revision=excluded.revision, deleted=excluded.deleted, client_record_id=excluded.client_record_id;
    return null;
end;
$$;

create or replace function public.advance_updated_evidence_revision() returns trigger
language plpgsql set search_path = pg_catalog, public as $$
declare entity_name text := case TG_TABLE_NAME
    when 'clinical_notes' then 'clinical_note' else 'health_evidence' end;
begin
    perform id from public.users where id in (
        select user_id from old_evidence union select user_id from changed_evidence
    ) order by id for update;
    update public.users set evidence_revision = evidence_revision + 1 where id in (
        select user_id from old_evidence union select user_id from changed_evidence
    );
    insert into public.evidence_sync_changes(user_id, entity, record_id, client_record_id, revision, deleted)
    select e.user_id, entity_name, e.id, coalesce(to_jsonb(e)#>>'{payload,id}', e.id::text), u.evidence_revision, true
    from old_evidence e join public.users u on u.id=e.user_id
    where not exists(select 1 from changed_evidence n where n.user_id=e.user_id and n.id=e.id)
    on conflict(user_id, entity, record_id) do update
    set revision=excluded.revision, deleted=excluded.deleted, client_record_id=excluded.client_record_id;
    insert into public.evidence_sync_changes(user_id, entity, record_id, client_record_id, revision, deleted)
    select e.user_id, entity_name, e.id, coalesce(to_jsonb(e)#>>'{payload,id}', e.id::text), u.evidence_revision, false
    from changed_evidence e join public.users u on u.id=e.user_id
    on conflict(user_id, entity, record_id) do update
    set revision=excluded.revision, deleted=excluded.deleted, client_record_id=excluded.client_record_id;
    return null;
end;
$$;

insert into public.schema_migrations(version) values ('0050_evidence_change_index');
commit;
