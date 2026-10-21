begin;

-- Reproject every retained medication version from its original source bytes.
-- Directions contain authored text only; dose and route remain independent.
create function public.duet_medication_has_modifier(value jsonb)
returns boolean
language plpgsql immutable strict set search_path=pg_catalog,public as $$
declare child jsonb;
begin
    if jsonb_typeof(value)='object' then
        if value ? 'modifierExtension' then
            if jsonb_typeof(value->'modifierExtension')<>'array' then
                return true;
            end if;
            if jsonb_array_length(value->'modifierExtension')>0 then
                return true;
            end if;
        end if;
        for child in select item.value from jsonb_each(value) item loop
            if public.duet_medication_has_modifier(child) then return true; end if;
        end loop;
    elsif jsonb_typeof(value)='array' then
        for child in select item.value from jsonb_array_elements(value) item loop
            if public.duet_medication_has_modifier(child) then return true; end if;
        end loop;
    end if;
    return false;
end;
$$;

create function public.duet_medication_quantity(value jsonb)
returns text
language plpgsql immutable strict set search_path=pg_catalog,public as $$
declare number text; unit text; comparator text;
begin
    if jsonb_typeof(value)<>'object' then return null; end if;
    if jsonb_typeof(value->'value') not in ('number','string') then
        return null;
    end if;
    number := nullif(btrim(value->>'value'),'');
    if number is null then return null; end if;
    unit := nullif(btrim(case
        when jsonb_typeof(value->'unit')='string' then value->>'unit'
        when jsonb_typeof(value->'code')='string' then value->>'code'
        else null
    end),'');
    comparator := nullif(btrim(case
        when jsonb_typeof(value->'comparator')='string' then value->>'comparator'
        else null
    end),'');
    return concat_ws(' ',comparator,number,unit);
end;
$$;

create function public.duet_medication_range(value jsonb)
returns text
language plpgsql immutable strict set search_path=pg_catalog,public as $$
declare low text; high text;
begin
    if jsonb_typeof(value)<>'object' then return null; end if;
    low := public.duet_medication_quantity(value->'low');
    high := public.duet_medication_quantity(value->'high');
    if low is not null and high is not null then return low||' – '||high; end if;
    if low is not null then return 'At least '||low; end if;
    if high is not null then return 'Up to '||high; end if;
    return null;
end;
$$;

create function public.duet_medication_concept(value jsonb)
returns text
language plpgsql immutable strict set search_path=pg_catalog,public as $$
declare result text; label text; row jsonb; seen text[] := '{}'::text[];
begin
    if jsonb_typeof(value)<>'object' then return null; end if;
    result := case when jsonb_typeof(value->'text')='string' then value->>'text' end;
    if result is not null and btrim(result)<>'' then return result; end if;
    if jsonb_typeof(value->'coding')<>'array' then return null; end if;
    if exists (
           select 1 from jsonb_array_elements(value->'coding') item
           where jsonb_typeof(item.value)<>'object'
       ) then return null; end if;
    result := null;
    for row in select item.value from jsonb_array_elements(value->'coding') item loop
        label := case when jsonb_typeof(row->'display')='string' then row->>'display' end;
        if label is not null and btrim(label)<>'' and not label=any(seen) then
            if result is null then result := label;
            else result := result||' / '||label;
            end if;
            seen := array_append(seen,label);
        end if;
    end loop;
    return result;
end;
$$;

create function public.duet_medication_regimen(resource jsonb)
returns jsonb
language plpgsql immutable strict set search_path=pg_catalog,public as $$
declare entries jsonb; entry jsonb; dose jsonb; value text;
declare instructions jsonb := '[]'::jsonb;
declare doses jsonb := '[]'::jsonb;
declare routes jsonb := '[]'::jsonb;
begin
    if resource->>'resourceType' not in (
        'MedicationAdministration','MedicationOrder','MedicationRequest','MedicationStatement'
    ) or public.duet_medication_has_modifier(resource) then
        return jsonb_build_object('instructions',instructions,'doses',doses,'routes',routes);
    end if;
    if resource->>'resourceType' in ('MedicationRequest','MedicationOrder') then
        entries := coalesce(resource->'dosageInstruction','[]'::jsonb);
    elsif resource->>'resourceType'='MedicationStatement' then
        entries := coalesce(resource->'dosage','[]'::jsonb);
    elsif resource ? 'dosage' and jsonb_typeof(resource->'dosage')='object' then
        entries := jsonb_build_array(resource->'dosage');
    else
        entries := '[]'::jsonb;
    end if;
    if jsonb_typeof(entries)<>'array' then
        return jsonb_build_object('instructions',instructions,'doses',doses,'routes',routes);
    end if;
    if exists (
        select 1 from jsonb_array_elements(entries) item
        where jsonb_typeof(item.value)<>'object'
    ) then
        return jsonb_build_object('instructions',instructions,'doses',doses,'routes',routes);
    end if;
    for entry in select item.value from jsonb_array_elements(entries) item loop
        foreach value in array array[
            case when jsonb_typeof(entry->'text')='string' then entry->>'text' end,
            case when jsonb_typeof(entry->'patientInstruction')='string'
                 then entry->>'patientInstruction' end
        ] loop
            if value is not null and btrim(value)<>'' then
                instructions := instructions||jsonb_build_array(value);
            end if;
        end loop;
        foreach value in array array[
            public.duet_medication_quantity(entry->'doseQuantity'),
            public.duet_medication_quantity(entry->'quantityQuantity'),
            public.duet_medication_quantity(entry->'quantity'),
            public.duet_medication_quantity(entry->'dose'),
            public.duet_medication_range(entry->'doseRange'),
            public.duet_medication_range(entry->'quantityRange')
        ] loop
            if value is not null then
                doses := doses||jsonb_build_array(value);
            end if;
        end loop;
        if jsonb_typeof(entry->'doseAndRate')='array' then
            if exists (
               select 1 from jsonb_array_elements(entry->'doseAndRate') item
               where jsonb_typeof(item.value)<>'object'
            ) then
                null;
            else
                for dose in select item.value from jsonb_array_elements(entry->'doseAndRate') item loop
                    foreach value in array array[
                        public.duet_medication_quantity(dose->'doseQuantity'),
                        public.duet_medication_range(dose->'doseRange')
                    ] loop
                        if value is not null then
                            doses := doses||jsonb_build_array(value);
                        end if;
                    end loop;
                end loop;
            end if;
        end if;
        value := public.duet_medication_concept(entry->'route');
        if value is not null then
            routes := routes||jsonb_build_array(value);
        end if;
    end loop;
    return jsonb_build_object('instructions',instructions,'doses',doses,'routes',routes);
