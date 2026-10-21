begin;

-- Repair Epic receipts emitted after 0014 by a stale writer. Identity belongs
-- to the account-owned source connection; this is a forward data correction,
-- never a runtime alias or compatibility path.
lock table public.health_evidence, public.canonical_evidence_revisions,
    public.source_connections, public.source_acquisition_units,
    public.source_acquisition_staging, public.source_query_memberships,
    public.health_contexts, public.insight_cycles,
    public.doctor_collection_items, public.doctor_preparations,
    public.operating_indexes
    in share row exclusive mode;

create function pg_temp.epic_receipt_id(
    connection_id uuid,
    issuer text,
    resource_type text,
    record_id text,
    concept_id text
) returns uuid
language plpgsql immutable strict as $$
declare material text; bytes bytea;
begin
    material := concat_ws(
        ':', 'duet', 'epic', issuer, resource_type, record_id, concept_id
    );
    bytes := substring(public.digest(
        uuid_send(connection_id) || convert_to(material, 'UTF8'), 'sha1'
    ) from 1 for 16);
    bytes := set_byte(bytes, 6, (get_byte(bytes, 6) & 15) | 80);
    bytes := set_byte(bytes, 8, (get_byte(bytes, 8) & 63) | 128);
    return encode(bytes, 'hex')::uuid;
end;
$$;

create temporary table epic_receipt_repairs (
    user_id uuid not null,
    connection_id uuid not null,
    old_id uuid primary key,
    new_id uuid not null unique
) on commit drop;

