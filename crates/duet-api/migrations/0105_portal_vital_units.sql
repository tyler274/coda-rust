-- duet-release: maintenance
begin;

-- Planned maintenance: the serving release still writes portal vitals with
-- the source's unit code as their canonical unit.

-- A portal vital stored its source's UCUM code as its canonical unit, so a
-- weight sent as 180 [lb_av] was charted as 180 kg. Its canonical value is now
-- set only by exact UCUM arithmetic from that code, (value + offset) * num /
-- den in double precision, else cleared; value and unit as sent are unchanged.
-- This matches app/services/vital_units.py.
-- Projection repair is not a new source version: the retained revisions are
-- repaired in place with their content hashes, and each affected person's
-- evidence fence advances once so no read in flight mixes the two.

create function pg_temp.duet_vital_quantity(concept text, quantity jsonb) returns jsonb
language sql immutable as $$
    select quantity || coalesce((
        select jsonb_build_object(
            'canonical_value', to_jsonb(
                ((quantity->>'value')::float8 + exact.value_offset)
                * exact.numerator / exact.denominator),
            'canonical_unit', exact.duet_unit)
        from (values
            ('body_mass', 'kg', 'kg', 0::float8, 1::float8, 1::float8), ('body_mass', 'g', 'kg', 0, 1, 1000),
            ('body_mass', '[lb_av]', 'kg', 0, 45359237, 100000000),
            ('body_mass', '[oz_av]', 'kg', 0, 45359237, 1600000000),
            ('height', 'cm', 'cm', 0, 1, 1), ('height', 'm', 'cm', 0, 100, 1), ('height', '[in_i]', 'cm', 0, 254, 100),
            ('waist', 'cm', 'cm', 0, 1, 1), ('waist', 'm', 'cm', 0, 100, 1), ('waist', '[in_i]', 'cm', 0, 254, 100),
            ('body_temperature', 'Cel', 'Cel', 0, 1, 1), ('body_temperature', '[degF]', 'Cel', -32, 5, 9),
            ('bmi', 'kg/m2', 'kg/m2', 0, 1, 1), ('blood_oxygen', '%', '%', 0, 1, 1),
            ('heart_rate', '/min', 'bpm', 0, 1, 1), ('heart_rate', '{beats}/min', 'bpm', 0, 1, 1),
            ('respiratory_rate', '/min', 'br/min', 0, 1, 1), ('respiratory_rate', '{breaths}/min', 'br/min', 0, 1, 1),
            ('blood_pressure', 'mm[Hg]', 'mmHg', 0, 1, 1)
        ) as exact(vital, ucum_code, duet_unit, value_offset, numerator, denominator)
        where exact.vital = concept and exact.ucum_code = quantity->>'canonical_unit'
    ), jsonb_build_object('canonical_value', null, 'canonical_unit', null))
$$;

create function pg_temp.duet_vital_payload(payload jsonb) returns jsonb
language plpgsql immutable as $$
declare
    concept text := payload->>'concept_id';
begin
    if concept is null or concept not in ('body_mass', 'height', 'waist', 'body_temperature', 'bmi',
                       'heart_rate', 'respiratory_rate', 'blood_oxygen', 'blood_pressure')
       or payload#>>'{source,kind}' in ('appleHealth', 'healthRecords', 'manual') then
        return payload;
    end if;
    if payload#>>'{value,type}' = 'quantity' then
        return jsonb_set(payload, '{value,quantity}',
            pg_temp.duet_vital_quantity(concept, payload#>'{value,quantity}'));
    end if;
    if payload#>>'{value,type}' = 'components' then
        return jsonb_set(payload, '{value,components}', (
            select coalesce(jsonb_agg(
                case when component#>>'{value,type}' = 'quantity'
                     then jsonb_set(component, '{value,quantity}',
                         pg_temp.duet_vital_quantity(concept, component#>'{value,quantity}'))
                     else component end
                order by position), '[]'::jsonb)
            from jsonb_array_elements(payload#>'{value,components}')
                with ordinality as rows(component, position)
        ));
    end if;
    return payload;
end;
$$;

lock table public.health_evidence, public.canonical_evidence_revisions,
    public.users in share row exclusive mode;

create temporary table duet_vital_affected_users on commit drop as
select user_id from public.health_evidence
where payload is distinct from pg_temp.duet_vital_payload(payload)
union
select user_id from public.canonical_evidence_revisions
where evidence_id is not null
  and payload is distinct from pg_temp.duet_vital_payload(payload);

update public.users u
set evidence_revision = u.evidence_revision + 1
where exists (select 1 from duet_vital_affected_users affected where affected.user_id = u.id);

alter table public.health_evidence
    disable trigger health_evidence_retain_revision,
    disable trigger health_evidence_update_revision;

update public.health_evidence
set payload = pg_temp.duet_vital_payload(payload)
where payload is distinct from pg_temp.duet_vital_payload(payload);

alter table public.health_evidence
    enable trigger health_evidence_retain_revision,
    enable trigger health_evidence_update_revision;

update public.canonical_evidence_revisions
set payload = pg_temp.duet_vital_payload(payload),
    content_sha256 = encode(public.digest(
        public.canonical_evidence_content(pg_temp.duet_vital_payload(payload))::text,
        'sha256'
    ), 'hex')
where evidence_id is not null
  and payload is distinct from pg_temp.duet_vital_payload(payload);

insert into public.schema_migrations(version) values ('0105_portal_vital_units');
commit;