end;
$$;

create function public.duet_reproject_medication(payload jsonb)
returns jsonb
language plpgsql immutable strict set search_path=pg_catalog,public as $$
declare resource jsonb; content text;
begin
    content := payload#>>'{source_record,content}';
    if content is null then return payload; end if;
    if payload#>>'{source_record,format_version}' is not null
       and payload#>>'{source_record,format_version}' not in (
           '1.0.2','3.0.1','3.0.2','4.0.0','4.0.1','4.3.0'
       ) then
        return jsonb_set(
            payload,
            '{clinical_record,medication_regimen}',
            '{"instructions":[],"doses":[],"routes":[]}'::jsonb,
            true
        );
    end if;
    begin resource := content::jsonb;
    exception when others then return payload;
    end;
    return jsonb_set(
        payload,
        '{clinical_record,medication_regimen}',
        public.duet_medication_regimen(resource),
        true
    );
end;
$$;

-- Projection repair is not a new source version. Capture every affected owner
-- before mutation, including owners whose medication exists only in retained
-- history, then advance the ordinary evidence fence exactly once and supersede
-- private working context.
lock table public.health_evidence, public.canonical_evidence_revisions,
    public.users, public.operating_indexes in share row exclusive mode;

create temporary table duet_medication_affected_users on commit drop as
select user_id
from public.health_evidence
where payload#>>'{clinical_record,kind}'='medication'
  and jsonb_typeof(payload->'source_record')='object'
  and payload is distinct from public.duet_reproject_medication(payload)
union
select user_id
from public.canonical_evidence_revisions
where evidence_id is not null
  and payload#>>'{clinical_record,kind}'='medication'
  and jsonb_typeof(payload->'source_record')='object'
  and payload is distinct from public.duet_reproject_medication(payload);

update public.users u
set evidence_revision=u.evidence_revision+1
where exists (
    select 1 from duet_medication_affected_users affected
    where affected.user_id=u.id
);

update public.operating_indexes o
set superseded_at=now()
where o.superseded_at is null
  and exists (
      select 1 from duet_medication_affected_users affected
      where affected.user_id=o.user_id
  );

alter table public.health_evidence
    disable trigger health_evidence_retain_revision,
    disable trigger health_evidence_update_revision;

update public.health_evidence
set payload=public.duet_reproject_medication(payload)
where payload#>>'{clinical_record,kind}'='medication'
  and jsonb_typeof(payload->'source_record')='object'
  and payload is distinct from public.duet_reproject_medication(payload);

alter table public.health_evidence
    enable trigger health_evidence_retain_revision,
    enable trigger health_evidence_update_revision;

update public.canonical_evidence_revisions
set payload=public.duet_reproject_medication(payload),
    content_sha256=encode(public.digest(
        public.canonical_evidence_content(
            public.duet_reproject_medication(payload)
        )::text,
        'sha256'
    ),'hex')
where evidence_id is not null
  and payload#>>'{clinical_record,kind}'='medication'
  and jsonb_typeof(payload->'source_record')='object'
  and payload is distinct from public.duet_reproject_medication(payload);

drop function public.duet_reproject_medication(jsonb);
drop function public.duet_medication_regimen(jsonb);
drop function public.duet_medication_concept(jsonb);
drop function public.duet_medication_range(jsonb);
drop function public.duet_medication_quantity(jsonb);
drop function public.duet_medication_has_modifier(jsonb);

insert into public.schema_migrations(version)
values ('0025_canonical_medication_regimen');

commit;