do $$
declare row_value record; expected_id uuid; existing record;
begin
    for row_value in
        select e.user_id, e.id, e.concept_id, e.source_connection_id,
            e.source_record_id, e.payload
        from public.health_evidence e
        where e.source_kind = 'epic'
    loop
        if nullif(row_value.source_connection_id, '') is null
           or row_value.payload->>'id' is distinct from row_value.id::text
           or row_value.payload #>> '{source,kind}' is distinct from 'epic'
           or row_value.payload #>> '{source,connection_id}'
                is distinct from row_value.source_connection_id
           or row_value.payload #>> '{source,record_id}'
                is distinct from row_value.source_record_id
           or row_value.payload->>'concept_id' is distinct from row_value.concept_id
           or nullif(row_value.payload #>> '{source,issuer}', '') is null
           or nullif(row_value.payload #>> '{source,resource_type}', '') is null
           or nullif(row_value.payload #>> '{source,record_id}', '') is null
           or not exists (
                select 1 from public.source_connections c
                where c.id = row_value.source_connection_id::uuid
                  and c.user_id = row_value.user_id
                  and c.provider = 'epic'
           ) then
            raise exception 'Epic receipt repair requires complete owned source identity';
        end if;

        expected_id := pg_temp.epic_receipt_id(
            row_value.source_connection_id::uuid,
            row_value.payload #>> '{source,issuer}',
            row_value.payload #>> '{source,resource_type}',
            row_value.payload #>> '{source,record_id}',
            row_value.concept_id
        );
        if expected_id <> row_value.id then
            insert into epic_receipt_repairs
            values (
                row_value.user_id,
                row_value.source_connection_id::uuid,
                row_value.id,
                expected_id
            );
        end if;
    end loop;

    -- A staged record can outlive the writer that created it. Include those
    -- records in the same correction even when they have not been published.
    for row_value in
        select c.user_id, c.id as connection_id, s.evidence_id as id,
            s.payload->>'concept_id' as concept_id, s.payload
        from public.source_acquisition_staging s
        join public.source_acquisition_units u on u.id = s.unit_id
        join public.source_connections c on c.id = u.connection_id
        where s.payload #>> '{source,kind}' = 'epic'
    loop
        if row_value.payload->>'id' is distinct from row_value.id::text
           or row_value.payload #>> '{source,connection_id}'
                is distinct from row_value.connection_id::text
           or nullif(row_value.concept_id, '') is null
           or nullif(row_value.payload #>> '{source,issuer}', '') is null
           or nullif(row_value.payload #>> '{source,resource_type}', '') is null
           or nullif(row_value.payload #>> '{source,record_id}', '') is null
           or row_value.payload #>> '{source,record_id}' is distinct from
                row_value.payload #>> '{source,upstream_record_id}'
           or not exists (
                select 1 from public.source_connections c
                where c.id = row_value.connection_id
                  and c.user_id = row_value.user_id
                  and c.provider = 'epic'
           ) then
            raise exception 'Epic staged receipt repair requires complete owned source identity';
        end if;

        expected_id := pg_temp.epic_receipt_id(
            row_value.connection_id,
            row_value.payload #>> '{source,issuer}',
            row_value.payload #>> '{source,resource_type}',
            row_value.payload #>> '{source,record_id}',
            row_value.concept_id
        );
        if expected_id <> row_value.id then
            select * into existing
            from epic_receipt_repairs where old_id = row_value.id;
            if found then
                if existing.user_id <> row_value.user_id
                   or existing.connection_id <> row_value.connection_id
                   or existing.new_id <> expected_id then
                    raise exception 'Epic receipt repair identity conflict';
                end if;
            else
                insert into epic_receipt_repairs
                values (
                    row_value.user_id,
                    row_value.connection_id,
                    row_value.id,
                    expected_id
                );
            end if;
        end if;
    end loop;
end;
$$;

do $$
begin
    if exists (
        select 1 from epic_receipt_repairs m
        join public.health_evidence e on e.id = m.new_id
    ) then
        raise exception 'Epic receipt repair target already exists';
    end if;
    if exists (
        select 1
        from public.source_acquisition_staging old_stage
        join epic_receipt_repairs m on m.old_id = old_stage.evidence_id
        join public.source_acquisition_staging new_stage
          on new_stage.unit_id = old_stage.unit_id
         and new_stage.evidence_id = m.new_id
    ) then
        raise exception 'Epic staged receipt repair target already exists';
    end if;
    if exists (
        select 1
        from public.canonical_evidence_revisions r
        join epic_receipt_repairs m on m.old_id = r.evidence_id
        where r.user_id <> m.user_id
           or r.payload->>'id' is distinct from m.old_id::text
    ) then
        raise exception 'Epic receipt repair revision identity mismatch';
    end if;

    -- Doctor preparation is patient-authored work and must never be discarded
    -- or rewritten by an evidence-identity repair. Stop before touching any
    -- source or derived row when that durable boundary is populated.
    if exists (
        select 1 from public.doctor_collection_items x
        where x.user_id in (select user_id from epic_receipt_repairs)
    ) or exists (
        select 1 from public.doctor_preparations x
        where x.user_id in (select user_id from epic_receipt_repairs)
    ) then
        raise exception 'Epic receipt repair requires empty doctor dependencies';
    end if;
end;
$$;

-- Context snapshots, cycles, and opaque operating indexes are regenerable
-- projections of canonical evidence. Their JSON may contain the retired UUID,
-- so discard them atomically rather than attempting a speculative prose or
-- model-state rewrite. The next ordinary read/generation rebuilds current state
-- from the preserved canonical receipts and revision history.
delete from public.operating_indexes x
where x.user_id in (select user_id from epic_receipt_repairs);

delete from public.insight_cycles x
where x.user_id in (select user_id from epic_receipt_repairs);

delete from public.health_contexts x
where x.user_id in (select user_id from epic_receipt_repairs);

-- Existing history is subordinate to current canonical evidence. Carry its
-- foreign key through the primary-key correction while retaining every row.
alter table public.canonical_evidence_revisions
    drop constraint canonical_evidence_revisions_user_id_evidence_id_fkey;
alter table public.canonical_evidence_revisions
    add constraint canonical_evidence_revisions_user_id_evidence_id_fkey
    foreign key (user_id, evidence_id)
    references public.health_evidence(user_id, id)
    on update cascade on delete cascade;

-- Identifier correction is not a new source-authored revision. Disable only
-- the revision-capture trigger while current rows and retained snapshots move.
alter table public.health_evidence disable trigger health_evidence_retain_revision;

update public.health_evidence e
set id = m.new_id,
    payload = jsonb_set(e.payload, '{id}', to_jsonb(m.new_id::text))
from epic_receipt_repairs m
where e.id = m.old_id;

update public.health_evidence e
set payload = jsonb_set(
    e.payload, '{corrects_evidence_id}', to_jsonb(m.new_id::text)
)
from epic_receipt_repairs m
where e.user_id = m.user_id
  and lower(e.payload->>'corrects_evidence_id') = m.old_id::text;

alter table public.health_evidence enable trigger health_evidence_retain_revision;

with changed as (
    select r.revision,
        jsonb_set(r.payload, '{id}', to_jsonb(m.new_id::text)) as payload
    from public.canonical_evidence_revisions r
    join epic_receipt_repairs m on m.new_id = r.evidence_id
    where r.payload->>'id' = m.old_id::text
)
update public.canonical_evidence_revisions r
set payload = changed.payload,
    content_sha256 = encode(public.digest(
        public.canonical_evidence_content(changed.payload)::text, 'sha256'
    ), 'hex')
from changed
where r.revision = changed.revision;

with changed as (
    select r.revision,
        jsonb_set(
            r.payload, '{corrects_evidence_id}', to_jsonb(m.new_id::text)
        ) as payload
    from public.canonical_evidence_revisions r
    join epic_receipt_repairs m on m.user_id = r.user_id
    where lower(r.payload->>'corrects_evidence_id') = m.old_id::text
)
update public.canonical_evidence_revisions r
set payload = changed.payload,
    content_sha256 = encode(public.digest(
        public.canonical_evidence_content(changed.payload)::text, 'sha256'
    ), 'hex')
from changed
where r.revision = changed.revision;

update public.source_acquisition_staging s
set evidence_id = m.new_id,
    payload = jsonb_set(s.payload, '{id}', to_jsonb(m.new_id::text))
from epic_receipt_repairs m,
     public.source_acquisition_units u
where s.evidence_id = m.old_id
  and s.unit_id = u.id
  and u.connection_id = m.connection_id;

drop function pg_temp.epic_receipt_id(uuid, text, text, text, text);

insert into public.schema_migrations(version)
values ('0019_repair_epic_receipt_identity');

commit;
