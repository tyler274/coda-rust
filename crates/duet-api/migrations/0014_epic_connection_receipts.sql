begin;

-- One-time forward conversion. No runtime old-ID alias table or decoder.
-- Quiesce old writers before release: they cannot emit the new receipt key.
lock table public.health_evidence, public.health_contexts,
    public.insight_cycles, public.insight_fact_evaluations, public.doctor_collection_items,
    public.doctor_preparations, public.source_acquisition_staging,
    public.source_query_memberships in share row exclusive mode;

create temporary table epic_receipt_rekeys (
    user_id uuid not null,
    old_id uuid primary key,
    new_id uuid not null unique
) on commit drop;

do $$
declare row_value record; material text; bytes bytea;
begin
    for row_value in select * from public.health_evidence where source_kind='epic'
    loop
        if nullif(row_value.source_connection_id, '') is null
           or row_value.payload #>> '{source,connection_id}' is distinct from row_value.source_connection_id
           or nullif(row_value.payload #>> '{source,issuer}', '') is null
           or nullif(row_value.payload #>> '{source,resource_type}', '') is null
           or nullif(row_value.payload #>> '{source,record_id}', '') is null then
            raise exception 'Epic receipt migration requires complete source identity';
        end if;
        -- RFC 9562 UUIDv5, matching EpicFHIRFact.evidence_id exactly. Connection
        -- UUID is the namespace; source strings and case are preserved verbatim.
        material := concat_ws(':','duet','epic',
            row_value.payload #>> '{source,issuer}',
            row_value.payload #>> '{source,resource_type}',
            row_value.payload #>> '{source,record_id}', row_value.concept_id);
        bytes := substring(public.digest(
            uuid_send(row_value.source_connection_id::uuid) || convert_to(material,'UTF8'),
            'sha1') from 1 for 16);
        bytes := set_byte(bytes,6,(get_byte(bytes,6) & 15) | 80);
        bytes := set_byte(bytes,8,(get_byte(bytes,8) & 63) | 128);
        insert into epic_receipt_rekeys values
            (row_value.user_id,row_value.id,encode(bytes,'hex')::uuid);
    end loop;
end;
$$;

-- These temporary functions traverse known reference fields, never patient
-- prose, code labels, source identifiers, rendered values or arbitrary JSON.
create function pg_temp.receipt_identifier(value text, owner uuid) returns text
language plpgsql as $$
declare parts text[]; receipt uuid; replacement uuid;
begin
    if value is null then return null; end if;
    parts := string_to_array(value, ':');
    if cardinality(parts)=1 then
        begin receipt := value::uuid; exception when invalid_text_representation then return value; end;
    elsif cardinality(parts) in (2,3) and parts[1] in ('lab','clinical','medication','record') then
        begin receipt := parts[2]::uuid; exception when invalid_text_representation then return value; end;
    else return value;
    end if;
    select new_id into replacement from epic_receipt_rekeys where old_id=receipt and user_id=owner;
    if replacement is null then return value; end if;
    -- Native UUID.uuidString is uppercase in links/evidence references, while
    -- fact IDs use lowercase. Preserve each structured field's original bytes
    -- convention so the next native projection keeps the same fingerprint.
    if cardinality(parts)=1 then
        return case when value=upper(value) then upper(replacement::text) else replacement::text end;
    end if;
    parts[2] := case when parts[2]=upper(parts[2]) then upper(replacement::text) else replacement::text end;
    return array_to_string(parts, ':');
end;
$$;

create function pg_temp.receipt_link(value text, owner uuid) returns text
language plpgsql as $$
declare parts text[]; path text[]; parameters text[]; i integer;
begin
    if value is null or not starts_with(value,'duet://health/') then return value; end if;
    parts := string_to_array(value,'?');
    if cardinality(parts)>2 then return value; end if;
    path := string_to_array(parts[1],'/');
    if path[4] in ('clinical','medication') and cardinality(path)=5 then
        path[5] := pg_temp.receipt_identifier(path[5],owner);
    elsif path[4]='clinical-group' and cardinality(path)=6 then
        path[6] := pg_temp.receipt_identifier(path[6],owner);
    elsif path[4]='lab' and cardinality(parts)=2 then
        parameters := string_to_array(parts[2],'&');
        for i in 1..cardinality(parameters) loop
            if starts_with(parameters[i],'result=') then
                parameters[i] := 'result=' || pg_temp.receipt_identifier(substring(parameters[i] from 8),owner);
            end if;
        end loop;
        parts[2] := array_to_string(parameters,'&');
    end if;
    parts[1] := array_to_string(path,'/');
    return array_to_string(parts,'?');
end;
$$;

create function pg_temp.receipt_references(value jsonb, owner uuid, shape text) returns jsonb
language plpgsql as $$
declare result jsonb := value; item jsonb; field text;
begin
    if jsonb_typeof(value)='array' then
        result := '[]'::jsonb;
        for item in select * from jsonb_array_elements(value) loop
            result := result || jsonb_build_array(pg_temp.receipt_references(item,owner,shape));
        end loop;
        return result;
    end if;
    if jsonb_typeof(value) is distinct from 'object' then return value; end if;
    if shape in ('evidence','fact') and value ? 'id' then
        result := jsonb_set(result,'{id}',to_jsonb(pg_temp.receipt_identifier(value->>'id',owner)));
    end if;
    if shape='evidence' and value->>'deep_link' is not null then
        result := jsonb_set(result,'{deep_link}',to_jsonb(pg_temp.receipt_link(value->>'deep_link',owner)));
    end if;
    if shape in ('fact','insight','doctor_item') and value ? 'evidence' then
        result := jsonb_set(result,'{evidence}',pg_temp.receipt_references(value->'evidence',owner,'evidence'));
    end if;
    if shape in ('insight','visual') and value ? 'fact_ids' then
        result := jsonb_set(result,'{fact_ids}',coalesce((select jsonb_agg(
            pg_temp.receipt_identifier(identifier,owner) order by position)
            from jsonb_array_elements_text(value->'fact_ids') with ordinality as identifiers(identifier,position)), '[]'::jsonb));
    end if;
    if shape='insight' and jsonb_typeof(value->'visual')='object' then
        result := jsonb_set(result,'{visual}',pg_temp.receipt_references(value->'visual',owner,'visual'));
    end if;
    if shape='doctor_item' and value->>'kind'='fact' then
        result := jsonb_set(result,'{source_id}',to_jsonb(pg_temp.receipt_identifier(value->>'source_id',owner)));
    end if;
    if shape='draft' and value ? 'evidence_items' then
        result := jsonb_set(result,'{evidence_items}',pg_temp.receipt_references(value->'evidence_items',owner,'doctor_item'));
    end if;
    return result;
end;
$$;

-- The existing release runner fills this temporary table using HealthFact and
-- fact_identity, not a second SQL implementation of Python serialization/hash.
-- Both phases run on the same connection under the locks above. Direct SQL
-- execution is safe for a fresh database, but fails closed for evaluated data.
create temporary table epic_evaluation_rekeys (
    user_id uuid not null,
    cycle_id uuid not null,
    old_fingerprint text not null,
    new_fingerprint text not null,
    primary key(user_id,cycle_id,old_fingerprint)
) on commit drop;

-- EPIC_TYPED_EVALUATION_REKEY

do $$
begin
    if exists (
        select 1 from public.insight_fact_evaluations e
        where e.user_id in (select user_id from epic_receipt_rekeys)
          and not exists (select 1 from epic_evaluation_rekeys m
              where m.user_id=e.user_id and m.cycle_id=e.cycle_id
                and m.old_fingerprint=e.fact_fingerprint)
    ) then
        raise exception 'Epic receipt migration requires typed evaluation rekey through apply_pending_migrations.py';
    end if;
end;
$$;

update public.insight_fact_evaluations e set fact_fingerprint=m.new_fingerprint
from epic_evaluation_rekeys m
where e.user_id=m.user_id and e.cycle_id=m.cycle_id
  and e.fact_fingerprint=m.old_fingerprint
  and m.old_fingerprint<>m.new_fingerprint;

-- Updating primary identity carries query coverage along with it.
alter table public.source_query_memberships drop constraint source_query_memberships_evidence_id_fkey;
alter table public.source_query_memberships add constraint source_query_memberships_evidence_id_fkey
    foreign key(evidence_id) references public.health_evidence(id) on update cascade on delete cascade;
update public.health_evidence e set id=m.new_id,
    payload=jsonb_set(e.payload,'{id}',to_jsonb(m.new_id::text))
from epic_receipt_rekeys m where e.id=m.old_id and e.id<>m.new_id;

update public.health_evidence e set payload=jsonb_set(e.payload,'{corrects_evidence_id}',to_jsonb(m.new_id::text))
from epic_receipt_rekeys m where e.user_id=m.user_id
    and lower(e.payload->>'corrects_evidence_id')=m.old_id::text;

update public.source_acquisition_staging s set evidence_id=m.new_id,
    payload=jsonb_set(s.payload,'{id}',to_jsonb(m.new_id::text))
from epic_receipt_rekeys m,public.source_acquisition_units u,public.source_connections c
where s.evidence_id=m.old_id and s.unit_id=u.id and u.connection_id=c.id and c.user_id=m.user_id;

update public.health_contexts set facts=pg_temp.receipt_references(facts,user_id,'fact'),
    revision=revision+1,updated_at=now()
where user_id in (select user_id from epic_receipt_rekeys);
update public.insight_cycles set
    supporting_facts=pg_temp.receipt_references(supporting_facts,user_id,'fact'),
    insights=pg_temp.receipt_references(insights,user_id,'insight'),
    evidence=pg_temp.receipt_references(evidence,user_id,'evidence')
where user_id in (select user_id from epic_receipt_rekeys);
update public.doctor_collection_items set
    source_id=case when kind='fact' then pg_temp.receipt_identifier(source_id,user_id) else source_id end,
    evidence=pg_temp.receipt_references(evidence,user_id,'evidence')
where user_id in (select user_id from epic_receipt_rekeys);
-- Preserve the authored drafts and original fingerprint. Changed references
-- make the existing preparation outdated, never fabricated as newly prepared.
update public.doctor_preparations set
    generated_draft=pg_temp.receipt_references(generated_draft,user_id,'draft'),
    edited_draft=pg_temp.receipt_references(edited_draft,user_id,'draft')
where user_id in (select user_id from epic_receipt_rekeys);

drop function pg_temp.receipt_references(jsonb,uuid,text);
drop function pg_temp.receipt_link(text,uuid);
drop function pg_temp.receipt_identifier(text,uuid);
insert into public.schema_migrations(version) values ('0014_epic_connection_receipts');
commit;
